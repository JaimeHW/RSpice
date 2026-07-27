#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_61(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[532] = (p[156] != 0.0);s.store_scalar(532, if s.b[532] { 1.0 } else { 0.0 });s.b[533] = (p[156] == 1.0);s.store_scalar(533, if s.b[533] { 1.0 } else { 0.0 });
        if (((!s.b[523]) && s.b[532]) && s.b[533]) {s.store_voltage(74, ctx, nodes, Some(9), Some(7));}
        if (((!s.b[523]) && s.b[532]) && (!s.b[533])) {s.store_voltage(74, ctx, nodes, Some(2), Some(7));}
        if ((!s.b[523]) && s.b[532]) {s.copy_ad(302, 74);s.store_scalar(146, (1.0 + p[204]));s.store_scaled_mul(83, 82, 146, 8.617087e-5);s.store_sub_from_scalar_ad(88, p[198], A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p[201])), (((-1.0)) * (p[201]))));s.store_scalar(295, (p[9] / p[199]));s.store_div_scalar_by_product_indices(136, p[200], 83, 83, (((2.0 * p[4]) * 1.602176634e-19) * 3.24e17));s.store_add_scaled_product_mixed_iia(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p[197]), 1.0);s.store_add_scaled_inputs4_mixed_iiai(160, 302, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(302), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);s.store_sub(294, 160, 88);s.store_div_scaled_inputs_indices(84, 295, 1.0, 83, (1.602176634e-19 * 3.24e17));s.store_div_from_scalar(150, 2.718281828459045, 84);s.store_div_from_scalar(151, 1.0, 84);s.store_primal_scale(99, 295, 6.241509074460763e18);s.store_scaled_add_mixed_ia(154, 294, A::sqrt_square_offset(s.ad_value(294), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);let t0: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, t0, (-(p[208] / 3.0)), A::add_scaled_offset_product_rhs(t0, ((2.0 * p[208]) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 294, 1.0, 83, 2.0);}
        s.b[534] = (s.v[136] < 200.0);s.store_scalar(534, if s.b[534] { 1.0 } else { 0.0 });
        if (((!s.b[523]) && s.b[532]) && s.b[534]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(294), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (((!s.b[523]) && s.b[532]) && (!s.b[534])) {s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(294), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((!s.b[523]) && s.b[532]) {s.store_sub_div_rhs_indices(100, 294, 153, 99);}
        s.b[535] = ((((s.v[100] - s.v[294])) as f64).abs() > 1e-19);s.store_scalar(535, if s.b[535] { 1.0 } else { 0.0 });
        if (((!s.b[523]) && s.b[532]) && s.b[535]) {s.store_sub(101, 294, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p[208]);s.store_scaled_mul(103, 136, 90, p[209]);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_62(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[523]) && s.b[532]) && s.b[535]) {
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
        if (((!s.b[523]) && s.b[532]) && s.b[535]) {s.store_scaled_mul(107, 136, 91, p[208]);s.store_scaled_mul(108, 136, 91, p[209]);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 294, 114);s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(137, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(116, 136, p[208], 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p[209], 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
        if (((!s.b[523]) && s.b[532]) && s.b[535]) {
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
        if (((!s.b[523]) && s.b[532]) && s.b[535]) {s.store_scaled_mul(121, 136, 137, p[208]);s.store_scaled_mul(122, 136, 137, p[209]);s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.copy_ad(296, 128);}
        if (((!s.b[523]) && s.b[532]) && (!s.b[535])) {s.copy_ad(296, 100);}
        if ((!s.b[523]) && s.b[532]) {s.store_scalar(303, 0.0);s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p[20], p[202]);s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p[19], p[203]);s.store_mul_scaled_abs_ad_rhs(136, 295, 1.0 / (p[9]), A::sub(s.ad_value(294), s.ad_value(296)));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(296)), (s.v[81] / p[9]));s.store_div_mixed_ia(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p[14], A::square(s.ad_value(136)), p[15], s.ad_value(90), p[16], 1.0));s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);s.store_scaled_add_mixed_ia(90, 294, A::sqrt_square_offset(s.ad_value(294), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p[200], 136, p[200], 90, 1.0, 1.0);s.store_powf_ad(136, A::div(s.ad_value(303), s.ad_value(85)), p[18]);s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p[18]));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_63(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[523]) && s.b[532]) {s.store_mul(86, 303, 90);s.store_sub(39, 294, 86);s.copy_ad(130, 39);s.store_scaled_add_mixed_ia(131, 130, A::sqrt_square_offset(s.ad_value(130), ((4.0 * 0.3) * 0.3)), 0.5);s.copy_ad(154, 131);s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);let t1: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, t1, (-(p[208] / 3.0)), A::add_scaled_offset_product_rhs(t1, ((2.0 * p[208]) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);}
        s.b[536] = (s.v[136] < 200.0);s.store_scalar(536, if s.b[536] { 1.0 } else { 0.0 });
        if (((!s.b[523]) && s.b[532]) && s.b[536]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (((!s.b[523]) && s.b[532]) && (!s.b[536])) {s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((!s.b[523]) && s.b[532]) {s.store_sub_div_rhs_indices(100, 130, 156, 99);}
        s.b[537] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);s.store_scalar(537, if s.b[537] { 1.0 } else { 0.0 });
        if (((!s.b[523]) && s.b[532]) && s.b[537]) {s.store_sub(101, 130, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p[208]);s.store_scaled_mul(103, 136, 90, p[209]);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
        if (((!s.b[523]) && s.b[532]) && s.b[537]) {
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
        if (((!s.b[523]) && s.b[532]) && s.b[537]) {s.store_scaled_mul(107, 136, 91, p[208]);s.store_scaled_mul(108, 136, 91, p[209]);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_64(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[523]) && s.b[532]) && s.b[537]) {s.store_sub(115, 130, 114);s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_mul_scaled_powf_rhs(116, 136, p[208], 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p[209], 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
        if (((!s.b[523]) && s.b[532]) && s.b[537]) {
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
        if (((!s.b[523]) && s.b[532]) && s.b[537]) {s.store_mul_scaled_powf_rhs(121, 136, p[208], 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(122, 136, p[209], 115, (-0.3333333333333333));s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.store_add(297, 128, 86);}
        if (((!s.b[523]) && s.b[532]) && (!s.b[537])) {s.store_add(297, 100, 86);}
        if ((!s.b[523]) && s.b[532]) {s.store_scaled_add(298, 296, 297, 0.5);s.store_sub(299, 297, 296);s.store_sub(90, 297, 296);s.store_add_scaled_inputs3_indices(91, 294, 1.0, 83, 1.0, 298, -1.0);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(137, 295, 294, ((p[4] * p[5]) * p[200]), 298, (((-1.0)) * (((p[4] * p[5]) * p[200]))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p[4] * p[5]) * p[200]), 0.0);s.store_scale(188, 137, (1.0 / (p[245]) * 1e26));s.store_offset_powf_ad(189, s.ad_value(188), p[244], 1.0);s.store_div_from_scalar(190, p[243], 189);s.store_div_from_scalar_offset_input(191, p[9], 190, p[199]);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(300, 191, 294, ((p[4] * p[5]) * p[200]), 298, (((-1.0)) * (((p[4] * p[5]) * p[200]))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p[4] * p[5]) * p[200]), 0.0);s.store_add_scaled_inputs3_indices(136, 294, 1.0, 83, 1.0, 298, -1.0);s.store_add_scaled_inputs(90, 296, 0.3333333333333333, 297, (2.0 * 0.3333333333333333));s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(299)), (1.0 / 12.0), 136, 1.0);s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(299)), 299, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);s.store_mul_add_scaled_inputs4_indices_rhs(301, 191, 294, (-(((p[4] * p[200]) * p[5]) * 0.5)), 90, (((-1.0)) * ((-(((p[4] * p[200]) * p[5]) * 0.5)))), 91, (-(((p[4] * p[200]) * p[5]) * 0.5)), 137, (-(((p[4] * p[200]) * p[5]) * 0.5)));}
        if ((!s.b[523]) && (!s.b[532])) {s.store_scalar(300, 0.0);s.store_scalar(301, 0.0);}
        s.b[538] = (p[149] == 0.0);s.store_scalar(538, if s.b[538] { 1.0 } else { 0.0 });s.b[539] = (p[157] != 0.0);s.store_scalar(539, if s.b[539] { 1.0 } else { 0.0 });
        if (s.b[538] && s.b[539]) {s.store_voltage(77, ctx, nodes, Some(21), Some(22));}
        s.b[540] = (p[157] == 1.0);s.store_scalar(540, if s.b[540] { 1.0 } else { 0.0 });
        if ((s.b[538] && s.b[539]) && s.b[540]) {s.store_voltage(78, ctx, nodes, Some(9), Some(22));s.store_voltage(79, ctx, nodes, Some(9), Some(21));}
        if ((s.b[538] && s.b[539]) && (!s.b[540])) {s.store_voltage(78, ctx, nodes, Some(2), Some(22));s.store_voltage(79, ctx, nodes, Some(2), Some(21));}
        if (s.b[538] && s.b[539]) {s.store_scalar(76, 1.0);}
        s.b[541] = (s.v[77] < 0.0);s.store_scalar(541, if s.b[541] { 1.0 } else { 0.0 });
        if ((s.b[538] && s.b[539]) && s.b[541]) {s.store_scalar(76, (-1.0));s.store_mul(315, 76, 77);s.copy_ad(314, 79);}
        if ((s.b[538] && s.b[539]) && (!s.b[541])) {s.copy_ad(315, 77);s.copy_ad(314, 78);}
        if (s.b[538] && s.b[539]) {s.store_offset_sqrt_ad(316, A::offset(A::square(s.ad_value(315)), 0.01), (-0.1));s.store_offset_scaled(146, 316, p[205], (1.0 + p[204]));s.store_scaled_mul(83, 82, 146, 8.617087e-5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_65(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[538] && s.b[539]) {s.store_sub_ad(88, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p[201])), (((((-1.0)) * (p[201]))) + (p[198]))), A::div_scaled_inputs(s.ad_value(316), (p[207] * p[206]), A::sqrt_square_offset(s.ad_value(316), (p[207] * p[207])), 1.0));s.store_scalar(307, (p[9] / p[199]));s.store_div_scalar_by_product_indices(136, p[200], 83, 83, (((2.0 * p[4]) * 1.602176634e-19) * 3.24e17));s.store_add_scaled_product_mixed_iia(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p[197]), 1.0);s.store_add_scaled_inputs4_mixed_iiai(160, 314, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(314), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);s.store_sub(306, 160, 88);s.store_div_scaled_inputs_indices(84, 307, 1.0, 83, (1.602176634e-19 * 3.24e17));s.store_div_from_scalar(150, 2.718281828459045, 84);s.store_div_from_scalar(151, 1.0, 84);s.store_primal_scale(99, 307, 6.241509074460763e18);s.store_scaled_add_mixed_ia(154, 306, A::sqrt_square_offset(s.ad_value(306), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);let t2: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, t2, (-(p[208] / 3.0)), A::add_scaled_offset_product_rhs(t2, ((2.0 * p[208]) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 306, 1.0, 83, 2.0);}
        s.b[542] = (s.v[136] < 200.0);s.store_scalar(542, if s.b[542] { 1.0 } else { 0.0 });
        if ((s.b[538] && s.b[539]) && s.b[542]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((s.b[538] && s.b[539]) && (!s.b[542])) {s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (s.b[538] && s.b[539]) {s.store_sub_div_rhs_indices(100, 306, 153, 99);}
        s.b[543] = ((((s.v[100] - s.v[306])) as f64).abs() > 1e-19);s.store_scalar(543, if s.b[543] { 1.0 } else { 0.0 });
        if ((s.b[538] && s.b[539]) && s.b[543]) {s.store_sub(101, 306, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p[208]);s.store_scaled_mul(103, 136, 90, p[209]);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_66(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[538] && s.b[539]) && s.b[543]) {
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
        if ((s.b[538] && s.b[539]) && s.b[543]) {s.store_scaled_mul(107, 136, 91, p[208]);s.store_scaled_mul(108, 136, 91, p[209]);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 306, 114);s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(137, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(116, 136, p[208], 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p[209], 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
        if ((s.b[538] && s.b[539]) && s.b[543]) {
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
        if ((s.b[538] && s.b[539]) && s.b[543]) {s.store_scaled_mul(121, 136, 137, p[208]);s.store_scaled_mul(122, 136, 137, p[209]);s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.copy_ad(308, 128);}
        if ((s.b[538] && s.b[539]) && (!s.b[543])) {s.copy_ad(308, 100);}
        if (s.b[538] && s.b[539]) {s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p[20], p[202]);s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p[19], p[203]);s.store_mul_scaled_abs_ad_rhs(136, 307, 1.0 / (p[9]), A::sub(s.ad_value(306), s.ad_value(308)));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(308)), (s.v[81] / p[9]));s.store_div_mixed_ia(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p[14], A::square(s.ad_value(136)), p[15], s.ad_value(90), p[16], 1.0));s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);s.store_scaled_add_mixed_ia(90, 306, A::sqrt_square_offset(s.ad_value(306), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p[200], 136, p[200], 90, 1.0, 1.0);s.store_powf_ad(136, A::div(s.ad_value(315), s.ad_value(85)), p[18]);s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p[18]));s.store_mul(86, 315, 90);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_67(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[538] && s.b[539]) {s.store_sub(39, 306, 86);s.copy_ad(130, 39);s.store_scaled_add_mixed_ia(131, 130, A::sqrt_square_offset(s.ad_value(130), ((4.0 * 0.3) * 0.3)), 0.5);s.copy_ad(154, 131);s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);let t3: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, t3, (-(p[208] / 3.0)), A::add_scaled_offset_product_rhs(t3, ((2.0 * p[208]) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);}
        s.b[544] = (s.v[136] < 200.0);s.store_scalar(544, if s.b[544] { 1.0 } else { 0.0 });
        if ((s.b[538] && s.b[539]) && s.b[544]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((s.b[538] && s.b[539]) && (!s.b[544])) {s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (s.b[538] && s.b[539]) {s.store_sub_div_rhs_indices(100, 130, 156, 99);}
        s.b[545] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);s.store_scalar(545, if s.b[545] { 1.0 } else { 0.0 });
        if ((s.b[538] && s.b[539]) && s.b[545]) {s.store_sub(101, 130, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p[208]);s.store_scaled_mul(103, 136, 90, p[209]);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
        if ((s.b[538] && s.b[539]) && s.b[545]) {
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
        if ((s.b[538] && s.b[539]) && s.b[545]) {s.store_scaled_mul(107, 136, 91, p[208]);s.store_scaled_mul(108, 136, 91, p[209]);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 130, 114);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_68(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[538] && s.b[539]) && s.b[545]) {s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_mul_scaled_powf_rhs(116, 136, p[208], 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p[209], 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
        if ((s.b[538] && s.b[539]) && s.b[545]) {
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
        if ((s.b[538] && s.b[539]) && s.b[545]) {s.store_mul_scaled_powf_rhs(121, 136, p[208], 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(122, 136, p[209], 115, (-0.3333333333333333));s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.store_add(309, 128, 86);}
        if ((s.b[538] && s.b[539]) && (!s.b[545])) {s.store_add(309, 100, 86);}
        if (s.b[538] && s.b[539]) {s.store_scaled_add(310, 308, 309, 0.5);s.store_sub(311, 309, 308);s.store_mul_add_scaled_inputs3_offset_rhs_indices(135, 311, 306, 1.0, 310, (-1.0), 83, 1.0, 0.0);s.store_mul_scaled_abs_ad_rhs(136, 307, 1.0 / (p[9]), A::sub(s.ad_value(306), s.ad_value(310)));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p[9]));s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p[14], 1.0), 1.0, s.ad_value(136), s.ad_value(136), p[15]), 1.0, 90, p[16]);s.store_scaled_mul(96, 95, 307, (p[4] * (p[5] * 1.0 / (p[200]))));s.store_mul_scale_offset_mixed_ia(98, 96, A::sub_scaled_inputs(s.ad_value(316), p[21], s.ad_value(86), p[21]), 1.0, 1.0);s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(311), (p[25] * p[25]), s.ad_value(311)), 1.0);s.store_div(93, 98, 92);s.store_sub(90, 309, 308);s.store_add_scaled_inputs3_indices(91, 306, 1.0, 83, 1.0, 310, -1.0);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(137, 307, 306, ((p[4] * p[5]) * p[200]), 310, (((-1.0)) * (((p[4] * p[5]) * p[200]))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p[4] * p[5]) * p[200]), 0.0);s.store_scale(188, 137, (1.0 / (p[245]) * 1e26));s.store_offset_powf_ad(189, s.ad_value(188), p[244], 1.0);s.store_div_from_scalar(190, p[243], 189);s.store_div_from_scalar_offset_input(191, p[9], 190, p[199]);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(312, 191, 306, ((p[4] * p[5]) * p[200]), 310, (((-1.0)) * (((p[4] * p[5]) * p[200]))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p[4] * p[5]) * p[200]), 0.0);s.store_add_scaled_inputs3_indices(136, 306, 1.0, 83, 1.0, 310, -1.0);s.store_add_scaled_inputs(90, 308, 0.3333333333333333, 309, (2.0 * 0.3333333333333333));s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(311)), (1.0 / 12.0), 136, 1.0);s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(311)), 311, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);s.store_mul_add_scaled_inputs4_indices_rhs(313, 191, 306, (-(((p[4] * p[200]) * p[5]) * 0.5)), 90, (((-1.0)) * ((-(((p[4] * p[200]) * p[5]) * 0.5)))), 91, (-(((p[4] * p[200]) * p[5]) * 0.5)), 137, (-(((p[4] * p[200]) * p[5]) * 0.5)));}
        s.b[546] = (s.v[76] < 0.0);s.store_scalar(546, if s.b[546] { 1.0 } else { 0.0 });
        if ((s.b[538] && s.b[539]) && s.b[546]) {s.store_sub_scaled_inputs(313, 312, (-1.0), 313, 1.0);}
        if (s.b[538] && (!s.b[539])) {s.store_scalar(312, 0.0);s.store_scalar(313, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_69(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[547] = (p[157] != 0.0);s.store_scalar(547, if s.b[547] { 1.0 } else { 0.0 });s.b[548] = (p[157] == 1.0);s.store_scalar(548, if s.b[548] { 1.0 } else { 0.0 });
        if (((!s.b[538]) && s.b[547]) && s.b[548]) {s.store_voltage(78, ctx, nodes, Some(9), Some(8));}
        if (((!s.b[538]) && s.b[547]) && (!s.b[548])) {s.store_voltage(78, ctx, nodes, Some(2), Some(8));}
        if ((!s.b[538]) && s.b[547]) {s.copy_ad(314, 78);s.store_scalar(146, (1.0 + p[204]));s.store_scaled_mul(83, 82, 146, 8.617087e-5);s.store_offset_scaled(88, 82, ((1.0 / (s.v[35])) * (p[201])), (((((-1.0)) * (p[201]))) + (p[198])));s.store_scalar(307, (p[9] / p[199]));s.store_div_scalar_by_product_indices(136, p[200], 83, 83, (((2.0 * p[4]) * 1.602176634e-19) * 3.24e17));s.store_add_scaled_product_mixed_iia(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p[197]), 1.0);s.store_add_scaled_inputs4_mixed_iiai(160, 314, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(314), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);s.store_sub(306, 160, 88);s.store_div_scaled_inputs_indices(84, 307, 1.0, 83, (1.602176634e-19 * 3.24e17));s.store_div_from_scalar(150, 2.718281828459045, 84);s.store_div_from_scalar(151, 1.0, 84);s.store_primal_scale(99, 307, 6.241509074460763e18);s.store_scaled_add_mixed_ia(154, 306, A::sqrt_square_offset(s.ad_value(306), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);let t4: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, t4, (-(p[208] / 3.0)), A::add_scaled_offset_product_rhs(t4, ((2.0 * p[208]) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 306, 1.0, 83, 2.0);}
        s.b[549] = (s.v[136] < 200.0);s.store_scalar(549, if s.b[549] { 1.0 } else { 0.0 });
        if (((!s.b[538]) && s.b[547]) && s.b[549]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (((!s.b[538]) && s.b[547]) && (!s.b[549])) {s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((!s.b[538]) && s.b[547]) {s.store_sub_div_rhs_indices(100, 306, 153, 99);}
        s.b[550] = ((((s.v[100] - s.v[306])) as f64).abs() > 1e-19);s.store_scalar(550, if s.b[550] { 1.0 } else { 0.0 });
        if (((!s.b[538]) && s.b[547]) && s.b[550]) {s.store_sub(101, 306, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p[208]);s.store_scaled_mul(103, 136, 90, p[209]);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_70(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[538]) && s.b[547]) && s.b[550]) {
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
        if (((!s.b[538]) && s.b[547]) && s.b[550]) {s.store_scaled_mul(107, 136, 91, p[208]);s.store_scaled_mul(108, 136, 91, p[209]);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 306, 114);s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(137, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(116, 136, p[208], 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p[209], 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
        if (((!s.b[538]) && s.b[547]) && s.b[550]) {
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
        if (((!s.b[538]) && s.b[547]) && s.b[550]) {s.store_scaled_mul(121, 136, 137, p[208]);s.store_scaled_mul(122, 136, 137, p[209]);s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.copy_ad(308, 128);}
        if (((!s.b[538]) && s.b[547]) && (!s.b[550])) {s.copy_ad(308, 100);}
        if ((!s.b[538]) && s.b[547]) {s.store_scalar(315, 0.0);s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p[20], p[202]);s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p[19], p[203]);s.store_mul_scaled_abs_ad_rhs(136, 307, 1.0 / (p[9]), A::sub(s.ad_value(306), s.ad_value(308)));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(308)), (s.v[81] / p[9]));s.store_div_mixed_ia(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p[14], A::square(s.ad_value(136)), p[15], s.ad_value(90), p[16], 1.0));s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);s.store_scaled_add_mixed_ia(90, 306, A::sqrt_square_offset(s.ad_value(306), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p[200], 136, p[200], 90, 1.0, 1.0);s.store_powf_ad(136, A::div(s.ad_value(315), s.ad_value(85)), p[18]);s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p[18]));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_71(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[538]) && s.b[547]) {s.store_mul(86, 315, 90);s.store_sub(39, 306, 86);s.copy_ad(130, 39);s.store_scaled_add_mixed_ia(131, 130, A::sqrt_square_offset(s.ad_value(130), ((4.0 * 0.3) * 0.3)), 0.5);s.copy_ad(154, 131);s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);let t5: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, t5, (-(p[208] / 3.0)), A::add_scaled_offset_product_rhs(t5, ((2.0 * p[208]) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);}
        s.b[551] = (s.v[136] < 200.0);s.store_scalar(551, if s.b[551] { 1.0 } else { 0.0 });
        if (((!s.b[538]) && s.b[547]) && s.b[551]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (((!s.b[538]) && s.b[547]) && (!s.b[551])) {s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((!s.b[538]) && s.b[547]) {s.store_sub_div_rhs_indices(100, 130, 156, 99);}
        s.b[552] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);s.store_scalar(552, if s.b[552] { 1.0 } else { 0.0 });
        if (((!s.b[538]) && s.b[547]) && s.b[552]) {s.store_sub(101, 130, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p[208]);s.store_scaled_mul(103, 136, 90, p[209]);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
        if (((!s.b[538]) && s.b[547]) && s.b[552]) {
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
        if (((!s.b[538]) && s.b[547]) && s.b[552]) {s.store_scaled_mul(107, 136, 91, p[208]);s.store_scaled_mul(108, 136, 91, p[209]);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_72(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[538]) && s.b[547]) && s.b[552]) {s.store_sub(115, 130, 114);s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_mul_scaled_powf_rhs(116, 136, p[208], 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p[209], 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
        if (((!s.b[538]) && s.b[547]) && s.b[552]) {
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
        if (((!s.b[538]) && s.b[547]) && s.b[552]) {s.store_mul_scaled_powf_rhs(121, 136, p[208], 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(122, 136, p[209], 115, (-0.3333333333333333));s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.store_add(309, 128, 86);}
        if (((!s.b[538]) && s.b[547]) && (!s.b[552])) {s.store_add(309, 100, 86);}
        if ((!s.b[538]) && s.b[547]) {s.store_scaled_add(310, 308, 309, 0.5);s.store_sub(311, 309, 308);s.store_sub(90, 309, 308);s.store_add_scaled_inputs3_indices(91, 306, 1.0, 83, 1.0, 310, -1.0);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(137, 307, 306, ((p[4] * p[5]) * p[200]), 310, (((-1.0)) * (((p[4] * p[5]) * p[200]))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p[4] * p[5]) * p[200]), 0.0);s.store_scale(188, 137, (1.0 / (p[245]) * 1e26));s.store_offset_powf_ad(189, s.ad_value(188), p[244], 1.0);s.store_div_from_scalar(190, p[243], 189);s.store_div_from_scalar_offset_input(191, p[9], 190, p[199]);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(312, 191, 306, ((p[4] * p[5]) * p[200]), 310, (((-1.0)) * (((p[4] * p[5]) * p[200]))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p[4] * p[5]) * p[200]), 0.0);s.store_add_scaled_inputs3_indices(136, 306, 1.0, 83, 1.0, 310, -1.0);s.store_add_scaled_inputs(90, 308, 0.3333333333333333, 309, (2.0 * 0.3333333333333333));s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(311)), (1.0 / 12.0), 136, 1.0);s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(311)), 311, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);s.store_mul_add_scaled_inputs4_indices_rhs(313, 191, 306, (-(((p[4] * p[200]) * p[5]) * 0.5)), 90, (((-1.0)) * ((-(((p[4] * p[200]) * p[5]) * 0.5)))), 91, (-(((p[4] * p[200]) * p[5]) * 0.5)), 137, (-(((p[4] * p[200]) * p[5]) * 0.5)));}
        if ((!s.b[538]) && (!s.b[547])) {s.store_scalar(312, 0.0);s.store_scalar(313, 0.0);}
        s.b[558] = (p[255] == 2.0);s.store_scalar(558, if s.b[558] { 1.0 } else { 0.0 });
        if s.b[558] {s.store_scaled_voltage(162, ctx, nodes, Some(10), Some(2), ((p[4] * p[5]) * p[210]));s.store_div_scaled_inputs(168, A::voltage(ctx, nodes, Some(0), Some(2)), p[214], A::sqrt(A::offset(A::square(A::voltage(ctx, nodes, Some(0), Some(2))), (p[214] * p[214]))), 1.0);s.store_scalar(169, (p[213]).min((p[211] / (2.0 * p[214]))));s.store_sub_from_scalar_scaled_mul(167, ((p[4] * p[5]) * p[211]), 169, 168, (p[4] * p[5]));s.store_mul_voltage_ad(163, A::max_with_scalar(s.ad_value(167), 0.0), ctx, nodes, Some(10), Some(0));}
        if (!s.b[558]) {s.store_scaled_voltage(162, ctx, nodes, Some(1), Some(2), ((p[4] * p[5]) * p[210]));s.store_div_scaled_inputs(168, A::voltage(ctx, nodes, Some(0), Some(2)), p[214], A::sqrt(A::offset(A::square(A::voltage(ctx, nodes, Some(0), Some(2))), (p[214] * p[214]))), 1.0);s.store_scalar(169, (p[213]).min((p[211] / (2.0 * p[214]))));s.store_sub_from_scalar_scaled_mul(167, ((p[4] * p[5]) * p[211]), 169, 168, (p[4] * p[5]));s.store_mul_voltage_ad(163, A::max_with_scalar(s.ad_value(167), 0.0), ctx, nodes, Some(1), Some(0));}
        s.store_scaled_voltage(164, ctx, nodes, Some(0), Some(2), ((p[4] * p[5]) * p[212]));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_73(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.store_scaled_voltage(219, ctx, nodes, Some(3), Some(0), ((p[4] * p[5]) * p[215]));s.store_scaled_voltage(220, ctx, nodes, Some(3), Some(2), ((p[4] * p[5]) * p[216]));s.store_scaled_voltage(221, ctx, nodes, Some(3), Some(1), ((p[4] * p[5]) * p[217]));s.store_scale_ad(377, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p[281]), p[277]);s.store_scale_ad(378, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p[282]), p[278]);s.store_scale(137, 378, (p[4] * p[5]));s.store_scale(137, 377, (p[4] * p[5]));s.b[569] = (p[255] == 2.0);s.store_scalar(569, if s.b[569] { 1.0 } else { 0.0 });s.b[570] = (p[149] == 0.0);s.store_scalar(570, if s.b[570] { 1.0 } else { 0.0 });s.b[571] = (p[150] != 0.0);s.store_scalar(571, if s.b[571] { 1.0 } else { 0.0 });s.b[572] = (p[150] == 1.0);s.store_scalar(572, if s.b[572] { 1.0 } else { 0.0 });s.b[573] = (p[150] != 0.0);s.store_scalar(573, if s.b[573] { 1.0 } else { 0.0 });s.b[574] = (p[150] == 1.0);s.store_scalar(574, if s.b[574] { 1.0 } else { 0.0 });s.b[575] = (p[149] == 0.0);s.store_scalar(575, if s.b[575] { 1.0 } else { 0.0 });s.b[576] = (p[151] != 0.0);s.store_scalar(576, if s.b[576] { 1.0 } else { 0.0 });s.b[577] = (p[151] == 1.0);s.store_scalar(577, if s.b[577] { 1.0 } else { 0.0 });s.b[578] = (p[151] != 0.0);s.store_scalar(578, if s.b[578] { 1.0 } else { 0.0 });s.b[579] = (p[151] == 1.0);s.store_scalar(579, if s.b[579] { 1.0 } else { 0.0 });s.b[580] = (p[149] == 0.0);s.store_scalar(580, if s.b[580] { 1.0 } else { 0.0 });s.b[581] = (p[152] != 0.0);s.store_scalar(581, if s.b[581] { 1.0 } else { 0.0 });s.b[582] = (p[152] == 1.0);s.store_scalar(582, if s.b[582] { 1.0 } else { 0.0 });s.b[583] = (p[152] != 0.0);s.store_scalar(583, if s.b[583] { 1.0 } else { 0.0 });s.b[584] = (p[152] == 1.0);s.store_scalar(584, if s.b[584] { 1.0 } else { 0.0 });s.b[585] = (p[149] == 0.0);s.store_scalar(585, if s.b[585] { 1.0 } else { 0.0 });s.b[586] = (p[153] != 0.0);s.store_scalar(586, if s.b[586] { 1.0 } else { 0.0 });s.b[587] = (p[153] == 1.0);s.store_scalar(587, if s.b[587] { 1.0 } else { 0.0 });s.b[588] = (p[153] != 0.0);s.store_scalar(588, if s.b[588] { 1.0 } else { 0.0 });s.b[589] = (p[153] == 1.0);s.store_scalar(589, if s.b[589] { 1.0 } else { 0.0 });s.b[590] = (p[149] == 0.0);s.store_scalar(590, if s.b[590] { 1.0 } else { 0.0 });s.b[591] = (p[154] != 0.0);s.store_scalar(591, if s.b[591] { 1.0 } else { 0.0 });s.b[592] = (p[154] == 1.0);s.store_scalar(592, if s.b[592] { 1.0 } else { 0.0 });s.b[593] = (p[154] != 0.0);s.store_scalar(593, if s.b[593] { 1.0 } else { 0.0 });s.b[594] = (p[154] == 1.0);s.store_scalar(594, if s.b[594] { 1.0 } else { 0.0 });s.b[595] = (p[149] == 0.0);s.store_scalar(595, if s.b[595] { 1.0 } else { 0.0 });s.b[596] = (p[155] != 0.0);s.store_scalar(596, if s.b[596] { 1.0 } else { 0.0 });s.b[597] = (p[155] == 1.0);s.store_scalar(597, if s.b[597] { 1.0 } else { 0.0 });s.b[598] = (p[155] != 0.0);s.store_scalar(598, if s.b[598] { 1.0 } else { 0.0 });s.b[599] = (p[155] == 1.0);s.store_scalar(599, if s.b[599] { 1.0 } else { 0.0 });s.b[600] = (p[149] == 0.0);s.store_scalar(600, if s.b[600] { 1.0 } else { 0.0 });s.b[601] = (p[156] != 0.0);s.store_scalar(601, if s.b[601] { 1.0 } else { 0.0 });s.b[602] = (p[156] == 1.0);s.store_scalar(602, if s.b[602] { 1.0 } else { 0.0 });s.b[603] = (p[156] != 0.0);s.store_scalar(603, if s.b[603] { 1.0 } else { 0.0 });s.b[604] = (p[156] == 1.0);s.store_scalar(604, if s.b[604] { 1.0 } else { 0.0 });s.b[605] = (p[149] == 0.0);s.store_scalar(605, if s.b[605] { 1.0 } else { 0.0 });s.b[606] = (p[157] != 0.0);s.store_scalar(606, if s.b[606] { 1.0 } else { 0.0 });s.b[607] = (p[157] == 1.0);s.store_scalar(607, if s.b[607] { 1.0 } else { 0.0 });s.b[608] = (p[157] != 0.0);s.store_scalar(608, if s.b[608] { 1.0 } else { 0.0 });s.b[609] = (p[157] == 1.0);s.store_scalar(609, if s.b[609] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_74(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.store_sub_from_scalar_ad(195, p[222], A::mul(A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p[227])), (((((-1.0)) * (p[227]))) + (p[220]))), A::voltage(ctx, nodes, Some(0), Some(2))));s.store_add_scaled_inputs3_offset_mixed_iia(195, 195, (p[4] * p[5]), 195, ((-0.5) * (p[4] * p[5])), A::sqrt_square_offset(A::offset(s.ad_value(195), (-1e-25)), p[221]), ((-(-0.5)) * (p[4] * p[5])), ((1e-25 + ((-0.5) * 1e-25)) * (p[4] * p[5])));s.store_scaled_add_offset_sqrt_square_offset_ad(136, A::sub_from_scalar(p[218], A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p[226])), (((-1.0)) * (p[226])))), 1e-18, (-1e-18), ((0.25 * 1e-19) * 1e-19), 0.5);s.store_mul_scaled_voltage(196, 136, (p[4] * p[5]), ctx, nodes, Some(9), Some(2));s.store_scaled_voltage(197, ctx, nodes, Some(2), Some(0), ((p[4] * p[5]) * p[219]));s.store_offset_scaled_ad(136, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p[225])), (((-1.0)) * (p[225]))), (-(1.0 - { let limited_exp_arg = ((-((p[229]) as f64).ln()) / p[228]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })), ((p[224]) * ((1.0 - { let limited_exp_arg = ((-((p[229]) as f64).ln()) / p[228]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))));s.store_div_scaled_inputs2_mixed_iai(90, 136, 1.0, A::voltage(ctx, nodes, Some(2), Some(0)), (-1.0), 36, 1.0);s.store_sqrt_offset_ad(91, A::mul_scaled_lhs(s.ad_value(90), p[230], s.ad_value(90)), 1.92);s.store_scaled_add(137, 90, 91, 0.5);s.store_add_scaled_product_indices(106, 136, 1.0, 36, 137, (-1.0));s.store_ln_ad(192, A::sub_from_scalar(1.0, A::scale(s.ad_value(106), 1.0 / (p[224]))));s.store_mul_scale_offset(193, A::sub_from_scalar(1.0, A::limited_exp_scaled_input(s.ad_value(192), (1.0 - p[228]))), A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p[225])), (((-1.0)) * (p[225]))), -((p[223] * 1.0 / ((1.0 - p[228])))), (p[224]) * ((p[223] * 1.0 / ((1.0 - p[228])))));s.store_add_scaled_inputs3_mixed_iai(194, 193, (p[4] * p[5]), A::voltage(ctx, nodes, Some(2), Some(0)), ((p[229] * p[223]) * (p[4] * p[5])), 106, ((-(p[229] * p[223])) * (p[4] * p[5])));s.b[610] = ((p[31] == 1.0) && (p[32] > 0.0));s.store_scalar(610, if s.b[610] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let (eq0_e298,) = {
    if (s.b[382] && s.b[383]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq0_value: f64 = eq0_e298;
        stamper.stamp_potential_const_local(
            0,
            eq0_value,
        );
        let (eq1_e302,) = {
    if s.b[387] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq1_value: f64 = eq1_e302;
        stamper.stamp_potential_const_local(
            1,
            eq1_value,
        );
        let (eq2_e306,) = {
    if s.b[387] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq2_value: f64 = eq2_e306;
        stamper.stamp_potential_const_local(
            2,
            eq2_value,
        );
        let (eq3_e310,) = {
    if s.b[387] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq3_value: f64 = eq3_e310;
        stamper.stamp_potential_const_local(
            3,
            eq3_value,
        );
        let (eq4_e314,) = {
    if s.b[387] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq4_value: f64 = eq4_e314;
        stamper.stamp_potential_const_local(
            4,
            eq4_value,
        );
        let (eq5_e318,) = {
    if s.b[387] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq5_value: f64 = eq5_e318;
        stamper.stamp_potential_const_local(
            5,
            eq5_value,
        );
        let (eq6_e322,) = {
    if s.b[387] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq6_value: f64 = eq6_e322;
        stamper.stamp_potential_const_local(
            6,
            eq6_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_1(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let (eq7_e331, eq7_e331_d_n0, eq7_e331_d_n1, eq7_e331_d_n2, eq7_e331_d_n3, eq7_e331_d_n4, eq7_e331_d_n5, eq7_e331_d_n6, eq7_e331_d_n7, eq7_e331_d_n8, eq7_e331_d_n9, eq7_e331_d_n10, eq7_e331_d_n11, eq7_e331_d_n12, eq7_e331_d_n13, eq7_e331_d_n14, eq7_e331_d_n15, eq7_e331_d_n16, eq7_e331_d_n17, eq7_e331_d_n18, eq7_e331_d_n19, eq7_e331_d_n20, eq7_e331_d_n21, eq7_e331_d_n22, eq7_e331_d_b0, eq7_e331_d_b1, eq7_e331_d_b2, eq7_e331_d_b3, eq7_e331_d_b4, eq7_e331_d_b5, eq7_e331_d_b6, eq7_e331_d_b7, eq7_e331_d_b8, eq7_e331_d_b9, eq7_e331_d_b10, eq7_e331_d_b11, eq7_e331_d_b12, eq7_e331_d_b13, eq7_e331_d_b14, eq7_e331_d_b15, eq7_e331_d_b16, eq7_e331_d_b17, eq7_e331_d_b18, eq7_e331_d_b19, eq7_e331_d_b20, eq7_e331_d_b21, eq7_e331_d_b22, eq7_e331_d_b23, eq7_e331_d_b24, eq7_e331_d_b25, eq7_e331_d_b26, eq7_e331_d_b27, eq7_e331_d_b28, eq7_e331_d_b29, eq7_e331_d_b30, eq7_e331_d_b31, eq7_e331_d_b32, eq7_e331_d_b33, eq7_e331_d_b34, eq7_e331_d_b35, eq7_e331_d_b36, eq7_e331_d_b37, eq7_e331_d_b38, eq7_e331_d_b39, eq7_e331_d_b40, eq7_e331_d_b41, eq7_e331_d_b42, eq7_e331_d_b43, eq7_e331_d_b44, eq7_e331_d_b45, eq7_e331_d_b46, eq7_e331_d_b47, eq7_e331_d_b48, eq7_e331_d_b49, eq7_e331_d_b50, eq7_e331_d_b51, eq7_e331_d_b52, eq7_e331_d_b53, eq7_e331_d_b54,) = {
    if (s.b[388] && (!s.b[387])) {
        let eq7_e329: f64 = (s.v[38] * s.v[38]);let eq7_e329_d_n0: f64 = ((s.dn[38][0] * s.v[38]) + (s.v[38] * s.dn[38][0]));let eq7_e329_d_n1: f64 = ((s.dn[38][1] * s.v[38]) + (s.v[38] * s.dn[38][1]));let eq7_e329_d_n2: f64 = ((s.dn[38][2] * s.v[38]) + (s.v[38] * s.dn[38][2]));let eq7_e329_d_n3: f64 = ((s.dn[38][3] * s.v[38]) + (s.v[38] * s.dn[38][3]));let eq7_e329_d_n4: f64 = ((s.dn[38][4] * s.v[38]) + (s.v[38] * s.dn[38][4]));let eq7_e329_d_n5: f64 = ((s.dn[38][5] * s.v[38]) + (s.v[38] * s.dn[38][5]));let eq7_e329_d_n6: f64 = ((s.dn[38][6] * s.v[38]) + (s.v[38] * s.dn[38][6]));let eq7_e329_d_n7: f64 = ((s.dn[38][7] * s.v[38]) + (s.v[38] * s.dn[38][7]));let eq7_e329_d_n8: f64 = ((s.dn[38][8] * s.v[38]) + (s.v[38] * s.dn[38][8]));let eq7_e329_d_n9: f64 = ((s.dn[38][9] * s.v[38]) + (s.v[38] * s.dn[38][9]));let eq7_e329_d_n10: f64 = ((s.dn[38][10] * s.v[38]) + (s.v[38] * s.dn[38][10]));let eq7_e329_d_n11: f64 = ((s.dn[38][11] * s.v[38]) + (s.v[38] * s.dn[38][11]));let eq7_e329_d_n12: f64 = ((s.dn[38][12] * s.v[38]) + (s.v[38] * s.dn[38][12]));let eq7_e329_d_n13: f64 = ((s.dn[38][13] * s.v[38]) + (s.v[38] * s.dn[38][13]));let eq7_e329_d_n14: f64 = ((s.dn[38][14] * s.v[38]) + (s.v[38] * s.dn[38][14]));let eq7_e329_d_n15: f64 = ((s.dn[38][15] * s.v[38]) + (s.v[38] * s.dn[38][15]));let eq7_e329_d_n16: f64 = ((s.dn[38][16] * s.v[38]) + (s.v[38] * s.dn[38][16]));let eq7_e329_d_n17: f64 = ((s.dn[38][17] * s.v[38]) + (s.v[38] * s.dn[38][17]));let eq7_e329_d_n18: f64 = ((s.dn[38][18] * s.v[38]) + (s.v[38] * s.dn[38][18]));let eq7_e329_d_n19: f64 = ((s.dn[38][19] * s.v[38]) + (s.v[38] * s.dn[38][19]));let eq7_e329_d_n20: f64 = ((s.dn[38][20] * s.v[38]) + (s.v[38] * s.dn[38][20]));let eq7_e329_d_n21: f64 = ((s.dn[38][21] * s.v[38]) + (s.v[38] * s.dn[38][21]));let eq7_e329_d_n22: f64 = ((s.dn[38][22] * s.v[38]) + (s.v[38] * s.dn[38][22]));let eq7_e329_d_b0: f64 = ((s.db[38][0] * s.v[38]) + (s.v[38] * s.db[38][0]));let eq7_e329_d_b1: f64 = ((s.db[38][1] * s.v[38]) + (s.v[38] * s.db[38][1]));let eq7_e329_d_b2: f64 = ((s.db[38][2] * s.v[38]) + (s.v[38] * s.db[38][2]));let eq7_e329_d_b3: f64 = ((s.db[38][3] * s.v[38]) + (s.v[38] * s.db[38][3]));let eq7_e329_d_b4: f64 = ((s.db[38][4] * s.v[38]) + (s.v[38] * s.db[38][4]));let eq7_e329_d_b5: f64 = ((s.db[38][5] * s.v[38]) + (s.v[38] * s.db[38][5]));let eq7_e329_d_b6: f64 = ((s.db[38][6] * s.v[38]) + (s.v[38] * s.db[38][6]));let eq7_e329_d_b7: f64 = ((s.db[38][7] * s.v[38]) + (s.v[38] * s.db[38][7]));let eq7_e329_d_b8: f64 = ((s.db[38][8] * s.v[38]) + (s.v[38] * s.db[38][8]));let eq7_e329_d_b9: f64 = ((s.db[38][9] * s.v[38]) + (s.v[38] * s.db[38][9]));let eq7_e329_d_b10: f64 = ((s.db[38][10] * s.v[38]) + (s.v[38] * s.db[38][10]));let eq7_e329_d_b11: f64 = ((s.db[38][11] * s.v[38]) + (s.v[38] * s.db[38][11]));let eq7_e329_d_b12: f64 = ((s.db[38][12] * s.v[38]) + (s.v[38] * s.db[38][12]));let eq7_e329_d_b13: f64 = ((s.db[38][13] * s.v[38]) + (s.v[38] * s.db[38][13]));let eq7_e329_d_b14: f64 = ((s.db[38][14] * s.v[38]) + (s.v[38] * s.db[38][14]));let eq7_e329_d_b15: f64 = ((s.db[38][15] * s.v[38]) + (s.v[38] * s.db[38][15]));let eq7_e329_d_b16: f64 = ((s.db[38][16] * s.v[38]) + (s.v[38] * s.db[38][16]));let eq7_e329_d_b17: f64 = ((s.db[38][17] * s.v[38]) + (s.v[38] * s.db[38][17]));let eq7_e329_d_b18: f64 = ((s.db[38][18] * s.v[38]) + (s.v[38] * s.db[38][18]));let eq7_e329_d_b19: f64 = ((s.db[38][19] * s.v[38]) + (s.v[38] * s.db[38][19]));let eq7_e329_d_b20: f64 = ((s.db[38][20] * s.v[38]) + (s.v[38] * s.db[38][20]));let eq7_e329_d_b21: f64 = ((s.db[38][21] * s.v[38]) + (s.v[38] * s.db[38][21]));let eq7_e329_d_b22: f64 = ((s.db[38][22] * s.v[38]) + (s.v[38] * s.db[38][22]));let eq7_e329_d_b23: f64 = ((s.db[38][23] * s.v[38]) + (s.v[38] * s.db[38][23]));let eq7_e329_d_b24: f64 = ((s.db[38][24] * s.v[38]) + (s.v[38] * s.db[38][24]));let eq7_e329_d_b25: f64 = ((s.db[38][25] * s.v[38]) + (s.v[38] * s.db[38][25]));let eq7_e329_d_b26: f64 = ((s.db[38][26] * s.v[38]) + (s.v[38] * s.db[38][26]));let eq7_e329_d_b27: f64 = ((s.db[38][27] * s.v[38]) + (s.v[38] * s.db[38][27]));
        let eq7_e329_d_b28: f64 = ((s.db[38][28] * s.v[38]) + (s.v[38] * s.db[38][28]));let eq7_e329_d_b29: f64 = ((s.db[38][29] * s.v[38]) + (s.v[38] * s.db[38][29]));let eq7_e329_d_b30: f64 = ((s.db[38][30] * s.v[38]) + (s.v[38] * s.db[38][30]));let eq7_e329_d_b31: f64 = ((s.db[38][31] * s.v[38]) + (s.v[38] * s.db[38][31]));let eq7_e329_d_b32: f64 = ((s.db[38][32] * s.v[38]) + (s.v[38] * s.db[38][32]));let eq7_e329_d_b33: f64 = ((s.db[38][33] * s.v[38]) + (s.v[38] * s.db[38][33]));let eq7_e329_d_b34: f64 = ((s.db[38][34] * s.v[38]) + (s.v[38] * s.db[38][34]));let eq7_e329_d_b35: f64 = ((s.db[38][35] * s.v[38]) + (s.v[38] * s.db[38][35]));let eq7_e329_d_b36: f64 = ((s.db[38][36] * s.v[38]) + (s.v[38] * s.db[38][36]));let eq7_e329_d_b37: f64 = ((s.db[38][37] * s.v[38]) + (s.v[38] * s.db[38][37]));let eq7_e329_d_b38: f64 = ((s.db[38][38] * s.v[38]) + (s.v[38] * s.db[38][38]));let eq7_e329_d_b39: f64 = ((s.db[38][39] * s.v[38]) + (s.v[38] * s.db[38][39]));let eq7_e329_d_b40: f64 = ((s.db[38][40] * s.v[38]) + (s.v[38] * s.db[38][40]));let eq7_e329_d_b41: f64 = ((s.db[38][41] * s.v[38]) + (s.v[38] * s.db[38][41]));let eq7_e329_d_b42: f64 = ((s.db[38][42] * s.v[38]) + (s.v[38] * s.db[38][42]));let eq7_e329_d_b43: f64 = ((s.db[38][43] * s.v[38]) + (s.v[38] * s.db[38][43]));let eq7_e329_d_b44: f64 = ((s.db[38][44] * s.v[38]) + (s.v[38] * s.db[38][44]));let eq7_e329_d_b45: f64 = ((s.db[38][45] * s.v[38]) + (s.v[38] * s.db[38][45]));let eq7_e329_d_b46: f64 = ((s.db[38][46] * s.v[38]) + (s.v[38] * s.db[38][46]));let eq7_e329_d_b47: f64 = ((s.db[38][47] * s.v[38]) + (s.v[38] * s.db[38][47]));let eq7_e329_d_b48: f64 = ((s.db[38][48] * s.v[38]) + (s.v[38] * s.db[38][48]));let eq7_e329_d_b49: f64 = ((s.db[38][49] * s.v[38]) + (s.v[38] * s.db[38][49]));let eq7_e329_d_b50: f64 = ((s.db[38][50] * s.v[38]) + (s.v[38] * s.db[38][50]));let eq7_e329_d_b51: f64 = ((s.db[38][51] * s.v[38]) + (s.v[38] * s.db[38][51]));let eq7_e329_d_b52: f64 = ((s.db[38][52] * s.v[38]) + (s.v[38] * s.db[38][52]));let eq7_e329_d_b53: f64 = ((s.db[38][53] * s.v[38]) + (s.v[38] * s.db[38][53]));let eq7_e329_d_b54: f64 = ((s.db[38][54] * s.v[38]) + (s.v[38] * s.db[38][54]));
        (eq7_e329, eq7_e329_d_n0, eq7_e329_d_n1, eq7_e329_d_n2, eq7_e329_d_n3, eq7_e329_d_n4, eq7_e329_d_n5, eq7_e329_d_n6, eq7_e329_d_n7, eq7_e329_d_n8, eq7_e329_d_n9, eq7_e329_d_n10, eq7_e329_d_n11, eq7_e329_d_n12, eq7_e329_d_n13, eq7_e329_d_n14, eq7_e329_d_n15, eq7_e329_d_n16, eq7_e329_d_n17, eq7_e329_d_n18, eq7_e329_d_n19, eq7_e329_d_n20, eq7_e329_d_n21, eq7_e329_d_n22, eq7_e329_d_b0, eq7_e329_d_b1, eq7_e329_d_b2, eq7_e329_d_b3, eq7_e329_d_b4, eq7_e329_d_b5, eq7_e329_d_b6, eq7_e329_d_b7, eq7_e329_d_b8, eq7_e329_d_b9, eq7_e329_d_b10, eq7_e329_d_b11, eq7_e329_d_b12, eq7_e329_d_b13, eq7_e329_d_b14, eq7_e329_d_b15, eq7_e329_d_b16, eq7_e329_d_b17, eq7_e329_d_b18, eq7_e329_d_b19, eq7_e329_d_b20, eq7_e329_d_b21, eq7_e329_d_b22, eq7_e329_d_b23, eq7_e329_d_b24, eq7_e329_d_b25, eq7_e329_d_b26, eq7_e329_d_b27, eq7_e329_d_b28, eq7_e329_d_b29, eq7_e329_d_b30, eq7_e329_d_b31, eq7_e329_d_b32, eq7_e329_d_b33, eq7_e329_d_b34, eq7_e329_d_b35, eq7_e329_d_b36, eq7_e329_d_b37, eq7_e329_d_b38, eq7_e329_d_b39, eq7_e329_d_b40, eq7_e329_d_b41, eq7_e329_d_b42, eq7_e329_d_b43, eq7_e329_d_b44, eq7_e329_d_b45, eq7_e329_d_b46, eq7_e329_d_b47, eq7_e329_d_b48, eq7_e329_d_b49, eq7_e329_d_b50, eq7_e329_d_b51, eq7_e329_d_b52, eq7_e329_d_b53, eq7_e329_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e331;let eq7_node_derivatives: [f64; 23] = [eq7_e331_d_n0, eq7_e331_d_n1, eq7_e331_d_n2, eq7_e331_d_n3, eq7_e331_d_n4, eq7_e331_d_n5, eq7_e331_d_n6, eq7_e331_d_n7, eq7_e331_d_n8, eq7_e331_d_n9, eq7_e331_d_n10, eq7_e331_d_n11, eq7_e331_d_n12, eq7_e331_d_n13, eq7_e331_d_n14, eq7_e331_d_n15, eq7_e331_d_n16, eq7_e331_d_n17, eq7_e331_d_n18, eq7_e331_d_n19, eq7_e331_d_n20, eq7_e331_d_n21, eq7_e331_d_n22];let eq7_branch_derivatives: [f64; 55] = [eq7_e331_d_b0, eq7_e331_d_b1, eq7_e331_d_b2, eq7_e331_d_b3, eq7_e331_d_b4, eq7_e331_d_b5, eq7_e331_d_b6, eq7_e331_d_b7, eq7_e331_d_b8, eq7_e331_d_b9, eq7_e331_d_b10, eq7_e331_d_b11, eq7_e331_d_b12, eq7_e331_d_b13, eq7_e331_d_b14, eq7_e331_d_b15, eq7_e331_d_b16, eq7_e331_d_b17, eq7_e331_d_b18, eq7_e331_d_b19, eq7_e331_d_b20, eq7_e331_d_b21, eq7_e331_d_b22, eq7_e331_d_b23, eq7_e331_d_b24, eq7_e331_d_b25, eq7_e331_d_b26, eq7_e331_d_b27, eq7_e331_d_b28, eq7_e331_d_b29, eq7_e331_d_b30, eq7_e331_d_b31, eq7_e331_d_b32, eq7_e331_d_b33, eq7_e331_d_b34, eq7_e331_d_b35, eq7_e331_d_b36, eq7_e331_d_b37, eq7_e331_d_b38, eq7_e331_d_b39, eq7_e331_d_b40, eq7_e331_d_b41, eq7_e331_d_b42, eq7_e331_d_b43, eq7_e331_d_b44, eq7_e331_d_b45, eq7_e331_d_b46, eq7_e331_d_b47, eq7_e331_d_b48, eq7_e331_d_b49, eq7_e331_d_b50, eq7_e331_d_b51, eq7_e331_d_b52, eq7_e331_d_b53, eq7_e331_d_b54];
        stamper.stamp_potential_dense_local(
            7,
            eq7_value,
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
        );
    }
}
