#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_64(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[523] && s.b[524]) {s.store_sub(39, 294, 86);s.copy_ad(130, 39);s.store_scaled_add_mixed_ia(131, 130, A::sqrt_square_offset(s.ad_value(130), ((4.0 * 0.3) * 0.3)), 0.5);s.copy_ad(154, 131);s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);let t0: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, t0, (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(t0, ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);}
        s.b[529] = (s.v[136] < 200.0);s.store_scalar(529, if s.b[529] { 1.0 } else { 0.0 });
        if ((s.b[523] && s.b[524]) && s.b[529]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((s.b[523] && s.b[524]) && (!s.b[529])) {s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (s.b[523] && s.b[524]) {s.store_sub_div_rhs_indices(100, 130, 156, 99);}
        s.b[530] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);s.store_scalar(530, if s.b[530] { 1.0 } else { 0.0 });
        if ((s.b[523] && s.b[524]) && s.b[530]) {s.store_sub(101, 130, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p.p208);s.store_scaled_mul(103, 136, 90, p.p209);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
        if ((s.b[523] && s.b[524]) && s.b[530]) {
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
        if ((s.b[523] && s.b[524]) && s.b[530]) {s.store_scaled_mul(107, 136, 91, p.p208);s.store_scaled_mul(108, 136, 91, p.p209);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 130, 114);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_65(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[523] && s.b[524]) && s.b[530]) {s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_mul_scaled_powf_rhs(116, 136, p.p208, 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p.p209, 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
        if ((s.b[523] && s.b[524]) && s.b[530]) {
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
        if ((s.b[523] && s.b[524]) && s.b[530]) {s.store_mul_scaled_powf_rhs(121, 136, p.p208, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(122, 136, p.p209, 115, (-0.3333333333333333));s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.store_add(297, 128, 86);}
        if ((s.b[523] && s.b[524]) && (!s.b[530])) {s.store_add(297, 100, 86);}
        if (s.b[523] && s.b[524]) {s.store_scaled_add(298, 296, 297, 0.5);s.store_sub(299, 297, 296);s.store_mul_add_scaled_inputs3_offset_rhs_indices(135, 299, 294, 1.0, 298, (-1.0), 83, 1.0, 0.0);s.store_mul_scaled_abs_ad_rhs(136, 295, 1.0 / (p.p9), A::sub(s.ad_value(294), s.ad_value(298)));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);s.store_scaled_mul(96, 95, 295, (p.p4 * (p.p5 * 1.0 / (p.p200))));s.store_mul_scale_offset_mixed_ia(98, 96, A::sub_scaled_inputs(s.ad_value(304), p.p21, s.ad_value(86), p.p21), 1.0, 1.0);s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(299), (p.p25 * p.p25), s.ad_value(299)), 1.0);s.store_div(93, 98, 92);s.store_mul(305, 93, 135);s.store_sub(90, 297, 296);s.store_add_scaled_inputs3_indices(91, 294, 1.0, 83, 1.0, 298, -1.0);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(137, 295, 294, ((p.p4 * p.p5) * p.p200), 298, (((-1.0)) * (((p.p4 * p.p5) * p.p200))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p200), 0.0);s.store_scale(188, 137, (1.0 / (p.p245) * 1e26));s.store_offset_powf_ad(189, s.ad_value(188), p.p244, 1.0);s.store_div_from_scalar(190, p.p243, 189);s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p199);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(300, 191, 294, ((p.p4 * p.p5) * p.p200), 298, (((-1.0)) * (((p.p4 * p.p5) * p.p200))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p200), 0.0);s.store_add_scaled_inputs3_indices(136, 294, 1.0, 83, 1.0, 298, -1.0);s.store_add_scaled_inputs(90, 296, 0.3333333333333333, 297, (2.0 * 0.3333333333333333));s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(299)), (1.0 / 12.0), 136, 1.0);s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(299)), 299, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);s.store_mul_add_scaled_inputs4_indices_rhs(301, 191, 294, (-(((p.p4 * p.p200) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p200) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p200) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p200) * p.p5) * 0.5)));}
        s.b[531] = (s.v[72] < 0.0);s.store_scalar(531, if s.b[531] { 1.0 } else { 0.0 });
        if ((s.b[523] && s.b[524]) && s.b[531]) {s.store_sub_scaled_inputs(301, 300, (-1.0), 301, 1.0);}
        if (s.b[523] && (!s.b[524])) {s.store_scalar(300, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_66(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[523] && (!s.b[524])) {s.store_scalar(301, 0.0);}
        s.b[532] = (p.p156 != 0.0);s.store_scalar(532, if s.b[532] { 1.0 } else { 0.0 });s.b[533] = (p.p156 == 1.0);s.store_scalar(533, if s.b[533] { 1.0 } else { 0.0 });
        if (((!s.b[523]) && s.b[532]) && s.b[533]) {s.store_voltage(74, ctx, nodes, Some(9), Some(7));}
        if (((!s.b[523]) && s.b[532]) && (!s.b[533])) {s.store_voltage(74, ctx, nodes, Some(2), Some(7));}
        if ((!s.b[523]) && s.b[532]) {s.copy_ad(302, 74);s.store_scalar(146, (1.0 + p.p204));s.store_scaled_mul(83, 82, 146, 8.617087e-5);s.store_sub_from_scalar_ad(88, p.p198, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p201)), (((-1.0)) * (p.p201))));s.store_scalar(295, (p.p9 / p.p199));s.store_div_scalar_by_product_indices(136, p.p200, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));s.store_add_scaled_product_mixed_iia(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p197), 1.0);s.store_add_scaled_inputs4_mixed_iiai(160, 302, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(302), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);s.store_sub(294, 160, 88);s.store_div_scaled_inputs_indices(84, 295, 1.0, 83, (1.602176634e-19 * 3.24e17));s.store_div_from_scalar(150, 2.718281828459045, 84);s.store_div_from_scalar(151, 1.0, 84);s.store_primal_scale(99, 295, 6.241509074460763e18);s.store_scaled_add_mixed_ia(154, 294, A::sqrt_square_offset(s.ad_value(294), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);let t1: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, t1, (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(t1, ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 294, 1.0, 83, 2.0);}
        s.b[534] = (s.v[136] < 200.0);s.store_scalar(534, if s.b[534] { 1.0 } else { 0.0 });
        if (((!s.b[523]) && s.b[532]) && s.b[534]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(294), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (((!s.b[523]) && s.b[532]) && (!s.b[534])) {s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(294), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((!s.b[523]) && s.b[532]) {s.store_sub_div_rhs_indices(100, 294, 153, 99);}
        s.b[535] = ((((s.v[100] - s.v[294])) as f64).abs() > 1e-19);s.store_scalar(535, if s.b[535] { 1.0 } else { 0.0 });
        if (((!s.b[523]) && s.b[532]) && s.b[535]) {s.store_sub(101, 294, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p.p208);s.store_scaled_mul(103, 136, 90, p.p209);s.store_sub_div_same_denominator(104, 100, 102, 83);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_67(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[523]) && s.b[532]) && s.b[535]) {s.store_sub_div_same_denominator(105, 100, 103, 83);}
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
        if (((!s.b[523]) && s.b[532]) && s.b[535]) {s.store_scaled_mul(107, 136, 91, p.p208);s.store_scaled_mul(108, 136, 91, p.p209);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 294, 114);s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(137, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(116, 136, p.p208, 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p.p209, 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
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
        if (((!s.b[523]) && s.b[532]) && s.b[535]) {s.store_scaled_mul(121, 136, 137, p.p208);s.store_scaled_mul(122, 136, 137, p.p209);s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.copy_ad(296, 128);}
        if (((!s.b[523]) && s.b[532]) && (!s.b[535])) {s.copy_ad(296, 100);}
        if ((!s.b[523]) && s.b[532]) {s.store_scalar(303, 0.0);s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p202);s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p203);s.store_mul_scaled_abs_ad_rhs(136, 295, 1.0 / (p.p9), A::sub(s.ad_value(294), s.ad_value(296)));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(296)), (s.v[81] / p.p9));s.store_div_mixed_ia(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);s.store_scaled_add_mixed_ia(90, 294, A::sqrt_square_offset(s.ad_value(294), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p200, 136, p.p200, 90, 1.0, 1.0);s.store_powf_ad(136, A::div(s.ad_value(303), s.ad_value(85)), p.p18);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_68(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[523]) && s.b[532]) {s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));s.store_mul(86, 303, 90);s.store_sub(39, 294, 86);s.copy_ad(130, 39);s.store_scaled_add_mixed_ia(131, 130, A::sqrt_square_offset(s.ad_value(130), ((4.0 * 0.3) * 0.3)), 0.5);s.copy_ad(154, 131);s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);let t2: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, t2, (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(t2, ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);}
        s.b[536] = (s.v[136] < 200.0);s.store_scalar(536, if s.b[536] { 1.0 } else { 0.0 });
        if (((!s.b[523]) && s.b[532]) && s.b[536]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (((!s.b[523]) && s.b[532]) && (!s.b[536])) {s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((!s.b[523]) && s.b[532]) {s.store_sub_div_rhs_indices(100, 130, 156, 99);}
        s.b[537] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);s.store_scalar(537, if s.b[537] { 1.0 } else { 0.0 });
        if (((!s.b[523]) && s.b[532]) && s.b[537]) {s.store_sub(101, 130, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p.p208);s.store_scaled_mul(103, 136, 90, p.p209);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
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
        if (((!s.b[523]) && s.b[532]) && s.b[537]) {s.store_scaled_mul(107, 136, 91, p.p208);s.store_scaled_mul(108, 136, 91, p.p209);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_69(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[523]) && s.b[532]) && s.b[537]) {s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 130, 114);s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_mul_scaled_powf_rhs(116, 136, p.p208, 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p.p209, 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
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
        if (((!s.b[523]) && s.b[532]) && s.b[537]) {s.store_mul_scaled_powf_rhs(121, 136, p.p208, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(122, 136, p.p209, 115, (-0.3333333333333333));s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.store_add(297, 128, 86);}
        if (((!s.b[523]) && s.b[532]) && (!s.b[537])) {s.store_add(297, 100, 86);}
        if ((!s.b[523]) && s.b[532]) {s.store_scaled_add(298, 296, 297, 0.5);s.store_sub(299, 297, 296);s.store_sub(90, 297, 296);s.store_add_scaled_inputs3_indices(91, 294, 1.0, 83, 1.0, 298, -1.0);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(137, 295, 294, ((p.p4 * p.p5) * p.p200), 298, (((-1.0)) * (((p.p4 * p.p5) * p.p200))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p200), 0.0);s.store_scale(188, 137, (1.0 / (p.p245) * 1e26));s.store_offset_powf_ad(189, s.ad_value(188), p.p244, 1.0);s.store_div_from_scalar(190, p.p243, 189);s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p199);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(300, 191, 294, ((p.p4 * p.p5) * p.p200), 298, (((-1.0)) * (((p.p4 * p.p5) * p.p200))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p200), 0.0);s.store_add_scaled_inputs3_indices(136, 294, 1.0, 83, 1.0, 298, -1.0);s.store_add_scaled_inputs(90, 296, 0.3333333333333333, 297, (2.0 * 0.3333333333333333));s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(299)), (1.0 / 12.0), 136, 1.0);s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(299)), 299, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);s.store_mul_add_scaled_inputs4_indices_rhs(301, 191, 294, (-(((p.p4 * p.p200) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p200) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p200) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p200) * p.p5) * 0.5)));}
        if ((!s.b[523]) && (!s.b[532])) {s.store_scalar(300, 0.0);s.store_scalar(301, 0.0);}
        s.b[538] = (p.p149 == 0.0);s.store_scalar(538, if s.b[538] { 1.0 } else { 0.0 });s.b[539] = (p.p157 != 0.0);s.store_scalar(539, if s.b[539] { 1.0 } else { 0.0 });
        if (s.b[538] && s.b[539]) {s.store_voltage(77, ctx, nodes, Some(21), Some(22));}
        s.b[540] = (p.p157 == 1.0);s.store_scalar(540, if s.b[540] { 1.0 } else { 0.0 });
        if ((s.b[538] && s.b[539]) && s.b[540]) {s.store_voltage(78, ctx, nodes, Some(9), Some(22));s.store_voltage(79, ctx, nodes, Some(9), Some(21));}
        if ((s.b[538] && s.b[539]) && (!s.b[540])) {s.store_voltage(78, ctx, nodes, Some(2), Some(22));s.store_voltage(79, ctx, nodes, Some(2), Some(21));}
        if (s.b[538] && s.b[539]) {s.store_scalar(76, 1.0);}
        s.b[541] = (s.v[77] < 0.0);s.store_scalar(541, if s.b[541] { 1.0 } else { 0.0 });
        if ((s.b[538] && s.b[539]) && s.b[541]) {s.store_scalar(76, (-1.0));s.store_mul(315, 76, 77);s.copy_ad(314, 79);}
        if ((s.b[538] && s.b[539]) && (!s.b[541])) {s.copy_ad(315, 77);s.copy_ad(314, 78);}
        if (s.b[538] && s.b[539]) {s.store_offset_sqrt_ad(316, A::offset(A::square(s.ad_value(315)), 0.01), (-0.1));s.store_offset_scaled(146, 316, p.p205, (1.0 + p.p204));s.store_scaled_mul(83, 82, 146, 8.617087e-5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_70(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[538] && s.b[539]) {s.store_sub_ad(88, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p201)), (((((-1.0)) * (p.p201))) + (p.p198))), A::div_scaled_inputs(s.ad_value(316), (p.p207 * p.p206), A::sqrt_square_offset(s.ad_value(316), (p.p207 * p.p207)), 1.0));s.store_scalar(307, (p.p9 / p.p199));s.store_div_scalar_by_product_indices(136, p.p200, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));s.store_add_scaled_product_mixed_iia(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p197), 1.0);s.store_add_scaled_inputs4_mixed_iiai(160, 314, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(314), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);s.store_sub(306, 160, 88);s.store_div_scaled_inputs_indices(84, 307, 1.0, 83, (1.602176634e-19 * 3.24e17));s.store_div_from_scalar(150, 2.718281828459045, 84);s.store_div_from_scalar(151, 1.0, 84);s.store_primal_scale(99, 307, 6.241509074460763e18);s.store_scaled_add_mixed_ia(154, 306, A::sqrt_square_offset(s.ad_value(306), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);let t3: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, t3, (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(t3, ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 306, 1.0, 83, 2.0);}
        s.b[542] = (s.v[136] < 200.0);s.store_scalar(542, if s.b[542] { 1.0 } else { 0.0 });
        if ((s.b[538] && s.b[539]) && s.b[542]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((s.b[538] && s.b[539]) && (!s.b[542])) {s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (s.b[538] && s.b[539]) {s.store_sub_div_rhs_indices(100, 306, 153, 99);}
        s.b[543] = ((((s.v[100] - s.v[306])) as f64).abs() > 1e-19);s.store_scalar(543, if s.b[543] { 1.0 } else { 0.0 });
        if ((s.b[538] && s.b[539]) && s.b[543]) {s.store_sub(101, 306, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p.p208);s.store_scaled_mul(103, 136, 90, p.p209);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_71(
        s: &mut Scratch,
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
        if ((s.b[538] && s.b[539]) && s.b[543]) {s.store_scaled_mul(107, 136, 91, p.p208);s.store_scaled_mul(108, 136, 91, p.p209);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 306, 114);s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(137, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(116, 136, p.p208, 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p.p209, 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
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
        if ((s.b[538] && s.b[539]) && s.b[543]) {s.store_scaled_mul(121, 136, 137, p.p208);s.store_scaled_mul(122, 136, 137, p.p209);s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.copy_ad(308, 128);}
        if ((s.b[538] && s.b[539]) && (!s.b[543])) {s.copy_ad(308, 100);}
        if (s.b[538] && s.b[539]) {s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p202);s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p203);s.store_mul_scaled_abs_ad_rhs(136, 307, 1.0 / (p.p9), A::sub(s.ad_value(306), s.ad_value(308)));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(308)), (s.v[81] / p.p9));s.store_div_mixed_ia(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);s.store_scaled_add_mixed_ia(90, 306, A::sqrt_square_offset(s.ad_value(306), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p200, 136, p.p200, 90, 1.0, 1.0);s.store_powf_ad(136, A::div(s.ad_value(315), s.ad_value(85)), p.p18);s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));s.store_mul(86, 315, 90);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_72(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[538] && s.b[539]) {s.store_sub(39, 306, 86);s.copy_ad(130, 39);s.store_scaled_add_mixed_ia(131, 130, A::sqrt_square_offset(s.ad_value(130), ((4.0 * 0.3) * 0.3)), 0.5);s.copy_ad(154, 131);s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);let t4: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, t4, (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(t4, ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);}
        s.b[544] = (s.v[136] < 200.0);s.store_scalar(544, if s.b[544] { 1.0 } else { 0.0 });
        if ((s.b[538] && s.b[539]) && s.b[544]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((s.b[538] && s.b[539]) && (!s.b[544])) {s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (s.b[538] && s.b[539]) {s.store_sub_div_rhs_indices(100, 130, 156, 99);}
        s.b[545] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);s.store_scalar(545, if s.b[545] { 1.0 } else { 0.0 });
        if ((s.b[538] && s.b[539]) && s.b[545]) {s.store_sub(101, 130, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p.p208);s.store_scaled_mul(103, 136, 90, p.p209);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
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
        if ((s.b[538] && s.b[539]) && s.b[545]) {s.store_scaled_mul(107, 136, 91, p.p208);s.store_scaled_mul(108, 136, 91, p.p209);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 130, 114);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_73(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[538] && s.b[539]) && s.b[545]) {s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_mul_scaled_powf_rhs(116, 136, p.p208, 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p.p209, 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
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
        if ((s.b[538] && s.b[539]) && s.b[545]) {s.store_mul_scaled_powf_rhs(121, 136, p.p208, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(122, 136, p.p209, 115, (-0.3333333333333333));s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.store_add(309, 128, 86);}
        if ((s.b[538] && s.b[539]) && (!s.b[545])) {s.store_add(309, 100, 86);}
        if (s.b[538] && s.b[539]) {s.store_scaled_add(310, 308, 309, 0.5);s.store_sub(311, 309, 308);s.store_mul_add_scaled_inputs3_offset_rhs_indices(135, 311, 306, 1.0, 310, (-1.0), 83, 1.0, 0.0);s.store_mul_scaled_abs_ad_rhs(136, 307, 1.0 / (p.p9), A::sub(s.ad_value(306), s.ad_value(310)));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);s.store_scaled_mul(96, 95, 307, (p.p4 * (p.p5 * 1.0 / (p.p200))));s.store_mul_scale_offset_mixed_ia(98, 96, A::sub_scaled_inputs(s.ad_value(316), p.p21, s.ad_value(86), p.p21), 1.0, 1.0);s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(311), (p.p25 * p.p25), s.ad_value(311)), 1.0);s.store_div(93, 98, 92);s.store_mul(317, 93, 135);s.store_sub(90, 309, 308);s.store_add_scaled_inputs3_indices(91, 306, 1.0, 83, 1.0, 310, -1.0);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(137, 307, 306, ((p.p4 * p.p5) * p.p200), 310, (((-1.0)) * (((p.p4 * p.p5) * p.p200))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p200), 0.0);s.store_scale(188, 137, (1.0 / (p.p245) * 1e26));s.store_offset_powf_ad(189, s.ad_value(188), p.p244, 1.0);s.store_div_from_scalar(190, p.p243, 189);s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p199);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(312, 191, 306, ((p.p4 * p.p5) * p.p200), 310, (((-1.0)) * (((p.p4 * p.p5) * p.p200))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p200), 0.0);s.store_add_scaled_inputs3_indices(136, 306, 1.0, 83, 1.0, 310, -1.0);s.store_add_scaled_inputs(90, 308, 0.3333333333333333, 309, (2.0 * 0.3333333333333333));s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(311)), (1.0 / 12.0), 136, 1.0);s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(311)), 311, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);s.store_mul_add_scaled_inputs4_indices_rhs(313, 191, 306, (-(((p.p4 * p.p200) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p200) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p200) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p200) * p.p5) * 0.5)));}
        s.b[546] = (s.v[76] < 0.0);s.store_scalar(546, if s.b[546] { 1.0 } else { 0.0 });
        if ((s.b[538] && s.b[539]) && s.b[546]) {s.store_sub_scaled_inputs(313, 312, (-1.0), 313, 1.0);}
        if (s.b[538] && (!s.b[539])) {s.store_scalar(312, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_74(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[538] && (!s.b[539])) {s.store_scalar(313, 0.0);}
        s.b[547] = (p.p157 != 0.0);s.store_scalar(547, if s.b[547] { 1.0 } else { 0.0 });s.b[548] = (p.p157 == 1.0);s.store_scalar(548, if s.b[548] { 1.0 } else { 0.0 });
        if (((!s.b[538]) && s.b[547]) && s.b[548]) {s.store_voltage(78, ctx, nodes, Some(9), Some(8));}
        if (((!s.b[538]) && s.b[547]) && (!s.b[548])) {s.store_voltage(78, ctx, nodes, Some(2), Some(8));}
        if ((!s.b[538]) && s.b[547]) {s.copy_ad(314, 78);s.store_scalar(146, (1.0 + p.p204));s.store_scaled_mul(83, 82, 146, 8.617087e-5);s.store_offset_scaled(88, 82, ((1.0 / (s.v[35])) * (p.p201)), (((((-1.0)) * (p.p201))) + (p.p198)));s.store_scalar(307, (p.p9 / p.p199));s.store_div_scalar_by_product_indices(136, p.p200, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));s.store_add_scaled_product_mixed_iia(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p197), 1.0);s.store_add_scaled_inputs4_mixed_iiai(160, 314, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(314), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);s.store_sub(306, 160, 88);s.store_div_scaled_inputs_indices(84, 307, 1.0, 83, (1.602176634e-19 * 3.24e17));s.store_div_from_scalar(150, 2.718281828459045, 84);s.store_div_from_scalar(151, 1.0, 84);s.store_primal_scale(99, 307, 6.241509074460763e18);s.store_scaled_add_mixed_ia(154, 306, A::sqrt_square_offset(s.ad_value(306), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);let t5: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, t5, (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(t5, ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 306, 1.0, 83, 2.0);}
        s.b[549] = (s.v[136] < 200.0);s.store_scalar(549, if s.b[549] { 1.0 } else { 0.0 });
        if (((!s.b[538]) && s.b[547]) && s.b[549]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (((!s.b[538]) && s.b[547]) && (!s.b[549])) {s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((!s.b[538]) && s.b[547]) {s.store_sub_div_rhs_indices(100, 306, 153, 99);}
        s.b[550] = ((((s.v[100] - s.v[306])) as f64).abs() > 1e-19);s.store_scalar(550, if s.b[550] { 1.0 } else { 0.0 });
        if (((!s.b[538]) && s.b[547]) && s.b[550]) {s.store_sub(101, 306, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p.p208);s.store_scaled_mul(103, 136, 90, p.p209);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_75(
        s: &mut Scratch,
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
        if (((!s.b[538]) && s.b[547]) && s.b[550]) {s.store_scaled_mul(107, 136, 91, p.p208);s.store_scaled_mul(108, 136, 91, p.p209);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 306, 114);s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(137, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(116, 136, p.p208, 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p.p209, 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
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
        if (((!s.b[538]) && s.b[547]) && s.b[550]) {s.store_scaled_mul(121, 136, 137, p.p208);s.store_scaled_mul(122, 136, 137, p.p209);s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.copy_ad(308, 128);}
        if (((!s.b[538]) && s.b[547]) && (!s.b[550])) {s.copy_ad(308, 100);}
        if ((!s.b[538]) && s.b[547]) {s.store_scalar(315, 0.0);s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p202);s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p203);s.store_mul_scaled_abs_ad_rhs(136, 307, 1.0 / (p.p9), A::sub(s.ad_value(306), s.ad_value(308)));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(308)), (s.v[81] / p.p9));s.store_div_mixed_ia(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);s.store_scaled_add_mixed_ia(90, 306, A::sqrt_square_offset(s.ad_value(306), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p200, 136, p.p200, 90, 1.0, 1.0);s.store_powf_ad(136, A::div(s.ad_value(315), s.ad_value(85)), p.p18);s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_76(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[538]) && s.b[547]) {s.store_mul(86, 315, 90);s.store_sub(39, 306, 86);s.copy_ad(130, 39);s.store_scaled_add_mixed_ia(131, 130, A::sqrt_square_offset(s.ad_value(130), ((4.0 * 0.3) * 0.3)), 0.5);s.copy_ad(154, 131);s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);let t6: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, t6, (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(t6, ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);}
        s.b[551] = (s.v[136] < 200.0);s.store_scalar(551, if s.b[551] { 1.0 } else { 0.0 });
        if (((!s.b[538]) && s.b[547]) && s.b[551]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (((!s.b[538]) && s.b[547]) && (!s.b[551])) {s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((!s.b[538]) && s.b[547]) {s.store_sub_div_rhs_indices(100, 130, 156, 99);}
        s.b[552] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);s.store_scalar(552, if s.b[552] { 1.0 } else { 0.0 });
        if (((!s.b[538]) && s.b[547]) && s.b[552]) {s.store_sub(101, 130, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p.p208);s.store_scaled_mul(103, 136, 90, p.p209);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
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
        if (((!s.b[538]) && s.b[547]) && s.b[552]) {s.store_scaled_mul(107, 136, 91, p.p208);s.store_scaled_mul(108, 136, 91, p.p209);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_77(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[538]) && s.b[547]) && s.b[552]) {s.store_sub(115, 130, 114);s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_mul_scaled_powf_rhs(116, 136, p.p208, 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p.p209, 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
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
        if (((!s.b[538]) && s.b[547]) && s.b[552]) {s.store_mul_scaled_powf_rhs(121, 136, p.p208, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(122, 136, p.p209, 115, (-0.3333333333333333));s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.store_add(309, 128, 86);}
        if (((!s.b[538]) && s.b[547]) && (!s.b[552])) {s.store_add(309, 100, 86);}
        if ((!s.b[538]) && s.b[547]) {s.store_scaled_add(310, 308, 309, 0.5);s.store_sub(311, 309, 308);s.store_sub(90, 309, 308);s.store_add_scaled_inputs3_indices(91, 306, 1.0, 83, 1.0, 310, -1.0);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(137, 307, 306, ((p.p4 * p.p5) * p.p200), 310, (((-1.0)) * (((p.p4 * p.p5) * p.p200))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p200), 0.0);s.store_scale(188, 137, (1.0 / (p.p245) * 1e26));s.store_offset_powf_ad(189, s.ad_value(188), p.p244, 1.0);s.store_div_from_scalar(190, p.p243, 189);s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p199);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(312, 191, 306, ((p.p4 * p.p5) * p.p200), 310, (((-1.0)) * (((p.p4 * p.p5) * p.p200))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p200), 0.0);s.store_add_scaled_inputs3_indices(136, 306, 1.0, 83, 1.0, 310, -1.0);s.store_add_scaled_inputs(90, 308, 0.3333333333333333, 309, (2.0 * 0.3333333333333333));s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(311)), (1.0 / 12.0), 136, 1.0);s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(311)), 311, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);s.store_mul_add_scaled_inputs4_indices_rhs(313, 191, 306, (-(((p.p4 * p.p200) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p200) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p200) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p200) * p.p5) * 0.5)));}
        if ((!s.b[538]) && (!s.b[547])) {s.store_scalar(312, 0.0);s.store_scalar(313, 0.0);}
        s.b[553] = (p.p255 == 1.0);s.store_scalar(553, if s.b[553] { 1.0 } else { 0.0 });
        if s.b[553] {s.store_scalar(318, ((p.p258 * (p.p256 + ((p.p4 / 3.0) / p.p257))) / ((p.p257 * p.p5) * p.p3)));}
        s.b[554] = (s.v[318] > 0.0);s.store_scalar(554, if s.b[554] { 1.0 } else { 0.0 });
        if (s.b[553] && s.b[554]) {s.store_primal_div_from_scalar(318, 1.0, 318);}
        if (s.b[553] && (!s.b[554])) {s.store_scalar(318, (1.0 / 0.001));}
        s.b[555] = (p.p255 == 2.0);s.store_scalar(555, if s.b[555] { 1.0 } else { 0.0 });
        if ((!s.b[553]) && s.b[555]) {s.store_scalar(319, ((p.p258 * (p.p256 + ((p.p4 / 3.0) / p.p257))) / ((p.p257 * p.p5) * p.p3)));s.store_scalar(320, ((p.p258 * (((2.0 * p.p4) / 3.0) / p.p257)) / ((p.p257 * p.p5) * p.p3)));}
        s.b[556] = (s.v[319] > 0.0);s.store_scalar(556, if s.b[556] { 1.0 } else { 0.0 });
        if (((!s.b[553]) && s.b[555]) && s.b[556]) {s.store_primal_div_from_scalar(319, 1.0, 319);}
        if (((!s.b[553]) && s.b[555]) && (!s.b[556])) {s.store_scalar(319, (1.0 / 0.001));}
        s.b[557] = (s.v[320] > 0.0);s.store_scalar(557, if s.b[557] { 1.0 } else { 0.0 });
        if (((!s.b[553]) && s.b[555]) && s.b[557]) {s.store_primal_div_from_scalar(320, 1.0, 320);}
        if (((!s.b[553]) && s.b[555]) && (!s.b[557])) {s.store_scalar(320, (1.0 / 0.001));}
        s.b[558] = (p.p255 == 2.0);s.store_scalar(558, if s.b[558] { 1.0 } else { 0.0 });
        if s.b[558] {s.store_scaled_voltage(162, ctx, nodes, Some(10), Some(2), ((p.p4 * p.p5) * p.p210));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_78(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[558] {s.store_div_scaled_inputs(168, A::voltage(ctx, nodes, Some(0), Some(2)), p.p214, A::sqrt(A::offset(A::square(A::voltage(ctx, nodes, Some(0), Some(2))), (p.p214 * p.p214))), 1.0);s.store_scalar(169, (p.p213).min((p.p211 / (2.0 * p.p214))));s.store_sub_from_scalar_scaled_mul(167, ((p.p4 * p.p5) * p.p211), 169, 168, (p.p4 * p.p5));s.store_mul_voltage_ad(163, A::max_with_scalar(s.ad_value(167), 0.0), ctx, nodes, Some(10), Some(0));}
        if (!s.b[558]) {s.store_scaled_voltage(162, ctx, nodes, Some(1), Some(2), ((p.p4 * p.p5) * p.p210));s.store_div_scaled_inputs(168, A::voltage(ctx, nodes, Some(0), Some(2)), p.p214, A::sqrt(A::offset(A::square(A::voltage(ctx, nodes, Some(0), Some(2))), (p.p214 * p.p214))), 1.0);s.store_scalar(169, (p.p213).min((p.p211 / (2.0 * p.p214))));s.store_sub_from_scalar_scaled_mul(167, ((p.p4 * p.p5) * p.p211), 169, 168, (p.p4 * p.p5));s.store_mul_voltage_ad(163, A::max_with_scalar(s.ad_value(167), 0.0), ctx, nodes, Some(1), Some(0));}
        s.store_scaled_voltage(164, ctx, nodes, Some(0), Some(2), ((p.p4 * p.p5) * p.p212));s.store_scaled_voltage(219, ctx, nodes, Some(3), Some(0), ((p.p4 * p.p5) * p.p215));s.store_scaled_voltage(220, ctx, nodes, Some(3), Some(2), ((p.p4 * p.p5) * p.p216));s.store_scaled_voltage(221, ctx, nodes, Some(3), Some(1), ((p.p4 * p.p5) * p.p217));s.store_offset_scaled(375, 82, ((1.0 / (s.v[35])) * (p.p285)), (((((-1.0)) * (p.p285))) + (p.p279)));s.store_offset_scaled(373, 82, ((1.0 / (s.v[35])) * (p.p283)), (((((-1.0)) * (p.p283))) + (p.p275)));s.store_scale_ad(377, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p281), p.p277);s.store_offset_scaled(376, 82, ((1.0 / (s.v[35])) * (p.p286)), (((((-1.0)) * (p.p286))) + (p.p280)));s.store_offset_scaled(374, 82, ((1.0 / (s.v[35])) * (p.p284)), (((((-1.0)) * (p.p284))) + (p.p276)));s.store_scale_ad(378, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p282), p.p278);s.store_scale(137, 378, (p.p4 * p.p5));s.store_max_with_scalar_ad(371, A::sub(A::voltage(ctx, nodes, Some(0), Some(3)), s.ad_value(376)), 0.0);s.b[559] = (s.v[137] > 0.0);s.store_scalar(559, if s.b[559] { 1.0 } else { 0.0 });s.b[560] = (s.v[371] > 0.0);s.store_scalar(560, if s.b[560] { 1.0 } else { 0.0 });
        if (s.b[559] && s.b[560]) {s.store_div_scaled_value_by_product_indices(354, 371, 1.0, 374, 36, 1.0);}
        s.b[561] = (s.v[354] > 80.0);s.store_scalar(561, if s.b[561] { 1.0 } else { 0.0 });
        if ((s.b[559] && s.b[560]) && s.b[561]) {s.store_offset(355, 354, (((-80.0)) + (1.0)));s.store_scalar(354, 80.0);}
        if ((s.b[559] && s.b[560]) && (!s.b[561])) {s.store_scalar(355, 1.0);}
        if (s.b[559] && s.b[560]) {s.store_mul_exp_rhs(355, 355, 354);s.store_mul_scale_offset_indices(369, 137, 355, 1.0, (-1.0));}
        if (s.b[559] && (!s.b[560])) {s.store_div_scaled_value_by_product_indices(354, 371, 1.0, 374, 36, 1.0);}
        s.b[562] = (s.v[354] > 80.0);s.store_scalar(562, if s.b[562] { 1.0 } else { 0.0 });
        if ((s.b[559] && (!s.b[560])) && s.b[562]) {s.store_offset(355, 354, (((-80.0)) + (1.0)));s.store_scalar(354, 80.0);}
        if ((s.b[559] && (!s.b[560])) && (!s.b[562])) {s.store_scalar(355, 1.0);}
        if (s.b[559] && (!s.b[560])) {s.store_mul_exp_rhs(355, 355, 354);s.store_mul_scale_offset_indices(369, 137, 355, 1.0, (-1.0));}
        if (!s.b[559]) {s.store_scalar(369, 0.0);}
        s.store_max_with_scalar_ad(372, A::sub(A::voltage(ctx, nodes, Some(2), Some(3)), s.ad_value(375)), 0.0);s.store_scale(137, 377, (p.p4 * p.p5));s.b[563] = (s.v[137] > 0.0);s.store_scalar(563, if s.b[563] { 1.0 } else { 0.0 });s.b[564] = (s.v[372] > 0.0);s.store_scalar(564, if s.b[564] { 1.0 } else { 0.0 });
        if (s.b[563] && s.b[564]) {s.store_div_scaled_value_by_product_indices(354, 372, 1.0, 373, 36, 1.0);}
        s.b[565] = (s.v[354] > 80.0);s.store_scalar(565, if s.b[565] { 1.0 } else { 0.0 });
        if ((s.b[563] && s.b[564]) && s.b[565]) {s.store_offset(355, 354, (((-80.0)) + (1.0)));s.store_scalar(354, 80.0);}
        if ((s.b[563] && s.b[564]) && (!s.b[565])) {s.store_scalar(355, 1.0);}
        if (s.b[563] && s.b[564]) {s.store_mul_exp_rhs(355, 355, 354);s.store_mul_scale_offset_indices(370, 137, 355, 1.0, (-1.0));}
        if (s.b[563] && (!s.b[564])) {s.store_div_scaled_value_by_product_indices(354, 372, 1.0, 373, 36, 1.0);}
        s.b[566] = (s.v[354] > 80.0);s.store_scalar(566, if s.b[566] { 1.0 } else { 0.0 });
        if ((s.b[563] && (!s.b[564])) && s.b[566]) {s.store_offset(355, 354, (((-80.0)) + (1.0)));s.store_scalar(354, 80.0);}
        if ((s.b[563] && (!s.b[564])) && (!s.b[566])) {s.store_scalar(355, 1.0);}
        if (s.b[563] && (!s.b[564])) {s.store_mul_exp_rhs(355, 355, 354);s.store_mul_scale_offset_indices(370, 137, 355, 1.0, (-1.0));}
        if (!s.b[563]) {s.store_scalar(370, 0.0);}
        s.b[567] = (p.p259 == 1.0);s.store_scalar(567, if s.b[567] { 1.0 } else { 0.0 });s.b[569] = (p.p255 == 2.0);s.store_scalar(569, if s.b[569] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_79(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[570] = (p.p149 == 0.0);s.store_scalar(570, if s.b[570] { 1.0 } else { 0.0 });s.b[571] = (p.p150 != 0.0);s.store_scalar(571, if s.b[571] { 1.0 } else { 0.0 });s.b[572] = (p.p150 == 1.0);s.store_scalar(572, if s.b[572] { 1.0 } else { 0.0 });s.b[573] = (p.p150 != 0.0);s.store_scalar(573, if s.b[573] { 1.0 } else { 0.0 });s.b[574] = (p.p150 == 1.0);s.store_scalar(574, if s.b[574] { 1.0 } else { 0.0 });s.b[575] = (p.p149 == 0.0);s.store_scalar(575, if s.b[575] { 1.0 } else { 0.0 });s.b[576] = (p.p151 != 0.0);s.store_scalar(576, if s.b[576] { 1.0 } else { 0.0 });s.b[577] = (p.p151 == 1.0);s.store_scalar(577, if s.b[577] { 1.0 } else { 0.0 });s.b[578] = (p.p151 != 0.0);s.store_scalar(578, if s.b[578] { 1.0 } else { 0.0 });s.b[579] = (p.p151 == 1.0);s.store_scalar(579, if s.b[579] { 1.0 } else { 0.0 });s.b[580] = (p.p149 == 0.0);s.store_scalar(580, if s.b[580] { 1.0 } else { 0.0 });s.b[581] = (p.p152 != 0.0);s.store_scalar(581, if s.b[581] { 1.0 } else { 0.0 });s.b[582] = (p.p152 == 1.0);s.store_scalar(582, if s.b[582] { 1.0 } else { 0.0 });s.b[583] = (p.p152 != 0.0);s.store_scalar(583, if s.b[583] { 1.0 } else { 0.0 });s.b[584] = (p.p152 == 1.0);s.store_scalar(584, if s.b[584] { 1.0 } else { 0.0 });s.b[585] = (p.p149 == 0.0);s.store_scalar(585, if s.b[585] { 1.0 } else { 0.0 });s.b[586] = (p.p153 != 0.0);s.store_scalar(586, if s.b[586] { 1.0 } else { 0.0 });s.b[587] = (p.p153 == 1.0);s.store_scalar(587, if s.b[587] { 1.0 } else { 0.0 });s.b[588] = (p.p153 != 0.0);s.store_scalar(588, if s.b[588] { 1.0 } else { 0.0 });s.b[589] = (p.p153 == 1.0);s.store_scalar(589, if s.b[589] { 1.0 } else { 0.0 });s.b[590] = (p.p149 == 0.0);s.store_scalar(590, if s.b[590] { 1.0 } else { 0.0 });s.b[591] = (p.p154 != 0.0);s.store_scalar(591, if s.b[591] { 1.0 } else { 0.0 });s.b[592] = (p.p154 == 1.0);s.store_scalar(592, if s.b[592] { 1.0 } else { 0.0 });s.b[593] = (p.p154 != 0.0);s.store_scalar(593, if s.b[593] { 1.0 } else { 0.0 });s.b[594] = (p.p154 == 1.0);s.store_scalar(594, if s.b[594] { 1.0 } else { 0.0 });s.b[595] = (p.p149 == 0.0);s.store_scalar(595, if s.b[595] { 1.0 } else { 0.0 });s.b[596] = (p.p155 != 0.0);s.store_scalar(596, if s.b[596] { 1.0 } else { 0.0 });s.b[597] = (p.p155 == 1.0);s.store_scalar(597, if s.b[597] { 1.0 } else { 0.0 });s.b[598] = (p.p155 != 0.0);s.store_scalar(598, if s.b[598] { 1.0 } else { 0.0 });s.b[599] = (p.p155 == 1.0);s.store_scalar(599, if s.b[599] { 1.0 } else { 0.0 });s.b[600] = (p.p149 == 0.0);s.store_scalar(600, if s.b[600] { 1.0 } else { 0.0 });s.b[601] = (p.p156 != 0.0);s.store_scalar(601, if s.b[601] { 1.0 } else { 0.0 });s.b[602] = (p.p156 == 1.0);s.store_scalar(602, if s.b[602] { 1.0 } else { 0.0 });s.b[603] = (p.p156 != 0.0);s.store_scalar(603, if s.b[603] { 1.0 } else { 0.0 });s.b[604] = (p.p156 == 1.0);s.store_scalar(604, if s.b[604] { 1.0 } else { 0.0 });s.b[605] = (p.p149 == 0.0);s.store_scalar(605, if s.b[605] { 1.0 } else { 0.0 });s.b[606] = (p.p157 != 0.0);s.store_scalar(606, if s.b[606] { 1.0 } else { 0.0 });s.b[607] = (p.p157 == 1.0);s.store_scalar(607, if s.b[607] { 1.0 } else { 0.0 });s.b[608] = (p.p157 != 0.0);s.store_scalar(608, if s.b[608] { 1.0 } else { 0.0 });s.b[609] = (p.p157 == 1.0);s.store_scalar(609, if s.b[609] { 1.0 } else { 0.0 });s.store_sub_from_scalar_ad(195, p.p222, A::mul(A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p227)), (((((-1.0)) * (p.p227))) + (p.p220))), A::voltage(ctx, nodes, Some(0), Some(2))));s.store_add_scaled_inputs3_offset_mixed_iia(195, 195, (p.p4 * p.p5), 195, ((-0.5) * (p.p4 * p.p5)), A::sqrt_square_offset(A::offset(s.ad_value(195), (-1e-25)), p.p221), ((-(-0.5)) * (p.p4 * p.p5)), ((1e-25 + ((-0.5) * 1e-25)) * (p.p4 * p.p5)));
    }
}
