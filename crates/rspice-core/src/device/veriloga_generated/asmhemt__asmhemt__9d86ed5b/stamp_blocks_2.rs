#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_block_3(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_sub_ad(88, A::offset(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p162), p.p159), A::div(A::scale(s.ad_value(232), (p.p168 * p.p167)), A::sqrt(A::offset(A::square(s.ad_value(232)), (p.p168 * p.p168)))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_scalar(223, (p.p9 / p.p160));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p161, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p158))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(230), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(230), s.ad_value(159)), A::sub(s.ad_value(230), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_sub(222, 160, 88);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad_rhs(84, 223, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_scale(99, 223, 6.241509074460763e18);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(222), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(222)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            let assign6440_ad_e9675: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p169 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0))));
            s.store_ad(152, &assign6440_ad_e9675);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad_rhs(136, 222, A::scale(s.ad_value(83), 2.0));
        }

        s.v[437] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[437] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[437] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[437] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(222), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (!(s.v[437] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(222), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_sub_ad_rhs(100, 222, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[438] = if ((((s.v[100] - s.v[222])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_sub(101, 222, 100);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p169), 90);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p170), 90);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            let assign6620_ad_e9969: A = {
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
            let assign6620_ad_e10007: A = {
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign6620_ad_e9969)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign6620_ad_e10007));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p169), 91);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p170), 91);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_sub(115, 222, 114);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            let assign6780_ad_e10266: A = {
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
            let assign6780_ad_e10304: A = {
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign6780_ad_e10266)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign6780_ad_e10304));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p169), 137);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p170), 137);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[438] != 0.0)) {
            s.copy_ad(224, 128);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (!(s.v[438] != 0.0))) {
            s.copy_ad(224, 100);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p163);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p164);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(223), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(222), s.ad_value(224))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(224))), (s.v[81] / p.p9));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(222), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(222)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p161), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p161), s.ad_value(90)));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(231), s.ad_value(85)), p.p18);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_mul(86, 231, 90);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_sub(39, 222, 86);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            let assign7060_ad_e10704: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p169 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0))));
            s.store_ad(152, &assign7060_ad_e10704);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[439] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[439] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[439] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[439] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (!(s.v[439] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[440] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p169), 90);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p170), 90);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            let assign7240_ad_e10998: A = {
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
            let assign7240_ad_e11036: A = {
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign7240_ad_e10998)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign7240_ad_e11036));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p169), 91);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p170), 91);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_4(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            let assign7390_ad_e11284: A = {
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
            let assign7390_ad_e11322: A = {
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign7390_ad_e11284)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign7390_ad_e11322));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[440] != 0.0)) {
            s.store_add(225, 128, 86);
        }

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (!(s.v[440] != 0.0))) {
            s.store_add(225, 100, 86);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_scaled_add(226, 224, 225, 0.5);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_sub(227, 225, 224);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_mul_ad_lhs(135, A::add(A::sub(s.ad_value(222), s.ad_value(226)), s.ad_value(83)), 227);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(223), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(222), s.ad_value(226))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(129))), (s.v[81] / p.p9));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::mul(A::scale(s.ad_value(136), p.p15), s.ad_value(136))), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_scaled_mul(96, 95, 223, (p.p4 * (p.p5 * 1.0 / (p.p161))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_mul_ad_rhs(98, 96, A::offset(A::scale(A::sub(s.ad_value(232), s.ad_value(86)), p.p21), 1.0));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_sqrt_ad(92, A::offset(A::mul(A::scale(s.ad_value(227), (p.p25 * p.p25)), s.ad_value(227)), 1.0));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div(93, 98, 92);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_sub(90, 225, 224);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(222), s.ad_value(83)), 226);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(223), (p.p4 * (p.p5 * p.p161))), A::add(A::sub(s.ad_value(222), s.ad_value(226)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p236)), 1e26);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p235), 1.0);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_from_scalar(190, p.p234, 189);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p160));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_mul_ad(228, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p161))), A::add(A::sub(s.ad_value(222), s.ad_value(226)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(222), s.ad_value(83)), 226);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(224), A::scale(s.ad_value(225), 2.0)), 0.3333333333333333);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(227)), (1.0 / 12.0)), 136);
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(227)), s.ad_value(227)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((s.v[433] != 0.0) && (s.v[434] != 0.0)) {
            s.store_mul_ad(229, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p161 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(222), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        s.v[441] = if (s.v[48] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[433] != 0.0) && (s.v[434] != 0.0)) && (s.v[441] != 0.0)) {
            s.store_sub_ad_lhs(229, A::scale(s.ad_value(228), (-1.0)), 229);
        }

        if ((s.v[433] != 0.0) && (!(s.v[434] != 0.0))) {
            s.store_scalar(228, 0.0);
        }

        if ((s.v[433] != 0.0) && (!(s.v[434] != 0.0))) {
            s.store_scalar(229, 0.0);
        }

        s.v[442] = if (p.p150 != 0.0) { 1.0 } else { 0.0 };

        s.v[443] = if (p.p150 == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[443] != 0.0)) {
            s.store_ad(50, &A::voltage(ctx, &nodes, Some(9), Some(7)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (!(s.v[443] != 0.0))) {
            s.store_ad(50, &A::voltage(ctx, &nodes, Some(2), Some(7)));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.copy_ad(230, 50);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_scalar(146, (1.0 + p.p165));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_offset_ad(88, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p162), p.p159);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_scalar(223, (p.p9 / p.p160));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p161, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p158))));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(230), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(230), s.ad_value(159)), A::sub(s.ad_value(230), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_sub(222, 160, 88);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad_rhs(84, 223, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_scale(99, 223, 6.241509074460763e18);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(222), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(222)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            let assign7980_ad_e12102: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p169 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0))));
            s.store_ad(152, &assign7980_ad_e12102);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad_rhs(136, 222, A::scale(s.ad_value(83), 2.0));
        }

        s.v[444] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[444] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[444] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[444] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(222), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (!(s.v[444] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(222), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_sub_ad_rhs(100, 222, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[445] = if ((((s.v[100] - s.v[222])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_sub(101, 222, 100);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p169), 90);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p170), 90);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            let assign8160_ad_e12412: A = {
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
            let assign8160_ad_e12450: A = {
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign8160_ad_e12412)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign8160_ad_e12450));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p169), 91);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p170), 91);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_sub(115, 222, 114);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            let assign8320_ad_e12725: A = {
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
            let assign8320_ad_e12763: A = {
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign8320_ad_e12725)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign8320_ad_e12763));
        }

    }

    pub(super) fn stamp_reactive_block_5(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p169), 137);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p170), 137);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[445] != 0.0)) {
            s.copy_ad(224, 128);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (!(s.v[445] != 0.0))) {
            s.copy_ad(224, 100);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_scalar(231, 0.0);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p163);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p164);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(223), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(222), s.ad_value(224))));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(224))), (s.v[81] / p.p9));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(222), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(222)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p161), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p161), s.ad_value(90)));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(231), s.ad_value(85)), p.p18);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_mul(86, 231, 90);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_sub(39, 222, 86);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            let assign8610_ad_e13198: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p169 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0))));
            s.store_ad(152, &assign8610_ad_e13198);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[446] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[446] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[446] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[446] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (!(s.v[446] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[447] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p169), 90);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p170), 90);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            let assign8790_ad_e13508: A = {
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
            let assign8790_ad_e13546: A = {
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign8790_ad_e13508)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign8790_ad_e13546));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p169), 91);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p170), 91);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            let assign8940_ad_e13809: A = {
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
            let assign8940_ad_e13847: A = {
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign8940_ad_e13809)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign8940_ad_e13847));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (s.v[447] != 0.0)) {
            s.store_add(225, 128, 86);
        }

        if (((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) && (!(s.v[447] != 0.0))) {
            s.store_add(225, 100, 86);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_scaled_add(226, 224, 225, 0.5);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_sub(227, 225, 224);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_sub(90, 225, 224);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(222), s.ad_value(83)), 226);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(223), (p.p4 * (p.p5 * p.p161))), A::add(A::sub(s.ad_value(222), s.ad_value(226)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p236)), 1e26);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p235), 1.0);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_from_scalar(190, p.p234, 189);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p160));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_mul_ad(228, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p161))), A::add(A::sub(s.ad_value(222), s.ad_value(226)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(222), s.ad_value(83)), 226);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(224), A::scale(s.ad_value(225), 2.0)), 0.3333333333333333);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(227)), (1.0 / 12.0)), 136);
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(227)), s.ad_value(227)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((!(s.v[433] != 0.0)) && (s.v[442] != 0.0)) {
            s.store_mul_ad(229, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p161 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(222), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        if ((!(s.v[433] != 0.0)) && (!(s.v[442] != 0.0))) {
            s.store_scalar(228, 0.0);
        }

        if ((!(s.v[433] != 0.0)) && (!(s.v[442] != 0.0))) {
            s.store_scalar(229, 0.0);
        }

        s.v[448] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[449] = if (p.p151 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_ad(53, &A::voltage(ctx, &nodes, Some(8), Some(19)));
        }

        s.v[450] = if (p.p151 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[450] != 0.0)) {
            s.store_ad(54, &A::voltage(ctx, &nodes, Some(9), Some(19)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[450] != 0.0)) {
            s.store_ad(55, &A::voltage(ctx, &nodes, Some(9), Some(8)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (!(s.v[450] != 0.0))) {
            s.store_ad(54, &A::voltage(ctx, &nodes, Some(2), Some(19)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (!(s.v[450] != 0.0))) {
            s.store_ad(55, &A::voltage(ctx, &nodes, Some(2), Some(8)));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_scalar(52, 1.0);
        }

        s.v[451] = if (s.v[53] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[451] != 0.0)) {
            s.store_scalar(52, (-1.0));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[451] != 0.0)) {
            s.store_mul(243, 52, 53);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[451] != 0.0)) {
            s.copy_ad(242, 55);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (!(s.v[451] != 0.0))) {
            s.copy_ad(243, 53);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (!(s.v[451] != 0.0))) {
            s.copy_ad(242, 54);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_offset_ad(244, A::sqrt(A::offset(A::square(s.ad_value(243)), 0.01)), (-0.1));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_offset_scaled(146, 244, p.p166, (1.0 + p.p165));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_sub_ad(88, A::offset(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p162), p.p159), A::div(A::scale(s.ad_value(244), (p.p168 * p.p167)), A::sqrt(A::offset(A::square(s.ad_value(244)), (p.p168 * p.p168)))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_scalar(235, (p.p9 / p.p160));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p161, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p158))));
        }

    }

    pub(super) fn stamp_reactive_block_6(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(242), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(242), s.ad_value(159)), A::sub(s.ad_value(242), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_sub(234, 160, 88);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad_rhs(84, 235, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_scale(99, 235, 6.241509074460763e18);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(234), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(234)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            let assign9530_ad_e14604: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p169 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0))));
            s.store_ad(152, &assign9530_ad_e14604);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad_rhs(136, 234, A::scale(s.ad_value(83), 2.0));
        }

        s.v[452] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[452] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[452] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[452] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(234), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (!(s.v[452] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(234), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_sub_ad_rhs(100, 234, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[453] = if ((((s.v[100] - s.v[234])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_sub(101, 234, 100);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p169), 90);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p170), 90);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            let assign9710_ad_e14898: A = {
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
            let assign9710_ad_e14936: A = {
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign9710_ad_e14898)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign9710_ad_e14936));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p169), 91);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p170), 91);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_sub(115, 234, 114);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            let assign9870_ad_e15195: A = {
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
            let assign9870_ad_e15233: A = {
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign9870_ad_e15195)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign9870_ad_e15233));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p169), 137);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p170), 137);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[453] != 0.0)) {
            s.copy_ad(236, 128);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (!(s.v[453] != 0.0))) {
            s.copy_ad(236, 100);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p163);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p164);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(235), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(234), s.ad_value(236))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(236))), (s.v[81] / p.p9));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(234), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(234)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p161), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p161), s.ad_value(90)));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(243), s.ad_value(85)), p.p18);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_mul(86, 243, 90);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_sub(39, 234, 86);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            let assign10150_ad_e15633: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p169 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0))));
            s.store_ad(152, &assign10150_ad_e15633);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[454] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[454] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[454] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[454] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (!(s.v[454] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[455] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p169), 90);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p170), 90);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            let assign10330_ad_e15927: A = {
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
            let assign10330_ad_e15965: A = {
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign10330_ad_e15927)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign10330_ad_e15965));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p169), 91);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p170), 91);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), 0.6666666666666666));
        }

    }

    pub(super) fn stamp_reactive_block_7(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            let assign10480_ad_e16213: A = {
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
            let assign10480_ad_e16251: A = {
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign10480_ad_e16213)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign10480_ad_e16251));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[455] != 0.0)) {
            s.store_add(237, 128, 86);
        }

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (!(s.v[455] != 0.0))) {
            s.store_add(237, 100, 86);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_scaled_add(238, 236, 237, 0.5);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_sub(239, 237, 236);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_mul_ad_lhs(135, A::add(A::sub(s.ad_value(234), s.ad_value(238)), s.ad_value(83)), 239);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(235), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(234), s.ad_value(238))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(129))), (s.v[81] / p.p9));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::mul(A::scale(s.ad_value(136), p.p15), s.ad_value(136))), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_scaled_mul(96, 95, 235, (p.p4 * (p.p5 * 1.0 / (p.p161))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_mul_ad_rhs(98, 96, A::offset(A::scale(A::sub(s.ad_value(244), s.ad_value(86)), p.p21), 1.0));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_sqrt_ad(92, A::offset(A::mul(A::scale(s.ad_value(239), (p.p25 * p.p25)), s.ad_value(239)), 1.0));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div(93, 98, 92);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_sub(90, 237, 236);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(234), s.ad_value(83)), 238);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(235), (p.p4 * (p.p5 * p.p161))), A::add(A::sub(s.ad_value(234), s.ad_value(238)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p236)), 1e26);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p235), 1.0);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_from_scalar(190, p.p234, 189);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p160));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_mul_ad(240, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p161))), A::add(A::sub(s.ad_value(234), s.ad_value(238)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(234), s.ad_value(83)), 238);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(236), A::scale(s.ad_value(237), 2.0)), 0.3333333333333333);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(239)), (1.0 / 12.0)), 136);
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(239)), s.ad_value(239)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((s.v[448] != 0.0) && (s.v[449] != 0.0)) {
            s.store_mul_ad(241, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p161 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(234), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        s.v[456] = if (s.v[52] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[448] != 0.0) && (s.v[449] != 0.0)) && (s.v[456] != 0.0)) {
            s.store_sub_ad_lhs(241, A::scale(s.ad_value(240), (-1.0)), 241);
        }

        if ((s.v[448] != 0.0) && (!(s.v[449] != 0.0))) {
            s.store_scalar(240, 0.0);
        }

        if ((s.v[448] != 0.0) && (!(s.v[449] != 0.0))) {
            s.store_scalar(241, 0.0);
        }

        s.v[457] = if (p.p151 != 0.0) { 1.0 } else { 0.0 };

        s.v[458] = if (p.p151 == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[458] != 0.0)) {
            s.store_ad(54, &A::voltage(ctx, &nodes, Some(9), Some(8)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (!(s.v[458] != 0.0))) {
            s.store_ad(54, &A::voltage(ctx, &nodes, Some(2), Some(8)));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.copy_ad(234, 54);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_scalar(146, (1.0 + p.p165));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_offset_ad(88, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p162), p.p159);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_scalar(235, (p.p9 / p.p160));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p161, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p158))));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(242), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(242), s.ad_value(159)), A::sub(s.ad_value(242), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_sub(234, 160, 88);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad_rhs(84, 235, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_scale(99, 235, 6.241509074460763e18);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(234), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(234)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            let assign11070_ad_e17031: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p169 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0))));
            s.store_ad(152, &assign11070_ad_e17031);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad_rhs(136, 234, A::scale(s.ad_value(83), 2.0));
        }

        s.v[459] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[459] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[459] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[459] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(234), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (!(s.v[459] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(234), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_sub_ad_rhs(100, 234, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[460] = if ((((s.v[100] - s.v[234])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_sub(101, 234, 100);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p169), 90);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p170), 90);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            let assign11250_ad_e17341: A = {
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
            let assign11250_ad_e17379: A = {
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign11250_ad_e17341)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign11250_ad_e17379));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p169), 91);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p170), 91);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_sub(115, 234, 114);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            let assign11410_ad_e17654: A = {
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
            let assign11410_ad_e17692: A = {
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign11410_ad_e17654)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign11410_ad_e17692));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p169), 137);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p170), 137);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

    }

    pub(super) fn stamp_reactive_block_8(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[460] != 0.0)) {
            s.copy_ad(236, 128);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (!(s.v[460] != 0.0))) {
            s.copy_ad(236, 100);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_scalar(243, 0.0);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p163);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p164);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(235), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(234), s.ad_value(236))));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(236))), (s.v[81] / p.p9));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(234), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(234)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p161), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p161), s.ad_value(90)));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(243), s.ad_value(85)), p.p18);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_mul(86, 243, 90);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_sub(39, 234, 86);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            let assign11700_ad_e18127: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p169 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p169) / 3.0))));
            s.store_ad(152, &assign11700_ad_e18127);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[461] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[461] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[461] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[461] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (!(s.v[461] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[462] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p169), 90);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p170), 90);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            let assign11880_ad_e18437: A = {
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
            let assign11880_ad_e18475: A = {
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign11880_ad_e18437)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign11880_ad_e18475));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p169), 91);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p170), 91);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            let assign12030_ad_e18738: A = {
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
            let assign12030_ad_e18776: A = {
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign12030_ad_e18738)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign12030_ad_e18776));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p169), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p170), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (s.v[462] != 0.0)) {
            s.store_add(237, 128, 86);
        }

        if (((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) && (!(s.v[462] != 0.0))) {
            s.store_add(237, 100, 86);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_scaled_add(238, 236, 237, 0.5);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_sub(239, 237, 236);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_sub(90, 237, 236);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(234), s.ad_value(83)), 238);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(235), (p.p4 * (p.p5 * p.p161))), A::add(A::sub(s.ad_value(234), s.ad_value(238)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p236)), 1e26);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p235), 1.0);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_from_scalar(190, p.p234, 189);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p160));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_mul_ad(240, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p161))), A::add(A::sub(s.ad_value(234), s.ad_value(238)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(234), s.ad_value(83)), 238);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(236), A::scale(s.ad_value(237), 2.0)), 0.3333333333333333);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(239)), (1.0 / 12.0)), 136);
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(239)), s.ad_value(239)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((!(s.v[448] != 0.0)) && (s.v[457] != 0.0)) {
            s.store_mul_ad(241, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p161 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(234), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        if ((!(s.v[448] != 0.0)) && (!(s.v[457] != 0.0))) {
            s.store_scalar(240, 0.0);
        }

        if ((!(s.v[448] != 0.0)) && (!(s.v[457] != 0.0))) {
            s.store_scalar(241, 0.0);
        }

        s.v[463] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[464] = if (p.p152 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_ad(57, &A::voltage(ctx, &nodes, Some(16), Some(15)));
        }

        s.v[465] = if (p.p152 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[465] != 0.0)) {
            s.store_ad(58, &A::voltage(ctx, &nodes, Some(9), Some(15)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[465] != 0.0)) {
            s.store_ad(59, &A::voltage(ctx, &nodes, Some(9), Some(16)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (!(s.v[465] != 0.0))) {
            s.store_ad(58, &A::voltage(ctx, &nodes, Some(2), Some(15)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (!(s.v[465] != 0.0))) {
            s.store_ad(59, &A::voltage(ctx, &nodes, Some(2), Some(16)));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_scalar(56, 1.0);
        }

        s.v[466] = if (s.v[57] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[466] != 0.0)) {
            s.store_scalar(56, (-1.0));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[466] != 0.0)) {
            s.store_mul(255, 56, 57);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[466] != 0.0)) {
            s.copy_ad(254, 59);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (!(s.v[466] != 0.0))) {
            s.copy_ad(255, 57);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (!(s.v[466] != 0.0))) {
            s.copy_ad(254, 58);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_offset_ad(256, A::sqrt(A::offset(A::square(s.ad_value(255)), 0.01)), (-0.1));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_offset_scaled(146, 256, p.p179, (1.0 + p.p178));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_sub_ad(88, A::sub_from_scalar(p.p172, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p175)), A::div(A::scale(s.ad_value(256), (p.p181 * p.p180)), A::sqrt(A::offset(A::square(s.ad_value(256)), (p.p181 * p.p181)))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_scalar(247, (p.p9 / p.p173));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p174, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p171))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(254), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(254), s.ad_value(159)), A::sub(s.ad_value(254), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_sub(246, 160, 88);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad_rhs(84, 247, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

    }

    pub(super) fn stamp_reactive_block_9(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_scale(99, 247, 6.241509074460763e18);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(246), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(246)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            let assign12620_ad_e19533: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p182 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0))));
            s.store_ad(152, &assign12620_ad_e19533);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad_rhs(136, 246, A::scale(s.ad_value(83), 2.0));
        }

        s.v[467] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(246), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (!(s.v[467] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(246), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_sub_ad_rhs(100, 246, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[468] = if ((((s.v[100] - s.v[246])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_sub(101, 246, 100);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p182), 90);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p183), 90);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            let assign12800_ad_e19827: A = {
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
            let assign12800_ad_e19865: A = {
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign12800_ad_e19827)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign12800_ad_e19865));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p182), 91);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p183), 91);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_sub(115, 246, 114);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            let assign12960_ad_e20124: A = {
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
            let assign12960_ad_e20162: A = {
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign12960_ad_e20124)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign12960_ad_e20162));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p182), 137);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p183), 137);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.copy_ad(248, 128);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (!(s.v[468] != 0.0))) {
            s.copy_ad(248, 100);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p176);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p177);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(247), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(246), s.ad_value(248))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(248))), (s.v[81] / p.p9));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(246), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(246)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p174), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p174), s.ad_value(90)));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(255), s.ad_value(85)), p.p18);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_mul(86, 255, 90);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_sub(39, 246, 86);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            let assign13240_ad_e20562: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p182 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0))));
            s.store_ad(152, &assign13240_ad_e20562);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[469] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[469] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[469] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[469] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (!(s.v[469] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[470] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p182), 90);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p183), 90);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            let assign13420_ad_e20856: A = {
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
            let assign13420_ad_e20894: A = {
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign13420_ad_e20856)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign13420_ad_e20894));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p182), 91);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p183), 91);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

    }

    pub(super) fn stamp_reactive_block_10(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            let assign13570_ad_e21142: A = {
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
            let assign13570_ad_e21180: A = {
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign13570_ad_e21142)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign13570_ad_e21180));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_add(249, 128, 86);
        }

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (!(s.v[470] != 0.0))) {
            s.store_add(249, 100, 86);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_scaled_add(250, 248, 249, 0.5);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_sub(251, 249, 248);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_mul_ad_lhs(135, A::add(A::sub(s.ad_value(246), s.ad_value(250)), s.ad_value(83)), 251);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(247), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(246), s.ad_value(250))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(129))), (s.v[81] / p.p9));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::mul(A::scale(s.ad_value(136), p.p15), s.ad_value(136))), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_scaled_mul(96, 95, 247, (p.p4 * (p.p5 * 1.0 / (p.p174))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_mul_ad_rhs(98, 96, A::offset(A::scale(A::sub(s.ad_value(256), s.ad_value(86)), p.p21), 1.0));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_sqrt_ad(92, A::offset(A::mul(A::scale(s.ad_value(251), (p.p25 * p.p25)), s.ad_value(251)), 1.0));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div(93, 98, 92);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_sub(90, 249, 248);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(246), s.ad_value(83)), 250);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(247), (p.p4 * (p.p5 * p.p174))), A::add(A::sub(s.ad_value(246), s.ad_value(250)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p239)), 1e26);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p238), 1.0);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_from_scalar(190, p.p237, 189);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p173));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_mul_ad(252, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p174))), A::add(A::sub(s.ad_value(246), s.ad_value(250)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(246), s.ad_value(83)), 250);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(248), A::scale(s.ad_value(249), 2.0)), 0.3333333333333333);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(251)), (1.0 / 12.0)), 136);
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(251)), s.ad_value(251)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_mul_ad(253, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p174 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(246), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        s.v[471] = if (s.v[56] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[463] != 0.0) && (s.v[464] != 0.0)) && (s.v[471] != 0.0)) {
            s.store_sub_ad_lhs(253, A::scale(s.ad_value(252), (-1.0)), 253);
        }

        if ((s.v[463] != 0.0) && (!(s.v[464] != 0.0))) {
            s.store_scalar(252, 0.0);
        }

        if ((s.v[463] != 0.0) && (!(s.v[464] != 0.0))) {
            s.store_scalar(253, 0.0);
        }

        s.v[472] = if (p.p152 != 0.0) { 1.0 } else { 0.0 };

        s.v[473] = if (p.p152 == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_ad(58, &A::voltage(ctx, &nodes, Some(9), Some(7)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (!(s.v[473] != 0.0))) {
            s.store_ad(58, &A::voltage(ctx, &nodes, Some(2), Some(7)));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.copy_ad(254, 58);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_scalar(146, (1.0 + p.p178));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_sub_from_scalar_ad(88, p.p172, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p175));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_scalar(247, (p.p9 / p.p173));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p174, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p171))));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(254), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(254), s.ad_value(159)), A::sub(s.ad_value(254), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_sub(246, 160, 88);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad_rhs(84, 247, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_scale(99, 247, 6.241509074460763e18);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(246), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(246)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            let assign14160_ad_e21960: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p182 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0))));
            s.store_ad(152, &assign14160_ad_e21960);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad_rhs(136, 246, A::scale(s.ad_value(83), 2.0));
        }

        s.v[474] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[474] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[474] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[474] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(246), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (!(s.v[474] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(246), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_sub_ad_rhs(100, 246, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[475] = if ((((s.v[100] - s.v[246])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_sub(101, 246, 100);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p182), 90);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p183), 90);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            let assign14340_ad_e22270: A = {
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
            let assign14340_ad_e22308: A = {
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign14340_ad_e22270)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign14340_ad_e22308));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p182), 91);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p183), 91);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_sub(115, 246, 114);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            let assign14500_ad_e22583: A = {
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
            let assign14500_ad_e22621: A = {
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign14500_ad_e22583)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign14500_ad_e22621));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p182), 137);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p183), 137);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

    }

    pub(super) fn stamp_reactive_block_11(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[475] != 0.0)) {
            s.copy_ad(248, 128);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (!(s.v[475] != 0.0))) {
            s.copy_ad(248, 100);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_scalar(255, 0.0);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p176);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p177);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(247), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(246), s.ad_value(248))));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(248))), (s.v[81] / p.p9));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(246), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(246)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p174), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p174), s.ad_value(90)));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(255), s.ad_value(85)), p.p18);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_mul(86, 255, 90);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_sub(39, 246, 86);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            let assign14790_ad_e23056: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p182 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0))));
            s.store_ad(152, &assign14790_ad_e23056);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[476] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[476] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[476] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[476] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (!(s.v[476] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[477] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p182), 90);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p183), 90);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            let assign14970_ad_e23366: A = {
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
            let assign14970_ad_e23404: A = {
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign14970_ad_e23366)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign14970_ad_e23404));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p182), 91);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p183), 91);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            let assign15120_ad_e23667: A = {
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
            let assign15120_ad_e23705: A = {
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign15120_ad_e23667)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign15120_ad_e23705));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (s.v[477] != 0.0)) {
            s.store_add(249, 128, 86);
        }

        if (((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) && (!(s.v[477] != 0.0))) {
            s.store_add(249, 100, 86);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_scaled_add(250, 248, 249, 0.5);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_sub(251, 249, 248);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_sub(90, 249, 248);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(246), s.ad_value(83)), 250);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(247), (p.p4 * (p.p5 * p.p174))), A::add(A::sub(s.ad_value(246), s.ad_value(250)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p239)), 1e26);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p238), 1.0);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_from_scalar(190, p.p237, 189);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p173));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_mul_ad(252, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p174))), A::add(A::sub(s.ad_value(246), s.ad_value(250)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(246), s.ad_value(83)), 250);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(248), A::scale(s.ad_value(249), 2.0)), 0.3333333333333333);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(251)), (1.0 / 12.0)), 136);
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(251)), s.ad_value(251)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((!(s.v[463] != 0.0)) && (s.v[472] != 0.0)) {
            s.store_mul_ad(253, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p174 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(246), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        if ((!(s.v[463] != 0.0)) && (!(s.v[472] != 0.0))) {
            s.store_scalar(252, 0.0);
        }

        if ((!(s.v[463] != 0.0)) && (!(s.v[472] != 0.0))) {
            s.store_scalar(253, 0.0);
        }

        s.v[478] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[479] = if (p.p153 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_ad(61, &A::voltage(ctx, &nodes, Some(19), Some(20)));
        }

        s.v[480] = if (p.p153 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[480] != 0.0)) {
            s.store_ad(62, &A::voltage(ctx, &nodes, Some(9), Some(20)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[480] != 0.0)) {
            s.store_ad(63, &A::voltage(ctx, &nodes, Some(9), Some(19)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[480] != 0.0))) {
            s.store_ad(62, &A::voltage(ctx, &nodes, Some(2), Some(20)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[480] != 0.0))) {
            s.store_ad(63, &A::voltage(ctx, &nodes, Some(2), Some(19)));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scalar(60, 1.0);
        }

        s.v[481] = if (s.v[61] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[481] != 0.0)) {
            s.store_scalar(60, (-1.0));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[481] != 0.0)) {
            s.store_mul(267, 60, 61);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[481] != 0.0)) {
            s.copy_ad(266, 63);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[481] != 0.0))) {
            s.copy_ad(267, 61);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[481] != 0.0))) {
            s.copy_ad(266, 62);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_offset_ad(268, A::sqrt(A::offset(A::square(s.ad_value(267)), 0.01)), (-0.1));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_offset_scaled(146, 268, p.p179, (1.0 + p.p178));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub_ad(88, A::offset(A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p175), p.p172), A::div(A::scale(s.ad_value(268), (p.p181 * p.p180)), A::sqrt(A::offset(A::square(s.ad_value(268)), (p.p181 * p.p181)))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scalar(259, (p.p9 / p.p173));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p174, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p171))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(266), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(266), s.ad_value(159)), A::sub(s.ad_value(266), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub(258, 160, 88);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad_rhs(84, 259, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scale(99, 259, 6.241509074460763e18);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(258), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(258)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

    }

    pub(super) fn stamp_reactive_block_12(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            let assign15710_ad_e24462: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p182 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0))));
            s.store_ad(152, &assign15710_ad_e24462);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad_rhs(136, 258, A::scale(s.ad_value(83), 2.0));
        }

        s.v[482] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[482] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(258), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[482] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(258), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub_ad_rhs(100, 258, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[483] = if ((((s.v[100] - s.v[258])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_sub(101, 258, 100);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p182), 90);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p183), 90);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            let assign15890_ad_e24756: A = {
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
            let assign15890_ad_e24794: A = {
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign15890_ad_e24756)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign15890_ad_e24794));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p182), 91);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p183), 91);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_sub(115, 258, 114);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            let assign16050_ad_e25053: A = {
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
            let assign16050_ad_e25091: A = {
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign16050_ad_e25053)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign16050_ad_e25091));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p182), 137);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p183), 137);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[483] != 0.0)) {
            s.copy_ad(260, 128);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[483] != 0.0))) {
            s.copy_ad(260, 100);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p176);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p177);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(259), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(258), s.ad_value(260))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(260))), (s.v[81] / p.p9));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(258), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(258)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p174), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p174), s.ad_value(90)));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(267), s.ad_value(85)), p.p18);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul(86, 267, 90);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub(39, 258, 86);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            let assign16330_ad_e25491: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p182 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0))));
            s.store_ad(152, &assign16330_ad_e25491);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[484] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[484] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[484] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[484] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[484] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[485] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p182), 90);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p183), 90);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            let assign16510_ad_e25785: A = {
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
            let assign16510_ad_e25823: A = {
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign16510_ad_e25785)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign16510_ad_e25823));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p182), 91);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p183), 91);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

    }

    pub(super) fn stamp_reactive_block_13(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            let assign16660_ad_e26071: A = {
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
            let assign16660_ad_e26109: A = {
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign16660_ad_e26071)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign16660_ad_e26109));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_add(261, 128, 86);
        }

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (!(s.v[485] != 0.0))) {
            s.store_add(261, 100, 86);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scaled_add(262, 260, 261, 0.5);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub(263, 261, 260);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad_lhs(135, A::add(A::sub(s.ad_value(258), s.ad_value(262)), s.ad_value(83)), 263);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(259), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(258), s.ad_value(262))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(129))), (s.v[81] / p.p9));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::mul(A::scale(s.ad_value(136), p.p15), s.ad_value(136))), A::scale(s.ad_value(90), p.p16)));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scaled_mul(96, 95, 259, (p.p4 * (p.p5 * 1.0 / (p.p174))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad_rhs(98, 96, A::offset(A::scale(A::sub(s.ad_value(268), s.ad_value(86)), p.p21), 1.0));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sqrt_ad(92, A::offset(A::mul(A::scale(s.ad_value(263), (p.p25 * p.p25)), s.ad_value(263)), 1.0));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div(93, 98, 92);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub(90, 261, 260);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(258), s.ad_value(83)), 262);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(259), (p.p4 * (p.p5 * p.p174))), A::add(A::sub(s.ad_value(258), s.ad_value(262)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p239)), 1e26);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p238), 1.0);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_from_scalar(190, p.p237, 189);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p173));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad(264, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p174))), A::add(A::sub(s.ad_value(258), s.ad_value(262)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(258), s.ad_value(83)), 262);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(260), A::scale(s.ad_value(261), 2.0)), 0.3333333333333333);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(263)), (1.0 / 12.0)), 136);
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(263)), s.ad_value(263)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((s.v[478] != 0.0) && (s.v[479] != 0.0)) {
            s.store_mul_ad(265, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p174 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(258), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        s.v[486] = if (s.v[60] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[478] != 0.0) && (s.v[479] != 0.0)) && (s.v[486] != 0.0)) {
            s.store_sub_ad_lhs(265, A::scale(s.ad_value(264), (-1.0)), 265);
        }

        if ((s.v[478] != 0.0) && (!(s.v[479] != 0.0))) {
            s.store_scalar(264, 0.0);
        }

        if ((s.v[478] != 0.0) && (!(s.v[479] != 0.0))) {
            s.store_scalar(265, 0.0);
        }

        s.v[487] = if (p.p153 != 0.0) { 1.0 } else { 0.0 };

        s.v[488] = if (p.p153 == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[488] != 0.0)) {
            s.store_ad(62, &A::voltage(ctx, &nodes, Some(9), Some(8)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (!(s.v[488] != 0.0))) {
            s.store_ad(62, &A::voltage(ctx, &nodes, Some(2), Some(8)));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.copy_ad(266, 62);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_scalar(146, (1.0 + p.p178));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_mul_ad_lhs(83, A::scale(s.ad_value(82), 8.617087e-5), 146);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_offset_ad(88, A::scale(A::offset(A::scale(s.ad_value(82), 1.0 / (s.v[35])), (-1.0)), p.p175), p.p172);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_scalar(259, (p.p9 / p.p173));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_from_scalar_ad(136, p.p174, A::mul(A::scale(s.ad_value(83), (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17)), s.ad_value(83)));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_add_ad_rhs(159, 88, A::mul(s.ad_value(83), A::ln(A::scale(s.ad_value(136), p.p171))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_add_ad_lhs(160, A::scale(A::add(A::sub(s.ad_value(266), s.ad_value(159)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(266), s.ad_value(159)), A::sub(s.ad_value(266), s.ad_value(159))), 0.0001))), 0.5), 159);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_sub(258, 160, 88);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad_rhs(84, 259, A::scale(s.ad_value(83), (1.602176634e-19 * 3.24e17)));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_from_scalar(150, 2.718281828459045, 84);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_from_scalar(151, 1.0, 84);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_scale(99, 259, 6.241509074460763e18);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_add_ad(154, A::scale(s.ad_value(258), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(258)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad(155, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad(130, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            let assign17250_ad_e26889: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p182 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(130)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0))));
            s.store_ad(152, &assign17250_ad_e26889);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad_rhs(136, 258, A::scale(s.ad_value(83), 2.0));
        }

        s.v[489] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[489] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[489] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[489] != 0.0)) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(258), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (!(s.v[489] != 0.0))) {
            s.store_div_ad(153, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(258), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_sub_ad_rhs(100, 258, A::div(s.ad_value(153), s.ad_value(99)));
        }

        s.v[490] = if ((((s.v[100] - s.v[258])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_sub(101, 258, 100);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p182), 90);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p183), 90);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            let assign17430_ad_e27199: A = {
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
            let assign17430_ad_e27237: A = {
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign17430_ad_e27199)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign17430_ad_e27237));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p182), 91);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p183), 91);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_sub(115, 258, 114);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_powf(137, 115, (-0.3333333333333333));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            let assign17590_ad_e27512: A = {
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
            let assign17590_ad_e27550: A = {
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign17590_ad_e27512)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign17590_ad_e27550));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad_lhs(121, A::scale(s.ad_value(136), p.p182), 137);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad_lhs(122, A::scale(s.ad_value(136), p.p183), 137);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

    }

    pub(super) fn stamp_reactive_block_14(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[490] != 0.0)) {
            s.copy_ad(260, 128);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (!(s.v[490] != 0.0))) {
            s.copy_ad(260, 100);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_scalar(267, 0.0);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_scale_ad(97, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20), p.p176);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_scale_ad(89, A::powf(A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19), p.p177);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_mul_ad(136, A::scale(s.ad_value(259), 1.0 / (p.p9)), A::abs(A::sub(s.ad_value(258), s.ad_value(260))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_scale_ad(90, A::abs(A::sub(s.ad_value(45), s.ad_value(260))), (s.v[81] / p.p9));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad_rhs(95, 97, A::add(A::add(A::offset(A::scale(s.ad_value(136), p.p14), 1.0), A::scale(A::square(s.ad_value(136)), p.p15)), A::scale(s.ad_value(90), p.p16)));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad_lhs(136, A::scale(s.ad_value(89), 2.0), 95);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_add_ad(90, A::scale(s.ad_value(258), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(258)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad(85, A::mul(A::scale(s.ad_value(136), p.p174), s.ad_value(90)), A::add(A::scale(s.ad_value(136), p.p174), s.ad_value(90)));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_powf_ad(136, A::div(s.ad_value(267), s.ad_value(85)), p.p18);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_powf_ad(90, A::offset(s.ad_value(136), 1.0), ((-1.0) / p.p18));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_mul(86, 267, 90);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_sub(39, 258, 86);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.copy_ad(130, 39);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_add_ad(131, A::scale(s.ad_value(130), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(130)), ((4.0 * 0.3) * 0.3))), 0.5));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.copy_ad(154, 131);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad(157, A::mul(s.ad_value(154), s.ad_value(150)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(150)))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad(158, A::mul(s.ad_value(154), s.ad_value(151)), A::sqrt(A::add(A::square(s.ad_value(154)), A::square(s.ad_value(151)))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            let assign17880_ad_e27985: A = A::div(A::sub(A::add(s.ad_value(154), A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))))), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), (p.p182 / 3.0))), A::add(A::mul(s.ad_value(154), A::offset(A::div(s.ad_value(83), s.ad_value(158)), 1.0)), A::scale(A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666), ((2.0 * p.p182) / 3.0))));
            s.store_ad(152, &assign17880_ad_e27985);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad_rhs(136, 130, A::scale(s.ad_value(83), 2.0));
        }

        s.v[491] = if (s.v[136] < 200.0) { 1.0 } else { 0.0 };

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[491] != 0.0)) {
            s.store_limited_exp_ad(90, A::scale(s.ad_value(136), 0.25));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[491] != 0.0)) {
            s.store_limited_exp_ad(91, A::scale(s.ad_value(136), ((-3.0) * 0.25)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[491] != 0.0)) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), A::add(A::scale(s.ad_value(136), (3.0 * 0.25)), A::ln(A::add(s.ad_value(90), s.ad_value(91))))), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (!(s.v[491] != 0.0))) {
            s.store_div_ad(156, A::mul(A::mul(A::scale(s.ad_value(83), 2.0), s.ad_value(99)), s.ad_value(136)), A::add(A::div_from_scalar(1.0, s.ad_value(152)), A::mul(A::scale(s.ad_value(99), 3.08641975308642e-18), A::limited_exp(A::div(A::scale(s.ad_value(130), (-1.0)), A::scale(s.ad_value(83), 2.0))))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_sub_ad_rhs(100, 130, A::div(s.ad_value(156), s.ad_value(99)));
        }

        s.v[492] = if ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19) { 1.0 } else { 0.0 };

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_sub(101, 130, 100);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_add_ad(101, A::scale(s.ad_value(101), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(101)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_powf(136, 99, 0.6666666666666666);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_powf(90, 101, 0.6666666666666666);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_powf(91, 101, (-0.3333333333333333));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad_lhs(102, A::scale(s.ad_value(136), p.p182), 90);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad_lhs(103, A::scale(s.ad_value(136), p.p183), 90);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_sub_ad(104, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(102), s.ad_value(83)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_sub_ad(105, A::div(s.ad_value(100), s.ad_value(83)), A::div(s.ad_value(103), s.ad_value(83)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            let assign18060_ad_e28295: A = {
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
            let assign18060_ad_e28333: A = {
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
            s.store_sub_ad(106, A::sub(A::mul(s.ad_value(99), s.ad_value(101)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign18060_ad_e28295)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign18060_ad_e28333));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad_lhs(107, A::scale(s.ad_value(136), p.p182), 91);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad_lhs(108, A::scale(s.ad_value(136), p.p183), 91);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad(109, A::scale(A::limited_exp(s.ad_value(104)), 3.24e17), A::offset(A::scale(s.ad_value(107), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_offset_ad(110, A::limited_exp(s.ad_value(104)), 1.0);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad(111, A::scale(A::limited_exp(s.ad_value(105)), 3.24e17), A::offset(A::scale(s.ad_value(108), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_offset_ad(112, A::limited_exp(s.ad_value(105)), 1.0);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_sub_ad(113, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_sub_ad_rhs(114, 100, A::div(s.ad_value(106), s.ad_value(113)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_sub(115, 130, 114);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_add_ad(115, A::scale(s.ad_value(115), 0.5), A::scale(A::sqrt(A::offset(A::square(s.ad_value(115)), ((4.0 * 1e-9) * 1e-9))), 0.5));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad(116, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad(117, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), 0.6666666666666666));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_sub_ad(118, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(116), s.ad_value(83)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_sub_ad(119, A::div(s.ad_value(114), s.ad_value(83)), A::div(s.ad_value(117), s.ad_value(83)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            let assign18210_ad_e28596: A = {
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
            let assign18210_ad_e28634: A = {
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
            s.store_sub_ad(120, A::sub(A::mul(s.ad_value(99), s.ad_value(115)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign18210_ad_e28596)), A::mul(A::scale(s.ad_value(83), 3.24e17), assign18210_ad_e28634));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad(121, A::scale(s.ad_value(136), p.p182), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad(122, A::scale(s.ad_value(136), p.p183), A::powf(s.ad_value(115), (-0.3333333333333333)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad(123, A::scale(A::limited_exp(s.ad_value(118)), 3.24e17), A::offset(A::scale(s.ad_value(121), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_offset_ad(124, A::limited_exp(s.ad_value(118)), 1.0);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_mul_ad(125, A::scale(A::limited_exp(s.ad_value(119)), 3.24e17), A::offset(A::scale(s.ad_value(122), 0.6666666666666666), 1.0));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_offset_ad(126, A::limited_exp(s.ad_value(119)), 1.0);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_sub_ad(127, A::sub(A::scale(s.ad_value(99), (-1.0)), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_sub_ad_rhs(128, 114, A::div(s.ad_value(120), s.ad_value(127)));
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (s.v[492] != 0.0)) {
            s.store_add(261, 128, 86);
        }

        if (((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) && (!(s.v[492] != 0.0))) {
            s.store_add(261, 100, 86);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_scaled_add(262, 260, 261, 0.5);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_sub(263, 261, 260);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_sub(90, 261, 260);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_sub_ad_lhs(91, A::add(s.ad_value(258), s.ad_value(83)), 262);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_mul_ad(137, A::scale(s.ad_value(259), (p.p4 * (p.p5 * p.p174))), A::add(A::sub(s.ad_value(258), s.ad_value(262)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_scale_ad(188, A::scale(s.ad_value(137), 1.0 / (p.p239)), 1e26);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_offset_ad(189, A::powf(s.ad_value(188), p.p238), 1.0);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_from_scalar(190, p.p237, 189);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_from_scalar_ad(191, p.p9, A::offset(s.ad_value(190), p.p173));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_mul_ad(264, A::scale(s.ad_value(191), (p.p4 * (p.p5 * p.p174))), A::add(A::sub(s.ad_value(258), s.ad_value(262)), A::div(A::mul(A::scale(s.ad_value(90), 0.5), s.ad_value(90)), A::scale(s.ad_value(91), 6.0))));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_sub_ad_lhs(136, A::add(s.ad_value(258), s.ad_value(83)), 262);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_scale_ad(90, A::add(s.ad_value(260), A::scale(s.ad_value(261), 2.0)), 0.3333333333333333);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad_lhs(91, A::scale(A::square(s.ad_value(263)), (1.0 / 12.0)), 136);
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_div_ad(137, A::scale(A::mul(A::square(s.ad_value(263)), s.ad_value(263)), (1.0 / 120.0)), A::square(s.ad_value(136)));
        }

        if ((!(s.v[478] != 0.0)) && (s.v[487] != 0.0)) {
            s.store_mul_ad(265, A::neg(A::scale(s.ad_value(191), (p.p4 * (p.p174 * (p.p5 * 0.5))))), A::add(A::add(A::sub(s.ad_value(258), s.ad_value(90)), s.ad_value(91)), s.ad_value(137)));
        }

        if ((!(s.v[478] != 0.0)) && (!(s.v[487] != 0.0))) {
            s.store_scalar(264, 0.0);
        }

        if ((!(s.v[478] != 0.0)) && (!(s.v[487] != 0.0))) {
            s.store_scalar(265, 0.0);
        }

        s.v[493] = if (p.p149 == 0.0) { 1.0 } else { 0.0 };

        s.v[494] = if (p.p154 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[493] != 0.0) && (s.v[494] != 0.0)) {
            s.store_ad(65, &A::voltage(ctx, &nodes, Some(17), Some(16)));
        }

        s.v[495] = if (p.p154 == 1.0) { 1.0 } else { 0.0 };

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

    }

    pub(super) fn stamp_reactive_block_15(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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

    }

    pub(super) fn stamp_reactive_block_16(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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

    }

    pub(super) fn stamp_reactive_block_17(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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

    }

    pub(super) fn stamp_reactive_block_18(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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

    }
}
