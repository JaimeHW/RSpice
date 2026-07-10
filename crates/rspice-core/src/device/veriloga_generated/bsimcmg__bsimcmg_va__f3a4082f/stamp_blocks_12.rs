#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_61(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1367] && (!s.b[1368])) && s.b[1375]) {s.store_add_scaled_product_indices(316, 817, 1.0, 843, 232, 1.0);}
        s.b[1376] = (p.p75 != 0.0);s.store_scalar(1376, if s.b[1376] { 1.0 } else { 0.0 });
        if s.b[1376] {s.store_add_scaled_inputs3_mixed_iai(296, 673, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p164 * 0.5), s.ad_value(673), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p164, s.ad_value(673), -1.0), (-1e-6))), 1.0, s.ad_value(673), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 673, (-1.0));}
        if (!s.b[1376]) {
            s.store_mul_mixed_ia(296, 673, {
                            if (!(((1.0 + (p.p164 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p164, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p164, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (p.p164 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p164, ((1.0) + ((-1e-6)))))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1377] = (p.p67 == 1.0);s.store_scalar(1377, if s.b[1377] { 1.0 } else { 0.0 });s.b[1378] = (p.p75 != 0.0);s.store_scalar(1378, if s.b[1378] { 1.0 } else { 0.0 });
        if (s.b[1377] && s.b[1378]) {s.store_add_scaled_inputs3_mixed_iai(297, 675, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p165 * 0.5), s.ad_value(675), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p165, s.ad_value(675), -1.0), (-1e-6))), 1.0, s.ad_value(675), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 675, (-1.0));}
        if (s.b[1377] && (!s.b[1378])) {
            s.store_mul_mixed_ia(297, 675, {
                            if (!(((1.0 + (p.p165 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p165, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p165, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (p.p165 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p165, ((1.0) + ((-1e-6)))))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1379] = (p.p75 != 0.0);s.store_scalar(1379, if s.b[1379] { 1.0 } else { 0.0 });
        if s.b[1379] {s.store_add_scaled_inputs3_mixed_iai(298, 677, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p166 * 0.5), s.ad_value(677), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p166, s.ad_value(677), -1.0), (-1e-6))), 1.0, s.ad_value(677), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 677, (-1.0));}
        if (!s.b[1379]) {
            s.store_mul_mixed_ia(298, 677, {
                            if (!(((1.0 + (p.p166 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p166, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p166, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (p.p166 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p166, ((1.0) + ((-1e-6)))))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1380] = (p.p75 != 0.0);s.store_scalar(1380, if s.b[1380] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_62(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1380] {s.store_add_scaled_inputs4_offset_mixed_iaai(322, 707, 1.0, A::add_scaled_product(s.ad_value(707), 1.0, s.ad_value(842), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(707), 1.0, s.ad_value(842), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(707), (-(4.0 * 1e-6)))), 0.5, 707, (-1.0), (0.5 * (-1e-6)));}
        if (!s.b[1380]) {
            s.store_mul_mixed_ia(322, 707, {
                            if (!(((1.0 + (s.v[842] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul(s.ad_value(842), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(842), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (s.v[842] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(842), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1381] = (p.p75 != 0.0);s.store_scalar(1381, if s.b[1381] { 1.0 } else { 0.0 });
        if s.b[1381] {s.store_offset_add_scaled_inputs(299, A::scale_offset(s.ad_value(232), p.p923, (((-(-p.p917))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p923, (((-(-p.p917))) + ((-1e-6)))), (-((4.0 * (-p.p917)) * 1e-6))), 0.5, (((-p.p917)) + (p.p917)));}
        if (!s.b[1381]) {
            s.store_scale_ad(299, {
                if (!(((1.0 + (p.p923 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p923 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p917);
        }
        s.b[1382] = (p.p66 != 0.0);s.store_scalar(1382, if s.b[1382] { 1.0 } else { 0.0 });s.b[1383] = (p.p75 != 0.0);s.store_scalar(1383, if s.b[1383] { 1.0 } else { 0.0 });
        if (s.b[1382] && s.b[1383]) {s.store_offset_add_scaled_inputs(300, A::scale_offset(s.ad_value(232), p.p923, (((-(-p.p918))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p923, (((-(-p.p918))) + ((-1e-6)))), (-((4.0 * (-p.p918)) * 1e-6))), 0.5, (((-p.p918)) + (p.p918)));}
        if (s.b[1382] && (!s.b[1383])) {
            s.store_scale_ad(300, {
                if (!(((1.0 + (p.p923 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p923 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p918);
        }
        s.b[1384] = (p.p75 != 0.0);s.store_scalar(1384, if s.b[1384] { 1.0 } else { 0.0 });
        if s.b[1384] {s.store_offset_add_scaled_inputs(301, A::scale_offset(s.ad_value(232), p.p924, (((-(-p.p919))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p924, (((-(-p.p919))) + ((-1e-6)))), (-((4.0 * (-p.p919)) * 1e-6))), 0.5, (((-p.p919)) + (p.p919)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_63(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1384]) {
            s.store_scale_ad(301, {
                if (!(((1.0 + (p.p924 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p924 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p919);
        }
        s.b[1385] = (p.p66 != 0.0);s.store_scalar(1385, if s.b[1385] { 1.0 } else { 0.0 });s.b[1386] = (p.p75 != 0.0);s.store_scalar(1386, if s.b[1386] { 1.0 } else { 0.0 });
        if (s.b[1385] && s.b[1386]) {s.store_offset_add_scaled_inputs(302, A::scale_offset(s.ad_value(232), p.p924, (((-(-p.p920))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p924, (((-(-p.p920))) + ((-1e-6)))), (-((4.0 * (-p.p920)) * 1e-6))), 0.5, (((-p.p920)) + (p.p920)));}
        if (s.b[1385] && (!s.b[1386])) {
            s.store_scale_ad(302, {
                if (!(((1.0 + (p.p924 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p924 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p920);
        }
        s.b[1387] = (p.p75 != 0.0);s.store_scalar(1387, if s.b[1387] { 1.0 } else { 0.0 });
        if s.b[1387] {s.store_add_scaled_inputs4_offset_mixed_iaai(257, 700, 1.0, A::add_scaled_product(s.ad_value(700), 1.0, s.ad_value(848), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(700), 1.0, s.ad_value(848), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(700), (-(4.0 * 1e-6)))), 0.5, 700, (-1.0), (0.5 * (-1e-6)));}
        if (!s.b[1387]) {
            s.store_mul_mixed_ia(257, 700, {
                            if (!(((1.0 + ((-s.v[848]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + ((-s.v[848]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1388] = (p.p66 != 0.0);s.store_scalar(1388, if s.b[1388] { 1.0 } else { 0.0 });s.b[1389] = (p.p75 != 0.0);s.store_scalar(1389, if s.b[1389] { 1.0 } else { 0.0 });
        if (s.b[1388] && s.b[1389]) {s.store_add_scaled_inputs4_offset_mixed_iaai(258, 701, 1.0, A::add_scaled_product(s.ad_value(701), 1.0, s.ad_value(848), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(701), 1.0, s.ad_value(848), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(701), (-(4.0 * 1e-6)))), 0.5, 701, (-1.0), (0.5 * (-1e-6)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_64(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1388] && (!s.b[1389])) {
            s.store_mul_mixed_ia(258, 701, {
                            if (!(((1.0 + ((-s.v[848]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + ((-s.v[848]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.store_mul_exp_mixed_ia(248, 779, A::mul(s.ad_value(860), s.ad_value(418)));
        s.store_mul_scale_offset_mixed_ia(249, 785, {
            if (!(((1.0 + (s.v[789] * s.v[230])) - 0.01) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::offset(A::mul(s.ad_value(789), s.ad_value(230)), ((1.0) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(789), s.ad_value(230)), ((1.0) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((1.0 + (s.v[789] * s.v[230])) - 0.01) < ((-10000.0) * 0.001)) {
                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(789), s.ad_value(230)), ((1.0) + ((-0.01))), 1.0)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 1.0, 0.01);s.store_add_scaled_product_indices(236, 683, 1.0, 684, 232, 1.0);s.store_add_scaled_inputs4_offset_mixed_iaai(237, 685, 1.0, A::add_scaled_product(s.ad_value(685), 1.0, s.ad_value(686), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(685), 1.0, s.ad_value(686), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(685), (-(4.0 * 1e-6)))), 0.5, 685, (-1.0), (0.5 * (-1e-6)));s.store_add_scaled_inputs4_offset_mixed_iaai(238, 687, 1.0, A::add_scaled_product(s.ad_value(687), 1.0, s.ad_value(688), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(687), 1.0, s.ad_value(688), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(687), (-(4.0 * 1e-6)))), 0.5, 687, (-1.0), (0.5 * (-1e-6)));s.store_add_scaled_inputs4_offset_mixed_iaai(239, 690, 1.0, A::add_scaled_product(s.ad_value(690), 1.0, s.ad_value(691), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(690), 1.0, s.ad_value(691), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(690), (-(4.0 * 1e-6)))), 0.5, 690, (-1.0), (0.5 * (-1e-6)));s.store_add_scaled_product_indices(240, 692, 1.0, 693, 232, 1.0);s.store_add_scaled_product_indices(241, 798, 1.0, 800, 232, 1.0);s.store_add_scaled_product_indices(242, 799, 1.0, 801, 232, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_65(
        s: &mut ReactiveScratch,
    ) {
        s.store_add_scaled_inputs4_offset_mixed_iaai(293, 871, 1.0, A::add_scaled_product(s.ad_value(871), 1.0, s.ad_value(872), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(871), 1.0, s.ad_value(872), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(871), (-(4.0 * 1e-6)))), 0.5, 871, (-1.0), (0.5 * (-1e-6)));s.store_add_scaled_product_indices(294, 867, 1.0, 868, 232, 1.0);s.store_add_scaled_product_indices(295, 869, 1.0, 870, 232, 1.0);s.store_add_scaled_inputs4_offset_mixed_iaai(243, 721, 1.0, A::add_scaled_product(s.ad_value(721), 1.0, s.ad_value(722), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(721), 1.0, s.ad_value(722), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(721), (-(4.0 * 1e-6)))), 0.5, 721, (-1.0), (0.5 * (-1e-6)));s.store_add_scaled_inputs4_offset_mixed_iaai(244, 727, 1.0, A::add_scaled_product(s.ad_value(727), 1.0, s.ad_value(728), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(727), 1.0, s.ad_value(728), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(727), (-(4.0 * 1e-6)))), 0.5, 727, (-1.0), (0.5 * (-1e-6)));s.store_add_scaled_inputs4_offset_mixed_iaai(245, 732, 1.0, A::add_scaled_product(s.ad_value(732), 1.0, s.ad_value(733), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(732), 1.0, s.ad_value(733), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(732), (-(4.0 * 1e-6)))), 0.5, 732, (-1.0), (0.5 * (-1e-6)));s.store_add_scaled_inputs4_offset_mixed_iaai(246, 737, 1.0, A::add_scaled_product(s.ad_value(737), 1.0, s.ad_value(738), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(737), 1.0, s.ad_value(738), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(737), (-(4.0 * 1e-6)))), 0.5, 737, (-1.0), (0.5 * (-1e-6)));s.store_add_scaled_inputs4_offset_mixed_iaai(247, 743, 1.0, A::add_scaled_product(s.ad_value(743), 1.0, s.ad_value(744), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(743), 1.0, s.ad_value(744), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(743), (-(4.0 * 1e-6)))), 0.5, 743, (-1.0), (0.5 * (-1e-6)));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_66(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_mul_mixed_ia(252, 748, {
                    if (!(((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                    } else {
                        {
                            if (((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                });
        s.store_mul_mixed_ia(250, 762, {
                    if (!(((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                        A::add_scaled_inputs(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                    } else {
                        {
                            if (((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                });
        s.store_add_scaled_inputs3_mixed_iai(259, 775, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p1437 * 0.5), s.ad_value(775), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1437, s.ad_value(775), -1.0), (-1e-6))), 1.0, s.ad_value(775), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 775, (-1.0));s.store_add_scaled_inputs3_mixed_iai(260, 776, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p1438 * 0.5), s.ad_value(776), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1438, s.ad_value(776), -1.0), (-1e-6))), 1.0, s.ad_value(776), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 776, (-1.0));s.store_add_scaled_inputs3_mixed_iai(261, 777, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p1439 * 0.5), s.ad_value(777), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1439, s.ad_value(777), -1.0), (-1e-25))), 1.0, s.ad_value(777), (-(4.0 * 1e-25)))), 0.5, ((-1e-25) * 0.5)), 1.0, 777, (-1.0));s.store_add_scaled_inputs3_mixed_iai(262, 778, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p1440 * 0.5), s.ad_value(778), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1440, s.ad_value(778), -1.0), (-1e-20))), 1.0, s.ad_value(778), (-(4.0 * 1e-20)))), 0.5, ((-1e-20) * 0.5)), 1.0, 778, (-1.0));s.b[1390] = (p.p61 != 0.0);s.store_scalar(1390, if s.b[1390] { 1.0 } else { 0.0 });s.b[1391] = (p.p75 != 0.0);s.store_scalar(1391, if s.b[1391] { 1.0 } else { 0.0 });
        if (s.b[1390] && s.b[1391]) {s.store_offset_add_scaled_inputs(263, A::scale_offset(s.ad_value(232), p.p1721, (((-(-p.p1584))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1721, (((-(-p.p1584))) + ((-1e-6)))), (-((4.0 * (-p.p1584)) * 1e-6))), 0.5, (((-p.p1584)) + (p.p1584)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_67(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1390] && (!s.b[1391])) {
            s.store_scale_ad(263, {
                if (!(((1.0 + (p.p1721 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1721 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1584);
        }
        s.b[1392] = (p.p75 != 0.0);s.store_scalar(1392, if s.b[1392] { 1.0 } else { 0.0 });
        if (s.b[1390] && s.b[1392]) {s.store_offset_add_scaled_inputs(266, A::scale_offset(s.ad_value(232), p.p1721, (((-(-p.p1585))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1721, (((-(-p.p1585))) + ((-1e-6)))), (-((4.0 * (-p.p1585)) * 1e-6))), 0.5, (((-p.p1585)) + (p.p1585)));}
        if (s.b[1390] && (!s.b[1392])) {
            s.store_scale_ad(266, {
                if (!(((1.0 + (p.p1721 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1721 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1585);
        }
        s.b[1393] = (p.p75 != 0.0);s.store_scalar(1393, if s.b[1393] { 1.0 } else { 0.0 });
        if (s.b[1390] && s.b[1393]) {s.store_offset_add_scaled_inputs(264, A::scale_offset(s.ad_value(232), p.p1722, (((-(-p.p1586))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1722, (((-(-p.p1586))) + ((-1e-6)))), (-((4.0 * (-p.p1586)) * 1e-6))), 0.5, (((-p.p1586)) + (p.p1586)));}
        if (s.b[1390] && (!s.b[1393])) {
            s.store_scale_ad(264, {
                if (!(((1.0 + (p.p1722 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1722 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1586);
        }
        s.b[1394] = (p.p75 != 0.0);s.store_scalar(1394, if s.b[1394] { 1.0 } else { 0.0 });
        if (s.b[1390] && s.b[1394]) {s.store_offset_add_scaled_inputs(267, A::scale_offset(s.ad_value(232), p.p1722, (((-(-p.p1587))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1722, (((-(-p.p1587))) + ((-1e-6)))), (-((4.0 * (-p.p1587)) * 1e-6))), 0.5, (((-p.p1587)) + (p.p1587)));}
        if (s.b[1390] && (!s.b[1394])) {
            s.store_scale_ad(267, {
                if (!(((1.0 + (p.p1722 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1722 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1587);
        }
        s.b[1395] = (p.p75 != 0.0);s.store_scalar(1395, if s.b[1395] { 1.0 } else { 0.0 });
        if (s.b[1390] && s.b[1395]) {s.store_offset_add_scaled_inputs(268, A::scale_offset(s.ad_value(232), p.p1723, (((-(-p.p1588))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1723, (((-(-p.p1588))) + ((-1e-6)))), (-((4.0 * (-p.p1588)) * 1e-6))), 0.5, (((-p.p1588)) + (p.p1588)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_68(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1390] && (!s.b[1395])) {
            s.store_scale_ad(268, {
                if (!(((1.0 + (p.p1723 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1723 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1588);
        }
        s.b[1396] = (p.p75 != 0.0);s.store_scalar(1396, if s.b[1396] { 1.0 } else { 0.0 });
        if (s.b[1390] && s.b[1396]) {s.store_offset_add_scaled_inputs(265, A::scale_offset(s.ad_value(232), p.p1723, (((-(-p.p1589))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1723, (((-(-p.p1589))) + ((-1e-6)))), (-((4.0 * (-p.p1589)) * 1e-6))), 0.5, (((-p.p1589)) + (p.p1589)));}
        if (s.b[1390] && (!s.b[1396])) {
            s.store_scale_ad(265, {
                if (!(((1.0 + (p.p1723 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1723 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1589);
        }
        if s.b[1390] {
            s.store_offset_ad(269, {
                if (!(((p.p1590 - (p.p1724 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1590, A::scale(s.ad_value(232), p.p1724)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p1590, A::scale(s.ad_value(232), p.p1724)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1590 - (p.p1724 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p1590, A::scale(s.ad_value(232), p.p1724)), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
        if s.b[1390] {
            s.store_offset_ad(272, {
                if (!(((p.p1591 - (p.p1724 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1591, A::scale(s.ad_value(232), p.p1724)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p1591, A::scale(s.ad_value(232), p.p1724)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1591 - (p.p1724 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p1591, A::scale(s.ad_value(232), p.p1724)), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
        if s.b[1390] {
            s.store_offset_ad(270, {
                if (!(((p.p1592 - (p.p1725 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1592, A::scale(s.ad_value(232), p.p1725)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p1592, A::scale(s.ad_value(232), p.p1725)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1592 - (p.p1725 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p1592, A::scale(s.ad_value(232), p.p1725)), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_69(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1390] {
            s.store_offset_ad(273, {
                if (!(((p.p1593 - (p.p1725 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1593, A::scale(s.ad_value(232), p.p1725)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p1593, A::scale(s.ad_value(232), p.p1725)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1593 - (p.p1725 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p1593, A::scale(s.ad_value(232), p.p1725)), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
        if s.b[1390] {
            s.store_offset_ad(271, {
                if (!(((p.p1594 - (p.p1726 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1594, A::scale(s.ad_value(232), p.p1726)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p1594, A::scale(s.ad_value(232), p.p1726)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1594 - (p.p1726 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p1594, A::scale(s.ad_value(232), p.p1726)), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
        if s.b[1390] {
            s.store_offset_ad(274, {
                if (!(((p.p1595 - (p.p1726 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1595, A::scale(s.ad_value(232), p.p1726)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p1595, A::scale(s.ad_value(232), p.p1726)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1595 - (p.p1726 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p1595, A::scale(s.ad_value(232), p.p1726)), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
        if s.b[1390] {s.store_sub_ad(168, A::div(s.ad_value(147), s.ad_value(180)), A::div(s.ad_value(146), s.ad_value(179)));s.store_limited_exp_scaled_input_ad(171, A::add_scaled_inputs(s.ad_value(168), 1.0, s.ad_value(418), p.p1727), 1.0 / (p.p1620));s.store_scale(275, 171, p.p1614);s.store_scale(276, 171, p.p1616);s.store_scale(277, 171, p.p1618);s.store_limited_exp_scaled_input_ad(171, A::add_scaled_inputs(s.ad_value(168), 1.0, s.ad_value(418), p.p1728), 1.0 / (p.p1621));s.store_scale(278, 171, p.p1615);s.store_scale(279, 171, p.p1617);s.store_scale(280, 171, p.p1619);s.store_scaled_limited_exp_ad(281, A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1729, s.ad_value(179), 1.0), p.p1630);s.store_scaled_limited_exp_ad(282, A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1730, s.ad_value(179), 1.0), p.p1631);s.store_scaled_limited_exp_ad(283, A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1731, s.ad_value(179), 1.0), p.p1632);s.store_scaled_limited_exp_ad(284, A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1732, s.ad_value(179), 1.0), p.p1633);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_70(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1390] {s.store_scaled_mul_ad(285, A::offset(A::sqrt(A::div_from_scalar(p.p1636, s.ad_value(158))), 1.0), A::limited_exp(A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1733, s.ad_value(179), 1.0)), p.p1634);s.store_scaled_mul_ad(286, A::offset(A::sqrt(A::div_from_scalar(p.p1636, s.ad_value(158))), 1.0), A::limited_exp(A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1734, s.ad_value(179), 1.0)), p.p1635);}
        if s.b[1390] {
            s.store_offset_ad(287, {
                if (!(((p.p1637 * (1.0 + (p.p1735 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1735) * (p.p1637)), ((p.p1637) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p.p1735) * (p.p1637)), ((p.p1637) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1637 * (1.0 + (p.p1735 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1735) * (p.p1637)), ((p.p1637) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
        if s.b[1390] {
            s.store_offset_ad(288, {
                if (!(((p.p1638 * (1.0 + (p.p1736 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1736) * (p.p1638)), ((p.p1638) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p.p1736) * (p.p1638)), ((p.p1638) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1638 * (1.0 + (p.p1736 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1736) * (p.p1638)), ((p.p1638) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
        if s.b[1390] {
            s.store_offset_ad(289, {
                if (!(((p.p1639 * (1.0 + (p.p1737 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1737) * (p.p1639)), ((p.p1639) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p.p1737) * (p.p1639)), ((p.p1639) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1639 * (1.0 + (p.p1737 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1737) * (p.p1639)), ((p.p1639) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
        if s.b[1390] {
            s.store_offset_ad(290, {
                if (!(((p.p1640 * (1.0 + (p.p1738 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1738) * (p.p1640)), ((p.p1640) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p.p1738) * (p.p1640)), ((p.p1640) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1640 * (1.0 + (p.p1738 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1738) * (p.p1640)), ((p.p1640) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
        if s.b[1390] {
            s.store_offset_ad(291, {
                if (!(((p.p1641 * (1.0 + (p.p1739 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1739) * (p.p1641)), ((p.p1641) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p.p1739) * (p.p1641)), ((p.p1641) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1641 * (1.0 + (p.p1739 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1739) * (p.p1641)), ((p.p1641) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_71(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1390] {
            s.store_offset_ad(292, {
                if (!(((p.p1642 * (1.0 + (p.p1740 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1740) * (p.p1642)), ((p.p1642) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p.p1740) * (p.p1642)), ((p.p1642) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1642 * (1.0 + (p.p1740 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1740) * (p.p1642)), ((p.p1642) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
        s.b[1397] = (!param_given[1106]);s.store_scalar(1397, if s.b[1397] { 1.0 } else { 0.0 });s.b[1398] = (p.p145 > 0.0);s.store_scalar(1398, if s.b[1398] { 1.0 } else { 0.0 });s.b[1399] = (p.p80 == 0.0);s.store_scalar(1399, if s.b[1399] { 1.0 } else { 0.0 });
        if ((s.b[1397] && s.b[1398]) && s.b[1399]) {
            let t0: A = {
                if (!(((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p145 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p145 / s.v[141]) > 1e-38) { (((p.p145 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p.p145 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p145 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p145, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), 0.5, A::sqrt_square_offset(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p.p145 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p145 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p145, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p145 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p145 / s.v[141]) > 1e-38) { (((p.p145 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                                if (!((p.p145 / s.v[141]) > 1e-38)) {
                                    A::neg(A::constant(87.498233534))
                                } else {
                                    {
                                        if ((p.p145 / s.v[141]) > 1e-38) {
                                            A::ln(A::div_from_scalar(p.p145, s.ad_value(141)))
                                        } else {
                                            A::constant(0.0)
                                        }
                                    }
                                }
                            }, (-1.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            let t1: A = {
                if (!(((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p97 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p97 / s.v[141]) > 1e-38) { (((p.p97 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p.p97 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p97 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), 0.5, A::sqrt_square_offset(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p.p97 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p97 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p97 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p97 / s.v[141]) > 1e-38) { (((p.p97 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                                if (!((p.p97 / s.v[141]) > 1e-38)) {
                                    A::neg(A::constant(87.498233534))
                                } else {
                                    {
                                        if ((p.p97 / s.v[141]) > 1e-38) {
                                            A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                        } else {
                                            A::constant(0.0)
                                        }
                                    }
                                }
                            }, (-1.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };s.store_mul_sub_mixed_iaa(479, 114, t0, A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(114), A::sub_scaled_inputs(s.ad_value(146), 0.5, t1, 1.0), (-1.0)));
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_72(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1397] && s.b[1398]) && (!s.b[1399])) {
            let t2: A = A::sub({
                if (!(((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs3(s.ad_value(146), (0.5 * 0.5), A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), ((-1.0) * 0.5), A::sqrt_square_offset(A::sub_scaled_inputs(s.ad_value(146), 0.5, A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), 1.0), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::sub_scaled_inputs(s.ad_value(146), 0.5, A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(114), A::sub_scaled_inputs(s.ad_value(146), 0.5, {
                if (!(((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs3(s.ad_value(146), (0.5 * 0.5), A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), ((-1.0) * 0.5), A::sqrt_square_offset(A::sub_scaled_inputs(s.ad_value(146), 0.5, A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), 1.0), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::sub_scaled_inputs(s.ad_value(146), 0.5, A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), (-1.0)));s.store_mul_mixed_ia(479, 114, t2);
        }
        s.b[1400] = (p.p80 == 0.0);s.store_scalar(1400, if s.b[1400] { 1.0 } else { 0.0 });
        if ((s.b[1397] && (!s.b[1398])) && s.b[1400]) {
            let t3: A = {
                if (!(((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p97 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p97 / s.v[141]) > 1e-38) { (((p.p97 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p.p97 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p97 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), 0.5, A::sqrt_square_offset(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p.p97 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p97 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p97 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p97 / s.v[141]) > 1e-38) { (((p.p97 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                                if (!((p.p97 / s.v[141]) > 1e-38)) {
                                    A::neg(A::constant(87.498233534))
                                } else {
                                    {
                                        if ((p.p97 / s.v[141]) > 1e-38) {
                                            A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                        } else {
                                            A::constant(0.0)
                                        }
                                    }
                                }
                            }, (-1.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };s.store_mul_sub_mixed_iia(479, 114, 641, A::add_scaled_product(A::scale_offset(s.ad_value(146), 0.5, p.p104), 1.0, s.ad_value(114), A::sub_scaled_inputs(s.ad_value(146), 0.5, t3, 1.0), (-1.0)));
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_73(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((s.b[1397] && (!s.b[1398])) && (!s.b[1400])) {
            s.store_mul_sub_mixed_iia(479, 114, 641, A::add_scaled_product(A::scale_offset(s.ad_value(146), 0.5, p.p104), 1.0, s.ad_value(114), A::sub_scaled_inputs(s.ad_value(146), 0.5, {
                if (!(((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs3(s.ad_value(146), (0.5 * 0.5), A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), ((-1.0) * 0.5), A::sqrt_square_offset(A::sub_scaled_inputs(s.ad_value(146), 0.5, A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), 1.0), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::sub_scaled_inputs(s.ad_value(146), 0.5, A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), (-1.0)));
        }
        if (!s.b[1397]) {s.store_scalar(479, p.p1106);}
        s.b[1401] = (!param_given[1107]);s.store_scalar(1401, if s.b[1401] { 1.0 } else { 0.0 });
        if s.b[1401] {s.copy_ad(518, 479);}
        if (!s.b[1401]) {s.store_scalar(518, p.p1107);}
        s.b[1402] = (p.p80 == 0.0);s.store_scalar(1402, if s.b[1402] { 1.0 } else { 0.0 });
        if s.b[1402] {
            s.store_mul_mixed_ia(166, 179, {
                            if (!((s.v[640] / s.v[141]) > 1e-38)) {
                                A::neg(A::constant(87.498233534))
                            } else {
                                {
                                    if ((s.v[640] / s.v[141]) > 1e-38) {
                                        A::ln(A::div(s.ad_value(640), s.ad_value(141)))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if s.b[1402] {s.store_scaled_add_mixed_ia(166, 166, A::sqrt_square_offset(s.ad_value(166), ((0.25 * 1e-10) * 1e-10)), 0.5);}
        if s.b[1402] {
            s.store_mul_mixed_ia(352, 179, {
                            if (!(((s.v[640] * p.p97) / (s.v[141] * s.v[141])) > 1e-38)) {
                                A::neg(A::constant(87.498233534))
                            } else {
                                {
                                    if (((s.v[640] * p.p97) / (s.v[141] * s.v[141])) > 1e-38) {
                                        A::ln(A::div_scaled_inputs(s.ad_value(640), p.p97, A::square(s.ad_value(141)), 1.0))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if (!s.b[1402]) {
            s.store_mul_sub_mixed_iai(166, 179, {
                if (!(s.v[640] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[640] > 1e-38) {
                            A::ln(s.ad_value(640))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 142);
        }
        if (!s.b[1402]) {s.store_scaled_add_mixed_ia(166, 166, A::sqrt_square_offset(s.ad_value(166), ((0.25 * 1e-10) * 1e-10)), 0.5);}
        if (!s.b[1402]) {
            s.store_mul_sub_scaled_inputs_rhs_mixed_ai(352, 179, {
                if (!((s.v[640] * p.p97) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[640] * p.p97) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(640), p.p97)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 142, 2.0);
        }
        s.store_mul_sub_mixed_iia(167, 114, 641, A::offset({
            if (p.p60 == 1.0) {
                A::constant(0.0)
            } else {
                s.ad_value(146)
            }
        }, p.p104));s.store_scale(407, 322, 0.5);s.store_scalar(408, 0.5);s.b[1403] = (p.p60 != 1.0);s.store_scalar(1403, if s.b[1403] { 1.0 } else { 0.0 });
        if s.b[1403] {s.store_scale(407, 322, 0.333333333);s.store_scalar(408, 0.333333333);}
        s.b[1404] = (p.p61 != 0.0);s.store_scalar(1404, if s.b[1404] { 1.0 } else { 0.0 });
        if s.b[1404] {s.store_add_scaled_inputs3_indices(537, 275, p.p11, 276, p.p13, 277, (p.p3 * s.v[115]));}
        s.b[1405] = (s.v[537] > 0.0);s.store_scalar(1405, if s.b[1405] { 1.0 } else { 0.0 });
        if (s.b[1404] && s.b[1405]) {s.store_scale(539, 179, p.p1620);s.store_scaled_limited_exp_ad(547, A::div_from_scalar((-p.p1626), s.ad_value(539)), p.p1628);s.store_max_with_scalar_ad(170, A::div_from_scalar(p.p1622, s.ad_value(537)), 10.0);s.store_sub_offset_lhs(226, 170, 1.0, 547);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_74(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1404] && s.b[1405]) {
            s.store_mul_mixed_ia(546, 539, {
                            if (!((0.5 * (s.v[226] + ((((s.v[226] * s.v[226]) + (4.0 * s.v[547]))) as f64).sqrt())) > 1e-38)) {
                                A::neg(A::constant(87.498233534))
                            } else {
                                {
                                    if ((0.5 * (s.v[226] + ((((s.v[226] * s.v[226]) + (4.0 * s.v[547]))) as f64).sqrt())) > 1e-38) {
                                        A::ln_scaled_input(A::add(s.ad_value(226), A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(226)), 1.0, s.ad_value(547), 4.0))), 0.5)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if (s.b[1404] && s.b[1405]) {s.store_limited_exp_div(168, 546, 539);}
        if (s.b[1404] && s.b[1405]) {
            s.store_offset_ad(170, {
                if (!(((p.p1624 / s.v[537]) - 10.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::div_from_scalar(p.p1624, s.ad_value(537)), (-10.0)), 0.5, A::sqrt_square_offset(A::offset(A::div_from_scalar(p.p1624, s.ad_value(537)), (-10.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1624 / s.v[537]) - 10.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::div_from_scalar(p.p1624, s.ad_value(537)), (-10.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 10.0);
        }
        if (s.b[1404] && s.b[1405]) {
            s.store_sub_from_scalar_scaled_mul_mixed_ia(543, (-p.p1626), 539, {
                if (!(((s.v[170] - 1.0) / p.p1628) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((s.v[170] - 1.0) / p.p1628) > 1e-38) {
                            A::ln_scaled_input(A::offset(s.ad_value(170), (-1.0)), 1.0 / (p.p1628))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0);
        }
        if (s.b[1404] && s.b[1405]) {s.store_scale_ad(169, A::limited_exp_div_scaled_inputs(A::offset(s.ad_value(543), p.p1626), -1.0, s.ad_value(539), 1.0), p.p1628);s.store_mul_scale_offset_indices(542, 537, 169, 1.0, 1.0);s.store_div_scaled_product_indices(541, 537, 169, -1.0, 539, 1.0);}
        if s.b[1404] {s.store_add_scaled_inputs3_indices(538, 278, p.p12, 279, p.p14, 280, (p.p3 * s.v[115]));}
        s.b[1406] = (s.v[538] > 0.0);s.store_scalar(1406, if s.b[1406] { 1.0 } else { 0.0 });
        if (s.b[1404] && s.b[1406]) {s.store_scale(540, 179, p.p1621);s.store_scaled_limited_exp_ad(554, A::div_from_scalar((-p.p1627), s.ad_value(540)), p.p1629);s.store_max_with_scalar_ad(170, A::div_from_scalar(p.p1623, s.ad_value(538)), 10.0);s.store_sub_offset_lhs(226, 170, 1.0, 554);}
        if (s.b[1404] && s.b[1406]) {
            s.store_mul_mixed_ia(553, 540, {
                            if (!((0.5 * (s.v[226] + ((((s.v[226] * s.v[226]) + (4.0 * s.v[554]))) as f64).sqrt())) > 1e-38)) {
                                A::neg(A::constant(87.498233534))
                            } else {
                                {
                                    if ((0.5 * (s.v[226] + ((((s.v[226] * s.v[226]) + (4.0 * s.v[554]))) as f64).sqrt())) > 1e-38) {
                                        A::ln_scaled_input(A::add(s.ad_value(226), A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(226)), 1.0, s.ad_value(554), 4.0))), 0.5)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if (s.b[1404] && s.b[1406]) {s.store_limited_exp_div(168, 553, 540);}
        if (s.b[1404] && s.b[1406]) {
            s.store_offset_ad(170, {
                if (!(((p.p1625 / s.v[538]) - 10.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::div_from_scalar(p.p1625, s.ad_value(538)), (-10.0)), 0.5, A::sqrt_square_offset(A::offset(A::div_from_scalar(p.p1625, s.ad_value(538)), (-10.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1625 / s.v[538]) - 10.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::div_from_scalar(p.p1625, s.ad_value(538)), (-10.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 10.0);
        }
        if (s.b[1404] && s.b[1406]) {
            s.store_sub_from_scalar_scaled_mul_mixed_ia(550, (-p.p1627), 540, {
                if (!(((s.v[170] - 1.0) / p.p1629) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((s.v[170] - 1.0) / p.p1629) > 1e-38) {
                            A::ln_scaled_input(A::offset(s.ad_value(170), (-1.0)), 1.0 / (p.p1629))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0);
        }
        if (s.b[1404] && s.b[1406]) {s.store_scale_ad(169, A::limited_exp_div_scaled_inputs(A::offset(s.ad_value(550), p.p1627), -1.0, s.ad_value(540), 1.0), p.p1629);s.store_mul_scale_offset_indices(549, 538, 169, 1.0, 1.0);s.store_div_scaled_product_indices(548, 538, 169, -1.0, 540, 1.0);}
        if s.b[1404] {s.store_scale(523, 263, p.p11);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_75(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1404] {s.store_scale(524, 264, p.p13);s.store_scaled_mul(525, 268, 158, s.v[115]);s.store_scale(526, 266, p.p12);s.store_scale(527, 267, p.p14);s.store_scaled_mul(528, 265, 158, s.v[115]);}
        s.b[1407] = (p.p1602 > 0.0);s.store_scalar(1407, if s.b[1407] { 1.0 } else { 0.0 });
        if (s.b[1404] && s.b[1407]) {s.store_scale(557, 269, (1.0 - (((1.0 / p.p1602)) as f64).powf((1.0 / p.p1596))));s.store_div_scaled_inputs_mixed_ia(558, 269, (p.p1602 * (p.p1608 * 1.0 / (p.p1596))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(557), s.ad_value(269))), (-(1.0 + p.p1596))), 1.0);}
        s.b[1408] = (p.p1604 > 0.0);s.store_scalar(1408, if s.b[1408] { 1.0 } else { 0.0 });
        if (s.b[1404] && s.b[1408]) {s.store_scale(559, 270, (1.0 - (((1.0 / p.p1604)) as f64).powf((1.0 / p.p1598))));s.store_div_scaled_inputs_mixed_ia(560, 270, (p.p1604 * (p.p1610 * 1.0 / (p.p1598))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(559), s.ad_value(270))), (-(1.0 + p.p1598))), 1.0);}
        s.b[1409] = (p.p1606 > 0.0);s.store_scalar(1409, if s.b[1409] { 1.0 } else { 0.0 });
        if (s.b[1404] && s.b[1409]) {s.store_scale(561, 271, (1.0 - (((1.0 / p.p1606)) as f64).powf((1.0 / p.p1600))));s.store_div_scaled_inputs_mixed_ia(562, 271, (p.p1606 * (p.p1612 * 1.0 / (p.p1600))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(561), s.ad_value(271))), (-(1.0 + p.p1600))), 1.0);}
        s.b[1410] = (p.p1603 > 0.0);s.store_scalar(1410, if s.b[1410] { 1.0 } else { 0.0 });
        if (s.b[1404] && s.b[1410]) {s.store_scale(563, 272, (1.0 - (((1.0 / p.p1603)) as f64).powf((1.0 / p.p1597))));s.store_div_scaled_inputs_mixed_ia(564, 272, (p.p1603 * (p.p1609 * 1.0 / (p.p1597))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(563), s.ad_value(272))), (-(1.0 + p.p1597))), 1.0);}
        s.b[1411] = (p.p1605 > 0.0);s.store_scalar(1411, if s.b[1411] { 1.0 } else { 0.0 });
        if (s.b[1404] && s.b[1411]) {s.store_scale(565, 273, (1.0 - (((1.0 / p.p1605)) as f64).powf((1.0 / p.p1599))));s.store_div_scaled_inputs_mixed_ia(566, 273, (p.p1605 * (p.p1611 * 1.0 / (p.p1599))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(565), s.ad_value(273))), (-(1.0 + p.p1599))), 1.0);}
        s.b[1412] = (p.p1607 > 0.0);s.store_scalar(1412, if s.b[1412] { 1.0 } else { 0.0 });
        if (s.b[1404] && s.b[1412]) {s.store_scale(567, 274, (1.0 - (((1.0 / p.p1607)) as f64).powf((1.0 / p.p1601))));s.store_div_scaled_inputs_mixed_ia(568, 274, (p.p1607 * (p.p1613 * 1.0 / (p.p1601))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(567), s.ad_value(274))), (-(1.0 + p.p1601))), 1.0);}
        s.store_mul_voltage_ad(134, s.ad_value(114), ctx, nodes, Some(11), Some(6));s.store_mul_voltage_ad(135, s.ad_value(114), ctx, nodes, Some(5), Some(6));s.store_mul_voltage_ad(136, s.ad_value(114), ctx, nodes, Some(11), Some(5));s.store_mul_voltage_ad(521, s.ad_value(114), ctx, nodes, Some(3), Some(6));s.store_mul_voltage_ad(522, s.ad_value(114), ctx, nodes, Some(3), Some(5));s.store_mul_voltage_ad(497, s.ad_value(114), ctx, nodes, Some(11), Some(3));s.b[1413] = (p.p76 != 2.0);s.store_scalar(1413, if s.b[1413] { 1.0 } else { 0.0 });
        if s.b[1413] {s.store_mul_voltage_ad(132, s.ad_value(114), ctx, nodes, Some(10), Some(5));s.store_mul_voltage_ad(133, s.ad_value(114), ctx, nodes, Some(10), Some(6));}
        if (!s.b[1413]) {s.store_mul_voltage_ad(132, s.ad_value(114), ctx, nodes, Some(14), Some(5));s.store_mul_voltage_ad(133, s.ad_value(114), ctx, nodes, Some(13), Some(6));}
        s.store_scalar(128, 1.0);s.b[1414] = (s.v[135] < 0.0);s.store_scalar(1414, if s.b[1414] { 1.0 } else { 0.0 });
        if s.b[1414] {s.store_scalar(128, (-1.0));s.store_sub(125, 134, 135);s.store_scale(126, 135, (-1.0));s.copy_ad(367, 522);}
        if (!s.b[1414]) {s.copy_ad(125, 134);s.copy_ad(126, 135);s.copy_ad(367, 521);}
        s.store_sub(347, 125, 167);s.store_offset_sqrt_ad(127, A::offset(A::square(s.ad_value(126)), 0.01), (-0.1));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_76(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1415] = (p.p61 != 0.0);s.store_scalar(1415, if s.b[1415] { 1.0 } else { 0.0 });
        if s.b[1415] {s.store_add_scaled_inputs3_indices(368, 367, 1.0, 126, (-0.5), 127, (-(-0.5)));s.store_primal_scale(369, 689, 0.95);s.store_offset_sub(170, 369, 368, (-0.001));s.store_add_scaled_inputs3_sqrt_third_mixed_iia(370, 369, 1.0, 170, (-0.5), A::add_scaled_inputs(A::square(s.ad_value(170)), 1.0, s.ad_value(369), 0.004), (-0.5));}
        s.store_tanh_ad(168, A::div_scaled_inputs(s.ad_value(135), 0.6, s.ad_value(179), 1.0));s.store_offset_scaled(186, 168, 0.5, 0.5);s.store_sub_from_scalar(187, 1.0, 186);s.b[1416] = (p.p66 != 0.0);s.store_scalar(1416, if s.b[1416] { 1.0 } else { 0.0 });
        if s.b[1416] {s.store_add_scaled_products_indices(664, 665, 187, 1.0, 663, 186, 1.0);s.store_add_scaled_products_indices(676, 298, 187, 1.0, 296, 186, 1.0);s.store_add_scaled_products_indices(427, 715, 187, 1.0, 714, 186, 1.0);s.store_add_scaled_products_indices(718, 717, 187, 1.0, 716, 186, 1.0);s.store_add_scaled_products_indices(423, 338, 187, 1.0, 337, 186, 1.0);s.store_add_scaled_products_indices(424, 258, 187, 1.0, 257, 186, 1.0);s.store_add_scaled_products_indices(422, 335, 187, 1.0, 334, 186, 1.0);s.store_add_scaled_products_indices(425, 300, 187, 1.0, 299, 186, 1.0);s.store_add_scaled_products_indices(426, 302, 187, 1.0, 301, 186, 1.0);s.store_add_scaled_products_indices(795, 796, 187, 1.0, 797, 186, 1.0);s.store_add_scaled_products_indices(428, 333, 187, 1.0, 332, 186, 1.0);s.store_add_scaled_products_indices(659, 658, 187, 1.0, 660, 186, 1.0);s.store_add_scaled_products_indices(805, 806, 187, 1.0, 804, 186, 1.0);s.store_add_scaled_products_indices(669, 668, 187, 1.0, 666, 186, 1.0);s.store_add_scaled_products_indices(416, 417, 187, 1.0, 413, 186, 1.0);s.store_add_scaled_products_indices(819, 305, 187, 1.0, 303, 186, 1.0);s.store_add_scaled_products_indices(820, 320, 187, 1.0, 318, 186, 1.0);s.store_add_scaled_products_indices(821, 316, 187, 1.0, 314, 186, 1.0);s.store_add_scaled_products_indices(822, 816, 187, 1.0, 323, 186, 1.0);}
        if (!s.b[1416]) {s.copy_ad(664, 663);s.copy_ad(676, 296);s.copy_ad(427, 714);s.copy_ad(718, 716);s.copy_ad(423, 337);s.copy_ad(424, 257);s.copy_ad(422, 334);s.copy_ad(425, 299);s.copy_ad(426, 301);s.copy_ad(795, 797);s.copy_ad(428, 332);s.copy_ad(659, 660);s.copy_ad(805, 804);s.copy_ad(669, 666);s.copy_ad(416, 413);s.copy_ad(819, 303);s.copy_ad(820, 318);s.copy_ad(821, 314);s.copy_ad(822, 323);}
        s.store_div_from_scalar(212, 1.0, 423);s.store_add_offset_lhs(353, 166, 0.4, 672);s.store_div_scaled_value_by_product_mixed_iia(169, 893, 2.0, 895, A::offset(s.ad_value(898), 2.0), 1.0);s.store_mul_add_scaled_product_rhs_indices(164, 362, 662, 1.0, 664, 127, 1.0);s.b[1417] = (p.p175 == 0.0);s.store_scalar(1417, if s.b[1417] { 1.0 } else { 0.0 });s.b[1418] = (p.p80 == 0.0);s.store_scalar(1418, if s.b[1418] { 1.0 } else { 0.0 });
        if (s.b[1417] && s.b[1418]) {s.store_mul_ad_product_rhs_mixed_ia(181, 179, 235, A::offset(A::div_scaled_inputs2(s.ad_value(669), 1.0, s.ad_value(164), 1.0, s.ad_value(169), 1.0), 1.0));}
        if (s.b[1417] && (!s.b[1418])) {s.store_mul_ad_product_rhs_mixed_ia(181, 182, 235, A::offset(A::div_scaled_inputs2(s.ad_value(669), 1.0, s.ad_value(164), 1.0, s.ad_value(169), 1.0), 1.0));}
        if (!s.b[1417]) {s.store_scalar(181, p.p175);}
        s.store_div(897, 903, 181);
        if (!(((s.v[893] * s.v[181]) / (((1.60219e-19 * s.v[148]) * 2.0) * s.v[894])) > 1e-38)) {
            s.store_scalar(900, (-87.498233534));
        } else {
            if (((s.v[893] * s.v[181]) / (((1.60219e-19 * s.v[148]) * 2.0) * s.v[894])) > 1e-38) {
                s.store_ln_ad(900, A::div_scaled_product_by_product(s.ad_value(893), s.ad_value(181), 1.0, s.ad_value(148), s.ad_value(894), (1.60219e-19 * 2.0)));
            } else {
                s.store_scalar(900, 0.0);
            }
        }
    }
}
