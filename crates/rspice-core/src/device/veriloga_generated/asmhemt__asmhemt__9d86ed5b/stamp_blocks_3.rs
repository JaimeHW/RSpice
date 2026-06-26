#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

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
            s.store_sub_ad(88, A::offset(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p188), p.p185), A::div(A::scale(s.ad_value(292), (p.p194 * p.p193)), A::sqrt(A::offset(A::square(s.ad_value(292)), (p.p194 * p.p194)))));
            s.store_scalar(283, (p.p9 / p.p186));
            s.store_div_from_scalar_mul_ad(136, p.p187, A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83));
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p184))));
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(290), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(290), s.ad_value(159)), A::sub(s.ad_value(290), s.ad_value(159))), 0.0001))), 0.5), 159);
            s.store_sub(282, 160, 88);
            s.store_scaled_div(84, 283, 83, (1.0 / ((1.602176634e-19 * 3.24e17))));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 283, 6.241509074460763e18);
            s.store_add_scaled_ad_rhs(154, 282, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(282)), ((4.0 * 0.3) * 0.3))), 0.5));
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if (s.b[508] && s.b[509]) {
            let assign21890_ad_e34320: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p195 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0))));
            s.store_ad_value(152, assign21890_ad_e34320);
        }

        if (s.b[508] && s.b[509]) {
            s.store_scaled_div(136, 282, 83, (1.0 / (2.0)));
        }

        s.b[512] = (s.v[136] < 200.0);
        s.v[512] = if s.b[512] { 1.0 } else { 0.0 };

        if ((s.b[508] && s.b[509]) && s.b[512]) {
            s.store_limited_exp_scaled_input(90, 136, 0.25);
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 0.25));
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(282), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.b[508] && s.b[509]) && (!s.b[512])) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(282), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (s.b[508] && s.b[509]) {
            s.store_sub_ad_rhs(100, 282, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.b[513] = ((((s.v[100] - s.v[282])) as f64).abs() > 1e-19);
        s.v[513] = if s.b[513] { 1.0 } else { 0.0 };

        if ((s.b[508] && s.b[509]) && s.b[513]) {
            s.store_sub(101, 282, 100);
            s.store_add_scaled_ad_rhs(101, 101, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
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
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign22070_ad_e34614)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign22070_ad_e34652));
        }

        if ((s.b[508] && s.b[509]) && s.b[513]) {
            s.store_scaled_mul(107, 136, 91, p.p195);
            s.store_scaled_mul(108, 136, 91, p.p196);
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 282, 114);
            s.store_add_scaled_ad_rhs(115, 115, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p195, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p196, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if ((s.b[508] && s.b[509]) && s.b[513]) {
            let assign22230_ad_e34911: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign22230_ad_e34911)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign22230_ad_e34949));
        }

        if ((s.b[508] && s.b[509]) && s.b[513]) {
            s.store_scaled_mul(121, 136, 137, p.p195);
            s.store_scaled_mul(122, 136, 137, p.p196);
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
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
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
            s.store_scaled_div(136, 89, 95, 2.0);
            s.store_add_scaled_ad_rhs(90, 282, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(282)), ((4.0 * 0.3) * 0.3))), 0.5));
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p187), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p187), s.ad_value(90)));
            s.store_powf_ad(136, A::div(s.ad_value(291), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 291, 90);
            s.store_sub(39, 282, 86);
            s.copy_ad(130, 39);
            s.store_add_scaled_ad_rhs(131, 130, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
            s.copy_ad(154, 131);
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if (s.b[508] && s.b[509]) {
            let assign22510_ad_e35349: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p195 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0))));
            s.store_ad_value(152, assign22510_ad_e35349);
        }

        if (s.b[508] && s.b[509]) {
            s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));
        }

        s.b[514] = (s.v[136] < 200.0);
        s.v[514] = if s.b[514] { 1.0 } else { 0.0 };

        if ((s.b[508] && s.b[509]) && s.b[514]) {
            s.store_limited_exp_scaled_input(90, 136, 0.25);
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 0.25));
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.b[508] && s.b[509]) && (!s.b[514])) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (s.b[508] && s.b[509]) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.b[515] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[515] = if s.b[515] { 1.0 } else { 0.0 };

        if ((s.b[508] && s.b[509]) && s.b[515]) {
            s.store_sub(101, 130, 100);
            s.store_add_scaled_ad_rhs(101, 101, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p195);
            s.store_scaled_mul(103, 136, 90, p.p196);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

    }

    pub(super) fn stamp_reactive_block_19(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[508] && s.b[509]) && s.b[515]) {
            let assign22690_ad_e35643: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign22690_ad_e35643)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign22690_ad_e35681));
        }

        if ((s.b[508] && s.b[509]) && s.b[515]) {
            s.store_scaled_mul(107, 136, 91, p.p195);
            s.store_scaled_mul(108, 136, 91, p.p196);
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_add_scaled_ad_rhs(115, 115, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
            s.store_mul_scaled_ad_rhs(116, 136, p.p195, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p196, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if ((s.b[508] && s.b[509]) && s.b[515]) {
            let assign22840_ad_e35929: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign22840_ad_e35929)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign22840_ad_e35967));
        }

        if ((s.b[508] && s.b[509]) && s.b[515]) {
            s.store_mul_scaled_ad_rhs(121, 136, p.p195, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_scaled_ad_rhs(122, 136, p.p196, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.store_add(285, 128, 86);
        }

        if ((s.b[508] && s.b[509]) && (!s.b[515])) {
            s.store_add(285, 100, 86);
        }

        if (s.b[508] && s.b[509]) {
            s.store_scaled_add(286, 284, 285, 0.5);
            s.store_sub(287, 285, 284);
            s.store_mul_add_ad_lhs(135, A::sub(s.ad_value(282), s.ad_value(286)), s.ad_value(83), 287);
            s.store_mul_scaled_ad_rhs(136, 283, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(282), s.ad_value(286))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::mul(A::scale(s.ad_value(136), p.p15), s.ad_value(136))), A::scale(s.ad_value(90), p.p16)));
            s.store_scaled_mul(96, 95, 283, (p.p4 * (p.p5 * 1.0 / (p.p187))));
            s.store_mul_offset_ad_rhs(98, 96, A::scale(A::sub(s.ad_value(292), s.ad_value(86)), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul(A::scale(s.ad_value(287), (p.p25 * p.p25)), s.ad_value(287)), 1.0);
            s.store_div(93, 98, 92);
            s.store_sub(90, 285, 284);
            s.store_sub_ad_lhs(91, A::add(s.ad_value(282), s.ad_value(83)), 286);
            s.store_mul_scaled_ad_rhs(137, 283, (p.p4 * (p.p5 * p.p187)), A::add(A::sub(s.ad_value(282), s.ad_value(286)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p242)), 1e26);
            s.store_offset_powf_ad(189, s.ad_value(188), p.p241, 1.0);
            s.store_div_from_scalar(190, p.p240, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p186);
            s.store_mul_scaled_ad_rhs(288, 191, (p.p4 * (p.p5 * p.p187)), A::add(A::sub(s.ad_value(282), s.ad_value(286)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
            s.store_sub_ad_lhs(136, A::add(s.ad_value(282), s.ad_value(83)), 286);
            s.store_add_scaled_inputs(90, 284, 0.3333333333333333, 285, ((2.0) * (0.3333333333333333)));
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(287)), (1.0 / 12.0)), 136);
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(287)), s.ad_value(287)), (1.0 / 120.0)), A::square(s.ad_value(136)));
            s.store_mul_ad(289, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p187 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(282), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        s.b[516] = (s.v[68] < 0.0);
        s.v[516] = if s.b[516] { 1.0 } else { 0.0 };

        if ((s.b[508] && s.b[509]) && s.b[516]) {
            s.store_sub_ad_lhs(289, A::scale(s.ad_value(288), (-1.0)), 289);
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
            s.store_offset_scaled_ad(88, A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p188, p.p185);
            s.store_scalar(283, (p.p9 / p.p186));
            s.store_div_from_scalar_mul_ad(136, p.p187, A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83));
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p184))));
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(290), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(290), s.ad_value(159)), A::sub(s.ad_value(290), s.ad_value(159))), 0.0001))), 0.5), 159);
            s.store_sub(282, 160, 88);
            s.store_scaled_div(84, 283, 83, (1.0 / ((1.602176634e-19 * 3.24e17))));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 283, 6.241509074460763e18);
            s.store_add_scaled_ad_rhs(154, 282, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(282)), ((4.0 * 0.3) * 0.3))), 0.5));
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!s.b[508]) && s.b[517]) {
            let assign23430_ad_e36747: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p195 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0))));
            s.store_ad_value(152, assign23430_ad_e36747);
        }

        if ((!s.b[508]) && s.b[517]) {
            s.store_scaled_div(136, 282, 83, (1.0 / (2.0)));
        }

        s.b[519] = (s.v[136] < 200.0);
        s.v[519] = if s.b[519] { 1.0 } else { 0.0 };

        if (((!s.b[508]) && s.b[517]) && s.b[519]) {
            s.store_limited_exp_scaled_input(90, 136, 0.25);
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 0.25));
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(282), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!s.b[508]) && s.b[517]) && (!s.b[519])) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(282), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!s.b[508]) && s.b[517]) {
            s.store_sub_ad_rhs(100, 282, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.b[520] = ((((s.v[100] - s.v[282])) as f64).abs() > 1e-19);
        s.v[520] = if s.b[520] { 1.0 } else { 0.0 };

        if (((!s.b[508]) && s.b[517]) && s.b[520]) {
            s.store_sub(101, 282, 100);
            s.store_add_scaled_ad_rhs(101, 101, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p195);
            s.store_scaled_mul(103, 136, 90, p.p196);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!s.b[508]) && s.b[517]) && s.b[520]) {
            let assign23610_ad_e37057: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign23610_ad_e37057)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign23610_ad_e37095));
        }

        if (((!s.b[508]) && s.b[517]) && s.b[520]) {
            s.store_scaled_mul(107, 136, 91, p.p195);
            s.store_scaled_mul(108, 136, 91, p.p196);
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

    }

    pub(super) fn stamp_reactive_block_20(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[508]) && s.b[517]) && s.b[520]) {
            s.store_sub(115, 282, 114);
            s.store_add_scaled_ad_rhs(115, 115, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p195, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p196, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!s.b[508]) && s.b[517]) && s.b[520]) {
            let assign23770_ad_e37370: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign23770_ad_e37370)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign23770_ad_e37408));
        }

        if (((!s.b[508]) && s.b[517]) && s.b[520]) {
            s.store_scaled_mul(121, 136, 137, p.p195);
            s.store_scaled_mul(122, 136, 137, p.p196);
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
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
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
            s.store_scaled_div(136, 89, 95, 2.0);
            s.store_add_scaled_ad_rhs(90, 282, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(282)), ((4.0 * 0.3) * 0.3))), 0.5));
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p187), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p187), s.ad_value(90)));
            s.store_powf_ad(136, A::div(s.ad_value(291), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 291, 90);
            s.store_sub(39, 282, 86);
            s.copy_ad(130, 39);
            s.store_add_scaled_ad_rhs(131, 130, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
            s.copy_ad(154, 131);
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!s.b[508]) && s.b[517]) {
            let assign24060_ad_e37843: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p195 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0))));
            s.store_ad_value(152, assign24060_ad_e37843);
        }

        if ((!s.b[508]) && s.b[517]) {
            s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));
        }

        s.b[521] = (s.v[136] < 200.0);
        s.v[521] = if s.b[521] { 1.0 } else { 0.0 };

        if (((!s.b[508]) && s.b[517]) && s.b[521]) {
            s.store_limited_exp_scaled_input(90, 136, 0.25);
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 0.25));
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!s.b[508]) && s.b[517]) && (!s.b[521])) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!s.b[508]) && s.b[517]) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.b[522] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[522] = if s.b[522] { 1.0 } else { 0.0 };

        if (((!s.b[508]) && s.b[517]) && s.b[522]) {
            s.store_sub(101, 130, 100);
            s.store_add_scaled_ad_rhs(101, 101, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
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
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign24240_ad_e38153)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign24240_ad_e38191));
        }

        if (((!s.b[508]) && s.b[517]) && s.b[522]) {
            s.store_scaled_mul(107, 136, 91, p.p195);
            s.store_scaled_mul(108, 136, 91, p.p196);
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_add_scaled_ad_rhs(115, 115, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
            s.store_mul_scaled_ad_rhs(116, 136, p.p195, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p196, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!s.b[508]) && s.b[517]) && s.b[522]) {
            let assign24390_ad_e38454: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign24390_ad_e38454)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign24390_ad_e38492));
        }

        if (((!s.b[508]) && s.b[517]) && s.b[522]) {
            s.store_mul_scaled_ad_rhs(121, 136, p.p195, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_scaled_ad_rhs(122, 136, p.p196, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.store_add(285, 128, 86);
        }

        if (((!s.b[508]) && s.b[517]) && (!s.b[522])) {
            s.store_add(285, 100, 86);
        }

        if ((!s.b[508]) && s.b[517]) {
            s.store_scaled_add(286, 284, 285, 0.5);
            s.store_sub(287, 285, 284);
            s.store_sub(90, 285, 284);
            s.store_sub_ad_lhs(91, A::add(s.ad_value(282), s.ad_value(83)), 286);
            s.store_mul_scaled_ad_rhs(137, 283, (p.p4 * (p.p5 * p.p187)), A::add(A::sub(s.ad_value(282), s.ad_value(286)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p242)), 1e26);
            s.store_offset_powf_ad(189, s.ad_value(188), p.p241, 1.0);
            s.store_div_from_scalar(190, p.p240, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p186);
            s.store_mul_scaled_ad_rhs(288, 191, (p.p4 * (p.p5 * p.p187)), A::add(A::sub(s.ad_value(282), s.ad_value(286)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
            s.store_sub_ad_lhs(136, A::add(s.ad_value(282), s.ad_value(83)), 286);
            s.store_add_scaled_inputs(90, 284, 0.3333333333333333, 285, ((2.0) * (0.3333333333333333)));
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(287)), (1.0 / 12.0)), 136);
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(287)), s.ad_value(287)), (1.0 / 120.0)), A::square(s.ad_value(136)));
            s.store_mul_ad(289, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p187 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(282), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
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

    }

    pub(super) fn stamp_reactive_block_21(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
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
            s.store_sub_ad(88, A::sub_from_scalar(p.p198, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p201)), A::div(A::scale(s.ad_value(304), (p.p207 * p.p206)), A::sqrt(A::offset(A::square(s.ad_value(304)), (p.p207 * p.p207)))));
            s.store_scalar(295, (p.p9 / p.p199));
            s.store_div_from_scalar_mul_ad(136, p.p200, A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83));
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p197))));
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(302), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(302), s.ad_value(159)), A::sub(s.ad_value(302), s.ad_value(159))), 0.0001))), 0.5), 159);
            s.store_sub(294, 160, 88);
            s.store_scaled_div(84, 295, 83, (1.0 / ((1.602176634e-19 * 3.24e17))));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 295, 6.241509074460763e18);
            s.store_add_scaled_ad_rhs(154, 294, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(294)), ((4.0 * 0.3) * 0.3))), 0.5));
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if (s.b[523] && s.b[524]) {
            let assign24980_ad_e39249: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p208 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0))));
            s.store_ad_value(152, assign24980_ad_e39249);
        }

        if (s.b[523] && s.b[524]) {
            s.store_scaled_div(136, 294, 83, (1.0 / (2.0)));
        }

        s.b[527] = (s.v[136] < 200.0);
        s.v[527] = if s.b[527] { 1.0 } else { 0.0 };

        if ((s.b[523] && s.b[524]) && s.b[527]) {
            s.store_limited_exp_scaled_input(90, 136, 0.25);
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 0.25));
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(294), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.b[523] && s.b[524]) && (!s.b[527])) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(294), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (s.b[523] && s.b[524]) {
            s.store_sub_ad_rhs(100, 294, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.b[528] = ((((s.v[100] - s.v[294])) as f64).abs() > 1e-19);
        s.v[528] = if s.b[528] { 1.0 } else { 0.0 };

        if ((s.b[523] && s.b[524]) && s.b[528]) {
            s.store_sub(101, 294, 100);
            s.store_add_scaled_ad_rhs(101, 101, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
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
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign25160_ad_e39543)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign25160_ad_e39581));
        }

        if ((s.b[523] && s.b[524]) && s.b[528]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 294, 114);
            s.store_add_scaled_ad_rhs(115, 115, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p208, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p209, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if ((s.b[523] && s.b[524]) && s.b[528]) {
            let assign25320_ad_e39840: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign25320_ad_e39840)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign25320_ad_e39878));
        }

        if ((s.b[523] && s.b[524]) && s.b[528]) {
            s.store_scaled_mul(121, 136, 137, p.p208);
            s.store_scaled_mul(122, 136, 137, p.p209);
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
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
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
            s.store_scaled_div(136, 89, 95, 2.0);
            s.store_add_scaled_ad_rhs(90, 294, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(294)), ((4.0 * 0.3) * 0.3))), 0.5));
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p200), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p200), s.ad_value(90)));
            s.store_powf_ad(136, A::div(s.ad_value(303), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 303, 90);
            s.store_sub(39, 294, 86);
            s.copy_ad(130, 39);
            s.store_add_scaled_ad_rhs(131, 130, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
            s.copy_ad(154, 131);
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if (s.b[523] && s.b[524]) {
            let assign25600_ad_e40278: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p208 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0))));
            s.store_ad_value(152, assign25600_ad_e40278);
        }

        if (s.b[523] && s.b[524]) {
            s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));
        }

        s.b[529] = (s.v[136] < 200.0);
        s.v[529] = if s.b[529] { 1.0 } else { 0.0 };

        if ((s.b[523] && s.b[524]) && s.b[529]) {
            s.store_limited_exp_scaled_input(90, 136, 0.25);
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 0.25));
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.b[523] && s.b[524]) && (!s.b[529])) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (s.b[523] && s.b[524]) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.b[530] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[530] = if s.b[530] { 1.0 } else { 0.0 };

        if ((s.b[523] && s.b[524]) && s.b[530]) {
            s.store_sub(101, 130, 100);
            s.store_add_scaled_ad_rhs(101, 101, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p208);
            s.store_scaled_mul(103, 136, 90, p.p209);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

    }

    pub(super) fn stamp_reactive_block_22(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[523] && s.b[524]) && s.b[530]) {
            let assign25780_ad_e40572: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign25780_ad_e40572)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign25780_ad_e40610));
        }

        if ((s.b[523] && s.b[524]) && s.b[530]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_add_scaled_ad_rhs(115, 115, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
            s.store_mul_scaled_ad_rhs(116, 136, p.p208, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p209, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if ((s.b[523] && s.b[524]) && s.b[530]) {
            let assign25930_ad_e40858: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign25930_ad_e40858)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign25930_ad_e40896));
        }

        if ((s.b[523] && s.b[524]) && s.b[530]) {
            s.store_mul_scaled_ad_rhs(121, 136, p.p208, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_scaled_ad_rhs(122, 136, p.p209, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.store_add(297, 128, 86);
        }

        if ((s.b[523] && s.b[524]) && (!s.b[530])) {
            s.store_add(297, 100, 86);
        }

        if (s.b[523] && s.b[524]) {
            s.store_scaled_add(298, 296, 297, 0.5);
            s.store_sub(299, 297, 296);
            s.store_mul_add_ad_lhs(135, A::sub(s.ad_value(294), s.ad_value(298)), s.ad_value(83), 299);
            s.store_mul_scaled_ad_rhs(136, 295, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(294), s.ad_value(298))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::mul(A::scale(s.ad_value(136), p.p15), s.ad_value(136))), A::scale(s.ad_value(90), p.p16)));
            s.store_scaled_mul(96, 95, 295, (p.p4 * (p.p5 * 1.0 / (p.p200))));
            s.store_mul_offset_ad_rhs(98, 96, A::scale(A::sub(s.ad_value(304), s.ad_value(86)), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul(A::scale(s.ad_value(299), (p.p25 * p.p25)), s.ad_value(299)), 1.0);
            s.store_div(93, 98, 92);
            s.store_sub(90, 297, 296);
            s.store_sub_ad_lhs(91, A::add(s.ad_value(294), s.ad_value(83)), 298);
            s.store_mul_scaled_ad_rhs(137, 295, (p.p4 * (p.p5 * p.p200)), A::add(A::sub(s.ad_value(294), s.ad_value(298)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p245)), 1e26);
            s.store_offset_powf_ad(189, s.ad_value(188), p.p244, 1.0);
            s.store_div_from_scalar(190, p.p243, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p199);
            s.store_mul_scaled_ad_rhs(300, 191, (p.p4 * (p.p5 * p.p200)), A::add(A::sub(s.ad_value(294), s.ad_value(298)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
            s.store_sub_ad_lhs(136, A::add(s.ad_value(294), s.ad_value(83)), 298);
            s.store_add_scaled_inputs(90, 296, 0.3333333333333333, 297, ((2.0) * (0.3333333333333333)));
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(299)), (1.0 / 12.0)), 136);
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(299)), s.ad_value(299)), (1.0 / 120.0)), A::square(s.ad_value(136)));
            s.store_mul_ad(301, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p200 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(294), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        s.b[531] = (s.v[72] < 0.0);
        s.v[531] = if s.b[531] { 1.0 } else { 0.0 };

        if ((s.b[523] && s.b[524]) && s.b[531]) {
            s.store_sub_ad_lhs(301, A::scale(s.ad_value(300), (-1.0)), 301);
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
            s.store_sub_from_scalar_ad(88, p.p198, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p201));
            s.store_scalar(295, (p.p9 / p.p199));
            s.store_div_from_scalar_mul_ad(136, p.p200, A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83));
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p197))));
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(302), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(302), s.ad_value(159)), A::sub(s.ad_value(302), s.ad_value(159))), 0.0001))), 0.5), 159);
            s.store_sub(294, 160, 88);
            s.store_scaled_div(84, 295, 83, (1.0 / ((1.602176634e-19 * 3.24e17))));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 295, 6.241509074460763e18);
            s.store_add_scaled_ad_rhs(154, 294, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(294)), ((4.0 * 0.3) * 0.3))), 0.5));
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!s.b[523]) && s.b[532]) {
            let assign26520_ad_e41676: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p208 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0))));
            s.store_ad_value(152, assign26520_ad_e41676);
        }

        if ((!s.b[523]) && s.b[532]) {
            s.store_scaled_div(136, 294, 83, (1.0 / (2.0)));
        }

        s.b[534] = (s.v[136] < 200.0);
        s.v[534] = if s.b[534] { 1.0 } else { 0.0 };

        if (((!s.b[523]) && s.b[532]) && s.b[534]) {
            s.store_limited_exp_scaled_input(90, 136, 0.25);
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 0.25));
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(294), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!s.b[523]) && s.b[532]) && (!s.b[534])) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(294), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!s.b[523]) && s.b[532]) {
            s.store_sub_ad_rhs(100, 294, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.b[535] = ((((s.v[100] - s.v[294])) as f64).abs() > 1e-19);
        s.v[535] = if s.b[535] { 1.0 } else { 0.0 };

        if (((!s.b[523]) && s.b[532]) && s.b[535]) {
            s.store_sub(101, 294, 100);
            s.store_add_scaled_ad_rhs(101, 101, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p208);
            s.store_scaled_mul(103, 136, 90, p.p209);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!s.b[523]) && s.b[532]) && s.b[535]) {
            let assign26700_ad_e41986: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign26700_ad_e41986)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign26700_ad_e42024));
        }

        if (((!s.b[523]) && s.b[532]) && s.b[535]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

    }

    pub(super) fn stamp_reactive_block_23(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[523]) && s.b[532]) && s.b[535]) {
            s.store_sub(115, 294, 114);
            s.store_add_scaled_ad_rhs(115, 115, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p208, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p209, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!s.b[523]) && s.b[532]) && s.b[535]) {
            let assign26860_ad_e42299: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign26860_ad_e42299)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign26860_ad_e42337));
        }

        if (((!s.b[523]) && s.b[532]) && s.b[535]) {
            s.store_scaled_mul(121, 136, 137, p.p208);
            s.store_scaled_mul(122, 136, 137, p.p209);
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
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
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
            s.store_scaled_div(136, 89, 95, 2.0);
            s.store_add_scaled_ad_rhs(90, 294, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(294)), ((4.0 * 0.3) * 0.3))), 0.5));
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p200), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p200), s.ad_value(90)));
            s.store_powf_ad(136, A::div(s.ad_value(303), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 303, 90);
            s.store_sub(39, 294, 86);
            s.copy_ad(130, 39);
            s.store_add_scaled_ad_rhs(131, 130, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
            s.copy_ad(154, 131);
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!s.b[523]) && s.b[532]) {
            let assign27150_ad_e42772: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p208 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0))));
            s.store_ad_value(152, assign27150_ad_e42772);
        }

        if ((!s.b[523]) && s.b[532]) {
            s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));
        }

        s.b[536] = (s.v[136] < 200.0);
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        if (((!s.b[523]) && s.b[532]) && s.b[536]) {
            s.store_limited_exp_scaled_input(90, 136, 0.25);
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 0.25));
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!s.b[523]) && s.b[532]) && (!s.b[536])) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!s.b[523]) && s.b[532]) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.b[537] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[537] = if s.b[537] { 1.0 } else { 0.0 };

        if (((!s.b[523]) && s.b[532]) && s.b[537]) {
            s.store_sub(101, 130, 100);
            s.store_add_scaled_ad_rhs(101, 101, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
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
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign27330_ad_e43082)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign27330_ad_e43120));
        }

        if (((!s.b[523]) && s.b[532]) && s.b[537]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_add_scaled_ad_rhs(115, 115, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
            s.store_mul_scaled_ad_rhs(116, 136, p.p208, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p209, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!s.b[523]) && s.b[532]) && s.b[537]) {
            let assign27480_ad_e43383: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign27480_ad_e43383)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign27480_ad_e43421));
        }

        if (((!s.b[523]) && s.b[532]) && s.b[537]) {
            s.store_mul_scaled_ad_rhs(121, 136, p.p208, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_scaled_ad_rhs(122, 136, p.p209, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.store_add(297, 128, 86);
        }

        if (((!s.b[523]) && s.b[532]) && (!s.b[537])) {
            s.store_add(297, 100, 86);
        }

        if ((!s.b[523]) && s.b[532]) {
            s.store_scaled_add(298, 296, 297, 0.5);
            s.store_sub(299, 297, 296);
            s.store_sub(90, 297, 296);
            s.store_sub_ad_lhs(91, A::add(s.ad_value(294), s.ad_value(83)), 298);
            s.store_mul_scaled_ad_rhs(137, 295, (p.p4 * (p.p5 * p.p200)), A::add(A::sub(s.ad_value(294), s.ad_value(298)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p245)), 1e26);
            s.store_offset_powf_ad(189, s.ad_value(188), p.p244, 1.0);
            s.store_div_from_scalar(190, p.p243, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p199);
            s.store_mul_scaled_ad_rhs(300, 191, (p.p4 * (p.p5 * p.p200)), A::add(A::sub(s.ad_value(294), s.ad_value(298)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
            s.store_sub_ad_lhs(136, A::add(s.ad_value(294), s.ad_value(83)), 298);
            s.store_add_scaled_inputs(90, 296, 0.3333333333333333, 297, ((2.0) * (0.3333333333333333)));
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(299)), (1.0 / 12.0)), 136);
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(299)), s.ad_value(299)), (1.0 / 120.0)), A::square(s.ad_value(136)));
            s.store_mul_ad(301, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p200 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(294), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
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

    }

    pub(super) fn stamp_reactive_block_24(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
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
            s.store_sub_ad(88, A::offset(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p201), p.p198), A::div(A::scale(s.ad_value(316), (p.p207 * p.p206)), A::sqrt(A::offset(A::square(s.ad_value(316)), (p.p207 * p.p207)))));
            s.store_scalar(307, (p.p9 / p.p199));
            s.store_div_from_scalar_mul_ad(136, p.p200, A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83));
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p197))));
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(314), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(314), s.ad_value(159)), A::sub(s.ad_value(314), s.ad_value(159))), 0.0001))), 0.5), 159);
            s.store_sub(306, 160, 88);
            s.store_scaled_div(84, 307, 83, (1.0 / ((1.602176634e-19 * 3.24e17))));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 307, 6.241509074460763e18);
            s.store_add_scaled_ad_rhs(154, 306, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(306)), ((4.0 * 0.3) * 0.3))), 0.5));
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if (s.b[538] && s.b[539]) {
            let assign28070_ad_e44178: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p208 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0))));
            s.store_ad_value(152, assign28070_ad_e44178);
        }

        if (s.b[538] && s.b[539]) {
            s.store_scaled_div(136, 306, 83, (1.0 / (2.0)));
        }

        s.b[542] = (s.v[136] < 200.0);
        s.v[542] = if s.b[542] { 1.0 } else { 0.0 };

        if ((s.b[538] && s.b[539]) && s.b[542]) {
            s.store_limited_exp_scaled_input(90, 136, 0.25);
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 0.25));
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(306), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.b[538] && s.b[539]) && (!s.b[542])) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(306), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (s.b[538] && s.b[539]) {
            s.store_sub_ad_rhs(100, 306, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.b[543] = ((((s.v[100] - s.v[306])) as f64).abs() > 1e-19);
        s.v[543] = if s.b[543] { 1.0 } else { 0.0 };

        if ((s.b[538] && s.b[539]) && s.b[543]) {
            s.store_sub(101, 306, 100);
            s.store_add_scaled_ad_rhs(101, 101, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
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
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign28250_ad_e44472)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign28250_ad_e44510));
        }

        if ((s.b[538] && s.b[539]) && s.b[543]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 306, 114);
            s.store_add_scaled_ad_rhs(115, 115, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p208, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p209, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if ((s.b[538] && s.b[539]) && s.b[543]) {
            let assign28410_ad_e44769: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign28410_ad_e44769)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign28410_ad_e44807));
        }

        if ((s.b[538] && s.b[539]) && s.b[543]) {
            s.store_scaled_mul(121, 136, 137, p.p208);
            s.store_scaled_mul(122, 136, 137, p.p209);
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
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
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
            s.store_scaled_div(136, 89, 95, 2.0);
            s.store_add_scaled_ad_rhs(90, 306, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(306)), ((4.0 * 0.3) * 0.3))), 0.5));
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p200), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p200), s.ad_value(90)));
            s.store_powf_ad(136, A::div(s.ad_value(315), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 315, 90);
            s.store_sub(39, 306, 86);
            s.copy_ad(130, 39);
            s.store_add_scaled_ad_rhs(131, 130, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
            s.copy_ad(154, 131);
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if (s.b[538] && s.b[539]) {
            let assign28690_ad_e45207: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p208 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0))));
            s.store_ad_value(152, assign28690_ad_e45207);
        }

        if (s.b[538] && s.b[539]) {
            s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));
        }

        s.b[544] = (s.v[136] < 200.0);
        s.v[544] = if s.b[544] { 1.0 } else { 0.0 };

        if ((s.b[538] && s.b[539]) && s.b[544]) {
            s.store_limited_exp_scaled_input(90, 136, 0.25);
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 0.25));
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.b[538] && s.b[539]) && (!s.b[544])) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (s.b[538] && s.b[539]) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.b[545] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[545] = if s.b[545] { 1.0 } else { 0.0 };

        if ((s.b[538] && s.b[539]) && s.b[545]) {
            s.store_sub(101, 130, 100);
            s.store_add_scaled_ad_rhs(101, 101, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p208);
            s.store_scaled_mul(103, 136, 90, p.p209);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

    }

    pub(super) fn stamp_reactive_block_25(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[538] && s.b[539]) && s.b[545]) {
            let assign28870_ad_e45501: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign28870_ad_e45501)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign28870_ad_e45539));
        }

        if ((s.b[538] && s.b[539]) && s.b[545]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_add_scaled_ad_rhs(115, 115, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
            s.store_mul_scaled_ad_rhs(116, 136, p.p208, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p209, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if ((s.b[538] && s.b[539]) && s.b[545]) {
            let assign29020_ad_e45787: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign29020_ad_e45787)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign29020_ad_e45825));
        }

        if ((s.b[538] && s.b[539]) && s.b[545]) {
            s.store_mul_scaled_ad_rhs(121, 136, p.p208, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_scaled_ad_rhs(122, 136, p.p209, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.store_add(309, 128, 86);
        }

        if ((s.b[538] && s.b[539]) && (!s.b[545])) {
            s.store_add(309, 100, 86);
        }

        if (s.b[538] && s.b[539]) {
            s.store_scaled_add(310, 308, 309, 0.5);
            s.store_sub(311, 309, 308);
            s.store_mul_add_ad_lhs(135, A::sub(s.ad_value(306), s.ad_value(310)), s.ad_value(83), 311);
            s.store_mul_scaled_ad_rhs(136, 307, 1.0 / (p.p9), A::abs(A::sub(s.ad_value(306), s.ad_value(310))));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::mul(A::scale(s.ad_value(136), p.p15), s.ad_value(136))), A::scale(s.ad_value(90), p.p16)));
            s.store_scaled_mul(96, 95, 307, (p.p4 * (p.p5 * 1.0 / (p.p200))));
            s.store_mul_offset_ad_rhs(98, 96, A::scale(A::sub(s.ad_value(316), s.ad_value(86)), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul(A::scale(s.ad_value(311), (p.p25 * p.p25)), s.ad_value(311)), 1.0);
            s.store_div(93, 98, 92);
            s.store_sub(90, 309, 308);
            s.store_sub_ad_lhs(91, A::add(s.ad_value(306), s.ad_value(83)), 310);
            s.store_mul_scaled_ad_rhs(137, 307, (p.p4 * (p.p5 * p.p200)), A::add(A::sub(s.ad_value(306), s.ad_value(310)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p245)), 1e26);
            s.store_offset_powf_ad(189, s.ad_value(188), p.p244, 1.0);
            s.store_div_from_scalar(190, p.p243, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p199);
            s.store_mul_scaled_ad_rhs(312, 191, (p.p4 * (p.p5 * p.p200)), A::add(A::sub(s.ad_value(306), s.ad_value(310)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
            s.store_sub_ad_lhs(136, A::add(s.ad_value(306), s.ad_value(83)), 310);
            s.store_add_scaled_inputs(90, 308, 0.3333333333333333, 309, ((2.0) * (0.3333333333333333)));
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(311)), (1.0 / 12.0)), 136);
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(311)), s.ad_value(311)), (1.0 / 120.0)), A::square(s.ad_value(136)));
            s.store_mul_ad(313, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p200 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(306), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        s.b[546] = (s.v[76] < 0.0);
        s.v[546] = if s.b[546] { 1.0 } else { 0.0 };

        if ((s.b[538] && s.b[539]) && s.b[546]) {
            s.store_sub_ad_lhs(313, A::scale(s.ad_value(312), (-1.0)), 313);
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
            s.store_offset_scaled_ad(88, A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p201, p.p198);
            s.store_scalar(307, (p.p9 / p.p199));
            s.store_div_from_scalar_mul_ad(136, p.p200, A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83));
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p197))));
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(314), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(314), s.ad_value(159)), A::sub(s.ad_value(314), s.ad_value(159))), 0.0001))), 0.5), 159);
            s.store_sub(306, 160, 88);
            s.store_scaled_div(84, 307, 83, (1.0 / ((1.602176634e-19 * 3.24e17))));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 307, 6.241509074460763e18);
            s.store_add_scaled_ad_rhs(154, 306, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(306)), ((4.0 * 0.3) * 0.3))), 0.5));
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!s.b[538]) && s.b[547]) {
            let assign29610_ad_e46605: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p208 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0))));
            s.store_ad_value(152, assign29610_ad_e46605);
        }

        if ((!s.b[538]) && s.b[547]) {
            s.store_scaled_div(136, 306, 83, (1.0 / (2.0)));
        }

        s.b[549] = (s.v[136] < 200.0);
        s.v[549] = if s.b[549] { 1.0 } else { 0.0 };

        if (((!s.b[538]) && s.b[547]) && s.b[549]) {
            s.store_limited_exp_scaled_input(90, 136, 0.25);
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 0.25));
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(306), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!s.b[538]) && s.b[547]) && (!s.b[549])) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(306), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!s.b[538]) && s.b[547]) {
            s.store_sub_ad_rhs(100, 306, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.b[550] = ((((s.v[100] - s.v[306])) as f64).abs() > 1e-19);
        s.v[550] = if s.b[550] { 1.0 } else { 0.0 };

        if (((!s.b[538]) && s.b[547]) && s.b[550]) {
            s.store_sub(101, 306, 100);
            s.store_add_scaled_ad_rhs(101, 101, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p208);
            s.store_scaled_mul(103, 136, 90, p.p209);
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!s.b[538]) && s.b[547]) && s.b[550]) {
            let assign29790_ad_e46915: A = {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign29790_ad_e46915)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign29790_ad_e46953));
        }

        if (((!s.b[538]) && s.b[547]) && s.b[550]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

    }

    pub(super) fn stamp_reactive_block_26(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[538]) && s.b[547]) && s.b[550]) {
            s.store_sub(115, 306, 114);
            s.store_add_scaled_ad_rhs(115, 115, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_ad_rhs(116, 136, p.p208, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p209, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!s.b[538]) && s.b[547]) && s.b[550]) {
            let assign29950_ad_e47228: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign29950_ad_e47228)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign29950_ad_e47266));
        }

        if (((!s.b[538]) && s.b[547]) && s.b[550]) {
            s.store_scaled_mul(121, 136, 137, p.p208);
            s.store_scaled_mul(122, 136, 137, p.p209);
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
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
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
            s.store_scaled_div(136, 89, 95, 2.0);
            s.store_add_scaled_ad_rhs(90, 306, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(306)), ((4.0 * 0.3) * 0.3))), 0.5));
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p200), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p200), s.ad_value(90)));
            s.store_powf_ad(136, A::div(s.ad_value(315), s.ad_value(85)), p.p18);
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
            s.store_mul(86, 315, 90);
            s.store_sub(39, 306, 86);
            s.copy_ad(130, 39);
            s.store_add_scaled_ad_rhs(131, 130, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
            s.copy_ad(154, 131);
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!s.b[538]) && s.b[547]) {
            let assign30240_ad_e47701: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p208 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0))));
            s.store_ad_value(152, assign30240_ad_e47701);
        }

        if ((!s.b[538]) && s.b[547]) {
            s.store_scaled_div(136, 130, 83, (1.0 / (2.0)));
        }

        s.b[551] = (s.v[136] < 200.0);
        s.v[551] = if s.b[551] { 1.0 } else { 0.0 };

        if (((!s.b[538]) && s.b[547]) && s.b[551]) {
            s.store_limited_exp_scaled_input(90, 136, 0.25);
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 0.25));
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!s.b[538]) && s.b[547]) && (!s.b[551])) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!s.b[538]) && s.b[547]) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.b[552] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.v[552] = if s.b[552] { 1.0 } else { 0.0 };

        if (((!s.b[538]) && s.b[547]) && s.b[552]) {
            s.store_sub(101, 130, 100);
            s.store_add_scaled_ad_rhs(101, 101, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
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
                    A::ln(A::offset(A::exp(s.ad_value(104)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(105)), 1.0))
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign30420_ad_e48011)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign30420_ad_e48049));
        }

        if (((!s.b[538]) && s.b[547]) && s.b[552]) {
            s.store_scaled_mul(107, 136, 91, p.p208);
            s.store_scaled_mul(108, 136, 91, p.p209);
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
            s.store_sub(115, 130, 114);
            s.store_add_scaled_ad_rhs(115, 115, 0.5, A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
            s.store_mul_scaled_ad_rhs(116, 136, p.p208, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_mul_scaled_ad_rhs(117, 136, p.p209, A::powf(s.ad_value(115), 0.6666666666666666));
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!s.b[538]) && s.b[547]) && s.b[552]) {
            let assign30570_ad_e48312: A = {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(118)), 1.0))
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
                    A::ln(A::offset(A::exp(s.ad_value(119)), 1.0))
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign30570_ad_e48312)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign30570_ad_e48350));
        }

        if (((!s.b[538]) && s.b[547]) && s.b[552]) {
            s.store_mul_scaled_ad_rhs(121, 136, p.p208, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_scaled_ad_rhs(122, 136, p.p209, A::powf(s.ad_value(115), (-0.3333333333333333)));
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
            s.store_add(309, 128, 86);
        }

        if (((!s.b[538]) && s.b[547]) && (!s.b[552])) {
            s.store_add(309, 100, 86);
        }

        if ((!s.b[538]) && s.b[547]) {
            s.store_scaled_add(310, 308, 309, 0.5);
            s.store_sub(311, 309, 308);
            s.store_sub(90, 309, 308);
            s.store_sub_ad_lhs(91, A::add(s.ad_value(306), s.ad_value(83)), 310);
            s.store_mul_scaled_ad_rhs(137, 307, (p.p4 * (p.p5 * p.p200)), A::add(A::sub(s.ad_value(306), s.ad_value(310)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p245)), 1e26);
            s.store_offset_powf_ad(189, s.ad_value(188), p.p244, 1.0);
            s.store_div_from_scalar(190, p.p243, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p199);
            s.store_mul_scaled_ad_rhs(312, 191, (p.p4 * (p.p5 * p.p200)), A::add(A::sub(s.ad_value(306), s.ad_value(310)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
            s.store_sub_ad_lhs(136, A::add(s.ad_value(306), s.ad_value(83)), 310);
            s.store_add_scaled_inputs(90, 308, 0.3333333333333333, 309, ((2.0) * (0.3333333333333333)));
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(311)), (1.0 / 12.0)), 136);
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(311)), s.ad_value(311)), (1.0 / 120.0)), A::square(s.ad_value(136)));
            s.store_mul_ad(313, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p200 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(306), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        if ((!s.b[538]) && (!s.b[547])) {
            s.store_scalar(312, 0.0);
            s.store_scalar(313, 0.0);
        }

        s.b[558] = (p.p255 == 2.0);
        s.v[558] = if s.b[558] { 1.0 } else { 0.0 };

        if s.b[558] {
            s.store_scaled_voltage(162, ctx, nodes, Some(10), Some(2), ((p.p4 * p.p5) * p.p210));
            s.store_div_ad(168, A::scale(A::voltage(ctx, nodes, Some(0), Some(2)), p.p214), A::sqrt(A::offset(A::square(A::voltage(ctx, nodes, Some(0), Some(2))), (p.p214 * p.p214))));
        }

    }

    pub(super) fn stamp_reactive_block_27(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[558] {
            s.store_scalar(169, (p.p213).min((p.p211 / (2.0 * p.p214))));
            s.store_sub_from_scalar_ad(167, ((p.p4 * p.p5) * p.p211), A::mul(A::scale(s.ad_value(169), (p.p4 * p.p5)), s.ad_value(168)));
            s.store_mul_voltage_ad(163, A::max_with_scalar(s.ad_value(167), 0.0), ctx, nodes, Some(10), Some(0));
        }

        if (!s.b[558]) {
            s.store_scaled_voltage(162, ctx, nodes, Some(1), Some(2), ((p.p4 * p.p5) * p.p210));
            s.store_div_ad(168, A::scale(A::voltage(ctx, nodes, Some(0), Some(2)), p.p214), A::sqrt(A::offset(A::square(A::voltage(ctx, nodes, Some(0), Some(2))), (p.p214 * p.p214))));
            s.store_scalar(169, (p.p213).min((p.p211 / (2.0 * p.p214))));
            s.store_sub_from_scalar_ad(167, ((p.p4 * p.p5) * p.p211), A::mul(A::scale(s.ad_value(169), (p.p4 * p.p5)), s.ad_value(168)));
            s.store_mul_voltage_ad(163, A::max_with_scalar(s.ad_value(167), 0.0), ctx, nodes, Some(1), Some(0));
        }

        s.store_scaled_voltage(164, ctx, nodes, Some(0), Some(2), ((p.p4 * p.p5) * p.p212));

        s.store_scaled_voltage(219, ctx, nodes, Some(3), Some(0), ((p.p4 * p.p5) * p.p215));

        s.store_scaled_voltage(220, ctx, nodes, Some(3), Some(2), ((p.p4 * p.p5) * p.p216));

        s.store_scaled_voltage(221, ctx, nodes, Some(3), Some(1), ((p.p4 * p.p5) * p.p217));

        s.store_scale_ad(377, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p281)), p.p277);

        s.store_scale_ad(378, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p282)), p.p278);

        s.store_scale(137, 378, (p.p4 * p.p5));

        s.store_scale(137, 377, (p.p4 * p.p5));

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

        s.store_sub_from_scalar_ad(195, p.p222, A::mul(A::offset(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p227), p.p220), A::voltage(ctx, nodes, Some(0), Some(2))));

        s.store_scaled_sub_ad(195, A::offset(s.ad_value(195), 1e-25), A::scale(A::sub(A::offset(s.ad_value(195), 1e-25), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(195), (-1e-25)), A::offset(s.ad_value(195), (-1e-25))), p.p221))), 0.5), (p.p4 * p.p5));

        let assign32150_ad_e49745: A = A::add(A::offset(A::sub_from_scalar(p.p218, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p226)), 1e-18), A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p218, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p226)), (-1e-18)), A::offset(A::sub_from_scalar(p.p218, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p226)), (-1e-18))), ((0.25 * 1e-19) * 1e-19))));
        s.store_scale_ad(136, assign32150_ad_e49745, 0.5);

        s.store_mul_voltage_ad(196, A::scale(s.ad_value(136), (p.p4 * p.p5)), ctx, nodes, Some(9), Some(2));

        s.store_scaled_voltage(197, ctx, nodes, Some(2), Some(0), ((p.p4 * p.p5) * p.p219));

        s.store_scaled_sub_from_scalar_ad(136, p.p224, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p225), (1.0 - { let limited_exp_arg = ((-((p.p229) as f64).ln()) / p.p228); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }));

        s.store_div_ad_lhs(90, A::sub(s.ad_value(136), A::voltage(ctx, nodes, Some(2), Some(0))), 36);

        s.store_sqrt_offset_ad(91, A::mul(A::scale(s.ad_value(90), p.p230), s.ad_value(90)), 1.92);

        s.store_scaled_add(137, 90, 91, 0.5);

        s.store_sub_ad_rhs(106, 136, A::mul(s.ad_value(36), s.ad_value(137)));

        s.store_ln_ad(192, A::sub_from_scalar(1.0, A::scale(s.ad_value(106), 1.0 / (p.p224))));

        s.store_scaled_mul_ad(193, A::scale(A::sub_from_scalar(p.p224, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p225)), p.p223), A::sub_from_scalar(1.0, A::limited_exp(A::scale(s.ad_value(192), (1.0 - p.p228)))), 1.0 / ((1.0 - p.p228)));

        s.store_scaled_add_ad_rhs(194, 193, A::scale(A::sub(A::voltage(ctx, nodes, Some(2), Some(0)), s.ad_value(106)), (p.p229 * p.p223)), (p.p4 * p.p5));

        s.b[610] = ((p.p31 == 1.0) && (p.p32 > 0.0));
        s.v[610] = if s.b[610] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq0_e298,) = {
    if (s.b[382] && s.b[383]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq0_value: f64 = eq0_e298;
        stamper.stamp_potential_const(
            branches[0],
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
        stamper.stamp_potential_const(
            branches[1],
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
        stamper.stamp_potential_const(
            branches[2],
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
        stamper.stamp_potential_const(
            branches[3],
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
        stamper.stamp_potential_const(
            branches[4],
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
        stamper.stamp_potential_const(
            branches[5],
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
        stamper.stamp_potential_const(
            branches[6],
            eq6_value,
        );
        let (eq7_e331, eq7_e331_d_n0, eq7_e331_d_n1, eq7_e331_d_n2, eq7_e331_d_n3, eq7_e331_d_n4, eq7_e331_d_n5, eq7_e331_d_n6, eq7_e331_d_n7, eq7_e331_d_n8, eq7_e331_d_n9, eq7_e331_d_n10, eq7_e331_d_n11, eq7_e331_d_n12, eq7_e331_d_n13, eq7_e331_d_n14, eq7_e331_d_n15, eq7_e331_d_n16, eq7_e331_d_n17, eq7_e331_d_n18, eq7_e331_d_n19, eq7_e331_d_n20, eq7_e331_d_n21, eq7_e331_d_n22,) = {
    if (s.b[388] && (!s.b[387])) {
        let eq7_e329: f64 = (s.v[38] * s.v[38]);
        let eq7_e329_d_n0: f64 = ((s.dn[38][0] * s.v[38]) + (s.v[38] * s.dn[38][0]));
        let eq7_e329_d_n1: f64 = ((s.dn[38][1] * s.v[38]) + (s.v[38] * s.dn[38][1]));
        let eq7_e329_d_n2: f64 = ((s.dn[38][2] * s.v[38]) + (s.v[38] * s.dn[38][2]));
        let eq7_e329_d_n3: f64 = ((s.dn[38][3] * s.v[38]) + (s.v[38] * s.dn[38][3]));
        let eq7_e329_d_n4: f64 = ((s.dn[38][4] * s.v[38]) + (s.v[38] * s.dn[38][4]));
        let eq7_e329_d_n5: f64 = ((s.dn[38][5] * s.v[38]) + (s.v[38] * s.dn[38][5]));
        let eq7_e329_d_n6: f64 = ((s.dn[38][6] * s.v[38]) + (s.v[38] * s.dn[38][6]));
        let eq7_e329_d_n7: f64 = ((s.dn[38][7] * s.v[38]) + (s.v[38] * s.dn[38][7]));
        let eq7_e329_d_n8: f64 = ((s.dn[38][8] * s.v[38]) + (s.v[38] * s.dn[38][8]));
        let eq7_e329_d_n9: f64 = ((s.dn[38][9] * s.v[38]) + (s.v[38] * s.dn[38][9]));
        let eq7_e329_d_n10: f64 = ((s.dn[38][10] * s.v[38]) + (s.v[38] * s.dn[38][10]));
        let eq7_e329_d_n11: f64 = ((s.dn[38][11] * s.v[38]) + (s.v[38] * s.dn[38][11]));
        let eq7_e329_d_n12: f64 = ((s.dn[38][12] * s.v[38]) + (s.v[38] * s.dn[38][12]));
        let eq7_e329_d_n13: f64 = ((s.dn[38][13] * s.v[38]) + (s.v[38] * s.dn[38][13]));
        let eq7_e329_d_n14: f64 = ((s.dn[38][14] * s.v[38]) + (s.v[38] * s.dn[38][14]));
        let eq7_e329_d_n15: f64 = ((s.dn[38][15] * s.v[38]) + (s.v[38] * s.dn[38][15]));
        let eq7_e329_d_n16: f64 = ((s.dn[38][16] * s.v[38]) + (s.v[38] * s.dn[38][16]));
        let eq7_e329_d_n17: f64 = ((s.dn[38][17] * s.v[38]) + (s.v[38] * s.dn[38][17]));
        let eq7_e329_d_n18: f64 = ((s.dn[38][18] * s.v[38]) + (s.v[38] * s.dn[38][18]));
        let eq7_e329_d_n19: f64 = ((s.dn[38][19] * s.v[38]) + (s.v[38] * s.dn[38][19]));
        let eq7_e329_d_n20: f64 = ((s.dn[38][20] * s.v[38]) + (s.v[38] * s.dn[38][20]));
        let eq7_e329_d_n21: f64 = ((s.dn[38][21] * s.v[38]) + (s.v[38] * s.dn[38][21]));
        let eq7_e329_d_n22: f64 = ((s.dn[38][22] * s.v[38]) + (s.v[38] * s.dn[38][22]));
        (eq7_e329, eq7_e329_d_n0, eq7_e329_d_n1, eq7_e329_d_n2, eq7_e329_d_n3, eq7_e329_d_n4, eq7_e329_d_n5, eq7_e329_d_n6, eq7_e329_d_n7, eq7_e329_d_n8, eq7_e329_d_n9, eq7_e329_d_n10, eq7_e329_d_n11, eq7_e329_d_n12, eq7_e329_d_n13, eq7_e329_d_n14, eq7_e329_d_n15, eq7_e329_d_n16, eq7_e329_d_n17, eq7_e329_d_n18, eq7_e329_d_n19, eq7_e329_d_n20, eq7_e329_d_n21, eq7_e329_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e331;
        let eq7_node_derivatives: [f64; 23] = [eq7_e331_d_n0, eq7_e331_d_n1, eq7_e331_d_n2, eq7_e331_d_n3, eq7_e331_d_n4, eq7_e331_d_n5, eq7_e331_d_n6, eq7_e331_d_n7, eq7_e331_d_n8, eq7_e331_d_n9, eq7_e331_d_n10, eq7_e331_d_n11, eq7_e331_d_n12, eq7_e331_d_n13, eq7_e331_d_n14, eq7_e331_d_n15, eq7_e331_d_n16, eq7_e331_d_n17, eq7_e331_d_n18, eq7_e331_d_n19, eq7_e331_d_n20, eq7_e331_d_n21, eq7_e331_d_n22];
        let eq7_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_dense(
            branches[7],
            eq7_value,
            nodes,
            &eq7_node_derivatives,
            branches,
            &eq7_branch_derivatives,
        );
        let (eq8_e345, eq8_e345_d_n5, eq8_e345_d_n6,) = {
    if (s.b[388] && (!s.b[387])) {
        let eq8_e339: f64 = ((nv6 - nv5) / 10.0);
        let eq8_e339_d_n5: f64 = (-1.0 / 10.0);
        let eq8_e339_d_n6: f64 = (1.0 / 10.0);
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
        stamper.stamp_current_node2(
            Some(nodes[6]),
            Some(nodes[5]),
            multiplicity * (eq8_value),
            nodes[5],
            multiplicity * (eq8_e345_d_n5),
            nodes[6],
            multiplicity * (eq8_e345_d_n6),
        );
        let (eq9_e355, eq9_e355_d_n5,) = {
    if (s.b[388] && (!s.b[387])) {
        let eq9_e352: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, (nv5 - 0.0));
        let eq9_e353: f64 = (p.p97 * eq9_e352);
        let eq9_e353_d_n5: f64 = (p.p97 * ddt_scale);
        (eq9_e353, eq9_e353_d_n5,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e355;
        stamper.stamp_current_node1(
            Some(nodes[5]),
            None,
            multiplicity * (eq9_value),
            nodes[5],
            multiplicity * (eq9_e355_d_n5),
        );
        let (eq10_e364, eq10_e364_d_n5,) = {
    if (s.b[388] && (!s.b[387])) {
        let eq10_e362: f64 = ((nv5 - 0.0) / p.p98);
        let eq10_e362_d_n5: f64 = (1.0 / p.p98);
        (eq10_e362, eq10_e362_d_n5,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e364;
        stamper.stamp_current_node1(
            Some(nodes[5]),
            None,
            multiplicity * (eq10_value),
            nodes[5],
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
        stamper.stamp_potential_const(
            branches[8],
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
        stamper.stamp_potential_const(
            branches[9],
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
        stamper.stamp_potential_const(
            branches[10],
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
        stamper.stamp_potential_const(
            branches[11],
            eq14_value,
        );
        let (eq15_e403, eq15_e403_d_n5,) = {
    if (s.b[389] && (!(s.b[387] || s.b[388]))) {
        let eq15_e401: f64 = ((nv5 - 0.0) / p.p108);
        let eq15_e401_d_n5: f64 = (1.0 / p.p108);
        (eq15_e401, eq15_e401_d_n5,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e403;
        stamper.stamp_current_node1(
            Some(nodes[5]),
            None,
            multiplicity * (eq15_value),
            nodes[5],
            multiplicity * (eq15_e403_d_n5),
        );
        let (eq16_e415, eq16_e415_d_n0, eq16_e415_d_n1, eq16_e415_d_n2, eq16_e415_d_n3, eq16_e415_d_n4, eq16_e415_d_n5, eq16_e415_d_n6, eq16_e415_d_n7, eq16_e415_d_n8, eq16_e415_d_n9, eq16_e415_d_n10, eq16_e415_d_n11, eq16_e415_d_n12, eq16_e415_d_n13, eq16_e415_d_n14, eq16_e415_d_n15, eq16_e415_d_n16, eq16_e415_d_n17, eq16_e415_d_n18, eq16_e415_d_n19, eq16_e415_d_n20, eq16_e415_d_n21, eq16_e415_d_n22,) = {
    if (s.b[389] && (!(s.b[387] || s.b[388]))) {
        let eq16_e411: f64 = (-1.0);
        let eq16_e413: f64 = (eq16_e411 * s.v[208]);
        let eq16_e413_d_n0: f64 = (eq16_e411 * s.dn[208][0]);
        let eq16_e413_d_n1: f64 = (eq16_e411 * s.dn[208][1]);
        let eq16_e413_d_n2: f64 = (eq16_e411 * s.dn[208][2]);
        let eq16_e413_d_n3: f64 = (eq16_e411 * s.dn[208][3]);
        let eq16_e413_d_n4: f64 = (eq16_e411 * s.dn[208][4]);
        let eq16_e413_d_n5: f64 = (eq16_e411 * s.dn[208][5]);
        let eq16_e413_d_n6: f64 = (eq16_e411 * s.dn[208][6]);
        let eq16_e413_d_n7: f64 = (eq16_e411 * s.dn[208][7]);
        let eq16_e413_d_n8: f64 = (eq16_e411 * s.dn[208][8]);
        let eq16_e413_d_n9: f64 = (eq16_e411 * s.dn[208][9]);
        let eq16_e413_d_n10: f64 = (eq16_e411 * s.dn[208][10]);
        let eq16_e413_d_n11: f64 = (eq16_e411 * s.dn[208][11]);
        let eq16_e413_d_n12: f64 = (eq16_e411 * s.dn[208][12]);
        let eq16_e413_d_n13: f64 = (eq16_e411 * s.dn[208][13]);
        let eq16_e413_d_n14: f64 = (eq16_e411 * s.dn[208][14]);
        let eq16_e413_d_n15: f64 = (eq16_e411 * s.dn[208][15]);
        let eq16_e413_d_n16: f64 = (eq16_e411 * s.dn[208][16]);
        let eq16_e413_d_n17: f64 = (eq16_e411 * s.dn[208][17]);
        let eq16_e413_d_n18: f64 = (eq16_e411 * s.dn[208][18]);
        let eq16_e413_d_n19: f64 = (eq16_e411 * s.dn[208][19]);
        let eq16_e413_d_n20: f64 = (eq16_e411 * s.dn[208][20]);
        let eq16_e413_d_n21: f64 = (eq16_e411 * s.dn[208][21]);
        let eq16_e413_d_n22: f64 = (eq16_e411 * s.dn[208][22]);
        (eq16_e413, eq16_e413_d_n0, eq16_e413_d_n1, eq16_e413_d_n2, eq16_e413_d_n3, eq16_e413_d_n4, eq16_e413_d_n5, eq16_e413_d_n6, eq16_e413_d_n7, eq16_e413_d_n8, eq16_e413_d_n9, eq16_e413_d_n10, eq16_e413_d_n11, eq16_e413_d_n12, eq16_e413_d_n13, eq16_e413_d_n14, eq16_e413_d_n15, eq16_e413_d_n16, eq16_e413_d_n17, eq16_e413_d_n18, eq16_e413_d_n19, eq16_e413_d_n20, eq16_e413_d_n21, eq16_e413_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e415;
        let eq16_node_derivatives: [f64; 23] = [eq16_e415_d_n0, eq16_e415_d_n1, eq16_e415_d_n2, eq16_e415_d_n3, eq16_e415_d_n4, eq16_e415_d_n5, eq16_e415_d_n6, eq16_e415_d_n7, eq16_e415_d_n8, eq16_e415_d_n9, eq16_e415_d_n10, eq16_e415_d_n11, eq16_e415_d_n12, eq16_e415_d_n13, eq16_e415_d_n14, eq16_e415_d_n15, eq16_e415_d_n16, eq16_e415_d_n17, eq16_e415_d_n18, eq16_e415_d_n19, eq16_e415_d_n20, eq16_e415_d_n21, eq16_e415_d_n22];
        let eq16_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            None,
            multiplicity * (eq16_value),
            nodes,
            &eq16_node_derivatives,
            branches,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let (eq17_e427, eq17_e427_d_n5,) = {
    if (s.b[389] && (!(s.b[387] || s.b[388]))) {
        let eq17_e424: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, (nv5 - 0.0));
        let eq17_e425: f64 = (p.p110 * eq17_e424);
        let eq17_e425_d_n5: f64 = (p.p110 * ddt_scale);
        (eq17_e425, eq17_e425_d_n5,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e427;
        stamper.stamp_current_node1(
            Some(nodes[5]),
            None,
            multiplicity * (eq17_value),
            nodes[5],
            multiplicity * (eq17_e427_d_n5),
        );
        let (eq18_e438, eq18_e438_d_n6,) = {
    if (s.b[389] && (!(s.b[387] || s.b[388]))) {
        let eq18_e436: f64 = ((nv6 - 0.0) / p.p109);
        let eq18_e436_d_n6: f64 = (1.0 / p.p109);
        (eq18_e436, eq18_e436_d_n6,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e438;
        stamper.stamp_current_node1(
            Some(nodes[6]),
            None,
            multiplicity * (eq18_value),
            nodes[6],
            multiplicity * (eq18_e438_d_n6),
        );
        let (eq19_e450, eq19_e450_d_n0, eq19_e450_d_n2,) = {
    if (s.b[389] && (!(s.b[387] || s.b[388]))) {
        let eq19_e446: f64 = (-1.0);
        let eq19_e448: f64 = (eq19_e446 * (nv0 - nv2));
        let eq19_e448_d_n2: f64 = (-eq19_e446);
        (eq19_e448, eq19_e446, eq19_e448_d_n2,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e450;
        stamper.stamp_current_node2(
            Some(nodes[6]),
            None,
            multiplicity * (eq19_value),
            nodes[0],
            multiplicity * (eq19_e450_d_n0),
            nodes[2],
            multiplicity * (eq19_e450_d_n2),
        );
        let (eq20_e462, eq20_e462_d_n6,) = {
    if (s.b[389] && (!(s.b[387] || s.b[388]))) {
        let eq20_e459: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, (nv6 - 0.0));
        let eq20_e460: f64 = (p.p111 * eq20_e459);
        let eq20_e460_d_n6: f64 = (p.p111 * ddt_scale);
        (eq20_e460, eq20_e460_d_n6,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e462;
        stamper.stamp_current_node1(
            Some(nodes[6]),
            None,
            multiplicity * (eq20_value),
            nodes[6],
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
        stamper.stamp_potential_const(
            branches[12],
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
        stamper.stamp_potential_const(
            branches[13],
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
        stamper.stamp_potential_const(
            branches[14],
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
        stamper.stamp_potential_const(
            branches[15],
            eq24_value,
        );
        let (eq25_e511, eq25_e511_d_n5,) = {
    if (s.b[390] && (!((s.b[387] || s.b[388]) || s.b[389]))) {
        let eq25_e509: f64 = ((nv5 - 0.0) / p.p119);
        let eq25_e509_d_n5: f64 = (1.0 / p.p119);
        (eq25_e509, eq25_e509_d_n5,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e511;
        stamper.stamp_current_node1(
            Some(nodes[5]),
            None,
            multiplicity * (eq25_value),
            nodes[5],
            multiplicity * (eq25_e511_d_n5),
        );
        let (eq26_e525, eq26_e525_d_n0, eq26_e525_d_n1, eq26_e525_d_n2, eq26_e525_d_n3, eq26_e525_d_n4, eq26_e525_d_n5, eq26_e525_d_n6, eq26_e525_d_n7, eq26_e525_d_n8, eq26_e525_d_n9, eq26_e525_d_n10, eq26_e525_d_n11, eq26_e525_d_n12, eq26_e525_d_n13, eq26_e525_d_n14, eq26_e525_d_n15, eq26_e525_d_n16, eq26_e525_d_n17, eq26_e525_d_n18, eq26_e525_d_n19, eq26_e525_d_n20, eq26_e525_d_n21, eq26_e525_d_n22,) = {
    if (s.b[390] && (!((s.b[387] || s.b[388]) || s.b[389]))) {
        let eq26_e521: f64 = (-1.0);
        let eq26_e523: f64 = (eq26_e521 * s.v[148]);
        let eq26_e523_d_n0: f64 = (eq26_e521 * s.dn[148][0]);
        let eq26_e523_d_n1: f64 = (eq26_e521 * s.dn[148][1]);
        let eq26_e523_d_n2: f64 = (eq26_e521 * s.dn[148][2]);
        let eq26_e523_d_n3: f64 = (eq26_e521 * s.dn[148][3]);
        let eq26_e523_d_n4: f64 = (eq26_e521 * s.dn[148][4]);
        let eq26_e523_d_n5: f64 = (eq26_e521 * s.dn[148][5]);
        let eq26_e523_d_n6: f64 = (eq26_e521 * s.dn[148][6]);
        let eq26_e523_d_n7: f64 = (eq26_e521 * s.dn[148][7]);
        let eq26_e523_d_n8: f64 = (eq26_e521 * s.dn[148][8]);
        let eq26_e523_d_n9: f64 = (eq26_e521 * s.dn[148][9]);
        let eq26_e523_d_n10: f64 = (eq26_e521 * s.dn[148][10]);
        let eq26_e523_d_n11: f64 = (eq26_e521 * s.dn[148][11]);
        let eq26_e523_d_n12: f64 = (eq26_e521 * s.dn[148][12]);
        let eq26_e523_d_n13: f64 = (eq26_e521 * s.dn[148][13]);
        let eq26_e523_d_n14: f64 = (eq26_e521 * s.dn[148][14]);
        let eq26_e523_d_n15: f64 = (eq26_e521 * s.dn[148][15]);
        let eq26_e523_d_n16: f64 = (eq26_e521 * s.dn[148][16]);
        let eq26_e523_d_n17: f64 = (eq26_e521 * s.dn[148][17]);
        let eq26_e523_d_n18: f64 = (eq26_e521 * s.dn[148][18]);
        let eq26_e523_d_n19: f64 = (eq26_e521 * s.dn[148][19]);
        let eq26_e523_d_n20: f64 = (eq26_e521 * s.dn[148][20]);
        let eq26_e523_d_n21: f64 = (eq26_e521 * s.dn[148][21]);
        let eq26_e523_d_n22: f64 = (eq26_e521 * s.dn[148][22]);
        (eq26_e523, eq26_e523_d_n0, eq26_e523_d_n1, eq26_e523_d_n2, eq26_e523_d_n3, eq26_e523_d_n4, eq26_e523_d_n5, eq26_e523_d_n6, eq26_e523_d_n7, eq26_e523_d_n8, eq26_e523_d_n9, eq26_e523_d_n10, eq26_e523_d_n11, eq26_e523_d_n12, eq26_e523_d_n13, eq26_e523_d_n14, eq26_e523_d_n15, eq26_e523_d_n16, eq26_e523_d_n17, eq26_e523_d_n18, eq26_e523_d_n19, eq26_e523_d_n20, eq26_e523_d_n21, eq26_e523_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e525;
        let eq26_node_derivatives: [f64; 23] = [eq26_e525_d_n0, eq26_e525_d_n1, eq26_e525_d_n2, eq26_e525_d_n3, eq26_e525_d_n4, eq26_e525_d_n5, eq26_e525_d_n6, eq26_e525_d_n7, eq26_e525_d_n8, eq26_e525_d_n9, eq26_e525_d_n10, eq26_e525_d_n11, eq26_e525_d_n12, eq26_e525_d_n13, eq26_e525_d_n14, eq26_e525_d_n15, eq26_e525_d_n16, eq26_e525_d_n17, eq26_e525_d_n18, eq26_e525_d_n19, eq26_e525_d_n20, eq26_e525_d_n21, eq26_e525_d_n22];
        let eq26_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            None,
            multiplicity * (eq26_value),
            nodes,
            &eq26_node_derivatives,
            branches,
            &eq26_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq27_e539, eq27_e539_d_n0, eq27_e539_d_n1, eq27_e539_d_n2, eq27_e539_d_n3, eq27_e539_d_n4, eq27_e539_d_n5, eq27_e539_d_n6, eq27_e539_d_n7, eq27_e539_d_n8, eq27_e539_d_n9, eq27_e539_d_n10, eq27_e539_d_n11, eq27_e539_d_n12, eq27_e539_d_n13, eq27_e539_d_n14, eq27_e539_d_n15, eq27_e539_d_n16, eq27_e539_d_n17, eq27_e539_d_n18, eq27_e539_d_n19, eq27_e539_d_n20, eq27_e539_d_n21, eq27_e539_d_n22,) = {
    if (s.b[390] && (!((s.b[387] || s.b[388]) || s.b[389]))) {
        let eq27_e536: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, (nv5 - 0.0));
        let eq27_e537: f64 = (s.v[149] * eq27_e536);
        let eq27_e537_d_n0: f64 = (s.dn[149][0] * eq27_e536);
        let eq27_e537_d_n1: f64 = (s.dn[149][1] * eq27_e536);
        let eq27_e537_d_n2: f64 = (s.dn[149][2] * eq27_e536);
        let eq27_e537_d_n3: f64 = (s.dn[149][3] * eq27_e536);
        let eq27_e537_d_n4: f64 = (s.dn[149][4] * eq27_e536);
        let eq27_e537_d_n5: f64 = ((s.dn[149][5] * eq27_e536) + (s.v[149] * ddt_scale));
        let eq27_e537_d_n6: f64 = (s.dn[149][6] * eq27_e536);
        let eq27_e537_d_n7: f64 = (s.dn[149][7] * eq27_e536);
        let eq27_e537_d_n8: f64 = (s.dn[149][8] * eq27_e536);
        let eq27_e537_d_n9: f64 = (s.dn[149][9] * eq27_e536);
        let eq27_e537_d_n10: f64 = (s.dn[149][10] * eq27_e536);
        let eq27_e537_d_n11: f64 = (s.dn[149][11] * eq27_e536);
        let eq27_e537_d_n12: f64 = (s.dn[149][12] * eq27_e536);
        let eq27_e537_d_n13: f64 = (s.dn[149][13] * eq27_e536);
        let eq27_e537_d_n14: f64 = (s.dn[149][14] * eq27_e536);
        let eq27_e537_d_n15: f64 = (s.dn[149][15] * eq27_e536);
        let eq27_e537_d_n16: f64 = (s.dn[149][16] * eq27_e536);
        let eq27_e537_d_n17: f64 = (s.dn[149][17] * eq27_e536);
        let eq27_e537_d_n18: f64 = (s.dn[149][18] * eq27_e536);
        let eq27_e537_d_n19: f64 = (s.dn[149][19] * eq27_e536);
        let eq27_e537_d_n20: f64 = (s.dn[149][20] * eq27_e536);
        let eq27_e537_d_n21: f64 = (s.dn[149][21] * eq27_e536);
        let eq27_e537_d_n22: f64 = (s.dn[149][22] * eq27_e536);
        (eq27_e537, eq27_e537_d_n0, eq27_e537_d_n1, eq27_e537_d_n2, eq27_e537_d_n3, eq27_e537_d_n4, eq27_e537_d_n5, eq27_e537_d_n6, eq27_e537_d_n7, eq27_e537_d_n8, eq27_e537_d_n9, eq27_e537_d_n10, eq27_e537_d_n11, eq27_e537_d_n12, eq27_e537_d_n13, eq27_e537_d_n14, eq27_e537_d_n15, eq27_e537_d_n16, eq27_e537_d_n17, eq27_e537_d_n18, eq27_e537_d_n19, eq27_e537_d_n20, eq27_e537_d_n21, eq27_e537_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e539;
        let eq27_node_derivatives: [f64; 23] = [eq27_e539_d_n0, eq27_e539_d_n1, eq27_e539_d_n2, eq27_e539_d_n3, eq27_e539_d_n4, eq27_e539_d_n5, eq27_e539_d_n6, eq27_e539_d_n7, eq27_e539_d_n8, eq27_e539_d_n9, eq27_e539_d_n10, eq27_e539_d_n11, eq27_e539_d_n12, eq27_e539_d_n13, eq27_e539_d_n14, eq27_e539_d_n15, eq27_e539_d_n16, eq27_e539_d_n17, eq27_e539_d_n18, eq27_e539_d_n19, eq27_e539_d_n20, eq27_e539_d_n21, eq27_e539_d_n22];
        let eq27_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            None,
            multiplicity * (eq27_value),
            nodes,
            &eq27_node_derivatives,
            branches,
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
        stamper.stamp_potential_const(
            branches[16],
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
        stamper.stamp_potential_const(
            branches[17],
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
        stamper.stamp_potential_const(
            branches[18],
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
        stamper.stamp_potential_const(
            branches[19],
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
        stamper.stamp_potential_const(
            branches[20],
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
        stamper.stamp_potential_const(
            branches[21],
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
        stamper.stamp_potential_const(
            branches[22],
            eq34_value,
        );
        let (eq35_e633, eq35_e633_d_n0, eq35_e633_d_n1, eq35_e633_d_n2, eq35_e633_d_n3, eq35_e633_d_n4, eq35_e633_d_n5, eq35_e633_d_n6, eq35_e633_d_n7, eq35_e633_d_n8, eq35_e633_d_n9, eq35_e633_d_n10, eq35_e633_d_n11, eq35_e633_d_n12, eq35_e633_d_n13, eq35_e633_d_n14, eq35_e633_d_n15, eq35_e633_d_n16, eq35_e633_d_n17, eq35_e633_d_n18, eq35_e633_d_n19, eq35_e633_d_n20, eq35_e633_d_n21, eq35_e633_d_n22,) = {
    if (s.b[391] && (!(((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]))) {
        (s.v[136], s.dn[136][0], s.dn[136][1], s.dn[136][2], s.dn[136][3], s.dn[136][4], s.dn[136][5], s.dn[136][6], s.dn[136][7], s.dn[136][8], s.dn[136][9], s.dn[136][10], s.dn[136][11], s.dn[136][12], s.dn[136][13], s.dn[136][14], s.dn[136][15], s.dn[136][16], s.dn[136][17], s.dn[136][18], s.dn[136][19], s.dn[136][20], s.dn[136][21], s.dn[136][22],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e633;
        let eq35_node_derivatives: [f64; 23] = [eq35_e633_d_n0, eq35_e633_d_n1, eq35_e633_d_n2, eq35_e633_d_n3, eq35_e633_d_n4, eq35_e633_d_n5, eq35_e633_d_n6, eq35_e633_d_n7, eq35_e633_d_n8, eq35_e633_d_n9, eq35_e633_d_n10, eq35_e633_d_n11, eq35_e633_d_n12, eq35_e633_d_n13, eq35_e633_d_n14, eq35_e633_d_n15, eq35_e633_d_n16, eq35_e633_d_n17, eq35_e633_d_n18, eq35_e633_d_n19, eq35_e633_d_n20, eq35_e633_d_n21, eq35_e633_d_n22];
        let eq35_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_dense(
            branches[23],
            eq35_value,
            nodes,
            &eq35_node_derivatives,
            branches,
            &eq35_branch_derivatives,
        );
        let (eq36_e648, eq36_e648_d_n0, eq36_e648_d_n1, eq36_e648_d_n2, eq36_e648_d_n3, eq36_e648_d_n4, eq36_e648_d_n5, eq36_e648_d_n6, eq36_e648_d_n7, eq36_e648_d_n8, eq36_e648_d_n9, eq36_e648_d_n10, eq36_e648_d_n11, eq36_e648_d_n12, eq36_e648_d_n13, eq36_e648_d_n14, eq36_e648_d_n15, eq36_e648_d_n16, eq36_e648_d_n17, eq36_e648_d_n18, eq36_e648_d_n19, eq36_e648_d_n20, eq36_e648_d_n21, eq36_e648_d_n22,) = {
    if (s.b[391] && (!(((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]))) {
        let eq36_e646: f64 = ((nv11 - nv12) / s.v[338]);
        let eq36_e646_d_n0: f64 = (-(((nv11 - nv12) * s.dn[338][0]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n1: f64 = (-(((nv11 - nv12) * s.dn[338][1]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n2: f64 = (-(((nv11 - nv12) * s.dn[338][2]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n3: f64 = (-(((nv11 - nv12) * s.dn[338][3]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n4: f64 = (-(((nv11 - nv12) * s.dn[338][4]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n5: f64 = (-(((nv11 - nv12) * s.dn[338][5]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n6: f64 = (-(((nv11 - nv12) * s.dn[338][6]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n7: f64 = (-(((nv11 - nv12) * s.dn[338][7]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n8: f64 = (-(((nv11 - nv12) * s.dn[338][8]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n9: f64 = (-(((nv11 - nv12) * s.dn[338][9]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n10: f64 = (-(((nv11 - nv12) * s.dn[338][10]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n11: f64 = ((s.v[338] - ((nv11 - nv12) * s.dn[338][11])) / (s.v[338] * s.v[338]));
        let eq36_e646_d_n12: f64 = (((-s.v[338]) - ((nv11 - nv12) * s.dn[338][12])) / (s.v[338] * s.v[338]));
        let eq36_e646_d_n13: f64 = (-(((nv11 - nv12) * s.dn[338][13]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n14: f64 = (-(((nv11 - nv12) * s.dn[338][14]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n15: f64 = (-(((nv11 - nv12) * s.dn[338][15]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n16: f64 = (-(((nv11 - nv12) * s.dn[338][16]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n17: f64 = (-(((nv11 - nv12) * s.dn[338][17]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n18: f64 = (-(((nv11 - nv12) * s.dn[338][18]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n19: f64 = (-(((nv11 - nv12) * s.dn[338][19]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n20: f64 = (-(((nv11 - nv12) * s.dn[338][20]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n21: f64 = (-(((nv11 - nv12) * s.dn[338][21]) / (s.v[338] * s.v[338])));
        let eq36_e646_d_n22: f64 = (-(((nv11 - nv12) * s.dn[338][22]) / (s.v[338] * s.v[338])));
        (eq36_e646, eq36_e646_d_n0, eq36_e646_d_n1, eq36_e646_d_n2, eq36_e646_d_n3, eq36_e646_d_n4, eq36_e646_d_n5, eq36_e646_d_n6, eq36_e646_d_n7, eq36_e646_d_n8, eq36_e646_d_n9, eq36_e646_d_n10, eq36_e646_d_n11, eq36_e646_d_n12, eq36_e646_d_n13, eq36_e646_d_n14, eq36_e646_d_n15, eq36_e646_d_n16, eq36_e646_d_n17, eq36_e646_d_n18, eq36_e646_d_n19, eq36_e646_d_n20, eq36_e646_d_n21, eq36_e646_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e648;
        let eq36_node_derivatives: [f64; 23] = [eq36_e648_d_n0, eq36_e648_d_n1, eq36_e648_d_n2, eq36_e648_d_n3, eq36_e648_d_n4, eq36_e648_d_n5, eq36_e648_d_n6, eq36_e648_d_n7, eq36_e648_d_n8, eq36_e648_d_n9, eq36_e648_d_n10, eq36_e648_d_n11, eq36_e648_d_n12, eq36_e648_d_n13, eq36_e648_d_n14, eq36_e648_d_n15, eq36_e648_d_n16, eq36_e648_d_n17, eq36_e648_d_n18, eq36_e648_d_n19, eq36_e648_d_n20, eq36_e648_d_n21, eq36_e648_d_n22];
        let eq36_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[12]),
            multiplicity * (eq36_value),
            nodes,
            &eq36_node_derivatives,
            branches,
            &eq36_branch_derivatives,
            multiplicity,
        );
        let (eq37_e668, eq37_e668_d_n12,) = {
    if (s.b[391] && (!(((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]))) {
        let eq37_e661: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, (nv12 - 0.0));
        let eq37_e662: f64 = (p.p97 * eq37_e661);
        let eq37_e662_d_n12: f64 = (p.p97 * ddt_scale);
        let eq37_e665: f64 = (1e-12 * (nv12 - 0.0));
        let eq37_e665_d_n12: f64 = 1e-12;
        let eq37_e666: f64 = (eq37_e662 + eq37_e665);
        let eq37_e666_d_n12: f64 = (eq37_e662_d_n12 + eq37_e665_d_n12);
        (eq37_e666, eq37_e666_d_n12,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq37_value: f64 = eq37_e668;
        stamper.stamp_current_node1(
            Some(nodes[12]),
            None,
            multiplicity * (eq37_value),
            nodes[12],
            multiplicity * (eq37_e668_d_n12),
        );
        let (eq38_e681, eq38_e681_d_n0, eq38_e681_d_n1, eq38_e681_d_n2, eq38_e681_d_n3, eq38_e681_d_n4, eq38_e681_d_n5, eq38_e681_d_n6, eq38_e681_d_n7, eq38_e681_d_n8, eq38_e681_d_n9, eq38_e681_d_n10, eq38_e681_d_n11, eq38_e681_d_n12, eq38_e681_d_n13, eq38_e681_d_n14, eq38_e681_d_n15, eq38_e681_d_n16, eq38_e681_d_n17, eq38_e681_d_n18, eq38_e681_d_n19, eq38_e681_d_n20, eq38_e681_d_n21, eq38_e681_d_n22,) = {
    if (s.b[391] && (!(((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]))) {
        (s.v[90], s.dn[90][0], s.dn[90][1], s.dn[90][2], s.dn[90][3], s.dn[90][4], s.dn[90][5], s.dn[90][6], s.dn[90][7], s.dn[90][8], s.dn[90][9], s.dn[90][10], s.dn[90][11], s.dn[90][12], s.dn[90][13], s.dn[90][14], s.dn[90][15], s.dn[90][16], s.dn[90][17], s.dn[90][18], s.dn[90][19], s.dn[90][20], s.dn[90][21], s.dn[90][22],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e681;
        let eq38_node_derivatives: [f64; 23] = [eq38_e681_d_n0, eq38_e681_d_n1, eq38_e681_d_n2, eq38_e681_d_n3, eq38_e681_d_n4, eq38_e681_d_n5, eq38_e681_d_n6, eq38_e681_d_n7, eq38_e681_d_n8, eq38_e681_d_n9, eq38_e681_d_n10, eq38_e681_d_n11, eq38_e681_d_n12, eq38_e681_d_n13, eq38_e681_d_n14, eq38_e681_d_n15, eq38_e681_d_n16, eq38_e681_d_n17, eq38_e681_d_n18, eq38_e681_d_n19, eq38_e681_d_n20, eq38_e681_d_n21, eq38_e681_d_n22];
        let eq38_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_dense(
            branches[24],
            eq38_value,
            nodes,
            &eq38_node_derivatives,
            branches,
            &eq38_branch_derivatives,
        );
        let (eq39_e696, eq39_e696_d_n0, eq39_e696_d_n1, eq39_e696_d_n2, eq39_e696_d_n3, eq39_e696_d_n4, eq39_e696_d_n5, eq39_e696_d_n6, eq39_e696_d_n7, eq39_e696_d_n8, eq39_e696_d_n9, eq39_e696_d_n10, eq39_e696_d_n11, eq39_e696_d_n12, eq39_e696_d_n13, eq39_e696_d_n14, eq39_e696_d_n15, eq39_e696_d_n16, eq39_e696_d_n17, eq39_e696_d_n18, eq39_e696_d_n19, eq39_e696_d_n20, eq39_e696_d_n21, eq39_e696_d_n22,) = {
    if (s.b[391] && (!(((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]))) {
        let eq39_e694: f64 = ((nv13 - nv14) / s.v[343]);
        let eq39_e694_d_n0: f64 = (-(((nv13 - nv14) * s.dn[343][0]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n1: f64 = (-(((nv13 - nv14) * s.dn[343][1]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n2: f64 = (-(((nv13 - nv14) * s.dn[343][2]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n3: f64 = (-(((nv13 - nv14) * s.dn[343][3]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n4: f64 = (-(((nv13 - nv14) * s.dn[343][4]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n5: f64 = (-(((nv13 - nv14) * s.dn[343][5]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n6: f64 = (-(((nv13 - nv14) * s.dn[343][6]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n7: f64 = (-(((nv13 - nv14) * s.dn[343][7]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n8: f64 = (-(((nv13 - nv14) * s.dn[343][8]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n9: f64 = (-(((nv13 - nv14) * s.dn[343][9]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n10: f64 = (-(((nv13 - nv14) * s.dn[343][10]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n11: f64 = (-(((nv13 - nv14) * s.dn[343][11]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n12: f64 = (-(((nv13 - nv14) * s.dn[343][12]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n13: f64 = ((s.v[343] - ((nv13 - nv14) * s.dn[343][13])) / (s.v[343] * s.v[343]));
        let eq39_e694_d_n14: f64 = (((-s.v[343]) - ((nv13 - nv14) * s.dn[343][14])) / (s.v[343] * s.v[343]));
        let eq39_e694_d_n15: f64 = (-(((nv13 - nv14) * s.dn[343][15]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n16: f64 = (-(((nv13 - nv14) * s.dn[343][16]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n17: f64 = (-(((nv13 - nv14) * s.dn[343][17]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n18: f64 = (-(((nv13 - nv14) * s.dn[343][18]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n19: f64 = (-(((nv13 - nv14) * s.dn[343][19]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n20: f64 = (-(((nv13 - nv14) * s.dn[343][20]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n21: f64 = (-(((nv13 - nv14) * s.dn[343][21]) / (s.v[343] * s.v[343])));
        let eq39_e694_d_n22: f64 = (-(((nv13 - nv14) * s.dn[343][22]) / (s.v[343] * s.v[343])));
        (eq39_e694, eq39_e694_d_n0, eq39_e694_d_n1, eq39_e694_d_n2, eq39_e694_d_n3, eq39_e694_d_n4, eq39_e694_d_n5, eq39_e694_d_n6, eq39_e694_d_n7, eq39_e694_d_n8, eq39_e694_d_n9, eq39_e694_d_n10, eq39_e694_d_n11, eq39_e694_d_n12, eq39_e694_d_n13, eq39_e694_d_n14, eq39_e694_d_n15, eq39_e694_d_n16, eq39_e694_d_n17, eq39_e694_d_n18, eq39_e694_d_n19, eq39_e694_d_n20, eq39_e694_d_n21, eq39_e694_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e696;
        let eq39_node_derivatives: [f64; 23] = [eq39_e696_d_n0, eq39_e696_d_n1, eq39_e696_d_n2, eq39_e696_d_n3, eq39_e696_d_n4, eq39_e696_d_n5, eq39_e696_d_n6, eq39_e696_d_n7, eq39_e696_d_n8, eq39_e696_d_n9, eq39_e696_d_n10, eq39_e696_d_n11, eq39_e696_d_n12, eq39_e696_d_n13, eq39_e696_d_n14, eq39_e696_d_n15, eq39_e696_d_n16, eq39_e696_d_n17, eq39_e696_d_n18, eq39_e696_d_n19, eq39_e696_d_n20, eq39_e696_d_n21, eq39_e696_d_n22];
        let eq39_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            Some(nodes[14]),
            multiplicity * (eq39_value),
            nodes,
            &eq39_node_derivatives,
            branches,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let (eq40_e716, eq40_e716_d_n14,) = {
    if (s.b[391] && (!(((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]))) {
        let eq40_e709: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, (nv14 - 0.0));
        let eq40_e710: f64 = (p.p83 * eq40_e709);
        let eq40_e710_d_n14: f64 = (p.p83 * ddt_scale);
        let eq40_e713: f64 = (1e-12 * (nv14 - 0.0));
        let eq40_e713_d_n14: f64 = 1e-12;
        let eq40_e714: f64 = (eq40_e710 + eq40_e713);
        let eq40_e714_d_n14: f64 = (eq40_e710_d_n14 + eq40_e713_d_n14);
        (eq40_e714, eq40_e714_d_n14,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e716;
        stamper.stamp_current_node1(
            Some(nodes[14]),
            None,
            multiplicity * (eq40_value),
            nodes[14],
            multiplicity * (eq40_e716_d_n14),
        );
        let (eq41_e747, eq41_e747_d_n0, eq41_e747_d_n1, eq41_e747_d_n2, eq41_e747_d_n3, eq41_e747_d_n4, eq41_e747_d_n5, eq41_e747_d_n6, eq41_e747_d_n7, eq41_e747_d_n8, eq41_e747_d_n9, eq41_e747_d_n10, eq41_e747_d_n11, eq41_e747_d_n12, eq41_e747_d_n13, eq41_e747_d_n14, eq41_e747_d_n15, eq41_e747_d_n16, eq41_e747_d_n17, eq41_e747_d_n18, eq41_e747_d_n19, eq41_e747_d_n20, eq41_e747_d_n21, eq41_e747_d_n22,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        let eq41_e730: f64 = (-p.p135);
        let eq41_e732: f64 = (eq41_e730 * s.v[363]);
        let eq41_e732_d_n0: f64 = (eq41_e730 * s.dn[363][0]);
        let eq41_e732_d_n1: f64 = (eq41_e730 * s.dn[363][1]);
        let eq41_e732_d_n2: f64 = (eq41_e730 * s.dn[363][2]);
        let eq41_e732_d_n3: f64 = (eq41_e730 * s.dn[363][3]);
        let eq41_e732_d_n4: f64 = (eq41_e730 * s.dn[363][4]);
        let eq41_e732_d_n5: f64 = (eq41_e730 * s.dn[363][5]);
        let eq41_e732_d_n6: f64 = (eq41_e730 * s.dn[363][6]);
        let eq41_e732_d_n7: f64 = (eq41_e730 * s.dn[363][7]);
        let eq41_e732_d_n8: f64 = (eq41_e730 * s.dn[363][8]);
        let eq41_e732_d_n9: f64 = (eq41_e730 * s.dn[363][9]);
        let eq41_e732_d_n10: f64 = (eq41_e730 * s.dn[363][10]);
        let eq41_e732_d_n11: f64 = (eq41_e730 * s.dn[363][11]);
        let eq41_e732_d_n12: f64 = (eq41_e730 * s.dn[363][12]);
        let eq41_e732_d_n13: f64 = (eq41_e730 * s.dn[363][13]);
        let eq41_e732_d_n14: f64 = (eq41_e730 * s.dn[363][14]);
        let eq41_e732_d_n15: f64 = (eq41_e730 * s.dn[363][15]);
        let eq41_e732_d_n16: f64 = (eq41_e730 * s.dn[363][16]);
        let eq41_e732_d_n17: f64 = (eq41_e730 * s.dn[363][17]);
        let eq41_e732_d_n18: f64 = (eq41_e730 * s.dn[363][18]);
        let eq41_e732_d_n19: f64 = (eq41_e730 * s.dn[363][19]);
        let eq41_e732_d_n20: f64 = (eq41_e730 * s.dn[363][20]);
        let eq41_e732_d_n21: f64 = (eq41_e730 * s.dn[363][21]);
        let eq41_e732_d_n22: f64 = (eq41_e730 * s.dn[363][22]);
        let eq41_e735: f64 = (p.p136 - (nv5 - 0.0));
        let eq41_e735_d_n5: f64 = (-1.0);
        let eq41_e736: f64 = (eq41_e732 * eq41_e735);
        let eq41_e736_d_n0: f64 = (eq41_e732_d_n0 * eq41_e735);
        let eq41_e736_d_n1: f64 = (eq41_e732_d_n1 * eq41_e735);
        let eq41_e736_d_n2: f64 = (eq41_e732_d_n2 * eq41_e735);
        let eq41_e736_d_n3: f64 = (eq41_e732_d_n3 * eq41_e735);
        let eq41_e736_d_n4: f64 = (eq41_e732_d_n4 * eq41_e735);
        let eq41_e736_d_n5: f64 = ((eq41_e732_d_n5 * eq41_e735) + (eq41_e732 * eq41_e735_d_n5));
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
        let eq41_e739: f64 = (2.0 * s.v[362]);
        let eq41_e739_d_n0: f64 = (2.0 * s.dn[362][0]);
        let eq41_e739_d_n1: f64 = (2.0 * s.dn[362][1]);
        let eq41_e739_d_n2: f64 = (2.0 * s.dn[362][2]);
        let eq41_e739_d_n3: f64 = (2.0 * s.dn[362][3]);
        let eq41_e739_d_n4: f64 = (2.0 * s.dn[362][4]);
        let eq41_e739_d_n5: f64 = (2.0 * s.dn[362][5]);
        let eq41_e739_d_n6: f64 = (2.0 * s.dn[362][6]);
        let eq41_e739_d_n7: f64 = (2.0 * s.dn[362][7]);
        let eq41_e739_d_n8: f64 = (2.0 * s.dn[362][8]);
        let eq41_e739_d_n9: f64 = (2.0 * s.dn[362][9]);
        let eq41_e739_d_n10: f64 = (2.0 * s.dn[362][10]);
        let eq41_e739_d_n11: f64 = (2.0 * s.dn[362][11]);
        let eq41_e739_d_n12: f64 = (2.0 * s.dn[362][12]);
        let eq41_e739_d_n13: f64 = (2.0 * s.dn[362][13]);
        let eq41_e739_d_n14: f64 = (2.0 * s.dn[362][14]);
        let eq41_e739_d_n15: f64 = (2.0 * s.dn[362][15]);
        let eq41_e739_d_n16: f64 = (2.0 * s.dn[362][16]);
        let eq41_e739_d_n17: f64 = (2.0 * s.dn[362][17]);
        let eq41_e739_d_n18: f64 = (2.0 * s.dn[362][18]);
        let eq41_e739_d_n19: f64 = (2.0 * s.dn[362][19]);
        let eq41_e739_d_n20: f64 = (2.0 * s.dn[362][20]);
        let eq41_e739_d_n21: f64 = (2.0 * s.dn[362][21]);
        let eq41_e739_d_n22: f64 = (2.0 * s.dn[362][22]);
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
        (eq41_e745, eq41_e745_d_n0, eq41_e745_d_n1, eq41_e745_d_n2, eq41_e745_d_n3, eq41_e745_d_n4, eq41_e745_d_n5, eq41_e745_d_n6, eq41_e745_d_n7, eq41_e745_d_n8, eq41_e745_d_n9, eq41_e745_d_n10, eq41_e745_d_n11, eq41_e745_d_n12, eq41_e745_d_n13, eq41_e745_d_n14, eq41_e745_d_n15, eq41_e745_d_n16, eq41_e745_d_n17, eq41_e745_d_n18, eq41_e745_d_n19, eq41_e745_d_n20, eq41_e745_d_n21, eq41_e745_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_value: f64 = eq41_e747;
        let eq41_node_derivatives: [f64; 23] = [eq41_e747_d_n0, eq41_e747_d_n1, eq41_e747_d_n2, eq41_e747_d_n3, eq41_e747_d_n4, eq41_e747_d_n5, eq41_e747_d_n6, eq41_e747_d_n7, eq41_e747_d_n8, eq41_e747_d_n9, eq41_e747_d_n10, eq41_e747_d_n11, eq41_e747_d_n12, eq41_e747_d_n13, eq41_e747_d_n14, eq41_e747_d_n15, eq41_e747_d_n16, eq41_e747_d_n17, eq41_e747_d_n18, eq41_e747_d_n19, eq41_e747_d_n20, eq41_e747_d_n21, eq41_e747_d_n22];
        let eq41_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            None,
            multiplicity * (eq41_value),
            nodes,
            &eq41_node_derivatives,
            branches,
            &eq41_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq42_e766, eq42_e766_d_n0, eq42_e766_d_n1, eq42_e766_d_n2, eq42_e766_d_n3, eq42_e766_d_n4, eq42_e766_d_n5, eq42_e766_d_n6, eq42_e766_d_n7, eq42_e766_d_n8, eq42_e766_d_n9, eq42_e766_d_n10, eq42_e766_d_n11, eq42_e766_d_n12, eq42_e766_d_n13, eq42_e766_d_n14, eq42_e766_d_n15, eq42_e766_d_n16, eq42_e766_d_n17, eq42_e766_d_n18, eq42_e766_d_n19, eq42_e766_d_n20, eq42_e766_d_n21, eq42_e766_d_n22,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        let eq42_e762: f64 = (p.p135 * s.v[363]);
        let eq42_e762_d_n0: f64 = (p.p135 * s.dn[363][0]);
        let eq42_e762_d_n1: f64 = (p.p135 * s.dn[363][1]);
        let eq42_e762_d_n2: f64 = (p.p135 * s.dn[363][2]);
        let eq42_e762_d_n3: f64 = (p.p135 * s.dn[363][3]);
        let eq42_e762_d_n4: f64 = (p.p135 * s.dn[363][4]);
        let eq42_e762_d_n5: f64 = (p.p135 * s.dn[363][5]);
        let eq42_e762_d_n6: f64 = (p.p135 * s.dn[363][6]);
        let eq42_e762_d_n7: f64 = (p.p135 * s.dn[363][7]);
        let eq42_e762_d_n8: f64 = (p.p135 * s.dn[363][8]);
        let eq42_e762_d_n9: f64 = (p.p135 * s.dn[363][9]);
        let eq42_e762_d_n10: f64 = (p.p135 * s.dn[363][10]);
        let eq42_e762_d_n11: f64 = (p.p135 * s.dn[363][11]);
        let eq42_e762_d_n12: f64 = (p.p135 * s.dn[363][12]);
        let eq42_e762_d_n13: f64 = (p.p135 * s.dn[363][13]);
        let eq42_e762_d_n14: f64 = (p.p135 * s.dn[363][14]);
        let eq42_e762_d_n15: f64 = (p.p135 * s.dn[363][15]);
        let eq42_e762_d_n16: f64 = (p.p135 * s.dn[363][16]);
        let eq42_e762_d_n17: f64 = (p.p135 * s.dn[363][17]);
        let eq42_e762_d_n18: f64 = (p.p135 * s.dn[363][18]);
        let eq42_e762_d_n19: f64 = (p.p135 * s.dn[363][19]);
        let eq42_e762_d_n20: f64 = (p.p135 * s.dn[363][20]);
        let eq42_e762_d_n21: f64 = (p.p135 * s.dn[363][21]);
        let eq42_e762_d_n22: f64 = (p.p135 * s.dn[363][22]);
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
        (eq42_e764, eq42_e764_d_n0, eq42_e764_d_n1, eq42_e764_d_n2, eq42_e764_d_n3, eq42_e764_d_n4, eq42_e764_d_n5, eq42_e764_d_n6, eq42_e764_d_n7, eq42_e764_d_n8, eq42_e764_d_n9, eq42_e764_d_n10, eq42_e764_d_n11, eq42_e764_d_n12, eq42_e764_d_n13, eq42_e764_d_n14, eq42_e764_d_n15, eq42_e764_d_n16, eq42_e764_d_n17, eq42_e764_d_n18, eq42_e764_d_n19, eq42_e764_d_n20, eq42_e764_d_n21, eq42_e764_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_value: f64 = eq42_e766;
        let eq42_node_derivatives: [f64; 23] = [eq42_e766_d_n0, eq42_e766_d_n1, eq42_e766_d_n2, eq42_e766_d_n3, eq42_e766_d_n4, eq42_e766_d_n5, eq42_e766_d_n6, eq42_e766_d_n7, eq42_e766_d_n8, eq42_e766_d_n9, eq42_e766_d_n10, eq42_e766_d_n11, eq42_e766_d_n12, eq42_e766_d_n13, eq42_e766_d_n14, eq42_e766_d_n15, eq42_e766_d_n16, eq42_e766_d_n17, eq42_e766_d_n18, eq42_e766_d_n19, eq42_e766_d_n20, eq42_e766_d_n21, eq42_e766_d_n22];
        let eq42_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            None,
            multiplicity * (eq42_value),
            nodes,
            &eq42_node_derivatives,
            branches,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let (eq43_e784, eq43_e784_d_n5,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        let eq43_e781: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, (nv5 - 0.0));
        let eq43_e782: f64 = (p.p135 * eq43_e781);
        let eq43_e782_d_n5: f64 = (p.p135 * ddt_scale);
        (eq43_e782, eq43_e782_d_n5,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq43_value: f64 = eq43_e784;
        stamper.stamp_current_node1(
            Some(nodes[5]),
            None,
            multiplicity * (eq43_value),
            nodes[5],
            multiplicity * (eq43_e784_d_n5),
        );
        let (eq44_e815, eq44_e815_d_n0, eq44_e815_d_n1, eq44_e815_d_n2, eq44_e815_d_n3, eq44_e815_d_n4, eq44_e815_d_n5, eq44_e815_d_n6, eq44_e815_d_n7, eq44_e815_d_n8, eq44_e815_d_n9, eq44_e815_d_n10, eq44_e815_d_n11, eq44_e815_d_n12, eq44_e815_d_n13, eq44_e815_d_n14, eq44_e815_d_n15, eq44_e815_d_n16, eq44_e815_d_n17, eq44_e815_d_n18, eq44_e815_d_n19, eq44_e815_d_n20, eq44_e815_d_n21, eq44_e815_d_n22,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        let eq44_e798: f64 = (-p.p144);
        let eq44_e800: f64 = (eq44_e798 * s.v[367]);
        let eq44_e800_d_n0: f64 = (eq44_e798 * s.dn[367][0]);
        let eq44_e800_d_n1: f64 = (eq44_e798 * s.dn[367][1]);
        let eq44_e800_d_n2: f64 = (eq44_e798 * s.dn[367][2]);
        let eq44_e800_d_n3: f64 = (eq44_e798 * s.dn[367][3]);
        let eq44_e800_d_n4: f64 = (eq44_e798 * s.dn[367][4]);
        let eq44_e800_d_n5: f64 = (eq44_e798 * s.dn[367][5]);
        let eq44_e800_d_n6: f64 = (eq44_e798 * s.dn[367][6]);
        let eq44_e800_d_n7: f64 = (eq44_e798 * s.dn[367][7]);
        let eq44_e800_d_n8: f64 = (eq44_e798 * s.dn[367][8]);
        let eq44_e800_d_n9: f64 = (eq44_e798 * s.dn[367][9]);
        let eq44_e800_d_n10: f64 = (eq44_e798 * s.dn[367][10]);
        let eq44_e800_d_n11: f64 = (eq44_e798 * s.dn[367][11]);
        let eq44_e800_d_n12: f64 = (eq44_e798 * s.dn[367][12]);
        let eq44_e800_d_n13: f64 = (eq44_e798 * s.dn[367][13]);
        let eq44_e800_d_n14: f64 = (eq44_e798 * s.dn[367][14]);
        let eq44_e800_d_n15: f64 = (eq44_e798 * s.dn[367][15]);
        let eq44_e800_d_n16: f64 = (eq44_e798 * s.dn[367][16]);
        let eq44_e800_d_n17: f64 = (eq44_e798 * s.dn[367][17]);
        let eq44_e800_d_n18: f64 = (eq44_e798 * s.dn[367][18]);
        let eq44_e800_d_n19: f64 = (eq44_e798 * s.dn[367][19]);
        let eq44_e800_d_n20: f64 = (eq44_e798 * s.dn[367][20]);
        let eq44_e800_d_n21: f64 = (eq44_e798 * s.dn[367][21]);
        let eq44_e800_d_n22: f64 = (eq44_e798 * s.dn[367][22]);
        let eq44_e803: f64 = (p.p145 - (nv6 - 0.0));
        let eq44_e803_d_n6: f64 = (-1.0);
        let eq44_e804: f64 = (eq44_e800 * eq44_e803);
        let eq44_e804_d_n0: f64 = (eq44_e800_d_n0 * eq44_e803);
        let eq44_e804_d_n1: f64 = (eq44_e800_d_n1 * eq44_e803);
        let eq44_e804_d_n2: f64 = (eq44_e800_d_n2 * eq44_e803);
        let eq44_e804_d_n3: f64 = (eq44_e800_d_n3 * eq44_e803);
        let eq44_e804_d_n4: f64 = (eq44_e800_d_n4 * eq44_e803);
        let eq44_e804_d_n5: f64 = (eq44_e800_d_n5 * eq44_e803);
        let eq44_e804_d_n6: f64 = ((eq44_e800_d_n6 * eq44_e803) + (eq44_e800 * eq44_e803_d_n6));
        let eq44_e804_d_n7: f64 = (eq44_e800_d_n7 * eq44_e803);
        let eq44_e804_d_n8: f64 = (eq44_e800_d_n8 * eq44_e803);
        let eq44_e804_d_n9: f64 = (eq44_e800_d_n9 * eq44_e803);
        let eq44_e804_d_n10: f64 = (eq44_e800_d_n10 * eq44_e803);
        let eq44_e804_d_n11: f64 = (eq44_e800_d_n11 * eq44_e803);
        let eq44_e804_d_n12: f64 = (eq44_e800_d_n12 * eq44_e803);
        let eq44_e804_d_n13: f64 = (eq44_e800_d_n13 * eq44_e803);
        let eq44_e804_d_n14: f64 = (eq44_e800_d_n14 * eq44_e803);
        let eq44_e804_d_n15: f64 = (eq44_e800_d_n15 * eq44_e803);
        let eq44_e804_d_n16: f64 = (eq44_e800_d_n16 * eq44_e803);
        let eq44_e804_d_n17: f64 = (eq44_e800_d_n17 * eq44_e803);
        let eq44_e804_d_n18: f64 = (eq44_e800_d_n18 * eq44_e803);
        let eq44_e804_d_n19: f64 = (eq44_e800_d_n19 * eq44_e803);
        let eq44_e804_d_n20: f64 = (eq44_e800_d_n20 * eq44_e803);
        let eq44_e804_d_n21: f64 = (eq44_e800_d_n21 * eq44_e803);
        let eq44_e804_d_n22: f64 = (eq44_e800_d_n22 * eq44_e803);
        let eq44_e807: f64 = (2.0 * s.v[368]);
        let eq44_e807_d_n0: f64 = (2.0 * s.dn[368][0]);
        let eq44_e807_d_n1: f64 = (2.0 * s.dn[368][1]);
        let eq44_e807_d_n2: f64 = (2.0 * s.dn[368][2]);
        let eq44_e807_d_n3: f64 = (2.0 * s.dn[368][3]);
        let eq44_e807_d_n4: f64 = (2.0 * s.dn[368][4]);
        let eq44_e807_d_n5: f64 = (2.0 * s.dn[368][5]);
        let eq44_e807_d_n6: f64 = (2.0 * s.dn[368][6]);
        let eq44_e807_d_n7: f64 = (2.0 * s.dn[368][7]);
        let eq44_e807_d_n8: f64 = (2.0 * s.dn[368][8]);
        let eq44_e807_d_n9: f64 = (2.0 * s.dn[368][9]);
        let eq44_e807_d_n10: f64 = (2.0 * s.dn[368][10]);
        let eq44_e807_d_n11: f64 = (2.0 * s.dn[368][11]);
        let eq44_e807_d_n12: f64 = (2.0 * s.dn[368][12]);
        let eq44_e807_d_n13: f64 = (2.0 * s.dn[368][13]);
        let eq44_e807_d_n14: f64 = (2.0 * s.dn[368][14]);
        let eq44_e807_d_n15: f64 = (2.0 * s.dn[368][15]);
        let eq44_e807_d_n16: f64 = (2.0 * s.dn[368][16]);
        let eq44_e807_d_n17: f64 = (2.0 * s.dn[368][17]);
        let eq44_e807_d_n18: f64 = (2.0 * s.dn[368][18]);
        let eq44_e807_d_n19: f64 = (2.0 * s.dn[368][19]);
        let eq44_e807_d_n20: f64 = (2.0 * s.dn[368][20]);
        let eq44_e807_d_n21: f64 = (2.0 * s.dn[368][21]);
        let eq44_e807_d_n22: f64 = (2.0 * s.dn[368][22]);
        let eq44_e808: f64 = (eq44_e807).exp();
        let eq44_e808_d_n0: f64 = (eq44_e808 * eq44_e807_d_n0);
        let eq44_e808_d_n1: f64 = (eq44_e808 * eq44_e807_d_n1);
        let eq44_e808_d_n2: f64 = (eq44_e808 * eq44_e807_d_n2);
        let eq44_e808_d_n3: f64 = (eq44_e808 * eq44_e807_d_n3);
        let eq44_e808_d_n4: f64 = (eq44_e808 * eq44_e807_d_n4);
        let eq44_e808_d_n5: f64 = (eq44_e808 * eq44_e807_d_n5);
        let eq44_e808_d_n6: f64 = (eq44_e808 * eq44_e807_d_n6);
        let eq44_e808_d_n7: f64 = (eq44_e808 * eq44_e807_d_n7);
        let eq44_e808_d_n8: f64 = (eq44_e808 * eq44_e807_d_n8);
        let eq44_e808_d_n9: f64 = (eq44_e808 * eq44_e807_d_n9);
        let eq44_e808_d_n10: f64 = (eq44_e808 * eq44_e807_d_n10);
        let eq44_e808_d_n11: f64 = (eq44_e808 * eq44_e807_d_n11);
        let eq44_e808_d_n12: f64 = (eq44_e808 * eq44_e807_d_n12);
        let eq44_e808_d_n13: f64 = (eq44_e808 * eq44_e807_d_n13);
        let eq44_e808_d_n14: f64 = (eq44_e808 * eq44_e807_d_n14);
        let eq44_e808_d_n15: f64 = (eq44_e808 * eq44_e807_d_n15);
        let eq44_e808_d_n16: f64 = (eq44_e808 * eq44_e807_d_n16);
        let eq44_e808_d_n17: f64 = (eq44_e808 * eq44_e807_d_n17);
        let eq44_e808_d_n18: f64 = (eq44_e808 * eq44_e807_d_n18);
        let eq44_e808_d_n19: f64 = (eq44_e808 * eq44_e807_d_n19);
        let eq44_e808_d_n20: f64 = (eq44_e808 * eq44_e807_d_n20);
        let eq44_e808_d_n21: f64 = (eq44_e808 * eq44_e807_d_n21);
        let eq44_e808_d_n22: f64 = (eq44_e808 * eq44_e807_d_n22);
        let eq44_e810: f64 = (eq44_e808 - 1.0);
        let eq44_e811: f64 = (eq44_e804 * eq44_e810);
        let eq44_e811_d_n0: f64 = ((eq44_e804_d_n0 * eq44_e810) + (eq44_e804 * eq44_e808_d_n0));
        let eq44_e811_d_n1: f64 = ((eq44_e804_d_n1 * eq44_e810) + (eq44_e804 * eq44_e808_d_n1));
        let eq44_e811_d_n2: f64 = ((eq44_e804_d_n2 * eq44_e810) + (eq44_e804 * eq44_e808_d_n2));
        let eq44_e811_d_n3: f64 = ((eq44_e804_d_n3 * eq44_e810) + (eq44_e804 * eq44_e808_d_n3));
        let eq44_e811_d_n4: f64 = ((eq44_e804_d_n4 * eq44_e810) + (eq44_e804 * eq44_e808_d_n4));
        let eq44_e811_d_n5: f64 = ((eq44_e804_d_n5 * eq44_e810) + (eq44_e804 * eq44_e808_d_n5));
        let eq44_e811_d_n6: f64 = ((eq44_e804_d_n6 * eq44_e810) + (eq44_e804 * eq44_e808_d_n6));
        let eq44_e811_d_n7: f64 = ((eq44_e804_d_n7 * eq44_e810) + (eq44_e804 * eq44_e808_d_n7));
        let eq44_e811_d_n8: f64 = ((eq44_e804_d_n8 * eq44_e810) + (eq44_e804 * eq44_e808_d_n8));
        let eq44_e811_d_n9: f64 = ((eq44_e804_d_n9 * eq44_e810) + (eq44_e804 * eq44_e808_d_n9));
        let eq44_e811_d_n10: f64 = ((eq44_e804_d_n10 * eq44_e810) + (eq44_e804 * eq44_e808_d_n10));
        let eq44_e811_d_n11: f64 = ((eq44_e804_d_n11 * eq44_e810) + (eq44_e804 * eq44_e808_d_n11));
        let eq44_e811_d_n12: f64 = ((eq44_e804_d_n12 * eq44_e810) + (eq44_e804 * eq44_e808_d_n12));
        let eq44_e811_d_n13: f64 = ((eq44_e804_d_n13 * eq44_e810) + (eq44_e804 * eq44_e808_d_n13));
        let eq44_e811_d_n14: f64 = ((eq44_e804_d_n14 * eq44_e810) + (eq44_e804 * eq44_e808_d_n14));
        let eq44_e811_d_n15: f64 = ((eq44_e804_d_n15 * eq44_e810) + (eq44_e804 * eq44_e808_d_n15));
        let eq44_e811_d_n16: f64 = ((eq44_e804_d_n16 * eq44_e810) + (eq44_e804 * eq44_e808_d_n16));
        let eq44_e811_d_n17: f64 = ((eq44_e804_d_n17 * eq44_e810) + (eq44_e804 * eq44_e808_d_n17));
        let eq44_e811_d_n18: f64 = ((eq44_e804_d_n18 * eq44_e810) + (eq44_e804 * eq44_e808_d_n18));
        let eq44_e811_d_n19: f64 = ((eq44_e804_d_n19 * eq44_e810) + (eq44_e804 * eq44_e808_d_n19));
        let eq44_e811_d_n20: f64 = ((eq44_e804_d_n20 * eq44_e810) + (eq44_e804 * eq44_e808_d_n20));
        let eq44_e811_d_n21: f64 = ((eq44_e804_d_n21 * eq44_e810) + (eq44_e804 * eq44_e808_d_n21));
        let eq44_e811_d_n22: f64 = ((eq44_e804_d_n22 * eq44_e810) + (eq44_e804 * eq44_e808_d_n22));
        let eq44_e813: f64 = (eq44_e811 * 0.5);
        let eq44_e813_d_n0: f64 = (eq44_e811_d_n0 * 0.5);
        let eq44_e813_d_n1: f64 = (eq44_e811_d_n1 * 0.5);
        let eq44_e813_d_n2: f64 = (eq44_e811_d_n2 * 0.5);
        let eq44_e813_d_n3: f64 = (eq44_e811_d_n3 * 0.5);
        let eq44_e813_d_n4: f64 = (eq44_e811_d_n4 * 0.5);
        let eq44_e813_d_n5: f64 = (eq44_e811_d_n5 * 0.5);
        let eq44_e813_d_n6: f64 = (eq44_e811_d_n6 * 0.5);
        let eq44_e813_d_n7: f64 = (eq44_e811_d_n7 * 0.5);
        let eq44_e813_d_n8: f64 = (eq44_e811_d_n8 * 0.5);
        let eq44_e813_d_n9: f64 = (eq44_e811_d_n9 * 0.5);
        let eq44_e813_d_n10: f64 = (eq44_e811_d_n10 * 0.5);
        let eq44_e813_d_n11: f64 = (eq44_e811_d_n11 * 0.5);
        let eq44_e813_d_n12: f64 = (eq44_e811_d_n12 * 0.5);
        let eq44_e813_d_n13: f64 = (eq44_e811_d_n13 * 0.5);
        let eq44_e813_d_n14: f64 = (eq44_e811_d_n14 * 0.5);
        let eq44_e813_d_n15: f64 = (eq44_e811_d_n15 * 0.5);
        let eq44_e813_d_n16: f64 = (eq44_e811_d_n16 * 0.5);
        let eq44_e813_d_n17: f64 = (eq44_e811_d_n17 * 0.5);
        let eq44_e813_d_n18: f64 = (eq44_e811_d_n18 * 0.5);
        let eq44_e813_d_n19: f64 = (eq44_e811_d_n19 * 0.5);
        let eq44_e813_d_n20: f64 = (eq44_e811_d_n20 * 0.5);
        let eq44_e813_d_n21: f64 = (eq44_e811_d_n21 * 0.5);
        let eq44_e813_d_n22: f64 = (eq44_e811_d_n22 * 0.5);
        (eq44_e813, eq44_e813_d_n0, eq44_e813_d_n1, eq44_e813_d_n2, eq44_e813_d_n3, eq44_e813_d_n4, eq44_e813_d_n5, eq44_e813_d_n6, eq44_e813_d_n7, eq44_e813_d_n8, eq44_e813_d_n9, eq44_e813_d_n10, eq44_e813_d_n11, eq44_e813_d_n12, eq44_e813_d_n13, eq44_e813_d_n14, eq44_e813_d_n15, eq44_e813_d_n16, eq44_e813_d_n17, eq44_e813_d_n18, eq44_e813_d_n19, eq44_e813_d_n20, eq44_e813_d_n21, eq44_e813_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e815;
        let eq44_node_derivatives: [f64; 23] = [eq44_e815_d_n0, eq44_e815_d_n1, eq44_e815_d_n2, eq44_e815_d_n3, eq44_e815_d_n4, eq44_e815_d_n5, eq44_e815_d_n6, eq44_e815_d_n7, eq44_e815_d_n8, eq44_e815_d_n9, eq44_e815_d_n10, eq44_e815_d_n11, eq44_e815_d_n12, eq44_e815_d_n13, eq44_e815_d_n14, eq44_e815_d_n15, eq44_e815_d_n16, eq44_e815_d_n17, eq44_e815_d_n18, eq44_e815_d_n19, eq44_e815_d_n20, eq44_e815_d_n21, eq44_e815_d_n22];
        let eq44_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            None,
            multiplicity * (eq44_value),
            nodes,
            &eq44_node_derivatives,
            branches,
            &eq44_branch_derivatives,
            multiplicity,
        );
        let (eq45_e834, eq45_e834_d_n0, eq45_e834_d_n1, eq45_e834_d_n2, eq45_e834_d_n3, eq45_e834_d_n4, eq45_e834_d_n5, eq45_e834_d_n6, eq45_e834_d_n7, eq45_e834_d_n8, eq45_e834_d_n9, eq45_e834_d_n10, eq45_e834_d_n11, eq45_e834_d_n12, eq45_e834_d_n13, eq45_e834_d_n14, eq45_e834_d_n15, eq45_e834_d_n16, eq45_e834_d_n17, eq45_e834_d_n18, eq45_e834_d_n19, eq45_e834_d_n20, eq45_e834_d_n21, eq45_e834_d_n22,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        let eq45_e830: f64 = (p.p144 * s.v[367]);
        let eq45_e830_d_n0: f64 = (p.p144 * s.dn[367][0]);
        let eq45_e830_d_n1: f64 = (p.p144 * s.dn[367][1]);
        let eq45_e830_d_n2: f64 = (p.p144 * s.dn[367][2]);
        let eq45_e830_d_n3: f64 = (p.p144 * s.dn[367][3]);
        let eq45_e830_d_n4: f64 = (p.p144 * s.dn[367][4]);
        let eq45_e830_d_n5: f64 = (p.p144 * s.dn[367][5]);
        let eq45_e830_d_n6: f64 = (p.p144 * s.dn[367][6]);
        let eq45_e830_d_n7: f64 = (p.p144 * s.dn[367][7]);
        let eq45_e830_d_n8: f64 = (p.p144 * s.dn[367][8]);
        let eq45_e830_d_n9: f64 = (p.p144 * s.dn[367][9]);
        let eq45_e830_d_n10: f64 = (p.p144 * s.dn[367][10]);
        let eq45_e830_d_n11: f64 = (p.p144 * s.dn[367][11]);
        let eq45_e830_d_n12: f64 = (p.p144 * s.dn[367][12]);
        let eq45_e830_d_n13: f64 = (p.p144 * s.dn[367][13]);
        let eq45_e830_d_n14: f64 = (p.p144 * s.dn[367][14]);
        let eq45_e830_d_n15: f64 = (p.p144 * s.dn[367][15]);
        let eq45_e830_d_n16: f64 = (p.p144 * s.dn[367][16]);
        let eq45_e830_d_n17: f64 = (p.p144 * s.dn[367][17]);
        let eq45_e830_d_n18: f64 = (p.p144 * s.dn[367][18]);
        let eq45_e830_d_n19: f64 = (p.p144 * s.dn[367][19]);
        let eq45_e830_d_n20: f64 = (p.p144 * s.dn[367][20]);
        let eq45_e830_d_n21: f64 = (p.p144 * s.dn[367][21]);
        let eq45_e830_d_n22: f64 = (p.p144 * s.dn[367][22]);
        let eq45_e832: f64 = (eq45_e830 * (nv6 - 0.0));
        let eq45_e832_d_n0: f64 = (eq45_e830_d_n0 * (nv6 - 0.0));
        let eq45_e832_d_n1: f64 = (eq45_e830_d_n1 * (nv6 - 0.0));
        let eq45_e832_d_n2: f64 = (eq45_e830_d_n2 * (nv6 - 0.0));
        let eq45_e832_d_n3: f64 = (eq45_e830_d_n3 * (nv6 - 0.0));
        let eq45_e832_d_n4: f64 = (eq45_e830_d_n4 * (nv6 - 0.0));
        let eq45_e832_d_n5: f64 = (eq45_e830_d_n5 * (nv6 - 0.0));
        let eq45_e832_d_n6: f64 = ((eq45_e830_d_n6 * (nv6 - 0.0)) + eq45_e830);
        let eq45_e832_d_n7: f64 = (eq45_e830_d_n7 * (nv6 - 0.0));
        let eq45_e832_d_n8: f64 = (eq45_e830_d_n8 * (nv6 - 0.0));
        let eq45_e832_d_n9: f64 = (eq45_e830_d_n9 * (nv6 - 0.0));
        let eq45_e832_d_n10: f64 = (eq45_e830_d_n10 * (nv6 - 0.0));
        let eq45_e832_d_n11: f64 = (eq45_e830_d_n11 * (nv6 - 0.0));
        let eq45_e832_d_n12: f64 = (eq45_e830_d_n12 * (nv6 - 0.0));
        let eq45_e832_d_n13: f64 = (eq45_e830_d_n13 * (nv6 - 0.0));
        let eq45_e832_d_n14: f64 = (eq45_e830_d_n14 * (nv6 - 0.0));
        let eq45_e832_d_n15: f64 = (eq45_e830_d_n15 * (nv6 - 0.0));
        let eq45_e832_d_n16: f64 = (eq45_e830_d_n16 * (nv6 - 0.0));
        let eq45_e832_d_n17: f64 = (eq45_e830_d_n17 * (nv6 - 0.0));
        let eq45_e832_d_n18: f64 = (eq45_e830_d_n18 * (nv6 - 0.0));
        let eq45_e832_d_n19: f64 = (eq45_e830_d_n19 * (nv6 - 0.0));
        let eq45_e832_d_n20: f64 = (eq45_e830_d_n20 * (nv6 - 0.0));
        let eq45_e832_d_n21: f64 = (eq45_e830_d_n21 * (nv6 - 0.0));
        let eq45_e832_d_n22: f64 = (eq45_e830_d_n22 * (nv6 - 0.0));
        (eq45_e832, eq45_e832_d_n0, eq45_e832_d_n1, eq45_e832_d_n2, eq45_e832_d_n3, eq45_e832_d_n4, eq45_e832_d_n5, eq45_e832_d_n6, eq45_e832_d_n7, eq45_e832_d_n8, eq45_e832_d_n9, eq45_e832_d_n10, eq45_e832_d_n11, eq45_e832_d_n12, eq45_e832_d_n13, eq45_e832_d_n14, eq45_e832_d_n15, eq45_e832_d_n16, eq45_e832_d_n17, eq45_e832_d_n18, eq45_e832_d_n19, eq45_e832_d_n20, eq45_e832_d_n21, eq45_e832_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq45_value: f64 = eq45_e834;
        let eq45_node_derivatives: [f64; 23] = [eq45_e834_d_n0, eq45_e834_d_n1, eq45_e834_d_n2, eq45_e834_d_n3, eq45_e834_d_n4, eq45_e834_d_n5, eq45_e834_d_n6, eq45_e834_d_n7, eq45_e834_d_n8, eq45_e834_d_n9, eq45_e834_d_n10, eq45_e834_d_n11, eq45_e834_d_n12, eq45_e834_d_n13, eq45_e834_d_n14, eq45_e834_d_n15, eq45_e834_d_n16, eq45_e834_d_n17, eq45_e834_d_n18, eq45_e834_d_n19, eq45_e834_d_n20, eq45_e834_d_n21, eq45_e834_d_n22];
        let eq45_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            None,
            multiplicity * (eq45_value),
            nodes,
            &eq45_node_derivatives,
            branches,
            &eq45_branch_derivatives,
            multiplicity,
        );
        let (eq46_e852, eq46_e852_d_n6,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        let eq46_e849: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, (nv6 - 0.0));
        let eq46_e850: f64 = (p.p144 * eq46_e849);
        let eq46_e850_d_n6: f64 = (p.p144 * ddt_scale);
        (eq46_e850, eq46_e850_d_n6,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e852;
        stamper.stamp_current_node1(
            Some(nodes[6]),
            None,
            multiplicity * (eq46_value),
            nodes[6],
            multiplicity * (eq46_e852_d_n6),
        );
        let (eq47_e867,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq47_value: f64 = eq47_e867;
        stamper.stamp_potential_const(
            branches[25],
            eq47_value,
        );
        let (eq48_e882,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq48_value: f64 = eq48_e882;
        stamper.stamp_potential_const(
            branches[26],
            eq48_value,
        );
        let (eq49_e897,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e897;
        stamper.stamp_potential_const(
            branches[27],
            eq49_value,
        );
        let (eq50_e912,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq50_value: f64 = eq50_e912;
        stamper.stamp_potential_const(
            branches[28],
            eq50_value,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let eq51_e915: f64 = (p.p6 * s.v[41]);
        let eq51_e915_d_n0: f64 = (p.p6 * s.dn[41][0]);
        let eq51_e915_d_n1: f64 = (p.p6 * s.dn[41][1]);
        let eq51_e915_d_n2: f64 = (p.p6 * s.dn[41][2]);
        let eq51_e915_d_n3: f64 = (p.p6 * s.dn[41][3]);
        let eq51_e915_d_n4: f64 = (p.p6 * s.dn[41][4]);
        let eq51_e915_d_n5: f64 = (p.p6 * s.dn[41][5]);
        let eq51_e915_d_n6: f64 = (p.p6 * s.dn[41][6]);
        let eq51_e915_d_n7: f64 = (p.p6 * s.dn[41][7]);
        let eq51_e915_d_n8: f64 = (p.p6 * s.dn[41][8]);
        let eq51_e915_d_n9: f64 = (p.p6 * s.dn[41][9]);
        let eq51_e915_d_n10: f64 = (p.p6 * s.dn[41][10]);
        let eq51_e915_d_n11: f64 = (p.p6 * s.dn[41][11]);
        let eq51_e915_d_n12: f64 = (p.p6 * s.dn[41][12]);
        let eq51_e915_d_n13: f64 = (p.p6 * s.dn[41][13]);
        let eq51_e915_d_n14: f64 = (p.p6 * s.dn[41][14]);
        let eq51_e915_d_n15: f64 = (p.p6 * s.dn[41][15]);
        let eq51_e915_d_n16: f64 = (p.p6 * s.dn[41][16]);
        let eq51_e915_d_n17: f64 = (p.p6 * s.dn[41][17]);
        let eq51_e915_d_n18: f64 = (p.p6 * s.dn[41][18]);
        let eq51_e915_d_n19: f64 = (p.p6 * s.dn[41][19]);
        let eq51_e915_d_n20: f64 = (p.p6 * s.dn[41][20]);
        let eq51_e915_d_n21: f64 = (p.p6 * s.dn[41][21]);
        let eq51_e915_d_n22: f64 = (p.p6 * s.dn[41][22]);
        let eq51_e917: f64 = (eq51_e915 * s.v[94]);
        let eq51_e917_d_n0: f64 = ((eq51_e915_d_n0 * s.v[94]) + (eq51_e915 * s.dn[94][0]));
        let eq51_e917_d_n1: f64 = ((eq51_e915_d_n1 * s.v[94]) + (eq51_e915 * s.dn[94][1]));
        let eq51_e917_d_n2: f64 = ((eq51_e915_d_n2 * s.v[94]) + (eq51_e915 * s.dn[94][2]));
        let eq51_e917_d_n3: f64 = ((eq51_e915_d_n3 * s.v[94]) + (eq51_e915 * s.dn[94][3]));
        let eq51_e917_d_n4: f64 = ((eq51_e915_d_n4 * s.v[94]) + (eq51_e915 * s.dn[94][4]));
        let eq51_e917_d_n5: f64 = ((eq51_e915_d_n5 * s.v[94]) + (eq51_e915 * s.dn[94][5]));
        let eq51_e917_d_n6: f64 = ((eq51_e915_d_n6 * s.v[94]) + (eq51_e915 * s.dn[94][6]));
        let eq51_e917_d_n7: f64 = ((eq51_e915_d_n7 * s.v[94]) + (eq51_e915 * s.dn[94][7]));
        let eq51_e917_d_n8: f64 = ((eq51_e915_d_n8 * s.v[94]) + (eq51_e915 * s.dn[94][8]));
        let eq51_e917_d_n9: f64 = ((eq51_e915_d_n9 * s.v[94]) + (eq51_e915 * s.dn[94][9]));
        let eq51_e917_d_n10: f64 = ((eq51_e915_d_n10 * s.v[94]) + (eq51_e915 * s.dn[94][10]));
        let eq51_e917_d_n11: f64 = ((eq51_e915_d_n11 * s.v[94]) + (eq51_e915 * s.dn[94][11]));
        let eq51_e917_d_n12: f64 = ((eq51_e915_d_n12 * s.v[94]) + (eq51_e915 * s.dn[94][12]));
        let eq51_e917_d_n13: f64 = ((eq51_e915_d_n13 * s.v[94]) + (eq51_e915 * s.dn[94][13]));
        let eq51_e917_d_n14: f64 = ((eq51_e915_d_n14 * s.v[94]) + (eq51_e915 * s.dn[94][14]));
        let eq51_e917_d_n15: f64 = ((eq51_e915_d_n15 * s.v[94]) + (eq51_e915 * s.dn[94][15]));
        let eq51_e917_d_n16: f64 = ((eq51_e915_d_n16 * s.v[94]) + (eq51_e915 * s.dn[94][16]));
        let eq51_e917_d_n17: f64 = ((eq51_e915_d_n17 * s.v[94]) + (eq51_e915 * s.dn[94][17]));
        let eq51_e917_d_n18: f64 = ((eq51_e915_d_n18 * s.v[94]) + (eq51_e915 * s.dn[94][18]));
        let eq51_e917_d_n19: f64 = ((eq51_e915_d_n19 * s.v[94]) + (eq51_e915 * s.dn[94][19]));
        let eq51_e917_d_n20: f64 = ((eq51_e915_d_n20 * s.v[94]) + (eq51_e915 * s.dn[94][20]));
        let eq51_e917_d_n21: f64 = ((eq51_e915_d_n21 * s.v[94]) + (eq51_e915 * s.dn[94][21]));
        let eq51_e917_d_n22: f64 = ((eq51_e915_d_n22 * s.v[94]) + (eq51_e915 * s.dn[94][22]));
        let eq51_e920: f64 = (p.p6 * s.v[379]);
        let eq51_e920_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq51_e920_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq51_e920_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq51_e920_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq51_e920_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq51_e920_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq51_e920_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq51_e920_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq51_e920_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq51_e920_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq51_e920_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq51_e920_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq51_e920_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq51_e920_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq51_e920_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq51_e920_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq51_e920_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq51_e920_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq51_e920_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq51_e920_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq51_e920_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq51_e920_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq51_e920_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq51_e922: f64 = (eq51_e920 * (nv7 - nv8));
        let eq51_e922_d_n0: f64 = (eq51_e920_d_n0 * (nv7 - nv8));
        let eq51_e922_d_n1: f64 = (eq51_e920_d_n1 * (nv7 - nv8));
        let eq51_e922_d_n2: f64 = (eq51_e920_d_n2 * (nv7 - nv8));
        let eq51_e922_d_n3: f64 = (eq51_e920_d_n3 * (nv7 - nv8));
        let eq51_e922_d_n4: f64 = (eq51_e920_d_n4 * (nv7 - nv8));
        let eq51_e922_d_n5: f64 = (eq51_e920_d_n5 * (nv7 - nv8));
        let eq51_e922_d_n6: f64 = (eq51_e920_d_n6 * (nv7 - nv8));
        let eq51_e922_d_n7: f64 = ((eq51_e920_d_n7 * (nv7 - nv8)) + eq51_e920);
        let eq51_e922_d_n8: f64 = ((eq51_e920_d_n8 * (nv7 - nv8)) + (-eq51_e920));
        let eq51_e922_d_n9: f64 = (eq51_e920_d_n9 * (nv7 - nv8));
        let eq51_e922_d_n10: f64 = (eq51_e920_d_n10 * (nv7 - nv8));
        let eq51_e922_d_n11: f64 = (eq51_e920_d_n11 * (nv7 - nv8));
        let eq51_e922_d_n12: f64 = (eq51_e920_d_n12 * (nv7 - nv8));
        let eq51_e922_d_n13: f64 = (eq51_e920_d_n13 * (nv7 - nv8));
        let eq51_e922_d_n14: f64 = (eq51_e920_d_n14 * (nv7 - nv8));
        let eq51_e922_d_n15: f64 = (eq51_e920_d_n15 * (nv7 - nv8));
        let eq51_e922_d_n16: f64 = (eq51_e920_d_n16 * (nv7 - nv8));
        let eq51_e922_d_n17: f64 = (eq51_e920_d_n17 * (nv7 - nv8));
        let eq51_e922_d_n18: f64 = (eq51_e920_d_n18 * (nv7 - nv8));
        let eq51_e922_d_n19: f64 = (eq51_e920_d_n19 * (nv7 - nv8));
        let eq51_e922_d_n20: f64 = (eq51_e920_d_n20 * (nv7 - nv8));
        let eq51_e922_d_n21: f64 = (eq51_e920_d_n21 * (nv7 - nv8));
        let eq51_e922_d_n22: f64 = (eq51_e920_d_n22 * (nv7 - nv8));
        let eq51_e923: f64 = (eq51_e917 + eq51_e922);
        let eq51_e923_d_n0: f64 = (eq51_e917_d_n0 + eq51_e922_d_n0);
        let eq51_e923_d_n1: f64 = (eq51_e917_d_n1 + eq51_e922_d_n1);
        let eq51_e923_d_n2: f64 = (eq51_e917_d_n2 + eq51_e922_d_n2);
        let eq51_e923_d_n3: f64 = (eq51_e917_d_n3 + eq51_e922_d_n3);
        let eq51_e923_d_n4: f64 = (eq51_e917_d_n4 + eq51_e922_d_n4);
        let eq51_e923_d_n5: f64 = (eq51_e917_d_n5 + eq51_e922_d_n5);
        let eq51_e923_d_n6: f64 = (eq51_e917_d_n6 + eq51_e922_d_n6);
        let eq51_e923_d_n7: f64 = (eq51_e917_d_n7 + eq51_e922_d_n7);
        let eq51_e923_d_n8: f64 = (eq51_e917_d_n8 + eq51_e922_d_n8);
        let eq51_e923_d_n9: f64 = (eq51_e917_d_n9 + eq51_e922_d_n9);
        let eq51_e923_d_n10: f64 = (eq51_e917_d_n10 + eq51_e922_d_n10);
        let eq51_e923_d_n11: f64 = (eq51_e917_d_n11 + eq51_e922_d_n11);
        let eq51_e923_d_n12: f64 = (eq51_e917_d_n12 + eq51_e922_d_n12);
        let eq51_e923_d_n13: f64 = (eq51_e917_d_n13 + eq51_e922_d_n13);
        let eq51_e923_d_n14: f64 = (eq51_e917_d_n14 + eq51_e922_d_n14);
        let eq51_e923_d_n15: f64 = (eq51_e917_d_n15 + eq51_e922_d_n15);
        let eq51_e923_d_n16: f64 = (eq51_e917_d_n16 + eq51_e922_d_n16);
        let eq51_e923_d_n17: f64 = (eq51_e917_d_n17 + eq51_e922_d_n17);
        let eq51_e923_d_n18: f64 = (eq51_e917_d_n18 + eq51_e922_d_n18);
        let eq51_e923_d_n19: f64 = (eq51_e917_d_n19 + eq51_e922_d_n19);
        let eq51_e923_d_n20: f64 = (eq51_e917_d_n20 + eq51_e922_d_n20);
        let eq51_e923_d_n21: f64 = (eq51_e917_d_n21 + eq51_e922_d_n21);
        let eq51_e923_d_n22: f64 = (eq51_e917_d_n22 + eq51_e922_d_n22);
        let eq51_value: f64 = eq51_e923;
        let eq51_node_derivatives: [f64; 23] = [eq51_e923_d_n0, eq51_e923_d_n1, eq51_e923_d_n2, eq51_e923_d_n3, eq51_e923_d_n4, eq51_e923_d_n5, eq51_e923_d_n6, eq51_e923_d_n7, eq51_e923_d_n8, eq51_e923_d_n9, eq51_e923_d_n10, eq51_e923_d_n11, eq51_e923_d_n12, eq51_e923_d_n13, eq51_e923_d_n14, eq51_e923_d_n15, eq51_e923_d_n16, eq51_e923_d_n17, eq51_e923_d_n18, eq51_e923_d_n19, eq51_e923_d_n20, eq51_e923_d_n21, eq51_e923_d_n22];
        let eq51_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            multiplicity * (eq51_value),
            nodes,
            &eq51_node_derivatives,
            branches,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let eq52_e926: f64 = (p.p6 * s.v[41]);
        let eq52_e926_d_n0: f64 = (p.p6 * s.dn[41][0]);
        let eq52_e926_d_n1: f64 = (p.p6 * s.dn[41][1]);
        let eq52_e926_d_n2: f64 = (p.p6 * s.dn[41][2]);
        let eq52_e926_d_n3: f64 = (p.p6 * s.dn[41][3]);
        let eq52_e926_d_n4: f64 = (p.p6 * s.dn[41][4]);
        let eq52_e926_d_n5: f64 = (p.p6 * s.dn[41][5]);
        let eq52_e926_d_n6: f64 = (p.p6 * s.dn[41][6]);
        let eq52_e926_d_n7: f64 = (p.p6 * s.dn[41][7]);
        let eq52_e926_d_n8: f64 = (p.p6 * s.dn[41][8]);
        let eq52_e926_d_n9: f64 = (p.p6 * s.dn[41][9]);
        let eq52_e926_d_n10: f64 = (p.p6 * s.dn[41][10]);
        let eq52_e926_d_n11: f64 = (p.p6 * s.dn[41][11]);
        let eq52_e926_d_n12: f64 = (p.p6 * s.dn[41][12]);
        let eq52_e926_d_n13: f64 = (p.p6 * s.dn[41][13]);
        let eq52_e926_d_n14: f64 = (p.p6 * s.dn[41][14]);
        let eq52_e926_d_n15: f64 = (p.p6 * s.dn[41][15]);
        let eq52_e926_d_n16: f64 = (p.p6 * s.dn[41][16]);
        let eq52_e926_d_n17: f64 = (p.p6 * s.dn[41][17]);
        let eq52_e926_d_n18: f64 = (p.p6 * s.dn[41][18]);
        let eq52_e926_d_n19: f64 = (p.p6 * s.dn[41][19]);
        let eq52_e926_d_n20: f64 = (p.p6 * s.dn[41][20]);
        let eq52_e926_d_n21: f64 = (p.p6 * s.dn[41][21]);
        let eq52_e926_d_n22: f64 = (p.p6 * s.dn[41][22]);
        let eq52_e929: f64 = (p.p4 * p.p5);
        let eq52_e931: f64 = (eq52_e929 * s.v[332]);
        let eq52_e931_d_n0: f64 = (eq52_e929 * s.dn[332][0]);
        let eq52_e931_d_n1: f64 = (eq52_e929 * s.dn[332][1]);
        let eq52_e931_d_n2: f64 = (eq52_e929 * s.dn[332][2]);
        let eq52_e931_d_n3: f64 = (eq52_e929 * s.dn[332][3]);
        let eq52_e931_d_n4: f64 = (eq52_e929 * s.dn[332][4]);
        let eq52_e931_d_n5: f64 = (eq52_e929 * s.dn[332][5]);
        let eq52_e931_d_n6: f64 = (eq52_e929 * s.dn[332][6]);
        let eq52_e931_d_n7: f64 = (eq52_e929 * s.dn[332][7]);
        let eq52_e931_d_n8: f64 = (eq52_e929 * s.dn[332][8]);
        let eq52_e931_d_n9: f64 = (eq52_e929 * s.dn[332][9]);
        let eq52_e931_d_n10: f64 = (eq52_e929 * s.dn[332][10]);
        let eq52_e931_d_n11: f64 = (eq52_e929 * s.dn[332][11]);
        let eq52_e931_d_n12: f64 = (eq52_e929 * s.dn[332][12]);
        let eq52_e931_d_n13: f64 = (eq52_e929 * s.dn[332][13]);
        let eq52_e931_d_n14: f64 = (eq52_e929 * s.dn[332][14]);
        let eq52_e931_d_n15: f64 = (eq52_e929 * s.dn[332][15]);
        let eq52_e931_d_n16: f64 = (eq52_e929 * s.dn[332][16]);
        let eq52_e931_d_n17: f64 = (eq52_e929 * s.dn[332][17]);
        let eq52_e931_d_n18: f64 = (eq52_e929 * s.dn[332][18]);
        let eq52_e931_d_n19: f64 = (eq52_e929 * s.dn[332][19]);
        let eq52_e931_d_n20: f64 = (eq52_e929 * s.dn[332][20]);
        let eq52_e931_d_n21: f64 = (eq52_e929 * s.dn[332][21]);
        let eq52_e931_d_n22: f64 = (eq52_e929 * s.dn[332][22]);
        let eq52_e932: f64 = (eq52_e926 * eq52_e931);
        let eq52_e932_d_n0: f64 = ((eq52_e926_d_n0 * eq52_e931) + (eq52_e926 * eq52_e931_d_n0));
        let eq52_e932_d_n1: f64 = ((eq52_e926_d_n1 * eq52_e931) + (eq52_e926 * eq52_e931_d_n1));
        let eq52_e932_d_n2: f64 = ((eq52_e926_d_n2 * eq52_e931) + (eq52_e926 * eq52_e931_d_n2));
        let eq52_e932_d_n3: f64 = ((eq52_e926_d_n3 * eq52_e931) + (eq52_e926 * eq52_e931_d_n3));
        let eq52_e932_d_n4: f64 = ((eq52_e926_d_n4 * eq52_e931) + (eq52_e926 * eq52_e931_d_n4));
        let eq52_e932_d_n5: f64 = ((eq52_e926_d_n5 * eq52_e931) + (eq52_e926 * eq52_e931_d_n5));
        let eq52_e932_d_n6: f64 = ((eq52_e926_d_n6 * eq52_e931) + (eq52_e926 * eq52_e931_d_n6));
        let eq52_e932_d_n7: f64 = ((eq52_e926_d_n7 * eq52_e931) + (eq52_e926 * eq52_e931_d_n7));
        let eq52_e932_d_n8: f64 = ((eq52_e926_d_n8 * eq52_e931) + (eq52_e926 * eq52_e931_d_n8));
        let eq52_e932_d_n9: f64 = ((eq52_e926_d_n9 * eq52_e931) + (eq52_e926 * eq52_e931_d_n9));
        let eq52_e932_d_n10: f64 = ((eq52_e926_d_n10 * eq52_e931) + (eq52_e926 * eq52_e931_d_n10));
        let eq52_e932_d_n11: f64 = ((eq52_e926_d_n11 * eq52_e931) + (eq52_e926 * eq52_e931_d_n11));
        let eq52_e932_d_n12: f64 = ((eq52_e926_d_n12 * eq52_e931) + (eq52_e926 * eq52_e931_d_n12));
        let eq52_e932_d_n13: f64 = ((eq52_e926_d_n13 * eq52_e931) + (eq52_e926 * eq52_e931_d_n13));
        let eq52_e932_d_n14: f64 = ((eq52_e926_d_n14 * eq52_e931) + (eq52_e926 * eq52_e931_d_n14));
        let eq52_e932_d_n15: f64 = ((eq52_e926_d_n15 * eq52_e931) + (eq52_e926 * eq52_e931_d_n15));
        let eq52_e932_d_n16: f64 = ((eq52_e926_d_n16 * eq52_e931) + (eq52_e926 * eq52_e931_d_n16));
        let eq52_e932_d_n17: f64 = ((eq52_e926_d_n17 * eq52_e931) + (eq52_e926 * eq52_e931_d_n17));
        let eq52_e932_d_n18: f64 = ((eq52_e926_d_n18 * eq52_e931) + (eq52_e926 * eq52_e931_d_n18));
        let eq52_e932_d_n19: f64 = ((eq52_e926_d_n19 * eq52_e931) + (eq52_e926 * eq52_e931_d_n19));
        let eq52_e932_d_n20: f64 = ((eq52_e926_d_n20 * eq52_e931) + (eq52_e926 * eq52_e931_d_n20));
        let eq52_e932_d_n21: f64 = ((eq52_e926_d_n21 * eq52_e931) + (eq52_e926 * eq52_e931_d_n21));
        let eq52_e932_d_n22: f64 = ((eq52_e926_d_n22 * eq52_e931) + (eq52_e926 * eq52_e931_d_n22));
        let eq52_value: f64 = eq52_e932;
        let eq52_node_derivatives: [f64; 23] = [eq52_e932_d_n0, eq52_e932_d_n1, eq52_e932_d_n2, eq52_e932_d_n3, eq52_e932_d_n4, eq52_e932_d_n5, eq52_e932_d_n6, eq52_e932_d_n7, eq52_e932_d_n8, eq52_e932_d_n9, eq52_e932_d_n10, eq52_e932_d_n11, eq52_e932_d_n12, eq52_e932_d_n13, eq52_e932_d_n14, eq52_e932_d_n15, eq52_e932_d_n16, eq52_e932_d_n17, eq52_e932_d_n18, eq52_e932_d_n19, eq52_e932_d_n20, eq52_e932_d_n21, eq52_e932_d_n22];
        let eq52_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            multiplicity * (eq52_value),
            nodes,
            &eq52_node_derivatives,
            branches,
            &eq52_branch_derivatives,
            multiplicity,
        );
        let (eq53_e938, eq53_e938_d_n0, eq53_e938_d_n1, eq53_e938_d_n2, eq53_e938_d_n3, eq53_e938_d_n4, eq53_e938_d_n5, eq53_e938_d_n6, eq53_e938_d_n7, eq53_e938_d_n8, eq53_e938_d_n9, eq53_e938_d_n10, eq53_e938_d_n11, eq53_e938_d_n12, eq53_e938_d_n13, eq53_e938_d_n14, eq53_e938_d_n15, eq53_e938_d_n16, eq53_e938_d_n17, eq53_e938_d_n18, eq53_e938_d_n19, eq53_e938_d_n20, eq53_e938_d_n21, eq53_e938_d_n22,) = {
    if s.b[423] {
        let eq53_e936: f64 = (p.p6 * s.v[206]);
        let eq53_e936_d_n0: f64 = (p.p6 * s.dn[206][0]);
        let eq53_e936_d_n1: f64 = (p.p6 * s.dn[206][1]);
        let eq53_e936_d_n2: f64 = (p.p6 * s.dn[206][2]);
        let eq53_e936_d_n3: f64 = (p.p6 * s.dn[206][3]);
        let eq53_e936_d_n4: f64 = (p.p6 * s.dn[206][4]);
        let eq53_e936_d_n5: f64 = (p.p6 * s.dn[206][5]);
        let eq53_e936_d_n6: f64 = (p.p6 * s.dn[206][6]);
        let eq53_e936_d_n7: f64 = (p.p6 * s.dn[206][7]);
        let eq53_e936_d_n8: f64 = (p.p6 * s.dn[206][8]);
        let eq53_e936_d_n9: f64 = (p.p6 * s.dn[206][9]);
        let eq53_e936_d_n10: f64 = (p.p6 * s.dn[206][10]);
        let eq53_e936_d_n11: f64 = (p.p6 * s.dn[206][11]);
        let eq53_e936_d_n12: f64 = (p.p6 * s.dn[206][12]);
        let eq53_e936_d_n13: f64 = (p.p6 * s.dn[206][13]);
        let eq53_e936_d_n14: f64 = (p.p6 * s.dn[206][14]);
        let eq53_e936_d_n15: f64 = (p.p6 * s.dn[206][15]);
        let eq53_e936_d_n16: f64 = (p.p6 * s.dn[206][16]);
        let eq53_e936_d_n17: f64 = (p.p6 * s.dn[206][17]);
        let eq53_e936_d_n18: f64 = (p.p6 * s.dn[206][18]);
        let eq53_e936_d_n19: f64 = (p.p6 * s.dn[206][19]);
        let eq53_e936_d_n20: f64 = (p.p6 * s.dn[206][20]);
        let eq53_e936_d_n21: f64 = (p.p6 * s.dn[206][21]);
        let eq53_e936_d_n22: f64 = (p.p6 * s.dn[206][22]);
        (eq53_e936, eq53_e936_d_n0, eq53_e936_d_n1, eq53_e936_d_n2, eq53_e936_d_n3, eq53_e936_d_n4, eq53_e936_d_n5, eq53_e936_d_n6, eq53_e936_d_n7, eq53_e936_d_n8, eq53_e936_d_n9, eq53_e936_d_n10, eq53_e936_d_n11, eq53_e936_d_n12, eq53_e936_d_n13, eq53_e936_d_n14, eq53_e936_d_n15, eq53_e936_d_n16, eq53_e936_d_n17, eq53_e936_d_n18, eq53_e936_d_n19, eq53_e936_d_n20, eq53_e936_d_n21, eq53_e936_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e938;
        let eq53_node_derivatives: [f64; 23] = [eq53_e938_d_n0, eq53_e938_d_n1, eq53_e938_d_n2, eq53_e938_d_n3, eq53_e938_d_n4, eq53_e938_d_n5, eq53_e938_d_n6, eq53_e938_d_n7, eq53_e938_d_n8, eq53_e938_d_n9, eq53_e938_d_n10, eq53_e938_d_n11, eq53_e938_d_n12, eq53_e938_d_n13, eq53_e938_d_n14, eq53_e938_d_n15, eq53_e938_d_n16, eq53_e938_d_n17, eq53_e938_d_n18, eq53_e938_d_n19, eq53_e938_d_n20, eq53_e938_d_n21, eq53_e938_d_n22];
        let eq53_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            multiplicity * (eq53_value),
            nodes,
            &eq53_node_derivatives,
            branches,
            &eq53_branch_derivatives,
            multiplicity,
        );
        let (eq54_e944, eq54_e944_d_n0, eq54_e944_d_n1, eq54_e944_d_n2, eq54_e944_d_n3, eq54_e944_d_n4, eq54_e944_d_n5, eq54_e944_d_n6, eq54_e944_d_n7, eq54_e944_d_n8, eq54_e944_d_n9, eq54_e944_d_n10, eq54_e944_d_n11, eq54_e944_d_n12, eq54_e944_d_n13, eq54_e944_d_n14, eq54_e944_d_n15, eq54_e944_d_n16, eq54_e944_d_n17, eq54_e944_d_n18, eq54_e944_d_n19, eq54_e944_d_n20, eq54_e944_d_n21, eq54_e944_d_n22,) = {
    if s.b[423] {
        let eq54_e942: f64 = (p.p6 * s.v[207]);
        let eq54_e942_d_n0: f64 = (p.p6 * s.dn[207][0]);
        let eq54_e942_d_n1: f64 = (p.p6 * s.dn[207][1]);
        let eq54_e942_d_n2: f64 = (p.p6 * s.dn[207][2]);
        let eq54_e942_d_n3: f64 = (p.p6 * s.dn[207][3]);
        let eq54_e942_d_n4: f64 = (p.p6 * s.dn[207][4]);
        let eq54_e942_d_n5: f64 = (p.p6 * s.dn[207][5]);
        let eq54_e942_d_n6: f64 = (p.p6 * s.dn[207][6]);
        let eq54_e942_d_n7: f64 = (p.p6 * s.dn[207][7]);
        let eq54_e942_d_n8: f64 = (p.p6 * s.dn[207][8]);
        let eq54_e942_d_n9: f64 = (p.p6 * s.dn[207][9]);
        let eq54_e942_d_n10: f64 = (p.p6 * s.dn[207][10]);
        let eq54_e942_d_n11: f64 = (p.p6 * s.dn[207][11]);
        let eq54_e942_d_n12: f64 = (p.p6 * s.dn[207][12]);
        let eq54_e942_d_n13: f64 = (p.p6 * s.dn[207][13]);
        let eq54_e942_d_n14: f64 = (p.p6 * s.dn[207][14]);
        let eq54_e942_d_n15: f64 = (p.p6 * s.dn[207][15]);
        let eq54_e942_d_n16: f64 = (p.p6 * s.dn[207][16]);
        let eq54_e942_d_n17: f64 = (p.p6 * s.dn[207][17]);
        let eq54_e942_d_n18: f64 = (p.p6 * s.dn[207][18]);
        let eq54_e942_d_n19: f64 = (p.p6 * s.dn[207][19]);
        let eq54_e942_d_n20: f64 = (p.p6 * s.dn[207][20]);
        let eq54_e942_d_n21: f64 = (p.p6 * s.dn[207][21]);
        let eq54_e942_d_n22: f64 = (p.p6 * s.dn[207][22]);
        (eq54_e942, eq54_e942_d_n0, eq54_e942_d_n1, eq54_e942_d_n2, eq54_e942_d_n3, eq54_e942_d_n4, eq54_e942_d_n5, eq54_e942_d_n6, eq54_e942_d_n7, eq54_e942_d_n8, eq54_e942_d_n9, eq54_e942_d_n10, eq54_e942_d_n11, eq54_e942_d_n12, eq54_e942_d_n13, eq54_e942_d_n14, eq54_e942_d_n15, eq54_e942_d_n16, eq54_e942_d_n17, eq54_e942_d_n18, eq54_e942_d_n19, eq54_e942_d_n20, eq54_e942_d_n21, eq54_e942_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e944;
        let eq54_node_derivatives: [f64; 23] = [eq54_e944_d_n0, eq54_e944_d_n1, eq54_e944_d_n2, eq54_e944_d_n3, eq54_e944_d_n4, eq54_e944_d_n5, eq54_e944_d_n6, eq54_e944_d_n7, eq54_e944_d_n8, eq54_e944_d_n9, eq54_e944_d_n10, eq54_e944_d_n11, eq54_e944_d_n12, eq54_e944_d_n13, eq54_e944_d_n14, eq54_e944_d_n15, eq54_e944_d_n16, eq54_e944_d_n17, eq54_e944_d_n18, eq54_e944_d_n19, eq54_e944_d_n20, eq54_e944_d_n21, eq54_e944_d_n22];
        let eq54_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            multiplicity * (eq54_value),
            nodes,
            &eq54_node_derivatives,
            branches,
            &eq54_branch_derivatives,
            multiplicity,
        );
        let (eq55_e957, eq55_e957_d_n0, eq55_e957_d_n1, eq55_e957_d_n2, eq55_e957_d_n3, eq55_e957_d_n4, eq55_e957_d_n5, eq55_e957_d_n6, eq55_e957_d_n7, eq55_e957_d_n8, eq55_e957_d_n9, eq55_e957_d_n10, eq55_e957_d_n11, eq55_e957_d_n12, eq55_e957_d_n13, eq55_e957_d_n14, eq55_e957_d_n15, eq55_e957_d_n16, eq55_e957_d_n17, eq55_e957_d_n18, eq55_e957_d_n19, eq55_e957_d_n20, eq55_e957_d_n21, eq55_e957_d_n22,) = {
    if (!s.b[423]) {
        let eq55_e951: f64 = 0.0;
        let eq55_e953: f64 = (eq55_e951 * (nv9 - nv8));
        let eq55_e953_d_n8: f64 = (-eq55_e951);
        let eq55_e954: f64 = (s.v[206] + eq55_e953);
        let eq55_e954_d_n8: f64 = (s.dn[206][8] + eq55_e953_d_n8);
        let eq55_e954_d_n9: f64 = (s.dn[206][9] + eq55_e951);
        let eq55_e955: f64 = (p.p6 * eq55_e954);
        let eq55_e955_d_n0: f64 = (p.p6 * s.dn[206][0]);
        let eq55_e955_d_n1: f64 = (p.p6 * s.dn[206][1]);
        let eq55_e955_d_n2: f64 = (p.p6 * s.dn[206][2]);
        let eq55_e955_d_n3: f64 = (p.p6 * s.dn[206][3]);
        let eq55_e955_d_n4: f64 = (p.p6 * s.dn[206][4]);
        let eq55_e955_d_n5: f64 = (p.p6 * s.dn[206][5]);
        let eq55_e955_d_n6: f64 = (p.p6 * s.dn[206][6]);
        let eq55_e955_d_n7: f64 = (p.p6 * s.dn[206][7]);
        let eq55_e955_d_n8: f64 = (p.p6 * eq55_e954_d_n8);
        let eq55_e955_d_n9: f64 = (p.p6 * eq55_e954_d_n9);
        let eq55_e955_d_n10: f64 = (p.p6 * s.dn[206][10]);
        let eq55_e955_d_n11: f64 = (p.p6 * s.dn[206][11]);
        let eq55_e955_d_n12: f64 = (p.p6 * s.dn[206][12]);
        let eq55_e955_d_n13: f64 = (p.p6 * s.dn[206][13]);
        let eq55_e955_d_n14: f64 = (p.p6 * s.dn[206][14]);
        let eq55_e955_d_n15: f64 = (p.p6 * s.dn[206][15]);
        let eq55_e955_d_n16: f64 = (p.p6 * s.dn[206][16]);
        let eq55_e955_d_n17: f64 = (p.p6 * s.dn[206][17]);
        let eq55_e955_d_n18: f64 = (p.p6 * s.dn[206][18]);
        let eq55_e955_d_n19: f64 = (p.p6 * s.dn[206][19]);
        let eq55_e955_d_n20: f64 = (p.p6 * s.dn[206][20]);
        let eq55_e955_d_n21: f64 = (p.p6 * s.dn[206][21]);
        let eq55_e955_d_n22: f64 = (p.p6 * s.dn[206][22]);
        (eq55_e955, eq55_e955_d_n0, eq55_e955_d_n1, eq55_e955_d_n2, eq55_e955_d_n3, eq55_e955_d_n4, eq55_e955_d_n5, eq55_e955_d_n6, eq55_e955_d_n7, eq55_e955_d_n8, eq55_e955_d_n9, eq55_e955_d_n10, eq55_e955_d_n11, eq55_e955_d_n12, eq55_e955_d_n13, eq55_e955_d_n14, eq55_e955_d_n15, eq55_e955_d_n16, eq55_e955_d_n17, eq55_e955_d_n18, eq55_e955_d_n19, eq55_e955_d_n20, eq55_e955_d_n21, eq55_e955_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e957;
        let eq55_node_derivatives: [f64; 23] = [eq55_e957_d_n0, eq55_e957_d_n1, eq55_e957_d_n2, eq55_e957_d_n3, eq55_e957_d_n4, eq55_e957_d_n5, eq55_e957_d_n6, eq55_e957_d_n7, eq55_e957_d_n8, eq55_e957_d_n9, eq55_e957_d_n10, eq55_e957_d_n11, eq55_e957_d_n12, eq55_e957_d_n13, eq55_e957_d_n14, eq55_e957_d_n15, eq55_e957_d_n16, eq55_e957_d_n17, eq55_e957_d_n18, eq55_e957_d_n19, eq55_e957_d_n20, eq55_e957_d_n21, eq55_e957_d_n22];
        let eq55_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            multiplicity * (eq55_value),
            nodes,
            &eq55_node_derivatives,
            branches,
            &eq55_branch_derivatives,
            multiplicity,
        );
        let (eq56_e970, eq56_e970_d_n0, eq56_e970_d_n1, eq56_e970_d_n2, eq56_e970_d_n3, eq56_e970_d_n4, eq56_e970_d_n5, eq56_e970_d_n6, eq56_e970_d_n7, eq56_e970_d_n8, eq56_e970_d_n9, eq56_e970_d_n10, eq56_e970_d_n11, eq56_e970_d_n12, eq56_e970_d_n13, eq56_e970_d_n14, eq56_e970_d_n15, eq56_e970_d_n16, eq56_e970_d_n17, eq56_e970_d_n18, eq56_e970_d_n19, eq56_e970_d_n20, eq56_e970_d_n21, eq56_e970_d_n22,) = {
    if (!s.b[423]) {
        let eq56_e964: f64 = 0.0;
        let eq56_e966: f64 = (eq56_e964 * (nv9 - nv7));
        let eq56_e966_d_n7: f64 = (-eq56_e964);
        let eq56_e967: f64 = (s.v[207] + eq56_e966);
        let eq56_e967_d_n7: f64 = (s.dn[207][7] + eq56_e966_d_n7);
        let eq56_e967_d_n9: f64 = (s.dn[207][9] + eq56_e964);
        let eq56_e968: f64 = (p.p6 * eq56_e967);
        let eq56_e968_d_n0: f64 = (p.p6 * s.dn[207][0]);
        let eq56_e968_d_n1: f64 = (p.p6 * s.dn[207][1]);
        let eq56_e968_d_n2: f64 = (p.p6 * s.dn[207][2]);
        let eq56_e968_d_n3: f64 = (p.p6 * s.dn[207][3]);
        let eq56_e968_d_n4: f64 = (p.p6 * s.dn[207][4]);
        let eq56_e968_d_n5: f64 = (p.p6 * s.dn[207][5]);
        let eq56_e968_d_n6: f64 = (p.p6 * s.dn[207][6]);
        let eq56_e968_d_n7: f64 = (p.p6 * eq56_e967_d_n7);
        let eq56_e968_d_n8: f64 = (p.p6 * s.dn[207][8]);
        let eq56_e968_d_n9: f64 = (p.p6 * eq56_e967_d_n9);
        let eq56_e968_d_n10: f64 = (p.p6 * s.dn[207][10]);
        let eq56_e968_d_n11: f64 = (p.p6 * s.dn[207][11]);
        let eq56_e968_d_n12: f64 = (p.p6 * s.dn[207][12]);
        let eq56_e968_d_n13: f64 = (p.p6 * s.dn[207][13]);
        let eq56_e968_d_n14: f64 = (p.p6 * s.dn[207][14]);
        let eq56_e968_d_n15: f64 = (p.p6 * s.dn[207][15]);
        let eq56_e968_d_n16: f64 = (p.p6 * s.dn[207][16]);
        let eq56_e968_d_n17: f64 = (p.p6 * s.dn[207][17]);
        let eq56_e968_d_n18: f64 = (p.p6 * s.dn[207][18]);
        let eq56_e968_d_n19: f64 = (p.p6 * s.dn[207][19]);
        let eq56_e968_d_n20: f64 = (p.p6 * s.dn[207][20]);
        let eq56_e968_d_n21: f64 = (p.p6 * s.dn[207][21]);
        let eq56_e968_d_n22: f64 = (p.p6 * s.dn[207][22]);
        (eq56_e968, eq56_e968_d_n0, eq56_e968_d_n1, eq56_e968_d_n2, eq56_e968_d_n3, eq56_e968_d_n4, eq56_e968_d_n5, eq56_e968_d_n6, eq56_e968_d_n7, eq56_e968_d_n8, eq56_e968_d_n9, eq56_e968_d_n10, eq56_e968_d_n11, eq56_e968_d_n12, eq56_e968_d_n13, eq56_e968_d_n14, eq56_e968_d_n15, eq56_e968_d_n16, eq56_e968_d_n17, eq56_e968_d_n18, eq56_e968_d_n19, eq56_e968_d_n20, eq56_e968_d_n21, eq56_e968_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_value: f64 = eq56_e970;
        let eq56_node_derivatives: [f64; 23] = [eq56_e970_d_n0, eq56_e970_d_n1, eq56_e970_d_n2, eq56_e970_d_n3, eq56_e970_d_n4, eq56_e970_d_n5, eq56_e970_d_n6, eq56_e970_d_n7, eq56_e970_d_n8, eq56_e970_d_n9, eq56_e970_d_n10, eq56_e970_d_n11, eq56_e970_d_n12, eq56_e970_d_n13, eq56_e970_d_n14, eq56_e970_d_n15, eq56_e970_d_n16, eq56_e970_d_n17, eq56_e970_d_n18, eq56_e970_d_n19, eq56_e970_d_n20, eq56_e970_d_n21, eq56_e970_d_n22];
        let eq56_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            multiplicity * (eq56_value),
            nodes,
            &eq56_node_derivatives,
            branches,
            &eq56_branch_derivatives,
            multiplicity,
        );
        let (eq57_e980, eq57_e980_d_n0, eq57_e980_d_n1, eq57_e980_d_n2, eq57_e980_d_n3, eq57_e980_d_n4, eq57_e980_d_n5, eq57_e980_d_n6, eq57_e980_d_n7, eq57_e980_d_n8, eq57_e980_d_n9, eq57_e980_d_n10, eq57_e980_d_n11, eq57_e980_d_n12, eq57_e980_d_n13, eq57_e980_d_n14, eq57_e980_d_n15, eq57_e980_d_n16, eq57_e980_d_n17, eq57_e980_d_n18, eq57_e980_d_n19, eq57_e980_d_n20, eq57_e980_d_n21, eq57_e980_d_n22,) = {
    if (s.b[424] && s.b[427]) {
        let eq57_e976: f64 = (p.p6 * s.v[142]);
        let eq57_e976_d_n0: f64 = (p.p6 * s.dn[142][0]);
        let eq57_e976_d_n1: f64 = (p.p6 * s.dn[142][1]);
        let eq57_e976_d_n2: f64 = (p.p6 * s.dn[142][2]);
        let eq57_e976_d_n3: f64 = (p.p6 * s.dn[142][3]);
        let eq57_e976_d_n4: f64 = (p.p6 * s.dn[142][4]);
        let eq57_e976_d_n5: f64 = (p.p6 * s.dn[142][5]);
        let eq57_e976_d_n6: f64 = (p.p6 * s.dn[142][6]);
        let eq57_e976_d_n7: f64 = (p.p6 * s.dn[142][7]);
        let eq57_e976_d_n8: f64 = (p.p6 * s.dn[142][8]);
        let eq57_e976_d_n9: f64 = (p.p6 * s.dn[142][9]);
        let eq57_e976_d_n10: f64 = (p.p6 * s.dn[142][10]);
        let eq57_e976_d_n11: f64 = (p.p6 * s.dn[142][11]);
        let eq57_e976_d_n12: f64 = (p.p6 * s.dn[142][12]);
        let eq57_e976_d_n13: f64 = (p.p6 * s.dn[142][13]);
        let eq57_e976_d_n14: f64 = (p.p6 * s.dn[142][14]);
        let eq57_e976_d_n15: f64 = (p.p6 * s.dn[142][15]);
        let eq57_e976_d_n16: f64 = (p.p6 * s.dn[142][16]);
        let eq57_e976_d_n17: f64 = (p.p6 * s.dn[142][17]);
        let eq57_e976_d_n18: f64 = (p.p6 * s.dn[142][18]);
        let eq57_e976_d_n19: f64 = (p.p6 * s.dn[142][19]);
        let eq57_e976_d_n20: f64 = (p.p6 * s.dn[142][20]);
        let eq57_e976_d_n21: f64 = (p.p6 * s.dn[142][21]);
        let eq57_e976_d_n22: f64 = (p.p6 * s.dn[142][22]);
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
        let eq57_e978_d_n10: f64 = (eq57_e976_d_n10 * (nv0 - nv18));
        let eq57_e978_d_n11: f64 = (eq57_e976_d_n11 * (nv0 - nv18));
        let eq57_e978_d_n12: f64 = (eq57_e976_d_n12 * (nv0 - nv18));
        let eq57_e978_d_n13: f64 = (eq57_e976_d_n13 * (nv0 - nv18));
        let eq57_e978_d_n14: f64 = (eq57_e976_d_n14 * (nv0 - nv18));
        let eq57_e978_d_n15: f64 = (eq57_e976_d_n15 * (nv0 - nv18));
        let eq57_e978_d_n16: f64 = (eq57_e976_d_n16 * (nv0 - nv18));
        let eq57_e978_d_n17: f64 = (eq57_e976_d_n17 * (nv0 - nv18));
        let eq57_e978_d_n18: f64 = ((eq57_e976_d_n18 * (nv0 - nv18)) + (-eq57_e976));
        let eq57_e978_d_n19: f64 = (eq57_e976_d_n19 * (nv0 - nv18));
        let eq57_e978_d_n20: f64 = (eq57_e976_d_n20 * (nv0 - nv18));
        let eq57_e978_d_n21: f64 = (eq57_e976_d_n21 * (nv0 - nv18));
        let eq57_e978_d_n22: f64 = (eq57_e976_d_n22 * (nv0 - nv18));
        (eq57_e978, eq57_e978_d_n0, eq57_e978_d_n1, eq57_e978_d_n2, eq57_e978_d_n3, eq57_e978_d_n4, eq57_e978_d_n5, eq57_e978_d_n6, eq57_e978_d_n7, eq57_e978_d_n8, eq57_e978_d_n9, eq57_e978_d_n10, eq57_e978_d_n11, eq57_e978_d_n12, eq57_e978_d_n13, eq57_e978_d_n14, eq57_e978_d_n15, eq57_e978_d_n16, eq57_e978_d_n17, eq57_e978_d_n18, eq57_e978_d_n19, eq57_e978_d_n20, eq57_e978_d_n21, eq57_e978_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e980;
        let eq57_node_derivatives: [f64; 23] = [eq57_e980_d_n0, eq57_e980_d_n1, eq57_e980_d_n2, eq57_e980_d_n3, eq57_e980_d_n4, eq57_e980_d_n5, eq57_e980_d_n6, eq57_e980_d_n7, eq57_e980_d_n8, eq57_e980_d_n9, eq57_e980_d_n10, eq57_e980_d_n11, eq57_e980_d_n12, eq57_e980_d_n13, eq57_e980_d_n14, eq57_e980_d_n15, eq57_e980_d_n16, eq57_e980_d_n17, eq57_e980_d_n18, eq57_e980_d_n19, eq57_e980_d_n20, eq57_e980_d_n21, eq57_e980_d_n22];
        let eq57_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[18]),
            multiplicity * (eq57_value),
            nodes,
            &eq57_node_derivatives,
            branches,
            &eq57_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv22 = ctx.node_voltage(nodes[22]);
        let (eq58_e990, eq58_e990_d_n0, eq58_e990_d_n1, eq58_e990_d_n2, eq58_e990_d_n3, eq58_e990_d_n4, eq58_e990_d_n5, eq58_e990_d_n6, eq58_e990_d_n7, eq58_e990_d_n8, eq58_e990_d_n9, eq58_e990_d_n10, eq58_e990_d_n11, eq58_e990_d_n12, eq58_e990_d_n13, eq58_e990_d_n14, eq58_e990_d_n15, eq58_e990_d_n16, eq58_e990_d_n17, eq58_e990_d_n18, eq58_e990_d_n19, eq58_e990_d_n20, eq58_e990_d_n21, eq58_e990_d_n22,) = {
    if (s.b[424] && s.b[427]) {
        let eq58_e986: f64 = (p.p6 * s.v[143]);
        let eq58_e986_d_n0: f64 = (p.p6 * s.dn[143][0]);
        let eq58_e986_d_n1: f64 = (p.p6 * s.dn[143][1]);
        let eq58_e986_d_n2: f64 = (p.p6 * s.dn[143][2]);
        let eq58_e986_d_n3: f64 = (p.p6 * s.dn[143][3]);
        let eq58_e986_d_n4: f64 = (p.p6 * s.dn[143][4]);
        let eq58_e986_d_n5: f64 = (p.p6 * s.dn[143][5]);
        let eq58_e986_d_n6: f64 = (p.p6 * s.dn[143][6]);
        let eq58_e986_d_n7: f64 = (p.p6 * s.dn[143][7]);
        let eq58_e986_d_n8: f64 = (p.p6 * s.dn[143][8]);
        let eq58_e986_d_n9: f64 = (p.p6 * s.dn[143][9]);
        let eq58_e986_d_n10: f64 = (p.p6 * s.dn[143][10]);
        let eq58_e986_d_n11: f64 = (p.p6 * s.dn[143][11]);
        let eq58_e986_d_n12: f64 = (p.p6 * s.dn[143][12]);
        let eq58_e986_d_n13: f64 = (p.p6 * s.dn[143][13]);
        let eq58_e986_d_n14: f64 = (p.p6 * s.dn[143][14]);
        let eq58_e986_d_n15: f64 = (p.p6 * s.dn[143][15]);
        let eq58_e986_d_n16: f64 = (p.p6 * s.dn[143][16]);
        let eq58_e986_d_n17: f64 = (p.p6 * s.dn[143][17]);
        let eq58_e986_d_n18: f64 = (p.p6 * s.dn[143][18]);
        let eq58_e986_d_n19: f64 = (p.p6 * s.dn[143][19]);
        let eq58_e986_d_n20: f64 = (p.p6 * s.dn[143][20]);
        let eq58_e986_d_n21: f64 = (p.p6 * s.dn[143][21]);
        let eq58_e986_d_n22: f64 = (p.p6 * s.dn[143][22]);
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
        let eq58_e988_d_n10: f64 = (eq58_e986_d_n10 * (nv22 - nv2));
        let eq58_e988_d_n11: f64 = (eq58_e986_d_n11 * (nv22 - nv2));
        let eq58_e988_d_n12: f64 = (eq58_e986_d_n12 * (nv22 - nv2));
        let eq58_e988_d_n13: f64 = (eq58_e986_d_n13 * (nv22 - nv2));
        let eq58_e988_d_n14: f64 = (eq58_e986_d_n14 * (nv22 - nv2));
        let eq58_e988_d_n15: f64 = (eq58_e986_d_n15 * (nv22 - nv2));
        let eq58_e988_d_n16: f64 = (eq58_e986_d_n16 * (nv22 - nv2));
        let eq58_e988_d_n17: f64 = (eq58_e986_d_n17 * (nv22 - nv2));
        let eq58_e988_d_n18: f64 = (eq58_e986_d_n18 * (nv22 - nv2));
        let eq58_e988_d_n19: f64 = (eq58_e986_d_n19 * (nv22 - nv2));
        let eq58_e988_d_n20: f64 = (eq58_e986_d_n20 * (nv22 - nv2));
        let eq58_e988_d_n21: f64 = (eq58_e986_d_n21 * (nv22 - nv2));
        let eq58_e988_d_n22: f64 = ((eq58_e986_d_n22 * (nv22 - nv2)) + eq58_e986);
        (eq58_e988, eq58_e988_d_n0, eq58_e988_d_n1, eq58_e988_d_n2, eq58_e988_d_n3, eq58_e988_d_n4, eq58_e988_d_n5, eq58_e988_d_n6, eq58_e988_d_n7, eq58_e988_d_n8, eq58_e988_d_n9, eq58_e988_d_n10, eq58_e988_d_n11, eq58_e988_d_n12, eq58_e988_d_n13, eq58_e988_d_n14, eq58_e988_d_n15, eq58_e988_d_n16, eq58_e988_d_n17, eq58_e988_d_n18, eq58_e988_d_n19, eq58_e988_d_n20, eq58_e988_d_n21, eq58_e988_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e990;
        let eq58_node_derivatives: [f64; 23] = [eq58_e990_d_n0, eq58_e990_d_n1, eq58_e990_d_n2, eq58_e990_d_n3, eq58_e990_d_n4, eq58_e990_d_n5, eq58_e990_d_n6, eq58_e990_d_n7, eq58_e990_d_n8, eq58_e990_d_n9, eq58_e990_d_n10, eq58_e990_d_n11, eq58_e990_d_n12, eq58_e990_d_n13, eq58_e990_d_n14, eq58_e990_d_n15, eq58_e990_d_n16, eq58_e990_d_n17, eq58_e990_d_n18, eq58_e990_d_n19, eq58_e990_d_n20, eq58_e990_d_n21, eq58_e990_d_n22];
        let eq58_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[22]),
            Some(nodes[2]),
            multiplicity * (eq58_value),
            nodes,
            &eq58_node_derivatives,
            branches,
            &eq58_branch_derivatives,
            multiplicity,
        );
        let (eq59_e1001, eq59_e1001_d_n0, eq59_e1001_d_n1, eq59_e1001_d_n2, eq59_e1001_d_n3, eq59_e1001_d_n4, eq59_e1001_d_n5, eq59_e1001_d_n6, eq59_e1001_d_n7, eq59_e1001_d_n8, eq59_e1001_d_n9, eq59_e1001_d_n10, eq59_e1001_d_n11, eq59_e1001_d_n12, eq59_e1001_d_n13, eq59_e1001_d_n14, eq59_e1001_d_n15, eq59_e1001_d_n16, eq59_e1001_d_n17, eq59_e1001_d_n18, eq59_e1001_d_n19, eq59_e1001_d_n20, eq59_e1001_d_n21, eq59_e1001_d_n22,) = {
    if (s.b[424] && (!s.b[427])) {
        let eq59_e997: f64 = (p.p6 * s.v[142]);
        let eq59_e997_d_n0: f64 = (p.p6 * s.dn[142][0]);
        let eq59_e997_d_n1: f64 = (p.p6 * s.dn[142][1]);
        let eq59_e997_d_n2: f64 = (p.p6 * s.dn[142][2]);
        let eq59_e997_d_n3: f64 = (p.p6 * s.dn[142][3]);
        let eq59_e997_d_n4: f64 = (p.p6 * s.dn[142][4]);
        let eq59_e997_d_n5: f64 = (p.p6 * s.dn[142][5]);
        let eq59_e997_d_n6: f64 = (p.p6 * s.dn[142][6]);
        let eq59_e997_d_n7: f64 = (p.p6 * s.dn[142][7]);
        let eq59_e997_d_n8: f64 = (p.p6 * s.dn[142][8]);
        let eq59_e997_d_n9: f64 = (p.p6 * s.dn[142][9]);
        let eq59_e997_d_n10: f64 = (p.p6 * s.dn[142][10]);
        let eq59_e997_d_n11: f64 = (p.p6 * s.dn[142][11]);
        let eq59_e997_d_n12: f64 = (p.p6 * s.dn[142][12]);
        let eq59_e997_d_n13: f64 = (p.p6 * s.dn[142][13]);
        let eq59_e997_d_n14: f64 = (p.p6 * s.dn[142][14]);
        let eq59_e997_d_n15: f64 = (p.p6 * s.dn[142][15]);
        let eq59_e997_d_n16: f64 = (p.p6 * s.dn[142][16]);
        let eq59_e997_d_n17: f64 = (p.p6 * s.dn[142][17]);
        let eq59_e997_d_n18: f64 = (p.p6 * s.dn[142][18]);
        let eq59_e997_d_n19: f64 = (p.p6 * s.dn[142][19]);
        let eq59_e997_d_n20: f64 = (p.p6 * s.dn[142][20]);
        let eq59_e997_d_n21: f64 = (p.p6 * s.dn[142][21]);
        let eq59_e997_d_n22: f64 = (p.p6 * s.dn[142][22]);
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
        let eq59_e999_d_n10: f64 = (eq59_e997_d_n10 * (nv0 - nv7));
        let eq59_e999_d_n11: f64 = (eq59_e997_d_n11 * (nv0 - nv7));
        let eq59_e999_d_n12: f64 = (eq59_e997_d_n12 * (nv0 - nv7));
        let eq59_e999_d_n13: f64 = (eq59_e997_d_n13 * (nv0 - nv7));
        let eq59_e999_d_n14: f64 = (eq59_e997_d_n14 * (nv0 - nv7));
        let eq59_e999_d_n15: f64 = (eq59_e997_d_n15 * (nv0 - nv7));
        let eq59_e999_d_n16: f64 = (eq59_e997_d_n16 * (nv0 - nv7));
        let eq59_e999_d_n17: f64 = (eq59_e997_d_n17 * (nv0 - nv7));
        let eq59_e999_d_n18: f64 = (eq59_e997_d_n18 * (nv0 - nv7));
        let eq59_e999_d_n19: f64 = (eq59_e997_d_n19 * (nv0 - nv7));
        let eq59_e999_d_n20: f64 = (eq59_e997_d_n20 * (nv0 - nv7));
        let eq59_e999_d_n21: f64 = (eq59_e997_d_n21 * (nv0 - nv7));
        let eq59_e999_d_n22: f64 = (eq59_e997_d_n22 * (nv0 - nv7));
        (eq59_e999, eq59_e999_d_n0, eq59_e999_d_n1, eq59_e999_d_n2, eq59_e999_d_n3, eq59_e999_d_n4, eq59_e999_d_n5, eq59_e999_d_n6, eq59_e999_d_n7, eq59_e999_d_n8, eq59_e999_d_n9, eq59_e999_d_n10, eq59_e999_d_n11, eq59_e999_d_n12, eq59_e999_d_n13, eq59_e999_d_n14, eq59_e999_d_n15, eq59_e999_d_n16, eq59_e999_d_n17, eq59_e999_d_n18, eq59_e999_d_n19, eq59_e999_d_n20, eq59_e999_d_n21, eq59_e999_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e1001;
        let eq59_node_derivatives: [f64; 23] = [eq59_e1001_d_n0, eq59_e1001_d_n1, eq59_e1001_d_n2, eq59_e1001_d_n3, eq59_e1001_d_n4, eq59_e1001_d_n5, eq59_e1001_d_n6, eq59_e1001_d_n7, eq59_e1001_d_n8, eq59_e1001_d_n9, eq59_e1001_d_n10, eq59_e1001_d_n11, eq59_e1001_d_n12, eq59_e1001_d_n13, eq59_e1001_d_n14, eq59_e1001_d_n15, eq59_e1001_d_n16, eq59_e1001_d_n17, eq59_e1001_d_n18, eq59_e1001_d_n19, eq59_e1001_d_n20, eq59_e1001_d_n21, eq59_e1001_d_n22];
        let eq59_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            multiplicity * (eq59_value),
            nodes,
            &eq59_node_derivatives,
            branches,
            &eq59_branch_derivatives,
            multiplicity,
        );
        let (eq60_e1012, eq60_e1012_d_n0, eq60_e1012_d_n1, eq60_e1012_d_n2, eq60_e1012_d_n3, eq60_e1012_d_n4, eq60_e1012_d_n5, eq60_e1012_d_n6, eq60_e1012_d_n7, eq60_e1012_d_n8, eq60_e1012_d_n9, eq60_e1012_d_n10, eq60_e1012_d_n11, eq60_e1012_d_n12, eq60_e1012_d_n13, eq60_e1012_d_n14, eq60_e1012_d_n15, eq60_e1012_d_n16, eq60_e1012_d_n17, eq60_e1012_d_n18, eq60_e1012_d_n19, eq60_e1012_d_n20, eq60_e1012_d_n21, eq60_e1012_d_n22,) = {
    if (s.b[424] && (!s.b[427])) {
        let eq60_e1008: f64 = (p.p6 * s.v[143]);
        let eq60_e1008_d_n0: f64 = (p.p6 * s.dn[143][0]);
        let eq60_e1008_d_n1: f64 = (p.p6 * s.dn[143][1]);
        let eq60_e1008_d_n2: f64 = (p.p6 * s.dn[143][2]);
        let eq60_e1008_d_n3: f64 = (p.p6 * s.dn[143][3]);
        let eq60_e1008_d_n4: f64 = (p.p6 * s.dn[143][4]);
        let eq60_e1008_d_n5: f64 = (p.p6 * s.dn[143][5]);
        let eq60_e1008_d_n6: f64 = (p.p6 * s.dn[143][6]);
        let eq60_e1008_d_n7: f64 = (p.p6 * s.dn[143][7]);
        let eq60_e1008_d_n8: f64 = (p.p6 * s.dn[143][8]);
        let eq60_e1008_d_n9: f64 = (p.p6 * s.dn[143][9]);
        let eq60_e1008_d_n10: f64 = (p.p6 * s.dn[143][10]);
        let eq60_e1008_d_n11: f64 = (p.p6 * s.dn[143][11]);
        let eq60_e1008_d_n12: f64 = (p.p6 * s.dn[143][12]);
        let eq60_e1008_d_n13: f64 = (p.p6 * s.dn[143][13]);
        let eq60_e1008_d_n14: f64 = (p.p6 * s.dn[143][14]);
        let eq60_e1008_d_n15: f64 = (p.p6 * s.dn[143][15]);
        let eq60_e1008_d_n16: f64 = (p.p6 * s.dn[143][16]);
        let eq60_e1008_d_n17: f64 = (p.p6 * s.dn[143][17]);
        let eq60_e1008_d_n18: f64 = (p.p6 * s.dn[143][18]);
        let eq60_e1008_d_n19: f64 = (p.p6 * s.dn[143][19]);
        let eq60_e1008_d_n20: f64 = (p.p6 * s.dn[143][20]);
        let eq60_e1008_d_n21: f64 = (p.p6 * s.dn[143][21]);
        let eq60_e1008_d_n22: f64 = (p.p6 * s.dn[143][22]);
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
        let eq60_e1010_d_n10: f64 = (eq60_e1008_d_n10 * (nv8 - nv2));
        let eq60_e1010_d_n11: f64 = (eq60_e1008_d_n11 * (nv8 - nv2));
        let eq60_e1010_d_n12: f64 = (eq60_e1008_d_n12 * (nv8 - nv2));
        let eq60_e1010_d_n13: f64 = (eq60_e1008_d_n13 * (nv8 - nv2));
        let eq60_e1010_d_n14: f64 = (eq60_e1008_d_n14 * (nv8 - nv2));
        let eq60_e1010_d_n15: f64 = (eq60_e1008_d_n15 * (nv8 - nv2));
        let eq60_e1010_d_n16: f64 = (eq60_e1008_d_n16 * (nv8 - nv2));
        let eq60_e1010_d_n17: f64 = (eq60_e1008_d_n17 * (nv8 - nv2));
        let eq60_e1010_d_n18: f64 = (eq60_e1008_d_n18 * (nv8 - nv2));
        let eq60_e1010_d_n19: f64 = (eq60_e1008_d_n19 * (nv8 - nv2));
        let eq60_e1010_d_n20: f64 = (eq60_e1008_d_n20 * (nv8 - nv2));
        let eq60_e1010_d_n21: f64 = (eq60_e1008_d_n21 * (nv8 - nv2));
        let eq60_e1010_d_n22: f64 = (eq60_e1008_d_n22 * (nv8 - nv2));
        (eq60_e1010, eq60_e1010_d_n0, eq60_e1010_d_n1, eq60_e1010_d_n2, eq60_e1010_d_n3, eq60_e1010_d_n4, eq60_e1010_d_n5, eq60_e1010_d_n6, eq60_e1010_d_n7, eq60_e1010_d_n8, eq60_e1010_d_n9, eq60_e1010_d_n10, eq60_e1010_d_n11, eq60_e1010_d_n12, eq60_e1010_d_n13, eq60_e1010_d_n14, eq60_e1010_d_n15, eq60_e1010_d_n16, eq60_e1010_d_n17, eq60_e1010_d_n18, eq60_e1010_d_n19, eq60_e1010_d_n20, eq60_e1010_d_n21, eq60_e1010_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e1012;
        let eq60_node_derivatives: [f64; 23] = [eq60_e1012_d_n0, eq60_e1012_d_n1, eq60_e1012_d_n2, eq60_e1012_d_n3, eq60_e1012_d_n4, eq60_e1012_d_n5, eq60_e1012_d_n6, eq60_e1012_d_n7, eq60_e1012_d_n8, eq60_e1012_d_n9, eq60_e1012_d_n10, eq60_e1012_d_n11, eq60_e1012_d_n12, eq60_e1012_d_n13, eq60_e1012_d_n14, eq60_e1012_d_n15, eq60_e1012_d_n16, eq60_e1012_d_n17, eq60_e1012_d_n18, eq60_e1012_d_n19, eq60_e1012_d_n20, eq60_e1012_d_n21, eq60_e1012_d_n22];
        let eq60_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            multiplicity * (eq60_value),
            nodes,
            &eq60_node_derivatives,
            branches,
            &eq60_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1019,) = {
    if ((!s.b[424]) && s.b[428]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq61_value: f64 = eq61_e1019;
        stamper.stamp_potential_const(
            branches[29],
            eq61_value,
        );
        let (eq62_e1026,) = {
    if ((!s.b[424]) && s.b[428]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq62_value: f64 = eq62_e1026;
        stamper.stamp_potential_const(
            branches[30],
            eq62_value,
        );
        let (eq63_e1034,) = {
    if ((!s.b[424]) && (!s.b[428])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq63_value: f64 = eq63_e1034;
        stamper.stamp_potential_const(
            branches[31],
            eq63_value,
        );
        let (eq64_e1042,) = {
    if ((!s.b[424]) && (!s.b[428])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq64_value: f64 = eq64_e1042;
        stamper.stamp_potential_const(
            branches[32],
            eq64_value,
        );
        let (eq65_e1050,) = {
    if s.b[429] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq65_value: f64 = eq65_e1050;
        stamper.stamp_current_const(
            Some(nodes[7]),
            Some(nodes[8]),
            multiplicity * (eq65_value),
        );
        let (eq66_e1068,) = {
    if ((s.b[429] && s.b[430]) && s.b[431]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq66_value: f64 = eq66_e1068;
        stamper.stamp_current_const(
            Some(nodes[0]),
            Some(nodes[18]),
            multiplicity * (eq66_value),
        );
        let (eq67_e1086,) = {
    if ((s.b[429] && s.b[430]) && s.b[431]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq67_value: f64 = eq67_e1086;
        stamper.stamp_current_const(
            Some(nodes[2]),
            Some(nodes[22]),
            multiplicity * (eq67_value),
        );
        let (eq68_e1105,) = {
    if ((s.b[429] && s.b[430]) && (!s.b[431])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq68_value: f64 = eq68_e1105;
        stamper.stamp_current_const(
            Some(nodes[0]),
            Some(nodes[7]),
            multiplicity * (eq68_value),
        );
        let (eq69_e1124,) = {
    if ((s.b[429] && s.b[430]) && (!s.b[431])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq69_value: f64 = eq69_e1124;
        stamper.stamp_current_const(
            Some(nodes[2]),
            Some(nodes[8]),
            multiplicity * (eq69_value),
        );
        let (eq70_e1137,) = {
    if s.b[432] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq70_value: f64 = eq70_e1137;
        stamper.stamp_current_const(
            Some(nodes[9]),
            Some(nodes[8]),
            multiplicity * (eq70_value),
        );
        let (eq71_e1150,) = {
    if s.b[432] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq71_value: f64 = eq71_e1150;
        stamper.stamp_current_const(
            Some(nodes[9]),
            Some(nodes[7]),
            multiplicity * (eq71_value),
        );
        let (eq72_e1166, eq72_e1166_d_n0, eq72_e1166_d_n1, eq72_e1166_d_n2, eq72_e1166_d_n3, eq72_e1166_d_n4, eq72_e1166_d_n5, eq72_e1166_d_n6, eq72_e1166_d_n7, eq72_e1166_d_n8, eq72_e1166_d_n9, eq72_e1166_d_n10, eq72_e1166_d_n11, eq72_e1166_d_n12, eq72_e1166_d_n13, eq72_e1166_d_n14, eq72_e1166_d_n15, eq72_e1166_d_n16, eq72_e1166_d_n17, eq72_e1166_d_n18, eq72_e1166_d_n19, eq72_e1166_d_n20, eq72_e1166_d_n21, eq72_e1166_d_n22,) = {
    if (s.b[433] && s.b[434]) {
        let eq72_e1156: f64 = (p.p6 * s.v[48]);
        let eq72_e1156_d_n0: f64 = (p.p6 * s.dn[48][0]);
        let eq72_e1156_d_n1: f64 = (p.p6 * s.dn[48][1]);
        let eq72_e1156_d_n2: f64 = (p.p6 * s.dn[48][2]);
        let eq72_e1156_d_n3: f64 = (p.p6 * s.dn[48][3]);
        let eq72_e1156_d_n4: f64 = (p.p6 * s.dn[48][4]);
        let eq72_e1156_d_n5: f64 = (p.p6 * s.dn[48][5]);
        let eq72_e1156_d_n6: f64 = (p.p6 * s.dn[48][6]);
        let eq72_e1156_d_n7: f64 = (p.p6 * s.dn[48][7]);
        let eq72_e1156_d_n8: f64 = (p.p6 * s.dn[48][8]);
        let eq72_e1156_d_n9: f64 = (p.p6 * s.dn[48][9]);
        let eq72_e1156_d_n10: f64 = (p.p6 * s.dn[48][10]);
        let eq72_e1156_d_n11: f64 = (p.p6 * s.dn[48][11]);
        let eq72_e1156_d_n12: f64 = (p.p6 * s.dn[48][12]);
        let eq72_e1156_d_n13: f64 = (p.p6 * s.dn[48][13]);
        let eq72_e1156_d_n14: f64 = (p.p6 * s.dn[48][14]);
        let eq72_e1156_d_n15: f64 = (p.p6 * s.dn[48][15]);
        let eq72_e1156_d_n16: f64 = (p.p6 * s.dn[48][16]);
        let eq72_e1156_d_n17: f64 = (p.p6 * s.dn[48][17]);
        let eq72_e1156_d_n18: f64 = (p.p6 * s.dn[48][18]);
        let eq72_e1156_d_n19: f64 = (p.p6 * s.dn[48][19]);
        let eq72_e1156_d_n20: f64 = (p.p6 * s.dn[48][20]);
        let eq72_e1156_d_n21: f64 = (p.p6 * s.dn[48][21]);
        let eq72_e1156_d_n22: f64 = (p.p6 * s.dn[48][22]);
        let eq72_e1158: f64 = (eq72_e1156 * s.v[233]);
        let eq72_e1158_d_n0: f64 = ((eq72_e1156_d_n0 * s.v[233]) + (eq72_e1156 * s.dn[233][0]));
        let eq72_e1158_d_n1: f64 = ((eq72_e1156_d_n1 * s.v[233]) + (eq72_e1156 * s.dn[233][1]));
        let eq72_e1158_d_n2: f64 = ((eq72_e1156_d_n2 * s.v[233]) + (eq72_e1156 * s.dn[233][2]));
        let eq72_e1158_d_n3: f64 = ((eq72_e1156_d_n3 * s.v[233]) + (eq72_e1156 * s.dn[233][3]));
        let eq72_e1158_d_n4: f64 = ((eq72_e1156_d_n4 * s.v[233]) + (eq72_e1156 * s.dn[233][4]));
        let eq72_e1158_d_n5: f64 = ((eq72_e1156_d_n5 * s.v[233]) + (eq72_e1156 * s.dn[233][5]));
        let eq72_e1158_d_n6: f64 = ((eq72_e1156_d_n6 * s.v[233]) + (eq72_e1156 * s.dn[233][6]));
        let eq72_e1158_d_n7: f64 = ((eq72_e1156_d_n7 * s.v[233]) + (eq72_e1156 * s.dn[233][7]));
        let eq72_e1158_d_n8: f64 = ((eq72_e1156_d_n8 * s.v[233]) + (eq72_e1156 * s.dn[233][8]));
        let eq72_e1158_d_n9: f64 = ((eq72_e1156_d_n9 * s.v[233]) + (eq72_e1156 * s.dn[233][9]));
        let eq72_e1158_d_n10: f64 = ((eq72_e1156_d_n10 * s.v[233]) + (eq72_e1156 * s.dn[233][10]));
        let eq72_e1158_d_n11: f64 = ((eq72_e1156_d_n11 * s.v[233]) + (eq72_e1156 * s.dn[233][11]));
        let eq72_e1158_d_n12: f64 = ((eq72_e1156_d_n12 * s.v[233]) + (eq72_e1156 * s.dn[233][12]));
        let eq72_e1158_d_n13: f64 = ((eq72_e1156_d_n13 * s.v[233]) + (eq72_e1156 * s.dn[233][13]));
        let eq72_e1158_d_n14: f64 = ((eq72_e1156_d_n14 * s.v[233]) + (eq72_e1156 * s.dn[233][14]));
        let eq72_e1158_d_n15: f64 = ((eq72_e1156_d_n15 * s.v[233]) + (eq72_e1156 * s.dn[233][15]));
        let eq72_e1158_d_n16: f64 = ((eq72_e1156_d_n16 * s.v[233]) + (eq72_e1156 * s.dn[233][16]));
        let eq72_e1158_d_n17: f64 = ((eq72_e1156_d_n17 * s.v[233]) + (eq72_e1156 * s.dn[233][17]));
        let eq72_e1158_d_n18: f64 = ((eq72_e1156_d_n18 * s.v[233]) + (eq72_e1156 * s.dn[233][18]));
        let eq72_e1158_d_n19: f64 = ((eq72_e1156_d_n19 * s.v[233]) + (eq72_e1156 * s.dn[233][19]));
        let eq72_e1158_d_n20: f64 = ((eq72_e1156_d_n20 * s.v[233]) + (eq72_e1156 * s.dn[233][20]));
        let eq72_e1158_d_n21: f64 = ((eq72_e1156_d_n21 * s.v[233]) + (eq72_e1156 * s.dn[233][21]));
        let eq72_e1158_d_n22: f64 = ((eq72_e1156_d_n22 * s.v[233]) + (eq72_e1156 * s.dn[233][22]));
        let eq72_e1161: f64 = (p.p6 * s.v[379]);
        let eq72_e1161_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq72_e1161_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq72_e1161_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq72_e1161_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq72_e1161_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq72_e1161_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq72_e1161_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq72_e1161_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq72_e1161_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq72_e1161_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq72_e1161_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq72_e1161_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq72_e1161_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq72_e1161_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq72_e1161_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq72_e1161_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq72_e1161_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq72_e1161_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq72_e1161_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq72_e1161_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq72_e1161_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq72_e1161_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq72_e1161_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq72_e1163: f64 = (eq72_e1161 * (nv15 - nv7));
        let eq72_e1163_d_n0: f64 = (eq72_e1161_d_n0 * (nv15 - nv7));
        let eq72_e1163_d_n1: f64 = (eq72_e1161_d_n1 * (nv15 - nv7));
        let eq72_e1163_d_n2: f64 = (eq72_e1161_d_n2 * (nv15 - nv7));
        let eq72_e1163_d_n3: f64 = (eq72_e1161_d_n3 * (nv15 - nv7));
        let eq72_e1163_d_n4: f64 = (eq72_e1161_d_n4 * (nv15 - nv7));
        let eq72_e1163_d_n5: f64 = (eq72_e1161_d_n5 * (nv15 - nv7));
        let eq72_e1163_d_n6: f64 = (eq72_e1161_d_n6 * (nv15 - nv7));
        let eq72_e1163_d_n7: f64 = ((eq72_e1161_d_n7 * (nv15 - nv7)) + (-eq72_e1161));
        let eq72_e1163_d_n8: f64 = (eq72_e1161_d_n8 * (nv15 - nv7));
        let eq72_e1163_d_n9: f64 = (eq72_e1161_d_n9 * (nv15 - nv7));
        let eq72_e1163_d_n10: f64 = (eq72_e1161_d_n10 * (nv15 - nv7));
        let eq72_e1163_d_n11: f64 = (eq72_e1161_d_n11 * (nv15 - nv7));
        let eq72_e1163_d_n12: f64 = (eq72_e1161_d_n12 * (nv15 - nv7));
        let eq72_e1163_d_n13: f64 = (eq72_e1161_d_n13 * (nv15 - nv7));
        let eq72_e1163_d_n14: f64 = (eq72_e1161_d_n14 * (nv15 - nv7));
        let eq72_e1163_d_n15: f64 = ((eq72_e1161_d_n15 * (nv15 - nv7)) + eq72_e1161);
        let eq72_e1163_d_n16: f64 = (eq72_e1161_d_n16 * (nv15 - nv7));
        let eq72_e1163_d_n17: f64 = (eq72_e1161_d_n17 * (nv15 - nv7));
        let eq72_e1163_d_n18: f64 = (eq72_e1161_d_n18 * (nv15 - nv7));
        let eq72_e1163_d_n19: f64 = (eq72_e1161_d_n19 * (nv15 - nv7));
        let eq72_e1163_d_n20: f64 = (eq72_e1161_d_n20 * (nv15 - nv7));
        let eq72_e1163_d_n21: f64 = (eq72_e1161_d_n21 * (nv15 - nv7));
        let eq72_e1163_d_n22: f64 = (eq72_e1161_d_n22 * (nv15 - nv7));
        let eq72_e1164: f64 = (eq72_e1158 + eq72_e1163);
        let eq72_e1164_d_n0: f64 = (eq72_e1158_d_n0 + eq72_e1163_d_n0);
        let eq72_e1164_d_n1: f64 = (eq72_e1158_d_n1 + eq72_e1163_d_n1);
        let eq72_e1164_d_n2: f64 = (eq72_e1158_d_n2 + eq72_e1163_d_n2);
        let eq72_e1164_d_n3: f64 = (eq72_e1158_d_n3 + eq72_e1163_d_n3);
        let eq72_e1164_d_n4: f64 = (eq72_e1158_d_n4 + eq72_e1163_d_n4);
        let eq72_e1164_d_n5: f64 = (eq72_e1158_d_n5 + eq72_e1163_d_n5);
        let eq72_e1164_d_n6: f64 = (eq72_e1158_d_n6 + eq72_e1163_d_n6);
        let eq72_e1164_d_n7: f64 = (eq72_e1158_d_n7 + eq72_e1163_d_n7);
        let eq72_e1164_d_n8: f64 = (eq72_e1158_d_n8 + eq72_e1163_d_n8);
        let eq72_e1164_d_n9: f64 = (eq72_e1158_d_n9 + eq72_e1163_d_n9);
        let eq72_e1164_d_n10: f64 = (eq72_e1158_d_n10 + eq72_e1163_d_n10);
        let eq72_e1164_d_n11: f64 = (eq72_e1158_d_n11 + eq72_e1163_d_n11);
        let eq72_e1164_d_n12: f64 = (eq72_e1158_d_n12 + eq72_e1163_d_n12);
        let eq72_e1164_d_n13: f64 = (eq72_e1158_d_n13 + eq72_e1163_d_n13);
        let eq72_e1164_d_n14: f64 = (eq72_e1158_d_n14 + eq72_e1163_d_n14);
        let eq72_e1164_d_n15: f64 = (eq72_e1158_d_n15 + eq72_e1163_d_n15);
        let eq72_e1164_d_n16: f64 = (eq72_e1158_d_n16 + eq72_e1163_d_n16);
        let eq72_e1164_d_n17: f64 = (eq72_e1158_d_n17 + eq72_e1163_d_n17);
        let eq72_e1164_d_n18: f64 = (eq72_e1158_d_n18 + eq72_e1163_d_n18);
        let eq72_e1164_d_n19: f64 = (eq72_e1158_d_n19 + eq72_e1163_d_n19);
        let eq72_e1164_d_n20: f64 = (eq72_e1158_d_n20 + eq72_e1163_d_n20);
        let eq72_e1164_d_n21: f64 = (eq72_e1158_d_n21 + eq72_e1163_d_n21);
        let eq72_e1164_d_n22: f64 = (eq72_e1158_d_n22 + eq72_e1163_d_n22);
        (eq72_e1164, eq72_e1164_d_n0, eq72_e1164_d_n1, eq72_e1164_d_n2, eq72_e1164_d_n3, eq72_e1164_d_n4, eq72_e1164_d_n5, eq72_e1164_d_n6, eq72_e1164_d_n7, eq72_e1164_d_n8, eq72_e1164_d_n9, eq72_e1164_d_n10, eq72_e1164_d_n11, eq72_e1164_d_n12, eq72_e1164_d_n13, eq72_e1164_d_n14, eq72_e1164_d_n15, eq72_e1164_d_n16, eq72_e1164_d_n17, eq72_e1164_d_n18, eq72_e1164_d_n19, eq72_e1164_d_n20, eq72_e1164_d_n21, eq72_e1164_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_value: f64 = eq72_e1166;
        let eq72_node_derivatives: [f64; 23] = [eq72_e1166_d_n0, eq72_e1166_d_n1, eq72_e1166_d_n2, eq72_e1166_d_n3, eq72_e1166_d_n4, eq72_e1166_d_n5, eq72_e1166_d_n6, eq72_e1166_d_n7, eq72_e1166_d_n8, eq72_e1166_d_n9, eq72_e1166_d_n10, eq72_e1166_d_n11, eq72_e1166_d_n12, eq72_e1166_d_n13, eq72_e1166_d_n14, eq72_e1166_d_n15, eq72_e1166_d_n16, eq72_e1166_d_n17, eq72_e1166_d_n18, eq72_e1166_d_n19, eq72_e1166_d_n20, eq72_e1166_d_n21, eq72_e1166_d_n22];
        let eq72_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[15]),
            Some(nodes[7]),
            multiplicity * (eq72_value),
            nodes,
            &eq72_node_derivatives,
            branches,
            &eq72_branch_derivatives,
            multiplicity,
        );
        let (eq73_e1173,) = {
    if (s.b[433] && (!s.b[434])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq73_value: f64 = eq73_e1173;
        stamper.stamp_potential_const(
            branches[33],
            eq73_value,
        );
        let (eq74_e1178,) = {
    if (!s.b[433]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq74_value: f64 = eq74_e1178;
        stamper.stamp_potential_const(
            branches[34],
            eq74_value,
        );
    }

    pub(super) fn stamp_transient_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv19 = ctx.node_voltage(nodes[19]);
        let nv20 = ctx.node_voltage(nodes[20]);
        let (eq75_e1194, eq75_e1194_d_n0, eq75_e1194_d_n1, eq75_e1194_d_n2, eq75_e1194_d_n3, eq75_e1194_d_n4, eq75_e1194_d_n5, eq75_e1194_d_n6, eq75_e1194_d_n7, eq75_e1194_d_n8, eq75_e1194_d_n9, eq75_e1194_d_n10, eq75_e1194_d_n11, eq75_e1194_d_n12, eq75_e1194_d_n13, eq75_e1194_d_n14, eq75_e1194_d_n15, eq75_e1194_d_n16, eq75_e1194_d_n17, eq75_e1194_d_n18, eq75_e1194_d_n19, eq75_e1194_d_n20, eq75_e1194_d_n21, eq75_e1194_d_n22,) = {
    if (s.b[448] && s.b[449]) {
        let eq75_e1184: f64 = (p.p6 * s.v[52]);
        let eq75_e1184_d_n0: f64 = (p.p6 * s.dn[52][0]);
        let eq75_e1184_d_n1: f64 = (p.p6 * s.dn[52][1]);
        let eq75_e1184_d_n2: f64 = (p.p6 * s.dn[52][2]);
        let eq75_e1184_d_n3: f64 = (p.p6 * s.dn[52][3]);
        let eq75_e1184_d_n4: f64 = (p.p6 * s.dn[52][4]);
        let eq75_e1184_d_n5: f64 = (p.p6 * s.dn[52][5]);
        let eq75_e1184_d_n6: f64 = (p.p6 * s.dn[52][6]);
        let eq75_e1184_d_n7: f64 = (p.p6 * s.dn[52][7]);
        let eq75_e1184_d_n8: f64 = (p.p6 * s.dn[52][8]);
        let eq75_e1184_d_n9: f64 = (p.p6 * s.dn[52][9]);
        let eq75_e1184_d_n10: f64 = (p.p6 * s.dn[52][10]);
        let eq75_e1184_d_n11: f64 = (p.p6 * s.dn[52][11]);
        let eq75_e1184_d_n12: f64 = (p.p6 * s.dn[52][12]);
        let eq75_e1184_d_n13: f64 = (p.p6 * s.dn[52][13]);
        let eq75_e1184_d_n14: f64 = (p.p6 * s.dn[52][14]);
        let eq75_e1184_d_n15: f64 = (p.p6 * s.dn[52][15]);
        let eq75_e1184_d_n16: f64 = (p.p6 * s.dn[52][16]);
        let eq75_e1184_d_n17: f64 = (p.p6 * s.dn[52][17]);
        let eq75_e1184_d_n18: f64 = (p.p6 * s.dn[52][18]);
        let eq75_e1184_d_n19: f64 = (p.p6 * s.dn[52][19]);
        let eq75_e1184_d_n20: f64 = (p.p6 * s.dn[52][20]);
        let eq75_e1184_d_n21: f64 = (p.p6 * s.dn[52][21]);
        let eq75_e1184_d_n22: f64 = (p.p6 * s.dn[52][22]);
        let eq75_e1186: f64 = (eq75_e1184 * s.v[245]);
        let eq75_e1186_d_n0: f64 = ((eq75_e1184_d_n0 * s.v[245]) + (eq75_e1184 * s.dn[245][0]));
        let eq75_e1186_d_n1: f64 = ((eq75_e1184_d_n1 * s.v[245]) + (eq75_e1184 * s.dn[245][1]));
        let eq75_e1186_d_n2: f64 = ((eq75_e1184_d_n2 * s.v[245]) + (eq75_e1184 * s.dn[245][2]));
        let eq75_e1186_d_n3: f64 = ((eq75_e1184_d_n3 * s.v[245]) + (eq75_e1184 * s.dn[245][3]));
        let eq75_e1186_d_n4: f64 = ((eq75_e1184_d_n4 * s.v[245]) + (eq75_e1184 * s.dn[245][4]));
        let eq75_e1186_d_n5: f64 = ((eq75_e1184_d_n5 * s.v[245]) + (eq75_e1184 * s.dn[245][5]));
        let eq75_e1186_d_n6: f64 = ((eq75_e1184_d_n6 * s.v[245]) + (eq75_e1184 * s.dn[245][6]));
        let eq75_e1186_d_n7: f64 = ((eq75_e1184_d_n7 * s.v[245]) + (eq75_e1184 * s.dn[245][7]));
        let eq75_e1186_d_n8: f64 = ((eq75_e1184_d_n8 * s.v[245]) + (eq75_e1184 * s.dn[245][8]));
        let eq75_e1186_d_n9: f64 = ((eq75_e1184_d_n9 * s.v[245]) + (eq75_e1184 * s.dn[245][9]));
        let eq75_e1186_d_n10: f64 = ((eq75_e1184_d_n10 * s.v[245]) + (eq75_e1184 * s.dn[245][10]));
        let eq75_e1186_d_n11: f64 = ((eq75_e1184_d_n11 * s.v[245]) + (eq75_e1184 * s.dn[245][11]));
        let eq75_e1186_d_n12: f64 = ((eq75_e1184_d_n12 * s.v[245]) + (eq75_e1184 * s.dn[245][12]));
        let eq75_e1186_d_n13: f64 = ((eq75_e1184_d_n13 * s.v[245]) + (eq75_e1184 * s.dn[245][13]));
        let eq75_e1186_d_n14: f64 = ((eq75_e1184_d_n14 * s.v[245]) + (eq75_e1184 * s.dn[245][14]));
        let eq75_e1186_d_n15: f64 = ((eq75_e1184_d_n15 * s.v[245]) + (eq75_e1184 * s.dn[245][15]));
        let eq75_e1186_d_n16: f64 = ((eq75_e1184_d_n16 * s.v[245]) + (eq75_e1184 * s.dn[245][16]));
        let eq75_e1186_d_n17: f64 = ((eq75_e1184_d_n17 * s.v[245]) + (eq75_e1184 * s.dn[245][17]));
        let eq75_e1186_d_n18: f64 = ((eq75_e1184_d_n18 * s.v[245]) + (eq75_e1184 * s.dn[245][18]));
        let eq75_e1186_d_n19: f64 = ((eq75_e1184_d_n19 * s.v[245]) + (eq75_e1184 * s.dn[245][19]));
        let eq75_e1186_d_n20: f64 = ((eq75_e1184_d_n20 * s.v[245]) + (eq75_e1184 * s.dn[245][20]));
        let eq75_e1186_d_n21: f64 = ((eq75_e1184_d_n21 * s.v[245]) + (eq75_e1184 * s.dn[245][21]));
        let eq75_e1186_d_n22: f64 = ((eq75_e1184_d_n22 * s.v[245]) + (eq75_e1184 * s.dn[245][22]));
        let eq75_e1189: f64 = (p.p6 * s.v[379]);
        let eq75_e1189_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq75_e1189_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq75_e1189_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq75_e1189_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq75_e1189_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq75_e1189_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq75_e1189_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq75_e1189_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq75_e1189_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq75_e1189_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq75_e1189_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq75_e1189_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq75_e1189_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq75_e1189_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq75_e1189_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq75_e1189_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq75_e1189_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq75_e1189_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq75_e1189_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq75_e1189_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq75_e1189_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq75_e1189_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq75_e1189_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq75_e1191: f64 = (eq75_e1189 * (nv8 - nv19));
        let eq75_e1191_d_n0: f64 = (eq75_e1189_d_n0 * (nv8 - nv19));
        let eq75_e1191_d_n1: f64 = (eq75_e1189_d_n1 * (nv8 - nv19));
        let eq75_e1191_d_n2: f64 = (eq75_e1189_d_n2 * (nv8 - nv19));
        let eq75_e1191_d_n3: f64 = (eq75_e1189_d_n3 * (nv8 - nv19));
        let eq75_e1191_d_n4: f64 = (eq75_e1189_d_n4 * (nv8 - nv19));
        let eq75_e1191_d_n5: f64 = (eq75_e1189_d_n5 * (nv8 - nv19));
        let eq75_e1191_d_n6: f64 = (eq75_e1189_d_n6 * (nv8 - nv19));
        let eq75_e1191_d_n7: f64 = (eq75_e1189_d_n7 * (nv8 - nv19));
        let eq75_e1191_d_n8: f64 = ((eq75_e1189_d_n8 * (nv8 - nv19)) + eq75_e1189);
        let eq75_e1191_d_n9: f64 = (eq75_e1189_d_n9 * (nv8 - nv19));
        let eq75_e1191_d_n10: f64 = (eq75_e1189_d_n10 * (nv8 - nv19));
        let eq75_e1191_d_n11: f64 = (eq75_e1189_d_n11 * (nv8 - nv19));
        let eq75_e1191_d_n12: f64 = (eq75_e1189_d_n12 * (nv8 - nv19));
        let eq75_e1191_d_n13: f64 = (eq75_e1189_d_n13 * (nv8 - nv19));
        let eq75_e1191_d_n14: f64 = (eq75_e1189_d_n14 * (nv8 - nv19));
        let eq75_e1191_d_n15: f64 = (eq75_e1189_d_n15 * (nv8 - nv19));
        let eq75_e1191_d_n16: f64 = (eq75_e1189_d_n16 * (nv8 - nv19));
        let eq75_e1191_d_n17: f64 = (eq75_e1189_d_n17 * (nv8 - nv19));
        let eq75_e1191_d_n18: f64 = (eq75_e1189_d_n18 * (nv8 - nv19));
        let eq75_e1191_d_n19: f64 = ((eq75_e1189_d_n19 * (nv8 - nv19)) + (-eq75_e1189));
        let eq75_e1191_d_n20: f64 = (eq75_e1189_d_n20 * (nv8 - nv19));
        let eq75_e1191_d_n21: f64 = (eq75_e1189_d_n21 * (nv8 - nv19));
        let eq75_e1191_d_n22: f64 = (eq75_e1189_d_n22 * (nv8 - nv19));
        let eq75_e1192: f64 = (eq75_e1186 + eq75_e1191);
        let eq75_e1192_d_n0: f64 = (eq75_e1186_d_n0 + eq75_e1191_d_n0);
        let eq75_e1192_d_n1: f64 = (eq75_e1186_d_n1 + eq75_e1191_d_n1);
        let eq75_e1192_d_n2: f64 = (eq75_e1186_d_n2 + eq75_e1191_d_n2);
        let eq75_e1192_d_n3: f64 = (eq75_e1186_d_n3 + eq75_e1191_d_n3);
        let eq75_e1192_d_n4: f64 = (eq75_e1186_d_n4 + eq75_e1191_d_n4);
        let eq75_e1192_d_n5: f64 = (eq75_e1186_d_n5 + eq75_e1191_d_n5);
        let eq75_e1192_d_n6: f64 = (eq75_e1186_d_n6 + eq75_e1191_d_n6);
        let eq75_e1192_d_n7: f64 = (eq75_e1186_d_n7 + eq75_e1191_d_n7);
        let eq75_e1192_d_n8: f64 = (eq75_e1186_d_n8 + eq75_e1191_d_n8);
        let eq75_e1192_d_n9: f64 = (eq75_e1186_d_n9 + eq75_e1191_d_n9);
        let eq75_e1192_d_n10: f64 = (eq75_e1186_d_n10 + eq75_e1191_d_n10);
        let eq75_e1192_d_n11: f64 = (eq75_e1186_d_n11 + eq75_e1191_d_n11);
        let eq75_e1192_d_n12: f64 = (eq75_e1186_d_n12 + eq75_e1191_d_n12);
        let eq75_e1192_d_n13: f64 = (eq75_e1186_d_n13 + eq75_e1191_d_n13);
        let eq75_e1192_d_n14: f64 = (eq75_e1186_d_n14 + eq75_e1191_d_n14);
        let eq75_e1192_d_n15: f64 = (eq75_e1186_d_n15 + eq75_e1191_d_n15);
        let eq75_e1192_d_n16: f64 = (eq75_e1186_d_n16 + eq75_e1191_d_n16);
        let eq75_e1192_d_n17: f64 = (eq75_e1186_d_n17 + eq75_e1191_d_n17);
        let eq75_e1192_d_n18: f64 = (eq75_e1186_d_n18 + eq75_e1191_d_n18);
        let eq75_e1192_d_n19: f64 = (eq75_e1186_d_n19 + eq75_e1191_d_n19);
        let eq75_e1192_d_n20: f64 = (eq75_e1186_d_n20 + eq75_e1191_d_n20);
        let eq75_e1192_d_n21: f64 = (eq75_e1186_d_n21 + eq75_e1191_d_n21);
        let eq75_e1192_d_n22: f64 = (eq75_e1186_d_n22 + eq75_e1191_d_n22);
        (eq75_e1192, eq75_e1192_d_n0, eq75_e1192_d_n1, eq75_e1192_d_n2, eq75_e1192_d_n3, eq75_e1192_d_n4, eq75_e1192_d_n5, eq75_e1192_d_n6, eq75_e1192_d_n7, eq75_e1192_d_n8, eq75_e1192_d_n9, eq75_e1192_d_n10, eq75_e1192_d_n11, eq75_e1192_d_n12, eq75_e1192_d_n13, eq75_e1192_d_n14, eq75_e1192_d_n15, eq75_e1192_d_n16, eq75_e1192_d_n17, eq75_e1192_d_n18, eq75_e1192_d_n19, eq75_e1192_d_n20, eq75_e1192_d_n21, eq75_e1192_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq75_value: f64 = eq75_e1194;
        let eq75_node_derivatives: [f64; 23] = [eq75_e1194_d_n0, eq75_e1194_d_n1, eq75_e1194_d_n2, eq75_e1194_d_n3, eq75_e1194_d_n4, eq75_e1194_d_n5, eq75_e1194_d_n6, eq75_e1194_d_n7, eq75_e1194_d_n8, eq75_e1194_d_n9, eq75_e1194_d_n10, eq75_e1194_d_n11, eq75_e1194_d_n12, eq75_e1194_d_n13, eq75_e1194_d_n14, eq75_e1194_d_n15, eq75_e1194_d_n16, eq75_e1194_d_n17, eq75_e1194_d_n18, eq75_e1194_d_n19, eq75_e1194_d_n20, eq75_e1194_d_n21, eq75_e1194_d_n22];
        let eq75_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[19]),
            multiplicity * (eq75_value),
            nodes,
            &eq75_node_derivatives,
            branches,
            &eq75_branch_derivatives,
            multiplicity,
        );
        let (eq76_e1201,) = {
    if (s.b[448] && (!s.b[449])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq76_value: f64 = eq76_e1201;
        stamper.stamp_potential_const(
            branches[35],
            eq76_value,
        );
        let (eq77_e1206,) = {
    if (!s.b[448]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq77_value: f64 = eq77_e1206;
        stamper.stamp_potential_const(
            branches[36],
            eq77_value,
        );
        let (eq78_e1214,) = {
    if ((!s.b[448]) && (!s.b[457])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq78_value: f64 = eq78_e1214;
        stamper.stamp_potential_const(
            branches[37],
            eq78_value,
        );
        let (eq79_e1230, eq79_e1230_d_n0, eq79_e1230_d_n1, eq79_e1230_d_n2, eq79_e1230_d_n3, eq79_e1230_d_n4, eq79_e1230_d_n5, eq79_e1230_d_n6, eq79_e1230_d_n7, eq79_e1230_d_n8, eq79_e1230_d_n9, eq79_e1230_d_n10, eq79_e1230_d_n11, eq79_e1230_d_n12, eq79_e1230_d_n13, eq79_e1230_d_n14, eq79_e1230_d_n15, eq79_e1230_d_n16, eq79_e1230_d_n17, eq79_e1230_d_n18, eq79_e1230_d_n19, eq79_e1230_d_n20, eq79_e1230_d_n21, eq79_e1230_d_n22,) = {
    if (s.b[463] && s.b[464]) {
        let eq79_e1220: f64 = (p.p6 * s.v[56]);
        let eq79_e1220_d_n0: f64 = (p.p6 * s.dn[56][0]);
        let eq79_e1220_d_n1: f64 = (p.p6 * s.dn[56][1]);
        let eq79_e1220_d_n2: f64 = (p.p6 * s.dn[56][2]);
        let eq79_e1220_d_n3: f64 = (p.p6 * s.dn[56][3]);
        let eq79_e1220_d_n4: f64 = (p.p6 * s.dn[56][4]);
        let eq79_e1220_d_n5: f64 = (p.p6 * s.dn[56][5]);
        let eq79_e1220_d_n6: f64 = (p.p6 * s.dn[56][6]);
        let eq79_e1220_d_n7: f64 = (p.p6 * s.dn[56][7]);
        let eq79_e1220_d_n8: f64 = (p.p6 * s.dn[56][8]);
        let eq79_e1220_d_n9: f64 = (p.p6 * s.dn[56][9]);
        let eq79_e1220_d_n10: f64 = (p.p6 * s.dn[56][10]);
        let eq79_e1220_d_n11: f64 = (p.p6 * s.dn[56][11]);
        let eq79_e1220_d_n12: f64 = (p.p6 * s.dn[56][12]);
        let eq79_e1220_d_n13: f64 = (p.p6 * s.dn[56][13]);
        let eq79_e1220_d_n14: f64 = (p.p6 * s.dn[56][14]);
        let eq79_e1220_d_n15: f64 = (p.p6 * s.dn[56][15]);
        let eq79_e1220_d_n16: f64 = (p.p6 * s.dn[56][16]);
        let eq79_e1220_d_n17: f64 = (p.p6 * s.dn[56][17]);
        let eq79_e1220_d_n18: f64 = (p.p6 * s.dn[56][18]);
        let eq79_e1220_d_n19: f64 = (p.p6 * s.dn[56][19]);
        let eq79_e1220_d_n20: f64 = (p.p6 * s.dn[56][20]);
        let eq79_e1220_d_n21: f64 = (p.p6 * s.dn[56][21]);
        let eq79_e1220_d_n22: f64 = (p.p6 * s.dn[56][22]);
        let eq79_e1222: f64 = (eq79_e1220 * s.v[257]);
        let eq79_e1222_d_n0: f64 = ((eq79_e1220_d_n0 * s.v[257]) + (eq79_e1220 * s.dn[257][0]));
        let eq79_e1222_d_n1: f64 = ((eq79_e1220_d_n1 * s.v[257]) + (eq79_e1220 * s.dn[257][1]));
        let eq79_e1222_d_n2: f64 = ((eq79_e1220_d_n2 * s.v[257]) + (eq79_e1220 * s.dn[257][2]));
        let eq79_e1222_d_n3: f64 = ((eq79_e1220_d_n3 * s.v[257]) + (eq79_e1220 * s.dn[257][3]));
        let eq79_e1222_d_n4: f64 = ((eq79_e1220_d_n4 * s.v[257]) + (eq79_e1220 * s.dn[257][4]));
        let eq79_e1222_d_n5: f64 = ((eq79_e1220_d_n5 * s.v[257]) + (eq79_e1220 * s.dn[257][5]));
        let eq79_e1222_d_n6: f64 = ((eq79_e1220_d_n6 * s.v[257]) + (eq79_e1220 * s.dn[257][6]));
        let eq79_e1222_d_n7: f64 = ((eq79_e1220_d_n7 * s.v[257]) + (eq79_e1220 * s.dn[257][7]));
        let eq79_e1222_d_n8: f64 = ((eq79_e1220_d_n8 * s.v[257]) + (eq79_e1220 * s.dn[257][8]));
        let eq79_e1222_d_n9: f64 = ((eq79_e1220_d_n9 * s.v[257]) + (eq79_e1220 * s.dn[257][9]));
        let eq79_e1222_d_n10: f64 = ((eq79_e1220_d_n10 * s.v[257]) + (eq79_e1220 * s.dn[257][10]));
        let eq79_e1222_d_n11: f64 = ((eq79_e1220_d_n11 * s.v[257]) + (eq79_e1220 * s.dn[257][11]));
        let eq79_e1222_d_n12: f64 = ((eq79_e1220_d_n12 * s.v[257]) + (eq79_e1220 * s.dn[257][12]));
        let eq79_e1222_d_n13: f64 = ((eq79_e1220_d_n13 * s.v[257]) + (eq79_e1220 * s.dn[257][13]));
        let eq79_e1222_d_n14: f64 = ((eq79_e1220_d_n14 * s.v[257]) + (eq79_e1220 * s.dn[257][14]));
        let eq79_e1222_d_n15: f64 = ((eq79_e1220_d_n15 * s.v[257]) + (eq79_e1220 * s.dn[257][15]));
        let eq79_e1222_d_n16: f64 = ((eq79_e1220_d_n16 * s.v[257]) + (eq79_e1220 * s.dn[257][16]));
        let eq79_e1222_d_n17: f64 = ((eq79_e1220_d_n17 * s.v[257]) + (eq79_e1220 * s.dn[257][17]));
        let eq79_e1222_d_n18: f64 = ((eq79_e1220_d_n18 * s.v[257]) + (eq79_e1220 * s.dn[257][18]));
        let eq79_e1222_d_n19: f64 = ((eq79_e1220_d_n19 * s.v[257]) + (eq79_e1220 * s.dn[257][19]));
        let eq79_e1222_d_n20: f64 = ((eq79_e1220_d_n20 * s.v[257]) + (eq79_e1220 * s.dn[257][20]));
        let eq79_e1222_d_n21: f64 = ((eq79_e1220_d_n21 * s.v[257]) + (eq79_e1220 * s.dn[257][21]));
        let eq79_e1222_d_n22: f64 = ((eq79_e1220_d_n22 * s.v[257]) + (eq79_e1220 * s.dn[257][22]));
        let eq79_e1225: f64 = (p.p6 * s.v[379]);
        let eq79_e1225_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq79_e1225_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq79_e1225_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq79_e1225_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq79_e1225_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq79_e1225_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq79_e1225_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq79_e1225_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq79_e1225_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq79_e1225_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq79_e1225_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq79_e1225_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq79_e1225_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq79_e1225_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq79_e1225_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq79_e1225_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq79_e1225_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq79_e1225_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq79_e1225_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq79_e1225_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq79_e1225_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq79_e1225_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq79_e1225_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq79_e1227: f64 = (eq79_e1225 * (nv16 - nv15));
        let eq79_e1227_d_n0: f64 = (eq79_e1225_d_n0 * (nv16 - nv15));
        let eq79_e1227_d_n1: f64 = (eq79_e1225_d_n1 * (nv16 - nv15));
        let eq79_e1227_d_n2: f64 = (eq79_e1225_d_n2 * (nv16 - nv15));
        let eq79_e1227_d_n3: f64 = (eq79_e1225_d_n3 * (nv16 - nv15));
        let eq79_e1227_d_n4: f64 = (eq79_e1225_d_n4 * (nv16 - nv15));
        let eq79_e1227_d_n5: f64 = (eq79_e1225_d_n5 * (nv16 - nv15));
        let eq79_e1227_d_n6: f64 = (eq79_e1225_d_n6 * (nv16 - nv15));
        let eq79_e1227_d_n7: f64 = (eq79_e1225_d_n7 * (nv16 - nv15));
        let eq79_e1227_d_n8: f64 = (eq79_e1225_d_n8 * (nv16 - nv15));
        let eq79_e1227_d_n9: f64 = (eq79_e1225_d_n9 * (nv16 - nv15));
        let eq79_e1227_d_n10: f64 = (eq79_e1225_d_n10 * (nv16 - nv15));
        let eq79_e1227_d_n11: f64 = (eq79_e1225_d_n11 * (nv16 - nv15));
        let eq79_e1227_d_n12: f64 = (eq79_e1225_d_n12 * (nv16 - nv15));
        let eq79_e1227_d_n13: f64 = (eq79_e1225_d_n13 * (nv16 - nv15));
        let eq79_e1227_d_n14: f64 = (eq79_e1225_d_n14 * (nv16 - nv15));
        let eq79_e1227_d_n15: f64 = ((eq79_e1225_d_n15 * (nv16 - nv15)) + (-eq79_e1225));
        let eq79_e1227_d_n16: f64 = ((eq79_e1225_d_n16 * (nv16 - nv15)) + eq79_e1225);
        let eq79_e1227_d_n17: f64 = (eq79_e1225_d_n17 * (nv16 - nv15));
        let eq79_e1227_d_n18: f64 = (eq79_e1225_d_n18 * (nv16 - nv15));
        let eq79_e1227_d_n19: f64 = (eq79_e1225_d_n19 * (nv16 - nv15));
        let eq79_e1227_d_n20: f64 = (eq79_e1225_d_n20 * (nv16 - nv15));
        let eq79_e1227_d_n21: f64 = (eq79_e1225_d_n21 * (nv16 - nv15));
        let eq79_e1227_d_n22: f64 = (eq79_e1225_d_n22 * (nv16 - nv15));
        let eq79_e1228: f64 = (eq79_e1222 + eq79_e1227);
        let eq79_e1228_d_n0: f64 = (eq79_e1222_d_n0 + eq79_e1227_d_n0);
        let eq79_e1228_d_n1: f64 = (eq79_e1222_d_n1 + eq79_e1227_d_n1);
        let eq79_e1228_d_n2: f64 = (eq79_e1222_d_n2 + eq79_e1227_d_n2);
        let eq79_e1228_d_n3: f64 = (eq79_e1222_d_n3 + eq79_e1227_d_n3);
        let eq79_e1228_d_n4: f64 = (eq79_e1222_d_n4 + eq79_e1227_d_n4);
        let eq79_e1228_d_n5: f64 = (eq79_e1222_d_n5 + eq79_e1227_d_n5);
        let eq79_e1228_d_n6: f64 = (eq79_e1222_d_n6 + eq79_e1227_d_n6);
        let eq79_e1228_d_n7: f64 = (eq79_e1222_d_n7 + eq79_e1227_d_n7);
        let eq79_e1228_d_n8: f64 = (eq79_e1222_d_n8 + eq79_e1227_d_n8);
        let eq79_e1228_d_n9: f64 = (eq79_e1222_d_n9 + eq79_e1227_d_n9);
        let eq79_e1228_d_n10: f64 = (eq79_e1222_d_n10 + eq79_e1227_d_n10);
        let eq79_e1228_d_n11: f64 = (eq79_e1222_d_n11 + eq79_e1227_d_n11);
        let eq79_e1228_d_n12: f64 = (eq79_e1222_d_n12 + eq79_e1227_d_n12);
        let eq79_e1228_d_n13: f64 = (eq79_e1222_d_n13 + eq79_e1227_d_n13);
        let eq79_e1228_d_n14: f64 = (eq79_e1222_d_n14 + eq79_e1227_d_n14);
        let eq79_e1228_d_n15: f64 = (eq79_e1222_d_n15 + eq79_e1227_d_n15);
        let eq79_e1228_d_n16: f64 = (eq79_e1222_d_n16 + eq79_e1227_d_n16);
        let eq79_e1228_d_n17: f64 = (eq79_e1222_d_n17 + eq79_e1227_d_n17);
        let eq79_e1228_d_n18: f64 = (eq79_e1222_d_n18 + eq79_e1227_d_n18);
        let eq79_e1228_d_n19: f64 = (eq79_e1222_d_n19 + eq79_e1227_d_n19);
        let eq79_e1228_d_n20: f64 = (eq79_e1222_d_n20 + eq79_e1227_d_n20);
        let eq79_e1228_d_n21: f64 = (eq79_e1222_d_n21 + eq79_e1227_d_n21);
        let eq79_e1228_d_n22: f64 = (eq79_e1222_d_n22 + eq79_e1227_d_n22);
        (eq79_e1228, eq79_e1228_d_n0, eq79_e1228_d_n1, eq79_e1228_d_n2, eq79_e1228_d_n3, eq79_e1228_d_n4, eq79_e1228_d_n5, eq79_e1228_d_n6, eq79_e1228_d_n7, eq79_e1228_d_n8, eq79_e1228_d_n9, eq79_e1228_d_n10, eq79_e1228_d_n11, eq79_e1228_d_n12, eq79_e1228_d_n13, eq79_e1228_d_n14, eq79_e1228_d_n15, eq79_e1228_d_n16, eq79_e1228_d_n17, eq79_e1228_d_n18, eq79_e1228_d_n19, eq79_e1228_d_n20, eq79_e1228_d_n21, eq79_e1228_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq79_value: f64 = eq79_e1230;
        let eq79_node_derivatives: [f64; 23] = [eq79_e1230_d_n0, eq79_e1230_d_n1, eq79_e1230_d_n2, eq79_e1230_d_n3, eq79_e1230_d_n4, eq79_e1230_d_n5, eq79_e1230_d_n6, eq79_e1230_d_n7, eq79_e1230_d_n8, eq79_e1230_d_n9, eq79_e1230_d_n10, eq79_e1230_d_n11, eq79_e1230_d_n12, eq79_e1230_d_n13, eq79_e1230_d_n14, eq79_e1230_d_n15, eq79_e1230_d_n16, eq79_e1230_d_n17, eq79_e1230_d_n18, eq79_e1230_d_n19, eq79_e1230_d_n20, eq79_e1230_d_n21, eq79_e1230_d_n22];
        let eq79_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[16]),
            Some(nodes[15]),
            multiplicity * (eq79_value),
            nodes,
            &eq79_node_derivatives,
            branches,
            &eq79_branch_derivatives,
            multiplicity,
        );
        let (eq80_e1237,) = {
    if (s.b[463] && (!s.b[464])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq80_value: f64 = eq80_e1237;
        stamper.stamp_potential_const(
            branches[38],
            eq80_value,
        );
        let (eq81_e1242,) = {
    if (!s.b[463]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq81_value: f64 = eq81_e1242;
        stamper.stamp_potential_const(
            branches[39],
            eq81_value,
        );
        let (eq82_e1258, eq82_e1258_d_n0, eq82_e1258_d_n1, eq82_e1258_d_n2, eq82_e1258_d_n3, eq82_e1258_d_n4, eq82_e1258_d_n5, eq82_e1258_d_n6, eq82_e1258_d_n7, eq82_e1258_d_n8, eq82_e1258_d_n9, eq82_e1258_d_n10, eq82_e1258_d_n11, eq82_e1258_d_n12, eq82_e1258_d_n13, eq82_e1258_d_n14, eq82_e1258_d_n15, eq82_e1258_d_n16, eq82_e1258_d_n17, eq82_e1258_d_n18, eq82_e1258_d_n19, eq82_e1258_d_n20, eq82_e1258_d_n21, eq82_e1258_d_n22,) = {
    if (s.b[478] && s.b[479]) {
        let eq82_e1248: f64 = (p.p6 * s.v[60]);
        let eq82_e1248_d_n0: f64 = (p.p6 * s.dn[60][0]);
        let eq82_e1248_d_n1: f64 = (p.p6 * s.dn[60][1]);
        let eq82_e1248_d_n2: f64 = (p.p6 * s.dn[60][2]);
        let eq82_e1248_d_n3: f64 = (p.p6 * s.dn[60][3]);
        let eq82_e1248_d_n4: f64 = (p.p6 * s.dn[60][4]);
        let eq82_e1248_d_n5: f64 = (p.p6 * s.dn[60][5]);
        let eq82_e1248_d_n6: f64 = (p.p6 * s.dn[60][6]);
        let eq82_e1248_d_n7: f64 = (p.p6 * s.dn[60][7]);
        let eq82_e1248_d_n8: f64 = (p.p6 * s.dn[60][8]);
        let eq82_e1248_d_n9: f64 = (p.p6 * s.dn[60][9]);
        let eq82_e1248_d_n10: f64 = (p.p6 * s.dn[60][10]);
        let eq82_e1248_d_n11: f64 = (p.p6 * s.dn[60][11]);
        let eq82_e1248_d_n12: f64 = (p.p6 * s.dn[60][12]);
        let eq82_e1248_d_n13: f64 = (p.p6 * s.dn[60][13]);
        let eq82_e1248_d_n14: f64 = (p.p6 * s.dn[60][14]);
        let eq82_e1248_d_n15: f64 = (p.p6 * s.dn[60][15]);
        let eq82_e1248_d_n16: f64 = (p.p6 * s.dn[60][16]);
        let eq82_e1248_d_n17: f64 = (p.p6 * s.dn[60][17]);
        let eq82_e1248_d_n18: f64 = (p.p6 * s.dn[60][18]);
        let eq82_e1248_d_n19: f64 = (p.p6 * s.dn[60][19]);
        let eq82_e1248_d_n20: f64 = (p.p6 * s.dn[60][20]);
        let eq82_e1248_d_n21: f64 = (p.p6 * s.dn[60][21]);
        let eq82_e1248_d_n22: f64 = (p.p6 * s.dn[60][22]);
        let eq82_e1250: f64 = (eq82_e1248 * s.v[269]);
        let eq82_e1250_d_n0: f64 = ((eq82_e1248_d_n0 * s.v[269]) + (eq82_e1248 * s.dn[269][0]));
        let eq82_e1250_d_n1: f64 = ((eq82_e1248_d_n1 * s.v[269]) + (eq82_e1248 * s.dn[269][1]));
        let eq82_e1250_d_n2: f64 = ((eq82_e1248_d_n2 * s.v[269]) + (eq82_e1248 * s.dn[269][2]));
        let eq82_e1250_d_n3: f64 = ((eq82_e1248_d_n3 * s.v[269]) + (eq82_e1248 * s.dn[269][3]));
        let eq82_e1250_d_n4: f64 = ((eq82_e1248_d_n4 * s.v[269]) + (eq82_e1248 * s.dn[269][4]));
        let eq82_e1250_d_n5: f64 = ((eq82_e1248_d_n5 * s.v[269]) + (eq82_e1248 * s.dn[269][5]));
        let eq82_e1250_d_n6: f64 = ((eq82_e1248_d_n6 * s.v[269]) + (eq82_e1248 * s.dn[269][6]));
        let eq82_e1250_d_n7: f64 = ((eq82_e1248_d_n7 * s.v[269]) + (eq82_e1248 * s.dn[269][7]));
        let eq82_e1250_d_n8: f64 = ((eq82_e1248_d_n8 * s.v[269]) + (eq82_e1248 * s.dn[269][8]));
        let eq82_e1250_d_n9: f64 = ((eq82_e1248_d_n9 * s.v[269]) + (eq82_e1248 * s.dn[269][9]));
        let eq82_e1250_d_n10: f64 = ((eq82_e1248_d_n10 * s.v[269]) + (eq82_e1248 * s.dn[269][10]));
        let eq82_e1250_d_n11: f64 = ((eq82_e1248_d_n11 * s.v[269]) + (eq82_e1248 * s.dn[269][11]));
        let eq82_e1250_d_n12: f64 = ((eq82_e1248_d_n12 * s.v[269]) + (eq82_e1248 * s.dn[269][12]));
        let eq82_e1250_d_n13: f64 = ((eq82_e1248_d_n13 * s.v[269]) + (eq82_e1248 * s.dn[269][13]));
        let eq82_e1250_d_n14: f64 = ((eq82_e1248_d_n14 * s.v[269]) + (eq82_e1248 * s.dn[269][14]));
        let eq82_e1250_d_n15: f64 = ((eq82_e1248_d_n15 * s.v[269]) + (eq82_e1248 * s.dn[269][15]));
        let eq82_e1250_d_n16: f64 = ((eq82_e1248_d_n16 * s.v[269]) + (eq82_e1248 * s.dn[269][16]));
        let eq82_e1250_d_n17: f64 = ((eq82_e1248_d_n17 * s.v[269]) + (eq82_e1248 * s.dn[269][17]));
        let eq82_e1250_d_n18: f64 = ((eq82_e1248_d_n18 * s.v[269]) + (eq82_e1248 * s.dn[269][18]));
        let eq82_e1250_d_n19: f64 = ((eq82_e1248_d_n19 * s.v[269]) + (eq82_e1248 * s.dn[269][19]));
        let eq82_e1250_d_n20: f64 = ((eq82_e1248_d_n20 * s.v[269]) + (eq82_e1248 * s.dn[269][20]));
        let eq82_e1250_d_n21: f64 = ((eq82_e1248_d_n21 * s.v[269]) + (eq82_e1248 * s.dn[269][21]));
        let eq82_e1250_d_n22: f64 = ((eq82_e1248_d_n22 * s.v[269]) + (eq82_e1248 * s.dn[269][22]));
        let eq82_e1253: f64 = (p.p6 * s.v[379]);
        let eq82_e1253_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq82_e1253_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq82_e1253_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq82_e1253_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq82_e1253_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq82_e1253_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq82_e1253_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq82_e1253_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq82_e1253_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq82_e1253_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq82_e1253_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq82_e1253_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq82_e1253_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq82_e1253_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq82_e1253_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq82_e1253_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq82_e1253_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq82_e1253_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq82_e1253_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq82_e1253_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq82_e1253_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq82_e1253_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq82_e1253_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq82_e1255: f64 = (eq82_e1253 * (nv19 - nv20));
        let eq82_e1255_d_n0: f64 = (eq82_e1253_d_n0 * (nv19 - nv20));
        let eq82_e1255_d_n1: f64 = (eq82_e1253_d_n1 * (nv19 - nv20));
        let eq82_e1255_d_n2: f64 = (eq82_e1253_d_n2 * (nv19 - nv20));
        let eq82_e1255_d_n3: f64 = (eq82_e1253_d_n3 * (nv19 - nv20));
        let eq82_e1255_d_n4: f64 = (eq82_e1253_d_n4 * (nv19 - nv20));
        let eq82_e1255_d_n5: f64 = (eq82_e1253_d_n5 * (nv19 - nv20));
        let eq82_e1255_d_n6: f64 = (eq82_e1253_d_n6 * (nv19 - nv20));
        let eq82_e1255_d_n7: f64 = (eq82_e1253_d_n7 * (nv19 - nv20));
        let eq82_e1255_d_n8: f64 = (eq82_e1253_d_n8 * (nv19 - nv20));
        let eq82_e1255_d_n9: f64 = (eq82_e1253_d_n9 * (nv19 - nv20));
        let eq82_e1255_d_n10: f64 = (eq82_e1253_d_n10 * (nv19 - nv20));
        let eq82_e1255_d_n11: f64 = (eq82_e1253_d_n11 * (nv19 - nv20));
        let eq82_e1255_d_n12: f64 = (eq82_e1253_d_n12 * (nv19 - nv20));
        let eq82_e1255_d_n13: f64 = (eq82_e1253_d_n13 * (nv19 - nv20));
        let eq82_e1255_d_n14: f64 = (eq82_e1253_d_n14 * (nv19 - nv20));
        let eq82_e1255_d_n15: f64 = (eq82_e1253_d_n15 * (nv19 - nv20));
        let eq82_e1255_d_n16: f64 = (eq82_e1253_d_n16 * (nv19 - nv20));
        let eq82_e1255_d_n17: f64 = (eq82_e1253_d_n17 * (nv19 - nv20));
        let eq82_e1255_d_n18: f64 = (eq82_e1253_d_n18 * (nv19 - nv20));
        let eq82_e1255_d_n19: f64 = ((eq82_e1253_d_n19 * (nv19 - nv20)) + eq82_e1253);
        let eq82_e1255_d_n20: f64 = ((eq82_e1253_d_n20 * (nv19 - nv20)) + (-eq82_e1253));
        let eq82_e1255_d_n21: f64 = (eq82_e1253_d_n21 * (nv19 - nv20));
        let eq82_e1255_d_n22: f64 = (eq82_e1253_d_n22 * (nv19 - nv20));
        let eq82_e1256: f64 = (eq82_e1250 + eq82_e1255);
        let eq82_e1256_d_n0: f64 = (eq82_e1250_d_n0 + eq82_e1255_d_n0);
        let eq82_e1256_d_n1: f64 = (eq82_e1250_d_n1 + eq82_e1255_d_n1);
        let eq82_e1256_d_n2: f64 = (eq82_e1250_d_n2 + eq82_e1255_d_n2);
        let eq82_e1256_d_n3: f64 = (eq82_e1250_d_n3 + eq82_e1255_d_n3);
        let eq82_e1256_d_n4: f64 = (eq82_e1250_d_n4 + eq82_e1255_d_n4);
        let eq82_e1256_d_n5: f64 = (eq82_e1250_d_n5 + eq82_e1255_d_n5);
        let eq82_e1256_d_n6: f64 = (eq82_e1250_d_n6 + eq82_e1255_d_n6);
        let eq82_e1256_d_n7: f64 = (eq82_e1250_d_n7 + eq82_e1255_d_n7);
        let eq82_e1256_d_n8: f64 = (eq82_e1250_d_n8 + eq82_e1255_d_n8);
        let eq82_e1256_d_n9: f64 = (eq82_e1250_d_n9 + eq82_e1255_d_n9);
        let eq82_e1256_d_n10: f64 = (eq82_e1250_d_n10 + eq82_e1255_d_n10);
        let eq82_e1256_d_n11: f64 = (eq82_e1250_d_n11 + eq82_e1255_d_n11);
        let eq82_e1256_d_n12: f64 = (eq82_e1250_d_n12 + eq82_e1255_d_n12);
        let eq82_e1256_d_n13: f64 = (eq82_e1250_d_n13 + eq82_e1255_d_n13);
        let eq82_e1256_d_n14: f64 = (eq82_e1250_d_n14 + eq82_e1255_d_n14);
        let eq82_e1256_d_n15: f64 = (eq82_e1250_d_n15 + eq82_e1255_d_n15);
        let eq82_e1256_d_n16: f64 = (eq82_e1250_d_n16 + eq82_e1255_d_n16);
        let eq82_e1256_d_n17: f64 = (eq82_e1250_d_n17 + eq82_e1255_d_n17);
        let eq82_e1256_d_n18: f64 = (eq82_e1250_d_n18 + eq82_e1255_d_n18);
        let eq82_e1256_d_n19: f64 = (eq82_e1250_d_n19 + eq82_e1255_d_n19);
        let eq82_e1256_d_n20: f64 = (eq82_e1250_d_n20 + eq82_e1255_d_n20);
        let eq82_e1256_d_n21: f64 = (eq82_e1250_d_n21 + eq82_e1255_d_n21);
        let eq82_e1256_d_n22: f64 = (eq82_e1250_d_n22 + eq82_e1255_d_n22);
        (eq82_e1256, eq82_e1256_d_n0, eq82_e1256_d_n1, eq82_e1256_d_n2, eq82_e1256_d_n3, eq82_e1256_d_n4, eq82_e1256_d_n5, eq82_e1256_d_n6, eq82_e1256_d_n7, eq82_e1256_d_n8, eq82_e1256_d_n9, eq82_e1256_d_n10, eq82_e1256_d_n11, eq82_e1256_d_n12, eq82_e1256_d_n13, eq82_e1256_d_n14, eq82_e1256_d_n15, eq82_e1256_d_n16, eq82_e1256_d_n17, eq82_e1256_d_n18, eq82_e1256_d_n19, eq82_e1256_d_n20, eq82_e1256_d_n21, eq82_e1256_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq82_value: f64 = eq82_e1258;
        let eq82_node_derivatives: [f64; 23] = [eq82_e1258_d_n0, eq82_e1258_d_n1, eq82_e1258_d_n2, eq82_e1258_d_n3, eq82_e1258_d_n4, eq82_e1258_d_n5, eq82_e1258_d_n6, eq82_e1258_d_n7, eq82_e1258_d_n8, eq82_e1258_d_n9, eq82_e1258_d_n10, eq82_e1258_d_n11, eq82_e1258_d_n12, eq82_e1258_d_n13, eq82_e1258_d_n14, eq82_e1258_d_n15, eq82_e1258_d_n16, eq82_e1258_d_n17, eq82_e1258_d_n18, eq82_e1258_d_n19, eq82_e1258_d_n20, eq82_e1258_d_n21, eq82_e1258_d_n22];
        let eq82_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[19]),
            Some(nodes[20]),
            multiplicity * (eq82_value),
            nodes,
            &eq82_node_derivatives,
            branches,
            &eq82_branch_derivatives,
            multiplicity,
        );
        let (eq83_e1265,) = {
    if (s.b[478] && (!s.b[479])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq83_value: f64 = eq83_e1265;
        stamper.stamp_potential_const(
            branches[40],
            eq83_value,
        );
        let (eq84_e1270,) = {
    if (!s.b[478]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq84_value: f64 = eq84_e1270;
        stamper.stamp_potential_const(
            branches[41],
            eq84_value,
        );
    }
}
