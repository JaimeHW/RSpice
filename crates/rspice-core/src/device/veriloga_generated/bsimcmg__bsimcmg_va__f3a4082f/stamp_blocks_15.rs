#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_105(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1501] {s.store_offset_sqrt_ad(473, A::offset(A::square(s.ad_value(390)), 0.01), (-0.1));s.store_mul(169, 736, 473);s.store_limited_exp_neg_input(474, 169);s.store_offset_add(171, 169, 474, (((-1.0)) + (0.0001)));s.store_offset_sub_from_scalar_ad(172, 1.0, A::mul_offset_lhs(s.ad_value(169), 1.0, s.ad_value(474)), 0.0001);s.store_offset_square(174, 169, 0.0002);s.store_sub(168, 134, 479);s.store_sqrt_square_offset(482, 168, 0.0001);}
        s.b[1502] = (p[82] == 1.0);s.store_scalar(1502, if s.b[1502] { 1.0 } else { 0.0 });
        if (s.b[1501] && s.b[1502]) {
            if (!((s.v[246] - (s.v[739] * s.v[482])) < ((-10000.0) * 1e-6))) {
                s.store_scaled_add_sqrt_square_offset_ad(169, A::add_scaled_product(s.ad_value(246), 1.0, s.ad_value(739), s.ad_value(482), (-1.0)), ((4.0 * 1e-6) * 1e-6), 0.5);
            } else {
                if ((s.v[246] - (s.v[739] * s.v[482])) < ((-10000.0) * 1e-6)) {
                    s.store_div_from_scalar_ad(169, ((-1e-6) * 1e-6), A::add_scaled_product(s.ad_value(246), 1.0, s.ad_value(739), s.ad_value(482), (-1.0)));
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }
        s.b[1503] = (s.v[740] < 0.01);s.store_scalar(1503, if s.b[1503] { 1.0 } else { 0.0 });
        if ((s.b[1501] && s.b[1502]) && s.b[1503]) {s.store_scalar(740, 0.01);}
        if (s.b[1501] && (!s.b[1502])) {s.store_add_scaled_product_indices(169, 246, 1.0, 739, 482, (-1.0));}
        if s.b[1501] {s.store_offset_mul(170, 740, 482, 1.0);s.store_mul_product3_indices(171, 170, 485, 742, 169, (-p[1109]));s.store_limited_exp(172, 171);s.store_sub(168, 136, 479);s.store_sqrt_square_offset(483, 168, 0.0001);}
        s.b[1505] = (p[82] == 1.0);s.store_scalar(1505, if s.b[1505] { 1.0 } else { 0.0 });
        if (s.b[1501] && s.b[1505]) {
            if (!((s.v[247] - (s.v[745] * s.v[483])) < ((-10000.0) * 1e-6))) {
                s.store_scaled_add_sqrt_square_offset_ad(169, A::add_scaled_product(s.ad_value(247), 1.0, s.ad_value(745), s.ad_value(483), (-1.0)), ((4.0 * 1e-6) * 1e-6), 0.5);
            } else {
                if ((s.v[247] - (s.v[745] * s.v[483])) < ((-10000.0) * 1e-6)) {
                    s.store_div_from_scalar_ad(169, ((-1e-6) * 1e-6), A::add_scaled_product(s.ad_value(247), 1.0, s.ad_value(745), s.ad_value(483), (-1.0)));
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }
        s.b[1506] = (s.v[746] < 0.01);s.store_scalar(1506, if s.b[1506] { 1.0 } else { 0.0 });
        if ((s.b[1501] && s.b[1505]) && s.b[1506]) {s.store_scalar(746, 0.01);}
        if (s.b[1501] && (!s.b[1505])) {s.store_add_scaled_product_indices(169, 247, 1.0, 745, 483, (-1.0));}
        if s.b[1501] {s.store_offset_mul(170, 746, 483, 1.0);s.store_mul_product3_indices(171, 170, 485, 742, 169, (-p[1109]));s.store_limited_exp(172, 171);}
        s.b[1508] = (p[70] != 0.0);s.store_scalar(1508, if s.b[1508] { 1.0 } else { 0.0 });
        if s.b[1508] {s.store_scalar(168, (s.v[145] * p[89]));}
        s.b[1509] = ((s.v[747] <= 0.0) || (s.v[252] <= 0.0));s.store_scalar(1509, if s.b[1509] { 1.0 } else { 0.0 });
        if (s.b[1508] && s.b[1509]) {s.store_scalar(175, 0.0);}
        if (s.b[1508] && (!s.b[1509])) {s.store_div_scaled_inputs3_indices(169, 136, -1.0, 750, (-1.0), 479, 1.0, 168, 1.0);}
        if (s.b[1508] && (!s.b[1509])) {
            if (!(s.v[169] < ((-10000.0) * 0.01))) {
                s.store_scaled_add_sqrt_square_offset_rhs(169, 169, 169, ((4.0 * 0.01) * 0.01), 0.5);
            } else {
                if (s.v[169] < ((-10000.0) * 0.01)) {
                    s.store_div_from_scalar(169, ((-0.01) * 0.01), 169);
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }
        if (s.b[1508] && (!s.b[1509])) {s.store_div_scaled_value_offset_denominator(170, s.ad_value(252), 1.0, s.ad_value(169), 0.001, 1.0);s.store_pow_indices(171, 169, 751);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_106(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1510] = (p[61] != 0.0);s.store_scalar(1510, if s.b[1510] { 1.0 } else { 0.0 });
        if ((s.b[1508] && (!s.b[1509])) && s.b[1510]) {s.store_mul3_affine_lhs(172, 522, 522, -1.0, 0.0, 522);s.store_offset_add_ad(173, s.ad_value(749), A::abs(s.ad_value(172)), 1e-5);}
        if ((s.b[1508] && (!s.b[1509])) && s.b[1510]) {
            s.store_offset_ad(174, {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(172), s.ad_value(173)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(172), s.ad_value(173)), ((4.0 * 1e-6) * 1e-6)), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, (-1e-6));
        }
        if ((s.b[1508] && (!s.b[1509])) && s.b[1510]) {s.store_mul_ad_product_lhs(175, A::mul3(s.ad_value(747), s.ad_value(158), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), 174);}
        if ((s.b[1508] && (!s.b[1509])) && (!s.b[1510])) {s.store_mul_ad_product_lhs(175, A::mul3(s.ad_value(747), s.ad_value(158), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), 135);}
        s.b[1511] = ((p[70] == 3.0) && (s.v[752] > 0.0));s.store_scalar(1511, if s.b[1511] { 1.0 } else { 0.0 });s.b[1512] = (p[61] != 0.0);s.store_scalar(1512, if s.b[1512] { 1.0 } else { 0.0 });
        if ((s.b[1508] && s.b[1511]) && s.b[1512]) {
            s.store_mul_mixed_ia(254, 754, {
                            if (!(((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if ((s.b[1508] && s.b[1511]) && s.b[1512]) {s.store_div_scaled_inputs3_mixed_aiii(169, A::add_scaled_product(A::mul3(s.ad_value(753), s.ad_value(136), s.ad_value(136)), 1.0, s.ad_value(254), s.ad_value(136), (-1.0)), 1.0, 755, (-1.0), 479, 1.0, 179, 1.0);s.store_mul_product3_mixed_aiii(170, A::limited_exp(s.ad_value(169)), 752, 158, 141, 1.0);s.store_mul3_affine_lhs(172, 522, 522, -1.0, 0.0, 522);s.store_offset_add_ad(173, s.ad_value(749), A::abs(s.ad_value(172)), 1e-5);}
        if ((s.b[1508] && s.b[1511]) && s.b[1512]) {
            s.store_offset_ad(174, {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(172), s.ad_value(173)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(172), s.ad_value(173)), ((4.0 * 1e-6) * 1e-6)), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, (-1e-6));
        }
        if ((s.b[1508] && s.b[1511]) && s.b[1512]) {s.store_add_scaled_product_indices(175, 175, 1.0, 170, 174, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_107(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1508] && s.b[1511]) && (!s.b[1512])) {
            s.store_mul_mixed_ia(254, 754, {
                            if (!(((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if ((s.b[1508] && s.b[1511]) && (!s.b[1512])) {s.store_div_scaled_inputs3_mixed_aiii(169, A::add_scaled_product(A::mul3(s.ad_value(753), s.ad_value(136), s.ad_value(136)), 1.0, s.ad_value(254), s.ad_value(136), (-1.0)), 1.0, 755, (-1.0), 479, 1.0, 179, 1.0);s.store_mul_product3_mixed_aiii(170, A::limited_exp(s.ad_value(169)), 752, 158, 141, 1.0);s.store_add_scaled_product_indices(175, 175, 1.0, 170, 135, 1.0);}
        s.b[1513] = (((p[61] != 0.0) && ((p[70] == 2.0) || (p[70] == 3.0))) && (((p[62] == 2.0) || (p[62] == 3.0)) || (p[62] == 5.0)));s.store_scalar(1513, if s.b[1513] { 1.0 } else { 0.0 });
        if (s.b[1508] && s.b[1513]) {
            s.store_mul_mixed_ia(255, 757, {
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
        }
        s.b[1514] = ((s.v[756] <= 0.0) || (s.v[255] <= 0.0));s.store_scalar(1514, if s.b[1514] { 1.0 } else { 0.0 });
        if ((s.b[1508] && s.b[1513]) && s.b[1514]) {s.store_scalar(176, 0.0);}
        if ((s.b[1508] && s.b[1513]) && (!s.b[1514])) {s.store_div_scaled_inputs3_indices(169, 136, -1.0, 759, (-1.0), 479, 1.0, 168, 1.0);}
        if ((s.b[1508] && s.b[1513]) && (!s.b[1514])) {
            if (!(s.v[169] < ((-10000.0) * 0.01))) {
                s.store_scaled_add_sqrt_square_offset_rhs(169, 169, 169, ((4.0 * 0.01) * 0.01), 0.5);
            } else {
                if (s.v[169] < ((-10000.0) * 0.01)) {
                    s.store_div_from_scalar(169, ((-0.01) * 0.01), 169);
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }
        if ((s.b[1508] && s.b[1513]) && (!s.b[1514])) {s.store_div_scaled_value_offset_denominator(170, s.ad_value(255), 1.0, s.ad_value(169), 0.001, 1.0);s.store_pow_indices(171, 169, 760);s.store_mul3_affine_lhs(172, 522, 522, -1.0, 0.0, 522);s.store_offset_add_ad(173, s.ad_value(758), A::abs(s.ad_value(172)), 1e-5);}
        if ((s.b[1508] && s.b[1513]) && (!s.b[1514])) {
            s.store_offset_ad(174, {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(172), s.ad_value(173)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(172), s.ad_value(173)), ((4.0 * 1e-6) * 1e-6)), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, (-1e-6));
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_108(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1508] && s.b[1513]) && (!s.b[1514])) {s.store_mul_ad_product_lhs(176, A::mul3(s.ad_value(756), s.ad_value(896), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), 174);}
        s.b[1516] = ((s.v[761] <= 0.0) || (s.v[250] <= 0.0));s.store_scalar(1516, if s.b[1516] { 1.0 } else { 0.0 });
        if (s.b[1508] && s.b[1516]) {s.store_scalar(175, 0.0);}
        if (s.b[1508] && (!s.b[1516])) {s.store_div_scaled_inputs3_indices(169, 134, -1.0, 764, (-1.0), 479, 1.0, 168, 1.0);}
        if (s.b[1508] && (!s.b[1516])) {
            if (!(s.v[169] < ((-10000.0) * 0.01))) {
                s.store_scaled_add_sqrt_square_offset_rhs(169, 169, 169, ((4.0 * 0.01) * 0.01), 0.5);
            } else {
                if (s.v[169] < ((-10000.0) * 0.01)) {
                    s.store_div_from_scalar(169, ((-0.01) * 0.01), 169);
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }
        if (s.b[1508] && (!s.b[1516])) {s.store_div_scaled_value_offset_denominator(170, s.ad_value(250), 1.0, s.ad_value(169), 0.001, 1.0);s.store_pow_indices(171, 169, 765);}
        s.b[1517] = (p[61] != 0.0);s.store_scalar(1517, if s.b[1517] { 1.0 } else { 0.0 });
        if ((s.b[1508] && (!s.b[1516])) && s.b[1517]) {s.store_mul3_affine_lhs(172, 521, 521, -1.0, 0.0, 521);s.store_offset_add_ad(173, s.ad_value(763), A::abs(s.ad_value(172)), 1e-5);}
        if ((s.b[1508] && (!s.b[1516])) && s.b[1517]) {
            s.store_offset_ad(174, {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(172), s.ad_value(173)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(172), s.ad_value(173)), ((4.0 * 1e-6) * 1e-6)), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, (-1e-6));
        }
        if ((s.b[1508] && (!s.b[1516])) && s.b[1517]) {s.store_mul_ad_product_lhs(175, A::mul3(s.ad_value(761), s.ad_value(158), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), 174);}
        if ((s.b[1508] && (!s.b[1516])) && (!s.b[1517])) {s.store_mul_ad_affine_product_lhs(175, A::mul3(s.ad_value(761), s.ad_value(158), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), -1.0, 0.0, 135);}
        s.b[1518] = ((p[70] == 3.0) && (s.v[766] > 0.0));s.store_scalar(1518, if s.b[1518] { 1.0 } else { 0.0 });s.b[1519] = (p[61] != 0.0);s.store_scalar(1519, if s.b[1519] { 1.0 } else { 0.0 });
        if ((s.b[1508] && s.b[1518]) && s.b[1519]) {
            s.store_mul_mixed_ia(253, 768, {
                            if (!(((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if ((s.b[1508] && s.b[1518]) && s.b[1519]) {s.store_div_scaled_inputs3_mixed_aiii(169, A::add_scaled_product(A::mul3(s.ad_value(767), s.ad_value(134), s.ad_value(134)), 1.0, s.ad_value(253), s.ad_value(134), (-1.0)), 1.0, 769, (-1.0), 479, 1.0, 179, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_109(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1508] && s.b[1518]) && s.b[1519]) {s.store_mul_product3_mixed_aiii(170, A::limited_exp(s.ad_value(169)), 766, 158, 141, 1.0);s.store_mul3_affine_lhs(172, 521, 521, -1.0, 0.0, 521);s.store_offset_add_ad(173, s.ad_value(763), A::abs(s.ad_value(172)), 1e-5);}
        if ((s.b[1508] && s.b[1518]) && s.b[1519]) {
            s.store_offset_ad(174, {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(172), s.ad_value(173)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(172), s.ad_value(173)), ((4.0 * 1e-6) * 1e-6)), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, (-1e-6));
        }
        if ((s.b[1508] && s.b[1518]) && s.b[1519]) {s.store_add_scaled_product_indices(175, 175, 1.0, 170, 174, 1.0);}
        if ((s.b[1508] && s.b[1518]) && (!s.b[1519])) {
            s.store_mul_mixed_ia(253, 768, {
                            if (!(((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if ((s.b[1508] && s.b[1518]) && (!s.b[1519])) {s.store_div_scaled_inputs3_mixed_aiii(169, A::add_scaled_product(A::mul3(s.ad_value(767), s.ad_value(134), s.ad_value(134)), 1.0, s.ad_value(253), s.ad_value(134), (-1.0)), 1.0, 769, (-1.0), 479, 1.0, 179, 1.0);s.store_mul_product3_mixed_aiii(170, A::limited_exp(s.ad_value(169)), 766, 158, 141, 1.0);s.store_add_scaled_product_indices(175, 175, 1.0, 170, 135, -1.0);}
        s.b[1520] = (((p[61] != 0.0) && ((p[70] == 2.0) || (p[70] == 3.0))) && (((p[62] == 2.0) || (p[62] == 3.0)) || (p[62] == 5.0)));s.store_scalar(1520, if s.b[1520] { 1.0 } else { 0.0 });
        if (s.b[1508] && s.b[1520]) {
            s.store_mul_mixed_ia(251, 771, {
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
        }
        s.b[1521] = ((s.v[770] <= 0.0) || (s.v[251] <= 0.0));s.store_scalar(1521, if s.b[1521] { 1.0 } else { 0.0 });
        if ((s.b[1508] && s.b[1520]) && s.b[1521]) {s.store_scalar(176, 0.0);}
        if ((s.b[1508] && s.b[1520]) && (!s.b[1521])) {s.store_div_scaled_inputs3_indices(169, 134, -1.0, 773, (-1.0), 479, 1.0, 168, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_110(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1508] && s.b[1520]) && (!s.b[1521])) {
            if (!(s.v[169] < ((-10000.0) * 0.01))) {
                s.store_scaled_add_sqrt_square_offset_rhs(169, 169, 169, ((4.0 * 0.01) * 0.01), 0.5);
            } else {
                if (s.v[169] < ((-10000.0) * 0.01)) {
                    s.store_div_from_scalar(169, ((-0.01) * 0.01), 169);
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }
        if ((s.b[1508] && s.b[1520]) && (!s.b[1521])) {s.store_div_scaled_value_offset_denominator(170, s.ad_value(251), 1.0, s.ad_value(169), 0.001, 1.0);s.store_pow_indices(171, 169, 774);s.store_mul3_affine_lhs(172, 521, 521, -1.0, 0.0, 521);s.store_offset_add_ad(173, s.ad_value(772), A::abs(s.ad_value(172)), 1e-5);}
        if ((s.b[1508] && s.b[1520]) && (!s.b[1521])) {
            s.store_offset_ad(174, {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(172), s.ad_value(173)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(172), s.ad_value(173)), ((4.0 * 1e-6) * 1e-6)), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, (-1e-6));
        }
        if ((s.b[1508] && s.b[1520]) && (!s.b[1521])) {s.store_mul_ad_product_lhs(176, A::mul3(s.ad_value(770), s.ad_value(896), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), 174);}
        s.b[1523] = (p[61] != 0.0);s.store_scalar(1523, if s.b[1523] { 1.0 } else { 0.0 });s.b[1524] = (s.v[537] > 0.0);s.store_scalar(1524, if s.b[1524] { 1.0 } else { 0.0 });s.b[1525] = (s.v[521] < s.v[543]);s.store_scalar(1525, if s.b[1525] { 1.0 } else { 0.0 });
        if ((s.b[1523] && s.b[1524]) && s.b[1525]) {s.store_div(168, 521, 539);s.store_offset_limited_exp(169, 168, (-1.0));s.store_add_scaled_product_right_sub(170, 542, 1.0, 541, 521, 543, 1.0);}
        s.b[1526] = (s.v[521] <= s.v[546]);s.store_scalar(1526, if s.b[1526] { 1.0 } else { 0.0 });
        if (((s.b[1523] && s.b[1524]) && (!s.b[1525])) && s.b[1526]) {s.store_div(168, 521, 539);s.store_div_scaled_offset_numerator_indices(169, 521, 1.0, p[1626], 539, 1.0);s.store_limited_exp_neg_input(170, 169);}
        s.b[1527] = (s.v[281] > 0.0);s.store_scalar(1527, if s.b[1527] { 1.0 } else { 0.0 });s.b[1528] = ((p[1643] - s.v[521]) < (p[1643] * 0.001));s.store_scalar(1528, if s.b[1528] { 1.0 } else { 0.0 });
        if ((s.b[1523] && s.b[1527]) && s.b[1528]) {s.store_div_scaled_value_by_product_indices(168, 521, -1.0, 180, 287, 1.0);s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));}
        if ((s.b[1523] && s.b[1527]) && (!s.b[1528])) {s.store_div_scaled_value_by_product_indices(168, 521, -1.0, 180, 287, 1.0);s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p[1643], A::sub_from_scalar(p[1643], s.ad_value(521)), 1.0), (-1.0));}
        s.b[1529] = (s.v[283] > 0.0);s.store_scalar(1529, if s.b[1529] { 1.0 } else { 0.0 });s.b[1530] = ((p[1645] - s.v[521]) < (p[1645] * 0.001));s.store_scalar(1530, if s.b[1530] { 1.0 } else { 0.0 });
        if ((s.b[1523] && s.b[1529]) && s.b[1530]) {s.store_div_scaled_value_by_product_indices(168, 521, -1.0, 180, 289, 1.0);s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));}
        if ((s.b[1523] && s.b[1529]) && (!s.b[1530])) {s.store_div_scaled_value_by_product_indices(168, 521, -1.0, 180, 289, 1.0);s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p[1645], A::sub_from_scalar(p[1645], s.ad_value(521)), 1.0), (-1.0));}
        s.b[1531] = (s.v[285] > 0.0);s.store_scalar(1531, if s.b[1531] { 1.0 } else { 0.0 });s.b[1532] = ((p[1647] - s.v[521]) < (p[1647] * 0.001));s.store_scalar(1532, if s.b[1532] { 1.0 } else { 0.0 });
        if ((s.b[1523] && s.b[1531]) && s.b[1532]) {s.store_div_scaled_value_by_product_indices(168, 521, -1.0, 180, 291, 1.0);s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));}
        if ((s.b[1523] && s.b[1531]) && (!s.b[1532])) {s.store_div_scaled_value_by_product_indices(168, 521, -1.0, 180, 291, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_111(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1523] && s.b[1531]) && (!s.b[1532])) {s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p[1647], A::sub_from_scalar(p[1647], s.ad_value(521)), 1.0), (-1.0));}
        s.b[1533] = (s.v[538] > 0.0);s.store_scalar(1533, if s.b[1533] { 1.0 } else { 0.0 });s.b[1534] = (s.v[522] < s.v[550]);s.store_scalar(1534, if s.b[1534] { 1.0 } else { 0.0 });
        if ((s.b[1523] && s.b[1533]) && s.b[1534]) {s.store_div(168, 522, 540);s.store_offset_limited_exp(169, 168, (-1.0));s.store_add_scaled_product_right_sub(170, 549, 1.0, 548, 522, 550, 1.0);}
        s.b[1535] = (s.v[522] <= s.v[553]);s.store_scalar(1535, if s.b[1535] { 1.0 } else { 0.0 });
        if (((s.b[1523] && s.b[1533]) && (!s.b[1534])) && s.b[1535]) {s.store_div(168, 522, 540);s.store_div_scaled_offset_numerator_indices(169, 522, 1.0, p[1627], 540, 1.0);s.store_limited_exp_neg_input(170, 169);}
        s.b[1536] = (s.v[282] > 0.0);s.store_scalar(1536, if s.b[1536] { 1.0 } else { 0.0 });s.b[1537] = ((p[1644] - s.v[522]) < (p[1644] * 0.001));s.store_scalar(1537, if s.b[1537] { 1.0 } else { 0.0 });
        if ((s.b[1523] && s.b[1536]) && s.b[1537]) {s.store_div_scaled_value_by_product_indices(168, 522, -1.0, 180, 288, 1.0);s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));}
        if ((s.b[1523] && s.b[1536]) && (!s.b[1537])) {s.store_div_scaled_value_by_product_indices(168, 522, -1.0, 180, 288, 1.0);s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p[1644], A::sub_from_scalar(p[1644], s.ad_value(522)), 1.0), (-1.0));}
        s.b[1538] = (s.v[284] > 0.0);s.store_scalar(1538, if s.b[1538] { 1.0 } else { 0.0 });s.b[1539] = ((p[1646] - s.v[522]) < (p[1646] * 0.001));s.store_scalar(1539, if s.b[1539] { 1.0 } else { 0.0 });
        if ((s.b[1523] && s.b[1538]) && s.b[1539]) {s.store_div_scaled_value_by_product_indices(168, 522, -1.0, 180, 290, 1.0);s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));}
        if ((s.b[1523] && s.b[1538]) && (!s.b[1539])) {s.store_div_scaled_value_by_product_indices(168, 522, -1.0, 180, 290, 1.0);s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p[1646], A::sub_from_scalar(p[1646], s.ad_value(522)), 1.0), (-1.0));}
        s.b[1540] = (s.v[286] > 0.0);s.store_scalar(1540, if s.b[1540] { 1.0 } else { 0.0 });s.b[1541] = ((p[1648] - s.v[522]) < (p[1648] * 0.001));s.store_scalar(1541, if s.b[1541] { 1.0 } else { 0.0 });
        if ((s.b[1523] && s.b[1540]) && s.b[1541]) {s.store_div_scaled_value_by_product_indices(168, 522, -1.0, 180, 292, 1.0);s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));}
        if ((s.b[1523] && s.b[1540]) && (!s.b[1541])) {s.store_div_scaled_value_by_product_indices(168, 522, -1.0, 180, 292, 1.0);s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p[1648], A::sub_from_scalar(p[1648], s.ad_value(522)), 1.0), (-1.0));}
        s.b[1550] = (s.v[523] > 0.0);s.store_scalar(1550, if s.b[1550] { 1.0 } else { 0.0 });
        if (s.b[1523] && s.b[1550]) {s.store_div(1542, 521, 269);}
        s.b[1551] = (s.v[1542] < 0.9);s.store_scalar(1551, if s.b[1551] { 1.0 } else { 0.0 });s.b[1552] = (p[1602] > 0.0);s.store_scalar(1552, if s.b[1552] { 1.0 } else { 0.0 });s.b[1553] = (s.v[521] > s.v[557]);s.store_scalar(1553, if s.b[1553] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && s.b[1553]) {s.store_sub_from_scalar(1547, 1.0, 1542);}
        s.b[1554] = (p[1596] != 1.0);s.store_scalar(1554, if s.b[1554] { 1.0 } else { 0.0 });s.b[1555] = (p[1596] == 0.5);s.store_scalar(1555, if s.b[1555] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && s.b[1553]) && s.b[1554]) && s.b[1555]) {s.store_div_from_scalar_sqrt_ad(1548, 1.0, s.ad_value(1547));}
        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && s.b[1553]) && s.b[1554]) && (!s.b[1555])) {s.store_powf(1548, 1547, (-p[1596]));}
        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && s.b[1553]) && s.b[1554]) {s.store_mul_ad_affine_product_rhs(530, 269, s.ad_value(523), A::sub_from_scalar(1.0, A::mul(s.ad_value(1547), s.ad_value(1548))), 1.0 / ((1.0 - p[1596])), 0.0);}
        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && s.b[1553]) && (!s.b[1554])) {
            s.store_mul_ad_affine_product_rhs(530, 269, s.ad_value(523), {
                if (!(s.v[1547] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1547] > 1e-38) {
                            A::ln(s.ad_value(1547))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        if ((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) {s.store_sub_from_scalar_div_indices(1547, 1.0, 557, 269);}
        s.b[1556] = (p[1596] != 1.0);s.store_scalar(1556, if s.b[1556] { 1.0 } else { 0.0 });s.b[1557] = (p[1596] == 0.5);s.store_scalar(1557, if s.b[1557] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1556]) && s.b[1557]) {s.store_div_from_scalar_sqrt_ad(1548, 1.0, s.ad_value(1547));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_112(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1556]) && (!s.b[1557])) {s.store_powf(1548, 1547, (-p[1596]));}
        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1556]) {s.store_mul_ad_affine_product_rhs(1549, 269, s.ad_value(523), A::sub_from_scalar(1.0, A::mul(s.ad_value(1547), s.ad_value(1548))), 1.0 / ((1.0 - p[1596])), 0.0);}
        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && (!s.b[1556])) {
            s.store_mul_ad_affine_product_rhs(1549, 269, s.ad_value(523), {
                if (!(s.v[1547] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1547] > 1e-38) {
                            A::ln(s.ad_value(1547))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        if ((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) {s.store_sub_from_scalar_ad(1547, 1.0, A::div_scaled_inputs2(s.ad_value(521), 1.0, s.ad_value(557), (-1.0), s.ad_value(558), 1.0));}
        s.b[1558] = (p[1608] != 1.0);s.store_scalar(1558, if s.b[1558] { 1.0 } else { 0.0 });s.b[1559] = (p[1608] == 0.5);s.store_scalar(1559, if s.b[1559] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1558]) && s.b[1559]) {s.store_div_from_scalar_sqrt_ad(1548, 1.0, s.ad_value(1547));}
        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1558]) && (!s.b[1559])) {s.store_powf(1548, 1547, (-p[1608]));}
        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1558]) {s.store_add_product3_rhs_mixed_iia(530, 1549, 558, 523, A::sub_from_scalar(1.0, A::mul(s.ad_value(1547), s.ad_value(1548))), (p[1602] * 1.0 / ((1.0 - p[1608]))));}
        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && (!s.b[1558])) {
            s.store_sub_mixed_ia(530, 1549, A::mul3_scaled_output(s.ad_value(558), s.ad_value(523), {
                            if (!(s.v[1547] > 1e-38)) {
                                A::neg(A::constant(87.498233534))
                            } else {
                                {
                                    if (s.v[1547] > 1e-38) {
                                        A::ln(s.ad_value(1547))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        }, p[1602]));
        }
        if (((s.b[1523] && s.b[1550]) && s.b[1551]) && (!s.b[1552])) {s.store_sub_from_scalar(1547, 1.0, 1542);}
        s.b[1560] = (p[1596] != 1.0);s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });s.b[1561] = (p[1596] == 0.5);s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });
        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && (!s.b[1552])) && s.b[1560]) && s.b[1561]) {s.store_div_from_scalar_sqrt_ad(1548, 1.0, s.ad_value(1547));}
        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && (!s.b[1552])) && s.b[1560]) && (!s.b[1561])) {s.store_powf(1548, 1547, (-p[1596]));}
        if ((((s.b[1523] && s.b[1550]) && s.b[1551]) && (!s.b[1552])) && s.b[1560]) {s.store_mul_ad_affine_product_rhs(530, 269, s.ad_value(523), A::sub_from_scalar(1.0, A::mul(s.ad_value(1547), s.ad_value(1548))), 1.0 / ((1.0 - p[1596])), 0.0);}
        if ((((s.b[1523] && s.b[1550]) && s.b[1551]) && (!s.b[1552])) && (!s.b[1560])) {
            s.store_mul_ad_affine_product_rhs(530, 269, s.ad_value(523), {
                if (!(s.v[1547] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1547] > 1e-38) {
                            A::ln(s.ad_value(1547))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        s.b[1562] = (p[1596] != 1.0);s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });s.b[1563] = (p[1596] == 0.5);s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1550]) && (!s.b[1551])) && s.b[1562]) && s.b[1563]) {s.store_scalar(1543, (1.0 / ((0.1) as f64).sqrt()));}
        if ((((s.b[1523] && s.b[1550]) && (!s.b[1551])) && s.b[1562]) && (!s.b[1563])) {s.store_scalar(1543, ((0.1) as f64).powf((-p[1596])));}
        if (((s.b[1523] && s.b[1550]) && (!s.b[1551])) && s.b[1562]) {s.store_scalar(1544, (1.0 / (1.0 - p[1596])));s.store_primal_mul_scale_offset_mixed_ia(1546, 1544, A::scale(s.ad_value(1543), ((0.05 * p[1596]) * (1.0 + p[1596]))), -1.0, 1.0);}
        if (((s.b[1523] && s.b[1550]) && (!s.b[1551])) && (!s.b[1562])) {s.store_scalar(1543, 10.0);s.store_scalar(1546, (1.5 - ((0.1) as f64).ln()));}
        if ((s.b[1523] && s.b[1550]) && (!s.b[1551])) {s.store_mul_ad_product_rhs(1545, 1543, A::offset(s.ad_value(1542), (-1.0)), A::scale_offset(s.ad_value(1542), (5.0 * p[1596]), (((((-1.0)) * ((5.0 * p[1596])))) + ((1.0 + p[1596])))));s.store_mul_ad_product_rhs_mixed_ia(530, 269, 523, A::add(s.ad_value(1545), s.ad_value(1546)));}
        if (s.b[1523] && (!s.b[1550])) {s.store_scalar(530, 0.0);}
        s.b[1572] = (s.v[524] > 0.0);s.store_scalar(1572, if s.b[1572] { 1.0 } else { 0.0 });
        if (s.b[1523] && s.b[1572]) {s.store_div(1564, 521, 270);}
        s.b[1573] = (s.v[1564] < 0.9);s.store_scalar(1573, if s.b[1573] { 1.0 } else { 0.0 });s.b[1574] = (p[1604] > 0.0);s.store_scalar(1574, if s.b[1574] { 1.0 } else { 0.0 });s.b[1575] = (s.v[521] > s.v[559]);s.store_scalar(1575, if s.b[1575] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && s.b[1575]) {s.store_sub_from_scalar(1569, 1.0, 1564);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_113(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1576] = (p[1598] != 1.0);s.store_scalar(1576, if s.b[1576] { 1.0 } else { 0.0 });s.b[1577] = (p[1598] == 0.5);s.store_scalar(1577, if s.b[1577] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && s.b[1575]) && s.b[1576]) && s.b[1577]) {s.store_div_from_scalar_sqrt_ad(1570, 1.0, s.ad_value(1569));}
        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && s.b[1575]) && s.b[1576]) && (!s.b[1577])) {s.store_powf(1570, 1569, (-p[1598]));}
        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && s.b[1575]) && s.b[1576]) {s.store_mul_ad_affine_product_rhs(531, 270, s.ad_value(524), A::sub_from_scalar(1.0, A::mul(s.ad_value(1569), s.ad_value(1570))), 1.0 / ((1.0 - p[1598])), 0.0);}
        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && s.b[1575]) && (!s.b[1576])) {
            s.store_mul_ad_affine_product_rhs(531, 270, s.ad_value(524), {
                if (!(s.v[1569] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1569] > 1e-38) {
                            A::ln(s.ad_value(1569))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        if ((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) {s.store_sub_from_scalar_div_indices(1569, 1.0, 559, 270);}
        s.b[1578] = (p[1598] != 1.0);s.store_scalar(1578, if s.b[1578] { 1.0 } else { 0.0 });s.b[1579] = (p[1598] == 0.5);s.store_scalar(1579, if s.b[1579] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1578]) && s.b[1579]) {s.store_div_from_scalar_sqrt_ad(1570, 1.0, s.ad_value(1569));}
        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1578]) && (!s.b[1579])) {s.store_powf(1570, 1569, (-p[1598]));}
        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1578]) {s.store_mul_ad_affine_product_rhs(1571, 270, s.ad_value(524), A::sub_from_scalar(1.0, A::mul(s.ad_value(1569), s.ad_value(1570))), 1.0 / ((1.0 - p[1598])), 0.0);}
        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && (!s.b[1578])) {
            s.store_mul_ad_affine_product_rhs(1571, 270, s.ad_value(524), {
                if (!(s.v[1569] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1569] > 1e-38) {
                            A::ln(s.ad_value(1569))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        if ((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) {s.store_sub_from_scalar_ad(1569, 1.0, A::div_scaled_inputs2(s.ad_value(521), 1.0, s.ad_value(559), (-1.0), s.ad_value(560), 1.0));}
        s.b[1580] = (p[1610] != 1.0);s.store_scalar(1580, if s.b[1580] { 1.0 } else { 0.0 });s.b[1581] = (p[1610] == 0.5);s.store_scalar(1581, if s.b[1581] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1580]) && s.b[1581]) {s.store_div_from_scalar_sqrt_ad(1570, 1.0, s.ad_value(1569));}
        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1580]) && (!s.b[1581])) {s.store_powf(1570, 1569, (-p[1610]));}
        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1580]) {s.store_add_product3_rhs_mixed_iia(531, 1571, 560, 524, A::sub_from_scalar(1.0, A::mul(s.ad_value(1569), s.ad_value(1570))), (p[1604] * 1.0 / ((1.0 - p[1610]))));}
        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && (!s.b[1580])) {
            s.store_sub_mixed_ia(531, 1571, A::mul3_scaled_output(s.ad_value(560), s.ad_value(524), {
                            if (!(s.v[1569] > 1e-38)) {
                                A::neg(A::constant(87.498233534))
                            } else {
                                {
                                    if (s.v[1569] > 1e-38) {
                                        A::ln(s.ad_value(1569))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        }, p[1604]));
        }
        if (((s.b[1523] && s.b[1572]) && s.b[1573]) && (!s.b[1574])) {s.store_sub_from_scalar(1569, 1.0, 1564);}
        s.b[1582] = (p[1598] != 1.0);s.store_scalar(1582, if s.b[1582] { 1.0 } else { 0.0 });s.b[1583] = (p[1598] == 0.5);s.store_scalar(1583, if s.b[1583] { 1.0 } else { 0.0 });
        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && (!s.b[1574])) && s.b[1582]) && s.b[1583]) {s.store_div_from_scalar_sqrt_ad(1570, 1.0, s.ad_value(1569));}
        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && (!s.b[1574])) && s.b[1582]) && (!s.b[1583])) {s.store_powf(1570, 1569, (-p[1598]));}
        if ((((s.b[1523] && s.b[1572]) && s.b[1573]) && (!s.b[1574])) && s.b[1582]) {s.store_mul_ad_affine_product_rhs(531, 270, s.ad_value(524), A::sub_from_scalar(1.0, A::mul(s.ad_value(1569), s.ad_value(1570))), 1.0 / ((1.0 - p[1598])), 0.0);}
        if ((((s.b[1523] && s.b[1572]) && s.b[1573]) && (!s.b[1574])) && (!s.b[1582])) {
            s.store_mul_ad_affine_product_rhs(531, 270, s.ad_value(524), {
                if (!(s.v[1569] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1569] > 1e-38) {
                            A::ln(s.ad_value(1569))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        s.b[1584] = (p[1598] != 1.0);s.store_scalar(1584, if s.b[1584] { 1.0 } else { 0.0 });s.b[1585] = (p[1598] == 0.5);s.store_scalar(1585, if s.b[1585] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1572]) && (!s.b[1573])) && s.b[1584]) && s.b[1585]) {s.store_scalar(1565, (1.0 / ((0.1) as f64).sqrt()));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_114(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1523] && s.b[1572]) && (!s.b[1573])) && s.b[1584]) && (!s.b[1585])) {s.store_scalar(1565, ((0.1) as f64).powf((-p[1598])));}
        if (((s.b[1523] && s.b[1572]) && (!s.b[1573])) && s.b[1584]) {s.store_scalar(1566, (1.0 / (1.0 - p[1598])));s.store_primal_mul_scale_offset_mixed_ia(1568, 1566, A::scale(s.ad_value(1565), ((0.05 * p[1598]) * (1.0 + p[1598]))), -1.0, 1.0);}
        if (((s.b[1523] && s.b[1572]) && (!s.b[1573])) && (!s.b[1584])) {s.store_scalar(1565, 10.0);s.store_scalar(1568, (1.5 - ((0.1) as f64).ln()));}
        if ((s.b[1523] && s.b[1572]) && (!s.b[1573])) {s.store_mul_ad_product_rhs(1567, 1565, A::offset(s.ad_value(1564), (-1.0)), A::scale_offset(s.ad_value(1564), (5.0 * p[1598]), (((((-1.0)) * ((5.0 * p[1598])))) + ((1.0 + p[1598])))));s.store_mul_ad_product_rhs_mixed_ia(531, 270, 524, A::add(s.ad_value(1567), s.ad_value(1568)));}
        if (s.b[1523] && (!s.b[1572])) {s.store_scalar(531, 0.0);}
        s.b[1594] = (s.v[525] > 0.0);s.store_scalar(1594, if s.b[1594] { 1.0 } else { 0.0 });
        if (s.b[1523] && s.b[1594]) {s.store_div(1586, 521, 271);}
        s.b[1595] = (s.v[1586] < 0.9);s.store_scalar(1595, if s.b[1595] { 1.0 } else { 0.0 });s.b[1596] = (p[1606] > 0.0);s.store_scalar(1596, if s.b[1596] { 1.0 } else { 0.0 });s.b[1597] = (s.v[521] > s.v[561]);s.store_scalar(1597, if s.b[1597] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && s.b[1597]) {s.store_sub_from_scalar(1591, 1.0, 1586);}
        s.b[1598] = (p[1600] != 1.0);s.store_scalar(1598, if s.b[1598] { 1.0 } else { 0.0 });s.b[1599] = (p[1600] == 0.5);s.store_scalar(1599, if s.b[1599] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && s.b[1597]) && s.b[1598]) && s.b[1599]) {s.store_div_from_scalar_sqrt_ad(1592, 1.0, s.ad_value(1591));}
        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && s.b[1597]) && s.b[1598]) && (!s.b[1599])) {s.store_powf(1592, 1591, (-p[1600]));}
        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && s.b[1597]) && s.b[1598]) {s.store_mul_ad_affine_product_rhs(532, 271, s.ad_value(525), A::sub_from_scalar(1.0, A::mul(s.ad_value(1591), s.ad_value(1592))), 1.0 / ((1.0 - p[1600])), 0.0);}
        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && s.b[1597]) && (!s.b[1598])) {
            s.store_mul_ad_affine_product_rhs(532, 271, s.ad_value(525), {
                if (!(s.v[1591] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1591] > 1e-38) {
                            A::ln(s.ad_value(1591))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        if ((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) {s.store_sub_from_scalar_div_indices(1591, 1.0, 561, 271);}
        s.b[1600] = (p[1600] != 1.0);s.store_scalar(1600, if s.b[1600] { 1.0 } else { 0.0 });s.b[1601] = (p[1600] == 0.5);s.store_scalar(1601, if s.b[1601] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1600]) && s.b[1601]) {s.store_div_from_scalar_sqrt_ad(1592, 1.0, s.ad_value(1591));}
        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1600]) && (!s.b[1601])) {s.store_powf(1592, 1591, (-p[1600]));}
        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1600]) {s.store_mul_ad_affine_product_rhs(1593, 271, s.ad_value(525), A::sub_from_scalar(1.0, A::mul(s.ad_value(1591), s.ad_value(1592))), 1.0 / ((1.0 - p[1600])), 0.0);}
        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && (!s.b[1600])) {
            s.store_mul_ad_affine_product_rhs(1593, 271, s.ad_value(525), {
                if (!(s.v[1591] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1591] > 1e-38) {
                            A::ln(s.ad_value(1591))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        if ((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) {s.store_sub_from_scalar_ad(1591, 1.0, A::div_scaled_inputs2(s.ad_value(521), 1.0, s.ad_value(561), (-1.0), s.ad_value(562), 1.0));}
        s.b[1602] = (p[1612] != 1.0);s.store_scalar(1602, if s.b[1602] { 1.0 } else { 0.0 });s.b[1603] = (p[1612] == 0.5);s.store_scalar(1603, if s.b[1603] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1602]) && s.b[1603]) {s.store_div_from_scalar_sqrt_ad(1592, 1.0, s.ad_value(1591));}
        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1602]) && (!s.b[1603])) {s.store_powf(1592, 1591, (-p[1612]));}
        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1602]) {s.store_add_product3_rhs_mixed_iia(532, 1593, 562, 525, A::sub_from_scalar(1.0, A::mul(s.ad_value(1591), s.ad_value(1592))), (p[1606] * 1.0 / ((1.0 - p[1612]))));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_115(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && (!s.b[1602])) {
            s.store_sub_mixed_ia(532, 1593, A::mul3_scaled_output(s.ad_value(562), s.ad_value(525), {
                            if (!(s.v[1591] > 1e-38)) {
                                A::neg(A::constant(87.498233534))
                            } else {
                                {
                                    if (s.v[1591] > 1e-38) {
                                        A::ln(s.ad_value(1591))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        }, p[1606]));
        }
        if (((s.b[1523] && s.b[1594]) && s.b[1595]) && (!s.b[1596])) {s.store_sub_from_scalar(1591, 1.0, 1586);}
        s.b[1604] = (p[1600] != 1.0);s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });s.b[1605] = (p[1600] == 0.5);s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });
        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && s.b[1604]) && s.b[1605]) {s.store_div_from_scalar_sqrt_ad(1592, 1.0, s.ad_value(1591));}
        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && s.b[1604]) && (!s.b[1605])) {s.store_powf(1592, 1591, (-p[1600]));}
        if ((((s.b[1523] && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && s.b[1604]) {s.store_mul_ad_affine_product_rhs(532, 271, s.ad_value(525), A::sub_from_scalar(1.0, A::mul(s.ad_value(1591), s.ad_value(1592))), 1.0 / ((1.0 - p[1600])), 0.0);}
        if ((((s.b[1523] && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && (!s.b[1604])) {
            s.store_mul_ad_affine_product_rhs(532, 271, s.ad_value(525), {
                if (!(s.v[1591] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1591] > 1e-38) {
                            A::ln(s.ad_value(1591))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        s.b[1606] = (p[1600] != 1.0);s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });s.b[1607] = (p[1600] == 0.5);s.store_scalar(1607, if s.b[1607] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1594]) && (!s.b[1595])) && s.b[1606]) && s.b[1607]) {s.store_scalar(1587, (1.0 / ((0.1) as f64).sqrt()));}
        if ((((s.b[1523] && s.b[1594]) && (!s.b[1595])) && s.b[1606]) && (!s.b[1607])) {s.store_scalar(1587, ((0.1) as f64).powf((-p[1600])));}
        if (((s.b[1523] && s.b[1594]) && (!s.b[1595])) && s.b[1606]) {s.store_scalar(1588, (1.0 / (1.0 - p[1600])));s.store_primal_mul_scale_offset_mixed_ia(1590, 1588, A::scale(s.ad_value(1587), ((0.05 * p[1600]) * (1.0 + p[1600]))), -1.0, 1.0);}
        if (((s.b[1523] && s.b[1594]) && (!s.b[1595])) && (!s.b[1606])) {s.store_scalar(1587, 10.0);s.store_scalar(1590, (1.5 - ((0.1) as f64).ln()));}
        if ((s.b[1523] && s.b[1594]) && (!s.b[1595])) {s.store_mul_ad_product_rhs(1589, 1587, A::offset(s.ad_value(1586), (-1.0)), A::scale_offset(s.ad_value(1586), (5.0 * p[1600]), (((((-1.0)) * ((5.0 * p[1600])))) + ((1.0 + p[1600])))));s.store_mul_ad_product_rhs_mixed_ia(532, 271, 525, A::add(s.ad_value(1589), s.ad_value(1590)));}
        if (s.b[1523] && (!s.b[1594])) {s.store_scalar(532, 0.0);}
        if s.b[1523] {s.store_add_scaled_inputs3_indices(529, 530, 1.0, 531, 1.0, 532, 1.0);}
        s.b[1616] = (s.v[526] > 0.0);s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });
        if (s.b[1523] && s.b[1616]) {s.store_div(1608, 522, 272);}
        s.b[1617] = (s.v[1608] < 0.9);s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });s.b[1618] = (p[1603] > 0.0);s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });s.b[1619] = (s.v[522] > s.v[563]);s.store_scalar(1619, if s.b[1619] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) {s.store_sub_from_scalar(1613, 1.0, 1608);}
        s.b[1620] = (p[1597] != 1.0);s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });s.b[1621] = (p[1597] == 0.5);s.store_scalar(1621, if s.b[1621] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) && s.b[1620]) && s.b[1621]) {s.store_div_from_scalar_sqrt_ad(1614, 1.0, s.ad_value(1613));}
        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) && s.b[1620]) && (!s.b[1621])) {s.store_powf(1614, 1613, (-p[1597]));}
        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) && s.b[1620]) {s.store_mul_ad_affine_product_rhs(534, 272, s.ad_value(526), A::sub_from_scalar(1.0, A::mul(s.ad_value(1613), s.ad_value(1614))), 1.0 / ((1.0 - p[1597])), 0.0);}
        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) && (!s.b[1620])) {
            s.store_mul_ad_affine_product_rhs(534, 272, s.ad_value(526), {
                if (!(s.v[1613] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1613] > 1e-38) {
                            A::ln(s.ad_value(1613))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        if ((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) {s.store_sub_from_scalar_div_indices(1613, 1.0, 563, 272);}
        s.b[1622] = (p[1597] != 1.0);s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });s.b[1623] = (p[1597] == 0.5);s.store_scalar(1623, if s.b[1623] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1622]) && s.b[1623]) {s.store_div_from_scalar_sqrt_ad(1614, 1.0, s.ad_value(1613));}
        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1622]) && (!s.b[1623])) {s.store_powf(1614, 1613, (-p[1597]));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_116(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1622]) {s.store_mul_ad_affine_product_rhs(1615, 272, s.ad_value(526), A::sub_from_scalar(1.0, A::mul(s.ad_value(1613), s.ad_value(1614))), 1.0 / ((1.0 - p[1597])), 0.0);}
        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && (!s.b[1622])) {
            s.store_mul_ad_affine_product_rhs(1615, 272, s.ad_value(526), {
                if (!(s.v[1613] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1613] > 1e-38) {
                            A::ln(s.ad_value(1613))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        if ((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) {s.store_sub_from_scalar_ad(1613, 1.0, A::div_scaled_inputs2(s.ad_value(522), 1.0, s.ad_value(563), (-1.0), s.ad_value(564), 1.0));}
        s.b[1624] = (p[1609] != 1.0);s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });s.b[1625] = (p[1609] == 0.5);s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1624]) && s.b[1625]) {s.store_div_from_scalar_sqrt_ad(1614, 1.0, s.ad_value(1613));}
        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1624]) && (!s.b[1625])) {s.store_powf(1614, 1613, (-p[1609]));}
        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1624]) {s.store_add_product3_rhs_mixed_iia(534, 1615, 564, 526, A::sub_from_scalar(1.0, A::mul(s.ad_value(1613), s.ad_value(1614))), (p[1603] * 1.0 / ((1.0 - p[1609]))));}
        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && (!s.b[1624])) {
            s.store_sub_mixed_ia(534, 1615, A::mul3_scaled_output(s.ad_value(564), s.ad_value(526), {
                            if (!(s.v[1613] > 1e-38)) {
                                A::neg(A::constant(87.498233534))
                            } else {
                                {
                                    if (s.v[1613] > 1e-38) {
                                        A::ln(s.ad_value(1613))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        }, p[1603]));
        }
        if (((s.b[1523] && s.b[1616]) && s.b[1617]) && (!s.b[1618])) {s.store_sub_from_scalar(1613, 1.0, 1608);}
        s.b[1626] = (p[1597] != 1.0);s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });s.b[1627] = (p[1597] == 0.5);s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });
        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && (!s.b[1618])) && s.b[1626]) && s.b[1627]) {s.store_div_from_scalar_sqrt_ad(1614, 1.0, s.ad_value(1613));}
        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && (!s.b[1618])) && s.b[1626]) && (!s.b[1627])) {s.store_powf(1614, 1613, (-p[1597]));}
        if ((((s.b[1523] && s.b[1616]) && s.b[1617]) && (!s.b[1618])) && s.b[1626]) {s.store_mul_ad_affine_product_rhs(534, 272, s.ad_value(526), A::sub_from_scalar(1.0, A::mul(s.ad_value(1613), s.ad_value(1614))), 1.0 / ((1.0 - p[1597])), 0.0);}
        if ((((s.b[1523] && s.b[1616]) && s.b[1617]) && (!s.b[1618])) && (!s.b[1626])) {
            s.store_mul_ad_affine_product_rhs(534, 272, s.ad_value(526), {
                if (!(s.v[1613] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1613] > 1e-38) {
                            A::ln(s.ad_value(1613))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        s.b[1628] = (p[1597] != 1.0);s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });s.b[1629] = (p[1597] == 0.5);s.store_scalar(1629, if s.b[1629] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1616]) && (!s.b[1617])) && s.b[1628]) && s.b[1629]) {s.store_scalar(1609, (1.0 / ((0.1) as f64).sqrt()));}
        if ((((s.b[1523] && s.b[1616]) && (!s.b[1617])) && s.b[1628]) && (!s.b[1629])) {s.store_scalar(1609, ((0.1) as f64).powf((-p[1597])));}
        if (((s.b[1523] && s.b[1616]) && (!s.b[1617])) && s.b[1628]) {s.store_scalar(1610, (1.0 / (1.0 - p[1597])));s.store_primal_mul_scale_offset_mixed_ia(1612, 1610, A::scale(s.ad_value(1609), ((0.05 * p[1597]) * (1.0 + p[1597]))), -1.0, 1.0);}
        if (((s.b[1523] && s.b[1616]) && (!s.b[1617])) && (!s.b[1628])) {s.store_scalar(1609, 10.0);s.store_scalar(1612, (1.5 - ((0.1) as f64).ln()));}
        if ((s.b[1523] && s.b[1616]) && (!s.b[1617])) {s.store_mul_ad_product_rhs(1611, 1609, A::offset(s.ad_value(1608), (-1.0)), A::scale_offset(s.ad_value(1608), (5.0 * p[1597]), (((((-1.0)) * ((5.0 * p[1597])))) + ((1.0 + p[1597])))));s.store_mul_ad_product_rhs_mixed_ia(534, 272, 526, A::add(s.ad_value(1611), s.ad_value(1612)));}
        if (s.b[1523] && (!s.b[1616])) {s.store_scalar(534, 0.0);}
        s.b[1638] = (s.v[527] > 0.0);s.store_scalar(1638, if s.b[1638] { 1.0 } else { 0.0 });
        if (s.b[1523] && s.b[1638]) {s.store_div(1630, 522, 273);}
        s.b[1639] = (s.v[1630] < 0.9);s.store_scalar(1639, if s.b[1639] { 1.0 } else { 0.0 });s.b[1640] = (p[1605] > 0.0);s.store_scalar(1640, if s.b[1640] { 1.0 } else { 0.0 });s.b[1641] = (s.v[522] > s.v[565]);s.store_scalar(1641, if s.b[1641] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && s.b[1641]) {s.store_sub_from_scalar(1635, 1.0, 1630);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_117(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1642] = (p[1599] != 1.0);s.store_scalar(1642, if s.b[1642] { 1.0 } else { 0.0 });s.b[1643] = (p[1599] == 0.5);s.store_scalar(1643, if s.b[1643] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && s.b[1641]) && s.b[1642]) && s.b[1643]) {s.store_div_from_scalar_sqrt_ad(1636, 1.0, s.ad_value(1635));}
        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && s.b[1641]) && s.b[1642]) && (!s.b[1643])) {s.store_powf(1636, 1635, (-p[1599]));}
        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && s.b[1641]) && s.b[1642]) {s.store_mul_ad_affine_product_rhs(535, 273, s.ad_value(527), A::sub_from_scalar(1.0, A::mul(s.ad_value(1635), s.ad_value(1636))), 1.0 / ((1.0 - p[1599])), 0.0);}
        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && s.b[1641]) && (!s.b[1642])) {
            s.store_mul_ad_affine_product_rhs(535, 273, s.ad_value(527), {
                if (!(s.v[1635] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1635] > 1e-38) {
                            A::ln(s.ad_value(1635))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        if ((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) {s.store_sub_from_scalar_div_indices(1635, 1.0, 565, 273);}
        s.b[1644] = (p[1599] != 1.0);s.store_scalar(1644, if s.b[1644] { 1.0 } else { 0.0 });s.b[1645] = (p[1599] == 0.5);s.store_scalar(1645, if s.b[1645] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1644]) && s.b[1645]) {s.store_div_from_scalar_sqrt_ad(1636, 1.0, s.ad_value(1635));}
        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1644]) && (!s.b[1645])) {s.store_powf(1636, 1635, (-p[1599]));}
        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1644]) {s.store_mul_ad_affine_product_rhs(1637, 273, s.ad_value(527), A::sub_from_scalar(1.0, A::mul(s.ad_value(1635), s.ad_value(1636))), 1.0 / ((1.0 - p[1599])), 0.0);}
        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && (!s.b[1644])) {
            s.store_mul_ad_affine_product_rhs(1637, 273, s.ad_value(527), {
                if (!(s.v[1635] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1635] > 1e-38) {
                            A::ln(s.ad_value(1635))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        if ((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) {s.store_sub_from_scalar_ad(1635, 1.0, A::div_scaled_inputs2(s.ad_value(522), 1.0, s.ad_value(565), (-1.0), s.ad_value(566), 1.0));}
        s.b[1646] = (p[1611] != 1.0);s.store_scalar(1646, if s.b[1646] { 1.0 } else { 0.0 });s.b[1647] = (p[1611] == 0.5);s.store_scalar(1647, if s.b[1647] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1646]) && s.b[1647]) {s.store_div_from_scalar_sqrt_ad(1636, 1.0, s.ad_value(1635));}
        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1646]) && (!s.b[1647])) {s.store_powf(1636, 1635, (-p[1611]));}
        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1646]) {s.store_add_product3_rhs_mixed_iia(535, 1637, 566, 527, A::sub_from_scalar(1.0, A::mul(s.ad_value(1635), s.ad_value(1636))), (p[1605] * 1.0 / ((1.0 - p[1611]))));}
        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && (!s.b[1646])) {
            s.store_sub_mixed_ia(535, 1637, A::mul3_scaled_output(s.ad_value(566), s.ad_value(527), {
                            if (!(s.v[1635] > 1e-38)) {
                                A::neg(A::constant(87.498233534))
                            } else {
                                {
                                    if (s.v[1635] > 1e-38) {
                                        A::ln(s.ad_value(1635))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        }, p[1605]));
        }
        if (((s.b[1523] && s.b[1638]) && s.b[1639]) && (!s.b[1640])) {s.store_sub_from_scalar(1635, 1.0, 1630);}
        s.b[1648] = (p[1599] != 1.0);s.store_scalar(1648, if s.b[1648] { 1.0 } else { 0.0 });s.b[1649] = (p[1599] == 0.5);s.store_scalar(1649, if s.b[1649] { 1.0 } else { 0.0 });
        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && (!s.b[1640])) && s.b[1648]) && s.b[1649]) {s.store_div_from_scalar_sqrt_ad(1636, 1.0, s.ad_value(1635));}
        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && (!s.b[1640])) && s.b[1648]) && (!s.b[1649])) {s.store_powf(1636, 1635, (-p[1599]));}
        if ((((s.b[1523] && s.b[1638]) && s.b[1639]) && (!s.b[1640])) && s.b[1648]) {s.store_mul_ad_affine_product_rhs(535, 273, s.ad_value(527), A::sub_from_scalar(1.0, A::mul(s.ad_value(1635), s.ad_value(1636))), 1.0 / ((1.0 - p[1599])), 0.0);}
        if ((((s.b[1523] && s.b[1638]) && s.b[1639]) && (!s.b[1640])) && (!s.b[1648])) {
            s.store_mul_ad_affine_product_rhs(535, 273, s.ad_value(527), {
                if (!(s.v[1635] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1635] > 1e-38) {
                            A::ln(s.ad_value(1635))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        s.b[1650] = (p[1599] != 1.0);s.store_scalar(1650, if s.b[1650] { 1.0 } else { 0.0 });s.b[1651] = (p[1599] == 0.5);s.store_scalar(1651, if s.b[1651] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1638]) && (!s.b[1639])) && s.b[1650]) && s.b[1651]) {s.store_scalar(1631, (1.0 / ((0.1) as f64).sqrt()));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_118(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1523] && s.b[1638]) && (!s.b[1639])) && s.b[1650]) && (!s.b[1651])) {s.store_scalar(1631, ((0.1) as f64).powf((-p[1599])));}
        if (((s.b[1523] && s.b[1638]) && (!s.b[1639])) && s.b[1650]) {s.store_scalar(1632, (1.0 / (1.0 - p[1599])));s.store_primal_mul_scale_offset_mixed_ia(1634, 1632, A::scale(s.ad_value(1631), ((0.05 * p[1599]) * (1.0 + p[1599]))), -1.0, 1.0);}
        if (((s.b[1523] && s.b[1638]) && (!s.b[1639])) && (!s.b[1650])) {s.store_scalar(1631, 10.0);s.store_scalar(1634, (1.5 - ((0.1) as f64).ln()));}
        if ((s.b[1523] && s.b[1638]) && (!s.b[1639])) {s.store_mul_ad_product_rhs(1633, 1631, A::offset(s.ad_value(1630), (-1.0)), A::scale_offset(s.ad_value(1630), (5.0 * p[1599]), (((((-1.0)) * ((5.0 * p[1599])))) + ((1.0 + p[1599])))));s.store_mul_ad_product_rhs_mixed_ia(535, 273, 527, A::add(s.ad_value(1633), s.ad_value(1634)));}
        if (s.b[1523] && (!s.b[1638])) {s.store_scalar(535, 0.0);}
        s.b[1660] = (s.v[528] > 0.0);s.store_scalar(1660, if s.b[1660] { 1.0 } else { 0.0 });
        if (s.b[1523] && s.b[1660]) {s.store_div(1652, 522, 274);}
        s.b[1661] = (s.v[1652] < 0.9);s.store_scalar(1661, if s.b[1661] { 1.0 } else { 0.0 });s.b[1662] = (p[1607] > 0.0);s.store_scalar(1662, if s.b[1662] { 1.0 } else { 0.0 });s.b[1663] = (s.v[522] > s.v[567]);s.store_scalar(1663, if s.b[1663] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && s.b[1663]) {s.store_sub_from_scalar(1657, 1.0, 1652);}
        s.b[1664] = (p[1601] != 1.0);s.store_scalar(1664, if s.b[1664] { 1.0 } else { 0.0 });s.b[1665] = (p[1601] == 0.5);s.store_scalar(1665, if s.b[1665] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && s.b[1663]) && s.b[1664]) && s.b[1665]) {s.store_div_from_scalar_sqrt_ad(1658, 1.0, s.ad_value(1657));}
        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && s.b[1663]) && s.b[1664]) && (!s.b[1665])) {s.store_powf(1658, 1657, (-p[1601]));}
        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && s.b[1663]) && s.b[1664]) {s.store_mul_ad_affine_product_rhs(536, 274, s.ad_value(528), A::sub_from_scalar(1.0, A::mul(s.ad_value(1657), s.ad_value(1658))), 1.0 / ((1.0 - p[1601])), 0.0);}
        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && s.b[1663]) && (!s.b[1664])) {
            s.store_mul_ad_affine_product_rhs(536, 274, s.ad_value(528), {
                if (!(s.v[1657] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1657] > 1e-38) {
                            A::ln(s.ad_value(1657))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        if ((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) {s.store_sub_from_scalar_div_indices(1657, 1.0, 567, 274);}
        s.b[1666] = (p[1601] != 1.0);s.store_scalar(1666, if s.b[1666] { 1.0 } else { 0.0 });s.b[1667] = (p[1601] == 0.5);s.store_scalar(1667, if s.b[1667] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1666]) && s.b[1667]) {s.store_div_from_scalar_sqrt_ad(1658, 1.0, s.ad_value(1657));}
        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1666]) && (!s.b[1667])) {s.store_powf(1658, 1657, (-p[1601]));}
        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1666]) {s.store_mul_ad_affine_product_rhs(1659, 274, s.ad_value(528), A::sub_from_scalar(1.0, A::mul(s.ad_value(1657), s.ad_value(1658))), 1.0 / ((1.0 - p[1601])), 0.0);}
        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && (!s.b[1666])) {
            s.store_mul_ad_affine_product_rhs(1659, 274, s.ad_value(528), {
                if (!(s.v[1657] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1657] > 1e-38) {
                            A::ln(s.ad_value(1657))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        if ((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) {s.store_sub_from_scalar_ad(1657, 1.0, A::div_scaled_inputs2(s.ad_value(522), 1.0, s.ad_value(567), (-1.0), s.ad_value(568), 1.0));}
        s.b[1668] = (p[1613] != 1.0);s.store_scalar(1668, if s.b[1668] { 1.0 } else { 0.0 });s.b[1669] = (p[1613] == 0.5);s.store_scalar(1669, if s.b[1669] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1668]) && s.b[1669]) {s.store_div_from_scalar_sqrt_ad(1658, 1.0, s.ad_value(1657));}
        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1668]) && (!s.b[1669])) {s.store_powf(1658, 1657, (-p[1613]));}
        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1668]) {s.store_add_product3_rhs_mixed_iia(536, 1659, 568, 528, A::sub_from_scalar(1.0, A::mul(s.ad_value(1657), s.ad_value(1658))), (p[1607] * 1.0 / ((1.0 - p[1613]))));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_119(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && (!s.b[1668])) {
            s.store_sub_mixed_ia(536, 1659, A::mul3_scaled_output(s.ad_value(568), s.ad_value(528), {
                            if (!(s.v[1657] > 1e-38)) {
                                A::neg(A::constant(87.498233534))
                            } else {
                                {
                                    if (s.v[1657] > 1e-38) {
                                        A::ln(s.ad_value(1657))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        }, p[1607]));
        }
        if (((s.b[1523] && s.b[1660]) && s.b[1661]) && (!s.b[1662])) {s.store_sub_from_scalar(1657, 1.0, 1652);}
        s.b[1670] = (p[1601] != 1.0);s.store_scalar(1670, if s.b[1670] { 1.0 } else { 0.0 });s.b[1671] = (p[1601] == 0.5);s.store_scalar(1671, if s.b[1671] { 1.0 } else { 0.0 });
        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && (!s.b[1662])) && s.b[1670]) && s.b[1671]) {s.store_div_from_scalar_sqrt_ad(1658, 1.0, s.ad_value(1657));}
        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && (!s.b[1662])) && s.b[1670]) && (!s.b[1671])) {s.store_powf(1658, 1657, (-p[1601]));}
        if ((((s.b[1523] && s.b[1660]) && s.b[1661]) && (!s.b[1662])) && s.b[1670]) {s.store_mul_ad_affine_product_rhs(536, 274, s.ad_value(528), A::sub_from_scalar(1.0, A::mul(s.ad_value(1657), s.ad_value(1658))), 1.0 / ((1.0 - p[1601])), 0.0);}
        if ((((s.b[1523] && s.b[1660]) && s.b[1661]) && (!s.b[1662])) && (!s.b[1670])) {
            s.store_mul_ad_affine_product_rhs(536, 274, s.ad_value(528), {
                if (!(s.v[1657] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1657] > 1e-38) {
                            A::ln(s.ad_value(1657))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        s.b[1672] = (p[1601] != 1.0);s.store_scalar(1672, if s.b[1672] { 1.0 } else { 0.0 });s.b[1673] = (p[1601] == 0.5);s.store_scalar(1673, if s.b[1673] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1660]) && (!s.b[1661])) && s.b[1672]) && s.b[1673]) {s.store_scalar(1653, (1.0 / ((0.1) as f64).sqrt()));}
        if ((((s.b[1523] && s.b[1660]) && (!s.b[1661])) && s.b[1672]) && (!s.b[1673])) {s.store_scalar(1653, ((0.1) as f64).powf((-p[1601])));}
        if (((s.b[1523] && s.b[1660]) && (!s.b[1661])) && s.b[1672]) {s.store_scalar(1654, (1.0 / (1.0 - p[1601])));s.store_primal_mul_scale_offset_mixed_ia(1656, 1654, A::scale(s.ad_value(1653), ((0.05 * p[1601]) * (1.0 + p[1601]))), -1.0, 1.0);}
        if (((s.b[1523] && s.b[1660]) && (!s.b[1661])) && (!s.b[1672])) {s.store_scalar(1653, 10.0);s.store_scalar(1656, (1.5 - ((0.1) as f64).ln()));}
        if ((s.b[1523] && s.b[1660]) && (!s.b[1661])) {s.store_mul_ad_product_rhs(1655, 1653, A::offset(s.ad_value(1652), (-1.0)), A::scale_offset(s.ad_value(1652), (5.0 * p[1601]), (((((-1.0)) * ((5.0 * p[1601])))) + ((1.0 + p[1601])))));s.store_mul_ad_product_rhs_mixed_ia(536, 274, 528, A::add(s.ad_value(1655), s.ad_value(1656)));}
        if (s.b[1523] && (!s.b[1660])) {s.store_scalar(536, 0.0);}
        if s.b[1523] {s.store_add_scaled_inputs3_indices(533, 534, 1.0, 535, 1.0, 536, 1.0);}
        s.store_add_scaled_inputs(507, 529, 1.0, 521, s.v[515]);s.store_add_scaled_inputs(508, 533, 1.0, 522, s.v[516]);s.store_mul_ad_product_rhs_mixed_ia(509, 517, 114, A::voltage(ctx, nodes, Some(3), Some(10)));s.b[1674] = (p[61] != 0.0);s.store_scalar(1674, if s.b[1674] { 1.0 } else { 0.0 });
        if s.b[1674] {s.store_mul_voltage_ad(170, s.ad_value(114), ctx, nodes, Some(10), Some(3));s.store_add_scaled_inputs4_offset_indices(171, 170, 1.0, 167, (-1.0), 146, 0.5, 166, 1.0, (-p[1529]));s.store_offset(168, 171, 0.02);s.store_scaled_add_sqrt_square_offset_rhs(512, 168, 168, (4.0 * 0.02), 0.5);s.store_sub_mixed_ia(509, 509, A::mul3_scaled_output(s.ad_value(156), s.ad_value(650), A::add_scaled_inputs_product(s.ad_value(171), 1.0, s.ad_value(512), (-1.0), s.ad_value(653), A::offset(A::sqrt(A::offset(A::div_scaled_inputs(s.ad_value(512), 4.0, s.ad_value(653), 1.0), 1.0)), (-1.0)), 0.5), s.v[115]));}
        s.store_mul_add_mixed_iia(169, 126, 865, A::mul3(s.ad_value(866), s.ad_value(126), s.ad_value(126)));s.store_div_scaled_product3_indices(168, 415, 372, 158, 1.0, 153, 1.0);s.store_div_scaled_inputs_indices(579, 428, 2.0, 415, 1.0);s.b[1678] = (((p[1682] > 0.0) || (p[1683] > 0.0)) || (p[1684] > 0.0));s.store_scalar(1678, if s.b[1678] { 1.0 } else { 0.0 });
        if s.b[1678] {s.store_offset(580, 153, (-(2.0 * p[1687])));}
        s.b[1679] = (s.v[580] <= 0.0);s.store_scalar(1679, if s.b[1679] { 1.0 } else { 0.0 });
        if (s.b[1678] && s.b[1679]) {s.copy_ad(580, 153);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_120(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1680] = ((p[79] == 1.0) || (p[79] == 0.0));s.store_scalar(1680, if s.b[1680] { 1.0 } else { 0.0 });
        if (s.b[1678] && s.b[1680]) {s.store_square(581, 580);}
        s.b[1681] = (p[1681] > 0.0);s.store_scalar(1681, if s.b[1681] { 1.0 } else { 0.0 });
        if ((s.b[1678] && s.b[1680]) && s.b[1681]) {s.store_div_scaled_offset_numerator_indices(168, 202, 1.0 / (s.v[578]), p[1681], 579, 1.0);}
        if ((s.b[1678] && s.b[1680]) && s.b[1681]) {
            s.store_scale_ad(582, {
                if (!(s.v[168] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[168] > 1e-38) {
                            A::ln(s.ad_value(168))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.v[578]);
        }
        if ((s.b[1678] && s.b[1680]) && (!s.b[1681])) {s.store_scalar(582, 0.0);}
        s.b[1682] = (p[79] == 1.0);s.store_scalar(1682, if s.b[1682] { 1.0 } else { 0.0 });
        if ((s.b[1678] && s.b[1680]) && s.b[1682]) {s.store_div(169, 400, 576);s.store_offset_pow_ad(170, s.ad_value(169), s.ad_value(575), 1.0);s.store_div(171, 574, 170);s.store_scale(172, 171, 1.0 / (p[1682]));s.store_scaled_add_offset_sqrt_square_offset(174, 172, 1.0, (-1.0), ((0.25 * p[1688]) * p[1688]), 0.5);s.store_scale(573, 174, p[1682]);}
        if ((s.b[1678] && s.b[1680]) && (!s.b[1682])) {s.store_scalar(573, p[1682]);}
        if (s.b[1678] && s.b[1680]) {s.store_mul_ad_affine_product_lhs(169, s.ad_value(179), A::abs(s.ad_value(124)), ((1.60219e-19 * 1.60219e-19) * 1.60219e-19), 0.0, 415);s.store_scaled_mul(170, 372, 581, 10000000000.0);s.store_scaled_mul(583, 372, 392, 6.241457005723417e18);s.store_scaled_mul(584, 372, 393, 6.241457005723417e18);s.store_mul_add_scaled_inputs_rhs_indices(585, 179, 372, 1.0 / (1.60219e-19), 669, 1.0 / (1.60219e-19));}
        if (s.b[1678] && s.b[1680]) {
            s.store_mul_mixed_ia(171, 573, {
                            if (!(((s.v[583] + s.v[585]) / (s.v[584] + s.v[585])) > 1e-38)) {
                                A::neg(A::constant(87.498233534))
                            } else {
                                {
                                    if (((s.v[583] + s.v[585]) / (s.v[584] + s.v[585])) > 1e-38) {
                                        A::ln(A::div_scaled_inputs2(s.ad_value(583), 1.0, s.ad_value(585), 1.0, A::add(s.ad_value(584), s.ad_value(585)), 1.0))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if (s.b[1678] && s.b[1680]) {s.store_scaled_sub(172, 583, 584, p[1683]);s.store_scaled_sub_ad(174, A::square(s.ad_value(583)), A::square(s.ad_value(584)), (0.5 * p[1684]));s.store_mul3_affine_lhs(175, 179, 124, 1.60219e-19, 0.0, 124);s.store_scaled_mul(176, 581, 158, (10000000000.0 * s.v[115]));s.store_add_scaled_inputs_product_indices(177, 573, 1.0, 584, p[1683], 584, 584, p[1684]);s.store_square_ad(178, A::add(s.ad_value(584), s.ad_value(585)));s.store_add_scaled_product(586, A::div_scaled_product3_by_product(s.ad_value(175), s.ad_value(582), s.ad_value(177), 1.0, s.ad_value(176), s.ad_value(178), 1.0), 1.0, A::div(s.ad_value(169), s.ad_value(170)), A::add_scaled_inputs3(s.ad_value(171), 1.0, s.ad_value(172), 1.0, s.ad_value(174), 1.0), 1.0);s.store_scaled_mul(340, 573, 179, 1.60219e-19);s.store_mul_product3_indices(341, 585, 158, 580, 585, (s.v[115] * 10000000000.0));s.store_mul_ad_product_lhs_mixed_ai(587, A::div(s.ad_value(340), s.ad_value(341)), 124, 124);s.store_add(169, 587, 586);}
        s.b[1684] = (p[79] == 2.0);s.store_scalar(1684, if s.b[1684] { 1.0 } else { 0.0 });
        if ((s.b[1678] && (!s.b[1680])) && s.b[1684]) {s.store_div(169, 400, 576);s.store_offset_pow_ad(170, s.ad_value(169), s.ad_value(575), 1.0);s.store_div(171, 574, 170);s.store_scale(172, 171, 1.0 / (p[1682]));}
    }
}
