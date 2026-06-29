#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[1416]) {
            s.copy_ad(426, 301);
            s.copy_ad(795, 797);
            s.copy_ad(428, 332);
            s.copy_ad(659, 660);
            s.copy_ad(805, 804);
            s.copy_ad(669, 666);
            s.copy_ad(416, 413);
            s.copy_ad(819, 303);
            s.copy_ad(820, 318);
            s.copy_ad(821, 314);
            s.copy_ad(822, 323);
        }

        s.store_div_from_scalar(212, 1.0, 423);

        s.store_add_offset_lhs(353, 166, 0.4, 672);

        s.store_div_scaled_value_by_product(169, s.ad_value(893), 2.0, s.ad_value(895), A::offset(s.ad_value(898), 2.0), 1.0);

        s.store_mul_add_scaled_product_rhs(164, 362, s.ad_value(662), 1.0, s.ad_value(664), s.ad_value(127), 1.0);

        s.b[1417] = (p.p175 == 0.0);
        s.v[1417] = if s.b[1417] { 1.0 } else { 0.0 };

        s.b[1418] = (p.p80 == 0.0);
        s.v[1418] = if s.b[1418] { 1.0 } else { 0.0 };

        if (s.b[1417] && s.b[1418]) {
            s.store_mul_ad_product_rhs_mixed_ia(181, 179, 235, A::offset(A::div_scaled_inputs2(s.ad_value(669), 1.0, s.ad_value(164), 1.0, s.ad_value(169), 1.0), 1.0));
        }

        if (s.b[1417] && (!s.b[1418])) {
            s.store_mul_ad_product_rhs_mixed_ia(181, 182, 235, A::offset(A::div_scaled_inputs2(s.ad_value(669), 1.0, s.ad_value(164), 1.0, s.ad_value(169), 1.0), 1.0));
        }

        if (!s.b[1417]) {
            s.store_scalar(181, p.p175);
        }

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

        s.store_add_ad_lhs(899, {
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

        s.store_add_scaled_ad_lhs(339, A::div_scaled_inputs(s.ad_value(181), 10.0, s.ad_value(898), 1.0), 396, 2.0);

        s.store_div_scaled_product_indices(912, 179, 893, 1.0, 895, s.v[143]);

        s.v[913] = ((((((4.5 * 1.05457e-34) * 3.141592653589793) * 1.60219e-19) / (4.0 * (((2.0 * s.v[381])) as f64).sqrt()))) as f64).powf(0.666666667);

        s.store_div_scaled_inputs_mixed_ai(914, A::powf(s.ad_value(912), 0.666666667), (p.p1804 * s.v[913]), 179, 1.60219e-19);

        s.store_mul_ad_affine_product_rhs(354, 667, s.ad_value(361), A::sub(s.ad_value(352), s.ad_value(353)), -1.0, 0.0);

        s.store_add_ad(355, A::mul3_scaled_output(s.ad_value(676), s.ad_value(363), A::add_scaled_product(s.ad_value(127), 1.0, s.ad_value(674), A::sqrt(A::offset(s.ad_value(127), 0.01)), 1.0), -1.0), A::mul3(s.ad_value(681), s.ad_value(365), A::pow(A::offset(s.ad_value(127), 0.01), s.ad_value(682))));

        s.store_mul_ad_product_rhs_mixed_ia(357, 802, 364, A::sqrt(s.ad_value(353)));

        s.store_add_ad_lhs(358, A::add_scaled_inputs4(s.ad_value(354), 1.0, s.ad_value(355), 1.0, s.ad_value(357), 1.0, s.ad_value(231), 1.0), 805);

        s.store_sub(347, 347, 358);

        s.store_div_scaled_product3_indices(184, 416, 163, 158, 1.0, 153, 1.0);

        s.b[1419] = (p.p80 == 0.0);
        s.v[1419] = if s.b[1419] { 1.0 } else { 0.0 };

        if s.b[1419] {
            s.store_pow_ad(171, A::div_scaled_inputs(s.ad_value(163), (2.0 * p.p108), A::mul3_scaled_output(s.ad_value(184), s.ad_value(181), s.ad_value(148), (1.60219e-19 * p.p3)), 1.0), s.ad_value(181));
        }

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

        if s.b[1419] {
            s.store_offset_add(169, 347, 168, p.p23);
        }

        if s.b[1419] {
            s.store_sub_ad_lhs(348, {
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
            s.store_mul_scaled_ad_rhs(168, 181, -1.0, {
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
            });
        }

        if (!s.b[1419]) {
            s.store_sub_ad_lhs(169, A::add_scaled_inputs(A::offset(s.ad_value(168), 0.01), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(168), (-0.01)), ((0.25 * 0.0001) * 0.0001)), 0.5), 375);
            s.store_offset_add(170, 347, 169, p.p23);
        }

        if (!s.b[1419]) {
            s.store_sub_ad_lhs(348, {
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

        s.copy_ad(129, 375);

        s.store_powf_ad(172, A::neg(s.ad_value(897)), 0.666666667);

        s.b[1420] = (p.p61 != 0.0);
        s.v[1420] = if s.b[1420] { 1.0 } else { 0.0 };

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

        if s.b[1420] {
            s.store_mul_div_scaled_inputs_mixed_aii(171, A::sub(A::sqrt(s.ad_value(169)), A::sqrt_scaled_input(s.ad_value(166), 2.0)), 239, -1.0, 181, 2.0);
            s.store_add_scaled_product_value_ad(168, A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(899), 1.0), 1.0, 914, 172, 1.0);
            s.store_add_scaled_inputs3_indices(169, 897, -1.0, 171, (-1.0), 900, 1.0);
        }

        if (!s.b[1420]) {
            s.store_add_scaled_inputs_product_indices(168, 899, 1.0, 897, (-1.0), 914, 172, 1.0);
            s.store_sub(169, 900, 897);
        }

        s.store_div_scaled_inputs2_indices(170, 348, 1.0, 129, (-1.0), 181, 1.0);

        s.store_sub(924, 169, 170);

        s.store_scaled_sub(171, 170, 168, 0.5);

        s.store_limited_exp(901, 171);

        s.b[1421] = (s.v[901] > 1e-7);
        s.v[1421] = if s.b[1421] { 1.0 } else { 0.0 };

        if s.b[1421] {
            s.store_ln_offset_input(176, 901, 1.0);
            s.store_offset_scaled_ad(901, A::sqrt_square_offset(s.ad_value(176), 1.0), (-2.0), 2.0);
            s.store_mul_add_scaled_inputs_rhs(177, 898, s.ad_value(901), p.p1805, s.ad_value(897), 1.0);
            s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);
            s.store_mul(174, 177, 172);
            s.store_ln_neg_add(902, 901, 897);
        }

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

        if s.b[1421] {
            s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));
            s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));
            s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));
            s.store_mul_add_scaled_inputs_rhs(177, 898, s.ad_value(901), p.p1805, s.ad_value(897), 1.0);
            s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);
            s.store_mul(174, 177, 172);
            s.store_ln_neg_add(902, 901, 897);
        }

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

        if s.b[1421] {
            s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));
            s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));
            s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));
        }

        if (!s.b[1421]) {
            s.store_mul_neg_lhs(901, 901, 901);
        }

        s.store_mul_neg_lhs(392, 901, 181);

        s.b[1422] = (p.p57 == 1.0);
        s.v[1422] = if s.b[1422] { 1.0 } else { 0.0 };

        if s.b[1422] {
            s.store_div_scaled_inputs2_indices(1015, 347, 1.0, 129, (-1.0), 181, 1.0);
            s.store_scaled_add_ad_rhs(1016, 1015, A::sqrt(A::add_scaled_square_product(s.ad_value(1015), 1.0, s.ad_value(963), s.ad_value(963), 0.25)), 0.5);
            s.store_pow_ad(1017, s.ad_value(1016), A::scale(s.ad_value(960), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1004, 1010, 1017, A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));
            s.store_div_scaled_inputs3_indices(1018, 347, 1.0, 129, (-1.0), 985, -1.0, 181, 1.0);
            s.store_scaled_add_ad_rhs(1019, 1018, A::sqrt(A::add_scaled_square_product(s.ad_value(1018), 1.0, s.ad_value(964), s.ad_value(964), 0.25)), 0.5);
            s.store_pow_ad(1020, s.ad_value(1019), A::scale(s.ad_value(961), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1005, 1011, 1020, A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));
            s.store_div_scaled_inputs3_indices(1021, 347, 1.0, 129, (-1.0), 986, -1.0, 181, 1.0);
            s.store_scaled_add_ad_rhs(1022, 1021, A::sqrt(A::add_scaled_square_product(s.ad_value(1021), 1.0, s.ad_value(965), s.ad_value(965), 0.25)), 0.5);
            s.store_pow_ad(1023, s.ad_value(1022), A::scale(s.ad_value(962), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1006, 1012, 1023, A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));
            s.store_add_scaled_products_right_right_ad(392, 983, 392, 1.0, 984, A::add_scaled_inputs3(s.ad_value(1004), 1.0, s.ad_value(1005), 1.0, s.ad_value(1006), 1.0), 1.0);
        }

        s.store_div_from_scalar(406, 0.01, 163);

        s.store_add_scaled_product_indices(419, 396, s.v[420], 407, 392, s.v[420]);

        s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(392), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));

        s.store_pow_ad(171, s.ad_value(419), s.ad_value(822));

        s.b[1423] = (p.p61 != 0.0);
        s.v[1423] = if s.b[1423] { 1.0 } else { 0.0 };

        if s.b[1423] {
            s.store_add_scaled_product_mixed_aai(171, A::div(s.ad_value(820), s.ad_value(170)), 1.0, A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(821), s.ad_value(370), 1.0), 171, 1.0);
        }

        if (!s.b[1423]) {
            s.store_add_scaled_product_value_ad(171, A::div(s.ad_value(820), s.ad_value(170)), 1.0, 819, 171, 1.0);
        }

        s.store_offset(397, 171, 1.0);

        s.store_scaled_add_offset_sqrt_square_offset(397, 397, 1.0, (-1.0), ((0.25 * p.p604) * p.p604), 0.5);

        s.store_scale(397, 397, 1.0 / (p.p24));

        s.b[1424] = (p.p64 == 1.0);
        s.v[1424] = if s.b[1424] { 1.0 } else { 0.0 };

        if s.b[1424] {
            s.store_scalar(198, 0.0);
        }

        s.b[1425] = (p.p64 == 0.0);
        s.v[1425] = if s.b[1425] { 1.0 } else { 0.0 };

        if ((!s.b[1424]) && s.b[1425]) {
            s.store_offset_mul(172, 711, 392, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 169, 169, 0.01, 0.5);
            s.store_mul_ad_affine_product_lhs(198, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189), s.v[115], 0.0, 194);
        }

        if ((!s.b[1424]) && (!s.b[1425])) {
            s.store_offset_mul(172, 711, 392, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 169, 169, 0.01, 0.5);
            s.store_mul_ad_lhs(198, A::add_scaled_inputs_product(s.ad_value(190), 1.0, s.ad_value(191), 1.0, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189), s.v[115]), 194);
        }

        s.store_mul_div_scaled_inputs_indices(216, 397, 428, 2.0, 416, 1.0);

        s.store_mul(217, 216, 153);

        s.b[1426] = (p.p80 == 0.0);
        s.v[1426] = if s.b[1426] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1426] {
            s.store_mul_add_scaled_inputs_rhs(175, 659, s.ad_value(392), 1.0, s.ad_value(179), 2.0);
        }

        if (!s.b[1426]) {
            s.store_mul_add_scaled_inputs_rhs(175, 659, s.ad_value(392), 1.0, s.ad_value(182), 2.0);
        }

        s.b[1427] = (s.v[198] > 0.0);
        s.v[1427] = if s.b[1427] { 1.0 } else { 0.0 };

        if s.b[1427] {
            s.store_mul3_lhs(224, 158, 428, 163);
            s.store_mul(168, 224, 198);
            s.store_scale(225, 168, 2.0);
            s.store_add_scaled_inputs_product_indices(226, 175, 1.0, 217, 1.0, 175, 168, 3.0);
            s.store_mul_add_scaled_product_rhs(227, 175, s.ad_value(217), 1.0, s.ad_value(175), s.ad_value(168), 2.0);
            s.store_div_scaled_inputs2(210, A::square(s.ad_value(226)), 1.0, A::add_scaled_square_product(s.ad_value(226), 1.0, s.ad_value(225), s.ad_value(227), (-2.0)), (-1.0), A::mul(A::add(s.ad_value(226), A::sqrt(A::add_scaled_square_product(s.ad_value(226), 1.0, s.ad_value(225), s.ad_value(227), (-2.0)))), s.ad_value(225)), 1.0);
        }

        if (!s.b[1427]) {
            s.store_div_scaled_product_add_scaled_denominator_indices(210, 217, 175, 1.0, 217, 1.0, 175, 1.0, 1.0);
        }

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

        s.store_pow_ad(176, A::offset(A::div(s.ad_value(126), s.ad_value(210)), 1e-6), s.ad_value(423));

        s.store_pow_ad(177, A::offset(s.ad_value(176), 1.0), s.ad_value(212));

        s.store_min_ad(390, A::div(s.ad_value(126), s.ad_value(177)), s.ad_value(126));

        s.store_add(129, 390, 375);

        s.store_powf_ad(170, A::neg(s.ad_value(897)), 0.666666667);

        s.b[1428] = (p.p61 != 0.0);
        s.v[1428] = if s.b[1428] { 1.0 } else { 0.0 };

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

        if s.b[1428] {
            s.store_mul_div_scaled_inputs_mixed_aii(171, A::sub(A::sqrt(s.ad_value(169)), A::sqrt_scaled_input(s.ad_value(166), 2.0)), 239, -1.0, 181, 2.0);
            s.store_add_scaled_product_value_ad(168, A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(899), 1.0), 1.0, 914, 170, 1.0);
            s.store_add_scaled_inputs3_indices(169, 897, -1.0, 171, (-1.0), 900, 1.0);
        }

        if (!s.b[1428]) {
            s.store_add_scaled_inputs_product_indices(168, 899, 1.0, 897, (-1.0), 914, 170, 1.0);
            s.store_sub(169, 900, 897);
        }

        s.store_div_scaled_inputs2_indices(170, 348, 1.0, 129, (-1.0), 181, 1.0);

        s.store_sub(924, 169, 170);

        s.store_scaled_sub(171, 170, 168, 0.5);

        s.store_limited_exp(901, 171);

        s.b[1429] = (s.v[901] > 1e-7);
        s.v[1429] = if s.b[1429] { 1.0 } else { 0.0 };

        if s.b[1429] {
            s.store_ln_offset_input(176, 901, 1.0);
            s.store_offset_scaled_ad(901, A::sqrt_square_offset(s.ad_value(176), 1.0), (-2.0), 2.0);
            s.store_mul_add_scaled_inputs_rhs(177, 898, s.ad_value(901), p.p1805, s.ad_value(897), 1.0);
            s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);
            s.store_mul(174, 177, 172);
            s.store_ln_neg_add(902, 901, 897);
        }

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

        if s.b[1429] {
            s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));
            s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));
            s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));
            s.store_mul_add_scaled_inputs_rhs(177, 898, s.ad_value(901), p.p1805, s.ad_value(897), 1.0);
            s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);
            s.store_mul(174, 177, 172);
            s.store_ln_neg_add(902, 901, 897);
        }

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

        if s.b[1429] {
            s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));
            s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));
            s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));
        }

        if (!s.b[1429]) {
            s.store_mul_neg_lhs(901, 901, 901);
        }

        s.store_mul_neg_lhs(393, 901, 181);

        s.b[1430] = (p.p57 == 1.0);
        s.v[1430] = if s.b[1430] { 1.0 } else { 0.0 };

        if s.b[1430] {
            s.store_div_scaled_inputs2_indices(1015, 347, 1.0, 129, (-1.0), 181, 1.0);
            s.store_scaled_add_ad_rhs(1016, 1015, A::sqrt(A::add_scaled_square_product(s.ad_value(1015), 1.0, s.ad_value(963), s.ad_value(963), 0.25)), 0.5);
            s.store_pow_ad(1017, s.ad_value(1016), A::scale(s.ad_value(960), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1007, 1010, 1017, A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));
            s.store_div_scaled_inputs3_indices(1018, 347, 1.0, 129, (-1.0), 985, -1.0, 181, 1.0);
            s.store_scaled_add_ad_rhs(1019, 1018, A::sqrt(A::add_scaled_square_product(s.ad_value(1018), 1.0, s.ad_value(964), s.ad_value(964), 0.25)), 0.5);
            s.store_pow_ad(1020, s.ad_value(1019), A::scale(s.ad_value(961), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1008, 1011, 1020, A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));
            s.store_div_scaled_inputs3_indices(1021, 347, 1.0, 129, (-1.0), 986, -1.0, 181, 1.0);
            s.store_scaled_add_ad_rhs(1022, 1021, A::sqrt(A::add_scaled_square_product(s.ad_value(1021), 1.0, s.ad_value(965), s.ad_value(965), 0.25)), 0.5);
            s.store_pow_ad(1023, s.ad_value(1022), A::scale(s.ad_value(962), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1009, 1012, 1023, A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));
            s.store_add_scaled_products_right_right_ad(393, 983, 393, 1.0, 984, A::add_scaled_inputs3(s.ad_value(1007), 1.0, s.ad_value(1008), 1.0, s.ad_value(1009), 1.0), 1.0);
        }

        s.b[1431] = (p.p67 == 1.0);
        s.v[1431] = if s.b[1431] { 1.0 } else { 0.0 };

        if s.b[1431] {
            s.store_add_ad(356, A::mul3_scaled_output(s.ad_value(297), s.ad_value(363), A::add_scaled_product(s.ad_value(127), 1.0, s.ad_value(674), A::sqrt(A::offset(s.ad_value(127), 0.01)), 1.0), -1.0), A::mul3(s.ad_value(681), s.ad_value(365), A::pow(A::offset(s.ad_value(127), 0.01), s.ad_value(682))));
            s.store_add_ad_lhs(359, A::add_scaled_inputs4(s.ad_value(354), 1.0, s.ad_value(356), 1.0, s.ad_value(357), 1.0, s.ad_value(231), 1.0), 805);
            s.store_add_scaled_inputs3_indices(349, 125, 1.0, 167, (-1.0), 359, -1.0);
            s.store_div_scaled_product3_indices(185, 414, 163, 158, 1.0, 153, 1.0);
        }

        s.b[1432] = (p.p80 == 0.0);
        s.v[1432] = if s.b[1432] { 1.0 } else { 0.0 };

        if (s.b[1431] && s.b[1432]) {
            s.store_pow_ad(171, A::div_scaled_inputs(s.ad_value(163), (2.0 * p.p108), A::mul3_scaled_output(s.ad_value(185), s.ad_value(181), s.ad_value(148), (1.60219e-19 * p.p3)), 1.0), s.ad_value(181));
        }

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

        if (s.b[1431] && s.b[1432]) {
            s.store_offset_add(169, 349, 168, p.p23);
        }

        if (s.b[1431] && s.b[1432]) {
            s.store_sub_ad_lhs(350, {
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

        if (s.b[1431] && (!s.b[1432])) {
            s.store_mul_scaled_ad_rhs(168, 181, -1.0, {
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
            });
        }

        if (s.b[1431] && (!s.b[1432])) {
            s.store_sub_ad_lhs(169, A::add_scaled_inputs(A::offset(s.ad_value(168), 0.01), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(168), (-0.01)), ((0.25 * 0.0001) * 0.0001)), 0.5), 375);
            s.store_offset_add(170, 349, 169, p.p23);
        }

        if (s.b[1431] && (!s.b[1432])) {
            s.store_sub_ad_lhs(350, {
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

        if s.b[1431] {
            s.copy_ad(130, 375);
            s.store_powf_ad(172, A::neg(s.ad_value(897)), 0.666666667);
        }

        s.b[1433] = (p.p61 != 0.0);
        s.v[1433] = if s.b[1433] { 1.0 } else { 0.0 };

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

        if (s.b[1431] && s.b[1433]) {
            s.store_mul_div_scaled_inputs_mixed_aii(171, A::sub(A::sqrt(s.ad_value(169)), A::sqrt_scaled_input(s.ad_value(166), 2.0)), 239, -1.0, 181, 2.0);
            s.store_add_scaled_product_value_ad(168, A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(899), 1.0), 1.0, 914, 172, 1.0);
            s.store_add_scaled_inputs3_indices(169, 897, -1.0, 171, (-1.0), 900, 1.0);
        }

        if (s.b[1431] && (!s.b[1433])) {
            s.store_add_scaled_inputs_product_indices(168, 899, 1.0, 897, (-1.0), 914, 172, 1.0);
            s.store_sub(169, 900, 897);
        }

        if s.b[1431] {
            s.store_div_scaled_inputs2_indices(170, 350, 1.0, 130, (-1.0), 181, 1.0);
            s.store_sub(924, 169, 170);
            s.store_scaled_sub(171, 170, 168, 0.5);
            s.store_limited_exp(901, 171);
        }

        s.b[1434] = (s.v[901] > 1e-7);
        s.v[1434] = if s.b[1434] { 1.0 } else { 0.0 };

        if (s.b[1431] && s.b[1434]) {
            s.store_ln_offset_input(176, 901, 1.0);
            s.store_offset_scaled_ad(901, A::sqrt_square_offset(s.ad_value(176), 1.0), (-2.0), 2.0);
            s.store_mul_add_scaled_inputs_rhs(177, 898, s.ad_value(901), p.p1805, s.ad_value(897), 1.0);
            s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);
            s.store_mul(174, 177, 172);
            s.store_ln_neg_add(902, 901, 897);
        }

    }

    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
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

        if (s.b[1431] && s.b[1434]) {
            s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));
            s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));
            s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));
            s.store_mul_add_scaled_inputs_rhs(177, 898, s.ad_value(901), p.p1805, s.ad_value(897), 1.0);
            s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);
            s.store_mul(174, 177, 172);
            s.store_ln_neg_add(902, 901, 897);
        }

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

        if (s.b[1431] && s.b[1434]) {
            s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));
            s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));
            s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));
        }

        if (s.b[1431] && (!s.b[1434])) {
            s.store_mul_neg_lhs(901, 901, 901);
        }

        if s.b[1431] {
            s.store_mul_neg_lhs(394, 901, 181);
        }

        s.b[1435] = (p.p57 == 1.0);
        s.v[1435] = if s.b[1435] { 1.0 } else { 0.0 };

        if (s.b[1431] && s.b[1435]) {
            s.store_div_scaled_inputs2_indices(1015, 349, 1.0, 130, (-1.0), 181, 1.0);
            s.store_scaled_add_ad_rhs(1016, 1015, A::sqrt(A::add_scaled_square_product(s.ad_value(1015), 1.0, s.ad_value(963), s.ad_value(963), 0.25)), 0.5);
            s.store_pow_ad(1017, s.ad_value(1016), A::scale(s.ad_value(960), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1004, 1010, 1017, A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));
            s.store_div_scaled_inputs3_indices(1018, 349, 1.0, 130, (-1.0), 985, -1.0, 181, 1.0);
            s.store_scaled_add_ad_rhs(1019, 1018, A::sqrt(A::add_scaled_square_product(s.ad_value(1018), 1.0, s.ad_value(964), s.ad_value(964), 0.25)), 0.5);
            s.store_pow_ad(1020, s.ad_value(1019), A::scale(s.ad_value(961), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1005, 1011, 1020, A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));
            s.store_div_scaled_inputs3_indices(1021, 349, 1.0, 130, (-1.0), 986, -1.0, 181, 1.0);
            s.store_scaled_add_ad_rhs(1022, 1021, A::sqrt(A::add_scaled_square_product(s.ad_value(1021), 1.0, s.ad_value(965), s.ad_value(965), 0.25)), 0.5);
            s.store_pow_ad(1023, s.ad_value(1022), A::scale(s.ad_value(962), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1006, 1012, 1023, A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));
            s.store_add_scaled_products_right_right_ad(394, 983, 394, 1.0, 984, A::add_scaled_inputs3(s.ad_value(1004), 1.0, s.ad_value(1005), 1.0, s.ad_value(1006), 1.0), 1.0);
        }

        if s.b[1431] {
            s.store_add_scaled_product_indices(421, 396, s.v[420], 407, 394, s.v[420]);
            s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(394), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));
            s.store_pow_ad(171, s.ad_value(421), s.ad_value(822));
        }

        s.b[1436] = (p.p61 != 0.0);
        s.v[1436] = if s.b[1436] { 1.0 } else { 0.0 };

        if (s.b[1431] && s.b[1436]) {
            s.store_add_scaled_product_mixed_aai(171, A::div(s.ad_value(319), s.ad_value(170)), 1.0, A::add_scaled_product(s.ad_value(304), 1.0, s.ad_value(315), s.ad_value(370), 1.0), 171, 1.0);
        }

        if (s.b[1431] && (!s.b[1436])) {
            s.store_add_scaled_product_value_ad(171, A::div(s.ad_value(319), s.ad_value(170)), 1.0, 304, 171, 1.0);
        }

        if s.b[1431] {
            s.store_offset(398, 171, 1.0);
            s.store_scaled_add_offset_sqrt_square_offset(398, 398, 1.0, (-1.0), ((0.25 * p.p604) * p.p604), 0.5);
            s.store_scale(398, 398, 1.0 / (p.p24));
        }

        s.b[1437] = (p.p64 == 1.0);
        s.v[1437] = if s.b[1437] { 1.0 } else { 0.0 };

        if (s.b[1431] && s.b[1437]) {
            s.store_scalar(199, 0.0);
        }

        s.b[1438] = (p.p64 == 0.0);
        s.v[1438] = if s.b[1438] { 1.0 } else { 0.0 };

        if ((s.b[1431] && (!s.b[1437])) && s.b[1438]) {
            s.store_offset_mul(172, 711, 394, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 169, 169, 0.01, 0.5);
            s.store_mul_ad_affine_product_lhs(199, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189), s.v[115], 0.0, 194);
        }

        if ((s.b[1431] && (!s.b[1437])) && (!s.b[1438])) {
            s.store_offset_mul(172, 711, 394, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 169, 169, 0.01, 0.5);
            s.store_mul_ad_lhs(199, A::add_scaled_inputs_product(s.ad_value(190), 1.0, s.ad_value(191), 1.0, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189), s.v[115]), 194);
        }

        if s.b[1431] {
            s.store_mul_div_scaled_inputs_indices(222, 398, 336, 2.0, 414, 1.0);
            s.store_mul(223, 222, 153);
        }

        s.b[1439] = (p.p80 == 0.0);
        s.v[1439] = if s.b[1439] { 1.0 } else { 0.0 };

        if (s.b[1431] && s.b[1439]) {
            s.store_mul_add_scaled_inputs_rhs(175, 659, s.ad_value(394), 1.0, s.ad_value(179), 2.0);
        }

        if (s.b[1431] && (!s.b[1439])) {
            s.store_mul_add_scaled_inputs_rhs(175, 659, s.ad_value(394), 1.0, s.ad_value(182), 2.0);
        }

        s.b[1440] = (s.v[199] > 0.0);
        s.v[1440] = if s.b[1440] { 1.0 } else { 0.0 };

        if (s.b[1431] && s.b[1440]) {
            s.store_mul_product3_indices(168, 199, 158, 336, 163, 1.0);
            s.store_scale(225, 168, 2.0);
            s.store_add_scaled_inputs_product_indices(226, 175, 1.0, 223, 1.0, 175, 168, 3.0);
            s.store_mul_add_scaled_product_rhs(227, 175, s.ad_value(223), 1.0, s.ad_value(175), s.ad_value(168), 2.0);
            s.store_div_scaled_inputs2(211, A::square(s.ad_value(226)), 1.0, A::add_scaled_square_product(s.ad_value(226), 1.0, s.ad_value(225), s.ad_value(227), (-2.0)), (-1.0), A::mul(A::add(s.ad_value(226), A::sqrt(A::add_scaled_square_product(s.ad_value(226), 1.0, s.ad_value(225), s.ad_value(227), (-2.0)))), s.ad_value(225)), 1.0);
        }

        if (s.b[1431] && (!s.b[1440])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(211, 223, 175, 1.0, 223, 1.0, 175, 1.0, 1.0);
        }

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

        if s.b[1431] {
            s.store_pow_ad(176, A::offset(A::div(s.ad_value(126), s.ad_value(211)), 1e-6), s.ad_value(423));
            s.store_pow_ad(177, A::offset(s.ad_value(176), 1.0), s.ad_value(212));
            s.store_min_ad(391, A::div(s.ad_value(126), s.ad_value(177)), s.ad_value(126));
            s.store_add(130, 391, 375);
            s.store_powf_ad(170, A::neg(s.ad_value(897)), 0.666666667);
        }

        s.b[1441] = (p.p61 != 0.0);
        s.v[1441] = if s.b[1441] { 1.0 } else { 0.0 };

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

        if (s.b[1431] && s.b[1441]) {
            s.store_mul_div_scaled_inputs_mixed_aii(171, A::sub(A::sqrt(s.ad_value(169)), A::sqrt_scaled_input(s.ad_value(166), 2.0)), 239, -1.0, 181, 2.0);
            s.store_add_scaled_product_value_ad(168, A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(899), 1.0), 1.0, 914, 170, 1.0);
            s.store_add_scaled_inputs3_indices(169, 897, -1.0, 171, (-1.0), 900, 1.0);
        }

        if (s.b[1431] && (!s.b[1441])) {
            s.store_add_scaled_inputs_product_indices(168, 899, 1.0, 897, (-1.0), 914, 170, 1.0);
            s.store_sub(169, 900, 897);
        }

        if s.b[1431] {
            s.store_div_scaled_inputs2_indices(170, 350, 1.0, 130, (-1.0), 181, 1.0);
            s.store_sub(924, 169, 170);
            s.store_scaled_sub(171, 170, 168, 0.5);
            s.store_limited_exp(901, 171);
        }

        s.b[1442] = (s.v[901] > 1e-7);
        s.v[1442] = if s.b[1442] { 1.0 } else { 0.0 };

        if (s.b[1431] && s.b[1442]) {
            s.store_ln_offset_input(176, 901, 1.0);
            s.store_offset_scaled_ad(901, A::sqrt_square_offset(s.ad_value(176), 1.0), (-2.0), 2.0);
            s.store_mul_add_scaled_inputs_rhs(177, 898, s.ad_value(901), p.p1805, s.ad_value(897), 1.0);
            s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);
            s.store_mul(174, 177, 172);
            s.store_ln_neg_add(902, 901, 897);
        }

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

        if (s.b[1431] && s.b[1442]) {
            s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));
            s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));
            s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));
            s.store_mul_add_scaled_inputs_rhs(177, 898, s.ad_value(901), p.p1805, s.ad_value(897), 1.0);
            s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);
            s.store_mul(174, 177, 172);
            s.store_ln_neg_add(902, 901, 897);
        }

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

        if (s.b[1431] && s.b[1442]) {
            s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));
            s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));
            s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));
        }

        if (s.b[1431] && (!s.b[1442])) {
            s.store_mul_neg_lhs(901, 901, 901);
        }

        if s.b[1431] {
            s.store_mul_neg_lhs(395, 901, 181);
        }

        s.b[1443] = (p.p57 == 1.0);
        s.v[1443] = if s.b[1443] { 1.0 } else { 0.0 };

        if (s.b[1431] && s.b[1443]) {
            s.store_div_scaled_inputs2_indices(1015, 349, 1.0, 130, (-1.0), 181, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1431] && s.b[1443]) {
            s.store_scaled_add_ad_rhs(1016, 1015, A::sqrt(A::add_scaled_square_product(s.ad_value(1015), 1.0, s.ad_value(963), s.ad_value(963), 0.25)), 0.5);
            s.store_pow_ad(1017, s.ad_value(1016), A::scale(s.ad_value(960), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1007, 1010, 1017, A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));
            s.store_div_scaled_inputs3_indices(1018, 349, 1.0, 130, (-1.0), 985, -1.0, 181, 1.0);
            s.store_scaled_add_ad_rhs(1019, 1018, A::sqrt(A::add_scaled_square_product(s.ad_value(1018), 1.0, s.ad_value(964), s.ad_value(964), 0.25)), 0.5);
            s.store_pow_ad(1020, s.ad_value(1019), A::scale(s.ad_value(961), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1008, 1011, 1020, A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));
            s.store_div_scaled_inputs3_indices(1021, 349, 1.0, 130, (-1.0), 986, -1.0, 181, 1.0);
            s.store_scaled_add_ad_rhs(1022, 1021, A::sqrt(A::add_scaled_square_product(s.ad_value(1021), 1.0, s.ad_value(965), s.ad_value(965), 0.25)), 0.5);
            s.store_pow_ad(1023, s.ad_value(1022), A::scale(s.ad_value(962), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1009, 1012, 1023, A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));
            s.store_add_scaled_products_right_right_ad(395, 983, 395, 1.0, 984, A::add_scaled_inputs3(s.ad_value(1007), 1.0, s.ad_value(1008), 1.0, s.ad_value(1009), 1.0), 1.0);
        }

        if s.b[1431] {
            s.store_scaled_add(403, 394, 395, 0.5);
            s.store_sub(405, 394, 395);
            s.store_scaled_square(168, 391, 1600.0);
        }

        s.b[1444] = (p.p603 != 0.0);
        s.v[1444] = if s.b[1444] { 1.0 } else { 0.0 };

        if (s.b[1431] && s.b[1444]) {
            s.store_add_scaled_inputs3_mixed_iia(404, 394, 0.5, 395, 0.5, A::mul_sub_from_scalar_lhs_scaled_output(1.0, A::limited_exp_scaled_input(s.ad_value(168), -1.0), s.ad_value(405), (p.p603 * 0.5)), 1.0);
        }

        if (s.b[1431] && (!s.b[1444])) {
            s.store_scaled_add(404, 394, 395, 0.5);
        }

        s.b[1445] = (p.p61 != 0.0);
        s.v[1445] = if s.b[1445] { 1.0 } else { 0.0 };

        if s.b[1445] {
            s.store_mul_div_scaled_inputs_mixed_aii(178, A::sqrt(s.ad_value(179)), 239, 1.0, 181, 2.0);
            s.store_scale(168, 178, 0.5);
        }

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

        s.b[1446] = ((s.v[170] * s.v[179]) > (s.v[166] + (s.v[178] * (((s.v[166] * s.v[179])) as f64).sqrt())));
        s.v[1446] = if s.b[1446] { 1.0 } else { 0.0 };

        if (s.b[1445] && s.b[1446]) {
            s.store_sub_ad_lhs(169, A::sqrt(A::add(A::offset(s.ad_value(170), (-1.0)), A::square(s.ad_value(168)))), 168);
            s.store_offset_square(340, 169, 1.0);
        }

        if (s.b[1445] && s.b[1446]) {
            if (!((((-s.v[340])) as f64).abs() < 1e-7)) {
                s.store_offset_ad(175, A::limited_exp_scaled_input(s.ad_value(340), -1.0), (-1.0));
            } else {
                if ((((-s.v[340])) as f64).abs() < 1e-7) {
                    s.store_sub_ad_lhs(175, A::mul_scaled_lhs(s.ad_value(340), (-(-0.5)), s.ad_value(340)), 340);
                } else {
                    s.store_scalar(175, 0.0);
                }
            }
        }

        if (s.b[1445] && (!s.b[1446])) {
            s.store_sub_scaled_ad_rhs(171, 170, 0.5, A::scale_offset(s.ad_value(178), ((1.0 / (((2.0) as f64).sqrt())) * (3.0)), 3.0));
            s.store_add_ad_rhs(340, 171, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(171)), 1.0, s.ad_value(170), 6.0)));
        }

        s.b[1447] = (s.v[170] < 0.0);
        s.v[1447] = if s.b[1447] { 1.0 } else { 0.0 };

        if ((s.b[1445] && (!s.b[1446])) && s.b[1447]) {
            s.store_div_scaled_inputs2_indices(172, 170, 1.0, 340, (-1.0), 178, 1.0);
            s.store_sub_ad_lhs(175, A::square(s.ad_value(172)), 340);
        }

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

        if ((s.b[1445] && (!s.b[1446])) && (!s.b[1447])) {
            s.store_limited_exp_neg_input(341, 340);
            s.store_sub_ad_lhs(172, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(170), 1.0, s.ad_value(341), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);
            s.store_add_ad(340, A::sub_from_scalar(1.0, s.ad_value(341)), A::square(s.ad_value(172)));
        }

        if ((s.b[1445] && (!s.b[1446])) && (!s.b[1447])) {
            if (!((((-s.v[340])) as f64).abs() < 1e-7)) {
                s.store_offset_ad(175, A::limited_exp_scaled_input(s.ad_value(340), -1.0), (-1.0));
            } else {
                if ((((-s.v[340])) as f64).abs() < 1e-7) {
                    s.store_sub_ad_lhs(175, A::mul_scaled_lhs(s.ad_value(340), (-(-0.5)), s.ad_value(340)), 340);
                } else {
                    s.store_scalar(175, 0.0);
                }
            }
        }

        if s.b[1445] {
            s.store_sqrt_add(176, 175, 340);
        }

        s.b[1448] = (s.v[340] > 1e-15);
        s.v[1448] = if s.b[1448] { 1.0 } else { 0.0 };

        if (s.b[1445] && s.b[1448]) {
            s.store_add_scaled_inputs_product_indices(344, 170, -1.0, 340, 1.0, 178, 176, 1.0);
            s.store_sub_from_scalar_ad(345, 1.0, A::div_scaled_product(s.ad_value(178), s.ad_value(175), 0.5, s.ad_value(176), 1.0));
            s.store_sub_div_rhs_indices(177, 340, 344, 345);
        }

        if (s.b[1445] && s.b[1448]) {
            if (!((((-s.v[177])) as f64).abs() < 1e-7)) {
                s.store_offset_ad(341, A::limited_exp_scaled_input(s.ad_value(177), -1.0), (-1.0));
            } else {
                if ((((-s.v[177])) as f64).abs() < 1e-7) {
                    s.store_sub_ad_lhs(341, A::mul_scaled_lhs(s.ad_value(177), (-(-0.5)), s.ad_value(177)), 177);
                } else {
                    s.store_scalar(341, 0.0);
                }
            }
        }

        if (s.b[1445] && s.b[1448]) {
            s.store_sqrt_add(342, 341, 177);
            s.store_mul3_affine_lhs(401, 178, 342, -1.0, 0.0, 179);
        }

        s.b[1449] = (s.v[340] < (-1e-15));
        s.v[1449] = if s.b[1449] { 1.0 } else { 0.0 };

        if ((s.b[1445] && (!s.b[1448])) && s.b[1449]) {
            s.store_add_scaled_inputs_product_indices(344, 170, -1.0, 340, 1.0, 178, 176, (-1.0));
            s.store_offset_div_scaled_product(345, s.ad_value(178), s.ad_value(175), 0.5, s.ad_value(176), 1.0, 1.0);
            s.store_sub_div_rhs_indices(177, 340, 344, 345);
        }

        if ((s.b[1445] && (!s.b[1448])) && s.b[1449]) {
            s.store_add_ad_lhs(343, {
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

        if ((s.b[1445] && (!s.b[1448])) && s.b[1449]) {
            s.store_mul_sqrt_rhs(342, 178, 343);
        }

        if ((s.b[1445] && (!s.b[1448])) && (!s.b[1449])) {
            s.store_scalar(177, 0.0);
            s.store_scalar(342, 0.0);
        }

        if (s.b[1445] && (!s.b[1448])) {
            s.store_mul(401, 342, 179);
        }

        if s.b[1445] {
            s.store_mul_ad_product_lhs_mixed_ia(904, 178, A::limited_exp_scaled_input(s.ad_value(177), (-1.0 / (2.0))), 179);
            s.store_scaled_add_offset_sqrt_square_offset(921, 177, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);
            s.store_sqrt(922, 921);
            s.store_offset_div(923, 178, 922, 1.0);
        }

        s.store_scaled_add(399, 392, 393, 0.5);

        s.store_sub(402, 392, 393);

        s.store_scaled_square(168, 390, 1600.0);

        s.b[1450] = (p.p603 != 0.0);
        s.v[1450] = if s.b[1450] { 1.0 } else { 0.0 };

        if s.b[1450] {
            s.store_add_scaled_inputs3_mixed_iia(400, 392, 0.5, 393, 0.5, A::mul_sub_from_scalar_lhs_scaled_output(1.0, A::limited_exp_scaled_input(s.ad_value(168), -1.0), s.ad_value(402), (p.p603 * 0.5)), 1.0);
        }

        if (!s.b[1450]) {
            s.store_scaled_add(400, 392, 393, 0.5);
        }

        s.b[1451] = (s.v[655] > 0.0);
        s.v[1451] = if s.b[1451] { 1.0 } else { 0.0 };

        if s.b[1451] {
            s.store_scale(172, 399, 1.0 / (p.p400));
            s.store_offset_pow_ad(174, s.ad_value(172), s.ad_value(661), 1.0);
            s.store_div(374, 373, 174);
            s.store_div_from_scalar_ad(372, 1.0, A::add_scaled_product(A::div_from_scalar(1.0, A::scale(s.ad_value(163), (p.p89 * 1.0 / (p.p90)))), 1.0, s.ad_value(374), s.ad_value(655), 1.0 / (s.v[143])));
        }

        if (!s.b[1451]) {
            s.copy_ad(372, 163);
        }

        s.b[1452] = ((p.p61 != 0.0) && (s.v[656] != 0.0));
        s.v[1452] = if s.b[1452] { 1.0 } else { 0.0 };

        if s.b[1452] {
            s.store_offset_powf_ad(175, A::scale(s.ad_value(904), 1.0 / (p.p401)), p.p402, 1.0);
            s.store_div(374, 373, 175);
            s.store_div_from_scalar_ad(494, 1.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(494)), 1.0, s.ad_value(374), s.ad_value(656), 1.0 / (s.v[143])));
        }

        s.store_div_scaled_product3_indices(183, 416, 163, 158, 1.0, 153, 1.0);

        s.store_add_scaled_product_indices(409, 396, s.v[420], 407, 400, s.v[420]);

        s.b[1453] = (p.p80 == 0.0);
        s.v[1453] = if s.b[1453] { 1.0 } else { 0.0 };

        if s.b[1453] {
            s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(400), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));
        }

        if (!s.b[1453]) {
            s.store_scaled_square(168, 390, 1600.0);
            s.store_sub_from_scalar_ad(169, 1.0, A::limited_exp_scaled_input(s.ad_value(168), -1.0));
            s.store_mul_ad_lhs(168, A::add_scaled_products(s.ad_value(330), s.ad_value(392), 1.0, s.ad_value(331), s.ad_value(393), 1.0), 169);
        }

        if (!s.b[1453]) {
            if (!(s.v[168] < ((-10000.0) * 1e-12))) {
                s.store_scaled_add_sqrt_square_offset_rhs(169, 168, 168, ((4.0 * 1e-12) * 1e-12), 0.5);
            } else {
                if (s.v[168] < ((-10000.0) * 1e-12)) {
                    s.store_div_from_scalar(169, ((-1e-12) * 1e-12), 168);
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }

        if (!s.b[1453]) {
            s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(169), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));
        }

        s.store_pow_ad(168, s.ad_value(409), s.ad_value(822));

        s.b[1454] = (p.p61 != 0.0);
        s.v[1454] = if s.b[1454] { 1.0 } else { 0.0 };

        if s.b[1454] {
            s.store_add_scaled_product_mixed_aai(171, A::div(s.ad_value(820), s.ad_value(170)), 1.0, A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(821), s.ad_value(370), 1.0), 168, 1.0);
        }

        if (!s.b[1454]) {
            s.store_add_scaled_product_value_ad(171, A::div(s.ad_value(820), s.ad_value(170)), 1.0, 819, 168, 1.0);
        }

        s.store_offset(411, 171, 1.0);

        s.store_scaled_add_offset_sqrt_square_offset(411, 411, 1.0, (-1.0), ((0.25 * p.p604) * p.p604), 0.5);

        s.store_offset_scaled_ad(215, A::scale(A::limited_exp_scaled_input(s.ad_value(390), (-p.p888)), p.p887), (-p.p24), p.p24);

        s.store_div(411, 411, 215);

        s.store_div(415, 416, 411);

        s.b[1455] = (p.p67 == 1.0);
        s.v[1455] = if s.b[1455] { 1.0 } else { 0.0 };

        s.b[1456] = (p.p80 == 0.0);
        s.v[1456] = if s.b[1456] { 1.0 } else { 0.0 };

        if (s.b[1455] && s.b[1456]) {
            s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(404), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));
        }

        if (s.b[1455] && (!s.b[1456])) {
            s.store_add_scaled_products_indices(168, 330, 394, 1.0, 331, 395, 1.0);
        }

        if (s.b[1455] && (!s.b[1456])) {
            if (!(s.v[168] < ((-10000.0) * 1e-12))) {
                s.store_scaled_add_sqrt_square_offset_rhs(169, 168, 168, ((4.0 * 1e-12) * 1e-12), 0.5);
            } else {
                if (s.v[168] < ((-10000.0) * 1e-12)) {
                    s.store_div_from_scalar(169, ((-1e-12) * 1e-12), 168);
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }

        if (s.b[1455] && (!s.b[1456])) {
            s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(169), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));
        }

        if s.b[1455] {
            s.store_add_scaled_product_indices(410, 396, s.v[420], 408, 404, s.v[420]);
            s.store_add_scaled_product_mixed_aia(171, A::div(s.ad_value(319), s.ad_value(170)), 1.0, 304, A::pow(s.ad_value(410), s.ad_value(822)), 1.0);
        }

        if (!s.b[1455]) {
            s.store_add_scaled_product_indices(410, 396, s.v[420], 408, 400, s.v[420]);
            s.store_add_scaled_product_mixed_aia(171, A::div(s.ad_value(820), s.ad_value(170)), 1.0, 819, A::pow(s.ad_value(410), s.ad_value(822)), 1.0);
        }

        s.store_offset(412, 171, 1.0);

        s.store_scaled_add_offset_sqrt_square_offset(412, 412, 1.0, (-1.0), ((0.25 * p.p604) * p.p604), 0.5);

        s.store_div(412, 412, 215);

        s.store_offset_div_scaled_product(360, s.ad_value(719), s.ad_value(153), 1.0, s.ad_value(351), 1.0, 1e-6);

        s.b[1457] = (s.v[360] < 40.0);
        s.v[1457] = if s.b[1457] { 1.0 } else { 0.0 };

        if s.b[1457] {
            s.store_add_ad_lhs(200, A::div_scaled_value_offset_denominator(s.ad_value(427), 0.5, A::cosh(s.ad_value(360)), (-1.0), 1.0), 718);
        }

        if (!s.b[1457]) {
            s.store_add_scaled_product_right_ad(200, 718, 1.0, 427, A::limited_exp_scaled_input(s.ad_value(360), -1.0), 1.0);
        }

        s.b[1458] = (s.v[720] > 0.0);
        s.v[1458] = if s.b[1458] { 1.0 } else { 0.0 };

        if s.b[1458] {
            s.store_offset_div_scaled_product(201, s.ad_value(720), s.ad_value(399), 1.0, s.ad_value(217), 1.0, 1.0);
        }

        if (!s.b[1458]) {
            s.store_div_from_scalar_sub_from_scalar_ad(201, 1.0, 1.0, A::div_scaled_product(s.ad_value(720), s.ad_value(399), 1.0, s.ad_value(217), 1.0));
        }

        s.store_sub(202, 126, 390);

        s.b[1459] = (p.p80 == 0.0);
        s.v[1459] = if s.b[1459] { 1.0 } else { 0.0 };

        if s.b[1459] {
            s.store_add_scaled_inputs(204, 399, 1.0, 179, 2.0);
        }

        if (!s.b[1459]) {
            s.store_add_scaled_inputs(204, 399, 1.0, 182, 2.0);
        }

        s.b[1460] = (s.v[200] > 0.0);
        s.v[1460] = if s.b[1460] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_20(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1460] {
            s.copy_ad(169, 204);
            s.store_div_add_scaled_inputs_rhs_indices(171, 169, 210, 1.0, 169, 1.0);
            s.store_mul_ad_product_lhs_mixed_ai(203, A::div(s.ad_value(169), s.ad_value(200)), 171, 201);
            s.store_offset_div(205, 202, 203, 1.0);
        }

        if (!s.b[1460]) {
            s.store_scalar(205, 1.0);
        }

        s.b[1461] = (s.v[795] > 0.0);
        s.v[1461] = if s.b[1461] { 1.0 } else { 0.0 };

        s.b[1462] = (s.v[793] < 0.0);
        s.v[1462] = if s.b[1462] { 1.0 } else { 0.0 };

        if (s.b[1461] && s.b[1462]) {
            s.store_div_from_scalar_ad(169, 1.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(795)), 1.0, s.ad_value(793), s.ad_value(399), (-1.0)));
        }

        if (s.b[1461] && (!s.b[1462])) {
            s.store_add_scaled_product_indices(169, 795, 1.0, 793, 399, 1.0);
        }

        if s.b[1461] {
            s.store_offset_mul_ad(206, s.ad_value(169), {
                if (!((1.0 + (((s.v[126] - s.v[390]) / s.v[169]) / (s.v[210] + s.v[217]))) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((1.0 + (((s.v[126] - s.v[390]) / s.v[169]) / (s.v[210] + s.v[217]))) > 1e-38) {
                            A::ln(A::offset(A::div_scaled_inputs2_by_product(s.ad_value(126), 1.0, s.ad_value(390), (-1.0), s.ad_value(169), A::add(s.ad_value(210), s.ad_value(217)), 1.0), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0);
        }

        if (!s.b[1461]) {
            s.store_scalar(206, 1.0);
        }

        s.store_mul(205, 205, 206);

        s.store_div_scaled_inputs_indices(218, 422, 2.0, 415, 1.0);

        s.store_mul(219, 218, 153);

        s.store_limited_exp_ad(168, A::mul(s.ad_value(695), {
            if (!((s.v[402] / s.v[219]) > 1e-38)) {
                A::neg(A::constant(87.498233534))
            } else {
                {
                    if ((s.v[402] / s.v[219]) > 1e-38) {
                        A::ln(A::div(s.ad_value(402), s.ad_value(219)))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }));

        s.store_div_from_scalar(169, 1.0, 695);

        s.store_offset_limited_exp_ad(225, A::mul(s.ad_value(169), {
            if (!(s.v[694] > 1e-38)) {
                A::neg(A::constant(87.498233534))
            } else {
                {
                    if (s.v[694] > 1e-38) {
                        A::ln(s.ad_value(694))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }), 1.0);

        s.store_div_scaled_offset_numerator(209, A::limited_exp(A::mul(s.ad_value(169), {
            if (!((s.v[694] + s.v[168]) > 1e-38)) {
                A::neg(A::constant(87.498233534))
            } else {
                {
                    if ((s.v[694] + s.v[168]) > 1e-38) {
                        A::ln(A::add(s.ad_value(694), s.ad_value(168)))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        })), 1.0, 1.0, s.ad_value(225), 1.0);

        s.store_add_scaled_product_left_ad(209, 209, 1.0, A::mul3_scaled_output(s.ad_value(424), s.ad_value(399), s.ad_value(402), 0.5), 402, 1.0);

        s.store_add_div_rhs_mixed_ia(168, 241, 242, A::add_scaled_inputs(s.ad_value(399), 1.0, s.ad_value(181), 2.0));

        s.store_mul3_lhs(169, 168, 402, 402);

        s.store_offset(170, 169, ((1.0) + ((-0.001))));

        s.store_offset_add_scaled_inputs_mixed_ia(171, 170, 0.5, A::sqrt_square_offset(s.ad_value(170), 0.004), 0.5, (-1.0));

        s.store_scaled_offset_ad(214, A::sqrt(A::offset(s.ad_value(171), 1.0)), 1.0, 0.5);

        s.store_mul(209, 209, 214);

        s.store_scaled_add_offset_sqrt_square_offset(209, 209, 1.0, (-1.0), ((0.25 * p.p453) * p.p453), 0.5);

        s.store_div_ad_rhs(169, 236, A::add_scaled_product(s.ad_value(181), 2.0, A::max_from_scalar(0.0, A::add(s.ad_value(237), A::mul3(s.ad_value(294), s.ad_value(402), s.ad_value(402)))), s.ad_value(399), 1.0));

        s.store_limited_exp_neg_input(366, 169);

        s.b[1463] = (p.p61 == 2.0);
        s.v[1463] = if s.b[1463] { 1.0 } else { 0.0 };

        if s.b[1463] {
            if (!((s.v[293] + (s.v[240] * s.v[127])) < ((-10000.0) * 1e-6))) {
                s.store_scaled_add_sqrt_square_offset_ad(168, A::add_scaled_product(s.ad_value(293), 1.0, s.ad_value(240), s.ad_value(127), 1.0), ((4.0 * 1e-6) * 1e-6), 0.5);
            } else {
                if ((s.v[293] + (s.v[240] * s.v[127])) < ((-10000.0) * 1e-6)) {
                    s.store_div_from_scalar_ad(168, ((-1e-6) * 1e-6), A::add_scaled_product(s.ad_value(293), 1.0, s.ad_value(240), s.ad_value(127), 1.0));
                } else {
                    s.store_scalar(168, 0.0);
                }
            }
        }

        if s.b[1463] {
            s.store_div_ad_rhs(169, 168, A::add_scaled_product(s.ad_value(181), 2.0, A::max_from_scalar(0.0, A::add(s.ad_value(238), A::mul3(s.ad_value(295), s.ad_value(402), s.ad_value(402)))), s.ad_value(399), 1.0));
            s.store_sub_ad(171, A::sqrt(A::sub(s.ad_value(689), s.ad_value(370))), A::sqrt(s.ad_value(689)));
            s.store_limited_exp_ad(371, A::mul_scaled_lhs(s.ad_value(169), -1.0, s.ad_value(171)));
        }

        if (!s.b[1463]) {
            s.store_scalar(371, 1.0);
        }

        s.b[1464] = (p.p67 == 1.0);
        s.v[1464] = if s.b[1464] { 1.0 } else { 0.0 };

        if s.b[1464] {
            s.store_div_scaled_product_indices(220, 336, 412, 2.0, 414, 1.0);
        }

        if (!s.b[1464]) {
            s.store_div_scaled_product_indices(220, 336, 412, 2.0, 416, 1.0);
        }

        s.store_mul(221, 220, 156);

        s.b[1465] = (p.p67 == 1.0);
        s.v[1465] = if s.b[1465] { 1.0 } else { 0.0 };

        if s.b[1465] {
            s.store_pow_ad(168, A::div(s.ad_value(405), s.ad_value(221)), s.ad_value(697));
        }

        if (!s.b[1465]) {
            s.store_pow_ad(168, A::div(s.ad_value(402), s.ad_value(221)), s.ad_value(697));
        }

        s.store_div_from_scalar(169, 1.0, 697);

        s.store_offset_pow_ad(225, s.ad_value(696), s.ad_value(169), 1.0);

        s.store_div_scaled_offset_numerator(213, A::pow(A::add(s.ad_value(696), s.ad_value(168)), s.ad_value(169)), 1.0, 1.0, s.ad_value(225), 1.0);

        s.store_scaled_add_offset_sqrt_square_offset(881, 881, 0.1, (-0.1), ((0.25 * 0.001) * 0.001), 0.5);

        s.store_mul(213, 213, 881);

        s.b[1466] = (s.v[794] != 0.0);
        s.v[1466] = if s.b[1466] { 1.0 } else { 0.0 };

        if s.b[1466] {
            s.store_offset_mul_ad(207, s.ad_value(794), {
                if (!((1.0 + (((s.v[126] - s.v[390]) / s.v[794]) / (s.v[210] + s.v[221]))) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((1.0 + (((s.v[126] - s.v[390]) / s.v[794]) / (s.v[210] + s.v[221]))) > 1e-38) {
                            A::ln(A::offset(A::div_scaled_inputs2_by_product(s.ad_value(126), 1.0, s.ad_value(390), (-1.0), s.ad_value(794), A::add(s.ad_value(210), s.ad_value(221)), 1.0), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0);
        }

        if (!s.b[1466]) {
            s.store_scalar(207, 1.0);
        }

        s.store_mul3_affine_lhs(140, 640, 894, (-1.60219e-19), 0.0, 156);

        s.store_div_add_scaled_inputs_rhs_indices(131, 339, 339, 1.0, 399, 1.0);

        s.store_add_ad_rhs(123, 399, A::mul_sub_from_scalar_lhs(2.0, s.ad_value(131), s.ad_value(181)));

        s.store_mul(122, 123, 402);

        s.b[1467] = (p.p64 == 0.0);
        s.v[1467] = if s.b[1467] { 1.0 } else { 0.0 };

        s.b[1468] = (p.p64 == 1.0);
        s.v[1468] = if s.b[1468] { 1.0 } else { 0.0 };

        s.b[1469] = (p.p64 == 2.0);
        s.v[1469] = if s.b[1469] { 1.0 } else { 0.0 };

        if s.b[1467] {
            s.copy_ad(193, 190);
            s.copy_ad(192, 191);
            s.store_offset_mul(172, 711, 399, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 169, 169, 0.01, 0.5);
            s.store_mul_ad_product_lhs_mixed_ia(197, 194, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), 189);
            s.store_offset_mul_ad(188, A::div_scaled_product_by_product(s.ad_value(183), s.ad_value(123), s.v[115], s.ad_value(411), s.ad_value(209), 1.0), s.ad_value(197), 1.0);
        }

        if (s.b[1468] && (!s.b[1467])) {
            s.store_scalar(197, 0.0);
            s.store_scalar(188, 1.0);
            s.store_add_scaled_product_right_ad(170, 479, (-1.0), 114, A::voltage(ctx, nodes, Some(11), Some(8)), 1.0);
            s.store_sqrt_square_offset(171, 170, 0.1);
            s.store_scaled_add(482, 170, 171, 0.5);
            s.store_offset_mul(172, 711, 482, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 169, 169, 0.01, 0.5);
            s.store_mul_offset_ad_rhs(174, 853, A::mul(s.ad_value(425), A::powf(A::offset(A::square(A::voltage(ctx, nodes, Some(2), Some(8))), 1e-6), (0.5 * p.p921))), 1.0);
            s.store_mul_ad_rhs(193, 194, A::add_scaled_offset_product_lhs(s.ad_value(190), 1.0, A::mul(s.ad_value(174), s.ad_value(168)), p.p911, s.ad_value(189), 1.0));
            s.store_add_scaled_product_right_ad(170, 479, (-1.0), 114, A::voltage(ctx, nodes, Some(11), Some(9)), 1.0);
            s.store_sqrt_square_offset(171, 170, 0.1);
            s.store_scaled_add(483, 170, 171, 0.5);
            s.store_offset_mul(172, 712, 483, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 169, 169, 0.01, 0.5);
            s.store_mul_offset_ad_rhs(174, 852, A::mul(s.ad_value(426), A::powf(A::offset(A::square(A::voltage(ctx, nodes, Some(0), Some(9))), 1e-6), (0.5 * p.p922))), 1.0);
            s.store_mul_ad_rhs(192, 194, A::add_scaled_offset_product_lhs(s.ad_value(191), 1.0, A::mul(s.ad_value(174), s.ad_value(168)), p.p914, s.ad_value(189), 1.0));
        }

        if (s.b[1469] && (!(s.b[1467] || s.b[1468]))) {
            s.store_offset_mul(172, 711, 399, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 169, 169, 0.01, 0.5);
            s.store_mul_add_ad_rhs(197, 194, A::add_scaled_offset_product_lhs(s.ad_value(190), 1.0, A::mul(s.ad_value(709), s.ad_value(168)), p.p908, s.ad_value(189), 1.0), s.ad_value(191));
            s.store_offset_mul_ad(188, A::div_scaled_product_by_product(s.ad_value(183), s.ad_value(123), s.v[115], s.ad_value(411), s.ad_value(209), 1.0), s.ad_value(197), 1.0);
            s.store_scalar(193, 0.0);
            s.store_scalar(192, 0.0);
        }

        s.store_div_scaled_product3_mixed_aiia(124, A::mul3_scaled_output(s.ad_value(183), s.ad_value(122), s.ad_value(205), s.v[115]), 366, 371, 1.0, A::mul3(s.ad_value(411), s.ad_value(209), s.ad_value(188)), 1.0);

        s.store_scale(124, 124, p.p25);

        s.b[1470] = (p.p67 == 1.0);
        s.v[1470] = if s.b[1470] { 1.0 } else { 0.0 };

        if s.b[1470] {
            s.store_div_scaled_inputs2_indices(341, 403, 2.0, 181, 1.0, 213, 1.0);
            s.store_add_ad_rhs(138, 403, A::div_scaled_product(s.ad_value(405), s.ad_value(405), 1.0, s.ad_value(341), 6.0));
            s.store_scaled_sub_ad_rhs(137, 403, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(405), 1.0, A::mul_offset_rhs(A::div(s.ad_value(405), s.ad_value(341)), A::div_scaled_inputs(s.ad_value(405), 1.0, s.ad_value(341), 5.0), 1.0), 1.0 / (6.0)), (-0.5));
        }

        if (!s.b[1470]) {
            s.store_div_scaled_inputs2_indices(341, 399, 2.0, 181, 1.0, 213, 1.0);
            s.store_add_ad_rhs(138, 399, A::div_scaled_product(s.ad_value(402), s.ad_value(402), 1.0, s.ad_value(341), 6.0));
            s.store_scaled_sub_ad_rhs(137, 399, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(402), 1.0, A::mul_offset_rhs(A::div(s.ad_value(402), s.ad_value(341)), A::div_scaled_inputs(s.ad_value(402), 1.0, s.ad_value(341), 5.0), 1.0), 1.0 / (6.0)), (-0.5));
        }

        s.store_div_from_scalar(208, 1.0, 207);

        s.store_add_scaled_products_right_left_ad(138, 208, 138, 1.0, A::offset(s.ad_value(207), (-1.0)), 393, 1.0);

        s.store_add_scaled_products_mixed_aiai(137, A::square(s.ad_value(208)), 137, 1.0, A::sub(s.ad_value(207), s.ad_value(208)), 393, 0.5);

        s.b[1471] = (p.p73 == 2.0);
        s.v[1471] = if s.b[1471] { 1.0 } else { 0.0 };

        if s.b[1471] {
            s.store_div_scaled_inputs_indices(571, 137, -1.0, 138, 1.0);
        }

        if (!s.b[1471]) {
            s.store_scalar(571, 0.0);
        }

        s.store_sub_scaled_inputs(139, 138, -1.0, 137, 1.0);

        s.store_mul3_affine_lhs(175, 159, 156, s.v[115], 0.0, 372);

        s.store_mul(138, 175, 138);

        s.store_mul(137, 175, 137);

        s.store_mul(139, 175, 139);

        s.copy_ad(592, 138);

        s.b[1472] = (p.p61 != 0.0);
        s.v[1472] = if s.b[1472] { 1.0 } else { 0.0 };

        s.b[1473] = (p.p62 == 5.0);
        s.v[1473] = if s.b[1473] { 1.0 } else { 0.0 };

        if (s.b[1472] && s.b[1473]) {
            s.store_mul3_affine_lhs(169, 160, 157, s.v[115], 0.0, 494);
        }

        if (s.b[1472] && (!s.b[1473])) {
            s.store_mul3_affine_lhs(169, 159, 157, s.v[115], 0.0, 494);
        }

        if s.b[1472] {
            s.copy_ad(176, 904);
            s.store_mul(340, 176, 169);
            s.store_neg(495, 340);
            s.copy_ad(496, 340);
            s.store_mul3_affine_lhs(169, 159, 156, s.v[115], 0.0, 163);
            s.store_sub(170, 401, 904);
            s.store_mul(340, 169, 170);
            s.store_sub(495, 495, 340);
            s.store_add(496, 496, 340);
            s.store_mul3_affine_lhs(169, 159, 156, s.v[115], 0.0, 163);
            s.store_scaled_mul_ad(170, A::offset(s.ad_value(923), (-1.0)), A::add(s.ad_value(399), A::div_scaled_product(s.ad_value(402), s.ad_value(402), 1.0, s.ad_value(341), 6.0)), 0.5);
            s.store_mul(340, 169, 170);
            s.store_sub(495, 495, 340);
            s.store_add(496, 496, 340);
        }

        s.b[1474] = (s.v[128] < 0.0);
        s.v[1474] = if s.b[1474] { 1.0 } else { 0.0 };

        if s.b[1474] {
            s.copy_ad(169, 137);
            s.copy_ad(137, 139);
            s.copy_ad(139, 169);
        }

        s.b[1475] = (p.p78 != 1.0);
        s.v[1475] = if s.b[1475] { 1.0 } else { 0.0 };

        s.b[1476] = (p.p76 != 2.0);
        s.v[1476] = if s.b[1476] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_21(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1475] && s.b[1476]) {
            s.store_scaled_mul(169, 159, 114, s.v[115]);
            s.store_mul_voltage_ad(170, s.ad_value(114), ctx, nodes, Some(10), Some(6));
            s.store_offset_sub(168, 170, 518, 0.02);
            s.store_scaled_sub_sqrt_square_offset_rhs(510, 168, 168, (4.0 * 0.02), 0.5);
            s.store_mul_ad_rhs(498, 169, A::add_scaled_products(s.ad_value(648), A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(170), 1.0, s.ad_value(518), (-1.0), s.ad_value(510), -1.0), 1.0, s.ad_value(651), A::sqrt(A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(510), 4.0, s.ad_value(651), 1.0))), (-1.0), (-0.5)), 1.0, s.ad_value(646), s.ad_value(170), 1.0));
            s.store_mul_voltage_ad(170, s.ad_value(114), ctx, nodes, Some(10), Some(5));
            s.store_offset_sub(168, 170, 518, 0.02);
            s.store_scaled_sub_sqrt_square_offset_rhs(511, 168, 168, (4.0 * 0.02), 0.5);
            s.store_mul_ad_rhs(499, 169, A::add_scaled_products(s.ad_value(649), A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(170), 1.0, s.ad_value(518), (-1.0), s.ad_value(511), -1.0), 1.0, s.ad_value(652), A::sqrt(A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(511), 4.0, s.ad_value(652), 1.0))), (-1.0), (-0.5)), 1.0, s.ad_value(647), s.ad_value(170), 1.0));
        }

        if (s.b[1475] && (!s.b[1476])) {
            s.store_scaled_mul(169, 159, 114, s.v[115]);
            s.store_mul_voltage_ad(170, s.ad_value(114), ctx, nodes, Some(13), Some(6));
            s.store_offset_sub(168, 170, 518, 0.02);
            s.store_scaled_sub_sqrt_square_offset_rhs(510, 168, 168, (4.0 * 0.02), 0.5);
            s.store_mul_ad_rhs(498, 169, A::add_scaled_products(s.ad_value(648), A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(170), 1.0, s.ad_value(518), (-1.0), s.ad_value(510), -1.0), 1.0, s.ad_value(651), A::sqrt(A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(510), 4.0, s.ad_value(651), 1.0))), (-1.0), (-0.5)), 1.0, s.ad_value(646), s.ad_value(170), 1.0));
            s.store_mul_voltage_ad(170, s.ad_value(114), ctx, nodes, Some(14), Some(5));
            s.store_offset_sub(168, 170, 518, 0.02);
            s.store_scaled_sub_sqrt_square_offset_rhs(511, 168, 168, (4.0 * 0.02), 0.5);
            s.store_mul_ad_rhs(499, 169, A::add_scaled_products(s.ad_value(649), A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(170), 1.0, s.ad_value(518), (-1.0), s.ad_value(511), -1.0), 1.0, s.ad_value(652), A::sqrt(A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(511), 4.0, s.ad_value(652), 1.0))), (-1.0), (-0.5)), 1.0, s.ad_value(647), s.ad_value(170), 1.0));
        }

        s.b[1477] = (p.p78 == 0.0);
        s.v[1477] = if s.b[1477] { 1.0 } else { 0.0 };

        s.b[1478] = (p.p76 != 2.0);
        s.v[1478] = if s.b[1478] { 1.0 } else { 0.0 };

        if (s.b[1477] && s.b[1478]) {
            s.store_scale(169, 159, s.v[115]);
            s.store_mul_ad_product_rhs_mixed_ia(500, 169, 643, A::voltage(ctx, nodes, Some(10), Some(6)));
            s.store_mul_ad_product_rhs_mixed_ia(501, 169, 642, A::voltage(ctx, nodes, Some(10), Some(5)));
            s.store_add(505, 498, 500);
            s.store_add(506, 499, 501);
        }

        if (s.b[1477] && (!s.b[1478])) {
            s.store_scale(169, 159, s.v[115]);
            s.store_mul_ad_product_rhs_mixed_ia(500, 169, 643, A::voltage(ctx, nodes, Some(13), Some(6)));
            s.store_mul_ad_product_rhs_mixed_ia(501, 169, 642, A::voltage(ctx, nodes, Some(14), Some(5)));
            s.store_add(505, 498, 500);
            s.store_add(506, 499, 501);
        }

        s.b[1479] = (p.p78 == 1.0);
        s.v[1479] = if s.b[1479] { 1.0 } else { 0.0 };

        s.b[1480] = (p.p76 != 2.0);
        s.v[1480] = if s.b[1480] { 1.0 } else { 0.0 };

        s.b[1481] = (p.p63 == 1.0);
        s.v[1481] = if s.b[1481] { 1.0 } else { 0.0 };

        if ((((!s.b[1477]) && s.b[1479]) && s.b[1480]) && s.b[1481]) {
            s.store_scale(168, 159, s.v[115]);
            s.store_mul(644, 168, 644);
            s.store_mul(645, 168, 645);
            s.store_scale(513, 168, p.p15);
            s.store_scale(514, 168, p.p16);
        }

        if ((((!s.b[1477]) && s.b[1479]) && s.b[1480]) && (!s.b[1481])) {
            s.store_scalar(513, p.p15);
            s.store_scalar(514, p.p16);
        }

        if (((!s.b[1477]) && s.b[1479]) && s.b[1480]) {
            s.store_mul_voltage_ad(498, s.ad_value(644), ctx, nodes, Some(10), Some(6));
            s.store_mul_voltage_ad(499, s.ad_value(645), ctx, nodes, Some(10), Some(5));
            s.copy_ad(505, 498);
            s.copy_ad(506, 499);
            s.store_mul_voltage_ad(500, s.ad_value(513), ctx, nodes, Some(10), Some(2));
            s.store_mul_voltage_ad(501, s.ad_value(514), ctx, nodes, Some(10), Some(0));
        }

        s.b[1482] = (p.p63 == 1.0);
        s.v[1482] = if s.b[1482] { 1.0 } else { 0.0 };

        if ((((!s.b[1477]) && s.b[1479]) && (!s.b[1480])) && s.b[1482]) {
            s.store_scale(168, 159, s.v[115]);
            s.store_mul(644, 168, 644);
            s.store_mul(645, 168, 645);
            s.store_scale(513, 168, p.p15);
            s.store_scale(514, 168, p.p16);
        }

        if ((((!s.b[1477]) && s.b[1479]) && (!s.b[1480])) && (!s.b[1482])) {
            s.store_scalar(513, p.p15);
            s.store_scalar(514, p.p16);
        }

        if (((!s.b[1477]) && s.b[1479]) && (!s.b[1480])) {
            s.store_mul_voltage_ad(498, s.ad_value(644), ctx, nodes, Some(13), Some(6));
            s.store_mul_voltage_ad(499, s.ad_value(645), ctx, nodes, Some(14), Some(5));
            s.copy_ad(505, 498);
            s.copy_ad(506, 499);
            s.store_mul_voltage_ad(500, s.ad_value(513), ctx, nodes, Some(13), Some(2));
            s.store_mul_voltage_ad(501, s.ad_value(514), ctx, nodes, Some(14), Some(0));
        }

        s.b[1483] = (p.p76 != 2.0);
        s.v[1483] = if s.b[1483] { 1.0 } else { 0.0 };

        if (((!s.b[1477]) && (!s.b[1479])) && s.b[1483]) {
            s.store_mul_voltage_ad(500, s.ad_value(453), ctx, nodes, Some(10), Some(6));
            s.store_mul_voltage_ad(501, s.ad_value(453), ctx, nodes, Some(10), Some(5));
            s.store_add(505, 498, 500);
            s.store_add(506, 499, 501);
        }

        if (((!s.b[1477]) && (!s.b[1479])) && (!s.b[1483])) {
            s.store_mul_voltage_ad(500, s.ad_value(453), ctx, nodes, Some(13), Some(6));
            s.store_mul_voltage_ad(501, s.ad_value(453), ctx, nodes, Some(14), Some(5));
            s.store_add(505, 498, 500);
            s.store_add(506, 499, 501);
        }

        s.b[1484] = (p.p65 == 1.0);
        s.v[1484] = if s.b[1484] { 1.0 } else { 0.0 };

        if s.b[1484] {
            s.store_scalar(239, 1e-6);
            s.store_mul_div_scaled_inputs_mixed_aii(178, A::sqrt(s.ad_value(179)), 239, 1.0, 181, 2.0);
            s.store_scale(168, 178, 0.5);
            s.store_div_scaled_inputs_mixed_ai(170, A::offset(s.ad_value(132), (-p.p144)), -1.0, 179, 1.0);
        }

        s.b[1485] = ((s.v[170] * s.v[179]) > (s.v[166] + (s.v[178] * (((s.v[166] * s.v[179])) as f64).sqrt())));
        s.v[1485] = if s.b[1485] { 1.0 } else { 0.0 };

        if (s.b[1484] && s.b[1485]) {
            s.store_sub_ad_lhs(169, A::sqrt(A::add(A::offset(s.ad_value(170), (-1.0)), A::square(s.ad_value(168)))), 168);
            s.store_offset_square(340, 169, 1.0);
            s.store_offset_ad(175, A::limited_exp_scaled_input(s.ad_value(340), -1.0), (-1.0));
        }

        if (s.b[1484] && (!s.b[1485])) {
            s.store_sub_scaled_ad_rhs(171, 170, 0.5, A::scale_offset(s.ad_value(178), ((1.0 / (((2.0) as f64).sqrt())) * (3.0)), 3.0));
            s.store_add_ad_rhs(340, 171, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(171)), 1.0, s.ad_value(170), 6.0)));
        }

        s.b[1486] = (s.v[170] < 0.0);
        s.v[1486] = if s.b[1486] { 1.0 } else { 0.0 };

        if ((s.b[1484] && (!s.b[1485])) && s.b[1486]) {
            s.store_div_scaled_inputs2_indices(172, 170, 1.0, 340, (-1.0), 178, 1.0);
            s.store_sub_ad_lhs(175, A::square(s.ad_value(172)), 340);
        }

        if ((s.b[1484] && (!s.b[1485])) && s.b[1486]) {
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

        if ((s.b[1484] && (!s.b[1485])) && (!s.b[1486])) {
            s.store_limited_exp_scaled_input(341, 340, (-1.2));
            s.store_sub_ad_lhs(172, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(170), 1.0, s.ad_value(341), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);
            s.store_add_ad(340, A::sub_from_scalar(1.0, s.ad_value(341)), A::square(s.ad_value(172)));
            s.store_offset_ad(175, A::limited_exp_scaled_input(s.ad_value(340), -1.0), (-1.0));
        }

        if s.b[1484] {
            s.store_sqrt_add(176, 175, 340);
        }

        s.b[1487] = (s.v[340] > 1e-15);
        s.v[1487] = if s.b[1487] { 1.0 } else { 0.0 };

        if (s.b[1484] && s.b[1487]) {
            s.store_add_scaled_inputs_product_indices(344, 170, -1.0, 340, 1.0, 178, 176, 1.0);
            s.store_sub_from_scalar_ad(345, 1.0, A::div_scaled_product(s.ad_value(178), s.ad_value(175), 0.5, s.ad_value(176), 1.0));
            s.store_sub_div_rhs_indices(177, 340, 344, 345);
        }

        s.b[1488] = (s.v[340] < (-1e-15));
        s.v[1488] = if s.b[1488] { 1.0 } else { 0.0 };

        if ((s.b[1484] && (!s.b[1487])) && s.b[1488]) {
            s.store_add_scaled_inputs_product_indices(344, 170, -1.0, 340, 1.0, 178, 176, (-1.0));
            s.store_offset_div_scaled_product(345, s.ad_value(178), s.ad_value(175), 0.5, s.ad_value(176), 1.0, 1.0);
            s.store_sub_div_rhs_indices(177, 340, 344, 345);
        }

        if ((s.b[1484] && (!s.b[1487])) && (!s.b[1488])) {
            s.store_scalar(177, 0.0);
        }

        if s.b[1484] {
            s.store_mul_ad_product_lhs_mixed_ia(906, 178, A::limited_exp_scaled_input(s.ad_value(177), (-1.0 / (2.0))), 179);
            s.store_abs_voltage(915, ctx, nodes, Some(7), Some(6));
            s.store_mul_div_from_scalar_lhs(916, (2.0 * p.p454), 416, 397);
            s.store_scale(917, 916, p.p1);
            s.store_scalar(920, (1.0 / p.p530));
            s.store_add_scaled_inputs(175, 906, p.p491, 182, (2.0 * p.p491));
            s.store_div_scaled_product_add_scaled_denominator_indices(918, 917, 175, 1.0, 917, 1.0, 175, 1.0, 1.0);
        }

        if s.b[1484] {
            s.store_offset_ad(918, {
                if (!((s.v[918] - 0.001) < ((-10000.0) * 1e-5))) {
                    A::add_scaled_inputs(A::offset(s.ad_value(918), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(918), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5)
                } else {
                    {
                        if ((s.v[918] - 0.001) < ((-10000.0) * 1e-5)) {
                            A::div_scalar_offset_denominator(((-1e-5) * 1e-5), s.ad_value(918), (-0.001), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.001);
        }

        if s.b[1484] {
            s.store_powf_ad(176, A::offset(A::div(s.ad_value(915), s.ad_value(918)), 1e-6), p.p530);
            s.store_pow_ad(177, A::offset(s.ad_value(176), 1.0), s.ad_value(920));
            s.store_min_ad(919, A::div(s.ad_value(915), s.ad_value(177)), s.ad_value(915));
            s.store_scalar(239, 1e-6);
            s.store_mul_div_scaled_inputs_mixed_aii(178, A::sqrt(s.ad_value(179)), 239, 1.0, 181, 2.0);
            s.store_scale(168, 178, 0.5);
            s.store_div_scaled_inputs_mixed_ai(170, A::offset(A::add(s.ad_value(133), s.ad_value(919)), (-p.p143)), -1.0, 179, 1.0);
        }

        s.b[1489] = ((s.v[170] * s.v[179]) > (s.v[166] + (s.v[178] * (((s.v[166] * s.v[179])) as f64).sqrt())));
        s.v[1489] = if s.b[1489] { 1.0 } else { 0.0 };

        if (s.b[1484] && s.b[1489]) {
            s.store_sub_ad_lhs(169, A::sqrt(A::add(A::offset(s.ad_value(170), (-1.0)), A::square(s.ad_value(168)))), 168);
            s.store_offset_square(340, 169, 1.0);
            s.store_offset_ad(175, A::limited_exp_scaled_input(s.ad_value(340), -1.0), (-1.0));
        }

        if (s.b[1484] && (!s.b[1489])) {
            s.store_sub_scaled_ad_rhs(171, 170, 0.5, A::scale_offset(s.ad_value(178), ((1.0 / (((2.0) as f64).sqrt())) * (3.0)), 3.0));
            s.store_add_ad_rhs(340, 171, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(171)), 1.0, s.ad_value(170), 6.0)));
        }

        s.b[1490] = (s.v[170] < 0.0);
        s.v[1490] = if s.b[1490] { 1.0 } else { 0.0 };

        if ((s.b[1484] && (!s.b[1489])) && s.b[1490]) {
            s.store_div_scaled_inputs2_indices(172, 170, 1.0, 340, (-1.0), 178, 1.0);
            s.store_sub_ad_lhs(175, A::square(s.ad_value(172)), 340);
        }

    }

    pub(super) fn stamp_transient_block_22(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[1484] && (!s.b[1489])) && s.b[1490]) {
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

        if ((s.b[1484] && (!s.b[1489])) && (!s.b[1490])) {
            s.store_limited_exp_scaled_input(341, 340, (-1.2));
            s.store_sub_ad_lhs(172, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(170), 1.0, s.ad_value(341), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);
            s.store_add_ad(340, A::sub_from_scalar(1.0, s.ad_value(341)), A::square(s.ad_value(172)));
            s.store_offset_ad(175, A::limited_exp_scaled_input(s.ad_value(340), -1.0), (-1.0));
        }

        if s.b[1484] {
            s.store_sqrt_add(176, 175, 340);
        }

        s.b[1491] = (s.v[340] > 1e-15);
        s.v[1491] = if s.b[1491] { 1.0 } else { 0.0 };

        if (s.b[1484] && s.b[1491]) {
            s.store_add_scaled_inputs_product_indices(344, 170, -1.0, 340, 1.0, 178, 176, 1.0);
            s.store_sub_from_scalar_ad(345, 1.0, A::div_scaled_product(s.ad_value(178), s.ad_value(175), 0.5, s.ad_value(176), 1.0));
            s.store_sub_div_rhs_indices(177, 340, 344, 345);
        }

        s.b[1492] = (s.v[340] < (-1e-15));
        s.v[1492] = if s.b[1492] { 1.0 } else { 0.0 };

        if ((s.b[1484] && (!s.b[1491])) && s.b[1492]) {
            s.store_add_scaled_inputs_product_indices(344, 170, -1.0, 340, 1.0, 178, 176, (-1.0));
            s.store_offset_div_scaled_product(345, s.ad_value(178), s.ad_value(175), 0.5, s.ad_value(176), 1.0, 1.0);
            s.store_sub_div_rhs_indices(177, 340, 344, 345);
        }

        if ((s.b[1484] && (!s.b[1491])) && (!s.b[1492])) {
            s.store_scalar(177, 0.0);
        }

        if s.b[1484] {
            s.store_mul_ad_product_lhs_mixed_ia(907, 178, A::limited_exp_scaled_input(s.ad_value(177), (-1.0 / (2.0))), 179);
            s.store_sub(911, 906, 907);
            s.store_scaled_add(910, 906, 907, 0.5);
            s.store_div_scaled_inputs2_indices(341, 910, 2.0, 181, 1.0, 209, 1.0);
            s.store_add_ad_rhs(905, 910, A::div_scaled_product(s.ad_value(911), s.ad_value(911), 1.0, s.ad_value(341), 6.0));
            s.store_scaled_sub_ad_rhs(909, 910, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(911), 1.0, A::mul_offset_rhs(A::div(s.ad_value(911), s.ad_value(341)), A::div_scaled_inputs(s.ad_value(911), 1.0, s.ad_value(341), 5.0), 1.0), 1.0 / (6.0)), 0.5);
            s.store_sub(908, 905, 909);
        }

        s.b[1493] = (p.p62 == 5.0);
        s.v[1493] = if s.b[1493] { 1.0 } else { 0.0 };

        if (s.b[1484] && s.b[1493]) {
            s.store_scaled_mul(169, 160, 494, (s.v[115] * p.p1));
        }

        if (s.b[1484] && (!s.b[1493])) {
            s.store_scaled_mul(169, 159, 494, (s.v[115] * p.p1));
        }

        if s.b[1484] {
            s.copy_ad(176, 908);
            s.copy_ad(177, 909);
            s.store_mul(340, 176, 169);
            s.store_mul(341, 177, 169);
            s.copy_ad(908, 340);
            s.copy_ad(909, 341);
            s.copy_ad(504, 908);
            s.copy_ad(503, 909);
        }

        s.store_scaled_voltage(502, ctx, nodes, Some(0), Some(2), p.p17);

        s.b[1494] = (p.p71 == 1.0);
        s.v[1494] = if s.b[1494] { 1.0 } else { 0.0 };

        if s.b[1494] {
            s.store_div_scaled_add_product(168, s.ad_value(259), 1.0, s.ad_value(260), s.ad_value(153), 1.0, s.ad_value(153), 1.0);
        }

        s.b[1495] = ((s.v[168] <= 0.0) || (s.v[248] <= 0.0));
        s.v[1495] = if s.b[1495] { 1.0 } else { 0.0 };

        if (s.b[1494] && s.b[1495]) {
            s.store_scalar(488, 0.0);
        }

        if (s.b[1494] && (!s.b[1495])) {
            s.store_div_scaled_value_offset_denominator(169, s.ad_value(248), -1.0, s.ad_value(202), 1e-30, 1.0);
            s.store_mul_product3_mixed_aiii(488, A::limited_exp(s.ad_value(169)), 168, 202, 124, 1.0);
        }

        s.b[1496] = (p.p71 == 2.0);
        s.v[1496] = if s.b[1496] { 1.0 } else { 0.0 };

        if ((!s.b[1494]) && s.b[1496]) {
            s.store_div_scaled_add_product(493, s.ad_value(261), 1.0, s.ad_value(262), s.ad_value(153), 1.0, s.ad_value(153), 1.0);
        }

        s.b[1497] = (s.v[493] <= 0.0);
        s.v[1497] = if s.b[1497] { 1.0 } else { 0.0 };

        if (((!s.b[1494]) && s.b[1496]) && s.b[1497]) {
            s.store_scalar(488, 0.0);
        }

        if (((!s.b[1494]) && s.b[1496]) && (!s.b[1497])) {
            s.store_mul(168, 783, 153);
            s.store_div_scaled_product_offset_denominator(169, s.ad_value(249), s.ad_value(168), 1.0, s.ad_value(168), 1.0, 1.0);
        }

        if (((!s.b[1494]) && s.b[1496]) && (!s.b[1497])) {
            s.store_div_from_scalar_offset_ad(168, 1.0, {
                if (!((s.v[786] * s.v[348]) < ((-10000.0) * p.p1441))) {
                    A::add_scaled_product(A::sqrt_square_offset(A::mul(s.ad_value(786), s.ad_value(348)), ((4.0 * p.p1441) * p.p1441)), 0.5, s.ad_value(786), s.ad_value(348), 0.5)
                } else {
                    {
                        if ((s.v[786] * s.v[348]) < ((-10000.0) * p.p1441)) {
                            A::div_from_scalar(((-p.p1441) * p.p1441), A::mul(s.ad_value(786), s.ad_value(348)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0);
        }

        if (((!s.b[1494]) && s.b[1496]) && (!s.b[1497])) {
            s.store_add(171, 168, 787);
        }

        if (((!s.b[1494]) && s.b[1496]) && (!s.b[1497])) {
            if (!((s.v[348] * s.v[171]) < ((-10000.0) * p.p1442))) {
                s.store_add_scaled_product_value_ad(170, A::sqrt_square_offset(A::mul(s.ad_value(348), s.ad_value(171)), ((4.0 * p.p1442) * p.p1442)), 0.5, 348, 171, 0.5);
            } else {
                if ((s.v[348] * s.v[171]) < ((-10000.0) * p.p1442)) {
                    s.store_div_from_scalar_mul_ad(170, ((-p.p1442) * p.p1442), s.ad_value(348), s.ad_value(171));
                } else {
                    s.store_scalar(170, 0.0);
                }
            }
        }

        if (((!s.b[1494]) && s.b[1496]) && (!s.b[1497])) {
            s.store_div_from_scalar_offset_product(171, 1.0, 788, 126, 1.0);
            s.store_mul3_lhs(491, 169, 170, 171);
            s.store_mul_sub_from_scalar_ad_rhs(490, 491, 1.0, A::div(s.ad_value(784), s.ad_value(153)));
            s.store_sub(489, 126, 490);
            s.store_add_ad(168, A::add_scaled_product(s.ad_value(782), 1.0, s.ad_value(781), s.ad_value(489), 1.0), A::mul3(s.ad_value(780), s.ad_value(489), s.ad_value(489)));
            s.store_sqrt_square_offset(169, 168, 1e-10);
        }

        if (((!s.b[1494]) && s.b[1496]) && (!s.b[1497])) {
            let assign27440_ad_e47205: A = A::limited_exp(A::div(s.ad_value(489), s.ad_value(169)));
            s.store_neg_ad(492, A::offset(A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(493), -1.0, assign27440_ad_e47205), (((-(-10.0))) + ((-p.p1443)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(493), -1.0, assign27440_ad_e47205), (((-(-10.0))) + ((-p.p1443)))), (-((4.0 * (-10.0)) * p.p1443))), 0.5), (-10.0)));
        }

        if (((!s.b[1494]) && s.b[1496]) && (!s.b[1497])) {
            s.store_mul(488, 492, 124);
        }

        s.b[1498] = (p.p69 != 0.0);
        s.v[1498] = if s.b[1498] { 1.0 } else { 0.0 };

        if s.b[1498] {
            s.store_div_scaled_inputs2_by_product(169, s.ad_value(399), 1.0, s.ad_value(725), (-1.0), s.ad_value(726), s.ad_value(179), 1.0);
        }

        if s.b[1498] {
            s.store_mul_ad_product_rhs_mixed_ia(460, 726, 179, {
                if ((!(s.v[169] > 37.0)) && (!(s.v[169] < (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(169))
                } else {
                    {
                        if ((!(s.v[169] > 37.0)) && (s.v[169] < (-37.0))) {
                            A::exp(s.ad_value(169))
                        } else {
                            {
                                if (s.v[169] > 37.0) {
                                    s.ad_value(169)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            });
        }

        if s.b[1498] {
            s.store_offset_add_scaled_inputs(170, A::offset(A::add_scaled_product(s.ad_value(243), 1.0, s.ad_value(723), s.ad_value(399), (-1.0)), (((-(-p.p1110))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(s.ad_value(243), 1.0, s.ad_value(723), s.ad_value(399), (-1.0)), (((-(-p.p1110))) + ((-1e-6)))), (-((4.0 * (-p.p1110)) * 1e-6))), 0.5, (-p.p1110));
            s.store_offset_mul(171, 724, 399, 1.0);
            s.store_scaled_mul(172, 170, 171, ((-982222000000.0) * p.p1109));
            s.store_limited_exp(174, 172);
            s.store_scalar(175, 3.75956e-7);
            s.store_mul_ad_product_lhs_mixed_ai(461, A::mul3(A::mul3(s.ad_value(158), s.ad_value(153), s.ad_value(175)), s.ad_value(486), s.ad_value(497)), 460, 174);
            s.store_scaled_mul(461, 461, 256, p.p27);
            s.store_add_scaled_inputs3_indices(468, 167, 1.0, 146, (-0.5), 166, -1.0);
            s.store_sub(168, 468, 497);
            s.store_div_scaled_value_by_product(169, s.ad_value(168), 1.0, s.ad_value(731), s.ad_value(179), 1.0);
        }

        if s.b[1498] {
            s.store_mul_ad_product_rhs_mixed_ia(467, 731, 179, {
                if ((!(s.v[169] > 37.0)) && (!(s.v[169] < (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(169))
                } else {
                    {
                        if ((!(s.v[169] > 37.0)) && (s.v[169] < (-37.0))) {
                            A::exp(s.ad_value(169))
                        } else {
                            {
                                if (s.v[169] > 37.0) {
                                    s.ad_value(169)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            });
        }

        s.b[1499] = (p.p61 != 0.0);
        s.v[1499] = if s.b[1499] { 1.0 } else { 0.0 };

        if (s.b[1498] && s.b[1499]) {
            s.copy_ad(466, 904);
        }

        s.b[1500] = (s.v[468] <= 0.0);
        s.v[1500] = if s.b[1500] { 1.0 } else { 0.0 };

        if ((s.b[1498] && (!s.b[1499])) && s.b[1500]) {
            s.store_scaled_add_ad(466, A::offset(s.ad_value(168), (-0.02)), A::sqrt(A::sub_scaled_inputs(A::square(A::offset(s.ad_value(168), (-0.02))), 1.0, s.ad_value(468), 0.08)), 0.5);
        }

        if ((s.b[1498] && (!s.b[1499])) && (!s.b[1500])) {
            s.store_scaled_add_ad(466, A::offset(s.ad_value(168), (-0.02)), A::sqrt(A::add_scaled_inputs(A::square(A::offset(s.ad_value(168), (-0.02))), 1.0, s.ad_value(468), 0.08)), 0.5);
        }

        if s.b[1498] {
            s.store_offset_add_scaled_inputs(170, A::offset(A::add_scaled_product(s.ad_value(244), 1.0, s.ad_value(729), s.ad_value(466), (-1.0)), (((-(-p.p1111))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(s.ad_value(244), 1.0, s.ad_value(729), s.ad_value(466), (-1.0)), (((-(-p.p1111))) + ((-1e-6)))), (-((4.0 * (-p.p1111)) * 1e-6))), 0.5, (-p.p1111));
            s.store_offset_mul(171, 730, 466, 1.0);
            s.store_scaled_mul(172, 170, 171, ((-745669000000.0) * p.p1109));
            s.store_limited_exp(174, 172);
            s.store_scalar(175, 4.97232e-7);
            s.store_mul_ad_product_lhs_mixed_ai(469, A::mul3(A::mul3(s.ad_value(158), s.ad_value(153), s.ad_value(175)), s.ad_value(486), s.ad_value(497)), 467, 174);
            s.store_scaled_mul(469, 469, 256, p.p27);
        }

        s.b[1501] = (p.p68 != 0.0);
        s.v[1501] = if s.b[1501] { 1.0 } else { 0.0 };

        if s.b[1501] {
            s.store_offset_add_scaled_inputs(169, A::offset(A::add_scaled_product(s.ad_value(245), 1.0, s.ad_value(734), s.ad_value(399), (-1.0)), (((-(-p.p1112))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(s.ad_value(245), 1.0, s.ad_value(734), s.ad_value(399), (-1.0)), (((-(-p.p1112))) + ((-1e-6)))), (-((4.0 * (-p.p1112)) * 1e-6))), 0.5, (-p.p1112));
            s.store_offset_mul(170, 735, 399, 1.0);
            s.store_mul3_affine_lhs(171, 485, 169, (-p.p1109), 0.0, 170);
            s.store_mul_limited_exp_rhs(172, 399, 171);
            s.store_add_scaled_inputs4_indices(174, 497, 1.0, 127, 0.5, 521, 0.5, 522, 0.5);
            s.store_mul_ad_product_lhs_mixed_ai(472, A::mul3(A::mul3_scaled_output(s.ad_value(158), s.ad_value(153), s.ad_value(484), p.p26), s.ad_value(486), s.ad_value(172)), 174, 256);
            s.store_offset_sqrt_ad(473, A::offset(A::square(s.ad_value(390)), 0.01), (-0.1));
            s.store_mul(169, 736, 473);
            s.store_limited_exp_neg_input(474, 169);
            s.store_offset_add(171, 169, 474, (((-1.0)) + (0.0001)));
            s.store_offset_sub_from_scalar_ad(172, 1.0, A::mul_offset_lhs(s.ad_value(169), 1.0, s.ad_value(474)), 0.0001);
            s.store_offset_square(174, 169, 0.0002);
            s.store_div_scaled_product_indices(471, 472, 172, 1.0, 174, 1.0);
            s.store_div_scaled_product_indices(470, 472, 171, 1.0, 174, 1.0);
            s.store_sub(168, 134, 479);
            s.store_sqrt_square_offset(482, 168, 0.0001);
        }

        s.b[1502] = (p.p82 == 1.0);
        s.v[1502] = if s.b[1502] { 1.0 } else { 0.0 };

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

        s.b[1503] = (s.v[740] < 0.01);
        s.v[1503] = if s.b[1503] { 1.0 } else { 0.0 };

        if ((s.b[1501] && s.b[1502]) && s.b[1503]) {
            s.store_scalar(740, 0.01);
        }

        if (s.b[1501] && (!s.b[1502])) {
            s.store_add_scaled_product_indices(169, 246, 1.0, 739, 482, (-1.0));
        }

        if s.b[1501] {
            s.store_offset_mul(170, 740, 482, 1.0);
            s.store_mul_product3_indices(171, 170, 485, 742, 169, (-p.p1109));
            s.store_limited_exp(172, 171);
        }

        s.b[1504] = (s.v[128] > 0.0);
        s.v[1504] = if s.b[1504] { 1.0 } else { 0.0 };

        if (s.b[1501] && s.b[1504]) {
            s.store_mul_product3_indices(480, 172, 462, 134, 482, p.p1104);
        }

    }

    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1501] && (!s.b[1504])) {
            s.store_mul_product3_indices(481, 172, 462, 134, 482, p.p1104);
        }

        if s.b[1501] {
            s.store_sub(168, 136, 479);
            s.store_sqrt_square_offset(483, 168, 0.0001);
        }

        s.b[1505] = (p.p82 == 1.0);
        s.v[1505] = if s.b[1505] { 1.0 } else { 0.0 };

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

        s.b[1506] = (s.v[746] < 0.01);
        s.v[1506] = if s.b[1506] { 1.0 } else { 0.0 };

        if ((s.b[1501] && s.b[1505]) && s.b[1506]) {
            s.store_scalar(746, 0.01);
        }

        if (s.b[1501] && (!s.b[1505])) {
            s.store_add_scaled_product_indices(169, 247, 1.0, 745, 483, (-1.0));
        }

        if s.b[1501] {
            s.store_offset_mul(170, 746, 483, 1.0);
            s.store_mul_product3_indices(171, 170, 485, 742, 169, (-p.p1109));
            s.store_limited_exp(172, 171);
        }

        s.b[1507] = (s.v[128] > 0.0);
        s.v[1507] = if s.b[1507] { 1.0 } else { 0.0 };

        if (s.b[1501] && s.b[1507]) {
            s.store_mul_product3_indices(481, 172, 462, 136, 483, p.p1105);
        }

        if (s.b[1501] && (!s.b[1507])) {
            s.store_mul_product3_indices(480, 172, 462, 136, 483, p.p1105);
        }

        s.b[1508] = (p.p70 != 0.0);
        s.v[1508] = if s.b[1508] { 1.0 } else { 0.0 };

        if s.b[1508] {
            s.store_scalar(168, (s.v[145] * p.p89));
        }

        s.b[1509] = ((s.v[747] <= 0.0) || (s.v[252] <= 0.0));
        s.v[1509] = if s.b[1509] { 1.0 } else { 0.0 };

        if (s.b[1508] && s.b[1509]) {
            s.store_scalar(175, 0.0);
        }

        if (s.b[1508] && (!s.b[1509])) {
            s.store_div_scaled_inputs3_indices(169, 136, -1.0, 750, (-1.0), 479, 1.0, 168, 1.0);
        }

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

        if (s.b[1508] && (!s.b[1509])) {
            s.store_div_scaled_value_offset_denominator(170, s.ad_value(252), 1.0, s.ad_value(169), 0.001, 1.0);
            s.store_pow_ad(171, s.ad_value(169), s.ad_value(751));
        }

        s.b[1510] = (p.p61 != 0.0);
        s.v[1510] = if s.b[1510] { 1.0 } else { 0.0 };

        if ((s.b[1508] && (!s.b[1509])) && s.b[1510]) {
            s.store_mul3_affine_lhs(172, 522, 522, -1.0, 0.0, 522);
            s.store_offset_add_ad(173, s.ad_value(749), A::abs(s.ad_value(172)), 1e-5);
        }

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

        if ((s.b[1508] && (!s.b[1509])) && s.b[1510]) {
            s.store_mul_ad_product_lhs(175, A::mul3(s.ad_value(747), s.ad_value(158), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), 174);
        }

        if ((s.b[1508] && (!s.b[1509])) && (!s.b[1510])) {
            s.store_mul_ad_product_lhs(175, A::mul3(s.ad_value(747), s.ad_value(158), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), 135);
        }

        s.b[1511] = ((p.p70 == 3.0) && (s.v[752] > 0.0));
        s.v[1511] = if s.b[1511] { 1.0 } else { 0.0 };

        s.b[1512] = (p.p61 != 0.0);
        s.v[1512] = if s.b[1512] { 1.0 } else { 0.0 };

        if ((s.b[1508] && s.b[1511]) && s.b[1512]) {
            s.store_mul_ad_rhs(254, 754, {
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

        if ((s.b[1508] && s.b[1511]) && s.b[1512]) {
            s.store_div_scaled_inputs3_mixed_aiii(169, A::add_scaled_product(A::mul3(s.ad_value(753), s.ad_value(136), s.ad_value(136)), 1.0, s.ad_value(254), s.ad_value(136), (-1.0)), 1.0, 755, (-1.0), 479, 1.0, 179, 1.0);
            s.store_mul_product3_mixed_aiii(170, A::limited_exp(s.ad_value(169)), 752, 158, 141, 1.0);
            s.store_mul3_affine_lhs(172, 522, 522, -1.0, 0.0, 522);
            s.store_offset_add_ad(173, s.ad_value(749), A::abs(s.ad_value(172)), 1e-5);
        }

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

        if ((s.b[1508] && s.b[1511]) && s.b[1512]) {
            s.store_add_scaled_product_indices(175, 175, 1.0, 170, 174, 1.0);
        }

        if ((s.b[1508] && s.b[1511]) && (!s.b[1512])) {
            s.store_mul_ad_rhs(254, 754, {
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

        if ((s.b[1508] && s.b[1511]) && (!s.b[1512])) {
            s.store_div_scaled_inputs3_mixed_aiii(169, A::add_scaled_product(A::mul3(s.ad_value(753), s.ad_value(136), s.ad_value(136)), 1.0, s.ad_value(254), s.ad_value(136), (-1.0)), 1.0, 755, (-1.0), 479, 1.0, 179, 1.0);
            s.store_mul_product3_mixed_aiii(170, A::limited_exp(s.ad_value(169)), 752, 158, 141, 1.0);
            s.store_add_scaled_product_indices(175, 175, 1.0, 170, 135, 1.0);
        }

        s.b[1513] = (((p.p61 != 0.0) && ((p.p70 == 2.0) || (p.p70 == 3.0))) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0)));
        s.v[1513] = if s.b[1513] { 1.0 } else { 0.0 };

        if (s.b[1508] && s.b[1513]) {
            s.store_mul_ad_rhs(255, 757, {
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

        s.b[1514] = ((s.v[756] <= 0.0) || (s.v[255] <= 0.0));
        s.v[1514] = if s.b[1514] { 1.0 } else { 0.0 };

        if ((s.b[1508] && s.b[1513]) && s.b[1514]) {
            s.store_scalar(176, 0.0);
        }

        if ((s.b[1508] && s.b[1513]) && (!s.b[1514])) {
            s.store_div_scaled_inputs3_indices(169, 136, -1.0, 759, (-1.0), 479, 1.0, 168, 1.0);
        }

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

        if ((s.b[1508] && s.b[1513]) && (!s.b[1514])) {
            s.store_div_scaled_value_offset_denominator(170, s.ad_value(255), 1.0, s.ad_value(169), 0.001, 1.0);
            s.store_pow_ad(171, s.ad_value(169), s.ad_value(760));
            s.store_mul3_affine_lhs(172, 522, 522, -1.0, 0.0, 522);
            s.store_offset_add_ad(173, s.ad_value(758), A::abs(s.ad_value(172)), 1e-5);
        }

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

        if ((s.b[1508] && s.b[1513]) && (!s.b[1514])) {
            s.store_mul_ad_product_lhs(176, A::mul3(s.ad_value(756), s.ad_value(896), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), 174);
        }

        s.b[1515] = (s.v[128] > 0.0);
        s.v[1515] = if s.b[1515] { 1.0 } else { 0.0 };

        if (s.b[1508] && s.b[1515]) {
            s.copy_ad(476, 175);
            s.copy_ad(478, 176);
        }

        if (s.b[1508] && (!s.b[1515])) {
            s.copy_ad(475, 175);
            s.copy_ad(477, 176);
        }

        s.b[1516] = ((s.v[761] <= 0.0) || (s.v[250] <= 0.0));
        s.v[1516] = if s.b[1516] { 1.0 } else { 0.0 };

        if (s.b[1508] && s.b[1516]) {
            s.store_scalar(175, 0.0);
        }

        if (s.b[1508] && (!s.b[1516])) {
            s.store_div_scaled_inputs3_indices(169, 134, -1.0, 764, (-1.0), 479, 1.0, 168, 1.0);
        }

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

        if (s.b[1508] && (!s.b[1516])) {
            s.store_div_scaled_value_offset_denominator(170, s.ad_value(250), 1.0, s.ad_value(169), 0.001, 1.0);
            s.store_pow_ad(171, s.ad_value(169), s.ad_value(765));
        }

        s.b[1517] = (p.p61 != 0.0);
        s.v[1517] = if s.b[1517] { 1.0 } else { 0.0 };

        if ((s.b[1508] && (!s.b[1516])) && s.b[1517]) {
            s.store_mul3_affine_lhs(172, 521, 521, -1.0, 0.0, 521);
            s.store_offset_add_ad(173, s.ad_value(763), A::abs(s.ad_value(172)), 1e-5);
        }

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

        if ((s.b[1508] && (!s.b[1516])) && s.b[1517]) {
            s.store_mul_ad_product_lhs(175, A::mul3(s.ad_value(761), s.ad_value(158), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), 174);
        }

        if ((s.b[1508] && (!s.b[1516])) && (!s.b[1517])) {
            s.store_mul_ad_affine_product_lhs(175, A::mul3(s.ad_value(761), s.ad_value(158), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), -1.0, 0.0, 135);
        }

        s.b[1518] = ((p.p70 == 3.0) && (s.v[766] > 0.0));
        s.v[1518] = if s.b[1518] { 1.0 } else { 0.0 };

        s.b[1519] = (p.p61 != 0.0);
        s.v[1519] = if s.b[1519] { 1.0 } else { 0.0 };

        if ((s.b[1508] && s.b[1518]) && s.b[1519]) {
            s.store_mul_ad_rhs(253, 768, {
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

        if ((s.b[1508] && s.b[1518]) && s.b[1519]) {
            s.store_div_scaled_inputs3_mixed_aiii(169, A::add_scaled_product(A::mul3(s.ad_value(767), s.ad_value(134), s.ad_value(134)), 1.0, s.ad_value(253), s.ad_value(134), (-1.0)), 1.0, 769, (-1.0), 479, 1.0, 179, 1.0);
            s.store_mul_product3_mixed_aiii(170, A::limited_exp(s.ad_value(169)), 766, 158, 141, 1.0);
            s.store_mul3_affine_lhs(172, 521, 521, -1.0, 0.0, 521);
            s.store_offset_add_ad(173, s.ad_value(763), A::abs(s.ad_value(172)), 1e-5);
        }

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

        if ((s.b[1508] && s.b[1518]) && s.b[1519]) {
            s.store_add_scaled_product_indices(175, 175, 1.0, 170, 174, 1.0);
        }

        if ((s.b[1508] && s.b[1518]) && (!s.b[1519])) {
            s.store_mul_ad_rhs(253, 768, {
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

        if ((s.b[1508] && s.b[1518]) && (!s.b[1519])) {
            s.store_div_scaled_inputs3_mixed_aiii(169, A::add_scaled_product(A::mul3(s.ad_value(767), s.ad_value(134), s.ad_value(134)), 1.0, s.ad_value(253), s.ad_value(134), (-1.0)), 1.0, 769, (-1.0), 479, 1.0, 179, 1.0);
            s.store_mul_product3_mixed_aiii(170, A::limited_exp(s.ad_value(169)), 766, 158, 141, 1.0);
            s.store_add_scaled_product_indices(175, 175, 1.0, 170, 135, -1.0);
        }

        s.b[1520] = (((p.p61 != 0.0) && ((p.p70 == 2.0) || (p.p70 == 3.0))) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0)));
        s.v[1520] = if s.b[1520] { 1.0 } else { 0.0 };

        if (s.b[1508] && s.b[1520]) {
            s.store_mul_ad_rhs(251, 771, {
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

        s.b[1521] = ((s.v[770] <= 0.0) || (s.v[251] <= 0.0));
        s.v[1521] = if s.b[1521] { 1.0 } else { 0.0 };

        if ((s.b[1508] && s.b[1520]) && s.b[1521]) {
            s.store_scalar(176, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1508] && s.b[1520]) && (!s.b[1521])) {
            s.store_div_scaled_inputs3_indices(169, 134, -1.0, 773, (-1.0), 479, 1.0, 168, 1.0);
        }

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

        if ((s.b[1508] && s.b[1520]) && (!s.b[1521])) {
            s.store_div_scaled_value_offset_denominator(170, s.ad_value(251), 1.0, s.ad_value(169), 0.001, 1.0);
            s.store_pow_ad(171, s.ad_value(169), s.ad_value(774));
            s.store_mul3_affine_lhs(172, 521, 521, -1.0, 0.0, 521);
            s.store_offset_add_ad(173, s.ad_value(772), A::abs(s.ad_value(172)), 1e-5);
        }

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

        if ((s.b[1508] && s.b[1520]) && (!s.b[1521])) {
            s.store_mul_ad_product_lhs(176, A::mul3(s.ad_value(770), s.ad_value(896), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), 174);
        }

        s.b[1522] = (s.v[128] > 0.0);
        s.v[1522] = if s.b[1522] { 1.0 } else { 0.0 };

        if (s.b[1508] && s.b[1522]) {
            s.copy_ad(475, 175);
            s.copy_ad(477, 176);
        }

        if (s.b[1508] && (!s.b[1522])) {
            s.copy_ad(476, 175);
            s.copy_ad(478, 176);
        }

        s.b[1523] = (p.p61 != 0.0);
        s.v[1523] = if s.b[1523] { 1.0 } else { 0.0 };

        s.b[1524] = (s.v[537] > 0.0);
        s.v[1524] = if s.b[1524] { 1.0 } else { 0.0 };

        s.b[1525] = (s.v[521] < s.v[543]);
        s.v[1525] = if s.b[1525] { 1.0 } else { 0.0 };

        if ((s.b[1523] && s.b[1524]) && s.b[1525]) {
            s.store_div(168, 521, 539);
            s.store_offset_limited_exp(169, 168, (-1.0));
            s.store_add_scaled_product_right_sub(170, 542, 1.0, 541, 521, 543, 1.0);
            s.store_mul(519, 169, 170);
        }

        s.b[1526] = (s.v[521] <= s.v[546]);
        s.v[1526] = if s.b[1526] { 1.0 } else { 0.0 };

        if (((s.b[1523] && s.b[1524]) && (!s.b[1525])) && s.b[1526]) {
            s.store_div(168, 521, 539);
            s.store_div_scaled_offset_numerator(169, s.ad_value(521), 1.0, p.p1626, s.ad_value(539), 1.0);
            s.store_limited_exp_neg_input(170, 169);
            s.store_mul_add_scaled_inputs3_offset_rhs(519, 537, A::limited_exp(s.ad_value(168)), 1.0, s.ad_value(547), 1.0, s.ad_value(170), (-p.p1628), (-1.0));
        }

        if (((s.b[1523] && s.b[1524]) && (!s.b[1525])) && (!s.b[1526])) {
            s.store_add_scaled_product_right_sub(519, 545, 1.0, 544, 521, 546, 1.0);
        }

        if (s.b[1523] && (!s.b[1524])) {
            s.store_scalar(519, 0.0);
        }

        s.b[1527] = (s.v[281] > 0.0);
        s.v[1527] = if s.b[1527] { 1.0 } else { 0.0 };

        s.b[1528] = ((p.p1643 - s.v[521]) < (p.p1643 * 0.001));
        s.v[1528] = if s.b[1528] { 1.0 } else { 0.0 };

        if ((s.b[1523] && s.b[1527]) && s.b[1528]) {
            s.store_div_scaled_value_by_product(168, s.ad_value(521), -1.0, s.ad_value(180), s.ad_value(287), 1.0);
            s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));
            s.store_add_scaled_product_indices(519, 519, 1.0, 281, 169, (-p.p11));
        }

        if ((s.b[1523] && s.b[1527]) && (!s.b[1528])) {
            s.store_div_scaled_value_by_product(168, s.ad_value(521), -1.0, s.ad_value(180), s.ad_value(287), 1.0);
            s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p.p1643, A::sub_from_scalar(p.p1643, s.ad_value(521)), 1.0), (-1.0));
            s.store_add_scaled_product_indices(519, 519, 1.0, 281, 169, (-p.p11));
        }

        s.b[1529] = (s.v[283] > 0.0);
        s.v[1529] = if s.b[1529] { 1.0 } else { 0.0 };

        s.b[1530] = ((p.p1645 - s.v[521]) < (p.p1645 * 0.001));
        s.v[1530] = if s.b[1530] { 1.0 } else { 0.0 };

        if ((s.b[1523] && s.b[1529]) && s.b[1530]) {
            s.store_div_scaled_value_by_product(168, s.ad_value(521), -1.0, s.ad_value(180), s.ad_value(289), 1.0);
            s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));
            s.store_add_scaled_product_indices(519, 519, 1.0, 283, 169, (-p.p13));
        }

        if ((s.b[1523] && s.b[1529]) && (!s.b[1530])) {
            s.store_div_scaled_value_by_product(168, s.ad_value(521), -1.0, s.ad_value(180), s.ad_value(289), 1.0);
            s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p.p1645, A::sub_from_scalar(p.p1645, s.ad_value(521)), 1.0), (-1.0));
            s.store_add_scaled_product_indices(519, 519, 1.0, 283, 169, (-p.p13));
        }

        s.b[1531] = (s.v[285] > 0.0);
        s.v[1531] = if s.b[1531] { 1.0 } else { 0.0 };

        s.b[1532] = ((p.p1647 - s.v[521]) < (p.p1647 * 0.001));
        s.v[1532] = if s.b[1532] { 1.0 } else { 0.0 };

        if ((s.b[1523] && s.b[1531]) && s.b[1532]) {
            s.store_div_scaled_value_by_product(168, s.ad_value(521), -1.0, s.ad_value(180), s.ad_value(291), 1.0);
            s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));
            s.store_add_scaled_product_indices(519, 519, 1.0, 285, 169, (-(p.p3 * s.v[115])));
        }

        if ((s.b[1523] && s.b[1531]) && (!s.b[1532])) {
            s.store_div_scaled_value_by_product(168, s.ad_value(521), -1.0, s.ad_value(180), s.ad_value(291), 1.0);
            s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p.p1647, A::sub_from_scalar(p.p1647, s.ad_value(521)), 1.0), (-1.0));
            s.store_add_scaled_product_indices(519, 519, 1.0, 285, 169, (-(p.p3 * s.v[115])));
        }

        s.b[1533] = (s.v[538] > 0.0);
        s.v[1533] = if s.b[1533] { 1.0 } else { 0.0 };

        s.b[1534] = (s.v[522] < s.v[550]);
        s.v[1534] = if s.b[1534] { 1.0 } else { 0.0 };

        if ((s.b[1523] && s.b[1533]) && s.b[1534]) {
            s.store_div(168, 522, 540);
            s.store_offset_limited_exp(169, 168, (-1.0));
            s.store_add_scaled_product_right_sub(170, 549, 1.0, 548, 522, 550, 1.0);
            s.store_mul(520, 169, 170);
        }

        s.b[1535] = (s.v[522] <= s.v[553]);
        s.v[1535] = if s.b[1535] { 1.0 } else { 0.0 };

        if (((s.b[1523] && s.b[1533]) && (!s.b[1534])) && s.b[1535]) {
            s.store_div(168, 522, 540);
            s.store_div_scaled_offset_numerator(169, s.ad_value(522), 1.0, p.p1627, s.ad_value(540), 1.0);
            s.store_limited_exp_neg_input(170, 169);
            s.store_mul_add_scaled_inputs3_offset_rhs(520, 538, A::limited_exp(s.ad_value(168)), 1.0, s.ad_value(554), 1.0, s.ad_value(170), (-p.p1629), (-1.0));
        }

        if (((s.b[1523] && s.b[1533]) && (!s.b[1534])) && (!s.b[1535])) {
            s.store_add_scaled_product_right_sub(520, 552, 1.0, 551, 522, 553, 1.0);
        }

        if (s.b[1523] && (!s.b[1533])) {
            s.store_scalar(520, 0.0);
        }

        s.b[1536] = (s.v[282] > 0.0);
        s.v[1536] = if s.b[1536] { 1.0 } else { 0.0 };

        s.b[1537] = ((p.p1644 - s.v[522]) < (p.p1644 * 0.001));
        s.v[1537] = if s.b[1537] { 1.0 } else { 0.0 };

        if ((s.b[1523] && s.b[1536]) && s.b[1537]) {
            s.store_div_scaled_value_by_product(168, s.ad_value(522), -1.0, s.ad_value(180), s.ad_value(288), 1.0);
            s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));
            s.store_add_scaled_product_indices(520, 520, 1.0, 282, 169, (-p.p12));
        }

        if ((s.b[1523] && s.b[1536]) && (!s.b[1537])) {
            s.store_div_scaled_value_by_product(168, s.ad_value(522), -1.0, s.ad_value(180), s.ad_value(288), 1.0);
            s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p.p1644, A::sub_from_scalar(p.p1644, s.ad_value(522)), 1.0), (-1.0));
            s.store_add_scaled_product_indices(520, 520, 1.0, 282, 169, (-p.p12));
        }

        s.b[1538] = (s.v[284] > 0.0);
        s.v[1538] = if s.b[1538] { 1.0 } else { 0.0 };

        s.b[1539] = ((p.p1646 - s.v[522]) < (p.p1646 * 0.001));
        s.v[1539] = if s.b[1539] { 1.0 } else { 0.0 };

        if ((s.b[1523] && s.b[1538]) && s.b[1539]) {
            s.store_div_scaled_value_by_product(168, s.ad_value(522), -1.0, s.ad_value(180), s.ad_value(290), 1.0);
            s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));
            s.store_add_scaled_product_indices(520, 520, 1.0, 284, 169, (-p.p14));
        }

        if ((s.b[1523] && s.b[1538]) && (!s.b[1539])) {
            s.store_div_scaled_value_by_product(168, s.ad_value(522), -1.0, s.ad_value(180), s.ad_value(290), 1.0);
            s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p.p1646, A::sub_from_scalar(p.p1646, s.ad_value(522)), 1.0), (-1.0));
            s.store_add_scaled_product_indices(520, 520, 1.0, 284, 169, (-p.p14));
        }

        s.b[1540] = (s.v[286] > 0.0);
        s.v[1540] = if s.b[1540] { 1.0 } else { 0.0 };

        s.b[1541] = ((p.p1648 - s.v[522]) < (p.p1648 * 0.001));
        s.v[1541] = if s.b[1541] { 1.0 } else { 0.0 };

        if ((s.b[1523] && s.b[1540]) && s.b[1541]) {
            s.store_div_scaled_value_by_product(168, s.ad_value(522), -1.0, s.ad_value(180), s.ad_value(292), 1.0);
            s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));
            s.store_add_scaled_product_indices(520, 520, 1.0, 286, 169, (-(p.p3 * s.v[115])));
        }

        if ((s.b[1523] && s.b[1540]) && (!s.b[1541])) {
            s.store_div_scaled_value_by_product(168, s.ad_value(522), -1.0, s.ad_value(180), s.ad_value(292), 1.0);
            s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p.p1648, A::sub_from_scalar(p.p1648, s.ad_value(522)), 1.0), (-1.0));
            s.store_add_scaled_product_indices(520, 520, 1.0, 286, 169, (-(p.p3 * s.v[115])));
        }

        s.b[1550] = (s.v[523] > 0.0);
        s.v[1550] = if s.b[1550] { 1.0 } else { 0.0 };

        if (s.b[1523] && s.b[1550]) {
            s.store_div(1542, 521, 269);
        }

        s.b[1551] = (s.v[1542] < 0.9);
        s.v[1551] = if s.b[1551] { 1.0 } else { 0.0 };

        s.b[1552] = (p.p1602 > 0.0);
        s.v[1552] = if s.b[1552] { 1.0 } else { 0.0 };

        s.b[1553] = (s.v[521] > s.v[557]);
        s.v[1553] = if s.b[1553] { 1.0 } else { 0.0 };

        if ((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && s.b[1553]) {
            s.store_sub_from_scalar(1547, 1.0, 1542);
        }

        s.b[1554] = (p.p1596 != 1.0);
        s.v[1554] = if s.b[1554] { 1.0 } else { 0.0 };

        s.b[1555] = (p.p1596 == 0.5);
        s.v[1555] = if s.b[1555] { 1.0 } else { 0.0 };

        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && s.b[1553]) && s.b[1554]) && s.b[1555]) {
            s.store_div_from_scalar_sqrt_ad(1548, 1.0, s.ad_value(1547));
        }

        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && s.b[1553]) && s.b[1554]) && (!s.b[1555])) {
            s.store_powf(1548, 1547, (-p.p1596));
        }

        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && s.b[1553]) && s.b[1554]) {
            s.store_mul_ad_affine_product_rhs(530, 269, s.ad_value(523), A::sub_from_scalar(1.0, A::mul(s.ad_value(1547), s.ad_value(1548))), 1.0 / ((1.0 - p.p1596)), 0.0);
        }

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

        if ((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) {
            s.store_sub_from_scalar_div_indices(1547, 1.0, 557, 269);
        }

        s.b[1556] = (p.p1596 != 1.0);
        s.v[1556] = if s.b[1556] { 1.0 } else { 0.0 };

        s.b[1557] = (p.p1596 == 0.5);
        s.v[1557] = if s.b[1557] { 1.0 } else { 0.0 };

        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1556]) && s.b[1557]) {
            s.store_div_from_scalar_sqrt_ad(1548, 1.0, s.ad_value(1547));
        }

        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1556]) && (!s.b[1557])) {
            s.store_powf(1548, 1547, (-p.p1596));
        }

        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1556]) {
            s.store_mul_ad_affine_product_rhs(1549, 269, s.ad_value(523), A::sub_from_scalar(1.0, A::mul(s.ad_value(1547), s.ad_value(1548))), 1.0 / ((1.0 - p.p1596)), 0.0);
        }

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

        if ((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) {
            s.store_sub_from_scalar_ad(1547, 1.0, A::div_scaled_inputs2(s.ad_value(521), 1.0, s.ad_value(557), (-1.0), s.ad_value(558), 1.0));
        }

        s.b[1558] = (p.p1608 != 1.0);
        s.v[1558] = if s.b[1558] { 1.0 } else { 0.0 };

        s.b[1559] = (p.p1608 == 0.5);
        s.v[1559] = if s.b[1559] { 1.0 } else { 0.0 };

        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1558]) && s.b[1559]) {
            s.store_div_from_scalar_sqrt_ad(1548, 1.0, s.ad_value(1547));
        }

        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1558]) && (!s.b[1559])) {
            s.store_powf(1548, 1547, (-p.p1608));
        }

        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1558]) {
            s.store_add_product3_rhs_mixed_iia(530, 1549, 558, 523, A::sub_from_scalar(1.0, A::mul(s.ad_value(1547), s.ad_value(1548))), (p.p1602 * 1.0 / ((1.0 - p.p1608))));
        }

        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && (!s.b[1558])) {
            s.store_sub_ad_rhs(530, 1549, A::mul3_scaled_output(s.ad_value(558), s.ad_value(523), {
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
            }, p.p1602));
        }

        if (((s.b[1523] && s.b[1550]) && s.b[1551]) && (!s.b[1552])) {
            s.store_sub_from_scalar(1547, 1.0, 1542);
        }

        s.b[1560] = (p.p1596 != 1.0);
        s.v[1560] = if s.b[1560] { 1.0 } else { 0.0 };

        s.b[1561] = (p.p1596 == 0.5);
        s.v[1561] = if s.b[1561] { 1.0 } else { 0.0 };

        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && (!s.b[1552])) && s.b[1560]) && s.b[1561]) {
            s.store_div_from_scalar_sqrt_ad(1548, 1.0, s.ad_value(1547));
        }

        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && (!s.b[1552])) && s.b[1560]) && (!s.b[1561])) {
            s.store_powf(1548, 1547, (-p.p1596));
        }

        if ((((s.b[1523] && s.b[1550]) && s.b[1551]) && (!s.b[1552])) && s.b[1560]) {
            s.store_mul_ad_affine_product_rhs(530, 269, s.ad_value(523), A::sub_from_scalar(1.0, A::mul(s.ad_value(1547), s.ad_value(1548))), 1.0 / ((1.0 - p.p1596)), 0.0);
        }

    }

    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
        p: &Parameters,
    ) {
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

        s.b[1562] = (p.p1596 != 1.0);
        s.v[1562] = if s.b[1562] { 1.0 } else { 0.0 };

        s.b[1563] = (p.p1596 == 0.5);
        s.v[1563] = if s.b[1563] { 1.0 } else { 0.0 };

        if ((((s.b[1523] && s.b[1550]) && (!s.b[1551])) && s.b[1562]) && s.b[1563]) {
            s.store_scalar(1543, (1.0 / ((0.1) as f64).sqrt()));
        }

        if ((((s.b[1523] && s.b[1550]) && (!s.b[1551])) && s.b[1562]) && (!s.b[1563])) {
            s.store_scalar(1543, ((0.1) as f64).powf((-p.p1596)));
        }

        if (((s.b[1523] && s.b[1550]) && (!s.b[1551])) && s.b[1562]) {
            s.store_scalar(1544, (1.0 / (1.0 - p.p1596)));
            s.store_mul_sub_from_scalar_ad_rhs(1546, 1544, 1.0, A::scale(s.ad_value(1543), ((0.05 * p.p1596) * (1.0 + p.p1596))));
        }

        if (((s.b[1523] && s.b[1550]) && (!s.b[1551])) && (!s.b[1562])) {
            s.store_scalar(1543, 10.0);
            s.store_scalar(1546, (1.5 - ((0.1) as f64).ln()));
        }

        if ((s.b[1523] && s.b[1550]) && (!s.b[1551])) {
            s.store_mul_ad_product_rhs(1545, 1543, A::offset(s.ad_value(1542), (-1.0)), A::scale_offset(s.ad_value(1542), (5.0 * p.p1596), (((((-1.0)) * ((5.0 * p.p1596)))) + ((1.0 + p.p1596)))));
            s.store_mul_ad_product_rhs_mixed_ia(530, 269, 523, A::add(s.ad_value(1545), s.ad_value(1546)));
        }

        if (s.b[1523] && (!s.b[1550])) {
            s.store_scalar(530, 0.0);
        }

        s.b[1572] = (s.v[524] > 0.0);
        s.v[1572] = if s.b[1572] { 1.0 } else { 0.0 };

        if (s.b[1523] && s.b[1572]) {
            s.store_div(1564, 521, 270);
        }

        s.b[1573] = (s.v[1564] < 0.9);
        s.v[1573] = if s.b[1573] { 1.0 } else { 0.0 };

        s.b[1574] = (p.p1604 > 0.0);
        s.v[1574] = if s.b[1574] { 1.0 } else { 0.0 };

        s.b[1575] = (s.v[521] > s.v[559]);
        s.v[1575] = if s.b[1575] { 1.0 } else { 0.0 };

        if ((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && s.b[1575]) {
            s.store_sub_from_scalar(1569, 1.0, 1564);
        }

        s.b[1576] = (p.p1598 != 1.0);
        s.v[1576] = if s.b[1576] { 1.0 } else { 0.0 };

        s.b[1577] = (p.p1598 == 0.5);
        s.v[1577] = if s.b[1577] { 1.0 } else { 0.0 };

        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && s.b[1575]) && s.b[1576]) && s.b[1577]) {
            s.store_div_from_scalar_sqrt_ad(1570, 1.0, s.ad_value(1569));
        }

        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && s.b[1575]) && s.b[1576]) && (!s.b[1577])) {
            s.store_powf(1570, 1569, (-p.p1598));
        }

        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && s.b[1575]) && s.b[1576]) {
            s.store_mul_ad_affine_product_rhs(531, 270, s.ad_value(524), A::sub_from_scalar(1.0, A::mul(s.ad_value(1569), s.ad_value(1570))), 1.0 / ((1.0 - p.p1598)), 0.0);
        }

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

        if ((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) {
            s.store_sub_from_scalar_div_indices(1569, 1.0, 559, 270);
        }

        s.b[1578] = (p.p1598 != 1.0);
        s.v[1578] = if s.b[1578] { 1.0 } else { 0.0 };

        s.b[1579] = (p.p1598 == 0.5);
        s.v[1579] = if s.b[1579] { 1.0 } else { 0.0 };

        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1578]) && s.b[1579]) {
            s.store_div_from_scalar_sqrt_ad(1570, 1.0, s.ad_value(1569));
        }

        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1578]) && (!s.b[1579])) {
            s.store_powf(1570, 1569, (-p.p1598));
        }

        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1578]) {
            s.store_mul_ad_affine_product_rhs(1571, 270, s.ad_value(524), A::sub_from_scalar(1.0, A::mul(s.ad_value(1569), s.ad_value(1570))), 1.0 / ((1.0 - p.p1598)), 0.0);
        }

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

        if ((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) {
            s.store_sub_from_scalar_ad(1569, 1.0, A::div_scaled_inputs2(s.ad_value(521), 1.0, s.ad_value(559), (-1.0), s.ad_value(560), 1.0));
        }

        s.b[1580] = (p.p1610 != 1.0);
        s.v[1580] = if s.b[1580] { 1.0 } else { 0.0 };

        s.b[1581] = (p.p1610 == 0.5);
        s.v[1581] = if s.b[1581] { 1.0 } else { 0.0 };

        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1580]) && s.b[1581]) {
            s.store_div_from_scalar_sqrt_ad(1570, 1.0, s.ad_value(1569));
        }

        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1580]) && (!s.b[1581])) {
            s.store_powf(1570, 1569, (-p.p1610));
        }

        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1580]) {
            s.store_add_product3_rhs_mixed_iia(531, 1571, 560, 524, A::sub_from_scalar(1.0, A::mul(s.ad_value(1569), s.ad_value(1570))), (p.p1604 * 1.0 / ((1.0 - p.p1610))));
        }

        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && (!s.b[1580])) {
            s.store_sub_ad_rhs(531, 1571, A::mul3_scaled_output(s.ad_value(560), s.ad_value(524), {
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
            }, p.p1604));
        }

        if (((s.b[1523] && s.b[1572]) && s.b[1573]) && (!s.b[1574])) {
            s.store_sub_from_scalar(1569, 1.0, 1564);
        }

        s.b[1582] = (p.p1598 != 1.0);
        s.v[1582] = if s.b[1582] { 1.0 } else { 0.0 };

        s.b[1583] = (p.p1598 == 0.5);
        s.v[1583] = if s.b[1583] { 1.0 } else { 0.0 };

        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && (!s.b[1574])) && s.b[1582]) && s.b[1583]) {
            s.store_div_from_scalar_sqrt_ad(1570, 1.0, s.ad_value(1569));
        }

        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && (!s.b[1574])) && s.b[1582]) && (!s.b[1583])) {
            s.store_powf(1570, 1569, (-p.p1598));
        }

        if ((((s.b[1523] && s.b[1572]) && s.b[1573]) && (!s.b[1574])) && s.b[1582]) {
            s.store_mul_ad_affine_product_rhs(531, 270, s.ad_value(524), A::sub_from_scalar(1.0, A::mul(s.ad_value(1569), s.ad_value(1570))), 1.0 / ((1.0 - p.p1598)), 0.0);
        }

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

        s.b[1584] = (p.p1598 != 1.0);
        s.v[1584] = if s.b[1584] { 1.0 } else { 0.0 };

        s.b[1585] = (p.p1598 == 0.5);
        s.v[1585] = if s.b[1585] { 1.0 } else { 0.0 };

        if ((((s.b[1523] && s.b[1572]) && (!s.b[1573])) && s.b[1584]) && s.b[1585]) {
            s.store_scalar(1565, (1.0 / ((0.1) as f64).sqrt()));
        }

        if ((((s.b[1523] && s.b[1572]) && (!s.b[1573])) && s.b[1584]) && (!s.b[1585])) {
            s.store_scalar(1565, ((0.1) as f64).powf((-p.p1598)));
        }

        if (((s.b[1523] && s.b[1572]) && (!s.b[1573])) && s.b[1584]) {
            s.store_scalar(1566, (1.0 / (1.0 - p.p1598)));
            s.store_mul_sub_from_scalar_ad_rhs(1568, 1566, 1.0, A::scale(s.ad_value(1565), ((0.05 * p.p1598) * (1.0 + p.p1598))));
        }

        if (((s.b[1523] && s.b[1572]) && (!s.b[1573])) && (!s.b[1584])) {
            s.store_scalar(1565, 10.0);
            s.store_scalar(1568, (1.5 - ((0.1) as f64).ln()));
        }

        if ((s.b[1523] && s.b[1572]) && (!s.b[1573])) {
            s.store_mul_ad_product_rhs(1567, 1565, A::offset(s.ad_value(1564), (-1.0)), A::scale_offset(s.ad_value(1564), (5.0 * p.p1598), (((((-1.0)) * ((5.0 * p.p1598)))) + ((1.0 + p.p1598)))));
            s.store_mul_ad_product_rhs_mixed_ia(531, 270, 524, A::add(s.ad_value(1567), s.ad_value(1568)));
        }

        if (s.b[1523] && (!s.b[1572])) {
            s.store_scalar(531, 0.0);
        }

        s.b[1594] = (s.v[525] > 0.0);
        s.v[1594] = if s.b[1594] { 1.0 } else { 0.0 };

        if (s.b[1523] && s.b[1594]) {
            s.store_div(1586, 521, 271);
        }

        s.b[1595] = (s.v[1586] < 0.9);
        s.v[1595] = if s.b[1595] { 1.0 } else { 0.0 };

        s.b[1596] = (p.p1606 > 0.0);
        s.v[1596] = if s.b[1596] { 1.0 } else { 0.0 };

        s.b[1597] = (s.v[521] > s.v[561]);
        s.v[1597] = if s.b[1597] { 1.0 } else { 0.0 };

        if ((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && s.b[1597]) {
            s.store_sub_from_scalar(1591, 1.0, 1586);
        }

        s.b[1598] = (p.p1600 != 1.0);
        s.v[1598] = if s.b[1598] { 1.0 } else { 0.0 };

        s.b[1599] = (p.p1600 == 0.5);
        s.v[1599] = if s.b[1599] { 1.0 } else { 0.0 };

        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && s.b[1597]) && s.b[1598]) && s.b[1599]) {
            s.store_div_from_scalar_sqrt_ad(1592, 1.0, s.ad_value(1591));
        }

        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && s.b[1597]) && s.b[1598]) && (!s.b[1599])) {
            s.store_powf(1592, 1591, (-p.p1600));
        }

        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && s.b[1597]) && s.b[1598]) {
            s.store_mul_ad_affine_product_rhs(532, 271, s.ad_value(525), A::sub_from_scalar(1.0, A::mul(s.ad_value(1591), s.ad_value(1592))), 1.0 / ((1.0 - p.p1600)), 0.0);
        }

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

        if ((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) {
            s.store_sub_from_scalar_div_indices(1591, 1.0, 561, 271);
        }

        s.b[1600] = (p.p1600 != 1.0);
        s.v[1600] = if s.b[1600] { 1.0 } else { 0.0 };

        s.b[1601] = (p.p1600 == 0.5);
        s.v[1601] = if s.b[1601] { 1.0 } else { 0.0 };

        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1600]) && s.b[1601]) {
            s.store_div_from_scalar_sqrt_ad(1592, 1.0, s.ad_value(1591));
        }

        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1600]) && (!s.b[1601])) {
            s.store_powf(1592, 1591, (-p.p1600));
        }

        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1600]) {
            s.store_mul_ad_affine_product_rhs(1593, 271, s.ad_value(525), A::sub_from_scalar(1.0, A::mul(s.ad_value(1591), s.ad_value(1592))), 1.0 / ((1.0 - p.p1600)), 0.0);
        }

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

        if ((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) {
            s.store_sub_from_scalar_ad(1591, 1.0, A::div_scaled_inputs2(s.ad_value(521), 1.0, s.ad_value(561), (-1.0), s.ad_value(562), 1.0));
        }

        s.b[1602] = (p.p1612 != 1.0);
        s.v[1602] = if s.b[1602] { 1.0 } else { 0.0 };

        s.b[1603] = (p.p1612 == 0.5);
        s.v[1603] = if s.b[1603] { 1.0 } else { 0.0 };

        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1602]) && s.b[1603]) {
            s.store_div_from_scalar_sqrt_ad(1592, 1.0, s.ad_value(1591));
        }

        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1602]) && (!s.b[1603])) {
            s.store_powf(1592, 1591, (-p.p1612));
        }

        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1602]) {
            s.store_add_product3_rhs_mixed_iia(532, 1593, 562, 525, A::sub_from_scalar(1.0, A::mul(s.ad_value(1591), s.ad_value(1592))), (p.p1606 * 1.0 / ((1.0 - p.p1612))));
        }

        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && (!s.b[1602])) {
            s.store_sub_ad_rhs(532, 1593, A::mul3_scaled_output(s.ad_value(562), s.ad_value(525), {
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
            }, p.p1606));
        }

        if (((s.b[1523] && s.b[1594]) && s.b[1595]) && (!s.b[1596])) {
            s.store_sub_from_scalar(1591, 1.0, 1586);
        }

        s.b[1604] = (p.p1600 != 1.0);
        s.v[1604] = if s.b[1604] { 1.0 } else { 0.0 };

        s.b[1605] = (p.p1600 == 0.5);
        s.v[1605] = if s.b[1605] { 1.0 } else { 0.0 };

        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && s.b[1604]) && s.b[1605]) {
            s.store_div_from_scalar_sqrt_ad(1592, 1.0, s.ad_value(1591));
        }

        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && s.b[1604]) && (!s.b[1605])) {
            s.store_powf(1592, 1591, (-p.p1600));
        }

        if ((((s.b[1523] && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && s.b[1604]) {
            s.store_mul_ad_affine_product_rhs(532, 271, s.ad_value(525), A::sub_from_scalar(1.0, A::mul(s.ad_value(1591), s.ad_value(1592))), 1.0 / ((1.0 - p.p1600)), 0.0);
        }

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

        s.b[1606] = (p.p1600 != 1.0);
        s.v[1606] = if s.b[1606] { 1.0 } else { 0.0 };

        s.b[1607] = (p.p1600 == 0.5);
        s.v[1607] = if s.b[1607] { 1.0 } else { 0.0 };

        if ((((s.b[1523] && s.b[1594]) && (!s.b[1595])) && s.b[1606]) && s.b[1607]) {
            s.store_scalar(1587, (1.0 / ((0.1) as f64).sqrt()));
        }

        if ((((s.b[1523] && s.b[1594]) && (!s.b[1595])) && s.b[1606]) && (!s.b[1607])) {
            s.store_scalar(1587, ((0.1) as f64).powf((-p.p1600)));
        }

        if (((s.b[1523] && s.b[1594]) && (!s.b[1595])) && s.b[1606]) {
            s.store_scalar(1588, (1.0 / (1.0 - p.p1600)));
            s.store_mul_sub_from_scalar_ad_rhs(1590, 1588, 1.0, A::scale(s.ad_value(1587), ((0.05 * p.p1600) * (1.0 + p.p1600))));
        }

        if (((s.b[1523] && s.b[1594]) && (!s.b[1595])) && (!s.b[1606])) {
            s.store_scalar(1587, 10.0);
            s.store_scalar(1590, (1.5 - ((0.1) as f64).ln()));
        }

        if ((s.b[1523] && s.b[1594]) && (!s.b[1595])) {
            s.store_mul_ad_product_rhs(1589, 1587, A::offset(s.ad_value(1586), (-1.0)), A::scale_offset(s.ad_value(1586), (5.0 * p.p1600), (((((-1.0)) * ((5.0 * p.p1600)))) + ((1.0 + p.p1600)))));
            s.store_mul_ad_product_rhs_mixed_ia(532, 271, 525, A::add(s.ad_value(1589), s.ad_value(1590)));
        }

        if (s.b[1523] && (!s.b[1594])) {
            s.store_scalar(532, 0.0);
        }

        if s.b[1523] {
            s.store_add_scaled_inputs3_indices(529, 530, 1.0, 531, 1.0, 532, 1.0);
        }

        s.b[1616] = (s.v[526] > 0.0);
        s.v[1616] = if s.b[1616] { 1.0 } else { 0.0 };

        if (s.b[1523] && s.b[1616]) {
            s.store_div(1608, 522, 272);
        }

        s.b[1617] = (s.v[1608] < 0.9);
        s.v[1617] = if s.b[1617] { 1.0 } else { 0.0 };

        s.b[1618] = (p.p1603 > 0.0);
        s.v[1618] = if s.b[1618] { 1.0 } else { 0.0 };

        s.b[1619] = (s.v[522] > s.v[563]);
        s.v[1619] = if s.b[1619] { 1.0 } else { 0.0 };

        if ((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) {
            s.store_sub_from_scalar(1613, 1.0, 1608);
        }

        s.b[1620] = (p.p1597 != 1.0);
        s.v[1620] = if s.b[1620] { 1.0 } else { 0.0 };

        s.b[1621] = (p.p1597 == 0.5);
        s.v[1621] = if s.b[1621] { 1.0 } else { 0.0 };

        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) && s.b[1620]) && s.b[1621]) {
            s.store_div_from_scalar_sqrt_ad(1614, 1.0, s.ad_value(1613));
        }

    }

    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) && s.b[1620]) && (!s.b[1621])) {
            s.store_powf(1614, 1613, (-p.p1597));
        }

        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) && s.b[1620]) {
            s.store_mul_ad_affine_product_rhs(534, 272, s.ad_value(526), A::sub_from_scalar(1.0, A::mul(s.ad_value(1613), s.ad_value(1614))), 1.0 / ((1.0 - p.p1597)), 0.0);
        }

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

        if ((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) {
            s.store_sub_from_scalar_div_indices(1613, 1.0, 563, 272);
        }

        s.b[1622] = (p.p1597 != 1.0);
        s.v[1622] = if s.b[1622] { 1.0 } else { 0.0 };

        s.b[1623] = (p.p1597 == 0.5);
        s.v[1623] = if s.b[1623] { 1.0 } else { 0.0 };

        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1622]) && s.b[1623]) {
            s.store_div_from_scalar_sqrt_ad(1614, 1.0, s.ad_value(1613));
        }

        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1622]) && (!s.b[1623])) {
            s.store_powf(1614, 1613, (-p.p1597));
        }

        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1622]) {
            s.store_mul_ad_affine_product_rhs(1615, 272, s.ad_value(526), A::sub_from_scalar(1.0, A::mul(s.ad_value(1613), s.ad_value(1614))), 1.0 / ((1.0 - p.p1597)), 0.0);
        }

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

        if ((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) {
            s.store_sub_from_scalar_ad(1613, 1.0, A::div_scaled_inputs2(s.ad_value(522), 1.0, s.ad_value(563), (-1.0), s.ad_value(564), 1.0));
        }

        s.b[1624] = (p.p1609 != 1.0);
        s.v[1624] = if s.b[1624] { 1.0 } else { 0.0 };

        s.b[1625] = (p.p1609 == 0.5);
        s.v[1625] = if s.b[1625] { 1.0 } else { 0.0 };

        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1624]) && s.b[1625]) {
            s.store_div_from_scalar_sqrt_ad(1614, 1.0, s.ad_value(1613));
        }

        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1624]) && (!s.b[1625])) {
            s.store_powf(1614, 1613, (-p.p1609));
        }

        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1624]) {
            s.store_add_product3_rhs_mixed_iia(534, 1615, 564, 526, A::sub_from_scalar(1.0, A::mul(s.ad_value(1613), s.ad_value(1614))), (p.p1603 * 1.0 / ((1.0 - p.p1609))));
        }

        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && (!s.b[1624])) {
            s.store_sub_ad_rhs(534, 1615, A::mul3_scaled_output(s.ad_value(564), s.ad_value(526), {
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
            }, p.p1603));
        }

        if (((s.b[1523] && s.b[1616]) && s.b[1617]) && (!s.b[1618])) {
            s.store_sub_from_scalar(1613, 1.0, 1608);
        }

        s.b[1626] = (p.p1597 != 1.0);
        s.v[1626] = if s.b[1626] { 1.0 } else { 0.0 };

        s.b[1627] = (p.p1597 == 0.5);
        s.v[1627] = if s.b[1627] { 1.0 } else { 0.0 };

        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && (!s.b[1618])) && s.b[1626]) && s.b[1627]) {
            s.store_div_from_scalar_sqrt_ad(1614, 1.0, s.ad_value(1613));
        }

        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && (!s.b[1618])) && s.b[1626]) && (!s.b[1627])) {
            s.store_powf(1614, 1613, (-p.p1597));
        }

        if ((((s.b[1523] && s.b[1616]) && s.b[1617]) && (!s.b[1618])) && s.b[1626]) {
            s.store_mul_ad_affine_product_rhs(534, 272, s.ad_value(526), A::sub_from_scalar(1.0, A::mul(s.ad_value(1613), s.ad_value(1614))), 1.0 / ((1.0 - p.p1597)), 0.0);
        }

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

        s.b[1628] = (p.p1597 != 1.0);
        s.v[1628] = if s.b[1628] { 1.0 } else { 0.0 };

        s.b[1629] = (p.p1597 == 0.5);
        s.v[1629] = if s.b[1629] { 1.0 } else { 0.0 };

        if ((((s.b[1523] && s.b[1616]) && (!s.b[1617])) && s.b[1628]) && s.b[1629]) {
            s.store_scalar(1609, (1.0 / ((0.1) as f64).sqrt()));
        }

        if ((((s.b[1523] && s.b[1616]) && (!s.b[1617])) && s.b[1628]) && (!s.b[1629])) {
            s.store_scalar(1609, ((0.1) as f64).powf((-p.p1597)));
        }

        if (((s.b[1523] && s.b[1616]) && (!s.b[1617])) && s.b[1628]) {
            s.store_scalar(1610, (1.0 / (1.0 - p.p1597)));
            s.store_mul_sub_from_scalar_ad_rhs(1612, 1610, 1.0, A::scale(s.ad_value(1609), ((0.05 * p.p1597) * (1.0 + p.p1597))));
        }

        if (((s.b[1523] && s.b[1616]) && (!s.b[1617])) && (!s.b[1628])) {
            s.store_scalar(1609, 10.0);
            s.store_scalar(1612, (1.5 - ((0.1) as f64).ln()));
        }

        if ((s.b[1523] && s.b[1616]) && (!s.b[1617])) {
            s.store_mul_ad_product_rhs(1611, 1609, A::offset(s.ad_value(1608), (-1.0)), A::scale_offset(s.ad_value(1608), (5.0 * p.p1597), (((((-1.0)) * ((5.0 * p.p1597)))) + ((1.0 + p.p1597)))));
            s.store_mul_ad_product_rhs_mixed_ia(534, 272, 526, A::add(s.ad_value(1611), s.ad_value(1612)));
        }

        if (s.b[1523] && (!s.b[1616])) {
            s.store_scalar(534, 0.0);
        }

        s.b[1638] = (s.v[527] > 0.0);
        s.v[1638] = if s.b[1638] { 1.0 } else { 0.0 };

        if (s.b[1523] && s.b[1638]) {
            s.store_div(1630, 522, 273);
        }

        s.b[1639] = (s.v[1630] < 0.9);
        s.v[1639] = if s.b[1639] { 1.0 } else { 0.0 };

        s.b[1640] = (p.p1605 > 0.0);
        s.v[1640] = if s.b[1640] { 1.0 } else { 0.0 };

        s.b[1641] = (s.v[522] > s.v[565]);
        s.v[1641] = if s.b[1641] { 1.0 } else { 0.0 };

        if ((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && s.b[1641]) {
            s.store_sub_from_scalar(1635, 1.0, 1630);
        }

        s.b[1642] = (p.p1599 != 1.0);
        s.v[1642] = if s.b[1642] { 1.0 } else { 0.0 };

        s.b[1643] = (p.p1599 == 0.5);
        s.v[1643] = if s.b[1643] { 1.0 } else { 0.0 };

        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && s.b[1641]) && s.b[1642]) && s.b[1643]) {
            s.store_div_from_scalar_sqrt_ad(1636, 1.0, s.ad_value(1635));
        }

        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && s.b[1641]) && s.b[1642]) && (!s.b[1643])) {
            s.store_powf(1636, 1635, (-p.p1599));
        }

        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && s.b[1641]) && s.b[1642]) {
            s.store_mul_ad_affine_product_rhs(535, 273, s.ad_value(527), A::sub_from_scalar(1.0, A::mul(s.ad_value(1635), s.ad_value(1636))), 1.0 / ((1.0 - p.p1599)), 0.0);
        }

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

        if ((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) {
            s.store_sub_from_scalar_div_indices(1635, 1.0, 565, 273);
        }

        s.b[1644] = (p.p1599 != 1.0);
        s.v[1644] = if s.b[1644] { 1.0 } else { 0.0 };

        s.b[1645] = (p.p1599 == 0.5);
        s.v[1645] = if s.b[1645] { 1.0 } else { 0.0 };

        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1644]) && s.b[1645]) {
            s.store_div_from_scalar_sqrt_ad(1636, 1.0, s.ad_value(1635));
        }

        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1644]) && (!s.b[1645])) {
            s.store_powf(1636, 1635, (-p.p1599));
        }

        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1644]) {
            s.store_mul_ad_affine_product_rhs(1637, 273, s.ad_value(527), A::sub_from_scalar(1.0, A::mul(s.ad_value(1635), s.ad_value(1636))), 1.0 / ((1.0 - p.p1599)), 0.0);
        }

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

        if ((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) {
            s.store_sub_from_scalar_ad(1635, 1.0, A::div_scaled_inputs2(s.ad_value(522), 1.0, s.ad_value(565), (-1.0), s.ad_value(566), 1.0));
        }

        s.b[1646] = (p.p1611 != 1.0);
        s.v[1646] = if s.b[1646] { 1.0 } else { 0.0 };

        s.b[1647] = (p.p1611 == 0.5);
        s.v[1647] = if s.b[1647] { 1.0 } else { 0.0 };

        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1646]) && s.b[1647]) {
            s.store_div_from_scalar_sqrt_ad(1636, 1.0, s.ad_value(1635));
        }

        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1646]) && (!s.b[1647])) {
            s.store_powf(1636, 1635, (-p.p1611));
        }

        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1646]) {
            s.store_add_product3_rhs_mixed_iia(535, 1637, 566, 527, A::sub_from_scalar(1.0, A::mul(s.ad_value(1635), s.ad_value(1636))), (p.p1605 * 1.0 / ((1.0 - p.p1611))));
        }

        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && (!s.b[1646])) {
            s.store_sub_ad_rhs(535, 1637, A::mul3_scaled_output(s.ad_value(566), s.ad_value(527), {
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
            }, p.p1605));
        }

        if (((s.b[1523] && s.b[1638]) && s.b[1639]) && (!s.b[1640])) {
            s.store_sub_from_scalar(1635, 1.0, 1630);
        }

        s.b[1648] = (p.p1599 != 1.0);
        s.v[1648] = if s.b[1648] { 1.0 } else { 0.0 };

        s.b[1649] = (p.p1599 == 0.5);
        s.v[1649] = if s.b[1649] { 1.0 } else { 0.0 };

        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && (!s.b[1640])) && s.b[1648]) && s.b[1649]) {
            s.store_div_from_scalar_sqrt_ad(1636, 1.0, s.ad_value(1635));
        }

        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && (!s.b[1640])) && s.b[1648]) && (!s.b[1649])) {
            s.store_powf(1636, 1635, (-p.p1599));
        }

        if ((((s.b[1523] && s.b[1638]) && s.b[1639]) && (!s.b[1640])) && s.b[1648]) {
            s.store_mul_ad_affine_product_rhs(535, 273, s.ad_value(527), A::sub_from_scalar(1.0, A::mul(s.ad_value(1635), s.ad_value(1636))), 1.0 / ((1.0 - p.p1599)), 0.0);
        }

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

        s.b[1650] = (p.p1599 != 1.0);
        s.v[1650] = if s.b[1650] { 1.0 } else { 0.0 };

        s.b[1651] = (p.p1599 == 0.5);
        s.v[1651] = if s.b[1651] { 1.0 } else { 0.0 };

        if ((((s.b[1523] && s.b[1638]) && (!s.b[1639])) && s.b[1650]) && s.b[1651]) {
            s.store_scalar(1631, (1.0 / ((0.1) as f64).sqrt()));
        }

        if ((((s.b[1523] && s.b[1638]) && (!s.b[1639])) && s.b[1650]) && (!s.b[1651])) {
            s.store_scalar(1631, ((0.1) as f64).powf((-p.p1599)));
        }

        if (((s.b[1523] && s.b[1638]) && (!s.b[1639])) && s.b[1650]) {
            s.store_scalar(1632, (1.0 / (1.0 - p.p1599)));
            s.store_mul_sub_from_scalar_ad_rhs(1634, 1632, 1.0, A::scale(s.ad_value(1631), ((0.05 * p.p1599) * (1.0 + p.p1599))));
        }

        if (((s.b[1523] && s.b[1638]) && (!s.b[1639])) && (!s.b[1650])) {
            s.store_scalar(1631, 10.0);
            s.store_scalar(1634, (1.5 - ((0.1) as f64).ln()));
        }

        if ((s.b[1523] && s.b[1638]) && (!s.b[1639])) {
            s.store_mul_ad_product_rhs(1633, 1631, A::offset(s.ad_value(1630), (-1.0)), A::scale_offset(s.ad_value(1630), (5.0 * p.p1599), (((((-1.0)) * ((5.0 * p.p1599)))) + ((1.0 + p.p1599)))));
            s.store_mul_ad_product_rhs_mixed_ia(535, 273, 527, A::add(s.ad_value(1633), s.ad_value(1634)));
        }

        if (s.b[1523] && (!s.b[1638])) {
            s.store_scalar(535, 0.0);
        }

        s.b[1660] = (s.v[528] > 0.0);
        s.v[1660] = if s.b[1660] { 1.0 } else { 0.0 };

        if (s.b[1523] && s.b[1660]) {
            s.store_div(1652, 522, 274);
        }

        s.b[1661] = (s.v[1652] < 0.9);
        s.v[1661] = if s.b[1661] { 1.0 } else { 0.0 };

        s.b[1662] = (p.p1607 > 0.0);
        s.v[1662] = if s.b[1662] { 1.0 } else { 0.0 };

        s.b[1663] = (s.v[522] > s.v[567]);
        s.v[1663] = if s.b[1663] { 1.0 } else { 0.0 };

        if ((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && s.b[1663]) {
            s.store_sub_from_scalar(1657, 1.0, 1652);
        }

        s.b[1664] = (p.p1601 != 1.0);
        s.v[1664] = if s.b[1664] { 1.0 } else { 0.0 };

        s.b[1665] = (p.p1601 == 0.5);
        s.v[1665] = if s.b[1665] { 1.0 } else { 0.0 };

        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && s.b[1663]) && s.b[1664]) && s.b[1665]) {
            s.store_div_from_scalar_sqrt_ad(1658, 1.0, s.ad_value(1657));
        }

        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && s.b[1663]) && s.b[1664]) && (!s.b[1665])) {
            s.store_powf(1658, 1657, (-p.p1601));
        }

        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && s.b[1663]) && s.b[1664]) {
            s.store_mul_ad_affine_product_rhs(536, 274, s.ad_value(528), A::sub_from_scalar(1.0, A::mul(s.ad_value(1657), s.ad_value(1658))), 1.0 / ((1.0 - p.p1601)), 0.0);
        }

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

        if ((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) {
            s.store_sub_from_scalar_div_indices(1657, 1.0, 567, 274);
        }

        s.b[1666] = (p.p1601 != 1.0);
        s.v[1666] = if s.b[1666] { 1.0 } else { 0.0 };

        s.b[1667] = (p.p1601 == 0.5);
        s.v[1667] = if s.b[1667] { 1.0 } else { 0.0 };

        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1666]) && s.b[1667]) {
            s.store_div_from_scalar_sqrt_ad(1658, 1.0, s.ad_value(1657));
        }

        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1666]) && (!s.b[1667])) {
            s.store_powf(1658, 1657, (-p.p1601));
        }

        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1666]) {
            s.store_mul_ad_affine_product_rhs(1659, 274, s.ad_value(528), A::sub_from_scalar(1.0, A::mul(s.ad_value(1657), s.ad_value(1658))), 1.0 / ((1.0 - p.p1601)), 0.0);
        }

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

        if ((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) {
            s.store_sub_from_scalar_ad(1657, 1.0, A::div_scaled_inputs2(s.ad_value(522), 1.0, s.ad_value(567), (-1.0), s.ad_value(568), 1.0));
        }

        s.b[1668] = (p.p1613 != 1.0);
        s.v[1668] = if s.b[1668] { 1.0 } else { 0.0 };

        s.b[1669] = (p.p1613 == 0.5);
        s.v[1669] = if s.b[1669] { 1.0 } else { 0.0 };

        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1668]) && s.b[1669]) {
            s.store_div_from_scalar_sqrt_ad(1658, 1.0, s.ad_value(1657));
        }

        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1668]) && (!s.b[1669])) {
            s.store_powf(1658, 1657, (-p.p1613));
        }

        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1668]) {
            s.store_add_product3_rhs_mixed_iia(536, 1659, 568, 528, A::sub_from_scalar(1.0, A::mul(s.ad_value(1657), s.ad_value(1658))), (p.p1607 * 1.0 / ((1.0 - p.p1613))));
        }

    }

    pub(super) fn stamp_transient_block_27(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && (!s.b[1668])) {
            s.store_sub_ad_rhs(536, 1659, A::mul3_scaled_output(s.ad_value(568), s.ad_value(528), {
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
            }, p.p1607));
        }

        if (((s.b[1523] && s.b[1660]) && s.b[1661]) && (!s.b[1662])) {
            s.store_sub_from_scalar(1657, 1.0, 1652);
        }

        s.b[1670] = (p.p1601 != 1.0);
        s.v[1670] = if s.b[1670] { 1.0 } else { 0.0 };

        s.b[1671] = (p.p1601 == 0.5);
        s.v[1671] = if s.b[1671] { 1.0 } else { 0.0 };

        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && (!s.b[1662])) && s.b[1670]) && s.b[1671]) {
            s.store_div_from_scalar_sqrt_ad(1658, 1.0, s.ad_value(1657));
        }

        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && (!s.b[1662])) && s.b[1670]) && (!s.b[1671])) {
            s.store_powf(1658, 1657, (-p.p1601));
        }

        if ((((s.b[1523] && s.b[1660]) && s.b[1661]) && (!s.b[1662])) && s.b[1670]) {
            s.store_mul_ad_affine_product_rhs(536, 274, s.ad_value(528), A::sub_from_scalar(1.0, A::mul(s.ad_value(1657), s.ad_value(1658))), 1.0 / ((1.0 - p.p1601)), 0.0);
        }

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

        s.b[1672] = (p.p1601 != 1.0);
        s.v[1672] = if s.b[1672] { 1.0 } else { 0.0 };

        s.b[1673] = (p.p1601 == 0.5);
        s.v[1673] = if s.b[1673] { 1.0 } else { 0.0 };

        if ((((s.b[1523] && s.b[1660]) && (!s.b[1661])) && s.b[1672]) && s.b[1673]) {
            s.store_scalar(1653, (1.0 / ((0.1) as f64).sqrt()));
        }

        if ((((s.b[1523] && s.b[1660]) && (!s.b[1661])) && s.b[1672]) && (!s.b[1673])) {
            s.store_scalar(1653, ((0.1) as f64).powf((-p.p1601)));
        }

        if (((s.b[1523] && s.b[1660]) && (!s.b[1661])) && s.b[1672]) {
            s.store_scalar(1654, (1.0 / (1.0 - p.p1601)));
            s.store_mul_sub_from_scalar_ad_rhs(1656, 1654, 1.0, A::scale(s.ad_value(1653), ((0.05 * p.p1601) * (1.0 + p.p1601))));
        }

        if (((s.b[1523] && s.b[1660]) && (!s.b[1661])) && (!s.b[1672])) {
            s.store_scalar(1653, 10.0);
            s.store_scalar(1656, (1.5 - ((0.1) as f64).ln()));
        }

        if ((s.b[1523] && s.b[1660]) && (!s.b[1661])) {
            s.store_mul_ad_product_rhs(1655, 1653, A::offset(s.ad_value(1652), (-1.0)), A::scale_offset(s.ad_value(1652), (5.0 * p.p1601), (((((-1.0)) * ((5.0 * p.p1601)))) + ((1.0 + p.p1601)))));
            s.store_mul_ad_product_rhs_mixed_ia(536, 274, 528, A::add(s.ad_value(1655), s.ad_value(1656)));
        }

        if (s.b[1523] && (!s.b[1660])) {
            s.store_scalar(536, 0.0);
        }

        if s.b[1523] {
            s.store_add_scaled_inputs3_indices(533, 534, 1.0, 535, 1.0, 536, 1.0);
        }

        s.store_add_scaled_inputs(507, 529, 1.0, 521, s.v[515]);

        s.store_add_scaled_inputs(508, 533, 1.0, 522, s.v[516]);

        s.store_mul_ad_product_rhs_mixed_ia(509, 517, 114, A::voltage(ctx, nodes, Some(3), Some(10)));

        s.b[1674] = (p.p61 != 0.0);
        s.v[1674] = if s.b[1674] { 1.0 } else { 0.0 };

        if s.b[1674] {
            s.store_mul_voltage_ad(170, s.ad_value(114), ctx, nodes, Some(10), Some(3));
            s.store_add_scaled_inputs4_offset_indices(171, 170, 1.0, 167, (-1.0), 146, 0.5, 166, 1.0, (-p.p1529));
            s.store_offset(168, 171, 0.02);
            s.store_scaled_add_sqrt_square_offset_rhs(512, 168, 168, (4.0 * 0.02), 0.5);
            s.store_sub_ad_rhs(509, 509, A::mul3_scaled_output(s.ad_value(156), s.ad_value(650), A::add_scaled_inputs_product(s.ad_value(171), 1.0, s.ad_value(512), (-1.0), s.ad_value(653), A::offset(A::sqrt(A::offset(A::div_scaled_inputs(s.ad_value(512), 4.0, s.ad_value(653), 1.0), 1.0)), (-1.0)), 0.5), s.v[115]));
        }

        s.store_mul_add_ad_rhs(169, 126, s.ad_value(865), A::mul3(s.ad_value(866), s.ad_value(126), s.ad_value(126)));

        s.store_mul_ad_affine_product_lhs(556, A::sub_scaled_inputs(s.ad_value(153), 1.0, s.ad_value(875), 2.0), s.ad_value(555), (p.p92 * p.p3), 0.0, 169);

        s.store_div_scaled_product3_indices(168, 415, 372, 158, 1.0, 153, 1.0);

        s.b[1675] = ((p.p73 != 0.0) && (s.v[873] != 0.0));
        s.v[1675] = if s.b[1675] { 1.0 } else { 0.0 };

        if s.b[1675] {
            s.store_div_scaled_product3_mixed_iiia(572, 183, 123, 205, 1.0, A::mul3(s.ad_value(411), s.ad_value(209), s.ad_value(188)), 1.0);
            s.store_mul_add_scaled_inputs_rhs(569, 873, s.ad_value(572), s.v[115], A::mul3(s.ad_value(874), s.ad_value(179), s.ad_value(168)), s.v[115]);
        }

        s.b[1676] = (p.p73 == 2.0);
        s.v[1676] = if s.b[1676] { 1.0 } else { 0.0 };

        if s.b[1676] {
            s.store_div_scaled_inputs_mixed_ia(570, 569, 1e-9, A::mul3(s.ad_value(163), s.ad_value(158), s.ad_value(153)), 1.0);
        }

        s.store_scale(476, 476, s.v[115]);

        s.store_scale(475, 475, s.v[115]);

        s.store_scale(478, 478, s.v[115]);

        s.store_scale(477, 477, s.v[115]);

        s.store_scale(471, 471, s.v[115]);

        s.store_scale(470, 470, s.v[115]);

        s.store_scale(480, 480, s.v[115]);

        s.store_scale(481, 481, s.v[115]);

        s.store_scale(461, 461, s.v[115]);

        s.store_scale(469, 469, s.v[115]);

        s.store_scale(556, 556, s.v[115]);

        s.b[1677] = (p.p61 == 0.0);
        s.v[1677] = if s.b[1677] { 1.0 } else { 0.0 };

        if s.b[1677] {
            s.store_mul_add_lhs(464, 461, 469, 186);
            s.store_mul_add_lhs(465, 461, 469, 187);
        }

        s.store_div_scaled_inputs_indices(579, 428, 2.0, 415, 1.0);

        s.b[1678] = (((p.p1682 > 0.0) || (p.p1683 > 0.0)) || (p.p1684 > 0.0));
        s.v[1678] = if s.b[1678] { 1.0 } else { 0.0 };

        if s.b[1678] {
            s.store_offset(580, 153, (-(2.0 * p.p1687)));
        }

        s.b[1679] = (s.v[580] <= 0.0);
        s.v[1679] = if s.b[1679] { 1.0 } else { 0.0 };

        if (s.b[1678] && s.b[1679]) {
            s.copy_ad(580, 153);
        }

        s.b[1680] = ((p.p79 == 1.0) || (p.p79 == 0.0));
        s.v[1680] = if s.b[1680] { 1.0 } else { 0.0 };

        if (s.b[1678] && s.b[1680]) {
            s.store_square(581, 580);
        }

        s.b[1681] = (p.p1681 > 0.0);
        s.v[1681] = if s.b[1681] { 1.0 } else { 0.0 };

        if ((s.b[1678] && s.b[1680]) && s.b[1681]) {
            s.store_div_scaled_offset_numerator(168, s.ad_value(202), 1.0 / (s.v[578]), p.p1681, s.ad_value(579), 1.0);
        }

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

        if ((s.b[1678] && s.b[1680]) && (!s.b[1681])) {
            s.store_scalar(582, 0.0);
        }

        s.b[1682] = (p.p79 == 1.0);
        s.v[1682] = if s.b[1682] { 1.0 } else { 0.0 };

        if ((s.b[1678] && s.b[1680]) && s.b[1682]) {
            s.store_div(169, 400, 576);
            s.store_offset_pow_ad(170, s.ad_value(169), s.ad_value(575), 1.0);
            s.store_div(171, 574, 170);
            s.store_scale(172, 171, 1.0 / (p.p1682));
            s.store_scaled_add_offset_sqrt_square_offset(174, 172, 1.0, (-1.0), ((0.25 * p.p1688) * p.p1688), 0.5);
            s.store_scale(573, 174, p.p1682);
        }

        if ((s.b[1678] && s.b[1680]) && (!s.b[1682])) {
            s.store_scalar(573, p.p1682);
        }

        if (s.b[1678] && s.b[1680]) {
            s.store_mul_ad_affine_product_lhs(169, s.ad_value(179), A::abs(s.ad_value(124)), ((1.60219e-19 * 1.60219e-19) * 1.60219e-19), 0.0, 415);
            s.store_scaled_mul(170, 372, 581, 10000000000.0);
            s.store_scaled_mul(583, 372, 392, 6.241457005723417e18);
            s.store_scaled_mul(584, 372, 393, 6.241457005723417e18);
            s.store_mul_add_scaled_inputs_rhs(585, 179, s.ad_value(372), 1.0 / (1.60219e-19), s.ad_value(669), 1.0 / (1.60219e-19));
        }

        if (s.b[1678] && s.b[1680]) {
            s.store_mul_ad_rhs(171, 573, {
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

        if (s.b[1678] && s.b[1680]) {
            s.store_scaled_sub(172, 583, 584, p.p1683);
            s.store_scaled_sub_ad(174, A::square(s.ad_value(583)), A::square(s.ad_value(584)), (0.5 * p.p1684));
            s.store_mul3_affine_lhs(175, 179, 124, 1.60219e-19, 0.0, 124);
            s.store_scaled_mul(176, 581, 158, (10000000000.0 * s.v[115]));
            s.store_add_scaled_inputs_product_indices(177, 573, 1.0, 584, p.p1683, 584, 584, p.p1684);
            s.store_square_ad(178, A::add(s.ad_value(584), s.ad_value(585)));
            s.store_add_scaled_product(586, A::div_scaled_product3_by_product(s.ad_value(175), s.ad_value(582), s.ad_value(177), 1.0, s.ad_value(176), s.ad_value(178), 1.0), 1.0, A::div(s.ad_value(169), s.ad_value(170)), A::add_scaled_inputs3(s.ad_value(171), 1.0, s.ad_value(172), 1.0, s.ad_value(174), 1.0), 1.0);
            s.store_scaled_mul(340, 573, 179, 1.60219e-19);
            s.store_mul_product3_indices(341, 585, 158, 580, 585, (s.v[115] * 10000000000.0));
            s.store_mul_ad_product_lhs_mixed_ai(587, A::div(s.ad_value(340), s.ad_value(341)), 124, 124);
            s.store_add(169, 587, 586);
        }

        s.b[1684] = (p.p79 == 2.0);
        s.v[1684] = if s.b[1684] { 1.0 } else { 0.0 };

        if ((s.b[1678] && (!s.b[1680])) && s.b[1684]) {
            s.store_div(169, 400, 576);
            s.store_offset_pow_ad(170, s.ad_value(169), s.ad_value(575), 1.0);
            s.store_div(171, 574, 170);
            s.store_scale(172, 171, 1.0 / (p.p1682));
            s.store_scaled_add_offset_sqrt_square_offset(174, 172, 1.0, (-1.0), ((0.25 * p.p1688) * p.p1688), 0.5);
            s.store_scale(573, 174, p.p1682);
            s.store_div_scaled_inputs_indices(589, 179, 2.0, 217, 1.0);
            s.store_offset_mul(169, 589, 402, 1.0);
            s.store_offset_scaled(170, 402, p.p1685, 1.0);
        }

        s.b[1685] = ((s.v[169] > 0.0) && (s.v[170] > 0.0));
        s.v[1685] = if s.b[1685] { 1.0 } else { 0.0 };

        if (((s.b[1678] && (!s.b[1680])) && s.b[1684]) && s.b[1685]) {
            s.store_mul_offset_rhs_ad(171, {
                if (!(((s.v[392] + 0.5) / (s.v[393] + 0.5)) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((s.v[392] + 0.5) / (s.v[393] + 0.5)) > 1e-38) {
                            A::ln(A::div_scaled_offset_numerator(s.ad_value(392), 1.0, 0.5, A::offset(s.ad_value(393), 0.5), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, A::add(s.ad_value(392), s.ad_value(393)), 1.0);
        }

        if (((s.b[1678] && (!s.b[1680])) && s.b[1684]) && s.b[1685]) {
            s.store_scaled_sub(172, 392, 393, 2.0);
        }

        s.b[1686] = (p.p72 == 0.0);
        s.v[1686] = if s.b[1686] { 1.0 } else { 0.0 };

        s.b[1687] = (p.p72 == 1.0);
        s.v[1687] = if s.b[1687] { 1.0 } else { 0.0 };

        if s.b[1686] {
            s.store_mul(168, 415, 592);
            s.store_add_scaled_square_product_indices(169, 153, 1.0, 168, 197, 1.0);
        }

        if (s.b[1687] && (!s.b[1686])) {
            s.store_div(168, 399, 217);
            s.store_square(168, 168);
            s.store_scaled_offset_ad(597, A::mul_scaled_lhs(s.ad_value(168), p.p1709, s.ad_value(153)), 1.0, p.p1708);
            s.store_scaled_offset_ad(598, A::mul_scaled_lhs(s.ad_value(168), p.p1711, s.ad_value(153)), 1.0, p.p1710);
            s.store_scaled_offset_ad(599, A::mul_scaled_lhs(s.ad_value(168), p.p1713, s.ad_value(153)), 1.0, p.p1712);
            s.store_scaled_offset_ad(600, A::mul_scaled_lhs(s.ad_value(168), p.p1715, s.ad_value(153)), 1.0, p.p1714);
            s.store_scaled_mul(169, 597, 597, 3.0);
            s.store_scaled_mul(170, 598, 598, 7.5);
            s.store_scale(171, 599, 2.5298);
            s.store_mul_sub_from_scalar_rhs_ad(601, A::div(s.ad_value(393), s.ad_value(392)), 1.0, A::div(s.ad_value(390), s.ad_value(210)));
            s.store_mul_square_lhs(604, 209, 209);
            s.store_div_add_scaled_inputs_rhs_indices(602, 339, 339, 1.0, 399, 1.0);
            s.store_div_ad_rhs(172, 236, A::add_scaled_product(s.ad_value(181), 2.0, A::max_from_scalar(0.0, s.ad_value(237)), s.ad_value(392), 1.0));
            s.store_limited_exp_neg_input(616, 172);
        }

        s.b[1688] = (p.p61 == 2.0);
        s.v[1688] = if s.b[1688] { 1.0 } else { 0.0 };

        if ((s.b[1687] && (!s.b[1686])) && s.b[1688]) {
            if (!(s.v[293] < ((-10000.0) * 1e-6))) {
                s.store_scaled_add_sqrt_square_offset_rhs(172, 293, 293, ((4.0 * 1e-6) * 1e-6), 0.5);
            } else {
                if (s.v[293] < ((-10000.0) * 1e-6)) {
                    s.store_div_from_scalar(172, ((-1e-6) * 1e-6), 293);
                } else {
                    s.store_scalar(172, 0.0);
                }
            }
        }

        if ((s.b[1687] && (!s.b[1686])) && s.b[1688]) {
            s.store_div_ad_rhs(174, 172, A::add_scaled_product(s.ad_value(181), 2.0, A::max_from_scalar(0.0, s.ad_value(238)), s.ad_value(392), 1.0));
            s.store_sub_ad(175, A::sqrt(A::sub(s.ad_value(689), s.ad_value(370))), A::sqrt(s.ad_value(689)));
            s.store_limited_exp_ad(617, A::mul_scaled_lhs(s.ad_value(174), -1.0, s.ad_value(175)));
        }

    }

    pub(super) fn stamp_transient_block_28(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[1687] && (!s.b[1686])) && (!s.b[1688])) {
            s.store_scalar(617, 1.0);
        }

        if (s.b[1687] && (!s.b[1686])) {
            s.store_add_scaled_product_indices(615, 401, s.v[420], 407, 392, s.v[420]);
            s.store_pow_ad(172, A::scaled_offset(A::abs(A::div(s.ad_value(392), s.ad_value(406))), 1.0, 0.5), s.ad_value(317));
        }

        s.b[1689] = (p.p61 != 0.0);
        s.v[1689] = if s.b[1689] { 1.0 } else { 0.0 };

        if ((s.b[1687] && (!s.b[1686])) && s.b[1689]) {
            s.store_add_scaled_product(174, A::div(s.ad_value(820), s.ad_value(172)), 1.0, A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(821), s.ad_value(370), 1.0), A::pow(A::abs(s.ad_value(615)), s.ad_value(822)), 1.0);
        }

        if ((s.b[1687] && (!s.b[1686])) && (!s.b[1689])) {
            s.store_add_scaled_product_mixed_aia(174, A::div(s.ad_value(820), s.ad_value(172)), 1.0, 819, A::pow(A::abs(s.ad_value(615)), s.ad_value(822)), 1.0);
        }

        if (s.b[1687] && (!s.b[1686])) {
            s.store_offset(618, 174, 1.0);
            s.store_scaled_add_offset_sqrt_square_offset(618, 618, 1.0, (-1.0), ((0.25 * p.p604) * p.p604), 0.5);
            s.store_scale(618, 618, 1.0 / (p.p24));
            s.store_scalar(619, (1.0 + (0.25 * p.p453)));
            s.store_div_add_scaled_inputs_rhs_indices(612, 339, 339, 1.0, 392, 1.0);
            s.store_mul_sub_from_scalar_lhs(172, 2.0, 612, 181);
            s.store_add(613, 392, 172);
        }

        s.b[1690] = (p.p64 == 0.0);
        s.v[1690] = if s.b[1690] { 1.0 } else { 0.0 };

        s.b[1691] = (p.p64 == 1.0);
        s.v[1691] = if s.b[1691] { 1.0 } else { 0.0 };

        s.b[1692] = (p.p64 == 2.0);
        s.v[1692] = if s.b[1692] { 1.0 } else { 0.0 };

        if ((s.b[1687] && (!s.b[1686])) && s.b[1690]) {
            s.store_offset_mul(172, 711, 392, 1.0);
            s.store_div_from_scalar(174, 1.0, 172);
            s.store_scaled_add_sqrt_square_offset_rhs(175, 174, 174, 0.01, 0.5);
            s.store_mul_ad_product_lhs_mixed_ia(614, 194, A::offset(A::mul(s.ad_value(709), s.ad_value(175)), p.p908), 189);
            s.store_offset_mul_ad(620, A::div_scaled_product_by_product(s.ad_value(183), s.ad_value(613), s.v[115], s.ad_value(618), s.ad_value(619), 1.0), s.ad_value(614), 1.0);
        }

        if ((s.b[1687] && (!s.b[1686])) && (s.b[1691] && (!s.b[1690]))) {
            s.store_scalar(620, 1.0);
        }

        if ((s.b[1687] && (!s.b[1686])) && (s.b[1692] && (!(s.b[1690] || s.b[1691])))) {
            s.store_offset_mul(172, 711, 392, 1.0);
            s.store_div_from_scalar(174, 1.0, 172);
            s.store_scaled_add_sqrt_square_offset_rhs(175, 174, 174, 0.01, 0.5);
            s.store_mul_offset_ad_lhs(614, A::mul(s.ad_value(709), s.ad_value(175)), p.p908, 189);
            s.store_mul_add_scaled_inputs3_offset_rhs(614, 194, s.ad_value(190), 1.0, s.ad_value(191), 1.0, s.ad_value(614), 1.0, 0.0);
            s.store_offset_mul_ad(620, A::div_scaled_product_by_product(s.ad_value(183), s.ad_value(613), s.v[115], s.ad_value(618), s.ad_value(619), 1.0), s.ad_value(614), 1.0);
        }

        if (s.b[1687] && (!s.b[1686])) {
            s.store_div_scaled_product_mixed_aia(603, A::mul3_scaled_output(s.ad_value(183), s.ad_value(392), s.ad_value(616), s.v[115]), 617, 1.0, A::mul3(s.ad_value(618), s.ad_value(619), s.ad_value(620)), 1.0);
            s.store_offset(172, 601, 1.0);
            s.store_sub_from_scalar(174, 1.0, 601);
            s.store_mul_div_scaled_inputs_indices(175, 181, 602, 2.0, 392, 1.0);
            s.store_add(176, 172, 175);
            s.store_square(605, 174);
            s.store_mul(606, 605, 174);
            s.store_mul(607, 606, 174);
            s.store_square(608, 176);
            s.store_mul(609, 608, 176);
            s.store_mul(610, 609, 176);
            s.store_mul(611, 610, 176);
            s.store_scale(621, 172, 0.5);
            s.store_div_scaled_inputs_indices(622, 605, 1.0, 176, 6.0);
            s.store_mul_div_scaled_inputs_mixed_aii(623, A::add(s.ad_value(621), s.ad_value(622)), 205, 1.0, 209, 1.0);
            s.store_div(624, 172, 608);
            s.store_div_scaled_product_left_ad(625, A::add_scaled_inputs(s.ad_value(172), 6.0, s.ad_value(175), 1.0), 605, 1.0, 610, 15.0);
            s.store_div_scaled_inputs_indices(626, 607, 1.0, 611, 9.0);
            s.store_mul_ad_affine_product_rhs(627, 205, s.ad_value(604), A::add_scaled_inputs3(s.ad_value(624), 1.0, s.ad_value(625), (-1.0), s.ad_value(626), 1.0), 1.0 / (6.0), 0.0);
            s.store_div(628, 174, 176);
            s.store_div_scaled_inputs_indices(629, 606, 1.0, 609, 3.0);
            s.store_mul_ad_affine_product_rhs(630, 205, s.ad_value(209), A::sub(s.ad_value(628), s.ad_value(629)), 1.0 / (6.0), 0.0);
            s.store_div_scaled_product_denominator_ad(631, 171, 630, 1.0, A::sqrt(A::mul(s.ad_value(623), s.ad_value(627))), 1.0);
        }

        s.b[1693] = (s.v[631] > 1.0);
        s.v[1693] = if s.b[1693] { 1.0 } else { 0.0 };

        if ((s.b[1687] && (!s.b[1686])) && s.b[1693]) {
            s.store_scalar(631, 1.0);
        }

        s.b[1694] = (s.v[631] < 0.0);
        s.v[1694] = if s.b[1694] { 1.0 } else { 0.0 };

        if (((s.b[1687] && (!s.b[1686])) && (!s.b[1693])) && s.b[1694]) {
            s.store_scalar(631, 0.0);
        }

        if (s.b[1687] && (!s.b[1686])) {
            s.store_offset_mul_ad(177, A::div_scaled_product_offset_denominator(s.ad_value(600), s.ad_value(600), 1.0, s.ad_value(399), p.p1716, 1.0), A::div(s.ad_value(390), s.ad_value(210)), 1.0);
            s.store_mul_div_scaled_inputs_mixed_aii(623, A::add_scaled_products(s.ad_value(177), s.ad_value(621), 1.0, s.ad_value(169), s.ad_value(622), 1.0), 205, 1.0, 209, 1.0);
            s.store_mul_product3_mixed_aiii(627, A::add_scaled_inputs3(s.ad_value(624), 1.0, s.ad_value(625), (-1.0), s.ad_value(626), 1.0), 205, 604, 170, 1.0 / (6.0));
            s.store_div_scaled_product_left_ad(632, A::mul3_scaled_output(A::sqrt(A::div(s.ad_value(627), s.ad_value(623))), s.ad_value(372), s.ad_value(159), s.v[115]), 156, 1.0, 603, 1.0);
        }

        s.b[1695] = (s.v[128] > 0.0);
        s.v[1695] = if s.b[1695] { 1.0 } else { 0.0 };

        s.b[1696] = (p.p73 == 2.0);
        s.v[1696] = if s.b[1696] { 1.0 } else { 0.0 };

        s.b[1698] = (s.v[128] > 0.0);
        s.v[1698] = if s.b[1698] { 1.0 } else { 0.0 };

        s.b[1699] = (p.p61 != 0.0);
        s.v[1699] = if s.b[1699] { 1.0 } else { 0.0 };

        s.b[1700] = ((p.p70 == 2.0) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0)));
        s.v[1700] = if s.b[1700] { 1.0 } else { 0.0 };

        s.b[1701] = (p.p61 != 0.0);
        s.v[1701] = if s.b[1701] { 1.0 } else { 0.0 };

        s.b[1702] = ((p.p70 == 2.0) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0)));
        s.v[1702] = if s.b[1702] { 1.0 } else { 0.0 };

        s.b[1703] = (p.p61 == 0.0);
        s.v[1703] = if s.b[1703] { 1.0 } else { 0.0 };

        s.b[1704] = (p.p61 != 0.0);
        s.v[1704] = if s.b[1704] { 1.0 } else { 0.0 };

        s.b[1705] = (p.p76 != 2.0);
        s.v[1705] = if s.b[1705] { 1.0 } else { 0.0 };

        s.b[1706] = (p.p65 == 1.0);
        s.v[1706] = if s.b[1706] { 1.0 } else { 0.0 };

        s.b[1707] = (p.p78 == 1.0);
        s.v[1707] = if s.b[1707] { 1.0 } else { 0.0 };

        s.b[1708] = (p.p65 == 1.0);
        s.v[1708] = if s.b[1708] { 1.0 } else { 0.0 };

        s.b[1709] = (p.p78 == 1.0);
        s.v[1709] = if s.b[1709] { 1.0 } else { 0.0 };

        s.b[1710] = (p.p61 != 0.0);
        s.v[1710] = if s.b[1710] { 1.0 } else { 0.0 };

        s.b[1711] = (p.p64 == 1.0);
        s.v[1711] = if s.b[1711] { 1.0 } else { 0.0 };

        s.b[1712] = (p.p1910 > 0.0);
        s.v[1712] = if s.b[1712] { 1.0 } else { 0.0 };

        if (s.b[1711] && s.b[1712]) {
            if (!(((1.0 + (p.p1912 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                s.store_scaled_add_sqrt_square_offset_ad(1039, A::scale_offset(s.ad_value(232), p.p1912, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), 0.5);
            } else {
                if (((1.0 + (p.p1912 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                    s.store_div_from_scalar_offset_scaled_input(1039, ((-0.001) * 0.001), 232, p.p1912, ((1.0) + ((-1e-6))));
                } else {
                    s.store_scalar(1039, 0.0);
                }
            }
        }

        s.b[1713] = (p.p75 != 0.0);
        s.v[1713] = if s.b[1713] { 1.0 } else { 0.0 };

        if ((s.b[1711] && s.b[1712]) && s.b[1713]) {
            s.store_offset_add_scaled_inputs(1044, A::scale_offset(s.ad_value(232), (-p.p1913), (((-(-p.p1904))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), (-p.p1913), (((-(-p.p1904))) + ((-1e-6)))), (-((4.0 * (-p.p1904)) * 1e-6))), 0.5, (((-p.p1904)) + (p.p1904)));
        }

        if ((s.b[1711] && s.b[1712]) && (!s.b[1713])) {
            s.store_scale_ad(1044, {
                if (!(((1.0 + ((-p.p1913) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + ((-p.p1913) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1904);
        }

        if (s.b[1711] && s.b[1712]) {
            s.store_offset(168, 392, (-p.p1906));
            s.store_scaled_add_offset_sqrt_square_offset(168, 168, 0.1, (-0.1), ((0.25 * 2.0) * 2.0), 0.5);
            s.store_div_scaled_value_offset_denominator(169, s.ad_value(168), (10.0 * p.p1907), s.ad_value(168), (10.0 * p.p1907), 1.0);
            s.store_mul_scale_offset_rhs(1045, 1044, 169, p.p1905, 1.0);
        }

        if (s.b[1711] && s.b[1712]) {
            if (!(s.v[1045] < ((-10000.0) * 10.0))) {
                s.store_scaled_add_sqrt_square_offset_rhs(1045, 1045, 1045, ((4.0 * 10.0) * 10.0), 0.5);
            } else {
                if (s.v[1045] < ((-10000.0) * 10.0)) {
                    s.store_div_from_scalar(1045, ((-10.0) * 10.0), 1045);
                } else {
                    s.store_scalar(1045, 0.0);
                }
            }
        }

        if (s.b[1711] && s.b[1712]) {
            s.store_scaled_mul(170, 158, 1045, (s.v[115] * 1.60219e-19));
            s.store_abs_voltage(174, ctx, nodes, Some(9), Some(7));
        }

        s.b[1714] = (p.p1917 == 0.0);
        s.v[1714] = if s.b[1714] { 1.0 } else { 0.0 };

        if ((s.b[1711] && s.b[1712]) && s.b[1714]) {
            s.store_scalar(171, 1.0);
        }

        if ((s.b[1711] && s.b[1712]) && (!s.b[1714])) {
            s.store_scaled_add_sqrt_square_offset_ad(171, A::offset(s.ad_value(174), (-p.p1916)), ((0.25 * 0.5) * 0.5), 0.5);
            s.store_offset_scaled(171, 171, p.p1917, 1.0);
        }

        if (s.b[1711] && s.b[1712]) {
            s.store_scaled_mul(1047, 170, 171, p.p1903);
            s.store_scaled_mul(172, 1039, 189, p.p1910);
            s.store_mul(1048, 1047, 172);
        }

        if (s.b[1711] && s.b[1712]) {
            let assign34510_ad_e57399: A = A::powf(s.ad_value(174), (4.0 - p.p1908));
            s.store_div_ad(1050, assign34510_ad_e57399, A::add_scaled_inputs(assign34510_ad_e57399, 1.0, A::powf(s.ad_value(1048), (4.0 - p.p1908)), p.p1914));
        }

        if (s.b[1711] && s.b[1712]) {
            s.store_div_scaled_product_left_ad(175, A::powf(s.ad_value(1050), (1.0 / p.p1908)), 174, 1.0, 1048, 1.0);
            s.store_mul_powf_ad_rhs(1041, 172, A::offset(A::powf(s.ad_value(175), p.p1908), 1.0), (1.0 / p.p1908));
        }

        s.b[1715] = (p.p1911 > 0.0);
        s.v[1715] = if s.b[1715] { 1.0 } else { 0.0 };

        s.b[1716] = (p.p1910 == 0.0);
        s.v[1716] = if s.b[1716] { 1.0 } else { 0.0 };

        if ((s.b[1711] && s.b[1715]) && s.b[1716]) {
            if (!(((1.0 + (p.p1912 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                s.store_scaled_add_sqrt_square_offset_ad(1039, A::scale_offset(s.ad_value(232), p.p1912, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), 0.5);
            } else {
                if (((1.0 + (p.p1912 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                    s.store_div_from_scalar_offset_scaled_input(1039, ((-0.001) * 0.001), 232, p.p1912, ((1.0) + ((-1e-6))));
                } else {
                    s.store_scalar(1039, 0.0);
                }
            }
        }

        s.b[1717] = (p.p75 != 0.0);
        s.v[1717] = if s.b[1717] { 1.0 } else { 0.0 };

        if (((s.b[1711] && s.b[1715]) && s.b[1716]) && s.b[1717]) {
            s.store_offset_add_scaled_inputs(1044, A::scale_offset(s.ad_value(232), (-p.p1913), (((-(-p.p1904))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), (-p.p1913), (((-(-p.p1904))) + ((-1e-6)))), (-((4.0 * (-p.p1904)) * 1e-6))), 0.5, (((-p.p1904)) + (p.p1904)));
        }

        if (((s.b[1711] && s.b[1715]) && s.b[1716]) && (!s.b[1717])) {
            s.store_scale_ad(1044, {
                if (!(((1.0 + ((-p.p1913) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + ((-p.p1913) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1904);
        }

        if ((s.b[1711] && s.b[1715]) && s.b[1716]) {
            s.store_offset(168, 392, (-p.p1906));
            s.store_scaled_add_offset_sqrt_square_offset(168, 168, 0.1, (-0.1), ((0.25 * 2.0) * 2.0), 0.5);
            s.store_div_scaled_value_offset_denominator(169, s.ad_value(168), (10.0 * p.p1907), s.ad_value(168), (10.0 * p.p1907), 1.0);
            s.store_mul_scale_offset_rhs(1045, 1044, 169, p.p1905, 1.0);
        }

        if ((s.b[1711] && s.b[1715]) && s.b[1716]) {
            if (!(s.v[1045] < ((-10000.0) * 10.0))) {
                s.store_scaled_add_sqrt_square_offset_rhs(1045, 1045, 1045, ((4.0 * 10.0) * 10.0), 0.5);
            } else {
                if (s.v[1045] < ((-10000.0) * 10.0)) {
                    s.store_div_from_scalar(1045, ((-10.0) * 10.0), 1045);
                } else {
                    s.store_scalar(1045, 0.0);
                }
            }
        }

        if ((s.b[1711] && s.b[1715]) && s.b[1716]) {
            s.store_scaled_mul(170, 158, 1045, (s.v[115] * 1.60219e-19));
        }

        if (s.b[1711] && s.b[1715]) {
            s.store_scale(1046, 170, p.p1909);
            s.store_scaled_mul(172, 1039, 189, p.p1911);
            s.store_mul(1049, 1046, 172);
            s.store_abs_voltage(174, ctx, nodes, Some(6), Some(8));
        }

        if (s.b[1711] && s.b[1715]) {
            let assign34700_ad_e57843: A = A::powf(s.ad_value(174), (4.0 - p.p1908));
            s.store_div_ad(1051, assign34700_ad_e57843, A::add_scaled_inputs(assign34700_ad_e57843, 1.0, A::powf(s.ad_value(1049), (4.0 - p.p1908)), p.p1915));
        }

        if (s.b[1711] && s.b[1715]) {
            s.store_div_scaled_product_left_ad(175, A::powf(s.ad_value(1051), (1.0 / p.p1908)), 174, 1.0, 1049, 1.0);
            s.store_mul_powf_ad_rhs(1040, 172, A::offset(A::powf(s.ad_value(175), p.p1908), 1.0), (1.0 / p.p1908));
        }

    }

    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1718] = ((p.p64 != 2.0) && (s.v[191] > 0.0));
        s.v[1718] = if s.b[1718] { 1.0 } else { 0.0 };

        if s.b[1718] {
            s.store_div_from_scalar(596, 1.0, 192);
        }

        s.b[1719] = ((p.p64 == 1.0) && (p.p1910 > 0.0));
        s.v[1719] = if s.b[1719] { 1.0 } else { 0.0 };

        if (s.b[1718] && s.b[1719]) {
            s.store_div_from_scalar(1042, 1.0, 1041);
        }

        s.b[1720] = ((p.p64 != 2.0) && (s.v[190] > 0.0));
        s.v[1720] = if s.b[1720] { 1.0 } else { 0.0 };

        if s.b[1720] {
            s.store_div_from_scalar(595, 1.0, 193);
        }

        s.b[1721] = ((p.p64 == 1.0) && (p.p1911 > 0.0));
        s.v[1721] = if s.b[1721] { 1.0 } else { 0.0 };

        if (s.b[1720] && s.b[1721]) {
            s.store_div_from_scalar(1043, 1.0, 1040);
        }

        s.b[1722] = ((p.p73 == 1.0) && (s.v[873] != 0.0));
        s.v[1722] = if s.b[1722] { 1.0 } else { 0.0 };

        s.b[1723] = (p.p73 == 2.0);
        s.v[1723] = if s.b[1723] { 1.0 } else { 0.0 };

        s.b[1731] = (p.p72 == 0.0);
        s.v[1731] = if s.b[1731] { 1.0 } else { 0.0 };

        s.b[1736] = ((p.p74 != 0.0) && (p.p1791 > 0.0));
        s.v[1736] = if s.b[1736] { 1.0 } else { 0.0 };

        s.b[1737] = ((p.p64 != 2.0) && (s.v[191] > 0.0));
        s.v[1737] = if s.b[1737] { 1.0 } else { 0.0 };

        s.b[1738] = ((p.p64 == 1.0) && (p.p1910 > 0.0));
        s.v[1738] = if s.b[1738] { 1.0 } else { 0.0 };

        s.b[1739] = ((p.p64 != 2.0) && (s.v[190] > 0.0));
        s.v[1739] = if s.b[1739] { 1.0 } else { 0.0 };

        s.b[1740] = ((p.p64 == 1.0) && (p.p1911 > 0.0));
        s.v[1740] = if s.b[1740] { 1.0 } else { 0.0 };

        s.store_add_scaled_ad_lhs(339, A::div_scaled_inputs(s.ad_value(179), 10.0, s.ad_value(898), 1.0), 396, 2.0);

        s.store_mul_add_rhs(169, 179, 179, 339);

        s.store_mul_square_lhs(170, 163, 169);

        s.store_scaled_mul(171, 141, 179, ((2.0 * 1.60219e-19) * s.v[143]));

    }

    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.v[188] = 0.0;

        s.v[197] = 0.0;

        s.v[263] = 0.0;

        s.v[264] = 0.0;

        s.v[265] = 0.0;

        s.v[266] = 0.0;

        s.v[267] = 0.0;

        s.v[268] = 0.0;

        s.v[269] = 0.0;

        s.v[270] = 0.0;

        s.v[271] = 0.0;

        s.v[272] = 0.0;

        s.v[273] = 0.0;

        s.v[274] = 0.0;

        s.v[275] = 0.0;

        s.v[276] = 0.0;

        s.v[277] = 0.0;

        s.v[278] = 0.0;

        s.v[279] = 0.0;

        s.v[280] = 0.0;

        s.v[281] = 0.0;

        s.v[282] = 0.0;

        s.v[283] = 0.0;

        s.v[284] = 0.0;

        s.v[285] = 0.0;

        s.v[286] = 0.0;

        s.v[287] = 0.0;

        s.v[288] = 0.0;

        s.v[289] = 0.0;

        s.v[290] = 0.0;

        s.v[291] = 0.0;

        s.v[292] = 0.0;

        s.v[300] = 0.0;

        s.v[302] = 0.0;

        s.v[305] = 0.0;

        s.v[314] = 0.0;

        s.v[315] = 0.0;

        s.v[316] = 0.0;

        s.v[320] = 0.0;

        s.v[333] = 0.0;

        s.v[335] = 0.0;

        s.v[338] = 0.0;

        s.v[258] = 0.0;

        s.v[857] = 0.0;

        s.v[373] = 0.0;

        s.v[401] = 0.0;

        s.v[417] = 0.0;

        s.v[453] = 0.0;

        s.v[756] = 0.0;

        s.v[757] = 0.0;

        s.v[255] = 0.0;

        s.v[758] = 0.0;

        s.v[759] = 0.0;

        s.v[760] = 0.0;

        s.v[770] = 0.0;

        s.v[771] = 0.0;

        s.v[251] = 0.0;

        s.v[772] = 0.0;

        s.v[773] = 0.0;

        s.v[774] = 0.0;

        s.v[494] = 0.0;

        s.v[495] = 0.0;

        s.v[496] = 0.0;

        s.v[498] = 0.0;

        s.v[499] = 0.0;

        s.v[523] = 0.0;

        s.v[524] = 0.0;

        s.v[525] = 0.0;

        s.v[526] = 0.0;

        s.v[527] = 0.0;

        s.v[528] = 0.0;

        s.v[529] = 0.0;

        s.v[533] = 0.0;

        s.v[537] = 0.0;

        s.v[538] = 0.0;

        s.v[539] = 0.0;

        s.v[540] = 0.0;

        s.v[546] = 0.0;

        s.v[547] = 0.0;

        s.v[541] = 0.0;

        s.v[542] = 0.0;

        s.v[543] = 0.0;

        s.v[553] = 0.0;

        s.v[554] = 0.0;

        s.v[548] = 0.0;

        s.v[549] = 0.0;

        s.v[550] = 0.0;

        s.v[557] = 0.0;

        s.v[558] = 0.0;

        s.v[559] = 0.0;

        s.v[560] = 0.0;

        s.v[561] = 0.0;

        s.v[562] = 0.0;

        s.v[563] = 0.0;

        s.v[564] = 0.0;

        s.v[565] = 0.0;

        s.v[566] = 0.0;

        s.v[567] = 0.0;

        s.v[568] = 0.0;

        s.v[589] = 0.0;

        s.v[574] = 0.0;

        s.v[575] = 0.0;

        s.v[620] = 0.0;

        s.v[632] = 0.0;

        s.v[634] = 0.0;

        s.v[668] = 0.0;

        s.v[665] = 0.0;

        s.v[677] = 0.0;

        s.v[806] = 0.0;

        s.v[370] = 0.0;

        s.v[689] = 0.0;

        s.v[690] = 0.0;

        s.v[691] = 0.0;

        s.v[692] = 0.0;

        s.v[693] = 0.0;

        s.v[871] = 0.0;

        s.v[872] = 0.0;

        s.v[680] = 0.0;

        s.v[699] = 0.0;

        s.v[658] = 0.0;

        s.v[791] = 0.0;

        s.v[701] = 0.0;

        s.v[851] = 0.0;

        s.v[706] = 0.0;

        s.v[710] = 0.0;

        s.v[815] = 0.0;

        s.v[809] = 0.0;

        s.v[817] = 0.0;

        s.v[816] = 0.0;

        s.v[818] = 0.0;

        s.v[845] = 0.0;

        s.v[846] = 0.0;

        s.v[825] = 0.0;

        s.v[828] = 0.0;

        s.v[843] = 0.0;

        s.v[844] = 0.0;

        s.v[715] = 0.0;

        s.v[717] = 0.0;

        s.v[796] = 0.0;

        s.v[646] = 0.0;

        s.v[647] = 0.0;

        s.v[645] = 0.0;

        s.v[644] = 0.0;

        s.v[893] = 0.0;

        s.v[894] = 0.0;

        s.v[895] = 0.0;

        s.v[896] = 0.0;

        s.v[898] = 0.0;

        s.v[903] = 0.0;

        s.v[904] = 0.0;

        s.v[923] = 0.0;

        s.v[392] = 0.0;

        s.v[393] = 0.0;

        s.v[503] = 0.0;

        s.v[504] = 0.0;

        s.v[949] = 0.0;

        s.v[950] = 0.0;

        s.v[951] = 0.0;

        s.v[952] = 0.0;

        s.v[953] = 0.0;

        s.v[955] = 0.0;

        s.v[956] = 0.0;

        s.v[957] = 0.0;

        s.v[958] = 0.0;

        s.v[959] = 0.0;

        s.v[1004] = 0.0;

        s.v[1005] = 0.0;

        s.v[1006] = 0.0;

        s.v[1007] = 0.0;

        s.v[1008] = 0.0;

        s.v[1009] = 0.0;

        s.v[983] = 1.0;

        s.v[960] = 0.0;

        s.v[961] = 0.0;

        s.v[962] = 0.0;

        s.v[963] = 0.0;

        s.v[964] = 0.0;

        s.v[965] = 0.0;

        s.v[984] = 0.0;

        s.v[985] = 0.0;

        s.v[986] = 0.0;

        s.v[1010] = 0.0;

        s.v[1011] = 0.0;

        s.v[1012] = 0.0;

        s.v[882] = 0.0;

        s.v[883] = 0.0;

        s.v[884] = 0.0;

        s.v[885] = 0.0;

        s.v[886] = 0.0;

        s.v[887] = 0.0;

        s.v[888] = 0.0;

        s.v[889] = 0.0;

        s.v[890] = 0.0;

        s.v[891] = 0.0;

        s.v[892] = 0.0;

        s.v[119] = 0.0;

        s.v[120] = 0.0;

        s.v[118] = 0.0;

        s.v[117] = 0.0;

        s.v[233] = 0.0;

        s.v[234] = 0.0;

        s.v[182] = 0.0;

        s.v[142] = 0.0;

        s.v[324] = 0.0;

        s.v[327] = 0.0;

        s.v[306] = 0.0;

        s.v[307] = 0.0;

        s.v[310] = 0.0;

        s.v[311] = 0.0;

        s.v[313] = 0.0;

        s.v[312] = 0.0;

        s.v[331] = 0.0;

        s.v[330] = 0.0;

        s.v[1039] = 0.0;

        s.v[446] = 0.0;

        s.v[576] = 0.0;

        s.b[1057] = (p.p60 == 1.0);
        s.v[1057] = if s.b[1057] { 1.0 } else { 0.0 };

        if s.b[1057] {
            s.store_scalar(114, 1.0);
        }

        if (!s.b[1057]) {
            s.store_scalar(114, (-1.0));
        }

        s.v[143] = (p.p103 * 8.8542e-12);

        s.v[144] = (p.p1088 * 8.8542e-12);

        s.v[165] = ((p.p102 * 8.8542e-12) / p.p91);

        s.v[145] = (p.p103 / p.p102);

        s.v[381] = (0.916 * 9.11e-31);

        s.v[382] = (0.19 * 9.11e-31);

        s.v[383] = (0.19 * 9.11e-31);

        s.v[384] = (0.417 * 9.11e-31);

        s.v[385] = 4.0;

        s.v[386] = 2.0;

        s.v[876] = (((p.p109 + ((1e-6 * p.p110) / p.p0)) + (p.p111 / p.p5)) + ((p.p112 * 1e-6) / (p.p0 * p.p5)));

        s.v[878] = (((p.p117 + ((1e-6 * p.p118) / p.p0)) + (p.p119 / p.p5)) + ((p.p120 * 1e-6) / (p.p0 * p.p5)));

        s.v[877] = (((p.p113 + ((1e-6 * p.p114) / p.p0)) + (p.p115 / p.p5)) + ((p.p116 * 1e-6) / (p.p0 * p.p5)));

        s.v[149] = (p.p0 + s.v[876]);

        s.b[1058] = (s.v[149] <= 0.0);
        s.v[1058] = if s.b[1058] { 1.0 } else { 0.0 };

        if s.b[1058] {
            s.store_scalar(149, p.p0);
        }

        s.store_powf(168, 149, (-p.p84));

        s.store_offset_scaled(150, 168, p.p83, s.v[877]);

        s.store_offset_scaled_ad(151, A::powf(A::offset(s.ad_value(149), s.v[878]), (-p.p84)), p.p83, s.v[877]);

        s.store_offset_scaled(152, 168, p.p88, p.p85);

        s.store_sub_scaled_inputs(153, 149, 1.0, 150, 2.0);

        s.store_sub_scaled_ad_lhs(155, A::offset(s.ad_value(149), s.v[878]), 151, 2.0);

        s.store_sub_scaled_inputs(156, 149, 1.0, 152, 2.0);

        s.store_offset(157, 156, (-p.p86));

        s.b[1059] = (s.v[153] <= 0.0);
        s.v[1059] = if s.b[1059] { 1.0 } else { 0.0 };

        if s.b[1059] {
            s.copy_ad(153, 149);
        }

        s.b[1061] = (s.v[155] <= 0.0);
        s.v[1061] = if s.b[1061] { 1.0 } else { 0.0 };

        if s.b[1061] {
            s.copy_ad(155, 149);
        }

        s.b[1063] = (s.v[156] <= 0.0);
        s.v[1063] = if s.b[1063] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1063] {
            s.copy_ad(156, 149);
        }

        s.b[1065] = (p.p61 != 0.0);
        s.v[1065] = if s.b[1065] { 1.0 } else { 0.0 };

        s.b[1066] = (s.v[157] <= 0.0);
        s.v[1066] = if s.b[1066] { 1.0 } else { 0.0 };

        if (s.b[1065] && s.b[1066]) {
            s.copy_ad(157, 149);
        }

        s.b[1068] = (p.p62 == 5.0);
        s.v[1068] = if s.b[1068] { 1.0 } else { 0.0 };

        if s.b[1068] {
            s.store_scalar(879, (((((p.p121 + ((1e-6 * p.p122) / p.p0)) + (p.p123 / p.p5)) + ((p.p124 * 1e-6) / (p.p0 * p.p5))) + ((1e-6 * p.p125) / p.p43)) + ((p.p126 * 1e-12) / (p.p0 * p.p43))));
            s.store_scalar(880, (((((p.p127 + ((1e-6 * p.p128) / p.p0)) + (p.p129 / p.p5)) + ((p.p130 * 1e-6) / (p.p0 * p.p5))) + ((1e-6 * p.p131) / p.p43)) + ((p.p132 * 1e-12) / (p.p0 * p.p43))));
        }

        if (!s.b[1068]) {
            s.store_scalar(879, 0.0);
            s.store_scalar(880, 0.0);
        }

        s.store_offset(161, 879, p.p43);

        s.store_add(162, 161, 880);

        s.b[1069] = (p.p62 == 5.0);
        s.v[1069] = if s.b[1069] { 1.0 } else { 0.0 };

        s.b[1070] = (s.v[162] <= 0.0);
        s.v[1070] = if s.b[1070] { 1.0 } else { 0.0 };

        if (s.b[1069] && s.b[1070]) {
            s.store_scalar(162, p.p43);
        }

        s.v[115] = (p.p5 * p.p59);

        s.store_div_from_scalar(635, 1e-6, 155);

        s.v[636] = (1.0 / p.p5);

        s.store_div_from_scalar_scaled_input(637, 1e-6, 155, p.p5);

        s.b[1072] = (p.p62 == 5.0);
        s.v[1072] = if s.b[1072] { 1.0 } else { 0.0 };

        if s.b[1072] {
            s.store_div_from_scalar(638, 1e-6, 162);
            s.store_div_from_scalar_mul_ad(639, 1e-12, s.ad_value(162), s.ad_value(155));
        }

        if (!s.b[1072]) {
            s.store_scalar(638, 0.0);
            s.store_scalar(639, 0.0);
        }

        s.store_add_scaled_inputs4_offset_indices(640, 635, p.p134, 637, p.p136, 638, 0.0, 639, 0.0, ((p.p133) + ((s.v[636] * p.p135))));

        s.b[1073] = (p.p95 != 0.0);
        s.v[1073] = if s.b[1073] { 1.0 } else { 0.0 };

        if s.b[1073] {
            s.store_scale(640, 640, (1.0 + ((p.p95 / p.p5) * (if (!((1.0 + (p.p5 / p.p96)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p96)) > 1e-38) { (((1.0 + (p.p5 / p.p96))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1074] = (s.v[640] <= 0.0);
        s.v[1074] = if s.b[1074] { 1.0 } else { 0.0 };

        if s.b[1074] {
            s.store_scalar(640, 1e22);
        }

        s.b[1076] = (p.p62 == 0.0);
        s.v[1076] = if s.b[1076] { 1.0 } else { 0.0 };

        s.b[1077] = (p.p62 == 1.0);
        s.v[1077] = if s.b[1077] { 1.0 } else { 0.0 };

        s.b[1078] = (p.p62 == 2.0);
        s.v[1078] = if s.b[1078] { 1.0 } else { 0.0 };

        s.b[1079] = (p.p62 == 3.0);
        s.v[1079] = if s.b[1079] { 1.0 } else { 0.0 };

        s.b[1080] = (p.p62 == 4.0);
        s.v[1080] = if s.b[1080] { 1.0 } else { 0.0 };

        s.b[1081] = (p.p62 == 5.0);
        s.v[1081] = if s.b[1081] { 1.0 } else { 0.0 };

        s.b[1082] = ((p.p1802 == 0.0) || (p.p1803 == 0.0));
        s.v[1082] = if s.b[1082] { 1.0 } else { 0.0 };

        if (s.b[1076] && s.b[1082]) {
            s.store_scalar(895, (2.0 * p.p92));
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
            s.store_scalar(894, (p.p92 * p.p3));
        }

        if (s.b[1076] && (!s.b[1082])) {
            s.store_scalar(895, (2.0 * ((((p.p92 * p.p92) + (((p.p1802 - p.p1803) * (p.p1802 - p.p1803)) / 4.0))) as f64).sqrt()));
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
            s.store_scalar(894, ((p.p92 * (p.p1802 + p.p1803)) / 2.0));
        }

        s.b[1083] = ((p.p1802 == 0.0) || (p.p1803 == 0.0));
        s.v[1083] = if s.b[1083] { 1.0 } else { 0.0 };

        if ((s.b[1077] && (!s.b[1076])) && s.b[1083]) {
            s.store_scalar(895, ((2.0 * p.p92) + p.p3));
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
            s.store_scalar(894, (p.p92 * p.p3));
        }

        if ((s.b[1077] && (!s.b[1076])) && (!s.b[1083])) {
            s.store_scalar(895, ((2.0 * ((((p.p92 * p.p92) + (((p.p1802 - p.p1803) * (p.p1802 - p.p1803)) / 4.0))) as f64).sqrt()) + p.p1802));
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
            s.store_scalar(894, ((p.p92 * (p.p1802 + p.p1803)) / 2.0));
        }

        s.b[1084] = ((p.p1802 == 0.0) || (p.p1803 == 0.0));
        s.v[1084] = if s.b[1084] { 1.0 } else { 0.0 };

        if ((s.b[1078] && (!(s.b[1076] || s.b[1077]))) && s.b[1084]) {
            s.store_scalar(895, ((2.0 * p.p92) + (2.0 * p.p3)));
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
            s.store_scalar(894, (p.p92 * p.p3));
        }

        if ((s.b[1078] && (!(s.b[1076] || s.b[1077]))) && (!s.b[1084])) {
            s.store_scalar(895, (((2.0 * ((((p.p92 * p.p92) + (((p.p1802 - p.p1803) * (p.p1802 - p.p1803)) / 4.0))) as f64).sqrt()) + p.p1802) + p.p1803));
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
            s.store_scalar(894, ((p.p92 * (p.p1802 + p.p1803)) / 2.0));
        }

        if (s.b[1078] && (!(s.b[1076] || s.b[1077]))) {
            s.store_scalar(896, p.p1803);
        }

        if (s.b[1079] && (!((s.b[1076] || s.b[1077]) || s.b[1078]))) {
            s.store_scalar(895, (3.141592653589793 * p.p2));
        }

        if (s.b[1079] && (!((s.b[1076] || s.b[1077]) || s.b[1078]))) {
            s.store_scalar(893, ((((2.0 * 3.141592653589793) * p.p102) * 8.8542e-12) / (if (!((1.0 + ((2.0 * p.p89) / p.p2)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + ((2.0 * p.p89) / p.p2)) > 1e-38) { (((1.0 + ((2.0 * p.p89) / p.p2))) as f64).ln() } else { 0.0 }) })));
        }

        if (s.b[1079] && (!((s.b[1076] || s.b[1077]) || s.b[1078]))) {
            s.store_scalar(894, (((3.141592653589793 * p.p2) * p.p2) / 4.0));
            s.store_scalar(896, p.p2);
        }

        if (s.b[1080] && (!(((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]))) {
            s.store_scalar(895, p.p1801);
            s.store_scalar(893, p.p1800);
            s.store_scalar(894, p.p1799);
        }

        if (s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) {
            s.store_offset_scaled(954, 161, 2.0, ((((p.p40) * (2.0))) + (p.p44)));
            s.store_offset_scaled(948, 161, p.p40, p.p45);
            s.copy_ad(895, 954);
            s.copy_ad(894, 948);
        }

        s.b[1085] = (p.p56 > 1.0);
        s.v[1085] = if s.b[1085] { 1.0 } else { 0.0 };

        if ((s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) && s.b[1085]) {
            s.store_offset_scaled(955, 161, 2.0, ((((p.p40) * (2.0))) + (p.p46)));
            s.store_offset_scaled(949, 161, p.p40, p.p47);
            s.store_add(895, 954, 955);
            s.store_add(894, 948, 949);
        }

        s.b[1086] = (p.p56 > 2.0);
        s.v[1086] = if s.b[1086] { 1.0 } else { 0.0 };

        if ((s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) && s.b[1086]) {
            s.store_offset_scaled(956, 161, 2.0, ((((p.p40) * (2.0))) + (p.p48)));
            s.store_offset_scaled(950, 161, p.p40, p.p49);
            s.store_add_scaled_inputs3_indices(895, 954, 1.0, 955, 1.0, 956, 1.0);
            s.store_add_scaled_inputs3_indices(894, 948, 1.0, 949, 1.0, 950, 1.0);
        }

        s.b[1087] = (p.p56 > 3.0);
        s.v[1087] = if s.b[1087] { 1.0 } else { 0.0 };

        if ((s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) && s.b[1087]) {
            s.store_offset_scaled(957, 161, 2.0, ((((p.p40) * (2.0))) + (p.p50)));
            s.store_offset_scaled(951, 161, p.p40, p.p51);
            s.store_add_scaled_inputs4_indices(895, 954, 1.0, 955, 1.0, 956, 1.0, 957, 1.0);
            s.store_add_scaled_inputs4_indices(894, 948, 1.0, 949, 1.0, 950, 1.0, 951, 1.0);
        }

        s.b[1088] = (p.p56 > 4.0);
        s.v[1088] = if s.b[1088] { 1.0 } else { 0.0 };

        if ((s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) && s.b[1088]) {
            s.store_offset_scaled(958, 161, 2.0, ((((p.p40) * (2.0))) + (p.p52)));
            s.store_offset_scaled(952, 161, p.p40, p.p53);
            s.store_add_ad_lhs(895, A::add_scaled_inputs4(s.ad_value(954), 1.0, s.ad_value(955), 1.0, s.ad_value(956), 1.0, s.ad_value(957), 1.0), 958);
            s.store_add_ad_lhs(894, A::add_scaled_inputs4(s.ad_value(948), 1.0, s.ad_value(949), 1.0, s.ad_value(950), 1.0, s.ad_value(951), 1.0), 952);
        }

        s.b[1089] = (p.p56 > 5.0);
        s.v[1089] = if s.b[1089] { 1.0 } else { 0.0 };

        if ((s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) && s.b[1089]) {
            s.store_offset_scaled(959, 161, 2.0, ((((p.p40) * (2.0))) + (p.p54)));
            s.store_offset_scaled(953, 161, p.p40, p.p55);
            s.store_add_ad_lhs(895, A::add(A::add_scaled_inputs4(s.ad_value(954), 1.0, s.ad_value(955), 1.0, s.ad_value(956), 1.0, s.ad_value(957), 1.0), s.ad_value(958)), 959);
            s.store_add_ad_lhs(894, A::add(A::add_scaled_inputs4(s.ad_value(948), 1.0, s.ad_value(949), 1.0, s.ad_value(950), 1.0, s.ad_value(951), 1.0), s.ad_value(952)), 953);
        }

        if (s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) {
            s.store_scalar(896, p.p43);
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
        }

        s.store_div_scaled_inputs_mixed_ia(898, 893, 2.0, A::div_scaled_inputs(A::square(s.ad_value(895)), s.v[143], s.ad_value(894), 1.0), 1.0);

        s.store_div_scaled_product_indices(903, 640, 894, (-1.60219e-19), 893, 1.0);

        s.store_div(163, 893, 895);

        s.b[1090] = (p.p61 != 0.0);
        s.v[1090] = if s.b[1090] { 1.0 } else { 0.0 };

        if s.b[1090] {
            s.store_scale(494, 163, (p.p89 * 1.0 / (p.p1528)));
        }

        s.store_offset(158, 895, (-p.p93));

        s.store_offset(159, 895, (-p.p94));

        s.b[1091] = (p.p62 == 5.0);
        s.v[1091] = if s.b[1091] { 1.0 } else { 0.0 };

        if s.b[1091] {
            s.store_offset(160, 158, (-((2.0 * p.p56) * p.p87)));
        }

        if (!s.b[1091]) {
            s.copy_ad(160, 158);
        }

        s.b[1092] = (p.p62 == 5.0);
        s.v[1092] = if s.b[1092] { 1.0 } else { 0.0 };

        s.b[1093] = (p.p61 != 0.0);
        s.v[1093] = if s.b[1093] { 1.0 } else { 0.0 };

        s.b[1094] = (s.v[160] <= 0.0);
        s.v[1094] = if s.b[1094] { 1.0 } else { 0.0 };

        if ((s.b[1092] && s.b[1093]) && s.b[1094]) {
            s.copy_ad(160, 895);
        }

        s.v[446] = p.p1085;

        s.store_add_scaled_inputs4_offset_indices(641, 635, p.p138, 637, p.p140, 638, p.p141, 639, p.p142, ((p.p137) + ((s.v[636] * p.p139))));

        s.store_add_scaled_inputs4_offset_indices(666, 635, p.p189, 637, p.p191, 638, p.p192, 639, p.p193, ((p.p188) + ((s.v[636] * p.p190))));

        s.store_add_scaled_inputs4_offset_indices(662, 635, p.p201, 637, p.p203, 638, p.p204, 639, p.p205, ((p.p200) + ((s.v[636] * p.p202))));

        s.store_add_scaled_inputs4_offset_indices(663, 635, p.p207, 637, p.p209, 638, p.p210, 639, p.p211, ((p.p206) + ((s.v[636] * p.p208))));

        s.store_add_scaled_inputs4_offset_indices(667, 635, p.p219, 637, p.p221, 638, p.p222, 639, p.p223, ((p.p218) + ((s.v[636] * p.p220))));

        s.store_add_scaled_inputs4_offset_indices(670, 635, p.p225, 637, p.p227, 638, p.p228, 639, p.p229, ((p.p224) + ((s.v[636] * p.p226))));

        s.store_add_scaled_inputs4_offset_indices(671, 635, p.p231, 637, p.p233, 638, p.p234, 639, p.p235, ((p.p230) + ((s.v[636] * p.p232))));

        s.store_add_scaled_inputs4_offset_indices(672, 635, p.p237, 637, p.p239, 638, p.p240, 639, p.p241, ((p.p236) + ((s.v[636] * p.p238))));

        s.store_add_scaled_inputs4_offset_indices(673, 635, p.p243, 637, p.p245, 638, p.p246, 639, p.p247, ((p.p242) + ((s.v[636] * p.p244))));

        s.store_add_scaled_inputs4_offset_indices(674, 635, p.p249, 637, p.p251, 638, p.p252, 639, p.p253, ((p.p248) + ((s.v[636] * p.p250))));

        s.store_add_scaled_inputs4_offset_indices(678, 635, p.p267, 637, p.p269, 638, p.p270, 639, p.p271, ((p.p266) + ((s.v[636] * p.p268))));

        s.store_add_scaled_inputs4_offset_indices(802, 635, p.p273, 637, p.p275, 638, p.p276, 639, p.p277, ((p.p272) + ((s.v[636] * p.p274))));

        s.store_add_scaled_inputs4_offset_indices(803, 635, p.p279, 637, p.p281, 638, p.p282, 639, p.p283, ((p.p278) + ((s.v[636] * p.p280))));

        s.store_add_scaled_inputs4_offset_indices(804, 635, p.p285, 637, p.p287, 638, p.p288, 639, p.p289, ((p.p284) + ((s.v[636] * p.p286))));

        s.store_add_scaled_inputs4_offset_indices(683, 635, p.p297, 637, p.p299, 638, p.p300, 639, p.p301, ((p.p296) + ((s.v[636] * p.p298))));

        s.store_add_scaled_inputs4_offset_indices(684, 635, p.p303, 637, p.p305, 638, p.p306, 639, p.p307, ((p.p302) + ((s.v[636] * p.p304))));

        s.store_add_scaled_inputs4_offset_indices(685, 635, p.p309, 637, p.p311, 638, p.p312, 639, p.p313, ((p.p308) + ((s.v[636] * p.p310))));

        s.store_add_scaled_inputs4_offset_indices(686, 635, p.p315, 637, p.p317, 638, p.p318, 639, p.p319, ((p.p314) + ((s.v[636] * p.p316))));

        s.store_add_scaled_inputs4_offset_indices(687, 635, p.p321, 637, p.p323, 638, p.p324, 639, p.p325, ((p.p320) + ((s.v[636] * p.p322))));

        s.store_add_scaled_inputs4_offset_indices(688, 635, p.p327, 637, p.p329, 638, p.p330, 639, p.p331, ((p.p326) + ((s.v[636] * p.p328))));

        s.store_add_scaled_inputs4_offset_indices(867, 635, p.p333, 637, p.p335, 638, p.p336, 639, p.p337, ((p.p332) + ((s.v[636] * p.p334))));

        s.store_add_scaled_inputs4_offset_indices(868, 635, p.p339, 637, p.p341, 638, p.p342, 639, p.p343, ((p.p338) + ((s.v[636] * p.p340))));

        s.store_add_scaled_inputs4_offset_indices(869, 635, p.p345, 637, p.p347, 638, p.p348, 639, p.p349, ((p.p344) + ((s.v[636] * p.p346))));

        s.store_add_scaled_inputs4_offset_indices(870, 635, p.p351, 637, p.p353, 638, p.p354, 639, p.p355, ((p.p350) + ((s.v[636] * p.p352))));

        s.store_add_scaled_inputs4_offset_indices(654, 635, p.p404, 637, p.p406, 638, p.p407, 639, p.p408, ((p.p403) + ((s.v[636] * p.p405))));

        s.store_add_scaled_inputs4_offset_indices(655, 635, p.p410, 637, p.p412, 638, p.p413, 639, p.p414, ((p.p409) + ((s.v[636] * p.p411))));

        s.store_add_scaled_inputs4_offset_indices(656, 635, p.p416, 637, p.p418, 638, p.p419, 639, p.p420, ((p.p415) + ((s.v[636] * p.p417))));

        s.store_add_scaled_inputs4_offset_indices(661, 635, p.p422, 637, p.p424, 638, p.p425, 639, p.p426, ((p.p421) + ((s.v[636] * p.p423))));

        s.store_add_scaled_inputs4_offset_indices(679, 635, p.p456, 637, p.p458, 638, p.p459, 639, p.p460, ((p.p455) + ((s.v[636] * p.p457))));

        s.store_add_scaled_inputs4_offset_indices(698, 635, p.p468, 637, p.p470, 638, p.p471, 639, p.p472, ((p.p467) + ((s.v[636] * p.p469))));

        s.store_add_scaled_inputs4_offset_indices(702, 635, p.p507, 637, p.p509, 638, p.p510, 639, p.p511, ((p.p506) + ((s.v[636] * p.p508))));

        s.store_add_scaled_inputs4_offset_indices(881, 635, p.p513, 637, p.p515, 638, p.p516, 639, p.p517, ((p.p512) + ((s.v[636] * p.p514))));

        s.store_add_scaled_inputs4_offset_indices(694, 635, p.p480, 637, p.p482, 638, p.p483, 639, p.p484, ((p.p479) + ((s.v[636] * p.p481))));

        s.store_add_scaled_inputs4_offset_indices(695, 635, p.p486, 637, p.p488, 638, p.p489, 639, p.p490, ((p.p485) + ((s.v[636] * p.p487))));

        s.store_add_scaled_inputs4_offset_indices(696, 635, p.p519, 637, p.p521, 638, p.p522, 639, p.p523, ((p.p518) + ((s.v[636] * p.p520))));

        s.store_add_scaled_inputs4_offset_indices(697, 635, p.p525, 637, p.p527, 638, p.p528, 639, p.p529, ((p.p524) + ((s.v[636] * p.p526))));

        s.store_add_scaled_inputs4_offset_indices(657, 635, p.p493, 637, p.p495, 638, p.p496, 639, p.p497, ((p.p492) + ((s.v[636] * p.p494))));

        s.store_add_scaled_inputs4_offset_indices(790, 635, p.p532, 637, p.p534, 638, p.p535, 639, p.p536, ((p.p531) + ((s.v[636] * p.p533))));

        s.store_add_scaled_inputs4_offset_indices(700, 635, p.p544, 637, p.p546, 638, p.p547, 639, p.p548, ((p.p543) + ((s.v[636] * p.p545))));

        s.store_add_scaled_inputs4_offset_indices(704, 635, p.p606, 637, p.p608, 638, p.p609, 639, p.p610, ((p.p605) + ((s.v[636] * p.p607))));

        s.store_add_scaled_inputs4_offset_indices(707, 635, p.p624, 637, p.p626, 638, p.p627, 639, p.p628, ((p.p623) + ((s.v[636] * p.p625))));

        s.store_add_scaled_inputs4_offset_indices(703, 635, p.p630, 637, p.p632, 638, p.p633, 639, p.p634, ((p.p629) + ((s.v[636] * p.p631))));

        s.store_add_scaled_inputs4_offset_indices(807, 635, p.p642, 637, p.p644, 638, p.p645, 639, p.p646, ((p.p641) + ((s.v[636] * p.p643))));

        s.store_add_scaled_inputs4_offset_indices(811, 635, p.p678, 637, p.p680, 638, p.p681, 639, p.p682, ((p.p677) + ((s.v[636] * p.p679))));

        s.store_add_scaled_inputs4_offset_indices(812, 635, p.p690, 637, p.p692, 638, p.p693, 639, p.p694, ((p.p689) + ((s.v[636] * p.p691))));

        s.store_add_scaled_inputs4_offset_indices(814, 635, p.p708, 637, p.p710, 638, p.p711, 639, p.p712, ((p.p707) + ((s.v[636] * p.p709))));

        s.store_add_scaled_inputs4_offset_indices(325, 635, p.p714, 637, p.p716, 638, p.p717, 639, p.p718, ((p.p713) + ((s.v[636] * p.p715))));

        s.store_add_scaled_inputs4_offset_indices(326, 635, p.p720, 637, p.p722, 638, p.p723, 639, p.p724, ((p.p719) + ((s.v[636] * p.p721))));

        s.store_add_scaled_inputs4_offset_indices(328, 635, p.p726, 637, p.p728, 638, p.p729, 639, p.p730, ((p.p725) + ((s.v[636] * p.p727))));

        s.store_add_scaled_inputs4_offset_indices(329, 635, p.p732, 637, p.p734, 638, p.p735, 639, p.p736, ((p.p731) + ((s.v[636] * p.p733))));

        s.store_add_scaled_inputs4_offset_indices(792, 635, p.p1027, 637, p.p1029, 638, p.p1030, 639, p.p1031, ((p.p1025) + ((s.v[636] * p.p1028))));

        s.store_add_scaled_inputs4_offset_indices(793, 635, p.p1039, 637, p.p1041, 638, p.p1042, 639, p.p1043, ((p.p1038) + ((s.v[636] * p.p1040))));

        s.store_add_scaled_inputs4_offset_indices(794, 635, p.p1045, 637, p.p1047, 638, p.p1048, 639, p.p1049, ((p.p1044) + ((s.v[636] * p.p1046))));

        s.store_add_scaled_inputs4_offset_indices(798, 635, p.p1051, 637, p.p1053, 638, p.p1054, 639, p.p1055, ((p.p1050) + ((s.v[636] * p.p1052))));

        s.store_add_scaled_inputs4_offset_indices(800, 635, p.p1057, 637, p.p1059, 638, p.p1060, 639, p.p1061, ((p.p1056) + ((s.v[636] * p.p1058))));

        s.store_add_scaled_inputs4_offset_indices(799, 635, p.p1063, 637, p.p1065, 638, p.p1066, 639, p.p1067, ((p.p1062) + ((s.v[636] * p.p1064))));

        s.store_add_scaled_inputs4_offset_indices(801, 635, p.p1069, 637, p.p1071, 638, p.p1072, 639, p.p1073, ((p.p1068) + ((s.v[636] * p.p1070))));

        s.store_add_scaled_inputs4_offset_indices(709, 635, p.p926, 637, p.p928, 638, p.p929, 639, p.p930, ((p.p925) + ((s.v[636] * p.p927))));

        s.store_add_scaled_inputs4_offset_indices(853, 635, p.p932, 637, p.p934, 638, p.p935, 639, p.p936, ((p.p931) + ((s.v[636] * p.p933))));

        s.store_add_scaled_inputs4_offset_indices(852, 635, p.p938, 637, p.p940, 638, p.p941, 639, p.p942, ((p.p937) + ((s.v[636] * p.p939))));

        s.store_add_scaled_inputs4_offset_indices(712, 635, p.p950, 637, p.p952, 638, p.p953, 639, p.p954, ((p.p949) + ((s.v[636] * p.p951))));

        s.store_add_scaled_inputs4_offset_indices(711, 635, p.p944, 637, p.p946, 638, p.p947, 639, p.p948, ((p.p943) + ((s.v[636] * p.p945))));

        s.store_add_scaled_inputs4_offset_indices(713, 635, p.p956, 637, p.p958, 638, p.p959, 639, p.p960, ((p.p955) + ((s.v[636] * p.p957))));

        s.store_add_scaled_inputs4_offset_indices(714, 635, p.p986, 637, p.p988, 638, p.p989, 639, p.p990, ((p.p985) + ((s.v[636] * p.p987))));

    }
}
