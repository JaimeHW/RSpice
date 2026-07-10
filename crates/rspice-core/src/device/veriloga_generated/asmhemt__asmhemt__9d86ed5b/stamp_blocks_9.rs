#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_63(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[523]) && s.b[532]) {s.store_mul(86, 303, 90);s.store_sub(39, 294, 86);s.copy_ad(130, 39);s.store_scaled_add_mixed_ia(131, 130, A::sqrt_square_offset(s.ad_value(130), ((4.0 * 0.3) * 0.3)), 0.5);s.copy_ad(154, 131);s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);let t0: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, t0, (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(t0, ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);}
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
        if (((!s.b[523]) && s.b[532]) && s.b[537]) {s.store_scaled_mul(107, 136, 91, p.p208);s.store_scaled_mul(108, 136, 91, p.p209);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_64(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[523]) && s.b[532]) && s.b[537]) {s.store_sub(115, 130, 114);s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_mul_scaled_powf_rhs(116, 136, p.p208, 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p.p209, 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
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
    pub(super) fn stamp_reactive_block_65(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[538] && s.b[539]) {s.store_sub_ad(88, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p201)), (((((-1.0)) * (p.p201))) + (p.p198))), A::div_scaled_inputs(s.ad_value(316), (p.p207 * p.p206), A::sqrt_square_offset(s.ad_value(316), (p.p207 * p.p207)), 1.0));s.store_scalar(307, (p.p9 / p.p199));s.store_div_scalar_by_product_indices(136, p.p200, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));s.store_add_scaled_product_mixed_iia(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p197), 1.0);s.store_add_scaled_inputs4_mixed_iiai(160, 314, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(314), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);s.store_sub(306, 160, 88);s.store_div_scaled_inputs_indices(84, 307, 1.0, 83, (1.602176634e-19 * 3.24e17));s.store_div_from_scalar(150, 2.718281828459045, 84);s.store_div_from_scalar(151, 1.0, 84);s.store_primal_scale(99, 307, 6.241509074460763e18);s.store_scaled_add_mixed_ia(154, 306, A::sqrt_square_offset(s.ad_value(306), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);let t1: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, t1, (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(t1, ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 306, 1.0, 83, 2.0);}
        s.b[542] = (s.v[136] < 200.0);s.store_scalar(542, if s.b[542] { 1.0 } else { 0.0 });
        if ((s.b[538] && s.b[539]) && s.b[542]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((s.b[538] && s.b[539]) && (!s.b[542])) {s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (s.b[538] && s.b[539]) {s.store_sub_div_rhs_indices(100, 306, 153, 99);}
        s.b[543] = ((((s.v[100] - s.v[306])) as f64).abs() > 1e-19);s.store_scalar(543, if s.b[543] { 1.0 } else { 0.0 });
        if ((s.b[538] && s.b[539]) && s.b[543]) {s.store_sub(101, 306, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p.p208);s.store_scaled_mul(103, 136, 90, p.p209);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
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
    pub(super) fn stamp_reactive_block_67(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[538] && s.b[539]) {s.store_sub(39, 306, 86);s.copy_ad(130, 39);s.store_scaled_add_mixed_ia(131, 130, A::sqrt_square_offset(s.ad_value(130), ((4.0 * 0.3) * 0.3)), 0.5);s.copy_ad(154, 131);s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);let t2: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, t2, (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(t2, ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);}
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
    pub(super) fn stamp_reactive_block_68(
        s: &mut ReactiveScratch,
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
        if (s.b[538] && s.b[539]) {s.store_scaled_add(310, 308, 309, 0.5);s.store_sub(311, 309, 308);s.store_mul_add_scaled_inputs3_offset_rhs_indices(135, 311, 306, 1.0, 310, (-1.0), 83, 1.0, 0.0);s.store_mul_scaled_abs_ad_rhs(136, 307, 1.0 / (p.p9), A::sub(s.ad_value(306), s.ad_value(310)));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);s.store_scaled_mul(96, 95, 307, (p.p4 * (p.p5 * 1.0 / (p.p200))));s.store_mul_scale_offset_mixed_ia(98, 96, A::sub_scaled_inputs(s.ad_value(316), p.p21, s.ad_value(86), p.p21), 1.0, 1.0);s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(311), (p.p25 * p.p25), s.ad_value(311)), 1.0);s.store_div(93, 98, 92);s.store_sub(90, 309, 308);s.store_add_scaled_inputs3_indices(91, 306, 1.0, 83, 1.0, 310, -1.0);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(137, 307, 306, ((p.p4 * p.p5) * p.p200), 310, (((-1.0)) * (((p.p4 * p.p5) * p.p200))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p200), 0.0);s.store_scale(188, 137, (1.0 / (p.p245) * 1e26));s.store_offset_powf_ad(189, s.ad_value(188), p.p244, 1.0);s.store_div_from_scalar(190, p.p243, 189);s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p199);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(312, 191, 306, ((p.p4 * p.p5) * p.p200), 310, (((-1.0)) * (((p.p4 * p.p5) * p.p200))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p200), 0.0);s.store_add_scaled_inputs3_indices(136, 306, 1.0, 83, 1.0, 310, -1.0);s.store_add_scaled_inputs(90, 308, 0.3333333333333333, 309, (2.0 * 0.3333333333333333));s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(311)), (1.0 / 12.0), 136, 1.0);s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(311)), 311, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);s.store_mul_add_scaled_inputs4_indices_rhs(313, 191, 306, (-(((p.p4 * p.p200) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p200) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p200) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p200) * p.p5) * 0.5)));}
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
        s.b[547] = (p.p157 != 0.0);s.store_scalar(547, if s.b[547] { 1.0 } else { 0.0 });s.b[548] = (p.p157 == 1.0);s.store_scalar(548, if s.b[548] { 1.0 } else { 0.0 });
        if (((!s.b[538]) && s.b[547]) && s.b[548]) {s.store_voltage(78, ctx, nodes, Some(9), Some(8));}
        if (((!s.b[538]) && s.b[547]) && (!s.b[548])) {s.store_voltage(78, ctx, nodes, Some(2), Some(8));}
        if ((!s.b[538]) && s.b[547]) {s.copy_ad(314, 78);s.store_scalar(146, (1.0 + p.p204));s.store_scaled_mul(83, 82, 146, 8.617087e-5);s.store_offset_scaled(88, 82, ((1.0 / (s.v[35])) * (p.p201)), (((((-1.0)) * (p.p201))) + (p.p198)));s.store_scalar(307, (p.p9 / p.p199));s.store_div_scalar_by_product_indices(136, p.p200, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));s.store_add_scaled_product_mixed_iia(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p197), 1.0);s.store_add_scaled_inputs4_mixed_iiai(160, 314, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(314), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);s.store_sub(306, 160, 88);s.store_div_scaled_inputs_indices(84, 307, 1.0, 83, (1.602176634e-19 * 3.24e17));s.store_div_from_scalar(150, 2.718281828459045, 84);s.store_div_from_scalar(151, 1.0, 84);s.store_primal_scale(99, 307, 6.241509074460763e18);s.store_scaled_add_mixed_ia(154, 306, A::sqrt_square_offset(s.ad_value(306), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);let t3: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, t3, (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(t3, ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 306, 1.0, 83, 2.0);}
        s.b[549] = (s.v[136] < 200.0);s.store_scalar(549, if s.b[549] { 1.0 } else { 0.0 });
        if (((!s.b[538]) && s.b[547]) && s.b[549]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (((!s.b[538]) && s.b[547]) && (!s.b[549])) {s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((!s.b[538]) && s.b[547]) {s.store_sub_div_rhs_indices(100, 306, 153, 99);}
        s.b[550] = ((((s.v[100] - s.v[306])) as f64).abs() > 1e-19);s.store_scalar(550, if s.b[550] { 1.0 } else { 0.0 });
        if (((!s.b[538]) && s.b[547]) && s.b[550]) {s.store_sub(101, 306, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p.p208);s.store_scaled_mul(103, 136, 90, p.p209);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
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
    pub(super) fn stamp_reactive_block_71(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[538]) && s.b[547]) {s.store_mul(86, 315, 90);s.store_sub(39, 306, 86);s.copy_ad(130, 39);s.store_scaled_add_mixed_ia(131, 130, A::sqrt_square_offset(s.ad_value(130), ((4.0 * 0.3) * 0.3)), 0.5);s.copy_ad(154, 131);s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);let t4: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, t4, (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(t4, ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);}
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
    pub(super) fn stamp_reactive_block_72(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
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
        s.b[558] = (p.p255 == 2.0);s.store_scalar(558, if s.b[558] { 1.0 } else { 0.0 });
        if s.b[558] {s.store_scaled_voltage(162, ctx, nodes, Some(10), Some(2), ((p.p4 * p.p5) * p.p210));s.store_div_scaled_inputs(168, A::voltage(ctx, nodes, Some(0), Some(2)), p.p214, A::sqrt(A::offset(A::square(A::voltage(ctx, nodes, Some(0), Some(2))), (p.p214 * p.p214))), 1.0);s.store_scalar(169, (p.p213).min((p.p211 / (2.0 * p.p214))));s.store_sub_from_scalar_scaled_mul(167, ((p.p4 * p.p5) * p.p211), 169, 168, (p.p4 * p.p5));s.store_mul_voltage_ad(163, A::max_with_scalar(s.ad_value(167), 0.0), ctx, nodes, Some(10), Some(0));}
        if (!s.b[558]) {s.store_scaled_voltage(162, ctx, nodes, Some(1), Some(2), ((p.p4 * p.p5) * p.p210));s.store_div_scaled_inputs(168, A::voltage(ctx, nodes, Some(0), Some(2)), p.p214, A::sqrt(A::offset(A::square(A::voltage(ctx, nodes, Some(0), Some(2))), (p.p214 * p.p214))), 1.0);s.store_scalar(169, (p.p213).min((p.p211 / (2.0 * p.p214))));s.store_sub_from_scalar_scaled_mul(167, ((p.p4 * p.p5) * p.p211), 169, 168, (p.p4 * p.p5));s.store_mul_voltage_ad(163, A::max_with_scalar(s.ad_value(167), 0.0), ctx, nodes, Some(1), Some(0));}
        s.store_scaled_voltage(164, ctx, nodes, Some(0), Some(2), ((p.p4 * p.p5) * p.p212));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_73(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.store_scaled_voltage(219, ctx, nodes, Some(3), Some(0), ((p.p4 * p.p5) * p.p215));s.store_scaled_voltage(220, ctx, nodes, Some(3), Some(2), ((p.p4 * p.p5) * p.p216));s.store_scaled_voltage(221, ctx, nodes, Some(3), Some(1), ((p.p4 * p.p5) * p.p217));s.store_scale_ad(377, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p281), p.p277);s.store_scale_ad(378, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p282), p.p278);s.store_scale(137, 378, (p.p4 * p.p5));s.store_scale(137, 377, (p.p4 * p.p5));s.b[569] = (p.p255 == 2.0);s.store_scalar(569, if s.b[569] { 1.0 } else { 0.0 });s.b[570] = (p.p149 == 0.0);s.store_scalar(570, if s.b[570] { 1.0 } else { 0.0 });s.b[571] = (p.p150 != 0.0);s.store_scalar(571, if s.b[571] { 1.0 } else { 0.0 });s.b[572] = (p.p150 == 1.0);s.store_scalar(572, if s.b[572] { 1.0 } else { 0.0 });s.b[573] = (p.p150 != 0.0);s.store_scalar(573, if s.b[573] { 1.0 } else { 0.0 });s.b[574] = (p.p150 == 1.0);s.store_scalar(574, if s.b[574] { 1.0 } else { 0.0 });s.b[575] = (p.p149 == 0.0);s.store_scalar(575, if s.b[575] { 1.0 } else { 0.0 });s.b[576] = (p.p151 != 0.0);s.store_scalar(576, if s.b[576] { 1.0 } else { 0.0 });s.b[577] = (p.p151 == 1.0);s.store_scalar(577, if s.b[577] { 1.0 } else { 0.0 });s.b[578] = (p.p151 != 0.0);s.store_scalar(578, if s.b[578] { 1.0 } else { 0.0 });s.b[579] = (p.p151 == 1.0);s.store_scalar(579, if s.b[579] { 1.0 } else { 0.0 });s.b[580] = (p.p149 == 0.0);s.store_scalar(580, if s.b[580] { 1.0 } else { 0.0 });s.b[581] = (p.p152 != 0.0);s.store_scalar(581, if s.b[581] { 1.0 } else { 0.0 });s.b[582] = (p.p152 == 1.0);s.store_scalar(582, if s.b[582] { 1.0 } else { 0.0 });s.b[583] = (p.p152 != 0.0);s.store_scalar(583, if s.b[583] { 1.0 } else { 0.0 });s.b[584] = (p.p152 == 1.0);s.store_scalar(584, if s.b[584] { 1.0 } else { 0.0 });s.b[585] = (p.p149 == 0.0);s.store_scalar(585, if s.b[585] { 1.0 } else { 0.0 });s.b[586] = (p.p153 != 0.0);s.store_scalar(586, if s.b[586] { 1.0 } else { 0.0 });s.b[587] = (p.p153 == 1.0);s.store_scalar(587, if s.b[587] { 1.0 } else { 0.0 });s.b[588] = (p.p153 != 0.0);s.store_scalar(588, if s.b[588] { 1.0 } else { 0.0 });s.b[589] = (p.p153 == 1.0);s.store_scalar(589, if s.b[589] { 1.0 } else { 0.0 });s.b[590] = (p.p149 == 0.0);s.store_scalar(590, if s.b[590] { 1.0 } else { 0.0 });s.b[591] = (p.p154 != 0.0);s.store_scalar(591, if s.b[591] { 1.0 } else { 0.0 });s.b[592] = (p.p154 == 1.0);s.store_scalar(592, if s.b[592] { 1.0 } else { 0.0 });s.b[593] = (p.p154 != 0.0);s.store_scalar(593, if s.b[593] { 1.0 } else { 0.0 });s.b[594] = (p.p154 == 1.0);s.store_scalar(594, if s.b[594] { 1.0 } else { 0.0 });s.b[595] = (p.p149 == 0.0);s.store_scalar(595, if s.b[595] { 1.0 } else { 0.0 });s.b[596] = (p.p155 != 0.0);s.store_scalar(596, if s.b[596] { 1.0 } else { 0.0 });s.b[597] = (p.p155 == 1.0);s.store_scalar(597, if s.b[597] { 1.0 } else { 0.0 });s.b[598] = (p.p155 != 0.0);s.store_scalar(598, if s.b[598] { 1.0 } else { 0.0 });s.b[599] = (p.p155 == 1.0);s.store_scalar(599, if s.b[599] { 1.0 } else { 0.0 });s.b[600] = (p.p149 == 0.0);s.store_scalar(600, if s.b[600] { 1.0 } else { 0.0 });s.b[601] = (p.p156 != 0.0);s.store_scalar(601, if s.b[601] { 1.0 } else { 0.0 });s.b[602] = (p.p156 == 1.0);s.store_scalar(602, if s.b[602] { 1.0 } else { 0.0 });s.b[603] = (p.p156 != 0.0);s.store_scalar(603, if s.b[603] { 1.0 } else { 0.0 });s.b[604] = (p.p156 == 1.0);s.store_scalar(604, if s.b[604] { 1.0 } else { 0.0 });s.b[605] = (p.p149 == 0.0);s.store_scalar(605, if s.b[605] { 1.0 } else { 0.0 });s.b[606] = (p.p157 != 0.0);s.store_scalar(606, if s.b[606] { 1.0 } else { 0.0 });s.b[607] = (p.p157 == 1.0);s.store_scalar(607, if s.b[607] { 1.0 } else { 0.0 });s.b[608] = (p.p157 != 0.0);s.store_scalar(608, if s.b[608] { 1.0 } else { 0.0 });s.b[609] = (p.p157 == 1.0);s.store_scalar(609, if s.b[609] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_74(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.store_sub_from_scalar_ad(195, p.p222, A::mul(A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p227)), (((((-1.0)) * (p.p227))) + (p.p220))), A::voltage(ctx, nodes, Some(0), Some(2))));s.store_add_scaled_inputs3_offset_mixed_iia(195, 195, (p.p4 * p.p5), 195, ((-0.5) * (p.p4 * p.p5)), A::sqrt_square_offset(A::offset(s.ad_value(195), (-1e-25)), p.p221), ((-(-0.5)) * (p.p4 * p.p5)), ((1e-25 + ((-0.5) * 1e-25)) * (p.p4 * p.p5)));s.store_scaled_add_offset_sqrt_square_offset_ad(136, A::sub_from_scalar(p.p218, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p226)), (((-1.0)) * (p.p226)))), 1e-18, (-1e-18), ((0.25 * 1e-19) * 1e-19), 0.5);s.store_mul_scaled_voltage(196, 136, (p.p4 * p.p5), ctx, nodes, Some(9), Some(2));s.store_scaled_voltage(197, ctx, nodes, Some(2), Some(0), ((p.p4 * p.p5) * p.p219));s.store_offset_scaled_ad(136, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p225)), (((-1.0)) * (p.p225))), (-(1.0 - { let limited_exp_arg = ((-((p.p229) as f64).ln()) / p.p228); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })), ((p.p224) * ((1.0 - { let limited_exp_arg = ((-((p.p229) as f64).ln()) / p.p228); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))));s.store_div_scaled_inputs2_mixed_iai(90, 136, 1.0, A::voltage(ctx, nodes, Some(2), Some(0)), (-1.0), 36, 1.0);s.store_sqrt_offset_ad(91, A::mul_scaled_lhs(s.ad_value(90), p.p230, s.ad_value(90)), 1.92);s.store_scaled_add(137, 90, 91, 0.5);s.store_add_scaled_product_indices(106, 136, 1.0, 36, 137, (-1.0));s.store_ln_ad(192, A::sub_from_scalar(1.0, A::scale(s.ad_value(106), 1.0 / (p.p224))));s.store_mul_scale_offset(193, A::sub_from_scalar(1.0, A::limited_exp_scaled_input(s.ad_value(192), (1.0 - p.p228))), A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p225)), (((-1.0)) * (p.p225))), -((p.p223 * 1.0 / ((1.0 - p.p228)))), (p.p224) * ((p.p223 * 1.0 / ((1.0 - p.p228)))));s.store_add_scaled_inputs3_mixed_iai(194, 193, (p.p4 * p.p5), A::voltage(ctx, nodes, Some(2), Some(0)), ((p.p229 * p.p223) * (p.p4 * p.p5)), 106, ((-(p.p229 * p.p223)) * (p.p4 * p.p5)));s.b[610] = ((p.p31 == 1.0) && (p.p32 > 0.0));s.store_scalar(610, if s.b[610] { 1.0 } else { 0.0 });
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
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_2(
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
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let nv5 = ctx.node_voltage(nodes[5]);let nv6 = ctx.node_voltage(nodes[6]);
        let (eq8_e345, eq8_e345_d_n5, eq8_e345_d_n6,) = {
    if (s.b[388] && (!s.b[387])) {
        let __rspice_inv_cse_0: f64 = 1.0 / 10.0;let eq8_e339: f64 = ((nv6 - nv5) * __rspice_inv_cse_0);let eq8_e339_d_n5: f64 = ((-1.0) * __rspice_inv_cse_0);let eq8_e339_d_n6: f64 = (1.0 * __rspice_inv_cse_0);let eq8_e340: f64 = { let limited_exp_arg = eq8_e339; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };let eq8_e340_d_n5: f64 = ({ let limited_exp_arg = eq8_e339; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * eq8_e339_d_n5);let eq8_e340_d_n6: f64 = ({ let limited_exp_arg = eq8_e339; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * eq8_e339_d_n6);let eq8_e342: f64 = (eq8_e340 - 1.0);let eq8_e343: f64 = (p.p99 * eq8_e342);let eq8_e343_d_n5: f64 = (p.p99 * eq8_e340_d_n5);let eq8_e343_d_n6: f64 = (p.p99 * eq8_e340_d_n6);
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
        let eq9_e352: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (nv5 - 0.0));let eq9_e353: f64 = (p.p97 * eq9_e352);
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
        let (eq10_e364, eq10_e364_d_n5,) = {
    if (s.b[388] && (!s.b[387])) {
        let __rspice_inv_cse_1: f64 = 1.0 / p.p98;let eq10_e362: f64 = ((nv5 - 0.0) * __rspice_inv_cse_1);let eq10_e362_d_n5: f64 = (1.0 * __rspice_inv_cse_1);
        (eq10_e362, eq10_e362_d_n5,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e364;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (eq10_value),
            5,
            multiplicity * (eq10_e364_d_n5),
        );
        let (eq11_e371,) = {
    if (s.b[388] && (!s.b[387])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq11_value: f64 = eq11_e371;
        stamper.stamp_potential_const_local(
            8,
            eq11_value,
        );
        let (eq12_e378,) = {
    if (s.b[388] && (!s.b[387])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq12_value: f64 = eq12_e378;
        stamper.stamp_potential_const_local(
            9,
            eq12_value,
        );
        let (eq13_e385,) = {
    if (s.b[388] && (!s.b[387])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq13_value: f64 = eq13_e385;
        stamper.stamp_potential_const_local(
            10,
            eq13_value,
        );
        let (eq14_e392,) = {
    if (s.b[388] && (!s.b[387])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq14_value: f64 = eq14_e392;
        stamper.stamp_potential_const_local(
            11,
            eq14_value,
        );
        let (eq15_e403, eq15_e403_d_n5,) = {
    if (s.b[389] && (!(s.b[387] || s.b[388]))) {
        let __rspice_inv_cse_2: f64 = 1.0 / p.p108;let eq15_e401: f64 = ((nv5 - 0.0) * __rspice_inv_cse_2);let eq15_e401_d_n5: f64 = (1.0 * __rspice_inv_cse_2);
        (eq15_e401, eq15_e401_d_n5,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e403;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (eq15_value),
            5,
            multiplicity * (eq15_e403_d_n5),
        );
        let (eq16_e415, eq16_e415_d_n0, eq16_e415_d_n1, eq16_e415_d_n2, eq16_e415_d_n3, eq16_e415_d_n4, eq16_e415_d_n5, eq16_e415_d_n6, eq16_e415_d_n7, eq16_e415_d_n8, eq16_e415_d_n9, eq16_e415_d_n10, eq16_e415_d_n11, eq16_e415_d_n12, eq16_e415_d_n13, eq16_e415_d_n14, eq16_e415_d_n15, eq16_e415_d_n16, eq16_e415_d_n17, eq16_e415_d_n18, eq16_e415_d_n19, eq16_e415_d_n20, eq16_e415_d_n21, eq16_e415_d_n22, eq16_e415_d_b0, eq16_e415_d_b1, eq16_e415_d_b2, eq16_e415_d_b3, eq16_e415_d_b4, eq16_e415_d_b5, eq16_e415_d_b6, eq16_e415_d_b7, eq16_e415_d_b8, eq16_e415_d_b9, eq16_e415_d_b10, eq16_e415_d_b11, eq16_e415_d_b12, eq16_e415_d_b13, eq16_e415_d_b14, eq16_e415_d_b15, eq16_e415_d_b16, eq16_e415_d_b17, eq16_e415_d_b18, eq16_e415_d_b19, eq16_e415_d_b20, eq16_e415_d_b21, eq16_e415_d_b22, eq16_e415_d_b23, eq16_e415_d_b24, eq16_e415_d_b25, eq16_e415_d_b26, eq16_e415_d_b27, eq16_e415_d_b28, eq16_e415_d_b29, eq16_e415_d_b30, eq16_e415_d_b31, eq16_e415_d_b32, eq16_e415_d_b33, eq16_e415_d_b34, eq16_e415_d_b35, eq16_e415_d_b36, eq16_e415_d_b37, eq16_e415_d_b38, eq16_e415_d_b39, eq16_e415_d_b40, eq16_e415_d_b41, eq16_e415_d_b42, eq16_e415_d_b43, eq16_e415_d_b44, eq16_e415_d_b45, eq16_e415_d_b46, eq16_e415_d_b47, eq16_e415_d_b48, eq16_e415_d_b49, eq16_e415_d_b50, eq16_e415_d_b51, eq16_e415_d_b52, eq16_e415_d_b53, eq16_e415_d_b54,) = {
    if (s.b[389] && (!(s.b[387] || s.b[388]))) {
        let eq16_e411: f64 = (-1.0);let eq16_e413: f64 = (eq16_e411 * s.v[208]);
        (eq16_e413, (eq16_e411 * s.dn[208][0]), (eq16_e411 * s.dn[208][1]), (eq16_e411 * s.dn[208][2]), (eq16_e411 * s.dn[208][3]), (eq16_e411 * s.dn[208][4]), (eq16_e411 * s.dn[208][5]), (eq16_e411 * s.dn[208][6]), (eq16_e411 * s.dn[208][7]), (eq16_e411 * s.dn[208][8]), (eq16_e411 * s.dn[208][9]), (eq16_e411 * s.dn[208][10]), (eq16_e411 * s.dn[208][11]), (eq16_e411 * s.dn[208][12]), (eq16_e411 * s.dn[208][13]), (eq16_e411 * s.dn[208][14]), (eq16_e411 * s.dn[208][15]), (eq16_e411 * s.dn[208][16]), (eq16_e411 * s.dn[208][17]), (eq16_e411 * s.dn[208][18]), (eq16_e411 * s.dn[208][19]), (eq16_e411 * s.dn[208][20]), (eq16_e411 * s.dn[208][21]), (eq16_e411 * s.dn[208][22]), (eq16_e411 * s.db[208][0]), (eq16_e411 * s.db[208][1]), (eq16_e411 * s.db[208][2]), (eq16_e411 * s.db[208][3]), (eq16_e411 * s.db[208][4]), (eq16_e411 * s.db[208][5]), (eq16_e411 * s.db[208][6]), (eq16_e411 * s.db[208][7]), (eq16_e411 * s.db[208][8]), (eq16_e411 * s.db[208][9]), (eq16_e411 * s.db[208][10]), (eq16_e411 * s.db[208][11]), (eq16_e411 * s.db[208][12]), (eq16_e411 * s.db[208][13]), (eq16_e411 * s.db[208][14]), (eq16_e411 * s.db[208][15]), (eq16_e411 * s.db[208][16]), (eq16_e411 * s.db[208][17]), (eq16_e411 * s.db[208][18]), (eq16_e411 * s.db[208][19]), (eq16_e411 * s.db[208][20]), (eq16_e411 * s.db[208][21]), (eq16_e411 * s.db[208][22]), (eq16_e411 * s.db[208][23]), (eq16_e411 * s.db[208][24]), (eq16_e411 * s.db[208][25]), (eq16_e411 * s.db[208][26]), (eq16_e411 * s.db[208][27]), (eq16_e411 * s.db[208][28]), (eq16_e411 * s.db[208][29]), (eq16_e411 * s.db[208][30]), (eq16_e411 * s.db[208][31]), (eq16_e411 * s.db[208][32]), (eq16_e411 * s.db[208][33]), (eq16_e411 * s.db[208][34]), (eq16_e411 * s.db[208][35]), (eq16_e411 * s.db[208][36]), (eq16_e411 * s.db[208][37]), (eq16_e411 * s.db[208][38]), (eq16_e411 * s.db[208][39]), (eq16_e411 * s.db[208][40]), (eq16_e411 * s.db[208][41]), (eq16_e411 * s.db[208][42]), (eq16_e411 * s.db[208][43]), (eq16_e411 * s.db[208][44]), (eq16_e411 * s.db[208][45]), (eq16_e411 * s.db[208][46]), (eq16_e411 * s.db[208][47]), (eq16_e411 * s.db[208][48]), (eq16_e411 * s.db[208][49]), (eq16_e411 * s.db[208][50]), (eq16_e411 * s.db[208][51]), (eq16_e411 * s.db[208][52]), (eq16_e411 * s.db[208][53]), (eq16_e411 * s.db[208][54]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e415;let eq16_node_derivatives: [f64; 23] = [eq16_e415_d_n0, eq16_e415_d_n1, eq16_e415_d_n2, eq16_e415_d_n3, eq16_e415_d_n4, eq16_e415_d_n5, eq16_e415_d_n6, eq16_e415_d_n7, eq16_e415_d_n8, eq16_e415_d_n9, eq16_e415_d_n10, eq16_e415_d_n11, eq16_e415_d_n12, eq16_e415_d_n13, eq16_e415_d_n14, eq16_e415_d_n15, eq16_e415_d_n16, eq16_e415_d_n17, eq16_e415_d_n18, eq16_e415_d_n19, eq16_e415_d_n20, eq16_e415_d_n21, eq16_e415_d_n22];let eq16_branch_derivatives: [f64; 55] = [eq16_e415_d_b0, eq16_e415_d_b1, eq16_e415_d_b2, eq16_e415_d_b3, eq16_e415_d_b4, eq16_e415_d_b5, eq16_e415_d_b6, eq16_e415_d_b7, eq16_e415_d_b8, eq16_e415_d_b9, eq16_e415_d_b10, eq16_e415_d_b11, eq16_e415_d_b12, eq16_e415_d_b13, eq16_e415_d_b14, eq16_e415_d_b15, eq16_e415_d_b16, eq16_e415_d_b17, eq16_e415_d_b18, eq16_e415_d_b19, eq16_e415_d_b20, eq16_e415_d_b21, eq16_e415_d_b22, eq16_e415_d_b23, eq16_e415_d_b24, eq16_e415_d_b25, eq16_e415_d_b26, eq16_e415_d_b27, eq16_e415_d_b28, eq16_e415_d_b29, eq16_e415_d_b30, eq16_e415_d_b31, eq16_e415_d_b32, eq16_e415_d_b33, eq16_e415_d_b34, eq16_e415_d_b35, eq16_e415_d_b36, eq16_e415_d_b37, eq16_e415_d_b38, eq16_e415_d_b39, eq16_e415_d_b40, eq16_e415_d_b41, eq16_e415_d_b42, eq16_e415_d_b43, eq16_e415_d_b44, eq16_e415_d_b45, eq16_e415_d_b46, eq16_e415_d_b47, eq16_e415_d_b48, eq16_e415_d_b49, eq16_e415_d_b50, eq16_e415_d_b51, eq16_e415_d_b52, eq16_e415_d_b53, eq16_e415_d_b54];
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
        let eq17_e424: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, (nv5 - 0.0));let eq17_e425: f64 = (p.p110 * eq17_e424);
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
        let (eq18_e438, eq18_e438_d_n6,) = {
    if (s.b[389] && (!(s.b[387] || s.b[388]))) {
        let __rspice_inv_cse_3: f64 = 1.0 / p.p109;let eq18_e436: f64 = ((nv6 - 0.0) * __rspice_inv_cse_3);let eq18_e436_d_n6: f64 = (1.0 * __rspice_inv_cse_3);
        (eq18_e436, eq18_e436_d_n6,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e438;
        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * (eq18_value),
            6,
            multiplicity * (eq18_e438_d_n6),
        );
        let (eq19_e450, eq19_e450_d_n0, eq19_e450_d_n2,) = {
    if (s.b[389] && (!(s.b[387] || s.b[388]))) {
        let eq19_e446: f64 = (-1.0);let eq19_e448: f64 = (eq19_e446 * (nv0 - nv2));
        (eq19_e448, eq19_e446, (-eq19_e446),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e450;
        stamper.stamp_current_node2_local(
            Some(6),
            None,
            multiplicity * (eq19_value),
            0,
            multiplicity * (eq19_e450_d_n0),
            2,
            multiplicity * (eq19_e450_d_n2),
        );
        let (eq20_e462, eq20_e462_d_n6,) = {
    if (s.b[389] && (!(s.b[387] || s.b[388]))) {
        let eq20_e459: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, (nv6 - 0.0));let eq20_e460: f64 = (p.p111 * eq20_e459);
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
        let (eq21_e471,) = {
    if (s.b[389] && (!(s.b[387] || s.b[388]))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq21_value: f64 = eq21_e471;
        stamper.stamp_potential_const_local(
            12,
            eq21_value,
        );
        let (eq22_e480,) = {
    if (s.b[389] && (!(s.b[387] || s.b[388]))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e480;
        stamper.stamp_potential_const_local(
            13,
            eq22_value,
        );
        let (eq23_e489,) = {
    if (s.b[389] && (!(s.b[387] || s.b[388]))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e489;
        stamper.stamp_potential_const_local(
            14,
            eq23_value,
        );
        let (eq24_e498,) = {
    if (s.b[389] && (!(s.b[387] || s.b[388]))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq24_value: f64 = eq24_e498;
        stamper.stamp_potential_const_local(
            15,
            eq24_value,
        );
        let (eq25_e511, eq25_e511_d_n5,) = {
    if (s.b[390] && (!((s.b[387] || s.b[388]) || s.b[389]))) {
        let __rspice_inv_cse_4: f64 = 1.0 / p.p119;let eq25_e509: f64 = ((nv5 - 0.0) * __rspice_inv_cse_4);let eq25_e509_d_n5: f64 = (1.0 * __rspice_inv_cse_4);
        (eq25_e509, eq25_e509_d_n5,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e511;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (eq25_value),
            5,
            multiplicity * (eq25_e511_d_n5),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq26_e525, eq26_e525_d_n0, eq26_e525_d_n1, eq26_e525_d_n2, eq26_e525_d_n3, eq26_e525_d_n4, eq26_e525_d_n5, eq26_e525_d_n6, eq26_e525_d_n7, eq26_e525_d_n8, eq26_e525_d_n9, eq26_e525_d_n10, eq26_e525_d_n11, eq26_e525_d_n12, eq26_e525_d_n13, eq26_e525_d_n14, eq26_e525_d_n15, eq26_e525_d_n16, eq26_e525_d_n17, eq26_e525_d_n18, eq26_e525_d_n19, eq26_e525_d_n20, eq26_e525_d_n21, eq26_e525_d_n22, eq26_e525_d_b0, eq26_e525_d_b1, eq26_e525_d_b2, eq26_e525_d_b3, eq26_e525_d_b4, eq26_e525_d_b5, eq26_e525_d_b6, eq26_e525_d_b7, eq26_e525_d_b8, eq26_e525_d_b9, eq26_e525_d_b10, eq26_e525_d_b11, eq26_e525_d_b12, eq26_e525_d_b13, eq26_e525_d_b14, eq26_e525_d_b15, eq26_e525_d_b16, eq26_e525_d_b17, eq26_e525_d_b18, eq26_e525_d_b19, eq26_e525_d_b20, eq26_e525_d_b21, eq26_e525_d_b22, eq26_e525_d_b23, eq26_e525_d_b24, eq26_e525_d_b25, eq26_e525_d_b26, eq26_e525_d_b27, eq26_e525_d_b28, eq26_e525_d_b29, eq26_e525_d_b30, eq26_e525_d_b31, eq26_e525_d_b32, eq26_e525_d_b33, eq26_e525_d_b34, eq26_e525_d_b35, eq26_e525_d_b36, eq26_e525_d_b37, eq26_e525_d_b38, eq26_e525_d_b39, eq26_e525_d_b40, eq26_e525_d_b41, eq26_e525_d_b42, eq26_e525_d_b43, eq26_e525_d_b44, eq26_e525_d_b45, eq26_e525_d_b46, eq26_e525_d_b47, eq26_e525_d_b48, eq26_e525_d_b49, eq26_e525_d_b50, eq26_e525_d_b51, eq26_e525_d_b52, eq26_e525_d_b53, eq26_e525_d_b54,) = {
    if (s.b[390] && (!((s.b[387] || s.b[388]) || s.b[389]))) {
        let eq26_e521: f64 = (-1.0);let eq26_e523: f64 = (eq26_e521 * s.v[148]);
        (eq26_e523, (eq26_e521 * s.dn[148][0]), (eq26_e521 * s.dn[148][1]), (eq26_e521 * s.dn[148][2]), (eq26_e521 * s.dn[148][3]), (eq26_e521 * s.dn[148][4]), (eq26_e521 * s.dn[148][5]), (eq26_e521 * s.dn[148][6]), (eq26_e521 * s.dn[148][7]), (eq26_e521 * s.dn[148][8]), (eq26_e521 * s.dn[148][9]), (eq26_e521 * s.dn[148][10]), (eq26_e521 * s.dn[148][11]), (eq26_e521 * s.dn[148][12]), (eq26_e521 * s.dn[148][13]), (eq26_e521 * s.dn[148][14]), (eq26_e521 * s.dn[148][15]), (eq26_e521 * s.dn[148][16]), (eq26_e521 * s.dn[148][17]), (eq26_e521 * s.dn[148][18]), (eq26_e521 * s.dn[148][19]), (eq26_e521 * s.dn[148][20]), (eq26_e521 * s.dn[148][21]), (eq26_e521 * s.dn[148][22]), (eq26_e521 * s.db[148][0]), (eq26_e521 * s.db[148][1]), (eq26_e521 * s.db[148][2]), (eq26_e521 * s.db[148][3]), (eq26_e521 * s.db[148][4]), (eq26_e521 * s.db[148][5]), (eq26_e521 * s.db[148][6]), (eq26_e521 * s.db[148][7]), (eq26_e521 * s.db[148][8]), (eq26_e521 * s.db[148][9]), (eq26_e521 * s.db[148][10]), (eq26_e521 * s.db[148][11]), (eq26_e521 * s.db[148][12]), (eq26_e521 * s.db[148][13]), (eq26_e521 * s.db[148][14]), (eq26_e521 * s.db[148][15]), (eq26_e521 * s.db[148][16]), (eq26_e521 * s.db[148][17]), (eq26_e521 * s.db[148][18]), (eq26_e521 * s.db[148][19]), (eq26_e521 * s.db[148][20]), (eq26_e521 * s.db[148][21]), (eq26_e521 * s.db[148][22]), (eq26_e521 * s.db[148][23]), (eq26_e521 * s.db[148][24]), (eq26_e521 * s.db[148][25]), (eq26_e521 * s.db[148][26]), (eq26_e521 * s.db[148][27]), (eq26_e521 * s.db[148][28]), (eq26_e521 * s.db[148][29]), (eq26_e521 * s.db[148][30]), (eq26_e521 * s.db[148][31]), (eq26_e521 * s.db[148][32]), (eq26_e521 * s.db[148][33]), (eq26_e521 * s.db[148][34]), (eq26_e521 * s.db[148][35]), (eq26_e521 * s.db[148][36]), (eq26_e521 * s.db[148][37]), (eq26_e521 * s.db[148][38]), (eq26_e521 * s.db[148][39]), (eq26_e521 * s.db[148][40]), (eq26_e521 * s.db[148][41]), (eq26_e521 * s.db[148][42]), (eq26_e521 * s.db[148][43]), (eq26_e521 * s.db[148][44]), (eq26_e521 * s.db[148][45]), (eq26_e521 * s.db[148][46]), (eq26_e521 * s.db[148][47]), (eq26_e521 * s.db[148][48]), (eq26_e521 * s.db[148][49]), (eq26_e521 * s.db[148][50]), (eq26_e521 * s.db[148][51]), (eq26_e521 * s.db[148][52]), (eq26_e521 * s.db[148][53]), (eq26_e521 * s.db[148][54]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e525;let eq26_node_derivatives: [f64; 23] = [eq26_e525_d_n0, eq26_e525_d_n1, eq26_e525_d_n2, eq26_e525_d_n3, eq26_e525_d_n4, eq26_e525_d_n5, eq26_e525_d_n6, eq26_e525_d_n7, eq26_e525_d_n8, eq26_e525_d_n9, eq26_e525_d_n10, eq26_e525_d_n11, eq26_e525_d_n12, eq26_e525_d_n13, eq26_e525_d_n14, eq26_e525_d_n15, eq26_e525_d_n16, eq26_e525_d_n17, eq26_e525_d_n18, eq26_e525_d_n19, eq26_e525_d_n20, eq26_e525_d_n21, eq26_e525_d_n22];let eq26_branch_derivatives: [f64; 55] = [eq26_e525_d_b0, eq26_e525_d_b1, eq26_e525_d_b2, eq26_e525_d_b3, eq26_e525_d_b4, eq26_e525_d_b5, eq26_e525_d_b6, eq26_e525_d_b7, eq26_e525_d_b8, eq26_e525_d_b9, eq26_e525_d_b10, eq26_e525_d_b11, eq26_e525_d_b12, eq26_e525_d_b13, eq26_e525_d_b14, eq26_e525_d_b15, eq26_e525_d_b16, eq26_e525_d_b17, eq26_e525_d_b18, eq26_e525_d_b19, eq26_e525_d_b20, eq26_e525_d_b21, eq26_e525_d_b22, eq26_e525_d_b23, eq26_e525_d_b24, eq26_e525_d_b25, eq26_e525_d_b26, eq26_e525_d_b27, eq26_e525_d_b28, eq26_e525_d_b29, eq26_e525_d_b30, eq26_e525_d_b31, eq26_e525_d_b32, eq26_e525_d_b33, eq26_e525_d_b34, eq26_e525_d_b35, eq26_e525_d_b36, eq26_e525_d_b37, eq26_e525_d_b38, eq26_e525_d_b39, eq26_e525_d_b40, eq26_e525_d_b41, eq26_e525_d_b42, eq26_e525_d_b43, eq26_e525_d_b44, eq26_e525_d_b45, eq26_e525_d_b46, eq26_e525_d_b47, eq26_e525_d_b48, eq26_e525_d_b49, eq26_e525_d_b50, eq26_e525_d_b51, eq26_e525_d_b52, eq26_e525_d_b53, eq26_e525_d_b54];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq26_value),
            &eq26_node_derivatives,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let (eq27_e539, eq27_e539_d_n0, eq27_e539_d_n1, eq27_e539_d_n2, eq27_e539_d_n3, eq27_e539_d_n4, eq27_e539_d_n5, eq27_e539_d_n6, eq27_e539_d_n7, eq27_e539_d_n8, eq27_e539_d_n9, eq27_e539_d_n10, eq27_e539_d_n11, eq27_e539_d_n12, eq27_e539_d_n13, eq27_e539_d_n14, eq27_e539_d_n15, eq27_e539_d_n16, eq27_e539_d_n17, eq27_e539_d_n18, eq27_e539_d_n19, eq27_e539_d_n20, eq27_e539_d_n21, eq27_e539_d_n22, eq27_e539_d_b0, eq27_e539_d_b1, eq27_e539_d_b2, eq27_e539_d_b3, eq27_e539_d_b4, eq27_e539_d_b5, eq27_e539_d_b6, eq27_e539_d_b7, eq27_e539_d_b8, eq27_e539_d_b9, eq27_e539_d_b10, eq27_e539_d_b11, eq27_e539_d_b12, eq27_e539_d_b13, eq27_e539_d_b14, eq27_e539_d_b15, eq27_e539_d_b16, eq27_e539_d_b17, eq27_e539_d_b18, eq27_e539_d_b19, eq27_e539_d_b20, eq27_e539_d_b21, eq27_e539_d_b22, eq27_e539_d_b23, eq27_e539_d_b24, eq27_e539_d_b25, eq27_e539_d_b26, eq27_e539_d_b27, eq27_e539_d_b28, eq27_e539_d_b29, eq27_e539_d_b30, eq27_e539_d_b31, eq27_e539_d_b32, eq27_e539_d_b33, eq27_e539_d_b34, eq27_e539_d_b35, eq27_e539_d_b36, eq27_e539_d_b37, eq27_e539_d_b38, eq27_e539_d_b39, eq27_e539_d_b40, eq27_e539_d_b41, eq27_e539_d_b42, eq27_e539_d_b43, eq27_e539_d_b44, eq27_e539_d_b45, eq27_e539_d_b46, eq27_e539_d_b47, eq27_e539_d_b48, eq27_e539_d_b49, eq27_e539_d_b50, eq27_e539_d_b51, eq27_e539_d_b52, eq27_e539_d_b53, eq27_e539_d_b54,) = {
    if (s.b[390] && (!((s.b[387] || s.b[388]) || s.b[389]))) {
        let eq27_e536: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, (nv5 - 0.0));let eq27_e537: f64 = (s.v[149] * eq27_e536);let eq27_e537_d_n5: f64 = ((s.dn[149][5] * eq27_e536) + (s.v[149] * ddt_scale));
        (eq27_e537, (s.dn[149][0] * eq27_e536), (s.dn[149][1] * eq27_e536), (s.dn[149][2] * eq27_e536), (s.dn[149][3] * eq27_e536), (s.dn[149][4] * eq27_e536), eq27_e537_d_n5, (s.dn[149][6] * eq27_e536), (s.dn[149][7] * eq27_e536), (s.dn[149][8] * eq27_e536), (s.dn[149][9] * eq27_e536), (s.dn[149][10] * eq27_e536), (s.dn[149][11] * eq27_e536), (s.dn[149][12] * eq27_e536), (s.dn[149][13] * eq27_e536), (s.dn[149][14] * eq27_e536), (s.dn[149][15] * eq27_e536), (s.dn[149][16] * eq27_e536), (s.dn[149][17] * eq27_e536), (s.dn[149][18] * eq27_e536), (s.dn[149][19] * eq27_e536), (s.dn[149][20] * eq27_e536), (s.dn[149][21] * eq27_e536), (s.dn[149][22] * eq27_e536), (s.db[149][0] * eq27_e536), (s.db[149][1] * eq27_e536), (s.db[149][2] * eq27_e536), (s.db[149][3] * eq27_e536), (s.db[149][4] * eq27_e536), (s.db[149][5] * eq27_e536), (s.db[149][6] * eq27_e536), (s.db[149][7] * eq27_e536), (s.db[149][8] * eq27_e536), (s.db[149][9] * eq27_e536), (s.db[149][10] * eq27_e536), (s.db[149][11] * eq27_e536), (s.db[149][12] * eq27_e536), (s.db[149][13] * eq27_e536), (s.db[149][14] * eq27_e536), (s.db[149][15] * eq27_e536), (s.db[149][16] * eq27_e536), (s.db[149][17] * eq27_e536), (s.db[149][18] * eq27_e536), (s.db[149][19] * eq27_e536), (s.db[149][20] * eq27_e536), (s.db[149][21] * eq27_e536), (s.db[149][22] * eq27_e536), (s.db[149][23] * eq27_e536), (s.db[149][24] * eq27_e536), (s.db[149][25] * eq27_e536), (s.db[149][26] * eq27_e536), (s.db[149][27] * eq27_e536), (s.db[149][28] * eq27_e536), (s.db[149][29] * eq27_e536), (s.db[149][30] * eq27_e536), (s.db[149][31] * eq27_e536), (s.db[149][32] * eq27_e536), (s.db[149][33] * eq27_e536), (s.db[149][34] * eq27_e536), (s.db[149][35] * eq27_e536), (s.db[149][36] * eq27_e536), (s.db[149][37] * eq27_e536), (s.db[149][38] * eq27_e536), (s.db[149][39] * eq27_e536), (s.db[149][40] * eq27_e536), (s.db[149][41] * eq27_e536), (s.db[149][42] * eq27_e536), (s.db[149][43] * eq27_e536), (s.db[149][44] * eq27_e536), (s.db[149][45] * eq27_e536), (s.db[149][46] * eq27_e536), (s.db[149][47] * eq27_e536), (s.db[149][48] * eq27_e536), (s.db[149][49] * eq27_e536), (s.db[149][50] * eq27_e536), (s.db[149][51] * eq27_e536), (s.db[149][52] * eq27_e536), (s.db[149][53] * eq27_e536), (s.db[149][54] * eq27_e536),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e539;let eq27_node_derivatives: [f64; 23] = [eq27_e539_d_n0, eq27_e539_d_n1, eq27_e539_d_n2, eq27_e539_d_n3, eq27_e539_d_n4, eq27_e539_d_n5, eq27_e539_d_n6, eq27_e539_d_n7, eq27_e539_d_n8, eq27_e539_d_n9, eq27_e539_d_n10, eq27_e539_d_n11, eq27_e539_d_n12, eq27_e539_d_n13, eq27_e539_d_n14, eq27_e539_d_n15, eq27_e539_d_n16, eq27_e539_d_n17, eq27_e539_d_n18, eq27_e539_d_n19, eq27_e539_d_n20, eq27_e539_d_n21, eq27_e539_d_n22];let eq27_branch_derivatives: [f64; 55] = [eq27_e539_d_b0, eq27_e539_d_b1, eq27_e539_d_b2, eq27_e539_d_b3, eq27_e539_d_b4, eq27_e539_d_b5, eq27_e539_d_b6, eq27_e539_d_b7, eq27_e539_d_b8, eq27_e539_d_b9, eq27_e539_d_b10, eq27_e539_d_b11, eq27_e539_d_b12, eq27_e539_d_b13, eq27_e539_d_b14, eq27_e539_d_b15, eq27_e539_d_b16, eq27_e539_d_b17, eq27_e539_d_b18, eq27_e539_d_b19, eq27_e539_d_b20, eq27_e539_d_b21, eq27_e539_d_b22, eq27_e539_d_b23, eq27_e539_d_b24, eq27_e539_d_b25, eq27_e539_d_b26, eq27_e539_d_b27, eq27_e539_d_b28, eq27_e539_d_b29, eq27_e539_d_b30, eq27_e539_d_b31, eq27_e539_d_b32, eq27_e539_d_b33, eq27_e539_d_b34, eq27_e539_d_b35, eq27_e539_d_b36, eq27_e539_d_b37, eq27_e539_d_b38, eq27_e539_d_b39, eq27_e539_d_b40, eq27_e539_d_b41, eq27_e539_d_b42, eq27_e539_d_b43, eq27_e539_d_b44, eq27_e539_d_b45, eq27_e539_d_b46, eq27_e539_d_b47, eq27_e539_d_b48, eq27_e539_d_b49, eq27_e539_d_b50, eq27_e539_d_b51, eq27_e539_d_b52, eq27_e539_d_b53, eq27_e539_d_b54];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let (eq28_e550,) = {
    if (s.b[390] && (!((s.b[387] || s.b[388]) || s.b[389]))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq28_value: f64 = eq28_e550;
        stamper.stamp_potential_const_local(
            16,
            eq28_value,
        );
        let (eq29_e561,) = {
    if (s.b[390] && (!((s.b[387] || s.b[388]) || s.b[389]))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e561;
        stamper.stamp_potential_const_local(
            17,
            eq29_value,
        );
        let (eq30_e572,) = {
    if (s.b[390] && (!((s.b[387] || s.b[388]) || s.b[389]))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq30_value: f64 = eq30_e572;
        stamper.stamp_potential_const_local(
            18,
            eq30_value,
        );
        let (eq31_e583,) = {
    if (s.b[390] && (!((s.b[387] || s.b[388]) || s.b[389]))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq31_value: f64 = eq31_e583;
        stamper.stamp_potential_const_local(
            19,
            eq31_value,
        );
        let (eq32_e594,) = {
    if (s.b[390] && (!((s.b[387] || s.b[388]) || s.b[389]))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e594;
        stamper.stamp_potential_const_local(
            20,
            eq32_value,
        );
        let (eq33_e607,) = {
    if (s.b[391] && (!(((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq33_value: f64 = eq33_e607;
        stamper.stamp_potential_const_local(
            21,
            eq33_value,
        );
        let (eq34_e620,) = {
    if (s.b[391] && (!(((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq34_value: f64 = eq34_e620;
        stamper.stamp_potential_const_local(
            22,
            eq34_value,
        );
        let (eq35_e633, eq35_e633_d_n0, eq35_e633_d_n1, eq35_e633_d_n2, eq35_e633_d_n3, eq35_e633_d_n4, eq35_e633_d_n5, eq35_e633_d_n6, eq35_e633_d_n7, eq35_e633_d_n8, eq35_e633_d_n9, eq35_e633_d_n10, eq35_e633_d_n11, eq35_e633_d_n12, eq35_e633_d_n13, eq35_e633_d_n14, eq35_e633_d_n15, eq35_e633_d_n16, eq35_e633_d_n17, eq35_e633_d_n18, eq35_e633_d_n19, eq35_e633_d_n20, eq35_e633_d_n21, eq35_e633_d_n22, eq35_e633_d_b0, eq35_e633_d_b1, eq35_e633_d_b2, eq35_e633_d_b3, eq35_e633_d_b4, eq35_e633_d_b5, eq35_e633_d_b6, eq35_e633_d_b7, eq35_e633_d_b8, eq35_e633_d_b9, eq35_e633_d_b10, eq35_e633_d_b11, eq35_e633_d_b12, eq35_e633_d_b13, eq35_e633_d_b14, eq35_e633_d_b15, eq35_e633_d_b16, eq35_e633_d_b17, eq35_e633_d_b18, eq35_e633_d_b19, eq35_e633_d_b20, eq35_e633_d_b21, eq35_e633_d_b22, eq35_e633_d_b23, eq35_e633_d_b24, eq35_e633_d_b25, eq35_e633_d_b26, eq35_e633_d_b27, eq35_e633_d_b28, eq35_e633_d_b29, eq35_e633_d_b30, eq35_e633_d_b31, eq35_e633_d_b32, eq35_e633_d_b33, eq35_e633_d_b34, eq35_e633_d_b35, eq35_e633_d_b36, eq35_e633_d_b37, eq35_e633_d_b38, eq35_e633_d_b39, eq35_e633_d_b40, eq35_e633_d_b41, eq35_e633_d_b42, eq35_e633_d_b43, eq35_e633_d_b44, eq35_e633_d_b45, eq35_e633_d_b46, eq35_e633_d_b47, eq35_e633_d_b48, eq35_e633_d_b49, eq35_e633_d_b50, eq35_e633_d_b51, eq35_e633_d_b52, eq35_e633_d_b53, eq35_e633_d_b54,) = {
    if (s.b[391] && (!(((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]))) {
        (s.v[136], s.dn[136][0], s.dn[136][1], s.dn[136][2], s.dn[136][3], s.dn[136][4], s.dn[136][5], s.dn[136][6], s.dn[136][7], s.dn[136][8], s.dn[136][9], s.dn[136][10], s.dn[136][11], s.dn[136][12], s.dn[136][13], s.dn[136][14], s.dn[136][15], s.dn[136][16], s.dn[136][17], s.dn[136][18], s.dn[136][19], s.dn[136][20], s.dn[136][21], s.dn[136][22], s.db[136][0], s.db[136][1], s.db[136][2], s.db[136][3], s.db[136][4], s.db[136][5], s.db[136][6], s.db[136][7], s.db[136][8], s.db[136][9], s.db[136][10], s.db[136][11], s.db[136][12], s.db[136][13], s.db[136][14], s.db[136][15], s.db[136][16], s.db[136][17], s.db[136][18], s.db[136][19], s.db[136][20], s.db[136][21], s.db[136][22], s.db[136][23], s.db[136][24], s.db[136][25], s.db[136][26], s.db[136][27], s.db[136][28], s.db[136][29], s.db[136][30], s.db[136][31], s.db[136][32], s.db[136][33], s.db[136][34], s.db[136][35], s.db[136][36], s.db[136][37], s.db[136][38], s.db[136][39], s.db[136][40], s.db[136][41], s.db[136][42], s.db[136][43], s.db[136][44], s.db[136][45], s.db[136][46], s.db[136][47], s.db[136][48], s.db[136][49], s.db[136][50], s.db[136][51], s.db[136][52], s.db[136][53], s.db[136][54],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e633;let eq35_node_derivatives: [f64; 23] = [eq35_e633_d_n0, eq35_e633_d_n1, eq35_e633_d_n2, eq35_e633_d_n3, eq35_e633_d_n4, eq35_e633_d_n5, eq35_e633_d_n6, eq35_e633_d_n7, eq35_e633_d_n8, eq35_e633_d_n9, eq35_e633_d_n10, eq35_e633_d_n11, eq35_e633_d_n12, eq35_e633_d_n13, eq35_e633_d_n14, eq35_e633_d_n15, eq35_e633_d_n16, eq35_e633_d_n17, eq35_e633_d_n18, eq35_e633_d_n19, eq35_e633_d_n20, eq35_e633_d_n21, eq35_e633_d_n22];let eq35_branch_derivatives: [f64; 55] = [eq35_e633_d_b0, eq35_e633_d_b1, eq35_e633_d_b2, eq35_e633_d_b3, eq35_e633_d_b4, eq35_e633_d_b5, eq35_e633_d_b6, eq35_e633_d_b7, eq35_e633_d_b8, eq35_e633_d_b9, eq35_e633_d_b10, eq35_e633_d_b11, eq35_e633_d_b12, eq35_e633_d_b13, eq35_e633_d_b14, eq35_e633_d_b15, eq35_e633_d_b16, eq35_e633_d_b17, eq35_e633_d_b18, eq35_e633_d_b19, eq35_e633_d_b20, eq35_e633_d_b21, eq35_e633_d_b22, eq35_e633_d_b23, eq35_e633_d_b24, eq35_e633_d_b25, eq35_e633_d_b26, eq35_e633_d_b27, eq35_e633_d_b28, eq35_e633_d_b29, eq35_e633_d_b30, eq35_e633_d_b31, eq35_e633_d_b32, eq35_e633_d_b33, eq35_e633_d_b34, eq35_e633_d_b35, eq35_e633_d_b36, eq35_e633_d_b37, eq35_e633_d_b38, eq35_e633_d_b39, eq35_e633_d_b40, eq35_e633_d_b41, eq35_e633_d_b42, eq35_e633_d_b43, eq35_e633_d_b44, eq35_e633_d_b45, eq35_e633_d_b46, eq35_e633_d_b47, eq35_e633_d_b48, eq35_e633_d_b49, eq35_e633_d_b50, eq35_e633_d_b51, eq35_e633_d_b52, eq35_e633_d_b53, eq35_e633_d_b54];
        stamper.stamp_potential_dense_local(
            23,
            eq35_value,
            &eq35_node_derivatives,
            &eq35_branch_derivatives,
        );
    }
}
