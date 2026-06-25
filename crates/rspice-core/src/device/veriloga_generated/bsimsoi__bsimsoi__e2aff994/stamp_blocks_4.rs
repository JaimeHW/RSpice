#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_block_20(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1752] != 0.0))) {
            let assign32090_ad_e48629: A = {
                if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))), 1.0))
                } else {
                    let assign32090_ad_e48628: A = {
                        if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271)))
                        } else {
                            {
                                if ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign32090_ad_e48628
                }
            };
            let assign32090_ad_e48711: A = {
                if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))), 1.0))
                } else {
                    let assign32090_ad_e48710: A = {
                        if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(271)), s.ad_value(6)), s.ad_value(271)))
                        } else {
                            {
                                if ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign32090_ad_e48710
                }
            };
            s.store_sub_ad(169, assign32090_ad_e48629, assign32090_ad_e48711);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1752] != 0.0))) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div(s.ad_value(169), A::offset(s.ad_value(3), 1.0))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1752] != 0.0))) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1752] != 0.0))) {
            s.store_limited_exp_ad(173, A::neg(s.ad_value(123)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1752] != 0.0))) {
            s.store_square(174, 123);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1752] != 0.0))) {
            s.store_div_from_scalar_ad(175, 1.0, A::offset(s.ad_value(174), 2.0));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1752] != 0.0))) {
            s.store_limited_exp_ad(176, A::neg(s.ad_value(97)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1752] != 0.0))) {
            s.store_limited_exp_ad(177, A::sub(s.ad_value(123), s.ad_value(97)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1752] != 0.0))) {
            let assign32170_ad_e48847: A = A::sub(A::sub(A::mul(A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123))), A::mul(A::mul(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170))), A::add(s.ad_value(216), s.ad_value(170)))), A::mul(s.ad_value(296), A::sub(A::add(A::add(A::add(A::sub(s.ad_value(173), s.ad_value(171)), s.ad_value(123)), s.ad_value(170)), s.ad_value(177)), A::mul(s.ad_value(176), A::add(A::offset(s.ad_value(123), 1.0), A::mul(s.ad_value(175), s.ad_value(174)))))));
            s.store_ad(19, &assign32170_ad_e48847);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1752] != 0.0))) {
            let assign32180_ad_e48919: A = A::sub(A::sub(A::add(s.ad_value(177), A::mul(s.ad_value(176), A::offset(A::add(A::mul(A::scale(s.ad_value(123), (-2.0)), s.ad_value(175)), A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(123), 2.0), s.ad_value(123)), s.ad_value(123)), s.ad_value(175)), s.ad_value(175))), (-1.0)))), s.ad_value(173)), A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))));
            let assign32180_ad_e48935: A = A::sub(A::add(A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170))), s.ad_value(2)), s.ad_value(2)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::scale(s.ad_value(214), 2.0)), A::scale(s.ad_value(123), 2.0)), A::mul(s.ad_value(296), A::offset(A::add(assign32180_ad_e48919, A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)))), 1.0)));
            s.store_ad(20, &assign32180_ad_e48935);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1752] != 0.0))) {
            let assign32190_ad_e48988: A = A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::mul(A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0))));
            let assign32190_ad_e49047: A = A::mul(A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::add(A::sub(A::sub_from_scalar(1.0, A::div(s.ad_value(167), A::offset(s.ad_value(167), 1.0))), s.ad_value(171)), A::mul(A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::offset(s.ad_value(167), 1.0)), A::offset(A::div_from_scalar(1.0, A::offset(s.ad_value(3), 1.0)), 1.0))));
            let assign32190_ad_e49076: A = A::sub(A::sub(assign32190_ad_e48988, A::mul(s.ad_value(296), A::sub(A::sub(A::add(s.ad_value(173), s.ad_value(177)), A::mul(A::mul(A::scale(s.ad_value(176), 2.0), s.ad_value(175)), A::sub_from_scalar(1.0, A::mul(A::mul(s.ad_value(174), s.ad_value(175)), A::sub_from_scalar(5.0, A::mul(A::scale(s.ad_value(174), 4.0), s.ad_value(175))))))), assign32190_ad_e49047))), A::div(A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(167), 1.0))));
            s.store_offset_ad(21, assign32190_ad_e49076, 2.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1752] != 0.0))) {
            s.store_sub_ad_rhs(22, 123, A::mul(A::div(s.ad_value(19), s.ad_value(20)), A::offset(A::div(A::mul(s.ad_value(19), s.ad_value(21)), A::mul(A::scale(s.ad_value(20), 2.0), s.ad_value(20))), 1.0)));
        }

        if ((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) {
            s.copy_ad(123, 22);
        }

        s.v[1753] = if (((s.v[214]) as f64).abs() <= s.v[102]) { 1.0 } else { 0.0 };

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1753] != 0.0)) {
            s.store_scale_ad(167, A::square(s.ad_value(96)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1753] != 0.0)) {
            s.store_mul_ad(22, A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(99))), s.ad_value(294)), s.ad_value(167)), 1.0));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1753] != 0.0))) {
            s.store_scale_ad(167, A::add(A::tanh(A::scale(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0))), A::tanh(A::scale(A::add(s.ad_value(214), s.ad_value(103)), 5.0))), 0.5);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1753] != 0.0))) {
            s.store_mul_ad_lhs(45, A::mul(A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(99))), s.ad_value(294)), s.ad_value(35)), 1.0)), 167);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1753] != 0.0))) {
            s.store_limited_exp_ad(167, A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1753] != 0.0))) {
            let assign32280_ad_e49304: A = {
                if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))), 1.0))
                } else {
                    let assign32280_ad_e49303: A = {
                        if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271)))
                        } else {
                            {
                                if ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign32280_ad_e49303
                }
            };
            let assign32280_ad_e49386: A = {
                if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))), 1.0))
                } else {
                    let assign32280_ad_e49385: A = {
                        if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(271)), s.ad_value(6)), s.ad_value(271)))
                        } else {
                            {
                                if ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign32280_ad_e49385
                }
            };
            s.store_sub_ad(169, assign32280_ad_e49304, assign32280_ad_e49386);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1753] != 0.0))) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div(s.ad_value(169), A::offset(s.ad_value(3), 1.0))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1753] != 0.0))) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1753] != 0.0))) {
            s.store_limited_exp_ad(173, A::neg(s.ad_value(123)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1753] != 0.0))) {
            s.store_square(174, 123);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1753] != 0.0))) {
            s.store_div_from_scalar_ad(175, 1.0, A::offset(s.ad_value(174), 2.0));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1753] != 0.0))) {
            s.store_limited_exp_ad(176, A::neg(s.ad_value(97)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1753] != 0.0))) {
            s.store_limited_exp_ad(177, A::sub(s.ad_value(123), s.ad_value(97)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1753] != 0.0))) {
            let assign32360_ad_e49522: A = A::sub(A::sub(A::mul(A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123))), A::mul(A::mul(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170))), A::add(s.ad_value(216), s.ad_value(170)))), A::mul(s.ad_value(296), A::sub(A::add(A::add(A::add(A::sub(s.ad_value(173), s.ad_value(171)), s.ad_value(123)), s.ad_value(170)), s.ad_value(177)), A::mul(s.ad_value(176), A::add(A::offset(s.ad_value(123), 1.0), A::mul(s.ad_value(175), s.ad_value(174)))))));
            s.store_ad(19, &assign32360_ad_e49522);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1753] != 0.0))) {
            let assign32370_ad_e49594: A = A::sub(A::sub(A::add(s.ad_value(177), A::mul(s.ad_value(176), A::offset(A::add(A::mul(A::scale(s.ad_value(123), (-2.0)), s.ad_value(175)), A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(123), 2.0), s.ad_value(123)), s.ad_value(123)), s.ad_value(175)), s.ad_value(175))), (-1.0)))), s.ad_value(173)), A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))));
            let assign32370_ad_e49610: A = A::sub(A::add(A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170))), s.ad_value(2)), s.ad_value(2)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::scale(s.ad_value(214), 2.0)), A::scale(s.ad_value(123), 2.0)), A::mul(s.ad_value(296), A::offset(A::add(assign32370_ad_e49594, A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)))), 1.0)));
            s.store_ad(20, &assign32370_ad_e49610);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1753] != 0.0))) {
            let assign32380_ad_e49663: A = A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::mul(A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0))));
            let assign32380_ad_e49722: A = A::mul(A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::add(A::sub(A::sub_from_scalar(1.0, A::div(s.ad_value(167), A::offset(s.ad_value(167), 1.0))), s.ad_value(171)), A::mul(A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::offset(s.ad_value(167), 1.0)), A::offset(A::div_from_scalar(1.0, A::offset(s.ad_value(3), 1.0)), 1.0))));
            let assign32380_ad_e49751: A = A::sub(A::sub(assign32380_ad_e49663, A::mul(s.ad_value(296), A::sub(A::sub(A::add(s.ad_value(173), s.ad_value(177)), A::mul(A::mul(A::scale(s.ad_value(176), 2.0), s.ad_value(175)), A::sub_from_scalar(1.0, A::mul(A::mul(s.ad_value(174), s.ad_value(175)), A::sub_from_scalar(5.0, A::mul(A::scale(s.ad_value(174), 4.0), s.ad_value(175))))))), assign32380_ad_e49722))), A::div(A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(167), 1.0))));
            s.store_offset_ad(21, assign32380_ad_e49751, 2.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1753] != 0.0))) {
            s.store_sub_ad_rhs(22, 123, A::mul(A::div(s.ad_value(19), s.ad_value(20)), A::offset(A::div(A::mul(s.ad_value(19), s.ad_value(21)), A::mul(A::scale(s.ad_value(20), 2.0), s.ad_value(20))), 1.0)));
        }

        if ((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) {
            s.copy_ad(123, 22);
        }

        s.v[1754] = if (((s.v[214]) as f64).abs() <= s.v[102]) { 1.0 } else { 0.0 };

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1754] != 0.0)) {
            s.store_scale_ad(167, A::square(s.ad_value(96)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1754] != 0.0)) {
            s.store_mul_ad(22, A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(99))), s.ad_value(294)), s.ad_value(167)), 1.0));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1754] != 0.0))) {
            s.store_scale_ad(167, A::add(A::tanh(A::scale(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0))), A::tanh(A::scale(A::add(s.ad_value(214), s.ad_value(103)), 5.0))), 0.5);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1754] != 0.0))) {
            s.store_mul_ad_lhs(45, A::mul(A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(99))), s.ad_value(294)), s.ad_value(35)), 1.0)), 167);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1754] != 0.0))) {
            s.store_limited_exp_ad(167, A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1754] != 0.0))) {
            let assign32470_ad_e49979: A = {
                if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))), 1.0))
                } else {
                    let assign32470_ad_e49978: A = {
                        if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271)))
                        } else {
                            {
                                if ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign32470_ad_e49978
                }
            };
            let assign32470_ad_e50061: A = {
                if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))), 1.0))
                } else {
                    let assign32470_ad_e50060: A = {
                        if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(271)), s.ad_value(6)), s.ad_value(271)))
                        } else {
                            {
                                if ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign32470_ad_e50060
                }
            };
            s.store_sub_ad(169, assign32470_ad_e49979, assign32470_ad_e50061);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1754] != 0.0))) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div(s.ad_value(169), A::offset(s.ad_value(3), 1.0))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1754] != 0.0))) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1754] != 0.0))) {
            s.store_limited_exp_ad(173, A::neg(s.ad_value(123)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1754] != 0.0))) {
            s.store_square(174, 123);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1754] != 0.0))) {
            s.store_div_from_scalar_ad(175, 1.0, A::offset(s.ad_value(174), 2.0));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1754] != 0.0))) {
            s.store_limited_exp_ad(176, A::neg(s.ad_value(97)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1754] != 0.0))) {
            s.store_limited_exp_ad(177, A::sub(s.ad_value(123), s.ad_value(97)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1754] != 0.0))) {
            let assign32550_ad_e50197: A = A::sub(A::sub(A::mul(A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123))), A::mul(A::mul(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170))), A::add(s.ad_value(216), s.ad_value(170)))), A::mul(s.ad_value(296), A::sub(A::add(A::add(A::add(A::sub(s.ad_value(173), s.ad_value(171)), s.ad_value(123)), s.ad_value(170)), s.ad_value(177)), A::mul(s.ad_value(176), A::add(A::offset(s.ad_value(123), 1.0), A::mul(s.ad_value(175), s.ad_value(174)))))));
            s.store_ad(19, &assign32550_ad_e50197);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1754] != 0.0))) {
            let assign32560_ad_e50269: A = A::sub(A::sub(A::add(s.ad_value(177), A::mul(s.ad_value(176), A::offset(A::add(A::mul(A::scale(s.ad_value(123), (-2.0)), s.ad_value(175)), A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(123), 2.0), s.ad_value(123)), s.ad_value(123)), s.ad_value(175)), s.ad_value(175))), (-1.0)))), s.ad_value(173)), A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))));
            let assign32560_ad_e50285: A = A::sub(A::add(A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170))), s.ad_value(2)), s.ad_value(2)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::scale(s.ad_value(214), 2.0)), A::scale(s.ad_value(123), 2.0)), A::mul(s.ad_value(296), A::offset(A::add(assign32560_ad_e50269, A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)))), 1.0)));
            s.store_ad(20, &assign32560_ad_e50285);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1754] != 0.0))) {
            let assign32570_ad_e50338: A = A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::mul(A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0))));
            let assign32570_ad_e50397: A = A::mul(A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::add(A::sub(A::sub_from_scalar(1.0, A::div(s.ad_value(167), A::offset(s.ad_value(167), 1.0))), s.ad_value(171)), A::mul(A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::offset(s.ad_value(167), 1.0)), A::offset(A::div_from_scalar(1.0, A::offset(s.ad_value(3), 1.0)), 1.0))));
            let assign32570_ad_e50426: A = A::sub(A::sub(assign32570_ad_e50338, A::mul(s.ad_value(296), A::sub(A::sub(A::add(s.ad_value(173), s.ad_value(177)), A::mul(A::mul(A::scale(s.ad_value(176), 2.0), s.ad_value(175)), A::sub_from_scalar(1.0, A::mul(A::mul(s.ad_value(174), s.ad_value(175)), A::sub_from_scalar(5.0, A::mul(A::scale(s.ad_value(174), 4.0), s.ad_value(175))))))), assign32570_ad_e50397))), A::div(A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(167), 1.0))));
            s.store_offset_ad(21, assign32570_ad_e50426, 2.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1754] != 0.0))) {
            s.store_sub_ad_rhs(22, 123, A::mul(A::div(s.ad_value(19), s.ad_value(20)), A::offset(A::div(A::mul(s.ad_value(19), s.ad_value(21)), A::mul(A::scale(s.ad_value(20), 2.0), s.ad_value(20))), 1.0)));
        }

        if ((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) {
            s.store_scale(50, 271, 3.912023005);
        }

        s.v[1755] = if (s.v[22] <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1755] != 0.0)) {
            s.store_scalar(306, 0.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1755] != 0.0)) {
            s.store_sub(51, 214, 22);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1755] != 0.0)) {
            s.store_mul(52, 51, 271);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1755] != 0.0)) {
            s.copy_ad(312, 50);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1755] != 0.0)) {
            s.store_scalar(458, 1.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1755] != 0.0)) {
            s.store_scalar(834, 1.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1755] != 0.0)) {
            s.store_scalar(853, 1.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1755] != 0.0)) {
            s.store_scalar(343, 1.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1755] != 0.0)) {
            s.store_scalar(339, 1.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1755] != 0.0)) {
            s.store_scalar(363, 1.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1755] != 0.0)) {
            s.store_scalar(365, 1.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1755] != 0.0)) {
            s.copy_ad(455, 453);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1755] != 0.0)) {
            s.copy_ad(454, 452);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_div_from_scalar_ad(54, 1.0, A::offset(A::square(s.ad_value(22)), 2.0));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_mul_ad_lhs(55, A::square(s.ad_value(22)), 54);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_ad(53, &A::limited_exp(s.ad_value(22)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_div_from_scalar(56, 1.0, 53);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_limited_exp_ad(53, A::sub(s.ad_value(22), s.ad_value(97)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_sub_ad_rhs(57, 53, A::mul(A::limited_exp(A::neg(s.ad_value(97))), A::add(A::offset(s.ad_value(22), 1.0), s.ad_value(55))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_sub_ad_lhs(58, A::mul(A::mul(A::sub(s.ad_value(214), s.ad_value(22)), A::sub(s.ad_value(214), s.ad_value(22))), A::div_from_scalar(1.0, s.ad_value(296))), 57);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_offset_ad(58, A::scale(A::add(A::offset(s.ad_value(58), (-0.001)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(58), (-0.001)), A::offset(s.ad_value(58), (-0.001))), ((4.0 * 1e-5) * 1e-5)))), 0.5), 0.001);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_sqrt(59, 58);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_mul_ad_rhs(61, 294, A::sqrt(A::add(s.ad_value(58), s.ad_value(57))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_div_ad(306, A::mul(A::mul(s.ad_value(296), s.ad_value(57)), s.ad_value(271)), A::add(s.ad_value(61), A::mul(s.ad_value(294), s.ad_value(59))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_mul_ad_lhs(247, A::mul(s.ad_value(59), s.ad_value(294)), 271);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.copy_ad(76, 56);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.copy_ad(78, 57);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_scalar(335, (1e-8 / (s.v[200] * p.p76)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_mul_ad_rhs(308, 335, A::add(s.ad_value(247), A::scale(s.ad_value(306), s.v[338])));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_limited_exp_ad(169, A::mul(s.ad_value(757), A::ln(A::max_with_scalar(A::scale(A::offset(A::div(s.ad_value(306), s.ad_value(247)), 1.0), 0.5), 1e-38))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_add_ad(170, A::mul(A::add(s.ad_value(750), A::mul(s.ad_value(760), s.ad_value(241))), A::pow(s.ad_value(308), s.ad_value(651))), A::div(s.ad_value(754), s.ad_value(169)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_offset(171, 170, 1.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_scale_ad(309, A::add(A::offset(s.ad_value(171), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(171), (-1.0)), A::offset(s.ad_value(171), (-1.0))), ((0.25 * 0.0015) * 0.0015)))), 0.5);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_div_from_scalar_ad(448, 1.0, A::scale(A::pow_from_scalar((s.v[183] * 1000000.0), s.ad_value(771)), p.p2));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_scale_ad(273, A::add(A::offset(A::sub(s.ad_value(298), s.ad_value(241)), 0.05), A::sqrt(A::offset(A::mul(A::offset(A::sub(s.ad_value(298), s.ad_value(241)), (-0.05)), A::offset(A::sub(s.ad_value(298), s.ad_value(241)), (-0.05))), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_sqrt(274, 273);
        }

        s.v[1756] = if (p.p33 == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1756] != 0.0)) {
            s.store_scalar(456, 0.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1756] != 0.0))) {
            s.store_offset_ad(167, A::mul(s.ad_value(770), s.ad_value(306)), 1.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1756] != 0.0))) {
            s.store_mul_ad_rhs(168, 787, A::sub(s.ad_value(274), s.ad_value(299)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1756] != 0.0))) {
            s.store_add_ad_lhs(169, A::div_from_scalar(1.0, s.ad_value(167)), 168);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1756] != 0.0))) {
            s.store_add_ad_rhs(170, 169, A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1756] != 0.0))) {
            s.store_mul_ad_lhs(456, A::scale(A::mul(A::add(s.ad_value(777), A::mul(s.ad_value(776), s.ad_value(170))), s.ad_value(448)), p.p2), 652);
        }

        s.v[1757] = if (p.p33 == 2.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_21(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1756] != 0.0))) && (s.v[1757] != 0.0)) {
            s.store_mul_ad_lhs(456, A::add(A::add(s.ad_value(452), A::scale(A::mul(A::add(s.ad_value(777), A::mul(s.ad_value(776), s.ad_value(170))), s.ad_value(448)), p.p2)), s.ad_value(453)), 652);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_mul_ad_lhs(310, A::div(A::scale(s.ad_value(746), 2.0), s.ad_value(740)), 309);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_scale(311, 310, s.v[184]);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_mul_ad_rhs(173, 742, A::add(s.ad_value(306), A::scale(s.ad_value(271), 2.0)));
        }

        s.v[1758] = if (s.v[456] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1758] != 0.0)) {
            s.store_scale(324, 746, (s.v[183] * s.v[199]));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1758] != 0.0)) {
            s.store_mul(167, 324, 456);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1758] != 0.0)) {
            s.store_scale(325, 167, 2.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1758] != 0.0)) {
            s.store_add_ad(326, A::add(s.ad_value(173), s.ad_value(311)), A::mul(A::scale(s.ad_value(173), 3.0), s.ad_value(167)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1758] != 0.0)) {
            s.store_mul_ad_rhs(327, 173, A::add(s.ad_value(311), A::mul(A::scale(s.ad_value(173), 2.0), s.ad_value(167))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1758] != 0.0)) {
            s.store_div_ad_lhs(312, A::sub(s.ad_value(326), A::sqrt(A::sub(A::square(s.ad_value(326)), A::mul(A::scale(s.ad_value(325), 2.0), s.ad_value(327))))), 325);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1758] != 0.0))) {
            s.store_div_ad(312, A::mul(s.ad_value(311), s.ad_value(173)), A::add(s.ad_value(311), s.ad_value(173)));
        }

        s.v[1759] = if ((p.p1349 == 0.0) && (p.p1350 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1759] != 0.0)) {
            s.store_scalar(1019, 1.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1759] != 0.0))) {
            s.store_div_from_scalar_ad(168, s.v[184], A::offset(A::sqrt(A::mul(s.ad_value(782), s.ad_value(275))), s.v[184]));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1759] != 0.0))) {
            s.store_offset_ad(1019, A::div(A::sub(A::scale(s.ad_value(168), p.p1349), A::mul(A::mul(A::scale(s.ad_value(168), p.p1350), A::powf(s.ad_value(306), p.p1351)), s.ad_value(271))), A::offset(A::scale(s.ad_value(241), p.p1352), 1.0)), 1.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1759] != 0.0))) {
            s.store_scale_ad(1019, A::add(A::offset(s.ad_value(1019), 0.1), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(1019), (-0.1)), A::offset(s.ad_value(1019), (-0.1))), ((0.25 * 0.0005) * 0.0005)))), 0.5);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_offset_ad(312, A::scale(A::add(A::offset(s.ad_value(312), (-0.001)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(312), (-0.001)), A::offset(s.ad_value(312), (-0.001))), ((4.0 * 1e-5) * 1e-5)))), 0.5), 0.001);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_div(312, 312, 1019);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_pow_ad(174, A::offset(A::div(s.ad_value(226), s.ad_value(312)), 1e-6), A::div_from_scalar(1.0, s.ad_value(656)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_pow_ad(175, A::offset(s.ad_value(174), 1.0), A::neg(s.ad_value(656)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_mul(315, 226, 175);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_mul_ad_lhs(318, A::add(s.ad_value(315), s.ad_value(232)), 272);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_add_ad_lhs(98, A::div(A::scale(s.ad_value(251), 2.0), s.ad_value(267)), 318);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_limited_exp_ad(100, A::neg(s.ad_value(98)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_scale(101, 95, 0.001);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_div_ad_lhs(167, A::scale(A::neg(s.ad_value(726)), s.v[184]), 300);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_mul_ad(168, A::mul(s.ad_value(725), A::add(A::limited_exp(A::scale(s.ad_value(167), 0.5)), A::scale(A::limited_exp(s.ad_value(167)), 2.0))), A::add(A::mul(s.ad_value(226), s.ad_value(272)), A::mul(s.ad_value(724), s.ad_value(272))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            let assign33330_ad_e51563: A = A::add(A::sub(A::add(A::div(A::mul(A::offset(A::scale(s.ad_value(743), 1.0 / (s.v[184])), 1.0), A::scale(s.ad_value(706), (1.602176462e-19 * (p.p74 * p.p74)))), A::scale(s.ad_value(271), (2.0 * s.v[180]))), A::div_from_scalar(p.p294, s.ad_value(271))), A::mul(A::mul(s.ad_value(3), s.ad_value(216)), s.ad_value(727))), A::mul(A::offset(s.ad_value(3), 1.0), s.ad_value(168)));
            s.store_ad(4, &assign33330_ad_e51563);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_add_ad_rhs(104, 4, A::mul(s.ad_value(294), A::sqrt(A::offset(A::add(A::limited_exp(A::neg(s.ad_value(4))), s.ad_value(4)), (-1.0)))));
        }

        s.v[1760] = if (s.v[4] < s.v[98]) { 1.0 } else { 0.0 };

        s.v[1761] = if (s.v[214] < s.v[104]) { 1.0 } else { 0.0 };

        s.v[1762] = if (((s.v[214]) as f64).abs() <= s.v[101]) { 1.0 } else { 0.0 };

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (s.v[1762] != 0.0)) {
            s.store_scale_ad(167, A::square(s.ad_value(96)), (0.16666666666666666 * 0.7071067811865475));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (s.v[1762] != 0.0)) {
            s.store_mul_ad(9, A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(100))), s.ad_value(294)), s.ad_value(167)), 1.0));
        }

        s.v[1763] = if (s.v[214] < (-s.v[101])) { 1.0 } else { 0.0 };

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_neg(10, 214);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_scaled_mul(11, 10, 96, 1.25);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_scale_ad(12, A::sub(A::offset(s.ad_value(11), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(11), (-6.0)), A::offset(s.ad_value(11), (-6.0))), 64.0))), 0.5);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_sub(13, 10, 12);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_add_ad(14, A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::offset(s.ad_value(12), 1.0)));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_sub_ad_lhs(16, A::scale(s.ad_value(13), 2.0), 296);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_add(0, 14, 16);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_add_ad(1, A::square(s.ad_value(0)), A::mul(s.ad_value(17), A::sub(A::scale(A::square(s.ad_value(16)), 0.5), s.ad_value(14))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_add_ad_rhs(18, 12, A::div(A::mul(A::mul(s.ad_value(14), s.ad_value(0)), s.ad_value(17)), A::add(s.ad_value(1), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17)), s.ad_value(17)), s.ad_value(16)), A::sub(A::scale(A::square(s.ad_value(16)), 0.3333333333333333), s.ad_value(14))))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_ad(28, &A::limited_exp(s.ad_value(18)));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_div_from_scalar(29, 1.0, 28);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_div_from_scalar_ad(13, 1.0, A::offset(A::square(s.ad_value(18)), 2.0));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_mul_ad_lhs(30, A::square(s.ad_value(18)), 13);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_scale_ad(31, A::mul(A::mul(s.ad_value(18), s.ad_value(13)), s.ad_value(13)), 4.0);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_mul_ad_lhs(32, A::mul(A::sub(A::scale(s.ad_value(13), 8.0), A::scale(s.ad_value(30), 12.0)), s.ad_value(13)), 13);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_sub(13, 10, 18);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_mul(33, 100, 29);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_add_ad(36, A::scale(s.ad_value(13), 2.0), A::mul(s.ad_value(296), A::add(A::sub(A::offset(s.ad_value(28), (-1.0)), s.ad_value(33)), A::mul(s.ad_value(100), A::sub_from_scalar(1.0, s.ad_value(31))))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_sub_ad(37, A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::add(A::add(A::offset(A::sub(s.ad_value(28), s.ad_value(18)), (-1.0)), s.ad_value(33)), A::mul(s.ad_value(100), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30))))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::sub(A::add(s.ad_value(28), s.ad_value(33)), A::mul(s.ad_value(100), s.ad_value(32)))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_sub_ad(13, A::square(s.ad_value(36)), A::scale(A::mul(s.ad_value(37), s.ad_value(13)), 2.0));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_sub_ad(9, A::neg(s.ad_value(18)), A::scale(A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_div_from_scalar_ad(38, 1.0, A::offset(A::scale(s.ad_value(294), 0.7324648775608221), 1.25));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_mul_ad_lhs(39, A::offset(A::mul(A::scale(s.ad_value(95), 1.25), s.ad_value(38)), (-1.0)), 38);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_mul_ad(40, A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_limited_exp_ad(13, A::neg(s.ad_value(40)));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_sub_from_scalar(41, 1.0, 13);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_sub_ad(42, A::add(s.ad_value(214), A::scale(s.ad_value(296), 0.5)), A::mul(s.ad_value(294), A::sqrt(A::sub(A::add(s.ad_value(214), A::scale(s.ad_value(296), 0.25)), s.ad_value(41)))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_offset(43, 98, 3.0);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_sub_ad(12, A::scale(A::sub(A::add(s.ad_value(42), s.ad_value(43)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(42), s.ad_value(43)), A::sub(s.ad_value(42), s.ad_value(43))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(43), A::sqrt(A::offset(A::square(s.ad_value(43)), 5.0))), 0.5));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_sub(13, 214, 12);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_limited_exp_ad(33, A::neg(s.ad_value(12)));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_div_from_scalar_ad(34, 1.0, A::offset(A::square(s.ad_value(12)), 2.0));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_mul_ad_lhs(30, A::square(s.ad_value(12)), 34);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_scale_ad(31, A::mul(A::mul(s.ad_value(12), s.ad_value(34)), s.ad_value(34)), 4.0);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_mul_ad_lhs(32, A::mul(A::sub(A::scale(s.ad_value(34), 8.0), A::scale(s.ad_value(30), 12.0)), s.ad_value(34)), 34);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_max_from_scalar_ad(14, 1e-40, A::sub(A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::sub(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), A::mul(s.ad_value(100), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)))))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_sub_from_scalar_ad(15, 1.0, A::scale(A::mul(s.ad_value(296), A::sub(s.ad_value(33), A::mul(s.ad_value(100), s.ad_value(32)))), 0.5));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_add_ad(16, A::scale(s.ad_value(13), 2.0), A::mul(s.ad_value(296), A::sub(A::sub_from_scalar(1.0, s.ad_value(33)), A::mul(s.ad_value(100), A::offset(s.ad_value(31), 1.0)))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_add_ad(17, A::sub(s.ad_value(98), s.ad_value(12)), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_add(0, 14, 16);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_add_ad(1, A::square(s.ad_value(0)), A::mul(s.ad_value(17), A::sub(A::scale(A::square(s.ad_value(16)), 0.5), A::mul(s.ad_value(14), s.ad_value(15)))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_add_ad_rhs(44, 12, A::div(A::mul(A::mul(s.ad_value(14), s.ad_value(0)), s.ad_value(17)), A::add(s.ad_value(1), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17)), s.ad_value(17)), s.ad_value(16)), A::sub(A::scale(A::square(s.ad_value(16)), 0.3333333333333333), A::mul(s.ad_value(14), s.ad_value(15)))))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_ad(28, &A::limited_exp(s.ad_value(44)));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_div_from_scalar(29, 1.0, 28);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_limited_exp_ad(28, A::sub(s.ad_value(44), s.ad_value(98)));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_div_from_scalar_ad(13, 1.0, A::offset(A::square(s.ad_value(44)), 2.0));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_mul_ad_lhs(30, A::square(s.ad_value(44)), 13);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_scale_ad(31, A::mul(A::mul(s.ad_value(44), s.ad_value(13)), s.ad_value(13)), 4.0);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_mul_ad_lhs(32, A::mul(A::sub(A::scale(s.ad_value(13), 8.0), A::scale(s.ad_value(30), 12.0)), s.ad_value(13)), 13);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_sub(13, 214, 44);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_add_ad(36, A::scale(s.ad_value(13), 2.0), A::mul(s.ad_value(296), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(29)), s.ad_value(28)), A::mul(s.ad_value(100), A::offset(s.ad_value(31), 1.0)))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_sub_ad(37, A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::sub(A::add(A::offset(A::add(s.ad_value(29), s.ad_value(44)), (-1.0)), s.ad_value(28)), A::mul(s.ad_value(100), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30))))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::sub(A::add(s.ad_value(29), s.ad_value(28)), A::mul(s.ad_value(100), s.ad_value(32)))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_sub_ad(13, A::square(s.ad_value(36)), A::scale(A::mul(s.ad_value(37), s.ad_value(13)), 2.0));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (s.v[1761] != 0.0)) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_add_ad_rhs(9, 44, A::scale(A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.copy_ad(47, 2);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_square(48, 47);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_sub_ad_rhs(8, 4, A::mul(s.ad_value(46), s.ad_value(272)));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_sub_ad_rhs(105, 214, A::mul(s.ad_value(294), A::sqrt(A::offset(A::add(A::limited_exp(A::neg(s.ad_value(8))), s.ad_value(8)), (-1.0)))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_offset(43, 98, 3.0);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_scale_ad(106, A::sub(A::add(s.ad_value(105), s.ad_value(43)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(105), s.ad_value(43)), A::sub(s.ad_value(105), s.ad_value(43))), 40.0))), 0.5);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_sub_ad(107, A::sub(A::mul(A::sub(s.ad_value(214), s.ad_value(106)), A::sub(s.ad_value(214), s.ad_value(106))), A::mul(A::mul(s.ad_value(48), A::add(A::sub(s.ad_value(216), s.ad_value(106)), s.ad_value(4))), A::add(A::sub(s.ad_value(216), s.ad_value(106)), s.ad_value(4)))), A::mul(s.ad_value(296), s.ad_value(4)));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_sub_ad(108, A::scale(A::sub(s.ad_value(214), s.ad_value(106)), 2.0), A::mul(A::scale(s.ad_value(48), 2.0), A::add(A::sub(s.ad_value(216), s.ad_value(106)), s.ad_value(4))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_square(109, 108);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_sub_from_scalar(110, 1.0, 48);
        }

        s.v[1764] = if (s.v[107] < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) && (s.v[1764] != 0.0)) {
            s.store_scalar(107, 0.0);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_add_ad(49, A::sub(s.ad_value(98), s.ad_value(106)), A::ln(A::max_with_scalar(A::mul(s.ad_value(107), s.ad_value(297)), 1e-38)));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_add(111, 107, 108);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_square(112, 111);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_sub_ad(113, A::add(A::div(s.ad_value(112), s.ad_value(49)), A::scale(s.ad_value(109), 0.5)), A::mul(s.ad_value(107), s.ad_value(110)));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_div_ad_lhs(114, A::mul(s.ad_value(108), s.ad_value(111)), 113);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_sub_ad(115, A::scale(s.ad_value(109), 0.3333333333333333), A::mul(s.ad_value(107), s.ad_value(110)));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_div_ad(116, A::mul(s.ad_value(111), s.ad_value(107)), A::add(s.ad_value(113), A::mul(s.ad_value(114), s.ad_value(115))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_add(117, 106, 116);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_limited_exp_ad(118, A::sub(s.ad_value(117), s.ad_value(98)));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_add_ad(119, A::sub(A::scale(A::sub(s.ad_value(214), s.ad_value(117)), 2.0), A::mul(A::scale(s.ad_value(48), 2.0), A::add(A::sub(s.ad_value(216), s.ad_value(117)), s.ad_value(4)))), A::mul(s.ad_value(296), s.ad_value(118)));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_sub_ad(120, A::sub(A::mul(A::sub(s.ad_value(214), s.ad_value(117)), A::sub(s.ad_value(214), s.ad_value(117))), A::mul(A::mul(s.ad_value(48), A::add(A::sub(s.ad_value(216), s.ad_value(117)), s.ad_value(4))), A::add(A::sub(s.ad_value(216), s.ad_value(117)), s.ad_value(4)))), A::mul(s.ad_value(296), A::add(s.ad_value(4), s.ad_value(118))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_mul_ad(121, A::scale(s.ad_value(120), 2.0), A::sub(A::sub_from_scalar(2.0, A::scale(s.ad_value(48), 2.0)), A::mul(s.ad_value(296), s.ad_value(118))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_div_ad(122, A::scale(s.ad_value(120), 2.0), A::add(s.ad_value(119), A::sqrt(A::sub(A::square(s.ad_value(119)), s.ad_value(121)))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1760] != 0.0)) && (!(s.v[1761] != 0.0))) {
            s.store_add(9, 117, 122);
        }

        s.v[1765] = if (((s.v[214]) as f64).abs() <= s.v[101]) { 1.0 } else { 0.0 };

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (s.v[1765] != 0.0)) {
            s.store_scale_ad(167, A::square(s.ad_value(96)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (s.v[1765] != 0.0)) {
            s.store_mul_ad(9, A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(100))), s.ad_value(294)), s.ad_value(167)), 1.0));
        }

        s.v[1766] = if (s.v[214] < (-s.v[101])) { 1.0 } else { 0.0 };

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (s.v[1766] != 0.0)) {
            s.store_neg(10, 214);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (s.v[1766] != 0.0)) {
            s.store_scaled_mul(11, 10, 96, 1.25);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (s.v[1766] != 0.0)) {
            s.store_scale_ad(12, A::sub(A::offset(s.ad_value(11), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(11), (-6.0)), A::offset(s.ad_value(11), (-6.0))), 64.0))), 0.5);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (s.v[1766] != 0.0)) {
            s.store_sub(13, 10, 12);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (s.v[1766] != 0.0)) {
            s.store_add_ad(14, A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::offset(s.ad_value(12), 1.0)));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (s.v[1766] != 0.0)) {
            s.store_sub_ad_lhs(16, A::scale(s.ad_value(13), 2.0), 296);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (s.v[1766] != 0.0)) {
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (s.v[1766] != 0.0)) {
            s.store_add(0, 14, 16);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (s.v[1766] != 0.0)) {
            s.store_add_ad(1, A::square(s.ad_value(0)), A::mul(s.ad_value(17), A::sub(A::scale(A::square(s.ad_value(16)), 0.5), s.ad_value(14))));
        }

    }

    pub(super) fn stamp_reactive_block_22(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (s.v[1766] != 0.0)) {
            s.store_add_ad_rhs(18, 12, A::div(A::mul(A::mul(s.ad_value(14), s.ad_value(0)), s.ad_value(17)), A::add(s.ad_value(1), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17)), s.ad_value(17)), s.ad_value(16)), A::sub(A::scale(A::square(s.ad_value(16)), 0.3333333333333333), s.ad_value(14))))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (s.v[1766] != 0.0)) {
            s.store_ad(28, &A::limited_exp(s.ad_value(18)));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (s.v[1766] != 0.0)) {
            s.store_div_from_scalar(29, 1.0, 28);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (s.v[1766] != 0.0)) {
            s.store_div_from_scalar_ad(13, 1.0, A::offset(A::square(s.ad_value(18)), 2.0));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (s.v[1766] != 0.0)) {
            s.store_mul_ad_lhs(30, A::square(s.ad_value(18)), 13);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (s.v[1766] != 0.0)) {
            s.store_scale_ad(31, A::mul(A::mul(s.ad_value(18), s.ad_value(13)), s.ad_value(13)), 4.0);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (s.v[1766] != 0.0)) {
            s.store_mul_ad_lhs(32, A::mul(A::sub(A::scale(s.ad_value(13), 8.0), A::scale(s.ad_value(30), 12.0)), s.ad_value(13)), 13);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (s.v[1766] != 0.0)) {
            s.store_sub(13, 10, 18);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (s.v[1766] != 0.0)) {
            s.store_mul(33, 100, 29);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (s.v[1766] != 0.0)) {
            s.store_add_ad(36, A::scale(s.ad_value(13), 2.0), A::mul(s.ad_value(296), A::add(A::sub(A::offset(s.ad_value(28), (-1.0)), s.ad_value(33)), A::mul(s.ad_value(100), A::sub_from_scalar(1.0, s.ad_value(31))))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (s.v[1766] != 0.0)) {
            s.store_sub_ad(37, A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::add(A::add(A::offset(A::sub(s.ad_value(28), s.ad_value(18)), (-1.0)), s.ad_value(33)), A::mul(s.ad_value(100), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30))))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (s.v[1766] != 0.0)) {
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::sub(A::add(s.ad_value(28), s.ad_value(33)), A::mul(s.ad_value(100), s.ad_value(32)))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (s.v[1766] != 0.0)) {
            s.store_sub_ad(13, A::square(s.ad_value(36)), A::scale(A::mul(s.ad_value(37), s.ad_value(13)), 2.0));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (s.v[1766] != 0.0)) {
            s.store_sub_ad(9, A::neg(s.ad_value(18)), A::scale(A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_div_from_scalar_ad(38, 1.0, A::offset(A::scale(s.ad_value(294), 0.7324648775608221), 1.25));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_mul_ad_lhs(39, A::offset(A::mul(A::scale(s.ad_value(95), 1.25), s.ad_value(38)), (-1.0)), 38);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_mul_ad(40, A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_limited_exp_ad(13, A::neg(s.ad_value(40)));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_sub_from_scalar(41, 1.0, 13);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_sub_ad(42, A::add(s.ad_value(214), A::scale(s.ad_value(296), 0.5)), A::mul(s.ad_value(294), A::sqrt(A::sub(A::add(s.ad_value(214), A::scale(s.ad_value(296), 0.25)), s.ad_value(41)))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_offset(43, 98, 3.0);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_sub_ad(12, A::scale(A::sub(A::add(s.ad_value(42), s.ad_value(43)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(42), s.ad_value(43)), A::sub(s.ad_value(42), s.ad_value(43))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(43), A::sqrt(A::offset(A::square(s.ad_value(43)), 5.0))), 0.5));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_sub(13, 214, 12);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_limited_exp_ad(33, A::neg(s.ad_value(12)));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_div_from_scalar_ad(34, 1.0, A::offset(A::square(s.ad_value(12)), 2.0));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_mul_ad_lhs(30, A::square(s.ad_value(12)), 34);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_scale_ad(31, A::mul(A::mul(s.ad_value(12), s.ad_value(34)), s.ad_value(34)), 4.0);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_mul_ad_lhs(32, A::mul(A::sub(A::scale(s.ad_value(34), 8.0), A::scale(s.ad_value(30), 12.0)), s.ad_value(34)), 34);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_max_from_scalar_ad(14, 1e-40, A::sub(A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::sub(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), A::mul(s.ad_value(100), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)))))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_sub_from_scalar_ad(15, 1.0, A::scale(A::mul(s.ad_value(296), A::sub(s.ad_value(33), A::mul(s.ad_value(100), s.ad_value(32)))), 0.5));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_add_ad(16, A::scale(s.ad_value(13), 2.0), A::mul(s.ad_value(296), A::sub(A::sub_from_scalar(1.0, s.ad_value(33)), A::mul(s.ad_value(100), A::offset(s.ad_value(31), 1.0)))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_add_ad(17, A::sub(s.ad_value(98), s.ad_value(12)), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_add(0, 14, 16);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_add_ad(1, A::square(s.ad_value(0)), A::mul(s.ad_value(17), A::sub(A::scale(A::square(s.ad_value(16)), 0.5), A::mul(s.ad_value(14), s.ad_value(15)))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_add_ad_rhs(44, 12, A::div(A::mul(A::mul(s.ad_value(14), s.ad_value(0)), s.ad_value(17)), A::add(s.ad_value(1), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17)), s.ad_value(17)), s.ad_value(16)), A::sub(A::scale(A::square(s.ad_value(16)), 0.3333333333333333), A::mul(s.ad_value(14), s.ad_value(15)))))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_ad(28, &A::limited_exp(s.ad_value(44)));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_div_from_scalar(29, 1.0, 28);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_limited_exp_ad(28, A::sub(s.ad_value(44), s.ad_value(98)));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_div_from_scalar_ad(13, 1.0, A::offset(A::square(s.ad_value(44)), 2.0));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_mul_ad_lhs(30, A::square(s.ad_value(44)), 13);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_scale_ad(31, A::mul(A::mul(s.ad_value(44), s.ad_value(13)), s.ad_value(13)), 4.0);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_mul_ad_lhs(32, A::mul(A::sub(A::scale(s.ad_value(13), 8.0), A::scale(s.ad_value(30), 12.0)), s.ad_value(13)), 13);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_sub(13, 214, 44);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_add_ad(36, A::scale(s.ad_value(13), 2.0), A::mul(s.ad_value(296), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(29)), s.ad_value(28)), A::mul(s.ad_value(100), A::offset(s.ad_value(31), 1.0)))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_sub_ad(37, A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::sub(A::add(A::offset(A::add(s.ad_value(29), s.ad_value(44)), (-1.0)), s.ad_value(28)), A::mul(s.ad_value(100), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30))))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::sub(A::add(s.ad_value(29), s.ad_value(28)), A::mul(s.ad_value(100), s.ad_value(32)))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_sub_ad(13, A::square(s.ad_value(36)), A::scale(A::mul(s.ad_value(37), s.ad_value(13)), 2.0));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1760] != 0.0))) && (!(s.v[1765] != 0.0))) && (!(s.v[1766] != 0.0))) {
            s.store_add_ad_rhs(9, 44, A::scale(A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.copy_ad(123, 9);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_scalar(102, 1e-7);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_scalar(103, 2.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_scale_ad(35, A::square(s.ad_value(96)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_offset_ad(24, A::scale(A::mul(A::offset(A::scale(s.ad_value(743), 1.0 / (s.v[184])), 1.0), A::scale(s.ad_value(706), (1.602176462e-19 * (p.p74 * p.p74)))), 1.0 / ((2.0 * s.v[180]))), p.p294);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_div_ad_lhs(167, A::scale(A::neg(s.ad_value(726)), s.v[184]), 300);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_mul_ad(168, A::mul(s.ad_value(725), A::add(A::limited_exp(A::scale(s.ad_value(167), 0.5)), A::scale(A::limited_exp(s.ad_value(167)), 2.0))), A::add(A::mul(s.ad_value(226), s.ad_value(272)), A::mul(s.ad_value(724), s.ad_value(272))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_add_ad(6, A::add(A::sub(s.ad_value(24), A::mul(A::mul(s.ad_value(3), A::mul(s.ad_value(216), s.ad_value(271))), s.ad_value(727))), A::mul(A::mul(A::offset(s.ad_value(3), 1.0), s.ad_value(168)), s.ad_value(271))), A::mul(A::offset(s.ad_value(3), 1.0), s.ad_value(46)));
        }

        s.v[1767] = if (((s.v[214]) as f64).abs() <= s.v[102]) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1767] != 0.0)) {
            s.store_scale_ad(167, A::square(s.ad_value(96)), (0.16666666666666666 * 0.7071067811865475));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1767] != 0.0)) {
            s.store_mul_ad(23, A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(99))), s.ad_value(294)), s.ad_value(167)), 1.0));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1767] != 0.0))) {
            s.store_scale_ad(167, A::add(A::tanh(A::scale(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0))), A::tanh(A::scale(A::add(s.ad_value(214), s.ad_value(103)), 5.0))), 0.5);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1767] != 0.0))) {
            s.store_mul_ad_lhs(45, A::mul(A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(99))), s.ad_value(294)), s.ad_value(35)), 1.0)), 167);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1767] != 0.0))) {
            s.store_limited_exp_ad(167, A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1767] != 0.0))) {
            let assign34990_ad_e55610: A = {
                if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))), 1.0))
                } else {
                    let assign34990_ad_e55609: A = {
                        if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271)))
                        } else {
                            {
                                if ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign34990_ad_e55609
                }
            };
            let assign34990_ad_e55692: A = {
                if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))), 1.0))
                } else {
                    let assign34990_ad_e55691: A = {
                        if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(271)), s.ad_value(6)), s.ad_value(271)))
                        } else {
                            {
                                if ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign34990_ad_e55691
                }
            };
            s.store_sub_ad(169, assign34990_ad_e55610, assign34990_ad_e55692);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1767] != 0.0))) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div(s.ad_value(169), A::offset(s.ad_value(3), 1.0))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1767] != 0.0))) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1767] != 0.0))) {
            s.store_limited_exp_ad(173, A::neg(s.ad_value(123)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1767] != 0.0))) {
            s.store_square(174, 123);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1767] != 0.0))) {
            s.store_div_from_scalar_ad(175, 1.0, A::offset(s.ad_value(174), 2.0));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1767] != 0.0))) {
            s.store_limited_exp_ad(176, A::neg(s.ad_value(98)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1767] != 0.0))) {
            s.store_limited_exp_ad(177, A::sub(s.ad_value(123), s.ad_value(98)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1767] != 0.0))) {
            let assign35070_ad_e55852: A = A::sub(A::sub(A::mul(A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123))), A::mul(A::mul(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170))), A::add(s.ad_value(216), s.ad_value(170)))), A::mul(s.ad_value(296), A::sub(A::add(A::add(A::add(A::sub(s.ad_value(173), s.ad_value(171)), s.ad_value(123)), s.ad_value(170)), s.ad_value(177)), A::mul(s.ad_value(176), A::add(A::offset(s.ad_value(123), 1.0), A::mul(s.ad_value(175), s.ad_value(174)))))));
            s.store_ad(19, &assign35070_ad_e55852);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1767] != 0.0))) {
            let assign35080_ad_e55927: A = A::sub(A::sub(A::add(s.ad_value(177), A::mul(s.ad_value(176), A::offset(A::add(A::mul(A::scale(s.ad_value(123), (-2.0)), s.ad_value(175)), A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(123), 2.0), s.ad_value(123)), s.ad_value(123)), s.ad_value(175)), s.ad_value(175))), (-1.0)))), s.ad_value(173)), A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))));
            let assign35080_ad_e55943: A = A::sub(A::add(A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170))), s.ad_value(2)), s.ad_value(2)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::scale(s.ad_value(214), 2.0)), A::scale(s.ad_value(123), 2.0)), A::mul(s.ad_value(296), A::offset(A::add(assign35080_ad_e55927, A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)))), 1.0)));
            s.store_ad(20, &assign35080_ad_e55943);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1767] != 0.0))) {
            let assign35090_ad_e55999: A = A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::mul(A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0))));
            let assign35090_ad_e56058: A = A::mul(A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::add(A::sub(A::sub_from_scalar(1.0, A::div(s.ad_value(167), A::offset(s.ad_value(167), 1.0))), s.ad_value(171)), A::mul(A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::offset(s.ad_value(167), 1.0)), A::offset(A::div_from_scalar(1.0, A::offset(s.ad_value(3), 1.0)), 1.0))));
            let assign35090_ad_e56087: A = A::sub(A::sub(assign35090_ad_e55999, A::mul(s.ad_value(296), A::sub(A::sub(A::add(s.ad_value(173), s.ad_value(177)), A::mul(A::mul(A::scale(s.ad_value(176), 2.0), s.ad_value(175)), A::sub_from_scalar(1.0, A::mul(A::mul(s.ad_value(174), s.ad_value(175)), A::sub_from_scalar(5.0, A::mul(A::scale(s.ad_value(174), 4.0), s.ad_value(175))))))), assign35090_ad_e56058))), A::div(A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(167), 1.0))));
            s.store_offset_ad(21, assign35090_ad_e56087, 2.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1767] != 0.0))) {
            s.store_sub_ad_rhs(23, 123, A::mul(A::div(s.ad_value(19), s.ad_value(20)), A::offset(A::div(A::mul(s.ad_value(19), s.ad_value(21)), A::mul(A::scale(s.ad_value(20), 2.0), s.ad_value(20))), 1.0)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.copy_ad(123, 23);
        }

        s.v[1768] = if (((s.v[214]) as f64).abs() <= s.v[102]) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1768] != 0.0)) {
            s.store_scale_ad(167, A::square(s.ad_value(96)), (0.16666666666666666 * 0.7071067811865475));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1768] != 0.0)) {
            s.store_mul_ad(23, A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(99))), s.ad_value(294)), s.ad_value(167)), 1.0));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1768] != 0.0))) {
            s.store_scale_ad(167, A::add(A::tanh(A::scale(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0))), A::tanh(A::scale(A::add(s.ad_value(214), s.ad_value(103)), 5.0))), 0.5);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1768] != 0.0))) {
            s.store_mul_ad_lhs(45, A::mul(A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(100))), s.ad_value(294)), s.ad_value(35)), 1.0)), 167);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1768] != 0.0))) {
            s.store_limited_exp_ad(167, A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1768] != 0.0))) {
            let assign35180_ad_e56339: A = {
                if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))), 1.0))
                } else {
                    let assign35180_ad_e56338: A = {
                        if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271)))
                        } else {
                            {
                                if ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign35180_ad_e56338
                }
            };
            let assign35180_ad_e56421: A = {
                if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))), 1.0))
                } else {
                    let assign35180_ad_e56420: A = {
                        if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(271)), s.ad_value(6)), s.ad_value(271)))
                        } else {
                            {
                                if ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign35180_ad_e56420
                }
            };
            s.store_sub_ad(169, assign35180_ad_e56339, assign35180_ad_e56421);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1768] != 0.0))) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div(s.ad_value(169), A::offset(s.ad_value(3), 1.0))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1768] != 0.0))) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1768] != 0.0))) {
            s.store_limited_exp_ad(173, A::neg(s.ad_value(123)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1768] != 0.0))) {
            s.store_square(174, 123);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1768] != 0.0))) {
            s.store_div_from_scalar_ad(175, 1.0, A::offset(s.ad_value(174), 2.0));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1768] != 0.0))) {
            s.store_limited_exp_ad(176, A::neg(s.ad_value(98)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1768] != 0.0))) {
            s.store_limited_exp_ad(177, A::sub(s.ad_value(123), s.ad_value(98)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1768] != 0.0))) {
            let assign35260_ad_e56581: A = A::sub(A::sub(A::mul(A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123))), A::mul(A::mul(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170))), A::add(s.ad_value(216), s.ad_value(170)))), A::mul(s.ad_value(296), A::sub(A::add(A::add(A::add(A::sub(s.ad_value(173), s.ad_value(171)), s.ad_value(123)), s.ad_value(170)), s.ad_value(177)), A::mul(s.ad_value(176), A::add(A::offset(s.ad_value(123), 1.0), A::mul(s.ad_value(175), s.ad_value(174)))))));
            s.store_ad(19, &assign35260_ad_e56581);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1768] != 0.0))) {
            let assign35270_ad_e56656: A = A::sub(A::sub(A::add(s.ad_value(177), A::mul(s.ad_value(176), A::offset(A::add(A::mul(A::scale(s.ad_value(123), (-2.0)), s.ad_value(175)), A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(123), 2.0), s.ad_value(123)), s.ad_value(123)), s.ad_value(175)), s.ad_value(175))), (-1.0)))), s.ad_value(173)), A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))));
            let assign35270_ad_e56672: A = A::sub(A::add(A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170))), s.ad_value(2)), s.ad_value(2)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::scale(s.ad_value(214), 2.0)), A::scale(s.ad_value(123), 2.0)), A::mul(s.ad_value(296), A::offset(A::add(assign35270_ad_e56656, A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)))), 1.0)));
            s.store_ad(20, &assign35270_ad_e56672);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1768] != 0.0))) {
            let assign35280_ad_e56728: A = A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::mul(A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0))));
            let assign35280_ad_e56787: A = A::mul(A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::add(A::sub(A::sub_from_scalar(1.0, A::div(s.ad_value(167), A::offset(s.ad_value(167), 1.0))), s.ad_value(171)), A::mul(A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::offset(s.ad_value(167), 1.0)), A::offset(A::div_from_scalar(1.0, A::offset(s.ad_value(3), 1.0)), 1.0))));
            let assign35280_ad_e56816: A = A::sub(A::sub(assign35280_ad_e56728, A::mul(s.ad_value(296), A::sub(A::sub(A::add(s.ad_value(173), s.ad_value(177)), A::mul(A::mul(A::scale(s.ad_value(176), 2.0), s.ad_value(175)), A::sub_from_scalar(1.0, A::mul(A::mul(s.ad_value(174), s.ad_value(175)), A::sub_from_scalar(5.0, A::mul(A::scale(s.ad_value(174), 4.0), s.ad_value(175))))))), assign35280_ad_e56787))), A::div(A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(167), 1.0))));
            s.store_offset_ad(21, assign35280_ad_e56816, 2.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1768] != 0.0))) {
            s.store_sub_ad_rhs(23, 123, A::mul(A::div(s.ad_value(19), s.ad_value(20)), A::offset(A::div(A::mul(s.ad_value(19), s.ad_value(21)), A::mul(A::scale(s.ad_value(20), 2.0), s.ad_value(20))), 1.0)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.copy_ad(123, 23);
        }

        s.v[1769] = if (((s.v[214]) as f64).abs() <= s.v[102]) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1769] != 0.0)) {
            s.store_scale_ad(167, A::square(s.ad_value(96)), (0.16666666666666666 * 0.7071067811865475));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1769] != 0.0)) {
            s.store_mul_ad(23, A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(99))), s.ad_value(294)), s.ad_value(167)), 1.0));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1769] != 0.0))) {
            s.store_scale_ad(167, A::add(A::tanh(A::scale(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0))), A::tanh(A::scale(A::add(s.ad_value(214), s.ad_value(103)), 5.0))), 0.5);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1769] != 0.0))) {
            s.store_mul_ad_lhs(45, A::mul(A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(100))), s.ad_value(294)), s.ad_value(35)), 1.0)), 167);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1769] != 0.0))) {
            s.store_limited_exp_ad(167, A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271)));
        }

    }

    pub(super) fn stamp_reactive_block_23(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1769] != 0.0))) {
            let assign35370_ad_e57068: A = {
                if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))), 1.0))
                } else {
                    let assign35370_ad_e57067: A = {
                        if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271)))
                        } else {
                            {
                                if ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign35370_ad_e57067
                }
            };
            let assign35370_ad_e57150: A = {
                if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))), 1.0))
                } else {
                    let assign35370_ad_e57149: A = {
                        if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(271)), s.ad_value(6)), s.ad_value(271)))
                        } else {
                            {
                                if ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign35370_ad_e57149
                }
            };
            s.store_sub_ad(169, assign35370_ad_e57068, assign35370_ad_e57150);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1769] != 0.0))) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div(s.ad_value(169), A::offset(s.ad_value(3), 1.0))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1769] != 0.0))) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1769] != 0.0))) {
            s.store_limited_exp_ad(173, A::neg(s.ad_value(123)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1769] != 0.0))) {
            s.store_square(174, 123);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1769] != 0.0))) {
            s.store_div_from_scalar_ad(175, 1.0, A::offset(s.ad_value(174), 2.0));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1769] != 0.0))) {
            s.store_limited_exp_ad(176, A::neg(s.ad_value(98)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1769] != 0.0))) {
            s.store_limited_exp_ad(177, A::sub(s.ad_value(123), s.ad_value(98)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1769] != 0.0))) {
            let assign35450_ad_e57310: A = A::sub(A::sub(A::mul(A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123))), A::mul(A::mul(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170))), A::add(s.ad_value(216), s.ad_value(170)))), A::mul(s.ad_value(296), A::sub(A::add(A::add(A::add(A::sub(s.ad_value(173), s.ad_value(171)), s.ad_value(123)), s.ad_value(170)), s.ad_value(177)), A::mul(s.ad_value(176), A::add(A::offset(s.ad_value(123), 1.0), A::mul(s.ad_value(175), s.ad_value(174)))))));
            s.store_ad(19, &assign35450_ad_e57310);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1769] != 0.0))) {
            let assign35460_ad_e57385: A = A::sub(A::sub(A::add(s.ad_value(177), A::mul(s.ad_value(176), A::offset(A::add(A::mul(A::scale(s.ad_value(123), (-2.0)), s.ad_value(175)), A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(123), 2.0), s.ad_value(123)), s.ad_value(123)), s.ad_value(175)), s.ad_value(175))), (-1.0)))), s.ad_value(173)), A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))));
            let assign35460_ad_e57401: A = A::sub(A::add(A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170))), s.ad_value(2)), s.ad_value(2)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::scale(s.ad_value(214), 2.0)), A::scale(s.ad_value(123), 2.0)), A::mul(s.ad_value(296), A::offset(A::add(assign35460_ad_e57385, A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)))), 1.0)));
            s.store_ad(20, &assign35460_ad_e57401);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1769] != 0.0))) {
            let assign35470_ad_e57457: A = A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::mul(A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0))));
            let assign35470_ad_e57516: A = A::mul(A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::add(A::sub(A::sub_from_scalar(1.0, A::div(s.ad_value(167), A::offset(s.ad_value(167), 1.0))), s.ad_value(171)), A::mul(A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::offset(s.ad_value(167), 1.0)), A::offset(A::div_from_scalar(1.0, A::offset(s.ad_value(3), 1.0)), 1.0))));
            let assign35470_ad_e57545: A = A::sub(A::sub(assign35470_ad_e57457, A::mul(s.ad_value(296), A::sub(A::sub(A::add(s.ad_value(173), s.ad_value(177)), A::mul(A::mul(A::scale(s.ad_value(176), 2.0), s.ad_value(175)), A::sub_from_scalar(1.0, A::mul(A::mul(s.ad_value(174), s.ad_value(175)), A::sub_from_scalar(5.0, A::mul(A::scale(s.ad_value(174), 4.0), s.ad_value(175))))))), assign35470_ad_e57516))), A::div(A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(167), 1.0))));
            s.store_offset_ad(21, assign35470_ad_e57545, 2.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (!(s.v[1769] != 0.0))) {
            s.store_sub_ad_rhs(23, 123, A::mul(A::div(s.ad_value(19), s.ad_value(20)), A::offset(A::div(A::mul(s.ad_value(19), s.ad_value(21)), A::mul(A::scale(s.ad_value(20), 2.0), s.ad_value(20))), 1.0)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_sub(62, 23, 22);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_mul(63, 226, 272);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_limited_exp_ad(64, A::neg(s.ad_value(63)));
        }

        s.v[1770] = if (s.v[62] < 1e-10) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1770] != 0.0)) {
            s.store_limited_exp_ad(167, A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1770] != 0.0)) {
            let assign35540_ad_e57721: A = {
                if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))), 1.0))
                } else {
                    let assign35540_ad_e57720: A = {
                        if ((!((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271)))
                        } else {
                            {
                                if ((((s.v[123] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign35540_ad_e57720
                }
            };
            let assign35540_ad_e57803: A = {
                if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && (!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))), 1.0))
                } else {
                    let assign35540_ad_e57802: A = {
                        if ((!((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0)) && ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(271)), s.ad_value(6)), s.ad_value(271)))
                        } else {
                            {
                                if ((((s.v[45] * s.v[271]) - s.v[6]) / s.v[271]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign35540_ad_e57802
                }
            };
            s.store_sub_ad(169, assign35540_ad_e57721, assign35540_ad_e57803);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1770] != 0.0)) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(271)), A::div(s.ad_value(169), A::offset(s.ad_value(3), 1.0))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1770] != 0.0)) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1770] != 0.0)) {
            s.store_limited_exp_ad(172, A::add(s.ad_value(170), A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), s.ad_value(271))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1770] != 0.0)) {
            s.store_limited_exp_ad(173, A::neg(s.ad_value(123)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1770] != 0.0)) {
            s.store_square(174, 123);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1770] != 0.0)) {
            s.store_div_from_scalar_ad(175, 1.0, A::offset(s.ad_value(174), 2.0));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1770] != 0.0)) {
            s.store_limited_exp_ad(177, A::sub(s.ad_value(123), s.ad_value(98)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1770] != 0.0)) {
            s.store_limited_exp_ad(178, A::div(A::scale(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), 2.0), s.ad_value(271)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1770] != 0.0)) {
            s.store_limited_exp_ad(179, A::add(A::div(A::scale(A::sub(A::mul(s.ad_value(123), s.ad_value(271)), s.ad_value(6)), 2.0), s.ad_value(271)), s.ad_value(170)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1770] != 0.0)) {
            let assign35640_ad_e58024: A = A::add(A::limited_exp(A::sub(A::sub(s.ad_value(123), s.ad_value(63)), s.ad_value(98))), A::mul(A::limited_exp(A::sub(A::neg(s.ad_value(63)), s.ad_value(98))), A::offset(A::add(A::mul(A::scale(s.ad_value(123), (-2.0)), s.ad_value(175)), A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(123), 2.0), s.ad_value(123)), s.ad_value(123)), s.ad_value(175)), s.ad_value(175))), (-1.0))));
            let assign35640_ad_e58050: A = A::sub(A::add(A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170))), s.ad_value(2)), s.ad_value(2)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::scale(s.ad_value(214), 2.0)), A::scale(s.ad_value(123), 2.0)), A::mul(s.ad_value(296), A::offset(A::add(A::sub(A::sub(assign35640_ad_e58024, s.ad_value(173)), A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)))), A::div(s.ad_value(172), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)))), 1.0)));
            s.store_neg_ad(65, assign35640_ad_e58050);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1770] != 0.0)) {
            s.store_mul_ad_lhs(66, A::mul(s.ad_value(296), A::sub_from_scalar(1.0, s.ad_value(64))), 57);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1770] != 0.0)) {
            let assign35660_ad_e58121: A = A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::div(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(178)), A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(3), 1.0)), A::add(A::offset(A::scale(s.ad_value(167), 2.0), 1.0), s.ad_value(178)))));
            let assign35660_ad_e58166: A = A::mul(A::limited_exp(A::sub(A::neg(s.ad_value(98)), s.ad_value(63))), A::sub(A::add(A::scale(s.ad_value(175), (-2.0)), A::mul(A::mul(A::mul(A::scale(s.ad_value(123), 10.0), s.ad_value(123)), s.ad_value(175)), s.ad_value(175))), A::mul(A::mul(A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(123), 8.0), s.ad_value(123)), s.ad_value(123)), s.ad_value(123)), s.ad_value(175)), s.ad_value(175)), s.ad_value(175))));
            let assign35660_ad_e58191: A = A::add(A::sub(A::add(A::add(s.ad_value(173), A::limited_exp(A::sub(A::sub(s.ad_value(123), s.ad_value(98)), s.ad_value(63)))), assign35660_ad_e58166), A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)))), A::div(s.ad_value(178), A::mul(A::offset(s.ad_value(3), 1.0), A::add(A::offset(A::scale(s.ad_value(167), 2.0), 1.0), s.ad_value(178)))));
            let assign35660_ad_e58233: A = A::sub(A::sub(A::add(assign35660_ad_e58191, A::div(s.ad_value(172), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)))), A::div(s.ad_value(179), A::mul(A::offset(s.ad_value(3), 1.0), A::add(A::offset(A::scale(s.ad_value(167), 2.0), 1.0), s.ad_value(178))))), A::div(s.ad_value(179), A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(3), 1.0)), A::add(A::offset(A::scale(s.ad_value(167), 2.0), 1.0), s.ad_value(178)))));
            s.store_offset_ad(54, A::sub(A::sub(assign35660_ad_e58121, A::mul(s.ad_value(296), assign35660_ad_e58233)), A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(178)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::offset(s.ad_value(3), 1.0), A::add(A::offset(A::scale(s.ad_value(167), 2.0), 1.0), s.ad_value(178))))), 2.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1770] != 0.0)) {
            s.store_sub_ad(54, A::square(s.ad_value(65)), A::scale(A::mul(s.ad_value(54), s.ad_value(66)), 2.0));
        }

        s.v[1771] = if (s.v[54] >= 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1770] != 0.0)) && (s.v[1771] != 0.0)) {
            s.store_scale_ad(62, A::div(s.ad_value(66), A::add(s.ad_value(65), A::sqrt(s.ad_value(54)))), 2.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1770] != 0.0)) {
            s.store_add(23, 22, 62);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_mul(250, 62, 271);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_div_ad(67, A::square(s.ad_value(23)), A::offset(A::square(s.ad_value(23)), 2.0));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_limited_exp_ad(68, A::neg(s.ad_value(23)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_sub_ad(69, A::limited_exp(A::sub(s.ad_value(23), s.ad_value(98))), A::mul(A::limited_exp(A::neg(s.ad_value(98))), A::add(A::offset(s.ad_value(23), 1.0), s.ad_value(67))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_sub_ad_lhs(70, A::mul(A::mul(A::sub(s.ad_value(214), s.ad_value(23)), A::sub(s.ad_value(214), s.ad_value(23))), A::div_from_scalar(1.0, s.ad_value(296))), 69);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_offset_ad(70, A::scale(A::add(A::offset(s.ad_value(70), (-0.001)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(70), (-0.001)), A::offset(s.ad_value(70), (-0.001))), ((4.0 * 1e-5) * 1e-5)))), 0.5), 0.001);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_sqrt(60, 70);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_mul_ad_rhs(72, 294, A::sqrt(A::add(s.ad_value(70), s.ad_value(69))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_div_ad(73, A::mul(A::mul(s.ad_value(296), s.ad_value(69)), s.ad_value(271)), A::add(s.ad_value(72), A::mul(s.ad_value(294), s.ad_value(60))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_scaled_add(75, 22, 23, 0.5);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_abs_ad(54, A::mul(s.ad_value(68), s.ad_value(56)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_sqrt(76, 54);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_scaled_add(77, 57, 69, 0.5);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_add_ad_rhs(78, 77, A::scale(A::mul(A::square(s.ad_value(62)), A::sub(s.ad_value(76), A::scale(s.ad_value(297), 2.0))), 0.125));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_sub_ad_lhs(79, A::mul(A::mul(A::sub(s.ad_value(214), s.ad_value(75)), A::sub(s.ad_value(214), s.ad_value(75))), A::div_from_scalar(1.0, s.ad_value(296))), 78);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_mul_ad_rhs(51, 294, A::sqrt(A::add(s.ad_value(78), s.ad_value(79))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_offset_ad(79, A::scale(A::add(A::offset(s.ad_value(79), (-0.001)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(79), (-0.001)), A::offset(s.ad_value(79), (-0.001))), ((4.0 * 1e-5) * 1e-5)))), 0.5), 0.001);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_sqrt(71, 79);
        }

        s.v[1772] = if (((s.v[250]) as f64).abs() > 1e-35) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) && (s.v[1772] != 0.0)) {
            s.store_div_ad_lhs(74, A::sub(s.ad_value(306), s.ad_value(73)), 250);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_mul_ad_rhs(80, 271, A::div(A::mul(s.ad_value(296), s.ad_value(78)), A::add(s.ad_value(51), A::mul(s.ad_value(294), s.ad_value(71)))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_mul(52, 51, 271);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.copy_ad(83, 74);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_offset_ad(83, A::scale(A::add(A::offset(s.ad_value(83), (-0.001)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(83), (-0.001)), A::offset(s.ad_value(83), (-0.001))), ((4.0 * 1e-5) * 1e-5)))), 0.5), 0.001);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_add_ad_rhs(81, 80, A::mul(s.ad_value(271), s.ad_value(83)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1755] != 0.0))) {
            s.store_div(84, 81, 83);
        }

        s.v[1773] = if (s.v[22] <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1773] != 0.0)) {
            s.copy_ad(447, 52);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1773] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1773] != 0.0)) {
            s.copy_ad(445, 447);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1773] != 0.0)) {
            s.store_scalar(446, 0.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1773] != 0.0))) {
            s.store_scaled_div(26, 250, 84, 0.5);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1773] != 0.0))) {
            s.store_square(27, 26);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1773] != 0.0))) {
            s.store_add_ad_rhs(447, 52, A::scale(A::mul(s.ad_value(250), A::scale(s.ad_value(26), 0.3333333333333333)), 0.5));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1773] != 0.0))) {
            s.store_scaled_mul(54, 74, 250, 0.16666666666666666);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1773] != 0.0))) {
            s.store_add_ad_rhs(443, 80, A::mul(s.ad_value(54), s.ad_value(26)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1773] != 0.0))) {
            s.store_scale_ad(444, A::sub(s.ad_value(80), A::mul(s.ad_value(54), A::sub(A::sub_from_scalar(1.0, s.ad_value(26)), A::scale(s.ad_value(27), 0.2)))), 0.5);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1773] != 0.0))) {
            s.store_sub(445, 447, 443);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1773] != 0.0))) {
            s.store_sub_ad_lhs(446, A::sub(s.ad_value(447), s.ad_value(445)), 444);
        }

        if ((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) {
            s.store_scale_ad(437, A::div_from_scalar((8.8541878128e-12 * p.p110), s.ad_value(429)), p.p1380);
        }

        if ((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) {
            s.copy_ad(391, 437);
        }

        if ((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) {
            s.store_mul_ad_lhs(440, A::neg(s.ad_value(391)), 445);
        }

        s.v[1774] = if (s.v[211] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1774] != 0.0)) {
            s.store_mul_ad_lhs(441, A::neg(s.ad_value(391)), 446);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (s.v[1774] != 0.0)) {
            s.store_mul_ad_lhs(439, A::neg(s.ad_value(391)), 444);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1774] != 0.0))) {
            s.store_mul_ad_lhs(441, A::neg(s.ad_value(391)), 444);
        }

        if (((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) && (!(s.v[1774] != 0.0))) {
            s.store_mul_ad_lhs(439, A::neg(s.ad_value(391)), 446);
        }

        if ((s.v[1620] != 0.0) && (s.v[1741] != 0.0)) {
            s.store_neg_ad(442, A::add(A::add(s.ad_value(440), s.ad_value(441)), s.ad_value(439)));
        }

        if ((s.v[1620] != 0.0) && (!(s.v[1741] != 0.0))) {
            s.store_scalar(440, 0.0);
        }

        if ((s.v[1620] != 0.0) && (!(s.v[1741] != 0.0))) {
            s.store_scalar(439, 0.0);
        }

        if ((s.v[1620] != 0.0) && (!(s.v[1741] != 0.0))) {
            s.store_scalar(438, 0.0);
        }

        if ((s.v[1620] != 0.0) && (!(s.v[1741] != 0.0))) {
            s.store_scalar(441, 0.0);
        }

        if ((s.v[1620] != 0.0) && (!(s.v[1741] != 0.0))) {
            s.store_scalar(442, 0.0);
        }

        if (s.v[1620] != 0.0) {
            s.store_mul_ad_rhs(1075, 379, A::add(A::add(A::add(A::add(s.ad_value(387), s.ad_value(440)), s.ad_value(421)), s.ad_value(520)), s.ad_value(525)));
        }

        if (s.v[1620] != 0.0) {
            s.store_mul_ad_rhs(1050, 379, A::add(s.ad_value(388), s.ad_value(441)));
        }

        if (s.v[1620] != 0.0) {
            s.store_mul_ad_rhs(1053, 379, A::add(s.ad_value(389), s.ad_value(439)));
        }

        if (s.v[1620] != 0.0) {
            s.store_mul_ad_rhs(1076, 379, A::sub(A::add(A::add(s.ad_value(388), s.ad_value(441)), s.ad_value(423)), s.ad_value(520)));
        }

        if (s.v[1620] != 0.0) {
            s.store_mul_ad_rhs(1077, 379, A::sub(A::add(A::add(s.ad_value(389), s.ad_value(439)), s.ad_value(424)), s.ad_value(525)));
        }

        if (s.v[1620] != 0.0) {
            s.store_mul_ad_rhs(1078, 379, A::add(A::add(s.ad_value(390), s.ad_value(442)), s.ad_value(422)));
        }

        if (s.v[1620] != 0.0) {
            s.store_mul(1057, 379, 390);
        }

        if (s.v[1620] != 0.0) {
            s.store_mul(1058, 379, 442);
        }

        if (s.v[1620] != 0.0) {
            s.store_mul(1051, 379, 388);
        }

        if (s.v[1620] != 0.0) {
            s.store_mul(1052, 379, 441);
        }

        if (s.v[1620] != 0.0) {
            s.store_mul(1054, 379, 389);
        }

        if (s.v[1620] != 0.0) {
            s.store_mul(1055, 379, 439);
        }

        if (s.v[1620] != 0.0) {
            s.store_add_ad_rhs(810, 810, A::mul(s.ad_value(813), A::offset(s.ad_value(639), (-1.0))));
        }

        if (s.v[1620] != 0.0) {
            s.store_add_ad_rhs(816, 816, A::mul(s.ad_value(814), A::offset(s.ad_value(639), (-1.0))));
        }

        if (s.v[1620] != 0.0) {
            s.store_add_ad_rhs(819, 819, A::mul(s.ad_value(815), A::offset(s.ad_value(639), (-1.0))));
        }

        if (s.v[1620] != 0.0) {
            s.store_add_ad_rhs(884, 884, A::mul(s.ad_value(886), A::offset(s.ad_value(639), (-1.0))));
        }

        if (s.v[1620] != 0.0) {
            s.store_add_ad_rhs(882, 882, A::mul(s.ad_value(887), A::offset(s.ad_value(639), (-1.0))));
        }

        if (s.v[1620] != 0.0) {
            s.store_add_ad_rhs(888, 888, A::mul(s.ad_value(891), A::offset(s.ad_value(639), (-1.0))));
        }

        s.v[1775] = if ((p.p37 != 0.0) || (p.p38 != 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) {
            s.store_mul_ad_rhs(469, 269, A::sub(s.ad_value(213), A::scale(A::add(s.ad_value(22), s.ad_value(23)), 0.5)));
        }

        if ((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) {
            s.store_sqrt_ad(168, A::offset(A::square(s.ad_value(469)), 0.0001));
        }

    }

    pub(super) fn stamp_reactive_block_24(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) {
            s.store_scaled_sub(471, 168, 469, 0.5);
        }

        if ((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) {
            s.store_scaled_add(470, 469, 168, 0.5);
        }

        s.v[1776] = if (p.p38 != 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1776] != 0.0)) {
            s.store_scale(168, 469, 1.0 / (p.p671));
        }

        s.v[1777] = if (p.p696 != 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1776] != 0.0)) && (s.v[1777] != 0.0)) {
            s.store_sub_from_scalar_ad(167, 1.0, A::scale(s.ad_value(471), 1.0 / (p.p696)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1776] != 0.0)) && (!(s.v[1777] != 0.0))) {
            s.store_scalar(167, 1.0);
        }

        s.v[1778] = if (s.v[167] < 0.01) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1776] != 0.0)) && (s.v[1778] != 0.0)) {
            s.store_scalar(167, 0.01);
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1776] != 0.0)) {
            s.store_scale(168, 492, ((((s.v[184] * s.v[183]) / p.p1373) + (p.p1381 / p.p2)) * p.p700));
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1776] != 0.0)) {
            s.store_scalar(169, (p.p701 * p.p76));
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1776] != 0.0)) {
            s.store_div_ad_lhs(170, A::mul(s.ad_value(169), A::sub(s.ad_value(882), A::mul(s.ad_value(883), s.ad_value(471)))), 167);
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1776] != 0.0)) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1776] != 0.0)) {
            s.store_scaled_sub(168, 469, 809, 1.0 / (p.p671));
        }

        s.v[1779] = if (p.p697 != 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1776] != 0.0)) && (s.v[1779] != 0.0)) {
            s.store_sub_from_scalar_ad(167, 1.0, A::scale(s.ad_value(470), 1.0 / (p.p697)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1776] != 0.0)) && (!(s.v[1779] != 0.0))) {
            s.store_scalar(167, 1.0);
        }

        s.v[1780] = if (s.v[167] < 0.01) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1776] != 0.0)) && (s.v[1780] != 0.0)) {
            s.store_scalar(167, 0.01);
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1776] != 0.0)) {
            s.store_scale(168, 492, ((((s.v[184] * s.v[183]) / p.p1373) + (p.p1381 / p.p2)) * p.p698));
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1776] != 0.0)) {
            s.store_scalar(169, (p.p699 * p.p76));
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1776] != 0.0)) {
            s.store_div_ad_lhs(170, A::mul(s.ad_value(169), A::sub(s.ad_value(884), A::mul(s.ad_value(885), s.ad_value(470)))), 167);
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1776] != 0.0)) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1776] != 0.0)) {
            s.store_offset_ad(478, A::mul(s.ad_value(212), s.ad_value(269)), p.p1383);
        }

        s.v[1781] = if (((((p.p43 != 0.0) && (1.0 != 0.0)) && (!((p.p40 != 0.0) && (!(1.0 != 0.0))))) && (p.p45 == 1.0)) && (p.p1380 > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1781] != 0.0)) {
            s.store_mul_ad_rhs(208, 379, A::voltage(ctx, &nodes, Some(8), Some(11)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1781] != 0.0)) {
            s.store_sub(167, 208, 478);
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1781] != 0.0)) {
            s.store_sqrt_ad(168, A::offset(A::square(s.ad_value(167)), 0.0001));
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1781] != 0.0)) {
            s.store_scale_ad(209, A::offset(A::sub(s.ad_value(168), s.ad_value(167)), (-0.01)), 0.5);
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1781] != 0.0)) {
            s.store_scalar(178, (if (p.p30 == 1.0) { p.p702 } else { p.p703 }));
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1781] != 0.0)) {
            s.store_scalar(179, (if (p.p30 == 1.0) { p.p704 } else { p.p705 }));
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1781] != 0.0)) {
            s.store_mul(169, 208, 209);
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1781] != 0.0)) {
            s.store_sub_ad_lhs(170, A::mul(s.ad_value(888), s.ad_value(890)), 889);
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1781] != 0.0)) {
            s.store_mul(171, 889, 890);
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1781] != 0.0)) {
            s.store_mul_ad(172, A::scale(A::neg(s.ad_value(179)), p.p76), A::sub(A::add(s.ad_value(888), A::mul(s.ad_value(170), s.ad_value(209))), A::mul(A::mul(s.ad_value(171), s.ad_value(209)), s.ad_value(209))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1781] != 0.0)) {
            s.store_ad(173, &A::limited_exp(s.ad_value(172)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1781] != 0.0)) {
            s.store_mul_ad_lhs(178, A::scale(s.ad_value(178), p.p1380), 492);
        }

        s.v[1782] = if (p.p37 != 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) {
            s.store_sub_ad_rhs(168, 810, A::mul(s.ad_value(811), s.ad_value(470)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) {
            s.store_offset_ad(169, A::mul(s.ad_value(812), s.ad_value(470)), 1.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) {
            s.store_mul_ad_lhs(170, A::scale(s.ad_value(168), s.v[488]), 169);
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) {
            s.store_mul_ad(171, A::mul(A::mul(s.ad_value(253), s.ad_value(269)), s.ad_value(243)), A::limited_exp(s.ad_value(170)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) {
            s.store_offset_ad(472, A::sqrt(A::offset(A::square(s.ad_value(315)), 0.01)), (-0.1));
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) {
            s.store_scale(168, 472, s.v[823]);
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) {
            s.store_limited_exp_ad(482, A::neg(s.ad_value(168)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) {
            s.store_offset_ad(170, A::offset(A::add(s.ad_value(168), s.ad_value(482)), (-1.0)), 0.0001);
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) {
            s.store_offset_ad(171, A::sub_from_scalar(1.0, A::mul(A::offset(s.ad_value(168), 1.0), s.ad_value(482))), 0.0001);
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) {
            s.store_offset_ad(172, A::square(s.ad_value(168)), 0.0002);
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) {
            s.store_sub(169, 203, 219);
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) {
            s.store_sqrt_ad(228, A::offset(A::square(s.ad_value(169)), 0.0001));
        }

        s.v[1784] = if (p.p1295 == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) && (s.v[1784] != 0.0)) {
            s.store_scale_ad(168, A::add(A::sub(s.ad_value(816), A::mul(s.ad_value(817), s.ad_value(228))), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(816), A::mul(s.ad_value(817), s.ad_value(228))), A::sub(s.ad_value(816), A::mul(s.ad_value(817), s.ad_value(228)))), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[1785] = if (s.v[818] < 0.01) { 1.0 } else { 0.0 };

        if (((((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) && (s.v[1784] != 0.0)) && (s.v[1785] != 0.0)) {
            s.store_scalar(818, 0.01);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) && (!(s.v[1784] != 0.0))) {
            s.store_sub_ad_rhs(168, 816, A::mul(s.ad_value(817), s.ad_value(228)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) {
            s.store_offset_ad(169, A::mul(s.ad_value(818), s.ad_value(228)), 1.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) {
            s.store_mul_ad_lhs(170, A::mul(s.ad_value(491), s.ad_value(168)), 169);
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) {
            s.store_sub(169, 204, 219);
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) {
            s.store_sqrt_ad(229, A::offset(A::square(s.ad_value(169)), 0.0001));
        }

        s.v[1786] = if (p.p1295 == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) && (s.v[1786] != 0.0)) {
            s.store_scale_ad(168, A::add(A::sub(s.ad_value(819), A::mul(s.ad_value(820), s.ad_value(229))), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(819), A::mul(s.ad_value(820), s.ad_value(229))), A::sub(s.ad_value(819), A::mul(s.ad_value(820), s.ad_value(229)))), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[1787] = if (s.v[821] < 0.01) { 1.0 } else { 0.0 };

        if (((((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) && (s.v[1786] != 0.0)) && (s.v[1787] != 0.0)) {
            s.store_scalar(821, 0.01);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) && (!(s.v[1786] != 0.0))) {
            s.store_sub_ad_rhs(168, 819, A::mul(s.ad_value(820), s.ad_value(229)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) {
            s.store_offset_ad(169, A::mul(s.ad_value(821), s.ad_value(229)), 1.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) {
            s.store_mul_ad_lhs(170, A::mul(s.ad_value(491), s.ad_value(168)), 169);
        }

        if (((s.v[1620] != 0.0) && (s.v[1775] != 0.0)) && (s.v[1782] != 0.0)) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if (s.v[1620] != 0.0) {
            s.store_div_ad_lhs(607, A::scale(s.ad_value(746), 2.0), 337);
        }

        s.v[1788] = if (p.p1011 <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1620] != 0.0) && (s.v[1788] != 0.0)) {
            s.store_scalar(610, 0.0);
        }

        if ((s.v[1620] != 0.0) && (!(s.v[1788] != 0.0))) {
            s.store_div_ad_lhs(167, A::offset(A::div(s.ad_value(355), s.ad_value(300)), p.p1011), 607);
        }

        if ((s.v[1620] != 0.0) && (!(s.v[1788] != 0.0))) {
            s.store_mul_ad_rhs(610, 300, A::ln(A::max_with_scalar(s.ad_value(167), 1e-38)));
        }

        s.v[1789] = if (s.v[610] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1620] != 0.0) && (!(s.v[1788] != 0.0))) && (s.v[1789] != 0.0)) {
            s.store_scalar(610, 0.0);
        }

        if (s.v[1620] != 0.0) {
            s.store_mul_ad(613, A::scale(s.ad_value(271), 6.241509744511525e18), A::add(A::offset(s.ad_value(260), s.v[199]), s.ad_value(709)));
        }

        if (s.v[1620] != 0.0) {
            s.store_scale_ad(612, A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(253), (2.0 * s.v[199])), s.ad_value(271)), s.ad_value(73)), s.ad_value(853)), s.ad_value(834)), 6.241509744511525e18);
        }

        if (s.v[1620] != 0.0) {
            s.store_mul_ad_lhs(1004, A::mul(A::scale(s.ad_value(271), ((1.602176462e-19 * 1.602176462e-19) * 1.602176462e-19)), A::abs(s.ad_value(380))), 337);
        }

        if (s.v[1620] != 0.0) {
            s.store_mul_ad_lhs(1005, A::mul(A::scale(s.ad_value(271), 1.602176462e-19), s.ad_value(380)), 380);
        }

        if (s.v[1620] != 0.0) {
            s.store_add_ad(1006, A::offset(A::scale(s.ad_value(612), p.p1013), p.p1012), A::mul(A::scale(s.ad_value(612), p.p1014), s.ad_value(612)));
        }

        if (s.v[1620] != 0.0) {
            s.store_mul_ad(1007, A::add(s.ad_value(612), s.ad_value(613)), A::add(s.ad_value(612), s.ad_value(613)));
        }

        if (s.v[1620] != 0.0) {
            s.store_scale(1008, 271, (p.p1012 * 1.602176462e-19));
        }

        s.v[1790] = if (p.p1015 >= (s.v[184] / 2.0)) { 1.0 } else { 0.0 };

        if ((s.v[1620] != 0.0) && (s.v[1790] != 0.0)) {
            s.store_scalar(606, 0.0);
        }

        if ((s.v[1620] != 0.0) && (!(s.v[1790] != 0.0))) {
            s.store_scalar(606, p.p1015);
        }

        s.v[1791] = if (((p.p1012 > 0.0) || (p.p1013 > 0.0)) || (p.p1014 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1620] != 0.0) && (s.v[1791] != 0.0)) {
            s.store_sub_from_scalar_ad(608, s.v[184], A::scale(s.ad_value(606), 2.0));
        }

        if ((s.v[1620] != 0.0) && (s.v[1791] != 0.0)) {
            s.store_square(609, 608);
        }

        if ((s.v[1620] != 0.0) && (s.v[1791] != 0.0)) {
            s.store_scale(167, 609, (10000000000.0 * s.v[199]));
        }

        if ((s.v[1620] != 0.0) && (s.v[1791] != 0.0)) {
            s.store_scale_ad(611, A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(253), (2.0 * s.v[199])), s.ad_value(271)), s.ad_value(306)), s.ad_value(853)), s.ad_value(834)), 6.241509744511525e18);
        }

        if ((s.v[1620] != 0.0) && (s.v[1791] != 0.0)) {
            s.store_scale_ad(168, A::ln(A::max_with_scalar(A::div(A::add(s.ad_value(611), s.ad_value(613)), A::add(s.ad_value(612), s.ad_value(613))), 1e-38)), p.p1012);
        }

        if ((s.v[1620] != 0.0) && (s.v[1791] != 0.0)) {
            s.store_scaled_sub(169, 611, 612, p.p1013);
        }

        if ((s.v[1620] != 0.0) && (s.v[1791] != 0.0)) {
            s.store_scale_ad(170, A::sub(A::square(s.ad_value(611)), A::square(s.ad_value(612))), (0.5 * p.p1014));
        }

        if ((s.v[1620] != 0.0) && (s.v[1791] != 0.0)) {
            s.store_scale(171, 609, (10000000000.0 * (s.v[183] * p.p2)));
        }

        if ((s.v[1620] != 0.0) && (s.v[1791] != 0.0)) {
            s.store_add_ad(614, A::mul(A::div(s.ad_value(1004), s.ad_value(167)), A::add(A::add(s.ad_value(168), s.ad_value(169)), s.ad_value(170))), A::div(A::mul(A::mul(A::div(s.ad_value(1005), s.ad_value(171)), s.ad_value(610)), s.ad_value(1006)), s.ad_value(1007)));
        }

        if ((s.v[1620] != 0.0) && (s.v[1791] != 0.0)) {
            s.store_mul_ad_lhs(172, A::mul(A::scale(s.ad_value(608), ((s.v[183] * p.p2) * 10000000000.0)), s.ad_value(613)), 613);
        }

        if ((s.v[1620] != 0.0) && (s.v[1791] != 0.0)) {
            s.store_mul_ad_lhs(615, A::mul(A::div(s.ad_value(1008), s.ad_value(172)), s.ad_value(380)), 380);
        }

        if ((s.v[1620] != 0.0) && (s.v[1791] != 0.0)) {
            s.store_add(173, 615, 614);
        }

        if (s.v[1620] != 0.0) {
            s.store_scaled_div(167, 243, 607, 1.0 / (s.v[184]));
        }

        if (s.v[1620] != 0.0) {
            s.store_square(168, 167);
        }

        if (s.v[1620] != 0.0) {
            s.store_scale_ad(170, A::offset(A::scale(s.ad_value(168), (p.p1022 * s.v[184])), 1.0), p.p1019);
        }

        if (s.v[1620] != 0.0) {
            s.store_scale_ad(171, A::offset(A::scale(s.ad_value(168), (p.p1023 * s.v[184])), 1.0), p.p1020);
        }

        if (s.v[1620] != 0.0) {
            s.store_scale_ad(172, A::offset(A::scale(s.ad_value(168), (p.p1298 * s.v[184])), 1.0), p.p1297);
        }

        if (s.v[1620] != 0.0) {
            s.store_square(633, 172);
        }

        if (s.v[1620] != 0.0) {
            s.store_square(632, 171);
        }

        if (s.v[1620] != 0.0) {
            s.copy_ad(345, 343);
        }

        s.v[1793] = if (p.p39 == 0.0) { 1.0 } else { 0.0 };

        s.v[1794] = if (p.p39 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1620] != 0.0) && (s.v[1793] != 0.0)) {
            s.store_mul_ad_lhs(388, A::scale(s.ad_value(271), ((((-p.p2) * s.v[183]) * s.v[184]) * s.v[199])), 1050);
        }

        if ((s.v[1620] != 0.0) && (s.v[1793] != 0.0)) {
            s.store_mul_ad_lhs(389, A::scale(s.ad_value(271), ((((-p.p2) * s.v[183]) * s.v[184]) * s.v[199])), 1053);
        }

        if ((s.v[1620] != 0.0) && (s.v[1793] != 0.0)) {
            s.store_mul_ad_rhs(167, 337, A::abs(A::add(s.ad_value(388), s.ad_value(389))));
        }

        if ((s.v[1620] != 0.0) && (s.v[1793] != 0.0)) {
            s.store_offset_ad(168, A::mul(s.ad_value(167), s.ad_value(457)), (s.v[184] * s.v[184]));
        }

        if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
            s.store_mul_ad_lhs(626, A::scale(s.ad_value(253), 2.0), 269);
        }

        if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
            s.store_mul_ad_lhs(167, A::scale(A::mul(A::mul(s.ad_value(337), s.ad_value(345)), s.ad_value(363)), s.v[199]), 626);
        }

        if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
            s.store_scaled_add(168, 306, 73, 0.5);
        }

        if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
            s.store_offset(170, 168, 0.5);
        }

        if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
            s.store_square(171, 170);
        }

        if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
            s.store_mul(172, 171, 170);
        }

        if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
            s.store_sub(173, 306, 73);
        }

        if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
            s.store_square(174, 173);
        }

        if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
            s.store_mul(175, 174, 173);
        }

        if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
            s.store_mul_ad_lhs(176, A::offset(A::scale(s.ad_value(168), 6.0), 0.5), 174);
        }

        if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
            s.store_scale(625, 345, s.v[184]);
        }

        if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
            s.store_scale(177, 625, 1.0 / (s.v[184]));
        }

        if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
            s.store_offset_ad(179, A::div(A::mul(s.ad_value(633), A::div(s.ad_value(315), s.ad_value(312))), A::offset(s.ad_value(243), p.p1299)), 1.0);
        }

        if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
            s.store_offset_ad(179, A::scale(A::offset(s.ad_value(179), (-1.0)), if ((-s.v[184]) / p.p1296) > 80.0 { 5.540622384e34 * (1.0 + (((-s.v[184]) / p.p1296)) - 80.0) } else if ((-s.v[184]) / p.p1296) < -80.0 { 1.804851387e-35 } else { ((((-s.v[184]) / p.p1296)) as f64).exp() }), 1.0);
        }

        if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
            s.store_scale_ad(179, A::add(s.ad_value(179), A::sqrt(A::offset(A::mul(s.ad_value(179), s.ad_value(179)), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
            let assign38150_ad_e61223: A = A::mul(A::scale(A::mul(A::mul(A::mul(s.ad_value(625), s.ad_value(177)), s.ad_value(177)), A::add(A::sub(A::div(s.ad_value(168), s.ad_value(171)), A::div(s.ad_value(176), A::mul(A::scale(s.ad_value(171), 60.0), s.ad_value(171)))), A::div(A::square(s.ad_value(174)), A::mul(A::scale(s.ad_value(171), 144.0), s.ad_value(172))))), (15.0 * 0.25)), s.ad_value(632));
            s.store_div_ad(622, assign38150_ad_e61223, A::scale(s.ad_value(167), ((p.p2 * s.v[183]) * 12.0)));
        }

        s.v[1799] = if (p.p27 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_ln_ad(951, A::max_with_scalar(A::div(s.ad_value(953), s.ad_value(182)), 1e-38));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_max_with_scalar_ad(298, A::add(A::offset(A::mul(s.ad_value(271), s.ad_value(951)), 0.4), s.ad_value(729)), 0.4);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_sqrt(299, 298);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_sqrt_ad(277, A::div_from_scalar((2.0 * s.v[180]), A::scale(s.ad_value(953), 1.602176462e-19)));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_mul_ad_rhs(941, 835, A::scale(A::add(A::offset(A::mul(s.ad_value(847), A::offset(s.ad_value(639), (-1.0))), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(847), A::offset(s.ad_value(639), (-1.0))), 1.0), A::offset(A::mul(s.ad_value(847), A::offset(s.ad_value(639), (-1.0))), 1.0)), ((4.0 * 0.001) * 0.001)))), 0.5));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_mul_ad_rhs(940, 841, A::offset(A::mul(s.ad_value(848), A::offset(s.ad_value(639), (-1.0))), 1.0));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_scale_ad(273, A::add(A::offset(A::sub(s.ad_value(298), s.ad_value(218)), 0.05), A::sqrt(A::offset(A::mul(A::offset(A::sub(s.ad_value(298), s.ad_value(218)), (-0.05)), A::offset(A::sub(s.ad_value(298), s.ad_value(218)), (-0.05))), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

    }

    pub(super) fn stamp_reactive_block_25(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_sqrt(274, 273);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_mul(275, 277, 274);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_div_from_scalar(260, s.v[180], 275);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_div_ad(169, A::mul(s.ad_value(5), s.ad_value(7)), A::add(s.ad_value(5), s.ad_value(7)));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_mul_ad_lhs(170, A::sub(A::add(s.ad_value(838), A::scale(s.ad_value(220), p.p1183)), A::scale(s.ad_value(218), p.p1195)), 227);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            let assign38400_ad_e61582: A = A::add(A::sub(A::sub(A::add(A::scale(s.ad_value(220), p.p1181), A::mul(A::scale(s.ad_value(220), p.p1182), s.ad_value(220))), A::scale(s.ad_value(218), p.p1184)), A::mul(A::scale(s.ad_value(218), p.p1185), s.ad_value(218))), A::mul(s.ad_value(955), A::add(A::add(A::add(A::add(A::add(s.ad_value(715), A::mul(s.ad_value(712), s.ad_value(220))), A::mul(A::scale(s.ad_value(220), p.p1180), s.ad_value(220))), A::mul(s.ad_value(716), s.ad_value(218))), A::mul(A::scale(s.ad_value(218), p.p1190), s.ad_value(218))), s.ad_value(170))));
            s.store_ad(171, &assign38400_ad_e61582);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_div_ad(168, A::add(A::add(A::add(A::offset(s.ad_value(169), s.v[199]), s.ad_value(836)), s.ad_value(941)), s.ad_value(171)), A::offset(s.ad_value(169), s.v[199]));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_scale_ad(267, A::add(A::offset(s.ad_value(168), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(168), (-1.0)), A::offset(s.ad_value(168), (-1.0))), ((0.25 * 0.05) * 0.05)))), 0.5);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_mul(269, 267, 271);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_div_from_scalar(270, 1.0, 269);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_mul(222, 221, 270);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_mul(225, 224, 270);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_mul(212, 707, 270);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_mul(215, 708, 270);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_mul(238, 234, 270);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_sub_ad(291, A::mul(s.ad_value(736), A::sub(s.ad_value(274), s.ad_value(299))), A::mul(s.ad_value(849), s.ad_value(218)));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_mul_ad_lhs(944, A::neg(A::add(s.ad_value(940), A::mul(s.ad_value(842), s.ad_value(218)))), 227);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_mul_ad(293, A::add(A::add(s.ad_value(843), A::scale(s.ad_value(844), 1.0 / (s.v[184]))), A::mul(s.ad_value(845), s.ad_value(218))), A::offset(A::pow(s.ad_value(639), s.ad_value(846)), (-1.0)));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_mul_ad_rhs(946, 300, A::offset(A::scale(s.ad_value(218), p.p1264), 1.0));
        }

        s.v[1800] = if (s.v[946] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1800] != 0.0)) {
            s.store_div_from_scalar(167, (p.p1263 * s.v[184]), 946);
        }

        s.v[1801] = if (s.v[167] < 40.0) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1800] != 0.0)) && (s.v[1801] != 0.0)) {
            s.store_div_from_scalar_ad(943, (0.5 * p.p1262), A::offset(A::cosh(s.ad_value(167)), (-1.0)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1800] != 0.0)) && (!(s.v[1801] != 0.0))) {
            s.store_scale_ad(943, A::limited_exp(A::neg(s.ad_value(167))), p.p1262);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1800] != 0.0))) {
            s.store_scalar(943, 0.0);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_mul_ad_rhs(945, 943, A::sub(s.ad_value(942), s.ad_value(298)));
        }

        s.v[1802] = if (s.v[280] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1802] != 0.0)) {
            s.store_mul_ad_lhs(167, A::neg(s.ad_value(282)), 227);
        }

        s.v[1803] = if (s.v[167] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1802] != 0.0)) && (s.v[1803] != 0.0)) {
            s.store_scalar(169, 1.804851387e-35);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1802] != 0.0)) && (!(s.v[1803] != 0.0))) {
            s.store_ad(169, &A::limited_exp(s.ad_value(167)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1802] != 0.0)) {
            s.store_offset_ad(170, A::mul(s.ad_value(280), A::offset(s.ad_value(169), 1.0)), s.v[184]);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1802] != 0.0)) {
            s.store_mul_ad(278, A::neg(s.ad_value(269)), A::ln(A::max_with_scalar(A::div_from_scalar(s.v[184], s.ad_value(170)), 1e-38)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1802] != 0.0))) {
            s.store_scalar(278, 0.0);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_add_ad_rhs(171, 290, A::div(s.ad_value(284), A::pow_from_scalar(s.v[184], s.ad_value(286))));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_sub_ad_rhs(278, 278, A::mul(s.ad_value(171), A::tanh(A::mul(s.ad_value(288), s.ad_value(227)))));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_add_ad_lhs(242, A::add(A::offset(A::add(A::sub(A::add(A::add(s.ad_value(291), s.ad_value(278)), s.ad_value(944)), s.ad_value(293)), s.ad_value(945)), p.p1151), s.ad_value(956)), 932);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_sub_ad(213, A::sub(s.ad_value(222), s.ad_value(212)), A::mul(s.ad_value(242), s.ad_value(270)));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_sub_ad(367, A::sub(A::sub(s.ad_value(222), A::mul(s.ad_value(218), s.ad_value(270))), s.ad_value(212)), A::mul(s.ad_value(242), s.ad_value(270)));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_sub_ad(214, A::sub(s.ad_value(222), s.ad_value(212)), A::mul(s.ad_value(242), s.ad_value(270)));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_sub(216, 238, 215);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_scale_ad(294, A::sqrt(A::mul(A::scale(s.ad_value(953), ((2.0 * 1.602176462e-19) * s.v[180])), s.ad_value(270))), 1.0 / (s.v[199]));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_scalar(947, (p.p1148 * (1.0 + (p.p1149 * ((s.v[184]) as f64).powf((-p.p1150))))));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_mul_ad_rhs(294, 294, A::offset(s.ad_value(947), 1.0));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_div_from_scalar(295, 1.0, 294);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_square(296, 294);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_div_from_scalar(297, 1.0, 296);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_scalar(5, (s.v[180] / p.p74));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_scalar(7, (s.v[181] / p.p75));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_div_ad_lhs(3, A::add(s.ad_value(7), s.ad_value(728)), 5);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_scalar(2, (p.p76 / p.p75));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_div(124, 294, 2);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_offset_scaled(125, 124, 0.7071067811865475, 1.0);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_scale(126, 125, 1e-7);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_scalar(127, (5.0 / 4.0));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_div_from_scalar(128, 1.0, 124);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_square(129, 124);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_div_from_scalar_ad(130, 1.0, A::add(s.ad_value(127), A::scale(s.ad_value(124), 0.7324648775608221)));
        }

        s.v[1804] = if (((s.v[216]) as f64).abs() <= s.v[126]) { 1.0 } else { 0.0 };

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1804] != 0.0)) {
            s.store_mul_ad(131, A::mul(A::neg(s.ad_value(216)), s.ad_value(128)), A::offset(A::mul(s.ad_value(124), A::div(A::neg(s.ad_value(216)), A::mul(A::scale(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt())), s.ad_value(125)))), 1.0));
        }

        s.v[1805] = if (s.v[216] < (-s.v[126])) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (s.v[1805] != 0.0)) {
            s.store_neg(132, 216);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (s.v[1805] != 0.0)) {
            s.store_mul_ad_lhs(133, A::mul(s.ad_value(127), s.ad_value(132)), 128);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (s.v[1805] != 0.0)) {
            s.store_scale_ad(134, A::sub(A::offset(s.ad_value(133), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(133), (-6.0)), A::offset(s.ad_value(133), (-6.0))), 64.0))), 0.5);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (s.v[1805] != 0.0)) {
            s.store_add_ad(135, A::mul(A::sub(s.ad_value(132), s.ad_value(134)), A::sub(s.ad_value(132), s.ad_value(134))), A::mul(s.ad_value(129), A::offset(s.ad_value(134), 1.0)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (s.v[1805] != 0.0)) {
            s.store_sub_ad_lhs(137, A::scale(A::sub(s.ad_value(132), s.ad_value(134)), 2.0), 129);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (s.v[1805] != 0.0)) {
            s.store_sub_ad_lhs(138, A::ln(A::max_with_scalar(A::div(s.ad_value(135), s.ad_value(129)), 1e-38)), 134);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (s.v[1805] != 0.0)) {
            s.store_add(0, 135, 137);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (s.v[1805] != 0.0)) {
            s.store_add_ad(1, A::square(s.ad_value(0)), A::mul(s.ad_value(138), A::sub(A::scale(A::square(s.ad_value(137)), 0.5), s.ad_value(135))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (s.v[1805] != 0.0)) {
            s.store_add_ad_rhs(140, 134, A::div(A::mul(A::mul(s.ad_value(135), s.ad_value(0)), s.ad_value(138)), A::add(s.ad_value(1), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(138)), s.ad_value(138)), s.ad_value(137)), A::sub(A::scale(A::square(s.ad_value(137)), 0.3333333333333333), s.ad_value(135))))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (s.v[1805] != 0.0)) {
            s.store_ad(141, &A::limited_exp(s.ad_value(140)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (s.v[1805] != 0.0)) {
            s.store_sub(142, 132, 140);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (s.v[1805] != 0.0)) {
            s.store_add_ad(143, A::scale(s.ad_value(142), 2.0), A::mul(s.ad_value(129), A::offset(s.ad_value(141), (-1.0))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (s.v[1805] != 0.0)) {
            s.store_add_ad(136, A::square(s.ad_value(142)), A::mul(s.ad_value(129), A::sub(A::offset(s.ad_value(140), 1.0), s.ad_value(141))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (s.v[1805] != 0.0)) {
            s.store_sub_from_scalar_ad(144, 1.0, A::mul(A::scale(s.ad_value(129), 0.5), s.ad_value(141)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (s.v[1805] != 0.0)) {
            s.store_sub_ad(142, A::square(s.ad_value(143)), A::scale(A::mul(s.ad_value(144), s.ad_value(136)), 4.0));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (s.v[1805] != 0.0)) {
            s.store_scale_ad(145, A::div(s.ad_value(136), A::add(s.ad_value(143), A::sqrt(s.ad_value(142)))), 2.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (s.v[1805] != 0.0)) {
            s.store_neg_ad(131, A::add(s.ad_value(140), s.ad_value(145)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (!(s.v[1805] != 0.0))) {
            s.store_mul_ad_lhs(146, A::offset(A::mul(A::mul(s.ad_value(125), s.ad_value(127)), s.ad_value(130)), (-1.0)), 130);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (!(s.v[1805] != 0.0))) {
            s.store_mul_ad(147, A::mul(s.ad_value(216), s.ad_value(128)), A::offset(A::mul(s.ad_value(146), s.ad_value(216)), 1.0));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (!(s.v[1805] != 0.0))) {
            s.store_limited_exp_ad(150, A::neg(s.ad_value(147)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (!(s.v[1805] != 0.0))) {
            s.store_sub_from_scalar(149, 1.0, 150);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (!(s.v[1805] != 0.0))) {
            s.store_sub_ad(148, A::add(s.ad_value(216), A::scale(s.ad_value(129), 0.5)), A::mul(s.ad_value(124), A::sqrt(A::sub(A::add(s.ad_value(216), A::scale(s.ad_value(129), 0.25)), s.ad_value(149)))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (!(s.v[1805] != 0.0))) {
            s.store_limited_exp_ad(151, A::neg(s.ad_value(148)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (!(s.v[1805] != 0.0))) {
            s.store_add_ad(152, A::scale(A::sub(s.ad_value(216), s.ad_value(148)), 2.0), A::mul(s.ad_value(129), A::sub_from_scalar(1.0, s.ad_value(151))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (!(s.v[1805] != 0.0))) {
            s.store_sub_ad(153, A::mul(A::sub(s.ad_value(216), s.ad_value(148)), A::sub(s.ad_value(216), s.ad_value(148))), A::mul(s.ad_value(129), A::add(A::offset(s.ad_value(148), (-1.0)), s.ad_value(151))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (!(s.v[1805] != 0.0))) {
            s.store_sub_from_scalar_ad(154, 1.0, A::mul(A::scale(s.ad_value(129), 0.5), s.ad_value(151)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (!(s.v[1805] != 0.0))) {
            s.store_sub_ad(150, A::square(s.ad_value(152)), A::scale(A::mul(s.ad_value(154), s.ad_value(153)), 4.0));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (!(s.v[1805] != 0.0))) {
            s.store_scale_ad(139, A::div(s.ad_value(153), A::add(s.ad_value(152), A::sqrt(s.ad_value(150)))), 2.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1804] != 0.0))) && (!(s.v[1805] != 0.0))) {
            s.store_add(131, 148, 139);
        }

        s.v[1806] = if (((s.v[216]) as f64).abs() < s.v[126]) { 1.0 } else { 0.0 };

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1806] != 0.0)) {
            s.store_mul_ad(46, A::mul(A::neg(s.ad_value(216)), s.ad_value(128)), A::offset(A::mul(s.ad_value(124), A::div(A::neg(s.ad_value(216)), A::mul(A::scale(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt())), s.ad_value(125)))), 1.0));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1806] != 0.0)) {
            s.store_mul_ad(131, A::mul(A::neg(s.ad_value(216)), s.ad_value(128)), A::offset(A::mul(s.ad_value(124), A::div(A::neg(s.ad_value(216)), A::mul(A::scale(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt())), s.ad_value(125)))), 1.0));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1806] != 0.0))) {
            s.store_sub_ad(19, A::mul(A::mul(A::mul(A::mul(A::square(s.ad_value(2)), A::sub(s.ad_value(216), s.ad_value(131))), A::sub(s.ad_value(216), s.ad_value(131))), A::div_from_scalar(1.0, s.ad_value(294))), A::div_from_scalar(1.0, s.ad_value(294))), A::offset(A::add(A::limited_exp(A::neg(s.ad_value(131))), s.ad_value(131)), (-1.0)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1806] != 0.0))) {
            s.store_offset_ad(20, A::add(A::limited_exp(A::neg(s.ad_value(131))), A::div(A::mul(A::square(s.ad_value(2)), A::sub(A::scale(s.ad_value(131), 2.0), A::scale(s.ad_value(216), 2.0))), A::square(s.ad_value(294)))), (-1.0));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1806] != 0.0))) {
            s.store_sub_ad_rhs(46, 131, A::div(s.ad_value(19), s.ad_value(20)));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_mul(46, 46, 269);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_offset_scaled(95, 294, 0.7071067811865475, 1.0);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_div_from_scalar(96, 1.0, 95);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_add_ad_lhs(97, A::div(A::scale(s.ad_value(251), 2.0), s.ad_value(267)), 225);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_limited_exp_ad(99, A::neg(s.ad_value(97)));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_scale(101, 95, 0.001);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_div_ad_lhs(167, A::scale(A::neg(s.ad_value(726)), s.v[184]), 300);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_mul_ad_lhs(168, A::mul(s.ad_value(725), A::add(A::limited_exp(A::scale(s.ad_value(167), 0.5)), A::scale(A::limited_exp(s.ad_value(167)), 2.0))), 724);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_sub_ad(4, A::add(A::div(A::mul(A::offset(A::scale(s.ad_value(743), 1.0 / (s.v[184])), 1.0), A::scale(s.ad_value(706), (1.602176462e-19 * (p.p74 * p.p74)))), A::scale(s.ad_value(269), (2.0 * s.v[180]))), A::div_from_scalar(p.p294, s.ad_value(269))), A::mul(s.ad_value(3), s.ad_value(216)));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_add_ad_rhs(104, 4, A::mul(s.ad_value(294), A::sqrt(A::offset(A::add(A::limited_exp(A::neg(s.ad_value(4))), s.ad_value(4)), (-1.0)))));
        }

        s.v[1807] = if (s.v[4] < s.v[97]) { 1.0 } else { 0.0 };

        s.v[1808] = if (s.v[214] < s.v[104]) { 1.0 } else { 0.0 };

        s.v[1809] = if (((s.v[214]) as f64).abs() <= s.v[101]) { 1.0 } else { 0.0 };

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (s.v[1809] != 0.0)) {
            s.store_scale_ad(167, A::square(s.ad_value(96)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (s.v[1809] != 0.0)) {
            s.store_mul_ad(9, A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(99))), s.ad_value(294)), s.ad_value(167)), 1.0));
        }

        s.v[1810] = if (s.v[214] < (-s.v[101])) { 1.0 } else { 0.0 };

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (s.v[1810] != 0.0)) {
            s.store_neg(10, 214);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (s.v[1810] != 0.0)) {
            s.store_scaled_mul(11, 10, 96, 1.25);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (s.v[1810] != 0.0)) {
            s.store_scale_ad(12, A::sub(A::offset(s.ad_value(11), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(11), (-6.0)), A::offset(s.ad_value(11), (-6.0))), 64.0))), 0.5);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (s.v[1810] != 0.0)) {
            s.store_sub(13, 10, 12);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (s.v[1810] != 0.0)) {
            s.store_add_ad(14, A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::offset(s.ad_value(12), 1.0)));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (s.v[1810] != 0.0)) {
            s.store_sub_ad_lhs(16, A::scale(s.ad_value(13), 2.0), 296);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (s.v[1810] != 0.0)) {
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (s.v[1810] != 0.0)) {
            s.store_add(0, 14, 16);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (s.v[1810] != 0.0)) {
            s.store_add_ad(1, A::square(s.ad_value(0)), A::mul(s.ad_value(17), A::sub(A::scale(A::square(s.ad_value(16)), 0.5), s.ad_value(14))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (s.v[1810] != 0.0)) {
            s.store_add_ad_rhs(18, 12, A::div(A::mul(A::mul(s.ad_value(14), s.ad_value(0)), s.ad_value(17)), A::add(s.ad_value(1), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17)), s.ad_value(17)), s.ad_value(16)), A::sub(A::scale(A::square(s.ad_value(16)), 0.3333333333333333), s.ad_value(14))))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (s.v[1810] != 0.0)) {
            s.store_ad(28, &A::limited_exp(s.ad_value(18)));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (s.v[1810] != 0.0)) {
            s.store_div_from_scalar(29, 1.0, 28);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (s.v[1810] != 0.0)) {
            s.store_div_from_scalar_ad(13, 1.0, A::offset(A::square(s.ad_value(18)), 2.0));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (s.v[1810] != 0.0)) {
            s.store_mul_ad_lhs(30, A::square(s.ad_value(18)), 13);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (s.v[1810] != 0.0)) {
            s.store_scale_ad(31, A::mul(A::mul(s.ad_value(18), s.ad_value(13)), s.ad_value(13)), 4.0);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (s.v[1810] != 0.0)) {
            s.store_mul_ad_lhs(32, A::mul(A::sub(A::scale(s.ad_value(13), 8.0), A::scale(s.ad_value(30), 12.0)), s.ad_value(13)), 13);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (s.v[1810] != 0.0)) {
            s.store_sub(13, 10, 18);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (s.v[1810] != 0.0)) {
            s.store_mul(33, 99, 29);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (s.v[1810] != 0.0)) {
            s.store_add_ad(36, A::scale(s.ad_value(13), 2.0), A::mul(s.ad_value(296), A::add(A::sub(A::offset(s.ad_value(28), (-1.0)), s.ad_value(33)), A::mul(s.ad_value(99), A::sub_from_scalar(1.0, s.ad_value(31))))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (s.v[1810] != 0.0)) {
            s.store_sub_ad(37, A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::add(A::add(A::offset(A::sub(s.ad_value(28), s.ad_value(18)), (-1.0)), s.ad_value(33)), A::mul(s.ad_value(99), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30))))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (s.v[1810] != 0.0)) {
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::sub(A::add(s.ad_value(28), s.ad_value(33)), A::mul(s.ad_value(99), s.ad_value(32)))));
        }

    }

    pub(super) fn stamp_reactive_block_26(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (s.v[1810] != 0.0)) {
            s.store_sub_ad(13, A::square(s.ad_value(36)), A::scale(A::mul(s.ad_value(37), s.ad_value(13)), 2.0));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (s.v[1810] != 0.0)) {
            s.store_sub_ad(9, A::neg(s.ad_value(18)), A::scale(A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_div_from_scalar_ad(38, 1.0, A::offset(A::scale(s.ad_value(294), 0.7324648775608221), 1.25));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_mul_ad_lhs(39, A::offset(A::mul(A::scale(s.ad_value(95), 1.25), s.ad_value(38)), (-1.0)), 38);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_mul_ad(40, A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_limited_exp_ad(13, A::neg(s.ad_value(40)));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_sub_from_scalar(41, 1.0, 13);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_sub_ad(42, A::add(s.ad_value(214), A::scale(s.ad_value(296), 0.5)), A::mul(s.ad_value(294), A::sqrt(A::sub(A::add(s.ad_value(214), A::scale(s.ad_value(296), 0.25)), s.ad_value(41)))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_offset(43, 97, 3.0);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_sub_ad(12, A::scale(A::sub(A::add(s.ad_value(42), s.ad_value(43)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(42), s.ad_value(43)), A::sub(s.ad_value(42), s.ad_value(43))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(43), A::sqrt(A::offset(A::square(s.ad_value(43)), 5.0))), 0.5));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_sub(13, 214, 12);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_limited_exp_ad(33, A::neg(s.ad_value(12)));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_div_from_scalar_ad(34, 1.0, A::offset(A::square(s.ad_value(12)), 2.0));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_mul_ad_lhs(30, A::square(s.ad_value(12)), 34);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_scale_ad(31, A::mul(A::mul(s.ad_value(12), s.ad_value(34)), s.ad_value(34)), 4.0);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_mul_ad_lhs(32, A::mul(A::sub(A::scale(s.ad_value(34), 8.0), A::scale(s.ad_value(30), 12.0)), s.ad_value(34)), 34);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_max_from_scalar_ad(14, 1e-40, A::sub(A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::sub(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), A::mul(s.ad_value(99), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)))))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_sub_from_scalar_ad(15, 1.0, A::scale(A::mul(s.ad_value(296), A::sub(s.ad_value(33), A::mul(s.ad_value(99), s.ad_value(32)))), 0.5));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_add_ad(16, A::scale(s.ad_value(13), 2.0), A::mul(s.ad_value(296), A::sub(A::sub_from_scalar(1.0, s.ad_value(33)), A::mul(s.ad_value(99), A::offset(s.ad_value(31), 1.0)))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_add_ad(17, A::sub(s.ad_value(97), s.ad_value(12)), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_add(0, 14, 16);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_add_ad(1, A::square(s.ad_value(0)), A::mul(s.ad_value(17), A::sub(A::scale(A::square(s.ad_value(16)), 0.5), A::mul(s.ad_value(14), s.ad_value(15)))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_add_ad_rhs(44, 12, A::div(A::mul(A::mul(s.ad_value(14), s.ad_value(0)), s.ad_value(17)), A::add(s.ad_value(1), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17)), s.ad_value(17)), s.ad_value(16)), A::sub(A::scale(A::square(s.ad_value(16)), 0.3333333333333333), A::mul(s.ad_value(14), s.ad_value(15)))))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_ad(28, &A::limited_exp(s.ad_value(44)));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_div_from_scalar(29, 1.0, 28);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_limited_exp_ad(28, A::sub(s.ad_value(44), s.ad_value(97)));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_div_from_scalar_ad(13, 1.0, A::offset(A::square(s.ad_value(44)), 2.0));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_mul_ad_lhs(30, A::square(s.ad_value(44)), 13);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_scale_ad(31, A::mul(A::mul(s.ad_value(44), s.ad_value(13)), s.ad_value(13)), 4.0);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_mul_ad_lhs(32, A::mul(A::sub(A::scale(s.ad_value(13), 8.0), A::scale(s.ad_value(30), 12.0)), s.ad_value(13)), 13);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_sub(13, 214, 44);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_add_ad(36, A::scale(s.ad_value(13), 2.0), A::mul(s.ad_value(296), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(29)), s.ad_value(28)), A::mul(s.ad_value(99), A::offset(s.ad_value(31), 1.0)))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_sub_ad(37, A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::sub(A::add(A::offset(A::add(s.ad_value(29), s.ad_value(44)), (-1.0)), s.ad_value(28)), A::mul(s.ad_value(99), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30))))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::sub(A::add(s.ad_value(29), s.ad_value(28)), A::mul(s.ad_value(99), s.ad_value(32)))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_sub_ad(13, A::square(s.ad_value(36)), A::scale(A::mul(s.ad_value(37), s.ad_value(13)), 2.0));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (s.v[1808] != 0.0)) && (!(s.v[1809] != 0.0))) && (!(s.v[1810] != 0.0))) {
            s.store_add_ad_rhs(9, 44, A::scale(A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.copy_ad(47, 2);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.store_square(48, 47);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.store_sub_ad_rhs(8, 4, A::mul(s.ad_value(46), s.ad_value(270)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.store_sub_ad_rhs(105, 214, A::mul(s.ad_value(294), A::sqrt(A::offset(A::add(A::limited_exp(A::neg(s.ad_value(8))), s.ad_value(8)), (-1.0)))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.store_offset(43, 97, 3.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.store_scale_ad(106, A::sub(A::add(s.ad_value(105), s.ad_value(43)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(105), s.ad_value(43)), A::sub(s.ad_value(105), s.ad_value(43))), 40.0))), 0.5);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.store_sub_ad(107, A::sub(A::mul(A::sub(s.ad_value(214), s.ad_value(106)), A::sub(s.ad_value(214), s.ad_value(106))), A::mul(A::mul(s.ad_value(48), A::add(A::sub(s.ad_value(216), s.ad_value(106)), s.ad_value(4))), A::add(A::sub(s.ad_value(216), s.ad_value(106)), s.ad_value(4)))), A::mul(s.ad_value(296), s.ad_value(4)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.store_sub_ad(108, A::scale(A::sub(s.ad_value(214), s.ad_value(106)), 2.0), A::mul(A::scale(s.ad_value(48), 2.0), A::add(A::sub(s.ad_value(216), s.ad_value(106)), s.ad_value(4))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.store_square(109, 108);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.store_sub_from_scalar(110, 1.0, 48);
        }

        s.v[1811] = if (s.v[107] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) && (s.v[1811] != 0.0)) {
            s.store_scalar(107, 0.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.store_add_ad(49, A::sub(s.ad_value(97), s.ad_value(106)), A::ln(A::max_with_scalar(A::mul(s.ad_value(107), s.ad_value(297)), 1e-38)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.store_add(111, 107, 108);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.store_square(112, 111);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.store_sub_ad(113, A::add(A::div(s.ad_value(112), s.ad_value(49)), A::scale(s.ad_value(109), 0.5)), A::mul(s.ad_value(107), s.ad_value(110)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.store_div_ad_lhs(114, A::mul(s.ad_value(108), s.ad_value(111)), 113);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.store_sub_ad(115, A::scale(s.ad_value(109), 0.3333333333333333), A::mul(s.ad_value(107), s.ad_value(110)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.store_div_ad(116, A::mul(s.ad_value(111), s.ad_value(107)), A::add(s.ad_value(113), A::mul(s.ad_value(114), s.ad_value(115))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.store_add(117, 106, 116);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.store_limited_exp_ad(118, A::sub(s.ad_value(117), s.ad_value(97)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.store_add_ad(119, A::sub(A::scale(A::sub(s.ad_value(214), s.ad_value(117)), 2.0), A::mul(A::scale(s.ad_value(48), 2.0), A::add(A::sub(s.ad_value(216), s.ad_value(117)), s.ad_value(4)))), A::mul(s.ad_value(296), s.ad_value(118)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.store_sub_ad(120, A::sub(A::mul(A::sub(s.ad_value(214), s.ad_value(117)), A::sub(s.ad_value(214), s.ad_value(117))), A::mul(A::mul(s.ad_value(48), A::add(A::sub(s.ad_value(216), s.ad_value(117)), s.ad_value(4))), A::add(A::sub(s.ad_value(216), s.ad_value(117)), s.ad_value(4)))), A::mul(s.ad_value(296), A::add(s.ad_value(4), s.ad_value(118))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.store_mul_ad(121, A::scale(s.ad_value(120), 2.0), A::sub(A::sub_from_scalar(2.0, A::scale(s.ad_value(48), 2.0)), A::mul(s.ad_value(296), s.ad_value(118))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.store_div_ad(122, A::scale(s.ad_value(120), 2.0), A::add(s.ad_value(119), A::sqrt(A::sub(A::square(s.ad_value(119)), s.ad_value(121)))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1807] != 0.0)) && (!(s.v[1808] != 0.0))) {
            s.store_add(9, 117, 122);
        }

        s.v[1812] = if (((s.v[214]) as f64).abs() <= s.v[101]) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (s.v[1812] != 0.0)) {
            s.store_scale_ad(167, A::square(s.ad_value(96)), (0.16666666666666666 * 0.7071067811865475));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (s.v[1812] != 0.0)) {
            s.store_mul_ad(9, A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(99))), s.ad_value(294)), s.ad_value(167)), 1.0));
        }

        s.v[1813] = if (s.v[214] < (-s.v[101])) { 1.0 } else { 0.0 };

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_neg(10, 214);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_scaled_mul(11, 10, 96, 1.25);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_scale_ad(12, A::sub(A::offset(s.ad_value(11), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(11), (-6.0)), A::offset(s.ad_value(11), (-6.0))), 64.0))), 0.5);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_sub(13, 10, 12);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_add_ad(14, A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::offset(s.ad_value(12), 1.0)));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_sub_ad_lhs(16, A::scale(s.ad_value(13), 2.0), 296);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_add(0, 14, 16);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_add_ad(1, A::square(s.ad_value(0)), A::mul(s.ad_value(17), A::sub(A::scale(A::square(s.ad_value(16)), 0.5), s.ad_value(14))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_add_ad_rhs(18, 12, A::div(A::mul(A::mul(s.ad_value(14), s.ad_value(0)), s.ad_value(17)), A::add(s.ad_value(1), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17)), s.ad_value(17)), s.ad_value(16)), A::sub(A::scale(A::square(s.ad_value(16)), 0.3333333333333333), s.ad_value(14))))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_ad(28, &A::limited_exp(s.ad_value(18)));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_div_from_scalar(29, 1.0, 28);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_div_from_scalar_ad(13, 1.0, A::offset(A::square(s.ad_value(18)), 2.0));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_mul_ad_lhs(30, A::square(s.ad_value(18)), 13);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_scale_ad(31, A::mul(A::mul(s.ad_value(18), s.ad_value(13)), s.ad_value(13)), 4.0);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_mul_ad_lhs(32, A::mul(A::sub(A::scale(s.ad_value(13), 8.0), A::scale(s.ad_value(30), 12.0)), s.ad_value(13)), 13);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_sub(13, 10, 18);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_mul(33, 99, 29);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_add_ad(36, A::scale(s.ad_value(13), 2.0), A::mul(s.ad_value(296), A::add(A::sub(A::offset(s.ad_value(28), (-1.0)), s.ad_value(33)), A::mul(s.ad_value(99), A::sub_from_scalar(1.0, s.ad_value(31))))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_sub_ad(37, A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::add(A::add(A::offset(A::sub(s.ad_value(28), s.ad_value(18)), (-1.0)), s.ad_value(33)), A::mul(s.ad_value(99), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30))))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::sub(A::add(s.ad_value(28), s.ad_value(33)), A::mul(s.ad_value(99), s.ad_value(32)))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_sub_ad(13, A::square(s.ad_value(36)), A::scale(A::mul(s.ad_value(37), s.ad_value(13)), 2.0));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_offset_ad(13, A::sqrt(A::offset(A::square(s.ad_value(13)), 6.4e-7)), (-0.0008));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (s.v[1813] != 0.0)) {
            s.store_sub_ad(9, A::neg(s.ad_value(18)), A::scale(A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_div_from_scalar_ad(38, 1.0, A::offset(A::scale(s.ad_value(294), 0.7324648775608221), 1.25));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_mul_ad_lhs(39, A::offset(A::mul(A::scale(s.ad_value(95), 1.25), s.ad_value(38)), (-1.0)), 38);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_mul_ad(40, A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_limited_exp_ad(13, A::neg(s.ad_value(40)));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_sub_from_scalar(41, 1.0, 13);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_sub_ad(42, A::add(s.ad_value(214), A::scale(s.ad_value(296), 0.5)), A::mul(s.ad_value(294), A::sqrt(A::sub(A::add(s.ad_value(214), A::scale(s.ad_value(296), 0.25)), s.ad_value(41)))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_offset(43, 97, 3.0);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_sub_ad(12, A::scale(A::sub(A::add(s.ad_value(42), s.ad_value(43)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(42), s.ad_value(43)), A::sub(s.ad_value(42), s.ad_value(43))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(43), A::sqrt(A::offset(A::square(s.ad_value(43)), 5.0))), 0.5));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_sub(13, 214, 12);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_limited_exp_ad(33, A::neg(s.ad_value(12)));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_div_from_scalar_ad(34, 1.0, A::offset(A::square(s.ad_value(12)), 2.0));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_mul_ad_lhs(30, A::square(s.ad_value(12)), 34);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_scale_ad(31, A::mul(A::mul(s.ad_value(12), s.ad_value(34)), s.ad_value(34)), 4.0);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_mul_ad_lhs(32, A::mul(A::sub(A::scale(s.ad_value(34), 8.0), A::scale(s.ad_value(30), 12.0)), s.ad_value(34)), 34);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_max_from_scalar_ad(14, 1e-40, A::sub(A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::sub(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), A::mul(s.ad_value(99), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)))))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_sub_from_scalar_ad(15, 1.0, A::scale(A::mul(s.ad_value(296), A::sub(s.ad_value(33), A::mul(s.ad_value(99), s.ad_value(32)))), 0.5));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_add_ad(16, A::scale(s.ad_value(13), 2.0), A::mul(s.ad_value(296), A::sub(A::sub_from_scalar(1.0, s.ad_value(33)), A::mul(s.ad_value(99), A::offset(s.ad_value(31), 1.0)))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_add_ad(17, A::sub(s.ad_value(97), s.ad_value(12)), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_add(0, 14, 16);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_add_ad(1, A::square(s.ad_value(0)), A::mul(s.ad_value(17), A::sub(A::scale(A::square(s.ad_value(16)), 0.5), A::mul(s.ad_value(14), s.ad_value(15)))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_add_ad_rhs(44, 12, A::div(A::mul(A::mul(s.ad_value(14), s.ad_value(0)), s.ad_value(17)), A::add(s.ad_value(1), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17)), s.ad_value(17)), s.ad_value(16)), A::sub(A::scale(A::square(s.ad_value(16)), 0.3333333333333333), A::mul(s.ad_value(14), s.ad_value(15)))))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_ad(28, &A::limited_exp(s.ad_value(44)));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_div_from_scalar(29, 1.0, 28);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_limited_exp_ad(28, A::sub(s.ad_value(44), s.ad_value(97)));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_div_from_scalar_ad(13, 1.0, A::offset(A::square(s.ad_value(44)), 2.0));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_mul_ad_lhs(30, A::square(s.ad_value(44)), 13);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_scale_ad(31, A::mul(A::mul(s.ad_value(44), s.ad_value(13)), s.ad_value(13)), 4.0);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_mul_ad_lhs(32, A::mul(A::sub(A::scale(s.ad_value(13), 8.0), A::scale(s.ad_value(30), 12.0)), s.ad_value(13)), 13);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_sub(13, 214, 44);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_add_ad(36, A::scale(s.ad_value(13), 2.0), A::mul(s.ad_value(296), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(29)), s.ad_value(28)), A::mul(s.ad_value(99), A::offset(s.ad_value(31), 1.0)))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_sub_ad(37, A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::sub(A::add(A::offset(A::add(s.ad_value(29), s.ad_value(44)), (-1.0)), s.ad_value(28)), A::mul(s.ad_value(99), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30))))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::sub(A::add(s.ad_value(29), s.ad_value(28)), A::mul(s.ad_value(99), s.ad_value(32)))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_sub_ad(13, A::square(s.ad_value(36)), A::scale(A::mul(s.ad_value(37), s.ad_value(13)), 2.0));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_offset_ad(13, A::sqrt(A::offset(A::square(s.ad_value(13)), 6.4e-7)), (-0.0008));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1807] != 0.0))) && (!(s.v[1812] != 0.0))) && (!(s.v[1813] != 0.0))) {
            s.store_add_ad_rhs(9, 44, A::scale(A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.copy_ad(123, 9);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_scalar(102, 1e-7);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_scalar(103, 2.0);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_scale_ad(35, A::square(s.ad_value(96)), (0.16666666666666666 * 0.7071067811865475));
        }

    }

    pub(super) fn stamp_reactive_block_27(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_div_ad_lhs(167, A::scale(A::neg(s.ad_value(726)), s.v[184]), 300);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_mul_ad_lhs(168, A::mul(s.ad_value(725), A::add(A::limited_exp(A::scale(s.ad_value(167), 0.5)), A::scale(A::limited_exp(s.ad_value(167)), 2.0))), 724);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_offset_ad(24, A::scale(A::mul(A::offset(A::scale(s.ad_value(743), 1.0 / (s.v[184])), 1.0), A::scale(s.ad_value(706), (1.602176462e-19 * (p.p74 * p.p74)))), 1.0 / ((2.0 * s.v[180]))), p.p294);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_add_ad(6, A::sub(s.ad_value(24), A::mul(A::mul(s.ad_value(3), A::mul(s.ad_value(216), s.ad_value(269))), s.ad_value(727))), A::mul(A::offset(s.ad_value(3), 1.0), s.ad_value(46)));
        }

        s.v[1814] = if (((s.v[214]) as f64).abs() <= s.v[102]) { 1.0 } else { 0.0 };

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1814] != 0.0)) {
            s.store_scale_ad(167, A::square(s.ad_value(96)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1814] != 0.0)) {
            s.store_mul_ad(22, A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(99))), s.ad_value(294)), s.ad_value(167)), 1.0));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1814] != 0.0))) {
            s.store_scale_ad(167, A::add(A::tanh(A::scale(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0))), A::tanh(A::scale(A::add(s.ad_value(214), s.ad_value(103)), 5.0))), 0.5);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1814] != 0.0))) {
            s.store_mul_ad_lhs(45, A::mul(A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(99))), s.ad_value(294)), s.ad_value(35)), 1.0)), 167);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1814] != 0.0))) {
            s.store_limited_exp_ad(167, A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1814] != 0.0))) {
            s.store_limited_exp_ad(168, A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1814] != 0.0))) {
            let assign41080_ad_e66595: A = {
                if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))), 1.0))
                } else {
                    let assign41080_ad_e66594: A = {
                        if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)))
                        } else {
                            {
                                if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign41080_ad_e66594
                }
            };
            let assign41080_ad_e66677: A = {
                if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))), 1.0))
                } else {
                    let assign41080_ad_e66676: A = {
                        if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)))
                        } else {
                            {
                                if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign41080_ad_e66676
                }
            };
            s.store_sub_ad(169, assign41080_ad_e66595, assign41080_ad_e66677);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1814] != 0.0))) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div(s.ad_value(169), A::offset(s.ad_value(3), 1.0))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1814] != 0.0))) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1814] != 0.0))) {
            s.store_limited_exp_ad(173, A::neg(s.ad_value(123)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1814] != 0.0))) {
            s.store_square(174, 123);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1814] != 0.0))) {
            s.store_div_from_scalar_ad(175, 1.0, A::offset(s.ad_value(174), 2.0));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1814] != 0.0))) {
            s.store_limited_exp_ad(176, A::neg(s.ad_value(97)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1814] != 0.0))) {
            s.store_limited_exp_ad(177, A::sub(s.ad_value(123), s.ad_value(97)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1814] != 0.0))) {
            let assign41160_ad_e66813: A = A::sub(A::sub(A::mul(A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123))), A::mul(A::mul(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170))), A::add(s.ad_value(216), s.ad_value(170)))), A::mul(s.ad_value(296), A::sub(A::add(A::add(A::add(A::sub(s.ad_value(173), s.ad_value(171)), s.ad_value(123)), s.ad_value(170)), s.ad_value(177)), A::mul(s.ad_value(176), A::add(A::offset(s.ad_value(123), 1.0), A::mul(s.ad_value(175), s.ad_value(174)))))));
            s.store_ad(19, &assign41160_ad_e66813);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1814] != 0.0))) {
            let assign41170_ad_e66885: A = A::sub(A::sub(A::add(s.ad_value(177), A::mul(s.ad_value(176), A::offset(A::add(A::mul(A::scale(s.ad_value(123), (-2.0)), s.ad_value(175)), A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(123), 2.0), s.ad_value(123)), s.ad_value(123)), s.ad_value(175)), s.ad_value(175))), (-1.0)))), s.ad_value(173)), A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))));
            let assign41170_ad_e66901: A = A::sub(A::add(A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170))), s.ad_value(2)), s.ad_value(2)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::scale(s.ad_value(214), 2.0)), A::scale(s.ad_value(123), 2.0)), A::mul(s.ad_value(296), A::offset(A::add(assign41170_ad_e66885, A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)))), 1.0)));
            s.store_ad(20, &assign41170_ad_e66901);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1814] != 0.0))) {
            let assign41180_ad_e66954: A = A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::mul(A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0))));
            let assign41180_ad_e67013: A = A::mul(A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::add(A::sub(A::sub_from_scalar(1.0, A::div(s.ad_value(167), A::offset(s.ad_value(167), 1.0))), s.ad_value(171)), A::mul(A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::offset(s.ad_value(167), 1.0)), A::offset(A::div_from_scalar(1.0, A::offset(s.ad_value(3), 1.0)), 1.0))));
            let assign41180_ad_e67042: A = A::sub(A::sub(assign41180_ad_e66954, A::mul(s.ad_value(296), A::sub(A::sub(A::add(s.ad_value(173), s.ad_value(177)), A::mul(A::mul(A::scale(s.ad_value(176), 2.0), s.ad_value(175)), A::sub_from_scalar(1.0, A::mul(A::mul(s.ad_value(174), s.ad_value(175)), A::sub_from_scalar(5.0, A::mul(A::scale(s.ad_value(174), 4.0), s.ad_value(175))))))), assign41180_ad_e67013))), A::div(A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(167), 1.0))));
            s.store_offset_ad(21, assign41180_ad_e67042, 2.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1814] != 0.0))) {
            s.store_sub_ad_rhs(22, 123, A::mul(A::div(s.ad_value(19), s.ad_value(20)), A::offset(A::div(A::mul(s.ad_value(19), s.ad_value(21)), A::mul(A::scale(s.ad_value(20), 2.0), s.ad_value(20))), 1.0)));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.copy_ad(123, 22);
        }

        s.v[1815] = if (((s.v[214]) as f64).abs() <= s.v[102]) { 1.0 } else { 0.0 };

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1815] != 0.0)) {
            s.store_scale_ad(167, A::square(s.ad_value(96)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1815] != 0.0)) {
            s.store_mul_ad(22, A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(99))), s.ad_value(294)), s.ad_value(167)), 1.0));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1815] != 0.0))) {
            s.store_scale_ad(167, A::add(A::tanh(A::scale(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0))), A::tanh(A::scale(A::add(s.ad_value(214), s.ad_value(103)), 5.0))), 0.5);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1815] != 0.0))) {
            s.store_mul_ad_lhs(45, A::mul(A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(99))), s.ad_value(294)), s.ad_value(35)), 1.0)), 167);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1815] != 0.0))) {
            s.store_limited_exp_ad(167, A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1815] != 0.0))) {
            s.store_limited_exp_ad(168, A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1815] != 0.0))) {
            let assign41280_ad_e67286: A = {
                if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))), 1.0))
                } else {
                    let assign41280_ad_e67285: A = {
                        if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)))
                        } else {
                            {
                                if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign41280_ad_e67285
                }
            };
            let assign41280_ad_e67368: A = {
                if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))), 1.0))
                } else {
                    let assign41280_ad_e67367: A = {
                        if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)))
                        } else {
                            {
                                if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign41280_ad_e67367
                }
            };
            s.store_sub_ad(169, assign41280_ad_e67286, assign41280_ad_e67368);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1815] != 0.0))) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div(s.ad_value(169), A::offset(s.ad_value(3), 1.0))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1815] != 0.0))) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1815] != 0.0))) {
            s.store_limited_exp_ad(173, A::neg(s.ad_value(123)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1815] != 0.0))) {
            s.store_square(174, 123);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1815] != 0.0))) {
            s.store_div_from_scalar_ad(175, 1.0, A::offset(s.ad_value(174), 2.0));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1815] != 0.0))) {
            s.store_limited_exp_ad(176, A::neg(s.ad_value(97)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1815] != 0.0))) {
            s.store_limited_exp_ad(177, A::sub(s.ad_value(123), s.ad_value(97)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1815] != 0.0))) {
            let assign41360_ad_e67504: A = A::sub(A::sub(A::mul(A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123))), A::mul(A::mul(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170))), A::add(s.ad_value(216), s.ad_value(170)))), A::mul(s.ad_value(296), A::sub(A::add(A::add(A::add(A::sub(s.ad_value(173), s.ad_value(171)), s.ad_value(123)), s.ad_value(170)), s.ad_value(177)), A::mul(s.ad_value(176), A::add(A::offset(s.ad_value(123), 1.0), A::mul(s.ad_value(175), s.ad_value(174)))))));
            s.store_ad(19, &assign41360_ad_e67504);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1815] != 0.0))) {
            let assign41370_ad_e67576: A = A::sub(A::sub(A::add(s.ad_value(177), A::mul(s.ad_value(176), A::offset(A::add(A::mul(A::scale(s.ad_value(123), (-2.0)), s.ad_value(175)), A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(123), 2.0), s.ad_value(123)), s.ad_value(123)), s.ad_value(175)), s.ad_value(175))), (-1.0)))), s.ad_value(173)), A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))));
            let assign41370_ad_e67592: A = A::sub(A::add(A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170))), s.ad_value(2)), s.ad_value(2)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::scale(s.ad_value(214), 2.0)), A::scale(s.ad_value(123), 2.0)), A::mul(s.ad_value(296), A::offset(A::add(assign41370_ad_e67576, A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)))), 1.0)));
            s.store_ad(20, &assign41370_ad_e67592);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1815] != 0.0))) {
            let assign41380_ad_e67645: A = A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::mul(A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0))));
            let assign41380_ad_e67704: A = A::mul(A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::add(A::sub(A::sub_from_scalar(1.0, A::div(s.ad_value(167), A::offset(s.ad_value(167), 1.0))), s.ad_value(171)), A::mul(A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::offset(s.ad_value(167), 1.0)), A::offset(A::div_from_scalar(1.0, A::offset(s.ad_value(3), 1.0)), 1.0))));
            let assign41380_ad_e67733: A = A::sub(A::sub(assign41380_ad_e67645, A::mul(s.ad_value(296), A::sub(A::sub(A::add(s.ad_value(173), s.ad_value(177)), A::mul(A::mul(A::scale(s.ad_value(176), 2.0), s.ad_value(175)), A::sub_from_scalar(1.0, A::mul(A::mul(s.ad_value(174), s.ad_value(175)), A::sub_from_scalar(5.0, A::mul(A::scale(s.ad_value(174), 4.0), s.ad_value(175))))))), assign41380_ad_e67704))), A::div(A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(167), 1.0))));
            s.store_offset_ad(21, assign41380_ad_e67733, 2.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1815] != 0.0))) {
            s.store_sub_ad_rhs(22, 123, A::mul(A::div(s.ad_value(19), s.ad_value(20)), A::offset(A::div(A::mul(s.ad_value(19), s.ad_value(21)), A::mul(A::scale(s.ad_value(20), 2.0), s.ad_value(20))), 1.0)));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.copy_ad(123, 22);
        }

        s.v[1816] = if (((s.v[214]) as f64).abs() <= s.v[102]) { 1.0 } else { 0.0 };

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1816] != 0.0)) {
            s.store_scale_ad(167, A::square(s.ad_value(96)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1816] != 0.0)) {
            s.store_mul_ad(22, A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(99))), s.ad_value(294)), s.ad_value(167)), 1.0));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1816] != 0.0))) {
            s.store_scale_ad(167, A::add(A::tanh(A::scale(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0))), A::tanh(A::scale(A::add(s.ad_value(214), s.ad_value(103)), 5.0))), 0.5);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1816] != 0.0))) {
            s.store_mul_ad_lhs(45, A::mul(A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(99))), s.ad_value(294)), s.ad_value(35)), 1.0)), 167);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1816] != 0.0))) {
            s.store_limited_exp_ad(167, A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1816] != 0.0))) {
            s.store_limited_exp_ad(168, A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1816] != 0.0))) {
            let assign41480_ad_e67977: A = {
                if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))), 1.0))
                } else {
                    let assign41480_ad_e67976: A = {
                        if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)))
                        } else {
                            {
                                if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign41480_ad_e67976
                }
            };
            let assign41480_ad_e68059: A = {
                if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))), 1.0))
                } else {
                    let assign41480_ad_e68058: A = {
                        if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)))
                        } else {
                            {
                                if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign41480_ad_e68058
                }
            };
            s.store_sub_ad(169, assign41480_ad_e67977, assign41480_ad_e68059);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1816] != 0.0))) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div(s.ad_value(169), A::offset(s.ad_value(3), 1.0))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1816] != 0.0))) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1816] != 0.0))) {
            s.store_limited_exp_ad(173, A::neg(s.ad_value(123)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1816] != 0.0))) {
            s.store_square(174, 123);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1816] != 0.0))) {
            s.store_div_from_scalar_ad(175, 1.0, A::offset(s.ad_value(174), 2.0));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1816] != 0.0))) {
            s.store_limited_exp_ad(176, A::neg(s.ad_value(97)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1816] != 0.0))) {
            s.store_limited_exp_ad(177, A::sub(s.ad_value(123), s.ad_value(97)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1816] != 0.0))) {
            let assign41560_ad_e68195: A = A::sub(A::sub(A::mul(A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123))), A::mul(A::mul(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170))), A::add(s.ad_value(216), s.ad_value(170)))), A::mul(s.ad_value(296), A::sub(A::add(A::add(A::add(A::sub(s.ad_value(173), s.ad_value(171)), s.ad_value(123)), s.ad_value(170)), s.ad_value(177)), A::mul(s.ad_value(176), A::add(A::offset(s.ad_value(123), 1.0), A::mul(s.ad_value(175), s.ad_value(174)))))));
            s.store_ad(19, &assign41560_ad_e68195);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1816] != 0.0))) {
            let assign41570_ad_e68267: A = A::sub(A::sub(A::add(s.ad_value(177), A::mul(s.ad_value(176), A::offset(A::add(A::mul(A::scale(s.ad_value(123), (-2.0)), s.ad_value(175)), A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(123), 2.0), s.ad_value(123)), s.ad_value(123)), s.ad_value(175)), s.ad_value(175))), (-1.0)))), s.ad_value(173)), A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))));
            let assign41570_ad_e68283: A = A::sub(A::add(A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170))), s.ad_value(2)), s.ad_value(2)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::scale(s.ad_value(214), 2.0)), A::scale(s.ad_value(123), 2.0)), A::mul(s.ad_value(296), A::offset(A::add(assign41570_ad_e68267, A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)))), 1.0)));
            s.store_ad(20, &assign41570_ad_e68283);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1816] != 0.0))) {
            let assign41580_ad_e68336: A = A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::mul(A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0))));
            let assign41580_ad_e68395: A = A::mul(A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::add(A::sub(A::sub_from_scalar(1.0, A::div(s.ad_value(167), A::offset(s.ad_value(167), 1.0))), s.ad_value(171)), A::mul(A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::offset(s.ad_value(167), 1.0)), A::offset(A::div_from_scalar(1.0, A::offset(s.ad_value(3), 1.0)), 1.0))));
            let assign41580_ad_e68424: A = A::sub(A::sub(assign41580_ad_e68336, A::mul(s.ad_value(296), A::sub(A::sub(A::add(s.ad_value(173), s.ad_value(177)), A::mul(A::mul(A::scale(s.ad_value(176), 2.0), s.ad_value(175)), A::sub_from_scalar(1.0, A::mul(A::mul(s.ad_value(174), s.ad_value(175)), A::sub_from_scalar(5.0, A::mul(A::scale(s.ad_value(174), 4.0), s.ad_value(175))))))), assign41580_ad_e68395))), A::div(A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(167), 1.0))));
            s.store_offset_ad(21, assign41580_ad_e68424, 2.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1816] != 0.0))) {
            s.store_sub_ad_rhs(22, 123, A::mul(A::div(s.ad_value(19), s.ad_value(20)), A::offset(A::div(A::mul(s.ad_value(19), s.ad_value(21)), A::mul(A::scale(s.ad_value(20), 2.0), s.ad_value(20))), 1.0)));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_scale(50, 269, 3.912023005);
        }

        s.v[1817] = if (s.v[22] <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1817] != 0.0)) {
            s.store_scalar(306, 0.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1817] != 0.0)) {
            s.store_sub(51, 214, 22);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1817] != 0.0)) {
            s.copy_ad(312, 50);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1817] != 0.0)) {
            s.store_scalar(458, 1.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1817] != 0.0)) {
            s.store_scalar(334, 1.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1817] != 0.0)) {
            s.store_scalar(853, 1.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1817] != 0.0)) {
            s.store_scalar(343, 1.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1817] != 0.0)) {
            s.store_scalar(339, 1.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1817] != 0.0)) {
            s.store_scalar(363, 1.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1817] != 0.0)) {
            s.store_scalar(365, 1.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1817] != 0.0)) {
            s.copy_ad(455, 453);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (s.v[1817] != 0.0)) {
            s.copy_ad(454, 452);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_div_from_scalar_ad(54, 1.0, A::offset(A::square(s.ad_value(22)), 2.0));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_mul_ad_lhs(55, A::square(s.ad_value(22)), 54);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_ad(53, &A::limited_exp(s.ad_value(22)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_div_from_scalar(56, 1.0, 53);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_limited_exp_ad(53, A::sub(s.ad_value(22), s.ad_value(97)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_sub_ad_rhs(57, 53, A::mul(A::limited_exp(A::neg(s.ad_value(97))), A::add(A::offset(s.ad_value(22), 1.0), s.ad_value(55))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_sub_ad_lhs(58, A::mul(A::mul(A::sub(s.ad_value(214), s.ad_value(22)), A::sub(s.ad_value(214), s.ad_value(22))), A::div_from_scalar(1.0, s.ad_value(296))), 57);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_offset_ad(58, A::scale(A::add(A::offset(s.ad_value(58), (-0.001)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(58), (-0.001)), A::offset(s.ad_value(58), (-0.001))), ((4.0 * 1e-5) * 1e-5)))), 0.5), 0.001);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_sqrt(59, 58);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_mul_ad_rhs(61, 294, A::sqrt(A::add(s.ad_value(58), s.ad_value(57))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_div_ad(306, A::mul(A::mul(s.ad_value(296), s.ad_value(57)), s.ad_value(269)), A::add(s.ad_value(61), A::mul(s.ad_value(294), s.ad_value(59))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_mul_ad_lhs(247, A::mul(s.ad_value(59), s.ad_value(294)), 269);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.copy_ad(76, 56);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.copy_ad(78, 57);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_scalar(335, (1e-8 / (s.v[200] * p.p76)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_mul_ad_rhs(308, 335, A::add(s.ad_value(247), A::scale(s.ad_value(306), s.v[338])));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_limited_exp_ad(169, A::mul(s.ad_value(757), A::ln(A::max_with_scalar(A::scale(A::offset(A::div(s.ad_value(306), s.ad_value(247)), 1.0), 0.5), 1e-38))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_add_ad(170, A::mul(A::add(s.ad_value(750), A::mul(s.ad_value(760), s.ad_value(218))), A::pow(s.ad_value(308), s.ad_value(651))), A::div(s.ad_value(754), s.ad_value(169)));
        }

    }

    pub(super) fn stamp_reactive_block_28(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_offset(171, 170, 1.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_scale_ad(309, A::add(A::offset(s.ad_value(171), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(171), (-1.0)), A::offset(s.ad_value(171), (-1.0))), ((0.25 * 0.0015) * 0.0015)))), 0.5);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_div_from_scalar_ad(448, 1.0, A::scale(A::pow_from_scalar((s.v[183] * 1000000.0), s.ad_value(771)), p.p2));
        }

        s.v[1818] = if (p.p33 == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1818] != 0.0)) {
            s.store_scalar(456, 0.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1818] != 0.0))) {
            s.store_offset_ad(167, A::mul(s.ad_value(770), s.ad_value(306)), 1.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1818] != 0.0))) {
            s.store_mul_ad_rhs(168, 787, A::sub(s.ad_value(274), s.ad_value(299)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1818] != 0.0))) {
            s.store_add_ad_lhs(169, A::div_from_scalar(1.0, s.ad_value(167)), 168);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1818] != 0.0))) {
            s.store_add_ad_rhs(170, 169, A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1818] != 0.0))) {
            s.store_mul_ad_lhs(456, A::scale(A::mul(A::add(s.ad_value(777), A::mul(s.ad_value(776), s.ad_value(170))), s.ad_value(448)), p.p2), 652);
        }

        s.v[1819] = if (p.p33 == 2.0) { 1.0 } else { 0.0 };

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1818] != 0.0))) && (s.v[1819] != 0.0)) {
            s.store_mul_ad_lhs(456, A::add(A::add(s.ad_value(452), A::scale(A::mul(A::add(s.ad_value(777), A::mul(s.ad_value(776), s.ad_value(170))), s.ad_value(448)), p.p2)), s.ad_value(453)), 652);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_mul_ad_lhs(310, A::div(A::scale(s.ad_value(746), 2.0), s.ad_value(740)), 309);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_scale(311, 310, s.v[184]);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_mul_ad_rhs(173, 742, A::add(s.ad_value(306), A::scale(s.ad_value(269), 2.0)));
        }

        s.v[1820] = if (s.v[456] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1820] != 0.0)) {
            s.store_scale(324, 746, (s.v[183] * s.v[199]));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1820] != 0.0)) {
            s.store_mul(167, 324, 456);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1820] != 0.0)) {
            s.store_scale(325, 167, 2.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1820] != 0.0)) {
            s.store_add_ad(326, A::add(s.ad_value(173), s.ad_value(311)), A::mul(A::scale(s.ad_value(173), 3.0), s.ad_value(167)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1820] != 0.0)) {
            s.store_mul_ad_rhs(327, 173, A::add(s.ad_value(311), A::mul(A::scale(s.ad_value(173), 2.0), s.ad_value(167))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1820] != 0.0)) {
            s.store_div_ad_lhs(312, A::sub(s.ad_value(326), A::sqrt(A::sub(A::square(s.ad_value(326)), A::mul(A::scale(s.ad_value(325), 2.0), s.ad_value(327))))), 325);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1820] != 0.0))) {
            s.store_div_ad(312, A::mul(s.ad_value(311), s.ad_value(173)), A::add(s.ad_value(311), s.ad_value(173)));
        }

        s.v[1821] = if ((p.p1349 == 0.0) && (p.p1350 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1821] != 0.0)) {
            s.store_scalar(1019, 1.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1821] != 0.0))) {
            s.store_div_from_scalar_ad(168, s.v[184], A::offset(A::sqrt(A::mul(s.ad_value(782), s.ad_value(275))), s.v[184]));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1821] != 0.0))) {
            s.store_offset_ad(1019, A::div(A::sub(A::scale(s.ad_value(168), p.p1349), A::mul(A::mul(A::scale(s.ad_value(168), p.p1350), A::powf(s.ad_value(306), p.p1351)), s.ad_value(269))), A::offset(A::scale(s.ad_value(218), p.p1352), 1.0)), 1.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1821] != 0.0))) {
            s.store_scale_ad(1019, A::add(A::offset(s.ad_value(1019), 0.1), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(1019), (-0.1)), A::offset(s.ad_value(1019), (-0.1))), ((0.25 * 0.0005) * 0.0005)))), 0.5);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_offset_ad(312, A::scale(A::add(A::offset(s.ad_value(312), (-0.001)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(312), (-0.001)), A::offset(s.ad_value(312), (-0.001))), ((4.0 * 1e-5) * 1e-5)))), 0.5), 0.001);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_div(312, 312, 1019);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_pow_ad(174, A::offset(A::div(s.ad_value(226), s.ad_value(312)), 1e-6), A::div_from_scalar(1.0, s.ad_value(656)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_pow_ad(175, A::offset(s.ad_value(174), 1.0), A::neg(s.ad_value(656)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_mul(315, 226, 175);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_mul_ad_lhs(318, A::add(s.ad_value(315), s.ad_value(224)), 270);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_add_ad_lhs(98, A::div(A::scale(s.ad_value(251), 2.0), s.ad_value(267)), 318);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_limited_exp_ad(100, A::neg(s.ad_value(98)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_scale(101, 95, 0.001);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_div_ad_lhs(167, A::scale(A::neg(s.ad_value(726)), s.v[184]), 300);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_mul_ad(168, A::mul(s.ad_value(725), A::add(A::limited_exp(A::scale(s.ad_value(167), 0.5)), A::scale(A::limited_exp(s.ad_value(167)), 2.0))), A::add(A::mul(s.ad_value(226), s.ad_value(270)), A::mul(s.ad_value(724), s.ad_value(270))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            let assign42310_ad_e69507: A = A::add(A::sub(A::add(A::div(A::mul(A::offset(A::scale(s.ad_value(743), 1.0 / (s.v[184])), 1.0), A::scale(s.ad_value(706), (1.602176462e-19 * (p.p74 * p.p74)))), A::scale(s.ad_value(269), (2.0 * s.v[180]))), A::div_from_scalar(p.p294, s.ad_value(269))), A::mul(A::mul(s.ad_value(3), s.ad_value(216)), s.ad_value(727))), A::mul(A::offset(s.ad_value(3), 1.0), s.ad_value(168)));
            s.store_ad(4, &assign42310_ad_e69507);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_add_ad_rhs(104, 4, A::mul(s.ad_value(294), A::sqrt(A::offset(A::add(A::limited_exp(A::neg(s.ad_value(4))), s.ad_value(4)), (-1.0)))));
        }

        s.v[1822] = if (s.v[4] < s.v[98]) { 1.0 } else { 0.0 };

        s.v[1823] = if (s.v[214] < s.v[104]) { 1.0 } else { 0.0 };

        s.v[1824] = if (((s.v[214]) as f64).abs() <= s.v[101]) { 1.0 } else { 0.0 };

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (s.v[1824] != 0.0)) {
            s.store_scale_ad(167, A::square(s.ad_value(96)), (0.16666666666666666 * 0.7071067811865475));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (s.v[1824] != 0.0)) {
            s.store_mul_ad(9, A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(100))), s.ad_value(294)), s.ad_value(167)), 1.0));
        }

        s.v[1825] = if (s.v[214] < (-s.v[101])) { 1.0 } else { 0.0 };

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (s.v[1825] != 0.0)) {
            s.store_neg(10, 214);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (s.v[1825] != 0.0)) {
            s.store_scaled_mul(11, 10, 96, 1.25);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (s.v[1825] != 0.0)) {
            s.store_scale_ad(12, A::sub(A::offset(s.ad_value(11), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(11), (-6.0)), A::offset(s.ad_value(11), (-6.0))), 64.0))), 0.5);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (s.v[1825] != 0.0)) {
            s.store_sub(13, 10, 12);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (s.v[1825] != 0.0)) {
            s.store_add_ad(14, A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::offset(s.ad_value(12), 1.0)));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (s.v[1825] != 0.0)) {
            s.store_sub_ad_lhs(16, A::scale(s.ad_value(13), 2.0), 296);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (s.v[1825] != 0.0)) {
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (s.v[1825] != 0.0)) {
            s.store_add(0, 14, 16);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (s.v[1825] != 0.0)) {
            s.store_add_ad(1, A::square(s.ad_value(0)), A::mul(s.ad_value(17), A::sub(A::scale(A::square(s.ad_value(16)), 0.5), s.ad_value(14))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (s.v[1825] != 0.0)) {
            s.store_add_ad_rhs(18, 12, A::div(A::mul(A::mul(s.ad_value(14), s.ad_value(0)), s.ad_value(17)), A::add(s.ad_value(1), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17)), s.ad_value(17)), s.ad_value(16)), A::sub(A::scale(A::square(s.ad_value(16)), 0.3333333333333333), s.ad_value(14))))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (s.v[1825] != 0.0)) {
            s.store_ad(28, &A::limited_exp(s.ad_value(18)));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (s.v[1825] != 0.0)) {
            s.store_div_from_scalar(29, 1.0, 28);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (s.v[1825] != 0.0)) {
            s.store_div_from_scalar_ad(13, 1.0, A::offset(A::square(s.ad_value(18)), 2.0));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (s.v[1825] != 0.0)) {
            s.store_mul_ad_lhs(30, A::square(s.ad_value(18)), 13);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (s.v[1825] != 0.0)) {
            s.store_scale_ad(31, A::mul(A::mul(s.ad_value(18), s.ad_value(13)), s.ad_value(13)), 4.0);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (s.v[1825] != 0.0)) {
            s.store_mul_ad_lhs(32, A::mul(A::sub(A::scale(s.ad_value(13), 8.0), A::scale(s.ad_value(30), 12.0)), s.ad_value(13)), 13);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (s.v[1825] != 0.0)) {
            s.store_sub(13, 10, 18);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (s.v[1825] != 0.0)) {
            s.store_mul(33, 100, 29);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (s.v[1825] != 0.0)) {
            s.store_add_ad(36, A::scale(s.ad_value(13), 2.0), A::mul(s.ad_value(296), A::add(A::sub(A::offset(s.ad_value(28), (-1.0)), s.ad_value(33)), A::mul(s.ad_value(100), A::sub_from_scalar(1.0, s.ad_value(31))))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (s.v[1825] != 0.0)) {
            s.store_sub_ad(37, A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::add(A::add(A::offset(A::sub(s.ad_value(28), s.ad_value(18)), (-1.0)), s.ad_value(33)), A::mul(s.ad_value(100), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30))))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (s.v[1825] != 0.0)) {
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::sub(A::add(s.ad_value(28), s.ad_value(33)), A::mul(s.ad_value(100), s.ad_value(32)))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (s.v[1825] != 0.0)) {
            s.store_sub_ad(13, A::square(s.ad_value(36)), A::scale(A::mul(s.ad_value(37), s.ad_value(13)), 2.0));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (s.v[1825] != 0.0)) {
            s.store_sub_ad(9, A::neg(s.ad_value(18)), A::scale(A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_div_from_scalar_ad(38, 1.0, A::offset(A::scale(s.ad_value(294), 0.7324648775608221), 1.25));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_mul_ad_lhs(39, A::offset(A::mul(A::scale(s.ad_value(95), 1.25), s.ad_value(38)), (-1.0)), 38);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_mul_ad(40, A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_limited_exp_ad(13, A::neg(s.ad_value(40)));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_sub_from_scalar(41, 1.0, 13);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_sub_ad(42, A::add(s.ad_value(214), A::scale(s.ad_value(296), 0.5)), A::mul(s.ad_value(294), A::sqrt(A::sub(A::add(s.ad_value(214), A::scale(s.ad_value(296), 0.25)), s.ad_value(41)))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_offset(43, 98, 3.0);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_sub_ad(12, A::scale(A::sub(A::add(s.ad_value(42), s.ad_value(43)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(42), s.ad_value(43)), A::sub(s.ad_value(42), s.ad_value(43))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(43), A::sqrt(A::offset(A::square(s.ad_value(43)), 5.0))), 0.5));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_sub(13, 214, 12);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_limited_exp_ad(33, A::neg(s.ad_value(12)));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_div_from_scalar_ad(34, 1.0, A::offset(A::square(s.ad_value(12)), 2.0));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_mul_ad_lhs(30, A::square(s.ad_value(12)), 34);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_scale_ad(31, A::mul(A::mul(s.ad_value(12), s.ad_value(34)), s.ad_value(34)), 4.0);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_mul_ad_lhs(32, A::mul(A::sub(A::scale(s.ad_value(34), 8.0), A::scale(s.ad_value(30), 12.0)), s.ad_value(34)), 34);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_max_from_scalar_ad(14, 1e-40, A::sub(A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::sub(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), A::mul(s.ad_value(100), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)))))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_sub_from_scalar_ad(15, 1.0, A::scale(A::mul(s.ad_value(296), A::sub(s.ad_value(33), A::mul(s.ad_value(100), s.ad_value(32)))), 0.5));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_add_ad(16, A::scale(s.ad_value(13), 2.0), A::mul(s.ad_value(296), A::sub(A::sub_from_scalar(1.0, s.ad_value(33)), A::mul(s.ad_value(100), A::offset(s.ad_value(31), 1.0)))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_add_ad(17, A::sub(s.ad_value(98), s.ad_value(12)), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_add(0, 14, 16);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_add_ad(1, A::square(s.ad_value(0)), A::mul(s.ad_value(17), A::sub(A::scale(A::square(s.ad_value(16)), 0.5), A::mul(s.ad_value(14), s.ad_value(15)))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_add_ad_rhs(44, 12, A::div(A::mul(A::mul(s.ad_value(14), s.ad_value(0)), s.ad_value(17)), A::add(s.ad_value(1), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17)), s.ad_value(17)), s.ad_value(16)), A::sub(A::scale(A::square(s.ad_value(16)), 0.3333333333333333), A::mul(s.ad_value(14), s.ad_value(15)))))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_ad(28, &A::limited_exp(s.ad_value(44)));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_div_from_scalar(29, 1.0, 28);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_limited_exp_ad(28, A::sub(s.ad_value(44), s.ad_value(98)));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_div_from_scalar_ad(13, 1.0, A::offset(A::square(s.ad_value(44)), 2.0));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_mul_ad_lhs(30, A::square(s.ad_value(44)), 13);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_scale_ad(31, A::mul(A::mul(s.ad_value(44), s.ad_value(13)), s.ad_value(13)), 4.0);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_mul_ad_lhs(32, A::mul(A::sub(A::scale(s.ad_value(13), 8.0), A::scale(s.ad_value(30), 12.0)), s.ad_value(13)), 13);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_sub(13, 214, 44);
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_add_ad(36, A::scale(s.ad_value(13), 2.0), A::mul(s.ad_value(296), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(29)), s.ad_value(28)), A::mul(s.ad_value(100), A::offset(s.ad_value(31), 1.0)))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_sub_ad(37, A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::sub(A::add(A::offset(A::add(s.ad_value(29), s.ad_value(44)), (-1.0)), s.ad_value(28)), A::mul(s.ad_value(100), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30))))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::sub(A::add(s.ad_value(29), s.ad_value(28)), A::mul(s.ad_value(100), s.ad_value(32)))));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_sub_ad(13, A::square(s.ad_value(36)), A::scale(A::mul(s.ad_value(37), s.ad_value(13)), 2.0));
        }

        if (((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (s.v[1823] != 0.0)) && (!(s.v[1824] != 0.0))) && (!(s.v[1825] != 0.0))) {
            s.store_add_ad_rhs(9, 44, A::scale(A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.copy_ad(47, 2);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.store_square(48, 47);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.store_sub_ad_rhs(8, 4, A::mul(s.ad_value(46), s.ad_value(270)));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.store_sub_ad_rhs(105, 214, A::mul(s.ad_value(294), A::sqrt(A::offset(A::add(A::limited_exp(A::neg(s.ad_value(8))), s.ad_value(8)), (-1.0)))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.store_offset(43, 98, 3.0);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.store_scale_ad(106, A::sub(A::add(s.ad_value(105), s.ad_value(43)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(105), s.ad_value(43)), A::sub(s.ad_value(105), s.ad_value(43))), 40.0))), 0.5);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.store_sub_ad(107, A::sub(A::mul(A::sub(s.ad_value(214), s.ad_value(106)), A::sub(s.ad_value(214), s.ad_value(106))), A::mul(A::mul(s.ad_value(48), A::add(A::sub(s.ad_value(216), s.ad_value(106)), s.ad_value(4))), A::add(A::sub(s.ad_value(216), s.ad_value(106)), s.ad_value(4)))), A::mul(s.ad_value(296), s.ad_value(4)));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.store_sub_ad(108, A::scale(A::sub(s.ad_value(214), s.ad_value(106)), 2.0), A::mul(A::scale(s.ad_value(48), 2.0), A::add(A::sub(s.ad_value(216), s.ad_value(106)), s.ad_value(4))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.store_square(109, 108);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.store_sub_from_scalar(110, 1.0, 48);
        }

        s.v[1826] = if (s.v[107] < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) && (s.v[1826] != 0.0)) {
            s.store_scalar(107, 0.0);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.store_add_ad(49, A::sub(s.ad_value(98), s.ad_value(106)), A::ln(A::max_with_scalar(A::mul(s.ad_value(107), s.ad_value(297)), 1e-38)));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.store_add(111, 107, 108);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.store_square(112, 111);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.store_sub_ad(113, A::add(A::div(s.ad_value(112), s.ad_value(49)), A::scale(s.ad_value(109), 0.5)), A::mul(s.ad_value(107), s.ad_value(110)));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.store_div_ad_lhs(114, A::mul(s.ad_value(108), s.ad_value(111)), 113);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.store_sub_ad(115, A::scale(s.ad_value(109), 0.3333333333333333), A::mul(s.ad_value(107), s.ad_value(110)));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.store_div_ad(116, A::mul(s.ad_value(111), s.ad_value(107)), A::add(s.ad_value(113), A::mul(s.ad_value(114), s.ad_value(115))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.store_add(117, 106, 116);
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.store_limited_exp_ad(118, A::sub(s.ad_value(117), s.ad_value(98)));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.store_add_ad(119, A::sub(A::scale(A::sub(s.ad_value(214), s.ad_value(117)), 2.0), A::mul(A::scale(s.ad_value(48), 2.0), A::add(A::sub(s.ad_value(216), s.ad_value(117)), s.ad_value(4)))), A::mul(s.ad_value(296), s.ad_value(118)));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.store_sub_ad(120, A::sub(A::mul(A::sub(s.ad_value(214), s.ad_value(117)), A::sub(s.ad_value(214), s.ad_value(117))), A::mul(A::mul(s.ad_value(48), A::add(A::sub(s.ad_value(216), s.ad_value(117)), s.ad_value(4))), A::add(A::sub(s.ad_value(216), s.ad_value(117)), s.ad_value(4)))), A::mul(s.ad_value(296), A::add(s.ad_value(4), s.ad_value(118))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.store_mul_ad(121, A::scale(s.ad_value(120), 2.0), A::sub(A::sub_from_scalar(2.0, A::scale(s.ad_value(48), 2.0)), A::mul(s.ad_value(296), s.ad_value(118))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.store_div_ad(122, A::scale(s.ad_value(120), 2.0), A::add(s.ad_value(119), A::sqrt(A::sub(A::square(s.ad_value(119)), s.ad_value(121)))));
        }

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1822] != 0.0)) && (!(s.v[1823] != 0.0))) {
            s.store_add(9, 117, 122);
        }

        s.v[1827] = if (((s.v[214]) as f64).abs() <= s.v[101]) { 1.0 } else { 0.0 };

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (s.v[1827] != 0.0)) {
            s.store_scale_ad(167, A::square(s.ad_value(96)), (0.16666666666666666 * 0.7071067811865475));
        }

    }

    pub(super) fn stamp_reactive_block_29(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (s.v[1827] != 0.0)) {
            s.store_mul_ad(9, A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(100))), s.ad_value(294)), s.ad_value(167)), 1.0));
        }

        s.v[1828] = if (s.v[214] < (-s.v[101])) { 1.0 } else { 0.0 };

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_neg(10, 214);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_scaled_mul(11, 10, 96, 1.25);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_scale_ad(12, A::sub(A::offset(s.ad_value(11), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(11), (-6.0)), A::offset(s.ad_value(11), (-6.0))), 64.0))), 0.5);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_sub(13, 10, 12);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_add_ad(14, A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::offset(s.ad_value(12), 1.0)));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_sub_ad_lhs(16, A::scale(s.ad_value(13), 2.0), 296);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_sub_ad_lhs(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_add(0, 14, 16);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_add_ad(1, A::square(s.ad_value(0)), A::mul(s.ad_value(17), A::sub(A::scale(A::square(s.ad_value(16)), 0.5), s.ad_value(14))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_add_ad_rhs(18, 12, A::div(A::mul(A::mul(s.ad_value(14), s.ad_value(0)), s.ad_value(17)), A::add(s.ad_value(1), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17)), s.ad_value(17)), s.ad_value(16)), A::sub(A::scale(A::square(s.ad_value(16)), 0.3333333333333333), s.ad_value(14))))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_ad(28, &A::limited_exp(s.ad_value(18)));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_div_from_scalar(29, 1.0, 28);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_div_from_scalar_ad(13, 1.0, A::offset(A::square(s.ad_value(18)), 2.0));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_mul_ad_lhs(30, A::square(s.ad_value(18)), 13);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_scale_ad(31, A::mul(A::mul(s.ad_value(18), s.ad_value(13)), s.ad_value(13)), 4.0);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_mul_ad_lhs(32, A::mul(A::sub(A::scale(s.ad_value(13), 8.0), A::scale(s.ad_value(30), 12.0)), s.ad_value(13)), 13);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_sub(13, 10, 18);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_mul(33, 100, 29);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_add_ad(36, A::scale(s.ad_value(13), 2.0), A::mul(s.ad_value(296), A::add(A::sub(A::offset(s.ad_value(28), (-1.0)), s.ad_value(33)), A::mul(s.ad_value(100), A::sub_from_scalar(1.0, s.ad_value(31))))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_sub_ad(37, A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::add(A::add(A::offset(A::sub(s.ad_value(28), s.ad_value(18)), (-1.0)), s.ad_value(33)), A::mul(s.ad_value(100), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30))))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::sub(A::add(s.ad_value(28), s.ad_value(33)), A::mul(s.ad_value(100), s.ad_value(32)))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_sub_ad(13, A::square(s.ad_value(36)), A::scale(A::mul(s.ad_value(37), s.ad_value(13)), 2.0));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (s.v[1828] != 0.0)) {
            s.store_sub_ad(9, A::neg(s.ad_value(18)), A::scale(A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_div_from_scalar_ad(38, 1.0, A::offset(A::scale(s.ad_value(294), 0.7324648775608221), 1.25));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_mul_ad_lhs(39, A::offset(A::mul(A::scale(s.ad_value(95), 1.25), s.ad_value(38)), (-1.0)), 38);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_mul_ad(40, A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_limited_exp_ad(13, A::neg(s.ad_value(40)));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_sub_from_scalar(41, 1.0, 13);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_sub_ad(42, A::add(s.ad_value(214), A::scale(s.ad_value(296), 0.5)), A::mul(s.ad_value(294), A::sqrt(A::sub(A::add(s.ad_value(214), A::scale(s.ad_value(296), 0.25)), s.ad_value(41)))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_offset(43, 98, 3.0);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_sub_ad(12, A::scale(A::sub(A::add(s.ad_value(42), s.ad_value(43)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(42), s.ad_value(43)), A::sub(s.ad_value(42), s.ad_value(43))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(43), A::sqrt(A::offset(A::square(s.ad_value(43)), 5.0))), 0.5));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_sub(13, 214, 12);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_limited_exp_ad(33, A::neg(s.ad_value(12)));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_div_from_scalar_ad(34, 1.0, A::offset(A::square(s.ad_value(12)), 2.0));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_mul_ad_lhs(30, A::square(s.ad_value(12)), 34);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_scale_ad(31, A::mul(A::mul(s.ad_value(12), s.ad_value(34)), s.ad_value(34)), 4.0);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_mul_ad_lhs(32, A::mul(A::sub(A::scale(s.ad_value(34), 8.0), A::scale(s.ad_value(30), 12.0)), s.ad_value(34)), 34);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_max_from_scalar_ad(14, 1e-40, A::sub(A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::sub(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), A::mul(s.ad_value(100), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)))))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_sub_from_scalar_ad(15, 1.0, A::scale(A::mul(s.ad_value(296), A::sub(s.ad_value(33), A::mul(s.ad_value(100), s.ad_value(32)))), 0.5));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_add_ad(16, A::scale(s.ad_value(13), 2.0), A::mul(s.ad_value(296), A::sub(A::sub_from_scalar(1.0, s.ad_value(33)), A::mul(s.ad_value(100), A::offset(s.ad_value(31), 1.0)))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_add_ad(17, A::sub(s.ad_value(98), s.ad_value(12)), A::ln(A::max_with_scalar(A::div(s.ad_value(14), s.ad_value(296)), 1e-38)));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_add(0, 14, 16);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_add_ad(1, A::square(s.ad_value(0)), A::mul(s.ad_value(17), A::sub(A::scale(A::square(s.ad_value(16)), 0.5), A::mul(s.ad_value(14), s.ad_value(15)))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_add_ad_rhs(44, 12, A::div(A::mul(A::mul(s.ad_value(14), s.ad_value(0)), s.ad_value(17)), A::add(s.ad_value(1), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17)), s.ad_value(17)), s.ad_value(16)), A::sub(A::scale(A::square(s.ad_value(16)), 0.3333333333333333), A::mul(s.ad_value(14), s.ad_value(15)))))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_ad(28, &A::limited_exp(s.ad_value(44)));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_div_from_scalar(29, 1.0, 28);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_limited_exp_ad(28, A::sub(s.ad_value(44), s.ad_value(98)));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_div_from_scalar_ad(13, 1.0, A::offset(A::square(s.ad_value(44)), 2.0));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_mul_ad_lhs(30, A::square(s.ad_value(44)), 13);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_scale_ad(31, A::mul(A::mul(s.ad_value(44), s.ad_value(13)), s.ad_value(13)), 4.0);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_mul_ad_lhs(32, A::mul(A::sub(A::scale(s.ad_value(13), 8.0), A::scale(s.ad_value(30), 12.0)), s.ad_value(13)), 13);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_sub(13, 214, 44);
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_add_ad(36, A::scale(s.ad_value(13), 2.0), A::mul(s.ad_value(296), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(29)), s.ad_value(28)), A::mul(s.ad_value(100), A::offset(s.ad_value(31), 1.0)))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_sub_ad(37, A::square(s.ad_value(13)), A::mul(s.ad_value(296), A::sub(A::add(A::offset(A::add(s.ad_value(29), s.ad_value(44)), (-1.0)), s.ad_value(28)), A::mul(s.ad_value(100), A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(30))))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_sub_from_scalar_ad(13, 2.0, A::mul(s.ad_value(296), A::sub(A::add(s.ad_value(29), s.ad_value(28)), A::mul(s.ad_value(100), s.ad_value(32)))));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_sub_ad(13, A::square(s.ad_value(36)), A::scale(A::mul(s.ad_value(37), s.ad_value(13)), 2.0));
        }

        if ((((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1822] != 0.0))) && (!(s.v[1827] != 0.0))) && (!(s.v[1828] != 0.0))) {
            s.store_add_ad_rhs(9, 44, A::scale(A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.copy_ad(123, 9);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_scalar(102, 1e-7);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_scalar(103, 2.0);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_scale_ad(35, A::square(s.ad_value(96)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_offset_ad(24, A::scale(A::mul(A::offset(A::scale(s.ad_value(743), 1.0 / (s.v[184])), 1.0), A::scale(s.ad_value(706), (1.602176462e-19 * (p.p74 * p.p74)))), 1.0 / ((2.0 * s.v[180]))), p.p294);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_div_ad_lhs(167, A::scale(A::neg(s.ad_value(726)), s.v[184]), 300);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_mul_ad(168, A::mul(s.ad_value(725), A::add(A::limited_exp(A::scale(s.ad_value(167), 0.5)), A::scale(A::limited_exp(s.ad_value(167)), 2.0))), A::add(A::mul(s.ad_value(226), s.ad_value(270)), A::mul(s.ad_value(724), s.ad_value(270))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_add_ad(6, A::add(A::sub(s.ad_value(24), A::mul(A::mul(s.ad_value(3), A::mul(s.ad_value(216), s.ad_value(269))), s.ad_value(727))), A::mul(A::mul(A::offset(s.ad_value(3), 1.0), s.ad_value(168)), s.ad_value(269))), A::mul(A::offset(s.ad_value(3), 1.0), s.ad_value(46)));
        }

        s.v[1829] = if (((s.v[214]) as f64).abs() <= s.v[102]) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1829] != 0.0)) {
            s.store_scale_ad(167, A::square(s.ad_value(96)), (0.16666666666666666 * 0.7071067811865475));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1829] != 0.0)) {
            s.store_mul_ad(23, A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(100))), s.ad_value(294)), s.ad_value(167)), 1.0));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1829] != 0.0))) {
            s.store_scale_ad(167, A::add(A::tanh(A::scale(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0))), A::tanh(A::scale(A::add(s.ad_value(214), s.ad_value(103)), 5.0))), 0.5);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1829] != 0.0))) {
            s.store_mul_ad_lhs(45, A::mul(A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, A::limited_exp(A::neg(s.ad_value(98))))), s.ad_value(294)), s.ad_value(35)), 1.0)), 167);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1829] != 0.0))) {
            s.store_limited_exp_ad(167, A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1829] != 0.0))) {
            s.store_limited_exp_ad(168, A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1829] != 0.0))) {
            let assign43980_ad_e73575: A = {
                if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))), 1.0))
                } else {
                    let assign43980_ad_e73574: A = {
                        if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)))
                        } else {
                            {
                                if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign43980_ad_e73574
                }
            };
            let assign43980_ad_e73657: A = {
                if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))), 1.0))
                } else {
                    let assign43980_ad_e73656: A = {
                        if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)))
                        } else {
                            {
                                if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign43980_ad_e73656
                }
            };
            s.store_sub_ad(169, assign43980_ad_e73575, assign43980_ad_e73657);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1829] != 0.0))) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div(s.ad_value(169), A::offset(s.ad_value(3), 1.0))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1829] != 0.0))) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1829] != 0.0))) {
            s.store_limited_exp_ad(173, A::neg(s.ad_value(123)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1829] != 0.0))) {
            s.store_square(174, 123);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1829] != 0.0))) {
            s.store_div_from_scalar_ad(175, 1.0, A::offset(s.ad_value(174), 2.0));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1829] != 0.0))) {
            s.store_limited_exp_ad(176, A::neg(s.ad_value(98)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1829] != 0.0))) {
            s.store_limited_exp_ad(177, A::sub(s.ad_value(123), s.ad_value(98)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1829] != 0.0))) {
            let assign44060_ad_e73817: A = A::sub(A::sub(A::mul(A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123))), A::mul(A::mul(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170))), A::add(s.ad_value(216), s.ad_value(170)))), A::mul(s.ad_value(296), A::sub(A::add(A::add(A::add(A::sub(s.ad_value(173), s.ad_value(171)), s.ad_value(123)), s.ad_value(170)), s.ad_value(177)), A::mul(s.ad_value(176), A::add(A::offset(s.ad_value(123), 1.0), A::mul(s.ad_value(175), s.ad_value(174)))))));
            s.store_ad(19, &assign44060_ad_e73817);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1829] != 0.0))) {
            let assign44070_ad_e73892: A = A::sub(A::sub(A::add(s.ad_value(177), A::mul(s.ad_value(176), A::offset(A::add(A::mul(A::scale(s.ad_value(123), (-2.0)), s.ad_value(175)), A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(123), 2.0), s.ad_value(123)), s.ad_value(123)), s.ad_value(175)), s.ad_value(175))), (-1.0)))), s.ad_value(173)), A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))));
            let assign44070_ad_e73908: A = A::sub(A::add(A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170))), s.ad_value(2)), s.ad_value(2)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::scale(s.ad_value(214), 2.0)), A::scale(s.ad_value(123), 2.0)), A::mul(s.ad_value(296), A::offset(A::add(assign44070_ad_e73892, A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)))), 1.0)));
            s.store_ad(20, &assign44070_ad_e73908);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1829] != 0.0))) {
            let assign44080_ad_e73964: A = A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::mul(A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0))));
            let assign44080_ad_e74023: A = A::mul(A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::add(A::sub(A::sub_from_scalar(1.0, A::div(s.ad_value(167), A::offset(s.ad_value(167), 1.0))), s.ad_value(171)), A::mul(A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::offset(s.ad_value(167), 1.0)), A::offset(A::div_from_scalar(1.0, A::offset(s.ad_value(3), 1.0)), 1.0))));
            let assign44080_ad_e74052: A = A::sub(A::sub(assign44080_ad_e73964, A::mul(s.ad_value(296), A::sub(A::sub(A::add(s.ad_value(173), s.ad_value(177)), A::mul(A::mul(A::scale(s.ad_value(176), 2.0), s.ad_value(175)), A::sub_from_scalar(1.0, A::mul(A::mul(s.ad_value(174), s.ad_value(175)), A::sub_from_scalar(5.0, A::mul(A::scale(s.ad_value(174), 4.0), s.ad_value(175))))))), assign44080_ad_e74023))), A::div(A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(167), 1.0))));
            s.store_offset_ad(21, assign44080_ad_e74052, 2.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1829] != 0.0))) {
            s.store_sub_ad_rhs(23, 123, A::mul(A::div(s.ad_value(19), s.ad_value(20)), A::offset(A::div(A::mul(s.ad_value(19), s.ad_value(21)), A::mul(A::scale(s.ad_value(20), 2.0), s.ad_value(20))), 1.0)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.copy_ad(123, 23);
        }

        s.v[1830] = if (((s.v[214]) as f64).abs() <= s.v[102]) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1830] != 0.0)) {
            s.store_scale_ad(167, A::square(s.ad_value(96)), (0.16666666666666666 * 0.7071067811865475));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1830] != 0.0)) {
            s.store_mul_ad(23, A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(100))), s.ad_value(294)), s.ad_value(167)), 1.0));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1830] != 0.0))) {
            s.store_scale_ad(167, A::add(A::tanh(A::scale(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0))), A::tanh(A::scale(A::add(s.ad_value(214), s.ad_value(103)), 5.0))), 0.5);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1830] != 0.0))) {
            s.store_mul_ad_lhs(45, A::mul(A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, A::limited_exp(A::neg(s.ad_value(98))))), s.ad_value(294)), s.ad_value(35)), 1.0)), 167);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1830] != 0.0))) {
            s.store_limited_exp_ad(167, A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1830] != 0.0))) {
            s.store_limited_exp_ad(168, A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1830] != 0.0))) {
            let assign44180_ad_e74325: A = {
                if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))), 1.0))
                } else {
                    let assign44180_ad_e74324: A = {
                        if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)))
                        } else {
                            {
                                if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign44180_ad_e74324
                }
            };
            let assign44180_ad_e74407: A = {
                if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))), 1.0))
                } else {
                    let assign44180_ad_e74406: A = {
                        if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)))
                        } else {
                            {
                                if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    };
                    assign44180_ad_e74406
                }
            };
            s.store_sub_ad(169, assign44180_ad_e74325, assign44180_ad_e74407);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1830] != 0.0))) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div(s.ad_value(169), A::offset(s.ad_value(3), 1.0))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1830] != 0.0))) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1830] != 0.0))) {
            s.store_limited_exp_ad(173, A::neg(s.ad_value(123)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1830] != 0.0))) {
            s.store_square(174, 123);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1830] != 0.0))) {
            s.store_div_from_scalar_ad(175, 1.0, A::offset(s.ad_value(174), 2.0));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1830] != 0.0))) {
            s.store_limited_exp_ad(176, A::neg(s.ad_value(98)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1830] != 0.0))) {
            s.store_limited_exp_ad(177, A::sub(s.ad_value(123), s.ad_value(98)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1830] != 0.0))) {
            let assign44260_ad_e74567: A = A::sub(A::sub(A::mul(A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123))), A::mul(A::mul(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170))), A::add(s.ad_value(216), s.ad_value(170)))), A::mul(s.ad_value(296), A::sub(A::add(A::add(A::add(A::sub(s.ad_value(173), s.ad_value(171)), s.ad_value(123)), s.ad_value(170)), s.ad_value(177)), A::mul(s.ad_value(176), A::add(A::offset(s.ad_value(123), 1.0), A::mul(s.ad_value(175), s.ad_value(174)))))));
            s.store_ad(19, &assign44260_ad_e74567);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1830] != 0.0))) {
            let assign44270_ad_e74642: A = A::sub(A::sub(A::add(s.ad_value(177), A::mul(s.ad_value(176), A::offset(A::add(A::mul(A::scale(s.ad_value(123), (-2.0)), s.ad_value(175)), A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(123), 2.0), s.ad_value(123)), s.ad_value(123)), s.ad_value(175)), s.ad_value(175))), (-1.0)))), s.ad_value(173)), A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))));
            let assign44270_ad_e74658: A = A::sub(A::add(A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170))), s.ad_value(2)), s.ad_value(2)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::scale(s.ad_value(214), 2.0)), A::scale(s.ad_value(123), 2.0)), A::mul(s.ad_value(296), A::offset(A::add(assign44270_ad_e74642, A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)))), 1.0)));
            s.store_ad(20, &assign44270_ad_e74658);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1830] != 0.0))) {
            let assign44280_ad_e74714: A = A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::mul(A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0))));
            let assign44280_ad_e74773: A = A::mul(A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::add(A::sub(A::sub_from_scalar(1.0, A::div(s.ad_value(167), A::offset(s.ad_value(167), 1.0))), s.ad_value(171)), A::mul(A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::offset(s.ad_value(167), 1.0)), A::offset(A::div_from_scalar(1.0, A::offset(s.ad_value(3), 1.0)), 1.0))));
            let assign44280_ad_e74802: A = A::sub(A::sub(assign44280_ad_e74714, A::mul(s.ad_value(296), A::sub(A::sub(A::add(s.ad_value(173), s.ad_value(177)), A::mul(A::mul(A::scale(s.ad_value(176), 2.0), s.ad_value(175)), A::sub_from_scalar(1.0, A::mul(A::mul(s.ad_value(174), s.ad_value(175)), A::sub_from_scalar(5.0, A::mul(A::scale(s.ad_value(174), 4.0), s.ad_value(175))))))), assign44280_ad_e74773))), A::div(A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(167), 1.0))));
            s.store_offset_ad(21, assign44280_ad_e74802, 2.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1830] != 0.0))) {
            s.store_sub_ad_rhs(23, 123, A::mul(A::div(s.ad_value(19), s.ad_value(20)), A::offset(A::div(A::mul(s.ad_value(19), s.ad_value(21)), A::mul(A::scale(s.ad_value(20), 2.0), s.ad_value(20))), 1.0)));
        }

    }

    pub(super) fn stamp_reactive_block_30(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.copy_ad(123, 23);
        }

        s.v[1831] = if (((s.v[214]) as f64).abs() <= s.v[102]) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1831] != 0.0)) {
            s.store_scale_ad(167, A::square(s.ad_value(96)), (0.16666666666666666 * 0.7071067811865475));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1831] != 0.0)) {
            s.store_mul_ad(23, A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, s.ad_value(100))), s.ad_value(294)), s.ad_value(167)), 1.0));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1831] != 0.0))) {
            s.store_scale_ad(167, A::add(A::tanh(A::scale(A::sub(s.ad_value(214), s.ad_value(103)), (-5.0))), A::tanh(A::scale(A::add(s.ad_value(214), s.ad_value(103)), 5.0))), 0.5);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1831] != 0.0))) {
            s.store_mul_ad_lhs(45, A::mul(A::mul(s.ad_value(214), s.ad_value(96)), A::offset(A::mul(A::mul(A::mul(s.ad_value(214), A::sub_from_scalar(1.0, A::limited_exp(A::neg(s.ad_value(98))))), s.ad_value(294)), s.ad_value(35)), 1.0)), 167);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1831] != 0.0))) {
            s.store_limited_exp_ad(167, A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1831] != 0.0))) {
            s.store_limited_exp_ad(168, A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1831] != 0.0))) {
            let assign44380_ad_e75075: A = {
                if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))), 1.0))
                } else {
                    let assign44380_ad_e75074: A = {
                        if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)))
                        } else {
                            {
                                if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))
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
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))), 1.0))
                } else {
                    let assign44380_ad_e75156: A = {
                        if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)))
                        } else {
                            {
                                if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))
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

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1831] != 0.0))) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div(s.ad_value(169), A::offset(s.ad_value(3), 1.0))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1831] != 0.0))) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1831] != 0.0))) {
            s.store_limited_exp_ad(173, A::neg(s.ad_value(123)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1831] != 0.0))) {
            s.store_square(174, 123);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1831] != 0.0))) {
            s.store_div_from_scalar_ad(175, 1.0, A::offset(s.ad_value(174), 2.0));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1831] != 0.0))) {
            s.store_limited_exp_ad(176, A::neg(s.ad_value(98)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1831] != 0.0))) {
            s.store_limited_exp_ad(177, A::sub(s.ad_value(123), s.ad_value(98)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1831] != 0.0))) {
            let assign44460_ad_e75317: A = A::sub(A::sub(A::mul(A::sub(s.ad_value(214), s.ad_value(123)), A::sub(s.ad_value(214), s.ad_value(123))), A::mul(A::mul(A::square(s.ad_value(2)), A::add(s.ad_value(216), s.ad_value(170))), A::add(s.ad_value(216), s.ad_value(170)))), A::mul(s.ad_value(296), A::sub(A::add(A::add(A::add(A::sub(s.ad_value(173), s.ad_value(171)), s.ad_value(123)), s.ad_value(170)), s.ad_value(177)), A::mul(s.ad_value(176), A::add(A::offset(s.ad_value(123), 1.0), A::mul(s.ad_value(175), s.ad_value(174)))))));
            s.store_ad(19, &assign44460_ad_e75317);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1831] != 0.0))) {
            let assign44470_ad_e75392: A = A::sub(A::sub(A::add(s.ad_value(177), A::mul(s.ad_value(176), A::offset(A::add(A::mul(A::scale(s.ad_value(123), (-2.0)), s.ad_value(175)), A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(123), 2.0), s.ad_value(123)), s.ad_value(123)), s.ad_value(175)), s.ad_value(175))), (-1.0)))), s.ad_value(173)), A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))));
            let assign44470_ad_e75408: A = A::sub(A::add(A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170))), s.ad_value(2)), s.ad_value(2)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::scale(s.ad_value(214), 2.0)), A::scale(s.ad_value(123), 2.0)), A::mul(s.ad_value(296), A::offset(A::add(assign44470_ad_e75392, A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)))), 1.0)));
            s.store_ad(20, &assign44470_ad_e75408);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1831] != 0.0))) {
            let assign44480_ad_e75464: A = A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::mul(A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(3), 1.0)), A::offset(s.ad_value(167), 1.0))));
            let assign44480_ad_e75523: A = A::mul(A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::add(A::sub(A::sub_from_scalar(1.0, A::div(s.ad_value(167), A::offset(s.ad_value(167), 1.0))), s.ad_value(171)), A::mul(A::div(A::mul(s.ad_value(167), s.ad_value(171)), A::offset(s.ad_value(167), 1.0)), A::offset(A::div_from_scalar(1.0, A::offset(s.ad_value(3), 1.0)), 1.0))));
            let assign44480_ad_e75552: A = A::sub(A::sub(assign44480_ad_e75464, A::mul(s.ad_value(296), A::sub(A::sub(A::add(s.ad_value(173), s.ad_value(177)), A::mul(A::mul(A::scale(s.ad_value(176), 2.0), s.ad_value(175)), A::sub_from_scalar(1.0, A::mul(A::mul(s.ad_value(174), s.ad_value(175)), A::sub_from_scalar(5.0, A::mul(A::scale(s.ad_value(174), 4.0), s.ad_value(175))))))), assign44480_ad_e75523))), A::div(A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)), A::offset(s.ad_value(167), 1.0))));
            s.store_offset_ad(21, assign44480_ad_e75552, 2.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (!(s.v[1831] != 0.0))) {
            s.store_sub_ad_rhs(23, 123, A::mul(A::div(s.ad_value(19), s.ad_value(20)), A::offset(A::div(A::mul(s.ad_value(19), s.ad_value(21)), A::mul(A::scale(s.ad_value(20), 2.0), s.ad_value(20))), 1.0)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_sub(62, 23, 22);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_mul(63, 226, 270);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_limited_exp_ad(64, A::neg(s.ad_value(63)));
        }

        s.v[1832] = if (s.v[62] < 1e-10) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1832] != 0.0)) {
            s.store_limited_exp_ad(167, A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1832] != 0.0)) {
            s.store_limited_exp_ad(168, A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1832] != 0.0)) {
            let assign44560_ad_e75746: A = {
                if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && (!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))), 1.0))
                } else {
                    let assign44560_ad_e75745: A = {
                        if ((!((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)))
                        } else {
                            {
                                if ((((s.v[123] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))
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
                    A::ln(A::offset(A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))), 1.0))
                } else {
                    let assign44560_ad_e75827: A = {
                        if ((!((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0)) && ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) < (-37.0))) {
                            A::exp(A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269)))
                        } else {
                            {
                                if ((((s.v[45] * s.v[269]) - s.v[6]) / s.v[269]) > 37.0) {
                                    A::div(A::sub(A::mul(s.ad_value(45), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))
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

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1832] != 0.0)) {
            s.store_neg_ad(170, A::add(A::div(s.ad_value(46), s.ad_value(269)), A::div(s.ad_value(169), A::offset(s.ad_value(3), 1.0))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1832] != 0.0)) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1832] != 0.0)) {
            s.store_limited_exp_ad(172, A::add(s.ad_value(170), A::div(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), s.ad_value(269))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1832] != 0.0)) {
            s.store_limited_exp_ad(173, A::neg(s.ad_value(123)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1832] != 0.0)) {
            s.store_square(174, 123);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1832] != 0.0)) {
            s.store_div_from_scalar_ad(175, 1.0, A::offset(s.ad_value(174), 2.0));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1832] != 0.0)) {
            s.store_limited_exp_ad(177, A::sub(s.ad_value(123), s.ad_value(98)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1832] != 0.0)) {
            s.store_limited_exp_ad(178, A::div(A::scale(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), 2.0), s.ad_value(269)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1832] != 0.0)) {
            s.store_limited_exp_ad(179, A::add(A::div(A::scale(A::sub(A::mul(s.ad_value(123), s.ad_value(269)), s.ad_value(6)), 2.0), s.ad_value(269)), s.ad_value(170)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1832] != 0.0)) {
            let assign44660_ad_e76049: A = A::add(A::limited_exp(A::sub(A::sub(s.ad_value(123), s.ad_value(63)), s.ad_value(98))), A::mul(A::limited_exp(A::sub(A::neg(s.ad_value(63)), s.ad_value(98))), A::offset(A::add(A::mul(A::scale(s.ad_value(123), (-2.0)), s.ad_value(175)), A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(123), 2.0), s.ad_value(123)), s.ad_value(123)), s.ad_value(175)), s.ad_value(175))), (-1.0))));
            let assign44660_ad_e76075: A = A::sub(A::add(A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(167), 2.0), A::add(s.ad_value(216), s.ad_value(170))), s.ad_value(2)), s.ad_value(2)), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::scale(s.ad_value(214), 2.0)), A::scale(s.ad_value(123), 2.0)), A::mul(s.ad_value(296), A::offset(A::add(A::sub(A::sub(assign44660_ad_e76049, s.ad_value(173)), A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)))), A::div(s.ad_value(172), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)))), 1.0)));
            s.store_neg_ad(65, assign44660_ad_e76075);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1832] != 0.0)) {
            s.store_mul_ad_lhs(66, A::mul(s.ad_value(296), A::sub_from_scalar(1.0, s.ad_value(64))), 57);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1832] != 0.0)) {
            let assign44680_ad_e76146: A = A::sub(A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(167)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0))), A::div(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(178)), A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(3), 1.0)), A::add(A::offset(A::scale(s.ad_value(167), 2.0), 1.0), s.ad_value(178)))));
            let assign44680_ad_e76191: A = A::mul(A::limited_exp(A::sub(A::neg(s.ad_value(98)), s.ad_value(63))), A::sub(A::add(A::scale(s.ad_value(175), (-2.0)), A::mul(A::mul(A::mul(A::scale(s.ad_value(123), 10.0), s.ad_value(123)), s.ad_value(175)), s.ad_value(175))), A::mul(A::mul(A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(123), 8.0), s.ad_value(123)), s.ad_value(123)), s.ad_value(123)), s.ad_value(175)), s.ad_value(175)), s.ad_value(175))));
            let assign44680_ad_e76216: A = A::add(A::sub(A::add(A::add(s.ad_value(173), A::limited_exp(A::sub(A::sub(s.ad_value(123), s.ad_value(98)), s.ad_value(63)))), assign44680_ad_e76191), A::div(s.ad_value(167), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)))), A::div(s.ad_value(178), A::mul(A::offset(s.ad_value(3), 1.0), A::add(A::offset(A::scale(s.ad_value(167), 2.0), 1.0), s.ad_value(178)))));
            let assign44680_ad_e76258: A = A::sub(A::sub(A::add(assign44680_ad_e76216, A::div(s.ad_value(172), A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(167), 1.0)))), A::div(s.ad_value(179), A::mul(A::offset(s.ad_value(3), 1.0), A::add(A::offset(A::scale(s.ad_value(167), 2.0), 1.0), s.ad_value(178))))), A::div(s.ad_value(179), A::mul(A::mul(A::offset(s.ad_value(3), 1.0), A::offset(s.ad_value(3), 1.0)), A::add(A::offset(A::scale(s.ad_value(167), 2.0), 1.0), s.ad_value(178)))));
            s.store_offset_ad(54, A::sub(A::sub(assign44680_ad_e76146, A::mul(s.ad_value(296), assign44680_ad_e76258)), A::div(A::mul(A::mul(A::mul(A::scale(s.ad_value(2), 2.0), s.ad_value(2)), s.ad_value(178)), A::add(s.ad_value(216), s.ad_value(170))), A::mul(A::offset(s.ad_value(3), 1.0), A::add(A::offset(A::scale(s.ad_value(167), 2.0), 1.0), s.ad_value(178))))), 2.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1832] != 0.0)) {
            s.store_sub_ad(54, A::square(s.ad_value(65)), A::scale(A::mul(s.ad_value(54), s.ad_value(66)), 2.0));
        }

        s.v[1833] = if (s.v[54] >= 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1832] != 0.0)) && (s.v[1833] != 0.0)) {
            s.store_scale_ad(62, A::div(s.ad_value(66), A::add(s.ad_value(65), A::sqrt(s.ad_value(54)))), 2.0);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1832] != 0.0)) {
            s.store_add(23, 22, 62);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_mul(250, 62, 269);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_div_ad(67, A::square(s.ad_value(23)), A::offset(A::square(s.ad_value(23)), 2.0));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_limited_exp_ad(68, A::neg(s.ad_value(23)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_sub_ad(69, A::limited_exp(A::sub(s.ad_value(23), s.ad_value(98))), A::mul(A::limited_exp(A::neg(s.ad_value(98))), A::add(A::offset(s.ad_value(23), 1.0), s.ad_value(67))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_sub_ad_lhs(70, A::mul(A::mul(A::sub(s.ad_value(214), s.ad_value(23)), A::sub(s.ad_value(214), s.ad_value(23))), A::div_from_scalar(1.0, s.ad_value(296))), 69);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_offset_ad(70, A::scale(A::add(A::offset(s.ad_value(70), (-0.001)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(70), (-0.001)), A::offset(s.ad_value(70), (-0.001))), ((4.0 * 1e-5) * 1e-5)))), 0.5), 0.001);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_sqrt(60, 70);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_mul_ad_rhs(72, 294, A::sqrt(A::add(s.ad_value(70), s.ad_value(69))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_div_ad(73, A::mul(A::mul(s.ad_value(296), s.ad_value(69)), s.ad_value(269)), A::add(s.ad_value(72), A::mul(s.ad_value(294), s.ad_value(60))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_scaled_add(75, 22, 23, 0.5);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_abs_ad(54, A::mul(s.ad_value(68), s.ad_value(56)));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_sqrt(76, 54);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_scaled_add(77, 57, 69, 0.5);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_add_ad_rhs(78, 77, A::scale(A::mul(A::square(s.ad_value(62)), A::sub(s.ad_value(76), A::scale(s.ad_value(297), 2.0))), 0.125));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_sub_ad_lhs(79, A::mul(A::mul(A::sub(s.ad_value(214), s.ad_value(75)), A::sub(s.ad_value(214), s.ad_value(75))), A::div_from_scalar(1.0, s.ad_value(296))), 78);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_mul_ad_rhs(51, 294, A::sqrt(A::add(s.ad_value(78), s.ad_value(79))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_offset_ad(79, A::scale(A::add(A::offset(s.ad_value(79), (-0.001)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(79), (-0.001)), A::offset(s.ad_value(79), (-0.001))), ((4.0 * 1e-5) * 1e-5)))), 0.5), 0.001);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_sqrt(71, 79);
        }

        s.v[1834] = if (p.p46 == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1834] != 0.0)) {
            s.store_div_ad(85, A::scale(s.ad_value(269), ((2.0 * s.v[199]) * s.v[199])), A::scale(s.ad_value(704), (1.602176462e-19 * s.v[180])));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1834] != 0.0)) {
            s.store_add_ad(86, A::sub_from_scalar(1.0, s.ad_value(76)), A::scale(A::mul(s.ad_value(51), A::div_from_scalar(1.0, s.ad_value(296))), 2.0));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1834] != 0.0)) {
            s.store_div_from_scalar_ad(87, 1.0, A::sqrt(A::offset(A::mul(s.ad_value(85), s.ad_value(51)), 1.0)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1834] != 0.0)) {
            s.store_div_ad_rhs(54, 87, A::offset(s.ad_value(87), 1.0));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1834] != 0.0)) {
            s.store_mul_ad(88, A::mul(s.ad_value(85), A::mul(A::mul(A::square(s.ad_value(54)), s.ad_value(51)), s.ad_value(51))), A::div(s.ad_value(78), A::add(s.ad_value(78), s.ad_value(79))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1834] != 0.0)) {
            s.store_add_ad(89, A::scale(A::sub(s.ad_value(51), s.ad_value(88)), 2.0), A::mul(s.ad_value(296), A::add(A::sub_from_scalar(1.0, s.ad_value(76)), s.ad_value(78))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1834] != 0.0)) {
            s.store_mul_ad_rhs(90, 88, A::sub(s.ad_value(88), A::scale(s.ad_value(51), 2.0)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1834] != 0.0)) {
            s.store_sub_from_scalar_ad(91, 1.0, A::scale(A::mul(s.ad_value(296), A::add(s.ad_value(76), s.ad_value(78))), 0.5));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1834] != 0.0)) {
            s.store_div_ad(92, A::mul(s.ad_value(90), s.ad_value(89)), A::sub(A::square(s.ad_value(89)), A::mul(s.ad_value(91), s.ad_value(90))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1834] != 0.0)) {
            s.store_add(75, 75, 92);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1834] != 0.0)) {
            s.store_ad(93, &A::limited_exp(s.ad_value(92)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1834] != 0.0)) {
            s.store_div(76, 76, 93);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1834] != 0.0)) {
            s.store_mul(78, 78, 93);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1834] != 0.0)) {
            s.store_sub_ad(79, A::mul(A::mul(A::add(A::sub(s.ad_value(214), s.ad_value(75)), s.ad_value(92)), A::add(A::sub(s.ad_value(214), s.ad_value(75)), s.ad_value(92))), A::div_from_scalar(1.0, s.ad_value(296))), A::div(s.ad_value(78), s.ad_value(93)));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1834] != 0.0)) {
            s.store_mul_ad_rhs(51, 294, A::sqrt(A::add(s.ad_value(78), s.ad_value(79))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1834] != 0.0)) {
            s.store_add_ad(94, A::sub_from_scalar(1.0, s.ad_value(76)), A::scale(A::mul(A::mul(s.ad_value(51), s.ad_value(87)), s.ad_value(297)), 2.0));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1834] != 0.0)) {
            s.store_div_ad(62, A::mul(A::mul(s.ad_value(62), s.ad_value(93)), A::add(s.ad_value(86), s.ad_value(77))), A::add(s.ad_value(94), A::mul(s.ad_value(93), s.ad_value(77))));
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1834] != 0.0)) {
            s.store_mul(250, 62, 269);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1834] != 0.0)) {
            s.store_offset_ad(79, A::scale(A::add(A::offset(s.ad_value(79), (-0.001)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(79), (-0.001)), A::offset(s.ad_value(79), (-0.001))), ((4.0 * 1e-5) * 1e-5)))), 0.5), 0.001);
        }

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1834] != 0.0)) {
            s.store_sqrt(71, 79);
        }

        s.v[1835] = if (((s.v[250]) as f64).abs() > 1e-35) { 1.0 } else { 0.0 };

        if ((((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) && (s.v[1835] != 0.0)) {
            s.store_div_ad_lhs(74, A::sub(s.ad_value(306), s.ad_value(73)), 250);
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_mul_ad_rhs(80, 269, A::div(A::mul(s.ad_value(296), s.ad_value(78)), A::add(s.ad_value(51), A::mul(s.ad_value(294), s.ad_value(71)))));
        }

        if (((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) && (!(s.v[1817] != 0.0))) {
            s.store_add_ad_rhs(81, 80, A::mul(s.ad_value(269), s.ad_value(74)));
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_mul_ad_lhs(939, A::mul(A::mul(A::mul(A::scale(s.ad_value(740), (p.p2 * ((p.p1147 / s.v[184]) * s.v[199]))), s.ad_value(81)), s.ad_value(250)), A::div(A::mul(s.ad_value(354), s.ad_value(344)), s.ad_value(458))), 363);
        }

        if ((s.v[1620] != 0.0) && (s.v[1799] != 0.0)) {
            s.store_add(380, 380, 939);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div(252, 251, 267);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(168, 1.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div(404, 213, 168);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div(405, 294, 168);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sub_ad(168, A::scale(s.ad_value(404), 0.5), A::scale(A::offset(A::scale(s.ad_value(405), 0.7071067811865475), 1.0), 3.0));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_rhs(169, 168, A::sqrt(A::add(A::square(s.ad_value(168)), A::scale(s.ad_value(404), 6.0))));
        }

        s.v[1836] = if (s.v[404] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1836] != 0.0)) {
            s.store_div_ad_lhs(170, A::sub(s.ad_value(404), s.ad_value(169)), 405);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1836] != 0.0)) {
            s.store_neg_ad(254, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(169)), A::square(s.ad_value(170))), 1e-38)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1836] != 0.0))) {
            s.store_limited_exp_ad(170, A::neg(s.ad_value(169)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1836] != 0.0))) {
            s.store_scale(168, 405, 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1836] != 0.0))) {
            s.store_sub_ad_lhs(169, A::sqrt(A::add(A::add(A::offset(s.ad_value(404), (-1.0)), s.ad_value(170)), A::square(s.ad_value(168)))), 168);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1836] != 0.0))) {
            s.store_sub_ad_lhs(254, A::offset(A::square(s.ad_value(169)), 1.0), 170);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(175, A::add(A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(254), (-1.0)), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sqrt(259, 175);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_ad_lhs(167, A::offset(A::div(s.ad_value(294), A::scale(s.ad_value(259), 2.0)), 1.0), 294);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sub_ad_lhs(168, A::sub(s.ad_value(254), A::scale(s.ad_value(252), 2.0)), 225);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(167), 4.0), s.ad_value(259)), 1e-38)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(175, A::sub(A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(169), A::offset(s.ad_value(169), 0.402982)), 2.446562))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.copy_ad(257, 259);
        }

    }

    pub(super) fn stamp_reactive_block_31(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[1837] = if (s.v[175] <= (-68.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1837] != 0.0)) {
            s.store_scalar(171, (-100.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1837] != 0.0)) {
            s.store_scalar(172, 20.0);
        }

        s.v[1838] = if (s.v[175] < (s.v[171] - (0.5 * s.v[172]))) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1837] != 0.0)) && (s.v[1838] != 0.0)) {
            s.store_ad(170, &A::limited_exp(s.ad_value(171)));
        }

        s.v[1839] = if (s.v[175] > (s.v[171] + (0.5 * s.v[172]))) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1837] != 0.0)) && (!(s.v[1838] != 0.0))) && (s.v[1839] != 0.0)) {
            s.store_ad(170, &A::limited_exp(s.ad_value(175)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1837] != 0.0)) && (!(s.v[1838] != 0.0))) && (!(s.v[1839] != 0.0))) {
            s.store_div_ad_lhs(169, A::sub(s.ad_value(175), s.ad_value(171)), 172);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1837] != 0.0)) && (!(s.v[1838] != 0.0))) && (!(s.v[1839] != 0.0))) {
            s.store_square(173, 169);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1837] != 0.0)) && (!(s.v[1838] != 0.0))) && (!(s.v[1839] != 0.0))) {
            s.store_limited_exp_ad(170, A::add(s.ad_value(171), A::mul(s.ad_value(172), A::add(A::offset(A::scale(s.ad_value(169), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(173), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(173), A::sub_from_scalar(1.25, s.ad_value(173)))))))));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1837] != 0.0)) {
            s.store_mul_ad_rhs(400, 170, A::sub(A::sub(A::offset(s.ad_value(168), 1.0), s.ad_value(175)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(167), 2.0), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1837] != 0.0))) {
            s.store_ad(170, &A::limited_exp(s.ad_value(175)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1837] != 0.0))) {
            s.store_div_from_scalar(258, 1.0, 257);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1837] != 0.0))) {
            s.store_sub_ad_lhs(171, A::add(A::scale(s.ad_value(170), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))), 168);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1837] != 0.0))) {
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1837] != 0.0))) {
            s.store_sub_ad_rhs(170, 170, A::div(s.ad_value(171), s.ad_value(172)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1837] != 0.0))) {
            s.store_sub_ad_lhs(171, A::add(A::scale(s.ad_value(170), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))), 168);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1837] != 0.0))) {
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1837] != 0.0))) {
            s.store_mul_ad(173, A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1837] != 0.0))) {
            s.store_sub_ad_lhs(174, A::sub(A::neg(A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170)))), A::div_from_scalar(1.0, A::mul(A::mul(A::square(s.ad_value(257)), s.ad_value(257)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))))), 173);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1837] != 0.0))) {
            s.store_sub_ad_rhs(400, 170, A::mul(A::div(s.ad_value(171), s.ad_value(172)), A::offset(A::div(A::mul(s.ad_value(171), s.ad_value(174)), A::mul(A::scale(s.ad_value(172), 2.0), s.ad_value(172))), 1.0)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(256, A::add(A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(254), (-1.0)), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sqrt(259, 256);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sub_ad_rhs(255, 254, A::scale(s.ad_value(400), 2.0));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(167, A::add(A::offset(s.ad_value(255), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(255), (-1.0)), A::offset(s.ad_value(255), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset_ad(253, A::div(s.ad_value(294), A::add(s.ad_value(259), A::sqrt(s.ad_value(167)))), 1.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(335, (1e-8 / (s.v[200] * p.p76)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_rhs(167, 269, A::sub(A::sub(s.ad_value(213), s.ad_value(254)), A::mul(A::scale(s.ad_value(400), 2.0), A::offset(s.ad_value(253), (-1.0)))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(247, A::add(s.ad_value(167), A::sqrt(A::offset(A::mul(s.ad_value(167), s.ad_value(167)), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(306, A::mul(A::scale(s.ad_value(253), 2.0), s.ad_value(269)), 400);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_rhs(308, 335, A::add(s.ad_value(247), A::scale(s.ad_value(306), s.v[338])));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_ad(169, &A::pow(A::scale(A::offset(A::div(s.ad_value(306), s.ad_value(247)), 1.0), 0.5), s.ad_value(757)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad(170, A::mul(A::add(s.ad_value(750), A::mul(s.ad_value(760), s.ad_value(218))), A::pow(s.ad_value(308), s.ad_value(651))), A::div(s.ad_value(754), s.ad_value(169)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset(171, 170, 1.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(309, A::add(A::offset(s.ad_value(171), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(171), (-1.0)), A::offset(s.ad_value(171), (-1.0))), ((0.25 * 0.0015) * 0.0015)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_from_scalar_ad(448, 1.0, A::scale(A::pow_from_scalar((s.v[183] * 1000000.0), s.ad_value(771)), p.p2));
        }

        s.v[1840] = if (p.p33 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1840] != 0.0)) {
            s.store_scalar(456, 0.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1840] != 0.0))) {
            s.store_offset_ad(167, A::mul(s.ad_value(770), s.ad_value(306)), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1840] != 0.0))) {
            s.store_mul_ad_rhs(168, 787, A::sub(s.ad_value(274), s.ad_value(299)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1840] != 0.0))) {
            s.store_add_ad_lhs(169, A::div_from_scalar(1.0, s.ad_value(167)), 168);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1840] != 0.0))) {
            s.store_add_ad_rhs(170, 169, A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01)));
        }

        s.v[1841] = if (p.p33 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1840] != 0.0))) && (s.v[1841] != 0.0)) {
            s.store_mul_ad_lhs(456, A::scale(A::mul(A::add(s.ad_value(777), A::mul(s.ad_value(776), s.ad_value(170))), s.ad_value(448)), p.p2), 652);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1840] != 0.0))) && (!(s.v[1841] != 0.0))) {
            s.store_mul_ad_lhs(456, A::add(A::add(s.ad_value(452), A::scale(A::mul(A::add(s.ad_value(777), A::mul(s.ad_value(776), s.ad_value(170))), s.ad_value(448)), p.p2)), s.ad_value(453)), 652);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_ad(167, &A::pow(s.ad_value(309), A::div_from_scalar(1.0, s.ad_value(348))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(178, 678, 218);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sqrt_ad(179, A::offset(A::square(s.ad_value(178)), 0.1));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(168, A::add(A::sub_from_scalar(1.0, s.ad_value(178)), A::sqrt(A::add(A::mul(A::sub_from_scalar(1.0, s.ad_value(178)), A::sub_from_scalar(1.0, s.ad_value(178))), s.ad_value(179)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_ad(169, A::mul(A::scale(s.ad_value(400), (10.0 * p.p497)), s.ad_value(168)), A::offset(A::mul(s.ad_value(400), s.ad_value(168)), (10.0 * p.p497)));
        }

        s.v[1842] = if (s.v[780] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1842] != 0.0)) {
            s.store_mul_ad(314, A::scale(A::div(A::mul(A::div(s.ad_value(740), s.ad_value(167)), s.ad_value(269)), A::scale(s.ad_value(746), s.v[184])), 2.0), A::div_from_scalar(1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(780), s.ad_value(169)))));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1842] != 0.0))) {
            s.store_mul_ad(314, A::scale(A::div(A::mul(A::div(s.ad_value(740), s.ad_value(167)), s.ad_value(269)), A::scale(s.ad_value(746), s.v[184])), 2.0), A::offset(A::mul(s.ad_value(780), s.ad_value(169)), 1.0));
        }

        s.v[1843] = if (s.v[456] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_mul_ad_lhs(178, A::mul(A::scale(s.ad_value(253), ((s.v[183] * 2.0) * s.v[199])), s.ad_value(269)), 746);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_div_ad(179, A::mul(A::mul(s.ad_value(178), s.ad_value(314)), s.ad_value(456)), A::scale(s.ad_value(269), 2.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_div_ad(167, A::mul(A::scale(s.ad_value(314), 0.5), A::add(A::square(s.ad_value(400)), s.ad_value(400))), A::offset(A::mul(A::scale(s.ad_value(314), 0.5), A::offset(s.ad_value(400), 1.0)), 1.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_mul_ad(168, A::scale(s.ad_value(314), 2.0), A::sub(s.ad_value(400), s.ad_value(167)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_sqrt_ad(169, A::offset(A::square(s.ad_value(168)), 1.0));
        }

        s.v[1844] = if (s.v[168] != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) && (s.v[1844] != 0.0)) {
            s.store_asinh(323, 168);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) && (s.v[1844] != 0.0)) {
            s.store_add_ad_rhs(170, 169, A::mul(A::div_from_scalar(1.0, s.ad_value(168)), s.ad_value(323)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) && (!(s.v[1844] != 0.0))) {
            s.store_add_ad_rhs(170, 169, A::div_from_scalar(1.0, s.ad_value(169)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_sub_ad(171, A::add(A::mul(s.ad_value(167), s.ad_value(170)), A::mul(A::mul(s.ad_value(179), s.ad_value(167)), A::offset(A::add(s.ad_value(400), s.ad_value(167)), 1.0))), A::mul(s.ad_value(314), A::sub(A::add(A::square(s.ad_value(400)), s.ad_value(400)), A::add(A::square(s.ad_value(167)), s.ad_value(167)))));
        }

        s.v[1845] = if (s.v[168] != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) && (s.v[1845] != 0.0)) {
            s.store_div_ad(172, A::mul(A::scale(s.ad_value(314), (-2.0)), A::sub(A::mul(s.ad_value(168), s.ad_value(169)), s.ad_value(323))), A::square(s.ad_value(168)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) && (!(s.v[1845] != 0.0))) {
            s.store_mul_ad(172, A::scale(s.ad_value(314), (-2.0)), A::div(s.ad_value(168), s.ad_value(169)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_add_ad(173, A::add(A::add(A::mul(s.ad_value(167), s.ad_value(172)), s.ad_value(170)), A::mul(s.ad_value(179), A::offset(A::add(s.ad_value(400), A::scale(s.ad_value(167), 2.0)), 1.0))), A::mul(s.ad_value(314), A::offset(A::scale(s.ad_value(167), 2.0), 1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_sub_ad_rhs(167, 167, A::div(s.ad_value(171), s.ad_value(173)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_mul_ad(168, A::scale(s.ad_value(314), 2.0), A::sub(s.ad_value(400), s.ad_value(167)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_sqrt_ad(169, A::offset(A::square(s.ad_value(168)), 1.0));
        }

        s.v[1846] = if (s.v[168] != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) && (s.v[1846] != 0.0)) {
            s.store_asinh(323, 168);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) && (s.v[1846] != 0.0)) {
            s.store_add_ad_rhs(170, 169, A::mul(A::div_from_scalar(1.0, s.ad_value(168)), s.ad_value(323)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) && (!(s.v[1846] != 0.0))) {
            s.store_add_ad_rhs(170, 169, A::div_from_scalar(1.0, s.ad_value(169)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_sub_ad(171, A::add(A::mul(s.ad_value(167), s.ad_value(170)), A::mul(A::mul(s.ad_value(179), s.ad_value(167)), A::offset(A::add(s.ad_value(400), s.ad_value(167)), 1.0))), A::mul(s.ad_value(314), A::sub(A::add(A::square(s.ad_value(400)), s.ad_value(400)), A::add(A::square(s.ad_value(167)), s.ad_value(167)))));
        }

        s.v[1847] = if (s.v[168] != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) && (s.v[1847] != 0.0)) {
            s.store_div_ad(172, A::mul(A::scale(s.ad_value(314), (-2.0)), A::sub(A::mul(s.ad_value(168), s.ad_value(169)), s.ad_value(323))), A::square(s.ad_value(168)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) && (!(s.v[1847] != 0.0))) {
            s.store_mul_ad(172, A::scale(s.ad_value(314), (-2.0)), A::div(s.ad_value(168), s.ad_value(169)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_add_ad(173, A::add(A::add(A::mul(s.ad_value(167), s.ad_value(172)), s.ad_value(170)), A::mul(s.ad_value(179), A::offset(A::add(s.ad_value(400), A::scale(s.ad_value(167), 2.0)), 1.0))), A::mul(s.ad_value(314), A::offset(A::scale(s.ad_value(167), 2.0), 1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_sub_ad_rhs(307, 167, A::div(s.ad_value(171), s.ad_value(173)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) {
            s.store_div_ad(167, A::mul(A::scale(s.ad_value(314), 0.5), A::add(A::square(s.ad_value(400)), s.ad_value(400))), A::offset(A::mul(A::scale(s.ad_value(314), 0.5), A::offset(s.ad_value(400), 1.0)), 1.0));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) {
            s.store_mul_ad(168, A::scale(s.ad_value(314), 2.0), A::sub(s.ad_value(400), s.ad_value(167)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) {
            s.store_sqrt_ad(169, A::offset(A::square(s.ad_value(168)), 1.0));
        }

        s.v[1848] = if (s.v[168] != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) && (s.v[1848] != 0.0)) {
            s.store_asinh(323, 168);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) && (s.v[1848] != 0.0)) {
            s.store_add_ad_rhs(170, 169, A::mul(A::div_from_scalar(1.0, s.ad_value(168)), s.ad_value(323)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) && (!(s.v[1848] != 0.0))) {
            s.store_add_ad_rhs(170, 169, A::div_from_scalar(1.0, s.ad_value(169)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) {
            s.store_sub_ad(171, A::mul(s.ad_value(167), s.ad_value(170)), A::mul(s.ad_value(314), A::sub(A::add(A::square(s.ad_value(400)), s.ad_value(400)), A::add(A::square(s.ad_value(167)), s.ad_value(167)))));
        }

        s.v[1849] = if (s.v[168] != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) && (s.v[1849] != 0.0)) {
            s.store_div_ad(172, A::mul(A::scale(s.ad_value(314), (-2.0)), A::sub(A::mul(s.ad_value(168), s.ad_value(169)), s.ad_value(323))), A::square(s.ad_value(168)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) && (!(s.v[1849] != 0.0))) {
            s.store_mul_ad(172, A::scale(s.ad_value(314), (-2.0)), A::div(s.ad_value(168), s.ad_value(169)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) {
            s.store_add_ad(173, A::add(A::mul(s.ad_value(167), s.ad_value(172)), s.ad_value(170)), A::mul(s.ad_value(314), A::offset(A::scale(s.ad_value(167), 2.0), 1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) {
            s.store_sub_ad_rhs(167, 167, A::div(s.ad_value(171), s.ad_value(173)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) {
            s.store_mul_ad(168, A::scale(s.ad_value(314), 2.0), A::sub(s.ad_value(400), s.ad_value(167)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) {
            s.store_sqrt_ad(169, A::offset(A::square(s.ad_value(168)), 1.0));
        }

        s.v[1850] = if (s.v[168] != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) && (s.v[1850] != 0.0)) {
            s.store_asinh(323, 168);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) && (s.v[1850] != 0.0)) {
            s.store_add_ad_rhs(170, 169, A::mul(A::div_from_scalar(1.0, s.ad_value(168)), s.ad_value(323)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) && (!(s.v[1850] != 0.0))) {
            s.store_add_ad_rhs(170, 169, A::div_from_scalar(1.0, s.ad_value(169)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) {
            s.store_sub_ad(171, A::mul(s.ad_value(167), s.ad_value(170)), A::mul(s.ad_value(314), A::sub(A::add(A::square(s.ad_value(400)), s.ad_value(400)), A::add(A::square(s.ad_value(167)), s.ad_value(167)))));
        }

        s.v[1851] = if (s.v[168] != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) && (s.v[1851] != 0.0)) {
            s.store_div_ad(172, A::mul(A::scale(s.ad_value(314), (-2.0)), A::sub(A::mul(s.ad_value(168), s.ad_value(169)), s.ad_value(323))), A::square(s.ad_value(168)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) && (!(s.v[1851] != 0.0))) {
            s.store_mul_ad(172, A::scale(s.ad_value(314), (-2.0)), A::div(s.ad_value(168), s.ad_value(169)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) {
            s.store_add_ad(173, A::add(A::mul(s.ad_value(167), s.ad_value(172)), s.ad_value(170)), A::mul(s.ad_value(314), A::offset(A::scale(s.ad_value(167), 2.0), 1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) {
            s.store_sub_ad_rhs(307, 167, A::div(s.ad_value(171), s.ad_value(173)));
        }

        if (!(s.v[1620] != 0.0)) {
            let assign46440_ad_e78990: A = A::sub(A::sub(s.ad_value(254), A::scale(s.ad_value(252), 2.0)), A::add(A::scale(s.ad_value(307), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::mul(A::scale(s.ad_value(307), 2.0), s.ad_value(253)), s.ad_value(295)), A::add(A::mul(A::mul(A::scale(s.ad_value(307), 2.0), s.ad_value(253)), s.ad_value(295)), A::div(s.ad_value(294), A::offset(s.ad_value(253), (-1.0))))), 1e-38))));
            s.store_ad(319, &assign46440_ad_e78990);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(312, 319, 269);
        }

        s.v[1852] = if ((p.p1349 == 0.0) && (p.p1350 == 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1852] != 0.0)) {
            s.store_scalar(1019, 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1852] != 0.0))) {
            s.store_div_from_scalar_ad(168, s.v[184], A::offset(A::sqrt(A::mul(s.ad_value(782), s.ad_value(275))), s.v[184]));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1852] != 0.0))) {
            s.store_offset_ad(1019, A::div(A::sub(A::scale(s.ad_value(168), p.p1349), A::mul(A::mul(A::scale(s.ad_value(168), p.p1350), A::powf(s.ad_value(400), p.p1351)), s.ad_value(269))), A::offset(A::scale(s.ad_value(218), p.p1352), 1.0)), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1852] != 0.0))) {
            s.store_scale_ad(1019, A::add(A::offset(s.ad_value(1019), 0.1), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(1019), (-0.1)), A::offset(s.ad_value(1019), (-0.1))), ((0.25 * 0.0005) * 0.0005)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(316, A::add(A::sub(s.ad_value(312), s.ad_value(224)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(312), s.ad_value(224)), A::sub(s.ad_value(312), s.ad_value(224))), ((0.25 * 0.001) * 0.001)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div(316, 316, 1019);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_pow_ad(174, A::offset(A::div(s.ad_value(226), s.ad_value(316)), 1e-6), A::div_from_scalar(1.0, s.ad_value(656)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_pow_ad(175, A::offset(s.ad_value(174), 1.0), A::neg(s.ad_value(656)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(315, 226, 175);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(318, A::add(s.ad_value(315), s.ad_value(224)), 270);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(175, A::add(A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(254), (-1.0)), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sqrt(259, 175);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_ad_lhs(167, A::offset(A::div(s.ad_value(294), A::scale(s.ad_value(259), 2.0)), 1.0), 294);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sub_ad_lhs(168, A::sub(s.ad_value(254), A::scale(s.ad_value(252), 2.0)), 318);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(167), 4.0), s.ad_value(259)), 1e-38)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(175, A::sub(A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(169), A::offset(s.ad_value(169), 0.402982)), 2.446562))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.copy_ad(257, 259);
        }

        s.v[1853] = if (s.v[175] <= (-68.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1853] != 0.0)) {
            s.store_scalar(171, (-100.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1853] != 0.0)) {
            s.store_scalar(172, 20.0);
        }

        s.v[1854] = if (s.v[175] < (s.v[171] - (0.5 * s.v[172]))) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1853] != 0.0)) && (s.v[1854] != 0.0)) {
            s.store_ad(170, &A::limited_exp(s.ad_value(171)));
        }

        s.v[1855] = if (s.v[175] > (s.v[171] + (0.5 * s.v[172]))) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1853] != 0.0)) && (!(s.v[1854] != 0.0))) && (s.v[1855] != 0.0)) {
            s.store_ad(170, &A::limited_exp(s.ad_value(175)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1853] != 0.0)) && (!(s.v[1854] != 0.0))) && (!(s.v[1855] != 0.0))) {
            s.store_div_ad_lhs(169, A::sub(s.ad_value(175), s.ad_value(171)), 172);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1853] != 0.0)) && (!(s.v[1854] != 0.0))) && (!(s.v[1855] != 0.0))) {
            s.store_square(173, 169);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1853] != 0.0)) && (!(s.v[1854] != 0.0))) && (!(s.v[1855] != 0.0))) {
            s.store_limited_exp_ad(170, A::add(s.ad_value(171), A::mul(s.ad_value(172), A::add(A::offset(A::scale(s.ad_value(169), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(173), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(173), A::sub_from_scalar(1.25, s.ad_value(173)))))))));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1853] != 0.0)) {
            s.store_mul_ad_rhs(320, 170, A::sub(A::sub(A::offset(s.ad_value(168), 1.0), s.ad_value(175)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(167), 2.0), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))));
        }

    }

    pub(super) fn stamp_reactive_block_32(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((!(s.v[1620] != 0.0)) && (!(s.v[1853] != 0.0))) {
            s.store_ad(170, &A::limited_exp(s.ad_value(175)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1853] != 0.0))) {
            s.store_div_from_scalar(258, 1.0, 257);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1853] != 0.0))) {
            s.store_sub_ad_lhs(171, A::add(A::scale(s.ad_value(170), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))), 168);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1853] != 0.0))) {
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1853] != 0.0))) {
            s.store_sub_ad_rhs(170, 170, A::div(s.ad_value(171), s.ad_value(172)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1853] != 0.0))) {
            s.store_sub_ad_lhs(171, A::add(A::scale(s.ad_value(170), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))), 168);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1853] != 0.0))) {
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1853] != 0.0))) {
            s.store_mul_ad(173, A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1853] != 0.0))) {
            s.store_sub_ad_lhs(174, A::sub(A::neg(A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170)))), A::div_from_scalar(1.0, A::mul(A::mul(A::square(s.ad_value(257)), s.ad_value(257)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))))), 173);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1853] != 0.0))) {
            s.store_sub_ad_rhs(320, 170, A::mul(A::div(s.ad_value(171), s.ad_value(172)), A::offset(A::div(A::mul(s.ad_value(171), s.ad_value(174)), A::mul(A::scale(s.ad_value(172), 2.0), s.ad_value(172))), 1.0)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset_ad(255, A::sub(A::sub(s.ad_value(254), s.ad_value(400)), s.ad_value(320)), (-1.0));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(167, A::add(A::offset(s.ad_value(255), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(255), (-1.0)), A::offset(s.ad_value(255), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sqrt(169, 167);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset_ad(253, A::div(s.ad_value(294), A::add(s.ad_value(259), s.ad_value(169))), 1.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad(417, A::sub(s.ad_value(400), s.ad_value(320)), A::sub(s.ad_value(400), s.ad_value(320)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_from_scalar_ad(167, 1.0, A::add(A::offset(s.ad_value(400), 1.0), s.ad_value(320)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(168, 417, 167);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sub_ad(381, A::sub(s.ad_value(213), s.ad_value(254)), A::mul(A::offset(s.ad_value(253), (-1.0)), A::add(A::add(s.ad_value(400), s.ad_value(320)), A::scale(s.ad_value(168), 0.3333333333333333))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale(169, 253, 0.3333333333333333);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(170, 168, 167);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_rhs(382, 169, A::add(A::add(A::scale(s.ad_value(400), 2.0), s.ad_value(320)), A::mul(A::scale(A::add(A::offset(A::scale(s.ad_value(400), 0.8), 1.0), A::scale(s.ad_value(320), 1.2)), 0.5), s.ad_value(170))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_rhs(385, 169, A::add(A::add(s.ad_value(400), A::scale(s.ad_value(320), 2.0)), A::mul(A::scale(A::add(A::offset(A::scale(s.ad_value(400), 1.2), 1.0), A::scale(s.ad_value(320), 0.8)), 0.5), s.ad_value(170))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(244, A::add(A::mul(s.ad_value(269), s.ad_value(381)), A::sqrt(A::offset(A::mul(A::mul(s.ad_value(269), s.ad_value(381)), A::mul(s.ad_value(269), s.ad_value(381))), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_rhs(243, 269, A::add(s.ad_value(382), s.ad_value(385)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_rhs(336, 335, A::add(s.ad_value(244), A::scale(s.ad_value(243), s.v[338])));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_ad(169, &A::pow(A::scale(A::offset(A::div(s.ad_value(243), s.ad_value(244)), 1.0), 0.5), s.ad_value(757)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad(170, A::mul(A::add(s.ad_value(750), A::mul(s.ad_value(760), s.ad_value(218))), A::pow(s.ad_value(336), s.ad_value(651))), A::div(s.ad_value(754), s.ad_value(169)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset(171, 170, 1.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(339, A::add(A::offset(s.ad_value(171), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(171), (-1.0)), A::offset(s.ad_value(171), (-1.0))), ((0.25 * 0.0015) * 0.0015)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_ad(310, A::scale(s.ad_value(746), 2.0), A::div(s.ad_value(740), s.ad_value(339)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale(311, 310, s.v[184]);
        }

        s.v[1856] = if (s.v[781] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1856] != 0.0)) {
            s.store_offset_ad(360, A::div(A::mul(s.ad_value(781), s.ad_value(243)), s.ad_value(311)), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1856] != 0.0))) {
            s.store_div_from_scalar_ad(360, 1.0, A::sub_from_scalar(1.0, A::div(A::mul(s.ad_value(781), s.ad_value(243)), s.ad_value(311))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.copy_ad(359, 763);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sub(355, 226, 315);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_rhs(362, 243, A::scale(s.ad_value(269), 2.0));
        }

        s.v[1857] = if (s.v[359] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1857] != 0.0)) {
            s.store_div_ad_rhs(170, 362, A::add(s.ad_value(316), s.ad_value(362)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1857] != 0.0)) {
            s.store_scale_ad(171, A::add(A::offset(A::mul(s.ad_value(764), s.ad_value(218)), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(764), s.ad_value(218)), 1.0), A::offset(A::mul(s.ad_value(764), s.ad_value(218)), 1.0)), ((4.0 * 0.001) * 0.001)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1857] != 0.0)) {
            s.store_div_from_scalar(172, 1.0, 171);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1857] != 0.0)) {
            s.store_mul_ad_lhs(361, A::mul(A::mul(A::div(s.ad_value(362), s.ad_value(359)), s.ad_value(170)), s.ad_value(360)), 172);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1857] != 0.0)) {
            s.store_offset_ad(363, A::div(s.ad_value(355), s.ad_value(361)), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1857] != 0.0))) {
            s.store_scalar(363, 1.0);
        }

        s.v[1858] = if (s.v[769] <= 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1858] != 0.0)) {
            s.store_scalar(268, 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1858] != 0.0))) {
            s.store_div_ad_lhs(176, A::scale(s.ad_value(769), ((s.v[184]) as f64).sqrt()), 362);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1858] != 0.0))) {
            s.store_div_from_scalar_ad(268, 1.0, A::offset(s.ad_value(176), 1.0));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add(358, 316, 311);
        }

        s.v[1859] = if (s.v[785] > 0.0) { 1.0 } else { 0.0 };

        s.v[1860] = if (p.p414 < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1859] != 0.0)) && (s.v[1860] != 0.0)) {
            s.store_div_ad_lhs(168, A::div(s.ad_value(785), A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(243), p.p414), s.ad_value(311)))), 268);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1859] != 0.0)) && (!(s.v[1860] != 0.0))) {
            s.store_div_ad_lhs(168, A::mul(s.ad_value(785), A::offset(A::div(A::scale(s.ad_value(243), p.p414), s.ad_value(311)), 1.0)), 268);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1859] != 0.0)) {
            s.store_offset_ad(364, A::mul(s.ad_value(168), A::ln(A::max_with_scalar(A::offset(A::div(A::div(s.ad_value(355), s.ad_value(168)), s.ad_value(358)), 1.0), 1e-38))), 1.0);
        }

        s.v[1861] = if (p.p414 < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1859] != 0.0))) && (s.v[1861] != 0.0)) {
            s.store_div_ad_lhs(168, A::div(s.ad_value(785), A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(243), p.p414), s.ad_value(311)))), 268);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1859] != 0.0))) && (!(s.v[1861] != 0.0))) {
            s.store_div_ad_lhs(168, A::mul(s.ad_value(785), A::offset(A::div(A::scale(s.ad_value(243), p.p414), s.ad_value(311)), 1.0)), 268);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1859] != 0.0))) {
            s.store_offset(364, 168, 1.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(363, 363, 364);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_limited_exp_ad(168, A::mul(s.ad_value(768), s.ad_value(226)));
        }

        s.v[1862] = if (s.v[767] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1862] != 0.0)) {
            s.store_scalar(169, (1.0 + (p.p433 * s.v[184])));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1862] != 0.0)) {
            s.store_div_ad_lhs(356, A::offset(A::mul(s.ad_value(169), s.ad_value(168)), 1.0), 767);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1862] != 0.0)) {
            s.store_mul(356, 356, 268);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1862] != 0.0))) {
            s.store_scalar(356, 5.540622384e34);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div(171, 355, 356);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset(167, 171, 1.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(363, 363, 167);
        }

        s.v[1863] = if (s.v[766] > 0.0) { 1.0 } else { 0.0 };

        s.v[1864] = if (s.v[355] > ((s.v[765] * s.v[300]) / 80.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1863] != 0.0)) && (s.v[1864] != 0.0)) {
            s.store_div_ad_lhs(167, A::mul(s.ad_value(765), s.ad_value(300)), 355);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1863] != 0.0)) && (s.v[1864] != 0.0)) {
            s.store_div_ad_lhs(357, A::scale(A::limited_exp(s.ad_value(167)), s.v[184]), 766);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1863] != 0.0)) && (!(s.v[1864] != 0.0))) {
            s.store_div_from_scalar(357, (5.540622384e34 * s.v[184]), 766);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1863] != 0.0))) {
            s.store_scalar(357, 5.540622384e34);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset_ad(365, A::div(s.ad_value(355), s.ad_value(357)), 1.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(363, 363, 365);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_ad(167, &A::pow(s.ad_value(339), A::div_from_scalar(1.0, s.ad_value(348))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(178, 678, 218);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sqrt_ad(179, A::offset(A::square(s.ad_value(178)), 0.1));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(168, A::add(A::sub_from_scalar(1.0, s.ad_value(178)), A::sqrt(A::add(A::mul(A::sub_from_scalar(1.0, s.ad_value(178)), A::sub_from_scalar(1.0, s.ad_value(178))), s.ad_value(179)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_ad(169, A::mul(A::scale(s.ad_value(243), (10.0 * p.p497)), s.ad_value(168)), A::offset(A::mul(s.ad_value(243), s.ad_value(168)), (10.0 * p.p497)));
        }

        s.v[1865] = if (s.v[780] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1865] != 0.0)) {
            s.store_mul_ad(314, A::scale(A::div(A::mul(A::div(s.ad_value(740), s.ad_value(167)), s.ad_value(269)), A::scale(s.ad_value(746), s.v[184])), 2.0), A::div_from_scalar(1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(780), s.ad_value(169)))));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1865] != 0.0))) {
            s.store_mul_ad(314, A::scale(A::div(A::mul(A::div(s.ad_value(740), s.ad_value(167)), s.ad_value(269)), A::scale(s.ad_value(746), s.v[184])), 2.0), A::offset(A::mul(s.ad_value(780), s.ad_value(169)), 1.0));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad(168, A::scale(s.ad_value(314), 2.0), A::sub(s.ad_value(400), s.ad_value(320)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sqrt_ad(169, A::offset(A::square(s.ad_value(168)), 1.0));
        }

        s.v[1866] = if (s.v[168] != 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1866] != 0.0)) {
            s.store_scale_ad(343, A::add(s.ad_value(169), A::mul(A::div_from_scalar(1.0, s.ad_value(168)), A::asinh(s.ad_value(168)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1866] != 0.0))) {
            s.store_scale_ad(343, A::add(s.ad_value(169), A::div_from_scalar(1.0, s.ad_value(169))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.copy_ad(345, 343);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(454, 0.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(455, 0.0);
        }

        s.v[1867] = if (p.p33 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_scalar(457, 0.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_scalar(458, 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_sub(169, 203, 219);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_sqrt_ad(170, A::offset(A::square(s.ad_value(169)), 0.01));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_scaled_add(228, 169, 170, 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_offset_ad(172, A::mul(s.ad_value(770), s.ad_value(228)), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_add_ad(173, A::div_from_scalar(1.0, s.ad_value(172)), A::mul(s.ad_value(787), s.ad_value(202)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_scale_ad(171, A::add(s.ad_value(173), A::sqrt(A::offset(A::square(s.ad_value(173)), 0.01))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_mul_ad_rhs(454, 652, A::add(s.ad_value(452), A::mul(A::add(s.ad_value(773), A::mul(s.ad_value(775), s.ad_value(171))), s.ad_value(448))));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_sub(169, 204, 219);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_sqrt_ad(170, A::offset(A::square(s.ad_value(169)), 0.01));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_scaled_add(229, 169, 170, 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_offset_ad(172, A::mul(s.ad_value(770), s.ad_value(229)), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_add_ad(173, A::div_from_scalar(1.0, s.ad_value(172)), A::mul(s.ad_value(787), s.ad_value(201)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_scale_ad(171, A::add(s.ad_value(173), A::sqrt(A::offset(A::square(s.ad_value(173)), 0.01))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_mul_ad_rhs(455, 652, A::add(s.ad_value(453), A::mul(A::add(s.ad_value(772), A::mul(s.ad_value(774), s.ad_value(171))), s.ad_value(448))));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) {
            s.store_offset_ad(167, A::mul(s.ad_value(770), s.ad_value(243)), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) {
            s.store_mul_ad_rhs(168, 787, A::sub(s.ad_value(274), s.ad_value(299)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) {
            s.store_add_ad_lhs(169, A::div_from_scalar(1.0, s.ad_value(167)), 168);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) {
            s.store_scale_ad(170, A::add(s.ad_value(169), A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) {
            s.store_scale_ad(457, A::mul(A::mul(s.ad_value(652), A::add(s.ad_value(777), A::mul(s.ad_value(776), s.ad_value(170)))), s.ad_value(448)), p.p2);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) {
            s.copy_ad(455, 453);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) {
            s.copy_ad(454, 452);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) {
            s.store_offset_ad(458, A::mul(A::mul(A::scale(A::div(s.ad_value(740), A::mul(s.ad_value(343), s.ad_value(339))), (s.v[199] * (s.v[183] * 1.0 / (s.v[184])))), s.ad_value(243)), s.ad_value(457)), 1.0);
        }

        s.v[1868] = if (p.p33 == 2.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) && (s.v[1868] != 0.0)) {
            s.store_mul_ad_rhs(457, 652, A::add(A::add(s.ad_value(452), A::scale(A::mul(A::add(s.ad_value(777), A::mul(s.ad_value(776), s.ad_value(170))), s.ad_value(448)), p.p2)), s.ad_value(453)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) && (s.v[1868] != 0.0)) {
            s.store_scalar(455, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) && (s.v[1868] != 0.0)) {
            s.store_scalar(454, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) && (s.v[1868] != 0.0)) {
            s.store_offset_ad(458, A::mul(A::mul(A::scale(A::div(s.ad_value(740), A::mul(s.ad_value(343), s.ad_value(339))), (s.v[199] * (s.v[183] * 1.0 / (s.v[184])))), s.ad_value(243)), s.ad_value(457)), 1.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_rhs(167, 330, A::div(s.ad_value(333), A::add(s.ad_value(243), A::mul(A::scale(s.ad_value(267), 2.0), s.ad_value(637)))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sub(416, 400, 320);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(168, A::mul(s.ad_value(167), s.ad_value(416)), 416);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset(169, 168, ((1.0) + ((-0.001))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset_ad(170, A::scale(A::add(s.ad_value(169), A::sqrt(A::offset(A::square(s.ad_value(169)), 0.004))), 0.5), (-1.0));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(334, A::offset(A::sqrt(A::offset(s.ad_value(170), 1.0)), 1.0), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset_ad(334, A::scale(A::sub(A::offset(s.ad_value(334), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(334), (-1.0)), A::offset(s.ad_value(334), (-1.0))), ((0.25 * 0.01) * 0.01)))), 0.5), (0.25 * 0.01));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add(167, 400, 320);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sub(168, 400, 320);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_ad_rhs(169, 168, A::add(s.ad_value(167), s.ad_value(833)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(170, A::mul(s.ad_value(832), s.ad_value(169)), 169);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset(834, 170, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_33(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (!(s.v[1620] != 0.0)) {
            s.store_div_ad_rhs(176, 858, A::add(A::mul(A::max_from_scalar(0.0, A::add(s.ad_value(861), A::mul(A::mul(s.ad_value(864), s.ad_value(168)), s.ad_value(168)))), s.ad_value(167)), A::mul(A::scale(s.ad_value(267), 2.0), s.ad_value(637))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_limited_exp_ad(853, A::neg(s.ad_value(176)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(340, A::mul(s.ad_value(339), s.ad_value(343)), 458);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div(337, 740, 340);
        }

        if (!(s.v[1620] != 0.0)) {
            let assign48130_ad_e81153: A = A::div(A::mul(A::mul(A::mul(A::mul(A::scale(A::scale(A::mul(A::scale(s.ad_value(253), (2.0 * p.p2)), s.ad_value(337)), (s.v[183] * 1.0 / (s.v[184]))), s.v[199]), s.ad_value(269)), s.ad_value(269)), A::mul(A::sub(s.ad_value(400), s.ad_value(320)), A::add(A::offset(s.ad_value(400), 1.0), s.ad_value(320)))), s.ad_value(363)), s.ad_value(334));
            s.store_mul_ad_lhs(380, A::mul(assign48130_ad_e81153, s.ad_value(834)), 853);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale(380, 380, p.p26);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(467, 0.0);
        }

        s.v[1869] = if (p.p7 > 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1869] != 0.0)) {
            s.store_mul_ad_lhs(468, A::scale(A::scale(s.ad_value(337), (s.v[183] * 1.0 / (s.v[184]))), s.v[199]), 243);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1869] != 0.0)) {
            s.store_scale(176, 271, p.p1009);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1869] != 0.0)) {
            s.store_scale_ad(167, A::scale(A::mul(s.ad_value(176), s.ad_value(337)), (s.v[183] * 1.0 / (s.v[184]))), s.v[199]);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1869] != 0.0)) {
            s.store_scaled_add(467, 167, 468, (p.p1008 * p.p2));
        }

        s.v[1870] = if (p.p7 == 2.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1869] != 0.0)) && (s.v[1870] != 0.0)) {
            s.store_div_from_scalar(466, 1.0, 465);
        }

        s.v[1871] = if (s.v[466] < p.p1347) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1869] != 0.0)) && (s.v[1870] != 0.0)) && (s.v[1871] != 0.0)) {
            s.store_scalar(466, p.p1347);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1869] != 0.0)) && (s.v[1870] != 0.0)) && (s.v[1871] != 0.0)) {
            s.store_div_from_scalar(465, 1.0, 466);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1869] != 0.0)) && (s.v[1870] != 0.0)) {
            s.store_add(178, 465, 467);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1869] != 0.0)) && (s.v[1870] != 0.0)) {
            s.store_div_ad_lhs(467, A::mul(s.ad_value(465), s.ad_value(467)), 178);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(544, ((s.v[183] / p.p1373) + p.p1377));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(543, ((s.v[183] / p.p1373) + p.p1378));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale(545, 543, p.p74);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale(546, 544, p.p74);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(593, 637, 590);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div(167, 498, 593);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_ad(595, &A::limited_exp(s.ad_value(167)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(594, 637, 590);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div(167, 499, 594);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_ad(596, &A::limited_exp(s.ad_value(167)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad(171, A::div_from_scalar(1.115, s.ad_value(637)), A::offset(s.ad_value(639), (-1.0)));
        }

        s.v[1872] = if (s.v[550] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1872] != 0.0))) {
            s.store_div_ad_lhs(174, A::mul(s.ad_value(547), s.ad_value(171)), 590);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1872] != 0.0))) {
            s.store_ad(168, &A::limited_exp(s.ad_value(174)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1872] != 0.0))) {
            s.store_mul(548, 550, 168);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1872] != 0.0))) {
            s.store_mul(167, 545, 548);
        }

        s.v[1873] = if (s.v[551] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1873] != 0.0))) {
            s.store_div_ad_lhs(174, A::mul(s.ad_value(547), s.ad_value(171)), 590);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1873] != 0.0))) {
            s.store_ad(168, &A::limited_exp(s.ad_value(174)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1873] != 0.0))) {
            s.store_mul(549, 551, 168);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1873] != 0.0))) {
            s.store_mul(167, 546, 549);
        }

        s.v[1874] = if (s.v[552] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) {
            s.store_div_ad_lhs(174, A::mul(s.ad_value(556), s.ad_value(171)), 557);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) {
            s.store_ad(169, &A::limited_exp(s.ad_value(174)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) {
            s.store_mul(554, 552, 169);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) {
            s.store_mul_ad(562, A::scale(s.ad_value(557), p.p925), A::offset(A::mul(s.ad_value(565), A::offset(s.ad_value(639), (-1.0))), 1.0));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) {
            s.store_mul_ad(563, A::scale(s.ad_value(564), p.p925), A::offset(A::mul(s.ad_value(566), A::offset(s.ad_value(639), (-1.0))), 1.0));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) {
            s.store_div(167, 498, 562);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) {
            s.store_ad(177, &A::limited_exp(s.ad_value(167)));
        }

        s.v[1875] = if ((s.v[558] - s.v[498]) < 0.001) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) && (s.v[1875] != 0.0)) {
            s.store_scalar(168, 1000.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) && (s.v[1875] != 0.0)) {
            s.store_mul_ad_lhs(167, A::mul(A::div(A::neg(s.ad_value(498)), s.ad_value(563)), s.ad_value(558)), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) && (s.v[1875] != 0.0)) {
            s.store_ad(178, &A::limited_exp(s.ad_value(167)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) && (s.v[1875] != 0.0)) {
            s.store_neg(178, 178);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) && (!(s.v[1875] != 0.0))) {
            s.store_div_from_scalar_ad(168, 1.0, A::sub(s.ad_value(558), s.ad_value(498)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) && (!(s.v[1875] != 0.0))) {
            s.store_mul_ad_lhs(167, A::mul(A::div(A::neg(s.ad_value(498)), s.ad_value(563)), s.ad_value(558)), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) && (!(s.v[1875] != 0.0))) {
            s.store_ad(178, &A::limited_exp(s.ad_value(167)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) && (!(s.v[1875] != 0.0))) {
            s.store_neg(178, 178);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) {
            s.store_mul(170, 545, 554);
        }

        s.v[1876] = if (s.v[553] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) {
            s.store_div_ad_lhs(174, A::mul(s.ad_value(556), s.ad_value(171)), 557);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) {
            s.store_ad(169, &A::limited_exp(s.ad_value(174)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) {
            s.store_mul(555, 553, 169);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) {
            s.store_mul_ad(562, A::scale(s.ad_value(557), p.p925), A::offset(A::mul(s.ad_value(565), A::offset(s.ad_value(639), (-1.0))), 1.0));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) {
            s.store_mul_ad(563, A::scale(s.ad_value(564), p.p925), A::offset(A::mul(s.ad_value(566), A::offset(s.ad_value(639), (-1.0))), 1.0));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) {
            s.store_div(167, 499, 562);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) {
            s.store_ad(177, &A::limited_exp(s.ad_value(167)));
        }

        s.v[1877] = if ((s.v[559] - s.v[499]) < 0.001) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) && (s.v[1877] != 0.0)) {
            s.store_scalar(168, 1000.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) && (s.v[1877] != 0.0)) {
            s.store_mul_ad_lhs(167, A::mul(A::div(A::neg(s.ad_value(499)), s.ad_value(563)), s.ad_value(559)), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) && (s.v[1877] != 0.0)) {
            s.store_ad(178, &A::limited_exp(s.ad_value(167)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) && (s.v[1877] != 0.0)) {
            s.store_neg(178, 178);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) && (!(s.v[1877] != 0.0))) {
            s.store_div_from_scalar_ad(168, 1.0, A::sub(s.ad_value(559), s.ad_value(499)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) && (!(s.v[1877] != 0.0))) {
            s.store_mul_ad_lhs(167, A::mul(A::div(A::neg(s.ad_value(499)), s.ad_value(563)), s.ad_value(559)), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) && (!(s.v[1877] != 0.0))) {
            s.store_ad(178, &A::limited_exp(s.ad_value(167)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) && (!(s.v[1877] != 0.0))) {
            s.store_neg(178, 178);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) {
            s.store_mul(170, 546, 555);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(602, ((s.v[183] / p.p1373) * p.p74));
        }

        s.v[1878] = if ((s.v[598] == 0.0) && (s.v[597] == 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_div_ad_lhs(174, A::mul(s.ad_value(589), s.ad_value(171)), 590);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_ad(167, &A::limited_exp(s.ad_value(174)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul(585, 587, 167);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul(578, 598, 167);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_div_ad_lhs(174, A::mul(s.ad_value(589), s.ad_value(171)), 590);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_ad(167, &A::limited_exp(s.ad_value(174)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul(586, 588, 167);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul(577, 597, 167);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul_ad_rhs(583, 585, A::offset(s.ad_value(595), (-1.0)));
        }

        s.v[1879] = if (s.v[583] < 1e-5) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (s.v[1879] != 0.0)) {
            s.store_scalar(583, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (s.v[1879] != 0.0)) {
            s.store_scalar(591, 1.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (!(s.v[1879] != 0.0))) {
            s.store_div_from_scalar_ad(591, 1.0, A::sqrt(A::offset(s.ad_value(583), 1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul_ad_rhs(584, 586, A::offset(s.ad_value(596), (-1.0)));
        }

        s.v[1880] = if (s.v[584] < 1e-5) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (s.v[1880] != 0.0)) {
            s.store_scalar(584, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (s.v[1880] != 0.0)) {
            s.store_scalar(592, 1.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (!(s.v[1880] != 0.0))) {
            s.store_div_from_scalar_ad(592, 1.0, A::sqrt(A::offset(s.ad_value(584), 1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_scalar(167, (((((-0.5) * s.v[184]) * s.v[184]) / p.p595) / p.p595));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_ad(603, &A::limited_exp(s.ad_value(167)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_sub_from_scalar(169, 1.0, 603);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_scale(167, 601, ((1.0 / s.v[184]) + (1.0 / p.p595)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_ad(599, &A::pow(s.ad_value(167), s.ad_value(600)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul_ad_lhs(604, A::mul(s.ad_value(602), s.ad_value(578)), 599);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul(168, 167, 604);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul_ad_lhs(604, A::mul(s.ad_value(602), s.ad_value(577)), 599);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul(168, 167, 604);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_offset_ad(531, A::scale(A::pow(s.ad_value(167), s.ad_value(530)), p.p920), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul_ad_lhs(532, A::mul(s.ad_value(602), s.ad_value(578)), 531);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul_ad_lhs(533, A::mul(s.ad_value(532), A::offset(s.ad_value(595), (-1.0))), 591);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul_ad_lhs(532, A::mul(s.ad_value(602), s.ad_value(577)), 531);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul_ad_lhs(534, A::mul(s.ad_value(532), A::offset(s.ad_value(596), (-1.0))), 592);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_add_ad_rhs(580, 581, A::scale(s.ad_value(582), s.v[184]));
        }

        s.v[1881] = if (s.v[580] < 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (s.v[1881] != 0.0)) {
            s.store_scalar(580, 1.0);
        }

        s.v[1882] = if (p.p554 == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (!(s.v[1882] != 0.0))) {
            s.store_offset_ad(167, A::div(A::add(s.ad_value(498), s.ad_value(499)), s.ad_value(580)), 1.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (!(s.v[1882] != 0.0))) {
            s.store_add(168, 583, 584);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (!(s.v[1882] != 0.0))) {
            s.store_sqrt_ad(170, A::add(A::square(s.ad_value(167)), A::scale(s.ad_value(168), 4.0)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (!(s.v[1882] != 0.0))) {
            s.store_scaled_add(169, 167, 170, 0.5);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (!(s.v[1882] != 0.0))) {
            s.store_mul(167, 603, 604);
        }

        s.v[1884] = if ((s.v[567] == 0.0) && (s.v[568] == 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) {
            s.store_mul_ad_rhs(174, 569, A::offset(s.ad_value(639), (-1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) {
            s.store_ad(167, &A::limited_exp(s.ad_value(174)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) {
            s.store_mul(571, 567, 167);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) {
            s.store_mul_ad_rhs(174, 570, A::offset(s.ad_value(639), (-1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) {
            s.store_ad(167, &A::limited_exp(s.ad_value(174)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) {
            s.store_mul(572, 568, 167);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) {
            s.store_scale(594, 573, p.p925);
        }

        s.v[1885] = if ((s.v[575] - s.v[498]) < 0.001) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (s.v[1885] != 0.0)) {
            s.store_scalar(168, 1000.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (s.v[1885] != 0.0)) {
            s.store_mul_ad_lhs(167, A::mul(A::div(A::neg(s.ad_value(498)), s.ad_value(594)), s.ad_value(575)), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (s.v[1885] != 0.0)) {
            s.store_ad(168, &A::limited_exp(s.ad_value(167)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (s.v[1885] != 0.0)) {
            s.store_mul(170, 545, 571);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (!(s.v[1885] != 0.0))) {
            s.store_div_from_scalar_ad(168, 1.0, A::sub(s.ad_value(575), s.ad_value(498)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (!(s.v[1885] != 0.0))) {
            s.store_mul_ad_lhs(167, A::mul(A::div(A::neg(s.ad_value(498)), s.ad_value(594)), s.ad_value(575)), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (!(s.v[1885] != 0.0))) {
            s.store_ad(168, &A::limited_exp(s.ad_value(167)));
        }

    }

    pub(super) fn stamp_reactive_block_34(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (!(s.v[1885] != 0.0))) {
            s.store_mul(170, 545, 571);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) {
            s.store_scale(594, 574, p.p925);
        }

        s.v[1886] = if ((s.v[576] - s.v[499]) < 0.001) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (s.v[1886] != 0.0)) {
            s.store_scalar(168, 1000.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (s.v[1886] != 0.0)) {
            s.store_mul_ad_lhs(167, A::mul(A::div(A::neg(s.ad_value(499)), s.ad_value(594)), s.ad_value(576)), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (s.v[1886] != 0.0)) {
            s.store_ad(168, &A::limited_exp(s.ad_value(167)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (s.v[1886] != 0.0)) {
            s.store_mul(170, 545, 572);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (!(s.v[1886] != 0.0))) {
            s.store_div_from_scalar_ad(168, 1.0, A::sub(s.ad_value(576), s.ad_value(499)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (!(s.v[1886] != 0.0))) {
            s.store_mul_ad_lhs(167, A::mul(A::div(A::neg(s.ad_value(499)), s.ad_value(594)), s.ad_value(576)), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (!(s.v[1886] != 0.0))) {
            s.store_ad(168, &A::limited_exp(s.ad_value(167)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (!(s.v[1886] != 0.0))) {
            s.store_mul(170, 545, 572);
        }

        s.v[1887] = if (p.p36 == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) {
            s.store_scalar(167, (s.v[200] * p.p76));
        }

        s.v[1888] = if (((s.v[892] <= 0.0) || (s.v[660] <= 0.0)) || (s.v[894] < 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (s.v[1888] != 0.0)) {
            s.store_scalar(173, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1888] != 0.0))) {
            s.store_div_ad_lhs(168, A::add(A::sub(A::neg(s.ad_value(204)), s.ad_value(895)), s.ad_value(219)), 167);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1888] != 0.0))) {
            s.store_scale_ad(168, A::add(s.ad_value(168), A::sqrt(A::offset(A::square(s.ad_value(168)), ((4.0 * 0.01) * 0.01)))), 0.5);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1888] != 0.0))) {
            s.store_div_ad_rhs(169, 660, A::offset(s.ad_value(168), 0.001));
        }

        s.v[1889] = if (s.v[894] != 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1888] != 0.0))) && (s.v[1889] != 0.0)) {
            s.store_mul_ad_lhs(170, A::square(s.ad_value(201)), 201);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1888] != 0.0))) && (s.v[1889] != 0.0)) {
            s.store_offset_ad(171, A::add(s.ad_value(894), A::abs(s.ad_value(170))), 0.0001);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1888] != 0.0))) && (s.v[1889] != 0.0)) {
            s.store_offset_ad(172, A::scale(A::add(A::div(s.ad_value(170), s.ad_value(171)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(170), s.ad_value(171)), A::div(s.ad_value(170), s.ad_value(171))), ((4.0 * 1e-6) * 1e-6)))), 0.5), (-1e-6));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1888] != 0.0))) && (!(s.v[1889] != 0.0))) {
            s.store_scalar(172, 1.0);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1888] != 0.0))) {
            s.store_mul_ad_lhs(173, A::mul(A::mul(A::mul(s.ad_value(892), s.ad_value(544)), s.ad_value(168)), A::limited_exp(A::neg(s.ad_value(169)))), 172);
        }

        s.v[1890] = if (((s.v[896] <= 0.0) || (s.v[661] <= 0.0)) || (s.v[898] < 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (s.v[1890] != 0.0)) {
            s.store_scalar(173, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1890] != 0.0))) {
            s.store_div_ad_lhs(168, A::add(A::sub(A::neg(s.ad_value(203)), s.ad_value(899)), s.ad_value(219)), 167);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1890] != 0.0))) {
            s.store_scale_ad(168, A::add(s.ad_value(168), A::sqrt(A::offset(A::square(s.ad_value(168)), ((4.0 * 0.01) * 0.01)))), 0.5);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1890] != 0.0))) {
            s.store_div_ad_rhs(169, 661, A::offset(s.ad_value(168), 0.001));
        }

        s.v[1891] = if (s.v[898] != 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1890] != 0.0))) && (s.v[1891] != 0.0)) {
            s.store_mul_ad_lhs(170, A::square(s.ad_value(202)), 202);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1890] != 0.0))) && (s.v[1891] != 0.0)) {
            s.store_offset_ad(171, A::add(s.ad_value(898), A::abs(s.ad_value(170))), 0.0001);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1890] != 0.0))) && (s.v[1891] != 0.0)) {
            s.store_offset_ad(172, A::scale(A::add(A::div(s.ad_value(170), s.ad_value(171)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(170), s.ad_value(171)), A::div(s.ad_value(170), s.ad_value(171))), ((4.0 * 1e-6) * 1e-6)))), 0.5), (-1e-6));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1890] != 0.0))) && (!(s.v[1891] != 0.0))) {
            s.store_scalar(172, 1.0);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1890] != 0.0))) {
            s.store_mul_ad_lhs(173, A::mul(A::mul(A::mul(s.ad_value(896), s.ad_value(543)), s.ad_value(168)), A::limited_exp(A::neg(s.ad_value(169)))), 172);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) {
            s.store_scalar(167, (s.v[200] * p.p76));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) {
            s.store_sub_ad_lhs(207, A::mul(s.ad_value(905), s.ad_value(221)), 223);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) {
            s.store_sub_ad_lhs(206, A::mul(s.ad_value(902), s.ad_value(221)), 224);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) {
            s.store_sub(169, 203, 219);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) {
            s.store_sqrt_ad(228, A::offset(A::square(s.ad_value(169)), 0.0001));
        }

        s.v[1892] = if ((s.v[892] <= 0.0) || (s.v[660] <= 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (s.v[1892] != 0.0)) {
            s.store_scalar(173, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1892] != 0.0))) {
            s.store_div_ad_lhs(168, A::add(A::sub(A::neg(s.ad_value(207)), s.ad_value(895)), s.ad_value(219)), 167);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1892] != 0.0))) {
            s.store_scale_ad(168, A::add(s.ad_value(168), A::sqrt(A::offset(A::square(s.ad_value(168)), ((4.0 * 0.01) * 0.01)))), 0.5);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1892] != 0.0))) {
            s.store_div_ad_rhs(169, 660, A::offset(s.ad_value(168), 0.001));
        }

        s.v[1893] = if (s.v[903] != 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1892] != 0.0))) && (s.v[1893] != 0.0)) {
            s.store_sub_ad_lhs(170, A::neg(s.ad_value(201)), 904);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1892] != 0.0))) && (s.v[1893] != 0.0)) {
            s.store_offset(171, 170, 0.0001);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1892] != 0.0))) && (s.v[1893] != 0.0)) {
            s.store_offset_ad(172, A::scale(A::add(A::div(s.ad_value(903), s.ad_value(171)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(903), s.ad_value(171)), A::div(s.ad_value(903), s.ad_value(171))), ((4.0 * 1e-6) * 1e-6)))), 0.5), (-1e-6));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1892] != 0.0))) && (!(s.v[1893] != 0.0))) {
            s.store_scalar(172, 1.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1892] != 0.0))) {
            s.store_mul_ad(173, A::mul(A::mul(A::mul(s.ad_value(892), s.ad_value(544)), s.ad_value(168)), A::limited_exp(A::neg(s.ad_value(169)))), A::limited_exp(s.ad_value(172)));
        }

        s.v[1894] = if ((s.v[896] <= 0.0) || (s.v[661] <= 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (s.v[1894] != 0.0)) {
            s.store_scalar(173, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1894] != 0.0))) {
            s.store_div_ad_lhs(168, A::add(A::sub(A::neg(s.ad_value(206)), s.ad_value(899)), s.ad_value(219)), 167);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1894] != 0.0))) {
            s.store_scale_ad(168, A::add(s.ad_value(168), A::sqrt(A::offset(A::square(s.ad_value(168)), ((4.0 * 0.01) * 0.01)))), 0.5);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1894] != 0.0))) {
            s.store_div_ad_rhs(169, 661, A::offset(s.ad_value(168), 0.001));
        }

        s.v[1895] = if (s.v[906] != 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1894] != 0.0))) && (s.v[1895] != 0.0)) {
            s.store_sub_ad_lhs(170, A::neg(s.ad_value(202)), 907);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1894] != 0.0))) && (s.v[1895] != 0.0)) {
            s.store_offset(171, 170, 0.0001);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1894] != 0.0))) && (s.v[1895] != 0.0)) {
            s.store_offset_ad(172, A::scale(A::add(A::div(s.ad_value(906), s.ad_value(171)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(906), s.ad_value(171)), A::div(s.ad_value(906), s.ad_value(171))), ((4.0 * 1e-6) * 1e-6)))), 0.5), (-1e-6));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1894] != 0.0))) && (!(s.v[1895] != 0.0))) {
            s.store_scalar(172, 1.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1894] != 0.0))) {
            s.store_mul_ad(173, A::mul(A::mul(A::mul(s.ad_value(896), s.ad_value(543)), s.ad_value(168)), A::limited_exp(A::neg(s.ad_value(169)))), A::limited_exp(s.ad_value(172)));
        }

        s.v[1896] = if (p.p44 == 0.0) { 1.0 } else { 0.0 };

        s.v[1897] = if ((s.v[865] <= 0.0) || (s.v[659] <= 0.0)) { 1.0 } else { 0.0 };

        s.v[1898] = if (s.v[355] > (s.v[659] / 80.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1896] != 0.0)) && (!(s.v[1897] != 0.0))) && (s.v[1898] != 0.0)) {
            s.store_div_ad_lhs(168, A::neg(s.ad_value(659)), 355);
        }

        s.v[1899] = if (p.p44 == 1.0) { 1.0 } else { 0.0 };

        s.v[1900] = if ((s.v[865] <= 0.0) || (((s.v[868] == 0.0) && (s.v[867] == 0.0)) && (s.v[659] == 0.0))) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_sub_ad(370, A::mul(s.ad_value(874), A::offset(A::scale(A::offset(s.ad_value(639), (-1.0)), p.p600), 1.0)), A::scale(s.ad_value(869), 1.0 / (s.v[184])));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_scale(167, 875, s.v[184]);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_div_ad(168, A::mul(s.ad_value(870), s.ad_value(167)), A::offset(s.ad_value(167), 1.0));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_div_from_scalar_ad(167, 1.0, A::offset(A::scale(A::add(A::mul(A::mul(s.ad_value(871), s.ad_value(367)), s.ad_value(269)), A::sqrt(A::offset(A::mul(A::mul(A::mul(s.ad_value(871), s.ad_value(367)), s.ad_value(269)), A::mul(A::mul(s.ad_value(871), s.ad_value(367)), s.ad_value(269))), ((4.0 * p.p643) * p.p643)))), 0.5), 1.0));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_add(170, 167, 872);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_scale_ad(169, A::add(A::mul(A::mul(s.ad_value(367), s.ad_value(269)), s.ad_value(170)), A::sqrt(A::offset(A::mul(A::mul(A::mul(s.ad_value(367), s.ad_value(269)), s.ad_value(170)), A::mul(A::mul(s.ad_value(367), s.ad_value(269)), s.ad_value(170))), ((4.0 * p.p644) * p.p644)))), 0.5);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_div_from_scalar_ad(170, 1.0, A::offset(A::mul(s.ad_value(873), s.ad_value(227)), 1.0));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_mul_ad_lhs(368, A::mul(s.ad_value(168), s.ad_value(169)), 170);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_add(369, 370, 368);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_sub(371, 227, 369);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_add_ad(167, A::add(s.ad_value(868), A::mul(s.ad_value(867), s.ad_value(371))), A::mul(A::mul(s.ad_value(659), s.ad_value(371)), s.ad_value(371)));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_sqrt_ad(168, A::offset(A::square(s.ad_value(167)), 1e-10));
        }

        s.v[1901] = if ((s.v[865] <= 0.0) || (((s.v[868] == 0.0) && (s.v[867] == 0.0)) && (s.v[659] == 0.0))) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_sub_ad(370, A::mul(s.ad_value(874), A::offset(A::scale(A::offset(s.ad_value(639), (-1.0)), p.p600), 1.0)), A::scale(s.ad_value(869), 1.0 / (s.v[184])));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_scale(167, 875, s.v[184]);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_div_ad(168, A::mul(s.ad_value(870), s.ad_value(167)), A::offset(s.ad_value(167), 1.0));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_div_from_scalar_ad(167, 1.0, A::offset(A::scale(A::add(A::mul(A::mul(s.ad_value(871), s.ad_value(367)), s.ad_value(269)), A::sqrt(A::offset(A::mul(A::mul(A::mul(s.ad_value(871), s.ad_value(367)), s.ad_value(269)), A::mul(A::mul(s.ad_value(871), s.ad_value(367)), s.ad_value(269))), ((4.0 * p.p643) * p.p643)))), 0.5), 1.0));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_add(170, 167, 872);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_scale_ad(169, A::add(A::mul(A::mul(s.ad_value(367), s.ad_value(269)), s.ad_value(170)), A::sqrt(A::offset(A::mul(A::mul(A::mul(s.ad_value(367), s.ad_value(269)), s.ad_value(170)), A::mul(A::mul(s.ad_value(367), s.ad_value(269)), s.ad_value(170))), ((4.0 * p.p644) * p.p644)))), 0.5);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_div_from_scalar_ad(170, 1.0, A::offset(A::mul(s.ad_value(873), s.ad_value(227)), 1.0));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_mul_ad_lhs(368, A::mul(s.ad_value(168), s.ad_value(169)), 170);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_add(369, 370, 368);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_sub(371, 227, 369);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_add_ad(167, A::add(s.ad_value(868), A::mul(s.ad_value(867), s.ad_value(371))), A::mul(A::mul(s.ad_value(659), s.ad_value(371)), s.ad_value(371)));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_sqrt_ad(168, A::offset(A::square(s.ad_value(167)), 1e-10));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) {
            s.store_scale_ad(167, A::add(s.ad_value(878), A::scale(s.ad_value(877), s.v[184])), 1.0 / (s.v[184]));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) {
            s.store_mul_ad_rhs(378, 880, A::offset(A::scale(A::offset(s.ad_value(639), (-1.0)), p.p666), 1.0));
        }

        s.v[1902] = if (s.v[211] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (s.v[1902] != 0.0)) {
            s.store_sub(168, 378, 499);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1902] != 0.0))) {
            s.store_sub(168, 378, 498);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) {
            s.store_offset(169, 881, (-1.0));
        }

        s.v[1903] = if (s.v[168] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (s.v[1903] != 0.0)) {
            s.store_mul_ad(170, A::neg(s.ad_value(879)), A::pow(s.ad_value(168), s.ad_value(169)));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1903] != 0.0))) {
            s.store_scalar(170, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_rhs(810, 810, A::mul(s.ad_value(813), A::offset(s.ad_value(639), (-1.0))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_rhs(816, 816, A::mul(s.ad_value(814), A::offset(s.ad_value(639), (-1.0))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_rhs(819, 819, A::mul(s.ad_value(815), A::offset(s.ad_value(639), (-1.0))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_rhs(884, 884, A::mul(s.ad_value(886), A::offset(s.ad_value(639), (-1.0))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_rhs(882, 882, A::mul(s.ad_value(887), A::offset(s.ad_value(639), (-1.0))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_rhs(888, 888, A::mul(s.ad_value(891), A::offset(s.ad_value(639), (-1.0))));
        }

        s.v[1904] = if ((p.p37 != 0.0) || (p.p38 != 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) {
            s.store_mul_ad_rhs(469, 269, A::add(A::add(A::sub(s.ad_value(213), s.ad_value(254)), s.ad_value(400)), s.ad_value(320)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) {
            s.store_sqrt_ad(168, A::offset(A::square(s.ad_value(469)), 0.0001));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) {
            s.store_scaled_sub(471, 168, 469, 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) {
            s.store_scaled_add(470, 469, 168, 0.5);
        }

        s.v[1905] = if (p.p38 != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_scale(168, 469, 1.0 / (p.p671));
        }

        s.v[1906] = if (p.p696 != 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) && (s.v[1906] != 0.0)) {
            s.store_sub_from_scalar_ad(167, 1.0, A::scale(s.ad_value(471), 1.0 / (p.p696)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) && (!(s.v[1906] != 0.0))) {
            s.store_scalar(167, 1.0);
        }

        s.v[1907] = if (s.v[167] < 0.01) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) && (s.v[1907] != 0.0)) {
            s.store_scalar(167, 0.01);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_scale(168, 492, ((((s.v[184] * s.v[183]) / p.p1373) + (p.p1381 / p.p2)) * p.p700));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_scalar(169, (p.p701 * p.p76));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_div_ad_lhs(170, A::mul(s.ad_value(169), A::sub(s.ad_value(882), A::mul(s.ad_value(883), s.ad_value(471)))), 167);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_scaled_sub(168, 469, 809, 1.0 / (p.p671));
        }

        s.v[1908] = if (p.p697 != 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) && (s.v[1908] != 0.0)) {
            s.store_sub_from_scalar_ad(167, 1.0, A::scale(s.ad_value(470), 1.0 / (p.p697)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) && (!(s.v[1908] != 0.0))) {
            s.store_scalar(167, 1.0);
        }

        s.v[1909] = if (s.v[167] < 0.01) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) && (s.v[1909] != 0.0)) {
            s.store_scalar(167, 0.01);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_scale(168, 492, ((((s.v[184] * s.v[183]) / p.p1373) + (p.p1381 / p.p2)) * p.p698));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_scalar(169, (p.p699 * p.p76));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_div_ad_lhs(170, A::mul(s.ad_value(169), A::sub(s.ad_value(884), A::mul(s.ad_value(885), s.ad_value(470)))), 167);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_offset_ad(478, A::mul(s.ad_value(212), s.ad_value(269)), p.p1383);
        }

        s.v[1910] = if (((((p.p43 != 0.0) && (1.0 != 0.0)) && (!((p.p40 != 0.0) && (!(1.0 != 0.0))))) && (p.p45 == 1.0)) && (p.p1380 > 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_mul_ad_rhs(208, 379, A::voltage(ctx, &nodes, Some(8), Some(11)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_sub(167, 208, 478);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_sqrt_ad(168, A::offset(A::square(s.ad_value(167)), 0.0001));
        }

    }

    pub(super) fn stamp_reactive_block_35(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_scale_ad(209, A::offset(A::sub(s.ad_value(168), s.ad_value(167)), (-0.01)), 0.5);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_scalar(178, (if (p.p30 == 1.0) { p.p702 } else { p.p703 }));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_scalar(179, (if (p.p30 == 1.0) { p.p704 } else { p.p705 }));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_mul(169, 208, 209);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_sub_ad_lhs(170, A::mul(s.ad_value(888), s.ad_value(890)), 889);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_mul(171, 889, 890);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_mul_ad(172, A::scale(A::neg(s.ad_value(179)), p.p76), A::sub(A::add(s.ad_value(888), A::mul(s.ad_value(170), s.ad_value(209))), A::mul(A::mul(s.ad_value(171), s.ad_value(209)), s.ad_value(209))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_ad(173, &A::limited_exp(s.ad_value(172)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_mul_ad_lhs(178, A::scale(s.ad_value(178), p.p1380), 492);
        }

        s.v[1911] = if (p.p37 != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_sub_ad_rhs(168, 810, A::mul(s.ad_value(811), s.ad_value(470)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_offset_ad(169, A::mul(s.ad_value(812), s.ad_value(470)), 1.0);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_mul_ad_lhs(170, A::scale(s.ad_value(168), s.v[488]), 169);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_mul_ad(171, A::mul(A::mul(s.ad_value(253), s.ad_value(269)), A::add(s.ad_value(400), s.ad_value(320))), A::limited_exp(s.ad_value(170)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_offset_ad(472, A::sqrt(A::offset(A::square(s.ad_value(315)), 0.01)), (-0.1));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_scale(168, 472, s.v[823]);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_limited_exp_ad(482, A::neg(s.ad_value(168)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_offset_ad(170, A::offset(A::add(s.ad_value(168), s.ad_value(482)), (-1.0)), 0.0001);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_offset_ad(171, A::sub_from_scalar(1.0, A::mul(A::offset(s.ad_value(168), 1.0), s.ad_value(482))), 0.0001);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_offset_ad(172, A::square(s.ad_value(168)), 0.0002);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_sub(169, 203, 219);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_sqrt_ad(228, A::offset(A::square(s.ad_value(169)), 0.0001));
        }

        s.v[1913] = if (p.p1295 == 1.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) && (s.v[1913] != 0.0)) {
            s.store_scale_ad(168, A::add(A::sub(s.ad_value(816), A::mul(s.ad_value(817), s.ad_value(228))), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(816), A::mul(s.ad_value(817), s.ad_value(228))), A::sub(s.ad_value(816), A::mul(s.ad_value(817), s.ad_value(228)))), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[1914] = if (s.v[818] < 0.01) { 1.0 } else { 0.0 };

        if (((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) && (s.v[1913] != 0.0)) && (s.v[1914] != 0.0)) {
            s.store_scalar(818, 0.01);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) && (!(s.v[1913] != 0.0))) {
            s.store_sub_ad_rhs(168, 816, A::mul(s.ad_value(817), s.ad_value(228)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_offset_ad(169, A::mul(s.ad_value(818), s.ad_value(228)), 1.0);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_mul_ad_lhs(170, A::mul(s.ad_value(491), s.ad_value(168)), 169);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_sub(169, 204, 219);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_sqrt_ad(229, A::offset(A::square(s.ad_value(169)), 0.0001));
        }

        s.v[1915] = if (p.p1295 == 1.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) && (s.v[1915] != 0.0)) {
            s.store_scale_ad(168, A::add(A::sub(s.ad_value(819), A::mul(s.ad_value(820), s.ad_value(229))), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(819), A::mul(s.ad_value(820), s.ad_value(229))), A::sub(s.ad_value(819), A::mul(s.ad_value(820), s.ad_value(229)))), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[1916] = if (s.v[821] < 0.01) { 1.0 } else { 0.0 };

        if (((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) && (s.v[1915] != 0.0)) && (s.v[1916] != 0.0)) {
            s.store_scalar(821, 0.01);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) && (!(s.v[1915] != 0.0))) {
            s.store_sub_ad_rhs(168, 819, A::mul(s.ad_value(820), s.ad_value(229)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_offset_ad(169, A::mul(s.ad_value(821), s.ad_value(229)), 1.0);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_mul_ad_lhs(170, A::mul(s.ad_value(491), s.ad_value(168)), 169);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(502, 666, 463);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(505, 667, 494);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale(508, 671, (s.v[189] * p.p2));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(503, ((0.1) as f64).powf((-p.p913)));
        }

        s.v[1917] = if (p.p913 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1917] != 0.0)) {
            s.store_scalar(504, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1917] != 0.0))) {
            s.store_scale_ad(504, A::sub_from_scalar(1.0, A::scale(s.ad_value(503), ((0.05 * p.p913) * (1.0 + p.p913)))), (1.0 / (1.0 - p.p913)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(506, ((0.1) as f64).powf((-p.p915)));
        }

        s.v[1918] = if (p.p915 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1918] != 0.0)) {
            s.store_scalar(507, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1918] != 0.0))) {
            s.store_scale_ad(507, A::sub_from_scalar(1.0, A::scale(s.ad_value(506), ((0.05 * p.p915) * (1.0 + p.p915)))), (1.0 / (1.0 - p.p915)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(509, ((0.1) as f64).powf((-p.p917)));
        }

        s.v[1919] = if (p.p917 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1919] != 0.0)) {
            s.store_scalar(510, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1919] != 0.0))) {
            s.store_scale_ad(510, A::sub_from_scalar(1.0, A::scale(s.ad_value(509), ((0.05 * p.p917) * (1.0 + p.p917)))), (1.0 / (1.0 - p.p917)));
        }

        s.v[1920] = if (s.v[502] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1920] != 0.0)) {
            s.store_div(168, 498, 672);
        }

        s.v[1921] = if (s.v[168] < 0.9) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1920] != 0.0)) && (s.v[1921] != 0.0)) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.v[1922] = if (p.p913 != 1.0) { 1.0 } else { 0.0 };

        s.v[1923] = if (p.p913 == 0.5) { 1.0 } else { 0.0 };

        if (((((!(s.v[1620] != 0.0)) && (s.v[1920] != 0.0)) && (s.v[1921] != 0.0)) && (s.v[1922] != 0.0)) && (s.v[1923] != 0.0)) {
            s.store_div_from_scalar_ad(501, 1.0, A::sqrt(s.ad_value(500)));
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1920] != 0.0)) && (s.v[1921] != 0.0)) && (s.v[1922] != 0.0)) && (!(s.v[1923] != 0.0))) {
            s.store_limited_exp_ad(501, A::scale(A::ln(s.ad_value(500)), (-p.p913)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1920] != 0.0)) && (s.v[1921] != 0.0)) && (s.v[1922] != 0.0)) {
            s.store_scale_ad(521, A::mul(A::mul(s.ad_value(672), s.ad_value(502)), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501)))), 1.0 / ((1.0 - p.p913)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1920] != 0.0)) && (s.v[1921] != 0.0)) && (!(s.v[1922] != 0.0))) {
            s.store_mul_ad(521, A::mul(s.ad_value(672), s.ad_value(502)), A::neg(A::ln(s.ad_value(500))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1920] != 0.0)) && (!(s.v[1921] != 0.0))) {
            s.store_mul_ad(169, A::mul(s.ad_value(503), A::offset(s.ad_value(168), (-1.0))), A::offset(A::scale(A::offset(s.ad_value(168), (-1.0)), (5.0 * p.p913)), (1.0 + p.p913)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1920] != 0.0)) && (!(s.v[1921] != 0.0))) {
            s.store_mul_ad(521, A::mul(s.ad_value(672), s.ad_value(502)), A::add(s.ad_value(169), s.ad_value(504)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1920] != 0.0))) {
            s.store_scalar(521, 0.0);
        }

        s.v[1924] = if (s.v[505] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1924] != 0.0)) {
            s.store_div(168, 498, 673);
        }

        s.v[1925] = if (s.v[168] < 0.9) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1924] != 0.0)) && (s.v[1925] != 0.0)) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.v[1926] = if (p.p915 != 1.0) { 1.0 } else { 0.0 };

        s.v[1927] = if (p.p915 == 0.5) { 1.0 } else { 0.0 };

        if (((((!(s.v[1620] != 0.0)) && (s.v[1924] != 0.0)) && (s.v[1925] != 0.0)) && (s.v[1926] != 0.0)) && (s.v[1927] != 0.0)) {
            s.store_div_from_scalar_ad(501, 1.0, A::sqrt(s.ad_value(500)));
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1924] != 0.0)) && (s.v[1925] != 0.0)) && (s.v[1926] != 0.0)) && (!(s.v[1927] != 0.0))) {
            s.store_limited_exp_ad(501, A::scale(A::ln(s.ad_value(500)), (-p.p915)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1924] != 0.0)) && (s.v[1925] != 0.0)) && (s.v[1926] != 0.0)) {
            s.store_scale_ad(522, A::mul(A::mul(s.ad_value(673), s.ad_value(505)), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501)))), 1.0 / ((1.0 - p.p915)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1924] != 0.0)) && (s.v[1925] != 0.0)) && (!(s.v[1926] != 0.0))) {
            s.store_mul_ad(522, A::mul(s.ad_value(673), s.ad_value(505)), A::neg(A::ln(s.ad_value(500))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1924] != 0.0)) && (!(s.v[1925] != 0.0))) {
            s.store_mul_ad(169, A::mul(s.ad_value(506), A::offset(s.ad_value(168), (-1.0))), A::offset(A::scale(A::offset(s.ad_value(168), (-1.0)), (5.0 * p.p915)), (1.0 + p.p915)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1924] != 0.0)) && (!(s.v[1925] != 0.0))) {
            s.store_mul_ad(522, A::mul(s.ad_value(673), s.ad_value(505)), A::add(s.ad_value(169), s.ad_value(507)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1924] != 0.0))) {
            s.store_scalar(522, 0.0);
        }

        s.v[1928] = if (s.v[508] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1928] != 0.0)) {
            s.store_div(168, 498, 674);
        }

        s.v[1929] = if (s.v[168] < 0.9) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1928] != 0.0)) && (s.v[1929] != 0.0)) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.v[1930] = if (p.p917 != 1.0) { 1.0 } else { 0.0 };

        s.v[1931] = if (p.p917 == 0.5) { 1.0 } else { 0.0 };

        if (((((!(s.v[1620] != 0.0)) && (s.v[1928] != 0.0)) && (s.v[1929] != 0.0)) && (s.v[1930] != 0.0)) && (s.v[1931] != 0.0)) {
            s.store_div_from_scalar_ad(501, 1.0, A::sqrt(s.ad_value(500)));
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1928] != 0.0)) && (s.v[1929] != 0.0)) && (s.v[1930] != 0.0)) && (!(s.v[1931] != 0.0))) {
            s.store_limited_exp_ad(501, A::scale(A::ln(s.ad_value(500)), (-p.p917)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1928] != 0.0)) && (s.v[1929] != 0.0)) && (s.v[1930] != 0.0)) {
            s.store_scale_ad(523, A::mul(A::mul(s.ad_value(674), s.ad_value(508)), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501)))), 1.0 / ((1.0 - p.p917)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1928] != 0.0)) && (s.v[1929] != 0.0)) && (!(s.v[1930] != 0.0))) {
            s.store_mul_ad(523, A::mul(s.ad_value(674), s.ad_value(508)), A::neg(A::ln(s.ad_value(500))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1928] != 0.0)) && (!(s.v[1929] != 0.0))) {
            s.store_mul_ad(169, A::mul(s.ad_value(509), A::offset(s.ad_value(168), (-1.0))), A::offset(A::scale(A::offset(s.ad_value(168), (-1.0)), (5.0 * p.p917)), (1.0 + p.p917)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1928] != 0.0)) && (!(s.v[1929] != 0.0))) {
            s.store_mul_ad(523, A::mul(s.ad_value(674), s.ad_value(508)), A::add(s.ad_value(169), s.ad_value(510)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1928] != 0.0))) {
            s.store_scalar(523, 0.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale(524, 533, (p.p919 * p.p2));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_lhs(520, A::add(A::add(s.ad_value(521), s.ad_value(522)), s.ad_value(523)), 524);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(511, 669, 464);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(514, 670, 495);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale(517, 668, (s.v[189] * p.p2));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(512, ((0.1) as f64).powf((-p.p914)));
        }

        s.v[1932] = if (p.p914 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1932] != 0.0)) {
            s.store_scalar(513, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1932] != 0.0))) {
            s.store_scale_ad(513, A::sub_from_scalar(1.0, A::scale(s.ad_value(512), ((0.05 * p.p914) * (1.0 + p.p914)))), (1.0 / (1.0 - p.p914)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(515, ((0.1) as f64).powf((-p.p916)));
        }

        s.v[1933] = if (p.p916 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1933] != 0.0)) {
            s.store_scalar(516, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1933] != 0.0))) {
            s.store_scale_ad(516, A::sub_from_scalar(1.0, A::scale(s.ad_value(515), ((0.05 * p.p916) * (1.0 + p.p916)))), (1.0 / (1.0 - p.p916)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(518, ((0.1) as f64).powf((-p.p918)));
        }

        s.v[1934] = if (p.p918 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1934] != 0.0)) {
            s.store_scalar(519, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1934] != 0.0))) {
            s.store_scale_ad(519, A::sub_from_scalar(1.0, A::scale(s.ad_value(518), ((0.05 * p.p918) * (1.0 + p.p918)))), (1.0 / (1.0 - p.p918)));
        }

        s.v[1935] = if (s.v[511] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1935] != 0.0)) {
            s.store_div(168, 499, 675);
        }

        s.v[1936] = if (s.v[168] < 0.9) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1935] != 0.0)) && (s.v[1936] != 0.0)) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.v[1937] = if (p.p914 != 1.0) { 1.0 } else { 0.0 };

        s.v[1938] = if (p.p914 == 0.5) { 1.0 } else { 0.0 };

        if (((((!(s.v[1620] != 0.0)) && (s.v[1935] != 0.0)) && (s.v[1936] != 0.0)) && (s.v[1937] != 0.0)) && (s.v[1938] != 0.0)) {
            s.store_div_from_scalar_ad(501, 1.0, A::sqrt(s.ad_value(500)));
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1935] != 0.0)) && (s.v[1936] != 0.0)) && (s.v[1937] != 0.0)) && (!(s.v[1938] != 0.0))) {
            s.store_limited_exp_ad(501, A::scale(A::ln(s.ad_value(500)), (-p.p914)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1935] != 0.0)) && (s.v[1936] != 0.0)) && (s.v[1937] != 0.0)) {
            s.store_scale_ad(526, A::mul(A::mul(s.ad_value(675), s.ad_value(511)), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501)))), 1.0 / ((1.0 - p.p914)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1935] != 0.0)) && (s.v[1936] != 0.0)) && (!(s.v[1937] != 0.0))) {
            s.store_mul_ad(526, A::mul(s.ad_value(675), s.ad_value(511)), A::neg(A::ln(s.ad_value(500))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1935] != 0.0)) && (!(s.v[1936] != 0.0))) {
            s.store_mul_ad(169, A::mul(s.ad_value(512), A::offset(s.ad_value(168), (-1.0))), A::offset(A::scale(A::offset(s.ad_value(168), (-1.0)), (5.0 * p.p914)), (1.0 + p.p914)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1935] != 0.0)) && (!(s.v[1936] != 0.0))) {
            s.store_mul_ad(526, A::mul(s.ad_value(675), s.ad_value(511)), A::add(s.ad_value(169), s.ad_value(513)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1935] != 0.0))) {
            s.store_scalar(526, 0.0);
        }

        s.v[1939] = if (s.v[514] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1939] != 0.0)) {
            s.store_div(168, 499, 676);
        }

        s.v[1940] = if (s.v[168] < 0.9) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1939] != 0.0)) && (s.v[1940] != 0.0)) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.v[1941] = if (p.p916 != 1.0) { 1.0 } else { 0.0 };

        s.v[1942] = if (p.p916 == 0.5) { 1.0 } else { 0.0 };

        if (((((!(s.v[1620] != 0.0)) && (s.v[1939] != 0.0)) && (s.v[1940] != 0.0)) && (s.v[1941] != 0.0)) && (s.v[1942] != 0.0)) {
            s.store_div_from_scalar_ad(501, 1.0, A::sqrt(s.ad_value(500)));
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1939] != 0.0)) && (s.v[1940] != 0.0)) && (s.v[1941] != 0.0)) && (!(s.v[1942] != 0.0))) {
            s.store_limited_exp_ad(501, A::scale(A::ln(s.ad_value(500)), (-p.p916)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1939] != 0.0)) && (s.v[1940] != 0.0)) && (s.v[1941] != 0.0)) {
            s.store_scale_ad(527, A::mul(A::mul(s.ad_value(676), s.ad_value(514)), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501)))), 1.0 / ((1.0 - p.p916)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1939] != 0.0)) && (s.v[1940] != 0.0)) && (!(s.v[1941] != 0.0))) {
            s.store_mul_ad(527, A::mul(s.ad_value(676), s.ad_value(514)), A::neg(A::ln(s.ad_value(500))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1939] != 0.0)) && (!(s.v[1940] != 0.0))) {
            s.store_mul_ad(169, A::mul(s.ad_value(515), A::offset(s.ad_value(168), (-1.0))), A::offset(A::scale(A::offset(s.ad_value(168), (-1.0)), (5.0 * p.p916)), (1.0 + p.p916)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1939] != 0.0)) && (!(s.v[1940] != 0.0))) {
            s.store_mul_ad(527, A::mul(s.ad_value(676), s.ad_value(514)), A::add(s.ad_value(169), s.ad_value(516)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1939] != 0.0))) {
            s.store_scalar(527, 0.0);
        }

        s.v[1943] = if (s.v[517] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1943] != 0.0)) {
            s.store_div(168, 499, 677);
        }

        s.v[1944] = if (s.v[168] < 0.9) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1943] != 0.0)) && (s.v[1944] != 0.0)) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.v[1945] = if (p.p918 != 1.0) { 1.0 } else { 0.0 };

        s.v[1946] = if (p.p918 == 0.5) { 1.0 } else { 0.0 };

        if (((((!(s.v[1620] != 0.0)) && (s.v[1943] != 0.0)) && (s.v[1944] != 0.0)) && (s.v[1945] != 0.0)) && (s.v[1946] != 0.0)) {
            s.store_div_from_scalar_ad(501, 1.0, A::sqrt(s.ad_value(500)));
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1943] != 0.0)) && (s.v[1944] != 0.0)) && (s.v[1945] != 0.0)) && (!(s.v[1946] != 0.0))) {
            s.store_limited_exp_ad(501, A::scale(A::ln(s.ad_value(500)), (-p.p918)));
        }

    }
}
