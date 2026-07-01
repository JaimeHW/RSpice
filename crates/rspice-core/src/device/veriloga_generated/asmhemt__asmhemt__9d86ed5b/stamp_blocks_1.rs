#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_tdev: f64,
        var_tdev_db0: f64,
        var_tdev_db1: f64,
        var_tdev_db10: f64,
        var_tdev_db11: f64,
        var_tdev_db12: f64,
        var_tdev_db13: f64,
        var_tdev_db14: f64,
        var_tdev_db15: f64,
        var_tdev_db16: f64,
        var_tdev_db17: f64,
        var_tdev_db18: f64,
        var_tdev_db19: f64,
        var_tdev_db2: f64,
        var_tdev_db20: f64,
        var_tdev_db21: f64,
        var_tdev_db22: f64,
        var_tdev_db23: f64,
        var_tdev_db24: f64,
        var_tdev_db25: f64,
        var_tdev_db26: f64,
        var_tdev_db27: f64,
        var_tdev_db28: f64,
        var_tdev_db29: f64,
        var_tdev_db3: f64,
        var_tdev_db30: f64,
        var_tdev_db31: f64,
        var_tdev_db32: f64,
        var_tdev_db33: f64,
        var_tdev_db34: f64,
        var_tdev_db35: f64,
        var_tdev_db36: f64,
        var_tdev_db37: f64,
        var_tdev_db38: f64,
        var_tdev_db39: f64,
        var_tdev_db4: f64,
        var_tdev_db40: f64,
        var_tdev_db41: f64,
        var_tdev_db42: f64,
        var_tdev_db43: f64,
        var_tdev_db44: f64,
        var_tdev_db45: f64,
        var_tdev_db46: f64,
        var_tdev_db47: f64,
        var_tdev_db48: f64,
        var_tdev_db49: f64,
        var_tdev_db5: f64,
        var_tdev_db50: f64,
        var_tdev_db51: f64,
        var_tdev_db52: f64,
        var_tdev_db53: f64,
        var_tdev_db54: f64,
        var_tdev_db55: f64,
        var_tdev_db56: f64,
        var_tdev_db6: f64,
        var_tdev_db7: f64,
        var_tdev_db8: f64,
        var_tdev_db9: f64,
        var_tdev_dn0: f64,
        var_tdev_dn1: f64,
        var_tdev_dn10: f64,
        var_tdev_dn11: f64,
        var_tdev_dn12: f64,
        var_tdev_dn13: f64,
        var_tdev_dn14: f64,
        var_tdev_dn15: f64,
        var_tdev_dn16: f64,
        var_tdev_dn17: f64,
        var_tdev_dn18: f64,
        var_tdev_dn19: f64,
        var_tdev_dn2: f64,
        var_tdev_dn20: f64,
        var_tdev_dn21: f64,
        var_tdev_dn22: f64,
        var_tdev_dn3: f64,
        var_tdev_dn4: f64,
        var_tdev_dn5: f64,
        var_tdev_dn6: f64,
        var_tdev_dn7: f64,
        var_tdev_dn8: f64,
        var_tdev_dn9: f64,
        var_tnom: f64,
    ) {
        if (s.b[478] && s.b[479]) {
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);
            s.store_scaled_mul(96, 95, 259, (p.p4 * (p.p5 * 1.0 / (p.p174))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(268), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(263), (p.p25 * p.p25), s.ad_value(263)), 1.0);
            s.store_div(93, 98, 92);
            s.store_mul(269, 93, 135);
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
        s.store_scalar(486, if s.b[486] { 1.0 } else { 0.0 });

        if ((s.b[478] && s.b[479]) && s.b[486]) {
            s.store_sub_scaled_inputs(265, 264, (-1.0), 265, 1.0);
        }

        if (s.b[478] && (!s.b[479])) {
            s.store_scalar(264, 0.0);
            s.store_scalar(265, 0.0);
        }

        s.b[487] = (p.p153 != 0.0);
        s.store_scalar(487, if s.b[487] { 1.0 } else { 0.0 });

        s.b[488] = (p.p153 == 1.0);
        s.store_scalar(488, if s.b[488] { 1.0 } else { 0.0 });

        if (((!s.b[478]) && s.b[487]) && s.b[488]) {
            s.store_voltage(62, ctx, nodes, Some(9), Some(8));
        }

        if (((!s.b[478]) && s.b[487]) && (!s.b[488])) {
            s.store_voltage(62, ctx, nodes, Some(2), Some(8));
        }

        if ((!s.b[478]) && s.b[487]) {
            s.copy_ad(266, 62);
            s.store_scalar(146, (1.0 + p.p178));
            s.store_mul_scale_ad_lhs(83, A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 8.617087e-5, 146);
            s.store_offset_scaled_ad(88, A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), ((1.0 / (var_tnom)) * (p.p175)), (((((-1.0)) * (p.p175))) + (p.p172)));
            s.store_scalar(259, (p.p9 / p.p173));
            s.store_div_from_scalar_scaled_mul(136, p.p174, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p171), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 266, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(266), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);
            s.store_sub(258, 160, 88);
            s.store_div_scaled_inputs_indices(84, 259, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 259, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 258, 258, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
        }

        if ((!s.b[478]) && s.b[487]) {
            let assign17250_ad_e26867: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, assign17250_ad_e26867, (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(assign17250_ad_e26867, ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
        }

        if ((!s.b[478]) && s.b[487]) {
            s.store_div_scaled_inputs_indices(136, 258, 1.0, 83, 2.0);
        }

        s.b[489] = (s.v[136] < 200.0);
        s.store_scalar(489, if s.b[489] { 1.0 } else { 0.0 });

        if (((!s.b[478]) && s.b[487]) && s.b[489]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(258), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[478]) && s.b[487]) && (!s.b[489])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(258), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[478]) && s.b[487]) {
            s.store_sub_div_rhs_indices(100, 258, 153, 99);
        }

        s.b[490] = ((((s.v[100] - s.v[258])) as f64).abs() > 1e-19);
        s.store_scalar(490, if s.b[490] { 1.0 } else { 0.0 });

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

        if (((!s.b[478]) && s.b[487]) && s.b[490]) {
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

        if (((!s.b[478]) && s.b[487]) && s.b[490]) {
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
            s.store_scaled_powf_ad(97, A::scale(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 1.0 / (var_tnom)), p.p20, p.p176);
            s.store_scaled_powf_ad(89, A::scale(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 1.0 / (var_tnom)), p.p19, p.p177);
            s.store_mul_scaled_abs_ad_rhs(136, 259, 1.0 / (p.p9), A::sub(s.ad_value(258), s.ad_value(260)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(260)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(90, 258, 258, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p174, 136, p.p174, 90, 1.0, 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(267), s.ad_value(85)), p.p18);
            s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));
            s.store_mul(86, 267, 90);
            s.store_sub(39, 258, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
        }

        if ((!s.b[478]) && s.b[487]) {
            let assign17880_ad_e27963: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, assign17880_ad_e27963, (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(assign17880_ad_e27963, ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
        }

        if ((!s.b[478]) && s.b[487]) {
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[491] = (s.v[136] < 200.0);
        s.store_scalar(491, if s.b[491] { 1.0 } else { 0.0 });

        if (((!s.b[478]) && s.b[487]) && s.b[491]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
        }

    }

    pub(super) fn stamp_transient_block_17(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_tdev: f64,
        var_tdev_db0: f64,
        var_tdev_db1: f64,
        var_tdev_db10: f64,
        var_tdev_db11: f64,
        var_tdev_db12: f64,
        var_tdev_db13: f64,
        var_tdev_db14: f64,
        var_tdev_db15: f64,
        var_tdev_db16: f64,
        var_tdev_db17: f64,
        var_tdev_db18: f64,
        var_tdev_db19: f64,
        var_tdev_db2: f64,
        var_tdev_db20: f64,
        var_tdev_db21: f64,
        var_tdev_db22: f64,
        var_tdev_db23: f64,
        var_tdev_db24: f64,
        var_tdev_db25: f64,
        var_tdev_db26: f64,
        var_tdev_db27: f64,
        var_tdev_db28: f64,
        var_tdev_db29: f64,
        var_tdev_db3: f64,
        var_tdev_db30: f64,
        var_tdev_db31: f64,
        var_tdev_db32: f64,
        var_tdev_db33: f64,
        var_tdev_db34: f64,
        var_tdev_db35: f64,
        var_tdev_db36: f64,
        var_tdev_db37: f64,
        var_tdev_db38: f64,
        var_tdev_db39: f64,
        var_tdev_db4: f64,
        var_tdev_db40: f64,
        var_tdev_db41: f64,
        var_tdev_db42: f64,
        var_tdev_db43: f64,
        var_tdev_db44: f64,
        var_tdev_db45: f64,
        var_tdev_db46: f64,
        var_tdev_db47: f64,
        var_tdev_db48: f64,
        var_tdev_db49: f64,
        var_tdev_db5: f64,
        var_tdev_db50: f64,
        var_tdev_db51: f64,
        var_tdev_db52: f64,
        var_tdev_db53: f64,
        var_tdev_db54: f64,
        var_tdev_db55: f64,
        var_tdev_db56: f64,
        var_tdev_db6: f64,
        var_tdev_db7: f64,
        var_tdev_db8: f64,
        var_tdev_db9: f64,
        var_tdev_dn0: f64,
        var_tdev_dn1: f64,
        var_tdev_dn10: f64,
        var_tdev_dn11: f64,
        var_tdev_dn12: f64,
        var_tdev_dn13: f64,
        var_tdev_dn14: f64,
        var_tdev_dn15: f64,
        var_tdev_dn16: f64,
        var_tdev_dn17: f64,
        var_tdev_dn18: f64,
        var_tdev_dn19: f64,
        var_tdev_dn2: f64,
        var_tdev_dn20: f64,
        var_tdev_dn21: f64,
        var_tdev_dn22: f64,
        var_tdev_dn3: f64,
        var_tdev_dn4: f64,
        var_tdev_dn5: f64,
        var_tdev_dn6: f64,
        var_tdev_dn7: f64,
        var_tdev_dn8: f64,
        var_tdev_dn9: f64,
        var_tnom: f64,
    ) {
        if (((!s.b[478]) && s.b[487]) && s.b[491]) {
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[478]) && s.b[487]) && (!s.b[491])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[478]) && s.b[487]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[492] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.store_scalar(492, if s.b[492] { 1.0 } else { 0.0 });

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
        s.store_scalar(493, if s.b[493] { 1.0 } else { 0.0 });

        s.b[494] = (p.p154 != 0.0);
        s.store_scalar(494, if s.b[494] { 1.0 } else { 0.0 });

        if (s.b[493] && s.b[494]) {
            s.store_voltage(65, ctx, nodes, Some(17), Some(16));
        }

        s.b[495] = (p.p154 == 1.0);
        s.store_scalar(495, if s.b[495] { 1.0 } else { 0.0 });

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
        s.store_scalar(496, if s.b[496] { 1.0 } else { 0.0 });

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
            s.store_mul_scale_ad_lhs(83, A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 8.617087e-5, 146);
            s.store_sub_ad(88, A::sub_from_scalar(p.p185, A::scale_offset(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), ((1.0 / (var_tnom)) * (p.p188)), (((-1.0)) * (p.p188)))), A::div_scaled_inputs(s.ad_value(280), (p.p194 * p.p193), A::sqrt_square_offset(s.ad_value(280), (p.p194 * p.p194)), 1.0));
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

    }

    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
        var_tdev: f64,
        var_tdev_db0: f64,
        var_tdev_db1: f64,
        var_tdev_db10: f64,
        var_tdev_db11: f64,
        var_tdev_db12: f64,
        var_tdev_db13: f64,
        var_tdev_db14: f64,
        var_tdev_db15: f64,
        var_tdev_db16: f64,
        var_tdev_db17: f64,
        var_tdev_db18: f64,
        var_tdev_db19: f64,
        var_tdev_db2: f64,
        var_tdev_db20: f64,
        var_tdev_db21: f64,
        var_tdev_db22: f64,
        var_tdev_db23: f64,
        var_tdev_db24: f64,
        var_tdev_db25: f64,
        var_tdev_db26: f64,
        var_tdev_db27: f64,
        var_tdev_db28: f64,
        var_tdev_db29: f64,
        var_tdev_db3: f64,
        var_tdev_db30: f64,
        var_tdev_db31: f64,
        var_tdev_db32: f64,
        var_tdev_db33: f64,
        var_tdev_db34: f64,
        var_tdev_db35: f64,
        var_tdev_db36: f64,
        var_tdev_db37: f64,
        var_tdev_db38: f64,
        var_tdev_db39: f64,
        var_tdev_db4: f64,
        var_tdev_db40: f64,
        var_tdev_db41: f64,
        var_tdev_db42: f64,
        var_tdev_db43: f64,
        var_tdev_db44: f64,
        var_tdev_db45: f64,
        var_tdev_db46: f64,
        var_tdev_db47: f64,
        var_tdev_db48: f64,
        var_tdev_db49: f64,
        var_tdev_db5: f64,
        var_tdev_db50: f64,
        var_tdev_db51: f64,
        var_tdev_db52: f64,
        var_tdev_db53: f64,
        var_tdev_db54: f64,
        var_tdev_db55: f64,
        var_tdev_db56: f64,
        var_tdev_db6: f64,
        var_tdev_db7: f64,
        var_tdev_db8: f64,
        var_tdev_db9: f64,
        var_tdev_dn0: f64,
        var_tdev_dn1: f64,
        var_tdev_dn10: f64,
        var_tdev_dn11: f64,
        var_tdev_dn12: f64,
        var_tdev_dn13: f64,
        var_tdev_dn14: f64,
        var_tdev_dn15: f64,
        var_tdev_dn16: f64,
        var_tdev_dn17: f64,
        var_tdev_dn18: f64,
        var_tdev_dn19: f64,
        var_tdev_dn2: f64,
        var_tdev_dn20: f64,
        var_tdev_dn21: f64,
        var_tdev_dn22: f64,
        var_tdev_dn3: f64,
        var_tdev_dn4: f64,
        var_tdev_dn5: f64,
        var_tdev_dn6: f64,
        var_tdev_dn7: f64,
        var_tdev_dn8: f64,
        var_tdev_dn9: f64,
        var_tnom: f64,
    ) {
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
            s.store_scaled_powf_ad(97, A::scale(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 1.0 / (var_tnom)), p.p20, p.p189);
            s.store_scaled_powf_ad(89, A::scale(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 1.0 / (var_tnom)), p.p19, p.p190);
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
        }

    }

    pub(super) fn stamp_transient_block_19(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_tdev: f64,
        var_tdev_db0: f64,
        var_tdev_db1: f64,
        var_tdev_db10: f64,
        var_tdev_db11: f64,
        var_tdev_db12: f64,
        var_tdev_db13: f64,
        var_tdev_db14: f64,
        var_tdev_db15: f64,
        var_tdev_db16: f64,
        var_tdev_db17: f64,
        var_tdev_db18: f64,
        var_tdev_db19: f64,
        var_tdev_db2: f64,
        var_tdev_db20: f64,
        var_tdev_db21: f64,
        var_tdev_db22: f64,
        var_tdev_db23: f64,
        var_tdev_db24: f64,
        var_tdev_db25: f64,
        var_tdev_db26: f64,
        var_tdev_db27: f64,
        var_tdev_db28: f64,
        var_tdev_db29: f64,
        var_tdev_db3: f64,
        var_tdev_db30: f64,
        var_tdev_db31: f64,
        var_tdev_db32: f64,
        var_tdev_db33: f64,
        var_tdev_db34: f64,
        var_tdev_db35: f64,
        var_tdev_db36: f64,
        var_tdev_db37: f64,
        var_tdev_db38: f64,
        var_tdev_db39: f64,
        var_tdev_db4: f64,
        var_tdev_db40: f64,
        var_tdev_db41: f64,
        var_tdev_db42: f64,
        var_tdev_db43: f64,
        var_tdev_db44: f64,
        var_tdev_db45: f64,
        var_tdev_db46: f64,
        var_tdev_db47: f64,
        var_tdev_db48: f64,
        var_tdev_db49: f64,
        var_tdev_db5: f64,
        var_tdev_db50: f64,
        var_tdev_db51: f64,
        var_tdev_db52: f64,
        var_tdev_db53: f64,
        var_tdev_db54: f64,
        var_tdev_db55: f64,
        var_tdev_db56: f64,
        var_tdev_db6: f64,
        var_tdev_db7: f64,
        var_tdev_db8: f64,
        var_tdev_db9: f64,
        var_tdev_dn0: f64,
        var_tdev_dn1: f64,
        var_tdev_dn10: f64,
        var_tdev_dn11: f64,
        var_tdev_dn12: f64,
        var_tdev_dn13: f64,
        var_tdev_dn14: f64,
        var_tdev_dn15: f64,
        var_tdev_dn16: f64,
        var_tdev_dn17: f64,
        var_tdev_dn18: f64,
        var_tdev_dn19: f64,
        var_tdev_dn2: f64,
        var_tdev_dn20: f64,
        var_tdev_dn21: f64,
        var_tdev_dn22: f64,
        var_tdev_dn3: f64,
        var_tdev_dn4: f64,
        var_tdev_dn5: f64,
        var_tdev_dn6: f64,
        var_tdev_dn7: f64,
        var_tdev_dn8: f64,
        var_tdev_dn9: f64,
        var_tnom: f64,
    ) {
        if (s.b[493] && s.b[494]) {
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);
            s.store_scaled_mul(96, 95, 271, (p.p4 * (p.p5 * 1.0 / (p.p187))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(280), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(275), (p.p25 * p.p25), s.ad_value(275)), 1.0);
            s.store_div(93, 98, 92);
            s.store_mul(281, 93, 135);
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
            s.store_mul_scale_ad_lhs(83, A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 8.617087e-5, 146);
            s.store_sub_from_scalar_ad(88, p.p185, A::scale_offset(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), ((1.0 / (var_tnom)) * (p.p188)), (((-1.0)) * (p.p188))));
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
            s.store_scaled_powf_ad(97, A::scale(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 1.0 / (var_tnom)), p.p20, p.p189);
            s.store_scaled_powf_ad(89, A::scale(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 1.0 / (var_tnom)), p.p19, p.p190);
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
        }

    }

    pub(super) fn stamp_transient_block_20(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_tdev: f64,
        var_tdev_db0: f64,
        var_tdev_db1: f64,
        var_tdev_db10: f64,
        var_tdev_db11: f64,
        var_tdev_db12: f64,
        var_tdev_db13: f64,
        var_tdev_db14: f64,
        var_tdev_db15: f64,
        var_tdev_db16: f64,
        var_tdev_db17: f64,
        var_tdev_db18: f64,
        var_tdev_db19: f64,
        var_tdev_db2: f64,
        var_tdev_db20: f64,
        var_tdev_db21: f64,
        var_tdev_db22: f64,
        var_tdev_db23: f64,
        var_tdev_db24: f64,
        var_tdev_db25: f64,
        var_tdev_db26: f64,
        var_tdev_db27: f64,
        var_tdev_db28: f64,
        var_tdev_db29: f64,
        var_tdev_db3: f64,
        var_tdev_db30: f64,
        var_tdev_db31: f64,
        var_tdev_db32: f64,
        var_tdev_db33: f64,
        var_tdev_db34: f64,
        var_tdev_db35: f64,
        var_tdev_db36: f64,
        var_tdev_db37: f64,
        var_tdev_db38: f64,
        var_tdev_db39: f64,
        var_tdev_db4: f64,
        var_tdev_db40: f64,
        var_tdev_db41: f64,
        var_tdev_db42: f64,
        var_tdev_db43: f64,
        var_tdev_db44: f64,
        var_tdev_db45: f64,
        var_tdev_db46: f64,
        var_tdev_db47: f64,
        var_tdev_db48: f64,
        var_tdev_db49: f64,
        var_tdev_db5: f64,
        var_tdev_db50: f64,
        var_tdev_db51: f64,
        var_tdev_db52: f64,
        var_tdev_db53: f64,
        var_tdev_db54: f64,
        var_tdev_db55: f64,
        var_tdev_db56: f64,
        var_tdev_db6: f64,
        var_tdev_db7: f64,
        var_tdev_db8: f64,
        var_tdev_db9: f64,
        var_tdev_dn0: f64,
        var_tdev_dn1: f64,
        var_tdev_dn10: f64,
        var_tdev_dn11: f64,
        var_tdev_dn12: f64,
        var_tdev_dn13: f64,
        var_tdev_dn14: f64,
        var_tdev_dn15: f64,
        var_tdev_dn16: f64,
        var_tdev_dn17: f64,
        var_tdev_dn18: f64,
        var_tdev_dn19: f64,
        var_tdev_dn2: f64,
        var_tdev_dn20: f64,
        var_tdev_dn21: f64,
        var_tdev_dn22: f64,
        var_tdev_dn3: f64,
        var_tdev_dn4: f64,
        var_tdev_dn5: f64,
        var_tdev_dn6: f64,
        var_tdev_dn7: f64,
        var_tdev_dn8: f64,
        var_tdev_dn9: f64,
        var_tnom: f64,
    ) {
        if (((!s.b[493]) && s.b[502]) && s.b[506]) {
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
            s.copy_ad(290, 71);
        }

        if ((s.b[508] && s.b[509]) && (!s.b[511])) {
            s.copy_ad(291, 69);
            s.copy_ad(290, 70);
        }

        if (s.b[508] && s.b[509]) {
            s.store_offset_sqrt_ad(292, A::offset(A::square(s.ad_value(291)), 0.01), (-0.1));
            s.store_offset_scaled(146, 292, p.p192, (1.0 + p.p191));
            s.store_mul_scale_ad_lhs(83, A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 8.617087e-5, 146);
            s.store_sub_ad(88, A::scale_offset(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), ((1.0 / (var_tnom)) * (p.p188)), (((((-1.0)) * (p.p188))) + (p.p185))), A::div_scaled_inputs(s.ad_value(292), (p.p194 * p.p193), A::sqrt_square_offset(s.ad_value(292), (p.p194 * p.p194)), 1.0));
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

    }

    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
        var_tdev: f64,
        var_tdev_db0: f64,
        var_tdev_db1: f64,
        var_tdev_db10: f64,
        var_tdev_db11: f64,
        var_tdev_db12: f64,
        var_tdev_db13: f64,
        var_tdev_db14: f64,
        var_tdev_db15: f64,
        var_tdev_db16: f64,
        var_tdev_db17: f64,
        var_tdev_db18: f64,
        var_tdev_db19: f64,
        var_tdev_db2: f64,
        var_tdev_db20: f64,
        var_tdev_db21: f64,
        var_tdev_db22: f64,
        var_tdev_db23: f64,
        var_tdev_db24: f64,
        var_tdev_db25: f64,
        var_tdev_db26: f64,
        var_tdev_db27: f64,
        var_tdev_db28: f64,
        var_tdev_db29: f64,
        var_tdev_db3: f64,
        var_tdev_db30: f64,
        var_tdev_db31: f64,
        var_tdev_db32: f64,
        var_tdev_db33: f64,
        var_tdev_db34: f64,
        var_tdev_db35: f64,
        var_tdev_db36: f64,
        var_tdev_db37: f64,
        var_tdev_db38: f64,
        var_tdev_db39: f64,
        var_tdev_db4: f64,
        var_tdev_db40: f64,
        var_tdev_db41: f64,
        var_tdev_db42: f64,
        var_tdev_db43: f64,
        var_tdev_db44: f64,
        var_tdev_db45: f64,
        var_tdev_db46: f64,
        var_tdev_db47: f64,
        var_tdev_db48: f64,
        var_tdev_db49: f64,
        var_tdev_db5: f64,
        var_tdev_db50: f64,
        var_tdev_db51: f64,
        var_tdev_db52: f64,
        var_tdev_db53: f64,
        var_tdev_db54: f64,
        var_tdev_db55: f64,
        var_tdev_db56: f64,
        var_tdev_db6: f64,
        var_tdev_db7: f64,
        var_tdev_db8: f64,
        var_tdev_db9: f64,
        var_tdev_dn0: f64,
        var_tdev_dn1: f64,
        var_tdev_dn10: f64,
        var_tdev_dn11: f64,
        var_tdev_dn12: f64,
        var_tdev_dn13: f64,
        var_tdev_dn14: f64,
        var_tdev_dn15: f64,
        var_tdev_dn16: f64,
        var_tdev_dn17: f64,
        var_tdev_dn18: f64,
        var_tdev_dn19: f64,
        var_tdev_dn2: f64,
        var_tdev_dn20: f64,
        var_tdev_dn21: f64,
        var_tdev_dn22: f64,
        var_tdev_dn3: f64,
        var_tdev_dn4: f64,
        var_tdev_dn5: f64,
        var_tdev_dn6: f64,
        var_tdev_dn7: f64,
        var_tdev_dn8: f64,
        var_tdev_dn9: f64,
        var_tnom: f64,
    ) {
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
            s.store_scaled_powf_ad(97, A::scale(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 1.0 / (var_tnom)), p.p20, p.p189);
            s.store_scaled_powf_ad(89, A::scale(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 1.0 / (var_tnom)), p.p19, p.p190);
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
        }

    }

    pub(super) fn stamp_transient_block_22(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_tdev: f64,
        var_tdev_db0: f64,
        var_tdev_db1: f64,
        var_tdev_db10: f64,
        var_tdev_db11: f64,
        var_tdev_db12: f64,
        var_tdev_db13: f64,
        var_tdev_db14: f64,
        var_tdev_db15: f64,
        var_tdev_db16: f64,
        var_tdev_db17: f64,
        var_tdev_db18: f64,
        var_tdev_db19: f64,
        var_tdev_db2: f64,
        var_tdev_db20: f64,
        var_tdev_db21: f64,
        var_tdev_db22: f64,
        var_tdev_db23: f64,
        var_tdev_db24: f64,
        var_tdev_db25: f64,
        var_tdev_db26: f64,
        var_tdev_db27: f64,
        var_tdev_db28: f64,
        var_tdev_db29: f64,
        var_tdev_db3: f64,
        var_tdev_db30: f64,
        var_tdev_db31: f64,
        var_tdev_db32: f64,
        var_tdev_db33: f64,
        var_tdev_db34: f64,
        var_tdev_db35: f64,
        var_tdev_db36: f64,
        var_tdev_db37: f64,
        var_tdev_db38: f64,
        var_tdev_db39: f64,
        var_tdev_db4: f64,
        var_tdev_db40: f64,
        var_tdev_db41: f64,
        var_tdev_db42: f64,
        var_tdev_db43: f64,
        var_tdev_db44: f64,
        var_tdev_db45: f64,
        var_tdev_db46: f64,
        var_tdev_db47: f64,
        var_tdev_db48: f64,
        var_tdev_db49: f64,
        var_tdev_db5: f64,
        var_tdev_db50: f64,
        var_tdev_db51: f64,
        var_tdev_db52: f64,
        var_tdev_db53: f64,
        var_tdev_db54: f64,
        var_tdev_db55: f64,
        var_tdev_db56: f64,
        var_tdev_db6: f64,
        var_tdev_db7: f64,
        var_tdev_db8: f64,
        var_tdev_db9: f64,
        var_tdev_dn0: f64,
        var_tdev_dn1: f64,
        var_tdev_dn10: f64,
        var_tdev_dn11: f64,
        var_tdev_dn12: f64,
        var_tdev_dn13: f64,
        var_tdev_dn14: f64,
        var_tdev_dn15: f64,
        var_tdev_dn16: f64,
        var_tdev_dn17: f64,
        var_tdev_dn18: f64,
        var_tdev_dn19: f64,
        var_tdev_dn2: f64,
        var_tdev_dn20: f64,
        var_tdev_dn21: f64,
        var_tdev_dn22: f64,
        var_tdev_dn3: f64,
        var_tdev_dn4: f64,
        var_tdev_dn5: f64,
        var_tdev_dn6: f64,
        var_tdev_dn7: f64,
        var_tdev_dn8: f64,
        var_tdev_dn9: f64,
        var_tnom: f64,
    ) {
        if (s.b[508] && s.b[509]) {
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);
            s.store_scaled_mul(96, 95, 283, (p.p4 * (p.p5 * 1.0 / (p.p187))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(292), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(287), (p.p25 * p.p25), s.ad_value(287)), 1.0);
            s.store_div(93, 98, 92);
            s.store_mul(293, 93, 135);
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
            s.store_mul_scale_ad_lhs(83, A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 8.617087e-5, 146);
            s.store_offset_scaled_ad(88, A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), ((1.0 / (var_tnom)) * (p.p188)), (((((-1.0)) * (p.p188))) + (p.p185)));
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
            s.store_scaled_powf_ad(97, A::scale(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 1.0 / (var_tnom)), p.p20, p.p189);
            s.store_scaled_powf_ad(89, A::scale(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 1.0 / (var_tnom)), p.p19, p.p190);
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
        }

    }

    pub(super) fn stamp_transient_block_23(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_tdev: f64,
        var_tdev_db0: f64,
        var_tdev_db1: f64,
        var_tdev_db10: f64,
        var_tdev_db11: f64,
        var_tdev_db12: f64,
        var_tdev_db13: f64,
        var_tdev_db14: f64,
        var_tdev_db15: f64,
        var_tdev_db16: f64,
        var_tdev_db17: f64,
        var_tdev_db18: f64,
        var_tdev_db19: f64,
        var_tdev_db2: f64,
        var_tdev_db20: f64,
        var_tdev_db21: f64,
        var_tdev_db22: f64,
        var_tdev_db23: f64,
        var_tdev_db24: f64,
        var_tdev_db25: f64,
        var_tdev_db26: f64,
        var_tdev_db27: f64,
        var_tdev_db28: f64,
        var_tdev_db29: f64,
        var_tdev_db3: f64,
        var_tdev_db30: f64,
        var_tdev_db31: f64,
        var_tdev_db32: f64,
        var_tdev_db33: f64,
        var_tdev_db34: f64,
        var_tdev_db35: f64,
        var_tdev_db36: f64,
        var_tdev_db37: f64,
        var_tdev_db38: f64,
        var_tdev_db39: f64,
        var_tdev_db4: f64,
        var_tdev_db40: f64,
        var_tdev_db41: f64,
        var_tdev_db42: f64,
        var_tdev_db43: f64,
        var_tdev_db44: f64,
        var_tdev_db45: f64,
        var_tdev_db46: f64,
        var_tdev_db47: f64,
        var_tdev_db48: f64,
        var_tdev_db49: f64,
        var_tdev_db5: f64,
        var_tdev_db50: f64,
        var_tdev_db51: f64,
        var_tdev_db52: f64,
        var_tdev_db53: f64,
        var_tdev_db54: f64,
        var_tdev_db55: f64,
        var_tdev_db56: f64,
        var_tdev_db6: f64,
        var_tdev_db7: f64,
        var_tdev_db8: f64,
        var_tdev_db9: f64,
        var_tdev_dn0: f64,
        var_tdev_dn1: f64,
        var_tdev_dn10: f64,
        var_tdev_dn11: f64,
        var_tdev_dn12: f64,
        var_tdev_dn13: f64,
        var_tdev_dn14: f64,
        var_tdev_dn15: f64,
        var_tdev_dn16: f64,
        var_tdev_dn17: f64,
        var_tdev_dn18: f64,
        var_tdev_dn19: f64,
        var_tdev_dn2: f64,
        var_tdev_dn20: f64,
        var_tdev_dn21: f64,
        var_tdev_dn22: f64,
        var_tdev_dn3: f64,
        var_tdev_dn4: f64,
        var_tdev_dn5: f64,
        var_tdev_dn6: f64,
        var_tdev_dn7: f64,
        var_tdev_dn8: f64,
        var_tdev_dn9: f64,
        var_tnom: f64,
    ) {
        if (((!s.b[508]) && s.b[517]) && s.b[521]) {
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
            s.copy_ad(302, 75);
        }

        if ((s.b[523] && s.b[524]) && (!s.b[526])) {
            s.copy_ad(303, 73);
            s.copy_ad(302, 74);
        }

        if (s.b[523] && s.b[524]) {
            s.store_offset_sqrt_ad(304, A::offset(A::square(s.ad_value(303)), 0.01), (-0.1));
            s.store_offset_scaled(146, 304, p.p205, (1.0 + p.p204));
            s.store_mul_scale_ad_lhs(83, A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 8.617087e-5, 146);
            s.store_sub_ad(88, A::sub_from_scalar(p.p198, A::scale_offset(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), ((1.0 / (var_tnom)) * (p.p201)), (((-1.0)) * (p.p201)))), A::div_scaled_inputs(s.ad_value(304), (p.p207 * p.p206), A::sqrt_square_offset(s.ad_value(304), (p.p207 * p.p207)), 1.0));
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

    }

    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
        p: &Parameters,
        var_tdev: f64,
        var_tdev_db0: f64,
        var_tdev_db1: f64,
        var_tdev_db10: f64,
        var_tdev_db11: f64,
        var_tdev_db12: f64,
        var_tdev_db13: f64,
        var_tdev_db14: f64,
        var_tdev_db15: f64,
        var_tdev_db16: f64,
        var_tdev_db17: f64,
        var_tdev_db18: f64,
        var_tdev_db19: f64,
        var_tdev_db2: f64,
        var_tdev_db20: f64,
        var_tdev_db21: f64,
        var_tdev_db22: f64,
        var_tdev_db23: f64,
        var_tdev_db24: f64,
        var_tdev_db25: f64,
        var_tdev_db26: f64,
        var_tdev_db27: f64,
        var_tdev_db28: f64,
        var_tdev_db29: f64,
        var_tdev_db3: f64,
        var_tdev_db30: f64,
        var_tdev_db31: f64,
        var_tdev_db32: f64,
        var_tdev_db33: f64,
        var_tdev_db34: f64,
        var_tdev_db35: f64,
        var_tdev_db36: f64,
        var_tdev_db37: f64,
        var_tdev_db38: f64,
        var_tdev_db39: f64,
        var_tdev_db4: f64,
        var_tdev_db40: f64,
        var_tdev_db41: f64,
        var_tdev_db42: f64,
        var_tdev_db43: f64,
        var_tdev_db44: f64,
        var_tdev_db45: f64,
        var_tdev_db46: f64,
        var_tdev_db47: f64,
        var_tdev_db48: f64,
        var_tdev_db49: f64,
        var_tdev_db5: f64,
        var_tdev_db50: f64,
        var_tdev_db51: f64,
        var_tdev_db52: f64,
        var_tdev_db53: f64,
        var_tdev_db54: f64,
        var_tdev_db55: f64,
        var_tdev_db56: f64,
        var_tdev_db6: f64,
        var_tdev_db7: f64,
        var_tdev_db8: f64,
        var_tdev_db9: f64,
        var_tdev_dn0: f64,
        var_tdev_dn1: f64,
        var_tdev_dn10: f64,
        var_tdev_dn11: f64,
        var_tdev_dn12: f64,
        var_tdev_dn13: f64,
        var_tdev_dn14: f64,
        var_tdev_dn15: f64,
        var_tdev_dn16: f64,
        var_tdev_dn17: f64,
        var_tdev_dn18: f64,
        var_tdev_dn19: f64,
        var_tdev_dn2: f64,
        var_tdev_dn20: f64,
        var_tdev_dn21: f64,
        var_tdev_dn22: f64,
        var_tdev_dn3: f64,
        var_tdev_dn4: f64,
        var_tdev_dn5: f64,
        var_tdev_dn6: f64,
        var_tdev_dn7: f64,
        var_tdev_dn8: f64,
        var_tdev_dn9: f64,
        var_tnom: f64,
    ) {
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
            s.store_scaled_powf_ad(97, A::scale(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 1.0 / (var_tnom)), p.p20, p.p202);
            s.store_scaled_powf_ad(89, A::scale(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 1.0 / (var_tnom)), p.p19, p.p203);
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
        }

    }

    pub(super) fn stamp_transient_block_25(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_tdev: f64,
        var_tdev_db0: f64,
        var_tdev_db1: f64,
        var_tdev_db10: f64,
        var_tdev_db11: f64,
        var_tdev_db12: f64,
        var_tdev_db13: f64,
        var_tdev_db14: f64,
        var_tdev_db15: f64,
        var_tdev_db16: f64,
        var_tdev_db17: f64,
        var_tdev_db18: f64,
        var_tdev_db19: f64,
        var_tdev_db2: f64,
        var_tdev_db20: f64,
        var_tdev_db21: f64,
        var_tdev_db22: f64,
        var_tdev_db23: f64,
        var_tdev_db24: f64,
        var_tdev_db25: f64,
        var_tdev_db26: f64,
        var_tdev_db27: f64,
        var_tdev_db28: f64,
        var_tdev_db29: f64,
        var_tdev_db3: f64,
        var_tdev_db30: f64,
        var_tdev_db31: f64,
        var_tdev_db32: f64,
        var_tdev_db33: f64,
        var_tdev_db34: f64,
        var_tdev_db35: f64,
        var_tdev_db36: f64,
        var_tdev_db37: f64,
        var_tdev_db38: f64,
        var_tdev_db39: f64,
        var_tdev_db4: f64,
        var_tdev_db40: f64,
        var_tdev_db41: f64,
        var_tdev_db42: f64,
        var_tdev_db43: f64,
        var_tdev_db44: f64,
        var_tdev_db45: f64,
        var_tdev_db46: f64,
        var_tdev_db47: f64,
        var_tdev_db48: f64,
        var_tdev_db49: f64,
        var_tdev_db5: f64,
        var_tdev_db50: f64,
        var_tdev_db51: f64,
        var_tdev_db52: f64,
        var_tdev_db53: f64,
        var_tdev_db54: f64,
        var_tdev_db55: f64,
        var_tdev_db56: f64,
        var_tdev_db6: f64,
        var_tdev_db7: f64,
        var_tdev_db8: f64,
        var_tdev_db9: f64,
        var_tdev_dn0: f64,
        var_tdev_dn1: f64,
        var_tdev_dn10: f64,
        var_tdev_dn11: f64,
        var_tdev_dn12: f64,
        var_tdev_dn13: f64,
        var_tdev_dn14: f64,
        var_tdev_dn15: f64,
        var_tdev_dn16: f64,
        var_tdev_dn17: f64,
        var_tdev_dn18: f64,
        var_tdev_dn19: f64,
        var_tdev_dn2: f64,
        var_tdev_dn20: f64,
        var_tdev_dn21: f64,
        var_tdev_dn22: f64,
        var_tdev_dn3: f64,
        var_tdev_dn4: f64,
        var_tdev_dn5: f64,
        var_tdev_dn6: f64,
        var_tdev_dn7: f64,
        var_tdev_dn8: f64,
        var_tdev_dn9: f64,
        var_tnom: f64,
    ) {
        if (s.b[523] && s.b[524]) {
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);
            s.store_scaled_mul(96, 95, 295, (p.p4 * (p.p5 * 1.0 / (p.p200))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(304), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(299), (p.p25 * p.p25), s.ad_value(299)), 1.0);
            s.store_div(93, 98, 92);
            s.store_mul(305, 93, 135);
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
            s.store_mul_scale_ad_lhs(83, A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 8.617087e-5, 146);
            s.store_sub_from_scalar_ad(88, p.p198, A::scale_offset(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), ((1.0 / (var_tnom)) * (p.p201)), (((-1.0)) * (p.p201))));
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
            s.store_scaled_powf_ad(97, A::scale(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 1.0 / (var_tnom)), p.p20, p.p202);
            s.store_scaled_powf_ad(89, A::scale(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 1.0 / (var_tnom)), p.p19, p.p203);
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
        }

    }

    pub(super) fn stamp_transient_block_26(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_tdev: f64,
        var_tdev_db0: f64,
        var_tdev_db1: f64,
        var_tdev_db10: f64,
        var_tdev_db11: f64,
        var_tdev_db12: f64,
        var_tdev_db13: f64,
        var_tdev_db14: f64,
        var_tdev_db15: f64,
        var_tdev_db16: f64,
        var_tdev_db17: f64,
        var_tdev_db18: f64,
        var_tdev_db19: f64,
        var_tdev_db2: f64,
        var_tdev_db20: f64,
        var_tdev_db21: f64,
        var_tdev_db22: f64,
        var_tdev_db23: f64,
        var_tdev_db24: f64,
        var_tdev_db25: f64,
        var_tdev_db26: f64,
        var_tdev_db27: f64,
        var_tdev_db28: f64,
        var_tdev_db29: f64,
        var_tdev_db3: f64,
        var_tdev_db30: f64,
        var_tdev_db31: f64,
        var_tdev_db32: f64,
        var_tdev_db33: f64,
        var_tdev_db34: f64,
        var_tdev_db35: f64,
        var_tdev_db36: f64,
        var_tdev_db37: f64,
        var_tdev_db38: f64,
        var_tdev_db39: f64,
        var_tdev_db4: f64,
        var_tdev_db40: f64,
        var_tdev_db41: f64,
        var_tdev_db42: f64,
        var_tdev_db43: f64,
        var_tdev_db44: f64,
        var_tdev_db45: f64,
        var_tdev_db46: f64,
        var_tdev_db47: f64,
        var_tdev_db48: f64,
        var_tdev_db49: f64,
        var_tdev_db5: f64,
        var_tdev_db50: f64,
        var_tdev_db51: f64,
        var_tdev_db52: f64,
        var_tdev_db53: f64,
        var_tdev_db54: f64,
        var_tdev_db55: f64,
        var_tdev_db56: f64,
        var_tdev_db6: f64,
        var_tdev_db7: f64,
        var_tdev_db8: f64,
        var_tdev_db9: f64,
        var_tdev_dn0: f64,
        var_tdev_dn1: f64,
        var_tdev_dn10: f64,
        var_tdev_dn11: f64,
        var_tdev_dn12: f64,
        var_tdev_dn13: f64,
        var_tdev_dn14: f64,
        var_tdev_dn15: f64,
        var_tdev_dn16: f64,
        var_tdev_dn17: f64,
        var_tdev_dn18: f64,
        var_tdev_dn19: f64,
        var_tdev_dn2: f64,
        var_tdev_dn20: f64,
        var_tdev_dn21: f64,
        var_tdev_dn22: f64,
        var_tdev_dn3: f64,
        var_tdev_dn4: f64,
        var_tdev_dn5: f64,
        var_tdev_dn6: f64,
        var_tdev_dn7: f64,
        var_tdev_dn8: f64,
        var_tdev_dn9: f64,
        var_tnom: f64,
    ) {
        if (((!s.b[523]) && s.b[532]) && s.b[536]) {
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
            s.copy_ad(314, 79);
        }

        if ((s.b[538] && s.b[539]) && (!s.b[541])) {
            s.copy_ad(315, 77);
            s.copy_ad(314, 78);
        }

        if (s.b[538] && s.b[539]) {
            s.store_offset_sqrt_ad(316, A::offset(A::square(s.ad_value(315)), 0.01), (-0.1));
            s.store_offset_scaled(146, 316, p.p205, (1.0 + p.p204));
            s.store_mul_scale_ad_lhs(83, A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 8.617087e-5, 146);
            s.store_sub_ad(88, A::scale_offset(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), ((1.0 / (var_tnom)) * (p.p201)), (((((-1.0)) * (p.p201))) + (p.p198))), A::div_scaled_inputs(s.ad_value(316), (p.p207 * p.p206), A::sqrt_square_offset(s.ad_value(316), (p.p207 * p.p207)), 1.0));
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

    }

    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
        p: &Parameters,
        var_tdev: f64,
        var_tdev_db0: f64,
        var_tdev_db1: f64,
        var_tdev_db10: f64,
        var_tdev_db11: f64,
        var_tdev_db12: f64,
        var_tdev_db13: f64,
        var_tdev_db14: f64,
        var_tdev_db15: f64,
        var_tdev_db16: f64,
        var_tdev_db17: f64,
        var_tdev_db18: f64,
        var_tdev_db19: f64,
        var_tdev_db2: f64,
        var_tdev_db20: f64,
        var_tdev_db21: f64,
        var_tdev_db22: f64,
        var_tdev_db23: f64,
        var_tdev_db24: f64,
        var_tdev_db25: f64,
        var_tdev_db26: f64,
        var_tdev_db27: f64,
        var_tdev_db28: f64,
        var_tdev_db29: f64,
        var_tdev_db3: f64,
        var_tdev_db30: f64,
        var_tdev_db31: f64,
        var_tdev_db32: f64,
        var_tdev_db33: f64,
        var_tdev_db34: f64,
        var_tdev_db35: f64,
        var_tdev_db36: f64,
        var_tdev_db37: f64,
        var_tdev_db38: f64,
        var_tdev_db39: f64,
        var_tdev_db4: f64,
        var_tdev_db40: f64,
        var_tdev_db41: f64,
        var_tdev_db42: f64,
        var_tdev_db43: f64,
        var_tdev_db44: f64,
        var_tdev_db45: f64,
        var_tdev_db46: f64,
        var_tdev_db47: f64,
        var_tdev_db48: f64,
        var_tdev_db49: f64,
        var_tdev_db5: f64,
        var_tdev_db50: f64,
        var_tdev_db51: f64,
        var_tdev_db52: f64,
        var_tdev_db53: f64,
        var_tdev_db54: f64,
        var_tdev_db55: f64,
        var_tdev_db56: f64,
        var_tdev_db6: f64,
        var_tdev_db7: f64,
        var_tdev_db8: f64,
        var_tdev_db9: f64,
        var_tdev_dn0: f64,
        var_tdev_dn1: f64,
        var_tdev_dn10: f64,
        var_tdev_dn11: f64,
        var_tdev_dn12: f64,
        var_tdev_dn13: f64,
        var_tdev_dn14: f64,
        var_tdev_dn15: f64,
        var_tdev_dn16: f64,
        var_tdev_dn17: f64,
        var_tdev_dn18: f64,
        var_tdev_dn19: f64,
        var_tdev_dn2: f64,
        var_tdev_dn20: f64,
        var_tdev_dn21: f64,
        var_tdev_dn22: f64,
        var_tdev_dn3: f64,
        var_tdev_dn4: f64,
        var_tdev_dn5: f64,
        var_tdev_dn6: f64,
        var_tdev_dn7: f64,
        var_tdev_dn8: f64,
        var_tdev_dn9: f64,
        var_tnom: f64,
    ) {
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
            s.store_scaled_powf_ad(97, A::scale(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 1.0 / (var_tnom)), p.p20, p.p202);
            s.store_scaled_powf_ad(89, A::scale(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 1.0 / (var_tnom)), p.p19, p.p203);
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
        }

    }

    pub(super) fn stamp_transient_block_28(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_tdev: f64,
        var_tdev_db0: f64,
        var_tdev_db1: f64,
        var_tdev_db10: f64,
        var_tdev_db11: f64,
        var_tdev_db12: f64,
        var_tdev_db13: f64,
        var_tdev_db14: f64,
        var_tdev_db15: f64,
        var_tdev_db16: f64,
        var_tdev_db17: f64,
        var_tdev_db18: f64,
        var_tdev_db19: f64,
        var_tdev_db2: f64,
        var_tdev_db20: f64,
        var_tdev_db21: f64,
        var_tdev_db22: f64,
        var_tdev_db23: f64,
        var_tdev_db24: f64,
        var_tdev_db25: f64,
        var_tdev_db26: f64,
        var_tdev_db27: f64,
        var_tdev_db28: f64,
        var_tdev_db29: f64,
        var_tdev_db3: f64,
        var_tdev_db30: f64,
        var_tdev_db31: f64,
        var_tdev_db32: f64,
        var_tdev_db33: f64,
        var_tdev_db34: f64,
        var_tdev_db35: f64,
        var_tdev_db36: f64,
        var_tdev_db37: f64,
        var_tdev_db38: f64,
        var_tdev_db39: f64,
        var_tdev_db4: f64,
        var_tdev_db40: f64,
        var_tdev_db41: f64,
        var_tdev_db42: f64,
        var_tdev_db43: f64,
        var_tdev_db44: f64,
        var_tdev_db45: f64,
        var_tdev_db46: f64,
        var_tdev_db47: f64,
        var_tdev_db48: f64,
        var_tdev_db49: f64,
        var_tdev_db5: f64,
        var_tdev_db50: f64,
        var_tdev_db51: f64,
        var_tdev_db52: f64,
        var_tdev_db53: f64,
        var_tdev_db54: f64,
        var_tdev_db55: f64,
        var_tdev_db56: f64,
        var_tdev_db6: f64,
        var_tdev_db7: f64,
        var_tdev_db8: f64,
        var_tdev_db9: f64,
        var_tdev_dn0: f64,
        var_tdev_dn1: f64,
        var_tdev_dn10: f64,
        var_tdev_dn11: f64,
        var_tdev_dn12: f64,
        var_tdev_dn13: f64,
        var_tdev_dn14: f64,
        var_tdev_dn15: f64,
        var_tdev_dn16: f64,
        var_tdev_dn17: f64,
        var_tdev_dn18: f64,
        var_tdev_dn19: f64,
        var_tdev_dn2: f64,
        var_tdev_dn20: f64,
        var_tdev_dn21: f64,
        var_tdev_dn22: f64,
        var_tdev_dn3: f64,
        var_tdev_dn4: f64,
        var_tdev_dn5: f64,
        var_tdev_dn6: f64,
        var_tdev_dn7: f64,
        var_tdev_dn8: f64,
        var_tdev_dn9: f64,
        var_tnom: f64,
    ) {
        if (s.b[538] && s.b[539]) {
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);
            s.store_scaled_mul(96, 95, 307, (p.p4 * (p.p5 * 1.0 / (p.p200))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(316), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(311), (p.p25 * p.p25), s.ad_value(311)), 1.0);
            s.store_div(93, 98, 92);
            s.store_mul(317, 93, 135);
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
            s.store_mul_scale_ad_lhs(83, A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 8.617087e-5, 146);
            s.store_offset_scaled_ad(88, A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), ((1.0 / (var_tnom)) * (p.p201)), (((((-1.0)) * (p.p201))) + (p.p198)));
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
            s.store_scaled_powf_ad(97, A::scale(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 1.0 / (var_tnom)), p.p20, p.p202);
            s.store_scaled_powf_ad(89, A::scale(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 1.0 / (var_tnom)), p.p19, p.p203);
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
        }

    }

    pub(super) fn stamp_transient_block_29(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_cgdl_l_slot: &mut f64,
        var_guard524_slot: &mut f64,
        var_qsov_slot: &mut f64,
        var_qsov_db0_slot: &mut f64,
        var_qsov_db1_slot: &mut f64,
        var_qsov_db10_slot: &mut f64,
        var_qsov_db11_slot: &mut f64,
        var_qsov_db12_slot: &mut f64,
        var_qsov_db13_slot: &mut f64,
        var_qsov_db14_slot: &mut f64,
        var_qsov_db15_slot: &mut f64,
        var_qsov_db16_slot: &mut f64,
        var_qsov_db17_slot: &mut f64,
        var_qsov_db18_slot: &mut f64,
        var_qsov_db19_slot: &mut f64,
        var_qsov_db2_slot: &mut f64,
        var_qsov_db20_slot: &mut f64,
        var_qsov_db21_slot: &mut f64,
        var_qsov_db22_slot: &mut f64,
        var_qsov_db23_slot: &mut f64,
        var_qsov_db24_slot: &mut f64,
        var_qsov_db25_slot: &mut f64,
        var_qsov_db26_slot: &mut f64,
        var_qsov_db27_slot: &mut f64,
        var_qsov_db28_slot: &mut f64,
        var_qsov_db29_slot: &mut f64,
        var_qsov_db3_slot: &mut f64,
        var_qsov_db30_slot: &mut f64,
        var_qsov_db31_slot: &mut f64,
        var_qsov_db32_slot: &mut f64,
        var_qsov_db33_slot: &mut f64,
        var_qsov_db34_slot: &mut f64,
        var_qsov_db35_slot: &mut f64,
        var_qsov_db36_slot: &mut f64,
        var_qsov_db37_slot: &mut f64,
        var_qsov_db38_slot: &mut f64,
        var_qsov_db39_slot: &mut f64,
        var_qsov_db4_slot: &mut f64,
        var_qsov_db40_slot: &mut f64,
        var_qsov_db41_slot: &mut f64,
        var_qsov_db42_slot: &mut f64,
        var_qsov_db43_slot: &mut f64,
        var_qsov_db44_slot: &mut f64,
        var_qsov_db45_slot: &mut f64,
        var_qsov_db46_slot: &mut f64,
        var_qsov_db47_slot: &mut f64,
        var_qsov_db48_slot: &mut f64,
        var_qsov_db49_slot: &mut f64,
        var_qsov_db5_slot: &mut f64,
        var_qsov_db50_slot: &mut f64,
        var_qsov_db51_slot: &mut f64,
        var_qsov_db52_slot: &mut f64,
        var_qsov_db53_slot: &mut f64,
        var_qsov_db54_slot: &mut f64,
        var_qsov_db55_slot: &mut f64,
        var_qsov_db56_slot: &mut f64,
        var_qsov_db6_slot: &mut f64,
        var_qsov_db7_slot: &mut f64,
        var_qsov_db8_slot: &mut f64,
        var_qsov_db9_slot: &mut f64,
        var_qsov_dn0_slot: &mut f64,
        var_qsov_dn1_slot: &mut f64,
        var_qsov_dn10_slot: &mut f64,
        var_qsov_dn11_slot: &mut f64,
        var_qsov_dn12_slot: &mut f64,
        var_qsov_dn13_slot: &mut f64,
        var_qsov_dn14_slot: &mut f64,
        var_qsov_dn15_slot: &mut f64,
        var_qsov_dn16_slot: &mut f64,
        var_qsov_dn17_slot: &mut f64,
        var_qsov_dn18_slot: &mut f64,
        var_qsov_dn19_slot: &mut f64,
        var_qsov_dn2_slot: &mut f64,
        var_qsov_dn20_slot: &mut f64,
        var_qsov_dn21_slot: &mut f64,
        var_qsov_dn22_slot: &mut f64,
        var_qsov_dn3_slot: &mut f64,
        var_qsov_dn4_slot: &mut f64,
        var_qsov_dn5_slot: &mut f64,
        var_qsov_dn6_slot: &mut f64,
        var_qsov_dn7_slot: &mut f64,
        var_qsov_dn8_slot: &mut f64,
        var_qsov_dn9_slot: &mut f64,
        var_vdseffcv_slot: &mut f64,
        var_vdseffcv_db0_slot: &mut f64,
        var_vdseffcv_db1_slot: &mut f64,
        var_vdseffcv_db10_slot: &mut f64,
        var_vdseffcv_db11_slot: &mut f64,
        var_vdseffcv_db12_slot: &mut f64,
        var_vdseffcv_db13_slot: &mut f64,
        var_vdseffcv_db14_slot: &mut f64,
        var_vdseffcv_db15_slot: &mut f64,
        var_vdseffcv_db16_slot: &mut f64,
        var_vdseffcv_db17_slot: &mut f64,
        var_vdseffcv_db18_slot: &mut f64,
        var_vdseffcv_db19_slot: &mut f64,
        var_vdseffcv_db2_slot: &mut f64,
        var_vdseffcv_db20_slot: &mut f64,
        var_vdseffcv_db21_slot: &mut f64,
        var_vdseffcv_db22_slot: &mut f64,
        var_vdseffcv_db23_slot: &mut f64,
        var_vdseffcv_db24_slot: &mut f64,
        var_vdseffcv_db25_slot: &mut f64,
        var_vdseffcv_db26_slot: &mut f64,
        var_vdseffcv_db27_slot: &mut f64,
        var_vdseffcv_db28_slot: &mut f64,
        var_vdseffcv_db29_slot: &mut f64,
        var_vdseffcv_db3_slot: &mut f64,
        var_vdseffcv_db30_slot: &mut f64,
        var_vdseffcv_db31_slot: &mut f64,
        var_vdseffcv_db32_slot: &mut f64,
        var_vdseffcv_db33_slot: &mut f64,
        var_vdseffcv_db34_slot: &mut f64,
        var_vdseffcv_db35_slot: &mut f64,
        var_vdseffcv_db36_slot: &mut f64,
        var_vdseffcv_db37_slot: &mut f64,
        var_vdseffcv_db38_slot: &mut f64,
        var_vdseffcv_db39_slot: &mut f64,
        var_vdseffcv_db4_slot: &mut f64,
        var_vdseffcv_db40_slot: &mut f64,
        var_vdseffcv_db41_slot: &mut f64,
        var_vdseffcv_db42_slot: &mut f64,
        var_vdseffcv_db43_slot: &mut f64,
        var_vdseffcv_db44_slot: &mut f64,
        var_vdseffcv_db45_slot: &mut f64,
        var_vdseffcv_db46_slot: &mut f64,
        var_vdseffcv_db47_slot: &mut f64,
        var_vdseffcv_db48_slot: &mut f64,
        var_vdseffcv_db49_slot: &mut f64,
        var_vdseffcv_db5_slot: &mut f64,
        var_vdseffcv_db50_slot: &mut f64,
        var_vdseffcv_db51_slot: &mut f64,
        var_vdseffcv_db52_slot: &mut f64,
        var_vdseffcv_db53_slot: &mut f64,
        var_vdseffcv_db54_slot: &mut f64,
        var_vdseffcv_db55_slot: &mut f64,
        var_vdseffcv_db56_slot: &mut f64,
        var_vdseffcv_db6_slot: &mut f64,
        var_vdseffcv_db7_slot: &mut f64,
        var_vdseffcv_db8_slot: &mut f64,
        var_vdseffcv_db9_slot: &mut f64,
        var_vdseffcv_dn0_slot: &mut f64,
        var_vdseffcv_dn1_slot: &mut f64,
        var_vdseffcv_dn10_slot: &mut f64,
        var_vdseffcv_dn11_slot: &mut f64,
        var_vdseffcv_dn12_slot: &mut f64,
        var_vdseffcv_dn13_slot: &mut f64,
        var_vdseffcv_dn14_slot: &mut f64,
        var_vdseffcv_dn15_slot: &mut f64,
        var_vdseffcv_dn16_slot: &mut f64,
        var_vdseffcv_dn17_slot: &mut f64,
        var_vdseffcv_dn18_slot: &mut f64,
        var_vdseffcv_dn19_slot: &mut f64,
        var_vdseffcv_dn2_slot: &mut f64,
        var_vdseffcv_dn20_slot: &mut f64,
        var_vdseffcv_dn21_slot: &mut f64,
        var_vdseffcv_dn22_slot: &mut f64,
        var_vdseffcv_dn3_slot: &mut f64,
        var_vdseffcv_dn4_slot: &mut f64,
        var_vdseffcv_dn5_slot: &mut f64,
        var_vdseffcv_dn6_slot: &mut f64,
        var_vdseffcv_dn7_slot: &mut f64,
        var_vdseffcv_dn8_slot: &mut f64,
        var_vdseffcv_dn9_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let mut var_cgdl_l: f64 = *var_cgdl_l_slot;
        let mut var_guard524: f64 = *var_guard524_slot;
        let mut var_qsov: f64 = *var_qsov_slot;
        let mut var_qsov_db0: f64 = *var_qsov_db0_slot;
        let mut var_qsov_db1: f64 = *var_qsov_db1_slot;
        let mut var_qsov_db10: f64 = *var_qsov_db10_slot;
        let mut var_qsov_db11: f64 = *var_qsov_db11_slot;
        let mut var_qsov_db12: f64 = *var_qsov_db12_slot;
        let mut var_qsov_db13: f64 = *var_qsov_db13_slot;
        let mut var_qsov_db14: f64 = *var_qsov_db14_slot;
        let mut var_qsov_db15: f64 = *var_qsov_db15_slot;
        let mut var_qsov_db16: f64 = *var_qsov_db16_slot;
        let mut var_qsov_db17: f64 = *var_qsov_db17_slot;
        let mut var_qsov_db18: f64 = *var_qsov_db18_slot;
        let mut var_qsov_db19: f64 = *var_qsov_db19_slot;
        let mut var_qsov_db2: f64 = *var_qsov_db2_slot;
        let mut var_qsov_db20: f64 = *var_qsov_db20_slot;
        let mut var_qsov_db21: f64 = *var_qsov_db21_slot;
        let mut var_qsov_db22: f64 = *var_qsov_db22_slot;
        let mut var_qsov_db23: f64 = *var_qsov_db23_slot;
        let mut var_qsov_db24: f64 = *var_qsov_db24_slot;
        let mut var_qsov_db25: f64 = *var_qsov_db25_slot;
        let mut var_qsov_db26: f64 = *var_qsov_db26_slot;
        let mut var_qsov_db27: f64 = *var_qsov_db27_slot;
        let mut var_qsov_db28: f64 = *var_qsov_db28_slot;
        let mut var_qsov_db29: f64 = *var_qsov_db29_slot;
        let mut var_qsov_db3: f64 = *var_qsov_db3_slot;
        let mut var_qsov_db30: f64 = *var_qsov_db30_slot;
        let mut var_qsov_db31: f64 = *var_qsov_db31_slot;
        let mut var_qsov_db32: f64 = *var_qsov_db32_slot;
        let mut var_qsov_db33: f64 = *var_qsov_db33_slot;
        let mut var_qsov_db34: f64 = *var_qsov_db34_slot;
        let mut var_qsov_db35: f64 = *var_qsov_db35_slot;
        let mut var_qsov_db36: f64 = *var_qsov_db36_slot;
        let mut var_qsov_db37: f64 = *var_qsov_db37_slot;
        let mut var_qsov_db38: f64 = *var_qsov_db38_slot;
        let mut var_qsov_db39: f64 = *var_qsov_db39_slot;
        let mut var_qsov_db4: f64 = *var_qsov_db4_slot;
        let mut var_qsov_db40: f64 = *var_qsov_db40_slot;
        let mut var_qsov_db41: f64 = *var_qsov_db41_slot;
        let mut var_qsov_db42: f64 = *var_qsov_db42_slot;
        let mut var_qsov_db43: f64 = *var_qsov_db43_slot;
        let mut var_qsov_db44: f64 = *var_qsov_db44_slot;
        let mut var_qsov_db45: f64 = *var_qsov_db45_slot;
        let mut var_qsov_db46: f64 = *var_qsov_db46_slot;
        let mut var_qsov_db47: f64 = *var_qsov_db47_slot;
        let mut var_qsov_db48: f64 = *var_qsov_db48_slot;
        let mut var_qsov_db49: f64 = *var_qsov_db49_slot;
        let mut var_qsov_db5: f64 = *var_qsov_db5_slot;
        let mut var_qsov_db50: f64 = *var_qsov_db50_slot;
        let mut var_qsov_db51: f64 = *var_qsov_db51_slot;
        let mut var_qsov_db52: f64 = *var_qsov_db52_slot;
        let mut var_qsov_db53: f64 = *var_qsov_db53_slot;
        let mut var_qsov_db54: f64 = *var_qsov_db54_slot;
        let mut var_qsov_db55: f64 = *var_qsov_db55_slot;
        let mut var_qsov_db56: f64 = *var_qsov_db56_slot;
        let mut var_qsov_db6: f64 = *var_qsov_db6_slot;
        let mut var_qsov_db7: f64 = *var_qsov_db7_slot;
        let mut var_qsov_db8: f64 = *var_qsov_db8_slot;
        let mut var_qsov_db9: f64 = *var_qsov_db9_slot;
        let mut var_qsov_dn0: f64 = *var_qsov_dn0_slot;
        let mut var_qsov_dn1: f64 = *var_qsov_dn1_slot;
        let mut var_qsov_dn10: f64 = *var_qsov_dn10_slot;
        let mut var_qsov_dn11: f64 = *var_qsov_dn11_slot;
        let mut var_qsov_dn12: f64 = *var_qsov_dn12_slot;
        let mut var_qsov_dn13: f64 = *var_qsov_dn13_slot;
        let mut var_qsov_dn14: f64 = *var_qsov_dn14_slot;
        let mut var_qsov_dn15: f64 = *var_qsov_dn15_slot;
        let mut var_qsov_dn16: f64 = *var_qsov_dn16_slot;
        let mut var_qsov_dn17: f64 = *var_qsov_dn17_slot;
        let mut var_qsov_dn18: f64 = *var_qsov_dn18_slot;
        let mut var_qsov_dn19: f64 = *var_qsov_dn19_slot;
        let mut var_qsov_dn2: f64 = *var_qsov_dn2_slot;
        let mut var_qsov_dn20: f64 = *var_qsov_dn20_slot;
        let mut var_qsov_dn21: f64 = *var_qsov_dn21_slot;
        let mut var_qsov_dn22: f64 = *var_qsov_dn22_slot;
        let mut var_qsov_dn3: f64 = *var_qsov_dn3_slot;
        let mut var_qsov_dn4: f64 = *var_qsov_dn4_slot;
        let mut var_qsov_dn5: f64 = *var_qsov_dn5_slot;
        let mut var_qsov_dn6: f64 = *var_qsov_dn6_slot;
        let mut var_qsov_dn7: f64 = *var_qsov_dn7_slot;
        let mut var_qsov_dn8: f64 = *var_qsov_dn8_slot;
        let mut var_qsov_dn9: f64 = *var_qsov_dn9_slot;
        let mut var_vdseffcv: f64 = *var_vdseffcv_slot;
        let mut var_vdseffcv_db0: f64 = *var_vdseffcv_db0_slot;
        let mut var_vdseffcv_db1: f64 = *var_vdseffcv_db1_slot;
        let mut var_vdseffcv_db10: f64 = *var_vdseffcv_db10_slot;
        let mut var_vdseffcv_db11: f64 = *var_vdseffcv_db11_slot;
        let mut var_vdseffcv_db12: f64 = *var_vdseffcv_db12_slot;
        let mut var_vdseffcv_db13: f64 = *var_vdseffcv_db13_slot;
        let mut var_vdseffcv_db14: f64 = *var_vdseffcv_db14_slot;
        let mut var_vdseffcv_db15: f64 = *var_vdseffcv_db15_slot;
        let mut var_vdseffcv_db16: f64 = *var_vdseffcv_db16_slot;
        let mut var_vdseffcv_db17: f64 = *var_vdseffcv_db17_slot;
        let mut var_vdseffcv_db18: f64 = *var_vdseffcv_db18_slot;
        let mut var_vdseffcv_db19: f64 = *var_vdseffcv_db19_slot;
        let mut var_vdseffcv_db2: f64 = *var_vdseffcv_db2_slot;
        let mut var_vdseffcv_db20: f64 = *var_vdseffcv_db20_slot;
        let mut var_vdseffcv_db21: f64 = *var_vdseffcv_db21_slot;
        let mut var_vdseffcv_db22: f64 = *var_vdseffcv_db22_slot;
        let mut var_vdseffcv_db23: f64 = *var_vdseffcv_db23_slot;
        let mut var_vdseffcv_db24: f64 = *var_vdseffcv_db24_slot;
        let mut var_vdseffcv_db25: f64 = *var_vdseffcv_db25_slot;
        let mut var_vdseffcv_db26: f64 = *var_vdseffcv_db26_slot;
        let mut var_vdseffcv_db27: f64 = *var_vdseffcv_db27_slot;
        let mut var_vdseffcv_db28: f64 = *var_vdseffcv_db28_slot;
        let mut var_vdseffcv_db29: f64 = *var_vdseffcv_db29_slot;
        let mut var_vdseffcv_db3: f64 = *var_vdseffcv_db3_slot;
        let mut var_vdseffcv_db30: f64 = *var_vdseffcv_db30_slot;
        let mut var_vdseffcv_db31: f64 = *var_vdseffcv_db31_slot;
        let mut var_vdseffcv_db32: f64 = *var_vdseffcv_db32_slot;
        let mut var_vdseffcv_db33: f64 = *var_vdseffcv_db33_slot;
        let mut var_vdseffcv_db34: f64 = *var_vdseffcv_db34_slot;
        let mut var_vdseffcv_db35: f64 = *var_vdseffcv_db35_slot;
        let mut var_vdseffcv_db36: f64 = *var_vdseffcv_db36_slot;
        let mut var_vdseffcv_db37: f64 = *var_vdseffcv_db37_slot;
        let mut var_vdseffcv_db38: f64 = *var_vdseffcv_db38_slot;
        let mut var_vdseffcv_db39: f64 = *var_vdseffcv_db39_slot;
        let mut var_vdseffcv_db4: f64 = *var_vdseffcv_db4_slot;
        let mut var_vdseffcv_db40: f64 = *var_vdseffcv_db40_slot;
        let mut var_vdseffcv_db41: f64 = *var_vdseffcv_db41_slot;
        let mut var_vdseffcv_db42: f64 = *var_vdseffcv_db42_slot;
        let mut var_vdseffcv_db43: f64 = *var_vdseffcv_db43_slot;
        let mut var_vdseffcv_db44: f64 = *var_vdseffcv_db44_slot;
        let mut var_vdseffcv_db45: f64 = *var_vdseffcv_db45_slot;
        let mut var_vdseffcv_db46: f64 = *var_vdseffcv_db46_slot;
        let mut var_vdseffcv_db47: f64 = *var_vdseffcv_db47_slot;
        let mut var_vdseffcv_db48: f64 = *var_vdseffcv_db48_slot;
        let mut var_vdseffcv_db49: f64 = *var_vdseffcv_db49_slot;
        let mut var_vdseffcv_db5: f64 = *var_vdseffcv_db5_slot;
        let mut var_vdseffcv_db50: f64 = *var_vdseffcv_db50_slot;
        let mut var_vdseffcv_db51: f64 = *var_vdseffcv_db51_slot;
        let mut var_vdseffcv_db52: f64 = *var_vdseffcv_db52_slot;
        let mut var_vdseffcv_db53: f64 = *var_vdseffcv_db53_slot;
        let mut var_vdseffcv_db54: f64 = *var_vdseffcv_db54_slot;
        let mut var_vdseffcv_db55: f64 = *var_vdseffcv_db55_slot;
        let mut var_vdseffcv_db56: f64 = *var_vdseffcv_db56_slot;
        let mut var_vdseffcv_db6: f64 = *var_vdseffcv_db6_slot;
        let mut var_vdseffcv_db7: f64 = *var_vdseffcv_db7_slot;
        let mut var_vdseffcv_db8: f64 = *var_vdseffcv_db8_slot;
        let mut var_vdseffcv_db9: f64 = *var_vdseffcv_db9_slot;
        let mut var_vdseffcv_dn0: f64 = *var_vdseffcv_dn0_slot;
        let mut var_vdseffcv_dn1: f64 = *var_vdseffcv_dn1_slot;
        let mut var_vdseffcv_dn10: f64 = *var_vdseffcv_dn10_slot;
        let mut var_vdseffcv_dn11: f64 = *var_vdseffcv_dn11_slot;
        let mut var_vdseffcv_dn12: f64 = *var_vdseffcv_dn12_slot;
        let mut var_vdseffcv_dn13: f64 = *var_vdseffcv_dn13_slot;
        let mut var_vdseffcv_dn14: f64 = *var_vdseffcv_dn14_slot;
        let mut var_vdseffcv_dn15: f64 = *var_vdseffcv_dn15_slot;
        let mut var_vdseffcv_dn16: f64 = *var_vdseffcv_dn16_slot;
        let mut var_vdseffcv_dn17: f64 = *var_vdseffcv_dn17_slot;
        let mut var_vdseffcv_dn18: f64 = *var_vdseffcv_dn18_slot;
        let mut var_vdseffcv_dn19: f64 = *var_vdseffcv_dn19_slot;
        let mut var_vdseffcv_dn2: f64 = *var_vdseffcv_dn2_slot;
        let mut var_vdseffcv_dn20: f64 = *var_vdseffcv_dn20_slot;
        let mut var_vdseffcv_dn21: f64 = *var_vdseffcv_dn21_slot;
        let mut var_vdseffcv_dn22: f64 = *var_vdseffcv_dn22_slot;
        let mut var_vdseffcv_dn3: f64 = *var_vdseffcv_dn3_slot;
        let mut var_vdseffcv_dn4: f64 = *var_vdseffcv_dn4_slot;
        let mut var_vdseffcv_dn5: f64 = *var_vdseffcv_dn5_slot;
        let mut var_vdseffcv_dn6: f64 = *var_vdseffcv_dn6_slot;
        let mut var_vdseffcv_dn7: f64 = *var_vdseffcv_dn7_slot;
        let mut var_vdseffcv_dn8: f64 = *var_vdseffcv_dn8_slot;
        let mut var_vdseffcv_dn9: f64 = *var_vdseffcv_dn9_slot;

        if (((!s.b[538]) && s.b[547]) && s.b[551]) {
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

        let assign30990_e48883: f64 = if p.p255 == 2.0 { 1.0 } else { 0.0 };
        var_guard524 = assign30990_e48883;

        let (assign31000_e48893, assign31000_e48893_d_n0, assign31000_e48893_d_n1, assign31000_e48893_d_n2, assign31000_e48893_d_n3, assign31000_e48893_d_n4, assign31000_e48893_d_n5, assign31000_e48893_d_n6, assign31000_e48893_d_n7, assign31000_e48893_d_n8, assign31000_e48893_d_n9, assign31000_e48893_d_n10, assign31000_e48893_d_n11, assign31000_e48893_d_n12, assign31000_e48893_d_n13, assign31000_e48893_d_n14, assign31000_e48893_d_n15, assign31000_e48893_d_n16, assign31000_e48893_d_n17, assign31000_e48893_d_n18, assign31000_e48893_d_n19, assign31000_e48893_d_n20, assign31000_e48893_d_n21, assign31000_e48893_d_n22, assign31000_e48893_d_b0, assign31000_e48893_d_b1, assign31000_e48893_d_b2, assign31000_e48893_d_b3, assign31000_e48893_d_b4, assign31000_e48893_d_b5, assign31000_e48893_d_b6, assign31000_e48893_d_b7, assign31000_e48893_d_b8, assign31000_e48893_d_b9, assign31000_e48893_d_b10, assign31000_e48893_d_b11, assign31000_e48893_d_b12, assign31000_e48893_d_b13, assign31000_e48893_d_b14, assign31000_e48893_d_b15, assign31000_e48893_d_b16, assign31000_e48893_d_b17, assign31000_e48893_d_b18, assign31000_e48893_d_b19, assign31000_e48893_d_b20, assign31000_e48893_d_b21, assign31000_e48893_d_b22, assign31000_e48893_d_b23, assign31000_e48893_d_b24, assign31000_e48893_d_b25, assign31000_e48893_d_b26, assign31000_e48893_d_b27, assign31000_e48893_d_b28, assign31000_e48893_d_b29, assign31000_e48893_d_b30, assign31000_e48893_d_b31, assign31000_e48893_d_b32, assign31000_e48893_d_b33, assign31000_e48893_d_b34, assign31000_e48893_d_b35, assign31000_e48893_d_b36, assign31000_e48893_d_b37, assign31000_e48893_d_b38, assign31000_e48893_d_b39, assign31000_e48893_d_b40, assign31000_e48893_d_b41, assign31000_e48893_d_b42, assign31000_e48893_d_b43, assign31000_e48893_d_b44, assign31000_e48893_d_b45, assign31000_e48893_d_b46, assign31000_e48893_d_b47, assign31000_e48893_d_b48, assign31000_e48893_d_b49, assign31000_e48893_d_b50, assign31000_e48893_d_b51, assign31000_e48893_d_b52, assign31000_e48893_d_b53, assign31000_e48893_d_b54,) = {
    if (var_guard524 != 0.0) {
        let assign31000_e48887: f64 = (p.p4 * p.p5);
        let assign31000_e48889: f64 = (assign31000_e48887 * p.p210);
        let assign31000_e48891: f64 = (assign31000_e48889 * (nv10 - nv2));
        (assign31000_e48891, 0.0, 0.0, (-assign31000_e48889), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, assign31000_e48889, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qsov, var_qsov_dn0, var_qsov_dn1, var_qsov_dn2, var_qsov_dn3, var_qsov_dn4, var_qsov_dn5, var_qsov_dn6, var_qsov_dn7, var_qsov_dn8, var_qsov_dn9, var_qsov_dn10, var_qsov_dn11, var_qsov_dn12, var_qsov_dn13, var_qsov_dn14, var_qsov_dn15, var_qsov_dn16, var_qsov_dn17, var_qsov_dn18, var_qsov_dn19, var_qsov_dn20, var_qsov_dn21, var_qsov_dn22, var_qsov_db0, var_qsov_db1, var_qsov_db2, var_qsov_db3, var_qsov_db4, var_qsov_db5, var_qsov_db6, var_qsov_db7, var_qsov_db8, var_qsov_db9, var_qsov_db10, var_qsov_db11, var_qsov_db12, var_qsov_db13, var_qsov_db14, var_qsov_db15, var_qsov_db16, var_qsov_db17, var_qsov_db18, var_qsov_db19, var_qsov_db20, var_qsov_db21, var_qsov_db22, var_qsov_db23, var_qsov_db24, var_qsov_db25, var_qsov_db26, var_qsov_db27, var_qsov_db28, var_qsov_db29, var_qsov_db30, var_qsov_db31, var_qsov_db32, var_qsov_db33, var_qsov_db34, var_qsov_db35, var_qsov_db36, var_qsov_db37, var_qsov_db38, var_qsov_db39, var_qsov_db40, var_qsov_db41, var_qsov_db42, var_qsov_db43, var_qsov_db44, var_qsov_db45, var_qsov_db46, var_qsov_db47, var_qsov_db48, var_qsov_db49, var_qsov_db50, var_qsov_db51, var_qsov_db52, var_qsov_db53, var_qsov_db54,)
    }
};
        var_qsov = assign31000_e48893;
        var_qsov_dn0 = assign31000_e48893_d_n0;
        var_qsov_dn1 = assign31000_e48893_d_n1;
        var_qsov_dn2 = assign31000_e48893_d_n2;
        var_qsov_dn3 = assign31000_e48893_d_n3;
        var_qsov_dn4 = assign31000_e48893_d_n4;
        var_qsov_dn5 = assign31000_e48893_d_n5;
        var_qsov_dn6 = assign31000_e48893_d_n6;
        var_qsov_dn7 = assign31000_e48893_d_n7;
        var_qsov_dn8 = assign31000_e48893_d_n8;
        var_qsov_dn9 = assign31000_e48893_d_n9;
        var_qsov_dn10 = assign31000_e48893_d_n10;
        var_qsov_dn11 = assign31000_e48893_d_n11;
        var_qsov_dn12 = assign31000_e48893_d_n12;
        var_qsov_dn13 = assign31000_e48893_d_n13;
        var_qsov_dn14 = assign31000_e48893_d_n14;
        var_qsov_dn15 = assign31000_e48893_d_n15;
        var_qsov_dn16 = assign31000_e48893_d_n16;
        var_qsov_dn17 = assign31000_e48893_d_n17;
        var_qsov_dn18 = assign31000_e48893_d_n18;
        var_qsov_dn19 = assign31000_e48893_d_n19;
        var_qsov_dn20 = assign31000_e48893_d_n20;
        var_qsov_dn21 = assign31000_e48893_d_n21;
        var_qsov_dn22 = assign31000_e48893_d_n22;
        var_qsov_db0 = assign31000_e48893_d_b0;
        var_qsov_db1 = assign31000_e48893_d_b1;
        var_qsov_db2 = assign31000_e48893_d_b2;
        var_qsov_db3 = assign31000_e48893_d_b3;
        var_qsov_db4 = assign31000_e48893_d_b4;
        var_qsov_db5 = assign31000_e48893_d_b5;
        var_qsov_db6 = assign31000_e48893_d_b6;
        var_qsov_db7 = assign31000_e48893_d_b7;
        var_qsov_db8 = assign31000_e48893_d_b8;
        var_qsov_db9 = assign31000_e48893_d_b9;
        var_qsov_db10 = assign31000_e48893_d_b10;
        var_qsov_db11 = assign31000_e48893_d_b11;
        var_qsov_db12 = assign31000_e48893_d_b12;
        var_qsov_db13 = assign31000_e48893_d_b13;
        var_qsov_db14 = assign31000_e48893_d_b14;
        var_qsov_db15 = assign31000_e48893_d_b15;
        var_qsov_db16 = assign31000_e48893_d_b16;
        var_qsov_db17 = assign31000_e48893_d_b17;
        var_qsov_db18 = assign31000_e48893_d_b18;
        var_qsov_db19 = assign31000_e48893_d_b19;
        var_qsov_db20 = assign31000_e48893_d_b20;
        var_qsov_db21 = assign31000_e48893_d_b21;
        var_qsov_db22 = assign31000_e48893_d_b22;
        var_qsov_db23 = assign31000_e48893_d_b23;
        var_qsov_db24 = assign31000_e48893_d_b24;
        var_qsov_db25 = assign31000_e48893_d_b25;
        var_qsov_db26 = assign31000_e48893_d_b26;
        var_qsov_db27 = assign31000_e48893_d_b27;
        var_qsov_db28 = assign31000_e48893_d_b28;
        var_qsov_db29 = assign31000_e48893_d_b29;
        var_qsov_db30 = assign31000_e48893_d_b30;
        var_qsov_db31 = assign31000_e48893_d_b31;
        var_qsov_db32 = assign31000_e48893_d_b32;
        var_qsov_db33 = assign31000_e48893_d_b33;
        var_qsov_db34 = assign31000_e48893_d_b34;
        var_qsov_db35 = assign31000_e48893_d_b35;
        var_qsov_db36 = assign31000_e48893_d_b36;
        var_qsov_db37 = assign31000_e48893_d_b37;
        var_qsov_db38 = assign31000_e48893_d_b38;
        var_qsov_db39 = assign31000_e48893_d_b39;
        var_qsov_db40 = assign31000_e48893_d_b40;
        var_qsov_db41 = assign31000_e48893_d_b41;
        var_qsov_db42 = assign31000_e48893_d_b42;
        var_qsov_db43 = assign31000_e48893_d_b43;
        var_qsov_db44 = assign31000_e48893_d_b44;
        var_qsov_db45 = assign31000_e48893_d_b45;
        var_qsov_db46 = assign31000_e48893_d_b46;
        var_qsov_db47 = assign31000_e48893_d_b47;
        var_qsov_db48 = assign31000_e48893_d_b48;
        var_qsov_db49 = assign31000_e48893_d_b49;
        var_qsov_db50 = assign31000_e48893_d_b50;
        var_qsov_db51 = assign31000_e48893_d_b51;
        var_qsov_db52 = assign31000_e48893_d_b52;
        var_qsov_db53 = assign31000_e48893_d_b53;
        var_qsov_db54 = assign31000_e48893_d_b54;
        var_qsov_db55 = 0.0;
        var_qsov_db56 = 0.0;

        let (assign31010_e48908, assign31010_e48908_d_n0, assign31010_e48908_d_n1, assign31010_e48908_d_n2, assign31010_e48908_d_n3, assign31010_e48908_d_n4, assign31010_e48908_d_n5, assign31010_e48908_d_n6, assign31010_e48908_d_n7, assign31010_e48908_d_n8, assign31010_e48908_d_n9, assign31010_e48908_d_n10, assign31010_e48908_d_n11, assign31010_e48908_d_n12, assign31010_e48908_d_n13, assign31010_e48908_d_n14, assign31010_e48908_d_n15, assign31010_e48908_d_n16, assign31010_e48908_d_n17, assign31010_e48908_d_n18, assign31010_e48908_d_n19, assign31010_e48908_d_n20, assign31010_e48908_d_n21, assign31010_e48908_d_n22, assign31010_e48908_d_b0, assign31010_e48908_d_b1, assign31010_e48908_d_b2, assign31010_e48908_d_b3, assign31010_e48908_d_b4, assign31010_e48908_d_b5, assign31010_e48908_d_b6, assign31010_e48908_d_b7, assign31010_e48908_d_b8, assign31010_e48908_d_b9, assign31010_e48908_d_b10, assign31010_e48908_d_b11, assign31010_e48908_d_b12, assign31010_e48908_d_b13, assign31010_e48908_d_b14, assign31010_e48908_d_b15, assign31010_e48908_d_b16, assign31010_e48908_d_b17, assign31010_e48908_d_b18, assign31010_e48908_d_b19, assign31010_e48908_d_b20, assign31010_e48908_d_b21, assign31010_e48908_d_b22, assign31010_e48908_d_b23, assign31010_e48908_d_b24, assign31010_e48908_d_b25, assign31010_e48908_d_b26, assign31010_e48908_d_b27, assign31010_e48908_d_b28, assign31010_e48908_d_b29, assign31010_e48908_d_b30, assign31010_e48908_d_b31, assign31010_e48908_d_b32, assign31010_e48908_d_b33, assign31010_e48908_d_b34, assign31010_e48908_d_b35, assign31010_e48908_d_b36, assign31010_e48908_d_b37, assign31010_e48908_d_b38, assign31010_e48908_d_b39, assign31010_e48908_d_b40, assign31010_e48908_d_b41, assign31010_e48908_d_b42, assign31010_e48908_d_b43, assign31010_e48908_d_b44, assign31010_e48908_d_b45, assign31010_e48908_d_b46, assign31010_e48908_d_b47, assign31010_e48908_d_b48, assign31010_e48908_d_b49, assign31010_e48908_d_b50, assign31010_e48908_d_b51, assign31010_e48908_d_b52, assign31010_e48908_d_b53, assign31010_e48908_d_b54,) = {
    if (var_guard524 != 0.0) {
        let assign31010_e48897: f64 = ((nv0 - nv2) * p.p214);
        let assign31010_e48900: f64 = ((nv0 - nv2) * (nv0 - nv2));
        let assign31010_e48903: f64 = (p.p214 * p.p214);
        let assign31010_e48904: f64 = (assign31010_e48900 + assign31010_e48903);
        let assign31010_e48905: f64 = (assign31010_e48904).sqrt();
        let assign31010_e48906: f64 = (assign31010_e48897 / assign31010_e48905);
        (assign31010_e48906, (((p.p214 * assign31010_e48905) - (assign31010_e48897 * (((nv0 - nv2) + (nv0 - nv2)) / (2.0 * assign31010_e48905)))) / (assign31010_e48905 * assign31010_e48905)), 0.0, ((((-p.p214) * assign31010_e48905) - (assign31010_e48897 * (((-(nv0 - nv2)) + (-(nv0 - nv2))) / (2.0 * assign31010_e48905)))) / (assign31010_e48905 * assign31010_e48905)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vdseffcv, var_vdseffcv_dn0, var_vdseffcv_dn1, var_vdseffcv_dn2, var_vdseffcv_dn3, var_vdseffcv_dn4, var_vdseffcv_dn5, var_vdseffcv_dn6, var_vdseffcv_dn7, var_vdseffcv_dn8, var_vdseffcv_dn9, var_vdseffcv_dn10, var_vdseffcv_dn11, var_vdseffcv_dn12, var_vdseffcv_dn13, var_vdseffcv_dn14, var_vdseffcv_dn15, var_vdseffcv_dn16, var_vdseffcv_dn17, var_vdseffcv_dn18, var_vdseffcv_dn19, var_vdseffcv_dn20, var_vdseffcv_dn21, var_vdseffcv_dn22, var_vdseffcv_db0, var_vdseffcv_db1, var_vdseffcv_db2, var_vdseffcv_db3, var_vdseffcv_db4, var_vdseffcv_db5, var_vdseffcv_db6, var_vdseffcv_db7, var_vdseffcv_db8, var_vdseffcv_db9, var_vdseffcv_db10, var_vdseffcv_db11, var_vdseffcv_db12, var_vdseffcv_db13, var_vdseffcv_db14, var_vdseffcv_db15, var_vdseffcv_db16, var_vdseffcv_db17, var_vdseffcv_db18, var_vdseffcv_db19, var_vdseffcv_db20, var_vdseffcv_db21, var_vdseffcv_db22, var_vdseffcv_db23, var_vdseffcv_db24, var_vdseffcv_db25, var_vdseffcv_db26, var_vdseffcv_db27, var_vdseffcv_db28, var_vdseffcv_db29, var_vdseffcv_db30, var_vdseffcv_db31, var_vdseffcv_db32, var_vdseffcv_db33, var_vdseffcv_db34, var_vdseffcv_db35, var_vdseffcv_db36, var_vdseffcv_db37, var_vdseffcv_db38, var_vdseffcv_db39, var_vdseffcv_db40, var_vdseffcv_db41, var_vdseffcv_db42, var_vdseffcv_db43, var_vdseffcv_db44, var_vdseffcv_db45, var_vdseffcv_db46, var_vdseffcv_db47, var_vdseffcv_db48, var_vdseffcv_db49, var_vdseffcv_db50, var_vdseffcv_db51, var_vdseffcv_db52, var_vdseffcv_db53, var_vdseffcv_db54,)
    }
};
        var_vdseffcv = assign31010_e48908;
        var_vdseffcv_dn0 = assign31010_e48908_d_n0;
        var_vdseffcv_dn1 = assign31010_e48908_d_n1;
        var_vdseffcv_dn2 = assign31010_e48908_d_n2;
        var_vdseffcv_dn3 = assign31010_e48908_d_n3;
        var_vdseffcv_dn4 = assign31010_e48908_d_n4;
        var_vdseffcv_dn5 = assign31010_e48908_d_n5;
        var_vdseffcv_dn6 = assign31010_e48908_d_n6;
        var_vdseffcv_dn7 = assign31010_e48908_d_n7;
        var_vdseffcv_dn8 = assign31010_e48908_d_n8;
        var_vdseffcv_dn9 = assign31010_e48908_d_n9;
        var_vdseffcv_dn10 = assign31010_e48908_d_n10;
        var_vdseffcv_dn11 = assign31010_e48908_d_n11;
        var_vdseffcv_dn12 = assign31010_e48908_d_n12;
        var_vdseffcv_dn13 = assign31010_e48908_d_n13;
        var_vdseffcv_dn14 = assign31010_e48908_d_n14;
        var_vdseffcv_dn15 = assign31010_e48908_d_n15;
        var_vdseffcv_dn16 = assign31010_e48908_d_n16;
        var_vdseffcv_dn17 = assign31010_e48908_d_n17;
        var_vdseffcv_dn18 = assign31010_e48908_d_n18;
        var_vdseffcv_dn19 = assign31010_e48908_d_n19;
        var_vdseffcv_dn20 = assign31010_e48908_d_n20;
        var_vdseffcv_dn21 = assign31010_e48908_d_n21;
        var_vdseffcv_dn22 = assign31010_e48908_d_n22;
        var_vdseffcv_db0 = assign31010_e48908_d_b0;
        var_vdseffcv_db1 = assign31010_e48908_d_b1;
        var_vdseffcv_db2 = assign31010_e48908_d_b2;
        var_vdseffcv_db3 = assign31010_e48908_d_b3;
        var_vdseffcv_db4 = assign31010_e48908_d_b4;
        var_vdseffcv_db5 = assign31010_e48908_d_b5;
        var_vdseffcv_db6 = assign31010_e48908_d_b6;
        var_vdseffcv_db7 = assign31010_e48908_d_b7;
        var_vdseffcv_db8 = assign31010_e48908_d_b8;
        var_vdseffcv_db9 = assign31010_e48908_d_b9;
        var_vdseffcv_db10 = assign31010_e48908_d_b10;
        var_vdseffcv_db11 = assign31010_e48908_d_b11;
        var_vdseffcv_db12 = assign31010_e48908_d_b12;
        var_vdseffcv_db13 = assign31010_e48908_d_b13;
        var_vdseffcv_db14 = assign31010_e48908_d_b14;
        var_vdseffcv_db15 = assign31010_e48908_d_b15;
        var_vdseffcv_db16 = assign31010_e48908_d_b16;
        var_vdseffcv_db17 = assign31010_e48908_d_b17;
        var_vdseffcv_db18 = assign31010_e48908_d_b18;
        var_vdseffcv_db19 = assign31010_e48908_d_b19;
        var_vdseffcv_db20 = assign31010_e48908_d_b20;
        var_vdseffcv_db21 = assign31010_e48908_d_b21;
        var_vdseffcv_db22 = assign31010_e48908_d_b22;
        var_vdseffcv_db23 = assign31010_e48908_d_b23;
        var_vdseffcv_db24 = assign31010_e48908_d_b24;
        var_vdseffcv_db25 = assign31010_e48908_d_b25;
        var_vdseffcv_db26 = assign31010_e48908_d_b26;
        var_vdseffcv_db27 = assign31010_e48908_d_b27;
        var_vdseffcv_db28 = assign31010_e48908_d_b28;
        var_vdseffcv_db29 = assign31010_e48908_d_b29;
        var_vdseffcv_db30 = assign31010_e48908_d_b30;
        var_vdseffcv_db31 = assign31010_e48908_d_b31;
        var_vdseffcv_db32 = assign31010_e48908_d_b32;
        var_vdseffcv_db33 = assign31010_e48908_d_b33;
        var_vdseffcv_db34 = assign31010_e48908_d_b34;
        var_vdseffcv_db35 = assign31010_e48908_d_b35;
        var_vdseffcv_db36 = assign31010_e48908_d_b36;
        var_vdseffcv_db37 = assign31010_e48908_d_b37;
        var_vdseffcv_db38 = assign31010_e48908_d_b38;
        var_vdseffcv_db39 = assign31010_e48908_d_b39;
        var_vdseffcv_db40 = assign31010_e48908_d_b40;
        var_vdseffcv_db41 = assign31010_e48908_d_b41;
        var_vdseffcv_db42 = assign31010_e48908_d_b42;
        var_vdseffcv_db43 = assign31010_e48908_d_b43;
        var_vdseffcv_db44 = assign31010_e48908_d_b44;
        var_vdseffcv_db45 = assign31010_e48908_d_b45;
        var_vdseffcv_db46 = assign31010_e48908_d_b46;
        var_vdseffcv_db47 = assign31010_e48908_d_b47;
        var_vdseffcv_db48 = assign31010_e48908_d_b48;
        var_vdseffcv_db49 = assign31010_e48908_d_b49;
        var_vdseffcv_db50 = assign31010_e48908_d_b50;
        var_vdseffcv_db51 = assign31010_e48908_d_b51;
        var_vdseffcv_db52 = assign31010_e48908_d_b52;
        var_vdseffcv_db53 = assign31010_e48908_d_b53;
        var_vdseffcv_db54 = assign31010_e48908_d_b54;
        var_vdseffcv_db55 = 0.0;
        var_vdseffcv_db56 = 0.0;

        let (assign31020_e48918,) = {
    if (var_guard524 != 0.0) {
        let assign31020_e48914: f64 = (2.0 * p.p214);
        let assign31020_e48915: f64 = (p.p211 / assign31020_e48914);
        let assign31020_e48916: f64 = (p.p213).min(assign31020_e48915);
        (assign31020_e48916,)
    } else {
        (var_cgdl_l,)
    }
};
        var_cgdl_l = assign31020_e48918;

        *var_cgdl_l_slot = var_cgdl_l;
        *var_guard524_slot = var_guard524;
        *var_qsov_slot = var_qsov;
        *var_qsov_db0_slot = var_qsov_db0;
        *var_qsov_db1_slot = var_qsov_db1;
        *var_qsov_db10_slot = var_qsov_db10;
        *var_qsov_db11_slot = var_qsov_db11;
        *var_qsov_db12_slot = var_qsov_db12;
        *var_qsov_db13_slot = var_qsov_db13;
        *var_qsov_db14_slot = var_qsov_db14;
        *var_qsov_db15_slot = var_qsov_db15;
        *var_qsov_db16_slot = var_qsov_db16;
        *var_qsov_db17_slot = var_qsov_db17;
        *var_qsov_db18_slot = var_qsov_db18;
        *var_qsov_db19_slot = var_qsov_db19;
        *var_qsov_db2_slot = var_qsov_db2;
        *var_qsov_db20_slot = var_qsov_db20;
        *var_qsov_db21_slot = var_qsov_db21;
        *var_qsov_db22_slot = var_qsov_db22;
        *var_qsov_db23_slot = var_qsov_db23;
        *var_qsov_db24_slot = var_qsov_db24;
        *var_qsov_db25_slot = var_qsov_db25;
        *var_qsov_db26_slot = var_qsov_db26;
        *var_qsov_db27_slot = var_qsov_db27;
        *var_qsov_db28_slot = var_qsov_db28;
        *var_qsov_db29_slot = var_qsov_db29;
        *var_qsov_db3_slot = var_qsov_db3;
        *var_qsov_db30_slot = var_qsov_db30;
        *var_qsov_db31_slot = var_qsov_db31;
        *var_qsov_db32_slot = var_qsov_db32;
        *var_qsov_db33_slot = var_qsov_db33;
        *var_qsov_db34_slot = var_qsov_db34;
        *var_qsov_db35_slot = var_qsov_db35;
        *var_qsov_db36_slot = var_qsov_db36;
        *var_qsov_db37_slot = var_qsov_db37;
        *var_qsov_db38_slot = var_qsov_db38;
        *var_qsov_db39_slot = var_qsov_db39;
        *var_qsov_db4_slot = var_qsov_db4;
        *var_qsov_db40_slot = var_qsov_db40;
        *var_qsov_db41_slot = var_qsov_db41;
        *var_qsov_db42_slot = var_qsov_db42;
        *var_qsov_db43_slot = var_qsov_db43;
        *var_qsov_db44_slot = var_qsov_db44;
        *var_qsov_db45_slot = var_qsov_db45;
        *var_qsov_db46_slot = var_qsov_db46;
        *var_qsov_db47_slot = var_qsov_db47;
        *var_qsov_db48_slot = var_qsov_db48;
        *var_qsov_db49_slot = var_qsov_db49;
        *var_qsov_db5_slot = var_qsov_db5;
        *var_qsov_db50_slot = var_qsov_db50;
        *var_qsov_db51_slot = var_qsov_db51;
        *var_qsov_db52_slot = var_qsov_db52;
        *var_qsov_db53_slot = var_qsov_db53;
        *var_qsov_db54_slot = var_qsov_db54;
        *var_qsov_db55_slot = var_qsov_db55;
        *var_qsov_db56_slot = var_qsov_db56;
        *var_qsov_db6_slot = var_qsov_db6;
        *var_qsov_db7_slot = var_qsov_db7;
        *var_qsov_db8_slot = var_qsov_db8;
        *var_qsov_db9_slot = var_qsov_db9;
        *var_qsov_dn0_slot = var_qsov_dn0;
        *var_qsov_dn1_slot = var_qsov_dn1;
        *var_qsov_dn10_slot = var_qsov_dn10;
        *var_qsov_dn11_slot = var_qsov_dn11;
        *var_qsov_dn12_slot = var_qsov_dn12;
        *var_qsov_dn13_slot = var_qsov_dn13;
        *var_qsov_dn14_slot = var_qsov_dn14;
        *var_qsov_dn15_slot = var_qsov_dn15;
        *var_qsov_dn16_slot = var_qsov_dn16;
        *var_qsov_dn17_slot = var_qsov_dn17;
        *var_qsov_dn18_slot = var_qsov_dn18;
        *var_qsov_dn19_slot = var_qsov_dn19;
        *var_qsov_dn2_slot = var_qsov_dn2;
        *var_qsov_dn20_slot = var_qsov_dn20;
        *var_qsov_dn21_slot = var_qsov_dn21;
        *var_qsov_dn22_slot = var_qsov_dn22;
        *var_qsov_dn3_slot = var_qsov_dn3;
        *var_qsov_dn4_slot = var_qsov_dn4;
        *var_qsov_dn5_slot = var_qsov_dn5;
        *var_qsov_dn6_slot = var_qsov_dn6;
        *var_qsov_dn7_slot = var_qsov_dn7;
        *var_qsov_dn8_slot = var_qsov_dn8;
        *var_qsov_dn9_slot = var_qsov_dn9;
        *var_vdseffcv_slot = var_vdseffcv;
        *var_vdseffcv_db0_slot = var_vdseffcv_db0;
        *var_vdseffcv_db1_slot = var_vdseffcv_db1;
        *var_vdseffcv_db10_slot = var_vdseffcv_db10;
        *var_vdseffcv_db11_slot = var_vdseffcv_db11;
        *var_vdseffcv_db12_slot = var_vdseffcv_db12;
        *var_vdseffcv_db13_slot = var_vdseffcv_db13;
        *var_vdseffcv_db14_slot = var_vdseffcv_db14;
        *var_vdseffcv_db15_slot = var_vdseffcv_db15;
        *var_vdseffcv_db16_slot = var_vdseffcv_db16;
        *var_vdseffcv_db17_slot = var_vdseffcv_db17;
        *var_vdseffcv_db18_slot = var_vdseffcv_db18;
        *var_vdseffcv_db19_slot = var_vdseffcv_db19;
        *var_vdseffcv_db2_slot = var_vdseffcv_db2;
        *var_vdseffcv_db20_slot = var_vdseffcv_db20;
        *var_vdseffcv_db21_slot = var_vdseffcv_db21;
        *var_vdseffcv_db22_slot = var_vdseffcv_db22;
        *var_vdseffcv_db23_slot = var_vdseffcv_db23;
        *var_vdseffcv_db24_slot = var_vdseffcv_db24;
        *var_vdseffcv_db25_slot = var_vdseffcv_db25;
        *var_vdseffcv_db26_slot = var_vdseffcv_db26;
        *var_vdseffcv_db27_slot = var_vdseffcv_db27;
        *var_vdseffcv_db28_slot = var_vdseffcv_db28;
        *var_vdseffcv_db29_slot = var_vdseffcv_db29;
        *var_vdseffcv_db3_slot = var_vdseffcv_db3;
        *var_vdseffcv_db30_slot = var_vdseffcv_db30;
        *var_vdseffcv_db31_slot = var_vdseffcv_db31;
        *var_vdseffcv_db32_slot = var_vdseffcv_db32;
        *var_vdseffcv_db33_slot = var_vdseffcv_db33;
        *var_vdseffcv_db34_slot = var_vdseffcv_db34;
        *var_vdseffcv_db35_slot = var_vdseffcv_db35;
        *var_vdseffcv_db36_slot = var_vdseffcv_db36;
        *var_vdseffcv_db37_slot = var_vdseffcv_db37;
        *var_vdseffcv_db38_slot = var_vdseffcv_db38;
        *var_vdseffcv_db39_slot = var_vdseffcv_db39;
        *var_vdseffcv_db4_slot = var_vdseffcv_db4;
        *var_vdseffcv_db40_slot = var_vdseffcv_db40;
        *var_vdseffcv_db41_slot = var_vdseffcv_db41;
        *var_vdseffcv_db42_slot = var_vdseffcv_db42;
        *var_vdseffcv_db43_slot = var_vdseffcv_db43;
        *var_vdseffcv_db44_slot = var_vdseffcv_db44;
        *var_vdseffcv_db45_slot = var_vdseffcv_db45;
        *var_vdseffcv_db46_slot = var_vdseffcv_db46;
        *var_vdseffcv_db47_slot = var_vdseffcv_db47;
        *var_vdseffcv_db48_slot = var_vdseffcv_db48;
        *var_vdseffcv_db49_slot = var_vdseffcv_db49;
        *var_vdseffcv_db5_slot = var_vdseffcv_db5;
        *var_vdseffcv_db50_slot = var_vdseffcv_db50;
        *var_vdseffcv_db51_slot = var_vdseffcv_db51;
        *var_vdseffcv_db52_slot = var_vdseffcv_db52;
        *var_vdseffcv_db53_slot = var_vdseffcv_db53;
        *var_vdseffcv_db54_slot = var_vdseffcv_db54;
        *var_vdseffcv_db55_slot = var_vdseffcv_db55;
        *var_vdseffcv_db56_slot = var_vdseffcv_db56;
        *var_vdseffcv_db6_slot = var_vdseffcv_db6;
        *var_vdseffcv_db7_slot = var_vdseffcv_db7;
        *var_vdseffcv_db8_slot = var_vdseffcv_db8;
        *var_vdseffcv_db9_slot = var_vdseffcv_db9;
        *var_vdseffcv_dn0_slot = var_vdseffcv_dn0;
        *var_vdseffcv_dn1_slot = var_vdseffcv_dn1;
        *var_vdseffcv_dn10_slot = var_vdseffcv_dn10;
        *var_vdseffcv_dn11_slot = var_vdseffcv_dn11;
        *var_vdseffcv_dn12_slot = var_vdseffcv_dn12;
        *var_vdseffcv_dn13_slot = var_vdseffcv_dn13;
        *var_vdseffcv_dn14_slot = var_vdseffcv_dn14;
        *var_vdseffcv_dn15_slot = var_vdseffcv_dn15;
        *var_vdseffcv_dn16_slot = var_vdseffcv_dn16;
        *var_vdseffcv_dn17_slot = var_vdseffcv_dn17;
        *var_vdseffcv_dn18_slot = var_vdseffcv_dn18;
        *var_vdseffcv_dn19_slot = var_vdseffcv_dn19;
        *var_vdseffcv_dn2_slot = var_vdseffcv_dn2;
        *var_vdseffcv_dn20_slot = var_vdseffcv_dn20;
        *var_vdseffcv_dn21_slot = var_vdseffcv_dn21;
        *var_vdseffcv_dn22_slot = var_vdseffcv_dn22;
        *var_vdseffcv_dn3_slot = var_vdseffcv_dn3;
        *var_vdseffcv_dn4_slot = var_vdseffcv_dn4;
        *var_vdseffcv_dn5_slot = var_vdseffcv_dn5;
        *var_vdseffcv_dn6_slot = var_vdseffcv_dn6;
        *var_vdseffcv_dn7_slot = var_vdseffcv_dn7;
        *var_vdseffcv_dn8_slot = var_vdseffcv_dn8;
        *var_vdseffcv_dn9_slot = var_vdseffcv_dn9;
    }

    pub(super) fn stamp_transient_block_30(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_guard524: f64,
        var_cgdl_l_slot: &mut f64,
        var_cgdvar_slot: &mut f64,
        var_cgdvar_db0_slot: &mut f64,
        var_cgdvar_db1_slot: &mut f64,
        var_cgdvar_db10_slot: &mut f64,
        var_cgdvar_db11_slot: &mut f64,
        var_cgdvar_db12_slot: &mut f64,
        var_cgdvar_db13_slot: &mut f64,
        var_cgdvar_db14_slot: &mut f64,
        var_cgdvar_db15_slot: &mut f64,
        var_cgdvar_db16_slot: &mut f64,
        var_cgdvar_db17_slot: &mut f64,
        var_cgdvar_db18_slot: &mut f64,
        var_cgdvar_db19_slot: &mut f64,
        var_cgdvar_db2_slot: &mut f64,
        var_cgdvar_db20_slot: &mut f64,
        var_cgdvar_db21_slot: &mut f64,
        var_cgdvar_db22_slot: &mut f64,
        var_cgdvar_db23_slot: &mut f64,
        var_cgdvar_db24_slot: &mut f64,
        var_cgdvar_db25_slot: &mut f64,
        var_cgdvar_db26_slot: &mut f64,
        var_cgdvar_db27_slot: &mut f64,
        var_cgdvar_db28_slot: &mut f64,
        var_cgdvar_db29_slot: &mut f64,
        var_cgdvar_db3_slot: &mut f64,
        var_cgdvar_db30_slot: &mut f64,
        var_cgdvar_db31_slot: &mut f64,
        var_cgdvar_db32_slot: &mut f64,
        var_cgdvar_db33_slot: &mut f64,
        var_cgdvar_db34_slot: &mut f64,
        var_cgdvar_db35_slot: &mut f64,
        var_cgdvar_db36_slot: &mut f64,
        var_cgdvar_db37_slot: &mut f64,
        var_cgdvar_db38_slot: &mut f64,
        var_cgdvar_db39_slot: &mut f64,
        var_cgdvar_db4_slot: &mut f64,
        var_cgdvar_db40_slot: &mut f64,
        var_cgdvar_db41_slot: &mut f64,
        var_cgdvar_db42_slot: &mut f64,
        var_cgdvar_db43_slot: &mut f64,
        var_cgdvar_db44_slot: &mut f64,
        var_cgdvar_db45_slot: &mut f64,
        var_cgdvar_db46_slot: &mut f64,
        var_cgdvar_db47_slot: &mut f64,
        var_cgdvar_db48_slot: &mut f64,
        var_cgdvar_db49_slot: &mut f64,
        var_cgdvar_db5_slot: &mut f64,
        var_cgdvar_db50_slot: &mut f64,
        var_cgdvar_db51_slot: &mut f64,
        var_cgdvar_db52_slot: &mut f64,
        var_cgdvar_db53_slot: &mut f64,
        var_cgdvar_db54_slot: &mut f64,
        var_cgdvar_db55_slot: &mut f64,
        var_cgdvar_db56_slot: &mut f64,
        var_cgdvar_db6_slot: &mut f64,
        var_cgdvar_db7_slot: &mut f64,
        var_cgdvar_db8_slot: &mut f64,
        var_cgdvar_db9_slot: &mut f64,
        var_cgdvar_dn0_slot: &mut f64,
        var_cgdvar_dn1_slot: &mut f64,
        var_cgdvar_dn10_slot: &mut f64,
        var_cgdvar_dn11_slot: &mut f64,
        var_cgdvar_dn12_slot: &mut f64,
        var_cgdvar_dn13_slot: &mut f64,
        var_cgdvar_dn14_slot: &mut f64,
        var_cgdvar_dn15_slot: &mut f64,
        var_cgdvar_dn16_slot: &mut f64,
        var_cgdvar_dn17_slot: &mut f64,
        var_cgdvar_dn18_slot: &mut f64,
        var_cgdvar_dn19_slot: &mut f64,
        var_cgdvar_dn2_slot: &mut f64,
        var_cgdvar_dn20_slot: &mut f64,
        var_cgdvar_dn21_slot: &mut f64,
        var_cgdvar_dn22_slot: &mut f64,
        var_cgdvar_dn3_slot: &mut f64,
        var_cgdvar_dn4_slot: &mut f64,
        var_cgdvar_dn5_slot: &mut f64,
        var_cgdvar_dn6_slot: &mut f64,
        var_cgdvar_dn7_slot: &mut f64,
        var_cgdvar_dn8_slot: &mut f64,
        var_cgdvar_dn9_slot: &mut f64,
        var_qdov_slot: &mut f64,
        var_qdov_db0_slot: &mut f64,
        var_qdov_db1_slot: &mut f64,
        var_qdov_db10_slot: &mut f64,
        var_qdov_db11_slot: &mut f64,
        var_qdov_db12_slot: &mut f64,
        var_qdov_db13_slot: &mut f64,
        var_qdov_db14_slot: &mut f64,
        var_qdov_db15_slot: &mut f64,
        var_qdov_db16_slot: &mut f64,
        var_qdov_db17_slot: &mut f64,
        var_qdov_db18_slot: &mut f64,
        var_qdov_db19_slot: &mut f64,
        var_qdov_db2_slot: &mut f64,
        var_qdov_db20_slot: &mut f64,
        var_qdov_db21_slot: &mut f64,
        var_qdov_db22_slot: &mut f64,
        var_qdov_db23_slot: &mut f64,
        var_qdov_db24_slot: &mut f64,
        var_qdov_db25_slot: &mut f64,
        var_qdov_db26_slot: &mut f64,
        var_qdov_db27_slot: &mut f64,
        var_qdov_db28_slot: &mut f64,
        var_qdov_db29_slot: &mut f64,
        var_qdov_db3_slot: &mut f64,
        var_qdov_db30_slot: &mut f64,
        var_qdov_db31_slot: &mut f64,
        var_qdov_db32_slot: &mut f64,
        var_qdov_db33_slot: &mut f64,
        var_qdov_db34_slot: &mut f64,
        var_qdov_db35_slot: &mut f64,
        var_qdov_db36_slot: &mut f64,
        var_qdov_db37_slot: &mut f64,
        var_qdov_db38_slot: &mut f64,
        var_qdov_db39_slot: &mut f64,
        var_qdov_db4_slot: &mut f64,
        var_qdov_db40_slot: &mut f64,
        var_qdov_db41_slot: &mut f64,
        var_qdov_db42_slot: &mut f64,
        var_qdov_db43_slot: &mut f64,
        var_qdov_db44_slot: &mut f64,
        var_qdov_db45_slot: &mut f64,
        var_qdov_db46_slot: &mut f64,
        var_qdov_db47_slot: &mut f64,
        var_qdov_db48_slot: &mut f64,
        var_qdov_db49_slot: &mut f64,
        var_qdov_db5_slot: &mut f64,
        var_qdov_db50_slot: &mut f64,
        var_qdov_db51_slot: &mut f64,
        var_qdov_db52_slot: &mut f64,
        var_qdov_db53_slot: &mut f64,
        var_qdov_db54_slot: &mut f64,
        var_qdov_db55_slot: &mut f64,
        var_qdov_db56_slot: &mut f64,
        var_qdov_db6_slot: &mut f64,
        var_qdov_db7_slot: &mut f64,
        var_qdov_db8_slot: &mut f64,
        var_qdov_db9_slot: &mut f64,
        var_qdov_dn0_slot: &mut f64,
        var_qdov_dn1_slot: &mut f64,
        var_qdov_dn10_slot: &mut f64,
        var_qdov_dn11_slot: &mut f64,
        var_qdov_dn12_slot: &mut f64,
        var_qdov_dn13_slot: &mut f64,
        var_qdov_dn14_slot: &mut f64,
        var_qdov_dn15_slot: &mut f64,
        var_qdov_dn16_slot: &mut f64,
        var_qdov_dn17_slot: &mut f64,
        var_qdov_dn18_slot: &mut f64,
        var_qdov_dn19_slot: &mut f64,
        var_qdov_dn2_slot: &mut f64,
        var_qdov_dn20_slot: &mut f64,
        var_qdov_dn21_slot: &mut f64,
        var_qdov_dn22_slot: &mut f64,
        var_qdov_dn3_slot: &mut f64,
        var_qdov_dn4_slot: &mut f64,
        var_qdov_dn5_slot: &mut f64,
        var_qdov_dn6_slot: &mut f64,
        var_qdov_dn7_slot: &mut f64,
        var_qdov_dn8_slot: &mut f64,
        var_qdov_dn9_slot: &mut f64,
        var_qsov_slot: &mut f64,
        var_qsov_db0_slot: &mut f64,
        var_qsov_db1_slot: &mut f64,
        var_qsov_db10_slot: &mut f64,
        var_qsov_db11_slot: &mut f64,
        var_qsov_db12_slot: &mut f64,
        var_qsov_db13_slot: &mut f64,
        var_qsov_db14_slot: &mut f64,
        var_qsov_db15_slot: &mut f64,
        var_qsov_db16_slot: &mut f64,
        var_qsov_db17_slot: &mut f64,
        var_qsov_db18_slot: &mut f64,
        var_qsov_db19_slot: &mut f64,
        var_qsov_db2_slot: &mut f64,
        var_qsov_db20_slot: &mut f64,
        var_qsov_db21_slot: &mut f64,
        var_qsov_db22_slot: &mut f64,
        var_qsov_db23_slot: &mut f64,
        var_qsov_db24_slot: &mut f64,
        var_qsov_db25_slot: &mut f64,
        var_qsov_db26_slot: &mut f64,
        var_qsov_db27_slot: &mut f64,
        var_qsov_db28_slot: &mut f64,
        var_qsov_db29_slot: &mut f64,
        var_qsov_db3_slot: &mut f64,
        var_qsov_db30_slot: &mut f64,
        var_qsov_db31_slot: &mut f64,
        var_qsov_db32_slot: &mut f64,
        var_qsov_db33_slot: &mut f64,
        var_qsov_db34_slot: &mut f64,
        var_qsov_db35_slot: &mut f64,
        var_qsov_db36_slot: &mut f64,
        var_qsov_db37_slot: &mut f64,
        var_qsov_db38_slot: &mut f64,
        var_qsov_db39_slot: &mut f64,
        var_qsov_db4_slot: &mut f64,
        var_qsov_db40_slot: &mut f64,
        var_qsov_db41_slot: &mut f64,
        var_qsov_db42_slot: &mut f64,
        var_qsov_db43_slot: &mut f64,
        var_qsov_db44_slot: &mut f64,
        var_qsov_db45_slot: &mut f64,
        var_qsov_db46_slot: &mut f64,
        var_qsov_db47_slot: &mut f64,
        var_qsov_db48_slot: &mut f64,
        var_qsov_db49_slot: &mut f64,
        var_qsov_db5_slot: &mut f64,
        var_qsov_db50_slot: &mut f64,
        var_qsov_db51_slot: &mut f64,
        var_qsov_db52_slot: &mut f64,
        var_qsov_db53_slot: &mut f64,
        var_qsov_db54_slot: &mut f64,
        var_qsov_db55_slot: &mut f64,
        var_qsov_db56_slot: &mut f64,
        var_qsov_db6_slot: &mut f64,
        var_qsov_db7_slot: &mut f64,
        var_qsov_db8_slot: &mut f64,
        var_qsov_db9_slot: &mut f64,
        var_qsov_dn0_slot: &mut f64,
        var_qsov_dn1_slot: &mut f64,
        var_qsov_dn10_slot: &mut f64,
        var_qsov_dn11_slot: &mut f64,
        var_qsov_dn12_slot: &mut f64,
        var_qsov_dn13_slot: &mut f64,
        var_qsov_dn14_slot: &mut f64,
        var_qsov_dn15_slot: &mut f64,
        var_qsov_dn16_slot: &mut f64,
        var_qsov_dn17_slot: &mut f64,
        var_qsov_dn18_slot: &mut f64,
        var_qsov_dn19_slot: &mut f64,
        var_qsov_dn2_slot: &mut f64,
        var_qsov_dn20_slot: &mut f64,
        var_qsov_dn21_slot: &mut f64,
        var_qsov_dn22_slot: &mut f64,
        var_qsov_dn3_slot: &mut f64,
        var_qsov_dn4_slot: &mut f64,
        var_qsov_dn5_slot: &mut f64,
        var_qsov_dn6_slot: &mut f64,
        var_qsov_dn7_slot: &mut f64,
        var_qsov_dn8_slot: &mut f64,
        var_qsov_dn9_slot: &mut f64,
        var_vdseffcv_slot: &mut f64,
        var_vdseffcv_db0_slot: &mut f64,
        var_vdseffcv_db1_slot: &mut f64,
        var_vdseffcv_db10_slot: &mut f64,
        var_vdseffcv_db11_slot: &mut f64,
        var_vdseffcv_db12_slot: &mut f64,
        var_vdseffcv_db13_slot: &mut f64,
        var_vdseffcv_db14_slot: &mut f64,
        var_vdseffcv_db15_slot: &mut f64,
        var_vdseffcv_db16_slot: &mut f64,
        var_vdseffcv_db17_slot: &mut f64,
        var_vdseffcv_db18_slot: &mut f64,
        var_vdseffcv_db19_slot: &mut f64,
        var_vdseffcv_db2_slot: &mut f64,
        var_vdseffcv_db20_slot: &mut f64,
        var_vdseffcv_db21_slot: &mut f64,
        var_vdseffcv_db22_slot: &mut f64,
        var_vdseffcv_db23_slot: &mut f64,
        var_vdseffcv_db24_slot: &mut f64,
        var_vdseffcv_db25_slot: &mut f64,
        var_vdseffcv_db26_slot: &mut f64,
        var_vdseffcv_db27_slot: &mut f64,
        var_vdseffcv_db28_slot: &mut f64,
        var_vdseffcv_db29_slot: &mut f64,
        var_vdseffcv_db3_slot: &mut f64,
        var_vdseffcv_db30_slot: &mut f64,
        var_vdseffcv_db31_slot: &mut f64,
        var_vdseffcv_db32_slot: &mut f64,
        var_vdseffcv_db33_slot: &mut f64,
        var_vdseffcv_db34_slot: &mut f64,
        var_vdseffcv_db35_slot: &mut f64,
        var_vdseffcv_db36_slot: &mut f64,
        var_vdseffcv_db37_slot: &mut f64,
        var_vdseffcv_db38_slot: &mut f64,
        var_vdseffcv_db39_slot: &mut f64,
        var_vdseffcv_db4_slot: &mut f64,
        var_vdseffcv_db40_slot: &mut f64,
        var_vdseffcv_db41_slot: &mut f64,
        var_vdseffcv_db42_slot: &mut f64,
        var_vdseffcv_db43_slot: &mut f64,
        var_vdseffcv_db44_slot: &mut f64,
        var_vdseffcv_db45_slot: &mut f64,
        var_vdseffcv_db46_slot: &mut f64,
        var_vdseffcv_db47_slot: &mut f64,
        var_vdseffcv_db48_slot: &mut f64,
        var_vdseffcv_db49_slot: &mut f64,
        var_vdseffcv_db5_slot: &mut f64,
        var_vdseffcv_db50_slot: &mut f64,
        var_vdseffcv_db51_slot: &mut f64,
        var_vdseffcv_db52_slot: &mut f64,
        var_vdseffcv_db53_slot: &mut f64,
        var_vdseffcv_db54_slot: &mut f64,
        var_vdseffcv_db55_slot: &mut f64,
        var_vdseffcv_db56_slot: &mut f64,
        var_vdseffcv_db6_slot: &mut f64,
        var_vdseffcv_db7_slot: &mut f64,
        var_vdseffcv_db8_slot: &mut f64,
        var_vdseffcv_db9_slot: &mut f64,
        var_vdseffcv_dn0_slot: &mut f64,
        var_vdseffcv_dn1_slot: &mut f64,
        var_vdseffcv_dn10_slot: &mut f64,
        var_vdseffcv_dn11_slot: &mut f64,
        var_vdseffcv_dn12_slot: &mut f64,
        var_vdseffcv_dn13_slot: &mut f64,
        var_vdseffcv_dn14_slot: &mut f64,
        var_vdseffcv_dn15_slot: &mut f64,
        var_vdseffcv_dn16_slot: &mut f64,
        var_vdseffcv_dn17_slot: &mut f64,
        var_vdseffcv_dn18_slot: &mut f64,
        var_vdseffcv_dn19_slot: &mut f64,
        var_vdseffcv_dn2_slot: &mut f64,
        var_vdseffcv_dn20_slot: &mut f64,
        var_vdseffcv_dn21_slot: &mut f64,
        var_vdseffcv_dn22_slot: &mut f64,
        var_vdseffcv_dn3_slot: &mut f64,
        var_vdseffcv_dn4_slot: &mut f64,
        var_vdseffcv_dn5_slot: &mut f64,
        var_vdseffcv_dn6_slot: &mut f64,
        var_vdseffcv_dn7_slot: &mut f64,
        var_vdseffcv_dn8_slot: &mut f64,
        var_vdseffcv_dn9_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let mut var_cgdl_l: f64 = *var_cgdl_l_slot;
        let mut var_cgdvar: f64 = *var_cgdvar_slot;
        let mut var_cgdvar_db0: f64 = *var_cgdvar_db0_slot;
        let mut var_cgdvar_db1: f64 = *var_cgdvar_db1_slot;
        let mut var_cgdvar_db10: f64 = *var_cgdvar_db10_slot;
        let mut var_cgdvar_db11: f64 = *var_cgdvar_db11_slot;
        let mut var_cgdvar_db12: f64 = *var_cgdvar_db12_slot;
        let mut var_cgdvar_db13: f64 = *var_cgdvar_db13_slot;
        let mut var_cgdvar_db14: f64 = *var_cgdvar_db14_slot;
        let mut var_cgdvar_db15: f64 = *var_cgdvar_db15_slot;
        let mut var_cgdvar_db16: f64 = *var_cgdvar_db16_slot;
        let mut var_cgdvar_db17: f64 = *var_cgdvar_db17_slot;
        let mut var_cgdvar_db18: f64 = *var_cgdvar_db18_slot;
        let mut var_cgdvar_db19: f64 = *var_cgdvar_db19_slot;
        let mut var_cgdvar_db2: f64 = *var_cgdvar_db2_slot;
        let mut var_cgdvar_db20: f64 = *var_cgdvar_db20_slot;
        let mut var_cgdvar_db21: f64 = *var_cgdvar_db21_slot;
        let mut var_cgdvar_db22: f64 = *var_cgdvar_db22_slot;
        let mut var_cgdvar_db23: f64 = *var_cgdvar_db23_slot;
        let mut var_cgdvar_db24: f64 = *var_cgdvar_db24_slot;
        let mut var_cgdvar_db25: f64 = *var_cgdvar_db25_slot;
        let mut var_cgdvar_db26: f64 = *var_cgdvar_db26_slot;
        let mut var_cgdvar_db27: f64 = *var_cgdvar_db27_slot;
        let mut var_cgdvar_db28: f64 = *var_cgdvar_db28_slot;
        let mut var_cgdvar_db29: f64 = *var_cgdvar_db29_slot;
        let mut var_cgdvar_db3: f64 = *var_cgdvar_db3_slot;
        let mut var_cgdvar_db30: f64 = *var_cgdvar_db30_slot;
        let mut var_cgdvar_db31: f64 = *var_cgdvar_db31_slot;
        let mut var_cgdvar_db32: f64 = *var_cgdvar_db32_slot;
        let mut var_cgdvar_db33: f64 = *var_cgdvar_db33_slot;
        let mut var_cgdvar_db34: f64 = *var_cgdvar_db34_slot;
        let mut var_cgdvar_db35: f64 = *var_cgdvar_db35_slot;
        let mut var_cgdvar_db36: f64 = *var_cgdvar_db36_slot;
        let mut var_cgdvar_db37: f64 = *var_cgdvar_db37_slot;
        let mut var_cgdvar_db38: f64 = *var_cgdvar_db38_slot;
        let mut var_cgdvar_db39: f64 = *var_cgdvar_db39_slot;
        let mut var_cgdvar_db4: f64 = *var_cgdvar_db4_slot;
        let mut var_cgdvar_db40: f64 = *var_cgdvar_db40_slot;
        let mut var_cgdvar_db41: f64 = *var_cgdvar_db41_slot;
        let mut var_cgdvar_db42: f64 = *var_cgdvar_db42_slot;
        let mut var_cgdvar_db43: f64 = *var_cgdvar_db43_slot;
        let mut var_cgdvar_db44: f64 = *var_cgdvar_db44_slot;
        let mut var_cgdvar_db45: f64 = *var_cgdvar_db45_slot;
        let mut var_cgdvar_db46: f64 = *var_cgdvar_db46_slot;
        let mut var_cgdvar_db47: f64 = *var_cgdvar_db47_slot;
        let mut var_cgdvar_db48: f64 = *var_cgdvar_db48_slot;
        let mut var_cgdvar_db49: f64 = *var_cgdvar_db49_slot;
        let mut var_cgdvar_db5: f64 = *var_cgdvar_db5_slot;
        let mut var_cgdvar_db50: f64 = *var_cgdvar_db50_slot;
        let mut var_cgdvar_db51: f64 = *var_cgdvar_db51_slot;
        let mut var_cgdvar_db52: f64 = *var_cgdvar_db52_slot;
        let mut var_cgdvar_db53: f64 = *var_cgdvar_db53_slot;
        let mut var_cgdvar_db54: f64 = *var_cgdvar_db54_slot;
        let mut var_cgdvar_db55: f64 = *var_cgdvar_db55_slot;
        let mut var_cgdvar_db56: f64 = *var_cgdvar_db56_slot;
        let mut var_cgdvar_db6: f64 = *var_cgdvar_db6_slot;
        let mut var_cgdvar_db7: f64 = *var_cgdvar_db7_slot;
        let mut var_cgdvar_db8: f64 = *var_cgdvar_db8_slot;
        let mut var_cgdvar_db9: f64 = *var_cgdvar_db9_slot;
        let mut var_cgdvar_dn0: f64 = *var_cgdvar_dn0_slot;
        let mut var_cgdvar_dn1: f64 = *var_cgdvar_dn1_slot;
        let mut var_cgdvar_dn10: f64 = *var_cgdvar_dn10_slot;
        let mut var_cgdvar_dn11: f64 = *var_cgdvar_dn11_slot;
        let mut var_cgdvar_dn12: f64 = *var_cgdvar_dn12_slot;
        let mut var_cgdvar_dn13: f64 = *var_cgdvar_dn13_slot;
        let mut var_cgdvar_dn14: f64 = *var_cgdvar_dn14_slot;
        let mut var_cgdvar_dn15: f64 = *var_cgdvar_dn15_slot;
        let mut var_cgdvar_dn16: f64 = *var_cgdvar_dn16_slot;
        let mut var_cgdvar_dn17: f64 = *var_cgdvar_dn17_slot;
        let mut var_cgdvar_dn18: f64 = *var_cgdvar_dn18_slot;
        let mut var_cgdvar_dn19: f64 = *var_cgdvar_dn19_slot;
        let mut var_cgdvar_dn2: f64 = *var_cgdvar_dn2_slot;
        let mut var_cgdvar_dn20: f64 = *var_cgdvar_dn20_slot;
        let mut var_cgdvar_dn21: f64 = *var_cgdvar_dn21_slot;
        let mut var_cgdvar_dn22: f64 = *var_cgdvar_dn22_slot;
        let mut var_cgdvar_dn3: f64 = *var_cgdvar_dn3_slot;
        let mut var_cgdvar_dn4: f64 = *var_cgdvar_dn4_slot;
        let mut var_cgdvar_dn5: f64 = *var_cgdvar_dn5_slot;
        let mut var_cgdvar_dn6: f64 = *var_cgdvar_dn6_slot;
        let mut var_cgdvar_dn7: f64 = *var_cgdvar_dn7_slot;
        let mut var_cgdvar_dn8: f64 = *var_cgdvar_dn8_slot;
        let mut var_cgdvar_dn9: f64 = *var_cgdvar_dn9_slot;
        let mut var_qdov: f64 = *var_qdov_slot;
        let mut var_qdov_db0: f64 = *var_qdov_db0_slot;
        let mut var_qdov_db1: f64 = *var_qdov_db1_slot;
        let mut var_qdov_db10: f64 = *var_qdov_db10_slot;
        let mut var_qdov_db11: f64 = *var_qdov_db11_slot;
        let mut var_qdov_db12: f64 = *var_qdov_db12_slot;
        let mut var_qdov_db13: f64 = *var_qdov_db13_slot;
        let mut var_qdov_db14: f64 = *var_qdov_db14_slot;
        let mut var_qdov_db15: f64 = *var_qdov_db15_slot;
        let mut var_qdov_db16: f64 = *var_qdov_db16_slot;
        let mut var_qdov_db17: f64 = *var_qdov_db17_slot;
        let mut var_qdov_db18: f64 = *var_qdov_db18_slot;
        let mut var_qdov_db19: f64 = *var_qdov_db19_slot;
        let mut var_qdov_db2: f64 = *var_qdov_db2_slot;
        let mut var_qdov_db20: f64 = *var_qdov_db20_slot;
        let mut var_qdov_db21: f64 = *var_qdov_db21_slot;
        let mut var_qdov_db22: f64 = *var_qdov_db22_slot;
        let mut var_qdov_db23: f64 = *var_qdov_db23_slot;
        let mut var_qdov_db24: f64 = *var_qdov_db24_slot;
        let mut var_qdov_db25: f64 = *var_qdov_db25_slot;
        let mut var_qdov_db26: f64 = *var_qdov_db26_slot;
        let mut var_qdov_db27: f64 = *var_qdov_db27_slot;
        let mut var_qdov_db28: f64 = *var_qdov_db28_slot;
        let mut var_qdov_db29: f64 = *var_qdov_db29_slot;
        let mut var_qdov_db3: f64 = *var_qdov_db3_slot;
        let mut var_qdov_db30: f64 = *var_qdov_db30_slot;
        let mut var_qdov_db31: f64 = *var_qdov_db31_slot;
        let mut var_qdov_db32: f64 = *var_qdov_db32_slot;
        let mut var_qdov_db33: f64 = *var_qdov_db33_slot;
        let mut var_qdov_db34: f64 = *var_qdov_db34_slot;
        let mut var_qdov_db35: f64 = *var_qdov_db35_slot;
        let mut var_qdov_db36: f64 = *var_qdov_db36_slot;
        let mut var_qdov_db37: f64 = *var_qdov_db37_slot;
        let mut var_qdov_db38: f64 = *var_qdov_db38_slot;
        let mut var_qdov_db39: f64 = *var_qdov_db39_slot;
        let mut var_qdov_db4: f64 = *var_qdov_db4_slot;
        let mut var_qdov_db40: f64 = *var_qdov_db40_slot;
        let mut var_qdov_db41: f64 = *var_qdov_db41_slot;
        let mut var_qdov_db42: f64 = *var_qdov_db42_slot;
        let mut var_qdov_db43: f64 = *var_qdov_db43_slot;
        let mut var_qdov_db44: f64 = *var_qdov_db44_slot;
        let mut var_qdov_db45: f64 = *var_qdov_db45_slot;
        let mut var_qdov_db46: f64 = *var_qdov_db46_slot;
        let mut var_qdov_db47: f64 = *var_qdov_db47_slot;
        let mut var_qdov_db48: f64 = *var_qdov_db48_slot;
        let mut var_qdov_db49: f64 = *var_qdov_db49_slot;
        let mut var_qdov_db5: f64 = *var_qdov_db5_slot;
        let mut var_qdov_db50: f64 = *var_qdov_db50_slot;
        let mut var_qdov_db51: f64 = *var_qdov_db51_slot;
        let mut var_qdov_db52: f64 = *var_qdov_db52_slot;
        let mut var_qdov_db53: f64 = *var_qdov_db53_slot;
        let mut var_qdov_db54: f64 = *var_qdov_db54_slot;
        let mut var_qdov_db55: f64 = *var_qdov_db55_slot;
        let mut var_qdov_db56: f64 = *var_qdov_db56_slot;
        let mut var_qdov_db6: f64 = *var_qdov_db6_slot;
        let mut var_qdov_db7: f64 = *var_qdov_db7_slot;
        let mut var_qdov_db8: f64 = *var_qdov_db8_slot;
        let mut var_qdov_db9: f64 = *var_qdov_db9_slot;
        let mut var_qdov_dn0: f64 = *var_qdov_dn0_slot;
        let mut var_qdov_dn1: f64 = *var_qdov_dn1_slot;
        let mut var_qdov_dn10: f64 = *var_qdov_dn10_slot;
        let mut var_qdov_dn11: f64 = *var_qdov_dn11_slot;
        let mut var_qdov_dn12: f64 = *var_qdov_dn12_slot;
        let mut var_qdov_dn13: f64 = *var_qdov_dn13_slot;
        let mut var_qdov_dn14: f64 = *var_qdov_dn14_slot;
        let mut var_qdov_dn15: f64 = *var_qdov_dn15_slot;
        let mut var_qdov_dn16: f64 = *var_qdov_dn16_slot;
        let mut var_qdov_dn17: f64 = *var_qdov_dn17_slot;
        let mut var_qdov_dn18: f64 = *var_qdov_dn18_slot;
        let mut var_qdov_dn19: f64 = *var_qdov_dn19_slot;
        let mut var_qdov_dn2: f64 = *var_qdov_dn2_slot;
        let mut var_qdov_dn20: f64 = *var_qdov_dn20_slot;
        let mut var_qdov_dn21: f64 = *var_qdov_dn21_slot;
        let mut var_qdov_dn22: f64 = *var_qdov_dn22_slot;
        let mut var_qdov_dn3: f64 = *var_qdov_dn3_slot;
        let mut var_qdov_dn4: f64 = *var_qdov_dn4_slot;
        let mut var_qdov_dn5: f64 = *var_qdov_dn5_slot;
        let mut var_qdov_dn6: f64 = *var_qdov_dn6_slot;
        let mut var_qdov_dn7: f64 = *var_qdov_dn7_slot;
        let mut var_qdov_dn8: f64 = *var_qdov_dn8_slot;
        let mut var_qdov_dn9: f64 = *var_qdov_dn9_slot;
        let mut var_qsov: f64 = *var_qsov_slot;
        let mut var_qsov_db0: f64 = *var_qsov_db0_slot;
        let mut var_qsov_db1: f64 = *var_qsov_db1_slot;
        let mut var_qsov_db10: f64 = *var_qsov_db10_slot;
        let mut var_qsov_db11: f64 = *var_qsov_db11_slot;
        let mut var_qsov_db12: f64 = *var_qsov_db12_slot;
        let mut var_qsov_db13: f64 = *var_qsov_db13_slot;
        let mut var_qsov_db14: f64 = *var_qsov_db14_slot;
        let mut var_qsov_db15: f64 = *var_qsov_db15_slot;
        let mut var_qsov_db16: f64 = *var_qsov_db16_slot;
        let mut var_qsov_db17: f64 = *var_qsov_db17_slot;
        let mut var_qsov_db18: f64 = *var_qsov_db18_slot;
        let mut var_qsov_db19: f64 = *var_qsov_db19_slot;
        let mut var_qsov_db2: f64 = *var_qsov_db2_slot;
        let mut var_qsov_db20: f64 = *var_qsov_db20_slot;
        let mut var_qsov_db21: f64 = *var_qsov_db21_slot;
        let mut var_qsov_db22: f64 = *var_qsov_db22_slot;
        let mut var_qsov_db23: f64 = *var_qsov_db23_slot;
        let mut var_qsov_db24: f64 = *var_qsov_db24_slot;
        let mut var_qsov_db25: f64 = *var_qsov_db25_slot;
        let mut var_qsov_db26: f64 = *var_qsov_db26_slot;
        let mut var_qsov_db27: f64 = *var_qsov_db27_slot;
        let mut var_qsov_db28: f64 = *var_qsov_db28_slot;
        let mut var_qsov_db29: f64 = *var_qsov_db29_slot;
        let mut var_qsov_db3: f64 = *var_qsov_db3_slot;
        let mut var_qsov_db30: f64 = *var_qsov_db30_slot;
        let mut var_qsov_db31: f64 = *var_qsov_db31_slot;
        let mut var_qsov_db32: f64 = *var_qsov_db32_slot;
        let mut var_qsov_db33: f64 = *var_qsov_db33_slot;
        let mut var_qsov_db34: f64 = *var_qsov_db34_slot;
        let mut var_qsov_db35: f64 = *var_qsov_db35_slot;
        let mut var_qsov_db36: f64 = *var_qsov_db36_slot;
        let mut var_qsov_db37: f64 = *var_qsov_db37_slot;
        let mut var_qsov_db38: f64 = *var_qsov_db38_slot;
        let mut var_qsov_db39: f64 = *var_qsov_db39_slot;
        let mut var_qsov_db4: f64 = *var_qsov_db4_slot;
        let mut var_qsov_db40: f64 = *var_qsov_db40_slot;
        let mut var_qsov_db41: f64 = *var_qsov_db41_slot;
        let mut var_qsov_db42: f64 = *var_qsov_db42_slot;
        let mut var_qsov_db43: f64 = *var_qsov_db43_slot;
        let mut var_qsov_db44: f64 = *var_qsov_db44_slot;
        let mut var_qsov_db45: f64 = *var_qsov_db45_slot;
        let mut var_qsov_db46: f64 = *var_qsov_db46_slot;
        let mut var_qsov_db47: f64 = *var_qsov_db47_slot;
        let mut var_qsov_db48: f64 = *var_qsov_db48_slot;
        let mut var_qsov_db49: f64 = *var_qsov_db49_slot;
        let mut var_qsov_db5: f64 = *var_qsov_db5_slot;
        let mut var_qsov_db50: f64 = *var_qsov_db50_slot;
        let mut var_qsov_db51: f64 = *var_qsov_db51_slot;
        let mut var_qsov_db52: f64 = *var_qsov_db52_slot;
        let mut var_qsov_db53: f64 = *var_qsov_db53_slot;
        let mut var_qsov_db54: f64 = *var_qsov_db54_slot;
        let mut var_qsov_db55: f64 = *var_qsov_db55_slot;
        let mut var_qsov_db56: f64 = *var_qsov_db56_slot;
        let mut var_qsov_db6: f64 = *var_qsov_db6_slot;
        let mut var_qsov_db7: f64 = *var_qsov_db7_slot;
        let mut var_qsov_db8: f64 = *var_qsov_db8_slot;
        let mut var_qsov_db9: f64 = *var_qsov_db9_slot;
        let mut var_qsov_dn0: f64 = *var_qsov_dn0_slot;
        let mut var_qsov_dn1: f64 = *var_qsov_dn1_slot;
        let mut var_qsov_dn10: f64 = *var_qsov_dn10_slot;
        let mut var_qsov_dn11: f64 = *var_qsov_dn11_slot;
        let mut var_qsov_dn12: f64 = *var_qsov_dn12_slot;
        let mut var_qsov_dn13: f64 = *var_qsov_dn13_slot;
        let mut var_qsov_dn14: f64 = *var_qsov_dn14_slot;
        let mut var_qsov_dn15: f64 = *var_qsov_dn15_slot;
        let mut var_qsov_dn16: f64 = *var_qsov_dn16_slot;
        let mut var_qsov_dn17: f64 = *var_qsov_dn17_slot;
        let mut var_qsov_dn18: f64 = *var_qsov_dn18_slot;
        let mut var_qsov_dn19: f64 = *var_qsov_dn19_slot;
        let mut var_qsov_dn2: f64 = *var_qsov_dn2_slot;
        let mut var_qsov_dn20: f64 = *var_qsov_dn20_slot;
        let mut var_qsov_dn21: f64 = *var_qsov_dn21_slot;
        let mut var_qsov_dn22: f64 = *var_qsov_dn22_slot;
        let mut var_qsov_dn3: f64 = *var_qsov_dn3_slot;
        let mut var_qsov_dn4: f64 = *var_qsov_dn4_slot;
        let mut var_qsov_dn5: f64 = *var_qsov_dn5_slot;
        let mut var_qsov_dn6: f64 = *var_qsov_dn6_slot;
        let mut var_qsov_dn7: f64 = *var_qsov_dn7_slot;
        let mut var_qsov_dn8: f64 = *var_qsov_dn8_slot;
        let mut var_qsov_dn9: f64 = *var_qsov_dn9_slot;
        let mut var_vdseffcv: f64 = *var_vdseffcv_slot;
        let mut var_vdseffcv_db0: f64 = *var_vdseffcv_db0_slot;
        let mut var_vdseffcv_db1: f64 = *var_vdseffcv_db1_slot;
        let mut var_vdseffcv_db10: f64 = *var_vdseffcv_db10_slot;
        let mut var_vdseffcv_db11: f64 = *var_vdseffcv_db11_slot;
        let mut var_vdseffcv_db12: f64 = *var_vdseffcv_db12_slot;
        let mut var_vdseffcv_db13: f64 = *var_vdseffcv_db13_slot;
        let mut var_vdseffcv_db14: f64 = *var_vdseffcv_db14_slot;
        let mut var_vdseffcv_db15: f64 = *var_vdseffcv_db15_slot;
        let mut var_vdseffcv_db16: f64 = *var_vdseffcv_db16_slot;
        let mut var_vdseffcv_db17: f64 = *var_vdseffcv_db17_slot;
        let mut var_vdseffcv_db18: f64 = *var_vdseffcv_db18_slot;
        let mut var_vdseffcv_db19: f64 = *var_vdseffcv_db19_slot;
        let mut var_vdseffcv_db2: f64 = *var_vdseffcv_db2_slot;
        let mut var_vdseffcv_db20: f64 = *var_vdseffcv_db20_slot;
        let mut var_vdseffcv_db21: f64 = *var_vdseffcv_db21_slot;
        let mut var_vdseffcv_db22: f64 = *var_vdseffcv_db22_slot;
        let mut var_vdseffcv_db23: f64 = *var_vdseffcv_db23_slot;
        let mut var_vdseffcv_db24: f64 = *var_vdseffcv_db24_slot;
        let mut var_vdseffcv_db25: f64 = *var_vdseffcv_db25_slot;
        let mut var_vdseffcv_db26: f64 = *var_vdseffcv_db26_slot;
        let mut var_vdseffcv_db27: f64 = *var_vdseffcv_db27_slot;
        let mut var_vdseffcv_db28: f64 = *var_vdseffcv_db28_slot;
        let mut var_vdseffcv_db29: f64 = *var_vdseffcv_db29_slot;
        let mut var_vdseffcv_db3: f64 = *var_vdseffcv_db3_slot;
        let mut var_vdseffcv_db30: f64 = *var_vdseffcv_db30_slot;
        let mut var_vdseffcv_db31: f64 = *var_vdseffcv_db31_slot;
        let mut var_vdseffcv_db32: f64 = *var_vdseffcv_db32_slot;
        let mut var_vdseffcv_db33: f64 = *var_vdseffcv_db33_slot;
        let mut var_vdseffcv_db34: f64 = *var_vdseffcv_db34_slot;
        let mut var_vdseffcv_db35: f64 = *var_vdseffcv_db35_slot;
        let mut var_vdseffcv_db36: f64 = *var_vdseffcv_db36_slot;
        let mut var_vdseffcv_db37: f64 = *var_vdseffcv_db37_slot;
        let mut var_vdseffcv_db38: f64 = *var_vdseffcv_db38_slot;
        let mut var_vdseffcv_db39: f64 = *var_vdseffcv_db39_slot;
        let mut var_vdseffcv_db4: f64 = *var_vdseffcv_db4_slot;
        let mut var_vdseffcv_db40: f64 = *var_vdseffcv_db40_slot;
        let mut var_vdseffcv_db41: f64 = *var_vdseffcv_db41_slot;
        let mut var_vdseffcv_db42: f64 = *var_vdseffcv_db42_slot;
        let mut var_vdseffcv_db43: f64 = *var_vdseffcv_db43_slot;
        let mut var_vdseffcv_db44: f64 = *var_vdseffcv_db44_slot;
        let mut var_vdseffcv_db45: f64 = *var_vdseffcv_db45_slot;
        let mut var_vdseffcv_db46: f64 = *var_vdseffcv_db46_slot;
        let mut var_vdseffcv_db47: f64 = *var_vdseffcv_db47_slot;
        let mut var_vdseffcv_db48: f64 = *var_vdseffcv_db48_slot;
        let mut var_vdseffcv_db49: f64 = *var_vdseffcv_db49_slot;
        let mut var_vdseffcv_db5: f64 = *var_vdseffcv_db5_slot;
        let mut var_vdseffcv_db50: f64 = *var_vdseffcv_db50_slot;
        let mut var_vdseffcv_db51: f64 = *var_vdseffcv_db51_slot;
        let mut var_vdseffcv_db52: f64 = *var_vdseffcv_db52_slot;
        let mut var_vdseffcv_db53: f64 = *var_vdseffcv_db53_slot;
        let mut var_vdseffcv_db54: f64 = *var_vdseffcv_db54_slot;
        let mut var_vdseffcv_db55: f64 = *var_vdseffcv_db55_slot;
        let mut var_vdseffcv_db56: f64 = *var_vdseffcv_db56_slot;
        let mut var_vdseffcv_db6: f64 = *var_vdseffcv_db6_slot;
        let mut var_vdseffcv_db7: f64 = *var_vdseffcv_db7_slot;
        let mut var_vdseffcv_db8: f64 = *var_vdseffcv_db8_slot;
        let mut var_vdseffcv_db9: f64 = *var_vdseffcv_db9_slot;
        let mut var_vdseffcv_dn0: f64 = *var_vdseffcv_dn0_slot;
        let mut var_vdseffcv_dn1: f64 = *var_vdseffcv_dn1_slot;
        let mut var_vdseffcv_dn10: f64 = *var_vdseffcv_dn10_slot;
        let mut var_vdseffcv_dn11: f64 = *var_vdseffcv_dn11_slot;
        let mut var_vdseffcv_dn12: f64 = *var_vdseffcv_dn12_slot;
        let mut var_vdseffcv_dn13: f64 = *var_vdseffcv_dn13_slot;
        let mut var_vdseffcv_dn14: f64 = *var_vdseffcv_dn14_slot;
        let mut var_vdseffcv_dn15: f64 = *var_vdseffcv_dn15_slot;
        let mut var_vdseffcv_dn16: f64 = *var_vdseffcv_dn16_slot;
        let mut var_vdseffcv_dn17: f64 = *var_vdseffcv_dn17_slot;
        let mut var_vdseffcv_dn18: f64 = *var_vdseffcv_dn18_slot;
        let mut var_vdseffcv_dn19: f64 = *var_vdseffcv_dn19_slot;
        let mut var_vdseffcv_dn2: f64 = *var_vdseffcv_dn2_slot;
        let mut var_vdseffcv_dn20: f64 = *var_vdseffcv_dn20_slot;
        let mut var_vdseffcv_dn21: f64 = *var_vdseffcv_dn21_slot;
        let mut var_vdseffcv_dn22: f64 = *var_vdseffcv_dn22_slot;
        let mut var_vdseffcv_dn3: f64 = *var_vdseffcv_dn3_slot;
        let mut var_vdseffcv_dn4: f64 = *var_vdseffcv_dn4_slot;
        let mut var_vdseffcv_dn5: f64 = *var_vdseffcv_dn5_slot;
        let mut var_vdseffcv_dn6: f64 = *var_vdseffcv_dn6_slot;
        let mut var_vdseffcv_dn7: f64 = *var_vdseffcv_dn7_slot;
        let mut var_vdseffcv_dn8: f64 = *var_vdseffcv_dn8_slot;
        let mut var_vdseffcv_dn9: f64 = *var_vdseffcv_dn9_slot;

        let (assign31030_e48934, assign31030_e48934_d_n0, assign31030_e48934_d_n1, assign31030_e48934_d_n2, assign31030_e48934_d_n3, assign31030_e48934_d_n4, assign31030_e48934_d_n5, assign31030_e48934_d_n6, assign31030_e48934_d_n7, assign31030_e48934_d_n8, assign31030_e48934_d_n9, assign31030_e48934_d_n10, assign31030_e48934_d_n11, assign31030_e48934_d_n12, assign31030_e48934_d_n13, assign31030_e48934_d_n14, assign31030_e48934_d_n15, assign31030_e48934_d_n16, assign31030_e48934_d_n17, assign31030_e48934_d_n18, assign31030_e48934_d_n19, assign31030_e48934_d_n20, assign31030_e48934_d_n21, assign31030_e48934_d_n22, assign31030_e48934_d_b0, assign31030_e48934_d_b1, assign31030_e48934_d_b2, assign31030_e48934_d_b3, assign31030_e48934_d_b4, assign31030_e48934_d_b5, assign31030_e48934_d_b6, assign31030_e48934_d_b7, assign31030_e48934_d_b8, assign31030_e48934_d_b9, assign31030_e48934_d_b10, assign31030_e48934_d_b11, assign31030_e48934_d_b12, assign31030_e48934_d_b13, assign31030_e48934_d_b14, assign31030_e48934_d_b15, assign31030_e48934_d_b16, assign31030_e48934_d_b17, assign31030_e48934_d_b18, assign31030_e48934_d_b19, assign31030_e48934_d_b20, assign31030_e48934_d_b21, assign31030_e48934_d_b22, assign31030_e48934_d_b23, assign31030_e48934_d_b24, assign31030_e48934_d_b25, assign31030_e48934_d_b26, assign31030_e48934_d_b27, assign31030_e48934_d_b28, assign31030_e48934_d_b29, assign31030_e48934_d_b30, assign31030_e48934_d_b31, assign31030_e48934_d_b32, assign31030_e48934_d_b33, assign31030_e48934_d_b34, assign31030_e48934_d_b35, assign31030_e48934_d_b36, assign31030_e48934_d_b37, assign31030_e48934_d_b38, assign31030_e48934_d_b39, assign31030_e48934_d_b40, assign31030_e48934_d_b41, assign31030_e48934_d_b42, assign31030_e48934_d_b43, assign31030_e48934_d_b44, assign31030_e48934_d_b45, assign31030_e48934_d_b46, assign31030_e48934_d_b47, assign31030_e48934_d_b48, assign31030_e48934_d_b49, assign31030_e48934_d_b50, assign31030_e48934_d_b51, assign31030_e48934_d_b52, assign31030_e48934_d_b53, assign31030_e48934_d_b54,) = {
    if (var_guard524 != 0.0) {
        let assign31030_e48922: f64 = (p.p4 * p.p5);
        let assign31030_e48924: f64 = (assign31030_e48922 * p.p211);
        let assign31030_e48927: f64 = (p.p4 * p.p5);
        let assign31030_e48929: f64 = (assign31030_e48927 * var_cgdl_l);
        let assign31030_e48931: f64 = (assign31030_e48929 * var_vdseffcv);
        let assign31030_e48932: f64 = (assign31030_e48924 - assign31030_e48931);
        (assign31030_e48932, (-(assign31030_e48929 * var_vdseffcv_dn0)), (-(assign31030_e48929 * var_vdseffcv_dn1)), (-(assign31030_e48929 * var_vdseffcv_dn2)), (-(assign31030_e48929 * var_vdseffcv_dn3)), (-(assign31030_e48929 * var_vdseffcv_dn4)), (-(assign31030_e48929 * var_vdseffcv_dn5)), (-(assign31030_e48929 * var_vdseffcv_dn6)), (-(assign31030_e48929 * var_vdseffcv_dn7)), (-(assign31030_e48929 * var_vdseffcv_dn8)), (-(assign31030_e48929 * var_vdseffcv_dn9)), (-(assign31030_e48929 * var_vdseffcv_dn10)), (-(assign31030_e48929 * var_vdseffcv_dn11)), (-(assign31030_e48929 * var_vdseffcv_dn12)), (-(assign31030_e48929 * var_vdseffcv_dn13)), (-(assign31030_e48929 * var_vdseffcv_dn14)), (-(assign31030_e48929 * var_vdseffcv_dn15)), (-(assign31030_e48929 * var_vdseffcv_dn16)), (-(assign31030_e48929 * var_vdseffcv_dn17)), (-(assign31030_e48929 * var_vdseffcv_dn18)), (-(assign31030_e48929 * var_vdseffcv_dn19)), (-(assign31030_e48929 * var_vdseffcv_dn20)), (-(assign31030_e48929 * var_vdseffcv_dn21)), (-(assign31030_e48929 * var_vdseffcv_dn22)), (-(assign31030_e48929 * var_vdseffcv_db0)), (-(assign31030_e48929 * var_vdseffcv_db1)), (-(assign31030_e48929 * var_vdseffcv_db2)), (-(assign31030_e48929 * var_vdseffcv_db3)), (-(assign31030_e48929 * var_vdseffcv_db4)), (-(assign31030_e48929 * var_vdseffcv_db5)), (-(assign31030_e48929 * var_vdseffcv_db6)), (-(assign31030_e48929 * var_vdseffcv_db7)), (-(assign31030_e48929 * var_vdseffcv_db8)), (-(assign31030_e48929 * var_vdseffcv_db9)), (-(assign31030_e48929 * var_vdseffcv_db10)), (-(assign31030_e48929 * var_vdseffcv_db11)), (-(assign31030_e48929 * var_vdseffcv_db12)), (-(assign31030_e48929 * var_vdseffcv_db13)), (-(assign31030_e48929 * var_vdseffcv_db14)), (-(assign31030_e48929 * var_vdseffcv_db15)), (-(assign31030_e48929 * var_vdseffcv_db16)), (-(assign31030_e48929 * var_vdseffcv_db17)), (-(assign31030_e48929 * var_vdseffcv_db18)), (-(assign31030_e48929 * var_vdseffcv_db19)), (-(assign31030_e48929 * var_vdseffcv_db20)), (-(assign31030_e48929 * var_vdseffcv_db21)), (-(assign31030_e48929 * var_vdseffcv_db22)), (-(assign31030_e48929 * var_vdseffcv_db23)), (-(assign31030_e48929 * var_vdseffcv_db24)), (-(assign31030_e48929 * var_vdseffcv_db25)), (-(assign31030_e48929 * var_vdseffcv_db26)), (-(assign31030_e48929 * var_vdseffcv_db27)), (-(assign31030_e48929 * var_vdseffcv_db28)), (-(assign31030_e48929 * var_vdseffcv_db29)), (-(assign31030_e48929 * var_vdseffcv_db30)), (-(assign31030_e48929 * var_vdseffcv_db31)), (-(assign31030_e48929 * var_vdseffcv_db32)), (-(assign31030_e48929 * var_vdseffcv_db33)), (-(assign31030_e48929 * var_vdseffcv_db34)), (-(assign31030_e48929 * var_vdseffcv_db35)), (-(assign31030_e48929 * var_vdseffcv_db36)), (-(assign31030_e48929 * var_vdseffcv_db37)), (-(assign31030_e48929 * var_vdseffcv_db38)), (-(assign31030_e48929 * var_vdseffcv_db39)), (-(assign31030_e48929 * var_vdseffcv_db40)), (-(assign31030_e48929 * var_vdseffcv_db41)), (-(assign31030_e48929 * var_vdseffcv_db42)), (-(assign31030_e48929 * var_vdseffcv_db43)), (-(assign31030_e48929 * var_vdseffcv_db44)), (-(assign31030_e48929 * var_vdseffcv_db45)), (-(assign31030_e48929 * var_vdseffcv_db46)), (-(assign31030_e48929 * var_vdseffcv_db47)), (-(assign31030_e48929 * var_vdseffcv_db48)), (-(assign31030_e48929 * var_vdseffcv_db49)), (-(assign31030_e48929 * var_vdseffcv_db50)), (-(assign31030_e48929 * var_vdseffcv_db51)), (-(assign31030_e48929 * var_vdseffcv_db52)), (-(assign31030_e48929 * var_vdseffcv_db53)), (-(assign31030_e48929 * var_vdseffcv_db54)),)
    } else {
        (var_cgdvar, var_cgdvar_dn0, var_cgdvar_dn1, var_cgdvar_dn2, var_cgdvar_dn3, var_cgdvar_dn4, var_cgdvar_dn5, var_cgdvar_dn6, var_cgdvar_dn7, var_cgdvar_dn8, var_cgdvar_dn9, var_cgdvar_dn10, var_cgdvar_dn11, var_cgdvar_dn12, var_cgdvar_dn13, var_cgdvar_dn14, var_cgdvar_dn15, var_cgdvar_dn16, var_cgdvar_dn17, var_cgdvar_dn18, var_cgdvar_dn19, var_cgdvar_dn20, var_cgdvar_dn21, var_cgdvar_dn22, var_cgdvar_db0, var_cgdvar_db1, var_cgdvar_db2, var_cgdvar_db3, var_cgdvar_db4, var_cgdvar_db5, var_cgdvar_db6, var_cgdvar_db7, var_cgdvar_db8, var_cgdvar_db9, var_cgdvar_db10, var_cgdvar_db11, var_cgdvar_db12, var_cgdvar_db13, var_cgdvar_db14, var_cgdvar_db15, var_cgdvar_db16, var_cgdvar_db17, var_cgdvar_db18, var_cgdvar_db19, var_cgdvar_db20, var_cgdvar_db21, var_cgdvar_db22, var_cgdvar_db23, var_cgdvar_db24, var_cgdvar_db25, var_cgdvar_db26, var_cgdvar_db27, var_cgdvar_db28, var_cgdvar_db29, var_cgdvar_db30, var_cgdvar_db31, var_cgdvar_db32, var_cgdvar_db33, var_cgdvar_db34, var_cgdvar_db35, var_cgdvar_db36, var_cgdvar_db37, var_cgdvar_db38, var_cgdvar_db39, var_cgdvar_db40, var_cgdvar_db41, var_cgdvar_db42, var_cgdvar_db43, var_cgdvar_db44, var_cgdvar_db45, var_cgdvar_db46, var_cgdvar_db47, var_cgdvar_db48, var_cgdvar_db49, var_cgdvar_db50, var_cgdvar_db51, var_cgdvar_db52, var_cgdvar_db53, var_cgdvar_db54,)
    }
};
        var_cgdvar = assign31030_e48934;
        var_cgdvar_dn0 = assign31030_e48934_d_n0;
        var_cgdvar_dn1 = assign31030_e48934_d_n1;
        var_cgdvar_dn2 = assign31030_e48934_d_n2;
        var_cgdvar_dn3 = assign31030_e48934_d_n3;
        var_cgdvar_dn4 = assign31030_e48934_d_n4;
        var_cgdvar_dn5 = assign31030_e48934_d_n5;
        var_cgdvar_dn6 = assign31030_e48934_d_n6;
        var_cgdvar_dn7 = assign31030_e48934_d_n7;
        var_cgdvar_dn8 = assign31030_e48934_d_n8;
        var_cgdvar_dn9 = assign31030_e48934_d_n9;
        var_cgdvar_dn10 = assign31030_e48934_d_n10;
        var_cgdvar_dn11 = assign31030_e48934_d_n11;
        var_cgdvar_dn12 = assign31030_e48934_d_n12;
        var_cgdvar_dn13 = assign31030_e48934_d_n13;
        var_cgdvar_dn14 = assign31030_e48934_d_n14;
        var_cgdvar_dn15 = assign31030_e48934_d_n15;
        var_cgdvar_dn16 = assign31030_e48934_d_n16;
        var_cgdvar_dn17 = assign31030_e48934_d_n17;
        var_cgdvar_dn18 = assign31030_e48934_d_n18;
        var_cgdvar_dn19 = assign31030_e48934_d_n19;
        var_cgdvar_dn20 = assign31030_e48934_d_n20;
        var_cgdvar_dn21 = assign31030_e48934_d_n21;
        var_cgdvar_dn22 = assign31030_e48934_d_n22;
        var_cgdvar_db0 = assign31030_e48934_d_b0;
        var_cgdvar_db1 = assign31030_e48934_d_b1;
        var_cgdvar_db2 = assign31030_e48934_d_b2;
        var_cgdvar_db3 = assign31030_e48934_d_b3;
        var_cgdvar_db4 = assign31030_e48934_d_b4;
        var_cgdvar_db5 = assign31030_e48934_d_b5;
        var_cgdvar_db6 = assign31030_e48934_d_b6;
        var_cgdvar_db7 = assign31030_e48934_d_b7;
        var_cgdvar_db8 = assign31030_e48934_d_b8;
        var_cgdvar_db9 = assign31030_e48934_d_b9;
        var_cgdvar_db10 = assign31030_e48934_d_b10;
        var_cgdvar_db11 = assign31030_e48934_d_b11;
        var_cgdvar_db12 = assign31030_e48934_d_b12;
        var_cgdvar_db13 = assign31030_e48934_d_b13;
        var_cgdvar_db14 = assign31030_e48934_d_b14;
        var_cgdvar_db15 = assign31030_e48934_d_b15;
        var_cgdvar_db16 = assign31030_e48934_d_b16;
        var_cgdvar_db17 = assign31030_e48934_d_b17;
        var_cgdvar_db18 = assign31030_e48934_d_b18;
        var_cgdvar_db19 = assign31030_e48934_d_b19;
        var_cgdvar_db20 = assign31030_e48934_d_b20;
        var_cgdvar_db21 = assign31030_e48934_d_b21;
        var_cgdvar_db22 = assign31030_e48934_d_b22;
        var_cgdvar_db23 = assign31030_e48934_d_b23;
        var_cgdvar_db24 = assign31030_e48934_d_b24;
        var_cgdvar_db25 = assign31030_e48934_d_b25;
        var_cgdvar_db26 = assign31030_e48934_d_b26;
        var_cgdvar_db27 = assign31030_e48934_d_b27;
        var_cgdvar_db28 = assign31030_e48934_d_b28;
        var_cgdvar_db29 = assign31030_e48934_d_b29;
        var_cgdvar_db30 = assign31030_e48934_d_b30;
        var_cgdvar_db31 = assign31030_e48934_d_b31;
        var_cgdvar_db32 = assign31030_e48934_d_b32;
        var_cgdvar_db33 = assign31030_e48934_d_b33;
        var_cgdvar_db34 = assign31030_e48934_d_b34;
        var_cgdvar_db35 = assign31030_e48934_d_b35;
        var_cgdvar_db36 = assign31030_e48934_d_b36;
        var_cgdvar_db37 = assign31030_e48934_d_b37;
        var_cgdvar_db38 = assign31030_e48934_d_b38;
        var_cgdvar_db39 = assign31030_e48934_d_b39;
        var_cgdvar_db40 = assign31030_e48934_d_b40;
        var_cgdvar_db41 = assign31030_e48934_d_b41;
        var_cgdvar_db42 = assign31030_e48934_d_b42;
        var_cgdvar_db43 = assign31030_e48934_d_b43;
        var_cgdvar_db44 = assign31030_e48934_d_b44;
        var_cgdvar_db45 = assign31030_e48934_d_b45;
        var_cgdvar_db46 = assign31030_e48934_d_b46;
        var_cgdvar_db47 = assign31030_e48934_d_b47;
        var_cgdvar_db48 = assign31030_e48934_d_b48;
        var_cgdvar_db49 = assign31030_e48934_d_b49;
        var_cgdvar_db50 = assign31030_e48934_d_b50;
        var_cgdvar_db51 = assign31030_e48934_d_b51;
        var_cgdvar_db52 = assign31030_e48934_d_b52;
        var_cgdvar_db53 = assign31030_e48934_d_b53;
        var_cgdvar_db54 = assign31030_e48934_d_b54;
        var_cgdvar_db55 = 0.0;
        var_cgdvar_db56 = 0.0;

        let (assign31040_e48942, assign31040_e48942_d_n0, assign31040_e48942_d_n1, assign31040_e48942_d_n2, assign31040_e48942_d_n3, assign31040_e48942_d_n4, assign31040_e48942_d_n5, assign31040_e48942_d_n6, assign31040_e48942_d_n7, assign31040_e48942_d_n8, assign31040_e48942_d_n9, assign31040_e48942_d_n10, assign31040_e48942_d_n11, assign31040_e48942_d_n12, assign31040_e48942_d_n13, assign31040_e48942_d_n14, assign31040_e48942_d_n15, assign31040_e48942_d_n16, assign31040_e48942_d_n17, assign31040_e48942_d_n18, assign31040_e48942_d_n19, assign31040_e48942_d_n20, assign31040_e48942_d_n21, assign31040_e48942_d_n22, assign31040_e48942_d_b0, assign31040_e48942_d_b1, assign31040_e48942_d_b2, assign31040_e48942_d_b3, assign31040_e48942_d_b4, assign31040_e48942_d_b5, assign31040_e48942_d_b6, assign31040_e48942_d_b7, assign31040_e48942_d_b8, assign31040_e48942_d_b9, assign31040_e48942_d_b10, assign31040_e48942_d_b11, assign31040_e48942_d_b12, assign31040_e48942_d_b13, assign31040_e48942_d_b14, assign31040_e48942_d_b15, assign31040_e48942_d_b16, assign31040_e48942_d_b17, assign31040_e48942_d_b18, assign31040_e48942_d_b19, assign31040_e48942_d_b20, assign31040_e48942_d_b21, assign31040_e48942_d_b22, assign31040_e48942_d_b23, assign31040_e48942_d_b24, assign31040_e48942_d_b25, assign31040_e48942_d_b26, assign31040_e48942_d_b27, assign31040_e48942_d_b28, assign31040_e48942_d_b29, assign31040_e48942_d_b30, assign31040_e48942_d_b31, assign31040_e48942_d_b32, assign31040_e48942_d_b33, assign31040_e48942_d_b34, assign31040_e48942_d_b35, assign31040_e48942_d_b36, assign31040_e48942_d_b37, assign31040_e48942_d_b38, assign31040_e48942_d_b39, assign31040_e48942_d_b40, assign31040_e48942_d_b41, assign31040_e48942_d_b42, assign31040_e48942_d_b43, assign31040_e48942_d_b44, assign31040_e48942_d_b45, assign31040_e48942_d_b46, assign31040_e48942_d_b47, assign31040_e48942_d_b48, assign31040_e48942_d_b49, assign31040_e48942_d_b50, assign31040_e48942_d_b51, assign31040_e48942_d_b52, assign31040_e48942_d_b53, assign31040_e48942_d_b54,) = {
    if (var_guard524 != 0.0) {
        let assign31040_e48938: f64 = (var_cgdvar).max(0.0);
        let assign31040_e48940: f64 = (assign31040_e48938 * (nv10 - nv0));
        (assign31040_e48940, ((if var_cgdvar >= 0.0 { var_cgdvar_dn0 } else { 0.0 } * (nv10 - nv0)) + (-assign31040_e48938)), (if var_cgdvar >= 0.0 { var_cgdvar_dn1 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn2 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn3 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn4 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn5 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn6 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn7 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn8 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn9 } else { 0.0 } * (nv10 - nv0)), ((if var_cgdvar >= 0.0 { var_cgdvar_dn10 } else { 0.0 } * (nv10 - nv0)) + assign31040_e48938), (if var_cgdvar >= 0.0 { var_cgdvar_dn11 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn12 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn13 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn14 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn15 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn16 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn17 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn18 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn19 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn20 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn21 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn22 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db0 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db1 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db2 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db3 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db4 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db5 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db6 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db7 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db8 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db9 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db10 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db11 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db12 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db13 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db14 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db15 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db16 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db17 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db18 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db19 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db20 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db21 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db22 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db23 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db24 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db25 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db26 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db27 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db28 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db29 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db30 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db31 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db32 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db33 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db34 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db35 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db36 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db37 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db38 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db39 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db40 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db41 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db42 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db43 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db44 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db45 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db46 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db47 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db48 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db49 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db50 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db51 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db52 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db53 } else { 0.0 } * (nv10 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db54 } else { 0.0 } * (nv10 - nv0)),)
    } else {
        (var_qdov, var_qdov_dn0, var_qdov_dn1, var_qdov_dn2, var_qdov_dn3, var_qdov_dn4, var_qdov_dn5, var_qdov_dn6, var_qdov_dn7, var_qdov_dn8, var_qdov_dn9, var_qdov_dn10, var_qdov_dn11, var_qdov_dn12, var_qdov_dn13, var_qdov_dn14, var_qdov_dn15, var_qdov_dn16, var_qdov_dn17, var_qdov_dn18, var_qdov_dn19, var_qdov_dn20, var_qdov_dn21, var_qdov_dn22, var_qdov_db0, var_qdov_db1, var_qdov_db2, var_qdov_db3, var_qdov_db4, var_qdov_db5, var_qdov_db6, var_qdov_db7, var_qdov_db8, var_qdov_db9, var_qdov_db10, var_qdov_db11, var_qdov_db12, var_qdov_db13, var_qdov_db14, var_qdov_db15, var_qdov_db16, var_qdov_db17, var_qdov_db18, var_qdov_db19, var_qdov_db20, var_qdov_db21, var_qdov_db22, var_qdov_db23, var_qdov_db24, var_qdov_db25, var_qdov_db26, var_qdov_db27, var_qdov_db28, var_qdov_db29, var_qdov_db30, var_qdov_db31, var_qdov_db32, var_qdov_db33, var_qdov_db34, var_qdov_db35, var_qdov_db36, var_qdov_db37, var_qdov_db38, var_qdov_db39, var_qdov_db40, var_qdov_db41, var_qdov_db42, var_qdov_db43, var_qdov_db44, var_qdov_db45, var_qdov_db46, var_qdov_db47, var_qdov_db48, var_qdov_db49, var_qdov_db50, var_qdov_db51, var_qdov_db52, var_qdov_db53, var_qdov_db54,)
    }
};
        var_qdov = assign31040_e48942;
        var_qdov_dn0 = assign31040_e48942_d_n0;
        var_qdov_dn1 = assign31040_e48942_d_n1;
        var_qdov_dn2 = assign31040_e48942_d_n2;
        var_qdov_dn3 = assign31040_e48942_d_n3;
        var_qdov_dn4 = assign31040_e48942_d_n4;
        var_qdov_dn5 = assign31040_e48942_d_n5;
        var_qdov_dn6 = assign31040_e48942_d_n6;
        var_qdov_dn7 = assign31040_e48942_d_n7;
        var_qdov_dn8 = assign31040_e48942_d_n8;
        var_qdov_dn9 = assign31040_e48942_d_n9;
        var_qdov_dn10 = assign31040_e48942_d_n10;
        var_qdov_dn11 = assign31040_e48942_d_n11;
        var_qdov_dn12 = assign31040_e48942_d_n12;
        var_qdov_dn13 = assign31040_e48942_d_n13;
        var_qdov_dn14 = assign31040_e48942_d_n14;
        var_qdov_dn15 = assign31040_e48942_d_n15;
        var_qdov_dn16 = assign31040_e48942_d_n16;
        var_qdov_dn17 = assign31040_e48942_d_n17;
        var_qdov_dn18 = assign31040_e48942_d_n18;
        var_qdov_dn19 = assign31040_e48942_d_n19;
        var_qdov_dn20 = assign31040_e48942_d_n20;
        var_qdov_dn21 = assign31040_e48942_d_n21;
        var_qdov_dn22 = assign31040_e48942_d_n22;
        var_qdov_db0 = assign31040_e48942_d_b0;
        var_qdov_db1 = assign31040_e48942_d_b1;
        var_qdov_db2 = assign31040_e48942_d_b2;
        var_qdov_db3 = assign31040_e48942_d_b3;
        var_qdov_db4 = assign31040_e48942_d_b4;
        var_qdov_db5 = assign31040_e48942_d_b5;
        var_qdov_db6 = assign31040_e48942_d_b6;
        var_qdov_db7 = assign31040_e48942_d_b7;
        var_qdov_db8 = assign31040_e48942_d_b8;
        var_qdov_db9 = assign31040_e48942_d_b9;
        var_qdov_db10 = assign31040_e48942_d_b10;
        var_qdov_db11 = assign31040_e48942_d_b11;
        var_qdov_db12 = assign31040_e48942_d_b12;
        var_qdov_db13 = assign31040_e48942_d_b13;
        var_qdov_db14 = assign31040_e48942_d_b14;
        var_qdov_db15 = assign31040_e48942_d_b15;
        var_qdov_db16 = assign31040_e48942_d_b16;
        var_qdov_db17 = assign31040_e48942_d_b17;
        var_qdov_db18 = assign31040_e48942_d_b18;
        var_qdov_db19 = assign31040_e48942_d_b19;
        var_qdov_db20 = assign31040_e48942_d_b20;
        var_qdov_db21 = assign31040_e48942_d_b21;
        var_qdov_db22 = assign31040_e48942_d_b22;
        var_qdov_db23 = assign31040_e48942_d_b23;
        var_qdov_db24 = assign31040_e48942_d_b24;
        var_qdov_db25 = assign31040_e48942_d_b25;
        var_qdov_db26 = assign31040_e48942_d_b26;
        var_qdov_db27 = assign31040_e48942_d_b27;
        var_qdov_db28 = assign31040_e48942_d_b28;
        var_qdov_db29 = assign31040_e48942_d_b29;
        var_qdov_db30 = assign31040_e48942_d_b30;
        var_qdov_db31 = assign31040_e48942_d_b31;
        var_qdov_db32 = assign31040_e48942_d_b32;
        var_qdov_db33 = assign31040_e48942_d_b33;
        var_qdov_db34 = assign31040_e48942_d_b34;
        var_qdov_db35 = assign31040_e48942_d_b35;
        var_qdov_db36 = assign31040_e48942_d_b36;
        var_qdov_db37 = assign31040_e48942_d_b37;
        var_qdov_db38 = assign31040_e48942_d_b38;
        var_qdov_db39 = assign31040_e48942_d_b39;
        var_qdov_db40 = assign31040_e48942_d_b40;
        var_qdov_db41 = assign31040_e48942_d_b41;
        var_qdov_db42 = assign31040_e48942_d_b42;
        var_qdov_db43 = assign31040_e48942_d_b43;
        var_qdov_db44 = assign31040_e48942_d_b44;
        var_qdov_db45 = assign31040_e48942_d_b45;
        var_qdov_db46 = assign31040_e48942_d_b46;
        var_qdov_db47 = assign31040_e48942_d_b47;
        var_qdov_db48 = assign31040_e48942_d_b48;
        var_qdov_db49 = assign31040_e48942_d_b49;
        var_qdov_db50 = assign31040_e48942_d_b50;
        var_qdov_db51 = assign31040_e48942_d_b51;
        var_qdov_db52 = assign31040_e48942_d_b52;
        var_qdov_db53 = assign31040_e48942_d_b53;
        var_qdov_db54 = assign31040_e48942_d_b54;
        var_qdov_db55 = 0.0;
        var_qdov_db56 = 0.0;

        let (assign31050_e48953, assign31050_e48953_d_n0, assign31050_e48953_d_n1, assign31050_e48953_d_n2, assign31050_e48953_d_n3, assign31050_e48953_d_n4, assign31050_e48953_d_n5, assign31050_e48953_d_n6, assign31050_e48953_d_n7, assign31050_e48953_d_n8, assign31050_e48953_d_n9, assign31050_e48953_d_n10, assign31050_e48953_d_n11, assign31050_e48953_d_n12, assign31050_e48953_d_n13, assign31050_e48953_d_n14, assign31050_e48953_d_n15, assign31050_e48953_d_n16, assign31050_e48953_d_n17, assign31050_e48953_d_n18, assign31050_e48953_d_n19, assign31050_e48953_d_n20, assign31050_e48953_d_n21, assign31050_e48953_d_n22, assign31050_e48953_d_b0, assign31050_e48953_d_b1, assign31050_e48953_d_b2, assign31050_e48953_d_b3, assign31050_e48953_d_b4, assign31050_e48953_d_b5, assign31050_e48953_d_b6, assign31050_e48953_d_b7, assign31050_e48953_d_b8, assign31050_e48953_d_b9, assign31050_e48953_d_b10, assign31050_e48953_d_b11, assign31050_e48953_d_b12, assign31050_e48953_d_b13, assign31050_e48953_d_b14, assign31050_e48953_d_b15, assign31050_e48953_d_b16, assign31050_e48953_d_b17, assign31050_e48953_d_b18, assign31050_e48953_d_b19, assign31050_e48953_d_b20, assign31050_e48953_d_b21, assign31050_e48953_d_b22, assign31050_e48953_d_b23, assign31050_e48953_d_b24, assign31050_e48953_d_b25, assign31050_e48953_d_b26, assign31050_e48953_d_b27, assign31050_e48953_d_b28, assign31050_e48953_d_b29, assign31050_e48953_d_b30, assign31050_e48953_d_b31, assign31050_e48953_d_b32, assign31050_e48953_d_b33, assign31050_e48953_d_b34, assign31050_e48953_d_b35, assign31050_e48953_d_b36, assign31050_e48953_d_b37, assign31050_e48953_d_b38, assign31050_e48953_d_b39, assign31050_e48953_d_b40, assign31050_e48953_d_b41, assign31050_e48953_d_b42, assign31050_e48953_d_b43, assign31050_e48953_d_b44, assign31050_e48953_d_b45, assign31050_e48953_d_b46, assign31050_e48953_d_b47, assign31050_e48953_d_b48, assign31050_e48953_d_b49, assign31050_e48953_d_b50, assign31050_e48953_d_b51, assign31050_e48953_d_b52, assign31050_e48953_d_b53, assign31050_e48953_d_b54,) = {
    if (var_guard524 == 0.0) {
        let assign31050_e48947: f64 = (p.p4 * p.p5);
        let assign31050_e48949: f64 = (assign31050_e48947 * p.p210);
        let assign31050_e48951: f64 = (assign31050_e48949 * (nv1 - nv2));
        (assign31050_e48951, 0.0, assign31050_e48949, (-assign31050_e48949), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qsov, var_qsov_dn0, var_qsov_dn1, var_qsov_dn2, var_qsov_dn3, var_qsov_dn4, var_qsov_dn5, var_qsov_dn6, var_qsov_dn7, var_qsov_dn8, var_qsov_dn9, var_qsov_dn10, var_qsov_dn11, var_qsov_dn12, var_qsov_dn13, var_qsov_dn14, var_qsov_dn15, var_qsov_dn16, var_qsov_dn17, var_qsov_dn18, var_qsov_dn19, var_qsov_dn20, var_qsov_dn21, var_qsov_dn22, var_qsov_db0, var_qsov_db1, var_qsov_db2, var_qsov_db3, var_qsov_db4, var_qsov_db5, var_qsov_db6, var_qsov_db7, var_qsov_db8, var_qsov_db9, var_qsov_db10, var_qsov_db11, var_qsov_db12, var_qsov_db13, var_qsov_db14, var_qsov_db15, var_qsov_db16, var_qsov_db17, var_qsov_db18, var_qsov_db19, var_qsov_db20, var_qsov_db21, var_qsov_db22, var_qsov_db23, var_qsov_db24, var_qsov_db25, var_qsov_db26, var_qsov_db27, var_qsov_db28, var_qsov_db29, var_qsov_db30, var_qsov_db31, var_qsov_db32, var_qsov_db33, var_qsov_db34, var_qsov_db35, var_qsov_db36, var_qsov_db37, var_qsov_db38, var_qsov_db39, var_qsov_db40, var_qsov_db41, var_qsov_db42, var_qsov_db43, var_qsov_db44, var_qsov_db45, var_qsov_db46, var_qsov_db47, var_qsov_db48, var_qsov_db49, var_qsov_db50, var_qsov_db51, var_qsov_db52, var_qsov_db53, var_qsov_db54,)
    }
};
        var_qsov = assign31050_e48953;
        var_qsov_dn0 = assign31050_e48953_d_n0;
        var_qsov_dn1 = assign31050_e48953_d_n1;
        var_qsov_dn2 = assign31050_e48953_d_n2;
        var_qsov_dn3 = assign31050_e48953_d_n3;
        var_qsov_dn4 = assign31050_e48953_d_n4;
        var_qsov_dn5 = assign31050_e48953_d_n5;
        var_qsov_dn6 = assign31050_e48953_d_n6;
        var_qsov_dn7 = assign31050_e48953_d_n7;
        var_qsov_dn8 = assign31050_e48953_d_n8;
        var_qsov_dn9 = assign31050_e48953_d_n9;
        var_qsov_dn10 = assign31050_e48953_d_n10;
        var_qsov_dn11 = assign31050_e48953_d_n11;
        var_qsov_dn12 = assign31050_e48953_d_n12;
        var_qsov_dn13 = assign31050_e48953_d_n13;
        var_qsov_dn14 = assign31050_e48953_d_n14;
        var_qsov_dn15 = assign31050_e48953_d_n15;
        var_qsov_dn16 = assign31050_e48953_d_n16;
        var_qsov_dn17 = assign31050_e48953_d_n17;
        var_qsov_dn18 = assign31050_e48953_d_n18;
        var_qsov_dn19 = assign31050_e48953_d_n19;
        var_qsov_dn20 = assign31050_e48953_d_n20;
        var_qsov_dn21 = assign31050_e48953_d_n21;
        var_qsov_dn22 = assign31050_e48953_d_n22;
        var_qsov_db0 = assign31050_e48953_d_b0;
        var_qsov_db1 = assign31050_e48953_d_b1;
        var_qsov_db2 = assign31050_e48953_d_b2;
        var_qsov_db3 = assign31050_e48953_d_b3;
        var_qsov_db4 = assign31050_e48953_d_b4;
        var_qsov_db5 = assign31050_e48953_d_b5;
        var_qsov_db6 = assign31050_e48953_d_b6;
        var_qsov_db7 = assign31050_e48953_d_b7;
        var_qsov_db8 = assign31050_e48953_d_b8;
        var_qsov_db9 = assign31050_e48953_d_b9;
        var_qsov_db10 = assign31050_e48953_d_b10;
        var_qsov_db11 = assign31050_e48953_d_b11;
        var_qsov_db12 = assign31050_e48953_d_b12;
        var_qsov_db13 = assign31050_e48953_d_b13;
        var_qsov_db14 = assign31050_e48953_d_b14;
        var_qsov_db15 = assign31050_e48953_d_b15;
        var_qsov_db16 = assign31050_e48953_d_b16;
        var_qsov_db17 = assign31050_e48953_d_b17;
        var_qsov_db18 = assign31050_e48953_d_b18;
        var_qsov_db19 = assign31050_e48953_d_b19;
        var_qsov_db20 = assign31050_e48953_d_b20;
        var_qsov_db21 = assign31050_e48953_d_b21;
        var_qsov_db22 = assign31050_e48953_d_b22;
        var_qsov_db23 = assign31050_e48953_d_b23;
        var_qsov_db24 = assign31050_e48953_d_b24;
        var_qsov_db25 = assign31050_e48953_d_b25;
        var_qsov_db26 = assign31050_e48953_d_b26;
        var_qsov_db27 = assign31050_e48953_d_b27;
        var_qsov_db28 = assign31050_e48953_d_b28;
        var_qsov_db29 = assign31050_e48953_d_b29;
        var_qsov_db30 = assign31050_e48953_d_b30;
        var_qsov_db31 = assign31050_e48953_d_b31;
        var_qsov_db32 = assign31050_e48953_d_b32;
        var_qsov_db33 = assign31050_e48953_d_b33;
        var_qsov_db34 = assign31050_e48953_d_b34;
        var_qsov_db35 = assign31050_e48953_d_b35;
        var_qsov_db36 = assign31050_e48953_d_b36;
        var_qsov_db37 = assign31050_e48953_d_b37;
        var_qsov_db38 = assign31050_e48953_d_b38;
        var_qsov_db39 = assign31050_e48953_d_b39;
        var_qsov_db40 = assign31050_e48953_d_b40;
        var_qsov_db41 = assign31050_e48953_d_b41;
        var_qsov_db42 = assign31050_e48953_d_b42;
        var_qsov_db43 = assign31050_e48953_d_b43;
        var_qsov_db44 = assign31050_e48953_d_b44;
        var_qsov_db45 = assign31050_e48953_d_b45;
        var_qsov_db46 = assign31050_e48953_d_b46;
        var_qsov_db47 = assign31050_e48953_d_b47;
        var_qsov_db48 = assign31050_e48953_d_b48;
        var_qsov_db49 = assign31050_e48953_d_b49;
        var_qsov_db50 = assign31050_e48953_d_b50;
        var_qsov_db51 = assign31050_e48953_d_b51;
        var_qsov_db52 = assign31050_e48953_d_b52;
        var_qsov_db53 = assign31050_e48953_d_b53;
        var_qsov_db54 = assign31050_e48953_d_b54;
        var_qsov_db55 = 0.0;
        var_qsov_db56 = 0.0;

        let (assign31060_e48969, assign31060_e48969_d_n0, assign31060_e48969_d_n1, assign31060_e48969_d_n2, assign31060_e48969_d_n3, assign31060_e48969_d_n4, assign31060_e48969_d_n5, assign31060_e48969_d_n6, assign31060_e48969_d_n7, assign31060_e48969_d_n8, assign31060_e48969_d_n9, assign31060_e48969_d_n10, assign31060_e48969_d_n11, assign31060_e48969_d_n12, assign31060_e48969_d_n13, assign31060_e48969_d_n14, assign31060_e48969_d_n15, assign31060_e48969_d_n16, assign31060_e48969_d_n17, assign31060_e48969_d_n18, assign31060_e48969_d_n19, assign31060_e48969_d_n20, assign31060_e48969_d_n21, assign31060_e48969_d_n22, assign31060_e48969_d_b0, assign31060_e48969_d_b1, assign31060_e48969_d_b2, assign31060_e48969_d_b3, assign31060_e48969_d_b4, assign31060_e48969_d_b5, assign31060_e48969_d_b6, assign31060_e48969_d_b7, assign31060_e48969_d_b8, assign31060_e48969_d_b9, assign31060_e48969_d_b10, assign31060_e48969_d_b11, assign31060_e48969_d_b12, assign31060_e48969_d_b13, assign31060_e48969_d_b14, assign31060_e48969_d_b15, assign31060_e48969_d_b16, assign31060_e48969_d_b17, assign31060_e48969_d_b18, assign31060_e48969_d_b19, assign31060_e48969_d_b20, assign31060_e48969_d_b21, assign31060_e48969_d_b22, assign31060_e48969_d_b23, assign31060_e48969_d_b24, assign31060_e48969_d_b25, assign31060_e48969_d_b26, assign31060_e48969_d_b27, assign31060_e48969_d_b28, assign31060_e48969_d_b29, assign31060_e48969_d_b30, assign31060_e48969_d_b31, assign31060_e48969_d_b32, assign31060_e48969_d_b33, assign31060_e48969_d_b34, assign31060_e48969_d_b35, assign31060_e48969_d_b36, assign31060_e48969_d_b37, assign31060_e48969_d_b38, assign31060_e48969_d_b39, assign31060_e48969_d_b40, assign31060_e48969_d_b41, assign31060_e48969_d_b42, assign31060_e48969_d_b43, assign31060_e48969_d_b44, assign31060_e48969_d_b45, assign31060_e48969_d_b46, assign31060_e48969_d_b47, assign31060_e48969_d_b48, assign31060_e48969_d_b49, assign31060_e48969_d_b50, assign31060_e48969_d_b51, assign31060_e48969_d_b52, assign31060_e48969_d_b53, assign31060_e48969_d_b54,) = {
    if (var_guard524 == 0.0) {
        let assign31060_e48958: f64 = ((nv0 - nv2) * p.p214);
        let assign31060_e48961: f64 = ((nv0 - nv2) * (nv0 - nv2));
        let assign31060_e48964: f64 = (p.p214 * p.p214);
        let assign31060_e48965: f64 = (assign31060_e48961 + assign31060_e48964);
        let assign31060_e48966: f64 = (assign31060_e48965).sqrt();
        let assign31060_e48967: f64 = (assign31060_e48958 / assign31060_e48966);
        (assign31060_e48967, (((p.p214 * assign31060_e48966) - (assign31060_e48958 * (((nv0 - nv2) + (nv0 - nv2)) / (2.0 * assign31060_e48966)))) / (assign31060_e48966 * assign31060_e48966)), 0.0, ((((-p.p214) * assign31060_e48966) - (assign31060_e48958 * (((-(nv0 - nv2)) + (-(nv0 - nv2))) / (2.0 * assign31060_e48966)))) / (assign31060_e48966 * assign31060_e48966)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vdseffcv, var_vdseffcv_dn0, var_vdseffcv_dn1, var_vdseffcv_dn2, var_vdseffcv_dn3, var_vdseffcv_dn4, var_vdseffcv_dn5, var_vdseffcv_dn6, var_vdseffcv_dn7, var_vdseffcv_dn8, var_vdseffcv_dn9, var_vdseffcv_dn10, var_vdseffcv_dn11, var_vdseffcv_dn12, var_vdseffcv_dn13, var_vdseffcv_dn14, var_vdseffcv_dn15, var_vdseffcv_dn16, var_vdseffcv_dn17, var_vdseffcv_dn18, var_vdseffcv_dn19, var_vdseffcv_dn20, var_vdseffcv_dn21, var_vdseffcv_dn22, var_vdseffcv_db0, var_vdseffcv_db1, var_vdseffcv_db2, var_vdseffcv_db3, var_vdseffcv_db4, var_vdseffcv_db5, var_vdseffcv_db6, var_vdseffcv_db7, var_vdseffcv_db8, var_vdseffcv_db9, var_vdseffcv_db10, var_vdseffcv_db11, var_vdseffcv_db12, var_vdseffcv_db13, var_vdseffcv_db14, var_vdseffcv_db15, var_vdseffcv_db16, var_vdseffcv_db17, var_vdseffcv_db18, var_vdseffcv_db19, var_vdseffcv_db20, var_vdseffcv_db21, var_vdseffcv_db22, var_vdseffcv_db23, var_vdseffcv_db24, var_vdseffcv_db25, var_vdseffcv_db26, var_vdseffcv_db27, var_vdseffcv_db28, var_vdseffcv_db29, var_vdseffcv_db30, var_vdseffcv_db31, var_vdseffcv_db32, var_vdseffcv_db33, var_vdseffcv_db34, var_vdseffcv_db35, var_vdseffcv_db36, var_vdseffcv_db37, var_vdseffcv_db38, var_vdseffcv_db39, var_vdseffcv_db40, var_vdseffcv_db41, var_vdseffcv_db42, var_vdseffcv_db43, var_vdseffcv_db44, var_vdseffcv_db45, var_vdseffcv_db46, var_vdseffcv_db47, var_vdseffcv_db48, var_vdseffcv_db49, var_vdseffcv_db50, var_vdseffcv_db51, var_vdseffcv_db52, var_vdseffcv_db53, var_vdseffcv_db54,)
    }
};
        var_vdseffcv = assign31060_e48969;
        var_vdseffcv_dn0 = assign31060_e48969_d_n0;
        var_vdseffcv_dn1 = assign31060_e48969_d_n1;
        var_vdseffcv_dn2 = assign31060_e48969_d_n2;
        var_vdseffcv_dn3 = assign31060_e48969_d_n3;
        var_vdseffcv_dn4 = assign31060_e48969_d_n4;
        var_vdseffcv_dn5 = assign31060_e48969_d_n5;
        var_vdseffcv_dn6 = assign31060_e48969_d_n6;
        var_vdseffcv_dn7 = assign31060_e48969_d_n7;
        var_vdseffcv_dn8 = assign31060_e48969_d_n8;
        var_vdseffcv_dn9 = assign31060_e48969_d_n9;
        var_vdseffcv_dn10 = assign31060_e48969_d_n10;
        var_vdseffcv_dn11 = assign31060_e48969_d_n11;
        var_vdseffcv_dn12 = assign31060_e48969_d_n12;
        var_vdseffcv_dn13 = assign31060_e48969_d_n13;
        var_vdseffcv_dn14 = assign31060_e48969_d_n14;
        var_vdseffcv_dn15 = assign31060_e48969_d_n15;
        var_vdseffcv_dn16 = assign31060_e48969_d_n16;
        var_vdseffcv_dn17 = assign31060_e48969_d_n17;
        var_vdseffcv_dn18 = assign31060_e48969_d_n18;
        var_vdseffcv_dn19 = assign31060_e48969_d_n19;
        var_vdseffcv_dn20 = assign31060_e48969_d_n20;
        var_vdseffcv_dn21 = assign31060_e48969_d_n21;
        var_vdseffcv_dn22 = assign31060_e48969_d_n22;
        var_vdseffcv_db0 = assign31060_e48969_d_b0;
        var_vdseffcv_db1 = assign31060_e48969_d_b1;
        var_vdseffcv_db2 = assign31060_e48969_d_b2;
        var_vdseffcv_db3 = assign31060_e48969_d_b3;
        var_vdseffcv_db4 = assign31060_e48969_d_b4;
        var_vdseffcv_db5 = assign31060_e48969_d_b5;
        var_vdseffcv_db6 = assign31060_e48969_d_b6;
        var_vdseffcv_db7 = assign31060_e48969_d_b7;
        var_vdseffcv_db8 = assign31060_e48969_d_b8;
        var_vdseffcv_db9 = assign31060_e48969_d_b9;
        var_vdseffcv_db10 = assign31060_e48969_d_b10;
        var_vdseffcv_db11 = assign31060_e48969_d_b11;
        var_vdseffcv_db12 = assign31060_e48969_d_b12;
        var_vdseffcv_db13 = assign31060_e48969_d_b13;
        var_vdseffcv_db14 = assign31060_e48969_d_b14;
        var_vdseffcv_db15 = assign31060_e48969_d_b15;
        var_vdseffcv_db16 = assign31060_e48969_d_b16;
        var_vdseffcv_db17 = assign31060_e48969_d_b17;
        var_vdseffcv_db18 = assign31060_e48969_d_b18;
        var_vdseffcv_db19 = assign31060_e48969_d_b19;
        var_vdseffcv_db20 = assign31060_e48969_d_b20;
        var_vdseffcv_db21 = assign31060_e48969_d_b21;
        var_vdseffcv_db22 = assign31060_e48969_d_b22;
        var_vdseffcv_db23 = assign31060_e48969_d_b23;
        var_vdseffcv_db24 = assign31060_e48969_d_b24;
        var_vdseffcv_db25 = assign31060_e48969_d_b25;
        var_vdseffcv_db26 = assign31060_e48969_d_b26;
        var_vdseffcv_db27 = assign31060_e48969_d_b27;
        var_vdseffcv_db28 = assign31060_e48969_d_b28;
        var_vdseffcv_db29 = assign31060_e48969_d_b29;
        var_vdseffcv_db30 = assign31060_e48969_d_b30;
        var_vdseffcv_db31 = assign31060_e48969_d_b31;
        var_vdseffcv_db32 = assign31060_e48969_d_b32;
        var_vdseffcv_db33 = assign31060_e48969_d_b33;
        var_vdseffcv_db34 = assign31060_e48969_d_b34;
        var_vdseffcv_db35 = assign31060_e48969_d_b35;
        var_vdseffcv_db36 = assign31060_e48969_d_b36;
        var_vdseffcv_db37 = assign31060_e48969_d_b37;
        var_vdseffcv_db38 = assign31060_e48969_d_b38;
        var_vdseffcv_db39 = assign31060_e48969_d_b39;
        var_vdseffcv_db40 = assign31060_e48969_d_b40;
        var_vdseffcv_db41 = assign31060_e48969_d_b41;
        var_vdseffcv_db42 = assign31060_e48969_d_b42;
        var_vdseffcv_db43 = assign31060_e48969_d_b43;
        var_vdseffcv_db44 = assign31060_e48969_d_b44;
        var_vdseffcv_db45 = assign31060_e48969_d_b45;
        var_vdseffcv_db46 = assign31060_e48969_d_b46;
        var_vdseffcv_db47 = assign31060_e48969_d_b47;
        var_vdseffcv_db48 = assign31060_e48969_d_b48;
        var_vdseffcv_db49 = assign31060_e48969_d_b49;
        var_vdseffcv_db50 = assign31060_e48969_d_b50;
        var_vdseffcv_db51 = assign31060_e48969_d_b51;
        var_vdseffcv_db52 = assign31060_e48969_d_b52;
        var_vdseffcv_db53 = assign31060_e48969_d_b53;
        var_vdseffcv_db54 = assign31060_e48969_d_b54;
        var_vdseffcv_db55 = 0.0;
        var_vdseffcv_db56 = 0.0;

        let (assign31070_e48980,) = {
    if (var_guard524 == 0.0) {
        let assign31070_e48976: f64 = (2.0 * p.p214);
        let assign31070_e48977: f64 = (p.p211 / assign31070_e48976);
        let assign31070_e48978: f64 = (p.p213).min(assign31070_e48977);
        (assign31070_e48978,)
    } else {
        (var_cgdl_l,)
    }
};
        var_cgdl_l = assign31070_e48980;

        let (assign31080_e48997, assign31080_e48997_d_n0, assign31080_e48997_d_n1, assign31080_e48997_d_n2, assign31080_e48997_d_n3, assign31080_e48997_d_n4, assign31080_e48997_d_n5, assign31080_e48997_d_n6, assign31080_e48997_d_n7, assign31080_e48997_d_n8, assign31080_e48997_d_n9, assign31080_e48997_d_n10, assign31080_e48997_d_n11, assign31080_e48997_d_n12, assign31080_e48997_d_n13, assign31080_e48997_d_n14, assign31080_e48997_d_n15, assign31080_e48997_d_n16, assign31080_e48997_d_n17, assign31080_e48997_d_n18, assign31080_e48997_d_n19, assign31080_e48997_d_n20, assign31080_e48997_d_n21, assign31080_e48997_d_n22, assign31080_e48997_d_b0, assign31080_e48997_d_b1, assign31080_e48997_d_b2, assign31080_e48997_d_b3, assign31080_e48997_d_b4, assign31080_e48997_d_b5, assign31080_e48997_d_b6, assign31080_e48997_d_b7, assign31080_e48997_d_b8, assign31080_e48997_d_b9, assign31080_e48997_d_b10, assign31080_e48997_d_b11, assign31080_e48997_d_b12, assign31080_e48997_d_b13, assign31080_e48997_d_b14, assign31080_e48997_d_b15, assign31080_e48997_d_b16, assign31080_e48997_d_b17, assign31080_e48997_d_b18, assign31080_e48997_d_b19, assign31080_e48997_d_b20, assign31080_e48997_d_b21, assign31080_e48997_d_b22, assign31080_e48997_d_b23, assign31080_e48997_d_b24, assign31080_e48997_d_b25, assign31080_e48997_d_b26, assign31080_e48997_d_b27, assign31080_e48997_d_b28, assign31080_e48997_d_b29, assign31080_e48997_d_b30, assign31080_e48997_d_b31, assign31080_e48997_d_b32, assign31080_e48997_d_b33, assign31080_e48997_d_b34, assign31080_e48997_d_b35, assign31080_e48997_d_b36, assign31080_e48997_d_b37, assign31080_e48997_d_b38, assign31080_e48997_d_b39, assign31080_e48997_d_b40, assign31080_e48997_d_b41, assign31080_e48997_d_b42, assign31080_e48997_d_b43, assign31080_e48997_d_b44, assign31080_e48997_d_b45, assign31080_e48997_d_b46, assign31080_e48997_d_b47, assign31080_e48997_d_b48, assign31080_e48997_d_b49, assign31080_e48997_d_b50, assign31080_e48997_d_b51, assign31080_e48997_d_b52, assign31080_e48997_d_b53, assign31080_e48997_d_b54,) = {
    if (var_guard524 == 0.0) {
        let assign31080_e48985: f64 = (p.p4 * p.p5);
        let assign31080_e48987: f64 = (assign31080_e48985 * p.p211);
        let assign31080_e48990: f64 = (p.p4 * p.p5);
        let assign31080_e48992: f64 = (assign31080_e48990 * var_cgdl_l);
        let assign31080_e48994: f64 = (assign31080_e48992 * var_vdseffcv);
        let assign31080_e48995: f64 = (assign31080_e48987 - assign31080_e48994);
        (assign31080_e48995, (-(assign31080_e48992 * var_vdseffcv_dn0)), (-(assign31080_e48992 * var_vdseffcv_dn1)), (-(assign31080_e48992 * var_vdseffcv_dn2)), (-(assign31080_e48992 * var_vdseffcv_dn3)), (-(assign31080_e48992 * var_vdseffcv_dn4)), (-(assign31080_e48992 * var_vdseffcv_dn5)), (-(assign31080_e48992 * var_vdseffcv_dn6)), (-(assign31080_e48992 * var_vdseffcv_dn7)), (-(assign31080_e48992 * var_vdseffcv_dn8)), (-(assign31080_e48992 * var_vdseffcv_dn9)), (-(assign31080_e48992 * var_vdseffcv_dn10)), (-(assign31080_e48992 * var_vdseffcv_dn11)), (-(assign31080_e48992 * var_vdseffcv_dn12)), (-(assign31080_e48992 * var_vdseffcv_dn13)), (-(assign31080_e48992 * var_vdseffcv_dn14)), (-(assign31080_e48992 * var_vdseffcv_dn15)), (-(assign31080_e48992 * var_vdseffcv_dn16)), (-(assign31080_e48992 * var_vdseffcv_dn17)), (-(assign31080_e48992 * var_vdseffcv_dn18)), (-(assign31080_e48992 * var_vdseffcv_dn19)), (-(assign31080_e48992 * var_vdseffcv_dn20)), (-(assign31080_e48992 * var_vdseffcv_dn21)), (-(assign31080_e48992 * var_vdseffcv_dn22)), (-(assign31080_e48992 * var_vdseffcv_db0)), (-(assign31080_e48992 * var_vdseffcv_db1)), (-(assign31080_e48992 * var_vdseffcv_db2)), (-(assign31080_e48992 * var_vdseffcv_db3)), (-(assign31080_e48992 * var_vdseffcv_db4)), (-(assign31080_e48992 * var_vdseffcv_db5)), (-(assign31080_e48992 * var_vdseffcv_db6)), (-(assign31080_e48992 * var_vdseffcv_db7)), (-(assign31080_e48992 * var_vdseffcv_db8)), (-(assign31080_e48992 * var_vdseffcv_db9)), (-(assign31080_e48992 * var_vdseffcv_db10)), (-(assign31080_e48992 * var_vdseffcv_db11)), (-(assign31080_e48992 * var_vdseffcv_db12)), (-(assign31080_e48992 * var_vdseffcv_db13)), (-(assign31080_e48992 * var_vdseffcv_db14)), (-(assign31080_e48992 * var_vdseffcv_db15)), (-(assign31080_e48992 * var_vdseffcv_db16)), (-(assign31080_e48992 * var_vdseffcv_db17)), (-(assign31080_e48992 * var_vdseffcv_db18)), (-(assign31080_e48992 * var_vdseffcv_db19)), (-(assign31080_e48992 * var_vdseffcv_db20)), (-(assign31080_e48992 * var_vdseffcv_db21)), (-(assign31080_e48992 * var_vdseffcv_db22)), (-(assign31080_e48992 * var_vdseffcv_db23)), (-(assign31080_e48992 * var_vdseffcv_db24)), (-(assign31080_e48992 * var_vdseffcv_db25)), (-(assign31080_e48992 * var_vdseffcv_db26)), (-(assign31080_e48992 * var_vdseffcv_db27)), (-(assign31080_e48992 * var_vdseffcv_db28)), (-(assign31080_e48992 * var_vdseffcv_db29)), (-(assign31080_e48992 * var_vdseffcv_db30)), (-(assign31080_e48992 * var_vdseffcv_db31)), (-(assign31080_e48992 * var_vdseffcv_db32)), (-(assign31080_e48992 * var_vdseffcv_db33)), (-(assign31080_e48992 * var_vdseffcv_db34)), (-(assign31080_e48992 * var_vdseffcv_db35)), (-(assign31080_e48992 * var_vdseffcv_db36)), (-(assign31080_e48992 * var_vdseffcv_db37)), (-(assign31080_e48992 * var_vdseffcv_db38)), (-(assign31080_e48992 * var_vdseffcv_db39)), (-(assign31080_e48992 * var_vdseffcv_db40)), (-(assign31080_e48992 * var_vdseffcv_db41)), (-(assign31080_e48992 * var_vdseffcv_db42)), (-(assign31080_e48992 * var_vdseffcv_db43)), (-(assign31080_e48992 * var_vdseffcv_db44)), (-(assign31080_e48992 * var_vdseffcv_db45)), (-(assign31080_e48992 * var_vdseffcv_db46)), (-(assign31080_e48992 * var_vdseffcv_db47)), (-(assign31080_e48992 * var_vdseffcv_db48)), (-(assign31080_e48992 * var_vdseffcv_db49)), (-(assign31080_e48992 * var_vdseffcv_db50)), (-(assign31080_e48992 * var_vdseffcv_db51)), (-(assign31080_e48992 * var_vdseffcv_db52)), (-(assign31080_e48992 * var_vdseffcv_db53)), (-(assign31080_e48992 * var_vdseffcv_db54)),)
    } else {
        (var_cgdvar, var_cgdvar_dn0, var_cgdvar_dn1, var_cgdvar_dn2, var_cgdvar_dn3, var_cgdvar_dn4, var_cgdvar_dn5, var_cgdvar_dn6, var_cgdvar_dn7, var_cgdvar_dn8, var_cgdvar_dn9, var_cgdvar_dn10, var_cgdvar_dn11, var_cgdvar_dn12, var_cgdvar_dn13, var_cgdvar_dn14, var_cgdvar_dn15, var_cgdvar_dn16, var_cgdvar_dn17, var_cgdvar_dn18, var_cgdvar_dn19, var_cgdvar_dn20, var_cgdvar_dn21, var_cgdvar_dn22, var_cgdvar_db0, var_cgdvar_db1, var_cgdvar_db2, var_cgdvar_db3, var_cgdvar_db4, var_cgdvar_db5, var_cgdvar_db6, var_cgdvar_db7, var_cgdvar_db8, var_cgdvar_db9, var_cgdvar_db10, var_cgdvar_db11, var_cgdvar_db12, var_cgdvar_db13, var_cgdvar_db14, var_cgdvar_db15, var_cgdvar_db16, var_cgdvar_db17, var_cgdvar_db18, var_cgdvar_db19, var_cgdvar_db20, var_cgdvar_db21, var_cgdvar_db22, var_cgdvar_db23, var_cgdvar_db24, var_cgdvar_db25, var_cgdvar_db26, var_cgdvar_db27, var_cgdvar_db28, var_cgdvar_db29, var_cgdvar_db30, var_cgdvar_db31, var_cgdvar_db32, var_cgdvar_db33, var_cgdvar_db34, var_cgdvar_db35, var_cgdvar_db36, var_cgdvar_db37, var_cgdvar_db38, var_cgdvar_db39, var_cgdvar_db40, var_cgdvar_db41, var_cgdvar_db42, var_cgdvar_db43, var_cgdvar_db44, var_cgdvar_db45, var_cgdvar_db46, var_cgdvar_db47, var_cgdvar_db48, var_cgdvar_db49, var_cgdvar_db50, var_cgdvar_db51, var_cgdvar_db52, var_cgdvar_db53, var_cgdvar_db54,)
    }
};
        var_cgdvar = assign31080_e48997;
        var_cgdvar_dn0 = assign31080_e48997_d_n0;
        var_cgdvar_dn1 = assign31080_e48997_d_n1;
        var_cgdvar_dn2 = assign31080_e48997_d_n2;
        var_cgdvar_dn3 = assign31080_e48997_d_n3;
        var_cgdvar_dn4 = assign31080_e48997_d_n4;
        var_cgdvar_dn5 = assign31080_e48997_d_n5;
        var_cgdvar_dn6 = assign31080_e48997_d_n6;
        var_cgdvar_dn7 = assign31080_e48997_d_n7;
        var_cgdvar_dn8 = assign31080_e48997_d_n8;
        var_cgdvar_dn9 = assign31080_e48997_d_n9;
        var_cgdvar_dn10 = assign31080_e48997_d_n10;
        var_cgdvar_dn11 = assign31080_e48997_d_n11;
        var_cgdvar_dn12 = assign31080_e48997_d_n12;
        var_cgdvar_dn13 = assign31080_e48997_d_n13;
        var_cgdvar_dn14 = assign31080_e48997_d_n14;
        var_cgdvar_dn15 = assign31080_e48997_d_n15;
        var_cgdvar_dn16 = assign31080_e48997_d_n16;
        var_cgdvar_dn17 = assign31080_e48997_d_n17;
        var_cgdvar_dn18 = assign31080_e48997_d_n18;
        var_cgdvar_dn19 = assign31080_e48997_d_n19;
        var_cgdvar_dn20 = assign31080_e48997_d_n20;
        var_cgdvar_dn21 = assign31080_e48997_d_n21;
        var_cgdvar_dn22 = assign31080_e48997_d_n22;
        var_cgdvar_db0 = assign31080_e48997_d_b0;
        var_cgdvar_db1 = assign31080_e48997_d_b1;
        var_cgdvar_db2 = assign31080_e48997_d_b2;
        var_cgdvar_db3 = assign31080_e48997_d_b3;
        var_cgdvar_db4 = assign31080_e48997_d_b4;
        var_cgdvar_db5 = assign31080_e48997_d_b5;
        var_cgdvar_db6 = assign31080_e48997_d_b6;
        var_cgdvar_db7 = assign31080_e48997_d_b7;
        var_cgdvar_db8 = assign31080_e48997_d_b8;
        var_cgdvar_db9 = assign31080_e48997_d_b9;
        var_cgdvar_db10 = assign31080_e48997_d_b10;
        var_cgdvar_db11 = assign31080_e48997_d_b11;
        var_cgdvar_db12 = assign31080_e48997_d_b12;
        var_cgdvar_db13 = assign31080_e48997_d_b13;
        var_cgdvar_db14 = assign31080_e48997_d_b14;
        var_cgdvar_db15 = assign31080_e48997_d_b15;
        var_cgdvar_db16 = assign31080_e48997_d_b16;
        var_cgdvar_db17 = assign31080_e48997_d_b17;
        var_cgdvar_db18 = assign31080_e48997_d_b18;
        var_cgdvar_db19 = assign31080_e48997_d_b19;
        var_cgdvar_db20 = assign31080_e48997_d_b20;
        var_cgdvar_db21 = assign31080_e48997_d_b21;
        var_cgdvar_db22 = assign31080_e48997_d_b22;
        var_cgdvar_db23 = assign31080_e48997_d_b23;
        var_cgdvar_db24 = assign31080_e48997_d_b24;
        var_cgdvar_db25 = assign31080_e48997_d_b25;
        var_cgdvar_db26 = assign31080_e48997_d_b26;
        var_cgdvar_db27 = assign31080_e48997_d_b27;
        var_cgdvar_db28 = assign31080_e48997_d_b28;
        var_cgdvar_db29 = assign31080_e48997_d_b29;
        var_cgdvar_db30 = assign31080_e48997_d_b30;
        var_cgdvar_db31 = assign31080_e48997_d_b31;
        var_cgdvar_db32 = assign31080_e48997_d_b32;
        var_cgdvar_db33 = assign31080_e48997_d_b33;
        var_cgdvar_db34 = assign31080_e48997_d_b34;
        var_cgdvar_db35 = assign31080_e48997_d_b35;
        var_cgdvar_db36 = assign31080_e48997_d_b36;
        var_cgdvar_db37 = assign31080_e48997_d_b37;
        var_cgdvar_db38 = assign31080_e48997_d_b38;
        var_cgdvar_db39 = assign31080_e48997_d_b39;
        var_cgdvar_db40 = assign31080_e48997_d_b40;
        var_cgdvar_db41 = assign31080_e48997_d_b41;
        var_cgdvar_db42 = assign31080_e48997_d_b42;
        var_cgdvar_db43 = assign31080_e48997_d_b43;
        var_cgdvar_db44 = assign31080_e48997_d_b44;
        var_cgdvar_db45 = assign31080_e48997_d_b45;
        var_cgdvar_db46 = assign31080_e48997_d_b46;
        var_cgdvar_db47 = assign31080_e48997_d_b47;
        var_cgdvar_db48 = assign31080_e48997_d_b48;
        var_cgdvar_db49 = assign31080_e48997_d_b49;
        var_cgdvar_db50 = assign31080_e48997_d_b50;
        var_cgdvar_db51 = assign31080_e48997_d_b51;
        var_cgdvar_db52 = assign31080_e48997_d_b52;
        var_cgdvar_db53 = assign31080_e48997_d_b53;
        var_cgdvar_db54 = assign31080_e48997_d_b54;
        var_cgdvar_db55 = 0.0;
        var_cgdvar_db56 = 0.0;

        *var_cgdl_l_slot = var_cgdl_l;
        *var_cgdvar_slot = var_cgdvar;
        *var_cgdvar_db0_slot = var_cgdvar_db0;
        *var_cgdvar_db1_slot = var_cgdvar_db1;
        *var_cgdvar_db10_slot = var_cgdvar_db10;
        *var_cgdvar_db11_slot = var_cgdvar_db11;
        *var_cgdvar_db12_slot = var_cgdvar_db12;
        *var_cgdvar_db13_slot = var_cgdvar_db13;
        *var_cgdvar_db14_slot = var_cgdvar_db14;
        *var_cgdvar_db15_slot = var_cgdvar_db15;
        *var_cgdvar_db16_slot = var_cgdvar_db16;
        *var_cgdvar_db17_slot = var_cgdvar_db17;
        *var_cgdvar_db18_slot = var_cgdvar_db18;
        *var_cgdvar_db19_slot = var_cgdvar_db19;
        *var_cgdvar_db2_slot = var_cgdvar_db2;
        *var_cgdvar_db20_slot = var_cgdvar_db20;
        *var_cgdvar_db21_slot = var_cgdvar_db21;
        *var_cgdvar_db22_slot = var_cgdvar_db22;
        *var_cgdvar_db23_slot = var_cgdvar_db23;
        *var_cgdvar_db24_slot = var_cgdvar_db24;
        *var_cgdvar_db25_slot = var_cgdvar_db25;
        *var_cgdvar_db26_slot = var_cgdvar_db26;
        *var_cgdvar_db27_slot = var_cgdvar_db27;
        *var_cgdvar_db28_slot = var_cgdvar_db28;
        *var_cgdvar_db29_slot = var_cgdvar_db29;
        *var_cgdvar_db3_slot = var_cgdvar_db3;
        *var_cgdvar_db30_slot = var_cgdvar_db30;
        *var_cgdvar_db31_slot = var_cgdvar_db31;
        *var_cgdvar_db32_slot = var_cgdvar_db32;
        *var_cgdvar_db33_slot = var_cgdvar_db33;
        *var_cgdvar_db34_slot = var_cgdvar_db34;
        *var_cgdvar_db35_slot = var_cgdvar_db35;
        *var_cgdvar_db36_slot = var_cgdvar_db36;
        *var_cgdvar_db37_slot = var_cgdvar_db37;
        *var_cgdvar_db38_slot = var_cgdvar_db38;
        *var_cgdvar_db39_slot = var_cgdvar_db39;
        *var_cgdvar_db4_slot = var_cgdvar_db4;
        *var_cgdvar_db40_slot = var_cgdvar_db40;
        *var_cgdvar_db41_slot = var_cgdvar_db41;
        *var_cgdvar_db42_slot = var_cgdvar_db42;
        *var_cgdvar_db43_slot = var_cgdvar_db43;
        *var_cgdvar_db44_slot = var_cgdvar_db44;
        *var_cgdvar_db45_slot = var_cgdvar_db45;
        *var_cgdvar_db46_slot = var_cgdvar_db46;
        *var_cgdvar_db47_slot = var_cgdvar_db47;
        *var_cgdvar_db48_slot = var_cgdvar_db48;
        *var_cgdvar_db49_slot = var_cgdvar_db49;
        *var_cgdvar_db5_slot = var_cgdvar_db5;
        *var_cgdvar_db50_slot = var_cgdvar_db50;
        *var_cgdvar_db51_slot = var_cgdvar_db51;
        *var_cgdvar_db52_slot = var_cgdvar_db52;
        *var_cgdvar_db53_slot = var_cgdvar_db53;
        *var_cgdvar_db54_slot = var_cgdvar_db54;
        *var_cgdvar_db55_slot = var_cgdvar_db55;
        *var_cgdvar_db56_slot = var_cgdvar_db56;
        *var_cgdvar_db6_slot = var_cgdvar_db6;
        *var_cgdvar_db7_slot = var_cgdvar_db7;
        *var_cgdvar_db8_slot = var_cgdvar_db8;
        *var_cgdvar_db9_slot = var_cgdvar_db9;
        *var_cgdvar_dn0_slot = var_cgdvar_dn0;
        *var_cgdvar_dn1_slot = var_cgdvar_dn1;
        *var_cgdvar_dn10_slot = var_cgdvar_dn10;
        *var_cgdvar_dn11_slot = var_cgdvar_dn11;
        *var_cgdvar_dn12_slot = var_cgdvar_dn12;
        *var_cgdvar_dn13_slot = var_cgdvar_dn13;
        *var_cgdvar_dn14_slot = var_cgdvar_dn14;
        *var_cgdvar_dn15_slot = var_cgdvar_dn15;
        *var_cgdvar_dn16_slot = var_cgdvar_dn16;
        *var_cgdvar_dn17_slot = var_cgdvar_dn17;
        *var_cgdvar_dn18_slot = var_cgdvar_dn18;
        *var_cgdvar_dn19_slot = var_cgdvar_dn19;
        *var_cgdvar_dn2_slot = var_cgdvar_dn2;
        *var_cgdvar_dn20_slot = var_cgdvar_dn20;
        *var_cgdvar_dn21_slot = var_cgdvar_dn21;
        *var_cgdvar_dn22_slot = var_cgdvar_dn22;
        *var_cgdvar_dn3_slot = var_cgdvar_dn3;
        *var_cgdvar_dn4_slot = var_cgdvar_dn4;
        *var_cgdvar_dn5_slot = var_cgdvar_dn5;
        *var_cgdvar_dn6_slot = var_cgdvar_dn6;
        *var_cgdvar_dn7_slot = var_cgdvar_dn7;
        *var_cgdvar_dn8_slot = var_cgdvar_dn8;
        *var_cgdvar_dn9_slot = var_cgdvar_dn9;
        *var_qdov_slot = var_qdov;
        *var_qdov_db0_slot = var_qdov_db0;
        *var_qdov_db1_slot = var_qdov_db1;
        *var_qdov_db10_slot = var_qdov_db10;
        *var_qdov_db11_slot = var_qdov_db11;
        *var_qdov_db12_slot = var_qdov_db12;
        *var_qdov_db13_slot = var_qdov_db13;
        *var_qdov_db14_slot = var_qdov_db14;
        *var_qdov_db15_slot = var_qdov_db15;
        *var_qdov_db16_slot = var_qdov_db16;
        *var_qdov_db17_slot = var_qdov_db17;
        *var_qdov_db18_slot = var_qdov_db18;
        *var_qdov_db19_slot = var_qdov_db19;
        *var_qdov_db2_slot = var_qdov_db2;
        *var_qdov_db20_slot = var_qdov_db20;
        *var_qdov_db21_slot = var_qdov_db21;
        *var_qdov_db22_slot = var_qdov_db22;
        *var_qdov_db23_slot = var_qdov_db23;
        *var_qdov_db24_slot = var_qdov_db24;
        *var_qdov_db25_slot = var_qdov_db25;
        *var_qdov_db26_slot = var_qdov_db26;
        *var_qdov_db27_slot = var_qdov_db27;
        *var_qdov_db28_slot = var_qdov_db28;
        *var_qdov_db29_slot = var_qdov_db29;
        *var_qdov_db3_slot = var_qdov_db3;
        *var_qdov_db30_slot = var_qdov_db30;
        *var_qdov_db31_slot = var_qdov_db31;
        *var_qdov_db32_slot = var_qdov_db32;
        *var_qdov_db33_slot = var_qdov_db33;
        *var_qdov_db34_slot = var_qdov_db34;
        *var_qdov_db35_slot = var_qdov_db35;
        *var_qdov_db36_slot = var_qdov_db36;
        *var_qdov_db37_slot = var_qdov_db37;
        *var_qdov_db38_slot = var_qdov_db38;
        *var_qdov_db39_slot = var_qdov_db39;
        *var_qdov_db4_slot = var_qdov_db4;
        *var_qdov_db40_slot = var_qdov_db40;
        *var_qdov_db41_slot = var_qdov_db41;
        *var_qdov_db42_slot = var_qdov_db42;
        *var_qdov_db43_slot = var_qdov_db43;
        *var_qdov_db44_slot = var_qdov_db44;
        *var_qdov_db45_slot = var_qdov_db45;
        *var_qdov_db46_slot = var_qdov_db46;
        *var_qdov_db47_slot = var_qdov_db47;
        *var_qdov_db48_slot = var_qdov_db48;
        *var_qdov_db49_slot = var_qdov_db49;
        *var_qdov_db5_slot = var_qdov_db5;
        *var_qdov_db50_slot = var_qdov_db50;
        *var_qdov_db51_slot = var_qdov_db51;
        *var_qdov_db52_slot = var_qdov_db52;
        *var_qdov_db53_slot = var_qdov_db53;
        *var_qdov_db54_slot = var_qdov_db54;
        *var_qdov_db55_slot = var_qdov_db55;
        *var_qdov_db56_slot = var_qdov_db56;
        *var_qdov_db6_slot = var_qdov_db6;
        *var_qdov_db7_slot = var_qdov_db7;
        *var_qdov_db8_slot = var_qdov_db8;
        *var_qdov_db9_slot = var_qdov_db9;
        *var_qdov_dn0_slot = var_qdov_dn0;
        *var_qdov_dn1_slot = var_qdov_dn1;
        *var_qdov_dn10_slot = var_qdov_dn10;
        *var_qdov_dn11_slot = var_qdov_dn11;
        *var_qdov_dn12_slot = var_qdov_dn12;
        *var_qdov_dn13_slot = var_qdov_dn13;
        *var_qdov_dn14_slot = var_qdov_dn14;
        *var_qdov_dn15_slot = var_qdov_dn15;
        *var_qdov_dn16_slot = var_qdov_dn16;
        *var_qdov_dn17_slot = var_qdov_dn17;
        *var_qdov_dn18_slot = var_qdov_dn18;
        *var_qdov_dn19_slot = var_qdov_dn19;
        *var_qdov_dn2_slot = var_qdov_dn2;
        *var_qdov_dn20_slot = var_qdov_dn20;
        *var_qdov_dn21_slot = var_qdov_dn21;
        *var_qdov_dn22_slot = var_qdov_dn22;
        *var_qdov_dn3_slot = var_qdov_dn3;
        *var_qdov_dn4_slot = var_qdov_dn4;
        *var_qdov_dn5_slot = var_qdov_dn5;
        *var_qdov_dn6_slot = var_qdov_dn6;
        *var_qdov_dn7_slot = var_qdov_dn7;
        *var_qdov_dn8_slot = var_qdov_dn8;
        *var_qdov_dn9_slot = var_qdov_dn9;
        *var_qsov_slot = var_qsov;
        *var_qsov_db0_slot = var_qsov_db0;
        *var_qsov_db1_slot = var_qsov_db1;
        *var_qsov_db10_slot = var_qsov_db10;
        *var_qsov_db11_slot = var_qsov_db11;
        *var_qsov_db12_slot = var_qsov_db12;
        *var_qsov_db13_slot = var_qsov_db13;
        *var_qsov_db14_slot = var_qsov_db14;
        *var_qsov_db15_slot = var_qsov_db15;
        *var_qsov_db16_slot = var_qsov_db16;
        *var_qsov_db17_slot = var_qsov_db17;
        *var_qsov_db18_slot = var_qsov_db18;
        *var_qsov_db19_slot = var_qsov_db19;
        *var_qsov_db2_slot = var_qsov_db2;
        *var_qsov_db20_slot = var_qsov_db20;
        *var_qsov_db21_slot = var_qsov_db21;
        *var_qsov_db22_slot = var_qsov_db22;
        *var_qsov_db23_slot = var_qsov_db23;
        *var_qsov_db24_slot = var_qsov_db24;
        *var_qsov_db25_slot = var_qsov_db25;
        *var_qsov_db26_slot = var_qsov_db26;
        *var_qsov_db27_slot = var_qsov_db27;
        *var_qsov_db28_slot = var_qsov_db28;
        *var_qsov_db29_slot = var_qsov_db29;
        *var_qsov_db3_slot = var_qsov_db3;
        *var_qsov_db30_slot = var_qsov_db30;
        *var_qsov_db31_slot = var_qsov_db31;
        *var_qsov_db32_slot = var_qsov_db32;
        *var_qsov_db33_slot = var_qsov_db33;
        *var_qsov_db34_slot = var_qsov_db34;
        *var_qsov_db35_slot = var_qsov_db35;
        *var_qsov_db36_slot = var_qsov_db36;
        *var_qsov_db37_slot = var_qsov_db37;
        *var_qsov_db38_slot = var_qsov_db38;
        *var_qsov_db39_slot = var_qsov_db39;
        *var_qsov_db4_slot = var_qsov_db4;
        *var_qsov_db40_slot = var_qsov_db40;
        *var_qsov_db41_slot = var_qsov_db41;
        *var_qsov_db42_slot = var_qsov_db42;
        *var_qsov_db43_slot = var_qsov_db43;
        *var_qsov_db44_slot = var_qsov_db44;
        *var_qsov_db45_slot = var_qsov_db45;
        *var_qsov_db46_slot = var_qsov_db46;
        *var_qsov_db47_slot = var_qsov_db47;
        *var_qsov_db48_slot = var_qsov_db48;
        *var_qsov_db49_slot = var_qsov_db49;
        *var_qsov_db5_slot = var_qsov_db5;
        *var_qsov_db50_slot = var_qsov_db50;
        *var_qsov_db51_slot = var_qsov_db51;
        *var_qsov_db52_slot = var_qsov_db52;
        *var_qsov_db53_slot = var_qsov_db53;
        *var_qsov_db54_slot = var_qsov_db54;
        *var_qsov_db55_slot = var_qsov_db55;
        *var_qsov_db56_slot = var_qsov_db56;
        *var_qsov_db6_slot = var_qsov_db6;
        *var_qsov_db7_slot = var_qsov_db7;
        *var_qsov_db8_slot = var_qsov_db8;
        *var_qsov_db9_slot = var_qsov_db9;
        *var_qsov_dn0_slot = var_qsov_dn0;
        *var_qsov_dn1_slot = var_qsov_dn1;
        *var_qsov_dn10_slot = var_qsov_dn10;
        *var_qsov_dn11_slot = var_qsov_dn11;
        *var_qsov_dn12_slot = var_qsov_dn12;
        *var_qsov_dn13_slot = var_qsov_dn13;
        *var_qsov_dn14_slot = var_qsov_dn14;
        *var_qsov_dn15_slot = var_qsov_dn15;
        *var_qsov_dn16_slot = var_qsov_dn16;
        *var_qsov_dn17_slot = var_qsov_dn17;
        *var_qsov_dn18_slot = var_qsov_dn18;
        *var_qsov_dn19_slot = var_qsov_dn19;
        *var_qsov_dn2_slot = var_qsov_dn2;
        *var_qsov_dn20_slot = var_qsov_dn20;
        *var_qsov_dn21_slot = var_qsov_dn21;
        *var_qsov_dn22_slot = var_qsov_dn22;
        *var_qsov_dn3_slot = var_qsov_dn3;
        *var_qsov_dn4_slot = var_qsov_dn4;
        *var_qsov_dn5_slot = var_qsov_dn5;
        *var_qsov_dn6_slot = var_qsov_dn6;
        *var_qsov_dn7_slot = var_qsov_dn7;
        *var_qsov_dn8_slot = var_qsov_dn8;
        *var_qsov_dn9_slot = var_qsov_dn9;
        *var_vdseffcv_slot = var_vdseffcv;
        *var_vdseffcv_db0_slot = var_vdseffcv_db0;
        *var_vdseffcv_db1_slot = var_vdseffcv_db1;
        *var_vdseffcv_db10_slot = var_vdseffcv_db10;
        *var_vdseffcv_db11_slot = var_vdseffcv_db11;
        *var_vdseffcv_db12_slot = var_vdseffcv_db12;
        *var_vdseffcv_db13_slot = var_vdseffcv_db13;
        *var_vdseffcv_db14_slot = var_vdseffcv_db14;
        *var_vdseffcv_db15_slot = var_vdseffcv_db15;
        *var_vdseffcv_db16_slot = var_vdseffcv_db16;
        *var_vdseffcv_db17_slot = var_vdseffcv_db17;
        *var_vdseffcv_db18_slot = var_vdseffcv_db18;
        *var_vdseffcv_db19_slot = var_vdseffcv_db19;
        *var_vdseffcv_db2_slot = var_vdseffcv_db2;
        *var_vdseffcv_db20_slot = var_vdseffcv_db20;
        *var_vdseffcv_db21_slot = var_vdseffcv_db21;
        *var_vdseffcv_db22_slot = var_vdseffcv_db22;
        *var_vdseffcv_db23_slot = var_vdseffcv_db23;
        *var_vdseffcv_db24_slot = var_vdseffcv_db24;
        *var_vdseffcv_db25_slot = var_vdseffcv_db25;
        *var_vdseffcv_db26_slot = var_vdseffcv_db26;
        *var_vdseffcv_db27_slot = var_vdseffcv_db27;
        *var_vdseffcv_db28_slot = var_vdseffcv_db28;
        *var_vdseffcv_db29_slot = var_vdseffcv_db29;
        *var_vdseffcv_db3_slot = var_vdseffcv_db3;
        *var_vdseffcv_db30_slot = var_vdseffcv_db30;
        *var_vdseffcv_db31_slot = var_vdseffcv_db31;
        *var_vdseffcv_db32_slot = var_vdseffcv_db32;
        *var_vdseffcv_db33_slot = var_vdseffcv_db33;
        *var_vdseffcv_db34_slot = var_vdseffcv_db34;
        *var_vdseffcv_db35_slot = var_vdseffcv_db35;
        *var_vdseffcv_db36_slot = var_vdseffcv_db36;
        *var_vdseffcv_db37_slot = var_vdseffcv_db37;
        *var_vdseffcv_db38_slot = var_vdseffcv_db38;
        *var_vdseffcv_db39_slot = var_vdseffcv_db39;
        *var_vdseffcv_db4_slot = var_vdseffcv_db4;
        *var_vdseffcv_db40_slot = var_vdseffcv_db40;
        *var_vdseffcv_db41_slot = var_vdseffcv_db41;
        *var_vdseffcv_db42_slot = var_vdseffcv_db42;
        *var_vdseffcv_db43_slot = var_vdseffcv_db43;
        *var_vdseffcv_db44_slot = var_vdseffcv_db44;
        *var_vdseffcv_db45_slot = var_vdseffcv_db45;
        *var_vdseffcv_db46_slot = var_vdseffcv_db46;
        *var_vdseffcv_db47_slot = var_vdseffcv_db47;
        *var_vdseffcv_db48_slot = var_vdseffcv_db48;
        *var_vdseffcv_db49_slot = var_vdseffcv_db49;
        *var_vdseffcv_db5_slot = var_vdseffcv_db5;
        *var_vdseffcv_db50_slot = var_vdseffcv_db50;
        *var_vdseffcv_db51_slot = var_vdseffcv_db51;
        *var_vdseffcv_db52_slot = var_vdseffcv_db52;
        *var_vdseffcv_db53_slot = var_vdseffcv_db53;
        *var_vdseffcv_db54_slot = var_vdseffcv_db54;
        *var_vdseffcv_db55_slot = var_vdseffcv_db55;
        *var_vdseffcv_db56_slot = var_vdseffcv_db56;
        *var_vdseffcv_db6_slot = var_vdseffcv_db6;
        *var_vdseffcv_db7_slot = var_vdseffcv_db7;
        *var_vdseffcv_db8_slot = var_vdseffcv_db8;
        *var_vdseffcv_db9_slot = var_vdseffcv_db9;
        *var_vdseffcv_dn0_slot = var_vdseffcv_dn0;
        *var_vdseffcv_dn1_slot = var_vdseffcv_dn1;
        *var_vdseffcv_dn10_slot = var_vdseffcv_dn10;
        *var_vdseffcv_dn11_slot = var_vdseffcv_dn11;
        *var_vdseffcv_dn12_slot = var_vdseffcv_dn12;
        *var_vdseffcv_dn13_slot = var_vdseffcv_dn13;
        *var_vdseffcv_dn14_slot = var_vdseffcv_dn14;
        *var_vdseffcv_dn15_slot = var_vdseffcv_dn15;
        *var_vdseffcv_dn16_slot = var_vdseffcv_dn16;
        *var_vdseffcv_dn17_slot = var_vdseffcv_dn17;
        *var_vdseffcv_dn18_slot = var_vdseffcv_dn18;
        *var_vdseffcv_dn19_slot = var_vdseffcv_dn19;
        *var_vdseffcv_dn2_slot = var_vdseffcv_dn2;
        *var_vdseffcv_dn20_slot = var_vdseffcv_dn20;
        *var_vdseffcv_dn21_slot = var_vdseffcv_dn21;
        *var_vdseffcv_dn22_slot = var_vdseffcv_dn22;
        *var_vdseffcv_dn3_slot = var_vdseffcv_dn3;
        *var_vdseffcv_dn4_slot = var_vdseffcv_dn4;
        *var_vdseffcv_dn5_slot = var_vdseffcv_dn5;
        *var_vdseffcv_dn6_slot = var_vdseffcv_dn6;
        *var_vdseffcv_dn7_slot = var_vdseffcv_dn7;
        *var_vdseffcv_dn8_slot = var_vdseffcv_dn8;
        *var_vdseffcv_dn9_slot = var_vdseffcv_dn9;
    }

    pub(super) fn stamp_transient_block_31(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_cgdvar: f64,
        var_cgdvar_db0: f64,
        var_cgdvar_db1: f64,
        var_cgdvar_db10: f64,
        var_cgdvar_db11: f64,
        var_cgdvar_db12: f64,
        var_cgdvar_db13: f64,
        var_cgdvar_db14: f64,
        var_cgdvar_db15: f64,
        var_cgdvar_db16: f64,
        var_cgdvar_db17: f64,
        var_cgdvar_db18: f64,
        var_cgdvar_db19: f64,
        var_cgdvar_db2: f64,
        var_cgdvar_db20: f64,
        var_cgdvar_db21: f64,
        var_cgdvar_db22: f64,
        var_cgdvar_db23: f64,
        var_cgdvar_db24: f64,
        var_cgdvar_db25: f64,
        var_cgdvar_db26: f64,
        var_cgdvar_db27: f64,
        var_cgdvar_db28: f64,
        var_cgdvar_db29: f64,
        var_cgdvar_db3: f64,
        var_cgdvar_db30: f64,
        var_cgdvar_db31: f64,
        var_cgdvar_db32: f64,
        var_cgdvar_db33: f64,
        var_cgdvar_db34: f64,
        var_cgdvar_db35: f64,
        var_cgdvar_db36: f64,
        var_cgdvar_db37: f64,
        var_cgdvar_db38: f64,
        var_cgdvar_db39: f64,
        var_cgdvar_db4: f64,
        var_cgdvar_db40: f64,
        var_cgdvar_db41: f64,
        var_cgdvar_db42: f64,
        var_cgdvar_db43: f64,
        var_cgdvar_db44: f64,
        var_cgdvar_db45: f64,
        var_cgdvar_db46: f64,
        var_cgdvar_db47: f64,
        var_cgdvar_db48: f64,
        var_cgdvar_db49: f64,
        var_cgdvar_db5: f64,
        var_cgdvar_db50: f64,
        var_cgdvar_db51: f64,
        var_cgdvar_db52: f64,
        var_cgdvar_db53: f64,
        var_cgdvar_db54: f64,
        var_cgdvar_db6: f64,
        var_cgdvar_db7: f64,
        var_cgdvar_db8: f64,
        var_cgdvar_db9: f64,
        var_cgdvar_dn0: f64,
        var_cgdvar_dn1: f64,
        var_cgdvar_dn10: f64,
        var_cgdvar_dn11: f64,
        var_cgdvar_dn12: f64,
        var_cgdvar_dn13: f64,
        var_cgdvar_dn14: f64,
        var_cgdvar_dn15: f64,
        var_cgdvar_dn16: f64,
        var_cgdvar_dn17: f64,
        var_cgdvar_dn18: f64,
        var_cgdvar_dn19: f64,
        var_cgdvar_dn2: f64,
        var_cgdvar_dn20: f64,
        var_cgdvar_dn21: f64,
        var_cgdvar_dn22: f64,
        var_cgdvar_dn3: f64,
        var_cgdvar_dn4: f64,
        var_cgdvar_dn5: f64,
        var_cgdvar_dn6: f64,
        var_cgdvar_dn7: f64,
        var_cgdvar_dn8: f64,
        var_cgdvar_dn9: f64,
        var_guard524: f64,
        var_tdev: f64,
        var_tdev_db0: f64,
        var_tdev_db1: f64,
        var_tdev_db10: f64,
        var_tdev_db11: f64,
        var_tdev_db12: f64,
        var_tdev_db13: f64,
        var_tdev_db14: f64,
        var_tdev_db15: f64,
        var_tdev_db16: f64,
        var_tdev_db17: f64,
        var_tdev_db18: f64,
        var_tdev_db19: f64,
        var_tdev_db2: f64,
        var_tdev_db20: f64,
        var_tdev_db21: f64,
        var_tdev_db22: f64,
        var_tdev_db23: f64,
        var_tdev_db24: f64,
        var_tdev_db25: f64,
        var_tdev_db26: f64,
        var_tdev_db27: f64,
        var_tdev_db28: f64,
        var_tdev_db29: f64,
        var_tdev_db3: f64,
        var_tdev_db30: f64,
        var_tdev_db31: f64,
        var_tdev_db32: f64,
        var_tdev_db33: f64,
        var_tdev_db34: f64,
        var_tdev_db35: f64,
        var_tdev_db36: f64,
        var_tdev_db37: f64,
        var_tdev_db38: f64,
        var_tdev_db39: f64,
        var_tdev_db4: f64,
        var_tdev_db40: f64,
        var_tdev_db41: f64,
        var_tdev_db42: f64,
        var_tdev_db43: f64,
        var_tdev_db44: f64,
        var_tdev_db45: f64,
        var_tdev_db46: f64,
        var_tdev_db47: f64,
        var_tdev_db48: f64,
        var_tdev_db49: f64,
        var_tdev_db5: f64,
        var_tdev_db50: f64,
        var_tdev_db51: f64,
        var_tdev_db52: f64,
        var_tdev_db53: f64,
        var_tdev_db54: f64,
        var_tdev_db55: f64,
        var_tdev_db56: f64,
        var_tdev_db6: f64,
        var_tdev_db7: f64,
        var_tdev_db8: f64,
        var_tdev_db9: f64,
        var_tdev_dn0: f64,
        var_tdev_dn1: f64,
        var_tdev_dn10: f64,
        var_tdev_dn11: f64,
        var_tdev_dn12: f64,
        var_tdev_dn13: f64,
        var_tdev_dn14: f64,
        var_tdev_dn15: f64,
        var_tdev_dn16: f64,
        var_tdev_dn17: f64,
        var_tdev_dn18: f64,
        var_tdev_dn19: f64,
        var_tdev_dn2: f64,
        var_tdev_dn20: f64,
        var_tdev_dn21: f64,
        var_tdev_dn22: f64,
        var_tdev_dn3: f64,
        var_tdev_dn4: f64,
        var_tdev_dn5: f64,
        var_tdev_dn6: f64,
        var_tdev_dn7: f64,
        var_tdev_dn8: f64,
        var_tdev_dn9: f64,
        var_tnom: f64,
        var_qbdov_slot: &mut f64,
        var_qbdov_db0_slot: &mut f64,
        var_qbdov_db1_slot: &mut f64,
        var_qbdov_db10_slot: &mut f64,
        var_qbdov_db11_slot: &mut f64,
        var_qbdov_db12_slot: &mut f64,
        var_qbdov_db13_slot: &mut f64,
        var_qbdov_db14_slot: &mut f64,
        var_qbdov_db15_slot: &mut f64,
        var_qbdov_db16_slot: &mut f64,
        var_qbdov_db17_slot: &mut f64,
        var_qbdov_db18_slot: &mut f64,
        var_qbdov_db19_slot: &mut f64,
        var_qbdov_db2_slot: &mut f64,
        var_qbdov_db20_slot: &mut f64,
        var_qbdov_db21_slot: &mut f64,
        var_qbdov_db22_slot: &mut f64,
        var_qbdov_db23_slot: &mut f64,
        var_qbdov_db24_slot: &mut f64,
        var_qbdov_db25_slot: &mut f64,
        var_qbdov_db26_slot: &mut f64,
        var_qbdov_db27_slot: &mut f64,
        var_qbdov_db28_slot: &mut f64,
        var_qbdov_db29_slot: &mut f64,
        var_qbdov_db3_slot: &mut f64,
        var_qbdov_db30_slot: &mut f64,
        var_qbdov_db31_slot: &mut f64,
        var_qbdov_db32_slot: &mut f64,
        var_qbdov_db33_slot: &mut f64,
        var_qbdov_db34_slot: &mut f64,
        var_qbdov_db35_slot: &mut f64,
        var_qbdov_db36_slot: &mut f64,
        var_qbdov_db37_slot: &mut f64,
        var_qbdov_db38_slot: &mut f64,
        var_qbdov_db39_slot: &mut f64,
        var_qbdov_db4_slot: &mut f64,
        var_qbdov_db40_slot: &mut f64,
        var_qbdov_db41_slot: &mut f64,
        var_qbdov_db42_slot: &mut f64,
        var_qbdov_db43_slot: &mut f64,
        var_qbdov_db44_slot: &mut f64,
        var_qbdov_db45_slot: &mut f64,
        var_qbdov_db46_slot: &mut f64,
        var_qbdov_db47_slot: &mut f64,
        var_qbdov_db48_slot: &mut f64,
        var_qbdov_db49_slot: &mut f64,
        var_qbdov_db5_slot: &mut f64,
        var_qbdov_db50_slot: &mut f64,
        var_qbdov_db51_slot: &mut f64,
        var_qbdov_db52_slot: &mut f64,
        var_qbdov_db53_slot: &mut f64,
        var_qbdov_db54_slot: &mut f64,
        var_qbdov_db55_slot: &mut f64,
        var_qbdov_db56_slot: &mut f64,
        var_qbdov_db6_slot: &mut f64,
        var_qbdov_db7_slot: &mut f64,
        var_qbdov_db8_slot: &mut f64,
        var_qbdov_db9_slot: &mut f64,
        var_qbdov_dn0_slot: &mut f64,
        var_qbdov_dn1_slot: &mut f64,
        var_qbdov_dn10_slot: &mut f64,
        var_qbdov_dn11_slot: &mut f64,
        var_qbdov_dn12_slot: &mut f64,
        var_qbdov_dn13_slot: &mut f64,
        var_qbdov_dn14_slot: &mut f64,
        var_qbdov_dn15_slot: &mut f64,
        var_qbdov_dn16_slot: &mut f64,
        var_qbdov_dn17_slot: &mut f64,
        var_qbdov_dn18_slot: &mut f64,
        var_qbdov_dn19_slot: &mut f64,
        var_qbdov_dn2_slot: &mut f64,
        var_qbdov_dn20_slot: &mut f64,
        var_qbdov_dn21_slot: &mut f64,
        var_qbdov_dn22_slot: &mut f64,
        var_qbdov_dn3_slot: &mut f64,
        var_qbdov_dn4_slot: &mut f64,
        var_qbdov_dn5_slot: &mut f64,
        var_qbdov_dn6_slot: &mut f64,
        var_qbdov_dn7_slot: &mut f64,
        var_qbdov_dn8_slot: &mut f64,
        var_qbdov_dn9_slot: &mut f64,
        var_qbgov_slot: &mut f64,
        var_qbgov_db0_slot: &mut f64,
        var_qbgov_db1_slot: &mut f64,
        var_qbgov_db10_slot: &mut f64,
        var_qbgov_db11_slot: &mut f64,
        var_qbgov_db12_slot: &mut f64,
        var_qbgov_db13_slot: &mut f64,
        var_qbgov_db14_slot: &mut f64,
        var_qbgov_db15_slot: &mut f64,
        var_qbgov_db16_slot: &mut f64,
        var_qbgov_db17_slot: &mut f64,
        var_qbgov_db18_slot: &mut f64,
        var_qbgov_db19_slot: &mut f64,
        var_qbgov_db2_slot: &mut f64,
        var_qbgov_db20_slot: &mut f64,
        var_qbgov_db21_slot: &mut f64,
        var_qbgov_db22_slot: &mut f64,
        var_qbgov_db23_slot: &mut f64,
        var_qbgov_db24_slot: &mut f64,
        var_qbgov_db25_slot: &mut f64,
        var_qbgov_db26_slot: &mut f64,
        var_qbgov_db27_slot: &mut f64,
        var_qbgov_db28_slot: &mut f64,
        var_qbgov_db29_slot: &mut f64,
        var_qbgov_db3_slot: &mut f64,
        var_qbgov_db30_slot: &mut f64,
        var_qbgov_db31_slot: &mut f64,
        var_qbgov_db32_slot: &mut f64,
        var_qbgov_db33_slot: &mut f64,
        var_qbgov_db34_slot: &mut f64,
        var_qbgov_db35_slot: &mut f64,
        var_qbgov_db36_slot: &mut f64,
        var_qbgov_db37_slot: &mut f64,
        var_qbgov_db38_slot: &mut f64,
        var_qbgov_db39_slot: &mut f64,
        var_qbgov_db4_slot: &mut f64,
        var_qbgov_db40_slot: &mut f64,
        var_qbgov_db41_slot: &mut f64,
        var_qbgov_db42_slot: &mut f64,
        var_qbgov_db43_slot: &mut f64,
        var_qbgov_db44_slot: &mut f64,
        var_qbgov_db45_slot: &mut f64,
        var_qbgov_db46_slot: &mut f64,
        var_qbgov_db47_slot: &mut f64,
        var_qbgov_db48_slot: &mut f64,
        var_qbgov_db49_slot: &mut f64,
        var_qbgov_db5_slot: &mut f64,
        var_qbgov_db50_slot: &mut f64,
        var_qbgov_db51_slot: &mut f64,
        var_qbgov_db52_slot: &mut f64,
        var_qbgov_db53_slot: &mut f64,
        var_qbgov_db54_slot: &mut f64,
        var_qbgov_db55_slot: &mut f64,
        var_qbgov_db56_slot: &mut f64,
        var_qbgov_db6_slot: &mut f64,
        var_qbgov_db7_slot: &mut f64,
        var_qbgov_db8_slot: &mut f64,
        var_qbgov_db9_slot: &mut f64,
        var_qbgov_dn0_slot: &mut f64,
        var_qbgov_dn1_slot: &mut f64,
        var_qbgov_dn10_slot: &mut f64,
        var_qbgov_dn11_slot: &mut f64,
        var_qbgov_dn12_slot: &mut f64,
        var_qbgov_dn13_slot: &mut f64,
        var_qbgov_dn14_slot: &mut f64,
        var_qbgov_dn15_slot: &mut f64,
        var_qbgov_dn16_slot: &mut f64,
        var_qbgov_dn17_slot: &mut f64,
        var_qbgov_dn18_slot: &mut f64,
        var_qbgov_dn19_slot: &mut f64,
        var_qbgov_dn2_slot: &mut f64,
        var_qbgov_dn20_slot: &mut f64,
        var_qbgov_dn21_slot: &mut f64,
        var_qbgov_dn22_slot: &mut f64,
        var_qbgov_dn3_slot: &mut f64,
        var_qbgov_dn4_slot: &mut f64,
        var_qbgov_dn5_slot: &mut f64,
        var_qbgov_dn6_slot: &mut f64,
        var_qbgov_dn7_slot: &mut f64,
        var_qbgov_dn8_slot: &mut f64,
        var_qbgov_dn9_slot: &mut f64,
        var_qbsov_slot: &mut f64,
        var_qbsov_db0_slot: &mut f64,
        var_qbsov_db1_slot: &mut f64,
        var_qbsov_db10_slot: &mut f64,
        var_qbsov_db11_slot: &mut f64,
        var_qbsov_db12_slot: &mut f64,
        var_qbsov_db13_slot: &mut f64,
        var_qbsov_db14_slot: &mut f64,
        var_qbsov_db15_slot: &mut f64,
        var_qbsov_db16_slot: &mut f64,
        var_qbsov_db17_slot: &mut f64,
        var_qbsov_db18_slot: &mut f64,
        var_qbsov_db19_slot: &mut f64,
        var_qbsov_db2_slot: &mut f64,
        var_qbsov_db20_slot: &mut f64,
        var_qbsov_db21_slot: &mut f64,
        var_qbsov_db22_slot: &mut f64,
        var_qbsov_db23_slot: &mut f64,
        var_qbsov_db24_slot: &mut f64,
        var_qbsov_db25_slot: &mut f64,
        var_qbsov_db26_slot: &mut f64,
        var_qbsov_db27_slot: &mut f64,
        var_qbsov_db28_slot: &mut f64,
        var_qbsov_db29_slot: &mut f64,
        var_qbsov_db3_slot: &mut f64,
        var_qbsov_db30_slot: &mut f64,
        var_qbsov_db31_slot: &mut f64,
        var_qbsov_db32_slot: &mut f64,
        var_qbsov_db33_slot: &mut f64,
        var_qbsov_db34_slot: &mut f64,
        var_qbsov_db35_slot: &mut f64,
        var_qbsov_db36_slot: &mut f64,
        var_qbsov_db37_slot: &mut f64,
        var_qbsov_db38_slot: &mut f64,
        var_qbsov_db39_slot: &mut f64,
        var_qbsov_db4_slot: &mut f64,
        var_qbsov_db40_slot: &mut f64,
        var_qbsov_db41_slot: &mut f64,
        var_qbsov_db42_slot: &mut f64,
        var_qbsov_db43_slot: &mut f64,
        var_qbsov_db44_slot: &mut f64,
        var_qbsov_db45_slot: &mut f64,
        var_qbsov_db46_slot: &mut f64,
        var_qbsov_db47_slot: &mut f64,
        var_qbsov_db48_slot: &mut f64,
        var_qbsov_db49_slot: &mut f64,
        var_qbsov_db5_slot: &mut f64,
        var_qbsov_db50_slot: &mut f64,
        var_qbsov_db51_slot: &mut f64,
        var_qbsov_db52_slot: &mut f64,
        var_qbsov_db53_slot: &mut f64,
        var_qbsov_db54_slot: &mut f64,
        var_qbsov_db55_slot: &mut f64,
        var_qbsov_db56_slot: &mut f64,
        var_qbsov_db6_slot: &mut f64,
        var_qbsov_db7_slot: &mut f64,
        var_qbsov_db8_slot: &mut f64,
        var_qbsov_db9_slot: &mut f64,
        var_qbsov_dn0_slot: &mut f64,
        var_qbsov_dn1_slot: &mut f64,
        var_qbsov_dn10_slot: &mut f64,
        var_qbsov_dn11_slot: &mut f64,
        var_qbsov_dn12_slot: &mut f64,
        var_qbsov_dn13_slot: &mut f64,
        var_qbsov_dn14_slot: &mut f64,
        var_qbsov_dn15_slot: &mut f64,
        var_qbsov_dn16_slot: &mut f64,
        var_qbsov_dn17_slot: &mut f64,
        var_qbsov_dn18_slot: &mut f64,
        var_qbsov_dn19_slot: &mut f64,
        var_qbsov_dn2_slot: &mut f64,
        var_qbsov_dn20_slot: &mut f64,
        var_qbsov_dn21_slot: &mut f64,
        var_qbsov_dn22_slot: &mut f64,
        var_qbsov_dn3_slot: &mut f64,
        var_qbsov_dn4_slot: &mut f64,
        var_qbsov_dn5_slot: &mut f64,
        var_qbsov_dn6_slot: &mut f64,
        var_qbsov_dn7_slot: &mut f64,
        var_qbsov_dn8_slot: &mut f64,
        var_qbsov_dn9_slot: &mut f64,
        var_qdov_slot: &mut f64,
        var_qdov_db0_slot: &mut f64,
        var_qdov_db1_slot: &mut f64,
        var_qdov_db10_slot: &mut f64,
        var_qdov_db11_slot: &mut f64,
        var_qdov_db12_slot: &mut f64,
        var_qdov_db13_slot: &mut f64,
        var_qdov_db14_slot: &mut f64,
        var_qdov_db15_slot: &mut f64,
        var_qdov_db16_slot: &mut f64,
        var_qdov_db17_slot: &mut f64,
        var_qdov_db18_slot: &mut f64,
        var_qdov_db19_slot: &mut f64,
        var_qdov_db2_slot: &mut f64,
        var_qdov_db20_slot: &mut f64,
        var_qdov_db21_slot: &mut f64,
        var_qdov_db22_slot: &mut f64,
        var_qdov_db23_slot: &mut f64,
        var_qdov_db24_slot: &mut f64,
        var_qdov_db25_slot: &mut f64,
        var_qdov_db26_slot: &mut f64,
        var_qdov_db27_slot: &mut f64,
        var_qdov_db28_slot: &mut f64,
        var_qdov_db29_slot: &mut f64,
        var_qdov_db3_slot: &mut f64,
        var_qdov_db30_slot: &mut f64,
        var_qdov_db31_slot: &mut f64,
        var_qdov_db32_slot: &mut f64,
        var_qdov_db33_slot: &mut f64,
        var_qdov_db34_slot: &mut f64,
        var_qdov_db35_slot: &mut f64,
        var_qdov_db36_slot: &mut f64,
        var_qdov_db37_slot: &mut f64,
        var_qdov_db38_slot: &mut f64,
        var_qdov_db39_slot: &mut f64,
        var_qdov_db4_slot: &mut f64,
        var_qdov_db40_slot: &mut f64,
        var_qdov_db41_slot: &mut f64,
        var_qdov_db42_slot: &mut f64,
        var_qdov_db43_slot: &mut f64,
        var_qdov_db44_slot: &mut f64,
        var_qdov_db45_slot: &mut f64,
        var_qdov_db46_slot: &mut f64,
        var_qdov_db47_slot: &mut f64,
        var_qdov_db48_slot: &mut f64,
        var_qdov_db49_slot: &mut f64,
        var_qdov_db5_slot: &mut f64,
        var_qdov_db50_slot: &mut f64,
        var_qdov_db51_slot: &mut f64,
        var_qdov_db52_slot: &mut f64,
        var_qdov_db53_slot: &mut f64,
        var_qdov_db54_slot: &mut f64,
        var_qdov_db55_slot: &mut f64,
        var_qdov_db56_slot: &mut f64,
        var_qdov_db6_slot: &mut f64,
        var_qdov_db7_slot: &mut f64,
        var_qdov_db8_slot: &mut f64,
        var_qdov_db9_slot: &mut f64,
        var_qdov_dn0_slot: &mut f64,
        var_qdov_dn1_slot: &mut f64,
        var_qdov_dn10_slot: &mut f64,
        var_qdov_dn11_slot: &mut f64,
        var_qdov_dn12_slot: &mut f64,
        var_qdov_dn13_slot: &mut f64,
        var_qdov_dn14_slot: &mut f64,
        var_qdov_dn15_slot: &mut f64,
        var_qdov_dn16_slot: &mut f64,
        var_qdov_dn17_slot: &mut f64,
        var_qdov_dn18_slot: &mut f64,
        var_qdov_dn19_slot: &mut f64,
        var_qdov_dn2_slot: &mut f64,
        var_qdov_dn20_slot: &mut f64,
        var_qdov_dn21_slot: &mut f64,
        var_qdov_dn22_slot: &mut f64,
        var_qdov_dn3_slot: &mut f64,
        var_qdov_dn4_slot: &mut f64,
        var_qdov_dn5_slot: &mut f64,
        var_qdov_dn6_slot: &mut f64,
        var_qdov_dn7_slot: &mut f64,
        var_qdov_dn8_slot: &mut f64,
        var_qdov_dn9_slot: &mut f64,
        var_qdsov_slot: &mut f64,
        var_qdsov_db0_slot: &mut f64,
        var_qdsov_db1_slot: &mut f64,
        var_qdsov_db10_slot: &mut f64,
        var_qdsov_db11_slot: &mut f64,
        var_qdsov_db12_slot: &mut f64,
        var_qdsov_db13_slot: &mut f64,
        var_qdsov_db14_slot: &mut f64,
        var_qdsov_db15_slot: &mut f64,
        var_qdsov_db16_slot: &mut f64,
        var_qdsov_db17_slot: &mut f64,
        var_qdsov_db18_slot: &mut f64,
        var_qdsov_db19_slot: &mut f64,
        var_qdsov_db2_slot: &mut f64,
        var_qdsov_db20_slot: &mut f64,
        var_qdsov_db21_slot: &mut f64,
        var_qdsov_db22_slot: &mut f64,
        var_qdsov_db23_slot: &mut f64,
        var_qdsov_db24_slot: &mut f64,
        var_qdsov_db25_slot: &mut f64,
        var_qdsov_db26_slot: &mut f64,
        var_qdsov_db27_slot: &mut f64,
        var_qdsov_db28_slot: &mut f64,
        var_qdsov_db29_slot: &mut f64,
        var_qdsov_db3_slot: &mut f64,
        var_qdsov_db30_slot: &mut f64,
        var_qdsov_db31_slot: &mut f64,
        var_qdsov_db32_slot: &mut f64,
        var_qdsov_db33_slot: &mut f64,
        var_qdsov_db34_slot: &mut f64,
        var_qdsov_db35_slot: &mut f64,
        var_qdsov_db36_slot: &mut f64,
        var_qdsov_db37_slot: &mut f64,
        var_qdsov_db38_slot: &mut f64,
        var_qdsov_db39_slot: &mut f64,
        var_qdsov_db4_slot: &mut f64,
        var_qdsov_db40_slot: &mut f64,
        var_qdsov_db41_slot: &mut f64,
        var_qdsov_db42_slot: &mut f64,
        var_qdsov_db43_slot: &mut f64,
        var_qdsov_db44_slot: &mut f64,
        var_qdsov_db45_slot: &mut f64,
        var_qdsov_db46_slot: &mut f64,
        var_qdsov_db47_slot: &mut f64,
        var_qdsov_db48_slot: &mut f64,
        var_qdsov_db49_slot: &mut f64,
        var_qdsov_db5_slot: &mut f64,
        var_qdsov_db50_slot: &mut f64,
        var_qdsov_db51_slot: &mut f64,
        var_qdsov_db52_slot: &mut f64,
        var_qdsov_db53_slot: &mut f64,
        var_qdsov_db54_slot: &mut f64,
        var_qdsov_db55_slot: &mut f64,
        var_qdsov_db56_slot: &mut f64,
        var_qdsov_db6_slot: &mut f64,
        var_qdsov_db7_slot: &mut f64,
        var_qdsov_db8_slot: &mut f64,
        var_qdsov_db9_slot: &mut f64,
        var_qdsov_dn0_slot: &mut f64,
        var_qdsov_dn1_slot: &mut f64,
        var_qdsov_dn10_slot: &mut f64,
        var_qdsov_dn11_slot: &mut f64,
        var_qdsov_dn12_slot: &mut f64,
        var_qdsov_dn13_slot: &mut f64,
        var_qdsov_dn14_slot: &mut f64,
        var_qdsov_dn15_slot: &mut f64,
        var_qdsov_dn16_slot: &mut f64,
        var_qdsov_dn17_slot: &mut f64,
        var_qdsov_dn18_slot: &mut f64,
        var_qdsov_dn19_slot: &mut f64,
        var_qdsov_dn2_slot: &mut f64,
        var_qdsov_dn20_slot: &mut f64,
        var_qdsov_dn21_slot: &mut f64,
        var_qdsov_dn22_slot: &mut f64,
        var_qdsov_dn3_slot: &mut f64,
        var_qdsov_dn4_slot: &mut f64,
        var_qdsov_dn5_slot: &mut f64,
        var_qdsov_dn6_slot: &mut f64,
        var_qdsov_dn7_slot: &mut f64,
        var_qdsov_dn8_slot: &mut f64,
        var_qdsov_dn9_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let mut var_qbdov: f64 = *var_qbdov_slot;
        let mut var_qbdov_db0: f64 = *var_qbdov_db0_slot;
        let mut var_qbdov_db1: f64 = *var_qbdov_db1_slot;
        let mut var_qbdov_db10: f64 = *var_qbdov_db10_slot;
        let mut var_qbdov_db11: f64 = *var_qbdov_db11_slot;
        let mut var_qbdov_db12: f64 = *var_qbdov_db12_slot;
        let mut var_qbdov_db13: f64 = *var_qbdov_db13_slot;
        let mut var_qbdov_db14: f64 = *var_qbdov_db14_slot;
        let mut var_qbdov_db15: f64 = *var_qbdov_db15_slot;
        let mut var_qbdov_db16: f64 = *var_qbdov_db16_slot;
        let mut var_qbdov_db17: f64 = *var_qbdov_db17_slot;
        let mut var_qbdov_db18: f64 = *var_qbdov_db18_slot;
        let mut var_qbdov_db19: f64 = *var_qbdov_db19_slot;
        let mut var_qbdov_db2: f64 = *var_qbdov_db2_slot;
        let mut var_qbdov_db20: f64 = *var_qbdov_db20_slot;
        let mut var_qbdov_db21: f64 = *var_qbdov_db21_slot;
        let mut var_qbdov_db22: f64 = *var_qbdov_db22_slot;
        let mut var_qbdov_db23: f64 = *var_qbdov_db23_slot;
        let mut var_qbdov_db24: f64 = *var_qbdov_db24_slot;
        let mut var_qbdov_db25: f64 = *var_qbdov_db25_slot;
        let mut var_qbdov_db26: f64 = *var_qbdov_db26_slot;
        let mut var_qbdov_db27: f64 = *var_qbdov_db27_slot;
        let mut var_qbdov_db28: f64 = *var_qbdov_db28_slot;
        let mut var_qbdov_db29: f64 = *var_qbdov_db29_slot;
        let mut var_qbdov_db3: f64 = *var_qbdov_db3_slot;
        let mut var_qbdov_db30: f64 = *var_qbdov_db30_slot;
        let mut var_qbdov_db31: f64 = *var_qbdov_db31_slot;
        let mut var_qbdov_db32: f64 = *var_qbdov_db32_slot;
        let mut var_qbdov_db33: f64 = *var_qbdov_db33_slot;
        let mut var_qbdov_db34: f64 = *var_qbdov_db34_slot;
        let mut var_qbdov_db35: f64 = *var_qbdov_db35_slot;
        let mut var_qbdov_db36: f64 = *var_qbdov_db36_slot;
        let mut var_qbdov_db37: f64 = *var_qbdov_db37_slot;
        let mut var_qbdov_db38: f64 = *var_qbdov_db38_slot;
        let mut var_qbdov_db39: f64 = *var_qbdov_db39_slot;
        let mut var_qbdov_db4: f64 = *var_qbdov_db4_slot;
        let mut var_qbdov_db40: f64 = *var_qbdov_db40_slot;
        let mut var_qbdov_db41: f64 = *var_qbdov_db41_slot;
        let mut var_qbdov_db42: f64 = *var_qbdov_db42_slot;
        let mut var_qbdov_db43: f64 = *var_qbdov_db43_slot;
        let mut var_qbdov_db44: f64 = *var_qbdov_db44_slot;
        let mut var_qbdov_db45: f64 = *var_qbdov_db45_slot;
        let mut var_qbdov_db46: f64 = *var_qbdov_db46_slot;
        let mut var_qbdov_db47: f64 = *var_qbdov_db47_slot;
        let mut var_qbdov_db48: f64 = *var_qbdov_db48_slot;
        let mut var_qbdov_db49: f64 = *var_qbdov_db49_slot;
        let mut var_qbdov_db5: f64 = *var_qbdov_db5_slot;
        let mut var_qbdov_db50: f64 = *var_qbdov_db50_slot;
        let mut var_qbdov_db51: f64 = *var_qbdov_db51_slot;
        let mut var_qbdov_db52: f64 = *var_qbdov_db52_slot;
        let mut var_qbdov_db53: f64 = *var_qbdov_db53_slot;
        let mut var_qbdov_db54: f64 = *var_qbdov_db54_slot;
        let mut var_qbdov_db55: f64 = *var_qbdov_db55_slot;
        let mut var_qbdov_db56: f64 = *var_qbdov_db56_slot;
        let mut var_qbdov_db6: f64 = *var_qbdov_db6_slot;
        let mut var_qbdov_db7: f64 = *var_qbdov_db7_slot;
        let mut var_qbdov_db8: f64 = *var_qbdov_db8_slot;
        let mut var_qbdov_db9: f64 = *var_qbdov_db9_slot;
        let mut var_qbdov_dn0: f64 = *var_qbdov_dn0_slot;
        let mut var_qbdov_dn1: f64 = *var_qbdov_dn1_slot;
        let mut var_qbdov_dn10: f64 = *var_qbdov_dn10_slot;
        let mut var_qbdov_dn11: f64 = *var_qbdov_dn11_slot;
        let mut var_qbdov_dn12: f64 = *var_qbdov_dn12_slot;
        let mut var_qbdov_dn13: f64 = *var_qbdov_dn13_slot;
        let mut var_qbdov_dn14: f64 = *var_qbdov_dn14_slot;
        let mut var_qbdov_dn15: f64 = *var_qbdov_dn15_slot;
        let mut var_qbdov_dn16: f64 = *var_qbdov_dn16_slot;
        let mut var_qbdov_dn17: f64 = *var_qbdov_dn17_slot;
        let mut var_qbdov_dn18: f64 = *var_qbdov_dn18_slot;
        let mut var_qbdov_dn19: f64 = *var_qbdov_dn19_slot;
        let mut var_qbdov_dn2: f64 = *var_qbdov_dn2_slot;
        let mut var_qbdov_dn20: f64 = *var_qbdov_dn20_slot;
        let mut var_qbdov_dn21: f64 = *var_qbdov_dn21_slot;
        let mut var_qbdov_dn22: f64 = *var_qbdov_dn22_slot;
        let mut var_qbdov_dn3: f64 = *var_qbdov_dn3_slot;
        let mut var_qbdov_dn4: f64 = *var_qbdov_dn4_slot;
        let mut var_qbdov_dn5: f64 = *var_qbdov_dn5_slot;
        let mut var_qbdov_dn6: f64 = *var_qbdov_dn6_slot;
        let mut var_qbdov_dn7: f64 = *var_qbdov_dn7_slot;
        let mut var_qbdov_dn8: f64 = *var_qbdov_dn8_slot;
        let mut var_qbdov_dn9: f64 = *var_qbdov_dn9_slot;
        let mut var_qbgov: f64 = *var_qbgov_slot;
        let mut var_qbgov_db0: f64 = *var_qbgov_db0_slot;
        let mut var_qbgov_db1: f64 = *var_qbgov_db1_slot;
        let mut var_qbgov_db10: f64 = *var_qbgov_db10_slot;
        let mut var_qbgov_db11: f64 = *var_qbgov_db11_slot;
        let mut var_qbgov_db12: f64 = *var_qbgov_db12_slot;
        let mut var_qbgov_db13: f64 = *var_qbgov_db13_slot;
        let mut var_qbgov_db14: f64 = *var_qbgov_db14_slot;
        let mut var_qbgov_db15: f64 = *var_qbgov_db15_slot;
        let mut var_qbgov_db16: f64 = *var_qbgov_db16_slot;
        let mut var_qbgov_db17: f64 = *var_qbgov_db17_slot;
        let mut var_qbgov_db18: f64 = *var_qbgov_db18_slot;
        let mut var_qbgov_db19: f64 = *var_qbgov_db19_slot;
        let mut var_qbgov_db2: f64 = *var_qbgov_db2_slot;
        let mut var_qbgov_db20: f64 = *var_qbgov_db20_slot;
        let mut var_qbgov_db21: f64 = *var_qbgov_db21_slot;
        let mut var_qbgov_db22: f64 = *var_qbgov_db22_slot;
        let mut var_qbgov_db23: f64 = *var_qbgov_db23_slot;
        let mut var_qbgov_db24: f64 = *var_qbgov_db24_slot;
        let mut var_qbgov_db25: f64 = *var_qbgov_db25_slot;
        let mut var_qbgov_db26: f64 = *var_qbgov_db26_slot;
        let mut var_qbgov_db27: f64 = *var_qbgov_db27_slot;
        let mut var_qbgov_db28: f64 = *var_qbgov_db28_slot;
        let mut var_qbgov_db29: f64 = *var_qbgov_db29_slot;
        let mut var_qbgov_db3: f64 = *var_qbgov_db3_slot;
        let mut var_qbgov_db30: f64 = *var_qbgov_db30_slot;
        let mut var_qbgov_db31: f64 = *var_qbgov_db31_slot;
        let mut var_qbgov_db32: f64 = *var_qbgov_db32_slot;
        let mut var_qbgov_db33: f64 = *var_qbgov_db33_slot;
        let mut var_qbgov_db34: f64 = *var_qbgov_db34_slot;
        let mut var_qbgov_db35: f64 = *var_qbgov_db35_slot;
        let mut var_qbgov_db36: f64 = *var_qbgov_db36_slot;
        let mut var_qbgov_db37: f64 = *var_qbgov_db37_slot;
        let mut var_qbgov_db38: f64 = *var_qbgov_db38_slot;
        let mut var_qbgov_db39: f64 = *var_qbgov_db39_slot;
        let mut var_qbgov_db4: f64 = *var_qbgov_db4_slot;
        let mut var_qbgov_db40: f64 = *var_qbgov_db40_slot;
        let mut var_qbgov_db41: f64 = *var_qbgov_db41_slot;
        let mut var_qbgov_db42: f64 = *var_qbgov_db42_slot;
        let mut var_qbgov_db43: f64 = *var_qbgov_db43_slot;
        let mut var_qbgov_db44: f64 = *var_qbgov_db44_slot;
        let mut var_qbgov_db45: f64 = *var_qbgov_db45_slot;
        let mut var_qbgov_db46: f64 = *var_qbgov_db46_slot;
        let mut var_qbgov_db47: f64 = *var_qbgov_db47_slot;
        let mut var_qbgov_db48: f64 = *var_qbgov_db48_slot;
        let mut var_qbgov_db49: f64 = *var_qbgov_db49_slot;
        let mut var_qbgov_db5: f64 = *var_qbgov_db5_slot;
        let mut var_qbgov_db50: f64 = *var_qbgov_db50_slot;
        let mut var_qbgov_db51: f64 = *var_qbgov_db51_slot;
        let mut var_qbgov_db52: f64 = *var_qbgov_db52_slot;
        let mut var_qbgov_db53: f64 = *var_qbgov_db53_slot;
        let mut var_qbgov_db54: f64 = *var_qbgov_db54_slot;
        let mut var_qbgov_db55: f64 = *var_qbgov_db55_slot;
        let mut var_qbgov_db56: f64 = *var_qbgov_db56_slot;
        let mut var_qbgov_db6: f64 = *var_qbgov_db6_slot;
        let mut var_qbgov_db7: f64 = *var_qbgov_db7_slot;
        let mut var_qbgov_db8: f64 = *var_qbgov_db8_slot;
        let mut var_qbgov_db9: f64 = *var_qbgov_db9_slot;
        let mut var_qbgov_dn0: f64 = *var_qbgov_dn0_slot;
        let mut var_qbgov_dn1: f64 = *var_qbgov_dn1_slot;
        let mut var_qbgov_dn10: f64 = *var_qbgov_dn10_slot;
        let mut var_qbgov_dn11: f64 = *var_qbgov_dn11_slot;
        let mut var_qbgov_dn12: f64 = *var_qbgov_dn12_slot;
        let mut var_qbgov_dn13: f64 = *var_qbgov_dn13_slot;
        let mut var_qbgov_dn14: f64 = *var_qbgov_dn14_slot;
        let mut var_qbgov_dn15: f64 = *var_qbgov_dn15_slot;
        let mut var_qbgov_dn16: f64 = *var_qbgov_dn16_slot;
        let mut var_qbgov_dn17: f64 = *var_qbgov_dn17_slot;
        let mut var_qbgov_dn18: f64 = *var_qbgov_dn18_slot;
        let mut var_qbgov_dn19: f64 = *var_qbgov_dn19_slot;
        let mut var_qbgov_dn2: f64 = *var_qbgov_dn2_slot;
        let mut var_qbgov_dn20: f64 = *var_qbgov_dn20_slot;
        let mut var_qbgov_dn21: f64 = *var_qbgov_dn21_slot;
        let mut var_qbgov_dn22: f64 = *var_qbgov_dn22_slot;
        let mut var_qbgov_dn3: f64 = *var_qbgov_dn3_slot;
        let mut var_qbgov_dn4: f64 = *var_qbgov_dn4_slot;
        let mut var_qbgov_dn5: f64 = *var_qbgov_dn5_slot;
        let mut var_qbgov_dn6: f64 = *var_qbgov_dn6_slot;
        let mut var_qbgov_dn7: f64 = *var_qbgov_dn7_slot;
        let mut var_qbgov_dn8: f64 = *var_qbgov_dn8_slot;
        let mut var_qbgov_dn9: f64 = *var_qbgov_dn9_slot;
        let mut var_qbsov: f64 = *var_qbsov_slot;
        let mut var_qbsov_db0: f64 = *var_qbsov_db0_slot;
        let mut var_qbsov_db1: f64 = *var_qbsov_db1_slot;
        let mut var_qbsov_db10: f64 = *var_qbsov_db10_slot;
        let mut var_qbsov_db11: f64 = *var_qbsov_db11_slot;
        let mut var_qbsov_db12: f64 = *var_qbsov_db12_slot;
        let mut var_qbsov_db13: f64 = *var_qbsov_db13_slot;
        let mut var_qbsov_db14: f64 = *var_qbsov_db14_slot;
        let mut var_qbsov_db15: f64 = *var_qbsov_db15_slot;
        let mut var_qbsov_db16: f64 = *var_qbsov_db16_slot;
        let mut var_qbsov_db17: f64 = *var_qbsov_db17_slot;
        let mut var_qbsov_db18: f64 = *var_qbsov_db18_slot;
        let mut var_qbsov_db19: f64 = *var_qbsov_db19_slot;
        let mut var_qbsov_db2: f64 = *var_qbsov_db2_slot;
        let mut var_qbsov_db20: f64 = *var_qbsov_db20_slot;
        let mut var_qbsov_db21: f64 = *var_qbsov_db21_slot;
        let mut var_qbsov_db22: f64 = *var_qbsov_db22_slot;
        let mut var_qbsov_db23: f64 = *var_qbsov_db23_slot;
        let mut var_qbsov_db24: f64 = *var_qbsov_db24_slot;
        let mut var_qbsov_db25: f64 = *var_qbsov_db25_slot;
        let mut var_qbsov_db26: f64 = *var_qbsov_db26_slot;
        let mut var_qbsov_db27: f64 = *var_qbsov_db27_slot;
        let mut var_qbsov_db28: f64 = *var_qbsov_db28_slot;
        let mut var_qbsov_db29: f64 = *var_qbsov_db29_slot;
        let mut var_qbsov_db3: f64 = *var_qbsov_db3_slot;
        let mut var_qbsov_db30: f64 = *var_qbsov_db30_slot;
        let mut var_qbsov_db31: f64 = *var_qbsov_db31_slot;
        let mut var_qbsov_db32: f64 = *var_qbsov_db32_slot;
        let mut var_qbsov_db33: f64 = *var_qbsov_db33_slot;
        let mut var_qbsov_db34: f64 = *var_qbsov_db34_slot;
        let mut var_qbsov_db35: f64 = *var_qbsov_db35_slot;
        let mut var_qbsov_db36: f64 = *var_qbsov_db36_slot;
        let mut var_qbsov_db37: f64 = *var_qbsov_db37_slot;
        let mut var_qbsov_db38: f64 = *var_qbsov_db38_slot;
        let mut var_qbsov_db39: f64 = *var_qbsov_db39_slot;
        let mut var_qbsov_db4: f64 = *var_qbsov_db4_slot;
        let mut var_qbsov_db40: f64 = *var_qbsov_db40_slot;
        let mut var_qbsov_db41: f64 = *var_qbsov_db41_slot;
        let mut var_qbsov_db42: f64 = *var_qbsov_db42_slot;
        let mut var_qbsov_db43: f64 = *var_qbsov_db43_slot;
        let mut var_qbsov_db44: f64 = *var_qbsov_db44_slot;
        let mut var_qbsov_db45: f64 = *var_qbsov_db45_slot;
        let mut var_qbsov_db46: f64 = *var_qbsov_db46_slot;
        let mut var_qbsov_db47: f64 = *var_qbsov_db47_slot;
        let mut var_qbsov_db48: f64 = *var_qbsov_db48_slot;
        let mut var_qbsov_db49: f64 = *var_qbsov_db49_slot;
        let mut var_qbsov_db5: f64 = *var_qbsov_db5_slot;
        let mut var_qbsov_db50: f64 = *var_qbsov_db50_slot;
        let mut var_qbsov_db51: f64 = *var_qbsov_db51_slot;
        let mut var_qbsov_db52: f64 = *var_qbsov_db52_slot;
        let mut var_qbsov_db53: f64 = *var_qbsov_db53_slot;
        let mut var_qbsov_db54: f64 = *var_qbsov_db54_slot;
        let mut var_qbsov_db55: f64 = *var_qbsov_db55_slot;
        let mut var_qbsov_db56: f64 = *var_qbsov_db56_slot;
        let mut var_qbsov_db6: f64 = *var_qbsov_db6_slot;
        let mut var_qbsov_db7: f64 = *var_qbsov_db7_slot;
        let mut var_qbsov_db8: f64 = *var_qbsov_db8_slot;
        let mut var_qbsov_db9: f64 = *var_qbsov_db9_slot;
        let mut var_qbsov_dn0: f64 = *var_qbsov_dn0_slot;
        let mut var_qbsov_dn1: f64 = *var_qbsov_dn1_slot;
        let mut var_qbsov_dn10: f64 = *var_qbsov_dn10_slot;
        let mut var_qbsov_dn11: f64 = *var_qbsov_dn11_slot;
        let mut var_qbsov_dn12: f64 = *var_qbsov_dn12_slot;
        let mut var_qbsov_dn13: f64 = *var_qbsov_dn13_slot;
        let mut var_qbsov_dn14: f64 = *var_qbsov_dn14_slot;
        let mut var_qbsov_dn15: f64 = *var_qbsov_dn15_slot;
        let mut var_qbsov_dn16: f64 = *var_qbsov_dn16_slot;
        let mut var_qbsov_dn17: f64 = *var_qbsov_dn17_slot;
        let mut var_qbsov_dn18: f64 = *var_qbsov_dn18_slot;
        let mut var_qbsov_dn19: f64 = *var_qbsov_dn19_slot;
        let mut var_qbsov_dn2: f64 = *var_qbsov_dn2_slot;
        let mut var_qbsov_dn20: f64 = *var_qbsov_dn20_slot;
        let mut var_qbsov_dn21: f64 = *var_qbsov_dn21_slot;
        let mut var_qbsov_dn22: f64 = *var_qbsov_dn22_slot;
        let mut var_qbsov_dn3: f64 = *var_qbsov_dn3_slot;
        let mut var_qbsov_dn4: f64 = *var_qbsov_dn4_slot;
        let mut var_qbsov_dn5: f64 = *var_qbsov_dn5_slot;
        let mut var_qbsov_dn6: f64 = *var_qbsov_dn6_slot;
        let mut var_qbsov_dn7: f64 = *var_qbsov_dn7_slot;
        let mut var_qbsov_dn8: f64 = *var_qbsov_dn8_slot;
        let mut var_qbsov_dn9: f64 = *var_qbsov_dn9_slot;
        let mut var_qdov: f64 = *var_qdov_slot;
        let mut var_qdov_db0: f64 = *var_qdov_db0_slot;
        let mut var_qdov_db1: f64 = *var_qdov_db1_slot;
        let mut var_qdov_db10: f64 = *var_qdov_db10_slot;
        let mut var_qdov_db11: f64 = *var_qdov_db11_slot;
        let mut var_qdov_db12: f64 = *var_qdov_db12_slot;
        let mut var_qdov_db13: f64 = *var_qdov_db13_slot;
        let mut var_qdov_db14: f64 = *var_qdov_db14_slot;
        let mut var_qdov_db15: f64 = *var_qdov_db15_slot;
        let mut var_qdov_db16: f64 = *var_qdov_db16_slot;
        let mut var_qdov_db17: f64 = *var_qdov_db17_slot;
        let mut var_qdov_db18: f64 = *var_qdov_db18_slot;
        let mut var_qdov_db19: f64 = *var_qdov_db19_slot;
        let mut var_qdov_db2: f64 = *var_qdov_db2_slot;
        let mut var_qdov_db20: f64 = *var_qdov_db20_slot;
        let mut var_qdov_db21: f64 = *var_qdov_db21_slot;
        let mut var_qdov_db22: f64 = *var_qdov_db22_slot;
        let mut var_qdov_db23: f64 = *var_qdov_db23_slot;
        let mut var_qdov_db24: f64 = *var_qdov_db24_slot;
        let mut var_qdov_db25: f64 = *var_qdov_db25_slot;
        let mut var_qdov_db26: f64 = *var_qdov_db26_slot;
        let mut var_qdov_db27: f64 = *var_qdov_db27_slot;
        let mut var_qdov_db28: f64 = *var_qdov_db28_slot;
        let mut var_qdov_db29: f64 = *var_qdov_db29_slot;
        let mut var_qdov_db3: f64 = *var_qdov_db3_slot;
        let mut var_qdov_db30: f64 = *var_qdov_db30_slot;
        let mut var_qdov_db31: f64 = *var_qdov_db31_slot;
        let mut var_qdov_db32: f64 = *var_qdov_db32_slot;
        let mut var_qdov_db33: f64 = *var_qdov_db33_slot;
        let mut var_qdov_db34: f64 = *var_qdov_db34_slot;
        let mut var_qdov_db35: f64 = *var_qdov_db35_slot;
        let mut var_qdov_db36: f64 = *var_qdov_db36_slot;
        let mut var_qdov_db37: f64 = *var_qdov_db37_slot;
        let mut var_qdov_db38: f64 = *var_qdov_db38_slot;
        let mut var_qdov_db39: f64 = *var_qdov_db39_slot;
        let mut var_qdov_db4: f64 = *var_qdov_db4_slot;
        let mut var_qdov_db40: f64 = *var_qdov_db40_slot;
        let mut var_qdov_db41: f64 = *var_qdov_db41_slot;
        let mut var_qdov_db42: f64 = *var_qdov_db42_slot;
        let mut var_qdov_db43: f64 = *var_qdov_db43_slot;
        let mut var_qdov_db44: f64 = *var_qdov_db44_slot;
        let mut var_qdov_db45: f64 = *var_qdov_db45_slot;
        let mut var_qdov_db46: f64 = *var_qdov_db46_slot;
        let mut var_qdov_db47: f64 = *var_qdov_db47_slot;
        let mut var_qdov_db48: f64 = *var_qdov_db48_slot;
        let mut var_qdov_db49: f64 = *var_qdov_db49_slot;
        let mut var_qdov_db5: f64 = *var_qdov_db5_slot;
        let mut var_qdov_db50: f64 = *var_qdov_db50_slot;
        let mut var_qdov_db51: f64 = *var_qdov_db51_slot;
        let mut var_qdov_db52: f64 = *var_qdov_db52_slot;
        let mut var_qdov_db53: f64 = *var_qdov_db53_slot;
        let mut var_qdov_db54: f64 = *var_qdov_db54_slot;
        let mut var_qdov_db55: f64 = *var_qdov_db55_slot;
        let mut var_qdov_db56: f64 = *var_qdov_db56_slot;
        let mut var_qdov_db6: f64 = *var_qdov_db6_slot;
        let mut var_qdov_db7: f64 = *var_qdov_db7_slot;
        let mut var_qdov_db8: f64 = *var_qdov_db8_slot;
        let mut var_qdov_db9: f64 = *var_qdov_db9_slot;
        let mut var_qdov_dn0: f64 = *var_qdov_dn0_slot;
        let mut var_qdov_dn1: f64 = *var_qdov_dn1_slot;
        let mut var_qdov_dn10: f64 = *var_qdov_dn10_slot;
        let mut var_qdov_dn11: f64 = *var_qdov_dn11_slot;
        let mut var_qdov_dn12: f64 = *var_qdov_dn12_slot;
        let mut var_qdov_dn13: f64 = *var_qdov_dn13_slot;
        let mut var_qdov_dn14: f64 = *var_qdov_dn14_slot;
        let mut var_qdov_dn15: f64 = *var_qdov_dn15_slot;
        let mut var_qdov_dn16: f64 = *var_qdov_dn16_slot;
        let mut var_qdov_dn17: f64 = *var_qdov_dn17_slot;
        let mut var_qdov_dn18: f64 = *var_qdov_dn18_slot;
        let mut var_qdov_dn19: f64 = *var_qdov_dn19_slot;
        let mut var_qdov_dn2: f64 = *var_qdov_dn2_slot;
        let mut var_qdov_dn20: f64 = *var_qdov_dn20_slot;
        let mut var_qdov_dn21: f64 = *var_qdov_dn21_slot;
        let mut var_qdov_dn22: f64 = *var_qdov_dn22_slot;
        let mut var_qdov_dn3: f64 = *var_qdov_dn3_slot;
        let mut var_qdov_dn4: f64 = *var_qdov_dn4_slot;
        let mut var_qdov_dn5: f64 = *var_qdov_dn5_slot;
        let mut var_qdov_dn6: f64 = *var_qdov_dn6_slot;
        let mut var_qdov_dn7: f64 = *var_qdov_dn7_slot;
        let mut var_qdov_dn8: f64 = *var_qdov_dn8_slot;
        let mut var_qdov_dn9: f64 = *var_qdov_dn9_slot;
        let mut var_qdsov: f64 = *var_qdsov_slot;
        let mut var_qdsov_db0: f64 = *var_qdsov_db0_slot;
        let mut var_qdsov_db1: f64 = *var_qdsov_db1_slot;
        let mut var_qdsov_db10: f64 = *var_qdsov_db10_slot;
        let mut var_qdsov_db11: f64 = *var_qdsov_db11_slot;
        let mut var_qdsov_db12: f64 = *var_qdsov_db12_slot;
        let mut var_qdsov_db13: f64 = *var_qdsov_db13_slot;
        let mut var_qdsov_db14: f64 = *var_qdsov_db14_slot;
        let mut var_qdsov_db15: f64 = *var_qdsov_db15_slot;
        let mut var_qdsov_db16: f64 = *var_qdsov_db16_slot;
        let mut var_qdsov_db17: f64 = *var_qdsov_db17_slot;
        let mut var_qdsov_db18: f64 = *var_qdsov_db18_slot;
        let mut var_qdsov_db19: f64 = *var_qdsov_db19_slot;
        let mut var_qdsov_db2: f64 = *var_qdsov_db2_slot;
        let mut var_qdsov_db20: f64 = *var_qdsov_db20_slot;
        let mut var_qdsov_db21: f64 = *var_qdsov_db21_slot;
        let mut var_qdsov_db22: f64 = *var_qdsov_db22_slot;
        let mut var_qdsov_db23: f64 = *var_qdsov_db23_slot;
        let mut var_qdsov_db24: f64 = *var_qdsov_db24_slot;
        let mut var_qdsov_db25: f64 = *var_qdsov_db25_slot;
        let mut var_qdsov_db26: f64 = *var_qdsov_db26_slot;
        let mut var_qdsov_db27: f64 = *var_qdsov_db27_slot;
        let mut var_qdsov_db28: f64 = *var_qdsov_db28_slot;
        let mut var_qdsov_db29: f64 = *var_qdsov_db29_slot;
        let mut var_qdsov_db3: f64 = *var_qdsov_db3_slot;
        let mut var_qdsov_db30: f64 = *var_qdsov_db30_slot;
        let mut var_qdsov_db31: f64 = *var_qdsov_db31_slot;
        let mut var_qdsov_db32: f64 = *var_qdsov_db32_slot;
        let mut var_qdsov_db33: f64 = *var_qdsov_db33_slot;
        let mut var_qdsov_db34: f64 = *var_qdsov_db34_slot;
        let mut var_qdsov_db35: f64 = *var_qdsov_db35_slot;
        let mut var_qdsov_db36: f64 = *var_qdsov_db36_slot;
        let mut var_qdsov_db37: f64 = *var_qdsov_db37_slot;
        let mut var_qdsov_db38: f64 = *var_qdsov_db38_slot;
        let mut var_qdsov_db39: f64 = *var_qdsov_db39_slot;
        let mut var_qdsov_db4: f64 = *var_qdsov_db4_slot;
        let mut var_qdsov_db40: f64 = *var_qdsov_db40_slot;
        let mut var_qdsov_db41: f64 = *var_qdsov_db41_slot;
        let mut var_qdsov_db42: f64 = *var_qdsov_db42_slot;
        let mut var_qdsov_db43: f64 = *var_qdsov_db43_slot;
        let mut var_qdsov_db44: f64 = *var_qdsov_db44_slot;
        let mut var_qdsov_db45: f64 = *var_qdsov_db45_slot;
        let mut var_qdsov_db46: f64 = *var_qdsov_db46_slot;
        let mut var_qdsov_db47: f64 = *var_qdsov_db47_slot;
        let mut var_qdsov_db48: f64 = *var_qdsov_db48_slot;
        let mut var_qdsov_db49: f64 = *var_qdsov_db49_slot;
        let mut var_qdsov_db5: f64 = *var_qdsov_db5_slot;
        let mut var_qdsov_db50: f64 = *var_qdsov_db50_slot;
        let mut var_qdsov_db51: f64 = *var_qdsov_db51_slot;
        let mut var_qdsov_db52: f64 = *var_qdsov_db52_slot;
        let mut var_qdsov_db53: f64 = *var_qdsov_db53_slot;
        let mut var_qdsov_db54: f64 = *var_qdsov_db54_slot;
        let mut var_qdsov_db55: f64 = *var_qdsov_db55_slot;
        let mut var_qdsov_db56: f64 = *var_qdsov_db56_slot;
        let mut var_qdsov_db6: f64 = *var_qdsov_db6_slot;
        let mut var_qdsov_db7: f64 = *var_qdsov_db7_slot;
        let mut var_qdsov_db8: f64 = *var_qdsov_db8_slot;
        let mut var_qdsov_db9: f64 = *var_qdsov_db9_slot;
        let mut var_qdsov_dn0: f64 = *var_qdsov_dn0_slot;
        let mut var_qdsov_dn1: f64 = *var_qdsov_dn1_slot;
        let mut var_qdsov_dn10: f64 = *var_qdsov_dn10_slot;
        let mut var_qdsov_dn11: f64 = *var_qdsov_dn11_slot;
        let mut var_qdsov_dn12: f64 = *var_qdsov_dn12_slot;
        let mut var_qdsov_dn13: f64 = *var_qdsov_dn13_slot;
        let mut var_qdsov_dn14: f64 = *var_qdsov_dn14_slot;
        let mut var_qdsov_dn15: f64 = *var_qdsov_dn15_slot;
        let mut var_qdsov_dn16: f64 = *var_qdsov_dn16_slot;
        let mut var_qdsov_dn17: f64 = *var_qdsov_dn17_slot;
        let mut var_qdsov_dn18: f64 = *var_qdsov_dn18_slot;
        let mut var_qdsov_dn19: f64 = *var_qdsov_dn19_slot;
        let mut var_qdsov_dn2: f64 = *var_qdsov_dn2_slot;
        let mut var_qdsov_dn20: f64 = *var_qdsov_dn20_slot;
        let mut var_qdsov_dn21: f64 = *var_qdsov_dn21_slot;
        let mut var_qdsov_dn22: f64 = *var_qdsov_dn22_slot;
        let mut var_qdsov_dn3: f64 = *var_qdsov_dn3_slot;
        let mut var_qdsov_dn4: f64 = *var_qdsov_dn4_slot;
        let mut var_qdsov_dn5: f64 = *var_qdsov_dn5_slot;
        let mut var_qdsov_dn6: f64 = *var_qdsov_dn6_slot;
        let mut var_qdsov_dn7: f64 = *var_qdsov_dn7_slot;
        let mut var_qdsov_dn8: f64 = *var_qdsov_dn8_slot;
        let mut var_qdsov_dn9: f64 = *var_qdsov_dn9_slot;

        let (assign31090_e49006, assign31090_e49006_d_n0, assign31090_e49006_d_n1, assign31090_e49006_d_n2, assign31090_e49006_d_n3, assign31090_e49006_d_n4, assign31090_e49006_d_n5, assign31090_e49006_d_n6, assign31090_e49006_d_n7, assign31090_e49006_d_n8, assign31090_e49006_d_n9, assign31090_e49006_d_n10, assign31090_e49006_d_n11, assign31090_e49006_d_n12, assign31090_e49006_d_n13, assign31090_e49006_d_n14, assign31090_e49006_d_n15, assign31090_e49006_d_n16, assign31090_e49006_d_n17, assign31090_e49006_d_n18, assign31090_e49006_d_n19, assign31090_e49006_d_n20, assign31090_e49006_d_n21, assign31090_e49006_d_n22, assign31090_e49006_d_b0, assign31090_e49006_d_b1, assign31090_e49006_d_b2, assign31090_e49006_d_b3, assign31090_e49006_d_b4, assign31090_e49006_d_b5, assign31090_e49006_d_b6, assign31090_e49006_d_b7, assign31090_e49006_d_b8, assign31090_e49006_d_b9, assign31090_e49006_d_b10, assign31090_e49006_d_b11, assign31090_e49006_d_b12, assign31090_e49006_d_b13, assign31090_e49006_d_b14, assign31090_e49006_d_b15, assign31090_e49006_d_b16, assign31090_e49006_d_b17, assign31090_e49006_d_b18, assign31090_e49006_d_b19, assign31090_e49006_d_b20, assign31090_e49006_d_b21, assign31090_e49006_d_b22, assign31090_e49006_d_b23, assign31090_e49006_d_b24, assign31090_e49006_d_b25, assign31090_e49006_d_b26, assign31090_e49006_d_b27, assign31090_e49006_d_b28, assign31090_e49006_d_b29, assign31090_e49006_d_b30, assign31090_e49006_d_b31, assign31090_e49006_d_b32, assign31090_e49006_d_b33, assign31090_e49006_d_b34, assign31090_e49006_d_b35, assign31090_e49006_d_b36, assign31090_e49006_d_b37, assign31090_e49006_d_b38, assign31090_e49006_d_b39, assign31090_e49006_d_b40, assign31090_e49006_d_b41, assign31090_e49006_d_b42, assign31090_e49006_d_b43, assign31090_e49006_d_b44, assign31090_e49006_d_b45, assign31090_e49006_d_b46, assign31090_e49006_d_b47, assign31090_e49006_d_b48, assign31090_e49006_d_b49, assign31090_e49006_d_b50, assign31090_e49006_d_b51, assign31090_e49006_d_b52, assign31090_e49006_d_b53, assign31090_e49006_d_b54,) = {
    if (var_guard524 == 0.0) {
        let assign31090_e49002: f64 = (var_cgdvar).max(0.0);
        let assign31090_e49004: f64 = (assign31090_e49002 * (nv1 - nv0));
        (assign31090_e49004, ((if var_cgdvar >= 0.0 { var_cgdvar_dn0 } else { 0.0 } * (nv1 - nv0)) + (-assign31090_e49002)), ((if var_cgdvar >= 0.0 { var_cgdvar_dn1 } else { 0.0 } * (nv1 - nv0)) + assign31090_e49002), (if var_cgdvar >= 0.0 { var_cgdvar_dn2 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn3 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn4 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn5 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn6 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn7 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn8 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn9 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn10 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn11 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn12 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn13 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn14 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn15 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn16 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn17 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn18 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn19 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn20 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn21 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_dn22 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db0 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db1 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db2 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db3 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db4 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db5 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db6 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db7 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db8 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db9 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db10 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db11 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db12 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db13 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db14 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db15 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db16 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db17 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db18 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db19 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db20 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db21 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db22 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db23 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db24 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db25 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db26 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db27 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db28 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db29 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db30 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db31 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db32 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db33 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db34 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db35 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db36 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db37 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db38 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db39 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db40 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db41 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db42 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db43 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db44 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db45 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db46 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db47 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db48 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db49 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db50 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db51 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db52 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db53 } else { 0.0 } * (nv1 - nv0)), (if var_cgdvar >= 0.0 { var_cgdvar_db54 } else { 0.0 } * (nv1 - nv0)),)
    } else {
        (var_qdov, var_qdov_dn0, var_qdov_dn1, var_qdov_dn2, var_qdov_dn3, var_qdov_dn4, var_qdov_dn5, var_qdov_dn6, var_qdov_dn7, var_qdov_dn8, var_qdov_dn9, var_qdov_dn10, var_qdov_dn11, var_qdov_dn12, var_qdov_dn13, var_qdov_dn14, var_qdov_dn15, var_qdov_dn16, var_qdov_dn17, var_qdov_dn18, var_qdov_dn19, var_qdov_dn20, var_qdov_dn21, var_qdov_dn22, var_qdov_db0, var_qdov_db1, var_qdov_db2, var_qdov_db3, var_qdov_db4, var_qdov_db5, var_qdov_db6, var_qdov_db7, var_qdov_db8, var_qdov_db9, var_qdov_db10, var_qdov_db11, var_qdov_db12, var_qdov_db13, var_qdov_db14, var_qdov_db15, var_qdov_db16, var_qdov_db17, var_qdov_db18, var_qdov_db19, var_qdov_db20, var_qdov_db21, var_qdov_db22, var_qdov_db23, var_qdov_db24, var_qdov_db25, var_qdov_db26, var_qdov_db27, var_qdov_db28, var_qdov_db29, var_qdov_db30, var_qdov_db31, var_qdov_db32, var_qdov_db33, var_qdov_db34, var_qdov_db35, var_qdov_db36, var_qdov_db37, var_qdov_db38, var_qdov_db39, var_qdov_db40, var_qdov_db41, var_qdov_db42, var_qdov_db43, var_qdov_db44, var_qdov_db45, var_qdov_db46, var_qdov_db47, var_qdov_db48, var_qdov_db49, var_qdov_db50, var_qdov_db51, var_qdov_db52, var_qdov_db53, var_qdov_db54,)
    }
};
        var_qdov = assign31090_e49006;
        var_qdov_dn0 = assign31090_e49006_d_n0;
        var_qdov_dn1 = assign31090_e49006_d_n1;
        var_qdov_dn2 = assign31090_e49006_d_n2;
        var_qdov_dn3 = assign31090_e49006_d_n3;
        var_qdov_dn4 = assign31090_e49006_d_n4;
        var_qdov_dn5 = assign31090_e49006_d_n5;
        var_qdov_dn6 = assign31090_e49006_d_n6;
        var_qdov_dn7 = assign31090_e49006_d_n7;
        var_qdov_dn8 = assign31090_e49006_d_n8;
        var_qdov_dn9 = assign31090_e49006_d_n9;
        var_qdov_dn10 = assign31090_e49006_d_n10;
        var_qdov_dn11 = assign31090_e49006_d_n11;
        var_qdov_dn12 = assign31090_e49006_d_n12;
        var_qdov_dn13 = assign31090_e49006_d_n13;
        var_qdov_dn14 = assign31090_e49006_d_n14;
        var_qdov_dn15 = assign31090_e49006_d_n15;
        var_qdov_dn16 = assign31090_e49006_d_n16;
        var_qdov_dn17 = assign31090_e49006_d_n17;
        var_qdov_dn18 = assign31090_e49006_d_n18;
        var_qdov_dn19 = assign31090_e49006_d_n19;
        var_qdov_dn20 = assign31090_e49006_d_n20;
        var_qdov_dn21 = assign31090_e49006_d_n21;
        var_qdov_dn22 = assign31090_e49006_d_n22;
        var_qdov_db0 = assign31090_e49006_d_b0;
        var_qdov_db1 = assign31090_e49006_d_b1;
        var_qdov_db2 = assign31090_e49006_d_b2;
        var_qdov_db3 = assign31090_e49006_d_b3;
        var_qdov_db4 = assign31090_e49006_d_b4;
        var_qdov_db5 = assign31090_e49006_d_b5;
        var_qdov_db6 = assign31090_e49006_d_b6;
        var_qdov_db7 = assign31090_e49006_d_b7;
        var_qdov_db8 = assign31090_e49006_d_b8;
        var_qdov_db9 = assign31090_e49006_d_b9;
        var_qdov_db10 = assign31090_e49006_d_b10;
        var_qdov_db11 = assign31090_e49006_d_b11;
        var_qdov_db12 = assign31090_e49006_d_b12;
        var_qdov_db13 = assign31090_e49006_d_b13;
        var_qdov_db14 = assign31090_e49006_d_b14;
        var_qdov_db15 = assign31090_e49006_d_b15;
        var_qdov_db16 = assign31090_e49006_d_b16;
        var_qdov_db17 = assign31090_e49006_d_b17;
        var_qdov_db18 = assign31090_e49006_d_b18;
        var_qdov_db19 = assign31090_e49006_d_b19;
        var_qdov_db20 = assign31090_e49006_d_b20;
        var_qdov_db21 = assign31090_e49006_d_b21;
        var_qdov_db22 = assign31090_e49006_d_b22;
        var_qdov_db23 = assign31090_e49006_d_b23;
        var_qdov_db24 = assign31090_e49006_d_b24;
        var_qdov_db25 = assign31090_e49006_d_b25;
        var_qdov_db26 = assign31090_e49006_d_b26;
        var_qdov_db27 = assign31090_e49006_d_b27;
        var_qdov_db28 = assign31090_e49006_d_b28;
        var_qdov_db29 = assign31090_e49006_d_b29;
        var_qdov_db30 = assign31090_e49006_d_b30;
        var_qdov_db31 = assign31090_e49006_d_b31;
        var_qdov_db32 = assign31090_e49006_d_b32;
        var_qdov_db33 = assign31090_e49006_d_b33;
        var_qdov_db34 = assign31090_e49006_d_b34;
        var_qdov_db35 = assign31090_e49006_d_b35;
        var_qdov_db36 = assign31090_e49006_d_b36;
        var_qdov_db37 = assign31090_e49006_d_b37;
        var_qdov_db38 = assign31090_e49006_d_b38;
        var_qdov_db39 = assign31090_e49006_d_b39;
        var_qdov_db40 = assign31090_e49006_d_b40;
        var_qdov_db41 = assign31090_e49006_d_b41;
        var_qdov_db42 = assign31090_e49006_d_b42;
        var_qdov_db43 = assign31090_e49006_d_b43;
        var_qdov_db44 = assign31090_e49006_d_b44;
        var_qdov_db45 = assign31090_e49006_d_b45;
        var_qdov_db46 = assign31090_e49006_d_b46;
        var_qdov_db47 = assign31090_e49006_d_b47;
        var_qdov_db48 = assign31090_e49006_d_b48;
        var_qdov_db49 = assign31090_e49006_d_b49;
        var_qdov_db50 = assign31090_e49006_d_b50;
        var_qdov_db51 = assign31090_e49006_d_b51;
        var_qdov_db52 = assign31090_e49006_d_b52;
        var_qdov_db53 = assign31090_e49006_d_b53;
        var_qdov_db54 = assign31090_e49006_d_b54;
        var_qdov_db55 = 0.0;
        var_qdov_db56 = 0.0;

        let assign31100_e49009: f64 = (p.p4 * p.p5);
        let assign31100_e49011: f64 = (assign31100_e49009 * p.p212);
        let assign31100_e49013: f64 = (assign31100_e49011 * (nv0 - nv2));
        var_qdsov = assign31100_e49013;
        var_qdsov_dn0 = assign31100_e49011;
        var_qdsov_dn1 = 0.0;
        var_qdsov_dn2 = (-assign31100_e49011);
        var_qdsov_dn3 = 0.0;
        var_qdsov_dn4 = 0.0;
        var_qdsov_dn5 = 0.0;
        var_qdsov_dn6 = 0.0;
        var_qdsov_dn7 = 0.0;
        var_qdsov_dn8 = 0.0;
        var_qdsov_dn9 = 0.0;
        var_qdsov_dn10 = 0.0;
        var_qdsov_dn11 = 0.0;
        var_qdsov_dn12 = 0.0;
        var_qdsov_dn13 = 0.0;
        var_qdsov_dn14 = 0.0;
        var_qdsov_dn15 = 0.0;
        var_qdsov_dn16 = 0.0;
        var_qdsov_dn17 = 0.0;
        var_qdsov_dn18 = 0.0;
        var_qdsov_dn19 = 0.0;
        var_qdsov_dn20 = 0.0;
        var_qdsov_dn21 = 0.0;
        var_qdsov_dn22 = 0.0;
        var_qdsov_db0 = 0.0;
        var_qdsov_db1 = 0.0;
        var_qdsov_db2 = 0.0;
        var_qdsov_db3 = 0.0;
        var_qdsov_db4 = 0.0;
        var_qdsov_db5 = 0.0;
        var_qdsov_db6 = 0.0;
        var_qdsov_db7 = 0.0;
        var_qdsov_db8 = 0.0;
        var_qdsov_db9 = 0.0;
        var_qdsov_db10 = 0.0;
        var_qdsov_db11 = 0.0;
        var_qdsov_db12 = 0.0;
        var_qdsov_db13 = 0.0;
        var_qdsov_db14 = 0.0;
        var_qdsov_db15 = 0.0;
        var_qdsov_db16 = 0.0;
        var_qdsov_db17 = 0.0;
        var_qdsov_db18 = 0.0;
        var_qdsov_db19 = 0.0;
        var_qdsov_db20 = 0.0;
        var_qdsov_db21 = 0.0;
        var_qdsov_db22 = 0.0;
        var_qdsov_db23 = 0.0;
        var_qdsov_db24 = 0.0;
        var_qdsov_db25 = 0.0;
        var_qdsov_db26 = 0.0;
        var_qdsov_db27 = 0.0;
        var_qdsov_db28 = 0.0;
        var_qdsov_db29 = 0.0;
        var_qdsov_db30 = 0.0;
        var_qdsov_db31 = 0.0;
        var_qdsov_db32 = 0.0;
        var_qdsov_db33 = 0.0;
        var_qdsov_db34 = 0.0;
        var_qdsov_db35 = 0.0;
        var_qdsov_db36 = 0.0;
        var_qdsov_db37 = 0.0;
        var_qdsov_db38 = 0.0;
        var_qdsov_db39 = 0.0;
        var_qdsov_db40 = 0.0;
        var_qdsov_db41 = 0.0;
        var_qdsov_db42 = 0.0;
        var_qdsov_db43 = 0.0;
        var_qdsov_db44 = 0.0;
        var_qdsov_db45 = 0.0;
        var_qdsov_db46 = 0.0;
        var_qdsov_db47 = 0.0;
        var_qdsov_db48 = 0.0;
        var_qdsov_db49 = 0.0;
        var_qdsov_db50 = 0.0;
        var_qdsov_db51 = 0.0;
        var_qdsov_db52 = 0.0;
        var_qdsov_db53 = 0.0;
        var_qdsov_db54 = 0.0;
        var_qdsov_db55 = 0.0;
        var_qdsov_db56 = 0.0;

        let assign31150_e49030: f64 = (p.p4 * p.p5);
        let assign31150_e49032: f64 = (assign31150_e49030 * p.p215);
        let assign31150_e49034: f64 = (assign31150_e49032 * (nv3 - nv0));
        var_qbdov = assign31150_e49034;
        var_qbdov_dn0 = (-assign31150_e49032);
        var_qbdov_dn1 = 0.0;
        var_qbdov_dn2 = 0.0;
        var_qbdov_dn3 = assign31150_e49032;
        var_qbdov_dn4 = 0.0;
        var_qbdov_dn5 = 0.0;
        var_qbdov_dn6 = 0.0;
        var_qbdov_dn7 = 0.0;
        var_qbdov_dn8 = 0.0;
        var_qbdov_dn9 = 0.0;
        var_qbdov_dn10 = 0.0;
        var_qbdov_dn11 = 0.0;
        var_qbdov_dn12 = 0.0;
        var_qbdov_dn13 = 0.0;
        var_qbdov_dn14 = 0.0;
        var_qbdov_dn15 = 0.0;
        var_qbdov_dn16 = 0.0;
        var_qbdov_dn17 = 0.0;
        var_qbdov_dn18 = 0.0;
        var_qbdov_dn19 = 0.0;
        var_qbdov_dn20 = 0.0;
        var_qbdov_dn21 = 0.0;
        var_qbdov_dn22 = 0.0;
        var_qbdov_db0 = 0.0;
        var_qbdov_db1 = 0.0;
        var_qbdov_db2 = 0.0;
        var_qbdov_db3 = 0.0;
        var_qbdov_db4 = 0.0;
        var_qbdov_db5 = 0.0;
        var_qbdov_db6 = 0.0;
        var_qbdov_db7 = 0.0;
        var_qbdov_db8 = 0.0;
        var_qbdov_db9 = 0.0;
        var_qbdov_db10 = 0.0;
        var_qbdov_db11 = 0.0;
        var_qbdov_db12 = 0.0;
        var_qbdov_db13 = 0.0;
        var_qbdov_db14 = 0.0;
        var_qbdov_db15 = 0.0;
        var_qbdov_db16 = 0.0;
        var_qbdov_db17 = 0.0;
        var_qbdov_db18 = 0.0;
        var_qbdov_db19 = 0.0;
        var_qbdov_db20 = 0.0;
        var_qbdov_db21 = 0.0;
        var_qbdov_db22 = 0.0;
        var_qbdov_db23 = 0.0;
        var_qbdov_db24 = 0.0;
        var_qbdov_db25 = 0.0;
        var_qbdov_db26 = 0.0;
        var_qbdov_db27 = 0.0;
        var_qbdov_db28 = 0.0;
        var_qbdov_db29 = 0.0;
        var_qbdov_db30 = 0.0;
        var_qbdov_db31 = 0.0;
        var_qbdov_db32 = 0.0;
        var_qbdov_db33 = 0.0;
        var_qbdov_db34 = 0.0;
        var_qbdov_db35 = 0.0;
        var_qbdov_db36 = 0.0;
        var_qbdov_db37 = 0.0;
        var_qbdov_db38 = 0.0;
        var_qbdov_db39 = 0.0;
        var_qbdov_db40 = 0.0;
        var_qbdov_db41 = 0.0;
        var_qbdov_db42 = 0.0;
        var_qbdov_db43 = 0.0;
        var_qbdov_db44 = 0.0;
        var_qbdov_db45 = 0.0;
        var_qbdov_db46 = 0.0;
        var_qbdov_db47 = 0.0;
        var_qbdov_db48 = 0.0;
        var_qbdov_db49 = 0.0;
        var_qbdov_db50 = 0.0;
        var_qbdov_db51 = 0.0;
        var_qbdov_db52 = 0.0;
        var_qbdov_db53 = 0.0;
        var_qbdov_db54 = 0.0;
        var_qbdov_db55 = 0.0;
        var_qbdov_db56 = 0.0;

        let assign31160_e49037: f64 = (p.p4 * p.p5);
        let assign31160_e49039: f64 = (assign31160_e49037 * p.p216);
        let assign31160_e49041: f64 = (assign31160_e49039 * (nv3 - nv2));
        var_qbsov = assign31160_e49041;
        var_qbsov_dn0 = 0.0;
        var_qbsov_dn1 = 0.0;
        var_qbsov_dn2 = (-assign31160_e49039);
        var_qbsov_dn3 = assign31160_e49039;
        var_qbsov_dn4 = 0.0;
        var_qbsov_dn5 = 0.0;
        var_qbsov_dn6 = 0.0;
        var_qbsov_dn7 = 0.0;
        var_qbsov_dn8 = 0.0;
        var_qbsov_dn9 = 0.0;
        var_qbsov_dn10 = 0.0;
        var_qbsov_dn11 = 0.0;
        var_qbsov_dn12 = 0.0;
        var_qbsov_dn13 = 0.0;
        var_qbsov_dn14 = 0.0;
        var_qbsov_dn15 = 0.0;
        var_qbsov_dn16 = 0.0;
        var_qbsov_dn17 = 0.0;
        var_qbsov_dn18 = 0.0;
        var_qbsov_dn19 = 0.0;
        var_qbsov_dn20 = 0.0;
        var_qbsov_dn21 = 0.0;
        var_qbsov_dn22 = 0.0;
        var_qbsov_db0 = 0.0;
        var_qbsov_db1 = 0.0;
        var_qbsov_db2 = 0.0;
        var_qbsov_db3 = 0.0;
        var_qbsov_db4 = 0.0;
        var_qbsov_db5 = 0.0;
        var_qbsov_db6 = 0.0;
        var_qbsov_db7 = 0.0;
        var_qbsov_db8 = 0.0;
        var_qbsov_db9 = 0.0;
        var_qbsov_db10 = 0.0;
        var_qbsov_db11 = 0.0;
        var_qbsov_db12 = 0.0;
        var_qbsov_db13 = 0.0;
        var_qbsov_db14 = 0.0;
        var_qbsov_db15 = 0.0;
        var_qbsov_db16 = 0.0;
        var_qbsov_db17 = 0.0;
        var_qbsov_db18 = 0.0;
        var_qbsov_db19 = 0.0;
        var_qbsov_db20 = 0.0;
        var_qbsov_db21 = 0.0;
        var_qbsov_db22 = 0.0;
        var_qbsov_db23 = 0.0;
        var_qbsov_db24 = 0.0;
        var_qbsov_db25 = 0.0;
        var_qbsov_db26 = 0.0;
        var_qbsov_db27 = 0.0;
        var_qbsov_db28 = 0.0;
        var_qbsov_db29 = 0.0;
        var_qbsov_db30 = 0.0;
        var_qbsov_db31 = 0.0;
        var_qbsov_db32 = 0.0;
        var_qbsov_db33 = 0.0;
        var_qbsov_db34 = 0.0;
        var_qbsov_db35 = 0.0;
        var_qbsov_db36 = 0.0;
        var_qbsov_db37 = 0.0;
        var_qbsov_db38 = 0.0;
        var_qbsov_db39 = 0.0;
        var_qbsov_db40 = 0.0;
        var_qbsov_db41 = 0.0;
        var_qbsov_db42 = 0.0;
        var_qbsov_db43 = 0.0;
        var_qbsov_db44 = 0.0;
        var_qbsov_db45 = 0.0;
        var_qbsov_db46 = 0.0;
        var_qbsov_db47 = 0.0;
        var_qbsov_db48 = 0.0;
        var_qbsov_db49 = 0.0;
        var_qbsov_db50 = 0.0;
        var_qbsov_db51 = 0.0;
        var_qbsov_db52 = 0.0;
        var_qbsov_db53 = 0.0;
        var_qbsov_db54 = 0.0;
        var_qbsov_db55 = 0.0;
        var_qbsov_db56 = 0.0;

        let assign31170_e49044: f64 = (p.p4 * p.p5);
        let assign31170_e49046: f64 = (assign31170_e49044 * p.p217);
        let assign31170_e49048: f64 = (assign31170_e49046 * (nv3 - nv1));
        var_qbgov = assign31170_e49048;
        var_qbgov_dn0 = 0.0;
        var_qbgov_dn1 = (-assign31170_e49046);
        var_qbgov_dn2 = 0.0;
        var_qbgov_dn3 = assign31170_e49046;
        var_qbgov_dn4 = 0.0;
        var_qbgov_dn5 = 0.0;
        var_qbgov_dn6 = 0.0;
        var_qbgov_dn7 = 0.0;
        var_qbgov_dn8 = 0.0;
        var_qbgov_dn9 = 0.0;
        var_qbgov_dn10 = 0.0;
        var_qbgov_dn11 = 0.0;
        var_qbgov_dn12 = 0.0;
        var_qbgov_dn13 = 0.0;
        var_qbgov_dn14 = 0.0;
        var_qbgov_dn15 = 0.0;
        var_qbgov_dn16 = 0.0;
        var_qbgov_dn17 = 0.0;
        var_qbgov_dn18 = 0.0;
        var_qbgov_dn19 = 0.0;
        var_qbgov_dn20 = 0.0;
        var_qbgov_dn21 = 0.0;
        var_qbgov_dn22 = 0.0;
        var_qbgov_db0 = 0.0;
        var_qbgov_db1 = 0.0;
        var_qbgov_db2 = 0.0;
        var_qbgov_db3 = 0.0;
        var_qbgov_db4 = 0.0;
        var_qbgov_db5 = 0.0;
        var_qbgov_db6 = 0.0;
        var_qbgov_db7 = 0.0;
        var_qbgov_db8 = 0.0;
        var_qbgov_db9 = 0.0;
        var_qbgov_db10 = 0.0;
        var_qbgov_db11 = 0.0;
        var_qbgov_db12 = 0.0;
        var_qbgov_db13 = 0.0;
        var_qbgov_db14 = 0.0;
        var_qbgov_db15 = 0.0;
        var_qbgov_db16 = 0.0;
        var_qbgov_db17 = 0.0;
        var_qbgov_db18 = 0.0;
        var_qbgov_db19 = 0.0;
        var_qbgov_db20 = 0.0;
        var_qbgov_db21 = 0.0;
        var_qbgov_db22 = 0.0;
        var_qbgov_db23 = 0.0;
        var_qbgov_db24 = 0.0;
        var_qbgov_db25 = 0.0;
        var_qbgov_db26 = 0.0;
        var_qbgov_db27 = 0.0;
        var_qbgov_db28 = 0.0;
        var_qbgov_db29 = 0.0;
        var_qbgov_db30 = 0.0;
        var_qbgov_db31 = 0.0;
        var_qbgov_db32 = 0.0;
        var_qbgov_db33 = 0.0;
        var_qbgov_db34 = 0.0;
        var_qbgov_db35 = 0.0;
        var_qbgov_db36 = 0.0;
        var_qbgov_db37 = 0.0;
        var_qbgov_db38 = 0.0;
        var_qbgov_db39 = 0.0;
        var_qbgov_db40 = 0.0;
        var_qbgov_db41 = 0.0;
        var_qbgov_db42 = 0.0;
        var_qbgov_db43 = 0.0;
        var_qbgov_db44 = 0.0;
        var_qbgov_db45 = 0.0;
        var_qbgov_db46 = 0.0;
        var_qbgov_db47 = 0.0;
        var_qbgov_db48 = 0.0;
        var_qbgov_db49 = 0.0;
        var_qbgov_db50 = 0.0;
        var_qbgov_db51 = 0.0;
        var_qbgov_db52 = 0.0;
        var_qbgov_db53 = 0.0;
        var_qbgov_db54 = 0.0;
        var_qbgov_db55 = 0.0;
        var_qbgov_db56 = 0.0;

        s.store_offset_scaled_ad(375, A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), ((1.0 / (var_tnom)) * (p.p285)), (((((-1.0)) * (p.p285))) + (p.p279)));

        s.store_offset_scaled_ad(373, A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), ((1.0 / (var_tnom)) * (p.p283)), (((((-1.0)) * (p.p283))) + (p.p275)));

        s.store_scale_ad(377, A::exp_scaled_input(A::scale_offset(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 1.0 / (var_tnom), (-1.0)), p.p281), p.p277);

        s.store_offset_scaled_ad(376, A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), ((1.0 / (var_tnom)) * (p.p286)), (((((-1.0)) * (p.p286))) + (p.p280)));

        s.store_offset_scaled_ad(374, A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), ((1.0 / (var_tnom)) * (p.p284)), (((((-1.0)) * (p.p284))) + (p.p276)));

        s.store_scale_ad(378, A::exp_scaled_input(A::scale_offset(A::from_derivatives(var_tdev, [var_tdev_dn0, var_tdev_dn1, var_tdev_dn2, var_tdev_dn3, var_tdev_dn4, var_tdev_dn5, var_tdev_dn6, var_tdev_dn7, var_tdev_dn8, var_tdev_dn9, var_tdev_dn10, var_tdev_dn11, var_tdev_dn12, var_tdev_dn13, var_tdev_dn14, var_tdev_dn15, var_tdev_dn16, var_tdev_dn17, var_tdev_dn18, var_tdev_dn19, var_tdev_dn20, var_tdev_dn21, var_tdev_dn22], [var_tdev_db0, var_tdev_db1, var_tdev_db2, var_tdev_db3, var_tdev_db4, var_tdev_db5, var_tdev_db6, var_tdev_db7, var_tdev_db8, var_tdev_db9, var_tdev_db10, var_tdev_db11, var_tdev_db12, var_tdev_db13, var_tdev_db14, var_tdev_db15, var_tdev_db16, var_tdev_db17, var_tdev_db18, var_tdev_db19, var_tdev_db20, var_tdev_db21, var_tdev_db22, var_tdev_db23, var_tdev_db24, var_tdev_db25, var_tdev_db26, var_tdev_db27, var_tdev_db28, var_tdev_db29, var_tdev_db30, var_tdev_db31, var_tdev_db32, var_tdev_db33, var_tdev_db34, var_tdev_db35, var_tdev_db36, var_tdev_db37, var_tdev_db38, var_tdev_db39, var_tdev_db40, var_tdev_db41, var_tdev_db42, var_tdev_db43, var_tdev_db44, var_tdev_db45, var_tdev_db46, var_tdev_db47, var_tdev_db48, var_tdev_db49, var_tdev_db50, var_tdev_db51, var_tdev_db52, var_tdev_db53, var_tdev_db54, var_tdev_db55, var_tdev_db56]), 1.0 / (var_tnom), (-1.0)), p.p282), p.p278);

        s.store_scale(137, 378, (p.p4 * p.p5));

        s.store_max_with_scalar_ad(371, A::sub(A::voltage(ctx, nodes, Some(0), Some(3)), s.ad_value(376)), 0.0);

        s.b[559] = (s.v[137] > 0.0);
        s.store_scalar(559, if s.b[559] { 1.0 } else { 0.0 });

        s.b[560] = (s.v[371] > 0.0);
        s.store_scalar(560, if s.b[560] { 1.0 } else { 0.0 });

        if (s.b[559] && s.b[560]) {
            s.store_div_ad_rhs(354, 371, A::mul(s.ad_value(374), s.ad_value(36)));
        }

        s.b[561] = (s.v[354] > 80.0);
        s.store_scalar(561, if s.b[561] { 1.0 } else { 0.0 });

        if ((s.b[559] && s.b[560]) && s.b[561]) {
            s.store_offset(355, 354, (((-80.0)) + (1.0)));
            s.store_scalar(354, 80.0);
        }

        if ((s.b[559] && s.b[560]) && (!s.b[561])) {
            s.store_scalar(355, 1.0);
        }

        if (s.b[559] && s.b[560]) {
            s.store_mul_exp_rhs(355, 355, 354);
            s.store_mul_offset_rhs(369, 137, 355, (-1.0));
        }

        if (s.b[559] && (!s.b[560])) {
            s.store_div_ad_rhs(354, 371, A::mul(s.ad_value(374), s.ad_value(36)));
        }

        s.b[562] = (s.v[354] > 80.0);
        s.store_scalar(562, if s.b[562] { 1.0 } else { 0.0 });

        if ((s.b[559] && (!s.b[560])) && s.b[562]) {
            s.store_offset(355, 354, (((-80.0)) + (1.0)));
            s.store_scalar(354, 80.0);
        }

        if ((s.b[559] && (!s.b[560])) && (!s.b[562])) {
            s.store_scalar(355, 1.0);
        }

        if (s.b[559] && (!s.b[560])) {
            s.store_mul_exp_rhs(355, 355, 354);
            s.store_mul_offset_rhs(369, 137, 355, (-1.0));
        }

        if (!s.b[559]) {
            s.store_scalar(369, 0.0);
        }

        *var_qbdov_slot = var_qbdov;
        *var_qbdov_db0_slot = var_qbdov_db0;
        *var_qbdov_db1_slot = var_qbdov_db1;
        *var_qbdov_db10_slot = var_qbdov_db10;
        *var_qbdov_db11_slot = var_qbdov_db11;
        *var_qbdov_db12_slot = var_qbdov_db12;
        *var_qbdov_db13_slot = var_qbdov_db13;
        *var_qbdov_db14_slot = var_qbdov_db14;
        *var_qbdov_db15_slot = var_qbdov_db15;
        *var_qbdov_db16_slot = var_qbdov_db16;
        *var_qbdov_db17_slot = var_qbdov_db17;
        *var_qbdov_db18_slot = var_qbdov_db18;
        *var_qbdov_db19_slot = var_qbdov_db19;
        *var_qbdov_db2_slot = var_qbdov_db2;
        *var_qbdov_db20_slot = var_qbdov_db20;
        *var_qbdov_db21_slot = var_qbdov_db21;
        *var_qbdov_db22_slot = var_qbdov_db22;
        *var_qbdov_db23_slot = var_qbdov_db23;
        *var_qbdov_db24_slot = var_qbdov_db24;
        *var_qbdov_db25_slot = var_qbdov_db25;
        *var_qbdov_db26_slot = var_qbdov_db26;
        *var_qbdov_db27_slot = var_qbdov_db27;
        *var_qbdov_db28_slot = var_qbdov_db28;
        *var_qbdov_db29_slot = var_qbdov_db29;
        *var_qbdov_db3_slot = var_qbdov_db3;
        *var_qbdov_db30_slot = var_qbdov_db30;
        *var_qbdov_db31_slot = var_qbdov_db31;
        *var_qbdov_db32_slot = var_qbdov_db32;
        *var_qbdov_db33_slot = var_qbdov_db33;
        *var_qbdov_db34_slot = var_qbdov_db34;
        *var_qbdov_db35_slot = var_qbdov_db35;
        *var_qbdov_db36_slot = var_qbdov_db36;
        *var_qbdov_db37_slot = var_qbdov_db37;
        *var_qbdov_db38_slot = var_qbdov_db38;
        *var_qbdov_db39_slot = var_qbdov_db39;
        *var_qbdov_db4_slot = var_qbdov_db4;
        *var_qbdov_db40_slot = var_qbdov_db40;
        *var_qbdov_db41_slot = var_qbdov_db41;
        *var_qbdov_db42_slot = var_qbdov_db42;
        *var_qbdov_db43_slot = var_qbdov_db43;
        *var_qbdov_db44_slot = var_qbdov_db44;
        *var_qbdov_db45_slot = var_qbdov_db45;
        *var_qbdov_db46_slot = var_qbdov_db46;
        *var_qbdov_db47_slot = var_qbdov_db47;
        *var_qbdov_db48_slot = var_qbdov_db48;
        *var_qbdov_db49_slot = var_qbdov_db49;
        *var_qbdov_db5_slot = var_qbdov_db5;
        *var_qbdov_db50_slot = var_qbdov_db50;
        *var_qbdov_db51_slot = var_qbdov_db51;
        *var_qbdov_db52_slot = var_qbdov_db52;
        *var_qbdov_db53_slot = var_qbdov_db53;
        *var_qbdov_db54_slot = var_qbdov_db54;
        *var_qbdov_db55_slot = var_qbdov_db55;
        *var_qbdov_db56_slot = var_qbdov_db56;
        *var_qbdov_db6_slot = var_qbdov_db6;
        *var_qbdov_db7_slot = var_qbdov_db7;
        *var_qbdov_db8_slot = var_qbdov_db8;
        *var_qbdov_db9_slot = var_qbdov_db9;
        *var_qbdov_dn0_slot = var_qbdov_dn0;
        *var_qbdov_dn1_slot = var_qbdov_dn1;
        *var_qbdov_dn10_slot = var_qbdov_dn10;
        *var_qbdov_dn11_slot = var_qbdov_dn11;
        *var_qbdov_dn12_slot = var_qbdov_dn12;
        *var_qbdov_dn13_slot = var_qbdov_dn13;
        *var_qbdov_dn14_slot = var_qbdov_dn14;
        *var_qbdov_dn15_slot = var_qbdov_dn15;
        *var_qbdov_dn16_slot = var_qbdov_dn16;
        *var_qbdov_dn17_slot = var_qbdov_dn17;
        *var_qbdov_dn18_slot = var_qbdov_dn18;
        *var_qbdov_dn19_slot = var_qbdov_dn19;
        *var_qbdov_dn2_slot = var_qbdov_dn2;
        *var_qbdov_dn20_slot = var_qbdov_dn20;
        *var_qbdov_dn21_slot = var_qbdov_dn21;
        *var_qbdov_dn22_slot = var_qbdov_dn22;
        *var_qbdov_dn3_slot = var_qbdov_dn3;
        *var_qbdov_dn4_slot = var_qbdov_dn4;
        *var_qbdov_dn5_slot = var_qbdov_dn5;
        *var_qbdov_dn6_slot = var_qbdov_dn6;
        *var_qbdov_dn7_slot = var_qbdov_dn7;
        *var_qbdov_dn8_slot = var_qbdov_dn8;
        *var_qbdov_dn9_slot = var_qbdov_dn9;
        *var_qbgov_slot = var_qbgov;
        *var_qbgov_db0_slot = var_qbgov_db0;
        *var_qbgov_db1_slot = var_qbgov_db1;
        *var_qbgov_db10_slot = var_qbgov_db10;
        *var_qbgov_db11_slot = var_qbgov_db11;
        *var_qbgov_db12_slot = var_qbgov_db12;
        *var_qbgov_db13_slot = var_qbgov_db13;
        *var_qbgov_db14_slot = var_qbgov_db14;
        *var_qbgov_db15_slot = var_qbgov_db15;
        *var_qbgov_db16_slot = var_qbgov_db16;
        *var_qbgov_db17_slot = var_qbgov_db17;
        *var_qbgov_db18_slot = var_qbgov_db18;
        *var_qbgov_db19_slot = var_qbgov_db19;
        *var_qbgov_db2_slot = var_qbgov_db2;
        *var_qbgov_db20_slot = var_qbgov_db20;
        *var_qbgov_db21_slot = var_qbgov_db21;
        *var_qbgov_db22_slot = var_qbgov_db22;
        *var_qbgov_db23_slot = var_qbgov_db23;
        *var_qbgov_db24_slot = var_qbgov_db24;
        *var_qbgov_db25_slot = var_qbgov_db25;
        *var_qbgov_db26_slot = var_qbgov_db26;
        *var_qbgov_db27_slot = var_qbgov_db27;
        *var_qbgov_db28_slot = var_qbgov_db28;
        *var_qbgov_db29_slot = var_qbgov_db29;
        *var_qbgov_db3_slot = var_qbgov_db3;
        *var_qbgov_db30_slot = var_qbgov_db30;
        *var_qbgov_db31_slot = var_qbgov_db31;
        *var_qbgov_db32_slot = var_qbgov_db32;
        *var_qbgov_db33_slot = var_qbgov_db33;
        *var_qbgov_db34_slot = var_qbgov_db34;
        *var_qbgov_db35_slot = var_qbgov_db35;
        *var_qbgov_db36_slot = var_qbgov_db36;
        *var_qbgov_db37_slot = var_qbgov_db37;
        *var_qbgov_db38_slot = var_qbgov_db38;
        *var_qbgov_db39_slot = var_qbgov_db39;
        *var_qbgov_db4_slot = var_qbgov_db4;
        *var_qbgov_db40_slot = var_qbgov_db40;
        *var_qbgov_db41_slot = var_qbgov_db41;
        *var_qbgov_db42_slot = var_qbgov_db42;
        *var_qbgov_db43_slot = var_qbgov_db43;
        *var_qbgov_db44_slot = var_qbgov_db44;
        *var_qbgov_db45_slot = var_qbgov_db45;
        *var_qbgov_db46_slot = var_qbgov_db46;
        *var_qbgov_db47_slot = var_qbgov_db47;
        *var_qbgov_db48_slot = var_qbgov_db48;
        *var_qbgov_db49_slot = var_qbgov_db49;
        *var_qbgov_db5_slot = var_qbgov_db5;
        *var_qbgov_db50_slot = var_qbgov_db50;
        *var_qbgov_db51_slot = var_qbgov_db51;
        *var_qbgov_db52_slot = var_qbgov_db52;
        *var_qbgov_db53_slot = var_qbgov_db53;
        *var_qbgov_db54_slot = var_qbgov_db54;
        *var_qbgov_db55_slot = var_qbgov_db55;
        *var_qbgov_db56_slot = var_qbgov_db56;
        *var_qbgov_db6_slot = var_qbgov_db6;
        *var_qbgov_db7_slot = var_qbgov_db7;
        *var_qbgov_db8_slot = var_qbgov_db8;
        *var_qbgov_db9_slot = var_qbgov_db9;
        *var_qbgov_dn0_slot = var_qbgov_dn0;
        *var_qbgov_dn1_slot = var_qbgov_dn1;
        *var_qbgov_dn10_slot = var_qbgov_dn10;
        *var_qbgov_dn11_slot = var_qbgov_dn11;
        *var_qbgov_dn12_slot = var_qbgov_dn12;
        *var_qbgov_dn13_slot = var_qbgov_dn13;
        *var_qbgov_dn14_slot = var_qbgov_dn14;
        *var_qbgov_dn15_slot = var_qbgov_dn15;
        *var_qbgov_dn16_slot = var_qbgov_dn16;
        *var_qbgov_dn17_slot = var_qbgov_dn17;
        *var_qbgov_dn18_slot = var_qbgov_dn18;
        *var_qbgov_dn19_slot = var_qbgov_dn19;
        *var_qbgov_dn2_slot = var_qbgov_dn2;
        *var_qbgov_dn20_slot = var_qbgov_dn20;
        *var_qbgov_dn21_slot = var_qbgov_dn21;
        *var_qbgov_dn22_slot = var_qbgov_dn22;
        *var_qbgov_dn3_slot = var_qbgov_dn3;
        *var_qbgov_dn4_slot = var_qbgov_dn4;
        *var_qbgov_dn5_slot = var_qbgov_dn5;
        *var_qbgov_dn6_slot = var_qbgov_dn6;
        *var_qbgov_dn7_slot = var_qbgov_dn7;
        *var_qbgov_dn8_slot = var_qbgov_dn8;
        *var_qbgov_dn9_slot = var_qbgov_dn9;
        *var_qbsov_slot = var_qbsov;
        *var_qbsov_db0_slot = var_qbsov_db0;
        *var_qbsov_db1_slot = var_qbsov_db1;
        *var_qbsov_db10_slot = var_qbsov_db10;
        *var_qbsov_db11_slot = var_qbsov_db11;
        *var_qbsov_db12_slot = var_qbsov_db12;
        *var_qbsov_db13_slot = var_qbsov_db13;
        *var_qbsov_db14_slot = var_qbsov_db14;
        *var_qbsov_db15_slot = var_qbsov_db15;
        *var_qbsov_db16_slot = var_qbsov_db16;
        *var_qbsov_db17_slot = var_qbsov_db17;
        *var_qbsov_db18_slot = var_qbsov_db18;
        *var_qbsov_db19_slot = var_qbsov_db19;
        *var_qbsov_db2_slot = var_qbsov_db2;
        *var_qbsov_db20_slot = var_qbsov_db20;
        *var_qbsov_db21_slot = var_qbsov_db21;
        *var_qbsov_db22_slot = var_qbsov_db22;
        *var_qbsov_db23_slot = var_qbsov_db23;
        *var_qbsov_db24_slot = var_qbsov_db24;
        *var_qbsov_db25_slot = var_qbsov_db25;
        *var_qbsov_db26_slot = var_qbsov_db26;
        *var_qbsov_db27_slot = var_qbsov_db27;
        *var_qbsov_db28_slot = var_qbsov_db28;
        *var_qbsov_db29_slot = var_qbsov_db29;
        *var_qbsov_db3_slot = var_qbsov_db3;
        *var_qbsov_db30_slot = var_qbsov_db30;
        *var_qbsov_db31_slot = var_qbsov_db31;
        *var_qbsov_db32_slot = var_qbsov_db32;
        *var_qbsov_db33_slot = var_qbsov_db33;
        *var_qbsov_db34_slot = var_qbsov_db34;
        *var_qbsov_db35_slot = var_qbsov_db35;
        *var_qbsov_db36_slot = var_qbsov_db36;
        *var_qbsov_db37_slot = var_qbsov_db37;
        *var_qbsov_db38_slot = var_qbsov_db38;
        *var_qbsov_db39_slot = var_qbsov_db39;
        *var_qbsov_db4_slot = var_qbsov_db4;
        *var_qbsov_db40_slot = var_qbsov_db40;
        *var_qbsov_db41_slot = var_qbsov_db41;
        *var_qbsov_db42_slot = var_qbsov_db42;
        *var_qbsov_db43_slot = var_qbsov_db43;
        *var_qbsov_db44_slot = var_qbsov_db44;
        *var_qbsov_db45_slot = var_qbsov_db45;
        *var_qbsov_db46_slot = var_qbsov_db46;
        *var_qbsov_db47_slot = var_qbsov_db47;
        *var_qbsov_db48_slot = var_qbsov_db48;
        *var_qbsov_db49_slot = var_qbsov_db49;
        *var_qbsov_db5_slot = var_qbsov_db5;
        *var_qbsov_db50_slot = var_qbsov_db50;
        *var_qbsov_db51_slot = var_qbsov_db51;
        *var_qbsov_db52_slot = var_qbsov_db52;
        *var_qbsov_db53_slot = var_qbsov_db53;
        *var_qbsov_db54_slot = var_qbsov_db54;
        *var_qbsov_db55_slot = var_qbsov_db55;
        *var_qbsov_db56_slot = var_qbsov_db56;
        *var_qbsov_db6_slot = var_qbsov_db6;
        *var_qbsov_db7_slot = var_qbsov_db7;
        *var_qbsov_db8_slot = var_qbsov_db8;
        *var_qbsov_db9_slot = var_qbsov_db9;
        *var_qbsov_dn0_slot = var_qbsov_dn0;
        *var_qbsov_dn1_slot = var_qbsov_dn1;
        *var_qbsov_dn10_slot = var_qbsov_dn10;
        *var_qbsov_dn11_slot = var_qbsov_dn11;
        *var_qbsov_dn12_slot = var_qbsov_dn12;
        *var_qbsov_dn13_slot = var_qbsov_dn13;
        *var_qbsov_dn14_slot = var_qbsov_dn14;
        *var_qbsov_dn15_slot = var_qbsov_dn15;
        *var_qbsov_dn16_slot = var_qbsov_dn16;
        *var_qbsov_dn17_slot = var_qbsov_dn17;
        *var_qbsov_dn18_slot = var_qbsov_dn18;
        *var_qbsov_dn19_slot = var_qbsov_dn19;
        *var_qbsov_dn2_slot = var_qbsov_dn2;
        *var_qbsov_dn20_slot = var_qbsov_dn20;
        *var_qbsov_dn21_slot = var_qbsov_dn21;
        *var_qbsov_dn22_slot = var_qbsov_dn22;
        *var_qbsov_dn3_slot = var_qbsov_dn3;
        *var_qbsov_dn4_slot = var_qbsov_dn4;
        *var_qbsov_dn5_slot = var_qbsov_dn5;
        *var_qbsov_dn6_slot = var_qbsov_dn6;
        *var_qbsov_dn7_slot = var_qbsov_dn7;
        *var_qbsov_dn8_slot = var_qbsov_dn8;
        *var_qbsov_dn9_slot = var_qbsov_dn9;
        *var_qdov_slot = var_qdov;
        *var_qdov_db0_slot = var_qdov_db0;
        *var_qdov_db1_slot = var_qdov_db1;
        *var_qdov_db10_slot = var_qdov_db10;
        *var_qdov_db11_slot = var_qdov_db11;
        *var_qdov_db12_slot = var_qdov_db12;
        *var_qdov_db13_slot = var_qdov_db13;
        *var_qdov_db14_slot = var_qdov_db14;
        *var_qdov_db15_slot = var_qdov_db15;
        *var_qdov_db16_slot = var_qdov_db16;
        *var_qdov_db17_slot = var_qdov_db17;
        *var_qdov_db18_slot = var_qdov_db18;
        *var_qdov_db19_slot = var_qdov_db19;
        *var_qdov_db2_slot = var_qdov_db2;
        *var_qdov_db20_slot = var_qdov_db20;
        *var_qdov_db21_slot = var_qdov_db21;
        *var_qdov_db22_slot = var_qdov_db22;
        *var_qdov_db23_slot = var_qdov_db23;
        *var_qdov_db24_slot = var_qdov_db24;
        *var_qdov_db25_slot = var_qdov_db25;
        *var_qdov_db26_slot = var_qdov_db26;
        *var_qdov_db27_slot = var_qdov_db27;
        *var_qdov_db28_slot = var_qdov_db28;
        *var_qdov_db29_slot = var_qdov_db29;
        *var_qdov_db3_slot = var_qdov_db3;
        *var_qdov_db30_slot = var_qdov_db30;
        *var_qdov_db31_slot = var_qdov_db31;
        *var_qdov_db32_slot = var_qdov_db32;
        *var_qdov_db33_slot = var_qdov_db33;
        *var_qdov_db34_slot = var_qdov_db34;
        *var_qdov_db35_slot = var_qdov_db35;
        *var_qdov_db36_slot = var_qdov_db36;
        *var_qdov_db37_slot = var_qdov_db37;
        *var_qdov_db38_slot = var_qdov_db38;
        *var_qdov_db39_slot = var_qdov_db39;
        *var_qdov_db4_slot = var_qdov_db4;
        *var_qdov_db40_slot = var_qdov_db40;
        *var_qdov_db41_slot = var_qdov_db41;
        *var_qdov_db42_slot = var_qdov_db42;
        *var_qdov_db43_slot = var_qdov_db43;
        *var_qdov_db44_slot = var_qdov_db44;
        *var_qdov_db45_slot = var_qdov_db45;
        *var_qdov_db46_slot = var_qdov_db46;
        *var_qdov_db47_slot = var_qdov_db47;
        *var_qdov_db48_slot = var_qdov_db48;
        *var_qdov_db49_slot = var_qdov_db49;
        *var_qdov_db5_slot = var_qdov_db5;
        *var_qdov_db50_slot = var_qdov_db50;
        *var_qdov_db51_slot = var_qdov_db51;
        *var_qdov_db52_slot = var_qdov_db52;
        *var_qdov_db53_slot = var_qdov_db53;
        *var_qdov_db54_slot = var_qdov_db54;
        *var_qdov_db55_slot = var_qdov_db55;
        *var_qdov_db56_slot = var_qdov_db56;
        *var_qdov_db6_slot = var_qdov_db6;
        *var_qdov_db7_slot = var_qdov_db7;
        *var_qdov_db8_slot = var_qdov_db8;
        *var_qdov_db9_slot = var_qdov_db9;
        *var_qdov_dn0_slot = var_qdov_dn0;
        *var_qdov_dn1_slot = var_qdov_dn1;
        *var_qdov_dn10_slot = var_qdov_dn10;
        *var_qdov_dn11_slot = var_qdov_dn11;
        *var_qdov_dn12_slot = var_qdov_dn12;
        *var_qdov_dn13_slot = var_qdov_dn13;
        *var_qdov_dn14_slot = var_qdov_dn14;
        *var_qdov_dn15_slot = var_qdov_dn15;
        *var_qdov_dn16_slot = var_qdov_dn16;
        *var_qdov_dn17_slot = var_qdov_dn17;
        *var_qdov_dn18_slot = var_qdov_dn18;
        *var_qdov_dn19_slot = var_qdov_dn19;
        *var_qdov_dn2_slot = var_qdov_dn2;
        *var_qdov_dn20_slot = var_qdov_dn20;
        *var_qdov_dn21_slot = var_qdov_dn21;
        *var_qdov_dn22_slot = var_qdov_dn22;
        *var_qdov_dn3_slot = var_qdov_dn3;
        *var_qdov_dn4_slot = var_qdov_dn4;
        *var_qdov_dn5_slot = var_qdov_dn5;
        *var_qdov_dn6_slot = var_qdov_dn6;
        *var_qdov_dn7_slot = var_qdov_dn7;
        *var_qdov_dn8_slot = var_qdov_dn8;
        *var_qdov_dn9_slot = var_qdov_dn9;
        *var_qdsov_slot = var_qdsov;
        *var_qdsov_db0_slot = var_qdsov_db0;
        *var_qdsov_db1_slot = var_qdsov_db1;
        *var_qdsov_db10_slot = var_qdsov_db10;
        *var_qdsov_db11_slot = var_qdsov_db11;
        *var_qdsov_db12_slot = var_qdsov_db12;
        *var_qdsov_db13_slot = var_qdsov_db13;
        *var_qdsov_db14_slot = var_qdsov_db14;
        *var_qdsov_db15_slot = var_qdsov_db15;
        *var_qdsov_db16_slot = var_qdsov_db16;
        *var_qdsov_db17_slot = var_qdsov_db17;
        *var_qdsov_db18_slot = var_qdsov_db18;
        *var_qdsov_db19_slot = var_qdsov_db19;
        *var_qdsov_db2_slot = var_qdsov_db2;
        *var_qdsov_db20_slot = var_qdsov_db20;
        *var_qdsov_db21_slot = var_qdsov_db21;
        *var_qdsov_db22_slot = var_qdsov_db22;
        *var_qdsov_db23_slot = var_qdsov_db23;
        *var_qdsov_db24_slot = var_qdsov_db24;
        *var_qdsov_db25_slot = var_qdsov_db25;
        *var_qdsov_db26_slot = var_qdsov_db26;
        *var_qdsov_db27_slot = var_qdsov_db27;
        *var_qdsov_db28_slot = var_qdsov_db28;
        *var_qdsov_db29_slot = var_qdsov_db29;
        *var_qdsov_db3_slot = var_qdsov_db3;
        *var_qdsov_db30_slot = var_qdsov_db30;
        *var_qdsov_db31_slot = var_qdsov_db31;
        *var_qdsov_db32_slot = var_qdsov_db32;
        *var_qdsov_db33_slot = var_qdsov_db33;
        *var_qdsov_db34_slot = var_qdsov_db34;
        *var_qdsov_db35_slot = var_qdsov_db35;
        *var_qdsov_db36_slot = var_qdsov_db36;
        *var_qdsov_db37_slot = var_qdsov_db37;
        *var_qdsov_db38_slot = var_qdsov_db38;
        *var_qdsov_db39_slot = var_qdsov_db39;
        *var_qdsov_db4_slot = var_qdsov_db4;
        *var_qdsov_db40_slot = var_qdsov_db40;
        *var_qdsov_db41_slot = var_qdsov_db41;
        *var_qdsov_db42_slot = var_qdsov_db42;
        *var_qdsov_db43_slot = var_qdsov_db43;
        *var_qdsov_db44_slot = var_qdsov_db44;
        *var_qdsov_db45_slot = var_qdsov_db45;
        *var_qdsov_db46_slot = var_qdsov_db46;
        *var_qdsov_db47_slot = var_qdsov_db47;
        *var_qdsov_db48_slot = var_qdsov_db48;
        *var_qdsov_db49_slot = var_qdsov_db49;
        *var_qdsov_db5_slot = var_qdsov_db5;
        *var_qdsov_db50_slot = var_qdsov_db50;
        *var_qdsov_db51_slot = var_qdsov_db51;
        *var_qdsov_db52_slot = var_qdsov_db52;
        *var_qdsov_db53_slot = var_qdsov_db53;
        *var_qdsov_db54_slot = var_qdsov_db54;
        *var_qdsov_db55_slot = var_qdsov_db55;
        *var_qdsov_db56_slot = var_qdsov_db56;
        *var_qdsov_db6_slot = var_qdsov_db6;
        *var_qdsov_db7_slot = var_qdsov_db7;
        *var_qdsov_db8_slot = var_qdsov_db8;
        *var_qdsov_db9_slot = var_qdsov_db9;
        *var_qdsov_dn0_slot = var_qdsov_dn0;
        *var_qdsov_dn1_slot = var_qdsov_dn1;
        *var_qdsov_dn10_slot = var_qdsov_dn10;
        *var_qdsov_dn11_slot = var_qdsov_dn11;
        *var_qdsov_dn12_slot = var_qdsov_dn12;
        *var_qdsov_dn13_slot = var_qdsov_dn13;
        *var_qdsov_dn14_slot = var_qdsov_dn14;
        *var_qdsov_dn15_slot = var_qdsov_dn15;
        *var_qdsov_dn16_slot = var_qdsov_dn16;
        *var_qdsov_dn17_slot = var_qdsov_dn17;
        *var_qdsov_dn18_slot = var_qdsov_dn18;
        *var_qdsov_dn19_slot = var_qdsov_dn19;
        *var_qdsov_dn2_slot = var_qdsov_dn2;
        *var_qdsov_dn20_slot = var_qdsov_dn20;
        *var_qdsov_dn21_slot = var_qdsov_dn21;
        *var_qdsov_dn22_slot = var_qdsov_dn22;
        *var_qdsov_dn3_slot = var_qdsov_dn3;
        *var_qdsov_dn4_slot = var_qdsov_dn4;
        *var_qdsov_dn5_slot = var_qdsov_dn5;
        *var_qdsov_dn6_slot = var_qdsov_dn6;
        *var_qdsov_dn7_slot = var_qdsov_dn7;
        *var_qdsov_dn8_slot = var_qdsov_dn8;
        *var_qdsov_dn9_slot = var_qdsov_dn9;
    }
}
