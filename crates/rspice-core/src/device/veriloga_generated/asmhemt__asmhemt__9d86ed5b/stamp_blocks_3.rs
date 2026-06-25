#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_block_19(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
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

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p195), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p196), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_add(285, 128, 86);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (!(s.v[515] != 0.0))) {
            s.store_add(285, 100, 86);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_scaled_add(286, 284, 285, 0.5);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_sub(287, 285, 284);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_mul_ad_lhs(135, A::add(A::sub(s.ad_value(282), s.ad_value(286)), s.ad_value(83)), 287);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(283), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(282), s.ad_value(286))));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(129))), (s.v[81] / p.p9));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::mul(A::scale(s.ad_value(136), p.p15), s.ad_value(136))), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_scaled_mul(96, 95, 283, (p.p4 * (p.p5 * 1.0 / (p.p187))));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_mul_ad_rhs(98, 96, A::offset(A::scale(A::sub(s.ad_value(292), s.ad_value(86)), p.p21), 1.0));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_sqrt_ad(92, A::offset(A::mul(A::scale(s.ad_value(287), (p.p25 * p.p25)), s.ad_value(287)), 1.0));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_div(93, 98, 92);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_sub(90, 285, 284);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(282), s.ad_value(83)), 286);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(283), (p.p4 * (p.p5 * p.p187))), A::add(A::sub(s.ad_value(282), s.ad_value(286)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p242)), 1e26);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p241), 1.0);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_div_from_scalar(190, p.p240, 189);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p186));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_mul_ad(288, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p187))), A::add(A::sub(s.ad_value(282), s.ad_value(286)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(282), s.ad_value(83)), 286);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(284), A::scale(s.ad_value(285), 2.0)), 0.3333333333333333);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(287)), (1.0 / 12.0)), 136);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(287)), s.ad_value(287)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_mul_ad(289, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p187 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(282), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        s.v[516] = if (s.v[68] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[516] != 0.0)) {
            s.store_sub_ad_lhs(289, A::scale(s.ad_value(288), (-1.0)), 289);
        }

        if ((s.v[508] != 0.0) && (!(s.v[509] != 0.0))) {
            s.store_scalar(288, 0.0);
        }

        if ((s.v[508] != 0.0) && (!(s.v[509] != 0.0))) {
            s.store_scalar(289, 0.0);
        }

        s.v[517] = if (p.p155 != 0.0) { 1.0 } else { 0.0 };

        s.v[518] = if (p.p155 == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[518] != 0.0)) {
            s.store_ad(70, &A::voltage(ctx, &nodes, Some(9), Some(8)));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (!(s.v[518] != 0.0))) {
            s.store_ad(70, &A::voltage(ctx, &nodes, Some(2), Some(8)));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.copy_ad(290, 70);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_scalar(146, (1.0 + p.p191));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_offset_ad(88, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p188), p.p185);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_scalar(283, (p.p9 / p.p186));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p187, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p184))));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(290), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(290), s.ad_value(159)), A::sub(s.ad_value(290), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_sub(282, 160, 88);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_div_ad_rhs(84, 283, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_scale(99, 283, 6.241509074460763e18);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(282), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(282)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            let assign23430_ad_e36747: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p195 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0))));
            s.store_ad(152, &assign23430_ad_e36747);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_div_ad_rhs(136, 282, A::scale(s.ad_value(83), 2.0));
        }

        s.v[519] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[519] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[519] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[519] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(282), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (!(s.v[519] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(282), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_sub_ad_rhs(100, 282, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[520] = if ((((s.v[100] - s.v[282])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_sub(101, 282, 100);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p195), 90);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p196), 90);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
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

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p195), 91);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p196), 91);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_sub(115, 282, 114);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p195), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p196), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
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

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p195), 137);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p196), 137);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

    }

    pub(super) fn stamp_reactive_block_20(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[520] != 0.0)) {
            s.copy_ad(284, 128);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (!(s.v[520] != 0.0))) {
            s.copy_ad(284, 100);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_scalar(291, 0.0);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p189);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p190);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(283), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(282), s.ad_value(284))));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(284))), (s.v[81] / p.p9));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(282), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(282)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p187), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p187), s.ad_value(90)));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(291), s.ad_value(85)), p.p18);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_mul(86, 291, 90);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_sub(39, 282, 86);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            let assign24060_ad_e37843: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p195 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0))));
            s.store_ad(152, &assign24060_ad_e37843);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[521] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[521] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[521] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[521] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (!(s.v[521] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[522] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p195), 90);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p196), 90);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
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

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p195), 91);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p196), 91);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p195), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p196), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
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

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p195), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p196), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_add(285, 128, 86);
        }

        if (((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) && (!(s.v[522] != 0.0))) {
            s.store_add(285, 100, 86);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_scaled_add(286, 284, 285, 0.5);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_sub(287, 285, 284);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_sub(90, 285, 284);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(282), s.ad_value(83)), 286);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(283), (p.p4 * (p.p5 * p.p187))), A::add(A::sub(s.ad_value(282), s.ad_value(286)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p242)), 1e26);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p241), 1.0);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_div_from_scalar(190, p.p240, 189);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p186));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_mul_ad(288, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p187))), A::add(A::sub(s.ad_value(282), s.ad_value(286)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(282), s.ad_value(83)), 286);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(284), A::scale(s.ad_value(285), 2.0)), 0.3333333333333333);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(287)), (1.0 / 12.0)), 136);
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(287)), s.ad_value(287)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((!(s.v[508] != 0.0)) && (s.v[517] != 0.0)) {
            s.store_mul_ad(289, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p187 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(282), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        if ((!(s.v[508] != 0.0)) && (!(s.v[517] != 0.0))) {
            s.store_scalar(288, 0.0);
        }

        if ((!(s.v[508] != 0.0)) && (!(s.v[517] != 0.0))) {
            s.store_scalar(289, 0.0);
        }

        s.v[523] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[524] = if (p.p156 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_ad(73, &A::voltage(ctx, &nodes, Some(18), Some(17)));
        }

        s.v[525] = if (p.p156 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[525] != 0.0)) {
            s.store_ad(74, &A::voltage(ctx, &nodes, Some(9), Some(17)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[525] != 0.0)) {
            s.store_ad(75, &A::voltage(ctx, &nodes, Some(9), Some(18)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (!(s.v[525] != 0.0))) {
            s.store_ad(74, &A::voltage(ctx, &nodes, Some(2), Some(17)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (!(s.v[525] != 0.0))) {
            s.store_ad(75, &A::voltage(ctx, &nodes, Some(2), Some(18)));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_scalar(72, 1.0);
        }

        s.v[526] = if (s.v[73] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[526] != 0.0)) {
            s.store_scalar(72, (-1.0));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[526] != 0.0)) {
            s.store_mul(303, 72, 73);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[526] != 0.0)) {
            s.copy_ad(302, 75);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (!(s.v[526] != 0.0))) {
            s.copy_ad(303, 73);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (!(s.v[526] != 0.0))) {
            s.copy_ad(302, 74);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_offset_ad(304, A::sqrt(A::offset(A::square(s.ad_value(303)), 0.01)), (-0.1));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_offset_scaled(146, 304, p.p205, (1.0 + p.p204));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_sub_ad(88, A::sub_from_scalar(p.p198, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p201)), A::div(A::scale(s.ad_value(304), (p.p207 * p.p206)), A::sqrt(A::offset(A::square(s.ad_value(304)), (p.p207 * p.p207)))));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_scalar(295, (p.p9 / p.p199));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p200, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p197))));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(302), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(302), s.ad_value(159)), A::sub(s.ad_value(302), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_sub(294, 160, 88);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_div_ad_rhs(84, 295, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_scale(99, 295, 6.241509074460763e18);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(294), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(294)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

    }

    pub(super) fn stamp_reactive_block_21(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            let assign24980_ad_e39249: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p208 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0))));
            s.store_ad(152, &assign24980_ad_e39249);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_div_ad_rhs(136, 294, A::scale(s.ad_value(83), 2.0));
        }

        s.v[527] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[527] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[527] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[527] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(294), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (!(s.v[527] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(294), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_sub_ad_rhs(100, 294, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[528] = if ((((s.v[100] - s.v[294])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_sub(101, 294, 100);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p208), 90);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p209), 90);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
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

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p208), 91);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p209), 91);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_sub(115, 294, 114);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p208), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p209), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
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

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p208), 137);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p209), 137);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[528] != 0.0)) {
            s.copy_ad(296, 128);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (!(s.v[528] != 0.0))) {
            s.copy_ad(296, 100);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p202);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p203);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(295), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(294), s.ad_value(296))));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(296))), (s.v[81] / p.p9));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(294), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(294)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p200), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p200), s.ad_value(90)));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(303), s.ad_value(85)), p.p18);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_mul(86, 303, 90);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_sub(39, 294, 86);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            let assign25600_ad_e40278: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p208 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0))));
            s.store_ad(152, &assign25600_ad_e40278);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[529] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[529] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[529] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[529] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (!(s.v[529] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[530] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p208), 90);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p209), 90);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
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

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p208), 91);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p209), 91);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p208), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p209), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

    }

    pub(super) fn stamp_reactive_block_22(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
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

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p208), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p209), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_add(297, 128, 86);
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (!(s.v[530] != 0.0))) {
            s.store_add(297, 100, 86);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_scaled_add(298, 296, 297, 0.5);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_sub(299, 297, 296);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_mul_ad_lhs(135, A::add(A::sub(s.ad_value(294), s.ad_value(298)), s.ad_value(83)), 299);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(295), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(294), s.ad_value(298))));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(129))), (s.v[81] / p.p9));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::mul(A::scale(s.ad_value(136), p.p15), s.ad_value(136))), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_scaled_mul(96, 95, 295, (p.p4 * (p.p5 * 1.0 / (p.p200))));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_mul_ad_rhs(98, 96, A::offset(A::scale(A::sub(s.ad_value(304), s.ad_value(86)), p.p21), 1.0));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_sqrt_ad(92, A::offset(A::mul(A::scale(s.ad_value(299), (p.p25 * p.p25)), s.ad_value(299)), 1.0));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_div(93, 98, 92);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_sub(90, 297, 296);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(294), s.ad_value(83)), 298);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(295), (p.p4 * (p.p5 * p.p200))), A::add(A::sub(s.ad_value(294), s.ad_value(298)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p245)), 1e26);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p244), 1.0);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_div_from_scalar(190, p.p243, 189);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p199));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_mul_ad(300, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p200))), A::add(A::sub(s.ad_value(294), s.ad_value(298)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(294), s.ad_value(83)), 298);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(296), A::scale(s.ad_value(297), 2.0)), 0.3333333333333333);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(299)), (1.0 / 12.0)), 136);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(299)), s.ad_value(299)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_mul_ad(301, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p200 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(294), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        s.v[531] = if (s.v[72] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[531] != 0.0)) {
            s.store_sub_ad_lhs(301, A::scale(s.ad_value(300), (-1.0)), 301);
        }

        if ((s.v[523] != 0.0) && (!(s.v[524] != 0.0))) {
            s.store_scalar(300, 0.0);
        }

        if ((s.v[523] != 0.0) && (!(s.v[524] != 0.0))) {
            s.store_scalar(301, 0.0);
        }

        s.v[532] = if (p.p156 != 0.0) { 1.0 } else { 0.0 };

        s.v[533] = if (p.p156 == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[533] != 0.0)) {
            s.store_ad(74, &A::voltage(ctx, &nodes, Some(9), Some(7)));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (!(s.v[533] != 0.0))) {
            s.store_ad(74, &A::voltage(ctx, &nodes, Some(2), Some(7)));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.copy_ad(302, 74);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_scalar(146, (1.0 + p.p204));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_sub_from_scalar_ad(88, p.p198, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p201));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_scalar(295, (p.p9 / p.p199));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p200, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p197))));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(302), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(302), s.ad_value(159)), A::sub(s.ad_value(302), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_sub(294, 160, 88);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_div_ad_rhs(84, 295, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_scale(99, 295, 6.241509074460763e18);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(294), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(294)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            let assign26520_ad_e41676: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p208 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0))));
            s.store_ad(152, &assign26520_ad_e41676);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_div_ad_rhs(136, 294, A::scale(s.ad_value(83), 2.0));
        }

        s.v[534] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[534] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[534] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[534] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(294), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (!(s.v[534] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(294), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_sub_ad_rhs(100, 294, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[535] = if ((((s.v[100] - s.v[294])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_sub(101, 294, 100);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p208), 90);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p209), 90);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
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

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p208), 91);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p209), 91);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_sub(115, 294, 114);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p208), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p209), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
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

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p208), 137);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p209), 137);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

    }

    pub(super) fn stamp_reactive_block_23(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.copy_ad(296, 128);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (!(s.v[535] != 0.0))) {
            s.copy_ad(296, 100);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_scalar(303, 0.0);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p202);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p203);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(295), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(294), s.ad_value(296))));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(296))), (s.v[81] / p.p9));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(294), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(294)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p200), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p200), s.ad_value(90)));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(303), s.ad_value(85)), p.p18);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_mul(86, 303, 90);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_sub(39, 294, 86);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            let assign27150_ad_e42772: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p208 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0))));
            s.store_ad(152, &assign27150_ad_e42772);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[536] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[536] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[536] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[536] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (!(s.v[536] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[537] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p208), 90);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p209), 90);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
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

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p208), 91);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p209), 91);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p208), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p209), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
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

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p208), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p209), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_add(297, 128, 86);
        }

        if (((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) && (!(s.v[537] != 0.0))) {
            s.store_add(297, 100, 86);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_scaled_add(298, 296, 297, 0.5);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_sub(299, 297, 296);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_sub(90, 297, 296);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(294), s.ad_value(83)), 298);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(295), (p.p4 * (p.p5 * p.p200))), A::add(A::sub(s.ad_value(294), s.ad_value(298)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p245)), 1e26);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p244), 1.0);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_div_from_scalar(190, p.p243, 189);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p199));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_mul_ad(300, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p200))), A::add(A::sub(s.ad_value(294), s.ad_value(298)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(294), s.ad_value(83)), 298);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(296), A::scale(s.ad_value(297), 2.0)), 0.3333333333333333);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(299)), (1.0 / 12.0)), 136);
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(299)), s.ad_value(299)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((!(s.v[523] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_mul_ad(301, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p200 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(294), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        if ((!(s.v[523] != 0.0)) && (!(s.v[532] != 0.0))) {
            s.store_scalar(300, 0.0);
        }

        if ((!(s.v[523] != 0.0)) && (!(s.v[532] != 0.0))) {
            s.store_scalar(301, 0.0);
        }

        s.v[538] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[539] = if (p.p157 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_ad(77, &A::voltage(ctx, &nodes, Some(21), Some(22)));
        }

        s.v[540] = if (p.p157 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[540] != 0.0)) {
            s.store_ad(78, &A::voltage(ctx, &nodes, Some(9), Some(22)));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[540] != 0.0)) {
            s.store_ad(79, &A::voltage(ctx, &nodes, Some(9), Some(21)));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (!(s.v[540] != 0.0))) {
            s.store_ad(78, &A::voltage(ctx, &nodes, Some(2), Some(22)));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (!(s.v[540] != 0.0))) {
            s.store_ad(79, &A::voltage(ctx, &nodes, Some(2), Some(21)));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_scalar(76, 1.0);
        }

        s.v[541] = if (s.v[77] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[541] != 0.0)) {
            s.store_scalar(76, (-1.0));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[541] != 0.0)) {
            s.store_mul(315, 76, 77);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[541] != 0.0)) {
            s.copy_ad(314, 79);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.copy_ad(315, 77);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.copy_ad(314, 78);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_offset_ad(316, A::sqrt(A::offset(A::square(s.ad_value(315)), 0.01)), (-0.1));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_offset_scaled(146, 316, p.p205, (1.0 + p.p204));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_sub_ad(88, A::offset(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p201), p.p198), A::div(A::scale(s.ad_value(316), (p.p207 * p.p206)), A::sqrt(A::offset(A::square(s.ad_value(316)), (p.p207 * p.p207)))));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_scalar(307, (p.p9 / p.p199));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p200, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p197))));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(314), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(314), s.ad_value(159)), A::sub(s.ad_value(314), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_sub(306, 160, 88);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_div_ad_rhs(84, 307, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_scale(99, 307, 6.241509074460763e18);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(306), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(306)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

    }

    pub(super) fn stamp_reactive_block_24(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            let assign28070_ad_e44178: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p208 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0))));
            s.store_ad(152, &assign28070_ad_e44178);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_div_ad_rhs(136, 306, A::scale(s.ad_value(83), 2.0));
        }

        s.v[542] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[542] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[542] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[542] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(306), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (!(s.v[542] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(306), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_sub_ad_rhs(100, 306, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[543] = if ((((s.v[100] - s.v[306])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_sub(101, 306, 100);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p208), 90);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p209), 90);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
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

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p208), 91);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p209), 91);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_sub(115, 306, 114);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p208), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p209), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
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

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p208), 137);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p209), 137);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[543] != 0.0)) {
            s.copy_ad(308, 128);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (!(s.v[543] != 0.0))) {
            s.copy_ad(308, 100);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p202);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p203);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(307), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(306), s.ad_value(308))));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(308))), (s.v[81] / p.p9));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(306), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(306)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p200), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p200), s.ad_value(90)));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(315), s.ad_value(85)), p.p18);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_mul(86, 315, 90);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_sub(39, 306, 86);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            let assign28690_ad_e45207: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p208 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0))));
            s.store_ad(152, &assign28690_ad_e45207);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[544] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (!(s.v[544] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[545] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p208), 90);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p209), 90);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
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

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p208), 91);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p209), 91);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p208), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p209), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

    }

    pub(super) fn stamp_reactive_block_25(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
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

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p208), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p209), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_add(309, 128, 86);
        }

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (!(s.v[545] != 0.0))) {
            s.store_add(309, 100, 86);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_scaled_add(310, 308, 309, 0.5);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_sub(311, 309, 308);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_mul_ad_lhs(135, A::add(A::sub(s.ad_value(306), s.ad_value(310)), s.ad_value(83)), 311);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(307), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(306), s.ad_value(310))));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(129))), (s.v[81] / p.p9));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::mul(A::scale(s.ad_value(136), p.p15), s.ad_value(136))), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_scaled_mul(96, 95, 307, (p.p4 * (p.p5 * 1.0 / (p.p200))));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_mul_ad_rhs(98, 96, A::offset(A::scale(A::sub(s.ad_value(316), s.ad_value(86)), p.p21), 1.0));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_sqrt_ad(92, A::offset(A::mul(A::scale(s.ad_value(311), (p.p25 * p.p25)), s.ad_value(311)), 1.0));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_div(93, 98, 92);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_sub(90, 309, 308);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(306), s.ad_value(83)), 310);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(307), (p.p4 * (p.p5 * p.p200))), A::add(A::sub(s.ad_value(306), s.ad_value(310)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p245)), 1e26);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p244), 1.0);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_div_from_scalar(190, p.p243, 189);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p199));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_mul_ad(312, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p200))), A::add(A::sub(s.ad_value(306), s.ad_value(310)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(306), s.ad_value(83)), 310);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(308), A::scale(s.ad_value(309), 2.0)), 0.3333333333333333);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(311)), (1.0 / 12.0)), 136);
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(311)), s.ad_value(311)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((s.v[538] != 0.0) && (s.v[539] != 0.0)) {
            s.store_mul_ad(313, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p200 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(306), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        s.v[546] = if (s.v[76] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[538] != 0.0) && (s.v[539] != 0.0)) && (s.v[546] != 0.0)) {
            s.store_sub_ad_lhs(313, A::scale(s.ad_value(312), (-1.0)), 313);
        }

        if ((s.v[538] != 0.0) && (!(s.v[539] != 0.0))) {
            s.store_scalar(312, 0.0);
        }

        if ((s.v[538] != 0.0) && (!(s.v[539] != 0.0))) {
            s.store_scalar(313, 0.0);
        }

        s.v[547] = if (p.p157 != 0.0) { 1.0 } else { 0.0 };

        s.v[548] = if (p.p157 == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[548] != 0.0)) {
            s.store_ad(78, &A::voltage(ctx, &nodes, Some(9), Some(8)));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (!(s.v[548] != 0.0))) {
            s.store_ad(78, &A::voltage(ctx, &nodes, Some(2), Some(8)));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.copy_ad(314, 78);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_scalar(146, (1.0 + p.p204));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_offset_ad(88, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p201), p.p198);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_scalar(307, (p.p9 / p.p199));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p200, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p197))));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(314), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(314), s.ad_value(159)), A::sub(s.ad_value(314), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_sub(306, 160, 88);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_div_ad_rhs(84, 307, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_scale(99, 307, 6.241509074460763e18);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(306), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(306)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            let assign29610_ad_e46605: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p208 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0))));
            s.store_ad(152, &assign29610_ad_e46605);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_div_ad_rhs(136, 306, A::scale(s.ad_value(83), 2.0));
        }

        s.v[549] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[549] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[549] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[549] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(306), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (!(s.v[549] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(306), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_sub_ad_rhs(100, 306, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[550] = if ((((s.v[100] - s.v[306])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sub(101, 306, 100);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p208), 90);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p209), 90);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
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

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p208), 91);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p209), 91);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sub(115, 306, 114);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p208), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p209), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
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

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p208), 137);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p209), 137);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

    }

    pub(super) fn stamp_reactive_block_26(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[550] != 0.0)) {
            s.copy_ad(308, 128);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (!(s.v[550] != 0.0))) {
            s.copy_ad(308, 100);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_scalar(315, 0.0);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p202);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p203);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(307), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(306), s.ad_value(308))));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(308))), (s.v[81] / p.p9));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(306), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(306)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p200), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p200), s.ad_value(90)));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(315), s.ad_value(85)), p.p18);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_mul(86, 315, 90);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_sub(39, 306, 86);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            let assign30240_ad_e47701: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p208 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p208) / 3.0))));
            s.store_ad(152, &assign30240_ad_e47701);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[551] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[551] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[551] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[551] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (!(s.v[551] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[552] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p208), 90);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p209), 90);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
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

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p208), 91);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p209), 91);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p208), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p209), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
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

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p208), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p209), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (s.v[552] != 0.0)) {
            s.store_add(309, 128, 86);
        }

        if (((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) && (!(s.v[552] != 0.0))) {
            s.store_add(309, 100, 86);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_scaled_add(310, 308, 309, 0.5);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_sub(311, 309, 308);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_sub(90, 309, 308);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(306), s.ad_value(83)), 310);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(307), (p.p4 * (p.p5 * p.p200))), A::add(A::sub(s.ad_value(306), s.ad_value(310)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p245)), 1e26);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p244), 1.0);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_div_from_scalar(190, p.p243, 189);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p199));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_mul_ad(312, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p200))), A::add(A::sub(s.ad_value(306), s.ad_value(310)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(306), s.ad_value(83)), 310);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(308), A::scale(s.ad_value(309), 2.0)), 0.3333333333333333);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(311)), (1.0 / 12.0)), 136);
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(311)), s.ad_value(311)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((!(s.v[538] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_mul_ad(313, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p200 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(306), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        if ((!(s.v[538] != 0.0)) && (!(s.v[547] != 0.0))) {
            s.store_scalar(312, 0.0);
        }

        if ((!(s.v[538] != 0.0)) && (!(s.v[547] != 0.0))) {
            s.store_scalar(313, 0.0);
        }

        s.v[558] = if (p.p255 == 2.0) { 1.0 } else { 0.0 };

        if (s.v[558] != 0.0) {
            s.store_ad(162, &A::scale(A::voltage(ctx, &nodes, Some(10), Some(2)), ((p.p4 * p.p5) * p.p210)));
        }

        if (s.v[558] != 0.0) {
            s.store_div_ad(168, A::scale(A::voltage(ctx, &nodes, Some(0), Some(2)), p.p214), A::sqrt(A::offset(A::square(A::voltage(ctx, &nodes, Some(0), Some(2))), (p.p214 * p.p214))));
        }

        if (s.v[558] != 0.0) {
            s.store_scalar(169, (p.p213).min((p.p211 / (2.0 * p.p214))));
        }

        if (s.v[558] != 0.0) {
            s.store_sub_from_scalar_ad(167, ((p.p4 * p.p5) * p.p211), A::mul(A::scale(s.ad_value(169), (p.p4 * p.p5)), s.ad_value(168)));
        }

        if (s.v[558] != 0.0) {
            s.store_ad(163, &A::mul(A::max_with_scalar(s.ad_value(167), 0.0), A::voltage(ctx, &nodes, Some(10), Some(0))));
        }

        if (!(s.v[558] != 0.0)) {
            s.store_ad(162, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(2)), ((p.p4 * p.p5) * p.p210)));
        }

        if (!(s.v[558] != 0.0)) {
            s.store_div_ad(168, A::scale(A::voltage(ctx, &nodes, Some(0), Some(2)), p.p214), A::sqrt(A::offset(A::square(A::voltage(ctx, &nodes, Some(0), Some(2))), (p.p214 * p.p214))));
        }

        if (!(s.v[558] != 0.0)) {
            s.store_scalar(169, (p.p213).min((p.p211 / (2.0 * p.p214))));
        }

        if (!(s.v[558] != 0.0)) {
            s.store_sub_from_scalar_ad(167, ((p.p4 * p.p5) * p.p211), A::mul(A::scale(s.ad_value(169), (p.p4 * p.p5)), s.ad_value(168)));
        }

        if (!(s.v[558] != 0.0)) {
            s.store_ad(163, &A::mul(A::max_with_scalar(s.ad_value(167), 0.0), A::voltage(ctx, &nodes, Some(1), Some(0))));
        }

        s.store_ad(164, &A::scale(A::voltage(ctx, &nodes, Some(0), Some(2)), ((p.p4 * p.p5) * p.p212)));

        s.store_ad(219, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(0)), ((p.p4 * p.p5) * p.p215)));

        s.store_ad(220, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(2)), ((p.p4 * p.p5) * p.p216)));

        s.store_ad(221, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(1)), ((p.p4 * p.p5) * p.p217)));

        s.store_scale_ad(377, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p281)), p.p277);

        s.store_scale_ad(378, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p282)), p.p278);

        s.store_scale(137, 378, (p.p4 * p.p5));

        s.store_scale(137, 377, (p.p4 * p.p5));

        s.v[569] = if (p.p255 == 2.0) { 1.0 } else { 0.0 };

        s.v[570] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[571] = if (p.p150 != 0.0) { 1.0 } else { 0.0 };

        s.v[572] = if (p.p150 == 1.0) { 1.0 } else { 0.0 };

        s.v[573] = if (p.p150 != 0.0) { 1.0 } else { 0.0 };

        s.v[574] = if (p.p150 == 1.0) { 1.0 } else { 0.0 };

        s.v[575] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[576] = if (p.p151 != 0.0) { 1.0 } else { 0.0 };

        s.v[577] = if (p.p151 == 1.0) { 1.0 } else { 0.0 };

        s.v[578] = if (p.p151 != 0.0) { 1.0 } else { 0.0 };

        s.v[579] = if (p.p151 == 1.0) { 1.0 } else { 0.0 };

        s.v[580] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[581] = if (p.p152 != 0.0) { 1.0 } else { 0.0 };

        s.v[582] = if (p.p152 == 1.0) { 1.0 } else { 0.0 };

        s.v[583] = if (p.p152 != 0.0) { 1.0 } else { 0.0 };

        s.v[584] = if (p.p152 == 1.0) { 1.0 } else { 0.0 };

        s.v[585] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[586] = if (p.p153 != 0.0) { 1.0 } else { 0.0 };

        s.v[587] = if (p.p153 == 1.0) { 1.0 } else { 0.0 };

        s.v[588] = if (p.p153 != 0.0) { 1.0 } else { 0.0 };

        s.v[589] = if (p.p153 == 1.0) { 1.0 } else { 0.0 };

        s.v[590] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[591] = if (p.p154 != 0.0) { 1.0 } else { 0.0 };

        s.v[592] = if (p.p154 == 1.0) { 1.0 } else { 0.0 };

        s.v[593] = if (p.p154 != 0.0) { 1.0 } else { 0.0 };

        s.v[594] = if (p.p154 == 1.0) { 1.0 } else { 0.0 };

        s.v[595] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[596] = if (p.p155 != 0.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_27(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[597] = if (p.p155 == 1.0) { 1.0 } else { 0.0 };

        s.v[598] = if (p.p155 != 0.0) { 1.0 } else { 0.0 };

        s.v[599] = if (p.p155 == 1.0) { 1.0 } else { 0.0 };

        s.v[600] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[601] = if (p.p156 != 0.0) { 1.0 } else { 0.0 };

        s.v[602] = if (p.p156 == 1.0) { 1.0 } else { 0.0 };

        s.v[603] = if (p.p156 != 0.0) { 1.0 } else { 0.0 };

        s.v[604] = if (p.p156 == 1.0) { 1.0 } else { 0.0 };

        s.v[605] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[606] = if (p.p157 != 0.0) { 1.0 } else { 0.0 };

        s.v[607] = if (p.p157 == 1.0) { 1.0 } else { 0.0 };

        s.v[608] = if (p.p157 != 0.0) { 1.0 } else { 0.0 };

        s.v[609] = if (p.p157 == 1.0) { 1.0 } else { 0.0 };

        s.store_sub_from_scalar_ad(195, p.p222, A::mul(A::offset(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p227), p.p220), A::voltage(ctx, &nodes, Some(0), Some(2))));

        s.store_scale_ad(195, A::sub(A::offset(s.ad_value(195), 1e-25), A::scale(A::sub(A::offset(s.ad_value(195), 1e-25), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(195), (-1e-25)), A::offset(s.ad_value(195), (-1e-25))), p.p221))), 0.5)), (p.p4 * p.p5));

        let assign32150_ad_e49745: A = A::add(A::offset(A::sub_from_scalar(p.p218, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p226)), 1e-18), A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p218, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p226)), (-1e-18)), A::offset(A::sub_from_scalar(p.p218, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p226)), (-1e-18))), ((0.25 * 1e-19) * 1e-19))));
        s.store_scale_ad(136, assign32150_ad_e49745, 0.5);

        s.store_ad(196, &A::mul(A::scale(s.ad_value(136), (p.p4 * p.p5)), A::voltage(ctx, &nodes, Some(9), Some(2))));

        s.store_ad(197, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(0)), ((p.p4 * p.p5) * p.p219)));

        s.store_scale_ad(136, A::sub_from_scalar(p.p224, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p225)), (1.0 - if ((-((p.p229) as f64).ln()) / p.p228) > 80.0 { 5.540622384e34 * (1.0 + (((-((p.p229) as f64).ln()) / p.p228)) - 80.0) } else if ((-((p.p229) as f64).ln()) / p.p228) < -80.0 { 1.804851387e-35 } else { ((((-((p.p229) as f64).ln()) / p.p228)) as f64).exp() }));

        s.store_div_ad_lhs(90, A::sub(s.ad_value(136), A::voltage(ctx, &nodes, Some(2), Some(0))), 36);

        s.store_sqrt_ad(91, A::offset(A::mul(A::scale(s.ad_value(90), p.p230), s.ad_value(90)), 1.92));

        s.store_scaled_add(137, 90, 91, 0.5);

        s.store_sub_ad_rhs(106, 136, A::mul(s.ad_value(36), s.ad_value(137)));

        s.store_ln_ad(192, A::sub_from_scalar(1.0, A::scale(s.ad_value(106), 1.0 / (p.p224))));

        s.store_scale_ad(193, A::mul(A::scale(A::sub_from_scalar(p.p224, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p225)), p.p223), A::sub_from_scalar(1.0, A::limited_exp(A::scale(s.ad_value(192), (1.0 - p.p228))))), 1.0 / ((1.0 - p.p228)));

        s.store_scale_ad(194, A::add(s.ad_value(193), A::scale(A::sub(A::voltage(ctx, &nodes, Some(2), Some(0)), s.ad_value(106)), (p.p229 * p.p223))), (p.p4 * p.p5));

        s.v[610] = if ((p.p31 == 1.0) && (p.p32 > 0.0)) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_equation_0_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq0_e298,) = {
    if ((s.v[382] != 0.0) && (s.v[383] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq0_value: f64 = eq0_e298;
        stamper.stamp_potential(
            branches[0],
            eq0_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_1_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq1_e302,) = {
    if (s.v[387] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq1_value: f64 = eq1_e302;
        stamper.stamp_potential(
            branches[1],
            eq1_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_2_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq2_e306,) = {
    if (s.v[387] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq2_value: f64 = eq2_e306;
        stamper.stamp_potential(
            branches[2],
            eq2_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_3_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq3_e310,) = {
    if (s.v[387] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq3_value: f64 = eq3_e310;
        stamper.stamp_potential(
            branches[3],
            eq3_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_4_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq4_e314,) = {
    if (s.v[387] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq4_value: f64 = eq4_e314;
        stamper.stamp_potential(
            branches[4],
            eq4_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_5_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq5_e318,) = {
    if (s.v[387] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq5_value: f64 = eq5_e318;
        stamper.stamp_potential(
            branches[5],
            eq5_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_6_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq6_e322,) = {
    if (s.v[387] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq6_value: f64 = eq6_e322;
        stamper.stamp_potential(
            branches[6],
            eq6_value,
            &[
            ],
        );
    }
}
