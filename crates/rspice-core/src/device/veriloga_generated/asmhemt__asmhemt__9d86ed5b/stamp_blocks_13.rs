#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

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
        var_ct: f64,
        var_ct_dn0: f64,
        var_ct_dn1: f64,
        var_ct_dn12: f64,
        var_ct_dn14: f64,
        var_ct_dn15: f64,
        var_ct_dn16: f64,
        var_ct_dn17: f64,
        var_ct_dn18: f64,
        var_ct_dn19: f64,
        var_ct_dn2: f64,
        var_ct_dn20: f64,
        var_ct_dn21: f64,
        var_ct_dn22: f64,
        var_ct_dn3: f64,
        var_ct_dn4: f64,
        var_ct_dn5: f64,
        var_ct_dn6: f64,
        var_ct_dn7: f64,
        var_ct_dn8: f64,
        var_ct_dn9: f64,
        var_en: f64,
        var_en1: f64,
        var_en1_dn4: f64,
        var_en_dn4: f64,
        var_gdsmin_t: f64,
        var_gdsmin_t_dn4: f64,
        var_guard353: f64,
        var_guard354: f64,
        var_guard355: f64,
        var_guard356: f64,
        var_guard357: f64,
        var_guard358: f64,
        var_guard389: f64,
        var_ids: f64,
        var_ids_dn0: f64,
        var_ids_dn1: f64,
        var_ids_dn12: f64,
        var_ids_dn14: f64,
        var_ids_dn15: f64,
        var_ids_dn16: f64,
        var_ids_dn17: f64,
        var_ids_dn18: f64,
        var_ids_dn19: f64,
        var_ids_dn2: f64,
        var_ids_dn20: f64,
        var_ids_dn21: f64,
        var_ids_dn22: f64,
        var_ids_dn3: f64,
        var_ids_dn4: f64,
        var_ids_dn5: f64,
        var_ids_dn6: f64,
        var_ids_dn7: f64,
        var_ids_dn8: f64,
        var_ids_dn9: f64,
        var_igs_1: f64,
        var_igs_1_dn0: f64,
        var_igs_1_dn1: f64,
        var_igs_1_dn12: f64,
        var_igs_1_dn14: f64,
        var_igs_1_dn15: f64,
        var_igs_1_dn16: f64,
        var_igs_1_dn17: f64,
        var_igs_1_dn18: f64,
        var_igs_1_dn19: f64,
        var_igs_1_dn2: f64,
        var_igs_1_dn20: f64,
        var_igs_1_dn21: f64,
        var_igs_1_dn22: f64,
        var_igs_1_dn3: f64,
        var_igs_1_dn4: f64,
        var_igs_1_dn5: f64,
        var_igs_1_dn6: f64,
        var_igs_1_dn7: f64,
        var_igs_1_dn8: f64,
        var_igs_1_dn9: f64,
        var_isl: f64,
        var_isl_dn0: f64,
        var_isl_dn2: f64,
        var_isl_dn3: f64,
        var_isl_dn4: f64,
        var_isl_dn7: f64,
        var_isl_dn8: f64,
        var_isl_dn9: f64,
        var_phixn: f64,
        var_phixn_dn0: f64,
        var_phixn_dn1: f64,
        var_phixn_dn2: f64,
        var_phiyn: f64,
        var_phiyn_dn0: f64,
        var_phiyn_dn1: f64,
        var_phiyn_dn2: f64,
        var_sigvds: f64,
        var_t0: f64,
        var_t0_dn0: f64,
        var_t0_dn1: f64,
        var_t0_dn12: f64,
        var_t0_dn14: f64,
        var_t0_dn15: f64,
        var_t0_dn16: f64,
        var_t0_dn17: f64,
        var_t0_dn18: f64,
        var_t0_dn19: f64,
        var_t0_dn2: f64,
        var_t0_dn20: f64,
        var_t0_dn21: f64,
        var_t0_dn22: f64,
        var_t0_dn3: f64,
        var_t0_dn4: f64,
        var_t0_dn5: f64,
        var_t0_dn6: f64,
        var_t0_dn7: f64,
        var_t0_dn8: f64,
        var_t0_dn9: f64,
        var_t1: f64,
        var_t1_dn0: f64,
        var_t1_dn1: f64,
        var_t1_dn12: f64,
        var_t1_dn14: f64,
        var_t1_dn15: f64,
        var_t1_dn16: f64,
        var_t1_dn17: f64,
        var_t1_dn18: f64,
        var_t1_dn19: f64,
        var_t1_dn2: f64,
        var_t1_dn20: f64,
        var_t1_dn21: f64,
        var_t1_dn22: f64,
        var_t1_dn3: f64,
        var_t1_dn4: f64,
        var_t1_dn5: f64,
        var_t1_dn6: f64,
        var_t1_dn7: f64,
        var_t1_dn8: f64,
        var_t1_dn9: f64,
        var_vdgeff1: f64,
        var_vdgeff1_dn1: f64,
        var_vdgeff1_dn2: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv14 = ctx.node_voltage(nodes[14]);
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
        let (eq16_e415, eq16_e415_d_n1, eq16_e415_d_n2,) = {
    if ((var_guard355 != 0.0) && (!((var_guard353 != 0.0) || (var_guard354 != 0.0)))) {
        let eq16_e411: f64 = (-1.0);
        let eq16_e413: f64 = (eq16_e411 * var_vdgeff1);
        let eq16_e413_d_n1: f64 = (eq16_e411 * var_vdgeff1_dn1);
        let eq16_e413_d_n2: f64 = (eq16_e411 * var_vdgeff1_dn2);
        (eq16_e413, eq16_e413_d_n1, eq16_e413_d_n2,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e415;
        stamper.stamp_current_node2_local(
            Some(5),
            None,
            multiplicity * (eq16_value),
            1,
            multiplicity * (eq16_e415_d_n1),
            2,
            multiplicity * (eq16_e415_d_n2),
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
        let (eq27_e539, eq27_e539_d_n0, eq27_e539_d_n1, eq27_e539_d_n2, eq27_e539_d_n3, eq27_e539_d_n4, eq27_e539_d_n5, eq27_e539_d_n6, eq27_e539_d_n7, eq27_e539_d_n8, eq27_e539_d_n9, eq27_e539_d_n12, eq27_e539_d_n14, eq27_e539_d_n15, eq27_e539_d_n16, eq27_e539_d_n17, eq27_e539_d_n18, eq27_e539_d_n19, eq27_e539_d_n20, eq27_e539_d_n21, eq27_e539_d_n22,) = {
    if ((var_guard356 != 0.0) && (!(((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)))) {
        let eq27_e536: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, (nv5 - 0.0));
        let eq27_e537: f64 = (var_ct * eq27_e536);
        let eq27_e537_d_n0: f64 = (var_ct_dn0 * eq27_e536);
        let eq27_e537_d_n1: f64 = (var_ct_dn1 * eq27_e536);
        let eq27_e537_d_n2: f64 = (var_ct_dn2 * eq27_e536);
        let eq27_e537_d_n3: f64 = (var_ct_dn3 * eq27_e536);
        let eq27_e537_d_n4: f64 = (var_ct_dn4 * eq27_e536);
        let eq27_e537_d_n5: f64 = ((var_ct_dn5 * eq27_e536) + (var_ct * ddt_scale));
        let eq27_e537_d_n6: f64 = (var_ct_dn6 * eq27_e536);
        let eq27_e537_d_n7: f64 = (var_ct_dn7 * eq27_e536);
        let eq27_e537_d_n8: f64 = (var_ct_dn8 * eq27_e536);
        let eq27_e537_d_n9: f64 = (var_ct_dn9 * eq27_e536);
        let eq27_e537_d_n12: f64 = (var_ct_dn12 * eq27_e536);
        let eq27_e537_d_n14: f64 = (var_ct_dn14 * eq27_e536);
        let eq27_e537_d_n15: f64 = (var_ct_dn15 * eq27_e536);
        let eq27_e537_d_n16: f64 = (var_ct_dn16 * eq27_e536);
        let eq27_e537_d_n17: f64 = (var_ct_dn17 * eq27_e536);
        let eq27_e537_d_n18: f64 = (var_ct_dn18 * eq27_e536);
        let eq27_e537_d_n19: f64 = (var_ct_dn19 * eq27_e536);
        let eq27_e537_d_n20: f64 = (var_ct_dn20 * eq27_e536);
        let eq27_e537_d_n21: f64 = (var_ct_dn21 * eq27_e536);
        let eq27_e537_d_n22: f64 = (var_ct_dn22 * eq27_e536);
        (eq27_e537, eq27_e537_d_n0, eq27_e537_d_n1, eq27_e537_d_n2, eq27_e537_d_n3, eq27_e537_d_n4, eq27_e537_d_n5, eq27_e537_d_n6, eq27_e537_d_n7, eq27_e537_d_n8, eq27_e537_d_n9, eq27_e537_d_n12, eq27_e537_d_n14, eq27_e537_d_n15, eq27_e537_d_n16, eq27_e537_d_n17, eq27_e537_d_n18, eq27_e537_d_n19, eq27_e537_d_n20, eq27_e537_d_n21, eq27_e537_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e539;
        let eq27_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq27_node_derivatives: [f64; 20] = [eq27_e539_d_n0, eq27_e539_d_n1, eq27_e539_d_n2, eq27_e539_d_n3, eq27_e539_d_n4, eq27_e539_d_n5, eq27_e539_d_n6, eq27_e539_d_n7, eq27_e539_d_n8, eq27_e539_d_n9, eq27_e539_d_n12, eq27_e539_d_n14, eq27_e539_d_n15, eq27_e539_d_n16, eq27_e539_d_n17, eq27_e539_d_n18, eq27_e539_d_n19, eq27_e539_d_n20, eq27_e539_d_n21, eq27_e539_d_n22];
        let eq27_branch_derivative_indices: [usize; 0] = [];
        let eq27_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            None,
            multiplicity * (eq27_value),
            &eq27_node_derivative_indices,
            &eq27_node_derivatives,
            &eq27_branch_derivative_indices,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let (eq35_e633, eq35_e633_d_n0, eq35_e633_d_n1, eq35_e633_d_n2, eq35_e633_d_n3, eq35_e633_d_n4, eq35_e633_d_n5, eq35_e633_d_n6, eq35_e633_d_n7, eq35_e633_d_n8, eq35_e633_d_n9, eq35_e633_d_n12, eq35_e633_d_n14, eq35_e633_d_n15, eq35_e633_d_n16, eq35_e633_d_n17, eq35_e633_d_n18, eq35_e633_d_n19, eq35_e633_d_n20, eq35_e633_d_n21, eq35_e633_d_n22,) = {
    if ((var_guard357 != 0.0) && (!((((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)) || (var_guard356 != 0.0)))) {
        (var_t0, var_t0_dn0, var_t0_dn1, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn12, var_t0_dn14, var_t0_dn15, var_t0_dn16, var_t0_dn17, var_t0_dn18, var_t0_dn19, var_t0_dn20, var_t0_dn21, var_t0_dn22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e633;
        let eq35_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq35_node_derivatives: [f64; 20] = [eq35_e633_d_n0, eq35_e633_d_n1, eq35_e633_d_n2, eq35_e633_d_n3, eq35_e633_d_n4, eq35_e633_d_n5, eq35_e633_d_n6, eq35_e633_d_n7, eq35_e633_d_n8, eq35_e633_d_n9, eq35_e633_d_n12, eq35_e633_d_n14, eq35_e633_d_n15, eq35_e633_d_n16, eq35_e633_d_n17, eq35_e633_d_n18, eq35_e633_d_n19, eq35_e633_d_n20, eq35_e633_d_n21, eq35_e633_d_n22];
        let eq35_branch_derivative_indices: [usize; 0] = [];
        let eq35_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            23,
            eq35_value,
            &eq35_node_derivative_indices,
            &eq35_node_derivatives,
            &eq35_branch_derivative_indices,
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
        let (eq38_e681, eq38_e681_d_n0, eq38_e681_d_n1, eq38_e681_d_n2, eq38_e681_d_n3, eq38_e681_d_n4, eq38_e681_d_n5, eq38_e681_d_n6, eq38_e681_d_n7, eq38_e681_d_n8, eq38_e681_d_n9, eq38_e681_d_n12, eq38_e681_d_n14, eq38_e681_d_n15, eq38_e681_d_n16, eq38_e681_d_n17, eq38_e681_d_n18, eq38_e681_d_n19, eq38_e681_d_n20, eq38_e681_d_n21, eq38_e681_d_n22,) = {
    if ((var_guard357 != 0.0) && (!((((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)) || (var_guard356 != 0.0)))) {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn12, var_t1_dn14, var_t1_dn15, var_t1_dn16, var_t1_dn17, var_t1_dn18, var_t1_dn19, var_t1_dn20, var_t1_dn21, var_t1_dn22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e681;
        let eq38_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq38_node_derivatives: [f64; 20] = [eq38_e681_d_n0, eq38_e681_d_n1, eq38_e681_d_n2, eq38_e681_d_n3, eq38_e681_d_n4, eq38_e681_d_n5, eq38_e681_d_n6, eq38_e681_d_n7, eq38_e681_d_n8, eq38_e681_d_n9, eq38_e681_d_n12, eq38_e681_d_n14, eq38_e681_d_n15, eq38_e681_d_n16, eq38_e681_d_n17, eq38_e681_d_n18, eq38_e681_d_n19, eq38_e681_d_n20, eq38_e681_d_n21, eq38_e681_d_n22];
        let eq38_branch_derivative_indices: [usize; 0] = [];
        let eq38_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            24,
            eq38_value,
            &eq38_node_derivative_indices,
            &eq38_node_derivatives,
            &eq38_branch_derivative_indices,
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
        let (eq41_e747, eq41_e747_d_n0, eq41_e747_d_n1, eq41_e747_d_n2, eq41_e747_d_n4, eq41_e747_d_n5,) = {
    if ((var_guard358 != 0.0) && (!(((((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)) || (var_guard356 != 0.0)) || (var_guard357 != 0.0)))) {
        let eq41_e730: f64 = (-p.p135);
        let eq41_e732: f64 = (eq41_e730 * var_en);
        let eq41_e732_d_n4: f64 = (eq41_e730 * var_en_dn4);
        let eq41_e735: f64 = (p.p136 - (nv5 - 0.0));
        let eq41_e736: f64 = (eq41_e732 * eq41_e735);
        let eq41_e736_d_n4: f64 = (eq41_e732_d_n4 * eq41_e735);
        let eq41_e736_d_n5: f64 = (eq41_e732 * (-1.0));
        let eq41_e739: f64 = (2.0 * var_phixn);
        let eq41_e739_d_n0: f64 = (2.0 * var_phixn_dn0);
        let eq41_e739_d_n1: f64 = (2.0 * var_phixn_dn1);
        let eq41_e739_d_n2: f64 = (2.0 * var_phixn_dn2);
        let eq41_e740: f64 = (eq41_e739).exp();
        let eq41_e740_d_n0: f64 = (eq41_e740 * eq41_e739_d_n0);
        let eq41_e740_d_n1: f64 = (eq41_e740 * eq41_e739_d_n1);
        let eq41_e740_d_n2: f64 = (eq41_e740 * eq41_e739_d_n2);
        let eq41_e742: f64 = (eq41_e740 - 1.0);
        let eq41_e743: f64 = (eq41_e736 * eq41_e742);
        let eq41_e743_d_n0: f64 = (eq41_e736 * eq41_e740_d_n0);
        let eq41_e743_d_n1: f64 = (eq41_e736 * eq41_e740_d_n1);
        let eq41_e743_d_n2: f64 = (eq41_e736 * eq41_e740_d_n2);
        let eq41_e743_d_n4: f64 = (eq41_e736_d_n4 * eq41_e742);
        let eq41_e743_d_n5: f64 = (eq41_e736_d_n5 * eq41_e742);
        let eq41_e745: f64 = (eq41_e743 * 0.5);
        let eq41_e745_d_n0: f64 = (eq41_e743_d_n0 * 0.5);
        let eq41_e745_d_n1: f64 = (eq41_e743_d_n1 * 0.5);
        let eq41_e745_d_n2: f64 = (eq41_e743_d_n2 * 0.5);
        let eq41_e745_d_n4: f64 = (eq41_e743_d_n4 * 0.5);
        let eq41_e745_d_n5: f64 = (eq41_e743_d_n5 * 0.5);
        (eq41_e745, eq41_e745_d_n0, eq41_e745_d_n1, eq41_e745_d_n2, eq41_e745_d_n4, eq41_e745_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_value: f64 = eq41_e747;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            None,
            multiplicity * (eq41_value),
            [0, 1, 2, 4, 5],
            [multiplicity * (eq41_e747_d_n0), multiplicity * (eq41_e747_d_n1), multiplicity * (eq41_e747_d_n2), multiplicity * (eq41_e747_d_n4), multiplicity * (eq41_e747_d_n5)],
            [],
            [],
            1.0,
        );
        let (eq42_e766, eq42_e766_d_n4, eq42_e766_d_n5,) = {
    if ((var_guard358 != 0.0) && (!(((((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)) || (var_guard356 != 0.0)) || (var_guard357 != 0.0)))) {
        let eq42_e762: f64 = (p.p135 * var_en);
        let eq42_e762_d_n4: f64 = (p.p135 * var_en_dn4);
        let eq42_e764: f64 = (eq42_e762 * (nv5 - 0.0));
        let eq42_e764_d_n4: f64 = (eq42_e762_d_n4 * (nv5 - 0.0));
        (eq42_e764, eq42_e764_d_n4, eq42_e762,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq42_value: f64 = eq42_e766;
        stamper.stamp_current_node2_local(
            Some(5),
            None,
            multiplicity * (eq42_value),
            4,
            multiplicity * (eq42_e766_d_n4),
            5,
            multiplicity * (eq42_e766_d_n5),
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
        let (eq44_e815, eq44_e815_d_n0, eq44_e815_d_n1, eq44_e815_d_n2, eq44_e815_d_n4, eq44_e815_d_n6,) = {
    if ((var_guard358 != 0.0) && (!(((((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)) || (var_guard356 != 0.0)) || (var_guard357 != 0.0)))) {
        let eq44_e798: f64 = (-p.p144);
        let eq44_e800: f64 = (eq44_e798 * var_en1);
        let eq44_e800_d_n4: f64 = (eq44_e798 * var_en1_dn4);
        let eq44_e803: f64 = (p.p145 - (nv6 - 0.0));
        let eq44_e804: f64 = (eq44_e800 * eq44_e803);
        let eq44_e804_d_n4: f64 = (eq44_e800_d_n4 * eq44_e803);
        let eq44_e804_d_n6: f64 = (eq44_e800 * (-1.0));
        let eq44_e807: f64 = (2.0 * var_phiyn);
        let eq44_e807_d_n0: f64 = (2.0 * var_phiyn_dn0);
        let eq44_e807_d_n1: f64 = (2.0 * var_phiyn_dn1);
        let eq44_e807_d_n2: f64 = (2.0 * var_phiyn_dn2);
        let eq44_e808: f64 = (eq44_e807).exp();
        let eq44_e808_d_n0: f64 = (eq44_e808 * eq44_e807_d_n0);
        let eq44_e808_d_n1: f64 = (eq44_e808 * eq44_e807_d_n1);
        let eq44_e808_d_n2: f64 = (eq44_e808 * eq44_e807_d_n2);
        let eq44_e810: f64 = (eq44_e808 - 1.0);
        let eq44_e811: f64 = (eq44_e804 * eq44_e810);
        let eq44_e811_d_n0: f64 = (eq44_e804 * eq44_e808_d_n0);
        let eq44_e811_d_n1: f64 = (eq44_e804 * eq44_e808_d_n1);
        let eq44_e811_d_n2: f64 = (eq44_e804 * eq44_e808_d_n2);
        let eq44_e811_d_n4: f64 = (eq44_e804_d_n4 * eq44_e810);
        let eq44_e811_d_n6: f64 = (eq44_e804_d_n6 * eq44_e810);
        let eq44_e813: f64 = (eq44_e811 * 0.5);
        let eq44_e813_d_n0: f64 = (eq44_e811_d_n0 * 0.5);
        let eq44_e813_d_n1: f64 = (eq44_e811_d_n1 * 0.5);
        let eq44_e813_d_n2: f64 = (eq44_e811_d_n2 * 0.5);
        let eq44_e813_d_n4: f64 = (eq44_e811_d_n4 * 0.5);
        let eq44_e813_d_n6: f64 = (eq44_e811_d_n6 * 0.5);
        (eq44_e813, eq44_e813_d_n0, eq44_e813_d_n1, eq44_e813_d_n2, eq44_e813_d_n4, eq44_e813_d_n6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e815;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            None,
            multiplicity * (eq44_value),
            [0, 1, 2, 4, 6],
            [multiplicity * (eq44_e815_d_n0), multiplicity * (eq44_e815_d_n1), multiplicity * (eq44_e815_d_n2), multiplicity * (eq44_e815_d_n4), multiplicity * (eq44_e815_d_n6)],
            [],
            [],
            1.0,
        );
        let (eq45_e834, eq45_e834_d_n4, eq45_e834_d_n6,) = {
    if ((var_guard358 != 0.0) && (!(((((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)) || (var_guard356 != 0.0)) || (var_guard357 != 0.0)))) {
        let eq45_e830: f64 = (p.p144 * var_en1);
        let eq45_e830_d_n4: f64 = (p.p144 * var_en1_dn4);
        let eq45_e832: f64 = (eq45_e830 * (nv6 - 0.0));
        let eq45_e832_d_n4: f64 = (eq45_e830_d_n4 * (nv6 - 0.0));
        (eq45_e832, eq45_e832_d_n4, eq45_e830,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq45_value: f64 = eq45_e834;
        stamper.stamp_current_node2_local(
            Some(6),
            None,
            multiplicity * (eq45_value),
            4,
            multiplicity * (eq45_e834_d_n4),
            6,
            multiplicity * (eq45_e834_d_n6),
        );
        let (eq46_e852, eq46_e852_d_n6,) = {
    if ((var_guard358 != 0.0) && (!(((((var_guard353 != 0.0) || (var_guard354 != 0.0)) || (var_guard355 != 0.0)) || (var_guard356 != 0.0)) || (var_guard357 != 0.0)))) {
        let eq46_e849: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, (nv6 - 0.0));
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
        let eq51_e915: f64 = (p.p6 * var_sigvds);
        let eq51_e917: f64 = (eq51_e915 * var_ids);
        let eq51_e917_d_n0: f64 = (eq51_e915 * var_ids_dn0);
        let eq51_e917_d_n1: f64 = (eq51_e915 * var_ids_dn1);
        let eq51_e917_d_n2: f64 = (eq51_e915 * var_ids_dn2);
        let eq51_e917_d_n3: f64 = (eq51_e915 * var_ids_dn3);
        let eq51_e917_d_n4: f64 = (eq51_e915 * var_ids_dn4);
        let eq51_e917_d_n5: f64 = (eq51_e915 * var_ids_dn5);
        let eq51_e917_d_n6: f64 = (eq51_e915 * var_ids_dn6);
        let eq51_e917_d_n7: f64 = (eq51_e915 * var_ids_dn7);
        let eq51_e917_d_n8: f64 = (eq51_e915 * var_ids_dn8);
        let eq51_e917_d_n9: f64 = (eq51_e915 * var_ids_dn9);
        let eq51_e917_d_n12: f64 = (eq51_e915 * var_ids_dn12);
        let eq51_e917_d_n14: f64 = (eq51_e915 * var_ids_dn14);
        let eq51_e917_d_n15: f64 = (eq51_e915 * var_ids_dn15);
        let eq51_e917_d_n16: f64 = (eq51_e915 * var_ids_dn16);
        let eq51_e917_d_n17: f64 = (eq51_e915 * var_ids_dn17);
        let eq51_e917_d_n18: f64 = (eq51_e915 * var_ids_dn18);
        let eq51_e917_d_n19: f64 = (eq51_e915 * var_ids_dn19);
        let eq51_e917_d_n20: f64 = (eq51_e915 * var_ids_dn20);
        let eq51_e917_d_n21: f64 = (eq51_e915 * var_ids_dn21);
        let eq51_e917_d_n22: f64 = (eq51_e915 * var_ids_dn22);
        let eq51_e920: f64 = (p.p6 * var_gdsmin_t);
        let eq51_e920_d_n4: f64 = (p.p6 * var_gdsmin_t_dn4);
        let eq51_e922: f64 = (eq51_e920 * (nv7 - nv8));
        let eq51_e922_d_n4: f64 = (eq51_e920_d_n4 * (nv7 - nv8));
        let eq51_e923: f64 = (eq51_e917 + eq51_e922);
        let eq51_e923_d_n4: f64 = (eq51_e917_d_n4 + eq51_e922_d_n4);
        let eq51_e923_d_n7: f64 = (eq51_e917_d_n7 + eq51_e920);
        let eq51_e923_d_n8: f64 = (eq51_e917_d_n8 + (-eq51_e920));
        let eq51_value: f64 = eq51_e923;
        let eq51_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq51_node_derivatives: [f64; 20] = [eq51_e917_d_n0, eq51_e917_d_n1, eq51_e917_d_n2, eq51_e917_d_n3, eq51_e923_d_n4, eq51_e917_d_n5, eq51_e917_d_n6, eq51_e923_d_n7, eq51_e923_d_n8, eq51_e917_d_n9, eq51_e917_d_n12, eq51_e917_d_n14, eq51_e917_d_n15, eq51_e917_d_n16, eq51_e917_d_n17, eq51_e917_d_n18, eq51_e917_d_n19, eq51_e917_d_n20, eq51_e917_d_n21, eq51_e917_d_n22];
        let eq51_branch_derivative_indices: [usize; 0] = [];
        let eq51_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq51_value),
            &eq51_node_derivative_indices,
            &eq51_node_derivatives,
            &eq51_branch_derivative_indices,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let eq52_e926: f64 = (p.p6 * var_sigvds);
        let eq52_e929: f64 = (p.p4 * p.p5);
        let eq52_e931: f64 = (eq52_e929 * var_isl);
        let eq52_e931_d_n0: f64 = (eq52_e929 * var_isl_dn0);
        let eq52_e931_d_n2: f64 = (eq52_e929 * var_isl_dn2);
        let eq52_e931_d_n3: f64 = (eq52_e929 * var_isl_dn3);
        let eq52_e931_d_n4: f64 = (eq52_e929 * var_isl_dn4);
        let eq52_e931_d_n7: f64 = (eq52_e929 * var_isl_dn7);
        let eq52_e931_d_n8: f64 = (eq52_e929 * var_isl_dn8);
        let eq52_e931_d_n9: f64 = (eq52_e929 * var_isl_dn9);
        let eq52_e932: f64 = (eq52_e926 * eq52_e931);
        let eq52_e932_d_n0: f64 = (eq52_e926 * eq52_e931_d_n0);
        let eq52_e932_d_n2: f64 = (eq52_e926 * eq52_e931_d_n2);
        let eq52_e932_d_n3: f64 = (eq52_e926 * eq52_e931_d_n3);
        let eq52_e932_d_n4: f64 = (eq52_e926 * eq52_e931_d_n4);
        let eq52_e932_d_n7: f64 = (eq52_e926 * eq52_e931_d_n7);
        let eq52_e932_d_n8: f64 = (eq52_e926 * eq52_e931_d_n8);
        let eq52_e932_d_n9: f64 = (eq52_e926 * eq52_e931_d_n9);
        let eq52_value: f64 = eq52_e932;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(0),
            Some(2),
            multiplicity * (eq52_value),
            [0, 2, 3, 4, 7, 8, 9],
            [multiplicity * (eq52_e932_d_n0), multiplicity * (eq52_e932_d_n2), multiplicity * (eq52_e932_d_n3), multiplicity * (eq52_e932_d_n4), multiplicity * (eq52_e932_d_n7), multiplicity * (eq52_e932_d_n8), multiplicity * (eq52_e932_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq53_e938, eq53_e938_d_n0, eq53_e938_d_n1, eq53_e938_d_n2, eq53_e938_d_n3, eq53_e938_d_n4, eq53_e938_d_n5, eq53_e938_d_n6, eq53_e938_d_n7, eq53_e938_d_n8, eq53_e938_d_n9, eq53_e938_d_n12, eq53_e938_d_n14, eq53_e938_d_n15, eq53_e938_d_n16, eq53_e938_d_n17, eq53_e938_d_n18, eq53_e938_d_n19, eq53_e938_d_n20, eq53_e938_d_n21, eq53_e938_d_n22,) = {
    if (var_guard389 != 0.0) {
        let eq53_e936: f64 = (p.p6 * var_igs_1);
        let eq53_e936_d_n0: f64 = (p.p6 * var_igs_1_dn0);
        let eq53_e936_d_n1: f64 = (p.p6 * var_igs_1_dn1);
        let eq53_e936_d_n2: f64 = (p.p6 * var_igs_1_dn2);
        let eq53_e936_d_n3: f64 = (p.p6 * var_igs_1_dn3);
        let eq53_e936_d_n4: f64 = (p.p6 * var_igs_1_dn4);
        let eq53_e936_d_n5: f64 = (p.p6 * var_igs_1_dn5);
        let eq53_e936_d_n6: f64 = (p.p6 * var_igs_1_dn6);
        let eq53_e936_d_n7: f64 = (p.p6 * var_igs_1_dn7);
        let eq53_e936_d_n8: f64 = (p.p6 * var_igs_1_dn8);
        let eq53_e936_d_n9: f64 = (p.p6 * var_igs_1_dn9);
        let eq53_e936_d_n12: f64 = (p.p6 * var_igs_1_dn12);
        let eq53_e936_d_n14: f64 = (p.p6 * var_igs_1_dn14);
        let eq53_e936_d_n15: f64 = (p.p6 * var_igs_1_dn15);
        let eq53_e936_d_n16: f64 = (p.p6 * var_igs_1_dn16);
        let eq53_e936_d_n17: f64 = (p.p6 * var_igs_1_dn17);
        let eq53_e936_d_n18: f64 = (p.p6 * var_igs_1_dn18);
        let eq53_e936_d_n19: f64 = (p.p6 * var_igs_1_dn19);
        let eq53_e936_d_n20: f64 = (p.p6 * var_igs_1_dn20);
        let eq53_e936_d_n21: f64 = (p.p6 * var_igs_1_dn21);
        let eq53_e936_d_n22: f64 = (p.p6 * var_igs_1_dn22);
        (eq53_e936, eq53_e936_d_n0, eq53_e936_d_n1, eq53_e936_d_n2, eq53_e936_d_n3, eq53_e936_d_n4, eq53_e936_d_n5, eq53_e936_d_n6, eq53_e936_d_n7, eq53_e936_d_n8, eq53_e936_d_n9, eq53_e936_d_n12, eq53_e936_d_n14, eq53_e936_d_n15, eq53_e936_d_n16, eq53_e936_d_n17, eq53_e936_d_n18, eq53_e936_d_n19, eq53_e936_d_n20, eq53_e936_d_n21, eq53_e936_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e938;
        let eq53_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq53_node_derivatives: [f64; 20] = [eq53_e938_d_n0, eq53_e938_d_n1, eq53_e938_d_n2, eq53_e938_d_n3, eq53_e938_d_n4, eq53_e938_d_n5, eq53_e938_d_n6, eq53_e938_d_n7, eq53_e938_d_n8, eq53_e938_d_n9, eq53_e938_d_n12, eq53_e938_d_n14, eq53_e938_d_n15, eq53_e938_d_n16, eq53_e938_d_n17, eq53_e938_d_n18, eq53_e938_d_n19, eq53_e938_d_n20, eq53_e938_d_n21, eq53_e938_d_n22];
        let eq53_branch_derivative_indices: [usize; 0] = [];
        let eq53_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq53_value),
            &eq53_node_derivative_indices,
            &eq53_node_derivatives,
            &eq53_branch_derivative_indices,
            &eq53_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        var_gdpr: f64,
        var_gdpr_dn0: f64,
        var_gdpr_dn1: f64,
        var_gdpr_dn12: f64,
        var_gdpr_dn14: f64,
        var_gdpr_dn15: f64,
        var_gdpr_dn16: f64,
        var_gdpr_dn17: f64,
        var_gdpr_dn18: f64,
        var_gdpr_dn19: f64,
        var_gdpr_dn2: f64,
        var_gdpr_dn20: f64,
        var_gdpr_dn21: f64,
        var_gdpr_dn22: f64,
        var_gdpr_dn3: f64,
        var_gdpr_dn4: f64,
        var_gdpr_dn5: f64,
        var_gdpr_dn6: f64,
        var_gdpr_dn7: f64,
        var_gdpr_dn8: f64,
        var_gdpr_dn9: f64,
        var_gdsmin_t: f64,
        var_gdsmin_t_dn4: f64,
        var_gspr: f64,
        var_gspr_dn0: f64,
        var_gspr_dn1: f64,
        var_gspr_dn12: f64,
        var_gspr_dn14: f64,
        var_gspr_dn15: f64,
        var_gspr_dn16: f64,
        var_gspr_dn17: f64,
        var_gspr_dn18: f64,
        var_gspr_dn19: f64,
        var_gspr_dn2: f64,
        var_gspr_dn20: f64,
        var_gspr_dn21: f64,
        var_gspr_dn22: f64,
        var_gspr_dn3: f64,
        var_gspr_dn4: f64,
        var_gspr_dn5: f64,
        var_gspr_dn6: f64,
        var_gspr_dn7: f64,
        var_gspr_dn8: f64,
        var_gspr_dn9: f64,
        var_guard389: f64,
        var_guard390: f64,
        var_guard393: f64,
        var_guard399: f64,
        var_guard400: f64,
        var_guard414: f64,
        var_guard415: f64,
        var_ids_fp1: f64,
        var_ids_fp1_dn0: f64,
        var_ids_fp1_dn1: f64,
        var_ids_fp1_dn12: f64,
        var_ids_fp1_dn14: f64,
        var_ids_fp1_dn15: f64,
        var_ids_fp1_dn16: f64,
        var_ids_fp1_dn17: f64,
        var_ids_fp1_dn18: f64,
        var_ids_fp1_dn19: f64,
        var_ids_fp1_dn2: f64,
        var_ids_fp1_dn20: f64,
        var_ids_fp1_dn21: f64,
        var_ids_fp1_dn22: f64,
        var_ids_fp1_dn3: f64,
        var_ids_fp1_dn4: f64,
        var_ids_fp1_dn5: f64,
        var_ids_fp1_dn6: f64,
        var_ids_fp1_dn7: f64,
        var_ids_fp1_dn8: f64,
        var_ids_fp1_dn9: f64,
        var_ids_fp1s: f64,
        var_ids_fp1s_dn0: f64,
        var_ids_fp1s_dn1: f64,
        var_ids_fp1s_dn12: f64,
        var_ids_fp1s_dn14: f64,
        var_ids_fp1s_dn15: f64,
        var_ids_fp1s_dn16: f64,
        var_ids_fp1s_dn17: f64,
        var_ids_fp1s_dn18: f64,
        var_ids_fp1s_dn19: f64,
        var_ids_fp1s_dn2: f64,
        var_ids_fp1s_dn20: f64,
        var_ids_fp1s_dn21: f64,
        var_ids_fp1s_dn22: f64,
        var_ids_fp1s_dn3: f64,
        var_ids_fp1s_dn4: f64,
        var_ids_fp1s_dn5: f64,
        var_ids_fp1s_dn6: f64,
        var_ids_fp1s_dn7: f64,
        var_ids_fp1s_dn8: f64,
        var_ids_fp1s_dn9: f64,
        var_igd_1: f64,
        var_igd_1_dn0: f64,
        var_igd_1_dn1: f64,
        var_igd_1_dn12: f64,
        var_igd_1_dn14: f64,
        var_igd_1_dn15: f64,
        var_igd_1_dn16: f64,
        var_igd_1_dn17: f64,
        var_igd_1_dn18: f64,
        var_igd_1_dn19: f64,
        var_igd_1_dn2: f64,
        var_igd_1_dn20: f64,
        var_igd_1_dn21: f64,
        var_igd_1_dn22: f64,
        var_igd_1_dn3: f64,
        var_igd_1_dn4: f64,
        var_igd_1_dn5: f64,
        var_igd_1_dn6: f64,
        var_igd_1_dn7: f64,
        var_igd_1_dn8: f64,
        var_igd_1_dn9: f64,
        var_igs_1: f64,
        var_igs_1_dn0: f64,
        var_igs_1_dn1: f64,
        var_igs_1_dn12: f64,
        var_igs_1_dn14: f64,
        var_igs_1_dn15: f64,
        var_igs_1_dn16: f64,
        var_igs_1_dn17: f64,
        var_igs_1_dn18: f64,
        var_igs_1_dn19: f64,
        var_igs_1_dn2: f64,
        var_igs_1_dn20: f64,
        var_igs_1_dn21: f64,
        var_igs_1_dn22: f64,
        var_igs_1_dn3: f64,
        var_igs_1_dn4: f64,
        var_igs_1_dn5: f64,
        var_igs_1_dn6: f64,
        var_igs_1_dn7: f64,
        var_igs_1_dn8: f64,
        var_igs_1_dn9: f64,
        var_sigvdsfp1: f64,
        var_sigvdsfp1s: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let nv19 = ctx.node_voltage(nodes[19]);
        let nv22 = ctx.node_voltage(nodes[22]);
        let (eq54_e944, eq54_e944_d_n0, eq54_e944_d_n1, eq54_e944_d_n2, eq54_e944_d_n3, eq54_e944_d_n4, eq54_e944_d_n5, eq54_e944_d_n6, eq54_e944_d_n7, eq54_e944_d_n8, eq54_e944_d_n9, eq54_e944_d_n12, eq54_e944_d_n14, eq54_e944_d_n15, eq54_e944_d_n16, eq54_e944_d_n17, eq54_e944_d_n18, eq54_e944_d_n19, eq54_e944_d_n20, eq54_e944_d_n21, eq54_e944_d_n22,) = {
    if (var_guard389 != 0.0) {
        let eq54_e942: f64 = (p.p6 * var_igd_1);
        let eq54_e942_d_n0: f64 = (p.p6 * var_igd_1_dn0);
        let eq54_e942_d_n1: f64 = (p.p6 * var_igd_1_dn1);
        let eq54_e942_d_n2: f64 = (p.p6 * var_igd_1_dn2);
        let eq54_e942_d_n3: f64 = (p.p6 * var_igd_1_dn3);
        let eq54_e942_d_n4: f64 = (p.p6 * var_igd_1_dn4);
        let eq54_e942_d_n5: f64 = (p.p6 * var_igd_1_dn5);
        let eq54_e942_d_n6: f64 = (p.p6 * var_igd_1_dn6);
        let eq54_e942_d_n7: f64 = (p.p6 * var_igd_1_dn7);
        let eq54_e942_d_n8: f64 = (p.p6 * var_igd_1_dn8);
        let eq54_e942_d_n9: f64 = (p.p6 * var_igd_1_dn9);
        let eq54_e942_d_n12: f64 = (p.p6 * var_igd_1_dn12);
        let eq54_e942_d_n14: f64 = (p.p6 * var_igd_1_dn14);
        let eq54_e942_d_n15: f64 = (p.p6 * var_igd_1_dn15);
        let eq54_e942_d_n16: f64 = (p.p6 * var_igd_1_dn16);
        let eq54_e942_d_n17: f64 = (p.p6 * var_igd_1_dn17);
        let eq54_e942_d_n18: f64 = (p.p6 * var_igd_1_dn18);
        let eq54_e942_d_n19: f64 = (p.p6 * var_igd_1_dn19);
        let eq54_e942_d_n20: f64 = (p.p6 * var_igd_1_dn20);
        let eq54_e942_d_n21: f64 = (p.p6 * var_igd_1_dn21);
        let eq54_e942_d_n22: f64 = (p.p6 * var_igd_1_dn22);
        (eq54_e942, eq54_e942_d_n0, eq54_e942_d_n1, eq54_e942_d_n2, eq54_e942_d_n3, eq54_e942_d_n4, eq54_e942_d_n5, eq54_e942_d_n6, eq54_e942_d_n7, eq54_e942_d_n8, eq54_e942_d_n9, eq54_e942_d_n12, eq54_e942_d_n14, eq54_e942_d_n15, eq54_e942_d_n16, eq54_e942_d_n17, eq54_e942_d_n18, eq54_e942_d_n19, eq54_e942_d_n20, eq54_e942_d_n21, eq54_e942_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e944;
        let eq54_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq54_node_derivatives: [f64; 20] = [eq54_e944_d_n0, eq54_e944_d_n1, eq54_e944_d_n2, eq54_e944_d_n3, eq54_e944_d_n4, eq54_e944_d_n5, eq54_e944_d_n6, eq54_e944_d_n7, eq54_e944_d_n8, eq54_e944_d_n9, eq54_e944_d_n12, eq54_e944_d_n14, eq54_e944_d_n15, eq54_e944_d_n16, eq54_e944_d_n17, eq54_e944_d_n18, eq54_e944_d_n19, eq54_e944_d_n20, eq54_e944_d_n21, eq54_e944_d_n22];
        let eq54_branch_derivative_indices: [usize; 0] = [];
        let eq54_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq54_value),
            &eq54_node_derivative_indices,
            &eq54_node_derivatives,
            &eq54_branch_derivative_indices,
            &eq54_branch_derivatives,
            multiplicity,
        );
        let (eq55_e957, eq55_e957_d_n0, eq55_e957_d_n1, eq55_e957_d_n2, eq55_e957_d_n3, eq55_e957_d_n4, eq55_e957_d_n5, eq55_e957_d_n6, eq55_e957_d_n7, eq55_e957_d_n8, eq55_e957_d_n9, eq55_e957_d_n12, eq55_e957_d_n14, eq55_e957_d_n15, eq55_e957_d_n16, eq55_e957_d_n17, eq55_e957_d_n18, eq55_e957_d_n19, eq55_e957_d_n20, eq55_e957_d_n21, eq55_e957_d_n22,) = {
    if (var_guard389 == 0.0) {
        let eq55_e951: f64 = 0.0;
        let eq55_e953: f64 = (eq55_e951 * (nv9 - nv8));
        let eq55_e954: f64 = (var_igs_1 + eq55_e953);
        let eq55_e954_d_n8: f64 = (var_igs_1_dn8 + (-eq55_e951));
        let eq55_e954_d_n9: f64 = (var_igs_1_dn9 + eq55_e951);
        let eq55_e955: f64 = (p.p6 * eq55_e954);
        let eq55_e955_d_n0: f64 = (p.p6 * var_igs_1_dn0);
        let eq55_e955_d_n1: f64 = (p.p6 * var_igs_1_dn1);
        let eq55_e955_d_n2: f64 = (p.p6 * var_igs_1_dn2);
        let eq55_e955_d_n3: f64 = (p.p6 * var_igs_1_dn3);
        let eq55_e955_d_n4: f64 = (p.p6 * var_igs_1_dn4);
        let eq55_e955_d_n5: f64 = (p.p6 * var_igs_1_dn5);
        let eq55_e955_d_n6: f64 = (p.p6 * var_igs_1_dn6);
        let eq55_e955_d_n7: f64 = (p.p6 * var_igs_1_dn7);
        let eq55_e955_d_n8: f64 = (p.p6 * eq55_e954_d_n8);
        let eq55_e955_d_n9: f64 = (p.p6 * eq55_e954_d_n9);
        let eq55_e955_d_n12: f64 = (p.p6 * var_igs_1_dn12);
        let eq55_e955_d_n14: f64 = (p.p6 * var_igs_1_dn14);
        let eq55_e955_d_n15: f64 = (p.p6 * var_igs_1_dn15);
        let eq55_e955_d_n16: f64 = (p.p6 * var_igs_1_dn16);
        let eq55_e955_d_n17: f64 = (p.p6 * var_igs_1_dn17);
        let eq55_e955_d_n18: f64 = (p.p6 * var_igs_1_dn18);
        let eq55_e955_d_n19: f64 = (p.p6 * var_igs_1_dn19);
        let eq55_e955_d_n20: f64 = (p.p6 * var_igs_1_dn20);
        let eq55_e955_d_n21: f64 = (p.p6 * var_igs_1_dn21);
        let eq55_e955_d_n22: f64 = (p.p6 * var_igs_1_dn22);
        (eq55_e955, eq55_e955_d_n0, eq55_e955_d_n1, eq55_e955_d_n2, eq55_e955_d_n3, eq55_e955_d_n4, eq55_e955_d_n5, eq55_e955_d_n6, eq55_e955_d_n7, eq55_e955_d_n8, eq55_e955_d_n9, eq55_e955_d_n12, eq55_e955_d_n14, eq55_e955_d_n15, eq55_e955_d_n16, eq55_e955_d_n17, eq55_e955_d_n18, eq55_e955_d_n19, eq55_e955_d_n20, eq55_e955_d_n21, eq55_e955_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e957;
        let eq55_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq55_node_derivatives: [f64; 20] = [eq55_e957_d_n0, eq55_e957_d_n1, eq55_e957_d_n2, eq55_e957_d_n3, eq55_e957_d_n4, eq55_e957_d_n5, eq55_e957_d_n6, eq55_e957_d_n7, eq55_e957_d_n8, eq55_e957_d_n9, eq55_e957_d_n12, eq55_e957_d_n14, eq55_e957_d_n15, eq55_e957_d_n16, eq55_e957_d_n17, eq55_e957_d_n18, eq55_e957_d_n19, eq55_e957_d_n20, eq55_e957_d_n21, eq55_e957_d_n22];
        let eq55_branch_derivative_indices: [usize; 0] = [];
        let eq55_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq55_value),
            &eq55_node_derivative_indices,
            &eq55_node_derivatives,
            &eq55_branch_derivative_indices,
            &eq55_branch_derivatives,
            multiplicity,
        );
        let (eq56_e970, eq56_e970_d_n0, eq56_e970_d_n1, eq56_e970_d_n2, eq56_e970_d_n3, eq56_e970_d_n4, eq56_e970_d_n5, eq56_e970_d_n6, eq56_e970_d_n7, eq56_e970_d_n8, eq56_e970_d_n9, eq56_e970_d_n12, eq56_e970_d_n14, eq56_e970_d_n15, eq56_e970_d_n16, eq56_e970_d_n17, eq56_e970_d_n18, eq56_e970_d_n19, eq56_e970_d_n20, eq56_e970_d_n21, eq56_e970_d_n22,) = {
    if (var_guard389 == 0.0) {
        let eq56_e964: f64 = 0.0;
        let eq56_e966: f64 = (eq56_e964 * (nv9 - nv7));
        let eq56_e967: f64 = (var_igd_1 + eq56_e966);
        let eq56_e967_d_n7: f64 = (var_igd_1_dn7 + (-eq56_e964));
        let eq56_e967_d_n9: f64 = (var_igd_1_dn9 + eq56_e964);
        let eq56_e968: f64 = (p.p6 * eq56_e967);
        let eq56_e968_d_n0: f64 = (p.p6 * var_igd_1_dn0);
        let eq56_e968_d_n1: f64 = (p.p6 * var_igd_1_dn1);
        let eq56_e968_d_n2: f64 = (p.p6 * var_igd_1_dn2);
        let eq56_e968_d_n3: f64 = (p.p6 * var_igd_1_dn3);
        let eq56_e968_d_n4: f64 = (p.p6 * var_igd_1_dn4);
        let eq56_e968_d_n5: f64 = (p.p6 * var_igd_1_dn5);
        let eq56_e968_d_n6: f64 = (p.p6 * var_igd_1_dn6);
        let eq56_e968_d_n7: f64 = (p.p6 * eq56_e967_d_n7);
        let eq56_e968_d_n8: f64 = (p.p6 * var_igd_1_dn8);
        let eq56_e968_d_n9: f64 = (p.p6 * eq56_e967_d_n9);
        let eq56_e968_d_n12: f64 = (p.p6 * var_igd_1_dn12);
        let eq56_e968_d_n14: f64 = (p.p6 * var_igd_1_dn14);
        let eq56_e968_d_n15: f64 = (p.p6 * var_igd_1_dn15);
        let eq56_e968_d_n16: f64 = (p.p6 * var_igd_1_dn16);
        let eq56_e968_d_n17: f64 = (p.p6 * var_igd_1_dn17);
        let eq56_e968_d_n18: f64 = (p.p6 * var_igd_1_dn18);
        let eq56_e968_d_n19: f64 = (p.p6 * var_igd_1_dn19);
        let eq56_e968_d_n20: f64 = (p.p6 * var_igd_1_dn20);
        let eq56_e968_d_n21: f64 = (p.p6 * var_igd_1_dn21);
        let eq56_e968_d_n22: f64 = (p.p6 * var_igd_1_dn22);
        (eq56_e968, eq56_e968_d_n0, eq56_e968_d_n1, eq56_e968_d_n2, eq56_e968_d_n3, eq56_e968_d_n4, eq56_e968_d_n5, eq56_e968_d_n6, eq56_e968_d_n7, eq56_e968_d_n8, eq56_e968_d_n9, eq56_e968_d_n12, eq56_e968_d_n14, eq56_e968_d_n15, eq56_e968_d_n16, eq56_e968_d_n17, eq56_e968_d_n18, eq56_e968_d_n19, eq56_e968_d_n20, eq56_e968_d_n21, eq56_e968_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_value: f64 = eq56_e970;
        let eq56_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq56_node_derivatives: [f64; 20] = [eq56_e970_d_n0, eq56_e970_d_n1, eq56_e970_d_n2, eq56_e970_d_n3, eq56_e970_d_n4, eq56_e970_d_n5, eq56_e970_d_n6, eq56_e970_d_n7, eq56_e970_d_n8, eq56_e970_d_n9, eq56_e970_d_n12, eq56_e970_d_n14, eq56_e970_d_n15, eq56_e970_d_n16, eq56_e970_d_n17, eq56_e970_d_n18, eq56_e970_d_n19, eq56_e970_d_n20, eq56_e970_d_n21, eq56_e970_d_n22];
        let eq56_branch_derivative_indices: [usize; 0] = [];
        let eq56_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq56_value),
            &eq56_node_derivative_indices,
            &eq56_node_derivatives,
            &eq56_branch_derivative_indices,
            &eq56_branch_derivatives,
            multiplicity,
        );
        let (eq57_e980, eq57_e980_d_n0, eq57_e980_d_n1, eq57_e980_d_n2, eq57_e980_d_n3, eq57_e980_d_n4, eq57_e980_d_n5, eq57_e980_d_n6, eq57_e980_d_n7, eq57_e980_d_n8, eq57_e980_d_n9, eq57_e980_d_n12, eq57_e980_d_n14, eq57_e980_d_n15, eq57_e980_d_n16, eq57_e980_d_n17, eq57_e980_d_n18, eq57_e980_d_n19, eq57_e980_d_n20, eq57_e980_d_n21, eq57_e980_d_n22,) = {
    if ((var_guard390 != 0.0) && (var_guard393 != 0.0)) {
        let eq57_e976: f64 = (p.p6 * var_gdpr);
        let eq57_e976_d_n0: f64 = (p.p6 * var_gdpr_dn0);
        let eq57_e976_d_n1: f64 = (p.p6 * var_gdpr_dn1);
        let eq57_e976_d_n2: f64 = (p.p6 * var_gdpr_dn2);
        let eq57_e976_d_n3: f64 = (p.p6 * var_gdpr_dn3);
        let eq57_e976_d_n4: f64 = (p.p6 * var_gdpr_dn4);
        let eq57_e976_d_n5: f64 = (p.p6 * var_gdpr_dn5);
        let eq57_e976_d_n6: f64 = (p.p6 * var_gdpr_dn6);
        let eq57_e976_d_n7: f64 = (p.p6 * var_gdpr_dn7);
        let eq57_e976_d_n8: f64 = (p.p6 * var_gdpr_dn8);
        let eq57_e976_d_n9: f64 = (p.p6 * var_gdpr_dn9);
        let eq57_e976_d_n12: f64 = (p.p6 * var_gdpr_dn12);
        let eq57_e976_d_n14: f64 = (p.p6 * var_gdpr_dn14);
        let eq57_e976_d_n15: f64 = (p.p6 * var_gdpr_dn15);
        let eq57_e976_d_n16: f64 = (p.p6 * var_gdpr_dn16);
        let eq57_e976_d_n17: f64 = (p.p6 * var_gdpr_dn17);
        let eq57_e976_d_n18: f64 = (p.p6 * var_gdpr_dn18);
        let eq57_e976_d_n19: f64 = (p.p6 * var_gdpr_dn19);
        let eq57_e976_d_n20: f64 = (p.p6 * var_gdpr_dn20);
        let eq57_e976_d_n21: f64 = (p.p6 * var_gdpr_dn21);
        let eq57_e976_d_n22: f64 = (p.p6 * var_gdpr_dn22);
        let eq57_e978: f64 = (eq57_e976 * (nv0 - nv18));
        let eq57_e978_d_n0: f64 = ((eq57_e976_d_n0 * (nv0 - nv18)) + eq57_e976);
        let eq57_e978_d_n1: f64 = (eq57_e976_d_n1 * (nv0 - nv18));
        let eq57_e978_d_n2: f64 = (eq57_e976_d_n2 * (nv0 - nv18));
        let eq57_e978_d_n3: f64 = (eq57_e976_d_n3 * (nv0 - nv18));
        let eq57_e978_d_n4: f64 = (eq57_e976_d_n4 * (nv0 - nv18));
        let eq57_e978_d_n5: f64 = (eq57_e976_d_n5 * (nv0 - nv18));
        let eq57_e978_d_n6: f64 = (eq57_e976_d_n6 * (nv0 - nv18));
        let eq57_e978_d_n7: f64 = (eq57_e976_d_n7 * (nv0 - nv18));
        let eq57_e978_d_n8: f64 = (eq57_e976_d_n8 * (nv0 - nv18));
        let eq57_e978_d_n9: f64 = (eq57_e976_d_n9 * (nv0 - nv18));
        let eq57_e978_d_n12: f64 = (eq57_e976_d_n12 * (nv0 - nv18));
        let eq57_e978_d_n14: f64 = (eq57_e976_d_n14 * (nv0 - nv18));
        let eq57_e978_d_n15: f64 = (eq57_e976_d_n15 * (nv0 - nv18));
        let eq57_e978_d_n16: f64 = (eq57_e976_d_n16 * (nv0 - nv18));
        let eq57_e978_d_n17: f64 = (eq57_e976_d_n17 * (nv0 - nv18));
        let eq57_e978_d_n18: f64 = ((eq57_e976_d_n18 * (nv0 - nv18)) + (-eq57_e976));
        let eq57_e978_d_n19: f64 = (eq57_e976_d_n19 * (nv0 - nv18));
        let eq57_e978_d_n20: f64 = (eq57_e976_d_n20 * (nv0 - nv18));
        let eq57_e978_d_n21: f64 = (eq57_e976_d_n21 * (nv0 - nv18));
        let eq57_e978_d_n22: f64 = (eq57_e976_d_n22 * (nv0 - nv18));
        (eq57_e978, eq57_e978_d_n0, eq57_e978_d_n1, eq57_e978_d_n2, eq57_e978_d_n3, eq57_e978_d_n4, eq57_e978_d_n5, eq57_e978_d_n6, eq57_e978_d_n7, eq57_e978_d_n8, eq57_e978_d_n9, eq57_e978_d_n12, eq57_e978_d_n14, eq57_e978_d_n15, eq57_e978_d_n16, eq57_e978_d_n17, eq57_e978_d_n18, eq57_e978_d_n19, eq57_e978_d_n20, eq57_e978_d_n21, eq57_e978_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e980;
        let eq57_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq57_node_derivatives: [f64; 20] = [eq57_e980_d_n0, eq57_e980_d_n1, eq57_e980_d_n2, eq57_e980_d_n3, eq57_e980_d_n4, eq57_e980_d_n5, eq57_e980_d_n6, eq57_e980_d_n7, eq57_e980_d_n8, eq57_e980_d_n9, eq57_e980_d_n12, eq57_e980_d_n14, eq57_e980_d_n15, eq57_e980_d_n16, eq57_e980_d_n17, eq57_e980_d_n18, eq57_e980_d_n19, eq57_e980_d_n20, eq57_e980_d_n21, eq57_e980_d_n22];
        let eq57_branch_derivative_indices: [usize; 0] = [];
        let eq57_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(18),
            multiplicity * (eq57_value),
            &eq57_node_derivative_indices,
            &eq57_node_derivatives,
            &eq57_branch_derivative_indices,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let (eq58_e990, eq58_e990_d_n0, eq58_e990_d_n1, eq58_e990_d_n2, eq58_e990_d_n3, eq58_e990_d_n4, eq58_e990_d_n5, eq58_e990_d_n6, eq58_e990_d_n7, eq58_e990_d_n8, eq58_e990_d_n9, eq58_e990_d_n12, eq58_e990_d_n14, eq58_e990_d_n15, eq58_e990_d_n16, eq58_e990_d_n17, eq58_e990_d_n18, eq58_e990_d_n19, eq58_e990_d_n20, eq58_e990_d_n21, eq58_e990_d_n22,) = {
    if ((var_guard390 != 0.0) && (var_guard393 != 0.0)) {
        let eq58_e986: f64 = (p.p6 * var_gspr);
        let eq58_e986_d_n0: f64 = (p.p6 * var_gspr_dn0);
        let eq58_e986_d_n1: f64 = (p.p6 * var_gspr_dn1);
        let eq58_e986_d_n2: f64 = (p.p6 * var_gspr_dn2);
        let eq58_e986_d_n3: f64 = (p.p6 * var_gspr_dn3);
        let eq58_e986_d_n4: f64 = (p.p6 * var_gspr_dn4);
        let eq58_e986_d_n5: f64 = (p.p6 * var_gspr_dn5);
        let eq58_e986_d_n6: f64 = (p.p6 * var_gspr_dn6);
        let eq58_e986_d_n7: f64 = (p.p6 * var_gspr_dn7);
        let eq58_e986_d_n8: f64 = (p.p6 * var_gspr_dn8);
        let eq58_e986_d_n9: f64 = (p.p6 * var_gspr_dn9);
        let eq58_e986_d_n12: f64 = (p.p6 * var_gspr_dn12);
        let eq58_e986_d_n14: f64 = (p.p6 * var_gspr_dn14);
        let eq58_e986_d_n15: f64 = (p.p6 * var_gspr_dn15);
        let eq58_e986_d_n16: f64 = (p.p6 * var_gspr_dn16);
        let eq58_e986_d_n17: f64 = (p.p6 * var_gspr_dn17);
        let eq58_e986_d_n18: f64 = (p.p6 * var_gspr_dn18);
        let eq58_e986_d_n19: f64 = (p.p6 * var_gspr_dn19);
        let eq58_e986_d_n20: f64 = (p.p6 * var_gspr_dn20);
        let eq58_e986_d_n21: f64 = (p.p6 * var_gspr_dn21);
        let eq58_e986_d_n22: f64 = (p.p6 * var_gspr_dn22);
        let eq58_e988: f64 = (eq58_e986 * (nv22 - nv2));
        let eq58_e988_d_n0: f64 = (eq58_e986_d_n0 * (nv22 - nv2));
        let eq58_e988_d_n1: f64 = (eq58_e986_d_n1 * (nv22 - nv2));
        let eq58_e988_d_n2: f64 = ((eq58_e986_d_n2 * (nv22 - nv2)) + (-eq58_e986));
        let eq58_e988_d_n3: f64 = (eq58_e986_d_n3 * (nv22 - nv2));
        let eq58_e988_d_n4: f64 = (eq58_e986_d_n4 * (nv22 - nv2));
        let eq58_e988_d_n5: f64 = (eq58_e986_d_n5 * (nv22 - nv2));
        let eq58_e988_d_n6: f64 = (eq58_e986_d_n6 * (nv22 - nv2));
        let eq58_e988_d_n7: f64 = (eq58_e986_d_n7 * (nv22 - nv2));
        let eq58_e988_d_n8: f64 = (eq58_e986_d_n8 * (nv22 - nv2));
        let eq58_e988_d_n9: f64 = (eq58_e986_d_n9 * (nv22 - nv2));
        let eq58_e988_d_n12: f64 = (eq58_e986_d_n12 * (nv22 - nv2));
        let eq58_e988_d_n14: f64 = (eq58_e986_d_n14 * (nv22 - nv2));
        let eq58_e988_d_n15: f64 = (eq58_e986_d_n15 * (nv22 - nv2));
        let eq58_e988_d_n16: f64 = (eq58_e986_d_n16 * (nv22 - nv2));
        let eq58_e988_d_n17: f64 = (eq58_e986_d_n17 * (nv22 - nv2));
        let eq58_e988_d_n18: f64 = (eq58_e986_d_n18 * (nv22 - nv2));
        let eq58_e988_d_n19: f64 = (eq58_e986_d_n19 * (nv22 - nv2));
        let eq58_e988_d_n20: f64 = (eq58_e986_d_n20 * (nv22 - nv2));
        let eq58_e988_d_n21: f64 = (eq58_e986_d_n21 * (nv22 - nv2));
        let eq58_e988_d_n22: f64 = ((eq58_e986_d_n22 * (nv22 - nv2)) + eq58_e986);
        (eq58_e988, eq58_e988_d_n0, eq58_e988_d_n1, eq58_e988_d_n2, eq58_e988_d_n3, eq58_e988_d_n4, eq58_e988_d_n5, eq58_e988_d_n6, eq58_e988_d_n7, eq58_e988_d_n8, eq58_e988_d_n9, eq58_e988_d_n12, eq58_e988_d_n14, eq58_e988_d_n15, eq58_e988_d_n16, eq58_e988_d_n17, eq58_e988_d_n18, eq58_e988_d_n19, eq58_e988_d_n20, eq58_e988_d_n21, eq58_e988_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e990;
        let eq58_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq58_node_derivatives: [f64; 20] = [eq58_e990_d_n0, eq58_e990_d_n1, eq58_e990_d_n2, eq58_e990_d_n3, eq58_e990_d_n4, eq58_e990_d_n5, eq58_e990_d_n6, eq58_e990_d_n7, eq58_e990_d_n8, eq58_e990_d_n9, eq58_e990_d_n12, eq58_e990_d_n14, eq58_e990_d_n15, eq58_e990_d_n16, eq58_e990_d_n17, eq58_e990_d_n18, eq58_e990_d_n19, eq58_e990_d_n20, eq58_e990_d_n21, eq58_e990_d_n22];
        let eq58_branch_derivative_indices: [usize; 0] = [];
        let eq58_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(22),
            Some(2),
            multiplicity * (eq58_value),
            &eq58_node_derivative_indices,
            &eq58_node_derivatives,
            &eq58_branch_derivative_indices,
            &eq58_branch_derivatives,
            multiplicity,
        );
        let (eq59_e1001, eq59_e1001_d_n0, eq59_e1001_d_n1, eq59_e1001_d_n2, eq59_e1001_d_n3, eq59_e1001_d_n4, eq59_e1001_d_n5, eq59_e1001_d_n6, eq59_e1001_d_n7, eq59_e1001_d_n8, eq59_e1001_d_n9, eq59_e1001_d_n12, eq59_e1001_d_n14, eq59_e1001_d_n15, eq59_e1001_d_n16, eq59_e1001_d_n17, eq59_e1001_d_n18, eq59_e1001_d_n19, eq59_e1001_d_n20, eq59_e1001_d_n21, eq59_e1001_d_n22,) = {
    if ((var_guard390 != 0.0) && (var_guard393 == 0.0)) {
        let eq59_e997: f64 = (p.p6 * var_gdpr);
        let eq59_e997_d_n0: f64 = (p.p6 * var_gdpr_dn0);
        let eq59_e997_d_n1: f64 = (p.p6 * var_gdpr_dn1);
        let eq59_e997_d_n2: f64 = (p.p6 * var_gdpr_dn2);
        let eq59_e997_d_n3: f64 = (p.p6 * var_gdpr_dn3);
        let eq59_e997_d_n4: f64 = (p.p6 * var_gdpr_dn4);
        let eq59_e997_d_n5: f64 = (p.p6 * var_gdpr_dn5);
        let eq59_e997_d_n6: f64 = (p.p6 * var_gdpr_dn6);
        let eq59_e997_d_n7: f64 = (p.p6 * var_gdpr_dn7);
        let eq59_e997_d_n8: f64 = (p.p6 * var_gdpr_dn8);
        let eq59_e997_d_n9: f64 = (p.p6 * var_gdpr_dn9);
        let eq59_e997_d_n12: f64 = (p.p6 * var_gdpr_dn12);
        let eq59_e997_d_n14: f64 = (p.p6 * var_gdpr_dn14);
        let eq59_e997_d_n15: f64 = (p.p6 * var_gdpr_dn15);
        let eq59_e997_d_n16: f64 = (p.p6 * var_gdpr_dn16);
        let eq59_e997_d_n17: f64 = (p.p6 * var_gdpr_dn17);
        let eq59_e997_d_n18: f64 = (p.p6 * var_gdpr_dn18);
        let eq59_e997_d_n19: f64 = (p.p6 * var_gdpr_dn19);
        let eq59_e997_d_n20: f64 = (p.p6 * var_gdpr_dn20);
        let eq59_e997_d_n21: f64 = (p.p6 * var_gdpr_dn21);
        let eq59_e997_d_n22: f64 = (p.p6 * var_gdpr_dn22);
        let eq59_e999: f64 = (eq59_e997 * (nv0 - nv7));
        let eq59_e999_d_n0: f64 = ((eq59_e997_d_n0 * (nv0 - nv7)) + eq59_e997);
        let eq59_e999_d_n1: f64 = (eq59_e997_d_n1 * (nv0 - nv7));
        let eq59_e999_d_n2: f64 = (eq59_e997_d_n2 * (nv0 - nv7));
        let eq59_e999_d_n3: f64 = (eq59_e997_d_n3 * (nv0 - nv7));
        let eq59_e999_d_n4: f64 = (eq59_e997_d_n4 * (nv0 - nv7));
        let eq59_e999_d_n5: f64 = (eq59_e997_d_n5 * (nv0 - nv7));
        let eq59_e999_d_n6: f64 = (eq59_e997_d_n6 * (nv0 - nv7));
        let eq59_e999_d_n7: f64 = ((eq59_e997_d_n7 * (nv0 - nv7)) + (-eq59_e997));
        let eq59_e999_d_n8: f64 = (eq59_e997_d_n8 * (nv0 - nv7));
        let eq59_e999_d_n9: f64 = (eq59_e997_d_n9 * (nv0 - nv7));
        let eq59_e999_d_n12: f64 = (eq59_e997_d_n12 * (nv0 - nv7));
        let eq59_e999_d_n14: f64 = (eq59_e997_d_n14 * (nv0 - nv7));
        let eq59_e999_d_n15: f64 = (eq59_e997_d_n15 * (nv0 - nv7));
        let eq59_e999_d_n16: f64 = (eq59_e997_d_n16 * (nv0 - nv7));
        let eq59_e999_d_n17: f64 = (eq59_e997_d_n17 * (nv0 - nv7));
        let eq59_e999_d_n18: f64 = (eq59_e997_d_n18 * (nv0 - nv7));
        let eq59_e999_d_n19: f64 = (eq59_e997_d_n19 * (nv0 - nv7));
        let eq59_e999_d_n20: f64 = (eq59_e997_d_n20 * (nv0 - nv7));
        let eq59_e999_d_n21: f64 = (eq59_e997_d_n21 * (nv0 - nv7));
        let eq59_e999_d_n22: f64 = (eq59_e997_d_n22 * (nv0 - nv7));
        (eq59_e999, eq59_e999_d_n0, eq59_e999_d_n1, eq59_e999_d_n2, eq59_e999_d_n3, eq59_e999_d_n4, eq59_e999_d_n5, eq59_e999_d_n6, eq59_e999_d_n7, eq59_e999_d_n8, eq59_e999_d_n9, eq59_e999_d_n12, eq59_e999_d_n14, eq59_e999_d_n15, eq59_e999_d_n16, eq59_e999_d_n17, eq59_e999_d_n18, eq59_e999_d_n19, eq59_e999_d_n20, eq59_e999_d_n21, eq59_e999_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e1001;
        let eq59_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq59_node_derivatives: [f64; 20] = [eq59_e1001_d_n0, eq59_e1001_d_n1, eq59_e1001_d_n2, eq59_e1001_d_n3, eq59_e1001_d_n4, eq59_e1001_d_n5, eq59_e1001_d_n6, eq59_e1001_d_n7, eq59_e1001_d_n8, eq59_e1001_d_n9, eq59_e1001_d_n12, eq59_e1001_d_n14, eq59_e1001_d_n15, eq59_e1001_d_n16, eq59_e1001_d_n17, eq59_e1001_d_n18, eq59_e1001_d_n19, eq59_e1001_d_n20, eq59_e1001_d_n21, eq59_e1001_d_n22];
        let eq59_branch_derivative_indices: [usize; 0] = [];
        let eq59_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq59_value),
            &eq59_node_derivative_indices,
            &eq59_node_derivatives,
            &eq59_branch_derivative_indices,
            &eq59_branch_derivatives,
            multiplicity,
        );
        let (eq60_e1012, eq60_e1012_d_n0, eq60_e1012_d_n1, eq60_e1012_d_n2, eq60_e1012_d_n3, eq60_e1012_d_n4, eq60_e1012_d_n5, eq60_e1012_d_n6, eq60_e1012_d_n7, eq60_e1012_d_n8, eq60_e1012_d_n9, eq60_e1012_d_n12, eq60_e1012_d_n14, eq60_e1012_d_n15, eq60_e1012_d_n16, eq60_e1012_d_n17, eq60_e1012_d_n18, eq60_e1012_d_n19, eq60_e1012_d_n20, eq60_e1012_d_n21, eq60_e1012_d_n22,) = {
    if ((var_guard390 != 0.0) && (var_guard393 == 0.0)) {
        let eq60_e1008: f64 = (p.p6 * var_gspr);
        let eq60_e1008_d_n0: f64 = (p.p6 * var_gspr_dn0);
        let eq60_e1008_d_n1: f64 = (p.p6 * var_gspr_dn1);
        let eq60_e1008_d_n2: f64 = (p.p6 * var_gspr_dn2);
        let eq60_e1008_d_n3: f64 = (p.p6 * var_gspr_dn3);
        let eq60_e1008_d_n4: f64 = (p.p6 * var_gspr_dn4);
        let eq60_e1008_d_n5: f64 = (p.p6 * var_gspr_dn5);
        let eq60_e1008_d_n6: f64 = (p.p6 * var_gspr_dn6);
        let eq60_e1008_d_n7: f64 = (p.p6 * var_gspr_dn7);
        let eq60_e1008_d_n8: f64 = (p.p6 * var_gspr_dn8);
        let eq60_e1008_d_n9: f64 = (p.p6 * var_gspr_dn9);
        let eq60_e1008_d_n12: f64 = (p.p6 * var_gspr_dn12);
        let eq60_e1008_d_n14: f64 = (p.p6 * var_gspr_dn14);
        let eq60_e1008_d_n15: f64 = (p.p6 * var_gspr_dn15);
        let eq60_e1008_d_n16: f64 = (p.p6 * var_gspr_dn16);
        let eq60_e1008_d_n17: f64 = (p.p6 * var_gspr_dn17);
        let eq60_e1008_d_n18: f64 = (p.p6 * var_gspr_dn18);
        let eq60_e1008_d_n19: f64 = (p.p6 * var_gspr_dn19);
        let eq60_e1008_d_n20: f64 = (p.p6 * var_gspr_dn20);
        let eq60_e1008_d_n21: f64 = (p.p6 * var_gspr_dn21);
        let eq60_e1008_d_n22: f64 = (p.p6 * var_gspr_dn22);
        let eq60_e1010: f64 = (eq60_e1008 * (nv8 - nv2));
        let eq60_e1010_d_n0: f64 = (eq60_e1008_d_n0 * (nv8 - nv2));
        let eq60_e1010_d_n1: f64 = (eq60_e1008_d_n1 * (nv8 - nv2));
        let eq60_e1010_d_n2: f64 = ((eq60_e1008_d_n2 * (nv8 - nv2)) + (-eq60_e1008));
        let eq60_e1010_d_n3: f64 = (eq60_e1008_d_n3 * (nv8 - nv2));
        let eq60_e1010_d_n4: f64 = (eq60_e1008_d_n4 * (nv8 - nv2));
        let eq60_e1010_d_n5: f64 = (eq60_e1008_d_n5 * (nv8 - nv2));
        let eq60_e1010_d_n6: f64 = (eq60_e1008_d_n6 * (nv8 - nv2));
        let eq60_e1010_d_n7: f64 = (eq60_e1008_d_n7 * (nv8 - nv2));
        let eq60_e1010_d_n8: f64 = ((eq60_e1008_d_n8 * (nv8 - nv2)) + eq60_e1008);
        let eq60_e1010_d_n9: f64 = (eq60_e1008_d_n9 * (nv8 - nv2));
        let eq60_e1010_d_n12: f64 = (eq60_e1008_d_n12 * (nv8 - nv2));
        let eq60_e1010_d_n14: f64 = (eq60_e1008_d_n14 * (nv8 - nv2));
        let eq60_e1010_d_n15: f64 = (eq60_e1008_d_n15 * (nv8 - nv2));
        let eq60_e1010_d_n16: f64 = (eq60_e1008_d_n16 * (nv8 - nv2));
        let eq60_e1010_d_n17: f64 = (eq60_e1008_d_n17 * (nv8 - nv2));
        let eq60_e1010_d_n18: f64 = (eq60_e1008_d_n18 * (nv8 - nv2));
        let eq60_e1010_d_n19: f64 = (eq60_e1008_d_n19 * (nv8 - nv2));
        let eq60_e1010_d_n20: f64 = (eq60_e1008_d_n20 * (nv8 - nv2));
        let eq60_e1010_d_n21: f64 = (eq60_e1008_d_n21 * (nv8 - nv2));
        let eq60_e1010_d_n22: f64 = (eq60_e1008_d_n22 * (nv8 - nv2));
        (eq60_e1010, eq60_e1010_d_n0, eq60_e1010_d_n1, eq60_e1010_d_n2, eq60_e1010_d_n3, eq60_e1010_d_n4, eq60_e1010_d_n5, eq60_e1010_d_n6, eq60_e1010_d_n7, eq60_e1010_d_n8, eq60_e1010_d_n9, eq60_e1010_d_n12, eq60_e1010_d_n14, eq60_e1010_d_n15, eq60_e1010_d_n16, eq60_e1010_d_n17, eq60_e1010_d_n18, eq60_e1010_d_n19, eq60_e1010_d_n20, eq60_e1010_d_n21, eq60_e1010_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e1012;
        let eq60_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq60_node_derivatives: [f64; 20] = [eq60_e1012_d_n0, eq60_e1012_d_n1, eq60_e1012_d_n2, eq60_e1012_d_n3, eq60_e1012_d_n4, eq60_e1012_d_n5, eq60_e1012_d_n6, eq60_e1012_d_n7, eq60_e1012_d_n8, eq60_e1012_d_n9, eq60_e1012_d_n12, eq60_e1012_d_n14, eq60_e1012_d_n15, eq60_e1012_d_n16, eq60_e1012_d_n17, eq60_e1012_d_n18, eq60_e1012_d_n19, eq60_e1012_d_n20, eq60_e1012_d_n21, eq60_e1012_d_n22];
        let eq60_branch_derivative_indices: [usize; 0] = [];
        let eq60_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(2),
            multiplicity * (eq60_value),
            &eq60_node_derivative_indices,
            &eq60_node_derivatives,
            &eq60_branch_derivative_indices,
            &eq60_branch_derivatives,
            multiplicity,
        );
        let (eq72_e1166, eq72_e1166_d_n0, eq72_e1166_d_n1, eq72_e1166_d_n2, eq72_e1166_d_n3, eq72_e1166_d_n4, eq72_e1166_d_n5, eq72_e1166_d_n6, eq72_e1166_d_n7, eq72_e1166_d_n8, eq72_e1166_d_n9, eq72_e1166_d_n12, eq72_e1166_d_n14, eq72_e1166_d_n15, eq72_e1166_d_n16, eq72_e1166_d_n17, eq72_e1166_d_n18, eq72_e1166_d_n19, eq72_e1166_d_n20, eq72_e1166_d_n21, eq72_e1166_d_n22,) = {
    if ((var_guard399 != 0.0) && (var_guard400 != 0.0)) {
        let eq72_e1156: f64 = (p.p6 * var_sigvdsfp1);
        let eq72_e1158: f64 = (eq72_e1156 * var_ids_fp1);
        let eq72_e1158_d_n0: f64 = (eq72_e1156 * var_ids_fp1_dn0);
        let eq72_e1158_d_n1: f64 = (eq72_e1156 * var_ids_fp1_dn1);
        let eq72_e1158_d_n2: f64 = (eq72_e1156 * var_ids_fp1_dn2);
        let eq72_e1158_d_n3: f64 = (eq72_e1156 * var_ids_fp1_dn3);
        let eq72_e1158_d_n4: f64 = (eq72_e1156 * var_ids_fp1_dn4);
        let eq72_e1158_d_n5: f64 = (eq72_e1156 * var_ids_fp1_dn5);
        let eq72_e1158_d_n6: f64 = (eq72_e1156 * var_ids_fp1_dn6);
        let eq72_e1158_d_n7: f64 = (eq72_e1156 * var_ids_fp1_dn7);
        let eq72_e1158_d_n8: f64 = (eq72_e1156 * var_ids_fp1_dn8);
        let eq72_e1158_d_n9: f64 = (eq72_e1156 * var_ids_fp1_dn9);
        let eq72_e1158_d_n12: f64 = (eq72_e1156 * var_ids_fp1_dn12);
        let eq72_e1158_d_n14: f64 = (eq72_e1156 * var_ids_fp1_dn14);
        let eq72_e1158_d_n15: f64 = (eq72_e1156 * var_ids_fp1_dn15);
        let eq72_e1158_d_n16: f64 = (eq72_e1156 * var_ids_fp1_dn16);
        let eq72_e1158_d_n17: f64 = (eq72_e1156 * var_ids_fp1_dn17);
        let eq72_e1158_d_n18: f64 = (eq72_e1156 * var_ids_fp1_dn18);
        let eq72_e1158_d_n19: f64 = (eq72_e1156 * var_ids_fp1_dn19);
        let eq72_e1158_d_n20: f64 = (eq72_e1156 * var_ids_fp1_dn20);
        let eq72_e1158_d_n21: f64 = (eq72_e1156 * var_ids_fp1_dn21);
        let eq72_e1158_d_n22: f64 = (eq72_e1156 * var_ids_fp1_dn22);
        let eq72_e1161: f64 = (p.p6 * var_gdsmin_t);
        let eq72_e1161_d_n4: f64 = (p.p6 * var_gdsmin_t_dn4);
        let eq72_e1163: f64 = (eq72_e1161 * (nv15 - nv7));
        let eq72_e1163_d_n4: f64 = (eq72_e1161_d_n4 * (nv15 - nv7));
        let eq72_e1164: f64 = (eq72_e1158 + eq72_e1163);
        let eq72_e1164_d_n4: f64 = (eq72_e1158_d_n4 + eq72_e1163_d_n4);
        let eq72_e1164_d_n7: f64 = (eq72_e1158_d_n7 + (-eq72_e1161));
        let eq72_e1164_d_n15: f64 = (eq72_e1158_d_n15 + eq72_e1161);
        (eq72_e1164, eq72_e1158_d_n0, eq72_e1158_d_n1, eq72_e1158_d_n2, eq72_e1158_d_n3, eq72_e1164_d_n4, eq72_e1158_d_n5, eq72_e1158_d_n6, eq72_e1164_d_n7, eq72_e1158_d_n8, eq72_e1158_d_n9, eq72_e1158_d_n12, eq72_e1158_d_n14, eq72_e1164_d_n15, eq72_e1158_d_n16, eq72_e1158_d_n17, eq72_e1158_d_n18, eq72_e1158_d_n19, eq72_e1158_d_n20, eq72_e1158_d_n21, eq72_e1158_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_value: f64 = eq72_e1166;
        let eq72_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq72_node_derivatives: [f64; 20] = [eq72_e1166_d_n0, eq72_e1166_d_n1, eq72_e1166_d_n2, eq72_e1166_d_n3, eq72_e1166_d_n4, eq72_e1166_d_n5, eq72_e1166_d_n6, eq72_e1166_d_n7, eq72_e1166_d_n8, eq72_e1166_d_n9, eq72_e1166_d_n12, eq72_e1166_d_n14, eq72_e1166_d_n15, eq72_e1166_d_n16, eq72_e1166_d_n17, eq72_e1166_d_n18, eq72_e1166_d_n19, eq72_e1166_d_n20, eq72_e1166_d_n21, eq72_e1166_d_n22];
        let eq72_branch_derivative_indices: [usize; 0] = [];
        let eq72_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(15),
            Some(7),
            multiplicity * (eq72_value),
            &eq72_node_derivative_indices,
            &eq72_node_derivatives,
            &eq72_branch_derivative_indices,
            &eq72_branch_derivatives,
            multiplicity,
        );
        let (eq75_e1194, eq75_e1194_d_n0, eq75_e1194_d_n1, eq75_e1194_d_n2, eq75_e1194_d_n3, eq75_e1194_d_n4, eq75_e1194_d_n5, eq75_e1194_d_n6, eq75_e1194_d_n7, eq75_e1194_d_n8, eq75_e1194_d_n9, eq75_e1194_d_n12, eq75_e1194_d_n14, eq75_e1194_d_n15, eq75_e1194_d_n16, eq75_e1194_d_n17, eq75_e1194_d_n18, eq75_e1194_d_n19, eq75_e1194_d_n20, eq75_e1194_d_n21, eq75_e1194_d_n22,) = {
    if ((var_guard414 != 0.0) && (var_guard415 != 0.0)) {
        let eq75_e1184: f64 = (p.p6 * var_sigvdsfp1s);
        let eq75_e1186: f64 = (eq75_e1184 * var_ids_fp1s);
        let eq75_e1186_d_n0: f64 = (eq75_e1184 * var_ids_fp1s_dn0);
        let eq75_e1186_d_n1: f64 = (eq75_e1184 * var_ids_fp1s_dn1);
        let eq75_e1186_d_n2: f64 = (eq75_e1184 * var_ids_fp1s_dn2);
        let eq75_e1186_d_n3: f64 = (eq75_e1184 * var_ids_fp1s_dn3);
        let eq75_e1186_d_n4: f64 = (eq75_e1184 * var_ids_fp1s_dn4);
        let eq75_e1186_d_n5: f64 = (eq75_e1184 * var_ids_fp1s_dn5);
        let eq75_e1186_d_n6: f64 = (eq75_e1184 * var_ids_fp1s_dn6);
        let eq75_e1186_d_n7: f64 = (eq75_e1184 * var_ids_fp1s_dn7);
        let eq75_e1186_d_n8: f64 = (eq75_e1184 * var_ids_fp1s_dn8);
        let eq75_e1186_d_n9: f64 = (eq75_e1184 * var_ids_fp1s_dn9);
        let eq75_e1186_d_n12: f64 = (eq75_e1184 * var_ids_fp1s_dn12);
        let eq75_e1186_d_n14: f64 = (eq75_e1184 * var_ids_fp1s_dn14);
        let eq75_e1186_d_n15: f64 = (eq75_e1184 * var_ids_fp1s_dn15);
        let eq75_e1186_d_n16: f64 = (eq75_e1184 * var_ids_fp1s_dn16);
        let eq75_e1186_d_n17: f64 = (eq75_e1184 * var_ids_fp1s_dn17);
        let eq75_e1186_d_n18: f64 = (eq75_e1184 * var_ids_fp1s_dn18);
        let eq75_e1186_d_n19: f64 = (eq75_e1184 * var_ids_fp1s_dn19);
        let eq75_e1186_d_n20: f64 = (eq75_e1184 * var_ids_fp1s_dn20);
        let eq75_e1186_d_n21: f64 = (eq75_e1184 * var_ids_fp1s_dn21);
        let eq75_e1186_d_n22: f64 = (eq75_e1184 * var_ids_fp1s_dn22);
        let eq75_e1189: f64 = (p.p6 * var_gdsmin_t);
        let eq75_e1189_d_n4: f64 = (p.p6 * var_gdsmin_t_dn4);
        let eq75_e1191: f64 = (eq75_e1189 * (nv8 - nv19));
        let eq75_e1191_d_n4: f64 = (eq75_e1189_d_n4 * (nv8 - nv19));
        let eq75_e1192: f64 = (eq75_e1186 + eq75_e1191);
        let eq75_e1192_d_n4: f64 = (eq75_e1186_d_n4 + eq75_e1191_d_n4);
        let eq75_e1192_d_n8: f64 = (eq75_e1186_d_n8 + eq75_e1189);
        let eq75_e1192_d_n19: f64 = (eq75_e1186_d_n19 + (-eq75_e1189));
        (eq75_e1192, eq75_e1186_d_n0, eq75_e1186_d_n1, eq75_e1186_d_n2, eq75_e1186_d_n3, eq75_e1192_d_n4, eq75_e1186_d_n5, eq75_e1186_d_n6, eq75_e1186_d_n7, eq75_e1192_d_n8, eq75_e1186_d_n9, eq75_e1186_d_n12, eq75_e1186_d_n14, eq75_e1186_d_n15, eq75_e1186_d_n16, eq75_e1186_d_n17, eq75_e1186_d_n18, eq75_e1192_d_n19, eq75_e1186_d_n20, eq75_e1186_d_n21, eq75_e1186_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq75_value: f64 = eq75_e1194;
        let eq75_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq75_node_derivatives: [f64; 20] = [eq75_e1194_d_n0, eq75_e1194_d_n1, eq75_e1194_d_n2, eq75_e1194_d_n3, eq75_e1194_d_n4, eq75_e1194_d_n5, eq75_e1194_d_n6, eq75_e1194_d_n7, eq75_e1194_d_n8, eq75_e1194_d_n9, eq75_e1194_d_n12, eq75_e1194_d_n14, eq75_e1194_d_n15, eq75_e1194_d_n16, eq75_e1194_d_n17, eq75_e1194_d_n18, eq75_e1194_d_n19, eq75_e1194_d_n20, eq75_e1194_d_n21, eq75_e1194_d_n22];
        let eq75_branch_derivative_indices: [usize; 0] = [];
        let eq75_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(19),
            multiplicity * (eq75_value),
            &eq75_node_derivative_indices,
            &eq75_node_derivatives,
            &eq75_branch_derivative_indices,
            &eq75_branch_derivatives,
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
        var_gdsmin_t: f64,
        var_gdsmin_t_dn4: f64,
        var_guard429: f64,
        var_guard430: f64,
        var_guard444: f64,
        var_guard445: f64,
        var_guard459: f64,
        var_guard460: f64,
        var_guard474: f64,
        var_guard475: f64,
        var_guard489: f64,
        var_guard490: f64,
        var_guard504: f64,
        var_guard505: f64,
        var_guard535: f64,
        var_idb: f64,
        var_idb_dn0: f64,
        var_idb_dn1: f64,
        var_idb_dn12: f64,
        var_idb_dn14: f64,
        var_idb_dn15: f64,
        var_idb_dn16: f64,
        var_idb_dn17: f64,
        var_idb_dn18: f64,
        var_idb_dn19: f64,
        var_idb_dn2: f64,
        var_idb_dn20: f64,
        var_idb_dn21: f64,
        var_idb_dn22: f64,
        var_idb_dn3: f64,
        var_idb_dn4: f64,
        var_idb_dn5: f64,
        var_idb_dn6: f64,
        var_idb_dn7: f64,
        var_idb_dn8: f64,
        var_idb_dn9: f64,
        var_ids_fp2: f64,
        var_ids_fp2_dn0: f64,
        var_ids_fp2_dn1: f64,
        var_ids_fp2_dn12: f64,
        var_ids_fp2_dn14: f64,
        var_ids_fp2_dn15: f64,
        var_ids_fp2_dn16: f64,
        var_ids_fp2_dn17: f64,
        var_ids_fp2_dn18: f64,
        var_ids_fp2_dn19: f64,
        var_ids_fp2_dn2: f64,
        var_ids_fp2_dn20: f64,
        var_ids_fp2_dn21: f64,
        var_ids_fp2_dn22: f64,
        var_ids_fp2_dn3: f64,
        var_ids_fp2_dn4: f64,
        var_ids_fp2_dn5: f64,
        var_ids_fp2_dn6: f64,
        var_ids_fp2_dn7: f64,
        var_ids_fp2_dn8: f64,
        var_ids_fp2_dn9: f64,
        var_ids_fp2s: f64,
        var_ids_fp2s_dn0: f64,
        var_ids_fp2s_dn1: f64,
        var_ids_fp2s_dn12: f64,
        var_ids_fp2s_dn14: f64,
        var_ids_fp2s_dn15: f64,
        var_ids_fp2s_dn16: f64,
        var_ids_fp2s_dn17: f64,
        var_ids_fp2s_dn18: f64,
        var_ids_fp2s_dn19: f64,
        var_ids_fp2s_dn2: f64,
        var_ids_fp2s_dn20: f64,
        var_ids_fp2s_dn21: f64,
        var_ids_fp2s_dn22: f64,
        var_ids_fp2s_dn3: f64,
        var_ids_fp2s_dn4: f64,
        var_ids_fp2s_dn5: f64,
        var_ids_fp2s_dn6: f64,
        var_ids_fp2s_dn7: f64,
        var_ids_fp2s_dn8: f64,
        var_ids_fp2s_dn9: f64,
        var_ids_fp3: f64,
        var_ids_fp3_dn0: f64,
        var_ids_fp3_dn1: f64,
        var_ids_fp3_dn12: f64,
        var_ids_fp3_dn14: f64,
        var_ids_fp3_dn15: f64,
        var_ids_fp3_dn16: f64,
        var_ids_fp3_dn17: f64,
        var_ids_fp3_dn18: f64,
        var_ids_fp3_dn19: f64,
        var_ids_fp3_dn2: f64,
        var_ids_fp3_dn20: f64,
        var_ids_fp3_dn21: f64,
        var_ids_fp3_dn22: f64,
        var_ids_fp3_dn3: f64,
        var_ids_fp3_dn4: f64,
        var_ids_fp3_dn5: f64,
        var_ids_fp3_dn6: f64,
        var_ids_fp3_dn7: f64,
        var_ids_fp3_dn8: f64,
        var_ids_fp3_dn9: f64,
        var_ids_fp3s: f64,
        var_ids_fp3s_dn0: f64,
        var_ids_fp3s_dn1: f64,
        var_ids_fp3s_dn12: f64,
        var_ids_fp3s_dn14: f64,
        var_ids_fp3s_dn15: f64,
        var_ids_fp3s_dn16: f64,
        var_ids_fp3s_dn17: f64,
        var_ids_fp3s_dn18: f64,
        var_ids_fp3s_dn19: f64,
        var_ids_fp3s_dn2: f64,
        var_ids_fp3s_dn20: f64,
        var_ids_fp3s_dn21: f64,
        var_ids_fp3s_dn22: f64,
        var_ids_fp3s_dn3: f64,
        var_ids_fp3s_dn4: f64,
        var_ids_fp3s_dn5: f64,
        var_ids_fp3s_dn6: f64,
        var_ids_fp3s_dn7: f64,
        var_ids_fp3s_dn8: f64,
        var_ids_fp3s_dn9: f64,
        var_ids_fp4: f64,
        var_ids_fp4_dn0: f64,
        var_ids_fp4_dn1: f64,
        var_ids_fp4_dn12: f64,
        var_ids_fp4_dn14: f64,
        var_ids_fp4_dn15: f64,
        var_ids_fp4_dn16: f64,
        var_ids_fp4_dn17: f64,
        var_ids_fp4_dn18: f64,
        var_ids_fp4_dn19: f64,
        var_ids_fp4_dn2: f64,
        var_ids_fp4_dn20: f64,
        var_ids_fp4_dn21: f64,
        var_ids_fp4_dn22: f64,
        var_ids_fp4_dn3: f64,
        var_ids_fp4_dn4: f64,
        var_ids_fp4_dn5: f64,
        var_ids_fp4_dn6: f64,
        var_ids_fp4_dn7: f64,
        var_ids_fp4_dn8: f64,
        var_ids_fp4_dn9: f64,
        var_ids_fp4s: f64,
        var_ids_fp4s_dn0: f64,
        var_ids_fp4s_dn1: f64,
        var_ids_fp4s_dn12: f64,
        var_ids_fp4s_dn14: f64,
        var_ids_fp4s_dn15: f64,
        var_ids_fp4s_dn16: f64,
        var_ids_fp4s_dn17: f64,
        var_ids_fp4s_dn18: f64,
        var_ids_fp4s_dn19: f64,
        var_ids_fp4s_dn2: f64,
        var_ids_fp4s_dn20: f64,
        var_ids_fp4s_dn21: f64,
        var_ids_fp4s_dn22: f64,
        var_ids_fp4s_dn3: f64,
        var_ids_fp4s_dn4: f64,
        var_ids_fp4s_dn5: f64,
        var_ids_fp4s_dn6: f64,
        var_ids_fp4s_dn7: f64,
        var_ids_fp4s_dn8: f64,
        var_ids_fp4s_dn9: f64,
        var_isb: f64,
        var_isb_dn0: f64,
        var_isb_dn1: f64,
        var_isb_dn12: f64,
        var_isb_dn14: f64,
        var_isb_dn15: f64,
        var_isb_dn16: f64,
        var_isb_dn17: f64,
        var_isb_dn18: f64,
        var_isb_dn19: f64,
        var_isb_dn2: f64,
        var_isb_dn20: f64,
        var_isb_dn21: f64,
        var_isb_dn22: f64,
        var_isb_dn3: f64,
        var_isb_dn4: f64,
        var_isb_dn5: f64,
        var_isb_dn6: f64,
        var_isb_dn7: f64,
        var_isb_dn8: f64,
        var_isb_dn9: f64,
        var_qdint: f64,
        var_qdint_dn0: f64,
        var_qdint_dn1: f64,
        var_qdint_dn12: f64,
        var_qdint_dn14: f64,
        var_qdint_dn15: f64,
        var_qdint_dn16: f64,
        var_qdint_dn17: f64,
        var_qdint_dn18: f64,
        var_qdint_dn19: f64,
        var_qdint_dn2: f64,
        var_qdint_dn20: f64,
        var_qdint_dn21: f64,
        var_qdint_dn22: f64,
        var_qdint_dn3: f64,
        var_qdint_dn4: f64,
        var_qdint_dn5: f64,
        var_qdint_dn6: f64,
        var_qdint_dn7: f64,
        var_qdint_dn8: f64,
        var_qdint_dn9: f64,
        var_qdov: f64,
        var_qdov_dn0: f64,
        var_qdov_dn1: f64,
        var_qdov_dn10: f64,
        var_qdov_dn2: f64,
        var_qgint: f64,
        var_qgint_dn0: f64,
        var_qgint_dn1: f64,
        var_qgint_dn12: f64,
        var_qgint_dn14: f64,
        var_qgint_dn15: f64,
        var_qgint_dn16: f64,
        var_qgint_dn17: f64,
        var_qgint_dn18: f64,
        var_qgint_dn19: f64,
        var_qgint_dn2: f64,
        var_qgint_dn20: f64,
        var_qgint_dn21: f64,
        var_qgint_dn22: f64,
        var_qgint_dn3: f64,
        var_qgint_dn4: f64,
        var_qgint_dn5: f64,
        var_qgint_dn6: f64,
        var_qgint_dn7: f64,
        var_qgint_dn8: f64,
        var_qgint_dn9: f64,
        var_qsov: f64,
        var_qsov_dn1: f64,
        var_qsov_dn10: f64,
        var_qsov_dn2: f64,
        var_sigvdsfp2: f64,
        var_sigvdsfp2s: f64,
        var_sigvdsfp3: f64,
        var_sigvdsfp3s: f64,
        var_sigvdsfp4: f64,
        var_sigvdsfp4s: f64,
    ) {
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let nv19 = ctx.node_voltage(nodes[19]);
        let nv20 = ctx.node_voltage(nodes[20]);
        let nv21 = ctx.node_voltage(nodes[21]);
        let nv22 = ctx.node_voltage(nodes[22]);
        let (eq79_e1230, eq79_e1230_d_n0, eq79_e1230_d_n1, eq79_e1230_d_n2, eq79_e1230_d_n3, eq79_e1230_d_n4, eq79_e1230_d_n5, eq79_e1230_d_n6, eq79_e1230_d_n7, eq79_e1230_d_n8, eq79_e1230_d_n9, eq79_e1230_d_n12, eq79_e1230_d_n14, eq79_e1230_d_n15, eq79_e1230_d_n16, eq79_e1230_d_n17, eq79_e1230_d_n18, eq79_e1230_d_n19, eq79_e1230_d_n20, eq79_e1230_d_n21, eq79_e1230_d_n22,) = {
    if ((var_guard429 != 0.0) && (var_guard430 != 0.0)) {
        let eq79_e1220: f64 = (p.p6 * var_sigvdsfp2);
        let eq79_e1222: f64 = (eq79_e1220 * var_ids_fp2);
        let eq79_e1222_d_n0: f64 = (eq79_e1220 * var_ids_fp2_dn0);
        let eq79_e1222_d_n1: f64 = (eq79_e1220 * var_ids_fp2_dn1);
        let eq79_e1222_d_n2: f64 = (eq79_e1220 * var_ids_fp2_dn2);
        let eq79_e1222_d_n3: f64 = (eq79_e1220 * var_ids_fp2_dn3);
        let eq79_e1222_d_n4: f64 = (eq79_e1220 * var_ids_fp2_dn4);
        let eq79_e1222_d_n5: f64 = (eq79_e1220 * var_ids_fp2_dn5);
        let eq79_e1222_d_n6: f64 = (eq79_e1220 * var_ids_fp2_dn6);
        let eq79_e1222_d_n7: f64 = (eq79_e1220 * var_ids_fp2_dn7);
        let eq79_e1222_d_n8: f64 = (eq79_e1220 * var_ids_fp2_dn8);
        let eq79_e1222_d_n9: f64 = (eq79_e1220 * var_ids_fp2_dn9);
        let eq79_e1222_d_n12: f64 = (eq79_e1220 * var_ids_fp2_dn12);
        let eq79_e1222_d_n14: f64 = (eq79_e1220 * var_ids_fp2_dn14);
        let eq79_e1222_d_n15: f64 = (eq79_e1220 * var_ids_fp2_dn15);
        let eq79_e1222_d_n16: f64 = (eq79_e1220 * var_ids_fp2_dn16);
        let eq79_e1222_d_n17: f64 = (eq79_e1220 * var_ids_fp2_dn17);
        let eq79_e1222_d_n18: f64 = (eq79_e1220 * var_ids_fp2_dn18);
        let eq79_e1222_d_n19: f64 = (eq79_e1220 * var_ids_fp2_dn19);
        let eq79_e1222_d_n20: f64 = (eq79_e1220 * var_ids_fp2_dn20);
        let eq79_e1222_d_n21: f64 = (eq79_e1220 * var_ids_fp2_dn21);
        let eq79_e1222_d_n22: f64 = (eq79_e1220 * var_ids_fp2_dn22);
        let eq79_e1225: f64 = (p.p6 * var_gdsmin_t);
        let eq79_e1225_d_n4: f64 = (p.p6 * var_gdsmin_t_dn4);
        let eq79_e1227: f64 = (eq79_e1225 * (nv16 - nv15));
        let eq79_e1227_d_n4: f64 = (eq79_e1225_d_n4 * (nv16 - nv15));
        let eq79_e1228: f64 = (eq79_e1222 + eq79_e1227);
        let eq79_e1228_d_n4: f64 = (eq79_e1222_d_n4 + eq79_e1227_d_n4);
        let eq79_e1228_d_n15: f64 = (eq79_e1222_d_n15 + (-eq79_e1225));
        let eq79_e1228_d_n16: f64 = (eq79_e1222_d_n16 + eq79_e1225);
        (eq79_e1228, eq79_e1222_d_n0, eq79_e1222_d_n1, eq79_e1222_d_n2, eq79_e1222_d_n3, eq79_e1228_d_n4, eq79_e1222_d_n5, eq79_e1222_d_n6, eq79_e1222_d_n7, eq79_e1222_d_n8, eq79_e1222_d_n9, eq79_e1222_d_n12, eq79_e1222_d_n14, eq79_e1228_d_n15, eq79_e1228_d_n16, eq79_e1222_d_n17, eq79_e1222_d_n18, eq79_e1222_d_n19, eq79_e1222_d_n20, eq79_e1222_d_n21, eq79_e1222_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq79_value: f64 = eq79_e1230;
        let eq79_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq79_node_derivatives: [f64; 20] = [eq79_e1230_d_n0, eq79_e1230_d_n1, eq79_e1230_d_n2, eq79_e1230_d_n3, eq79_e1230_d_n4, eq79_e1230_d_n5, eq79_e1230_d_n6, eq79_e1230_d_n7, eq79_e1230_d_n8, eq79_e1230_d_n9, eq79_e1230_d_n12, eq79_e1230_d_n14, eq79_e1230_d_n15, eq79_e1230_d_n16, eq79_e1230_d_n17, eq79_e1230_d_n18, eq79_e1230_d_n19, eq79_e1230_d_n20, eq79_e1230_d_n21, eq79_e1230_d_n22];
        let eq79_branch_derivative_indices: [usize; 0] = [];
        let eq79_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(16),
            Some(15),
            multiplicity * (eq79_value),
            &eq79_node_derivative_indices,
            &eq79_node_derivatives,
            &eq79_branch_derivative_indices,
            &eq79_branch_derivatives,
            multiplicity,
        );
        let (eq82_e1258, eq82_e1258_d_n0, eq82_e1258_d_n1, eq82_e1258_d_n2, eq82_e1258_d_n3, eq82_e1258_d_n4, eq82_e1258_d_n5, eq82_e1258_d_n6, eq82_e1258_d_n7, eq82_e1258_d_n8, eq82_e1258_d_n9, eq82_e1258_d_n12, eq82_e1258_d_n14, eq82_e1258_d_n15, eq82_e1258_d_n16, eq82_e1258_d_n17, eq82_e1258_d_n18, eq82_e1258_d_n19, eq82_e1258_d_n20, eq82_e1258_d_n21, eq82_e1258_d_n22,) = {
    if ((var_guard444 != 0.0) && (var_guard445 != 0.0)) {
        let eq82_e1248: f64 = (p.p6 * var_sigvdsfp2s);
        let eq82_e1250: f64 = (eq82_e1248 * var_ids_fp2s);
        let eq82_e1250_d_n0: f64 = (eq82_e1248 * var_ids_fp2s_dn0);
        let eq82_e1250_d_n1: f64 = (eq82_e1248 * var_ids_fp2s_dn1);
        let eq82_e1250_d_n2: f64 = (eq82_e1248 * var_ids_fp2s_dn2);
        let eq82_e1250_d_n3: f64 = (eq82_e1248 * var_ids_fp2s_dn3);
        let eq82_e1250_d_n4: f64 = (eq82_e1248 * var_ids_fp2s_dn4);
        let eq82_e1250_d_n5: f64 = (eq82_e1248 * var_ids_fp2s_dn5);
        let eq82_e1250_d_n6: f64 = (eq82_e1248 * var_ids_fp2s_dn6);
        let eq82_e1250_d_n7: f64 = (eq82_e1248 * var_ids_fp2s_dn7);
        let eq82_e1250_d_n8: f64 = (eq82_e1248 * var_ids_fp2s_dn8);
        let eq82_e1250_d_n9: f64 = (eq82_e1248 * var_ids_fp2s_dn9);
        let eq82_e1250_d_n12: f64 = (eq82_e1248 * var_ids_fp2s_dn12);
        let eq82_e1250_d_n14: f64 = (eq82_e1248 * var_ids_fp2s_dn14);
        let eq82_e1250_d_n15: f64 = (eq82_e1248 * var_ids_fp2s_dn15);
        let eq82_e1250_d_n16: f64 = (eq82_e1248 * var_ids_fp2s_dn16);
        let eq82_e1250_d_n17: f64 = (eq82_e1248 * var_ids_fp2s_dn17);
        let eq82_e1250_d_n18: f64 = (eq82_e1248 * var_ids_fp2s_dn18);
        let eq82_e1250_d_n19: f64 = (eq82_e1248 * var_ids_fp2s_dn19);
        let eq82_e1250_d_n20: f64 = (eq82_e1248 * var_ids_fp2s_dn20);
        let eq82_e1250_d_n21: f64 = (eq82_e1248 * var_ids_fp2s_dn21);
        let eq82_e1250_d_n22: f64 = (eq82_e1248 * var_ids_fp2s_dn22);
        let eq82_e1253: f64 = (p.p6 * var_gdsmin_t);
        let eq82_e1253_d_n4: f64 = (p.p6 * var_gdsmin_t_dn4);
        let eq82_e1255: f64 = (eq82_e1253 * (nv19 - nv20));
        let eq82_e1255_d_n4: f64 = (eq82_e1253_d_n4 * (nv19 - nv20));
        let eq82_e1256: f64 = (eq82_e1250 + eq82_e1255);
        let eq82_e1256_d_n4: f64 = (eq82_e1250_d_n4 + eq82_e1255_d_n4);
        let eq82_e1256_d_n19: f64 = (eq82_e1250_d_n19 + eq82_e1253);
        let eq82_e1256_d_n20: f64 = (eq82_e1250_d_n20 + (-eq82_e1253));
        (eq82_e1256, eq82_e1250_d_n0, eq82_e1250_d_n1, eq82_e1250_d_n2, eq82_e1250_d_n3, eq82_e1256_d_n4, eq82_e1250_d_n5, eq82_e1250_d_n6, eq82_e1250_d_n7, eq82_e1250_d_n8, eq82_e1250_d_n9, eq82_e1250_d_n12, eq82_e1250_d_n14, eq82_e1250_d_n15, eq82_e1250_d_n16, eq82_e1250_d_n17, eq82_e1250_d_n18, eq82_e1256_d_n19, eq82_e1256_d_n20, eq82_e1250_d_n21, eq82_e1250_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq82_value: f64 = eq82_e1258;
        let eq82_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq82_node_derivatives: [f64; 20] = [eq82_e1258_d_n0, eq82_e1258_d_n1, eq82_e1258_d_n2, eq82_e1258_d_n3, eq82_e1258_d_n4, eq82_e1258_d_n5, eq82_e1258_d_n6, eq82_e1258_d_n7, eq82_e1258_d_n8, eq82_e1258_d_n9, eq82_e1258_d_n12, eq82_e1258_d_n14, eq82_e1258_d_n15, eq82_e1258_d_n16, eq82_e1258_d_n17, eq82_e1258_d_n18, eq82_e1258_d_n19, eq82_e1258_d_n20, eq82_e1258_d_n21, eq82_e1258_d_n22];
        let eq82_branch_derivative_indices: [usize; 0] = [];
        let eq82_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(19),
            Some(20),
            multiplicity * (eq82_value),
            &eq82_node_derivative_indices,
            &eq82_node_derivatives,
            &eq82_branch_derivative_indices,
            &eq82_branch_derivatives,
            multiplicity,
        );
        let (eq86_e1294, eq86_e1294_d_n0, eq86_e1294_d_n1, eq86_e1294_d_n2, eq86_e1294_d_n3, eq86_e1294_d_n4, eq86_e1294_d_n5, eq86_e1294_d_n6, eq86_e1294_d_n7, eq86_e1294_d_n8, eq86_e1294_d_n9, eq86_e1294_d_n12, eq86_e1294_d_n14, eq86_e1294_d_n15, eq86_e1294_d_n16, eq86_e1294_d_n17, eq86_e1294_d_n18, eq86_e1294_d_n19, eq86_e1294_d_n20, eq86_e1294_d_n21, eq86_e1294_d_n22,) = {
    if ((var_guard459 != 0.0) && (var_guard460 != 0.0)) {
        let eq86_e1284: f64 = (p.p6 * var_sigvdsfp3);
        let eq86_e1286: f64 = (eq86_e1284 * var_ids_fp3);
        let eq86_e1286_d_n0: f64 = (eq86_e1284 * var_ids_fp3_dn0);
        let eq86_e1286_d_n1: f64 = (eq86_e1284 * var_ids_fp3_dn1);
        let eq86_e1286_d_n2: f64 = (eq86_e1284 * var_ids_fp3_dn2);
        let eq86_e1286_d_n3: f64 = (eq86_e1284 * var_ids_fp3_dn3);
        let eq86_e1286_d_n4: f64 = (eq86_e1284 * var_ids_fp3_dn4);
        let eq86_e1286_d_n5: f64 = (eq86_e1284 * var_ids_fp3_dn5);
        let eq86_e1286_d_n6: f64 = (eq86_e1284 * var_ids_fp3_dn6);
        let eq86_e1286_d_n7: f64 = (eq86_e1284 * var_ids_fp3_dn7);
        let eq86_e1286_d_n8: f64 = (eq86_e1284 * var_ids_fp3_dn8);
        let eq86_e1286_d_n9: f64 = (eq86_e1284 * var_ids_fp3_dn9);
        let eq86_e1286_d_n12: f64 = (eq86_e1284 * var_ids_fp3_dn12);
        let eq86_e1286_d_n14: f64 = (eq86_e1284 * var_ids_fp3_dn14);
        let eq86_e1286_d_n15: f64 = (eq86_e1284 * var_ids_fp3_dn15);
        let eq86_e1286_d_n16: f64 = (eq86_e1284 * var_ids_fp3_dn16);
        let eq86_e1286_d_n17: f64 = (eq86_e1284 * var_ids_fp3_dn17);
        let eq86_e1286_d_n18: f64 = (eq86_e1284 * var_ids_fp3_dn18);
        let eq86_e1286_d_n19: f64 = (eq86_e1284 * var_ids_fp3_dn19);
        let eq86_e1286_d_n20: f64 = (eq86_e1284 * var_ids_fp3_dn20);
        let eq86_e1286_d_n21: f64 = (eq86_e1284 * var_ids_fp3_dn21);
        let eq86_e1286_d_n22: f64 = (eq86_e1284 * var_ids_fp3_dn22);
        let eq86_e1289: f64 = (p.p6 * var_gdsmin_t);
        let eq86_e1289_d_n4: f64 = (p.p6 * var_gdsmin_t_dn4);
        let eq86_e1291: f64 = (eq86_e1289 * (nv17 - nv16));
        let eq86_e1291_d_n4: f64 = (eq86_e1289_d_n4 * (nv17 - nv16));
        let eq86_e1292: f64 = (eq86_e1286 + eq86_e1291);
        let eq86_e1292_d_n4: f64 = (eq86_e1286_d_n4 + eq86_e1291_d_n4);
        let eq86_e1292_d_n16: f64 = (eq86_e1286_d_n16 + (-eq86_e1289));
        let eq86_e1292_d_n17: f64 = (eq86_e1286_d_n17 + eq86_e1289);
        (eq86_e1292, eq86_e1286_d_n0, eq86_e1286_d_n1, eq86_e1286_d_n2, eq86_e1286_d_n3, eq86_e1292_d_n4, eq86_e1286_d_n5, eq86_e1286_d_n6, eq86_e1286_d_n7, eq86_e1286_d_n8, eq86_e1286_d_n9, eq86_e1286_d_n12, eq86_e1286_d_n14, eq86_e1286_d_n15, eq86_e1292_d_n16, eq86_e1292_d_n17, eq86_e1286_d_n18, eq86_e1286_d_n19, eq86_e1286_d_n20, eq86_e1286_d_n21, eq86_e1286_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq86_value: f64 = eq86_e1294;
        let eq86_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq86_node_derivatives: [f64; 20] = [eq86_e1294_d_n0, eq86_e1294_d_n1, eq86_e1294_d_n2, eq86_e1294_d_n3, eq86_e1294_d_n4, eq86_e1294_d_n5, eq86_e1294_d_n6, eq86_e1294_d_n7, eq86_e1294_d_n8, eq86_e1294_d_n9, eq86_e1294_d_n12, eq86_e1294_d_n14, eq86_e1294_d_n15, eq86_e1294_d_n16, eq86_e1294_d_n17, eq86_e1294_d_n18, eq86_e1294_d_n19, eq86_e1294_d_n20, eq86_e1294_d_n21, eq86_e1294_d_n22];
        let eq86_branch_derivative_indices: [usize; 0] = [];
        let eq86_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(17),
            Some(16),
            multiplicity * (eq86_value),
            &eq86_node_derivative_indices,
            &eq86_node_derivatives,
            &eq86_branch_derivative_indices,
            &eq86_branch_derivatives,
            multiplicity,
        );
        let (eq89_e1322, eq89_e1322_d_n0, eq89_e1322_d_n1, eq89_e1322_d_n2, eq89_e1322_d_n3, eq89_e1322_d_n4, eq89_e1322_d_n5, eq89_e1322_d_n6, eq89_e1322_d_n7, eq89_e1322_d_n8, eq89_e1322_d_n9, eq89_e1322_d_n12, eq89_e1322_d_n14, eq89_e1322_d_n15, eq89_e1322_d_n16, eq89_e1322_d_n17, eq89_e1322_d_n18, eq89_e1322_d_n19, eq89_e1322_d_n20, eq89_e1322_d_n21, eq89_e1322_d_n22,) = {
    if ((var_guard474 != 0.0) && (var_guard475 != 0.0)) {
        let eq89_e1312: f64 = (p.p6 * var_sigvdsfp3s);
        let eq89_e1314: f64 = (eq89_e1312 * var_ids_fp3s);
        let eq89_e1314_d_n0: f64 = (eq89_e1312 * var_ids_fp3s_dn0);
        let eq89_e1314_d_n1: f64 = (eq89_e1312 * var_ids_fp3s_dn1);
        let eq89_e1314_d_n2: f64 = (eq89_e1312 * var_ids_fp3s_dn2);
        let eq89_e1314_d_n3: f64 = (eq89_e1312 * var_ids_fp3s_dn3);
        let eq89_e1314_d_n4: f64 = (eq89_e1312 * var_ids_fp3s_dn4);
        let eq89_e1314_d_n5: f64 = (eq89_e1312 * var_ids_fp3s_dn5);
        let eq89_e1314_d_n6: f64 = (eq89_e1312 * var_ids_fp3s_dn6);
        let eq89_e1314_d_n7: f64 = (eq89_e1312 * var_ids_fp3s_dn7);
        let eq89_e1314_d_n8: f64 = (eq89_e1312 * var_ids_fp3s_dn8);
        let eq89_e1314_d_n9: f64 = (eq89_e1312 * var_ids_fp3s_dn9);
        let eq89_e1314_d_n12: f64 = (eq89_e1312 * var_ids_fp3s_dn12);
        let eq89_e1314_d_n14: f64 = (eq89_e1312 * var_ids_fp3s_dn14);
        let eq89_e1314_d_n15: f64 = (eq89_e1312 * var_ids_fp3s_dn15);
        let eq89_e1314_d_n16: f64 = (eq89_e1312 * var_ids_fp3s_dn16);
        let eq89_e1314_d_n17: f64 = (eq89_e1312 * var_ids_fp3s_dn17);
        let eq89_e1314_d_n18: f64 = (eq89_e1312 * var_ids_fp3s_dn18);
        let eq89_e1314_d_n19: f64 = (eq89_e1312 * var_ids_fp3s_dn19);
        let eq89_e1314_d_n20: f64 = (eq89_e1312 * var_ids_fp3s_dn20);
        let eq89_e1314_d_n21: f64 = (eq89_e1312 * var_ids_fp3s_dn21);
        let eq89_e1314_d_n22: f64 = (eq89_e1312 * var_ids_fp3s_dn22);
        let eq89_e1317: f64 = (p.p6 * var_gdsmin_t);
        let eq89_e1317_d_n4: f64 = (p.p6 * var_gdsmin_t_dn4);
        let eq89_e1319: f64 = (eq89_e1317 * (nv20 - nv21));
        let eq89_e1319_d_n4: f64 = (eq89_e1317_d_n4 * (nv20 - nv21));
        let eq89_e1320: f64 = (eq89_e1314 + eq89_e1319);
        let eq89_e1320_d_n4: f64 = (eq89_e1314_d_n4 + eq89_e1319_d_n4);
        let eq89_e1320_d_n20: f64 = (eq89_e1314_d_n20 + eq89_e1317);
        let eq89_e1320_d_n21: f64 = (eq89_e1314_d_n21 + (-eq89_e1317));
        (eq89_e1320, eq89_e1314_d_n0, eq89_e1314_d_n1, eq89_e1314_d_n2, eq89_e1314_d_n3, eq89_e1320_d_n4, eq89_e1314_d_n5, eq89_e1314_d_n6, eq89_e1314_d_n7, eq89_e1314_d_n8, eq89_e1314_d_n9, eq89_e1314_d_n12, eq89_e1314_d_n14, eq89_e1314_d_n15, eq89_e1314_d_n16, eq89_e1314_d_n17, eq89_e1314_d_n18, eq89_e1314_d_n19, eq89_e1320_d_n20, eq89_e1320_d_n21, eq89_e1314_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq89_value: f64 = eq89_e1322;
        let eq89_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq89_node_derivatives: [f64; 20] = [eq89_e1322_d_n0, eq89_e1322_d_n1, eq89_e1322_d_n2, eq89_e1322_d_n3, eq89_e1322_d_n4, eq89_e1322_d_n5, eq89_e1322_d_n6, eq89_e1322_d_n7, eq89_e1322_d_n8, eq89_e1322_d_n9, eq89_e1322_d_n12, eq89_e1322_d_n14, eq89_e1322_d_n15, eq89_e1322_d_n16, eq89_e1322_d_n17, eq89_e1322_d_n18, eq89_e1322_d_n19, eq89_e1322_d_n20, eq89_e1322_d_n21, eq89_e1322_d_n22];
        let eq89_branch_derivative_indices: [usize; 0] = [];
        let eq89_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(20),
            Some(21),
            multiplicity * (eq89_value),
            &eq89_node_derivative_indices,
            &eq89_node_derivatives,
            &eq89_branch_derivative_indices,
            &eq89_branch_derivatives,
            multiplicity,
        );
        let (eq93_e1358, eq93_e1358_d_n0, eq93_e1358_d_n1, eq93_e1358_d_n2, eq93_e1358_d_n3, eq93_e1358_d_n4, eq93_e1358_d_n5, eq93_e1358_d_n6, eq93_e1358_d_n7, eq93_e1358_d_n8, eq93_e1358_d_n9, eq93_e1358_d_n12, eq93_e1358_d_n14, eq93_e1358_d_n15, eq93_e1358_d_n16, eq93_e1358_d_n17, eq93_e1358_d_n18, eq93_e1358_d_n19, eq93_e1358_d_n20, eq93_e1358_d_n21, eq93_e1358_d_n22,) = {
    if ((var_guard489 != 0.0) && (var_guard490 != 0.0)) {
        let eq93_e1348: f64 = (p.p6 * var_sigvdsfp4);
        let eq93_e1350: f64 = (eq93_e1348 * var_ids_fp4);
        let eq93_e1350_d_n0: f64 = (eq93_e1348 * var_ids_fp4_dn0);
        let eq93_e1350_d_n1: f64 = (eq93_e1348 * var_ids_fp4_dn1);
        let eq93_e1350_d_n2: f64 = (eq93_e1348 * var_ids_fp4_dn2);
        let eq93_e1350_d_n3: f64 = (eq93_e1348 * var_ids_fp4_dn3);
        let eq93_e1350_d_n4: f64 = (eq93_e1348 * var_ids_fp4_dn4);
        let eq93_e1350_d_n5: f64 = (eq93_e1348 * var_ids_fp4_dn5);
        let eq93_e1350_d_n6: f64 = (eq93_e1348 * var_ids_fp4_dn6);
        let eq93_e1350_d_n7: f64 = (eq93_e1348 * var_ids_fp4_dn7);
        let eq93_e1350_d_n8: f64 = (eq93_e1348 * var_ids_fp4_dn8);
        let eq93_e1350_d_n9: f64 = (eq93_e1348 * var_ids_fp4_dn9);
        let eq93_e1350_d_n12: f64 = (eq93_e1348 * var_ids_fp4_dn12);
        let eq93_e1350_d_n14: f64 = (eq93_e1348 * var_ids_fp4_dn14);
        let eq93_e1350_d_n15: f64 = (eq93_e1348 * var_ids_fp4_dn15);
        let eq93_e1350_d_n16: f64 = (eq93_e1348 * var_ids_fp4_dn16);
        let eq93_e1350_d_n17: f64 = (eq93_e1348 * var_ids_fp4_dn17);
        let eq93_e1350_d_n18: f64 = (eq93_e1348 * var_ids_fp4_dn18);
        let eq93_e1350_d_n19: f64 = (eq93_e1348 * var_ids_fp4_dn19);
        let eq93_e1350_d_n20: f64 = (eq93_e1348 * var_ids_fp4_dn20);
        let eq93_e1350_d_n21: f64 = (eq93_e1348 * var_ids_fp4_dn21);
        let eq93_e1350_d_n22: f64 = (eq93_e1348 * var_ids_fp4_dn22);
        let eq93_e1353: f64 = (p.p6 * var_gdsmin_t);
        let eq93_e1353_d_n4: f64 = (p.p6 * var_gdsmin_t_dn4);
        let eq93_e1355: f64 = (eq93_e1353 * (nv18 - nv17));
        let eq93_e1355_d_n4: f64 = (eq93_e1353_d_n4 * (nv18 - nv17));
        let eq93_e1356: f64 = (eq93_e1350 + eq93_e1355);
        let eq93_e1356_d_n4: f64 = (eq93_e1350_d_n4 + eq93_e1355_d_n4);
        let eq93_e1356_d_n17: f64 = (eq93_e1350_d_n17 + (-eq93_e1353));
        let eq93_e1356_d_n18: f64 = (eq93_e1350_d_n18 + eq93_e1353);
        (eq93_e1356, eq93_e1350_d_n0, eq93_e1350_d_n1, eq93_e1350_d_n2, eq93_e1350_d_n3, eq93_e1356_d_n4, eq93_e1350_d_n5, eq93_e1350_d_n6, eq93_e1350_d_n7, eq93_e1350_d_n8, eq93_e1350_d_n9, eq93_e1350_d_n12, eq93_e1350_d_n14, eq93_e1350_d_n15, eq93_e1350_d_n16, eq93_e1356_d_n17, eq93_e1356_d_n18, eq93_e1350_d_n19, eq93_e1350_d_n20, eq93_e1350_d_n21, eq93_e1350_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq93_value: f64 = eq93_e1358;
        let eq93_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq93_node_derivatives: [f64; 20] = [eq93_e1358_d_n0, eq93_e1358_d_n1, eq93_e1358_d_n2, eq93_e1358_d_n3, eq93_e1358_d_n4, eq93_e1358_d_n5, eq93_e1358_d_n6, eq93_e1358_d_n7, eq93_e1358_d_n8, eq93_e1358_d_n9, eq93_e1358_d_n12, eq93_e1358_d_n14, eq93_e1358_d_n15, eq93_e1358_d_n16, eq93_e1358_d_n17, eq93_e1358_d_n18, eq93_e1358_d_n19, eq93_e1358_d_n20, eq93_e1358_d_n21, eq93_e1358_d_n22];
        let eq93_branch_derivative_indices: [usize; 0] = [];
        let eq93_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(18),
            Some(17),
            multiplicity * (eq93_value),
            &eq93_node_derivative_indices,
            &eq93_node_derivatives,
            &eq93_branch_derivative_indices,
            &eq93_branch_derivatives,
            multiplicity,
        );
        let (eq96_e1386, eq96_e1386_d_n0, eq96_e1386_d_n1, eq96_e1386_d_n2, eq96_e1386_d_n3, eq96_e1386_d_n4, eq96_e1386_d_n5, eq96_e1386_d_n6, eq96_e1386_d_n7, eq96_e1386_d_n8, eq96_e1386_d_n9, eq96_e1386_d_n12, eq96_e1386_d_n14, eq96_e1386_d_n15, eq96_e1386_d_n16, eq96_e1386_d_n17, eq96_e1386_d_n18, eq96_e1386_d_n19, eq96_e1386_d_n20, eq96_e1386_d_n21, eq96_e1386_d_n22,) = {
    if ((var_guard504 != 0.0) && (var_guard505 != 0.0)) {
        let eq96_e1376: f64 = (p.p6 * var_sigvdsfp4s);
        let eq96_e1378: f64 = (eq96_e1376 * var_ids_fp4s);
        let eq96_e1378_d_n0: f64 = (eq96_e1376 * var_ids_fp4s_dn0);
        let eq96_e1378_d_n1: f64 = (eq96_e1376 * var_ids_fp4s_dn1);
        let eq96_e1378_d_n2: f64 = (eq96_e1376 * var_ids_fp4s_dn2);
        let eq96_e1378_d_n3: f64 = (eq96_e1376 * var_ids_fp4s_dn3);
        let eq96_e1378_d_n4: f64 = (eq96_e1376 * var_ids_fp4s_dn4);
        let eq96_e1378_d_n5: f64 = (eq96_e1376 * var_ids_fp4s_dn5);
        let eq96_e1378_d_n6: f64 = (eq96_e1376 * var_ids_fp4s_dn6);
        let eq96_e1378_d_n7: f64 = (eq96_e1376 * var_ids_fp4s_dn7);
        let eq96_e1378_d_n8: f64 = (eq96_e1376 * var_ids_fp4s_dn8);
        let eq96_e1378_d_n9: f64 = (eq96_e1376 * var_ids_fp4s_dn9);
        let eq96_e1378_d_n12: f64 = (eq96_e1376 * var_ids_fp4s_dn12);
        let eq96_e1378_d_n14: f64 = (eq96_e1376 * var_ids_fp4s_dn14);
        let eq96_e1378_d_n15: f64 = (eq96_e1376 * var_ids_fp4s_dn15);
        let eq96_e1378_d_n16: f64 = (eq96_e1376 * var_ids_fp4s_dn16);
        let eq96_e1378_d_n17: f64 = (eq96_e1376 * var_ids_fp4s_dn17);
        let eq96_e1378_d_n18: f64 = (eq96_e1376 * var_ids_fp4s_dn18);
        let eq96_e1378_d_n19: f64 = (eq96_e1376 * var_ids_fp4s_dn19);
        let eq96_e1378_d_n20: f64 = (eq96_e1376 * var_ids_fp4s_dn20);
        let eq96_e1378_d_n21: f64 = (eq96_e1376 * var_ids_fp4s_dn21);
        let eq96_e1378_d_n22: f64 = (eq96_e1376 * var_ids_fp4s_dn22);
        let eq96_e1381: f64 = (p.p6 * var_gdsmin_t);
        let eq96_e1381_d_n4: f64 = (p.p6 * var_gdsmin_t_dn4);
        let eq96_e1383: f64 = (eq96_e1381 * (nv21 - nv22));
        let eq96_e1383_d_n4: f64 = (eq96_e1381_d_n4 * (nv21 - nv22));
        let eq96_e1384: f64 = (eq96_e1378 + eq96_e1383);
        let eq96_e1384_d_n4: f64 = (eq96_e1378_d_n4 + eq96_e1383_d_n4);
        let eq96_e1384_d_n21: f64 = (eq96_e1378_d_n21 + eq96_e1381);
        let eq96_e1384_d_n22: f64 = (eq96_e1378_d_n22 + (-eq96_e1381));
        (eq96_e1384, eq96_e1378_d_n0, eq96_e1378_d_n1, eq96_e1378_d_n2, eq96_e1378_d_n3, eq96_e1384_d_n4, eq96_e1378_d_n5, eq96_e1378_d_n6, eq96_e1378_d_n7, eq96_e1378_d_n8, eq96_e1378_d_n9, eq96_e1378_d_n12, eq96_e1378_d_n14, eq96_e1378_d_n15, eq96_e1378_d_n16, eq96_e1378_d_n17, eq96_e1378_d_n18, eq96_e1378_d_n19, eq96_e1378_d_n20, eq96_e1384_d_n21, eq96_e1384_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq96_value: f64 = eq96_e1386;
        let eq96_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq96_node_derivatives: [f64; 20] = [eq96_e1386_d_n0, eq96_e1386_d_n1, eq96_e1386_d_n2, eq96_e1386_d_n3, eq96_e1386_d_n4, eq96_e1386_d_n5, eq96_e1386_d_n6, eq96_e1386_d_n7, eq96_e1386_d_n8, eq96_e1386_d_n9, eq96_e1386_d_n12, eq96_e1386_d_n14, eq96_e1386_d_n15, eq96_e1386_d_n16, eq96_e1386_d_n17, eq96_e1386_d_n18, eq96_e1386_d_n19, eq96_e1386_d_n20, eq96_e1386_d_n21, eq96_e1386_d_n22];
        let eq96_branch_derivative_indices: [usize; 0] = [];
        let eq96_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(21),
            Some(22),
            multiplicity * (eq96_value),
            &eq96_node_derivative_indices,
            &eq96_node_derivatives,
            &eq96_branch_derivative_indices,
            &eq96_branch_derivatives,
            multiplicity,
        );
        let eq106_e1459: f64 = (p.p6 * var_idb);
        let eq106_e1459_d_n0: f64 = (p.p6 * var_idb_dn0);
        let eq106_e1459_d_n1: f64 = (p.p6 * var_idb_dn1);
        let eq106_e1459_d_n2: f64 = (p.p6 * var_idb_dn2);
        let eq106_e1459_d_n3: f64 = (p.p6 * var_idb_dn3);
        let eq106_e1459_d_n4: f64 = (p.p6 * var_idb_dn4);
        let eq106_e1459_d_n5: f64 = (p.p6 * var_idb_dn5);
        let eq106_e1459_d_n6: f64 = (p.p6 * var_idb_dn6);
        let eq106_e1459_d_n7: f64 = (p.p6 * var_idb_dn7);
        let eq106_e1459_d_n8: f64 = (p.p6 * var_idb_dn8);
        let eq106_e1459_d_n9: f64 = (p.p6 * var_idb_dn9);
        let eq106_e1459_d_n12: f64 = (p.p6 * var_idb_dn12);
        let eq106_e1459_d_n14: f64 = (p.p6 * var_idb_dn14);
        let eq106_e1459_d_n15: f64 = (p.p6 * var_idb_dn15);
        let eq106_e1459_d_n16: f64 = (p.p6 * var_idb_dn16);
        let eq106_e1459_d_n17: f64 = (p.p6 * var_idb_dn17);
        let eq106_e1459_d_n18: f64 = (p.p6 * var_idb_dn18);
        let eq106_e1459_d_n19: f64 = (p.p6 * var_idb_dn19);
        let eq106_e1459_d_n20: f64 = (p.p6 * var_idb_dn20);
        let eq106_e1459_d_n21: f64 = (p.p6 * var_idb_dn21);
        let eq106_e1459_d_n22: f64 = (p.p6 * var_idb_dn22);
        let eq106_value: f64 = eq106_e1459;
        let eq106_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq106_node_derivatives: [f64; 20] = [eq106_e1459_d_n0, eq106_e1459_d_n1, eq106_e1459_d_n2, eq106_e1459_d_n3, eq106_e1459_d_n4, eq106_e1459_d_n5, eq106_e1459_d_n6, eq106_e1459_d_n7, eq106_e1459_d_n8, eq106_e1459_d_n9, eq106_e1459_d_n12, eq106_e1459_d_n14, eq106_e1459_d_n15, eq106_e1459_d_n16, eq106_e1459_d_n17, eq106_e1459_d_n18, eq106_e1459_d_n19, eq106_e1459_d_n20, eq106_e1459_d_n21, eq106_e1459_d_n22];
        let eq106_branch_derivative_indices: [usize; 0] = [];
        let eq106_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(3),
            multiplicity * (eq106_value),
            &eq106_node_derivative_indices,
            &eq106_node_derivatives,
            &eq106_branch_derivative_indices,
            &eq106_branch_derivatives,
            multiplicity,
        );
        let eq107_e1462: f64 = (p.p6 * var_isb);
        let eq107_e1462_d_n0: f64 = (p.p6 * var_isb_dn0);
        let eq107_e1462_d_n1: f64 = (p.p6 * var_isb_dn1);
        let eq107_e1462_d_n2: f64 = (p.p6 * var_isb_dn2);
        let eq107_e1462_d_n3: f64 = (p.p6 * var_isb_dn3);
        let eq107_e1462_d_n4: f64 = (p.p6 * var_isb_dn4);
        let eq107_e1462_d_n5: f64 = (p.p6 * var_isb_dn5);
        let eq107_e1462_d_n6: f64 = (p.p6 * var_isb_dn6);
        let eq107_e1462_d_n7: f64 = (p.p6 * var_isb_dn7);
        let eq107_e1462_d_n8: f64 = (p.p6 * var_isb_dn8);
        let eq107_e1462_d_n9: f64 = (p.p6 * var_isb_dn9);
        let eq107_e1462_d_n12: f64 = (p.p6 * var_isb_dn12);
        let eq107_e1462_d_n14: f64 = (p.p6 * var_isb_dn14);
        let eq107_e1462_d_n15: f64 = (p.p6 * var_isb_dn15);
        let eq107_e1462_d_n16: f64 = (p.p6 * var_isb_dn16);
        let eq107_e1462_d_n17: f64 = (p.p6 * var_isb_dn17);
        let eq107_e1462_d_n18: f64 = (p.p6 * var_isb_dn18);
        let eq107_e1462_d_n19: f64 = (p.p6 * var_isb_dn19);
        let eq107_e1462_d_n20: f64 = (p.p6 * var_isb_dn20);
        let eq107_e1462_d_n21: f64 = (p.p6 * var_isb_dn21);
        let eq107_e1462_d_n22: f64 = (p.p6 * var_isb_dn22);
        let eq107_value: f64 = eq107_e1462;
        let eq107_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq107_node_derivatives: [f64; 20] = [eq107_e1462_d_n0, eq107_e1462_d_n1, eq107_e1462_d_n2, eq107_e1462_d_n3, eq107_e1462_d_n4, eq107_e1462_d_n5, eq107_e1462_d_n6, eq107_e1462_d_n7, eq107_e1462_d_n8, eq107_e1462_d_n9, eq107_e1462_d_n12, eq107_e1462_d_n14, eq107_e1462_d_n15, eq107_e1462_d_n16, eq107_e1462_d_n17, eq107_e1462_d_n18, eq107_e1462_d_n19, eq107_e1462_d_n20, eq107_e1462_d_n21, eq107_e1462_d_n22];
        let eq107_branch_derivative_indices: [usize; 0] = [];
        let eq107_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(3),
            multiplicity * (eq107_value),
            &eq107_node_derivative_indices,
            &eq107_node_derivatives,
            &eq107_branch_derivative_indices,
            &eq107_branch_derivatives,
            multiplicity,
        );
        let eq109_e1474: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, var_qdint);
        let eq109_e1475: f64 = (p.p7 * eq109_e1474);
        let eq109_e1475_d_n0: f64 = (p.p7 * (var_qdint_dn0 * ddt_scale));
        let eq109_e1475_d_n1: f64 = (p.p7 * (var_qdint_dn1 * ddt_scale));
        let eq109_e1475_d_n2: f64 = (p.p7 * (var_qdint_dn2 * ddt_scale));
        let eq109_e1475_d_n3: f64 = (p.p7 * (var_qdint_dn3 * ddt_scale));
        let eq109_e1475_d_n4: f64 = (p.p7 * (var_qdint_dn4 * ddt_scale));
        let eq109_e1475_d_n5: f64 = (p.p7 * (var_qdint_dn5 * ddt_scale));
        let eq109_e1475_d_n6: f64 = (p.p7 * (var_qdint_dn6 * ddt_scale));
        let eq109_e1475_d_n7: f64 = (p.p7 * (var_qdint_dn7 * ddt_scale));
        let eq109_e1475_d_n8: f64 = (p.p7 * (var_qdint_dn8 * ddt_scale));
        let eq109_e1475_d_n9: f64 = (p.p7 * (var_qdint_dn9 * ddt_scale));
        let eq109_e1475_d_n12: f64 = (p.p7 * (var_qdint_dn12 * ddt_scale));
        let eq109_e1475_d_n14: f64 = (p.p7 * (var_qdint_dn14 * ddt_scale));
        let eq109_e1475_d_n15: f64 = (p.p7 * (var_qdint_dn15 * ddt_scale));
        let eq109_e1475_d_n16: f64 = (p.p7 * (var_qdint_dn16 * ddt_scale));
        let eq109_e1475_d_n17: f64 = (p.p7 * (var_qdint_dn17 * ddt_scale));
        let eq109_e1475_d_n18: f64 = (p.p7 * (var_qdint_dn18 * ddt_scale));
        let eq109_e1475_d_n19: f64 = (p.p7 * (var_qdint_dn19 * ddt_scale));
        let eq109_e1475_d_n20: f64 = (p.p7 * (var_qdint_dn20 * ddt_scale));
        let eq109_e1475_d_n21: f64 = (p.p7 * (var_qdint_dn21 * ddt_scale));
        let eq109_e1475_d_n22: f64 = (p.p7 * (var_qdint_dn22 * ddt_scale));
        let eq109_value: f64 = eq109_e1475;
        let eq109_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq109_node_derivatives: [f64; 20] = [eq109_e1475_d_n0, eq109_e1475_d_n1, eq109_e1475_d_n2, eq109_e1475_d_n3, eq109_e1475_d_n4, eq109_e1475_d_n5, eq109_e1475_d_n6, eq109_e1475_d_n7, eq109_e1475_d_n8, eq109_e1475_d_n9, eq109_e1475_d_n12, eq109_e1475_d_n14, eq109_e1475_d_n15, eq109_e1475_d_n16, eq109_e1475_d_n17, eq109_e1475_d_n18, eq109_e1475_d_n19, eq109_e1475_d_n20, eq109_e1475_d_n21, eq109_e1475_d_n22];
        let eq109_branch_derivative_indices: [usize; 0] = [];
        let eq109_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq109_value),
            &eq109_node_derivative_indices,
            &eq109_node_derivatives,
            &eq109_branch_derivative_indices,
            &eq109_branch_derivatives,
            multiplicity,
        );
        let eq110_e1478: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, var_qgint);
        let eq110_e1479: f64 = (p.p7 * eq110_e1478);
        let eq110_e1479_d_n0: f64 = (p.p7 * (var_qgint_dn0 * ddt_scale));
        let eq110_e1479_d_n1: f64 = (p.p7 * (var_qgint_dn1 * ddt_scale));
        let eq110_e1479_d_n2: f64 = (p.p7 * (var_qgint_dn2 * ddt_scale));
        let eq110_e1479_d_n3: f64 = (p.p7 * (var_qgint_dn3 * ddt_scale));
        let eq110_e1479_d_n4: f64 = (p.p7 * (var_qgint_dn4 * ddt_scale));
        let eq110_e1479_d_n5: f64 = (p.p7 * (var_qgint_dn5 * ddt_scale));
        let eq110_e1479_d_n6: f64 = (p.p7 * (var_qgint_dn6 * ddt_scale));
        let eq110_e1479_d_n7: f64 = (p.p7 * (var_qgint_dn7 * ddt_scale));
        let eq110_e1479_d_n8: f64 = (p.p7 * (var_qgint_dn8 * ddt_scale));
        let eq110_e1479_d_n9: f64 = (p.p7 * (var_qgint_dn9 * ddt_scale));
        let eq110_e1479_d_n12: f64 = (p.p7 * (var_qgint_dn12 * ddt_scale));
        let eq110_e1479_d_n14: f64 = (p.p7 * (var_qgint_dn14 * ddt_scale));
        let eq110_e1479_d_n15: f64 = (p.p7 * (var_qgint_dn15 * ddt_scale));
        let eq110_e1479_d_n16: f64 = (p.p7 * (var_qgint_dn16 * ddt_scale));
        let eq110_e1479_d_n17: f64 = (p.p7 * (var_qgint_dn17 * ddt_scale));
        let eq110_e1479_d_n18: f64 = (p.p7 * (var_qgint_dn18 * ddt_scale));
        let eq110_e1479_d_n19: f64 = (p.p7 * (var_qgint_dn19 * ddt_scale));
        let eq110_e1479_d_n20: f64 = (p.p7 * (var_qgint_dn20 * ddt_scale));
        let eq110_e1479_d_n21: f64 = (p.p7 * (var_qgint_dn21 * ddt_scale));
        let eq110_e1479_d_n22: f64 = (p.p7 * (var_qgint_dn22 * ddt_scale));
        let eq110_value: f64 = eq110_e1479;
        let eq110_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq110_node_derivatives: [f64; 20] = [eq110_e1479_d_n0, eq110_e1479_d_n1, eq110_e1479_d_n2, eq110_e1479_d_n3, eq110_e1479_d_n4, eq110_e1479_d_n5, eq110_e1479_d_n6, eq110_e1479_d_n7, eq110_e1479_d_n8, eq110_e1479_d_n9, eq110_e1479_d_n12, eq110_e1479_d_n14, eq110_e1479_d_n15, eq110_e1479_d_n16, eq110_e1479_d_n17, eq110_e1479_d_n18, eq110_e1479_d_n19, eq110_e1479_d_n20, eq110_e1479_d_n21, eq110_e1479_d_n22];
        let eq110_branch_derivative_indices: [usize; 0] = [];
        let eq110_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq110_value),
            &eq110_node_derivative_indices,
            &eq110_node_derivatives,
            &eq110_branch_derivative_indices,
            &eq110_branch_derivatives,
            multiplicity,
        );
        let (eq111_e1486, eq111_e1486_d_n1, eq111_e1486_d_n2, eq111_e1486_d_n10,) = {
    if (var_guard535 != 0.0) {
        let eq111_e1483: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, var_qsov);
        let eq111_e1484: f64 = (p.p7 * eq111_e1483);
        let eq111_e1484_d_n1: f64 = (p.p7 * (var_qsov_dn1 * ddt_scale));
        let eq111_e1484_d_n2: f64 = (p.p7 * (var_qsov_dn2 * ddt_scale));
        let eq111_e1484_d_n10: f64 = (p.p7 * (var_qsov_dn10 * ddt_scale));
        (eq111_e1484, eq111_e1484_d_n1, eq111_e1484_d_n2, eq111_e1484_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_value: f64 = eq111_e1486;
        stamper.stamp_current_node3_local(
            Some(10),
            Some(2),
            multiplicity * (eq111_value),
            1,
            multiplicity * (eq111_e1486_d_n1),
            2,
            multiplicity * (eq111_e1486_d_n2),
            10,
            multiplicity * (eq111_e1486_d_n10),
        );
        let (eq112_e1493, eq112_e1493_d_n0, eq112_e1493_d_n1, eq112_e1493_d_n2, eq112_e1493_d_n10,) = {
    if (var_guard535 != 0.0) {
        let eq112_e1490: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, var_qdov);
        let eq112_e1491: f64 = (p.p7 * eq112_e1490);
        let eq112_e1491_d_n0: f64 = (p.p7 * (var_qdov_dn0 * ddt_scale));
        let eq112_e1491_d_n1: f64 = (p.p7 * (var_qdov_dn1 * ddt_scale));
        let eq112_e1491_d_n2: f64 = (p.p7 * (var_qdov_dn2 * ddt_scale));
        let eq112_e1491_d_n10: f64 = (p.p7 * (var_qdov_dn10 * ddt_scale));
        (eq112_e1491, eq112_e1491_d_n0, eq112_e1491_d_n1, eq112_e1491_d_n2, eq112_e1491_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq112_value: f64 = eq112_e1493;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(10),
            Some(0),
            multiplicity * (eq112_value),
            [0, 1, 2, 10],
            [multiplicity * (eq112_e1493_d_n0), multiplicity * (eq112_e1493_d_n1), multiplicity * (eq112_e1493_d_n2), multiplicity * (eq112_e1493_d_n10)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
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
        var_guard535: f64,
        var_guard536: f64,
        var_guard537: f64,
        var_guard538: f64,
        var_qbdov: f64,
        var_qbdov_dn0: f64,
        var_qbdov_dn3: f64,
        var_qbgov: f64,
        var_qbgov_dn1: f64,
        var_qbgov_dn3: f64,
        var_qbsov: f64,
        var_qbsov_dn2: f64,
        var_qbsov_dn3: f64,
        var_qd_fp1: f64,
        var_qd_fp1_dn0: f64,
        var_qd_fp1_dn1: f64,
        var_qd_fp1_dn12: f64,
        var_qd_fp1_dn14: f64,
        var_qd_fp1_dn15: f64,
        var_qd_fp1_dn16: f64,
        var_qd_fp1_dn17: f64,
        var_qd_fp1_dn18: f64,
        var_qd_fp1_dn19: f64,
        var_qd_fp1_dn2: f64,
        var_qd_fp1_dn20: f64,
        var_qd_fp1_dn21: f64,
        var_qd_fp1_dn22: f64,
        var_qd_fp1_dn3: f64,
        var_qd_fp1_dn4: f64,
        var_qd_fp1_dn5: f64,
        var_qd_fp1_dn6: f64,
        var_qd_fp1_dn7: f64,
        var_qd_fp1_dn8: f64,
        var_qd_fp1_dn9: f64,
        var_qdov: f64,
        var_qdov_dn0: f64,
        var_qdov_dn1: f64,
        var_qdov_dn10: f64,
        var_qdov_dn2: f64,
        var_qdsov: f64,
        var_qdsov_dn0: f64,
        var_qdsov_dn2: f64,
        var_qg_fp1: f64,
        var_qg_fp1_dn0: f64,
        var_qg_fp1_dn1: f64,
        var_qg_fp1_dn12: f64,
        var_qg_fp1_dn14: f64,
        var_qg_fp1_dn15: f64,
        var_qg_fp1_dn16: f64,
        var_qg_fp1_dn17: f64,
        var_qg_fp1_dn18: f64,
        var_qg_fp1_dn19: f64,
        var_qg_fp1_dn2: f64,
        var_qg_fp1_dn20: f64,
        var_qg_fp1_dn21: f64,
        var_qg_fp1_dn22: f64,
        var_qg_fp1_dn3: f64,
        var_qg_fp1_dn4: f64,
        var_qg_fp1_dn5: f64,
        var_qg_fp1_dn6: f64,
        var_qg_fp1_dn7: f64,
        var_qg_fp1_dn8: f64,
        var_qg_fp1_dn9: f64,
        var_qgint: f64,
        var_qgint_dn0: f64,
        var_qgint_dn1: f64,
        var_qgint_dn12: f64,
        var_qgint_dn14: f64,
        var_qgint_dn15: f64,
        var_qgint_dn16: f64,
        var_qgint_dn17: f64,
        var_qgint_dn18: f64,
        var_qgint_dn19: f64,
        var_qgint_dn2: f64,
        var_qgint_dn20: f64,
        var_qgint_dn21: f64,
        var_qgint_dn22: f64,
        var_qgint_dn3: f64,
        var_qgint_dn4: f64,
        var_qgint_dn5: f64,
        var_qgint_dn6: f64,
        var_qgint_dn7: f64,
        var_qgint_dn8: f64,
        var_qgint_dn9: f64,
        var_qsov: f64,
        var_qsov_dn1: f64,
        var_qsov_dn10: f64,
        var_qsov_dn2: f64,
    ) {
        let (eq113_e1501, eq113_e1501_d_n1, eq113_e1501_d_n2, eq113_e1501_d_n10,) = {
    if (var_guard535 == 0.0) {
        let eq113_e1498: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, var_qsov);
        let eq113_e1499: f64 = (p.p7 * eq113_e1498);
        let eq113_e1499_d_n1: f64 = (p.p7 * (var_qsov_dn1 * ddt_scale));
        let eq113_e1499_d_n2: f64 = (p.p7 * (var_qsov_dn2 * ddt_scale));
        let eq113_e1499_d_n10: f64 = (p.p7 * (var_qsov_dn10 * ddt_scale));
        (eq113_e1499, eq113_e1499_d_n1, eq113_e1499_d_n2, eq113_e1499_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq113_value: f64 = eq113_e1501;
        stamper.stamp_current_node3_local(
            Some(1),
            Some(2),
            multiplicity * (eq113_value),
            1,
            multiplicity * (eq113_e1501_d_n1),
            2,
            multiplicity * (eq113_e1501_d_n2),
            10,
            multiplicity * (eq113_e1501_d_n10),
        );
        let (eq114_e1509, eq114_e1509_d_n0, eq114_e1509_d_n1, eq114_e1509_d_n2, eq114_e1509_d_n10,) = {
    if (var_guard535 == 0.0) {
        let eq114_e1506: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, var_qdov);
        let eq114_e1507: f64 = (p.p7 * eq114_e1506);
        let eq114_e1507_d_n0: f64 = (p.p7 * (var_qdov_dn0 * ddt_scale));
        let eq114_e1507_d_n1: f64 = (p.p7 * (var_qdov_dn1 * ddt_scale));
        let eq114_e1507_d_n2: f64 = (p.p7 * (var_qdov_dn2 * ddt_scale));
        let eq114_e1507_d_n10: f64 = (p.p7 * (var_qdov_dn10 * ddt_scale));
        (eq114_e1507, eq114_e1507_d_n0, eq114_e1507_d_n1, eq114_e1507_d_n2, eq114_e1507_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq114_value: f64 = eq114_e1509;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(1),
            Some(0),
            multiplicity * (eq114_value),
            [0, 1, 2, 10],
            [multiplicity * (eq114_e1509_d_n0), multiplicity * (eq114_e1509_d_n1), multiplicity * (eq114_e1509_d_n2), multiplicity * (eq114_e1509_d_n10)],
            [],
            [],
            1.0,
        );
        let eq115_e1512: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, var_qdsov);
        let eq115_e1513: f64 = (p.p7 * eq115_e1512);
        let eq115_e1513_d_n0: f64 = (p.p7 * (var_qdsov_dn0 * ddt_scale));
        let eq115_e1513_d_n2: f64 = (p.p7 * (var_qdsov_dn2 * ddt_scale));
        let eq115_value: f64 = eq115_e1513;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(2),
            multiplicity * (eq115_value),
            0,
            multiplicity * (eq115_e1513_d_n0),
            2,
            multiplicity * (eq115_e1513_d_n2),
        );
        let eq116_e1516: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, var_qbdov);
        let eq116_e1517: f64 = (p.p7 * eq116_e1516);
        let eq116_e1517_d_n0: f64 = (p.p7 * (var_qbdov_dn0 * ddt_scale));
        let eq116_e1517_d_n3: f64 = (p.p7 * (var_qbdov_dn3 * ddt_scale));
        let eq116_value: f64 = eq116_e1517;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(0),
            multiplicity * (eq116_value),
            0,
            multiplicity * (eq116_e1517_d_n0),
            3,
            multiplicity * (eq116_e1517_d_n3),
        );
        let eq117_e1520: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, var_qbsov);
        let eq117_e1521: f64 = (p.p7 * eq117_e1520);
        let eq117_e1521_d_n2: f64 = (p.p7 * (var_qbsov_dn2 * ddt_scale));
        let eq117_e1521_d_n3: f64 = (p.p7 * (var_qbsov_dn3 * ddt_scale));
        let eq117_value: f64 = eq117_e1521;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(2),
            multiplicity * (eq117_value),
            2,
            multiplicity * (eq117_e1521_d_n2),
            3,
            multiplicity * (eq117_e1521_d_n3),
        );
        let eq118_e1524: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, var_qbgov);
        let eq118_e1525: f64 = (p.p7 * eq118_e1524);
        let eq118_e1525_d_n1: f64 = (p.p7 * (var_qbgov_dn1 * ddt_scale));
        let eq118_e1525_d_n3: f64 = (p.p7 * (var_qbgov_dn3 * ddt_scale));
        let eq118_value: f64 = eq118_e1525;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(1),
            multiplicity * (eq118_value),
            1,
            multiplicity * (eq118_e1525_d_n1),
            3,
            multiplicity * (eq118_e1525_d_n3),
        );
        let eq119_e1529: f64 = (p.p250 * var_qgint);
        let eq119_e1529_d_n0: f64 = (p.p250 * var_qgint_dn0);
        let eq119_e1529_d_n1: f64 = (p.p250 * var_qgint_dn1);
        let eq119_e1529_d_n2: f64 = (p.p250 * var_qgint_dn2);
        let eq119_e1529_d_n3: f64 = (p.p250 * var_qgint_dn3);
        let eq119_e1529_d_n4: f64 = (p.p250 * var_qgint_dn4);
        let eq119_e1529_d_n5: f64 = (p.p250 * var_qgint_dn5);
        let eq119_e1529_d_n6: f64 = (p.p250 * var_qgint_dn6);
        let eq119_e1529_d_n7: f64 = (p.p250 * var_qgint_dn7);
        let eq119_e1529_d_n8: f64 = (p.p250 * var_qgint_dn8);
        let eq119_e1529_d_n9: f64 = (p.p250 * var_qgint_dn9);
        let eq119_e1529_d_n12: f64 = (p.p250 * var_qgint_dn12);
        let eq119_e1529_d_n14: f64 = (p.p250 * var_qgint_dn14);
        let eq119_e1529_d_n15: f64 = (p.p250 * var_qgint_dn15);
        let eq119_e1529_d_n16: f64 = (p.p250 * var_qgint_dn16);
        let eq119_e1529_d_n17: f64 = (p.p250 * var_qgint_dn17);
        let eq119_e1529_d_n18: f64 = (p.p250 * var_qgint_dn18);
        let eq119_e1529_d_n19: f64 = (p.p250 * var_qgint_dn19);
        let eq119_e1529_d_n20: f64 = (p.p250 * var_qgint_dn20);
        let eq119_e1529_d_n21: f64 = (p.p250 * var_qgint_dn21);
        let eq119_e1529_d_n22: f64 = (p.p250 * var_qgint_dn22);
        let eq119_e1530: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq119_e1529);
        let eq119_e1531: f64 = (p.p7 * eq119_e1530);
        let eq119_e1531_d_n0: f64 = (p.p7 * (eq119_e1529_d_n0 * ddt_scale));
        let eq119_e1531_d_n1: f64 = (p.p7 * (eq119_e1529_d_n1 * ddt_scale));
        let eq119_e1531_d_n2: f64 = (p.p7 * (eq119_e1529_d_n2 * ddt_scale));
        let eq119_e1531_d_n3: f64 = (p.p7 * (eq119_e1529_d_n3 * ddt_scale));
        let eq119_e1531_d_n4: f64 = (p.p7 * (eq119_e1529_d_n4 * ddt_scale));
        let eq119_e1531_d_n5: f64 = (p.p7 * (eq119_e1529_d_n5 * ddt_scale));
        let eq119_e1531_d_n6: f64 = (p.p7 * (eq119_e1529_d_n6 * ddt_scale));
        let eq119_e1531_d_n7: f64 = (p.p7 * (eq119_e1529_d_n7 * ddt_scale));
        let eq119_e1531_d_n8: f64 = (p.p7 * (eq119_e1529_d_n8 * ddt_scale));
        let eq119_e1531_d_n9: f64 = (p.p7 * (eq119_e1529_d_n9 * ddt_scale));
        let eq119_e1531_d_n12: f64 = (p.p7 * (eq119_e1529_d_n12 * ddt_scale));
        let eq119_e1531_d_n14: f64 = (p.p7 * (eq119_e1529_d_n14 * ddt_scale));
        let eq119_e1531_d_n15: f64 = (p.p7 * (eq119_e1529_d_n15 * ddt_scale));
        let eq119_e1531_d_n16: f64 = (p.p7 * (eq119_e1529_d_n16 * ddt_scale));
        let eq119_e1531_d_n17: f64 = (p.p7 * (eq119_e1529_d_n17 * ddt_scale));
        let eq119_e1531_d_n18: f64 = (p.p7 * (eq119_e1529_d_n18 * ddt_scale));
        let eq119_e1531_d_n19: f64 = (p.p7 * (eq119_e1529_d_n19 * ddt_scale));
        let eq119_e1531_d_n20: f64 = (p.p7 * (eq119_e1529_d_n20 * ddt_scale));
        let eq119_e1531_d_n21: f64 = (p.p7 * (eq119_e1529_d_n21 * ddt_scale));
        let eq119_e1531_d_n22: f64 = (p.p7 * (eq119_e1529_d_n22 * ddt_scale));
        let eq119_value: f64 = eq119_e1531;
        let eq119_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq119_node_derivatives: [f64; 20] = [eq119_e1531_d_n0, eq119_e1531_d_n1, eq119_e1531_d_n2, eq119_e1531_d_n3, eq119_e1531_d_n4, eq119_e1531_d_n5, eq119_e1531_d_n6, eq119_e1531_d_n7, eq119_e1531_d_n8, eq119_e1531_d_n9, eq119_e1531_d_n12, eq119_e1531_d_n14, eq119_e1531_d_n15, eq119_e1531_d_n16, eq119_e1531_d_n17, eq119_e1531_d_n18, eq119_e1531_d_n19, eq119_e1531_d_n20, eq119_e1531_d_n21, eq119_e1531_d_n22];
        let eq119_branch_derivative_indices: [usize; 0] = [];
        let eq119_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(8),
            multiplicity * (eq119_value),
            &eq119_node_derivative_indices,
            &eq119_node_derivatives,
            &eq119_branch_derivative_indices,
            &eq119_branch_derivatives,
            multiplicity,
        );
        let (eq120_e1540, eq120_e1540_d_n0, eq120_e1540_d_n1, eq120_e1540_d_n2, eq120_e1540_d_n3, eq120_e1540_d_n4, eq120_e1540_d_n5, eq120_e1540_d_n6, eq120_e1540_d_n7, eq120_e1540_d_n8, eq120_e1540_d_n9, eq120_e1540_d_n12, eq120_e1540_d_n14, eq120_e1540_d_n15, eq120_e1540_d_n16, eq120_e1540_d_n17, eq120_e1540_d_n18, eq120_e1540_d_n19, eq120_e1540_d_n20, eq120_e1540_d_n21, eq120_e1540_d_n22,) = {
    if ((var_guard536 != 0.0) && (var_guard537 != 0.0)) {
        let eq120_e1537: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, var_qd_fp1);
        let eq120_e1538: f64 = (p.p7 * eq120_e1537);
        let eq120_e1538_d_n0: f64 = (p.p7 * (var_qd_fp1_dn0 * ddt_scale));
        let eq120_e1538_d_n1: f64 = (p.p7 * (var_qd_fp1_dn1 * ddt_scale));
        let eq120_e1538_d_n2: f64 = (p.p7 * (var_qd_fp1_dn2 * ddt_scale));
        let eq120_e1538_d_n3: f64 = (p.p7 * (var_qd_fp1_dn3 * ddt_scale));
        let eq120_e1538_d_n4: f64 = (p.p7 * (var_qd_fp1_dn4 * ddt_scale));
        let eq120_e1538_d_n5: f64 = (p.p7 * (var_qd_fp1_dn5 * ddt_scale));
        let eq120_e1538_d_n6: f64 = (p.p7 * (var_qd_fp1_dn6 * ddt_scale));
        let eq120_e1538_d_n7: f64 = (p.p7 * (var_qd_fp1_dn7 * ddt_scale));
        let eq120_e1538_d_n8: f64 = (p.p7 * (var_qd_fp1_dn8 * ddt_scale));
        let eq120_e1538_d_n9: f64 = (p.p7 * (var_qd_fp1_dn9 * ddt_scale));
        let eq120_e1538_d_n12: f64 = (p.p7 * (var_qd_fp1_dn12 * ddt_scale));
        let eq120_e1538_d_n14: f64 = (p.p7 * (var_qd_fp1_dn14 * ddt_scale));
        let eq120_e1538_d_n15: f64 = (p.p7 * (var_qd_fp1_dn15 * ddt_scale));
        let eq120_e1538_d_n16: f64 = (p.p7 * (var_qd_fp1_dn16 * ddt_scale));
        let eq120_e1538_d_n17: f64 = (p.p7 * (var_qd_fp1_dn17 * ddt_scale));
        let eq120_e1538_d_n18: f64 = (p.p7 * (var_qd_fp1_dn18 * ddt_scale));
        let eq120_e1538_d_n19: f64 = (p.p7 * (var_qd_fp1_dn19 * ddt_scale));
        let eq120_e1538_d_n20: f64 = (p.p7 * (var_qd_fp1_dn20 * ddt_scale));
        let eq120_e1538_d_n21: f64 = (p.p7 * (var_qd_fp1_dn21 * ddt_scale));
        let eq120_e1538_d_n22: f64 = (p.p7 * (var_qd_fp1_dn22 * ddt_scale));
        (eq120_e1538, eq120_e1538_d_n0, eq120_e1538_d_n1, eq120_e1538_d_n2, eq120_e1538_d_n3, eq120_e1538_d_n4, eq120_e1538_d_n5, eq120_e1538_d_n6, eq120_e1538_d_n7, eq120_e1538_d_n8, eq120_e1538_d_n9, eq120_e1538_d_n12, eq120_e1538_d_n14, eq120_e1538_d_n15, eq120_e1538_d_n16, eq120_e1538_d_n17, eq120_e1538_d_n18, eq120_e1538_d_n19, eq120_e1538_d_n20, eq120_e1538_d_n21, eq120_e1538_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq120_value: f64 = eq120_e1540;
        let eq120_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq120_node_derivatives: [f64; 20] = [eq120_e1540_d_n0, eq120_e1540_d_n1, eq120_e1540_d_n2, eq120_e1540_d_n3, eq120_e1540_d_n4, eq120_e1540_d_n5, eq120_e1540_d_n6, eq120_e1540_d_n7, eq120_e1540_d_n8, eq120_e1540_d_n9, eq120_e1540_d_n12, eq120_e1540_d_n14, eq120_e1540_d_n15, eq120_e1540_d_n16, eq120_e1540_d_n17, eq120_e1540_d_n18, eq120_e1540_d_n19, eq120_e1540_d_n20, eq120_e1540_d_n21, eq120_e1540_d_n22];
        let eq120_branch_derivative_indices: [usize; 0] = [];
        let eq120_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(15),
            Some(7),
            multiplicity * (eq120_value),
            &eq120_node_derivative_indices,
            &eq120_node_derivatives,
            &eq120_branch_derivative_indices,
            &eq120_branch_derivatives,
            multiplicity,
        );
        let (eq121_e1551, eq121_e1551_d_n0, eq121_e1551_d_n1, eq121_e1551_d_n2, eq121_e1551_d_n3, eq121_e1551_d_n4, eq121_e1551_d_n5, eq121_e1551_d_n6, eq121_e1551_d_n7, eq121_e1551_d_n8, eq121_e1551_d_n9, eq121_e1551_d_n12, eq121_e1551_d_n14, eq121_e1551_d_n15, eq121_e1551_d_n16, eq121_e1551_d_n17, eq121_e1551_d_n18, eq121_e1551_d_n19, eq121_e1551_d_n20, eq121_e1551_d_n21, eq121_e1551_d_n22,) = {
    if (((var_guard536 != 0.0) && (var_guard537 != 0.0)) && (var_guard538 != 0.0)) {
        let eq121_e1548: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, var_qg_fp1);
        let eq121_e1549: f64 = (p.p7 * eq121_e1548);
        let eq121_e1549_d_n0: f64 = (p.p7 * (var_qg_fp1_dn0 * ddt_scale));
        let eq121_e1549_d_n1: f64 = (p.p7 * (var_qg_fp1_dn1 * ddt_scale));
        let eq121_e1549_d_n2: f64 = (p.p7 * (var_qg_fp1_dn2 * ddt_scale));
        let eq121_e1549_d_n3: f64 = (p.p7 * (var_qg_fp1_dn3 * ddt_scale));
        let eq121_e1549_d_n4: f64 = (p.p7 * (var_qg_fp1_dn4 * ddt_scale));
        let eq121_e1549_d_n5: f64 = (p.p7 * (var_qg_fp1_dn5 * ddt_scale));
        let eq121_e1549_d_n6: f64 = (p.p7 * (var_qg_fp1_dn6 * ddt_scale));
        let eq121_e1549_d_n7: f64 = (p.p7 * (var_qg_fp1_dn7 * ddt_scale));
        let eq121_e1549_d_n8: f64 = (p.p7 * (var_qg_fp1_dn8 * ddt_scale));
        let eq121_e1549_d_n9: f64 = (p.p7 * (var_qg_fp1_dn9 * ddt_scale));
        let eq121_e1549_d_n12: f64 = (p.p7 * (var_qg_fp1_dn12 * ddt_scale));
        let eq121_e1549_d_n14: f64 = (p.p7 * (var_qg_fp1_dn14 * ddt_scale));
        let eq121_e1549_d_n15: f64 = (p.p7 * (var_qg_fp1_dn15 * ddt_scale));
        let eq121_e1549_d_n16: f64 = (p.p7 * (var_qg_fp1_dn16 * ddt_scale));
        let eq121_e1549_d_n17: f64 = (p.p7 * (var_qg_fp1_dn17 * ddt_scale));
        let eq121_e1549_d_n18: f64 = (p.p7 * (var_qg_fp1_dn18 * ddt_scale));
        let eq121_e1549_d_n19: f64 = (p.p7 * (var_qg_fp1_dn19 * ddt_scale));
        let eq121_e1549_d_n20: f64 = (p.p7 * (var_qg_fp1_dn20 * ddt_scale));
        let eq121_e1549_d_n21: f64 = (p.p7 * (var_qg_fp1_dn21 * ddt_scale));
        let eq121_e1549_d_n22: f64 = (p.p7 * (var_qg_fp1_dn22 * ddt_scale));
        (eq121_e1549, eq121_e1549_d_n0, eq121_e1549_d_n1, eq121_e1549_d_n2, eq121_e1549_d_n3, eq121_e1549_d_n4, eq121_e1549_d_n5, eq121_e1549_d_n6, eq121_e1549_d_n7, eq121_e1549_d_n8, eq121_e1549_d_n9, eq121_e1549_d_n12, eq121_e1549_d_n14, eq121_e1549_d_n15, eq121_e1549_d_n16, eq121_e1549_d_n17, eq121_e1549_d_n18, eq121_e1549_d_n19, eq121_e1549_d_n20, eq121_e1549_d_n21, eq121_e1549_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq121_value: f64 = eq121_e1551;
        let eq121_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq121_node_derivatives: [f64; 20] = [eq121_e1551_d_n0, eq121_e1551_d_n1, eq121_e1551_d_n2, eq121_e1551_d_n3, eq121_e1551_d_n4, eq121_e1551_d_n5, eq121_e1551_d_n6, eq121_e1551_d_n7, eq121_e1551_d_n8, eq121_e1551_d_n9, eq121_e1551_d_n12, eq121_e1551_d_n14, eq121_e1551_d_n15, eq121_e1551_d_n16, eq121_e1551_d_n17, eq121_e1551_d_n18, eq121_e1551_d_n19, eq121_e1551_d_n20, eq121_e1551_d_n21, eq121_e1551_d_n22];
        let eq121_branch_derivative_indices: [usize; 0] = [];
        let eq121_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq121_value),
            &eq121_node_derivative_indices,
            &eq121_node_derivatives,
            &eq121_branch_derivative_indices,
            &eq121_branch_derivatives,
            multiplicity,
        );
        let (eq122_e1564, eq122_e1564_d_n0, eq122_e1564_d_n1, eq122_e1564_d_n2, eq122_e1564_d_n3, eq122_e1564_d_n4, eq122_e1564_d_n5, eq122_e1564_d_n6, eq122_e1564_d_n7, eq122_e1564_d_n8, eq122_e1564_d_n9, eq122_e1564_d_n12, eq122_e1564_d_n14, eq122_e1564_d_n15, eq122_e1564_d_n16, eq122_e1564_d_n17, eq122_e1564_d_n18, eq122_e1564_d_n19, eq122_e1564_d_n20, eq122_e1564_d_n21, eq122_e1564_d_n22,) = {
    if (((var_guard536 != 0.0) && (var_guard537 != 0.0)) && (var_guard538 != 0.0)) {
        let eq122_e1559: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 21, var_qg_fp1);
        let eq122_e1560: f64 = (p.p7 * eq122_e1559);
        let eq122_e1560_d_n0: f64 = (p.p7 * (var_qg_fp1_dn0 * ddt_scale));
        let eq122_e1560_d_n1: f64 = (p.p7 * (var_qg_fp1_dn1 * ddt_scale));
        let eq122_e1560_d_n2: f64 = (p.p7 * (var_qg_fp1_dn2 * ddt_scale));
        let eq122_e1560_d_n3: f64 = (p.p7 * (var_qg_fp1_dn3 * ddt_scale));
        let eq122_e1560_d_n4: f64 = (p.p7 * (var_qg_fp1_dn4 * ddt_scale));
        let eq122_e1560_d_n5: f64 = (p.p7 * (var_qg_fp1_dn5 * ddt_scale));
        let eq122_e1560_d_n6: f64 = (p.p7 * (var_qg_fp1_dn6 * ddt_scale));
        let eq122_e1560_d_n7: f64 = (p.p7 * (var_qg_fp1_dn7 * ddt_scale));
        let eq122_e1560_d_n8: f64 = (p.p7 * (var_qg_fp1_dn8 * ddt_scale));
        let eq122_e1560_d_n9: f64 = (p.p7 * (var_qg_fp1_dn9 * ddt_scale));
        let eq122_e1560_d_n12: f64 = (p.p7 * (var_qg_fp1_dn12 * ddt_scale));
        let eq122_e1560_d_n14: f64 = (p.p7 * (var_qg_fp1_dn14 * ddt_scale));
        let eq122_e1560_d_n15: f64 = (p.p7 * (var_qg_fp1_dn15 * ddt_scale));
        let eq122_e1560_d_n16: f64 = (p.p7 * (var_qg_fp1_dn16 * ddt_scale));
        let eq122_e1560_d_n17: f64 = (p.p7 * (var_qg_fp1_dn17 * ddt_scale));
        let eq122_e1560_d_n18: f64 = (p.p7 * (var_qg_fp1_dn18 * ddt_scale));
        let eq122_e1560_d_n19: f64 = (p.p7 * (var_qg_fp1_dn19 * ddt_scale));
        let eq122_e1560_d_n20: f64 = (p.p7 * (var_qg_fp1_dn20 * ddt_scale));
        let eq122_e1560_d_n21: f64 = (p.p7 * (var_qg_fp1_dn21 * ddt_scale));
        let eq122_e1560_d_n22: f64 = (p.p7 * (var_qg_fp1_dn22 * ddt_scale));
        let eq122_e1562: f64 = (eq122_e1560 * p.p246);
        let eq122_e1562_d_n0: f64 = (eq122_e1560_d_n0 * p.p246);
        let eq122_e1562_d_n1: f64 = (eq122_e1560_d_n1 * p.p246);
        let eq122_e1562_d_n2: f64 = (eq122_e1560_d_n2 * p.p246);
        let eq122_e1562_d_n3: f64 = (eq122_e1560_d_n3 * p.p246);
        let eq122_e1562_d_n4: f64 = (eq122_e1560_d_n4 * p.p246);
        let eq122_e1562_d_n5: f64 = (eq122_e1560_d_n5 * p.p246);
        let eq122_e1562_d_n6: f64 = (eq122_e1560_d_n6 * p.p246);
        let eq122_e1562_d_n7: f64 = (eq122_e1560_d_n7 * p.p246);
        let eq122_e1562_d_n8: f64 = (eq122_e1560_d_n8 * p.p246);
        let eq122_e1562_d_n9: f64 = (eq122_e1560_d_n9 * p.p246);
        let eq122_e1562_d_n12: f64 = (eq122_e1560_d_n12 * p.p246);
        let eq122_e1562_d_n14: f64 = (eq122_e1560_d_n14 * p.p246);
        let eq122_e1562_d_n15: f64 = (eq122_e1560_d_n15 * p.p246);
        let eq122_e1562_d_n16: f64 = (eq122_e1560_d_n16 * p.p246);
        let eq122_e1562_d_n17: f64 = (eq122_e1560_d_n17 * p.p246);
        let eq122_e1562_d_n18: f64 = (eq122_e1560_d_n18 * p.p246);
        let eq122_e1562_d_n19: f64 = (eq122_e1560_d_n19 * p.p246);
        let eq122_e1562_d_n20: f64 = (eq122_e1560_d_n20 * p.p246);
        let eq122_e1562_d_n21: f64 = (eq122_e1560_d_n21 * p.p246);
        let eq122_e1562_d_n22: f64 = (eq122_e1560_d_n22 * p.p246);
        (eq122_e1562, eq122_e1562_d_n0, eq122_e1562_d_n1, eq122_e1562_d_n2, eq122_e1562_d_n3, eq122_e1562_d_n4, eq122_e1562_d_n5, eq122_e1562_d_n6, eq122_e1562_d_n7, eq122_e1562_d_n8, eq122_e1562_d_n9, eq122_e1562_d_n12, eq122_e1562_d_n14, eq122_e1562_d_n15, eq122_e1562_d_n16, eq122_e1562_d_n17, eq122_e1562_d_n18, eq122_e1562_d_n19, eq122_e1562_d_n20, eq122_e1562_d_n21, eq122_e1562_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq122_value: f64 = eq122_e1564;
        let eq122_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq122_node_derivatives: [f64; 20] = [eq122_e1564_d_n0, eq122_e1564_d_n1, eq122_e1564_d_n2, eq122_e1564_d_n3, eq122_e1564_d_n4, eq122_e1564_d_n5, eq122_e1564_d_n6, eq122_e1564_d_n7, eq122_e1564_d_n8, eq122_e1564_d_n9, eq122_e1564_d_n12, eq122_e1564_d_n14, eq122_e1564_d_n15, eq122_e1564_d_n16, eq122_e1564_d_n17, eq122_e1564_d_n18, eq122_e1564_d_n19, eq122_e1564_d_n20, eq122_e1564_d_n21, eq122_e1564_d_n22];
        let eq122_branch_derivative_indices: [usize; 0] = [];
        let eq122_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq122_value),
            &eq122_node_derivative_indices,
            &eq122_node_derivatives,
            &eq122_branch_derivative_indices,
            &eq122_branch_derivatives,
            multiplicity,
        );
        let (eq123_e1576, eq123_e1576_d_n0, eq123_e1576_d_n1, eq123_e1576_d_n2, eq123_e1576_d_n3, eq123_e1576_d_n4, eq123_e1576_d_n5, eq123_e1576_d_n6, eq123_e1576_d_n7, eq123_e1576_d_n8, eq123_e1576_d_n9, eq123_e1576_d_n12, eq123_e1576_d_n14, eq123_e1576_d_n15, eq123_e1576_d_n16, eq123_e1576_d_n17, eq123_e1576_d_n18, eq123_e1576_d_n19, eq123_e1576_d_n20, eq123_e1576_d_n21, eq123_e1576_d_n22,) = {
    if (((var_guard536 != 0.0) && (var_guard537 != 0.0)) && (var_guard538 == 0.0)) {
        let eq123_e1573: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 22, var_qg_fp1);
        let eq123_e1574: f64 = (p.p7 * eq123_e1573);
        let eq123_e1574_d_n0: f64 = (p.p7 * (var_qg_fp1_dn0 * ddt_scale));
        let eq123_e1574_d_n1: f64 = (p.p7 * (var_qg_fp1_dn1 * ddt_scale));
        let eq123_e1574_d_n2: f64 = (p.p7 * (var_qg_fp1_dn2 * ddt_scale));
        let eq123_e1574_d_n3: f64 = (p.p7 * (var_qg_fp1_dn3 * ddt_scale));
        let eq123_e1574_d_n4: f64 = (p.p7 * (var_qg_fp1_dn4 * ddt_scale));
        let eq123_e1574_d_n5: f64 = (p.p7 * (var_qg_fp1_dn5 * ddt_scale));
        let eq123_e1574_d_n6: f64 = (p.p7 * (var_qg_fp1_dn6 * ddt_scale));
        let eq123_e1574_d_n7: f64 = (p.p7 * (var_qg_fp1_dn7 * ddt_scale));
        let eq123_e1574_d_n8: f64 = (p.p7 * (var_qg_fp1_dn8 * ddt_scale));
        let eq123_e1574_d_n9: f64 = (p.p7 * (var_qg_fp1_dn9 * ddt_scale));
        let eq123_e1574_d_n12: f64 = (p.p7 * (var_qg_fp1_dn12 * ddt_scale));
        let eq123_e1574_d_n14: f64 = (p.p7 * (var_qg_fp1_dn14 * ddt_scale));
        let eq123_e1574_d_n15: f64 = (p.p7 * (var_qg_fp1_dn15 * ddt_scale));
        let eq123_e1574_d_n16: f64 = (p.p7 * (var_qg_fp1_dn16 * ddt_scale));
        let eq123_e1574_d_n17: f64 = (p.p7 * (var_qg_fp1_dn17 * ddt_scale));
        let eq123_e1574_d_n18: f64 = (p.p7 * (var_qg_fp1_dn18 * ddt_scale));
        let eq123_e1574_d_n19: f64 = (p.p7 * (var_qg_fp1_dn19 * ddt_scale));
        let eq123_e1574_d_n20: f64 = (p.p7 * (var_qg_fp1_dn20 * ddt_scale));
        let eq123_e1574_d_n21: f64 = (p.p7 * (var_qg_fp1_dn21 * ddt_scale));
        let eq123_e1574_d_n22: f64 = (p.p7 * (var_qg_fp1_dn22 * ddt_scale));
        (eq123_e1574, eq123_e1574_d_n0, eq123_e1574_d_n1, eq123_e1574_d_n2, eq123_e1574_d_n3, eq123_e1574_d_n4, eq123_e1574_d_n5, eq123_e1574_d_n6, eq123_e1574_d_n7, eq123_e1574_d_n8, eq123_e1574_d_n9, eq123_e1574_d_n12, eq123_e1574_d_n14, eq123_e1574_d_n15, eq123_e1574_d_n16, eq123_e1574_d_n17, eq123_e1574_d_n18, eq123_e1574_d_n19, eq123_e1574_d_n20, eq123_e1574_d_n21, eq123_e1574_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq123_value: f64 = eq123_e1576;
        let eq123_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq123_node_derivatives: [f64; 20] = [eq123_e1576_d_n0, eq123_e1576_d_n1, eq123_e1576_d_n2, eq123_e1576_d_n3, eq123_e1576_d_n4, eq123_e1576_d_n5, eq123_e1576_d_n6, eq123_e1576_d_n7, eq123_e1576_d_n8, eq123_e1576_d_n9, eq123_e1576_d_n12, eq123_e1576_d_n14, eq123_e1576_d_n15, eq123_e1576_d_n16, eq123_e1576_d_n17, eq123_e1576_d_n18, eq123_e1576_d_n19, eq123_e1576_d_n20, eq123_e1576_d_n21, eq123_e1576_d_n22];
        let eq123_branch_derivative_indices: [usize; 0] = [];
        let eq123_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq123_value),
            &eq123_node_derivative_indices,
            &eq123_node_derivatives,
            &eq123_branch_derivative_indices,
            &eq123_branch_derivatives,
            multiplicity,
        );
        let (eq124_e1590, eq124_e1590_d_n0, eq124_e1590_d_n1, eq124_e1590_d_n2, eq124_e1590_d_n3, eq124_e1590_d_n4, eq124_e1590_d_n5, eq124_e1590_d_n6, eq124_e1590_d_n7, eq124_e1590_d_n8, eq124_e1590_d_n9, eq124_e1590_d_n12, eq124_e1590_d_n14, eq124_e1590_d_n15, eq124_e1590_d_n16, eq124_e1590_d_n17, eq124_e1590_d_n18, eq124_e1590_d_n19, eq124_e1590_d_n20, eq124_e1590_d_n21, eq124_e1590_d_n22,) = {
    if (((var_guard536 != 0.0) && (var_guard537 != 0.0)) && (var_guard538 == 0.0)) {
        let eq124_e1585: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 23, var_qg_fp1);
        let eq124_e1586: f64 = (p.p7 * eq124_e1585);
        let eq124_e1586_d_n0: f64 = (p.p7 * (var_qg_fp1_dn0 * ddt_scale));
        let eq124_e1586_d_n1: f64 = (p.p7 * (var_qg_fp1_dn1 * ddt_scale));
        let eq124_e1586_d_n2: f64 = (p.p7 * (var_qg_fp1_dn2 * ddt_scale));
        let eq124_e1586_d_n3: f64 = (p.p7 * (var_qg_fp1_dn3 * ddt_scale));
        let eq124_e1586_d_n4: f64 = (p.p7 * (var_qg_fp1_dn4 * ddt_scale));
        let eq124_e1586_d_n5: f64 = (p.p7 * (var_qg_fp1_dn5 * ddt_scale));
        let eq124_e1586_d_n6: f64 = (p.p7 * (var_qg_fp1_dn6 * ddt_scale));
        let eq124_e1586_d_n7: f64 = (p.p7 * (var_qg_fp1_dn7 * ddt_scale));
        let eq124_e1586_d_n8: f64 = (p.p7 * (var_qg_fp1_dn8 * ddt_scale));
        let eq124_e1586_d_n9: f64 = (p.p7 * (var_qg_fp1_dn9 * ddt_scale));
        let eq124_e1586_d_n12: f64 = (p.p7 * (var_qg_fp1_dn12 * ddt_scale));
        let eq124_e1586_d_n14: f64 = (p.p7 * (var_qg_fp1_dn14 * ddt_scale));
        let eq124_e1586_d_n15: f64 = (p.p7 * (var_qg_fp1_dn15 * ddt_scale));
        let eq124_e1586_d_n16: f64 = (p.p7 * (var_qg_fp1_dn16 * ddt_scale));
        let eq124_e1586_d_n17: f64 = (p.p7 * (var_qg_fp1_dn17 * ddt_scale));
        let eq124_e1586_d_n18: f64 = (p.p7 * (var_qg_fp1_dn18 * ddt_scale));
        let eq124_e1586_d_n19: f64 = (p.p7 * (var_qg_fp1_dn19 * ddt_scale));
        let eq124_e1586_d_n20: f64 = (p.p7 * (var_qg_fp1_dn20 * ddt_scale));
        let eq124_e1586_d_n21: f64 = (p.p7 * (var_qg_fp1_dn21 * ddt_scale));
        let eq124_e1586_d_n22: f64 = (p.p7 * (var_qg_fp1_dn22 * ddt_scale));
        let eq124_e1588: f64 = (eq124_e1586 * p.p246);
        let eq124_e1588_d_n0: f64 = (eq124_e1586_d_n0 * p.p246);
        let eq124_e1588_d_n1: f64 = (eq124_e1586_d_n1 * p.p246);
        let eq124_e1588_d_n2: f64 = (eq124_e1586_d_n2 * p.p246);
        let eq124_e1588_d_n3: f64 = (eq124_e1586_d_n3 * p.p246);
        let eq124_e1588_d_n4: f64 = (eq124_e1586_d_n4 * p.p246);
        let eq124_e1588_d_n5: f64 = (eq124_e1586_d_n5 * p.p246);
        let eq124_e1588_d_n6: f64 = (eq124_e1586_d_n6 * p.p246);
        let eq124_e1588_d_n7: f64 = (eq124_e1586_d_n7 * p.p246);
        let eq124_e1588_d_n8: f64 = (eq124_e1586_d_n8 * p.p246);
        let eq124_e1588_d_n9: f64 = (eq124_e1586_d_n9 * p.p246);
        let eq124_e1588_d_n12: f64 = (eq124_e1586_d_n12 * p.p246);
        let eq124_e1588_d_n14: f64 = (eq124_e1586_d_n14 * p.p246);
        let eq124_e1588_d_n15: f64 = (eq124_e1586_d_n15 * p.p246);
        let eq124_e1588_d_n16: f64 = (eq124_e1586_d_n16 * p.p246);
        let eq124_e1588_d_n17: f64 = (eq124_e1586_d_n17 * p.p246);
        let eq124_e1588_d_n18: f64 = (eq124_e1586_d_n18 * p.p246);
        let eq124_e1588_d_n19: f64 = (eq124_e1586_d_n19 * p.p246);
        let eq124_e1588_d_n20: f64 = (eq124_e1586_d_n20 * p.p246);
        let eq124_e1588_d_n21: f64 = (eq124_e1586_d_n21 * p.p246);
        let eq124_e1588_d_n22: f64 = (eq124_e1586_d_n22 * p.p246);
        (eq124_e1588, eq124_e1588_d_n0, eq124_e1588_d_n1, eq124_e1588_d_n2, eq124_e1588_d_n3, eq124_e1588_d_n4, eq124_e1588_d_n5, eq124_e1588_d_n6, eq124_e1588_d_n7, eq124_e1588_d_n8, eq124_e1588_d_n9, eq124_e1588_d_n12, eq124_e1588_d_n14, eq124_e1588_d_n15, eq124_e1588_d_n16, eq124_e1588_d_n17, eq124_e1588_d_n18, eq124_e1588_d_n19, eq124_e1588_d_n20, eq124_e1588_d_n21, eq124_e1588_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq124_value: f64 = eq124_e1590;
        let eq124_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq124_node_derivatives: [f64; 20] = [eq124_e1590_d_n0, eq124_e1590_d_n1, eq124_e1590_d_n2, eq124_e1590_d_n3, eq124_e1590_d_n4, eq124_e1590_d_n5, eq124_e1590_d_n6, eq124_e1590_d_n7, eq124_e1590_d_n8, eq124_e1590_d_n9, eq124_e1590_d_n12, eq124_e1590_d_n14, eq124_e1590_d_n15, eq124_e1590_d_n16, eq124_e1590_d_n17, eq124_e1590_d_n18, eq124_e1590_d_n19, eq124_e1590_d_n20, eq124_e1590_d_n21, eq124_e1590_d_n22];
        let eq124_branch_derivative_indices: [usize; 0] = [];
        let eq124_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq124_value),
            &eq124_node_derivative_indices,
            &eq124_node_derivatives,
            &eq124_branch_derivative_indices,
            &eq124_branch_derivatives,
            multiplicity,
        );
        let (eq125_e1601, eq125_e1601_d_n0, eq125_e1601_d_n1, eq125_e1601_d_n2, eq125_e1601_d_n3, eq125_e1601_d_n4, eq125_e1601_d_n5, eq125_e1601_d_n6, eq125_e1601_d_n7, eq125_e1601_d_n8, eq125_e1601_d_n9, eq125_e1601_d_n12, eq125_e1601_d_n14, eq125_e1601_d_n15, eq125_e1601_d_n16, eq125_e1601_d_n17, eq125_e1601_d_n18, eq125_e1601_d_n19, eq125_e1601_d_n20, eq125_e1601_d_n21, eq125_e1601_d_n22,) = {
    if ((var_guard536 != 0.0) && (var_guard537 != 0.0)) {
        let eq125_e1597: f64 = (p.p251 * var_qg_fp1);
        let eq125_e1597_d_n0: f64 = (p.p251 * var_qg_fp1_dn0);
        let eq125_e1597_d_n1: f64 = (p.p251 * var_qg_fp1_dn1);
        let eq125_e1597_d_n2: f64 = (p.p251 * var_qg_fp1_dn2);
        let eq125_e1597_d_n3: f64 = (p.p251 * var_qg_fp1_dn3);
        let eq125_e1597_d_n4: f64 = (p.p251 * var_qg_fp1_dn4);
        let eq125_e1597_d_n5: f64 = (p.p251 * var_qg_fp1_dn5);
        let eq125_e1597_d_n6: f64 = (p.p251 * var_qg_fp1_dn6);
        let eq125_e1597_d_n7: f64 = (p.p251 * var_qg_fp1_dn7);
        let eq125_e1597_d_n8: f64 = (p.p251 * var_qg_fp1_dn8);
        let eq125_e1597_d_n9: f64 = (p.p251 * var_qg_fp1_dn9);
        let eq125_e1597_d_n12: f64 = (p.p251 * var_qg_fp1_dn12);
        let eq125_e1597_d_n14: f64 = (p.p251 * var_qg_fp1_dn14);
        let eq125_e1597_d_n15: f64 = (p.p251 * var_qg_fp1_dn15);
        let eq125_e1597_d_n16: f64 = (p.p251 * var_qg_fp1_dn16);
        let eq125_e1597_d_n17: f64 = (p.p251 * var_qg_fp1_dn17);
        let eq125_e1597_d_n18: f64 = (p.p251 * var_qg_fp1_dn18);
        let eq125_e1597_d_n19: f64 = (p.p251 * var_qg_fp1_dn19);
        let eq125_e1597_d_n20: f64 = (p.p251 * var_qg_fp1_dn20);
        let eq125_e1597_d_n21: f64 = (p.p251 * var_qg_fp1_dn21);
        let eq125_e1597_d_n22: f64 = (p.p251 * var_qg_fp1_dn22);
        let eq125_e1598: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 24, eq125_e1597);
        let eq125_e1599: f64 = (p.p7 * eq125_e1598);
        let eq125_e1599_d_n0: f64 = (p.p7 * (eq125_e1597_d_n0 * ddt_scale));
        let eq125_e1599_d_n1: f64 = (p.p7 * (eq125_e1597_d_n1 * ddt_scale));
        let eq125_e1599_d_n2: f64 = (p.p7 * (eq125_e1597_d_n2 * ddt_scale));
        let eq125_e1599_d_n3: f64 = (p.p7 * (eq125_e1597_d_n3 * ddt_scale));
        let eq125_e1599_d_n4: f64 = (p.p7 * (eq125_e1597_d_n4 * ddt_scale));
        let eq125_e1599_d_n5: f64 = (p.p7 * (eq125_e1597_d_n5 * ddt_scale));
        let eq125_e1599_d_n6: f64 = (p.p7 * (eq125_e1597_d_n6 * ddt_scale));
        let eq125_e1599_d_n7: f64 = (p.p7 * (eq125_e1597_d_n7 * ddt_scale));
        let eq125_e1599_d_n8: f64 = (p.p7 * (eq125_e1597_d_n8 * ddt_scale));
        let eq125_e1599_d_n9: f64 = (p.p7 * (eq125_e1597_d_n9 * ddt_scale));
        let eq125_e1599_d_n12: f64 = (p.p7 * (eq125_e1597_d_n12 * ddt_scale));
        let eq125_e1599_d_n14: f64 = (p.p7 * (eq125_e1597_d_n14 * ddt_scale));
        let eq125_e1599_d_n15: f64 = (p.p7 * (eq125_e1597_d_n15 * ddt_scale));
        let eq125_e1599_d_n16: f64 = (p.p7 * (eq125_e1597_d_n16 * ddt_scale));
        let eq125_e1599_d_n17: f64 = (p.p7 * (eq125_e1597_d_n17 * ddt_scale));
        let eq125_e1599_d_n18: f64 = (p.p7 * (eq125_e1597_d_n18 * ddt_scale));
        let eq125_e1599_d_n19: f64 = (p.p7 * (eq125_e1597_d_n19 * ddt_scale));
        let eq125_e1599_d_n20: f64 = (p.p7 * (eq125_e1597_d_n20 * ddt_scale));
        let eq125_e1599_d_n21: f64 = (p.p7 * (eq125_e1597_d_n21 * ddt_scale));
        let eq125_e1599_d_n22: f64 = (p.p7 * (eq125_e1597_d_n22 * ddt_scale));
        (eq125_e1599, eq125_e1599_d_n0, eq125_e1599_d_n1, eq125_e1599_d_n2, eq125_e1599_d_n3, eq125_e1599_d_n4, eq125_e1599_d_n5, eq125_e1599_d_n6, eq125_e1599_d_n7, eq125_e1599_d_n8, eq125_e1599_d_n9, eq125_e1599_d_n12, eq125_e1599_d_n14, eq125_e1599_d_n15, eq125_e1599_d_n16, eq125_e1599_d_n17, eq125_e1599_d_n18, eq125_e1599_d_n19, eq125_e1599_d_n20, eq125_e1599_d_n21, eq125_e1599_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq125_value: f64 = eq125_e1601;
        let eq125_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq125_node_derivatives: [f64; 20] = [eq125_e1601_d_n0, eq125_e1601_d_n1, eq125_e1601_d_n2, eq125_e1601_d_n3, eq125_e1601_d_n4, eq125_e1601_d_n5, eq125_e1601_d_n6, eq125_e1601_d_n7, eq125_e1601_d_n8, eq125_e1601_d_n9, eq125_e1601_d_n12, eq125_e1601_d_n14, eq125_e1601_d_n15, eq125_e1601_d_n16, eq125_e1601_d_n17, eq125_e1601_d_n18, eq125_e1601_d_n19, eq125_e1601_d_n20, eq125_e1601_d_n21, eq125_e1601_d_n22];
        let eq125_branch_derivative_indices: [usize; 0] = [];
        let eq125_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(7),
            multiplicity * (eq125_value),
            &eq125_node_derivative_indices,
            &eq125_node_derivatives,
            &eq125_branch_derivative_indices,
            &eq125_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
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
        var_guard536: f64,
        var_guard539: f64,
        var_guard540: f64,
        var_guard541: f64,
        var_guard542: f64,
        var_guard543: f64,
        var_qd_fp1: f64,
        var_qd_fp1_dn0: f64,
        var_qd_fp1_dn1: f64,
        var_qd_fp1_dn12: f64,
        var_qd_fp1_dn14: f64,
        var_qd_fp1_dn15: f64,
        var_qd_fp1_dn16: f64,
        var_qd_fp1_dn17: f64,
        var_qd_fp1_dn18: f64,
        var_qd_fp1_dn19: f64,
        var_qd_fp1_dn2: f64,
        var_qd_fp1_dn20: f64,
        var_qd_fp1_dn21: f64,
        var_qd_fp1_dn22: f64,
        var_qd_fp1_dn3: f64,
        var_qd_fp1_dn4: f64,
        var_qd_fp1_dn5: f64,
        var_qd_fp1_dn6: f64,
        var_qd_fp1_dn7: f64,
        var_qd_fp1_dn8: f64,
        var_qd_fp1_dn9: f64,
        var_qd_fp1s: f64,
        var_qd_fp1s_dn0: f64,
        var_qd_fp1s_dn1: f64,
        var_qd_fp1s_dn12: f64,
        var_qd_fp1s_dn14: f64,
        var_qd_fp1s_dn15: f64,
        var_qd_fp1s_dn16: f64,
        var_qd_fp1s_dn17: f64,
        var_qd_fp1s_dn18: f64,
        var_qd_fp1s_dn19: f64,
        var_qd_fp1s_dn2: f64,
        var_qd_fp1s_dn20: f64,
        var_qd_fp1s_dn21: f64,
        var_qd_fp1s_dn22: f64,
        var_qd_fp1s_dn3: f64,
        var_qd_fp1s_dn4: f64,
        var_qd_fp1s_dn5: f64,
        var_qd_fp1s_dn6: f64,
        var_qd_fp1s_dn7: f64,
        var_qd_fp1s_dn8: f64,
        var_qd_fp1s_dn9: f64,
        var_qg_fp1: f64,
        var_qg_fp1_dn0: f64,
        var_qg_fp1_dn1: f64,
        var_qg_fp1_dn12: f64,
        var_qg_fp1_dn14: f64,
        var_qg_fp1_dn15: f64,
        var_qg_fp1_dn16: f64,
        var_qg_fp1_dn17: f64,
        var_qg_fp1_dn18: f64,
        var_qg_fp1_dn19: f64,
        var_qg_fp1_dn2: f64,
        var_qg_fp1_dn20: f64,
        var_qg_fp1_dn21: f64,
        var_qg_fp1_dn22: f64,
        var_qg_fp1_dn3: f64,
        var_qg_fp1_dn4: f64,
        var_qg_fp1_dn5: f64,
        var_qg_fp1_dn6: f64,
        var_qg_fp1_dn7: f64,
        var_qg_fp1_dn8: f64,
        var_qg_fp1_dn9: f64,
        var_qg_fp1s: f64,
        var_qg_fp1s_dn0: f64,
        var_qg_fp1s_dn1: f64,
        var_qg_fp1s_dn12: f64,
        var_qg_fp1s_dn14: f64,
        var_qg_fp1s_dn15: f64,
        var_qg_fp1s_dn16: f64,
        var_qg_fp1s_dn17: f64,
        var_qg_fp1s_dn18: f64,
        var_qg_fp1s_dn19: f64,
        var_qg_fp1s_dn2: f64,
        var_qg_fp1s_dn20: f64,
        var_qg_fp1s_dn21: f64,
        var_qg_fp1s_dn22: f64,
        var_qg_fp1s_dn3: f64,
        var_qg_fp1s_dn4: f64,
        var_qg_fp1s_dn5: f64,
        var_qg_fp1s_dn6: f64,
        var_qg_fp1s_dn7: f64,
        var_qg_fp1s_dn8: f64,
        var_qg_fp1s_dn9: f64,
    ) {
        let (eq126_e1611, eq126_e1611_d_n0, eq126_e1611_d_n1, eq126_e1611_d_n2, eq126_e1611_d_n3, eq126_e1611_d_n4, eq126_e1611_d_n5, eq126_e1611_d_n6, eq126_e1611_d_n7, eq126_e1611_d_n8, eq126_e1611_d_n9, eq126_e1611_d_n12, eq126_e1611_d_n14, eq126_e1611_d_n15, eq126_e1611_d_n16, eq126_e1611_d_n17, eq126_e1611_d_n18, eq126_e1611_d_n19, eq126_e1611_d_n20, eq126_e1611_d_n21, eq126_e1611_d_n22,) = {
    if ((var_guard536 == 0.0) && (var_guard539 != 0.0)) {
        let eq126_e1608: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 25, var_qd_fp1);
        let eq126_e1609: f64 = (p.p7 * eq126_e1608);
        let eq126_e1609_d_n0: f64 = (p.p7 * (var_qd_fp1_dn0 * ddt_scale));
        let eq126_e1609_d_n1: f64 = (p.p7 * (var_qd_fp1_dn1 * ddt_scale));
        let eq126_e1609_d_n2: f64 = (p.p7 * (var_qd_fp1_dn2 * ddt_scale));
        let eq126_e1609_d_n3: f64 = (p.p7 * (var_qd_fp1_dn3 * ddt_scale));
        let eq126_e1609_d_n4: f64 = (p.p7 * (var_qd_fp1_dn4 * ddt_scale));
        let eq126_e1609_d_n5: f64 = (p.p7 * (var_qd_fp1_dn5 * ddt_scale));
        let eq126_e1609_d_n6: f64 = (p.p7 * (var_qd_fp1_dn6 * ddt_scale));
        let eq126_e1609_d_n7: f64 = (p.p7 * (var_qd_fp1_dn7 * ddt_scale));
        let eq126_e1609_d_n8: f64 = (p.p7 * (var_qd_fp1_dn8 * ddt_scale));
        let eq126_e1609_d_n9: f64 = (p.p7 * (var_qd_fp1_dn9 * ddt_scale));
        let eq126_e1609_d_n12: f64 = (p.p7 * (var_qd_fp1_dn12 * ddt_scale));
        let eq126_e1609_d_n14: f64 = (p.p7 * (var_qd_fp1_dn14 * ddt_scale));
        let eq126_e1609_d_n15: f64 = (p.p7 * (var_qd_fp1_dn15 * ddt_scale));
        let eq126_e1609_d_n16: f64 = (p.p7 * (var_qd_fp1_dn16 * ddt_scale));
        let eq126_e1609_d_n17: f64 = (p.p7 * (var_qd_fp1_dn17 * ddt_scale));
        let eq126_e1609_d_n18: f64 = (p.p7 * (var_qd_fp1_dn18 * ddt_scale));
        let eq126_e1609_d_n19: f64 = (p.p7 * (var_qd_fp1_dn19 * ddt_scale));
        let eq126_e1609_d_n20: f64 = (p.p7 * (var_qd_fp1_dn20 * ddt_scale));
        let eq126_e1609_d_n21: f64 = (p.p7 * (var_qd_fp1_dn21 * ddt_scale));
        let eq126_e1609_d_n22: f64 = (p.p7 * (var_qd_fp1_dn22 * ddt_scale));
        (eq126_e1609, eq126_e1609_d_n0, eq126_e1609_d_n1, eq126_e1609_d_n2, eq126_e1609_d_n3, eq126_e1609_d_n4, eq126_e1609_d_n5, eq126_e1609_d_n6, eq126_e1609_d_n7, eq126_e1609_d_n8, eq126_e1609_d_n9, eq126_e1609_d_n12, eq126_e1609_d_n14, eq126_e1609_d_n15, eq126_e1609_d_n16, eq126_e1609_d_n17, eq126_e1609_d_n18, eq126_e1609_d_n19, eq126_e1609_d_n20, eq126_e1609_d_n21, eq126_e1609_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq126_value: f64 = eq126_e1611;
        let eq126_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq126_node_derivatives: [f64; 20] = [eq126_e1611_d_n0, eq126_e1611_d_n1, eq126_e1611_d_n2, eq126_e1611_d_n3, eq126_e1611_d_n4, eq126_e1611_d_n5, eq126_e1611_d_n6, eq126_e1611_d_n7, eq126_e1611_d_n8, eq126_e1611_d_n9, eq126_e1611_d_n12, eq126_e1611_d_n14, eq126_e1611_d_n15, eq126_e1611_d_n16, eq126_e1611_d_n17, eq126_e1611_d_n18, eq126_e1611_d_n19, eq126_e1611_d_n20, eq126_e1611_d_n21, eq126_e1611_d_n22];
        let eq126_branch_derivative_indices: [usize; 0] = [];
        let eq126_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq126_value),
            &eq126_node_derivative_indices,
            &eq126_node_derivatives,
            &eq126_branch_derivative_indices,
            &eq126_branch_derivatives,
            multiplicity,
        );
        let (eq127_e1623, eq127_e1623_d_n0, eq127_e1623_d_n1, eq127_e1623_d_n2, eq127_e1623_d_n3, eq127_e1623_d_n4, eq127_e1623_d_n5, eq127_e1623_d_n6, eq127_e1623_d_n7, eq127_e1623_d_n8, eq127_e1623_d_n9, eq127_e1623_d_n12, eq127_e1623_d_n14, eq127_e1623_d_n15, eq127_e1623_d_n16, eq127_e1623_d_n17, eq127_e1623_d_n18, eq127_e1623_d_n19, eq127_e1623_d_n20, eq127_e1623_d_n21, eq127_e1623_d_n22,) = {
    if (((var_guard536 == 0.0) && (var_guard539 != 0.0)) && (var_guard540 != 0.0)) {
        let eq127_e1620: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 26, var_qg_fp1);
        let eq127_e1621: f64 = (p.p7 * eq127_e1620);
        let eq127_e1621_d_n0: f64 = (p.p7 * (var_qg_fp1_dn0 * ddt_scale));
        let eq127_e1621_d_n1: f64 = (p.p7 * (var_qg_fp1_dn1 * ddt_scale));
        let eq127_e1621_d_n2: f64 = (p.p7 * (var_qg_fp1_dn2 * ddt_scale));
        let eq127_e1621_d_n3: f64 = (p.p7 * (var_qg_fp1_dn3 * ddt_scale));
        let eq127_e1621_d_n4: f64 = (p.p7 * (var_qg_fp1_dn4 * ddt_scale));
        let eq127_e1621_d_n5: f64 = (p.p7 * (var_qg_fp1_dn5 * ddt_scale));
        let eq127_e1621_d_n6: f64 = (p.p7 * (var_qg_fp1_dn6 * ddt_scale));
        let eq127_e1621_d_n7: f64 = (p.p7 * (var_qg_fp1_dn7 * ddt_scale));
        let eq127_e1621_d_n8: f64 = (p.p7 * (var_qg_fp1_dn8 * ddt_scale));
        let eq127_e1621_d_n9: f64 = (p.p7 * (var_qg_fp1_dn9 * ddt_scale));
        let eq127_e1621_d_n12: f64 = (p.p7 * (var_qg_fp1_dn12 * ddt_scale));
        let eq127_e1621_d_n14: f64 = (p.p7 * (var_qg_fp1_dn14 * ddt_scale));
        let eq127_e1621_d_n15: f64 = (p.p7 * (var_qg_fp1_dn15 * ddt_scale));
        let eq127_e1621_d_n16: f64 = (p.p7 * (var_qg_fp1_dn16 * ddt_scale));
        let eq127_e1621_d_n17: f64 = (p.p7 * (var_qg_fp1_dn17 * ddt_scale));
        let eq127_e1621_d_n18: f64 = (p.p7 * (var_qg_fp1_dn18 * ddt_scale));
        let eq127_e1621_d_n19: f64 = (p.p7 * (var_qg_fp1_dn19 * ddt_scale));
        let eq127_e1621_d_n20: f64 = (p.p7 * (var_qg_fp1_dn20 * ddt_scale));
        let eq127_e1621_d_n21: f64 = (p.p7 * (var_qg_fp1_dn21 * ddt_scale));
        let eq127_e1621_d_n22: f64 = (p.p7 * (var_qg_fp1_dn22 * ddt_scale));
        (eq127_e1621, eq127_e1621_d_n0, eq127_e1621_d_n1, eq127_e1621_d_n2, eq127_e1621_d_n3, eq127_e1621_d_n4, eq127_e1621_d_n5, eq127_e1621_d_n6, eq127_e1621_d_n7, eq127_e1621_d_n8, eq127_e1621_d_n9, eq127_e1621_d_n12, eq127_e1621_d_n14, eq127_e1621_d_n15, eq127_e1621_d_n16, eq127_e1621_d_n17, eq127_e1621_d_n18, eq127_e1621_d_n19, eq127_e1621_d_n20, eq127_e1621_d_n21, eq127_e1621_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq127_value: f64 = eq127_e1623;
        let eq127_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq127_node_derivatives: [f64; 20] = [eq127_e1623_d_n0, eq127_e1623_d_n1, eq127_e1623_d_n2, eq127_e1623_d_n3, eq127_e1623_d_n4, eq127_e1623_d_n5, eq127_e1623_d_n6, eq127_e1623_d_n7, eq127_e1623_d_n8, eq127_e1623_d_n9, eq127_e1623_d_n12, eq127_e1623_d_n14, eq127_e1623_d_n15, eq127_e1623_d_n16, eq127_e1623_d_n17, eq127_e1623_d_n18, eq127_e1623_d_n19, eq127_e1623_d_n20, eq127_e1623_d_n21, eq127_e1623_d_n22];
        let eq127_branch_derivative_indices: [usize; 0] = [];
        let eq127_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq127_value),
            &eq127_node_derivative_indices,
            &eq127_node_derivatives,
            &eq127_branch_derivative_indices,
            &eq127_branch_derivatives,
            multiplicity,
        );
        let (eq128_e1637, eq128_e1637_d_n0, eq128_e1637_d_n1, eq128_e1637_d_n2, eq128_e1637_d_n3, eq128_e1637_d_n4, eq128_e1637_d_n5, eq128_e1637_d_n6, eq128_e1637_d_n7, eq128_e1637_d_n8, eq128_e1637_d_n9, eq128_e1637_d_n12, eq128_e1637_d_n14, eq128_e1637_d_n15, eq128_e1637_d_n16, eq128_e1637_d_n17, eq128_e1637_d_n18, eq128_e1637_d_n19, eq128_e1637_d_n20, eq128_e1637_d_n21, eq128_e1637_d_n22,) = {
    if (((var_guard536 == 0.0) && (var_guard539 != 0.0)) && (var_guard540 != 0.0)) {
        let eq128_e1632: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 27, var_qg_fp1);
        let eq128_e1633: f64 = (p.p7 * eq128_e1632);
        let eq128_e1633_d_n0: f64 = (p.p7 * (var_qg_fp1_dn0 * ddt_scale));
        let eq128_e1633_d_n1: f64 = (p.p7 * (var_qg_fp1_dn1 * ddt_scale));
        let eq128_e1633_d_n2: f64 = (p.p7 * (var_qg_fp1_dn2 * ddt_scale));
        let eq128_e1633_d_n3: f64 = (p.p7 * (var_qg_fp1_dn3 * ddt_scale));
        let eq128_e1633_d_n4: f64 = (p.p7 * (var_qg_fp1_dn4 * ddt_scale));
        let eq128_e1633_d_n5: f64 = (p.p7 * (var_qg_fp1_dn5 * ddt_scale));
        let eq128_e1633_d_n6: f64 = (p.p7 * (var_qg_fp1_dn6 * ddt_scale));
        let eq128_e1633_d_n7: f64 = (p.p7 * (var_qg_fp1_dn7 * ddt_scale));
        let eq128_e1633_d_n8: f64 = (p.p7 * (var_qg_fp1_dn8 * ddt_scale));
        let eq128_e1633_d_n9: f64 = (p.p7 * (var_qg_fp1_dn9 * ddt_scale));
        let eq128_e1633_d_n12: f64 = (p.p7 * (var_qg_fp1_dn12 * ddt_scale));
        let eq128_e1633_d_n14: f64 = (p.p7 * (var_qg_fp1_dn14 * ddt_scale));
        let eq128_e1633_d_n15: f64 = (p.p7 * (var_qg_fp1_dn15 * ddt_scale));
        let eq128_e1633_d_n16: f64 = (p.p7 * (var_qg_fp1_dn16 * ddt_scale));
        let eq128_e1633_d_n17: f64 = (p.p7 * (var_qg_fp1_dn17 * ddt_scale));
        let eq128_e1633_d_n18: f64 = (p.p7 * (var_qg_fp1_dn18 * ddt_scale));
        let eq128_e1633_d_n19: f64 = (p.p7 * (var_qg_fp1_dn19 * ddt_scale));
        let eq128_e1633_d_n20: f64 = (p.p7 * (var_qg_fp1_dn20 * ddt_scale));
        let eq128_e1633_d_n21: f64 = (p.p7 * (var_qg_fp1_dn21 * ddt_scale));
        let eq128_e1633_d_n22: f64 = (p.p7 * (var_qg_fp1_dn22 * ddt_scale));
        let eq128_e1635: f64 = (eq128_e1633 * p.p246);
        let eq128_e1635_d_n0: f64 = (eq128_e1633_d_n0 * p.p246);
        let eq128_e1635_d_n1: f64 = (eq128_e1633_d_n1 * p.p246);
        let eq128_e1635_d_n2: f64 = (eq128_e1633_d_n2 * p.p246);
        let eq128_e1635_d_n3: f64 = (eq128_e1633_d_n3 * p.p246);
        let eq128_e1635_d_n4: f64 = (eq128_e1633_d_n4 * p.p246);
        let eq128_e1635_d_n5: f64 = (eq128_e1633_d_n5 * p.p246);
        let eq128_e1635_d_n6: f64 = (eq128_e1633_d_n6 * p.p246);
        let eq128_e1635_d_n7: f64 = (eq128_e1633_d_n7 * p.p246);
        let eq128_e1635_d_n8: f64 = (eq128_e1633_d_n8 * p.p246);
        let eq128_e1635_d_n9: f64 = (eq128_e1633_d_n9 * p.p246);
        let eq128_e1635_d_n12: f64 = (eq128_e1633_d_n12 * p.p246);
        let eq128_e1635_d_n14: f64 = (eq128_e1633_d_n14 * p.p246);
        let eq128_e1635_d_n15: f64 = (eq128_e1633_d_n15 * p.p246);
        let eq128_e1635_d_n16: f64 = (eq128_e1633_d_n16 * p.p246);
        let eq128_e1635_d_n17: f64 = (eq128_e1633_d_n17 * p.p246);
        let eq128_e1635_d_n18: f64 = (eq128_e1633_d_n18 * p.p246);
        let eq128_e1635_d_n19: f64 = (eq128_e1633_d_n19 * p.p246);
        let eq128_e1635_d_n20: f64 = (eq128_e1633_d_n20 * p.p246);
        let eq128_e1635_d_n21: f64 = (eq128_e1633_d_n21 * p.p246);
        let eq128_e1635_d_n22: f64 = (eq128_e1633_d_n22 * p.p246);
        (eq128_e1635, eq128_e1635_d_n0, eq128_e1635_d_n1, eq128_e1635_d_n2, eq128_e1635_d_n3, eq128_e1635_d_n4, eq128_e1635_d_n5, eq128_e1635_d_n6, eq128_e1635_d_n7, eq128_e1635_d_n8, eq128_e1635_d_n9, eq128_e1635_d_n12, eq128_e1635_d_n14, eq128_e1635_d_n15, eq128_e1635_d_n16, eq128_e1635_d_n17, eq128_e1635_d_n18, eq128_e1635_d_n19, eq128_e1635_d_n20, eq128_e1635_d_n21, eq128_e1635_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq128_value: f64 = eq128_e1637;
        let eq128_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq128_node_derivatives: [f64; 20] = [eq128_e1637_d_n0, eq128_e1637_d_n1, eq128_e1637_d_n2, eq128_e1637_d_n3, eq128_e1637_d_n4, eq128_e1637_d_n5, eq128_e1637_d_n6, eq128_e1637_d_n7, eq128_e1637_d_n8, eq128_e1637_d_n9, eq128_e1637_d_n12, eq128_e1637_d_n14, eq128_e1637_d_n15, eq128_e1637_d_n16, eq128_e1637_d_n17, eq128_e1637_d_n18, eq128_e1637_d_n19, eq128_e1637_d_n20, eq128_e1637_d_n21, eq128_e1637_d_n22];
        let eq128_branch_derivative_indices: [usize; 0] = [];
        let eq128_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq128_value),
            &eq128_node_derivative_indices,
            &eq128_node_derivatives,
            &eq128_branch_derivative_indices,
            &eq128_branch_derivatives,
            multiplicity,
        );
        let (eq129_e1650, eq129_e1650_d_n0, eq129_e1650_d_n1, eq129_e1650_d_n2, eq129_e1650_d_n3, eq129_e1650_d_n4, eq129_e1650_d_n5, eq129_e1650_d_n6, eq129_e1650_d_n7, eq129_e1650_d_n8, eq129_e1650_d_n9, eq129_e1650_d_n12, eq129_e1650_d_n14, eq129_e1650_d_n15, eq129_e1650_d_n16, eq129_e1650_d_n17, eq129_e1650_d_n18, eq129_e1650_d_n19, eq129_e1650_d_n20, eq129_e1650_d_n21, eq129_e1650_d_n22,) = {
    if (((var_guard536 == 0.0) && (var_guard539 != 0.0)) && (var_guard540 == 0.0)) {
        let eq129_e1647: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 28, var_qg_fp1);
        let eq129_e1648: f64 = (p.p7 * eq129_e1647);
        let eq129_e1648_d_n0: f64 = (p.p7 * (var_qg_fp1_dn0 * ddt_scale));
        let eq129_e1648_d_n1: f64 = (p.p7 * (var_qg_fp1_dn1 * ddt_scale));
        let eq129_e1648_d_n2: f64 = (p.p7 * (var_qg_fp1_dn2 * ddt_scale));
        let eq129_e1648_d_n3: f64 = (p.p7 * (var_qg_fp1_dn3 * ddt_scale));
        let eq129_e1648_d_n4: f64 = (p.p7 * (var_qg_fp1_dn4 * ddt_scale));
        let eq129_e1648_d_n5: f64 = (p.p7 * (var_qg_fp1_dn5 * ddt_scale));
        let eq129_e1648_d_n6: f64 = (p.p7 * (var_qg_fp1_dn6 * ddt_scale));
        let eq129_e1648_d_n7: f64 = (p.p7 * (var_qg_fp1_dn7 * ddt_scale));
        let eq129_e1648_d_n8: f64 = (p.p7 * (var_qg_fp1_dn8 * ddt_scale));
        let eq129_e1648_d_n9: f64 = (p.p7 * (var_qg_fp1_dn9 * ddt_scale));
        let eq129_e1648_d_n12: f64 = (p.p7 * (var_qg_fp1_dn12 * ddt_scale));
        let eq129_e1648_d_n14: f64 = (p.p7 * (var_qg_fp1_dn14 * ddt_scale));
        let eq129_e1648_d_n15: f64 = (p.p7 * (var_qg_fp1_dn15 * ddt_scale));
        let eq129_e1648_d_n16: f64 = (p.p7 * (var_qg_fp1_dn16 * ddt_scale));
        let eq129_e1648_d_n17: f64 = (p.p7 * (var_qg_fp1_dn17 * ddt_scale));
        let eq129_e1648_d_n18: f64 = (p.p7 * (var_qg_fp1_dn18 * ddt_scale));
        let eq129_e1648_d_n19: f64 = (p.p7 * (var_qg_fp1_dn19 * ddt_scale));
        let eq129_e1648_d_n20: f64 = (p.p7 * (var_qg_fp1_dn20 * ddt_scale));
        let eq129_e1648_d_n21: f64 = (p.p7 * (var_qg_fp1_dn21 * ddt_scale));
        let eq129_e1648_d_n22: f64 = (p.p7 * (var_qg_fp1_dn22 * ddt_scale));
        (eq129_e1648, eq129_e1648_d_n0, eq129_e1648_d_n1, eq129_e1648_d_n2, eq129_e1648_d_n3, eq129_e1648_d_n4, eq129_e1648_d_n5, eq129_e1648_d_n6, eq129_e1648_d_n7, eq129_e1648_d_n8, eq129_e1648_d_n9, eq129_e1648_d_n12, eq129_e1648_d_n14, eq129_e1648_d_n15, eq129_e1648_d_n16, eq129_e1648_d_n17, eq129_e1648_d_n18, eq129_e1648_d_n19, eq129_e1648_d_n20, eq129_e1648_d_n21, eq129_e1648_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq129_value: f64 = eq129_e1650;
        let eq129_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq129_node_derivatives: [f64; 20] = [eq129_e1650_d_n0, eq129_e1650_d_n1, eq129_e1650_d_n2, eq129_e1650_d_n3, eq129_e1650_d_n4, eq129_e1650_d_n5, eq129_e1650_d_n6, eq129_e1650_d_n7, eq129_e1650_d_n8, eq129_e1650_d_n9, eq129_e1650_d_n12, eq129_e1650_d_n14, eq129_e1650_d_n15, eq129_e1650_d_n16, eq129_e1650_d_n17, eq129_e1650_d_n18, eq129_e1650_d_n19, eq129_e1650_d_n20, eq129_e1650_d_n21, eq129_e1650_d_n22];
        let eq129_branch_derivative_indices: [usize; 0] = [];
        let eq129_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq129_value),
            &eq129_node_derivative_indices,
            &eq129_node_derivatives,
            &eq129_branch_derivative_indices,
            &eq129_branch_derivatives,
            multiplicity,
        );
        let (eq130_e1665, eq130_e1665_d_n0, eq130_e1665_d_n1, eq130_e1665_d_n2, eq130_e1665_d_n3, eq130_e1665_d_n4, eq130_e1665_d_n5, eq130_e1665_d_n6, eq130_e1665_d_n7, eq130_e1665_d_n8, eq130_e1665_d_n9, eq130_e1665_d_n12, eq130_e1665_d_n14, eq130_e1665_d_n15, eq130_e1665_d_n16, eq130_e1665_d_n17, eq130_e1665_d_n18, eq130_e1665_d_n19, eq130_e1665_d_n20, eq130_e1665_d_n21, eq130_e1665_d_n22,) = {
    if (((var_guard536 == 0.0) && (var_guard539 != 0.0)) && (var_guard540 == 0.0)) {
        let eq130_e1660: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 29, var_qg_fp1);
        let eq130_e1661: f64 = (p.p7 * eq130_e1660);
        let eq130_e1661_d_n0: f64 = (p.p7 * (var_qg_fp1_dn0 * ddt_scale));
        let eq130_e1661_d_n1: f64 = (p.p7 * (var_qg_fp1_dn1 * ddt_scale));
        let eq130_e1661_d_n2: f64 = (p.p7 * (var_qg_fp1_dn2 * ddt_scale));
        let eq130_e1661_d_n3: f64 = (p.p7 * (var_qg_fp1_dn3 * ddt_scale));
        let eq130_e1661_d_n4: f64 = (p.p7 * (var_qg_fp1_dn4 * ddt_scale));
        let eq130_e1661_d_n5: f64 = (p.p7 * (var_qg_fp1_dn5 * ddt_scale));
        let eq130_e1661_d_n6: f64 = (p.p7 * (var_qg_fp1_dn6 * ddt_scale));
        let eq130_e1661_d_n7: f64 = (p.p7 * (var_qg_fp1_dn7 * ddt_scale));
        let eq130_e1661_d_n8: f64 = (p.p7 * (var_qg_fp1_dn8 * ddt_scale));
        let eq130_e1661_d_n9: f64 = (p.p7 * (var_qg_fp1_dn9 * ddt_scale));
        let eq130_e1661_d_n12: f64 = (p.p7 * (var_qg_fp1_dn12 * ddt_scale));
        let eq130_e1661_d_n14: f64 = (p.p7 * (var_qg_fp1_dn14 * ddt_scale));
        let eq130_e1661_d_n15: f64 = (p.p7 * (var_qg_fp1_dn15 * ddt_scale));
        let eq130_e1661_d_n16: f64 = (p.p7 * (var_qg_fp1_dn16 * ddt_scale));
        let eq130_e1661_d_n17: f64 = (p.p7 * (var_qg_fp1_dn17 * ddt_scale));
        let eq130_e1661_d_n18: f64 = (p.p7 * (var_qg_fp1_dn18 * ddt_scale));
        let eq130_e1661_d_n19: f64 = (p.p7 * (var_qg_fp1_dn19 * ddt_scale));
        let eq130_e1661_d_n20: f64 = (p.p7 * (var_qg_fp1_dn20 * ddt_scale));
        let eq130_e1661_d_n21: f64 = (p.p7 * (var_qg_fp1_dn21 * ddt_scale));
        let eq130_e1661_d_n22: f64 = (p.p7 * (var_qg_fp1_dn22 * ddt_scale));
        let eq130_e1663: f64 = (eq130_e1661 * p.p246);
        let eq130_e1663_d_n0: f64 = (eq130_e1661_d_n0 * p.p246);
        let eq130_e1663_d_n1: f64 = (eq130_e1661_d_n1 * p.p246);
        let eq130_e1663_d_n2: f64 = (eq130_e1661_d_n2 * p.p246);
        let eq130_e1663_d_n3: f64 = (eq130_e1661_d_n3 * p.p246);
        let eq130_e1663_d_n4: f64 = (eq130_e1661_d_n4 * p.p246);
        let eq130_e1663_d_n5: f64 = (eq130_e1661_d_n5 * p.p246);
        let eq130_e1663_d_n6: f64 = (eq130_e1661_d_n6 * p.p246);
        let eq130_e1663_d_n7: f64 = (eq130_e1661_d_n7 * p.p246);
        let eq130_e1663_d_n8: f64 = (eq130_e1661_d_n8 * p.p246);
        let eq130_e1663_d_n9: f64 = (eq130_e1661_d_n9 * p.p246);
        let eq130_e1663_d_n12: f64 = (eq130_e1661_d_n12 * p.p246);
        let eq130_e1663_d_n14: f64 = (eq130_e1661_d_n14 * p.p246);
        let eq130_e1663_d_n15: f64 = (eq130_e1661_d_n15 * p.p246);
        let eq130_e1663_d_n16: f64 = (eq130_e1661_d_n16 * p.p246);
        let eq130_e1663_d_n17: f64 = (eq130_e1661_d_n17 * p.p246);
        let eq130_e1663_d_n18: f64 = (eq130_e1661_d_n18 * p.p246);
        let eq130_e1663_d_n19: f64 = (eq130_e1661_d_n19 * p.p246);
        let eq130_e1663_d_n20: f64 = (eq130_e1661_d_n20 * p.p246);
        let eq130_e1663_d_n21: f64 = (eq130_e1661_d_n21 * p.p246);
        let eq130_e1663_d_n22: f64 = (eq130_e1661_d_n22 * p.p246);
        (eq130_e1663, eq130_e1663_d_n0, eq130_e1663_d_n1, eq130_e1663_d_n2, eq130_e1663_d_n3, eq130_e1663_d_n4, eq130_e1663_d_n5, eq130_e1663_d_n6, eq130_e1663_d_n7, eq130_e1663_d_n8, eq130_e1663_d_n9, eq130_e1663_d_n12, eq130_e1663_d_n14, eq130_e1663_d_n15, eq130_e1663_d_n16, eq130_e1663_d_n17, eq130_e1663_d_n18, eq130_e1663_d_n19, eq130_e1663_d_n20, eq130_e1663_d_n21, eq130_e1663_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq130_value: f64 = eq130_e1665;
        let eq130_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq130_node_derivatives: [f64; 20] = [eq130_e1665_d_n0, eq130_e1665_d_n1, eq130_e1665_d_n2, eq130_e1665_d_n3, eq130_e1665_d_n4, eq130_e1665_d_n5, eq130_e1665_d_n6, eq130_e1665_d_n7, eq130_e1665_d_n8, eq130_e1665_d_n9, eq130_e1665_d_n12, eq130_e1665_d_n14, eq130_e1665_d_n15, eq130_e1665_d_n16, eq130_e1665_d_n17, eq130_e1665_d_n18, eq130_e1665_d_n19, eq130_e1665_d_n20, eq130_e1665_d_n21, eq130_e1665_d_n22];
        let eq130_branch_derivative_indices: [usize; 0] = [];
        let eq130_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq130_value),
            &eq130_node_derivative_indices,
            &eq130_node_derivatives,
            &eq130_branch_derivative_indices,
            &eq130_branch_derivatives,
            multiplicity,
        );
        let (eq131_e1677, eq131_e1677_d_n0, eq131_e1677_d_n1, eq131_e1677_d_n2, eq131_e1677_d_n3, eq131_e1677_d_n4, eq131_e1677_d_n5, eq131_e1677_d_n6, eq131_e1677_d_n7, eq131_e1677_d_n8, eq131_e1677_d_n9, eq131_e1677_d_n12, eq131_e1677_d_n14, eq131_e1677_d_n15, eq131_e1677_d_n16, eq131_e1677_d_n17, eq131_e1677_d_n18, eq131_e1677_d_n19, eq131_e1677_d_n20, eq131_e1677_d_n21, eq131_e1677_d_n22,) = {
    if ((var_guard536 == 0.0) && (var_guard539 != 0.0)) {
        let eq131_e1673: f64 = (p.p251 * var_qg_fp1);
        let eq131_e1673_d_n0: f64 = (p.p251 * var_qg_fp1_dn0);
        let eq131_e1673_d_n1: f64 = (p.p251 * var_qg_fp1_dn1);
        let eq131_e1673_d_n2: f64 = (p.p251 * var_qg_fp1_dn2);
        let eq131_e1673_d_n3: f64 = (p.p251 * var_qg_fp1_dn3);
        let eq131_e1673_d_n4: f64 = (p.p251 * var_qg_fp1_dn4);
        let eq131_e1673_d_n5: f64 = (p.p251 * var_qg_fp1_dn5);
        let eq131_e1673_d_n6: f64 = (p.p251 * var_qg_fp1_dn6);
        let eq131_e1673_d_n7: f64 = (p.p251 * var_qg_fp1_dn7);
        let eq131_e1673_d_n8: f64 = (p.p251 * var_qg_fp1_dn8);
        let eq131_e1673_d_n9: f64 = (p.p251 * var_qg_fp1_dn9);
        let eq131_e1673_d_n12: f64 = (p.p251 * var_qg_fp1_dn12);
        let eq131_e1673_d_n14: f64 = (p.p251 * var_qg_fp1_dn14);
        let eq131_e1673_d_n15: f64 = (p.p251 * var_qg_fp1_dn15);
        let eq131_e1673_d_n16: f64 = (p.p251 * var_qg_fp1_dn16);
        let eq131_e1673_d_n17: f64 = (p.p251 * var_qg_fp1_dn17);
        let eq131_e1673_d_n18: f64 = (p.p251 * var_qg_fp1_dn18);
        let eq131_e1673_d_n19: f64 = (p.p251 * var_qg_fp1_dn19);
        let eq131_e1673_d_n20: f64 = (p.p251 * var_qg_fp1_dn20);
        let eq131_e1673_d_n21: f64 = (p.p251 * var_qg_fp1_dn21);
        let eq131_e1673_d_n22: f64 = (p.p251 * var_qg_fp1_dn22);
        let eq131_e1674: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 30, eq131_e1673);
        let eq131_e1675: f64 = (p.p7 * eq131_e1674);
        let eq131_e1675_d_n0: f64 = (p.p7 * (eq131_e1673_d_n0 * ddt_scale));
        let eq131_e1675_d_n1: f64 = (p.p7 * (eq131_e1673_d_n1 * ddt_scale));
        let eq131_e1675_d_n2: f64 = (p.p7 * (eq131_e1673_d_n2 * ddt_scale));
        let eq131_e1675_d_n3: f64 = (p.p7 * (eq131_e1673_d_n3 * ddt_scale));
        let eq131_e1675_d_n4: f64 = (p.p7 * (eq131_e1673_d_n4 * ddt_scale));
        let eq131_e1675_d_n5: f64 = (p.p7 * (eq131_e1673_d_n5 * ddt_scale));
        let eq131_e1675_d_n6: f64 = (p.p7 * (eq131_e1673_d_n6 * ddt_scale));
        let eq131_e1675_d_n7: f64 = (p.p7 * (eq131_e1673_d_n7 * ddt_scale));
        let eq131_e1675_d_n8: f64 = (p.p7 * (eq131_e1673_d_n8 * ddt_scale));
        let eq131_e1675_d_n9: f64 = (p.p7 * (eq131_e1673_d_n9 * ddt_scale));
        let eq131_e1675_d_n12: f64 = (p.p7 * (eq131_e1673_d_n12 * ddt_scale));
        let eq131_e1675_d_n14: f64 = (p.p7 * (eq131_e1673_d_n14 * ddt_scale));
        let eq131_e1675_d_n15: f64 = (p.p7 * (eq131_e1673_d_n15 * ddt_scale));
        let eq131_e1675_d_n16: f64 = (p.p7 * (eq131_e1673_d_n16 * ddt_scale));
        let eq131_e1675_d_n17: f64 = (p.p7 * (eq131_e1673_d_n17 * ddt_scale));
        let eq131_e1675_d_n18: f64 = (p.p7 * (eq131_e1673_d_n18 * ddt_scale));
        let eq131_e1675_d_n19: f64 = (p.p7 * (eq131_e1673_d_n19 * ddt_scale));
        let eq131_e1675_d_n20: f64 = (p.p7 * (eq131_e1673_d_n20 * ddt_scale));
        let eq131_e1675_d_n21: f64 = (p.p7 * (eq131_e1673_d_n21 * ddt_scale));
        let eq131_e1675_d_n22: f64 = (p.p7 * (eq131_e1673_d_n22 * ddt_scale));
        (eq131_e1675, eq131_e1675_d_n0, eq131_e1675_d_n1, eq131_e1675_d_n2, eq131_e1675_d_n3, eq131_e1675_d_n4, eq131_e1675_d_n5, eq131_e1675_d_n6, eq131_e1675_d_n7, eq131_e1675_d_n8, eq131_e1675_d_n9, eq131_e1675_d_n12, eq131_e1675_d_n14, eq131_e1675_d_n15, eq131_e1675_d_n16, eq131_e1675_d_n17, eq131_e1675_d_n18, eq131_e1675_d_n19, eq131_e1675_d_n20, eq131_e1675_d_n21, eq131_e1675_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq131_value: f64 = eq131_e1677;
        let eq131_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq131_node_derivatives: [f64; 20] = [eq131_e1677_d_n0, eq131_e1677_d_n1, eq131_e1677_d_n2, eq131_e1677_d_n3, eq131_e1677_d_n4, eq131_e1677_d_n5, eq131_e1677_d_n6, eq131_e1677_d_n7, eq131_e1677_d_n8, eq131_e1677_d_n9, eq131_e1677_d_n12, eq131_e1677_d_n14, eq131_e1677_d_n15, eq131_e1677_d_n16, eq131_e1677_d_n17, eq131_e1677_d_n18, eq131_e1677_d_n19, eq131_e1677_d_n20, eq131_e1677_d_n21, eq131_e1677_d_n22];
        let eq131_branch_derivative_indices: [usize; 0] = [];
        let eq131_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(7),
            multiplicity * (eq131_value),
            &eq131_node_derivative_indices,
            &eq131_node_derivatives,
            &eq131_branch_derivative_indices,
            &eq131_branch_derivatives,
            multiplicity,
        );
        let (eq132_e1686, eq132_e1686_d_n0, eq132_e1686_d_n1, eq132_e1686_d_n2, eq132_e1686_d_n3, eq132_e1686_d_n4, eq132_e1686_d_n5, eq132_e1686_d_n6, eq132_e1686_d_n7, eq132_e1686_d_n8, eq132_e1686_d_n9, eq132_e1686_d_n12, eq132_e1686_d_n14, eq132_e1686_d_n15, eq132_e1686_d_n16, eq132_e1686_d_n17, eq132_e1686_d_n18, eq132_e1686_d_n19, eq132_e1686_d_n20, eq132_e1686_d_n21, eq132_e1686_d_n22,) = {
    if ((var_guard541 != 0.0) && (var_guard542 != 0.0)) {
        let eq132_e1683: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 31, var_qd_fp1s);
        let eq132_e1684: f64 = (p.p7 * eq132_e1683);
        let eq132_e1684_d_n0: f64 = (p.p7 * (var_qd_fp1s_dn0 * ddt_scale));
        let eq132_e1684_d_n1: f64 = (p.p7 * (var_qd_fp1s_dn1 * ddt_scale));
        let eq132_e1684_d_n2: f64 = (p.p7 * (var_qd_fp1s_dn2 * ddt_scale));
        let eq132_e1684_d_n3: f64 = (p.p7 * (var_qd_fp1s_dn3 * ddt_scale));
        let eq132_e1684_d_n4: f64 = (p.p7 * (var_qd_fp1s_dn4 * ddt_scale));
        let eq132_e1684_d_n5: f64 = (p.p7 * (var_qd_fp1s_dn5 * ddt_scale));
        let eq132_e1684_d_n6: f64 = (p.p7 * (var_qd_fp1s_dn6 * ddt_scale));
        let eq132_e1684_d_n7: f64 = (p.p7 * (var_qd_fp1s_dn7 * ddt_scale));
        let eq132_e1684_d_n8: f64 = (p.p7 * (var_qd_fp1s_dn8 * ddt_scale));
        let eq132_e1684_d_n9: f64 = (p.p7 * (var_qd_fp1s_dn9 * ddt_scale));
        let eq132_e1684_d_n12: f64 = (p.p7 * (var_qd_fp1s_dn12 * ddt_scale));
        let eq132_e1684_d_n14: f64 = (p.p7 * (var_qd_fp1s_dn14 * ddt_scale));
        let eq132_e1684_d_n15: f64 = (p.p7 * (var_qd_fp1s_dn15 * ddt_scale));
        let eq132_e1684_d_n16: f64 = (p.p7 * (var_qd_fp1s_dn16 * ddt_scale));
        let eq132_e1684_d_n17: f64 = (p.p7 * (var_qd_fp1s_dn17 * ddt_scale));
        let eq132_e1684_d_n18: f64 = (p.p7 * (var_qd_fp1s_dn18 * ddt_scale));
        let eq132_e1684_d_n19: f64 = (p.p7 * (var_qd_fp1s_dn19 * ddt_scale));
        let eq132_e1684_d_n20: f64 = (p.p7 * (var_qd_fp1s_dn20 * ddt_scale));
        let eq132_e1684_d_n21: f64 = (p.p7 * (var_qd_fp1s_dn21 * ddt_scale));
        let eq132_e1684_d_n22: f64 = (p.p7 * (var_qd_fp1s_dn22 * ddt_scale));
        (eq132_e1684, eq132_e1684_d_n0, eq132_e1684_d_n1, eq132_e1684_d_n2, eq132_e1684_d_n3, eq132_e1684_d_n4, eq132_e1684_d_n5, eq132_e1684_d_n6, eq132_e1684_d_n7, eq132_e1684_d_n8, eq132_e1684_d_n9, eq132_e1684_d_n12, eq132_e1684_d_n14, eq132_e1684_d_n15, eq132_e1684_d_n16, eq132_e1684_d_n17, eq132_e1684_d_n18, eq132_e1684_d_n19, eq132_e1684_d_n20, eq132_e1684_d_n21, eq132_e1684_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq132_value: f64 = eq132_e1686;
        let eq132_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq132_node_derivatives: [f64; 20] = [eq132_e1686_d_n0, eq132_e1686_d_n1, eq132_e1686_d_n2, eq132_e1686_d_n3, eq132_e1686_d_n4, eq132_e1686_d_n5, eq132_e1686_d_n6, eq132_e1686_d_n7, eq132_e1686_d_n8, eq132_e1686_d_n9, eq132_e1686_d_n12, eq132_e1686_d_n14, eq132_e1686_d_n15, eq132_e1686_d_n16, eq132_e1686_d_n17, eq132_e1686_d_n18, eq132_e1686_d_n19, eq132_e1686_d_n20, eq132_e1686_d_n21, eq132_e1686_d_n22];
        let eq132_branch_derivative_indices: [usize; 0] = [];
        let eq132_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(19),
            multiplicity * (eq132_value),
            &eq132_node_derivative_indices,
            &eq132_node_derivatives,
            &eq132_branch_derivative_indices,
            &eq132_branch_derivatives,
            multiplicity,
        );
        let (eq133_e1697, eq133_e1697_d_n0, eq133_e1697_d_n1, eq133_e1697_d_n2, eq133_e1697_d_n3, eq133_e1697_d_n4, eq133_e1697_d_n5, eq133_e1697_d_n6, eq133_e1697_d_n7, eq133_e1697_d_n8, eq133_e1697_d_n9, eq133_e1697_d_n12, eq133_e1697_d_n14, eq133_e1697_d_n15, eq133_e1697_d_n16, eq133_e1697_d_n17, eq133_e1697_d_n18, eq133_e1697_d_n19, eq133_e1697_d_n20, eq133_e1697_d_n21, eq133_e1697_d_n22,) = {
    if (((var_guard541 != 0.0) && (var_guard542 != 0.0)) && (var_guard543 != 0.0)) {
        let eq133_e1694: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 32, var_qg_fp1s);
        let eq133_e1695: f64 = (p.p7 * eq133_e1694);
        let eq133_e1695_d_n0: f64 = (p.p7 * (var_qg_fp1s_dn0 * ddt_scale));
        let eq133_e1695_d_n1: f64 = (p.p7 * (var_qg_fp1s_dn1 * ddt_scale));
        let eq133_e1695_d_n2: f64 = (p.p7 * (var_qg_fp1s_dn2 * ddt_scale));
        let eq133_e1695_d_n3: f64 = (p.p7 * (var_qg_fp1s_dn3 * ddt_scale));
        let eq133_e1695_d_n4: f64 = (p.p7 * (var_qg_fp1s_dn4 * ddt_scale));
        let eq133_e1695_d_n5: f64 = (p.p7 * (var_qg_fp1s_dn5 * ddt_scale));
        let eq133_e1695_d_n6: f64 = (p.p7 * (var_qg_fp1s_dn6 * ddt_scale));
        let eq133_e1695_d_n7: f64 = (p.p7 * (var_qg_fp1s_dn7 * ddt_scale));
        let eq133_e1695_d_n8: f64 = (p.p7 * (var_qg_fp1s_dn8 * ddt_scale));
        let eq133_e1695_d_n9: f64 = (p.p7 * (var_qg_fp1s_dn9 * ddt_scale));
        let eq133_e1695_d_n12: f64 = (p.p7 * (var_qg_fp1s_dn12 * ddt_scale));
        let eq133_e1695_d_n14: f64 = (p.p7 * (var_qg_fp1s_dn14 * ddt_scale));
        let eq133_e1695_d_n15: f64 = (p.p7 * (var_qg_fp1s_dn15 * ddt_scale));
        let eq133_e1695_d_n16: f64 = (p.p7 * (var_qg_fp1s_dn16 * ddt_scale));
        let eq133_e1695_d_n17: f64 = (p.p7 * (var_qg_fp1s_dn17 * ddt_scale));
        let eq133_e1695_d_n18: f64 = (p.p7 * (var_qg_fp1s_dn18 * ddt_scale));
        let eq133_e1695_d_n19: f64 = (p.p7 * (var_qg_fp1s_dn19 * ddt_scale));
        let eq133_e1695_d_n20: f64 = (p.p7 * (var_qg_fp1s_dn20 * ddt_scale));
        let eq133_e1695_d_n21: f64 = (p.p7 * (var_qg_fp1s_dn21 * ddt_scale));
        let eq133_e1695_d_n22: f64 = (p.p7 * (var_qg_fp1s_dn22 * ddt_scale));
        (eq133_e1695, eq133_e1695_d_n0, eq133_e1695_d_n1, eq133_e1695_d_n2, eq133_e1695_d_n3, eq133_e1695_d_n4, eq133_e1695_d_n5, eq133_e1695_d_n6, eq133_e1695_d_n7, eq133_e1695_d_n8, eq133_e1695_d_n9, eq133_e1695_d_n12, eq133_e1695_d_n14, eq133_e1695_d_n15, eq133_e1695_d_n16, eq133_e1695_d_n17, eq133_e1695_d_n18, eq133_e1695_d_n19, eq133_e1695_d_n20, eq133_e1695_d_n21, eq133_e1695_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq133_value: f64 = eq133_e1697;
        let eq133_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq133_node_derivatives: [f64; 20] = [eq133_e1697_d_n0, eq133_e1697_d_n1, eq133_e1697_d_n2, eq133_e1697_d_n3, eq133_e1697_d_n4, eq133_e1697_d_n5, eq133_e1697_d_n6, eq133_e1697_d_n7, eq133_e1697_d_n8, eq133_e1697_d_n9, eq133_e1697_d_n12, eq133_e1697_d_n14, eq133_e1697_d_n15, eq133_e1697_d_n16, eq133_e1697_d_n17, eq133_e1697_d_n18, eq133_e1697_d_n19, eq133_e1697_d_n20, eq133_e1697_d_n21, eq133_e1697_d_n22];
        let eq133_branch_derivative_indices: [usize; 0] = [];
        let eq133_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(19),
            multiplicity * (eq133_value),
            &eq133_node_derivative_indices,
            &eq133_node_derivatives,
            &eq133_branch_derivative_indices,
            &eq133_branch_derivatives,
            multiplicity,
        );
        let (eq134_e1710, eq134_e1710_d_n0, eq134_e1710_d_n1, eq134_e1710_d_n2, eq134_e1710_d_n3, eq134_e1710_d_n4, eq134_e1710_d_n5, eq134_e1710_d_n6, eq134_e1710_d_n7, eq134_e1710_d_n8, eq134_e1710_d_n9, eq134_e1710_d_n12, eq134_e1710_d_n14, eq134_e1710_d_n15, eq134_e1710_d_n16, eq134_e1710_d_n17, eq134_e1710_d_n18, eq134_e1710_d_n19, eq134_e1710_d_n20, eq134_e1710_d_n21, eq134_e1710_d_n22,) = {
    if (((var_guard541 != 0.0) && (var_guard542 != 0.0)) && (var_guard543 != 0.0)) {
        let eq134_e1705: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 33, var_qg_fp1s);
        let eq134_e1706: f64 = (p.p7 * eq134_e1705);
        let eq134_e1706_d_n0: f64 = (p.p7 * (var_qg_fp1s_dn0 * ddt_scale));
        let eq134_e1706_d_n1: f64 = (p.p7 * (var_qg_fp1s_dn1 * ddt_scale));
        let eq134_e1706_d_n2: f64 = (p.p7 * (var_qg_fp1s_dn2 * ddt_scale));
        let eq134_e1706_d_n3: f64 = (p.p7 * (var_qg_fp1s_dn3 * ddt_scale));
        let eq134_e1706_d_n4: f64 = (p.p7 * (var_qg_fp1s_dn4 * ddt_scale));
        let eq134_e1706_d_n5: f64 = (p.p7 * (var_qg_fp1s_dn5 * ddt_scale));
        let eq134_e1706_d_n6: f64 = (p.p7 * (var_qg_fp1s_dn6 * ddt_scale));
        let eq134_e1706_d_n7: f64 = (p.p7 * (var_qg_fp1s_dn7 * ddt_scale));
        let eq134_e1706_d_n8: f64 = (p.p7 * (var_qg_fp1s_dn8 * ddt_scale));
        let eq134_e1706_d_n9: f64 = (p.p7 * (var_qg_fp1s_dn9 * ddt_scale));
        let eq134_e1706_d_n12: f64 = (p.p7 * (var_qg_fp1s_dn12 * ddt_scale));
        let eq134_e1706_d_n14: f64 = (p.p7 * (var_qg_fp1s_dn14 * ddt_scale));
        let eq134_e1706_d_n15: f64 = (p.p7 * (var_qg_fp1s_dn15 * ddt_scale));
        let eq134_e1706_d_n16: f64 = (p.p7 * (var_qg_fp1s_dn16 * ddt_scale));
        let eq134_e1706_d_n17: f64 = (p.p7 * (var_qg_fp1s_dn17 * ddt_scale));
        let eq134_e1706_d_n18: f64 = (p.p7 * (var_qg_fp1s_dn18 * ddt_scale));
        let eq134_e1706_d_n19: f64 = (p.p7 * (var_qg_fp1s_dn19 * ddt_scale));
        let eq134_e1706_d_n20: f64 = (p.p7 * (var_qg_fp1s_dn20 * ddt_scale));
        let eq134_e1706_d_n21: f64 = (p.p7 * (var_qg_fp1s_dn21 * ddt_scale));
        let eq134_e1706_d_n22: f64 = (p.p7 * (var_qg_fp1s_dn22 * ddt_scale));
        let eq134_e1708: f64 = (eq134_e1706 * p.p246);
        let eq134_e1708_d_n0: f64 = (eq134_e1706_d_n0 * p.p246);
        let eq134_e1708_d_n1: f64 = (eq134_e1706_d_n1 * p.p246);
        let eq134_e1708_d_n2: f64 = (eq134_e1706_d_n2 * p.p246);
        let eq134_e1708_d_n3: f64 = (eq134_e1706_d_n3 * p.p246);
        let eq134_e1708_d_n4: f64 = (eq134_e1706_d_n4 * p.p246);
        let eq134_e1708_d_n5: f64 = (eq134_e1706_d_n5 * p.p246);
        let eq134_e1708_d_n6: f64 = (eq134_e1706_d_n6 * p.p246);
        let eq134_e1708_d_n7: f64 = (eq134_e1706_d_n7 * p.p246);
        let eq134_e1708_d_n8: f64 = (eq134_e1706_d_n8 * p.p246);
        let eq134_e1708_d_n9: f64 = (eq134_e1706_d_n9 * p.p246);
        let eq134_e1708_d_n12: f64 = (eq134_e1706_d_n12 * p.p246);
        let eq134_e1708_d_n14: f64 = (eq134_e1706_d_n14 * p.p246);
        let eq134_e1708_d_n15: f64 = (eq134_e1706_d_n15 * p.p246);
        let eq134_e1708_d_n16: f64 = (eq134_e1706_d_n16 * p.p246);
        let eq134_e1708_d_n17: f64 = (eq134_e1706_d_n17 * p.p246);
        let eq134_e1708_d_n18: f64 = (eq134_e1706_d_n18 * p.p246);
        let eq134_e1708_d_n19: f64 = (eq134_e1706_d_n19 * p.p246);
        let eq134_e1708_d_n20: f64 = (eq134_e1706_d_n20 * p.p246);
        let eq134_e1708_d_n21: f64 = (eq134_e1706_d_n21 * p.p246);
        let eq134_e1708_d_n22: f64 = (eq134_e1706_d_n22 * p.p246);
        (eq134_e1708, eq134_e1708_d_n0, eq134_e1708_d_n1, eq134_e1708_d_n2, eq134_e1708_d_n3, eq134_e1708_d_n4, eq134_e1708_d_n5, eq134_e1708_d_n6, eq134_e1708_d_n7, eq134_e1708_d_n8, eq134_e1708_d_n9, eq134_e1708_d_n12, eq134_e1708_d_n14, eq134_e1708_d_n15, eq134_e1708_d_n16, eq134_e1708_d_n17, eq134_e1708_d_n18, eq134_e1708_d_n19, eq134_e1708_d_n20, eq134_e1708_d_n21, eq134_e1708_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq134_value: f64 = eq134_e1710;
        let eq134_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq134_node_derivatives: [f64; 20] = [eq134_e1710_d_n0, eq134_e1710_d_n1, eq134_e1710_d_n2, eq134_e1710_d_n3, eq134_e1710_d_n4, eq134_e1710_d_n5, eq134_e1710_d_n6, eq134_e1710_d_n7, eq134_e1710_d_n8, eq134_e1710_d_n9, eq134_e1710_d_n12, eq134_e1710_d_n14, eq134_e1710_d_n15, eq134_e1710_d_n16, eq134_e1710_d_n17, eq134_e1710_d_n18, eq134_e1710_d_n19, eq134_e1710_d_n20, eq134_e1710_d_n21, eq134_e1710_d_n22];
        let eq134_branch_derivative_indices: [usize; 0] = [];
        let eq134_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(19),
            multiplicity * (eq134_value),
            &eq134_node_derivative_indices,
            &eq134_node_derivatives,
            &eq134_branch_derivative_indices,
            &eq134_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_5(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
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
        var_guard541: f64,
        var_guard542: f64,
        var_guard543: f64,
        var_guard544: f64,
        var_guard545: f64,
        var_qd_fp1s: f64,
        var_qd_fp1s_dn0: f64,
        var_qd_fp1s_dn1: f64,
        var_qd_fp1s_dn12: f64,
        var_qd_fp1s_dn14: f64,
        var_qd_fp1s_dn15: f64,
        var_qd_fp1s_dn16: f64,
        var_qd_fp1s_dn17: f64,
        var_qd_fp1s_dn18: f64,
        var_qd_fp1s_dn19: f64,
        var_qd_fp1s_dn2: f64,
        var_qd_fp1s_dn20: f64,
        var_qd_fp1s_dn21: f64,
        var_qd_fp1s_dn22: f64,
        var_qd_fp1s_dn3: f64,
        var_qd_fp1s_dn4: f64,
        var_qd_fp1s_dn5: f64,
        var_qd_fp1s_dn6: f64,
        var_qd_fp1s_dn7: f64,
        var_qd_fp1s_dn8: f64,
        var_qd_fp1s_dn9: f64,
        var_qg_fp1s: f64,
        var_qg_fp1s_dn0: f64,
        var_qg_fp1s_dn1: f64,
        var_qg_fp1s_dn12: f64,
        var_qg_fp1s_dn14: f64,
        var_qg_fp1s_dn15: f64,
        var_qg_fp1s_dn16: f64,
        var_qg_fp1s_dn17: f64,
        var_qg_fp1s_dn18: f64,
        var_qg_fp1s_dn19: f64,
        var_qg_fp1s_dn2: f64,
        var_qg_fp1s_dn20: f64,
        var_qg_fp1s_dn21: f64,
        var_qg_fp1s_dn22: f64,
        var_qg_fp1s_dn3: f64,
        var_qg_fp1s_dn4: f64,
        var_qg_fp1s_dn5: f64,
        var_qg_fp1s_dn6: f64,
        var_qg_fp1s_dn7: f64,
        var_qg_fp1s_dn8: f64,
        var_qg_fp1s_dn9: f64,
    ) {
        let (eq135_e1722, eq135_e1722_d_n0, eq135_e1722_d_n1, eq135_e1722_d_n2, eq135_e1722_d_n3, eq135_e1722_d_n4, eq135_e1722_d_n5, eq135_e1722_d_n6, eq135_e1722_d_n7, eq135_e1722_d_n8, eq135_e1722_d_n9, eq135_e1722_d_n12, eq135_e1722_d_n14, eq135_e1722_d_n15, eq135_e1722_d_n16, eq135_e1722_d_n17, eq135_e1722_d_n18, eq135_e1722_d_n19, eq135_e1722_d_n20, eq135_e1722_d_n21, eq135_e1722_d_n22,) = {
    if (((var_guard541 != 0.0) && (var_guard542 != 0.0)) && (var_guard543 == 0.0)) {
        let eq135_e1719: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 34, var_qg_fp1s);
        let eq135_e1720: f64 = (p.p7 * eq135_e1719);
        let eq135_e1720_d_n0: f64 = (p.p7 * (var_qg_fp1s_dn0 * ddt_scale));
        let eq135_e1720_d_n1: f64 = (p.p7 * (var_qg_fp1s_dn1 * ddt_scale));
        let eq135_e1720_d_n2: f64 = (p.p7 * (var_qg_fp1s_dn2 * ddt_scale));
        let eq135_e1720_d_n3: f64 = (p.p7 * (var_qg_fp1s_dn3 * ddt_scale));
        let eq135_e1720_d_n4: f64 = (p.p7 * (var_qg_fp1s_dn4 * ddt_scale));
        let eq135_e1720_d_n5: f64 = (p.p7 * (var_qg_fp1s_dn5 * ddt_scale));
        let eq135_e1720_d_n6: f64 = (p.p7 * (var_qg_fp1s_dn6 * ddt_scale));
        let eq135_e1720_d_n7: f64 = (p.p7 * (var_qg_fp1s_dn7 * ddt_scale));
        let eq135_e1720_d_n8: f64 = (p.p7 * (var_qg_fp1s_dn8 * ddt_scale));
        let eq135_e1720_d_n9: f64 = (p.p7 * (var_qg_fp1s_dn9 * ddt_scale));
        let eq135_e1720_d_n12: f64 = (p.p7 * (var_qg_fp1s_dn12 * ddt_scale));
        let eq135_e1720_d_n14: f64 = (p.p7 * (var_qg_fp1s_dn14 * ddt_scale));
        let eq135_e1720_d_n15: f64 = (p.p7 * (var_qg_fp1s_dn15 * ddt_scale));
        let eq135_e1720_d_n16: f64 = (p.p7 * (var_qg_fp1s_dn16 * ddt_scale));
        let eq135_e1720_d_n17: f64 = (p.p7 * (var_qg_fp1s_dn17 * ddt_scale));
        let eq135_e1720_d_n18: f64 = (p.p7 * (var_qg_fp1s_dn18 * ddt_scale));
        let eq135_e1720_d_n19: f64 = (p.p7 * (var_qg_fp1s_dn19 * ddt_scale));
        let eq135_e1720_d_n20: f64 = (p.p7 * (var_qg_fp1s_dn20 * ddt_scale));
        let eq135_e1720_d_n21: f64 = (p.p7 * (var_qg_fp1s_dn21 * ddt_scale));
        let eq135_e1720_d_n22: f64 = (p.p7 * (var_qg_fp1s_dn22 * ddt_scale));
        (eq135_e1720, eq135_e1720_d_n0, eq135_e1720_d_n1, eq135_e1720_d_n2, eq135_e1720_d_n3, eq135_e1720_d_n4, eq135_e1720_d_n5, eq135_e1720_d_n6, eq135_e1720_d_n7, eq135_e1720_d_n8, eq135_e1720_d_n9, eq135_e1720_d_n12, eq135_e1720_d_n14, eq135_e1720_d_n15, eq135_e1720_d_n16, eq135_e1720_d_n17, eq135_e1720_d_n18, eq135_e1720_d_n19, eq135_e1720_d_n20, eq135_e1720_d_n21, eq135_e1720_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq135_value: f64 = eq135_e1722;
        let eq135_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq135_node_derivatives: [f64; 20] = [eq135_e1722_d_n0, eq135_e1722_d_n1, eq135_e1722_d_n2, eq135_e1722_d_n3, eq135_e1722_d_n4, eq135_e1722_d_n5, eq135_e1722_d_n6, eq135_e1722_d_n7, eq135_e1722_d_n8, eq135_e1722_d_n9, eq135_e1722_d_n12, eq135_e1722_d_n14, eq135_e1722_d_n15, eq135_e1722_d_n16, eq135_e1722_d_n17, eq135_e1722_d_n18, eq135_e1722_d_n19, eq135_e1722_d_n20, eq135_e1722_d_n21, eq135_e1722_d_n22];
        let eq135_branch_derivative_indices: [usize; 0] = [];
        let eq135_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(19),
            multiplicity * (eq135_value),
            &eq135_node_derivative_indices,
            &eq135_node_derivatives,
            &eq135_branch_derivative_indices,
            &eq135_branch_derivatives,
            multiplicity,
        );
        let (eq136_e1736, eq136_e1736_d_n0, eq136_e1736_d_n1, eq136_e1736_d_n2, eq136_e1736_d_n3, eq136_e1736_d_n4, eq136_e1736_d_n5, eq136_e1736_d_n6, eq136_e1736_d_n7, eq136_e1736_d_n8, eq136_e1736_d_n9, eq136_e1736_d_n12, eq136_e1736_d_n14, eq136_e1736_d_n15, eq136_e1736_d_n16, eq136_e1736_d_n17, eq136_e1736_d_n18, eq136_e1736_d_n19, eq136_e1736_d_n20, eq136_e1736_d_n21, eq136_e1736_d_n22,) = {
    if (((var_guard541 != 0.0) && (var_guard542 != 0.0)) && (var_guard543 == 0.0)) {
        let eq136_e1731: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 35, var_qg_fp1s);
        let eq136_e1732: f64 = (p.p7 * eq136_e1731);
        let eq136_e1732_d_n0: f64 = (p.p7 * (var_qg_fp1s_dn0 * ddt_scale));
        let eq136_e1732_d_n1: f64 = (p.p7 * (var_qg_fp1s_dn1 * ddt_scale));
        let eq136_e1732_d_n2: f64 = (p.p7 * (var_qg_fp1s_dn2 * ddt_scale));
        let eq136_e1732_d_n3: f64 = (p.p7 * (var_qg_fp1s_dn3 * ddt_scale));
        let eq136_e1732_d_n4: f64 = (p.p7 * (var_qg_fp1s_dn4 * ddt_scale));
        let eq136_e1732_d_n5: f64 = (p.p7 * (var_qg_fp1s_dn5 * ddt_scale));
        let eq136_e1732_d_n6: f64 = (p.p7 * (var_qg_fp1s_dn6 * ddt_scale));
        let eq136_e1732_d_n7: f64 = (p.p7 * (var_qg_fp1s_dn7 * ddt_scale));
        let eq136_e1732_d_n8: f64 = (p.p7 * (var_qg_fp1s_dn8 * ddt_scale));
        let eq136_e1732_d_n9: f64 = (p.p7 * (var_qg_fp1s_dn9 * ddt_scale));
        let eq136_e1732_d_n12: f64 = (p.p7 * (var_qg_fp1s_dn12 * ddt_scale));
        let eq136_e1732_d_n14: f64 = (p.p7 * (var_qg_fp1s_dn14 * ddt_scale));
        let eq136_e1732_d_n15: f64 = (p.p7 * (var_qg_fp1s_dn15 * ddt_scale));
        let eq136_e1732_d_n16: f64 = (p.p7 * (var_qg_fp1s_dn16 * ddt_scale));
        let eq136_e1732_d_n17: f64 = (p.p7 * (var_qg_fp1s_dn17 * ddt_scale));
        let eq136_e1732_d_n18: f64 = (p.p7 * (var_qg_fp1s_dn18 * ddt_scale));
        let eq136_e1732_d_n19: f64 = (p.p7 * (var_qg_fp1s_dn19 * ddt_scale));
        let eq136_e1732_d_n20: f64 = (p.p7 * (var_qg_fp1s_dn20 * ddt_scale));
        let eq136_e1732_d_n21: f64 = (p.p7 * (var_qg_fp1s_dn21 * ddt_scale));
        let eq136_e1732_d_n22: f64 = (p.p7 * (var_qg_fp1s_dn22 * ddt_scale));
        let eq136_e1734: f64 = (eq136_e1732 * p.p246);
        let eq136_e1734_d_n0: f64 = (eq136_e1732_d_n0 * p.p246);
        let eq136_e1734_d_n1: f64 = (eq136_e1732_d_n1 * p.p246);
        let eq136_e1734_d_n2: f64 = (eq136_e1732_d_n2 * p.p246);
        let eq136_e1734_d_n3: f64 = (eq136_e1732_d_n3 * p.p246);
        let eq136_e1734_d_n4: f64 = (eq136_e1732_d_n4 * p.p246);
        let eq136_e1734_d_n5: f64 = (eq136_e1732_d_n5 * p.p246);
        let eq136_e1734_d_n6: f64 = (eq136_e1732_d_n6 * p.p246);
        let eq136_e1734_d_n7: f64 = (eq136_e1732_d_n7 * p.p246);
        let eq136_e1734_d_n8: f64 = (eq136_e1732_d_n8 * p.p246);
        let eq136_e1734_d_n9: f64 = (eq136_e1732_d_n9 * p.p246);
        let eq136_e1734_d_n12: f64 = (eq136_e1732_d_n12 * p.p246);
        let eq136_e1734_d_n14: f64 = (eq136_e1732_d_n14 * p.p246);
        let eq136_e1734_d_n15: f64 = (eq136_e1732_d_n15 * p.p246);
        let eq136_e1734_d_n16: f64 = (eq136_e1732_d_n16 * p.p246);
        let eq136_e1734_d_n17: f64 = (eq136_e1732_d_n17 * p.p246);
        let eq136_e1734_d_n18: f64 = (eq136_e1732_d_n18 * p.p246);
        let eq136_e1734_d_n19: f64 = (eq136_e1732_d_n19 * p.p246);
        let eq136_e1734_d_n20: f64 = (eq136_e1732_d_n20 * p.p246);
        let eq136_e1734_d_n21: f64 = (eq136_e1732_d_n21 * p.p246);
        let eq136_e1734_d_n22: f64 = (eq136_e1732_d_n22 * p.p246);
        (eq136_e1734, eq136_e1734_d_n0, eq136_e1734_d_n1, eq136_e1734_d_n2, eq136_e1734_d_n3, eq136_e1734_d_n4, eq136_e1734_d_n5, eq136_e1734_d_n6, eq136_e1734_d_n7, eq136_e1734_d_n8, eq136_e1734_d_n9, eq136_e1734_d_n12, eq136_e1734_d_n14, eq136_e1734_d_n15, eq136_e1734_d_n16, eq136_e1734_d_n17, eq136_e1734_d_n18, eq136_e1734_d_n19, eq136_e1734_d_n20, eq136_e1734_d_n21, eq136_e1734_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq136_value: f64 = eq136_e1736;
        let eq136_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq136_node_derivatives: [f64; 20] = [eq136_e1736_d_n0, eq136_e1736_d_n1, eq136_e1736_d_n2, eq136_e1736_d_n3, eq136_e1736_d_n4, eq136_e1736_d_n5, eq136_e1736_d_n6, eq136_e1736_d_n7, eq136_e1736_d_n8, eq136_e1736_d_n9, eq136_e1736_d_n12, eq136_e1736_d_n14, eq136_e1736_d_n15, eq136_e1736_d_n16, eq136_e1736_d_n17, eq136_e1736_d_n18, eq136_e1736_d_n19, eq136_e1736_d_n20, eq136_e1736_d_n21, eq136_e1736_d_n22];
        let eq136_branch_derivative_indices: [usize; 0] = [];
        let eq136_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(19),
            multiplicity * (eq136_value),
            &eq136_node_derivative_indices,
            &eq136_node_derivatives,
            &eq136_branch_derivative_indices,
            &eq136_branch_derivatives,
            multiplicity,
        );
        let (eq137_e1747, eq137_e1747_d_n0, eq137_e1747_d_n1, eq137_e1747_d_n2, eq137_e1747_d_n3, eq137_e1747_d_n4, eq137_e1747_d_n5, eq137_e1747_d_n6, eq137_e1747_d_n7, eq137_e1747_d_n8, eq137_e1747_d_n9, eq137_e1747_d_n12, eq137_e1747_d_n14, eq137_e1747_d_n15, eq137_e1747_d_n16, eq137_e1747_d_n17, eq137_e1747_d_n18, eq137_e1747_d_n19, eq137_e1747_d_n20, eq137_e1747_d_n21, eq137_e1747_d_n22,) = {
    if ((var_guard541 != 0.0) && (var_guard542 != 0.0)) {
        let eq137_e1743: f64 = (p.p251 * var_qg_fp1s);
        let eq137_e1743_d_n0: f64 = (p.p251 * var_qg_fp1s_dn0);
        let eq137_e1743_d_n1: f64 = (p.p251 * var_qg_fp1s_dn1);
        let eq137_e1743_d_n2: f64 = (p.p251 * var_qg_fp1s_dn2);
        let eq137_e1743_d_n3: f64 = (p.p251 * var_qg_fp1s_dn3);
        let eq137_e1743_d_n4: f64 = (p.p251 * var_qg_fp1s_dn4);
        let eq137_e1743_d_n5: f64 = (p.p251 * var_qg_fp1s_dn5);
        let eq137_e1743_d_n6: f64 = (p.p251 * var_qg_fp1s_dn6);
        let eq137_e1743_d_n7: f64 = (p.p251 * var_qg_fp1s_dn7);
        let eq137_e1743_d_n8: f64 = (p.p251 * var_qg_fp1s_dn8);
        let eq137_e1743_d_n9: f64 = (p.p251 * var_qg_fp1s_dn9);
        let eq137_e1743_d_n12: f64 = (p.p251 * var_qg_fp1s_dn12);
        let eq137_e1743_d_n14: f64 = (p.p251 * var_qg_fp1s_dn14);
        let eq137_e1743_d_n15: f64 = (p.p251 * var_qg_fp1s_dn15);
        let eq137_e1743_d_n16: f64 = (p.p251 * var_qg_fp1s_dn16);
        let eq137_e1743_d_n17: f64 = (p.p251 * var_qg_fp1s_dn17);
        let eq137_e1743_d_n18: f64 = (p.p251 * var_qg_fp1s_dn18);
        let eq137_e1743_d_n19: f64 = (p.p251 * var_qg_fp1s_dn19);
        let eq137_e1743_d_n20: f64 = (p.p251 * var_qg_fp1s_dn20);
        let eq137_e1743_d_n21: f64 = (p.p251 * var_qg_fp1s_dn21);
        let eq137_e1743_d_n22: f64 = (p.p251 * var_qg_fp1s_dn22);
        let eq137_e1744: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 36, eq137_e1743);
        let eq137_e1745: f64 = (p.p7 * eq137_e1744);
        let eq137_e1745_d_n0: f64 = (p.p7 * (eq137_e1743_d_n0 * ddt_scale));
        let eq137_e1745_d_n1: f64 = (p.p7 * (eq137_e1743_d_n1 * ddt_scale));
        let eq137_e1745_d_n2: f64 = (p.p7 * (eq137_e1743_d_n2 * ddt_scale));
        let eq137_e1745_d_n3: f64 = (p.p7 * (eq137_e1743_d_n3 * ddt_scale));
        let eq137_e1745_d_n4: f64 = (p.p7 * (eq137_e1743_d_n4 * ddt_scale));
        let eq137_e1745_d_n5: f64 = (p.p7 * (eq137_e1743_d_n5 * ddt_scale));
        let eq137_e1745_d_n6: f64 = (p.p7 * (eq137_e1743_d_n6 * ddt_scale));
        let eq137_e1745_d_n7: f64 = (p.p7 * (eq137_e1743_d_n7 * ddt_scale));
        let eq137_e1745_d_n8: f64 = (p.p7 * (eq137_e1743_d_n8 * ddt_scale));
        let eq137_e1745_d_n9: f64 = (p.p7 * (eq137_e1743_d_n9 * ddt_scale));
        let eq137_e1745_d_n12: f64 = (p.p7 * (eq137_e1743_d_n12 * ddt_scale));
        let eq137_e1745_d_n14: f64 = (p.p7 * (eq137_e1743_d_n14 * ddt_scale));
        let eq137_e1745_d_n15: f64 = (p.p7 * (eq137_e1743_d_n15 * ddt_scale));
        let eq137_e1745_d_n16: f64 = (p.p7 * (eq137_e1743_d_n16 * ddt_scale));
        let eq137_e1745_d_n17: f64 = (p.p7 * (eq137_e1743_d_n17 * ddt_scale));
        let eq137_e1745_d_n18: f64 = (p.p7 * (eq137_e1743_d_n18 * ddt_scale));
        let eq137_e1745_d_n19: f64 = (p.p7 * (eq137_e1743_d_n19 * ddt_scale));
        let eq137_e1745_d_n20: f64 = (p.p7 * (eq137_e1743_d_n20 * ddt_scale));
        let eq137_e1745_d_n21: f64 = (p.p7 * (eq137_e1743_d_n21 * ddt_scale));
        let eq137_e1745_d_n22: f64 = (p.p7 * (eq137_e1743_d_n22 * ddt_scale));
        (eq137_e1745, eq137_e1745_d_n0, eq137_e1745_d_n1, eq137_e1745_d_n2, eq137_e1745_d_n3, eq137_e1745_d_n4, eq137_e1745_d_n5, eq137_e1745_d_n6, eq137_e1745_d_n7, eq137_e1745_d_n8, eq137_e1745_d_n9, eq137_e1745_d_n12, eq137_e1745_d_n14, eq137_e1745_d_n15, eq137_e1745_d_n16, eq137_e1745_d_n17, eq137_e1745_d_n18, eq137_e1745_d_n19, eq137_e1745_d_n20, eq137_e1745_d_n21, eq137_e1745_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq137_value: f64 = eq137_e1747;
        let eq137_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq137_node_derivatives: [f64; 20] = [eq137_e1747_d_n0, eq137_e1747_d_n1, eq137_e1747_d_n2, eq137_e1747_d_n3, eq137_e1747_d_n4, eq137_e1747_d_n5, eq137_e1747_d_n6, eq137_e1747_d_n7, eq137_e1747_d_n8, eq137_e1747_d_n9, eq137_e1747_d_n12, eq137_e1747_d_n14, eq137_e1747_d_n15, eq137_e1747_d_n16, eq137_e1747_d_n17, eq137_e1747_d_n18, eq137_e1747_d_n19, eq137_e1747_d_n20, eq137_e1747_d_n21, eq137_e1747_d_n22];
        let eq137_branch_derivative_indices: [usize; 0] = [];
        let eq137_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(19),
            multiplicity * (eq137_value),
            &eq137_node_derivative_indices,
            &eq137_node_derivatives,
            &eq137_branch_derivative_indices,
            &eq137_branch_derivatives,
            multiplicity,
        );
        let (eq138_e1757, eq138_e1757_d_n0, eq138_e1757_d_n1, eq138_e1757_d_n2, eq138_e1757_d_n3, eq138_e1757_d_n4, eq138_e1757_d_n5, eq138_e1757_d_n6, eq138_e1757_d_n7, eq138_e1757_d_n8, eq138_e1757_d_n9, eq138_e1757_d_n12, eq138_e1757_d_n14, eq138_e1757_d_n15, eq138_e1757_d_n16, eq138_e1757_d_n17, eq138_e1757_d_n18, eq138_e1757_d_n19, eq138_e1757_d_n20, eq138_e1757_d_n21, eq138_e1757_d_n22,) = {
    if ((var_guard541 == 0.0) && (var_guard544 != 0.0)) {
        let eq138_e1754: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 37, var_qd_fp1s);
        let eq138_e1755: f64 = (p.p7 * eq138_e1754);
        let eq138_e1755_d_n0: f64 = (p.p7 * (var_qd_fp1s_dn0 * ddt_scale));
        let eq138_e1755_d_n1: f64 = (p.p7 * (var_qd_fp1s_dn1 * ddt_scale));
        let eq138_e1755_d_n2: f64 = (p.p7 * (var_qd_fp1s_dn2 * ddt_scale));
        let eq138_e1755_d_n3: f64 = (p.p7 * (var_qd_fp1s_dn3 * ddt_scale));
        let eq138_e1755_d_n4: f64 = (p.p7 * (var_qd_fp1s_dn4 * ddt_scale));
        let eq138_e1755_d_n5: f64 = (p.p7 * (var_qd_fp1s_dn5 * ddt_scale));
        let eq138_e1755_d_n6: f64 = (p.p7 * (var_qd_fp1s_dn6 * ddt_scale));
        let eq138_e1755_d_n7: f64 = (p.p7 * (var_qd_fp1s_dn7 * ddt_scale));
        let eq138_e1755_d_n8: f64 = (p.p7 * (var_qd_fp1s_dn8 * ddt_scale));
        let eq138_e1755_d_n9: f64 = (p.p7 * (var_qd_fp1s_dn9 * ddt_scale));
        let eq138_e1755_d_n12: f64 = (p.p7 * (var_qd_fp1s_dn12 * ddt_scale));
        let eq138_e1755_d_n14: f64 = (p.p7 * (var_qd_fp1s_dn14 * ddt_scale));
        let eq138_e1755_d_n15: f64 = (p.p7 * (var_qd_fp1s_dn15 * ddt_scale));
        let eq138_e1755_d_n16: f64 = (p.p7 * (var_qd_fp1s_dn16 * ddt_scale));
        let eq138_e1755_d_n17: f64 = (p.p7 * (var_qd_fp1s_dn17 * ddt_scale));
        let eq138_e1755_d_n18: f64 = (p.p7 * (var_qd_fp1s_dn18 * ddt_scale));
        let eq138_e1755_d_n19: f64 = (p.p7 * (var_qd_fp1s_dn19 * ddt_scale));
        let eq138_e1755_d_n20: f64 = (p.p7 * (var_qd_fp1s_dn20 * ddt_scale));
        let eq138_e1755_d_n21: f64 = (p.p7 * (var_qd_fp1s_dn21 * ddt_scale));
        let eq138_e1755_d_n22: f64 = (p.p7 * (var_qd_fp1s_dn22 * ddt_scale));
        (eq138_e1755, eq138_e1755_d_n0, eq138_e1755_d_n1, eq138_e1755_d_n2, eq138_e1755_d_n3, eq138_e1755_d_n4, eq138_e1755_d_n5, eq138_e1755_d_n6, eq138_e1755_d_n7, eq138_e1755_d_n8, eq138_e1755_d_n9, eq138_e1755_d_n12, eq138_e1755_d_n14, eq138_e1755_d_n15, eq138_e1755_d_n16, eq138_e1755_d_n17, eq138_e1755_d_n18, eq138_e1755_d_n19, eq138_e1755_d_n20, eq138_e1755_d_n21, eq138_e1755_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq138_value: f64 = eq138_e1757;
        let eq138_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq138_node_derivatives: [f64; 20] = [eq138_e1757_d_n0, eq138_e1757_d_n1, eq138_e1757_d_n2, eq138_e1757_d_n3, eq138_e1757_d_n4, eq138_e1757_d_n5, eq138_e1757_d_n6, eq138_e1757_d_n7, eq138_e1757_d_n8, eq138_e1757_d_n9, eq138_e1757_d_n12, eq138_e1757_d_n14, eq138_e1757_d_n15, eq138_e1757_d_n16, eq138_e1757_d_n17, eq138_e1757_d_n18, eq138_e1757_d_n19, eq138_e1757_d_n20, eq138_e1757_d_n21, eq138_e1757_d_n22];
        let eq138_branch_derivative_indices: [usize; 0] = [];
        let eq138_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(2),
            multiplicity * (eq138_value),
            &eq138_node_derivative_indices,
            &eq138_node_derivatives,
            &eq138_branch_derivative_indices,
            &eq138_branch_derivatives,
            multiplicity,
        );
        let (eq139_e1769, eq139_e1769_d_n0, eq139_e1769_d_n1, eq139_e1769_d_n2, eq139_e1769_d_n3, eq139_e1769_d_n4, eq139_e1769_d_n5, eq139_e1769_d_n6, eq139_e1769_d_n7, eq139_e1769_d_n8, eq139_e1769_d_n9, eq139_e1769_d_n12, eq139_e1769_d_n14, eq139_e1769_d_n15, eq139_e1769_d_n16, eq139_e1769_d_n17, eq139_e1769_d_n18, eq139_e1769_d_n19, eq139_e1769_d_n20, eq139_e1769_d_n21, eq139_e1769_d_n22,) = {
    if (((var_guard541 == 0.0) && (var_guard544 != 0.0)) && (var_guard545 != 0.0)) {
        let eq139_e1766: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 38, var_qg_fp1s);
        let eq139_e1767: f64 = (p.p7 * eq139_e1766);
        let eq139_e1767_d_n0: f64 = (p.p7 * (var_qg_fp1s_dn0 * ddt_scale));
        let eq139_e1767_d_n1: f64 = (p.p7 * (var_qg_fp1s_dn1 * ddt_scale));
        let eq139_e1767_d_n2: f64 = (p.p7 * (var_qg_fp1s_dn2 * ddt_scale));
        let eq139_e1767_d_n3: f64 = (p.p7 * (var_qg_fp1s_dn3 * ddt_scale));
        let eq139_e1767_d_n4: f64 = (p.p7 * (var_qg_fp1s_dn4 * ddt_scale));
        let eq139_e1767_d_n5: f64 = (p.p7 * (var_qg_fp1s_dn5 * ddt_scale));
        let eq139_e1767_d_n6: f64 = (p.p7 * (var_qg_fp1s_dn6 * ddt_scale));
        let eq139_e1767_d_n7: f64 = (p.p7 * (var_qg_fp1s_dn7 * ddt_scale));
        let eq139_e1767_d_n8: f64 = (p.p7 * (var_qg_fp1s_dn8 * ddt_scale));
        let eq139_e1767_d_n9: f64 = (p.p7 * (var_qg_fp1s_dn9 * ddt_scale));
        let eq139_e1767_d_n12: f64 = (p.p7 * (var_qg_fp1s_dn12 * ddt_scale));
        let eq139_e1767_d_n14: f64 = (p.p7 * (var_qg_fp1s_dn14 * ddt_scale));
        let eq139_e1767_d_n15: f64 = (p.p7 * (var_qg_fp1s_dn15 * ddt_scale));
        let eq139_e1767_d_n16: f64 = (p.p7 * (var_qg_fp1s_dn16 * ddt_scale));
        let eq139_e1767_d_n17: f64 = (p.p7 * (var_qg_fp1s_dn17 * ddt_scale));
        let eq139_e1767_d_n18: f64 = (p.p7 * (var_qg_fp1s_dn18 * ddt_scale));
        let eq139_e1767_d_n19: f64 = (p.p7 * (var_qg_fp1s_dn19 * ddt_scale));
        let eq139_e1767_d_n20: f64 = (p.p7 * (var_qg_fp1s_dn20 * ddt_scale));
        let eq139_e1767_d_n21: f64 = (p.p7 * (var_qg_fp1s_dn21 * ddt_scale));
        let eq139_e1767_d_n22: f64 = (p.p7 * (var_qg_fp1s_dn22 * ddt_scale));
        (eq139_e1767, eq139_e1767_d_n0, eq139_e1767_d_n1, eq139_e1767_d_n2, eq139_e1767_d_n3, eq139_e1767_d_n4, eq139_e1767_d_n5, eq139_e1767_d_n6, eq139_e1767_d_n7, eq139_e1767_d_n8, eq139_e1767_d_n9, eq139_e1767_d_n12, eq139_e1767_d_n14, eq139_e1767_d_n15, eq139_e1767_d_n16, eq139_e1767_d_n17, eq139_e1767_d_n18, eq139_e1767_d_n19, eq139_e1767_d_n20, eq139_e1767_d_n21, eq139_e1767_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq139_value: f64 = eq139_e1769;
        let eq139_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq139_node_derivatives: [f64; 20] = [eq139_e1769_d_n0, eq139_e1769_d_n1, eq139_e1769_d_n2, eq139_e1769_d_n3, eq139_e1769_d_n4, eq139_e1769_d_n5, eq139_e1769_d_n6, eq139_e1769_d_n7, eq139_e1769_d_n8, eq139_e1769_d_n9, eq139_e1769_d_n12, eq139_e1769_d_n14, eq139_e1769_d_n15, eq139_e1769_d_n16, eq139_e1769_d_n17, eq139_e1769_d_n18, eq139_e1769_d_n19, eq139_e1769_d_n20, eq139_e1769_d_n21, eq139_e1769_d_n22];
        let eq139_branch_derivative_indices: [usize; 0] = [];
        let eq139_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq139_value),
            &eq139_node_derivative_indices,
            &eq139_node_derivatives,
            &eq139_branch_derivative_indices,
            &eq139_branch_derivatives,
            multiplicity,
        );
        let (eq140_e1783, eq140_e1783_d_n0, eq140_e1783_d_n1, eq140_e1783_d_n2, eq140_e1783_d_n3, eq140_e1783_d_n4, eq140_e1783_d_n5, eq140_e1783_d_n6, eq140_e1783_d_n7, eq140_e1783_d_n8, eq140_e1783_d_n9, eq140_e1783_d_n12, eq140_e1783_d_n14, eq140_e1783_d_n15, eq140_e1783_d_n16, eq140_e1783_d_n17, eq140_e1783_d_n18, eq140_e1783_d_n19, eq140_e1783_d_n20, eq140_e1783_d_n21, eq140_e1783_d_n22,) = {
    if (((var_guard541 == 0.0) && (var_guard544 != 0.0)) && (var_guard545 != 0.0)) {
        let eq140_e1778: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 39, var_qg_fp1s);
        let eq140_e1779: f64 = (p.p7 * eq140_e1778);
        let eq140_e1779_d_n0: f64 = (p.p7 * (var_qg_fp1s_dn0 * ddt_scale));
        let eq140_e1779_d_n1: f64 = (p.p7 * (var_qg_fp1s_dn1 * ddt_scale));
        let eq140_e1779_d_n2: f64 = (p.p7 * (var_qg_fp1s_dn2 * ddt_scale));
        let eq140_e1779_d_n3: f64 = (p.p7 * (var_qg_fp1s_dn3 * ddt_scale));
        let eq140_e1779_d_n4: f64 = (p.p7 * (var_qg_fp1s_dn4 * ddt_scale));
        let eq140_e1779_d_n5: f64 = (p.p7 * (var_qg_fp1s_dn5 * ddt_scale));
        let eq140_e1779_d_n6: f64 = (p.p7 * (var_qg_fp1s_dn6 * ddt_scale));
        let eq140_e1779_d_n7: f64 = (p.p7 * (var_qg_fp1s_dn7 * ddt_scale));
        let eq140_e1779_d_n8: f64 = (p.p7 * (var_qg_fp1s_dn8 * ddt_scale));
        let eq140_e1779_d_n9: f64 = (p.p7 * (var_qg_fp1s_dn9 * ddt_scale));
        let eq140_e1779_d_n12: f64 = (p.p7 * (var_qg_fp1s_dn12 * ddt_scale));
        let eq140_e1779_d_n14: f64 = (p.p7 * (var_qg_fp1s_dn14 * ddt_scale));
        let eq140_e1779_d_n15: f64 = (p.p7 * (var_qg_fp1s_dn15 * ddt_scale));
        let eq140_e1779_d_n16: f64 = (p.p7 * (var_qg_fp1s_dn16 * ddt_scale));
        let eq140_e1779_d_n17: f64 = (p.p7 * (var_qg_fp1s_dn17 * ddt_scale));
        let eq140_e1779_d_n18: f64 = (p.p7 * (var_qg_fp1s_dn18 * ddt_scale));
        let eq140_e1779_d_n19: f64 = (p.p7 * (var_qg_fp1s_dn19 * ddt_scale));
        let eq140_e1779_d_n20: f64 = (p.p7 * (var_qg_fp1s_dn20 * ddt_scale));
        let eq140_e1779_d_n21: f64 = (p.p7 * (var_qg_fp1s_dn21 * ddt_scale));
        let eq140_e1779_d_n22: f64 = (p.p7 * (var_qg_fp1s_dn22 * ddt_scale));
        let eq140_e1781: f64 = (eq140_e1779 * p.p246);
        let eq140_e1781_d_n0: f64 = (eq140_e1779_d_n0 * p.p246);
        let eq140_e1781_d_n1: f64 = (eq140_e1779_d_n1 * p.p246);
        let eq140_e1781_d_n2: f64 = (eq140_e1779_d_n2 * p.p246);
        let eq140_e1781_d_n3: f64 = (eq140_e1779_d_n3 * p.p246);
        let eq140_e1781_d_n4: f64 = (eq140_e1779_d_n4 * p.p246);
        let eq140_e1781_d_n5: f64 = (eq140_e1779_d_n5 * p.p246);
        let eq140_e1781_d_n6: f64 = (eq140_e1779_d_n6 * p.p246);
        let eq140_e1781_d_n7: f64 = (eq140_e1779_d_n7 * p.p246);
        let eq140_e1781_d_n8: f64 = (eq140_e1779_d_n8 * p.p246);
        let eq140_e1781_d_n9: f64 = (eq140_e1779_d_n9 * p.p246);
        let eq140_e1781_d_n12: f64 = (eq140_e1779_d_n12 * p.p246);
        let eq140_e1781_d_n14: f64 = (eq140_e1779_d_n14 * p.p246);
        let eq140_e1781_d_n15: f64 = (eq140_e1779_d_n15 * p.p246);
        let eq140_e1781_d_n16: f64 = (eq140_e1779_d_n16 * p.p246);
        let eq140_e1781_d_n17: f64 = (eq140_e1779_d_n17 * p.p246);
        let eq140_e1781_d_n18: f64 = (eq140_e1779_d_n18 * p.p246);
        let eq140_e1781_d_n19: f64 = (eq140_e1779_d_n19 * p.p246);
        let eq140_e1781_d_n20: f64 = (eq140_e1779_d_n20 * p.p246);
        let eq140_e1781_d_n21: f64 = (eq140_e1779_d_n21 * p.p246);
        let eq140_e1781_d_n22: f64 = (eq140_e1779_d_n22 * p.p246);
        (eq140_e1781, eq140_e1781_d_n0, eq140_e1781_d_n1, eq140_e1781_d_n2, eq140_e1781_d_n3, eq140_e1781_d_n4, eq140_e1781_d_n5, eq140_e1781_d_n6, eq140_e1781_d_n7, eq140_e1781_d_n8, eq140_e1781_d_n9, eq140_e1781_d_n12, eq140_e1781_d_n14, eq140_e1781_d_n15, eq140_e1781_d_n16, eq140_e1781_d_n17, eq140_e1781_d_n18, eq140_e1781_d_n19, eq140_e1781_d_n20, eq140_e1781_d_n21, eq140_e1781_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq140_value: f64 = eq140_e1783;
        let eq140_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq140_node_derivatives: [f64; 20] = [eq140_e1783_d_n0, eq140_e1783_d_n1, eq140_e1783_d_n2, eq140_e1783_d_n3, eq140_e1783_d_n4, eq140_e1783_d_n5, eq140_e1783_d_n6, eq140_e1783_d_n7, eq140_e1783_d_n8, eq140_e1783_d_n9, eq140_e1783_d_n12, eq140_e1783_d_n14, eq140_e1783_d_n15, eq140_e1783_d_n16, eq140_e1783_d_n17, eq140_e1783_d_n18, eq140_e1783_d_n19, eq140_e1783_d_n20, eq140_e1783_d_n21, eq140_e1783_d_n22];
        let eq140_branch_derivative_indices: [usize; 0] = [];
        let eq140_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq140_value),
            &eq140_node_derivative_indices,
            &eq140_node_derivatives,
            &eq140_branch_derivative_indices,
            &eq140_branch_derivatives,
            multiplicity,
        );
        let (eq141_e1796, eq141_e1796_d_n0, eq141_e1796_d_n1, eq141_e1796_d_n2, eq141_e1796_d_n3, eq141_e1796_d_n4, eq141_e1796_d_n5, eq141_e1796_d_n6, eq141_e1796_d_n7, eq141_e1796_d_n8, eq141_e1796_d_n9, eq141_e1796_d_n12, eq141_e1796_d_n14, eq141_e1796_d_n15, eq141_e1796_d_n16, eq141_e1796_d_n17, eq141_e1796_d_n18, eq141_e1796_d_n19, eq141_e1796_d_n20, eq141_e1796_d_n21, eq141_e1796_d_n22,) = {
    if (((var_guard541 == 0.0) && (var_guard544 != 0.0)) && (var_guard545 == 0.0)) {
        let eq141_e1793: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 40, var_qg_fp1s);
        let eq141_e1794: f64 = (p.p7 * eq141_e1793);
        let eq141_e1794_d_n0: f64 = (p.p7 * (var_qg_fp1s_dn0 * ddt_scale));
        let eq141_e1794_d_n1: f64 = (p.p7 * (var_qg_fp1s_dn1 * ddt_scale));
        let eq141_e1794_d_n2: f64 = (p.p7 * (var_qg_fp1s_dn2 * ddt_scale));
        let eq141_e1794_d_n3: f64 = (p.p7 * (var_qg_fp1s_dn3 * ddt_scale));
        let eq141_e1794_d_n4: f64 = (p.p7 * (var_qg_fp1s_dn4 * ddt_scale));
        let eq141_e1794_d_n5: f64 = (p.p7 * (var_qg_fp1s_dn5 * ddt_scale));
        let eq141_e1794_d_n6: f64 = (p.p7 * (var_qg_fp1s_dn6 * ddt_scale));
        let eq141_e1794_d_n7: f64 = (p.p7 * (var_qg_fp1s_dn7 * ddt_scale));
        let eq141_e1794_d_n8: f64 = (p.p7 * (var_qg_fp1s_dn8 * ddt_scale));
        let eq141_e1794_d_n9: f64 = (p.p7 * (var_qg_fp1s_dn9 * ddt_scale));
        let eq141_e1794_d_n12: f64 = (p.p7 * (var_qg_fp1s_dn12 * ddt_scale));
        let eq141_e1794_d_n14: f64 = (p.p7 * (var_qg_fp1s_dn14 * ddt_scale));
        let eq141_e1794_d_n15: f64 = (p.p7 * (var_qg_fp1s_dn15 * ddt_scale));
        let eq141_e1794_d_n16: f64 = (p.p7 * (var_qg_fp1s_dn16 * ddt_scale));
        let eq141_e1794_d_n17: f64 = (p.p7 * (var_qg_fp1s_dn17 * ddt_scale));
        let eq141_e1794_d_n18: f64 = (p.p7 * (var_qg_fp1s_dn18 * ddt_scale));
        let eq141_e1794_d_n19: f64 = (p.p7 * (var_qg_fp1s_dn19 * ddt_scale));
        let eq141_e1794_d_n20: f64 = (p.p7 * (var_qg_fp1s_dn20 * ddt_scale));
        let eq141_e1794_d_n21: f64 = (p.p7 * (var_qg_fp1s_dn21 * ddt_scale));
        let eq141_e1794_d_n22: f64 = (p.p7 * (var_qg_fp1s_dn22 * ddt_scale));
        (eq141_e1794, eq141_e1794_d_n0, eq141_e1794_d_n1, eq141_e1794_d_n2, eq141_e1794_d_n3, eq141_e1794_d_n4, eq141_e1794_d_n5, eq141_e1794_d_n6, eq141_e1794_d_n7, eq141_e1794_d_n8, eq141_e1794_d_n9, eq141_e1794_d_n12, eq141_e1794_d_n14, eq141_e1794_d_n15, eq141_e1794_d_n16, eq141_e1794_d_n17, eq141_e1794_d_n18, eq141_e1794_d_n19, eq141_e1794_d_n20, eq141_e1794_d_n21, eq141_e1794_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq141_value: f64 = eq141_e1796;
        let eq141_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq141_node_derivatives: [f64; 20] = [eq141_e1796_d_n0, eq141_e1796_d_n1, eq141_e1796_d_n2, eq141_e1796_d_n3, eq141_e1796_d_n4, eq141_e1796_d_n5, eq141_e1796_d_n6, eq141_e1796_d_n7, eq141_e1796_d_n8, eq141_e1796_d_n9, eq141_e1796_d_n12, eq141_e1796_d_n14, eq141_e1796_d_n15, eq141_e1796_d_n16, eq141_e1796_d_n17, eq141_e1796_d_n18, eq141_e1796_d_n19, eq141_e1796_d_n20, eq141_e1796_d_n21, eq141_e1796_d_n22];
        let eq141_branch_derivative_indices: [usize; 0] = [];
        let eq141_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq141_value),
            &eq141_node_derivative_indices,
            &eq141_node_derivatives,
            &eq141_branch_derivative_indices,
            &eq141_branch_derivatives,
            multiplicity,
        );
        let (eq142_e1811, eq142_e1811_d_n0, eq142_e1811_d_n1, eq142_e1811_d_n2, eq142_e1811_d_n3, eq142_e1811_d_n4, eq142_e1811_d_n5, eq142_e1811_d_n6, eq142_e1811_d_n7, eq142_e1811_d_n8, eq142_e1811_d_n9, eq142_e1811_d_n12, eq142_e1811_d_n14, eq142_e1811_d_n15, eq142_e1811_d_n16, eq142_e1811_d_n17, eq142_e1811_d_n18, eq142_e1811_d_n19, eq142_e1811_d_n20, eq142_e1811_d_n21, eq142_e1811_d_n22,) = {
    if (((var_guard541 == 0.0) && (var_guard544 != 0.0)) && (var_guard545 == 0.0)) {
        let eq142_e1806: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 41, var_qg_fp1s);
        let eq142_e1807: f64 = (p.p7 * eq142_e1806);
        let eq142_e1807_d_n0: f64 = (p.p7 * (var_qg_fp1s_dn0 * ddt_scale));
        let eq142_e1807_d_n1: f64 = (p.p7 * (var_qg_fp1s_dn1 * ddt_scale));
        let eq142_e1807_d_n2: f64 = (p.p7 * (var_qg_fp1s_dn2 * ddt_scale));
        let eq142_e1807_d_n3: f64 = (p.p7 * (var_qg_fp1s_dn3 * ddt_scale));
        let eq142_e1807_d_n4: f64 = (p.p7 * (var_qg_fp1s_dn4 * ddt_scale));
        let eq142_e1807_d_n5: f64 = (p.p7 * (var_qg_fp1s_dn5 * ddt_scale));
        let eq142_e1807_d_n6: f64 = (p.p7 * (var_qg_fp1s_dn6 * ddt_scale));
        let eq142_e1807_d_n7: f64 = (p.p7 * (var_qg_fp1s_dn7 * ddt_scale));
        let eq142_e1807_d_n8: f64 = (p.p7 * (var_qg_fp1s_dn8 * ddt_scale));
        let eq142_e1807_d_n9: f64 = (p.p7 * (var_qg_fp1s_dn9 * ddt_scale));
        let eq142_e1807_d_n12: f64 = (p.p7 * (var_qg_fp1s_dn12 * ddt_scale));
        let eq142_e1807_d_n14: f64 = (p.p7 * (var_qg_fp1s_dn14 * ddt_scale));
        let eq142_e1807_d_n15: f64 = (p.p7 * (var_qg_fp1s_dn15 * ddt_scale));
        let eq142_e1807_d_n16: f64 = (p.p7 * (var_qg_fp1s_dn16 * ddt_scale));
        let eq142_e1807_d_n17: f64 = (p.p7 * (var_qg_fp1s_dn17 * ddt_scale));
        let eq142_e1807_d_n18: f64 = (p.p7 * (var_qg_fp1s_dn18 * ddt_scale));
        let eq142_e1807_d_n19: f64 = (p.p7 * (var_qg_fp1s_dn19 * ddt_scale));
        let eq142_e1807_d_n20: f64 = (p.p7 * (var_qg_fp1s_dn20 * ddt_scale));
        let eq142_e1807_d_n21: f64 = (p.p7 * (var_qg_fp1s_dn21 * ddt_scale));
        let eq142_e1807_d_n22: f64 = (p.p7 * (var_qg_fp1s_dn22 * ddt_scale));
        let eq142_e1809: f64 = (eq142_e1807 * p.p246);
        let eq142_e1809_d_n0: f64 = (eq142_e1807_d_n0 * p.p246);
        let eq142_e1809_d_n1: f64 = (eq142_e1807_d_n1 * p.p246);
        let eq142_e1809_d_n2: f64 = (eq142_e1807_d_n2 * p.p246);
        let eq142_e1809_d_n3: f64 = (eq142_e1807_d_n3 * p.p246);
        let eq142_e1809_d_n4: f64 = (eq142_e1807_d_n4 * p.p246);
        let eq142_e1809_d_n5: f64 = (eq142_e1807_d_n5 * p.p246);
        let eq142_e1809_d_n6: f64 = (eq142_e1807_d_n6 * p.p246);
        let eq142_e1809_d_n7: f64 = (eq142_e1807_d_n7 * p.p246);
        let eq142_e1809_d_n8: f64 = (eq142_e1807_d_n8 * p.p246);
        let eq142_e1809_d_n9: f64 = (eq142_e1807_d_n9 * p.p246);
        let eq142_e1809_d_n12: f64 = (eq142_e1807_d_n12 * p.p246);
        let eq142_e1809_d_n14: f64 = (eq142_e1807_d_n14 * p.p246);
        let eq142_e1809_d_n15: f64 = (eq142_e1807_d_n15 * p.p246);
        let eq142_e1809_d_n16: f64 = (eq142_e1807_d_n16 * p.p246);
        let eq142_e1809_d_n17: f64 = (eq142_e1807_d_n17 * p.p246);
        let eq142_e1809_d_n18: f64 = (eq142_e1807_d_n18 * p.p246);
        let eq142_e1809_d_n19: f64 = (eq142_e1807_d_n19 * p.p246);
        let eq142_e1809_d_n20: f64 = (eq142_e1807_d_n20 * p.p246);
        let eq142_e1809_d_n21: f64 = (eq142_e1807_d_n21 * p.p246);
        let eq142_e1809_d_n22: f64 = (eq142_e1807_d_n22 * p.p246);
        (eq142_e1809, eq142_e1809_d_n0, eq142_e1809_d_n1, eq142_e1809_d_n2, eq142_e1809_d_n3, eq142_e1809_d_n4, eq142_e1809_d_n5, eq142_e1809_d_n6, eq142_e1809_d_n7, eq142_e1809_d_n8, eq142_e1809_d_n9, eq142_e1809_d_n12, eq142_e1809_d_n14, eq142_e1809_d_n15, eq142_e1809_d_n16, eq142_e1809_d_n17, eq142_e1809_d_n18, eq142_e1809_d_n19, eq142_e1809_d_n20, eq142_e1809_d_n21, eq142_e1809_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq142_value: f64 = eq142_e1811;
        let eq142_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq142_node_derivatives: [f64; 20] = [eq142_e1811_d_n0, eq142_e1811_d_n1, eq142_e1811_d_n2, eq142_e1811_d_n3, eq142_e1811_d_n4, eq142_e1811_d_n5, eq142_e1811_d_n6, eq142_e1811_d_n7, eq142_e1811_d_n8, eq142_e1811_d_n9, eq142_e1811_d_n12, eq142_e1811_d_n14, eq142_e1811_d_n15, eq142_e1811_d_n16, eq142_e1811_d_n17, eq142_e1811_d_n18, eq142_e1811_d_n19, eq142_e1811_d_n20, eq142_e1811_d_n21, eq142_e1811_d_n22];
        let eq142_branch_derivative_indices: [usize; 0] = [];
        let eq142_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq142_value),
            &eq142_node_derivative_indices,
            &eq142_node_derivatives,
            &eq142_branch_derivative_indices,
            &eq142_branch_derivatives,
            multiplicity,
        );
        let (eq143_e1823, eq143_e1823_d_n0, eq143_e1823_d_n1, eq143_e1823_d_n2, eq143_e1823_d_n3, eq143_e1823_d_n4, eq143_e1823_d_n5, eq143_e1823_d_n6, eq143_e1823_d_n7, eq143_e1823_d_n8, eq143_e1823_d_n9, eq143_e1823_d_n12, eq143_e1823_d_n14, eq143_e1823_d_n15, eq143_e1823_d_n16, eq143_e1823_d_n17, eq143_e1823_d_n18, eq143_e1823_d_n19, eq143_e1823_d_n20, eq143_e1823_d_n21, eq143_e1823_d_n22,) = {
    if ((var_guard541 == 0.0) && (var_guard544 != 0.0)) {
        let eq143_e1819: f64 = (p.p251 * var_qg_fp1s);
        let eq143_e1819_d_n0: f64 = (p.p251 * var_qg_fp1s_dn0);
        let eq143_e1819_d_n1: f64 = (p.p251 * var_qg_fp1s_dn1);
        let eq143_e1819_d_n2: f64 = (p.p251 * var_qg_fp1s_dn2);
        let eq143_e1819_d_n3: f64 = (p.p251 * var_qg_fp1s_dn3);
        let eq143_e1819_d_n4: f64 = (p.p251 * var_qg_fp1s_dn4);
        let eq143_e1819_d_n5: f64 = (p.p251 * var_qg_fp1s_dn5);
        let eq143_e1819_d_n6: f64 = (p.p251 * var_qg_fp1s_dn6);
        let eq143_e1819_d_n7: f64 = (p.p251 * var_qg_fp1s_dn7);
        let eq143_e1819_d_n8: f64 = (p.p251 * var_qg_fp1s_dn8);
        let eq143_e1819_d_n9: f64 = (p.p251 * var_qg_fp1s_dn9);
        let eq143_e1819_d_n12: f64 = (p.p251 * var_qg_fp1s_dn12);
        let eq143_e1819_d_n14: f64 = (p.p251 * var_qg_fp1s_dn14);
        let eq143_e1819_d_n15: f64 = (p.p251 * var_qg_fp1s_dn15);
        let eq143_e1819_d_n16: f64 = (p.p251 * var_qg_fp1s_dn16);
        let eq143_e1819_d_n17: f64 = (p.p251 * var_qg_fp1s_dn17);
        let eq143_e1819_d_n18: f64 = (p.p251 * var_qg_fp1s_dn18);
        let eq143_e1819_d_n19: f64 = (p.p251 * var_qg_fp1s_dn19);
        let eq143_e1819_d_n20: f64 = (p.p251 * var_qg_fp1s_dn20);
        let eq143_e1819_d_n21: f64 = (p.p251 * var_qg_fp1s_dn21);
        let eq143_e1819_d_n22: f64 = (p.p251 * var_qg_fp1s_dn22);
        let eq143_e1820: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 42, eq143_e1819);
        let eq143_e1821: f64 = (p.p7 * eq143_e1820);
        let eq143_e1821_d_n0: f64 = (p.p7 * (eq143_e1819_d_n0 * ddt_scale));
        let eq143_e1821_d_n1: f64 = (p.p7 * (eq143_e1819_d_n1 * ddt_scale));
        let eq143_e1821_d_n2: f64 = (p.p7 * (eq143_e1819_d_n2 * ddt_scale));
        let eq143_e1821_d_n3: f64 = (p.p7 * (eq143_e1819_d_n3 * ddt_scale));
        let eq143_e1821_d_n4: f64 = (p.p7 * (eq143_e1819_d_n4 * ddt_scale));
        let eq143_e1821_d_n5: f64 = (p.p7 * (eq143_e1819_d_n5 * ddt_scale));
        let eq143_e1821_d_n6: f64 = (p.p7 * (eq143_e1819_d_n6 * ddt_scale));
        let eq143_e1821_d_n7: f64 = (p.p7 * (eq143_e1819_d_n7 * ddt_scale));
        let eq143_e1821_d_n8: f64 = (p.p7 * (eq143_e1819_d_n8 * ddt_scale));
        let eq143_e1821_d_n9: f64 = (p.p7 * (eq143_e1819_d_n9 * ddt_scale));
        let eq143_e1821_d_n12: f64 = (p.p7 * (eq143_e1819_d_n12 * ddt_scale));
        let eq143_e1821_d_n14: f64 = (p.p7 * (eq143_e1819_d_n14 * ddt_scale));
        let eq143_e1821_d_n15: f64 = (p.p7 * (eq143_e1819_d_n15 * ddt_scale));
        let eq143_e1821_d_n16: f64 = (p.p7 * (eq143_e1819_d_n16 * ddt_scale));
        let eq143_e1821_d_n17: f64 = (p.p7 * (eq143_e1819_d_n17 * ddt_scale));
        let eq143_e1821_d_n18: f64 = (p.p7 * (eq143_e1819_d_n18 * ddt_scale));
        let eq143_e1821_d_n19: f64 = (p.p7 * (eq143_e1819_d_n19 * ddt_scale));
        let eq143_e1821_d_n20: f64 = (p.p7 * (eq143_e1819_d_n20 * ddt_scale));
        let eq143_e1821_d_n21: f64 = (p.p7 * (eq143_e1819_d_n21 * ddt_scale));
        let eq143_e1821_d_n22: f64 = (p.p7 * (eq143_e1819_d_n22 * ddt_scale));
        (eq143_e1821, eq143_e1821_d_n0, eq143_e1821_d_n1, eq143_e1821_d_n2, eq143_e1821_d_n3, eq143_e1821_d_n4, eq143_e1821_d_n5, eq143_e1821_d_n6, eq143_e1821_d_n7, eq143_e1821_d_n8, eq143_e1821_d_n9, eq143_e1821_d_n12, eq143_e1821_d_n14, eq143_e1821_d_n15, eq143_e1821_d_n16, eq143_e1821_d_n17, eq143_e1821_d_n18, eq143_e1821_d_n19, eq143_e1821_d_n20, eq143_e1821_d_n21, eq143_e1821_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq143_value: f64 = eq143_e1823;
        let eq143_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq143_node_derivatives: [f64; 20] = [eq143_e1823_d_n0, eq143_e1823_d_n1, eq143_e1823_d_n2, eq143_e1823_d_n3, eq143_e1823_d_n4, eq143_e1823_d_n5, eq143_e1823_d_n6, eq143_e1823_d_n7, eq143_e1823_d_n8, eq143_e1823_d_n9, eq143_e1823_d_n12, eq143_e1823_d_n14, eq143_e1823_d_n15, eq143_e1823_d_n16, eq143_e1823_d_n17, eq143_e1823_d_n18, eq143_e1823_d_n19, eq143_e1823_d_n20, eq143_e1823_d_n21, eq143_e1823_d_n22];
        let eq143_branch_derivative_indices: [usize; 0] = [];
        let eq143_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(8),
            multiplicity * (eq143_value),
            &eq143_node_derivative_indices,
            &eq143_node_derivatives,
            &eq143_branch_derivative_indices,
            &eq143_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_6(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
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
        var_guard546: f64,
        var_guard547: f64,
        var_guard548: f64,
        var_guard549: f64,
        var_guard550: f64,
        var_qd_fp2: f64,
        var_qd_fp2_dn0: f64,
        var_qd_fp2_dn1: f64,
        var_qd_fp2_dn12: f64,
        var_qd_fp2_dn14: f64,
        var_qd_fp2_dn15: f64,
        var_qd_fp2_dn16: f64,
        var_qd_fp2_dn17: f64,
        var_qd_fp2_dn18: f64,
        var_qd_fp2_dn19: f64,
        var_qd_fp2_dn2: f64,
        var_qd_fp2_dn20: f64,
        var_qd_fp2_dn21: f64,
        var_qd_fp2_dn22: f64,
        var_qd_fp2_dn3: f64,
        var_qd_fp2_dn4: f64,
        var_qd_fp2_dn5: f64,
        var_qd_fp2_dn6: f64,
        var_qd_fp2_dn7: f64,
        var_qd_fp2_dn8: f64,
        var_qd_fp2_dn9: f64,
        var_qg_fp2: f64,
        var_qg_fp2_dn0: f64,
        var_qg_fp2_dn1: f64,
        var_qg_fp2_dn12: f64,
        var_qg_fp2_dn14: f64,
        var_qg_fp2_dn15: f64,
        var_qg_fp2_dn16: f64,
        var_qg_fp2_dn17: f64,
        var_qg_fp2_dn18: f64,
        var_qg_fp2_dn19: f64,
        var_qg_fp2_dn2: f64,
        var_qg_fp2_dn20: f64,
        var_qg_fp2_dn21: f64,
        var_qg_fp2_dn22: f64,
        var_qg_fp2_dn3: f64,
        var_qg_fp2_dn4: f64,
        var_qg_fp2_dn5: f64,
        var_qg_fp2_dn6: f64,
        var_qg_fp2_dn7: f64,
        var_qg_fp2_dn8: f64,
        var_qg_fp2_dn9: f64,
    ) {
        let (eq144_e1832, eq144_e1832_d_n0, eq144_e1832_d_n1, eq144_e1832_d_n2, eq144_e1832_d_n3, eq144_e1832_d_n4, eq144_e1832_d_n5, eq144_e1832_d_n6, eq144_e1832_d_n7, eq144_e1832_d_n8, eq144_e1832_d_n9, eq144_e1832_d_n12, eq144_e1832_d_n14, eq144_e1832_d_n15, eq144_e1832_d_n16, eq144_e1832_d_n17, eq144_e1832_d_n18, eq144_e1832_d_n19, eq144_e1832_d_n20, eq144_e1832_d_n21, eq144_e1832_d_n22,) = {
    if ((var_guard546 != 0.0) && (var_guard547 != 0.0)) {
        let eq144_e1829: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 43, var_qd_fp2);
        let eq144_e1830: f64 = (p.p7 * eq144_e1829);
        let eq144_e1830_d_n0: f64 = (p.p7 * (var_qd_fp2_dn0 * ddt_scale));
        let eq144_e1830_d_n1: f64 = (p.p7 * (var_qd_fp2_dn1 * ddt_scale));
        let eq144_e1830_d_n2: f64 = (p.p7 * (var_qd_fp2_dn2 * ddt_scale));
        let eq144_e1830_d_n3: f64 = (p.p7 * (var_qd_fp2_dn3 * ddt_scale));
        let eq144_e1830_d_n4: f64 = (p.p7 * (var_qd_fp2_dn4 * ddt_scale));
        let eq144_e1830_d_n5: f64 = (p.p7 * (var_qd_fp2_dn5 * ddt_scale));
        let eq144_e1830_d_n6: f64 = (p.p7 * (var_qd_fp2_dn6 * ddt_scale));
        let eq144_e1830_d_n7: f64 = (p.p7 * (var_qd_fp2_dn7 * ddt_scale));
        let eq144_e1830_d_n8: f64 = (p.p7 * (var_qd_fp2_dn8 * ddt_scale));
        let eq144_e1830_d_n9: f64 = (p.p7 * (var_qd_fp2_dn9 * ddt_scale));
        let eq144_e1830_d_n12: f64 = (p.p7 * (var_qd_fp2_dn12 * ddt_scale));
        let eq144_e1830_d_n14: f64 = (p.p7 * (var_qd_fp2_dn14 * ddt_scale));
        let eq144_e1830_d_n15: f64 = (p.p7 * (var_qd_fp2_dn15 * ddt_scale));
        let eq144_e1830_d_n16: f64 = (p.p7 * (var_qd_fp2_dn16 * ddt_scale));
        let eq144_e1830_d_n17: f64 = (p.p7 * (var_qd_fp2_dn17 * ddt_scale));
        let eq144_e1830_d_n18: f64 = (p.p7 * (var_qd_fp2_dn18 * ddt_scale));
        let eq144_e1830_d_n19: f64 = (p.p7 * (var_qd_fp2_dn19 * ddt_scale));
        let eq144_e1830_d_n20: f64 = (p.p7 * (var_qd_fp2_dn20 * ddt_scale));
        let eq144_e1830_d_n21: f64 = (p.p7 * (var_qd_fp2_dn21 * ddt_scale));
        let eq144_e1830_d_n22: f64 = (p.p7 * (var_qd_fp2_dn22 * ddt_scale));
        (eq144_e1830, eq144_e1830_d_n0, eq144_e1830_d_n1, eq144_e1830_d_n2, eq144_e1830_d_n3, eq144_e1830_d_n4, eq144_e1830_d_n5, eq144_e1830_d_n6, eq144_e1830_d_n7, eq144_e1830_d_n8, eq144_e1830_d_n9, eq144_e1830_d_n12, eq144_e1830_d_n14, eq144_e1830_d_n15, eq144_e1830_d_n16, eq144_e1830_d_n17, eq144_e1830_d_n18, eq144_e1830_d_n19, eq144_e1830_d_n20, eq144_e1830_d_n21, eq144_e1830_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq144_value: f64 = eq144_e1832;
        let eq144_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq144_node_derivatives: [f64; 20] = [eq144_e1832_d_n0, eq144_e1832_d_n1, eq144_e1832_d_n2, eq144_e1832_d_n3, eq144_e1832_d_n4, eq144_e1832_d_n5, eq144_e1832_d_n6, eq144_e1832_d_n7, eq144_e1832_d_n8, eq144_e1832_d_n9, eq144_e1832_d_n12, eq144_e1832_d_n14, eq144_e1832_d_n15, eq144_e1832_d_n16, eq144_e1832_d_n17, eq144_e1832_d_n18, eq144_e1832_d_n19, eq144_e1832_d_n20, eq144_e1832_d_n21, eq144_e1832_d_n22];
        let eq144_branch_derivative_indices: [usize; 0] = [];
        let eq144_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(16),
            Some(15),
            multiplicity * (eq144_value),
            &eq144_node_derivative_indices,
            &eq144_node_derivatives,
            &eq144_branch_derivative_indices,
            &eq144_branch_derivatives,
            multiplicity,
        );
        let (eq145_e1843, eq145_e1843_d_n0, eq145_e1843_d_n1, eq145_e1843_d_n2, eq145_e1843_d_n3, eq145_e1843_d_n4, eq145_e1843_d_n5, eq145_e1843_d_n6, eq145_e1843_d_n7, eq145_e1843_d_n8, eq145_e1843_d_n9, eq145_e1843_d_n12, eq145_e1843_d_n14, eq145_e1843_d_n15, eq145_e1843_d_n16, eq145_e1843_d_n17, eq145_e1843_d_n18, eq145_e1843_d_n19, eq145_e1843_d_n20, eq145_e1843_d_n21, eq145_e1843_d_n22,) = {
    if (((var_guard546 != 0.0) && (var_guard547 != 0.0)) && (var_guard548 != 0.0)) {
        let eq145_e1840: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 44, var_qg_fp2);
        let eq145_e1841: f64 = (p.p7 * eq145_e1840);
        let eq145_e1841_d_n0: f64 = (p.p7 * (var_qg_fp2_dn0 * ddt_scale));
        let eq145_e1841_d_n1: f64 = (p.p7 * (var_qg_fp2_dn1 * ddt_scale));
        let eq145_e1841_d_n2: f64 = (p.p7 * (var_qg_fp2_dn2 * ddt_scale));
        let eq145_e1841_d_n3: f64 = (p.p7 * (var_qg_fp2_dn3 * ddt_scale));
        let eq145_e1841_d_n4: f64 = (p.p7 * (var_qg_fp2_dn4 * ddt_scale));
        let eq145_e1841_d_n5: f64 = (p.p7 * (var_qg_fp2_dn5 * ddt_scale));
        let eq145_e1841_d_n6: f64 = (p.p7 * (var_qg_fp2_dn6 * ddt_scale));
        let eq145_e1841_d_n7: f64 = (p.p7 * (var_qg_fp2_dn7 * ddt_scale));
        let eq145_e1841_d_n8: f64 = (p.p7 * (var_qg_fp2_dn8 * ddt_scale));
        let eq145_e1841_d_n9: f64 = (p.p7 * (var_qg_fp2_dn9 * ddt_scale));
        let eq145_e1841_d_n12: f64 = (p.p7 * (var_qg_fp2_dn12 * ddt_scale));
        let eq145_e1841_d_n14: f64 = (p.p7 * (var_qg_fp2_dn14 * ddt_scale));
        let eq145_e1841_d_n15: f64 = (p.p7 * (var_qg_fp2_dn15 * ddt_scale));
        let eq145_e1841_d_n16: f64 = (p.p7 * (var_qg_fp2_dn16 * ddt_scale));
        let eq145_e1841_d_n17: f64 = (p.p7 * (var_qg_fp2_dn17 * ddt_scale));
        let eq145_e1841_d_n18: f64 = (p.p7 * (var_qg_fp2_dn18 * ddt_scale));
        let eq145_e1841_d_n19: f64 = (p.p7 * (var_qg_fp2_dn19 * ddt_scale));
        let eq145_e1841_d_n20: f64 = (p.p7 * (var_qg_fp2_dn20 * ddt_scale));
        let eq145_e1841_d_n21: f64 = (p.p7 * (var_qg_fp2_dn21 * ddt_scale));
        let eq145_e1841_d_n22: f64 = (p.p7 * (var_qg_fp2_dn22 * ddt_scale));
        (eq145_e1841, eq145_e1841_d_n0, eq145_e1841_d_n1, eq145_e1841_d_n2, eq145_e1841_d_n3, eq145_e1841_d_n4, eq145_e1841_d_n5, eq145_e1841_d_n6, eq145_e1841_d_n7, eq145_e1841_d_n8, eq145_e1841_d_n9, eq145_e1841_d_n12, eq145_e1841_d_n14, eq145_e1841_d_n15, eq145_e1841_d_n16, eq145_e1841_d_n17, eq145_e1841_d_n18, eq145_e1841_d_n19, eq145_e1841_d_n20, eq145_e1841_d_n21, eq145_e1841_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq145_value: f64 = eq145_e1843;
        let eq145_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq145_node_derivatives: [f64; 20] = [eq145_e1843_d_n0, eq145_e1843_d_n1, eq145_e1843_d_n2, eq145_e1843_d_n3, eq145_e1843_d_n4, eq145_e1843_d_n5, eq145_e1843_d_n6, eq145_e1843_d_n7, eq145_e1843_d_n8, eq145_e1843_d_n9, eq145_e1843_d_n12, eq145_e1843_d_n14, eq145_e1843_d_n15, eq145_e1843_d_n16, eq145_e1843_d_n17, eq145_e1843_d_n18, eq145_e1843_d_n19, eq145_e1843_d_n20, eq145_e1843_d_n21, eq145_e1843_d_n22];
        let eq145_branch_derivative_indices: [usize; 0] = [];
        let eq145_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(15),
            multiplicity * (eq145_value),
            &eq145_node_derivative_indices,
            &eq145_node_derivatives,
            &eq145_branch_derivative_indices,
            &eq145_branch_derivatives,
            multiplicity,
        );
        let (eq146_e1856, eq146_e1856_d_n0, eq146_e1856_d_n1, eq146_e1856_d_n2, eq146_e1856_d_n3, eq146_e1856_d_n4, eq146_e1856_d_n5, eq146_e1856_d_n6, eq146_e1856_d_n7, eq146_e1856_d_n8, eq146_e1856_d_n9, eq146_e1856_d_n12, eq146_e1856_d_n14, eq146_e1856_d_n15, eq146_e1856_d_n16, eq146_e1856_d_n17, eq146_e1856_d_n18, eq146_e1856_d_n19, eq146_e1856_d_n20, eq146_e1856_d_n21, eq146_e1856_d_n22,) = {
    if (((var_guard546 != 0.0) && (var_guard547 != 0.0)) && (var_guard548 != 0.0)) {
        let eq146_e1851: f64 = (p.p7 * p.p247);
        let eq146_e1853: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 45, var_qg_fp2);
        let eq146_e1854: f64 = (eq146_e1851 * eq146_e1853);
        let eq146_e1854_d_n0: f64 = (eq146_e1851 * (var_qg_fp2_dn0 * ddt_scale));
        let eq146_e1854_d_n1: f64 = (eq146_e1851 * (var_qg_fp2_dn1 * ddt_scale));
        let eq146_e1854_d_n2: f64 = (eq146_e1851 * (var_qg_fp2_dn2 * ddt_scale));
        let eq146_e1854_d_n3: f64 = (eq146_e1851 * (var_qg_fp2_dn3 * ddt_scale));
        let eq146_e1854_d_n4: f64 = (eq146_e1851 * (var_qg_fp2_dn4 * ddt_scale));
        let eq146_e1854_d_n5: f64 = (eq146_e1851 * (var_qg_fp2_dn5 * ddt_scale));
        let eq146_e1854_d_n6: f64 = (eq146_e1851 * (var_qg_fp2_dn6 * ddt_scale));
        let eq146_e1854_d_n7: f64 = (eq146_e1851 * (var_qg_fp2_dn7 * ddt_scale));
        let eq146_e1854_d_n8: f64 = (eq146_e1851 * (var_qg_fp2_dn8 * ddt_scale));
        let eq146_e1854_d_n9: f64 = (eq146_e1851 * (var_qg_fp2_dn9 * ddt_scale));
        let eq146_e1854_d_n12: f64 = (eq146_e1851 * (var_qg_fp2_dn12 * ddt_scale));
        let eq146_e1854_d_n14: f64 = (eq146_e1851 * (var_qg_fp2_dn14 * ddt_scale));
        let eq146_e1854_d_n15: f64 = (eq146_e1851 * (var_qg_fp2_dn15 * ddt_scale));
        let eq146_e1854_d_n16: f64 = (eq146_e1851 * (var_qg_fp2_dn16 * ddt_scale));
        let eq146_e1854_d_n17: f64 = (eq146_e1851 * (var_qg_fp2_dn17 * ddt_scale));
        let eq146_e1854_d_n18: f64 = (eq146_e1851 * (var_qg_fp2_dn18 * ddt_scale));
        let eq146_e1854_d_n19: f64 = (eq146_e1851 * (var_qg_fp2_dn19 * ddt_scale));
        let eq146_e1854_d_n20: f64 = (eq146_e1851 * (var_qg_fp2_dn20 * ddt_scale));
        let eq146_e1854_d_n21: f64 = (eq146_e1851 * (var_qg_fp2_dn21 * ddt_scale));
        let eq146_e1854_d_n22: f64 = (eq146_e1851 * (var_qg_fp2_dn22 * ddt_scale));
        (eq146_e1854, eq146_e1854_d_n0, eq146_e1854_d_n1, eq146_e1854_d_n2, eq146_e1854_d_n3, eq146_e1854_d_n4, eq146_e1854_d_n5, eq146_e1854_d_n6, eq146_e1854_d_n7, eq146_e1854_d_n8, eq146_e1854_d_n9, eq146_e1854_d_n12, eq146_e1854_d_n14, eq146_e1854_d_n15, eq146_e1854_d_n16, eq146_e1854_d_n17, eq146_e1854_d_n18, eq146_e1854_d_n19, eq146_e1854_d_n20, eq146_e1854_d_n21, eq146_e1854_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq146_value: f64 = eq146_e1856;
        let eq146_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq146_node_derivatives: [f64; 20] = [eq146_e1856_d_n0, eq146_e1856_d_n1, eq146_e1856_d_n2, eq146_e1856_d_n3, eq146_e1856_d_n4, eq146_e1856_d_n5, eq146_e1856_d_n6, eq146_e1856_d_n7, eq146_e1856_d_n8, eq146_e1856_d_n9, eq146_e1856_d_n12, eq146_e1856_d_n14, eq146_e1856_d_n15, eq146_e1856_d_n16, eq146_e1856_d_n17, eq146_e1856_d_n18, eq146_e1856_d_n19, eq146_e1856_d_n20, eq146_e1856_d_n21, eq146_e1856_d_n22];
        let eq146_branch_derivative_indices: [usize; 0] = [];
        let eq146_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(15),
            multiplicity * (eq146_value),
            &eq146_node_derivative_indices,
            &eq146_node_derivatives,
            &eq146_branch_derivative_indices,
            &eq146_branch_derivatives,
            multiplicity,
        );
        let (eq147_e1868, eq147_e1868_d_n0, eq147_e1868_d_n1, eq147_e1868_d_n2, eq147_e1868_d_n3, eq147_e1868_d_n4, eq147_e1868_d_n5, eq147_e1868_d_n6, eq147_e1868_d_n7, eq147_e1868_d_n8, eq147_e1868_d_n9, eq147_e1868_d_n12, eq147_e1868_d_n14, eq147_e1868_d_n15, eq147_e1868_d_n16, eq147_e1868_d_n17, eq147_e1868_d_n18, eq147_e1868_d_n19, eq147_e1868_d_n20, eq147_e1868_d_n21, eq147_e1868_d_n22,) = {
    if (((var_guard546 != 0.0) && (var_guard547 != 0.0)) && (var_guard548 == 0.0)) {
        let eq147_e1865: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 46, var_qg_fp2);
        let eq147_e1866: f64 = (p.p7 * eq147_e1865);
        let eq147_e1866_d_n0: f64 = (p.p7 * (var_qg_fp2_dn0 * ddt_scale));
        let eq147_e1866_d_n1: f64 = (p.p7 * (var_qg_fp2_dn1 * ddt_scale));
        let eq147_e1866_d_n2: f64 = (p.p7 * (var_qg_fp2_dn2 * ddt_scale));
        let eq147_e1866_d_n3: f64 = (p.p7 * (var_qg_fp2_dn3 * ddt_scale));
        let eq147_e1866_d_n4: f64 = (p.p7 * (var_qg_fp2_dn4 * ddt_scale));
        let eq147_e1866_d_n5: f64 = (p.p7 * (var_qg_fp2_dn5 * ddt_scale));
        let eq147_e1866_d_n6: f64 = (p.p7 * (var_qg_fp2_dn6 * ddt_scale));
        let eq147_e1866_d_n7: f64 = (p.p7 * (var_qg_fp2_dn7 * ddt_scale));
        let eq147_e1866_d_n8: f64 = (p.p7 * (var_qg_fp2_dn8 * ddt_scale));
        let eq147_e1866_d_n9: f64 = (p.p7 * (var_qg_fp2_dn9 * ddt_scale));
        let eq147_e1866_d_n12: f64 = (p.p7 * (var_qg_fp2_dn12 * ddt_scale));
        let eq147_e1866_d_n14: f64 = (p.p7 * (var_qg_fp2_dn14 * ddt_scale));
        let eq147_e1866_d_n15: f64 = (p.p7 * (var_qg_fp2_dn15 * ddt_scale));
        let eq147_e1866_d_n16: f64 = (p.p7 * (var_qg_fp2_dn16 * ddt_scale));
        let eq147_e1866_d_n17: f64 = (p.p7 * (var_qg_fp2_dn17 * ddt_scale));
        let eq147_e1866_d_n18: f64 = (p.p7 * (var_qg_fp2_dn18 * ddt_scale));
        let eq147_e1866_d_n19: f64 = (p.p7 * (var_qg_fp2_dn19 * ddt_scale));
        let eq147_e1866_d_n20: f64 = (p.p7 * (var_qg_fp2_dn20 * ddt_scale));
        let eq147_e1866_d_n21: f64 = (p.p7 * (var_qg_fp2_dn21 * ddt_scale));
        let eq147_e1866_d_n22: f64 = (p.p7 * (var_qg_fp2_dn22 * ddt_scale));
        (eq147_e1866, eq147_e1866_d_n0, eq147_e1866_d_n1, eq147_e1866_d_n2, eq147_e1866_d_n3, eq147_e1866_d_n4, eq147_e1866_d_n5, eq147_e1866_d_n6, eq147_e1866_d_n7, eq147_e1866_d_n8, eq147_e1866_d_n9, eq147_e1866_d_n12, eq147_e1866_d_n14, eq147_e1866_d_n15, eq147_e1866_d_n16, eq147_e1866_d_n17, eq147_e1866_d_n18, eq147_e1866_d_n19, eq147_e1866_d_n20, eq147_e1866_d_n21, eq147_e1866_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq147_value: f64 = eq147_e1868;
        let eq147_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq147_node_derivatives: [f64; 20] = [eq147_e1868_d_n0, eq147_e1868_d_n1, eq147_e1868_d_n2, eq147_e1868_d_n3, eq147_e1868_d_n4, eq147_e1868_d_n5, eq147_e1868_d_n6, eq147_e1868_d_n7, eq147_e1868_d_n8, eq147_e1868_d_n9, eq147_e1868_d_n12, eq147_e1868_d_n14, eq147_e1868_d_n15, eq147_e1868_d_n16, eq147_e1868_d_n17, eq147_e1868_d_n18, eq147_e1868_d_n19, eq147_e1868_d_n20, eq147_e1868_d_n21, eq147_e1868_d_n22];
        let eq147_branch_derivative_indices: [usize; 0] = [];
        let eq147_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(15),
            multiplicity * (eq147_value),
            &eq147_node_derivative_indices,
            &eq147_node_derivatives,
            &eq147_branch_derivative_indices,
            &eq147_branch_derivatives,
            multiplicity,
        );
        let (eq148_e1882, eq148_e1882_d_n0, eq148_e1882_d_n1, eq148_e1882_d_n2, eq148_e1882_d_n3, eq148_e1882_d_n4, eq148_e1882_d_n5, eq148_e1882_d_n6, eq148_e1882_d_n7, eq148_e1882_d_n8, eq148_e1882_d_n9, eq148_e1882_d_n12, eq148_e1882_d_n14, eq148_e1882_d_n15, eq148_e1882_d_n16, eq148_e1882_d_n17, eq148_e1882_d_n18, eq148_e1882_d_n19, eq148_e1882_d_n20, eq148_e1882_d_n21, eq148_e1882_d_n22,) = {
    if (((var_guard546 != 0.0) && (var_guard547 != 0.0)) && (var_guard548 == 0.0)) {
        let eq148_e1877: f64 = (p.p7 * p.p247);
        let eq148_e1879: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 47, var_qg_fp2);
        let eq148_e1880: f64 = (eq148_e1877 * eq148_e1879);
        let eq148_e1880_d_n0: f64 = (eq148_e1877 * (var_qg_fp2_dn0 * ddt_scale));
        let eq148_e1880_d_n1: f64 = (eq148_e1877 * (var_qg_fp2_dn1 * ddt_scale));
        let eq148_e1880_d_n2: f64 = (eq148_e1877 * (var_qg_fp2_dn2 * ddt_scale));
        let eq148_e1880_d_n3: f64 = (eq148_e1877 * (var_qg_fp2_dn3 * ddt_scale));
        let eq148_e1880_d_n4: f64 = (eq148_e1877 * (var_qg_fp2_dn4 * ddt_scale));
        let eq148_e1880_d_n5: f64 = (eq148_e1877 * (var_qg_fp2_dn5 * ddt_scale));
        let eq148_e1880_d_n6: f64 = (eq148_e1877 * (var_qg_fp2_dn6 * ddt_scale));
        let eq148_e1880_d_n7: f64 = (eq148_e1877 * (var_qg_fp2_dn7 * ddt_scale));
        let eq148_e1880_d_n8: f64 = (eq148_e1877 * (var_qg_fp2_dn8 * ddt_scale));
        let eq148_e1880_d_n9: f64 = (eq148_e1877 * (var_qg_fp2_dn9 * ddt_scale));
        let eq148_e1880_d_n12: f64 = (eq148_e1877 * (var_qg_fp2_dn12 * ddt_scale));
        let eq148_e1880_d_n14: f64 = (eq148_e1877 * (var_qg_fp2_dn14 * ddt_scale));
        let eq148_e1880_d_n15: f64 = (eq148_e1877 * (var_qg_fp2_dn15 * ddt_scale));
        let eq148_e1880_d_n16: f64 = (eq148_e1877 * (var_qg_fp2_dn16 * ddt_scale));
        let eq148_e1880_d_n17: f64 = (eq148_e1877 * (var_qg_fp2_dn17 * ddt_scale));
        let eq148_e1880_d_n18: f64 = (eq148_e1877 * (var_qg_fp2_dn18 * ddt_scale));
        let eq148_e1880_d_n19: f64 = (eq148_e1877 * (var_qg_fp2_dn19 * ddt_scale));
        let eq148_e1880_d_n20: f64 = (eq148_e1877 * (var_qg_fp2_dn20 * ddt_scale));
        let eq148_e1880_d_n21: f64 = (eq148_e1877 * (var_qg_fp2_dn21 * ddt_scale));
        let eq148_e1880_d_n22: f64 = (eq148_e1877 * (var_qg_fp2_dn22 * ddt_scale));
        (eq148_e1880, eq148_e1880_d_n0, eq148_e1880_d_n1, eq148_e1880_d_n2, eq148_e1880_d_n3, eq148_e1880_d_n4, eq148_e1880_d_n5, eq148_e1880_d_n6, eq148_e1880_d_n7, eq148_e1880_d_n8, eq148_e1880_d_n9, eq148_e1880_d_n12, eq148_e1880_d_n14, eq148_e1880_d_n15, eq148_e1880_d_n16, eq148_e1880_d_n17, eq148_e1880_d_n18, eq148_e1880_d_n19, eq148_e1880_d_n20, eq148_e1880_d_n21, eq148_e1880_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq148_value: f64 = eq148_e1882;
        let eq148_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq148_node_derivatives: [f64; 20] = [eq148_e1882_d_n0, eq148_e1882_d_n1, eq148_e1882_d_n2, eq148_e1882_d_n3, eq148_e1882_d_n4, eq148_e1882_d_n5, eq148_e1882_d_n6, eq148_e1882_d_n7, eq148_e1882_d_n8, eq148_e1882_d_n9, eq148_e1882_d_n12, eq148_e1882_d_n14, eq148_e1882_d_n15, eq148_e1882_d_n16, eq148_e1882_d_n17, eq148_e1882_d_n18, eq148_e1882_d_n19, eq148_e1882_d_n20, eq148_e1882_d_n21, eq148_e1882_d_n22];
        let eq148_branch_derivative_indices: [usize; 0] = [];
        let eq148_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(15),
            multiplicity * (eq148_value),
            &eq148_node_derivative_indices,
            &eq148_node_derivatives,
            &eq148_branch_derivative_indices,
            &eq148_branch_derivatives,
            multiplicity,
        );
        let (eq149_e1893, eq149_e1893_d_n0, eq149_e1893_d_n1, eq149_e1893_d_n2, eq149_e1893_d_n3, eq149_e1893_d_n4, eq149_e1893_d_n5, eq149_e1893_d_n6, eq149_e1893_d_n7, eq149_e1893_d_n8, eq149_e1893_d_n9, eq149_e1893_d_n12, eq149_e1893_d_n14, eq149_e1893_d_n15, eq149_e1893_d_n16, eq149_e1893_d_n17, eq149_e1893_d_n18, eq149_e1893_d_n19, eq149_e1893_d_n20, eq149_e1893_d_n21, eq149_e1893_d_n22,) = {
    if ((var_guard546 != 0.0) && (var_guard547 != 0.0)) {
        let eq149_e1889: f64 = (p.p252 * var_qg_fp2);
        let eq149_e1889_d_n0: f64 = (p.p252 * var_qg_fp2_dn0);
        let eq149_e1889_d_n1: f64 = (p.p252 * var_qg_fp2_dn1);
        let eq149_e1889_d_n2: f64 = (p.p252 * var_qg_fp2_dn2);
        let eq149_e1889_d_n3: f64 = (p.p252 * var_qg_fp2_dn3);
        let eq149_e1889_d_n4: f64 = (p.p252 * var_qg_fp2_dn4);
        let eq149_e1889_d_n5: f64 = (p.p252 * var_qg_fp2_dn5);
        let eq149_e1889_d_n6: f64 = (p.p252 * var_qg_fp2_dn6);
        let eq149_e1889_d_n7: f64 = (p.p252 * var_qg_fp2_dn7);
        let eq149_e1889_d_n8: f64 = (p.p252 * var_qg_fp2_dn8);
        let eq149_e1889_d_n9: f64 = (p.p252 * var_qg_fp2_dn9);
        let eq149_e1889_d_n12: f64 = (p.p252 * var_qg_fp2_dn12);
        let eq149_e1889_d_n14: f64 = (p.p252 * var_qg_fp2_dn14);
        let eq149_e1889_d_n15: f64 = (p.p252 * var_qg_fp2_dn15);
        let eq149_e1889_d_n16: f64 = (p.p252 * var_qg_fp2_dn16);
        let eq149_e1889_d_n17: f64 = (p.p252 * var_qg_fp2_dn17);
        let eq149_e1889_d_n18: f64 = (p.p252 * var_qg_fp2_dn18);
        let eq149_e1889_d_n19: f64 = (p.p252 * var_qg_fp2_dn19);
        let eq149_e1889_d_n20: f64 = (p.p252 * var_qg_fp2_dn20);
        let eq149_e1889_d_n21: f64 = (p.p252 * var_qg_fp2_dn21);
        let eq149_e1889_d_n22: f64 = (p.p252 * var_qg_fp2_dn22);
        let eq149_e1890: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 48, eq149_e1889);
        let eq149_e1891: f64 = (p.p7 * eq149_e1890);
        let eq149_e1891_d_n0: f64 = (p.p7 * (eq149_e1889_d_n0 * ddt_scale));
        let eq149_e1891_d_n1: f64 = (p.p7 * (eq149_e1889_d_n1 * ddt_scale));
        let eq149_e1891_d_n2: f64 = (p.p7 * (eq149_e1889_d_n2 * ddt_scale));
        let eq149_e1891_d_n3: f64 = (p.p7 * (eq149_e1889_d_n3 * ddt_scale));
        let eq149_e1891_d_n4: f64 = (p.p7 * (eq149_e1889_d_n4 * ddt_scale));
        let eq149_e1891_d_n5: f64 = (p.p7 * (eq149_e1889_d_n5 * ddt_scale));
        let eq149_e1891_d_n6: f64 = (p.p7 * (eq149_e1889_d_n6 * ddt_scale));
        let eq149_e1891_d_n7: f64 = (p.p7 * (eq149_e1889_d_n7 * ddt_scale));
        let eq149_e1891_d_n8: f64 = (p.p7 * (eq149_e1889_d_n8 * ddt_scale));
        let eq149_e1891_d_n9: f64 = (p.p7 * (eq149_e1889_d_n9 * ddt_scale));
        let eq149_e1891_d_n12: f64 = (p.p7 * (eq149_e1889_d_n12 * ddt_scale));
        let eq149_e1891_d_n14: f64 = (p.p7 * (eq149_e1889_d_n14 * ddt_scale));
        let eq149_e1891_d_n15: f64 = (p.p7 * (eq149_e1889_d_n15 * ddt_scale));
        let eq149_e1891_d_n16: f64 = (p.p7 * (eq149_e1889_d_n16 * ddt_scale));
        let eq149_e1891_d_n17: f64 = (p.p7 * (eq149_e1889_d_n17 * ddt_scale));
        let eq149_e1891_d_n18: f64 = (p.p7 * (eq149_e1889_d_n18 * ddt_scale));
        let eq149_e1891_d_n19: f64 = (p.p7 * (eq149_e1889_d_n19 * ddt_scale));
        let eq149_e1891_d_n20: f64 = (p.p7 * (eq149_e1889_d_n20 * ddt_scale));
        let eq149_e1891_d_n21: f64 = (p.p7 * (eq149_e1889_d_n21 * ddt_scale));
        let eq149_e1891_d_n22: f64 = (p.p7 * (eq149_e1889_d_n22 * ddt_scale));
        (eq149_e1891, eq149_e1891_d_n0, eq149_e1891_d_n1, eq149_e1891_d_n2, eq149_e1891_d_n3, eq149_e1891_d_n4, eq149_e1891_d_n5, eq149_e1891_d_n6, eq149_e1891_d_n7, eq149_e1891_d_n8, eq149_e1891_d_n9, eq149_e1891_d_n12, eq149_e1891_d_n14, eq149_e1891_d_n15, eq149_e1891_d_n16, eq149_e1891_d_n17, eq149_e1891_d_n18, eq149_e1891_d_n19, eq149_e1891_d_n20, eq149_e1891_d_n21, eq149_e1891_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq149_value: f64 = eq149_e1893;
        let eq149_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq149_node_derivatives: [f64; 20] = [eq149_e1893_d_n0, eq149_e1893_d_n1, eq149_e1893_d_n2, eq149_e1893_d_n3, eq149_e1893_d_n4, eq149_e1893_d_n5, eq149_e1893_d_n6, eq149_e1893_d_n7, eq149_e1893_d_n8, eq149_e1893_d_n9, eq149_e1893_d_n12, eq149_e1893_d_n14, eq149_e1893_d_n15, eq149_e1893_d_n16, eq149_e1893_d_n17, eq149_e1893_d_n18, eq149_e1893_d_n19, eq149_e1893_d_n20, eq149_e1893_d_n21, eq149_e1893_d_n22];
        let eq149_branch_derivative_indices: [usize; 0] = [];
        let eq149_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(15),
            multiplicity * (eq149_value),
            &eq149_node_derivative_indices,
            &eq149_node_derivatives,
            &eq149_branch_derivative_indices,
            &eq149_branch_derivatives,
            multiplicity,
        );
        let (eq150_e1903, eq150_e1903_d_n0, eq150_e1903_d_n1, eq150_e1903_d_n2, eq150_e1903_d_n3, eq150_e1903_d_n4, eq150_e1903_d_n5, eq150_e1903_d_n6, eq150_e1903_d_n7, eq150_e1903_d_n8, eq150_e1903_d_n9, eq150_e1903_d_n12, eq150_e1903_d_n14, eq150_e1903_d_n15, eq150_e1903_d_n16, eq150_e1903_d_n17, eq150_e1903_d_n18, eq150_e1903_d_n19, eq150_e1903_d_n20, eq150_e1903_d_n21, eq150_e1903_d_n22,) = {
    if ((var_guard546 == 0.0) && (var_guard549 != 0.0)) {
        let eq150_e1900: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 49, var_qd_fp2);
        let eq150_e1901: f64 = (p.p7 * eq150_e1900);
        let eq150_e1901_d_n0: f64 = (p.p7 * (var_qd_fp2_dn0 * ddt_scale));
        let eq150_e1901_d_n1: f64 = (p.p7 * (var_qd_fp2_dn1 * ddt_scale));
        let eq150_e1901_d_n2: f64 = (p.p7 * (var_qd_fp2_dn2 * ddt_scale));
        let eq150_e1901_d_n3: f64 = (p.p7 * (var_qd_fp2_dn3 * ddt_scale));
        let eq150_e1901_d_n4: f64 = (p.p7 * (var_qd_fp2_dn4 * ddt_scale));
        let eq150_e1901_d_n5: f64 = (p.p7 * (var_qd_fp2_dn5 * ddt_scale));
        let eq150_e1901_d_n6: f64 = (p.p7 * (var_qd_fp2_dn6 * ddt_scale));
        let eq150_e1901_d_n7: f64 = (p.p7 * (var_qd_fp2_dn7 * ddt_scale));
        let eq150_e1901_d_n8: f64 = (p.p7 * (var_qd_fp2_dn8 * ddt_scale));
        let eq150_e1901_d_n9: f64 = (p.p7 * (var_qd_fp2_dn9 * ddt_scale));
        let eq150_e1901_d_n12: f64 = (p.p7 * (var_qd_fp2_dn12 * ddt_scale));
        let eq150_e1901_d_n14: f64 = (p.p7 * (var_qd_fp2_dn14 * ddt_scale));
        let eq150_e1901_d_n15: f64 = (p.p7 * (var_qd_fp2_dn15 * ddt_scale));
        let eq150_e1901_d_n16: f64 = (p.p7 * (var_qd_fp2_dn16 * ddt_scale));
        let eq150_e1901_d_n17: f64 = (p.p7 * (var_qd_fp2_dn17 * ddt_scale));
        let eq150_e1901_d_n18: f64 = (p.p7 * (var_qd_fp2_dn18 * ddt_scale));
        let eq150_e1901_d_n19: f64 = (p.p7 * (var_qd_fp2_dn19 * ddt_scale));
        let eq150_e1901_d_n20: f64 = (p.p7 * (var_qd_fp2_dn20 * ddt_scale));
        let eq150_e1901_d_n21: f64 = (p.p7 * (var_qd_fp2_dn21 * ddt_scale));
        let eq150_e1901_d_n22: f64 = (p.p7 * (var_qd_fp2_dn22 * ddt_scale));
        (eq150_e1901, eq150_e1901_d_n0, eq150_e1901_d_n1, eq150_e1901_d_n2, eq150_e1901_d_n3, eq150_e1901_d_n4, eq150_e1901_d_n5, eq150_e1901_d_n6, eq150_e1901_d_n7, eq150_e1901_d_n8, eq150_e1901_d_n9, eq150_e1901_d_n12, eq150_e1901_d_n14, eq150_e1901_d_n15, eq150_e1901_d_n16, eq150_e1901_d_n17, eq150_e1901_d_n18, eq150_e1901_d_n19, eq150_e1901_d_n20, eq150_e1901_d_n21, eq150_e1901_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq150_value: f64 = eq150_e1903;
        let eq150_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq150_node_derivatives: [f64; 20] = [eq150_e1903_d_n0, eq150_e1903_d_n1, eq150_e1903_d_n2, eq150_e1903_d_n3, eq150_e1903_d_n4, eq150_e1903_d_n5, eq150_e1903_d_n6, eq150_e1903_d_n7, eq150_e1903_d_n8, eq150_e1903_d_n9, eq150_e1903_d_n12, eq150_e1903_d_n14, eq150_e1903_d_n15, eq150_e1903_d_n16, eq150_e1903_d_n17, eq150_e1903_d_n18, eq150_e1903_d_n19, eq150_e1903_d_n20, eq150_e1903_d_n21, eq150_e1903_d_n22];
        let eq150_branch_derivative_indices: [usize; 0] = [];
        let eq150_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq150_value),
            &eq150_node_derivative_indices,
            &eq150_node_derivatives,
            &eq150_branch_derivative_indices,
            &eq150_branch_derivatives,
            multiplicity,
        );
        let (eq151_e1915, eq151_e1915_d_n0, eq151_e1915_d_n1, eq151_e1915_d_n2, eq151_e1915_d_n3, eq151_e1915_d_n4, eq151_e1915_d_n5, eq151_e1915_d_n6, eq151_e1915_d_n7, eq151_e1915_d_n8, eq151_e1915_d_n9, eq151_e1915_d_n12, eq151_e1915_d_n14, eq151_e1915_d_n15, eq151_e1915_d_n16, eq151_e1915_d_n17, eq151_e1915_d_n18, eq151_e1915_d_n19, eq151_e1915_d_n20, eq151_e1915_d_n21, eq151_e1915_d_n22,) = {
    if (((var_guard546 == 0.0) && (var_guard549 != 0.0)) && (var_guard550 != 0.0)) {
        let eq151_e1912: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 50, var_qg_fp2);
        let eq151_e1913: f64 = (p.p7 * eq151_e1912);
        let eq151_e1913_d_n0: f64 = (p.p7 * (var_qg_fp2_dn0 * ddt_scale));
        let eq151_e1913_d_n1: f64 = (p.p7 * (var_qg_fp2_dn1 * ddt_scale));
        let eq151_e1913_d_n2: f64 = (p.p7 * (var_qg_fp2_dn2 * ddt_scale));
        let eq151_e1913_d_n3: f64 = (p.p7 * (var_qg_fp2_dn3 * ddt_scale));
        let eq151_e1913_d_n4: f64 = (p.p7 * (var_qg_fp2_dn4 * ddt_scale));
        let eq151_e1913_d_n5: f64 = (p.p7 * (var_qg_fp2_dn5 * ddt_scale));
        let eq151_e1913_d_n6: f64 = (p.p7 * (var_qg_fp2_dn6 * ddt_scale));
        let eq151_e1913_d_n7: f64 = (p.p7 * (var_qg_fp2_dn7 * ddt_scale));
        let eq151_e1913_d_n8: f64 = (p.p7 * (var_qg_fp2_dn8 * ddt_scale));
        let eq151_e1913_d_n9: f64 = (p.p7 * (var_qg_fp2_dn9 * ddt_scale));
        let eq151_e1913_d_n12: f64 = (p.p7 * (var_qg_fp2_dn12 * ddt_scale));
        let eq151_e1913_d_n14: f64 = (p.p7 * (var_qg_fp2_dn14 * ddt_scale));
        let eq151_e1913_d_n15: f64 = (p.p7 * (var_qg_fp2_dn15 * ddt_scale));
        let eq151_e1913_d_n16: f64 = (p.p7 * (var_qg_fp2_dn16 * ddt_scale));
        let eq151_e1913_d_n17: f64 = (p.p7 * (var_qg_fp2_dn17 * ddt_scale));
        let eq151_e1913_d_n18: f64 = (p.p7 * (var_qg_fp2_dn18 * ddt_scale));
        let eq151_e1913_d_n19: f64 = (p.p7 * (var_qg_fp2_dn19 * ddt_scale));
        let eq151_e1913_d_n20: f64 = (p.p7 * (var_qg_fp2_dn20 * ddt_scale));
        let eq151_e1913_d_n21: f64 = (p.p7 * (var_qg_fp2_dn21 * ddt_scale));
        let eq151_e1913_d_n22: f64 = (p.p7 * (var_qg_fp2_dn22 * ddt_scale));
        (eq151_e1913, eq151_e1913_d_n0, eq151_e1913_d_n1, eq151_e1913_d_n2, eq151_e1913_d_n3, eq151_e1913_d_n4, eq151_e1913_d_n5, eq151_e1913_d_n6, eq151_e1913_d_n7, eq151_e1913_d_n8, eq151_e1913_d_n9, eq151_e1913_d_n12, eq151_e1913_d_n14, eq151_e1913_d_n15, eq151_e1913_d_n16, eq151_e1913_d_n17, eq151_e1913_d_n18, eq151_e1913_d_n19, eq151_e1913_d_n20, eq151_e1913_d_n21, eq151_e1913_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq151_value: f64 = eq151_e1915;
        let eq151_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq151_node_derivatives: [f64; 20] = [eq151_e1915_d_n0, eq151_e1915_d_n1, eq151_e1915_d_n2, eq151_e1915_d_n3, eq151_e1915_d_n4, eq151_e1915_d_n5, eq151_e1915_d_n6, eq151_e1915_d_n7, eq151_e1915_d_n8, eq151_e1915_d_n9, eq151_e1915_d_n12, eq151_e1915_d_n14, eq151_e1915_d_n15, eq151_e1915_d_n16, eq151_e1915_d_n17, eq151_e1915_d_n18, eq151_e1915_d_n19, eq151_e1915_d_n20, eq151_e1915_d_n21, eq151_e1915_d_n22];
        let eq151_branch_derivative_indices: [usize; 0] = [];
        let eq151_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq151_value),
            &eq151_node_derivative_indices,
            &eq151_node_derivatives,
            &eq151_branch_derivative_indices,
            &eq151_branch_derivatives,
            multiplicity,
        );
        let (eq152_e1929, eq152_e1929_d_n0, eq152_e1929_d_n1, eq152_e1929_d_n2, eq152_e1929_d_n3, eq152_e1929_d_n4, eq152_e1929_d_n5, eq152_e1929_d_n6, eq152_e1929_d_n7, eq152_e1929_d_n8, eq152_e1929_d_n9, eq152_e1929_d_n12, eq152_e1929_d_n14, eq152_e1929_d_n15, eq152_e1929_d_n16, eq152_e1929_d_n17, eq152_e1929_d_n18, eq152_e1929_d_n19, eq152_e1929_d_n20, eq152_e1929_d_n21, eq152_e1929_d_n22,) = {
    if (((var_guard546 == 0.0) && (var_guard549 != 0.0)) && (var_guard550 != 0.0)) {
        let eq152_e1924: f64 = (p.p7 * p.p247);
        let eq152_e1926: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 51, var_qg_fp2);
        let eq152_e1927: f64 = (eq152_e1924 * eq152_e1926);
        let eq152_e1927_d_n0: f64 = (eq152_e1924 * (var_qg_fp2_dn0 * ddt_scale));
        let eq152_e1927_d_n1: f64 = (eq152_e1924 * (var_qg_fp2_dn1 * ddt_scale));
        let eq152_e1927_d_n2: f64 = (eq152_e1924 * (var_qg_fp2_dn2 * ddt_scale));
        let eq152_e1927_d_n3: f64 = (eq152_e1924 * (var_qg_fp2_dn3 * ddt_scale));
        let eq152_e1927_d_n4: f64 = (eq152_e1924 * (var_qg_fp2_dn4 * ddt_scale));
        let eq152_e1927_d_n5: f64 = (eq152_e1924 * (var_qg_fp2_dn5 * ddt_scale));
        let eq152_e1927_d_n6: f64 = (eq152_e1924 * (var_qg_fp2_dn6 * ddt_scale));
        let eq152_e1927_d_n7: f64 = (eq152_e1924 * (var_qg_fp2_dn7 * ddt_scale));
        let eq152_e1927_d_n8: f64 = (eq152_e1924 * (var_qg_fp2_dn8 * ddt_scale));
        let eq152_e1927_d_n9: f64 = (eq152_e1924 * (var_qg_fp2_dn9 * ddt_scale));
        let eq152_e1927_d_n12: f64 = (eq152_e1924 * (var_qg_fp2_dn12 * ddt_scale));
        let eq152_e1927_d_n14: f64 = (eq152_e1924 * (var_qg_fp2_dn14 * ddt_scale));
        let eq152_e1927_d_n15: f64 = (eq152_e1924 * (var_qg_fp2_dn15 * ddt_scale));
        let eq152_e1927_d_n16: f64 = (eq152_e1924 * (var_qg_fp2_dn16 * ddt_scale));
        let eq152_e1927_d_n17: f64 = (eq152_e1924 * (var_qg_fp2_dn17 * ddt_scale));
        let eq152_e1927_d_n18: f64 = (eq152_e1924 * (var_qg_fp2_dn18 * ddt_scale));
        let eq152_e1927_d_n19: f64 = (eq152_e1924 * (var_qg_fp2_dn19 * ddt_scale));
        let eq152_e1927_d_n20: f64 = (eq152_e1924 * (var_qg_fp2_dn20 * ddt_scale));
        let eq152_e1927_d_n21: f64 = (eq152_e1924 * (var_qg_fp2_dn21 * ddt_scale));
        let eq152_e1927_d_n22: f64 = (eq152_e1924 * (var_qg_fp2_dn22 * ddt_scale));
        (eq152_e1927, eq152_e1927_d_n0, eq152_e1927_d_n1, eq152_e1927_d_n2, eq152_e1927_d_n3, eq152_e1927_d_n4, eq152_e1927_d_n5, eq152_e1927_d_n6, eq152_e1927_d_n7, eq152_e1927_d_n8, eq152_e1927_d_n9, eq152_e1927_d_n12, eq152_e1927_d_n14, eq152_e1927_d_n15, eq152_e1927_d_n16, eq152_e1927_d_n17, eq152_e1927_d_n18, eq152_e1927_d_n19, eq152_e1927_d_n20, eq152_e1927_d_n21, eq152_e1927_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq152_value: f64 = eq152_e1929;
        let eq152_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq152_node_derivatives: [f64; 20] = [eq152_e1929_d_n0, eq152_e1929_d_n1, eq152_e1929_d_n2, eq152_e1929_d_n3, eq152_e1929_d_n4, eq152_e1929_d_n5, eq152_e1929_d_n6, eq152_e1929_d_n7, eq152_e1929_d_n8, eq152_e1929_d_n9, eq152_e1929_d_n12, eq152_e1929_d_n14, eq152_e1929_d_n15, eq152_e1929_d_n16, eq152_e1929_d_n17, eq152_e1929_d_n18, eq152_e1929_d_n19, eq152_e1929_d_n20, eq152_e1929_d_n21, eq152_e1929_d_n22];
        let eq152_branch_derivative_indices: [usize; 0] = [];
        let eq152_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq152_value),
            &eq152_node_derivative_indices,
            &eq152_node_derivatives,
            &eq152_branch_derivative_indices,
            &eq152_branch_derivatives,
            multiplicity,
        );
        let (eq153_e1942, eq153_e1942_d_n0, eq153_e1942_d_n1, eq153_e1942_d_n2, eq153_e1942_d_n3, eq153_e1942_d_n4, eq153_e1942_d_n5, eq153_e1942_d_n6, eq153_e1942_d_n7, eq153_e1942_d_n8, eq153_e1942_d_n9, eq153_e1942_d_n12, eq153_e1942_d_n14, eq153_e1942_d_n15, eq153_e1942_d_n16, eq153_e1942_d_n17, eq153_e1942_d_n18, eq153_e1942_d_n19, eq153_e1942_d_n20, eq153_e1942_d_n21, eq153_e1942_d_n22,) = {
    if (((var_guard546 == 0.0) && (var_guard549 != 0.0)) && (var_guard550 == 0.0)) {
        let eq153_e1939: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 52, var_qg_fp2);
        let eq153_e1940: f64 = (p.p7 * eq153_e1939);
        let eq153_e1940_d_n0: f64 = (p.p7 * (var_qg_fp2_dn0 * ddt_scale));
        let eq153_e1940_d_n1: f64 = (p.p7 * (var_qg_fp2_dn1 * ddt_scale));
        let eq153_e1940_d_n2: f64 = (p.p7 * (var_qg_fp2_dn2 * ddt_scale));
        let eq153_e1940_d_n3: f64 = (p.p7 * (var_qg_fp2_dn3 * ddt_scale));
        let eq153_e1940_d_n4: f64 = (p.p7 * (var_qg_fp2_dn4 * ddt_scale));
        let eq153_e1940_d_n5: f64 = (p.p7 * (var_qg_fp2_dn5 * ddt_scale));
        let eq153_e1940_d_n6: f64 = (p.p7 * (var_qg_fp2_dn6 * ddt_scale));
        let eq153_e1940_d_n7: f64 = (p.p7 * (var_qg_fp2_dn7 * ddt_scale));
        let eq153_e1940_d_n8: f64 = (p.p7 * (var_qg_fp2_dn8 * ddt_scale));
        let eq153_e1940_d_n9: f64 = (p.p7 * (var_qg_fp2_dn9 * ddt_scale));
        let eq153_e1940_d_n12: f64 = (p.p7 * (var_qg_fp2_dn12 * ddt_scale));
        let eq153_e1940_d_n14: f64 = (p.p7 * (var_qg_fp2_dn14 * ddt_scale));
        let eq153_e1940_d_n15: f64 = (p.p7 * (var_qg_fp2_dn15 * ddt_scale));
        let eq153_e1940_d_n16: f64 = (p.p7 * (var_qg_fp2_dn16 * ddt_scale));
        let eq153_e1940_d_n17: f64 = (p.p7 * (var_qg_fp2_dn17 * ddt_scale));
        let eq153_e1940_d_n18: f64 = (p.p7 * (var_qg_fp2_dn18 * ddt_scale));
        let eq153_e1940_d_n19: f64 = (p.p7 * (var_qg_fp2_dn19 * ddt_scale));
        let eq153_e1940_d_n20: f64 = (p.p7 * (var_qg_fp2_dn20 * ddt_scale));
        let eq153_e1940_d_n21: f64 = (p.p7 * (var_qg_fp2_dn21 * ddt_scale));
        let eq153_e1940_d_n22: f64 = (p.p7 * (var_qg_fp2_dn22 * ddt_scale));
        (eq153_e1940, eq153_e1940_d_n0, eq153_e1940_d_n1, eq153_e1940_d_n2, eq153_e1940_d_n3, eq153_e1940_d_n4, eq153_e1940_d_n5, eq153_e1940_d_n6, eq153_e1940_d_n7, eq153_e1940_d_n8, eq153_e1940_d_n9, eq153_e1940_d_n12, eq153_e1940_d_n14, eq153_e1940_d_n15, eq153_e1940_d_n16, eq153_e1940_d_n17, eq153_e1940_d_n18, eq153_e1940_d_n19, eq153_e1940_d_n20, eq153_e1940_d_n21, eq153_e1940_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq153_value: f64 = eq153_e1942;
        let eq153_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq153_node_derivatives: [f64; 20] = [eq153_e1942_d_n0, eq153_e1942_d_n1, eq153_e1942_d_n2, eq153_e1942_d_n3, eq153_e1942_d_n4, eq153_e1942_d_n5, eq153_e1942_d_n6, eq153_e1942_d_n7, eq153_e1942_d_n8, eq153_e1942_d_n9, eq153_e1942_d_n12, eq153_e1942_d_n14, eq153_e1942_d_n15, eq153_e1942_d_n16, eq153_e1942_d_n17, eq153_e1942_d_n18, eq153_e1942_d_n19, eq153_e1942_d_n20, eq153_e1942_d_n21, eq153_e1942_d_n22];
        let eq153_branch_derivative_indices: [usize; 0] = [];
        let eq153_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq153_value),
            &eq153_node_derivative_indices,
            &eq153_node_derivatives,
            &eq153_branch_derivative_indices,
            &eq153_branch_derivatives,
            multiplicity,
        );
        let (eq154_e1957, eq154_e1957_d_n0, eq154_e1957_d_n1, eq154_e1957_d_n2, eq154_e1957_d_n3, eq154_e1957_d_n4, eq154_e1957_d_n5, eq154_e1957_d_n6, eq154_e1957_d_n7, eq154_e1957_d_n8, eq154_e1957_d_n9, eq154_e1957_d_n12, eq154_e1957_d_n14, eq154_e1957_d_n15, eq154_e1957_d_n16, eq154_e1957_d_n17, eq154_e1957_d_n18, eq154_e1957_d_n19, eq154_e1957_d_n20, eq154_e1957_d_n21, eq154_e1957_d_n22,) = {
    if (((var_guard546 == 0.0) && (var_guard549 != 0.0)) && (var_guard550 == 0.0)) {
        let eq154_e1952: f64 = (p.p7 * p.p247);
        let eq154_e1954: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 53, var_qg_fp2);
        let eq154_e1955: f64 = (eq154_e1952 * eq154_e1954);
        let eq154_e1955_d_n0: f64 = (eq154_e1952 * (var_qg_fp2_dn0 * ddt_scale));
        let eq154_e1955_d_n1: f64 = (eq154_e1952 * (var_qg_fp2_dn1 * ddt_scale));
        let eq154_e1955_d_n2: f64 = (eq154_e1952 * (var_qg_fp2_dn2 * ddt_scale));
        let eq154_e1955_d_n3: f64 = (eq154_e1952 * (var_qg_fp2_dn3 * ddt_scale));
        let eq154_e1955_d_n4: f64 = (eq154_e1952 * (var_qg_fp2_dn4 * ddt_scale));
        let eq154_e1955_d_n5: f64 = (eq154_e1952 * (var_qg_fp2_dn5 * ddt_scale));
        let eq154_e1955_d_n6: f64 = (eq154_e1952 * (var_qg_fp2_dn6 * ddt_scale));
        let eq154_e1955_d_n7: f64 = (eq154_e1952 * (var_qg_fp2_dn7 * ddt_scale));
        let eq154_e1955_d_n8: f64 = (eq154_e1952 * (var_qg_fp2_dn8 * ddt_scale));
        let eq154_e1955_d_n9: f64 = (eq154_e1952 * (var_qg_fp2_dn9 * ddt_scale));
        let eq154_e1955_d_n12: f64 = (eq154_e1952 * (var_qg_fp2_dn12 * ddt_scale));
        let eq154_e1955_d_n14: f64 = (eq154_e1952 * (var_qg_fp2_dn14 * ddt_scale));
        let eq154_e1955_d_n15: f64 = (eq154_e1952 * (var_qg_fp2_dn15 * ddt_scale));
        let eq154_e1955_d_n16: f64 = (eq154_e1952 * (var_qg_fp2_dn16 * ddt_scale));
        let eq154_e1955_d_n17: f64 = (eq154_e1952 * (var_qg_fp2_dn17 * ddt_scale));
        let eq154_e1955_d_n18: f64 = (eq154_e1952 * (var_qg_fp2_dn18 * ddt_scale));
        let eq154_e1955_d_n19: f64 = (eq154_e1952 * (var_qg_fp2_dn19 * ddt_scale));
        let eq154_e1955_d_n20: f64 = (eq154_e1952 * (var_qg_fp2_dn20 * ddt_scale));
        let eq154_e1955_d_n21: f64 = (eq154_e1952 * (var_qg_fp2_dn21 * ddt_scale));
        let eq154_e1955_d_n22: f64 = (eq154_e1952 * (var_qg_fp2_dn22 * ddt_scale));
        (eq154_e1955, eq154_e1955_d_n0, eq154_e1955_d_n1, eq154_e1955_d_n2, eq154_e1955_d_n3, eq154_e1955_d_n4, eq154_e1955_d_n5, eq154_e1955_d_n6, eq154_e1955_d_n7, eq154_e1955_d_n8, eq154_e1955_d_n9, eq154_e1955_d_n12, eq154_e1955_d_n14, eq154_e1955_d_n15, eq154_e1955_d_n16, eq154_e1955_d_n17, eq154_e1955_d_n18, eq154_e1955_d_n19, eq154_e1955_d_n20, eq154_e1955_d_n21, eq154_e1955_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq154_value: f64 = eq154_e1957;
        let eq154_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq154_node_derivatives: [f64; 20] = [eq154_e1957_d_n0, eq154_e1957_d_n1, eq154_e1957_d_n2, eq154_e1957_d_n3, eq154_e1957_d_n4, eq154_e1957_d_n5, eq154_e1957_d_n6, eq154_e1957_d_n7, eq154_e1957_d_n8, eq154_e1957_d_n9, eq154_e1957_d_n12, eq154_e1957_d_n14, eq154_e1957_d_n15, eq154_e1957_d_n16, eq154_e1957_d_n17, eq154_e1957_d_n18, eq154_e1957_d_n19, eq154_e1957_d_n20, eq154_e1957_d_n21, eq154_e1957_d_n22];
        let eq154_branch_derivative_indices: [usize; 0] = [];
        let eq154_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq154_value),
            &eq154_node_derivative_indices,
            &eq154_node_derivatives,
            &eq154_branch_derivative_indices,
            &eq154_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_7(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
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
        var_guard546: f64,
        var_guard549: f64,
        var_guard551: f64,
        var_guard552: f64,
        var_guard553: f64,
        var_guard554: f64,
        var_guard555: f64,
        var_qd_fp2s: f64,
        var_qd_fp2s_dn0: f64,
        var_qd_fp2s_dn1: f64,
        var_qd_fp2s_dn12: f64,
        var_qd_fp2s_dn14: f64,
        var_qd_fp2s_dn15: f64,
        var_qd_fp2s_dn16: f64,
        var_qd_fp2s_dn17: f64,
        var_qd_fp2s_dn18: f64,
        var_qd_fp2s_dn19: f64,
        var_qd_fp2s_dn2: f64,
        var_qd_fp2s_dn20: f64,
        var_qd_fp2s_dn21: f64,
        var_qd_fp2s_dn22: f64,
        var_qd_fp2s_dn3: f64,
        var_qd_fp2s_dn4: f64,
        var_qd_fp2s_dn5: f64,
        var_qd_fp2s_dn6: f64,
        var_qd_fp2s_dn7: f64,
        var_qd_fp2s_dn8: f64,
        var_qd_fp2s_dn9: f64,
        var_qg_fp2: f64,
        var_qg_fp2_dn0: f64,
        var_qg_fp2_dn1: f64,
        var_qg_fp2_dn12: f64,
        var_qg_fp2_dn14: f64,
        var_qg_fp2_dn15: f64,
        var_qg_fp2_dn16: f64,
        var_qg_fp2_dn17: f64,
        var_qg_fp2_dn18: f64,
        var_qg_fp2_dn19: f64,
        var_qg_fp2_dn2: f64,
        var_qg_fp2_dn20: f64,
        var_qg_fp2_dn21: f64,
        var_qg_fp2_dn22: f64,
        var_qg_fp2_dn3: f64,
        var_qg_fp2_dn4: f64,
        var_qg_fp2_dn5: f64,
        var_qg_fp2_dn6: f64,
        var_qg_fp2_dn7: f64,
        var_qg_fp2_dn8: f64,
        var_qg_fp2_dn9: f64,
        var_qg_fp2s: f64,
        var_qg_fp2s_dn0: f64,
        var_qg_fp2s_dn1: f64,
        var_qg_fp2s_dn12: f64,
        var_qg_fp2s_dn14: f64,
        var_qg_fp2s_dn15: f64,
        var_qg_fp2s_dn16: f64,
        var_qg_fp2s_dn17: f64,
        var_qg_fp2s_dn18: f64,
        var_qg_fp2s_dn19: f64,
        var_qg_fp2s_dn2: f64,
        var_qg_fp2s_dn20: f64,
        var_qg_fp2s_dn21: f64,
        var_qg_fp2s_dn22: f64,
        var_qg_fp2s_dn3: f64,
        var_qg_fp2s_dn4: f64,
        var_qg_fp2s_dn5: f64,
        var_qg_fp2s_dn6: f64,
        var_qg_fp2s_dn7: f64,
        var_qg_fp2s_dn8: f64,
        var_qg_fp2s_dn9: f64,
    ) {
        let (eq155_e1969, eq155_e1969_d_n0, eq155_e1969_d_n1, eq155_e1969_d_n2, eq155_e1969_d_n3, eq155_e1969_d_n4, eq155_e1969_d_n5, eq155_e1969_d_n6, eq155_e1969_d_n7, eq155_e1969_d_n8, eq155_e1969_d_n9, eq155_e1969_d_n12, eq155_e1969_d_n14, eq155_e1969_d_n15, eq155_e1969_d_n16, eq155_e1969_d_n17, eq155_e1969_d_n18, eq155_e1969_d_n19, eq155_e1969_d_n20, eq155_e1969_d_n21, eq155_e1969_d_n22,) = {
    if ((var_guard546 == 0.0) && (var_guard549 != 0.0)) {
        let eq155_e1965: f64 = (p.p252 * var_qg_fp2);
        let eq155_e1965_d_n0: f64 = (p.p252 * var_qg_fp2_dn0);
        let eq155_e1965_d_n1: f64 = (p.p252 * var_qg_fp2_dn1);
        let eq155_e1965_d_n2: f64 = (p.p252 * var_qg_fp2_dn2);
        let eq155_e1965_d_n3: f64 = (p.p252 * var_qg_fp2_dn3);
        let eq155_e1965_d_n4: f64 = (p.p252 * var_qg_fp2_dn4);
        let eq155_e1965_d_n5: f64 = (p.p252 * var_qg_fp2_dn5);
        let eq155_e1965_d_n6: f64 = (p.p252 * var_qg_fp2_dn6);
        let eq155_e1965_d_n7: f64 = (p.p252 * var_qg_fp2_dn7);
        let eq155_e1965_d_n8: f64 = (p.p252 * var_qg_fp2_dn8);
        let eq155_e1965_d_n9: f64 = (p.p252 * var_qg_fp2_dn9);
        let eq155_e1965_d_n12: f64 = (p.p252 * var_qg_fp2_dn12);
        let eq155_e1965_d_n14: f64 = (p.p252 * var_qg_fp2_dn14);
        let eq155_e1965_d_n15: f64 = (p.p252 * var_qg_fp2_dn15);
        let eq155_e1965_d_n16: f64 = (p.p252 * var_qg_fp2_dn16);
        let eq155_e1965_d_n17: f64 = (p.p252 * var_qg_fp2_dn17);
        let eq155_e1965_d_n18: f64 = (p.p252 * var_qg_fp2_dn18);
        let eq155_e1965_d_n19: f64 = (p.p252 * var_qg_fp2_dn19);
        let eq155_e1965_d_n20: f64 = (p.p252 * var_qg_fp2_dn20);
        let eq155_e1965_d_n21: f64 = (p.p252 * var_qg_fp2_dn21);
        let eq155_e1965_d_n22: f64 = (p.p252 * var_qg_fp2_dn22);
        let eq155_e1966: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 54, eq155_e1965);
        let eq155_e1967: f64 = (p.p7 * eq155_e1966);
        let eq155_e1967_d_n0: f64 = (p.p7 * (eq155_e1965_d_n0 * ddt_scale));
        let eq155_e1967_d_n1: f64 = (p.p7 * (eq155_e1965_d_n1 * ddt_scale));
        let eq155_e1967_d_n2: f64 = (p.p7 * (eq155_e1965_d_n2 * ddt_scale));
        let eq155_e1967_d_n3: f64 = (p.p7 * (eq155_e1965_d_n3 * ddt_scale));
        let eq155_e1967_d_n4: f64 = (p.p7 * (eq155_e1965_d_n4 * ddt_scale));
        let eq155_e1967_d_n5: f64 = (p.p7 * (eq155_e1965_d_n5 * ddt_scale));
        let eq155_e1967_d_n6: f64 = (p.p7 * (eq155_e1965_d_n6 * ddt_scale));
        let eq155_e1967_d_n7: f64 = (p.p7 * (eq155_e1965_d_n7 * ddt_scale));
        let eq155_e1967_d_n8: f64 = (p.p7 * (eq155_e1965_d_n8 * ddt_scale));
        let eq155_e1967_d_n9: f64 = (p.p7 * (eq155_e1965_d_n9 * ddt_scale));
        let eq155_e1967_d_n12: f64 = (p.p7 * (eq155_e1965_d_n12 * ddt_scale));
        let eq155_e1967_d_n14: f64 = (p.p7 * (eq155_e1965_d_n14 * ddt_scale));
        let eq155_e1967_d_n15: f64 = (p.p7 * (eq155_e1965_d_n15 * ddt_scale));
        let eq155_e1967_d_n16: f64 = (p.p7 * (eq155_e1965_d_n16 * ddt_scale));
        let eq155_e1967_d_n17: f64 = (p.p7 * (eq155_e1965_d_n17 * ddt_scale));
        let eq155_e1967_d_n18: f64 = (p.p7 * (eq155_e1965_d_n18 * ddt_scale));
        let eq155_e1967_d_n19: f64 = (p.p7 * (eq155_e1965_d_n19 * ddt_scale));
        let eq155_e1967_d_n20: f64 = (p.p7 * (eq155_e1965_d_n20 * ddt_scale));
        let eq155_e1967_d_n21: f64 = (p.p7 * (eq155_e1965_d_n21 * ddt_scale));
        let eq155_e1967_d_n22: f64 = (p.p7 * (eq155_e1965_d_n22 * ddt_scale));
        (eq155_e1967, eq155_e1967_d_n0, eq155_e1967_d_n1, eq155_e1967_d_n2, eq155_e1967_d_n3, eq155_e1967_d_n4, eq155_e1967_d_n5, eq155_e1967_d_n6, eq155_e1967_d_n7, eq155_e1967_d_n8, eq155_e1967_d_n9, eq155_e1967_d_n12, eq155_e1967_d_n14, eq155_e1967_d_n15, eq155_e1967_d_n16, eq155_e1967_d_n17, eq155_e1967_d_n18, eq155_e1967_d_n19, eq155_e1967_d_n20, eq155_e1967_d_n21, eq155_e1967_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq155_value: f64 = eq155_e1969;
        let eq155_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq155_node_derivatives: [f64; 20] = [eq155_e1969_d_n0, eq155_e1969_d_n1, eq155_e1969_d_n2, eq155_e1969_d_n3, eq155_e1969_d_n4, eq155_e1969_d_n5, eq155_e1969_d_n6, eq155_e1969_d_n7, eq155_e1969_d_n8, eq155_e1969_d_n9, eq155_e1969_d_n12, eq155_e1969_d_n14, eq155_e1969_d_n15, eq155_e1969_d_n16, eq155_e1969_d_n17, eq155_e1969_d_n18, eq155_e1969_d_n19, eq155_e1969_d_n20, eq155_e1969_d_n21, eq155_e1969_d_n22];
        let eq155_branch_derivative_indices: [usize; 0] = [];
        let eq155_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(7),
            multiplicity * (eq155_value),
            &eq155_node_derivative_indices,
            &eq155_node_derivatives,
            &eq155_branch_derivative_indices,
            &eq155_branch_derivatives,
            multiplicity,
        );
        let (eq156_e1978, eq156_e1978_d_n0, eq156_e1978_d_n1, eq156_e1978_d_n2, eq156_e1978_d_n3, eq156_e1978_d_n4, eq156_e1978_d_n5, eq156_e1978_d_n6, eq156_e1978_d_n7, eq156_e1978_d_n8, eq156_e1978_d_n9, eq156_e1978_d_n12, eq156_e1978_d_n14, eq156_e1978_d_n15, eq156_e1978_d_n16, eq156_e1978_d_n17, eq156_e1978_d_n18, eq156_e1978_d_n19, eq156_e1978_d_n20, eq156_e1978_d_n21, eq156_e1978_d_n22,) = {
    if ((var_guard551 != 0.0) && (var_guard552 != 0.0)) {
        let eq156_e1975: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 55, var_qd_fp2s);
        let eq156_e1976: f64 = (p.p7 * eq156_e1975);
        let eq156_e1976_d_n0: f64 = (p.p7 * (var_qd_fp2s_dn0 * ddt_scale));
        let eq156_e1976_d_n1: f64 = (p.p7 * (var_qd_fp2s_dn1 * ddt_scale));
        let eq156_e1976_d_n2: f64 = (p.p7 * (var_qd_fp2s_dn2 * ddt_scale));
        let eq156_e1976_d_n3: f64 = (p.p7 * (var_qd_fp2s_dn3 * ddt_scale));
        let eq156_e1976_d_n4: f64 = (p.p7 * (var_qd_fp2s_dn4 * ddt_scale));
        let eq156_e1976_d_n5: f64 = (p.p7 * (var_qd_fp2s_dn5 * ddt_scale));
        let eq156_e1976_d_n6: f64 = (p.p7 * (var_qd_fp2s_dn6 * ddt_scale));
        let eq156_e1976_d_n7: f64 = (p.p7 * (var_qd_fp2s_dn7 * ddt_scale));
        let eq156_e1976_d_n8: f64 = (p.p7 * (var_qd_fp2s_dn8 * ddt_scale));
        let eq156_e1976_d_n9: f64 = (p.p7 * (var_qd_fp2s_dn9 * ddt_scale));
        let eq156_e1976_d_n12: f64 = (p.p7 * (var_qd_fp2s_dn12 * ddt_scale));
        let eq156_e1976_d_n14: f64 = (p.p7 * (var_qd_fp2s_dn14 * ddt_scale));
        let eq156_e1976_d_n15: f64 = (p.p7 * (var_qd_fp2s_dn15 * ddt_scale));
        let eq156_e1976_d_n16: f64 = (p.p7 * (var_qd_fp2s_dn16 * ddt_scale));
        let eq156_e1976_d_n17: f64 = (p.p7 * (var_qd_fp2s_dn17 * ddt_scale));
        let eq156_e1976_d_n18: f64 = (p.p7 * (var_qd_fp2s_dn18 * ddt_scale));
        let eq156_e1976_d_n19: f64 = (p.p7 * (var_qd_fp2s_dn19 * ddt_scale));
        let eq156_e1976_d_n20: f64 = (p.p7 * (var_qd_fp2s_dn20 * ddt_scale));
        let eq156_e1976_d_n21: f64 = (p.p7 * (var_qd_fp2s_dn21 * ddt_scale));
        let eq156_e1976_d_n22: f64 = (p.p7 * (var_qd_fp2s_dn22 * ddt_scale));
        (eq156_e1976, eq156_e1976_d_n0, eq156_e1976_d_n1, eq156_e1976_d_n2, eq156_e1976_d_n3, eq156_e1976_d_n4, eq156_e1976_d_n5, eq156_e1976_d_n6, eq156_e1976_d_n7, eq156_e1976_d_n8, eq156_e1976_d_n9, eq156_e1976_d_n12, eq156_e1976_d_n14, eq156_e1976_d_n15, eq156_e1976_d_n16, eq156_e1976_d_n17, eq156_e1976_d_n18, eq156_e1976_d_n19, eq156_e1976_d_n20, eq156_e1976_d_n21, eq156_e1976_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq156_value: f64 = eq156_e1978;
        let eq156_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq156_node_derivatives: [f64; 20] = [eq156_e1978_d_n0, eq156_e1978_d_n1, eq156_e1978_d_n2, eq156_e1978_d_n3, eq156_e1978_d_n4, eq156_e1978_d_n5, eq156_e1978_d_n6, eq156_e1978_d_n7, eq156_e1978_d_n8, eq156_e1978_d_n9, eq156_e1978_d_n12, eq156_e1978_d_n14, eq156_e1978_d_n15, eq156_e1978_d_n16, eq156_e1978_d_n17, eq156_e1978_d_n18, eq156_e1978_d_n19, eq156_e1978_d_n20, eq156_e1978_d_n21, eq156_e1978_d_n22];
        let eq156_branch_derivative_indices: [usize; 0] = [];
        let eq156_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(20),
            multiplicity * (eq156_value),
            &eq156_node_derivative_indices,
            &eq156_node_derivatives,
            &eq156_branch_derivative_indices,
            &eq156_branch_derivatives,
            multiplicity,
        );
        let (eq157_e1989, eq157_e1989_d_n0, eq157_e1989_d_n1, eq157_e1989_d_n2, eq157_e1989_d_n3, eq157_e1989_d_n4, eq157_e1989_d_n5, eq157_e1989_d_n6, eq157_e1989_d_n7, eq157_e1989_d_n8, eq157_e1989_d_n9, eq157_e1989_d_n12, eq157_e1989_d_n14, eq157_e1989_d_n15, eq157_e1989_d_n16, eq157_e1989_d_n17, eq157_e1989_d_n18, eq157_e1989_d_n19, eq157_e1989_d_n20, eq157_e1989_d_n21, eq157_e1989_d_n22,) = {
    if (((var_guard551 != 0.0) && (var_guard552 != 0.0)) && (var_guard553 != 0.0)) {
        let eq157_e1986: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 56, var_qg_fp2s);
        let eq157_e1987: f64 = (p.p7 * eq157_e1986);
        let eq157_e1987_d_n0: f64 = (p.p7 * (var_qg_fp2s_dn0 * ddt_scale));
        let eq157_e1987_d_n1: f64 = (p.p7 * (var_qg_fp2s_dn1 * ddt_scale));
        let eq157_e1987_d_n2: f64 = (p.p7 * (var_qg_fp2s_dn2 * ddt_scale));
        let eq157_e1987_d_n3: f64 = (p.p7 * (var_qg_fp2s_dn3 * ddt_scale));
        let eq157_e1987_d_n4: f64 = (p.p7 * (var_qg_fp2s_dn4 * ddt_scale));
        let eq157_e1987_d_n5: f64 = (p.p7 * (var_qg_fp2s_dn5 * ddt_scale));
        let eq157_e1987_d_n6: f64 = (p.p7 * (var_qg_fp2s_dn6 * ddt_scale));
        let eq157_e1987_d_n7: f64 = (p.p7 * (var_qg_fp2s_dn7 * ddt_scale));
        let eq157_e1987_d_n8: f64 = (p.p7 * (var_qg_fp2s_dn8 * ddt_scale));
        let eq157_e1987_d_n9: f64 = (p.p7 * (var_qg_fp2s_dn9 * ddt_scale));
        let eq157_e1987_d_n12: f64 = (p.p7 * (var_qg_fp2s_dn12 * ddt_scale));
        let eq157_e1987_d_n14: f64 = (p.p7 * (var_qg_fp2s_dn14 * ddt_scale));
        let eq157_e1987_d_n15: f64 = (p.p7 * (var_qg_fp2s_dn15 * ddt_scale));
        let eq157_e1987_d_n16: f64 = (p.p7 * (var_qg_fp2s_dn16 * ddt_scale));
        let eq157_e1987_d_n17: f64 = (p.p7 * (var_qg_fp2s_dn17 * ddt_scale));
        let eq157_e1987_d_n18: f64 = (p.p7 * (var_qg_fp2s_dn18 * ddt_scale));
        let eq157_e1987_d_n19: f64 = (p.p7 * (var_qg_fp2s_dn19 * ddt_scale));
        let eq157_e1987_d_n20: f64 = (p.p7 * (var_qg_fp2s_dn20 * ddt_scale));
        let eq157_e1987_d_n21: f64 = (p.p7 * (var_qg_fp2s_dn21 * ddt_scale));
        let eq157_e1987_d_n22: f64 = (p.p7 * (var_qg_fp2s_dn22 * ddt_scale));
        (eq157_e1987, eq157_e1987_d_n0, eq157_e1987_d_n1, eq157_e1987_d_n2, eq157_e1987_d_n3, eq157_e1987_d_n4, eq157_e1987_d_n5, eq157_e1987_d_n6, eq157_e1987_d_n7, eq157_e1987_d_n8, eq157_e1987_d_n9, eq157_e1987_d_n12, eq157_e1987_d_n14, eq157_e1987_d_n15, eq157_e1987_d_n16, eq157_e1987_d_n17, eq157_e1987_d_n18, eq157_e1987_d_n19, eq157_e1987_d_n20, eq157_e1987_d_n21, eq157_e1987_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq157_value: f64 = eq157_e1989;
        let eq157_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq157_node_derivatives: [f64; 20] = [eq157_e1989_d_n0, eq157_e1989_d_n1, eq157_e1989_d_n2, eq157_e1989_d_n3, eq157_e1989_d_n4, eq157_e1989_d_n5, eq157_e1989_d_n6, eq157_e1989_d_n7, eq157_e1989_d_n8, eq157_e1989_d_n9, eq157_e1989_d_n12, eq157_e1989_d_n14, eq157_e1989_d_n15, eq157_e1989_d_n16, eq157_e1989_d_n17, eq157_e1989_d_n18, eq157_e1989_d_n19, eq157_e1989_d_n20, eq157_e1989_d_n21, eq157_e1989_d_n22];
        let eq157_branch_derivative_indices: [usize; 0] = [];
        let eq157_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(20),
            multiplicity * (eq157_value),
            &eq157_node_derivative_indices,
            &eq157_node_derivatives,
            &eq157_branch_derivative_indices,
            &eq157_branch_derivatives,
            multiplicity,
        );
        let (eq158_e2002, eq158_e2002_d_n0, eq158_e2002_d_n1, eq158_e2002_d_n2, eq158_e2002_d_n3, eq158_e2002_d_n4, eq158_e2002_d_n5, eq158_e2002_d_n6, eq158_e2002_d_n7, eq158_e2002_d_n8, eq158_e2002_d_n9, eq158_e2002_d_n12, eq158_e2002_d_n14, eq158_e2002_d_n15, eq158_e2002_d_n16, eq158_e2002_d_n17, eq158_e2002_d_n18, eq158_e2002_d_n19, eq158_e2002_d_n20, eq158_e2002_d_n21, eq158_e2002_d_n22,) = {
    if (((var_guard551 != 0.0) && (var_guard552 != 0.0)) && (var_guard553 != 0.0)) {
        let eq158_e1997: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 57, var_qg_fp2s);
        let eq158_e1998: f64 = (p.p7 * eq158_e1997);
        let eq158_e1998_d_n0: f64 = (p.p7 * (var_qg_fp2s_dn0 * ddt_scale));
        let eq158_e1998_d_n1: f64 = (p.p7 * (var_qg_fp2s_dn1 * ddt_scale));
        let eq158_e1998_d_n2: f64 = (p.p7 * (var_qg_fp2s_dn2 * ddt_scale));
        let eq158_e1998_d_n3: f64 = (p.p7 * (var_qg_fp2s_dn3 * ddt_scale));
        let eq158_e1998_d_n4: f64 = (p.p7 * (var_qg_fp2s_dn4 * ddt_scale));
        let eq158_e1998_d_n5: f64 = (p.p7 * (var_qg_fp2s_dn5 * ddt_scale));
        let eq158_e1998_d_n6: f64 = (p.p7 * (var_qg_fp2s_dn6 * ddt_scale));
        let eq158_e1998_d_n7: f64 = (p.p7 * (var_qg_fp2s_dn7 * ddt_scale));
        let eq158_e1998_d_n8: f64 = (p.p7 * (var_qg_fp2s_dn8 * ddt_scale));
        let eq158_e1998_d_n9: f64 = (p.p7 * (var_qg_fp2s_dn9 * ddt_scale));
        let eq158_e1998_d_n12: f64 = (p.p7 * (var_qg_fp2s_dn12 * ddt_scale));
        let eq158_e1998_d_n14: f64 = (p.p7 * (var_qg_fp2s_dn14 * ddt_scale));
        let eq158_e1998_d_n15: f64 = (p.p7 * (var_qg_fp2s_dn15 * ddt_scale));
        let eq158_e1998_d_n16: f64 = (p.p7 * (var_qg_fp2s_dn16 * ddt_scale));
        let eq158_e1998_d_n17: f64 = (p.p7 * (var_qg_fp2s_dn17 * ddt_scale));
        let eq158_e1998_d_n18: f64 = (p.p7 * (var_qg_fp2s_dn18 * ddt_scale));
        let eq158_e1998_d_n19: f64 = (p.p7 * (var_qg_fp2s_dn19 * ddt_scale));
        let eq158_e1998_d_n20: f64 = (p.p7 * (var_qg_fp2s_dn20 * ddt_scale));
        let eq158_e1998_d_n21: f64 = (p.p7 * (var_qg_fp2s_dn21 * ddt_scale));
        let eq158_e1998_d_n22: f64 = (p.p7 * (var_qg_fp2s_dn22 * ddt_scale));
        let eq158_e2000: f64 = (eq158_e1998 * p.p247);
        let eq158_e2000_d_n0: f64 = (eq158_e1998_d_n0 * p.p247);
        let eq158_e2000_d_n1: f64 = (eq158_e1998_d_n1 * p.p247);
        let eq158_e2000_d_n2: f64 = (eq158_e1998_d_n2 * p.p247);
        let eq158_e2000_d_n3: f64 = (eq158_e1998_d_n3 * p.p247);
        let eq158_e2000_d_n4: f64 = (eq158_e1998_d_n4 * p.p247);
        let eq158_e2000_d_n5: f64 = (eq158_e1998_d_n5 * p.p247);
        let eq158_e2000_d_n6: f64 = (eq158_e1998_d_n6 * p.p247);
        let eq158_e2000_d_n7: f64 = (eq158_e1998_d_n7 * p.p247);
        let eq158_e2000_d_n8: f64 = (eq158_e1998_d_n8 * p.p247);
        let eq158_e2000_d_n9: f64 = (eq158_e1998_d_n9 * p.p247);
        let eq158_e2000_d_n12: f64 = (eq158_e1998_d_n12 * p.p247);
        let eq158_e2000_d_n14: f64 = (eq158_e1998_d_n14 * p.p247);
        let eq158_e2000_d_n15: f64 = (eq158_e1998_d_n15 * p.p247);
        let eq158_e2000_d_n16: f64 = (eq158_e1998_d_n16 * p.p247);
        let eq158_e2000_d_n17: f64 = (eq158_e1998_d_n17 * p.p247);
        let eq158_e2000_d_n18: f64 = (eq158_e1998_d_n18 * p.p247);
        let eq158_e2000_d_n19: f64 = (eq158_e1998_d_n19 * p.p247);
        let eq158_e2000_d_n20: f64 = (eq158_e1998_d_n20 * p.p247);
        let eq158_e2000_d_n21: f64 = (eq158_e1998_d_n21 * p.p247);
        let eq158_e2000_d_n22: f64 = (eq158_e1998_d_n22 * p.p247);
        (eq158_e2000, eq158_e2000_d_n0, eq158_e2000_d_n1, eq158_e2000_d_n2, eq158_e2000_d_n3, eq158_e2000_d_n4, eq158_e2000_d_n5, eq158_e2000_d_n6, eq158_e2000_d_n7, eq158_e2000_d_n8, eq158_e2000_d_n9, eq158_e2000_d_n12, eq158_e2000_d_n14, eq158_e2000_d_n15, eq158_e2000_d_n16, eq158_e2000_d_n17, eq158_e2000_d_n18, eq158_e2000_d_n19, eq158_e2000_d_n20, eq158_e2000_d_n21, eq158_e2000_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq158_value: f64 = eq158_e2002;
        let eq158_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq158_node_derivatives: [f64; 20] = [eq158_e2002_d_n0, eq158_e2002_d_n1, eq158_e2002_d_n2, eq158_e2002_d_n3, eq158_e2002_d_n4, eq158_e2002_d_n5, eq158_e2002_d_n6, eq158_e2002_d_n7, eq158_e2002_d_n8, eq158_e2002_d_n9, eq158_e2002_d_n12, eq158_e2002_d_n14, eq158_e2002_d_n15, eq158_e2002_d_n16, eq158_e2002_d_n17, eq158_e2002_d_n18, eq158_e2002_d_n19, eq158_e2002_d_n20, eq158_e2002_d_n21, eq158_e2002_d_n22];
        let eq158_branch_derivative_indices: [usize; 0] = [];
        let eq158_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(20),
            multiplicity * (eq158_value),
            &eq158_node_derivative_indices,
            &eq158_node_derivatives,
            &eq158_branch_derivative_indices,
            &eq158_branch_derivatives,
            multiplicity,
        );
        let (eq159_e2014, eq159_e2014_d_n0, eq159_e2014_d_n1, eq159_e2014_d_n2, eq159_e2014_d_n3, eq159_e2014_d_n4, eq159_e2014_d_n5, eq159_e2014_d_n6, eq159_e2014_d_n7, eq159_e2014_d_n8, eq159_e2014_d_n9, eq159_e2014_d_n12, eq159_e2014_d_n14, eq159_e2014_d_n15, eq159_e2014_d_n16, eq159_e2014_d_n17, eq159_e2014_d_n18, eq159_e2014_d_n19, eq159_e2014_d_n20, eq159_e2014_d_n21, eq159_e2014_d_n22,) = {
    if (((var_guard551 != 0.0) && (var_guard552 != 0.0)) && (var_guard553 == 0.0)) {
        let eq159_e2011: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 58, var_qg_fp2s);
        let eq159_e2012: f64 = (p.p7 * eq159_e2011);
        let eq159_e2012_d_n0: f64 = (p.p7 * (var_qg_fp2s_dn0 * ddt_scale));
        let eq159_e2012_d_n1: f64 = (p.p7 * (var_qg_fp2s_dn1 * ddt_scale));
        let eq159_e2012_d_n2: f64 = (p.p7 * (var_qg_fp2s_dn2 * ddt_scale));
        let eq159_e2012_d_n3: f64 = (p.p7 * (var_qg_fp2s_dn3 * ddt_scale));
        let eq159_e2012_d_n4: f64 = (p.p7 * (var_qg_fp2s_dn4 * ddt_scale));
        let eq159_e2012_d_n5: f64 = (p.p7 * (var_qg_fp2s_dn5 * ddt_scale));
        let eq159_e2012_d_n6: f64 = (p.p7 * (var_qg_fp2s_dn6 * ddt_scale));
        let eq159_e2012_d_n7: f64 = (p.p7 * (var_qg_fp2s_dn7 * ddt_scale));
        let eq159_e2012_d_n8: f64 = (p.p7 * (var_qg_fp2s_dn8 * ddt_scale));
        let eq159_e2012_d_n9: f64 = (p.p7 * (var_qg_fp2s_dn9 * ddt_scale));
        let eq159_e2012_d_n12: f64 = (p.p7 * (var_qg_fp2s_dn12 * ddt_scale));
        let eq159_e2012_d_n14: f64 = (p.p7 * (var_qg_fp2s_dn14 * ddt_scale));
        let eq159_e2012_d_n15: f64 = (p.p7 * (var_qg_fp2s_dn15 * ddt_scale));
        let eq159_e2012_d_n16: f64 = (p.p7 * (var_qg_fp2s_dn16 * ddt_scale));
        let eq159_e2012_d_n17: f64 = (p.p7 * (var_qg_fp2s_dn17 * ddt_scale));
        let eq159_e2012_d_n18: f64 = (p.p7 * (var_qg_fp2s_dn18 * ddt_scale));
        let eq159_e2012_d_n19: f64 = (p.p7 * (var_qg_fp2s_dn19 * ddt_scale));
        let eq159_e2012_d_n20: f64 = (p.p7 * (var_qg_fp2s_dn20 * ddt_scale));
        let eq159_e2012_d_n21: f64 = (p.p7 * (var_qg_fp2s_dn21 * ddt_scale));
        let eq159_e2012_d_n22: f64 = (p.p7 * (var_qg_fp2s_dn22 * ddt_scale));
        (eq159_e2012, eq159_e2012_d_n0, eq159_e2012_d_n1, eq159_e2012_d_n2, eq159_e2012_d_n3, eq159_e2012_d_n4, eq159_e2012_d_n5, eq159_e2012_d_n6, eq159_e2012_d_n7, eq159_e2012_d_n8, eq159_e2012_d_n9, eq159_e2012_d_n12, eq159_e2012_d_n14, eq159_e2012_d_n15, eq159_e2012_d_n16, eq159_e2012_d_n17, eq159_e2012_d_n18, eq159_e2012_d_n19, eq159_e2012_d_n20, eq159_e2012_d_n21, eq159_e2012_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq159_value: f64 = eq159_e2014;
        let eq159_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq159_node_derivatives: [f64; 20] = [eq159_e2014_d_n0, eq159_e2014_d_n1, eq159_e2014_d_n2, eq159_e2014_d_n3, eq159_e2014_d_n4, eq159_e2014_d_n5, eq159_e2014_d_n6, eq159_e2014_d_n7, eq159_e2014_d_n8, eq159_e2014_d_n9, eq159_e2014_d_n12, eq159_e2014_d_n14, eq159_e2014_d_n15, eq159_e2014_d_n16, eq159_e2014_d_n17, eq159_e2014_d_n18, eq159_e2014_d_n19, eq159_e2014_d_n20, eq159_e2014_d_n21, eq159_e2014_d_n22];
        let eq159_branch_derivative_indices: [usize; 0] = [];
        let eq159_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(20),
            multiplicity * (eq159_value),
            &eq159_node_derivative_indices,
            &eq159_node_derivatives,
            &eq159_branch_derivative_indices,
            &eq159_branch_derivatives,
            multiplicity,
        );
        let (eq160_e2028, eq160_e2028_d_n0, eq160_e2028_d_n1, eq160_e2028_d_n2, eq160_e2028_d_n3, eq160_e2028_d_n4, eq160_e2028_d_n5, eq160_e2028_d_n6, eq160_e2028_d_n7, eq160_e2028_d_n8, eq160_e2028_d_n9, eq160_e2028_d_n12, eq160_e2028_d_n14, eq160_e2028_d_n15, eq160_e2028_d_n16, eq160_e2028_d_n17, eq160_e2028_d_n18, eq160_e2028_d_n19, eq160_e2028_d_n20, eq160_e2028_d_n21, eq160_e2028_d_n22,) = {
    if (((var_guard551 != 0.0) && (var_guard552 != 0.0)) && (var_guard553 == 0.0)) {
        let eq160_e2023: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 59, var_qg_fp2s);
        let eq160_e2024: f64 = (p.p7 * eq160_e2023);
        let eq160_e2024_d_n0: f64 = (p.p7 * (var_qg_fp2s_dn0 * ddt_scale));
        let eq160_e2024_d_n1: f64 = (p.p7 * (var_qg_fp2s_dn1 * ddt_scale));
        let eq160_e2024_d_n2: f64 = (p.p7 * (var_qg_fp2s_dn2 * ddt_scale));
        let eq160_e2024_d_n3: f64 = (p.p7 * (var_qg_fp2s_dn3 * ddt_scale));
        let eq160_e2024_d_n4: f64 = (p.p7 * (var_qg_fp2s_dn4 * ddt_scale));
        let eq160_e2024_d_n5: f64 = (p.p7 * (var_qg_fp2s_dn5 * ddt_scale));
        let eq160_e2024_d_n6: f64 = (p.p7 * (var_qg_fp2s_dn6 * ddt_scale));
        let eq160_e2024_d_n7: f64 = (p.p7 * (var_qg_fp2s_dn7 * ddt_scale));
        let eq160_e2024_d_n8: f64 = (p.p7 * (var_qg_fp2s_dn8 * ddt_scale));
        let eq160_e2024_d_n9: f64 = (p.p7 * (var_qg_fp2s_dn9 * ddt_scale));
        let eq160_e2024_d_n12: f64 = (p.p7 * (var_qg_fp2s_dn12 * ddt_scale));
        let eq160_e2024_d_n14: f64 = (p.p7 * (var_qg_fp2s_dn14 * ddt_scale));
        let eq160_e2024_d_n15: f64 = (p.p7 * (var_qg_fp2s_dn15 * ddt_scale));
        let eq160_e2024_d_n16: f64 = (p.p7 * (var_qg_fp2s_dn16 * ddt_scale));
        let eq160_e2024_d_n17: f64 = (p.p7 * (var_qg_fp2s_dn17 * ddt_scale));
        let eq160_e2024_d_n18: f64 = (p.p7 * (var_qg_fp2s_dn18 * ddt_scale));
        let eq160_e2024_d_n19: f64 = (p.p7 * (var_qg_fp2s_dn19 * ddt_scale));
        let eq160_e2024_d_n20: f64 = (p.p7 * (var_qg_fp2s_dn20 * ddt_scale));
        let eq160_e2024_d_n21: f64 = (p.p7 * (var_qg_fp2s_dn21 * ddt_scale));
        let eq160_e2024_d_n22: f64 = (p.p7 * (var_qg_fp2s_dn22 * ddt_scale));
        let eq160_e2026: f64 = (eq160_e2024 * p.p247);
        let eq160_e2026_d_n0: f64 = (eq160_e2024_d_n0 * p.p247);
        let eq160_e2026_d_n1: f64 = (eq160_e2024_d_n1 * p.p247);
        let eq160_e2026_d_n2: f64 = (eq160_e2024_d_n2 * p.p247);
        let eq160_e2026_d_n3: f64 = (eq160_e2024_d_n3 * p.p247);
        let eq160_e2026_d_n4: f64 = (eq160_e2024_d_n4 * p.p247);
        let eq160_e2026_d_n5: f64 = (eq160_e2024_d_n5 * p.p247);
        let eq160_e2026_d_n6: f64 = (eq160_e2024_d_n6 * p.p247);
        let eq160_e2026_d_n7: f64 = (eq160_e2024_d_n7 * p.p247);
        let eq160_e2026_d_n8: f64 = (eq160_e2024_d_n8 * p.p247);
        let eq160_e2026_d_n9: f64 = (eq160_e2024_d_n9 * p.p247);
        let eq160_e2026_d_n12: f64 = (eq160_e2024_d_n12 * p.p247);
        let eq160_e2026_d_n14: f64 = (eq160_e2024_d_n14 * p.p247);
        let eq160_e2026_d_n15: f64 = (eq160_e2024_d_n15 * p.p247);
        let eq160_e2026_d_n16: f64 = (eq160_e2024_d_n16 * p.p247);
        let eq160_e2026_d_n17: f64 = (eq160_e2024_d_n17 * p.p247);
        let eq160_e2026_d_n18: f64 = (eq160_e2024_d_n18 * p.p247);
        let eq160_e2026_d_n19: f64 = (eq160_e2024_d_n19 * p.p247);
        let eq160_e2026_d_n20: f64 = (eq160_e2024_d_n20 * p.p247);
        let eq160_e2026_d_n21: f64 = (eq160_e2024_d_n21 * p.p247);
        let eq160_e2026_d_n22: f64 = (eq160_e2024_d_n22 * p.p247);
        (eq160_e2026, eq160_e2026_d_n0, eq160_e2026_d_n1, eq160_e2026_d_n2, eq160_e2026_d_n3, eq160_e2026_d_n4, eq160_e2026_d_n5, eq160_e2026_d_n6, eq160_e2026_d_n7, eq160_e2026_d_n8, eq160_e2026_d_n9, eq160_e2026_d_n12, eq160_e2026_d_n14, eq160_e2026_d_n15, eq160_e2026_d_n16, eq160_e2026_d_n17, eq160_e2026_d_n18, eq160_e2026_d_n19, eq160_e2026_d_n20, eq160_e2026_d_n21, eq160_e2026_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq160_value: f64 = eq160_e2028;
        let eq160_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq160_node_derivatives: [f64; 20] = [eq160_e2028_d_n0, eq160_e2028_d_n1, eq160_e2028_d_n2, eq160_e2028_d_n3, eq160_e2028_d_n4, eq160_e2028_d_n5, eq160_e2028_d_n6, eq160_e2028_d_n7, eq160_e2028_d_n8, eq160_e2028_d_n9, eq160_e2028_d_n12, eq160_e2028_d_n14, eq160_e2028_d_n15, eq160_e2028_d_n16, eq160_e2028_d_n17, eq160_e2028_d_n18, eq160_e2028_d_n19, eq160_e2028_d_n20, eq160_e2028_d_n21, eq160_e2028_d_n22];
        let eq160_branch_derivative_indices: [usize; 0] = [];
        let eq160_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(20),
            multiplicity * (eq160_value),
            &eq160_node_derivative_indices,
            &eq160_node_derivatives,
            &eq160_branch_derivative_indices,
            &eq160_branch_derivatives,
            multiplicity,
        );
        let (eq161_e2039, eq161_e2039_d_n0, eq161_e2039_d_n1, eq161_e2039_d_n2, eq161_e2039_d_n3, eq161_e2039_d_n4, eq161_e2039_d_n5, eq161_e2039_d_n6, eq161_e2039_d_n7, eq161_e2039_d_n8, eq161_e2039_d_n9, eq161_e2039_d_n12, eq161_e2039_d_n14, eq161_e2039_d_n15, eq161_e2039_d_n16, eq161_e2039_d_n17, eq161_e2039_d_n18, eq161_e2039_d_n19, eq161_e2039_d_n20, eq161_e2039_d_n21, eq161_e2039_d_n22,) = {
    if ((var_guard551 != 0.0) && (var_guard552 != 0.0)) {
        let eq161_e2035: f64 = (p.p252 * var_qg_fp2s);
        let eq161_e2035_d_n0: f64 = (p.p252 * var_qg_fp2s_dn0);
        let eq161_e2035_d_n1: f64 = (p.p252 * var_qg_fp2s_dn1);
        let eq161_e2035_d_n2: f64 = (p.p252 * var_qg_fp2s_dn2);
        let eq161_e2035_d_n3: f64 = (p.p252 * var_qg_fp2s_dn3);
        let eq161_e2035_d_n4: f64 = (p.p252 * var_qg_fp2s_dn4);
        let eq161_e2035_d_n5: f64 = (p.p252 * var_qg_fp2s_dn5);
        let eq161_e2035_d_n6: f64 = (p.p252 * var_qg_fp2s_dn6);
        let eq161_e2035_d_n7: f64 = (p.p252 * var_qg_fp2s_dn7);
        let eq161_e2035_d_n8: f64 = (p.p252 * var_qg_fp2s_dn8);
        let eq161_e2035_d_n9: f64 = (p.p252 * var_qg_fp2s_dn9);
        let eq161_e2035_d_n12: f64 = (p.p252 * var_qg_fp2s_dn12);
        let eq161_e2035_d_n14: f64 = (p.p252 * var_qg_fp2s_dn14);
        let eq161_e2035_d_n15: f64 = (p.p252 * var_qg_fp2s_dn15);
        let eq161_e2035_d_n16: f64 = (p.p252 * var_qg_fp2s_dn16);
        let eq161_e2035_d_n17: f64 = (p.p252 * var_qg_fp2s_dn17);
        let eq161_e2035_d_n18: f64 = (p.p252 * var_qg_fp2s_dn18);
        let eq161_e2035_d_n19: f64 = (p.p252 * var_qg_fp2s_dn19);
        let eq161_e2035_d_n20: f64 = (p.p252 * var_qg_fp2s_dn20);
        let eq161_e2035_d_n21: f64 = (p.p252 * var_qg_fp2s_dn21);
        let eq161_e2035_d_n22: f64 = (p.p252 * var_qg_fp2s_dn22);
        let eq161_e2036: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 60, eq161_e2035);
        let eq161_e2037: f64 = (p.p7 * eq161_e2036);
        let eq161_e2037_d_n0: f64 = (p.p7 * (eq161_e2035_d_n0 * ddt_scale));
        let eq161_e2037_d_n1: f64 = (p.p7 * (eq161_e2035_d_n1 * ddt_scale));
        let eq161_e2037_d_n2: f64 = (p.p7 * (eq161_e2035_d_n2 * ddt_scale));
        let eq161_e2037_d_n3: f64 = (p.p7 * (eq161_e2035_d_n3 * ddt_scale));
        let eq161_e2037_d_n4: f64 = (p.p7 * (eq161_e2035_d_n4 * ddt_scale));
        let eq161_e2037_d_n5: f64 = (p.p7 * (eq161_e2035_d_n5 * ddt_scale));
        let eq161_e2037_d_n6: f64 = (p.p7 * (eq161_e2035_d_n6 * ddt_scale));
        let eq161_e2037_d_n7: f64 = (p.p7 * (eq161_e2035_d_n7 * ddt_scale));
        let eq161_e2037_d_n8: f64 = (p.p7 * (eq161_e2035_d_n8 * ddt_scale));
        let eq161_e2037_d_n9: f64 = (p.p7 * (eq161_e2035_d_n9 * ddt_scale));
        let eq161_e2037_d_n12: f64 = (p.p7 * (eq161_e2035_d_n12 * ddt_scale));
        let eq161_e2037_d_n14: f64 = (p.p7 * (eq161_e2035_d_n14 * ddt_scale));
        let eq161_e2037_d_n15: f64 = (p.p7 * (eq161_e2035_d_n15 * ddt_scale));
        let eq161_e2037_d_n16: f64 = (p.p7 * (eq161_e2035_d_n16 * ddt_scale));
        let eq161_e2037_d_n17: f64 = (p.p7 * (eq161_e2035_d_n17 * ddt_scale));
        let eq161_e2037_d_n18: f64 = (p.p7 * (eq161_e2035_d_n18 * ddt_scale));
        let eq161_e2037_d_n19: f64 = (p.p7 * (eq161_e2035_d_n19 * ddt_scale));
        let eq161_e2037_d_n20: f64 = (p.p7 * (eq161_e2035_d_n20 * ddt_scale));
        let eq161_e2037_d_n21: f64 = (p.p7 * (eq161_e2035_d_n21 * ddt_scale));
        let eq161_e2037_d_n22: f64 = (p.p7 * (eq161_e2035_d_n22 * ddt_scale));
        (eq161_e2037, eq161_e2037_d_n0, eq161_e2037_d_n1, eq161_e2037_d_n2, eq161_e2037_d_n3, eq161_e2037_d_n4, eq161_e2037_d_n5, eq161_e2037_d_n6, eq161_e2037_d_n7, eq161_e2037_d_n8, eq161_e2037_d_n9, eq161_e2037_d_n12, eq161_e2037_d_n14, eq161_e2037_d_n15, eq161_e2037_d_n16, eq161_e2037_d_n17, eq161_e2037_d_n18, eq161_e2037_d_n19, eq161_e2037_d_n20, eq161_e2037_d_n21, eq161_e2037_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq161_value: f64 = eq161_e2039;
        let eq161_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq161_node_derivatives: [f64; 20] = [eq161_e2039_d_n0, eq161_e2039_d_n1, eq161_e2039_d_n2, eq161_e2039_d_n3, eq161_e2039_d_n4, eq161_e2039_d_n5, eq161_e2039_d_n6, eq161_e2039_d_n7, eq161_e2039_d_n8, eq161_e2039_d_n9, eq161_e2039_d_n12, eq161_e2039_d_n14, eq161_e2039_d_n15, eq161_e2039_d_n16, eq161_e2039_d_n17, eq161_e2039_d_n18, eq161_e2039_d_n19, eq161_e2039_d_n20, eq161_e2039_d_n21, eq161_e2039_d_n22];
        let eq161_branch_derivative_indices: [usize; 0] = [];
        let eq161_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(20),
            multiplicity * (eq161_value),
            &eq161_node_derivative_indices,
            &eq161_node_derivatives,
            &eq161_branch_derivative_indices,
            &eq161_branch_derivatives,
            multiplicity,
        );
        let (eq162_e2049, eq162_e2049_d_n0, eq162_e2049_d_n1, eq162_e2049_d_n2, eq162_e2049_d_n3, eq162_e2049_d_n4, eq162_e2049_d_n5, eq162_e2049_d_n6, eq162_e2049_d_n7, eq162_e2049_d_n8, eq162_e2049_d_n9, eq162_e2049_d_n12, eq162_e2049_d_n14, eq162_e2049_d_n15, eq162_e2049_d_n16, eq162_e2049_d_n17, eq162_e2049_d_n18, eq162_e2049_d_n19, eq162_e2049_d_n20, eq162_e2049_d_n21, eq162_e2049_d_n22,) = {
    if ((var_guard551 == 0.0) && (var_guard554 != 0.0)) {
        let eq162_e2046: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 61, var_qd_fp2s);
        let eq162_e2047: f64 = (p.p7 * eq162_e2046);
        let eq162_e2047_d_n0: f64 = (p.p7 * (var_qd_fp2s_dn0 * ddt_scale));
        let eq162_e2047_d_n1: f64 = (p.p7 * (var_qd_fp2s_dn1 * ddt_scale));
        let eq162_e2047_d_n2: f64 = (p.p7 * (var_qd_fp2s_dn2 * ddt_scale));
        let eq162_e2047_d_n3: f64 = (p.p7 * (var_qd_fp2s_dn3 * ddt_scale));
        let eq162_e2047_d_n4: f64 = (p.p7 * (var_qd_fp2s_dn4 * ddt_scale));
        let eq162_e2047_d_n5: f64 = (p.p7 * (var_qd_fp2s_dn5 * ddt_scale));
        let eq162_e2047_d_n6: f64 = (p.p7 * (var_qd_fp2s_dn6 * ddt_scale));
        let eq162_e2047_d_n7: f64 = (p.p7 * (var_qd_fp2s_dn7 * ddt_scale));
        let eq162_e2047_d_n8: f64 = (p.p7 * (var_qd_fp2s_dn8 * ddt_scale));
        let eq162_e2047_d_n9: f64 = (p.p7 * (var_qd_fp2s_dn9 * ddt_scale));
        let eq162_e2047_d_n12: f64 = (p.p7 * (var_qd_fp2s_dn12 * ddt_scale));
        let eq162_e2047_d_n14: f64 = (p.p7 * (var_qd_fp2s_dn14 * ddt_scale));
        let eq162_e2047_d_n15: f64 = (p.p7 * (var_qd_fp2s_dn15 * ddt_scale));
        let eq162_e2047_d_n16: f64 = (p.p7 * (var_qd_fp2s_dn16 * ddt_scale));
        let eq162_e2047_d_n17: f64 = (p.p7 * (var_qd_fp2s_dn17 * ddt_scale));
        let eq162_e2047_d_n18: f64 = (p.p7 * (var_qd_fp2s_dn18 * ddt_scale));
        let eq162_e2047_d_n19: f64 = (p.p7 * (var_qd_fp2s_dn19 * ddt_scale));
        let eq162_e2047_d_n20: f64 = (p.p7 * (var_qd_fp2s_dn20 * ddt_scale));
        let eq162_e2047_d_n21: f64 = (p.p7 * (var_qd_fp2s_dn21 * ddt_scale));
        let eq162_e2047_d_n22: f64 = (p.p7 * (var_qd_fp2s_dn22 * ddt_scale));
        (eq162_e2047, eq162_e2047_d_n0, eq162_e2047_d_n1, eq162_e2047_d_n2, eq162_e2047_d_n3, eq162_e2047_d_n4, eq162_e2047_d_n5, eq162_e2047_d_n6, eq162_e2047_d_n7, eq162_e2047_d_n8, eq162_e2047_d_n9, eq162_e2047_d_n12, eq162_e2047_d_n14, eq162_e2047_d_n15, eq162_e2047_d_n16, eq162_e2047_d_n17, eq162_e2047_d_n18, eq162_e2047_d_n19, eq162_e2047_d_n20, eq162_e2047_d_n21, eq162_e2047_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq162_value: f64 = eq162_e2049;
        let eq162_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq162_node_derivatives: [f64; 20] = [eq162_e2049_d_n0, eq162_e2049_d_n1, eq162_e2049_d_n2, eq162_e2049_d_n3, eq162_e2049_d_n4, eq162_e2049_d_n5, eq162_e2049_d_n6, eq162_e2049_d_n7, eq162_e2049_d_n8, eq162_e2049_d_n9, eq162_e2049_d_n12, eq162_e2049_d_n14, eq162_e2049_d_n15, eq162_e2049_d_n16, eq162_e2049_d_n17, eq162_e2049_d_n18, eq162_e2049_d_n19, eq162_e2049_d_n20, eq162_e2049_d_n21, eq162_e2049_d_n22];
        let eq162_branch_derivative_indices: [usize; 0] = [];
        let eq162_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(2),
            multiplicity * (eq162_value),
            &eq162_node_derivative_indices,
            &eq162_node_derivatives,
            &eq162_branch_derivative_indices,
            &eq162_branch_derivatives,
            multiplicity,
        );
        let (eq163_e2061, eq163_e2061_d_n0, eq163_e2061_d_n1, eq163_e2061_d_n2, eq163_e2061_d_n3, eq163_e2061_d_n4, eq163_e2061_d_n5, eq163_e2061_d_n6, eq163_e2061_d_n7, eq163_e2061_d_n8, eq163_e2061_d_n9, eq163_e2061_d_n12, eq163_e2061_d_n14, eq163_e2061_d_n15, eq163_e2061_d_n16, eq163_e2061_d_n17, eq163_e2061_d_n18, eq163_e2061_d_n19, eq163_e2061_d_n20, eq163_e2061_d_n21, eq163_e2061_d_n22,) = {
    if (((var_guard551 == 0.0) && (var_guard554 != 0.0)) && (var_guard555 != 0.0)) {
        let eq163_e2058: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 62, var_qg_fp2s);
        let eq163_e2059: f64 = (p.p7 * eq163_e2058);
        let eq163_e2059_d_n0: f64 = (p.p7 * (var_qg_fp2s_dn0 * ddt_scale));
        let eq163_e2059_d_n1: f64 = (p.p7 * (var_qg_fp2s_dn1 * ddt_scale));
        let eq163_e2059_d_n2: f64 = (p.p7 * (var_qg_fp2s_dn2 * ddt_scale));
        let eq163_e2059_d_n3: f64 = (p.p7 * (var_qg_fp2s_dn3 * ddt_scale));
        let eq163_e2059_d_n4: f64 = (p.p7 * (var_qg_fp2s_dn4 * ddt_scale));
        let eq163_e2059_d_n5: f64 = (p.p7 * (var_qg_fp2s_dn5 * ddt_scale));
        let eq163_e2059_d_n6: f64 = (p.p7 * (var_qg_fp2s_dn6 * ddt_scale));
        let eq163_e2059_d_n7: f64 = (p.p7 * (var_qg_fp2s_dn7 * ddt_scale));
        let eq163_e2059_d_n8: f64 = (p.p7 * (var_qg_fp2s_dn8 * ddt_scale));
        let eq163_e2059_d_n9: f64 = (p.p7 * (var_qg_fp2s_dn9 * ddt_scale));
        let eq163_e2059_d_n12: f64 = (p.p7 * (var_qg_fp2s_dn12 * ddt_scale));
        let eq163_e2059_d_n14: f64 = (p.p7 * (var_qg_fp2s_dn14 * ddt_scale));
        let eq163_e2059_d_n15: f64 = (p.p7 * (var_qg_fp2s_dn15 * ddt_scale));
        let eq163_e2059_d_n16: f64 = (p.p7 * (var_qg_fp2s_dn16 * ddt_scale));
        let eq163_e2059_d_n17: f64 = (p.p7 * (var_qg_fp2s_dn17 * ddt_scale));
        let eq163_e2059_d_n18: f64 = (p.p7 * (var_qg_fp2s_dn18 * ddt_scale));
        let eq163_e2059_d_n19: f64 = (p.p7 * (var_qg_fp2s_dn19 * ddt_scale));
        let eq163_e2059_d_n20: f64 = (p.p7 * (var_qg_fp2s_dn20 * ddt_scale));
        let eq163_e2059_d_n21: f64 = (p.p7 * (var_qg_fp2s_dn21 * ddt_scale));
        let eq163_e2059_d_n22: f64 = (p.p7 * (var_qg_fp2s_dn22 * ddt_scale));
        (eq163_e2059, eq163_e2059_d_n0, eq163_e2059_d_n1, eq163_e2059_d_n2, eq163_e2059_d_n3, eq163_e2059_d_n4, eq163_e2059_d_n5, eq163_e2059_d_n6, eq163_e2059_d_n7, eq163_e2059_d_n8, eq163_e2059_d_n9, eq163_e2059_d_n12, eq163_e2059_d_n14, eq163_e2059_d_n15, eq163_e2059_d_n16, eq163_e2059_d_n17, eq163_e2059_d_n18, eq163_e2059_d_n19, eq163_e2059_d_n20, eq163_e2059_d_n21, eq163_e2059_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq163_value: f64 = eq163_e2061;
        let eq163_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq163_node_derivatives: [f64; 20] = [eq163_e2061_d_n0, eq163_e2061_d_n1, eq163_e2061_d_n2, eq163_e2061_d_n3, eq163_e2061_d_n4, eq163_e2061_d_n5, eq163_e2061_d_n6, eq163_e2061_d_n7, eq163_e2061_d_n8, eq163_e2061_d_n9, eq163_e2061_d_n12, eq163_e2061_d_n14, eq163_e2061_d_n15, eq163_e2061_d_n16, eq163_e2061_d_n17, eq163_e2061_d_n18, eq163_e2061_d_n19, eq163_e2061_d_n20, eq163_e2061_d_n21, eq163_e2061_d_n22];
        let eq163_branch_derivative_indices: [usize; 0] = [];
        let eq163_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq163_value),
            &eq163_node_derivative_indices,
            &eq163_node_derivatives,
            &eq163_branch_derivative_indices,
            &eq163_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_8(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
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
        var_guard551: f64,
        var_guard554: f64,
        var_guard555: f64,
        var_guard556: f64,
        var_guard557: f64,
        var_guard558: f64,
        var_qd_fp3: f64,
        var_qd_fp3_dn0: f64,
        var_qd_fp3_dn1: f64,
        var_qd_fp3_dn12: f64,
        var_qd_fp3_dn14: f64,
        var_qd_fp3_dn15: f64,
        var_qd_fp3_dn16: f64,
        var_qd_fp3_dn17: f64,
        var_qd_fp3_dn18: f64,
        var_qd_fp3_dn19: f64,
        var_qd_fp3_dn2: f64,
        var_qd_fp3_dn20: f64,
        var_qd_fp3_dn21: f64,
        var_qd_fp3_dn22: f64,
        var_qd_fp3_dn3: f64,
        var_qd_fp3_dn4: f64,
        var_qd_fp3_dn5: f64,
        var_qd_fp3_dn6: f64,
        var_qd_fp3_dn7: f64,
        var_qd_fp3_dn8: f64,
        var_qd_fp3_dn9: f64,
        var_qg_fp2s: f64,
        var_qg_fp2s_dn0: f64,
        var_qg_fp2s_dn1: f64,
        var_qg_fp2s_dn12: f64,
        var_qg_fp2s_dn14: f64,
        var_qg_fp2s_dn15: f64,
        var_qg_fp2s_dn16: f64,
        var_qg_fp2s_dn17: f64,
        var_qg_fp2s_dn18: f64,
        var_qg_fp2s_dn19: f64,
        var_qg_fp2s_dn2: f64,
        var_qg_fp2s_dn20: f64,
        var_qg_fp2s_dn21: f64,
        var_qg_fp2s_dn22: f64,
        var_qg_fp2s_dn3: f64,
        var_qg_fp2s_dn4: f64,
        var_qg_fp2s_dn5: f64,
        var_qg_fp2s_dn6: f64,
        var_qg_fp2s_dn7: f64,
        var_qg_fp2s_dn8: f64,
        var_qg_fp2s_dn9: f64,
        var_qg_fp3: f64,
        var_qg_fp3_dn0: f64,
        var_qg_fp3_dn1: f64,
        var_qg_fp3_dn12: f64,
        var_qg_fp3_dn14: f64,
        var_qg_fp3_dn15: f64,
        var_qg_fp3_dn16: f64,
        var_qg_fp3_dn17: f64,
        var_qg_fp3_dn18: f64,
        var_qg_fp3_dn19: f64,
        var_qg_fp3_dn2: f64,
        var_qg_fp3_dn20: f64,
        var_qg_fp3_dn21: f64,
        var_qg_fp3_dn22: f64,
        var_qg_fp3_dn3: f64,
        var_qg_fp3_dn4: f64,
        var_qg_fp3_dn5: f64,
        var_qg_fp3_dn6: f64,
        var_qg_fp3_dn7: f64,
        var_qg_fp3_dn8: f64,
        var_qg_fp3_dn9: f64,
    ) {
        let (eq164_e2075, eq164_e2075_d_n0, eq164_e2075_d_n1, eq164_e2075_d_n2, eq164_e2075_d_n3, eq164_e2075_d_n4, eq164_e2075_d_n5, eq164_e2075_d_n6, eq164_e2075_d_n7, eq164_e2075_d_n8, eq164_e2075_d_n9, eq164_e2075_d_n12, eq164_e2075_d_n14, eq164_e2075_d_n15, eq164_e2075_d_n16, eq164_e2075_d_n17, eq164_e2075_d_n18, eq164_e2075_d_n19, eq164_e2075_d_n20, eq164_e2075_d_n21, eq164_e2075_d_n22,) = {
    if (((var_guard551 == 0.0) && (var_guard554 != 0.0)) && (var_guard555 != 0.0)) {
        let eq164_e2070: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 63, var_qg_fp2s);
        let eq164_e2071: f64 = (p.p7 * eq164_e2070);
        let eq164_e2071_d_n0: f64 = (p.p7 * (var_qg_fp2s_dn0 * ddt_scale));
        let eq164_e2071_d_n1: f64 = (p.p7 * (var_qg_fp2s_dn1 * ddt_scale));
        let eq164_e2071_d_n2: f64 = (p.p7 * (var_qg_fp2s_dn2 * ddt_scale));
        let eq164_e2071_d_n3: f64 = (p.p7 * (var_qg_fp2s_dn3 * ddt_scale));
        let eq164_e2071_d_n4: f64 = (p.p7 * (var_qg_fp2s_dn4 * ddt_scale));
        let eq164_e2071_d_n5: f64 = (p.p7 * (var_qg_fp2s_dn5 * ddt_scale));
        let eq164_e2071_d_n6: f64 = (p.p7 * (var_qg_fp2s_dn6 * ddt_scale));
        let eq164_e2071_d_n7: f64 = (p.p7 * (var_qg_fp2s_dn7 * ddt_scale));
        let eq164_e2071_d_n8: f64 = (p.p7 * (var_qg_fp2s_dn8 * ddt_scale));
        let eq164_e2071_d_n9: f64 = (p.p7 * (var_qg_fp2s_dn9 * ddt_scale));
        let eq164_e2071_d_n12: f64 = (p.p7 * (var_qg_fp2s_dn12 * ddt_scale));
        let eq164_e2071_d_n14: f64 = (p.p7 * (var_qg_fp2s_dn14 * ddt_scale));
        let eq164_e2071_d_n15: f64 = (p.p7 * (var_qg_fp2s_dn15 * ddt_scale));
        let eq164_e2071_d_n16: f64 = (p.p7 * (var_qg_fp2s_dn16 * ddt_scale));
        let eq164_e2071_d_n17: f64 = (p.p7 * (var_qg_fp2s_dn17 * ddt_scale));
        let eq164_e2071_d_n18: f64 = (p.p7 * (var_qg_fp2s_dn18 * ddt_scale));
        let eq164_e2071_d_n19: f64 = (p.p7 * (var_qg_fp2s_dn19 * ddt_scale));
        let eq164_e2071_d_n20: f64 = (p.p7 * (var_qg_fp2s_dn20 * ddt_scale));
        let eq164_e2071_d_n21: f64 = (p.p7 * (var_qg_fp2s_dn21 * ddt_scale));
        let eq164_e2071_d_n22: f64 = (p.p7 * (var_qg_fp2s_dn22 * ddt_scale));
        let eq164_e2073: f64 = (eq164_e2071 * p.p247);
        let eq164_e2073_d_n0: f64 = (eq164_e2071_d_n0 * p.p247);
        let eq164_e2073_d_n1: f64 = (eq164_e2071_d_n1 * p.p247);
        let eq164_e2073_d_n2: f64 = (eq164_e2071_d_n2 * p.p247);
        let eq164_e2073_d_n3: f64 = (eq164_e2071_d_n3 * p.p247);
        let eq164_e2073_d_n4: f64 = (eq164_e2071_d_n4 * p.p247);
        let eq164_e2073_d_n5: f64 = (eq164_e2071_d_n5 * p.p247);
        let eq164_e2073_d_n6: f64 = (eq164_e2071_d_n6 * p.p247);
        let eq164_e2073_d_n7: f64 = (eq164_e2071_d_n7 * p.p247);
        let eq164_e2073_d_n8: f64 = (eq164_e2071_d_n8 * p.p247);
        let eq164_e2073_d_n9: f64 = (eq164_e2071_d_n9 * p.p247);
        let eq164_e2073_d_n12: f64 = (eq164_e2071_d_n12 * p.p247);
        let eq164_e2073_d_n14: f64 = (eq164_e2071_d_n14 * p.p247);
        let eq164_e2073_d_n15: f64 = (eq164_e2071_d_n15 * p.p247);
        let eq164_e2073_d_n16: f64 = (eq164_e2071_d_n16 * p.p247);
        let eq164_e2073_d_n17: f64 = (eq164_e2071_d_n17 * p.p247);
        let eq164_e2073_d_n18: f64 = (eq164_e2071_d_n18 * p.p247);
        let eq164_e2073_d_n19: f64 = (eq164_e2071_d_n19 * p.p247);
        let eq164_e2073_d_n20: f64 = (eq164_e2071_d_n20 * p.p247);
        let eq164_e2073_d_n21: f64 = (eq164_e2071_d_n21 * p.p247);
        let eq164_e2073_d_n22: f64 = (eq164_e2071_d_n22 * p.p247);
        (eq164_e2073, eq164_e2073_d_n0, eq164_e2073_d_n1, eq164_e2073_d_n2, eq164_e2073_d_n3, eq164_e2073_d_n4, eq164_e2073_d_n5, eq164_e2073_d_n6, eq164_e2073_d_n7, eq164_e2073_d_n8, eq164_e2073_d_n9, eq164_e2073_d_n12, eq164_e2073_d_n14, eq164_e2073_d_n15, eq164_e2073_d_n16, eq164_e2073_d_n17, eq164_e2073_d_n18, eq164_e2073_d_n19, eq164_e2073_d_n20, eq164_e2073_d_n21, eq164_e2073_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq164_value: f64 = eq164_e2075;
        let eq164_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq164_node_derivatives: [f64; 20] = [eq164_e2075_d_n0, eq164_e2075_d_n1, eq164_e2075_d_n2, eq164_e2075_d_n3, eq164_e2075_d_n4, eq164_e2075_d_n5, eq164_e2075_d_n6, eq164_e2075_d_n7, eq164_e2075_d_n8, eq164_e2075_d_n9, eq164_e2075_d_n12, eq164_e2075_d_n14, eq164_e2075_d_n15, eq164_e2075_d_n16, eq164_e2075_d_n17, eq164_e2075_d_n18, eq164_e2075_d_n19, eq164_e2075_d_n20, eq164_e2075_d_n21, eq164_e2075_d_n22];
        let eq164_branch_derivative_indices: [usize; 0] = [];
        let eq164_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq164_value),
            &eq164_node_derivative_indices,
            &eq164_node_derivatives,
            &eq164_branch_derivative_indices,
            &eq164_branch_derivatives,
            multiplicity,
        );
        let (eq165_e2088, eq165_e2088_d_n0, eq165_e2088_d_n1, eq165_e2088_d_n2, eq165_e2088_d_n3, eq165_e2088_d_n4, eq165_e2088_d_n5, eq165_e2088_d_n6, eq165_e2088_d_n7, eq165_e2088_d_n8, eq165_e2088_d_n9, eq165_e2088_d_n12, eq165_e2088_d_n14, eq165_e2088_d_n15, eq165_e2088_d_n16, eq165_e2088_d_n17, eq165_e2088_d_n18, eq165_e2088_d_n19, eq165_e2088_d_n20, eq165_e2088_d_n21, eq165_e2088_d_n22,) = {
    if (((var_guard551 == 0.0) && (var_guard554 != 0.0)) && (var_guard555 == 0.0)) {
        let eq165_e2085: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 64, var_qg_fp2s);
        let eq165_e2086: f64 = (p.p7 * eq165_e2085);
        let eq165_e2086_d_n0: f64 = (p.p7 * (var_qg_fp2s_dn0 * ddt_scale));
        let eq165_e2086_d_n1: f64 = (p.p7 * (var_qg_fp2s_dn1 * ddt_scale));
        let eq165_e2086_d_n2: f64 = (p.p7 * (var_qg_fp2s_dn2 * ddt_scale));
        let eq165_e2086_d_n3: f64 = (p.p7 * (var_qg_fp2s_dn3 * ddt_scale));
        let eq165_e2086_d_n4: f64 = (p.p7 * (var_qg_fp2s_dn4 * ddt_scale));
        let eq165_e2086_d_n5: f64 = (p.p7 * (var_qg_fp2s_dn5 * ddt_scale));
        let eq165_e2086_d_n6: f64 = (p.p7 * (var_qg_fp2s_dn6 * ddt_scale));
        let eq165_e2086_d_n7: f64 = (p.p7 * (var_qg_fp2s_dn7 * ddt_scale));
        let eq165_e2086_d_n8: f64 = (p.p7 * (var_qg_fp2s_dn8 * ddt_scale));
        let eq165_e2086_d_n9: f64 = (p.p7 * (var_qg_fp2s_dn9 * ddt_scale));
        let eq165_e2086_d_n12: f64 = (p.p7 * (var_qg_fp2s_dn12 * ddt_scale));
        let eq165_e2086_d_n14: f64 = (p.p7 * (var_qg_fp2s_dn14 * ddt_scale));
        let eq165_e2086_d_n15: f64 = (p.p7 * (var_qg_fp2s_dn15 * ddt_scale));
        let eq165_e2086_d_n16: f64 = (p.p7 * (var_qg_fp2s_dn16 * ddt_scale));
        let eq165_e2086_d_n17: f64 = (p.p7 * (var_qg_fp2s_dn17 * ddt_scale));
        let eq165_e2086_d_n18: f64 = (p.p7 * (var_qg_fp2s_dn18 * ddt_scale));
        let eq165_e2086_d_n19: f64 = (p.p7 * (var_qg_fp2s_dn19 * ddt_scale));
        let eq165_e2086_d_n20: f64 = (p.p7 * (var_qg_fp2s_dn20 * ddt_scale));
        let eq165_e2086_d_n21: f64 = (p.p7 * (var_qg_fp2s_dn21 * ddt_scale));
        let eq165_e2086_d_n22: f64 = (p.p7 * (var_qg_fp2s_dn22 * ddt_scale));
        (eq165_e2086, eq165_e2086_d_n0, eq165_e2086_d_n1, eq165_e2086_d_n2, eq165_e2086_d_n3, eq165_e2086_d_n4, eq165_e2086_d_n5, eq165_e2086_d_n6, eq165_e2086_d_n7, eq165_e2086_d_n8, eq165_e2086_d_n9, eq165_e2086_d_n12, eq165_e2086_d_n14, eq165_e2086_d_n15, eq165_e2086_d_n16, eq165_e2086_d_n17, eq165_e2086_d_n18, eq165_e2086_d_n19, eq165_e2086_d_n20, eq165_e2086_d_n21, eq165_e2086_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq165_value: f64 = eq165_e2088;
        let eq165_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq165_node_derivatives: [f64; 20] = [eq165_e2088_d_n0, eq165_e2088_d_n1, eq165_e2088_d_n2, eq165_e2088_d_n3, eq165_e2088_d_n4, eq165_e2088_d_n5, eq165_e2088_d_n6, eq165_e2088_d_n7, eq165_e2088_d_n8, eq165_e2088_d_n9, eq165_e2088_d_n12, eq165_e2088_d_n14, eq165_e2088_d_n15, eq165_e2088_d_n16, eq165_e2088_d_n17, eq165_e2088_d_n18, eq165_e2088_d_n19, eq165_e2088_d_n20, eq165_e2088_d_n21, eq165_e2088_d_n22];
        let eq165_branch_derivative_indices: [usize; 0] = [];
        let eq165_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq165_value),
            &eq165_node_derivative_indices,
            &eq165_node_derivatives,
            &eq165_branch_derivative_indices,
            &eq165_branch_derivatives,
            multiplicity,
        );
        let (eq166_e2103, eq166_e2103_d_n0, eq166_e2103_d_n1, eq166_e2103_d_n2, eq166_e2103_d_n3, eq166_e2103_d_n4, eq166_e2103_d_n5, eq166_e2103_d_n6, eq166_e2103_d_n7, eq166_e2103_d_n8, eq166_e2103_d_n9, eq166_e2103_d_n12, eq166_e2103_d_n14, eq166_e2103_d_n15, eq166_e2103_d_n16, eq166_e2103_d_n17, eq166_e2103_d_n18, eq166_e2103_d_n19, eq166_e2103_d_n20, eq166_e2103_d_n21, eq166_e2103_d_n22,) = {
    if (((var_guard551 == 0.0) && (var_guard554 != 0.0)) && (var_guard555 == 0.0)) {
        let eq166_e2098: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 65, var_qg_fp2s);
        let eq166_e2099: f64 = (p.p7 * eq166_e2098);
        let eq166_e2099_d_n0: f64 = (p.p7 * (var_qg_fp2s_dn0 * ddt_scale));
        let eq166_e2099_d_n1: f64 = (p.p7 * (var_qg_fp2s_dn1 * ddt_scale));
        let eq166_e2099_d_n2: f64 = (p.p7 * (var_qg_fp2s_dn2 * ddt_scale));
        let eq166_e2099_d_n3: f64 = (p.p7 * (var_qg_fp2s_dn3 * ddt_scale));
        let eq166_e2099_d_n4: f64 = (p.p7 * (var_qg_fp2s_dn4 * ddt_scale));
        let eq166_e2099_d_n5: f64 = (p.p7 * (var_qg_fp2s_dn5 * ddt_scale));
        let eq166_e2099_d_n6: f64 = (p.p7 * (var_qg_fp2s_dn6 * ddt_scale));
        let eq166_e2099_d_n7: f64 = (p.p7 * (var_qg_fp2s_dn7 * ddt_scale));
        let eq166_e2099_d_n8: f64 = (p.p7 * (var_qg_fp2s_dn8 * ddt_scale));
        let eq166_e2099_d_n9: f64 = (p.p7 * (var_qg_fp2s_dn9 * ddt_scale));
        let eq166_e2099_d_n12: f64 = (p.p7 * (var_qg_fp2s_dn12 * ddt_scale));
        let eq166_e2099_d_n14: f64 = (p.p7 * (var_qg_fp2s_dn14 * ddt_scale));
        let eq166_e2099_d_n15: f64 = (p.p7 * (var_qg_fp2s_dn15 * ddt_scale));
        let eq166_e2099_d_n16: f64 = (p.p7 * (var_qg_fp2s_dn16 * ddt_scale));
        let eq166_e2099_d_n17: f64 = (p.p7 * (var_qg_fp2s_dn17 * ddt_scale));
        let eq166_e2099_d_n18: f64 = (p.p7 * (var_qg_fp2s_dn18 * ddt_scale));
        let eq166_e2099_d_n19: f64 = (p.p7 * (var_qg_fp2s_dn19 * ddt_scale));
        let eq166_e2099_d_n20: f64 = (p.p7 * (var_qg_fp2s_dn20 * ddt_scale));
        let eq166_e2099_d_n21: f64 = (p.p7 * (var_qg_fp2s_dn21 * ddt_scale));
        let eq166_e2099_d_n22: f64 = (p.p7 * (var_qg_fp2s_dn22 * ddt_scale));
        let eq166_e2101: f64 = (eq166_e2099 * p.p247);
        let eq166_e2101_d_n0: f64 = (eq166_e2099_d_n0 * p.p247);
        let eq166_e2101_d_n1: f64 = (eq166_e2099_d_n1 * p.p247);
        let eq166_e2101_d_n2: f64 = (eq166_e2099_d_n2 * p.p247);
        let eq166_e2101_d_n3: f64 = (eq166_e2099_d_n3 * p.p247);
        let eq166_e2101_d_n4: f64 = (eq166_e2099_d_n4 * p.p247);
        let eq166_e2101_d_n5: f64 = (eq166_e2099_d_n5 * p.p247);
        let eq166_e2101_d_n6: f64 = (eq166_e2099_d_n6 * p.p247);
        let eq166_e2101_d_n7: f64 = (eq166_e2099_d_n7 * p.p247);
        let eq166_e2101_d_n8: f64 = (eq166_e2099_d_n8 * p.p247);
        let eq166_e2101_d_n9: f64 = (eq166_e2099_d_n9 * p.p247);
        let eq166_e2101_d_n12: f64 = (eq166_e2099_d_n12 * p.p247);
        let eq166_e2101_d_n14: f64 = (eq166_e2099_d_n14 * p.p247);
        let eq166_e2101_d_n15: f64 = (eq166_e2099_d_n15 * p.p247);
        let eq166_e2101_d_n16: f64 = (eq166_e2099_d_n16 * p.p247);
        let eq166_e2101_d_n17: f64 = (eq166_e2099_d_n17 * p.p247);
        let eq166_e2101_d_n18: f64 = (eq166_e2099_d_n18 * p.p247);
        let eq166_e2101_d_n19: f64 = (eq166_e2099_d_n19 * p.p247);
        let eq166_e2101_d_n20: f64 = (eq166_e2099_d_n20 * p.p247);
        let eq166_e2101_d_n21: f64 = (eq166_e2099_d_n21 * p.p247);
        let eq166_e2101_d_n22: f64 = (eq166_e2099_d_n22 * p.p247);
        (eq166_e2101, eq166_e2101_d_n0, eq166_e2101_d_n1, eq166_e2101_d_n2, eq166_e2101_d_n3, eq166_e2101_d_n4, eq166_e2101_d_n5, eq166_e2101_d_n6, eq166_e2101_d_n7, eq166_e2101_d_n8, eq166_e2101_d_n9, eq166_e2101_d_n12, eq166_e2101_d_n14, eq166_e2101_d_n15, eq166_e2101_d_n16, eq166_e2101_d_n17, eq166_e2101_d_n18, eq166_e2101_d_n19, eq166_e2101_d_n20, eq166_e2101_d_n21, eq166_e2101_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq166_value: f64 = eq166_e2103;
        let eq166_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq166_node_derivatives: [f64; 20] = [eq166_e2103_d_n0, eq166_e2103_d_n1, eq166_e2103_d_n2, eq166_e2103_d_n3, eq166_e2103_d_n4, eq166_e2103_d_n5, eq166_e2103_d_n6, eq166_e2103_d_n7, eq166_e2103_d_n8, eq166_e2103_d_n9, eq166_e2103_d_n12, eq166_e2103_d_n14, eq166_e2103_d_n15, eq166_e2103_d_n16, eq166_e2103_d_n17, eq166_e2103_d_n18, eq166_e2103_d_n19, eq166_e2103_d_n20, eq166_e2103_d_n21, eq166_e2103_d_n22];
        let eq166_branch_derivative_indices: [usize; 0] = [];
        let eq166_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq166_value),
            &eq166_node_derivative_indices,
            &eq166_node_derivatives,
            &eq166_branch_derivative_indices,
            &eq166_branch_derivatives,
            multiplicity,
        );
        let (eq167_e2115, eq167_e2115_d_n0, eq167_e2115_d_n1, eq167_e2115_d_n2, eq167_e2115_d_n3, eq167_e2115_d_n4, eq167_e2115_d_n5, eq167_e2115_d_n6, eq167_e2115_d_n7, eq167_e2115_d_n8, eq167_e2115_d_n9, eq167_e2115_d_n12, eq167_e2115_d_n14, eq167_e2115_d_n15, eq167_e2115_d_n16, eq167_e2115_d_n17, eq167_e2115_d_n18, eq167_e2115_d_n19, eq167_e2115_d_n20, eq167_e2115_d_n21, eq167_e2115_d_n22,) = {
    if ((var_guard551 == 0.0) && (var_guard554 != 0.0)) {
        let eq167_e2111: f64 = (p.p252 * var_qg_fp2s);
        let eq167_e2111_d_n0: f64 = (p.p252 * var_qg_fp2s_dn0);
        let eq167_e2111_d_n1: f64 = (p.p252 * var_qg_fp2s_dn1);
        let eq167_e2111_d_n2: f64 = (p.p252 * var_qg_fp2s_dn2);
        let eq167_e2111_d_n3: f64 = (p.p252 * var_qg_fp2s_dn3);
        let eq167_e2111_d_n4: f64 = (p.p252 * var_qg_fp2s_dn4);
        let eq167_e2111_d_n5: f64 = (p.p252 * var_qg_fp2s_dn5);
        let eq167_e2111_d_n6: f64 = (p.p252 * var_qg_fp2s_dn6);
        let eq167_e2111_d_n7: f64 = (p.p252 * var_qg_fp2s_dn7);
        let eq167_e2111_d_n8: f64 = (p.p252 * var_qg_fp2s_dn8);
        let eq167_e2111_d_n9: f64 = (p.p252 * var_qg_fp2s_dn9);
        let eq167_e2111_d_n12: f64 = (p.p252 * var_qg_fp2s_dn12);
        let eq167_e2111_d_n14: f64 = (p.p252 * var_qg_fp2s_dn14);
        let eq167_e2111_d_n15: f64 = (p.p252 * var_qg_fp2s_dn15);
        let eq167_e2111_d_n16: f64 = (p.p252 * var_qg_fp2s_dn16);
        let eq167_e2111_d_n17: f64 = (p.p252 * var_qg_fp2s_dn17);
        let eq167_e2111_d_n18: f64 = (p.p252 * var_qg_fp2s_dn18);
        let eq167_e2111_d_n19: f64 = (p.p252 * var_qg_fp2s_dn19);
        let eq167_e2111_d_n20: f64 = (p.p252 * var_qg_fp2s_dn20);
        let eq167_e2111_d_n21: f64 = (p.p252 * var_qg_fp2s_dn21);
        let eq167_e2111_d_n22: f64 = (p.p252 * var_qg_fp2s_dn22);
        let eq167_e2112: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 66, eq167_e2111);
        let eq167_e2113: f64 = (p.p7 * eq167_e2112);
        let eq167_e2113_d_n0: f64 = (p.p7 * (eq167_e2111_d_n0 * ddt_scale));
        let eq167_e2113_d_n1: f64 = (p.p7 * (eq167_e2111_d_n1 * ddt_scale));
        let eq167_e2113_d_n2: f64 = (p.p7 * (eq167_e2111_d_n2 * ddt_scale));
        let eq167_e2113_d_n3: f64 = (p.p7 * (eq167_e2111_d_n3 * ddt_scale));
        let eq167_e2113_d_n4: f64 = (p.p7 * (eq167_e2111_d_n4 * ddt_scale));
        let eq167_e2113_d_n5: f64 = (p.p7 * (eq167_e2111_d_n5 * ddt_scale));
        let eq167_e2113_d_n6: f64 = (p.p7 * (eq167_e2111_d_n6 * ddt_scale));
        let eq167_e2113_d_n7: f64 = (p.p7 * (eq167_e2111_d_n7 * ddt_scale));
        let eq167_e2113_d_n8: f64 = (p.p7 * (eq167_e2111_d_n8 * ddt_scale));
        let eq167_e2113_d_n9: f64 = (p.p7 * (eq167_e2111_d_n9 * ddt_scale));
        let eq167_e2113_d_n12: f64 = (p.p7 * (eq167_e2111_d_n12 * ddt_scale));
        let eq167_e2113_d_n14: f64 = (p.p7 * (eq167_e2111_d_n14 * ddt_scale));
        let eq167_e2113_d_n15: f64 = (p.p7 * (eq167_e2111_d_n15 * ddt_scale));
        let eq167_e2113_d_n16: f64 = (p.p7 * (eq167_e2111_d_n16 * ddt_scale));
        let eq167_e2113_d_n17: f64 = (p.p7 * (eq167_e2111_d_n17 * ddt_scale));
        let eq167_e2113_d_n18: f64 = (p.p7 * (eq167_e2111_d_n18 * ddt_scale));
        let eq167_e2113_d_n19: f64 = (p.p7 * (eq167_e2111_d_n19 * ddt_scale));
        let eq167_e2113_d_n20: f64 = (p.p7 * (eq167_e2111_d_n20 * ddt_scale));
        let eq167_e2113_d_n21: f64 = (p.p7 * (eq167_e2111_d_n21 * ddt_scale));
        let eq167_e2113_d_n22: f64 = (p.p7 * (eq167_e2111_d_n22 * ddt_scale));
        (eq167_e2113, eq167_e2113_d_n0, eq167_e2113_d_n1, eq167_e2113_d_n2, eq167_e2113_d_n3, eq167_e2113_d_n4, eq167_e2113_d_n5, eq167_e2113_d_n6, eq167_e2113_d_n7, eq167_e2113_d_n8, eq167_e2113_d_n9, eq167_e2113_d_n12, eq167_e2113_d_n14, eq167_e2113_d_n15, eq167_e2113_d_n16, eq167_e2113_d_n17, eq167_e2113_d_n18, eq167_e2113_d_n19, eq167_e2113_d_n20, eq167_e2113_d_n21, eq167_e2113_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq167_value: f64 = eq167_e2115;
        let eq167_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq167_node_derivatives: [f64; 20] = [eq167_e2115_d_n0, eq167_e2115_d_n1, eq167_e2115_d_n2, eq167_e2115_d_n3, eq167_e2115_d_n4, eq167_e2115_d_n5, eq167_e2115_d_n6, eq167_e2115_d_n7, eq167_e2115_d_n8, eq167_e2115_d_n9, eq167_e2115_d_n12, eq167_e2115_d_n14, eq167_e2115_d_n15, eq167_e2115_d_n16, eq167_e2115_d_n17, eq167_e2115_d_n18, eq167_e2115_d_n19, eq167_e2115_d_n20, eq167_e2115_d_n21, eq167_e2115_d_n22];
        let eq167_branch_derivative_indices: [usize; 0] = [];
        let eq167_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(8),
            multiplicity * (eq167_value),
            &eq167_node_derivative_indices,
            &eq167_node_derivatives,
            &eq167_branch_derivative_indices,
            &eq167_branch_derivatives,
            multiplicity,
        );
        let (eq168_e2124, eq168_e2124_d_n0, eq168_e2124_d_n1, eq168_e2124_d_n2, eq168_e2124_d_n3, eq168_e2124_d_n4, eq168_e2124_d_n5, eq168_e2124_d_n6, eq168_e2124_d_n7, eq168_e2124_d_n8, eq168_e2124_d_n9, eq168_e2124_d_n12, eq168_e2124_d_n14, eq168_e2124_d_n15, eq168_e2124_d_n16, eq168_e2124_d_n17, eq168_e2124_d_n18, eq168_e2124_d_n19, eq168_e2124_d_n20, eq168_e2124_d_n21, eq168_e2124_d_n22,) = {
    if ((var_guard556 != 0.0) && (var_guard557 != 0.0)) {
        let eq168_e2121: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 67, var_qd_fp3);
        let eq168_e2122: f64 = (p.p7 * eq168_e2121);
        let eq168_e2122_d_n0: f64 = (p.p7 * (var_qd_fp3_dn0 * ddt_scale));
        let eq168_e2122_d_n1: f64 = (p.p7 * (var_qd_fp3_dn1 * ddt_scale));
        let eq168_e2122_d_n2: f64 = (p.p7 * (var_qd_fp3_dn2 * ddt_scale));
        let eq168_e2122_d_n3: f64 = (p.p7 * (var_qd_fp3_dn3 * ddt_scale));
        let eq168_e2122_d_n4: f64 = (p.p7 * (var_qd_fp3_dn4 * ddt_scale));
        let eq168_e2122_d_n5: f64 = (p.p7 * (var_qd_fp3_dn5 * ddt_scale));
        let eq168_e2122_d_n6: f64 = (p.p7 * (var_qd_fp3_dn6 * ddt_scale));
        let eq168_e2122_d_n7: f64 = (p.p7 * (var_qd_fp3_dn7 * ddt_scale));
        let eq168_e2122_d_n8: f64 = (p.p7 * (var_qd_fp3_dn8 * ddt_scale));
        let eq168_e2122_d_n9: f64 = (p.p7 * (var_qd_fp3_dn9 * ddt_scale));
        let eq168_e2122_d_n12: f64 = (p.p7 * (var_qd_fp3_dn12 * ddt_scale));
        let eq168_e2122_d_n14: f64 = (p.p7 * (var_qd_fp3_dn14 * ddt_scale));
        let eq168_e2122_d_n15: f64 = (p.p7 * (var_qd_fp3_dn15 * ddt_scale));
        let eq168_e2122_d_n16: f64 = (p.p7 * (var_qd_fp3_dn16 * ddt_scale));
        let eq168_e2122_d_n17: f64 = (p.p7 * (var_qd_fp3_dn17 * ddt_scale));
        let eq168_e2122_d_n18: f64 = (p.p7 * (var_qd_fp3_dn18 * ddt_scale));
        let eq168_e2122_d_n19: f64 = (p.p7 * (var_qd_fp3_dn19 * ddt_scale));
        let eq168_e2122_d_n20: f64 = (p.p7 * (var_qd_fp3_dn20 * ddt_scale));
        let eq168_e2122_d_n21: f64 = (p.p7 * (var_qd_fp3_dn21 * ddt_scale));
        let eq168_e2122_d_n22: f64 = (p.p7 * (var_qd_fp3_dn22 * ddt_scale));
        (eq168_e2122, eq168_e2122_d_n0, eq168_e2122_d_n1, eq168_e2122_d_n2, eq168_e2122_d_n3, eq168_e2122_d_n4, eq168_e2122_d_n5, eq168_e2122_d_n6, eq168_e2122_d_n7, eq168_e2122_d_n8, eq168_e2122_d_n9, eq168_e2122_d_n12, eq168_e2122_d_n14, eq168_e2122_d_n15, eq168_e2122_d_n16, eq168_e2122_d_n17, eq168_e2122_d_n18, eq168_e2122_d_n19, eq168_e2122_d_n20, eq168_e2122_d_n21, eq168_e2122_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq168_value: f64 = eq168_e2124;
        let eq168_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq168_node_derivatives: [f64; 20] = [eq168_e2124_d_n0, eq168_e2124_d_n1, eq168_e2124_d_n2, eq168_e2124_d_n3, eq168_e2124_d_n4, eq168_e2124_d_n5, eq168_e2124_d_n6, eq168_e2124_d_n7, eq168_e2124_d_n8, eq168_e2124_d_n9, eq168_e2124_d_n12, eq168_e2124_d_n14, eq168_e2124_d_n15, eq168_e2124_d_n16, eq168_e2124_d_n17, eq168_e2124_d_n18, eq168_e2124_d_n19, eq168_e2124_d_n20, eq168_e2124_d_n21, eq168_e2124_d_n22];
        let eq168_branch_derivative_indices: [usize; 0] = [];
        let eq168_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(17),
            Some(16),
            multiplicity * (eq168_value),
            &eq168_node_derivative_indices,
            &eq168_node_derivatives,
            &eq168_branch_derivative_indices,
            &eq168_branch_derivatives,
            multiplicity,
        );
        let (eq169_e2135, eq169_e2135_d_n0, eq169_e2135_d_n1, eq169_e2135_d_n2, eq169_e2135_d_n3, eq169_e2135_d_n4, eq169_e2135_d_n5, eq169_e2135_d_n6, eq169_e2135_d_n7, eq169_e2135_d_n8, eq169_e2135_d_n9, eq169_e2135_d_n12, eq169_e2135_d_n14, eq169_e2135_d_n15, eq169_e2135_d_n16, eq169_e2135_d_n17, eq169_e2135_d_n18, eq169_e2135_d_n19, eq169_e2135_d_n20, eq169_e2135_d_n21, eq169_e2135_d_n22,) = {
    if (((var_guard556 != 0.0) && (var_guard557 != 0.0)) && (var_guard558 != 0.0)) {
        let eq169_e2132: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 68, var_qg_fp3);
        let eq169_e2133: f64 = (p.p7 * eq169_e2132);
        let eq169_e2133_d_n0: f64 = (p.p7 * (var_qg_fp3_dn0 * ddt_scale));
        let eq169_e2133_d_n1: f64 = (p.p7 * (var_qg_fp3_dn1 * ddt_scale));
        let eq169_e2133_d_n2: f64 = (p.p7 * (var_qg_fp3_dn2 * ddt_scale));
        let eq169_e2133_d_n3: f64 = (p.p7 * (var_qg_fp3_dn3 * ddt_scale));
        let eq169_e2133_d_n4: f64 = (p.p7 * (var_qg_fp3_dn4 * ddt_scale));
        let eq169_e2133_d_n5: f64 = (p.p7 * (var_qg_fp3_dn5 * ddt_scale));
        let eq169_e2133_d_n6: f64 = (p.p7 * (var_qg_fp3_dn6 * ddt_scale));
        let eq169_e2133_d_n7: f64 = (p.p7 * (var_qg_fp3_dn7 * ddt_scale));
        let eq169_e2133_d_n8: f64 = (p.p7 * (var_qg_fp3_dn8 * ddt_scale));
        let eq169_e2133_d_n9: f64 = (p.p7 * (var_qg_fp3_dn9 * ddt_scale));
        let eq169_e2133_d_n12: f64 = (p.p7 * (var_qg_fp3_dn12 * ddt_scale));
        let eq169_e2133_d_n14: f64 = (p.p7 * (var_qg_fp3_dn14 * ddt_scale));
        let eq169_e2133_d_n15: f64 = (p.p7 * (var_qg_fp3_dn15 * ddt_scale));
        let eq169_e2133_d_n16: f64 = (p.p7 * (var_qg_fp3_dn16 * ddt_scale));
        let eq169_e2133_d_n17: f64 = (p.p7 * (var_qg_fp3_dn17 * ddt_scale));
        let eq169_e2133_d_n18: f64 = (p.p7 * (var_qg_fp3_dn18 * ddt_scale));
        let eq169_e2133_d_n19: f64 = (p.p7 * (var_qg_fp3_dn19 * ddt_scale));
        let eq169_e2133_d_n20: f64 = (p.p7 * (var_qg_fp3_dn20 * ddt_scale));
        let eq169_e2133_d_n21: f64 = (p.p7 * (var_qg_fp3_dn21 * ddt_scale));
        let eq169_e2133_d_n22: f64 = (p.p7 * (var_qg_fp3_dn22 * ddt_scale));
        (eq169_e2133, eq169_e2133_d_n0, eq169_e2133_d_n1, eq169_e2133_d_n2, eq169_e2133_d_n3, eq169_e2133_d_n4, eq169_e2133_d_n5, eq169_e2133_d_n6, eq169_e2133_d_n7, eq169_e2133_d_n8, eq169_e2133_d_n9, eq169_e2133_d_n12, eq169_e2133_d_n14, eq169_e2133_d_n15, eq169_e2133_d_n16, eq169_e2133_d_n17, eq169_e2133_d_n18, eq169_e2133_d_n19, eq169_e2133_d_n20, eq169_e2133_d_n21, eq169_e2133_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq169_value: f64 = eq169_e2135;
        let eq169_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq169_node_derivatives: [f64; 20] = [eq169_e2135_d_n0, eq169_e2135_d_n1, eq169_e2135_d_n2, eq169_e2135_d_n3, eq169_e2135_d_n4, eq169_e2135_d_n5, eq169_e2135_d_n6, eq169_e2135_d_n7, eq169_e2135_d_n8, eq169_e2135_d_n9, eq169_e2135_d_n12, eq169_e2135_d_n14, eq169_e2135_d_n15, eq169_e2135_d_n16, eq169_e2135_d_n17, eq169_e2135_d_n18, eq169_e2135_d_n19, eq169_e2135_d_n20, eq169_e2135_d_n21, eq169_e2135_d_n22];
        let eq169_branch_derivative_indices: [usize; 0] = [];
        let eq169_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(16),
            multiplicity * (eq169_value),
            &eq169_node_derivative_indices,
            &eq169_node_derivatives,
            &eq169_branch_derivative_indices,
            &eq169_branch_derivatives,
            multiplicity,
        );
        let (eq170_e2148, eq170_e2148_d_n0, eq170_e2148_d_n1, eq170_e2148_d_n2, eq170_e2148_d_n3, eq170_e2148_d_n4, eq170_e2148_d_n5, eq170_e2148_d_n6, eq170_e2148_d_n7, eq170_e2148_d_n8, eq170_e2148_d_n9, eq170_e2148_d_n12, eq170_e2148_d_n14, eq170_e2148_d_n15, eq170_e2148_d_n16, eq170_e2148_d_n17, eq170_e2148_d_n18, eq170_e2148_d_n19, eq170_e2148_d_n20, eq170_e2148_d_n21, eq170_e2148_d_n22,) = {
    if (((var_guard556 != 0.0) && (var_guard557 != 0.0)) && (var_guard558 != 0.0)) {
        let eq170_e2143: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 69, var_qg_fp3);
        let eq170_e2144: f64 = (p.p7 * eq170_e2143);
        let eq170_e2144_d_n0: f64 = (p.p7 * (var_qg_fp3_dn0 * ddt_scale));
        let eq170_e2144_d_n1: f64 = (p.p7 * (var_qg_fp3_dn1 * ddt_scale));
        let eq170_e2144_d_n2: f64 = (p.p7 * (var_qg_fp3_dn2 * ddt_scale));
        let eq170_e2144_d_n3: f64 = (p.p7 * (var_qg_fp3_dn3 * ddt_scale));
        let eq170_e2144_d_n4: f64 = (p.p7 * (var_qg_fp3_dn4 * ddt_scale));
        let eq170_e2144_d_n5: f64 = (p.p7 * (var_qg_fp3_dn5 * ddt_scale));
        let eq170_e2144_d_n6: f64 = (p.p7 * (var_qg_fp3_dn6 * ddt_scale));
        let eq170_e2144_d_n7: f64 = (p.p7 * (var_qg_fp3_dn7 * ddt_scale));
        let eq170_e2144_d_n8: f64 = (p.p7 * (var_qg_fp3_dn8 * ddt_scale));
        let eq170_e2144_d_n9: f64 = (p.p7 * (var_qg_fp3_dn9 * ddt_scale));
        let eq170_e2144_d_n12: f64 = (p.p7 * (var_qg_fp3_dn12 * ddt_scale));
        let eq170_e2144_d_n14: f64 = (p.p7 * (var_qg_fp3_dn14 * ddt_scale));
        let eq170_e2144_d_n15: f64 = (p.p7 * (var_qg_fp3_dn15 * ddt_scale));
        let eq170_e2144_d_n16: f64 = (p.p7 * (var_qg_fp3_dn16 * ddt_scale));
        let eq170_e2144_d_n17: f64 = (p.p7 * (var_qg_fp3_dn17 * ddt_scale));
        let eq170_e2144_d_n18: f64 = (p.p7 * (var_qg_fp3_dn18 * ddt_scale));
        let eq170_e2144_d_n19: f64 = (p.p7 * (var_qg_fp3_dn19 * ddt_scale));
        let eq170_e2144_d_n20: f64 = (p.p7 * (var_qg_fp3_dn20 * ddt_scale));
        let eq170_e2144_d_n21: f64 = (p.p7 * (var_qg_fp3_dn21 * ddt_scale));
        let eq170_e2144_d_n22: f64 = (p.p7 * (var_qg_fp3_dn22 * ddt_scale));
        let eq170_e2146: f64 = (eq170_e2144 * p.p248);
        let eq170_e2146_d_n0: f64 = (eq170_e2144_d_n0 * p.p248);
        let eq170_e2146_d_n1: f64 = (eq170_e2144_d_n1 * p.p248);
        let eq170_e2146_d_n2: f64 = (eq170_e2144_d_n2 * p.p248);
        let eq170_e2146_d_n3: f64 = (eq170_e2144_d_n3 * p.p248);
        let eq170_e2146_d_n4: f64 = (eq170_e2144_d_n4 * p.p248);
        let eq170_e2146_d_n5: f64 = (eq170_e2144_d_n5 * p.p248);
        let eq170_e2146_d_n6: f64 = (eq170_e2144_d_n6 * p.p248);
        let eq170_e2146_d_n7: f64 = (eq170_e2144_d_n7 * p.p248);
        let eq170_e2146_d_n8: f64 = (eq170_e2144_d_n8 * p.p248);
        let eq170_e2146_d_n9: f64 = (eq170_e2144_d_n9 * p.p248);
        let eq170_e2146_d_n12: f64 = (eq170_e2144_d_n12 * p.p248);
        let eq170_e2146_d_n14: f64 = (eq170_e2144_d_n14 * p.p248);
        let eq170_e2146_d_n15: f64 = (eq170_e2144_d_n15 * p.p248);
        let eq170_e2146_d_n16: f64 = (eq170_e2144_d_n16 * p.p248);
        let eq170_e2146_d_n17: f64 = (eq170_e2144_d_n17 * p.p248);
        let eq170_e2146_d_n18: f64 = (eq170_e2144_d_n18 * p.p248);
        let eq170_e2146_d_n19: f64 = (eq170_e2144_d_n19 * p.p248);
        let eq170_e2146_d_n20: f64 = (eq170_e2144_d_n20 * p.p248);
        let eq170_e2146_d_n21: f64 = (eq170_e2144_d_n21 * p.p248);
        let eq170_e2146_d_n22: f64 = (eq170_e2144_d_n22 * p.p248);
        (eq170_e2146, eq170_e2146_d_n0, eq170_e2146_d_n1, eq170_e2146_d_n2, eq170_e2146_d_n3, eq170_e2146_d_n4, eq170_e2146_d_n5, eq170_e2146_d_n6, eq170_e2146_d_n7, eq170_e2146_d_n8, eq170_e2146_d_n9, eq170_e2146_d_n12, eq170_e2146_d_n14, eq170_e2146_d_n15, eq170_e2146_d_n16, eq170_e2146_d_n17, eq170_e2146_d_n18, eq170_e2146_d_n19, eq170_e2146_d_n20, eq170_e2146_d_n21, eq170_e2146_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq170_value: f64 = eq170_e2148;
        let eq170_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq170_node_derivatives: [f64; 20] = [eq170_e2148_d_n0, eq170_e2148_d_n1, eq170_e2148_d_n2, eq170_e2148_d_n3, eq170_e2148_d_n4, eq170_e2148_d_n5, eq170_e2148_d_n6, eq170_e2148_d_n7, eq170_e2148_d_n8, eq170_e2148_d_n9, eq170_e2148_d_n12, eq170_e2148_d_n14, eq170_e2148_d_n15, eq170_e2148_d_n16, eq170_e2148_d_n17, eq170_e2148_d_n18, eq170_e2148_d_n19, eq170_e2148_d_n20, eq170_e2148_d_n21, eq170_e2148_d_n22];
        let eq170_branch_derivative_indices: [usize; 0] = [];
        let eq170_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(16),
            multiplicity * (eq170_value),
            &eq170_node_derivative_indices,
            &eq170_node_derivatives,
            &eq170_branch_derivative_indices,
            &eq170_branch_derivatives,
            multiplicity,
        );
        let (eq171_e2160, eq171_e2160_d_n0, eq171_e2160_d_n1, eq171_e2160_d_n2, eq171_e2160_d_n3, eq171_e2160_d_n4, eq171_e2160_d_n5, eq171_e2160_d_n6, eq171_e2160_d_n7, eq171_e2160_d_n8, eq171_e2160_d_n9, eq171_e2160_d_n12, eq171_e2160_d_n14, eq171_e2160_d_n15, eq171_e2160_d_n16, eq171_e2160_d_n17, eq171_e2160_d_n18, eq171_e2160_d_n19, eq171_e2160_d_n20, eq171_e2160_d_n21, eq171_e2160_d_n22,) = {
    if (((var_guard556 != 0.0) && (var_guard557 != 0.0)) && (var_guard558 == 0.0)) {
        let eq171_e2157: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 70, var_qg_fp3);
        let eq171_e2158: f64 = (p.p7 * eq171_e2157);
        let eq171_e2158_d_n0: f64 = (p.p7 * (var_qg_fp3_dn0 * ddt_scale));
        let eq171_e2158_d_n1: f64 = (p.p7 * (var_qg_fp3_dn1 * ddt_scale));
        let eq171_e2158_d_n2: f64 = (p.p7 * (var_qg_fp3_dn2 * ddt_scale));
        let eq171_e2158_d_n3: f64 = (p.p7 * (var_qg_fp3_dn3 * ddt_scale));
        let eq171_e2158_d_n4: f64 = (p.p7 * (var_qg_fp3_dn4 * ddt_scale));
        let eq171_e2158_d_n5: f64 = (p.p7 * (var_qg_fp3_dn5 * ddt_scale));
        let eq171_e2158_d_n6: f64 = (p.p7 * (var_qg_fp3_dn6 * ddt_scale));
        let eq171_e2158_d_n7: f64 = (p.p7 * (var_qg_fp3_dn7 * ddt_scale));
        let eq171_e2158_d_n8: f64 = (p.p7 * (var_qg_fp3_dn8 * ddt_scale));
        let eq171_e2158_d_n9: f64 = (p.p7 * (var_qg_fp3_dn9 * ddt_scale));
        let eq171_e2158_d_n12: f64 = (p.p7 * (var_qg_fp3_dn12 * ddt_scale));
        let eq171_e2158_d_n14: f64 = (p.p7 * (var_qg_fp3_dn14 * ddt_scale));
        let eq171_e2158_d_n15: f64 = (p.p7 * (var_qg_fp3_dn15 * ddt_scale));
        let eq171_e2158_d_n16: f64 = (p.p7 * (var_qg_fp3_dn16 * ddt_scale));
        let eq171_e2158_d_n17: f64 = (p.p7 * (var_qg_fp3_dn17 * ddt_scale));
        let eq171_e2158_d_n18: f64 = (p.p7 * (var_qg_fp3_dn18 * ddt_scale));
        let eq171_e2158_d_n19: f64 = (p.p7 * (var_qg_fp3_dn19 * ddt_scale));
        let eq171_e2158_d_n20: f64 = (p.p7 * (var_qg_fp3_dn20 * ddt_scale));
        let eq171_e2158_d_n21: f64 = (p.p7 * (var_qg_fp3_dn21 * ddt_scale));
        let eq171_e2158_d_n22: f64 = (p.p7 * (var_qg_fp3_dn22 * ddt_scale));
        (eq171_e2158, eq171_e2158_d_n0, eq171_e2158_d_n1, eq171_e2158_d_n2, eq171_e2158_d_n3, eq171_e2158_d_n4, eq171_e2158_d_n5, eq171_e2158_d_n6, eq171_e2158_d_n7, eq171_e2158_d_n8, eq171_e2158_d_n9, eq171_e2158_d_n12, eq171_e2158_d_n14, eq171_e2158_d_n15, eq171_e2158_d_n16, eq171_e2158_d_n17, eq171_e2158_d_n18, eq171_e2158_d_n19, eq171_e2158_d_n20, eq171_e2158_d_n21, eq171_e2158_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq171_value: f64 = eq171_e2160;
        let eq171_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq171_node_derivatives: [f64; 20] = [eq171_e2160_d_n0, eq171_e2160_d_n1, eq171_e2160_d_n2, eq171_e2160_d_n3, eq171_e2160_d_n4, eq171_e2160_d_n5, eq171_e2160_d_n6, eq171_e2160_d_n7, eq171_e2160_d_n8, eq171_e2160_d_n9, eq171_e2160_d_n12, eq171_e2160_d_n14, eq171_e2160_d_n15, eq171_e2160_d_n16, eq171_e2160_d_n17, eq171_e2160_d_n18, eq171_e2160_d_n19, eq171_e2160_d_n20, eq171_e2160_d_n21, eq171_e2160_d_n22];
        let eq171_branch_derivative_indices: [usize; 0] = [];
        let eq171_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(16),
            multiplicity * (eq171_value),
            &eq171_node_derivative_indices,
            &eq171_node_derivatives,
            &eq171_branch_derivative_indices,
            &eq171_branch_derivatives,
            multiplicity,
        );
        let (eq172_e2174, eq172_e2174_d_n0, eq172_e2174_d_n1, eq172_e2174_d_n2, eq172_e2174_d_n3, eq172_e2174_d_n4, eq172_e2174_d_n5, eq172_e2174_d_n6, eq172_e2174_d_n7, eq172_e2174_d_n8, eq172_e2174_d_n9, eq172_e2174_d_n12, eq172_e2174_d_n14, eq172_e2174_d_n15, eq172_e2174_d_n16, eq172_e2174_d_n17, eq172_e2174_d_n18, eq172_e2174_d_n19, eq172_e2174_d_n20, eq172_e2174_d_n21, eq172_e2174_d_n22,) = {
    if (((var_guard556 != 0.0) && (var_guard557 != 0.0)) && (var_guard558 == 0.0)) {
        let eq172_e2169: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 71, var_qg_fp3);
        let eq172_e2170: f64 = (p.p7 * eq172_e2169);
        let eq172_e2170_d_n0: f64 = (p.p7 * (var_qg_fp3_dn0 * ddt_scale));
        let eq172_e2170_d_n1: f64 = (p.p7 * (var_qg_fp3_dn1 * ddt_scale));
        let eq172_e2170_d_n2: f64 = (p.p7 * (var_qg_fp3_dn2 * ddt_scale));
        let eq172_e2170_d_n3: f64 = (p.p7 * (var_qg_fp3_dn3 * ddt_scale));
        let eq172_e2170_d_n4: f64 = (p.p7 * (var_qg_fp3_dn4 * ddt_scale));
        let eq172_e2170_d_n5: f64 = (p.p7 * (var_qg_fp3_dn5 * ddt_scale));
        let eq172_e2170_d_n6: f64 = (p.p7 * (var_qg_fp3_dn6 * ddt_scale));
        let eq172_e2170_d_n7: f64 = (p.p7 * (var_qg_fp3_dn7 * ddt_scale));
        let eq172_e2170_d_n8: f64 = (p.p7 * (var_qg_fp3_dn8 * ddt_scale));
        let eq172_e2170_d_n9: f64 = (p.p7 * (var_qg_fp3_dn9 * ddt_scale));
        let eq172_e2170_d_n12: f64 = (p.p7 * (var_qg_fp3_dn12 * ddt_scale));
        let eq172_e2170_d_n14: f64 = (p.p7 * (var_qg_fp3_dn14 * ddt_scale));
        let eq172_e2170_d_n15: f64 = (p.p7 * (var_qg_fp3_dn15 * ddt_scale));
        let eq172_e2170_d_n16: f64 = (p.p7 * (var_qg_fp3_dn16 * ddt_scale));
        let eq172_e2170_d_n17: f64 = (p.p7 * (var_qg_fp3_dn17 * ddt_scale));
        let eq172_e2170_d_n18: f64 = (p.p7 * (var_qg_fp3_dn18 * ddt_scale));
        let eq172_e2170_d_n19: f64 = (p.p7 * (var_qg_fp3_dn19 * ddt_scale));
        let eq172_e2170_d_n20: f64 = (p.p7 * (var_qg_fp3_dn20 * ddt_scale));
        let eq172_e2170_d_n21: f64 = (p.p7 * (var_qg_fp3_dn21 * ddt_scale));
        let eq172_e2170_d_n22: f64 = (p.p7 * (var_qg_fp3_dn22 * ddt_scale));
        let eq172_e2172: f64 = (eq172_e2170 * p.p248);
        let eq172_e2172_d_n0: f64 = (eq172_e2170_d_n0 * p.p248);
        let eq172_e2172_d_n1: f64 = (eq172_e2170_d_n1 * p.p248);
        let eq172_e2172_d_n2: f64 = (eq172_e2170_d_n2 * p.p248);
        let eq172_e2172_d_n3: f64 = (eq172_e2170_d_n3 * p.p248);
        let eq172_e2172_d_n4: f64 = (eq172_e2170_d_n4 * p.p248);
        let eq172_e2172_d_n5: f64 = (eq172_e2170_d_n5 * p.p248);
        let eq172_e2172_d_n6: f64 = (eq172_e2170_d_n6 * p.p248);
        let eq172_e2172_d_n7: f64 = (eq172_e2170_d_n7 * p.p248);
        let eq172_e2172_d_n8: f64 = (eq172_e2170_d_n8 * p.p248);
        let eq172_e2172_d_n9: f64 = (eq172_e2170_d_n9 * p.p248);
        let eq172_e2172_d_n12: f64 = (eq172_e2170_d_n12 * p.p248);
        let eq172_e2172_d_n14: f64 = (eq172_e2170_d_n14 * p.p248);
        let eq172_e2172_d_n15: f64 = (eq172_e2170_d_n15 * p.p248);
        let eq172_e2172_d_n16: f64 = (eq172_e2170_d_n16 * p.p248);
        let eq172_e2172_d_n17: f64 = (eq172_e2170_d_n17 * p.p248);
        let eq172_e2172_d_n18: f64 = (eq172_e2170_d_n18 * p.p248);
        let eq172_e2172_d_n19: f64 = (eq172_e2170_d_n19 * p.p248);
        let eq172_e2172_d_n20: f64 = (eq172_e2170_d_n20 * p.p248);
        let eq172_e2172_d_n21: f64 = (eq172_e2170_d_n21 * p.p248);
        let eq172_e2172_d_n22: f64 = (eq172_e2170_d_n22 * p.p248);
        (eq172_e2172, eq172_e2172_d_n0, eq172_e2172_d_n1, eq172_e2172_d_n2, eq172_e2172_d_n3, eq172_e2172_d_n4, eq172_e2172_d_n5, eq172_e2172_d_n6, eq172_e2172_d_n7, eq172_e2172_d_n8, eq172_e2172_d_n9, eq172_e2172_d_n12, eq172_e2172_d_n14, eq172_e2172_d_n15, eq172_e2172_d_n16, eq172_e2172_d_n17, eq172_e2172_d_n18, eq172_e2172_d_n19, eq172_e2172_d_n20, eq172_e2172_d_n21, eq172_e2172_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq172_value: f64 = eq172_e2174;
        let eq172_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq172_node_derivatives: [f64; 20] = [eq172_e2174_d_n0, eq172_e2174_d_n1, eq172_e2174_d_n2, eq172_e2174_d_n3, eq172_e2174_d_n4, eq172_e2174_d_n5, eq172_e2174_d_n6, eq172_e2174_d_n7, eq172_e2174_d_n8, eq172_e2174_d_n9, eq172_e2174_d_n12, eq172_e2174_d_n14, eq172_e2174_d_n15, eq172_e2174_d_n16, eq172_e2174_d_n17, eq172_e2174_d_n18, eq172_e2174_d_n19, eq172_e2174_d_n20, eq172_e2174_d_n21, eq172_e2174_d_n22];
        let eq172_branch_derivative_indices: [usize; 0] = [];
        let eq172_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(16),
            multiplicity * (eq172_value),
            &eq172_node_derivative_indices,
            &eq172_node_derivatives,
            &eq172_branch_derivative_indices,
            &eq172_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_9(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
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
        var_guard556: f64,
        var_guard557: f64,
        var_guard559: f64,
        var_guard560: f64,
        var_guard561: f64,
        var_guard562: f64,
        var_guard563: f64,
        var_qd_fp3: f64,
        var_qd_fp3_dn0: f64,
        var_qd_fp3_dn1: f64,
        var_qd_fp3_dn12: f64,
        var_qd_fp3_dn14: f64,
        var_qd_fp3_dn15: f64,
        var_qd_fp3_dn16: f64,
        var_qd_fp3_dn17: f64,
        var_qd_fp3_dn18: f64,
        var_qd_fp3_dn19: f64,
        var_qd_fp3_dn2: f64,
        var_qd_fp3_dn20: f64,
        var_qd_fp3_dn21: f64,
        var_qd_fp3_dn22: f64,
        var_qd_fp3_dn3: f64,
        var_qd_fp3_dn4: f64,
        var_qd_fp3_dn5: f64,
        var_qd_fp3_dn6: f64,
        var_qd_fp3_dn7: f64,
        var_qd_fp3_dn8: f64,
        var_qd_fp3_dn9: f64,
        var_qd_fp3s: f64,
        var_qd_fp3s_dn0: f64,
        var_qd_fp3s_dn1: f64,
        var_qd_fp3s_dn12: f64,
        var_qd_fp3s_dn14: f64,
        var_qd_fp3s_dn15: f64,
        var_qd_fp3s_dn16: f64,
        var_qd_fp3s_dn17: f64,
        var_qd_fp3s_dn18: f64,
        var_qd_fp3s_dn19: f64,
        var_qd_fp3s_dn2: f64,
        var_qd_fp3s_dn20: f64,
        var_qd_fp3s_dn21: f64,
        var_qd_fp3s_dn22: f64,
        var_qd_fp3s_dn3: f64,
        var_qd_fp3s_dn4: f64,
        var_qd_fp3s_dn5: f64,
        var_qd_fp3s_dn6: f64,
        var_qd_fp3s_dn7: f64,
        var_qd_fp3s_dn8: f64,
        var_qd_fp3s_dn9: f64,
        var_qg_fp3: f64,
        var_qg_fp3_dn0: f64,
        var_qg_fp3_dn1: f64,
        var_qg_fp3_dn12: f64,
        var_qg_fp3_dn14: f64,
        var_qg_fp3_dn15: f64,
        var_qg_fp3_dn16: f64,
        var_qg_fp3_dn17: f64,
        var_qg_fp3_dn18: f64,
        var_qg_fp3_dn19: f64,
        var_qg_fp3_dn2: f64,
        var_qg_fp3_dn20: f64,
        var_qg_fp3_dn21: f64,
        var_qg_fp3_dn22: f64,
        var_qg_fp3_dn3: f64,
        var_qg_fp3_dn4: f64,
        var_qg_fp3_dn5: f64,
        var_qg_fp3_dn6: f64,
        var_qg_fp3_dn7: f64,
        var_qg_fp3_dn8: f64,
        var_qg_fp3_dn9: f64,
        var_qg_fp3s: f64,
        var_qg_fp3s_dn0: f64,
        var_qg_fp3s_dn1: f64,
        var_qg_fp3s_dn12: f64,
        var_qg_fp3s_dn14: f64,
        var_qg_fp3s_dn15: f64,
        var_qg_fp3s_dn16: f64,
        var_qg_fp3s_dn17: f64,
        var_qg_fp3s_dn18: f64,
        var_qg_fp3s_dn19: f64,
        var_qg_fp3s_dn2: f64,
        var_qg_fp3s_dn20: f64,
        var_qg_fp3s_dn21: f64,
        var_qg_fp3s_dn22: f64,
        var_qg_fp3s_dn3: f64,
        var_qg_fp3s_dn4: f64,
        var_qg_fp3s_dn5: f64,
        var_qg_fp3s_dn6: f64,
        var_qg_fp3s_dn7: f64,
        var_qg_fp3s_dn8: f64,
        var_qg_fp3s_dn9: f64,
    ) {
        let (eq173_e2185, eq173_e2185_d_n0, eq173_e2185_d_n1, eq173_e2185_d_n2, eq173_e2185_d_n3, eq173_e2185_d_n4, eq173_e2185_d_n5, eq173_e2185_d_n6, eq173_e2185_d_n7, eq173_e2185_d_n8, eq173_e2185_d_n9, eq173_e2185_d_n12, eq173_e2185_d_n14, eq173_e2185_d_n15, eq173_e2185_d_n16, eq173_e2185_d_n17, eq173_e2185_d_n18, eq173_e2185_d_n19, eq173_e2185_d_n20, eq173_e2185_d_n21, eq173_e2185_d_n22,) = {
    if ((var_guard556 != 0.0) && (var_guard557 != 0.0)) {
        let eq173_e2181: f64 = (p.p253 * var_qg_fp3);
        let eq173_e2181_d_n0: f64 = (p.p253 * var_qg_fp3_dn0);
        let eq173_e2181_d_n1: f64 = (p.p253 * var_qg_fp3_dn1);
        let eq173_e2181_d_n2: f64 = (p.p253 * var_qg_fp3_dn2);
        let eq173_e2181_d_n3: f64 = (p.p253 * var_qg_fp3_dn3);
        let eq173_e2181_d_n4: f64 = (p.p253 * var_qg_fp3_dn4);
        let eq173_e2181_d_n5: f64 = (p.p253 * var_qg_fp3_dn5);
        let eq173_e2181_d_n6: f64 = (p.p253 * var_qg_fp3_dn6);
        let eq173_e2181_d_n7: f64 = (p.p253 * var_qg_fp3_dn7);
        let eq173_e2181_d_n8: f64 = (p.p253 * var_qg_fp3_dn8);
        let eq173_e2181_d_n9: f64 = (p.p253 * var_qg_fp3_dn9);
        let eq173_e2181_d_n12: f64 = (p.p253 * var_qg_fp3_dn12);
        let eq173_e2181_d_n14: f64 = (p.p253 * var_qg_fp3_dn14);
        let eq173_e2181_d_n15: f64 = (p.p253 * var_qg_fp3_dn15);
        let eq173_e2181_d_n16: f64 = (p.p253 * var_qg_fp3_dn16);
        let eq173_e2181_d_n17: f64 = (p.p253 * var_qg_fp3_dn17);
        let eq173_e2181_d_n18: f64 = (p.p253 * var_qg_fp3_dn18);
        let eq173_e2181_d_n19: f64 = (p.p253 * var_qg_fp3_dn19);
        let eq173_e2181_d_n20: f64 = (p.p253 * var_qg_fp3_dn20);
        let eq173_e2181_d_n21: f64 = (p.p253 * var_qg_fp3_dn21);
        let eq173_e2181_d_n22: f64 = (p.p253 * var_qg_fp3_dn22);
        let eq173_e2182: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 72, eq173_e2181);
        let eq173_e2183: f64 = (p.p7 * eq173_e2182);
        let eq173_e2183_d_n0: f64 = (p.p7 * (eq173_e2181_d_n0 * ddt_scale));
        let eq173_e2183_d_n1: f64 = (p.p7 * (eq173_e2181_d_n1 * ddt_scale));
        let eq173_e2183_d_n2: f64 = (p.p7 * (eq173_e2181_d_n2 * ddt_scale));
        let eq173_e2183_d_n3: f64 = (p.p7 * (eq173_e2181_d_n3 * ddt_scale));
        let eq173_e2183_d_n4: f64 = (p.p7 * (eq173_e2181_d_n4 * ddt_scale));
        let eq173_e2183_d_n5: f64 = (p.p7 * (eq173_e2181_d_n5 * ddt_scale));
        let eq173_e2183_d_n6: f64 = (p.p7 * (eq173_e2181_d_n6 * ddt_scale));
        let eq173_e2183_d_n7: f64 = (p.p7 * (eq173_e2181_d_n7 * ddt_scale));
        let eq173_e2183_d_n8: f64 = (p.p7 * (eq173_e2181_d_n8 * ddt_scale));
        let eq173_e2183_d_n9: f64 = (p.p7 * (eq173_e2181_d_n9 * ddt_scale));
        let eq173_e2183_d_n12: f64 = (p.p7 * (eq173_e2181_d_n12 * ddt_scale));
        let eq173_e2183_d_n14: f64 = (p.p7 * (eq173_e2181_d_n14 * ddt_scale));
        let eq173_e2183_d_n15: f64 = (p.p7 * (eq173_e2181_d_n15 * ddt_scale));
        let eq173_e2183_d_n16: f64 = (p.p7 * (eq173_e2181_d_n16 * ddt_scale));
        let eq173_e2183_d_n17: f64 = (p.p7 * (eq173_e2181_d_n17 * ddt_scale));
        let eq173_e2183_d_n18: f64 = (p.p7 * (eq173_e2181_d_n18 * ddt_scale));
        let eq173_e2183_d_n19: f64 = (p.p7 * (eq173_e2181_d_n19 * ddt_scale));
        let eq173_e2183_d_n20: f64 = (p.p7 * (eq173_e2181_d_n20 * ddt_scale));
        let eq173_e2183_d_n21: f64 = (p.p7 * (eq173_e2181_d_n21 * ddt_scale));
        let eq173_e2183_d_n22: f64 = (p.p7 * (eq173_e2181_d_n22 * ddt_scale));
        (eq173_e2183, eq173_e2183_d_n0, eq173_e2183_d_n1, eq173_e2183_d_n2, eq173_e2183_d_n3, eq173_e2183_d_n4, eq173_e2183_d_n5, eq173_e2183_d_n6, eq173_e2183_d_n7, eq173_e2183_d_n8, eq173_e2183_d_n9, eq173_e2183_d_n12, eq173_e2183_d_n14, eq173_e2183_d_n15, eq173_e2183_d_n16, eq173_e2183_d_n17, eq173_e2183_d_n18, eq173_e2183_d_n19, eq173_e2183_d_n20, eq173_e2183_d_n21, eq173_e2183_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq173_value: f64 = eq173_e2185;
        let eq173_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq173_node_derivatives: [f64; 20] = [eq173_e2185_d_n0, eq173_e2185_d_n1, eq173_e2185_d_n2, eq173_e2185_d_n3, eq173_e2185_d_n4, eq173_e2185_d_n5, eq173_e2185_d_n6, eq173_e2185_d_n7, eq173_e2185_d_n8, eq173_e2185_d_n9, eq173_e2185_d_n12, eq173_e2185_d_n14, eq173_e2185_d_n15, eq173_e2185_d_n16, eq173_e2185_d_n17, eq173_e2185_d_n18, eq173_e2185_d_n19, eq173_e2185_d_n20, eq173_e2185_d_n21, eq173_e2185_d_n22];
        let eq173_branch_derivative_indices: [usize; 0] = [];
        let eq173_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(16),
            multiplicity * (eq173_value),
            &eq173_node_derivative_indices,
            &eq173_node_derivatives,
            &eq173_branch_derivative_indices,
            &eq173_branch_derivatives,
            multiplicity,
        );
        let (eq174_e2195, eq174_e2195_d_n0, eq174_e2195_d_n1, eq174_e2195_d_n2, eq174_e2195_d_n3, eq174_e2195_d_n4, eq174_e2195_d_n5, eq174_e2195_d_n6, eq174_e2195_d_n7, eq174_e2195_d_n8, eq174_e2195_d_n9, eq174_e2195_d_n12, eq174_e2195_d_n14, eq174_e2195_d_n15, eq174_e2195_d_n16, eq174_e2195_d_n17, eq174_e2195_d_n18, eq174_e2195_d_n19, eq174_e2195_d_n20, eq174_e2195_d_n21, eq174_e2195_d_n22,) = {
    if ((var_guard556 == 0.0) && (var_guard559 != 0.0)) {
        let eq174_e2192: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 73, var_qd_fp3);
        let eq174_e2193: f64 = (p.p7 * eq174_e2192);
        let eq174_e2193_d_n0: f64 = (p.p7 * (var_qd_fp3_dn0 * ddt_scale));
        let eq174_e2193_d_n1: f64 = (p.p7 * (var_qd_fp3_dn1 * ddt_scale));
        let eq174_e2193_d_n2: f64 = (p.p7 * (var_qd_fp3_dn2 * ddt_scale));
        let eq174_e2193_d_n3: f64 = (p.p7 * (var_qd_fp3_dn3 * ddt_scale));
        let eq174_e2193_d_n4: f64 = (p.p7 * (var_qd_fp3_dn4 * ddt_scale));
        let eq174_e2193_d_n5: f64 = (p.p7 * (var_qd_fp3_dn5 * ddt_scale));
        let eq174_e2193_d_n6: f64 = (p.p7 * (var_qd_fp3_dn6 * ddt_scale));
        let eq174_e2193_d_n7: f64 = (p.p7 * (var_qd_fp3_dn7 * ddt_scale));
        let eq174_e2193_d_n8: f64 = (p.p7 * (var_qd_fp3_dn8 * ddt_scale));
        let eq174_e2193_d_n9: f64 = (p.p7 * (var_qd_fp3_dn9 * ddt_scale));
        let eq174_e2193_d_n12: f64 = (p.p7 * (var_qd_fp3_dn12 * ddt_scale));
        let eq174_e2193_d_n14: f64 = (p.p7 * (var_qd_fp3_dn14 * ddt_scale));
        let eq174_e2193_d_n15: f64 = (p.p7 * (var_qd_fp3_dn15 * ddt_scale));
        let eq174_e2193_d_n16: f64 = (p.p7 * (var_qd_fp3_dn16 * ddt_scale));
        let eq174_e2193_d_n17: f64 = (p.p7 * (var_qd_fp3_dn17 * ddt_scale));
        let eq174_e2193_d_n18: f64 = (p.p7 * (var_qd_fp3_dn18 * ddt_scale));
        let eq174_e2193_d_n19: f64 = (p.p7 * (var_qd_fp3_dn19 * ddt_scale));
        let eq174_e2193_d_n20: f64 = (p.p7 * (var_qd_fp3_dn20 * ddt_scale));
        let eq174_e2193_d_n21: f64 = (p.p7 * (var_qd_fp3_dn21 * ddt_scale));
        let eq174_e2193_d_n22: f64 = (p.p7 * (var_qd_fp3_dn22 * ddt_scale));
        (eq174_e2193, eq174_e2193_d_n0, eq174_e2193_d_n1, eq174_e2193_d_n2, eq174_e2193_d_n3, eq174_e2193_d_n4, eq174_e2193_d_n5, eq174_e2193_d_n6, eq174_e2193_d_n7, eq174_e2193_d_n8, eq174_e2193_d_n9, eq174_e2193_d_n12, eq174_e2193_d_n14, eq174_e2193_d_n15, eq174_e2193_d_n16, eq174_e2193_d_n17, eq174_e2193_d_n18, eq174_e2193_d_n19, eq174_e2193_d_n20, eq174_e2193_d_n21, eq174_e2193_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq174_value: f64 = eq174_e2195;
        let eq174_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq174_node_derivatives: [f64; 20] = [eq174_e2195_d_n0, eq174_e2195_d_n1, eq174_e2195_d_n2, eq174_e2195_d_n3, eq174_e2195_d_n4, eq174_e2195_d_n5, eq174_e2195_d_n6, eq174_e2195_d_n7, eq174_e2195_d_n8, eq174_e2195_d_n9, eq174_e2195_d_n12, eq174_e2195_d_n14, eq174_e2195_d_n15, eq174_e2195_d_n16, eq174_e2195_d_n17, eq174_e2195_d_n18, eq174_e2195_d_n19, eq174_e2195_d_n20, eq174_e2195_d_n21, eq174_e2195_d_n22];
        let eq174_branch_derivative_indices: [usize; 0] = [];
        let eq174_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq174_value),
            &eq174_node_derivative_indices,
            &eq174_node_derivatives,
            &eq174_branch_derivative_indices,
            &eq174_branch_derivatives,
            multiplicity,
        );
        let (eq175_e2207, eq175_e2207_d_n0, eq175_e2207_d_n1, eq175_e2207_d_n2, eq175_e2207_d_n3, eq175_e2207_d_n4, eq175_e2207_d_n5, eq175_e2207_d_n6, eq175_e2207_d_n7, eq175_e2207_d_n8, eq175_e2207_d_n9, eq175_e2207_d_n12, eq175_e2207_d_n14, eq175_e2207_d_n15, eq175_e2207_d_n16, eq175_e2207_d_n17, eq175_e2207_d_n18, eq175_e2207_d_n19, eq175_e2207_d_n20, eq175_e2207_d_n21, eq175_e2207_d_n22,) = {
    if (((var_guard556 == 0.0) && (var_guard559 != 0.0)) && (var_guard560 != 0.0)) {
        let eq175_e2204: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 74, var_qg_fp3);
        let eq175_e2205: f64 = (p.p7 * eq175_e2204);
        let eq175_e2205_d_n0: f64 = (p.p7 * (var_qg_fp3_dn0 * ddt_scale));
        let eq175_e2205_d_n1: f64 = (p.p7 * (var_qg_fp3_dn1 * ddt_scale));
        let eq175_e2205_d_n2: f64 = (p.p7 * (var_qg_fp3_dn2 * ddt_scale));
        let eq175_e2205_d_n3: f64 = (p.p7 * (var_qg_fp3_dn3 * ddt_scale));
        let eq175_e2205_d_n4: f64 = (p.p7 * (var_qg_fp3_dn4 * ddt_scale));
        let eq175_e2205_d_n5: f64 = (p.p7 * (var_qg_fp3_dn5 * ddt_scale));
        let eq175_e2205_d_n6: f64 = (p.p7 * (var_qg_fp3_dn6 * ddt_scale));
        let eq175_e2205_d_n7: f64 = (p.p7 * (var_qg_fp3_dn7 * ddt_scale));
        let eq175_e2205_d_n8: f64 = (p.p7 * (var_qg_fp3_dn8 * ddt_scale));
        let eq175_e2205_d_n9: f64 = (p.p7 * (var_qg_fp3_dn9 * ddt_scale));
        let eq175_e2205_d_n12: f64 = (p.p7 * (var_qg_fp3_dn12 * ddt_scale));
        let eq175_e2205_d_n14: f64 = (p.p7 * (var_qg_fp3_dn14 * ddt_scale));
        let eq175_e2205_d_n15: f64 = (p.p7 * (var_qg_fp3_dn15 * ddt_scale));
        let eq175_e2205_d_n16: f64 = (p.p7 * (var_qg_fp3_dn16 * ddt_scale));
        let eq175_e2205_d_n17: f64 = (p.p7 * (var_qg_fp3_dn17 * ddt_scale));
        let eq175_e2205_d_n18: f64 = (p.p7 * (var_qg_fp3_dn18 * ddt_scale));
        let eq175_e2205_d_n19: f64 = (p.p7 * (var_qg_fp3_dn19 * ddt_scale));
        let eq175_e2205_d_n20: f64 = (p.p7 * (var_qg_fp3_dn20 * ddt_scale));
        let eq175_e2205_d_n21: f64 = (p.p7 * (var_qg_fp3_dn21 * ddt_scale));
        let eq175_e2205_d_n22: f64 = (p.p7 * (var_qg_fp3_dn22 * ddt_scale));
        (eq175_e2205, eq175_e2205_d_n0, eq175_e2205_d_n1, eq175_e2205_d_n2, eq175_e2205_d_n3, eq175_e2205_d_n4, eq175_e2205_d_n5, eq175_e2205_d_n6, eq175_e2205_d_n7, eq175_e2205_d_n8, eq175_e2205_d_n9, eq175_e2205_d_n12, eq175_e2205_d_n14, eq175_e2205_d_n15, eq175_e2205_d_n16, eq175_e2205_d_n17, eq175_e2205_d_n18, eq175_e2205_d_n19, eq175_e2205_d_n20, eq175_e2205_d_n21, eq175_e2205_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq175_value: f64 = eq175_e2207;
        let eq175_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq175_node_derivatives: [f64; 20] = [eq175_e2207_d_n0, eq175_e2207_d_n1, eq175_e2207_d_n2, eq175_e2207_d_n3, eq175_e2207_d_n4, eq175_e2207_d_n5, eq175_e2207_d_n6, eq175_e2207_d_n7, eq175_e2207_d_n8, eq175_e2207_d_n9, eq175_e2207_d_n12, eq175_e2207_d_n14, eq175_e2207_d_n15, eq175_e2207_d_n16, eq175_e2207_d_n17, eq175_e2207_d_n18, eq175_e2207_d_n19, eq175_e2207_d_n20, eq175_e2207_d_n21, eq175_e2207_d_n22];
        let eq175_branch_derivative_indices: [usize; 0] = [];
        let eq175_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq175_value),
            &eq175_node_derivative_indices,
            &eq175_node_derivatives,
            &eq175_branch_derivative_indices,
            &eq175_branch_derivatives,
            multiplicity,
        );
        let (eq176_e2221, eq176_e2221_d_n0, eq176_e2221_d_n1, eq176_e2221_d_n2, eq176_e2221_d_n3, eq176_e2221_d_n4, eq176_e2221_d_n5, eq176_e2221_d_n6, eq176_e2221_d_n7, eq176_e2221_d_n8, eq176_e2221_d_n9, eq176_e2221_d_n12, eq176_e2221_d_n14, eq176_e2221_d_n15, eq176_e2221_d_n16, eq176_e2221_d_n17, eq176_e2221_d_n18, eq176_e2221_d_n19, eq176_e2221_d_n20, eq176_e2221_d_n21, eq176_e2221_d_n22,) = {
    if (((var_guard556 == 0.0) && (var_guard559 != 0.0)) && (var_guard560 != 0.0)) {
        let eq176_e2216: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 75, var_qg_fp3);
        let eq176_e2217: f64 = (p.p7 * eq176_e2216);
        let eq176_e2217_d_n0: f64 = (p.p7 * (var_qg_fp3_dn0 * ddt_scale));
        let eq176_e2217_d_n1: f64 = (p.p7 * (var_qg_fp3_dn1 * ddt_scale));
        let eq176_e2217_d_n2: f64 = (p.p7 * (var_qg_fp3_dn2 * ddt_scale));
        let eq176_e2217_d_n3: f64 = (p.p7 * (var_qg_fp3_dn3 * ddt_scale));
        let eq176_e2217_d_n4: f64 = (p.p7 * (var_qg_fp3_dn4 * ddt_scale));
        let eq176_e2217_d_n5: f64 = (p.p7 * (var_qg_fp3_dn5 * ddt_scale));
        let eq176_e2217_d_n6: f64 = (p.p7 * (var_qg_fp3_dn6 * ddt_scale));
        let eq176_e2217_d_n7: f64 = (p.p7 * (var_qg_fp3_dn7 * ddt_scale));
        let eq176_e2217_d_n8: f64 = (p.p7 * (var_qg_fp3_dn8 * ddt_scale));
        let eq176_e2217_d_n9: f64 = (p.p7 * (var_qg_fp3_dn9 * ddt_scale));
        let eq176_e2217_d_n12: f64 = (p.p7 * (var_qg_fp3_dn12 * ddt_scale));
        let eq176_e2217_d_n14: f64 = (p.p7 * (var_qg_fp3_dn14 * ddt_scale));
        let eq176_e2217_d_n15: f64 = (p.p7 * (var_qg_fp3_dn15 * ddt_scale));
        let eq176_e2217_d_n16: f64 = (p.p7 * (var_qg_fp3_dn16 * ddt_scale));
        let eq176_e2217_d_n17: f64 = (p.p7 * (var_qg_fp3_dn17 * ddt_scale));
        let eq176_e2217_d_n18: f64 = (p.p7 * (var_qg_fp3_dn18 * ddt_scale));
        let eq176_e2217_d_n19: f64 = (p.p7 * (var_qg_fp3_dn19 * ddt_scale));
        let eq176_e2217_d_n20: f64 = (p.p7 * (var_qg_fp3_dn20 * ddt_scale));
        let eq176_e2217_d_n21: f64 = (p.p7 * (var_qg_fp3_dn21 * ddt_scale));
        let eq176_e2217_d_n22: f64 = (p.p7 * (var_qg_fp3_dn22 * ddt_scale));
        let eq176_e2219: f64 = (eq176_e2217 * p.p248);
        let eq176_e2219_d_n0: f64 = (eq176_e2217_d_n0 * p.p248);
        let eq176_e2219_d_n1: f64 = (eq176_e2217_d_n1 * p.p248);
        let eq176_e2219_d_n2: f64 = (eq176_e2217_d_n2 * p.p248);
        let eq176_e2219_d_n3: f64 = (eq176_e2217_d_n3 * p.p248);
        let eq176_e2219_d_n4: f64 = (eq176_e2217_d_n4 * p.p248);
        let eq176_e2219_d_n5: f64 = (eq176_e2217_d_n5 * p.p248);
        let eq176_e2219_d_n6: f64 = (eq176_e2217_d_n6 * p.p248);
        let eq176_e2219_d_n7: f64 = (eq176_e2217_d_n7 * p.p248);
        let eq176_e2219_d_n8: f64 = (eq176_e2217_d_n8 * p.p248);
        let eq176_e2219_d_n9: f64 = (eq176_e2217_d_n9 * p.p248);
        let eq176_e2219_d_n12: f64 = (eq176_e2217_d_n12 * p.p248);
        let eq176_e2219_d_n14: f64 = (eq176_e2217_d_n14 * p.p248);
        let eq176_e2219_d_n15: f64 = (eq176_e2217_d_n15 * p.p248);
        let eq176_e2219_d_n16: f64 = (eq176_e2217_d_n16 * p.p248);
        let eq176_e2219_d_n17: f64 = (eq176_e2217_d_n17 * p.p248);
        let eq176_e2219_d_n18: f64 = (eq176_e2217_d_n18 * p.p248);
        let eq176_e2219_d_n19: f64 = (eq176_e2217_d_n19 * p.p248);
        let eq176_e2219_d_n20: f64 = (eq176_e2217_d_n20 * p.p248);
        let eq176_e2219_d_n21: f64 = (eq176_e2217_d_n21 * p.p248);
        let eq176_e2219_d_n22: f64 = (eq176_e2217_d_n22 * p.p248);
        (eq176_e2219, eq176_e2219_d_n0, eq176_e2219_d_n1, eq176_e2219_d_n2, eq176_e2219_d_n3, eq176_e2219_d_n4, eq176_e2219_d_n5, eq176_e2219_d_n6, eq176_e2219_d_n7, eq176_e2219_d_n8, eq176_e2219_d_n9, eq176_e2219_d_n12, eq176_e2219_d_n14, eq176_e2219_d_n15, eq176_e2219_d_n16, eq176_e2219_d_n17, eq176_e2219_d_n18, eq176_e2219_d_n19, eq176_e2219_d_n20, eq176_e2219_d_n21, eq176_e2219_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq176_value: f64 = eq176_e2221;
        let eq176_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq176_node_derivatives: [f64; 20] = [eq176_e2221_d_n0, eq176_e2221_d_n1, eq176_e2221_d_n2, eq176_e2221_d_n3, eq176_e2221_d_n4, eq176_e2221_d_n5, eq176_e2221_d_n6, eq176_e2221_d_n7, eq176_e2221_d_n8, eq176_e2221_d_n9, eq176_e2221_d_n12, eq176_e2221_d_n14, eq176_e2221_d_n15, eq176_e2221_d_n16, eq176_e2221_d_n17, eq176_e2221_d_n18, eq176_e2221_d_n19, eq176_e2221_d_n20, eq176_e2221_d_n21, eq176_e2221_d_n22];
        let eq176_branch_derivative_indices: [usize; 0] = [];
        let eq176_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq176_value),
            &eq176_node_derivative_indices,
            &eq176_node_derivatives,
            &eq176_branch_derivative_indices,
            &eq176_branch_derivatives,
            multiplicity,
        );
        let (eq177_e2234, eq177_e2234_d_n0, eq177_e2234_d_n1, eq177_e2234_d_n2, eq177_e2234_d_n3, eq177_e2234_d_n4, eq177_e2234_d_n5, eq177_e2234_d_n6, eq177_e2234_d_n7, eq177_e2234_d_n8, eq177_e2234_d_n9, eq177_e2234_d_n12, eq177_e2234_d_n14, eq177_e2234_d_n15, eq177_e2234_d_n16, eq177_e2234_d_n17, eq177_e2234_d_n18, eq177_e2234_d_n19, eq177_e2234_d_n20, eq177_e2234_d_n21, eq177_e2234_d_n22,) = {
    if (((var_guard556 == 0.0) && (var_guard559 != 0.0)) && (var_guard560 == 0.0)) {
        let eq177_e2231: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 76, var_qg_fp3);
        let eq177_e2232: f64 = (p.p7 * eq177_e2231);
        let eq177_e2232_d_n0: f64 = (p.p7 * (var_qg_fp3_dn0 * ddt_scale));
        let eq177_e2232_d_n1: f64 = (p.p7 * (var_qg_fp3_dn1 * ddt_scale));
        let eq177_e2232_d_n2: f64 = (p.p7 * (var_qg_fp3_dn2 * ddt_scale));
        let eq177_e2232_d_n3: f64 = (p.p7 * (var_qg_fp3_dn3 * ddt_scale));
        let eq177_e2232_d_n4: f64 = (p.p7 * (var_qg_fp3_dn4 * ddt_scale));
        let eq177_e2232_d_n5: f64 = (p.p7 * (var_qg_fp3_dn5 * ddt_scale));
        let eq177_e2232_d_n6: f64 = (p.p7 * (var_qg_fp3_dn6 * ddt_scale));
        let eq177_e2232_d_n7: f64 = (p.p7 * (var_qg_fp3_dn7 * ddt_scale));
        let eq177_e2232_d_n8: f64 = (p.p7 * (var_qg_fp3_dn8 * ddt_scale));
        let eq177_e2232_d_n9: f64 = (p.p7 * (var_qg_fp3_dn9 * ddt_scale));
        let eq177_e2232_d_n12: f64 = (p.p7 * (var_qg_fp3_dn12 * ddt_scale));
        let eq177_e2232_d_n14: f64 = (p.p7 * (var_qg_fp3_dn14 * ddt_scale));
        let eq177_e2232_d_n15: f64 = (p.p7 * (var_qg_fp3_dn15 * ddt_scale));
        let eq177_e2232_d_n16: f64 = (p.p7 * (var_qg_fp3_dn16 * ddt_scale));
        let eq177_e2232_d_n17: f64 = (p.p7 * (var_qg_fp3_dn17 * ddt_scale));
        let eq177_e2232_d_n18: f64 = (p.p7 * (var_qg_fp3_dn18 * ddt_scale));
        let eq177_e2232_d_n19: f64 = (p.p7 * (var_qg_fp3_dn19 * ddt_scale));
        let eq177_e2232_d_n20: f64 = (p.p7 * (var_qg_fp3_dn20 * ddt_scale));
        let eq177_e2232_d_n21: f64 = (p.p7 * (var_qg_fp3_dn21 * ddt_scale));
        let eq177_e2232_d_n22: f64 = (p.p7 * (var_qg_fp3_dn22 * ddt_scale));
        (eq177_e2232, eq177_e2232_d_n0, eq177_e2232_d_n1, eq177_e2232_d_n2, eq177_e2232_d_n3, eq177_e2232_d_n4, eq177_e2232_d_n5, eq177_e2232_d_n6, eq177_e2232_d_n7, eq177_e2232_d_n8, eq177_e2232_d_n9, eq177_e2232_d_n12, eq177_e2232_d_n14, eq177_e2232_d_n15, eq177_e2232_d_n16, eq177_e2232_d_n17, eq177_e2232_d_n18, eq177_e2232_d_n19, eq177_e2232_d_n20, eq177_e2232_d_n21, eq177_e2232_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq177_value: f64 = eq177_e2234;
        let eq177_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq177_node_derivatives: [f64; 20] = [eq177_e2234_d_n0, eq177_e2234_d_n1, eq177_e2234_d_n2, eq177_e2234_d_n3, eq177_e2234_d_n4, eq177_e2234_d_n5, eq177_e2234_d_n6, eq177_e2234_d_n7, eq177_e2234_d_n8, eq177_e2234_d_n9, eq177_e2234_d_n12, eq177_e2234_d_n14, eq177_e2234_d_n15, eq177_e2234_d_n16, eq177_e2234_d_n17, eq177_e2234_d_n18, eq177_e2234_d_n19, eq177_e2234_d_n20, eq177_e2234_d_n21, eq177_e2234_d_n22];
        let eq177_branch_derivative_indices: [usize; 0] = [];
        let eq177_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq177_value),
            &eq177_node_derivative_indices,
            &eq177_node_derivatives,
            &eq177_branch_derivative_indices,
            &eq177_branch_derivatives,
            multiplicity,
        );
        let (eq178_e2249, eq178_e2249_d_n0, eq178_e2249_d_n1, eq178_e2249_d_n2, eq178_e2249_d_n3, eq178_e2249_d_n4, eq178_e2249_d_n5, eq178_e2249_d_n6, eq178_e2249_d_n7, eq178_e2249_d_n8, eq178_e2249_d_n9, eq178_e2249_d_n12, eq178_e2249_d_n14, eq178_e2249_d_n15, eq178_e2249_d_n16, eq178_e2249_d_n17, eq178_e2249_d_n18, eq178_e2249_d_n19, eq178_e2249_d_n20, eq178_e2249_d_n21, eq178_e2249_d_n22,) = {
    if (((var_guard556 == 0.0) && (var_guard559 != 0.0)) && (var_guard560 == 0.0)) {
        let eq178_e2244: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 77, var_qg_fp3);
        let eq178_e2245: f64 = (p.p7 * eq178_e2244);
        let eq178_e2245_d_n0: f64 = (p.p7 * (var_qg_fp3_dn0 * ddt_scale));
        let eq178_e2245_d_n1: f64 = (p.p7 * (var_qg_fp3_dn1 * ddt_scale));
        let eq178_e2245_d_n2: f64 = (p.p7 * (var_qg_fp3_dn2 * ddt_scale));
        let eq178_e2245_d_n3: f64 = (p.p7 * (var_qg_fp3_dn3 * ddt_scale));
        let eq178_e2245_d_n4: f64 = (p.p7 * (var_qg_fp3_dn4 * ddt_scale));
        let eq178_e2245_d_n5: f64 = (p.p7 * (var_qg_fp3_dn5 * ddt_scale));
        let eq178_e2245_d_n6: f64 = (p.p7 * (var_qg_fp3_dn6 * ddt_scale));
        let eq178_e2245_d_n7: f64 = (p.p7 * (var_qg_fp3_dn7 * ddt_scale));
        let eq178_e2245_d_n8: f64 = (p.p7 * (var_qg_fp3_dn8 * ddt_scale));
        let eq178_e2245_d_n9: f64 = (p.p7 * (var_qg_fp3_dn9 * ddt_scale));
        let eq178_e2245_d_n12: f64 = (p.p7 * (var_qg_fp3_dn12 * ddt_scale));
        let eq178_e2245_d_n14: f64 = (p.p7 * (var_qg_fp3_dn14 * ddt_scale));
        let eq178_e2245_d_n15: f64 = (p.p7 * (var_qg_fp3_dn15 * ddt_scale));
        let eq178_e2245_d_n16: f64 = (p.p7 * (var_qg_fp3_dn16 * ddt_scale));
        let eq178_e2245_d_n17: f64 = (p.p7 * (var_qg_fp3_dn17 * ddt_scale));
        let eq178_e2245_d_n18: f64 = (p.p7 * (var_qg_fp3_dn18 * ddt_scale));
        let eq178_e2245_d_n19: f64 = (p.p7 * (var_qg_fp3_dn19 * ddt_scale));
        let eq178_e2245_d_n20: f64 = (p.p7 * (var_qg_fp3_dn20 * ddt_scale));
        let eq178_e2245_d_n21: f64 = (p.p7 * (var_qg_fp3_dn21 * ddt_scale));
        let eq178_e2245_d_n22: f64 = (p.p7 * (var_qg_fp3_dn22 * ddt_scale));
        let eq178_e2247: f64 = (eq178_e2245 * p.p248);
        let eq178_e2247_d_n0: f64 = (eq178_e2245_d_n0 * p.p248);
        let eq178_e2247_d_n1: f64 = (eq178_e2245_d_n1 * p.p248);
        let eq178_e2247_d_n2: f64 = (eq178_e2245_d_n2 * p.p248);
        let eq178_e2247_d_n3: f64 = (eq178_e2245_d_n3 * p.p248);
        let eq178_e2247_d_n4: f64 = (eq178_e2245_d_n4 * p.p248);
        let eq178_e2247_d_n5: f64 = (eq178_e2245_d_n5 * p.p248);
        let eq178_e2247_d_n6: f64 = (eq178_e2245_d_n6 * p.p248);
        let eq178_e2247_d_n7: f64 = (eq178_e2245_d_n7 * p.p248);
        let eq178_e2247_d_n8: f64 = (eq178_e2245_d_n8 * p.p248);
        let eq178_e2247_d_n9: f64 = (eq178_e2245_d_n9 * p.p248);
        let eq178_e2247_d_n12: f64 = (eq178_e2245_d_n12 * p.p248);
        let eq178_e2247_d_n14: f64 = (eq178_e2245_d_n14 * p.p248);
        let eq178_e2247_d_n15: f64 = (eq178_e2245_d_n15 * p.p248);
        let eq178_e2247_d_n16: f64 = (eq178_e2245_d_n16 * p.p248);
        let eq178_e2247_d_n17: f64 = (eq178_e2245_d_n17 * p.p248);
        let eq178_e2247_d_n18: f64 = (eq178_e2245_d_n18 * p.p248);
        let eq178_e2247_d_n19: f64 = (eq178_e2245_d_n19 * p.p248);
        let eq178_e2247_d_n20: f64 = (eq178_e2245_d_n20 * p.p248);
        let eq178_e2247_d_n21: f64 = (eq178_e2245_d_n21 * p.p248);
        let eq178_e2247_d_n22: f64 = (eq178_e2245_d_n22 * p.p248);
        (eq178_e2247, eq178_e2247_d_n0, eq178_e2247_d_n1, eq178_e2247_d_n2, eq178_e2247_d_n3, eq178_e2247_d_n4, eq178_e2247_d_n5, eq178_e2247_d_n6, eq178_e2247_d_n7, eq178_e2247_d_n8, eq178_e2247_d_n9, eq178_e2247_d_n12, eq178_e2247_d_n14, eq178_e2247_d_n15, eq178_e2247_d_n16, eq178_e2247_d_n17, eq178_e2247_d_n18, eq178_e2247_d_n19, eq178_e2247_d_n20, eq178_e2247_d_n21, eq178_e2247_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq178_value: f64 = eq178_e2249;
        let eq178_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq178_node_derivatives: [f64; 20] = [eq178_e2249_d_n0, eq178_e2249_d_n1, eq178_e2249_d_n2, eq178_e2249_d_n3, eq178_e2249_d_n4, eq178_e2249_d_n5, eq178_e2249_d_n6, eq178_e2249_d_n7, eq178_e2249_d_n8, eq178_e2249_d_n9, eq178_e2249_d_n12, eq178_e2249_d_n14, eq178_e2249_d_n15, eq178_e2249_d_n16, eq178_e2249_d_n17, eq178_e2249_d_n18, eq178_e2249_d_n19, eq178_e2249_d_n20, eq178_e2249_d_n21, eq178_e2249_d_n22];
        let eq178_branch_derivative_indices: [usize; 0] = [];
        let eq178_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq178_value),
            &eq178_node_derivative_indices,
            &eq178_node_derivatives,
            &eq178_branch_derivative_indices,
            &eq178_branch_derivatives,
            multiplicity,
        );
        let (eq179_e2261, eq179_e2261_d_n0, eq179_e2261_d_n1, eq179_e2261_d_n2, eq179_e2261_d_n3, eq179_e2261_d_n4, eq179_e2261_d_n5, eq179_e2261_d_n6, eq179_e2261_d_n7, eq179_e2261_d_n8, eq179_e2261_d_n9, eq179_e2261_d_n12, eq179_e2261_d_n14, eq179_e2261_d_n15, eq179_e2261_d_n16, eq179_e2261_d_n17, eq179_e2261_d_n18, eq179_e2261_d_n19, eq179_e2261_d_n20, eq179_e2261_d_n21, eq179_e2261_d_n22,) = {
    if ((var_guard556 == 0.0) && (var_guard559 != 0.0)) {
        let eq179_e2257: f64 = (p.p253 * var_qg_fp3);
        let eq179_e2257_d_n0: f64 = (p.p253 * var_qg_fp3_dn0);
        let eq179_e2257_d_n1: f64 = (p.p253 * var_qg_fp3_dn1);
        let eq179_e2257_d_n2: f64 = (p.p253 * var_qg_fp3_dn2);
        let eq179_e2257_d_n3: f64 = (p.p253 * var_qg_fp3_dn3);
        let eq179_e2257_d_n4: f64 = (p.p253 * var_qg_fp3_dn4);
        let eq179_e2257_d_n5: f64 = (p.p253 * var_qg_fp3_dn5);
        let eq179_e2257_d_n6: f64 = (p.p253 * var_qg_fp3_dn6);
        let eq179_e2257_d_n7: f64 = (p.p253 * var_qg_fp3_dn7);
        let eq179_e2257_d_n8: f64 = (p.p253 * var_qg_fp3_dn8);
        let eq179_e2257_d_n9: f64 = (p.p253 * var_qg_fp3_dn9);
        let eq179_e2257_d_n12: f64 = (p.p253 * var_qg_fp3_dn12);
        let eq179_e2257_d_n14: f64 = (p.p253 * var_qg_fp3_dn14);
        let eq179_e2257_d_n15: f64 = (p.p253 * var_qg_fp3_dn15);
        let eq179_e2257_d_n16: f64 = (p.p253 * var_qg_fp3_dn16);
        let eq179_e2257_d_n17: f64 = (p.p253 * var_qg_fp3_dn17);
        let eq179_e2257_d_n18: f64 = (p.p253 * var_qg_fp3_dn18);
        let eq179_e2257_d_n19: f64 = (p.p253 * var_qg_fp3_dn19);
        let eq179_e2257_d_n20: f64 = (p.p253 * var_qg_fp3_dn20);
        let eq179_e2257_d_n21: f64 = (p.p253 * var_qg_fp3_dn21);
        let eq179_e2257_d_n22: f64 = (p.p253 * var_qg_fp3_dn22);
        let eq179_e2258: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 78, eq179_e2257);
        let eq179_e2259: f64 = (p.p7 * eq179_e2258);
        let eq179_e2259_d_n0: f64 = (p.p7 * (eq179_e2257_d_n0 * ddt_scale));
        let eq179_e2259_d_n1: f64 = (p.p7 * (eq179_e2257_d_n1 * ddt_scale));
        let eq179_e2259_d_n2: f64 = (p.p7 * (eq179_e2257_d_n2 * ddt_scale));
        let eq179_e2259_d_n3: f64 = (p.p7 * (eq179_e2257_d_n3 * ddt_scale));
        let eq179_e2259_d_n4: f64 = (p.p7 * (eq179_e2257_d_n4 * ddt_scale));
        let eq179_e2259_d_n5: f64 = (p.p7 * (eq179_e2257_d_n5 * ddt_scale));
        let eq179_e2259_d_n6: f64 = (p.p7 * (eq179_e2257_d_n6 * ddt_scale));
        let eq179_e2259_d_n7: f64 = (p.p7 * (eq179_e2257_d_n7 * ddt_scale));
        let eq179_e2259_d_n8: f64 = (p.p7 * (eq179_e2257_d_n8 * ddt_scale));
        let eq179_e2259_d_n9: f64 = (p.p7 * (eq179_e2257_d_n9 * ddt_scale));
        let eq179_e2259_d_n12: f64 = (p.p7 * (eq179_e2257_d_n12 * ddt_scale));
        let eq179_e2259_d_n14: f64 = (p.p7 * (eq179_e2257_d_n14 * ddt_scale));
        let eq179_e2259_d_n15: f64 = (p.p7 * (eq179_e2257_d_n15 * ddt_scale));
        let eq179_e2259_d_n16: f64 = (p.p7 * (eq179_e2257_d_n16 * ddt_scale));
        let eq179_e2259_d_n17: f64 = (p.p7 * (eq179_e2257_d_n17 * ddt_scale));
        let eq179_e2259_d_n18: f64 = (p.p7 * (eq179_e2257_d_n18 * ddt_scale));
        let eq179_e2259_d_n19: f64 = (p.p7 * (eq179_e2257_d_n19 * ddt_scale));
        let eq179_e2259_d_n20: f64 = (p.p7 * (eq179_e2257_d_n20 * ddt_scale));
        let eq179_e2259_d_n21: f64 = (p.p7 * (eq179_e2257_d_n21 * ddt_scale));
        let eq179_e2259_d_n22: f64 = (p.p7 * (eq179_e2257_d_n22 * ddt_scale));
        (eq179_e2259, eq179_e2259_d_n0, eq179_e2259_d_n1, eq179_e2259_d_n2, eq179_e2259_d_n3, eq179_e2259_d_n4, eq179_e2259_d_n5, eq179_e2259_d_n6, eq179_e2259_d_n7, eq179_e2259_d_n8, eq179_e2259_d_n9, eq179_e2259_d_n12, eq179_e2259_d_n14, eq179_e2259_d_n15, eq179_e2259_d_n16, eq179_e2259_d_n17, eq179_e2259_d_n18, eq179_e2259_d_n19, eq179_e2259_d_n20, eq179_e2259_d_n21, eq179_e2259_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq179_value: f64 = eq179_e2261;
        let eq179_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq179_node_derivatives: [f64; 20] = [eq179_e2261_d_n0, eq179_e2261_d_n1, eq179_e2261_d_n2, eq179_e2261_d_n3, eq179_e2261_d_n4, eq179_e2261_d_n5, eq179_e2261_d_n6, eq179_e2261_d_n7, eq179_e2261_d_n8, eq179_e2261_d_n9, eq179_e2261_d_n12, eq179_e2261_d_n14, eq179_e2261_d_n15, eq179_e2261_d_n16, eq179_e2261_d_n17, eq179_e2261_d_n18, eq179_e2261_d_n19, eq179_e2261_d_n20, eq179_e2261_d_n21, eq179_e2261_d_n22];
        let eq179_branch_derivative_indices: [usize; 0] = [];
        let eq179_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(7),
            multiplicity * (eq179_value),
            &eq179_node_derivative_indices,
            &eq179_node_derivatives,
            &eq179_branch_derivative_indices,
            &eq179_branch_derivatives,
            multiplicity,
        );
        let (eq180_e2270, eq180_e2270_d_n0, eq180_e2270_d_n1, eq180_e2270_d_n2, eq180_e2270_d_n3, eq180_e2270_d_n4, eq180_e2270_d_n5, eq180_e2270_d_n6, eq180_e2270_d_n7, eq180_e2270_d_n8, eq180_e2270_d_n9, eq180_e2270_d_n12, eq180_e2270_d_n14, eq180_e2270_d_n15, eq180_e2270_d_n16, eq180_e2270_d_n17, eq180_e2270_d_n18, eq180_e2270_d_n19, eq180_e2270_d_n20, eq180_e2270_d_n21, eq180_e2270_d_n22,) = {
    if ((var_guard561 != 0.0) && (var_guard562 != 0.0)) {
        let eq180_e2267: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 79, var_qd_fp3s);
        let eq180_e2268: f64 = (p.p7 * eq180_e2267);
        let eq180_e2268_d_n0: f64 = (p.p7 * (var_qd_fp3s_dn0 * ddt_scale));
        let eq180_e2268_d_n1: f64 = (p.p7 * (var_qd_fp3s_dn1 * ddt_scale));
        let eq180_e2268_d_n2: f64 = (p.p7 * (var_qd_fp3s_dn2 * ddt_scale));
        let eq180_e2268_d_n3: f64 = (p.p7 * (var_qd_fp3s_dn3 * ddt_scale));
        let eq180_e2268_d_n4: f64 = (p.p7 * (var_qd_fp3s_dn4 * ddt_scale));
        let eq180_e2268_d_n5: f64 = (p.p7 * (var_qd_fp3s_dn5 * ddt_scale));
        let eq180_e2268_d_n6: f64 = (p.p7 * (var_qd_fp3s_dn6 * ddt_scale));
        let eq180_e2268_d_n7: f64 = (p.p7 * (var_qd_fp3s_dn7 * ddt_scale));
        let eq180_e2268_d_n8: f64 = (p.p7 * (var_qd_fp3s_dn8 * ddt_scale));
        let eq180_e2268_d_n9: f64 = (p.p7 * (var_qd_fp3s_dn9 * ddt_scale));
        let eq180_e2268_d_n12: f64 = (p.p7 * (var_qd_fp3s_dn12 * ddt_scale));
        let eq180_e2268_d_n14: f64 = (p.p7 * (var_qd_fp3s_dn14 * ddt_scale));
        let eq180_e2268_d_n15: f64 = (p.p7 * (var_qd_fp3s_dn15 * ddt_scale));
        let eq180_e2268_d_n16: f64 = (p.p7 * (var_qd_fp3s_dn16 * ddt_scale));
        let eq180_e2268_d_n17: f64 = (p.p7 * (var_qd_fp3s_dn17 * ddt_scale));
        let eq180_e2268_d_n18: f64 = (p.p7 * (var_qd_fp3s_dn18 * ddt_scale));
        let eq180_e2268_d_n19: f64 = (p.p7 * (var_qd_fp3s_dn19 * ddt_scale));
        let eq180_e2268_d_n20: f64 = (p.p7 * (var_qd_fp3s_dn20 * ddt_scale));
        let eq180_e2268_d_n21: f64 = (p.p7 * (var_qd_fp3s_dn21 * ddt_scale));
        let eq180_e2268_d_n22: f64 = (p.p7 * (var_qd_fp3s_dn22 * ddt_scale));
        (eq180_e2268, eq180_e2268_d_n0, eq180_e2268_d_n1, eq180_e2268_d_n2, eq180_e2268_d_n3, eq180_e2268_d_n4, eq180_e2268_d_n5, eq180_e2268_d_n6, eq180_e2268_d_n7, eq180_e2268_d_n8, eq180_e2268_d_n9, eq180_e2268_d_n12, eq180_e2268_d_n14, eq180_e2268_d_n15, eq180_e2268_d_n16, eq180_e2268_d_n17, eq180_e2268_d_n18, eq180_e2268_d_n19, eq180_e2268_d_n20, eq180_e2268_d_n21, eq180_e2268_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq180_value: f64 = eq180_e2270;
        let eq180_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq180_node_derivatives: [f64; 20] = [eq180_e2270_d_n0, eq180_e2270_d_n1, eq180_e2270_d_n2, eq180_e2270_d_n3, eq180_e2270_d_n4, eq180_e2270_d_n5, eq180_e2270_d_n6, eq180_e2270_d_n7, eq180_e2270_d_n8, eq180_e2270_d_n9, eq180_e2270_d_n12, eq180_e2270_d_n14, eq180_e2270_d_n15, eq180_e2270_d_n16, eq180_e2270_d_n17, eq180_e2270_d_n18, eq180_e2270_d_n19, eq180_e2270_d_n20, eq180_e2270_d_n21, eq180_e2270_d_n22];
        let eq180_branch_derivative_indices: [usize; 0] = [];
        let eq180_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(21),
            multiplicity * (eq180_value),
            &eq180_node_derivative_indices,
            &eq180_node_derivatives,
            &eq180_branch_derivative_indices,
            &eq180_branch_derivatives,
            multiplicity,
        );
        let (eq181_e2281, eq181_e2281_d_n0, eq181_e2281_d_n1, eq181_e2281_d_n2, eq181_e2281_d_n3, eq181_e2281_d_n4, eq181_e2281_d_n5, eq181_e2281_d_n6, eq181_e2281_d_n7, eq181_e2281_d_n8, eq181_e2281_d_n9, eq181_e2281_d_n12, eq181_e2281_d_n14, eq181_e2281_d_n15, eq181_e2281_d_n16, eq181_e2281_d_n17, eq181_e2281_d_n18, eq181_e2281_d_n19, eq181_e2281_d_n20, eq181_e2281_d_n21, eq181_e2281_d_n22,) = {
    if (((var_guard561 != 0.0) && (var_guard562 != 0.0)) && (var_guard563 != 0.0)) {
        let eq181_e2278: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 80, var_qg_fp3s);
        let eq181_e2279: f64 = (p.p7 * eq181_e2278);
        let eq181_e2279_d_n0: f64 = (p.p7 * (var_qg_fp3s_dn0 * ddt_scale));
        let eq181_e2279_d_n1: f64 = (p.p7 * (var_qg_fp3s_dn1 * ddt_scale));
        let eq181_e2279_d_n2: f64 = (p.p7 * (var_qg_fp3s_dn2 * ddt_scale));
        let eq181_e2279_d_n3: f64 = (p.p7 * (var_qg_fp3s_dn3 * ddt_scale));
        let eq181_e2279_d_n4: f64 = (p.p7 * (var_qg_fp3s_dn4 * ddt_scale));
        let eq181_e2279_d_n5: f64 = (p.p7 * (var_qg_fp3s_dn5 * ddt_scale));
        let eq181_e2279_d_n6: f64 = (p.p7 * (var_qg_fp3s_dn6 * ddt_scale));
        let eq181_e2279_d_n7: f64 = (p.p7 * (var_qg_fp3s_dn7 * ddt_scale));
        let eq181_e2279_d_n8: f64 = (p.p7 * (var_qg_fp3s_dn8 * ddt_scale));
        let eq181_e2279_d_n9: f64 = (p.p7 * (var_qg_fp3s_dn9 * ddt_scale));
        let eq181_e2279_d_n12: f64 = (p.p7 * (var_qg_fp3s_dn12 * ddt_scale));
        let eq181_e2279_d_n14: f64 = (p.p7 * (var_qg_fp3s_dn14 * ddt_scale));
        let eq181_e2279_d_n15: f64 = (p.p7 * (var_qg_fp3s_dn15 * ddt_scale));
        let eq181_e2279_d_n16: f64 = (p.p7 * (var_qg_fp3s_dn16 * ddt_scale));
        let eq181_e2279_d_n17: f64 = (p.p7 * (var_qg_fp3s_dn17 * ddt_scale));
        let eq181_e2279_d_n18: f64 = (p.p7 * (var_qg_fp3s_dn18 * ddt_scale));
        let eq181_e2279_d_n19: f64 = (p.p7 * (var_qg_fp3s_dn19 * ddt_scale));
        let eq181_e2279_d_n20: f64 = (p.p7 * (var_qg_fp3s_dn20 * ddt_scale));
        let eq181_e2279_d_n21: f64 = (p.p7 * (var_qg_fp3s_dn21 * ddt_scale));
        let eq181_e2279_d_n22: f64 = (p.p7 * (var_qg_fp3s_dn22 * ddt_scale));
        (eq181_e2279, eq181_e2279_d_n0, eq181_e2279_d_n1, eq181_e2279_d_n2, eq181_e2279_d_n3, eq181_e2279_d_n4, eq181_e2279_d_n5, eq181_e2279_d_n6, eq181_e2279_d_n7, eq181_e2279_d_n8, eq181_e2279_d_n9, eq181_e2279_d_n12, eq181_e2279_d_n14, eq181_e2279_d_n15, eq181_e2279_d_n16, eq181_e2279_d_n17, eq181_e2279_d_n18, eq181_e2279_d_n19, eq181_e2279_d_n20, eq181_e2279_d_n21, eq181_e2279_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq181_value: f64 = eq181_e2281;
        let eq181_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq181_node_derivatives: [f64; 20] = [eq181_e2281_d_n0, eq181_e2281_d_n1, eq181_e2281_d_n2, eq181_e2281_d_n3, eq181_e2281_d_n4, eq181_e2281_d_n5, eq181_e2281_d_n6, eq181_e2281_d_n7, eq181_e2281_d_n8, eq181_e2281_d_n9, eq181_e2281_d_n12, eq181_e2281_d_n14, eq181_e2281_d_n15, eq181_e2281_d_n16, eq181_e2281_d_n17, eq181_e2281_d_n18, eq181_e2281_d_n19, eq181_e2281_d_n20, eq181_e2281_d_n21, eq181_e2281_d_n22];
        let eq181_branch_derivative_indices: [usize; 0] = [];
        let eq181_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(21),
            multiplicity * (eq181_value),
            &eq181_node_derivative_indices,
            &eq181_node_derivatives,
            &eq181_branch_derivative_indices,
            &eq181_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_10(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
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
        var_guard561: f64,
        var_guard562: f64,
        var_guard563: f64,
        var_guard564: f64,
        var_guard565: f64,
        var_qd_fp3s: f64,
        var_qd_fp3s_dn0: f64,
        var_qd_fp3s_dn1: f64,
        var_qd_fp3s_dn12: f64,
        var_qd_fp3s_dn14: f64,
        var_qd_fp3s_dn15: f64,
        var_qd_fp3s_dn16: f64,
        var_qd_fp3s_dn17: f64,
        var_qd_fp3s_dn18: f64,
        var_qd_fp3s_dn19: f64,
        var_qd_fp3s_dn2: f64,
        var_qd_fp3s_dn20: f64,
        var_qd_fp3s_dn21: f64,
        var_qd_fp3s_dn22: f64,
        var_qd_fp3s_dn3: f64,
        var_qd_fp3s_dn4: f64,
        var_qd_fp3s_dn5: f64,
        var_qd_fp3s_dn6: f64,
        var_qd_fp3s_dn7: f64,
        var_qd_fp3s_dn8: f64,
        var_qd_fp3s_dn9: f64,
        var_qg_fp3s: f64,
        var_qg_fp3s_dn0: f64,
        var_qg_fp3s_dn1: f64,
        var_qg_fp3s_dn12: f64,
        var_qg_fp3s_dn14: f64,
        var_qg_fp3s_dn15: f64,
        var_qg_fp3s_dn16: f64,
        var_qg_fp3s_dn17: f64,
        var_qg_fp3s_dn18: f64,
        var_qg_fp3s_dn19: f64,
        var_qg_fp3s_dn2: f64,
        var_qg_fp3s_dn20: f64,
        var_qg_fp3s_dn21: f64,
        var_qg_fp3s_dn22: f64,
        var_qg_fp3s_dn3: f64,
        var_qg_fp3s_dn4: f64,
        var_qg_fp3s_dn5: f64,
        var_qg_fp3s_dn6: f64,
        var_qg_fp3s_dn7: f64,
        var_qg_fp3s_dn8: f64,
        var_qg_fp3s_dn9: f64,
    ) {
        let (eq182_e2294, eq182_e2294_d_n0, eq182_e2294_d_n1, eq182_e2294_d_n2, eq182_e2294_d_n3, eq182_e2294_d_n4, eq182_e2294_d_n5, eq182_e2294_d_n6, eq182_e2294_d_n7, eq182_e2294_d_n8, eq182_e2294_d_n9, eq182_e2294_d_n12, eq182_e2294_d_n14, eq182_e2294_d_n15, eq182_e2294_d_n16, eq182_e2294_d_n17, eq182_e2294_d_n18, eq182_e2294_d_n19, eq182_e2294_d_n20, eq182_e2294_d_n21, eq182_e2294_d_n22,) = {
    if (((var_guard561 != 0.0) && (var_guard562 != 0.0)) && (var_guard563 != 0.0)) {
        let eq182_e2289: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 81, var_qg_fp3s);
        let eq182_e2290: f64 = (p.p7 * eq182_e2289);
        let eq182_e2290_d_n0: f64 = (p.p7 * (var_qg_fp3s_dn0 * ddt_scale));
        let eq182_e2290_d_n1: f64 = (p.p7 * (var_qg_fp3s_dn1 * ddt_scale));
        let eq182_e2290_d_n2: f64 = (p.p7 * (var_qg_fp3s_dn2 * ddt_scale));
        let eq182_e2290_d_n3: f64 = (p.p7 * (var_qg_fp3s_dn3 * ddt_scale));
        let eq182_e2290_d_n4: f64 = (p.p7 * (var_qg_fp3s_dn4 * ddt_scale));
        let eq182_e2290_d_n5: f64 = (p.p7 * (var_qg_fp3s_dn5 * ddt_scale));
        let eq182_e2290_d_n6: f64 = (p.p7 * (var_qg_fp3s_dn6 * ddt_scale));
        let eq182_e2290_d_n7: f64 = (p.p7 * (var_qg_fp3s_dn7 * ddt_scale));
        let eq182_e2290_d_n8: f64 = (p.p7 * (var_qg_fp3s_dn8 * ddt_scale));
        let eq182_e2290_d_n9: f64 = (p.p7 * (var_qg_fp3s_dn9 * ddt_scale));
        let eq182_e2290_d_n12: f64 = (p.p7 * (var_qg_fp3s_dn12 * ddt_scale));
        let eq182_e2290_d_n14: f64 = (p.p7 * (var_qg_fp3s_dn14 * ddt_scale));
        let eq182_e2290_d_n15: f64 = (p.p7 * (var_qg_fp3s_dn15 * ddt_scale));
        let eq182_e2290_d_n16: f64 = (p.p7 * (var_qg_fp3s_dn16 * ddt_scale));
        let eq182_e2290_d_n17: f64 = (p.p7 * (var_qg_fp3s_dn17 * ddt_scale));
        let eq182_e2290_d_n18: f64 = (p.p7 * (var_qg_fp3s_dn18 * ddt_scale));
        let eq182_e2290_d_n19: f64 = (p.p7 * (var_qg_fp3s_dn19 * ddt_scale));
        let eq182_e2290_d_n20: f64 = (p.p7 * (var_qg_fp3s_dn20 * ddt_scale));
        let eq182_e2290_d_n21: f64 = (p.p7 * (var_qg_fp3s_dn21 * ddt_scale));
        let eq182_e2290_d_n22: f64 = (p.p7 * (var_qg_fp3s_dn22 * ddt_scale));
        let eq182_e2292: f64 = (eq182_e2290 * p.p248);
        let eq182_e2292_d_n0: f64 = (eq182_e2290_d_n0 * p.p248);
        let eq182_e2292_d_n1: f64 = (eq182_e2290_d_n1 * p.p248);
        let eq182_e2292_d_n2: f64 = (eq182_e2290_d_n2 * p.p248);
        let eq182_e2292_d_n3: f64 = (eq182_e2290_d_n3 * p.p248);
        let eq182_e2292_d_n4: f64 = (eq182_e2290_d_n4 * p.p248);
        let eq182_e2292_d_n5: f64 = (eq182_e2290_d_n5 * p.p248);
        let eq182_e2292_d_n6: f64 = (eq182_e2290_d_n6 * p.p248);
        let eq182_e2292_d_n7: f64 = (eq182_e2290_d_n7 * p.p248);
        let eq182_e2292_d_n8: f64 = (eq182_e2290_d_n8 * p.p248);
        let eq182_e2292_d_n9: f64 = (eq182_e2290_d_n9 * p.p248);
        let eq182_e2292_d_n12: f64 = (eq182_e2290_d_n12 * p.p248);
        let eq182_e2292_d_n14: f64 = (eq182_e2290_d_n14 * p.p248);
        let eq182_e2292_d_n15: f64 = (eq182_e2290_d_n15 * p.p248);
        let eq182_e2292_d_n16: f64 = (eq182_e2290_d_n16 * p.p248);
        let eq182_e2292_d_n17: f64 = (eq182_e2290_d_n17 * p.p248);
        let eq182_e2292_d_n18: f64 = (eq182_e2290_d_n18 * p.p248);
        let eq182_e2292_d_n19: f64 = (eq182_e2290_d_n19 * p.p248);
        let eq182_e2292_d_n20: f64 = (eq182_e2290_d_n20 * p.p248);
        let eq182_e2292_d_n21: f64 = (eq182_e2290_d_n21 * p.p248);
        let eq182_e2292_d_n22: f64 = (eq182_e2290_d_n22 * p.p248);
        (eq182_e2292, eq182_e2292_d_n0, eq182_e2292_d_n1, eq182_e2292_d_n2, eq182_e2292_d_n3, eq182_e2292_d_n4, eq182_e2292_d_n5, eq182_e2292_d_n6, eq182_e2292_d_n7, eq182_e2292_d_n8, eq182_e2292_d_n9, eq182_e2292_d_n12, eq182_e2292_d_n14, eq182_e2292_d_n15, eq182_e2292_d_n16, eq182_e2292_d_n17, eq182_e2292_d_n18, eq182_e2292_d_n19, eq182_e2292_d_n20, eq182_e2292_d_n21, eq182_e2292_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq182_value: f64 = eq182_e2294;
        let eq182_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq182_node_derivatives: [f64; 20] = [eq182_e2294_d_n0, eq182_e2294_d_n1, eq182_e2294_d_n2, eq182_e2294_d_n3, eq182_e2294_d_n4, eq182_e2294_d_n5, eq182_e2294_d_n6, eq182_e2294_d_n7, eq182_e2294_d_n8, eq182_e2294_d_n9, eq182_e2294_d_n12, eq182_e2294_d_n14, eq182_e2294_d_n15, eq182_e2294_d_n16, eq182_e2294_d_n17, eq182_e2294_d_n18, eq182_e2294_d_n19, eq182_e2294_d_n20, eq182_e2294_d_n21, eq182_e2294_d_n22];
        let eq182_branch_derivative_indices: [usize; 0] = [];
        let eq182_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(21),
            multiplicity * (eq182_value),
            &eq182_node_derivative_indices,
            &eq182_node_derivatives,
            &eq182_branch_derivative_indices,
            &eq182_branch_derivatives,
            multiplicity,
        );
        let (eq183_e2306, eq183_e2306_d_n0, eq183_e2306_d_n1, eq183_e2306_d_n2, eq183_e2306_d_n3, eq183_e2306_d_n4, eq183_e2306_d_n5, eq183_e2306_d_n6, eq183_e2306_d_n7, eq183_e2306_d_n8, eq183_e2306_d_n9, eq183_e2306_d_n12, eq183_e2306_d_n14, eq183_e2306_d_n15, eq183_e2306_d_n16, eq183_e2306_d_n17, eq183_e2306_d_n18, eq183_e2306_d_n19, eq183_e2306_d_n20, eq183_e2306_d_n21, eq183_e2306_d_n22,) = {
    if (((var_guard561 != 0.0) && (var_guard562 != 0.0)) && (var_guard563 == 0.0)) {
        let eq183_e2303: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 82, var_qg_fp3s);
        let eq183_e2304: f64 = (p.p7 * eq183_e2303);
        let eq183_e2304_d_n0: f64 = (p.p7 * (var_qg_fp3s_dn0 * ddt_scale));
        let eq183_e2304_d_n1: f64 = (p.p7 * (var_qg_fp3s_dn1 * ddt_scale));
        let eq183_e2304_d_n2: f64 = (p.p7 * (var_qg_fp3s_dn2 * ddt_scale));
        let eq183_e2304_d_n3: f64 = (p.p7 * (var_qg_fp3s_dn3 * ddt_scale));
        let eq183_e2304_d_n4: f64 = (p.p7 * (var_qg_fp3s_dn4 * ddt_scale));
        let eq183_e2304_d_n5: f64 = (p.p7 * (var_qg_fp3s_dn5 * ddt_scale));
        let eq183_e2304_d_n6: f64 = (p.p7 * (var_qg_fp3s_dn6 * ddt_scale));
        let eq183_e2304_d_n7: f64 = (p.p7 * (var_qg_fp3s_dn7 * ddt_scale));
        let eq183_e2304_d_n8: f64 = (p.p7 * (var_qg_fp3s_dn8 * ddt_scale));
        let eq183_e2304_d_n9: f64 = (p.p7 * (var_qg_fp3s_dn9 * ddt_scale));
        let eq183_e2304_d_n12: f64 = (p.p7 * (var_qg_fp3s_dn12 * ddt_scale));
        let eq183_e2304_d_n14: f64 = (p.p7 * (var_qg_fp3s_dn14 * ddt_scale));
        let eq183_e2304_d_n15: f64 = (p.p7 * (var_qg_fp3s_dn15 * ddt_scale));
        let eq183_e2304_d_n16: f64 = (p.p7 * (var_qg_fp3s_dn16 * ddt_scale));
        let eq183_e2304_d_n17: f64 = (p.p7 * (var_qg_fp3s_dn17 * ddt_scale));
        let eq183_e2304_d_n18: f64 = (p.p7 * (var_qg_fp3s_dn18 * ddt_scale));
        let eq183_e2304_d_n19: f64 = (p.p7 * (var_qg_fp3s_dn19 * ddt_scale));
        let eq183_e2304_d_n20: f64 = (p.p7 * (var_qg_fp3s_dn20 * ddt_scale));
        let eq183_e2304_d_n21: f64 = (p.p7 * (var_qg_fp3s_dn21 * ddt_scale));
        let eq183_e2304_d_n22: f64 = (p.p7 * (var_qg_fp3s_dn22 * ddt_scale));
        (eq183_e2304, eq183_e2304_d_n0, eq183_e2304_d_n1, eq183_e2304_d_n2, eq183_e2304_d_n3, eq183_e2304_d_n4, eq183_e2304_d_n5, eq183_e2304_d_n6, eq183_e2304_d_n7, eq183_e2304_d_n8, eq183_e2304_d_n9, eq183_e2304_d_n12, eq183_e2304_d_n14, eq183_e2304_d_n15, eq183_e2304_d_n16, eq183_e2304_d_n17, eq183_e2304_d_n18, eq183_e2304_d_n19, eq183_e2304_d_n20, eq183_e2304_d_n21, eq183_e2304_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq183_value: f64 = eq183_e2306;
        let eq183_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq183_node_derivatives: [f64; 20] = [eq183_e2306_d_n0, eq183_e2306_d_n1, eq183_e2306_d_n2, eq183_e2306_d_n3, eq183_e2306_d_n4, eq183_e2306_d_n5, eq183_e2306_d_n6, eq183_e2306_d_n7, eq183_e2306_d_n8, eq183_e2306_d_n9, eq183_e2306_d_n12, eq183_e2306_d_n14, eq183_e2306_d_n15, eq183_e2306_d_n16, eq183_e2306_d_n17, eq183_e2306_d_n18, eq183_e2306_d_n19, eq183_e2306_d_n20, eq183_e2306_d_n21, eq183_e2306_d_n22];
        let eq183_branch_derivative_indices: [usize; 0] = [];
        let eq183_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(21),
            multiplicity * (eq183_value),
            &eq183_node_derivative_indices,
            &eq183_node_derivatives,
            &eq183_branch_derivative_indices,
            &eq183_branch_derivatives,
            multiplicity,
        );
        let (eq184_e2320, eq184_e2320_d_n0, eq184_e2320_d_n1, eq184_e2320_d_n2, eq184_e2320_d_n3, eq184_e2320_d_n4, eq184_e2320_d_n5, eq184_e2320_d_n6, eq184_e2320_d_n7, eq184_e2320_d_n8, eq184_e2320_d_n9, eq184_e2320_d_n12, eq184_e2320_d_n14, eq184_e2320_d_n15, eq184_e2320_d_n16, eq184_e2320_d_n17, eq184_e2320_d_n18, eq184_e2320_d_n19, eq184_e2320_d_n20, eq184_e2320_d_n21, eq184_e2320_d_n22,) = {
    if (((var_guard561 != 0.0) && (var_guard562 != 0.0)) && (var_guard563 == 0.0)) {
        let eq184_e2315: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 83, var_qg_fp3s);
        let eq184_e2316: f64 = (p.p7 * eq184_e2315);
        let eq184_e2316_d_n0: f64 = (p.p7 * (var_qg_fp3s_dn0 * ddt_scale));
        let eq184_e2316_d_n1: f64 = (p.p7 * (var_qg_fp3s_dn1 * ddt_scale));
        let eq184_e2316_d_n2: f64 = (p.p7 * (var_qg_fp3s_dn2 * ddt_scale));
        let eq184_e2316_d_n3: f64 = (p.p7 * (var_qg_fp3s_dn3 * ddt_scale));
        let eq184_e2316_d_n4: f64 = (p.p7 * (var_qg_fp3s_dn4 * ddt_scale));
        let eq184_e2316_d_n5: f64 = (p.p7 * (var_qg_fp3s_dn5 * ddt_scale));
        let eq184_e2316_d_n6: f64 = (p.p7 * (var_qg_fp3s_dn6 * ddt_scale));
        let eq184_e2316_d_n7: f64 = (p.p7 * (var_qg_fp3s_dn7 * ddt_scale));
        let eq184_e2316_d_n8: f64 = (p.p7 * (var_qg_fp3s_dn8 * ddt_scale));
        let eq184_e2316_d_n9: f64 = (p.p7 * (var_qg_fp3s_dn9 * ddt_scale));
        let eq184_e2316_d_n12: f64 = (p.p7 * (var_qg_fp3s_dn12 * ddt_scale));
        let eq184_e2316_d_n14: f64 = (p.p7 * (var_qg_fp3s_dn14 * ddt_scale));
        let eq184_e2316_d_n15: f64 = (p.p7 * (var_qg_fp3s_dn15 * ddt_scale));
        let eq184_e2316_d_n16: f64 = (p.p7 * (var_qg_fp3s_dn16 * ddt_scale));
        let eq184_e2316_d_n17: f64 = (p.p7 * (var_qg_fp3s_dn17 * ddt_scale));
        let eq184_e2316_d_n18: f64 = (p.p7 * (var_qg_fp3s_dn18 * ddt_scale));
        let eq184_e2316_d_n19: f64 = (p.p7 * (var_qg_fp3s_dn19 * ddt_scale));
        let eq184_e2316_d_n20: f64 = (p.p7 * (var_qg_fp3s_dn20 * ddt_scale));
        let eq184_e2316_d_n21: f64 = (p.p7 * (var_qg_fp3s_dn21 * ddt_scale));
        let eq184_e2316_d_n22: f64 = (p.p7 * (var_qg_fp3s_dn22 * ddt_scale));
        let eq184_e2318: f64 = (eq184_e2316 * p.p248);
        let eq184_e2318_d_n0: f64 = (eq184_e2316_d_n0 * p.p248);
        let eq184_e2318_d_n1: f64 = (eq184_e2316_d_n1 * p.p248);
        let eq184_e2318_d_n2: f64 = (eq184_e2316_d_n2 * p.p248);
        let eq184_e2318_d_n3: f64 = (eq184_e2316_d_n3 * p.p248);
        let eq184_e2318_d_n4: f64 = (eq184_e2316_d_n4 * p.p248);
        let eq184_e2318_d_n5: f64 = (eq184_e2316_d_n5 * p.p248);
        let eq184_e2318_d_n6: f64 = (eq184_e2316_d_n6 * p.p248);
        let eq184_e2318_d_n7: f64 = (eq184_e2316_d_n7 * p.p248);
        let eq184_e2318_d_n8: f64 = (eq184_e2316_d_n8 * p.p248);
        let eq184_e2318_d_n9: f64 = (eq184_e2316_d_n9 * p.p248);
        let eq184_e2318_d_n12: f64 = (eq184_e2316_d_n12 * p.p248);
        let eq184_e2318_d_n14: f64 = (eq184_e2316_d_n14 * p.p248);
        let eq184_e2318_d_n15: f64 = (eq184_e2316_d_n15 * p.p248);
        let eq184_e2318_d_n16: f64 = (eq184_e2316_d_n16 * p.p248);
        let eq184_e2318_d_n17: f64 = (eq184_e2316_d_n17 * p.p248);
        let eq184_e2318_d_n18: f64 = (eq184_e2316_d_n18 * p.p248);
        let eq184_e2318_d_n19: f64 = (eq184_e2316_d_n19 * p.p248);
        let eq184_e2318_d_n20: f64 = (eq184_e2316_d_n20 * p.p248);
        let eq184_e2318_d_n21: f64 = (eq184_e2316_d_n21 * p.p248);
        let eq184_e2318_d_n22: f64 = (eq184_e2316_d_n22 * p.p248);
        (eq184_e2318, eq184_e2318_d_n0, eq184_e2318_d_n1, eq184_e2318_d_n2, eq184_e2318_d_n3, eq184_e2318_d_n4, eq184_e2318_d_n5, eq184_e2318_d_n6, eq184_e2318_d_n7, eq184_e2318_d_n8, eq184_e2318_d_n9, eq184_e2318_d_n12, eq184_e2318_d_n14, eq184_e2318_d_n15, eq184_e2318_d_n16, eq184_e2318_d_n17, eq184_e2318_d_n18, eq184_e2318_d_n19, eq184_e2318_d_n20, eq184_e2318_d_n21, eq184_e2318_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq184_value: f64 = eq184_e2320;
        let eq184_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq184_node_derivatives: [f64; 20] = [eq184_e2320_d_n0, eq184_e2320_d_n1, eq184_e2320_d_n2, eq184_e2320_d_n3, eq184_e2320_d_n4, eq184_e2320_d_n5, eq184_e2320_d_n6, eq184_e2320_d_n7, eq184_e2320_d_n8, eq184_e2320_d_n9, eq184_e2320_d_n12, eq184_e2320_d_n14, eq184_e2320_d_n15, eq184_e2320_d_n16, eq184_e2320_d_n17, eq184_e2320_d_n18, eq184_e2320_d_n19, eq184_e2320_d_n20, eq184_e2320_d_n21, eq184_e2320_d_n22];
        let eq184_branch_derivative_indices: [usize; 0] = [];
        let eq184_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(21),
            multiplicity * (eq184_value),
            &eq184_node_derivative_indices,
            &eq184_node_derivatives,
            &eq184_branch_derivative_indices,
            &eq184_branch_derivatives,
            multiplicity,
        );
        let (eq185_e2331, eq185_e2331_d_n0, eq185_e2331_d_n1, eq185_e2331_d_n2, eq185_e2331_d_n3, eq185_e2331_d_n4, eq185_e2331_d_n5, eq185_e2331_d_n6, eq185_e2331_d_n7, eq185_e2331_d_n8, eq185_e2331_d_n9, eq185_e2331_d_n12, eq185_e2331_d_n14, eq185_e2331_d_n15, eq185_e2331_d_n16, eq185_e2331_d_n17, eq185_e2331_d_n18, eq185_e2331_d_n19, eq185_e2331_d_n20, eq185_e2331_d_n21, eq185_e2331_d_n22,) = {
    if ((var_guard561 != 0.0) && (var_guard562 != 0.0)) {
        let eq185_e2327: f64 = (p.p253 * var_qg_fp3s);
        let eq185_e2327_d_n0: f64 = (p.p253 * var_qg_fp3s_dn0);
        let eq185_e2327_d_n1: f64 = (p.p253 * var_qg_fp3s_dn1);
        let eq185_e2327_d_n2: f64 = (p.p253 * var_qg_fp3s_dn2);
        let eq185_e2327_d_n3: f64 = (p.p253 * var_qg_fp3s_dn3);
        let eq185_e2327_d_n4: f64 = (p.p253 * var_qg_fp3s_dn4);
        let eq185_e2327_d_n5: f64 = (p.p253 * var_qg_fp3s_dn5);
        let eq185_e2327_d_n6: f64 = (p.p253 * var_qg_fp3s_dn6);
        let eq185_e2327_d_n7: f64 = (p.p253 * var_qg_fp3s_dn7);
        let eq185_e2327_d_n8: f64 = (p.p253 * var_qg_fp3s_dn8);
        let eq185_e2327_d_n9: f64 = (p.p253 * var_qg_fp3s_dn9);
        let eq185_e2327_d_n12: f64 = (p.p253 * var_qg_fp3s_dn12);
        let eq185_e2327_d_n14: f64 = (p.p253 * var_qg_fp3s_dn14);
        let eq185_e2327_d_n15: f64 = (p.p253 * var_qg_fp3s_dn15);
        let eq185_e2327_d_n16: f64 = (p.p253 * var_qg_fp3s_dn16);
        let eq185_e2327_d_n17: f64 = (p.p253 * var_qg_fp3s_dn17);
        let eq185_e2327_d_n18: f64 = (p.p253 * var_qg_fp3s_dn18);
        let eq185_e2327_d_n19: f64 = (p.p253 * var_qg_fp3s_dn19);
        let eq185_e2327_d_n20: f64 = (p.p253 * var_qg_fp3s_dn20);
        let eq185_e2327_d_n21: f64 = (p.p253 * var_qg_fp3s_dn21);
        let eq185_e2327_d_n22: f64 = (p.p253 * var_qg_fp3s_dn22);
        let eq185_e2328: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 84, eq185_e2327);
        let eq185_e2329: f64 = (p.p7 * eq185_e2328);
        let eq185_e2329_d_n0: f64 = (p.p7 * (eq185_e2327_d_n0 * ddt_scale));
        let eq185_e2329_d_n1: f64 = (p.p7 * (eq185_e2327_d_n1 * ddt_scale));
        let eq185_e2329_d_n2: f64 = (p.p7 * (eq185_e2327_d_n2 * ddt_scale));
        let eq185_e2329_d_n3: f64 = (p.p7 * (eq185_e2327_d_n3 * ddt_scale));
        let eq185_e2329_d_n4: f64 = (p.p7 * (eq185_e2327_d_n4 * ddt_scale));
        let eq185_e2329_d_n5: f64 = (p.p7 * (eq185_e2327_d_n5 * ddt_scale));
        let eq185_e2329_d_n6: f64 = (p.p7 * (eq185_e2327_d_n6 * ddt_scale));
        let eq185_e2329_d_n7: f64 = (p.p7 * (eq185_e2327_d_n7 * ddt_scale));
        let eq185_e2329_d_n8: f64 = (p.p7 * (eq185_e2327_d_n8 * ddt_scale));
        let eq185_e2329_d_n9: f64 = (p.p7 * (eq185_e2327_d_n9 * ddt_scale));
        let eq185_e2329_d_n12: f64 = (p.p7 * (eq185_e2327_d_n12 * ddt_scale));
        let eq185_e2329_d_n14: f64 = (p.p7 * (eq185_e2327_d_n14 * ddt_scale));
        let eq185_e2329_d_n15: f64 = (p.p7 * (eq185_e2327_d_n15 * ddt_scale));
        let eq185_e2329_d_n16: f64 = (p.p7 * (eq185_e2327_d_n16 * ddt_scale));
        let eq185_e2329_d_n17: f64 = (p.p7 * (eq185_e2327_d_n17 * ddt_scale));
        let eq185_e2329_d_n18: f64 = (p.p7 * (eq185_e2327_d_n18 * ddt_scale));
        let eq185_e2329_d_n19: f64 = (p.p7 * (eq185_e2327_d_n19 * ddt_scale));
        let eq185_e2329_d_n20: f64 = (p.p7 * (eq185_e2327_d_n20 * ddt_scale));
        let eq185_e2329_d_n21: f64 = (p.p7 * (eq185_e2327_d_n21 * ddt_scale));
        let eq185_e2329_d_n22: f64 = (p.p7 * (eq185_e2327_d_n22 * ddt_scale));
        (eq185_e2329, eq185_e2329_d_n0, eq185_e2329_d_n1, eq185_e2329_d_n2, eq185_e2329_d_n3, eq185_e2329_d_n4, eq185_e2329_d_n5, eq185_e2329_d_n6, eq185_e2329_d_n7, eq185_e2329_d_n8, eq185_e2329_d_n9, eq185_e2329_d_n12, eq185_e2329_d_n14, eq185_e2329_d_n15, eq185_e2329_d_n16, eq185_e2329_d_n17, eq185_e2329_d_n18, eq185_e2329_d_n19, eq185_e2329_d_n20, eq185_e2329_d_n21, eq185_e2329_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq185_value: f64 = eq185_e2331;
        let eq185_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq185_node_derivatives: [f64; 20] = [eq185_e2331_d_n0, eq185_e2331_d_n1, eq185_e2331_d_n2, eq185_e2331_d_n3, eq185_e2331_d_n4, eq185_e2331_d_n5, eq185_e2331_d_n6, eq185_e2331_d_n7, eq185_e2331_d_n8, eq185_e2331_d_n9, eq185_e2331_d_n12, eq185_e2331_d_n14, eq185_e2331_d_n15, eq185_e2331_d_n16, eq185_e2331_d_n17, eq185_e2331_d_n18, eq185_e2331_d_n19, eq185_e2331_d_n20, eq185_e2331_d_n21, eq185_e2331_d_n22];
        let eq185_branch_derivative_indices: [usize; 0] = [];
        let eq185_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(21),
            multiplicity * (eq185_value),
            &eq185_node_derivative_indices,
            &eq185_node_derivatives,
            &eq185_branch_derivative_indices,
            &eq185_branch_derivatives,
            multiplicity,
        );
        let (eq186_e2341, eq186_e2341_d_n0, eq186_e2341_d_n1, eq186_e2341_d_n2, eq186_e2341_d_n3, eq186_e2341_d_n4, eq186_e2341_d_n5, eq186_e2341_d_n6, eq186_e2341_d_n7, eq186_e2341_d_n8, eq186_e2341_d_n9, eq186_e2341_d_n12, eq186_e2341_d_n14, eq186_e2341_d_n15, eq186_e2341_d_n16, eq186_e2341_d_n17, eq186_e2341_d_n18, eq186_e2341_d_n19, eq186_e2341_d_n20, eq186_e2341_d_n21, eq186_e2341_d_n22,) = {
    if ((var_guard561 == 0.0) && (var_guard564 != 0.0)) {
        let eq186_e2338: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 85, var_qd_fp3s);
        let eq186_e2339: f64 = (p.p7 * eq186_e2338);
        let eq186_e2339_d_n0: f64 = (p.p7 * (var_qd_fp3s_dn0 * ddt_scale));
        let eq186_e2339_d_n1: f64 = (p.p7 * (var_qd_fp3s_dn1 * ddt_scale));
        let eq186_e2339_d_n2: f64 = (p.p7 * (var_qd_fp3s_dn2 * ddt_scale));
        let eq186_e2339_d_n3: f64 = (p.p7 * (var_qd_fp3s_dn3 * ddt_scale));
        let eq186_e2339_d_n4: f64 = (p.p7 * (var_qd_fp3s_dn4 * ddt_scale));
        let eq186_e2339_d_n5: f64 = (p.p7 * (var_qd_fp3s_dn5 * ddt_scale));
        let eq186_e2339_d_n6: f64 = (p.p7 * (var_qd_fp3s_dn6 * ddt_scale));
        let eq186_e2339_d_n7: f64 = (p.p7 * (var_qd_fp3s_dn7 * ddt_scale));
        let eq186_e2339_d_n8: f64 = (p.p7 * (var_qd_fp3s_dn8 * ddt_scale));
        let eq186_e2339_d_n9: f64 = (p.p7 * (var_qd_fp3s_dn9 * ddt_scale));
        let eq186_e2339_d_n12: f64 = (p.p7 * (var_qd_fp3s_dn12 * ddt_scale));
        let eq186_e2339_d_n14: f64 = (p.p7 * (var_qd_fp3s_dn14 * ddt_scale));
        let eq186_e2339_d_n15: f64 = (p.p7 * (var_qd_fp3s_dn15 * ddt_scale));
        let eq186_e2339_d_n16: f64 = (p.p7 * (var_qd_fp3s_dn16 * ddt_scale));
        let eq186_e2339_d_n17: f64 = (p.p7 * (var_qd_fp3s_dn17 * ddt_scale));
        let eq186_e2339_d_n18: f64 = (p.p7 * (var_qd_fp3s_dn18 * ddt_scale));
        let eq186_e2339_d_n19: f64 = (p.p7 * (var_qd_fp3s_dn19 * ddt_scale));
        let eq186_e2339_d_n20: f64 = (p.p7 * (var_qd_fp3s_dn20 * ddt_scale));
        let eq186_e2339_d_n21: f64 = (p.p7 * (var_qd_fp3s_dn21 * ddt_scale));
        let eq186_e2339_d_n22: f64 = (p.p7 * (var_qd_fp3s_dn22 * ddt_scale));
        (eq186_e2339, eq186_e2339_d_n0, eq186_e2339_d_n1, eq186_e2339_d_n2, eq186_e2339_d_n3, eq186_e2339_d_n4, eq186_e2339_d_n5, eq186_e2339_d_n6, eq186_e2339_d_n7, eq186_e2339_d_n8, eq186_e2339_d_n9, eq186_e2339_d_n12, eq186_e2339_d_n14, eq186_e2339_d_n15, eq186_e2339_d_n16, eq186_e2339_d_n17, eq186_e2339_d_n18, eq186_e2339_d_n19, eq186_e2339_d_n20, eq186_e2339_d_n21, eq186_e2339_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq186_value: f64 = eq186_e2341;
        let eq186_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq186_node_derivatives: [f64; 20] = [eq186_e2341_d_n0, eq186_e2341_d_n1, eq186_e2341_d_n2, eq186_e2341_d_n3, eq186_e2341_d_n4, eq186_e2341_d_n5, eq186_e2341_d_n6, eq186_e2341_d_n7, eq186_e2341_d_n8, eq186_e2341_d_n9, eq186_e2341_d_n12, eq186_e2341_d_n14, eq186_e2341_d_n15, eq186_e2341_d_n16, eq186_e2341_d_n17, eq186_e2341_d_n18, eq186_e2341_d_n19, eq186_e2341_d_n20, eq186_e2341_d_n21, eq186_e2341_d_n22];
        let eq186_branch_derivative_indices: [usize; 0] = [];
        let eq186_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(2),
            multiplicity * (eq186_value),
            &eq186_node_derivative_indices,
            &eq186_node_derivatives,
            &eq186_branch_derivative_indices,
            &eq186_branch_derivatives,
            multiplicity,
        );
        let (eq187_e2353, eq187_e2353_d_n0, eq187_e2353_d_n1, eq187_e2353_d_n2, eq187_e2353_d_n3, eq187_e2353_d_n4, eq187_e2353_d_n5, eq187_e2353_d_n6, eq187_e2353_d_n7, eq187_e2353_d_n8, eq187_e2353_d_n9, eq187_e2353_d_n12, eq187_e2353_d_n14, eq187_e2353_d_n15, eq187_e2353_d_n16, eq187_e2353_d_n17, eq187_e2353_d_n18, eq187_e2353_d_n19, eq187_e2353_d_n20, eq187_e2353_d_n21, eq187_e2353_d_n22,) = {
    if (((var_guard561 == 0.0) && (var_guard564 != 0.0)) && (var_guard565 != 0.0)) {
        let eq187_e2350: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 86, var_qg_fp3s);
        let eq187_e2351: f64 = (p.p7 * eq187_e2350);
        let eq187_e2351_d_n0: f64 = (p.p7 * (var_qg_fp3s_dn0 * ddt_scale));
        let eq187_e2351_d_n1: f64 = (p.p7 * (var_qg_fp3s_dn1 * ddt_scale));
        let eq187_e2351_d_n2: f64 = (p.p7 * (var_qg_fp3s_dn2 * ddt_scale));
        let eq187_e2351_d_n3: f64 = (p.p7 * (var_qg_fp3s_dn3 * ddt_scale));
        let eq187_e2351_d_n4: f64 = (p.p7 * (var_qg_fp3s_dn4 * ddt_scale));
        let eq187_e2351_d_n5: f64 = (p.p7 * (var_qg_fp3s_dn5 * ddt_scale));
        let eq187_e2351_d_n6: f64 = (p.p7 * (var_qg_fp3s_dn6 * ddt_scale));
        let eq187_e2351_d_n7: f64 = (p.p7 * (var_qg_fp3s_dn7 * ddt_scale));
        let eq187_e2351_d_n8: f64 = (p.p7 * (var_qg_fp3s_dn8 * ddt_scale));
        let eq187_e2351_d_n9: f64 = (p.p7 * (var_qg_fp3s_dn9 * ddt_scale));
        let eq187_e2351_d_n12: f64 = (p.p7 * (var_qg_fp3s_dn12 * ddt_scale));
        let eq187_e2351_d_n14: f64 = (p.p7 * (var_qg_fp3s_dn14 * ddt_scale));
        let eq187_e2351_d_n15: f64 = (p.p7 * (var_qg_fp3s_dn15 * ddt_scale));
        let eq187_e2351_d_n16: f64 = (p.p7 * (var_qg_fp3s_dn16 * ddt_scale));
        let eq187_e2351_d_n17: f64 = (p.p7 * (var_qg_fp3s_dn17 * ddt_scale));
        let eq187_e2351_d_n18: f64 = (p.p7 * (var_qg_fp3s_dn18 * ddt_scale));
        let eq187_e2351_d_n19: f64 = (p.p7 * (var_qg_fp3s_dn19 * ddt_scale));
        let eq187_e2351_d_n20: f64 = (p.p7 * (var_qg_fp3s_dn20 * ddt_scale));
        let eq187_e2351_d_n21: f64 = (p.p7 * (var_qg_fp3s_dn21 * ddt_scale));
        let eq187_e2351_d_n22: f64 = (p.p7 * (var_qg_fp3s_dn22 * ddt_scale));
        (eq187_e2351, eq187_e2351_d_n0, eq187_e2351_d_n1, eq187_e2351_d_n2, eq187_e2351_d_n3, eq187_e2351_d_n4, eq187_e2351_d_n5, eq187_e2351_d_n6, eq187_e2351_d_n7, eq187_e2351_d_n8, eq187_e2351_d_n9, eq187_e2351_d_n12, eq187_e2351_d_n14, eq187_e2351_d_n15, eq187_e2351_d_n16, eq187_e2351_d_n17, eq187_e2351_d_n18, eq187_e2351_d_n19, eq187_e2351_d_n20, eq187_e2351_d_n21, eq187_e2351_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq187_value: f64 = eq187_e2353;
        let eq187_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq187_node_derivatives: [f64; 20] = [eq187_e2353_d_n0, eq187_e2353_d_n1, eq187_e2353_d_n2, eq187_e2353_d_n3, eq187_e2353_d_n4, eq187_e2353_d_n5, eq187_e2353_d_n6, eq187_e2353_d_n7, eq187_e2353_d_n8, eq187_e2353_d_n9, eq187_e2353_d_n12, eq187_e2353_d_n14, eq187_e2353_d_n15, eq187_e2353_d_n16, eq187_e2353_d_n17, eq187_e2353_d_n18, eq187_e2353_d_n19, eq187_e2353_d_n20, eq187_e2353_d_n21, eq187_e2353_d_n22];
        let eq187_branch_derivative_indices: [usize; 0] = [];
        let eq187_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(2),
            multiplicity * (eq187_value),
            &eq187_node_derivative_indices,
            &eq187_node_derivatives,
            &eq187_branch_derivative_indices,
            &eq187_branch_derivatives,
            multiplicity,
        );
        let (eq188_e2367, eq188_e2367_d_n0, eq188_e2367_d_n1, eq188_e2367_d_n2, eq188_e2367_d_n3, eq188_e2367_d_n4, eq188_e2367_d_n5, eq188_e2367_d_n6, eq188_e2367_d_n7, eq188_e2367_d_n8, eq188_e2367_d_n9, eq188_e2367_d_n12, eq188_e2367_d_n14, eq188_e2367_d_n15, eq188_e2367_d_n16, eq188_e2367_d_n17, eq188_e2367_d_n18, eq188_e2367_d_n19, eq188_e2367_d_n20, eq188_e2367_d_n21, eq188_e2367_d_n22,) = {
    if (((var_guard561 == 0.0) && (var_guard564 != 0.0)) && (var_guard565 != 0.0)) {
        let eq188_e2362: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 87, var_qg_fp3s);
        let eq188_e2363: f64 = (p.p7 * eq188_e2362);
        let eq188_e2363_d_n0: f64 = (p.p7 * (var_qg_fp3s_dn0 * ddt_scale));
        let eq188_e2363_d_n1: f64 = (p.p7 * (var_qg_fp3s_dn1 * ddt_scale));
        let eq188_e2363_d_n2: f64 = (p.p7 * (var_qg_fp3s_dn2 * ddt_scale));
        let eq188_e2363_d_n3: f64 = (p.p7 * (var_qg_fp3s_dn3 * ddt_scale));
        let eq188_e2363_d_n4: f64 = (p.p7 * (var_qg_fp3s_dn4 * ddt_scale));
        let eq188_e2363_d_n5: f64 = (p.p7 * (var_qg_fp3s_dn5 * ddt_scale));
        let eq188_e2363_d_n6: f64 = (p.p7 * (var_qg_fp3s_dn6 * ddt_scale));
        let eq188_e2363_d_n7: f64 = (p.p7 * (var_qg_fp3s_dn7 * ddt_scale));
        let eq188_e2363_d_n8: f64 = (p.p7 * (var_qg_fp3s_dn8 * ddt_scale));
        let eq188_e2363_d_n9: f64 = (p.p7 * (var_qg_fp3s_dn9 * ddt_scale));
        let eq188_e2363_d_n12: f64 = (p.p7 * (var_qg_fp3s_dn12 * ddt_scale));
        let eq188_e2363_d_n14: f64 = (p.p7 * (var_qg_fp3s_dn14 * ddt_scale));
        let eq188_e2363_d_n15: f64 = (p.p7 * (var_qg_fp3s_dn15 * ddt_scale));
        let eq188_e2363_d_n16: f64 = (p.p7 * (var_qg_fp3s_dn16 * ddt_scale));
        let eq188_e2363_d_n17: f64 = (p.p7 * (var_qg_fp3s_dn17 * ddt_scale));
        let eq188_e2363_d_n18: f64 = (p.p7 * (var_qg_fp3s_dn18 * ddt_scale));
        let eq188_e2363_d_n19: f64 = (p.p7 * (var_qg_fp3s_dn19 * ddt_scale));
        let eq188_e2363_d_n20: f64 = (p.p7 * (var_qg_fp3s_dn20 * ddt_scale));
        let eq188_e2363_d_n21: f64 = (p.p7 * (var_qg_fp3s_dn21 * ddt_scale));
        let eq188_e2363_d_n22: f64 = (p.p7 * (var_qg_fp3s_dn22 * ddt_scale));
        let eq188_e2365: f64 = (eq188_e2363 * p.p248);
        let eq188_e2365_d_n0: f64 = (eq188_e2363_d_n0 * p.p248);
        let eq188_e2365_d_n1: f64 = (eq188_e2363_d_n1 * p.p248);
        let eq188_e2365_d_n2: f64 = (eq188_e2363_d_n2 * p.p248);
        let eq188_e2365_d_n3: f64 = (eq188_e2363_d_n3 * p.p248);
        let eq188_e2365_d_n4: f64 = (eq188_e2363_d_n4 * p.p248);
        let eq188_e2365_d_n5: f64 = (eq188_e2363_d_n5 * p.p248);
        let eq188_e2365_d_n6: f64 = (eq188_e2363_d_n6 * p.p248);
        let eq188_e2365_d_n7: f64 = (eq188_e2363_d_n7 * p.p248);
        let eq188_e2365_d_n8: f64 = (eq188_e2363_d_n8 * p.p248);
        let eq188_e2365_d_n9: f64 = (eq188_e2363_d_n9 * p.p248);
        let eq188_e2365_d_n12: f64 = (eq188_e2363_d_n12 * p.p248);
        let eq188_e2365_d_n14: f64 = (eq188_e2363_d_n14 * p.p248);
        let eq188_e2365_d_n15: f64 = (eq188_e2363_d_n15 * p.p248);
        let eq188_e2365_d_n16: f64 = (eq188_e2363_d_n16 * p.p248);
        let eq188_e2365_d_n17: f64 = (eq188_e2363_d_n17 * p.p248);
        let eq188_e2365_d_n18: f64 = (eq188_e2363_d_n18 * p.p248);
        let eq188_e2365_d_n19: f64 = (eq188_e2363_d_n19 * p.p248);
        let eq188_e2365_d_n20: f64 = (eq188_e2363_d_n20 * p.p248);
        let eq188_e2365_d_n21: f64 = (eq188_e2363_d_n21 * p.p248);
        let eq188_e2365_d_n22: f64 = (eq188_e2363_d_n22 * p.p248);
        (eq188_e2365, eq188_e2365_d_n0, eq188_e2365_d_n1, eq188_e2365_d_n2, eq188_e2365_d_n3, eq188_e2365_d_n4, eq188_e2365_d_n5, eq188_e2365_d_n6, eq188_e2365_d_n7, eq188_e2365_d_n8, eq188_e2365_d_n9, eq188_e2365_d_n12, eq188_e2365_d_n14, eq188_e2365_d_n15, eq188_e2365_d_n16, eq188_e2365_d_n17, eq188_e2365_d_n18, eq188_e2365_d_n19, eq188_e2365_d_n20, eq188_e2365_d_n21, eq188_e2365_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq188_value: f64 = eq188_e2367;
        let eq188_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq188_node_derivatives: [f64; 20] = [eq188_e2367_d_n0, eq188_e2367_d_n1, eq188_e2367_d_n2, eq188_e2367_d_n3, eq188_e2367_d_n4, eq188_e2367_d_n5, eq188_e2367_d_n6, eq188_e2367_d_n7, eq188_e2367_d_n8, eq188_e2367_d_n9, eq188_e2367_d_n12, eq188_e2367_d_n14, eq188_e2367_d_n15, eq188_e2367_d_n16, eq188_e2367_d_n17, eq188_e2367_d_n18, eq188_e2367_d_n19, eq188_e2367_d_n20, eq188_e2367_d_n21, eq188_e2367_d_n22];
        let eq188_branch_derivative_indices: [usize; 0] = [];
        let eq188_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq188_value),
            &eq188_node_derivative_indices,
            &eq188_node_derivatives,
            &eq188_branch_derivative_indices,
            &eq188_branch_derivatives,
            multiplicity,
        );
        let (eq189_e2380, eq189_e2380_d_n0, eq189_e2380_d_n1, eq189_e2380_d_n2, eq189_e2380_d_n3, eq189_e2380_d_n4, eq189_e2380_d_n5, eq189_e2380_d_n6, eq189_e2380_d_n7, eq189_e2380_d_n8, eq189_e2380_d_n9, eq189_e2380_d_n12, eq189_e2380_d_n14, eq189_e2380_d_n15, eq189_e2380_d_n16, eq189_e2380_d_n17, eq189_e2380_d_n18, eq189_e2380_d_n19, eq189_e2380_d_n20, eq189_e2380_d_n21, eq189_e2380_d_n22,) = {
    if (((var_guard561 == 0.0) && (var_guard564 != 0.0)) && (var_guard565 == 0.0)) {
        let eq189_e2377: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 88, var_qg_fp3s);
        let eq189_e2378: f64 = (p.p7 * eq189_e2377);
        let eq189_e2378_d_n0: f64 = (p.p7 * (var_qg_fp3s_dn0 * ddt_scale));
        let eq189_e2378_d_n1: f64 = (p.p7 * (var_qg_fp3s_dn1 * ddt_scale));
        let eq189_e2378_d_n2: f64 = (p.p7 * (var_qg_fp3s_dn2 * ddt_scale));
        let eq189_e2378_d_n3: f64 = (p.p7 * (var_qg_fp3s_dn3 * ddt_scale));
        let eq189_e2378_d_n4: f64 = (p.p7 * (var_qg_fp3s_dn4 * ddt_scale));
        let eq189_e2378_d_n5: f64 = (p.p7 * (var_qg_fp3s_dn5 * ddt_scale));
        let eq189_e2378_d_n6: f64 = (p.p7 * (var_qg_fp3s_dn6 * ddt_scale));
        let eq189_e2378_d_n7: f64 = (p.p7 * (var_qg_fp3s_dn7 * ddt_scale));
        let eq189_e2378_d_n8: f64 = (p.p7 * (var_qg_fp3s_dn8 * ddt_scale));
        let eq189_e2378_d_n9: f64 = (p.p7 * (var_qg_fp3s_dn9 * ddt_scale));
        let eq189_e2378_d_n12: f64 = (p.p7 * (var_qg_fp3s_dn12 * ddt_scale));
        let eq189_e2378_d_n14: f64 = (p.p7 * (var_qg_fp3s_dn14 * ddt_scale));
        let eq189_e2378_d_n15: f64 = (p.p7 * (var_qg_fp3s_dn15 * ddt_scale));
        let eq189_e2378_d_n16: f64 = (p.p7 * (var_qg_fp3s_dn16 * ddt_scale));
        let eq189_e2378_d_n17: f64 = (p.p7 * (var_qg_fp3s_dn17 * ddt_scale));
        let eq189_e2378_d_n18: f64 = (p.p7 * (var_qg_fp3s_dn18 * ddt_scale));
        let eq189_e2378_d_n19: f64 = (p.p7 * (var_qg_fp3s_dn19 * ddt_scale));
        let eq189_e2378_d_n20: f64 = (p.p7 * (var_qg_fp3s_dn20 * ddt_scale));
        let eq189_e2378_d_n21: f64 = (p.p7 * (var_qg_fp3s_dn21 * ddt_scale));
        let eq189_e2378_d_n22: f64 = (p.p7 * (var_qg_fp3s_dn22 * ddt_scale));
        (eq189_e2378, eq189_e2378_d_n0, eq189_e2378_d_n1, eq189_e2378_d_n2, eq189_e2378_d_n3, eq189_e2378_d_n4, eq189_e2378_d_n5, eq189_e2378_d_n6, eq189_e2378_d_n7, eq189_e2378_d_n8, eq189_e2378_d_n9, eq189_e2378_d_n12, eq189_e2378_d_n14, eq189_e2378_d_n15, eq189_e2378_d_n16, eq189_e2378_d_n17, eq189_e2378_d_n18, eq189_e2378_d_n19, eq189_e2378_d_n20, eq189_e2378_d_n21, eq189_e2378_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq189_value: f64 = eq189_e2380;
        let eq189_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq189_node_derivatives: [f64; 20] = [eq189_e2380_d_n0, eq189_e2380_d_n1, eq189_e2380_d_n2, eq189_e2380_d_n3, eq189_e2380_d_n4, eq189_e2380_d_n5, eq189_e2380_d_n6, eq189_e2380_d_n7, eq189_e2380_d_n8, eq189_e2380_d_n9, eq189_e2380_d_n12, eq189_e2380_d_n14, eq189_e2380_d_n15, eq189_e2380_d_n16, eq189_e2380_d_n17, eq189_e2380_d_n18, eq189_e2380_d_n19, eq189_e2380_d_n20, eq189_e2380_d_n21, eq189_e2380_d_n22];
        let eq189_branch_derivative_indices: [usize; 0] = [];
        let eq189_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq189_value),
            &eq189_node_derivative_indices,
            &eq189_node_derivatives,
            &eq189_branch_derivative_indices,
            &eq189_branch_derivatives,
            multiplicity,
        );
        let (eq190_e2395, eq190_e2395_d_n0, eq190_e2395_d_n1, eq190_e2395_d_n2, eq190_e2395_d_n3, eq190_e2395_d_n4, eq190_e2395_d_n5, eq190_e2395_d_n6, eq190_e2395_d_n7, eq190_e2395_d_n8, eq190_e2395_d_n9, eq190_e2395_d_n12, eq190_e2395_d_n14, eq190_e2395_d_n15, eq190_e2395_d_n16, eq190_e2395_d_n17, eq190_e2395_d_n18, eq190_e2395_d_n19, eq190_e2395_d_n20, eq190_e2395_d_n21, eq190_e2395_d_n22,) = {
    if (((var_guard561 == 0.0) && (var_guard564 != 0.0)) && (var_guard565 == 0.0)) {
        let eq190_e2390: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 89, var_qg_fp3s);
        let eq190_e2391: f64 = (p.p7 * eq190_e2390);
        let eq190_e2391_d_n0: f64 = (p.p7 * (var_qg_fp3s_dn0 * ddt_scale));
        let eq190_e2391_d_n1: f64 = (p.p7 * (var_qg_fp3s_dn1 * ddt_scale));
        let eq190_e2391_d_n2: f64 = (p.p7 * (var_qg_fp3s_dn2 * ddt_scale));
        let eq190_e2391_d_n3: f64 = (p.p7 * (var_qg_fp3s_dn3 * ddt_scale));
        let eq190_e2391_d_n4: f64 = (p.p7 * (var_qg_fp3s_dn4 * ddt_scale));
        let eq190_e2391_d_n5: f64 = (p.p7 * (var_qg_fp3s_dn5 * ddt_scale));
        let eq190_e2391_d_n6: f64 = (p.p7 * (var_qg_fp3s_dn6 * ddt_scale));
        let eq190_e2391_d_n7: f64 = (p.p7 * (var_qg_fp3s_dn7 * ddt_scale));
        let eq190_e2391_d_n8: f64 = (p.p7 * (var_qg_fp3s_dn8 * ddt_scale));
        let eq190_e2391_d_n9: f64 = (p.p7 * (var_qg_fp3s_dn9 * ddt_scale));
        let eq190_e2391_d_n12: f64 = (p.p7 * (var_qg_fp3s_dn12 * ddt_scale));
        let eq190_e2391_d_n14: f64 = (p.p7 * (var_qg_fp3s_dn14 * ddt_scale));
        let eq190_e2391_d_n15: f64 = (p.p7 * (var_qg_fp3s_dn15 * ddt_scale));
        let eq190_e2391_d_n16: f64 = (p.p7 * (var_qg_fp3s_dn16 * ddt_scale));
        let eq190_e2391_d_n17: f64 = (p.p7 * (var_qg_fp3s_dn17 * ddt_scale));
        let eq190_e2391_d_n18: f64 = (p.p7 * (var_qg_fp3s_dn18 * ddt_scale));
        let eq190_e2391_d_n19: f64 = (p.p7 * (var_qg_fp3s_dn19 * ddt_scale));
        let eq190_e2391_d_n20: f64 = (p.p7 * (var_qg_fp3s_dn20 * ddt_scale));
        let eq190_e2391_d_n21: f64 = (p.p7 * (var_qg_fp3s_dn21 * ddt_scale));
        let eq190_e2391_d_n22: f64 = (p.p7 * (var_qg_fp3s_dn22 * ddt_scale));
        let eq190_e2393: f64 = (eq190_e2391 * p.p248);
        let eq190_e2393_d_n0: f64 = (eq190_e2391_d_n0 * p.p248);
        let eq190_e2393_d_n1: f64 = (eq190_e2391_d_n1 * p.p248);
        let eq190_e2393_d_n2: f64 = (eq190_e2391_d_n2 * p.p248);
        let eq190_e2393_d_n3: f64 = (eq190_e2391_d_n3 * p.p248);
        let eq190_e2393_d_n4: f64 = (eq190_e2391_d_n4 * p.p248);
        let eq190_e2393_d_n5: f64 = (eq190_e2391_d_n5 * p.p248);
        let eq190_e2393_d_n6: f64 = (eq190_e2391_d_n6 * p.p248);
        let eq190_e2393_d_n7: f64 = (eq190_e2391_d_n7 * p.p248);
        let eq190_e2393_d_n8: f64 = (eq190_e2391_d_n8 * p.p248);
        let eq190_e2393_d_n9: f64 = (eq190_e2391_d_n9 * p.p248);
        let eq190_e2393_d_n12: f64 = (eq190_e2391_d_n12 * p.p248);
        let eq190_e2393_d_n14: f64 = (eq190_e2391_d_n14 * p.p248);
        let eq190_e2393_d_n15: f64 = (eq190_e2391_d_n15 * p.p248);
        let eq190_e2393_d_n16: f64 = (eq190_e2391_d_n16 * p.p248);
        let eq190_e2393_d_n17: f64 = (eq190_e2391_d_n17 * p.p248);
        let eq190_e2393_d_n18: f64 = (eq190_e2391_d_n18 * p.p248);
        let eq190_e2393_d_n19: f64 = (eq190_e2391_d_n19 * p.p248);
        let eq190_e2393_d_n20: f64 = (eq190_e2391_d_n20 * p.p248);
        let eq190_e2393_d_n21: f64 = (eq190_e2391_d_n21 * p.p248);
        let eq190_e2393_d_n22: f64 = (eq190_e2391_d_n22 * p.p248);
        (eq190_e2393, eq190_e2393_d_n0, eq190_e2393_d_n1, eq190_e2393_d_n2, eq190_e2393_d_n3, eq190_e2393_d_n4, eq190_e2393_d_n5, eq190_e2393_d_n6, eq190_e2393_d_n7, eq190_e2393_d_n8, eq190_e2393_d_n9, eq190_e2393_d_n12, eq190_e2393_d_n14, eq190_e2393_d_n15, eq190_e2393_d_n16, eq190_e2393_d_n17, eq190_e2393_d_n18, eq190_e2393_d_n19, eq190_e2393_d_n20, eq190_e2393_d_n21, eq190_e2393_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq190_value: f64 = eq190_e2395;
        let eq190_node_derivative_indices: [usize; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let eq190_node_derivatives: [f64; 20] = [eq190_e2395_d_n0, eq190_e2395_d_n1, eq190_e2395_d_n2, eq190_e2395_d_n3, eq190_e2395_d_n4, eq190_e2395_d_n5, eq190_e2395_d_n6, eq190_e2395_d_n7, eq190_e2395_d_n8, eq190_e2395_d_n9, eq190_e2395_d_n12, eq190_e2395_d_n14, eq190_e2395_d_n15, eq190_e2395_d_n16, eq190_e2395_d_n17, eq190_e2395_d_n18, eq190_e2395_d_n19, eq190_e2395_d_n20, eq190_e2395_d_n21, eq190_e2395_d_n22];
        let eq190_branch_derivative_indices: [usize; 0] = [];
        let eq190_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq190_value),
            &eq190_node_derivative_indices,
            &eq190_node_derivatives,
            &eq190_branch_derivative_indices,
            &eq190_branch_derivatives,
            multiplicity,
        );
    }
}
