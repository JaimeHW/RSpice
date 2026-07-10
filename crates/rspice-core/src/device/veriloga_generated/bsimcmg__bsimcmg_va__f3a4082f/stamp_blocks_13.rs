#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_77(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_add_mixed_ai(899, {
                    if (!(A::div_scaled_product_offset_denominator(A::mul(s.ad_value(897), s.ad_value(898)), A::mul(s.ad_value(897), s.ad_value(898)), 1.0, A::add_scaled_product(A::limited_exp(A::mul(s.ad_value(897), s.ad_value(898))), 1.0, s.ad_value(897), s.ad_value(898), (-1.0)), (-1.0), 1.0).value > 1e-38)) {
                        A::neg(A::constant(87.498233534))
                    } else {
                        {
                            if (A::div_scaled_product_offset_denominator(A::mul(s.ad_value(897), s.ad_value(898)), A::mul(s.ad_value(897), s.ad_value(898)), 1.0, A::add_scaled_product(A::limited_exp(A::mul(s.ad_value(897), s.ad_value(898))), 1.0, s.ad_value(897), s.ad_value(898), (-1.0)), (-1.0), 1.0).value > 1e-38) {
                                A::ln(A::div_scaled_product_offset_denominator(A::mul(s.ad_value(897), s.ad_value(898)), A::mul(s.ad_value(897), s.ad_value(898)), 1.0, A::add_scaled_product(A::limited_exp(A::mul(s.ad_value(897), s.ad_value(898))), 1.0, s.ad_value(897), s.ad_value(898), (-1.0)), (-1.0), 1.0))
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                }, 900);
        s.store_add_scaled_inputs_mixed_ai(339, A::div_scaled_inputs(s.ad_value(181), 10.0, s.ad_value(898), 1.0), 1.0, 396, 2.0);s.store_div_scaled_product_indices(912, 179, 893, 1.0, 895, s.v[143]);s.store_scalar(913, ((((((4.5 * 1.05457e-34) * 3.141592653589793) * 1.60219e-19) / (4.0 * (((2.0 * s.v[381])) as f64).sqrt()))) as f64).powf(0.666666667));s.store_div_scaled_inputs_mixed_ai(914, A::powf(s.ad_value(912), 0.666666667), (p.p1804 * s.v[913]), 179, 1.60219e-19);s.store_mul_ad_affine_product_rhs(354, 667, s.ad_value(361), A::sub(s.ad_value(352), s.ad_value(353)), -1.0, 0.0);s.store_add_ad(355, A::mul3_scaled_output(s.ad_value(676), s.ad_value(363), A::add_scaled_product(s.ad_value(127), 1.0, s.ad_value(674), A::sqrt(A::offset(s.ad_value(127), 0.01)), 1.0), -1.0), A::mul3(s.ad_value(681), s.ad_value(365), A::pow(A::offset(s.ad_value(127), 0.01), s.ad_value(682))));s.store_mul_ad_product_rhs_mixed_ia(357, 802, 364, A::sqrt(s.ad_value(353)));s.store_add_mixed_ai(358, A::add_scaled_inputs4(s.ad_value(354), 1.0, s.ad_value(355), 1.0, s.ad_value(357), 1.0, s.ad_value(231), 1.0), 805);s.store_sub(347, 347, 358);s.store_div_scaled_product3_indices(184, 416, 163, 158, 1.0, 153, 1.0);s.b[1419] = (p.p80 == 0.0);s.store_scalar(1419, if s.b[1419] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_78(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1419] {s.store_pow_ad(171, A::div_scaled_inputs(s.ad_value(163), (2.0 * p.p108), A::mul3_scaled_output(s.ad_value(184), s.ad_value(181), s.ad_value(148), (1.60219e-19 * p.p3)), 1.0), s.ad_value(181));}
        if s.b[1419] {
            s.store_neg_ad(168, A::add(s.ad_value(375), {
                if (!(s.v[171] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[171] > 1e-38) {
                            A::ln(s.ad_value(171))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }
        if s.b[1419] {s.store_offset_add(169, 347, 168, p.p23);}
        if s.b[1419] {
            s.store_sub_mixed_ai(348, {
                            if (!(s.v[169] < ((-10000.0) * 0.0001))) {
                                A::add_scaled_inputs(s.ad_value(169), 0.5, A::sqrt_square_offset(s.ad_value(169), ((4.0 * 0.0001) * 0.0001)), 0.5)
                            } else {
                                {
                                    if (s.v[169] < ((-10000.0) * 0.0001)) {
                                        A::div_from_scalar(((-0.0001) * 0.0001), s.ad_value(169))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        }, 168);
        }
        if (!s.b[1419]) {
            s.store_mul_scale_offset_mixed_ia(168, 181, {
                if (!((((2.0 * s.v[163]) * p.p108) / ((((s.v[184] * s.v[181]) * 1.60219e-19) * s.v[148]) * p.p3)) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((((2.0 * s.v[163]) * p.p108) / ((((s.v[184] * s.v[181]) * 1.60219e-19) * s.v[148]) * p.p3)) > 1e-38) {
                            A::ln(A::div_scaled_inputs(s.ad_value(163), (2.0 * p.p108), A::mul3_scaled_output(s.ad_value(184), s.ad_value(181), s.ad_value(148), (1.60219e-19 * p.p3)), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        if (!s.b[1419]) {s.store_sub_mixed_ai(169, A::add_scaled_inputs(A::offset(s.ad_value(168), 0.01), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(168), (-0.01)), ((0.25 * 0.0001) * 0.0001)), 0.5), 375);s.store_offset_add(170, 347, 169, p.p23);}
        if (!s.b[1419]) {
            s.store_sub_mixed_ai(348, {
                            if (!(s.v[170] < ((-10000.0) * 0.0001))) {
                                A::add_scaled_inputs(s.ad_value(170), 0.5, A::sqrt_square_offset(s.ad_value(170), ((4.0 * 0.0001) * 0.0001)), 0.5)
                            } else {
                                {
                                    if (s.v[170] < ((-10000.0) * 0.0001)) {
                                        A::div_from_scalar(((-0.0001) * 0.0001), s.ad_value(170))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        }, 169);
        }
        s.copy_ad(129, 375);s.store_powf_ad(172, A::neg(s.ad_value(897)), 0.666666667);s.b[1420] = (p.p61 != 0.0);s.store_scalar(1420, if s.b[1420] { 1.0 } else { 0.0 });
        if s.b[1420] {
            if (!((((2.0 * s.v[166]) + s.v[129]) - s.v[367]) < ((-10000.0) * 0.1))) {
                s.store_add_scaled_inputs4_mixed_iiia(169, 166, (2.0 * 0.5), 129, 0.5, 367, (-0.5), A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(129), 1.0, s.ad_value(367), -1.0), ((4.0 * 0.1) * 0.1)), 0.5);
            } else {
                if ((((2.0 * s.v[166]) + s.v[129]) - s.v[367]) < ((-10000.0) * 0.1)) {
                    s.store_div_from_scalar_ad(169, ((-0.1) * 0.1), A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(129), 1.0, s.ad_value(367), -1.0));
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }
        if s.b[1420] {s.store_mul_div_scaled_inputs_mixed_aii(171, A::sub(A::sqrt(s.ad_value(169)), A::sqrt_scaled_input(s.ad_value(166), 2.0)), 239, -1.0, 181, 2.0);s.store_add_scaled_product_mixed_aii(168, A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(899), 1.0), 1.0, 914, 172, 1.0);s.store_add_scaled_inputs3_indices(169, 897, -1.0, 171, (-1.0), 900, 1.0);}
        if (!s.b[1420]) {s.store_add_scaled_inputs_product_indices(168, 899, 1.0, 897, (-1.0), 914, 172, 1.0);s.store_sub(169, 900, 897);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_79(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_div_scaled_inputs2_indices(170, 348, 1.0, 129, (-1.0), 181, 1.0);s.store_sub(924, 169, 170);s.store_scaled_sub(171, 170, 168, 0.5);s.store_limited_exp(901, 171);s.b[1421] = (s.v[901] > 1e-7);s.store_scalar(1421, if s.b[1421] { 1.0 } else { 0.0 });
        if s.b[1421] {s.store_ln_offset_input(176, 901, 1.0);s.store_offset_scaled_ad(901, A::sqrt_square_offset(s.ad_value(176), 1.0), (-2.0), 2.0);s.store_mul_add_scaled_inputs_rhs_indices(177, 898, 901, p.p1805, 897, 1.0);s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);s.store_mul(174, 177, 172);s.store_ln_neg_add(902, 901, 897);}
        if s.b[1421] {
            s.store_add_scaled_product_mixed_aia(344, A::add_scaled_inputs4(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(901), -1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), 1.0, 914, A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0);
        }
        if s.b[1421] {s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));s.store_mul_add_scaled_inputs_rhs_indices(177, 898, 901, p.p1805, 897, 1.0);s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);s.store_mul(174, 177, 172);s.store_ln_neg_add(902, 901, 897);}
        if s.b[1421] {
            s.store_add_scaled_product_mixed_aia(344, A::add_scaled_inputs4(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(901), -1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), 1.0, 914, A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_80(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1421] {s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));}
        if (!s.b[1421]) {s.store_mul_scale_offset_indices(901, 901, 901, -1.0, 0.0);}
        s.store_mul_scale_offset_indices(392, 181, 901, -1.0, 0.0);s.b[1422] = (p.p57 == 1.0);s.store_scalar(1422, if s.b[1422] { 1.0 } else { 0.0 });
        if s.b[1422] {s.store_div_scaled_inputs2_indices(1015, 347, 1.0, 129, (-1.0), 181, 1.0);s.store_scaled_add_mixed_ia(1016, 1015, A::sqrt(A::add_scaled_square_product(s.ad_value(1015), 1.0, s.ad_value(963), s.ad_value(963), 0.25)), 0.5);s.store_pow_ad(1017, s.ad_value(1016), A::scale(s.ad_value(960), 0.5));s.store_mul_ad_product_rhs_mixed_ia(1004, 1010, 1017, A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));s.store_div_scaled_inputs3_indices(1018, 347, 1.0, 129, (-1.0), 985, -1.0, 181, 1.0);s.store_scaled_add_mixed_ia(1019, 1018, A::sqrt(A::add_scaled_square_product(s.ad_value(1018), 1.0, s.ad_value(964), s.ad_value(964), 0.25)), 0.5);s.store_pow_ad(1020, s.ad_value(1019), A::scale(s.ad_value(961), 0.5));s.store_mul_ad_product_rhs_mixed_ia(1005, 1011, 1020, A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));s.store_div_scaled_inputs3_indices(1021, 347, 1.0, 129, (-1.0), 986, -1.0, 181, 1.0);s.store_scaled_add_mixed_ia(1022, 1021, A::sqrt(A::add_scaled_square_product(s.ad_value(1021), 1.0, s.ad_value(965), s.ad_value(965), 0.25)), 0.5);s.store_pow_ad(1023, s.ad_value(1022), A::scale(s.ad_value(962), 0.5));s.store_mul_ad_product_rhs_mixed_ia(1006, 1012, 1023, A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));s.store_add_scaled_products_mixed_iiia(392, 983, 392, 1.0, 984, A::add_scaled_inputs3(s.ad_value(1004), 1.0, s.ad_value(1005), 1.0, s.ad_value(1006), 1.0), 1.0);}
        s.store_primal_div_from_scalar(406, 0.01, 163);s.store_add_scaled_product_indices(419, 396, s.v[420], 407, 392, s.v[420]);s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(392), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));s.store_pow_indices(171, 419, 822);s.b[1423] = (p.p61 != 0.0);s.store_scalar(1423, if s.b[1423] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_81(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1423] {s.store_add_scaled_product_mixed_aai(171, A::div(s.ad_value(820), s.ad_value(170)), 1.0, A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(821), s.ad_value(370), 1.0), 171, 1.0);}
        if (!s.b[1423]) {s.store_add_scaled_product_mixed_aii(171, A::div(s.ad_value(820), s.ad_value(170)), 1.0, 819, 171, 1.0);}
        s.store_offset(397, 171, 1.0);s.store_scaled_add_offset_sqrt_square_offset(397, 397, 1.0, (-1.0), ((0.25 * p.p604) * p.p604), 0.5);s.store_scale(397, 397, 1.0 / (p.p24));s.b[1424] = (p.p64 == 1.0);s.store_scalar(1424, if s.b[1424] { 1.0 } else { 0.0 });
        if s.b[1424] {s.store_scalar(198, 0.0);}
        s.b[1425] = (p.p64 == 0.0);s.store_scalar(1425, if s.b[1425] { 1.0 } else { 0.0 });
        if ((!s.b[1424]) && s.b[1425]) {s.store_offset_mul(172, 711, 392, 1.0);s.store_div_from_scalar(169, 1.0, 172);s.store_scaled_add_mixed_ia(168, 169, A::sqrt_square_offset(s.ad_value(169), 0.01), 0.5);s.store_mul_ad_affine_product_lhs(198, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189), s.v[115], 0.0, 194);}
        if ((!s.b[1424]) && (!s.b[1425])) {s.store_offset_mul(172, 711, 392, 1.0);s.store_div_from_scalar(169, 1.0, 172);s.store_scaled_add_mixed_ia(168, 169, A::sqrt_square_offset(s.ad_value(169), 0.01), 0.5);s.store_mul_mixed_ai(198, A::add_scaled_inputs_product(s.ad_value(190), 1.0, s.ad_value(191), 1.0, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189), s.v[115]), 194);}
        s.store_mul_div_scaled_inputs_indices(216, 397, 428, 2.0, 416, 1.0);s.store_mul(217, 216, 153);s.b[1426] = (p.p80 == 0.0);s.store_scalar(1426, if s.b[1426] { 1.0 } else { 0.0 });
        if s.b[1426] {s.store_mul_add_scaled_inputs_rhs_indices(175, 659, 392, 1.0, 179, 2.0);}
        if (!s.b[1426]) {s.store_mul_add_scaled_inputs_rhs_indices(175, 659, 392, 1.0, 182, 2.0);}
        s.b[1427] = (s.v[198] > 0.0);s.store_scalar(1427, if s.b[1427] { 1.0 } else { 0.0 });
        if s.b[1427] {s.store_mul3_lhs(224, 158, 428, 163);s.store_mul(168, 224, 198);s.store_scale(225, 168, 2.0);s.store_add_scaled_inputs_product_indices(226, 175, 1.0, 217, 1.0, 175, 168, 3.0);s.store_mul_add_scaled_product_rhs_indices(227, 175, 217, 1.0, 175, 168, 2.0);s.store_div_scaled_inputs2_by_product_mixed_aaai(210, A::square(s.ad_value(226)), 1.0, A::add_scaled_square_product(s.ad_value(226), 1.0, s.ad_value(225), s.ad_value(227), (-2.0)), (-1.0), A::add(s.ad_value(226), A::sqrt(A::add_scaled_square_product(s.ad_value(226), 1.0, s.ad_value(225), s.ad_value(227), (-2.0)))), 225, 1.0);}
        if (!s.b[1427]) {s.store_div_scaled_product_add_scaled_denominator_indices(210, 217, 175, 1.0, 217, 1.0, 175, 1.0, 1.0);}
        s.store_offset_ad(210, {
            if (!((s.v[210] - 0.001) < ((-10000.0) * 1e-5))) {
                A::add_scaled_inputs(A::offset(s.ad_value(210), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(210), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5)
            } else {
                {
                    if ((s.v[210] - 0.001) < ((-10000.0) * 1e-5)) {
                        A::div_scalar_offset_denominator(((-1e-5) * 1e-5), s.ad_value(210), (-0.001), 1.0)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 0.001);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_82(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_pow_ad(176, A::offset(A::div(s.ad_value(126), s.ad_value(210)), 1e-6), s.ad_value(423));s.store_pow_ad(177, A::offset(s.ad_value(176), 1.0), s.ad_value(212));s.store_min_ad(390, A::div(s.ad_value(126), s.ad_value(177)), s.ad_value(126));s.store_add(129, 390, 375);s.store_powf_ad(170, A::neg(s.ad_value(897)), 0.666666667);s.b[1428] = (p.p61 != 0.0);s.store_scalar(1428, if s.b[1428] { 1.0 } else { 0.0 });
        if s.b[1428] {
            if (!((((2.0 * s.v[166]) + s.v[129]) - s.v[367]) < ((-10000.0) * 0.1))) {
                s.store_add_scaled_inputs4_mixed_iiia(169, 166, (2.0 * 0.5), 129, 0.5, 367, (-0.5), A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(129), 1.0, s.ad_value(367), -1.0), ((4.0 * 0.1) * 0.1)), 0.5);
            } else {
                if ((((2.0 * s.v[166]) + s.v[129]) - s.v[367]) < ((-10000.0) * 0.1)) {
                    s.store_div_from_scalar_ad(169, ((-0.1) * 0.1), A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(129), 1.0, s.ad_value(367), -1.0));
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }
        if s.b[1428] {s.store_mul_div_scaled_inputs_mixed_aii(171, A::sub(A::sqrt(s.ad_value(169)), A::sqrt_scaled_input(s.ad_value(166), 2.0)), 239, -1.0, 181, 2.0);s.store_add_scaled_product_mixed_aii(168, A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(899), 1.0), 1.0, 914, 170, 1.0);s.store_add_scaled_inputs3_indices(169, 897, -1.0, 171, (-1.0), 900, 1.0);}
        if (!s.b[1428]) {s.store_add_scaled_inputs_product_indices(168, 899, 1.0, 897, (-1.0), 914, 170, 1.0);s.store_sub(169, 900, 897);}
        s.store_div_scaled_inputs2_indices(170, 348, 1.0, 129, (-1.0), 181, 1.0);s.store_sub(924, 169, 170);s.store_scaled_sub(171, 170, 168, 0.5);s.store_limited_exp(901, 171);s.b[1429] = (s.v[901] > 1e-7);s.store_scalar(1429, if s.b[1429] { 1.0 } else { 0.0 });
        if s.b[1429] {s.store_ln_offset_input(176, 901, 1.0);s.store_offset_scaled_ad(901, A::sqrt_square_offset(s.ad_value(176), 1.0), (-2.0), 2.0);s.store_mul_add_scaled_inputs_rhs_indices(177, 898, 901, p.p1805, 897, 1.0);s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);s.store_mul(174, 177, 172);s.store_ln_neg_add(902, 901, 897);}
        if s.b[1429] {
            s.store_add_scaled_product_mixed_aia(344, A::add_scaled_inputs4(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(901), -1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), 1.0, 914, A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_83(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1429] {s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));s.store_mul_add_scaled_inputs_rhs_indices(177, 898, 901, p.p1805, 897, 1.0);s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);s.store_mul(174, 177, 172);s.store_ln_neg_add(902, 901, 897);}
        if s.b[1429] {
            s.store_add_scaled_product_mixed_aia(344, A::add_scaled_inputs4(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(901), -1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), 1.0, 914, A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0);
        }
        if s.b[1429] {s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));}
        if (!s.b[1429]) {s.store_mul_scale_offset_indices(901, 901, 901, -1.0, 0.0);}
        s.store_mul_scale_offset_indices(393, 181, 901, -1.0, 0.0);s.b[1430] = (p.p57 == 1.0);s.store_scalar(1430, if s.b[1430] { 1.0 } else { 0.0 });
        if s.b[1430] {s.store_div_scaled_inputs2_indices(1015, 347, 1.0, 129, (-1.0), 181, 1.0);s.store_scaled_add_mixed_ia(1016, 1015, A::sqrt(A::add_scaled_square_product(s.ad_value(1015), 1.0, s.ad_value(963), s.ad_value(963), 0.25)), 0.5);s.store_pow_ad(1017, s.ad_value(1016), A::scale(s.ad_value(960), 0.5));s.store_mul_ad_product_rhs_mixed_ia(1007, 1010, 1017, A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));s.store_div_scaled_inputs3_indices(1018, 347, 1.0, 129, (-1.0), 985, -1.0, 181, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_84(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1430] {s.store_scaled_add_mixed_ia(1019, 1018, A::sqrt(A::add_scaled_square_product(s.ad_value(1018), 1.0, s.ad_value(964), s.ad_value(964), 0.25)), 0.5);s.store_pow_ad(1020, s.ad_value(1019), A::scale(s.ad_value(961), 0.5));s.store_mul_ad_product_rhs_mixed_ia(1008, 1011, 1020, A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));s.store_div_scaled_inputs3_indices(1021, 347, 1.0, 129, (-1.0), 986, -1.0, 181, 1.0);s.store_scaled_add_mixed_ia(1022, 1021, A::sqrt(A::add_scaled_square_product(s.ad_value(1021), 1.0, s.ad_value(965), s.ad_value(965), 0.25)), 0.5);s.store_pow_ad(1023, s.ad_value(1022), A::scale(s.ad_value(962), 0.5));s.store_mul_ad_product_rhs_mixed_ia(1009, 1012, 1023, A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));s.store_add_scaled_products_mixed_iiia(393, 983, 393, 1.0, 984, A::add_scaled_inputs3(s.ad_value(1007), 1.0, s.ad_value(1008), 1.0, s.ad_value(1009), 1.0), 1.0);}
        s.b[1431] = (p.p67 == 1.0);s.store_scalar(1431, if s.b[1431] { 1.0 } else { 0.0 });
        if s.b[1431] {s.store_add_ad(356, A::mul3_scaled_output(s.ad_value(297), s.ad_value(363), A::add_scaled_product(s.ad_value(127), 1.0, s.ad_value(674), A::sqrt(A::offset(s.ad_value(127), 0.01)), 1.0), -1.0), A::mul3(s.ad_value(681), s.ad_value(365), A::pow(A::offset(s.ad_value(127), 0.01), s.ad_value(682))));s.store_add_mixed_ai(359, A::add_scaled_inputs4(s.ad_value(354), 1.0, s.ad_value(356), 1.0, s.ad_value(357), 1.0, s.ad_value(231), 1.0), 805);s.store_add_scaled_inputs3_indices(349, 125, 1.0, 167, (-1.0), 359, -1.0);s.store_div_scaled_product3_indices(185, 414, 163, 158, 1.0, 153, 1.0);}
        s.b[1432] = (p.p80 == 0.0);s.store_scalar(1432, if s.b[1432] { 1.0 } else { 0.0 });
        if (s.b[1431] && s.b[1432]) {s.store_pow_ad(171, A::div_scaled_inputs(s.ad_value(163), (2.0 * p.p108), A::mul3_scaled_output(s.ad_value(185), s.ad_value(181), s.ad_value(148), (1.60219e-19 * p.p3)), 1.0), s.ad_value(181));}
        if (s.b[1431] && s.b[1432]) {
            s.store_neg_ad(168, A::add(s.ad_value(375), {
                if (!(s.v[171] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[171] > 1e-38) {
                            A::ln(s.ad_value(171))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }
        if (s.b[1431] && s.b[1432]) {s.store_offset_add(169, 349, 168, p.p23);}
        if (s.b[1431] && s.b[1432]) {
            s.store_sub_mixed_ai(350, {
                            if (!(s.v[169] < ((-10000.0) * 0.0001))) {
                                A::add_scaled_inputs(s.ad_value(169), 0.5, A::sqrt_square_offset(s.ad_value(169), ((4.0 * 0.0001) * 0.0001)), 0.5)
                            } else {
                                {
                                    if (s.v[169] < ((-10000.0) * 0.0001)) {
                                        A::div_from_scalar(((-0.0001) * 0.0001), s.ad_value(169))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        }, 168);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_85(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1431] && (!s.b[1432])) {
            s.store_mul_scale_offset_mixed_ia(168, 181, {
                if (!((((2.0 * s.v[163]) * p.p108) / ((((s.v[185] * s.v[181]) * 1.60219e-19) * s.v[148]) * p.p3)) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((((2.0 * s.v[163]) * p.p108) / ((((s.v[185] * s.v[181]) * 1.60219e-19) * s.v[148]) * p.p3)) > 1e-38) {
                            A::ln(A::div_scaled_inputs(s.ad_value(163), (2.0 * p.p108), A::mul3_scaled_output(s.ad_value(185), s.ad_value(181), s.ad_value(148), (1.60219e-19 * p.p3)), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        if (s.b[1431] && (!s.b[1432])) {s.store_sub_mixed_ai(169, A::add_scaled_inputs(A::offset(s.ad_value(168), 0.01), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(168), (-0.01)), ((0.25 * 0.0001) * 0.0001)), 0.5), 375);s.store_offset_add(170, 349, 169, p.p23);}
        if (s.b[1431] && (!s.b[1432])) {
            s.store_sub_mixed_ai(350, {
                            if (!(s.v[170] < ((-10000.0) * 0.0001))) {
                                A::add_scaled_inputs(s.ad_value(170), 0.5, A::sqrt_square_offset(s.ad_value(170), ((4.0 * 0.0001) * 0.0001)), 0.5)
                            } else {
                                {
                                    if (s.v[170] < ((-10000.0) * 0.0001)) {
                                        A::div_from_scalar(((-0.0001) * 0.0001), s.ad_value(170))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        }, 169);
        }
        if s.b[1431] {s.copy_ad(130, 375);s.store_powf_ad(172, A::neg(s.ad_value(897)), 0.666666667);}
        s.b[1433] = (p.p61 != 0.0);s.store_scalar(1433, if s.b[1433] { 1.0 } else { 0.0 });
        if (s.b[1431] && s.b[1433]) {
            if (!((((2.0 * s.v[166]) + s.v[130]) - s.v[367]) < ((-10000.0) * 0.1))) {
                s.store_add_scaled_inputs4_mixed_iiia(169, 166, (2.0 * 0.5), 130, 0.5, 367, (-0.5), A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(130), 1.0, s.ad_value(367), -1.0), ((4.0 * 0.1) * 0.1)), 0.5);
            } else {
                if ((((2.0 * s.v[166]) + s.v[130]) - s.v[367]) < ((-10000.0) * 0.1)) {
                    s.store_div_from_scalar_ad(169, ((-0.1) * 0.1), A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(130), 1.0, s.ad_value(367), -1.0));
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }
        if (s.b[1431] && s.b[1433]) {s.store_mul_div_scaled_inputs_mixed_aii(171, A::sub(A::sqrt(s.ad_value(169)), A::sqrt_scaled_input(s.ad_value(166), 2.0)), 239, -1.0, 181, 2.0);s.store_add_scaled_product_mixed_aii(168, A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(899), 1.0), 1.0, 914, 172, 1.0);s.store_add_scaled_inputs3_indices(169, 897, -1.0, 171, (-1.0), 900, 1.0);}
        if (s.b[1431] && (!s.b[1433])) {s.store_add_scaled_inputs_product_indices(168, 899, 1.0, 897, (-1.0), 914, 172, 1.0);s.store_sub(169, 900, 897);}
        if s.b[1431] {s.store_div_scaled_inputs2_indices(170, 350, 1.0, 130, (-1.0), 181, 1.0);s.store_sub(924, 169, 170);s.store_scaled_sub(171, 170, 168, 0.5);s.store_limited_exp(901, 171);}
        s.b[1434] = (s.v[901] > 1e-7);s.store_scalar(1434, if s.b[1434] { 1.0 } else { 0.0 });
        if (s.b[1431] && s.b[1434]) {s.store_ln_offset_input(176, 901, 1.0);s.store_offset_scaled_ad(901, A::sqrt_square_offset(s.ad_value(176), 1.0), (-2.0), 2.0);s.store_mul_add_scaled_inputs_rhs_indices(177, 898, 901, p.p1805, 897, 1.0);s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);s.store_mul(174, 177, 172);s.store_ln_neg_add(902, 901, 897);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_86(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1431] && s.b[1434]) {
            s.store_add_scaled_product_mixed_aia(344, A::add_scaled_inputs4(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(901), -1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), 1.0, 914, A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0);
        }
        if (s.b[1431] && s.b[1434]) {s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));s.store_mul_add_scaled_inputs_rhs_indices(177, 898, 901, p.p1805, 897, 1.0);s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);s.store_mul(174, 177, 172);s.store_ln_neg_add(902, 901, 897);}
        if (s.b[1431] && s.b[1434]) {
            s.store_add_scaled_product_mixed_aia(344, A::add_scaled_inputs4(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(901), -1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), 1.0, 914, A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0);
        }
        if (s.b[1431] && s.b[1434]) {s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));}
        if (s.b[1431] && (!s.b[1434])) {s.store_mul_scale_offset_indices(901, 901, 901, -1.0, 0.0);}
        if s.b[1431] {s.store_mul_scale_offset_indices(394, 181, 901, -1.0, 0.0);}
        s.b[1435] = (p.p57 == 1.0);s.store_scalar(1435, if s.b[1435] { 1.0 } else { 0.0 });
        if (s.b[1431] && s.b[1435]) {s.store_div_scaled_inputs2_indices(1015, 349, 1.0, 130, (-1.0), 181, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_87(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1431] && s.b[1435]) {s.store_scaled_add_mixed_ia(1016, 1015, A::sqrt(A::add_scaled_square_product(s.ad_value(1015), 1.0, s.ad_value(963), s.ad_value(963), 0.25)), 0.5);s.store_pow_ad(1017, s.ad_value(1016), A::scale(s.ad_value(960), 0.5));s.store_mul_ad_product_rhs_mixed_ia(1004, 1010, 1017, A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));s.store_div_scaled_inputs3_indices(1018, 349, 1.0, 130, (-1.0), 985, -1.0, 181, 1.0);s.store_scaled_add_mixed_ia(1019, 1018, A::sqrt(A::add_scaled_square_product(s.ad_value(1018), 1.0, s.ad_value(964), s.ad_value(964), 0.25)), 0.5);s.store_pow_ad(1020, s.ad_value(1019), A::scale(s.ad_value(961), 0.5));s.store_mul_ad_product_rhs_mixed_ia(1005, 1011, 1020, A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));s.store_div_scaled_inputs3_indices(1021, 349, 1.0, 130, (-1.0), 986, -1.0, 181, 1.0);s.store_scaled_add_mixed_ia(1022, 1021, A::sqrt(A::add_scaled_square_product(s.ad_value(1021), 1.0, s.ad_value(965), s.ad_value(965), 0.25)), 0.5);s.store_pow_ad(1023, s.ad_value(1022), A::scale(s.ad_value(962), 0.5));s.store_mul_ad_product_rhs_mixed_ia(1006, 1012, 1023, A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));s.store_add_scaled_products_mixed_iiia(394, 983, 394, 1.0, 984, A::add_scaled_inputs3(s.ad_value(1004), 1.0, s.ad_value(1005), 1.0, s.ad_value(1006), 1.0), 1.0);}
        if s.b[1431] {s.store_add_scaled_product_indices(421, 396, s.v[420], 407, 394, s.v[420]);s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(394), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));s.store_pow_indices(171, 421, 822);}
        s.b[1436] = (p.p61 != 0.0);s.store_scalar(1436, if s.b[1436] { 1.0 } else { 0.0 });
        if (s.b[1431] && s.b[1436]) {s.store_add_scaled_product_mixed_aai(171, A::div(s.ad_value(319), s.ad_value(170)), 1.0, A::add_scaled_product(s.ad_value(304), 1.0, s.ad_value(315), s.ad_value(370), 1.0), 171, 1.0);}
        if (s.b[1431] && (!s.b[1436])) {s.store_add_scaled_product_mixed_aii(171, A::div(s.ad_value(319), s.ad_value(170)), 1.0, 304, 171, 1.0);}
        if s.b[1431] {s.store_offset(398, 171, 1.0);s.store_scaled_add_offset_sqrt_square_offset(398, 398, 1.0, (-1.0), ((0.25 * p.p604) * p.p604), 0.5);s.store_scale(398, 398, 1.0 / (p.p24));}
        s.b[1437] = (p.p64 == 1.0);s.store_scalar(1437, if s.b[1437] { 1.0 } else { 0.0 });
        if (s.b[1431] && s.b[1437]) {s.store_scalar(199, 0.0);}
        s.b[1438] = (p.p64 == 0.0);s.store_scalar(1438, if s.b[1438] { 1.0 } else { 0.0 });
        if ((s.b[1431] && (!s.b[1437])) && s.b[1438]) {s.store_offset_mul(172, 711, 394, 1.0);s.store_div_from_scalar(169, 1.0, 172);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_88(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1431] && (!s.b[1437])) && s.b[1438]) {s.store_scaled_add_mixed_ia(168, 169, A::sqrt_square_offset(s.ad_value(169), 0.01), 0.5);s.store_mul_ad_affine_product_lhs(199, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189), s.v[115], 0.0, 194);}
        if ((s.b[1431] && (!s.b[1437])) && (!s.b[1438])) {s.store_offset_mul(172, 711, 394, 1.0);s.store_div_from_scalar(169, 1.0, 172);s.store_scaled_add_mixed_ia(168, 169, A::sqrt_square_offset(s.ad_value(169), 0.01), 0.5);s.store_mul_mixed_ai(199, A::add_scaled_inputs_product(s.ad_value(190), 1.0, s.ad_value(191), 1.0, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189), s.v[115]), 194);}
        if s.b[1431] {s.store_mul_div_scaled_inputs_indices(222, 398, 336, 2.0, 414, 1.0);s.store_mul(223, 222, 153);}
        s.b[1439] = (p.p80 == 0.0);s.store_scalar(1439, if s.b[1439] { 1.0 } else { 0.0 });
        if (s.b[1431] && s.b[1439]) {s.store_mul_add_scaled_inputs_rhs_indices(175, 659, 394, 1.0, 179, 2.0);}
        if (s.b[1431] && (!s.b[1439])) {s.store_mul_add_scaled_inputs_rhs_indices(175, 659, 394, 1.0, 182, 2.0);}
        s.b[1440] = (s.v[199] > 0.0);s.store_scalar(1440, if s.b[1440] { 1.0 } else { 0.0 });
        if (s.b[1431] && s.b[1440]) {s.store_mul_product3_indices(168, 199, 158, 336, 163, 1.0);s.store_scale(225, 168, 2.0);s.store_add_scaled_inputs_product_indices(226, 175, 1.0, 223, 1.0, 175, 168, 3.0);s.store_mul_add_scaled_product_rhs_indices(227, 175, 223, 1.0, 175, 168, 2.0);s.store_div_scaled_inputs2_by_product_mixed_aaai(211, A::square(s.ad_value(226)), 1.0, A::add_scaled_square_product(s.ad_value(226), 1.0, s.ad_value(225), s.ad_value(227), (-2.0)), (-1.0), A::add(s.ad_value(226), A::sqrt(A::add_scaled_square_product(s.ad_value(226), 1.0, s.ad_value(225), s.ad_value(227), (-2.0)))), 225, 1.0);}
        if (s.b[1431] && (!s.b[1440])) {s.store_div_scaled_product_add_scaled_denominator_indices(211, 223, 175, 1.0, 223, 1.0, 175, 1.0, 1.0);}
        if s.b[1431] {
            s.store_offset_ad(211, {
                if (!((s.v[211] - 0.001) < ((-10000.0) * 1e-5))) {
                    A::add_scaled_inputs(A::offset(s.ad_value(211), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(211), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5)
                } else {
                    {
                        if ((s.v[211] - 0.001) < ((-10000.0) * 1e-5)) {
                            A::div_scalar_offset_denominator(((-1e-5) * 1e-5), s.ad_value(211), (-0.001), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.001);
        }
        if s.b[1431] {s.store_pow_ad(176, A::offset(A::div(s.ad_value(126), s.ad_value(211)), 1e-6), s.ad_value(423));s.store_pow_ad(177, A::offset(s.ad_value(176), 1.0), s.ad_value(212));s.store_min_ad(391, A::div(s.ad_value(126), s.ad_value(177)), s.ad_value(126));s.store_add(130, 391, 375);s.store_powf_ad(170, A::neg(s.ad_value(897)), 0.666666667);}
        s.b[1441] = (p.p61 != 0.0);s.store_scalar(1441, if s.b[1441] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_89(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1431] && s.b[1441]) {
            if (!((((2.0 * s.v[166]) + s.v[130]) - s.v[367]) < ((-10000.0) * 0.1))) {
                s.store_add_scaled_inputs4_mixed_iiia(169, 166, (2.0 * 0.5), 130, 0.5, 367, (-0.5), A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(130), 1.0, s.ad_value(367), -1.0), ((4.0 * 0.1) * 0.1)), 0.5);
            } else {
                if ((((2.0 * s.v[166]) + s.v[130]) - s.v[367]) < ((-10000.0) * 0.1)) {
                    s.store_div_from_scalar_ad(169, ((-0.1) * 0.1), A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(130), 1.0, s.ad_value(367), -1.0));
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }
        if (s.b[1431] && s.b[1441]) {s.store_mul_div_scaled_inputs_mixed_aii(171, A::sub(A::sqrt(s.ad_value(169)), A::sqrt_scaled_input(s.ad_value(166), 2.0)), 239, -1.0, 181, 2.0);s.store_add_scaled_product_mixed_aii(168, A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(899), 1.0), 1.0, 914, 170, 1.0);s.store_add_scaled_inputs3_indices(169, 897, -1.0, 171, (-1.0), 900, 1.0);}
        if (s.b[1431] && (!s.b[1441])) {s.store_add_scaled_inputs_product_indices(168, 899, 1.0, 897, (-1.0), 914, 170, 1.0);s.store_sub(169, 900, 897);}
        if s.b[1431] {s.store_div_scaled_inputs2_indices(170, 350, 1.0, 130, (-1.0), 181, 1.0);s.store_sub(924, 169, 170);s.store_scaled_sub(171, 170, 168, 0.5);s.store_limited_exp(901, 171);}
        s.b[1442] = (s.v[901] > 1e-7);s.store_scalar(1442, if s.b[1442] { 1.0 } else { 0.0 });
        if (s.b[1431] && s.b[1442]) {s.store_ln_offset_input(176, 901, 1.0);s.store_offset_scaled_ad(901, A::sqrt_square_offset(s.ad_value(176), 1.0), (-2.0), 2.0);s.store_mul_add_scaled_inputs_rhs_indices(177, 898, 901, p.p1805, 897, 1.0);s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);s.store_mul(174, 177, 172);s.store_ln_neg_add(902, 901, 897);}
        if (s.b[1431] && s.b[1442]) {
            s.store_add_scaled_product_mixed_aia(344, A::add_scaled_inputs4(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(901), -1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), 1.0, 914, A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0);
        }
        if (s.b[1431] && s.b[1442]) {s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));s.store_mul_add_scaled_inputs_rhs_indices(177, 898, 901, p.p1805, 897, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_90(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1431] && s.b[1442]) {s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);s.store_mul(174, 177, 172);s.store_ln_neg_add(902, 901, 897);}
        if (s.b[1431] && s.b[1442]) {
            s.store_add_scaled_product_mixed_aia(344, A::add_scaled_inputs4(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(901), -1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), 1.0, 914, A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0);
        }
        if (s.b[1431] && s.b[1442]) {s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));}
        if (s.b[1431] && (!s.b[1442])) {s.store_mul_scale_offset_indices(901, 901, 901, -1.0, 0.0);}
        if s.b[1431] {s.store_mul_scale_offset_indices(395, 181, 901, -1.0, 0.0);}
        s.b[1443] = (p.p57 == 1.0);s.store_scalar(1443, if s.b[1443] { 1.0 } else { 0.0 });
        if (s.b[1431] && s.b[1443]) {s.store_div_scaled_inputs2_indices(1015, 349, 1.0, 130, (-1.0), 181, 1.0);s.store_scaled_add_mixed_ia(1016, 1015, A::sqrt(A::add_scaled_square_product(s.ad_value(1015), 1.0, s.ad_value(963), s.ad_value(963), 0.25)), 0.5);s.store_pow_ad(1017, s.ad_value(1016), A::scale(s.ad_value(960), 0.5));s.store_mul_ad_product_rhs_mixed_ia(1007, 1010, 1017, A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));s.store_div_scaled_inputs3_indices(1018, 349, 1.0, 130, (-1.0), 985, -1.0, 181, 1.0);s.store_scaled_add_mixed_ia(1019, 1018, A::sqrt(A::add_scaled_square_product(s.ad_value(1018), 1.0, s.ad_value(964), s.ad_value(964), 0.25)), 0.5);s.store_pow_ad(1020, s.ad_value(1019), A::scale(s.ad_value(961), 0.5));s.store_mul_ad_product_rhs_mixed_ia(1008, 1011, 1020, A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));s.store_div_scaled_inputs3_indices(1021, 349, 1.0, 130, (-1.0), 986, -1.0, 181, 1.0);s.store_scaled_add_mixed_ia(1022, 1021, A::sqrt(A::add_scaled_square_product(s.ad_value(1021), 1.0, s.ad_value(965), s.ad_value(965), 0.25)), 0.5);s.store_pow_ad(1023, s.ad_value(1022), A::scale(s.ad_value(962), 0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_91(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1431] && s.b[1443]) {s.store_mul_ad_product_rhs_mixed_ia(1009, 1012, 1023, A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));s.store_add_scaled_products_mixed_iiia(395, 983, 395, 1.0, 984, A::add_scaled_inputs3(s.ad_value(1007), 1.0, s.ad_value(1008), 1.0, s.ad_value(1009), 1.0), 1.0);}
        if s.b[1431] {s.store_scaled_add(403, 394, 395, 0.5);s.store_sub(405, 394, 395);s.store_scaled_square(168, 391, 1600.0);}
        s.b[1444] = (p.p603 != 0.0);s.store_scalar(1444, if s.b[1444] { 1.0 } else { 0.0 });
        if (s.b[1431] && s.b[1444]) {s.store_add_scaled_inputs3_mixed_iia(404, 394, 0.5, 395, 0.5, A::mul_sub_from_scalar_lhs_scaled_output(1.0, A::limited_exp_scaled_input(s.ad_value(168), -1.0), s.ad_value(405), (p.p603 * 0.5)), 1.0);}
        if (s.b[1431] && (!s.b[1444])) {s.store_scaled_add(404, 394, 395, 0.5);}
        s.b[1445] = (p.p61 != 0.0);s.store_scalar(1445, if s.b[1445] { 1.0 } else { 0.0 });
        if s.b[1445] {s.store_mul_div_scaled_inputs_mixed_aii(178, A::sqrt(s.ad_value(179)), 239, 1.0, 181, 2.0);s.store_scale(168, 178, 0.5);}
        if s.b[1445] {
            s.store_div_scaled_inputs2_mixed_iai(170, 497, 1.0, A::offset(A::add_scaled_inputs_product(s.ad_value(167), 1.0, s.ad_value(146), (-1.0), s.ad_value(179), {
                if (!((s.v[640] / s.v[148]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[640] / s.v[148]) > 1e-38) {
                            A::ln(A::div(s.ad_value(640), s.ad_value(148)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, (-1.0)), p.p1529), (-1.0), 179, 1.0);
        }
        s.b[1446] = ((s.v[170] * s.v[179]) > (s.v[166] + (s.v[178] * (((s.v[166] * s.v[179])) as f64).sqrt())));s.store_scalar(1446, if s.b[1446] { 1.0 } else { 0.0 });
        if (s.b[1445] && s.b[1446]) {s.store_sub_mixed_ai(169, A::sqrt(A::add(A::offset(s.ad_value(170), (-1.0)), A::square(s.ad_value(168)))), 168);s.store_offset_square(340, 169, 1.0);}
        if (s.b[1445] && s.b[1446]) {
            if (!((((-s.v[340])) as f64).abs() < 1e-7)) {
                s.store_offset_ad(175, A::limited_exp_scaled_input(s.ad_value(340), -1.0), (-1.0));
            } else {
                if ((((-s.v[340])) as f64).abs() < 1e-7) {
                    s.store_sub_mixed_ai(175, A::mul_scaled_lhs(s.ad_value(340), (-(-0.5)), s.ad_value(340)), 340);
                } else {
                    s.store_scalar(175, 0.0);
                }
            }
        }
        if (s.b[1445] && (!s.b[1446])) {s.store_sub_scaled_inputs_mixed_ia(171, 170, 0.5, A::scale_offset(s.ad_value(178), ((1.0 / (((2.0) as f64).sqrt())) * (3.0)), 3.0), 1.0);s.store_add_mixed_ia(340, 171, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(171)), 1.0, s.ad_value(170), 6.0)));}
        s.b[1447] = (s.v[170] < 0.0);s.store_scalar(1447, if s.b[1447] { 1.0 } else { 0.0 });
        if ((s.b[1445] && (!s.b[1446])) && s.b[1447]) {s.store_div_scaled_inputs2_indices(172, 170, 1.0, 340, (-1.0), 178, 1.0);s.store_sub_square_lhs(175, 172, 340);}
        if ((s.b[1445] && (!s.b[1446])) && s.b[1447]) {
            s.store_neg_ad(340, {
                if (!(((1.0 - s.v[340]) + (s.v[172] * s.v[172])) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((1.0 - s.v[340]) + (s.v[172] * s.v[172])) > 1e-38) {
                            A::ln(A::add(A::sub_from_scalar(1.0, s.ad_value(340)), A::square(s.ad_value(172))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }
        if ((s.b[1445] && (!s.b[1446])) && (!s.b[1447])) {s.store_limited_exp_neg_input(341, 340);s.store_sub_mixed_ai(172, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(170), 1.0, s.ad_value(341), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_92(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1445] && (!s.b[1446])) && (!s.b[1447])) {s.store_add_ad(340, A::sub_from_scalar(1.0, s.ad_value(341)), A::square(s.ad_value(172)));}
        if ((s.b[1445] && (!s.b[1446])) && (!s.b[1447])) {
            if (!((((-s.v[340])) as f64).abs() < 1e-7)) {
                s.store_offset_ad(175, A::limited_exp_scaled_input(s.ad_value(340), -1.0), (-1.0));
            } else {
                if ((((-s.v[340])) as f64).abs() < 1e-7) {
                    s.store_sub_mixed_ai(175, A::mul_scaled_lhs(s.ad_value(340), (-(-0.5)), s.ad_value(340)), 340);
                } else {
                    s.store_scalar(175, 0.0);
                }
            }
        }
        if s.b[1445] {s.store_sqrt_add(176, 175, 340);}
        s.b[1448] = (s.v[340] > 1e-15);s.store_scalar(1448, if s.b[1448] { 1.0 } else { 0.0 });
        if (s.b[1445] && s.b[1448]) {s.store_add_scaled_inputs_product_indices(344, 170, -1.0, 340, 1.0, 178, 176, 1.0);s.store_sub_from_scalar_ad(345, 1.0, A::div_scaled_product(s.ad_value(178), s.ad_value(175), 0.5, s.ad_value(176), 1.0));s.store_sub_div_rhs_indices(177, 340, 344, 345);}
        if (s.b[1445] && s.b[1448]) {
            if (!((((-s.v[177])) as f64).abs() < 1e-7)) {
                s.store_offset_ad(341, A::limited_exp_scaled_input(s.ad_value(177), -1.0), (-1.0));
            } else {
                if ((((-s.v[177])) as f64).abs() < 1e-7) {
                    s.store_sub_mixed_ai(341, A::mul_scaled_lhs(s.ad_value(177), (-(-0.5)), s.ad_value(177)), 177);
                } else {
                    s.store_scalar(341, 0.0);
                }
            }
        }
        if (s.b[1445] && s.b[1448]) {s.store_sqrt_add(342, 341, 177);s.store_mul3_affine_lhs(401, 178, 342, -1.0, 0.0, 179);}
        s.b[1449] = (s.v[340] < (-1e-15));s.store_scalar(1449, if s.b[1449] { 1.0 } else { 0.0 });
        if ((s.b[1445] && (!s.b[1448])) && s.b[1449]) {s.store_add_scaled_inputs_product_indices(344, 170, -1.0, 340, 1.0, 178, 176, (-1.0));s.store_offset_div_scaled_product_indices(345, 178, 175, 0.5, 176, 1.0, 1.0);s.store_sub_div_rhs_indices(177, 340, 344, 345);}
        if ((s.b[1445] && (!s.b[1448])) && s.b[1449]) {
            s.store_add_mixed_ai(343, {
                            if (!((((-s.v[177])) as f64).abs() < 1e-7)) {
                                A::offset(A::limited_exp_scaled_input(s.ad_value(177), -1.0), (-1.0))
                            } else {
                                {
                                    if ((((-s.v[177])) as f64).abs() < 1e-7) {
                                        A::sub(A::mul_scaled_lhs(s.ad_value(177), (-(-0.5)), s.ad_value(177)), s.ad_value(177))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        }, 177);
        }
        if ((s.b[1445] && (!s.b[1448])) && s.b[1449]) {s.store_mul_sqrt_rhs(342, 178, 343);}
        if ((s.b[1445] && (!s.b[1448])) && (!s.b[1449])) {s.store_scalar(177, 0.0);s.store_scalar(342, 0.0);}
        if (s.b[1445] && (!s.b[1448])) {s.store_mul(401, 342, 179);}
        if s.b[1445] {s.store_mul_ad_product_lhs_mixed_ia(904, 178, A::limited_exp_scaled_input(s.ad_value(177), (-1.0 / (2.0))), 179);s.store_scaled_add_offset_sqrt_square_offset(921, 177, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(922, 921);s.store_offset_div(923, 178, 922, 1.0);}
        s.store_scaled_add(399, 392, 393, 0.5);s.store_sub(402, 392, 393);s.store_scaled_square(168, 390, 1600.0);s.b[1450] = (p.p603 != 0.0);s.store_scalar(1450, if s.b[1450] { 1.0 } else { 0.0 });
        if s.b[1450] {s.store_add_scaled_inputs3_mixed_iia(400, 392, 0.5, 393, 0.5, A::mul_sub_from_scalar_lhs_scaled_output(1.0, A::limited_exp_scaled_input(s.ad_value(168), -1.0), s.ad_value(402), (p.p603 * 0.5)), 1.0);}
        if (!s.b[1450]) {s.store_scaled_add(400, 392, 393, 0.5);}
        s.b[1451] = (s.v[655] > 0.0);s.store_scalar(1451, if s.b[1451] { 1.0 } else { 0.0 });
        if s.b[1451] {s.store_scale(172, 399, 1.0 / (p.p400));s.store_offset_pow_ad(174, s.ad_value(172), s.ad_value(661), 1.0);s.store_div(374, 373, 174);s.store_div_from_scalar_ad(372, 1.0, A::add_scaled_product(A::div_from_scalar(1.0, A::scale(s.ad_value(163), (p.p89 * 1.0 / (p.p90)))), 1.0, s.ad_value(374), s.ad_value(655), 1.0 / (s.v[143])));}
        if (!s.b[1451]) {s.copy_ad(372, 163);}
    }
}
