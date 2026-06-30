#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[493] && s.b[494]) && s.b[496]) {
            s.copy_ad(278, 67);
        }

        if ((s.b[493] && s.b[494]) && (!s.b[496])) {
            s.copy_ad(279, 65);
            s.copy_ad(278, 66);
        }

        if (s.b[493] && s.b[494]) {
            s.store_offset_sqrt_ad(280, A::offset(A::square(s.ad_value(279)), 0.01), (-0.1));
            s.store_offset_scaled(146, 280, p.p192, (1.0 + p.p191));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_sub_ad(88, A::sub_from_scalar(p.p185, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p188)), (((-1.0)) * (p.p188)))), A::div_scaled_inputs(s.ad_value(280), (p.p194 * p.p193), A::sqrt_square_offset(s.ad_value(280), (p.p194 * p.p194)), 1.0));
            s.store_scalar(271, (p.p9 / p.p186));
            s.store_div_from_scalar_scaled_mul(136, p.p187, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p184), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 278, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(278), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);
            s.store_sub(270, 160, 88);
            s.store_div_scaled_inputs_indices(84, 271, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 271, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 270, 270, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
        }

        if (s.b[493] && s.b[494]) {
            let assign18800_ad_e29369: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, assign18800_ad_e29369, (-(p.p195 / 3.0)), A::add_scaled_offset_product_rhs(assign18800_ad_e29369, ((2.0 * p.p195) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
        }

        if (s.b[493] && s.b[494]) {
            s.store_div_scaled_inputs_indices(136, 270, 1.0, 83, 2.0);
        }

        s.b[497] = (s.v[136] < 200.0);
        s.store_scalar(497, if s.b[497] { 1.0 } else { 0.0 });

        if ((s.b[493] && s.b[494]) && s.b[497]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(270), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[493] && s.b[494]) && (!s.b[497])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(270), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[493] && s.b[494]) {
            s.store_sub_div_rhs_indices(100, 270, 153, 99);
        }

        s.b[498] = ((((s.v[100] - s.v[270])) as f64).abs() > 1e-19);
        s.store_scalar(498, if s.b[498] { 1.0 } else { 0.0 });

        if ((s.b[493] && s.b[494]) && s.b[498]) {
            s.store_sub(101, 270, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p195);
            s.store_scaled_mul(103, 136, 90, p.p196);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if ((s.b[493] && s.b[494]) && s.b[498]) {
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

        if ((s.b[493] && s.b[494]) && s.b[498]) {
            s.store_scaled_mul(107, 136, 91, p.p195);
            s.store_scaled_mul(108, 136, 91, p.p196);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 270, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p195, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p196, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if ((s.b[493] && s.b[494]) && s.b[498]) {
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

        if ((s.b[493] && s.b[494]) && s.b[498]) {
            s.store_scaled_mul(121, 136, 137, p.p195);
            s.store_scaled_mul(122, 136, 137, p.p196);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.copy_ad(272, 128);
        }

        if ((s.b[493] && s.b[494]) && (!s.b[498])) {
            s.copy_ad(272, 100);
        }

        if (s.b[493] && s.b[494]) {
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p189);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p190);
            s.store_mul_scaled_abs_ad_rhs(136, 271, 1.0 / (p.p9), A::sub(s.ad_value(270), s.ad_value(272)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(272)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(90, 270, 270, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p187, 136, p.p187, 90, 1.0, 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(279), s.ad_value(85)), p.p18);
            s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));
            s.store_mul(86, 279, 90);
            s.store_sub(39, 270, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
        }

        if (s.b[493] && s.b[494]) {
            let assign19420_ad_e30398: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, assign19420_ad_e30398, (-(p.p195 / 3.0)), A::add_scaled_offset_product_rhs(assign19420_ad_e30398, ((2.0 * p.p195) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
        }

        if (s.b[493] && s.b[494]) {
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[499] = (s.v[136] < 200.0);
        s.store_scalar(499, if s.b[499] { 1.0 } else { 0.0 });

        if ((s.b[493] && s.b[494]) && s.b[499]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[493] && s.b[494]) && (!s.b[499])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[493] && s.b[494]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[500] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.store_scalar(500, if s.b[500] { 1.0 } else { 0.0 });

        if ((s.b[493] && s.b[494]) && s.b[500]) {
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

        if ((s.b[493] && s.b[494]) && s.b[500]) {
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

        if ((s.b[493] && s.b[494]) && s.b[500]) {
            s.store_scaled_mul(107, 136, 91, p.p195);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[493] && s.b[494]) && s.b[500]) {
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

        if ((s.b[493] && s.b[494]) && s.b[500]) {
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

        if ((s.b[493] && s.b[494]) && s.b[500]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p195, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p196, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(273, 128, 86);
        }

        if ((s.b[493] && s.b[494]) && (!s.b[500])) {
            s.store_add(273, 100, 86);
        }

        if (s.b[493] && s.b[494]) {
            s.store_scaled_add(274, 272, 273, 0.5);
            s.store_sub(275, 273, 272);
            s.store_mul_add_scaled_inputs3_offset_rhs(135, 275, s.ad_value(270), 1.0, s.ad_value(274), (-1.0), s.ad_value(83), 1.0, 0.0);
            s.store_mul_scaled_abs_ad_rhs(136, 271, 1.0 / (p.p9), A::sub(s.ad_value(270), s.ad_value(274)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);
            s.store_scaled_mul(96, 95, 271, (p.p4 * (p.p5 * 1.0 / (p.p187))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(280), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(275), (p.p25 * p.p25), s.ad_value(275)), 1.0);
            s.store_div(93, 98, 92);
            s.store_sub(90, 273, 272);
            s.store_add_scaled_inputs3_indices(91, 270, 1.0, 83, 1.0, 274, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 271, s.ad_value(270), ((p.p4 * p.p5) * p.p187), s.ad_value(274), (((-1.0)) * (((p.p4 * p.p5) * p.p187))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p187), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p242) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p241, 1.0);
            s.store_div_from_scalar(190, p.p240, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p186);
            s.store_mul_add_scaled_inputs3_offset_rhs(276, 191, s.ad_value(270), ((p.p4 * p.p5) * p.p187), s.ad_value(274), (((-1.0)) * (((p.p4 * p.p5) * p.p187))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p187), 0.0);
            s.store_add_scaled_inputs3_indices(136, 270, 1.0, 83, 1.0, 274, -1.0);
            s.store_add_scaled_inputs(90, 272, 0.3333333333333333, 273, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(275)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(275)), 275, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(277, 191, 270, (-(((p.p4 * p.p187) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p187) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p187) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p187) * p.p5) * 0.5)));
        }

        s.b[501] = (s.v[64] < 0.0);
        s.store_scalar(501, if s.b[501] { 1.0 } else { 0.0 });

        if ((s.b[493] && s.b[494]) && s.b[501]) {
            s.store_sub_scaled_inputs(277, 276, (-1.0), 277, 1.0);
        }

        if (s.b[493] && (!s.b[494])) {
            s.store_scalar(276, 0.0);
            s.store_scalar(277, 0.0);
        }

        s.b[502] = (p.p154 != 0.0);
        s.store_scalar(502, if s.b[502] { 1.0 } else { 0.0 });

        s.b[503] = (p.p154 == 1.0);
        s.store_scalar(503, if s.b[503] { 1.0 } else { 0.0 });

        if (((!s.b[493]) && s.b[502]) && s.b[503]) {
            s.store_voltage(66, ctx, nodes, Some(9), Some(7));
        }

        if (((!s.b[493]) && s.b[502]) && (!s.b[503])) {
            s.store_voltage(66, ctx, nodes, Some(2), Some(7));
        }

        if ((!s.b[493]) && s.b[502]) {
            s.copy_ad(278, 66);
            s.store_scalar(146, (1.0 + p.p191));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_sub_from_scalar_ad(88, p.p185, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p188)), (((-1.0)) * (p.p188))));
            s.store_scalar(271, (p.p9 / p.p186));
            s.store_div_from_scalar_scaled_mul(136, p.p187, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p184), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 278, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(278), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);
            s.store_sub(270, 160, 88);
            s.store_div_scaled_inputs_indices(84, 271, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 271, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 270, 270, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
        }

        if ((!s.b[493]) && s.b[502]) {
            let assign20340_ad_e31796: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, assign20340_ad_e31796, (-(p.p195 / 3.0)), A::add_scaled_offset_product_rhs(assign20340_ad_e31796, ((2.0 * p.p195) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
        }

        if ((!s.b[493]) && s.b[502]) {
            s.store_div_scaled_inputs_indices(136, 270, 1.0, 83, 2.0);
        }

        s.b[504] = (s.v[136] < 200.0);
        s.store_scalar(504, if s.b[504] { 1.0 } else { 0.0 });

        if (((!s.b[493]) && s.b[502]) && s.b[504]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(270), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[493]) && s.b[502]) && (!s.b[504])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(270), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[493]) && s.b[502]) {
            s.store_sub_div_rhs_indices(100, 270, 153, 99);
        }

        s.b[505] = ((((s.v[100] - s.v[270])) as f64).abs() > 1e-19);
        s.store_scalar(505, if s.b[505] { 1.0 } else { 0.0 });

        if (((!s.b[493]) && s.b[502]) && s.b[505]) {
            s.store_sub(101, 270, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p195);
            s.store_scaled_mul(103, 136, 90, p.p196);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if (((!s.b[493]) && s.b[502]) && s.b[505]) {
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

        if (((!s.b[493]) && s.b[502]) && s.b[505]) {
            s.store_scaled_mul(107, 136, 91, p.p195);
            s.store_scaled_mul(108, 136, 91, p.p196);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 270, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p195, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p196, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

    }

    pub(super) fn stamp_reactive_block_17(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[493]) && s.b[502]) && s.b[505]) {
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

        if (((!s.b[493]) && s.b[502]) && s.b[505]) {
            s.store_scaled_mul(121, 136, 137, p.p195);
            s.store_scaled_mul(122, 136, 137, p.p196);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.copy_ad(272, 128);
        }

        if (((!s.b[493]) && s.b[502]) && (!s.b[505])) {
            s.copy_ad(272, 100);
        }

        if ((!s.b[493]) && s.b[502]) {
            s.store_scalar(279, 0.0);
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p189);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p190);
            s.store_mul_scaled_abs_ad_rhs(136, 271, 1.0 / (p.p9), A::sub(s.ad_value(270), s.ad_value(272)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(272)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(90, 270, 270, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p187, 136, p.p187, 90, 1.0, 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(279), s.ad_value(85)), p.p18);
            s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));
            s.store_mul(86, 279, 90);
            s.store_sub(39, 270, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
        }

        if ((!s.b[493]) && s.b[502]) {
            let assign20970_ad_e32892: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, assign20970_ad_e32892, (-(p.p195 / 3.0)), A::add_scaled_offset_product_rhs(assign20970_ad_e32892, ((2.0 * p.p195) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
        }

        if ((!s.b[493]) && s.b[502]) {
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[506] = (s.v[136] < 200.0);
        s.store_scalar(506, if s.b[506] { 1.0 } else { 0.0 });

        if (((!s.b[493]) && s.b[502]) && s.b[506]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[493]) && s.b[502]) && (!s.b[506])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[493]) && s.b[502]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[507] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.store_scalar(507, if s.b[507] { 1.0 } else { 0.0 });

        if (((!s.b[493]) && s.b[502]) && s.b[507]) {
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

        if (((!s.b[493]) && s.b[502]) && s.b[507]) {
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

        if (((!s.b[493]) && s.b[502]) && s.b[507]) {
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

        if (((!s.b[493]) && s.b[502]) && s.b[507]) {
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

        if (((!s.b[493]) && s.b[502]) && s.b[507]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p195, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p196, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(273, 128, 86);
        }

        if (((!s.b[493]) && s.b[502]) && (!s.b[507])) {
            s.store_add(273, 100, 86);
        }

        if ((!s.b[493]) && s.b[502]) {
            s.store_scaled_add(274, 272, 273, 0.5);
            s.store_sub(275, 273, 272);
            s.store_sub(90, 273, 272);
            s.store_add_scaled_inputs3_indices(91, 270, 1.0, 83, 1.0, 274, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 271, s.ad_value(270), ((p.p4 * p.p5) * p.p187), s.ad_value(274), (((-1.0)) * (((p.p4 * p.p5) * p.p187))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p187), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p242) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p241, 1.0);
            s.store_div_from_scalar(190, p.p240, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p186);
            s.store_mul_add_scaled_inputs3_offset_rhs(276, 191, s.ad_value(270), ((p.p4 * p.p5) * p.p187), s.ad_value(274), (((-1.0)) * (((p.p4 * p.p5) * p.p187))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p187), 0.0);
            s.store_add_scaled_inputs3_indices(136, 270, 1.0, 83, 1.0, 274, -1.0);
            s.store_add_scaled_inputs(90, 272, 0.3333333333333333, 273, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(275)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(275)), 275, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(277, 191, 270, (-(((p.p4 * p.p187) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p187) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p187) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p187) * p.p5) * 0.5)));
        }

        if ((!s.b[493]) && (!s.b[502])) {
            s.store_scalar(276, 0.0);
            s.store_scalar(277, 0.0);
        }

        s.b[508] = (p.p149 == 0.0);
        s.store_scalar(508, if s.b[508] { 1.0 } else { 0.0 });

        s.b[509] = (p.p155 != 0.0);
        s.store_scalar(509, if s.b[509] { 1.0 } else { 0.0 });

        if (s.b[508] && s.b[509]) {
            s.store_voltage(69, ctx, nodes, Some(20), Some(21));
        }

        s.b[510] = (p.p155 == 1.0);
        s.store_scalar(510, if s.b[510] { 1.0 } else { 0.0 });

        if ((s.b[508] && s.b[509]) && s.b[510]) {
            s.store_voltage(70, ctx, nodes, Some(9), Some(21));
            s.store_voltage(71, ctx, nodes, Some(9), Some(20));
        }

        if ((s.b[508] && s.b[509]) && (!s.b[510])) {
            s.store_voltage(70, ctx, nodes, Some(2), Some(21));
            s.store_voltage(71, ctx, nodes, Some(2), Some(20));
        }

        if (s.b[508] && s.b[509]) {
            s.store_scalar(68, 1.0);
        }

        s.b[511] = (s.v[69] < 0.0);
        s.store_scalar(511, if s.b[511] { 1.0 } else { 0.0 });

        if ((s.b[508] && s.b[509]) && s.b[511]) {
            s.store_scalar(68, (-1.0));
            s.store_mul(291, 68, 69);
        }

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[508] && s.b[509]) && s.b[511]) {
            s.copy_ad(290, 71);
        }

        if ((s.b[508] && s.b[509]) && (!s.b[511])) {
            s.copy_ad(291, 69);
            s.copy_ad(290, 70);
        }

        if (s.b[508] && s.b[509]) {
            s.store_offset_sqrt_ad(292, A::offset(A::square(s.ad_value(291)), 0.01), (-0.1));
            s.store_offset_scaled(146, 292, p.p192, (1.0 + p.p191));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_sub_ad(88, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p188)), (((((-1.0)) * (p.p188))) + (p.p185))), A::div_scaled_inputs(s.ad_value(292), (p.p194 * p.p193), A::sqrt_square_offset(s.ad_value(292), (p.p194 * p.p194)), 1.0));
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
        }

        if (s.b[508] && s.b[509]) {
            let assign21890_ad_e34298: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, assign21890_ad_e34298, (-(p.p195 / 3.0)), A::add_scaled_offset_product_rhs(assign21890_ad_e34298, ((2.0 * p.p195) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
        }

        if (s.b[508] && s.b[509]) {
            s.store_div_scaled_inputs_indices(136, 282, 1.0, 83, 2.0);
        }

        s.b[512] = (s.v[136] < 200.0);
        s.store_scalar(512, if s.b[512] { 1.0 } else { 0.0 });

        if ((s.b[508] && s.b[509]) && s.b[512]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(282), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[508] && s.b[509]) && (!s.b[512])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(282), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[508] && s.b[509]) {
            s.store_sub_div_rhs_indices(100, 282, 153, 99);
        }

        s.b[513] = ((((s.v[100] - s.v[282])) as f64).abs() > 1e-19);
        s.store_scalar(513, if s.b[513] { 1.0 } else { 0.0 });

        if ((s.b[508] && s.b[509]) && s.b[513]) {
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

        if ((s.b[508] && s.b[509]) && s.b[513]) {
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

        if ((s.b[508] && s.b[509]) && s.b[513]) {
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

        if ((s.b[508] && s.b[509]) && s.b[513]) {
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

        if ((s.b[508] && s.b[509]) && s.b[513]) {
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

        if ((s.b[508] && s.b[509]) && (!s.b[513])) {
            s.copy_ad(284, 100);
        }

        if (s.b[508] && s.b[509]) {
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
        }

        if (s.b[508] && s.b[509]) {
            let assign22510_ad_e35327: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, assign22510_ad_e35327, (-(p.p195 / 3.0)), A::add_scaled_offset_product_rhs(assign22510_ad_e35327, ((2.0 * p.p195) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
        }

        if (s.b[508] && s.b[509]) {
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[514] = (s.v[136] < 200.0);
        s.store_scalar(514, if s.b[514] { 1.0 } else { 0.0 });

        if ((s.b[508] && s.b[509]) && s.b[514]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[508] && s.b[509]) && (!s.b[514])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[508] && s.b[509]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[515] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.store_scalar(515, if s.b[515] { 1.0 } else { 0.0 });

        if ((s.b[508] && s.b[509]) && s.b[515]) {
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

        if ((s.b[508] && s.b[509]) && s.b[515]) {
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

        if ((s.b[508] && s.b[509]) && s.b[515]) {
            s.store_scaled_mul(107, 136, 91, p.p195);
        }

    }

    pub(super) fn stamp_reactive_block_19(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[508] && s.b[509]) && s.b[515]) {
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
        s.store_scalar(516, if s.b[516] { 1.0 } else { 0.0 });

        if ((s.b[508] && s.b[509]) && s.b[516]) {
            s.store_sub_scaled_inputs(289, 288, (-1.0), 289, 1.0);
        }

        if (s.b[508] && (!s.b[509])) {
            s.store_scalar(288, 0.0);
            s.store_scalar(289, 0.0);
        }

        s.b[517] = (p.p155 != 0.0);
        s.store_scalar(517, if s.b[517] { 1.0 } else { 0.0 });

        s.b[518] = (p.p155 == 1.0);
        s.store_scalar(518, if s.b[518] { 1.0 } else { 0.0 });

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
        }

        if ((!s.b[508]) && s.b[517]) {
            let assign23430_ad_e36725: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, assign23430_ad_e36725, (-(p.p195 / 3.0)), A::add_scaled_offset_product_rhs(assign23430_ad_e36725, ((2.0 * p.p195) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
        }

        if ((!s.b[508]) && s.b[517]) {
            s.store_div_scaled_inputs_indices(136, 282, 1.0, 83, 2.0);
        }

        s.b[519] = (s.v[136] < 200.0);
        s.store_scalar(519, if s.b[519] { 1.0 } else { 0.0 });

        if (((!s.b[508]) && s.b[517]) && s.b[519]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(282), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[508]) && s.b[517]) && (!s.b[519])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(282), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[508]) && s.b[517]) {
            s.store_sub_div_rhs_indices(100, 282, 153, 99);
        }

        s.b[520] = ((((s.v[100] - s.v[282])) as f64).abs() > 1e-19);
        s.store_scalar(520, if s.b[520] { 1.0 } else { 0.0 });

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
        }

        if ((!s.b[508]) && s.b[517]) {
            let assign24060_ad_e37821: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, assign24060_ad_e37821, (-(p.p195 / 3.0)), A::add_scaled_offset_product_rhs(assign24060_ad_e37821, ((2.0 * p.p195) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
        }

        if ((!s.b[508]) && s.b[517]) {
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[521] = (s.v[136] < 200.0);
        s.store_scalar(521, if s.b[521] { 1.0 } else { 0.0 });

        if (((!s.b[508]) && s.b[517]) && s.b[521]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[508]) && s.b[517]) && (!s.b[521])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[508]) && s.b[517]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[522] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.store_scalar(522, if s.b[522] { 1.0 } else { 0.0 });

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
        s.store_scalar(523, if s.b[523] { 1.0 } else { 0.0 });

        s.b[524] = (p.p156 != 0.0);
        s.store_scalar(524, if s.b[524] { 1.0 } else { 0.0 });

        if (s.b[523] && s.b[524]) {
            s.store_voltage(73, ctx, nodes, Some(18), Some(17));
        }

        s.b[525] = (p.p156 == 1.0);
        s.store_scalar(525, if s.b[525] { 1.0 } else { 0.0 });

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
        s.store_scalar(526, if s.b[526] { 1.0 } else { 0.0 });

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
        }

        if (s.b[523] && s.b[524]) {
            let assign24980_ad_e39227: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, assign24980_ad_e39227, (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(assign24980_ad_e39227, ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
        }

        if (s.b[523] && s.b[524]) {
            s.store_div_scaled_inputs_indices(136, 294, 1.0, 83, 2.0);
        }

        s.b[527] = (s.v[136] < 200.0);
        s.store_scalar(527, if s.b[527] { 1.0 } else { 0.0 });

        if ((s.b[523] && s.b[524]) && s.b[527]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(294), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[523] && s.b[524]) && (!s.b[527])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(294), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[523] && s.b[524]) {
            s.store_sub_div_rhs_indices(100, 294, 153, 99);
        }

        s.b[528] = ((((s.v[100] - s.v[294])) as f64).abs() > 1e-19);
        s.store_scalar(528, if s.b[528] { 1.0 } else { 0.0 });

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
        }

        if (s.b[523] && s.b[524]) {
            let assign25600_ad_e40256: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, assign25600_ad_e40256, (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(assign25600_ad_e40256, ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
        }

        if (s.b[523] && s.b[524]) {
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[529] = (s.v[136] < 200.0);
        s.store_scalar(529, if s.b[529] { 1.0 } else { 0.0 });

        if ((s.b[523] && s.b[524]) && s.b[529]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[523] && s.b[524]) && (!s.b[529])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[523] && s.b[524]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[530] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.store_scalar(530, if s.b[530] { 1.0 } else { 0.0 });

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
        }

    }

    pub(super) fn stamp_reactive_block_22(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[523] && s.b[524]) && s.b[530]) {
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
        s.store_scalar(531, if s.b[531] { 1.0 } else { 0.0 });

        if ((s.b[523] && s.b[524]) && s.b[531]) {
            s.store_sub_scaled_inputs(301, 300, (-1.0), 301, 1.0);
        }

        if (s.b[523] && (!s.b[524])) {
            s.store_scalar(300, 0.0);
            s.store_scalar(301, 0.0);
        }

        s.b[532] = (p.p156 != 0.0);
        s.store_scalar(532, if s.b[532] { 1.0 } else { 0.0 });

        s.b[533] = (p.p156 == 1.0);
        s.store_scalar(533, if s.b[533] { 1.0 } else { 0.0 });

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
        }

        if ((!s.b[523]) && s.b[532]) {
            let assign26520_ad_e41654: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, assign26520_ad_e41654, (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(assign26520_ad_e41654, ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
        }

        if ((!s.b[523]) && s.b[532]) {
            s.store_div_scaled_inputs_indices(136, 294, 1.0, 83, 2.0);
        }

        s.b[534] = (s.v[136] < 200.0);
        s.store_scalar(534, if s.b[534] { 1.0 } else { 0.0 });

        if (((!s.b[523]) && s.b[532]) && s.b[534]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(294), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[523]) && s.b[532]) && (!s.b[534])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(294), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[523]) && s.b[532]) {
            s.store_sub_div_rhs_indices(100, 294, 153, 99);
        }

        s.b[535] = ((((s.v[100] - s.v[294])) as f64).abs() > 1e-19);
        s.store_scalar(535, if s.b[535] { 1.0 } else { 0.0 });

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
        }

        if ((!s.b[523]) && s.b[532]) {
            let assign27150_ad_e42750: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, assign27150_ad_e42750, (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(assign27150_ad_e42750, ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
        }

        if ((!s.b[523]) && s.b[532]) {
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[536] = (s.v[136] < 200.0);
        s.store_scalar(536, if s.b[536] { 1.0 } else { 0.0 });

        if (((!s.b[523]) && s.b[532]) && s.b[536]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[523]) && s.b[532]) && (!s.b[536])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[523]) && s.b[532]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[537] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.store_scalar(537, if s.b[537] { 1.0 } else { 0.0 });

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
        s.store_scalar(538, if s.b[538] { 1.0 } else { 0.0 });

        s.b[539] = (p.p157 != 0.0);
        s.store_scalar(539, if s.b[539] { 1.0 } else { 0.0 });

        if (s.b[538] && s.b[539]) {
            s.store_voltage(77, ctx, nodes, Some(21), Some(22));
        }

        s.b[540] = (p.p157 == 1.0);
        s.store_scalar(540, if s.b[540] { 1.0 } else { 0.0 });

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
        s.store_scalar(541, if s.b[541] { 1.0 } else { 0.0 });

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
        }

        if (s.b[538] && s.b[539]) {
            let assign28070_ad_e44156: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, assign28070_ad_e44156, (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(assign28070_ad_e44156, ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
        }

        if (s.b[538] && s.b[539]) {
            s.store_div_scaled_inputs_indices(136, 306, 1.0, 83, 2.0);
        }

        s.b[542] = (s.v[136] < 200.0);
        s.store_scalar(542, if s.b[542] { 1.0 } else { 0.0 });

        if ((s.b[538] && s.b[539]) && s.b[542]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[538] && s.b[539]) && (!s.b[542])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[538] && s.b[539]) {
            s.store_sub_div_rhs_indices(100, 306, 153, 99);
        }

        s.b[543] = ((((s.v[100] - s.v[306])) as f64).abs() > 1e-19);
        s.store_scalar(543, if s.b[543] { 1.0 } else { 0.0 });

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
        }

        if (s.b[538] && s.b[539]) {
            let assign28690_ad_e45185: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, assign28690_ad_e45185, (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(assign28690_ad_e45185, ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
        }

        if (s.b[538] && s.b[539]) {
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[544] = (s.v[136] < 200.0);
        s.store_scalar(544, if s.b[544] { 1.0 } else { 0.0 });

        if ((s.b[538] && s.b[539]) && s.b[544]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[538] && s.b[539]) && (!s.b[544])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[538] && s.b[539]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[545] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.store_scalar(545, if s.b[545] { 1.0 } else { 0.0 });

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
        }

    }

    pub(super) fn stamp_reactive_block_25(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[538] && s.b[539]) && s.b[545]) {
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
        s.store_scalar(546, if s.b[546] { 1.0 } else { 0.0 });

        if ((s.b[538] && s.b[539]) && s.b[546]) {
            s.store_sub_scaled_inputs(313, 312, (-1.0), 313, 1.0);
        }

        if (s.b[538] && (!s.b[539])) {
            s.store_scalar(312, 0.0);
            s.store_scalar(313, 0.0);
        }

        s.b[547] = (p.p157 != 0.0);
        s.store_scalar(547, if s.b[547] { 1.0 } else { 0.0 });

        s.b[548] = (p.p157 == 1.0);
        s.store_scalar(548, if s.b[548] { 1.0 } else { 0.0 });

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
        }

        if ((!s.b[538]) && s.b[547]) {
            let assign29610_ad_e46583: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, assign29610_ad_e46583, (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(assign29610_ad_e46583, ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
        }

        if ((!s.b[538]) && s.b[547]) {
            s.store_div_scaled_inputs_indices(136, 306, 1.0, 83, 2.0);
        }

        s.b[549] = (s.v[136] < 200.0);
        s.store_scalar(549, if s.b[549] { 1.0 } else { 0.0 });

        if (((!s.b[538]) && s.b[547]) && s.b[549]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[538]) && s.b[547]) && (!s.b[549])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[538]) && s.b[547]) {
            s.store_sub_div_rhs_indices(100, 306, 153, 99);
        }

        s.b[550] = ((((s.v[100] - s.v[306])) as f64).abs() > 1e-19);
        s.store_scalar(550, if s.b[550] { 1.0 } else { 0.0 });

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
        }

        if ((!s.b[538]) && s.b[547]) {
            let assign30240_ad_e47679: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, assign30240_ad_e47679, (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(assign30240_ad_e47679, ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
        }

        if ((!s.b[538]) && s.b[547]) {
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[551] = (s.v[136] < 200.0);
        s.store_scalar(551, if s.b[551] { 1.0 } else { 0.0 });

        if (((!s.b[538]) && s.b[547]) && s.b[551]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[538]) && s.b[547]) && (!s.b[551])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[538]) && s.b[547]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[552] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.store_scalar(552, if s.b[552] { 1.0 } else { 0.0 });

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
        s.store_scalar(558, if s.b[558] { 1.0 } else { 0.0 });

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
        s.store_scalar(569, if s.b[569] { 1.0 } else { 0.0 });

        s.b[570] = (p.p149 == 0.0);
        s.store_scalar(570, if s.b[570] { 1.0 } else { 0.0 });

        s.b[571] = (p.p150 != 0.0);
        s.store_scalar(571, if s.b[571] { 1.0 } else { 0.0 });

        s.b[572] = (p.p150 == 1.0);
        s.store_scalar(572, if s.b[572] { 1.0 } else { 0.0 });

        s.b[573] = (p.p150 != 0.0);
        s.store_scalar(573, if s.b[573] { 1.0 } else { 0.0 });

        s.b[574] = (p.p150 == 1.0);
        s.store_scalar(574, if s.b[574] { 1.0 } else { 0.0 });

        s.b[575] = (p.p149 == 0.0);
        s.store_scalar(575, if s.b[575] { 1.0 } else { 0.0 });

        s.b[576] = (p.p151 != 0.0);
        s.store_scalar(576, if s.b[576] { 1.0 } else { 0.0 });

        s.b[577] = (p.p151 == 1.0);
        s.store_scalar(577, if s.b[577] { 1.0 } else { 0.0 });

        s.b[578] = (p.p151 != 0.0);
        s.store_scalar(578, if s.b[578] { 1.0 } else { 0.0 });

        s.b[579] = (p.p151 == 1.0);
        s.store_scalar(579, if s.b[579] { 1.0 } else { 0.0 });

        s.b[580] = (p.p149 == 0.0);
        s.store_scalar(580, if s.b[580] { 1.0 } else { 0.0 });

        s.b[581] = (p.p152 != 0.0);
        s.store_scalar(581, if s.b[581] { 1.0 } else { 0.0 });

        s.b[582] = (p.p152 == 1.0);
        s.store_scalar(582, if s.b[582] { 1.0 } else { 0.0 });

        s.b[583] = (p.p152 != 0.0);
        s.store_scalar(583, if s.b[583] { 1.0 } else { 0.0 });

        s.b[584] = (p.p152 == 1.0);
        s.store_scalar(584, if s.b[584] { 1.0 } else { 0.0 });

        s.b[585] = (p.p149 == 0.0);
        s.store_scalar(585, if s.b[585] { 1.0 } else { 0.0 });

        s.b[586] = (p.p153 != 0.0);
        s.store_scalar(586, if s.b[586] { 1.0 } else { 0.0 });

        s.b[587] = (p.p153 == 1.0);
        s.store_scalar(587, if s.b[587] { 1.0 } else { 0.0 });

        s.b[588] = (p.p153 != 0.0);
        s.store_scalar(588, if s.b[588] { 1.0 } else { 0.0 });

        s.b[589] = (p.p153 == 1.0);
        s.store_scalar(589, if s.b[589] { 1.0 } else { 0.0 });

        s.b[590] = (p.p149 == 0.0);
        s.store_scalar(590, if s.b[590] { 1.0 } else { 0.0 });

        s.b[591] = (p.p154 != 0.0);
        s.store_scalar(591, if s.b[591] { 1.0 } else { 0.0 });

        s.b[592] = (p.p154 == 1.0);
        s.store_scalar(592, if s.b[592] { 1.0 } else { 0.0 });

        s.b[593] = (p.p154 != 0.0);
        s.store_scalar(593, if s.b[593] { 1.0 } else { 0.0 });

        s.b[594] = (p.p154 == 1.0);
        s.store_scalar(594, if s.b[594] { 1.0 } else { 0.0 });

        s.b[595] = (p.p149 == 0.0);
        s.store_scalar(595, if s.b[595] { 1.0 } else { 0.0 });

        s.b[596] = (p.p155 != 0.0);
        s.store_scalar(596, if s.b[596] { 1.0 } else { 0.0 });

        s.b[597] = (p.p155 == 1.0);
        s.store_scalar(597, if s.b[597] { 1.0 } else { 0.0 });

        s.b[598] = (p.p155 != 0.0);
        s.store_scalar(598, if s.b[598] { 1.0 } else { 0.0 });

        s.b[599] = (p.p155 == 1.0);
        s.store_scalar(599, if s.b[599] { 1.0 } else { 0.0 });

        s.b[600] = (p.p149 == 0.0);
        s.store_scalar(600, if s.b[600] { 1.0 } else { 0.0 });

        s.b[601] = (p.p156 != 0.0);
        s.store_scalar(601, if s.b[601] { 1.0 } else { 0.0 });

        s.b[602] = (p.p156 == 1.0);
        s.store_scalar(602, if s.b[602] { 1.0 } else { 0.0 });

        s.b[603] = (p.p156 != 0.0);
        s.store_scalar(603, if s.b[603] { 1.0 } else { 0.0 });

        s.b[604] = (p.p156 == 1.0);
        s.store_scalar(604, if s.b[604] { 1.0 } else { 0.0 });

        s.b[605] = (p.p149 == 0.0);
        s.store_scalar(605, if s.b[605] { 1.0 } else { 0.0 });

        s.b[606] = (p.p157 != 0.0);
        s.store_scalar(606, if s.b[606] { 1.0 } else { 0.0 });

        s.b[607] = (p.p157 == 1.0);
        s.store_scalar(607, if s.b[607] { 1.0 } else { 0.0 });

        s.b[608] = (p.p157 != 0.0);
        s.store_scalar(608, if s.b[608] { 1.0 } else { 0.0 });

        s.b[609] = (p.p157 == 1.0);
        s.store_scalar(609, if s.b[609] { 1.0 } else { 0.0 });

        s.store_sub_from_scalar_ad(195, p.p222, A::mul(A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p227)), (((((-1.0)) * (p.p227))) + (p.p220))), A::voltage(ctx, nodes, Some(0), Some(2))));

        s.store_add_scaled_inputs3_offset_mixed_iia(195, 195, (p.p4 * p.p5), 195, ((-0.5) * (p.p4 * p.p5)), A::sqrt_square_offset(A::offset(s.ad_value(195), (-1e-25)), p.p221), ((-(-0.5)) * (p.p4 * p.p5)), ((1e-25 + ((-0.5) * 1e-25)) * (p.p4 * p.p5)));

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
        s.store_scalar(610, if s.b[610] { 1.0 } else { 0.0 });

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
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_guard353: f64,
        var_guard354: f64,
        var_guard355: f64,
        var_guard356: f64,
        var_guard357: f64,
        var_vdgeff1: f64,
        var_vdgeff1_db0: f64,
        var_vdgeff1_db1: f64,
        var_vdgeff1_db10: f64,
        var_vdgeff1_db11: f64,
        var_vdgeff1_db12: f64,
        var_vdgeff1_db13: f64,
        var_vdgeff1_db14: f64,
        var_vdgeff1_db15: f64,
        var_vdgeff1_db16: f64,
        var_vdgeff1_db17: f64,
        var_vdgeff1_db18: f64,
        var_vdgeff1_db19: f64,
        var_vdgeff1_db2: f64,
        var_vdgeff1_db20: f64,
        var_vdgeff1_db21: f64,
        var_vdgeff1_db22: f64,
        var_vdgeff1_db23: f64,
        var_vdgeff1_db24: f64,
        var_vdgeff1_db25: f64,
        var_vdgeff1_db26: f64,
        var_vdgeff1_db27: f64,
        var_vdgeff1_db28: f64,
        var_vdgeff1_db29: f64,
        var_vdgeff1_db3: f64,
        var_vdgeff1_db30: f64,
        var_vdgeff1_db31: f64,
        var_vdgeff1_db32: f64,
        var_vdgeff1_db33: f64,
        var_vdgeff1_db34: f64,
        var_vdgeff1_db35: f64,
        var_vdgeff1_db36: f64,
        var_vdgeff1_db37: f64,
        var_vdgeff1_db38: f64,
        var_vdgeff1_db39: f64,
        var_vdgeff1_db4: f64,
        var_vdgeff1_db40: f64,
        var_vdgeff1_db41: f64,
        var_vdgeff1_db42: f64,
        var_vdgeff1_db43: f64,
        var_vdgeff1_db44: f64,
        var_vdgeff1_db45: f64,
        var_vdgeff1_db46: f64,
        var_vdgeff1_db47: f64,
        var_vdgeff1_db48: f64,
        var_vdgeff1_db49: f64,
        var_vdgeff1_db5: f64,
        var_vdgeff1_db50: f64,
        var_vdgeff1_db51: f64,
        var_vdgeff1_db52: f64,
        var_vdgeff1_db53: f64,
        var_vdgeff1_db54: f64,
        var_vdgeff1_db6: f64,
        var_vdgeff1_db7: f64,
        var_vdgeff1_db8: f64,
        var_vdgeff1_db9: f64,
        var_vdgeff1_dn0: f64,
        var_vdgeff1_dn1: f64,
        var_vdgeff1_dn10: f64,
        var_vdgeff1_dn11: f64,
        var_vdgeff1_dn12: f64,
        var_vdgeff1_dn13: f64,
        var_vdgeff1_dn14: f64,
        var_vdgeff1_dn15: f64,
        var_vdgeff1_dn16: f64,
        var_vdgeff1_dn17: f64,
        var_vdgeff1_dn18: f64,
        var_vdgeff1_dn19: f64,
        var_vdgeff1_dn2: f64,
        var_vdgeff1_dn20: f64,
        var_vdgeff1_dn21: f64,
        var_vdgeff1_dn22: f64,
        var_vdgeff1_dn3: f64,
        var_vdgeff1_dn4: f64,
        var_vdgeff1_dn5: f64,
        var_vdgeff1_dn6: f64,
        var_vdgeff1_dn7: f64,
        var_vdgeff1_dn8: f64,
        var_vdgeff1_dn9: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq8_e345, eq8_e345_d_n5, eq8_e345_d_n6,) = {
    if ((var_guard354 != 0.0) && (var_guard353 == 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / 10.0;
        let eq8_e339: f64 = ((nv6 - nv5) * __rspice_inv_cse_0);
        let eq8_e339_d_n5: f64 = ((-1.0) * __rspice_inv_cse_0);
        let eq8_e339_d_n6: f64 = (1.0 * __rspice_inv_cse_0);
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
    if ((var_guard354 != 0.0) && (var_guard353 == 0.0)) {
        let eq9_e352: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (nv5 - 0.0));
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
    if ((var_guard355 != 0.0) && (!((var_guard353 != 0.0) || (var_guard354 != 0.0)))) {
        let eq16_e411: f64 = (-1.0);
        let eq16_e413: f64 = (eq16_e411 * var_vdgeff1);
        let eq16_e413_d_n0: f64 = (eq16_e411 * var_vdgeff1_dn0);
        let eq16_e413_d_n1: f64 = (eq16_e411 * var_vdgeff1_dn1);
        let eq16_e413_d_n2: f64 = (eq16_e411 * var_vdgeff1_dn2);
        let eq16_e413_d_n3: f64 = (eq16_e411 * var_vdgeff1_dn3);
        let eq16_e413_d_n4: f64 = (eq16_e411 * var_vdgeff1_dn4);
        let eq16_e413_d_n5: f64 = (eq16_e411 * var_vdgeff1_dn5);
        let eq16_e413_d_n6: f64 = (eq16_e411 * var_vdgeff1_dn6);
        let eq16_e413_d_n7: f64 = (eq16_e411 * var_vdgeff1_dn7);
        let eq16_e413_d_n8: f64 = (eq16_e411 * var_vdgeff1_dn8);
        let eq16_e413_d_n9: f64 = (eq16_e411 * var_vdgeff1_dn9);
        let eq16_e413_d_n10: f64 = (eq16_e411 * var_vdgeff1_dn10);
        let eq16_e413_d_n11: f64 = (eq16_e411 * var_vdgeff1_dn11);
        let eq16_e413_d_n12: f64 = (eq16_e411 * var_vdgeff1_dn12);
        let eq16_e413_d_n13: f64 = (eq16_e411 * var_vdgeff1_dn13);
        let eq16_e413_d_n14: f64 = (eq16_e411 * var_vdgeff1_dn14);
        let eq16_e413_d_n15: f64 = (eq16_e411 * var_vdgeff1_dn15);
        let eq16_e413_d_n16: f64 = (eq16_e411 * var_vdgeff1_dn16);
        let eq16_e413_d_n17: f64 = (eq16_e411 * var_vdgeff1_dn17);
        let eq16_e413_d_n18: f64 = (eq16_e411 * var_vdgeff1_dn18);
        let eq16_e413_d_n19: f64 = (eq16_e411 * var_vdgeff1_dn19);
        let eq16_e413_d_n20: f64 = (eq16_e411 * var_vdgeff1_dn20);
        let eq16_e413_d_n21: f64 = (eq16_e411 * var_vdgeff1_dn21);
        let eq16_e413_d_n22: f64 = (eq16_e411 * var_vdgeff1_dn22);
        let eq16_e413_d_b0: f64 = (eq16_e411 * var_vdgeff1_db0);
        let eq16_e413_d_b1: f64 = (eq16_e411 * var_vdgeff1_db1);
        let eq16_e413_d_b2: f64 = (eq16_e411 * var_vdgeff1_db2);
        let eq16_e413_d_b3: f64 = (eq16_e411 * var_vdgeff1_db3);
        let eq16_e413_d_b4: f64 = (eq16_e411 * var_vdgeff1_db4);
        let eq16_e413_d_b5: f64 = (eq16_e411 * var_vdgeff1_db5);
        let eq16_e413_d_b6: f64 = (eq16_e411 * var_vdgeff1_db6);
        let eq16_e413_d_b7: f64 = (eq16_e411 * var_vdgeff1_db7);
        let eq16_e413_d_b8: f64 = (eq16_e411 * var_vdgeff1_db8);
        let eq16_e413_d_b9: f64 = (eq16_e411 * var_vdgeff1_db9);
        let eq16_e413_d_b10: f64 = (eq16_e411 * var_vdgeff1_db10);
        let eq16_e413_d_b11: f64 = (eq16_e411 * var_vdgeff1_db11);
        let eq16_e413_d_b12: f64 = (eq16_e411 * var_vdgeff1_db12);
        let eq16_e413_d_b13: f64 = (eq16_e411 * var_vdgeff1_db13);
        let eq16_e413_d_b14: f64 = (eq16_e411 * var_vdgeff1_db14);
        let eq16_e413_d_b15: f64 = (eq16_e411 * var_vdgeff1_db15);
        let eq16_e413_d_b16: f64 = (eq16_e411 * var_vdgeff1_db16);
        let eq16_e413_d_b17: f64 = (eq16_e411 * var_vdgeff1_db17);
        let eq16_e413_d_b18: f64 = (eq16_e411 * var_vdgeff1_db18);
        let eq16_e413_d_b19: f64 = (eq16_e411 * var_vdgeff1_db19);
        let eq16_e413_d_b20: f64 = (eq16_e411 * var_vdgeff1_db20);
        let eq16_e413_d_b21: f64 = (eq16_e411 * var_vdgeff1_db21);
        let eq16_e413_d_b22: f64 = (eq16_e411 * var_vdgeff1_db22);
        let eq16_e413_d_b23: f64 = (eq16_e411 * var_vdgeff1_db23);
        let eq16_e413_d_b24: f64 = (eq16_e411 * var_vdgeff1_db24);
        let eq16_e413_d_b25: f64 = (eq16_e411 * var_vdgeff1_db25);
        let eq16_e413_d_b26: f64 = (eq16_e411 * var_vdgeff1_db26);
        let eq16_e413_d_b27: f64 = (eq16_e411 * var_vdgeff1_db27);
        let eq16_e413_d_b28: f64 = (eq16_e411 * var_vdgeff1_db28);
        let eq16_e413_d_b29: f64 = (eq16_e411 * var_vdgeff1_db29);
        let eq16_e413_d_b30: f64 = (eq16_e411 * var_vdgeff1_db30);
        let eq16_e413_d_b31: f64 = (eq16_e411 * var_vdgeff1_db31);
        let eq16_e413_d_b32: f64 = (eq16_e411 * var_vdgeff1_db32);
        let eq16_e413_d_b33: f64 = (eq16_e411 * var_vdgeff1_db33);
        let eq16_e413_d_b34: f64 = (eq16_e411 * var_vdgeff1_db34);
        let eq16_e413_d_b35: f64 = (eq16_e411 * var_vdgeff1_db35);
        let eq16_e413_d_b36: f64 = (eq16_e411 * var_vdgeff1_db36);
        let eq16_e413_d_b37: f64 = (eq16_e411 * var_vdgeff1_db37);
        let eq16_e413_d_b38: f64 = (eq16_e411 * var_vdgeff1_db38);
        let eq16_e413_d_b39: f64 = (eq16_e411 * var_vdgeff1_db39);
        let eq16_e413_d_b40: f64 = (eq16_e411 * var_vdgeff1_db40);
        let eq16_e413_d_b41: f64 = (eq16_e411 * var_vdgeff1_db41);
        let eq16_e413_d_b42: f64 = (eq16_e411 * var_vdgeff1_db42);
        let eq16_e413_d_b43: f64 = (eq16_e411 * var_vdgeff1_db43);
        let eq16_e413_d_b44: f64 = (eq16_e411 * var_vdgeff1_db44);
        let eq16_e413_d_b45: f64 = (eq16_e411 * var_vdgeff1_db45);
        let eq16_e413_d_b46: f64 = (eq16_e411 * var_vdgeff1_db46);
        let eq16_e413_d_b47: f64 = (eq16_e411 * var_vdgeff1_db47);
        let eq16_e413_d_b48: f64 = (eq16_e411 * var_vdgeff1_db48);
        let eq16_e413_d_b49: f64 = (eq16_e411 * var_vdgeff1_db49);
        let eq16_e413_d_b50: f64 = (eq16_e411 * var_vdgeff1_db50);
        let eq16_e413_d_b51: f64 = (eq16_e411 * var_vdgeff1_db51);
        let eq16_e413_d_b52: f64 = (eq16_e411 * var_vdgeff1_db52);
        let eq16_e413_d_b53: f64 = (eq16_e411 * var_vdgeff1_db53);
        let eq16_e413_d_b54: f64 = (eq16_e411 * var_vdgeff1_db54);
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
    if ((var_guard355 != 0.0) && (!((var_guard353 != 0.0) || (var_guard354 != 0.0)))) {
        let eq17_e424: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, (nv5 - 0.0));
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
    if ((var_guard355 != 0.0) && (!((var_guard353 != 0.0) || (var_guard354 != 0.0)))) {
        let eq20_e459: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, (nv6 - 0.0));
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
    if ((var_guard356 != 0.0) && (!(((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)))) {
        let eq27_e536: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, (nv5 - 0.0));
        let eq27_e537: f64 = (s.v[149] * eq27_e536);
        let eq27_e537_d_n5: f64 = ((s.dn[149][5] * eq27_e536) + (s.v[149] * ddt_scale));
        (eq27_e537, (s.dn[149][0] * eq27_e536), (s.dn[149][1] * eq27_e536), (s.dn[149][2] * eq27_e536), (s.dn[149][3] * eq27_e536), (s.dn[149][4] * eq27_e536), eq27_e537_d_n5, (s.dn[149][6] * eq27_e536), (s.dn[149][7] * eq27_e536), (s.dn[149][8] * eq27_e536), (s.dn[149][9] * eq27_e536), (s.dn[149][10] * eq27_e536), (s.dn[149][11] * eq27_e536), (s.dn[149][12] * eq27_e536), (s.dn[149][13] * eq27_e536), (s.dn[149][14] * eq27_e536), (s.dn[149][15] * eq27_e536), (s.dn[149][16] * eq27_e536), (s.dn[149][17] * eq27_e536), (s.dn[149][18] * eq27_e536), (s.dn[149][19] * eq27_e536), (s.dn[149][20] * eq27_e536), (s.dn[149][21] * eq27_e536), (s.dn[149][22] * eq27_e536), (s.db[149][0] * eq27_e536), (s.db[149][1] * eq27_e536), (s.db[149][2] * eq27_e536), (s.db[149][3] * eq27_e536), (s.db[149][4] * eq27_e536), (s.db[149][5] * eq27_e536), (s.db[149][6] * eq27_e536), (s.db[149][7] * eq27_e536), (s.db[149][8] * eq27_e536), (s.db[149][9] * eq27_e536), (s.db[149][10] * eq27_e536), (s.db[149][11] * eq27_e536), (s.db[149][12] * eq27_e536), (s.db[149][13] * eq27_e536), (s.db[149][14] * eq27_e536), (s.db[149][15] * eq27_e536), (s.db[149][16] * eq27_e536), (s.db[149][17] * eq27_e536), (s.db[149][18] * eq27_e536), (s.db[149][19] * eq27_e536), (s.db[149][20] * eq27_e536), (s.db[149][21] * eq27_e536), (s.db[149][22] * eq27_e536), (s.db[149][23] * eq27_e536), (s.db[149][24] * eq27_e536), (s.db[149][25] * eq27_e536), (s.db[149][26] * eq27_e536), (s.db[149][27] * eq27_e536), (s.db[149][28] * eq27_e536), (s.db[149][29] * eq27_e536), (s.db[149][30] * eq27_e536), (s.db[149][31] * eq27_e536), (s.db[149][32] * eq27_e536), (s.db[149][33] * eq27_e536), (s.db[149][34] * eq27_e536), (s.db[149][35] * eq27_e536), (s.db[149][36] * eq27_e536), (s.db[149][37] * eq27_e536), (s.db[149][38] * eq27_e536), (s.db[149][39] * eq27_e536), (s.db[149][40] * eq27_e536), (s.db[149][41] * eq27_e536), (s.db[149][42] * eq27_e536), (s.db[149][43] * eq27_e536), (s.db[149][44] * eq27_e536), (s.db[149][45] * eq27_e536), (s.db[149][46] * eq27_e536), (s.db[149][47] * eq27_e536), (s.db[149][48] * eq27_e536), (s.db[149][49] * eq27_e536), (s.db[149][50] * eq27_e536), (s.db[149][51] * eq27_e536), (s.db[149][52] * eq27_e536), (s.db[149][53] * eq27_e536), (s.db[149][54] * eq27_e536),)
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
    if ((var_guard357 != 0.0) && (!((((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)) || (var_guard356 != 0.0)))) {
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
    if ((var_guard357 != 0.0) && (!((((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)) || (var_guard356 != 0.0)))) {
        let eq37_e661: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, (nv12 - 0.0));
        let eq37_e662: f64 = (p.p97 * eq37_e661);
        let eq37_e665: f64 = (1e-12 * (nv12 - 0.0));
        let eq37_e666: f64 = (eq37_e662 + eq37_e665);
        let eq37_e666_d_n12: f64 = ((p.p97 * ddt_scale) + 1e-12);
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
    if ((var_guard357 != 0.0) && (!((((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)) || (var_guard356 != 0.0)))) {
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
    if ((var_guard357 != 0.0) && (!((((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)) || (var_guard356 != 0.0)))) {
        let eq40_e709: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, (nv14 - 0.0));
        let eq40_e710: f64 = (p.p83 * eq40_e709);
        let eq40_e713: f64 = (1e-12 * (nv14 - 0.0));
        let eq40_e714: f64 = (eq40_e710 + eq40_e713);
        let eq40_e714_d_n14: f64 = ((p.p83 * ddt_scale) + 1e-12);
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
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        var_en: f64,
        var_en_db0: f64,
        var_en_db1: f64,
        var_en_db10: f64,
        var_en_db11: f64,
        var_en_db12: f64,
        var_en_db13: f64,
        var_en_db14: f64,
        var_en_db15: f64,
        var_en_db16: f64,
        var_en_db17: f64,
        var_en_db18: f64,
        var_en_db19: f64,
        var_en_db2: f64,
        var_en_db20: f64,
        var_en_db21: f64,
        var_en_db22: f64,
        var_en_db23: f64,
        var_en_db24: f64,
        var_en_db25: f64,
        var_en_db26: f64,
        var_en_db27: f64,
        var_en_db28: f64,
        var_en_db29: f64,
        var_en_db3: f64,
        var_en_db30: f64,
        var_en_db31: f64,
        var_en_db32: f64,
        var_en_db33: f64,
        var_en_db34: f64,
        var_en_db35: f64,
        var_en_db36: f64,
        var_en_db37: f64,
        var_en_db38: f64,
        var_en_db39: f64,
        var_en_db4: f64,
        var_en_db40: f64,
        var_en_db41: f64,
        var_en_db42: f64,
        var_en_db43: f64,
        var_en_db44: f64,
        var_en_db45: f64,
        var_en_db46: f64,
        var_en_db47: f64,
        var_en_db48: f64,
        var_en_db49: f64,
        var_en_db5: f64,
        var_en_db50: f64,
        var_en_db51: f64,
        var_en_db52: f64,
        var_en_db53: f64,
        var_en_db54: f64,
        var_en_db6: f64,
        var_en_db7: f64,
        var_en_db8: f64,
        var_en_db9: f64,
        var_en_dn0: f64,
        var_en_dn1: f64,
        var_en_dn10: f64,
        var_en_dn11: f64,
        var_en_dn12: f64,
        var_en_dn13: f64,
        var_en_dn14: f64,
        var_en_dn15: f64,
        var_en_dn16: f64,
        var_en_dn17: f64,
        var_en_dn18: f64,
        var_en_dn19: f64,
        var_en_dn2: f64,
        var_en_dn20: f64,
        var_en_dn21: f64,
        var_en_dn22: f64,
        var_en_dn3: f64,
        var_en_dn4: f64,
        var_en_dn5: f64,
        var_en_dn6: f64,
        var_en_dn7: f64,
        var_en_dn8: f64,
        var_en_dn9: f64,
        var_guard353: f64,
        var_guard354: f64,
        var_guard355: f64,
        var_guard356: f64,
        var_guard357: f64,
        var_guard358: f64,
        var_phixn: f64,
        var_phixn_db0: f64,
        var_phixn_db1: f64,
        var_phixn_db10: f64,
        var_phixn_db11: f64,
        var_phixn_db12: f64,
        var_phixn_db13: f64,
        var_phixn_db14: f64,
        var_phixn_db15: f64,
        var_phixn_db16: f64,
        var_phixn_db17: f64,
        var_phixn_db18: f64,
        var_phixn_db19: f64,
        var_phixn_db2: f64,
        var_phixn_db20: f64,
        var_phixn_db21: f64,
        var_phixn_db22: f64,
        var_phixn_db23: f64,
        var_phixn_db24: f64,
        var_phixn_db25: f64,
        var_phixn_db26: f64,
        var_phixn_db27: f64,
        var_phixn_db28: f64,
        var_phixn_db29: f64,
        var_phixn_db3: f64,
        var_phixn_db30: f64,
        var_phixn_db31: f64,
        var_phixn_db32: f64,
        var_phixn_db33: f64,
        var_phixn_db34: f64,
        var_phixn_db35: f64,
        var_phixn_db36: f64,
        var_phixn_db37: f64,
        var_phixn_db38: f64,
        var_phixn_db39: f64,
        var_phixn_db4: f64,
        var_phixn_db40: f64,
        var_phixn_db41: f64,
        var_phixn_db42: f64,
        var_phixn_db43: f64,
        var_phixn_db44: f64,
        var_phixn_db45: f64,
        var_phixn_db46: f64,
        var_phixn_db47: f64,
        var_phixn_db48: f64,
        var_phixn_db49: f64,
        var_phixn_db5: f64,
        var_phixn_db50: f64,
        var_phixn_db51: f64,
        var_phixn_db52: f64,
        var_phixn_db53: f64,
        var_phixn_db54: f64,
        var_phixn_db6: f64,
        var_phixn_db7: f64,
        var_phixn_db8: f64,
        var_phixn_db9: f64,
        var_phixn_dn0: f64,
        var_phixn_dn1: f64,
        var_phixn_dn10: f64,
        var_phixn_dn11: f64,
        var_phixn_dn12: f64,
        var_phixn_dn13: f64,
        var_phixn_dn14: f64,
        var_phixn_dn15: f64,
        var_phixn_dn16: f64,
        var_phixn_dn17: f64,
        var_phixn_dn18: f64,
        var_phixn_dn19: f64,
        var_phixn_dn2: f64,
        var_phixn_dn20: f64,
        var_phixn_dn21: f64,
        var_phixn_dn22: f64,
        var_phixn_dn3: f64,
        var_phixn_dn4: f64,
        var_phixn_dn5: f64,
        var_phixn_dn6: f64,
        var_phixn_dn7: f64,
        var_phixn_dn8: f64,
        var_phixn_dn9: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq41_e747, eq41_e747_d_n0, eq41_e747_d_n1, eq41_e747_d_n2, eq41_e747_d_n3, eq41_e747_d_n4, eq41_e747_d_n5, eq41_e747_d_n6, eq41_e747_d_n7, eq41_e747_d_n8, eq41_e747_d_n9, eq41_e747_d_n10, eq41_e747_d_n11, eq41_e747_d_n12, eq41_e747_d_n13, eq41_e747_d_n14, eq41_e747_d_n15, eq41_e747_d_n16, eq41_e747_d_n17, eq41_e747_d_n18, eq41_e747_d_n19, eq41_e747_d_n20, eq41_e747_d_n21, eq41_e747_d_n22, eq41_e747_d_b0, eq41_e747_d_b1, eq41_e747_d_b2, eq41_e747_d_b3, eq41_e747_d_b4, eq41_e747_d_b5, eq41_e747_d_b6, eq41_e747_d_b7, eq41_e747_d_b8, eq41_e747_d_b9, eq41_e747_d_b10, eq41_e747_d_b11, eq41_e747_d_b12, eq41_e747_d_b13, eq41_e747_d_b14, eq41_e747_d_b15, eq41_e747_d_b16, eq41_e747_d_b17, eq41_e747_d_b18, eq41_e747_d_b19, eq41_e747_d_b20, eq41_e747_d_b21, eq41_e747_d_b22, eq41_e747_d_b23, eq41_e747_d_b24, eq41_e747_d_b25, eq41_e747_d_b26, eq41_e747_d_b27, eq41_e747_d_b28, eq41_e747_d_b29, eq41_e747_d_b30, eq41_e747_d_b31, eq41_e747_d_b32, eq41_e747_d_b33, eq41_e747_d_b34, eq41_e747_d_b35, eq41_e747_d_b36, eq41_e747_d_b37, eq41_e747_d_b38, eq41_e747_d_b39, eq41_e747_d_b40, eq41_e747_d_b41, eq41_e747_d_b42, eq41_e747_d_b43, eq41_e747_d_b44, eq41_e747_d_b45, eq41_e747_d_b46, eq41_e747_d_b47, eq41_e747_d_b48, eq41_e747_d_b49, eq41_e747_d_b50, eq41_e747_d_b51, eq41_e747_d_b52, eq41_e747_d_b53, eq41_e747_d_b54,) = {
    if ((var_guard358 != 0.0) && (!(((((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)) || (var_guard356 != 0.0)) || (var_guard357 != 0.0)))) {
        let eq41_e730: f64 = (-p.p135);
        let eq41_e732: f64 = (eq41_e730 * var_en);
        let eq41_e732_d_n0: f64 = (eq41_e730 * var_en_dn0);
        let eq41_e732_d_n1: f64 = (eq41_e730 * var_en_dn1);
        let eq41_e732_d_n2: f64 = (eq41_e730 * var_en_dn2);
        let eq41_e732_d_n3: f64 = (eq41_e730 * var_en_dn3);
        let eq41_e732_d_n4: f64 = (eq41_e730 * var_en_dn4);
        let eq41_e732_d_n5: f64 = (eq41_e730 * var_en_dn5);
        let eq41_e732_d_n6: f64 = (eq41_e730 * var_en_dn6);
        let eq41_e732_d_n7: f64 = (eq41_e730 * var_en_dn7);
        let eq41_e732_d_n8: f64 = (eq41_e730 * var_en_dn8);
        let eq41_e732_d_n9: f64 = (eq41_e730 * var_en_dn9);
        let eq41_e732_d_n10: f64 = (eq41_e730 * var_en_dn10);
        let eq41_e732_d_n11: f64 = (eq41_e730 * var_en_dn11);
        let eq41_e732_d_n12: f64 = (eq41_e730 * var_en_dn12);
        let eq41_e732_d_n13: f64 = (eq41_e730 * var_en_dn13);
        let eq41_e732_d_n14: f64 = (eq41_e730 * var_en_dn14);
        let eq41_e732_d_n15: f64 = (eq41_e730 * var_en_dn15);
        let eq41_e732_d_n16: f64 = (eq41_e730 * var_en_dn16);
        let eq41_e732_d_n17: f64 = (eq41_e730 * var_en_dn17);
        let eq41_e732_d_n18: f64 = (eq41_e730 * var_en_dn18);
        let eq41_e732_d_n19: f64 = (eq41_e730 * var_en_dn19);
        let eq41_e732_d_n20: f64 = (eq41_e730 * var_en_dn20);
        let eq41_e732_d_n21: f64 = (eq41_e730 * var_en_dn21);
        let eq41_e732_d_n22: f64 = (eq41_e730 * var_en_dn22);
        let eq41_e732_d_b0: f64 = (eq41_e730 * var_en_db0);
        let eq41_e732_d_b1: f64 = (eq41_e730 * var_en_db1);
        let eq41_e732_d_b2: f64 = (eq41_e730 * var_en_db2);
        let eq41_e732_d_b3: f64 = (eq41_e730 * var_en_db3);
        let eq41_e732_d_b4: f64 = (eq41_e730 * var_en_db4);
        let eq41_e732_d_b5: f64 = (eq41_e730 * var_en_db5);
        let eq41_e732_d_b6: f64 = (eq41_e730 * var_en_db6);
        let eq41_e732_d_b7: f64 = (eq41_e730 * var_en_db7);
        let eq41_e732_d_b8: f64 = (eq41_e730 * var_en_db8);
        let eq41_e732_d_b9: f64 = (eq41_e730 * var_en_db9);
        let eq41_e732_d_b10: f64 = (eq41_e730 * var_en_db10);
        let eq41_e732_d_b11: f64 = (eq41_e730 * var_en_db11);
        let eq41_e732_d_b12: f64 = (eq41_e730 * var_en_db12);
        let eq41_e732_d_b13: f64 = (eq41_e730 * var_en_db13);
        let eq41_e732_d_b14: f64 = (eq41_e730 * var_en_db14);
        let eq41_e732_d_b15: f64 = (eq41_e730 * var_en_db15);
        let eq41_e732_d_b16: f64 = (eq41_e730 * var_en_db16);
        let eq41_e732_d_b17: f64 = (eq41_e730 * var_en_db17);
        let eq41_e732_d_b18: f64 = (eq41_e730 * var_en_db18);
        let eq41_e732_d_b19: f64 = (eq41_e730 * var_en_db19);
        let eq41_e732_d_b20: f64 = (eq41_e730 * var_en_db20);
        let eq41_e732_d_b21: f64 = (eq41_e730 * var_en_db21);
        let eq41_e732_d_b22: f64 = (eq41_e730 * var_en_db22);
        let eq41_e732_d_b23: f64 = (eq41_e730 * var_en_db23);
        let eq41_e732_d_b24: f64 = (eq41_e730 * var_en_db24);
        let eq41_e732_d_b25: f64 = (eq41_e730 * var_en_db25);
        let eq41_e732_d_b26: f64 = (eq41_e730 * var_en_db26);
        let eq41_e732_d_b27: f64 = (eq41_e730 * var_en_db27);
        let eq41_e732_d_b28: f64 = (eq41_e730 * var_en_db28);
        let eq41_e732_d_b29: f64 = (eq41_e730 * var_en_db29);
        let eq41_e732_d_b30: f64 = (eq41_e730 * var_en_db30);
        let eq41_e732_d_b31: f64 = (eq41_e730 * var_en_db31);
        let eq41_e732_d_b32: f64 = (eq41_e730 * var_en_db32);
        let eq41_e732_d_b33: f64 = (eq41_e730 * var_en_db33);
        let eq41_e732_d_b34: f64 = (eq41_e730 * var_en_db34);
        let eq41_e732_d_b35: f64 = (eq41_e730 * var_en_db35);
        let eq41_e732_d_b36: f64 = (eq41_e730 * var_en_db36);
        let eq41_e732_d_b37: f64 = (eq41_e730 * var_en_db37);
        let eq41_e732_d_b38: f64 = (eq41_e730 * var_en_db38);
        let eq41_e732_d_b39: f64 = (eq41_e730 * var_en_db39);
        let eq41_e732_d_b40: f64 = (eq41_e730 * var_en_db40);
        let eq41_e732_d_b41: f64 = (eq41_e730 * var_en_db41);
        let eq41_e732_d_b42: f64 = (eq41_e730 * var_en_db42);
        let eq41_e732_d_b43: f64 = (eq41_e730 * var_en_db43);
        let eq41_e732_d_b44: f64 = (eq41_e730 * var_en_db44);
        let eq41_e732_d_b45: f64 = (eq41_e730 * var_en_db45);
        let eq41_e732_d_b46: f64 = (eq41_e730 * var_en_db46);
        let eq41_e732_d_b47: f64 = (eq41_e730 * var_en_db47);
        let eq41_e732_d_b48: f64 = (eq41_e730 * var_en_db48);
        let eq41_e732_d_b49: f64 = (eq41_e730 * var_en_db49);
        let eq41_e732_d_b50: f64 = (eq41_e730 * var_en_db50);
        let eq41_e732_d_b51: f64 = (eq41_e730 * var_en_db51);
        let eq41_e732_d_b52: f64 = (eq41_e730 * var_en_db52);
        let eq41_e732_d_b53: f64 = (eq41_e730 * var_en_db53);
        let eq41_e732_d_b54: f64 = (eq41_e730 * var_en_db54);
        let eq41_e735: f64 = (p.p136 - (nv5 - 0.0));
        let eq41_e736: f64 = (eq41_e732 * eq41_e735);
        let eq41_e736_d_n0: f64 = (eq41_e732_d_n0 * eq41_e735);
        let eq41_e736_d_n1: f64 = (eq41_e732_d_n1 * eq41_e735);
        let eq41_e736_d_n2: f64 = (eq41_e732_d_n2 * eq41_e735);
        let eq41_e736_d_n3: f64 = (eq41_e732_d_n3 * eq41_e735);
        let eq41_e736_d_n4: f64 = (eq41_e732_d_n4 * eq41_e735);
        let eq41_e736_d_n5: f64 = ((eq41_e732_d_n5 * eq41_e735) + (eq41_e732 * (-1.0)));
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
        let eq41_e739: f64 = (2.0 * var_phixn);
        let eq41_e739_d_n0: f64 = (2.0 * var_phixn_dn0);
        let eq41_e739_d_n1: f64 = (2.0 * var_phixn_dn1);
        let eq41_e739_d_n2: f64 = (2.0 * var_phixn_dn2);
        let eq41_e739_d_n3: f64 = (2.0 * var_phixn_dn3);
        let eq41_e739_d_n4: f64 = (2.0 * var_phixn_dn4);
        let eq41_e739_d_n5: f64 = (2.0 * var_phixn_dn5);
        let eq41_e739_d_n6: f64 = (2.0 * var_phixn_dn6);
        let eq41_e739_d_n7: f64 = (2.0 * var_phixn_dn7);
        let eq41_e739_d_n8: f64 = (2.0 * var_phixn_dn8);
        let eq41_e739_d_n9: f64 = (2.0 * var_phixn_dn9);
        let eq41_e739_d_n10: f64 = (2.0 * var_phixn_dn10);
        let eq41_e739_d_n11: f64 = (2.0 * var_phixn_dn11);
        let eq41_e739_d_n12: f64 = (2.0 * var_phixn_dn12);
        let eq41_e739_d_n13: f64 = (2.0 * var_phixn_dn13);
        let eq41_e739_d_n14: f64 = (2.0 * var_phixn_dn14);
        let eq41_e739_d_n15: f64 = (2.0 * var_phixn_dn15);
        let eq41_e739_d_n16: f64 = (2.0 * var_phixn_dn16);
        let eq41_e739_d_n17: f64 = (2.0 * var_phixn_dn17);
        let eq41_e739_d_n18: f64 = (2.0 * var_phixn_dn18);
        let eq41_e739_d_n19: f64 = (2.0 * var_phixn_dn19);
        let eq41_e739_d_n20: f64 = (2.0 * var_phixn_dn20);
        let eq41_e739_d_n21: f64 = (2.0 * var_phixn_dn21);
        let eq41_e739_d_n22: f64 = (2.0 * var_phixn_dn22);
        let eq41_e739_d_b0: f64 = (2.0 * var_phixn_db0);
        let eq41_e739_d_b1: f64 = (2.0 * var_phixn_db1);
        let eq41_e739_d_b2: f64 = (2.0 * var_phixn_db2);
        let eq41_e739_d_b3: f64 = (2.0 * var_phixn_db3);
        let eq41_e739_d_b4: f64 = (2.0 * var_phixn_db4);
        let eq41_e739_d_b5: f64 = (2.0 * var_phixn_db5);
        let eq41_e739_d_b6: f64 = (2.0 * var_phixn_db6);
        let eq41_e739_d_b7: f64 = (2.0 * var_phixn_db7);
        let eq41_e739_d_b8: f64 = (2.0 * var_phixn_db8);
        let eq41_e739_d_b9: f64 = (2.0 * var_phixn_db9);
        let eq41_e739_d_b10: f64 = (2.0 * var_phixn_db10);
        let eq41_e739_d_b11: f64 = (2.0 * var_phixn_db11);
        let eq41_e739_d_b12: f64 = (2.0 * var_phixn_db12);
        let eq41_e739_d_b13: f64 = (2.0 * var_phixn_db13);
        let eq41_e739_d_b14: f64 = (2.0 * var_phixn_db14);
        let eq41_e739_d_b15: f64 = (2.0 * var_phixn_db15);
        let eq41_e739_d_b16: f64 = (2.0 * var_phixn_db16);
        let eq41_e739_d_b17: f64 = (2.0 * var_phixn_db17);
        let eq41_e739_d_b18: f64 = (2.0 * var_phixn_db18);
        let eq41_e739_d_b19: f64 = (2.0 * var_phixn_db19);
        let eq41_e739_d_b20: f64 = (2.0 * var_phixn_db20);
        let eq41_e739_d_b21: f64 = (2.0 * var_phixn_db21);
        let eq41_e739_d_b22: f64 = (2.0 * var_phixn_db22);
        let eq41_e739_d_b23: f64 = (2.0 * var_phixn_db23);
        let eq41_e739_d_b24: f64 = (2.0 * var_phixn_db24);
        let eq41_e739_d_b25: f64 = (2.0 * var_phixn_db25);
        let eq41_e739_d_b26: f64 = (2.0 * var_phixn_db26);
        let eq41_e739_d_b27: f64 = (2.0 * var_phixn_db27);
        let eq41_e739_d_b28: f64 = (2.0 * var_phixn_db28);
        let eq41_e739_d_b29: f64 = (2.0 * var_phixn_db29);
        let eq41_e739_d_b30: f64 = (2.0 * var_phixn_db30);
        let eq41_e739_d_b31: f64 = (2.0 * var_phixn_db31);
        let eq41_e739_d_b32: f64 = (2.0 * var_phixn_db32);
        let eq41_e739_d_b33: f64 = (2.0 * var_phixn_db33);
        let eq41_e739_d_b34: f64 = (2.0 * var_phixn_db34);
        let eq41_e739_d_b35: f64 = (2.0 * var_phixn_db35);
        let eq41_e739_d_b36: f64 = (2.0 * var_phixn_db36);
        let eq41_e739_d_b37: f64 = (2.0 * var_phixn_db37);
        let eq41_e739_d_b38: f64 = (2.0 * var_phixn_db38);
        let eq41_e739_d_b39: f64 = (2.0 * var_phixn_db39);
        let eq41_e739_d_b40: f64 = (2.0 * var_phixn_db40);
        let eq41_e739_d_b41: f64 = (2.0 * var_phixn_db41);
        let eq41_e739_d_b42: f64 = (2.0 * var_phixn_db42);
        let eq41_e739_d_b43: f64 = (2.0 * var_phixn_db43);
        let eq41_e739_d_b44: f64 = (2.0 * var_phixn_db44);
        let eq41_e739_d_b45: f64 = (2.0 * var_phixn_db45);
        let eq41_e739_d_b46: f64 = (2.0 * var_phixn_db46);
        let eq41_e739_d_b47: f64 = (2.0 * var_phixn_db47);
        let eq41_e739_d_b48: f64 = (2.0 * var_phixn_db48);
        let eq41_e739_d_b49: f64 = (2.0 * var_phixn_db49);
        let eq41_e739_d_b50: f64 = (2.0 * var_phixn_db50);
        let eq41_e739_d_b51: f64 = (2.0 * var_phixn_db51);
        let eq41_e739_d_b52: f64 = (2.0 * var_phixn_db52);
        let eq41_e739_d_b53: f64 = (2.0 * var_phixn_db53);
        let eq41_e739_d_b54: f64 = (2.0 * var_phixn_db54);
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
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_en: f64,
        var_en_db0: f64,
        var_en_db1: f64,
        var_en_db10: f64,
        var_en_db11: f64,
        var_en_db12: f64,
        var_en_db13: f64,
        var_en_db14: f64,
        var_en_db15: f64,
        var_en_db16: f64,
        var_en_db17: f64,
        var_en_db18: f64,
        var_en_db19: f64,
        var_en_db2: f64,
        var_en_db20: f64,
        var_en_db21: f64,
        var_en_db22: f64,
        var_en_db23: f64,
        var_en_db24: f64,
        var_en_db25: f64,
        var_en_db26: f64,
        var_en_db27: f64,
        var_en_db28: f64,
        var_en_db29: f64,
        var_en_db3: f64,
        var_en_db30: f64,
        var_en_db31: f64,
        var_en_db32: f64,
        var_en_db33: f64,
        var_en_db34: f64,
        var_en_db35: f64,
        var_en_db36: f64,
        var_en_db37: f64,
        var_en_db38: f64,
        var_en_db39: f64,
        var_en_db4: f64,
        var_en_db40: f64,
        var_en_db41: f64,
        var_en_db42: f64,
        var_en_db43: f64,
        var_en_db44: f64,
        var_en_db45: f64,
        var_en_db46: f64,
        var_en_db47: f64,
        var_en_db48: f64,
        var_en_db49: f64,
        var_en_db5: f64,
        var_en_db50: f64,
        var_en_db51: f64,
        var_en_db52: f64,
        var_en_db53: f64,
        var_en_db54: f64,
        var_en_db6: f64,
        var_en_db7: f64,
        var_en_db8: f64,
        var_en_db9: f64,
        var_en_dn0: f64,
        var_en_dn1: f64,
        var_en_dn10: f64,
        var_en_dn11: f64,
        var_en_dn12: f64,
        var_en_dn13: f64,
        var_en_dn14: f64,
        var_en_dn15: f64,
        var_en_dn16: f64,
        var_en_dn17: f64,
        var_en_dn18: f64,
        var_en_dn19: f64,
        var_en_dn2: f64,
        var_en_dn20: f64,
        var_en_dn21: f64,
        var_en_dn22: f64,
        var_en_dn3: f64,
        var_en_dn4: f64,
        var_en_dn5: f64,
        var_en_dn6: f64,
        var_en_dn7: f64,
        var_en_dn8: f64,
        var_en_dn9: f64,
        var_guard353: f64,
        var_guard354: f64,
        var_guard355: f64,
        var_guard356: f64,
        var_guard357: f64,
        var_guard358: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq42_e766, eq42_e766_d_n0, eq42_e766_d_n1, eq42_e766_d_n2, eq42_e766_d_n3, eq42_e766_d_n4, eq42_e766_d_n5, eq42_e766_d_n6, eq42_e766_d_n7, eq42_e766_d_n8, eq42_e766_d_n9, eq42_e766_d_n10, eq42_e766_d_n11, eq42_e766_d_n12, eq42_e766_d_n13, eq42_e766_d_n14, eq42_e766_d_n15, eq42_e766_d_n16, eq42_e766_d_n17, eq42_e766_d_n18, eq42_e766_d_n19, eq42_e766_d_n20, eq42_e766_d_n21, eq42_e766_d_n22, eq42_e766_d_b0, eq42_e766_d_b1, eq42_e766_d_b2, eq42_e766_d_b3, eq42_e766_d_b4, eq42_e766_d_b5, eq42_e766_d_b6, eq42_e766_d_b7, eq42_e766_d_b8, eq42_e766_d_b9, eq42_e766_d_b10, eq42_e766_d_b11, eq42_e766_d_b12, eq42_e766_d_b13, eq42_e766_d_b14, eq42_e766_d_b15, eq42_e766_d_b16, eq42_e766_d_b17, eq42_e766_d_b18, eq42_e766_d_b19, eq42_e766_d_b20, eq42_e766_d_b21, eq42_e766_d_b22, eq42_e766_d_b23, eq42_e766_d_b24, eq42_e766_d_b25, eq42_e766_d_b26, eq42_e766_d_b27, eq42_e766_d_b28, eq42_e766_d_b29, eq42_e766_d_b30, eq42_e766_d_b31, eq42_e766_d_b32, eq42_e766_d_b33, eq42_e766_d_b34, eq42_e766_d_b35, eq42_e766_d_b36, eq42_e766_d_b37, eq42_e766_d_b38, eq42_e766_d_b39, eq42_e766_d_b40, eq42_e766_d_b41, eq42_e766_d_b42, eq42_e766_d_b43, eq42_e766_d_b44, eq42_e766_d_b45, eq42_e766_d_b46, eq42_e766_d_b47, eq42_e766_d_b48, eq42_e766_d_b49, eq42_e766_d_b50, eq42_e766_d_b51, eq42_e766_d_b52, eq42_e766_d_b53, eq42_e766_d_b54,) = {
    if ((var_guard358 != 0.0) && (!(((((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)) || (var_guard356 != 0.0)) || (var_guard357 != 0.0)))) {
        let eq42_e762: f64 = (p.p135 * var_en);
        let eq42_e762_d_n0: f64 = (p.p135 * var_en_dn0);
        let eq42_e762_d_n1: f64 = (p.p135 * var_en_dn1);
        let eq42_e762_d_n2: f64 = (p.p135 * var_en_dn2);
        let eq42_e762_d_n3: f64 = (p.p135 * var_en_dn3);
        let eq42_e762_d_n4: f64 = (p.p135 * var_en_dn4);
        let eq42_e762_d_n5: f64 = (p.p135 * var_en_dn5);
        let eq42_e762_d_n6: f64 = (p.p135 * var_en_dn6);
        let eq42_e762_d_n7: f64 = (p.p135 * var_en_dn7);
        let eq42_e762_d_n8: f64 = (p.p135 * var_en_dn8);
        let eq42_e762_d_n9: f64 = (p.p135 * var_en_dn9);
        let eq42_e762_d_n10: f64 = (p.p135 * var_en_dn10);
        let eq42_e762_d_n11: f64 = (p.p135 * var_en_dn11);
        let eq42_e762_d_n12: f64 = (p.p135 * var_en_dn12);
        let eq42_e762_d_n13: f64 = (p.p135 * var_en_dn13);
        let eq42_e762_d_n14: f64 = (p.p135 * var_en_dn14);
        let eq42_e762_d_n15: f64 = (p.p135 * var_en_dn15);
        let eq42_e762_d_n16: f64 = (p.p135 * var_en_dn16);
        let eq42_e762_d_n17: f64 = (p.p135 * var_en_dn17);
        let eq42_e762_d_n18: f64 = (p.p135 * var_en_dn18);
        let eq42_e762_d_n19: f64 = (p.p135 * var_en_dn19);
        let eq42_e762_d_n20: f64 = (p.p135 * var_en_dn20);
        let eq42_e762_d_n21: f64 = (p.p135 * var_en_dn21);
        let eq42_e762_d_n22: f64 = (p.p135 * var_en_dn22);
        let eq42_e762_d_b0: f64 = (p.p135 * var_en_db0);
        let eq42_e762_d_b1: f64 = (p.p135 * var_en_db1);
        let eq42_e762_d_b2: f64 = (p.p135 * var_en_db2);
        let eq42_e762_d_b3: f64 = (p.p135 * var_en_db3);
        let eq42_e762_d_b4: f64 = (p.p135 * var_en_db4);
        let eq42_e762_d_b5: f64 = (p.p135 * var_en_db5);
        let eq42_e762_d_b6: f64 = (p.p135 * var_en_db6);
        let eq42_e762_d_b7: f64 = (p.p135 * var_en_db7);
        let eq42_e762_d_b8: f64 = (p.p135 * var_en_db8);
        let eq42_e762_d_b9: f64 = (p.p135 * var_en_db9);
        let eq42_e762_d_b10: f64 = (p.p135 * var_en_db10);
        let eq42_e762_d_b11: f64 = (p.p135 * var_en_db11);
        let eq42_e762_d_b12: f64 = (p.p135 * var_en_db12);
        let eq42_e762_d_b13: f64 = (p.p135 * var_en_db13);
        let eq42_e762_d_b14: f64 = (p.p135 * var_en_db14);
        let eq42_e762_d_b15: f64 = (p.p135 * var_en_db15);
        let eq42_e762_d_b16: f64 = (p.p135 * var_en_db16);
        let eq42_e762_d_b17: f64 = (p.p135 * var_en_db17);
        let eq42_e762_d_b18: f64 = (p.p135 * var_en_db18);
        let eq42_e762_d_b19: f64 = (p.p135 * var_en_db19);
        let eq42_e762_d_b20: f64 = (p.p135 * var_en_db20);
        let eq42_e762_d_b21: f64 = (p.p135 * var_en_db21);
        let eq42_e762_d_b22: f64 = (p.p135 * var_en_db22);
        let eq42_e762_d_b23: f64 = (p.p135 * var_en_db23);
        let eq42_e762_d_b24: f64 = (p.p135 * var_en_db24);
        let eq42_e762_d_b25: f64 = (p.p135 * var_en_db25);
        let eq42_e762_d_b26: f64 = (p.p135 * var_en_db26);
        let eq42_e762_d_b27: f64 = (p.p135 * var_en_db27);
        let eq42_e762_d_b28: f64 = (p.p135 * var_en_db28);
        let eq42_e762_d_b29: f64 = (p.p135 * var_en_db29);
        let eq42_e762_d_b30: f64 = (p.p135 * var_en_db30);
        let eq42_e762_d_b31: f64 = (p.p135 * var_en_db31);
        let eq42_e762_d_b32: f64 = (p.p135 * var_en_db32);
        let eq42_e762_d_b33: f64 = (p.p135 * var_en_db33);
        let eq42_e762_d_b34: f64 = (p.p135 * var_en_db34);
        let eq42_e762_d_b35: f64 = (p.p135 * var_en_db35);
        let eq42_e762_d_b36: f64 = (p.p135 * var_en_db36);
        let eq42_e762_d_b37: f64 = (p.p135 * var_en_db37);
        let eq42_e762_d_b38: f64 = (p.p135 * var_en_db38);
        let eq42_e762_d_b39: f64 = (p.p135 * var_en_db39);
        let eq42_e762_d_b40: f64 = (p.p135 * var_en_db40);
        let eq42_e762_d_b41: f64 = (p.p135 * var_en_db41);
        let eq42_e762_d_b42: f64 = (p.p135 * var_en_db42);
        let eq42_e762_d_b43: f64 = (p.p135 * var_en_db43);
        let eq42_e762_d_b44: f64 = (p.p135 * var_en_db44);
        let eq42_e762_d_b45: f64 = (p.p135 * var_en_db45);
        let eq42_e762_d_b46: f64 = (p.p135 * var_en_db46);
        let eq42_e762_d_b47: f64 = (p.p135 * var_en_db47);
        let eq42_e762_d_b48: f64 = (p.p135 * var_en_db48);
        let eq42_e762_d_b49: f64 = (p.p135 * var_en_db49);
        let eq42_e762_d_b50: f64 = (p.p135 * var_en_db50);
        let eq42_e762_d_b51: f64 = (p.p135 * var_en_db51);
        let eq42_e762_d_b52: f64 = (p.p135 * var_en_db52);
        let eq42_e762_d_b53: f64 = (p.p135 * var_en_db53);
        let eq42_e762_d_b54: f64 = (p.p135 * var_en_db54);
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
    if ((var_guard358 != 0.0) && (!(((((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)) || (var_guard356 != 0.0)) || (var_guard357 != 0.0)))) {
        let eq43_e781: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, (nv5 - 0.0));
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
}
