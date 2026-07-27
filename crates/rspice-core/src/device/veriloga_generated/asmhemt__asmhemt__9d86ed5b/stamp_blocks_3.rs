#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_48(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[493] && s.b[494]) {s.store_sub_ad(88, A::sub_from_scalar(p[185], A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p[188])), (((-1.0)) * (p[188])))), A::div_scaled_inputs(s.ad_value(280), (p[194] * p[193]), A::sqrt_square_offset(s.ad_value(280), (p[194] * p[194])), 1.0));s.store_scalar(271, (p[9] / p[186]));s.store_div_scalar_by_product_indices(136, p[187], 83, 83, (((2.0 * p[4]) * 1.602176634e-19) * 3.24e17));s.store_add_scaled_product_mixed_iia(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p[184]), 1.0);s.store_add_scaled_inputs4_mixed_iiai(160, 278, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(278), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);s.store_sub(270, 160, 88);s.store_div_scaled_inputs_indices(84, 271, 1.0, 83, (1.602176634e-19 * 3.24e17));s.store_div_from_scalar(150, 2.718281828459045, 84);s.store_div_from_scalar(151, 1.0, 84);s.store_primal_scale(99, 271, 6.241509074460763e18);s.store_scaled_add_sqrt_square_offset_rhs(154, 270, 270, ((4.0 * 0.3) * 0.3), 0.5);s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);let t0: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, t0, (-(p[195] / 3.0)), A::add_scaled_offset_product_rhs(t0, ((2.0 * p[195]) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 270, 1.0, 83, 2.0);}
        s.b[497] = (s.v[136] < 200.0);s.store_scalar(497, if s.b[497] { 1.0 } else { 0.0 });
        if ((s.b[493] && s.b[494]) && s.b[497]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(270), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((s.b[493] && s.b[494]) && (!s.b[497])) {s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(270), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (s.b[493] && s.b[494]) {s.store_sub_div_rhs_indices(100, 270, 153, 99);}
        s.b[498] = ((((s.v[100] - s.v[270])) as f64).abs() > 1e-19);s.store_scalar(498, if s.b[498] { 1.0 } else { 0.0 });
        if ((s.b[493] && s.b[494]) && s.b[498]) {s.store_sub(101, 270, 100);s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p[195]);s.store_scaled_mul(103, 136, 90, p[196]);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_49(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[493] && s.b[494]) && s.b[498]) {
            s.store_add_scaled_products3_mixed_iiiaia(106, 99, 101, 1.0, 83, {
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
            }, (-3.24e17), 83, {
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
        if ((s.b[493] && s.b[494]) && s.b[498]) {s.store_scaled_mul(107, 136, 91, p[195]);s.store_scaled_mul(108, 136, 91, p[196]);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 270, 114);s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);s.store_powf(137, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(116, 136, p[195], 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p[196], 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
        if ((s.b[493] && s.b[494]) && s.b[498]) {
            s.store_add_scaled_products3_mixed_iiiaia(120, 99, 115, 1.0, 83, {
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
            }, (-3.24e17), 83, {
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
        if ((s.b[493] && s.b[494]) && s.b[498]) {s.store_scaled_mul(121, 136, 137, p[195]);s.store_scaled_mul(122, 136, 137, p[196]);s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.copy_ad(272, 128);}
        if ((s.b[493] && s.b[494]) && (!s.b[498])) {s.copy_ad(272, 100);}
        if (s.b[493] && s.b[494]) {s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p[20], p[189]);s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p[19], p[190]);s.store_mul_scaled_abs_ad_rhs(136, 271, 1.0 / (p[9]), A::sub(s.ad_value(270), s.ad_value(272)));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(272)), (s.v[81] / p[9]));s.store_div_mixed_ia(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p[14], A::square(s.ad_value(136)), p[15], s.ad_value(90), p[16], 1.0));s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);s.store_scaled_add_sqrt_square_offset_rhs(90, 270, 270, ((4.0 * 0.3) * 0.3), 0.5);s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p[187], 136, p[187], 90, 1.0, 1.0);s.store_powf_ad(136, A::div(s.ad_value(279), s.ad_value(85)), p[18]);s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p[18]));s.store_mul(86, 279, 90);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_50(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[493] && s.b[494]) {s.store_sub(39, 270, 86);s.copy_ad(130, 39);s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);s.copy_ad(154, 131);s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);let t1: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, t1, (-(p[195] / 3.0)), A::add_scaled_offset_product_rhs(t1, ((2.0 * p[195]) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);}
        s.b[499] = (s.v[136] < 200.0);s.store_scalar(499, if s.b[499] { 1.0 } else { 0.0 });
        if ((s.b[493] && s.b[494]) && s.b[499]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((s.b[493] && s.b[494]) && (!s.b[499])) {s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (s.b[493] && s.b[494]) {s.store_sub_div_rhs_indices(100, 130, 156, 99);}
        s.b[500] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);s.store_scalar(500, if s.b[500] { 1.0 } else { 0.0 });
        if ((s.b[493] && s.b[494]) && s.b[500]) {s.store_sub(101, 130, 100);s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p[195]);s.store_scaled_mul(103, 136, 90, p[196]);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
        if ((s.b[493] && s.b[494]) && s.b[500]) {
            s.store_add_scaled_products3_mixed_iiiaia(106, 99, 101, 1.0, 83, {
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
            }, (-3.24e17), 83, {
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
        if ((s.b[493] && s.b[494]) && s.b[500]) {s.store_scaled_mul(107, 136, 91, p[195]);s.store_scaled_mul(108, 136, 91, p[196]);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 130, 114);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_51(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[493] && s.b[494]) && s.b[500]) {s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);s.store_mul_scaled_powf_rhs(116, 136, p[195], 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p[196], 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
        if ((s.b[493] && s.b[494]) && s.b[500]) {
            s.store_add_scaled_products3_mixed_iiiaia(120, 99, 115, 1.0, 83, {
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
            }, (-3.24e17), 83, {
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
        if ((s.b[493] && s.b[494]) && s.b[500]) {s.store_mul_scaled_powf_rhs(121, 136, p[195], 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(122, 136, p[196], 115, (-0.3333333333333333));s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.store_add(273, 128, 86);}
        if ((s.b[493] && s.b[494]) && (!s.b[500])) {s.store_add(273, 100, 86);}
        if (s.b[493] && s.b[494]) {s.store_scaled_add(274, 272, 273, 0.5);s.store_sub(275, 273, 272);s.store_mul_add_scaled_inputs3_offset_rhs_indices(135, 275, 270, 1.0, 274, (-1.0), 83, 1.0, 0.0);s.store_mul_scaled_abs_ad_rhs(136, 271, 1.0 / (p[9]), A::sub(s.ad_value(270), s.ad_value(274)));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p[9]));s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p[14], 1.0), 1.0, s.ad_value(136), s.ad_value(136), p[15]), 1.0, 90, p[16]);s.store_scaled_mul(96, 95, 271, (p[4] * (p[5] * 1.0 / (p[187]))));s.store_mul_scale_offset_mixed_ia(98, 96, A::sub_scaled_inputs(s.ad_value(280), p[21], s.ad_value(86), p[21]), 1.0, 1.0);s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(275), (p[25] * p[25]), s.ad_value(275)), 1.0);s.store_div(93, 98, 92);s.store_mul(281, 93, 135);s.store_sub(90, 273, 272);s.store_add_scaled_inputs3_indices(91, 270, 1.0, 83, 1.0, 274, -1.0);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(137, 271, 270, ((p[4] * p[5]) * p[187]), 274, (((-1.0)) * (((p[4] * p[5]) * p[187]))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p[4] * p[5]) * p[187]), 0.0);s.store_scale(188, 137, (1.0 / (p[242]) * 1e26));s.store_offset_powf_ad(189, s.ad_value(188), p[241], 1.0);s.store_div_from_scalar(190, p[240], 189);s.store_div_from_scalar_offset_input(191, p[9], 190, p[186]);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(276, 191, 270, ((p[4] * p[5]) * p[187]), 274, (((-1.0)) * (((p[4] * p[5]) * p[187]))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p[4] * p[5]) * p[187]), 0.0);s.store_add_scaled_inputs3_indices(136, 270, 1.0, 83, 1.0, 274, -1.0);s.store_add_scaled_inputs(90, 272, 0.3333333333333333, 273, (2.0 * 0.3333333333333333));s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(275)), (1.0 / 12.0), 136, 1.0);s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(275)), 275, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);s.store_mul_add_scaled_inputs4_indices_rhs(277, 191, 270, (-(((p[4] * p[187]) * p[5]) * 0.5)), 90, (((-1.0)) * ((-(((p[4] * p[187]) * p[5]) * 0.5)))), 91, (-(((p[4] * p[187]) * p[5]) * 0.5)), 137, (-(((p[4] * p[187]) * p[5]) * 0.5)));}
        s.b[501] = (s.v[64] < 0.0);s.store_scalar(501, if s.b[501] { 1.0 } else { 0.0 });
        if ((s.b[493] && s.b[494]) && s.b[501]) {s.store_sub_scaled_inputs(277, 276, (-1.0), 277, 1.0);}
        if (s.b[493] && (!s.b[494])) {s.store_scalar(276, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_52(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[493] && (!s.b[494])) {s.store_scalar(277, 0.0);}
        s.b[502] = (p[154] != 0.0);s.store_scalar(502, if s.b[502] { 1.0 } else { 0.0 });s.b[503] = (p[154] == 1.0);s.store_scalar(503, if s.b[503] { 1.0 } else { 0.0 });
        if (((!s.b[493]) && s.b[502]) && s.b[503]) {s.store_voltage(66, ctx, nodes, Some(9), Some(7));}
        if (((!s.b[493]) && s.b[502]) && (!s.b[503])) {s.store_voltage(66, ctx, nodes, Some(2), Some(7));}
        if ((!s.b[493]) && s.b[502]) {s.copy_ad(278, 66);s.store_scalar(146, (1.0 + p[191]));s.store_scaled_mul(83, 82, 146, 8.617087e-5);s.store_sub_from_scalar_ad(88, p[185], A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p[188])), (((-1.0)) * (p[188]))));s.store_scalar(271, (p[9] / p[186]));s.store_div_scalar_by_product_indices(136, p[187], 83, 83, (((2.0 * p[4]) * 1.602176634e-19) * 3.24e17));s.store_add_scaled_product_mixed_iia(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p[184]), 1.0);s.store_add_scaled_inputs4_mixed_iiai(160, 278, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(278), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);s.store_sub(270, 160, 88);s.store_div_scaled_inputs_indices(84, 271, 1.0, 83, (1.602176634e-19 * 3.24e17));s.store_div_from_scalar(150, 2.718281828459045, 84);s.store_div_from_scalar(151, 1.0, 84);s.store_primal_scale(99, 271, 6.241509074460763e18);s.store_scaled_add_sqrt_square_offset_rhs(154, 270, 270, ((4.0 * 0.3) * 0.3), 0.5);s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);let t2: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, t2, (-(p[195] / 3.0)), A::add_scaled_offset_product_rhs(t2, ((2.0 * p[195]) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 270, 1.0, 83, 2.0);}
        s.b[504] = (s.v[136] < 200.0);s.store_scalar(504, if s.b[504] { 1.0 } else { 0.0 });
        if (((!s.b[493]) && s.b[502]) && s.b[504]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(270), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (((!s.b[493]) && s.b[502]) && (!s.b[504])) {s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(270), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((!s.b[493]) && s.b[502]) {s.store_sub_div_rhs_indices(100, 270, 153, 99);}
        s.b[505] = ((((s.v[100] - s.v[270])) as f64).abs() > 1e-19);s.store_scalar(505, if s.b[505] { 1.0 } else { 0.0 });
        if (((!s.b[493]) && s.b[502]) && s.b[505]) {s.store_sub(101, 270, 100);s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p[195]);s.store_scaled_mul(103, 136, 90, p[196]);s.store_sub_div_same_denominator(104, 100, 102, 83);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_53(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[493]) && s.b[502]) && s.b[505]) {s.store_sub_div_same_denominator(105, 100, 103, 83);}
        if (((!s.b[493]) && s.b[502]) && s.b[505]) {
            s.store_add_scaled_products3_mixed_iiiaia(106, 99, 101, 1.0, 83, {
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
            }, (-3.24e17), 83, {
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
        if (((!s.b[493]) && s.b[502]) && s.b[505]) {s.store_scaled_mul(107, 136, 91, p[195]);s.store_scaled_mul(108, 136, 91, p[196]);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 270, 114);s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);s.store_powf(137, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(116, 136, p[195], 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p[196], 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
        if (((!s.b[493]) && s.b[502]) && s.b[505]) {
            s.store_add_scaled_products3_mixed_iiiaia(120, 99, 115, 1.0, 83, {
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
            }, (-3.24e17), 83, {
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
        if (((!s.b[493]) && s.b[502]) && s.b[505]) {s.store_scaled_mul(121, 136, 137, p[195]);s.store_scaled_mul(122, 136, 137, p[196]);s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.copy_ad(272, 128);}
        if (((!s.b[493]) && s.b[502]) && (!s.b[505])) {s.copy_ad(272, 100);}
        if ((!s.b[493]) && s.b[502]) {s.store_scalar(279, 0.0);s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p[20], p[189]);s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p[19], p[190]);s.store_mul_scaled_abs_ad_rhs(136, 271, 1.0 / (p[9]), A::sub(s.ad_value(270), s.ad_value(272)));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(272)), (s.v[81] / p[9]));s.store_div_mixed_ia(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p[14], A::square(s.ad_value(136)), p[15], s.ad_value(90), p[16], 1.0));s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);s.store_scaled_add_sqrt_square_offset_rhs(90, 270, 270, ((4.0 * 0.3) * 0.3), 0.5);s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p[187], 136, p[187], 90, 1.0, 1.0);s.store_powf_ad(136, A::div(s.ad_value(279), s.ad_value(85)), p[18]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_54(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[493]) && s.b[502]) {s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p[18]));s.store_mul(86, 279, 90);s.store_sub(39, 270, 86);s.copy_ad(130, 39);s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);s.copy_ad(154, 131);s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);let t3: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, t3, (-(p[195] / 3.0)), A::add_scaled_offset_product_rhs(t3, ((2.0 * p[195]) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);}
        s.b[506] = (s.v[136] < 200.0);s.store_scalar(506, if s.b[506] { 1.0 } else { 0.0 });
        if (((!s.b[493]) && s.b[502]) && s.b[506]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (((!s.b[493]) && s.b[502]) && (!s.b[506])) {s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((!s.b[493]) && s.b[502]) {s.store_sub_div_rhs_indices(100, 130, 156, 99);}
        s.b[507] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);s.store_scalar(507, if s.b[507] { 1.0 } else { 0.0 });
        if (((!s.b[493]) && s.b[502]) && s.b[507]) {s.store_sub(101, 130, 100);s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p[195]);s.store_scaled_mul(103, 136, 90, p[196]);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
        if (((!s.b[493]) && s.b[502]) && s.b[507]) {
            s.store_add_scaled_products3_mixed_iiiaia(106, 99, 101, 1.0, 83, {
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
            }, (-3.24e17), 83, {
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
        if (((!s.b[493]) && s.b[502]) && s.b[507]) {s.store_scaled_mul(107, 136, 91, p[195]);s.store_scaled_mul(108, 136, 91, p[196]);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_55(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[493]) && s.b[502]) && s.b[507]) {s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 130, 114);s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);s.store_mul_scaled_powf_rhs(116, 136, p[195], 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p[196], 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
        if (((!s.b[493]) && s.b[502]) && s.b[507]) {
            s.store_add_scaled_products3_mixed_iiiaia(120, 99, 115, 1.0, 83, {
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
            }, (-3.24e17), 83, {
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
        if (((!s.b[493]) && s.b[502]) && s.b[507]) {s.store_mul_scaled_powf_rhs(121, 136, p[195], 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(122, 136, p[196], 115, (-0.3333333333333333));s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.store_add(273, 128, 86);}
        if (((!s.b[493]) && s.b[502]) && (!s.b[507])) {s.store_add(273, 100, 86);}
        if ((!s.b[493]) && s.b[502]) {s.store_scaled_add(274, 272, 273, 0.5);s.store_sub(275, 273, 272);s.store_sub(90, 273, 272);s.store_add_scaled_inputs3_indices(91, 270, 1.0, 83, 1.0, 274, -1.0);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(137, 271, 270, ((p[4] * p[5]) * p[187]), 274, (((-1.0)) * (((p[4] * p[5]) * p[187]))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p[4] * p[5]) * p[187]), 0.0);s.store_scale(188, 137, (1.0 / (p[242]) * 1e26));s.store_offset_powf_ad(189, s.ad_value(188), p[241], 1.0);s.store_div_from_scalar(190, p[240], 189);s.store_div_from_scalar_offset_input(191, p[9], 190, p[186]);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(276, 191, 270, ((p[4] * p[5]) * p[187]), 274, (((-1.0)) * (((p[4] * p[5]) * p[187]))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p[4] * p[5]) * p[187]), 0.0);s.store_add_scaled_inputs3_indices(136, 270, 1.0, 83, 1.0, 274, -1.0);s.store_add_scaled_inputs(90, 272, 0.3333333333333333, 273, (2.0 * 0.3333333333333333));s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(275)), (1.0 / 12.0), 136, 1.0);s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(275)), 275, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);s.store_mul_add_scaled_inputs4_indices_rhs(277, 191, 270, (-(((p[4] * p[187]) * p[5]) * 0.5)), 90, (((-1.0)) * ((-(((p[4] * p[187]) * p[5]) * 0.5)))), 91, (-(((p[4] * p[187]) * p[5]) * 0.5)), 137, (-(((p[4] * p[187]) * p[5]) * 0.5)));}
        if ((!s.b[493]) && (!s.b[502])) {s.store_scalar(276, 0.0);s.store_scalar(277, 0.0);}
        s.b[508] = (p[149] == 0.0);s.store_scalar(508, if s.b[508] { 1.0 } else { 0.0 });s.b[509] = (p[155] != 0.0);s.store_scalar(509, if s.b[509] { 1.0 } else { 0.0 });
        if (s.b[508] && s.b[509]) {s.store_voltage(69, ctx, nodes, Some(20), Some(21));}
        s.b[510] = (p[155] == 1.0);s.store_scalar(510, if s.b[510] { 1.0 } else { 0.0 });
        if ((s.b[508] && s.b[509]) && s.b[510]) {s.store_voltage(70, ctx, nodes, Some(9), Some(21));s.store_voltage(71, ctx, nodes, Some(9), Some(20));}
        if ((s.b[508] && s.b[509]) && (!s.b[510])) {s.store_voltage(70, ctx, nodes, Some(2), Some(21));s.store_voltage(71, ctx, nodes, Some(2), Some(20));}
        if (s.b[508] && s.b[509]) {s.store_scalar(68, 1.0);}
        s.b[511] = (s.v[69] < 0.0);s.store_scalar(511, if s.b[511] { 1.0 } else { 0.0 });
        if ((s.b[508] && s.b[509]) && s.b[511]) {s.store_scalar(68, (-1.0));s.store_mul(291, 68, 69);s.copy_ad(290, 71);}
        if ((s.b[508] && s.b[509]) && (!s.b[511])) {s.copy_ad(291, 69);s.copy_ad(290, 70);}
        if (s.b[508] && s.b[509]) {s.store_offset_sqrt_ad(292, A::offset(A::square(s.ad_value(291)), 0.01), (-0.1));s.store_offset_scaled(146, 292, p[192], (1.0 + p[191]));s.store_scaled_mul(83, 82, 146, 8.617087e-5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_56(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[508] && s.b[509]) {s.store_sub_ad(88, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p[188])), (((((-1.0)) * (p[188]))) + (p[185]))), A::div_scaled_inputs(s.ad_value(292), (p[194] * p[193]), A::sqrt_square_offset(s.ad_value(292), (p[194] * p[194])), 1.0));s.store_scalar(283, (p[9] / p[186]));s.store_div_scalar_by_product_indices(136, p[187], 83, 83, (((2.0 * p[4]) * 1.602176634e-19) * 3.24e17));s.store_add_scaled_product_mixed_iia(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p[184]), 1.0);s.store_add_scaled_inputs4_mixed_iiai(160, 290, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(290), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);s.store_sub(282, 160, 88);s.store_div_scaled_inputs_indices(84, 283, 1.0, 83, (1.602176634e-19 * 3.24e17));s.store_div_from_scalar(150, 2.718281828459045, 84);s.store_div_from_scalar(151, 1.0, 84);s.store_primal_scale(99, 283, 6.241509074460763e18);s.store_scaled_add_sqrt_square_offset_rhs(154, 282, 282, ((4.0 * 0.3) * 0.3), 0.5);s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);let t4: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, t4, (-(p[195] / 3.0)), A::add_scaled_offset_product_rhs(t4, ((2.0 * p[195]) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 282, 1.0, 83, 2.0);}
        s.b[512] = (s.v[136] < 200.0);s.store_scalar(512, if s.b[512] { 1.0 } else { 0.0 });
        if ((s.b[508] && s.b[509]) && s.b[512]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(282), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((s.b[508] && s.b[509]) && (!s.b[512])) {s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(282), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (s.b[508] && s.b[509]) {s.store_sub_div_rhs_indices(100, 282, 153, 99);}
        s.b[513] = ((((s.v[100] - s.v[282])) as f64).abs() > 1e-19);s.store_scalar(513, if s.b[513] { 1.0 } else { 0.0 });
        if ((s.b[508] && s.b[509]) && s.b[513]) {s.store_sub(101, 282, 100);s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p[195]);s.store_scaled_mul(103, 136, 90, p[196]);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_57(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[508] && s.b[509]) && s.b[513]) {
            s.store_add_scaled_products3_mixed_iiiaia(106, 99, 101, 1.0, 83, {
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
            }, (-3.24e17), 83, {
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
        if ((s.b[508] && s.b[509]) && s.b[513]) {s.store_scaled_mul(107, 136, 91, p[195]);s.store_scaled_mul(108, 136, 91, p[196]);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 282, 114);s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);s.store_powf(137, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(116, 136, p[195], 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p[196], 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
        if ((s.b[508] && s.b[509]) && s.b[513]) {
            s.store_add_scaled_products3_mixed_iiiaia(120, 99, 115, 1.0, 83, {
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
            }, (-3.24e17), 83, {
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
        if ((s.b[508] && s.b[509]) && s.b[513]) {s.store_scaled_mul(121, 136, 137, p[195]);s.store_scaled_mul(122, 136, 137, p[196]);s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.copy_ad(284, 128);}
        if ((s.b[508] && s.b[509]) && (!s.b[513])) {s.copy_ad(284, 100);}
        if (s.b[508] && s.b[509]) {s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p[20], p[189]);s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p[19], p[190]);s.store_mul_scaled_abs_ad_rhs(136, 283, 1.0 / (p[9]), A::sub(s.ad_value(282), s.ad_value(284)));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(284)), (s.v[81] / p[9]));s.store_div_mixed_ia(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p[14], A::square(s.ad_value(136)), p[15], s.ad_value(90), p[16], 1.0));s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);s.store_scaled_add_sqrt_square_offset_rhs(90, 282, 282, ((4.0 * 0.3) * 0.3), 0.5);s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p[187], 136, p[187], 90, 1.0, 1.0);s.store_powf_ad(136, A::div(s.ad_value(291), s.ad_value(85)), p[18]);s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p[18]));s.store_mul(86, 291, 90);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_58(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[508] && s.b[509]) {s.store_sub(39, 282, 86);s.copy_ad(130, 39);s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);s.copy_ad(154, 131);s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);let t5: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, t5, (-(p[195] / 3.0)), A::add_scaled_offset_product_rhs(t5, ((2.0 * p[195]) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);}
        s.b[514] = (s.v[136] < 200.0);s.store_scalar(514, if s.b[514] { 1.0 } else { 0.0 });
        if ((s.b[508] && s.b[509]) && s.b[514]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((s.b[508] && s.b[509]) && (!s.b[514])) {s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (s.b[508] && s.b[509]) {s.store_sub_div_rhs_indices(100, 130, 156, 99);}
        s.b[515] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);s.store_scalar(515, if s.b[515] { 1.0 } else { 0.0 });
        if ((s.b[508] && s.b[509]) && s.b[515]) {s.store_sub(101, 130, 100);s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p[195]);s.store_scaled_mul(103, 136, 90, p[196]);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
        if ((s.b[508] && s.b[509]) && s.b[515]) {
            s.store_add_scaled_products3_mixed_iiiaia(106, 99, 101, 1.0, 83, {
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
            }, (-3.24e17), 83, {
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
        if ((s.b[508] && s.b[509]) && s.b[515]) {s.store_scaled_mul(107, 136, 91, p[195]);s.store_scaled_mul(108, 136, 91, p[196]);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 130, 114);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_59(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[508] && s.b[509]) && s.b[515]) {s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);s.store_mul_scaled_powf_rhs(116, 136, p[195], 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p[196], 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
        if ((s.b[508] && s.b[509]) && s.b[515]) {
            s.store_add_scaled_products3_mixed_iiiaia(120, 99, 115, 1.0, 83, {
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
            }, (-3.24e17), 83, {
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
        if ((s.b[508] && s.b[509]) && s.b[515]) {s.store_mul_scaled_powf_rhs(121, 136, p[195], 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(122, 136, p[196], 115, (-0.3333333333333333));s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.store_add(285, 128, 86);}
        if ((s.b[508] && s.b[509]) && (!s.b[515])) {s.store_add(285, 100, 86);}
        if (s.b[508] && s.b[509]) {s.store_scaled_add(286, 284, 285, 0.5);s.store_sub(287, 285, 284);s.store_mul_add_scaled_inputs3_offset_rhs_indices(135, 287, 282, 1.0, 286, (-1.0), 83, 1.0, 0.0);s.store_mul_scaled_abs_ad_rhs(136, 283, 1.0 / (p[9]), A::sub(s.ad_value(282), s.ad_value(286)));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p[9]));s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p[14], 1.0), 1.0, s.ad_value(136), s.ad_value(136), p[15]), 1.0, 90, p[16]);s.store_scaled_mul(96, 95, 283, (p[4] * (p[5] * 1.0 / (p[187]))));s.store_mul_scale_offset_mixed_ia(98, 96, A::sub_scaled_inputs(s.ad_value(292), p[21], s.ad_value(86), p[21]), 1.0, 1.0);s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(287), (p[25] * p[25]), s.ad_value(287)), 1.0);s.store_div(93, 98, 92);s.store_mul(293, 93, 135);s.store_sub(90, 285, 284);s.store_add_scaled_inputs3_indices(91, 282, 1.0, 83, 1.0, 286, -1.0);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(137, 283, 282, ((p[4] * p[5]) * p[187]), 286, (((-1.0)) * (((p[4] * p[5]) * p[187]))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p[4] * p[5]) * p[187]), 0.0);s.store_scale(188, 137, (1.0 / (p[242]) * 1e26));s.store_offset_powf_ad(189, s.ad_value(188), p[241], 1.0);s.store_div_from_scalar(190, p[240], 189);s.store_div_from_scalar_offset_input(191, p[9], 190, p[186]);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(288, 191, 282, ((p[4] * p[5]) * p[187]), 286, (((-1.0)) * (((p[4] * p[5]) * p[187]))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p[4] * p[5]) * p[187]), 0.0);s.store_add_scaled_inputs3_indices(136, 282, 1.0, 83, 1.0, 286, -1.0);s.store_add_scaled_inputs(90, 284, 0.3333333333333333, 285, (2.0 * 0.3333333333333333));s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(287)), (1.0 / 12.0), 136, 1.0);s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(287)), 287, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);s.store_mul_add_scaled_inputs4_indices_rhs(289, 191, 282, (-(((p[4] * p[187]) * p[5]) * 0.5)), 90, (((-1.0)) * ((-(((p[4] * p[187]) * p[5]) * 0.5)))), 91, (-(((p[4] * p[187]) * p[5]) * 0.5)), 137, (-(((p[4] * p[187]) * p[5]) * 0.5)));}
        s.b[516] = (s.v[68] < 0.0);s.store_scalar(516, if s.b[516] { 1.0 } else { 0.0 });
        if ((s.b[508] && s.b[509]) && s.b[516]) {s.store_sub_scaled_inputs(289, 288, (-1.0), 289, 1.0);}
        if (s.b[508] && (!s.b[509])) {s.store_scalar(288, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_60(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[508] && (!s.b[509])) {s.store_scalar(289, 0.0);}
        s.b[517] = (p[155] != 0.0);s.store_scalar(517, if s.b[517] { 1.0 } else { 0.0 });s.b[518] = (p[155] == 1.0);s.store_scalar(518, if s.b[518] { 1.0 } else { 0.0 });
        if (((!s.b[508]) && s.b[517]) && s.b[518]) {s.store_voltage(70, ctx, nodes, Some(9), Some(8));}
        if (((!s.b[508]) && s.b[517]) && (!s.b[518])) {s.store_voltage(70, ctx, nodes, Some(2), Some(8));}
        if ((!s.b[508]) && s.b[517]) {s.copy_ad(290, 70);s.store_scalar(146, (1.0 + p[191]));s.store_scaled_mul(83, 82, 146, 8.617087e-5);s.store_offset_scaled(88, 82, ((1.0 / (s.v[35])) * (p[188])), (((((-1.0)) * (p[188]))) + (p[185])));s.store_scalar(283, (p[9] / p[186]));s.store_div_scalar_by_product_indices(136, p[187], 83, 83, (((2.0 * p[4]) * 1.602176634e-19) * 3.24e17));s.store_add_scaled_product_mixed_iia(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p[184]), 1.0);s.store_add_scaled_inputs4_mixed_iiai(160, 290, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(290), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);s.store_sub(282, 160, 88);s.store_div_scaled_inputs_indices(84, 283, 1.0, 83, (1.602176634e-19 * 3.24e17));s.store_div_from_scalar(150, 2.718281828459045, 84);s.store_div_from_scalar(151, 1.0, 84);s.store_primal_scale(99, 283, 6.241509074460763e18);s.store_scaled_add_sqrt_square_offset_rhs(154, 282, 282, ((4.0 * 0.3) * 0.3), 0.5);s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);let t6: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, t6, (-(p[195] / 3.0)), A::add_scaled_offset_product_rhs(t6, ((2.0 * p[195]) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 282, 1.0, 83, 2.0);}
        s.b[519] = (s.v[136] < 200.0);s.store_scalar(519, if s.b[519] { 1.0 } else { 0.0 });
        if (((!s.b[508]) && s.b[517]) && s.b[519]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(282), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (((!s.b[508]) && s.b[517]) && (!s.b[519])) {s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(282), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((!s.b[508]) && s.b[517]) {s.store_sub_div_rhs_indices(100, 282, 153, 99);}
        s.b[520] = ((((s.v[100] - s.v[282])) as f64).abs() > 1e-19);s.store_scalar(520, if s.b[520] { 1.0 } else { 0.0 });
        if (((!s.b[508]) && s.b[517]) && s.b[520]) {s.store_sub(101, 282, 100);s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p[195]);s.store_scaled_mul(103, 136, 90, p[196]);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_61(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[508]) && s.b[517]) && s.b[520]) {
            s.store_add_scaled_products3_mixed_iiiaia(106, 99, 101, 1.0, 83, {
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
            }, (-3.24e17), 83, {
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
        if (((!s.b[508]) && s.b[517]) && s.b[520]) {s.store_scaled_mul(107, 136, 91, p[195]);s.store_scaled_mul(108, 136, 91, p[196]);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 282, 114);s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);s.store_powf(137, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(116, 136, p[195], 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p[196], 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
        if (((!s.b[508]) && s.b[517]) && s.b[520]) {
            s.store_add_scaled_products3_mixed_iiiaia(120, 99, 115, 1.0, 83, {
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
            }, (-3.24e17), 83, {
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
        if (((!s.b[508]) && s.b[517]) && s.b[520]) {s.store_scaled_mul(121, 136, 137, p[195]);s.store_scaled_mul(122, 136, 137, p[196]);s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.copy_ad(284, 128);}
        if (((!s.b[508]) && s.b[517]) && (!s.b[520])) {s.copy_ad(284, 100);}
        if ((!s.b[508]) && s.b[517]) {s.store_scalar(291, 0.0);s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p[20], p[189]);s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p[19], p[190]);s.store_mul_scaled_abs_ad_rhs(136, 283, 1.0 / (p[9]), A::sub(s.ad_value(282), s.ad_value(284)));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(284)), (s.v[81] / p[9]));s.store_div_mixed_ia(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p[14], A::square(s.ad_value(136)), p[15], s.ad_value(90), p[16], 1.0));s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);s.store_scaled_add_sqrt_square_offset_rhs(90, 282, 282, ((4.0 * 0.3) * 0.3), 0.5);s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p[187], 136, p[187], 90, 1.0, 1.0);s.store_powf_ad(136, A::div(s.ad_value(291), s.ad_value(85)), p[18]);s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p[18]));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_62(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[508]) && s.b[517]) {s.store_mul(86, 291, 90);s.store_sub(39, 282, 86);s.copy_ad(130, 39);s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);s.copy_ad(154, 131);s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);let t7: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, t7, (-(p[195] / 3.0)), A::add_scaled_offset_product_rhs(t7, ((2.0 * p[195]) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);}
        s.b[521] = (s.v[136] < 200.0);s.store_scalar(521, if s.b[521] { 1.0 } else { 0.0 });
        if (((!s.b[508]) && s.b[517]) && s.b[521]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (((!s.b[508]) && s.b[517]) && (!s.b[521])) {s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((!s.b[508]) && s.b[517]) {s.store_sub_div_rhs_indices(100, 130, 156, 99);}
        s.b[522] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);s.store_scalar(522, if s.b[522] { 1.0 } else { 0.0 });
        if (((!s.b[508]) && s.b[517]) && s.b[522]) {s.store_sub(101, 130, 100);s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p[195]);s.store_scaled_mul(103, 136, 90, p[196]);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
        if (((!s.b[508]) && s.b[517]) && s.b[522]) {
            s.store_add_scaled_products3_mixed_iiiaia(106, 99, 101, 1.0, 83, {
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
            }, (-3.24e17), 83, {
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
        if (((!s.b[508]) && s.b[517]) && s.b[522]) {s.store_scaled_mul(107, 136, 91, p[195]);s.store_scaled_mul(108, 136, 91, p[196]);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_63(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[508]) && s.b[517]) && s.b[522]) {s.store_sub(115, 130, 114);s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);s.store_mul_scaled_powf_rhs(116, 136, p[195], 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p[196], 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
        if (((!s.b[508]) && s.b[517]) && s.b[522]) {
            s.store_add_scaled_products3_mixed_iiiaia(120, 99, 115, 1.0, 83, {
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
            }, (-3.24e17), 83, {
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
        if (((!s.b[508]) && s.b[517]) && s.b[522]) {s.store_mul_scaled_powf_rhs(121, 136, p[195], 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(122, 136, p[196], 115, (-0.3333333333333333));s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.store_add(285, 128, 86);}
        if (((!s.b[508]) && s.b[517]) && (!s.b[522])) {s.store_add(285, 100, 86);}
        if ((!s.b[508]) && s.b[517]) {s.store_scaled_add(286, 284, 285, 0.5);s.store_sub(287, 285, 284);s.store_sub(90, 285, 284);s.store_add_scaled_inputs3_indices(91, 282, 1.0, 83, 1.0, 286, -1.0);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(137, 283, 282, ((p[4] * p[5]) * p[187]), 286, (((-1.0)) * (((p[4] * p[5]) * p[187]))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p[4] * p[5]) * p[187]), 0.0);s.store_scale(188, 137, (1.0 / (p[242]) * 1e26));s.store_offset_powf_ad(189, s.ad_value(188), p[241], 1.0);s.store_div_from_scalar(190, p[240], 189);s.store_div_from_scalar_offset_input(191, p[9], 190, p[186]);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(288, 191, 282, ((p[4] * p[5]) * p[187]), 286, (((-1.0)) * (((p[4] * p[5]) * p[187]))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p[4] * p[5]) * p[187]), 0.0);s.store_add_scaled_inputs3_indices(136, 282, 1.0, 83, 1.0, 286, -1.0);s.store_add_scaled_inputs(90, 284, 0.3333333333333333, 285, (2.0 * 0.3333333333333333));s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(287)), (1.0 / 12.0), 136, 1.0);s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(287)), 287, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);s.store_mul_add_scaled_inputs4_indices_rhs(289, 191, 282, (-(((p[4] * p[187]) * p[5]) * 0.5)), 90, (((-1.0)) * ((-(((p[4] * p[187]) * p[5]) * 0.5)))), 91, (-(((p[4] * p[187]) * p[5]) * 0.5)), 137, (-(((p[4] * p[187]) * p[5]) * 0.5)));}
        if ((!s.b[508]) && (!s.b[517])) {s.store_scalar(288, 0.0);s.store_scalar(289, 0.0);}
        s.b[523] = (p[149] == 0.0);s.store_scalar(523, if s.b[523] { 1.0 } else { 0.0 });s.b[524] = (p[156] != 0.0);s.store_scalar(524, if s.b[524] { 1.0 } else { 0.0 });
        if (s.b[523] && s.b[524]) {s.store_voltage(73, ctx, nodes, Some(18), Some(17));}
        s.b[525] = (p[156] == 1.0);s.store_scalar(525, if s.b[525] { 1.0 } else { 0.0 });
        if ((s.b[523] && s.b[524]) && s.b[525]) {s.store_voltage(74, ctx, nodes, Some(9), Some(17));s.store_voltage(75, ctx, nodes, Some(9), Some(18));}
        if ((s.b[523] && s.b[524]) && (!s.b[525])) {s.store_voltage(74, ctx, nodes, Some(2), Some(17));s.store_voltage(75, ctx, nodes, Some(2), Some(18));}
        if (s.b[523] && s.b[524]) {s.store_scalar(72, 1.0);}
        s.b[526] = (s.v[73] < 0.0);s.store_scalar(526, if s.b[526] { 1.0 } else { 0.0 });
        if ((s.b[523] && s.b[524]) && s.b[526]) {s.store_scalar(72, (-1.0));s.store_mul(303, 72, 73);s.copy_ad(302, 75);}
        if ((s.b[523] && s.b[524]) && (!s.b[526])) {s.copy_ad(303, 73);s.copy_ad(302, 74);}
        if (s.b[523] && s.b[524]) {s.store_offset_sqrt_ad(304, A::offset(A::square(s.ad_value(303)), 0.01), (-0.1));s.store_offset_scaled(146, 304, p[205], (1.0 + p[204]));s.store_scaled_mul(83, 82, 146, 8.617087e-5);}
    }
}
