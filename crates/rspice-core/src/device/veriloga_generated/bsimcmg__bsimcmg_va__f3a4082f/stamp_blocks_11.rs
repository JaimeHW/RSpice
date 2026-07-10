#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_45(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[1298]) && s.b[1313]) {
            s.store_offset_ad(337, {
                if (!(((s.v[790] * ((1.0 + (p.p450 * s.v[232])) + ((p.p451 * s.v[232]) * s.v[232]))) - 2.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0)), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((s.v[790] * ((1.0 + (p.p450 * s.v[232])) + ((p.p451 * s.v[232]) * s.v[232]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 2.0);
        }
        s.b[1332] = (p.p66 != 0.0);s.store_scalar(1332, if s.b[1332] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && s.b[1313]) && s.b[1332]) {
            s.store_offset_ad(338, {
                if (!(((s.v[791] * ((1.0 + (p.p452 * s.v[232])) + ((p.p451 * s.v[232]) * s.v[232]))) - 2.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0)), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((s.v[791] * ((1.0 + (p.p452 * s.v[232])) + ((p.p451 * s.v[232]) * s.v[232]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 2.0);
        }
        s.b[1333] = (p.p75 != 0.0);s.store_scalar(1333, if s.b[1333] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && s.b[1313]) && s.b[1333]) {s.store_add_scaled_inputs3_mixed_iai(660, 657, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_product(s.ad_value(232), p.p498, s.ad_value(232), s.ad_value(232), p.p499), 0.5, s.ad_value(657), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_product(s.ad_value(232), p.p498, s.ad_value(232), s.ad_value(232), p.p499), 1.0, s.ad_value(657), -1.0), (-1e-6))), 1.0, s.ad_value(657), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 657, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_46(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1333])) {
            s.store_mul_mixed_ia(660, 657, {
                            if (!((((1.0 + (p.p498 * s.v[232])) + ((p.p499 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p498, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p499), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p498, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p499), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if ((((1.0 + (p.p498 * s.v[232])) + ((p.p499 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p498, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p499), (-1e-6), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1334] = (p.p75 != 0.0);s.store_scalar(1334, if s.b[1334] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && s.b[1313]) && s.b[1334]) {s.store_add_scaled_inputs3_mixed_iai(797, 792, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p1026 * 0.5), s.ad_value(792), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1026, s.ad_value(792), -1.0), (-1e-6))), 1.0, s.ad_value(792), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 792, (-1.0));}
        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1334])) {
            s.store_mul_mixed_ia(797, 792, {
                            if (!(((1.0 + (p.p1026 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1026, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1026, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (p.p1026 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1026, ((1.0) + ((-1e-6)))))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if ((!s.b[1298]) && s.b[1313]) {s.store_sub_ad(231, A::add_scaled_product(A::div_scalar_offset_denominator(p.p1747, A::limited_exp_scaled_input(A::offset(s.ad_value(116), (-p.p1749)), p.p1748), 1.0, 1.0), 1.0, A::add(s.ad_value(858), A::div_from_scalar(p.p1720, s.ad_value(153))), s.ad_value(230), 1.0), A::div_scalar_offset_denominator(p.p1747, A::limited_exp_scaled_input(A::offset(s.ad_value(228), (-p.p1749)), p.p1748), 1.0, 1.0));}
        if ((!s.b[1298]) && (!s.b[1313])) {s.store_mul_exp_mixed_ia(169, 704, A::mul(A::add_scaled_product(s.ad_value(836), 1.0, s.ad_value(837), s.ad_value(234), 1.0), s.ad_value(418)));s.store_add_scaled_inputs4_offset_mixed_iiaa(413, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));}
        s.b[1335] = (p.p66 == 1.0);s.store_scalar(1335, if s.b[1335] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1335]) {s.store_mul_exp_mixed_ia(169, 706, A::mul(A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(837), s.ad_value(234), 1.0), s.ad_value(418)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_47(
        s: &mut ReactiveScratch,
    ) {
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1335]) {s.store_add_scaled_inputs4_offset_mixed_iiaa(321, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));s.copy_ad(417, 321);}
        s.b[1336] = (s.v[228] > 210.0);s.store_scalar(1336, if s.b[1336] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1336]) {
            s.store_scaled_sub_ad(170, A::div(s.ad_value(823), A::add(s.ad_value(807), A::mul_sub_from_scalar_rhs(s.ad_value(823), 210.0, s.ad_value(228)))), A::div_scaled_product_offset_rhs(s.ad_value(824), {
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1.0, s.ad_value(228), 1.0), 210.0);
        }
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1336]) {s.store_div_scaled_inputs2_mixed_iaa(169, 807, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(823), 210.0, s.ad_value(228)), 1.0, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), 1.0);s.store_mul_pow_mixed_iia(306, 169, 229, A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(824), s.ad_value(229), 1.0));s.store_add_scaled_product_indices(307, 807, 1.0, 823, 232, 1.0);}
        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1336])) {
            s.store_mul_ad_product_rhs(170, 807, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(823), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), A::add_scaled_inputs(s.ad_value(823), 0.004761904761904762, A::div_scaled_product_offset_rhs(s.ad_value(824), {
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1.0, s.ad_value(228), 1.0), 1.0));
        }
        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1336])) {s.store_add_scaled_product_mixed_aia(169, A::mul_sub_from_scalar_rhs(s.ad_value(170), 210.0, s.ad_value(228)), (-1.0), 807, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(823), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), 1.0);s.store_mul_pow_mixed_iia(306, 807, 229, A::add_scaled_product(s.ad_value(823), 1.0, s.ad_value(824), s.ad_value(229), 1.0));s.store_add_scaled_product_indices(307, 169, 1.0, 170, 232, 1.0);}
        if ((!s.b[1298]) && (!s.b[1313])) {s.store_add_scaled_products_indices(168, 313, 306, 1.0, 312, 307, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_48(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[1298]) && (!s.b[1313])) {
            if (!(s.v[168] < ((-10000.0) * 1e-6))) {
                s.store_scaled_add_mixed_ia(303, 168, A::sqrt_square_offset(s.ad_value(168), ((4.0 * 1e-6) * 1e-6)), 0.5);
            } else {
                if (s.v[168] < ((-10000.0) * 1e-6)) {
                    s.store_div_from_scalar(303, ((-1e-6) * 1e-6), 168);
                } else {
                    s.store_scalar(303, 0.0);
                }
            }
        }
        s.b[1337] = (p.p66 != 0.0);s.store_scalar(1337, if s.b[1337] { 1.0 } else { 0.0 });s.b[1338] = (s.v[228] > 210.0);s.store_scalar(1338, if s.b[1338] { 1.0 } else { 0.0 });
        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) && s.b[1338]) {
            s.store_scaled_sub_ad(170, A::div(s.ad_value(825), A::add(s.ad_value(815), A::mul_sub_from_scalar_rhs(s.ad_value(825), 210.0, s.ad_value(228)))), A::div_scaled_product_offset_rhs(s.ad_value(824), {
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1.0, s.ad_value(228), 1.0), 210.0);
        }
        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) && s.b[1338]) {s.store_div_scaled_inputs2_mixed_iaa(169, 815, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(825), 210.0, s.ad_value(228)), 1.0, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), 1.0);s.store_mul_pow_mixed_iia(310, 169, 229, A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(824), s.ad_value(229), 1.0));s.store_add_scaled_product_indices(311, 815, 1.0, 825, 232, 1.0);}
        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) && (!s.b[1338])) {
            s.store_mul_ad_product_rhs(170, 815, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(825), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), A::add_scaled_inputs(s.ad_value(825), 0.004761904761904762, A::div_scaled_product_offset_rhs(s.ad_value(824), {
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1.0, s.ad_value(228), 1.0), 1.0));
        }
        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) && (!s.b[1338])) {s.store_add_scaled_product_mixed_aia(169, A::mul_sub_from_scalar_rhs(s.ad_value(170), 210.0, s.ad_value(228)), (-1.0), 815, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(825), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), 1.0);s.store_mul_pow_mixed_iia(310, 815, 229, A::add_scaled_product(s.ad_value(825), 1.0, s.ad_value(824), s.ad_value(229), 1.0));s.store_add_scaled_product_indices(311, 169, 1.0, 170, 232, 1.0);}
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) {s.store_add_scaled_products_indices(168, 313, 310, 1.0, 312, 311, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_49(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) {
            if (!(s.v[168] < ((-10000.0) * 1e-6))) {
                s.store_scaled_add_mixed_ia(305, 168, A::sqrt_square_offset(s.ad_value(168), ((4.0 * 1e-6) * 1e-6)), 0.5);
            } else {
                if (s.v[168] < ((-10000.0) * 1e-6)) {
                    s.store_div_from_scalar(305, ((-1e-6) * 1e-6), 168);
                } else {
                    s.store_scalar(305, 0.0);
                }
            }
        }
        if ((!s.b[1298]) && (!s.b[1313])) {s.store_mul_exp_mixed_ia(318, 812, A::mul(A::add_scaled_product(s.ad_value(830), 1.0, s.ad_value(831), s.ad_value(234), 1.0), s.ad_value(418)));}
        s.b[1339] = (p.p66 != 0.0);s.store_scalar(1339, if s.b[1339] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1339]) {s.store_mul_exp_mixed_ia(320, 818, A::mul(A::add_scaled_product(s.ad_value(844), 1.0, s.ad_value(831), s.ad_value(234), 1.0), s.ad_value(418)));}
        if ((!s.b[1298]) && (!s.b[1313])) {s.store_mul_exp_mixed_ia(317, 814, A::mul(A::add_scaled_product(s.ad_value(834), 1.0, s.ad_value(835), s.ad_value(234), 1.0), s.ad_value(418)));}
        s.b[1340] = (((((s.v[326] * (s.v[228] - 210.0)) / s.v[228])) as f64).abs() < 1e-6);s.store_scalar(1340, if s.b[1340] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1340]) {s.store_mul_scale_offset_mixed_ia(324, 325, A::limited_exp(A::mul(s.ad_value(326), s.ad_value(234))), 1.0, (-1.0));}
        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1340])) {s.store_div_scaled_product_offset_rhs_mixed_iaa(324, 325, A::limited_exp(A::mul(s.ad_value(326), s.ad_value(234))), (-1.0), 1.0, A::abs(A::offset(A::limited_exp(A::div_scaled_product_offset_rhs(s.ad_value(326), s.ad_value(228), (-210.0), 1.0, s.ad_value(228), 1.0)), (-1.0))), 1.0);}
        s.b[1341] = (((((s.v[329] * (s.v[228] - 210.0)) / s.v[228])) as f64).abs() < 1e-6);s.store_scalar(1341, if s.b[1341] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1341]) {s.store_mul_scale_offset_mixed_ia(327, 328, A::limited_exp(A::mul(s.ad_value(329), s.ad_value(234))), 1.0, (-1.0));}
        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1341])) {s.store_div_scaled_product_offset_rhs_mixed_iaa(327, 328, A::limited_exp(A::mul(s.ad_value(329), s.ad_value(234))), (-1.0), 1.0, A::abs(A::offset(A::limited_exp(A::div_scaled_product_offset_rhs(s.ad_value(329), s.ad_value(228), (-210.0), 1.0, s.ad_value(228), 1.0)), (-1.0))), 1.0);}
        if ((!s.b[1298]) && (!s.b[1313])) {s.store_offset(330, 324, 0.5);s.store_offset(331, 327, 0.5);}
        s.b[1342] = (p.p75 != 0.0);s.store_scalar(1342, if s.b[1342] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) {s.store_add_scaled_inputs4_offset_mixed_iaai(323, 811, 1.0, A::add_scaled_product(s.ad_value(811), 1.0, s.ad_value(847), s.ad_value(233), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(811), 1.0, s.ad_value(847), s.ad_value(233), 1.0), (-1e-6))), 1.0, s.ad_value(811), (-(4.0 * 1e-6)))), 0.5, 811, (-1.0), (0.5 * (-1e-6)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_50(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) {s.store_add_scaled_inputs3_mixed_iai(332, 679, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 0.5, s.ad_value(679), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 1.0, s.ad_value(679), -1.0), (-1e-6))), 1.0, s.ad_value(679), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 679, (-1.0));}
        s.b[1343] = (p.p66 != 0.0);s.store_scalar(1343, if s.b[1343] { 1.0 } else { 0.0 });
        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) && s.b[1343]) {s.store_add_scaled_inputs3_mixed_iai(333, 680, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(851), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 0.5, s.ad_value(680), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(851), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 1.0, s.ad_value(680), -1.0), (-1e-6))), 1.0, s.ad_value(680), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 680, (-1.0));}
        s.b[1344] = (s.v[333] < 1000.0);s.store_scalar(1344, if s.b[1344] { 1.0 } else { 0.0 });
        if (((((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) && s.b[1343]) && s.b[1344]) {s.store_scalar(333, 1000.0);}
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) {s.store_add_scaled_inputs3_mixed_iai(334, 698, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 0.5, s.ad_value(698), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 1.0, s.ad_value(698), -1.0), (-1e-6))), 1.0, s.ad_value(698), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 698, (-1.0));}
        s.b[1345] = (p.p66 != 0.0);s.store_scalar(1345, if s.b[1345] { 1.0 } else { 0.0 });
        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) && s.b[1345]) {s.store_add_scaled_inputs3_mixed_iai(335, 699, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 0.5, s.ad_value(699), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 1.0, s.ad_value(699), -1.0), (-1e-6))), 1.0, s.ad_value(699), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 699, (-1.0));}
        s.b[1346] = (s.v[335] < 1000.0);s.store_scalar(1346, if s.b[1346] { 1.0 } else { 0.0 });
        if (((((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) && s.b[1345]) && s.b[1346]) {s.store_scalar(335, 1000.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_51(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) {s.store_add_scaled_inputs3_mixed_iai(336, 702, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(850), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p574), 0.5, s.ad_value(702), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(850), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p574), 1.0, s.ad_value(702), -1.0), (-1e-6))), 1.0, s.ad_value(702), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 702, (-1.0));s.store_add_scaled_inputs3_mixed_iai(660, 657, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_product(s.ad_value(233), p.p498, s.ad_value(233), s.ad_value(233), p.p499), 0.5, s.ad_value(657), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_product(s.ad_value(233), p.p498, s.ad_value(233), s.ad_value(233), p.p499), 1.0, s.ad_value(657), -1.0), (-1e-6))), 1.0, s.ad_value(657), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 657, (-1.0));s.store_add_scaled_inputs3_mixed_iai(797, 792, 1.0, A::add_scaled_inputs3_offset(s.ad_value(233), (p.p1026 * 0.5), s.ad_value(792), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(233), p.p1026, s.ad_value(792), -1.0), (-1e-6))), 1.0, s.ad_value(792), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 792, (-1.0));}
        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            s.store_mul_mixed_ia(323, 811, {
                            if (!(((1.0 + (s.v[847] * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul(s.ad_value(847), s.ad_value(233)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(847), s.ad_value(233)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (s.v[847] * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(847), s.ad_value(233)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            s.store_mul_mixed_ia(332, 679, {
                            if (!((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if ((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_52(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1347] = (p.p66 != 0.0);s.store_scalar(1347, if s.b[1347] { 1.0 } else { 0.0 });
        if ((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) && s.b[1347]) {
            s.store_mul_mixed_ia(333, 680, {
                            if (!((((1.0 - (s.v[851] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(851), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(851), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if ((((1.0 - (s.v[851] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(851), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1348] = (s.v[333] < 1000.0);s.store_scalar(1348, if s.b[1348] { 1.0 } else { 0.0 });
        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) && s.b[1347]) && s.b[1348]) {s.store_scalar(333, 1000.0);}
        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            s.store_mul_mixed_ia(334, 698, {
                            if (!((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if ((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1349] = (p.p66 != 0.0);s.store_scalar(1349, if s.b[1349] { 1.0 } else { 0.0 });
        if ((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) && s.b[1349]) {
            s.store_mul_mixed_ia(335, 699, {
                            if (!((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if ((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1350] = (s.v[335] < 1000.0);s.store_scalar(1350, if s.b[1350] { 1.0 } else { 0.0 });
        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) && s.b[1349]) && s.b[1350]) {s.store_scalar(335, 1000.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_53(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            s.store_mul_mixed_ia(336, 702, {
                            if (!((((1.0 - (s.v[850] * s.v[232])) + ((p.p574 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(850), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p574), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(850), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p574), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if ((((1.0 - (s.v[850] * s.v[232])) + ((p.p574 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(850), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p574), (-1e-6), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            s.store_mul_mixed_ia(660, 657, {
                            if (!((((1.0 + (p.p498 * s.v[233])) + ((p.p499 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::add_scaled_product(A::scale_offset(s.ad_value(233), p.p498, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p499), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(A::scale_offset(s.ad_value(233), p.p498, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p499), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if ((((1.0 + (p.p498 * s.v[233])) + ((p.p499 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_product(A::scale_offset(s.ad_value(233), p.p498, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p499), (-1e-6), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            s.store_mul_mixed_ia(797, 792, {
                            if (!(((1.0 + (p.p1026 * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::scale_offset(s.ad_value(233), p.p1026, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(233), p.p1026, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (p.p1026 * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(233), p.p1026, ((1.0) + ((-1e-6)))))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if ((!s.b[1298]) && (!s.b[1313])) {
            s.store_offset_ad(337, {
                if (!(((s.v[790] * ((1.0 + (p.p450 * s.v[232])) + ((p.p451 * s.v[233]) * s.v[233]))) - 2.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0)), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((s.v[790] * ((1.0 + (p.p450 * s.v[232])) + ((p.p451 * s.v[233]) * s.v[233]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 2.0);
        }
        s.b[1351] = (p.p66 != 0.0);s.store_scalar(1351, if s.b[1351] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_54(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1351]) {
            s.store_offset_ad(338, {
                if (!(((s.v[791] * ((1.0 + (p.p452 * s.v[232])) + ((p.p451 * s.v[233]) * s.v[233]))) - 2.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0)), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((s.v[791] * ((1.0 + (p.p452 * s.v[232])) + ((p.p451 * s.v[233]) * s.v[233]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 2.0);
        }
        s.b[1352] = (p.p67 == 1.0);s.store_scalar(1352, if s.b[1352] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) {s.store_mul_exp_mixed_ia(169, 705, A::mul(A::add_scaled_product(s.ad_value(839), 1.0, s.ad_value(840), s.ad_value(234), 1.0), s.ad_value(418)));s.store_add_scaled_inputs4_offset_mixed_iiaa(414, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));}
        s.b[1353] = (s.v[228] > 210.0);s.store_scalar(1353, if s.b[1353] { 1.0 } else { 0.0 });
        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) && s.b[1353]) {
            s.store_scaled_sub_ad(170, A::div(s.ad_value(826), A::add(s.ad_value(808), A::mul_sub_from_scalar_rhs(s.ad_value(826), 210.0, s.ad_value(228)))), A::div_scaled_product_offset_rhs(s.ad_value(827), {
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1.0, s.ad_value(228), 1.0), 210.0);
        }
        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) && s.b[1353]) {s.store_div_scaled_inputs2_mixed_iaa(169, 808, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(826), 210.0, s.ad_value(228)), 1.0, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(827), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), 1.0);s.store_mul_pow_mixed_iia(308, 169, 229, A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(827), s.ad_value(229), 1.0));s.store_add_scaled_product_indices(309, 808, 1.0, 826, 232, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_55(
        s: &mut ReactiveScratch,
    ) {
        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) && (!s.b[1353])) {
            s.store_mul_ad_product_rhs(170, 808, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(826), 1.0, s.ad_value(827), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), A::add_scaled_inputs(s.ad_value(826), 0.004761904761904762, A::div_scaled_product_offset_rhs(s.ad_value(827), {
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1.0, s.ad_value(228), 1.0), 1.0));
        }
        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) && (!s.b[1353])) {s.store_add_scaled_product_mixed_aia(169, A::mul_sub_from_scalar_rhs(s.ad_value(170), 210.0, s.ad_value(228)), (-1.0), 808, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(826), 1.0, s.ad_value(827), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), 1.0);s.store_mul_pow_mixed_iia(308, 808, 229, A::add_scaled_product(s.ad_value(826), 1.0, s.ad_value(827), s.ad_value(229), 1.0));s.store_add_scaled_product_indices(309, 169, 1.0, 170, 232, 1.0);}
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) {s.store_add_scaled_products_indices(168, 313, 308, 1.0, 312, 309, 1.0);}
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) {
            if (!(s.v[168] < ((-10000.0) * 1e-6))) {
                s.store_scaled_add_mixed_ia(304, 168, A::sqrt_square_offset(s.ad_value(168), ((4.0 * 1e-6) * 1e-6)), 0.5);
            } else {
                if (s.v[168] < ((-10000.0) * 1e-6)) {
                    s.store_div_from_scalar(304, ((-1e-6) * 1e-6), 168);
                } else {
                    s.store_scalar(304, 0.0);
                }
            }
        }
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) {s.store_mul_exp_mixed_ia(319, 813, A::mul(A::add_scaled_product(s.ad_value(832), 1.0, s.ad_value(833), s.ad_value(234), 1.0), s.ad_value(418)));}
        s.b[1354] = (s.v[854] == s.v[855]);s.store_scalar(1354, if s.b[1354] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1354]) {s.store_offset_mul(170, 854, 232, 1.0);}
        s.b[1355] = (s.v[856] < 210.0);s.store_scalar(1355, if s.b[1355] { 1.0 } else { 0.0 });s.b[1356] = (s.v[228] > 210.0);s.store_scalar(1356, if s.b[1356] { 1.0 } else { 0.0 });
        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && s.b[1356]) {s.store_offset_mul(195, 854, 232, 1.0);s.store_add_scaled_product_mixed_aia(196, A::offset(A::mul(s.ad_value(855), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), 1.0, 854, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);s.store_offset_ad(171, A::mul_sub_from_scalar_rhs(s.ad_value(854), 210.0, s.ad_value(228)), 1.0);s.store_add_scaled_product_mixed_aia(172, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(855), 210.0, s.ad_value(856)), 1.0), 1.0, 854, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);}
        s.b[1357] = (s.v[855] < s.v[854]);s.store_scalar(1357, if s.b[1357] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_56(
        s: &mut ReactiveScratch,
    ) {
        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && s.b[1356]) && s.b[1357]) {s.store_add_mixed_ai(174, A::sub(A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs3(s.ad_value(171), 0.5, s.ad_value(172), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(171), s.ad_value(172)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5)), 171);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 174, 0.5, 195, 0.5, 174, 195, ((0.25 * 0.001) * 0.001), 0.5);}
        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && s.b[1356]) && (!s.b[1357])) {s.store_add_mixed_ai(174, A::sub(A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::add_scaled_inputs3(s.ad_value(171), 0.5, s.ad_value(172), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(171), s.ad_value(172)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5))), 171);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 174, 0.5, 195, 0.5, 174, 195, ((0.25 * 0.001) * 0.001), (-0.5));}
        s.b[1358] = (s.v[228] > s.v[856]);s.store_scalar(1358, if s.b[1358] { 1.0 } else { 0.0 });
        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && s.b[1358]) {s.store_offset_mul(195, 854, 232, 1.0);s.store_add_scaled_product_mixed_aia(196, A::offset(A::mul(s.ad_value(855), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), 1.0, 854, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);s.store_mul_sub_by_sub(171, 854, 855, 856, 228);s.store_offset_ad(172, A::mul_sub_from_scalar_rhs(s.ad_value(854), 210.0, s.ad_value(228)), 1.0);s.store_add_scaled_product_mixed_aia(174, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(855), 210.0, s.ad_value(856)), 1.0), 1.0, 854, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);}
        s.b[1359] = (s.v[855] < s.v[854]);s.store_scalar(1359, if s.b[1359] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_57(
        s: &mut ReactiveScratch,
    ) {
        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && s.b[1358]) && s.b[1359]) {s.store_sub_ad(175, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));s.store_sub_ad(176, A::add_scaled_inputs3(s.ad_value(172), 0.5, s.ad_value(174), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(172), s.ad_value(174)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));s.store_add_scaled_offset_product_rhs(177, 176, 1.0, 854, 116, (-210.0), 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 175, 0.5, 177, 0.5, 175, 177, ((0.25 * 0.001) * 0.001), 0.5);}
        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && s.b[1358]) && (!s.b[1359])) {s.store_sub_ad(175, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::sub_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));s.store_sub_ad(176, A::add_scaled_inputs3(s.ad_value(172), 0.5, s.ad_value(174), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(172), s.ad_value(174)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::sub_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));s.store_add_scaled_offset_product_rhs(177, 176, 1.0, 854, 116, (-210.0), 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 175, 0.5, 177, 0.5, 175, 177, ((0.25 * 0.001) * 0.001), (-0.5));}
        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && (!s.b[1358])) {s.store_offset_mul(196, 855, 232, 1.0);s.store_add_scaled_product_mixed_aia(195, A::offset(A::mul(s.ad_value(854), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), 1.0, 855, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);s.store_mul_sub_by_sub(171, 855, 854, 856, 228);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_58(
        s: &mut ReactiveScratch,
    ) {
        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && (!s.b[1358])) {s.store_offset_ad(172, A::mul_sub_from_scalar_rhs(s.ad_value(855), 210.0, s.ad_value(228)), 1.0);s.store_add_scaled_product_mixed_aia(174, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(854), 210.0, s.ad_value(856)), 1.0), 1.0, 855, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);}
        s.b[1360] = (s.v[855] < s.v[854]);s.store_scalar(1360, if s.b[1360] { 1.0 } else { 0.0 });
        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && (!s.b[1358])) && s.b[1360]) {s.store_sub_ad(175, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));s.store_sub_ad(176, A::add_scaled_inputs3(s.ad_value(172), 0.5, s.ad_value(174), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(172), s.ad_value(174)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));s.store_add_scaled_offset_product_rhs(177, 176, 1.0, 854, 116, (-210.0), 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 175, 0.5, 177, 0.5, 175, 177, ((0.25 * 0.001) * 0.001), 0.5);}
        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && (!s.b[1358])) && (!s.b[1360])) {s.store_sub_ad(175, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::sub_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));s.store_sub_ad(176, A::add_scaled_inputs3(s.ad_value(172), 0.5, s.ad_value(174), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(172), s.ad_value(174)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::sub_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));s.store_add_scaled_offset_product_rhs(177, 176, 1.0, 854, 116, (-210.0), 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 175, 0.5, 177, 0.5, 175, 177, ((0.25 * 0.001) * 0.001), (-0.5));}
        s.b[1361] = (s.v[228] > 210.0);s.store_scalar(1361, if s.b[1361] { 1.0 } else { 0.0 });
        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && s.b[1361]) {s.store_offset_mul(195, 854, 232, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_59(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && s.b[1361]) {s.store_add_ad(196, A::offset(A::mul_offset_rhs(s.ad_value(855), s.ad_value(116), (-210.0)), 1.0), A::mul_sub_from_scalar_rhs(s.ad_value(854), 210.0, s.ad_value(228)));}
        s.b[1362] = (s.v[855] < s.v[854]);s.store_scalar(1362, if s.b[1362] { 1.0 } else { 0.0 });
        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && s.b[1361]) && s.b[1362]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 195, 0.5, 196, 0.5, 195, 196, ((0.25 * 0.01) * 0.01), 0.5);}
        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && s.b[1361]) && (!s.b[1362])) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 195, 0.5, 196, 0.5, 195, 196, ((0.25 * 0.01) * 0.01), (-0.5));}
        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && (!s.b[1361])) {s.store_offset_mul(196, 855, 232, 1.0);s.store_add_ad(195, A::offset(A::mul_offset_rhs(s.ad_value(854), s.ad_value(116), (-210.0)), 1.0), A::mul_sub_from_scalar_rhs(s.ad_value(855), 210.0, s.ad_value(228)));}
        s.b[1363] = (s.v[855] < s.v[854]);s.store_scalar(1363, if s.b[1363] { 1.0 } else { 0.0 });
        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && (!s.b[1361])) && s.b[1363]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 195, 0.5, 196, 0.5, 195, 196, ((0.25 * 0.01) * 0.01), 0.5);}
        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && (!s.b[1361])) && (!s.b[1363])) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 195, 0.5, 196, 0.5, 195, 196, ((0.25 * 0.01) * 0.01), (-0.5));}
        if ((!s.b[1298]) && (!s.b[1313])) {
            if (!((s.v[170] - 1e-6) < ((-10000.0) * 0.001))) {
                s.store_scaled_add_sqrt_square_offset_ad(194, A::offset(s.ad_value(170), (-1e-6)), ((4.0 * 0.001) * 0.001), 0.5);
            } else {
                if ((s.v[170] - 1e-6) < ((-10000.0) * 0.001)) {
                    s.store_div_from_scalar_offset_input(194, ((-0.001) * 0.001), 170, (-1e-6));
                } else {
                    s.store_scalar(194, 0.0);
                }
            }
        }
        if ((!s.b[1298]) && (!s.b[1313])) {s.store_scaled_sub_offset_sqrt_square_offset(172, 228, 210.0, (-210.0), ((0.25 * 0.2) * 0.2), 0.5);s.store_sub_ad(231, A::add_scaled_product(A::div_scalar_offset_denominator(p.p1747, A::limited_exp_scaled_input(A::offset(s.ad_value(117), (-p.p1749)), p.p1748), 1.0, 1.0), 1.0, A::add(s.ad_value(858), A::div_from_scalar(p.p1720, s.ad_value(153))), s.ad_value(230), 1.0), A::div_scalar_offset_denominator(p.p1747, A::limited_exp_scaled_input(A::offset(s.ad_value(172), (-p.p1749)), p.p1748), 1.0, 1.0));}
        s.b[1364] = (s.v[332] < 1000.0);s.store_scalar(1364, if s.b[1364] { 1.0 } else { 0.0 });
        if s.b[1364] {s.store_scalar(332, 1000.0);}
        s.b[1365] = (s.v[334] < 1000.0);s.store_scalar(1365, if s.b[1365] { 1.0 } else { 0.0 });
        if s.b[1365] {s.store_scalar(334, 1000.0);}
        s.b[1366] = (s.v[336] < 1000.0);s.store_scalar(1366, if s.b[1366] { 1.0 } else { 0.0 });
        if s.b[1366] {s.store_scalar(336, 1000.0);}
        s.b[1367] = (p.p61 != 0.0);s.store_scalar(1367, if s.b[1367] { 1.0 } else { 0.0 });s.b[1368] = (p.p75 == 0.0);s.store_scalar(1368, if s.b[1368] { 1.0 } else { 0.0 });s.b[1369] = (p.p75 != 0.0);s.store_scalar(1369, if s.b[1369] { 1.0 } else { 0.0 });
        if ((s.b[1367] && s.b[1368]) && s.b[1369]) {s.store_add_scaled_inputs4_offset_mixed_iaai(314, 809, 1.0, A::add_scaled_product(s.ad_value(809), 1.0, s.ad_value(828), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(809), 1.0, s.ad_value(828), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(809), (-(4.0 * 1e-6)))), 0.5, 809, (-1.0), (0.5 * (-1e-6)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_60(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1367] && s.b[1368]) && (!s.b[1369])) {
            s.store_mul_mixed_ia(314, 809, {
                            if (!(((1.0 + (s.v[828] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul(s.ad_value(828), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(828), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (s.v[828] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(828), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1370] = (p.p67 == 1.0);s.store_scalar(1370, if s.b[1370] { 1.0 } else { 0.0 });s.b[1371] = (p.p75 != 0.0);s.store_scalar(1371, if s.b[1371] { 1.0 } else { 0.0 });
        if (((s.b[1367] && s.b[1368]) && s.b[1370]) && s.b[1371]) {s.store_add_scaled_inputs4_offset_mixed_iaai(315, 810, 1.0, A::add_scaled_product(s.ad_value(810), 1.0, s.ad_value(829), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(810), 1.0, s.ad_value(829), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(810), (-(4.0 * 1e-6)))), 0.5, 810, (-1.0), (0.5 * (-1e-6)));}
        if (((s.b[1367] && s.b[1368]) && s.b[1370]) && (!s.b[1371])) {
            s.store_mul_mixed_ia(315, 810, {
                            if (!(((1.0 + (s.v[829] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul(s.ad_value(829), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(829), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (s.v[829] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(829), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1372] = (p.p66 != 0.0);s.store_scalar(1372, if s.b[1372] { 1.0 } else { 0.0 });s.b[1373] = (p.p75 != 0.0);s.store_scalar(1373, if s.b[1373] { 1.0 } else { 0.0 });
        if (((s.b[1367] && s.b[1368]) && s.b[1372]) && s.b[1373]) {s.store_add_scaled_inputs4_offset_mixed_iaai(316, 817, 1.0, A::add_scaled_product(s.ad_value(817), 1.0, s.ad_value(843), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(817), 1.0, s.ad_value(843), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(817), (-(4.0 * 1e-6)))), 0.5, 817, (-1.0), (0.5 * (-1e-6)));}
        if (((s.b[1367] && s.b[1368]) && s.b[1372]) && (!s.b[1373])) {
            s.store_mul_mixed_ia(316, 817, {
                            if (!(((1.0 + (s.v[843] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul(s.ad_value(843), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(843), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (s.v[843] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(843), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if (s.b[1367] && (!s.b[1368])) {s.store_add_scaled_product_indices(314, 809, 1.0, 828, 232, 1.0);}
        s.b[1374] = (p.p67 == 1.0);s.store_scalar(1374, if s.b[1374] { 1.0 } else { 0.0 });
        if ((s.b[1367] && (!s.b[1368])) && s.b[1374]) {s.store_add_scaled_product_indices(315, 810, 1.0, 829, 232, 1.0);}
        s.b[1375] = (p.p66 != 0.0);s.store_scalar(1375, if s.b[1375] { 1.0 } else { 0.0 });
    }
}
