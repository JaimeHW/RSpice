#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((!s.b[478]) && s.b[487]) {
            s.store_sub(263, 261, 260);
            s.store_sub(90, 261, 260);
            s.store_ad_value(91, A::add_scaled_inputs3(s.ad_value(258), 1.0, s.ad_value(83), 1.0, s.ad_value(262), -1.0));
            s.store_mul_scaled_ad_rhs(137, 259, ((p.p4 * p.p5) * p.p174), A::add_scaled_inputs3(s.ad_value(258), 1.0, s.ad_value(262), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_scale(188, 137, (1.0 / (p.p239) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p238, 1.0);
            s.store_div_from_scalar(190, p.p237, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p173);
            s.store_mul_scaled_ad_rhs(264, 191, ((p.p4 * p.p5) * p.p174), A::add_scaled_inputs3(s.ad_value(258), 1.0, s.ad_value(262), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_ad_value(136, A::add_scaled_inputs3(s.ad_value(258), 1.0, s.ad_value(83), 1.0, s.ad_value(262), -1.0));
            s.store_add_scaled_inputs(90, 260, 0.3333333333333333, 261, (2.0 * 0.3333333333333333));
            s.store_ad_value(91, A::div_scaled_inputs(A::square(s.ad_value(263)), (1.0 / 12.0), s.ad_value(136), 1.0));
            s.store_ad_value(137, A::div_scaled_product(A::square(s.ad_value(263)), s.ad_value(263), (1.0 / 120.0), A::square(s.ad_value(136)), 1.0));
            s.store_mul_scaled_ad_rhs(265, 191, (-(((p.p4 * p.p174) * p.p5) * 0.5)), A::add_scaled_inputs4(s.ad_value(258), 1.0, s.ad_value(90), (-1.0), s.ad_value(91), 1.0, s.ad_value(137), 1.0));
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
            s.store_ad_value(159, A::add_scaled_product(s.ad_value(88), 1.0, s.ad_value(83), A::ln_scaled_input(s.ad_value(136), p.p184), 1.0));
            s.store_ad_value(160, A::add_scaled_inputs4(s.ad_value(278), 0.5, s.ad_value(159), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(278), s.ad_value(159)), A::sub(s.ad_value(278), s.ad_value(159))), 0.0001)), 0.5, s.ad_value(159), 1.0));
            s.store_sub(270, 160, 88);
            s.store_scaled_div(84, 271, 83, (1.0 / ((1.602176634e-19 * 3.24e17))));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 271, 6.241509074460763e18);
            s.store_scaled_add_ad_rhs(154, 270, A::sqrt(A::offset(A::square(s.ad_value(270)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_ad_value(155, A::div_scaled_product(s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0));
            s.store_ad_value(130, A::div_scaled_product(s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0));
        }

        if (s.b[493] && s.b[494]) {
            let assign18800_ad_e29391: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p195 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign18800_ad_e29391);
        }

        if (s.b[493] && s.b[494]) {
            s.store_scaled_div(136, 270, 83, (1.0 / (2.0)));
        }

        s.b[497] = (s.v[136] < 200.0);
        s.v[497] = if s.b[497] { 1.0 } else { 0.0 };

        if ((s.b[493] && s.b[494]) && s.b[497]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(270), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((s.b[493] && s.b[494]) && (!s.b[497])) {
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(270), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (s.b[493] && s.b[494]) {
            s.store_sub_ad_rhs(100, 270, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.b[498] = ((((s.v[100] - s.v[270])) as f64).abs() > 1e-19);
        s.v[498] = if s.b[498] { 1.0 } else { 0.0 };

        if ((s.b[493] && s.b[494]) && s.b[498]) {
            s.store_sub(101, 270, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p195);
            s.store_scaled_mul(103, 136, 90, p.p196);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign18980_ad_e29685, (-3.24e17), s.ad_value(83), assign18980_ad_e29723, (-3.24e17)));
        }

        if ((s.b[493] && s.b[494]) && s.b[498]) {
            s.store_scaled_mul(107, 136, 91, p.p195);
            s.store_scaled_mul(108, 136, 91, p.p196);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 270, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p195, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p196, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign19140_ad_e29982, (-3.24e17), s.ad_value(83), assign19140_ad_e30020, (-3.24e17)));
        }

        if ((s.b[493] && s.b[494]) && s.b[498]) {
            s.store_scaled_mul(121, 136, 137, p.p195);
            s.store_scaled_mul(122, 136, 137, p.p196);
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.copy_ad(272, 128);
        }

        if ((s.b[493] && s.b[494]) && (!s.b[498])) {
            s.copy_ad(272, 100);
        }

        if (s.b[493] && s.b[494]) {
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p189);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p190);
            s.store_mul_scaled_ad_rhs(136, 271, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(270), s.ad_value(272))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(272)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_scaled_div(136, 89, 95, 2.0);
            s.store_scaled_add_ad_rhs(90, 270, A::sqrt(A::offset(A::square(s.ad_value(270)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_ad_value(85, A::div_scaled_product(s.ad_value(136), s.ad_value(90), p.p187, A::add_scaled_inputs(s.ad_value(136), p.p187, s.ad_value(90), 1.0), 1.0));
            s.store_powf_ad(136, A::div(s.ad_value(279), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 279, 90);
            s.store_sub(39, 270, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_ad_rhs(131, 130, A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.copy_ad(154, 131);
            s.store_ad_value(157, A::div_scaled_product(s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0));
            s.store_ad_value(158, A::div_scaled_product(s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0));
        }

        if (s.b[493] && s.b[494]) {
            let assign19420_ad_e30420: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p195 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign19420_ad_e30420);
        }

    }

    pub(super) fn stamp_transient_block_17(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[493] && s.b[494]) {
            s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));
        }

        s.b[499] = (s.v[136] < 200.0);
        s.v[499] = if s.b[499] { 1.0 } else { 0.0 };

        if ((s.b[493] && s.b[494]) && s.b[499]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((s.b[493] && s.b[494]) && (!s.b[499])) {
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (s.b[493] && s.b[494]) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.b[500] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[500] = if s.b[500] { 1.0 } else { 0.0 };

        if ((s.b[493] && s.b[494]) && s.b[500]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p195);
            s.store_scaled_mul(103, 136, 90, p.p196);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign19600_ad_e30714, (-3.24e17), s.ad_value(83), assign19600_ad_e30752, (-3.24e17)));
        }

        if ((s.b[493] && s.b[494]) && s.b[500]) {
            s.store_scaled_mul(107, 136, 91, p.p195);
            s.store_scaled_mul(108, 136, 91, p.p196);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_mul_scaled_ad_rhs(116, 136, p.p195, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p196, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign19750_ad_e31000, (-3.24e17), s.ad_value(83), assign19750_ad_e31038, (-3.24e17)));
        }

        if ((s.b[493] && s.b[494]) && s.b[500]) {
            s.store_mul_scaled_ad_rhs(121, 136, p.p195, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_scaled_ad_rhs(122, 136, p.p196, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.store_add(273, 128, 86);
        }

        if ((s.b[493] && s.b[494]) && (!s.b[500])) {
            s.store_add(273, 100, 86);
        }

        if (s.b[493] && s.b[494]) {
            s.store_scaled_add(274, 272, 273, 0.5);
            s.store_sub(275, 273, 272);
            s.store_mul_ad_lhs(135, A::add_scaled_inputs3(s.ad_value(270), 1.0, s.ad_value(274), (-1.0), s.ad_value(83), 1.0), 275);
            s.store_mul_scaled_ad_rhs(136, 271, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(270), s.ad_value(274))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs(A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, s.ad_value(90), p.p16));
            s.store_scaled_mul(96, 95, 271, (p.p4 * (p.p5 * 1.0 / (p.p187))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(280), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(275), (p.p25 * p.p25), s.ad_value(275)), 1.0);
            s.store_div(93, 98, 92);
            s.store_mul(281, 93, 135);
            s.store_sub(90, 273, 272);
            s.store_ad_value(91, A::add_scaled_inputs3(s.ad_value(270), 1.0, s.ad_value(83), 1.0, s.ad_value(274), -1.0));
            s.store_mul_scaled_ad_rhs(137, 271, ((p.p4 * p.p5) * p.p187), A::add_scaled_inputs3(s.ad_value(270), 1.0, s.ad_value(274), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_scale(188, 137, (1.0 / (p.p242) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p241, 1.0);
            s.store_div_from_scalar(190, p.p240, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p186);
            s.store_mul_scaled_ad_rhs(276, 191, ((p.p4 * p.p5) * p.p187), A::add_scaled_inputs3(s.ad_value(270), 1.0, s.ad_value(274), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_ad_value(136, A::add_scaled_inputs3(s.ad_value(270), 1.0, s.ad_value(83), 1.0, s.ad_value(274), -1.0));
            s.store_add_scaled_inputs(90, 272, 0.3333333333333333, 273, (2.0 * 0.3333333333333333));
            s.store_ad_value(91, A::div_scaled_inputs(A::square(s.ad_value(275)), (1.0 / 12.0), s.ad_value(136), 1.0));
            s.store_ad_value(137, A::div_scaled_product(A::square(s.ad_value(275)), s.ad_value(275), (1.0 / 120.0), A::square(s.ad_value(136)), 1.0));
            s.store_mul_scaled_ad_rhs(277, 191, (-(((p.p4 * p.p187) * p.p5) * 0.5)), A::add_scaled_inputs4(s.ad_value(270), 1.0, s.ad_value(90), (-1.0), s.ad_value(91), 1.0, s.ad_value(137), 1.0));
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
            s.store_ad_value(159, A::add_scaled_product(s.ad_value(88), 1.0, s.ad_value(83), A::ln_scaled_input(s.ad_value(136), p.p184), 1.0));
            s.store_ad_value(160, A::add_scaled_inputs4(s.ad_value(278), 0.5, s.ad_value(159), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(278), s.ad_value(159)), A::sub(s.ad_value(278), s.ad_value(159))), 0.0001)), 0.5, s.ad_value(159), 1.0));
            s.store_sub(270, 160, 88);
            s.store_scaled_div(84, 271, 83, (1.0 / ((1.602176634e-19 * 3.24e17))));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 271, 6.241509074460763e18);
            s.store_scaled_add_ad_rhs(154, 270, A::sqrt(A::offset(A::square(s.ad_value(270)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_ad_value(155, A::div_scaled_product(s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0));
            s.store_ad_value(130, A::div_scaled_product(s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0));
        }

        if ((!s.b[493]) && s.b[502]) {
            let assign20340_ad_e31818: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p195 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign20340_ad_e31818);
        }

        if ((!s.b[493]) && s.b[502]) {
            s.store_scaled_div(136, 270, 83, (1.0 / (2.0)));
        }

        s.b[504] = (s.v[136] < 200.0);
        s.v[504] = if s.b[504] { 1.0 } else { 0.0 };

        if (((!s.b[493]) && s.b[502]) && s.b[504]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(270), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (((!s.b[493]) && s.b[502]) && (!s.b[504])) {
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(270), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((!s.b[493]) && s.b[502]) {
            s.store_sub_ad_rhs(100, 270, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.b[505] = ((((s.v[100] - s.v[270])) as f64).abs() > 1e-19);
        s.v[505] = if s.b[505] { 1.0 } else { 0.0 };

        if (((!s.b[493]) && s.b[502]) && s.b[505]) {
            s.store_sub(101, 270, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p195);
            s.store_scaled_mul(103, 136, 90, p.p196);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

    }

    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign20520_ad_e32128, (-3.24e17), s.ad_value(83), assign20520_ad_e32166, (-3.24e17)));
        }

        if (((!s.b[493]) && s.b[502]) && s.b[505]) {
            s.store_scaled_mul(107, 136, 91, p.p195);
            s.store_scaled_mul(108, 136, 91, p.p196);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 270, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p195, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p196, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign20680_ad_e32441, (-3.24e17), s.ad_value(83), assign20680_ad_e32479, (-3.24e17)));
        }

        if (((!s.b[493]) && s.b[502]) && s.b[505]) {
            s.store_scaled_mul(121, 136, 137, p.p195);
            s.store_scaled_mul(122, 136, 137, p.p196);
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.copy_ad(272, 128);
        }

        if (((!s.b[493]) && s.b[502]) && (!s.b[505])) {
            s.copy_ad(272, 100);
        }

        if ((!s.b[493]) && s.b[502]) {
            s.store_scalar(279, 0.0);
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p189);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p190);
            s.store_mul_scaled_ad_rhs(136, 271, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(270), s.ad_value(272))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(272)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_scaled_div(136, 89, 95, 2.0);
            s.store_scaled_add_ad_rhs(90, 270, A::sqrt(A::offset(A::square(s.ad_value(270)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_ad_value(85, A::div_scaled_product(s.ad_value(136), s.ad_value(90), p.p187, A::add_scaled_inputs(s.ad_value(136), p.p187, s.ad_value(90), 1.0), 1.0));
            s.store_powf_ad(136, A::div(s.ad_value(279), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 279, 90);
            s.store_sub(39, 270, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_ad_rhs(131, 130, A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.copy_ad(154, 131);
            s.store_ad_value(157, A::div_scaled_product(s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0));
            s.store_ad_value(158, A::div_scaled_product(s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0));
        }

        if ((!s.b[493]) && s.b[502]) {
            let assign20970_ad_e32914: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p195 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign20970_ad_e32914);
        }

        if ((!s.b[493]) && s.b[502]) {
            s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));
        }

        s.b[506] = (s.v[136] < 200.0);
        s.v[506] = if s.b[506] { 1.0 } else { 0.0 };

        if (((!s.b[493]) && s.b[502]) && s.b[506]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (((!s.b[493]) && s.b[502]) && (!s.b[506])) {
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((!s.b[493]) && s.b[502]) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.b[507] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[507] = if s.b[507] { 1.0 } else { 0.0 };

        if (((!s.b[493]) && s.b[502]) && s.b[507]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p195);
            s.store_scaled_mul(103, 136, 90, p.p196);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign21150_ad_e33224, (-3.24e17), s.ad_value(83), assign21150_ad_e33262, (-3.24e17)));
        }

        if (((!s.b[493]) && s.b[502]) && s.b[507]) {
            s.store_scaled_mul(107, 136, 91, p.p195);
            s.store_scaled_mul(108, 136, 91, p.p196);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_mul_scaled_ad_rhs(116, 136, p.p195, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p196, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign21300_ad_e33525, (-3.24e17), s.ad_value(83), assign21300_ad_e33563, (-3.24e17)));
        }

        if (((!s.b[493]) && s.b[502]) && s.b[507]) {
            s.store_mul_scaled_ad_rhs(121, 136, p.p195, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_scaled_ad_rhs(122, 136, p.p196, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.store_add(273, 128, 86);
        }

        if (((!s.b[493]) && s.b[502]) && (!s.b[507])) {
            s.store_add(273, 100, 86);
        }

        if ((!s.b[493]) && s.b[502]) {
            s.store_scaled_add(274, 272, 273, 0.5);
        }

    }

    pub(super) fn stamp_transient_block_19(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((!s.b[493]) && s.b[502]) {
            s.store_sub(275, 273, 272);
            s.store_sub(90, 273, 272);
            s.store_ad_value(91, A::add_scaled_inputs3(s.ad_value(270), 1.0, s.ad_value(83), 1.0, s.ad_value(274), -1.0));
            s.store_mul_scaled_ad_rhs(137, 271, ((p.p4 * p.p5) * p.p187), A::add_scaled_inputs3(s.ad_value(270), 1.0, s.ad_value(274), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_scale(188, 137, (1.0 / (p.p242) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p241, 1.0);
            s.store_div_from_scalar(190, p.p240, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p186);
            s.store_mul_scaled_ad_rhs(276, 191, ((p.p4 * p.p5) * p.p187), A::add_scaled_inputs3(s.ad_value(270), 1.0, s.ad_value(274), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_ad_value(136, A::add_scaled_inputs3(s.ad_value(270), 1.0, s.ad_value(83), 1.0, s.ad_value(274), -1.0));
            s.store_add_scaled_inputs(90, 272, 0.3333333333333333, 273, (2.0 * 0.3333333333333333));
            s.store_ad_value(91, A::div_scaled_inputs(A::square(s.ad_value(275)), (1.0 / 12.0), s.ad_value(136), 1.0));
            s.store_ad_value(137, A::div_scaled_product(A::square(s.ad_value(275)), s.ad_value(275), (1.0 / 120.0), A::square(s.ad_value(136)), 1.0));
            s.store_mul_scaled_ad_rhs(277, 191, (-(((p.p4 * p.p187) * p.p5) * 0.5)), A::add_scaled_inputs4(s.ad_value(270), 1.0, s.ad_value(90), (-1.0), s.ad_value(91), 1.0, s.ad_value(137), 1.0));
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
            s.store_ad_value(159, A::add_scaled_product(s.ad_value(88), 1.0, s.ad_value(83), A::ln_scaled_input(s.ad_value(136), p.p184), 1.0));
            s.store_ad_value(160, A::add_scaled_inputs4(s.ad_value(290), 0.5, s.ad_value(159), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(290), s.ad_value(159)), A::sub(s.ad_value(290), s.ad_value(159))), 0.0001)), 0.5, s.ad_value(159), 1.0));
            s.store_sub(282, 160, 88);
            s.store_scaled_div(84, 283, 83, (1.0 / ((1.602176634e-19 * 3.24e17))));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 283, 6.241509074460763e18);
            s.store_scaled_add_ad_rhs(154, 282, A::sqrt(A::offset(A::square(s.ad_value(282)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_ad_value(155, A::div_scaled_product(s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0));
            s.store_ad_value(130, A::div_scaled_product(s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0));
        }

        if (s.b[508] && s.b[509]) {
            let assign21890_ad_e34320: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p195 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign21890_ad_e34320);
        }

        if (s.b[508] && s.b[509]) {
            s.store_scaled_div(136, 282, 83, (1.0 / (2.0)));
        }

        s.b[512] = (s.v[136] < 200.0);
        s.v[512] = if s.b[512] { 1.0 } else { 0.0 };

        if ((s.b[508] && s.b[509]) && s.b[512]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(282), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((s.b[508] && s.b[509]) && (!s.b[512])) {
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(282), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (s.b[508] && s.b[509]) {
            s.store_sub_ad_rhs(100, 282, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.b[513] = ((((s.v[100] - s.v[282])) as f64).abs() > 1e-19);
        s.v[513] = if s.b[513] { 1.0 } else { 0.0 };

        if ((s.b[508] && s.b[509]) && s.b[513]) {
            s.store_sub(101, 282, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p195);
            s.store_scaled_mul(103, 136, 90, p.p196);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign22070_ad_e34614, (-3.24e17), s.ad_value(83), assign22070_ad_e34652, (-3.24e17)));
        }

        if ((s.b[508] && s.b[509]) && s.b[513]) {
            s.store_scaled_mul(107, 136, 91, p.p195);
            s.store_scaled_mul(108, 136, 91, p.p196);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 282, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p195, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p196, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign22230_ad_e34911, (-3.24e17), s.ad_value(83), assign22230_ad_e34949, (-3.24e17)));
        }

        if ((s.b[508] && s.b[509]) && s.b[513]) {
            s.store_scaled_mul(121, 136, 137, p.p195);
            s.store_scaled_mul(122, 136, 137, p.p196);
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.copy_ad(284, 128);
        }

        if ((s.b[508] && s.b[509]) && (!s.b[513])) {
            s.copy_ad(284, 100);
        }

        if (s.b[508] && s.b[509]) {
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p189);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p190);
            s.store_mul_scaled_ad_rhs(136, 283, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(282), s.ad_value(284))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(284)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_scaled_div(136, 89, 95, 2.0);
            s.store_scaled_add_ad_rhs(90, 282, A::sqrt(A::offset(A::square(s.ad_value(282)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_ad_value(85, A::div_scaled_product(s.ad_value(136), s.ad_value(90), p.p187, A::add_scaled_inputs(s.ad_value(136), p.p187, s.ad_value(90), 1.0), 1.0));
            s.store_powf_ad(136, A::div(s.ad_value(291), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 291, 90);
            s.store_sub(39, 282, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_ad_rhs(131, 130, A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.copy_ad(154, 131);
            s.store_ad_value(157, A::div_scaled_product(s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0));
            s.store_ad_value(158, A::div_scaled_product(s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0));
        }

        if (s.b[508] && s.b[509]) {
            let assign22510_ad_e35349: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p195 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign22510_ad_e35349);
        }

    }

    pub(super) fn stamp_transient_block_20(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[508] && s.b[509]) {
            s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));
        }

        s.b[514] = (s.v[136] < 200.0);
        s.v[514] = if s.b[514] { 1.0 } else { 0.0 };

        if ((s.b[508] && s.b[509]) && s.b[514]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((s.b[508] && s.b[509]) && (!s.b[514])) {
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (s.b[508] && s.b[509]) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.b[515] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[515] = if s.b[515] { 1.0 } else { 0.0 };

        if ((s.b[508] && s.b[509]) && s.b[515]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p195);
            s.store_scaled_mul(103, 136, 90, p.p196);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if ((s.b[508] && s.b[509]) && s.b[515]) {
            let assign22690_ad_e35643: A = {
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
            let assign22690_ad_e35681: A = {
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign22690_ad_e35643, (-3.24e17), s.ad_value(83), assign22690_ad_e35681, (-3.24e17)));
        }

        if ((s.b[508] && s.b[509]) && s.b[515]) {
            s.store_scaled_mul(107, 136, 91, p.p195);
            s.store_scaled_mul(108, 136, 91, p.p196);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_mul_scaled_ad_rhs(116, 136, p.p195, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p196, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if ((s.b[508] && s.b[509]) && s.b[515]) {
            let assign22840_ad_e35929: A = {
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
            let assign22840_ad_e35967: A = {
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign22840_ad_e35929, (-3.24e17), s.ad_value(83), assign22840_ad_e35967, (-3.24e17)));
        }

        if ((s.b[508] && s.b[509]) && s.b[515]) {
            s.store_mul_scaled_ad_rhs(121, 136, p.p195, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_scaled_ad_rhs(122, 136, p.p196, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.store_add(285, 128, 86);
        }

        if ((s.b[508] && s.b[509]) && (!s.b[515])) {
            s.store_add(285, 100, 86);
        }

        if (s.b[508] && s.b[509]) {
            s.store_scaled_add(286, 284, 285, 0.5);
            s.store_sub(287, 285, 284);
            s.store_mul_ad_lhs(135, A::add_scaled_inputs3(s.ad_value(282), 1.0, s.ad_value(286), (-1.0), s.ad_value(83), 1.0), 287);
            s.store_mul_scaled_ad_rhs(136, 283, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(282), s.ad_value(286))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs(A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, s.ad_value(90), p.p16));
            s.store_scaled_mul(96, 95, 283, (p.p4 * (p.p5 * 1.0 / (p.p187))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(292), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(287), (p.p25 * p.p25), s.ad_value(287)), 1.0);
            s.store_div(93, 98, 92);
            s.store_mul(293, 93, 135);
            s.store_sub(90, 285, 284);
            s.store_ad_value(91, A::add_scaled_inputs3(s.ad_value(282), 1.0, s.ad_value(83), 1.0, s.ad_value(286), -1.0));
            s.store_mul_scaled_ad_rhs(137, 283, ((p.p4 * p.p5) * p.p187), A::add_scaled_inputs3(s.ad_value(282), 1.0, s.ad_value(286), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_scale(188, 137, (1.0 / (p.p242) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p241, 1.0);
            s.store_div_from_scalar(190, p.p240, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p186);
            s.store_mul_scaled_ad_rhs(288, 191, ((p.p4 * p.p5) * p.p187), A::add_scaled_inputs3(s.ad_value(282), 1.0, s.ad_value(286), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_ad_value(136, A::add_scaled_inputs3(s.ad_value(282), 1.0, s.ad_value(83), 1.0, s.ad_value(286), -1.0));
            s.store_add_scaled_inputs(90, 284, 0.3333333333333333, 285, (2.0 * 0.3333333333333333));
            s.store_ad_value(91, A::div_scaled_inputs(A::square(s.ad_value(287)), (1.0 / 12.0), s.ad_value(136), 1.0));
            s.store_ad_value(137, A::div_scaled_product(A::square(s.ad_value(287)), s.ad_value(287), (1.0 / 120.0), A::square(s.ad_value(136)), 1.0));
            s.store_mul_scaled_ad_rhs(289, 191, (-(((p.p4 * p.p187) * p.p5) * 0.5)), A::add_scaled_inputs4(s.ad_value(282), 1.0, s.ad_value(90), (-1.0), s.ad_value(91), 1.0, s.ad_value(137), 1.0));
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
            s.store_div_from_scalar_ad(136, p.p187, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_ad_value(159, A::add_scaled_product(s.ad_value(88), 1.0, s.ad_value(83), A::ln_scaled_input(s.ad_value(136), p.p184), 1.0));
            s.store_ad_value(160, A::add_scaled_inputs4(s.ad_value(290), 0.5, s.ad_value(159), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(290), s.ad_value(159)), A::sub(s.ad_value(290), s.ad_value(159))), 0.0001)), 0.5, s.ad_value(159), 1.0));
            s.store_sub(282, 160, 88);
            s.store_scaled_div(84, 283, 83, (1.0 / ((1.602176634e-19 * 3.24e17))));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 283, 6.241509074460763e18);
            s.store_scaled_add_ad_rhs(154, 282, A::sqrt(A::offset(A::square(s.ad_value(282)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_ad_value(155, A::div_scaled_product(s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0));
            s.store_ad_value(130, A::div_scaled_product(s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0));
        }

        if ((!s.b[508]) && s.b[517]) {
            let assign23430_ad_e36747: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p195 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign23430_ad_e36747);
        }

        if ((!s.b[508]) && s.b[517]) {
            s.store_scaled_div(136, 282, 83, (1.0 / (2.0)));
        }

        s.b[519] = (s.v[136] < 200.0);
        s.v[519] = if s.b[519] { 1.0 } else { 0.0 };

        if (((!s.b[508]) && s.b[517]) && s.b[519]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(282), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (((!s.b[508]) && s.b[517]) && (!s.b[519])) {
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(282), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((!s.b[508]) && s.b[517]) {
            s.store_sub_ad_rhs(100, 282, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.b[520] = ((((s.v[100] - s.v[282])) as f64).abs() > 1e-19);
        s.v[520] = if s.b[520] { 1.0 } else { 0.0 };

        if (((!s.b[508]) && s.b[517]) && s.b[520]) {
            s.store_sub(101, 282, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p195);
            s.store_scaled_mul(103, 136, 90, p.p196);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

    }

    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[508]) && s.b[517]) && s.b[520]) {
            let assign23610_ad_e37057: A = {
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
            let assign23610_ad_e37095: A = {
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign23610_ad_e37057, (-3.24e17), s.ad_value(83), assign23610_ad_e37095, (-3.24e17)));
        }

        if (((!s.b[508]) && s.b[517]) && s.b[520]) {
            s.store_scaled_mul(107, 136, 91, p.p195);
            s.store_scaled_mul(108, 136, 91, p.p196);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 282, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p195, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p196, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!s.b[508]) && s.b[517]) && s.b[520]) {
            let assign23770_ad_e37370: A = {
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
            let assign23770_ad_e37408: A = {
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign23770_ad_e37370, (-3.24e17), s.ad_value(83), assign23770_ad_e37408, (-3.24e17)));
        }

        if (((!s.b[508]) && s.b[517]) && s.b[520]) {
            s.store_scaled_mul(121, 136, 137, p.p195);
            s.store_scaled_mul(122, 136, 137, p.p196);
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.copy_ad(284, 128);
        }

        if (((!s.b[508]) && s.b[517]) && (!s.b[520])) {
            s.copy_ad(284, 100);
        }

        if ((!s.b[508]) && s.b[517]) {
            s.store_scalar(291, 0.0);
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p189);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p190);
            s.store_mul_scaled_ad_rhs(136, 283, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(282), s.ad_value(284))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(284)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_scaled_div(136, 89, 95, 2.0);
            s.store_scaled_add_ad_rhs(90, 282, A::sqrt(A::offset(A::square(s.ad_value(282)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_ad_value(85, A::div_scaled_product(s.ad_value(136), s.ad_value(90), p.p187, A::add_scaled_inputs(s.ad_value(136), p.p187, s.ad_value(90), 1.0), 1.0));
            s.store_powf_ad(136, A::div(s.ad_value(291), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 291, 90);
            s.store_sub(39, 282, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_ad_rhs(131, 130, A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.copy_ad(154, 131);
            s.store_ad_value(157, A::div_scaled_product(s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0));
            s.store_ad_value(158, A::div_scaled_product(s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0));
        }

        if ((!s.b[508]) && s.b[517]) {
            let assign24060_ad_e37843: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p195 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign24060_ad_e37843);
        }

        if ((!s.b[508]) && s.b[517]) {
            s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));
        }

        s.b[521] = (s.v[136] < 200.0);
        s.v[521] = if s.b[521] { 1.0 } else { 0.0 };

        if (((!s.b[508]) && s.b[517]) && s.b[521]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (((!s.b[508]) && s.b[517]) && (!s.b[521])) {
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((!s.b[508]) && s.b[517]) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.b[522] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[522] = if s.b[522] { 1.0 } else { 0.0 };

        if (((!s.b[508]) && s.b[517]) && s.b[522]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p195);
            s.store_scaled_mul(103, 136, 90, p.p196);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!s.b[508]) && s.b[517]) && s.b[522]) {
            let assign24240_ad_e38153: A = {
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
            let assign24240_ad_e38191: A = {
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign24240_ad_e38153, (-3.24e17), s.ad_value(83), assign24240_ad_e38191, (-3.24e17)));
        }

        if (((!s.b[508]) && s.b[517]) && s.b[522]) {
            s.store_scaled_mul(107, 136, 91, p.p195);
            s.store_scaled_mul(108, 136, 91, p.p196);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_mul_scaled_ad_rhs(116, 136, p.p195, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p196, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!s.b[508]) && s.b[517]) && s.b[522]) {
            let assign24390_ad_e38454: A = {
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
            let assign24390_ad_e38492: A = {
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign24390_ad_e38454, (-3.24e17), s.ad_value(83), assign24390_ad_e38492, (-3.24e17)));
        }

        if (((!s.b[508]) && s.b[517]) && s.b[522]) {
            s.store_mul_scaled_ad_rhs(121, 136, p.p195, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_scaled_ad_rhs(122, 136, p.p196, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.store_add(285, 128, 86);
        }

        if (((!s.b[508]) && s.b[517]) && (!s.b[522])) {
            s.store_add(285, 100, 86);
        }

        if ((!s.b[508]) && s.b[517]) {
            s.store_scaled_add(286, 284, 285, 0.5);
        }

    }

    pub(super) fn stamp_transient_block_22(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((!s.b[508]) && s.b[517]) {
            s.store_sub(287, 285, 284);
            s.store_sub(90, 285, 284);
            s.store_ad_value(91, A::add_scaled_inputs3(s.ad_value(282), 1.0, s.ad_value(83), 1.0, s.ad_value(286), -1.0));
            s.store_mul_scaled_ad_rhs(137, 283, ((p.p4 * p.p5) * p.p187), A::add_scaled_inputs3(s.ad_value(282), 1.0, s.ad_value(286), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_scale(188, 137, (1.0 / (p.p242) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p241, 1.0);
            s.store_div_from_scalar(190, p.p240, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p186);
            s.store_mul_scaled_ad_rhs(288, 191, ((p.p4 * p.p5) * p.p187), A::add_scaled_inputs3(s.ad_value(282), 1.0, s.ad_value(286), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_ad_value(136, A::add_scaled_inputs3(s.ad_value(282), 1.0, s.ad_value(83), 1.0, s.ad_value(286), -1.0));
            s.store_add_scaled_inputs(90, 284, 0.3333333333333333, 285, (2.0 * 0.3333333333333333));
            s.store_ad_value(91, A::div_scaled_inputs(A::square(s.ad_value(287)), (1.0 / 12.0), s.ad_value(136), 1.0));
            s.store_ad_value(137, A::div_scaled_product(A::square(s.ad_value(287)), s.ad_value(287), (1.0 / 120.0), A::square(s.ad_value(136)), 1.0));
            s.store_mul_scaled_ad_rhs(289, 191, (-(((p.p4 * p.p187) * p.p5) * 0.5)), A::add_scaled_inputs4(s.ad_value(282), 1.0, s.ad_value(90), (-1.0), s.ad_value(91), 1.0, s.ad_value(137), 1.0));
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
            s.store_sub_ad(88, A::sub_from_scalar(p.p198, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p201)), (((-1.0)) * (p.p201)))), A::div_scaled_inputs(s.ad_value(304), (p.p207 * p.p206), A::sqrt(A::offset(A::square(s.ad_value(304)), (p.p207 * p.p207))), 1.0));
            s.store_scalar(295, (p.p9 / p.p199));
            s.store_div_from_scalar_ad(136, p.p200, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_ad_value(159, A::add_scaled_product(s.ad_value(88), 1.0, s.ad_value(83), A::ln_scaled_input(s.ad_value(136), p.p197), 1.0));
            s.store_ad_value(160, A::add_scaled_inputs4(s.ad_value(302), 0.5, s.ad_value(159), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(302), s.ad_value(159)), A::sub(s.ad_value(302), s.ad_value(159))), 0.0001)), 0.5, s.ad_value(159), 1.0));
            s.store_sub(294, 160, 88);
            s.store_scaled_div(84, 295, 83, (1.0 / ((1.602176634e-19 * 3.24e17))));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 295, 6.241509074460763e18);
            s.store_scaled_add_ad_rhs(154, 294, A::sqrt(A::offset(A::square(s.ad_value(294)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_ad_value(155, A::div_scaled_product(s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0));
            s.store_ad_value(130, A::div_scaled_product(s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0));
        }

        if (s.b[523] && s.b[524]) {
            let assign24980_ad_e39249: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign24980_ad_e39249);
        }

        if (s.b[523] && s.b[524]) {
            s.store_scaled_div(136, 294, 83, (1.0 / (2.0)));
        }

        s.b[527] = (s.v[136] < 200.0);
        s.v[527] = if s.b[527] { 1.0 } else { 0.0 };

        if ((s.b[523] && s.b[524]) && s.b[527]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(294), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((s.b[523] && s.b[524]) && (!s.b[527])) {
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(294), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (s.b[523] && s.b[524]) {
            s.store_sub_ad_rhs(100, 294, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.b[528] = ((((s.v[100] - s.v[294])) as f64).abs() > 1e-19);
        s.v[528] = if s.b[528] { 1.0 } else { 0.0 };

        if ((s.b[523] && s.b[524]) && s.b[528]) {
            s.store_sub(101, 294, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p208);
            s.store_scaled_mul(103, 136, 90, p.p209);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if ((s.b[523] && s.b[524]) && s.b[528]) {
            let assign25160_ad_e39543: A = {
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
            let assign25160_ad_e39581: A = {
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign25160_ad_e39543, (-3.24e17), s.ad_value(83), assign25160_ad_e39581, (-3.24e17)));
        }

        if ((s.b[523] && s.b[524]) && s.b[528]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 294, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p208, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p209, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if ((s.b[523] && s.b[524]) && s.b[528]) {
            let assign25320_ad_e39840: A = {
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
            let assign25320_ad_e39878: A = {
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign25320_ad_e39840, (-3.24e17), s.ad_value(83), assign25320_ad_e39878, (-3.24e17)));
        }

        if ((s.b[523] && s.b[524]) && s.b[528]) {
            s.store_scaled_mul(121, 136, 137, p.p208);
            s.store_scaled_mul(122, 136, 137, p.p209);
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.copy_ad(296, 128);
        }

        if ((s.b[523] && s.b[524]) && (!s.b[528])) {
            s.copy_ad(296, 100);
        }

        if (s.b[523] && s.b[524]) {
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p202);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p203);
            s.store_mul_scaled_ad_rhs(136, 295, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(294), s.ad_value(296))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(296)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_scaled_div(136, 89, 95, 2.0);
            s.store_scaled_add_ad_rhs(90, 294, A::sqrt(A::offset(A::square(s.ad_value(294)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_ad_value(85, A::div_scaled_product(s.ad_value(136), s.ad_value(90), p.p200, A::add_scaled_inputs(s.ad_value(136), p.p200, s.ad_value(90), 1.0), 1.0));
            s.store_powf_ad(136, A::div(s.ad_value(303), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 303, 90);
            s.store_sub(39, 294, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_ad_rhs(131, 130, A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.copy_ad(154, 131);
            s.store_ad_value(157, A::div_scaled_product(s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0));
            s.store_ad_value(158, A::div_scaled_product(s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0));
        }

        if (s.b[523] && s.b[524]) {
            let assign25600_ad_e40278: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign25600_ad_e40278);
        }

    }

    pub(super) fn stamp_transient_block_23(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[523] && s.b[524]) {
            s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));
        }

        s.b[529] = (s.v[136] < 200.0);
        s.v[529] = if s.b[529] { 1.0 } else { 0.0 };

        if ((s.b[523] && s.b[524]) && s.b[529]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((s.b[523] && s.b[524]) && (!s.b[529])) {
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (s.b[523] && s.b[524]) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.b[530] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[530] = if s.b[530] { 1.0 } else { 0.0 };

        if ((s.b[523] && s.b[524]) && s.b[530]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p208);
            s.store_scaled_mul(103, 136, 90, p.p209);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if ((s.b[523] && s.b[524]) && s.b[530]) {
            let assign25780_ad_e40572: A = {
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
            let assign25780_ad_e40610: A = {
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign25780_ad_e40572, (-3.24e17), s.ad_value(83), assign25780_ad_e40610, (-3.24e17)));
        }

        if ((s.b[523] && s.b[524]) && s.b[530]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_mul_scaled_ad_rhs(116, 136, p.p208, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p209, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if ((s.b[523] && s.b[524]) && s.b[530]) {
            let assign25930_ad_e40858: A = {
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
            let assign25930_ad_e40896: A = {
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign25930_ad_e40858, (-3.24e17), s.ad_value(83), assign25930_ad_e40896, (-3.24e17)));
        }

        if ((s.b[523] && s.b[524]) && s.b[530]) {
            s.store_mul_scaled_ad_rhs(121, 136, p.p208, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_scaled_ad_rhs(122, 136, p.p209, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.store_add(297, 128, 86);
        }

        if ((s.b[523] && s.b[524]) && (!s.b[530])) {
            s.store_add(297, 100, 86);
        }

        if (s.b[523] && s.b[524]) {
            s.store_scaled_add(298, 296, 297, 0.5);
            s.store_sub(299, 297, 296);
            s.store_mul_ad_lhs(135, A::add_scaled_inputs3(s.ad_value(294), 1.0, s.ad_value(298), (-1.0), s.ad_value(83), 1.0), 299);
            s.store_mul_scaled_ad_rhs(136, 295, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(294), s.ad_value(298))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs(A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, s.ad_value(90), p.p16));
            s.store_scaled_mul(96, 95, 295, (p.p4 * (p.p5 * 1.0 / (p.p200))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(304), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(299), (p.p25 * p.p25), s.ad_value(299)), 1.0);
            s.store_div(93, 98, 92);
            s.store_mul(305, 93, 135);
            s.store_sub(90, 297, 296);
            s.store_ad_value(91, A::add_scaled_inputs3(s.ad_value(294), 1.0, s.ad_value(83), 1.0, s.ad_value(298), -1.0));
            s.store_mul_scaled_ad_rhs(137, 295, ((p.p4 * p.p5) * p.p200), A::add_scaled_inputs3(s.ad_value(294), 1.0, s.ad_value(298), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_scale(188, 137, (1.0 / (p.p245) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p244, 1.0);
            s.store_div_from_scalar(190, p.p243, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p199);
            s.store_mul_scaled_ad_rhs(300, 191, ((p.p4 * p.p5) * p.p200), A::add_scaled_inputs3(s.ad_value(294), 1.0, s.ad_value(298), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_ad_value(136, A::add_scaled_inputs3(s.ad_value(294), 1.0, s.ad_value(83), 1.0, s.ad_value(298), -1.0));
            s.store_add_scaled_inputs(90, 296, 0.3333333333333333, 297, (2.0 * 0.3333333333333333));
            s.store_ad_value(91, A::div_scaled_inputs(A::square(s.ad_value(299)), (1.0 / 12.0), s.ad_value(136), 1.0));
            s.store_ad_value(137, A::div_scaled_product(A::square(s.ad_value(299)), s.ad_value(299), (1.0 / 120.0), A::square(s.ad_value(136)), 1.0));
            s.store_mul_scaled_ad_rhs(301, 191, (-(((p.p4 * p.p200) * p.p5) * 0.5)), A::add_scaled_inputs4(s.ad_value(294), 1.0, s.ad_value(90), (-1.0), s.ad_value(91), 1.0, s.ad_value(137), 1.0));
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
            s.store_div_from_scalar_ad(136, p.p200, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_ad_value(159, A::add_scaled_product(s.ad_value(88), 1.0, s.ad_value(83), A::ln_scaled_input(s.ad_value(136), p.p197), 1.0));
            s.store_ad_value(160, A::add_scaled_inputs4(s.ad_value(302), 0.5, s.ad_value(159), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(302), s.ad_value(159)), A::sub(s.ad_value(302), s.ad_value(159))), 0.0001)), 0.5, s.ad_value(159), 1.0));
            s.store_sub(294, 160, 88);
            s.store_scaled_div(84, 295, 83, (1.0 / ((1.602176634e-19 * 3.24e17))));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 295, 6.241509074460763e18);
            s.store_scaled_add_ad_rhs(154, 294, A::sqrt(A::offset(A::square(s.ad_value(294)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_ad_value(155, A::div_scaled_product(s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0));
            s.store_ad_value(130, A::div_scaled_product(s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0));
        }

        if ((!s.b[523]) && s.b[532]) {
            let assign26520_ad_e41676: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign26520_ad_e41676);
        }

        if ((!s.b[523]) && s.b[532]) {
            s.store_scaled_div(136, 294, 83, (1.0 / (2.0)));
        }

        s.b[534] = (s.v[136] < 200.0);
        s.v[534] = if s.b[534] { 1.0 } else { 0.0 };

        if (((!s.b[523]) && s.b[532]) && s.b[534]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(294), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (((!s.b[523]) && s.b[532]) && (!s.b[534])) {
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(294), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((!s.b[523]) && s.b[532]) {
            s.store_sub_ad_rhs(100, 294, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.b[535] = ((((s.v[100] - s.v[294])) as f64).abs() > 1e-19);
        s.v[535] = if s.b[535] { 1.0 } else { 0.0 };

        if (((!s.b[523]) && s.b[532]) && s.b[535]) {
            s.store_sub(101, 294, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p208);
            s.store_scaled_mul(103, 136, 90, p.p209);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

    }

    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[523]) && s.b[532]) && s.b[535]) {
            let assign26700_ad_e41986: A = {
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
            let assign26700_ad_e42024: A = {
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign26700_ad_e41986, (-3.24e17), s.ad_value(83), assign26700_ad_e42024, (-3.24e17)));
        }

        if (((!s.b[523]) && s.b[532]) && s.b[535]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 294, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p208, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p209, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!s.b[523]) && s.b[532]) && s.b[535]) {
            let assign26860_ad_e42299: A = {
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
            let assign26860_ad_e42337: A = {
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign26860_ad_e42299, (-3.24e17), s.ad_value(83), assign26860_ad_e42337, (-3.24e17)));
        }

        if (((!s.b[523]) && s.b[532]) && s.b[535]) {
            s.store_scaled_mul(121, 136, 137, p.p208);
            s.store_scaled_mul(122, 136, 137, p.p209);
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.copy_ad(296, 128);
        }

        if (((!s.b[523]) && s.b[532]) && (!s.b[535])) {
            s.copy_ad(296, 100);
        }

        if ((!s.b[523]) && s.b[532]) {
            s.store_scalar(303, 0.0);
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p202);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p203);
            s.store_mul_scaled_ad_rhs(136, 295, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(294), s.ad_value(296))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(296)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_scaled_div(136, 89, 95, 2.0);
            s.store_scaled_add_ad_rhs(90, 294, A::sqrt(A::offset(A::square(s.ad_value(294)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_ad_value(85, A::div_scaled_product(s.ad_value(136), s.ad_value(90), p.p200, A::add_scaled_inputs(s.ad_value(136), p.p200, s.ad_value(90), 1.0), 1.0));
            s.store_powf_ad(136, A::div(s.ad_value(303), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 303, 90);
            s.store_sub(39, 294, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_ad_rhs(131, 130, A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.copy_ad(154, 131);
            s.store_ad_value(157, A::div_scaled_product(s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0));
            s.store_ad_value(158, A::div_scaled_product(s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0));
        }

        if ((!s.b[523]) && s.b[532]) {
            let assign27150_ad_e42772: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign27150_ad_e42772);
        }

        if ((!s.b[523]) && s.b[532]) {
            s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));
        }

        s.b[536] = (s.v[136] < 200.0);
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        if (((!s.b[523]) && s.b[532]) && s.b[536]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (((!s.b[523]) && s.b[532]) && (!s.b[536])) {
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((!s.b[523]) && s.b[532]) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.b[537] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[537] = if s.b[537] { 1.0 } else { 0.0 };

        if (((!s.b[523]) && s.b[532]) && s.b[537]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p208);
            s.store_scaled_mul(103, 136, 90, p.p209);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!s.b[523]) && s.b[532]) && s.b[537]) {
            let assign27330_ad_e43082: A = {
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
            let assign27330_ad_e43120: A = {
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign27330_ad_e43082, (-3.24e17), s.ad_value(83), assign27330_ad_e43120, (-3.24e17)));
        }

        if (((!s.b[523]) && s.b[532]) && s.b[537]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_mul_scaled_ad_rhs(116, 136, p.p208, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p209, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!s.b[523]) && s.b[532]) && s.b[537]) {
            let assign27480_ad_e43383: A = {
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
            let assign27480_ad_e43421: A = {
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign27480_ad_e43383, (-3.24e17), s.ad_value(83), assign27480_ad_e43421, (-3.24e17)));
        }

        if (((!s.b[523]) && s.b[532]) && s.b[537]) {
            s.store_mul_scaled_ad_rhs(121, 136, p.p208, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_scaled_ad_rhs(122, 136, p.p209, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.store_add(297, 128, 86);
        }

        if (((!s.b[523]) && s.b[532]) && (!s.b[537])) {
            s.store_add(297, 100, 86);
        }

        if ((!s.b[523]) && s.b[532]) {
            s.store_scaled_add(298, 296, 297, 0.5);
        }

    }

    pub(super) fn stamp_transient_block_25(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((!s.b[523]) && s.b[532]) {
            s.store_sub(299, 297, 296);
            s.store_sub(90, 297, 296);
            s.store_ad_value(91, A::add_scaled_inputs3(s.ad_value(294), 1.0, s.ad_value(83), 1.0, s.ad_value(298), -1.0));
            s.store_mul_scaled_ad_rhs(137, 295, ((p.p4 * p.p5) * p.p200), A::add_scaled_inputs3(s.ad_value(294), 1.0, s.ad_value(298), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_scale(188, 137, (1.0 / (p.p245) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p244, 1.0);
            s.store_div_from_scalar(190, p.p243, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p199);
            s.store_mul_scaled_ad_rhs(300, 191, ((p.p4 * p.p5) * p.p200), A::add_scaled_inputs3(s.ad_value(294), 1.0, s.ad_value(298), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_ad_value(136, A::add_scaled_inputs3(s.ad_value(294), 1.0, s.ad_value(83), 1.0, s.ad_value(298), -1.0));
            s.store_add_scaled_inputs(90, 296, 0.3333333333333333, 297, (2.0 * 0.3333333333333333));
            s.store_ad_value(91, A::div_scaled_inputs(A::square(s.ad_value(299)), (1.0 / 12.0), s.ad_value(136), 1.0));
            s.store_ad_value(137, A::div_scaled_product(A::square(s.ad_value(299)), s.ad_value(299), (1.0 / 120.0), A::square(s.ad_value(136)), 1.0));
            s.store_mul_scaled_ad_rhs(301, 191, (-(((p.p4 * p.p200) * p.p5) * 0.5)), A::add_scaled_inputs4(s.ad_value(294), 1.0, s.ad_value(90), (-1.0), s.ad_value(91), 1.0, s.ad_value(137), 1.0));
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
            s.store_sub_ad(88, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p201)), (((((-1.0)) * (p.p201))) + (p.p198))), A::div_scaled_inputs(s.ad_value(316), (p.p207 * p.p206), A::sqrt(A::offset(A::square(s.ad_value(316)), (p.p207 * p.p207))), 1.0));
            s.store_scalar(307, (p.p9 / p.p199));
            s.store_div_from_scalar_ad(136, p.p200, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_ad_value(159, A::add_scaled_product(s.ad_value(88), 1.0, s.ad_value(83), A::ln_scaled_input(s.ad_value(136), p.p197), 1.0));
            s.store_ad_value(160, A::add_scaled_inputs4(s.ad_value(314), 0.5, s.ad_value(159), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(314), s.ad_value(159)), A::sub(s.ad_value(314), s.ad_value(159))), 0.0001)), 0.5, s.ad_value(159), 1.0));
            s.store_sub(306, 160, 88);
            s.store_scaled_div(84, 307, 83, (1.0 / ((1.602176634e-19 * 3.24e17))));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 307, 6.241509074460763e18);
            s.store_scaled_add_ad_rhs(154, 306, A::sqrt(A::offset(A::square(s.ad_value(306)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_ad_value(155, A::div_scaled_product(s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0));
            s.store_ad_value(130, A::div_scaled_product(s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0));
        }

        if (s.b[538] && s.b[539]) {
            let assign28070_ad_e44178: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign28070_ad_e44178);
        }

        if (s.b[538] && s.b[539]) {
            s.store_scaled_div(136, 306, 83, (1.0 / (2.0)));
        }

        s.b[542] = (s.v[136] < 200.0);
        s.v[542] = if s.b[542] { 1.0 } else { 0.0 };

        if ((s.b[538] && s.b[539]) && s.b[542]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((s.b[538] && s.b[539]) && (!s.b[542])) {
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (s.b[538] && s.b[539]) {
            s.store_sub_ad_rhs(100, 306, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.b[543] = ((((s.v[100] - s.v[306])) as f64).abs() > 1e-19);
        s.v[543] = if s.b[543] { 1.0 } else { 0.0 };

        if ((s.b[538] && s.b[539]) && s.b[543]) {
            s.store_sub(101, 306, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p208);
            s.store_scaled_mul(103, 136, 90, p.p209);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if ((s.b[538] && s.b[539]) && s.b[543]) {
            let assign28250_ad_e44472: A = {
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
            let assign28250_ad_e44510: A = {
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign28250_ad_e44472, (-3.24e17), s.ad_value(83), assign28250_ad_e44510, (-3.24e17)));
        }

        if ((s.b[538] && s.b[539]) && s.b[543]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 306, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p208, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p209, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if ((s.b[538] && s.b[539]) && s.b[543]) {
            let assign28410_ad_e44769: A = {
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
            let assign28410_ad_e44807: A = {
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign28410_ad_e44769, (-3.24e17), s.ad_value(83), assign28410_ad_e44807, (-3.24e17)));
        }

        if ((s.b[538] && s.b[539]) && s.b[543]) {
            s.store_scaled_mul(121, 136, 137, p.p208);
            s.store_scaled_mul(122, 136, 137, p.p209);
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.copy_ad(308, 128);
        }

        if ((s.b[538] && s.b[539]) && (!s.b[543])) {
            s.copy_ad(308, 100);
        }

        if (s.b[538] && s.b[539]) {
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p202);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p203);
            s.store_mul_scaled_ad_rhs(136, 307, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(306), s.ad_value(308))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(308)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_scaled_div(136, 89, 95, 2.0);
            s.store_scaled_add_ad_rhs(90, 306, A::sqrt(A::offset(A::square(s.ad_value(306)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_ad_value(85, A::div_scaled_product(s.ad_value(136), s.ad_value(90), p.p200, A::add_scaled_inputs(s.ad_value(136), p.p200, s.ad_value(90), 1.0), 1.0));
            s.store_powf_ad(136, A::div(s.ad_value(315), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 315, 90);
            s.store_sub(39, 306, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_ad_rhs(131, 130, A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.copy_ad(154, 131);
            s.store_ad_value(157, A::div_scaled_product(s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0));
            s.store_ad_value(158, A::div_scaled_product(s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0));
        }

        if (s.b[538] && s.b[539]) {
            let assign28690_ad_e45207: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign28690_ad_e45207);
        }

    }

    pub(super) fn stamp_transient_block_26(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[538] && s.b[539]) {
            s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));
        }

        s.b[544] = (s.v[136] < 200.0);
        s.v[544] = if s.b[544] { 1.0 } else { 0.0 };

        if ((s.b[538] && s.b[539]) && s.b[544]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((s.b[538] && s.b[539]) && (!s.b[544])) {
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (s.b[538] && s.b[539]) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.b[545] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[545] = if s.b[545] { 1.0 } else { 0.0 };

        if ((s.b[538] && s.b[539]) && s.b[545]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p208);
            s.store_scaled_mul(103, 136, 90, p.p209);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if ((s.b[538] && s.b[539]) && s.b[545]) {
            let assign28870_ad_e45501: A = {
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
            let assign28870_ad_e45539: A = {
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign28870_ad_e45501, (-3.24e17), s.ad_value(83), assign28870_ad_e45539, (-3.24e17)));
        }

        if ((s.b[538] && s.b[539]) && s.b[545]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_mul_scaled_ad_rhs(116, 136, p.p208, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p209, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if ((s.b[538] && s.b[539]) && s.b[545]) {
            let assign29020_ad_e45787: A = {
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
            let assign29020_ad_e45825: A = {
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign29020_ad_e45787, (-3.24e17), s.ad_value(83), assign29020_ad_e45825, (-3.24e17)));
        }

        if ((s.b[538] && s.b[539]) && s.b[545]) {
            s.store_mul_scaled_ad_rhs(121, 136, p.p208, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_scaled_ad_rhs(122, 136, p.p209, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.store_add(309, 128, 86);
        }

        if ((s.b[538] && s.b[539]) && (!s.b[545])) {
            s.store_add(309, 100, 86);
        }

        if (s.b[538] && s.b[539]) {
            s.store_scaled_add(310, 308, 309, 0.5);
            s.store_sub(311, 309, 308);
            s.store_mul_ad_lhs(135, A::add_scaled_inputs3(s.ad_value(306), 1.0, s.ad_value(310), (-1.0), s.ad_value(83), 1.0), 311);
            s.store_mul_scaled_ad_rhs(136, 307, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(306), s.ad_value(310))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs(A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, s.ad_value(90), p.p16));
            s.store_scaled_mul(96, 95, 307, (p.p4 * (p.p5 * 1.0 / (p.p200))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(316), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(311), (p.p25 * p.p25), s.ad_value(311)), 1.0);
            s.store_div(93, 98, 92);
            s.store_mul(317, 93, 135);
            s.store_sub(90, 309, 308);
            s.store_ad_value(91, A::add_scaled_inputs3(s.ad_value(306), 1.0, s.ad_value(83), 1.0, s.ad_value(310), -1.0));
            s.store_mul_scaled_ad_rhs(137, 307, ((p.p4 * p.p5) * p.p200), A::add_scaled_inputs3(s.ad_value(306), 1.0, s.ad_value(310), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_scale(188, 137, (1.0 / (p.p245) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p244, 1.0);
            s.store_div_from_scalar(190, p.p243, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p199);
            s.store_mul_scaled_ad_rhs(312, 191, ((p.p4 * p.p5) * p.p200), A::add_scaled_inputs3(s.ad_value(306), 1.0, s.ad_value(310), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_ad_value(136, A::add_scaled_inputs3(s.ad_value(306), 1.0, s.ad_value(83), 1.0, s.ad_value(310), -1.0));
            s.store_add_scaled_inputs(90, 308, 0.3333333333333333, 309, (2.0 * 0.3333333333333333));
            s.store_ad_value(91, A::div_scaled_inputs(A::square(s.ad_value(311)), (1.0 / 12.0), s.ad_value(136), 1.0));
            s.store_ad_value(137, A::div_scaled_product(A::square(s.ad_value(311)), s.ad_value(311), (1.0 / 120.0), A::square(s.ad_value(136)), 1.0));
            s.store_mul_scaled_ad_rhs(313, 191, (-(((p.p4 * p.p200) * p.p5) * 0.5)), A::add_scaled_inputs4(s.ad_value(306), 1.0, s.ad_value(90), (-1.0), s.ad_value(91), 1.0, s.ad_value(137), 1.0));
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
            s.store_div_from_scalar_ad(136, p.p200, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));
            s.store_ad_value(159, A::add_scaled_product(s.ad_value(88), 1.0, s.ad_value(83), A::ln_scaled_input(s.ad_value(136), p.p197), 1.0));
            s.store_ad_value(160, A::add_scaled_inputs4(s.ad_value(314), 0.5, s.ad_value(159), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(314), s.ad_value(159)), A::sub(s.ad_value(314), s.ad_value(159))), 0.0001)), 0.5, s.ad_value(159), 1.0));
            s.store_sub(306, 160, 88);
            s.store_scaled_div(84, 307, 83, (1.0 / ((1.602176634e-19 * 3.24e17))));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 307, 6.241509074460763e18);
            s.store_scaled_add_ad_rhs(154, 306, A::sqrt(A::offset(A::square(s.ad_value(306)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_ad_value(155, A::div_scaled_product(s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0));
            s.store_ad_value(130, A::div_scaled_product(s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0));
        }

        if ((!s.b[538]) && s.b[547]) {
            let assign29610_ad_e46605: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign29610_ad_e46605);
        }

        if ((!s.b[538]) && s.b[547]) {
            s.store_scaled_div(136, 306, 83, (1.0 / (2.0)));
        }

        s.b[549] = (s.v[136] < 200.0);
        s.v[549] = if s.b[549] { 1.0 } else { 0.0 };

        if (((!s.b[538]) && s.b[547]) && s.b[549]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (((!s.b[538]) && s.b[547]) && (!s.b[549])) {
            s.store_ad_value(153, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(306), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((!s.b[538]) && s.b[547]) {
            s.store_sub_ad_rhs(100, 306, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.b[550] = ((((s.v[100] - s.v[306])) as f64).abs() > 1e-19);
        s.v[550] = if s.b[550] { 1.0 } else { 0.0 };

        if (((!s.b[538]) && s.b[547]) && s.b[550]) {
            s.store_sub(101, 306, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p208);
            s.store_scaled_mul(103, 136, 90, p.p209);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

    }

    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[538]) && s.b[547]) && s.b[550]) {
            let assign29790_ad_e46915: A = {
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
            let assign29790_ad_e46953: A = {
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign29790_ad_e46915, (-3.24e17), s.ad_value(83), assign29790_ad_e46953, (-3.24e17)));
        }

        if (((!s.b[538]) && s.b[547]) && s.b[550]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 306, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p208, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p209, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!s.b[538]) && s.b[547]) && s.b[550]) {
            let assign29950_ad_e47228: A = {
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
            let assign29950_ad_e47266: A = {
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign29950_ad_e47228, (-3.24e17), s.ad_value(83), assign29950_ad_e47266, (-3.24e17)));
        }

        if (((!s.b[538]) && s.b[547]) && s.b[550]) {
            s.store_scaled_mul(121, 136, 137, p.p208);
            s.store_scaled_mul(122, 136, 137, p.p209);
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.copy_ad(308, 128);
        }

        if (((!s.b[538]) && s.b[547]) && (!s.b[550])) {
            s.copy_ad(308, 100);
        }

        if ((!s.b[538]) && s.b[547]) {
            s.store_scalar(315, 0.0);
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p202);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p203);
            s.store_mul_scaled_ad_rhs(136, 307, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(306), s.ad_value(308))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(308)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_scaled_div(136, 89, 95, 2.0);
            s.store_scaled_add_ad_rhs(90, 306, A::sqrt(A::offset(A::square(s.ad_value(306)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.store_ad_value(85, A::div_scaled_product(s.ad_value(136), s.ad_value(90), p.p200, A::add_scaled_inputs(s.ad_value(136), p.p200, s.ad_value(90), 1.0), 1.0));
            s.store_powf_ad(136, A::div(s.ad_value(315), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 315, 90);
            s.store_sub(39, 306, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_ad_rhs(131, 130, A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5);
            s.copy_ad(154, 131);
            s.store_ad_value(157, A::div_scaled_product(s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0));
            s.store_ad_value(158, A::div_scaled_product(s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0));
        }

        if ((!s.b[538]) && s.b[547]) {
            let assign30240_ad_e47701: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (-(p.p208 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
            s.store_ad_value(152, assign30240_ad_e47701);
        }

        if ((!s.b[538]) && s.b[547]) {
            s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));
        }

        s.b[551] = (s.v[136] < 200.0);
        s.v[551] = if s.b[551] { 1.0 } else { 0.0 };

        if (((!s.b[538]) && s.b[547]) && s.b[551]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if (((!s.b[538]) && s.b[547]) && (!s.b[551])) {
            s.store_ad_value(156, A::div_scaled_product3(s.ad_value(83), s.ad_value(99), s.ad_value(136), (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), 1.0 / (3.24e17)), 1.0));
        }

        if ((!s.b[538]) && s.b[547]) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.b[552] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[552] = if s.b[552] { 1.0 } else { 0.0 };

        if (((!s.b[538]) && s.b[547]) && s.b[552]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p208);
            s.store_scaled_mul(103, 136, 90, p.p209);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!s.b[538]) && s.b[547]) && s.b[552]) {
            let assign30420_ad_e48011: A = {
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
            let assign30420_ad_e48049: A = {
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
            s.store_ad_value(106, A::add_scaled_products3(s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), assign30420_ad_e48011, (-3.24e17), s.ad_value(83), assign30420_ad_e48049, (-3.24e17)));
        }

        if (((!s.b[538]) && s.b[547]) && s.b[552]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_ad_value(113, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_mul_scaled_ad_rhs(116, 136, p.p208, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p209, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!s.b[538]) && s.b[547]) && s.b[552]) {
            let assign30570_ad_e48312: A = {
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
            let assign30570_ad_e48350: A = {
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
            s.store_ad_value(120, A::add_scaled_products3(s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), assign30570_ad_e48312, (-3.24e17), s.ad_value(83), assign30570_ad_e48350, (-3.24e17)));
        }

        if (((!s.b[538]) && s.b[547]) && s.b[552]) {
            s.store_mul_scaled_ad_rhs(121, 136, p.p208, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_scaled_ad_rhs(122, 136, p.p209, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_ad_value(127, A::add_scaled_inputs3(s.ad_value(99), (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.store_add(309, 128, 86);
        }

        if (((!s.b[538]) && s.b[547]) && (!s.b[552])) {
            s.store_add(309, 100, 86);
        }

        if ((!s.b[538]) && s.b[547]) {
            s.store_scaled_add(310, 308, 309, 0.5);
        }

    }

    pub(super) fn stamp_transient_block_28(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((!s.b[538]) && s.b[547]) {
            s.store_sub(311, 309, 308);
            s.store_sub(90, 309, 308);
            s.store_ad_value(91, A::add_scaled_inputs3(s.ad_value(306), 1.0, s.ad_value(83), 1.0, s.ad_value(310), -1.0));
            s.store_mul_scaled_ad_rhs(137, 307, ((p.p4 * p.p5) * p.p200), A::add_scaled_inputs3(s.ad_value(306), 1.0, s.ad_value(310), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_scale(188, 137, (1.0 / (p.p245) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p244, 1.0);
            s.store_div_from_scalar(190, p.p243, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p199);
            s.store_mul_scaled_ad_rhs(312, 191, ((p.p4 * p.p5) * p.p200), A::add_scaled_inputs3(s.ad_value(306), 1.0, s.ad_value(310), (-1.0), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), 1.0));
            s.store_ad_value(136, A::add_scaled_inputs3(s.ad_value(306), 1.0, s.ad_value(83), 1.0, s.ad_value(310), -1.0));
            s.store_add_scaled_inputs(90, 308, 0.3333333333333333, 309, (2.0 * 0.3333333333333333));
            s.store_ad_value(91, A::div_scaled_inputs(A::square(s.ad_value(311)), (1.0 / 12.0), s.ad_value(136), 1.0));
            s.store_ad_value(137, A::div_scaled_product(A::square(s.ad_value(311)), s.ad_value(311), (1.0 / 120.0), A::square(s.ad_value(136)), 1.0));
            s.store_mul_scaled_ad_rhs(313, 191, (-(((p.p4 * p.p200) * p.p5) * 0.5)), A::add_scaled_inputs4(s.ad_value(306), 1.0, s.ad_value(90), (-1.0), s.ad_value(91), 1.0, s.ad_value(137), 1.0));
        }

        if ((!s.b[538]) && (!s.b[547])) {
            s.store_scalar(312, 0.0);
            s.store_scalar(313, 0.0);
        }

        s.b[553] = (p.p255 == 1.0);
        s.v[553] = if s.b[553] { 1.0 } else { 0.0 };

        if s.b[553] {
            s.store_scalar(318, ((p.p258 * (p.p256 + ((p.p4 / 3.0) / p.p257))) / ((p.p257 * p.p5) * p.p3)));
        }

        s.b[554] = (s.v[318] > 0.0);
        s.v[554] = if s.b[554] { 1.0 } else { 0.0 };

        if (s.b[553] && s.b[554]) {
            s.store_div_from_scalar(318, 1.0, 318);
        }

        if (s.b[553] && (!s.b[554])) {
            s.store_scalar(318, (1.0 / 0.001));
        }

        s.b[555] = (p.p255 == 2.0);
        s.v[555] = if s.b[555] { 1.0 } else { 0.0 };

        if ((!s.b[553]) && s.b[555]) {
            s.store_scalar(319, ((p.p258 * (p.p256 + ((p.p4 / 3.0) / p.p257))) / ((p.p257 * p.p5) * p.p3)));
            s.store_scalar(320, ((p.p258 * (((2.0 * p.p4) / 3.0) / p.p257)) / ((p.p257 * p.p5) * p.p3)));
        }

        s.b[556] = (s.v[319] > 0.0);
        s.v[556] = if s.b[556] { 1.0 } else { 0.0 };

        if (((!s.b[553]) && s.b[555]) && s.b[556]) {
            s.store_div_from_scalar(319, 1.0, 319);
        }

        if (((!s.b[553]) && s.b[555]) && (!s.b[556])) {
            s.store_scalar(319, (1.0 / 0.001));
        }

        s.b[557] = (s.v[320] > 0.0);
        s.v[557] = if s.b[557] { 1.0 } else { 0.0 };

        if (((!s.b[553]) && s.b[555]) && s.b[557]) {
            s.store_div_from_scalar(320, 1.0, 320);
        }

        if (((!s.b[553]) && s.b[555]) && (!s.b[557])) {
            s.store_scalar(320, (1.0 / 0.001));
        }

        s.b[558] = (p.p255 == 2.0);
        s.v[558] = if s.b[558] { 1.0 } else { 0.0 };

        if s.b[558] {
            s.store_scaled_voltage(162, ctx, nodes, Some(10), Some(2), ((p.p4 * p.p5) * p.p210));
            s.store_ad_value(168, A::div_scaled_inputs(A::voltage(ctx, nodes, Some(0), Some(2)), p.p214, A::sqrt(A::offset(A::square(A::voltage(ctx, nodes, Some(0), Some(2))), (p.p214 * p.p214))), 1.0));
            s.store_scalar(169, (p.p213).min((p.p211 / (2.0 * p.p214))));
            s.store_sub_from_scalar_ad(167, ((p.p4 * p.p5) * p.p211), A::mul_scaled_lhs(s.ad_value(169), (p.p4 * p.p5), s.ad_value(168)));
            s.store_mul_voltage_ad(163, A::max_with_scalar(s.ad_value(167), 0.0), ctx, nodes, Some(10), Some(0));
        }

        if (!s.b[558]) {
            s.store_scaled_voltage(162, ctx, nodes, Some(1), Some(2), ((p.p4 * p.p5) * p.p210));
            s.store_ad_value(168, A::div_scaled_inputs(A::voltage(ctx, nodes, Some(0), Some(2)), p.p214, A::sqrt(A::offset(A::square(A::voltage(ctx, nodes, Some(0), Some(2))), (p.p214 * p.p214))), 1.0));
            s.store_scalar(169, (p.p213).min((p.p211 / (2.0 * p.p214))));
            s.store_sub_from_scalar_ad(167, ((p.p4 * p.p5) * p.p211), A::mul_scaled_lhs(s.ad_value(169), (p.p4 * p.p5), s.ad_value(168)));
            s.store_mul_voltage_ad(163, A::max_with_scalar(s.ad_value(167), 0.0), ctx, nodes, Some(1), Some(0));
        }

        s.store_scaled_voltage(164, ctx, nodes, Some(0), Some(2), ((p.p4 * p.p5) * p.p212));

        s.store_sub(217, 164, 163);

        s.store_sub_scaled_inputs(218, 162, -1.0, 164, 1.0);

        s.store_add(138, 165, 217);

        s.store_add(139, 166, 218);

        s.store_scaled_voltage(219, ctx, nodes, Some(3), Some(0), ((p.p4 * p.p5) * p.p215));

        s.store_scaled_voltage(220, ctx, nodes, Some(3), Some(2), ((p.p4 * p.p5) * p.p216));

        s.store_scaled_voltage(221, ctx, nodes, Some(3), Some(1), ((p.p4 * p.p5) * p.p217));

        s.store_offset_scaled(375, 82, ((1.0 / (s.v[35])) * (p.p285)), (((((-1.0)) * (p.p285))) + (p.p279)));

        s.store_offset_scaled(373, 82, ((1.0 / (s.v[35])) * (p.p283)), (((((-1.0)) * (p.p283))) + (p.p275)));

        s.store_scale_ad(377, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p281), p.p277);

        s.store_offset_scaled(376, 82, ((1.0 / (s.v[35])) * (p.p286)), (((((-1.0)) * (p.p286))) + (p.p280)));

        s.store_offset_scaled(374, 82, ((1.0 / (s.v[35])) * (p.p284)), (((((-1.0)) * (p.p284))) + (p.p276)));

        s.store_scale_ad(378, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p282), p.p278);

        s.store_scale(137, 378, (p.p4 * p.p5));

        s.store_max_with_scalar_ad(371, A::sub(A::voltage(ctx, nodes, Some(0), Some(3)), s.ad_value(376)), 0.0);

        s.b[559] = (s.v[137] > 0.0);
        s.v[559] = if s.b[559] { 1.0 } else { 0.0 };

        s.b[560] = (s.v[371] > 0.0);
        s.v[560] = if s.b[560] { 1.0 } else { 0.0 };

        if (s.b[559] && s.b[560]) {
            s.store_div_ad(354, A::powf(s.ad_value(371), 1.0), A::mul(s.ad_value(374), s.ad_value(36)));
        }

        s.b[561] = (s.v[354] > 80.0);
        s.v[561] = if s.b[561] { 1.0 } else { 0.0 };

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
        s.v[562] = if s.b[562] { 1.0 } else { 0.0 };

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

        s.store_max_with_scalar_ad(372, A::sub(A::voltage(ctx, nodes, Some(2), Some(3)), s.ad_value(375)), 0.0);

        s.store_scale(137, 377, (p.p4 * p.p5));

        s.b[563] = (s.v[137] > 0.0);
        s.v[563] = if s.b[563] { 1.0 } else { 0.0 };

        s.b[564] = (s.v[372] > 0.0);
        s.v[564] = if s.b[564] { 1.0 } else { 0.0 };

        if (s.b[563] && s.b[564]) {
            s.store_div_ad(354, A::powf(s.ad_value(372), 1.0), A::mul(s.ad_value(373), s.ad_value(36)));
        }

        s.b[565] = (s.v[354] > 80.0);
        s.v[565] = if s.b[565] { 1.0 } else { 0.0 };

        if ((s.b[563] && s.b[564]) && s.b[565]) {
            s.store_offset(355, 354, (((-80.0)) + (1.0)));
            s.store_scalar(354, 80.0);
        }

        if ((s.b[563] && s.b[564]) && (!s.b[565])) {
            s.store_scalar(355, 1.0);
        }

        if (s.b[563] && s.b[564]) {
            s.store_mul_exp_rhs(355, 355, 354);
            s.store_mul_offset_rhs(370, 137, 355, (-1.0));
        }

        if (s.b[563] && (!s.b[564])) {
            s.store_div_ad_rhs(354, 372, A::mul(s.ad_value(373), s.ad_value(36)));
        }

        s.b[566] = (s.v[354] > 80.0);
        s.v[566] = if s.b[566] { 1.0 } else { 0.0 };

        if ((s.b[563] && (!s.b[564])) && s.b[566]) {
            s.store_offset(355, 354, (((-80.0)) + (1.0)));
            s.store_scalar(354, 80.0);
        }

        if ((s.b[563] && (!s.b[564])) && (!s.b[566])) {
            s.store_scalar(355, 1.0);
        }

        if (s.b[563] && (!s.b[564])) {
            s.store_mul_exp_rhs(355, 355, 354);
            s.store_mul_offset_rhs(370, 137, 355, (-1.0));
        }

        if (!s.b[563]) {
            s.store_scalar(370, 0.0);
        }

        s.b[567] = (p.p259 == 1.0);
        s.v[567] = if s.b[567] { 1.0 } else { 0.0 };

        if s.b[567] {
            s.store_div_from_scalar_mul_ad(199, p.p3, A::add_scaled_inputs3(s.ad_value(37), 1.0, s.ad_value(133), (-1.0), s.ad_value(83), 1.0), A::max_with_scalar(s.ad_value(134), 1e-12));
            s.store_scale(198, 83, (1.602176634e-19 * (1.602176634e-19 * (1.602176634e-19 * 1.0 / ((((p.p4 * p.p5) * p.p3) * p.p3))))));
            s.store_mul_ad_affine_product_rhs(200, 83, A::div_from_scalar(1.0, A::max_with_scalar(s.ad_value(138), 1e-22)), A::sub_from_scalar(1.0, A::div(s.ad_value(138), A::max_with_scalar(s.ad_value(139), 1e-22))), (p.p261 * s.v[80]), 0.0);
            s.store_mul_ad(201, A::scale_offset(s.ad_value(83), (p.p262 * s.v[80]), p.p261), A::ln(A::div(A::max_with_scalar(s.ad_value(138), 1e-22), A::max_with_scalar(s.ad_value(139), 1e-22))));
            s.store_mul_ad(202, A::scale_offset(s.ad_value(83), (p.p263 * s.v[80]), p.p262), A::sub(s.ad_value(139), s.ad_value(138)));
            s.store_scaled_sub_ad(203, A::square(s.ad_value(138)), A::square(s.ad_value(139)), (p.p263 / 2.0));
            s.store_mul_ad(204, A::mul3_scaled_output(s.ad_value(198), A::square(s.ad_value(94)), s.ad_value(199), 1.0 / ((s.v[80] * s.v[80]))), A::add_scaled_inputs4(s.ad_value(200), 1.0, s.ad_value(201), 1.0, s.ad_value(202), 1.0, s.ad_value(203), 1.0));
        }

        s.b[568] = (s.v[41] < 0.0);
        s.v[568] = if s.b[568] { 1.0 } else { 0.0 };

        if (s.b[567] && s.b[568]) {
            s.store_neg(204, 204);
        }

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

        s.store_ad_value(195, A::add_scaled_inputs3_offset(s.ad_value(195), (p.p4 * p.p5), s.ad_value(195), ((-0.5) * (p.p4 * p.p5)), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(195), (-1e-25), A::offset(s.ad_value(195), (-1e-25))), p.p221)), ((-(-0.5)) * (p.p4 * p.p5)), ((1e-25 + ((-0.5) * 1e-25)) * (p.p4 * p.p5))));

        let assign32150_ad_e49745: A = A::add(A::offset(A::sub_from_scalar(p.p218, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p226)), (((-1.0)) * (p.p226)))), 1e-18), A::sqrt(A::offset(A::mul_offset_lhs(A::sub_from_scalar(p.p218, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p226)), (((-1.0)) * (p.p226)))), (-1e-18), A::offset(A::sub_from_scalar(p.p218, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p226)), (((-1.0)) * (p.p226)))), (-1e-18))), ((0.25 * 1e-19) * 1e-19))));
        s.store_scale_ad(136, assign32150_ad_e49745, 0.5);

        s.store_ad_value(196, A::mul_scaled_lhs(s.ad_value(136), (p.p4 * p.p5), A::voltage(ctx, nodes, Some(9), Some(2))));

        s.store_scaled_voltage(197, ctx, nodes, Some(2), Some(0), ((p.p4 * p.p5) * p.p219));

        let assign32180_ad_e49778: A = A::scale_offset(A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p225)), (((-1.0)) * (p.p225))), (-(1.0 - { let limited_exp_arg = ((-((p.p229) as f64).ln()) / p.p228); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })), ((p.p224) * ((1.0 - { let limited_exp_arg = ((-((p.p229) as f64).ln()) / p.p228); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))));
        s.store_ad_value(136, assign32180_ad_e49778);

        s.store_ad_value(90, A::div_scaled_inputs2(s.ad_value(136), 1.0, A::voltage(ctx, nodes, Some(2), Some(0)), (-1.0), s.ad_value(36), 1.0));

        s.store_sqrt_offset_ad(91, A::mul_scaled_lhs(s.ad_value(90), p.p230, s.ad_value(90)), 1.92);

        s.store_scaled_add(137, 90, 91, 0.5);

        s.store_ad_value(106, A::add_scaled_product(s.ad_value(136), 1.0, s.ad_value(36), s.ad_value(137), (-1.0)));

        s.store_ln_ad(192, A::sub_from_scalar(1.0, A::scale(s.ad_value(106), 1.0 / (p.p224))));

        s.store_ad_value(193, A::mul_sub_from_scalar_lhs_scaled_output(p.p224, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p225)), (((-1.0)) * (p.p225))), A::sub_from_scalar(1.0, A::limited_exp_scaled_input(s.ad_value(192), (1.0 - p.p228))), (p.p223 * 1.0 / ((1.0 - p.p228)))));

        s.store_ad_value(194, A::add_scaled_inputs3(s.ad_value(193), (p.p4 * p.p5), A::voltage(ctx, nodes, Some(2), Some(0)), ((p.p229 * p.p223) * (p.p4 * p.p5)), s.ad_value(106), ((-(p.p229 * p.p223)) * (p.p4 * p.p5))));

    }

    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[610] = ((p.p31 == 1.0) && (p.p32 > 0.0));
        s.v[610] = if s.b[610] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.v[192] = 0.0;

        s.v[193] = 0.0;

        s.v[194] = 0.0;

        s.v[195] = 0.0;

        s.v[196] = 0.0;

        s.v[197] = 0.0;

        s.v[186] = 1.0;

        s.v[213] = 0.0;

        s.v[216] = 0.0;

        s.v[94] = 0.0;

        s.v[209] = 0.0;

        s.v[211] = 0.0;

        s.v[212] = 0.0;

        s.v[222] = 0.0;

        s.v[223] = 0.0;

        s.v[224] = 0.0;

        s.v[225] = 0.0;

        s.v[226] = 0.0;

        s.v[227] = 0.0;

        s.v[228] = 0.0;

        s.v[229] = 0.0;

        s.v[230] = 0.0;

        s.v[231] = 0.0;

        s.v[234] = 0.0;

        s.v[235] = 0.0;

        s.v[236] = 0.0;

        s.v[237] = 0.0;

        s.v[238] = 0.0;

        s.v[239] = 0.0;

        s.v[240] = 0.0;

        s.v[241] = 0.0;

        s.v[242] = 0.0;

        s.v[243] = 0.0;

        s.v[246] = 0.0;

        s.v[247] = 0.0;

        s.v[248] = 0.0;

        s.v[249] = 0.0;

        s.v[250] = 0.0;

        s.v[251] = 0.0;

        s.v[252] = 0.0;

        s.v[253] = 0.0;

        s.v[254] = 0.0;

        s.v[255] = 0.0;

        s.v[258] = 0.0;

        s.v[259] = 0.0;

        s.v[260] = 0.0;

        s.v[261] = 0.0;

        s.v[262] = 0.0;

        s.v[263] = 0.0;

        s.v[264] = 0.0;

        s.v[265] = 0.0;

        s.v[266] = 0.0;

        s.v[267] = 0.0;

        s.v[270] = 0.0;

        s.v[271] = 0.0;

        s.v[272] = 0.0;

        s.v[273] = 0.0;

        s.v[274] = 0.0;

        s.v[275] = 0.0;

        s.v[276] = 0.0;

        s.v[277] = 0.0;

        s.v[278] = 0.0;

        s.v[279] = 0.0;

        s.v[282] = 0.0;

        s.v[283] = 0.0;

        s.v[284] = 0.0;

        s.v[285] = 0.0;

        s.v[286] = 0.0;

        s.v[287] = 0.0;

        s.v[288] = 0.0;

        s.v[289] = 0.0;

        s.v[290] = 0.0;

        s.v[291] = 0.0;

        s.v[294] = 0.0;

        s.v[295] = 0.0;

        s.v[296] = 0.0;

        s.v[297] = 0.0;

        s.v[298] = 0.0;

        s.v[299] = 0.0;

        s.v[300] = 0.0;

        s.v[301] = 0.0;

        s.v[302] = 0.0;

        s.v[303] = 0.0;

        s.v[306] = 0.0;

        s.v[307] = 0.0;

        s.v[308] = 0.0;

        s.v[309] = 0.0;

        s.v[310] = 0.0;

        s.v[311] = 0.0;

        s.v[312] = 0.0;

        s.v[313] = 0.0;

        s.v[314] = 0.0;

        s.v[315] = 0.0;

        s.v[182] = 0.01;

        s.v[183] = 0.01;

        s.v[48] = 1.0;

        s.v[56] = 1.0;

        s.v[64] = 1.0;

        s.v[72] = 1.0;

        s.v[52] = 1.0;

        s.v[60] = 1.0;

        s.v[68] = 1.0;

        s.v[76] = 1.0;

        s.v[321] = 0.0;

        s.v[323] = 0.0;

        s.v[326] = 0.0;

        s.v[327] = 0.0;

        s.v[328] = 1.0;

        s.v[329] = 1.0;

        s.v[339] = 0.0;

        s.v[344] = 0.0;

        s.v[345] = 0.0;

        s.v[341] = 0.0;

        s.v[340] = 0.0;

        s.v[346] = 0.0;

        s.v[366] = 0.0;

        s.v[365] = 0.0;

        s.v[361] = p.p34;

        s.b[384] = (p.p149 == 1.0);
        s.v[384] = if s.b[384] { 1.0 } else { 0.0 };

        s.b[385] = (s.v[361] == 0.0);
        s.v[385] = if s.b[385] { 1.0 } else { 0.0 };

        if (s.b[384] && s.b[385]) {
            s.store_scalar(361, 1.0);
        }

        s.v[35] = (p.p0 + 273.15);

        s.store_voltage(42, ctx, nodes, Some(7), Some(8));

        s.store_voltage(43, ctx, nodes, Some(9), Some(8));

        s.store_voltage(44, ctx, nodes, Some(9), Some(7));

        s.store_voltage(46, ctx, nodes, Some(3), Some(8));

        s.store_voltage(47, ctx, nodes, Some(3), Some(7));

        s.v[41] = 1.0;

        s.b[386] = (s.v[42] < 0.0);
        s.v[386] = if s.b[386] { 1.0 } else { 0.0 };

        if s.b[386] {
            s.store_scalar(41, (-1.0));
            s.store_mul(38, 41, 42);
            s.copy_ad(40, 44);
            s.copy_ad(45, 47);
        }

        if (!s.b[386]) {
            s.copy_ad(38, 42);
            s.copy_ad(40, 43);
            s.copy_ad(45, 46);
        }

        s.store_offset_sqrt_ad(140, A::offset(A::square(s.ad_value(38)), 0.01), (-0.1));

        s.store_offset_voltage(82, ctx, nodes, Some(4), None, ((ctx_temp) + (p.p274)));

        s.store_scale(36, 82, 8.617087e-5);

        s.b[387] = (p.p81 == 0.0);
        s.v[387] = if s.b[387] { 1.0 } else { 0.0 };

        s.b[388] = (p.p81 == 1.0);
        s.v[388] = if s.b[388] { 1.0 } else { 0.0 };

        s.b[389] = (p.p81 == 2.0);
        s.v[389] = if s.b[389] { 1.0 } else { 0.0 };

        s.b[390] = (p.p81 == 3.0);
        s.v[390] = if s.b[390] { 1.0 } else { 0.0 };

        s.b[391] = (p.p81 == 4.0);
        s.v[391] = if s.b[391] { 1.0 } else { 0.0 };

        s.b[392] = (p.p81 == 5.0);
        s.v[392] = if s.b[392] { 1.0 } else { 0.0 };

        if (s.b[388] && (!s.b[387])) {
            s.store_voltage(186, ctx, nodes, Some(5), None);
            s.store_ad_value(186, A::add_scaled_inputs3(s.ad_value(186), 0.5, s.ad_value(36), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(186), s.ad_value(36)), A::sub(s.ad_value(186), s.ad_value(36))), ((0.25 * p.p128) * p.p128))), 0.5));
            s.store_offset_scaled_ad(213, A::limited_exp(A::div_from_scalar((-1.0), s.ad_value(186))), p.p101, p.p100);
            s.store_offset_scaled_ad(216, A::limited_exp(A::div_from_scalar((-1.0), s.ad_value(186))), p.p103, p.p102);
        }

        if (s.b[389] && (!(s.b[387] || s.b[388]))) {
            s.store_scaled_voltage(209, ctx, nodes, Some(6), None, p.p113);
            s.store_scaled_voltage(211, ctx, nodes, Some(6), None, p.p114);
            s.store_scaled_voltage(212, ctx, nodes, Some(6), None, p.p115);
        }

        if (s.b[390] && (!((s.b[387] || s.b[388]) || s.b[389]))) {
            s.store_voltage(147, ctx, nodes, Some(0), Some(1));
            s.store_mul_div_from_scalar_ad_lhs(90, p.p124, A::scale_offset(s.ad_value(147), p.p123, 1.0), 147);
            s.store_scaled_offset(91, 147, (-p.p127), p.p125);
            s.store_ad_value(136, A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(1), Some(2)), (-p.p10)), ((-2.0) * 1.0 / (p.p122))));
            s.store_offset_scaled_ad(149, A::div(A::sub_from_scalar(1.0, s.ad_value(136)), A::offset(s.ad_value(136), 1.0)), ((p.p120 - 1e-9) * 0.5), ((((p.p120 - 1e-9) * 0.5)) + (1e-9)));
        }

        if (s.b[391] && (!(((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]))) {
            s.store_abs_voltage(136, ctx, nodes, Some(0), Some(2));
            s.store_abs_voltage(90, ctx, nodes, Some(1), Some(2));
            s.store_sub_voltage_abs_voltage(337, ctx, nodes, Some(12), None, Some(0), Some(2));
            s.store_scaled_add_ad_rhs(337, 337, A::sqrt(A::offset(A::mul(s.ad_value(337), s.ad_value(337)), ((0.25 * 1e-30) * 1e-30))), 0.5);
            s.store_sub_voltage_abs_voltage(342, ctx, nodes, Some(14), None, Some(1), Some(2));
            s.store_scaled_add_ad_rhs(342, 342, A::sqrt(A::offset(A::mul(s.ad_value(342), s.ad_value(342)), ((0.25 * 1e-30) * 1e-30))), 0.5);
            s.store_scale(136, 337, p.p89);
            s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));
            s.store_scaled_div(339, 136, 90, (((p.p91 * p.p10)) as f64).abs());
            s.store_scale(136, 342, p.p90);
            s.store_sqrt_square_offset(90, 342, (p.p90 * p.p90));
            s.store_scaled_div(344, 136, 90, (((p.p92 * p.p10)) as f64).abs());
            s.store_scale(136, 342, p.p90);
            s.store_sqrt_square_offset(90, 342, (p.p90 * p.p90));
            s.store_scaled_div(345, 136, 90, (((p.p93 * p.p13)) as f64).abs());
            s.store_scale(136, 342, p.p90);
            s.store_sqrt_square_offset(90, 342, (p.p90 * p.p90));
            s.store_scaled_div(346, 136, 90, (((p.p94 * p.p17)) as f64).abs());
            s.store_scale(136, 337, p.p89);
            s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));
            s.store_scaled_div(340, 136, 90, (((p.p95 * p.p36)) as f64).abs());
            s.store_scale(136, 337, p.p89);
            s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));
            s.store_scaled_div(341, 136, 90, (((p.p96 * p.p37)) as f64).abs());
        }

        if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
            s.store_voltage(337, ctx, nodes, Some(5), None);
            s.store_voltage(364, ctx, nodes, Some(6), None);
            s.store_scale(136, 337, p.p89);
            s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));
            s.store_scaled_div(339, 136, 90, (((p.p91 * p.p10)) as f64).abs());
            s.store_scale(136, 337, p.p89);
            s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));
            s.store_scaled_div(340, 136, 90, (((p.p95 * p.p36)) as f64).abs());
            s.store_scale(136, 337, p.p89);
            s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));
            s.store_scaled_div(341, 136, 90, (((p.p96 * p.p37)) as f64).abs());
            s.store_scale(136, 364, p.p90);
            s.store_sqrt_square_offset(90, 364, (p.p90 * p.p90));
        }

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
            s.store_scaled_div(344, 136, 90, (((p.p92 * p.p10)) as f64).abs());
            s.store_scale(136, 364, p.p90);
            s.store_sqrt_square_offset(90, 364, (p.p90 * p.p90));
            s.store_scaled_div(365, 136, 90, (((p.p147 * p.p36)) as f64).abs());
            s.store_scale(136, 364, p.p90);
            s.store_sqrt_square_offset(90, 364, (p.p90 * p.p90));
            s.store_scaled_div(366, 136, 90, (((p.p148 * p.p37)) as f64).abs());
        }

        s.v[80] = (p.p9 / p.p1);

        s.v[81] = (p.p9 / p.p2);

        s.store_offset_ad(146, A::mul_offset_lhs(s.ad_value(211), p.p27, s.ad_value(140)), (1.0 + p.p26));

        s.store_scaled_mul(83, 82, 146, 8.617087e-5);

        s.store_ad_value(87, A::add_scaled_inputs3_offset(s.ad_value(339), 1.0, s.ad_value(344), 1.0, A::div_scaled_product(A::sub(A::offset(s.ad_value(212), p.p22), s.ad_value(216)), s.ad_value(140), p.p23, A::sqrt(A::offset(A::square(s.ad_value(140)), (p.p23 * p.p23))), 1.0), -1.0, p.p10));

        s.store_scale(334, 82, 1.0 / (s.v[35]));

        s.store_add_scaled_ad_lhs(88, A::add_scaled_inputs4_offset(s.ad_value(87), 1.0, s.ad_value(334), ((-1.0) * p.p24), s.ad_value(209), 1.0, s.ad_value(213), 1.0, ((-1.0) * ((-1.0) * p.p24))), 45, ((s.v[81] / (s.v[81] + s.v[80])) * p.p11));

        s.store_div_from_scalar_ad(136, p.p3, A::mul_scaled_lhs(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17), s.ad_value(83)));

        s.store_ad_value(159, A::add_scaled_product(s.ad_value(88), 1.0, s.ad_value(83), A::ln_scaled_input(s.ad_value(136), p.p30), 1.0));

        s.store_ad_value(160, A::add_scaled_inputs4(s.ad_value(40), 0.5, s.ad_value(159), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(40), s.ad_value(159)), A::sub(s.ad_value(40), s.ad_value(159))), 0.0001)), 0.5, s.ad_value(159), 1.0));

        s.store_sub(37, 160, 88);

        s.store_div_from_scalar_scaled_input(84, s.v[80], 83, (1.602176634e-19 * 3.24e17));

        s.store_div_from_scalar(150, 2.718281828459045, 84);

        s.store_div_from_scalar(151, 1.0, 84);

        s.v[99] = (s.v[80] / 1.602176634e-19);

        s.store_scaled_add_ad_rhs(154, 37, A::sqrt(A::offset(A::square(s.ad_value(37)), ((4.0 * 0.3) * 0.3))), 0.5);

        s.store_ad_value(155, A::div_scaled_product(s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0));

        s.store_ad_value(130, A::div_scaled_product(s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0));

        let assign2600_ad_e4564: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, A::powf(A::scale(s.ad_value(154), s.v[99]), 0.6666666666666666), (-(p.p28 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::scale(s.ad_value(154), s.v[99]), 0.6666666666666666), ((2.0 * p.p28) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
        s.store_ad_value(152, assign2600_ad_e4564);

        s.store_scaled_div(136, 37, 83, (1.0 / (2.0)));

        s.b[393] = (s.v[136] < 200.0);
        s.v[393] = if s.b[393] { 1.0 } else { 0.0 };

        if s.b[393] {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(153, A::div_scaled_product(s.ad_value(83), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), (2.0 * s.v[99]), A::add_scaled_inputs(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, A::limited_exp(A::div_scaled_inputs(s.ad_value(37), (-1.0), s.ad_value(83), 2.0)), (s.v[99] / 3.24e17)), 1.0));
        }

        if (!s.b[393]) {
            s.store_ad_value(153, A::div_scaled_product(s.ad_value(83), s.ad_value(136), ((2.0 * s.v[99]) * 1.0 / (1.0)), A::add_scaled_inputs(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, A::limited_exp(A::div_scaled_inputs(s.ad_value(37), (-1.0), s.ad_value(83), 2.0)), (s.v[99] / 3.24e17)), 1.0));
        }

        s.store_sub_scaled_inputs(100, 37, 1.0, 153, 1.0 / (s.v[99]));

        s.b[394] = ((((s.v[100] - s.v[37])) as f64).abs() > 1e-19);
        s.v[394] = if s.b[394] { 1.0 } else { 0.0 };

        if s.b[394] {
            s.store_sub(101, 37, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_scalar(136, ((s.v[99]) as f64).powf(0.6666666666666666));
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p28);
            s.store_scaled_mul(103, 136, 90, p.p29);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if s.b[394] {
            let assign2780_ad_e4790: A = {
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
            let assign2780_ad_e4828: A = {
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
            s.store_ad_value(106, A::add_scaled_value_products(s.ad_value(101), s.v[99], s.ad_value(83), assign2780_ad_e4790, (-3.24e17), s.ad_value(83), assign2780_ad_e4828, (-3.24e17)));
        }

        if s.b[394] {
            s.store_scaled_mul(107, 136, 91, p.p28);
            s.store_scaled_mul(108, 136, 91, p.p29);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_sub_ad(113, A::sub_from_scalar(((-1.0) * s.v[99]), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 37, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p28, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p29, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if s.b[394] {
            let assign2940_ad_e5023: A = {
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
            let assign2940_ad_e5061: A = {
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
            s.store_ad_value(120, A::add_scaled_value_products(s.ad_value(115), s.v[99], s.ad_value(83), assign2940_ad_e5023, (-3.24e17), s.ad_value(83), assign2940_ad_e5061, (-3.24e17)));
        }

        if s.b[394] {
            s.store_scaled_mul(121, 136, 137, p.p28);
            s.store_scaled_mul(122, 136, 137, p.p29);
            s.store_scaled_mul_ad(123, A::limited_exp(s.ad_value(118)), A::scale_offset(s.ad_value(121), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_ad(125, A::limited_exp(s.ad_value(119)), A::scale_offset(s.ad_value(122), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_sub_ad(127, A::sub_from_scalar(((-1.0) * s.v[99]), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.copy_ad(129, 128);
        }

        if (!s.b[394]) {
            s.copy_ad(129, 100);
        }

        s.store_sub_from_scalar(347, p.p13, 345);

        s.store_sub_from_scalar(348, p.p17, 346);

        s.store_mul_powf_ad_rhs(97, 347, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20);

        s.store_mul_powf_ad_rhs(89, 348, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19);

        s.store_scaled_abs_ad(136, A::sub(s.ad_value(37), s.ad_value(129)), (s.v[80] / p.p9));

        s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));

        s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));

        s.store_scaled_div(136, 89, 95, 2.0);

        s.store_scaled_add_ad_rhs(90, 37, A::sqrt(A::offset(A::square(s.ad_value(37)), ((4.0 * 0.3) * 0.3))), 0.5);

        s.store_ad_value(85, A::div_scaled_product(s.ad_value(136), s.ad_value(90), p.p3, A::add_scaled_inputs(s.ad_value(136), p.p3, s.ad_value(90), 1.0), 1.0));

        s.store_powf_ad(136, A::div(s.ad_value(38), s.ad_value(85)), p.p18);

        s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));

        s.store_mul(86, 38, 90);

        s.store_sub(39, 37, 86);

        s.copy_ad(130, 39);

        s.store_scaled_add_ad_rhs(131, 130, A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5);

        s.copy_ad(154, 131);

        s.store_ad_value(157, A::div_scaled_product(s.ad_value(154), s.ad_value(150), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))), 1.0));

        s.store_ad_value(158, A::div_scaled_product(s.ad_value(154), s.ad_value(151), 1.0, A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))), 1.0));

        let assign3240_ad_e5339: A = A::div_scaled_inputs3(s.ad_value(154), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, A::powf(A::scale(s.ad_value(154), s.v[99]), 0.6666666666666666), (-(p.p28 / 3.0)), A::add_scaled_offset_product_rhs(A::powf(A::scale(s.ad_value(154), s.v[99]), 0.6666666666666666), ((2.0 * p.p28) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
        s.store_ad_value(152, assign3240_ad_e5339);

        s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));

        s.b[395] = (s.v[136] < 200.0);
        s.v[395] = if s.b[395] { 1.0 } else { 0.0 };

        if s.b[395] {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_ad_value(156, A::div_scaled_product(s.ad_value(83), A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), (2.0 * s.v[99]), A::add_scaled_inputs(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), (s.v[99] / 3.24e17)), 1.0));
        }

        if (!s.b[395]) {
            s.store_ad_value(156, A::div_scaled_product(s.ad_value(83), s.ad_value(136), ((2.0 * s.v[99]) * 1.0 / (1.0)), A::add_scaled_inputs(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, A::limited_exp(A::div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0)), (s.v[99] / 3.24e17)), 1.0));
        }

        s.store_sub_scaled_inputs(100, 130, 1.0, 156, 1.0 / (s.v[99]));

        s.b[396] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[396] = if s.b[396] { 1.0 } else { 0.0 };

        if s.b[396] {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_ad_rhs(101, 101, A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_scalar(136, ((s.v[99]) as f64).powf(0.6666666666666666));
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p28);
            s.store_scaled_mul(103, 136, 90, p.p29);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if s.b[396] {
            let assign3420_ad_e5565: A = {
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
            let assign3420_ad_e5603: A = {
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
            s.store_ad_value(106, A::add_scaled_value_products(s.ad_value(101), s.v[99], s.ad_value(83), assign3420_ad_e5565, (-3.24e17), s.ad_value(83), assign3420_ad_e5603, (-3.24e17)));
        }

        if s.b[396] {
            s.store_scaled_mul(107, 136, 91, p.p28);
            s.store_scaled_mul(108, 136, 91, p.p29);
            s.store_scaled_mul_ad(109, A::limited_exp(s.ad_value(104)), A::scale_offset(s.ad_value(107), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_ad(111, A::limited_exp(s.ad_value(105)), A::scale_offset(s.ad_value(108), 0.6666666666666666, 1.0), 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_sub_ad(113, A::sub_from_scalar(((-1.0) * s.v[99]), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_scaled_add_ad_rhs(115, 115, A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5);
            s.store_mul_scaled_ad_rhs(116, 136, p.p28, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p29, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

    }
}
