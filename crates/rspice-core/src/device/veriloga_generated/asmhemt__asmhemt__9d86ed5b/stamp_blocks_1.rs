#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[433] && s.b[434]) && (!s.b[439])) {s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (s.b[433] && s.b[434]) {s.store_sub_div_rhs_indices(100, 130, 156, 99);}
        s.b[440] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);s.store_scalar(440, if s.b[440] { 1.0 } else { 0.0 });
        if ((s.b[433] && s.b[434]) && s.b[440]) {s.store_sub(101, 130, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p.p169);s.store_scaled_mul(103, 136, 90, p.p170);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
        if ((s.b[433] && s.b[434]) && s.b[440]) {
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
        if ((s.b[433] && s.b[434]) && s.b[440]) {s.store_scaled_mul(107, 136, 91, p.p169);s.store_scaled_mul(108, 136, 91, p.p170);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 130, 114);s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_mul_scaled_powf_rhs(116, 136, p.p169, 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p.p170, 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
        if ((s.b[433] && s.b[434]) && s.b[440]) {
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
        if ((s.b[433] && s.b[434]) && s.b[440]) {s.store_mul_scaled_powf_rhs(121, 136, p.p169, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(122, 136, p.p170, 115, (-0.3333333333333333));s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.store_add(225, 128, 86);}
        if ((s.b[433] && s.b[434]) && (!s.b[440])) {s.store_add(225, 100, 86);}
        if (s.b[433] && s.b[434]) {s.store_scaled_add(226, 224, 225, 0.5);s.store_sub(227, 225, 224);s.store_mul_add_scaled_inputs3_offset_rhs_indices(135, 227, 222, 1.0, 226, (-1.0), 83, 1.0, 0.0);s.store_mul_scaled_abs_ad_rhs(136, 223, 1.0 / (p.p9), A::sub(s.ad_value(222), s.ad_value(226)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[433] && s.b[434]) {s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);s.store_scaled_mul(96, 95, 223, (p.p4 * (p.p5 * 1.0 / (p.p161))));s.store_mul_scale_offset_mixed_ia(98, 96, A::sub_scaled_inputs(s.ad_value(232), p.p21, s.ad_value(86), p.p21), 1.0, 1.0);s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(227), (p.p25 * p.p25), s.ad_value(227)), 1.0);s.store_div(93, 98, 92);s.store_mul(233, 93, 135);s.store_sub(90, 225, 224);s.store_add_scaled_inputs3_indices(91, 222, 1.0, 83, 1.0, 226, -1.0);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(137, 223, 222, ((p.p4 * p.p5) * p.p161), 226, (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);s.store_scale(188, 137, (1.0 / (p.p236) * 1e26));s.store_offset_powf_ad(189, s.ad_value(188), p.p235, 1.0);s.store_div_from_scalar(190, p.p234, 189);s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p160);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(228, 191, 222, ((p.p4 * p.p5) * p.p161), 226, (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);s.store_add_scaled_inputs3_indices(136, 222, 1.0, 83, 1.0, 226, -1.0);s.store_add_scaled_inputs(90, 224, 0.3333333333333333, 225, (2.0 * 0.3333333333333333));s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(227)), (1.0 / 12.0), 136, 1.0);s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(227)), 227, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);s.store_mul_add_scaled_inputs4_indices_rhs(229, 191, 222, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p161) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p161) * p.p5) * 0.5)));}
        s.b[441] = (s.v[48] < 0.0);s.store_scalar(441, if s.b[441] { 1.0 } else { 0.0 });
        if ((s.b[433] && s.b[434]) && s.b[441]) {s.store_sub_scaled_inputs(229, 228, (-1.0), 229, 1.0);}
        if (s.b[433] && (!s.b[434])) {s.store_scalar(228, 0.0);s.store_scalar(229, 0.0);}
        s.b[442] = (p.p150 != 0.0);s.store_scalar(442, if s.b[442] { 1.0 } else { 0.0 });s.b[443] = (p.p150 == 1.0);s.store_scalar(443, if s.b[443] { 1.0 } else { 0.0 });
        if (((!s.b[433]) && s.b[442]) && s.b[443]) {s.store_voltage(50, ctx, nodes, Some(9), Some(7));}
        if (((!s.b[433]) && s.b[442]) && (!s.b[443])) {s.store_voltage(50, ctx, nodes, Some(2), Some(7));}
        if ((!s.b[433]) && s.b[442]) {s.copy_ad(230, 50);s.store_scalar(146, (1.0 + p.p165));s.store_scaled_mul(83, 82, 146, 8.617087e-5);s.store_offset_scaled(88, 82, ((1.0 / (s.v[35])) * (p.p162)), (((((-1.0)) * (p.p162))) + (p.p159)));s.store_scalar(223, (p.p9 / p.p160));s.store_div_scalar_by_product_indices(136, p.p161, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));s.store_add_scaled_product_mixed_iia(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p158), 1.0);s.store_add_scaled_inputs4_mixed_iiai(160, 230, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(230), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);s.store_sub(222, 160, 88);s.store_div_scaled_inputs_indices(84, 223, 1.0, 83, (1.602176634e-19 * 3.24e17));s.store_div_from_scalar(150, 2.718281828459045, 84);s.store_div_from_scalar(151, 1.0, 84);s.store_primal_scale(99, 223, 6.241509074460763e18);s.store_scaled_add_mixed_ia(154, 222, A::sqrt_square_offset(s.ad_value(222), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[433]) && s.b[442]) {let t4: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, t4, (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(t4, ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 222, 1.0, 83, 2.0);}
        s.b[444] = (s.v[136] < 200.0);s.store_scalar(444, if s.b[444] { 1.0 } else { 0.0 });
        if (((!s.b[433]) && s.b[442]) && s.b[444]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(222), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (((!s.b[433]) && s.b[442]) && (!s.b[444])) {s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(222), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((!s.b[433]) && s.b[442]) {s.store_sub_div_rhs_indices(100, 222, 153, 99);}
        s.b[445] = ((((s.v[100] - s.v[222])) as f64).abs() > 1e-19);s.store_scalar(445, if s.b[445] { 1.0 } else { 0.0 });
        if (((!s.b[433]) && s.b[442]) && s.b[445]) {s.store_sub(101, 222, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p.p169);s.store_scaled_mul(103, 136, 90, p.p170);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
        if (((!s.b[433]) && s.b[442]) && s.b[445]) {
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
        if (((!s.b[433]) && s.b[442]) && s.b[445]) {s.store_scaled_mul(107, 136, 91, p.p169);s.store_scaled_mul(108, 136, 91, p.p170);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 222, 114);s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(137, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(116, 136, p.p169, 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p.p170, 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[433]) && s.b[442]) && s.b[445]) {
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
        if (((!s.b[433]) && s.b[442]) && s.b[445]) {s.store_scaled_mul(121, 136, 137, p.p169);s.store_scaled_mul(122, 136, 137, p.p170);s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.copy_ad(224, 128);}
        if (((!s.b[433]) && s.b[442]) && (!s.b[445])) {s.copy_ad(224, 100);}
        if ((!s.b[433]) && s.b[442]) {s.store_scalar(231, 0.0);s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p163);s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p164);s.store_mul_scaled_abs_ad_rhs(136, 223, 1.0 / (p.p9), A::sub(s.ad_value(222), s.ad_value(224)));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(224)), (s.v[81] / p.p9));s.store_div_mixed_ia(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);s.store_scaled_add_mixed_ia(90, 222, A::sqrt_square_offset(s.ad_value(222), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p161, 136, p.p161, 90, 1.0, 1.0);s.store_powf_ad(136, A::div(s.ad_value(231), s.ad_value(85)), p.p18);s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));s.store_mul(86, 231, 90);s.store_sub(39, 222, 86);s.copy_ad(130, 39);s.store_scaled_add_mixed_ia(131, 130, A::sqrt_square_offset(s.ad_value(130), ((4.0 * 0.3) * 0.3)), 0.5);s.copy_ad(154, 131);s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);let t5: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, t5, (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(t5, ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);}
        s.b[446] = (s.v[136] < 200.0);s.store_scalar(446, if s.b[446] { 1.0 } else { 0.0 });
        if (((!s.b[433]) && s.b[442]) && s.b[446]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[433]) && s.b[442]) && s.b[446]) {s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (((!s.b[433]) && s.b[442]) && (!s.b[446])) {s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((!s.b[433]) && s.b[442]) {s.store_sub_div_rhs_indices(100, 130, 156, 99);}
        s.b[447] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);s.store_scalar(447, if s.b[447] { 1.0 } else { 0.0 });
        if (((!s.b[433]) && s.b[442]) && s.b[447]) {s.store_sub(101, 130, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p.p169);s.store_scaled_mul(103, 136, 90, p.p170);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
        if (((!s.b[433]) && s.b[442]) && s.b[447]) {
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
        if (((!s.b[433]) && s.b[442]) && s.b[447]) {s.store_scaled_mul(107, 136, 91, p.p169);s.store_scaled_mul(108, 136, 91, p.p170);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 130, 114);s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_mul_scaled_powf_rhs(116, 136, p.p169, 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p.p170, 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
        if (((!s.b[433]) && s.b[442]) && s.b[447]) {
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
        if (((!s.b[433]) && s.b[442]) && s.b[447]) {s.store_mul_scaled_powf_rhs(121, 136, p.p169, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(122, 136, p.p170, 115, (-0.3333333333333333));s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[433]) && s.b[442]) && s.b[447]) {s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.store_add(225, 128, 86);}
        if (((!s.b[433]) && s.b[442]) && (!s.b[447])) {s.store_add(225, 100, 86);}
        if ((!s.b[433]) && s.b[442]) {s.store_scaled_add(226, 224, 225, 0.5);s.store_sub(227, 225, 224);s.store_sub(90, 225, 224);s.store_add_scaled_inputs3_indices(91, 222, 1.0, 83, 1.0, 226, -1.0);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(137, 223, 222, ((p.p4 * p.p5) * p.p161), 226, (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);s.store_scale(188, 137, (1.0 / (p.p236) * 1e26));s.store_offset_powf_ad(189, s.ad_value(188), p.p235, 1.0);s.store_div_from_scalar(190, p.p234, 189);s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p160);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(228, 191, 222, ((p.p4 * p.p5) * p.p161), 226, (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);s.store_add_scaled_inputs3_indices(136, 222, 1.0, 83, 1.0, 226, -1.0);s.store_add_scaled_inputs(90, 224, 0.3333333333333333, 225, (2.0 * 0.3333333333333333));s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(227)), (1.0 / 12.0), 136, 1.0);s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(227)), 227, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);s.store_mul_add_scaled_inputs4_indices_rhs(229, 191, 222, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p161) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p161) * p.p5) * 0.5)));}
        if ((!s.b[433]) && (!s.b[442])) {s.store_scalar(228, 0.0);s.store_scalar(229, 0.0);}
        s.b[448] = (p.p149 == 0.0);s.store_scalar(448, if s.b[448] { 1.0 } else { 0.0 });s.b[449] = (p.p151 != 0.0);s.store_scalar(449, if s.b[449] { 1.0 } else { 0.0 });
        if (s.b[448] && s.b[449]) {s.store_voltage(53, ctx, nodes, Some(8), Some(19));}
        s.b[450] = (p.p151 == 1.0);s.store_scalar(450, if s.b[450] { 1.0 } else { 0.0 });
        if ((s.b[448] && s.b[449]) && s.b[450]) {s.store_voltage(54, ctx, nodes, Some(9), Some(19));s.store_voltage(55, ctx, nodes, Some(9), Some(8));}
        if ((s.b[448] && s.b[449]) && (!s.b[450])) {s.store_voltage(54, ctx, nodes, Some(2), Some(19));s.store_voltage(55, ctx, nodes, Some(2), Some(8));}
        if (s.b[448] && s.b[449]) {s.store_scalar(52, 1.0);}
        s.b[451] = (s.v[53] < 0.0);s.store_scalar(451, if s.b[451] { 1.0 } else { 0.0 });
        if ((s.b[448] && s.b[449]) && s.b[451]) {s.store_scalar(52, (-1.0));s.store_mul(243, 52, 53);s.copy_ad(242, 55);}
        if ((s.b[448] && s.b[449]) && (!s.b[451])) {s.copy_ad(243, 53);s.copy_ad(242, 54);}
        if (s.b[448] && s.b[449]) {s.store_offset_sqrt_ad(244, A::offset(A::square(s.ad_value(243)), 0.01), (-0.1));s.store_offset_scaled(146, 244, p.p166, (1.0 + p.p165));s.store_scaled_mul(83, 82, 146, 8.617087e-5);s.store_sub_ad(88, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p162)), (((((-1.0)) * (p.p162))) + (p.p159))), A::div_scaled_inputs(s.ad_value(244), (p.p168 * p.p167), A::sqrt_square_offset(s.ad_value(244), (p.p168 * p.p168)), 1.0));s.store_scalar(235, (p.p9 / p.p160));s.store_div_scalar_by_product_indices(136, p.p161, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));s.store_add_scaled_product_mixed_iia(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p158), 1.0);s.store_add_scaled_inputs4_mixed_iiai(160, 242, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(242), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);s.store_sub(234, 160, 88);s.store_div_scaled_inputs_indices(84, 235, 1.0, 83, (1.602176634e-19 * 3.24e17));s.store_div_from_scalar(150, 2.718281828459045, 84);s.store_div_from_scalar(151, 1.0, 84);s.store_primal_scale(99, 235, 6.241509074460763e18);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[448] && s.b[449]) {s.store_scaled_add_mixed_ia(154, 234, A::sqrt_square_offset(s.ad_value(234), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);let t6: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, t6, (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(t6, ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 234, 1.0, 83, 2.0);}
        s.b[452] = (s.v[136] < 200.0);s.store_scalar(452, if s.b[452] { 1.0 } else { 0.0 });
        if ((s.b[448] && s.b[449]) && s.b[452]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(234), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((s.b[448] && s.b[449]) && (!s.b[452])) {s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(234), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (s.b[448] && s.b[449]) {s.store_sub_div_rhs_indices(100, 234, 153, 99);}
        s.b[453] = ((((s.v[100] - s.v[234])) as f64).abs() > 1e-19);s.store_scalar(453, if s.b[453] { 1.0 } else { 0.0 });
        if ((s.b[448] && s.b[449]) && s.b[453]) {s.store_sub(101, 234, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p.p169);s.store_scaled_mul(103, 136, 90, p.p170);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
        if ((s.b[448] && s.b[449]) && s.b[453]) {
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
        if ((s.b[448] && s.b[449]) && s.b[453]) {s.store_scaled_mul(107, 136, 91, p.p169);s.store_scaled_mul(108, 136, 91, p.p170);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 234, 114);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[448] && s.b[449]) && s.b[453]) {s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(137, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(116, 136, p.p169, 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p.p170, 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
        if ((s.b[448] && s.b[449]) && s.b[453]) {
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
        if ((s.b[448] && s.b[449]) && s.b[453]) {s.store_scaled_mul(121, 136, 137, p.p169);s.store_scaled_mul(122, 136, 137, p.p170);s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.copy_ad(236, 128);}
        if ((s.b[448] && s.b[449]) && (!s.b[453])) {s.copy_ad(236, 100);}
        if (s.b[448] && s.b[449]) {s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p163);s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p164);s.store_mul_scaled_abs_ad_rhs(136, 235, 1.0 / (p.p9), A::sub(s.ad_value(234), s.ad_value(236)));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(236)), (s.v[81] / p.p9));s.store_div_mixed_ia(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);s.store_scaled_add_mixed_ia(90, 234, A::sqrt_square_offset(s.ad_value(234), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p161, 136, p.p161, 90, 1.0, 1.0);s.store_powf_ad(136, A::div(s.ad_value(243), s.ad_value(85)), p.p18);s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));s.store_mul(86, 243, 90);s.store_sub(39, 234, 86);s.copy_ad(130, 39);s.store_scaled_add_mixed_ia(131, 130, A::sqrt_square_offset(s.ad_value(130), ((4.0 * 0.3) * 0.3)), 0.5);s.copy_ad(154, 131);s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[448] && s.b[449]) {let t0: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, t0, (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(t0, ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);}
        s.b[454] = (s.v[136] < 200.0);s.store_scalar(454, if s.b[454] { 1.0 } else { 0.0 });
        if ((s.b[448] && s.b[449]) && s.b[454]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((s.b[448] && s.b[449]) && (!s.b[454])) {s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (s.b[448] && s.b[449]) {s.store_sub_div_rhs_indices(100, 130, 156, 99);}
        s.b[455] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);s.store_scalar(455, if s.b[455] { 1.0 } else { 0.0 });
        if ((s.b[448] && s.b[449]) && s.b[455]) {s.store_sub(101, 130, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p.p169);s.store_scaled_mul(103, 136, 90, p.p170);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
        if ((s.b[448] && s.b[449]) && s.b[455]) {
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
        if ((s.b[448] && s.b[449]) && s.b[455]) {s.store_scaled_mul(107, 136, 91, p.p169);s.store_scaled_mul(108, 136, 91, p.p170);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 130, 114);s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_mul_scaled_powf_rhs(116, 136, p.p169, 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p.p170, 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[448] && s.b[449]) && s.b[455]) {
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
        if ((s.b[448] && s.b[449]) && s.b[455]) {s.store_mul_scaled_powf_rhs(121, 136, p.p169, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(122, 136, p.p170, 115, (-0.3333333333333333));s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.store_add(237, 128, 86);}
        if ((s.b[448] && s.b[449]) && (!s.b[455])) {s.store_add(237, 100, 86);}
        if (s.b[448] && s.b[449]) {s.store_scaled_add(238, 236, 237, 0.5);s.store_sub(239, 237, 236);s.store_mul_add_scaled_inputs3_offset_rhs_indices(135, 239, 234, 1.0, 238, (-1.0), 83, 1.0, 0.0);s.store_mul_scaled_abs_ad_rhs(136, 235, 1.0 / (p.p9), A::sub(s.ad_value(234), s.ad_value(238)));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);s.store_scaled_mul(96, 95, 235, (p.p4 * (p.p5 * 1.0 / (p.p161))));s.store_mul_scale_offset_mixed_ia(98, 96, A::sub_scaled_inputs(s.ad_value(244), p.p21, s.ad_value(86), p.p21), 1.0, 1.0);s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(239), (p.p25 * p.p25), s.ad_value(239)), 1.0);s.store_div(93, 98, 92);s.store_mul(245, 93, 135);s.store_sub(90, 237, 236);s.store_add_scaled_inputs3_indices(91, 234, 1.0, 83, 1.0, 238, -1.0);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(137, 235, 234, ((p.p4 * p.p5) * p.p161), 238, (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);s.store_scale(188, 137, (1.0 / (p.p236) * 1e26));s.store_offset_powf_ad(189, s.ad_value(188), p.p235, 1.0);s.store_div_from_scalar(190, p.p234, 189);s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p160);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(240, 191, 234, ((p.p4 * p.p5) * p.p161), 238, (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);s.store_add_scaled_inputs3_indices(136, 234, 1.0, 83, 1.0, 238, -1.0);s.store_add_scaled_inputs(90, 236, 0.3333333333333333, 237, (2.0 * 0.3333333333333333));s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(239)), (1.0 / 12.0), 136, 1.0);s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(239)), 239, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);s.store_mul_add_scaled_inputs4_indices_rhs(241, 191, 234, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p161) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p161) * p.p5) * 0.5)));}
        s.b[456] = (s.v[52] < 0.0);s.store_scalar(456, if s.b[456] { 1.0 } else { 0.0 });
        if ((s.b[448] && s.b[449]) && s.b[456]) {s.store_sub_scaled_inputs(241, 240, (-1.0), 241, 1.0);}
        if (s.b[448] && (!s.b[449])) {s.store_scalar(240, 0.0);s.store_scalar(241, 0.0);}
        s.b[457] = (p.p151 != 0.0);s.store_scalar(457, if s.b[457] { 1.0 } else { 0.0 });s.b[458] = (p.p151 == 1.0);s.store_scalar(458, if s.b[458] { 1.0 } else { 0.0 });
        if (((!s.b[448]) && s.b[457]) && s.b[458]) {s.store_voltage(54, ctx, nodes, Some(9), Some(8));}
        if (((!s.b[448]) && s.b[457]) && (!s.b[458])) {s.store_voltage(54, ctx, nodes, Some(2), Some(8));}
        if ((!s.b[448]) && s.b[457]) {s.copy_ad(234, 54);s.store_scalar(146, (1.0 + p.p165));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[448]) && s.b[457]) {s.store_scaled_mul(83, 82, 146, 8.617087e-5);s.store_offset_scaled(88, 82, ((1.0 / (s.v[35])) * (p.p162)), (((((-1.0)) * (p.p162))) + (p.p159)));s.store_scalar(235, (p.p9 / p.p160));s.store_div_scalar_by_product_indices(136, p.p161, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));s.store_add_scaled_product_mixed_iia(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p158), 1.0);s.store_add_scaled_inputs4_mixed_iiai(160, 242, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(242), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);s.store_sub(234, 160, 88);s.store_div_scaled_inputs_indices(84, 235, 1.0, 83, (1.602176634e-19 * 3.24e17));s.store_div_from_scalar(150, 2.718281828459045, 84);s.store_div_from_scalar(151, 1.0, 84);s.store_primal_scale(99, 235, 6.241509074460763e18);s.store_scaled_add_mixed_ia(154, 234, A::sqrt_square_offset(s.ad_value(234), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);let t1: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, t1, (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(t1, ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 234, 1.0, 83, 2.0);}
        s.b[459] = (s.v[136] < 200.0);s.store_scalar(459, if s.b[459] { 1.0 } else { 0.0 });
        if (((!s.b[448]) && s.b[457]) && s.b[459]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(234), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (((!s.b[448]) && s.b[457]) && (!s.b[459])) {s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(234), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((!s.b[448]) && s.b[457]) {s.store_sub_div_rhs_indices(100, 234, 153, 99);}
        s.b[460] = ((((s.v[100] - s.v[234])) as f64).abs() > 1e-19);s.store_scalar(460, if s.b[460] { 1.0 } else { 0.0 });
        if (((!s.b[448]) && s.b[457]) && s.b[460]) {s.store_sub(101, 234, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p.p169);s.store_scaled_mul(103, 136, 90, p.p170);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[448]) && s.b[457]) && s.b[460]) {
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
        if (((!s.b[448]) && s.b[457]) && s.b[460]) {s.store_scaled_mul(107, 136, 91, p.p169);s.store_scaled_mul(108, 136, 91, p.p170);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 234, 114);s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(137, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(116, 136, p.p169, 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p.p170, 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
        if (((!s.b[448]) && s.b[457]) && s.b[460]) {
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
        if (((!s.b[448]) && s.b[457]) && s.b[460]) {s.store_scaled_mul(121, 136, 137, p.p169);s.store_scaled_mul(122, 136, 137, p.p170);s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.copy_ad(236, 128);}
        if (((!s.b[448]) && s.b[457]) && (!s.b[460])) {s.copy_ad(236, 100);}
        if ((!s.b[448]) && s.b[457]) {s.store_scalar(243, 0.0);s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p163);s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p164);s.store_mul_scaled_abs_ad_rhs(136, 235, 1.0 / (p.p9), A::sub(s.ad_value(234), s.ad_value(236)));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(236)), (s.v[81] / p.p9));s.store_div_mixed_ia(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);s.store_scaled_add_mixed_ia(90, 234, A::sqrt_square_offset(s.ad_value(234), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p161, 136, p.p161, 90, 1.0, 1.0);s.store_powf_ad(136, A::div(s.ad_value(243), s.ad_value(85)), p.p18);s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[448]) && s.b[457]) {s.store_mul(86, 243, 90);s.store_sub(39, 234, 86);s.copy_ad(130, 39);s.store_scaled_add_mixed_ia(131, 130, A::sqrt_square_offset(s.ad_value(130), ((4.0 * 0.3) * 0.3)), 0.5);s.copy_ad(154, 131);s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);let t2: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, t2, (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(t2, ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);}
        s.b[461] = (s.v[136] < 200.0);s.store_scalar(461, if s.b[461] { 1.0 } else { 0.0 });
        if (((!s.b[448]) && s.b[457]) && s.b[461]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (((!s.b[448]) && s.b[457]) && (!s.b[461])) {s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((!s.b[448]) && s.b[457]) {s.store_sub_div_rhs_indices(100, 130, 156, 99);}
        s.b[462] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);s.store_scalar(462, if s.b[462] { 1.0 } else { 0.0 });
        if (((!s.b[448]) && s.b[457]) && s.b[462]) {s.store_sub(101, 130, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p.p169);s.store_scaled_mul(103, 136, 90, p.p170);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
        if (((!s.b[448]) && s.b[457]) && s.b[462]) {
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
        if (((!s.b[448]) && s.b[457]) && s.b[462]) {s.store_scaled_mul(107, 136, 91, p.p169);s.store_scaled_mul(108, 136, 91, p.p170);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[448]) && s.b[457]) && s.b[462]) {s.store_sub(115, 130, 114);s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_mul_scaled_powf_rhs(116, 136, p.p169, 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p.p170, 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
        if (((!s.b[448]) && s.b[457]) && s.b[462]) {
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
        if (((!s.b[448]) && s.b[457]) && s.b[462]) {s.store_mul_scaled_powf_rhs(121, 136, p.p169, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(122, 136, p.p170, 115, (-0.3333333333333333));s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.store_add(237, 128, 86);}
        if (((!s.b[448]) && s.b[457]) && (!s.b[462])) {s.store_add(237, 100, 86);}
        if ((!s.b[448]) && s.b[457]) {s.store_scaled_add(238, 236, 237, 0.5);s.store_sub(239, 237, 236);s.store_sub(90, 237, 236);s.store_add_scaled_inputs3_indices(91, 234, 1.0, 83, 1.0, 238, -1.0);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(137, 235, 234, ((p.p4 * p.p5) * p.p161), 238, (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);s.store_scale(188, 137, (1.0 / (p.p236) * 1e26));s.store_offset_powf_ad(189, s.ad_value(188), p.p235, 1.0);s.store_div_from_scalar(190, p.p234, 189);s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p160);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(240, 191, 234, ((p.p4 * p.p5) * p.p161), 238, (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);s.store_add_scaled_inputs3_indices(136, 234, 1.0, 83, 1.0, 238, -1.0);s.store_add_scaled_inputs(90, 236, 0.3333333333333333, 237, (2.0 * 0.3333333333333333));s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(239)), (1.0 / 12.0), 136, 1.0);s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(239)), 239, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);s.store_mul_add_scaled_inputs4_indices_rhs(241, 191, 234, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p161) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p161) * p.p5) * 0.5)));}
        if ((!s.b[448]) && (!s.b[457])) {s.store_scalar(240, 0.0);s.store_scalar(241, 0.0);}
        s.b[463] = (p.p149 == 0.0);s.store_scalar(463, if s.b[463] { 1.0 } else { 0.0 });s.b[464] = (p.p152 != 0.0);s.store_scalar(464, if s.b[464] { 1.0 } else { 0.0 });
        if (s.b[463] && s.b[464]) {s.store_voltage(57, ctx, nodes, Some(16), Some(15));}
        s.b[465] = (p.p152 == 1.0);s.store_scalar(465, if s.b[465] { 1.0 } else { 0.0 });
        if ((s.b[463] && s.b[464]) && s.b[465]) {s.store_voltage(58, ctx, nodes, Some(9), Some(15));s.store_voltage(59, ctx, nodes, Some(9), Some(16));}
        if ((s.b[463] && s.b[464]) && (!s.b[465])) {s.store_voltage(58, ctx, nodes, Some(2), Some(15));s.store_voltage(59, ctx, nodes, Some(2), Some(16));}
        if (s.b[463] && s.b[464]) {s.store_scalar(56, 1.0);}
        s.b[466] = (s.v[57] < 0.0);s.store_scalar(466, if s.b[466] { 1.0 } else { 0.0 });
        if ((s.b[463] && s.b[464]) && s.b[466]) {s.store_scalar(56, (-1.0));s.store_mul(255, 56, 57);s.copy_ad(254, 59);}
        if ((s.b[463] && s.b[464]) && (!s.b[466])) {s.copy_ad(255, 57);s.copy_ad(254, 58);}
        if (s.b[463] && s.b[464]) {s.store_offset_sqrt_ad(256, A::offset(A::square(s.ad_value(255)), 0.01), (-0.1));s.store_offset_scaled(146, 256, p.p179, (1.0 + p.p178));s.store_scaled_mul(83, 82, 146, 8.617087e-5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[463] && s.b[464]) {s.store_sub_ad(88, A::sub_from_scalar(p.p172, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p175)), (((-1.0)) * (p.p175)))), A::div_scaled_inputs(s.ad_value(256), (p.p181 * p.p180), A::sqrt_square_offset(s.ad_value(256), (p.p181 * p.p181)), 1.0));s.store_scalar(247, (p.p9 / p.p173));s.store_div_scalar_by_product_indices(136, p.p174, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));s.store_add_scaled_product_mixed_iia(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p171), 1.0);s.store_add_scaled_inputs4_mixed_iiai(160, 254, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(254), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);s.store_sub(246, 160, 88);s.store_div_scaled_inputs_indices(84, 247, 1.0, 83, (1.602176634e-19 * 3.24e17));s.store_div_from_scalar(150, 2.718281828459045, 84);s.store_div_from_scalar(151, 1.0, 84);s.store_primal_scale(99, 247, 6.241509074460763e18);s.store_scaled_add_mixed_ia(154, 246, A::sqrt_square_offset(s.ad_value(246), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);let t3: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, t3, (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(t3, ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 246, 1.0, 83, 2.0);}
        s.b[467] = (s.v[136] < 200.0);s.store_scalar(467, if s.b[467] { 1.0 } else { 0.0 });
        if ((s.b[463] && s.b[464]) && s.b[467]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(246), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((s.b[463] && s.b[464]) && (!s.b[467])) {s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(246), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (s.b[463] && s.b[464]) {s.store_sub_div_rhs_indices(100, 246, 153, 99);}
        s.b[468] = ((((s.v[100] - s.v[246])) as f64).abs() > 1e-19);s.store_scalar(468, if s.b[468] { 1.0 } else { 0.0 });
        if ((s.b[463] && s.b[464]) && s.b[468]) {s.store_sub(101, 246, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p.p182);s.store_scaled_mul(103, 136, 90, p.p183);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[463] && s.b[464]) && s.b[468]) {
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
        if ((s.b[463] && s.b[464]) && s.b[468]) {s.store_scaled_mul(107, 136, 91, p.p182);s.store_scaled_mul(108, 136, 91, p.p183);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 246, 114);s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(137, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(116, 136, p.p182, 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p.p183, 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
        if ((s.b[463] && s.b[464]) && s.b[468]) {
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
        if ((s.b[463] && s.b[464]) && s.b[468]) {s.store_scaled_mul(121, 136, 137, p.p182);s.store_scaled_mul(122, 136, 137, p.p183);s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.copy_ad(248, 128);}
        if ((s.b[463] && s.b[464]) && (!s.b[468])) {s.copy_ad(248, 100);}
        if (s.b[463] && s.b[464]) {s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p176);s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p177);s.store_mul_scaled_abs_ad_rhs(136, 247, 1.0 / (p.p9), A::sub(s.ad_value(246), s.ad_value(248)));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(248)), (s.v[81] / p.p9));s.store_div_mixed_ia(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);s.store_scaled_add_mixed_ia(90, 246, A::sqrt_square_offset(s.ad_value(246), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p174, 136, p.p174, 90, 1.0, 1.0);s.store_powf_ad(136, A::div(s.ad_value(255), s.ad_value(85)), p.p18);s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));s.store_mul(86, 255, 90);}
    }
}
