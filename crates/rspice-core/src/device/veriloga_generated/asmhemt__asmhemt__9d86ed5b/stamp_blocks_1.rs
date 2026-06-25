#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_16(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[495] != 0.0)) {
            s.store_ad(66, &A::voltage(ctx, &nodes, Some(9), Some(16)));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[495] != 0.0)) {
            s.store_ad(67, &A::voltage(ctx, &nodes, Some(9), Some(17)));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (!(s.v[495] != 0.0))) {
            s.store_ad(66, &A::voltage(ctx, &nodes, Some(2), Some(16)));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (!(s.v[495] != 0.0))) {
            s.store_ad(67, &A::voltage(ctx, &nodes, Some(2), Some(17)));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_scalar(64, 1.0);
        }

        s.v[496] = if (s.v[65] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[496] != 0.0)) {
            s.store_scalar(64, (-1.0));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[496] != 0.0)) {
            s.store_mul(279, 64, 65);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[496] != 0.0)) {
            s.copy_ad(278, 67);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (!(s.v[496] != 0.0))) {
            s.copy_ad(279, 65);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (!(s.v[496] != 0.0))) {
            s.copy_ad(278, 66);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_offset_ad(280, A::sqrt(A::offset(A::square(s.ad_value(279)), 0.01)), (-0.1));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_offset_scaled(146, 280, p.p192, (1.0 + p.p191));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_sub_ad(88, A::sub_from_scalar(p.p185, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p188)), A::div(A::scale(s.ad_value(280), (p.p194 * p.p193)), A::sqrt(A::offset(A::square(s.ad_value(280)), (p.p194 * p.p194)))));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_scalar(271, (p.p9 / p.p186));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p187, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p184))));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(278), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(278), s.ad_value(159)), A::sub(s.ad_value(278), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_sub(270, 160, 88);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_div_ad_rhs(84, 271, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_scale(99, 271, 6.241509074460763e18);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(270), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(270)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            let assign18800_ad_e29391: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p195 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0))));
            s.store_ad(152, &assign18800_ad_e29391);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_div_ad_rhs(136, 270, A::scale(s.ad_value(83), 2.0));
        }

        s.v[497] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[497] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[497] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[497] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(270), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (!(s.v[497] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(270), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_sub_ad_rhs(100, 270, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[498] = if ((((s.v[100] - s.v[270])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_sub(101, 270, 100);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p195), 90);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p196), 90);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            let assign18980_ad_e29685: A = {
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
            let assign18980_ad_e29723: A = {
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign18980_ad_e29685)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign18980_ad_e29723));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p195), 91);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p196), 91);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_sub(115, 270, 114);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p195), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p196), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            let assign19140_ad_e29982: A = {
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
            let assign19140_ad_e30020: A = {
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign19140_ad_e29982)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign19140_ad_e30020));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p195), 137);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p196), 137);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[498] != 0.0)) {
            s.copy_ad(272, 128);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (!(s.v[498] != 0.0))) {
            s.copy_ad(272, 100);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p189);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p190);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(271), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(270), s.ad_value(272))));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(272))), (s.v[81] / p.p9));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(270), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(270)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p187), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p187), s.ad_value(90)));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(279), s.ad_value(85)), p.p18);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_mul(86, 279, 90);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_sub(39, 270, 86);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            let assign19420_ad_e30420: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p195 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0))));
            s.store_ad(152, &assign19420_ad_e30420);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[499] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[499] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[499] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[499] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (!(s.v[499] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[500] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p195), 90);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p196), 90);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

    }

    pub(super) fn stamp_transient_block_17(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            let assign19600_ad_e30714: A = {
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
            let assign19600_ad_e30752: A = {
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign19600_ad_e30714)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign19600_ad_e30752));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p195), 91);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p196), 91);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p195), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p196), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            let assign19750_ad_e31000: A = {
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
            let assign19750_ad_e31038: A = {
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign19750_ad_e31000)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign19750_ad_e31038));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p195), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p196), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[500] != 0.0)) {
            s.store_add(273, 128, 86);
        }

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (!(s.v[500] != 0.0))) {
            s.store_add(273, 100, 86);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_scaled_add(274, 272, 273, 0.5);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_sub(275, 273, 272);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_mul_ad_lhs(135, A::add(A::sub(s.ad_value(270), s.ad_value(274)), s.ad_value(83)), 275);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(271), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(270), s.ad_value(274))));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(129))), (s.v[81] / p.p9));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::mul(A::scale(s.ad_value(136), p.p15), s.ad_value(136))), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_scaled_mul(96, 95, 271, (p.p4 * (p.p5 * 1.0 / (p.p187))));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_mul_ad_rhs(98, 96, A::offset(A::scale(A::sub(s.ad_value(280), s.ad_value(86)), p.p21), 1.0));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_sqrt_ad(92, A::offset(A::mul(A::scale(s.ad_value(275), (p.p25 * p.p25)), s.ad_value(275)), 1.0));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_div(93, 98, 92);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_mul(281, 93, 135);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_sub(90, 273, 272);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(270), s.ad_value(83)), 274);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(271), (p.p4 * (p.p5 * p.p187))), A::add(A::sub(s.ad_value(270), s.ad_value(274)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p242)), 1e26);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p241), 1.0);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_div_from_scalar(190, p.p240, 189);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p186));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_mul_ad(276, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p187))), A::add(A::sub(s.ad_value(270), s.ad_value(274)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(270), s.ad_value(83)), 274);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(272), A::scale(s.ad_value(273), 2.0)), 0.3333333333333333);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(275)), (1.0 / 12.0)), 136);
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(275)), s.ad_value(275)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_mul_ad(277, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p187 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(270), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        s.v[501] = if (s.v[64] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[493] != 0.0) && (s.v[494] != 0.0)) && (s.v[501] != 0.0)) {
            s.store_sub_ad_lhs(277, A::scale(s.ad_value(276), (-1.0)), 277);
        }

        if ((s.v[493] != 0.0) && (!(s.v[494] != 0.0))) {
            s.store_scalar(276, 0.0);
        }

        if ((s.v[493] != 0.0) && (!(s.v[494] != 0.0))) {
            s.store_scalar(277, 0.0);
        }

        s.v[502] = if (p.p154 != 0.0) { 1.0 } else { 0.0 };

        s.v[503] = if (p.p154 == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[503] != 0.0)) {
            s.store_ad(66, &A::voltage(ctx, &nodes, Some(9), Some(7)));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (!(s.v[503] != 0.0))) {
            s.store_ad(66, &A::voltage(ctx, &nodes, Some(2), Some(7)));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.copy_ad(278, 66);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_scalar(146, (1.0 + p.p191));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_sub_from_scalar_ad(88, p.p185, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p188));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_scalar(271, (p.p9 / p.p186));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p187, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p184))));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(278), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(278), s.ad_value(159)), A::sub(s.ad_value(278), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_sub(270, 160, 88);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_div_ad_rhs(84, 271, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_scale(99, 271, 6.241509074460763e18);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(270), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(270)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            let assign20340_ad_e31818: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p195 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0))));
            s.store_ad(152, &assign20340_ad_e31818);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_div_ad_rhs(136, 270, A::scale(s.ad_value(83), 2.0));
        }

        s.v[504] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[504] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[504] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[504] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(270), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (!(s.v[504] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(270), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_sub_ad_rhs(100, 270, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[505] = if ((((s.v[100] - s.v[270])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_sub(101, 270, 100);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p195), 90);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p196), 90);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            let assign20520_ad_e32128: A = {
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
            let assign20520_ad_e32166: A = {
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign20520_ad_e32128)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign20520_ad_e32166));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p195), 91);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p196), 91);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

    }

    pub(super) fn stamp_transient_block_18(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_sub(115, 270, 114);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p195), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p196), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            let assign20680_ad_e32441: A = {
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
            let assign20680_ad_e32479: A = {
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign20680_ad_e32441)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign20680_ad_e32479));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p195), 137);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p196), 137);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[505] != 0.0)) {
            s.copy_ad(272, 128);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (!(s.v[505] != 0.0))) {
            s.copy_ad(272, 100);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_scalar(279, 0.0);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p189);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p190);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(271), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(270), s.ad_value(272))));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(272))), (s.v[81] / p.p9));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(270), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(270)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p187), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p187), s.ad_value(90)));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(279), s.ad_value(85)), p.p18);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_mul(86, 279, 90);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_sub(39, 270, 86);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            let assign20970_ad_e32914: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p195 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0))));
            s.store_ad(152, &assign20970_ad_e32914);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[506] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[506] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[506] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[506] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (!(s.v[506] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[507] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p195), 90);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p196), 90);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            let assign21150_ad_e33224: A = {
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
            let assign21150_ad_e33262: A = {
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign21150_ad_e33224)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign21150_ad_e33262));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p195), 91);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p196), 91);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p195), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p196), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            let assign21300_ad_e33525: A = {
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
            let assign21300_ad_e33563: A = {
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign21300_ad_e33525)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign21300_ad_e33563));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p195), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p196), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_add(273, 128, 86);
        }

        if (((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) && (!(s.v[507] != 0.0))) {
            s.store_add(273, 100, 86);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_scaled_add(274, 272, 273, 0.5);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_sub(275, 273, 272);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_sub(90, 273, 272);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(270), s.ad_value(83)), 274);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(271), (p.p4 * (p.p5 * p.p187))), A::add(A::sub(s.ad_value(270), s.ad_value(274)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p242)), 1e26);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p241), 1.0);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_div_from_scalar(190, p.p240, 189);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p186));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_mul_ad(276, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p187))), A::add(A::sub(s.ad_value(270), s.ad_value(274)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(270), s.ad_value(83)), 274);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(272), A::scale(s.ad_value(273), 2.0)), 0.3333333333333333);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(275)), (1.0 / 12.0)), 136);
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(275)), s.ad_value(275)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((!(s.v[493] != 0.0)) && (s.v[502] != 0.0)) {
            s.store_mul_ad(277, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p187 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(270), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        if ((!(s.v[493] != 0.0)) && (!(s.v[502] != 0.0))) {
            s.store_scalar(276, 0.0);
        }

        if ((!(s.v[493] != 0.0)) && (!(s.v[502] != 0.0))) {
            s.store_scalar(277, 0.0);
        }

        s.v[508] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[509] = if (p.p155 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_ad(69, &A::voltage(ctx, &nodes, Some(20), Some(21)));
        }

        s.v[510] = if (p.p155 == 1.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_19(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[510] != 0.0)) {
            s.store_ad(70, &A::voltage(ctx, &nodes, Some(9), Some(21)));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[510] != 0.0)) {
            s.store_ad(71, &A::voltage(ctx, &nodes, Some(9), Some(20)));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (!(s.v[510] != 0.0))) {
            s.store_ad(70, &A::voltage(ctx, &nodes, Some(2), Some(21)));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (!(s.v[510] != 0.0))) {
            s.store_ad(71, &A::voltage(ctx, &nodes, Some(2), Some(20)));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_scalar(68, 1.0);
        }

        s.v[511] = if (s.v[69] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[511] != 0.0)) {
            s.store_scalar(68, (-1.0));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[511] != 0.0)) {
            s.store_mul(291, 68, 69);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[511] != 0.0)) {
            s.copy_ad(290, 71);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (!(s.v[511] != 0.0))) {
            s.copy_ad(291, 69);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (!(s.v[511] != 0.0))) {
            s.copy_ad(290, 70);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_offset_ad(292, A::sqrt(A::offset(A::square(s.ad_value(291)), 0.01)), (-0.1));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_offset_scaled(146, 292, p.p192, (1.0 + p.p191));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_sub_ad(88, A::offset(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p188), p.p185), A::div(A::scale(s.ad_value(292), (p.p194 * p.p193)), A::sqrt(A::offset(A::square(s.ad_value(292)), (p.p194 * p.p194)))));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_scalar(283, (p.p9 / p.p186));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p187, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p184))));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(290), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(290), s.ad_value(159)), A::sub(s.ad_value(290), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_sub(282, 160, 88);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_div_ad_rhs(84, 283, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_scale(99, 283, 6.241509074460763e18);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(282), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(282)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            let assign21890_ad_e34320: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p195 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0))));
            s.store_ad(152, &assign21890_ad_e34320);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_div_ad_rhs(136, 282, A::scale(s.ad_value(83), 2.0));
        }

        s.v[512] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[512] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[512] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[512] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(282), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (!(s.v[512] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(282), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_sub_ad_rhs(100, 282, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[513] = if ((((s.v[100] - s.v[282])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_sub(101, 282, 100);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p195), 90);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p196), 90);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
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

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p195), 91);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p196), 91);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_sub(115, 282, 114);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p195), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p196), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
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

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p195), 137);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p196), 137);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[513] != 0.0)) {
            s.copy_ad(284, 128);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (!(s.v[513] != 0.0))) {
            s.copy_ad(284, 100);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p189);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p190);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(283), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(282), s.ad_value(284))));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(284))), (s.v[81] / p.p9));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(282), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(282)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p187), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p187), s.ad_value(90)));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(291), s.ad_value(85)), p.p18);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_mul(86, 291, 90);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_sub(39, 282, 86);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            let assign22510_ad_e35349: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p195 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p195) / 3.0))));
            s.store_ad(152, &assign22510_ad_e35349);
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[514] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[514] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[514] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[514] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (!(s.v[514] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[515] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p195), 90);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p196), 90);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

    }

    pub(super) fn stamp_transient_block_20(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
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

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p195), 91);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p196), 91);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p195), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p196), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[508] != 0.0) && (s.v[509] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

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
            s.store_mul(293, 93, 135);
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

    }

    pub(super) fn stamp_transient_block_21(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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

    }

    pub(super) fn stamp_transient_block_22(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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

    }

    pub(super) fn stamp_transient_block_23(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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
            s.store_mul(305, 93, 135);
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

    }

    pub(super) fn stamp_transient_block_24(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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

    }

    pub(super) fn stamp_transient_block_25(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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

    }

    pub(super) fn stamp_transient_block_26(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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
            s.store_mul(317, 93, 135);
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

    }

    pub(super) fn stamp_transient_block_27(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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

        s.v[553] = if (p.p255 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[553] != 0.0) {
            s.store_scalar(318, ((p.p258 * (p.p256 + ((p.p4 / 3.0) / p.p257))) / ((p.p257 * p.p5) * p.p3)));
        }

        s.v[554] = if (s.v[318] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[553] != 0.0) && (s.v[554] != 0.0)) {
            s.store_div_from_scalar(318, 1.0, 318);
        }

    }

    pub(super) fn stamp_transient_block_28(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[553] != 0.0) && (!(s.v[554] != 0.0))) {
            s.store_scalar(318, (1.0 / 0.001));
        }

        s.v[555] = if (p.p255 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[553] != 0.0)) && (s.v[555] != 0.0)) {
            s.store_scalar(319, ((p.p258 * (p.p256 + ((p.p4 / 3.0) / p.p257))) / ((p.p257 * p.p5) * p.p3)));
        }

        if ((!(s.v[553] != 0.0)) && (s.v[555] != 0.0)) {
            s.store_scalar(320, ((p.p258 * (((2.0 * p.p4) / 3.0) / p.p257)) / ((p.p257 * p.p5) * p.p3)));
        }

        s.v[556] = if (s.v[319] > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[553] != 0.0)) && (s.v[555] != 0.0)) && (s.v[556] != 0.0)) {
            s.store_div_from_scalar(319, 1.0, 319);
        }

        if (((!(s.v[553] != 0.0)) && (s.v[555] != 0.0)) && (!(s.v[556] != 0.0))) {
            s.store_scalar(319, (1.0 / 0.001));
        }

        s.v[557] = if (s.v[320] > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[553] != 0.0)) && (s.v[555] != 0.0)) && (s.v[557] != 0.0)) {
            s.store_div_from_scalar(320, 1.0, 320);
        }

        if (((!(s.v[553] != 0.0)) && (s.v[555] != 0.0)) && (!(s.v[557] != 0.0))) {
            s.store_scalar(320, (1.0 / 0.001));
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

        s.store_sub(217, 164, 163);

        s.store_sub_ad_lhs(218, A::neg(s.ad_value(162)), 164);

        s.store_add(138, 165, 217);

        s.store_add(139, 166, 218);

        s.store_ad(219, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(0)), ((p.p4 * p.p5) * p.p215)));

        s.store_ad(220, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(2)), ((p.p4 * p.p5) * p.p216)));

        s.store_ad(221, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(1)), ((p.p4 * p.p5) * p.p217)));

        s.store_offset_ad(375, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p285), p.p279);

        s.store_offset_ad(373, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p283), p.p275);

        s.store_scale_ad(377, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p281)), p.p277);

        s.store_offset_ad(376, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p286), p.p280);

        s.store_offset_ad(374, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p284), p.p276);

        s.store_scale_ad(378, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p282)), p.p278);

        s.store_scale(137, 378, (p.p4 * p.p5));

        s.store_max_with_scalar_ad(371, A::sub(A::voltage(ctx, &nodes, Some(0), Some(3)), s.ad_value(376)), 0.0);

        s.v[559] = if (s.v[137] > 0.0) { 1.0 } else { 0.0 };

        s.v[560] = if (s.v[371] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[559] != 0.0) && (s.v[560] != 0.0)) {
            s.store_div_ad(354, A::powf(s.ad_value(371), 1.0), A::mul(s.ad_value(374), s.ad_value(36)));
        }

        s.v[561] = if (s.v[354] > 80.0) { 1.0 } else { 0.0 };

        if (((s.v[559] != 0.0) && (s.v[560] != 0.0)) && (s.v[561] != 0.0)) {
            s.store_offset(355, 354, (((-80.0)) + (1.0)));
        }

        if (((s.v[559] != 0.0) && (s.v[560] != 0.0)) && (s.v[561] != 0.0)) {
            s.store_scalar(354, 80.0);
        }

        if (((s.v[559] != 0.0) && (s.v[560] != 0.0)) && (!(s.v[561] != 0.0))) {
            s.store_scalar(355, 1.0);
        }

        if ((s.v[559] != 0.0) && (s.v[560] != 0.0)) {
            s.store_mul_ad_rhs(355, 355, A::exp(s.ad_value(354)));
        }

        if ((s.v[559] != 0.0) && (s.v[560] != 0.0)) {
            s.store_mul_ad_rhs(369, 137, A::offset(s.ad_value(355), (-1.0)));
        }

        if ((s.v[559] != 0.0) && (!(s.v[560] != 0.0))) {
            s.store_div_ad_rhs(354, 371, A::mul(s.ad_value(374), s.ad_value(36)));
        }

        s.v[562] = if (s.v[354] > 80.0) { 1.0 } else { 0.0 };

        if (((s.v[559] != 0.0) && (!(s.v[560] != 0.0))) && (s.v[562] != 0.0)) {
            s.store_offset(355, 354, (((-80.0)) + (1.0)));
        }

        if (((s.v[559] != 0.0) && (!(s.v[560] != 0.0))) && (s.v[562] != 0.0)) {
            s.store_scalar(354, 80.0);
        }

        if (((s.v[559] != 0.0) && (!(s.v[560] != 0.0))) && (!(s.v[562] != 0.0))) {
            s.store_scalar(355, 1.0);
        }

        if ((s.v[559] != 0.0) && (!(s.v[560] != 0.0))) {
            s.store_mul_ad_rhs(355, 355, A::exp(s.ad_value(354)));
        }

        if ((s.v[559] != 0.0) && (!(s.v[560] != 0.0))) {
            s.store_mul_ad_rhs(369, 137, A::offset(s.ad_value(355), (-1.0)));
        }

        if (!(s.v[559] != 0.0)) {
            s.store_scalar(369, 0.0);
        }

        s.store_max_with_scalar_ad(372, A::sub(A::voltage(ctx, &nodes, Some(2), Some(3)), s.ad_value(375)), 0.0);

        s.store_scale(137, 377, (p.p4 * p.p5));

        s.v[563] = if (s.v[137] > 0.0) { 1.0 } else { 0.0 };

        s.v[564] = if (s.v[372] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[563] != 0.0) && (s.v[564] != 0.0)) {
            s.store_div_ad(354, A::powf(s.ad_value(372), 1.0), A::mul(s.ad_value(373), s.ad_value(36)));
        }

        s.v[565] = if (s.v[354] > 80.0) { 1.0 } else { 0.0 };

        if (((s.v[563] != 0.0) && (s.v[564] != 0.0)) && (s.v[565] != 0.0)) {
            s.store_offset(355, 354, (((-80.0)) + (1.0)));
        }

        if (((s.v[563] != 0.0) && (s.v[564] != 0.0)) && (s.v[565] != 0.0)) {
            s.store_scalar(354, 80.0);
        }

        if (((s.v[563] != 0.0) && (s.v[564] != 0.0)) && (!(s.v[565] != 0.0))) {
            s.store_scalar(355, 1.0);
        }

        if ((s.v[563] != 0.0) && (s.v[564] != 0.0)) {
            s.store_mul_ad_rhs(355, 355, A::exp(s.ad_value(354)));
        }

        if ((s.v[563] != 0.0) && (s.v[564] != 0.0)) {
            s.store_mul_ad_rhs(370, 137, A::offset(s.ad_value(355), (-1.0)));
        }

        if ((s.v[563] != 0.0) && (!(s.v[564] != 0.0))) {
            s.store_div_ad_rhs(354, 372, A::mul(s.ad_value(373), s.ad_value(36)));
        }

        s.v[566] = if (s.v[354] > 80.0) { 1.0 } else { 0.0 };

        if (((s.v[563] != 0.0) && (!(s.v[564] != 0.0))) && (s.v[566] != 0.0)) {
            s.store_offset(355, 354, (((-80.0)) + (1.0)));
        }

        if (((s.v[563] != 0.0) && (!(s.v[564] != 0.0))) && (s.v[566] != 0.0)) {
            s.store_scalar(354, 80.0);
        }

        if (((s.v[563] != 0.0) && (!(s.v[564] != 0.0))) && (!(s.v[566] != 0.0))) {
            s.store_scalar(355, 1.0);
        }

        if ((s.v[563] != 0.0) && (!(s.v[564] != 0.0))) {
            s.store_mul_ad_rhs(355, 355, A::exp(s.ad_value(354)));
        }

        if ((s.v[563] != 0.0) && (!(s.v[564] != 0.0))) {
            s.store_mul_ad_rhs(370, 137, A::offset(s.ad_value(355), (-1.0)));
        }

        if (!(s.v[563] != 0.0)) {
            s.store_scalar(370, 0.0);
        }

        s.v[567] = if (p.p259 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[567] != 0.0) {
            s.store_div_from_scalar_ad(199, p.p3, A::mul(A::add(A::sub(s.ad_value(37), s.ad_value(133)), s.ad_value(83)), A::max_with_scalar(s.ad_value(134), 1e-12)));
        }

        if (s.v[567] != 0.0) {
            s.store_scale(198, 83, (1.602176634e-19 * (1.602176634e-19 * (1.602176634e-19 * 1.0 / ((((p.p4 * p.p5) * p.p3) * p.p3))))));
        }

        if (s.v[567] != 0.0) {
            s.store_mul_ad(200, A::mul(A::scale(s.ad_value(83), (p.p261 * s.v[80])), A::div_from_scalar(1.0, A::max_with_scalar(s.ad_value(138), 1e-22))), A::sub_from_scalar(1.0, A::div(s.ad_value(138), A::max_with_scalar(s.ad_value(139), 1e-22))));
        }

        if (s.v[567] != 0.0) {
            s.store_mul_ad(201, A::offset(A::scale(s.ad_value(83), (p.p262 * s.v[80])), p.p261), A::ln(A::div(A::max_with_scalar(s.ad_value(138), 1e-22), A::max_with_scalar(s.ad_value(139), 1e-22))));
        }

        if (s.v[567] != 0.0) {
            s.store_mul_ad(202, A::offset(A::scale(s.ad_value(83), (p.p263 * s.v[80])), p.p262), A::sub(s.ad_value(139), s.ad_value(138)));
        }

        if (s.v[567] != 0.0) {
            s.store_scale_ad(203, A::sub(A::square(s.ad_value(138)), A::square(s.ad_value(139))), (p.p263 / 2.0));
        }

        if (s.v[567] != 0.0) {
            s.store_mul_ad(204, A::mul(A::mul(s.ad_value(198), A::square(s.ad_value(94))), A::scale(s.ad_value(199), 1.0 / ((s.v[80] * s.v[80])))), A::add(A::add(A::add(s.ad_value(200), s.ad_value(201)), s.ad_value(202)), s.ad_value(203)));
        }

        s.v[568] = if (s.v[41] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[567] != 0.0) && (s.v[568] != 0.0)) {
            s.store_neg(204, 204);
        }

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

    pub(super) fn stamp_reactive_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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

        s.v[384] = if (p.p149 == 1.0) { 1.0 } else { 0.0 };

        s.v[385] = if (s.v[361] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[384] != 0.0) && (s.v[385] != 0.0)) {
            s.store_scalar(361, 1.0);
        }

        s.v[35] = (p.p0 + 273.15);

        s.store_ad(42, &A::voltage(ctx, &nodes, Some(7), Some(8)));

        s.store_ad(43, &A::voltage(ctx, &nodes, Some(9), Some(8)));

        s.store_ad(44, &A::voltage(ctx, &nodes, Some(9), Some(7)));

        s.store_ad(46, &A::voltage(ctx, &nodes, Some(3), Some(8)));

        s.store_ad(47, &A::voltage(ctx, &nodes, Some(3), Some(7)));

        s.v[41] = 1.0;

        s.v[386] = if (s.v[42] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[386] != 0.0) {
            s.store_scalar(41, (-1.0));
        }

        if (s.v[386] != 0.0) {
            s.store_mul(38, 41, 42);
        }

        if (s.v[386] != 0.0) {
            s.copy_ad(40, 44);
        }

        if (s.v[386] != 0.0) {
            s.copy_ad(45, 47);
        }

        if (!(s.v[386] != 0.0)) {
            s.copy_ad(38, 42);
        }

        if (!(s.v[386] != 0.0)) {
            s.copy_ad(40, 43);
        }

        if (!(s.v[386] != 0.0)) {
            s.copy_ad(45, 46);
        }

        s.store_offset_ad(140, A::sqrt(A::offset(A::square(s.ad_value(38)), 0.01)), (-0.1));

        s.store_offset_ad(82, A::offset(A::voltage(ctx, &nodes, Some(4), None), ctx.temperature()), p.p274);

        s.store_scale(36, 82, 8.617087e-5);

        s.v[387] = if (p.p81 == 0.0) { 1.0 } else { 0.0 };

        s.v[388] = if (p.p81 == 1.0) { 1.0 } else { 0.0 };

        s.v[389] = if (p.p81 == 2.0) { 1.0 } else { 0.0 };

        s.v[390] = if (p.p81 == 3.0) { 1.0 } else { 0.0 };

        s.v[391] = if (p.p81 == 4.0) { 1.0 } else { 0.0 };

        s.v[392] = if (p.p81 == 5.0) { 1.0 } else { 0.0 };

        if ((s.v[388] != 0.0) && (!(s.v[387] != 0.0))) {
            s.store_ad(186, &A::voltage(ctx, &nodes, Some(5), None));
        }

        if ((s.v[388] != 0.0) && (!(s.v[387] != 0.0))) {
            s.store_scale_ad(186, A::add(A::add(s.ad_value(186), s.ad_value(36)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(186), s.ad_value(36)), A::sub(s.ad_value(186), s.ad_value(36))), ((0.25 * p.p128) * p.p128)))), 0.5);
        }

        if ((s.v[388] != 0.0) && (!(s.v[387] != 0.0))) {
            s.store_offset_ad(213, A::scale(A::limited_exp(A::div_from_scalar((-1.0), s.ad_value(186))), p.p101), p.p100);
        }

        if ((s.v[388] != 0.0) && (!(s.v[387] != 0.0))) {
            s.store_offset_ad(216, A::scale(A::limited_exp(A::div_from_scalar((-1.0), s.ad_value(186))), p.p103), p.p102);
        }

        if ((s.v[389] != 0.0) && (!((s.v[387] != 0.0) || (s.v[388] != 0.0)))) {
            s.store_ad(209, &A::scale(A::voltage(ctx, &nodes, Some(6), None), p.p113));
        }

        if ((s.v[389] != 0.0) && (!((s.v[387] != 0.0) || (s.v[388] != 0.0)))) {
            s.store_ad(211, &A::scale(A::voltage(ctx, &nodes, Some(6), None), p.p114));
        }

        if ((s.v[389] != 0.0) && (!((s.v[387] != 0.0) || (s.v[388] != 0.0)))) {
            s.store_ad(212, &A::scale(A::voltage(ctx, &nodes, Some(6), None), p.p115));
        }

        if ((s.v[390] != 0.0) && (!(((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)))) {
            s.store_ad(147, &A::voltage(ctx, &nodes, Some(0), Some(1)));
        }

        if ((s.v[390] != 0.0) && (!(((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)))) {
            s.store_mul_ad_lhs(90, A::div_from_scalar(p.p124, A::offset(A::scale(s.ad_value(147), p.p123), 1.0)), 147);
        }

        if ((s.v[390] != 0.0) && (!(((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)))) {
            s.store_scaled_offset(91, 147, (-p.p127), p.p125);
        }

        if ((s.v[390] != 0.0) && (!(((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)))) {
            s.store_exp_ad(136, A::scale(A::offset(A::voltage(ctx, &nodes, Some(1), Some(2)), (-p.p10)), ((-2.0) * 1.0 / (p.p122))));
        }

        if ((s.v[390] != 0.0) && (!(((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)))) {
            s.store_offset_ad(149, A::scale(A::offset(A::div(A::sub_from_scalar(1.0, s.ad_value(136)), A::offset(s.ad_value(136), 1.0)), 1.0), ((p.p120 - 1e-9) * 0.5)), 1e-9);
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_ad(136, &A::abs(A::voltage(ctx, &nodes, Some(0), Some(2))));
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_ad(90, &A::abs(A::voltage(ctx, &nodes, Some(1), Some(2))));
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_ad(337, &A::sub(A::voltage(ctx, &nodes, Some(12), None), A::abs(A::voltage(ctx, &nodes, Some(0), Some(2)))));
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scale_ad(337, A::add(s.ad_value(337), A::sqrt(A::offset(A::mul(s.ad_value(337), s.ad_value(337)), ((0.25 * 1e-30) * 1e-30)))), 0.5);
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_ad(342, &A::sub(A::voltage(ctx, &nodes, Some(14), None), A::abs(A::voltage(ctx, &nodes, Some(1), Some(2)))));
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scale_ad(342, A::add(s.ad_value(342), A::sqrt(A::offset(A::mul(s.ad_value(342), s.ad_value(342)), ((0.25 * 1e-30) * 1e-30)))), 0.5);
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scale(136, 337, p.p89);
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(337)), (p.p89 * p.p89)));
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scaled_div(339, 136, 90, (((p.p91 * p.p10)) as f64).abs());
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scale(136, 342, p.p90);
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(342)), (p.p90 * p.p90)));
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scaled_div(344, 136, 90, (((p.p92 * p.p10)) as f64).abs());
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scale(136, 342, p.p90);
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(342)), (p.p90 * p.p90)));
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scaled_div(345, 136, 90, (((p.p93 * p.p13)) as f64).abs());
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scale(136, 342, p.p90);
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(342)), (p.p90 * p.p90)));
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scaled_div(346, 136, 90, (((p.p94 * p.p17)) as f64).abs());
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scale(136, 337, p.p89);
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(337)), (p.p89 * p.p89)));
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scaled_div(340, 136, 90, (((p.p95 * p.p36)) as f64).abs());
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scale(136, 337, p.p89);
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(337)), (p.p89 * p.p89)));
        }

        if ((s.v[391] != 0.0) && (!((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)))) {
            s.store_scaled_div(341, 136, 90, (((p.p96 * p.p37)) as f64).abs());
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_ad(337, &A::voltage(ctx, &nodes, Some(5), None));
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_ad(364, &A::voltage(ctx, &nodes, Some(6), None));
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scale(136, 337, p.p89);
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(337)), (p.p89 * p.p89)));
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scaled_div(339, 136, 90, (((p.p91 * p.p10)) as f64).abs());
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scale(136, 337, p.p89);
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(337)), (p.p89 * p.p89)));
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scaled_div(340, 136, 90, (((p.p95 * p.p36)) as f64).abs());
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scale(136, 337, p.p89);
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(337)), (p.p89 * p.p89)));
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scaled_div(341, 136, 90, (((p.p96 * p.p37)) as f64).abs());
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scale(136, 364, p.p90);
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(364)), (p.p90 * p.p90)));
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scaled_div(344, 136, 90, (((p.p92 * p.p10)) as f64).abs());
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scale(136, 364, p.p90);
        }

    }

    pub(super) fn stamp_reactive_block_1(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(364)), (p.p90 * p.p90)));
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scaled_div(365, 136, 90, (((p.p147 * p.p36)) as f64).abs());
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scale(136, 364, p.p90);
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_sqrt_ad(90, A::offset(A::square(s.ad_value(364)), (p.p90 * p.p90)));
        }

        if ((s.v[392] != 0.0) && (!(((((s.v[387] != 0.0) || (s.v[388] != 0.0)) || (s.v[389] != 0.0)) || (s.v[390] != 0.0)) || (s.v[391] != 0.0)))) {
            s.store_scaled_div(366, 136, 90, (((p.p148 * p.p37)) as f64).abs());
        }

        s.v[80] = (p.p9 / p.p1);

        s.v[81] = (p.p9 / p.p2);

        s.store_offset_ad(146, A::mul(A::offset(s.ad_value(211), p.p27), s.ad_value(140)), (1.0 + p.p26));

        s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);

        s.store_sub_ad(87, A::add(A::offset(s.ad_value(339), p.p10), s.ad_value(344)), A::div(A::mul(A::sub(A::offset(s.ad_value(212), p.p22), s.ad_value(216)), A::scale(s.ad_value(140), p.p23)), A::sqrt(A::offset(A::square(s.ad_value(140)), (p.p23 * p.p23)))));

        s.store_scale(334, 82, 1.0 / (s.v[35]));

        s.store_add_ad(88, A::add(A::add(A::sub(s.ad_value(87), A::scale(A::offset(s.ad_value(334), (-1.0)), p.p24)), s.ad_value(209)), s.ad_value(213)), A::scale(s.ad_value(45), ((s.v[81] / (s.v[81] + s.v[80])) * p.p11)));

        s.store_div_from_scalar_ad(136, p.p3, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));

        s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p30))));

        s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(40), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(40), s.ad_value(159)), A::sub(s.ad_value(40), s.ad_value(159))), 0.0001))), 0.5), 159);

        s.store_sub(37, 160, 88);

        s.store_div_from_scalar_ad(84, s.v[80], A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));

        s.store_div_from_scalar(150, 2.718281828459045, 84);

        s.store_div_from_scalar(151, 1.0, 84);

        s.v[99] = (s.v[80] / 1.602176634e-19);

        s.store_add_ad(154, A::scale(s.ad_value(37), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(37)), ((4.0 * 0.3) * 0.3))), 0.5));

        s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));

        s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));

        let assign2600_ad_e4564: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::scale(s.ad_value(154), s.v[99]), 0.6666666666666666), (p.p28 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::scale(s.ad_value(154), s.v[99]), 0.6666666666666666), ((2.0 * p.p28) / 3.0))));
        s.store_ad(152, &assign2600_ad_e4564);

        s.store_div_ad_rhs(136, 37, A::scale(s.ad_value(83), 2.0));

        s.v[393] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (s.v[393] != 0.0) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (s.v[393] != 0.0) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (s.v[393] != 0.0) {
            s.store_div_ad(153, A::mul(A::scale(s.ad_value(83), (2.0 * s.v[99])), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::scale(A::limited_exp(A::div(A::scale(s.ad_value(37), (-1.0)), A::scale(s.ad_value(83), 2.0))), (s.v[99] / 3.24e17))));
        }

        if (!(s.v[393] != 0.0)) {
            s.store_div_ad(153, A::mul(A::scale(s.ad_value(83), (2.0 * s.v[99])), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::scale(A::limited_exp(A::div(A::scale(s.ad_value(37), (-1.0)), A::scale(s.ad_value(83), 2.0))), (s.v[99] / 3.24e17))));
        }

        s.store_sub_ad_rhs(100, 37, A::scale(s.ad_value(153), 1.0 / (s.v[99])));

        s.v[394] = if ((((s.v[100] - s.v[37])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (s.v[394] != 0.0) {
            s.store_sub(101, 37, 100);
        }

        if (s.v[394] != 0.0) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (s.v[394] != 0.0) {
            s.store_scalar(136, ((s.v[99]) as f64).powf(0.6666666666666666));
        }

        if (s.v[394] != 0.0) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (s.v[394] != 0.0) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p28), 90);
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p29), 90);
        }

        if (s.v[394] != 0.0) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (s.v[394] != 0.0) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (s.v[394] != 0.0) {
            let assign2780_ad_e4790: A = {
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
            let assign2780_ad_e4828: A = {
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
            s.store_sub_ad(106, A::sub(A::scale(s.ad_value(101), s.v[99]), A::mul(A::scale(s.ad_value(83), 3.24e17), assign2780_ad_e4790)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign2780_ad_e4828));
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p28), 91);
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p29), 91);
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (s.v[394] != 0.0) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (s.v[394] != 0.0) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (s.v[394] != 0.0) {
            s.store_sub_ad(113, A::sub_from_scalar(((-1.0) * s.v[99]), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (s.v[394] != 0.0) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (s.v[394] != 0.0) {
            s.store_sub(115, 37, 114);
        }

        if (s.v[394] != 0.0) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (s.v[394] != 0.0) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p28), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p29), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (s.v[394] != 0.0) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (s.v[394] != 0.0) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (s.v[394] != 0.0) {
            let assign2940_ad_e5023: A = {
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
            let assign2940_ad_e5061: A = {
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
            s.store_sub_ad(120, A::sub(A::scale(s.ad_value(115), s.v[99]), A::mul(A::scale(s.ad_value(83), 3.24e17), assign2940_ad_e5023)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign2940_ad_e5061));
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p28), 137);
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p29), 137);
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (s.v[394] != 0.0) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (s.v[394] != 0.0) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (s.v[394] != 0.0) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (s.v[394] != 0.0) {
            s.store_sub_ad(127, A::sub_from_scalar(((-1.0) * s.v[99]), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (s.v[394] != 0.0) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (s.v[394] != 0.0) {
            s.copy_ad(129, 128);
        }

        if (!(s.v[394] != 0.0)) {
            s.copy_ad(129, 100);
        }

        s.store_sub_from_scalar(347, p.p13, 345);

        s.store_sub_from_scalar(348, p.p17, 346);

        s.store_mul_ad_rhs(97, 347, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20));

        s.store_mul_ad_rhs(89, 348, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19));

        s.store_scale_ad(136, A::abs(A::sub(s.ad_value(37), s.ad_value(129))), (s.v[80] / p.p9));

        s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(129))), (s.v[81] / p.p9));

        s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));

        s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);

        s.store_add_ad(90, A::scale(s.ad_value(37), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(37)), ((4.0 * 0.3) * 0.3))), 0.5));

        s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p3), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p3), s.ad_value(90)));

        s.store_powf_ad(136, A::div(s.ad_value(38), s.ad_value(85)), p.p18);

        s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));

        s.store_mul(86, 38, 90);

        s.store_sub(39, 37, 86);

        s.copy_ad(130, 39);

        s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));

        s.copy_ad(154, 131);

        s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));

        s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));

        let assign3240_ad_e5339: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::scale(s.ad_value(154), s.v[99]), 0.6666666666666666), (p.p28 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::scale(s.ad_value(154), s.v[99]), 0.6666666666666666), ((2.0 * p.p28) / 3.0))));
        s.store_ad(152, &assign3240_ad_e5339);

        s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));

        s.v[395] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (s.v[395] != 0.0) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (s.v[395] != 0.0) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (s.v[395] != 0.0) {
            s.store_div_ad(156, A::mul(A::scale(s.ad_value(83), (2.0 * s.v[99])), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::scale(A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))), (s.v[99] / 3.24e17))));
        }

        if (!(s.v[395] != 0.0)) {
            s.store_div_ad(156, A::mul(A::scale(s.ad_value(83), (2.0 * s.v[99])), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::scale(A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))), (s.v[99] / 3.24e17))));
        }

        s.store_sub_ad_rhs(100, 130, A::scale(s.ad_value(156), 1.0 / (s.v[99])));

        s.v[396] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (s.v[396] != 0.0) {
            s.store_sub(101, 130, 100);
        }

        if (s.v[396] != 0.0) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (s.v[396] != 0.0) {
            s.store_scalar(136, ((s.v[99]) as f64).powf(0.6666666666666666));
        }

        if (s.v[396] != 0.0) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (s.v[396] != 0.0) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p28), 90);
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p29), 90);
        }

        if (s.v[396] != 0.0) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (s.v[396] != 0.0) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (s.v[396] != 0.0) {
            let assign3420_ad_e5565: A = {
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
            let assign3420_ad_e5603: A = {
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
            s.store_sub_ad(106, A::sub(A::scale(s.ad_value(101), s.v[99]), A::mul(A::scale(s.ad_value(83), 3.24e17), assign3420_ad_e5565)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign3420_ad_e5603));
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p28), 91);
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p29), 91);
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (s.v[396] != 0.0) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (s.v[396] != 0.0) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (s.v[396] != 0.0) {
            s.store_sub_ad(113, A::sub_from_scalar(((-1.0) * s.v[99]), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (s.v[396] != 0.0) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (s.v[396] != 0.0) {
            s.store_sub(115, 130, 114);
        }

        if (s.v[396] != 0.0) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p28), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p29), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (s.v[396] != 0.0) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (s.v[396] != 0.0) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

    }

    pub(super) fn stamp_reactive_block_2(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[396] != 0.0) {
            let assign3570_ad_e5791: A = {
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
            let assign3570_ad_e5829: A = {
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
            s.store_sub_ad(120, A::sub(A::scale(s.ad_value(115), s.v[99]), A::mul(A::scale(s.ad_value(83), 3.24e17), assign3570_ad_e5791)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign3570_ad_e5829));
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p28), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p29), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (s.v[396] != 0.0) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (s.v[396] != 0.0) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (s.v[396] != 0.0) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (s.v[396] != 0.0) {
            s.store_sub_ad(127, A::sub_from_scalar(((-1.0) * s.v[99]), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (s.v[396] != 0.0) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (s.v[396] != 0.0) {
            s.store_add(132, 128, 86);
        }

        if (!(s.v[396] != 0.0)) {
            s.store_add(132, 100, 86);
        }

        s.store_scaled_add(133, 129, 132, 0.5);

        s.store_sub(134, 132, 129);

        s.store_mul_ad_lhs(135, A::add(A::sub(s.ad_value(37), s.ad_value(133)), s.ad_value(83)), 134);

        s.store_scale_ad(136, A::abs(A::sub(s.ad_value(37), s.ad_value(133))), (s.v[80] / p.p9));

        s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(129))), (s.v[81] / p.p9));

        s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::mul(A::scale(s.ad_value(136), p.p15), s.ad_value(136))), A::scale(s.ad_value(90), p.p16)));

        s.store_scale(96, 95, (s.v[80] * (p.p4 * (p.p5 * 1.0 / (p.p3)))));

        s.store_mul_ad_rhs(98, 96, A::offset(A::scale(A::sub(s.ad_value(140), s.ad_value(86)), p.p21), 1.0));

        s.store_sqrt_ad(92, A::offset(A::mul(A::scale(s.ad_value(134), (p.p25 * p.p25)), s.ad_value(134)), 1.0));

        s.store_div(93, 98, 92);

        s.store_mul(94, 93, 135);

        s.store_sub(90, 132, 129);

        s.store_sub_ad_lhs(91, A::add(s.ad_value(37), s.ad_value(83)), 133);

        s.store_scale_ad(137, A::add(A::sub(s.ad_value(37), s.ad_value(133)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))), (((s.v[80] * p.p4) * p.p5) * p.p3));

        s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p233)), 1e26);

        s.store_offset_ad(189, A::powf(s.ad_value(188), p.p232), 1.0);

        s.store_div_from_scalar(190, p.p231, 189);

        s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p1));

        s.store_mul_ad(161, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p3))), A::add(A::sub(s.ad_value(37), s.ad_value(133)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));

        s.store_sub_ad_lhs(136, A::add(s.ad_value(37), s.ad_value(83)), 133);

        s.store_scale_ad(90, A::add(s.ad_value(129), A::scale(s.ad_value(132), 2.0)), 0.3333333333333333);

        s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(134)), (1.0 / 12.0)), 136);

        s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(134)), s.ad_value(134)), (1.0 / 120.0)), A::square(s.ad_value(136)));

        s.store_mul_ad(165, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p3 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(37), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));

        s.store_sub_ad_lhs(166, A::scale(s.ad_value(161), (-1.0)), 165);

        s.v[401] = if (s.v[41] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[401] != 0.0) {
            s.copy_ad(90, 166);
        }

        if (s.v[401] != 0.0) {
            s.copy_ad(166, 165);
        }

        if (s.v[401] != 0.0) {
            s.copy_ad(165, 90);
        }

        s.v[402] = if (p.p56 == 0.0) { 1.0 } else { 0.0 };

        s.v[403] = if (p.p56 == 1.0) { 1.0 } else { 0.0 };

        s.v[404] = if (p.p56 == 2.0) { 1.0 } else { 0.0 };

        s.v[405] = if (p.p56 == 3.0) { 1.0 } else { 0.0 };

        s.v[406] = if (p.p56 == 4.0) { 1.0 } else { 0.0 };

        if ((s.v[403] != 0.0) && (!(s.v[402] != 0.0))) {
            s.store_ad(136, &A::div(A::voltage(ctx, &nodes, Some(9), Some(8)), A::scale(s.ad_value(82), (p.p57 * 8.617087e-5))));
        }

        if ((s.v[403] != 0.0) && (!(s.v[402] != 0.0))) {
            s.store_offset_ad(137, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p71), p.p63);
        }

        if ((s.v[403] != 0.0) && (!(s.v[402] != 0.0))) {
            s.store_ad(136, &A::div(A::voltage(ctx, &nodes, Some(9), Some(7)), A::scale(s.ad_value(82), (p.p60 * 8.617087e-5))));
        }

        if ((s.v[403] != 0.0) && (!(s.v[402] != 0.0))) {
            s.store_offset_ad(137, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p72), p.p64);
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_offset_ad(326, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p75), p.p67);
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_offset_ad(328, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p77), p.p57);
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_offset_ad(330, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p79), p.p61);
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_div_ad(136, A::sub(A::voltage(ctx, &nodes, Some(9), Some(8)), s.ad_value(326)), A::scale(s.ad_value(328), (8.617087e-5 * s.v[35])));
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_scale_ad(137, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p71)), p.p63);
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_sub_ad(321, A::neg(A::voltage(ctx, &nodes, Some(9), Some(8))), A::scale(A::sub(A::neg(A::voltage(ctx, &nodes, Some(9), Some(8))), A::sqrt(A::offset(A::mul(A::neg(A::voltage(ctx, &nodes, Some(9), Some(8))), A::neg(A::voltage(ctx, &nodes, Some(9), Some(8)))), 0.001))), 0.5));
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_offset_ad(136, A::sqrt(s.ad_value(321)), p.p69);
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_div_ad_rhs(90, 136, A::scale(s.ad_value(330), (8.617087e-5 * s.v[35])));
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_offset_ad(327, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p76), p.p68);
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_offset_ad(329, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p78), p.p60);
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_offset_ad(331, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p80), p.p62);
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_div_ad(136, A::sub(A::voltage(ctx, &nodes, Some(9), Some(7)), s.ad_value(327)), A::scale(s.ad_value(329), (8.617087e-5 * s.v[35])));
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_scale_ad(137, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p72)), p.p64);
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_sub_ad(323, A::neg(A::voltage(ctx, &nodes, Some(9), Some(7))), A::scale(A::sub(A::neg(A::voltage(ctx, &nodes, Some(9), Some(7))), A::sqrt(A::offset(A::mul(A::neg(A::voltage(ctx, &nodes, Some(9), Some(7))), A::neg(A::voltage(ctx, &nodes, Some(9), Some(7)))), 0.001))), 0.5));
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_offset_ad(136, A::sqrt(s.ad_value(323)), p.p70);
        }

        if ((s.v[404] != 0.0) && (!((s.v[402] != 0.0) || (s.v[403] != 0.0)))) {
            s.store_div_ad_rhs(136, 136, A::scale(s.ad_value(331), (8.617087e-5 * s.v[35])));
        }

        if ((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) {
            s.store_offset_ad(326, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p75), p.p67);
        }

        if ((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) {
            s.store_offset_ad(328, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p77), p.p57);
        }

        if ((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) {
            s.store_offset_ad(330, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p79), p.p61);
        }

        if ((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) {
            s.store_scale_ad(137, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p71)), (((p.p4 * p.p3) * p.p5) * p.p63));
        }

        if ((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) {
            s.store_offset_ad(327, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p76), p.p68);
        }

        if ((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) {
            s.store_offset_ad(329, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p78), p.p60);
        }

        if ((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) {
            s.store_offset_ad(331, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p80), p.p62);
        }

        if ((s.v[405] != 0.0) && (!(((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)))) {
            s.store_scale_ad(137, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p72)), (((p.p4 * p.p3) * p.p5) * p.p64));
        }

        if ((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) {
            s.store_offset_ad(326, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p75), p.p67);
        }

        if ((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) {
            s.store_offset_ad(328, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p77), p.p57);
        }

        if ((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) {
            s.store_offset_ad(330, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p79), p.p61);
        }

        if ((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) {
            s.store_scale_ad(137, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p71)), (((p.p4 * p.p3) * p.p5) * p.p63));
        }

        if ((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) {
            s.store_offset_ad(327, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p76), p.p68);
        }

        if ((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) {
            s.store_offset_ad(329, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p78), p.p60);
        }

        if ((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) {
            s.store_offset_ad(331, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p80), p.p62);
        }

        if ((s.v[406] != 0.0) && (!((((s.v[402] != 0.0) || (s.v[403] != 0.0)) || (s.v[404] != 0.0)) || (s.v[405] != 0.0)))) {
            s.store_scale_ad(137, A::exp(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p72)), (((p.p4 * p.p3) * p.p5) * p.p64));
        }

        s.v[359] = if self.param_given[45] { 1.0 } else { 0.0 };

        s.v[360] = if self.param_given[44] { 1.0 } else { 0.0 };

        s.copy_ad(187, 154);

        s.v[424] = if (s.v[361] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[424] != 0.0) {
            s.store_add_ad(177, A::sub(A::sub(A::scale(A::sub_from_scalar(1.0, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p50)), p.p36), s.ad_value(340)), s.ad_value(365)), A::scale(s.ad_value(45), ((p.p12 / 1.602176634e-19) * s.v[81])));
        }

        if (s.v[424] != 0.0) {
            s.store_sub_ad(177, A::offset(s.ad_value(177), 1.0), A::scale(A::sub(A::offset(s.ad_value(177), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(177), (-1.0)), A::offset(s.ad_value(177), (-1.0))), 0.001))), 0.5));
        }

        if (s.v[424] != 0.0) {
            s.store_mul_ad(172, A::scale(s.ad_value(177), 1.602176634e-19), A::offset(A::scale(s.ad_value(187), p.p38), 1.0));
        }

        if (s.v[424] != 0.0) {
            s.store_scale_ad(176, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p51), p.p35);
        }

        if (s.v[424] != 0.0) {
            s.store_mul_ad_lhs(173, A::scale(s.ad_value(172), (p.p4 * p.p5)), 176);
        }

        s.v[425] = if (s.v[359] != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[424] != 0.0) && (s.v[425] != 0.0)) {
            s.store_scalar(350, (1.0 + p.p45));
        }

        if ((s.v[424] != 0.0) && (s.v[425] != 0.0)) {
            s.store_mul_ad_lhs(351, A::sqrt(s.ad_value(350)), 94);
        }

        if ((s.v[424] != 0.0) && (s.v[425] != 0.0)) {
            s.store_div(352, 351, 173);
        }

        if ((s.v[424] != 0.0) && (s.v[425] != 0.0)) {
            s.store_scale(353, 352, 2.0);
        }

        if ((s.v[424] != 0.0) && (s.v[425] != 0.0)) {
            s.store_add_ad_rhs(350, 350, A::square(s.ad_value(352)));
        }

        if ((s.v[424] != 0.0) && (s.v[425] != 0.0)) {
            s.store_add_ad(350, A::sqrt(A::sub(s.ad_value(350), s.ad_value(353))), A::sqrt(A::add(s.ad_value(350), s.ad_value(353))));
        }

        if ((s.v[424] != 0.0) && (s.v[425] != 0.0)) {
            s.store_div_ad_lhs(349, A::scale(s.ad_value(351), 2.0), 350);
        }

        if ((s.v[424] != 0.0) && (s.v[425] != 0.0)) {
            s.store_sub_from_scalar_ad(91, 1.0, A::div(s.ad_value(349), s.ad_value(173)));
        }

        if ((s.v[424] != 0.0) && (!(s.v[425] != 0.0))) {
            s.store_abs_ad(182, A::div(s.ad_value(94), s.ad_value(173)));
        }

        if ((s.v[424] != 0.0) && (!(s.v[425] != 0.0))) {
            s.store_scale_ad(183, A::offset(A::sub(A::offset(s.ad_value(182), 0.9), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(182), (-0.9)), A::offset(s.ad_value(182), (-0.9))), (0.1 * 0.1)))), (-(0.9 - ((((0.9 * 0.9) + (0.1 * 0.1))) as f64).sqrt()))), 0.5);
        }

        if ((s.v[424] != 0.0) && (!(s.v[425] != 0.0))) {
            s.store_powf(136, 183, p.p42);
        }

        if ((s.v[424] != 0.0) && (!(s.v[425] != 0.0))) {
            s.store_sub_from_scalar(90, 1.0, 136);
        }

        if ((s.v[424] != 0.0) && (!(s.v[425] != 0.0))) {
            s.store_powf(91, 90, (1.0 / p.p42));
        }

        if (s.v[424] != 0.0) {
            s.store_add_ad(177, A::sub(A::sub(A::scale(A::sub_from_scalar(1.0, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p50)), p.p37), s.ad_value(341)), s.ad_value(366)), A::scale(s.ad_value(45), ((p.p12 / 1.602176634e-19) * s.v[81])));
        }

        if (s.v[424] != 0.0) {
            s.store_sub_ad(177, A::offset(s.ad_value(177), 1.0), A::scale(A::sub(A::offset(s.ad_value(177), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(177), (-1.0)), A::offset(s.ad_value(177), (-1.0))), 0.001))), 0.5));
        }

        if (s.v[424] != 0.0) {
            s.store_mul_ad(172, A::scale(s.ad_value(177), 1.602176634e-19), A::offset(A::scale(s.ad_value(187), p.p39), 1.0));
        }

        if (s.v[424] != 0.0) {
            s.store_mul_ad_lhs(173, A::scale(s.ad_value(172), (p.p4 * p.p5)), 176);
        }

        s.v[426] = if (s.v[360] != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[424] != 0.0) && (s.v[426] != 0.0)) {
            s.store_scalar(350, (1.0 + p.p44));
        }

        if ((s.v[424] != 0.0) && (s.v[426] != 0.0)) {
            s.store_mul_ad_lhs(351, A::sqrt(s.ad_value(350)), 94);
        }

        if ((s.v[424] != 0.0) && (s.v[426] != 0.0)) {
            s.store_div(352, 351, 173);
        }

        if ((s.v[424] != 0.0) && (s.v[426] != 0.0)) {
            s.store_scale(353, 352, 2.0);
        }

        if ((s.v[424] != 0.0) && (s.v[426] != 0.0)) {
            s.store_add_ad_rhs(350, 350, A::square(s.ad_value(352)));
        }

        if ((s.v[424] != 0.0) && (s.v[426] != 0.0)) {
            s.store_add_ad(350, A::sqrt(A::sub(s.ad_value(350), s.ad_value(353))), A::sqrt(A::add(s.ad_value(350), s.ad_value(353))));
        }

        if ((s.v[424] != 0.0) && (s.v[426] != 0.0)) {
            s.store_div_ad_lhs(349, A::scale(s.ad_value(351), 2.0), 350);
        }

        if ((s.v[424] != 0.0) && (s.v[426] != 0.0)) {
            s.store_sub_from_scalar_ad(91, 1.0, A::div(s.ad_value(349), s.ad_value(173)));
        }

        if ((s.v[424] != 0.0) && (!(s.v[426] != 0.0))) {
            s.store_abs_ad(182, A::div(s.ad_value(94), s.ad_value(173)));
        }

        if ((s.v[424] != 0.0) && (!(s.v[426] != 0.0))) {
            s.store_scale_ad(183, A::offset(A::sub(A::offset(s.ad_value(182), 0.9), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(182), (-0.9)), A::offset(s.ad_value(182), (-0.9))), (0.1 * 0.1)))), (-(0.9 - ((((0.9 * 0.9) + (0.1 * 0.1))) as f64).sqrt()))), 0.5);
        }

        if ((s.v[424] != 0.0) && (!(s.v[426] != 0.0))) {
            s.store_powf(136, 183, p.p43);
        }

        if ((s.v[424] != 0.0) && (!(s.v[426] != 0.0))) {
            s.store_sub_from_scalar(90, 1.0, 136);
        }

        if ((s.v[424] != 0.0) && (!(s.v[426] != 0.0))) {
            s.store_powf(91, 90, (1.0 / p.p43));
        }

        s.v[433] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[434] = if (p.p150 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_ad(49, &A::voltage(ctx, &nodes, Some(15), Some(7)));
        }

        s.v[435] = if (p.p150 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[435] != 0.0)) {
            s.store_ad(50, &A::voltage(ctx, &nodes, Some(9), Some(7)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[435] != 0.0)) {
            s.store_ad(51, &A::voltage(ctx, &nodes, Some(9), Some(15)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (!(s.v[435] != 0.0))) {
            s.store_ad(50, &A::voltage(ctx, &nodes, Some(2), Some(7)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (!(s.v[435] != 0.0))) {
            s.store_ad(51, &A::voltage(ctx, &nodes, Some(2), Some(15)));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_scalar(48, 1.0);
        }

        s.v[436] = if (s.v[49] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[436] != 0.0)) {
            s.store_scalar(48, (-1.0));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[436] != 0.0)) {
            s.store_mul(231, 48, 49);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[436] != 0.0)) {
            s.copy_ad(230, 51);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (!(s.v[436] != 0.0))) {
            s.copy_ad(231, 49);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (!(s.v[436] != 0.0))) {
            s.copy_ad(230, 50);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_offset_ad(232, A::sqrt(A::offset(A::square(s.ad_value(231)), 0.01)), (-0.1));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_offset_scaled(146, 232, p.p166, (1.0 + p.p165));
        }

    }
}
