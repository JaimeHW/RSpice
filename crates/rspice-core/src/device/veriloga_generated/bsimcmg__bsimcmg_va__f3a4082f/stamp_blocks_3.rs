#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_div_from_scalar(406, 0.01, 163);

        s.store_ad_value(419, A::add_scaled_product(s.ad_value(396), s.v[420], s.ad_value(407), s.ad_value(392), s.v[420]));

        s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(392), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));

        s.store_pow_ad(171, s.ad_value(419), s.ad_value(822));

        s.b[1423] = (p.p61 != 0.0);
        s.v[1423] = if s.b[1423] { 1.0 } else { 0.0 };

        if s.b[1423] {
            s.store_ad_value(171, A::add_scaled_product(A::div(s.ad_value(820), s.ad_value(170)), 1.0, A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(821), s.ad_value(370), 1.0), s.ad_value(171), 1.0));
        }

        if (!s.b[1423]) {
            s.store_ad_value(171, A::add_scaled_product(A::div(s.ad_value(820), s.ad_value(170)), 1.0, s.ad_value(819), s.ad_value(171), 1.0));
        }

        s.store_offset(397, 171, 1.0);

        s.store_scaled_add_ad(397, A::offset(s.ad_value(397), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(397), (-1.0)), A::offset(s.ad_value(397), (-1.0))), ((0.25 * p.p604) * p.p604))), 0.5);

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
            s.store_scaled_add_ad_rhs(168, 169, A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01)), 0.5);
            s.store_mul_ad_affine_product_lhs(198, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189), s.v[115], 0.0, 194);
        }

        if ((!s.b[1424]) && (!s.b[1425])) {
            s.store_offset_mul(172, 711, 392, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_ad_rhs(168, 169, A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01)), 0.5);
            s.store_mul_ad_lhs(198, A::add_scaled_product(A::add(s.ad_value(190), s.ad_value(191)), 1.0, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189), s.v[115]), 194);
        }

        s.store_mul_ad_lhs(216, A::div_scaled_inputs(s.ad_value(428), 2.0, s.ad_value(416), 1.0), 397);

        s.store_mul(217, 216, 153);

        s.b[1426] = (p.p80 == 0.0);
        s.v[1426] = if s.b[1426] { 1.0 } else { 0.0 };

        if s.b[1426] {
            s.store_mul_ad_rhs(175, 659, A::add_scaled_inputs(s.ad_value(392), 1.0, s.ad_value(179), 2.0));
        }

        if (!s.b[1426]) {
            s.store_mul_ad_rhs(175, 659, A::add_scaled_inputs(s.ad_value(392), 1.0, s.ad_value(182), 2.0));
        }

        s.b[1427] = (s.v[198] > 0.0);
        s.v[1427] = if s.b[1427] { 1.0 } else { 0.0 };

        if s.b[1427] {
            s.store_mul3_lhs(224, 158, 428, 163);
            s.store_mul(168, 224, 198);
            s.store_scale(225, 168, 2.0);
            s.store_ad_value(226, A::add_scaled_product(A::add(s.ad_value(175), s.ad_value(217)), 1.0, s.ad_value(175), s.ad_value(168), 3.0));
            s.store_mul_ad_rhs(227, 175, A::add_scaled_product(s.ad_value(217), 1.0, s.ad_value(175), s.ad_value(168), 2.0));
            s.store_div_ad(210, A::sub(A::square(s.ad_value(226)), A::add_scaled_product(A::square(s.ad_value(226)), 1.0, s.ad_value(225), s.ad_value(227), (-2.0))), A::mul(A::add(s.ad_value(226), A::sqrt(A::add_scaled_product(A::square(s.ad_value(226)), 1.0, s.ad_value(225), s.ad_value(227), (-2.0)))), s.ad_value(225)));
        }

        if (!s.b[1427]) {
            s.store_ad_value(210, A::div_scaled_product(s.ad_value(217), s.ad_value(175), 1.0, A::add(s.ad_value(217), s.ad_value(175)), 1.0));
        }

        let assign21320_ad_e39524: A = {
    if (!((s.v[210] - 0.001) < ((-10000.0) * 1e-5))) {
        A::add_scaled_inputs(A::offset(s.ad_value(210), (-0.001)), 0.5, A::sqrt(A::offset(A::mul(A::offset(s.ad_value(210), (-0.001)), A::offset(s.ad_value(210), (-0.001))), ((4.0 * 1e-5) * 1e-5))), 0.5)
    } else {
        {
            if ((s.v[210] - 0.001) < ((-10000.0) * 1e-5)) {
                A::div_from_scalar(((-1e-5) * 1e-5), A::offset(s.ad_value(210), (-0.001)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(210, assign21320_ad_e39524, 0.001);

        s.store_pow_ad(176, A::offset(A::div(s.ad_value(126), s.ad_value(210)), 1e-6), s.ad_value(423));

        s.store_pow_ad(177, A::offset(s.ad_value(176), 1.0), s.ad_value(212));

        s.store_min_ad(390, A::div(s.ad_value(126), s.ad_value(177)), s.ad_value(126));

        s.store_add(129, 390, 375);

        s.store_powf_ad(170, A::neg(s.ad_value(897)), 0.666666667);

        s.b[1428] = (p.p61 != 0.0);
        s.v[1428] = if s.b[1428] { 1.0 } else { 0.0 };

        if s.b[1428] {
            let assign21390_ad_e39626: A = {
                if (!((((2.0 * s.v[166]) + s.v[129]) - s.v[367]) < ((-10000.0) * 0.1))) {
                    A::add_scaled_inputs(A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(129), 1.0, s.ad_value(367), -1.0), 0.5, A::sqrt(A::offset(A::mul(A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(129), 1.0, s.ad_value(367), -1.0), A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(129), 1.0, s.ad_value(367), -1.0)), ((4.0 * 0.1) * 0.1))), 0.5)
                } else {
                    {
                        if ((((2.0 * s.v[166]) + s.v[129]) - s.v[367]) < ((-10000.0) * 0.1)) {
                            A::div_from_scalar(((-0.1) * 0.1), A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(129), 1.0, s.ad_value(367), -1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad_value(169, assign21390_ad_e39626);
        }

        if s.b[1428] {
            s.store_mul_ad(171, A::div_scaled_inputs(s.ad_value(239), -1.0, s.ad_value(181), 2.0), A::sub(A::sqrt(s.ad_value(169)), A::sqrt_scaled_input(s.ad_value(166), 2.0)));
            s.store_ad_value(168, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(899), 1.0), 1.0, s.ad_value(914), s.ad_value(170), 1.0));
            s.store_ad_value(169, A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(900), 1.0));
        }

        if (!s.b[1428]) {
            s.store_ad_value(168, A::add_scaled_product(A::sub(s.ad_value(899), s.ad_value(897)), 1.0, s.ad_value(914), s.ad_value(170), 1.0));
            s.store_sub(169, 900, 897);
        }

        s.store_div_ad_lhs(170, A::sub(s.ad_value(348), s.ad_value(129)), 181);

        s.store_sub(924, 169, 170);

        s.store_scaled_sub(171, 170, 168, 0.5);

        s.store_limited_exp(901, 171);

        s.b[1429] = (s.v[901] > 1e-7);
        s.v[1429] = if s.b[1429] { 1.0 } else { 0.0 };

        if s.b[1429] {
            s.store_ln_offset_input(176, 901, 1.0);
            s.store_scaled_sub_from_scalar_ad(901, 1.0, A::sqrt(A::offset(A::square(s.ad_value(176)), 1.0)), 2.0);
            s.store_mul_ad_lhs(177, A::add_scaled_inputs(s.ad_value(901), p.p1805, s.ad_value(897), 1.0), 898);
            s.store_div_ad_rhs(172, 177, A::offset(A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0)));
            s.store_mul(174, 177, 172);
            s.store_ad_value(902, A::ln_scaled_input(A::add(s.ad_value(901), s.ad_value(897)), -1.0));
        }

        if s.b[1429] {
            let assign21560_ad_e39798: A = A::add(A::add_scaled_inputs3(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
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
            }, 1.0), {
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
            });
            s.store_ad_value(344, A::add_scaled_product(assign21560_ad_e39798, 1.0, s.ad_value(914), A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0));
        }

        if s.b[1429] {
            s.store_ad_value(345, A::add_scaled_product(A::add_scaled_product(A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::offset(A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0)), s.ad_value(898), 1.0), 1.0, s.ad_value(914), A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667)));
            s.store_ad_value(346, A::add_scaled_product(A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, s.ad_value(914), A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0))));
            s.store_ad_value(901, A::add_scaled_product(s.ad_value(901), 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::offset(A::div_scaled_product(s.ad_value(344), s.ad_value(346), 1.0, A::mul_scaled_lhs(s.ad_value(345), 2.0, s.ad_value(345)), 1.0), 1.0), (-1.0)));
            s.store_mul_ad_lhs(177, A::add_scaled_inputs(s.ad_value(901), p.p1805, s.ad_value(897), 1.0), 898);
            s.store_div_ad_rhs(172, 177, A::offset(A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0)));
            s.store_mul(174, 177, 172);
            s.store_ad_value(902, A::ln_scaled_input(A::add(s.ad_value(901), s.ad_value(897)), -1.0));
        }

        if s.b[1429] {
            let assign21640_ad_e39949: A = A::add(A::add_scaled_inputs3(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
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
            }, 1.0), {
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
            });
            s.store_ad_value(344, A::add_scaled_product(assign21640_ad_e39949, 1.0, s.ad_value(914), A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0));
        }

        if s.b[1429] {
            s.store_ad_value(345, A::add_scaled_product(A::add_scaled_product(A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::offset(A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0)), s.ad_value(898), 1.0), 1.0, s.ad_value(914), A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667)));
            s.store_ad_value(346, A::add_scaled_product(A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, s.ad_value(914), A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0))));
            s.store_ad_value(901, A::add_scaled_product(s.ad_value(901), 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::offset(A::div_scaled_product(s.ad_value(344), s.ad_value(346), 1.0, A::mul_scaled_lhs(s.ad_value(345), 2.0, s.ad_value(345)), 1.0), 1.0), (-1.0)));
        }

        if (!s.b[1429]) {
            s.store_mul_neg_lhs(901, 901, 901);
        }

        s.store_mul_neg_lhs(393, 901, 181);

        s.b[1430] = (p.p57 == 1.0);
        s.v[1430] = if s.b[1430] { 1.0 } else { 0.0 };

        if s.b[1430] {
            s.store_div_ad_lhs(1015, A::sub(s.ad_value(347), s.ad_value(129)), 181);
            s.store_scaled_add_ad_rhs(1016, 1015, A::sqrt(A::add_scaled_products(s.ad_value(1015), s.ad_value(1015), 1.0, s.ad_value(963), s.ad_value(963), 0.25)), 0.5);
            s.store_pow_ad(1017, s.ad_value(1016), A::scale(s.ad_value(960), 0.5));
            s.store_mul_ad_product_rhs(1007, 1010, s.ad_value(1017), A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));
            s.store_div_ad_lhs(1018, A::add_scaled_inputs3(s.ad_value(347), 1.0, s.ad_value(129), (-1.0), s.ad_value(985), -1.0), 181);
            s.store_scaled_add_ad_rhs(1019, 1018, A::sqrt(A::add_scaled_products(s.ad_value(1018), s.ad_value(1018), 1.0, s.ad_value(964), s.ad_value(964), 0.25)), 0.5);
            s.store_pow_ad(1020, s.ad_value(1019), A::scale(s.ad_value(961), 0.5));
            s.store_mul_ad_product_rhs(1008, 1011, s.ad_value(1020), A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));
            s.store_div_ad_lhs(1021, A::add_scaled_inputs3(s.ad_value(347), 1.0, s.ad_value(129), (-1.0), s.ad_value(986), -1.0), 181);
            s.store_scaled_add_ad_rhs(1022, 1021, A::sqrt(A::add_scaled_products(s.ad_value(1021), s.ad_value(1021), 1.0, s.ad_value(965), s.ad_value(965), 0.25)), 0.5);
            s.store_pow_ad(1023, s.ad_value(1022), A::scale(s.ad_value(962), 0.5));
            s.store_mul_ad_product_rhs(1009, 1012, s.ad_value(1023), A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));
            s.store_ad_value(393, A::add_scaled_products(s.ad_value(983), s.ad_value(393), 1.0, s.ad_value(984), A::add_scaled_inputs3(s.ad_value(1007), 1.0, s.ad_value(1008), 1.0, s.ad_value(1009), 1.0), 1.0));
        }

        s.b[1431] = (p.p67 == 1.0);
        s.v[1431] = if s.b[1431] { 1.0 } else { 0.0 };

        if s.b[1431] {
            s.store_add_ad(356, A::mul3_scaled_output(s.ad_value(297), s.ad_value(363), A::add_scaled_product(s.ad_value(127), 1.0, s.ad_value(674), A::sqrt(A::offset(s.ad_value(127), 0.01)), 1.0), -1.0), A::mul3(s.ad_value(681), s.ad_value(365), A::pow(A::offset(s.ad_value(127), 0.01), s.ad_value(682))));
            s.store_ad_value(359, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(354), 1.0, s.ad_value(356), 1.0, s.ad_value(357), 1.0), 1.0, s.ad_value(231), 1.0, s.ad_value(805), 1.0));
            s.store_ad_value(349, A::add_scaled_inputs3(s.ad_value(125), 1.0, s.ad_value(167), (-1.0), s.ad_value(359), -1.0));
            s.store_ad_value(185, A::div_scaled_product3(s.ad_value(414), s.ad_value(163), s.ad_value(158), 1.0, s.ad_value(153), 1.0));
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
            let assign21930_ad_e40368: A = A::sub({
                if (!(s.v[169] < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs(s.ad_value(169), 0.5, A::sqrt(A::offset(A::square(s.ad_value(169)), ((4.0 * 0.0001) * 0.0001))), 0.5)
                } else {
                    {
                        if (s.v[169] < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), s.ad_value(169))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(168));
            s.store_ad_value(350, assign21930_ad_e40368);
        }

        if (s.b[1431] && (!s.b[1432])) {
            let assign21940_ad_e40432: A = {
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
            };
            s.store_mul_scaled_ad_rhs(168, 181, -1.0, assign21940_ad_e40432);
        }

        if (s.b[1431] && (!s.b[1432])) {
            s.store_sub_ad_lhs(169, A::add_scaled_inputs(A::offset(s.ad_value(168), 0.01), 0.5, A::sqrt(A::offset(A::mul(A::offset(s.ad_value(168), (-0.01)), A::offset(s.ad_value(168), (-0.01))), ((0.25 * 0.0001) * 0.0001))), 0.5), 375);
            s.store_offset_add(170, 349, 169, p.p23);
        }

        if (s.b[1431] && (!s.b[1432])) {
            let assign21970_ad_e40517: A = A::sub({
                if (!(s.v[170] < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs(s.ad_value(170), 0.5, A::sqrt(A::offset(A::square(s.ad_value(170)), ((4.0 * 0.0001) * 0.0001))), 0.5)
                } else {
                    {
                        if (s.v[170] < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), s.ad_value(170))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(169));
            s.store_ad_value(350, assign21970_ad_e40517);
        }

        if s.b[1431] {
            s.copy_ad(130, 375);
            s.store_powf_ad(172, A::neg(s.ad_value(897)), 0.666666667);
        }

        s.b[1433] = (p.p61 != 0.0);
        s.v[1433] = if s.b[1433] { 1.0 } else { 0.0 };

        if (s.b[1431] && s.b[1433]) {
            let assign22010_ad_e40608: A = {
                if (!((((2.0 * s.v[166]) + s.v[130]) - s.v[367]) < ((-10000.0) * 0.1))) {
                    A::add_scaled_inputs(A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(130), 1.0, s.ad_value(367), -1.0), 0.5, A::sqrt(A::offset(A::mul(A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(130), 1.0, s.ad_value(367), -1.0), A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(130), 1.0, s.ad_value(367), -1.0)), ((4.0 * 0.1) * 0.1))), 0.5)
                } else {
                    {
                        if ((((2.0 * s.v[166]) + s.v[130]) - s.v[367]) < ((-10000.0) * 0.1)) {
                            A::div_from_scalar(((-0.1) * 0.1), A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(130), 1.0, s.ad_value(367), -1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad_value(169, assign22010_ad_e40608);
        }

        if (s.b[1431] && s.b[1433]) {
            s.store_mul_ad(171, A::div_scaled_inputs(s.ad_value(239), -1.0, s.ad_value(181), 2.0), A::sub(A::sqrt(s.ad_value(169)), A::sqrt_scaled_input(s.ad_value(166), 2.0)));
        }

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1431] && s.b[1433]) {
            s.store_ad_value(168, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(899), 1.0), 1.0, s.ad_value(914), s.ad_value(172), 1.0));
            s.store_ad_value(169, A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(900), 1.0));
        }

        if (s.b[1431] && (!s.b[1433])) {
            s.store_ad_value(168, A::add_scaled_product(A::sub(s.ad_value(899), s.ad_value(897)), 1.0, s.ad_value(914), s.ad_value(172), 1.0));
            s.store_sub(169, 900, 897);
        }

        if s.b[1431] {
            s.store_div_ad_lhs(170, A::sub(s.ad_value(350), s.ad_value(130)), 181);
            s.store_sub(924, 169, 170);
            s.store_scaled_sub(171, 170, 168, 0.5);
            s.store_limited_exp(901, 171);
        }

        s.b[1434] = (s.v[901] > 1e-7);
        s.v[1434] = if s.b[1434] { 1.0 } else { 0.0 };

        if (s.b[1431] && s.b[1434]) {
            s.store_ln_offset_input(176, 901, 1.0);
            s.store_scaled_sub_from_scalar_ad(901, 1.0, A::sqrt(A::offset(A::square(s.ad_value(176)), 1.0)), 2.0);
            s.store_mul_ad_lhs(177, A::add_scaled_inputs(s.ad_value(901), p.p1805, s.ad_value(897), 1.0), 898);
            s.store_div_ad_rhs(172, 177, A::offset(A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0)));
            s.store_mul(174, 177, 172);
            s.store_ad_value(902, A::ln_scaled_input(A::add(s.ad_value(901), s.ad_value(897)), -1.0));
        }

        if (s.b[1431] && s.b[1434]) {
            let assign22180_ad_e40816: A = A::add(A::add_scaled_inputs3(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
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
            }, 1.0), {
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
            });
            s.store_ad_value(344, A::add_scaled_product(assign22180_ad_e40816, 1.0, s.ad_value(914), A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0));
        }

        if (s.b[1431] && s.b[1434]) {
            s.store_ad_value(345, A::add_scaled_product(A::add_scaled_product(A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::offset(A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0)), s.ad_value(898), 1.0), 1.0, s.ad_value(914), A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667)));
            s.store_ad_value(346, A::add_scaled_product(A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, s.ad_value(914), A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0))));
            s.store_ad_value(901, A::add_scaled_product(s.ad_value(901), 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::offset(A::div_scaled_product(s.ad_value(344), s.ad_value(346), 1.0, A::mul_scaled_lhs(s.ad_value(345), 2.0, s.ad_value(345)), 1.0), 1.0), (-1.0)));
            s.store_mul_ad_lhs(177, A::add_scaled_inputs(s.ad_value(901), p.p1805, s.ad_value(897), 1.0), 898);
            s.store_div_ad_rhs(172, 177, A::offset(A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0)));
            s.store_mul(174, 177, 172);
            s.store_ad_value(902, A::ln_scaled_input(A::add(s.ad_value(901), s.ad_value(897)), -1.0));
        }

        if (s.b[1431] && s.b[1434]) {
            let assign22260_ad_e40983: A = A::add(A::add_scaled_inputs3(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
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
            }, 1.0), {
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
            });
            s.store_ad_value(344, A::add_scaled_product(assign22260_ad_e40983, 1.0, s.ad_value(914), A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0));
        }

        if (s.b[1431] && s.b[1434]) {
            s.store_ad_value(345, A::add_scaled_product(A::add_scaled_product(A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::offset(A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0)), s.ad_value(898), 1.0), 1.0, s.ad_value(914), A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667)));
            s.store_ad_value(346, A::add_scaled_product(A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, s.ad_value(914), A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0))));
            s.store_ad_value(901, A::add_scaled_product(s.ad_value(901), 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::offset(A::div_scaled_product(s.ad_value(344), s.ad_value(346), 1.0, A::mul_scaled_lhs(s.ad_value(345), 2.0, s.ad_value(345)), 1.0), 1.0), (-1.0)));
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
            s.store_div_ad_lhs(1015, A::sub(s.ad_value(349), s.ad_value(130)), 181);
            s.store_scaled_add_ad_rhs(1016, 1015, A::sqrt(A::add_scaled_products(s.ad_value(1015), s.ad_value(1015), 1.0, s.ad_value(963), s.ad_value(963), 0.25)), 0.5);
            s.store_pow_ad(1017, s.ad_value(1016), A::scale(s.ad_value(960), 0.5));
            s.store_mul_ad_product_rhs(1004, 1010, s.ad_value(1017), A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));
            s.store_div_ad_lhs(1018, A::add_scaled_inputs3(s.ad_value(349), 1.0, s.ad_value(130), (-1.0), s.ad_value(985), -1.0), 181);
            s.store_scaled_add_ad_rhs(1019, 1018, A::sqrt(A::add_scaled_products(s.ad_value(1018), s.ad_value(1018), 1.0, s.ad_value(964), s.ad_value(964), 0.25)), 0.5);
            s.store_pow_ad(1020, s.ad_value(1019), A::scale(s.ad_value(961), 0.5));
            s.store_mul_ad_product_rhs(1005, 1011, s.ad_value(1020), A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));
            s.store_div_ad_lhs(1021, A::add_scaled_inputs3(s.ad_value(349), 1.0, s.ad_value(130), (-1.0), s.ad_value(986), -1.0), 181);
            s.store_scaled_add_ad_rhs(1022, 1021, A::sqrt(A::add_scaled_products(s.ad_value(1021), s.ad_value(1021), 1.0, s.ad_value(965), s.ad_value(965), 0.25)), 0.5);
            s.store_pow_ad(1023, s.ad_value(1022), A::scale(s.ad_value(962), 0.5));
            s.store_mul_ad_product_rhs(1006, 1012, s.ad_value(1023), A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));
            s.store_ad_value(394, A::add_scaled_products(s.ad_value(983), s.ad_value(394), 1.0, s.ad_value(984), A::add_scaled_inputs3(s.ad_value(1004), 1.0, s.ad_value(1005), 1.0, s.ad_value(1006), 1.0), 1.0));
        }

        if s.b[1431] {
            s.store_ad_value(421, A::add_scaled_product(s.ad_value(396), s.v[420], s.ad_value(407), s.ad_value(394), s.v[420]));
            s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(394), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));
            s.store_pow_ad(171, s.ad_value(421), s.ad_value(822));
        }

        s.b[1436] = (p.p61 != 0.0);
        s.v[1436] = if s.b[1436] { 1.0 } else { 0.0 };

        if (s.b[1431] && s.b[1436]) {
            s.store_ad_value(171, A::add_scaled_product(A::div(s.ad_value(319), s.ad_value(170)), 1.0, A::add_scaled_product(s.ad_value(304), 1.0, s.ad_value(315), s.ad_value(370), 1.0), s.ad_value(171), 1.0));
        }

        if (s.b[1431] && (!s.b[1436])) {
            s.store_ad_value(171, A::add_scaled_product(A::div(s.ad_value(319), s.ad_value(170)), 1.0, s.ad_value(304), s.ad_value(171), 1.0));
        }

        if s.b[1431] {
            s.store_offset(398, 171, 1.0);
            s.store_scaled_add_ad(398, A::offset(s.ad_value(398), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(398), (-1.0)), A::offset(s.ad_value(398), (-1.0))), ((0.25 * p.p604) * p.p604))), 0.5);
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
            s.store_scaled_add_ad_rhs(168, 169, A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01)), 0.5);
            s.store_mul_ad_affine_product_lhs(199, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189), s.v[115], 0.0, 194);
        }

        if ((s.b[1431] && (!s.b[1437])) && (!s.b[1438])) {
            s.store_offset_mul(172, 711, 394, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_ad_rhs(168, 169, A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01)), 0.5);
            s.store_mul_ad_lhs(199, A::add_scaled_product(A::add(s.ad_value(190), s.ad_value(191)), 1.0, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189), s.v[115]), 194);
        }

        if s.b[1431] {
            s.store_mul_ad_lhs(222, A::div_scaled_inputs(s.ad_value(336), 2.0, s.ad_value(414), 1.0), 398);
            s.store_mul(223, 222, 153);
        }

        s.b[1439] = (p.p80 == 0.0);
        s.v[1439] = if s.b[1439] { 1.0 } else { 0.0 };

        if (s.b[1431] && s.b[1439]) {
            s.store_mul_ad_rhs(175, 659, A::add_scaled_inputs(s.ad_value(394), 1.0, s.ad_value(179), 2.0));
        }

        if (s.b[1431] && (!s.b[1439])) {
            s.store_mul_ad_rhs(175, 659, A::add_scaled_inputs(s.ad_value(394), 1.0, s.ad_value(182), 2.0));
        }

        s.b[1440] = (s.v[199] > 0.0);
        s.v[1440] = if s.b[1440] { 1.0 } else { 0.0 };

        if (s.b[1431] && s.b[1440]) {
            s.store_mul_ad_lhs(168, A::mul3(s.ad_value(158), s.ad_value(336), s.ad_value(163)), 199);
            s.store_scale(225, 168, 2.0);
            s.store_ad_value(226, A::add_scaled_product(A::add(s.ad_value(175), s.ad_value(223)), 1.0, s.ad_value(175), s.ad_value(168), 3.0));
            s.store_mul_ad_rhs(227, 175, A::add_scaled_product(s.ad_value(223), 1.0, s.ad_value(175), s.ad_value(168), 2.0));
            s.store_div_ad(211, A::sub(A::square(s.ad_value(226)), A::add_scaled_product(A::square(s.ad_value(226)), 1.0, s.ad_value(225), s.ad_value(227), (-2.0))), A::mul(A::add(s.ad_value(226), A::sqrt(A::add_scaled_product(A::square(s.ad_value(226)), 1.0, s.ad_value(225), s.ad_value(227), (-2.0)))), s.ad_value(225)));
        }

        if (s.b[1431] && (!s.b[1440])) {
            s.store_ad_value(211, A::div_scaled_product(s.ad_value(223), s.ad_value(175), 1.0, A::add(s.ad_value(223), s.ad_value(175)), 1.0));
        }

        if s.b[1431] {
            let assign22780_ad_e41709: A = {
                if (!((s.v[211] - 0.001) < ((-10000.0) * 1e-5))) {
                    A::add_scaled_inputs(A::offset(s.ad_value(211), (-0.001)), 0.5, A::sqrt(A::offset(A::mul(A::offset(s.ad_value(211), (-0.001)), A::offset(s.ad_value(211), (-0.001))), ((4.0 * 1e-5) * 1e-5))), 0.5)
                } else {
                    {
                        if ((s.v[211] - 0.001) < ((-10000.0) * 1e-5)) {
                            A::div_from_scalar(((-1e-5) * 1e-5), A::offset(s.ad_value(211), (-0.001)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(211, assign22780_ad_e41709, 0.001);
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
            let assign22850_ad_e41830: A = {
                if (!((((2.0 * s.v[166]) + s.v[130]) - s.v[367]) < ((-10000.0) * 0.1))) {
                    A::add_scaled_inputs(A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(130), 1.0, s.ad_value(367), -1.0), 0.5, A::sqrt(A::offset(A::mul(A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(130), 1.0, s.ad_value(367), -1.0), A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(130), 1.0, s.ad_value(367), -1.0)), ((4.0 * 0.1) * 0.1))), 0.5)
                } else {
                    {
                        if ((((2.0 * s.v[166]) + s.v[130]) - s.v[367]) < ((-10000.0) * 0.1)) {
                            A::div_from_scalar(((-0.1) * 0.1), A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(130), 1.0, s.ad_value(367), -1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad_value(169, assign22850_ad_e41830);
        }

        if (s.b[1431] && s.b[1441]) {
            s.store_mul_ad(171, A::div_scaled_inputs(s.ad_value(239), -1.0, s.ad_value(181), 2.0), A::sub(A::sqrt(s.ad_value(169)), A::sqrt_scaled_input(s.ad_value(166), 2.0)));
            s.store_ad_value(168, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(899), 1.0), 1.0, s.ad_value(914), s.ad_value(170), 1.0));
            s.store_ad_value(169, A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(900), 1.0));
        }

        if (s.b[1431] && (!s.b[1441])) {
            s.store_ad_value(168, A::add_scaled_product(A::sub(s.ad_value(899), s.ad_value(897)), 1.0, s.ad_value(914), s.ad_value(170), 1.0));
            s.store_sub(169, 900, 897);
        }

        if s.b[1431] {
            s.store_div_ad_lhs(170, A::sub(s.ad_value(350), s.ad_value(130)), 181);
            s.store_sub(924, 169, 170);
            s.store_scaled_sub(171, 170, 168, 0.5);
            s.store_limited_exp(901, 171);
        }

        s.b[1442] = (s.v[901] > 1e-7);
        s.v[1442] = if s.b[1442] { 1.0 } else { 0.0 };

        if (s.b[1431] && s.b[1442]) {
            s.store_ln_offset_input(176, 901, 1.0);
            s.store_scaled_sub_from_scalar_ad(901, 1.0, A::sqrt(A::offset(A::square(s.ad_value(176)), 1.0)), 2.0);
            s.store_mul_ad_lhs(177, A::add_scaled_inputs(s.ad_value(901), p.p1805, s.ad_value(897), 1.0), 898);
            s.store_div_ad_rhs(172, 177, A::offset(A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0)));
            s.store_mul(174, 177, 172);
            s.store_ad_value(902, A::ln_scaled_input(A::add(s.ad_value(901), s.ad_value(897)), -1.0));
        }

        if (s.b[1431] && s.b[1442]) {
            let assign23020_ad_e42038: A = A::add(A::add_scaled_inputs3(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
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
            }, 1.0), {
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
            });
            s.store_ad_value(344, A::add_scaled_product(assign23020_ad_e42038, 1.0, s.ad_value(914), A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0));
        }

        if (s.b[1431] && s.b[1442]) {
            s.store_ad_value(345, A::add_scaled_product(A::add_scaled_product(A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::offset(A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0)), s.ad_value(898), 1.0), 1.0, s.ad_value(914), A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667)));
            s.store_ad_value(346, A::add_scaled_product(A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, s.ad_value(914), A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0))));
            s.store_ad_value(901, A::add_scaled_product(s.ad_value(901), 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::offset(A::div_scaled_product(s.ad_value(344), s.ad_value(346), 1.0, A::mul_scaled_lhs(s.ad_value(345), 2.0, s.ad_value(345)), 1.0), 1.0), (-1.0)));
            s.store_mul_ad_lhs(177, A::add_scaled_inputs(s.ad_value(901), p.p1805, s.ad_value(897), 1.0), 898);
            s.store_div_ad_rhs(172, 177, A::offset(A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0)));
        }

    }

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1431] && s.b[1442]) {
            s.store_mul(174, 177, 172);
            s.store_ad_value(902, A::ln_scaled_input(A::add(s.ad_value(901), s.ad_value(897)), -1.0));
        }

        if (s.b[1431] && s.b[1442]) {
            let assign23100_ad_e42205: A = A::add(A::add_scaled_inputs3(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
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
            }, 1.0), {
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
            });
            s.store_ad_value(344, A::add_scaled_product(assign23100_ad_e42205, 1.0, s.ad_value(914), A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0));
        }

        if (s.b[1431] && s.b[1442]) {
            s.store_ad_value(345, A::add_scaled_product(A::add_scaled_product(A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::offset(A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0)), s.ad_value(898), 1.0), 1.0, s.ad_value(914), A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667)));
            s.store_ad_value(346, A::add_scaled_product(A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, s.ad_value(914), A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0))));
            s.store_ad_value(901, A::add_scaled_product(s.ad_value(901), 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::offset(A::div_scaled_product(s.ad_value(344), s.ad_value(346), 1.0, A::mul_scaled_lhs(s.ad_value(345), 2.0, s.ad_value(345)), 1.0), 1.0), (-1.0)));
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
            s.store_div_ad_lhs(1015, A::sub(s.ad_value(349), s.ad_value(130)), 181);
            s.store_scaled_add_ad_rhs(1016, 1015, A::sqrt(A::add_scaled_products(s.ad_value(1015), s.ad_value(1015), 1.0, s.ad_value(963), s.ad_value(963), 0.25)), 0.5);
            s.store_pow_ad(1017, s.ad_value(1016), A::scale(s.ad_value(960), 0.5));
            s.store_mul_ad_product_rhs(1007, 1010, s.ad_value(1017), A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));
            s.store_div_ad_lhs(1018, A::add_scaled_inputs3(s.ad_value(349), 1.0, s.ad_value(130), (-1.0), s.ad_value(985), -1.0), 181);
            s.store_scaled_add_ad_rhs(1019, 1018, A::sqrt(A::add_scaled_products(s.ad_value(1018), s.ad_value(1018), 1.0, s.ad_value(964), s.ad_value(964), 0.25)), 0.5);
            s.store_pow_ad(1020, s.ad_value(1019), A::scale(s.ad_value(961), 0.5));
            s.store_mul_ad_product_rhs(1008, 1011, s.ad_value(1020), A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));
            s.store_div_ad_lhs(1021, A::add_scaled_inputs3(s.ad_value(349), 1.0, s.ad_value(130), (-1.0), s.ad_value(986), -1.0), 181);
            s.store_scaled_add_ad_rhs(1022, 1021, A::sqrt(A::add_scaled_products(s.ad_value(1021), s.ad_value(1021), 1.0, s.ad_value(965), s.ad_value(965), 0.25)), 0.5);
            s.store_pow_ad(1023, s.ad_value(1022), A::scale(s.ad_value(962), 0.5));
            s.store_mul_ad_product_rhs(1009, 1012, s.ad_value(1023), A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));
            s.store_ad_value(395, A::add_scaled_products(s.ad_value(983), s.ad_value(395), 1.0, s.ad_value(984), A::add_scaled_inputs3(s.ad_value(1007), 1.0, s.ad_value(1008), 1.0, s.ad_value(1009), 1.0), 1.0));
        }

        if s.b[1431] {
            s.store_scaled_add(403, 394, 395, 0.5);
            s.store_sub(405, 394, 395);
            s.store_scaled_square(168, 391, 1600.0);
        }

        s.b[1444] = (p.p603 != 0.0);
        s.v[1444] = if s.b[1444] { 1.0 } else { 0.0 };

        if (s.b[1431] && s.b[1444]) {
            s.store_ad_value(404, A::add_scaled_product(A::add_scaled_inputs(s.ad_value(394), 0.5, s.ad_value(395), 0.5), 1.0, A::sub_from_scalar(1.0, A::limited_exp_scaled_input(s.ad_value(168), -1.0)), s.ad_value(405), (p.p603 * 0.5)));
        }

        if (s.b[1431] && (!s.b[1444])) {
            s.store_scaled_add(404, 394, 395, 0.5);
        }

        s.b[1445] = (p.p61 != 0.0);
        s.v[1445] = if s.b[1445] { 1.0 } else { 0.0 };

        if s.b[1445] {
            s.store_mul_ad(178, A::div_scaled_inputs(s.ad_value(239), 1.0, s.ad_value(181), 2.0), A::sqrt(s.ad_value(179)));
            s.store_scale(168, 178, 0.5);
        }

        if s.b[1445] {
            let assign23390_ad_e42610: A = A::add_scaled_product(A::sub(s.ad_value(167), s.ad_value(146)), 1.0, s.ad_value(179), {
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
            }, (-1.0));
            s.store_div_ad_lhs(170, A::sub(s.ad_value(497), A::offset(assign23390_ad_e42610, p.p1529)), 179);
        }

        s.b[1446] = ((s.v[170] * s.v[179]) > (s.v[166] + (s.v[178] * (((s.v[166] * s.v[179])) as f64).sqrt())));
        s.v[1446] = if s.b[1446] { 1.0 } else { 0.0 };

        if (s.b[1445] && s.b[1446]) {
            s.store_sub_ad_lhs(169, A::sqrt(A::add(A::offset(s.ad_value(170), (-1.0)), A::square(s.ad_value(168)))), 168);
            s.store_offset_square(340, 169, 1.0);
        }

        if (s.b[1445] && s.b[1446]) {
            s.store_ad_value(175, {
                if (!((((-s.v[340])) as f64).abs() < 1e-7)) {
                    A::offset(A::limited_exp_scaled_input(s.ad_value(340), -1.0), (-1.0))
                } else {
                    {
                        if ((((-s.v[340])) as f64).abs() < 1e-7) {
                            A::sub(A::mul_scaled_lhs(s.ad_value(340), (-(-0.5)), s.ad_value(340)), s.ad_value(340))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.b[1445] && (!s.b[1446])) {
            s.store_sub_scaled_ad_rhs(171, 170, 0.5, A::scale_offset(s.ad_value(178), ((1.0 / (((2.0) as f64).sqrt())) * (3.0)), 3.0));
            s.store_add_ad_rhs(340, 171, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(171)), 1.0, s.ad_value(170), 6.0)));
        }

        s.b[1447] = (s.v[170] < 0.0);
        s.v[1447] = if s.b[1447] { 1.0 } else { 0.0 };

        if ((s.b[1445] && (!s.b[1446])) && s.b[1447]) {
            s.store_div_ad_lhs(172, A::sub(s.ad_value(170), s.ad_value(340)), 178);
            s.store_sub_ad_lhs(175, A::square(s.ad_value(172)), 340);
        }

        if ((s.b[1445] && (!s.b[1446])) && s.b[1447]) {
            let assign23490_ad_e42791: A = A::neg({
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
            s.store_ad_value(340, assign23490_ad_e42791);
        }

        if ((s.b[1445] && (!s.b[1446])) && (!s.b[1447])) {
            s.store_limited_exp_neg_input(341, 340);
            s.store_sub_ad_lhs(172, A::sqrt(A::add_scaled_inputs3(A::offset(s.ad_value(170), (-1.0)), 1.0, s.ad_value(341), 1.0, A::square(s.ad_value(168)), 1.0)), 168);
            s.store_add_ad(340, A::sub_from_scalar(1.0, s.ad_value(341)), A::square(s.ad_value(172)));
        }

        if ((s.b[1445] && (!s.b[1446])) && (!s.b[1447])) {
            s.store_ad_value(175, {
                if (!((((-s.v[340])) as f64).abs() < 1e-7)) {
                    A::offset(A::limited_exp_scaled_input(s.ad_value(340), -1.0), (-1.0))
                } else {
                    {
                        if ((((-s.v[340])) as f64).abs() < 1e-7) {
                            A::sub(A::mul_scaled_lhs(s.ad_value(340), (-(-0.5)), s.ad_value(340)), s.ad_value(340))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if s.b[1445] {
            s.store_sqrt_add(176, 175, 340);
        }

        s.b[1448] = (s.v[340] > 1e-15);
        s.v[1448] = if s.b[1448] { 1.0 } else { 0.0 };

        if (s.b[1445] && s.b[1448]) {
            s.store_ad_value(344, A::add_scaled_product(A::sub(s.ad_value(170), s.ad_value(340)), -1.0, s.ad_value(178), s.ad_value(176), 1.0));
            s.store_sub_from_scalar_ad(345, 1.0, A::div_scaled_product(s.ad_value(178), s.ad_value(175), 0.5, s.ad_value(176), 1.0));
            s.store_sub_ad_rhs(177, 340, A::div(s.ad_value(344), s.ad_value(345)));
        }

        if (s.b[1445] && s.b[1448]) {
            s.store_ad_value(341, {
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
            });
        }

        if (s.b[1445] && s.b[1448]) {
            s.store_sqrt_add(342, 341, 177);
            s.store_mul3_affine_lhs(401, 178, 342, -1.0, 0.0, 179);
        }

        s.b[1449] = (s.v[340] < (-1e-15));
        s.v[1449] = if s.b[1449] { 1.0 } else { 0.0 };

        if ((s.b[1445] && (!s.b[1448])) && s.b[1449]) {
            s.store_ad_value(344, A::add_scaled_product(A::sub(s.ad_value(170), s.ad_value(340)), -1.0, s.ad_value(178), s.ad_value(176), (-1.0)));
            s.store_offset_ad(345, A::div_scaled_product(s.ad_value(178), s.ad_value(175), 0.5, s.ad_value(176), 1.0), 1.0);
            s.store_sub_ad_rhs(177, 340, A::div(s.ad_value(344), s.ad_value(345)));
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
            s.store_mul_ad_product_lhs(904, s.ad_value(178), A::limited_exp_scaled_input(s.ad_value(177), (-1.0 / (2.0))), 179);
            s.store_scaled_add_ad(921, A::offset(s.ad_value(177), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(177), (-1.0)), A::offset(s.ad_value(177), (-1.0))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_sqrt(922, 921);
            s.store_offset_div(923, 178, 922, 1.0);
        }

        s.store_scaled_add(399, 392, 393, 0.5);

        s.store_sub(402, 392, 393);

        s.store_scaled_square(168, 390, 1600.0);

        s.b[1450] = (p.p603 != 0.0);
        s.v[1450] = if s.b[1450] { 1.0 } else { 0.0 };

        if s.b[1450] {
            s.store_ad_value(400, A::add_scaled_product(A::add_scaled_inputs(s.ad_value(392), 0.5, s.ad_value(393), 0.5), 1.0, A::sub_from_scalar(1.0, A::limited_exp_scaled_input(s.ad_value(168), -1.0)), s.ad_value(402), (p.p603 * 0.5)));
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

        s.store_ad_value(183, A::div_scaled_product3(s.ad_value(416), s.ad_value(163), s.ad_value(158), 1.0, s.ad_value(153), 1.0));

        s.store_ad_value(409, A::add_scaled_product(s.ad_value(396), s.v[420], s.ad_value(407), s.ad_value(400), s.v[420]));

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
            s.store_ad_value(169, {
                if (!(s.v[168] < ((-10000.0) * 1e-12))) {
                    A::add_scaled_inputs(s.ad_value(168), 0.5, A::sqrt(A::offset(A::square(s.ad_value(168)), ((4.0 * 1e-12) * 1e-12))), 0.5)
                } else {
                    {
                        if (s.v[168] < ((-10000.0) * 1e-12)) {
                            A::div_from_scalar(((-1e-12) * 1e-12), s.ad_value(168))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (!s.b[1453]) {
            s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(169), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));
        }

        s.store_pow_ad(168, s.ad_value(409), s.ad_value(822));

        s.b[1454] = (p.p61 != 0.0);
        s.v[1454] = if s.b[1454] { 1.0 } else { 0.0 };

        if s.b[1454] {
            s.store_ad_value(171, A::add_scaled_product(A::div(s.ad_value(820), s.ad_value(170)), 1.0, A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(821), s.ad_value(370), 1.0), s.ad_value(168), 1.0));
        }

        if (!s.b[1454]) {
            s.store_ad_value(171, A::add_scaled_product(A::div(s.ad_value(820), s.ad_value(170)), 1.0, s.ad_value(819), s.ad_value(168), 1.0));
        }

        s.store_offset(411, 171, 1.0);

        s.store_scaled_add_ad(411, A::offset(s.ad_value(411), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(411), (-1.0)), A::offset(s.ad_value(411), (-1.0))), ((0.25 * p.p604) * p.p604))), 0.5);

        s.store_scaled_sub_from_scalar_ad(215, 1.0, A::scale(A::limited_exp_scaled_input(s.ad_value(390), (-p.p888)), p.p887), p.p24);

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
            s.store_ad_value(168, A::add_scaled_products(s.ad_value(330), s.ad_value(394), 1.0, s.ad_value(331), s.ad_value(395), 1.0));
        }

        if (s.b[1455] && (!s.b[1456])) {
            s.store_ad_value(169, {
                if (!(s.v[168] < ((-10000.0) * 1e-12))) {
                    A::add_scaled_inputs(s.ad_value(168), 0.5, A::sqrt(A::offset(A::square(s.ad_value(168)), ((4.0 * 1e-12) * 1e-12))), 0.5)
                } else {
                    {
                        if (s.v[168] < ((-10000.0) * 1e-12)) {
                            A::div_from_scalar(((-1e-12) * 1e-12), s.ad_value(168))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

    }

    pub(super) fn stamp_reactive_block_20(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1455] && (!s.b[1456])) {
            s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(169), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));
        }

        if s.b[1455] {
            s.store_ad_value(410, A::add_scaled_product(s.ad_value(396), s.v[420], s.ad_value(408), s.ad_value(404), s.v[420]));
            s.store_ad_value(171, A::add_scaled_product(A::div(s.ad_value(319), s.ad_value(170)), 1.0, s.ad_value(304), A::pow(s.ad_value(410), s.ad_value(822)), 1.0));
        }

        if (!s.b[1455]) {
            s.store_ad_value(410, A::add_scaled_product(s.ad_value(396), s.v[420], s.ad_value(408), s.ad_value(400), s.v[420]));
            s.store_ad_value(171, A::add_scaled_product(A::div(s.ad_value(820), s.ad_value(170)), 1.0, s.ad_value(819), A::pow(s.ad_value(410), s.ad_value(822)), 1.0));
        }

        s.store_offset(412, 171, 1.0);

        s.store_scaled_add_ad(412, A::offset(s.ad_value(412), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(412), (-1.0)), A::offset(s.ad_value(412), (-1.0))), ((0.25 * p.p604) * p.p604))), 0.5);

        s.store_div(412, 412, 215);

        s.store_offset_ad(360, A::div_scaled_product(s.ad_value(719), s.ad_value(153), 1.0, s.ad_value(351), 1.0), 1e-6);

        s.b[1457] = (s.v[360] < 40.0);
        s.v[1457] = if s.b[1457] { 1.0 } else { 0.0 };

        if s.b[1457] {
            s.store_add_ad_lhs(200, A::div_scaled_inputs(s.ad_value(427), 0.5, A::offset(A::cosh(s.ad_value(360)), (-1.0)), 1.0), 718);
        }

        if (!s.b[1457]) {
            s.store_ad_value(200, A::add_scaled_product(s.ad_value(718), 1.0, s.ad_value(427), A::limited_exp_scaled_input(s.ad_value(360), -1.0), 1.0));
        }

        s.b[1458] = (s.v[720] > 0.0);
        s.v[1458] = if s.b[1458] { 1.0 } else { 0.0 };

        if s.b[1458] {
            s.store_offset_ad(201, A::div_scaled_product(s.ad_value(720), s.ad_value(399), 1.0, s.ad_value(217), 1.0), 1.0);
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

        if s.b[1460] {
            s.copy_ad(169, 204);
            s.store_div_ad_rhs(171, 169, A::add(s.ad_value(210), s.ad_value(169)));
            s.store_mul_ad_product_lhs(203, A::div(s.ad_value(169), s.ad_value(200)), s.ad_value(171), 201);
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
            s.store_ad_value(169, A::add_scaled_product(s.ad_value(795), 1.0, s.ad_value(793), s.ad_value(399), 1.0));
        }

        if s.b[1461] {
            let assign24430_ad_e43832: A = {
                if (!((1.0 + (((s.v[126] - s.v[390]) / s.v[169]) / (s.v[210] + s.v[217]))) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((1.0 + (((s.v[126] - s.v[390]) / s.v[169]) / (s.v[210] + s.v[217]))) > 1e-38) {
                            A::ln(A::offset(A::div(A::div(A::sub(s.ad_value(126), s.ad_value(390)), s.ad_value(169)), A::add(s.ad_value(210), s.ad_value(217))), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_mul_ad(206, s.ad_value(169), assign24430_ad_e43832, 1.0);
        }

        if (!s.b[1461]) {
            s.store_scalar(206, 1.0);
        }

        s.store_mul(205, 205, 206);

        s.store_scaled_div(218, 422, 415, 2.0);

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

        let assign24510_ad_e43924: A = A::div(A::offset(A::limited_exp(A::mul(s.ad_value(169), {
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
})), 1.0), s.ad_value(225));
        s.store_ad_value(209, assign24510_ad_e43924);

        s.store_ad_value(209, A::add_scaled_product(s.ad_value(209), 1.0, A::mul3_scaled_output(s.ad_value(424), s.ad_value(399), s.ad_value(402), 0.5), s.ad_value(402), 1.0));

        s.store_add_ad_rhs(168, 241, A::div(s.ad_value(242), A::add_scaled_inputs(s.ad_value(399), 1.0, s.ad_value(181), 2.0)));

        s.store_mul3_lhs(169, 168, 402, 402);

        s.store_offset(170, 169, ((1.0) + ((-0.001))));

        s.store_offset_ad(171, A::add_scaled_inputs(s.ad_value(170), 0.5, A::sqrt(A::offset(A::square(s.ad_value(170)), 0.004)), 0.5), (-1.0));

        s.store_scaled_offset_ad(214, A::sqrt(A::offset(s.ad_value(171), 1.0)), 1.0, 0.5);

        s.store_mul(209, 209, 214);

        s.store_scaled_add_ad(209, A::offset(s.ad_value(209), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(209), (-1.0)), A::offset(s.ad_value(209), (-1.0))), ((0.25 * p.p453) * p.p453))), 0.5);

        s.store_div_ad_rhs(169, 236, A::add_scaled_product(s.ad_value(181), 2.0, A::max_from_scalar(0.0, A::add(s.ad_value(237), A::mul3(s.ad_value(294), s.ad_value(402), s.ad_value(402)))), s.ad_value(399), 1.0));

        s.store_limited_exp_neg_input(366, 169);

        s.b[1463] = (p.p61 == 2.0);
        s.v[1463] = if s.b[1463] { 1.0 } else { 0.0 };

        if s.b[1463] {
            let assign24630_ad_e44082: A = {
                if (!((s.v[293] + (s.v[240] * s.v[127])) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::add_scaled_product(s.ad_value(293), 1.0, s.ad_value(240), s.ad_value(127), 1.0), 0.5, A::sqrt(A::offset(A::mul(A::add_scaled_product(s.ad_value(293), 1.0, s.ad_value(240), s.ad_value(127), 1.0), A::add_scaled_product(s.ad_value(293), 1.0, s.ad_value(240), s.ad_value(127), 1.0)), ((4.0 * 1e-6) * 1e-6))), 0.5)
                } else {
                    {
                        if ((s.v[293] + (s.v[240] * s.v[127])) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::add_scaled_product(s.ad_value(293), 1.0, s.ad_value(240), s.ad_value(127), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad_value(168, assign24630_ad_e44082);
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
            s.store_ad_value(220, A::div_scaled_product(s.ad_value(336), s.ad_value(412), 2.0, s.ad_value(414), 1.0));
        }

        if (!s.b[1464]) {
            s.store_ad_value(220, A::div_scaled_product(s.ad_value(336), s.ad_value(412), 2.0, s.ad_value(416), 1.0));
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

        s.store_div_ad_lhs(213, A::offset(A::pow(A::add(s.ad_value(696), s.ad_value(168)), s.ad_value(169)), 1.0), 225);

        s.store_scaled_add_ad(881, A::offset(s.ad_value(881), 0.1), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(881), (-0.1)), A::offset(s.ad_value(881), (-0.1))), ((0.25 * 0.001) * 0.001))), 0.5);

        s.store_mul(213, 213, 881);

        s.b[1466] = (s.v[794] != 0.0);
        s.v[1466] = if s.b[1466] { 1.0 } else { 0.0 };

        if s.b[1466] {
            let assign24810_ad_e44264: A = {
                if (!((1.0 + (((s.v[126] - s.v[390]) / s.v[794]) / (s.v[210] + s.v[221]))) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((1.0 + (((s.v[126] - s.v[390]) / s.v[794]) / (s.v[210] + s.v[221]))) > 1e-38) {
                            A::ln(A::offset(A::div(A::div(A::sub(s.ad_value(126), s.ad_value(390)), s.ad_value(794)), A::add(s.ad_value(210), s.ad_value(221))), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_mul_ad(207, s.ad_value(794), assign24810_ad_e44264, 1.0);
        }

        if (!s.b[1466]) {
            s.store_scalar(207, 1.0);
        }

        s.store_mul3_affine_lhs(140, 640, 894, (-1.60219e-19), 0.0, 156);

        s.store_div_ad_rhs(131, 339, A::add(s.ad_value(339), s.ad_value(399)));

        s.store_ad_value(123, A::add_scaled_product(s.ad_value(399), 1.0, A::sub_from_scalar(2.0, s.ad_value(131)), s.ad_value(181), 1.0));

        s.store_mul(122, 123, 402);

        s.b[1467] = (p.p64 == 0.0);
        s.v[1467] = if s.b[1467] { 1.0 } else { 0.0 };

        s.b[1468] = (p.p64 == 1.0);
        s.v[1468] = if s.b[1468] { 1.0 } else { 0.0 };

        s.b[1469] = (p.p64 == 2.0);
        s.v[1469] = if s.b[1469] { 1.0 } else { 0.0 };

        if s.b[1467] {
            s.store_offset_mul(172, 711, 399, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_ad_rhs(168, 169, A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01)), 0.5);
            s.store_mul_ad_product_lhs(197, s.ad_value(194), A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), 189);
            s.store_offset_mul_ad(188, A::div_scaled_product(s.ad_value(183), s.ad_value(123), s.v[115], A::mul(s.ad_value(411), s.ad_value(209)), 1.0), s.ad_value(197), 1.0);
        }

        if (s.b[1468] && (!s.b[1467])) {
            s.store_scalar(197, 0.0);
            s.store_scalar(188, 1.0);
            s.store_ad_value(170, A::add_scaled_product(s.ad_value(479), (-1.0), s.ad_value(114), A::voltage(ctx, nodes, Some(11), Some(8)), 1.0));
            s.store_sqrt_square_offset(171, 170, 0.1);
            s.store_scaled_add(482, 170, 171, 0.5);
            s.store_offset_mul(172, 711, 482, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_ad_rhs(168, 169, A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01)), 0.5);
            s.store_mul_ad_affine_product_rhs(174, 853, s.ad_value(425), A::powf(A::offset(A::square(A::voltage(ctx, nodes, Some(2), Some(8))), 1e-6), (0.5 * p.p921)), 1.0, 1.0);
            s.store_ad_value(170, A::add_scaled_product(s.ad_value(479), (-1.0), s.ad_value(114), A::voltage(ctx, nodes, Some(11), Some(9)), 1.0));
            s.store_sqrt_square_offset(171, 170, 0.1);
            s.store_scaled_add(483, 170, 171, 0.5);
            s.store_offset_mul(172, 712, 483, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_ad_rhs(168, 169, A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01)), 0.5);
            s.store_mul_ad_affine_product_rhs(174, 852, s.ad_value(426), A::powf(A::offset(A::square(A::voltage(ctx, nodes, Some(0), Some(9))), 1e-6), (0.5 * p.p922)), 1.0, 1.0);
        }

        if (s.b[1469] && (!(s.b[1467] || s.b[1468]))) {
            s.store_offset_mul(172, 711, 399, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_ad_rhs(168, 169, A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01)), 0.5);
            s.store_mul_add_ad_rhs(197, 194, A::add_scaled_product(s.ad_value(190), 1.0, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189), 1.0), s.ad_value(191));
            s.store_offset_mul_ad(188, A::div_scaled_product(s.ad_value(183), s.ad_value(123), s.v[115], A::mul(s.ad_value(411), s.ad_value(209)), 1.0), s.ad_value(197), 1.0);
        }

        s.store_ad_value(124, A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(183), s.ad_value(122), s.ad_value(205), s.v[115]), s.ad_value(366), s.ad_value(371), 1.0, A::mul3(s.ad_value(411), s.ad_value(209), s.ad_value(188)), 1.0));

        s.store_scale(124, 124, p.p25);

        s.b[1470] = (p.p67 == 1.0);
        s.v[1470] = if s.b[1470] { 1.0 } else { 0.0 };

        if s.b[1470] {
            s.store_div_ad_lhs(341, A::add_scaled_inputs(s.ad_value(403), 2.0, s.ad_value(181), 1.0), 213);
            s.store_add_ad_rhs(138, 403, A::div_scaled_product(s.ad_value(405), s.ad_value(405), 1.0, s.ad_value(341), 6.0));
            s.store_ad_value(137, A::add_scaled_product(s.ad_value(403), (-0.5), s.ad_value(405), A::sub_from_scalar(1.0, A::mul(A::div(s.ad_value(405), s.ad_value(341)), A::offset(A::div_scaled_inputs(s.ad_value(405), 1.0, s.ad_value(341), 5.0), 1.0))), ((-1.0 / (6.0)) * (-0.5))));
        }

        if (!s.b[1470]) {
            s.store_div_ad_lhs(341, A::add_scaled_inputs(s.ad_value(399), 2.0, s.ad_value(181), 1.0), 213);
            s.store_add_ad_rhs(138, 399, A::div_scaled_product(s.ad_value(402), s.ad_value(402), 1.0, s.ad_value(341), 6.0));
            s.store_ad_value(137, A::add_scaled_product(s.ad_value(399), (-0.5), s.ad_value(402), A::sub_from_scalar(1.0, A::mul(A::div(s.ad_value(402), s.ad_value(341)), A::offset(A::div_scaled_inputs(s.ad_value(402), 1.0, s.ad_value(341), 5.0), 1.0))), ((-1.0 / (6.0)) * (-0.5))));
        }

        s.store_div_from_scalar(208, 1.0, 207);

        s.store_ad_value(138, A::add_scaled_products(s.ad_value(208), s.ad_value(138), 1.0, A::offset(s.ad_value(207), (-1.0)), s.ad_value(393), 1.0));

        s.store_ad_value(137, A::add_scaled_products(A::square(s.ad_value(208)), s.ad_value(137), 1.0, A::sub(s.ad_value(207), s.ad_value(208)), s.ad_value(393), 0.5));

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
        }

    }

    pub(super) fn stamp_reactive_block_21(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1472] {
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

        if (s.b[1475] && s.b[1476]) {
            s.store_scaled_mul(169, 159, 114, s.v[115]);
            s.store_mul_voltage_ad(170, s.ad_value(114), ctx, nodes, Some(10), Some(6));
            s.store_offset_sub(168, 170, 518, 0.02);
            s.store_scaled_sub_ad_rhs(510, 168, A::sqrt(A::offset(A::square(s.ad_value(168)), (4.0 * 0.02))), 0.5);
            s.store_mul_ad_rhs(498, 169, A::add_scaled_products(s.ad_value(648), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(170), 1.0, s.ad_value(518), (-1.0), s.ad_value(510), -1.0), 1.0, s.ad_value(651), A::offset(A::sqrt(A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(510), 4.0, s.ad_value(651), 1.0))), (-1.0)), (-0.5)), 1.0, s.ad_value(646), s.ad_value(170), 1.0));
            s.store_mul_voltage_ad(170, s.ad_value(114), ctx, nodes, Some(10), Some(5));
            s.store_offset_sub(168, 170, 518, 0.02);
            s.store_scaled_sub_ad_rhs(511, 168, A::sqrt(A::offset(A::square(s.ad_value(168)), (4.0 * 0.02))), 0.5);
            s.store_mul_ad_rhs(499, 169, A::add_scaled_products(s.ad_value(649), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(170), 1.0, s.ad_value(518), (-1.0), s.ad_value(511), -1.0), 1.0, s.ad_value(652), A::offset(A::sqrt(A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(511), 4.0, s.ad_value(652), 1.0))), (-1.0)), (-0.5)), 1.0, s.ad_value(647), s.ad_value(170), 1.0));
        }

        if (s.b[1475] && (!s.b[1476])) {
            s.store_scaled_mul(169, 159, 114, s.v[115]);
            s.store_mul_voltage_ad(170, s.ad_value(114), ctx, nodes, Some(13), Some(6));
            s.store_offset_sub(168, 170, 518, 0.02);
            s.store_scaled_sub_ad_rhs(510, 168, A::sqrt(A::offset(A::square(s.ad_value(168)), (4.0 * 0.02))), 0.5);
            s.store_mul_ad_rhs(498, 169, A::add_scaled_products(s.ad_value(648), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(170), 1.0, s.ad_value(518), (-1.0), s.ad_value(510), -1.0), 1.0, s.ad_value(651), A::offset(A::sqrt(A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(510), 4.0, s.ad_value(651), 1.0))), (-1.0)), (-0.5)), 1.0, s.ad_value(646), s.ad_value(170), 1.0));
            s.store_mul_voltage_ad(170, s.ad_value(114), ctx, nodes, Some(14), Some(5));
            s.store_offset_sub(168, 170, 518, 0.02);
            s.store_scaled_sub_ad_rhs(511, 168, A::sqrt(A::offset(A::square(s.ad_value(168)), (4.0 * 0.02))), 0.5);
            s.store_mul_ad_rhs(499, 169, A::add_scaled_products(s.ad_value(649), A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(170), 1.0, s.ad_value(518), (-1.0), s.ad_value(511), -1.0), 1.0, s.ad_value(652), A::offset(A::sqrt(A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(511), 4.0, s.ad_value(652), 1.0))), (-1.0)), (-0.5)), 1.0, s.ad_value(647), s.ad_value(170), 1.0));
        }

        s.b[1477] = (p.p78 == 0.0);
        s.v[1477] = if s.b[1477] { 1.0 } else { 0.0 };

        s.b[1478] = (p.p76 != 2.0);
        s.v[1478] = if s.b[1478] { 1.0 } else { 0.0 };

        if (s.b[1477] && s.b[1478]) {
            s.store_scale(169, 159, s.v[115]);
            s.store_mul_ad_product_rhs(500, 169, s.ad_value(643), A::voltage(ctx, nodes, Some(10), Some(6)));
            s.store_mul_ad_product_rhs(501, 169, s.ad_value(642), A::voltage(ctx, nodes, Some(10), Some(5)));
            s.store_add(505, 498, 500);
            s.store_add(506, 499, 501);
        }

        if (s.b[1477] && (!s.b[1478])) {
            s.store_scale(169, 159, s.v[115]);
            s.store_mul_ad_product_rhs(500, 169, s.ad_value(643), A::voltage(ctx, nodes, Some(13), Some(6)));
            s.store_mul_ad_product_rhs(501, 169, s.ad_value(642), A::voltage(ctx, nodes, Some(14), Some(5)));
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
            s.store_mul_ad(178, A::div_scaled_inputs(s.ad_value(239), 1.0, s.ad_value(181), 2.0), A::sqrt(s.ad_value(179)));
            s.store_scale(168, 178, 0.5);
            s.store_ad_value(170, A::div_scaled_inputs(A::offset(s.ad_value(132), (-p.p144)), -1.0, s.ad_value(179), 1.0));
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
            s.store_div_ad_lhs(172, A::sub(s.ad_value(170), s.ad_value(340)), 178);
            s.store_sub_ad_lhs(175, A::square(s.ad_value(172)), 340);
        }

        if ((s.b[1484] && (!s.b[1485])) && s.b[1486]) {
            let assign26500_ad_e46045: A = A::neg({
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
            s.store_ad_value(340, assign26500_ad_e46045);
        }

        if ((s.b[1484] && (!s.b[1485])) && (!s.b[1486])) {
            s.store_limited_exp_scaled_input(341, 340, (-1.2));
            s.store_sub_ad_lhs(172, A::sqrt(A::add_scaled_inputs3(A::offset(s.ad_value(170), (-1.0)), 1.0, s.ad_value(341), 1.0, A::square(s.ad_value(168)), 1.0)), 168);
            s.store_add_ad(340, A::sub_from_scalar(1.0, s.ad_value(341)), A::square(s.ad_value(172)));
            s.store_offset_ad(175, A::limited_exp_scaled_input(s.ad_value(340), -1.0), (-1.0));
        }

        if s.b[1484] {
            s.store_sqrt_add(176, 175, 340);
        }

        s.b[1487] = (s.v[340] > 1e-15);
        s.v[1487] = if s.b[1487] { 1.0 } else { 0.0 };

        if (s.b[1484] && s.b[1487]) {
            s.store_ad_value(344, A::add_scaled_product(A::sub(s.ad_value(170), s.ad_value(340)), -1.0, s.ad_value(178), s.ad_value(176), 1.0));
            s.store_sub_from_scalar_ad(345, 1.0, A::div_scaled_product(s.ad_value(178), s.ad_value(175), 0.5, s.ad_value(176), 1.0));
            s.store_sub_ad_rhs(177, 340, A::div(s.ad_value(344), s.ad_value(345)));
        }

        s.b[1488] = (s.v[340] < (-1e-15));
        s.v[1488] = if s.b[1488] { 1.0 } else { 0.0 };

        if ((s.b[1484] && (!s.b[1487])) && s.b[1488]) {
            s.store_ad_value(344, A::add_scaled_product(A::sub(s.ad_value(170), s.ad_value(340)), -1.0, s.ad_value(178), s.ad_value(176), (-1.0)));
            s.store_offset_ad(345, A::div_scaled_product(s.ad_value(178), s.ad_value(175), 0.5, s.ad_value(176), 1.0), 1.0);
            s.store_sub_ad_rhs(177, 340, A::div(s.ad_value(344), s.ad_value(345)));
        }

        if ((s.b[1484] && (!s.b[1487])) && (!s.b[1488])) {
            s.store_scalar(177, 0.0);
        }

        if s.b[1484] {
            s.store_mul_ad_product_lhs(906, s.ad_value(178), A::limited_exp_scaled_input(s.ad_value(177), (-1.0 / (2.0))), 179);
            s.store_abs_voltage(915, ctx, nodes, Some(7), Some(6));
            s.store_mul_div_from_scalar_lhs(916, (2.0 * p.p454), 416, 397);
            s.store_scale(917, 916, p.p1);
            s.store_scalar(920, (1.0 / p.p530));
            s.store_add_scaled_inputs(175, 906, p.p491, 182, (2.0 * p.p491));
            s.store_ad_value(918, A::div_scaled_product(s.ad_value(917), s.ad_value(175), 1.0, A::add(s.ad_value(917), s.ad_value(175)), 1.0));
        }

        if s.b[1484] {
            let assign26720_ad_e46327: A = {
                if (!((s.v[918] - 0.001) < ((-10000.0) * 1e-5))) {
                    A::add_scaled_inputs(A::offset(s.ad_value(918), (-0.001)), 0.5, A::sqrt(A::offset(A::mul(A::offset(s.ad_value(918), (-0.001)), A::offset(s.ad_value(918), (-0.001))), ((4.0 * 1e-5) * 1e-5))), 0.5)
                } else {
                    {
                        if ((s.v[918] - 0.001) < ((-10000.0) * 1e-5)) {
                            A::div_from_scalar(((-1e-5) * 1e-5), A::offset(s.ad_value(918), (-0.001)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(918, assign26720_ad_e46327, 0.001);
        }

        if s.b[1484] {
            s.store_powf_ad(176, A::offset(A::div(s.ad_value(915), s.ad_value(918)), 1e-6), p.p530);
            s.store_pow_ad(177, A::offset(s.ad_value(176), 1.0), s.ad_value(920));
            s.store_min_ad(919, A::div(s.ad_value(915), s.ad_value(177)), s.ad_value(915));
            s.store_scalar(239, 1e-6);
            s.store_mul_ad(178, A::div_scaled_inputs(s.ad_value(239), 1.0, s.ad_value(181), 2.0), A::sqrt(s.ad_value(179)));
            s.store_scale(168, 178, 0.5);
            s.store_ad_value(170, A::div_scaled_inputs(A::offset(A::add(s.ad_value(133), s.ad_value(919)), (-p.p143)), -1.0, s.ad_value(179), 1.0));
        }

        s.b[1489] = ((s.v[170] * s.v[179]) > (s.v[166] + (s.v[178] * (((s.v[166] * s.v[179])) as f64).sqrt())));
        s.v[1489] = if s.b[1489] { 1.0 } else { 0.0 };

        if (s.b[1484] && s.b[1489]) {
            s.store_sub_ad_lhs(169, A::sqrt(A::add(A::offset(s.ad_value(170), (-1.0)), A::square(s.ad_value(168)))), 168);
        }

    }

    pub(super) fn stamp_reactive_block_22(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1484] && s.b[1489]) {
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
            s.store_div_ad_lhs(172, A::sub(s.ad_value(170), s.ad_value(340)), 178);
            s.store_sub_ad_lhs(175, A::square(s.ad_value(172)), 340);
        }

        if ((s.b[1484] && (!s.b[1489])) && s.b[1490]) {
            let assign26890_ad_e46539: A = A::neg({
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
            s.store_ad_value(340, assign26890_ad_e46539);
        }

        if ((s.b[1484] && (!s.b[1489])) && (!s.b[1490])) {
            s.store_limited_exp_scaled_input(341, 340, (-1.2));
            s.store_sub_ad_lhs(172, A::sqrt(A::add_scaled_inputs3(A::offset(s.ad_value(170), (-1.0)), 1.0, s.ad_value(341), 1.0, A::square(s.ad_value(168)), 1.0)), 168);
            s.store_add_ad(340, A::sub_from_scalar(1.0, s.ad_value(341)), A::square(s.ad_value(172)));
            s.store_offset_ad(175, A::limited_exp_scaled_input(s.ad_value(340), -1.0), (-1.0));
        }

        if s.b[1484] {
            s.store_sqrt_add(176, 175, 340);
        }

        s.b[1491] = (s.v[340] > 1e-15);
        s.v[1491] = if s.b[1491] { 1.0 } else { 0.0 };

        if (s.b[1484] && s.b[1491]) {
            s.store_ad_value(344, A::add_scaled_product(A::sub(s.ad_value(170), s.ad_value(340)), -1.0, s.ad_value(178), s.ad_value(176), 1.0));
            s.store_sub_from_scalar_ad(345, 1.0, A::div_scaled_product(s.ad_value(178), s.ad_value(175), 0.5, s.ad_value(176), 1.0));
            s.store_sub_ad_rhs(177, 340, A::div(s.ad_value(344), s.ad_value(345)));
        }

        s.b[1492] = (s.v[340] < (-1e-15));
        s.v[1492] = if s.b[1492] { 1.0 } else { 0.0 };

        if ((s.b[1484] && (!s.b[1491])) && s.b[1492]) {
            s.store_ad_value(344, A::add_scaled_product(A::sub(s.ad_value(170), s.ad_value(340)), -1.0, s.ad_value(178), s.ad_value(176), (-1.0)));
            s.store_offset_ad(345, A::div_scaled_product(s.ad_value(178), s.ad_value(175), 0.5, s.ad_value(176), 1.0), 1.0);
            s.store_sub_ad_rhs(177, 340, A::div(s.ad_value(344), s.ad_value(345)));
        }

        if ((s.b[1484] && (!s.b[1491])) && (!s.b[1492])) {
            s.store_scalar(177, 0.0);
        }

        if s.b[1484] {
            s.store_mul_ad_product_lhs(907, s.ad_value(178), A::limited_exp_scaled_input(s.ad_value(177), (-1.0 / (2.0))), 179);
            s.store_sub(911, 906, 907);
            s.store_scaled_add(910, 906, 907, 0.5);
            s.store_div_ad_lhs(341, A::add_scaled_inputs(s.ad_value(910), 2.0, s.ad_value(181), 1.0), 209);
            s.store_add_ad_rhs(905, 910, A::div_scaled_product(s.ad_value(911), s.ad_value(911), 1.0, s.ad_value(341), 6.0));
            s.store_ad_value(909, A::add_scaled_product(s.ad_value(910), 0.5, s.ad_value(911), A::sub_from_scalar(1.0, A::mul(A::div(s.ad_value(911), s.ad_value(341)), A::offset(A::div_scaled_inputs(s.ad_value(911), 1.0, s.ad_value(341), 5.0), 1.0))), ((-1.0 / (6.0)) * 0.5)));
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
            s.store_div_ad_lhs(168, A::add_scaled_product(s.ad_value(259), 1.0, s.ad_value(260), s.ad_value(153), 1.0), 153);
        }

        s.b[1495] = ((s.v[168] <= 0.0) || (s.v[248] <= 0.0));
        s.v[1495] = if s.b[1495] { 1.0 } else { 0.0 };

        if (s.b[1494] && (!s.b[1495])) {
            s.store_ad_value(169, A::div_scaled_inputs(s.ad_value(248), -1.0, A::offset(s.ad_value(202), 1e-30), 1.0));
        }

        s.b[1496] = (p.p71 == 2.0);
        s.v[1496] = if s.b[1496] { 1.0 } else { 0.0 };

        if ((!s.b[1494]) && s.b[1496]) {
            s.store_div_ad_lhs(493, A::add_scaled_product(s.ad_value(261), 1.0, s.ad_value(262), s.ad_value(153), 1.0), 153);
        }

        s.b[1497] = (s.v[493] <= 0.0);
        s.v[1497] = if s.b[1497] { 1.0 } else { 0.0 };

        if (((!s.b[1494]) && s.b[1496]) && (!s.b[1497])) {
            s.store_mul(168, 783, 153);
            s.store_ad_value(169, A::div_scaled_product(s.ad_value(249), s.ad_value(168), 1.0, A::offset(s.ad_value(168), 1.0), 1.0));
        }

        if (((!s.b[1494]) && s.b[1496]) && (!s.b[1497])) {
            let assign27350_ad_e47023: A = {
                if (!((s.v[786] * s.v[348]) < ((-10000.0) * p.p1441))) {
                    A::add_scaled_product(A::sqrt(A::offset(A::mul3(s.ad_value(786), s.ad_value(348), A::mul(s.ad_value(786), s.ad_value(348))), ((4.0 * p.p1441) * p.p1441))), 0.5, s.ad_value(786), s.ad_value(348), 0.5)
                } else {
                    {
                        if ((s.v[786] * s.v[348]) < ((-10000.0) * p.p1441)) {
                            A::div_from_scalar(((-p.p1441) * p.p1441), A::mul(s.ad_value(786), s.ad_value(348)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_offset_ad(168, 1.0, assign27350_ad_e47023, 1.0);
        }

        if (((!s.b[1494]) && s.b[1496]) && (!s.b[1497])) {
            s.store_add(171, 168, 787);
        }

        if (((!s.b[1494]) && s.b[1496]) && (!s.b[1497])) {
            let assign27370_ad_e47094: A = {
                if (!((s.v[348] * s.v[171]) < ((-10000.0) * p.p1442))) {
                    A::add_scaled_product(A::sqrt(A::offset(A::mul3(s.ad_value(348), s.ad_value(171), A::mul(s.ad_value(348), s.ad_value(171))), ((4.0 * p.p1442) * p.p1442))), 0.5, s.ad_value(348), s.ad_value(171), 0.5)
                } else {
                    {
                        if ((s.v[348] * s.v[171]) < ((-10000.0) * p.p1442)) {
                            A::div_from_scalar(((-p.p1442) * p.p1442), A::mul(s.ad_value(348), s.ad_value(171)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad_value(170, assign27370_ad_e47094);
        }

        if (((!s.b[1494]) && s.b[1496]) && (!s.b[1497])) {
            s.store_div_from_scalar_offset_ad(171, 1.0, A::mul(s.ad_value(788), s.ad_value(126)), 1.0);
            s.store_mul3_lhs(491, 169, 170, 171);
            s.store_mul_sub_from_scalar_ad_rhs(490, 491, 1.0, A::div(s.ad_value(784), s.ad_value(153)));
            s.store_sub(489, 126, 490);
            s.store_add_ad(168, A::add_scaled_product(s.ad_value(782), 1.0, s.ad_value(781), s.ad_value(489), 1.0), A::mul3(s.ad_value(780), s.ad_value(489), s.ad_value(489)));
            s.store_sqrt_square_offset(169, 168, 1e-10);
        }

        s.b[1498] = (p.p69 != 0.0);
        s.v[1498] = if s.b[1498] { 1.0 } else { 0.0 };

        if s.b[1498] {
            s.store_div_ad_lhs(169, A::div(A::sub(s.ad_value(399), s.ad_value(725)), s.ad_value(726)), 179);
        }

        if s.b[1498] {
            let assign27490_ad_e47360: A = A::add(A::offset(A::add_scaled_product(s.ad_value(243), 1.0, s.ad_value(723), s.ad_value(399), (-1.0)), (((-(-p.p1110))) + ((-1e-6)))), A::sqrt(A::offset(A::mul(A::offset(A::add_scaled_product(s.ad_value(243), 1.0, s.ad_value(723), s.ad_value(399), (-1.0)), (((-(-p.p1110))) + ((-1e-6)))), A::offset(A::add_scaled_product(s.ad_value(243), 1.0, s.ad_value(723), s.ad_value(399), (-1.0)), (((-(-p.p1110))) + ((-1e-6))))), (-((4.0 * (-p.p1110)) * 1e-6)))));
            s.store_offset_scaled_ad(170, assign27490_ad_e47360, 0.5, (-p.p1110));
        }

        if s.b[1498] {
            s.store_offset_mul(171, 724, 399, 1.0);
            s.store_scaled_mul(172, 170, 171, ((-982222000000.0) * p.p1109));
            s.store_limited_exp(174, 172);
            s.store_scalar(175, 3.75956e-7);
            s.store_ad_value(468, A::add_scaled_inputs3(s.ad_value(167), 1.0, s.ad_value(146), (-0.5), s.ad_value(166), -1.0));
            s.store_sub(168, 468, 497);
            s.store_div_ad_lhs(169, A::div(s.ad_value(168), s.ad_value(731)), 179);
        }

        s.b[1499] = (p.p61 != 0.0);
        s.v[1499] = if s.b[1499] { 1.0 } else { 0.0 };

        if (s.b[1498] && s.b[1499]) {
            s.copy_ad(466, 904);
        }

        s.b[1500] = (s.v[468] <= 0.0);
        s.v[1500] = if s.b[1500] { 1.0 } else { 0.0 };

        if ((s.b[1498] && (!s.b[1499])) && s.b[1500]) {
            s.store_scaled_add_ad(466, A::offset(s.ad_value(168), (-0.02)), A::sqrt(A::add_scaled_product(s.ad_value(468), (-0.08), A::offset(s.ad_value(168), (-0.02)), A::offset(s.ad_value(168), (-0.02)), 1.0)), 0.5);
        }

        if ((s.b[1498] && (!s.b[1499])) && (!s.b[1500])) {
            s.store_scaled_add_ad(466, A::offset(s.ad_value(168), (-0.02)), A::sqrt(A::add_scaled_product(s.ad_value(468), 0.08, A::offset(s.ad_value(168), (-0.02)), A::offset(s.ad_value(168), (-0.02)), 1.0)), 0.5);
        }

        if s.b[1498] {
            let assign27650_ad_e47590: A = A::add(A::offset(A::add_scaled_product(s.ad_value(244), 1.0, s.ad_value(729), s.ad_value(466), (-1.0)), (((-(-p.p1111))) + ((-1e-6)))), A::sqrt(A::offset(A::mul(A::offset(A::add_scaled_product(s.ad_value(244), 1.0, s.ad_value(729), s.ad_value(466), (-1.0)), (((-(-p.p1111))) + ((-1e-6)))), A::offset(A::add_scaled_product(s.ad_value(244), 1.0, s.ad_value(729), s.ad_value(466), (-1.0)), (((-(-p.p1111))) + ((-1e-6))))), (-((4.0 * (-p.p1111)) * 1e-6)))));
            s.store_offset_scaled_ad(170, assign27650_ad_e47590, 0.5, (-p.p1111));
        }

        if s.b[1498] {
            s.store_offset_mul(171, 730, 466, 1.0);
            s.store_scaled_mul(172, 170, 171, ((-745669000000.0) * p.p1109));
            s.store_limited_exp(174, 172);
            s.store_scalar(175, 4.97232e-7);
        }

        s.b[1501] = (p.p68 != 0.0);
        s.v[1501] = if s.b[1501] { 1.0 } else { 0.0 };

        if s.b[1501] {
            let assign27730_ad_e47693: A = A::add(A::offset(A::add_scaled_product(s.ad_value(245), 1.0, s.ad_value(734), s.ad_value(399), (-1.0)), (((-(-p.p1112))) + ((-1e-6)))), A::sqrt(A::offset(A::mul(A::offset(A::add_scaled_product(s.ad_value(245), 1.0, s.ad_value(734), s.ad_value(399), (-1.0)), (((-(-p.p1112))) + ((-1e-6)))), A::offset(A::add_scaled_product(s.ad_value(245), 1.0, s.ad_value(734), s.ad_value(399), (-1.0)), (((-(-p.p1112))) + ((-1e-6))))), (-((4.0 * (-p.p1112)) * 1e-6)))));
            s.store_offset_scaled_ad(169, assign27730_ad_e47693, 0.5, (-p.p1112));
        }

        if s.b[1501] {
            s.store_offset_mul(170, 735, 399, 1.0);
            s.store_mul3_affine_lhs(171, 485, 169, (-p.p1109), 0.0, 170);
            s.store_mul_limited_exp_rhs(172, 399, 171);
            s.store_add_ad(174, A::add_scaled_inputs(s.ad_value(497), 1.0, s.ad_value(127), 0.5), A::add_scaled_inputs(s.ad_value(521), 0.5, s.ad_value(522), 0.5));
            s.store_offset_sqrt_ad(473, A::offset(A::square(s.ad_value(390)), 0.01), (-0.1));
            s.store_mul(169, 736, 473);
            s.store_limited_exp_neg_input(474, 169);
            s.store_offset_add(171, 169, 474, (((-1.0)) + (0.0001)));
            s.store_offset_sub_from_scalar_ad(172, 1.0, A::mul(A::offset(s.ad_value(169), 1.0), s.ad_value(474)), 0.0001);
            s.store_offset_square(174, 169, 0.0002);
            s.store_sub(168, 134, 479);
            s.store_sqrt_square_offset(482, 168, 0.0001);
        }

        s.b[1502] = (p.p82 == 1.0);
        s.v[1502] = if s.b[1502] { 1.0 } else { 0.0 };

        if (s.b[1501] && s.b[1502]) {
            let assign27900_ad_e47905: A = {
                if (!((s.v[246] - (s.v[739] * s.v[482])) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::add_scaled_product(s.ad_value(246), 1.0, s.ad_value(739), s.ad_value(482), (-1.0)), 0.5, A::sqrt(A::offset(A::mul(A::add_scaled_product(s.ad_value(246), 1.0, s.ad_value(739), s.ad_value(482), (-1.0)), A::add_scaled_product(s.ad_value(246), 1.0, s.ad_value(739), s.ad_value(482), (-1.0))), ((4.0 * 1e-6) * 1e-6))), 0.5)
                } else {
                    {
                        if ((s.v[246] - (s.v[739] * s.v[482])) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::add_scaled_product(s.ad_value(246), 1.0, s.ad_value(739), s.ad_value(482), (-1.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad_value(169, assign27900_ad_e47905);
        }

        s.b[1503] = (s.v[740] < 0.01);
        s.v[1503] = if s.b[1503] { 1.0 } else { 0.0 };

        if ((s.b[1501] && s.b[1502]) && s.b[1503]) {
            s.store_scalar(740, 0.01);
        }

        if (s.b[1501] && (!s.b[1502])) {
            s.store_ad_value(169, A::add_scaled_product(s.ad_value(246), 1.0, s.ad_value(739), s.ad_value(482), (-1.0)));
        }

        if s.b[1501] {
            s.store_offset_mul(170, 740, 482, 1.0);
            s.store_mul_ad_lhs(171, A::mul3_scaled_output(s.ad_value(485), s.ad_value(742), s.ad_value(169), (-p.p1109)), 170);
            s.store_limited_exp(172, 171);
            s.store_sub(168, 136, 479);
            s.store_sqrt_square_offset(483, 168, 0.0001);
        }

        s.b[1505] = (p.p82 == 1.0);
        s.v[1505] = if s.b[1505] { 1.0 } else { 0.0 };

        if (s.b[1501] && s.b[1505]) {
            let assign28030_ad_e48068: A = {
                if (!((s.v[247] - (s.v[745] * s.v[483])) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::add_scaled_product(s.ad_value(247), 1.0, s.ad_value(745), s.ad_value(483), (-1.0)), 0.5, A::sqrt(A::offset(A::mul(A::add_scaled_product(s.ad_value(247), 1.0, s.ad_value(745), s.ad_value(483), (-1.0)), A::add_scaled_product(s.ad_value(247), 1.0, s.ad_value(745), s.ad_value(483), (-1.0))), ((4.0 * 1e-6) * 1e-6))), 0.5)
                } else {
                    {
                        if ((s.v[247] - (s.v[745] * s.v[483])) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::add_scaled_product(s.ad_value(247), 1.0, s.ad_value(745), s.ad_value(483), (-1.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad_value(169, assign28030_ad_e48068);
        }

        s.b[1506] = (s.v[746] < 0.01);
        s.v[1506] = if s.b[1506] { 1.0 } else { 0.0 };

        if ((s.b[1501] && s.b[1505]) && s.b[1506]) {
            s.store_scalar(746, 0.01);
        }

        if (s.b[1501] && (!s.b[1505])) {
            s.store_ad_value(169, A::add_scaled_product(s.ad_value(247), 1.0, s.ad_value(745), s.ad_value(483), (-1.0)));
        }

        if s.b[1501] {
            s.store_offset_mul(170, 746, 483, 1.0);
            s.store_mul_ad_lhs(171, A::mul3_scaled_output(s.ad_value(485), s.ad_value(742), s.ad_value(169), (-p.p1109)), 170);
            s.store_limited_exp(172, 171);
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
            s.store_div_ad_lhs(169, A::add_scaled_inputs3(s.ad_value(136), -1.0, s.ad_value(750), (-1.0), s.ad_value(479), 1.0), 168);
        }

    }

    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1508] && (!s.b[1509])) {
            s.store_ad_value(169, {
                if (!(s.v[169] < ((-10000.0) * 0.01))) {
                    A::add_scaled_inputs(s.ad_value(169), 0.5, A::sqrt(A::offset(A::square(s.ad_value(169)), ((4.0 * 0.01) * 0.01))), 0.5)
                } else {
                    {
                        if (s.v[169] < ((-10000.0) * 0.01)) {
                            A::div_from_scalar(((-0.01) * 0.01), s.ad_value(169))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.b[1508] && (!s.b[1509])) {
            s.store_div_ad_rhs(170, 252, A::offset(s.ad_value(169), 0.001));
            s.store_pow_ad(171, s.ad_value(169), s.ad_value(751));
        }

        s.b[1510] = (p.p61 != 0.0);
        s.v[1510] = if s.b[1510] { 1.0 } else { 0.0 };

        if ((s.b[1508] && (!s.b[1509])) && s.b[1510]) {
            s.store_mul3_affine_lhs(172, 522, 522, -1.0, 0.0, 522);
            s.store_offset_add_ad(173, s.ad_value(749), A::abs(s.ad_value(172)), 1e-5);
        }

        if ((s.b[1508] && (!s.b[1509])) && s.b[1510]) {
            let assign28240_ad_e48333: A = {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(172), s.ad_value(173)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(172), s.ad_value(173)), A::div(s.ad_value(172), s.ad_value(173))), ((4.0 * 1e-6) * 1e-6))), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(174, assign28240_ad_e48333, (-1e-6));
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
            let assign28290_ad_e48464: A = {
                if (!(((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(254, 754, assign28290_ad_e48464);
        }

        if ((s.b[1508] && s.b[1511]) && s.b[1512]) {
            s.store_div_ad_lhs(169, A::add_scaled_inputs3(A::add_scaled_product(A::mul3(s.ad_value(753), s.ad_value(136), s.ad_value(136)), 1.0, s.ad_value(254), s.ad_value(136), (-1.0)), 1.0, s.ad_value(755), (-1.0), s.ad_value(479), 1.0), 179);
            s.store_mul_ad(170, A::mul3(s.ad_value(752), s.ad_value(158), s.ad_value(141)), A::limited_exp(s.ad_value(169)));
            s.store_mul3_affine_lhs(172, 522, 522, -1.0, 0.0, 522);
            s.store_offset_add_ad(173, s.ad_value(749), A::abs(s.ad_value(172)), 1e-5);
        }

        if ((s.b[1508] && s.b[1511]) && s.b[1512]) {
            let assign28340_ad_e48583: A = {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(172), s.ad_value(173)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(172), s.ad_value(173)), A::div(s.ad_value(172), s.ad_value(173))), ((4.0 * 1e-6) * 1e-6))), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(174, assign28340_ad_e48583, (-1e-6));
        }

        if ((s.b[1508] && s.b[1511]) && s.b[1512]) {
            s.store_ad_value(175, A::add_scaled_product(s.ad_value(175), 1.0, s.ad_value(170), s.ad_value(174), 1.0));
        }

        if ((s.b[1508] && s.b[1511]) && (!s.b[1512])) {
            let assign28360_ad_e48678: A = {
                if (!(((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(254, 754, assign28360_ad_e48678);
        }

        if ((s.b[1508] && s.b[1511]) && (!s.b[1512])) {
            s.store_div_ad_lhs(169, A::add_scaled_inputs3(A::add_scaled_product(A::mul3(s.ad_value(753), s.ad_value(136), s.ad_value(136)), 1.0, s.ad_value(254), s.ad_value(136), (-1.0)), 1.0, s.ad_value(755), (-1.0), s.ad_value(479), 1.0), 179);
            s.store_mul_ad(170, A::mul3(s.ad_value(752), s.ad_value(158), s.ad_value(141)), A::limited_exp(s.ad_value(169)));
            s.store_ad_value(175, A::add_scaled_product(s.ad_value(175), 1.0, s.ad_value(170), s.ad_value(135), 1.0));
        }

        s.b[1513] = (((p.p61 != 0.0) && ((p.p70 == 2.0) || (p.p70 == 3.0))) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0)));
        s.v[1513] = if s.b[1513] { 1.0 } else { 0.0 };

        if (s.b[1508] && s.b[1513]) {
            let assign28410_ad_e48832: A = {
                if (!(((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(255, 757, assign28410_ad_e48832);
        }

        s.b[1514] = ((s.v[756] <= 0.0) || (s.v[255] <= 0.0));
        s.v[1514] = if s.b[1514] { 1.0 } else { 0.0 };

        if ((s.b[1508] && s.b[1513]) && s.b[1514]) {
            s.store_scalar(176, 0.0);
        }

        if ((s.b[1508] && s.b[1513]) && (!s.b[1514])) {
            s.store_div_ad_lhs(169, A::add_scaled_inputs3(s.ad_value(136), -1.0, s.ad_value(759), (-1.0), s.ad_value(479), 1.0), 168);
        }

        if ((s.b[1508] && s.b[1513]) && (!s.b[1514])) {
            s.store_ad_value(169, {
                if (!(s.v[169] < ((-10000.0) * 0.01))) {
                    A::add_scaled_inputs(s.ad_value(169), 0.5, A::sqrt(A::offset(A::square(s.ad_value(169)), ((4.0 * 0.01) * 0.01))), 0.5)
                } else {
                    {
                        if (s.v[169] < ((-10000.0) * 0.01)) {
                            A::div_from_scalar(((-0.01) * 0.01), s.ad_value(169))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((s.b[1508] && s.b[1513]) && (!s.b[1514])) {
            s.store_div_ad_rhs(170, 255, A::offset(s.ad_value(169), 0.001));
            s.store_pow_ad(171, s.ad_value(169), s.ad_value(760));
            s.store_mul3_affine_lhs(172, 522, 522, -1.0, 0.0, 522);
            s.store_offset_add_ad(173, s.ad_value(758), A::abs(s.ad_value(172)), 1e-5);
        }

        if ((s.b[1508] && s.b[1513]) && (!s.b[1514])) {
            let assign28500_ad_e49016: A = {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(172), s.ad_value(173)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(172), s.ad_value(173)), A::div(s.ad_value(172), s.ad_value(173))), ((4.0 * 1e-6) * 1e-6))), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(174, assign28500_ad_e49016, (-1e-6));
        }

        if ((s.b[1508] && s.b[1513]) && (!s.b[1514])) {
            s.store_mul_ad_product_lhs(176, A::mul3(s.ad_value(756), s.ad_value(896), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), 174);
        }

        s.b[1516] = ((s.v[761] <= 0.0) || (s.v[250] <= 0.0));
        s.v[1516] = if s.b[1516] { 1.0 } else { 0.0 };

        if (s.b[1508] && s.b[1516]) {
            s.store_scalar(175, 0.0);
        }

        if (s.b[1508] && (!s.b[1516])) {
            s.store_div_ad_lhs(169, A::add_scaled_inputs3(s.ad_value(134), -1.0, s.ad_value(764), (-1.0), s.ad_value(479), 1.0), 168);
        }

        if (s.b[1508] && (!s.b[1516])) {
            s.store_ad_value(169, {
                if (!(s.v[169] < ((-10000.0) * 0.01))) {
                    A::add_scaled_inputs(s.ad_value(169), 0.5, A::sqrt(A::offset(A::square(s.ad_value(169)), ((4.0 * 0.01) * 0.01))), 0.5)
                } else {
                    {
                        if (s.v[169] < ((-10000.0) * 0.01)) {
                            A::div_from_scalar(((-0.01) * 0.01), s.ad_value(169))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.b[1508] && (!s.b[1516])) {
            s.store_div_ad_rhs(170, 250, A::offset(s.ad_value(169), 0.001));
            s.store_pow_ad(171, s.ad_value(169), s.ad_value(765));
        }

        s.b[1517] = (p.p61 != 0.0);
        s.v[1517] = if s.b[1517] { 1.0 } else { 0.0 };

        if ((s.b[1508] && (!s.b[1516])) && s.b[1517]) {
            s.store_mul3_affine_lhs(172, 521, 521, -1.0, 0.0, 521);
            s.store_offset_add_ad(173, s.ad_value(763), A::abs(s.ad_value(172)), 1e-5);
        }

        if ((s.b[1508] && (!s.b[1516])) && s.b[1517]) {
            let assign28660_ad_e49242: A = {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(172), s.ad_value(173)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(172), s.ad_value(173)), A::div(s.ad_value(172), s.ad_value(173))), ((4.0 * 1e-6) * 1e-6))), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(174, assign28660_ad_e49242, (-1e-6));
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
            let assign28710_ad_e49374: A = {
                if (!(((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(253, 768, assign28710_ad_e49374);
        }

        if ((s.b[1508] && s.b[1518]) && s.b[1519]) {
            s.store_div_ad_lhs(169, A::add_scaled_inputs3(A::add_scaled_product(A::mul3(s.ad_value(767), s.ad_value(134), s.ad_value(134)), 1.0, s.ad_value(253), s.ad_value(134), (-1.0)), 1.0, s.ad_value(769), (-1.0), s.ad_value(479), 1.0), 179);
            s.store_mul_ad(170, A::mul3(s.ad_value(766), s.ad_value(158), s.ad_value(141)), A::limited_exp(s.ad_value(169)));
            s.store_mul3_affine_lhs(172, 521, 521, -1.0, 0.0, 521);
            s.store_offset_add_ad(173, s.ad_value(763), A::abs(s.ad_value(172)), 1e-5);
        }

        if ((s.b[1508] && s.b[1518]) && s.b[1519]) {
            let assign28760_ad_e49493: A = {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(172), s.ad_value(173)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(172), s.ad_value(173)), A::div(s.ad_value(172), s.ad_value(173))), ((4.0 * 1e-6) * 1e-6))), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(174, assign28760_ad_e49493, (-1e-6));
        }

        if ((s.b[1508] && s.b[1518]) && s.b[1519]) {
            s.store_ad_value(175, A::add_scaled_product(s.ad_value(175), 1.0, s.ad_value(170), s.ad_value(174), 1.0));
        }

        if ((s.b[1508] && s.b[1518]) && (!s.b[1519])) {
            let assign28780_ad_e49588: A = {
                if (!(((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(253, 768, assign28780_ad_e49588);
        }

        if ((s.b[1508] && s.b[1518]) && (!s.b[1519])) {
            s.store_div_ad_lhs(169, A::add_scaled_inputs3(A::add_scaled_product(A::mul3(s.ad_value(767), s.ad_value(134), s.ad_value(134)), 1.0, s.ad_value(253), s.ad_value(134), (-1.0)), 1.0, s.ad_value(769), (-1.0), s.ad_value(479), 1.0), 179);
            s.store_mul_ad(170, A::mul3(s.ad_value(766), s.ad_value(158), s.ad_value(141)), A::limited_exp(s.ad_value(169)));
            s.store_ad_value(175, A::add_scaled_product(s.ad_value(175), 1.0, s.ad_value(170), s.ad_value(135), -1.0));
        }

        s.b[1520] = (((p.p61 != 0.0) && ((p.p70 == 2.0) || (p.p70 == 3.0))) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0)));
        s.v[1520] = if s.b[1520] { 1.0 } else { 0.0 };

        if (s.b[1508] && s.b[1520]) {
            let assign28830_ad_e49743: A = {
                if (!(((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(251, 771, assign28830_ad_e49743);
        }

        s.b[1521] = ((s.v[770] <= 0.0) || (s.v[251] <= 0.0));
        s.v[1521] = if s.b[1521] { 1.0 } else { 0.0 };

        if ((s.b[1508] && s.b[1520]) && s.b[1521]) {
            s.store_scalar(176, 0.0);
        }

        if ((s.b[1508] && s.b[1520]) && (!s.b[1521])) {
            s.store_div_ad_lhs(169, A::add_scaled_inputs3(s.ad_value(134), -1.0, s.ad_value(773), (-1.0), s.ad_value(479), 1.0), 168);
        }

        if ((s.b[1508] && s.b[1520]) && (!s.b[1521])) {
            s.store_ad_value(169, {
                if (!(s.v[169] < ((-10000.0) * 0.01))) {
                    A::add_scaled_inputs(s.ad_value(169), 0.5, A::sqrt(A::offset(A::square(s.ad_value(169)), ((4.0 * 0.01) * 0.01))), 0.5)
                } else {
                    {
                        if (s.v[169] < ((-10000.0) * 0.01)) {
                            A::div_from_scalar(((-0.01) * 0.01), s.ad_value(169))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((s.b[1508] && s.b[1520]) && (!s.b[1521])) {
            s.store_div_ad_rhs(170, 251, A::offset(s.ad_value(169), 0.001));
            s.store_pow_ad(171, s.ad_value(169), s.ad_value(774));
            s.store_mul3_affine_lhs(172, 521, 521, -1.0, 0.0, 521);
            s.store_offset_add_ad(173, s.ad_value(772), A::abs(s.ad_value(172)), 1e-5);
        }

        if ((s.b[1508] && s.b[1520]) && (!s.b[1521])) {
            let assign28920_ad_e49927: A = {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(172), s.ad_value(173)), 0.5, A::sqrt(A::offset(A::mul(A::div(s.ad_value(172), s.ad_value(173)), A::div(s.ad_value(172), s.ad_value(173))), ((4.0 * 1e-6) * 1e-6))), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(174, assign28920_ad_e49927, (-1e-6));
        }

        if ((s.b[1508] && s.b[1520]) && (!s.b[1521])) {
            s.store_mul_ad_product_lhs(176, A::mul3(s.ad_value(770), s.ad_value(896), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), 174);
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
            s.store_ad_value(170, A::add_scaled_product(s.ad_value(542), 1.0, s.ad_value(541), A::sub(s.ad_value(521), s.ad_value(543)), 1.0));
        }

        s.b[1526] = (s.v[521] <= s.v[546]);
        s.v[1526] = if s.b[1526] { 1.0 } else { 0.0 };

        if (((s.b[1523] && s.b[1524]) && (!s.b[1525])) && s.b[1526]) {
            s.store_div(168, 521, 539);
            s.store_div_ad_lhs(169, A::offset(s.ad_value(521), p.p1626), 539);
        }

    }

    pub(super) fn stamp_reactive_block_24(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1523] && s.b[1524]) && (!s.b[1525])) && s.b[1526]) {
            s.store_limited_exp_neg_input(170, 169);
        }

        s.b[1527] = (s.v[281] > 0.0);
        s.v[1527] = if s.b[1527] { 1.0 } else { 0.0 };

        s.b[1528] = ((p.p1643 - s.v[521]) < (p.p1643 * 0.001));
        s.v[1528] = if s.b[1528] { 1.0 } else { 0.0 };

        if ((s.b[1523] && s.b[1527]) && s.b[1528]) {
            s.store_div_ad_lhs(168, A::div_scaled_inputs(s.ad_value(521), -1.0, s.ad_value(180), 1.0), 287);
            s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));
        }

        if ((s.b[1523] && s.b[1527]) && (!s.b[1528])) {
            s.store_div_ad_lhs(168, A::div_scaled_inputs(s.ad_value(521), -1.0, s.ad_value(180), 1.0), 287);
            s.store_offset_limited_exp_ad(169, A::div_scaled_inputs(s.ad_value(168), p.p1643, A::sub_from_scalar(p.p1643, s.ad_value(521)), 1.0), (-1.0));
        }

        s.b[1529] = (s.v[283] > 0.0);
        s.v[1529] = if s.b[1529] { 1.0 } else { 0.0 };

        s.b[1530] = ((p.p1645 - s.v[521]) < (p.p1645 * 0.001));
        s.v[1530] = if s.b[1530] { 1.0 } else { 0.0 };

        if ((s.b[1523] && s.b[1529]) && s.b[1530]) {
            s.store_div_ad_lhs(168, A::div_scaled_inputs(s.ad_value(521), -1.0, s.ad_value(180), 1.0), 289);
            s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));
        }

        if ((s.b[1523] && s.b[1529]) && (!s.b[1530])) {
            s.store_div_ad_lhs(168, A::div_scaled_inputs(s.ad_value(521), -1.0, s.ad_value(180), 1.0), 289);
            s.store_offset_limited_exp_ad(169, A::div_scaled_inputs(s.ad_value(168), p.p1645, A::sub_from_scalar(p.p1645, s.ad_value(521)), 1.0), (-1.0));
        }

        s.b[1531] = (s.v[285] > 0.0);
        s.v[1531] = if s.b[1531] { 1.0 } else { 0.0 };

        s.b[1532] = ((p.p1647 - s.v[521]) < (p.p1647 * 0.001));
        s.v[1532] = if s.b[1532] { 1.0 } else { 0.0 };

        if ((s.b[1523] && s.b[1531]) && s.b[1532]) {
            s.store_div_ad_lhs(168, A::div_scaled_inputs(s.ad_value(521), -1.0, s.ad_value(180), 1.0), 291);
            s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));
        }

        if ((s.b[1523] && s.b[1531]) && (!s.b[1532])) {
            s.store_div_ad_lhs(168, A::div_scaled_inputs(s.ad_value(521), -1.0, s.ad_value(180), 1.0), 291);
            s.store_offset_limited_exp_ad(169, A::div_scaled_inputs(s.ad_value(168), p.p1647, A::sub_from_scalar(p.p1647, s.ad_value(521)), 1.0), (-1.0));
        }

        s.b[1533] = (s.v[538] > 0.0);
        s.v[1533] = if s.b[1533] { 1.0 } else { 0.0 };

        s.b[1534] = (s.v[522] < s.v[550]);
        s.v[1534] = if s.b[1534] { 1.0 } else { 0.0 };

        if ((s.b[1523] && s.b[1533]) && s.b[1534]) {
            s.store_div(168, 522, 540);
            s.store_offset_limited_exp(169, 168, (-1.0));
            s.store_ad_value(170, A::add_scaled_product(s.ad_value(549), 1.0, s.ad_value(548), A::sub(s.ad_value(522), s.ad_value(550)), 1.0));
        }

        s.b[1535] = (s.v[522] <= s.v[553]);
        s.v[1535] = if s.b[1535] { 1.0 } else { 0.0 };

        if (((s.b[1523] && s.b[1533]) && (!s.b[1534])) && s.b[1535]) {
            s.store_div(168, 522, 540);
            s.store_div_ad_lhs(169, A::offset(s.ad_value(522), p.p1627), 540);
            s.store_limited_exp_neg_input(170, 169);
        }

        s.b[1536] = (s.v[282] > 0.0);
        s.v[1536] = if s.b[1536] { 1.0 } else { 0.0 };

        s.b[1537] = ((p.p1644 - s.v[522]) < (p.p1644 * 0.001));
        s.v[1537] = if s.b[1537] { 1.0 } else { 0.0 };

        if ((s.b[1523] && s.b[1536]) && s.b[1537]) {
            s.store_div_ad_lhs(168, A::div_scaled_inputs(s.ad_value(522), -1.0, s.ad_value(180), 1.0), 288);
            s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));
        }

        if ((s.b[1523] && s.b[1536]) && (!s.b[1537])) {
            s.store_div_ad_lhs(168, A::div_scaled_inputs(s.ad_value(522), -1.0, s.ad_value(180), 1.0), 288);
            s.store_offset_limited_exp_ad(169, A::div_scaled_inputs(s.ad_value(168), p.p1644, A::sub_from_scalar(p.p1644, s.ad_value(522)), 1.0), (-1.0));
        }

        s.b[1538] = (s.v[284] > 0.0);
        s.v[1538] = if s.b[1538] { 1.0 } else { 0.0 };

        s.b[1539] = ((p.p1646 - s.v[522]) < (p.p1646 * 0.001));
        s.v[1539] = if s.b[1539] { 1.0 } else { 0.0 };

        if ((s.b[1523] && s.b[1538]) && s.b[1539]) {
            s.store_div_ad_lhs(168, A::div_scaled_inputs(s.ad_value(522), -1.0, s.ad_value(180), 1.0), 290);
            s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));
        }

        if ((s.b[1523] && s.b[1538]) && (!s.b[1539])) {
            s.store_div_ad_lhs(168, A::div_scaled_inputs(s.ad_value(522), -1.0, s.ad_value(180), 1.0), 290);
            s.store_offset_limited_exp_ad(169, A::div_scaled_inputs(s.ad_value(168), p.p1646, A::sub_from_scalar(p.p1646, s.ad_value(522)), 1.0), (-1.0));
        }

        s.b[1540] = (s.v[286] > 0.0);
        s.v[1540] = if s.b[1540] { 1.0 } else { 0.0 };

        s.b[1541] = ((p.p1648 - s.v[522]) < (p.p1648 * 0.001));
        s.v[1541] = if s.b[1541] { 1.0 } else { 0.0 };

        if ((s.b[1523] && s.b[1540]) && s.b[1541]) {
            s.store_div_ad_lhs(168, A::div_scaled_inputs(s.ad_value(522), -1.0, s.ad_value(180), 1.0), 292);
            s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));
        }

        if ((s.b[1523] && s.b[1540]) && (!s.b[1541])) {
            s.store_div_ad_lhs(168, A::div_scaled_inputs(s.ad_value(522), -1.0, s.ad_value(180), 1.0), 292);
            s.store_offset_limited_exp_ad(169, A::div_scaled_inputs(s.ad_value(168), p.p1648, A::sub_from_scalar(p.p1648, s.ad_value(522)), 1.0), (-1.0));
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
            s.store_sub_from_scalar_ad(1547, 1.0, A::div(s.ad_value(557), s.ad_value(269)));
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
            s.store_sub_from_scalar_ad(1547, 1.0, A::div(A::sub(s.ad_value(521), s.ad_value(557)), s.ad_value(558)));
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
            s.store_add_ad_rhs(530, 1549, A::mul3_scaled_output(s.ad_value(558), s.ad_value(523), A::sub_from_scalar(1.0, A::mul(s.ad_value(1547), s.ad_value(1548))), (p.p1602 * 1.0 / ((1.0 - p.p1608)))));
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
            s.store_mul_ad_product_rhs(530, 269, s.ad_value(523), A::add(s.ad_value(1545), s.ad_value(1546)));
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
            s.store_sub_from_scalar_ad(1569, 1.0, A::div(s.ad_value(559), s.ad_value(270)));
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
            s.store_sub_from_scalar_ad(1569, 1.0, A::div(A::sub(s.ad_value(521), s.ad_value(559)), s.ad_value(560)));
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
            s.store_add_ad_rhs(531, 1571, A::mul3_scaled_output(s.ad_value(560), s.ad_value(524), A::sub_from_scalar(1.0, A::mul(s.ad_value(1569), s.ad_value(1570))), (p.p1604 * 1.0 / ((1.0 - p.p1610)))));
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

    }

    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
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
            s.store_mul_ad_product_rhs(531, 270, s.ad_value(524), A::add(s.ad_value(1567), s.ad_value(1568)));
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
            s.store_sub_from_scalar_ad(1591, 1.0, A::div(s.ad_value(561), s.ad_value(271)));
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
            s.store_sub_from_scalar_ad(1591, 1.0, A::div(A::sub(s.ad_value(521), s.ad_value(561)), s.ad_value(562)));
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
            s.store_add_ad_rhs(532, 1593, A::mul3_scaled_output(s.ad_value(562), s.ad_value(525), A::sub_from_scalar(1.0, A::mul(s.ad_value(1591), s.ad_value(1592))), (p.p1606 * 1.0 / ((1.0 - p.p1612)))));
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
            s.store_mul_ad_product_rhs(532, 271, s.ad_value(525), A::add(s.ad_value(1589), s.ad_value(1590)));
        }

        if (s.b[1523] && (!s.b[1594])) {
            s.store_scalar(532, 0.0);
        }

        if s.b[1523] {
            s.store_ad_value(529, A::add_scaled_inputs3(s.ad_value(530), 1.0, s.ad_value(531), 1.0, s.ad_value(532), 1.0));
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
            s.store_sub_from_scalar_ad(1613, 1.0, A::div(s.ad_value(563), s.ad_value(272)));
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
            s.store_sub_from_scalar_ad(1613, 1.0, A::div(A::sub(s.ad_value(522), s.ad_value(563)), s.ad_value(564)));
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
            s.store_add_ad_rhs(534, 1615, A::mul3_scaled_output(s.ad_value(564), s.ad_value(526), A::sub_from_scalar(1.0, A::mul(s.ad_value(1613), s.ad_value(1614))), (p.p1603 * 1.0 / ((1.0 - p.p1609)))));
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
            s.store_mul_ad_product_rhs(534, 272, s.ad_value(526), A::add(s.ad_value(1611), s.ad_value(1612)));
        }

        if (s.b[1523] && (!s.b[1616])) {
            s.store_scalar(534, 0.0);
        }

        s.b[1638] = (s.v[527] > 0.0);
        s.v[1638] = if s.b[1638] { 1.0 } else { 0.0 };

        if (s.b[1523] && s.b[1638]) {
            s.store_div(1630, 522, 273);
        }

    }

    pub(super) fn stamp_reactive_block_26(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
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
            s.store_sub_from_scalar_ad(1635, 1.0, A::div(s.ad_value(565), s.ad_value(273)));
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
            s.store_sub_from_scalar_ad(1635, 1.0, A::div(A::sub(s.ad_value(522), s.ad_value(565)), s.ad_value(566)));
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
            s.store_add_ad_rhs(535, 1637, A::mul3_scaled_output(s.ad_value(566), s.ad_value(527), A::sub_from_scalar(1.0, A::mul(s.ad_value(1635), s.ad_value(1636))), (p.p1605 * 1.0 / ((1.0 - p.p1611)))));
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
            s.store_mul_ad_product_rhs(535, 273, s.ad_value(527), A::add(s.ad_value(1633), s.ad_value(1634)));
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
            s.store_sub_from_scalar_ad(1657, 1.0, A::div(s.ad_value(567), s.ad_value(274)));
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
            s.store_sub_from_scalar_ad(1657, 1.0, A::div(A::sub(s.ad_value(522), s.ad_value(567)), s.ad_value(568)));
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
            s.store_add_ad_rhs(536, 1659, A::mul3_scaled_output(s.ad_value(568), s.ad_value(528), A::sub_from_scalar(1.0, A::mul(s.ad_value(1657), s.ad_value(1658))), (p.p1607 * 1.0 / ((1.0 - p.p1613)))));
        }

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
            s.store_mul_ad_product_rhs(536, 274, s.ad_value(528), A::add(s.ad_value(1655), s.ad_value(1656)));
        }

        if (s.b[1523] && (!s.b[1660])) {
            s.store_scalar(536, 0.0);
        }

        if s.b[1523] {
            s.store_ad_value(533, A::add_scaled_inputs3(s.ad_value(534), 1.0, s.ad_value(535), 1.0, s.ad_value(536), 1.0));
        }

        s.store_add_scaled_inputs(507, 529, 1.0, 521, s.v[515]);

        s.store_add_scaled_inputs(508, 533, 1.0, 522, s.v[516]);

        s.store_mul_ad_product_rhs(509, 517, s.ad_value(114), A::voltage(ctx, nodes, Some(3), Some(10)));

        s.b[1674] = (p.p61 != 0.0);
        s.v[1674] = if s.b[1674] { 1.0 } else { 0.0 };

        if s.b[1674] {
            s.store_mul_voltage_ad(170, s.ad_value(114), ctx, nodes, Some(10), Some(3));
            s.store_offset_add_ad(171, A::add_scaled_inputs3(s.ad_value(170), 1.0, s.ad_value(167), (-1.0), s.ad_value(146), 0.5), s.ad_value(166), (-p.p1529));
            s.store_offset(168, 171, 0.02);
            s.store_scaled_add_ad_rhs(512, 168, A::sqrt(A::offset(A::square(s.ad_value(168)), (4.0 * 0.02))), 0.5);
            s.store_sub_ad_rhs(509, 509, A::mul3_scaled_output(s.ad_value(156), s.ad_value(650), A::add_scaled_product(A::sub(s.ad_value(171), s.ad_value(512)), 1.0, s.ad_value(653), A::offset(A::sqrt(A::offset(A::div_scaled_inputs(s.ad_value(512), 4.0, s.ad_value(653), 1.0), 1.0)), (-1.0)), 0.5), s.v[115]));
        }

        s.store_mul_add_ad_rhs(169, 126, s.ad_value(865), A::mul3(s.ad_value(866), s.ad_value(126), s.ad_value(126)));

        s.store_ad_value(168, A::div_scaled_product3(s.ad_value(415), s.ad_value(372), s.ad_value(158), 1.0, s.ad_value(153), 1.0));

        s.store_scaled_div(579, 428, 415, 2.0);

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
            s.store_div_ad_lhs(168, A::scale_offset(s.ad_value(202), 1.0 / (s.v[578]), p.p1681), 579);
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
        }

    }

    pub(super) fn stamp_reactive_block_27(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1678] && s.b[1680]) && s.b[1682]) {
            s.store_div(171, 574, 170);
            s.store_scale(172, 171, 1.0 / (p.p1682));
            s.store_scaled_add_ad(174, A::offset(s.ad_value(172), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(172), (-1.0)), A::offset(s.ad_value(172), (-1.0))), ((0.25 * p.p1688) * p.p1688))), 0.5);
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
            s.store_mul_scaled_ad_rhs(585, 179, 1.0 / (1.60219e-19), A::add(s.ad_value(372), s.ad_value(669)));
        }

        if (s.b[1678] && s.b[1680]) {
            let assign32970_ad_e55346: A = {
                if (!(((s.v[583] + s.v[585]) / (s.v[584] + s.v[585])) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((s.v[583] + s.v[585]) / (s.v[584] + s.v[585])) > 1e-38) {
                            A::ln(A::div(A::add(s.ad_value(583), s.ad_value(585)), A::add(s.ad_value(584), s.ad_value(585))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(171, 573, assign32970_ad_e55346);
        }

        if (s.b[1678] && s.b[1680]) {
            s.store_scaled_sub(172, 583, 584, p.p1683);
            s.store_scaled_sub_ad(174, A::square(s.ad_value(583)), A::square(s.ad_value(584)), (0.5 * p.p1684));
            s.store_mul3_affine_lhs(175, 179, 124, 1.60219e-19, 0.0, 124);
            s.store_scaled_mul(176, 581, 158, (10000000000.0 * s.v[115]));
            s.store_ad_value(177, A::add_scaled_product(A::add_scaled_inputs(s.ad_value(573), 1.0, s.ad_value(584), p.p1683), 1.0, s.ad_value(584), s.ad_value(584), p.p1684));
            s.store_mul_ad(178, A::add(s.ad_value(584), s.ad_value(585)), A::add(s.ad_value(584), s.ad_value(585)));
            s.store_ad_value(586, A::add_scaled_product(A::div_scaled_product3(A::div(s.ad_value(175), s.ad_value(176)), s.ad_value(582), s.ad_value(177), 1.0, s.ad_value(178), 1.0), 1.0, A::div(s.ad_value(169), s.ad_value(170)), A::add_scaled_inputs3(s.ad_value(171), 1.0, s.ad_value(172), 1.0, s.ad_value(174), 1.0), 1.0));
            s.store_scaled_mul(340, 573, 179, 1.60219e-19);
            s.store_mul_ad_lhs(341, A::mul3_scaled_output(s.ad_value(158), s.ad_value(580), s.ad_value(585), (s.v[115] * 10000000000.0)), 585);
            s.store_mul_ad_product_lhs(587, A::div(s.ad_value(340), s.ad_value(341)), s.ad_value(124), 124);
            s.store_add(169, 587, 586);
        }

        s.b[1684] = (p.p79 == 2.0);
        s.v[1684] = if s.b[1684] { 1.0 } else { 0.0 };

        if ((s.b[1678] && (!s.b[1680])) && s.b[1684]) {
            s.store_div(169, 400, 576);
            s.store_offset_pow_ad(170, s.ad_value(169), s.ad_value(575), 1.0);
            s.store_div(171, 574, 170);
            s.store_scale(172, 171, 1.0 / (p.p1682));
            s.store_scaled_add_ad(174, A::offset(s.ad_value(172), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(172), (-1.0)), A::offset(s.ad_value(172), (-1.0))), ((0.25 * p.p1688) * p.p1688))), 0.5);
            s.store_scale(573, 174, p.p1682);
            s.store_scaled_div(589, 179, 217, 2.0);
            s.store_offset_mul(169, 589, 402, 1.0);
            s.store_offset_scaled(170, 402, p.p1685, 1.0);
        }

        s.b[1685] = ((s.v[169] > 0.0) && (s.v[170] > 0.0));
        s.v[1685] = if s.b[1685] { 1.0 } else { 0.0 };

        if (((s.b[1678] && (!s.b[1680])) && s.b[1684]) && s.b[1685]) {
            let assign33250_ad_e55734: A = A::mul({
                if (!(((s.v[392] + 0.5) / (s.v[393] + 0.5)) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((s.v[392] + 0.5) / (s.v[393] + 0.5)) > 1e-38) {
                            A::ln(A::div(A::offset(s.ad_value(392), 0.5), A::offset(s.ad_value(393), 0.5)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, A::offset(A::add(s.ad_value(392), s.ad_value(393)), 1.0));
            s.store_ad_value(171, assign33250_ad_e55734);
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
            s.store_ad_value(169, A::add_scaled_product(A::square(s.ad_value(153)), 1.0, s.ad_value(168), s.ad_value(197), 1.0));
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
            s.store_mul_ad(601, A::div(s.ad_value(393), s.ad_value(392)), A::sub_from_scalar(1.0, A::div(s.ad_value(390), s.ad_value(210))));
            s.store_mul_square_lhs(604, 209, 209);
            s.store_div_ad_rhs(602, 339, A::add(s.ad_value(339), s.ad_value(399)));
            s.store_div_ad_rhs(172, 236, A::add_scaled_product(s.ad_value(181), 2.0, A::max_from_scalar(0.0, s.ad_value(237)), s.ad_value(392), 1.0));
            s.store_limited_exp_neg_input(616, 172);
        }

        s.b[1688] = (p.p61 == 2.0);
        s.v[1688] = if s.b[1688] { 1.0 } else { 0.0 };

        if ((s.b[1687] && (!s.b[1686])) && s.b[1688]) {
            s.store_ad_value(172, {
                if (!(s.v[293] < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(s.ad_value(293), 0.5, A::sqrt(A::offset(A::square(s.ad_value(293)), ((4.0 * 1e-6) * 1e-6))), 0.5)
                } else {
                    {
                        if (s.v[293] < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), s.ad_value(293))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((s.b[1687] && (!s.b[1686])) && s.b[1688]) {
            s.store_div_ad_rhs(174, 172, A::add_scaled_product(s.ad_value(181), 2.0, A::max_from_scalar(0.0, s.ad_value(238)), s.ad_value(392), 1.0));
            s.store_sub_ad(175, A::sqrt(A::sub(s.ad_value(689), s.ad_value(370))), A::sqrt(s.ad_value(689)));
            s.store_limited_exp_ad(617, A::mul_scaled_lhs(s.ad_value(174), -1.0, s.ad_value(175)));
        }

        if ((s.b[1687] && (!s.b[1686])) && (!s.b[1688])) {
            s.store_scalar(617, 1.0);
        }

        if (s.b[1687] && (!s.b[1686])) {
            s.store_ad_value(615, A::add_scaled_product(s.ad_value(401), s.v[420], s.ad_value(407), s.ad_value(392), s.v[420]));
            s.store_pow_ad(172, A::scaled_offset(A::abs(A::div(s.ad_value(392), s.ad_value(406))), 1.0, 0.5), s.ad_value(317));
        }

        s.b[1689] = (p.p61 != 0.0);
        s.v[1689] = if s.b[1689] { 1.0 } else { 0.0 };

        if ((s.b[1687] && (!s.b[1686])) && s.b[1689]) {
            s.store_ad_value(174, A::add_scaled_product(A::div(s.ad_value(820), s.ad_value(172)), 1.0, A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(821), s.ad_value(370), 1.0), A::pow(A::abs(s.ad_value(615)), s.ad_value(822)), 1.0));
        }

        if ((s.b[1687] && (!s.b[1686])) && (!s.b[1689])) {
            s.store_ad_value(174, A::add_scaled_product(A::div(s.ad_value(820), s.ad_value(172)), 1.0, s.ad_value(819), A::pow(A::abs(s.ad_value(615)), s.ad_value(822)), 1.0));
        }

        if (s.b[1687] && (!s.b[1686])) {
            s.store_offset(618, 174, 1.0);
            s.store_scaled_add_ad(618, A::offset(s.ad_value(618), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(618), (-1.0)), A::offset(s.ad_value(618), (-1.0))), ((0.25 * p.p604) * p.p604))), 0.5);
            s.store_scale(618, 618, 1.0 / (p.p24));
            s.store_scalar(619, (1.0 + (0.25 * p.p453)));
            s.store_div_ad_rhs(612, 339, A::add(s.ad_value(339), s.ad_value(392)));
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
            s.store_scaled_add_ad_rhs(175, 174, A::sqrt(A::offset(A::square(s.ad_value(174)), 0.01)), 0.5);
            s.store_mul_ad_product_lhs(614, s.ad_value(194), A::offset(A::mul(s.ad_value(709), s.ad_value(175)), p.p908), 189);
            s.store_offset_mul_ad(620, A::div_scaled_product(s.ad_value(183), s.ad_value(613), s.v[115], A::mul(s.ad_value(618), s.ad_value(619)), 1.0), s.ad_value(614), 1.0);
        }

        if ((s.b[1687] && (!s.b[1686])) && (s.b[1691] && (!s.b[1690]))) {
            s.store_scalar(620, 1.0);
        }

        if ((s.b[1687] && (!s.b[1686])) && (s.b[1692] && (!(s.b[1690] || s.b[1691])))) {
            s.store_offset_mul(172, 711, 392, 1.0);
            s.store_div_from_scalar(174, 1.0, 172);
            s.store_scaled_add_ad_rhs(175, 174, A::sqrt(A::offset(A::square(s.ad_value(174)), 0.01)), 0.5);
            s.store_mul3_affine_lhs(614, 709, 175, 1.0, p.p908, 189);
            s.store_mul_ad_rhs(614, 194, A::add_scaled_inputs3(s.ad_value(190), 1.0, s.ad_value(191), 1.0, s.ad_value(614), 1.0));
            s.store_offset_mul_ad(620, A::div_scaled_product(s.ad_value(183), s.ad_value(613), s.v[115], A::mul(s.ad_value(618), s.ad_value(619)), 1.0), s.ad_value(614), 1.0);
        }

        if (s.b[1687] && (!s.b[1686])) {
            s.store_ad_value(603, A::div_scaled_product(A::mul3_scaled_output(s.ad_value(183), s.ad_value(392), s.ad_value(616), s.v[115]), s.ad_value(617), 1.0, A::mul3(s.ad_value(618), s.ad_value(619), s.ad_value(620)), 1.0));
            s.store_offset(172, 601, 1.0);
            s.store_sub_from_scalar(174, 1.0, 601);
            s.store_mul_ad_lhs(175, A::div_scaled_inputs(s.ad_value(602), 2.0, s.ad_value(392), 1.0), 181);
            s.store_add(176, 172, 175);
            s.store_square(605, 174);
            s.store_mul(606, 605, 174);
            s.store_mul(607, 606, 174);
            s.store_square(608, 176);
            s.store_mul(609, 608, 176);
            s.store_mul(610, 609, 176);
            s.store_mul(611, 610, 176);
            s.store_scale(621, 172, 0.5);
            s.store_scaled_div(622, 605, 176, (1.0 / (6.0)));
            s.store_mul_ad(623, A::div(s.ad_value(205), s.ad_value(209)), A::add(s.ad_value(621), s.ad_value(622)));
            s.store_div(624, 172, 608);
            s.store_ad_value(625, A::div_scaled_product(A::add_scaled_inputs(s.ad_value(172), 6.0, s.ad_value(175), 1.0), s.ad_value(605), 1.0, s.ad_value(610), 15.0));
            s.store_scaled_div(626, 607, 611, (1.0 / (9.0)));
            s.store_mul_ad_affine_product_rhs(627, 205, s.ad_value(604), A::add_scaled_inputs3(s.ad_value(624), 1.0, s.ad_value(625), (-1.0), s.ad_value(626), 1.0), 1.0 / (6.0), 0.0);
            s.store_offset_mul_ad(177, A::div_scaled_product(s.ad_value(600), s.ad_value(600), 1.0, A::offset(s.ad_value(399), p.p1716), 1.0), A::div(s.ad_value(390), s.ad_value(210)), 1.0);
            s.store_mul_ad(623, A::div(s.ad_value(205), s.ad_value(209)), A::add_scaled_products(s.ad_value(177), s.ad_value(621), 1.0, s.ad_value(169), s.ad_value(622), 1.0));
            s.store_mul_ad(627, A::mul3_scaled_output(s.ad_value(205), s.ad_value(604), s.ad_value(170), 1.0 / (6.0)), A::add_scaled_inputs3(s.ad_value(624), 1.0, s.ad_value(625), (-1.0), s.ad_value(626), 1.0));
            s.store_ad_value(632, A::div_scaled_product(A::mul3_scaled_output(A::sqrt(A::div(s.ad_value(627), s.ad_value(623))), s.ad_value(372), s.ad_value(159), s.v[115]), s.ad_value(156), 1.0, s.ad_value(603), 1.0));
        }

        s.b[1696] = (p.p73 == 2.0);
        s.v[1696] = if s.b[1696] { 1.0 } else { 0.0 };

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
            let assign34330_ad_e57040: A = {
                if (!(((1.0 + (p.p1912 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1912, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p1912, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p1912, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1912 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1912, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad_value(1039, assign34330_ad_e57040);
        }

        s.b[1713] = (p.p75 != 0.0);
        s.v[1713] = if s.b[1713] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_28(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[1711] && s.b[1712]) && s.b[1713]) {
            s.store_offset_ad(1044, A::add_scaled_inputs(A::scale_offset(s.ad_value(232), (-p.p1913), (((-(-p.p1904))) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), (-p.p1913), (((-(-p.p1904))) + ((-1e-6)))), A::scale_offset(s.ad_value(232), (-p.p1913), (((-(-p.p1904))) + ((-1e-6))))), (-((4.0 * (-p.p1904)) * 1e-6)))), 0.5), (((-p.p1904)) + (p.p1904)));
        }

        if ((s.b[1711] && s.b[1712]) && (!s.b[1713])) {
            let assign34360_ad_e57181: A = {
                if (!(((1.0 + ((-p.p1913) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + ((-p.p1913) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(1044, assign34360_ad_e57181, p.p1904);
        }

        if (s.b[1711] && s.b[1712]) {
            s.store_offset(168, 392, (-p.p1906));
            s.store_scaled_add_ad(168, A::offset(s.ad_value(168), 0.1), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(168), (-0.1)), A::offset(s.ad_value(168), (-0.1))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_ad_value(169, A::div_scaled_inputs(s.ad_value(168), (10.0 * p.p1907), A::offset(s.ad_value(168), (10.0 * p.p1907)), 1.0));
            s.store_mul_ad_rhs(1045, 1044, A::scale_offset(s.ad_value(169), p.p1905, 1.0));
        }

        if (s.b[1711] && s.b[1712]) {
            s.store_ad_value(1045, {
                if (!(s.v[1045] < ((-10000.0) * 10.0))) {
                    A::add_scaled_inputs(s.ad_value(1045), 0.5, A::sqrt(A::offset(A::square(s.ad_value(1045)), ((4.0 * 10.0) * 10.0))), 0.5)
                } else {
                    {
                        if (s.v[1045] < ((-10000.0) * 10.0)) {
                            A::div_from_scalar(((-10.0) * 10.0), s.ad_value(1045))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
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
            s.store_scaled_add_ad(171, A::offset(s.ad_value(174), (-p.p1916)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(174), (-p.p1916)), A::offset(s.ad_value(174), (-p.p1916))), ((0.25 * 0.5) * 0.5))), 0.5);
            s.store_offset_scaled(171, 171, p.p1917, 1.0);
        }

        if (s.b[1711] && s.b[1712]) {
            s.store_scaled_mul(1047, 170, 171, p.p1903);
            s.store_scaled_mul(172, 1039, 189, p.p1910);
            s.store_mul(1048, 1047, 172);
            s.store_div_ad(1050, A::powf(s.ad_value(174), (4.0 - p.p1908)), A::add_scaled_inputs(A::powf(s.ad_value(174), (4.0 - p.p1908)), 1.0, A::powf(s.ad_value(1048), (4.0 - p.p1908)), p.p1914));
            s.store_ad_value(175, A::div_scaled_product(A::powf(s.ad_value(1050), (1.0 / p.p1908)), s.ad_value(174), 1.0, s.ad_value(1048), 1.0));
        }

        s.b[1715] = (p.p1911 > 0.0);
        s.v[1715] = if s.b[1715] { 1.0 } else { 0.0 };

        s.b[1716] = (p.p1910 == 0.0);
        s.v[1716] = if s.b[1716] { 1.0 } else { 0.0 };

        if ((s.b[1711] && s.b[1715]) && s.b[1716]) {
            let assign34560_ad_e57528: A = {
                if (!(((1.0 + (p.p1912 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1912, ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), p.p1912, ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(232), p.p1912, ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1912 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1912, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_ad_value(1039, assign34560_ad_e57528);
        }

        s.b[1717] = (p.p75 != 0.0);
        s.v[1717] = if s.b[1717] { 1.0 } else { 0.0 };

        if (((s.b[1711] && s.b[1715]) && s.b[1716]) && s.b[1717]) {
            s.store_offset_ad(1044, A::add_scaled_inputs(A::scale_offset(s.ad_value(232), (-p.p1913), (((-(-p.p1904))) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), (-p.p1913), (((-(-p.p1904))) + ((-1e-6)))), A::scale_offset(s.ad_value(232), (-p.p1913), (((-(-p.p1904))) + ((-1e-6))))), (-((4.0 * (-p.p1904)) * 1e-6)))), 0.5), (((-p.p1904)) + (p.p1904)));
        }

        if (((s.b[1711] && s.b[1715]) && s.b[1716]) && (!s.b[1717])) {
            let assign34590_ad_e57673: A = {
                if (!(((1.0 + ((-p.p1913) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))), 0.5, A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))), A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6))))), ((4.0 * 0.001) * 0.001))), 0.5)
                } else {
                    {
                        if (((1.0 + ((-p.p1913) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_scale_ad(1044, assign34590_ad_e57673, p.p1904);
        }

        if ((s.b[1711] && s.b[1715]) && s.b[1716]) {
            s.store_offset(168, 392, (-p.p1906));
            s.store_scaled_add_ad(168, A::offset(s.ad_value(168), 0.1), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(168), (-0.1)), A::offset(s.ad_value(168), (-0.1))), ((0.25 * 2.0) * 2.0))), 0.5);
            s.store_ad_value(169, A::div_scaled_inputs(s.ad_value(168), (10.0 * p.p1907), A::offset(s.ad_value(168), (10.0 * p.p1907)), 1.0));
            s.store_mul_ad_rhs(1045, 1044, A::scale_offset(s.ad_value(169), p.p1905, 1.0));
        }

        if ((s.b[1711] && s.b[1715]) && s.b[1716]) {
            s.store_ad_value(1045, {
                if (!(s.v[1045] < ((-10000.0) * 10.0))) {
                    A::add_scaled_inputs(s.ad_value(1045), 0.5, A::sqrt(A::offset(A::square(s.ad_value(1045)), ((4.0 * 10.0) * 10.0))), 0.5)
                } else {
                    {
                        if (s.v[1045] < ((-10000.0) * 10.0)) {
                            A::div_from_scalar(((-10.0) * 10.0), s.ad_value(1045))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((s.b[1711] && s.b[1715]) && s.b[1716]) {
            s.store_scaled_mul(170, 158, 1045, (s.v[115] * 1.60219e-19));
        }

        if (s.b[1711] && s.b[1715]) {
            s.store_scale(1046, 170, p.p1909);
            s.store_scaled_mul(172, 1039, 189, p.p1911);
            s.store_mul(1049, 1046, 172);
            s.store_abs_voltage(174, ctx, nodes, Some(6), Some(8));
            s.store_div_ad(1051, A::powf(s.ad_value(174), (4.0 - p.p1908)), A::add_scaled_inputs(A::powf(s.ad_value(174), (4.0 - p.p1908)), 1.0, A::powf(s.ad_value(1049), (4.0 - p.p1908)), p.p1915));
            s.store_ad_value(175, A::div_scaled_product(A::powf(s.ad_value(1051), (1.0 / p.p1908)), s.ad_value(174), 1.0, s.ad_value(1049), 1.0));
        }

        s.b[1723] = (p.p73 == 2.0);
        s.v[1723] = if s.b[1723] { 1.0 } else { 0.0 };

        s.b[1731] = (p.p72 == 0.0);
        s.v[1731] = if s.b[1731] { 1.0 } else { 0.0 };

        s.b[1736] = ((p.p74 != 0.0) && (p.p1791 > 0.0));
        s.v[1736] = if s.b[1736] { 1.0 } else { 0.0 };

        s.store_add_scaled_ad_lhs(339, A::div_scaled_inputs(s.ad_value(179), 10.0, s.ad_value(898), 1.0), 396, 2.0);

        s.store_mul_add_rhs(169, 179, 179, 339);

        s.store_mul_square_lhs(170, 163, 169);

        s.store_scaled_mul(171, 141, 179, ((2.0 * 1.60219e-19) * s.v[143]));

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq0_e1945, eq0_e1945_d_n0, eq0_e1945_d_n1, eq0_e1945_d_n2, eq0_e1945_d_n3, eq0_e1945_d_n4, eq0_e1945_d_n5, eq0_e1945_d_n6, eq0_e1945_d_n7, eq0_e1945_d_n8, eq0_e1945_d_n9, eq0_e1945_d_n10, eq0_e1945_d_n11, eq0_e1945_d_n12, eq0_e1945_d_n13, eq0_e1945_d_n14, eq0_e1945_d_n15, eq0_e1945_d_n16, eq0_e1945_d_b0, eq0_e1945_d_b1, eq0_e1945_d_b2, eq0_e1945_d_b3, eq0_e1945_d_b4, eq0_e1945_d_b5, eq0_e1945_d_b6, eq0_e1945_d_b7, eq0_e1945_d_b8, eq0_e1945_d_b9, eq0_e1945_d_b10, eq0_e1945_d_b11, eq0_e1945_d_b12, eq0_e1945_d_b13, eq0_e1945_d_b14, eq0_e1945_d_b15, eq0_e1945_d_b16, eq0_e1945_d_b17,) = {
    if s.b[1695] {
        let eq0_e1943: f64 = (s.v[114] * s.v[124]);
        let eq0_e1943_d_n0: f64 = ((s.dn[114][0] * s.v[124]) + (s.v[114] * s.dn[124][0]));
        let eq0_e1943_d_n1: f64 = ((s.dn[114][1] * s.v[124]) + (s.v[114] * s.dn[124][1]));
        let eq0_e1943_d_n2: f64 = ((s.dn[114][2] * s.v[124]) + (s.v[114] * s.dn[124][2]));
        let eq0_e1943_d_n3: f64 = ((s.dn[114][3] * s.v[124]) + (s.v[114] * s.dn[124][3]));
        let eq0_e1943_d_n4: f64 = ((s.dn[114][4] * s.v[124]) + (s.v[114] * s.dn[124][4]));
        let eq0_e1943_d_n5: f64 = ((s.dn[114][5] * s.v[124]) + (s.v[114] * s.dn[124][5]));
        let eq0_e1943_d_n6: f64 = ((s.dn[114][6] * s.v[124]) + (s.v[114] * s.dn[124][6]));
        let eq0_e1943_d_n7: f64 = ((s.dn[114][7] * s.v[124]) + (s.v[114] * s.dn[124][7]));
        let eq0_e1943_d_n8: f64 = ((s.dn[114][8] * s.v[124]) + (s.v[114] * s.dn[124][8]));
        let eq0_e1943_d_n9: f64 = ((s.dn[114][9] * s.v[124]) + (s.v[114] * s.dn[124][9]));
        let eq0_e1943_d_n10: f64 = ((s.dn[114][10] * s.v[124]) + (s.v[114] * s.dn[124][10]));
        let eq0_e1943_d_n11: f64 = ((s.dn[114][11] * s.v[124]) + (s.v[114] * s.dn[124][11]));
        let eq0_e1943_d_n12: f64 = ((s.dn[114][12] * s.v[124]) + (s.v[114] * s.dn[124][12]));
        let eq0_e1943_d_n13: f64 = ((s.dn[114][13] * s.v[124]) + (s.v[114] * s.dn[124][13]));
        let eq0_e1943_d_n14: f64 = ((s.dn[114][14] * s.v[124]) + (s.v[114] * s.dn[124][14]));
        let eq0_e1943_d_n15: f64 = ((s.dn[114][15] * s.v[124]) + (s.v[114] * s.dn[124][15]));
        let eq0_e1943_d_n16: f64 = ((s.dn[114][16] * s.v[124]) + (s.v[114] * s.dn[124][16]));
        let eq0_e1943_d_b0: f64 = ((s.db[114][0] * s.v[124]) + (s.v[114] * s.db[124][0]));
        let eq0_e1943_d_b1: f64 = ((s.db[114][1] * s.v[124]) + (s.v[114] * s.db[124][1]));
        let eq0_e1943_d_b2: f64 = ((s.db[114][2] * s.v[124]) + (s.v[114] * s.db[124][2]));
        let eq0_e1943_d_b3: f64 = ((s.db[114][3] * s.v[124]) + (s.v[114] * s.db[124][3]));
        let eq0_e1943_d_b4: f64 = ((s.db[114][4] * s.v[124]) + (s.v[114] * s.db[124][4]));
        let eq0_e1943_d_b5: f64 = ((s.db[114][5] * s.v[124]) + (s.v[114] * s.db[124][5]));
        let eq0_e1943_d_b6: f64 = ((s.db[114][6] * s.v[124]) + (s.v[114] * s.db[124][6]));
        let eq0_e1943_d_b7: f64 = ((s.db[114][7] * s.v[124]) + (s.v[114] * s.db[124][7]));
        let eq0_e1943_d_b8: f64 = ((s.db[114][8] * s.v[124]) + (s.v[114] * s.db[124][8]));
        let eq0_e1943_d_b9: f64 = ((s.db[114][9] * s.v[124]) + (s.v[114] * s.db[124][9]));
        let eq0_e1943_d_b10: f64 = ((s.db[114][10] * s.v[124]) + (s.v[114] * s.db[124][10]));
        let eq0_e1943_d_b11: f64 = ((s.db[114][11] * s.v[124]) + (s.v[114] * s.db[124][11]));
        let eq0_e1943_d_b12: f64 = ((s.db[114][12] * s.v[124]) + (s.v[114] * s.db[124][12]));
        let eq0_e1943_d_b13: f64 = ((s.db[114][13] * s.v[124]) + (s.v[114] * s.db[124][13]));
        let eq0_e1943_d_b14: f64 = ((s.db[114][14] * s.v[124]) + (s.v[114] * s.db[124][14]));
        let eq0_e1943_d_b15: f64 = ((s.db[114][15] * s.v[124]) + (s.v[114] * s.db[124][15]));
        let eq0_e1943_d_b16: f64 = ((s.db[114][16] * s.v[124]) + (s.v[114] * s.db[124][16]));
        let eq0_e1943_d_b17: f64 = ((s.db[114][17] * s.v[124]) + (s.v[114] * s.db[124][17]));
        (eq0_e1943, eq0_e1943_d_n0, eq0_e1943_d_n1, eq0_e1943_d_n2, eq0_e1943_d_n3, eq0_e1943_d_n4, eq0_e1943_d_n5, eq0_e1943_d_n6, eq0_e1943_d_n7, eq0_e1943_d_n8, eq0_e1943_d_n9, eq0_e1943_d_n10, eq0_e1943_d_n11, eq0_e1943_d_n12, eq0_e1943_d_n13, eq0_e1943_d_n14, eq0_e1943_d_n15, eq0_e1943_d_n16, eq0_e1943_d_b0, eq0_e1943_d_b1, eq0_e1943_d_b2, eq0_e1943_d_b3, eq0_e1943_d_b4, eq0_e1943_d_b5, eq0_e1943_d_b6, eq0_e1943_d_b7, eq0_e1943_d_b8, eq0_e1943_d_b9, eq0_e1943_d_b10, eq0_e1943_d_b11, eq0_e1943_d_b12, eq0_e1943_d_b13, eq0_e1943_d_b14, eq0_e1943_d_b15, eq0_e1943_d_b16, eq0_e1943_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e1945;
        let eq0_node_derivatives: [f64; 17] = [eq0_e1945_d_n0, eq0_e1945_d_n1, eq0_e1945_d_n2, eq0_e1945_d_n3, eq0_e1945_d_n4, eq0_e1945_d_n5, eq0_e1945_d_n6, eq0_e1945_d_n7, eq0_e1945_d_n8, eq0_e1945_d_n9, eq0_e1945_d_n10, eq0_e1945_d_n11, eq0_e1945_d_n12, eq0_e1945_d_n13, eq0_e1945_d_n14, eq0_e1945_d_n15, eq0_e1945_d_n16];
        let eq0_branch_derivatives: [f64; 18] = [eq0_e1945_d_b0, eq0_e1945_d_b1, eq0_e1945_d_b2, eq0_e1945_d_b3, eq0_e1945_d_b4, eq0_e1945_d_b5, eq0_e1945_d_b6, eq0_e1945_d_b7, eq0_e1945_d_b8, eq0_e1945_d_b9, eq0_e1945_d_b10, eq0_e1945_d_b11, eq0_e1945_d_b12, eq0_e1945_d_b13, eq0_e1945_d_b14, eq0_e1945_d_b15, eq0_e1945_d_b16, eq0_e1945_d_b17];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let (eq1_e1952, eq1_e1952_d_n0, eq1_e1952_d_n1, eq1_e1952_d_n2, eq1_e1952_d_n3, eq1_e1952_d_n4, eq1_e1952_d_n5, eq1_e1952_d_n6, eq1_e1952_d_n7, eq1_e1952_d_n8, eq1_e1952_d_n9, eq1_e1952_d_n10, eq1_e1952_d_n11, eq1_e1952_d_n12, eq1_e1952_d_n13, eq1_e1952_d_n14, eq1_e1952_d_n15, eq1_e1952_d_n16, eq1_e1952_d_b0, eq1_e1952_d_b1, eq1_e1952_d_b2, eq1_e1952_d_b3, eq1_e1952_d_b4, eq1_e1952_d_b5, eq1_e1952_d_b6, eq1_e1952_d_b7, eq1_e1952_d_b8, eq1_e1952_d_b9, eq1_e1952_d_b10, eq1_e1952_d_b11, eq1_e1952_d_b12, eq1_e1952_d_b13, eq1_e1952_d_b14, eq1_e1952_d_b15, eq1_e1952_d_b16, eq1_e1952_d_b17,) = {
    if (!s.b[1695]) {
        let eq1_e1950: f64 = (s.v[114] * s.v[124]);
        let eq1_e1950_d_n0: f64 = ((s.dn[114][0] * s.v[124]) + (s.v[114] * s.dn[124][0]));
        let eq1_e1950_d_n1: f64 = ((s.dn[114][1] * s.v[124]) + (s.v[114] * s.dn[124][1]));
        let eq1_e1950_d_n2: f64 = ((s.dn[114][2] * s.v[124]) + (s.v[114] * s.dn[124][2]));
        let eq1_e1950_d_n3: f64 = ((s.dn[114][3] * s.v[124]) + (s.v[114] * s.dn[124][3]));
        let eq1_e1950_d_n4: f64 = ((s.dn[114][4] * s.v[124]) + (s.v[114] * s.dn[124][4]));
        let eq1_e1950_d_n5: f64 = ((s.dn[114][5] * s.v[124]) + (s.v[114] * s.dn[124][5]));
        let eq1_e1950_d_n6: f64 = ((s.dn[114][6] * s.v[124]) + (s.v[114] * s.dn[124][6]));
        let eq1_e1950_d_n7: f64 = ((s.dn[114][7] * s.v[124]) + (s.v[114] * s.dn[124][7]));
        let eq1_e1950_d_n8: f64 = ((s.dn[114][8] * s.v[124]) + (s.v[114] * s.dn[124][8]));
        let eq1_e1950_d_n9: f64 = ((s.dn[114][9] * s.v[124]) + (s.v[114] * s.dn[124][9]));
        let eq1_e1950_d_n10: f64 = ((s.dn[114][10] * s.v[124]) + (s.v[114] * s.dn[124][10]));
        let eq1_e1950_d_n11: f64 = ((s.dn[114][11] * s.v[124]) + (s.v[114] * s.dn[124][11]));
        let eq1_e1950_d_n12: f64 = ((s.dn[114][12] * s.v[124]) + (s.v[114] * s.dn[124][12]));
        let eq1_e1950_d_n13: f64 = ((s.dn[114][13] * s.v[124]) + (s.v[114] * s.dn[124][13]));
        let eq1_e1950_d_n14: f64 = ((s.dn[114][14] * s.v[124]) + (s.v[114] * s.dn[124][14]));
        let eq1_e1950_d_n15: f64 = ((s.dn[114][15] * s.v[124]) + (s.v[114] * s.dn[124][15]));
        let eq1_e1950_d_n16: f64 = ((s.dn[114][16] * s.v[124]) + (s.v[114] * s.dn[124][16]));
        let eq1_e1950_d_b0: f64 = ((s.db[114][0] * s.v[124]) + (s.v[114] * s.db[124][0]));
        let eq1_e1950_d_b1: f64 = ((s.db[114][1] * s.v[124]) + (s.v[114] * s.db[124][1]));
        let eq1_e1950_d_b2: f64 = ((s.db[114][2] * s.v[124]) + (s.v[114] * s.db[124][2]));
        let eq1_e1950_d_b3: f64 = ((s.db[114][3] * s.v[124]) + (s.v[114] * s.db[124][3]));
        let eq1_e1950_d_b4: f64 = ((s.db[114][4] * s.v[124]) + (s.v[114] * s.db[124][4]));
        let eq1_e1950_d_b5: f64 = ((s.db[114][5] * s.v[124]) + (s.v[114] * s.db[124][5]));
        let eq1_e1950_d_b6: f64 = ((s.db[114][6] * s.v[124]) + (s.v[114] * s.db[124][6]));
        let eq1_e1950_d_b7: f64 = ((s.db[114][7] * s.v[124]) + (s.v[114] * s.db[124][7]));
        let eq1_e1950_d_b8: f64 = ((s.db[114][8] * s.v[124]) + (s.v[114] * s.db[124][8]));
        let eq1_e1950_d_b9: f64 = ((s.db[114][9] * s.v[124]) + (s.v[114] * s.db[124][9]));
        let eq1_e1950_d_b10: f64 = ((s.db[114][10] * s.v[124]) + (s.v[114] * s.db[124][10]));
        let eq1_e1950_d_b11: f64 = ((s.db[114][11] * s.v[124]) + (s.v[114] * s.db[124][11]));
        let eq1_e1950_d_b12: f64 = ((s.db[114][12] * s.v[124]) + (s.v[114] * s.db[124][12]));
        let eq1_e1950_d_b13: f64 = ((s.db[114][13] * s.v[124]) + (s.v[114] * s.db[124][13]));
        let eq1_e1950_d_b14: f64 = ((s.db[114][14] * s.v[124]) + (s.v[114] * s.db[124][14]));
        let eq1_e1950_d_b15: f64 = ((s.db[114][15] * s.v[124]) + (s.v[114] * s.db[124][15]));
        let eq1_e1950_d_b16: f64 = ((s.db[114][16] * s.v[124]) + (s.v[114] * s.db[124][16]));
        let eq1_e1950_d_b17: f64 = ((s.db[114][17] * s.v[124]) + (s.v[114] * s.db[124][17]));
        (eq1_e1950, eq1_e1950_d_n0, eq1_e1950_d_n1, eq1_e1950_d_n2, eq1_e1950_d_n3, eq1_e1950_d_n4, eq1_e1950_d_n5, eq1_e1950_d_n6, eq1_e1950_d_n7, eq1_e1950_d_n8, eq1_e1950_d_n9, eq1_e1950_d_n10, eq1_e1950_d_n11, eq1_e1950_d_n12, eq1_e1950_d_n13, eq1_e1950_d_n14, eq1_e1950_d_n15, eq1_e1950_d_n16, eq1_e1950_d_b0, eq1_e1950_d_b1, eq1_e1950_d_b2, eq1_e1950_d_b3, eq1_e1950_d_b4, eq1_e1950_d_b5, eq1_e1950_d_b6, eq1_e1950_d_b7, eq1_e1950_d_b8, eq1_e1950_d_b9, eq1_e1950_d_b10, eq1_e1950_d_b11, eq1_e1950_d_b12, eq1_e1950_d_b13, eq1_e1950_d_b14, eq1_e1950_d_b15, eq1_e1950_d_b16, eq1_e1950_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e1952;
        let eq1_node_derivatives: [f64; 17] = [eq1_e1952_d_n0, eq1_e1952_d_n1, eq1_e1952_d_n2, eq1_e1952_d_n3, eq1_e1952_d_n4, eq1_e1952_d_n5, eq1_e1952_d_n6, eq1_e1952_d_n7, eq1_e1952_d_n8, eq1_e1952_d_n9, eq1_e1952_d_n10, eq1_e1952_d_n11, eq1_e1952_d_n12, eq1_e1952_d_n13, eq1_e1952_d_n14, eq1_e1952_d_n15, eq1_e1952_d_n16];
        let eq1_branch_derivatives: [f64; 18] = [eq1_e1952_d_b0, eq1_e1952_d_b1, eq1_e1952_d_b2, eq1_e1952_d_b3, eq1_e1952_d_b4, eq1_e1952_d_b5, eq1_e1952_d_b6, eq1_e1952_d_b7, eq1_e1952_d_b8, eq1_e1952_d_b9, eq1_e1952_d_b10, eq1_e1952_d_b11, eq1_e1952_d_b12, eq1_e1952_d_b13, eq1_e1952_d_b14, eq1_e1952_d_b15, eq1_e1952_d_b16, eq1_e1952_d_b17];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq2_e1961, eq2_e1961_d_n0, eq2_e1961_d_n1, eq2_e1961_d_n2, eq2_e1961_d_n3, eq2_e1961_d_n4, eq2_e1961_d_n5, eq2_e1961_d_n6, eq2_e1961_d_n7, eq2_e1961_d_n8, eq2_e1961_d_n9, eq2_e1961_d_n10, eq2_e1961_d_n11, eq2_e1961_d_n12, eq2_e1961_d_n13, eq2_e1961_d_n14, eq2_e1961_d_n15, eq2_e1961_d_n16, eq2_e1961_d_b0, eq2_e1961_d_b1, eq2_e1961_d_b2, eq2_e1961_d_b3, eq2_e1961_d_b4, eq2_e1961_d_b5, eq2_e1961_d_b6, eq2_e1961_d_b7, eq2_e1961_d_b8, eq2_e1961_d_b9, eq2_e1961_d_b10, eq2_e1961_d_b11, eq2_e1961_d_b12, eq2_e1961_d_b13, eq2_e1961_d_b14, eq2_e1961_d_b15, eq2_e1961_d_b16, eq2_e1961_d_b17,) = {
    if s.b[1696] {
        let eq2_e1956: f64 = (s.v[114] * s.v[570]);
        let eq2_e1956_d_n0: f64 = ((s.dn[114][0] * s.v[570]) + (s.v[114] * s.dn[570][0]));
        let eq2_e1956_d_n1: f64 = ((s.dn[114][1] * s.v[570]) + (s.v[114] * s.dn[570][1]));
        let eq2_e1956_d_n2: f64 = ((s.dn[114][2] * s.v[570]) + (s.v[114] * s.dn[570][2]));
        let eq2_e1956_d_n3: f64 = ((s.dn[114][3] * s.v[570]) + (s.v[114] * s.dn[570][3]));
        let eq2_e1956_d_n4: f64 = ((s.dn[114][4] * s.v[570]) + (s.v[114] * s.dn[570][4]));
        let eq2_e1956_d_n5: f64 = ((s.dn[114][5] * s.v[570]) + (s.v[114] * s.dn[570][5]));
        let eq2_e1956_d_n6: f64 = ((s.dn[114][6] * s.v[570]) + (s.v[114] * s.dn[570][6]));
        let eq2_e1956_d_n7: f64 = ((s.dn[114][7] * s.v[570]) + (s.v[114] * s.dn[570][7]));
        let eq2_e1956_d_n8: f64 = ((s.dn[114][8] * s.v[570]) + (s.v[114] * s.dn[570][8]));
        let eq2_e1956_d_n9: f64 = ((s.dn[114][9] * s.v[570]) + (s.v[114] * s.dn[570][9]));
        let eq2_e1956_d_n10: f64 = ((s.dn[114][10] * s.v[570]) + (s.v[114] * s.dn[570][10]));
        let eq2_e1956_d_n11: f64 = ((s.dn[114][11] * s.v[570]) + (s.v[114] * s.dn[570][11]));
        let eq2_e1956_d_n12: f64 = ((s.dn[114][12] * s.v[570]) + (s.v[114] * s.dn[570][12]));
        let eq2_e1956_d_n13: f64 = ((s.dn[114][13] * s.v[570]) + (s.v[114] * s.dn[570][13]));
        let eq2_e1956_d_n14: f64 = ((s.dn[114][14] * s.v[570]) + (s.v[114] * s.dn[570][14]));
        let eq2_e1956_d_n15: f64 = ((s.dn[114][15] * s.v[570]) + (s.v[114] * s.dn[570][15]));
        let eq2_e1956_d_n16: f64 = ((s.dn[114][16] * s.v[570]) + (s.v[114] * s.dn[570][16]));
        let eq2_e1956_d_b0: f64 = ((s.db[114][0] * s.v[570]) + (s.v[114] * s.db[570][0]));
        let eq2_e1956_d_b1: f64 = ((s.db[114][1] * s.v[570]) + (s.v[114] * s.db[570][1]));
        let eq2_e1956_d_b2: f64 = ((s.db[114][2] * s.v[570]) + (s.v[114] * s.db[570][2]));
        let eq2_e1956_d_b3: f64 = ((s.db[114][3] * s.v[570]) + (s.v[114] * s.db[570][3]));
        let eq2_e1956_d_b4: f64 = ((s.db[114][4] * s.v[570]) + (s.v[114] * s.db[570][4]));
        let eq2_e1956_d_b5: f64 = ((s.db[114][5] * s.v[570]) + (s.v[114] * s.db[570][5]));
        let eq2_e1956_d_b6: f64 = ((s.db[114][6] * s.v[570]) + (s.v[114] * s.db[570][6]));
        let eq2_e1956_d_b7: f64 = ((s.db[114][7] * s.v[570]) + (s.v[114] * s.db[570][7]));
        let eq2_e1956_d_b8: f64 = ((s.db[114][8] * s.v[570]) + (s.v[114] * s.db[570][8]));
        let eq2_e1956_d_b9: f64 = ((s.db[114][9] * s.v[570]) + (s.v[114] * s.db[570][9]));
        let eq2_e1956_d_b10: f64 = ((s.db[114][10] * s.v[570]) + (s.v[114] * s.db[570][10]));
        let eq2_e1956_d_b11: f64 = ((s.db[114][11] * s.v[570]) + (s.v[114] * s.db[570][11]));
        let eq2_e1956_d_b12: f64 = ((s.db[114][12] * s.v[570]) + (s.v[114] * s.db[570][12]));
        let eq2_e1956_d_b13: f64 = ((s.db[114][13] * s.v[570]) + (s.v[114] * s.db[570][13]));
        let eq2_e1956_d_b14: f64 = ((s.db[114][14] * s.v[570]) + (s.v[114] * s.db[570][14]));
        let eq2_e1956_d_b15: f64 = ((s.db[114][15] * s.v[570]) + (s.v[114] * s.db[570][15]));
        let eq2_e1956_d_b16: f64 = ((s.db[114][16] * s.v[570]) + (s.v[114] * s.db[570][16]));
        let eq2_e1956_d_b17: f64 = ((s.db[114][17] * s.v[570]) + (s.v[114] * s.db[570][17]));
        let eq2_e1958: f64 = (-(nv15 - 0.0));
        let eq2_e1958_d_n15: f64 = (-1.0);
        let eq2_e1959: f64 = (eq2_e1956 * eq2_e1958);
        let eq2_e1959_d_n0: f64 = (eq2_e1956_d_n0 * eq2_e1958);
        let eq2_e1959_d_n1: f64 = (eq2_e1956_d_n1 * eq2_e1958);
        let eq2_e1959_d_n2: f64 = (eq2_e1956_d_n2 * eq2_e1958);
        let eq2_e1959_d_n3: f64 = (eq2_e1956_d_n3 * eq2_e1958);
        let eq2_e1959_d_n4: f64 = (eq2_e1956_d_n4 * eq2_e1958);
        let eq2_e1959_d_n5: f64 = (eq2_e1956_d_n5 * eq2_e1958);
        let eq2_e1959_d_n6: f64 = (eq2_e1956_d_n6 * eq2_e1958);
        let eq2_e1959_d_n7: f64 = (eq2_e1956_d_n7 * eq2_e1958);
        let eq2_e1959_d_n8: f64 = (eq2_e1956_d_n8 * eq2_e1958);
        let eq2_e1959_d_n9: f64 = (eq2_e1956_d_n9 * eq2_e1958);
        let eq2_e1959_d_n10: f64 = (eq2_e1956_d_n10 * eq2_e1958);
        let eq2_e1959_d_n11: f64 = (eq2_e1956_d_n11 * eq2_e1958);
        let eq2_e1959_d_n12: f64 = (eq2_e1956_d_n12 * eq2_e1958);
        let eq2_e1959_d_n13: f64 = (eq2_e1956_d_n13 * eq2_e1958);
        let eq2_e1959_d_n14: f64 = (eq2_e1956_d_n14 * eq2_e1958);
        let eq2_e1959_d_n15: f64 = ((eq2_e1956_d_n15 * eq2_e1958) + (eq2_e1956 * eq2_e1958_d_n15));
        let eq2_e1959_d_n16: f64 = (eq2_e1956_d_n16 * eq2_e1958);
        let eq2_e1959_d_b0: f64 = (eq2_e1956_d_b0 * eq2_e1958);
        let eq2_e1959_d_b1: f64 = (eq2_e1956_d_b1 * eq2_e1958);
        let eq2_e1959_d_b2: f64 = (eq2_e1956_d_b2 * eq2_e1958);
        let eq2_e1959_d_b3: f64 = (eq2_e1956_d_b3 * eq2_e1958);
        let eq2_e1959_d_b4: f64 = (eq2_e1956_d_b4 * eq2_e1958);
        let eq2_e1959_d_b5: f64 = (eq2_e1956_d_b5 * eq2_e1958);
        let eq2_e1959_d_b6: f64 = (eq2_e1956_d_b6 * eq2_e1958);
        let eq2_e1959_d_b7: f64 = (eq2_e1956_d_b7 * eq2_e1958);
        let eq2_e1959_d_b8: f64 = (eq2_e1956_d_b8 * eq2_e1958);
        let eq2_e1959_d_b9: f64 = (eq2_e1956_d_b9 * eq2_e1958);
        let eq2_e1959_d_b10: f64 = (eq2_e1956_d_b10 * eq2_e1958);
        let eq2_e1959_d_b11: f64 = (eq2_e1956_d_b11 * eq2_e1958);
        let eq2_e1959_d_b12: f64 = (eq2_e1956_d_b12 * eq2_e1958);
        let eq2_e1959_d_b13: f64 = (eq2_e1956_d_b13 * eq2_e1958);
        let eq2_e1959_d_b14: f64 = (eq2_e1956_d_b14 * eq2_e1958);
        let eq2_e1959_d_b15: f64 = (eq2_e1956_d_b15 * eq2_e1958);
        let eq2_e1959_d_b16: f64 = (eq2_e1956_d_b16 * eq2_e1958);
        let eq2_e1959_d_b17: f64 = (eq2_e1956_d_b17 * eq2_e1958);
        (eq2_e1959, eq2_e1959_d_n0, eq2_e1959_d_n1, eq2_e1959_d_n2, eq2_e1959_d_n3, eq2_e1959_d_n4, eq2_e1959_d_n5, eq2_e1959_d_n6, eq2_e1959_d_n7, eq2_e1959_d_n8, eq2_e1959_d_n9, eq2_e1959_d_n10, eq2_e1959_d_n11, eq2_e1959_d_n12, eq2_e1959_d_n13, eq2_e1959_d_n14, eq2_e1959_d_n15, eq2_e1959_d_n16, eq2_e1959_d_b0, eq2_e1959_d_b1, eq2_e1959_d_b2, eq2_e1959_d_b3, eq2_e1959_d_b4, eq2_e1959_d_b5, eq2_e1959_d_b6, eq2_e1959_d_b7, eq2_e1959_d_b8, eq2_e1959_d_b9, eq2_e1959_d_b10, eq2_e1959_d_b11, eq2_e1959_d_b12, eq2_e1959_d_b13, eq2_e1959_d_b14, eq2_e1959_d_b15, eq2_e1959_d_b16, eq2_e1959_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e1961;
        let eq2_node_derivatives: [f64; 17] = [eq2_e1961_d_n0, eq2_e1961_d_n1, eq2_e1961_d_n2, eq2_e1961_d_n3, eq2_e1961_d_n4, eq2_e1961_d_n5, eq2_e1961_d_n6, eq2_e1961_d_n7, eq2_e1961_d_n8, eq2_e1961_d_n9, eq2_e1961_d_n10, eq2_e1961_d_n11, eq2_e1961_d_n12, eq2_e1961_d_n13, eq2_e1961_d_n14, eq2_e1961_d_n15, eq2_e1961_d_n16];
        let eq2_branch_derivatives: [f64; 18] = [eq2_e1961_d_b0, eq2_e1961_d_b1, eq2_e1961_d_b2, eq2_e1961_d_b3, eq2_e1961_d_b4, eq2_e1961_d_b5, eq2_e1961_d_b6, eq2_e1961_d_b7, eq2_e1961_d_b8, eq2_e1961_d_b9, eq2_e1961_d_b10, eq2_e1961_d_b11, eq2_e1961_d_b12, eq2_e1961_d_b13, eq2_e1961_d_b14, eq2_e1961_d_b15, eq2_e1961_d_b16, eq2_e1961_d_b17];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let (eq3_e1971, eq3_e1971_d_n0, eq3_e1971_d_n1, eq3_e1971_d_n2, eq3_e1971_d_n3, eq3_e1971_d_n4, eq3_e1971_d_n5, eq3_e1971_d_n6, eq3_e1971_d_n7, eq3_e1971_d_n8, eq3_e1971_d_n9, eq3_e1971_d_n10, eq3_e1971_d_n11, eq3_e1971_d_n12, eq3_e1971_d_n13, eq3_e1971_d_n14, eq3_e1971_d_n15, eq3_e1971_d_n16, eq3_e1971_d_b0, eq3_e1971_d_b1, eq3_e1971_d_b2, eq3_e1971_d_b3, eq3_e1971_d_b4, eq3_e1971_d_b5, eq3_e1971_d_b6, eq3_e1971_d_b7, eq3_e1971_d_b8, eq3_e1971_d_b9, eq3_e1971_d_b10, eq3_e1971_d_b11, eq3_e1971_d_b12, eq3_e1971_d_b13, eq3_e1971_d_b14, eq3_e1971_d_b15, eq3_e1971_d_b16, eq3_e1971_d_b17,) = {
    if s.b[1696] {
        let eq3_e1965: f64 = (s.v[114] * s.v[571]);
        let eq3_e1965_d_n0: f64 = ((s.dn[114][0] * s.v[571]) + (s.v[114] * s.dn[571][0]));
        let eq3_e1965_d_n1: f64 = ((s.dn[114][1] * s.v[571]) + (s.v[114] * s.dn[571][1]));
        let eq3_e1965_d_n2: f64 = ((s.dn[114][2] * s.v[571]) + (s.v[114] * s.dn[571][2]));
        let eq3_e1965_d_n3: f64 = ((s.dn[114][3] * s.v[571]) + (s.v[114] * s.dn[571][3]));
        let eq3_e1965_d_n4: f64 = ((s.dn[114][4] * s.v[571]) + (s.v[114] * s.dn[571][4]));
        let eq3_e1965_d_n5: f64 = ((s.dn[114][5] * s.v[571]) + (s.v[114] * s.dn[571][5]));
        let eq3_e1965_d_n6: f64 = ((s.dn[114][6] * s.v[571]) + (s.v[114] * s.dn[571][6]));
        let eq3_e1965_d_n7: f64 = ((s.dn[114][7] * s.v[571]) + (s.v[114] * s.dn[571][7]));
        let eq3_e1965_d_n8: f64 = ((s.dn[114][8] * s.v[571]) + (s.v[114] * s.dn[571][8]));
        let eq3_e1965_d_n9: f64 = ((s.dn[114][9] * s.v[571]) + (s.v[114] * s.dn[571][9]));
        let eq3_e1965_d_n10: f64 = ((s.dn[114][10] * s.v[571]) + (s.v[114] * s.dn[571][10]));
        let eq3_e1965_d_n11: f64 = ((s.dn[114][11] * s.v[571]) + (s.v[114] * s.dn[571][11]));
        let eq3_e1965_d_n12: f64 = ((s.dn[114][12] * s.v[571]) + (s.v[114] * s.dn[571][12]));
        let eq3_e1965_d_n13: f64 = ((s.dn[114][13] * s.v[571]) + (s.v[114] * s.dn[571][13]));
        let eq3_e1965_d_n14: f64 = ((s.dn[114][14] * s.v[571]) + (s.v[114] * s.dn[571][14]));
        let eq3_e1965_d_n15: f64 = ((s.dn[114][15] * s.v[571]) + (s.v[114] * s.dn[571][15]));
        let eq3_e1965_d_n16: f64 = ((s.dn[114][16] * s.v[571]) + (s.v[114] * s.dn[571][16]));
        let eq3_e1965_d_b0: f64 = ((s.db[114][0] * s.v[571]) + (s.v[114] * s.db[571][0]));
        let eq3_e1965_d_b1: f64 = ((s.db[114][1] * s.v[571]) + (s.v[114] * s.db[571][1]));
        let eq3_e1965_d_b2: f64 = ((s.db[114][2] * s.v[571]) + (s.v[114] * s.db[571][2]));
        let eq3_e1965_d_b3: f64 = ((s.db[114][3] * s.v[571]) + (s.v[114] * s.db[571][3]));
        let eq3_e1965_d_b4: f64 = ((s.db[114][4] * s.v[571]) + (s.v[114] * s.db[571][4]));
        let eq3_e1965_d_b5: f64 = ((s.db[114][5] * s.v[571]) + (s.v[114] * s.db[571][5]));
        let eq3_e1965_d_b6: f64 = ((s.db[114][6] * s.v[571]) + (s.v[114] * s.db[571][6]));
        let eq3_e1965_d_b7: f64 = ((s.db[114][7] * s.v[571]) + (s.v[114] * s.db[571][7]));
        let eq3_e1965_d_b8: f64 = ((s.db[114][8] * s.v[571]) + (s.v[114] * s.db[571][8]));
        let eq3_e1965_d_b9: f64 = ((s.db[114][9] * s.v[571]) + (s.v[114] * s.db[571][9]));
        let eq3_e1965_d_b10: f64 = ((s.db[114][10] * s.v[571]) + (s.v[114] * s.db[571][10]));
        let eq3_e1965_d_b11: f64 = ((s.db[114][11] * s.v[571]) + (s.v[114] * s.db[571][11]));
        let eq3_e1965_d_b12: f64 = ((s.db[114][12] * s.v[571]) + (s.v[114] * s.db[571][12]));
        let eq3_e1965_d_b13: f64 = ((s.db[114][13] * s.v[571]) + (s.v[114] * s.db[571][13]));
        let eq3_e1965_d_b14: f64 = ((s.db[114][14] * s.v[571]) + (s.v[114] * s.db[571][14]));
        let eq3_e1965_d_b15: f64 = ((s.db[114][15] * s.v[571]) + (s.v[114] * s.db[571][15]));
        let eq3_e1965_d_b16: f64 = ((s.db[114][16] * s.v[571]) + (s.v[114] * s.db[571][16]));
        let eq3_e1965_d_b17: f64 = ((s.db[114][17] * s.v[571]) + (s.v[114] * s.db[571][17]));
        let eq3_e1967: f64 = (eq3_e1965 * s.v[570]);
        let eq3_e1967_d_n0: f64 = ((eq3_e1965_d_n0 * s.v[570]) + (eq3_e1965 * s.dn[570][0]));
        let eq3_e1967_d_n1: f64 = ((eq3_e1965_d_n1 * s.v[570]) + (eq3_e1965 * s.dn[570][1]));
        let eq3_e1967_d_n2: f64 = ((eq3_e1965_d_n2 * s.v[570]) + (eq3_e1965 * s.dn[570][2]));
        let eq3_e1967_d_n3: f64 = ((eq3_e1965_d_n3 * s.v[570]) + (eq3_e1965 * s.dn[570][3]));
        let eq3_e1967_d_n4: f64 = ((eq3_e1965_d_n4 * s.v[570]) + (eq3_e1965 * s.dn[570][4]));
        let eq3_e1967_d_n5: f64 = ((eq3_e1965_d_n5 * s.v[570]) + (eq3_e1965 * s.dn[570][5]));
        let eq3_e1967_d_n6: f64 = ((eq3_e1965_d_n6 * s.v[570]) + (eq3_e1965 * s.dn[570][6]));
        let eq3_e1967_d_n7: f64 = ((eq3_e1965_d_n7 * s.v[570]) + (eq3_e1965 * s.dn[570][7]));
        let eq3_e1967_d_n8: f64 = ((eq3_e1965_d_n8 * s.v[570]) + (eq3_e1965 * s.dn[570][8]));
        let eq3_e1967_d_n9: f64 = ((eq3_e1965_d_n9 * s.v[570]) + (eq3_e1965 * s.dn[570][9]));
        let eq3_e1967_d_n10: f64 = ((eq3_e1965_d_n10 * s.v[570]) + (eq3_e1965 * s.dn[570][10]));
        let eq3_e1967_d_n11: f64 = ((eq3_e1965_d_n11 * s.v[570]) + (eq3_e1965 * s.dn[570][11]));
        let eq3_e1967_d_n12: f64 = ((eq3_e1965_d_n12 * s.v[570]) + (eq3_e1965 * s.dn[570][12]));
        let eq3_e1967_d_n13: f64 = ((eq3_e1965_d_n13 * s.v[570]) + (eq3_e1965 * s.dn[570][13]));
        let eq3_e1967_d_n14: f64 = ((eq3_e1965_d_n14 * s.v[570]) + (eq3_e1965 * s.dn[570][14]));
        let eq3_e1967_d_n15: f64 = ((eq3_e1965_d_n15 * s.v[570]) + (eq3_e1965 * s.dn[570][15]));
        let eq3_e1967_d_n16: f64 = ((eq3_e1965_d_n16 * s.v[570]) + (eq3_e1965 * s.dn[570][16]));
        let eq3_e1967_d_b0: f64 = ((eq3_e1965_d_b0 * s.v[570]) + (eq3_e1965 * s.db[570][0]));
        let eq3_e1967_d_b1: f64 = ((eq3_e1965_d_b1 * s.v[570]) + (eq3_e1965 * s.db[570][1]));
        let eq3_e1967_d_b2: f64 = ((eq3_e1965_d_b2 * s.v[570]) + (eq3_e1965 * s.db[570][2]));
        let eq3_e1967_d_b3: f64 = ((eq3_e1965_d_b3 * s.v[570]) + (eq3_e1965 * s.db[570][3]));
        let eq3_e1967_d_b4: f64 = ((eq3_e1965_d_b4 * s.v[570]) + (eq3_e1965 * s.db[570][4]));
        let eq3_e1967_d_b5: f64 = ((eq3_e1965_d_b5 * s.v[570]) + (eq3_e1965 * s.db[570][5]));
        let eq3_e1967_d_b6: f64 = ((eq3_e1965_d_b6 * s.v[570]) + (eq3_e1965 * s.db[570][6]));
        let eq3_e1967_d_b7: f64 = ((eq3_e1965_d_b7 * s.v[570]) + (eq3_e1965 * s.db[570][7]));
        let eq3_e1967_d_b8: f64 = ((eq3_e1965_d_b8 * s.v[570]) + (eq3_e1965 * s.db[570][8]));
        let eq3_e1967_d_b9: f64 = ((eq3_e1965_d_b9 * s.v[570]) + (eq3_e1965 * s.db[570][9]));
        let eq3_e1967_d_b10: f64 = ((eq3_e1965_d_b10 * s.v[570]) + (eq3_e1965 * s.db[570][10]));
        let eq3_e1967_d_b11: f64 = ((eq3_e1965_d_b11 * s.v[570]) + (eq3_e1965 * s.db[570][11]));
        let eq3_e1967_d_b12: f64 = ((eq3_e1965_d_b12 * s.v[570]) + (eq3_e1965 * s.db[570][12]));
        let eq3_e1967_d_b13: f64 = ((eq3_e1965_d_b13 * s.v[570]) + (eq3_e1965 * s.db[570][13]));
        let eq3_e1967_d_b14: f64 = ((eq3_e1965_d_b14 * s.v[570]) + (eq3_e1965 * s.db[570][14]));
        let eq3_e1967_d_b15: f64 = ((eq3_e1965_d_b15 * s.v[570]) + (eq3_e1965 * s.db[570][15]));
        let eq3_e1967_d_b16: f64 = ((eq3_e1965_d_b16 * s.v[570]) + (eq3_e1965 * s.db[570][16]));
        let eq3_e1967_d_b17: f64 = ((eq3_e1965_d_b17 * s.v[570]) + (eq3_e1965 * s.db[570][17]));
        let eq3_e1969: f64 = (eq3_e1967 * (nv15 - 0.0));
        let eq3_e1969_d_n0: f64 = (eq3_e1967_d_n0 * (nv15 - 0.0));
        let eq3_e1969_d_n1: f64 = (eq3_e1967_d_n1 * (nv15 - 0.0));
        let eq3_e1969_d_n2: f64 = (eq3_e1967_d_n2 * (nv15 - 0.0));
        let eq3_e1969_d_n3: f64 = (eq3_e1967_d_n3 * (nv15 - 0.0));
        let eq3_e1969_d_n4: f64 = (eq3_e1967_d_n4 * (nv15 - 0.0));
        let eq3_e1969_d_n5: f64 = (eq3_e1967_d_n5 * (nv15 - 0.0));
        let eq3_e1969_d_n6: f64 = (eq3_e1967_d_n6 * (nv15 - 0.0));
        let eq3_e1969_d_n7: f64 = (eq3_e1967_d_n7 * (nv15 - 0.0));
        let eq3_e1969_d_n8: f64 = (eq3_e1967_d_n8 * (nv15 - 0.0));
        let eq3_e1969_d_n9: f64 = (eq3_e1967_d_n9 * (nv15 - 0.0));
        let eq3_e1969_d_n10: f64 = (eq3_e1967_d_n10 * (nv15 - 0.0));
        let eq3_e1969_d_n11: f64 = (eq3_e1967_d_n11 * (nv15 - 0.0));
        let eq3_e1969_d_n12: f64 = (eq3_e1967_d_n12 * (nv15 - 0.0));
        let eq3_e1969_d_n13: f64 = (eq3_e1967_d_n13 * (nv15 - 0.0));
        let eq3_e1969_d_n14: f64 = (eq3_e1967_d_n14 * (nv15 - 0.0));
        let eq3_e1969_d_n15: f64 = ((eq3_e1967_d_n15 * (nv15 - 0.0)) + eq3_e1967);
        let eq3_e1969_d_n16: f64 = (eq3_e1967_d_n16 * (nv15 - 0.0));
        let eq3_e1969_d_b0: f64 = (eq3_e1967_d_b0 * (nv15 - 0.0));
        let eq3_e1969_d_b1: f64 = (eq3_e1967_d_b1 * (nv15 - 0.0));
        let eq3_e1969_d_b2: f64 = (eq3_e1967_d_b2 * (nv15 - 0.0));
        let eq3_e1969_d_b3: f64 = (eq3_e1967_d_b3 * (nv15 - 0.0));
        let eq3_e1969_d_b4: f64 = (eq3_e1967_d_b4 * (nv15 - 0.0));
        let eq3_e1969_d_b5: f64 = (eq3_e1967_d_b5 * (nv15 - 0.0));
        let eq3_e1969_d_b6: f64 = (eq3_e1967_d_b6 * (nv15 - 0.0));
        let eq3_e1969_d_b7: f64 = (eq3_e1967_d_b7 * (nv15 - 0.0));
        let eq3_e1969_d_b8: f64 = (eq3_e1967_d_b8 * (nv15 - 0.0));
        let eq3_e1969_d_b9: f64 = (eq3_e1967_d_b9 * (nv15 - 0.0));
        let eq3_e1969_d_b10: f64 = (eq3_e1967_d_b10 * (nv15 - 0.0));
        let eq3_e1969_d_b11: f64 = (eq3_e1967_d_b11 * (nv15 - 0.0));
        let eq3_e1969_d_b12: f64 = (eq3_e1967_d_b12 * (nv15 - 0.0));
        let eq3_e1969_d_b13: f64 = (eq3_e1967_d_b13 * (nv15 - 0.0));
        let eq3_e1969_d_b14: f64 = (eq3_e1967_d_b14 * (nv15 - 0.0));
        let eq3_e1969_d_b15: f64 = (eq3_e1967_d_b15 * (nv15 - 0.0));
        let eq3_e1969_d_b16: f64 = (eq3_e1967_d_b16 * (nv15 - 0.0));
        let eq3_e1969_d_b17: f64 = (eq3_e1967_d_b17 * (nv15 - 0.0));
        (eq3_e1969, eq3_e1969_d_n0, eq3_e1969_d_n1, eq3_e1969_d_n2, eq3_e1969_d_n3, eq3_e1969_d_n4, eq3_e1969_d_n5, eq3_e1969_d_n6, eq3_e1969_d_n7, eq3_e1969_d_n8, eq3_e1969_d_n9, eq3_e1969_d_n10, eq3_e1969_d_n11, eq3_e1969_d_n12, eq3_e1969_d_n13, eq3_e1969_d_n14, eq3_e1969_d_n15, eq3_e1969_d_n16, eq3_e1969_d_b0, eq3_e1969_d_b1, eq3_e1969_d_b2, eq3_e1969_d_b3, eq3_e1969_d_b4, eq3_e1969_d_b5, eq3_e1969_d_b6, eq3_e1969_d_b7, eq3_e1969_d_b8, eq3_e1969_d_b9, eq3_e1969_d_b10, eq3_e1969_d_b11, eq3_e1969_d_b12, eq3_e1969_d_b13, eq3_e1969_d_b14, eq3_e1969_d_b15, eq3_e1969_d_b16, eq3_e1969_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e1971;
        let eq3_node_derivatives: [f64; 17] = [eq3_e1971_d_n0, eq3_e1971_d_n1, eq3_e1971_d_n2, eq3_e1971_d_n3, eq3_e1971_d_n4, eq3_e1971_d_n5, eq3_e1971_d_n6, eq3_e1971_d_n7, eq3_e1971_d_n8, eq3_e1971_d_n9, eq3_e1971_d_n10, eq3_e1971_d_n11, eq3_e1971_d_n12, eq3_e1971_d_n13, eq3_e1971_d_n14, eq3_e1971_d_n15, eq3_e1971_d_n16];
        let eq3_branch_derivatives: [f64; 18] = [eq3_e1971_d_b0, eq3_e1971_d_b1, eq3_e1971_d_b2, eq3_e1971_d_b3, eq3_e1971_d_b4, eq3_e1971_d_b5, eq3_e1971_d_b6, eq3_e1971_d_b7, eq3_e1971_d_b8, eq3_e1971_d_b9, eq3_e1971_d_b10, eq3_e1971_d_b11, eq3_e1971_d_b12, eq3_e1971_d_b13, eq3_e1971_d_b14, eq3_e1971_d_b15, eq3_e1971_d_b16, eq3_e1971_d_b17];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e1979, eq4_e1979_d_n0, eq4_e1979_d_n1, eq4_e1979_d_n2, eq4_e1979_d_n3, eq4_e1979_d_n4, eq4_e1979_d_n5, eq4_e1979_d_n6, eq4_e1979_d_n7, eq4_e1979_d_n8, eq4_e1979_d_n9, eq4_e1979_d_n10, eq4_e1979_d_n11, eq4_e1979_d_n12, eq4_e1979_d_n13, eq4_e1979_d_n14, eq4_e1979_d_n15, eq4_e1979_d_n16, eq4_e1979_d_b0, eq4_e1979_d_b1, eq4_e1979_d_b2, eq4_e1979_d_b3, eq4_e1979_d_b4, eq4_e1979_d_b5, eq4_e1979_d_b6, eq4_e1979_d_b7, eq4_e1979_d_b8, eq4_e1979_d_b9, eq4_e1979_d_b10, eq4_e1979_d_b11, eq4_e1979_d_b12, eq4_e1979_d_b13, eq4_e1979_d_b14, eq4_e1979_d_b15, eq4_e1979_d_b16, eq4_e1979_d_b17,) = {
    if (!s.b[1696]) {
        let eq4_e1976: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, s.v[137]);
        let eq4_e1976_d_n0: f64 = (s.dn[137][0] * ddt_scale);
        let eq4_e1976_d_n1: f64 = (s.dn[137][1] * ddt_scale);
        let eq4_e1976_d_n2: f64 = (s.dn[137][2] * ddt_scale);
        let eq4_e1976_d_n3: f64 = (s.dn[137][3] * ddt_scale);
        let eq4_e1976_d_n4: f64 = (s.dn[137][4] * ddt_scale);
        let eq4_e1976_d_n5: f64 = (s.dn[137][5] * ddt_scale);
        let eq4_e1976_d_n6: f64 = (s.dn[137][6] * ddt_scale);
        let eq4_e1976_d_n7: f64 = (s.dn[137][7] * ddt_scale);
        let eq4_e1976_d_n8: f64 = (s.dn[137][8] * ddt_scale);
        let eq4_e1976_d_n9: f64 = (s.dn[137][9] * ddt_scale);
        let eq4_e1976_d_n10: f64 = (s.dn[137][10] * ddt_scale);
        let eq4_e1976_d_n11: f64 = (s.dn[137][11] * ddt_scale);
        let eq4_e1976_d_n12: f64 = (s.dn[137][12] * ddt_scale);
        let eq4_e1976_d_n13: f64 = (s.dn[137][13] * ddt_scale);
        let eq4_e1976_d_n14: f64 = (s.dn[137][14] * ddt_scale);
        let eq4_e1976_d_n15: f64 = (s.dn[137][15] * ddt_scale);
        let eq4_e1976_d_n16: f64 = (s.dn[137][16] * ddt_scale);
        let eq4_e1976_d_b0: f64 = (s.db[137][0] * ddt_scale);
        let eq4_e1976_d_b1: f64 = (s.db[137][1] * ddt_scale);
        let eq4_e1976_d_b2: f64 = (s.db[137][2] * ddt_scale);
        let eq4_e1976_d_b3: f64 = (s.db[137][3] * ddt_scale);
        let eq4_e1976_d_b4: f64 = (s.db[137][4] * ddt_scale);
        let eq4_e1976_d_b5: f64 = (s.db[137][5] * ddt_scale);
        let eq4_e1976_d_b6: f64 = (s.db[137][6] * ddt_scale);
        let eq4_e1976_d_b7: f64 = (s.db[137][7] * ddt_scale);
        let eq4_e1976_d_b8: f64 = (s.db[137][8] * ddt_scale);
        let eq4_e1976_d_b9: f64 = (s.db[137][9] * ddt_scale);
        let eq4_e1976_d_b10: f64 = (s.db[137][10] * ddt_scale);
        let eq4_e1976_d_b11: f64 = (s.db[137][11] * ddt_scale);
        let eq4_e1976_d_b12: f64 = (s.db[137][12] * ddt_scale);
        let eq4_e1976_d_b13: f64 = (s.db[137][13] * ddt_scale);
        let eq4_e1976_d_b14: f64 = (s.db[137][14] * ddt_scale);
        let eq4_e1976_d_b15: f64 = (s.db[137][15] * ddt_scale);
        let eq4_e1976_d_b16: f64 = (s.db[137][16] * ddt_scale);
        let eq4_e1976_d_b17: f64 = (s.db[137][17] * ddt_scale);
        let eq4_e1977: f64 = (s.v[114] * eq4_e1976);
        let eq4_e1977_d_n0: f64 = ((s.dn[114][0] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n0));
        let eq4_e1977_d_n1: f64 = ((s.dn[114][1] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n1));
        let eq4_e1977_d_n2: f64 = ((s.dn[114][2] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n2));
        let eq4_e1977_d_n3: f64 = ((s.dn[114][3] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n3));
        let eq4_e1977_d_n4: f64 = ((s.dn[114][4] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n4));
        let eq4_e1977_d_n5: f64 = ((s.dn[114][5] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n5));
        let eq4_e1977_d_n6: f64 = ((s.dn[114][6] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n6));
        let eq4_e1977_d_n7: f64 = ((s.dn[114][7] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n7));
        let eq4_e1977_d_n8: f64 = ((s.dn[114][8] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n8));
        let eq4_e1977_d_n9: f64 = ((s.dn[114][9] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n9));
        let eq4_e1977_d_n10: f64 = ((s.dn[114][10] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n10));
        let eq4_e1977_d_n11: f64 = ((s.dn[114][11] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n11));
        let eq4_e1977_d_n12: f64 = ((s.dn[114][12] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n12));
        let eq4_e1977_d_n13: f64 = ((s.dn[114][13] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n13));
        let eq4_e1977_d_n14: f64 = ((s.dn[114][14] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n14));
        let eq4_e1977_d_n15: f64 = ((s.dn[114][15] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n15));
        let eq4_e1977_d_n16: f64 = ((s.dn[114][16] * eq4_e1976) + (s.v[114] * eq4_e1976_d_n16));
        let eq4_e1977_d_b0: f64 = ((s.db[114][0] * eq4_e1976) + (s.v[114] * eq4_e1976_d_b0));
        let eq4_e1977_d_b1: f64 = ((s.db[114][1] * eq4_e1976) + (s.v[114] * eq4_e1976_d_b1));
        let eq4_e1977_d_b2: f64 = ((s.db[114][2] * eq4_e1976) + (s.v[114] * eq4_e1976_d_b2));
        let eq4_e1977_d_b3: f64 = ((s.db[114][3] * eq4_e1976) + (s.v[114] * eq4_e1976_d_b3));
        let eq4_e1977_d_b4: f64 = ((s.db[114][4] * eq4_e1976) + (s.v[114] * eq4_e1976_d_b4));
        let eq4_e1977_d_b5: f64 = ((s.db[114][5] * eq4_e1976) + (s.v[114] * eq4_e1976_d_b5));
        let eq4_e1977_d_b6: f64 = ((s.db[114][6] * eq4_e1976) + (s.v[114] * eq4_e1976_d_b6));
        let eq4_e1977_d_b7: f64 = ((s.db[114][7] * eq4_e1976) + (s.v[114] * eq4_e1976_d_b7));
        let eq4_e1977_d_b8: f64 = ((s.db[114][8] * eq4_e1976) + (s.v[114] * eq4_e1976_d_b8));
        let eq4_e1977_d_b9: f64 = ((s.db[114][9] * eq4_e1976) + (s.v[114] * eq4_e1976_d_b9));
        let eq4_e1977_d_b10: f64 = ((s.db[114][10] * eq4_e1976) + (s.v[114] * eq4_e1976_d_b10));
        let eq4_e1977_d_b11: f64 = ((s.db[114][11] * eq4_e1976) + (s.v[114] * eq4_e1976_d_b11));
        let eq4_e1977_d_b12: f64 = ((s.db[114][12] * eq4_e1976) + (s.v[114] * eq4_e1976_d_b12));
        let eq4_e1977_d_b13: f64 = ((s.db[114][13] * eq4_e1976) + (s.v[114] * eq4_e1976_d_b13));
        let eq4_e1977_d_b14: f64 = ((s.db[114][14] * eq4_e1976) + (s.v[114] * eq4_e1976_d_b14));
        let eq4_e1977_d_b15: f64 = ((s.db[114][15] * eq4_e1976) + (s.v[114] * eq4_e1976_d_b15));
        let eq4_e1977_d_b16: f64 = ((s.db[114][16] * eq4_e1976) + (s.v[114] * eq4_e1976_d_b16));
        let eq4_e1977_d_b17: f64 = ((s.db[114][17] * eq4_e1976) + (s.v[114] * eq4_e1976_d_b17));
        (eq4_e1977, eq4_e1977_d_n0, eq4_e1977_d_n1, eq4_e1977_d_n2, eq4_e1977_d_n3, eq4_e1977_d_n4, eq4_e1977_d_n5, eq4_e1977_d_n6, eq4_e1977_d_n7, eq4_e1977_d_n8, eq4_e1977_d_n9, eq4_e1977_d_n10, eq4_e1977_d_n11, eq4_e1977_d_n12, eq4_e1977_d_n13, eq4_e1977_d_n14, eq4_e1977_d_n15, eq4_e1977_d_n16, eq4_e1977_d_b0, eq4_e1977_d_b1, eq4_e1977_d_b2, eq4_e1977_d_b3, eq4_e1977_d_b4, eq4_e1977_d_b5, eq4_e1977_d_b6, eq4_e1977_d_b7, eq4_e1977_d_b8, eq4_e1977_d_b9, eq4_e1977_d_b10, eq4_e1977_d_b11, eq4_e1977_d_b12, eq4_e1977_d_b13, eq4_e1977_d_b14, eq4_e1977_d_b15, eq4_e1977_d_b16, eq4_e1977_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e1979;
        let eq4_node_derivatives: [f64; 17] = [eq4_e1979_d_n0, eq4_e1979_d_n1, eq4_e1979_d_n2, eq4_e1979_d_n3, eq4_e1979_d_n4, eq4_e1979_d_n5, eq4_e1979_d_n6, eq4_e1979_d_n7, eq4_e1979_d_n8, eq4_e1979_d_n9, eq4_e1979_d_n10, eq4_e1979_d_n11, eq4_e1979_d_n12, eq4_e1979_d_n13, eq4_e1979_d_n14, eq4_e1979_d_n15, eq4_e1979_d_n16];
        let eq4_branch_derivatives: [f64; 18] = [eq4_e1979_d_b0, eq4_e1979_d_b1, eq4_e1979_d_b2, eq4_e1979_d_b3, eq4_e1979_d_b4, eq4_e1979_d_b5, eq4_e1979_d_b6, eq4_e1979_d_b7, eq4_e1979_d_b8, eq4_e1979_d_b9, eq4_e1979_d_b10, eq4_e1979_d_b11, eq4_e1979_d_b12, eq4_e1979_d_b13, eq4_e1979_d_b14, eq4_e1979_d_b15, eq4_e1979_d_b16, eq4_e1979_d_b17];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e1987, eq5_e1987_d_n0, eq5_e1987_d_n1, eq5_e1987_d_n2, eq5_e1987_d_n3, eq5_e1987_d_n4, eq5_e1987_d_n5, eq5_e1987_d_n6, eq5_e1987_d_n7, eq5_e1987_d_n8, eq5_e1987_d_n9, eq5_e1987_d_n10, eq5_e1987_d_n11, eq5_e1987_d_n12, eq5_e1987_d_n13, eq5_e1987_d_n14, eq5_e1987_d_n15, eq5_e1987_d_n16, eq5_e1987_d_b0, eq5_e1987_d_b1, eq5_e1987_d_b2, eq5_e1987_d_b3, eq5_e1987_d_b4, eq5_e1987_d_b5, eq5_e1987_d_b6, eq5_e1987_d_b7, eq5_e1987_d_b8, eq5_e1987_d_b9, eq5_e1987_d_b10, eq5_e1987_d_b11, eq5_e1987_d_b12, eq5_e1987_d_b13, eq5_e1987_d_b14, eq5_e1987_d_b15, eq5_e1987_d_b16, eq5_e1987_d_b17,) = {
    if (!s.b[1696]) {
        let eq5_e1984: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, s.v[138]);
        let eq5_e1984_d_n0: f64 = (s.dn[138][0] * ddt_scale);
        let eq5_e1984_d_n1: f64 = (s.dn[138][1] * ddt_scale);
        let eq5_e1984_d_n2: f64 = (s.dn[138][2] * ddt_scale);
        let eq5_e1984_d_n3: f64 = (s.dn[138][3] * ddt_scale);
        let eq5_e1984_d_n4: f64 = (s.dn[138][4] * ddt_scale);
        let eq5_e1984_d_n5: f64 = (s.dn[138][5] * ddt_scale);
        let eq5_e1984_d_n6: f64 = (s.dn[138][6] * ddt_scale);
        let eq5_e1984_d_n7: f64 = (s.dn[138][7] * ddt_scale);
        let eq5_e1984_d_n8: f64 = (s.dn[138][8] * ddt_scale);
        let eq5_e1984_d_n9: f64 = (s.dn[138][9] * ddt_scale);
        let eq5_e1984_d_n10: f64 = (s.dn[138][10] * ddt_scale);
        let eq5_e1984_d_n11: f64 = (s.dn[138][11] * ddt_scale);
        let eq5_e1984_d_n12: f64 = (s.dn[138][12] * ddt_scale);
        let eq5_e1984_d_n13: f64 = (s.dn[138][13] * ddt_scale);
        let eq5_e1984_d_n14: f64 = (s.dn[138][14] * ddt_scale);
        let eq5_e1984_d_n15: f64 = (s.dn[138][15] * ddt_scale);
        let eq5_e1984_d_n16: f64 = (s.dn[138][16] * ddt_scale);
        let eq5_e1984_d_b0: f64 = (s.db[138][0] * ddt_scale);
        let eq5_e1984_d_b1: f64 = (s.db[138][1] * ddt_scale);
        let eq5_e1984_d_b2: f64 = (s.db[138][2] * ddt_scale);
        let eq5_e1984_d_b3: f64 = (s.db[138][3] * ddt_scale);
        let eq5_e1984_d_b4: f64 = (s.db[138][4] * ddt_scale);
        let eq5_e1984_d_b5: f64 = (s.db[138][5] * ddt_scale);
        let eq5_e1984_d_b6: f64 = (s.db[138][6] * ddt_scale);
        let eq5_e1984_d_b7: f64 = (s.db[138][7] * ddt_scale);
        let eq5_e1984_d_b8: f64 = (s.db[138][8] * ddt_scale);
        let eq5_e1984_d_b9: f64 = (s.db[138][9] * ddt_scale);
        let eq5_e1984_d_b10: f64 = (s.db[138][10] * ddt_scale);
        let eq5_e1984_d_b11: f64 = (s.db[138][11] * ddt_scale);
        let eq5_e1984_d_b12: f64 = (s.db[138][12] * ddt_scale);
        let eq5_e1984_d_b13: f64 = (s.db[138][13] * ddt_scale);
        let eq5_e1984_d_b14: f64 = (s.db[138][14] * ddt_scale);
        let eq5_e1984_d_b15: f64 = (s.db[138][15] * ddt_scale);
        let eq5_e1984_d_b16: f64 = (s.db[138][16] * ddt_scale);
        let eq5_e1984_d_b17: f64 = (s.db[138][17] * ddt_scale);
        let eq5_e1985: f64 = (s.v[114] * eq5_e1984);
        let eq5_e1985_d_n0: f64 = ((s.dn[114][0] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n0));
        let eq5_e1985_d_n1: f64 = ((s.dn[114][1] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n1));
        let eq5_e1985_d_n2: f64 = ((s.dn[114][2] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n2));
        let eq5_e1985_d_n3: f64 = ((s.dn[114][3] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n3));
        let eq5_e1985_d_n4: f64 = ((s.dn[114][4] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n4));
        let eq5_e1985_d_n5: f64 = ((s.dn[114][5] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n5));
        let eq5_e1985_d_n6: f64 = ((s.dn[114][6] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n6));
        let eq5_e1985_d_n7: f64 = ((s.dn[114][7] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n7));
        let eq5_e1985_d_n8: f64 = ((s.dn[114][8] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n8));
        let eq5_e1985_d_n9: f64 = ((s.dn[114][9] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n9));
        let eq5_e1985_d_n10: f64 = ((s.dn[114][10] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n10));
        let eq5_e1985_d_n11: f64 = ((s.dn[114][11] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n11));
        let eq5_e1985_d_n12: f64 = ((s.dn[114][12] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n12));
        let eq5_e1985_d_n13: f64 = ((s.dn[114][13] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n13));
        let eq5_e1985_d_n14: f64 = ((s.dn[114][14] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n14));
        let eq5_e1985_d_n15: f64 = ((s.dn[114][15] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n15));
        let eq5_e1985_d_n16: f64 = ((s.dn[114][16] * eq5_e1984) + (s.v[114] * eq5_e1984_d_n16));
        let eq5_e1985_d_b0: f64 = ((s.db[114][0] * eq5_e1984) + (s.v[114] * eq5_e1984_d_b0));
        let eq5_e1985_d_b1: f64 = ((s.db[114][1] * eq5_e1984) + (s.v[114] * eq5_e1984_d_b1));
        let eq5_e1985_d_b2: f64 = ((s.db[114][2] * eq5_e1984) + (s.v[114] * eq5_e1984_d_b2));
        let eq5_e1985_d_b3: f64 = ((s.db[114][3] * eq5_e1984) + (s.v[114] * eq5_e1984_d_b3));
        let eq5_e1985_d_b4: f64 = ((s.db[114][4] * eq5_e1984) + (s.v[114] * eq5_e1984_d_b4));
        let eq5_e1985_d_b5: f64 = ((s.db[114][5] * eq5_e1984) + (s.v[114] * eq5_e1984_d_b5));
        let eq5_e1985_d_b6: f64 = ((s.db[114][6] * eq5_e1984) + (s.v[114] * eq5_e1984_d_b6));
        let eq5_e1985_d_b7: f64 = ((s.db[114][7] * eq5_e1984) + (s.v[114] * eq5_e1984_d_b7));
        let eq5_e1985_d_b8: f64 = ((s.db[114][8] * eq5_e1984) + (s.v[114] * eq5_e1984_d_b8));
        let eq5_e1985_d_b9: f64 = ((s.db[114][9] * eq5_e1984) + (s.v[114] * eq5_e1984_d_b9));
        let eq5_e1985_d_b10: f64 = ((s.db[114][10] * eq5_e1984) + (s.v[114] * eq5_e1984_d_b10));
        let eq5_e1985_d_b11: f64 = ((s.db[114][11] * eq5_e1984) + (s.v[114] * eq5_e1984_d_b11));
        let eq5_e1985_d_b12: f64 = ((s.db[114][12] * eq5_e1984) + (s.v[114] * eq5_e1984_d_b12));
        let eq5_e1985_d_b13: f64 = ((s.db[114][13] * eq5_e1984) + (s.v[114] * eq5_e1984_d_b13));
        let eq5_e1985_d_b14: f64 = ((s.db[114][14] * eq5_e1984) + (s.v[114] * eq5_e1984_d_b14));
        let eq5_e1985_d_b15: f64 = ((s.db[114][15] * eq5_e1984) + (s.v[114] * eq5_e1984_d_b15));
        let eq5_e1985_d_b16: f64 = ((s.db[114][16] * eq5_e1984) + (s.v[114] * eq5_e1984_d_b16));
        let eq5_e1985_d_b17: f64 = ((s.db[114][17] * eq5_e1984) + (s.v[114] * eq5_e1984_d_b17));
        (eq5_e1985, eq5_e1985_d_n0, eq5_e1985_d_n1, eq5_e1985_d_n2, eq5_e1985_d_n3, eq5_e1985_d_n4, eq5_e1985_d_n5, eq5_e1985_d_n6, eq5_e1985_d_n7, eq5_e1985_d_n8, eq5_e1985_d_n9, eq5_e1985_d_n10, eq5_e1985_d_n11, eq5_e1985_d_n12, eq5_e1985_d_n13, eq5_e1985_d_n14, eq5_e1985_d_n15, eq5_e1985_d_n16, eq5_e1985_d_b0, eq5_e1985_d_b1, eq5_e1985_d_b2, eq5_e1985_d_b3, eq5_e1985_d_b4, eq5_e1985_d_b5, eq5_e1985_d_b6, eq5_e1985_d_b7, eq5_e1985_d_b8, eq5_e1985_d_b9, eq5_e1985_d_b10, eq5_e1985_d_b11, eq5_e1985_d_b12, eq5_e1985_d_b13, eq5_e1985_d_b14, eq5_e1985_d_b15, eq5_e1985_d_b16, eq5_e1985_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1987;
        let eq5_node_derivatives: [f64; 17] = [eq5_e1987_d_n0, eq5_e1987_d_n1, eq5_e1987_d_n2, eq5_e1987_d_n3, eq5_e1987_d_n4, eq5_e1987_d_n5, eq5_e1987_d_n6, eq5_e1987_d_n7, eq5_e1987_d_n8, eq5_e1987_d_n9, eq5_e1987_d_n10, eq5_e1987_d_n11, eq5_e1987_d_n12, eq5_e1987_d_n13, eq5_e1987_d_n14, eq5_e1987_d_n15, eq5_e1987_d_n16];
        let eq5_branch_derivatives: [f64; 18] = [eq5_e1987_d_b0, eq5_e1987_d_b1, eq5_e1987_d_b2, eq5_e1987_d_b3, eq5_e1987_d_b4, eq5_e1987_d_b5, eq5_e1987_d_b6, eq5_e1987_d_b7, eq5_e1987_d_b8, eq5_e1987_d_b9, eq5_e1987_d_b10, eq5_e1987_d_b11, eq5_e1987_d_b12, eq5_e1987_d_b13, eq5_e1987_d_b14, eq5_e1987_d_b15, eq5_e1987_d_b16, eq5_e1987_d_b17];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq6_e1993, eq6_e1993_d_n5, eq6_e1993_d_n7,) = {
    if s.b[1697] {
        let eq6_e1991: f64 = ((nv7 - nv5) * 1000.0);
        let eq6_e1991_d_n5: f64 = (-1000.0);
        let eq6_e1991_d_n7: f64 = 1000.0;
        (eq6_e1991, eq6_e1991_d_n5, eq6_e1991_d_n7,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1993;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(5),
            multiplicity * (eq6_value),
            5,
            multiplicity * (eq6_e1993_d_n5),
            7,
            multiplicity * (eq6_e1993_d_n7),
        );
        let (eq7_e1998,) = {
    if (!s.b[1697]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq7_value: f64 = eq7_e1998;
        stamper.stamp_potential_const_local(
            0,
            eq7_value,
        );
        let (eq8_e2004, eq8_e2004_d_n0, eq8_e2004_d_n1, eq8_e2004_d_n2, eq8_e2004_d_n3, eq8_e2004_d_n4, eq8_e2004_d_n5, eq8_e2004_d_n6, eq8_e2004_d_n7, eq8_e2004_d_n8, eq8_e2004_d_n9, eq8_e2004_d_n10, eq8_e2004_d_n11, eq8_e2004_d_n12, eq8_e2004_d_n13, eq8_e2004_d_n14, eq8_e2004_d_n15, eq8_e2004_d_n16, eq8_e2004_d_b0, eq8_e2004_d_b1, eq8_e2004_d_b2, eq8_e2004_d_b3, eq8_e2004_d_b4, eq8_e2004_d_b5, eq8_e2004_d_b6, eq8_e2004_d_b7, eq8_e2004_d_b8, eq8_e2004_d_b9, eq8_e2004_d_b10, eq8_e2004_d_b11, eq8_e2004_d_b12, eq8_e2004_d_b13, eq8_e2004_d_b14, eq8_e2004_d_b15, eq8_e2004_d_b16, eq8_e2004_d_b17,) = {
    if s.b[1698] {
        let eq8_e2002: f64 = (s.v[114] * s.v[556]);
        let eq8_e2002_d_n0: f64 = ((s.dn[114][0] * s.v[556]) + (s.v[114] * s.dn[556][0]));
        let eq8_e2002_d_n1: f64 = ((s.dn[114][1] * s.v[556]) + (s.v[114] * s.dn[556][1]));
        let eq8_e2002_d_n2: f64 = ((s.dn[114][2] * s.v[556]) + (s.v[114] * s.dn[556][2]));
        let eq8_e2002_d_n3: f64 = ((s.dn[114][3] * s.v[556]) + (s.v[114] * s.dn[556][3]));
        let eq8_e2002_d_n4: f64 = ((s.dn[114][4] * s.v[556]) + (s.v[114] * s.dn[556][4]));
        let eq8_e2002_d_n5: f64 = ((s.dn[114][5] * s.v[556]) + (s.v[114] * s.dn[556][5]));
        let eq8_e2002_d_n6: f64 = ((s.dn[114][6] * s.v[556]) + (s.v[114] * s.dn[556][6]));
        let eq8_e2002_d_n7: f64 = ((s.dn[114][7] * s.v[556]) + (s.v[114] * s.dn[556][7]));
        let eq8_e2002_d_n8: f64 = ((s.dn[114][8] * s.v[556]) + (s.v[114] * s.dn[556][8]));
        let eq8_e2002_d_n9: f64 = ((s.dn[114][9] * s.v[556]) + (s.v[114] * s.dn[556][9]));
        let eq8_e2002_d_n10: f64 = ((s.dn[114][10] * s.v[556]) + (s.v[114] * s.dn[556][10]));
        let eq8_e2002_d_n11: f64 = ((s.dn[114][11] * s.v[556]) + (s.v[114] * s.dn[556][11]));
        let eq8_e2002_d_n12: f64 = ((s.dn[114][12] * s.v[556]) + (s.v[114] * s.dn[556][12]));
        let eq8_e2002_d_n13: f64 = ((s.dn[114][13] * s.v[556]) + (s.v[114] * s.dn[556][13]));
        let eq8_e2002_d_n14: f64 = ((s.dn[114][14] * s.v[556]) + (s.v[114] * s.dn[556][14]));
        let eq8_e2002_d_n15: f64 = ((s.dn[114][15] * s.v[556]) + (s.v[114] * s.dn[556][15]));
        let eq8_e2002_d_n16: f64 = ((s.dn[114][16] * s.v[556]) + (s.v[114] * s.dn[556][16]));
        let eq8_e2002_d_b0: f64 = ((s.db[114][0] * s.v[556]) + (s.v[114] * s.db[556][0]));
        let eq8_e2002_d_b1: f64 = ((s.db[114][1] * s.v[556]) + (s.v[114] * s.db[556][1]));
        let eq8_e2002_d_b2: f64 = ((s.db[114][2] * s.v[556]) + (s.v[114] * s.db[556][2]));
        let eq8_e2002_d_b3: f64 = ((s.db[114][3] * s.v[556]) + (s.v[114] * s.db[556][3]));
        let eq8_e2002_d_b4: f64 = ((s.db[114][4] * s.v[556]) + (s.v[114] * s.db[556][4]));
        let eq8_e2002_d_b5: f64 = ((s.db[114][5] * s.v[556]) + (s.v[114] * s.db[556][5]));
        let eq8_e2002_d_b6: f64 = ((s.db[114][6] * s.v[556]) + (s.v[114] * s.db[556][6]));
        let eq8_e2002_d_b7: f64 = ((s.db[114][7] * s.v[556]) + (s.v[114] * s.db[556][7]));
        let eq8_e2002_d_b8: f64 = ((s.db[114][8] * s.v[556]) + (s.v[114] * s.db[556][8]));
        let eq8_e2002_d_b9: f64 = ((s.db[114][9] * s.v[556]) + (s.v[114] * s.db[556][9]));
        let eq8_e2002_d_b10: f64 = ((s.db[114][10] * s.v[556]) + (s.v[114] * s.db[556][10]));
        let eq8_e2002_d_b11: f64 = ((s.db[114][11] * s.v[556]) + (s.v[114] * s.db[556][11]));
        let eq8_e2002_d_b12: f64 = ((s.db[114][12] * s.v[556]) + (s.v[114] * s.db[556][12]));
        let eq8_e2002_d_b13: f64 = ((s.db[114][13] * s.v[556]) + (s.v[114] * s.db[556][13]));
        let eq8_e2002_d_b14: f64 = ((s.db[114][14] * s.v[556]) + (s.v[114] * s.db[556][14]));
        let eq8_e2002_d_b15: f64 = ((s.db[114][15] * s.v[556]) + (s.v[114] * s.db[556][15]));
        let eq8_e2002_d_b16: f64 = ((s.db[114][16] * s.v[556]) + (s.v[114] * s.db[556][16]));
        let eq8_e2002_d_b17: f64 = ((s.db[114][17] * s.v[556]) + (s.v[114] * s.db[556][17]));
        (eq8_e2002, eq8_e2002_d_n0, eq8_e2002_d_n1, eq8_e2002_d_n2, eq8_e2002_d_n3, eq8_e2002_d_n4, eq8_e2002_d_n5, eq8_e2002_d_n6, eq8_e2002_d_n7, eq8_e2002_d_n8, eq8_e2002_d_n9, eq8_e2002_d_n10, eq8_e2002_d_n11, eq8_e2002_d_n12, eq8_e2002_d_n13, eq8_e2002_d_n14, eq8_e2002_d_n15, eq8_e2002_d_n16, eq8_e2002_d_b0, eq8_e2002_d_b1, eq8_e2002_d_b2, eq8_e2002_d_b3, eq8_e2002_d_b4, eq8_e2002_d_b5, eq8_e2002_d_b6, eq8_e2002_d_b7, eq8_e2002_d_b8, eq8_e2002_d_b9, eq8_e2002_d_b10, eq8_e2002_d_b11, eq8_e2002_d_b12, eq8_e2002_d_b13, eq8_e2002_d_b14, eq8_e2002_d_b15, eq8_e2002_d_b16, eq8_e2002_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e2004;
        let eq8_node_derivatives: [f64; 17] = [eq8_e2004_d_n0, eq8_e2004_d_n1, eq8_e2004_d_n2, eq8_e2004_d_n3, eq8_e2004_d_n4, eq8_e2004_d_n5, eq8_e2004_d_n6, eq8_e2004_d_n7, eq8_e2004_d_n8, eq8_e2004_d_n9, eq8_e2004_d_n10, eq8_e2004_d_n11, eq8_e2004_d_n12, eq8_e2004_d_n13, eq8_e2004_d_n14, eq8_e2004_d_n15, eq8_e2004_d_n16];
        let eq8_branch_derivatives: [f64; 18] = [eq8_e2004_d_b0, eq8_e2004_d_b1, eq8_e2004_d_b2, eq8_e2004_d_b3, eq8_e2004_d_b4, eq8_e2004_d_b5, eq8_e2004_d_b6, eq8_e2004_d_b7, eq8_e2004_d_b8, eq8_e2004_d_b9, eq8_e2004_d_b10, eq8_e2004_d_b11, eq8_e2004_d_b12, eq8_e2004_d_b13, eq8_e2004_d_b14, eq8_e2004_d_b15, eq8_e2004_d_b16, eq8_e2004_d_b17];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let (eq9_e2012, eq9_e2012_d_n0, eq9_e2012_d_n1, eq9_e2012_d_n2, eq9_e2012_d_n3, eq9_e2012_d_n4, eq9_e2012_d_n5, eq9_e2012_d_n6, eq9_e2012_d_n7, eq9_e2012_d_n8, eq9_e2012_d_n9, eq9_e2012_d_n10, eq9_e2012_d_n11, eq9_e2012_d_n12, eq9_e2012_d_n13, eq9_e2012_d_n14, eq9_e2012_d_n15, eq9_e2012_d_n16, eq9_e2012_d_b0, eq9_e2012_d_b1, eq9_e2012_d_b2, eq9_e2012_d_b3, eq9_e2012_d_b4, eq9_e2012_d_b5, eq9_e2012_d_b6, eq9_e2012_d_b7, eq9_e2012_d_b8, eq9_e2012_d_b9, eq9_e2012_d_b10, eq9_e2012_d_b11, eq9_e2012_d_b12, eq9_e2012_d_b13, eq9_e2012_d_b14, eq9_e2012_d_b15, eq9_e2012_d_b16, eq9_e2012_d_b17,) = {
    if s.b[1698] {
        let eq9_e2009: f64 = (s.v[470] + s.v[480]);
        let eq9_e2009_d_n0: f64 = (s.dn[470][0] + s.dn[480][0]);
        let eq9_e2009_d_n1: f64 = (s.dn[470][1] + s.dn[480][1]);
        let eq9_e2009_d_n2: f64 = (s.dn[470][2] + s.dn[480][2]);
        let eq9_e2009_d_n3: f64 = (s.dn[470][3] + s.dn[480][3]);
        let eq9_e2009_d_n4: f64 = (s.dn[470][4] + s.dn[480][4]);
        let eq9_e2009_d_n5: f64 = (s.dn[470][5] + s.dn[480][5]);
        let eq9_e2009_d_n6: f64 = (s.dn[470][6] + s.dn[480][6]);
        let eq9_e2009_d_n7: f64 = (s.dn[470][7] + s.dn[480][7]);
        let eq9_e2009_d_n8: f64 = (s.dn[470][8] + s.dn[480][8]);
        let eq9_e2009_d_n9: f64 = (s.dn[470][9] + s.dn[480][9]);
        let eq9_e2009_d_n10: f64 = (s.dn[470][10] + s.dn[480][10]);
        let eq9_e2009_d_n11: f64 = (s.dn[470][11] + s.dn[480][11]);
        let eq9_e2009_d_n12: f64 = (s.dn[470][12] + s.dn[480][12]);
        let eq9_e2009_d_n13: f64 = (s.dn[470][13] + s.dn[480][13]);
        let eq9_e2009_d_n14: f64 = (s.dn[470][14] + s.dn[480][14]);
        let eq9_e2009_d_n15: f64 = (s.dn[470][15] + s.dn[480][15]);
        let eq9_e2009_d_n16: f64 = (s.dn[470][16] + s.dn[480][16]);
        let eq9_e2009_d_b0: f64 = (s.db[470][0] + s.db[480][0]);
        let eq9_e2009_d_b1: f64 = (s.db[470][1] + s.db[480][1]);
        let eq9_e2009_d_b2: f64 = (s.db[470][2] + s.db[480][2]);
        let eq9_e2009_d_b3: f64 = (s.db[470][3] + s.db[480][3]);
        let eq9_e2009_d_b4: f64 = (s.db[470][4] + s.db[480][4]);
        let eq9_e2009_d_b5: f64 = (s.db[470][5] + s.db[480][5]);
        let eq9_e2009_d_b6: f64 = (s.db[470][6] + s.db[480][6]);
        let eq9_e2009_d_b7: f64 = (s.db[470][7] + s.db[480][7]);
        let eq9_e2009_d_b8: f64 = (s.db[470][8] + s.db[480][8]);
        let eq9_e2009_d_b9: f64 = (s.db[470][9] + s.db[480][9]);
        let eq9_e2009_d_b10: f64 = (s.db[470][10] + s.db[480][10]);
        let eq9_e2009_d_b11: f64 = (s.db[470][11] + s.db[480][11]);
        let eq9_e2009_d_b12: f64 = (s.db[470][12] + s.db[480][12]);
        let eq9_e2009_d_b13: f64 = (s.db[470][13] + s.db[480][13]);
        let eq9_e2009_d_b14: f64 = (s.db[470][14] + s.db[480][14]);
        let eq9_e2009_d_b15: f64 = (s.db[470][15] + s.db[480][15]);
        let eq9_e2009_d_b16: f64 = (s.db[470][16] + s.db[480][16]);
        let eq9_e2009_d_b17: f64 = (s.db[470][17] + s.db[480][17]);
        let eq9_e2010: f64 = (s.v[114] * eq9_e2009);
        let eq9_e2010_d_n0: f64 = ((s.dn[114][0] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n0));
        let eq9_e2010_d_n1: f64 = ((s.dn[114][1] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n1));
        let eq9_e2010_d_n2: f64 = ((s.dn[114][2] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n2));
        let eq9_e2010_d_n3: f64 = ((s.dn[114][3] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n3));
        let eq9_e2010_d_n4: f64 = ((s.dn[114][4] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n4));
        let eq9_e2010_d_n5: f64 = ((s.dn[114][5] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n5));
        let eq9_e2010_d_n6: f64 = ((s.dn[114][6] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n6));
        let eq9_e2010_d_n7: f64 = ((s.dn[114][7] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n7));
        let eq9_e2010_d_n8: f64 = ((s.dn[114][8] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n8));
        let eq9_e2010_d_n9: f64 = ((s.dn[114][9] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n9));
        let eq9_e2010_d_n10: f64 = ((s.dn[114][10] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n10));
        let eq9_e2010_d_n11: f64 = ((s.dn[114][11] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n11));
        let eq9_e2010_d_n12: f64 = ((s.dn[114][12] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n12));
        let eq9_e2010_d_n13: f64 = ((s.dn[114][13] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n13));
        let eq9_e2010_d_n14: f64 = ((s.dn[114][14] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n14));
        let eq9_e2010_d_n15: f64 = ((s.dn[114][15] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n15));
        let eq9_e2010_d_n16: f64 = ((s.dn[114][16] * eq9_e2009) + (s.v[114] * eq9_e2009_d_n16));
        let eq9_e2010_d_b0: f64 = ((s.db[114][0] * eq9_e2009) + (s.v[114] * eq9_e2009_d_b0));
        let eq9_e2010_d_b1: f64 = ((s.db[114][1] * eq9_e2009) + (s.v[114] * eq9_e2009_d_b1));
        let eq9_e2010_d_b2: f64 = ((s.db[114][2] * eq9_e2009) + (s.v[114] * eq9_e2009_d_b2));
        let eq9_e2010_d_b3: f64 = ((s.db[114][3] * eq9_e2009) + (s.v[114] * eq9_e2009_d_b3));
        let eq9_e2010_d_b4: f64 = ((s.db[114][4] * eq9_e2009) + (s.v[114] * eq9_e2009_d_b4));
        let eq9_e2010_d_b5: f64 = ((s.db[114][5] * eq9_e2009) + (s.v[114] * eq9_e2009_d_b5));
        let eq9_e2010_d_b6: f64 = ((s.db[114][6] * eq9_e2009) + (s.v[114] * eq9_e2009_d_b6));
        let eq9_e2010_d_b7: f64 = ((s.db[114][7] * eq9_e2009) + (s.v[114] * eq9_e2009_d_b7));
        let eq9_e2010_d_b8: f64 = ((s.db[114][8] * eq9_e2009) + (s.v[114] * eq9_e2009_d_b8));
        let eq9_e2010_d_b9: f64 = ((s.db[114][9] * eq9_e2009) + (s.v[114] * eq9_e2009_d_b9));
        let eq9_e2010_d_b10: f64 = ((s.db[114][10] * eq9_e2009) + (s.v[114] * eq9_e2009_d_b10));
        let eq9_e2010_d_b11: f64 = ((s.db[114][11] * eq9_e2009) + (s.v[114] * eq9_e2009_d_b11));
        let eq9_e2010_d_b12: f64 = ((s.db[114][12] * eq9_e2009) + (s.v[114] * eq9_e2009_d_b12));
        let eq9_e2010_d_b13: f64 = ((s.db[114][13] * eq9_e2009) + (s.v[114] * eq9_e2009_d_b13));
        let eq9_e2010_d_b14: f64 = ((s.db[114][14] * eq9_e2009) + (s.v[114] * eq9_e2009_d_b14));
        let eq9_e2010_d_b15: f64 = ((s.db[114][15] * eq9_e2009) + (s.v[114] * eq9_e2009_d_b15));
        let eq9_e2010_d_b16: f64 = ((s.db[114][16] * eq9_e2009) + (s.v[114] * eq9_e2009_d_b16));
        let eq9_e2010_d_b17: f64 = ((s.db[114][17] * eq9_e2009) + (s.v[114] * eq9_e2009_d_b17));
        (eq9_e2010, eq9_e2010_d_n0, eq9_e2010_d_n1, eq9_e2010_d_n2, eq9_e2010_d_n3, eq9_e2010_d_n4, eq9_e2010_d_n5, eq9_e2010_d_n6, eq9_e2010_d_n7, eq9_e2010_d_n8, eq9_e2010_d_n9, eq9_e2010_d_n10, eq9_e2010_d_n11, eq9_e2010_d_n12, eq9_e2010_d_n13, eq9_e2010_d_n14, eq9_e2010_d_n15, eq9_e2010_d_n16, eq9_e2010_d_b0, eq9_e2010_d_b1, eq9_e2010_d_b2, eq9_e2010_d_b3, eq9_e2010_d_b4, eq9_e2010_d_b5, eq9_e2010_d_b6, eq9_e2010_d_b7, eq9_e2010_d_b8, eq9_e2010_d_b9, eq9_e2010_d_b10, eq9_e2010_d_b11, eq9_e2010_d_b12, eq9_e2010_d_b13, eq9_e2010_d_b14, eq9_e2010_d_b15, eq9_e2010_d_b16, eq9_e2010_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e2012;
        let eq9_node_derivatives: [f64; 17] = [eq9_e2012_d_n0, eq9_e2012_d_n1, eq9_e2012_d_n2, eq9_e2012_d_n3, eq9_e2012_d_n4, eq9_e2012_d_n5, eq9_e2012_d_n6, eq9_e2012_d_n7, eq9_e2012_d_n8, eq9_e2012_d_n9, eq9_e2012_d_n10, eq9_e2012_d_n11, eq9_e2012_d_n12, eq9_e2012_d_n13, eq9_e2012_d_n14, eq9_e2012_d_n15, eq9_e2012_d_n16];
        let eq9_branch_derivatives: [f64; 18] = [eq9_e2012_d_b0, eq9_e2012_d_b1, eq9_e2012_d_b2, eq9_e2012_d_b3, eq9_e2012_d_b4, eq9_e2012_d_b5, eq9_e2012_d_b6, eq9_e2012_d_b7, eq9_e2012_d_b8, eq9_e2012_d_b9, eq9_e2012_d_b10, eq9_e2012_d_b11, eq9_e2012_d_b12, eq9_e2012_d_b13, eq9_e2012_d_b14, eq9_e2012_d_b15, eq9_e2012_d_b16, eq9_e2012_d_b17];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let (eq10_e2020, eq10_e2020_d_n0, eq10_e2020_d_n1, eq10_e2020_d_n2, eq10_e2020_d_n3, eq10_e2020_d_n4, eq10_e2020_d_n5, eq10_e2020_d_n6, eq10_e2020_d_n7, eq10_e2020_d_n8, eq10_e2020_d_n9, eq10_e2020_d_n10, eq10_e2020_d_n11, eq10_e2020_d_n12, eq10_e2020_d_n13, eq10_e2020_d_n14, eq10_e2020_d_n15, eq10_e2020_d_n16, eq10_e2020_d_b0, eq10_e2020_d_b1, eq10_e2020_d_b2, eq10_e2020_d_b3, eq10_e2020_d_b4, eq10_e2020_d_b5, eq10_e2020_d_b6, eq10_e2020_d_b7, eq10_e2020_d_b8, eq10_e2020_d_b9, eq10_e2020_d_b10, eq10_e2020_d_b11, eq10_e2020_d_b12, eq10_e2020_d_b13, eq10_e2020_d_b14, eq10_e2020_d_b15, eq10_e2020_d_b16, eq10_e2020_d_b17,) = {
    if s.b[1698] {
        let eq10_e2017: f64 = (s.v[471] + s.v[481]);
        let eq10_e2017_d_n0: f64 = (s.dn[471][0] + s.dn[481][0]);
        let eq10_e2017_d_n1: f64 = (s.dn[471][1] + s.dn[481][1]);
        let eq10_e2017_d_n2: f64 = (s.dn[471][2] + s.dn[481][2]);
        let eq10_e2017_d_n3: f64 = (s.dn[471][3] + s.dn[481][3]);
        let eq10_e2017_d_n4: f64 = (s.dn[471][4] + s.dn[481][4]);
        let eq10_e2017_d_n5: f64 = (s.dn[471][5] + s.dn[481][5]);
        let eq10_e2017_d_n6: f64 = (s.dn[471][6] + s.dn[481][6]);
        let eq10_e2017_d_n7: f64 = (s.dn[471][7] + s.dn[481][7]);
        let eq10_e2017_d_n8: f64 = (s.dn[471][8] + s.dn[481][8]);
        let eq10_e2017_d_n9: f64 = (s.dn[471][9] + s.dn[481][9]);
        let eq10_e2017_d_n10: f64 = (s.dn[471][10] + s.dn[481][10]);
        let eq10_e2017_d_n11: f64 = (s.dn[471][11] + s.dn[481][11]);
        let eq10_e2017_d_n12: f64 = (s.dn[471][12] + s.dn[481][12]);
        let eq10_e2017_d_n13: f64 = (s.dn[471][13] + s.dn[481][13]);
        let eq10_e2017_d_n14: f64 = (s.dn[471][14] + s.dn[481][14]);
        let eq10_e2017_d_n15: f64 = (s.dn[471][15] + s.dn[481][15]);
        let eq10_e2017_d_n16: f64 = (s.dn[471][16] + s.dn[481][16]);
        let eq10_e2017_d_b0: f64 = (s.db[471][0] + s.db[481][0]);
        let eq10_e2017_d_b1: f64 = (s.db[471][1] + s.db[481][1]);
        let eq10_e2017_d_b2: f64 = (s.db[471][2] + s.db[481][2]);
        let eq10_e2017_d_b3: f64 = (s.db[471][3] + s.db[481][3]);
        let eq10_e2017_d_b4: f64 = (s.db[471][4] + s.db[481][4]);
        let eq10_e2017_d_b5: f64 = (s.db[471][5] + s.db[481][5]);
        let eq10_e2017_d_b6: f64 = (s.db[471][6] + s.db[481][6]);
        let eq10_e2017_d_b7: f64 = (s.db[471][7] + s.db[481][7]);
        let eq10_e2017_d_b8: f64 = (s.db[471][8] + s.db[481][8]);
        let eq10_e2017_d_b9: f64 = (s.db[471][9] + s.db[481][9]);
        let eq10_e2017_d_b10: f64 = (s.db[471][10] + s.db[481][10]);
        let eq10_e2017_d_b11: f64 = (s.db[471][11] + s.db[481][11]);
        let eq10_e2017_d_b12: f64 = (s.db[471][12] + s.db[481][12]);
        let eq10_e2017_d_b13: f64 = (s.db[471][13] + s.db[481][13]);
        let eq10_e2017_d_b14: f64 = (s.db[471][14] + s.db[481][14]);
        let eq10_e2017_d_b15: f64 = (s.db[471][15] + s.db[481][15]);
        let eq10_e2017_d_b16: f64 = (s.db[471][16] + s.db[481][16]);
        let eq10_e2017_d_b17: f64 = (s.db[471][17] + s.db[481][17]);
        let eq10_e2018: f64 = (s.v[114] * eq10_e2017);
        let eq10_e2018_d_n0: f64 = ((s.dn[114][0] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n0));
        let eq10_e2018_d_n1: f64 = ((s.dn[114][1] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n1));
        let eq10_e2018_d_n2: f64 = ((s.dn[114][2] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n2));
        let eq10_e2018_d_n3: f64 = ((s.dn[114][3] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n3));
        let eq10_e2018_d_n4: f64 = ((s.dn[114][4] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n4));
        let eq10_e2018_d_n5: f64 = ((s.dn[114][5] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n5));
        let eq10_e2018_d_n6: f64 = ((s.dn[114][6] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n6));
        let eq10_e2018_d_n7: f64 = ((s.dn[114][7] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n7));
        let eq10_e2018_d_n8: f64 = ((s.dn[114][8] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n8));
        let eq10_e2018_d_n9: f64 = ((s.dn[114][9] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n9));
        let eq10_e2018_d_n10: f64 = ((s.dn[114][10] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n10));
        let eq10_e2018_d_n11: f64 = ((s.dn[114][11] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n11));
        let eq10_e2018_d_n12: f64 = ((s.dn[114][12] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n12));
        let eq10_e2018_d_n13: f64 = ((s.dn[114][13] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n13));
        let eq10_e2018_d_n14: f64 = ((s.dn[114][14] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n14));
        let eq10_e2018_d_n15: f64 = ((s.dn[114][15] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n15));
        let eq10_e2018_d_n16: f64 = ((s.dn[114][16] * eq10_e2017) + (s.v[114] * eq10_e2017_d_n16));
        let eq10_e2018_d_b0: f64 = ((s.db[114][0] * eq10_e2017) + (s.v[114] * eq10_e2017_d_b0));
        let eq10_e2018_d_b1: f64 = ((s.db[114][1] * eq10_e2017) + (s.v[114] * eq10_e2017_d_b1));
        let eq10_e2018_d_b2: f64 = ((s.db[114][2] * eq10_e2017) + (s.v[114] * eq10_e2017_d_b2));
        let eq10_e2018_d_b3: f64 = ((s.db[114][3] * eq10_e2017) + (s.v[114] * eq10_e2017_d_b3));
        let eq10_e2018_d_b4: f64 = ((s.db[114][4] * eq10_e2017) + (s.v[114] * eq10_e2017_d_b4));
        let eq10_e2018_d_b5: f64 = ((s.db[114][5] * eq10_e2017) + (s.v[114] * eq10_e2017_d_b5));
        let eq10_e2018_d_b6: f64 = ((s.db[114][6] * eq10_e2017) + (s.v[114] * eq10_e2017_d_b6));
        let eq10_e2018_d_b7: f64 = ((s.db[114][7] * eq10_e2017) + (s.v[114] * eq10_e2017_d_b7));
        let eq10_e2018_d_b8: f64 = ((s.db[114][8] * eq10_e2017) + (s.v[114] * eq10_e2017_d_b8));
        let eq10_e2018_d_b9: f64 = ((s.db[114][9] * eq10_e2017) + (s.v[114] * eq10_e2017_d_b9));
        let eq10_e2018_d_b10: f64 = ((s.db[114][10] * eq10_e2017) + (s.v[114] * eq10_e2017_d_b10));
        let eq10_e2018_d_b11: f64 = ((s.db[114][11] * eq10_e2017) + (s.v[114] * eq10_e2017_d_b11));
        let eq10_e2018_d_b12: f64 = ((s.db[114][12] * eq10_e2017) + (s.v[114] * eq10_e2017_d_b12));
        let eq10_e2018_d_b13: f64 = ((s.db[114][13] * eq10_e2017) + (s.v[114] * eq10_e2017_d_b13));
        let eq10_e2018_d_b14: f64 = ((s.db[114][14] * eq10_e2017) + (s.v[114] * eq10_e2017_d_b14));
        let eq10_e2018_d_b15: f64 = ((s.db[114][15] * eq10_e2017) + (s.v[114] * eq10_e2017_d_b15));
        let eq10_e2018_d_b16: f64 = ((s.db[114][16] * eq10_e2017) + (s.v[114] * eq10_e2017_d_b16));
        let eq10_e2018_d_b17: f64 = ((s.db[114][17] * eq10_e2017) + (s.v[114] * eq10_e2017_d_b17));
        (eq10_e2018, eq10_e2018_d_n0, eq10_e2018_d_n1, eq10_e2018_d_n2, eq10_e2018_d_n3, eq10_e2018_d_n4, eq10_e2018_d_n5, eq10_e2018_d_n6, eq10_e2018_d_n7, eq10_e2018_d_n8, eq10_e2018_d_n9, eq10_e2018_d_n10, eq10_e2018_d_n11, eq10_e2018_d_n12, eq10_e2018_d_n13, eq10_e2018_d_n14, eq10_e2018_d_n15, eq10_e2018_d_n16, eq10_e2018_d_b0, eq10_e2018_d_b1, eq10_e2018_d_b2, eq10_e2018_d_b3, eq10_e2018_d_b4, eq10_e2018_d_b5, eq10_e2018_d_b6, eq10_e2018_d_b7, eq10_e2018_d_b8, eq10_e2018_d_b9, eq10_e2018_d_b10, eq10_e2018_d_b11, eq10_e2018_d_b12, eq10_e2018_d_b13, eq10_e2018_d_b14, eq10_e2018_d_b15, eq10_e2018_d_b16, eq10_e2018_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e2020;
        let eq10_node_derivatives: [f64; 17] = [eq10_e2020_d_n0, eq10_e2020_d_n1, eq10_e2020_d_n2, eq10_e2020_d_n3, eq10_e2020_d_n4, eq10_e2020_d_n5, eq10_e2020_d_n6, eq10_e2020_d_n7, eq10_e2020_d_n8, eq10_e2020_d_n9, eq10_e2020_d_n10, eq10_e2020_d_n11, eq10_e2020_d_n12, eq10_e2020_d_n13, eq10_e2020_d_n14, eq10_e2020_d_n15, eq10_e2020_d_n16];
        let eq10_branch_derivatives: [f64; 18] = [eq10_e2020_d_b0, eq10_e2020_d_b1, eq10_e2020_d_b2, eq10_e2020_d_b3, eq10_e2020_d_b4, eq10_e2020_d_b5, eq10_e2020_d_b6, eq10_e2020_d_b7, eq10_e2020_d_b8, eq10_e2020_d_b9, eq10_e2020_d_b10, eq10_e2020_d_b11, eq10_e2020_d_b12, eq10_e2020_d_b13, eq10_e2020_d_b14, eq10_e2020_d_b15, eq10_e2020_d_b16, eq10_e2020_d_b17];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(5),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let (eq11_e2032, eq11_e2032_d_n0, eq11_e2032_d_n1, eq11_e2032_d_n2, eq11_e2032_d_n3, eq11_e2032_d_n4, eq11_e2032_d_n5, eq11_e2032_d_n6, eq11_e2032_d_n7, eq11_e2032_d_n8, eq11_e2032_d_n9, eq11_e2032_d_n10, eq11_e2032_d_n11, eq11_e2032_d_n12, eq11_e2032_d_n13, eq11_e2032_d_n14, eq11_e2032_d_n15, eq11_e2032_d_n16, eq11_e2032_d_b0, eq11_e2032_d_b1, eq11_e2032_d_b2, eq11_e2032_d_b3, eq11_e2032_d_b4, eq11_e2032_d_b5, eq11_e2032_d_b6, eq11_e2032_d_b7, eq11_e2032_d_b8, eq11_e2032_d_b9, eq11_e2032_d_b10, eq11_e2032_d_b11, eq11_e2032_d_b12, eq11_e2032_d_b13, eq11_e2032_d_b14, eq11_e2032_d_b15, eq11_e2032_d_b16, eq11_e2032_d_b17,) = {
    if ((s.b[1698] && s.b[1699]) && s.b[1700]) {
        let eq11_e2029: f64 = (s.v[476] + s.v[488]);
        let eq11_e2029_d_n0: f64 = (s.dn[476][0] + s.dn[488][0]);
        let eq11_e2029_d_n1: f64 = (s.dn[476][1] + s.dn[488][1]);
        let eq11_e2029_d_n2: f64 = (s.dn[476][2] + s.dn[488][2]);
        let eq11_e2029_d_n3: f64 = (s.dn[476][3] + s.dn[488][3]);
        let eq11_e2029_d_n4: f64 = (s.dn[476][4] + s.dn[488][4]);
        let eq11_e2029_d_n5: f64 = (s.dn[476][5] + s.dn[488][5]);
        let eq11_e2029_d_n6: f64 = (s.dn[476][6] + s.dn[488][6]);
        let eq11_e2029_d_n7: f64 = (s.dn[476][7] + s.dn[488][7]);
        let eq11_e2029_d_n8: f64 = (s.dn[476][8] + s.dn[488][8]);
        let eq11_e2029_d_n9: f64 = (s.dn[476][9] + s.dn[488][9]);
        let eq11_e2029_d_n10: f64 = (s.dn[476][10] + s.dn[488][10]);
        let eq11_e2029_d_n11: f64 = (s.dn[476][11] + s.dn[488][11]);
        let eq11_e2029_d_n12: f64 = (s.dn[476][12] + s.dn[488][12]);
        let eq11_e2029_d_n13: f64 = (s.dn[476][13] + s.dn[488][13]);
        let eq11_e2029_d_n14: f64 = (s.dn[476][14] + s.dn[488][14]);
        let eq11_e2029_d_n15: f64 = (s.dn[476][15] + s.dn[488][15]);
        let eq11_e2029_d_n16: f64 = (s.dn[476][16] + s.dn[488][16]);
        let eq11_e2029_d_b0: f64 = (s.db[476][0] + s.db[488][0]);
        let eq11_e2029_d_b1: f64 = (s.db[476][1] + s.db[488][1]);
        let eq11_e2029_d_b2: f64 = (s.db[476][2] + s.db[488][2]);
        let eq11_e2029_d_b3: f64 = (s.db[476][3] + s.db[488][3]);
        let eq11_e2029_d_b4: f64 = (s.db[476][4] + s.db[488][4]);
        let eq11_e2029_d_b5: f64 = (s.db[476][5] + s.db[488][5]);
        let eq11_e2029_d_b6: f64 = (s.db[476][6] + s.db[488][6]);
        let eq11_e2029_d_b7: f64 = (s.db[476][7] + s.db[488][7]);
        let eq11_e2029_d_b8: f64 = (s.db[476][8] + s.db[488][8]);
        let eq11_e2029_d_b9: f64 = (s.db[476][9] + s.db[488][9]);
        let eq11_e2029_d_b10: f64 = (s.db[476][10] + s.db[488][10]);
        let eq11_e2029_d_b11: f64 = (s.db[476][11] + s.db[488][11]);
        let eq11_e2029_d_b12: f64 = (s.db[476][12] + s.db[488][12]);
        let eq11_e2029_d_b13: f64 = (s.db[476][13] + s.db[488][13]);
        let eq11_e2029_d_b14: f64 = (s.db[476][14] + s.db[488][14]);
        let eq11_e2029_d_b15: f64 = (s.db[476][15] + s.db[488][15]);
        let eq11_e2029_d_b16: f64 = (s.db[476][16] + s.db[488][16]);
        let eq11_e2029_d_b17: f64 = (s.db[476][17] + s.db[488][17]);
        let eq11_e2030: f64 = (s.v[114] * eq11_e2029);
        let eq11_e2030_d_n0: f64 = ((s.dn[114][0] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n0));
        let eq11_e2030_d_n1: f64 = ((s.dn[114][1] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n1));
        let eq11_e2030_d_n2: f64 = ((s.dn[114][2] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n2));
        let eq11_e2030_d_n3: f64 = ((s.dn[114][3] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n3));
        let eq11_e2030_d_n4: f64 = ((s.dn[114][4] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n4));
        let eq11_e2030_d_n5: f64 = ((s.dn[114][5] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n5));
        let eq11_e2030_d_n6: f64 = ((s.dn[114][6] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n6));
        let eq11_e2030_d_n7: f64 = ((s.dn[114][7] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n7));
        let eq11_e2030_d_n8: f64 = ((s.dn[114][8] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n8));
        let eq11_e2030_d_n9: f64 = ((s.dn[114][9] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n9));
        let eq11_e2030_d_n10: f64 = ((s.dn[114][10] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n10));
        let eq11_e2030_d_n11: f64 = ((s.dn[114][11] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n11));
        let eq11_e2030_d_n12: f64 = ((s.dn[114][12] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n12));
        let eq11_e2030_d_n13: f64 = ((s.dn[114][13] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n13));
        let eq11_e2030_d_n14: f64 = ((s.dn[114][14] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n14));
        let eq11_e2030_d_n15: f64 = ((s.dn[114][15] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n15));
        let eq11_e2030_d_n16: f64 = ((s.dn[114][16] * eq11_e2029) + (s.v[114] * eq11_e2029_d_n16));
        let eq11_e2030_d_b0: f64 = ((s.db[114][0] * eq11_e2029) + (s.v[114] * eq11_e2029_d_b0));
        let eq11_e2030_d_b1: f64 = ((s.db[114][1] * eq11_e2029) + (s.v[114] * eq11_e2029_d_b1));
        let eq11_e2030_d_b2: f64 = ((s.db[114][2] * eq11_e2029) + (s.v[114] * eq11_e2029_d_b2));
        let eq11_e2030_d_b3: f64 = ((s.db[114][3] * eq11_e2029) + (s.v[114] * eq11_e2029_d_b3));
        let eq11_e2030_d_b4: f64 = ((s.db[114][4] * eq11_e2029) + (s.v[114] * eq11_e2029_d_b4));
        let eq11_e2030_d_b5: f64 = ((s.db[114][5] * eq11_e2029) + (s.v[114] * eq11_e2029_d_b5));
        let eq11_e2030_d_b6: f64 = ((s.db[114][6] * eq11_e2029) + (s.v[114] * eq11_e2029_d_b6));
        let eq11_e2030_d_b7: f64 = ((s.db[114][7] * eq11_e2029) + (s.v[114] * eq11_e2029_d_b7));
        let eq11_e2030_d_b8: f64 = ((s.db[114][8] * eq11_e2029) + (s.v[114] * eq11_e2029_d_b8));
        let eq11_e2030_d_b9: f64 = ((s.db[114][9] * eq11_e2029) + (s.v[114] * eq11_e2029_d_b9));
        let eq11_e2030_d_b10: f64 = ((s.db[114][10] * eq11_e2029) + (s.v[114] * eq11_e2029_d_b10));
        let eq11_e2030_d_b11: f64 = ((s.db[114][11] * eq11_e2029) + (s.v[114] * eq11_e2029_d_b11));
        let eq11_e2030_d_b12: f64 = ((s.db[114][12] * eq11_e2029) + (s.v[114] * eq11_e2029_d_b12));
        let eq11_e2030_d_b13: f64 = ((s.db[114][13] * eq11_e2029) + (s.v[114] * eq11_e2029_d_b13));
        let eq11_e2030_d_b14: f64 = ((s.db[114][14] * eq11_e2029) + (s.v[114] * eq11_e2029_d_b14));
        let eq11_e2030_d_b15: f64 = ((s.db[114][15] * eq11_e2029) + (s.v[114] * eq11_e2029_d_b15));
        let eq11_e2030_d_b16: f64 = ((s.db[114][16] * eq11_e2029) + (s.v[114] * eq11_e2029_d_b16));
        let eq11_e2030_d_b17: f64 = ((s.db[114][17] * eq11_e2029) + (s.v[114] * eq11_e2029_d_b17));
        (eq11_e2030, eq11_e2030_d_n0, eq11_e2030_d_n1, eq11_e2030_d_n2, eq11_e2030_d_n3, eq11_e2030_d_n4, eq11_e2030_d_n5, eq11_e2030_d_n6, eq11_e2030_d_n7, eq11_e2030_d_n8, eq11_e2030_d_n9, eq11_e2030_d_n10, eq11_e2030_d_n11, eq11_e2030_d_n12, eq11_e2030_d_n13, eq11_e2030_d_n14, eq11_e2030_d_n15, eq11_e2030_d_n16, eq11_e2030_d_b0, eq11_e2030_d_b1, eq11_e2030_d_b2, eq11_e2030_d_b3, eq11_e2030_d_b4, eq11_e2030_d_b5, eq11_e2030_d_b6, eq11_e2030_d_b7, eq11_e2030_d_b8, eq11_e2030_d_b9, eq11_e2030_d_b10, eq11_e2030_d_b11, eq11_e2030_d_b12, eq11_e2030_d_b13, eq11_e2030_d_b14, eq11_e2030_d_b15, eq11_e2030_d_b16, eq11_e2030_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e2032;
        let eq11_node_derivatives: [f64; 17] = [eq11_e2032_d_n0, eq11_e2032_d_n1, eq11_e2032_d_n2, eq11_e2032_d_n3, eq11_e2032_d_n4, eq11_e2032_d_n5, eq11_e2032_d_n6, eq11_e2032_d_n7, eq11_e2032_d_n8, eq11_e2032_d_n9, eq11_e2032_d_n10, eq11_e2032_d_n11, eq11_e2032_d_n12, eq11_e2032_d_n13, eq11_e2032_d_n14, eq11_e2032_d_n15, eq11_e2032_d_n16];
        let eq11_branch_derivatives: [f64; 18] = [eq11_e2032_d_b0, eq11_e2032_d_b1, eq11_e2032_d_b2, eq11_e2032_d_b3, eq11_e2032_d_b4, eq11_e2032_d_b5, eq11_e2032_d_b6, eq11_e2032_d_b7, eq11_e2032_d_b8, eq11_e2032_d_b9, eq11_e2032_d_b10, eq11_e2032_d_b11, eq11_e2032_d_b12, eq11_e2032_d_b13, eq11_e2032_d_b14, eq11_e2032_d_b15, eq11_e2032_d_b16, eq11_e2032_d_b17];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let (eq12_e2042, eq12_e2042_d_n0, eq12_e2042_d_n1, eq12_e2042_d_n2, eq12_e2042_d_n3, eq12_e2042_d_n4, eq12_e2042_d_n5, eq12_e2042_d_n6, eq12_e2042_d_n7, eq12_e2042_d_n8, eq12_e2042_d_n9, eq12_e2042_d_n10, eq12_e2042_d_n11, eq12_e2042_d_n12, eq12_e2042_d_n13, eq12_e2042_d_n14, eq12_e2042_d_n15, eq12_e2042_d_n16, eq12_e2042_d_b0, eq12_e2042_d_b1, eq12_e2042_d_b2, eq12_e2042_d_b3, eq12_e2042_d_b4, eq12_e2042_d_b5, eq12_e2042_d_b6, eq12_e2042_d_b7, eq12_e2042_d_b8, eq12_e2042_d_b9, eq12_e2042_d_b10, eq12_e2042_d_b11, eq12_e2042_d_b12, eq12_e2042_d_b13, eq12_e2042_d_b14, eq12_e2042_d_b15, eq12_e2042_d_b16, eq12_e2042_d_b17,) = {
    if ((s.b[1698] && s.b[1699]) && s.b[1700]) {
        let eq12_e2040: f64 = (s.v[114] * s.v[475]);
        let eq12_e2040_d_n0: f64 = ((s.dn[114][0] * s.v[475]) + (s.v[114] * s.dn[475][0]));
        let eq12_e2040_d_n1: f64 = ((s.dn[114][1] * s.v[475]) + (s.v[114] * s.dn[475][1]));
        let eq12_e2040_d_n2: f64 = ((s.dn[114][2] * s.v[475]) + (s.v[114] * s.dn[475][2]));
        let eq12_e2040_d_n3: f64 = ((s.dn[114][3] * s.v[475]) + (s.v[114] * s.dn[475][3]));
        let eq12_e2040_d_n4: f64 = ((s.dn[114][4] * s.v[475]) + (s.v[114] * s.dn[475][4]));
        let eq12_e2040_d_n5: f64 = ((s.dn[114][5] * s.v[475]) + (s.v[114] * s.dn[475][5]));
        let eq12_e2040_d_n6: f64 = ((s.dn[114][6] * s.v[475]) + (s.v[114] * s.dn[475][6]));
        let eq12_e2040_d_n7: f64 = ((s.dn[114][7] * s.v[475]) + (s.v[114] * s.dn[475][7]));
        let eq12_e2040_d_n8: f64 = ((s.dn[114][8] * s.v[475]) + (s.v[114] * s.dn[475][8]));
        let eq12_e2040_d_n9: f64 = ((s.dn[114][9] * s.v[475]) + (s.v[114] * s.dn[475][9]));
        let eq12_e2040_d_n10: f64 = ((s.dn[114][10] * s.v[475]) + (s.v[114] * s.dn[475][10]));
        let eq12_e2040_d_n11: f64 = ((s.dn[114][11] * s.v[475]) + (s.v[114] * s.dn[475][11]));
        let eq12_e2040_d_n12: f64 = ((s.dn[114][12] * s.v[475]) + (s.v[114] * s.dn[475][12]));
        let eq12_e2040_d_n13: f64 = ((s.dn[114][13] * s.v[475]) + (s.v[114] * s.dn[475][13]));
        let eq12_e2040_d_n14: f64 = ((s.dn[114][14] * s.v[475]) + (s.v[114] * s.dn[475][14]));
        let eq12_e2040_d_n15: f64 = ((s.dn[114][15] * s.v[475]) + (s.v[114] * s.dn[475][15]));
        let eq12_e2040_d_n16: f64 = ((s.dn[114][16] * s.v[475]) + (s.v[114] * s.dn[475][16]));
        let eq12_e2040_d_b0: f64 = ((s.db[114][0] * s.v[475]) + (s.v[114] * s.db[475][0]));
        let eq12_e2040_d_b1: f64 = ((s.db[114][1] * s.v[475]) + (s.v[114] * s.db[475][1]));
        let eq12_e2040_d_b2: f64 = ((s.db[114][2] * s.v[475]) + (s.v[114] * s.db[475][2]));
        let eq12_e2040_d_b3: f64 = ((s.db[114][3] * s.v[475]) + (s.v[114] * s.db[475][3]));
        let eq12_e2040_d_b4: f64 = ((s.db[114][4] * s.v[475]) + (s.v[114] * s.db[475][4]));
        let eq12_e2040_d_b5: f64 = ((s.db[114][5] * s.v[475]) + (s.v[114] * s.db[475][5]));
        let eq12_e2040_d_b6: f64 = ((s.db[114][6] * s.v[475]) + (s.v[114] * s.db[475][6]));
        let eq12_e2040_d_b7: f64 = ((s.db[114][7] * s.v[475]) + (s.v[114] * s.db[475][7]));
        let eq12_e2040_d_b8: f64 = ((s.db[114][8] * s.v[475]) + (s.v[114] * s.db[475][8]));
        let eq12_e2040_d_b9: f64 = ((s.db[114][9] * s.v[475]) + (s.v[114] * s.db[475][9]));
        let eq12_e2040_d_b10: f64 = ((s.db[114][10] * s.v[475]) + (s.v[114] * s.db[475][10]));
        let eq12_e2040_d_b11: f64 = ((s.db[114][11] * s.v[475]) + (s.v[114] * s.db[475][11]));
        let eq12_e2040_d_b12: f64 = ((s.db[114][12] * s.v[475]) + (s.v[114] * s.db[475][12]));
        let eq12_e2040_d_b13: f64 = ((s.db[114][13] * s.v[475]) + (s.v[114] * s.db[475][13]));
        let eq12_e2040_d_b14: f64 = ((s.db[114][14] * s.v[475]) + (s.v[114] * s.db[475][14]));
        let eq12_e2040_d_b15: f64 = ((s.db[114][15] * s.v[475]) + (s.v[114] * s.db[475][15]));
        let eq12_e2040_d_b16: f64 = ((s.db[114][16] * s.v[475]) + (s.v[114] * s.db[475][16]));
        let eq12_e2040_d_b17: f64 = ((s.db[114][17] * s.v[475]) + (s.v[114] * s.db[475][17]));
        (eq12_e2040, eq12_e2040_d_n0, eq12_e2040_d_n1, eq12_e2040_d_n2, eq12_e2040_d_n3, eq12_e2040_d_n4, eq12_e2040_d_n5, eq12_e2040_d_n6, eq12_e2040_d_n7, eq12_e2040_d_n8, eq12_e2040_d_n9, eq12_e2040_d_n10, eq12_e2040_d_n11, eq12_e2040_d_n12, eq12_e2040_d_n13, eq12_e2040_d_n14, eq12_e2040_d_n15, eq12_e2040_d_n16, eq12_e2040_d_b0, eq12_e2040_d_b1, eq12_e2040_d_b2, eq12_e2040_d_b3, eq12_e2040_d_b4, eq12_e2040_d_b5, eq12_e2040_d_b6, eq12_e2040_d_b7, eq12_e2040_d_b8, eq12_e2040_d_b9, eq12_e2040_d_b10, eq12_e2040_d_b11, eq12_e2040_d_b12, eq12_e2040_d_b13, eq12_e2040_d_b14, eq12_e2040_d_b15, eq12_e2040_d_b16, eq12_e2040_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e2042;
        let eq12_node_derivatives: [f64; 17] = [eq12_e2042_d_n0, eq12_e2042_d_n1, eq12_e2042_d_n2, eq12_e2042_d_n3, eq12_e2042_d_n4, eq12_e2042_d_n5, eq12_e2042_d_n6, eq12_e2042_d_n7, eq12_e2042_d_n8, eq12_e2042_d_n9, eq12_e2042_d_n10, eq12_e2042_d_n11, eq12_e2042_d_n12, eq12_e2042_d_n13, eq12_e2042_d_n14, eq12_e2042_d_n15, eq12_e2042_d_n16];
        let eq12_branch_derivatives: [f64; 18] = [eq12_e2042_d_b0, eq12_e2042_d_b1, eq12_e2042_d_b2, eq12_e2042_d_b3, eq12_e2042_d_b4, eq12_e2042_d_b5, eq12_e2042_d_b6, eq12_e2042_d_b7, eq12_e2042_d_b8, eq12_e2042_d_b9, eq12_e2042_d_b10, eq12_e2042_d_b11, eq12_e2042_d_b12, eq12_e2042_d_b13, eq12_e2042_d_b14, eq12_e2042_d_b15, eq12_e2042_d_b16, eq12_e2042_d_b17];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let (eq13_e2052, eq13_e2052_d_n0, eq13_e2052_d_n1, eq13_e2052_d_n2, eq13_e2052_d_n3, eq13_e2052_d_n4, eq13_e2052_d_n5, eq13_e2052_d_n6, eq13_e2052_d_n7, eq13_e2052_d_n8, eq13_e2052_d_n9, eq13_e2052_d_n10, eq13_e2052_d_n11, eq13_e2052_d_n12, eq13_e2052_d_n13, eq13_e2052_d_n14, eq13_e2052_d_n15, eq13_e2052_d_n16, eq13_e2052_d_b0, eq13_e2052_d_b1, eq13_e2052_d_b2, eq13_e2052_d_b3, eq13_e2052_d_b4, eq13_e2052_d_b5, eq13_e2052_d_b6, eq13_e2052_d_b7, eq13_e2052_d_b8, eq13_e2052_d_b9, eq13_e2052_d_b10, eq13_e2052_d_b11, eq13_e2052_d_b12, eq13_e2052_d_b13, eq13_e2052_d_b14, eq13_e2052_d_b15, eq13_e2052_d_b16, eq13_e2052_d_b17,) = {
    if ((s.b[1698] && s.b[1699]) && s.b[1700]) {
        let eq13_e2050: f64 = (s.v[114] * s.v[478]);
        let eq13_e2050_d_n0: f64 = ((s.dn[114][0] * s.v[478]) + (s.v[114] * s.dn[478][0]));
        let eq13_e2050_d_n1: f64 = ((s.dn[114][1] * s.v[478]) + (s.v[114] * s.dn[478][1]));
        let eq13_e2050_d_n2: f64 = ((s.dn[114][2] * s.v[478]) + (s.v[114] * s.dn[478][2]));
        let eq13_e2050_d_n3: f64 = ((s.dn[114][3] * s.v[478]) + (s.v[114] * s.dn[478][3]));
        let eq13_e2050_d_n4: f64 = ((s.dn[114][4] * s.v[478]) + (s.v[114] * s.dn[478][4]));
        let eq13_e2050_d_n5: f64 = ((s.dn[114][5] * s.v[478]) + (s.v[114] * s.dn[478][5]));
        let eq13_e2050_d_n6: f64 = ((s.dn[114][6] * s.v[478]) + (s.v[114] * s.dn[478][6]));
        let eq13_e2050_d_n7: f64 = ((s.dn[114][7] * s.v[478]) + (s.v[114] * s.dn[478][7]));
        let eq13_e2050_d_n8: f64 = ((s.dn[114][8] * s.v[478]) + (s.v[114] * s.dn[478][8]));
        let eq13_e2050_d_n9: f64 = ((s.dn[114][9] * s.v[478]) + (s.v[114] * s.dn[478][9]));
        let eq13_e2050_d_n10: f64 = ((s.dn[114][10] * s.v[478]) + (s.v[114] * s.dn[478][10]));
        let eq13_e2050_d_n11: f64 = ((s.dn[114][11] * s.v[478]) + (s.v[114] * s.dn[478][11]));
        let eq13_e2050_d_n12: f64 = ((s.dn[114][12] * s.v[478]) + (s.v[114] * s.dn[478][12]));
        let eq13_e2050_d_n13: f64 = ((s.dn[114][13] * s.v[478]) + (s.v[114] * s.dn[478][13]));
        let eq13_e2050_d_n14: f64 = ((s.dn[114][14] * s.v[478]) + (s.v[114] * s.dn[478][14]));
        let eq13_e2050_d_n15: f64 = ((s.dn[114][15] * s.v[478]) + (s.v[114] * s.dn[478][15]));
        let eq13_e2050_d_n16: f64 = ((s.dn[114][16] * s.v[478]) + (s.v[114] * s.dn[478][16]));
        let eq13_e2050_d_b0: f64 = ((s.db[114][0] * s.v[478]) + (s.v[114] * s.db[478][0]));
        let eq13_e2050_d_b1: f64 = ((s.db[114][1] * s.v[478]) + (s.v[114] * s.db[478][1]));
        let eq13_e2050_d_b2: f64 = ((s.db[114][2] * s.v[478]) + (s.v[114] * s.db[478][2]));
        let eq13_e2050_d_b3: f64 = ((s.db[114][3] * s.v[478]) + (s.v[114] * s.db[478][3]));
        let eq13_e2050_d_b4: f64 = ((s.db[114][4] * s.v[478]) + (s.v[114] * s.db[478][4]));
        let eq13_e2050_d_b5: f64 = ((s.db[114][5] * s.v[478]) + (s.v[114] * s.db[478][5]));
        let eq13_e2050_d_b6: f64 = ((s.db[114][6] * s.v[478]) + (s.v[114] * s.db[478][6]));
        let eq13_e2050_d_b7: f64 = ((s.db[114][7] * s.v[478]) + (s.v[114] * s.db[478][7]));
        let eq13_e2050_d_b8: f64 = ((s.db[114][8] * s.v[478]) + (s.v[114] * s.db[478][8]));
        let eq13_e2050_d_b9: f64 = ((s.db[114][9] * s.v[478]) + (s.v[114] * s.db[478][9]));
        let eq13_e2050_d_b10: f64 = ((s.db[114][10] * s.v[478]) + (s.v[114] * s.db[478][10]));
        let eq13_e2050_d_b11: f64 = ((s.db[114][11] * s.v[478]) + (s.v[114] * s.db[478][11]));
        let eq13_e2050_d_b12: f64 = ((s.db[114][12] * s.v[478]) + (s.v[114] * s.db[478][12]));
        let eq13_e2050_d_b13: f64 = ((s.db[114][13] * s.v[478]) + (s.v[114] * s.db[478][13]));
        let eq13_e2050_d_b14: f64 = ((s.db[114][14] * s.v[478]) + (s.v[114] * s.db[478][14]));
        let eq13_e2050_d_b15: f64 = ((s.db[114][15] * s.v[478]) + (s.v[114] * s.db[478][15]));
        let eq13_e2050_d_b16: f64 = ((s.db[114][16] * s.v[478]) + (s.v[114] * s.db[478][16]));
        let eq13_e2050_d_b17: f64 = ((s.db[114][17] * s.v[478]) + (s.v[114] * s.db[478][17]));
        (eq13_e2050, eq13_e2050_d_n0, eq13_e2050_d_n1, eq13_e2050_d_n2, eq13_e2050_d_n3, eq13_e2050_d_n4, eq13_e2050_d_n5, eq13_e2050_d_n6, eq13_e2050_d_n7, eq13_e2050_d_n8, eq13_e2050_d_n9, eq13_e2050_d_n10, eq13_e2050_d_n11, eq13_e2050_d_n12, eq13_e2050_d_n13, eq13_e2050_d_n14, eq13_e2050_d_n15, eq13_e2050_d_n16, eq13_e2050_d_b0, eq13_e2050_d_b1, eq13_e2050_d_b2, eq13_e2050_d_b3, eq13_e2050_d_b4, eq13_e2050_d_b5, eq13_e2050_d_b6, eq13_e2050_d_b7, eq13_e2050_d_b8, eq13_e2050_d_b9, eq13_e2050_d_b10, eq13_e2050_d_b11, eq13_e2050_d_b12, eq13_e2050_d_b13, eq13_e2050_d_b14, eq13_e2050_d_b15, eq13_e2050_d_b16, eq13_e2050_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e2052;
        let eq13_node_derivatives: [f64; 17] = [eq13_e2052_d_n0, eq13_e2052_d_n1, eq13_e2052_d_n2, eq13_e2052_d_n3, eq13_e2052_d_n4, eq13_e2052_d_n5, eq13_e2052_d_n6, eq13_e2052_d_n7, eq13_e2052_d_n8, eq13_e2052_d_n9, eq13_e2052_d_n10, eq13_e2052_d_n11, eq13_e2052_d_n12, eq13_e2052_d_n13, eq13_e2052_d_n14, eq13_e2052_d_n15, eq13_e2052_d_n16];
        let eq13_branch_derivatives: [f64; 18] = [eq13_e2052_d_b0, eq13_e2052_d_b1, eq13_e2052_d_b2, eq13_e2052_d_b3, eq13_e2052_d_b4, eq13_e2052_d_b5, eq13_e2052_d_b6, eq13_e2052_d_b7, eq13_e2052_d_b8, eq13_e2052_d_b9, eq13_e2052_d_b10, eq13_e2052_d_b11, eq13_e2052_d_b12, eq13_e2052_d_b13, eq13_e2052_d_b14, eq13_e2052_d_b15, eq13_e2052_d_b16, eq13_e2052_d_b17];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(3),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
    ) {
        let (eq14_e2062, eq14_e2062_d_n0, eq14_e2062_d_n1, eq14_e2062_d_n2, eq14_e2062_d_n3, eq14_e2062_d_n4, eq14_e2062_d_n5, eq14_e2062_d_n6, eq14_e2062_d_n7, eq14_e2062_d_n8, eq14_e2062_d_n9, eq14_e2062_d_n10, eq14_e2062_d_n11, eq14_e2062_d_n12, eq14_e2062_d_n13, eq14_e2062_d_n14, eq14_e2062_d_n15, eq14_e2062_d_n16, eq14_e2062_d_b0, eq14_e2062_d_b1, eq14_e2062_d_b2, eq14_e2062_d_b3, eq14_e2062_d_b4, eq14_e2062_d_b5, eq14_e2062_d_b6, eq14_e2062_d_b7, eq14_e2062_d_b8, eq14_e2062_d_b9, eq14_e2062_d_b10, eq14_e2062_d_b11, eq14_e2062_d_b12, eq14_e2062_d_b13, eq14_e2062_d_b14, eq14_e2062_d_b15, eq14_e2062_d_b16, eq14_e2062_d_b17,) = {
    if ((s.b[1698] && s.b[1699]) && s.b[1700]) {
        let eq14_e2060: f64 = (s.v[114] * s.v[477]);
        let eq14_e2060_d_n0: f64 = ((s.dn[114][0] * s.v[477]) + (s.v[114] * s.dn[477][0]));
        let eq14_e2060_d_n1: f64 = ((s.dn[114][1] * s.v[477]) + (s.v[114] * s.dn[477][1]));
        let eq14_e2060_d_n2: f64 = ((s.dn[114][2] * s.v[477]) + (s.v[114] * s.dn[477][2]));
        let eq14_e2060_d_n3: f64 = ((s.dn[114][3] * s.v[477]) + (s.v[114] * s.dn[477][3]));
        let eq14_e2060_d_n4: f64 = ((s.dn[114][4] * s.v[477]) + (s.v[114] * s.dn[477][4]));
        let eq14_e2060_d_n5: f64 = ((s.dn[114][5] * s.v[477]) + (s.v[114] * s.dn[477][5]));
        let eq14_e2060_d_n6: f64 = ((s.dn[114][6] * s.v[477]) + (s.v[114] * s.dn[477][6]));
        let eq14_e2060_d_n7: f64 = ((s.dn[114][7] * s.v[477]) + (s.v[114] * s.dn[477][7]));
        let eq14_e2060_d_n8: f64 = ((s.dn[114][8] * s.v[477]) + (s.v[114] * s.dn[477][8]));
        let eq14_e2060_d_n9: f64 = ((s.dn[114][9] * s.v[477]) + (s.v[114] * s.dn[477][9]));
        let eq14_e2060_d_n10: f64 = ((s.dn[114][10] * s.v[477]) + (s.v[114] * s.dn[477][10]));
        let eq14_e2060_d_n11: f64 = ((s.dn[114][11] * s.v[477]) + (s.v[114] * s.dn[477][11]));
        let eq14_e2060_d_n12: f64 = ((s.dn[114][12] * s.v[477]) + (s.v[114] * s.dn[477][12]));
        let eq14_e2060_d_n13: f64 = ((s.dn[114][13] * s.v[477]) + (s.v[114] * s.dn[477][13]));
        let eq14_e2060_d_n14: f64 = ((s.dn[114][14] * s.v[477]) + (s.v[114] * s.dn[477][14]));
        let eq14_e2060_d_n15: f64 = ((s.dn[114][15] * s.v[477]) + (s.v[114] * s.dn[477][15]));
        let eq14_e2060_d_n16: f64 = ((s.dn[114][16] * s.v[477]) + (s.v[114] * s.dn[477][16]));
        let eq14_e2060_d_b0: f64 = ((s.db[114][0] * s.v[477]) + (s.v[114] * s.db[477][0]));
        let eq14_e2060_d_b1: f64 = ((s.db[114][1] * s.v[477]) + (s.v[114] * s.db[477][1]));
        let eq14_e2060_d_b2: f64 = ((s.db[114][2] * s.v[477]) + (s.v[114] * s.db[477][2]));
        let eq14_e2060_d_b3: f64 = ((s.db[114][3] * s.v[477]) + (s.v[114] * s.db[477][3]));
        let eq14_e2060_d_b4: f64 = ((s.db[114][4] * s.v[477]) + (s.v[114] * s.db[477][4]));
        let eq14_e2060_d_b5: f64 = ((s.db[114][5] * s.v[477]) + (s.v[114] * s.db[477][5]));
        let eq14_e2060_d_b6: f64 = ((s.db[114][6] * s.v[477]) + (s.v[114] * s.db[477][6]));
        let eq14_e2060_d_b7: f64 = ((s.db[114][7] * s.v[477]) + (s.v[114] * s.db[477][7]));
        let eq14_e2060_d_b8: f64 = ((s.db[114][8] * s.v[477]) + (s.v[114] * s.db[477][8]));
        let eq14_e2060_d_b9: f64 = ((s.db[114][9] * s.v[477]) + (s.v[114] * s.db[477][9]));
        let eq14_e2060_d_b10: f64 = ((s.db[114][10] * s.v[477]) + (s.v[114] * s.db[477][10]));
        let eq14_e2060_d_b11: f64 = ((s.db[114][11] * s.v[477]) + (s.v[114] * s.db[477][11]));
        let eq14_e2060_d_b12: f64 = ((s.db[114][12] * s.v[477]) + (s.v[114] * s.db[477][12]));
        let eq14_e2060_d_b13: f64 = ((s.db[114][13] * s.v[477]) + (s.v[114] * s.db[477][13]));
        let eq14_e2060_d_b14: f64 = ((s.db[114][14] * s.v[477]) + (s.v[114] * s.db[477][14]));
        let eq14_e2060_d_b15: f64 = ((s.db[114][15] * s.v[477]) + (s.v[114] * s.db[477][15]));
        let eq14_e2060_d_b16: f64 = ((s.db[114][16] * s.v[477]) + (s.v[114] * s.db[477][16]));
        let eq14_e2060_d_b17: f64 = ((s.db[114][17] * s.v[477]) + (s.v[114] * s.db[477][17]));
        (eq14_e2060, eq14_e2060_d_n0, eq14_e2060_d_n1, eq14_e2060_d_n2, eq14_e2060_d_n3, eq14_e2060_d_n4, eq14_e2060_d_n5, eq14_e2060_d_n6, eq14_e2060_d_n7, eq14_e2060_d_n8, eq14_e2060_d_n9, eq14_e2060_d_n10, eq14_e2060_d_n11, eq14_e2060_d_n12, eq14_e2060_d_n13, eq14_e2060_d_n14, eq14_e2060_d_n15, eq14_e2060_d_n16, eq14_e2060_d_b0, eq14_e2060_d_b1, eq14_e2060_d_b2, eq14_e2060_d_b3, eq14_e2060_d_b4, eq14_e2060_d_b5, eq14_e2060_d_b6, eq14_e2060_d_b7, eq14_e2060_d_b8, eq14_e2060_d_b9, eq14_e2060_d_b10, eq14_e2060_d_b11, eq14_e2060_d_b12, eq14_e2060_d_b13, eq14_e2060_d_b14, eq14_e2060_d_b15, eq14_e2060_d_b16, eq14_e2060_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e2062;
        let eq14_node_derivatives: [f64; 17] = [eq14_e2062_d_n0, eq14_e2062_d_n1, eq14_e2062_d_n2, eq14_e2062_d_n3, eq14_e2062_d_n4, eq14_e2062_d_n5, eq14_e2062_d_n6, eq14_e2062_d_n7, eq14_e2062_d_n8, eq14_e2062_d_n9, eq14_e2062_d_n10, eq14_e2062_d_n11, eq14_e2062_d_n12, eq14_e2062_d_n13, eq14_e2062_d_n14, eq14_e2062_d_n15, eq14_e2062_d_n16];
        let eq14_branch_derivatives: [f64; 18] = [eq14_e2062_d_b0, eq14_e2062_d_b1, eq14_e2062_d_b2, eq14_e2062_d_b3, eq14_e2062_d_b4, eq14_e2062_d_b5, eq14_e2062_d_b6, eq14_e2062_d_b7, eq14_e2062_d_b8, eq14_e2062_d_b9, eq14_e2062_d_b10, eq14_e2062_d_b11, eq14_e2062_d_b12, eq14_e2062_d_b13, eq14_e2062_d_b14, eq14_e2062_d_b15, eq14_e2062_d_b16, eq14_e2062_d_b17];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(3),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let (eq15_e2075, eq15_e2075_d_n0, eq15_e2075_d_n1, eq15_e2075_d_n2, eq15_e2075_d_n3, eq15_e2075_d_n4, eq15_e2075_d_n5, eq15_e2075_d_n6, eq15_e2075_d_n7, eq15_e2075_d_n8, eq15_e2075_d_n9, eq15_e2075_d_n10, eq15_e2075_d_n11, eq15_e2075_d_n12, eq15_e2075_d_n13, eq15_e2075_d_n14, eq15_e2075_d_n15, eq15_e2075_d_n16, eq15_e2075_d_b0, eq15_e2075_d_b1, eq15_e2075_d_b2, eq15_e2075_d_b3, eq15_e2075_d_b4, eq15_e2075_d_b5, eq15_e2075_d_b6, eq15_e2075_d_b7, eq15_e2075_d_b8, eq15_e2075_d_b9, eq15_e2075_d_b10, eq15_e2075_d_b11, eq15_e2075_d_b12, eq15_e2075_d_b13, eq15_e2075_d_b14, eq15_e2075_d_b15, eq15_e2075_d_b16, eq15_e2075_d_b17,) = {
    if ((s.b[1698] && s.b[1699]) && (!s.b[1700])) {
        let eq15_e2072: f64 = (s.v[476] + s.v[488]);
        let eq15_e2072_d_n0: f64 = (s.dn[476][0] + s.dn[488][0]);
        let eq15_e2072_d_n1: f64 = (s.dn[476][1] + s.dn[488][1]);
        let eq15_e2072_d_n2: f64 = (s.dn[476][2] + s.dn[488][2]);
        let eq15_e2072_d_n3: f64 = (s.dn[476][3] + s.dn[488][3]);
        let eq15_e2072_d_n4: f64 = (s.dn[476][4] + s.dn[488][4]);
        let eq15_e2072_d_n5: f64 = (s.dn[476][5] + s.dn[488][5]);
        let eq15_e2072_d_n6: f64 = (s.dn[476][6] + s.dn[488][6]);
        let eq15_e2072_d_n7: f64 = (s.dn[476][7] + s.dn[488][7]);
        let eq15_e2072_d_n8: f64 = (s.dn[476][8] + s.dn[488][8]);
        let eq15_e2072_d_n9: f64 = (s.dn[476][9] + s.dn[488][9]);
        let eq15_e2072_d_n10: f64 = (s.dn[476][10] + s.dn[488][10]);
        let eq15_e2072_d_n11: f64 = (s.dn[476][11] + s.dn[488][11]);
        let eq15_e2072_d_n12: f64 = (s.dn[476][12] + s.dn[488][12]);
        let eq15_e2072_d_n13: f64 = (s.dn[476][13] + s.dn[488][13]);
        let eq15_e2072_d_n14: f64 = (s.dn[476][14] + s.dn[488][14]);
        let eq15_e2072_d_n15: f64 = (s.dn[476][15] + s.dn[488][15]);
        let eq15_e2072_d_n16: f64 = (s.dn[476][16] + s.dn[488][16]);
        let eq15_e2072_d_b0: f64 = (s.db[476][0] + s.db[488][0]);
        let eq15_e2072_d_b1: f64 = (s.db[476][1] + s.db[488][1]);
        let eq15_e2072_d_b2: f64 = (s.db[476][2] + s.db[488][2]);
        let eq15_e2072_d_b3: f64 = (s.db[476][3] + s.db[488][3]);
        let eq15_e2072_d_b4: f64 = (s.db[476][4] + s.db[488][4]);
        let eq15_e2072_d_b5: f64 = (s.db[476][5] + s.db[488][5]);
        let eq15_e2072_d_b6: f64 = (s.db[476][6] + s.db[488][6]);
        let eq15_e2072_d_b7: f64 = (s.db[476][7] + s.db[488][7]);
        let eq15_e2072_d_b8: f64 = (s.db[476][8] + s.db[488][8]);
        let eq15_e2072_d_b9: f64 = (s.db[476][9] + s.db[488][9]);
        let eq15_e2072_d_b10: f64 = (s.db[476][10] + s.db[488][10]);
        let eq15_e2072_d_b11: f64 = (s.db[476][11] + s.db[488][11]);
        let eq15_e2072_d_b12: f64 = (s.db[476][12] + s.db[488][12]);
        let eq15_e2072_d_b13: f64 = (s.db[476][13] + s.db[488][13]);
        let eq15_e2072_d_b14: f64 = (s.db[476][14] + s.db[488][14]);
        let eq15_e2072_d_b15: f64 = (s.db[476][15] + s.db[488][15]);
        let eq15_e2072_d_b16: f64 = (s.db[476][16] + s.db[488][16]);
        let eq15_e2072_d_b17: f64 = (s.db[476][17] + s.db[488][17]);
        let eq15_e2073: f64 = (s.v[114] * eq15_e2072);
        let eq15_e2073_d_n0: f64 = ((s.dn[114][0] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n0));
        let eq15_e2073_d_n1: f64 = ((s.dn[114][1] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n1));
        let eq15_e2073_d_n2: f64 = ((s.dn[114][2] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n2));
        let eq15_e2073_d_n3: f64 = ((s.dn[114][3] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n3));
        let eq15_e2073_d_n4: f64 = ((s.dn[114][4] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n4));
        let eq15_e2073_d_n5: f64 = ((s.dn[114][5] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n5));
        let eq15_e2073_d_n6: f64 = ((s.dn[114][6] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n6));
        let eq15_e2073_d_n7: f64 = ((s.dn[114][7] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n7));
        let eq15_e2073_d_n8: f64 = ((s.dn[114][8] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n8));
        let eq15_e2073_d_n9: f64 = ((s.dn[114][9] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n9));
        let eq15_e2073_d_n10: f64 = ((s.dn[114][10] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n10));
        let eq15_e2073_d_n11: f64 = ((s.dn[114][11] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n11));
        let eq15_e2073_d_n12: f64 = ((s.dn[114][12] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n12));
        let eq15_e2073_d_n13: f64 = ((s.dn[114][13] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n13));
        let eq15_e2073_d_n14: f64 = ((s.dn[114][14] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n14));
        let eq15_e2073_d_n15: f64 = ((s.dn[114][15] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n15));
        let eq15_e2073_d_n16: f64 = ((s.dn[114][16] * eq15_e2072) + (s.v[114] * eq15_e2072_d_n16));
        let eq15_e2073_d_b0: f64 = ((s.db[114][0] * eq15_e2072) + (s.v[114] * eq15_e2072_d_b0));
        let eq15_e2073_d_b1: f64 = ((s.db[114][1] * eq15_e2072) + (s.v[114] * eq15_e2072_d_b1));
        let eq15_e2073_d_b2: f64 = ((s.db[114][2] * eq15_e2072) + (s.v[114] * eq15_e2072_d_b2));
        let eq15_e2073_d_b3: f64 = ((s.db[114][3] * eq15_e2072) + (s.v[114] * eq15_e2072_d_b3));
        let eq15_e2073_d_b4: f64 = ((s.db[114][4] * eq15_e2072) + (s.v[114] * eq15_e2072_d_b4));
        let eq15_e2073_d_b5: f64 = ((s.db[114][5] * eq15_e2072) + (s.v[114] * eq15_e2072_d_b5));
        let eq15_e2073_d_b6: f64 = ((s.db[114][6] * eq15_e2072) + (s.v[114] * eq15_e2072_d_b6));
        let eq15_e2073_d_b7: f64 = ((s.db[114][7] * eq15_e2072) + (s.v[114] * eq15_e2072_d_b7));
        let eq15_e2073_d_b8: f64 = ((s.db[114][8] * eq15_e2072) + (s.v[114] * eq15_e2072_d_b8));
        let eq15_e2073_d_b9: f64 = ((s.db[114][9] * eq15_e2072) + (s.v[114] * eq15_e2072_d_b9));
        let eq15_e2073_d_b10: f64 = ((s.db[114][10] * eq15_e2072) + (s.v[114] * eq15_e2072_d_b10));
        let eq15_e2073_d_b11: f64 = ((s.db[114][11] * eq15_e2072) + (s.v[114] * eq15_e2072_d_b11));
        let eq15_e2073_d_b12: f64 = ((s.db[114][12] * eq15_e2072) + (s.v[114] * eq15_e2072_d_b12));
        let eq15_e2073_d_b13: f64 = ((s.db[114][13] * eq15_e2072) + (s.v[114] * eq15_e2072_d_b13));
        let eq15_e2073_d_b14: f64 = ((s.db[114][14] * eq15_e2072) + (s.v[114] * eq15_e2072_d_b14));
        let eq15_e2073_d_b15: f64 = ((s.db[114][15] * eq15_e2072) + (s.v[114] * eq15_e2072_d_b15));
        let eq15_e2073_d_b16: f64 = ((s.db[114][16] * eq15_e2072) + (s.v[114] * eq15_e2072_d_b16));
        let eq15_e2073_d_b17: f64 = ((s.db[114][17] * eq15_e2072) + (s.v[114] * eq15_e2072_d_b17));
        (eq15_e2073, eq15_e2073_d_n0, eq15_e2073_d_n1, eq15_e2073_d_n2, eq15_e2073_d_n3, eq15_e2073_d_n4, eq15_e2073_d_n5, eq15_e2073_d_n6, eq15_e2073_d_n7, eq15_e2073_d_n8, eq15_e2073_d_n9, eq15_e2073_d_n10, eq15_e2073_d_n11, eq15_e2073_d_n12, eq15_e2073_d_n13, eq15_e2073_d_n14, eq15_e2073_d_n15, eq15_e2073_d_n16, eq15_e2073_d_b0, eq15_e2073_d_b1, eq15_e2073_d_b2, eq15_e2073_d_b3, eq15_e2073_d_b4, eq15_e2073_d_b5, eq15_e2073_d_b6, eq15_e2073_d_b7, eq15_e2073_d_b8, eq15_e2073_d_b9, eq15_e2073_d_b10, eq15_e2073_d_b11, eq15_e2073_d_b12, eq15_e2073_d_b13, eq15_e2073_d_b14, eq15_e2073_d_b15, eq15_e2073_d_b16, eq15_e2073_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e2075;
        let eq15_node_derivatives: [f64; 17] = [eq15_e2075_d_n0, eq15_e2075_d_n1, eq15_e2075_d_n2, eq15_e2075_d_n3, eq15_e2075_d_n4, eq15_e2075_d_n5, eq15_e2075_d_n6, eq15_e2075_d_n7, eq15_e2075_d_n8, eq15_e2075_d_n9, eq15_e2075_d_n10, eq15_e2075_d_n11, eq15_e2075_d_n12, eq15_e2075_d_n13, eq15_e2075_d_n14, eq15_e2075_d_n15, eq15_e2075_d_n16];
        let eq15_branch_derivatives: [f64; 18] = [eq15_e2075_d_b0, eq15_e2075_d_b1, eq15_e2075_d_b2, eq15_e2075_d_b3, eq15_e2075_d_b4, eq15_e2075_d_b5, eq15_e2075_d_b6, eq15_e2075_d_b7, eq15_e2075_d_b8, eq15_e2075_d_b9, eq15_e2075_d_b10, eq15_e2075_d_b11, eq15_e2075_d_b12, eq15_e2075_d_b13, eq15_e2075_d_b14, eq15_e2075_d_b15, eq15_e2075_d_b16, eq15_e2075_d_b17];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(3),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let (eq16_e2086, eq16_e2086_d_n0, eq16_e2086_d_n1, eq16_e2086_d_n2, eq16_e2086_d_n3, eq16_e2086_d_n4, eq16_e2086_d_n5, eq16_e2086_d_n6, eq16_e2086_d_n7, eq16_e2086_d_n8, eq16_e2086_d_n9, eq16_e2086_d_n10, eq16_e2086_d_n11, eq16_e2086_d_n12, eq16_e2086_d_n13, eq16_e2086_d_n14, eq16_e2086_d_n15, eq16_e2086_d_n16, eq16_e2086_d_b0, eq16_e2086_d_b1, eq16_e2086_d_b2, eq16_e2086_d_b3, eq16_e2086_d_b4, eq16_e2086_d_b5, eq16_e2086_d_b6, eq16_e2086_d_b7, eq16_e2086_d_b8, eq16_e2086_d_b9, eq16_e2086_d_b10, eq16_e2086_d_b11, eq16_e2086_d_b12, eq16_e2086_d_b13, eq16_e2086_d_b14, eq16_e2086_d_b15, eq16_e2086_d_b16, eq16_e2086_d_b17,) = {
    if ((s.b[1698] && s.b[1699]) && (!s.b[1700])) {
        let eq16_e2084: f64 = (s.v[114] * s.v[475]);
        let eq16_e2084_d_n0: f64 = ((s.dn[114][0] * s.v[475]) + (s.v[114] * s.dn[475][0]));
        let eq16_e2084_d_n1: f64 = ((s.dn[114][1] * s.v[475]) + (s.v[114] * s.dn[475][1]));
        let eq16_e2084_d_n2: f64 = ((s.dn[114][2] * s.v[475]) + (s.v[114] * s.dn[475][2]));
        let eq16_e2084_d_n3: f64 = ((s.dn[114][3] * s.v[475]) + (s.v[114] * s.dn[475][3]));
        let eq16_e2084_d_n4: f64 = ((s.dn[114][4] * s.v[475]) + (s.v[114] * s.dn[475][4]));
        let eq16_e2084_d_n5: f64 = ((s.dn[114][5] * s.v[475]) + (s.v[114] * s.dn[475][5]));
        let eq16_e2084_d_n6: f64 = ((s.dn[114][6] * s.v[475]) + (s.v[114] * s.dn[475][6]));
        let eq16_e2084_d_n7: f64 = ((s.dn[114][7] * s.v[475]) + (s.v[114] * s.dn[475][7]));
        let eq16_e2084_d_n8: f64 = ((s.dn[114][8] * s.v[475]) + (s.v[114] * s.dn[475][8]));
        let eq16_e2084_d_n9: f64 = ((s.dn[114][9] * s.v[475]) + (s.v[114] * s.dn[475][9]));
        let eq16_e2084_d_n10: f64 = ((s.dn[114][10] * s.v[475]) + (s.v[114] * s.dn[475][10]));
        let eq16_e2084_d_n11: f64 = ((s.dn[114][11] * s.v[475]) + (s.v[114] * s.dn[475][11]));
        let eq16_e2084_d_n12: f64 = ((s.dn[114][12] * s.v[475]) + (s.v[114] * s.dn[475][12]));
        let eq16_e2084_d_n13: f64 = ((s.dn[114][13] * s.v[475]) + (s.v[114] * s.dn[475][13]));
        let eq16_e2084_d_n14: f64 = ((s.dn[114][14] * s.v[475]) + (s.v[114] * s.dn[475][14]));
        let eq16_e2084_d_n15: f64 = ((s.dn[114][15] * s.v[475]) + (s.v[114] * s.dn[475][15]));
        let eq16_e2084_d_n16: f64 = ((s.dn[114][16] * s.v[475]) + (s.v[114] * s.dn[475][16]));
        let eq16_e2084_d_b0: f64 = ((s.db[114][0] * s.v[475]) + (s.v[114] * s.db[475][0]));
        let eq16_e2084_d_b1: f64 = ((s.db[114][1] * s.v[475]) + (s.v[114] * s.db[475][1]));
        let eq16_e2084_d_b2: f64 = ((s.db[114][2] * s.v[475]) + (s.v[114] * s.db[475][2]));
        let eq16_e2084_d_b3: f64 = ((s.db[114][3] * s.v[475]) + (s.v[114] * s.db[475][3]));
        let eq16_e2084_d_b4: f64 = ((s.db[114][4] * s.v[475]) + (s.v[114] * s.db[475][4]));
        let eq16_e2084_d_b5: f64 = ((s.db[114][5] * s.v[475]) + (s.v[114] * s.db[475][5]));
        let eq16_e2084_d_b6: f64 = ((s.db[114][6] * s.v[475]) + (s.v[114] * s.db[475][6]));
        let eq16_e2084_d_b7: f64 = ((s.db[114][7] * s.v[475]) + (s.v[114] * s.db[475][7]));
        let eq16_e2084_d_b8: f64 = ((s.db[114][8] * s.v[475]) + (s.v[114] * s.db[475][8]));
        let eq16_e2084_d_b9: f64 = ((s.db[114][9] * s.v[475]) + (s.v[114] * s.db[475][9]));
        let eq16_e2084_d_b10: f64 = ((s.db[114][10] * s.v[475]) + (s.v[114] * s.db[475][10]));
        let eq16_e2084_d_b11: f64 = ((s.db[114][11] * s.v[475]) + (s.v[114] * s.db[475][11]));
        let eq16_e2084_d_b12: f64 = ((s.db[114][12] * s.v[475]) + (s.v[114] * s.db[475][12]));
        let eq16_e2084_d_b13: f64 = ((s.db[114][13] * s.v[475]) + (s.v[114] * s.db[475][13]));
        let eq16_e2084_d_b14: f64 = ((s.db[114][14] * s.v[475]) + (s.v[114] * s.db[475][14]));
        let eq16_e2084_d_b15: f64 = ((s.db[114][15] * s.v[475]) + (s.v[114] * s.db[475][15]));
        let eq16_e2084_d_b16: f64 = ((s.db[114][16] * s.v[475]) + (s.v[114] * s.db[475][16]));
        let eq16_e2084_d_b17: f64 = ((s.db[114][17] * s.v[475]) + (s.v[114] * s.db[475][17]));
        (eq16_e2084, eq16_e2084_d_n0, eq16_e2084_d_n1, eq16_e2084_d_n2, eq16_e2084_d_n3, eq16_e2084_d_n4, eq16_e2084_d_n5, eq16_e2084_d_n6, eq16_e2084_d_n7, eq16_e2084_d_n8, eq16_e2084_d_n9, eq16_e2084_d_n10, eq16_e2084_d_n11, eq16_e2084_d_n12, eq16_e2084_d_n13, eq16_e2084_d_n14, eq16_e2084_d_n15, eq16_e2084_d_n16, eq16_e2084_d_b0, eq16_e2084_d_b1, eq16_e2084_d_b2, eq16_e2084_d_b3, eq16_e2084_d_b4, eq16_e2084_d_b5, eq16_e2084_d_b6, eq16_e2084_d_b7, eq16_e2084_d_b8, eq16_e2084_d_b9, eq16_e2084_d_b10, eq16_e2084_d_b11, eq16_e2084_d_b12, eq16_e2084_d_b13, eq16_e2084_d_b14, eq16_e2084_d_b15, eq16_e2084_d_b16, eq16_e2084_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e2086;
        let eq16_node_derivatives: [f64; 17] = [eq16_e2086_d_n0, eq16_e2086_d_n1, eq16_e2086_d_n2, eq16_e2086_d_n3, eq16_e2086_d_n4, eq16_e2086_d_n5, eq16_e2086_d_n6, eq16_e2086_d_n7, eq16_e2086_d_n8, eq16_e2086_d_n9, eq16_e2086_d_n10, eq16_e2086_d_n11, eq16_e2086_d_n12, eq16_e2086_d_n13, eq16_e2086_d_n14, eq16_e2086_d_n15, eq16_e2086_d_n16];
        let eq16_branch_derivatives: [f64; 18] = [eq16_e2086_d_b0, eq16_e2086_d_b1, eq16_e2086_d_b2, eq16_e2086_d_b3, eq16_e2086_d_b4, eq16_e2086_d_b5, eq16_e2086_d_b6, eq16_e2086_d_b7, eq16_e2086_d_b8, eq16_e2086_d_b9, eq16_e2086_d_b10, eq16_e2086_d_b11, eq16_e2086_d_b12, eq16_e2086_d_b13, eq16_e2086_d_b14, eq16_e2086_d_b15, eq16_e2086_d_b16, eq16_e2086_d_b17];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(3),
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let (eq17_e2096, eq17_e2096_d_n0, eq17_e2096_d_n1, eq17_e2096_d_n2, eq17_e2096_d_n3, eq17_e2096_d_n4, eq17_e2096_d_n5, eq17_e2096_d_n6, eq17_e2096_d_n7, eq17_e2096_d_n8, eq17_e2096_d_n9, eq17_e2096_d_n10, eq17_e2096_d_n11, eq17_e2096_d_n12, eq17_e2096_d_n13, eq17_e2096_d_n14, eq17_e2096_d_n15, eq17_e2096_d_n16, eq17_e2096_d_b0, eq17_e2096_d_b1, eq17_e2096_d_b2, eq17_e2096_d_b3, eq17_e2096_d_b4, eq17_e2096_d_b5, eq17_e2096_d_b6, eq17_e2096_d_b7, eq17_e2096_d_b8, eq17_e2096_d_b9, eq17_e2096_d_b10, eq17_e2096_d_b11, eq17_e2096_d_b12, eq17_e2096_d_b13, eq17_e2096_d_b14, eq17_e2096_d_b15, eq17_e2096_d_b16, eq17_e2096_d_b17,) = {
    if (s.b[1698] && s.b[1699]) {
        let eq17_e2093: f64 = (s.v[461] + s.v[469]);
        let eq17_e2093_d_n0: f64 = (s.dn[461][0] + s.dn[469][0]);
        let eq17_e2093_d_n1: f64 = (s.dn[461][1] + s.dn[469][1]);
        let eq17_e2093_d_n2: f64 = (s.dn[461][2] + s.dn[469][2]);
        let eq17_e2093_d_n3: f64 = (s.dn[461][3] + s.dn[469][3]);
        let eq17_e2093_d_n4: f64 = (s.dn[461][4] + s.dn[469][4]);
        let eq17_e2093_d_n5: f64 = (s.dn[461][5] + s.dn[469][5]);
        let eq17_e2093_d_n6: f64 = (s.dn[461][6] + s.dn[469][6]);
        let eq17_e2093_d_n7: f64 = (s.dn[461][7] + s.dn[469][7]);
        let eq17_e2093_d_n8: f64 = (s.dn[461][8] + s.dn[469][8]);
        let eq17_e2093_d_n9: f64 = (s.dn[461][9] + s.dn[469][9]);
        let eq17_e2093_d_n10: f64 = (s.dn[461][10] + s.dn[469][10]);
        let eq17_e2093_d_n11: f64 = (s.dn[461][11] + s.dn[469][11]);
        let eq17_e2093_d_n12: f64 = (s.dn[461][12] + s.dn[469][12]);
        let eq17_e2093_d_n13: f64 = (s.dn[461][13] + s.dn[469][13]);
        let eq17_e2093_d_n14: f64 = (s.dn[461][14] + s.dn[469][14]);
        let eq17_e2093_d_n15: f64 = (s.dn[461][15] + s.dn[469][15]);
        let eq17_e2093_d_n16: f64 = (s.dn[461][16] + s.dn[469][16]);
        let eq17_e2093_d_b0: f64 = (s.db[461][0] + s.db[469][0]);
        let eq17_e2093_d_b1: f64 = (s.db[461][1] + s.db[469][1]);
        let eq17_e2093_d_b2: f64 = (s.db[461][2] + s.db[469][2]);
        let eq17_e2093_d_b3: f64 = (s.db[461][3] + s.db[469][3]);
        let eq17_e2093_d_b4: f64 = (s.db[461][4] + s.db[469][4]);
        let eq17_e2093_d_b5: f64 = (s.db[461][5] + s.db[469][5]);
        let eq17_e2093_d_b6: f64 = (s.db[461][6] + s.db[469][6]);
        let eq17_e2093_d_b7: f64 = (s.db[461][7] + s.db[469][7]);
        let eq17_e2093_d_b8: f64 = (s.db[461][8] + s.db[469][8]);
        let eq17_e2093_d_b9: f64 = (s.db[461][9] + s.db[469][9]);
        let eq17_e2093_d_b10: f64 = (s.db[461][10] + s.db[469][10]);
        let eq17_e2093_d_b11: f64 = (s.db[461][11] + s.db[469][11]);
        let eq17_e2093_d_b12: f64 = (s.db[461][12] + s.db[469][12]);
        let eq17_e2093_d_b13: f64 = (s.db[461][13] + s.db[469][13]);
        let eq17_e2093_d_b14: f64 = (s.db[461][14] + s.db[469][14]);
        let eq17_e2093_d_b15: f64 = (s.db[461][15] + s.db[469][15]);
        let eq17_e2093_d_b16: f64 = (s.db[461][16] + s.db[469][16]);
        let eq17_e2093_d_b17: f64 = (s.db[461][17] + s.db[469][17]);
        let eq17_e2094: f64 = (s.v[114] * eq17_e2093);
        let eq17_e2094_d_n0: f64 = ((s.dn[114][0] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n0));
        let eq17_e2094_d_n1: f64 = ((s.dn[114][1] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n1));
        let eq17_e2094_d_n2: f64 = ((s.dn[114][2] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n2));
        let eq17_e2094_d_n3: f64 = ((s.dn[114][3] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n3));
        let eq17_e2094_d_n4: f64 = ((s.dn[114][4] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n4));
        let eq17_e2094_d_n5: f64 = ((s.dn[114][5] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n5));
        let eq17_e2094_d_n6: f64 = ((s.dn[114][6] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n6));
        let eq17_e2094_d_n7: f64 = ((s.dn[114][7] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n7));
        let eq17_e2094_d_n8: f64 = ((s.dn[114][8] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n8));
        let eq17_e2094_d_n9: f64 = ((s.dn[114][9] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n9));
        let eq17_e2094_d_n10: f64 = ((s.dn[114][10] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n10));
        let eq17_e2094_d_n11: f64 = ((s.dn[114][11] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n11));
        let eq17_e2094_d_n12: f64 = ((s.dn[114][12] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n12));
        let eq17_e2094_d_n13: f64 = ((s.dn[114][13] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n13));
        let eq17_e2094_d_n14: f64 = ((s.dn[114][14] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n14));
        let eq17_e2094_d_n15: f64 = ((s.dn[114][15] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n15));
        let eq17_e2094_d_n16: f64 = ((s.dn[114][16] * eq17_e2093) + (s.v[114] * eq17_e2093_d_n16));
        let eq17_e2094_d_b0: f64 = ((s.db[114][0] * eq17_e2093) + (s.v[114] * eq17_e2093_d_b0));
        let eq17_e2094_d_b1: f64 = ((s.db[114][1] * eq17_e2093) + (s.v[114] * eq17_e2093_d_b1));
        let eq17_e2094_d_b2: f64 = ((s.db[114][2] * eq17_e2093) + (s.v[114] * eq17_e2093_d_b2));
        let eq17_e2094_d_b3: f64 = ((s.db[114][3] * eq17_e2093) + (s.v[114] * eq17_e2093_d_b3));
        let eq17_e2094_d_b4: f64 = ((s.db[114][4] * eq17_e2093) + (s.v[114] * eq17_e2093_d_b4));
        let eq17_e2094_d_b5: f64 = ((s.db[114][5] * eq17_e2093) + (s.v[114] * eq17_e2093_d_b5));
        let eq17_e2094_d_b6: f64 = ((s.db[114][6] * eq17_e2093) + (s.v[114] * eq17_e2093_d_b6));
        let eq17_e2094_d_b7: f64 = ((s.db[114][7] * eq17_e2093) + (s.v[114] * eq17_e2093_d_b7));
        let eq17_e2094_d_b8: f64 = ((s.db[114][8] * eq17_e2093) + (s.v[114] * eq17_e2093_d_b8));
        let eq17_e2094_d_b9: f64 = ((s.db[114][9] * eq17_e2093) + (s.v[114] * eq17_e2093_d_b9));
        let eq17_e2094_d_b10: f64 = ((s.db[114][10] * eq17_e2093) + (s.v[114] * eq17_e2093_d_b10));
        let eq17_e2094_d_b11: f64 = ((s.db[114][11] * eq17_e2093) + (s.v[114] * eq17_e2093_d_b11));
        let eq17_e2094_d_b12: f64 = ((s.db[114][12] * eq17_e2093) + (s.v[114] * eq17_e2093_d_b12));
        let eq17_e2094_d_b13: f64 = ((s.db[114][13] * eq17_e2093) + (s.v[114] * eq17_e2093_d_b13));
        let eq17_e2094_d_b14: f64 = ((s.db[114][14] * eq17_e2093) + (s.v[114] * eq17_e2093_d_b14));
        let eq17_e2094_d_b15: f64 = ((s.db[114][15] * eq17_e2093) + (s.v[114] * eq17_e2093_d_b15));
        let eq17_e2094_d_b16: f64 = ((s.db[114][16] * eq17_e2093) + (s.v[114] * eq17_e2093_d_b16));
        let eq17_e2094_d_b17: f64 = ((s.db[114][17] * eq17_e2093) + (s.v[114] * eq17_e2093_d_b17));
        (eq17_e2094, eq17_e2094_d_n0, eq17_e2094_d_n1, eq17_e2094_d_n2, eq17_e2094_d_n3, eq17_e2094_d_n4, eq17_e2094_d_n5, eq17_e2094_d_n6, eq17_e2094_d_n7, eq17_e2094_d_n8, eq17_e2094_d_n9, eq17_e2094_d_n10, eq17_e2094_d_n11, eq17_e2094_d_n12, eq17_e2094_d_n13, eq17_e2094_d_n14, eq17_e2094_d_n15, eq17_e2094_d_n16, eq17_e2094_d_b0, eq17_e2094_d_b1, eq17_e2094_d_b2, eq17_e2094_d_b3, eq17_e2094_d_b4, eq17_e2094_d_b5, eq17_e2094_d_b6, eq17_e2094_d_b7, eq17_e2094_d_b8, eq17_e2094_d_b9, eq17_e2094_d_b10, eq17_e2094_d_b11, eq17_e2094_d_b12, eq17_e2094_d_b13, eq17_e2094_d_b14, eq17_e2094_d_b15, eq17_e2094_d_b16, eq17_e2094_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e2096;
        let eq17_node_derivatives: [f64; 17] = [eq17_e2096_d_n0, eq17_e2096_d_n1, eq17_e2096_d_n2, eq17_e2096_d_n3, eq17_e2096_d_n4, eq17_e2096_d_n5, eq17_e2096_d_n6, eq17_e2096_d_n7, eq17_e2096_d_n8, eq17_e2096_d_n9, eq17_e2096_d_n10, eq17_e2096_d_n11, eq17_e2096_d_n12, eq17_e2096_d_n13, eq17_e2096_d_n14, eq17_e2096_d_n15, eq17_e2096_d_n16];
        let eq17_branch_derivatives: [f64; 18] = [eq17_e2096_d_b0, eq17_e2096_d_b1, eq17_e2096_d_b2, eq17_e2096_d_b3, eq17_e2096_d_b4, eq17_e2096_d_b5, eq17_e2096_d_b6, eq17_e2096_d_b7, eq17_e2096_d_b8, eq17_e2096_d_b9, eq17_e2096_d_b10, eq17_e2096_d_b11, eq17_e2096_d_b12, eq17_e2096_d_b13, eq17_e2096_d_b14, eq17_e2096_d_b15, eq17_e2096_d_b16, eq17_e2096_d_b17];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(3),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let (eq18_e2107, eq18_e2107_d_n0, eq18_e2107_d_n1, eq18_e2107_d_n2, eq18_e2107_d_n3, eq18_e2107_d_n4, eq18_e2107_d_n5, eq18_e2107_d_n6, eq18_e2107_d_n7, eq18_e2107_d_n8, eq18_e2107_d_n9, eq18_e2107_d_n10, eq18_e2107_d_n11, eq18_e2107_d_n12, eq18_e2107_d_n13, eq18_e2107_d_n14, eq18_e2107_d_n15, eq18_e2107_d_n16, eq18_e2107_d_b0, eq18_e2107_d_b1, eq18_e2107_d_b2, eq18_e2107_d_b3, eq18_e2107_d_b4, eq18_e2107_d_b5, eq18_e2107_d_b6, eq18_e2107_d_b7, eq18_e2107_d_b8, eq18_e2107_d_b9, eq18_e2107_d_b10, eq18_e2107_d_b11, eq18_e2107_d_b12, eq18_e2107_d_b13, eq18_e2107_d_b14, eq18_e2107_d_b15, eq18_e2107_d_b16, eq18_e2107_d_b17,) = {
    if (s.b[1698] && (!s.b[1699])) {
        let eq18_e2104: f64 = (s.v[476] + s.v[488]);
        let eq18_e2104_d_n0: f64 = (s.dn[476][0] + s.dn[488][0]);
        let eq18_e2104_d_n1: f64 = (s.dn[476][1] + s.dn[488][1]);
        let eq18_e2104_d_n2: f64 = (s.dn[476][2] + s.dn[488][2]);
        let eq18_e2104_d_n3: f64 = (s.dn[476][3] + s.dn[488][3]);
        let eq18_e2104_d_n4: f64 = (s.dn[476][4] + s.dn[488][4]);
        let eq18_e2104_d_n5: f64 = (s.dn[476][5] + s.dn[488][5]);
        let eq18_e2104_d_n6: f64 = (s.dn[476][6] + s.dn[488][6]);
        let eq18_e2104_d_n7: f64 = (s.dn[476][7] + s.dn[488][7]);
        let eq18_e2104_d_n8: f64 = (s.dn[476][8] + s.dn[488][8]);
        let eq18_e2104_d_n9: f64 = (s.dn[476][9] + s.dn[488][9]);
        let eq18_e2104_d_n10: f64 = (s.dn[476][10] + s.dn[488][10]);
        let eq18_e2104_d_n11: f64 = (s.dn[476][11] + s.dn[488][11]);
        let eq18_e2104_d_n12: f64 = (s.dn[476][12] + s.dn[488][12]);
        let eq18_e2104_d_n13: f64 = (s.dn[476][13] + s.dn[488][13]);
        let eq18_e2104_d_n14: f64 = (s.dn[476][14] + s.dn[488][14]);
        let eq18_e2104_d_n15: f64 = (s.dn[476][15] + s.dn[488][15]);
        let eq18_e2104_d_n16: f64 = (s.dn[476][16] + s.dn[488][16]);
        let eq18_e2104_d_b0: f64 = (s.db[476][0] + s.db[488][0]);
        let eq18_e2104_d_b1: f64 = (s.db[476][1] + s.db[488][1]);
        let eq18_e2104_d_b2: f64 = (s.db[476][2] + s.db[488][2]);
        let eq18_e2104_d_b3: f64 = (s.db[476][3] + s.db[488][3]);
        let eq18_e2104_d_b4: f64 = (s.db[476][4] + s.db[488][4]);
        let eq18_e2104_d_b5: f64 = (s.db[476][5] + s.db[488][5]);
        let eq18_e2104_d_b6: f64 = (s.db[476][6] + s.db[488][6]);
        let eq18_e2104_d_b7: f64 = (s.db[476][7] + s.db[488][7]);
        let eq18_e2104_d_b8: f64 = (s.db[476][8] + s.db[488][8]);
        let eq18_e2104_d_b9: f64 = (s.db[476][9] + s.db[488][9]);
        let eq18_e2104_d_b10: f64 = (s.db[476][10] + s.db[488][10]);
        let eq18_e2104_d_b11: f64 = (s.db[476][11] + s.db[488][11]);
        let eq18_e2104_d_b12: f64 = (s.db[476][12] + s.db[488][12]);
        let eq18_e2104_d_b13: f64 = (s.db[476][13] + s.db[488][13]);
        let eq18_e2104_d_b14: f64 = (s.db[476][14] + s.db[488][14]);
        let eq18_e2104_d_b15: f64 = (s.db[476][15] + s.db[488][15]);
        let eq18_e2104_d_b16: f64 = (s.db[476][16] + s.db[488][16]);
        let eq18_e2104_d_b17: f64 = (s.db[476][17] + s.db[488][17]);
        let eq18_e2105: f64 = (s.v[114] * eq18_e2104);
        let eq18_e2105_d_n0: f64 = ((s.dn[114][0] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n0));
        let eq18_e2105_d_n1: f64 = ((s.dn[114][1] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n1));
        let eq18_e2105_d_n2: f64 = ((s.dn[114][2] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n2));
        let eq18_e2105_d_n3: f64 = ((s.dn[114][3] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n3));
        let eq18_e2105_d_n4: f64 = ((s.dn[114][4] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n4));
        let eq18_e2105_d_n5: f64 = ((s.dn[114][5] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n5));
        let eq18_e2105_d_n6: f64 = ((s.dn[114][6] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n6));
        let eq18_e2105_d_n7: f64 = ((s.dn[114][7] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n7));
        let eq18_e2105_d_n8: f64 = ((s.dn[114][8] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n8));
        let eq18_e2105_d_n9: f64 = ((s.dn[114][9] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n9));
        let eq18_e2105_d_n10: f64 = ((s.dn[114][10] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n10));
        let eq18_e2105_d_n11: f64 = ((s.dn[114][11] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n11));
        let eq18_e2105_d_n12: f64 = ((s.dn[114][12] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n12));
        let eq18_e2105_d_n13: f64 = ((s.dn[114][13] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n13));
        let eq18_e2105_d_n14: f64 = ((s.dn[114][14] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n14));
        let eq18_e2105_d_n15: f64 = ((s.dn[114][15] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n15));
        let eq18_e2105_d_n16: f64 = ((s.dn[114][16] * eq18_e2104) + (s.v[114] * eq18_e2104_d_n16));
        let eq18_e2105_d_b0: f64 = ((s.db[114][0] * eq18_e2104) + (s.v[114] * eq18_e2104_d_b0));
        let eq18_e2105_d_b1: f64 = ((s.db[114][1] * eq18_e2104) + (s.v[114] * eq18_e2104_d_b1));
        let eq18_e2105_d_b2: f64 = ((s.db[114][2] * eq18_e2104) + (s.v[114] * eq18_e2104_d_b2));
        let eq18_e2105_d_b3: f64 = ((s.db[114][3] * eq18_e2104) + (s.v[114] * eq18_e2104_d_b3));
        let eq18_e2105_d_b4: f64 = ((s.db[114][4] * eq18_e2104) + (s.v[114] * eq18_e2104_d_b4));
        let eq18_e2105_d_b5: f64 = ((s.db[114][5] * eq18_e2104) + (s.v[114] * eq18_e2104_d_b5));
        let eq18_e2105_d_b6: f64 = ((s.db[114][6] * eq18_e2104) + (s.v[114] * eq18_e2104_d_b6));
        let eq18_e2105_d_b7: f64 = ((s.db[114][7] * eq18_e2104) + (s.v[114] * eq18_e2104_d_b7));
        let eq18_e2105_d_b8: f64 = ((s.db[114][8] * eq18_e2104) + (s.v[114] * eq18_e2104_d_b8));
        let eq18_e2105_d_b9: f64 = ((s.db[114][9] * eq18_e2104) + (s.v[114] * eq18_e2104_d_b9));
        let eq18_e2105_d_b10: f64 = ((s.db[114][10] * eq18_e2104) + (s.v[114] * eq18_e2104_d_b10));
        let eq18_e2105_d_b11: f64 = ((s.db[114][11] * eq18_e2104) + (s.v[114] * eq18_e2104_d_b11));
        let eq18_e2105_d_b12: f64 = ((s.db[114][12] * eq18_e2104) + (s.v[114] * eq18_e2104_d_b12));
        let eq18_e2105_d_b13: f64 = ((s.db[114][13] * eq18_e2104) + (s.v[114] * eq18_e2104_d_b13));
        let eq18_e2105_d_b14: f64 = ((s.db[114][14] * eq18_e2104) + (s.v[114] * eq18_e2104_d_b14));
        let eq18_e2105_d_b15: f64 = ((s.db[114][15] * eq18_e2104) + (s.v[114] * eq18_e2104_d_b15));
        let eq18_e2105_d_b16: f64 = ((s.db[114][16] * eq18_e2104) + (s.v[114] * eq18_e2104_d_b16));
        let eq18_e2105_d_b17: f64 = ((s.db[114][17] * eq18_e2104) + (s.v[114] * eq18_e2104_d_b17));
        (eq18_e2105, eq18_e2105_d_n0, eq18_e2105_d_n1, eq18_e2105_d_n2, eq18_e2105_d_n3, eq18_e2105_d_n4, eq18_e2105_d_n5, eq18_e2105_d_n6, eq18_e2105_d_n7, eq18_e2105_d_n8, eq18_e2105_d_n9, eq18_e2105_d_n10, eq18_e2105_d_n11, eq18_e2105_d_n12, eq18_e2105_d_n13, eq18_e2105_d_n14, eq18_e2105_d_n15, eq18_e2105_d_n16, eq18_e2105_d_b0, eq18_e2105_d_b1, eq18_e2105_d_b2, eq18_e2105_d_b3, eq18_e2105_d_b4, eq18_e2105_d_b5, eq18_e2105_d_b6, eq18_e2105_d_b7, eq18_e2105_d_b8, eq18_e2105_d_b9, eq18_e2105_d_b10, eq18_e2105_d_b11, eq18_e2105_d_b12, eq18_e2105_d_b13, eq18_e2105_d_b14, eq18_e2105_d_b15, eq18_e2105_d_b16, eq18_e2105_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e2107;
        let eq18_node_derivatives: [f64; 17] = [eq18_e2107_d_n0, eq18_e2107_d_n1, eq18_e2107_d_n2, eq18_e2107_d_n3, eq18_e2107_d_n4, eq18_e2107_d_n5, eq18_e2107_d_n6, eq18_e2107_d_n7, eq18_e2107_d_n8, eq18_e2107_d_n9, eq18_e2107_d_n10, eq18_e2107_d_n11, eq18_e2107_d_n12, eq18_e2107_d_n13, eq18_e2107_d_n14, eq18_e2107_d_n15, eq18_e2107_d_n16];
        let eq18_branch_derivatives: [f64; 18] = [eq18_e2107_d_b0, eq18_e2107_d_b1, eq18_e2107_d_b2, eq18_e2107_d_b3, eq18_e2107_d_b4, eq18_e2107_d_b5, eq18_e2107_d_b6, eq18_e2107_d_b7, eq18_e2107_d_b8, eq18_e2107_d_b9, eq18_e2107_d_b10, eq18_e2107_d_b11, eq18_e2107_d_b12, eq18_e2107_d_b13, eq18_e2107_d_b14, eq18_e2107_d_b15, eq18_e2107_d_b16, eq18_e2107_d_b17];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let (eq19_e2116, eq19_e2116_d_n0, eq19_e2116_d_n1, eq19_e2116_d_n2, eq19_e2116_d_n3, eq19_e2116_d_n4, eq19_e2116_d_n5, eq19_e2116_d_n6, eq19_e2116_d_n7, eq19_e2116_d_n8, eq19_e2116_d_n9, eq19_e2116_d_n10, eq19_e2116_d_n11, eq19_e2116_d_n12, eq19_e2116_d_n13, eq19_e2116_d_n14, eq19_e2116_d_n15, eq19_e2116_d_n16, eq19_e2116_d_b0, eq19_e2116_d_b1, eq19_e2116_d_b2, eq19_e2116_d_b3, eq19_e2116_d_b4, eq19_e2116_d_b5, eq19_e2116_d_b6, eq19_e2116_d_b7, eq19_e2116_d_b8, eq19_e2116_d_b9, eq19_e2116_d_b10, eq19_e2116_d_b11, eq19_e2116_d_b12, eq19_e2116_d_b13, eq19_e2116_d_b14, eq19_e2116_d_b15, eq19_e2116_d_b16, eq19_e2116_d_b17,) = {
    if (s.b[1698] && (!s.b[1699])) {
        let eq19_e2114: f64 = (s.v[114] * s.v[475]);
        let eq19_e2114_d_n0: f64 = ((s.dn[114][0] * s.v[475]) + (s.v[114] * s.dn[475][0]));
        let eq19_e2114_d_n1: f64 = ((s.dn[114][1] * s.v[475]) + (s.v[114] * s.dn[475][1]));
        let eq19_e2114_d_n2: f64 = ((s.dn[114][2] * s.v[475]) + (s.v[114] * s.dn[475][2]));
        let eq19_e2114_d_n3: f64 = ((s.dn[114][3] * s.v[475]) + (s.v[114] * s.dn[475][3]));
        let eq19_e2114_d_n4: f64 = ((s.dn[114][4] * s.v[475]) + (s.v[114] * s.dn[475][4]));
        let eq19_e2114_d_n5: f64 = ((s.dn[114][5] * s.v[475]) + (s.v[114] * s.dn[475][5]));
        let eq19_e2114_d_n6: f64 = ((s.dn[114][6] * s.v[475]) + (s.v[114] * s.dn[475][6]));
        let eq19_e2114_d_n7: f64 = ((s.dn[114][7] * s.v[475]) + (s.v[114] * s.dn[475][7]));
        let eq19_e2114_d_n8: f64 = ((s.dn[114][8] * s.v[475]) + (s.v[114] * s.dn[475][8]));
        let eq19_e2114_d_n9: f64 = ((s.dn[114][9] * s.v[475]) + (s.v[114] * s.dn[475][9]));
        let eq19_e2114_d_n10: f64 = ((s.dn[114][10] * s.v[475]) + (s.v[114] * s.dn[475][10]));
        let eq19_e2114_d_n11: f64 = ((s.dn[114][11] * s.v[475]) + (s.v[114] * s.dn[475][11]));
        let eq19_e2114_d_n12: f64 = ((s.dn[114][12] * s.v[475]) + (s.v[114] * s.dn[475][12]));
        let eq19_e2114_d_n13: f64 = ((s.dn[114][13] * s.v[475]) + (s.v[114] * s.dn[475][13]));
        let eq19_e2114_d_n14: f64 = ((s.dn[114][14] * s.v[475]) + (s.v[114] * s.dn[475][14]));
        let eq19_e2114_d_n15: f64 = ((s.dn[114][15] * s.v[475]) + (s.v[114] * s.dn[475][15]));
        let eq19_e2114_d_n16: f64 = ((s.dn[114][16] * s.v[475]) + (s.v[114] * s.dn[475][16]));
        let eq19_e2114_d_b0: f64 = ((s.db[114][0] * s.v[475]) + (s.v[114] * s.db[475][0]));
        let eq19_e2114_d_b1: f64 = ((s.db[114][1] * s.v[475]) + (s.v[114] * s.db[475][1]));
        let eq19_e2114_d_b2: f64 = ((s.db[114][2] * s.v[475]) + (s.v[114] * s.db[475][2]));
        let eq19_e2114_d_b3: f64 = ((s.db[114][3] * s.v[475]) + (s.v[114] * s.db[475][3]));
        let eq19_e2114_d_b4: f64 = ((s.db[114][4] * s.v[475]) + (s.v[114] * s.db[475][4]));
        let eq19_e2114_d_b5: f64 = ((s.db[114][5] * s.v[475]) + (s.v[114] * s.db[475][5]));
        let eq19_e2114_d_b6: f64 = ((s.db[114][6] * s.v[475]) + (s.v[114] * s.db[475][6]));
        let eq19_e2114_d_b7: f64 = ((s.db[114][7] * s.v[475]) + (s.v[114] * s.db[475][7]));
        let eq19_e2114_d_b8: f64 = ((s.db[114][8] * s.v[475]) + (s.v[114] * s.db[475][8]));
        let eq19_e2114_d_b9: f64 = ((s.db[114][9] * s.v[475]) + (s.v[114] * s.db[475][9]));
        let eq19_e2114_d_b10: f64 = ((s.db[114][10] * s.v[475]) + (s.v[114] * s.db[475][10]));
        let eq19_e2114_d_b11: f64 = ((s.db[114][11] * s.v[475]) + (s.v[114] * s.db[475][11]));
        let eq19_e2114_d_b12: f64 = ((s.db[114][12] * s.v[475]) + (s.v[114] * s.db[475][12]));
        let eq19_e2114_d_b13: f64 = ((s.db[114][13] * s.v[475]) + (s.v[114] * s.db[475][13]));
        let eq19_e2114_d_b14: f64 = ((s.db[114][14] * s.v[475]) + (s.v[114] * s.db[475][14]));
        let eq19_e2114_d_b15: f64 = ((s.db[114][15] * s.v[475]) + (s.v[114] * s.db[475][15]));
        let eq19_e2114_d_b16: f64 = ((s.db[114][16] * s.v[475]) + (s.v[114] * s.db[475][16]));
        let eq19_e2114_d_b17: f64 = ((s.db[114][17] * s.v[475]) + (s.v[114] * s.db[475][17]));
        (eq19_e2114, eq19_e2114_d_n0, eq19_e2114_d_n1, eq19_e2114_d_n2, eq19_e2114_d_n3, eq19_e2114_d_n4, eq19_e2114_d_n5, eq19_e2114_d_n6, eq19_e2114_d_n7, eq19_e2114_d_n8, eq19_e2114_d_n9, eq19_e2114_d_n10, eq19_e2114_d_n11, eq19_e2114_d_n12, eq19_e2114_d_n13, eq19_e2114_d_n14, eq19_e2114_d_n15, eq19_e2114_d_n16, eq19_e2114_d_b0, eq19_e2114_d_b1, eq19_e2114_d_b2, eq19_e2114_d_b3, eq19_e2114_d_b4, eq19_e2114_d_b5, eq19_e2114_d_b6, eq19_e2114_d_b7, eq19_e2114_d_b8, eq19_e2114_d_b9, eq19_e2114_d_b10, eq19_e2114_d_b11, eq19_e2114_d_b12, eq19_e2114_d_b13, eq19_e2114_d_b14, eq19_e2114_d_b15, eq19_e2114_d_b16, eq19_e2114_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e2116;
        let eq19_node_derivatives: [f64; 17] = [eq19_e2116_d_n0, eq19_e2116_d_n1, eq19_e2116_d_n2, eq19_e2116_d_n3, eq19_e2116_d_n4, eq19_e2116_d_n5, eq19_e2116_d_n6, eq19_e2116_d_n7, eq19_e2116_d_n8, eq19_e2116_d_n9, eq19_e2116_d_n10, eq19_e2116_d_n11, eq19_e2116_d_n12, eq19_e2116_d_n13, eq19_e2116_d_n14, eq19_e2116_d_n15, eq19_e2116_d_n16];
        let eq19_branch_derivatives: [f64; 18] = [eq19_e2116_d_b0, eq19_e2116_d_b1, eq19_e2116_d_b2, eq19_e2116_d_b3, eq19_e2116_d_b4, eq19_e2116_d_b5, eq19_e2116_d_b6, eq19_e2116_d_b7, eq19_e2116_d_b8, eq19_e2116_d_b9, eq19_e2116_d_b10, eq19_e2116_d_b11, eq19_e2116_d_b12, eq19_e2116_d_b13, eq19_e2116_d_b14, eq19_e2116_d_b15, eq19_e2116_d_b16, eq19_e2116_d_b17];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let (eq20_e2123, eq20_e2123_d_n0, eq20_e2123_d_n1, eq20_e2123_d_n2, eq20_e2123_d_n3, eq20_e2123_d_n4, eq20_e2123_d_n5, eq20_e2123_d_n6, eq20_e2123_d_n7, eq20_e2123_d_n8, eq20_e2123_d_n9, eq20_e2123_d_n10, eq20_e2123_d_n11, eq20_e2123_d_n12, eq20_e2123_d_n13, eq20_e2123_d_n14, eq20_e2123_d_n15, eq20_e2123_d_n16, eq20_e2123_d_b0, eq20_e2123_d_b1, eq20_e2123_d_b2, eq20_e2123_d_b3, eq20_e2123_d_b4, eq20_e2123_d_b5, eq20_e2123_d_b6, eq20_e2123_d_b7, eq20_e2123_d_b8, eq20_e2123_d_b9, eq20_e2123_d_b10, eq20_e2123_d_b11, eq20_e2123_d_b12, eq20_e2123_d_b13, eq20_e2123_d_b14, eq20_e2123_d_b15, eq20_e2123_d_b16, eq20_e2123_d_b17,) = {
    if (!s.b[1698]) {
        let eq20_e2121: f64 = (s.v[114] * s.v[556]);
        let eq20_e2121_d_n0: f64 = ((s.dn[114][0] * s.v[556]) + (s.v[114] * s.dn[556][0]));
        let eq20_e2121_d_n1: f64 = ((s.dn[114][1] * s.v[556]) + (s.v[114] * s.dn[556][1]));
        let eq20_e2121_d_n2: f64 = ((s.dn[114][2] * s.v[556]) + (s.v[114] * s.dn[556][2]));
        let eq20_e2121_d_n3: f64 = ((s.dn[114][3] * s.v[556]) + (s.v[114] * s.dn[556][3]));
        let eq20_e2121_d_n4: f64 = ((s.dn[114][4] * s.v[556]) + (s.v[114] * s.dn[556][4]));
        let eq20_e2121_d_n5: f64 = ((s.dn[114][5] * s.v[556]) + (s.v[114] * s.dn[556][5]));
        let eq20_e2121_d_n6: f64 = ((s.dn[114][6] * s.v[556]) + (s.v[114] * s.dn[556][6]));
        let eq20_e2121_d_n7: f64 = ((s.dn[114][7] * s.v[556]) + (s.v[114] * s.dn[556][7]));
        let eq20_e2121_d_n8: f64 = ((s.dn[114][8] * s.v[556]) + (s.v[114] * s.dn[556][8]));
        let eq20_e2121_d_n9: f64 = ((s.dn[114][9] * s.v[556]) + (s.v[114] * s.dn[556][9]));
        let eq20_e2121_d_n10: f64 = ((s.dn[114][10] * s.v[556]) + (s.v[114] * s.dn[556][10]));
        let eq20_e2121_d_n11: f64 = ((s.dn[114][11] * s.v[556]) + (s.v[114] * s.dn[556][11]));
        let eq20_e2121_d_n12: f64 = ((s.dn[114][12] * s.v[556]) + (s.v[114] * s.dn[556][12]));
        let eq20_e2121_d_n13: f64 = ((s.dn[114][13] * s.v[556]) + (s.v[114] * s.dn[556][13]));
        let eq20_e2121_d_n14: f64 = ((s.dn[114][14] * s.v[556]) + (s.v[114] * s.dn[556][14]));
        let eq20_e2121_d_n15: f64 = ((s.dn[114][15] * s.v[556]) + (s.v[114] * s.dn[556][15]));
        let eq20_e2121_d_n16: f64 = ((s.dn[114][16] * s.v[556]) + (s.v[114] * s.dn[556][16]));
        let eq20_e2121_d_b0: f64 = ((s.db[114][0] * s.v[556]) + (s.v[114] * s.db[556][0]));
        let eq20_e2121_d_b1: f64 = ((s.db[114][1] * s.v[556]) + (s.v[114] * s.db[556][1]));
        let eq20_e2121_d_b2: f64 = ((s.db[114][2] * s.v[556]) + (s.v[114] * s.db[556][2]));
        let eq20_e2121_d_b3: f64 = ((s.db[114][3] * s.v[556]) + (s.v[114] * s.db[556][3]));
        let eq20_e2121_d_b4: f64 = ((s.db[114][4] * s.v[556]) + (s.v[114] * s.db[556][4]));
        let eq20_e2121_d_b5: f64 = ((s.db[114][5] * s.v[556]) + (s.v[114] * s.db[556][5]));
        let eq20_e2121_d_b6: f64 = ((s.db[114][6] * s.v[556]) + (s.v[114] * s.db[556][6]));
        let eq20_e2121_d_b7: f64 = ((s.db[114][7] * s.v[556]) + (s.v[114] * s.db[556][7]));
        let eq20_e2121_d_b8: f64 = ((s.db[114][8] * s.v[556]) + (s.v[114] * s.db[556][8]));
        let eq20_e2121_d_b9: f64 = ((s.db[114][9] * s.v[556]) + (s.v[114] * s.db[556][9]));
        let eq20_e2121_d_b10: f64 = ((s.db[114][10] * s.v[556]) + (s.v[114] * s.db[556][10]));
        let eq20_e2121_d_b11: f64 = ((s.db[114][11] * s.v[556]) + (s.v[114] * s.db[556][11]));
        let eq20_e2121_d_b12: f64 = ((s.db[114][12] * s.v[556]) + (s.v[114] * s.db[556][12]));
        let eq20_e2121_d_b13: f64 = ((s.db[114][13] * s.v[556]) + (s.v[114] * s.db[556][13]));
        let eq20_e2121_d_b14: f64 = ((s.db[114][14] * s.v[556]) + (s.v[114] * s.db[556][14]));
        let eq20_e2121_d_b15: f64 = ((s.db[114][15] * s.v[556]) + (s.v[114] * s.db[556][15]));
        let eq20_e2121_d_b16: f64 = ((s.db[114][16] * s.v[556]) + (s.v[114] * s.db[556][16]));
        let eq20_e2121_d_b17: f64 = ((s.db[114][17] * s.v[556]) + (s.v[114] * s.db[556][17]));
        (eq20_e2121, eq20_e2121_d_n0, eq20_e2121_d_n1, eq20_e2121_d_n2, eq20_e2121_d_n3, eq20_e2121_d_n4, eq20_e2121_d_n5, eq20_e2121_d_n6, eq20_e2121_d_n7, eq20_e2121_d_n8, eq20_e2121_d_n9, eq20_e2121_d_n10, eq20_e2121_d_n11, eq20_e2121_d_n12, eq20_e2121_d_n13, eq20_e2121_d_n14, eq20_e2121_d_n15, eq20_e2121_d_n16, eq20_e2121_d_b0, eq20_e2121_d_b1, eq20_e2121_d_b2, eq20_e2121_d_b3, eq20_e2121_d_b4, eq20_e2121_d_b5, eq20_e2121_d_b6, eq20_e2121_d_b7, eq20_e2121_d_b8, eq20_e2121_d_b9, eq20_e2121_d_b10, eq20_e2121_d_b11, eq20_e2121_d_b12, eq20_e2121_d_b13, eq20_e2121_d_b14, eq20_e2121_d_b15, eq20_e2121_d_b16, eq20_e2121_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e2123;
        let eq20_node_derivatives: [f64; 17] = [eq20_e2123_d_n0, eq20_e2123_d_n1, eq20_e2123_d_n2, eq20_e2123_d_n3, eq20_e2123_d_n4, eq20_e2123_d_n5, eq20_e2123_d_n6, eq20_e2123_d_n7, eq20_e2123_d_n8, eq20_e2123_d_n9, eq20_e2123_d_n10, eq20_e2123_d_n11, eq20_e2123_d_n12, eq20_e2123_d_n13, eq20_e2123_d_n14, eq20_e2123_d_n15, eq20_e2123_d_n16];
        let eq20_branch_derivatives: [f64; 18] = [eq20_e2123_d_b0, eq20_e2123_d_b1, eq20_e2123_d_b2, eq20_e2123_d_b3, eq20_e2123_d_b4, eq20_e2123_d_b5, eq20_e2123_d_b6, eq20_e2123_d_b7, eq20_e2123_d_b8, eq20_e2123_d_b9, eq20_e2123_d_b10, eq20_e2123_d_b11, eq20_e2123_d_b12, eq20_e2123_d_b13, eq20_e2123_d_b14, eq20_e2123_d_b15, eq20_e2123_d_b16, eq20_e2123_d_b17];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
    ) {
        let (eq21_e2132, eq21_e2132_d_n0, eq21_e2132_d_n1, eq21_e2132_d_n2, eq21_e2132_d_n3, eq21_e2132_d_n4, eq21_e2132_d_n5, eq21_e2132_d_n6, eq21_e2132_d_n7, eq21_e2132_d_n8, eq21_e2132_d_n9, eq21_e2132_d_n10, eq21_e2132_d_n11, eq21_e2132_d_n12, eq21_e2132_d_n13, eq21_e2132_d_n14, eq21_e2132_d_n15, eq21_e2132_d_n16, eq21_e2132_d_b0, eq21_e2132_d_b1, eq21_e2132_d_b2, eq21_e2132_d_b3, eq21_e2132_d_b4, eq21_e2132_d_b5, eq21_e2132_d_b6, eq21_e2132_d_b7, eq21_e2132_d_b8, eq21_e2132_d_b9, eq21_e2132_d_b10, eq21_e2132_d_b11, eq21_e2132_d_b12, eq21_e2132_d_b13, eq21_e2132_d_b14, eq21_e2132_d_b15, eq21_e2132_d_b16, eq21_e2132_d_b17,) = {
    if (!s.b[1698]) {
        let eq21_e2129: f64 = (s.v[470] + s.v[480]);
        let eq21_e2129_d_n0: f64 = (s.dn[470][0] + s.dn[480][0]);
        let eq21_e2129_d_n1: f64 = (s.dn[470][1] + s.dn[480][1]);
        let eq21_e2129_d_n2: f64 = (s.dn[470][2] + s.dn[480][2]);
        let eq21_e2129_d_n3: f64 = (s.dn[470][3] + s.dn[480][3]);
        let eq21_e2129_d_n4: f64 = (s.dn[470][4] + s.dn[480][4]);
        let eq21_e2129_d_n5: f64 = (s.dn[470][5] + s.dn[480][5]);
        let eq21_e2129_d_n6: f64 = (s.dn[470][6] + s.dn[480][6]);
        let eq21_e2129_d_n7: f64 = (s.dn[470][7] + s.dn[480][7]);
        let eq21_e2129_d_n8: f64 = (s.dn[470][8] + s.dn[480][8]);
        let eq21_e2129_d_n9: f64 = (s.dn[470][9] + s.dn[480][9]);
        let eq21_e2129_d_n10: f64 = (s.dn[470][10] + s.dn[480][10]);
        let eq21_e2129_d_n11: f64 = (s.dn[470][11] + s.dn[480][11]);
        let eq21_e2129_d_n12: f64 = (s.dn[470][12] + s.dn[480][12]);
        let eq21_e2129_d_n13: f64 = (s.dn[470][13] + s.dn[480][13]);
        let eq21_e2129_d_n14: f64 = (s.dn[470][14] + s.dn[480][14]);
        let eq21_e2129_d_n15: f64 = (s.dn[470][15] + s.dn[480][15]);
        let eq21_e2129_d_n16: f64 = (s.dn[470][16] + s.dn[480][16]);
        let eq21_e2129_d_b0: f64 = (s.db[470][0] + s.db[480][0]);
        let eq21_e2129_d_b1: f64 = (s.db[470][1] + s.db[480][1]);
        let eq21_e2129_d_b2: f64 = (s.db[470][2] + s.db[480][2]);
        let eq21_e2129_d_b3: f64 = (s.db[470][3] + s.db[480][3]);
        let eq21_e2129_d_b4: f64 = (s.db[470][4] + s.db[480][4]);
        let eq21_e2129_d_b5: f64 = (s.db[470][5] + s.db[480][5]);
        let eq21_e2129_d_b6: f64 = (s.db[470][6] + s.db[480][6]);
        let eq21_e2129_d_b7: f64 = (s.db[470][7] + s.db[480][7]);
        let eq21_e2129_d_b8: f64 = (s.db[470][8] + s.db[480][8]);
        let eq21_e2129_d_b9: f64 = (s.db[470][9] + s.db[480][9]);
        let eq21_e2129_d_b10: f64 = (s.db[470][10] + s.db[480][10]);
        let eq21_e2129_d_b11: f64 = (s.db[470][11] + s.db[480][11]);
        let eq21_e2129_d_b12: f64 = (s.db[470][12] + s.db[480][12]);
        let eq21_e2129_d_b13: f64 = (s.db[470][13] + s.db[480][13]);
        let eq21_e2129_d_b14: f64 = (s.db[470][14] + s.db[480][14]);
        let eq21_e2129_d_b15: f64 = (s.db[470][15] + s.db[480][15]);
        let eq21_e2129_d_b16: f64 = (s.db[470][16] + s.db[480][16]);
        let eq21_e2129_d_b17: f64 = (s.db[470][17] + s.db[480][17]);
        let eq21_e2130: f64 = (s.v[114] * eq21_e2129);
        let eq21_e2130_d_n0: f64 = ((s.dn[114][0] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n0));
        let eq21_e2130_d_n1: f64 = ((s.dn[114][1] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n1));
        let eq21_e2130_d_n2: f64 = ((s.dn[114][2] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n2));
        let eq21_e2130_d_n3: f64 = ((s.dn[114][3] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n3));
        let eq21_e2130_d_n4: f64 = ((s.dn[114][4] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n4));
        let eq21_e2130_d_n5: f64 = ((s.dn[114][5] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n5));
        let eq21_e2130_d_n6: f64 = ((s.dn[114][6] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n6));
        let eq21_e2130_d_n7: f64 = ((s.dn[114][7] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n7));
        let eq21_e2130_d_n8: f64 = ((s.dn[114][8] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n8));
        let eq21_e2130_d_n9: f64 = ((s.dn[114][9] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n9));
        let eq21_e2130_d_n10: f64 = ((s.dn[114][10] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n10));
        let eq21_e2130_d_n11: f64 = ((s.dn[114][11] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n11));
        let eq21_e2130_d_n12: f64 = ((s.dn[114][12] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n12));
        let eq21_e2130_d_n13: f64 = ((s.dn[114][13] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n13));
        let eq21_e2130_d_n14: f64 = ((s.dn[114][14] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n14));
        let eq21_e2130_d_n15: f64 = ((s.dn[114][15] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n15));
        let eq21_e2130_d_n16: f64 = ((s.dn[114][16] * eq21_e2129) + (s.v[114] * eq21_e2129_d_n16));
        let eq21_e2130_d_b0: f64 = ((s.db[114][0] * eq21_e2129) + (s.v[114] * eq21_e2129_d_b0));
        let eq21_e2130_d_b1: f64 = ((s.db[114][1] * eq21_e2129) + (s.v[114] * eq21_e2129_d_b1));
        let eq21_e2130_d_b2: f64 = ((s.db[114][2] * eq21_e2129) + (s.v[114] * eq21_e2129_d_b2));
        let eq21_e2130_d_b3: f64 = ((s.db[114][3] * eq21_e2129) + (s.v[114] * eq21_e2129_d_b3));
        let eq21_e2130_d_b4: f64 = ((s.db[114][4] * eq21_e2129) + (s.v[114] * eq21_e2129_d_b4));
        let eq21_e2130_d_b5: f64 = ((s.db[114][5] * eq21_e2129) + (s.v[114] * eq21_e2129_d_b5));
        let eq21_e2130_d_b6: f64 = ((s.db[114][6] * eq21_e2129) + (s.v[114] * eq21_e2129_d_b6));
        let eq21_e2130_d_b7: f64 = ((s.db[114][7] * eq21_e2129) + (s.v[114] * eq21_e2129_d_b7));
        let eq21_e2130_d_b8: f64 = ((s.db[114][8] * eq21_e2129) + (s.v[114] * eq21_e2129_d_b8));
        let eq21_e2130_d_b9: f64 = ((s.db[114][9] * eq21_e2129) + (s.v[114] * eq21_e2129_d_b9));
        let eq21_e2130_d_b10: f64 = ((s.db[114][10] * eq21_e2129) + (s.v[114] * eq21_e2129_d_b10));
        let eq21_e2130_d_b11: f64 = ((s.db[114][11] * eq21_e2129) + (s.v[114] * eq21_e2129_d_b11));
        let eq21_e2130_d_b12: f64 = ((s.db[114][12] * eq21_e2129) + (s.v[114] * eq21_e2129_d_b12));
        let eq21_e2130_d_b13: f64 = ((s.db[114][13] * eq21_e2129) + (s.v[114] * eq21_e2129_d_b13));
        let eq21_e2130_d_b14: f64 = ((s.db[114][14] * eq21_e2129) + (s.v[114] * eq21_e2129_d_b14));
        let eq21_e2130_d_b15: f64 = ((s.db[114][15] * eq21_e2129) + (s.v[114] * eq21_e2129_d_b15));
        let eq21_e2130_d_b16: f64 = ((s.db[114][16] * eq21_e2129) + (s.v[114] * eq21_e2129_d_b16));
        let eq21_e2130_d_b17: f64 = ((s.db[114][17] * eq21_e2129) + (s.v[114] * eq21_e2129_d_b17));
        (eq21_e2130, eq21_e2130_d_n0, eq21_e2130_d_n1, eq21_e2130_d_n2, eq21_e2130_d_n3, eq21_e2130_d_n4, eq21_e2130_d_n5, eq21_e2130_d_n6, eq21_e2130_d_n7, eq21_e2130_d_n8, eq21_e2130_d_n9, eq21_e2130_d_n10, eq21_e2130_d_n11, eq21_e2130_d_n12, eq21_e2130_d_n13, eq21_e2130_d_n14, eq21_e2130_d_n15, eq21_e2130_d_n16, eq21_e2130_d_b0, eq21_e2130_d_b1, eq21_e2130_d_b2, eq21_e2130_d_b3, eq21_e2130_d_b4, eq21_e2130_d_b5, eq21_e2130_d_b6, eq21_e2130_d_b7, eq21_e2130_d_b8, eq21_e2130_d_b9, eq21_e2130_d_b10, eq21_e2130_d_b11, eq21_e2130_d_b12, eq21_e2130_d_b13, eq21_e2130_d_b14, eq21_e2130_d_b15, eq21_e2130_d_b16, eq21_e2130_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e2132;
        let eq21_node_derivatives: [f64; 17] = [eq21_e2132_d_n0, eq21_e2132_d_n1, eq21_e2132_d_n2, eq21_e2132_d_n3, eq21_e2132_d_n4, eq21_e2132_d_n5, eq21_e2132_d_n6, eq21_e2132_d_n7, eq21_e2132_d_n8, eq21_e2132_d_n9, eq21_e2132_d_n10, eq21_e2132_d_n11, eq21_e2132_d_n12, eq21_e2132_d_n13, eq21_e2132_d_n14, eq21_e2132_d_n15, eq21_e2132_d_n16];
        let eq21_branch_derivatives: [f64; 18] = [eq21_e2132_d_b0, eq21_e2132_d_b1, eq21_e2132_d_b2, eq21_e2132_d_b3, eq21_e2132_d_b4, eq21_e2132_d_b5, eq21_e2132_d_b6, eq21_e2132_d_b7, eq21_e2132_d_b8, eq21_e2132_d_b9, eq21_e2132_d_b10, eq21_e2132_d_b11, eq21_e2132_d_b12, eq21_e2132_d_b13, eq21_e2132_d_b14, eq21_e2132_d_b15, eq21_e2132_d_b16, eq21_e2132_d_b17];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(5),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq22_e2141, eq22_e2141_d_n0, eq22_e2141_d_n1, eq22_e2141_d_n2, eq22_e2141_d_n3, eq22_e2141_d_n4, eq22_e2141_d_n5, eq22_e2141_d_n6, eq22_e2141_d_n7, eq22_e2141_d_n8, eq22_e2141_d_n9, eq22_e2141_d_n10, eq22_e2141_d_n11, eq22_e2141_d_n12, eq22_e2141_d_n13, eq22_e2141_d_n14, eq22_e2141_d_n15, eq22_e2141_d_n16, eq22_e2141_d_b0, eq22_e2141_d_b1, eq22_e2141_d_b2, eq22_e2141_d_b3, eq22_e2141_d_b4, eq22_e2141_d_b5, eq22_e2141_d_b6, eq22_e2141_d_b7, eq22_e2141_d_b8, eq22_e2141_d_b9, eq22_e2141_d_b10, eq22_e2141_d_b11, eq22_e2141_d_b12, eq22_e2141_d_b13, eq22_e2141_d_b14, eq22_e2141_d_b15, eq22_e2141_d_b16, eq22_e2141_d_b17,) = {
    if (!s.b[1698]) {
        let eq22_e2138: f64 = (s.v[471] + s.v[481]);
        let eq22_e2138_d_n0: f64 = (s.dn[471][0] + s.dn[481][0]);
        let eq22_e2138_d_n1: f64 = (s.dn[471][1] + s.dn[481][1]);
        let eq22_e2138_d_n2: f64 = (s.dn[471][2] + s.dn[481][2]);
        let eq22_e2138_d_n3: f64 = (s.dn[471][3] + s.dn[481][3]);
        let eq22_e2138_d_n4: f64 = (s.dn[471][4] + s.dn[481][4]);
        let eq22_e2138_d_n5: f64 = (s.dn[471][5] + s.dn[481][5]);
        let eq22_e2138_d_n6: f64 = (s.dn[471][6] + s.dn[481][6]);
        let eq22_e2138_d_n7: f64 = (s.dn[471][7] + s.dn[481][7]);
        let eq22_e2138_d_n8: f64 = (s.dn[471][8] + s.dn[481][8]);
        let eq22_e2138_d_n9: f64 = (s.dn[471][9] + s.dn[481][9]);
        let eq22_e2138_d_n10: f64 = (s.dn[471][10] + s.dn[481][10]);
        let eq22_e2138_d_n11: f64 = (s.dn[471][11] + s.dn[481][11]);
        let eq22_e2138_d_n12: f64 = (s.dn[471][12] + s.dn[481][12]);
        let eq22_e2138_d_n13: f64 = (s.dn[471][13] + s.dn[481][13]);
        let eq22_e2138_d_n14: f64 = (s.dn[471][14] + s.dn[481][14]);
        let eq22_e2138_d_n15: f64 = (s.dn[471][15] + s.dn[481][15]);
        let eq22_e2138_d_n16: f64 = (s.dn[471][16] + s.dn[481][16]);
        let eq22_e2138_d_b0: f64 = (s.db[471][0] + s.db[481][0]);
        let eq22_e2138_d_b1: f64 = (s.db[471][1] + s.db[481][1]);
        let eq22_e2138_d_b2: f64 = (s.db[471][2] + s.db[481][2]);
        let eq22_e2138_d_b3: f64 = (s.db[471][3] + s.db[481][3]);
        let eq22_e2138_d_b4: f64 = (s.db[471][4] + s.db[481][4]);
        let eq22_e2138_d_b5: f64 = (s.db[471][5] + s.db[481][5]);
        let eq22_e2138_d_b6: f64 = (s.db[471][6] + s.db[481][6]);
        let eq22_e2138_d_b7: f64 = (s.db[471][7] + s.db[481][7]);
        let eq22_e2138_d_b8: f64 = (s.db[471][8] + s.db[481][8]);
        let eq22_e2138_d_b9: f64 = (s.db[471][9] + s.db[481][9]);
        let eq22_e2138_d_b10: f64 = (s.db[471][10] + s.db[481][10]);
        let eq22_e2138_d_b11: f64 = (s.db[471][11] + s.db[481][11]);
        let eq22_e2138_d_b12: f64 = (s.db[471][12] + s.db[481][12]);
        let eq22_e2138_d_b13: f64 = (s.db[471][13] + s.db[481][13]);
        let eq22_e2138_d_b14: f64 = (s.db[471][14] + s.db[481][14]);
        let eq22_e2138_d_b15: f64 = (s.db[471][15] + s.db[481][15]);
        let eq22_e2138_d_b16: f64 = (s.db[471][16] + s.db[481][16]);
        let eq22_e2138_d_b17: f64 = (s.db[471][17] + s.db[481][17]);
        let eq22_e2139: f64 = (s.v[114] * eq22_e2138);
        let eq22_e2139_d_n0: f64 = ((s.dn[114][0] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n0));
        let eq22_e2139_d_n1: f64 = ((s.dn[114][1] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n1));
        let eq22_e2139_d_n2: f64 = ((s.dn[114][2] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n2));
        let eq22_e2139_d_n3: f64 = ((s.dn[114][3] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n3));
        let eq22_e2139_d_n4: f64 = ((s.dn[114][4] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n4));
        let eq22_e2139_d_n5: f64 = ((s.dn[114][5] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n5));
        let eq22_e2139_d_n6: f64 = ((s.dn[114][6] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n6));
        let eq22_e2139_d_n7: f64 = ((s.dn[114][7] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n7));
        let eq22_e2139_d_n8: f64 = ((s.dn[114][8] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n8));
        let eq22_e2139_d_n9: f64 = ((s.dn[114][9] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n9));
        let eq22_e2139_d_n10: f64 = ((s.dn[114][10] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n10));
        let eq22_e2139_d_n11: f64 = ((s.dn[114][11] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n11));
        let eq22_e2139_d_n12: f64 = ((s.dn[114][12] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n12));
        let eq22_e2139_d_n13: f64 = ((s.dn[114][13] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n13));
        let eq22_e2139_d_n14: f64 = ((s.dn[114][14] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n14));
        let eq22_e2139_d_n15: f64 = ((s.dn[114][15] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n15));
        let eq22_e2139_d_n16: f64 = ((s.dn[114][16] * eq22_e2138) + (s.v[114] * eq22_e2138_d_n16));
        let eq22_e2139_d_b0: f64 = ((s.db[114][0] * eq22_e2138) + (s.v[114] * eq22_e2138_d_b0));
        let eq22_e2139_d_b1: f64 = ((s.db[114][1] * eq22_e2138) + (s.v[114] * eq22_e2138_d_b1));
        let eq22_e2139_d_b2: f64 = ((s.db[114][2] * eq22_e2138) + (s.v[114] * eq22_e2138_d_b2));
        let eq22_e2139_d_b3: f64 = ((s.db[114][3] * eq22_e2138) + (s.v[114] * eq22_e2138_d_b3));
        let eq22_e2139_d_b4: f64 = ((s.db[114][4] * eq22_e2138) + (s.v[114] * eq22_e2138_d_b4));
        let eq22_e2139_d_b5: f64 = ((s.db[114][5] * eq22_e2138) + (s.v[114] * eq22_e2138_d_b5));
        let eq22_e2139_d_b6: f64 = ((s.db[114][6] * eq22_e2138) + (s.v[114] * eq22_e2138_d_b6));
        let eq22_e2139_d_b7: f64 = ((s.db[114][7] * eq22_e2138) + (s.v[114] * eq22_e2138_d_b7));
        let eq22_e2139_d_b8: f64 = ((s.db[114][8] * eq22_e2138) + (s.v[114] * eq22_e2138_d_b8));
        let eq22_e2139_d_b9: f64 = ((s.db[114][9] * eq22_e2138) + (s.v[114] * eq22_e2138_d_b9));
        let eq22_e2139_d_b10: f64 = ((s.db[114][10] * eq22_e2138) + (s.v[114] * eq22_e2138_d_b10));
        let eq22_e2139_d_b11: f64 = ((s.db[114][11] * eq22_e2138) + (s.v[114] * eq22_e2138_d_b11));
        let eq22_e2139_d_b12: f64 = ((s.db[114][12] * eq22_e2138) + (s.v[114] * eq22_e2138_d_b12));
        let eq22_e2139_d_b13: f64 = ((s.db[114][13] * eq22_e2138) + (s.v[114] * eq22_e2138_d_b13));
        let eq22_e2139_d_b14: f64 = ((s.db[114][14] * eq22_e2138) + (s.v[114] * eq22_e2138_d_b14));
        let eq22_e2139_d_b15: f64 = ((s.db[114][15] * eq22_e2138) + (s.v[114] * eq22_e2138_d_b15));
        let eq22_e2139_d_b16: f64 = ((s.db[114][16] * eq22_e2138) + (s.v[114] * eq22_e2138_d_b16));
        let eq22_e2139_d_b17: f64 = ((s.db[114][17] * eq22_e2138) + (s.v[114] * eq22_e2138_d_b17));
        (eq22_e2139, eq22_e2139_d_n0, eq22_e2139_d_n1, eq22_e2139_d_n2, eq22_e2139_d_n3, eq22_e2139_d_n4, eq22_e2139_d_n5, eq22_e2139_d_n6, eq22_e2139_d_n7, eq22_e2139_d_n8, eq22_e2139_d_n9, eq22_e2139_d_n10, eq22_e2139_d_n11, eq22_e2139_d_n12, eq22_e2139_d_n13, eq22_e2139_d_n14, eq22_e2139_d_n15, eq22_e2139_d_n16, eq22_e2139_d_b0, eq22_e2139_d_b1, eq22_e2139_d_b2, eq22_e2139_d_b3, eq22_e2139_d_b4, eq22_e2139_d_b5, eq22_e2139_d_b6, eq22_e2139_d_b7, eq22_e2139_d_b8, eq22_e2139_d_b9, eq22_e2139_d_b10, eq22_e2139_d_b11, eq22_e2139_d_b12, eq22_e2139_d_b13, eq22_e2139_d_b14, eq22_e2139_d_b15, eq22_e2139_d_b16, eq22_e2139_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e2141;
        let eq22_node_derivatives: [f64; 17] = [eq22_e2141_d_n0, eq22_e2141_d_n1, eq22_e2141_d_n2, eq22_e2141_d_n3, eq22_e2141_d_n4, eq22_e2141_d_n5, eq22_e2141_d_n6, eq22_e2141_d_n7, eq22_e2141_d_n8, eq22_e2141_d_n9, eq22_e2141_d_n10, eq22_e2141_d_n11, eq22_e2141_d_n12, eq22_e2141_d_n13, eq22_e2141_d_n14, eq22_e2141_d_n15, eq22_e2141_d_n16];
        let eq22_branch_derivatives: [f64; 18] = [eq22_e2141_d_b0, eq22_e2141_d_b1, eq22_e2141_d_b2, eq22_e2141_d_b3, eq22_e2141_d_b4, eq22_e2141_d_b5, eq22_e2141_d_b6, eq22_e2141_d_b7, eq22_e2141_d_b8, eq22_e2141_d_b9, eq22_e2141_d_b10, eq22_e2141_d_b11, eq22_e2141_d_b12, eq22_e2141_d_b13, eq22_e2141_d_b14, eq22_e2141_d_b15, eq22_e2141_d_b16, eq22_e2141_d_b17];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq22_value),
            &eq22_node_derivatives,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let (eq23_e2154, eq23_e2154_d_n0, eq23_e2154_d_n1, eq23_e2154_d_n2, eq23_e2154_d_n3, eq23_e2154_d_n4, eq23_e2154_d_n5, eq23_e2154_d_n6, eq23_e2154_d_n7, eq23_e2154_d_n8, eq23_e2154_d_n9, eq23_e2154_d_n10, eq23_e2154_d_n11, eq23_e2154_d_n12, eq23_e2154_d_n13, eq23_e2154_d_n14, eq23_e2154_d_n15, eq23_e2154_d_n16, eq23_e2154_d_b0, eq23_e2154_d_b1, eq23_e2154_d_b2, eq23_e2154_d_b3, eq23_e2154_d_b4, eq23_e2154_d_b5, eq23_e2154_d_b6, eq23_e2154_d_b7, eq23_e2154_d_b8, eq23_e2154_d_b9, eq23_e2154_d_b10, eq23_e2154_d_b11, eq23_e2154_d_b12, eq23_e2154_d_b13, eq23_e2154_d_b14, eq23_e2154_d_b15, eq23_e2154_d_b16, eq23_e2154_d_b17,) = {
    if (((!s.b[1698]) && s.b[1701]) && s.b[1702]) {
        let eq23_e2151: f64 = (s.v[476] + s.v[488]);
        let eq23_e2151_d_n0: f64 = (s.dn[476][0] + s.dn[488][0]);
        let eq23_e2151_d_n1: f64 = (s.dn[476][1] + s.dn[488][1]);
        let eq23_e2151_d_n2: f64 = (s.dn[476][2] + s.dn[488][2]);
        let eq23_e2151_d_n3: f64 = (s.dn[476][3] + s.dn[488][3]);
        let eq23_e2151_d_n4: f64 = (s.dn[476][4] + s.dn[488][4]);
        let eq23_e2151_d_n5: f64 = (s.dn[476][5] + s.dn[488][5]);
        let eq23_e2151_d_n6: f64 = (s.dn[476][6] + s.dn[488][6]);
        let eq23_e2151_d_n7: f64 = (s.dn[476][7] + s.dn[488][7]);
        let eq23_e2151_d_n8: f64 = (s.dn[476][8] + s.dn[488][8]);
        let eq23_e2151_d_n9: f64 = (s.dn[476][9] + s.dn[488][9]);
        let eq23_e2151_d_n10: f64 = (s.dn[476][10] + s.dn[488][10]);
        let eq23_e2151_d_n11: f64 = (s.dn[476][11] + s.dn[488][11]);
        let eq23_e2151_d_n12: f64 = (s.dn[476][12] + s.dn[488][12]);
        let eq23_e2151_d_n13: f64 = (s.dn[476][13] + s.dn[488][13]);
        let eq23_e2151_d_n14: f64 = (s.dn[476][14] + s.dn[488][14]);
        let eq23_e2151_d_n15: f64 = (s.dn[476][15] + s.dn[488][15]);
        let eq23_e2151_d_n16: f64 = (s.dn[476][16] + s.dn[488][16]);
        let eq23_e2151_d_b0: f64 = (s.db[476][0] + s.db[488][0]);
        let eq23_e2151_d_b1: f64 = (s.db[476][1] + s.db[488][1]);
        let eq23_e2151_d_b2: f64 = (s.db[476][2] + s.db[488][2]);
        let eq23_e2151_d_b3: f64 = (s.db[476][3] + s.db[488][3]);
        let eq23_e2151_d_b4: f64 = (s.db[476][4] + s.db[488][4]);
        let eq23_e2151_d_b5: f64 = (s.db[476][5] + s.db[488][5]);
        let eq23_e2151_d_b6: f64 = (s.db[476][6] + s.db[488][6]);
        let eq23_e2151_d_b7: f64 = (s.db[476][7] + s.db[488][7]);
        let eq23_e2151_d_b8: f64 = (s.db[476][8] + s.db[488][8]);
        let eq23_e2151_d_b9: f64 = (s.db[476][9] + s.db[488][9]);
        let eq23_e2151_d_b10: f64 = (s.db[476][10] + s.db[488][10]);
        let eq23_e2151_d_b11: f64 = (s.db[476][11] + s.db[488][11]);
        let eq23_e2151_d_b12: f64 = (s.db[476][12] + s.db[488][12]);
        let eq23_e2151_d_b13: f64 = (s.db[476][13] + s.db[488][13]);
        let eq23_e2151_d_b14: f64 = (s.db[476][14] + s.db[488][14]);
        let eq23_e2151_d_b15: f64 = (s.db[476][15] + s.db[488][15]);
        let eq23_e2151_d_b16: f64 = (s.db[476][16] + s.db[488][16]);
        let eq23_e2151_d_b17: f64 = (s.db[476][17] + s.db[488][17]);
        let eq23_e2152: f64 = (s.v[114] * eq23_e2151);
        let eq23_e2152_d_n0: f64 = ((s.dn[114][0] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n0));
        let eq23_e2152_d_n1: f64 = ((s.dn[114][1] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n1));
        let eq23_e2152_d_n2: f64 = ((s.dn[114][2] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n2));
        let eq23_e2152_d_n3: f64 = ((s.dn[114][3] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n3));
        let eq23_e2152_d_n4: f64 = ((s.dn[114][4] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n4));
        let eq23_e2152_d_n5: f64 = ((s.dn[114][5] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n5));
        let eq23_e2152_d_n6: f64 = ((s.dn[114][6] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n6));
        let eq23_e2152_d_n7: f64 = ((s.dn[114][7] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n7));
        let eq23_e2152_d_n8: f64 = ((s.dn[114][8] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n8));
        let eq23_e2152_d_n9: f64 = ((s.dn[114][9] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n9));
        let eq23_e2152_d_n10: f64 = ((s.dn[114][10] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n10));
        let eq23_e2152_d_n11: f64 = ((s.dn[114][11] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n11));
        let eq23_e2152_d_n12: f64 = ((s.dn[114][12] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n12));
        let eq23_e2152_d_n13: f64 = ((s.dn[114][13] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n13));
        let eq23_e2152_d_n14: f64 = ((s.dn[114][14] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n14));
        let eq23_e2152_d_n15: f64 = ((s.dn[114][15] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n15));
        let eq23_e2152_d_n16: f64 = ((s.dn[114][16] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n16));
        let eq23_e2152_d_b0: f64 = ((s.db[114][0] * eq23_e2151) + (s.v[114] * eq23_e2151_d_b0));
        let eq23_e2152_d_b1: f64 = ((s.db[114][1] * eq23_e2151) + (s.v[114] * eq23_e2151_d_b1));
        let eq23_e2152_d_b2: f64 = ((s.db[114][2] * eq23_e2151) + (s.v[114] * eq23_e2151_d_b2));
        let eq23_e2152_d_b3: f64 = ((s.db[114][3] * eq23_e2151) + (s.v[114] * eq23_e2151_d_b3));
        let eq23_e2152_d_b4: f64 = ((s.db[114][4] * eq23_e2151) + (s.v[114] * eq23_e2151_d_b4));
        let eq23_e2152_d_b5: f64 = ((s.db[114][5] * eq23_e2151) + (s.v[114] * eq23_e2151_d_b5));
        let eq23_e2152_d_b6: f64 = ((s.db[114][6] * eq23_e2151) + (s.v[114] * eq23_e2151_d_b6));
        let eq23_e2152_d_b7: f64 = ((s.db[114][7] * eq23_e2151) + (s.v[114] * eq23_e2151_d_b7));
        let eq23_e2152_d_b8: f64 = ((s.db[114][8] * eq23_e2151) + (s.v[114] * eq23_e2151_d_b8));
        let eq23_e2152_d_b9: f64 = ((s.db[114][9] * eq23_e2151) + (s.v[114] * eq23_e2151_d_b9));
        let eq23_e2152_d_b10: f64 = ((s.db[114][10] * eq23_e2151) + (s.v[114] * eq23_e2151_d_b10));
        let eq23_e2152_d_b11: f64 = ((s.db[114][11] * eq23_e2151) + (s.v[114] * eq23_e2151_d_b11));
        let eq23_e2152_d_b12: f64 = ((s.db[114][12] * eq23_e2151) + (s.v[114] * eq23_e2151_d_b12));
        let eq23_e2152_d_b13: f64 = ((s.db[114][13] * eq23_e2151) + (s.v[114] * eq23_e2151_d_b13));
        let eq23_e2152_d_b14: f64 = ((s.db[114][14] * eq23_e2151) + (s.v[114] * eq23_e2151_d_b14));
        let eq23_e2152_d_b15: f64 = ((s.db[114][15] * eq23_e2151) + (s.v[114] * eq23_e2151_d_b15));
        let eq23_e2152_d_b16: f64 = ((s.db[114][16] * eq23_e2151) + (s.v[114] * eq23_e2151_d_b16));
        let eq23_e2152_d_b17: f64 = ((s.db[114][17] * eq23_e2151) + (s.v[114] * eq23_e2151_d_b17));
        (eq23_e2152, eq23_e2152_d_n0, eq23_e2152_d_n1, eq23_e2152_d_n2, eq23_e2152_d_n3, eq23_e2152_d_n4, eq23_e2152_d_n5, eq23_e2152_d_n6, eq23_e2152_d_n7, eq23_e2152_d_n8, eq23_e2152_d_n9, eq23_e2152_d_n10, eq23_e2152_d_n11, eq23_e2152_d_n12, eq23_e2152_d_n13, eq23_e2152_d_n14, eq23_e2152_d_n15, eq23_e2152_d_n16, eq23_e2152_d_b0, eq23_e2152_d_b1, eq23_e2152_d_b2, eq23_e2152_d_b3, eq23_e2152_d_b4, eq23_e2152_d_b5, eq23_e2152_d_b6, eq23_e2152_d_b7, eq23_e2152_d_b8, eq23_e2152_d_b9, eq23_e2152_d_b10, eq23_e2152_d_b11, eq23_e2152_d_b12, eq23_e2152_d_b13, eq23_e2152_d_b14, eq23_e2152_d_b15, eq23_e2152_d_b16, eq23_e2152_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e2154;
        let eq23_node_derivatives: [f64; 17] = [eq23_e2154_d_n0, eq23_e2154_d_n1, eq23_e2154_d_n2, eq23_e2154_d_n3, eq23_e2154_d_n4, eq23_e2154_d_n5, eq23_e2154_d_n6, eq23_e2154_d_n7, eq23_e2154_d_n8, eq23_e2154_d_n9, eq23_e2154_d_n10, eq23_e2154_d_n11, eq23_e2154_d_n12, eq23_e2154_d_n13, eq23_e2154_d_n14, eq23_e2154_d_n15, eq23_e2154_d_n16];
        let eq23_branch_derivatives: [f64; 18] = [eq23_e2154_d_b0, eq23_e2154_d_b1, eq23_e2154_d_b2, eq23_e2154_d_b3, eq23_e2154_d_b4, eq23_e2154_d_b5, eq23_e2154_d_b6, eq23_e2154_d_b7, eq23_e2154_d_b8, eq23_e2154_d_b9, eq23_e2154_d_b10, eq23_e2154_d_b11, eq23_e2154_d_b12, eq23_e2154_d_b13, eq23_e2154_d_b14, eq23_e2154_d_b15, eq23_e2154_d_b16, eq23_e2154_d_b17];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let (eq24_e2165, eq24_e2165_d_n0, eq24_e2165_d_n1, eq24_e2165_d_n2, eq24_e2165_d_n3, eq24_e2165_d_n4, eq24_e2165_d_n5, eq24_e2165_d_n6, eq24_e2165_d_n7, eq24_e2165_d_n8, eq24_e2165_d_n9, eq24_e2165_d_n10, eq24_e2165_d_n11, eq24_e2165_d_n12, eq24_e2165_d_n13, eq24_e2165_d_n14, eq24_e2165_d_n15, eq24_e2165_d_n16, eq24_e2165_d_b0, eq24_e2165_d_b1, eq24_e2165_d_b2, eq24_e2165_d_b3, eq24_e2165_d_b4, eq24_e2165_d_b5, eq24_e2165_d_b6, eq24_e2165_d_b7, eq24_e2165_d_b8, eq24_e2165_d_b9, eq24_e2165_d_b10, eq24_e2165_d_b11, eq24_e2165_d_b12, eq24_e2165_d_b13, eq24_e2165_d_b14, eq24_e2165_d_b15, eq24_e2165_d_b16, eq24_e2165_d_b17,) = {
    if (((!s.b[1698]) && s.b[1701]) && s.b[1702]) {
        let eq24_e2163: f64 = (s.v[114] * s.v[475]);
        let eq24_e2163_d_n0: f64 = ((s.dn[114][0] * s.v[475]) + (s.v[114] * s.dn[475][0]));
        let eq24_e2163_d_n1: f64 = ((s.dn[114][1] * s.v[475]) + (s.v[114] * s.dn[475][1]));
        let eq24_e2163_d_n2: f64 = ((s.dn[114][2] * s.v[475]) + (s.v[114] * s.dn[475][2]));
        let eq24_e2163_d_n3: f64 = ((s.dn[114][3] * s.v[475]) + (s.v[114] * s.dn[475][3]));
        let eq24_e2163_d_n4: f64 = ((s.dn[114][4] * s.v[475]) + (s.v[114] * s.dn[475][4]));
        let eq24_e2163_d_n5: f64 = ((s.dn[114][5] * s.v[475]) + (s.v[114] * s.dn[475][5]));
        let eq24_e2163_d_n6: f64 = ((s.dn[114][6] * s.v[475]) + (s.v[114] * s.dn[475][6]));
        let eq24_e2163_d_n7: f64 = ((s.dn[114][7] * s.v[475]) + (s.v[114] * s.dn[475][7]));
        let eq24_e2163_d_n8: f64 = ((s.dn[114][8] * s.v[475]) + (s.v[114] * s.dn[475][8]));
        let eq24_e2163_d_n9: f64 = ((s.dn[114][9] * s.v[475]) + (s.v[114] * s.dn[475][9]));
        let eq24_e2163_d_n10: f64 = ((s.dn[114][10] * s.v[475]) + (s.v[114] * s.dn[475][10]));
        let eq24_e2163_d_n11: f64 = ((s.dn[114][11] * s.v[475]) + (s.v[114] * s.dn[475][11]));
        let eq24_e2163_d_n12: f64 = ((s.dn[114][12] * s.v[475]) + (s.v[114] * s.dn[475][12]));
        let eq24_e2163_d_n13: f64 = ((s.dn[114][13] * s.v[475]) + (s.v[114] * s.dn[475][13]));
        let eq24_e2163_d_n14: f64 = ((s.dn[114][14] * s.v[475]) + (s.v[114] * s.dn[475][14]));
        let eq24_e2163_d_n15: f64 = ((s.dn[114][15] * s.v[475]) + (s.v[114] * s.dn[475][15]));
        let eq24_e2163_d_n16: f64 = ((s.dn[114][16] * s.v[475]) + (s.v[114] * s.dn[475][16]));
        let eq24_e2163_d_b0: f64 = ((s.db[114][0] * s.v[475]) + (s.v[114] * s.db[475][0]));
        let eq24_e2163_d_b1: f64 = ((s.db[114][1] * s.v[475]) + (s.v[114] * s.db[475][1]));
        let eq24_e2163_d_b2: f64 = ((s.db[114][2] * s.v[475]) + (s.v[114] * s.db[475][2]));
        let eq24_e2163_d_b3: f64 = ((s.db[114][3] * s.v[475]) + (s.v[114] * s.db[475][3]));
        let eq24_e2163_d_b4: f64 = ((s.db[114][4] * s.v[475]) + (s.v[114] * s.db[475][4]));
        let eq24_e2163_d_b5: f64 = ((s.db[114][5] * s.v[475]) + (s.v[114] * s.db[475][5]));
        let eq24_e2163_d_b6: f64 = ((s.db[114][6] * s.v[475]) + (s.v[114] * s.db[475][6]));
        let eq24_e2163_d_b7: f64 = ((s.db[114][7] * s.v[475]) + (s.v[114] * s.db[475][7]));
        let eq24_e2163_d_b8: f64 = ((s.db[114][8] * s.v[475]) + (s.v[114] * s.db[475][8]));
        let eq24_e2163_d_b9: f64 = ((s.db[114][9] * s.v[475]) + (s.v[114] * s.db[475][9]));
        let eq24_e2163_d_b10: f64 = ((s.db[114][10] * s.v[475]) + (s.v[114] * s.db[475][10]));
        let eq24_e2163_d_b11: f64 = ((s.db[114][11] * s.v[475]) + (s.v[114] * s.db[475][11]));
        let eq24_e2163_d_b12: f64 = ((s.db[114][12] * s.v[475]) + (s.v[114] * s.db[475][12]));
        let eq24_e2163_d_b13: f64 = ((s.db[114][13] * s.v[475]) + (s.v[114] * s.db[475][13]));
        let eq24_e2163_d_b14: f64 = ((s.db[114][14] * s.v[475]) + (s.v[114] * s.db[475][14]));
        let eq24_e2163_d_b15: f64 = ((s.db[114][15] * s.v[475]) + (s.v[114] * s.db[475][15]));
        let eq24_e2163_d_b16: f64 = ((s.db[114][16] * s.v[475]) + (s.v[114] * s.db[475][16]));
        let eq24_e2163_d_b17: f64 = ((s.db[114][17] * s.v[475]) + (s.v[114] * s.db[475][17]));
        (eq24_e2163, eq24_e2163_d_n0, eq24_e2163_d_n1, eq24_e2163_d_n2, eq24_e2163_d_n3, eq24_e2163_d_n4, eq24_e2163_d_n5, eq24_e2163_d_n6, eq24_e2163_d_n7, eq24_e2163_d_n8, eq24_e2163_d_n9, eq24_e2163_d_n10, eq24_e2163_d_n11, eq24_e2163_d_n12, eq24_e2163_d_n13, eq24_e2163_d_n14, eq24_e2163_d_n15, eq24_e2163_d_n16, eq24_e2163_d_b0, eq24_e2163_d_b1, eq24_e2163_d_b2, eq24_e2163_d_b3, eq24_e2163_d_b4, eq24_e2163_d_b5, eq24_e2163_d_b6, eq24_e2163_d_b7, eq24_e2163_d_b8, eq24_e2163_d_b9, eq24_e2163_d_b10, eq24_e2163_d_b11, eq24_e2163_d_b12, eq24_e2163_d_b13, eq24_e2163_d_b14, eq24_e2163_d_b15, eq24_e2163_d_b16, eq24_e2163_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e2165;
        let eq24_node_derivatives: [f64; 17] = [eq24_e2165_d_n0, eq24_e2165_d_n1, eq24_e2165_d_n2, eq24_e2165_d_n3, eq24_e2165_d_n4, eq24_e2165_d_n5, eq24_e2165_d_n6, eq24_e2165_d_n7, eq24_e2165_d_n8, eq24_e2165_d_n9, eq24_e2165_d_n10, eq24_e2165_d_n11, eq24_e2165_d_n12, eq24_e2165_d_n13, eq24_e2165_d_n14, eq24_e2165_d_n15, eq24_e2165_d_n16];
        let eq24_branch_derivatives: [f64; 18] = [eq24_e2165_d_b0, eq24_e2165_d_b1, eq24_e2165_d_b2, eq24_e2165_d_b3, eq24_e2165_d_b4, eq24_e2165_d_b5, eq24_e2165_d_b6, eq24_e2165_d_b7, eq24_e2165_d_b8, eq24_e2165_d_b9, eq24_e2165_d_b10, eq24_e2165_d_b11, eq24_e2165_d_b12, eq24_e2165_d_b13, eq24_e2165_d_b14, eq24_e2165_d_b15, eq24_e2165_d_b16, eq24_e2165_d_b17];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let (eq25_e2176, eq25_e2176_d_n0, eq25_e2176_d_n1, eq25_e2176_d_n2, eq25_e2176_d_n3, eq25_e2176_d_n4, eq25_e2176_d_n5, eq25_e2176_d_n6, eq25_e2176_d_n7, eq25_e2176_d_n8, eq25_e2176_d_n9, eq25_e2176_d_n10, eq25_e2176_d_n11, eq25_e2176_d_n12, eq25_e2176_d_n13, eq25_e2176_d_n14, eq25_e2176_d_n15, eq25_e2176_d_n16, eq25_e2176_d_b0, eq25_e2176_d_b1, eq25_e2176_d_b2, eq25_e2176_d_b3, eq25_e2176_d_b4, eq25_e2176_d_b5, eq25_e2176_d_b6, eq25_e2176_d_b7, eq25_e2176_d_b8, eq25_e2176_d_b9, eq25_e2176_d_b10, eq25_e2176_d_b11, eq25_e2176_d_b12, eq25_e2176_d_b13, eq25_e2176_d_b14, eq25_e2176_d_b15, eq25_e2176_d_b16, eq25_e2176_d_b17,) = {
    if (((!s.b[1698]) && s.b[1701]) && s.b[1702]) {
        let eq25_e2174: f64 = (s.v[114] * s.v[478]);
        let eq25_e2174_d_n0: f64 = ((s.dn[114][0] * s.v[478]) + (s.v[114] * s.dn[478][0]));
        let eq25_e2174_d_n1: f64 = ((s.dn[114][1] * s.v[478]) + (s.v[114] * s.dn[478][1]));
        let eq25_e2174_d_n2: f64 = ((s.dn[114][2] * s.v[478]) + (s.v[114] * s.dn[478][2]));
        let eq25_e2174_d_n3: f64 = ((s.dn[114][3] * s.v[478]) + (s.v[114] * s.dn[478][3]));
        let eq25_e2174_d_n4: f64 = ((s.dn[114][4] * s.v[478]) + (s.v[114] * s.dn[478][4]));
        let eq25_e2174_d_n5: f64 = ((s.dn[114][5] * s.v[478]) + (s.v[114] * s.dn[478][5]));
        let eq25_e2174_d_n6: f64 = ((s.dn[114][6] * s.v[478]) + (s.v[114] * s.dn[478][6]));
        let eq25_e2174_d_n7: f64 = ((s.dn[114][7] * s.v[478]) + (s.v[114] * s.dn[478][7]));
        let eq25_e2174_d_n8: f64 = ((s.dn[114][8] * s.v[478]) + (s.v[114] * s.dn[478][8]));
        let eq25_e2174_d_n9: f64 = ((s.dn[114][9] * s.v[478]) + (s.v[114] * s.dn[478][9]));
        let eq25_e2174_d_n10: f64 = ((s.dn[114][10] * s.v[478]) + (s.v[114] * s.dn[478][10]));
        let eq25_e2174_d_n11: f64 = ((s.dn[114][11] * s.v[478]) + (s.v[114] * s.dn[478][11]));
        let eq25_e2174_d_n12: f64 = ((s.dn[114][12] * s.v[478]) + (s.v[114] * s.dn[478][12]));
        let eq25_e2174_d_n13: f64 = ((s.dn[114][13] * s.v[478]) + (s.v[114] * s.dn[478][13]));
        let eq25_e2174_d_n14: f64 = ((s.dn[114][14] * s.v[478]) + (s.v[114] * s.dn[478][14]));
        let eq25_e2174_d_n15: f64 = ((s.dn[114][15] * s.v[478]) + (s.v[114] * s.dn[478][15]));
        let eq25_e2174_d_n16: f64 = ((s.dn[114][16] * s.v[478]) + (s.v[114] * s.dn[478][16]));
        let eq25_e2174_d_b0: f64 = ((s.db[114][0] * s.v[478]) + (s.v[114] * s.db[478][0]));
        let eq25_e2174_d_b1: f64 = ((s.db[114][1] * s.v[478]) + (s.v[114] * s.db[478][1]));
        let eq25_e2174_d_b2: f64 = ((s.db[114][2] * s.v[478]) + (s.v[114] * s.db[478][2]));
        let eq25_e2174_d_b3: f64 = ((s.db[114][3] * s.v[478]) + (s.v[114] * s.db[478][3]));
        let eq25_e2174_d_b4: f64 = ((s.db[114][4] * s.v[478]) + (s.v[114] * s.db[478][4]));
        let eq25_e2174_d_b5: f64 = ((s.db[114][5] * s.v[478]) + (s.v[114] * s.db[478][5]));
        let eq25_e2174_d_b6: f64 = ((s.db[114][6] * s.v[478]) + (s.v[114] * s.db[478][6]));
        let eq25_e2174_d_b7: f64 = ((s.db[114][7] * s.v[478]) + (s.v[114] * s.db[478][7]));
        let eq25_e2174_d_b8: f64 = ((s.db[114][8] * s.v[478]) + (s.v[114] * s.db[478][8]));
        let eq25_e2174_d_b9: f64 = ((s.db[114][9] * s.v[478]) + (s.v[114] * s.db[478][9]));
        let eq25_e2174_d_b10: f64 = ((s.db[114][10] * s.v[478]) + (s.v[114] * s.db[478][10]));
        let eq25_e2174_d_b11: f64 = ((s.db[114][11] * s.v[478]) + (s.v[114] * s.db[478][11]));
        let eq25_e2174_d_b12: f64 = ((s.db[114][12] * s.v[478]) + (s.v[114] * s.db[478][12]));
        let eq25_e2174_d_b13: f64 = ((s.db[114][13] * s.v[478]) + (s.v[114] * s.db[478][13]));
        let eq25_e2174_d_b14: f64 = ((s.db[114][14] * s.v[478]) + (s.v[114] * s.db[478][14]));
        let eq25_e2174_d_b15: f64 = ((s.db[114][15] * s.v[478]) + (s.v[114] * s.db[478][15]));
        let eq25_e2174_d_b16: f64 = ((s.db[114][16] * s.v[478]) + (s.v[114] * s.db[478][16]));
        let eq25_e2174_d_b17: f64 = ((s.db[114][17] * s.v[478]) + (s.v[114] * s.db[478][17]));
        (eq25_e2174, eq25_e2174_d_n0, eq25_e2174_d_n1, eq25_e2174_d_n2, eq25_e2174_d_n3, eq25_e2174_d_n4, eq25_e2174_d_n5, eq25_e2174_d_n6, eq25_e2174_d_n7, eq25_e2174_d_n8, eq25_e2174_d_n9, eq25_e2174_d_n10, eq25_e2174_d_n11, eq25_e2174_d_n12, eq25_e2174_d_n13, eq25_e2174_d_n14, eq25_e2174_d_n15, eq25_e2174_d_n16, eq25_e2174_d_b0, eq25_e2174_d_b1, eq25_e2174_d_b2, eq25_e2174_d_b3, eq25_e2174_d_b4, eq25_e2174_d_b5, eq25_e2174_d_b6, eq25_e2174_d_b7, eq25_e2174_d_b8, eq25_e2174_d_b9, eq25_e2174_d_b10, eq25_e2174_d_b11, eq25_e2174_d_b12, eq25_e2174_d_b13, eq25_e2174_d_b14, eq25_e2174_d_b15, eq25_e2174_d_b16, eq25_e2174_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e2176;
        let eq25_node_derivatives: [f64; 17] = [eq25_e2176_d_n0, eq25_e2176_d_n1, eq25_e2176_d_n2, eq25_e2176_d_n3, eq25_e2176_d_n4, eq25_e2176_d_n5, eq25_e2176_d_n6, eq25_e2176_d_n7, eq25_e2176_d_n8, eq25_e2176_d_n9, eq25_e2176_d_n10, eq25_e2176_d_n11, eq25_e2176_d_n12, eq25_e2176_d_n13, eq25_e2176_d_n14, eq25_e2176_d_n15, eq25_e2176_d_n16];
        let eq25_branch_derivatives: [f64; 18] = [eq25_e2176_d_b0, eq25_e2176_d_b1, eq25_e2176_d_b2, eq25_e2176_d_b3, eq25_e2176_d_b4, eq25_e2176_d_b5, eq25_e2176_d_b6, eq25_e2176_d_b7, eq25_e2176_d_b8, eq25_e2176_d_b9, eq25_e2176_d_b10, eq25_e2176_d_b11, eq25_e2176_d_b12, eq25_e2176_d_b13, eq25_e2176_d_b14, eq25_e2176_d_b15, eq25_e2176_d_b16, eq25_e2176_d_b17];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(3),
            multiplicity * (eq25_value),
            &eq25_node_derivatives,
            &eq25_branch_derivatives,
            multiplicity,
        );
        let (eq26_e2187, eq26_e2187_d_n0, eq26_e2187_d_n1, eq26_e2187_d_n2, eq26_e2187_d_n3, eq26_e2187_d_n4, eq26_e2187_d_n5, eq26_e2187_d_n6, eq26_e2187_d_n7, eq26_e2187_d_n8, eq26_e2187_d_n9, eq26_e2187_d_n10, eq26_e2187_d_n11, eq26_e2187_d_n12, eq26_e2187_d_n13, eq26_e2187_d_n14, eq26_e2187_d_n15, eq26_e2187_d_n16, eq26_e2187_d_b0, eq26_e2187_d_b1, eq26_e2187_d_b2, eq26_e2187_d_b3, eq26_e2187_d_b4, eq26_e2187_d_b5, eq26_e2187_d_b6, eq26_e2187_d_b7, eq26_e2187_d_b8, eq26_e2187_d_b9, eq26_e2187_d_b10, eq26_e2187_d_b11, eq26_e2187_d_b12, eq26_e2187_d_b13, eq26_e2187_d_b14, eq26_e2187_d_b15, eq26_e2187_d_b16, eq26_e2187_d_b17,) = {
    if (((!s.b[1698]) && s.b[1701]) && s.b[1702]) {
        let eq26_e2185: f64 = (s.v[114] * s.v[477]);
        let eq26_e2185_d_n0: f64 = ((s.dn[114][0] * s.v[477]) + (s.v[114] * s.dn[477][0]));
        let eq26_e2185_d_n1: f64 = ((s.dn[114][1] * s.v[477]) + (s.v[114] * s.dn[477][1]));
        let eq26_e2185_d_n2: f64 = ((s.dn[114][2] * s.v[477]) + (s.v[114] * s.dn[477][2]));
        let eq26_e2185_d_n3: f64 = ((s.dn[114][3] * s.v[477]) + (s.v[114] * s.dn[477][3]));
        let eq26_e2185_d_n4: f64 = ((s.dn[114][4] * s.v[477]) + (s.v[114] * s.dn[477][4]));
        let eq26_e2185_d_n5: f64 = ((s.dn[114][5] * s.v[477]) + (s.v[114] * s.dn[477][5]));
        let eq26_e2185_d_n6: f64 = ((s.dn[114][6] * s.v[477]) + (s.v[114] * s.dn[477][6]));
        let eq26_e2185_d_n7: f64 = ((s.dn[114][7] * s.v[477]) + (s.v[114] * s.dn[477][7]));
        let eq26_e2185_d_n8: f64 = ((s.dn[114][8] * s.v[477]) + (s.v[114] * s.dn[477][8]));
        let eq26_e2185_d_n9: f64 = ((s.dn[114][9] * s.v[477]) + (s.v[114] * s.dn[477][9]));
        let eq26_e2185_d_n10: f64 = ((s.dn[114][10] * s.v[477]) + (s.v[114] * s.dn[477][10]));
        let eq26_e2185_d_n11: f64 = ((s.dn[114][11] * s.v[477]) + (s.v[114] * s.dn[477][11]));
        let eq26_e2185_d_n12: f64 = ((s.dn[114][12] * s.v[477]) + (s.v[114] * s.dn[477][12]));
        let eq26_e2185_d_n13: f64 = ((s.dn[114][13] * s.v[477]) + (s.v[114] * s.dn[477][13]));
        let eq26_e2185_d_n14: f64 = ((s.dn[114][14] * s.v[477]) + (s.v[114] * s.dn[477][14]));
        let eq26_e2185_d_n15: f64 = ((s.dn[114][15] * s.v[477]) + (s.v[114] * s.dn[477][15]));
        let eq26_e2185_d_n16: f64 = ((s.dn[114][16] * s.v[477]) + (s.v[114] * s.dn[477][16]));
        let eq26_e2185_d_b0: f64 = ((s.db[114][0] * s.v[477]) + (s.v[114] * s.db[477][0]));
        let eq26_e2185_d_b1: f64 = ((s.db[114][1] * s.v[477]) + (s.v[114] * s.db[477][1]));
        let eq26_e2185_d_b2: f64 = ((s.db[114][2] * s.v[477]) + (s.v[114] * s.db[477][2]));
        let eq26_e2185_d_b3: f64 = ((s.db[114][3] * s.v[477]) + (s.v[114] * s.db[477][3]));
        let eq26_e2185_d_b4: f64 = ((s.db[114][4] * s.v[477]) + (s.v[114] * s.db[477][4]));
        let eq26_e2185_d_b5: f64 = ((s.db[114][5] * s.v[477]) + (s.v[114] * s.db[477][5]));
        let eq26_e2185_d_b6: f64 = ((s.db[114][6] * s.v[477]) + (s.v[114] * s.db[477][6]));
        let eq26_e2185_d_b7: f64 = ((s.db[114][7] * s.v[477]) + (s.v[114] * s.db[477][7]));
        let eq26_e2185_d_b8: f64 = ((s.db[114][8] * s.v[477]) + (s.v[114] * s.db[477][8]));
        let eq26_e2185_d_b9: f64 = ((s.db[114][9] * s.v[477]) + (s.v[114] * s.db[477][9]));
        let eq26_e2185_d_b10: f64 = ((s.db[114][10] * s.v[477]) + (s.v[114] * s.db[477][10]));
        let eq26_e2185_d_b11: f64 = ((s.db[114][11] * s.v[477]) + (s.v[114] * s.db[477][11]));
        let eq26_e2185_d_b12: f64 = ((s.db[114][12] * s.v[477]) + (s.v[114] * s.db[477][12]));
        let eq26_e2185_d_b13: f64 = ((s.db[114][13] * s.v[477]) + (s.v[114] * s.db[477][13]));
        let eq26_e2185_d_b14: f64 = ((s.db[114][14] * s.v[477]) + (s.v[114] * s.db[477][14]));
        let eq26_e2185_d_b15: f64 = ((s.db[114][15] * s.v[477]) + (s.v[114] * s.db[477][15]));
        let eq26_e2185_d_b16: f64 = ((s.db[114][16] * s.v[477]) + (s.v[114] * s.db[477][16]));
        let eq26_e2185_d_b17: f64 = ((s.db[114][17] * s.v[477]) + (s.v[114] * s.db[477][17]));
        (eq26_e2185, eq26_e2185_d_n0, eq26_e2185_d_n1, eq26_e2185_d_n2, eq26_e2185_d_n3, eq26_e2185_d_n4, eq26_e2185_d_n5, eq26_e2185_d_n6, eq26_e2185_d_n7, eq26_e2185_d_n8, eq26_e2185_d_n9, eq26_e2185_d_n10, eq26_e2185_d_n11, eq26_e2185_d_n12, eq26_e2185_d_n13, eq26_e2185_d_n14, eq26_e2185_d_n15, eq26_e2185_d_n16, eq26_e2185_d_b0, eq26_e2185_d_b1, eq26_e2185_d_b2, eq26_e2185_d_b3, eq26_e2185_d_b4, eq26_e2185_d_b5, eq26_e2185_d_b6, eq26_e2185_d_b7, eq26_e2185_d_b8, eq26_e2185_d_b9, eq26_e2185_d_b10, eq26_e2185_d_b11, eq26_e2185_d_b12, eq26_e2185_d_b13, eq26_e2185_d_b14, eq26_e2185_d_b15, eq26_e2185_d_b16, eq26_e2185_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e2187;
        let eq26_node_derivatives: [f64; 17] = [eq26_e2187_d_n0, eq26_e2187_d_n1, eq26_e2187_d_n2, eq26_e2187_d_n3, eq26_e2187_d_n4, eq26_e2187_d_n5, eq26_e2187_d_n6, eq26_e2187_d_n7, eq26_e2187_d_n8, eq26_e2187_d_n9, eq26_e2187_d_n10, eq26_e2187_d_n11, eq26_e2187_d_n12, eq26_e2187_d_n13, eq26_e2187_d_n14, eq26_e2187_d_n15, eq26_e2187_d_n16];
        let eq26_branch_derivatives: [f64; 18] = [eq26_e2187_d_b0, eq26_e2187_d_b1, eq26_e2187_d_b2, eq26_e2187_d_b3, eq26_e2187_d_b4, eq26_e2187_d_b5, eq26_e2187_d_b6, eq26_e2187_d_b7, eq26_e2187_d_b8, eq26_e2187_d_b9, eq26_e2187_d_b10, eq26_e2187_d_b11, eq26_e2187_d_b12, eq26_e2187_d_b13, eq26_e2187_d_b14, eq26_e2187_d_b15, eq26_e2187_d_b16, eq26_e2187_d_b17];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(3),
            multiplicity * (eq26_value),
            &eq26_node_derivatives,
            &eq26_branch_derivatives,
            multiplicity,
        );
    }
}
