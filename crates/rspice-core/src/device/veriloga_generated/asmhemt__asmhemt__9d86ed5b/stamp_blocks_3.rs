#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_19(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[508] && s.b[509]) && s.b[515]) {
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 130, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_mul_scaled_powf_rhs(116, 136, p.p195, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p196, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if ((s.b[508] && s.b[509]) && s.b[515]) {
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(119))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if ((s.b[508] && s.b[509]) && s.b[515]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p195, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p196, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(285, 128, 86);
        }

        if ((s.b[508] && s.b[509]) && (!s.b[515])) {
            s.store_add(285, 100, 86);
        }

        if (s.b[508] && s.b[509]) {
            s.store_scaled_add(286, 284, 285, 0.5);
            s.store_sub(287, 285, 284);
            s.store_mul_add_scaled_inputs3_offset_rhs(135, 287, s.ad_value(282), 1.0, s.ad_value(286), (-1.0), s.ad_value(83), 1.0, 0.0);
            s.store_mul_scaled_abs_ad_rhs(136, 283, 1.0 / (p.p9), A::sub(s.ad_value(282), s.ad_value(286)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);
            s.store_scaled_mul(96, 95, 283, (p.p4 * (p.p5 * 1.0 / (p.p187))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(292), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(287), (p.p25 * p.p25), s.ad_value(287)), 1.0);
            s.store_div(93, 98, 92);
            s.store_sub(90, 285, 284);
            s.store_add_scaled_inputs3_indices(91, 282, 1.0, 83, 1.0, 286, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 283, s.ad_value(282), ((p.p4 * p.p5) * p.p187), s.ad_value(286), (((-1.0)) * (((p.p4 * p.p5) * p.p187))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p187), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p242) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p241, 1.0);
            s.store_div_from_scalar(190, p.p240, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p186);
            s.store_mul_add_scaled_inputs3_offset_rhs(288, 191, s.ad_value(282), ((p.p4 * p.p5) * p.p187), s.ad_value(286), (((-1.0)) * (((p.p4 * p.p5) * p.p187))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p187), 0.0);
            s.store_add_scaled_inputs3_indices(136, 282, 1.0, 83, 1.0, 286, -1.0);
            s.store_add_scaled_inputs(90, 284, 0.3333333333333333, 285, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(287)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(287)), 287, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(289, 191, 282, (-(((p.p4 * p.p187) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p187) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p187) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p187) * p.p5) * 0.5)));
        }

        s.b[516] = (s.v[68] < 0.0);
        s.v[516] = if s.b[516] { 1.0 } else { 0.0 };

        if ((s.b[508] && s.b[509]) && s.b[516]) {
            s.store_sub_scaled_inputs(289, 288, (-1.0), 289, 1.0);
        }

        if (s.b[508] && (!s.b[509])) {
            s.store_scalar(288, 0.0);
            s.store_scalar(289, 0.0);
        }

        s.b[517] = (p.p155 != 0.0);
        s.v[517] = if s.b[517] { 1.0 } else { 0.0 };

        s.b[518] = (p.p155 == 1.0);
        s.v[518] = if s.b[518] { 1.0 } else { 0.0 };

        if (((!s.b[508]) && s.b[517]) && s.b[518]) {
            s.store_voltage(70, ctx, nodes, Some(9), Some(8));
        }

        if (((!s.b[508]) && s.b[517]) && (!s.b[518])) {
            s.store_voltage(70, ctx, nodes, Some(2), Some(8));
        }

        if ((!s.b[508]) && s.b[517]) {
            s.copy_ad(290, 70);
            s.store_scalar(146, (1.0 + p.p191));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_offset_scaled(88, 82, ((1.0 / (s.v[35])) * (p.p188)), (((((-1.0)) * (p.p188))) + (p.p185)));
            s.store_scalar(283, (p.p9 / p.p186));
            s.store_div_from_scalar_scaled_mul(136, p.p187, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p184), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 290, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(290), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);
            s.store_sub(282, 160, 88);
            s.store_div_scaled_inputs_indices(84, 283, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 283, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 282, 282, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p195 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 282, 1.0, 83, 2.0);
        }

        s.b[519] = (s.v[136] < 200.0);
        s.v[519] = if s.b[519] { 1.0 } else { 0.0 };

        if (((!s.b[508]) && s.b[517]) && s.b[519]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(282), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[508]) && s.b[517]) && (!s.b[519])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(282), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[508]) && s.b[517]) {
            s.store_sub_div_rhs_indices(100, 282, 153, 99);
        }

        s.b[520] = ((((s.v[100] - s.v[282])) as f64).abs() > 1e-19);
        s.v[520] = if s.b[520] { 1.0 } else { 0.0 };

        if (((!s.b[508]) && s.b[517]) && s.b[520]) {
            s.store_sub(101, 282, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p195);
            s.store_scaled_mul(103, 136, 90, p.p196);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if (((!s.b[508]) && s.b[517]) && s.b[520]) {
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(105))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if (((!s.b[508]) && s.b[517]) && s.b[520]) {
            s.store_scaled_mul(107, 136, 91, p.p195);
            s.store_scaled_mul(108, 136, 91, p.p196);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 282, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p195, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p196, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

    }

    pub(super) fn stamp_reactive_block_20(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[508]) && s.b[517]) && s.b[520]) {
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(119))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if (((!s.b[508]) && s.b[517]) && s.b[520]) {
            s.store_scaled_mul(121, 136, 137, p.p195);
            s.store_scaled_mul(122, 136, 137, p.p196);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.copy_ad(284, 128);
        }

        if (((!s.b[508]) && s.b[517]) && (!s.b[520])) {
            s.copy_ad(284, 100);
        }

        if ((!s.b[508]) && s.b[517]) {
            s.store_scalar(291, 0.0);
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p189);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p190);
            s.store_mul_scaled_abs_ad_rhs(136, 283, 1.0 / (p.p9), A::sub(s.ad_value(282), s.ad_value(284)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(284)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(90, 282, 282, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p187, 136, p.p187, 90, 1.0, 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(291), s.ad_value(85)), p.p18);
            s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));
            s.store_mul(86, 291, 90);
            s.store_sub(39, 282, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p195 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[521] = (s.v[136] < 200.0);
        s.v[521] = if s.b[521] { 1.0 } else { 0.0 };

        if (((!s.b[508]) && s.b[517]) && s.b[521]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[508]) && s.b[517]) && (!s.b[521])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[508]) && s.b[517]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[522] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[522] = if s.b[522] { 1.0 } else { 0.0 };

        if (((!s.b[508]) && s.b[517]) && s.b[522]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p195);
            s.store_scaled_mul(103, 136, 90, p.p196);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if (((!s.b[508]) && s.b[517]) && s.b[522]) {
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(105))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if (((!s.b[508]) && s.b[517]) && s.b[522]) {
            s.store_scaled_mul(107, 136, 91, p.p195);
            s.store_scaled_mul(108, 136, 91, p.p196);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 130, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_mul_scaled_powf_rhs(116, 136, p.p195, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p196, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if (((!s.b[508]) && s.b[517]) && s.b[522]) {
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(119))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if (((!s.b[508]) && s.b[517]) && s.b[522]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p195, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p196, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(285, 128, 86);
        }

        if (((!s.b[508]) && s.b[517]) && (!s.b[522])) {
            s.store_add(285, 100, 86);
        }

        if ((!s.b[508]) && s.b[517]) {
            s.store_scaled_add(286, 284, 285, 0.5);
            s.store_sub(287, 285, 284);
            s.store_sub(90, 285, 284);
            s.store_add_scaled_inputs3_indices(91, 282, 1.0, 83, 1.0, 286, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 283, s.ad_value(282), ((p.p4 * p.p5) * p.p187), s.ad_value(286), (((-1.0)) * (((p.p4 * p.p5) * p.p187))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p187), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p242) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p241, 1.0);
            s.store_div_from_scalar(190, p.p240, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p186);
            s.store_mul_add_scaled_inputs3_offset_rhs(288, 191, s.ad_value(282), ((p.p4 * p.p5) * p.p187), s.ad_value(286), (((-1.0)) * (((p.p4 * p.p5) * p.p187))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p187), 0.0);
            s.store_add_scaled_inputs3_indices(136, 282, 1.0, 83, 1.0, 286, -1.0);
            s.store_add_scaled_inputs(90, 284, 0.3333333333333333, 285, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(287)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(287)), 287, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(289, 191, 282, (-(((p.p4 * p.p187) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p187) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p187) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p187) * p.p5) * 0.5)));
        }

        if ((!s.b[508]) && (!s.b[517])) {
            s.store_scalar(288, 0.0);
            s.store_scalar(289, 0.0);
        }

        s.b[523] = (p.p149 == 0.0);
        s.v[523] = if s.b[523] { 1.0 } else { 0.0 };

        s.b[524] = (p.p156 != 0.0);
        s.v[524] = if s.b[524] { 1.0 } else { 0.0 };

        if (s.b[523] && s.b[524]) {
            s.store_voltage(73, ctx, nodes, Some(18), Some(17));
        }

        s.b[525] = (p.p156 == 1.0);
        s.v[525] = if s.b[525] { 1.0 } else { 0.0 };

        if ((s.b[523] && s.b[524]) && s.b[525]) {
            s.store_voltage(74, ctx, nodes, Some(9), Some(17));
            s.store_voltage(75, ctx, nodes, Some(9), Some(18));
        }

        if ((s.b[523] && s.b[524]) && (!s.b[525])) {
            s.store_voltage(74, ctx, nodes, Some(2), Some(17));
            s.store_voltage(75, ctx, nodes, Some(2), Some(18));
        }

        if (s.b[523] && s.b[524]) {
            s.store_scalar(72, 1.0);
        }

        s.b[526] = (s.v[73] < 0.0);
        s.v[526] = if s.b[526] { 1.0 } else { 0.0 };

        if ((s.b[523] && s.b[524]) && s.b[526]) {
            s.store_scalar(72, (-1.0));
            s.store_mul(303, 72, 73);
        }

    }

    pub(super) fn stamp_reactive_block_21(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[523] && s.b[524]) && s.b[526]) {
            s.copy_ad(302, 75);
        }

        if ((s.b[523] && s.b[524]) && (!s.b[526])) {
            s.copy_ad(303, 73);
            s.copy_ad(302, 74);
        }

        if (s.b[523] && s.b[524]) {
            s.store_offset_sqrt_ad(304, A::offset(A::square(s.ad_value(303)), 0.01), (-0.1));
            s.store_offset_scaled(146, 304, p.p205, (1.0 + p.p204));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_sub_ad(88, A::sub_from_scalar(p.p198, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p201)), (((-1.0)) * (p.p201)))), A::div_scaled_inputs(s.ad_value(304), (p.p207 * p.p206), A::sqrt_square_offset(s.ad_value(304), (p.p207 * p.p207)), 1.0));
            s.store_scalar(295, (p.p9 / p.p199));
            s.store_div_from_scalar_scaled_mul(136, p.p200, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p197), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 302, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(302), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);
            s.store_sub(294, 160, 88);
            s.store_div_scaled_inputs_indices(84, 295, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 295, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 294, 294, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 294, 1.0, 83, 2.0);
        }

        s.b[527] = (s.v[136] < 200.0);
        s.v[527] = if s.b[527] { 1.0 } else { 0.0 };

        if ((s.b[523] && s.b[524]) && s.b[527]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(294), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[523] && s.b[524]) && (!s.b[527])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(294), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[523] && s.b[524]) {
            s.store_sub_div_rhs_indices(100, 294, 153, 99);
        }

        s.b[528] = ((((s.v[100] - s.v[294])) as f64).abs() > 1e-19);
        s.v[528] = if s.b[528] { 1.0 } else { 0.0 };

        if ((s.b[523] && s.b[524]) && s.b[528]) {
            s.store_sub(101, 294, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p208);
            s.store_scaled_mul(103, 136, 90, p.p209);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if ((s.b[523] && s.b[524]) && s.b[528]) {
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(105))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if ((s.b[523] && s.b[524]) && s.b[528]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 294, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p208, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p209, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if ((s.b[523] && s.b[524]) && s.b[528]) {
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(119))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if ((s.b[523] && s.b[524]) && s.b[528]) {
            s.store_scaled_mul(121, 136, 137, p.p208);
            s.store_scaled_mul(122, 136, 137, p.p209);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.copy_ad(296, 128);
        }

        if ((s.b[523] && s.b[524]) && (!s.b[528])) {
            s.copy_ad(296, 100);
        }

        if (s.b[523] && s.b[524]) {
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p202);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p203);
            s.store_mul_scaled_abs_ad_rhs(136, 295, 1.0 / (p.p9), A::sub(s.ad_value(294), s.ad_value(296)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(296)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(90, 294, 294, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p200, 136, p.p200, 90, 1.0, 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(303), s.ad_value(85)), p.p18);
            s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));
            s.store_mul(86, 303, 90);
            s.store_sub(39, 294, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[529] = (s.v[136] < 200.0);
        s.v[529] = if s.b[529] { 1.0 } else { 0.0 };

        if ((s.b[523] && s.b[524]) && s.b[529]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[523] && s.b[524]) && (!s.b[529])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[523] && s.b[524]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[530] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[530] = if s.b[530] { 1.0 } else { 0.0 };

        if ((s.b[523] && s.b[524]) && s.b[530]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p208);
            s.store_scaled_mul(103, 136, 90, p.p209);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if ((s.b[523] && s.b[524]) && s.b[530]) {
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(105))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if ((s.b[523] && s.b[524]) && s.b[530]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
        }

    }

    pub(super) fn stamp_reactive_block_22(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[523] && s.b[524]) && s.b[530]) {
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 130, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_mul_scaled_powf_rhs(116, 136, p.p208, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p209, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if ((s.b[523] && s.b[524]) && s.b[530]) {
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(119))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if ((s.b[523] && s.b[524]) && s.b[530]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p208, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p209, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(297, 128, 86);
        }

        if ((s.b[523] && s.b[524]) && (!s.b[530])) {
            s.store_add(297, 100, 86);
        }

        if (s.b[523] && s.b[524]) {
            s.store_scaled_add(298, 296, 297, 0.5);
            s.store_sub(299, 297, 296);
            s.store_mul_add_scaled_inputs3_offset_rhs(135, 299, s.ad_value(294), 1.0, s.ad_value(298), (-1.0), s.ad_value(83), 1.0, 0.0);
            s.store_mul_scaled_abs_ad_rhs(136, 295, 1.0 / (p.p9), A::sub(s.ad_value(294), s.ad_value(298)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);
            s.store_scaled_mul(96, 95, 295, (p.p4 * (p.p5 * 1.0 / (p.p200))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(304), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(299), (p.p25 * p.p25), s.ad_value(299)), 1.0);
            s.store_div(93, 98, 92);
            s.store_sub(90, 297, 296);
            s.store_add_scaled_inputs3_indices(91, 294, 1.0, 83, 1.0, 298, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 295, s.ad_value(294), ((p.p4 * p.p5) * p.p200), s.ad_value(298), (((-1.0)) * (((p.p4 * p.p5) * p.p200))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p200), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p245) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p244, 1.0);
            s.store_div_from_scalar(190, p.p243, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p199);
            s.store_mul_add_scaled_inputs3_offset_rhs(300, 191, s.ad_value(294), ((p.p4 * p.p5) * p.p200), s.ad_value(298), (((-1.0)) * (((p.p4 * p.p5) * p.p200))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p200), 0.0);
            s.store_add_scaled_inputs3_indices(136, 294, 1.0, 83, 1.0, 298, -1.0);
            s.store_add_scaled_inputs(90, 296, 0.3333333333333333, 297, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(299)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(299)), 299, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(301, 191, 294, (-(((p.p4 * p.p200) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p200) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p200) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p200) * p.p5) * 0.5)));
        }

        s.b[531] = (s.v[72] < 0.0);
        s.v[531] = if s.b[531] { 1.0 } else { 0.0 };

        if ((s.b[523] && s.b[524]) && s.b[531]) {
            s.store_sub_scaled_inputs(301, 300, (-1.0), 301, 1.0);
        }

        if (s.b[523] && (!s.b[524])) {
            s.store_scalar(300, 0.0);
            s.store_scalar(301, 0.0);
        }

        s.b[532] = (p.p156 != 0.0);
        s.v[532] = if s.b[532] { 1.0 } else { 0.0 };

        s.b[533] = (p.p156 == 1.0);
        s.v[533] = if s.b[533] { 1.0 } else { 0.0 };

        if (((!s.b[523]) && s.b[532]) && s.b[533]) {
            s.store_voltage(74, ctx, nodes, Some(9), Some(7));
        }

        if (((!s.b[523]) && s.b[532]) && (!s.b[533])) {
            s.store_voltage(74, ctx, nodes, Some(2), Some(7));
        }

        if ((!s.b[523]) && s.b[532]) {
            s.copy_ad(302, 74);
            s.store_scalar(146, (1.0 + p.p204));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_sub_from_scalar_ad(88, p.p198, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p201)), (((-1.0)) * (p.p201))));
            s.store_scalar(295, (p.p9 / p.p199));
            s.store_div_from_scalar_scaled_mul(136, p.p200, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p197), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 302, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(302), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);
            s.store_sub(294, 160, 88);
            s.store_div_scaled_inputs_indices(84, 295, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 295, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 294, 294, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 294, 1.0, 83, 2.0);
        }

        s.b[534] = (s.v[136] < 200.0);
        s.v[534] = if s.b[534] { 1.0 } else { 0.0 };

        if (((!s.b[523]) && s.b[532]) && s.b[534]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(294), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[523]) && s.b[532]) && (!s.b[534])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(294), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[523]) && s.b[532]) {
            s.store_sub_div_rhs_indices(100, 294, 153, 99);
        }

        s.b[535] = ((((s.v[100] - s.v[294])) as f64).abs() > 1e-19);
        s.v[535] = if s.b[535] { 1.0 } else { 0.0 };

        if (((!s.b[523]) && s.b[532]) && s.b[535]) {
            s.store_sub(101, 294, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p208);
            s.store_scaled_mul(103, 136, 90, p.p209);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if (((!s.b[523]) && s.b[532]) && s.b[535]) {
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(105))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if (((!s.b[523]) && s.b[532]) && s.b[535]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 294, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p208, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p209, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

    }

    pub(super) fn stamp_reactive_block_23(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[523]) && s.b[532]) && s.b[535]) {
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(119))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if (((!s.b[523]) && s.b[532]) && s.b[535]) {
            s.store_scaled_mul(121, 136, 137, p.p208);
            s.store_scaled_mul(122, 136, 137, p.p209);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.copy_ad(296, 128);
        }

        if (((!s.b[523]) && s.b[532]) && (!s.b[535])) {
            s.copy_ad(296, 100);
        }

        if ((!s.b[523]) && s.b[532]) {
            s.store_scalar(303, 0.0);
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p202);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p203);
            s.store_mul_scaled_abs_ad_rhs(136, 295, 1.0 / (p.p9), A::sub(s.ad_value(294), s.ad_value(296)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(296)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(90, 294, 294, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p200, 136, p.p200, 90, 1.0, 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(303), s.ad_value(85)), p.p18);
            s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));
            s.store_mul(86, 303, 90);
            s.store_sub(39, 294, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[536] = (s.v[136] < 200.0);
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        if (((!s.b[523]) && s.b[532]) && s.b[536]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[523]) && s.b[532]) && (!s.b[536])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[523]) && s.b[532]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[537] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[537] = if s.b[537] { 1.0 } else { 0.0 };

        if (((!s.b[523]) && s.b[532]) && s.b[537]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p208);
            s.store_scaled_mul(103, 136, 90, p.p209);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if (((!s.b[523]) && s.b[532]) && s.b[537]) {
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(105))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if (((!s.b[523]) && s.b[532]) && s.b[537]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 130, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_mul_scaled_powf_rhs(116, 136, p.p208, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p209, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if (((!s.b[523]) && s.b[532]) && s.b[537]) {
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(119))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if (((!s.b[523]) && s.b[532]) && s.b[537]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p208, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p209, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(297, 128, 86);
        }

        if (((!s.b[523]) && s.b[532]) && (!s.b[537])) {
            s.store_add(297, 100, 86);
        }

        if ((!s.b[523]) && s.b[532]) {
            s.store_scaled_add(298, 296, 297, 0.5);
            s.store_sub(299, 297, 296);
            s.store_sub(90, 297, 296);
            s.store_add_scaled_inputs3_indices(91, 294, 1.0, 83, 1.0, 298, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 295, s.ad_value(294), ((p.p4 * p.p5) * p.p200), s.ad_value(298), (((-1.0)) * (((p.p4 * p.p5) * p.p200))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p200), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p245) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p244, 1.0);
            s.store_div_from_scalar(190, p.p243, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p199);
            s.store_mul_add_scaled_inputs3_offset_rhs(300, 191, s.ad_value(294), ((p.p4 * p.p5) * p.p200), s.ad_value(298), (((-1.0)) * (((p.p4 * p.p5) * p.p200))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p200), 0.0);
            s.store_add_scaled_inputs3_indices(136, 294, 1.0, 83, 1.0, 298, -1.0);
            s.store_add_scaled_inputs(90, 296, 0.3333333333333333, 297, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(299)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(299)), 299, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(301, 191, 294, (-(((p.p4 * p.p200) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p200) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p200) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p200) * p.p5) * 0.5)));
        }

        if ((!s.b[523]) && (!s.b[532])) {
            s.store_scalar(300, 0.0);
            s.store_scalar(301, 0.0);
        }

        s.b[538] = (p.p149 == 0.0);
        s.v[538] = if s.b[538] { 1.0 } else { 0.0 };

        s.b[539] = (p.p157 != 0.0);
        s.v[539] = if s.b[539] { 1.0 } else { 0.0 };

        if (s.b[538] && s.b[539]) {
            s.store_voltage(77, ctx, nodes, Some(21), Some(22));
        }

        s.b[540] = (p.p157 == 1.0);
        s.v[540] = if s.b[540] { 1.0 } else { 0.0 };

        if ((s.b[538] && s.b[539]) && s.b[540]) {
            s.store_voltage(78, ctx, nodes, Some(9), Some(22));
            s.store_voltage(79, ctx, nodes, Some(9), Some(21));
        }

        if ((s.b[538] && s.b[539]) && (!s.b[540])) {
            s.store_voltage(78, ctx, nodes, Some(2), Some(22));
            s.store_voltage(79, ctx, nodes, Some(2), Some(21));
        }

        if (s.b[538] && s.b[539]) {
            s.store_scalar(76, 1.0);
        }

        s.b[541] = (s.v[77] < 0.0);
        s.v[541] = if s.b[541] { 1.0 } else { 0.0 };

        if ((s.b[538] && s.b[539]) && s.b[541]) {
            s.store_scalar(76, (-1.0));
            s.store_mul(315, 76, 77);
        }

    }

    pub(super) fn stamp_reactive_block_24(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[538] && s.b[539]) && s.b[541]) {
            s.copy_ad(314, 79);
        }

        if ((s.b[538] && s.b[539]) && (!s.b[541])) {
            s.copy_ad(315, 77);
            s.copy_ad(314, 78);
        }

        if (s.b[538] && s.b[539]) {
            s.store_offset_sqrt_ad(316, A::offset(A::square(s.ad_value(315)), 0.01), (-0.1));
            s.store_offset_scaled(146, 316, p.p205, (1.0 + p.p204));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_sub_ad(88, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p201)), (((((-1.0)) * (p.p201))) + (p.p198))), A::div_scaled_inputs(s.ad_value(316), (p.p207 * p.p206), A::sqrt_square_offset(s.ad_value(316), (p.p207 * p.p207)), 1.0));
            s.store_scalar(307, (p.p9 / p.p199));
            s.store_div_from_scalar_scaled_mul(136, p.p200, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p197), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 314, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(314), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);
            s.store_sub(306, 160, 88);
            s.store_div_scaled_inputs_indices(84, 307, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 307, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 306, 306, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 306, 1.0, 83, 2.0);
        }

        s.b[542] = (s.v[136] < 200.0);
        s.v[542] = if s.b[542] { 1.0 } else { 0.0 };

        if ((s.b[538] && s.b[539]) && s.b[542]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[538] && s.b[539]) && (!s.b[542])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[538] && s.b[539]) {
            s.store_sub_div_rhs_indices(100, 306, 153, 99);
        }

        s.b[543] = ((((s.v[100] - s.v[306])) as f64).abs() > 1e-19);
        s.v[543] = if s.b[543] { 1.0 } else { 0.0 };

        if ((s.b[538] && s.b[539]) && s.b[543]) {
            s.store_sub(101, 306, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p208);
            s.store_scaled_mul(103, 136, 90, p.p209);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if ((s.b[538] && s.b[539]) && s.b[543]) {
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(105))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if ((s.b[538] && s.b[539]) && s.b[543]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 306, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p208, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p209, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if ((s.b[538] && s.b[539]) && s.b[543]) {
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(119))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if ((s.b[538] && s.b[539]) && s.b[543]) {
            s.store_scaled_mul(121, 136, 137, p.p208);
            s.store_scaled_mul(122, 136, 137, p.p209);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.copy_ad(308, 128);
        }

        if ((s.b[538] && s.b[539]) && (!s.b[543])) {
            s.copy_ad(308, 100);
        }

        if (s.b[538] && s.b[539]) {
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p202);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p203);
            s.store_mul_scaled_abs_ad_rhs(136, 307, 1.0 / (p.p9), A::sub(s.ad_value(306), s.ad_value(308)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(308)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(90, 306, 306, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p200, 136, p.p200, 90, 1.0, 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(315), s.ad_value(85)), p.p18);
            s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));
            s.store_mul(86, 315, 90);
            s.store_sub(39, 306, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[544] = (s.v[136] < 200.0);
        s.v[544] = if s.b[544] { 1.0 } else { 0.0 };

        if ((s.b[538] && s.b[539]) && s.b[544]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[538] && s.b[539]) && (!s.b[544])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[538] && s.b[539]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[545] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[545] = if s.b[545] { 1.0 } else { 0.0 };

        if ((s.b[538] && s.b[539]) && s.b[545]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p208);
            s.store_scaled_mul(103, 136, 90, p.p209);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if ((s.b[538] && s.b[539]) && s.b[545]) {
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(105))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if ((s.b[538] && s.b[539]) && s.b[545]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
        }

    }

    pub(super) fn stamp_reactive_block_25(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[538] && s.b[539]) && s.b[545]) {
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 130, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_mul_scaled_powf_rhs(116, 136, p.p208, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p209, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if ((s.b[538] && s.b[539]) && s.b[545]) {
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(119))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if ((s.b[538] && s.b[539]) && s.b[545]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p208, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p209, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(309, 128, 86);
        }

        if ((s.b[538] && s.b[539]) && (!s.b[545])) {
            s.store_add(309, 100, 86);
        }

        if (s.b[538] && s.b[539]) {
            s.store_scaled_add(310, 308, 309, 0.5);
            s.store_sub(311, 309, 308);
            s.store_mul_add_scaled_inputs3_offset_rhs(135, 311, s.ad_value(306), 1.0, s.ad_value(310), (-1.0), s.ad_value(83), 1.0, 0.0);
            s.store_mul_scaled_abs_ad_rhs(136, 307, 1.0 / (p.p9), A::sub(s.ad_value(306), s.ad_value(310)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);
            s.store_scaled_mul(96, 95, 307, (p.p4 * (p.p5 * 1.0 / (p.p200))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(316), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(311), (p.p25 * p.p25), s.ad_value(311)), 1.0);
            s.store_div(93, 98, 92);
            s.store_sub(90, 309, 308);
            s.store_add_scaled_inputs3_indices(91, 306, 1.0, 83, 1.0, 310, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 307, s.ad_value(306), ((p.p4 * p.p5) * p.p200), s.ad_value(310), (((-1.0)) * (((p.p4 * p.p5) * p.p200))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p200), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p245) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p244, 1.0);
            s.store_div_from_scalar(190, p.p243, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p199);
            s.store_mul_add_scaled_inputs3_offset_rhs(312, 191, s.ad_value(306), ((p.p4 * p.p5) * p.p200), s.ad_value(310), (((-1.0)) * (((p.p4 * p.p5) * p.p200))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p200), 0.0);
            s.store_add_scaled_inputs3_indices(136, 306, 1.0, 83, 1.0, 310, -1.0);
            s.store_add_scaled_inputs(90, 308, 0.3333333333333333, 309, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(311)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(311)), 311, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(313, 191, 306, (-(((p.p4 * p.p200) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p200) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p200) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p200) * p.p5) * 0.5)));
        }

        s.b[546] = (s.v[76] < 0.0);
        s.v[546] = if s.b[546] { 1.0 } else { 0.0 };

        if ((s.b[538] && s.b[539]) && s.b[546]) {
            s.store_sub_scaled_inputs(313, 312, (-1.0), 313, 1.0);
        }

        if (s.b[538] && (!s.b[539])) {
            s.store_scalar(312, 0.0);
            s.store_scalar(313, 0.0);
        }

        s.b[547] = (p.p157 != 0.0);
        s.v[547] = if s.b[547] { 1.0 } else { 0.0 };

        s.b[548] = (p.p157 == 1.0);
        s.v[548] = if s.b[548] { 1.0 } else { 0.0 };

        if (((!s.b[538]) && s.b[547]) && s.b[548]) {
            s.store_voltage(78, ctx, nodes, Some(9), Some(8));
        }

        if (((!s.b[538]) && s.b[547]) && (!s.b[548])) {
            s.store_voltage(78, ctx, nodes, Some(2), Some(8));
        }

        if ((!s.b[538]) && s.b[547]) {
            s.copy_ad(314, 78);
            s.store_scalar(146, (1.0 + p.p204));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_offset_scaled(88, 82, ((1.0 / (s.v[35])) * (p.p201)), (((((-1.0)) * (p.p201))) + (p.p198)));
            s.store_scalar(307, (p.p9 / p.p199));
            s.store_div_from_scalar_scaled_mul(136, p.p200, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p197), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 314, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(314), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);
            s.store_sub(306, 160, 88);
            s.store_div_scaled_inputs_indices(84, 307, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 307, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 306, 306, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 306, 1.0, 83, 2.0);
        }

        s.b[549] = (s.v[136] < 200.0);
        s.v[549] = if s.b[549] { 1.0 } else { 0.0 };

        if (((!s.b[538]) && s.b[547]) && s.b[549]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[538]) && s.b[547]) && (!s.b[549])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[538]) && s.b[547]) {
            s.store_sub_div_rhs_indices(100, 306, 153, 99);
        }

        s.b[550] = ((((s.v[100] - s.v[306])) as f64).abs() > 1e-19);
        s.v[550] = if s.b[550] { 1.0 } else { 0.0 };

        if (((!s.b[538]) && s.b[547]) && s.b[550]) {
            s.store_sub(101, 306, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p208);
            s.store_scaled_mul(103, 136, 90, p.p209);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if (((!s.b[538]) && s.b[547]) && s.b[550]) {
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(105))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if (((!s.b[538]) && s.b[547]) && s.b[550]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 306, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p208, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p209, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

    }

    pub(super) fn stamp_reactive_block_26(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[538]) && s.b[547]) && s.b[550]) {
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(119))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if (((!s.b[538]) && s.b[547]) && s.b[550]) {
            s.store_scaled_mul(121, 136, 137, p.p208);
            s.store_scaled_mul(122, 136, 137, p.p209);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.copy_ad(308, 128);
        }

        if (((!s.b[538]) && s.b[547]) && (!s.b[550])) {
            s.copy_ad(308, 100);
        }

        if ((!s.b[538]) && s.b[547]) {
            s.store_scalar(315, 0.0);
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p202);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p203);
            s.store_mul_scaled_abs_ad_rhs(136, 307, 1.0 / (p.p9), A::sub(s.ad_value(306), s.ad_value(308)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(308)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(90, 306, 306, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p200, 136, p.p200, 90, 1.0, 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(315), s.ad_value(85)), p.p18);
            s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));
            s.store_mul(86, 315, 90);
            s.store_sub(39, 306, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[551] = (s.v[136] < 200.0);
        s.v[551] = if s.b[551] { 1.0 } else { 0.0 };

        if (((!s.b[538]) && s.b[547]) && s.b[551]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[538]) && s.b[547]) && (!s.b[551])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[538]) && s.b[547]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[552] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[552] = if s.b[552] { 1.0 } else { 0.0 };

        if (((!s.b[538]) && s.b[547]) && s.b[552]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p208);
            s.store_scaled_mul(103, 136, 90, p.p209);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if (((!s.b[538]) && s.b[547]) && s.b[552]) {
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(105))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if (((!s.b[538]) && s.b[547]) && s.b[552]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 130, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_mul_scaled_powf_rhs(116, 136, p.p208, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p209, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if (((!s.b[538]) && s.b[547]) && s.b[552]) {
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(119))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if (((!s.b[538]) && s.b[547]) && s.b[552]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p208, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p209, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(309, 128, 86);
        }

        if (((!s.b[538]) && s.b[547]) && (!s.b[552])) {
            s.store_add(309, 100, 86);
        }

        if ((!s.b[538]) && s.b[547]) {
            s.store_scaled_add(310, 308, 309, 0.5);
            s.store_sub(311, 309, 308);
            s.store_sub(90, 309, 308);
            s.store_add_scaled_inputs3_indices(91, 306, 1.0, 83, 1.0, 310, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 307, s.ad_value(306), ((p.p4 * p.p5) * p.p200), s.ad_value(310), (((-1.0)) * (((p.p4 * p.p5) * p.p200))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p200), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p245) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p244, 1.0);
            s.store_div_from_scalar(190, p.p243, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p199);
            s.store_mul_add_scaled_inputs3_offset_rhs(312, 191, s.ad_value(306), ((p.p4 * p.p5) * p.p200), s.ad_value(310), (((-1.0)) * (((p.p4 * p.p5) * p.p200))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p200), 0.0);
            s.store_add_scaled_inputs3_indices(136, 306, 1.0, 83, 1.0, 310, -1.0);
            s.store_add_scaled_inputs(90, 308, 0.3333333333333333, 309, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(311)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(311)), 311, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(313, 191, 306, (-(((p.p4 * p.p200) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p200) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p200) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p200) * p.p5) * 0.5)));
        }

        if ((!s.b[538]) && (!s.b[547])) {
            s.store_scalar(312, 0.0);
            s.store_scalar(313, 0.0);
        }

        s.b[558] = (p.p255 == 2.0);
        s.v[558] = if s.b[558] { 1.0 } else { 0.0 };

        if s.b[558] {
            s.store_scaled_voltage(162, ctx, nodes, Some(10), Some(2), ((p.p4 * p.p5) * p.p210));
            s.store_div_scaled_inputs(168, A::voltage(ctx, nodes, Some(0), Some(2)), p.p214, A::sqrt(A::offset(A::square(A::voltage(ctx, nodes, Some(0), Some(2))), (p.p214 * p.p214))), 1.0);
            s.store_scalar(169, (p.p213).min((p.p211 / (2.0 * p.p214))));
            s.store_sub_from_scalar_scaled_mul(167, ((p.p4 * p.p5) * p.p211), 169, 168, (p.p4 * p.p5));
            s.store_mul_voltage_ad(163, A::max_with_scalar(s.ad_value(167), 0.0), ctx, nodes, Some(10), Some(0));
        }

        if (!s.b[558]) {
            s.store_scaled_voltage(162, ctx, nodes, Some(1), Some(2), ((p.p4 * p.p5) * p.p210));
            s.store_div_scaled_inputs(168, A::voltage(ctx, nodes, Some(0), Some(2)), p.p214, A::sqrt(A::offset(A::square(A::voltage(ctx, nodes, Some(0), Some(2))), (p.p214 * p.p214))), 1.0);
            s.store_scalar(169, (p.p213).min((p.p211 / (2.0 * p.p214))));
            s.store_sub_from_scalar_scaled_mul(167, ((p.p4 * p.p5) * p.p211), 169, 168, (p.p4 * p.p5));
            s.store_mul_voltage_ad(163, A::max_with_scalar(s.ad_value(167), 0.0), ctx, nodes, Some(1), Some(0));
        }

        s.store_scaled_voltage(164, ctx, nodes, Some(0), Some(2), ((p.p4 * p.p5) * p.p212));

    }

    pub(super) fn stamp_reactive_block_27(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.store_scaled_voltage(219, ctx, nodes, Some(3), Some(0), ((p.p4 * p.p5) * p.p215));

        s.store_scaled_voltage(220, ctx, nodes, Some(3), Some(2), ((p.p4 * p.p5) * p.p216));

        s.store_scaled_voltage(221, ctx, nodes, Some(3), Some(1), ((p.p4 * p.p5) * p.p217));

        s.store_scale_ad(377, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p281), p.p277);

        s.store_scale_ad(378, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p282), p.p278);

        s.store_scale(137, 378, (p.p4 * p.p5));

        s.store_scale(137, 377, (p.p4 * p.p5));

        s.b[569] = (p.p255 == 2.0);
        s.v[569] = if s.b[569] { 1.0 } else { 0.0 };

        s.b[570] = (p.p149 == 0.0);
        s.v[570] = if s.b[570] { 1.0 } else { 0.0 };

        s.b[571] = (p.p150 != 0.0);
        s.v[571] = if s.b[571] { 1.0 } else { 0.0 };

        s.b[572] = (p.p150 == 1.0);
        s.v[572] = if s.b[572] { 1.0 } else { 0.0 };

        s.b[573] = (p.p150 != 0.0);
        s.v[573] = if s.b[573] { 1.0 } else { 0.0 };

        s.b[574] = (p.p150 == 1.0);
        s.v[574] = if s.b[574] { 1.0 } else { 0.0 };

        s.b[575] = (p.p149 == 0.0);
        s.v[575] = if s.b[575] { 1.0 } else { 0.0 };

        s.b[576] = (p.p151 != 0.0);
        s.v[576] = if s.b[576] { 1.0 } else { 0.0 };

        s.b[577] = (p.p151 == 1.0);
        s.v[577] = if s.b[577] { 1.0 } else { 0.0 };

        s.b[578] = (p.p151 != 0.0);
        s.v[578] = if s.b[578] { 1.0 } else { 0.0 };

        s.b[579] = (p.p151 == 1.0);
        s.v[579] = if s.b[579] { 1.0 } else { 0.0 };

        s.b[580] = (p.p149 == 0.0);
        s.v[580] = if s.b[580] { 1.0 } else { 0.0 };

        s.b[581] = (p.p152 != 0.0);
        s.v[581] = if s.b[581] { 1.0 } else { 0.0 };

        s.b[582] = (p.p152 == 1.0);
        s.v[582] = if s.b[582] { 1.0 } else { 0.0 };

        s.b[583] = (p.p152 != 0.0);
        s.v[583] = if s.b[583] { 1.0 } else { 0.0 };

        s.b[584] = (p.p152 == 1.0);
        s.v[584] = if s.b[584] { 1.0 } else { 0.0 };

        s.b[585] = (p.p149 == 0.0);
        s.v[585] = if s.b[585] { 1.0 } else { 0.0 };

        s.b[586] = (p.p153 != 0.0);
        s.v[586] = if s.b[586] { 1.0 } else { 0.0 };

        s.b[587] = (p.p153 == 1.0);
        s.v[587] = if s.b[587] { 1.0 } else { 0.0 };

        s.b[588] = (p.p153 != 0.0);
        s.v[588] = if s.b[588] { 1.0 } else { 0.0 };

        s.b[589] = (p.p153 == 1.0);
        s.v[589] = if s.b[589] { 1.0 } else { 0.0 };

        s.b[590] = (p.p149 == 0.0);
        s.v[590] = if s.b[590] { 1.0 } else { 0.0 };

        s.b[591] = (p.p154 != 0.0);
        s.v[591] = if s.b[591] { 1.0 } else { 0.0 };

        s.b[592] = (p.p154 == 1.0);
        s.v[592] = if s.b[592] { 1.0 } else { 0.0 };

        s.b[593] = (p.p154 != 0.0);
        s.v[593] = if s.b[593] { 1.0 } else { 0.0 };

        s.b[594] = (p.p154 == 1.0);
        s.v[594] = if s.b[594] { 1.0 } else { 0.0 };

        s.b[595] = (p.p149 == 0.0);
        s.v[595] = if s.b[595] { 1.0 } else { 0.0 };

        s.b[596] = (p.p155 != 0.0);
        s.v[596] = if s.b[596] { 1.0 } else { 0.0 };

        s.b[597] = (p.p155 == 1.0);
        s.v[597] = if s.b[597] { 1.0 } else { 0.0 };

        s.b[598] = (p.p155 != 0.0);
        s.v[598] = if s.b[598] { 1.0 } else { 0.0 };

        s.b[599] = (p.p155 == 1.0);
        s.v[599] = if s.b[599] { 1.0 } else { 0.0 };

        s.b[600] = (p.p149 == 0.0);
        s.v[600] = if s.b[600] { 1.0 } else { 0.0 };

        s.b[601] = (p.p156 != 0.0);
        s.v[601] = if s.b[601] { 1.0 } else { 0.0 };

        s.b[602] = (p.p156 == 1.0);
        s.v[602] = if s.b[602] { 1.0 } else { 0.0 };

        s.b[603] = (p.p156 != 0.0);
        s.v[603] = if s.b[603] { 1.0 } else { 0.0 };

        s.b[604] = (p.p156 == 1.0);
        s.v[604] = if s.b[604] { 1.0 } else { 0.0 };

        s.b[605] = (p.p149 == 0.0);
        s.v[605] = if s.b[605] { 1.0 } else { 0.0 };

        s.b[606] = (p.p157 != 0.0);
        s.v[606] = if s.b[606] { 1.0 } else { 0.0 };

        s.b[607] = (p.p157 == 1.0);
        s.v[607] = if s.b[607] { 1.0 } else { 0.0 };

        s.b[608] = (p.p157 != 0.0);
        s.v[608] = if s.b[608] { 1.0 } else { 0.0 };

        s.b[609] = (p.p157 == 1.0);
        s.v[609] = if s.b[609] { 1.0 } else { 0.0 };

        s.store_sub_from_scalar_ad(195, p.p222, A::mul(A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p227)), (((((-1.0)) * (p.p227))) + (p.p220))), A::voltage(ctx, nodes, Some(0), Some(2))));

        s.store_add_scaled_inputs3_offset_mixed_iia(195, 195, (p.p4 * p.p5), 195, ((-0.5) * (p.p4 * p.p5)), A::sqrt_offset_square_offset(s.ad_value(195), (-1e-25), p.p221), ((-(-0.5)) * (p.p4 * p.p5)), ((1e-25 + ((-0.5) * 1e-25)) * (p.p4 * p.p5)));

        s.store_scaled_add_offset_sqrt_square_offset_ad(136, A::sub_from_scalar(p.p218, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p226)), (((-1.0)) * (p.p226)))), 1e-18, (-1e-18), ((0.25 * 1e-19) * 1e-19), 0.5);

        s.store_mul_scaled_voltage(196, 136, (p.p4 * p.p5), ctx, nodes, Some(9), Some(2));

        s.store_scaled_voltage(197, ctx, nodes, Some(2), Some(0), ((p.p4 * p.p5) * p.p219));

        s.store_offset_scaled_ad(136, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p225)), (((-1.0)) * (p.p225))), (-(1.0 - { let limited_exp_arg = ((-((p.p229) as f64).ln()) / p.p228); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })), ((p.p224) * ((1.0 - { let limited_exp_arg = ((-((p.p229) as f64).ln()) / p.p228); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))));

        s.store_div_scaled_inputs2_mixed_iai(90, 136, 1.0, A::voltage(ctx, nodes, Some(2), Some(0)), (-1.0), 36, 1.0);

        s.store_sqrt_offset_ad(91, A::mul_scaled_lhs(s.ad_value(90), p.p230, s.ad_value(90)), 1.92);

        s.store_scaled_add(137, 90, 91, 0.5);

        s.store_add_scaled_product_indices(106, 136, 1.0, 36, 137, (-1.0));

        s.store_ln_ad(192, A::sub_from_scalar(1.0, A::scale(s.ad_value(106), 1.0 / (p.p224))));

        s.store_mul_sub_from_scalar_lhs_scaled_ad(193, p.p224, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p225)), (((-1.0)) * (p.p225))), A::sub_from_scalar(1.0, A::limited_exp_scaled_input(s.ad_value(192), (1.0 - p.p228))), (p.p223 * 1.0 / ((1.0 - p.p228))));

        s.store_add_scaled_inputs3_mixed_iai(194, 193, (p.p4 * p.p5), A::voltage(ctx, nodes, Some(2), Some(0)), ((p.p229 * p.p223) * (p.p4 * p.p5)), 106, ((-(p.p229 * p.p223)) * (p.p4 * p.p5)));

        s.b[610] = ((p.p31 == 1.0) && (p.p32 > 0.0));
        s.v[610] = if s.b[610] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq8_e345, eq8_e345_d_n5, eq8_e345_d_n6,) = {
    if (s.b[388] && (!s.b[387])) {
        let eq8_e339: f64 = ((nv6 - nv5) / 10.0);
        let eq8_e339_d_n5: f64 = (-1.0 / 10.0);
        let eq8_e339_d_n6: f64 = (1.0 / 10.0);
        let eq8_e340: f64 = { let limited_exp_arg = eq8_e339; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let eq8_e340_d_n5: f64 = ({ let limited_exp_arg = eq8_e339; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * eq8_e339_d_n5);
        let eq8_e340_d_n6: f64 = ({ let limited_exp_arg = eq8_e339; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * eq8_e339_d_n6);
        let eq8_e342: f64 = (eq8_e340 - 1.0);
        let eq8_e343: f64 = (p.p99 * eq8_e342);
        let eq8_e343_d_n5: f64 = (p.p99 * eq8_e340_d_n5);
        let eq8_e343_d_n6: f64 = (p.p99 * eq8_e340_d_n6);
        (eq8_e343, eq8_e343_d_n5, eq8_e343_d_n6,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e345;
        stamper.stamp_current_node2_local(
            Some(6),
            Some(5),
            multiplicity * (eq8_value),
            5,
            multiplicity * (eq8_e345_d_n5),
            6,
            multiplicity * (eq8_e345_d_n6),
        );
        let (eq9_e355, eq9_e355_d_n5,) = {
    if (s.b[388] && (!s.b[387])) {
        let eq9_e352: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, (nv5 - 0.0));
        let eq9_e353: f64 = (p.p97 * eq9_e352);
        (eq9_e353, (p.p97 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e355;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (eq9_value),
            5,
            multiplicity * (eq9_e355_d_n5),
        );
        let (eq16_e415, eq16_e415_d_n0, eq16_e415_d_n1, eq16_e415_d_n2, eq16_e415_d_n3, eq16_e415_d_n4, eq16_e415_d_n5, eq16_e415_d_n6, eq16_e415_d_n7, eq16_e415_d_n8, eq16_e415_d_n9, eq16_e415_d_n10, eq16_e415_d_n11, eq16_e415_d_n12, eq16_e415_d_n13, eq16_e415_d_n14, eq16_e415_d_n15, eq16_e415_d_n16, eq16_e415_d_n17, eq16_e415_d_n18, eq16_e415_d_n19, eq16_e415_d_n20, eq16_e415_d_n21, eq16_e415_d_n22, eq16_e415_d_b0, eq16_e415_d_b1, eq16_e415_d_b2, eq16_e415_d_b3, eq16_e415_d_b4, eq16_e415_d_b5, eq16_e415_d_b6, eq16_e415_d_b7, eq16_e415_d_b8, eq16_e415_d_b9, eq16_e415_d_b10, eq16_e415_d_b11, eq16_e415_d_b12, eq16_e415_d_b13, eq16_e415_d_b14, eq16_e415_d_b15, eq16_e415_d_b16, eq16_e415_d_b17, eq16_e415_d_b18, eq16_e415_d_b19, eq16_e415_d_b20, eq16_e415_d_b21, eq16_e415_d_b22, eq16_e415_d_b23, eq16_e415_d_b24, eq16_e415_d_b25, eq16_e415_d_b26, eq16_e415_d_b27, eq16_e415_d_b28, eq16_e415_d_b29, eq16_e415_d_b30, eq16_e415_d_b31, eq16_e415_d_b32, eq16_e415_d_b33, eq16_e415_d_b34, eq16_e415_d_b35, eq16_e415_d_b36, eq16_e415_d_b37, eq16_e415_d_b38, eq16_e415_d_b39, eq16_e415_d_b40, eq16_e415_d_b41, eq16_e415_d_b42, eq16_e415_d_b43, eq16_e415_d_b44, eq16_e415_d_b45, eq16_e415_d_b46, eq16_e415_d_b47, eq16_e415_d_b48, eq16_e415_d_b49, eq16_e415_d_b50, eq16_e415_d_b51, eq16_e415_d_b52, eq16_e415_d_b53, eq16_e415_d_b54,) = {
    if (s.b[389] && (!(s.b[387] || s.b[388]))) {
        let eq16_e411: f64 = (-1.0);
        let eq16_e413: f64 = (eq16_e411 * s.v[208]);
        let eq16_e413_d_n0: f64 = (eq16_e411 * s.dn[208][0]);
        let eq16_e413_d_n1: f64 = (eq16_e411 * s.dn[208][1]);
        let eq16_e413_d_n2: f64 = (eq16_e411 * s.dn[208][2]);
        let eq16_e413_d_n3: f64 = (eq16_e411 * s.dn[208][3]);
        let eq16_e413_d_n4: f64 = (eq16_e411 * s.dn[208][4]);
        let eq16_e413_d_n5: f64 = (eq16_e411 * s.dn[208][5]);
        let eq16_e413_d_n6: f64 = (eq16_e411 * s.dn[208][6]);
        let eq16_e413_d_n7: f64 = (eq16_e411 * s.dn[208][7]);
        let eq16_e413_d_n8: f64 = (eq16_e411 * s.dn[208][8]);
        let eq16_e413_d_n9: f64 = (eq16_e411 * s.dn[208][9]);
        let eq16_e413_d_n10: f64 = (eq16_e411 * s.dn[208][10]);
        let eq16_e413_d_n11: f64 = (eq16_e411 * s.dn[208][11]);
        let eq16_e413_d_n12: f64 = (eq16_e411 * s.dn[208][12]);
        let eq16_e413_d_n13: f64 = (eq16_e411 * s.dn[208][13]);
        let eq16_e413_d_n14: f64 = (eq16_e411 * s.dn[208][14]);
        let eq16_e413_d_n15: f64 = (eq16_e411 * s.dn[208][15]);
        let eq16_e413_d_n16: f64 = (eq16_e411 * s.dn[208][16]);
        let eq16_e413_d_n17: f64 = (eq16_e411 * s.dn[208][17]);
        let eq16_e413_d_n18: f64 = (eq16_e411 * s.dn[208][18]);
        let eq16_e413_d_n19: f64 = (eq16_e411 * s.dn[208][19]);
        let eq16_e413_d_n20: f64 = (eq16_e411 * s.dn[208][20]);
        let eq16_e413_d_n21: f64 = (eq16_e411 * s.dn[208][21]);
        let eq16_e413_d_n22: f64 = (eq16_e411 * s.dn[208][22]);
        let eq16_e413_d_b0: f64 = (eq16_e411 * s.db[208][0]);
        let eq16_e413_d_b1: f64 = (eq16_e411 * s.db[208][1]);
        let eq16_e413_d_b2: f64 = (eq16_e411 * s.db[208][2]);
        let eq16_e413_d_b3: f64 = (eq16_e411 * s.db[208][3]);
        let eq16_e413_d_b4: f64 = (eq16_e411 * s.db[208][4]);
        let eq16_e413_d_b5: f64 = (eq16_e411 * s.db[208][5]);
        let eq16_e413_d_b6: f64 = (eq16_e411 * s.db[208][6]);
        let eq16_e413_d_b7: f64 = (eq16_e411 * s.db[208][7]);
        let eq16_e413_d_b8: f64 = (eq16_e411 * s.db[208][8]);
        let eq16_e413_d_b9: f64 = (eq16_e411 * s.db[208][9]);
        let eq16_e413_d_b10: f64 = (eq16_e411 * s.db[208][10]);
        let eq16_e413_d_b11: f64 = (eq16_e411 * s.db[208][11]);
        let eq16_e413_d_b12: f64 = (eq16_e411 * s.db[208][12]);
        let eq16_e413_d_b13: f64 = (eq16_e411 * s.db[208][13]);
        let eq16_e413_d_b14: f64 = (eq16_e411 * s.db[208][14]);
        let eq16_e413_d_b15: f64 = (eq16_e411 * s.db[208][15]);
        let eq16_e413_d_b16: f64 = (eq16_e411 * s.db[208][16]);
        let eq16_e413_d_b17: f64 = (eq16_e411 * s.db[208][17]);
        let eq16_e413_d_b18: f64 = (eq16_e411 * s.db[208][18]);
        let eq16_e413_d_b19: f64 = (eq16_e411 * s.db[208][19]);
        let eq16_e413_d_b20: f64 = (eq16_e411 * s.db[208][20]);
        let eq16_e413_d_b21: f64 = (eq16_e411 * s.db[208][21]);
        let eq16_e413_d_b22: f64 = (eq16_e411 * s.db[208][22]);
        let eq16_e413_d_b23: f64 = (eq16_e411 * s.db[208][23]);
        let eq16_e413_d_b24: f64 = (eq16_e411 * s.db[208][24]);
        let eq16_e413_d_b25: f64 = (eq16_e411 * s.db[208][25]);
        let eq16_e413_d_b26: f64 = (eq16_e411 * s.db[208][26]);
        let eq16_e413_d_b27: f64 = (eq16_e411 * s.db[208][27]);
        let eq16_e413_d_b28: f64 = (eq16_e411 * s.db[208][28]);
        let eq16_e413_d_b29: f64 = (eq16_e411 * s.db[208][29]);
        let eq16_e413_d_b30: f64 = (eq16_e411 * s.db[208][30]);
        let eq16_e413_d_b31: f64 = (eq16_e411 * s.db[208][31]);
        let eq16_e413_d_b32: f64 = (eq16_e411 * s.db[208][32]);
        let eq16_e413_d_b33: f64 = (eq16_e411 * s.db[208][33]);
        let eq16_e413_d_b34: f64 = (eq16_e411 * s.db[208][34]);
        let eq16_e413_d_b35: f64 = (eq16_e411 * s.db[208][35]);
        let eq16_e413_d_b36: f64 = (eq16_e411 * s.db[208][36]);
        let eq16_e413_d_b37: f64 = (eq16_e411 * s.db[208][37]);
        let eq16_e413_d_b38: f64 = (eq16_e411 * s.db[208][38]);
        let eq16_e413_d_b39: f64 = (eq16_e411 * s.db[208][39]);
        let eq16_e413_d_b40: f64 = (eq16_e411 * s.db[208][40]);
        let eq16_e413_d_b41: f64 = (eq16_e411 * s.db[208][41]);
        let eq16_e413_d_b42: f64 = (eq16_e411 * s.db[208][42]);
        let eq16_e413_d_b43: f64 = (eq16_e411 * s.db[208][43]);
        let eq16_e413_d_b44: f64 = (eq16_e411 * s.db[208][44]);
        let eq16_e413_d_b45: f64 = (eq16_e411 * s.db[208][45]);
        let eq16_e413_d_b46: f64 = (eq16_e411 * s.db[208][46]);
        let eq16_e413_d_b47: f64 = (eq16_e411 * s.db[208][47]);
        let eq16_e413_d_b48: f64 = (eq16_e411 * s.db[208][48]);
        let eq16_e413_d_b49: f64 = (eq16_e411 * s.db[208][49]);
        let eq16_e413_d_b50: f64 = (eq16_e411 * s.db[208][50]);
        let eq16_e413_d_b51: f64 = (eq16_e411 * s.db[208][51]);
        let eq16_e413_d_b52: f64 = (eq16_e411 * s.db[208][52]);
        let eq16_e413_d_b53: f64 = (eq16_e411 * s.db[208][53]);
        let eq16_e413_d_b54: f64 = (eq16_e411 * s.db[208][54]);
        (eq16_e413, eq16_e413_d_n0, eq16_e413_d_n1, eq16_e413_d_n2, eq16_e413_d_n3, eq16_e413_d_n4, eq16_e413_d_n5, eq16_e413_d_n6, eq16_e413_d_n7, eq16_e413_d_n8, eq16_e413_d_n9, eq16_e413_d_n10, eq16_e413_d_n11, eq16_e413_d_n12, eq16_e413_d_n13, eq16_e413_d_n14, eq16_e413_d_n15, eq16_e413_d_n16, eq16_e413_d_n17, eq16_e413_d_n18, eq16_e413_d_n19, eq16_e413_d_n20, eq16_e413_d_n21, eq16_e413_d_n22, eq16_e413_d_b0, eq16_e413_d_b1, eq16_e413_d_b2, eq16_e413_d_b3, eq16_e413_d_b4, eq16_e413_d_b5, eq16_e413_d_b6, eq16_e413_d_b7, eq16_e413_d_b8, eq16_e413_d_b9, eq16_e413_d_b10, eq16_e413_d_b11, eq16_e413_d_b12, eq16_e413_d_b13, eq16_e413_d_b14, eq16_e413_d_b15, eq16_e413_d_b16, eq16_e413_d_b17, eq16_e413_d_b18, eq16_e413_d_b19, eq16_e413_d_b20, eq16_e413_d_b21, eq16_e413_d_b22, eq16_e413_d_b23, eq16_e413_d_b24, eq16_e413_d_b25, eq16_e413_d_b26, eq16_e413_d_b27, eq16_e413_d_b28, eq16_e413_d_b29, eq16_e413_d_b30, eq16_e413_d_b31, eq16_e413_d_b32, eq16_e413_d_b33, eq16_e413_d_b34, eq16_e413_d_b35, eq16_e413_d_b36, eq16_e413_d_b37, eq16_e413_d_b38, eq16_e413_d_b39, eq16_e413_d_b40, eq16_e413_d_b41, eq16_e413_d_b42, eq16_e413_d_b43, eq16_e413_d_b44, eq16_e413_d_b45, eq16_e413_d_b46, eq16_e413_d_b47, eq16_e413_d_b48, eq16_e413_d_b49, eq16_e413_d_b50, eq16_e413_d_b51, eq16_e413_d_b52, eq16_e413_d_b53, eq16_e413_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e415;
        let eq16_node_derivatives: [f64; 23] = [eq16_e415_d_n0, eq16_e415_d_n1, eq16_e415_d_n2, eq16_e415_d_n3, eq16_e415_d_n4, eq16_e415_d_n5, eq16_e415_d_n6, eq16_e415_d_n7, eq16_e415_d_n8, eq16_e415_d_n9, eq16_e415_d_n10, eq16_e415_d_n11, eq16_e415_d_n12, eq16_e415_d_n13, eq16_e415_d_n14, eq16_e415_d_n15, eq16_e415_d_n16, eq16_e415_d_n17, eq16_e415_d_n18, eq16_e415_d_n19, eq16_e415_d_n20, eq16_e415_d_n21, eq16_e415_d_n22];
        let eq16_branch_derivatives: [f64; 55] = [eq16_e415_d_b0, eq16_e415_d_b1, eq16_e415_d_b2, eq16_e415_d_b3, eq16_e415_d_b4, eq16_e415_d_b5, eq16_e415_d_b6, eq16_e415_d_b7, eq16_e415_d_b8, eq16_e415_d_b9, eq16_e415_d_b10, eq16_e415_d_b11, eq16_e415_d_b12, eq16_e415_d_b13, eq16_e415_d_b14, eq16_e415_d_b15, eq16_e415_d_b16, eq16_e415_d_b17, eq16_e415_d_b18, eq16_e415_d_b19, eq16_e415_d_b20, eq16_e415_d_b21, eq16_e415_d_b22, eq16_e415_d_b23, eq16_e415_d_b24, eq16_e415_d_b25, eq16_e415_d_b26, eq16_e415_d_b27, eq16_e415_d_b28, eq16_e415_d_b29, eq16_e415_d_b30, eq16_e415_d_b31, eq16_e415_d_b32, eq16_e415_d_b33, eq16_e415_d_b34, eq16_e415_d_b35, eq16_e415_d_b36, eq16_e415_d_b37, eq16_e415_d_b38, eq16_e415_d_b39, eq16_e415_d_b40, eq16_e415_d_b41, eq16_e415_d_b42, eq16_e415_d_b43, eq16_e415_d_b44, eq16_e415_d_b45, eq16_e415_d_b46, eq16_e415_d_b47, eq16_e415_d_b48, eq16_e415_d_b49, eq16_e415_d_b50, eq16_e415_d_b51, eq16_e415_d_b52, eq16_e415_d_b53, eq16_e415_d_b54];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let (eq17_e427, eq17_e427_d_n5,) = {
    if (s.b[389] && (!(s.b[387] || s.b[388]))) {
        let eq17_e424: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, (nv5 - 0.0));
        let eq17_e425: f64 = (p.p110 * eq17_e424);
        (eq17_e425, (p.p110 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e427;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (eq17_value),
            5,
            multiplicity * (eq17_e427_d_n5),
        );
        let (eq20_e462, eq20_e462_d_n6,) = {
    if (s.b[389] && (!(s.b[387] || s.b[388]))) {
        let eq20_e459: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, (nv6 - 0.0));
        let eq20_e460: f64 = (p.p111 * eq20_e459);
        (eq20_e460, (p.p111 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e462;
        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * (eq20_value),
            6,
            multiplicity * (eq20_e462_d_n6),
        );
        let (eq27_e539, eq27_e539_d_n0, eq27_e539_d_n1, eq27_e539_d_n2, eq27_e539_d_n3, eq27_e539_d_n4, eq27_e539_d_n5, eq27_e539_d_n6, eq27_e539_d_n7, eq27_e539_d_n8, eq27_e539_d_n9, eq27_e539_d_n10, eq27_e539_d_n11, eq27_e539_d_n12, eq27_e539_d_n13, eq27_e539_d_n14, eq27_e539_d_n15, eq27_e539_d_n16, eq27_e539_d_n17, eq27_e539_d_n18, eq27_e539_d_n19, eq27_e539_d_n20, eq27_e539_d_n21, eq27_e539_d_n22, eq27_e539_d_b0, eq27_e539_d_b1, eq27_e539_d_b2, eq27_e539_d_b3, eq27_e539_d_b4, eq27_e539_d_b5, eq27_e539_d_b6, eq27_e539_d_b7, eq27_e539_d_b8, eq27_e539_d_b9, eq27_e539_d_b10, eq27_e539_d_b11, eq27_e539_d_b12, eq27_e539_d_b13, eq27_e539_d_b14, eq27_e539_d_b15, eq27_e539_d_b16, eq27_e539_d_b17, eq27_e539_d_b18, eq27_e539_d_b19, eq27_e539_d_b20, eq27_e539_d_b21, eq27_e539_d_b22, eq27_e539_d_b23, eq27_e539_d_b24, eq27_e539_d_b25, eq27_e539_d_b26, eq27_e539_d_b27, eq27_e539_d_b28, eq27_e539_d_b29, eq27_e539_d_b30, eq27_e539_d_b31, eq27_e539_d_b32, eq27_e539_d_b33, eq27_e539_d_b34, eq27_e539_d_b35, eq27_e539_d_b36, eq27_e539_d_b37, eq27_e539_d_b38, eq27_e539_d_b39, eq27_e539_d_b40, eq27_e539_d_b41, eq27_e539_d_b42, eq27_e539_d_b43, eq27_e539_d_b44, eq27_e539_d_b45, eq27_e539_d_b46, eq27_e539_d_b47, eq27_e539_d_b48, eq27_e539_d_b49, eq27_e539_d_b50, eq27_e539_d_b51, eq27_e539_d_b52, eq27_e539_d_b53, eq27_e539_d_b54,) = {
    if (s.b[390] && (!((s.b[387] || s.b[388]) || s.b[389]))) {
        let eq27_e536: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, (nv5 - 0.0));
        let eq27_e537: f64 = (s.v[149] * eq27_e536);
        let eq27_e537_d_n0: f64 = (s.dn[149][0] * eq27_e536);
        let eq27_e537_d_n1: f64 = (s.dn[149][1] * eq27_e536);
        let eq27_e537_d_n2: f64 = (s.dn[149][2] * eq27_e536);
        let eq27_e537_d_n3: f64 = (s.dn[149][3] * eq27_e536);
        let eq27_e537_d_n4: f64 = (s.dn[149][4] * eq27_e536);
        let eq27_e537_d_n5: f64 = ((s.dn[149][5] * eq27_e536) + (s.v[149] * ddt_scale));
        let eq27_e537_d_n6: f64 = (s.dn[149][6] * eq27_e536);
        let eq27_e537_d_n7: f64 = (s.dn[149][7] * eq27_e536);
        let eq27_e537_d_n8: f64 = (s.dn[149][8] * eq27_e536);
        let eq27_e537_d_n9: f64 = (s.dn[149][9] * eq27_e536);
        let eq27_e537_d_n10: f64 = (s.dn[149][10] * eq27_e536);
        let eq27_e537_d_n11: f64 = (s.dn[149][11] * eq27_e536);
        let eq27_e537_d_n12: f64 = (s.dn[149][12] * eq27_e536);
        let eq27_e537_d_n13: f64 = (s.dn[149][13] * eq27_e536);
        let eq27_e537_d_n14: f64 = (s.dn[149][14] * eq27_e536);
        let eq27_e537_d_n15: f64 = (s.dn[149][15] * eq27_e536);
        let eq27_e537_d_n16: f64 = (s.dn[149][16] * eq27_e536);
        let eq27_e537_d_n17: f64 = (s.dn[149][17] * eq27_e536);
        let eq27_e537_d_n18: f64 = (s.dn[149][18] * eq27_e536);
        let eq27_e537_d_n19: f64 = (s.dn[149][19] * eq27_e536);
        let eq27_e537_d_n20: f64 = (s.dn[149][20] * eq27_e536);
        let eq27_e537_d_n21: f64 = (s.dn[149][21] * eq27_e536);
        let eq27_e537_d_n22: f64 = (s.dn[149][22] * eq27_e536);
        let eq27_e537_d_b0: f64 = (s.db[149][0] * eq27_e536);
        let eq27_e537_d_b1: f64 = (s.db[149][1] * eq27_e536);
        let eq27_e537_d_b2: f64 = (s.db[149][2] * eq27_e536);
        let eq27_e537_d_b3: f64 = (s.db[149][3] * eq27_e536);
        let eq27_e537_d_b4: f64 = (s.db[149][4] * eq27_e536);
        let eq27_e537_d_b5: f64 = (s.db[149][5] * eq27_e536);
        let eq27_e537_d_b6: f64 = (s.db[149][6] * eq27_e536);
        let eq27_e537_d_b7: f64 = (s.db[149][7] * eq27_e536);
        let eq27_e537_d_b8: f64 = (s.db[149][8] * eq27_e536);
        let eq27_e537_d_b9: f64 = (s.db[149][9] * eq27_e536);
        let eq27_e537_d_b10: f64 = (s.db[149][10] * eq27_e536);
        let eq27_e537_d_b11: f64 = (s.db[149][11] * eq27_e536);
        let eq27_e537_d_b12: f64 = (s.db[149][12] * eq27_e536);
        let eq27_e537_d_b13: f64 = (s.db[149][13] * eq27_e536);
        let eq27_e537_d_b14: f64 = (s.db[149][14] * eq27_e536);
        let eq27_e537_d_b15: f64 = (s.db[149][15] * eq27_e536);
        let eq27_e537_d_b16: f64 = (s.db[149][16] * eq27_e536);
        let eq27_e537_d_b17: f64 = (s.db[149][17] * eq27_e536);
        let eq27_e537_d_b18: f64 = (s.db[149][18] * eq27_e536);
        let eq27_e537_d_b19: f64 = (s.db[149][19] * eq27_e536);
        let eq27_e537_d_b20: f64 = (s.db[149][20] * eq27_e536);
        let eq27_e537_d_b21: f64 = (s.db[149][21] * eq27_e536);
        let eq27_e537_d_b22: f64 = (s.db[149][22] * eq27_e536);
        let eq27_e537_d_b23: f64 = (s.db[149][23] * eq27_e536);
        let eq27_e537_d_b24: f64 = (s.db[149][24] * eq27_e536);
        let eq27_e537_d_b25: f64 = (s.db[149][25] * eq27_e536);
        let eq27_e537_d_b26: f64 = (s.db[149][26] * eq27_e536);
        let eq27_e537_d_b27: f64 = (s.db[149][27] * eq27_e536);
        let eq27_e537_d_b28: f64 = (s.db[149][28] * eq27_e536);
        let eq27_e537_d_b29: f64 = (s.db[149][29] * eq27_e536);
        let eq27_e537_d_b30: f64 = (s.db[149][30] * eq27_e536);
        let eq27_e537_d_b31: f64 = (s.db[149][31] * eq27_e536);
        let eq27_e537_d_b32: f64 = (s.db[149][32] * eq27_e536);
        let eq27_e537_d_b33: f64 = (s.db[149][33] * eq27_e536);
        let eq27_e537_d_b34: f64 = (s.db[149][34] * eq27_e536);
        let eq27_e537_d_b35: f64 = (s.db[149][35] * eq27_e536);
        let eq27_e537_d_b36: f64 = (s.db[149][36] * eq27_e536);
        let eq27_e537_d_b37: f64 = (s.db[149][37] * eq27_e536);
        let eq27_e537_d_b38: f64 = (s.db[149][38] * eq27_e536);
        let eq27_e537_d_b39: f64 = (s.db[149][39] * eq27_e536);
        let eq27_e537_d_b40: f64 = (s.db[149][40] * eq27_e536);
        let eq27_e537_d_b41: f64 = (s.db[149][41] * eq27_e536);
        let eq27_e537_d_b42: f64 = (s.db[149][42] * eq27_e536);
        let eq27_e537_d_b43: f64 = (s.db[149][43] * eq27_e536);
        let eq27_e537_d_b44: f64 = (s.db[149][44] * eq27_e536);
        let eq27_e537_d_b45: f64 = (s.db[149][45] * eq27_e536);
        let eq27_e537_d_b46: f64 = (s.db[149][46] * eq27_e536);
        let eq27_e537_d_b47: f64 = (s.db[149][47] * eq27_e536);
        let eq27_e537_d_b48: f64 = (s.db[149][48] * eq27_e536);
        let eq27_e537_d_b49: f64 = (s.db[149][49] * eq27_e536);
        let eq27_e537_d_b50: f64 = (s.db[149][50] * eq27_e536);
        let eq27_e537_d_b51: f64 = (s.db[149][51] * eq27_e536);
        let eq27_e537_d_b52: f64 = (s.db[149][52] * eq27_e536);
        let eq27_e537_d_b53: f64 = (s.db[149][53] * eq27_e536);
        let eq27_e537_d_b54: f64 = (s.db[149][54] * eq27_e536);
        (eq27_e537, eq27_e537_d_n0, eq27_e537_d_n1, eq27_e537_d_n2, eq27_e537_d_n3, eq27_e537_d_n4, eq27_e537_d_n5, eq27_e537_d_n6, eq27_e537_d_n7, eq27_e537_d_n8, eq27_e537_d_n9, eq27_e537_d_n10, eq27_e537_d_n11, eq27_e537_d_n12, eq27_e537_d_n13, eq27_e537_d_n14, eq27_e537_d_n15, eq27_e537_d_n16, eq27_e537_d_n17, eq27_e537_d_n18, eq27_e537_d_n19, eq27_e537_d_n20, eq27_e537_d_n21, eq27_e537_d_n22, eq27_e537_d_b0, eq27_e537_d_b1, eq27_e537_d_b2, eq27_e537_d_b3, eq27_e537_d_b4, eq27_e537_d_b5, eq27_e537_d_b6, eq27_e537_d_b7, eq27_e537_d_b8, eq27_e537_d_b9, eq27_e537_d_b10, eq27_e537_d_b11, eq27_e537_d_b12, eq27_e537_d_b13, eq27_e537_d_b14, eq27_e537_d_b15, eq27_e537_d_b16, eq27_e537_d_b17, eq27_e537_d_b18, eq27_e537_d_b19, eq27_e537_d_b20, eq27_e537_d_b21, eq27_e537_d_b22, eq27_e537_d_b23, eq27_e537_d_b24, eq27_e537_d_b25, eq27_e537_d_b26, eq27_e537_d_b27, eq27_e537_d_b28, eq27_e537_d_b29, eq27_e537_d_b30, eq27_e537_d_b31, eq27_e537_d_b32, eq27_e537_d_b33, eq27_e537_d_b34, eq27_e537_d_b35, eq27_e537_d_b36, eq27_e537_d_b37, eq27_e537_d_b38, eq27_e537_d_b39, eq27_e537_d_b40, eq27_e537_d_b41, eq27_e537_d_b42, eq27_e537_d_b43, eq27_e537_d_b44, eq27_e537_d_b45, eq27_e537_d_b46, eq27_e537_d_b47, eq27_e537_d_b48, eq27_e537_d_b49, eq27_e537_d_b50, eq27_e537_d_b51, eq27_e537_d_b52, eq27_e537_d_b53, eq27_e537_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e539;
        let eq27_node_derivatives: [f64; 23] = [eq27_e539_d_n0, eq27_e539_d_n1, eq27_e539_d_n2, eq27_e539_d_n3, eq27_e539_d_n4, eq27_e539_d_n5, eq27_e539_d_n6, eq27_e539_d_n7, eq27_e539_d_n8, eq27_e539_d_n9, eq27_e539_d_n10, eq27_e539_d_n11, eq27_e539_d_n12, eq27_e539_d_n13, eq27_e539_d_n14, eq27_e539_d_n15, eq27_e539_d_n16, eq27_e539_d_n17, eq27_e539_d_n18, eq27_e539_d_n19, eq27_e539_d_n20, eq27_e539_d_n21, eq27_e539_d_n22];
        let eq27_branch_derivatives: [f64; 55] = [eq27_e539_d_b0, eq27_e539_d_b1, eq27_e539_d_b2, eq27_e539_d_b3, eq27_e539_d_b4, eq27_e539_d_b5, eq27_e539_d_b6, eq27_e539_d_b7, eq27_e539_d_b8, eq27_e539_d_b9, eq27_e539_d_b10, eq27_e539_d_b11, eq27_e539_d_b12, eq27_e539_d_b13, eq27_e539_d_b14, eq27_e539_d_b15, eq27_e539_d_b16, eq27_e539_d_b17, eq27_e539_d_b18, eq27_e539_d_b19, eq27_e539_d_b20, eq27_e539_d_b21, eq27_e539_d_b22, eq27_e539_d_b23, eq27_e539_d_b24, eq27_e539_d_b25, eq27_e539_d_b26, eq27_e539_d_b27, eq27_e539_d_b28, eq27_e539_d_b29, eq27_e539_d_b30, eq27_e539_d_b31, eq27_e539_d_b32, eq27_e539_d_b33, eq27_e539_d_b34, eq27_e539_d_b35, eq27_e539_d_b36, eq27_e539_d_b37, eq27_e539_d_b38, eq27_e539_d_b39, eq27_e539_d_b40, eq27_e539_d_b41, eq27_e539_d_b42, eq27_e539_d_b43, eq27_e539_d_b44, eq27_e539_d_b45, eq27_e539_d_b46, eq27_e539_d_b47, eq27_e539_d_b48, eq27_e539_d_b49, eq27_e539_d_b50, eq27_e539_d_b51, eq27_e539_d_b52, eq27_e539_d_b53, eq27_e539_d_b54];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let (eq35_e633, eq35_e633_d_n0, eq35_e633_d_n1, eq35_e633_d_n2, eq35_e633_d_n3, eq35_e633_d_n4, eq35_e633_d_n5, eq35_e633_d_n6, eq35_e633_d_n7, eq35_e633_d_n8, eq35_e633_d_n9, eq35_e633_d_n10, eq35_e633_d_n11, eq35_e633_d_n12, eq35_e633_d_n13, eq35_e633_d_n14, eq35_e633_d_n15, eq35_e633_d_n16, eq35_e633_d_n17, eq35_e633_d_n18, eq35_e633_d_n19, eq35_e633_d_n20, eq35_e633_d_n21, eq35_e633_d_n22, eq35_e633_d_b0, eq35_e633_d_b1, eq35_e633_d_b2, eq35_e633_d_b3, eq35_e633_d_b4, eq35_e633_d_b5, eq35_e633_d_b6, eq35_e633_d_b7, eq35_e633_d_b8, eq35_e633_d_b9, eq35_e633_d_b10, eq35_e633_d_b11, eq35_e633_d_b12, eq35_e633_d_b13, eq35_e633_d_b14, eq35_e633_d_b15, eq35_e633_d_b16, eq35_e633_d_b17, eq35_e633_d_b18, eq35_e633_d_b19, eq35_e633_d_b20, eq35_e633_d_b21, eq35_e633_d_b22, eq35_e633_d_b23, eq35_e633_d_b24, eq35_e633_d_b25, eq35_e633_d_b26, eq35_e633_d_b27, eq35_e633_d_b28, eq35_e633_d_b29, eq35_e633_d_b30, eq35_e633_d_b31, eq35_e633_d_b32, eq35_e633_d_b33, eq35_e633_d_b34, eq35_e633_d_b35, eq35_e633_d_b36, eq35_e633_d_b37, eq35_e633_d_b38, eq35_e633_d_b39, eq35_e633_d_b40, eq35_e633_d_b41, eq35_e633_d_b42, eq35_e633_d_b43, eq35_e633_d_b44, eq35_e633_d_b45, eq35_e633_d_b46, eq35_e633_d_b47, eq35_e633_d_b48, eq35_e633_d_b49, eq35_e633_d_b50, eq35_e633_d_b51, eq35_e633_d_b52, eq35_e633_d_b53, eq35_e633_d_b54,) = {
    if (s.b[391] && (!(((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]))) {
        (s.v[136], s.dn[136][0], s.dn[136][1], s.dn[136][2], s.dn[136][3], s.dn[136][4], s.dn[136][5], s.dn[136][6], s.dn[136][7], s.dn[136][8], s.dn[136][9], s.dn[136][10], s.dn[136][11], s.dn[136][12], s.dn[136][13], s.dn[136][14], s.dn[136][15], s.dn[136][16], s.dn[136][17], s.dn[136][18], s.dn[136][19], s.dn[136][20], s.dn[136][21], s.dn[136][22], s.db[136][0], s.db[136][1], s.db[136][2], s.db[136][3], s.db[136][4], s.db[136][5], s.db[136][6], s.db[136][7], s.db[136][8], s.db[136][9], s.db[136][10], s.db[136][11], s.db[136][12], s.db[136][13], s.db[136][14], s.db[136][15], s.db[136][16], s.db[136][17], s.db[136][18], s.db[136][19], s.db[136][20], s.db[136][21], s.db[136][22], s.db[136][23], s.db[136][24], s.db[136][25], s.db[136][26], s.db[136][27], s.db[136][28], s.db[136][29], s.db[136][30], s.db[136][31], s.db[136][32], s.db[136][33], s.db[136][34], s.db[136][35], s.db[136][36], s.db[136][37], s.db[136][38], s.db[136][39], s.db[136][40], s.db[136][41], s.db[136][42], s.db[136][43], s.db[136][44], s.db[136][45], s.db[136][46], s.db[136][47], s.db[136][48], s.db[136][49], s.db[136][50], s.db[136][51], s.db[136][52], s.db[136][53], s.db[136][54],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e633;
        let eq35_node_derivatives: [f64; 23] = [eq35_e633_d_n0, eq35_e633_d_n1, eq35_e633_d_n2, eq35_e633_d_n3, eq35_e633_d_n4, eq35_e633_d_n5, eq35_e633_d_n6, eq35_e633_d_n7, eq35_e633_d_n8, eq35_e633_d_n9, eq35_e633_d_n10, eq35_e633_d_n11, eq35_e633_d_n12, eq35_e633_d_n13, eq35_e633_d_n14, eq35_e633_d_n15, eq35_e633_d_n16, eq35_e633_d_n17, eq35_e633_d_n18, eq35_e633_d_n19, eq35_e633_d_n20, eq35_e633_d_n21, eq35_e633_d_n22];
        let eq35_branch_derivatives: [f64; 55] = [eq35_e633_d_b0, eq35_e633_d_b1, eq35_e633_d_b2, eq35_e633_d_b3, eq35_e633_d_b4, eq35_e633_d_b5, eq35_e633_d_b6, eq35_e633_d_b7, eq35_e633_d_b8, eq35_e633_d_b9, eq35_e633_d_b10, eq35_e633_d_b11, eq35_e633_d_b12, eq35_e633_d_b13, eq35_e633_d_b14, eq35_e633_d_b15, eq35_e633_d_b16, eq35_e633_d_b17, eq35_e633_d_b18, eq35_e633_d_b19, eq35_e633_d_b20, eq35_e633_d_b21, eq35_e633_d_b22, eq35_e633_d_b23, eq35_e633_d_b24, eq35_e633_d_b25, eq35_e633_d_b26, eq35_e633_d_b27, eq35_e633_d_b28, eq35_e633_d_b29, eq35_e633_d_b30, eq35_e633_d_b31, eq35_e633_d_b32, eq35_e633_d_b33, eq35_e633_d_b34, eq35_e633_d_b35, eq35_e633_d_b36, eq35_e633_d_b37, eq35_e633_d_b38, eq35_e633_d_b39, eq35_e633_d_b40, eq35_e633_d_b41, eq35_e633_d_b42, eq35_e633_d_b43, eq35_e633_d_b44, eq35_e633_d_b45, eq35_e633_d_b46, eq35_e633_d_b47, eq35_e633_d_b48, eq35_e633_d_b49, eq35_e633_d_b50, eq35_e633_d_b51, eq35_e633_d_b52, eq35_e633_d_b53, eq35_e633_d_b54];
        stamper.stamp_potential_dense_local(
            23,
            eq35_value,
            &eq35_node_derivatives,
            &eq35_branch_derivatives,
        );
        let (eq37_e668, eq37_e668_d_n12,) = {
    if (s.b[391] && (!(((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]))) {
        let eq37_e661: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, (nv12 - 0.0));
        let eq37_e662: f64 = (p.p97 * eq37_e661);
        let eq37_e665: f64 = (1e-12 * (nv12 - 0.0));
        let eq37_e665_d_n12: f64 = 1e-12;
        let eq37_e666: f64 = (eq37_e662 + eq37_e665);
        let eq37_e666_d_n12: f64 = ((p.p97 * ddt_scale) + eq37_e665_d_n12);
        (eq37_e666, eq37_e666_d_n12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq37_value: f64 = eq37_e668;
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (eq37_value),
            12,
            multiplicity * (eq37_e668_d_n12),
        );
        let (eq38_e681, eq38_e681_d_n0, eq38_e681_d_n1, eq38_e681_d_n2, eq38_e681_d_n3, eq38_e681_d_n4, eq38_e681_d_n5, eq38_e681_d_n6, eq38_e681_d_n7, eq38_e681_d_n8, eq38_e681_d_n9, eq38_e681_d_n10, eq38_e681_d_n11, eq38_e681_d_n12, eq38_e681_d_n13, eq38_e681_d_n14, eq38_e681_d_n15, eq38_e681_d_n16, eq38_e681_d_n17, eq38_e681_d_n18, eq38_e681_d_n19, eq38_e681_d_n20, eq38_e681_d_n21, eq38_e681_d_n22, eq38_e681_d_b0, eq38_e681_d_b1, eq38_e681_d_b2, eq38_e681_d_b3, eq38_e681_d_b4, eq38_e681_d_b5, eq38_e681_d_b6, eq38_e681_d_b7, eq38_e681_d_b8, eq38_e681_d_b9, eq38_e681_d_b10, eq38_e681_d_b11, eq38_e681_d_b12, eq38_e681_d_b13, eq38_e681_d_b14, eq38_e681_d_b15, eq38_e681_d_b16, eq38_e681_d_b17, eq38_e681_d_b18, eq38_e681_d_b19, eq38_e681_d_b20, eq38_e681_d_b21, eq38_e681_d_b22, eq38_e681_d_b23, eq38_e681_d_b24, eq38_e681_d_b25, eq38_e681_d_b26, eq38_e681_d_b27, eq38_e681_d_b28, eq38_e681_d_b29, eq38_e681_d_b30, eq38_e681_d_b31, eq38_e681_d_b32, eq38_e681_d_b33, eq38_e681_d_b34, eq38_e681_d_b35, eq38_e681_d_b36, eq38_e681_d_b37, eq38_e681_d_b38, eq38_e681_d_b39, eq38_e681_d_b40, eq38_e681_d_b41, eq38_e681_d_b42, eq38_e681_d_b43, eq38_e681_d_b44, eq38_e681_d_b45, eq38_e681_d_b46, eq38_e681_d_b47, eq38_e681_d_b48, eq38_e681_d_b49, eq38_e681_d_b50, eq38_e681_d_b51, eq38_e681_d_b52, eq38_e681_d_b53, eq38_e681_d_b54,) = {
    if (s.b[391] && (!(((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]))) {
        (s.v[90], s.dn[90][0], s.dn[90][1], s.dn[90][2], s.dn[90][3], s.dn[90][4], s.dn[90][5], s.dn[90][6], s.dn[90][7], s.dn[90][8], s.dn[90][9], s.dn[90][10], s.dn[90][11], s.dn[90][12], s.dn[90][13], s.dn[90][14], s.dn[90][15], s.dn[90][16], s.dn[90][17], s.dn[90][18], s.dn[90][19], s.dn[90][20], s.dn[90][21], s.dn[90][22], s.db[90][0], s.db[90][1], s.db[90][2], s.db[90][3], s.db[90][4], s.db[90][5], s.db[90][6], s.db[90][7], s.db[90][8], s.db[90][9], s.db[90][10], s.db[90][11], s.db[90][12], s.db[90][13], s.db[90][14], s.db[90][15], s.db[90][16], s.db[90][17], s.db[90][18], s.db[90][19], s.db[90][20], s.db[90][21], s.db[90][22], s.db[90][23], s.db[90][24], s.db[90][25], s.db[90][26], s.db[90][27], s.db[90][28], s.db[90][29], s.db[90][30], s.db[90][31], s.db[90][32], s.db[90][33], s.db[90][34], s.db[90][35], s.db[90][36], s.db[90][37], s.db[90][38], s.db[90][39], s.db[90][40], s.db[90][41], s.db[90][42], s.db[90][43], s.db[90][44], s.db[90][45], s.db[90][46], s.db[90][47], s.db[90][48], s.db[90][49], s.db[90][50], s.db[90][51], s.db[90][52], s.db[90][53], s.db[90][54],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e681;
        let eq38_node_derivatives: [f64; 23] = [eq38_e681_d_n0, eq38_e681_d_n1, eq38_e681_d_n2, eq38_e681_d_n3, eq38_e681_d_n4, eq38_e681_d_n5, eq38_e681_d_n6, eq38_e681_d_n7, eq38_e681_d_n8, eq38_e681_d_n9, eq38_e681_d_n10, eq38_e681_d_n11, eq38_e681_d_n12, eq38_e681_d_n13, eq38_e681_d_n14, eq38_e681_d_n15, eq38_e681_d_n16, eq38_e681_d_n17, eq38_e681_d_n18, eq38_e681_d_n19, eq38_e681_d_n20, eq38_e681_d_n21, eq38_e681_d_n22];
        let eq38_branch_derivatives: [f64; 55] = [eq38_e681_d_b0, eq38_e681_d_b1, eq38_e681_d_b2, eq38_e681_d_b3, eq38_e681_d_b4, eq38_e681_d_b5, eq38_e681_d_b6, eq38_e681_d_b7, eq38_e681_d_b8, eq38_e681_d_b9, eq38_e681_d_b10, eq38_e681_d_b11, eq38_e681_d_b12, eq38_e681_d_b13, eq38_e681_d_b14, eq38_e681_d_b15, eq38_e681_d_b16, eq38_e681_d_b17, eq38_e681_d_b18, eq38_e681_d_b19, eq38_e681_d_b20, eq38_e681_d_b21, eq38_e681_d_b22, eq38_e681_d_b23, eq38_e681_d_b24, eq38_e681_d_b25, eq38_e681_d_b26, eq38_e681_d_b27, eq38_e681_d_b28, eq38_e681_d_b29, eq38_e681_d_b30, eq38_e681_d_b31, eq38_e681_d_b32, eq38_e681_d_b33, eq38_e681_d_b34, eq38_e681_d_b35, eq38_e681_d_b36, eq38_e681_d_b37, eq38_e681_d_b38, eq38_e681_d_b39, eq38_e681_d_b40, eq38_e681_d_b41, eq38_e681_d_b42, eq38_e681_d_b43, eq38_e681_d_b44, eq38_e681_d_b45, eq38_e681_d_b46, eq38_e681_d_b47, eq38_e681_d_b48, eq38_e681_d_b49, eq38_e681_d_b50, eq38_e681_d_b51, eq38_e681_d_b52, eq38_e681_d_b53, eq38_e681_d_b54];
        stamper.stamp_potential_dense_local(
            24,
            eq38_value,
            &eq38_node_derivatives,
            &eq38_branch_derivatives,
        );
        let (eq40_e716, eq40_e716_d_n14,) = {
    if (s.b[391] && (!(((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]))) {
        let eq40_e709: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, (nv14 - 0.0));
        let eq40_e710: f64 = (p.p83 * eq40_e709);
        let eq40_e713: f64 = (1e-12 * (nv14 - 0.0));
        let eq40_e713_d_n14: f64 = 1e-12;
        let eq40_e714: f64 = (eq40_e710 + eq40_e713);
        let eq40_e714_d_n14: f64 = ((p.p83 * ddt_scale) + eq40_e713_d_n14);
        (eq40_e714, eq40_e714_d_n14,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e716;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (eq40_value),
            14,
            multiplicity * (eq40_e716_d_n14),
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq41_e747, eq41_e747_d_n0, eq41_e747_d_n1, eq41_e747_d_n2, eq41_e747_d_n3, eq41_e747_d_n4, eq41_e747_d_n5, eq41_e747_d_n6, eq41_e747_d_n7, eq41_e747_d_n8, eq41_e747_d_n9, eq41_e747_d_n10, eq41_e747_d_n11, eq41_e747_d_n12, eq41_e747_d_n13, eq41_e747_d_n14, eq41_e747_d_n15, eq41_e747_d_n16, eq41_e747_d_n17, eq41_e747_d_n18, eq41_e747_d_n19, eq41_e747_d_n20, eq41_e747_d_n21, eq41_e747_d_n22, eq41_e747_d_b0, eq41_e747_d_b1, eq41_e747_d_b2, eq41_e747_d_b3, eq41_e747_d_b4, eq41_e747_d_b5, eq41_e747_d_b6, eq41_e747_d_b7, eq41_e747_d_b8, eq41_e747_d_b9, eq41_e747_d_b10, eq41_e747_d_b11, eq41_e747_d_b12, eq41_e747_d_b13, eq41_e747_d_b14, eq41_e747_d_b15, eq41_e747_d_b16, eq41_e747_d_b17, eq41_e747_d_b18, eq41_e747_d_b19, eq41_e747_d_b20, eq41_e747_d_b21, eq41_e747_d_b22, eq41_e747_d_b23, eq41_e747_d_b24, eq41_e747_d_b25, eq41_e747_d_b26, eq41_e747_d_b27, eq41_e747_d_b28, eq41_e747_d_b29, eq41_e747_d_b30, eq41_e747_d_b31, eq41_e747_d_b32, eq41_e747_d_b33, eq41_e747_d_b34, eq41_e747_d_b35, eq41_e747_d_b36, eq41_e747_d_b37, eq41_e747_d_b38, eq41_e747_d_b39, eq41_e747_d_b40, eq41_e747_d_b41, eq41_e747_d_b42, eq41_e747_d_b43, eq41_e747_d_b44, eq41_e747_d_b45, eq41_e747_d_b46, eq41_e747_d_b47, eq41_e747_d_b48, eq41_e747_d_b49, eq41_e747_d_b50, eq41_e747_d_b51, eq41_e747_d_b52, eq41_e747_d_b53, eq41_e747_d_b54,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        let eq41_e730: f64 = (-p.p135);
        let eq41_e732: f64 = (eq41_e730 * s.v[363]);
        let eq41_e732_d_n0: f64 = (eq41_e730 * s.dn[363][0]);
        let eq41_e732_d_n1: f64 = (eq41_e730 * s.dn[363][1]);
        let eq41_e732_d_n2: f64 = (eq41_e730 * s.dn[363][2]);
        let eq41_e732_d_n3: f64 = (eq41_e730 * s.dn[363][3]);
        let eq41_e732_d_n4: f64 = (eq41_e730 * s.dn[363][4]);
        let eq41_e732_d_n5: f64 = (eq41_e730 * s.dn[363][5]);
        let eq41_e732_d_n6: f64 = (eq41_e730 * s.dn[363][6]);
        let eq41_e732_d_n7: f64 = (eq41_e730 * s.dn[363][7]);
        let eq41_e732_d_n8: f64 = (eq41_e730 * s.dn[363][8]);
        let eq41_e732_d_n9: f64 = (eq41_e730 * s.dn[363][9]);
        let eq41_e732_d_n10: f64 = (eq41_e730 * s.dn[363][10]);
        let eq41_e732_d_n11: f64 = (eq41_e730 * s.dn[363][11]);
        let eq41_e732_d_n12: f64 = (eq41_e730 * s.dn[363][12]);
        let eq41_e732_d_n13: f64 = (eq41_e730 * s.dn[363][13]);
        let eq41_e732_d_n14: f64 = (eq41_e730 * s.dn[363][14]);
        let eq41_e732_d_n15: f64 = (eq41_e730 * s.dn[363][15]);
        let eq41_e732_d_n16: f64 = (eq41_e730 * s.dn[363][16]);
        let eq41_e732_d_n17: f64 = (eq41_e730 * s.dn[363][17]);
        let eq41_e732_d_n18: f64 = (eq41_e730 * s.dn[363][18]);
        let eq41_e732_d_n19: f64 = (eq41_e730 * s.dn[363][19]);
        let eq41_e732_d_n20: f64 = (eq41_e730 * s.dn[363][20]);
        let eq41_e732_d_n21: f64 = (eq41_e730 * s.dn[363][21]);
        let eq41_e732_d_n22: f64 = (eq41_e730 * s.dn[363][22]);
        let eq41_e732_d_b0: f64 = (eq41_e730 * s.db[363][0]);
        let eq41_e732_d_b1: f64 = (eq41_e730 * s.db[363][1]);
        let eq41_e732_d_b2: f64 = (eq41_e730 * s.db[363][2]);
        let eq41_e732_d_b3: f64 = (eq41_e730 * s.db[363][3]);
        let eq41_e732_d_b4: f64 = (eq41_e730 * s.db[363][4]);
        let eq41_e732_d_b5: f64 = (eq41_e730 * s.db[363][5]);
        let eq41_e732_d_b6: f64 = (eq41_e730 * s.db[363][6]);
        let eq41_e732_d_b7: f64 = (eq41_e730 * s.db[363][7]);
        let eq41_e732_d_b8: f64 = (eq41_e730 * s.db[363][8]);
        let eq41_e732_d_b9: f64 = (eq41_e730 * s.db[363][9]);
        let eq41_e732_d_b10: f64 = (eq41_e730 * s.db[363][10]);
        let eq41_e732_d_b11: f64 = (eq41_e730 * s.db[363][11]);
        let eq41_e732_d_b12: f64 = (eq41_e730 * s.db[363][12]);
        let eq41_e732_d_b13: f64 = (eq41_e730 * s.db[363][13]);
        let eq41_e732_d_b14: f64 = (eq41_e730 * s.db[363][14]);
        let eq41_e732_d_b15: f64 = (eq41_e730 * s.db[363][15]);
        let eq41_e732_d_b16: f64 = (eq41_e730 * s.db[363][16]);
        let eq41_e732_d_b17: f64 = (eq41_e730 * s.db[363][17]);
        let eq41_e732_d_b18: f64 = (eq41_e730 * s.db[363][18]);
        let eq41_e732_d_b19: f64 = (eq41_e730 * s.db[363][19]);
        let eq41_e732_d_b20: f64 = (eq41_e730 * s.db[363][20]);
        let eq41_e732_d_b21: f64 = (eq41_e730 * s.db[363][21]);
        let eq41_e732_d_b22: f64 = (eq41_e730 * s.db[363][22]);
        let eq41_e732_d_b23: f64 = (eq41_e730 * s.db[363][23]);
        let eq41_e732_d_b24: f64 = (eq41_e730 * s.db[363][24]);
        let eq41_e732_d_b25: f64 = (eq41_e730 * s.db[363][25]);
        let eq41_e732_d_b26: f64 = (eq41_e730 * s.db[363][26]);
        let eq41_e732_d_b27: f64 = (eq41_e730 * s.db[363][27]);
        let eq41_e732_d_b28: f64 = (eq41_e730 * s.db[363][28]);
        let eq41_e732_d_b29: f64 = (eq41_e730 * s.db[363][29]);
        let eq41_e732_d_b30: f64 = (eq41_e730 * s.db[363][30]);
        let eq41_e732_d_b31: f64 = (eq41_e730 * s.db[363][31]);
        let eq41_e732_d_b32: f64 = (eq41_e730 * s.db[363][32]);
        let eq41_e732_d_b33: f64 = (eq41_e730 * s.db[363][33]);
        let eq41_e732_d_b34: f64 = (eq41_e730 * s.db[363][34]);
        let eq41_e732_d_b35: f64 = (eq41_e730 * s.db[363][35]);
        let eq41_e732_d_b36: f64 = (eq41_e730 * s.db[363][36]);
        let eq41_e732_d_b37: f64 = (eq41_e730 * s.db[363][37]);
        let eq41_e732_d_b38: f64 = (eq41_e730 * s.db[363][38]);
        let eq41_e732_d_b39: f64 = (eq41_e730 * s.db[363][39]);
        let eq41_e732_d_b40: f64 = (eq41_e730 * s.db[363][40]);
        let eq41_e732_d_b41: f64 = (eq41_e730 * s.db[363][41]);
        let eq41_e732_d_b42: f64 = (eq41_e730 * s.db[363][42]);
        let eq41_e732_d_b43: f64 = (eq41_e730 * s.db[363][43]);
        let eq41_e732_d_b44: f64 = (eq41_e730 * s.db[363][44]);
        let eq41_e732_d_b45: f64 = (eq41_e730 * s.db[363][45]);
        let eq41_e732_d_b46: f64 = (eq41_e730 * s.db[363][46]);
        let eq41_e732_d_b47: f64 = (eq41_e730 * s.db[363][47]);
        let eq41_e732_d_b48: f64 = (eq41_e730 * s.db[363][48]);
        let eq41_e732_d_b49: f64 = (eq41_e730 * s.db[363][49]);
        let eq41_e732_d_b50: f64 = (eq41_e730 * s.db[363][50]);
        let eq41_e732_d_b51: f64 = (eq41_e730 * s.db[363][51]);
        let eq41_e732_d_b52: f64 = (eq41_e730 * s.db[363][52]);
        let eq41_e732_d_b53: f64 = (eq41_e730 * s.db[363][53]);
        let eq41_e732_d_b54: f64 = (eq41_e730 * s.db[363][54]);
        let eq41_e735: f64 = (p.p136 - (nv5 - 0.0));
        let eq41_e735_d_n5: f64 = (-1.0);
        let eq41_e736: f64 = (eq41_e732 * eq41_e735);
        let eq41_e736_d_n0: f64 = (eq41_e732_d_n0 * eq41_e735);
        let eq41_e736_d_n1: f64 = (eq41_e732_d_n1 * eq41_e735);
        let eq41_e736_d_n2: f64 = (eq41_e732_d_n2 * eq41_e735);
        let eq41_e736_d_n3: f64 = (eq41_e732_d_n3 * eq41_e735);
        let eq41_e736_d_n4: f64 = (eq41_e732_d_n4 * eq41_e735);
        let eq41_e736_d_n5: f64 = ((eq41_e732_d_n5 * eq41_e735) + (eq41_e732 * eq41_e735_d_n5));
        let eq41_e736_d_n6: f64 = (eq41_e732_d_n6 * eq41_e735);
        let eq41_e736_d_n7: f64 = (eq41_e732_d_n7 * eq41_e735);
        let eq41_e736_d_n8: f64 = (eq41_e732_d_n8 * eq41_e735);
        let eq41_e736_d_n9: f64 = (eq41_e732_d_n9 * eq41_e735);
        let eq41_e736_d_n10: f64 = (eq41_e732_d_n10 * eq41_e735);
        let eq41_e736_d_n11: f64 = (eq41_e732_d_n11 * eq41_e735);
        let eq41_e736_d_n12: f64 = (eq41_e732_d_n12 * eq41_e735);
        let eq41_e736_d_n13: f64 = (eq41_e732_d_n13 * eq41_e735);
        let eq41_e736_d_n14: f64 = (eq41_e732_d_n14 * eq41_e735);
        let eq41_e736_d_n15: f64 = (eq41_e732_d_n15 * eq41_e735);
        let eq41_e736_d_n16: f64 = (eq41_e732_d_n16 * eq41_e735);
        let eq41_e736_d_n17: f64 = (eq41_e732_d_n17 * eq41_e735);
        let eq41_e736_d_n18: f64 = (eq41_e732_d_n18 * eq41_e735);
        let eq41_e736_d_n19: f64 = (eq41_e732_d_n19 * eq41_e735);
        let eq41_e736_d_n20: f64 = (eq41_e732_d_n20 * eq41_e735);
        let eq41_e736_d_n21: f64 = (eq41_e732_d_n21 * eq41_e735);
        let eq41_e736_d_n22: f64 = (eq41_e732_d_n22 * eq41_e735);
        let eq41_e736_d_b0: f64 = (eq41_e732_d_b0 * eq41_e735);
        let eq41_e736_d_b1: f64 = (eq41_e732_d_b1 * eq41_e735);
        let eq41_e736_d_b2: f64 = (eq41_e732_d_b2 * eq41_e735);
        let eq41_e736_d_b3: f64 = (eq41_e732_d_b3 * eq41_e735);
        let eq41_e736_d_b4: f64 = (eq41_e732_d_b4 * eq41_e735);
        let eq41_e736_d_b5: f64 = (eq41_e732_d_b5 * eq41_e735);
        let eq41_e736_d_b6: f64 = (eq41_e732_d_b6 * eq41_e735);
        let eq41_e736_d_b7: f64 = (eq41_e732_d_b7 * eq41_e735);
        let eq41_e736_d_b8: f64 = (eq41_e732_d_b8 * eq41_e735);
        let eq41_e736_d_b9: f64 = (eq41_e732_d_b9 * eq41_e735);
        let eq41_e736_d_b10: f64 = (eq41_e732_d_b10 * eq41_e735);
        let eq41_e736_d_b11: f64 = (eq41_e732_d_b11 * eq41_e735);
        let eq41_e736_d_b12: f64 = (eq41_e732_d_b12 * eq41_e735);
        let eq41_e736_d_b13: f64 = (eq41_e732_d_b13 * eq41_e735);
        let eq41_e736_d_b14: f64 = (eq41_e732_d_b14 * eq41_e735);
        let eq41_e736_d_b15: f64 = (eq41_e732_d_b15 * eq41_e735);
        let eq41_e736_d_b16: f64 = (eq41_e732_d_b16 * eq41_e735);
        let eq41_e736_d_b17: f64 = (eq41_e732_d_b17 * eq41_e735);
        let eq41_e736_d_b18: f64 = (eq41_e732_d_b18 * eq41_e735);
        let eq41_e736_d_b19: f64 = (eq41_e732_d_b19 * eq41_e735);
        let eq41_e736_d_b20: f64 = (eq41_e732_d_b20 * eq41_e735);
        let eq41_e736_d_b21: f64 = (eq41_e732_d_b21 * eq41_e735);
        let eq41_e736_d_b22: f64 = (eq41_e732_d_b22 * eq41_e735);
        let eq41_e736_d_b23: f64 = (eq41_e732_d_b23 * eq41_e735);
        let eq41_e736_d_b24: f64 = (eq41_e732_d_b24 * eq41_e735);
        let eq41_e736_d_b25: f64 = (eq41_e732_d_b25 * eq41_e735);
        let eq41_e736_d_b26: f64 = (eq41_e732_d_b26 * eq41_e735);
        let eq41_e736_d_b27: f64 = (eq41_e732_d_b27 * eq41_e735);
        let eq41_e736_d_b28: f64 = (eq41_e732_d_b28 * eq41_e735);
        let eq41_e736_d_b29: f64 = (eq41_e732_d_b29 * eq41_e735);
        let eq41_e736_d_b30: f64 = (eq41_e732_d_b30 * eq41_e735);
        let eq41_e736_d_b31: f64 = (eq41_e732_d_b31 * eq41_e735);
        let eq41_e736_d_b32: f64 = (eq41_e732_d_b32 * eq41_e735);
        let eq41_e736_d_b33: f64 = (eq41_e732_d_b33 * eq41_e735);
        let eq41_e736_d_b34: f64 = (eq41_e732_d_b34 * eq41_e735);
        let eq41_e736_d_b35: f64 = (eq41_e732_d_b35 * eq41_e735);
        let eq41_e736_d_b36: f64 = (eq41_e732_d_b36 * eq41_e735);
        let eq41_e736_d_b37: f64 = (eq41_e732_d_b37 * eq41_e735);
        let eq41_e736_d_b38: f64 = (eq41_e732_d_b38 * eq41_e735);
        let eq41_e736_d_b39: f64 = (eq41_e732_d_b39 * eq41_e735);
        let eq41_e736_d_b40: f64 = (eq41_e732_d_b40 * eq41_e735);
        let eq41_e736_d_b41: f64 = (eq41_e732_d_b41 * eq41_e735);
        let eq41_e736_d_b42: f64 = (eq41_e732_d_b42 * eq41_e735);
        let eq41_e736_d_b43: f64 = (eq41_e732_d_b43 * eq41_e735);
        let eq41_e736_d_b44: f64 = (eq41_e732_d_b44 * eq41_e735);
        let eq41_e736_d_b45: f64 = (eq41_e732_d_b45 * eq41_e735);
        let eq41_e736_d_b46: f64 = (eq41_e732_d_b46 * eq41_e735);
        let eq41_e736_d_b47: f64 = (eq41_e732_d_b47 * eq41_e735);
        let eq41_e736_d_b48: f64 = (eq41_e732_d_b48 * eq41_e735);
        let eq41_e736_d_b49: f64 = (eq41_e732_d_b49 * eq41_e735);
        let eq41_e736_d_b50: f64 = (eq41_e732_d_b50 * eq41_e735);
        let eq41_e736_d_b51: f64 = (eq41_e732_d_b51 * eq41_e735);
        let eq41_e736_d_b52: f64 = (eq41_e732_d_b52 * eq41_e735);
        let eq41_e736_d_b53: f64 = (eq41_e732_d_b53 * eq41_e735);
        let eq41_e736_d_b54: f64 = (eq41_e732_d_b54 * eq41_e735);
        let eq41_e739: f64 = (2.0 * s.v[362]);
        let eq41_e739_d_n0: f64 = (2.0 * s.dn[362][0]);
        let eq41_e739_d_n1: f64 = (2.0 * s.dn[362][1]);
        let eq41_e739_d_n2: f64 = (2.0 * s.dn[362][2]);
        let eq41_e739_d_n3: f64 = (2.0 * s.dn[362][3]);
        let eq41_e739_d_n4: f64 = (2.0 * s.dn[362][4]);
        let eq41_e739_d_n5: f64 = (2.0 * s.dn[362][5]);
        let eq41_e739_d_n6: f64 = (2.0 * s.dn[362][6]);
        let eq41_e739_d_n7: f64 = (2.0 * s.dn[362][7]);
        let eq41_e739_d_n8: f64 = (2.0 * s.dn[362][8]);
        let eq41_e739_d_n9: f64 = (2.0 * s.dn[362][9]);
        let eq41_e739_d_n10: f64 = (2.0 * s.dn[362][10]);
        let eq41_e739_d_n11: f64 = (2.0 * s.dn[362][11]);
        let eq41_e739_d_n12: f64 = (2.0 * s.dn[362][12]);
        let eq41_e739_d_n13: f64 = (2.0 * s.dn[362][13]);
        let eq41_e739_d_n14: f64 = (2.0 * s.dn[362][14]);
        let eq41_e739_d_n15: f64 = (2.0 * s.dn[362][15]);
        let eq41_e739_d_n16: f64 = (2.0 * s.dn[362][16]);
        let eq41_e739_d_n17: f64 = (2.0 * s.dn[362][17]);
        let eq41_e739_d_n18: f64 = (2.0 * s.dn[362][18]);
        let eq41_e739_d_n19: f64 = (2.0 * s.dn[362][19]);
        let eq41_e739_d_n20: f64 = (2.0 * s.dn[362][20]);
        let eq41_e739_d_n21: f64 = (2.0 * s.dn[362][21]);
        let eq41_e739_d_n22: f64 = (2.0 * s.dn[362][22]);
        let eq41_e739_d_b0: f64 = (2.0 * s.db[362][0]);
        let eq41_e739_d_b1: f64 = (2.0 * s.db[362][1]);
        let eq41_e739_d_b2: f64 = (2.0 * s.db[362][2]);
        let eq41_e739_d_b3: f64 = (2.0 * s.db[362][3]);
        let eq41_e739_d_b4: f64 = (2.0 * s.db[362][4]);
        let eq41_e739_d_b5: f64 = (2.0 * s.db[362][5]);
        let eq41_e739_d_b6: f64 = (2.0 * s.db[362][6]);
        let eq41_e739_d_b7: f64 = (2.0 * s.db[362][7]);
        let eq41_e739_d_b8: f64 = (2.0 * s.db[362][8]);
        let eq41_e739_d_b9: f64 = (2.0 * s.db[362][9]);
        let eq41_e739_d_b10: f64 = (2.0 * s.db[362][10]);
        let eq41_e739_d_b11: f64 = (2.0 * s.db[362][11]);
        let eq41_e739_d_b12: f64 = (2.0 * s.db[362][12]);
        let eq41_e739_d_b13: f64 = (2.0 * s.db[362][13]);
        let eq41_e739_d_b14: f64 = (2.0 * s.db[362][14]);
        let eq41_e739_d_b15: f64 = (2.0 * s.db[362][15]);
        let eq41_e739_d_b16: f64 = (2.0 * s.db[362][16]);
        let eq41_e739_d_b17: f64 = (2.0 * s.db[362][17]);
        let eq41_e739_d_b18: f64 = (2.0 * s.db[362][18]);
        let eq41_e739_d_b19: f64 = (2.0 * s.db[362][19]);
        let eq41_e739_d_b20: f64 = (2.0 * s.db[362][20]);
        let eq41_e739_d_b21: f64 = (2.0 * s.db[362][21]);
        let eq41_e739_d_b22: f64 = (2.0 * s.db[362][22]);
        let eq41_e739_d_b23: f64 = (2.0 * s.db[362][23]);
        let eq41_e739_d_b24: f64 = (2.0 * s.db[362][24]);
        let eq41_e739_d_b25: f64 = (2.0 * s.db[362][25]);
        let eq41_e739_d_b26: f64 = (2.0 * s.db[362][26]);
        let eq41_e739_d_b27: f64 = (2.0 * s.db[362][27]);
        let eq41_e739_d_b28: f64 = (2.0 * s.db[362][28]);
        let eq41_e739_d_b29: f64 = (2.0 * s.db[362][29]);
        let eq41_e739_d_b30: f64 = (2.0 * s.db[362][30]);
        let eq41_e739_d_b31: f64 = (2.0 * s.db[362][31]);
        let eq41_e739_d_b32: f64 = (2.0 * s.db[362][32]);
        let eq41_e739_d_b33: f64 = (2.0 * s.db[362][33]);
        let eq41_e739_d_b34: f64 = (2.0 * s.db[362][34]);
        let eq41_e739_d_b35: f64 = (2.0 * s.db[362][35]);
        let eq41_e739_d_b36: f64 = (2.0 * s.db[362][36]);
        let eq41_e739_d_b37: f64 = (2.0 * s.db[362][37]);
        let eq41_e739_d_b38: f64 = (2.0 * s.db[362][38]);
        let eq41_e739_d_b39: f64 = (2.0 * s.db[362][39]);
        let eq41_e739_d_b40: f64 = (2.0 * s.db[362][40]);
        let eq41_e739_d_b41: f64 = (2.0 * s.db[362][41]);
        let eq41_e739_d_b42: f64 = (2.0 * s.db[362][42]);
        let eq41_e739_d_b43: f64 = (2.0 * s.db[362][43]);
        let eq41_e739_d_b44: f64 = (2.0 * s.db[362][44]);
        let eq41_e739_d_b45: f64 = (2.0 * s.db[362][45]);
        let eq41_e739_d_b46: f64 = (2.0 * s.db[362][46]);
        let eq41_e739_d_b47: f64 = (2.0 * s.db[362][47]);
        let eq41_e739_d_b48: f64 = (2.0 * s.db[362][48]);
        let eq41_e739_d_b49: f64 = (2.0 * s.db[362][49]);
        let eq41_e739_d_b50: f64 = (2.0 * s.db[362][50]);
        let eq41_e739_d_b51: f64 = (2.0 * s.db[362][51]);
        let eq41_e739_d_b52: f64 = (2.0 * s.db[362][52]);
        let eq41_e739_d_b53: f64 = (2.0 * s.db[362][53]);
        let eq41_e739_d_b54: f64 = (2.0 * s.db[362][54]);
        let eq41_e740: f64 = (eq41_e739).exp();
        let eq41_e740_d_n0: f64 = (eq41_e740 * eq41_e739_d_n0);
        let eq41_e740_d_n1: f64 = (eq41_e740 * eq41_e739_d_n1);
        let eq41_e740_d_n2: f64 = (eq41_e740 * eq41_e739_d_n2);
        let eq41_e740_d_n3: f64 = (eq41_e740 * eq41_e739_d_n3);
        let eq41_e740_d_n4: f64 = (eq41_e740 * eq41_e739_d_n4);
        let eq41_e740_d_n5: f64 = (eq41_e740 * eq41_e739_d_n5);
        let eq41_e740_d_n6: f64 = (eq41_e740 * eq41_e739_d_n6);
        let eq41_e740_d_n7: f64 = (eq41_e740 * eq41_e739_d_n7);
        let eq41_e740_d_n8: f64 = (eq41_e740 * eq41_e739_d_n8);
        let eq41_e740_d_n9: f64 = (eq41_e740 * eq41_e739_d_n9);
        let eq41_e740_d_n10: f64 = (eq41_e740 * eq41_e739_d_n10);
        let eq41_e740_d_n11: f64 = (eq41_e740 * eq41_e739_d_n11);
        let eq41_e740_d_n12: f64 = (eq41_e740 * eq41_e739_d_n12);
        let eq41_e740_d_n13: f64 = (eq41_e740 * eq41_e739_d_n13);
        let eq41_e740_d_n14: f64 = (eq41_e740 * eq41_e739_d_n14);
        let eq41_e740_d_n15: f64 = (eq41_e740 * eq41_e739_d_n15);
        let eq41_e740_d_n16: f64 = (eq41_e740 * eq41_e739_d_n16);
        let eq41_e740_d_n17: f64 = (eq41_e740 * eq41_e739_d_n17);
        let eq41_e740_d_n18: f64 = (eq41_e740 * eq41_e739_d_n18);
        let eq41_e740_d_n19: f64 = (eq41_e740 * eq41_e739_d_n19);
        let eq41_e740_d_n20: f64 = (eq41_e740 * eq41_e739_d_n20);
        let eq41_e740_d_n21: f64 = (eq41_e740 * eq41_e739_d_n21);
        let eq41_e740_d_n22: f64 = (eq41_e740 * eq41_e739_d_n22);
        let eq41_e740_d_b0: f64 = (eq41_e740 * eq41_e739_d_b0);
        let eq41_e740_d_b1: f64 = (eq41_e740 * eq41_e739_d_b1);
        let eq41_e740_d_b2: f64 = (eq41_e740 * eq41_e739_d_b2);
        let eq41_e740_d_b3: f64 = (eq41_e740 * eq41_e739_d_b3);
        let eq41_e740_d_b4: f64 = (eq41_e740 * eq41_e739_d_b4);
        let eq41_e740_d_b5: f64 = (eq41_e740 * eq41_e739_d_b5);
        let eq41_e740_d_b6: f64 = (eq41_e740 * eq41_e739_d_b6);
        let eq41_e740_d_b7: f64 = (eq41_e740 * eq41_e739_d_b7);
        let eq41_e740_d_b8: f64 = (eq41_e740 * eq41_e739_d_b8);
        let eq41_e740_d_b9: f64 = (eq41_e740 * eq41_e739_d_b9);
        let eq41_e740_d_b10: f64 = (eq41_e740 * eq41_e739_d_b10);
        let eq41_e740_d_b11: f64 = (eq41_e740 * eq41_e739_d_b11);
        let eq41_e740_d_b12: f64 = (eq41_e740 * eq41_e739_d_b12);
        let eq41_e740_d_b13: f64 = (eq41_e740 * eq41_e739_d_b13);
        let eq41_e740_d_b14: f64 = (eq41_e740 * eq41_e739_d_b14);
        let eq41_e740_d_b15: f64 = (eq41_e740 * eq41_e739_d_b15);
        let eq41_e740_d_b16: f64 = (eq41_e740 * eq41_e739_d_b16);
        let eq41_e740_d_b17: f64 = (eq41_e740 * eq41_e739_d_b17);
        let eq41_e740_d_b18: f64 = (eq41_e740 * eq41_e739_d_b18);
        let eq41_e740_d_b19: f64 = (eq41_e740 * eq41_e739_d_b19);
        let eq41_e740_d_b20: f64 = (eq41_e740 * eq41_e739_d_b20);
        let eq41_e740_d_b21: f64 = (eq41_e740 * eq41_e739_d_b21);
        let eq41_e740_d_b22: f64 = (eq41_e740 * eq41_e739_d_b22);
        let eq41_e740_d_b23: f64 = (eq41_e740 * eq41_e739_d_b23);
        let eq41_e740_d_b24: f64 = (eq41_e740 * eq41_e739_d_b24);
        let eq41_e740_d_b25: f64 = (eq41_e740 * eq41_e739_d_b25);
        let eq41_e740_d_b26: f64 = (eq41_e740 * eq41_e739_d_b26);
        let eq41_e740_d_b27: f64 = (eq41_e740 * eq41_e739_d_b27);
        let eq41_e740_d_b28: f64 = (eq41_e740 * eq41_e739_d_b28);
        let eq41_e740_d_b29: f64 = (eq41_e740 * eq41_e739_d_b29);
        let eq41_e740_d_b30: f64 = (eq41_e740 * eq41_e739_d_b30);
        let eq41_e740_d_b31: f64 = (eq41_e740 * eq41_e739_d_b31);
        let eq41_e740_d_b32: f64 = (eq41_e740 * eq41_e739_d_b32);
        let eq41_e740_d_b33: f64 = (eq41_e740 * eq41_e739_d_b33);
        let eq41_e740_d_b34: f64 = (eq41_e740 * eq41_e739_d_b34);
        let eq41_e740_d_b35: f64 = (eq41_e740 * eq41_e739_d_b35);
        let eq41_e740_d_b36: f64 = (eq41_e740 * eq41_e739_d_b36);
        let eq41_e740_d_b37: f64 = (eq41_e740 * eq41_e739_d_b37);
        let eq41_e740_d_b38: f64 = (eq41_e740 * eq41_e739_d_b38);
        let eq41_e740_d_b39: f64 = (eq41_e740 * eq41_e739_d_b39);
        let eq41_e740_d_b40: f64 = (eq41_e740 * eq41_e739_d_b40);
        let eq41_e740_d_b41: f64 = (eq41_e740 * eq41_e739_d_b41);
        let eq41_e740_d_b42: f64 = (eq41_e740 * eq41_e739_d_b42);
        let eq41_e740_d_b43: f64 = (eq41_e740 * eq41_e739_d_b43);
        let eq41_e740_d_b44: f64 = (eq41_e740 * eq41_e739_d_b44);
        let eq41_e740_d_b45: f64 = (eq41_e740 * eq41_e739_d_b45);
        let eq41_e740_d_b46: f64 = (eq41_e740 * eq41_e739_d_b46);
        let eq41_e740_d_b47: f64 = (eq41_e740 * eq41_e739_d_b47);
        let eq41_e740_d_b48: f64 = (eq41_e740 * eq41_e739_d_b48);
        let eq41_e740_d_b49: f64 = (eq41_e740 * eq41_e739_d_b49);
        let eq41_e740_d_b50: f64 = (eq41_e740 * eq41_e739_d_b50);
        let eq41_e740_d_b51: f64 = (eq41_e740 * eq41_e739_d_b51);
        let eq41_e740_d_b52: f64 = (eq41_e740 * eq41_e739_d_b52);
        let eq41_e740_d_b53: f64 = (eq41_e740 * eq41_e739_d_b53);
        let eq41_e740_d_b54: f64 = (eq41_e740 * eq41_e739_d_b54);
        let eq41_e742: f64 = (eq41_e740 - 1.0);
        let eq41_e743: f64 = (eq41_e736 * eq41_e742);
        let eq41_e743_d_n0: f64 = ((eq41_e736_d_n0 * eq41_e742) + (eq41_e736 * eq41_e740_d_n0));
        let eq41_e743_d_n1: f64 = ((eq41_e736_d_n1 * eq41_e742) + (eq41_e736 * eq41_e740_d_n1));
        let eq41_e743_d_n2: f64 = ((eq41_e736_d_n2 * eq41_e742) + (eq41_e736 * eq41_e740_d_n2));
        let eq41_e743_d_n3: f64 = ((eq41_e736_d_n3 * eq41_e742) + (eq41_e736 * eq41_e740_d_n3));
        let eq41_e743_d_n4: f64 = ((eq41_e736_d_n4 * eq41_e742) + (eq41_e736 * eq41_e740_d_n4));
        let eq41_e743_d_n5: f64 = ((eq41_e736_d_n5 * eq41_e742) + (eq41_e736 * eq41_e740_d_n5));
        let eq41_e743_d_n6: f64 = ((eq41_e736_d_n6 * eq41_e742) + (eq41_e736 * eq41_e740_d_n6));
        let eq41_e743_d_n7: f64 = ((eq41_e736_d_n7 * eq41_e742) + (eq41_e736 * eq41_e740_d_n7));
        let eq41_e743_d_n8: f64 = ((eq41_e736_d_n8 * eq41_e742) + (eq41_e736 * eq41_e740_d_n8));
        let eq41_e743_d_n9: f64 = ((eq41_e736_d_n9 * eq41_e742) + (eq41_e736 * eq41_e740_d_n9));
        let eq41_e743_d_n10: f64 = ((eq41_e736_d_n10 * eq41_e742) + (eq41_e736 * eq41_e740_d_n10));
        let eq41_e743_d_n11: f64 = ((eq41_e736_d_n11 * eq41_e742) + (eq41_e736 * eq41_e740_d_n11));
        let eq41_e743_d_n12: f64 = ((eq41_e736_d_n12 * eq41_e742) + (eq41_e736 * eq41_e740_d_n12));
        let eq41_e743_d_n13: f64 = ((eq41_e736_d_n13 * eq41_e742) + (eq41_e736 * eq41_e740_d_n13));
        let eq41_e743_d_n14: f64 = ((eq41_e736_d_n14 * eq41_e742) + (eq41_e736 * eq41_e740_d_n14));
        let eq41_e743_d_n15: f64 = ((eq41_e736_d_n15 * eq41_e742) + (eq41_e736 * eq41_e740_d_n15));
        let eq41_e743_d_n16: f64 = ((eq41_e736_d_n16 * eq41_e742) + (eq41_e736 * eq41_e740_d_n16));
        let eq41_e743_d_n17: f64 = ((eq41_e736_d_n17 * eq41_e742) + (eq41_e736 * eq41_e740_d_n17));
        let eq41_e743_d_n18: f64 = ((eq41_e736_d_n18 * eq41_e742) + (eq41_e736 * eq41_e740_d_n18));
        let eq41_e743_d_n19: f64 = ((eq41_e736_d_n19 * eq41_e742) + (eq41_e736 * eq41_e740_d_n19));
        let eq41_e743_d_n20: f64 = ((eq41_e736_d_n20 * eq41_e742) + (eq41_e736 * eq41_e740_d_n20));
        let eq41_e743_d_n21: f64 = ((eq41_e736_d_n21 * eq41_e742) + (eq41_e736 * eq41_e740_d_n21));
        let eq41_e743_d_n22: f64 = ((eq41_e736_d_n22 * eq41_e742) + (eq41_e736 * eq41_e740_d_n22));
        let eq41_e743_d_b0: f64 = ((eq41_e736_d_b0 * eq41_e742) + (eq41_e736 * eq41_e740_d_b0));
        let eq41_e743_d_b1: f64 = ((eq41_e736_d_b1 * eq41_e742) + (eq41_e736 * eq41_e740_d_b1));
        let eq41_e743_d_b2: f64 = ((eq41_e736_d_b2 * eq41_e742) + (eq41_e736 * eq41_e740_d_b2));
        let eq41_e743_d_b3: f64 = ((eq41_e736_d_b3 * eq41_e742) + (eq41_e736 * eq41_e740_d_b3));
        let eq41_e743_d_b4: f64 = ((eq41_e736_d_b4 * eq41_e742) + (eq41_e736 * eq41_e740_d_b4));
        let eq41_e743_d_b5: f64 = ((eq41_e736_d_b5 * eq41_e742) + (eq41_e736 * eq41_e740_d_b5));
        let eq41_e743_d_b6: f64 = ((eq41_e736_d_b6 * eq41_e742) + (eq41_e736 * eq41_e740_d_b6));
        let eq41_e743_d_b7: f64 = ((eq41_e736_d_b7 * eq41_e742) + (eq41_e736 * eq41_e740_d_b7));
        let eq41_e743_d_b8: f64 = ((eq41_e736_d_b8 * eq41_e742) + (eq41_e736 * eq41_e740_d_b8));
        let eq41_e743_d_b9: f64 = ((eq41_e736_d_b9 * eq41_e742) + (eq41_e736 * eq41_e740_d_b9));
        let eq41_e743_d_b10: f64 = ((eq41_e736_d_b10 * eq41_e742) + (eq41_e736 * eq41_e740_d_b10));
        let eq41_e743_d_b11: f64 = ((eq41_e736_d_b11 * eq41_e742) + (eq41_e736 * eq41_e740_d_b11));
        let eq41_e743_d_b12: f64 = ((eq41_e736_d_b12 * eq41_e742) + (eq41_e736 * eq41_e740_d_b12));
        let eq41_e743_d_b13: f64 = ((eq41_e736_d_b13 * eq41_e742) + (eq41_e736 * eq41_e740_d_b13));
        let eq41_e743_d_b14: f64 = ((eq41_e736_d_b14 * eq41_e742) + (eq41_e736 * eq41_e740_d_b14));
        let eq41_e743_d_b15: f64 = ((eq41_e736_d_b15 * eq41_e742) + (eq41_e736 * eq41_e740_d_b15));
        let eq41_e743_d_b16: f64 = ((eq41_e736_d_b16 * eq41_e742) + (eq41_e736 * eq41_e740_d_b16));
        let eq41_e743_d_b17: f64 = ((eq41_e736_d_b17 * eq41_e742) + (eq41_e736 * eq41_e740_d_b17));
        let eq41_e743_d_b18: f64 = ((eq41_e736_d_b18 * eq41_e742) + (eq41_e736 * eq41_e740_d_b18));
        let eq41_e743_d_b19: f64 = ((eq41_e736_d_b19 * eq41_e742) + (eq41_e736 * eq41_e740_d_b19));
        let eq41_e743_d_b20: f64 = ((eq41_e736_d_b20 * eq41_e742) + (eq41_e736 * eq41_e740_d_b20));
        let eq41_e743_d_b21: f64 = ((eq41_e736_d_b21 * eq41_e742) + (eq41_e736 * eq41_e740_d_b21));
        let eq41_e743_d_b22: f64 = ((eq41_e736_d_b22 * eq41_e742) + (eq41_e736 * eq41_e740_d_b22));
        let eq41_e743_d_b23: f64 = ((eq41_e736_d_b23 * eq41_e742) + (eq41_e736 * eq41_e740_d_b23));
        let eq41_e743_d_b24: f64 = ((eq41_e736_d_b24 * eq41_e742) + (eq41_e736 * eq41_e740_d_b24));
        let eq41_e743_d_b25: f64 = ((eq41_e736_d_b25 * eq41_e742) + (eq41_e736 * eq41_e740_d_b25));
        let eq41_e743_d_b26: f64 = ((eq41_e736_d_b26 * eq41_e742) + (eq41_e736 * eq41_e740_d_b26));
        let eq41_e743_d_b27: f64 = ((eq41_e736_d_b27 * eq41_e742) + (eq41_e736 * eq41_e740_d_b27));
        let eq41_e743_d_b28: f64 = ((eq41_e736_d_b28 * eq41_e742) + (eq41_e736 * eq41_e740_d_b28));
        let eq41_e743_d_b29: f64 = ((eq41_e736_d_b29 * eq41_e742) + (eq41_e736 * eq41_e740_d_b29));
        let eq41_e743_d_b30: f64 = ((eq41_e736_d_b30 * eq41_e742) + (eq41_e736 * eq41_e740_d_b30));
        let eq41_e743_d_b31: f64 = ((eq41_e736_d_b31 * eq41_e742) + (eq41_e736 * eq41_e740_d_b31));
        let eq41_e743_d_b32: f64 = ((eq41_e736_d_b32 * eq41_e742) + (eq41_e736 * eq41_e740_d_b32));
        let eq41_e743_d_b33: f64 = ((eq41_e736_d_b33 * eq41_e742) + (eq41_e736 * eq41_e740_d_b33));
        let eq41_e743_d_b34: f64 = ((eq41_e736_d_b34 * eq41_e742) + (eq41_e736 * eq41_e740_d_b34));
        let eq41_e743_d_b35: f64 = ((eq41_e736_d_b35 * eq41_e742) + (eq41_e736 * eq41_e740_d_b35));
        let eq41_e743_d_b36: f64 = ((eq41_e736_d_b36 * eq41_e742) + (eq41_e736 * eq41_e740_d_b36));
        let eq41_e743_d_b37: f64 = ((eq41_e736_d_b37 * eq41_e742) + (eq41_e736 * eq41_e740_d_b37));
        let eq41_e743_d_b38: f64 = ((eq41_e736_d_b38 * eq41_e742) + (eq41_e736 * eq41_e740_d_b38));
        let eq41_e743_d_b39: f64 = ((eq41_e736_d_b39 * eq41_e742) + (eq41_e736 * eq41_e740_d_b39));
        let eq41_e743_d_b40: f64 = ((eq41_e736_d_b40 * eq41_e742) + (eq41_e736 * eq41_e740_d_b40));
        let eq41_e743_d_b41: f64 = ((eq41_e736_d_b41 * eq41_e742) + (eq41_e736 * eq41_e740_d_b41));
        let eq41_e743_d_b42: f64 = ((eq41_e736_d_b42 * eq41_e742) + (eq41_e736 * eq41_e740_d_b42));
        let eq41_e743_d_b43: f64 = ((eq41_e736_d_b43 * eq41_e742) + (eq41_e736 * eq41_e740_d_b43));
        let eq41_e743_d_b44: f64 = ((eq41_e736_d_b44 * eq41_e742) + (eq41_e736 * eq41_e740_d_b44));
        let eq41_e743_d_b45: f64 = ((eq41_e736_d_b45 * eq41_e742) + (eq41_e736 * eq41_e740_d_b45));
        let eq41_e743_d_b46: f64 = ((eq41_e736_d_b46 * eq41_e742) + (eq41_e736 * eq41_e740_d_b46));
        let eq41_e743_d_b47: f64 = ((eq41_e736_d_b47 * eq41_e742) + (eq41_e736 * eq41_e740_d_b47));
        let eq41_e743_d_b48: f64 = ((eq41_e736_d_b48 * eq41_e742) + (eq41_e736 * eq41_e740_d_b48));
        let eq41_e743_d_b49: f64 = ((eq41_e736_d_b49 * eq41_e742) + (eq41_e736 * eq41_e740_d_b49));
        let eq41_e743_d_b50: f64 = ((eq41_e736_d_b50 * eq41_e742) + (eq41_e736 * eq41_e740_d_b50));
        let eq41_e743_d_b51: f64 = ((eq41_e736_d_b51 * eq41_e742) + (eq41_e736 * eq41_e740_d_b51));
        let eq41_e743_d_b52: f64 = ((eq41_e736_d_b52 * eq41_e742) + (eq41_e736 * eq41_e740_d_b52));
        let eq41_e743_d_b53: f64 = ((eq41_e736_d_b53 * eq41_e742) + (eq41_e736 * eq41_e740_d_b53));
        let eq41_e743_d_b54: f64 = ((eq41_e736_d_b54 * eq41_e742) + (eq41_e736 * eq41_e740_d_b54));
        let eq41_e745: f64 = (eq41_e743 * 0.5);
        let eq41_e745_d_n0: f64 = (eq41_e743_d_n0 * 0.5);
        let eq41_e745_d_n1: f64 = (eq41_e743_d_n1 * 0.5);
        let eq41_e745_d_n2: f64 = (eq41_e743_d_n2 * 0.5);
        let eq41_e745_d_n3: f64 = (eq41_e743_d_n3 * 0.5);
        let eq41_e745_d_n4: f64 = (eq41_e743_d_n4 * 0.5);
        let eq41_e745_d_n5: f64 = (eq41_e743_d_n5 * 0.5);
        let eq41_e745_d_n6: f64 = (eq41_e743_d_n6 * 0.5);
        let eq41_e745_d_n7: f64 = (eq41_e743_d_n7 * 0.5);
        let eq41_e745_d_n8: f64 = (eq41_e743_d_n8 * 0.5);
        let eq41_e745_d_n9: f64 = (eq41_e743_d_n9 * 0.5);
        let eq41_e745_d_n10: f64 = (eq41_e743_d_n10 * 0.5);
        let eq41_e745_d_n11: f64 = (eq41_e743_d_n11 * 0.5);
        let eq41_e745_d_n12: f64 = (eq41_e743_d_n12 * 0.5);
        let eq41_e745_d_n13: f64 = (eq41_e743_d_n13 * 0.5);
        let eq41_e745_d_n14: f64 = (eq41_e743_d_n14 * 0.5);
        let eq41_e745_d_n15: f64 = (eq41_e743_d_n15 * 0.5);
        let eq41_e745_d_n16: f64 = (eq41_e743_d_n16 * 0.5);
        let eq41_e745_d_n17: f64 = (eq41_e743_d_n17 * 0.5);
        let eq41_e745_d_n18: f64 = (eq41_e743_d_n18 * 0.5);
        let eq41_e745_d_n19: f64 = (eq41_e743_d_n19 * 0.5);
        let eq41_e745_d_n20: f64 = (eq41_e743_d_n20 * 0.5);
        let eq41_e745_d_n21: f64 = (eq41_e743_d_n21 * 0.5);
        let eq41_e745_d_n22: f64 = (eq41_e743_d_n22 * 0.5);
        let eq41_e745_d_b0: f64 = (eq41_e743_d_b0 * 0.5);
        let eq41_e745_d_b1: f64 = (eq41_e743_d_b1 * 0.5);
        let eq41_e745_d_b2: f64 = (eq41_e743_d_b2 * 0.5);
        let eq41_e745_d_b3: f64 = (eq41_e743_d_b3 * 0.5);
        let eq41_e745_d_b4: f64 = (eq41_e743_d_b4 * 0.5);
        let eq41_e745_d_b5: f64 = (eq41_e743_d_b5 * 0.5);
        let eq41_e745_d_b6: f64 = (eq41_e743_d_b6 * 0.5);
        let eq41_e745_d_b7: f64 = (eq41_e743_d_b7 * 0.5);
        let eq41_e745_d_b8: f64 = (eq41_e743_d_b8 * 0.5);
        let eq41_e745_d_b9: f64 = (eq41_e743_d_b9 * 0.5);
        let eq41_e745_d_b10: f64 = (eq41_e743_d_b10 * 0.5);
        let eq41_e745_d_b11: f64 = (eq41_e743_d_b11 * 0.5);
        let eq41_e745_d_b12: f64 = (eq41_e743_d_b12 * 0.5);
        let eq41_e745_d_b13: f64 = (eq41_e743_d_b13 * 0.5);
        let eq41_e745_d_b14: f64 = (eq41_e743_d_b14 * 0.5);
        let eq41_e745_d_b15: f64 = (eq41_e743_d_b15 * 0.5);
        let eq41_e745_d_b16: f64 = (eq41_e743_d_b16 * 0.5);
        let eq41_e745_d_b17: f64 = (eq41_e743_d_b17 * 0.5);
        let eq41_e745_d_b18: f64 = (eq41_e743_d_b18 * 0.5);
        let eq41_e745_d_b19: f64 = (eq41_e743_d_b19 * 0.5);
        let eq41_e745_d_b20: f64 = (eq41_e743_d_b20 * 0.5);
        let eq41_e745_d_b21: f64 = (eq41_e743_d_b21 * 0.5);
        let eq41_e745_d_b22: f64 = (eq41_e743_d_b22 * 0.5);
        let eq41_e745_d_b23: f64 = (eq41_e743_d_b23 * 0.5);
        let eq41_e745_d_b24: f64 = (eq41_e743_d_b24 * 0.5);
        let eq41_e745_d_b25: f64 = (eq41_e743_d_b25 * 0.5);
        let eq41_e745_d_b26: f64 = (eq41_e743_d_b26 * 0.5);
        let eq41_e745_d_b27: f64 = (eq41_e743_d_b27 * 0.5);
        let eq41_e745_d_b28: f64 = (eq41_e743_d_b28 * 0.5);
        let eq41_e745_d_b29: f64 = (eq41_e743_d_b29 * 0.5);
        let eq41_e745_d_b30: f64 = (eq41_e743_d_b30 * 0.5);
        let eq41_e745_d_b31: f64 = (eq41_e743_d_b31 * 0.5);
        let eq41_e745_d_b32: f64 = (eq41_e743_d_b32 * 0.5);
        let eq41_e745_d_b33: f64 = (eq41_e743_d_b33 * 0.5);
        let eq41_e745_d_b34: f64 = (eq41_e743_d_b34 * 0.5);
        let eq41_e745_d_b35: f64 = (eq41_e743_d_b35 * 0.5);
        let eq41_e745_d_b36: f64 = (eq41_e743_d_b36 * 0.5);
        let eq41_e745_d_b37: f64 = (eq41_e743_d_b37 * 0.5);
        let eq41_e745_d_b38: f64 = (eq41_e743_d_b38 * 0.5);
        let eq41_e745_d_b39: f64 = (eq41_e743_d_b39 * 0.5);
        let eq41_e745_d_b40: f64 = (eq41_e743_d_b40 * 0.5);
        let eq41_e745_d_b41: f64 = (eq41_e743_d_b41 * 0.5);
        let eq41_e745_d_b42: f64 = (eq41_e743_d_b42 * 0.5);
        let eq41_e745_d_b43: f64 = (eq41_e743_d_b43 * 0.5);
        let eq41_e745_d_b44: f64 = (eq41_e743_d_b44 * 0.5);
        let eq41_e745_d_b45: f64 = (eq41_e743_d_b45 * 0.5);
        let eq41_e745_d_b46: f64 = (eq41_e743_d_b46 * 0.5);
        let eq41_e745_d_b47: f64 = (eq41_e743_d_b47 * 0.5);
        let eq41_e745_d_b48: f64 = (eq41_e743_d_b48 * 0.5);
        let eq41_e745_d_b49: f64 = (eq41_e743_d_b49 * 0.5);
        let eq41_e745_d_b50: f64 = (eq41_e743_d_b50 * 0.5);
        let eq41_e745_d_b51: f64 = (eq41_e743_d_b51 * 0.5);
        let eq41_e745_d_b52: f64 = (eq41_e743_d_b52 * 0.5);
        let eq41_e745_d_b53: f64 = (eq41_e743_d_b53 * 0.5);
        let eq41_e745_d_b54: f64 = (eq41_e743_d_b54 * 0.5);
        (eq41_e745, eq41_e745_d_n0, eq41_e745_d_n1, eq41_e745_d_n2, eq41_e745_d_n3, eq41_e745_d_n4, eq41_e745_d_n5, eq41_e745_d_n6, eq41_e745_d_n7, eq41_e745_d_n8, eq41_e745_d_n9, eq41_e745_d_n10, eq41_e745_d_n11, eq41_e745_d_n12, eq41_e745_d_n13, eq41_e745_d_n14, eq41_e745_d_n15, eq41_e745_d_n16, eq41_e745_d_n17, eq41_e745_d_n18, eq41_e745_d_n19, eq41_e745_d_n20, eq41_e745_d_n21, eq41_e745_d_n22, eq41_e745_d_b0, eq41_e745_d_b1, eq41_e745_d_b2, eq41_e745_d_b3, eq41_e745_d_b4, eq41_e745_d_b5, eq41_e745_d_b6, eq41_e745_d_b7, eq41_e745_d_b8, eq41_e745_d_b9, eq41_e745_d_b10, eq41_e745_d_b11, eq41_e745_d_b12, eq41_e745_d_b13, eq41_e745_d_b14, eq41_e745_d_b15, eq41_e745_d_b16, eq41_e745_d_b17, eq41_e745_d_b18, eq41_e745_d_b19, eq41_e745_d_b20, eq41_e745_d_b21, eq41_e745_d_b22, eq41_e745_d_b23, eq41_e745_d_b24, eq41_e745_d_b25, eq41_e745_d_b26, eq41_e745_d_b27, eq41_e745_d_b28, eq41_e745_d_b29, eq41_e745_d_b30, eq41_e745_d_b31, eq41_e745_d_b32, eq41_e745_d_b33, eq41_e745_d_b34, eq41_e745_d_b35, eq41_e745_d_b36, eq41_e745_d_b37, eq41_e745_d_b38, eq41_e745_d_b39, eq41_e745_d_b40, eq41_e745_d_b41, eq41_e745_d_b42, eq41_e745_d_b43, eq41_e745_d_b44, eq41_e745_d_b45, eq41_e745_d_b46, eq41_e745_d_b47, eq41_e745_d_b48, eq41_e745_d_b49, eq41_e745_d_b50, eq41_e745_d_b51, eq41_e745_d_b52, eq41_e745_d_b53, eq41_e745_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_value: f64 = eq41_e747;
        let eq41_node_derivatives: [f64; 23] = [eq41_e747_d_n0, eq41_e747_d_n1, eq41_e747_d_n2, eq41_e747_d_n3, eq41_e747_d_n4, eq41_e747_d_n5, eq41_e747_d_n6, eq41_e747_d_n7, eq41_e747_d_n8, eq41_e747_d_n9, eq41_e747_d_n10, eq41_e747_d_n11, eq41_e747_d_n12, eq41_e747_d_n13, eq41_e747_d_n14, eq41_e747_d_n15, eq41_e747_d_n16, eq41_e747_d_n17, eq41_e747_d_n18, eq41_e747_d_n19, eq41_e747_d_n20, eq41_e747_d_n21, eq41_e747_d_n22];
        let eq41_branch_derivatives: [f64; 55] = [eq41_e747_d_b0, eq41_e747_d_b1, eq41_e747_d_b2, eq41_e747_d_b3, eq41_e747_d_b4, eq41_e747_d_b5, eq41_e747_d_b6, eq41_e747_d_b7, eq41_e747_d_b8, eq41_e747_d_b9, eq41_e747_d_b10, eq41_e747_d_b11, eq41_e747_d_b12, eq41_e747_d_b13, eq41_e747_d_b14, eq41_e747_d_b15, eq41_e747_d_b16, eq41_e747_d_b17, eq41_e747_d_b18, eq41_e747_d_b19, eq41_e747_d_b20, eq41_e747_d_b21, eq41_e747_d_b22, eq41_e747_d_b23, eq41_e747_d_b24, eq41_e747_d_b25, eq41_e747_d_b26, eq41_e747_d_b27, eq41_e747_d_b28, eq41_e747_d_b29, eq41_e747_d_b30, eq41_e747_d_b31, eq41_e747_d_b32, eq41_e747_d_b33, eq41_e747_d_b34, eq41_e747_d_b35, eq41_e747_d_b36, eq41_e747_d_b37, eq41_e747_d_b38, eq41_e747_d_b39, eq41_e747_d_b40, eq41_e747_d_b41, eq41_e747_d_b42, eq41_e747_d_b43, eq41_e747_d_b44, eq41_e747_d_b45, eq41_e747_d_b46, eq41_e747_d_b47, eq41_e747_d_b48, eq41_e747_d_b49, eq41_e747_d_b50, eq41_e747_d_b51, eq41_e747_d_b52, eq41_e747_d_b53, eq41_e747_d_b54];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq42_e766, eq42_e766_d_n0, eq42_e766_d_n1, eq42_e766_d_n2, eq42_e766_d_n3, eq42_e766_d_n4, eq42_e766_d_n5, eq42_e766_d_n6, eq42_e766_d_n7, eq42_e766_d_n8, eq42_e766_d_n9, eq42_e766_d_n10, eq42_e766_d_n11, eq42_e766_d_n12, eq42_e766_d_n13, eq42_e766_d_n14, eq42_e766_d_n15, eq42_e766_d_n16, eq42_e766_d_n17, eq42_e766_d_n18, eq42_e766_d_n19, eq42_e766_d_n20, eq42_e766_d_n21, eq42_e766_d_n22, eq42_e766_d_b0, eq42_e766_d_b1, eq42_e766_d_b2, eq42_e766_d_b3, eq42_e766_d_b4, eq42_e766_d_b5, eq42_e766_d_b6, eq42_e766_d_b7, eq42_e766_d_b8, eq42_e766_d_b9, eq42_e766_d_b10, eq42_e766_d_b11, eq42_e766_d_b12, eq42_e766_d_b13, eq42_e766_d_b14, eq42_e766_d_b15, eq42_e766_d_b16, eq42_e766_d_b17, eq42_e766_d_b18, eq42_e766_d_b19, eq42_e766_d_b20, eq42_e766_d_b21, eq42_e766_d_b22, eq42_e766_d_b23, eq42_e766_d_b24, eq42_e766_d_b25, eq42_e766_d_b26, eq42_e766_d_b27, eq42_e766_d_b28, eq42_e766_d_b29, eq42_e766_d_b30, eq42_e766_d_b31, eq42_e766_d_b32, eq42_e766_d_b33, eq42_e766_d_b34, eq42_e766_d_b35, eq42_e766_d_b36, eq42_e766_d_b37, eq42_e766_d_b38, eq42_e766_d_b39, eq42_e766_d_b40, eq42_e766_d_b41, eq42_e766_d_b42, eq42_e766_d_b43, eq42_e766_d_b44, eq42_e766_d_b45, eq42_e766_d_b46, eq42_e766_d_b47, eq42_e766_d_b48, eq42_e766_d_b49, eq42_e766_d_b50, eq42_e766_d_b51, eq42_e766_d_b52, eq42_e766_d_b53, eq42_e766_d_b54,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        let eq42_e762: f64 = (p.p135 * s.v[363]);
        let eq42_e762_d_n0: f64 = (p.p135 * s.dn[363][0]);
        let eq42_e762_d_n1: f64 = (p.p135 * s.dn[363][1]);
        let eq42_e762_d_n2: f64 = (p.p135 * s.dn[363][2]);
        let eq42_e762_d_n3: f64 = (p.p135 * s.dn[363][3]);
        let eq42_e762_d_n4: f64 = (p.p135 * s.dn[363][4]);
        let eq42_e762_d_n5: f64 = (p.p135 * s.dn[363][5]);
        let eq42_e762_d_n6: f64 = (p.p135 * s.dn[363][6]);
        let eq42_e762_d_n7: f64 = (p.p135 * s.dn[363][7]);
        let eq42_e762_d_n8: f64 = (p.p135 * s.dn[363][8]);
        let eq42_e762_d_n9: f64 = (p.p135 * s.dn[363][9]);
        let eq42_e762_d_n10: f64 = (p.p135 * s.dn[363][10]);
        let eq42_e762_d_n11: f64 = (p.p135 * s.dn[363][11]);
        let eq42_e762_d_n12: f64 = (p.p135 * s.dn[363][12]);
        let eq42_e762_d_n13: f64 = (p.p135 * s.dn[363][13]);
        let eq42_e762_d_n14: f64 = (p.p135 * s.dn[363][14]);
        let eq42_e762_d_n15: f64 = (p.p135 * s.dn[363][15]);
        let eq42_e762_d_n16: f64 = (p.p135 * s.dn[363][16]);
        let eq42_e762_d_n17: f64 = (p.p135 * s.dn[363][17]);
        let eq42_e762_d_n18: f64 = (p.p135 * s.dn[363][18]);
        let eq42_e762_d_n19: f64 = (p.p135 * s.dn[363][19]);
        let eq42_e762_d_n20: f64 = (p.p135 * s.dn[363][20]);
        let eq42_e762_d_n21: f64 = (p.p135 * s.dn[363][21]);
        let eq42_e762_d_n22: f64 = (p.p135 * s.dn[363][22]);
        let eq42_e762_d_b0: f64 = (p.p135 * s.db[363][0]);
        let eq42_e762_d_b1: f64 = (p.p135 * s.db[363][1]);
        let eq42_e762_d_b2: f64 = (p.p135 * s.db[363][2]);
        let eq42_e762_d_b3: f64 = (p.p135 * s.db[363][3]);
        let eq42_e762_d_b4: f64 = (p.p135 * s.db[363][4]);
        let eq42_e762_d_b5: f64 = (p.p135 * s.db[363][5]);
        let eq42_e762_d_b6: f64 = (p.p135 * s.db[363][6]);
        let eq42_e762_d_b7: f64 = (p.p135 * s.db[363][7]);
        let eq42_e762_d_b8: f64 = (p.p135 * s.db[363][8]);
        let eq42_e762_d_b9: f64 = (p.p135 * s.db[363][9]);
        let eq42_e762_d_b10: f64 = (p.p135 * s.db[363][10]);
        let eq42_e762_d_b11: f64 = (p.p135 * s.db[363][11]);
        let eq42_e762_d_b12: f64 = (p.p135 * s.db[363][12]);
        let eq42_e762_d_b13: f64 = (p.p135 * s.db[363][13]);
        let eq42_e762_d_b14: f64 = (p.p135 * s.db[363][14]);
        let eq42_e762_d_b15: f64 = (p.p135 * s.db[363][15]);
        let eq42_e762_d_b16: f64 = (p.p135 * s.db[363][16]);
        let eq42_e762_d_b17: f64 = (p.p135 * s.db[363][17]);
        let eq42_e762_d_b18: f64 = (p.p135 * s.db[363][18]);
        let eq42_e762_d_b19: f64 = (p.p135 * s.db[363][19]);
        let eq42_e762_d_b20: f64 = (p.p135 * s.db[363][20]);
        let eq42_e762_d_b21: f64 = (p.p135 * s.db[363][21]);
        let eq42_e762_d_b22: f64 = (p.p135 * s.db[363][22]);
        let eq42_e762_d_b23: f64 = (p.p135 * s.db[363][23]);
        let eq42_e762_d_b24: f64 = (p.p135 * s.db[363][24]);
        let eq42_e762_d_b25: f64 = (p.p135 * s.db[363][25]);
        let eq42_e762_d_b26: f64 = (p.p135 * s.db[363][26]);
        let eq42_e762_d_b27: f64 = (p.p135 * s.db[363][27]);
        let eq42_e762_d_b28: f64 = (p.p135 * s.db[363][28]);
        let eq42_e762_d_b29: f64 = (p.p135 * s.db[363][29]);
        let eq42_e762_d_b30: f64 = (p.p135 * s.db[363][30]);
        let eq42_e762_d_b31: f64 = (p.p135 * s.db[363][31]);
        let eq42_e762_d_b32: f64 = (p.p135 * s.db[363][32]);
        let eq42_e762_d_b33: f64 = (p.p135 * s.db[363][33]);
        let eq42_e762_d_b34: f64 = (p.p135 * s.db[363][34]);
        let eq42_e762_d_b35: f64 = (p.p135 * s.db[363][35]);
        let eq42_e762_d_b36: f64 = (p.p135 * s.db[363][36]);
        let eq42_e762_d_b37: f64 = (p.p135 * s.db[363][37]);
        let eq42_e762_d_b38: f64 = (p.p135 * s.db[363][38]);
        let eq42_e762_d_b39: f64 = (p.p135 * s.db[363][39]);
        let eq42_e762_d_b40: f64 = (p.p135 * s.db[363][40]);
        let eq42_e762_d_b41: f64 = (p.p135 * s.db[363][41]);
        let eq42_e762_d_b42: f64 = (p.p135 * s.db[363][42]);
        let eq42_e762_d_b43: f64 = (p.p135 * s.db[363][43]);
        let eq42_e762_d_b44: f64 = (p.p135 * s.db[363][44]);
        let eq42_e762_d_b45: f64 = (p.p135 * s.db[363][45]);
        let eq42_e762_d_b46: f64 = (p.p135 * s.db[363][46]);
        let eq42_e762_d_b47: f64 = (p.p135 * s.db[363][47]);
        let eq42_e762_d_b48: f64 = (p.p135 * s.db[363][48]);
        let eq42_e762_d_b49: f64 = (p.p135 * s.db[363][49]);
        let eq42_e762_d_b50: f64 = (p.p135 * s.db[363][50]);
        let eq42_e762_d_b51: f64 = (p.p135 * s.db[363][51]);
        let eq42_e762_d_b52: f64 = (p.p135 * s.db[363][52]);
        let eq42_e762_d_b53: f64 = (p.p135 * s.db[363][53]);
        let eq42_e762_d_b54: f64 = (p.p135 * s.db[363][54]);
        let eq42_e764: f64 = (eq42_e762 * (nv5 - 0.0));
        let eq42_e764_d_n0: f64 = (eq42_e762_d_n0 * (nv5 - 0.0));
        let eq42_e764_d_n1: f64 = (eq42_e762_d_n1 * (nv5 - 0.0));
        let eq42_e764_d_n2: f64 = (eq42_e762_d_n2 * (nv5 - 0.0));
        let eq42_e764_d_n3: f64 = (eq42_e762_d_n3 * (nv5 - 0.0));
        let eq42_e764_d_n4: f64 = (eq42_e762_d_n4 * (nv5 - 0.0));
        let eq42_e764_d_n5: f64 = ((eq42_e762_d_n5 * (nv5 - 0.0)) + eq42_e762);
        let eq42_e764_d_n6: f64 = (eq42_e762_d_n6 * (nv5 - 0.0));
        let eq42_e764_d_n7: f64 = (eq42_e762_d_n7 * (nv5 - 0.0));
        let eq42_e764_d_n8: f64 = (eq42_e762_d_n8 * (nv5 - 0.0));
        let eq42_e764_d_n9: f64 = (eq42_e762_d_n9 * (nv5 - 0.0));
        let eq42_e764_d_n10: f64 = (eq42_e762_d_n10 * (nv5 - 0.0));
        let eq42_e764_d_n11: f64 = (eq42_e762_d_n11 * (nv5 - 0.0));
        let eq42_e764_d_n12: f64 = (eq42_e762_d_n12 * (nv5 - 0.0));
        let eq42_e764_d_n13: f64 = (eq42_e762_d_n13 * (nv5 - 0.0));
        let eq42_e764_d_n14: f64 = (eq42_e762_d_n14 * (nv5 - 0.0));
        let eq42_e764_d_n15: f64 = (eq42_e762_d_n15 * (nv5 - 0.0));
        let eq42_e764_d_n16: f64 = (eq42_e762_d_n16 * (nv5 - 0.0));
        let eq42_e764_d_n17: f64 = (eq42_e762_d_n17 * (nv5 - 0.0));
        let eq42_e764_d_n18: f64 = (eq42_e762_d_n18 * (nv5 - 0.0));
        let eq42_e764_d_n19: f64 = (eq42_e762_d_n19 * (nv5 - 0.0));
        let eq42_e764_d_n20: f64 = (eq42_e762_d_n20 * (nv5 - 0.0));
        let eq42_e764_d_n21: f64 = (eq42_e762_d_n21 * (nv5 - 0.0));
        let eq42_e764_d_n22: f64 = (eq42_e762_d_n22 * (nv5 - 0.0));
        let eq42_e764_d_b0: f64 = (eq42_e762_d_b0 * (nv5 - 0.0));
        let eq42_e764_d_b1: f64 = (eq42_e762_d_b1 * (nv5 - 0.0));
        let eq42_e764_d_b2: f64 = (eq42_e762_d_b2 * (nv5 - 0.0));
        let eq42_e764_d_b3: f64 = (eq42_e762_d_b3 * (nv5 - 0.0));
        let eq42_e764_d_b4: f64 = (eq42_e762_d_b4 * (nv5 - 0.0));
        let eq42_e764_d_b5: f64 = (eq42_e762_d_b5 * (nv5 - 0.0));
        let eq42_e764_d_b6: f64 = (eq42_e762_d_b6 * (nv5 - 0.0));
        let eq42_e764_d_b7: f64 = (eq42_e762_d_b7 * (nv5 - 0.0));
        let eq42_e764_d_b8: f64 = (eq42_e762_d_b8 * (nv5 - 0.0));
        let eq42_e764_d_b9: f64 = (eq42_e762_d_b9 * (nv5 - 0.0));
        let eq42_e764_d_b10: f64 = (eq42_e762_d_b10 * (nv5 - 0.0));
        let eq42_e764_d_b11: f64 = (eq42_e762_d_b11 * (nv5 - 0.0));
        let eq42_e764_d_b12: f64 = (eq42_e762_d_b12 * (nv5 - 0.0));
        let eq42_e764_d_b13: f64 = (eq42_e762_d_b13 * (nv5 - 0.0));
        let eq42_e764_d_b14: f64 = (eq42_e762_d_b14 * (nv5 - 0.0));
        let eq42_e764_d_b15: f64 = (eq42_e762_d_b15 * (nv5 - 0.0));
        let eq42_e764_d_b16: f64 = (eq42_e762_d_b16 * (nv5 - 0.0));
        let eq42_e764_d_b17: f64 = (eq42_e762_d_b17 * (nv5 - 0.0));
        let eq42_e764_d_b18: f64 = (eq42_e762_d_b18 * (nv5 - 0.0));
        let eq42_e764_d_b19: f64 = (eq42_e762_d_b19 * (nv5 - 0.0));
        let eq42_e764_d_b20: f64 = (eq42_e762_d_b20 * (nv5 - 0.0));
        let eq42_e764_d_b21: f64 = (eq42_e762_d_b21 * (nv5 - 0.0));
        let eq42_e764_d_b22: f64 = (eq42_e762_d_b22 * (nv5 - 0.0));
        let eq42_e764_d_b23: f64 = (eq42_e762_d_b23 * (nv5 - 0.0));
        let eq42_e764_d_b24: f64 = (eq42_e762_d_b24 * (nv5 - 0.0));
        let eq42_e764_d_b25: f64 = (eq42_e762_d_b25 * (nv5 - 0.0));
        let eq42_e764_d_b26: f64 = (eq42_e762_d_b26 * (nv5 - 0.0));
        let eq42_e764_d_b27: f64 = (eq42_e762_d_b27 * (nv5 - 0.0));
        let eq42_e764_d_b28: f64 = (eq42_e762_d_b28 * (nv5 - 0.0));
        let eq42_e764_d_b29: f64 = (eq42_e762_d_b29 * (nv5 - 0.0));
        let eq42_e764_d_b30: f64 = (eq42_e762_d_b30 * (nv5 - 0.0));
        let eq42_e764_d_b31: f64 = (eq42_e762_d_b31 * (nv5 - 0.0));
        let eq42_e764_d_b32: f64 = (eq42_e762_d_b32 * (nv5 - 0.0));
        let eq42_e764_d_b33: f64 = (eq42_e762_d_b33 * (nv5 - 0.0));
        let eq42_e764_d_b34: f64 = (eq42_e762_d_b34 * (nv5 - 0.0));
        let eq42_e764_d_b35: f64 = (eq42_e762_d_b35 * (nv5 - 0.0));
        let eq42_e764_d_b36: f64 = (eq42_e762_d_b36 * (nv5 - 0.0));
        let eq42_e764_d_b37: f64 = (eq42_e762_d_b37 * (nv5 - 0.0));
        let eq42_e764_d_b38: f64 = (eq42_e762_d_b38 * (nv5 - 0.0));
        let eq42_e764_d_b39: f64 = (eq42_e762_d_b39 * (nv5 - 0.0));
        let eq42_e764_d_b40: f64 = (eq42_e762_d_b40 * (nv5 - 0.0));
        let eq42_e764_d_b41: f64 = (eq42_e762_d_b41 * (nv5 - 0.0));
        let eq42_e764_d_b42: f64 = (eq42_e762_d_b42 * (nv5 - 0.0));
        let eq42_e764_d_b43: f64 = (eq42_e762_d_b43 * (nv5 - 0.0));
        let eq42_e764_d_b44: f64 = (eq42_e762_d_b44 * (nv5 - 0.0));
        let eq42_e764_d_b45: f64 = (eq42_e762_d_b45 * (nv5 - 0.0));
        let eq42_e764_d_b46: f64 = (eq42_e762_d_b46 * (nv5 - 0.0));
        let eq42_e764_d_b47: f64 = (eq42_e762_d_b47 * (nv5 - 0.0));
        let eq42_e764_d_b48: f64 = (eq42_e762_d_b48 * (nv5 - 0.0));
        let eq42_e764_d_b49: f64 = (eq42_e762_d_b49 * (nv5 - 0.0));
        let eq42_e764_d_b50: f64 = (eq42_e762_d_b50 * (nv5 - 0.0));
        let eq42_e764_d_b51: f64 = (eq42_e762_d_b51 * (nv5 - 0.0));
        let eq42_e764_d_b52: f64 = (eq42_e762_d_b52 * (nv5 - 0.0));
        let eq42_e764_d_b53: f64 = (eq42_e762_d_b53 * (nv5 - 0.0));
        let eq42_e764_d_b54: f64 = (eq42_e762_d_b54 * (nv5 - 0.0));
        (eq42_e764, eq42_e764_d_n0, eq42_e764_d_n1, eq42_e764_d_n2, eq42_e764_d_n3, eq42_e764_d_n4, eq42_e764_d_n5, eq42_e764_d_n6, eq42_e764_d_n7, eq42_e764_d_n8, eq42_e764_d_n9, eq42_e764_d_n10, eq42_e764_d_n11, eq42_e764_d_n12, eq42_e764_d_n13, eq42_e764_d_n14, eq42_e764_d_n15, eq42_e764_d_n16, eq42_e764_d_n17, eq42_e764_d_n18, eq42_e764_d_n19, eq42_e764_d_n20, eq42_e764_d_n21, eq42_e764_d_n22, eq42_e764_d_b0, eq42_e764_d_b1, eq42_e764_d_b2, eq42_e764_d_b3, eq42_e764_d_b4, eq42_e764_d_b5, eq42_e764_d_b6, eq42_e764_d_b7, eq42_e764_d_b8, eq42_e764_d_b9, eq42_e764_d_b10, eq42_e764_d_b11, eq42_e764_d_b12, eq42_e764_d_b13, eq42_e764_d_b14, eq42_e764_d_b15, eq42_e764_d_b16, eq42_e764_d_b17, eq42_e764_d_b18, eq42_e764_d_b19, eq42_e764_d_b20, eq42_e764_d_b21, eq42_e764_d_b22, eq42_e764_d_b23, eq42_e764_d_b24, eq42_e764_d_b25, eq42_e764_d_b26, eq42_e764_d_b27, eq42_e764_d_b28, eq42_e764_d_b29, eq42_e764_d_b30, eq42_e764_d_b31, eq42_e764_d_b32, eq42_e764_d_b33, eq42_e764_d_b34, eq42_e764_d_b35, eq42_e764_d_b36, eq42_e764_d_b37, eq42_e764_d_b38, eq42_e764_d_b39, eq42_e764_d_b40, eq42_e764_d_b41, eq42_e764_d_b42, eq42_e764_d_b43, eq42_e764_d_b44, eq42_e764_d_b45, eq42_e764_d_b46, eq42_e764_d_b47, eq42_e764_d_b48, eq42_e764_d_b49, eq42_e764_d_b50, eq42_e764_d_b51, eq42_e764_d_b52, eq42_e764_d_b53, eq42_e764_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_value: f64 = eq42_e766;
        let eq42_node_derivatives: [f64; 23] = [eq42_e766_d_n0, eq42_e766_d_n1, eq42_e766_d_n2, eq42_e766_d_n3, eq42_e766_d_n4, eq42_e766_d_n5, eq42_e766_d_n6, eq42_e766_d_n7, eq42_e766_d_n8, eq42_e766_d_n9, eq42_e766_d_n10, eq42_e766_d_n11, eq42_e766_d_n12, eq42_e766_d_n13, eq42_e766_d_n14, eq42_e766_d_n15, eq42_e766_d_n16, eq42_e766_d_n17, eq42_e766_d_n18, eq42_e766_d_n19, eq42_e766_d_n20, eq42_e766_d_n21, eq42_e766_d_n22];
        let eq42_branch_derivatives: [f64; 55] = [eq42_e766_d_b0, eq42_e766_d_b1, eq42_e766_d_b2, eq42_e766_d_b3, eq42_e766_d_b4, eq42_e766_d_b5, eq42_e766_d_b6, eq42_e766_d_b7, eq42_e766_d_b8, eq42_e766_d_b9, eq42_e766_d_b10, eq42_e766_d_b11, eq42_e766_d_b12, eq42_e766_d_b13, eq42_e766_d_b14, eq42_e766_d_b15, eq42_e766_d_b16, eq42_e766_d_b17, eq42_e766_d_b18, eq42_e766_d_b19, eq42_e766_d_b20, eq42_e766_d_b21, eq42_e766_d_b22, eq42_e766_d_b23, eq42_e766_d_b24, eq42_e766_d_b25, eq42_e766_d_b26, eq42_e766_d_b27, eq42_e766_d_b28, eq42_e766_d_b29, eq42_e766_d_b30, eq42_e766_d_b31, eq42_e766_d_b32, eq42_e766_d_b33, eq42_e766_d_b34, eq42_e766_d_b35, eq42_e766_d_b36, eq42_e766_d_b37, eq42_e766_d_b38, eq42_e766_d_b39, eq42_e766_d_b40, eq42_e766_d_b41, eq42_e766_d_b42, eq42_e766_d_b43, eq42_e766_d_b44, eq42_e766_d_b45, eq42_e766_d_b46, eq42_e766_d_b47, eq42_e766_d_b48, eq42_e766_d_b49, eq42_e766_d_b50, eq42_e766_d_b51, eq42_e766_d_b52, eq42_e766_d_b53, eq42_e766_d_b54];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq42_value),
            &eq42_node_derivatives,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let (eq43_e784, eq43_e784_d_n5,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        let eq43_e781: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, (nv5 - 0.0));
        let eq43_e782: f64 = (p.p135 * eq43_e781);
        (eq43_e782, (p.p135 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq43_value: f64 = eq43_e784;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (eq43_value),
            5,
            multiplicity * (eq43_e784_d_n5),
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq44_e815, eq44_e815_d_n0, eq44_e815_d_n1, eq44_e815_d_n2, eq44_e815_d_n3, eq44_e815_d_n4, eq44_e815_d_n5, eq44_e815_d_n6, eq44_e815_d_n7, eq44_e815_d_n8, eq44_e815_d_n9, eq44_e815_d_n10, eq44_e815_d_n11, eq44_e815_d_n12, eq44_e815_d_n13, eq44_e815_d_n14, eq44_e815_d_n15, eq44_e815_d_n16, eq44_e815_d_n17, eq44_e815_d_n18, eq44_e815_d_n19, eq44_e815_d_n20, eq44_e815_d_n21, eq44_e815_d_n22, eq44_e815_d_b0, eq44_e815_d_b1, eq44_e815_d_b2, eq44_e815_d_b3, eq44_e815_d_b4, eq44_e815_d_b5, eq44_e815_d_b6, eq44_e815_d_b7, eq44_e815_d_b8, eq44_e815_d_b9, eq44_e815_d_b10, eq44_e815_d_b11, eq44_e815_d_b12, eq44_e815_d_b13, eq44_e815_d_b14, eq44_e815_d_b15, eq44_e815_d_b16, eq44_e815_d_b17, eq44_e815_d_b18, eq44_e815_d_b19, eq44_e815_d_b20, eq44_e815_d_b21, eq44_e815_d_b22, eq44_e815_d_b23, eq44_e815_d_b24, eq44_e815_d_b25, eq44_e815_d_b26, eq44_e815_d_b27, eq44_e815_d_b28, eq44_e815_d_b29, eq44_e815_d_b30, eq44_e815_d_b31, eq44_e815_d_b32, eq44_e815_d_b33, eq44_e815_d_b34, eq44_e815_d_b35, eq44_e815_d_b36, eq44_e815_d_b37, eq44_e815_d_b38, eq44_e815_d_b39, eq44_e815_d_b40, eq44_e815_d_b41, eq44_e815_d_b42, eq44_e815_d_b43, eq44_e815_d_b44, eq44_e815_d_b45, eq44_e815_d_b46, eq44_e815_d_b47, eq44_e815_d_b48, eq44_e815_d_b49, eq44_e815_d_b50, eq44_e815_d_b51, eq44_e815_d_b52, eq44_e815_d_b53, eq44_e815_d_b54,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        let eq44_e798: f64 = (-p.p144);
        let eq44_e800: f64 = (eq44_e798 * s.v[367]);
        let eq44_e800_d_n0: f64 = (eq44_e798 * s.dn[367][0]);
        let eq44_e800_d_n1: f64 = (eq44_e798 * s.dn[367][1]);
        let eq44_e800_d_n2: f64 = (eq44_e798 * s.dn[367][2]);
        let eq44_e800_d_n3: f64 = (eq44_e798 * s.dn[367][3]);
        let eq44_e800_d_n4: f64 = (eq44_e798 * s.dn[367][4]);
        let eq44_e800_d_n5: f64 = (eq44_e798 * s.dn[367][5]);
        let eq44_e800_d_n6: f64 = (eq44_e798 * s.dn[367][6]);
        let eq44_e800_d_n7: f64 = (eq44_e798 * s.dn[367][7]);
        let eq44_e800_d_n8: f64 = (eq44_e798 * s.dn[367][8]);
        let eq44_e800_d_n9: f64 = (eq44_e798 * s.dn[367][9]);
        let eq44_e800_d_n10: f64 = (eq44_e798 * s.dn[367][10]);
        let eq44_e800_d_n11: f64 = (eq44_e798 * s.dn[367][11]);
        let eq44_e800_d_n12: f64 = (eq44_e798 * s.dn[367][12]);
        let eq44_e800_d_n13: f64 = (eq44_e798 * s.dn[367][13]);
        let eq44_e800_d_n14: f64 = (eq44_e798 * s.dn[367][14]);
        let eq44_e800_d_n15: f64 = (eq44_e798 * s.dn[367][15]);
        let eq44_e800_d_n16: f64 = (eq44_e798 * s.dn[367][16]);
        let eq44_e800_d_n17: f64 = (eq44_e798 * s.dn[367][17]);
        let eq44_e800_d_n18: f64 = (eq44_e798 * s.dn[367][18]);
        let eq44_e800_d_n19: f64 = (eq44_e798 * s.dn[367][19]);
        let eq44_e800_d_n20: f64 = (eq44_e798 * s.dn[367][20]);
        let eq44_e800_d_n21: f64 = (eq44_e798 * s.dn[367][21]);
        let eq44_e800_d_n22: f64 = (eq44_e798 * s.dn[367][22]);
        let eq44_e800_d_b0: f64 = (eq44_e798 * s.db[367][0]);
        let eq44_e800_d_b1: f64 = (eq44_e798 * s.db[367][1]);
        let eq44_e800_d_b2: f64 = (eq44_e798 * s.db[367][2]);
        let eq44_e800_d_b3: f64 = (eq44_e798 * s.db[367][3]);
        let eq44_e800_d_b4: f64 = (eq44_e798 * s.db[367][4]);
        let eq44_e800_d_b5: f64 = (eq44_e798 * s.db[367][5]);
        let eq44_e800_d_b6: f64 = (eq44_e798 * s.db[367][6]);
        let eq44_e800_d_b7: f64 = (eq44_e798 * s.db[367][7]);
        let eq44_e800_d_b8: f64 = (eq44_e798 * s.db[367][8]);
        let eq44_e800_d_b9: f64 = (eq44_e798 * s.db[367][9]);
        let eq44_e800_d_b10: f64 = (eq44_e798 * s.db[367][10]);
        let eq44_e800_d_b11: f64 = (eq44_e798 * s.db[367][11]);
        let eq44_e800_d_b12: f64 = (eq44_e798 * s.db[367][12]);
        let eq44_e800_d_b13: f64 = (eq44_e798 * s.db[367][13]);
        let eq44_e800_d_b14: f64 = (eq44_e798 * s.db[367][14]);
        let eq44_e800_d_b15: f64 = (eq44_e798 * s.db[367][15]);
        let eq44_e800_d_b16: f64 = (eq44_e798 * s.db[367][16]);
        let eq44_e800_d_b17: f64 = (eq44_e798 * s.db[367][17]);
        let eq44_e800_d_b18: f64 = (eq44_e798 * s.db[367][18]);
        let eq44_e800_d_b19: f64 = (eq44_e798 * s.db[367][19]);
        let eq44_e800_d_b20: f64 = (eq44_e798 * s.db[367][20]);
        let eq44_e800_d_b21: f64 = (eq44_e798 * s.db[367][21]);
        let eq44_e800_d_b22: f64 = (eq44_e798 * s.db[367][22]);
        let eq44_e800_d_b23: f64 = (eq44_e798 * s.db[367][23]);
        let eq44_e800_d_b24: f64 = (eq44_e798 * s.db[367][24]);
        let eq44_e800_d_b25: f64 = (eq44_e798 * s.db[367][25]);
        let eq44_e800_d_b26: f64 = (eq44_e798 * s.db[367][26]);
        let eq44_e800_d_b27: f64 = (eq44_e798 * s.db[367][27]);
        let eq44_e800_d_b28: f64 = (eq44_e798 * s.db[367][28]);
        let eq44_e800_d_b29: f64 = (eq44_e798 * s.db[367][29]);
        let eq44_e800_d_b30: f64 = (eq44_e798 * s.db[367][30]);
        let eq44_e800_d_b31: f64 = (eq44_e798 * s.db[367][31]);
        let eq44_e800_d_b32: f64 = (eq44_e798 * s.db[367][32]);
        let eq44_e800_d_b33: f64 = (eq44_e798 * s.db[367][33]);
        let eq44_e800_d_b34: f64 = (eq44_e798 * s.db[367][34]);
        let eq44_e800_d_b35: f64 = (eq44_e798 * s.db[367][35]);
        let eq44_e800_d_b36: f64 = (eq44_e798 * s.db[367][36]);
        let eq44_e800_d_b37: f64 = (eq44_e798 * s.db[367][37]);
        let eq44_e800_d_b38: f64 = (eq44_e798 * s.db[367][38]);
        let eq44_e800_d_b39: f64 = (eq44_e798 * s.db[367][39]);
        let eq44_e800_d_b40: f64 = (eq44_e798 * s.db[367][40]);
        let eq44_e800_d_b41: f64 = (eq44_e798 * s.db[367][41]);
        let eq44_e800_d_b42: f64 = (eq44_e798 * s.db[367][42]);
        let eq44_e800_d_b43: f64 = (eq44_e798 * s.db[367][43]);
        let eq44_e800_d_b44: f64 = (eq44_e798 * s.db[367][44]);
        let eq44_e800_d_b45: f64 = (eq44_e798 * s.db[367][45]);
        let eq44_e800_d_b46: f64 = (eq44_e798 * s.db[367][46]);
        let eq44_e800_d_b47: f64 = (eq44_e798 * s.db[367][47]);
        let eq44_e800_d_b48: f64 = (eq44_e798 * s.db[367][48]);
        let eq44_e800_d_b49: f64 = (eq44_e798 * s.db[367][49]);
        let eq44_e800_d_b50: f64 = (eq44_e798 * s.db[367][50]);
        let eq44_e800_d_b51: f64 = (eq44_e798 * s.db[367][51]);
        let eq44_e800_d_b52: f64 = (eq44_e798 * s.db[367][52]);
        let eq44_e800_d_b53: f64 = (eq44_e798 * s.db[367][53]);
        let eq44_e800_d_b54: f64 = (eq44_e798 * s.db[367][54]);
        let eq44_e803: f64 = (p.p145 - (nv6 - 0.0));
        let eq44_e803_d_n6: f64 = (-1.0);
        let eq44_e804: f64 = (eq44_e800 * eq44_e803);
        let eq44_e804_d_n0: f64 = (eq44_e800_d_n0 * eq44_e803);
        let eq44_e804_d_n1: f64 = (eq44_e800_d_n1 * eq44_e803);
        let eq44_e804_d_n2: f64 = (eq44_e800_d_n2 * eq44_e803);
        let eq44_e804_d_n3: f64 = (eq44_e800_d_n3 * eq44_e803);
        let eq44_e804_d_n4: f64 = (eq44_e800_d_n4 * eq44_e803);
        let eq44_e804_d_n5: f64 = (eq44_e800_d_n5 * eq44_e803);
        let eq44_e804_d_n6: f64 = ((eq44_e800_d_n6 * eq44_e803) + (eq44_e800 * eq44_e803_d_n6));
        let eq44_e804_d_n7: f64 = (eq44_e800_d_n7 * eq44_e803);
        let eq44_e804_d_n8: f64 = (eq44_e800_d_n8 * eq44_e803);
        let eq44_e804_d_n9: f64 = (eq44_e800_d_n9 * eq44_e803);
        let eq44_e804_d_n10: f64 = (eq44_e800_d_n10 * eq44_e803);
        let eq44_e804_d_n11: f64 = (eq44_e800_d_n11 * eq44_e803);
        let eq44_e804_d_n12: f64 = (eq44_e800_d_n12 * eq44_e803);
        let eq44_e804_d_n13: f64 = (eq44_e800_d_n13 * eq44_e803);
        let eq44_e804_d_n14: f64 = (eq44_e800_d_n14 * eq44_e803);
        let eq44_e804_d_n15: f64 = (eq44_e800_d_n15 * eq44_e803);
        let eq44_e804_d_n16: f64 = (eq44_e800_d_n16 * eq44_e803);
        let eq44_e804_d_n17: f64 = (eq44_e800_d_n17 * eq44_e803);
        let eq44_e804_d_n18: f64 = (eq44_e800_d_n18 * eq44_e803);
        let eq44_e804_d_n19: f64 = (eq44_e800_d_n19 * eq44_e803);
        let eq44_e804_d_n20: f64 = (eq44_e800_d_n20 * eq44_e803);
        let eq44_e804_d_n21: f64 = (eq44_e800_d_n21 * eq44_e803);
        let eq44_e804_d_n22: f64 = (eq44_e800_d_n22 * eq44_e803);
        let eq44_e804_d_b0: f64 = (eq44_e800_d_b0 * eq44_e803);
        let eq44_e804_d_b1: f64 = (eq44_e800_d_b1 * eq44_e803);
        let eq44_e804_d_b2: f64 = (eq44_e800_d_b2 * eq44_e803);
        let eq44_e804_d_b3: f64 = (eq44_e800_d_b3 * eq44_e803);
        let eq44_e804_d_b4: f64 = (eq44_e800_d_b4 * eq44_e803);
        let eq44_e804_d_b5: f64 = (eq44_e800_d_b5 * eq44_e803);
        let eq44_e804_d_b6: f64 = (eq44_e800_d_b6 * eq44_e803);
        let eq44_e804_d_b7: f64 = (eq44_e800_d_b7 * eq44_e803);
        let eq44_e804_d_b8: f64 = (eq44_e800_d_b8 * eq44_e803);
        let eq44_e804_d_b9: f64 = (eq44_e800_d_b9 * eq44_e803);
        let eq44_e804_d_b10: f64 = (eq44_e800_d_b10 * eq44_e803);
        let eq44_e804_d_b11: f64 = (eq44_e800_d_b11 * eq44_e803);
        let eq44_e804_d_b12: f64 = (eq44_e800_d_b12 * eq44_e803);
        let eq44_e804_d_b13: f64 = (eq44_e800_d_b13 * eq44_e803);
        let eq44_e804_d_b14: f64 = (eq44_e800_d_b14 * eq44_e803);
        let eq44_e804_d_b15: f64 = (eq44_e800_d_b15 * eq44_e803);
        let eq44_e804_d_b16: f64 = (eq44_e800_d_b16 * eq44_e803);
        let eq44_e804_d_b17: f64 = (eq44_e800_d_b17 * eq44_e803);
        let eq44_e804_d_b18: f64 = (eq44_e800_d_b18 * eq44_e803);
        let eq44_e804_d_b19: f64 = (eq44_e800_d_b19 * eq44_e803);
        let eq44_e804_d_b20: f64 = (eq44_e800_d_b20 * eq44_e803);
        let eq44_e804_d_b21: f64 = (eq44_e800_d_b21 * eq44_e803);
        let eq44_e804_d_b22: f64 = (eq44_e800_d_b22 * eq44_e803);
        let eq44_e804_d_b23: f64 = (eq44_e800_d_b23 * eq44_e803);
        let eq44_e804_d_b24: f64 = (eq44_e800_d_b24 * eq44_e803);
        let eq44_e804_d_b25: f64 = (eq44_e800_d_b25 * eq44_e803);
        let eq44_e804_d_b26: f64 = (eq44_e800_d_b26 * eq44_e803);
        let eq44_e804_d_b27: f64 = (eq44_e800_d_b27 * eq44_e803);
        let eq44_e804_d_b28: f64 = (eq44_e800_d_b28 * eq44_e803);
        let eq44_e804_d_b29: f64 = (eq44_e800_d_b29 * eq44_e803);
        let eq44_e804_d_b30: f64 = (eq44_e800_d_b30 * eq44_e803);
        let eq44_e804_d_b31: f64 = (eq44_e800_d_b31 * eq44_e803);
        let eq44_e804_d_b32: f64 = (eq44_e800_d_b32 * eq44_e803);
        let eq44_e804_d_b33: f64 = (eq44_e800_d_b33 * eq44_e803);
        let eq44_e804_d_b34: f64 = (eq44_e800_d_b34 * eq44_e803);
        let eq44_e804_d_b35: f64 = (eq44_e800_d_b35 * eq44_e803);
        let eq44_e804_d_b36: f64 = (eq44_e800_d_b36 * eq44_e803);
        let eq44_e804_d_b37: f64 = (eq44_e800_d_b37 * eq44_e803);
        let eq44_e804_d_b38: f64 = (eq44_e800_d_b38 * eq44_e803);
        let eq44_e804_d_b39: f64 = (eq44_e800_d_b39 * eq44_e803);
        let eq44_e804_d_b40: f64 = (eq44_e800_d_b40 * eq44_e803);
        let eq44_e804_d_b41: f64 = (eq44_e800_d_b41 * eq44_e803);
        let eq44_e804_d_b42: f64 = (eq44_e800_d_b42 * eq44_e803);
        let eq44_e804_d_b43: f64 = (eq44_e800_d_b43 * eq44_e803);
        let eq44_e804_d_b44: f64 = (eq44_e800_d_b44 * eq44_e803);
        let eq44_e804_d_b45: f64 = (eq44_e800_d_b45 * eq44_e803);
        let eq44_e804_d_b46: f64 = (eq44_e800_d_b46 * eq44_e803);
        let eq44_e804_d_b47: f64 = (eq44_e800_d_b47 * eq44_e803);
        let eq44_e804_d_b48: f64 = (eq44_e800_d_b48 * eq44_e803);
        let eq44_e804_d_b49: f64 = (eq44_e800_d_b49 * eq44_e803);
        let eq44_e804_d_b50: f64 = (eq44_e800_d_b50 * eq44_e803);
        let eq44_e804_d_b51: f64 = (eq44_e800_d_b51 * eq44_e803);
        let eq44_e804_d_b52: f64 = (eq44_e800_d_b52 * eq44_e803);
        let eq44_e804_d_b53: f64 = (eq44_e800_d_b53 * eq44_e803);
        let eq44_e804_d_b54: f64 = (eq44_e800_d_b54 * eq44_e803);
        let eq44_e807: f64 = (2.0 * s.v[368]);
        let eq44_e807_d_n0: f64 = (2.0 * s.dn[368][0]);
        let eq44_e807_d_n1: f64 = (2.0 * s.dn[368][1]);
        let eq44_e807_d_n2: f64 = (2.0 * s.dn[368][2]);
        let eq44_e807_d_n3: f64 = (2.0 * s.dn[368][3]);
        let eq44_e807_d_n4: f64 = (2.0 * s.dn[368][4]);
        let eq44_e807_d_n5: f64 = (2.0 * s.dn[368][5]);
        let eq44_e807_d_n6: f64 = (2.0 * s.dn[368][6]);
        let eq44_e807_d_n7: f64 = (2.0 * s.dn[368][7]);
        let eq44_e807_d_n8: f64 = (2.0 * s.dn[368][8]);
        let eq44_e807_d_n9: f64 = (2.0 * s.dn[368][9]);
        let eq44_e807_d_n10: f64 = (2.0 * s.dn[368][10]);
        let eq44_e807_d_n11: f64 = (2.0 * s.dn[368][11]);
        let eq44_e807_d_n12: f64 = (2.0 * s.dn[368][12]);
        let eq44_e807_d_n13: f64 = (2.0 * s.dn[368][13]);
        let eq44_e807_d_n14: f64 = (2.0 * s.dn[368][14]);
        let eq44_e807_d_n15: f64 = (2.0 * s.dn[368][15]);
        let eq44_e807_d_n16: f64 = (2.0 * s.dn[368][16]);
        let eq44_e807_d_n17: f64 = (2.0 * s.dn[368][17]);
        let eq44_e807_d_n18: f64 = (2.0 * s.dn[368][18]);
        let eq44_e807_d_n19: f64 = (2.0 * s.dn[368][19]);
        let eq44_e807_d_n20: f64 = (2.0 * s.dn[368][20]);
        let eq44_e807_d_n21: f64 = (2.0 * s.dn[368][21]);
        let eq44_e807_d_n22: f64 = (2.0 * s.dn[368][22]);
        let eq44_e807_d_b0: f64 = (2.0 * s.db[368][0]);
        let eq44_e807_d_b1: f64 = (2.0 * s.db[368][1]);
        let eq44_e807_d_b2: f64 = (2.0 * s.db[368][2]);
        let eq44_e807_d_b3: f64 = (2.0 * s.db[368][3]);
        let eq44_e807_d_b4: f64 = (2.0 * s.db[368][4]);
        let eq44_e807_d_b5: f64 = (2.0 * s.db[368][5]);
        let eq44_e807_d_b6: f64 = (2.0 * s.db[368][6]);
        let eq44_e807_d_b7: f64 = (2.0 * s.db[368][7]);
        let eq44_e807_d_b8: f64 = (2.0 * s.db[368][8]);
        let eq44_e807_d_b9: f64 = (2.0 * s.db[368][9]);
        let eq44_e807_d_b10: f64 = (2.0 * s.db[368][10]);
        let eq44_e807_d_b11: f64 = (2.0 * s.db[368][11]);
        let eq44_e807_d_b12: f64 = (2.0 * s.db[368][12]);
        let eq44_e807_d_b13: f64 = (2.0 * s.db[368][13]);
        let eq44_e807_d_b14: f64 = (2.0 * s.db[368][14]);
        let eq44_e807_d_b15: f64 = (2.0 * s.db[368][15]);
        let eq44_e807_d_b16: f64 = (2.0 * s.db[368][16]);
        let eq44_e807_d_b17: f64 = (2.0 * s.db[368][17]);
        let eq44_e807_d_b18: f64 = (2.0 * s.db[368][18]);
        let eq44_e807_d_b19: f64 = (2.0 * s.db[368][19]);
        let eq44_e807_d_b20: f64 = (2.0 * s.db[368][20]);
        let eq44_e807_d_b21: f64 = (2.0 * s.db[368][21]);
        let eq44_e807_d_b22: f64 = (2.0 * s.db[368][22]);
        let eq44_e807_d_b23: f64 = (2.0 * s.db[368][23]);
        let eq44_e807_d_b24: f64 = (2.0 * s.db[368][24]);
        let eq44_e807_d_b25: f64 = (2.0 * s.db[368][25]);
        let eq44_e807_d_b26: f64 = (2.0 * s.db[368][26]);
        let eq44_e807_d_b27: f64 = (2.0 * s.db[368][27]);
        let eq44_e807_d_b28: f64 = (2.0 * s.db[368][28]);
        let eq44_e807_d_b29: f64 = (2.0 * s.db[368][29]);
        let eq44_e807_d_b30: f64 = (2.0 * s.db[368][30]);
        let eq44_e807_d_b31: f64 = (2.0 * s.db[368][31]);
        let eq44_e807_d_b32: f64 = (2.0 * s.db[368][32]);
        let eq44_e807_d_b33: f64 = (2.0 * s.db[368][33]);
        let eq44_e807_d_b34: f64 = (2.0 * s.db[368][34]);
        let eq44_e807_d_b35: f64 = (2.0 * s.db[368][35]);
        let eq44_e807_d_b36: f64 = (2.0 * s.db[368][36]);
        let eq44_e807_d_b37: f64 = (2.0 * s.db[368][37]);
        let eq44_e807_d_b38: f64 = (2.0 * s.db[368][38]);
        let eq44_e807_d_b39: f64 = (2.0 * s.db[368][39]);
        let eq44_e807_d_b40: f64 = (2.0 * s.db[368][40]);
        let eq44_e807_d_b41: f64 = (2.0 * s.db[368][41]);
        let eq44_e807_d_b42: f64 = (2.0 * s.db[368][42]);
        let eq44_e807_d_b43: f64 = (2.0 * s.db[368][43]);
        let eq44_e807_d_b44: f64 = (2.0 * s.db[368][44]);
        let eq44_e807_d_b45: f64 = (2.0 * s.db[368][45]);
        let eq44_e807_d_b46: f64 = (2.0 * s.db[368][46]);
        let eq44_e807_d_b47: f64 = (2.0 * s.db[368][47]);
        let eq44_e807_d_b48: f64 = (2.0 * s.db[368][48]);
        let eq44_e807_d_b49: f64 = (2.0 * s.db[368][49]);
        let eq44_e807_d_b50: f64 = (2.0 * s.db[368][50]);
        let eq44_e807_d_b51: f64 = (2.0 * s.db[368][51]);
        let eq44_e807_d_b52: f64 = (2.0 * s.db[368][52]);
        let eq44_e807_d_b53: f64 = (2.0 * s.db[368][53]);
        let eq44_e807_d_b54: f64 = (2.0 * s.db[368][54]);
        let eq44_e808: f64 = (eq44_e807).exp();
        let eq44_e808_d_n0: f64 = (eq44_e808 * eq44_e807_d_n0);
        let eq44_e808_d_n1: f64 = (eq44_e808 * eq44_e807_d_n1);
        let eq44_e808_d_n2: f64 = (eq44_e808 * eq44_e807_d_n2);
        let eq44_e808_d_n3: f64 = (eq44_e808 * eq44_e807_d_n3);
        let eq44_e808_d_n4: f64 = (eq44_e808 * eq44_e807_d_n4);
        let eq44_e808_d_n5: f64 = (eq44_e808 * eq44_e807_d_n5);
        let eq44_e808_d_n6: f64 = (eq44_e808 * eq44_e807_d_n6);
        let eq44_e808_d_n7: f64 = (eq44_e808 * eq44_e807_d_n7);
        let eq44_e808_d_n8: f64 = (eq44_e808 * eq44_e807_d_n8);
        let eq44_e808_d_n9: f64 = (eq44_e808 * eq44_e807_d_n9);
        let eq44_e808_d_n10: f64 = (eq44_e808 * eq44_e807_d_n10);
        let eq44_e808_d_n11: f64 = (eq44_e808 * eq44_e807_d_n11);
        let eq44_e808_d_n12: f64 = (eq44_e808 * eq44_e807_d_n12);
        let eq44_e808_d_n13: f64 = (eq44_e808 * eq44_e807_d_n13);
        let eq44_e808_d_n14: f64 = (eq44_e808 * eq44_e807_d_n14);
        let eq44_e808_d_n15: f64 = (eq44_e808 * eq44_e807_d_n15);
        let eq44_e808_d_n16: f64 = (eq44_e808 * eq44_e807_d_n16);
        let eq44_e808_d_n17: f64 = (eq44_e808 * eq44_e807_d_n17);
        let eq44_e808_d_n18: f64 = (eq44_e808 * eq44_e807_d_n18);
        let eq44_e808_d_n19: f64 = (eq44_e808 * eq44_e807_d_n19);
        let eq44_e808_d_n20: f64 = (eq44_e808 * eq44_e807_d_n20);
        let eq44_e808_d_n21: f64 = (eq44_e808 * eq44_e807_d_n21);
        let eq44_e808_d_n22: f64 = (eq44_e808 * eq44_e807_d_n22);
        let eq44_e808_d_b0: f64 = (eq44_e808 * eq44_e807_d_b0);
        let eq44_e808_d_b1: f64 = (eq44_e808 * eq44_e807_d_b1);
        let eq44_e808_d_b2: f64 = (eq44_e808 * eq44_e807_d_b2);
        let eq44_e808_d_b3: f64 = (eq44_e808 * eq44_e807_d_b3);
        let eq44_e808_d_b4: f64 = (eq44_e808 * eq44_e807_d_b4);
        let eq44_e808_d_b5: f64 = (eq44_e808 * eq44_e807_d_b5);
        let eq44_e808_d_b6: f64 = (eq44_e808 * eq44_e807_d_b6);
        let eq44_e808_d_b7: f64 = (eq44_e808 * eq44_e807_d_b7);
        let eq44_e808_d_b8: f64 = (eq44_e808 * eq44_e807_d_b8);
        let eq44_e808_d_b9: f64 = (eq44_e808 * eq44_e807_d_b9);
        let eq44_e808_d_b10: f64 = (eq44_e808 * eq44_e807_d_b10);
        let eq44_e808_d_b11: f64 = (eq44_e808 * eq44_e807_d_b11);
        let eq44_e808_d_b12: f64 = (eq44_e808 * eq44_e807_d_b12);
        let eq44_e808_d_b13: f64 = (eq44_e808 * eq44_e807_d_b13);
        let eq44_e808_d_b14: f64 = (eq44_e808 * eq44_e807_d_b14);
        let eq44_e808_d_b15: f64 = (eq44_e808 * eq44_e807_d_b15);
        let eq44_e808_d_b16: f64 = (eq44_e808 * eq44_e807_d_b16);
        let eq44_e808_d_b17: f64 = (eq44_e808 * eq44_e807_d_b17);
        let eq44_e808_d_b18: f64 = (eq44_e808 * eq44_e807_d_b18);
        let eq44_e808_d_b19: f64 = (eq44_e808 * eq44_e807_d_b19);
        let eq44_e808_d_b20: f64 = (eq44_e808 * eq44_e807_d_b20);
        let eq44_e808_d_b21: f64 = (eq44_e808 * eq44_e807_d_b21);
        let eq44_e808_d_b22: f64 = (eq44_e808 * eq44_e807_d_b22);
        let eq44_e808_d_b23: f64 = (eq44_e808 * eq44_e807_d_b23);
        let eq44_e808_d_b24: f64 = (eq44_e808 * eq44_e807_d_b24);
        let eq44_e808_d_b25: f64 = (eq44_e808 * eq44_e807_d_b25);
        let eq44_e808_d_b26: f64 = (eq44_e808 * eq44_e807_d_b26);
        let eq44_e808_d_b27: f64 = (eq44_e808 * eq44_e807_d_b27);
        let eq44_e808_d_b28: f64 = (eq44_e808 * eq44_e807_d_b28);
        let eq44_e808_d_b29: f64 = (eq44_e808 * eq44_e807_d_b29);
        let eq44_e808_d_b30: f64 = (eq44_e808 * eq44_e807_d_b30);
        let eq44_e808_d_b31: f64 = (eq44_e808 * eq44_e807_d_b31);
        let eq44_e808_d_b32: f64 = (eq44_e808 * eq44_e807_d_b32);
        let eq44_e808_d_b33: f64 = (eq44_e808 * eq44_e807_d_b33);
        let eq44_e808_d_b34: f64 = (eq44_e808 * eq44_e807_d_b34);
        let eq44_e808_d_b35: f64 = (eq44_e808 * eq44_e807_d_b35);
        let eq44_e808_d_b36: f64 = (eq44_e808 * eq44_e807_d_b36);
        let eq44_e808_d_b37: f64 = (eq44_e808 * eq44_e807_d_b37);
        let eq44_e808_d_b38: f64 = (eq44_e808 * eq44_e807_d_b38);
        let eq44_e808_d_b39: f64 = (eq44_e808 * eq44_e807_d_b39);
        let eq44_e808_d_b40: f64 = (eq44_e808 * eq44_e807_d_b40);
        let eq44_e808_d_b41: f64 = (eq44_e808 * eq44_e807_d_b41);
        let eq44_e808_d_b42: f64 = (eq44_e808 * eq44_e807_d_b42);
        let eq44_e808_d_b43: f64 = (eq44_e808 * eq44_e807_d_b43);
        let eq44_e808_d_b44: f64 = (eq44_e808 * eq44_e807_d_b44);
        let eq44_e808_d_b45: f64 = (eq44_e808 * eq44_e807_d_b45);
        let eq44_e808_d_b46: f64 = (eq44_e808 * eq44_e807_d_b46);
        let eq44_e808_d_b47: f64 = (eq44_e808 * eq44_e807_d_b47);
        let eq44_e808_d_b48: f64 = (eq44_e808 * eq44_e807_d_b48);
        let eq44_e808_d_b49: f64 = (eq44_e808 * eq44_e807_d_b49);
        let eq44_e808_d_b50: f64 = (eq44_e808 * eq44_e807_d_b50);
        let eq44_e808_d_b51: f64 = (eq44_e808 * eq44_e807_d_b51);
        let eq44_e808_d_b52: f64 = (eq44_e808 * eq44_e807_d_b52);
        let eq44_e808_d_b53: f64 = (eq44_e808 * eq44_e807_d_b53);
        let eq44_e808_d_b54: f64 = (eq44_e808 * eq44_e807_d_b54);
        let eq44_e810: f64 = (eq44_e808 - 1.0);
        let eq44_e811: f64 = (eq44_e804 * eq44_e810);
        let eq44_e811_d_n0: f64 = ((eq44_e804_d_n0 * eq44_e810) + (eq44_e804 * eq44_e808_d_n0));
        let eq44_e811_d_n1: f64 = ((eq44_e804_d_n1 * eq44_e810) + (eq44_e804 * eq44_e808_d_n1));
        let eq44_e811_d_n2: f64 = ((eq44_e804_d_n2 * eq44_e810) + (eq44_e804 * eq44_e808_d_n2));
        let eq44_e811_d_n3: f64 = ((eq44_e804_d_n3 * eq44_e810) + (eq44_e804 * eq44_e808_d_n3));
        let eq44_e811_d_n4: f64 = ((eq44_e804_d_n4 * eq44_e810) + (eq44_e804 * eq44_e808_d_n4));
        let eq44_e811_d_n5: f64 = ((eq44_e804_d_n5 * eq44_e810) + (eq44_e804 * eq44_e808_d_n5));
        let eq44_e811_d_n6: f64 = ((eq44_e804_d_n6 * eq44_e810) + (eq44_e804 * eq44_e808_d_n6));
        let eq44_e811_d_n7: f64 = ((eq44_e804_d_n7 * eq44_e810) + (eq44_e804 * eq44_e808_d_n7));
        let eq44_e811_d_n8: f64 = ((eq44_e804_d_n8 * eq44_e810) + (eq44_e804 * eq44_e808_d_n8));
        let eq44_e811_d_n9: f64 = ((eq44_e804_d_n9 * eq44_e810) + (eq44_e804 * eq44_e808_d_n9));
        let eq44_e811_d_n10: f64 = ((eq44_e804_d_n10 * eq44_e810) + (eq44_e804 * eq44_e808_d_n10));
        let eq44_e811_d_n11: f64 = ((eq44_e804_d_n11 * eq44_e810) + (eq44_e804 * eq44_e808_d_n11));
        let eq44_e811_d_n12: f64 = ((eq44_e804_d_n12 * eq44_e810) + (eq44_e804 * eq44_e808_d_n12));
        let eq44_e811_d_n13: f64 = ((eq44_e804_d_n13 * eq44_e810) + (eq44_e804 * eq44_e808_d_n13));
        let eq44_e811_d_n14: f64 = ((eq44_e804_d_n14 * eq44_e810) + (eq44_e804 * eq44_e808_d_n14));
        let eq44_e811_d_n15: f64 = ((eq44_e804_d_n15 * eq44_e810) + (eq44_e804 * eq44_e808_d_n15));
        let eq44_e811_d_n16: f64 = ((eq44_e804_d_n16 * eq44_e810) + (eq44_e804 * eq44_e808_d_n16));
        let eq44_e811_d_n17: f64 = ((eq44_e804_d_n17 * eq44_e810) + (eq44_e804 * eq44_e808_d_n17));
        let eq44_e811_d_n18: f64 = ((eq44_e804_d_n18 * eq44_e810) + (eq44_e804 * eq44_e808_d_n18));
        let eq44_e811_d_n19: f64 = ((eq44_e804_d_n19 * eq44_e810) + (eq44_e804 * eq44_e808_d_n19));
        let eq44_e811_d_n20: f64 = ((eq44_e804_d_n20 * eq44_e810) + (eq44_e804 * eq44_e808_d_n20));
        let eq44_e811_d_n21: f64 = ((eq44_e804_d_n21 * eq44_e810) + (eq44_e804 * eq44_e808_d_n21));
        let eq44_e811_d_n22: f64 = ((eq44_e804_d_n22 * eq44_e810) + (eq44_e804 * eq44_e808_d_n22));
        let eq44_e811_d_b0: f64 = ((eq44_e804_d_b0 * eq44_e810) + (eq44_e804 * eq44_e808_d_b0));
        let eq44_e811_d_b1: f64 = ((eq44_e804_d_b1 * eq44_e810) + (eq44_e804 * eq44_e808_d_b1));
        let eq44_e811_d_b2: f64 = ((eq44_e804_d_b2 * eq44_e810) + (eq44_e804 * eq44_e808_d_b2));
        let eq44_e811_d_b3: f64 = ((eq44_e804_d_b3 * eq44_e810) + (eq44_e804 * eq44_e808_d_b3));
        let eq44_e811_d_b4: f64 = ((eq44_e804_d_b4 * eq44_e810) + (eq44_e804 * eq44_e808_d_b4));
        let eq44_e811_d_b5: f64 = ((eq44_e804_d_b5 * eq44_e810) + (eq44_e804 * eq44_e808_d_b5));
        let eq44_e811_d_b6: f64 = ((eq44_e804_d_b6 * eq44_e810) + (eq44_e804 * eq44_e808_d_b6));
        let eq44_e811_d_b7: f64 = ((eq44_e804_d_b7 * eq44_e810) + (eq44_e804 * eq44_e808_d_b7));
        let eq44_e811_d_b8: f64 = ((eq44_e804_d_b8 * eq44_e810) + (eq44_e804 * eq44_e808_d_b8));
        let eq44_e811_d_b9: f64 = ((eq44_e804_d_b9 * eq44_e810) + (eq44_e804 * eq44_e808_d_b9));
        let eq44_e811_d_b10: f64 = ((eq44_e804_d_b10 * eq44_e810) + (eq44_e804 * eq44_e808_d_b10));
        let eq44_e811_d_b11: f64 = ((eq44_e804_d_b11 * eq44_e810) + (eq44_e804 * eq44_e808_d_b11));
        let eq44_e811_d_b12: f64 = ((eq44_e804_d_b12 * eq44_e810) + (eq44_e804 * eq44_e808_d_b12));
        let eq44_e811_d_b13: f64 = ((eq44_e804_d_b13 * eq44_e810) + (eq44_e804 * eq44_e808_d_b13));
        let eq44_e811_d_b14: f64 = ((eq44_e804_d_b14 * eq44_e810) + (eq44_e804 * eq44_e808_d_b14));
        let eq44_e811_d_b15: f64 = ((eq44_e804_d_b15 * eq44_e810) + (eq44_e804 * eq44_e808_d_b15));
        let eq44_e811_d_b16: f64 = ((eq44_e804_d_b16 * eq44_e810) + (eq44_e804 * eq44_e808_d_b16));
        let eq44_e811_d_b17: f64 = ((eq44_e804_d_b17 * eq44_e810) + (eq44_e804 * eq44_e808_d_b17));
        let eq44_e811_d_b18: f64 = ((eq44_e804_d_b18 * eq44_e810) + (eq44_e804 * eq44_e808_d_b18));
        let eq44_e811_d_b19: f64 = ((eq44_e804_d_b19 * eq44_e810) + (eq44_e804 * eq44_e808_d_b19));
        let eq44_e811_d_b20: f64 = ((eq44_e804_d_b20 * eq44_e810) + (eq44_e804 * eq44_e808_d_b20));
        let eq44_e811_d_b21: f64 = ((eq44_e804_d_b21 * eq44_e810) + (eq44_e804 * eq44_e808_d_b21));
        let eq44_e811_d_b22: f64 = ((eq44_e804_d_b22 * eq44_e810) + (eq44_e804 * eq44_e808_d_b22));
        let eq44_e811_d_b23: f64 = ((eq44_e804_d_b23 * eq44_e810) + (eq44_e804 * eq44_e808_d_b23));
        let eq44_e811_d_b24: f64 = ((eq44_e804_d_b24 * eq44_e810) + (eq44_e804 * eq44_e808_d_b24));
        let eq44_e811_d_b25: f64 = ((eq44_e804_d_b25 * eq44_e810) + (eq44_e804 * eq44_e808_d_b25));
        let eq44_e811_d_b26: f64 = ((eq44_e804_d_b26 * eq44_e810) + (eq44_e804 * eq44_e808_d_b26));
        let eq44_e811_d_b27: f64 = ((eq44_e804_d_b27 * eq44_e810) + (eq44_e804 * eq44_e808_d_b27));
        let eq44_e811_d_b28: f64 = ((eq44_e804_d_b28 * eq44_e810) + (eq44_e804 * eq44_e808_d_b28));
        let eq44_e811_d_b29: f64 = ((eq44_e804_d_b29 * eq44_e810) + (eq44_e804 * eq44_e808_d_b29));
        let eq44_e811_d_b30: f64 = ((eq44_e804_d_b30 * eq44_e810) + (eq44_e804 * eq44_e808_d_b30));
        let eq44_e811_d_b31: f64 = ((eq44_e804_d_b31 * eq44_e810) + (eq44_e804 * eq44_e808_d_b31));
        let eq44_e811_d_b32: f64 = ((eq44_e804_d_b32 * eq44_e810) + (eq44_e804 * eq44_e808_d_b32));
        let eq44_e811_d_b33: f64 = ((eq44_e804_d_b33 * eq44_e810) + (eq44_e804 * eq44_e808_d_b33));
        let eq44_e811_d_b34: f64 = ((eq44_e804_d_b34 * eq44_e810) + (eq44_e804 * eq44_e808_d_b34));
        let eq44_e811_d_b35: f64 = ((eq44_e804_d_b35 * eq44_e810) + (eq44_e804 * eq44_e808_d_b35));
        let eq44_e811_d_b36: f64 = ((eq44_e804_d_b36 * eq44_e810) + (eq44_e804 * eq44_e808_d_b36));
        let eq44_e811_d_b37: f64 = ((eq44_e804_d_b37 * eq44_e810) + (eq44_e804 * eq44_e808_d_b37));
        let eq44_e811_d_b38: f64 = ((eq44_e804_d_b38 * eq44_e810) + (eq44_e804 * eq44_e808_d_b38));
        let eq44_e811_d_b39: f64 = ((eq44_e804_d_b39 * eq44_e810) + (eq44_e804 * eq44_e808_d_b39));
        let eq44_e811_d_b40: f64 = ((eq44_e804_d_b40 * eq44_e810) + (eq44_e804 * eq44_e808_d_b40));
        let eq44_e811_d_b41: f64 = ((eq44_e804_d_b41 * eq44_e810) + (eq44_e804 * eq44_e808_d_b41));
        let eq44_e811_d_b42: f64 = ((eq44_e804_d_b42 * eq44_e810) + (eq44_e804 * eq44_e808_d_b42));
        let eq44_e811_d_b43: f64 = ((eq44_e804_d_b43 * eq44_e810) + (eq44_e804 * eq44_e808_d_b43));
        let eq44_e811_d_b44: f64 = ((eq44_e804_d_b44 * eq44_e810) + (eq44_e804 * eq44_e808_d_b44));
        let eq44_e811_d_b45: f64 = ((eq44_e804_d_b45 * eq44_e810) + (eq44_e804 * eq44_e808_d_b45));
        let eq44_e811_d_b46: f64 = ((eq44_e804_d_b46 * eq44_e810) + (eq44_e804 * eq44_e808_d_b46));
        let eq44_e811_d_b47: f64 = ((eq44_e804_d_b47 * eq44_e810) + (eq44_e804 * eq44_e808_d_b47));
        let eq44_e811_d_b48: f64 = ((eq44_e804_d_b48 * eq44_e810) + (eq44_e804 * eq44_e808_d_b48));
        let eq44_e811_d_b49: f64 = ((eq44_e804_d_b49 * eq44_e810) + (eq44_e804 * eq44_e808_d_b49));
        let eq44_e811_d_b50: f64 = ((eq44_e804_d_b50 * eq44_e810) + (eq44_e804 * eq44_e808_d_b50));
        let eq44_e811_d_b51: f64 = ((eq44_e804_d_b51 * eq44_e810) + (eq44_e804 * eq44_e808_d_b51));
        let eq44_e811_d_b52: f64 = ((eq44_e804_d_b52 * eq44_e810) + (eq44_e804 * eq44_e808_d_b52));
        let eq44_e811_d_b53: f64 = ((eq44_e804_d_b53 * eq44_e810) + (eq44_e804 * eq44_e808_d_b53));
        let eq44_e811_d_b54: f64 = ((eq44_e804_d_b54 * eq44_e810) + (eq44_e804 * eq44_e808_d_b54));
        let eq44_e813: f64 = (eq44_e811 * 0.5);
        let eq44_e813_d_n0: f64 = (eq44_e811_d_n0 * 0.5);
        let eq44_e813_d_n1: f64 = (eq44_e811_d_n1 * 0.5);
        let eq44_e813_d_n2: f64 = (eq44_e811_d_n2 * 0.5);
        let eq44_e813_d_n3: f64 = (eq44_e811_d_n3 * 0.5);
        let eq44_e813_d_n4: f64 = (eq44_e811_d_n4 * 0.5);
        let eq44_e813_d_n5: f64 = (eq44_e811_d_n5 * 0.5);
        let eq44_e813_d_n6: f64 = (eq44_e811_d_n6 * 0.5);
        let eq44_e813_d_n7: f64 = (eq44_e811_d_n7 * 0.5);
        let eq44_e813_d_n8: f64 = (eq44_e811_d_n8 * 0.5);
        let eq44_e813_d_n9: f64 = (eq44_e811_d_n9 * 0.5);
        let eq44_e813_d_n10: f64 = (eq44_e811_d_n10 * 0.5);
        let eq44_e813_d_n11: f64 = (eq44_e811_d_n11 * 0.5);
        let eq44_e813_d_n12: f64 = (eq44_e811_d_n12 * 0.5);
        let eq44_e813_d_n13: f64 = (eq44_e811_d_n13 * 0.5);
        let eq44_e813_d_n14: f64 = (eq44_e811_d_n14 * 0.5);
        let eq44_e813_d_n15: f64 = (eq44_e811_d_n15 * 0.5);
        let eq44_e813_d_n16: f64 = (eq44_e811_d_n16 * 0.5);
        let eq44_e813_d_n17: f64 = (eq44_e811_d_n17 * 0.5);
        let eq44_e813_d_n18: f64 = (eq44_e811_d_n18 * 0.5);
        let eq44_e813_d_n19: f64 = (eq44_e811_d_n19 * 0.5);
        let eq44_e813_d_n20: f64 = (eq44_e811_d_n20 * 0.5);
        let eq44_e813_d_n21: f64 = (eq44_e811_d_n21 * 0.5);
        let eq44_e813_d_n22: f64 = (eq44_e811_d_n22 * 0.5);
        let eq44_e813_d_b0: f64 = (eq44_e811_d_b0 * 0.5);
        let eq44_e813_d_b1: f64 = (eq44_e811_d_b1 * 0.5);
        let eq44_e813_d_b2: f64 = (eq44_e811_d_b2 * 0.5);
        let eq44_e813_d_b3: f64 = (eq44_e811_d_b3 * 0.5);
        let eq44_e813_d_b4: f64 = (eq44_e811_d_b4 * 0.5);
        let eq44_e813_d_b5: f64 = (eq44_e811_d_b5 * 0.5);
        let eq44_e813_d_b6: f64 = (eq44_e811_d_b6 * 0.5);
        let eq44_e813_d_b7: f64 = (eq44_e811_d_b7 * 0.5);
        let eq44_e813_d_b8: f64 = (eq44_e811_d_b8 * 0.5);
        let eq44_e813_d_b9: f64 = (eq44_e811_d_b9 * 0.5);
        let eq44_e813_d_b10: f64 = (eq44_e811_d_b10 * 0.5);
        let eq44_e813_d_b11: f64 = (eq44_e811_d_b11 * 0.5);
        let eq44_e813_d_b12: f64 = (eq44_e811_d_b12 * 0.5);
        let eq44_e813_d_b13: f64 = (eq44_e811_d_b13 * 0.5);
        let eq44_e813_d_b14: f64 = (eq44_e811_d_b14 * 0.5);
        let eq44_e813_d_b15: f64 = (eq44_e811_d_b15 * 0.5);
        let eq44_e813_d_b16: f64 = (eq44_e811_d_b16 * 0.5);
        let eq44_e813_d_b17: f64 = (eq44_e811_d_b17 * 0.5);
        let eq44_e813_d_b18: f64 = (eq44_e811_d_b18 * 0.5);
        let eq44_e813_d_b19: f64 = (eq44_e811_d_b19 * 0.5);
        let eq44_e813_d_b20: f64 = (eq44_e811_d_b20 * 0.5);
        let eq44_e813_d_b21: f64 = (eq44_e811_d_b21 * 0.5);
        let eq44_e813_d_b22: f64 = (eq44_e811_d_b22 * 0.5);
        let eq44_e813_d_b23: f64 = (eq44_e811_d_b23 * 0.5);
        let eq44_e813_d_b24: f64 = (eq44_e811_d_b24 * 0.5);
        let eq44_e813_d_b25: f64 = (eq44_e811_d_b25 * 0.5);
        let eq44_e813_d_b26: f64 = (eq44_e811_d_b26 * 0.5);
        let eq44_e813_d_b27: f64 = (eq44_e811_d_b27 * 0.5);
        let eq44_e813_d_b28: f64 = (eq44_e811_d_b28 * 0.5);
        let eq44_e813_d_b29: f64 = (eq44_e811_d_b29 * 0.5);
        let eq44_e813_d_b30: f64 = (eq44_e811_d_b30 * 0.5);
        let eq44_e813_d_b31: f64 = (eq44_e811_d_b31 * 0.5);
        let eq44_e813_d_b32: f64 = (eq44_e811_d_b32 * 0.5);
        let eq44_e813_d_b33: f64 = (eq44_e811_d_b33 * 0.5);
        let eq44_e813_d_b34: f64 = (eq44_e811_d_b34 * 0.5);
        let eq44_e813_d_b35: f64 = (eq44_e811_d_b35 * 0.5);
        let eq44_e813_d_b36: f64 = (eq44_e811_d_b36 * 0.5);
        let eq44_e813_d_b37: f64 = (eq44_e811_d_b37 * 0.5);
        let eq44_e813_d_b38: f64 = (eq44_e811_d_b38 * 0.5);
        let eq44_e813_d_b39: f64 = (eq44_e811_d_b39 * 0.5);
        let eq44_e813_d_b40: f64 = (eq44_e811_d_b40 * 0.5);
        let eq44_e813_d_b41: f64 = (eq44_e811_d_b41 * 0.5);
        let eq44_e813_d_b42: f64 = (eq44_e811_d_b42 * 0.5);
        let eq44_e813_d_b43: f64 = (eq44_e811_d_b43 * 0.5);
        let eq44_e813_d_b44: f64 = (eq44_e811_d_b44 * 0.5);
        let eq44_e813_d_b45: f64 = (eq44_e811_d_b45 * 0.5);
        let eq44_e813_d_b46: f64 = (eq44_e811_d_b46 * 0.5);
        let eq44_e813_d_b47: f64 = (eq44_e811_d_b47 * 0.5);
        let eq44_e813_d_b48: f64 = (eq44_e811_d_b48 * 0.5);
        let eq44_e813_d_b49: f64 = (eq44_e811_d_b49 * 0.5);
        let eq44_e813_d_b50: f64 = (eq44_e811_d_b50 * 0.5);
        let eq44_e813_d_b51: f64 = (eq44_e811_d_b51 * 0.5);
        let eq44_e813_d_b52: f64 = (eq44_e811_d_b52 * 0.5);
        let eq44_e813_d_b53: f64 = (eq44_e811_d_b53 * 0.5);
        let eq44_e813_d_b54: f64 = (eq44_e811_d_b54 * 0.5);
        (eq44_e813, eq44_e813_d_n0, eq44_e813_d_n1, eq44_e813_d_n2, eq44_e813_d_n3, eq44_e813_d_n4, eq44_e813_d_n5, eq44_e813_d_n6, eq44_e813_d_n7, eq44_e813_d_n8, eq44_e813_d_n9, eq44_e813_d_n10, eq44_e813_d_n11, eq44_e813_d_n12, eq44_e813_d_n13, eq44_e813_d_n14, eq44_e813_d_n15, eq44_e813_d_n16, eq44_e813_d_n17, eq44_e813_d_n18, eq44_e813_d_n19, eq44_e813_d_n20, eq44_e813_d_n21, eq44_e813_d_n22, eq44_e813_d_b0, eq44_e813_d_b1, eq44_e813_d_b2, eq44_e813_d_b3, eq44_e813_d_b4, eq44_e813_d_b5, eq44_e813_d_b6, eq44_e813_d_b7, eq44_e813_d_b8, eq44_e813_d_b9, eq44_e813_d_b10, eq44_e813_d_b11, eq44_e813_d_b12, eq44_e813_d_b13, eq44_e813_d_b14, eq44_e813_d_b15, eq44_e813_d_b16, eq44_e813_d_b17, eq44_e813_d_b18, eq44_e813_d_b19, eq44_e813_d_b20, eq44_e813_d_b21, eq44_e813_d_b22, eq44_e813_d_b23, eq44_e813_d_b24, eq44_e813_d_b25, eq44_e813_d_b26, eq44_e813_d_b27, eq44_e813_d_b28, eq44_e813_d_b29, eq44_e813_d_b30, eq44_e813_d_b31, eq44_e813_d_b32, eq44_e813_d_b33, eq44_e813_d_b34, eq44_e813_d_b35, eq44_e813_d_b36, eq44_e813_d_b37, eq44_e813_d_b38, eq44_e813_d_b39, eq44_e813_d_b40, eq44_e813_d_b41, eq44_e813_d_b42, eq44_e813_d_b43, eq44_e813_d_b44, eq44_e813_d_b45, eq44_e813_d_b46, eq44_e813_d_b47, eq44_e813_d_b48, eq44_e813_d_b49, eq44_e813_d_b50, eq44_e813_d_b51, eq44_e813_d_b52, eq44_e813_d_b53, eq44_e813_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e815;
        let eq44_node_derivatives: [f64; 23] = [eq44_e815_d_n0, eq44_e815_d_n1, eq44_e815_d_n2, eq44_e815_d_n3, eq44_e815_d_n4, eq44_e815_d_n5, eq44_e815_d_n6, eq44_e815_d_n7, eq44_e815_d_n8, eq44_e815_d_n9, eq44_e815_d_n10, eq44_e815_d_n11, eq44_e815_d_n12, eq44_e815_d_n13, eq44_e815_d_n14, eq44_e815_d_n15, eq44_e815_d_n16, eq44_e815_d_n17, eq44_e815_d_n18, eq44_e815_d_n19, eq44_e815_d_n20, eq44_e815_d_n21, eq44_e815_d_n22];
        let eq44_branch_derivatives: [f64; 55] = [eq44_e815_d_b0, eq44_e815_d_b1, eq44_e815_d_b2, eq44_e815_d_b3, eq44_e815_d_b4, eq44_e815_d_b5, eq44_e815_d_b6, eq44_e815_d_b7, eq44_e815_d_b8, eq44_e815_d_b9, eq44_e815_d_b10, eq44_e815_d_b11, eq44_e815_d_b12, eq44_e815_d_b13, eq44_e815_d_b14, eq44_e815_d_b15, eq44_e815_d_b16, eq44_e815_d_b17, eq44_e815_d_b18, eq44_e815_d_b19, eq44_e815_d_b20, eq44_e815_d_b21, eq44_e815_d_b22, eq44_e815_d_b23, eq44_e815_d_b24, eq44_e815_d_b25, eq44_e815_d_b26, eq44_e815_d_b27, eq44_e815_d_b28, eq44_e815_d_b29, eq44_e815_d_b30, eq44_e815_d_b31, eq44_e815_d_b32, eq44_e815_d_b33, eq44_e815_d_b34, eq44_e815_d_b35, eq44_e815_d_b36, eq44_e815_d_b37, eq44_e815_d_b38, eq44_e815_d_b39, eq44_e815_d_b40, eq44_e815_d_b41, eq44_e815_d_b42, eq44_e815_d_b43, eq44_e815_d_b44, eq44_e815_d_b45, eq44_e815_d_b46, eq44_e815_d_b47, eq44_e815_d_b48, eq44_e815_d_b49, eq44_e815_d_b50, eq44_e815_d_b51, eq44_e815_d_b52, eq44_e815_d_b53, eq44_e815_d_b54];
        stamper.stamp_current_dense_local(
            Some(6),
            None,
            multiplicity * (eq44_value),
            &eq44_node_derivatives,
            &eq44_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq45_e834, eq45_e834_d_n0, eq45_e834_d_n1, eq45_e834_d_n2, eq45_e834_d_n3, eq45_e834_d_n4, eq45_e834_d_n5, eq45_e834_d_n6, eq45_e834_d_n7, eq45_e834_d_n8, eq45_e834_d_n9, eq45_e834_d_n10, eq45_e834_d_n11, eq45_e834_d_n12, eq45_e834_d_n13, eq45_e834_d_n14, eq45_e834_d_n15, eq45_e834_d_n16, eq45_e834_d_n17, eq45_e834_d_n18, eq45_e834_d_n19, eq45_e834_d_n20, eq45_e834_d_n21, eq45_e834_d_n22, eq45_e834_d_b0, eq45_e834_d_b1, eq45_e834_d_b2, eq45_e834_d_b3, eq45_e834_d_b4, eq45_e834_d_b5, eq45_e834_d_b6, eq45_e834_d_b7, eq45_e834_d_b8, eq45_e834_d_b9, eq45_e834_d_b10, eq45_e834_d_b11, eq45_e834_d_b12, eq45_e834_d_b13, eq45_e834_d_b14, eq45_e834_d_b15, eq45_e834_d_b16, eq45_e834_d_b17, eq45_e834_d_b18, eq45_e834_d_b19, eq45_e834_d_b20, eq45_e834_d_b21, eq45_e834_d_b22, eq45_e834_d_b23, eq45_e834_d_b24, eq45_e834_d_b25, eq45_e834_d_b26, eq45_e834_d_b27, eq45_e834_d_b28, eq45_e834_d_b29, eq45_e834_d_b30, eq45_e834_d_b31, eq45_e834_d_b32, eq45_e834_d_b33, eq45_e834_d_b34, eq45_e834_d_b35, eq45_e834_d_b36, eq45_e834_d_b37, eq45_e834_d_b38, eq45_e834_d_b39, eq45_e834_d_b40, eq45_e834_d_b41, eq45_e834_d_b42, eq45_e834_d_b43, eq45_e834_d_b44, eq45_e834_d_b45, eq45_e834_d_b46, eq45_e834_d_b47, eq45_e834_d_b48, eq45_e834_d_b49, eq45_e834_d_b50, eq45_e834_d_b51, eq45_e834_d_b52, eq45_e834_d_b53, eq45_e834_d_b54,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        let eq45_e830: f64 = (p.p144 * s.v[367]);
        let eq45_e830_d_n0: f64 = (p.p144 * s.dn[367][0]);
        let eq45_e830_d_n1: f64 = (p.p144 * s.dn[367][1]);
        let eq45_e830_d_n2: f64 = (p.p144 * s.dn[367][2]);
        let eq45_e830_d_n3: f64 = (p.p144 * s.dn[367][3]);
        let eq45_e830_d_n4: f64 = (p.p144 * s.dn[367][4]);
        let eq45_e830_d_n5: f64 = (p.p144 * s.dn[367][5]);
        let eq45_e830_d_n6: f64 = (p.p144 * s.dn[367][6]);
        let eq45_e830_d_n7: f64 = (p.p144 * s.dn[367][7]);
        let eq45_e830_d_n8: f64 = (p.p144 * s.dn[367][8]);
        let eq45_e830_d_n9: f64 = (p.p144 * s.dn[367][9]);
        let eq45_e830_d_n10: f64 = (p.p144 * s.dn[367][10]);
        let eq45_e830_d_n11: f64 = (p.p144 * s.dn[367][11]);
        let eq45_e830_d_n12: f64 = (p.p144 * s.dn[367][12]);
        let eq45_e830_d_n13: f64 = (p.p144 * s.dn[367][13]);
        let eq45_e830_d_n14: f64 = (p.p144 * s.dn[367][14]);
        let eq45_e830_d_n15: f64 = (p.p144 * s.dn[367][15]);
        let eq45_e830_d_n16: f64 = (p.p144 * s.dn[367][16]);
        let eq45_e830_d_n17: f64 = (p.p144 * s.dn[367][17]);
        let eq45_e830_d_n18: f64 = (p.p144 * s.dn[367][18]);
        let eq45_e830_d_n19: f64 = (p.p144 * s.dn[367][19]);
        let eq45_e830_d_n20: f64 = (p.p144 * s.dn[367][20]);
        let eq45_e830_d_n21: f64 = (p.p144 * s.dn[367][21]);
        let eq45_e830_d_n22: f64 = (p.p144 * s.dn[367][22]);
        let eq45_e830_d_b0: f64 = (p.p144 * s.db[367][0]);
        let eq45_e830_d_b1: f64 = (p.p144 * s.db[367][1]);
        let eq45_e830_d_b2: f64 = (p.p144 * s.db[367][2]);
        let eq45_e830_d_b3: f64 = (p.p144 * s.db[367][3]);
        let eq45_e830_d_b4: f64 = (p.p144 * s.db[367][4]);
        let eq45_e830_d_b5: f64 = (p.p144 * s.db[367][5]);
        let eq45_e830_d_b6: f64 = (p.p144 * s.db[367][6]);
        let eq45_e830_d_b7: f64 = (p.p144 * s.db[367][7]);
        let eq45_e830_d_b8: f64 = (p.p144 * s.db[367][8]);
        let eq45_e830_d_b9: f64 = (p.p144 * s.db[367][9]);
        let eq45_e830_d_b10: f64 = (p.p144 * s.db[367][10]);
        let eq45_e830_d_b11: f64 = (p.p144 * s.db[367][11]);
        let eq45_e830_d_b12: f64 = (p.p144 * s.db[367][12]);
        let eq45_e830_d_b13: f64 = (p.p144 * s.db[367][13]);
        let eq45_e830_d_b14: f64 = (p.p144 * s.db[367][14]);
        let eq45_e830_d_b15: f64 = (p.p144 * s.db[367][15]);
        let eq45_e830_d_b16: f64 = (p.p144 * s.db[367][16]);
        let eq45_e830_d_b17: f64 = (p.p144 * s.db[367][17]);
        let eq45_e830_d_b18: f64 = (p.p144 * s.db[367][18]);
        let eq45_e830_d_b19: f64 = (p.p144 * s.db[367][19]);
        let eq45_e830_d_b20: f64 = (p.p144 * s.db[367][20]);
        let eq45_e830_d_b21: f64 = (p.p144 * s.db[367][21]);
        let eq45_e830_d_b22: f64 = (p.p144 * s.db[367][22]);
        let eq45_e830_d_b23: f64 = (p.p144 * s.db[367][23]);
        let eq45_e830_d_b24: f64 = (p.p144 * s.db[367][24]);
        let eq45_e830_d_b25: f64 = (p.p144 * s.db[367][25]);
        let eq45_e830_d_b26: f64 = (p.p144 * s.db[367][26]);
        let eq45_e830_d_b27: f64 = (p.p144 * s.db[367][27]);
        let eq45_e830_d_b28: f64 = (p.p144 * s.db[367][28]);
        let eq45_e830_d_b29: f64 = (p.p144 * s.db[367][29]);
        let eq45_e830_d_b30: f64 = (p.p144 * s.db[367][30]);
        let eq45_e830_d_b31: f64 = (p.p144 * s.db[367][31]);
        let eq45_e830_d_b32: f64 = (p.p144 * s.db[367][32]);
        let eq45_e830_d_b33: f64 = (p.p144 * s.db[367][33]);
        let eq45_e830_d_b34: f64 = (p.p144 * s.db[367][34]);
        let eq45_e830_d_b35: f64 = (p.p144 * s.db[367][35]);
        let eq45_e830_d_b36: f64 = (p.p144 * s.db[367][36]);
        let eq45_e830_d_b37: f64 = (p.p144 * s.db[367][37]);
        let eq45_e830_d_b38: f64 = (p.p144 * s.db[367][38]);
        let eq45_e830_d_b39: f64 = (p.p144 * s.db[367][39]);
        let eq45_e830_d_b40: f64 = (p.p144 * s.db[367][40]);
        let eq45_e830_d_b41: f64 = (p.p144 * s.db[367][41]);
        let eq45_e830_d_b42: f64 = (p.p144 * s.db[367][42]);
        let eq45_e830_d_b43: f64 = (p.p144 * s.db[367][43]);
        let eq45_e830_d_b44: f64 = (p.p144 * s.db[367][44]);
        let eq45_e830_d_b45: f64 = (p.p144 * s.db[367][45]);
        let eq45_e830_d_b46: f64 = (p.p144 * s.db[367][46]);
        let eq45_e830_d_b47: f64 = (p.p144 * s.db[367][47]);
        let eq45_e830_d_b48: f64 = (p.p144 * s.db[367][48]);
        let eq45_e830_d_b49: f64 = (p.p144 * s.db[367][49]);
        let eq45_e830_d_b50: f64 = (p.p144 * s.db[367][50]);
        let eq45_e830_d_b51: f64 = (p.p144 * s.db[367][51]);
        let eq45_e830_d_b52: f64 = (p.p144 * s.db[367][52]);
        let eq45_e830_d_b53: f64 = (p.p144 * s.db[367][53]);
        let eq45_e830_d_b54: f64 = (p.p144 * s.db[367][54]);
        let eq45_e832: f64 = (eq45_e830 * (nv6 - 0.0));
        let eq45_e832_d_n0: f64 = (eq45_e830_d_n0 * (nv6 - 0.0));
        let eq45_e832_d_n1: f64 = (eq45_e830_d_n1 * (nv6 - 0.0));
        let eq45_e832_d_n2: f64 = (eq45_e830_d_n2 * (nv6 - 0.0));
        let eq45_e832_d_n3: f64 = (eq45_e830_d_n3 * (nv6 - 0.0));
        let eq45_e832_d_n4: f64 = (eq45_e830_d_n4 * (nv6 - 0.0));
        let eq45_e832_d_n5: f64 = (eq45_e830_d_n5 * (nv6 - 0.0));
        let eq45_e832_d_n6: f64 = ((eq45_e830_d_n6 * (nv6 - 0.0)) + eq45_e830);
        let eq45_e832_d_n7: f64 = (eq45_e830_d_n7 * (nv6 - 0.0));
        let eq45_e832_d_n8: f64 = (eq45_e830_d_n8 * (nv6 - 0.0));
        let eq45_e832_d_n9: f64 = (eq45_e830_d_n9 * (nv6 - 0.0));
        let eq45_e832_d_n10: f64 = (eq45_e830_d_n10 * (nv6 - 0.0));
        let eq45_e832_d_n11: f64 = (eq45_e830_d_n11 * (nv6 - 0.0));
        let eq45_e832_d_n12: f64 = (eq45_e830_d_n12 * (nv6 - 0.0));
        let eq45_e832_d_n13: f64 = (eq45_e830_d_n13 * (nv6 - 0.0));
        let eq45_e832_d_n14: f64 = (eq45_e830_d_n14 * (nv6 - 0.0));
        let eq45_e832_d_n15: f64 = (eq45_e830_d_n15 * (nv6 - 0.0));
        let eq45_e832_d_n16: f64 = (eq45_e830_d_n16 * (nv6 - 0.0));
        let eq45_e832_d_n17: f64 = (eq45_e830_d_n17 * (nv6 - 0.0));
        let eq45_e832_d_n18: f64 = (eq45_e830_d_n18 * (nv6 - 0.0));
        let eq45_e832_d_n19: f64 = (eq45_e830_d_n19 * (nv6 - 0.0));
        let eq45_e832_d_n20: f64 = (eq45_e830_d_n20 * (nv6 - 0.0));
        let eq45_e832_d_n21: f64 = (eq45_e830_d_n21 * (nv6 - 0.0));
        let eq45_e832_d_n22: f64 = (eq45_e830_d_n22 * (nv6 - 0.0));
        let eq45_e832_d_b0: f64 = (eq45_e830_d_b0 * (nv6 - 0.0));
        let eq45_e832_d_b1: f64 = (eq45_e830_d_b1 * (nv6 - 0.0));
        let eq45_e832_d_b2: f64 = (eq45_e830_d_b2 * (nv6 - 0.0));
        let eq45_e832_d_b3: f64 = (eq45_e830_d_b3 * (nv6 - 0.0));
        let eq45_e832_d_b4: f64 = (eq45_e830_d_b4 * (nv6 - 0.0));
        let eq45_e832_d_b5: f64 = (eq45_e830_d_b5 * (nv6 - 0.0));
        let eq45_e832_d_b6: f64 = (eq45_e830_d_b6 * (nv6 - 0.0));
        let eq45_e832_d_b7: f64 = (eq45_e830_d_b7 * (nv6 - 0.0));
        let eq45_e832_d_b8: f64 = (eq45_e830_d_b8 * (nv6 - 0.0));
        let eq45_e832_d_b9: f64 = (eq45_e830_d_b9 * (nv6 - 0.0));
        let eq45_e832_d_b10: f64 = (eq45_e830_d_b10 * (nv6 - 0.0));
        let eq45_e832_d_b11: f64 = (eq45_e830_d_b11 * (nv6 - 0.0));
        let eq45_e832_d_b12: f64 = (eq45_e830_d_b12 * (nv6 - 0.0));
        let eq45_e832_d_b13: f64 = (eq45_e830_d_b13 * (nv6 - 0.0));
        let eq45_e832_d_b14: f64 = (eq45_e830_d_b14 * (nv6 - 0.0));
        let eq45_e832_d_b15: f64 = (eq45_e830_d_b15 * (nv6 - 0.0));
        let eq45_e832_d_b16: f64 = (eq45_e830_d_b16 * (nv6 - 0.0));
        let eq45_e832_d_b17: f64 = (eq45_e830_d_b17 * (nv6 - 0.0));
        let eq45_e832_d_b18: f64 = (eq45_e830_d_b18 * (nv6 - 0.0));
        let eq45_e832_d_b19: f64 = (eq45_e830_d_b19 * (nv6 - 0.0));
        let eq45_e832_d_b20: f64 = (eq45_e830_d_b20 * (nv6 - 0.0));
        let eq45_e832_d_b21: f64 = (eq45_e830_d_b21 * (nv6 - 0.0));
        let eq45_e832_d_b22: f64 = (eq45_e830_d_b22 * (nv6 - 0.0));
        let eq45_e832_d_b23: f64 = (eq45_e830_d_b23 * (nv6 - 0.0));
        let eq45_e832_d_b24: f64 = (eq45_e830_d_b24 * (nv6 - 0.0));
        let eq45_e832_d_b25: f64 = (eq45_e830_d_b25 * (nv6 - 0.0));
        let eq45_e832_d_b26: f64 = (eq45_e830_d_b26 * (nv6 - 0.0));
        let eq45_e832_d_b27: f64 = (eq45_e830_d_b27 * (nv6 - 0.0));
        let eq45_e832_d_b28: f64 = (eq45_e830_d_b28 * (nv6 - 0.0));
        let eq45_e832_d_b29: f64 = (eq45_e830_d_b29 * (nv6 - 0.0));
        let eq45_e832_d_b30: f64 = (eq45_e830_d_b30 * (nv6 - 0.0));
        let eq45_e832_d_b31: f64 = (eq45_e830_d_b31 * (nv6 - 0.0));
        let eq45_e832_d_b32: f64 = (eq45_e830_d_b32 * (nv6 - 0.0));
        let eq45_e832_d_b33: f64 = (eq45_e830_d_b33 * (nv6 - 0.0));
        let eq45_e832_d_b34: f64 = (eq45_e830_d_b34 * (nv6 - 0.0));
        let eq45_e832_d_b35: f64 = (eq45_e830_d_b35 * (nv6 - 0.0));
        let eq45_e832_d_b36: f64 = (eq45_e830_d_b36 * (nv6 - 0.0));
        let eq45_e832_d_b37: f64 = (eq45_e830_d_b37 * (nv6 - 0.0));
        let eq45_e832_d_b38: f64 = (eq45_e830_d_b38 * (nv6 - 0.0));
        let eq45_e832_d_b39: f64 = (eq45_e830_d_b39 * (nv6 - 0.0));
        let eq45_e832_d_b40: f64 = (eq45_e830_d_b40 * (nv6 - 0.0));
        let eq45_e832_d_b41: f64 = (eq45_e830_d_b41 * (nv6 - 0.0));
        let eq45_e832_d_b42: f64 = (eq45_e830_d_b42 * (nv6 - 0.0));
        let eq45_e832_d_b43: f64 = (eq45_e830_d_b43 * (nv6 - 0.0));
        let eq45_e832_d_b44: f64 = (eq45_e830_d_b44 * (nv6 - 0.0));
        let eq45_e832_d_b45: f64 = (eq45_e830_d_b45 * (nv6 - 0.0));
        let eq45_e832_d_b46: f64 = (eq45_e830_d_b46 * (nv6 - 0.0));
        let eq45_e832_d_b47: f64 = (eq45_e830_d_b47 * (nv6 - 0.0));
        let eq45_e832_d_b48: f64 = (eq45_e830_d_b48 * (nv6 - 0.0));
        let eq45_e832_d_b49: f64 = (eq45_e830_d_b49 * (nv6 - 0.0));
        let eq45_e832_d_b50: f64 = (eq45_e830_d_b50 * (nv6 - 0.0));
        let eq45_e832_d_b51: f64 = (eq45_e830_d_b51 * (nv6 - 0.0));
        let eq45_e832_d_b52: f64 = (eq45_e830_d_b52 * (nv6 - 0.0));
        let eq45_e832_d_b53: f64 = (eq45_e830_d_b53 * (nv6 - 0.0));
        let eq45_e832_d_b54: f64 = (eq45_e830_d_b54 * (nv6 - 0.0));
        (eq45_e832, eq45_e832_d_n0, eq45_e832_d_n1, eq45_e832_d_n2, eq45_e832_d_n3, eq45_e832_d_n4, eq45_e832_d_n5, eq45_e832_d_n6, eq45_e832_d_n7, eq45_e832_d_n8, eq45_e832_d_n9, eq45_e832_d_n10, eq45_e832_d_n11, eq45_e832_d_n12, eq45_e832_d_n13, eq45_e832_d_n14, eq45_e832_d_n15, eq45_e832_d_n16, eq45_e832_d_n17, eq45_e832_d_n18, eq45_e832_d_n19, eq45_e832_d_n20, eq45_e832_d_n21, eq45_e832_d_n22, eq45_e832_d_b0, eq45_e832_d_b1, eq45_e832_d_b2, eq45_e832_d_b3, eq45_e832_d_b4, eq45_e832_d_b5, eq45_e832_d_b6, eq45_e832_d_b7, eq45_e832_d_b8, eq45_e832_d_b9, eq45_e832_d_b10, eq45_e832_d_b11, eq45_e832_d_b12, eq45_e832_d_b13, eq45_e832_d_b14, eq45_e832_d_b15, eq45_e832_d_b16, eq45_e832_d_b17, eq45_e832_d_b18, eq45_e832_d_b19, eq45_e832_d_b20, eq45_e832_d_b21, eq45_e832_d_b22, eq45_e832_d_b23, eq45_e832_d_b24, eq45_e832_d_b25, eq45_e832_d_b26, eq45_e832_d_b27, eq45_e832_d_b28, eq45_e832_d_b29, eq45_e832_d_b30, eq45_e832_d_b31, eq45_e832_d_b32, eq45_e832_d_b33, eq45_e832_d_b34, eq45_e832_d_b35, eq45_e832_d_b36, eq45_e832_d_b37, eq45_e832_d_b38, eq45_e832_d_b39, eq45_e832_d_b40, eq45_e832_d_b41, eq45_e832_d_b42, eq45_e832_d_b43, eq45_e832_d_b44, eq45_e832_d_b45, eq45_e832_d_b46, eq45_e832_d_b47, eq45_e832_d_b48, eq45_e832_d_b49, eq45_e832_d_b50, eq45_e832_d_b51, eq45_e832_d_b52, eq45_e832_d_b53, eq45_e832_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq45_value: f64 = eq45_e834;
        let eq45_node_derivatives: [f64; 23] = [eq45_e834_d_n0, eq45_e834_d_n1, eq45_e834_d_n2, eq45_e834_d_n3, eq45_e834_d_n4, eq45_e834_d_n5, eq45_e834_d_n6, eq45_e834_d_n7, eq45_e834_d_n8, eq45_e834_d_n9, eq45_e834_d_n10, eq45_e834_d_n11, eq45_e834_d_n12, eq45_e834_d_n13, eq45_e834_d_n14, eq45_e834_d_n15, eq45_e834_d_n16, eq45_e834_d_n17, eq45_e834_d_n18, eq45_e834_d_n19, eq45_e834_d_n20, eq45_e834_d_n21, eq45_e834_d_n22];
        let eq45_branch_derivatives: [f64; 55] = [eq45_e834_d_b0, eq45_e834_d_b1, eq45_e834_d_b2, eq45_e834_d_b3, eq45_e834_d_b4, eq45_e834_d_b5, eq45_e834_d_b6, eq45_e834_d_b7, eq45_e834_d_b8, eq45_e834_d_b9, eq45_e834_d_b10, eq45_e834_d_b11, eq45_e834_d_b12, eq45_e834_d_b13, eq45_e834_d_b14, eq45_e834_d_b15, eq45_e834_d_b16, eq45_e834_d_b17, eq45_e834_d_b18, eq45_e834_d_b19, eq45_e834_d_b20, eq45_e834_d_b21, eq45_e834_d_b22, eq45_e834_d_b23, eq45_e834_d_b24, eq45_e834_d_b25, eq45_e834_d_b26, eq45_e834_d_b27, eq45_e834_d_b28, eq45_e834_d_b29, eq45_e834_d_b30, eq45_e834_d_b31, eq45_e834_d_b32, eq45_e834_d_b33, eq45_e834_d_b34, eq45_e834_d_b35, eq45_e834_d_b36, eq45_e834_d_b37, eq45_e834_d_b38, eq45_e834_d_b39, eq45_e834_d_b40, eq45_e834_d_b41, eq45_e834_d_b42, eq45_e834_d_b43, eq45_e834_d_b44, eq45_e834_d_b45, eq45_e834_d_b46, eq45_e834_d_b47, eq45_e834_d_b48, eq45_e834_d_b49, eq45_e834_d_b50, eq45_e834_d_b51, eq45_e834_d_b52, eq45_e834_d_b53, eq45_e834_d_b54];
        stamper.stamp_current_dense_local(
            Some(6),
            None,
            multiplicity * (eq45_value),
            &eq45_node_derivatives,
            &eq45_branch_derivatives,
            multiplicity,
        );
        let (eq46_e852, eq46_e852_d_n6,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        let eq46_e849: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, (nv6 - 0.0));
        let eq46_e850: f64 = (p.p144 * eq46_e849);
        (eq46_e850, (p.p144 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e852;
        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * (eq46_value),
            6,
            multiplicity * (eq46_e852_d_n6),
        );
    }

    pub(super) fn stamp_transient_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let eq51_e915: f64 = (p.p6 * s.v[41]);
        let eq51_e915_d_n0: f64 = (p.p6 * s.dn[41][0]);
        let eq51_e915_d_n1: f64 = (p.p6 * s.dn[41][1]);
        let eq51_e915_d_n2: f64 = (p.p6 * s.dn[41][2]);
        let eq51_e915_d_n3: f64 = (p.p6 * s.dn[41][3]);
        let eq51_e915_d_n4: f64 = (p.p6 * s.dn[41][4]);
        let eq51_e915_d_n5: f64 = (p.p6 * s.dn[41][5]);
        let eq51_e915_d_n6: f64 = (p.p6 * s.dn[41][6]);
        let eq51_e915_d_n7: f64 = (p.p6 * s.dn[41][7]);
        let eq51_e915_d_n8: f64 = (p.p6 * s.dn[41][8]);
        let eq51_e915_d_n9: f64 = (p.p6 * s.dn[41][9]);
        let eq51_e915_d_n10: f64 = (p.p6 * s.dn[41][10]);
        let eq51_e915_d_n11: f64 = (p.p6 * s.dn[41][11]);
        let eq51_e915_d_n12: f64 = (p.p6 * s.dn[41][12]);
        let eq51_e915_d_n13: f64 = (p.p6 * s.dn[41][13]);
        let eq51_e915_d_n14: f64 = (p.p6 * s.dn[41][14]);
        let eq51_e915_d_n15: f64 = (p.p6 * s.dn[41][15]);
        let eq51_e915_d_n16: f64 = (p.p6 * s.dn[41][16]);
        let eq51_e915_d_n17: f64 = (p.p6 * s.dn[41][17]);
        let eq51_e915_d_n18: f64 = (p.p6 * s.dn[41][18]);
        let eq51_e915_d_n19: f64 = (p.p6 * s.dn[41][19]);
        let eq51_e915_d_n20: f64 = (p.p6 * s.dn[41][20]);
        let eq51_e915_d_n21: f64 = (p.p6 * s.dn[41][21]);
        let eq51_e915_d_n22: f64 = (p.p6 * s.dn[41][22]);
        let eq51_e915_d_b0: f64 = (p.p6 * s.db[41][0]);
        let eq51_e915_d_b1: f64 = (p.p6 * s.db[41][1]);
        let eq51_e915_d_b2: f64 = (p.p6 * s.db[41][2]);
        let eq51_e915_d_b3: f64 = (p.p6 * s.db[41][3]);
        let eq51_e915_d_b4: f64 = (p.p6 * s.db[41][4]);
        let eq51_e915_d_b5: f64 = (p.p6 * s.db[41][5]);
        let eq51_e915_d_b6: f64 = (p.p6 * s.db[41][6]);
        let eq51_e915_d_b7: f64 = (p.p6 * s.db[41][7]);
        let eq51_e915_d_b8: f64 = (p.p6 * s.db[41][8]);
        let eq51_e915_d_b9: f64 = (p.p6 * s.db[41][9]);
        let eq51_e915_d_b10: f64 = (p.p6 * s.db[41][10]);
        let eq51_e915_d_b11: f64 = (p.p6 * s.db[41][11]);
        let eq51_e915_d_b12: f64 = (p.p6 * s.db[41][12]);
        let eq51_e915_d_b13: f64 = (p.p6 * s.db[41][13]);
        let eq51_e915_d_b14: f64 = (p.p6 * s.db[41][14]);
        let eq51_e915_d_b15: f64 = (p.p6 * s.db[41][15]);
        let eq51_e915_d_b16: f64 = (p.p6 * s.db[41][16]);
        let eq51_e915_d_b17: f64 = (p.p6 * s.db[41][17]);
        let eq51_e915_d_b18: f64 = (p.p6 * s.db[41][18]);
        let eq51_e915_d_b19: f64 = (p.p6 * s.db[41][19]);
        let eq51_e915_d_b20: f64 = (p.p6 * s.db[41][20]);
        let eq51_e915_d_b21: f64 = (p.p6 * s.db[41][21]);
        let eq51_e915_d_b22: f64 = (p.p6 * s.db[41][22]);
        let eq51_e915_d_b23: f64 = (p.p6 * s.db[41][23]);
        let eq51_e915_d_b24: f64 = (p.p6 * s.db[41][24]);
        let eq51_e915_d_b25: f64 = (p.p6 * s.db[41][25]);
        let eq51_e915_d_b26: f64 = (p.p6 * s.db[41][26]);
        let eq51_e915_d_b27: f64 = (p.p6 * s.db[41][27]);
        let eq51_e915_d_b28: f64 = (p.p6 * s.db[41][28]);
        let eq51_e915_d_b29: f64 = (p.p6 * s.db[41][29]);
        let eq51_e915_d_b30: f64 = (p.p6 * s.db[41][30]);
        let eq51_e915_d_b31: f64 = (p.p6 * s.db[41][31]);
        let eq51_e915_d_b32: f64 = (p.p6 * s.db[41][32]);
        let eq51_e915_d_b33: f64 = (p.p6 * s.db[41][33]);
        let eq51_e915_d_b34: f64 = (p.p6 * s.db[41][34]);
        let eq51_e915_d_b35: f64 = (p.p6 * s.db[41][35]);
        let eq51_e915_d_b36: f64 = (p.p6 * s.db[41][36]);
        let eq51_e915_d_b37: f64 = (p.p6 * s.db[41][37]);
        let eq51_e915_d_b38: f64 = (p.p6 * s.db[41][38]);
        let eq51_e915_d_b39: f64 = (p.p6 * s.db[41][39]);
        let eq51_e915_d_b40: f64 = (p.p6 * s.db[41][40]);
        let eq51_e915_d_b41: f64 = (p.p6 * s.db[41][41]);
        let eq51_e915_d_b42: f64 = (p.p6 * s.db[41][42]);
        let eq51_e915_d_b43: f64 = (p.p6 * s.db[41][43]);
        let eq51_e915_d_b44: f64 = (p.p6 * s.db[41][44]);
        let eq51_e915_d_b45: f64 = (p.p6 * s.db[41][45]);
        let eq51_e915_d_b46: f64 = (p.p6 * s.db[41][46]);
        let eq51_e915_d_b47: f64 = (p.p6 * s.db[41][47]);
        let eq51_e915_d_b48: f64 = (p.p6 * s.db[41][48]);
        let eq51_e915_d_b49: f64 = (p.p6 * s.db[41][49]);
        let eq51_e915_d_b50: f64 = (p.p6 * s.db[41][50]);
        let eq51_e915_d_b51: f64 = (p.p6 * s.db[41][51]);
        let eq51_e915_d_b52: f64 = (p.p6 * s.db[41][52]);
        let eq51_e915_d_b53: f64 = (p.p6 * s.db[41][53]);
        let eq51_e915_d_b54: f64 = (p.p6 * s.db[41][54]);
        let eq51_e917: f64 = (eq51_e915 * s.v[94]);
        let eq51_e917_d_n0: f64 = ((eq51_e915_d_n0 * s.v[94]) + (eq51_e915 * s.dn[94][0]));
        let eq51_e917_d_n1: f64 = ((eq51_e915_d_n1 * s.v[94]) + (eq51_e915 * s.dn[94][1]));
        let eq51_e917_d_n2: f64 = ((eq51_e915_d_n2 * s.v[94]) + (eq51_e915 * s.dn[94][2]));
        let eq51_e917_d_n3: f64 = ((eq51_e915_d_n3 * s.v[94]) + (eq51_e915 * s.dn[94][3]));
        let eq51_e917_d_n4: f64 = ((eq51_e915_d_n4 * s.v[94]) + (eq51_e915 * s.dn[94][4]));
        let eq51_e917_d_n5: f64 = ((eq51_e915_d_n5 * s.v[94]) + (eq51_e915 * s.dn[94][5]));
        let eq51_e917_d_n6: f64 = ((eq51_e915_d_n6 * s.v[94]) + (eq51_e915 * s.dn[94][6]));
        let eq51_e917_d_n7: f64 = ((eq51_e915_d_n7 * s.v[94]) + (eq51_e915 * s.dn[94][7]));
        let eq51_e917_d_n8: f64 = ((eq51_e915_d_n8 * s.v[94]) + (eq51_e915 * s.dn[94][8]));
        let eq51_e917_d_n9: f64 = ((eq51_e915_d_n9 * s.v[94]) + (eq51_e915 * s.dn[94][9]));
        let eq51_e917_d_n10: f64 = ((eq51_e915_d_n10 * s.v[94]) + (eq51_e915 * s.dn[94][10]));
        let eq51_e917_d_n11: f64 = ((eq51_e915_d_n11 * s.v[94]) + (eq51_e915 * s.dn[94][11]));
        let eq51_e917_d_n12: f64 = ((eq51_e915_d_n12 * s.v[94]) + (eq51_e915 * s.dn[94][12]));
        let eq51_e917_d_n13: f64 = ((eq51_e915_d_n13 * s.v[94]) + (eq51_e915 * s.dn[94][13]));
        let eq51_e917_d_n14: f64 = ((eq51_e915_d_n14 * s.v[94]) + (eq51_e915 * s.dn[94][14]));
        let eq51_e917_d_n15: f64 = ((eq51_e915_d_n15 * s.v[94]) + (eq51_e915 * s.dn[94][15]));
        let eq51_e917_d_n16: f64 = ((eq51_e915_d_n16 * s.v[94]) + (eq51_e915 * s.dn[94][16]));
        let eq51_e917_d_n17: f64 = ((eq51_e915_d_n17 * s.v[94]) + (eq51_e915 * s.dn[94][17]));
        let eq51_e917_d_n18: f64 = ((eq51_e915_d_n18 * s.v[94]) + (eq51_e915 * s.dn[94][18]));
        let eq51_e917_d_n19: f64 = ((eq51_e915_d_n19 * s.v[94]) + (eq51_e915 * s.dn[94][19]));
        let eq51_e917_d_n20: f64 = ((eq51_e915_d_n20 * s.v[94]) + (eq51_e915 * s.dn[94][20]));
        let eq51_e917_d_n21: f64 = ((eq51_e915_d_n21 * s.v[94]) + (eq51_e915 * s.dn[94][21]));
        let eq51_e917_d_n22: f64 = ((eq51_e915_d_n22 * s.v[94]) + (eq51_e915 * s.dn[94][22]));
        let eq51_e917_d_b0: f64 = ((eq51_e915_d_b0 * s.v[94]) + (eq51_e915 * s.db[94][0]));
        let eq51_e917_d_b1: f64 = ((eq51_e915_d_b1 * s.v[94]) + (eq51_e915 * s.db[94][1]));
        let eq51_e917_d_b2: f64 = ((eq51_e915_d_b2 * s.v[94]) + (eq51_e915 * s.db[94][2]));
        let eq51_e917_d_b3: f64 = ((eq51_e915_d_b3 * s.v[94]) + (eq51_e915 * s.db[94][3]));
        let eq51_e917_d_b4: f64 = ((eq51_e915_d_b4 * s.v[94]) + (eq51_e915 * s.db[94][4]));
        let eq51_e917_d_b5: f64 = ((eq51_e915_d_b5 * s.v[94]) + (eq51_e915 * s.db[94][5]));
        let eq51_e917_d_b6: f64 = ((eq51_e915_d_b6 * s.v[94]) + (eq51_e915 * s.db[94][6]));
        let eq51_e917_d_b7: f64 = ((eq51_e915_d_b7 * s.v[94]) + (eq51_e915 * s.db[94][7]));
        let eq51_e917_d_b8: f64 = ((eq51_e915_d_b8 * s.v[94]) + (eq51_e915 * s.db[94][8]));
        let eq51_e917_d_b9: f64 = ((eq51_e915_d_b9 * s.v[94]) + (eq51_e915 * s.db[94][9]));
        let eq51_e917_d_b10: f64 = ((eq51_e915_d_b10 * s.v[94]) + (eq51_e915 * s.db[94][10]));
        let eq51_e917_d_b11: f64 = ((eq51_e915_d_b11 * s.v[94]) + (eq51_e915 * s.db[94][11]));
        let eq51_e917_d_b12: f64 = ((eq51_e915_d_b12 * s.v[94]) + (eq51_e915 * s.db[94][12]));
        let eq51_e917_d_b13: f64 = ((eq51_e915_d_b13 * s.v[94]) + (eq51_e915 * s.db[94][13]));
        let eq51_e917_d_b14: f64 = ((eq51_e915_d_b14 * s.v[94]) + (eq51_e915 * s.db[94][14]));
        let eq51_e917_d_b15: f64 = ((eq51_e915_d_b15 * s.v[94]) + (eq51_e915 * s.db[94][15]));
        let eq51_e917_d_b16: f64 = ((eq51_e915_d_b16 * s.v[94]) + (eq51_e915 * s.db[94][16]));
        let eq51_e917_d_b17: f64 = ((eq51_e915_d_b17 * s.v[94]) + (eq51_e915 * s.db[94][17]));
        let eq51_e917_d_b18: f64 = ((eq51_e915_d_b18 * s.v[94]) + (eq51_e915 * s.db[94][18]));
        let eq51_e917_d_b19: f64 = ((eq51_e915_d_b19 * s.v[94]) + (eq51_e915 * s.db[94][19]));
        let eq51_e917_d_b20: f64 = ((eq51_e915_d_b20 * s.v[94]) + (eq51_e915 * s.db[94][20]));
        let eq51_e917_d_b21: f64 = ((eq51_e915_d_b21 * s.v[94]) + (eq51_e915 * s.db[94][21]));
        let eq51_e917_d_b22: f64 = ((eq51_e915_d_b22 * s.v[94]) + (eq51_e915 * s.db[94][22]));
        let eq51_e917_d_b23: f64 = ((eq51_e915_d_b23 * s.v[94]) + (eq51_e915 * s.db[94][23]));
        let eq51_e917_d_b24: f64 = ((eq51_e915_d_b24 * s.v[94]) + (eq51_e915 * s.db[94][24]));
        let eq51_e917_d_b25: f64 = ((eq51_e915_d_b25 * s.v[94]) + (eq51_e915 * s.db[94][25]));
        let eq51_e917_d_b26: f64 = ((eq51_e915_d_b26 * s.v[94]) + (eq51_e915 * s.db[94][26]));
        let eq51_e917_d_b27: f64 = ((eq51_e915_d_b27 * s.v[94]) + (eq51_e915 * s.db[94][27]));
        let eq51_e917_d_b28: f64 = ((eq51_e915_d_b28 * s.v[94]) + (eq51_e915 * s.db[94][28]));
        let eq51_e917_d_b29: f64 = ((eq51_e915_d_b29 * s.v[94]) + (eq51_e915 * s.db[94][29]));
        let eq51_e917_d_b30: f64 = ((eq51_e915_d_b30 * s.v[94]) + (eq51_e915 * s.db[94][30]));
        let eq51_e917_d_b31: f64 = ((eq51_e915_d_b31 * s.v[94]) + (eq51_e915 * s.db[94][31]));
        let eq51_e917_d_b32: f64 = ((eq51_e915_d_b32 * s.v[94]) + (eq51_e915 * s.db[94][32]));
        let eq51_e917_d_b33: f64 = ((eq51_e915_d_b33 * s.v[94]) + (eq51_e915 * s.db[94][33]));
        let eq51_e917_d_b34: f64 = ((eq51_e915_d_b34 * s.v[94]) + (eq51_e915 * s.db[94][34]));
        let eq51_e917_d_b35: f64 = ((eq51_e915_d_b35 * s.v[94]) + (eq51_e915 * s.db[94][35]));
        let eq51_e917_d_b36: f64 = ((eq51_e915_d_b36 * s.v[94]) + (eq51_e915 * s.db[94][36]));
        let eq51_e917_d_b37: f64 = ((eq51_e915_d_b37 * s.v[94]) + (eq51_e915 * s.db[94][37]));
        let eq51_e917_d_b38: f64 = ((eq51_e915_d_b38 * s.v[94]) + (eq51_e915 * s.db[94][38]));
        let eq51_e917_d_b39: f64 = ((eq51_e915_d_b39 * s.v[94]) + (eq51_e915 * s.db[94][39]));
        let eq51_e917_d_b40: f64 = ((eq51_e915_d_b40 * s.v[94]) + (eq51_e915 * s.db[94][40]));
        let eq51_e917_d_b41: f64 = ((eq51_e915_d_b41 * s.v[94]) + (eq51_e915 * s.db[94][41]));
        let eq51_e917_d_b42: f64 = ((eq51_e915_d_b42 * s.v[94]) + (eq51_e915 * s.db[94][42]));
        let eq51_e917_d_b43: f64 = ((eq51_e915_d_b43 * s.v[94]) + (eq51_e915 * s.db[94][43]));
        let eq51_e917_d_b44: f64 = ((eq51_e915_d_b44 * s.v[94]) + (eq51_e915 * s.db[94][44]));
        let eq51_e917_d_b45: f64 = ((eq51_e915_d_b45 * s.v[94]) + (eq51_e915 * s.db[94][45]));
        let eq51_e917_d_b46: f64 = ((eq51_e915_d_b46 * s.v[94]) + (eq51_e915 * s.db[94][46]));
        let eq51_e917_d_b47: f64 = ((eq51_e915_d_b47 * s.v[94]) + (eq51_e915 * s.db[94][47]));
        let eq51_e917_d_b48: f64 = ((eq51_e915_d_b48 * s.v[94]) + (eq51_e915 * s.db[94][48]));
        let eq51_e917_d_b49: f64 = ((eq51_e915_d_b49 * s.v[94]) + (eq51_e915 * s.db[94][49]));
        let eq51_e917_d_b50: f64 = ((eq51_e915_d_b50 * s.v[94]) + (eq51_e915 * s.db[94][50]));
        let eq51_e917_d_b51: f64 = ((eq51_e915_d_b51 * s.v[94]) + (eq51_e915 * s.db[94][51]));
        let eq51_e917_d_b52: f64 = ((eq51_e915_d_b52 * s.v[94]) + (eq51_e915 * s.db[94][52]));
        let eq51_e917_d_b53: f64 = ((eq51_e915_d_b53 * s.v[94]) + (eq51_e915 * s.db[94][53]));
        let eq51_e917_d_b54: f64 = ((eq51_e915_d_b54 * s.v[94]) + (eq51_e915 * s.db[94][54]));
        let eq51_e920: f64 = (p.p6 * s.v[379]);
        let eq51_e920_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq51_e920_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq51_e920_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq51_e920_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq51_e920_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq51_e920_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq51_e920_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq51_e920_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq51_e920_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq51_e920_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq51_e920_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq51_e920_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq51_e920_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq51_e920_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq51_e920_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq51_e920_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq51_e920_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq51_e920_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq51_e920_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq51_e920_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq51_e920_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq51_e920_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq51_e920_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq51_e920_d_b0: f64 = (p.p6 * s.db[379][0]);
        let eq51_e920_d_b1: f64 = (p.p6 * s.db[379][1]);
        let eq51_e920_d_b2: f64 = (p.p6 * s.db[379][2]);
        let eq51_e920_d_b3: f64 = (p.p6 * s.db[379][3]);
        let eq51_e920_d_b4: f64 = (p.p6 * s.db[379][4]);
        let eq51_e920_d_b5: f64 = (p.p6 * s.db[379][5]);
        let eq51_e920_d_b6: f64 = (p.p6 * s.db[379][6]);
        let eq51_e920_d_b7: f64 = (p.p6 * s.db[379][7]);
        let eq51_e920_d_b8: f64 = (p.p6 * s.db[379][8]);
        let eq51_e920_d_b9: f64 = (p.p6 * s.db[379][9]);
        let eq51_e920_d_b10: f64 = (p.p6 * s.db[379][10]);
        let eq51_e920_d_b11: f64 = (p.p6 * s.db[379][11]);
        let eq51_e920_d_b12: f64 = (p.p6 * s.db[379][12]);
        let eq51_e920_d_b13: f64 = (p.p6 * s.db[379][13]);
        let eq51_e920_d_b14: f64 = (p.p6 * s.db[379][14]);
        let eq51_e920_d_b15: f64 = (p.p6 * s.db[379][15]);
        let eq51_e920_d_b16: f64 = (p.p6 * s.db[379][16]);
        let eq51_e920_d_b17: f64 = (p.p6 * s.db[379][17]);
        let eq51_e920_d_b18: f64 = (p.p6 * s.db[379][18]);
        let eq51_e920_d_b19: f64 = (p.p6 * s.db[379][19]);
        let eq51_e920_d_b20: f64 = (p.p6 * s.db[379][20]);
        let eq51_e920_d_b21: f64 = (p.p6 * s.db[379][21]);
        let eq51_e920_d_b22: f64 = (p.p6 * s.db[379][22]);
        let eq51_e920_d_b23: f64 = (p.p6 * s.db[379][23]);
        let eq51_e920_d_b24: f64 = (p.p6 * s.db[379][24]);
        let eq51_e920_d_b25: f64 = (p.p6 * s.db[379][25]);
        let eq51_e920_d_b26: f64 = (p.p6 * s.db[379][26]);
        let eq51_e920_d_b27: f64 = (p.p6 * s.db[379][27]);
        let eq51_e920_d_b28: f64 = (p.p6 * s.db[379][28]);
        let eq51_e920_d_b29: f64 = (p.p6 * s.db[379][29]);
        let eq51_e920_d_b30: f64 = (p.p6 * s.db[379][30]);
        let eq51_e920_d_b31: f64 = (p.p6 * s.db[379][31]);
        let eq51_e920_d_b32: f64 = (p.p6 * s.db[379][32]);
        let eq51_e920_d_b33: f64 = (p.p6 * s.db[379][33]);
        let eq51_e920_d_b34: f64 = (p.p6 * s.db[379][34]);
        let eq51_e920_d_b35: f64 = (p.p6 * s.db[379][35]);
        let eq51_e920_d_b36: f64 = (p.p6 * s.db[379][36]);
        let eq51_e920_d_b37: f64 = (p.p6 * s.db[379][37]);
        let eq51_e920_d_b38: f64 = (p.p6 * s.db[379][38]);
        let eq51_e920_d_b39: f64 = (p.p6 * s.db[379][39]);
        let eq51_e920_d_b40: f64 = (p.p6 * s.db[379][40]);
        let eq51_e920_d_b41: f64 = (p.p6 * s.db[379][41]);
        let eq51_e920_d_b42: f64 = (p.p6 * s.db[379][42]);
        let eq51_e920_d_b43: f64 = (p.p6 * s.db[379][43]);
        let eq51_e920_d_b44: f64 = (p.p6 * s.db[379][44]);
        let eq51_e920_d_b45: f64 = (p.p6 * s.db[379][45]);
        let eq51_e920_d_b46: f64 = (p.p6 * s.db[379][46]);
        let eq51_e920_d_b47: f64 = (p.p6 * s.db[379][47]);
        let eq51_e920_d_b48: f64 = (p.p6 * s.db[379][48]);
        let eq51_e920_d_b49: f64 = (p.p6 * s.db[379][49]);
        let eq51_e920_d_b50: f64 = (p.p6 * s.db[379][50]);
        let eq51_e920_d_b51: f64 = (p.p6 * s.db[379][51]);
        let eq51_e920_d_b52: f64 = (p.p6 * s.db[379][52]);
        let eq51_e920_d_b53: f64 = (p.p6 * s.db[379][53]);
        let eq51_e920_d_b54: f64 = (p.p6 * s.db[379][54]);
        let eq51_e922: f64 = (eq51_e920 * (nv7 - nv8));
        let eq51_e922_d_n0: f64 = (eq51_e920_d_n0 * (nv7 - nv8));
        let eq51_e922_d_n1: f64 = (eq51_e920_d_n1 * (nv7 - nv8));
        let eq51_e922_d_n2: f64 = (eq51_e920_d_n2 * (nv7 - nv8));
        let eq51_e922_d_n3: f64 = (eq51_e920_d_n3 * (nv7 - nv8));
        let eq51_e922_d_n4: f64 = (eq51_e920_d_n4 * (nv7 - nv8));
        let eq51_e922_d_n5: f64 = (eq51_e920_d_n5 * (nv7 - nv8));
        let eq51_e922_d_n6: f64 = (eq51_e920_d_n6 * (nv7 - nv8));
        let eq51_e922_d_n7: f64 = ((eq51_e920_d_n7 * (nv7 - nv8)) + eq51_e920);
        let eq51_e922_d_n8: f64 = ((eq51_e920_d_n8 * (nv7 - nv8)) + (-eq51_e920));
        let eq51_e922_d_n9: f64 = (eq51_e920_d_n9 * (nv7 - nv8));
        let eq51_e922_d_n10: f64 = (eq51_e920_d_n10 * (nv7 - nv8));
        let eq51_e922_d_n11: f64 = (eq51_e920_d_n11 * (nv7 - nv8));
        let eq51_e922_d_n12: f64 = (eq51_e920_d_n12 * (nv7 - nv8));
        let eq51_e922_d_n13: f64 = (eq51_e920_d_n13 * (nv7 - nv8));
        let eq51_e922_d_n14: f64 = (eq51_e920_d_n14 * (nv7 - nv8));
        let eq51_e922_d_n15: f64 = (eq51_e920_d_n15 * (nv7 - nv8));
        let eq51_e922_d_n16: f64 = (eq51_e920_d_n16 * (nv7 - nv8));
        let eq51_e922_d_n17: f64 = (eq51_e920_d_n17 * (nv7 - nv8));
        let eq51_e922_d_n18: f64 = (eq51_e920_d_n18 * (nv7 - nv8));
        let eq51_e922_d_n19: f64 = (eq51_e920_d_n19 * (nv7 - nv8));
        let eq51_e922_d_n20: f64 = (eq51_e920_d_n20 * (nv7 - nv8));
        let eq51_e922_d_n21: f64 = (eq51_e920_d_n21 * (nv7 - nv8));
        let eq51_e922_d_n22: f64 = (eq51_e920_d_n22 * (nv7 - nv8));
        let eq51_e922_d_b0: f64 = (eq51_e920_d_b0 * (nv7 - nv8));
        let eq51_e922_d_b1: f64 = (eq51_e920_d_b1 * (nv7 - nv8));
        let eq51_e922_d_b2: f64 = (eq51_e920_d_b2 * (nv7 - nv8));
        let eq51_e922_d_b3: f64 = (eq51_e920_d_b3 * (nv7 - nv8));
        let eq51_e922_d_b4: f64 = (eq51_e920_d_b4 * (nv7 - nv8));
        let eq51_e922_d_b5: f64 = (eq51_e920_d_b5 * (nv7 - nv8));
        let eq51_e922_d_b6: f64 = (eq51_e920_d_b6 * (nv7 - nv8));
        let eq51_e922_d_b7: f64 = (eq51_e920_d_b7 * (nv7 - nv8));
        let eq51_e922_d_b8: f64 = (eq51_e920_d_b8 * (nv7 - nv8));
        let eq51_e922_d_b9: f64 = (eq51_e920_d_b9 * (nv7 - nv8));
        let eq51_e922_d_b10: f64 = (eq51_e920_d_b10 * (nv7 - nv8));
        let eq51_e922_d_b11: f64 = (eq51_e920_d_b11 * (nv7 - nv8));
        let eq51_e922_d_b12: f64 = (eq51_e920_d_b12 * (nv7 - nv8));
        let eq51_e922_d_b13: f64 = (eq51_e920_d_b13 * (nv7 - nv8));
        let eq51_e922_d_b14: f64 = (eq51_e920_d_b14 * (nv7 - nv8));
        let eq51_e922_d_b15: f64 = (eq51_e920_d_b15 * (nv7 - nv8));
        let eq51_e922_d_b16: f64 = (eq51_e920_d_b16 * (nv7 - nv8));
        let eq51_e922_d_b17: f64 = (eq51_e920_d_b17 * (nv7 - nv8));
        let eq51_e922_d_b18: f64 = (eq51_e920_d_b18 * (nv7 - nv8));
        let eq51_e922_d_b19: f64 = (eq51_e920_d_b19 * (nv7 - nv8));
        let eq51_e922_d_b20: f64 = (eq51_e920_d_b20 * (nv7 - nv8));
        let eq51_e922_d_b21: f64 = (eq51_e920_d_b21 * (nv7 - nv8));
        let eq51_e922_d_b22: f64 = (eq51_e920_d_b22 * (nv7 - nv8));
        let eq51_e922_d_b23: f64 = (eq51_e920_d_b23 * (nv7 - nv8));
        let eq51_e922_d_b24: f64 = (eq51_e920_d_b24 * (nv7 - nv8));
        let eq51_e922_d_b25: f64 = (eq51_e920_d_b25 * (nv7 - nv8));
        let eq51_e922_d_b26: f64 = (eq51_e920_d_b26 * (nv7 - nv8));
        let eq51_e922_d_b27: f64 = (eq51_e920_d_b27 * (nv7 - nv8));
        let eq51_e922_d_b28: f64 = (eq51_e920_d_b28 * (nv7 - nv8));
        let eq51_e922_d_b29: f64 = (eq51_e920_d_b29 * (nv7 - nv8));
        let eq51_e922_d_b30: f64 = (eq51_e920_d_b30 * (nv7 - nv8));
        let eq51_e922_d_b31: f64 = (eq51_e920_d_b31 * (nv7 - nv8));
        let eq51_e922_d_b32: f64 = (eq51_e920_d_b32 * (nv7 - nv8));
        let eq51_e922_d_b33: f64 = (eq51_e920_d_b33 * (nv7 - nv8));
        let eq51_e922_d_b34: f64 = (eq51_e920_d_b34 * (nv7 - nv8));
        let eq51_e922_d_b35: f64 = (eq51_e920_d_b35 * (nv7 - nv8));
        let eq51_e922_d_b36: f64 = (eq51_e920_d_b36 * (nv7 - nv8));
        let eq51_e922_d_b37: f64 = (eq51_e920_d_b37 * (nv7 - nv8));
        let eq51_e922_d_b38: f64 = (eq51_e920_d_b38 * (nv7 - nv8));
        let eq51_e922_d_b39: f64 = (eq51_e920_d_b39 * (nv7 - nv8));
        let eq51_e922_d_b40: f64 = (eq51_e920_d_b40 * (nv7 - nv8));
        let eq51_e922_d_b41: f64 = (eq51_e920_d_b41 * (nv7 - nv8));
        let eq51_e922_d_b42: f64 = (eq51_e920_d_b42 * (nv7 - nv8));
        let eq51_e922_d_b43: f64 = (eq51_e920_d_b43 * (nv7 - nv8));
        let eq51_e922_d_b44: f64 = (eq51_e920_d_b44 * (nv7 - nv8));
        let eq51_e922_d_b45: f64 = (eq51_e920_d_b45 * (nv7 - nv8));
        let eq51_e922_d_b46: f64 = (eq51_e920_d_b46 * (nv7 - nv8));
        let eq51_e922_d_b47: f64 = (eq51_e920_d_b47 * (nv7 - nv8));
        let eq51_e922_d_b48: f64 = (eq51_e920_d_b48 * (nv7 - nv8));
        let eq51_e922_d_b49: f64 = (eq51_e920_d_b49 * (nv7 - nv8));
        let eq51_e922_d_b50: f64 = (eq51_e920_d_b50 * (nv7 - nv8));
        let eq51_e922_d_b51: f64 = (eq51_e920_d_b51 * (nv7 - nv8));
        let eq51_e922_d_b52: f64 = (eq51_e920_d_b52 * (nv7 - nv8));
        let eq51_e922_d_b53: f64 = (eq51_e920_d_b53 * (nv7 - nv8));
        let eq51_e922_d_b54: f64 = (eq51_e920_d_b54 * (nv7 - nv8));
        let eq51_e923: f64 = (eq51_e917 + eq51_e922);
        let eq51_e923_d_n0: f64 = (eq51_e917_d_n0 + eq51_e922_d_n0);
        let eq51_e923_d_n1: f64 = (eq51_e917_d_n1 + eq51_e922_d_n1);
        let eq51_e923_d_n2: f64 = (eq51_e917_d_n2 + eq51_e922_d_n2);
        let eq51_e923_d_n3: f64 = (eq51_e917_d_n3 + eq51_e922_d_n3);
        let eq51_e923_d_n4: f64 = (eq51_e917_d_n4 + eq51_e922_d_n4);
        let eq51_e923_d_n5: f64 = (eq51_e917_d_n5 + eq51_e922_d_n5);
        let eq51_e923_d_n6: f64 = (eq51_e917_d_n6 + eq51_e922_d_n6);
        let eq51_e923_d_n7: f64 = (eq51_e917_d_n7 + eq51_e922_d_n7);
        let eq51_e923_d_n8: f64 = (eq51_e917_d_n8 + eq51_e922_d_n8);
        let eq51_e923_d_n9: f64 = (eq51_e917_d_n9 + eq51_e922_d_n9);
        let eq51_e923_d_n10: f64 = (eq51_e917_d_n10 + eq51_e922_d_n10);
        let eq51_e923_d_n11: f64 = (eq51_e917_d_n11 + eq51_e922_d_n11);
        let eq51_e923_d_n12: f64 = (eq51_e917_d_n12 + eq51_e922_d_n12);
        let eq51_e923_d_n13: f64 = (eq51_e917_d_n13 + eq51_e922_d_n13);
        let eq51_e923_d_n14: f64 = (eq51_e917_d_n14 + eq51_e922_d_n14);
        let eq51_e923_d_n15: f64 = (eq51_e917_d_n15 + eq51_e922_d_n15);
        let eq51_e923_d_n16: f64 = (eq51_e917_d_n16 + eq51_e922_d_n16);
        let eq51_e923_d_n17: f64 = (eq51_e917_d_n17 + eq51_e922_d_n17);
        let eq51_e923_d_n18: f64 = (eq51_e917_d_n18 + eq51_e922_d_n18);
        let eq51_e923_d_n19: f64 = (eq51_e917_d_n19 + eq51_e922_d_n19);
        let eq51_e923_d_n20: f64 = (eq51_e917_d_n20 + eq51_e922_d_n20);
        let eq51_e923_d_n21: f64 = (eq51_e917_d_n21 + eq51_e922_d_n21);
        let eq51_e923_d_n22: f64 = (eq51_e917_d_n22 + eq51_e922_d_n22);
        let eq51_e923_d_b0: f64 = (eq51_e917_d_b0 + eq51_e922_d_b0);
        let eq51_e923_d_b1: f64 = (eq51_e917_d_b1 + eq51_e922_d_b1);
        let eq51_e923_d_b2: f64 = (eq51_e917_d_b2 + eq51_e922_d_b2);
        let eq51_e923_d_b3: f64 = (eq51_e917_d_b3 + eq51_e922_d_b3);
        let eq51_e923_d_b4: f64 = (eq51_e917_d_b4 + eq51_e922_d_b4);
        let eq51_e923_d_b5: f64 = (eq51_e917_d_b5 + eq51_e922_d_b5);
        let eq51_e923_d_b6: f64 = (eq51_e917_d_b6 + eq51_e922_d_b6);
        let eq51_e923_d_b7: f64 = (eq51_e917_d_b7 + eq51_e922_d_b7);
        let eq51_e923_d_b8: f64 = (eq51_e917_d_b8 + eq51_e922_d_b8);
        let eq51_e923_d_b9: f64 = (eq51_e917_d_b9 + eq51_e922_d_b9);
        let eq51_e923_d_b10: f64 = (eq51_e917_d_b10 + eq51_e922_d_b10);
        let eq51_e923_d_b11: f64 = (eq51_e917_d_b11 + eq51_e922_d_b11);
        let eq51_e923_d_b12: f64 = (eq51_e917_d_b12 + eq51_e922_d_b12);
        let eq51_e923_d_b13: f64 = (eq51_e917_d_b13 + eq51_e922_d_b13);
        let eq51_e923_d_b14: f64 = (eq51_e917_d_b14 + eq51_e922_d_b14);
        let eq51_e923_d_b15: f64 = (eq51_e917_d_b15 + eq51_e922_d_b15);
        let eq51_e923_d_b16: f64 = (eq51_e917_d_b16 + eq51_e922_d_b16);
        let eq51_e923_d_b17: f64 = (eq51_e917_d_b17 + eq51_e922_d_b17);
        let eq51_e923_d_b18: f64 = (eq51_e917_d_b18 + eq51_e922_d_b18);
        let eq51_e923_d_b19: f64 = (eq51_e917_d_b19 + eq51_e922_d_b19);
        let eq51_e923_d_b20: f64 = (eq51_e917_d_b20 + eq51_e922_d_b20);
        let eq51_e923_d_b21: f64 = (eq51_e917_d_b21 + eq51_e922_d_b21);
        let eq51_e923_d_b22: f64 = (eq51_e917_d_b22 + eq51_e922_d_b22);
        let eq51_e923_d_b23: f64 = (eq51_e917_d_b23 + eq51_e922_d_b23);
        let eq51_e923_d_b24: f64 = (eq51_e917_d_b24 + eq51_e922_d_b24);
        let eq51_e923_d_b25: f64 = (eq51_e917_d_b25 + eq51_e922_d_b25);
        let eq51_e923_d_b26: f64 = (eq51_e917_d_b26 + eq51_e922_d_b26);
        let eq51_e923_d_b27: f64 = (eq51_e917_d_b27 + eq51_e922_d_b27);
        let eq51_e923_d_b28: f64 = (eq51_e917_d_b28 + eq51_e922_d_b28);
        let eq51_e923_d_b29: f64 = (eq51_e917_d_b29 + eq51_e922_d_b29);
        let eq51_e923_d_b30: f64 = (eq51_e917_d_b30 + eq51_e922_d_b30);
        let eq51_e923_d_b31: f64 = (eq51_e917_d_b31 + eq51_e922_d_b31);
        let eq51_e923_d_b32: f64 = (eq51_e917_d_b32 + eq51_e922_d_b32);
        let eq51_e923_d_b33: f64 = (eq51_e917_d_b33 + eq51_e922_d_b33);
        let eq51_e923_d_b34: f64 = (eq51_e917_d_b34 + eq51_e922_d_b34);
        let eq51_e923_d_b35: f64 = (eq51_e917_d_b35 + eq51_e922_d_b35);
        let eq51_e923_d_b36: f64 = (eq51_e917_d_b36 + eq51_e922_d_b36);
        let eq51_e923_d_b37: f64 = (eq51_e917_d_b37 + eq51_e922_d_b37);
        let eq51_e923_d_b38: f64 = (eq51_e917_d_b38 + eq51_e922_d_b38);
        let eq51_e923_d_b39: f64 = (eq51_e917_d_b39 + eq51_e922_d_b39);
        let eq51_e923_d_b40: f64 = (eq51_e917_d_b40 + eq51_e922_d_b40);
        let eq51_e923_d_b41: f64 = (eq51_e917_d_b41 + eq51_e922_d_b41);
        let eq51_e923_d_b42: f64 = (eq51_e917_d_b42 + eq51_e922_d_b42);
        let eq51_e923_d_b43: f64 = (eq51_e917_d_b43 + eq51_e922_d_b43);
        let eq51_e923_d_b44: f64 = (eq51_e917_d_b44 + eq51_e922_d_b44);
        let eq51_e923_d_b45: f64 = (eq51_e917_d_b45 + eq51_e922_d_b45);
        let eq51_e923_d_b46: f64 = (eq51_e917_d_b46 + eq51_e922_d_b46);
        let eq51_e923_d_b47: f64 = (eq51_e917_d_b47 + eq51_e922_d_b47);
        let eq51_e923_d_b48: f64 = (eq51_e917_d_b48 + eq51_e922_d_b48);
        let eq51_e923_d_b49: f64 = (eq51_e917_d_b49 + eq51_e922_d_b49);
        let eq51_e923_d_b50: f64 = (eq51_e917_d_b50 + eq51_e922_d_b50);
        let eq51_e923_d_b51: f64 = (eq51_e917_d_b51 + eq51_e922_d_b51);
        let eq51_e923_d_b52: f64 = (eq51_e917_d_b52 + eq51_e922_d_b52);
        let eq51_e923_d_b53: f64 = (eq51_e917_d_b53 + eq51_e922_d_b53);
        let eq51_e923_d_b54: f64 = (eq51_e917_d_b54 + eq51_e922_d_b54);
        let eq51_value: f64 = eq51_e923;
        let eq51_node_derivatives: [f64; 23] = [eq51_e923_d_n0, eq51_e923_d_n1, eq51_e923_d_n2, eq51_e923_d_n3, eq51_e923_d_n4, eq51_e923_d_n5, eq51_e923_d_n6, eq51_e923_d_n7, eq51_e923_d_n8, eq51_e923_d_n9, eq51_e923_d_n10, eq51_e923_d_n11, eq51_e923_d_n12, eq51_e923_d_n13, eq51_e923_d_n14, eq51_e923_d_n15, eq51_e923_d_n16, eq51_e923_d_n17, eq51_e923_d_n18, eq51_e923_d_n19, eq51_e923_d_n20, eq51_e923_d_n21, eq51_e923_d_n22];
        let eq51_branch_derivatives: [f64; 55] = [eq51_e923_d_b0, eq51_e923_d_b1, eq51_e923_d_b2, eq51_e923_d_b3, eq51_e923_d_b4, eq51_e923_d_b5, eq51_e923_d_b6, eq51_e923_d_b7, eq51_e923_d_b8, eq51_e923_d_b9, eq51_e923_d_b10, eq51_e923_d_b11, eq51_e923_d_b12, eq51_e923_d_b13, eq51_e923_d_b14, eq51_e923_d_b15, eq51_e923_d_b16, eq51_e923_d_b17, eq51_e923_d_b18, eq51_e923_d_b19, eq51_e923_d_b20, eq51_e923_d_b21, eq51_e923_d_b22, eq51_e923_d_b23, eq51_e923_d_b24, eq51_e923_d_b25, eq51_e923_d_b26, eq51_e923_d_b27, eq51_e923_d_b28, eq51_e923_d_b29, eq51_e923_d_b30, eq51_e923_d_b31, eq51_e923_d_b32, eq51_e923_d_b33, eq51_e923_d_b34, eq51_e923_d_b35, eq51_e923_d_b36, eq51_e923_d_b37, eq51_e923_d_b38, eq51_e923_d_b39, eq51_e923_d_b40, eq51_e923_d_b41, eq51_e923_d_b42, eq51_e923_d_b43, eq51_e923_d_b44, eq51_e923_d_b45, eq51_e923_d_b46, eq51_e923_d_b47, eq51_e923_d_b48, eq51_e923_d_b49, eq51_e923_d_b50, eq51_e923_d_b51, eq51_e923_d_b52, eq51_e923_d_b53, eq51_e923_d_b54];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq51_value),
            &eq51_node_derivatives,
            &eq51_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_6(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq52_e926: f64 = (p.p6 * s.v[41]);
        let eq52_e926_d_n0: f64 = (p.p6 * s.dn[41][0]);
        let eq52_e926_d_n1: f64 = (p.p6 * s.dn[41][1]);
        let eq52_e926_d_n2: f64 = (p.p6 * s.dn[41][2]);
        let eq52_e926_d_n3: f64 = (p.p6 * s.dn[41][3]);
        let eq52_e926_d_n4: f64 = (p.p6 * s.dn[41][4]);
        let eq52_e926_d_n5: f64 = (p.p6 * s.dn[41][5]);
        let eq52_e926_d_n6: f64 = (p.p6 * s.dn[41][6]);
        let eq52_e926_d_n7: f64 = (p.p6 * s.dn[41][7]);
        let eq52_e926_d_n8: f64 = (p.p6 * s.dn[41][8]);
        let eq52_e926_d_n9: f64 = (p.p6 * s.dn[41][9]);
        let eq52_e926_d_n10: f64 = (p.p6 * s.dn[41][10]);
        let eq52_e926_d_n11: f64 = (p.p6 * s.dn[41][11]);
        let eq52_e926_d_n12: f64 = (p.p6 * s.dn[41][12]);
        let eq52_e926_d_n13: f64 = (p.p6 * s.dn[41][13]);
        let eq52_e926_d_n14: f64 = (p.p6 * s.dn[41][14]);
        let eq52_e926_d_n15: f64 = (p.p6 * s.dn[41][15]);
        let eq52_e926_d_n16: f64 = (p.p6 * s.dn[41][16]);
        let eq52_e926_d_n17: f64 = (p.p6 * s.dn[41][17]);
        let eq52_e926_d_n18: f64 = (p.p6 * s.dn[41][18]);
        let eq52_e926_d_n19: f64 = (p.p6 * s.dn[41][19]);
        let eq52_e926_d_n20: f64 = (p.p6 * s.dn[41][20]);
        let eq52_e926_d_n21: f64 = (p.p6 * s.dn[41][21]);
        let eq52_e926_d_n22: f64 = (p.p6 * s.dn[41][22]);
        let eq52_e926_d_b0: f64 = (p.p6 * s.db[41][0]);
        let eq52_e926_d_b1: f64 = (p.p6 * s.db[41][1]);
        let eq52_e926_d_b2: f64 = (p.p6 * s.db[41][2]);
        let eq52_e926_d_b3: f64 = (p.p6 * s.db[41][3]);
        let eq52_e926_d_b4: f64 = (p.p6 * s.db[41][4]);
        let eq52_e926_d_b5: f64 = (p.p6 * s.db[41][5]);
        let eq52_e926_d_b6: f64 = (p.p6 * s.db[41][6]);
        let eq52_e926_d_b7: f64 = (p.p6 * s.db[41][7]);
        let eq52_e926_d_b8: f64 = (p.p6 * s.db[41][8]);
        let eq52_e926_d_b9: f64 = (p.p6 * s.db[41][9]);
        let eq52_e926_d_b10: f64 = (p.p6 * s.db[41][10]);
        let eq52_e926_d_b11: f64 = (p.p6 * s.db[41][11]);
        let eq52_e926_d_b12: f64 = (p.p6 * s.db[41][12]);
        let eq52_e926_d_b13: f64 = (p.p6 * s.db[41][13]);
        let eq52_e926_d_b14: f64 = (p.p6 * s.db[41][14]);
        let eq52_e926_d_b15: f64 = (p.p6 * s.db[41][15]);
        let eq52_e926_d_b16: f64 = (p.p6 * s.db[41][16]);
        let eq52_e926_d_b17: f64 = (p.p6 * s.db[41][17]);
        let eq52_e926_d_b18: f64 = (p.p6 * s.db[41][18]);
        let eq52_e926_d_b19: f64 = (p.p6 * s.db[41][19]);
        let eq52_e926_d_b20: f64 = (p.p6 * s.db[41][20]);
        let eq52_e926_d_b21: f64 = (p.p6 * s.db[41][21]);
        let eq52_e926_d_b22: f64 = (p.p6 * s.db[41][22]);
        let eq52_e926_d_b23: f64 = (p.p6 * s.db[41][23]);
        let eq52_e926_d_b24: f64 = (p.p6 * s.db[41][24]);
        let eq52_e926_d_b25: f64 = (p.p6 * s.db[41][25]);
        let eq52_e926_d_b26: f64 = (p.p6 * s.db[41][26]);
        let eq52_e926_d_b27: f64 = (p.p6 * s.db[41][27]);
        let eq52_e926_d_b28: f64 = (p.p6 * s.db[41][28]);
        let eq52_e926_d_b29: f64 = (p.p6 * s.db[41][29]);
        let eq52_e926_d_b30: f64 = (p.p6 * s.db[41][30]);
        let eq52_e926_d_b31: f64 = (p.p6 * s.db[41][31]);
        let eq52_e926_d_b32: f64 = (p.p6 * s.db[41][32]);
        let eq52_e926_d_b33: f64 = (p.p6 * s.db[41][33]);
        let eq52_e926_d_b34: f64 = (p.p6 * s.db[41][34]);
        let eq52_e926_d_b35: f64 = (p.p6 * s.db[41][35]);
        let eq52_e926_d_b36: f64 = (p.p6 * s.db[41][36]);
        let eq52_e926_d_b37: f64 = (p.p6 * s.db[41][37]);
        let eq52_e926_d_b38: f64 = (p.p6 * s.db[41][38]);
        let eq52_e926_d_b39: f64 = (p.p6 * s.db[41][39]);
        let eq52_e926_d_b40: f64 = (p.p6 * s.db[41][40]);
        let eq52_e926_d_b41: f64 = (p.p6 * s.db[41][41]);
        let eq52_e926_d_b42: f64 = (p.p6 * s.db[41][42]);
        let eq52_e926_d_b43: f64 = (p.p6 * s.db[41][43]);
        let eq52_e926_d_b44: f64 = (p.p6 * s.db[41][44]);
        let eq52_e926_d_b45: f64 = (p.p6 * s.db[41][45]);
        let eq52_e926_d_b46: f64 = (p.p6 * s.db[41][46]);
        let eq52_e926_d_b47: f64 = (p.p6 * s.db[41][47]);
        let eq52_e926_d_b48: f64 = (p.p6 * s.db[41][48]);
        let eq52_e926_d_b49: f64 = (p.p6 * s.db[41][49]);
        let eq52_e926_d_b50: f64 = (p.p6 * s.db[41][50]);
        let eq52_e926_d_b51: f64 = (p.p6 * s.db[41][51]);
        let eq52_e926_d_b52: f64 = (p.p6 * s.db[41][52]);
        let eq52_e926_d_b53: f64 = (p.p6 * s.db[41][53]);
        let eq52_e926_d_b54: f64 = (p.p6 * s.db[41][54]);
        let eq52_e929: f64 = (p.p4 * p.p5);
        let eq52_e931: f64 = (eq52_e929 * s.v[332]);
        let eq52_e931_d_n0: f64 = (eq52_e929 * s.dn[332][0]);
        let eq52_e931_d_n1: f64 = (eq52_e929 * s.dn[332][1]);
        let eq52_e931_d_n2: f64 = (eq52_e929 * s.dn[332][2]);
        let eq52_e931_d_n3: f64 = (eq52_e929 * s.dn[332][3]);
        let eq52_e931_d_n4: f64 = (eq52_e929 * s.dn[332][4]);
        let eq52_e931_d_n5: f64 = (eq52_e929 * s.dn[332][5]);
        let eq52_e931_d_n6: f64 = (eq52_e929 * s.dn[332][6]);
        let eq52_e931_d_n7: f64 = (eq52_e929 * s.dn[332][7]);
        let eq52_e931_d_n8: f64 = (eq52_e929 * s.dn[332][8]);
        let eq52_e931_d_n9: f64 = (eq52_e929 * s.dn[332][9]);
        let eq52_e931_d_n10: f64 = (eq52_e929 * s.dn[332][10]);
        let eq52_e931_d_n11: f64 = (eq52_e929 * s.dn[332][11]);
        let eq52_e931_d_n12: f64 = (eq52_e929 * s.dn[332][12]);
        let eq52_e931_d_n13: f64 = (eq52_e929 * s.dn[332][13]);
        let eq52_e931_d_n14: f64 = (eq52_e929 * s.dn[332][14]);
        let eq52_e931_d_n15: f64 = (eq52_e929 * s.dn[332][15]);
        let eq52_e931_d_n16: f64 = (eq52_e929 * s.dn[332][16]);
        let eq52_e931_d_n17: f64 = (eq52_e929 * s.dn[332][17]);
        let eq52_e931_d_n18: f64 = (eq52_e929 * s.dn[332][18]);
        let eq52_e931_d_n19: f64 = (eq52_e929 * s.dn[332][19]);
        let eq52_e931_d_n20: f64 = (eq52_e929 * s.dn[332][20]);
        let eq52_e931_d_n21: f64 = (eq52_e929 * s.dn[332][21]);
        let eq52_e931_d_n22: f64 = (eq52_e929 * s.dn[332][22]);
        let eq52_e931_d_b0: f64 = (eq52_e929 * s.db[332][0]);
        let eq52_e931_d_b1: f64 = (eq52_e929 * s.db[332][1]);
        let eq52_e931_d_b2: f64 = (eq52_e929 * s.db[332][2]);
        let eq52_e931_d_b3: f64 = (eq52_e929 * s.db[332][3]);
        let eq52_e931_d_b4: f64 = (eq52_e929 * s.db[332][4]);
        let eq52_e931_d_b5: f64 = (eq52_e929 * s.db[332][5]);
        let eq52_e931_d_b6: f64 = (eq52_e929 * s.db[332][6]);
        let eq52_e931_d_b7: f64 = (eq52_e929 * s.db[332][7]);
        let eq52_e931_d_b8: f64 = (eq52_e929 * s.db[332][8]);
        let eq52_e931_d_b9: f64 = (eq52_e929 * s.db[332][9]);
        let eq52_e931_d_b10: f64 = (eq52_e929 * s.db[332][10]);
        let eq52_e931_d_b11: f64 = (eq52_e929 * s.db[332][11]);
        let eq52_e931_d_b12: f64 = (eq52_e929 * s.db[332][12]);
        let eq52_e931_d_b13: f64 = (eq52_e929 * s.db[332][13]);
        let eq52_e931_d_b14: f64 = (eq52_e929 * s.db[332][14]);
        let eq52_e931_d_b15: f64 = (eq52_e929 * s.db[332][15]);
        let eq52_e931_d_b16: f64 = (eq52_e929 * s.db[332][16]);
        let eq52_e931_d_b17: f64 = (eq52_e929 * s.db[332][17]);
        let eq52_e931_d_b18: f64 = (eq52_e929 * s.db[332][18]);
        let eq52_e931_d_b19: f64 = (eq52_e929 * s.db[332][19]);
        let eq52_e931_d_b20: f64 = (eq52_e929 * s.db[332][20]);
        let eq52_e931_d_b21: f64 = (eq52_e929 * s.db[332][21]);
        let eq52_e931_d_b22: f64 = (eq52_e929 * s.db[332][22]);
        let eq52_e931_d_b23: f64 = (eq52_e929 * s.db[332][23]);
        let eq52_e931_d_b24: f64 = (eq52_e929 * s.db[332][24]);
        let eq52_e931_d_b25: f64 = (eq52_e929 * s.db[332][25]);
        let eq52_e931_d_b26: f64 = (eq52_e929 * s.db[332][26]);
        let eq52_e931_d_b27: f64 = (eq52_e929 * s.db[332][27]);
        let eq52_e931_d_b28: f64 = (eq52_e929 * s.db[332][28]);
        let eq52_e931_d_b29: f64 = (eq52_e929 * s.db[332][29]);
        let eq52_e931_d_b30: f64 = (eq52_e929 * s.db[332][30]);
        let eq52_e931_d_b31: f64 = (eq52_e929 * s.db[332][31]);
        let eq52_e931_d_b32: f64 = (eq52_e929 * s.db[332][32]);
        let eq52_e931_d_b33: f64 = (eq52_e929 * s.db[332][33]);
        let eq52_e931_d_b34: f64 = (eq52_e929 * s.db[332][34]);
        let eq52_e931_d_b35: f64 = (eq52_e929 * s.db[332][35]);
        let eq52_e931_d_b36: f64 = (eq52_e929 * s.db[332][36]);
        let eq52_e931_d_b37: f64 = (eq52_e929 * s.db[332][37]);
        let eq52_e931_d_b38: f64 = (eq52_e929 * s.db[332][38]);
        let eq52_e931_d_b39: f64 = (eq52_e929 * s.db[332][39]);
        let eq52_e931_d_b40: f64 = (eq52_e929 * s.db[332][40]);
        let eq52_e931_d_b41: f64 = (eq52_e929 * s.db[332][41]);
        let eq52_e931_d_b42: f64 = (eq52_e929 * s.db[332][42]);
        let eq52_e931_d_b43: f64 = (eq52_e929 * s.db[332][43]);
        let eq52_e931_d_b44: f64 = (eq52_e929 * s.db[332][44]);
        let eq52_e931_d_b45: f64 = (eq52_e929 * s.db[332][45]);
        let eq52_e931_d_b46: f64 = (eq52_e929 * s.db[332][46]);
        let eq52_e931_d_b47: f64 = (eq52_e929 * s.db[332][47]);
        let eq52_e931_d_b48: f64 = (eq52_e929 * s.db[332][48]);
        let eq52_e931_d_b49: f64 = (eq52_e929 * s.db[332][49]);
        let eq52_e931_d_b50: f64 = (eq52_e929 * s.db[332][50]);
        let eq52_e931_d_b51: f64 = (eq52_e929 * s.db[332][51]);
        let eq52_e931_d_b52: f64 = (eq52_e929 * s.db[332][52]);
        let eq52_e931_d_b53: f64 = (eq52_e929 * s.db[332][53]);
        let eq52_e931_d_b54: f64 = (eq52_e929 * s.db[332][54]);
        let eq52_e932: f64 = (eq52_e926 * eq52_e931);
        let eq52_e932_d_n0: f64 = ((eq52_e926_d_n0 * eq52_e931) + (eq52_e926 * eq52_e931_d_n0));
        let eq52_e932_d_n1: f64 = ((eq52_e926_d_n1 * eq52_e931) + (eq52_e926 * eq52_e931_d_n1));
        let eq52_e932_d_n2: f64 = ((eq52_e926_d_n2 * eq52_e931) + (eq52_e926 * eq52_e931_d_n2));
        let eq52_e932_d_n3: f64 = ((eq52_e926_d_n3 * eq52_e931) + (eq52_e926 * eq52_e931_d_n3));
        let eq52_e932_d_n4: f64 = ((eq52_e926_d_n4 * eq52_e931) + (eq52_e926 * eq52_e931_d_n4));
        let eq52_e932_d_n5: f64 = ((eq52_e926_d_n5 * eq52_e931) + (eq52_e926 * eq52_e931_d_n5));
        let eq52_e932_d_n6: f64 = ((eq52_e926_d_n6 * eq52_e931) + (eq52_e926 * eq52_e931_d_n6));
        let eq52_e932_d_n7: f64 = ((eq52_e926_d_n7 * eq52_e931) + (eq52_e926 * eq52_e931_d_n7));
        let eq52_e932_d_n8: f64 = ((eq52_e926_d_n8 * eq52_e931) + (eq52_e926 * eq52_e931_d_n8));
        let eq52_e932_d_n9: f64 = ((eq52_e926_d_n9 * eq52_e931) + (eq52_e926 * eq52_e931_d_n9));
        let eq52_e932_d_n10: f64 = ((eq52_e926_d_n10 * eq52_e931) + (eq52_e926 * eq52_e931_d_n10));
        let eq52_e932_d_n11: f64 = ((eq52_e926_d_n11 * eq52_e931) + (eq52_e926 * eq52_e931_d_n11));
        let eq52_e932_d_n12: f64 = ((eq52_e926_d_n12 * eq52_e931) + (eq52_e926 * eq52_e931_d_n12));
        let eq52_e932_d_n13: f64 = ((eq52_e926_d_n13 * eq52_e931) + (eq52_e926 * eq52_e931_d_n13));
        let eq52_e932_d_n14: f64 = ((eq52_e926_d_n14 * eq52_e931) + (eq52_e926 * eq52_e931_d_n14));
        let eq52_e932_d_n15: f64 = ((eq52_e926_d_n15 * eq52_e931) + (eq52_e926 * eq52_e931_d_n15));
        let eq52_e932_d_n16: f64 = ((eq52_e926_d_n16 * eq52_e931) + (eq52_e926 * eq52_e931_d_n16));
        let eq52_e932_d_n17: f64 = ((eq52_e926_d_n17 * eq52_e931) + (eq52_e926 * eq52_e931_d_n17));
        let eq52_e932_d_n18: f64 = ((eq52_e926_d_n18 * eq52_e931) + (eq52_e926 * eq52_e931_d_n18));
        let eq52_e932_d_n19: f64 = ((eq52_e926_d_n19 * eq52_e931) + (eq52_e926 * eq52_e931_d_n19));
        let eq52_e932_d_n20: f64 = ((eq52_e926_d_n20 * eq52_e931) + (eq52_e926 * eq52_e931_d_n20));
        let eq52_e932_d_n21: f64 = ((eq52_e926_d_n21 * eq52_e931) + (eq52_e926 * eq52_e931_d_n21));
        let eq52_e932_d_n22: f64 = ((eq52_e926_d_n22 * eq52_e931) + (eq52_e926 * eq52_e931_d_n22));
        let eq52_e932_d_b0: f64 = ((eq52_e926_d_b0 * eq52_e931) + (eq52_e926 * eq52_e931_d_b0));
        let eq52_e932_d_b1: f64 = ((eq52_e926_d_b1 * eq52_e931) + (eq52_e926 * eq52_e931_d_b1));
        let eq52_e932_d_b2: f64 = ((eq52_e926_d_b2 * eq52_e931) + (eq52_e926 * eq52_e931_d_b2));
        let eq52_e932_d_b3: f64 = ((eq52_e926_d_b3 * eq52_e931) + (eq52_e926 * eq52_e931_d_b3));
        let eq52_e932_d_b4: f64 = ((eq52_e926_d_b4 * eq52_e931) + (eq52_e926 * eq52_e931_d_b4));
        let eq52_e932_d_b5: f64 = ((eq52_e926_d_b5 * eq52_e931) + (eq52_e926 * eq52_e931_d_b5));
        let eq52_e932_d_b6: f64 = ((eq52_e926_d_b6 * eq52_e931) + (eq52_e926 * eq52_e931_d_b6));
        let eq52_e932_d_b7: f64 = ((eq52_e926_d_b7 * eq52_e931) + (eq52_e926 * eq52_e931_d_b7));
        let eq52_e932_d_b8: f64 = ((eq52_e926_d_b8 * eq52_e931) + (eq52_e926 * eq52_e931_d_b8));
        let eq52_e932_d_b9: f64 = ((eq52_e926_d_b9 * eq52_e931) + (eq52_e926 * eq52_e931_d_b9));
        let eq52_e932_d_b10: f64 = ((eq52_e926_d_b10 * eq52_e931) + (eq52_e926 * eq52_e931_d_b10));
        let eq52_e932_d_b11: f64 = ((eq52_e926_d_b11 * eq52_e931) + (eq52_e926 * eq52_e931_d_b11));
        let eq52_e932_d_b12: f64 = ((eq52_e926_d_b12 * eq52_e931) + (eq52_e926 * eq52_e931_d_b12));
        let eq52_e932_d_b13: f64 = ((eq52_e926_d_b13 * eq52_e931) + (eq52_e926 * eq52_e931_d_b13));
        let eq52_e932_d_b14: f64 = ((eq52_e926_d_b14 * eq52_e931) + (eq52_e926 * eq52_e931_d_b14));
        let eq52_e932_d_b15: f64 = ((eq52_e926_d_b15 * eq52_e931) + (eq52_e926 * eq52_e931_d_b15));
        let eq52_e932_d_b16: f64 = ((eq52_e926_d_b16 * eq52_e931) + (eq52_e926 * eq52_e931_d_b16));
        let eq52_e932_d_b17: f64 = ((eq52_e926_d_b17 * eq52_e931) + (eq52_e926 * eq52_e931_d_b17));
        let eq52_e932_d_b18: f64 = ((eq52_e926_d_b18 * eq52_e931) + (eq52_e926 * eq52_e931_d_b18));
        let eq52_e932_d_b19: f64 = ((eq52_e926_d_b19 * eq52_e931) + (eq52_e926 * eq52_e931_d_b19));
        let eq52_e932_d_b20: f64 = ((eq52_e926_d_b20 * eq52_e931) + (eq52_e926 * eq52_e931_d_b20));
        let eq52_e932_d_b21: f64 = ((eq52_e926_d_b21 * eq52_e931) + (eq52_e926 * eq52_e931_d_b21));
        let eq52_e932_d_b22: f64 = ((eq52_e926_d_b22 * eq52_e931) + (eq52_e926 * eq52_e931_d_b22));
        let eq52_e932_d_b23: f64 = ((eq52_e926_d_b23 * eq52_e931) + (eq52_e926 * eq52_e931_d_b23));
        let eq52_e932_d_b24: f64 = ((eq52_e926_d_b24 * eq52_e931) + (eq52_e926 * eq52_e931_d_b24));
        let eq52_e932_d_b25: f64 = ((eq52_e926_d_b25 * eq52_e931) + (eq52_e926 * eq52_e931_d_b25));
        let eq52_e932_d_b26: f64 = ((eq52_e926_d_b26 * eq52_e931) + (eq52_e926 * eq52_e931_d_b26));
        let eq52_e932_d_b27: f64 = ((eq52_e926_d_b27 * eq52_e931) + (eq52_e926 * eq52_e931_d_b27));
        let eq52_e932_d_b28: f64 = ((eq52_e926_d_b28 * eq52_e931) + (eq52_e926 * eq52_e931_d_b28));
        let eq52_e932_d_b29: f64 = ((eq52_e926_d_b29 * eq52_e931) + (eq52_e926 * eq52_e931_d_b29));
        let eq52_e932_d_b30: f64 = ((eq52_e926_d_b30 * eq52_e931) + (eq52_e926 * eq52_e931_d_b30));
        let eq52_e932_d_b31: f64 = ((eq52_e926_d_b31 * eq52_e931) + (eq52_e926 * eq52_e931_d_b31));
        let eq52_e932_d_b32: f64 = ((eq52_e926_d_b32 * eq52_e931) + (eq52_e926 * eq52_e931_d_b32));
        let eq52_e932_d_b33: f64 = ((eq52_e926_d_b33 * eq52_e931) + (eq52_e926 * eq52_e931_d_b33));
        let eq52_e932_d_b34: f64 = ((eq52_e926_d_b34 * eq52_e931) + (eq52_e926 * eq52_e931_d_b34));
        let eq52_e932_d_b35: f64 = ((eq52_e926_d_b35 * eq52_e931) + (eq52_e926 * eq52_e931_d_b35));
        let eq52_e932_d_b36: f64 = ((eq52_e926_d_b36 * eq52_e931) + (eq52_e926 * eq52_e931_d_b36));
        let eq52_e932_d_b37: f64 = ((eq52_e926_d_b37 * eq52_e931) + (eq52_e926 * eq52_e931_d_b37));
        let eq52_e932_d_b38: f64 = ((eq52_e926_d_b38 * eq52_e931) + (eq52_e926 * eq52_e931_d_b38));
        let eq52_e932_d_b39: f64 = ((eq52_e926_d_b39 * eq52_e931) + (eq52_e926 * eq52_e931_d_b39));
        let eq52_e932_d_b40: f64 = ((eq52_e926_d_b40 * eq52_e931) + (eq52_e926 * eq52_e931_d_b40));
        let eq52_e932_d_b41: f64 = ((eq52_e926_d_b41 * eq52_e931) + (eq52_e926 * eq52_e931_d_b41));
        let eq52_e932_d_b42: f64 = ((eq52_e926_d_b42 * eq52_e931) + (eq52_e926 * eq52_e931_d_b42));
        let eq52_e932_d_b43: f64 = ((eq52_e926_d_b43 * eq52_e931) + (eq52_e926 * eq52_e931_d_b43));
        let eq52_e932_d_b44: f64 = ((eq52_e926_d_b44 * eq52_e931) + (eq52_e926 * eq52_e931_d_b44));
        let eq52_e932_d_b45: f64 = ((eq52_e926_d_b45 * eq52_e931) + (eq52_e926 * eq52_e931_d_b45));
        let eq52_e932_d_b46: f64 = ((eq52_e926_d_b46 * eq52_e931) + (eq52_e926 * eq52_e931_d_b46));
        let eq52_e932_d_b47: f64 = ((eq52_e926_d_b47 * eq52_e931) + (eq52_e926 * eq52_e931_d_b47));
        let eq52_e932_d_b48: f64 = ((eq52_e926_d_b48 * eq52_e931) + (eq52_e926 * eq52_e931_d_b48));
        let eq52_e932_d_b49: f64 = ((eq52_e926_d_b49 * eq52_e931) + (eq52_e926 * eq52_e931_d_b49));
        let eq52_e932_d_b50: f64 = ((eq52_e926_d_b50 * eq52_e931) + (eq52_e926 * eq52_e931_d_b50));
        let eq52_e932_d_b51: f64 = ((eq52_e926_d_b51 * eq52_e931) + (eq52_e926 * eq52_e931_d_b51));
        let eq52_e932_d_b52: f64 = ((eq52_e926_d_b52 * eq52_e931) + (eq52_e926 * eq52_e931_d_b52));
        let eq52_e932_d_b53: f64 = ((eq52_e926_d_b53 * eq52_e931) + (eq52_e926 * eq52_e931_d_b53));
        let eq52_e932_d_b54: f64 = ((eq52_e926_d_b54 * eq52_e931) + (eq52_e926 * eq52_e931_d_b54));
        let eq52_value: f64 = eq52_e932;
        let eq52_node_derivatives: [f64; 23] = [eq52_e932_d_n0, eq52_e932_d_n1, eq52_e932_d_n2, eq52_e932_d_n3, eq52_e932_d_n4, eq52_e932_d_n5, eq52_e932_d_n6, eq52_e932_d_n7, eq52_e932_d_n8, eq52_e932_d_n9, eq52_e932_d_n10, eq52_e932_d_n11, eq52_e932_d_n12, eq52_e932_d_n13, eq52_e932_d_n14, eq52_e932_d_n15, eq52_e932_d_n16, eq52_e932_d_n17, eq52_e932_d_n18, eq52_e932_d_n19, eq52_e932_d_n20, eq52_e932_d_n21, eq52_e932_d_n22];
        let eq52_branch_derivatives: [f64; 55] = [eq52_e932_d_b0, eq52_e932_d_b1, eq52_e932_d_b2, eq52_e932_d_b3, eq52_e932_d_b4, eq52_e932_d_b5, eq52_e932_d_b6, eq52_e932_d_b7, eq52_e932_d_b8, eq52_e932_d_b9, eq52_e932_d_b10, eq52_e932_d_b11, eq52_e932_d_b12, eq52_e932_d_b13, eq52_e932_d_b14, eq52_e932_d_b15, eq52_e932_d_b16, eq52_e932_d_b17, eq52_e932_d_b18, eq52_e932_d_b19, eq52_e932_d_b20, eq52_e932_d_b21, eq52_e932_d_b22, eq52_e932_d_b23, eq52_e932_d_b24, eq52_e932_d_b25, eq52_e932_d_b26, eq52_e932_d_b27, eq52_e932_d_b28, eq52_e932_d_b29, eq52_e932_d_b30, eq52_e932_d_b31, eq52_e932_d_b32, eq52_e932_d_b33, eq52_e932_d_b34, eq52_e932_d_b35, eq52_e932_d_b36, eq52_e932_d_b37, eq52_e932_d_b38, eq52_e932_d_b39, eq52_e932_d_b40, eq52_e932_d_b41, eq52_e932_d_b42, eq52_e932_d_b43, eq52_e932_d_b44, eq52_e932_d_b45, eq52_e932_d_b46, eq52_e932_d_b47, eq52_e932_d_b48, eq52_e932_d_b49, eq52_e932_d_b50, eq52_e932_d_b51, eq52_e932_d_b52, eq52_e932_d_b53, eq52_e932_d_b54];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq52_value),
            &eq52_node_derivatives,
            &eq52_branch_derivatives,
            multiplicity,
        );
        let (eq53_e938, eq53_e938_d_n0, eq53_e938_d_n1, eq53_e938_d_n2, eq53_e938_d_n3, eq53_e938_d_n4, eq53_e938_d_n5, eq53_e938_d_n6, eq53_e938_d_n7, eq53_e938_d_n8, eq53_e938_d_n9, eq53_e938_d_n10, eq53_e938_d_n11, eq53_e938_d_n12, eq53_e938_d_n13, eq53_e938_d_n14, eq53_e938_d_n15, eq53_e938_d_n16, eq53_e938_d_n17, eq53_e938_d_n18, eq53_e938_d_n19, eq53_e938_d_n20, eq53_e938_d_n21, eq53_e938_d_n22, eq53_e938_d_b0, eq53_e938_d_b1, eq53_e938_d_b2, eq53_e938_d_b3, eq53_e938_d_b4, eq53_e938_d_b5, eq53_e938_d_b6, eq53_e938_d_b7, eq53_e938_d_b8, eq53_e938_d_b9, eq53_e938_d_b10, eq53_e938_d_b11, eq53_e938_d_b12, eq53_e938_d_b13, eq53_e938_d_b14, eq53_e938_d_b15, eq53_e938_d_b16, eq53_e938_d_b17, eq53_e938_d_b18, eq53_e938_d_b19, eq53_e938_d_b20, eq53_e938_d_b21, eq53_e938_d_b22, eq53_e938_d_b23, eq53_e938_d_b24, eq53_e938_d_b25, eq53_e938_d_b26, eq53_e938_d_b27, eq53_e938_d_b28, eq53_e938_d_b29, eq53_e938_d_b30, eq53_e938_d_b31, eq53_e938_d_b32, eq53_e938_d_b33, eq53_e938_d_b34, eq53_e938_d_b35, eq53_e938_d_b36, eq53_e938_d_b37, eq53_e938_d_b38, eq53_e938_d_b39, eq53_e938_d_b40, eq53_e938_d_b41, eq53_e938_d_b42, eq53_e938_d_b43, eq53_e938_d_b44, eq53_e938_d_b45, eq53_e938_d_b46, eq53_e938_d_b47, eq53_e938_d_b48, eq53_e938_d_b49, eq53_e938_d_b50, eq53_e938_d_b51, eq53_e938_d_b52, eq53_e938_d_b53, eq53_e938_d_b54,) = {
    if s.b[423] {
        let eq53_e936: f64 = (p.p6 * s.v[206]);
        let eq53_e936_d_n0: f64 = (p.p6 * s.dn[206][0]);
        let eq53_e936_d_n1: f64 = (p.p6 * s.dn[206][1]);
        let eq53_e936_d_n2: f64 = (p.p6 * s.dn[206][2]);
        let eq53_e936_d_n3: f64 = (p.p6 * s.dn[206][3]);
        let eq53_e936_d_n4: f64 = (p.p6 * s.dn[206][4]);
        let eq53_e936_d_n5: f64 = (p.p6 * s.dn[206][5]);
        let eq53_e936_d_n6: f64 = (p.p6 * s.dn[206][6]);
        let eq53_e936_d_n7: f64 = (p.p6 * s.dn[206][7]);
        let eq53_e936_d_n8: f64 = (p.p6 * s.dn[206][8]);
        let eq53_e936_d_n9: f64 = (p.p6 * s.dn[206][9]);
        let eq53_e936_d_n10: f64 = (p.p6 * s.dn[206][10]);
        let eq53_e936_d_n11: f64 = (p.p6 * s.dn[206][11]);
        let eq53_e936_d_n12: f64 = (p.p6 * s.dn[206][12]);
        let eq53_e936_d_n13: f64 = (p.p6 * s.dn[206][13]);
        let eq53_e936_d_n14: f64 = (p.p6 * s.dn[206][14]);
        let eq53_e936_d_n15: f64 = (p.p6 * s.dn[206][15]);
        let eq53_e936_d_n16: f64 = (p.p6 * s.dn[206][16]);
        let eq53_e936_d_n17: f64 = (p.p6 * s.dn[206][17]);
        let eq53_e936_d_n18: f64 = (p.p6 * s.dn[206][18]);
        let eq53_e936_d_n19: f64 = (p.p6 * s.dn[206][19]);
        let eq53_e936_d_n20: f64 = (p.p6 * s.dn[206][20]);
        let eq53_e936_d_n21: f64 = (p.p6 * s.dn[206][21]);
        let eq53_e936_d_n22: f64 = (p.p6 * s.dn[206][22]);
        let eq53_e936_d_b0: f64 = (p.p6 * s.db[206][0]);
        let eq53_e936_d_b1: f64 = (p.p6 * s.db[206][1]);
        let eq53_e936_d_b2: f64 = (p.p6 * s.db[206][2]);
        let eq53_e936_d_b3: f64 = (p.p6 * s.db[206][3]);
        let eq53_e936_d_b4: f64 = (p.p6 * s.db[206][4]);
        let eq53_e936_d_b5: f64 = (p.p6 * s.db[206][5]);
        let eq53_e936_d_b6: f64 = (p.p6 * s.db[206][6]);
        let eq53_e936_d_b7: f64 = (p.p6 * s.db[206][7]);
        let eq53_e936_d_b8: f64 = (p.p6 * s.db[206][8]);
        let eq53_e936_d_b9: f64 = (p.p6 * s.db[206][9]);
        let eq53_e936_d_b10: f64 = (p.p6 * s.db[206][10]);
        let eq53_e936_d_b11: f64 = (p.p6 * s.db[206][11]);
        let eq53_e936_d_b12: f64 = (p.p6 * s.db[206][12]);
        let eq53_e936_d_b13: f64 = (p.p6 * s.db[206][13]);
        let eq53_e936_d_b14: f64 = (p.p6 * s.db[206][14]);
        let eq53_e936_d_b15: f64 = (p.p6 * s.db[206][15]);
        let eq53_e936_d_b16: f64 = (p.p6 * s.db[206][16]);
        let eq53_e936_d_b17: f64 = (p.p6 * s.db[206][17]);
        let eq53_e936_d_b18: f64 = (p.p6 * s.db[206][18]);
        let eq53_e936_d_b19: f64 = (p.p6 * s.db[206][19]);
        let eq53_e936_d_b20: f64 = (p.p6 * s.db[206][20]);
        let eq53_e936_d_b21: f64 = (p.p6 * s.db[206][21]);
        let eq53_e936_d_b22: f64 = (p.p6 * s.db[206][22]);
        let eq53_e936_d_b23: f64 = (p.p6 * s.db[206][23]);
        let eq53_e936_d_b24: f64 = (p.p6 * s.db[206][24]);
        let eq53_e936_d_b25: f64 = (p.p6 * s.db[206][25]);
        let eq53_e936_d_b26: f64 = (p.p6 * s.db[206][26]);
        let eq53_e936_d_b27: f64 = (p.p6 * s.db[206][27]);
        let eq53_e936_d_b28: f64 = (p.p6 * s.db[206][28]);
        let eq53_e936_d_b29: f64 = (p.p6 * s.db[206][29]);
        let eq53_e936_d_b30: f64 = (p.p6 * s.db[206][30]);
        let eq53_e936_d_b31: f64 = (p.p6 * s.db[206][31]);
        let eq53_e936_d_b32: f64 = (p.p6 * s.db[206][32]);
        let eq53_e936_d_b33: f64 = (p.p6 * s.db[206][33]);
        let eq53_e936_d_b34: f64 = (p.p6 * s.db[206][34]);
        let eq53_e936_d_b35: f64 = (p.p6 * s.db[206][35]);
        let eq53_e936_d_b36: f64 = (p.p6 * s.db[206][36]);
        let eq53_e936_d_b37: f64 = (p.p6 * s.db[206][37]);
        let eq53_e936_d_b38: f64 = (p.p6 * s.db[206][38]);
        let eq53_e936_d_b39: f64 = (p.p6 * s.db[206][39]);
        let eq53_e936_d_b40: f64 = (p.p6 * s.db[206][40]);
        let eq53_e936_d_b41: f64 = (p.p6 * s.db[206][41]);
        let eq53_e936_d_b42: f64 = (p.p6 * s.db[206][42]);
        let eq53_e936_d_b43: f64 = (p.p6 * s.db[206][43]);
        let eq53_e936_d_b44: f64 = (p.p6 * s.db[206][44]);
        let eq53_e936_d_b45: f64 = (p.p6 * s.db[206][45]);
        let eq53_e936_d_b46: f64 = (p.p6 * s.db[206][46]);
        let eq53_e936_d_b47: f64 = (p.p6 * s.db[206][47]);
        let eq53_e936_d_b48: f64 = (p.p6 * s.db[206][48]);
        let eq53_e936_d_b49: f64 = (p.p6 * s.db[206][49]);
        let eq53_e936_d_b50: f64 = (p.p6 * s.db[206][50]);
        let eq53_e936_d_b51: f64 = (p.p6 * s.db[206][51]);
        let eq53_e936_d_b52: f64 = (p.p6 * s.db[206][52]);
        let eq53_e936_d_b53: f64 = (p.p6 * s.db[206][53]);
        let eq53_e936_d_b54: f64 = (p.p6 * s.db[206][54]);
        (eq53_e936, eq53_e936_d_n0, eq53_e936_d_n1, eq53_e936_d_n2, eq53_e936_d_n3, eq53_e936_d_n4, eq53_e936_d_n5, eq53_e936_d_n6, eq53_e936_d_n7, eq53_e936_d_n8, eq53_e936_d_n9, eq53_e936_d_n10, eq53_e936_d_n11, eq53_e936_d_n12, eq53_e936_d_n13, eq53_e936_d_n14, eq53_e936_d_n15, eq53_e936_d_n16, eq53_e936_d_n17, eq53_e936_d_n18, eq53_e936_d_n19, eq53_e936_d_n20, eq53_e936_d_n21, eq53_e936_d_n22, eq53_e936_d_b0, eq53_e936_d_b1, eq53_e936_d_b2, eq53_e936_d_b3, eq53_e936_d_b4, eq53_e936_d_b5, eq53_e936_d_b6, eq53_e936_d_b7, eq53_e936_d_b8, eq53_e936_d_b9, eq53_e936_d_b10, eq53_e936_d_b11, eq53_e936_d_b12, eq53_e936_d_b13, eq53_e936_d_b14, eq53_e936_d_b15, eq53_e936_d_b16, eq53_e936_d_b17, eq53_e936_d_b18, eq53_e936_d_b19, eq53_e936_d_b20, eq53_e936_d_b21, eq53_e936_d_b22, eq53_e936_d_b23, eq53_e936_d_b24, eq53_e936_d_b25, eq53_e936_d_b26, eq53_e936_d_b27, eq53_e936_d_b28, eq53_e936_d_b29, eq53_e936_d_b30, eq53_e936_d_b31, eq53_e936_d_b32, eq53_e936_d_b33, eq53_e936_d_b34, eq53_e936_d_b35, eq53_e936_d_b36, eq53_e936_d_b37, eq53_e936_d_b38, eq53_e936_d_b39, eq53_e936_d_b40, eq53_e936_d_b41, eq53_e936_d_b42, eq53_e936_d_b43, eq53_e936_d_b44, eq53_e936_d_b45, eq53_e936_d_b46, eq53_e936_d_b47, eq53_e936_d_b48, eq53_e936_d_b49, eq53_e936_d_b50, eq53_e936_d_b51, eq53_e936_d_b52, eq53_e936_d_b53, eq53_e936_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e938;
        let eq53_node_derivatives: [f64; 23] = [eq53_e938_d_n0, eq53_e938_d_n1, eq53_e938_d_n2, eq53_e938_d_n3, eq53_e938_d_n4, eq53_e938_d_n5, eq53_e938_d_n6, eq53_e938_d_n7, eq53_e938_d_n8, eq53_e938_d_n9, eq53_e938_d_n10, eq53_e938_d_n11, eq53_e938_d_n12, eq53_e938_d_n13, eq53_e938_d_n14, eq53_e938_d_n15, eq53_e938_d_n16, eq53_e938_d_n17, eq53_e938_d_n18, eq53_e938_d_n19, eq53_e938_d_n20, eq53_e938_d_n21, eq53_e938_d_n22];
        let eq53_branch_derivatives: [f64; 55] = [eq53_e938_d_b0, eq53_e938_d_b1, eq53_e938_d_b2, eq53_e938_d_b3, eq53_e938_d_b4, eq53_e938_d_b5, eq53_e938_d_b6, eq53_e938_d_b7, eq53_e938_d_b8, eq53_e938_d_b9, eq53_e938_d_b10, eq53_e938_d_b11, eq53_e938_d_b12, eq53_e938_d_b13, eq53_e938_d_b14, eq53_e938_d_b15, eq53_e938_d_b16, eq53_e938_d_b17, eq53_e938_d_b18, eq53_e938_d_b19, eq53_e938_d_b20, eq53_e938_d_b21, eq53_e938_d_b22, eq53_e938_d_b23, eq53_e938_d_b24, eq53_e938_d_b25, eq53_e938_d_b26, eq53_e938_d_b27, eq53_e938_d_b28, eq53_e938_d_b29, eq53_e938_d_b30, eq53_e938_d_b31, eq53_e938_d_b32, eq53_e938_d_b33, eq53_e938_d_b34, eq53_e938_d_b35, eq53_e938_d_b36, eq53_e938_d_b37, eq53_e938_d_b38, eq53_e938_d_b39, eq53_e938_d_b40, eq53_e938_d_b41, eq53_e938_d_b42, eq53_e938_d_b43, eq53_e938_d_b44, eq53_e938_d_b45, eq53_e938_d_b46, eq53_e938_d_b47, eq53_e938_d_b48, eq53_e938_d_b49, eq53_e938_d_b50, eq53_e938_d_b51, eq53_e938_d_b52, eq53_e938_d_b53, eq53_e938_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq53_value),
            &eq53_node_derivatives,
            &eq53_branch_derivatives,
            multiplicity,
        );
        let (eq54_e944, eq54_e944_d_n0, eq54_e944_d_n1, eq54_e944_d_n2, eq54_e944_d_n3, eq54_e944_d_n4, eq54_e944_d_n5, eq54_e944_d_n6, eq54_e944_d_n7, eq54_e944_d_n8, eq54_e944_d_n9, eq54_e944_d_n10, eq54_e944_d_n11, eq54_e944_d_n12, eq54_e944_d_n13, eq54_e944_d_n14, eq54_e944_d_n15, eq54_e944_d_n16, eq54_e944_d_n17, eq54_e944_d_n18, eq54_e944_d_n19, eq54_e944_d_n20, eq54_e944_d_n21, eq54_e944_d_n22, eq54_e944_d_b0, eq54_e944_d_b1, eq54_e944_d_b2, eq54_e944_d_b3, eq54_e944_d_b4, eq54_e944_d_b5, eq54_e944_d_b6, eq54_e944_d_b7, eq54_e944_d_b8, eq54_e944_d_b9, eq54_e944_d_b10, eq54_e944_d_b11, eq54_e944_d_b12, eq54_e944_d_b13, eq54_e944_d_b14, eq54_e944_d_b15, eq54_e944_d_b16, eq54_e944_d_b17, eq54_e944_d_b18, eq54_e944_d_b19, eq54_e944_d_b20, eq54_e944_d_b21, eq54_e944_d_b22, eq54_e944_d_b23, eq54_e944_d_b24, eq54_e944_d_b25, eq54_e944_d_b26, eq54_e944_d_b27, eq54_e944_d_b28, eq54_e944_d_b29, eq54_e944_d_b30, eq54_e944_d_b31, eq54_e944_d_b32, eq54_e944_d_b33, eq54_e944_d_b34, eq54_e944_d_b35, eq54_e944_d_b36, eq54_e944_d_b37, eq54_e944_d_b38, eq54_e944_d_b39, eq54_e944_d_b40, eq54_e944_d_b41, eq54_e944_d_b42, eq54_e944_d_b43, eq54_e944_d_b44, eq54_e944_d_b45, eq54_e944_d_b46, eq54_e944_d_b47, eq54_e944_d_b48, eq54_e944_d_b49, eq54_e944_d_b50, eq54_e944_d_b51, eq54_e944_d_b52, eq54_e944_d_b53, eq54_e944_d_b54,) = {
    if s.b[423] {
        let eq54_e942: f64 = (p.p6 * s.v[207]);
        let eq54_e942_d_n0: f64 = (p.p6 * s.dn[207][0]);
        let eq54_e942_d_n1: f64 = (p.p6 * s.dn[207][1]);
        let eq54_e942_d_n2: f64 = (p.p6 * s.dn[207][2]);
        let eq54_e942_d_n3: f64 = (p.p6 * s.dn[207][3]);
        let eq54_e942_d_n4: f64 = (p.p6 * s.dn[207][4]);
        let eq54_e942_d_n5: f64 = (p.p6 * s.dn[207][5]);
        let eq54_e942_d_n6: f64 = (p.p6 * s.dn[207][6]);
        let eq54_e942_d_n7: f64 = (p.p6 * s.dn[207][7]);
        let eq54_e942_d_n8: f64 = (p.p6 * s.dn[207][8]);
        let eq54_e942_d_n9: f64 = (p.p6 * s.dn[207][9]);
        let eq54_e942_d_n10: f64 = (p.p6 * s.dn[207][10]);
        let eq54_e942_d_n11: f64 = (p.p6 * s.dn[207][11]);
        let eq54_e942_d_n12: f64 = (p.p6 * s.dn[207][12]);
        let eq54_e942_d_n13: f64 = (p.p6 * s.dn[207][13]);
        let eq54_e942_d_n14: f64 = (p.p6 * s.dn[207][14]);
        let eq54_e942_d_n15: f64 = (p.p6 * s.dn[207][15]);
        let eq54_e942_d_n16: f64 = (p.p6 * s.dn[207][16]);
        let eq54_e942_d_n17: f64 = (p.p6 * s.dn[207][17]);
        let eq54_e942_d_n18: f64 = (p.p6 * s.dn[207][18]);
        let eq54_e942_d_n19: f64 = (p.p6 * s.dn[207][19]);
        let eq54_e942_d_n20: f64 = (p.p6 * s.dn[207][20]);
        let eq54_e942_d_n21: f64 = (p.p6 * s.dn[207][21]);
        let eq54_e942_d_n22: f64 = (p.p6 * s.dn[207][22]);
        let eq54_e942_d_b0: f64 = (p.p6 * s.db[207][0]);
        let eq54_e942_d_b1: f64 = (p.p6 * s.db[207][1]);
        let eq54_e942_d_b2: f64 = (p.p6 * s.db[207][2]);
        let eq54_e942_d_b3: f64 = (p.p6 * s.db[207][3]);
        let eq54_e942_d_b4: f64 = (p.p6 * s.db[207][4]);
        let eq54_e942_d_b5: f64 = (p.p6 * s.db[207][5]);
        let eq54_e942_d_b6: f64 = (p.p6 * s.db[207][6]);
        let eq54_e942_d_b7: f64 = (p.p6 * s.db[207][7]);
        let eq54_e942_d_b8: f64 = (p.p6 * s.db[207][8]);
        let eq54_e942_d_b9: f64 = (p.p6 * s.db[207][9]);
        let eq54_e942_d_b10: f64 = (p.p6 * s.db[207][10]);
        let eq54_e942_d_b11: f64 = (p.p6 * s.db[207][11]);
        let eq54_e942_d_b12: f64 = (p.p6 * s.db[207][12]);
        let eq54_e942_d_b13: f64 = (p.p6 * s.db[207][13]);
        let eq54_e942_d_b14: f64 = (p.p6 * s.db[207][14]);
        let eq54_e942_d_b15: f64 = (p.p6 * s.db[207][15]);
        let eq54_e942_d_b16: f64 = (p.p6 * s.db[207][16]);
        let eq54_e942_d_b17: f64 = (p.p6 * s.db[207][17]);
        let eq54_e942_d_b18: f64 = (p.p6 * s.db[207][18]);
        let eq54_e942_d_b19: f64 = (p.p6 * s.db[207][19]);
        let eq54_e942_d_b20: f64 = (p.p6 * s.db[207][20]);
        let eq54_e942_d_b21: f64 = (p.p6 * s.db[207][21]);
        let eq54_e942_d_b22: f64 = (p.p6 * s.db[207][22]);
        let eq54_e942_d_b23: f64 = (p.p6 * s.db[207][23]);
        let eq54_e942_d_b24: f64 = (p.p6 * s.db[207][24]);
        let eq54_e942_d_b25: f64 = (p.p6 * s.db[207][25]);
        let eq54_e942_d_b26: f64 = (p.p6 * s.db[207][26]);
        let eq54_e942_d_b27: f64 = (p.p6 * s.db[207][27]);
        let eq54_e942_d_b28: f64 = (p.p6 * s.db[207][28]);
        let eq54_e942_d_b29: f64 = (p.p6 * s.db[207][29]);
        let eq54_e942_d_b30: f64 = (p.p6 * s.db[207][30]);
        let eq54_e942_d_b31: f64 = (p.p6 * s.db[207][31]);
        let eq54_e942_d_b32: f64 = (p.p6 * s.db[207][32]);
        let eq54_e942_d_b33: f64 = (p.p6 * s.db[207][33]);
        let eq54_e942_d_b34: f64 = (p.p6 * s.db[207][34]);
        let eq54_e942_d_b35: f64 = (p.p6 * s.db[207][35]);
        let eq54_e942_d_b36: f64 = (p.p6 * s.db[207][36]);
        let eq54_e942_d_b37: f64 = (p.p6 * s.db[207][37]);
        let eq54_e942_d_b38: f64 = (p.p6 * s.db[207][38]);
        let eq54_e942_d_b39: f64 = (p.p6 * s.db[207][39]);
        let eq54_e942_d_b40: f64 = (p.p6 * s.db[207][40]);
        let eq54_e942_d_b41: f64 = (p.p6 * s.db[207][41]);
        let eq54_e942_d_b42: f64 = (p.p6 * s.db[207][42]);
        let eq54_e942_d_b43: f64 = (p.p6 * s.db[207][43]);
        let eq54_e942_d_b44: f64 = (p.p6 * s.db[207][44]);
        let eq54_e942_d_b45: f64 = (p.p6 * s.db[207][45]);
        let eq54_e942_d_b46: f64 = (p.p6 * s.db[207][46]);
        let eq54_e942_d_b47: f64 = (p.p6 * s.db[207][47]);
        let eq54_e942_d_b48: f64 = (p.p6 * s.db[207][48]);
        let eq54_e942_d_b49: f64 = (p.p6 * s.db[207][49]);
        let eq54_e942_d_b50: f64 = (p.p6 * s.db[207][50]);
        let eq54_e942_d_b51: f64 = (p.p6 * s.db[207][51]);
        let eq54_e942_d_b52: f64 = (p.p6 * s.db[207][52]);
        let eq54_e942_d_b53: f64 = (p.p6 * s.db[207][53]);
        let eq54_e942_d_b54: f64 = (p.p6 * s.db[207][54]);
        (eq54_e942, eq54_e942_d_n0, eq54_e942_d_n1, eq54_e942_d_n2, eq54_e942_d_n3, eq54_e942_d_n4, eq54_e942_d_n5, eq54_e942_d_n6, eq54_e942_d_n7, eq54_e942_d_n8, eq54_e942_d_n9, eq54_e942_d_n10, eq54_e942_d_n11, eq54_e942_d_n12, eq54_e942_d_n13, eq54_e942_d_n14, eq54_e942_d_n15, eq54_e942_d_n16, eq54_e942_d_n17, eq54_e942_d_n18, eq54_e942_d_n19, eq54_e942_d_n20, eq54_e942_d_n21, eq54_e942_d_n22, eq54_e942_d_b0, eq54_e942_d_b1, eq54_e942_d_b2, eq54_e942_d_b3, eq54_e942_d_b4, eq54_e942_d_b5, eq54_e942_d_b6, eq54_e942_d_b7, eq54_e942_d_b8, eq54_e942_d_b9, eq54_e942_d_b10, eq54_e942_d_b11, eq54_e942_d_b12, eq54_e942_d_b13, eq54_e942_d_b14, eq54_e942_d_b15, eq54_e942_d_b16, eq54_e942_d_b17, eq54_e942_d_b18, eq54_e942_d_b19, eq54_e942_d_b20, eq54_e942_d_b21, eq54_e942_d_b22, eq54_e942_d_b23, eq54_e942_d_b24, eq54_e942_d_b25, eq54_e942_d_b26, eq54_e942_d_b27, eq54_e942_d_b28, eq54_e942_d_b29, eq54_e942_d_b30, eq54_e942_d_b31, eq54_e942_d_b32, eq54_e942_d_b33, eq54_e942_d_b34, eq54_e942_d_b35, eq54_e942_d_b36, eq54_e942_d_b37, eq54_e942_d_b38, eq54_e942_d_b39, eq54_e942_d_b40, eq54_e942_d_b41, eq54_e942_d_b42, eq54_e942_d_b43, eq54_e942_d_b44, eq54_e942_d_b45, eq54_e942_d_b46, eq54_e942_d_b47, eq54_e942_d_b48, eq54_e942_d_b49, eq54_e942_d_b50, eq54_e942_d_b51, eq54_e942_d_b52, eq54_e942_d_b53, eq54_e942_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e944;
        let eq54_node_derivatives: [f64; 23] = [eq54_e944_d_n0, eq54_e944_d_n1, eq54_e944_d_n2, eq54_e944_d_n3, eq54_e944_d_n4, eq54_e944_d_n5, eq54_e944_d_n6, eq54_e944_d_n7, eq54_e944_d_n8, eq54_e944_d_n9, eq54_e944_d_n10, eq54_e944_d_n11, eq54_e944_d_n12, eq54_e944_d_n13, eq54_e944_d_n14, eq54_e944_d_n15, eq54_e944_d_n16, eq54_e944_d_n17, eq54_e944_d_n18, eq54_e944_d_n19, eq54_e944_d_n20, eq54_e944_d_n21, eq54_e944_d_n22];
        let eq54_branch_derivatives: [f64; 55] = [eq54_e944_d_b0, eq54_e944_d_b1, eq54_e944_d_b2, eq54_e944_d_b3, eq54_e944_d_b4, eq54_e944_d_b5, eq54_e944_d_b6, eq54_e944_d_b7, eq54_e944_d_b8, eq54_e944_d_b9, eq54_e944_d_b10, eq54_e944_d_b11, eq54_e944_d_b12, eq54_e944_d_b13, eq54_e944_d_b14, eq54_e944_d_b15, eq54_e944_d_b16, eq54_e944_d_b17, eq54_e944_d_b18, eq54_e944_d_b19, eq54_e944_d_b20, eq54_e944_d_b21, eq54_e944_d_b22, eq54_e944_d_b23, eq54_e944_d_b24, eq54_e944_d_b25, eq54_e944_d_b26, eq54_e944_d_b27, eq54_e944_d_b28, eq54_e944_d_b29, eq54_e944_d_b30, eq54_e944_d_b31, eq54_e944_d_b32, eq54_e944_d_b33, eq54_e944_d_b34, eq54_e944_d_b35, eq54_e944_d_b36, eq54_e944_d_b37, eq54_e944_d_b38, eq54_e944_d_b39, eq54_e944_d_b40, eq54_e944_d_b41, eq54_e944_d_b42, eq54_e944_d_b43, eq54_e944_d_b44, eq54_e944_d_b45, eq54_e944_d_b46, eq54_e944_d_b47, eq54_e944_d_b48, eq54_e944_d_b49, eq54_e944_d_b50, eq54_e944_d_b51, eq54_e944_d_b52, eq54_e944_d_b53, eq54_e944_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq54_value),
            &eq54_node_derivatives,
            &eq54_branch_derivatives,
            multiplicity,
        );
    }
}
