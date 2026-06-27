#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[433] && s.b[434]) && s.b[436]) {
            s.copy_ad(230, 51);
        }

        if ((s.b[433] && s.b[434]) && (!s.b[436])) {
            s.copy_ad(231, 49);
            s.copy_ad(230, 50);
        }

        if (s.b[433] && s.b[434]) {
            s.store_offset_sqrt_ad(232, A::offset(A::square(s.ad_value(231)), 0.01), (-0.1));
            s.store_offset_scaled(146, 232, p.p166, (1.0 + p.p165));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_sub_ad(88, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p162)), (((((-1.0)) * (p.p162))) + (p.p159))), A::div_scaled_inputs(s.ad_value(232), (p.p168 * p.p167), A::sqrt(A::offset(A::square(s.ad_value(232)), (p.p168 * p.p168))), 1.0));
            s.store_scalar(223, (p.p9 / p.p160));
            s.store_div_from_scalar_ad(136, p.p161, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p158), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 230, 0.5, 159, ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(230), s.ad_value(159)), A::sub(s.ad_value(230), s.ad_value(159))), 0.0001)), 0.5, 159, 1.0);
            s.store_sub(222, 160, 88);
            s.store_div_scaled_inputs_indices(84, 223, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 223, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 222, 222, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 222, 1.0, 83, 2.0);
        }

        s.b[437] = (s.v[136] < 200.0);
        s.v[437] = if s.b[437] { 1.0 } else { 0.0 };

        if ((s.b[433] && s.b[434]) && s.b[437]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(222), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[433] && s.b[434]) && (!s.b[437])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(222), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[433] && s.b[434]) {
            s.store_sub_div_rhs_indices(100, 222, 153, 99);
        }

        s.b[438] = ((((s.v[100] - s.v[222])) as f64).abs() > 1e-19);
        s.v[438] = if s.b[438] { 1.0 } else { 0.0 };

        if ((s.b[433] && s.b[434]) && s.b[438]) {
            s.store_sub(101, 222, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p169);
            s.store_scaled_mul(103, 136, 90, p.p170);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if ((s.b[433] && s.b[434]) && s.b[438]) {
            let assign6620_ad_e9969: A = {
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
            };
            let assign6620_ad_e10007: A = {
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
            };
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign6620_ad_e9969, (-3.24e17), s.ad_value(83), assign6620_ad_e10007, (-3.24e17));
        }

        if ((s.b[433] && s.b[434]) && s.b[438]) {
            s.store_scaled_mul(107, 136, 91, p.p169);
            s.store_scaled_mul(108, 136, 91, p.p170);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 222, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p169, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p170, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if ((s.b[433] && s.b[434]) && s.b[438]) {
            let assign6780_ad_e10266: A = {
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
            };
            let assign6780_ad_e10304: A = {
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
            };
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign6780_ad_e10266, (-3.24e17), s.ad_value(83), assign6780_ad_e10304, (-3.24e17));
        }

        if ((s.b[433] && s.b[434]) && s.b[438]) {
            s.store_scaled_mul(121, 136, 137, p.p169);
            s.store_scaled_mul(122, 136, 137, p.p170);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.copy_ad(224, 128);
        }

        if ((s.b[433] && s.b[434]) && (!s.b[438])) {
            s.copy_ad(224, 100);
        }

        if (s.b[433] && s.b[434]) {
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p163);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p164);
            s.store_mul_scaled_abs_ad_rhs(136, 223, 1.0 / (p.p9), A::sub(s.ad_value(222), s.ad_value(224)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(224)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(90, 222, 222, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p161, 136, p.p161, 90, 1.0, 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(231), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 231, 90);
            s.store_sub(39, 222, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[439] = (s.v[136] < 200.0);
        s.v[439] = if s.b[439] { 1.0 } else { 0.0 };

        if ((s.b[433] && s.b[434]) && s.b[439]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[433] && s.b[434]) && (!s.b[439])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[433] && s.b[434]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[440] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[440] = if s.b[440] { 1.0 } else { 0.0 };

        if ((s.b[433] && s.b[434]) && s.b[440]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p169);
            s.store_scaled_mul(103, 136, 90, p.p170);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if ((s.b[433] && s.b[434]) && s.b[440]) {
            let assign7240_ad_e10998: A = {
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
            };
            let assign7240_ad_e11036: A = {
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
            };
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign7240_ad_e10998, (-3.24e17), s.ad_value(83), assign7240_ad_e11036, (-3.24e17));
        }

    }

    pub(super) fn stamp_reactive_block_4(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[433] && s.b[434]) && s.b[440]) {
            s.store_scaled_mul(107, 136, 91, p.p169);
            s.store_scaled_mul(108, 136, 91, p.p170);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 130, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_mul_scaled_powf_rhs(116, 136, p.p169, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p170, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if ((s.b[433] && s.b[434]) && s.b[440]) {
            let assign7390_ad_e11284: A = {
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
            };
            let assign7390_ad_e11322: A = {
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
            };
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign7390_ad_e11284, (-3.24e17), s.ad_value(83), assign7390_ad_e11322, (-3.24e17));
        }

        if ((s.b[433] && s.b[434]) && s.b[440]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p169, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p170, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(225, 128, 86);
        }

        if ((s.b[433] && s.b[434]) && (!s.b[440])) {
            s.store_add(225, 100, 86);
        }

        if (s.b[433] && s.b[434]) {
            s.store_scaled_add(226, 224, 225, 0.5);
            s.store_sub(227, 225, 224);
            s.store_mul_add_scaled_inputs3_offset_rhs(135, 227, s.ad_value(222), 1.0, s.ad_value(226), (-1.0), s.ad_value(83), 1.0, 0.0);
            s.store_mul_scaled_abs_ad_rhs(136, 223, 1.0 / (p.p9), A::sub(s.ad_value(222), s.ad_value(226)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);
            s.store_scaled_mul(96, 95, 223, (p.p4 * (p.p5 * 1.0 / (p.p161))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(232), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(227), (p.p25 * p.p25), s.ad_value(227)), 1.0);
            s.store_div(93, 98, 92);
            s.store_sub(90, 225, 224);
            s.store_add_scaled_inputs3_indices(91, 222, 1.0, 83, 1.0, 226, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 223, s.ad_value(222), ((p.p4 * p.p5) * p.p161), s.ad_value(226), (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p236) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p235, 1.0);
            s.store_div_from_scalar(190, p.p234, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p160);
            s.store_mul_add_scaled_inputs3_offset_rhs(228, 191, s.ad_value(222), ((p.p4 * p.p5) * p.p161), s.ad_value(226), (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);
            s.store_add_scaled_inputs3_indices(136, 222, 1.0, 83, 1.0, 226, -1.0);
            s.store_add_scaled_inputs(90, 224, 0.3333333333333333, 225, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(227)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(227)), 227, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(229, 191, 222, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p161) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p161) * p.p5) * 0.5)));
        }

        s.b[441] = (s.v[48] < 0.0);
        s.v[441] = if s.b[441] { 1.0 } else { 0.0 };

        if ((s.b[433] && s.b[434]) && s.b[441]) {
            s.store_sub_scaled_inputs(229, 228, (-1.0), 229, 1.0);
        }

        if (s.b[433] && (!s.b[434])) {
            s.store_scalar(228, 0.0);
            s.store_scalar(229, 0.0);
        }

        s.b[442] = (p.p150 != 0.0);
        s.v[442] = if s.b[442] { 1.0 } else { 0.0 };

        s.b[443] = (p.p150 == 1.0);
        s.v[443] = if s.b[443] { 1.0 } else { 0.0 };

        if (((!s.b[433]) && s.b[442]) && s.b[443]) {
            s.store_voltage(50, ctx, nodes, Some(9), Some(7));
        }

        if (((!s.b[433]) && s.b[442]) && (!s.b[443])) {
            s.store_voltage(50, ctx, nodes, Some(2), Some(7));
        }

        if ((!s.b[433]) && s.b[442]) {
            s.copy_ad(230, 50);
            s.store_scalar(146, (1.0 + p.p165));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_offset_scaled(88, 82, ((1.0 / (s.v[35])) * (p.p162)), (((((-1.0)) * (p.p162))) + (p.p159)));
            s.store_scalar(223, (p.p9 / p.p160));
            s.store_div_from_scalar_ad(136, p.p161, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p158), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 230, 0.5, 159, ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(230), s.ad_value(159)), A::sub(s.ad_value(230), s.ad_value(159))), 0.0001)), 0.5, 159, 1.0);
            s.store_sub(222, 160, 88);
            s.store_div_scaled_inputs_indices(84, 223, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 223, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 222, 222, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 222, 1.0, 83, 2.0);
        }

        s.b[444] = (s.v[136] < 200.0);
        s.v[444] = if s.b[444] { 1.0 } else { 0.0 };

        if (((!s.b[433]) && s.b[442]) && s.b[444]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(222), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[433]) && s.b[442]) && (!s.b[444])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(222), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[433]) && s.b[442]) {
            s.store_sub_div_rhs_indices(100, 222, 153, 99);
        }

        s.b[445] = ((((s.v[100] - s.v[222])) as f64).abs() > 1e-19);
        s.v[445] = if s.b[445] { 1.0 } else { 0.0 };

        if (((!s.b[433]) && s.b[442]) && s.b[445]) {
            s.store_sub(101, 222, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p169);
            s.store_scaled_mul(103, 136, 90, p.p170);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if (((!s.b[433]) && s.b[442]) && s.b[445]) {
            let assign8160_ad_e12412: A = {
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
            };
            let assign8160_ad_e12450: A = {
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
            };
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign8160_ad_e12412, (-3.24e17), s.ad_value(83), assign8160_ad_e12450, (-3.24e17));
        }

        if (((!s.b[433]) && s.b[442]) && s.b[445]) {
            s.store_scaled_mul(107, 136, 91, p.p169);
            s.store_scaled_mul(108, 136, 91, p.p170);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 222, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p169, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p170, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

    }

    pub(super) fn stamp_reactive_block_5(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[433]) && s.b[442]) && s.b[445]) {
            let assign8320_ad_e12725: A = {
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
            };
            let assign8320_ad_e12763: A = {
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
            };
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign8320_ad_e12725, (-3.24e17), s.ad_value(83), assign8320_ad_e12763, (-3.24e17));
        }

        if (((!s.b[433]) && s.b[442]) && s.b[445]) {
            s.store_scaled_mul(121, 136, 137, p.p169);
            s.store_scaled_mul(122, 136, 137, p.p170);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.copy_ad(224, 128);
        }

        if (((!s.b[433]) && s.b[442]) && (!s.b[445])) {
            s.copy_ad(224, 100);
        }

        if ((!s.b[433]) && s.b[442]) {
            s.store_scalar(231, 0.0);
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p163);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p164);
            s.store_mul_scaled_abs_ad_rhs(136, 223, 1.0 / (p.p9), A::sub(s.ad_value(222), s.ad_value(224)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(224)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(90, 222, 222, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p161, 136, p.p161, 90, 1.0, 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(231), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 231, 90);
            s.store_sub(39, 222, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[446] = (s.v[136] < 200.0);
        s.v[446] = if s.b[446] { 1.0 } else { 0.0 };

        if (((!s.b[433]) && s.b[442]) && s.b[446]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[433]) && s.b[442]) && (!s.b[446])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[433]) && s.b[442]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[447] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[447] = if s.b[447] { 1.0 } else { 0.0 };

        if (((!s.b[433]) && s.b[442]) && s.b[447]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p169);
            s.store_scaled_mul(103, 136, 90, p.p170);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if (((!s.b[433]) && s.b[442]) && s.b[447]) {
            let assign8790_ad_e13508: A = {
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
            };
            let assign8790_ad_e13546: A = {
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
            };
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign8790_ad_e13508, (-3.24e17), s.ad_value(83), assign8790_ad_e13546, (-3.24e17));
        }

        if (((!s.b[433]) && s.b[442]) && s.b[447]) {
            s.store_scaled_mul(107, 136, 91, p.p169);
            s.store_scaled_mul(108, 136, 91, p.p170);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 130, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_mul_scaled_powf_rhs(116, 136, p.p169, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p170, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if (((!s.b[433]) && s.b[442]) && s.b[447]) {
            let assign8940_ad_e13809: A = {
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
            };
            let assign8940_ad_e13847: A = {
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
            };
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign8940_ad_e13809, (-3.24e17), s.ad_value(83), assign8940_ad_e13847, (-3.24e17));
        }

        if (((!s.b[433]) && s.b[442]) && s.b[447]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p169, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p170, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(225, 128, 86);
        }

        if (((!s.b[433]) && s.b[442]) && (!s.b[447])) {
            s.store_add(225, 100, 86);
        }

        if ((!s.b[433]) && s.b[442]) {
            s.store_scaled_add(226, 224, 225, 0.5);
            s.store_sub(227, 225, 224);
            s.store_sub(90, 225, 224);
            s.store_add_scaled_inputs3_indices(91, 222, 1.0, 83, 1.0, 226, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 223, s.ad_value(222), ((p.p4 * p.p5) * p.p161), s.ad_value(226), (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p236) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p235, 1.0);
            s.store_div_from_scalar(190, p.p234, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p160);
            s.store_mul_add_scaled_inputs3_offset_rhs(228, 191, s.ad_value(222), ((p.p4 * p.p5) * p.p161), s.ad_value(226), (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);
            s.store_add_scaled_inputs3_indices(136, 222, 1.0, 83, 1.0, 226, -1.0);
            s.store_add_scaled_inputs(90, 224, 0.3333333333333333, 225, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(227)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(227)), 227, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(229, 191, 222, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p161) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p161) * p.p5) * 0.5)));
        }

        if ((!s.b[433]) && (!s.b[442])) {
            s.store_scalar(228, 0.0);
            s.store_scalar(229, 0.0);
        }

        s.b[448] = (p.p149 == 0.0);
        s.v[448] = if s.b[448] { 1.0 } else { 0.0 };

        s.b[449] = (p.p151 != 0.0);
        s.v[449] = if s.b[449] { 1.0 } else { 0.0 };

        if (s.b[448] && s.b[449]) {
            s.store_voltage(53, ctx, nodes, Some(8), Some(19));
        }

        s.b[450] = (p.p151 == 1.0);
        s.v[450] = if s.b[450] { 1.0 } else { 0.0 };

        if ((s.b[448] && s.b[449]) && s.b[450]) {
            s.store_voltage(54, ctx, nodes, Some(9), Some(19));
            s.store_voltage(55, ctx, nodes, Some(9), Some(8));
        }

        if ((s.b[448] && s.b[449]) && (!s.b[450])) {
            s.store_voltage(54, ctx, nodes, Some(2), Some(19));
            s.store_voltage(55, ctx, nodes, Some(2), Some(8));
        }

        if (s.b[448] && s.b[449]) {
            s.store_scalar(52, 1.0);
        }

        s.b[451] = (s.v[53] < 0.0);
        s.v[451] = if s.b[451] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[448] && s.b[449]) && s.b[451]) {
            s.store_scalar(52, (-1.0));
            s.store_mul(243, 52, 53);
            s.copy_ad(242, 55);
        }

        if ((s.b[448] && s.b[449]) && (!s.b[451])) {
            s.copy_ad(243, 53);
            s.copy_ad(242, 54);
        }

        if (s.b[448] && s.b[449]) {
            s.store_offset_sqrt_ad(244, A::offset(A::square(s.ad_value(243)), 0.01), (-0.1));
            s.store_offset_scaled(146, 244, p.p166, (1.0 + p.p165));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_sub_ad(88, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p162)), (((((-1.0)) * (p.p162))) + (p.p159))), A::div_scaled_inputs(s.ad_value(244), (p.p168 * p.p167), A::sqrt(A::offset(A::square(s.ad_value(244)), (p.p168 * p.p168))), 1.0));
            s.store_scalar(235, (p.p9 / p.p160));
            s.store_div_from_scalar_ad(136, p.p161, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p158), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 242, 0.5, 159, ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(242), s.ad_value(159)), A::sub(s.ad_value(242), s.ad_value(159))), 0.0001)), 0.5, 159, 1.0);
            s.store_sub(234, 160, 88);
            s.store_div_scaled_inputs_indices(84, 235, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 235, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 234, 234, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 234, 1.0, 83, 2.0);
        }

        s.b[452] = (s.v[136] < 200.0);
        s.v[452] = if s.b[452] { 1.0 } else { 0.0 };

        if ((s.b[448] && s.b[449]) && s.b[452]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(234), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[448] && s.b[449]) && (!s.b[452])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(234), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[448] && s.b[449]) {
            s.store_sub_div_rhs_indices(100, 234, 153, 99);
        }

        s.b[453] = ((((s.v[100] - s.v[234])) as f64).abs() > 1e-19);
        s.v[453] = if s.b[453] { 1.0 } else { 0.0 };

        if ((s.b[448] && s.b[449]) && s.b[453]) {
            s.store_sub(101, 234, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p169);
            s.store_scaled_mul(103, 136, 90, p.p170);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if ((s.b[448] && s.b[449]) && s.b[453]) {
            let assign9710_ad_e14898: A = {
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
            };
            let assign9710_ad_e14936: A = {
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
            };
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign9710_ad_e14898, (-3.24e17), s.ad_value(83), assign9710_ad_e14936, (-3.24e17));
        }

        if ((s.b[448] && s.b[449]) && s.b[453]) {
            s.store_scaled_mul(107, 136, 91, p.p169);
            s.store_scaled_mul(108, 136, 91, p.p170);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 234, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p169, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p170, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if ((s.b[448] && s.b[449]) && s.b[453]) {
            let assign9870_ad_e15195: A = {
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
            };
            let assign9870_ad_e15233: A = {
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
            };
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign9870_ad_e15195, (-3.24e17), s.ad_value(83), assign9870_ad_e15233, (-3.24e17));
        }

        if ((s.b[448] && s.b[449]) && s.b[453]) {
            s.store_scaled_mul(121, 136, 137, p.p169);
            s.store_scaled_mul(122, 136, 137, p.p170);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.copy_ad(236, 128);
        }

        if ((s.b[448] && s.b[449]) && (!s.b[453])) {
            s.copy_ad(236, 100);
        }

        if (s.b[448] && s.b[449]) {
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p163);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p164);
            s.store_mul_scaled_abs_ad_rhs(136, 235, 1.0 / (p.p9), A::sub(s.ad_value(234), s.ad_value(236)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(236)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(90, 234, 234, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p161, 136, p.p161, 90, 1.0, 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(243), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 243, 90);
            s.store_sub(39, 234, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[454] = (s.v[136] < 200.0);
        s.v[454] = if s.b[454] { 1.0 } else { 0.0 };

        if ((s.b[448] && s.b[449]) && s.b[454]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[448] && s.b[449]) && (!s.b[454])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[448] && s.b[449]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[455] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[455] = if s.b[455] { 1.0 } else { 0.0 };

        if ((s.b[448] && s.b[449]) && s.b[455]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p169);
            s.store_scaled_mul(103, 136, 90, p.p170);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

    }

    pub(super) fn stamp_reactive_block_7(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[448] && s.b[449]) && s.b[455]) {
            let assign10330_ad_e15927: A = {
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
            };
            let assign10330_ad_e15965: A = {
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
            };
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign10330_ad_e15927, (-3.24e17), s.ad_value(83), assign10330_ad_e15965, (-3.24e17));
        }

        if ((s.b[448] && s.b[449]) && s.b[455]) {
            s.store_scaled_mul(107, 136, 91, p.p169);
            s.store_scaled_mul(108, 136, 91, p.p170);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 130, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_mul_scaled_powf_rhs(116, 136, p.p169, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p170, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if ((s.b[448] && s.b[449]) && s.b[455]) {
            let assign10480_ad_e16213: A = {
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
            };
            let assign10480_ad_e16251: A = {
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
            };
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign10480_ad_e16213, (-3.24e17), s.ad_value(83), assign10480_ad_e16251, (-3.24e17));
        }

        if ((s.b[448] && s.b[449]) && s.b[455]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p169, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p170, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(237, 128, 86);
        }

        if ((s.b[448] && s.b[449]) && (!s.b[455])) {
            s.store_add(237, 100, 86);
        }

        if (s.b[448] && s.b[449]) {
            s.store_scaled_add(238, 236, 237, 0.5);
            s.store_sub(239, 237, 236);
            s.store_mul_add_scaled_inputs3_offset_rhs(135, 239, s.ad_value(234), 1.0, s.ad_value(238), (-1.0), s.ad_value(83), 1.0, 0.0);
            s.store_mul_scaled_abs_ad_rhs(136, 235, 1.0 / (p.p9), A::sub(s.ad_value(234), s.ad_value(238)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);
            s.store_scaled_mul(96, 95, 235, (p.p4 * (p.p5 * 1.0 / (p.p161))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(244), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(239), (p.p25 * p.p25), s.ad_value(239)), 1.0);
            s.store_div(93, 98, 92);
            s.store_sub(90, 237, 236);
            s.store_add_scaled_inputs3_indices(91, 234, 1.0, 83, 1.0, 238, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 235, s.ad_value(234), ((p.p4 * p.p5) * p.p161), s.ad_value(238), (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p236) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p235, 1.0);
            s.store_div_from_scalar(190, p.p234, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p160);
            s.store_mul_add_scaled_inputs3_offset_rhs(240, 191, s.ad_value(234), ((p.p4 * p.p5) * p.p161), s.ad_value(238), (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);
            s.store_add_scaled_inputs3_indices(136, 234, 1.0, 83, 1.0, 238, -1.0);
            s.store_add_scaled_inputs(90, 236, 0.3333333333333333, 237, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(239)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(239)), 239, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(241, 191, 234, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p161) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p161) * p.p5) * 0.5)));
        }

        s.b[456] = (s.v[52] < 0.0);
        s.v[456] = if s.b[456] { 1.0 } else { 0.0 };

        if ((s.b[448] && s.b[449]) && s.b[456]) {
            s.store_sub_scaled_inputs(241, 240, (-1.0), 241, 1.0);
        }

        if (s.b[448] && (!s.b[449])) {
            s.store_scalar(240, 0.0);
            s.store_scalar(241, 0.0);
        }

        s.b[457] = (p.p151 != 0.0);
        s.v[457] = if s.b[457] { 1.0 } else { 0.0 };

        s.b[458] = (p.p151 == 1.0);
        s.v[458] = if s.b[458] { 1.0 } else { 0.0 };

        if (((!s.b[448]) && s.b[457]) && s.b[458]) {
            s.store_voltage(54, ctx, nodes, Some(9), Some(8));
        }

        if (((!s.b[448]) && s.b[457]) && (!s.b[458])) {
            s.store_voltage(54, ctx, nodes, Some(2), Some(8));
        }

        if ((!s.b[448]) && s.b[457]) {
            s.copy_ad(234, 54);
            s.store_scalar(146, (1.0 + p.p165));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_offset_scaled(88, 82, ((1.0 / (s.v[35])) * (p.p162)), (((((-1.0)) * (p.p162))) + (p.p159)));
            s.store_scalar(235, (p.p9 / p.p160));
            s.store_div_from_scalar_ad(136, p.p161, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p158), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 242, 0.5, 159, ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(242), s.ad_value(159)), A::sub(s.ad_value(242), s.ad_value(159))), 0.0001)), 0.5, 159, 1.0);
            s.store_sub(234, 160, 88);
            s.store_div_scaled_inputs_indices(84, 235, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 235, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 234, 234, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 234, 1.0, 83, 2.0);
        }

        s.b[459] = (s.v[136] < 200.0);
        s.v[459] = if s.b[459] { 1.0 } else { 0.0 };

        if (((!s.b[448]) && s.b[457]) && s.b[459]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(234), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[448]) && s.b[457]) && (!s.b[459])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(234), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[448]) && s.b[457]) {
            s.store_sub_div_rhs_indices(100, 234, 153, 99);
        }

        s.b[460] = ((((s.v[100] - s.v[234])) as f64).abs() > 1e-19);
        s.v[460] = if s.b[460] { 1.0 } else { 0.0 };

        if (((!s.b[448]) && s.b[457]) && s.b[460]) {
            s.store_sub(101, 234, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p169);
            s.store_scaled_mul(103, 136, 90, p.p170);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if (((!s.b[448]) && s.b[457]) && s.b[460]) {
            let assign11250_ad_e17341: A = {
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
            };
            let assign11250_ad_e17379: A = {
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
            };
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign11250_ad_e17341, (-3.24e17), s.ad_value(83), assign11250_ad_e17379, (-3.24e17));
        }

        if (((!s.b[448]) && s.b[457]) && s.b[460]) {
            s.store_scaled_mul(107, 136, 91, p.p169);
            s.store_scaled_mul(108, 136, 91, p.p170);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
        }

    }

    pub(super) fn stamp_reactive_block_8(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[448]) && s.b[457]) && s.b[460]) {
            s.store_sub(115, 234, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p169, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p170, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if (((!s.b[448]) && s.b[457]) && s.b[460]) {
            let assign11410_ad_e17654: A = {
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
            };
            let assign11410_ad_e17692: A = {
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
            };
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign11410_ad_e17654, (-3.24e17), s.ad_value(83), assign11410_ad_e17692, (-3.24e17));
        }

        if (((!s.b[448]) && s.b[457]) && s.b[460]) {
            s.store_scaled_mul(121, 136, 137, p.p169);
            s.store_scaled_mul(122, 136, 137, p.p170);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.copy_ad(236, 128);
        }

        if (((!s.b[448]) && s.b[457]) && (!s.b[460])) {
            s.copy_ad(236, 100);
        }

        if ((!s.b[448]) && s.b[457]) {
            s.store_scalar(243, 0.0);
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p163);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p164);
            s.store_mul_scaled_abs_ad_rhs(136, 235, 1.0 / (p.p9), A::sub(s.ad_value(234), s.ad_value(236)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(236)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(90, 234, 234, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p161, 136, p.p161, 90, 1.0, 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(243), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 243, 90);
            s.store_sub(39, 234, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[461] = (s.v[136] < 200.0);
        s.v[461] = if s.b[461] { 1.0 } else { 0.0 };

        if (((!s.b[448]) && s.b[457]) && s.b[461]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[448]) && s.b[457]) && (!s.b[461])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[448]) && s.b[457]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[462] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[462] = if s.b[462] { 1.0 } else { 0.0 };

        if (((!s.b[448]) && s.b[457]) && s.b[462]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p169);
            s.store_scaled_mul(103, 136, 90, p.p170);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if (((!s.b[448]) && s.b[457]) && s.b[462]) {
            let assign11880_ad_e18437: A = {
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
            };
            let assign11880_ad_e18475: A = {
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
            };
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign11880_ad_e18437, (-3.24e17), s.ad_value(83), assign11880_ad_e18475, (-3.24e17));
        }

        if (((!s.b[448]) && s.b[457]) && s.b[462]) {
            s.store_scaled_mul(107, 136, 91, p.p169);
            s.store_scaled_mul(108, 136, 91, p.p170);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 130, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_mul_scaled_powf_rhs(116, 136, p.p169, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p170, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if (((!s.b[448]) && s.b[457]) && s.b[462]) {
            let assign12030_ad_e18738: A = {
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
            };
            let assign12030_ad_e18776: A = {
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
            };
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign12030_ad_e18738, (-3.24e17), s.ad_value(83), assign12030_ad_e18776, (-3.24e17));
        }

        if (((!s.b[448]) && s.b[457]) && s.b[462]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p169, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p170, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(237, 128, 86);
        }

        if (((!s.b[448]) && s.b[457]) && (!s.b[462])) {
            s.store_add(237, 100, 86);
        }

        if ((!s.b[448]) && s.b[457]) {
            s.store_scaled_add(238, 236, 237, 0.5);
            s.store_sub(239, 237, 236);
            s.store_sub(90, 237, 236);
            s.store_add_scaled_inputs3_indices(91, 234, 1.0, 83, 1.0, 238, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 235, s.ad_value(234), ((p.p4 * p.p5) * p.p161), s.ad_value(238), (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p236) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p235, 1.0);
            s.store_div_from_scalar(190, p.p234, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p160);
            s.store_mul_add_scaled_inputs3_offset_rhs(240, 191, s.ad_value(234), ((p.p4 * p.p5) * p.p161), s.ad_value(238), (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);
            s.store_add_scaled_inputs3_indices(136, 234, 1.0, 83, 1.0, 238, -1.0);
            s.store_add_scaled_inputs(90, 236, 0.3333333333333333, 237, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(239)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(239)), 239, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(241, 191, 234, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p161) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p161) * p.p5) * 0.5)));
        }

        if ((!s.b[448]) && (!s.b[457])) {
            s.store_scalar(240, 0.0);
            s.store_scalar(241, 0.0);
        }

        s.b[463] = (p.p149 == 0.0);
        s.v[463] = if s.b[463] { 1.0 } else { 0.0 };

        s.b[464] = (p.p152 != 0.0);
        s.v[464] = if s.b[464] { 1.0 } else { 0.0 };

        if (s.b[463] && s.b[464]) {
            s.store_voltage(57, ctx, nodes, Some(16), Some(15));
        }

    }

    pub(super) fn stamp_reactive_block_9(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[465] = (p.p152 == 1.0);
        s.v[465] = if s.b[465] { 1.0 } else { 0.0 };

        if ((s.b[463] && s.b[464]) && s.b[465]) {
            s.store_voltage(58, ctx, nodes, Some(9), Some(15));
            s.store_voltage(59, ctx, nodes, Some(9), Some(16));
        }

        if ((s.b[463] && s.b[464]) && (!s.b[465])) {
            s.store_voltage(58, ctx, nodes, Some(2), Some(15));
            s.store_voltage(59, ctx, nodes, Some(2), Some(16));
        }

        if (s.b[463] && s.b[464]) {
            s.store_scalar(56, 1.0);
        }

        s.b[466] = (s.v[57] < 0.0);
        s.v[466] = if s.b[466] { 1.0 } else { 0.0 };

        if ((s.b[463] && s.b[464]) && s.b[466]) {
            s.store_scalar(56, (-1.0));
            s.store_mul(255, 56, 57);
            s.copy_ad(254, 59);
        }

        if ((s.b[463] && s.b[464]) && (!s.b[466])) {
            s.copy_ad(255, 57);
            s.copy_ad(254, 58);
        }

        if (s.b[463] && s.b[464]) {
            s.store_offset_sqrt_ad(256, A::offset(A::square(s.ad_value(255)), 0.01), (-0.1));
            s.store_offset_scaled(146, 256, p.p179, (1.0 + p.p178));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_sub_ad(88, A::sub_from_scalar(p.p172, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p175)), (((-1.0)) * (p.p175)))), A::div_scaled_inputs(s.ad_value(256), (p.p181 * p.p180), A::sqrt(A::offset(A::square(s.ad_value(256)), (p.p181 * p.p181))), 1.0));
            s.store_scalar(247, (p.p9 / p.p173));
            s.store_div_from_scalar_ad(136, p.p174, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p171), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 254, 0.5, 159, ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(254), s.ad_value(159)), A::sub(s.ad_value(254), s.ad_value(159))), 0.0001)), 0.5, 159, 1.0);
            s.store_sub(246, 160, 88);
            s.store_div_scaled_inputs_indices(84, 247, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 247, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 246, 246, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 246, 1.0, 83, 2.0);
        }

        s.b[467] = (s.v[136] < 200.0);
        s.v[467] = if s.b[467] { 1.0 } else { 0.0 };

        if ((s.b[463] && s.b[464]) && s.b[467]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(246), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[463] && s.b[464]) && (!s.b[467])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(246), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[463] && s.b[464]) {
            s.store_sub_div_rhs_indices(100, 246, 153, 99);
        }

        s.b[468] = ((((s.v[100] - s.v[246])) as f64).abs() > 1e-19);
        s.v[468] = if s.b[468] { 1.0 } else { 0.0 };

        if ((s.b[463] && s.b[464]) && s.b[468]) {
            s.store_sub(101, 246, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if ((s.b[463] && s.b[464]) && s.b[468]) {
            let assign12800_ad_e19827: A = {
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
            };
            let assign12800_ad_e19865: A = {
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
            };
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign12800_ad_e19827, (-3.24e17), s.ad_value(83), assign12800_ad_e19865, (-3.24e17));
        }

        if ((s.b[463] && s.b[464]) && s.b[468]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 246, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p182, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p183, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if ((s.b[463] && s.b[464]) && s.b[468]) {
            let assign12960_ad_e20124: A = {
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
            };
            let assign12960_ad_e20162: A = {
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
            };
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign12960_ad_e20124, (-3.24e17), s.ad_value(83), assign12960_ad_e20162, (-3.24e17));
        }

        if ((s.b[463] && s.b[464]) && s.b[468]) {
            s.store_scaled_mul(121, 136, 137, p.p182);
            s.store_scaled_mul(122, 136, 137, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.copy_ad(248, 128);
        }

        if ((s.b[463] && s.b[464]) && (!s.b[468])) {
            s.copy_ad(248, 100);
        }

        if (s.b[463] && s.b[464]) {
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p176);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p177);
            s.store_mul_scaled_abs_ad_rhs(136, 247, 1.0 / (p.p9), A::sub(s.ad_value(246), s.ad_value(248)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(248)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(90, 246, 246, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p174, 136, p.p174, 90, 1.0, 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(255), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 255, 90);
            s.store_sub(39, 246, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[469] = (s.v[136] < 200.0);
        s.v[469] = if s.b[469] { 1.0 } else { 0.0 };

        if ((s.b[463] && s.b[464]) && s.b[469]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[463] && s.b[464]) && (!s.b[469])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[463] && s.b[464]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[470] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[470] = if s.b[470] { 1.0 } else { 0.0 };

        if ((s.b[463] && s.b[464]) && s.b[470]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

    }

    pub(super) fn stamp_reactive_block_10(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[463] && s.b[464]) && s.b[470]) {
            let assign13420_ad_e20856: A = {
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
            };
            let assign13420_ad_e20894: A = {
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
            };
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign13420_ad_e20856, (-3.24e17), s.ad_value(83), assign13420_ad_e20894, (-3.24e17));
        }

        if ((s.b[463] && s.b[464]) && s.b[470]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 130, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_mul_scaled_powf_rhs(116, 136, p.p182, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p183, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if ((s.b[463] && s.b[464]) && s.b[470]) {
            let assign13570_ad_e21142: A = {
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
            };
            let assign13570_ad_e21180: A = {
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
            };
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign13570_ad_e21142, (-3.24e17), s.ad_value(83), assign13570_ad_e21180, (-3.24e17));
        }

        if ((s.b[463] && s.b[464]) && s.b[470]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p182, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p183, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(249, 128, 86);
        }

        if ((s.b[463] && s.b[464]) && (!s.b[470])) {
            s.store_add(249, 100, 86);
        }

        if (s.b[463] && s.b[464]) {
            s.store_scaled_add(250, 248, 249, 0.5);
            s.store_sub(251, 249, 248);
            s.store_mul_add_scaled_inputs3_offset_rhs(135, 251, s.ad_value(246), 1.0, s.ad_value(250), (-1.0), s.ad_value(83), 1.0, 0.0);
            s.store_mul_scaled_abs_ad_rhs(136, 247, 1.0 / (p.p9), A::sub(s.ad_value(246), s.ad_value(250)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);
            s.store_scaled_mul(96, 95, 247, (p.p4 * (p.p5 * 1.0 / (p.p174))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(256), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(251), (p.p25 * p.p25), s.ad_value(251)), 1.0);
            s.store_div(93, 98, 92);
            s.store_sub(90, 249, 248);
            s.store_add_scaled_inputs3_indices(91, 246, 1.0, 83, 1.0, 250, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 247, s.ad_value(246), ((p.p4 * p.p5) * p.p174), s.ad_value(250), (((-1.0)) * (((p.p4 * p.p5) * p.p174))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p174), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p239) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p238, 1.0);
            s.store_div_from_scalar(190, p.p237, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p173);
            s.store_mul_add_scaled_inputs3_offset_rhs(252, 191, s.ad_value(246), ((p.p4 * p.p5) * p.p174), s.ad_value(250), (((-1.0)) * (((p.p4 * p.p5) * p.p174))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p174), 0.0);
            s.store_add_scaled_inputs3_indices(136, 246, 1.0, 83, 1.0, 250, -1.0);
            s.store_add_scaled_inputs(90, 248, 0.3333333333333333, 249, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(251)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(251)), 251, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(253, 191, 246, (-(((p.p4 * p.p174) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p174) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p174) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p174) * p.p5) * 0.5)));
        }

        s.b[471] = (s.v[56] < 0.0);
        s.v[471] = if s.b[471] { 1.0 } else { 0.0 };

        if ((s.b[463] && s.b[464]) && s.b[471]) {
            s.store_sub_scaled_inputs(253, 252, (-1.0), 253, 1.0);
        }

        if (s.b[463] && (!s.b[464])) {
            s.store_scalar(252, 0.0);
            s.store_scalar(253, 0.0);
        }

        s.b[472] = (p.p152 != 0.0);
        s.v[472] = if s.b[472] { 1.0 } else { 0.0 };

        s.b[473] = (p.p152 == 1.0);
        s.v[473] = if s.b[473] { 1.0 } else { 0.0 };

        if (((!s.b[463]) && s.b[472]) && s.b[473]) {
            s.store_voltage(58, ctx, nodes, Some(9), Some(7));
        }

        if (((!s.b[463]) && s.b[472]) && (!s.b[473])) {
            s.store_voltage(58, ctx, nodes, Some(2), Some(7));
        }

        if ((!s.b[463]) && s.b[472]) {
            s.copy_ad(254, 58);
            s.store_scalar(146, (1.0 + p.p178));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_sub_from_scalar_ad(88, p.p172, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p175)), (((-1.0)) * (p.p175))));
            s.store_scalar(247, (p.p9 / p.p173));
            s.store_div_from_scalar_ad(136, p.p174, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p171), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 254, 0.5, 159, ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(254), s.ad_value(159)), A::sub(s.ad_value(254), s.ad_value(159))), 0.0001)), 0.5, 159, 1.0);
            s.store_sub(246, 160, 88);
            s.store_div_scaled_inputs_indices(84, 247, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 247, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 246, 246, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 246, 1.0, 83, 2.0);
        }

        s.b[474] = (s.v[136] < 200.0);
        s.v[474] = if s.b[474] { 1.0 } else { 0.0 };

        if (((!s.b[463]) && s.b[472]) && s.b[474]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(246), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[463]) && s.b[472]) && (!s.b[474])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(246), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[463]) && s.b[472]) {
            s.store_sub_div_rhs_indices(100, 246, 153, 99);
        }

        s.b[475] = ((((s.v[100] - s.v[246])) as f64).abs() > 1e-19);
        s.v[475] = if s.b[475] { 1.0 } else { 0.0 };

        if (((!s.b[463]) && s.b[472]) && s.b[475]) {
            s.store_sub(101, 246, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if (((!s.b[463]) && s.b[472]) && s.b[475]) {
            let assign14340_ad_e22270: A = {
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
            };
            let assign14340_ad_e22308: A = {
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
            };
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign14340_ad_e22270, (-3.24e17), s.ad_value(83), assign14340_ad_e22308, (-3.24e17));
        }

        if (((!s.b[463]) && s.b[472]) && s.b[475]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
        }

    }

    pub(super) fn stamp_reactive_block_11(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[463]) && s.b[472]) && s.b[475]) {
            s.store_sub(115, 246, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p182, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p183, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if (((!s.b[463]) && s.b[472]) && s.b[475]) {
            let assign14500_ad_e22583: A = {
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
            };
            let assign14500_ad_e22621: A = {
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
            };
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign14500_ad_e22583, (-3.24e17), s.ad_value(83), assign14500_ad_e22621, (-3.24e17));
        }

        if (((!s.b[463]) && s.b[472]) && s.b[475]) {
            s.store_scaled_mul(121, 136, 137, p.p182);
            s.store_scaled_mul(122, 136, 137, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.copy_ad(248, 128);
        }

        if (((!s.b[463]) && s.b[472]) && (!s.b[475])) {
            s.copy_ad(248, 100);
        }

        if ((!s.b[463]) && s.b[472]) {
            s.store_scalar(255, 0.0);
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p176);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p177);
            s.store_mul_scaled_abs_ad_rhs(136, 247, 1.0 / (p.p9), A::sub(s.ad_value(246), s.ad_value(248)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(248)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(90, 246, 246, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p174, 136, p.p174, 90, 1.0, 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(255), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 255, 90);
            s.store_sub(39, 246, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[476] = (s.v[136] < 200.0);
        s.v[476] = if s.b[476] { 1.0 } else { 0.0 };

        if (((!s.b[463]) && s.b[472]) && s.b[476]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[463]) && s.b[472]) && (!s.b[476])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[463]) && s.b[472]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[477] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[477] = if s.b[477] { 1.0 } else { 0.0 };

        if (((!s.b[463]) && s.b[472]) && s.b[477]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if (((!s.b[463]) && s.b[472]) && s.b[477]) {
            let assign14970_ad_e23366: A = {
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
            };
            let assign14970_ad_e23404: A = {
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
            };
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign14970_ad_e23366, (-3.24e17), s.ad_value(83), assign14970_ad_e23404, (-3.24e17));
        }

        if (((!s.b[463]) && s.b[472]) && s.b[477]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 130, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_mul_scaled_powf_rhs(116, 136, p.p182, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p183, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if (((!s.b[463]) && s.b[472]) && s.b[477]) {
            let assign15120_ad_e23667: A = {
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
            };
            let assign15120_ad_e23705: A = {
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
            };
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign15120_ad_e23667, (-3.24e17), s.ad_value(83), assign15120_ad_e23705, (-3.24e17));
        }

        if (((!s.b[463]) && s.b[472]) && s.b[477]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p182, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p183, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(249, 128, 86);
        }

        if (((!s.b[463]) && s.b[472]) && (!s.b[477])) {
            s.store_add(249, 100, 86);
        }

        if ((!s.b[463]) && s.b[472]) {
            s.store_scaled_add(250, 248, 249, 0.5);
            s.store_sub(251, 249, 248);
            s.store_sub(90, 249, 248);
            s.store_add_scaled_inputs3_indices(91, 246, 1.0, 83, 1.0, 250, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 247, s.ad_value(246), ((p.p4 * p.p5) * p.p174), s.ad_value(250), (((-1.0)) * (((p.p4 * p.p5) * p.p174))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p174), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p239) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p238, 1.0);
            s.store_div_from_scalar(190, p.p237, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p173);
            s.store_mul_add_scaled_inputs3_offset_rhs(252, 191, s.ad_value(246), ((p.p4 * p.p5) * p.p174), s.ad_value(250), (((-1.0)) * (((p.p4 * p.p5) * p.p174))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p174), 0.0);
            s.store_add_scaled_inputs3_indices(136, 246, 1.0, 83, 1.0, 250, -1.0);
            s.store_add_scaled_inputs(90, 248, 0.3333333333333333, 249, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(251)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(251)), 251, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(253, 191, 246, (-(((p.p4 * p.p174) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p174) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p174) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p174) * p.p5) * 0.5)));
        }

        if ((!s.b[463]) && (!s.b[472])) {
            s.store_scalar(252, 0.0);
            s.store_scalar(253, 0.0);
        }

        s.b[478] = (p.p149 == 0.0);
        s.v[478] = if s.b[478] { 1.0 } else { 0.0 };

        s.b[479] = (p.p153 != 0.0);
        s.v[479] = if s.b[479] { 1.0 } else { 0.0 };

        if (s.b[478] && s.b[479]) {
            s.store_voltage(61, ctx, nodes, Some(19), Some(20));
        }

    }

    pub(super) fn stamp_reactive_block_12(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[480] = (p.p153 == 1.0);
        s.v[480] = if s.b[480] { 1.0 } else { 0.0 };

        if ((s.b[478] && s.b[479]) && s.b[480]) {
            s.store_voltage(62, ctx, nodes, Some(9), Some(20));
            s.store_voltage(63, ctx, nodes, Some(9), Some(19));
        }

        if ((s.b[478] && s.b[479]) && (!s.b[480])) {
            s.store_voltage(62, ctx, nodes, Some(2), Some(20));
            s.store_voltage(63, ctx, nodes, Some(2), Some(19));
        }

        if (s.b[478] && s.b[479]) {
            s.store_scalar(60, 1.0);
        }

        s.b[481] = (s.v[61] < 0.0);
        s.v[481] = if s.b[481] { 1.0 } else { 0.0 };

        if ((s.b[478] && s.b[479]) && s.b[481]) {
            s.store_scalar(60, (-1.0));
            s.store_mul(267, 60, 61);
            s.copy_ad(266, 63);
        }

        if ((s.b[478] && s.b[479]) && (!s.b[481])) {
            s.copy_ad(267, 61);
            s.copy_ad(266, 62);
        }

        if (s.b[478] && s.b[479]) {
            s.store_offset_sqrt_ad(268, A::offset(A::square(s.ad_value(267)), 0.01), (-0.1));
            s.store_offset_scaled(146, 268, p.p179, (1.0 + p.p178));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_sub_ad(88, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p175)), (((((-1.0)) * (p.p175))) + (p.p172))), A::div_scaled_inputs(s.ad_value(268), (p.p181 * p.p180), A::sqrt(A::offset(A::square(s.ad_value(268)), (p.p181 * p.p181))), 1.0));
            s.store_scalar(259, (p.p9 / p.p173));
            s.store_div_from_scalar_ad(136, p.p174, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p171), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 266, 0.5, 159, ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(266), s.ad_value(159)), A::sub(s.ad_value(266), s.ad_value(159))), 0.0001)), 0.5, 159, 1.0);
            s.store_sub(258, 160, 88);
            s.store_div_scaled_inputs_indices(84, 259, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 259, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 258, 258, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 258, 1.0, 83, 2.0);
        }

        s.b[482] = (s.v[136] < 200.0);
        s.v[482] = if s.b[482] { 1.0 } else { 0.0 };

        if ((s.b[478] && s.b[479]) && s.b[482]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(258), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[478] && s.b[479]) && (!s.b[482])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(258), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[478] && s.b[479]) {
            s.store_sub_div_rhs_indices(100, 258, 153, 99);
        }

        s.b[483] = ((((s.v[100] - s.v[258])) as f64).abs() > 1e-19);
        s.v[483] = if s.b[483] { 1.0 } else { 0.0 };

        if ((s.b[478] && s.b[479]) && s.b[483]) {
            s.store_sub(101, 258, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if ((s.b[478] && s.b[479]) && s.b[483]) {
            let assign15890_ad_e24756: A = {
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
            };
            let assign15890_ad_e24794: A = {
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
            };
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign15890_ad_e24756, (-3.24e17), s.ad_value(83), assign15890_ad_e24794, (-3.24e17));
        }

        if ((s.b[478] && s.b[479]) && s.b[483]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 258, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p182, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p183, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if ((s.b[478] && s.b[479]) && s.b[483]) {
            let assign16050_ad_e25053: A = {
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
            };
            let assign16050_ad_e25091: A = {
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
            };
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign16050_ad_e25053, (-3.24e17), s.ad_value(83), assign16050_ad_e25091, (-3.24e17));
        }

        if ((s.b[478] && s.b[479]) && s.b[483]) {
            s.store_scaled_mul(121, 136, 137, p.p182);
            s.store_scaled_mul(122, 136, 137, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.copy_ad(260, 128);
        }

        if ((s.b[478] && s.b[479]) && (!s.b[483])) {
            s.copy_ad(260, 100);
        }

        if (s.b[478] && s.b[479]) {
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p176);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p177);
            s.store_mul_scaled_abs_ad_rhs(136, 259, 1.0 / (p.p9), A::sub(s.ad_value(258), s.ad_value(260)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(260)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(90, 258, 258, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p174, 136, p.p174, 90, 1.0, 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(267), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 267, 90);
            s.store_sub(39, 258, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[484] = (s.v[136] < 200.0);
        s.v[484] = if s.b[484] { 1.0 } else { 0.0 };

        if ((s.b[478] && s.b[479]) && s.b[484]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[478] && s.b[479]) && (!s.b[484])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[478] && s.b[479]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[485] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[485] = if s.b[485] { 1.0 } else { 0.0 };

        if ((s.b[478] && s.b[479]) && s.b[485]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

    }

    pub(super) fn stamp_reactive_block_13(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[478] && s.b[479]) && s.b[485]) {
            let assign16510_ad_e25785: A = {
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
            };
            let assign16510_ad_e25823: A = {
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
            };
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign16510_ad_e25785, (-3.24e17), s.ad_value(83), assign16510_ad_e25823, (-3.24e17));
        }

        if ((s.b[478] && s.b[479]) && s.b[485]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 130, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_mul_scaled_powf_rhs(116, 136, p.p182, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p183, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if ((s.b[478] && s.b[479]) && s.b[485]) {
            let assign16660_ad_e26071: A = {
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
            };
            let assign16660_ad_e26109: A = {
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
            };
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign16660_ad_e26071, (-3.24e17), s.ad_value(83), assign16660_ad_e26109, (-3.24e17));
        }

        if ((s.b[478] && s.b[479]) && s.b[485]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p182, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p183, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(261, 128, 86);
        }

        if ((s.b[478] && s.b[479]) && (!s.b[485])) {
            s.store_add(261, 100, 86);
        }

        if (s.b[478] && s.b[479]) {
            s.store_scaled_add(262, 260, 261, 0.5);
            s.store_sub(263, 261, 260);
            s.store_mul_add_scaled_inputs3_offset_rhs(135, 263, s.ad_value(258), 1.0, s.ad_value(262), (-1.0), s.ad_value(83), 1.0, 0.0);
            s.store_mul_scaled_abs_ad_rhs(136, 259, 1.0 / (p.p9), A::sub(s.ad_value(258), s.ad_value(262)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);
            s.store_scaled_mul(96, 95, 259, (p.p4 * (p.p5 * 1.0 / (p.p174))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(268), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(263), (p.p25 * p.p25), s.ad_value(263)), 1.0);
            s.store_div(93, 98, 92);
            s.store_sub(90, 261, 260);
            s.store_add_scaled_inputs3_indices(91, 258, 1.0, 83, 1.0, 262, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 259, s.ad_value(258), ((p.p4 * p.p5) * p.p174), s.ad_value(262), (((-1.0)) * (((p.p4 * p.p5) * p.p174))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p174), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p239) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p238, 1.0);
            s.store_div_from_scalar(190, p.p237, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p173);
            s.store_mul_add_scaled_inputs3_offset_rhs(264, 191, s.ad_value(258), ((p.p4 * p.p5) * p.p174), s.ad_value(262), (((-1.0)) * (((p.p4 * p.p5) * p.p174))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p174), 0.0);
            s.store_add_scaled_inputs3_indices(136, 258, 1.0, 83, 1.0, 262, -1.0);
            s.store_add_scaled_inputs(90, 260, 0.3333333333333333, 261, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(263)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(263)), 263, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(265, 191, 258, (-(((p.p4 * p.p174) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p174) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p174) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p174) * p.p5) * 0.5)));
        }

        s.b[486] = (s.v[60] < 0.0);
        s.v[486] = if s.b[486] { 1.0 } else { 0.0 };

        if ((s.b[478] && s.b[479]) && s.b[486]) {
            s.store_sub_scaled_inputs(265, 264, (-1.0), 265, 1.0);
        }

        if (s.b[478] && (!s.b[479])) {
            s.store_scalar(264, 0.0);
            s.store_scalar(265, 0.0);
        }

        s.b[487] = (p.p153 != 0.0);
        s.v[487] = if s.b[487] { 1.0 } else { 0.0 };

        s.b[488] = (p.p153 == 1.0);
        s.v[488] = if s.b[488] { 1.0 } else { 0.0 };

        if (((!s.b[478]) && s.b[487]) && s.b[488]) {
            s.store_voltage(62, ctx, nodes, Some(9), Some(8));
        }

        if (((!s.b[478]) && s.b[487]) && (!s.b[488])) {
            s.store_voltage(62, ctx, nodes, Some(2), Some(8));
        }

        if ((!s.b[478]) && s.b[487]) {
            s.copy_ad(266, 62);
            s.store_scalar(146, (1.0 + p.p178));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_offset_scaled(88, 82, ((1.0 / (s.v[35])) * (p.p175)), (((((-1.0)) * (p.p175))) + (p.p172)));
            s.store_scalar(259, (p.p9 / p.p173));
            s.store_div_from_scalar_ad(136, p.p174, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p171), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 266, 0.5, 159, ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(266), s.ad_value(159)), A::sub(s.ad_value(266), s.ad_value(159))), 0.0001)), 0.5, 159, 1.0);
            s.store_sub(258, 160, 88);
            s.store_div_scaled_inputs_indices(84, 259, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 259, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 258, 258, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 258, 1.0, 83, 2.0);
        }

        s.b[489] = (s.v[136] < 200.0);
        s.v[489] = if s.b[489] { 1.0 } else { 0.0 };

        if (((!s.b[478]) && s.b[487]) && s.b[489]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(258), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[478]) && s.b[487]) && (!s.b[489])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(258), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[478]) && s.b[487]) {
            s.store_sub_div_rhs_indices(100, 258, 153, 99);
        }

        s.b[490] = ((((s.v[100] - s.v[258])) as f64).abs() > 1e-19);
        s.v[490] = if s.b[490] { 1.0 } else { 0.0 };

        if (((!s.b[478]) && s.b[487]) && s.b[490]) {
            s.store_sub(101, 258, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if (((!s.b[478]) && s.b[487]) && s.b[490]) {
            let assign17430_ad_e27199: A = {
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
            };
            let assign17430_ad_e27237: A = {
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
            };
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign17430_ad_e27199, (-3.24e17), s.ad_value(83), assign17430_ad_e27237, (-3.24e17));
        }

        if (((!s.b[478]) && s.b[487]) && s.b[490]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
        }

    }

    pub(super) fn stamp_reactive_block_14(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[478]) && s.b[487]) && s.b[490]) {
            s.store_sub(115, 258, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p182, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p183, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if (((!s.b[478]) && s.b[487]) && s.b[490]) {
            let assign17590_ad_e27512: A = {
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
            };
            let assign17590_ad_e27550: A = {
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
            };
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign17590_ad_e27512, (-3.24e17), s.ad_value(83), assign17590_ad_e27550, (-3.24e17));
        }

        if (((!s.b[478]) && s.b[487]) && s.b[490]) {
            s.store_scaled_mul(121, 136, 137, p.p182);
            s.store_scaled_mul(122, 136, 137, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.copy_ad(260, 128);
        }

        if (((!s.b[478]) && s.b[487]) && (!s.b[490])) {
            s.copy_ad(260, 100);
        }

        if ((!s.b[478]) && s.b[487]) {
            s.store_scalar(267, 0.0);
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p176);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p177);
            s.store_mul_scaled_abs_ad_rhs(136, 259, 1.0 / (p.p9), A::sub(s.ad_value(258), s.ad_value(260)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(260)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(90, 258, 258, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p174, 136, p.p174, 90, 1.0, 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(267), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 267, 90);
            s.store_sub(39, 258, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[491] = (s.v[136] < 200.0);
        s.v[491] = if s.b[491] { 1.0 } else { 0.0 };

        if (((!s.b[478]) && s.b[487]) && s.b[491]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[478]) && s.b[487]) && (!s.b[491])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[478]) && s.b[487]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[492] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[492] = if s.b[492] { 1.0 } else { 0.0 };

        if (((!s.b[478]) && s.b[487]) && s.b[492]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if (((!s.b[478]) && s.b[487]) && s.b[492]) {
            let assign18060_ad_e28295: A = {
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
            };
            let assign18060_ad_e28333: A = {
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
            };
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign18060_ad_e28295, (-3.24e17), s.ad_value(83), assign18060_ad_e28333, (-3.24e17));
        }

        if (((!s.b[478]) && s.b[487]) && s.b[492]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 130, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_mul_scaled_powf_rhs(116, 136, p.p182, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p183, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if (((!s.b[478]) && s.b[487]) && s.b[492]) {
            let assign18210_ad_e28596: A = {
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
            };
            let assign18210_ad_e28634: A = {
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
            };
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign18210_ad_e28596, (-3.24e17), s.ad_value(83), assign18210_ad_e28634, (-3.24e17));
        }

        if (((!s.b[478]) && s.b[487]) && s.b[492]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p182, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p183, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(261, 128, 86);
        }

        if (((!s.b[478]) && s.b[487]) && (!s.b[492])) {
            s.store_add(261, 100, 86);
        }

        if ((!s.b[478]) && s.b[487]) {
            s.store_scaled_add(262, 260, 261, 0.5);
            s.store_sub(263, 261, 260);
            s.store_sub(90, 261, 260);
            s.store_add_scaled_inputs3_indices(91, 258, 1.0, 83, 1.0, 262, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 259, s.ad_value(258), ((p.p4 * p.p5) * p.p174), s.ad_value(262), (((-1.0)) * (((p.p4 * p.p5) * p.p174))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p174), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p239) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p238, 1.0);
            s.store_div_from_scalar(190, p.p237, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p173);
            s.store_mul_add_scaled_inputs3_offset_rhs(264, 191, s.ad_value(258), ((p.p4 * p.p5) * p.p174), s.ad_value(262), (((-1.0)) * (((p.p4 * p.p5) * p.p174))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p174), 0.0);
            s.store_add_scaled_inputs3_indices(136, 258, 1.0, 83, 1.0, 262, -1.0);
            s.store_add_scaled_inputs(90, 260, 0.3333333333333333, 261, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(263)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(263)), 263, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(265, 191, 258, (-(((p.p4 * p.p174) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p174) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p174) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p174) * p.p5) * 0.5)));
        }

        if ((!s.b[478]) && (!s.b[487])) {
            s.store_scalar(264, 0.0);
            s.store_scalar(265, 0.0);
        }

        s.b[493] = (p.p149 == 0.0);
        s.v[493] = if s.b[493] { 1.0 } else { 0.0 };

        s.b[494] = (p.p154 != 0.0);
        s.v[494] = if s.b[494] { 1.0 } else { 0.0 };

        if (s.b[493] && s.b[494]) {
            s.store_voltage(65, ctx, nodes, Some(17), Some(16));
        }

    }

    pub(super) fn stamp_reactive_block_15(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[495] = (p.p154 == 1.0);
        s.v[495] = if s.b[495] { 1.0 } else { 0.0 };

        if ((s.b[493] && s.b[494]) && s.b[495]) {
            s.store_voltage(66, ctx, nodes, Some(9), Some(16));
            s.store_voltage(67, ctx, nodes, Some(9), Some(17));
        }

        if ((s.b[493] && s.b[494]) && (!s.b[495])) {
            s.store_voltage(66, ctx, nodes, Some(2), Some(16));
            s.store_voltage(67, ctx, nodes, Some(2), Some(17));
        }

        if (s.b[493] && s.b[494]) {
            s.store_scalar(64, 1.0);
        }

        s.b[496] = (s.v[65] < 0.0);
        s.v[496] = if s.b[496] { 1.0 } else { 0.0 };

        if ((s.b[493] && s.b[494]) && s.b[496]) {
            s.store_scalar(64, (-1.0));
            s.store_mul(279, 64, 65);
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
            s.store_sub_ad(88, A::sub_from_scalar(p.p185, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p188)), (((-1.0)) * (p.p188)))), A::div_scaled_inputs(s.ad_value(280), (p.p194 * p.p193), A::sqrt(A::offset(A::square(s.ad_value(280)), (p.p194 * p.p194))), 1.0));
            s.store_scalar(271, (p.p9 / p.p186));
            s.store_div_from_scalar_ad(136, p.p187, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p184), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 278, 0.5, 159, ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(278), s.ad_value(159)), A::sub(s.ad_value(278), s.ad_value(159))), 0.0001)), 0.5, 159, 1.0);
            s.store_sub(270, 160, 88);
            s.store_div_scaled_inputs_indices(84, 271, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 271, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 270, 270, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p195 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 270, 1.0, 83, 2.0);
        }

        s.b[497] = (s.v[136] < 200.0);
        s.v[497] = if s.b[497] { 1.0 } else { 0.0 };

        if ((s.b[493] && s.b[494]) && s.b[497]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(270), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[493] && s.b[494]) && (!s.b[497])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(270), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[493] && s.b[494]) {
            s.store_sub_div_rhs_indices(100, 270, 153, 99);
        }

        s.b[498] = ((((s.v[100] - s.v[270])) as f64).abs() > 1e-19);
        s.v[498] = if s.b[498] { 1.0 } else { 0.0 };

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
            let assign18980_ad_e29685: A = {
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
            };
            let assign18980_ad_e29723: A = {
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
            };
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign18980_ad_e29685, (-3.24e17), s.ad_value(83), assign18980_ad_e29723, (-3.24e17));
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
            let assign19140_ad_e29982: A = {
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
            };
            let assign19140_ad_e30020: A = {
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
            };
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign19140_ad_e29982, (-3.24e17), s.ad_value(83), assign19140_ad_e30020, (-3.24e17));
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
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 279, 90);
            s.store_sub(39, 270, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p195 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[499] = (s.v[136] < 200.0);
        s.v[499] = if s.b[499] { 1.0 } else { 0.0 };

        if ((s.b[493] && s.b[494]) && s.b[499]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[493] && s.b[494]) && (!s.b[499])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[493] && s.b[494]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[500] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[500] = if s.b[500] { 1.0 } else { 0.0 };

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

    }

    pub(super) fn stamp_reactive_block_16(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[493] && s.b[494]) && s.b[500]) {
            let assign19600_ad_e30714: A = {
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
            };
            let assign19600_ad_e30752: A = {
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
            };
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign19600_ad_e30714, (-3.24e17), s.ad_value(83), assign19600_ad_e30752, (-3.24e17));
        }

        if ((s.b[493] && s.b[494]) && s.b[500]) {
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

        if ((s.b[493] && s.b[494]) && s.b[500]) {
            let assign19750_ad_e31000: A = {
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
            };
            let assign19750_ad_e31038: A = {
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
            };
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign19750_ad_e31000, (-3.24e17), s.ad_value(83), assign19750_ad_e31038, (-3.24e17));
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
        s.v[501] = if s.b[501] { 1.0 } else { 0.0 };

        if ((s.b[493] && s.b[494]) && s.b[501]) {
            s.store_sub_scaled_inputs(277, 276, (-1.0), 277, 1.0);
        }

        if (s.b[493] && (!s.b[494])) {
            s.store_scalar(276, 0.0);
            s.store_scalar(277, 0.0);
        }

        s.b[502] = (p.p154 != 0.0);
        s.v[502] = if s.b[502] { 1.0 } else { 0.0 };

        s.b[503] = (p.p154 == 1.0);
        s.v[503] = if s.b[503] { 1.0 } else { 0.0 };

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
            s.store_div_from_scalar_ad(136, p.p187, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p184), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 278, 0.5, 159, ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(278), s.ad_value(159)), A::sub(s.ad_value(278), s.ad_value(159))), 0.0001)), 0.5, 159, 1.0);
            s.store_sub(270, 160, 88);
            s.store_div_scaled_inputs_indices(84, 271, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 271, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 270, 270, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p195 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 270, 1.0, 83, 2.0);
        }

        s.b[504] = (s.v[136] < 200.0);
        s.v[504] = if s.b[504] { 1.0 } else { 0.0 };

        if (((!s.b[493]) && s.b[502]) && s.b[504]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(270), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[493]) && s.b[502]) && (!s.b[504])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(270), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[493]) && s.b[502]) {
            s.store_sub_div_rhs_indices(100, 270, 153, 99);
        }

        s.b[505] = ((((s.v[100] - s.v[270])) as f64).abs() > 1e-19);
        s.v[505] = if s.b[505] { 1.0 } else { 0.0 };

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
            let assign20520_ad_e32128: A = {
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
            };
            let assign20520_ad_e32166: A = {
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
            };
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign20520_ad_e32128, (-3.24e17), s.ad_value(83), assign20520_ad_e32166, (-3.24e17));
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
        }

    }

    pub(super) fn stamp_reactive_block_17(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[493]) && s.b[502]) && s.b[505]) {
            s.store_sub(115, 270, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p195, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p196, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if (((!s.b[493]) && s.b[502]) && s.b[505]) {
            let assign20680_ad_e32441: A = {
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
            };
            let assign20680_ad_e32479: A = {
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
            };
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign20680_ad_e32441, (-3.24e17), s.ad_value(83), assign20680_ad_e32479, (-3.24e17));
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
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 279, 90);
            s.store_sub(39, 270, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p195 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[506] = (s.v[136] < 200.0);
        s.v[506] = if s.b[506] { 1.0 } else { 0.0 };

        if (((!s.b[493]) && s.b[502]) && s.b[506]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[493]) && s.b[502]) && (!s.b[506])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[493]) && s.b[502]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[507] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[507] = if s.b[507] { 1.0 } else { 0.0 };

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
            let assign21150_ad_e33224: A = {
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
            };
            let assign21150_ad_e33262: A = {
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
            };
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign21150_ad_e33224, (-3.24e17), s.ad_value(83), assign21150_ad_e33262, (-3.24e17));
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
            let assign21300_ad_e33525: A = {
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
            };
            let assign21300_ad_e33563: A = {
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
            };
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign21300_ad_e33525, (-3.24e17), s.ad_value(83), assign21300_ad_e33563, (-3.24e17));
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
        s.v[508] = if s.b[508] { 1.0 } else { 0.0 };

        s.b[509] = (p.p155 != 0.0);
        s.v[509] = if s.b[509] { 1.0 } else { 0.0 };

        if (s.b[508] && s.b[509]) {
            s.store_voltage(69, ctx, nodes, Some(20), Some(21));
        }

    }

    pub(super) fn stamp_reactive_block_18(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[510] = (p.p155 == 1.0);
        s.v[510] = if s.b[510] { 1.0 } else { 0.0 };

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
        s.v[511] = if s.b[511] { 1.0 } else { 0.0 };

        if ((s.b[508] && s.b[509]) && s.b[511]) {
            s.store_scalar(68, (-1.0));
            s.store_mul(291, 68, 69);
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
            s.store_sub_ad(88, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p188)), (((((-1.0)) * (p.p188))) + (p.p185))), A::div_scaled_inputs(s.ad_value(292), (p.p194 * p.p193), A::sqrt(A::offset(A::square(s.ad_value(292)), (p.p194 * p.p194))), 1.0));
            s.store_scalar(283, (p.p9 / p.p186));
            s.store_div_from_scalar_ad(136, p.p187, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p184), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 290, 0.5, 159, ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(290), s.ad_value(159)), A::sub(s.ad_value(290), s.ad_value(159))), 0.0001)), 0.5, 159, 1.0);
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

        s.b[512] = (s.v[136] < 200.0);
        s.v[512] = if s.b[512] { 1.0 } else { 0.0 };

        if ((s.b[508] && s.b[509]) && s.b[512]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(282), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[508] && s.b[509]) && (!s.b[512])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(282), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[508] && s.b[509]) {
            s.store_sub_div_rhs_indices(100, 282, 153, 99);
        }

        s.b[513] = ((((s.v[100] - s.v[282])) as f64).abs() > 1e-19);
        s.v[513] = if s.b[513] { 1.0 } else { 0.0 };

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
            let assign22070_ad_e34614: A = {
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
            };
            let assign22070_ad_e34652: A = {
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
            };
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign22070_ad_e34614, (-3.24e17), s.ad_value(83), assign22070_ad_e34652, (-3.24e17));
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
            let assign22230_ad_e34911: A = {
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
            };
            let assign22230_ad_e34949: A = {
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
            };
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign22230_ad_e34911, (-3.24e17), s.ad_value(83), assign22230_ad_e34949, (-3.24e17));
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
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
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

        s.b[514] = (s.v[136] < 200.0);
        s.v[514] = if s.b[514] { 1.0 } else { 0.0 };

        if ((s.b[508] && s.b[509]) && s.b[514]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[508] && s.b[509]) && (!s.b[514])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[508] && s.b[509]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[515] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[515] = if s.b[515] { 1.0 } else { 0.0 };

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

    }
}
