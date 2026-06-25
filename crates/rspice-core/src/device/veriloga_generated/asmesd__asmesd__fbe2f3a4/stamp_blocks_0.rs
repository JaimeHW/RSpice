#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.store_offset_ad(12, A::offset(A::voltage(ctx, &nodes, Some(3), None), ctx.temperature()), p.p45);

        if ((1026.85 + 273.15) < (if (s.v[12] > ((-100.0) + 273.15)) { s.v[12] } else { ((-100.0) + 273.15) })) {
            s.store_scalar(10, (1026.85 + 273.15));
        } else {
            s.store_ad(10, &{
                if (s.v[12] > ((-100.0) + 273.15)) {
                    s.ad_value(12)
                } else {
                    A::constant(((-100.0) + 273.15))
                }
            });
        }

        s.v[3] = (p.p43 * p.p42);

        s.store_ad(95, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(4)), p.p29));

        s.store_offset_ad(94, A::scale(A::powf(A::neg(A::min_with_scalar(s.ad_value(95), 0.0)), p.p80), p.p79), 1.0);

        s.v[11] = (p.p25 + 273.15);

        s.store_scale(15, 10, 8.6170869e-5);

        s.store_scale(13, 10, 1.0 / (s.v[11]));

        s.store_ln(14, 13);

        s.store_exp_ad(18, A::scale(s.ad_value(14), p.p77));

        s.store_mul_ad_lhs(16, A::scale(s.ad_value(18), p.p52), 94);

        s.store_scale(17, 18, p.p60);

        s.v[64] = (if (p.p53 > 0.0) { (1.0 / p.p53) } else { 0.0 });

        s.v[65] = (if (p.p62 > 0.0) { (1.0 / p.p62) } else { 0.0 });

        s.v[66] = (if (p.p54 > 0.0) { (1.0 / p.p54) } else { 0.0 });

        s.v[67] = (if (p.p63 > 0.0) { (1.0 / p.p63) } else { 0.0 });

        s.store_add_ad(68, A::scale(s.ad_value(14), p.p22), A::div(A::scale(A::offset(s.ad_value(13), (-1.0)), p.p21), s.ad_value(15)));

        s.store_scale(92, 14, p.p23);

        s.store_scaled_exp(19, 68, p.p0);

        s.store_scaled_exp(93, 92, p.p2);

        s.store_div_ad_lhs(20, A::scale(A::exp(A::scale(s.ad_value(68), 1.0 / (p.p59))), p.p58), 18);

        s.store_div_ad_lhs(21, A::scale(A::exp(A::scale(s.ad_value(68), 1.0 / (p.p65))), p.p64), 18);

        s.store_scale_ad(28, A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), p.p7), 1.0), p.p47);

        s.store_scale_ad(30, A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), p.p6), 1.0), p.p5);

        s.store_scale_ad(31, A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), p.p10), 1.0), p.p9);

        s.store_scale_ad(29, A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), p.p55), 1.0), p.p56);

        s.v[32] = p.p16;

        s.v[33] = p.p69;

        s.v[34] = p.p74;

        s.v[81] = (s.v[11] / 300.15);

        s.store_scale(82, 10, 0.003331667499583542);

        s.store_sub_from_scalar_ad(83, 1.16, A::div(A::mul(A::scale(s.ad_value(10), 0.000702), s.ad_value(10)), A::offset(s.ad_value(10), 1108.0)));

        s.store_offset_ad(84, A::div(A::neg(s.ad_value(83)), A::scale(s.ad_value(10), (2.0 * 1.3806226e-23))), (1.1150877 / (1.3806226e-23 * (300.15 + 300.15))));

        s.store_mul_ad(85, A::neg(A::scale(s.ad_value(15), 2.0)), A::add(A::scale(A::ln(s.ad_value(82)), 1.5), A::scale(s.ad_value(84), 1.6021918e-19)));

        s.store_scale_ad(86, A::sub_from_scalar(p.p17, s.ad_value(85)), 1.0 / (s.v[81]));

        s.store_div_ad_lhs(87, A::sub_from_scalar(p.p17, s.ad_value(86)), 86);

        s.store_div_from_scalar_ad(89, s.v[32], A::offset(A::scale(A::sub_from_scalar((0.0004 * (s.v[11] - 300.15)), s.ad_value(87)), p.p18), 1.0));

        s.store_add_ad_lhs(25, A::mul(s.ad_value(82), s.ad_value(86)), 85);

        s.store_div_ad_lhs(88, A::sub(s.ad_value(25), s.ad_value(86)), 86);

        s.store_mul_ad_rhs(22, 89, A::offset(A::scale(A::sub(A::scale(A::offset(s.ad_value(10), (-300.15)), 0.0004), s.ad_value(88)), p.p18), 1.0));

        s.v[81] = (s.v[11] / 300.15);

        s.store_scale(82, 10, 0.003331667499583542);

        s.store_sub_from_scalar_ad(83, 1.16, A::div(A::mul(A::scale(s.ad_value(10), 0.000702), s.ad_value(10)), A::offset(s.ad_value(10), 1108.0)));

        s.store_offset_ad(84, A::div(A::neg(s.ad_value(83)), A::scale(s.ad_value(10), (2.0 * 1.3806226e-23))), (1.1150877 / (1.3806226e-23 * (300.15 + 300.15))));

        s.store_mul_ad(85, A::neg(A::scale(s.ad_value(15), 2.0)), A::add(A::scale(A::ln(s.ad_value(82)), 1.5), A::scale(s.ad_value(84), 1.6021918e-19)));

        s.store_scale_ad(86, A::sub_from_scalar(p.p70, s.ad_value(85)), 1.0 / (s.v[81]));

        s.store_div_ad_lhs(87, A::sub_from_scalar(p.p70, s.ad_value(86)), 86);

        s.store_div_from_scalar_ad(89, s.v[33], A::offset(A::scale(A::sub_from_scalar((0.0004 * (s.v[11] - 300.15)), s.ad_value(87)), p.p71), 1.0));

        s.store_add_ad_lhs(26, A::mul(s.ad_value(82), s.ad_value(86)), 85);

        s.store_div_ad_lhs(88, A::sub(s.ad_value(26), s.ad_value(86)), 86);

        s.store_mul_ad_rhs(23, 89, A::offset(A::scale(A::sub(A::scale(A::offset(s.ad_value(10), (-300.15)), 0.0004), s.ad_value(88)), p.p71), 1.0));

        s.v[81] = (s.v[11] / 300.15);

        s.store_scale(82, 10, 0.003331667499583542);

        s.store_sub_from_scalar_ad(83, 1.16, A::div(A::mul(A::scale(s.ad_value(10), 0.000702), s.ad_value(10)), A::offset(s.ad_value(10), 1108.0)));

        s.store_offset_ad(84, A::div(A::neg(s.ad_value(83)), A::scale(s.ad_value(10), (2.0 * 1.3806226e-23))), (1.1150877 / (1.3806226e-23 * (300.15 + 300.15))));

        s.store_mul_ad(85, A::neg(A::scale(s.ad_value(15), 2.0)), A::add(A::scale(A::ln(s.ad_value(82)), 1.5), A::scale(s.ad_value(84), 1.6021918e-19)));

        s.store_scale_ad(86, A::sub_from_scalar(p.p75, s.ad_value(85)), 1.0 / (s.v[81]));

        s.store_div_ad_lhs(87, A::sub_from_scalar(p.p75, s.ad_value(86)), 86);

        s.store_div_from_scalar_ad(89, s.v[34], A::offset(A::scale(A::sub_from_scalar((0.0004 * (s.v[11] - 300.15)), s.ad_value(87)), p.p76), 1.0));

        s.store_add_ad_lhs(27, A::mul(s.ad_value(82), s.ad_value(86)), 85);

        s.store_div_ad_lhs(88, A::sub(s.ad_value(27), s.ad_value(86)), 86);

        s.store_mul_ad_rhs(24, 89, A::offset(A::scale(A::sub(A::scale(A::offset(s.ad_value(10), (-300.15)), 0.0004), s.ad_value(88)), p.p76), 1.0));

        s.v[9] = p.p29;

        s.store_ad(75, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(4)), s.v[9]));

        s.store_ad(76, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(6)), s.v[9]));

        s.store_ad(77, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(4)), s.v[9]));

        s.store_ad(78, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(4)), s.v[9]));

        s.store_ad(79, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(5)), s.v[9]));

        s.store_ad(80, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(6)), s.v[9]));

        s.v[105] = if (s.v[19] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[105] != 0.0) {
            s.store_div_ad_rhs(0, 76, A::scale(s.ad_value(15), p.p1));
        }

        if (s.v[105] != 0.0) {
            s.store_div_ad(90, A::sub(A::neg(s.ad_value(76)), s.ad_value(30)), A::scale(s.ad_value(15), p.p11));
        }

        if (s.v[105] != 0.0) {
            s.store_div_ad(91, A::neg(s.ad_value(30)), A::scale(s.ad_value(15), p.p11));
        }

        s.v[106] = if (s.v[0] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[105] != 0.0) && (s.v[106] != 0.0)) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
        }

        if ((s.v[105] != 0.0) && (s.v[106] != 0.0)) {
            s.store_scalar(0, 80.0);
        }

        if ((s.v[105] != 0.0) && (!(s.v[106] != 0.0))) {
            s.store_scalar(1, 1.0);
        }

        if (s.v[105] != 0.0) {
            s.store_mul_ad_rhs(1, 1, A::exp(s.ad_value(0)));
        }

        if (s.v[105] != 0.0) {
            let assign800_ad_e1032: A = {
                if ((!(s.v[90] >= 37.0)) && (!(s.v[90] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(90)), 1.0))
                } else {
                    {
                        if ((!(s.v[90] >= 37.0)) && (s.v[90] <= (-37.0))) {
                            A::exp(s.ad_value(90))
                        } else {
                            {
                                if (s.v[90] >= 37.0) {
                                    s.ad_value(90)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign800_ad_e1066: A = {
                if ((!(s.v[91] >= 37.0)) && (!(s.v[91] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(91)), 1.0))
                } else {
                    {
                        if ((!(s.v[91] >= 37.0)) && (s.v[91] <= (-37.0))) {
                            A::exp(s.ad_value(91))
                        } else {
                            {
                                if (s.v[91] >= 37.0) {
                                    s.ad_value(91)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(2, assign800_ad_e1032, assign800_ad_e1066);
        }

        if (s.v[105] != 0.0) {
            s.store_sub_ad(35, A::mul(s.ad_value(19), A::offset(s.ad_value(1), (-1.0))), A::div(A::mul(s.ad_value(28), s.ad_value(2)), A::offset(A::scale(A::pow(A::abs(s.ad_value(76)), s.ad_value(31)), p.p8), 1.0)));
        }

        if (!(s.v[105] != 0.0)) {
            s.store_scalar(35, 0.0);
        }

        s.v[107] = if (s.v[93] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[107] != 0.0) {
            s.store_max_with_scalar_ad(101, A::sub_from_scalar(p.p4, s.ad_value(76)), 0.001);
        }

        if (s.v[107] != 0.0) {
            s.store_div_ad(0, A::scale(s.ad_value(76), ((-1.0) * p.p4)), A::mul(A::scale(s.ad_value(15), p.p3), s.ad_value(101)));
        }

        s.v[108] = if (s.v[0] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[107] != 0.0) && (s.v[108] != 0.0)) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
        }

        if ((s.v[107] != 0.0) && (s.v[108] != 0.0)) {
            s.store_scalar(0, 80.0);
        }

        if ((s.v[107] != 0.0) && (!(s.v[108] != 0.0))) {
            s.store_scalar(1, 1.0);
        }

        if (s.v[107] != 0.0) {
            s.store_mul_ad_rhs(1, 1, A::exp(s.ad_value(0)));
        }

        if (s.v[107] != 0.0) {
            s.store_mul_ad_rhs(47, 93, A::offset(s.ad_value(1), (-1.0)));
        }

        if (!(s.v[107] != 0.0)) {
            s.store_scalar(47, 0.0);
        }

        s.v[109] = if (s.v[20] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[109] != 0.0) {
            s.store_div_ad_rhs(0, 76, A::scale(s.ad_value(15), p.p59));
        }

        if (s.v[109] != 0.0) {
            s.store_div_ad(90, A::sub(A::neg(s.ad_value(76)), s.ad_value(30)), A::scale(s.ad_value(15), p.p57));
        }

        if (s.v[109] != 0.0) {
            s.store_div_ad(91, A::neg(s.ad_value(30)), A::scale(s.ad_value(15), p.p57));
        }

        s.v[110] = if (s.v[0] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[109] != 0.0) && (s.v[110] != 0.0)) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
        }

        if ((s.v[109] != 0.0) && (s.v[110] != 0.0)) {
            s.store_scalar(0, 80.0);
        }

        if ((s.v[109] != 0.0) && (!(s.v[110] != 0.0))) {
            s.store_scalar(1, 1.0);
        }

        if (s.v[109] != 0.0) {
            s.store_mul_ad_rhs(1, 1, A::exp(s.ad_value(0)));
        }

        if (s.v[109] != 0.0) {
            let assign1020_ad_e1266: A = {
                if ((!(s.v[90] >= 37.0)) && (!(s.v[90] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(90)), 1.0))
                } else {
                    {
                        if ((!(s.v[90] >= 37.0)) && (s.v[90] <= (-37.0))) {
                            A::exp(s.ad_value(90))
                        } else {
                            {
                                if (s.v[90] >= 37.0) {
                                    s.ad_value(90)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign1020_ad_e1300: A = {
                if ((!(s.v[91] >= 37.0)) && (!(s.v[91] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(91)), 1.0))
                } else {
                    {
                        if ((!(s.v[91] >= 37.0)) && (s.v[91] <= (-37.0))) {
                            A::exp(s.ad_value(91))
                        } else {
                            {
                                if (s.v[91] >= 37.0) {
                                    s.ad_value(91)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(2, assign1020_ad_e1266, assign1020_ad_e1300);
        }

        if (s.v[109] != 0.0) {
            s.store_sub_ad(36, A::mul(s.ad_value(20), A::offset(s.ad_value(1), (-1.0))), A::div(A::scale(s.ad_value(2), 0.0), A::offset(A::scale(A::pow(A::abs(s.ad_value(76)), s.ad_value(31)), p.p8), 1.0)));
        }

        if (!(s.v[109] != 0.0)) {
            s.store_scalar(36, 0.0);
        }

        s.v[111] = if (s.v[19] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[111] != 0.0) {
            s.store_div_ad_rhs(0, 77, A::scale(s.ad_value(15), p.p61));
        }

        if (s.v[111] != 0.0) {
            s.store_div_ad(90, A::sub(A::neg(s.ad_value(77)), s.ad_value(30)), A::scale(s.ad_value(15), p.p57));
        }

        if (s.v[111] != 0.0) {
            s.store_div_ad(91, A::neg(s.ad_value(30)), A::scale(s.ad_value(15), p.p57));
        }

        s.v[112] = if (s.v[0] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[111] != 0.0) && (s.v[112] != 0.0)) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
        }

        if ((s.v[111] != 0.0) && (s.v[112] != 0.0)) {
            s.store_scalar(0, 80.0);
        }

        if ((s.v[111] != 0.0) && (!(s.v[112] != 0.0))) {
            s.store_scalar(1, 1.0);
        }

        if (s.v[111] != 0.0) {
            s.store_mul_ad_rhs(1, 1, A::exp(s.ad_value(0)));
        }

        if (s.v[111] != 0.0) {
            let assign1140_ad_e1428: A = {
                if ((!(s.v[90] >= 37.0)) && (!(s.v[90] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(90)), 1.0))
                } else {
                    {
                        if ((!(s.v[90] >= 37.0)) && (s.v[90] <= (-37.0))) {
                            A::exp(s.ad_value(90))
                        } else {
                            {
                                if (s.v[90] >= 37.0) {
                                    s.ad_value(90)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign1140_ad_e1462: A = {
                if ((!(s.v[91] >= 37.0)) && (!(s.v[91] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(91)), 1.0))
                } else {
                    {
                        if ((!(s.v[91] >= 37.0)) && (s.v[91] <= (-37.0))) {
                            A::exp(s.ad_value(91))
                        } else {
                            {
                                if (s.v[91] >= 37.0) {
                                    s.ad_value(91)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(2, assign1140_ad_e1428, assign1140_ad_e1462);
        }

        if (s.v[111] != 0.0) {
            s.store_sub_ad(38, A::mul(s.ad_value(19), A::offset(s.ad_value(1), (-1.0))), A::div(A::mul(s.ad_value(29), s.ad_value(2)), A::offset(A::scale(A::pow(A::abs(s.ad_value(77)), s.ad_value(31)), p.p8), 1.0)));
        }

        if (!(s.v[111] != 0.0)) {
            s.store_scalar(38, 0.0);
        }

        s.v[113] = if (s.v[21] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[113] != 0.0) {
            s.store_div_ad_rhs(0, 77, A::scale(s.ad_value(15), p.p65));
        }

        if (s.v[113] != 0.0) {
            s.store_div_ad(90, A::sub(A::neg(s.ad_value(77)), s.ad_value(30)), A::scale(s.ad_value(15), p.p57));
        }

        if (s.v[113] != 0.0) {
            s.store_div_ad(91, A::neg(s.ad_value(30)), A::scale(s.ad_value(15), p.p57));
        }

        s.v[114] = if (s.v[0] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[113] != 0.0) && (s.v[114] != 0.0)) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
        }

        if ((s.v[113] != 0.0) && (s.v[114] != 0.0)) {
            s.store_scalar(0, 80.0);
        }

        if ((s.v[113] != 0.0) && (!(s.v[114] != 0.0))) {
            s.store_scalar(1, 1.0);
        }

        if (s.v[113] != 0.0) {
            s.store_mul_ad_rhs(1, 1, A::exp(s.ad_value(0)));
        }

        if (s.v[113] != 0.0) {
            let assign1260_ad_e1590: A = {
                if ((!(s.v[90] >= 37.0)) && (!(s.v[90] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(90)), 1.0))
                } else {
                    {
                        if ((!(s.v[90] >= 37.0)) && (s.v[90] <= (-37.0))) {
                            A::exp(s.ad_value(90))
                        } else {
                            {
                                if (s.v[90] >= 37.0) {
                                    s.ad_value(90)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign1260_ad_e1624: A = {
                if ((!(s.v[91] >= 37.0)) && (!(s.v[91] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(91)), 1.0))
                } else {
                    {
                        if ((!(s.v[91] >= 37.0)) && (s.v[91] <= (-37.0))) {
                            A::exp(s.ad_value(91))
                        } else {
                            {
                                if (s.v[91] >= 37.0) {
                                    s.ad_value(91)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(2, assign1260_ad_e1590, assign1260_ad_e1624);
        }

        if (s.v[113] != 0.0) {
            s.store_sub_ad(39, A::mul(s.ad_value(21), A::offset(s.ad_value(1), (-1.0))), A::div(A::scale(s.ad_value(2), 0.0), A::offset(A::scale(A::powf(A::abs(s.ad_value(77)), p.p9), p.p8), 1.0)));
        }

        if (!(s.v[113] != 0.0)) {
            s.store_scalar(39, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_1(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.store_abs_ad(102, A::div(A::min(A::voltage(ctx, &nodes, Some(9), None), s.ad_value(76)), A::max_with_scalar(A::abs(s.ad_value(76)), 1e-9)));

        s.store_add_ad_lhs(37, A::div(A::sub(s.ad_value(35), s.ad_value(47)), s.ad_value(16)), 36);

        s.store_add_ad_lhs(40, A::div(s.ad_value(38), s.ad_value(17)), 39);

        s.store_scale_ad(66, A::offset(A::scale(s.ad_value(77), p.p81), 1.0), s.v[66]);

        s.store_add_ad(42, A::mul(s.ad_value(35), s.ad_value(66)), A::scale(s.ad_value(38), s.v[67]));

        s.store_sub_ad(41, A::sub_from_scalar(1.0, A::scale(s.ad_value(76), s.v[65])), A::scale(s.ad_value(77), s.v[64]));

        s.store_offset_ad(96, A::powf(A::abs(A::offset(A::scale(s.ad_value(42), 4.0), 1.0)), p.p82), 1.0);

        s.store_div_ad_lhs(43, A::scale(s.ad_value(41), 2.0), 96);

        s.store_mul(45, 38, 43);

        s.store_mul(44, 35, 43);

        s.store_add_ad(46, A::scale(A::mul(A::mul(s.ad_value(35), s.ad_value(43)), s.ad_value(102)), p.p84), A::mul(A::scale(s.ad_value(35), (1.0 - p.p84)), s.ad_value(43)));

        s.store_offset_ad(99, A::powf(A::abs(A::scale(s.ad_value(79), 1.0 / (p.p48))), p.p49), 1.0);

        s.store_offset_ad(100, A::powf(A::abs(A::scale(s.ad_value(80), 1.0 / (p.p50))), p.p51), 1.0);

        s.store_mul_ad(51, A::scale(A::exp(A::scale(s.ad_value(14), p.p37)), p.p12), A::powf(s.ad_value(99), (1.0 / p.p49)));

        s.store_scale_ad(52, A::exp(A::scale(s.ad_value(14), p.p78)), p.p66);

        s.store_mul_ad(53, A::scale(A::exp(A::scale(s.ad_value(14), p.p38)), p.p14), A::powf(s.ad_value(100), (1.0 / p.p51)));

        s.store_powf_ad(97, A::abs(A::scale(A::voltage(ctx, &nodes, Some(1), Some(2)), 1.0 / (p.p40))), p.p39);

        s.store_offset_ad(98, A::powf(A::offset(s.ad_value(97), 1.0), (1.0 / p.p39)), (-1.0));

        s.store_scale_ad(54, A::offset(A::scale(s.ad_value(98), p.p41), 1.0), p.p19);

        s.store_mul(55, 54, 35);

        s.store_scale(56, 45, p.p73);

        s.v[115] = if (p.p32 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[115] != 0.0) {
            s.store_div_ad_rhs(51, 51, A::offset(A::powf(A::scale(A::abs(A::voltage(ctx, &nodes, Some(8), None)), 1.0 / (p.p20)), p.p44), 1.0));
        }

        if (!(s.v[115] != 0.0)) {
        }

        s.v[116] = if (p.p31 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[116] != 0.0) {
            s.store_offset(51, 51, p.p13);
        }

        if (s.v[116] != 0.0) {
            s.store_offset(52, 52, p.p67);
        }

        if (s.v[116] != 0.0) {
            s.store_offset(53, 53, p.p15);
        }

        s.v[117] = if (s.v[75] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[117] != 0.0) {
            s.store_scale_ad(57, A::mul(A::mul(s.ad_value(24), s.ad_value(27)), A::sub_from_scalar(1.0, A::exp(A::scale(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(75), s.ad_value(27)))), (1.0 - p.p76))))), 1.0 / ((1.0 - p.p76)));
        }

        if (!(s.v[117] != 0.0)) {
            s.store_mul_ad(57, A::mul(s.ad_value(24), s.ad_value(75)), A::offset(A::div(A::scale(s.ad_value(75), (0.5 * p.p76)), s.ad_value(27)), 1.0));
        }

        s.store_scale_ad(4, A::neg(s.ad_value(25)), p.p24);

        s.store_add(5, 76, 4);

        s.v[118] = if (s.v[5] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[118] != 0.0) {
            s.store_scalar(6, (((((-1.0) - p.p18) * (((1.0 - p.p24)) as f64).ln())) as f64).exp());
        }

        if (s.v[118] != 0.0) {
            s.store_scale_ad(7, A::mul(s.ad_value(25), A::sub_from_scalar(1.0, A::scale(s.ad_value(6), ((1.0 - p.p24) * (1.0 - p.p24))))), 1.0 / ((1.0 - p.p18)));
        }

        if (s.v[118] != 0.0) {
            s.store_mul_ad_lhs(8, A::mul(s.ad_value(5), A::offset(A::div(A::scale(s.ad_value(5), (0.5 * p.p18)), s.ad_value(25)), (1.0 - p.p24))), 6);
        }

        if (!(s.v[118] != 0.0)) {
            s.store_scale_ad(7, A::mul(s.ad_value(25), A::sub_from_scalar(1.0, A::exp(A::scale(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(76), s.ad_value(25)))), (1.0 - p.p18))))), 1.0 / ((1.0 - p.p18)));
        }

        if (!(s.v[118] != 0.0)) {
            s.store_scalar(8, 0.0);
        }

        s.store_mul_ad_rhs(58, 22, A::add(s.ad_value(7), s.ad_value(8)));

        s.store_scale_ad(4, A::neg(s.ad_value(26)), p.p24);

        s.store_add(5, 78, 4);

        s.v[119] = if (s.v[5] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[119] != 0.0) {
            s.store_scalar(6, (((((-1.0) - p.p71) * (((1.0 - p.p24)) as f64).ln())) as f64).exp());
        }

        if (s.v[119] != 0.0) {
            s.store_scale_ad(7, A::mul(s.ad_value(26), A::sub_from_scalar(1.0, A::scale(s.ad_value(6), ((1.0 - p.p24) * (1.0 - p.p24))))), 1.0 / ((1.0 - p.p71)));
        }

        if (s.v[119] != 0.0) {
            s.store_mul_ad_lhs(8, A::mul(s.ad_value(5), A::offset(A::div(A::scale(s.ad_value(5), (0.5 * p.p71)), s.ad_value(26)), (1.0 - p.p24))), 6);
        }

        if (!(s.v[119] != 0.0)) {
            s.store_scale_ad(7, A::mul(s.ad_value(26), A::sub_from_scalar(1.0, A::exp(A::scale(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(78), s.ad_value(26)))), (1.0 - p.p71))))), 1.0 / ((1.0 - p.p71)));
        }

        if (!(s.v[119] != 0.0)) {
            s.store_scalar(8, 0.0);
        }

        s.store_mul_ad_rhs(59, 23, A::add(s.ad_value(7), s.ad_value(8)));

        s.store_scale(60, 59, (1.0 - p.p72));

        s.store_scale_ad(4, A::neg(s.ad_value(26)), p.p24);

        s.store_add(5, 77, 4);

        s.v[120] = if (s.v[5] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[120] != 0.0) {
            s.store_scalar(6, (((((-1.0) - p.p71) * (((1.0 - p.p24)) as f64).ln())) as f64).exp());
        }

        if (s.v[120] != 0.0) {
            s.store_scale_ad(7, A::mul(s.ad_value(26), A::sub_from_scalar(1.0, A::scale(s.ad_value(6), ((1.0 - p.p24) * (1.0 - p.p24))))), 1.0 / ((1.0 - p.p71)));
        }

        if (s.v[120] != 0.0) {
            s.store_mul_ad_lhs(8, A::mul(s.ad_value(5), A::offset(A::div(A::scale(s.ad_value(5), (0.5 * p.p71)), s.ad_value(26)), (1.0 - p.p24))), 6);
        }

        if (!(s.v[120] != 0.0)) {
            s.store_scale_ad(7, A::mul(s.ad_value(26), A::sub_from_scalar(1.0, A::exp(A::scale(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(26)))), (1.0 - p.p71))))), 1.0 / ((1.0 - p.p71)));
        }

        if (!(s.v[120] != 0.0)) {
            s.store_scalar(8, 0.0);
        }

        s.store_mul_ad_rhs(61, 23, A::add(s.ad_value(7), s.ad_value(8)));

        s.store_scale(62, 61, p.p72);

        s.v[121] = if ((p.p68 != 0.0) && (p.p19 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[121] != 0.0) {
            s.store_scale(63, 44, ((((s.v[9] * p.p68) * 3.141592653589793) / 180.0) * p.p19));
        }

        if (!(s.v[121] != 0.0)) {
            s.store_scalar(63, 0.0);
        }

        s.v[122] = if ((p.p30 == 1.0) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };

        s.v[123] = if (((p.p30 == 2.0) && (p.p33 > 0.0)) && (p.p35 > 0.0)) { 1.0 } else { 0.0 };

        s.v[124] = if (p.p30 == (-1.0)) { 1.0 } else { 0.0 };

        s.store_scale(69, 10, (4.0 * 1.3806226e-23));

        s.v[50] = ((p.p12 + (p.p31 * p.p13)) / s.v[3]);

        s.v[48] = ((p.p14 + (p.p31 * p.p15)) / s.v[3]);

        s.v[49] = ((p.p66 + (p.p31 * p.p67)) / s.v[3]);

        s.v[125] = if ((s.v[50] > 0.0) && (s.v[50] >= p.p46)) { 1.0 } else { 0.0 };

        if (s.v[125] != 0.0) {
            s.store_ad(72, &{
                if ((s.v[51] / s.v[3]) >= p.p46) {
                    A::div(s.ad_value(69), A::scale(s.ad_value(51), 1.0 / (s.v[3])))
                } else {
                    A::constant(0.0)
                }
            });
        }

        s.v[126] = if ((s.v[48] > 0.0) && (s.v[48] >= p.p46)) { 1.0 } else { 0.0 };

        if (s.v[126] != 0.0) {
            s.store_ad(73, &{
                if ((s.v[53] / s.v[3]) >= p.p46) {
                    A::div(s.ad_value(69), A::scale(s.ad_value(53), 1.0 / (s.v[3])))
                } else {
                    A::constant(0.0)
                }
            });
        }

        s.v[127] = if ((s.v[49] > 0.0) && (s.v[49] >= p.p46)) { 1.0 } else { 0.0 };

        if (s.v[127] != 0.0) {
            s.store_ad(74, &{
                if ((s.v[52] / s.v[3]) >= p.p46) {
                    A::div(s.ad_value(69), A::scale(s.ad_value(52), 1.0 / (s.v[3])))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (if ((p.p28 > 0.0) && (p.p27 > 0.0)) { 1.0 } else { 0.0 } > 0.0) {
            s.store_scale_ad(71, A::powf(A::abs(s.ad_value(37)), p.p28), p.p27);
        } else {
            s.store_scalar(71, 0.0);
        }

        s.v[70] = (2.0 * 1.6021918e-19);

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
        s.store_offset_ad(12, A::offset(A::voltage(ctx, &nodes, Some(3), None), ctx.temperature()), p.p45);

        if ((1026.85 + 273.15) < (if (s.v[12] > ((-100.0) + 273.15)) { s.v[12] } else { ((-100.0) + 273.15) })) {
            s.store_scalar(10, (1026.85 + 273.15));
        } else {
            s.store_ad(10, &{
                if (s.v[12] > ((-100.0) + 273.15)) {
                    s.ad_value(12)
                } else {
                    A::constant(((-100.0) + 273.15))
                }
            });
        }

        s.v[3] = (p.p43 * p.p42);

        s.v[11] = (p.p25 + 273.15);

        s.store_scale(15, 10, 8.6170869e-5);

        s.store_scale(13, 10, 1.0 / (s.v[11]));

        s.store_ln(14, 13);

        s.store_exp_ad(18, A::scale(s.ad_value(14), p.p77));

        s.v[64] = (if (p.p53 > 0.0) { (1.0 / p.p53) } else { 0.0 });

        s.v[65] = (if (p.p62 > 0.0) { (1.0 / p.p62) } else { 0.0 });

        s.v[66] = (if (p.p54 > 0.0) { (1.0 / p.p54) } else { 0.0 });

        s.v[67] = (if (p.p63 > 0.0) { (1.0 / p.p63) } else { 0.0 });

        s.store_add_ad(68, A::scale(s.ad_value(14), p.p22), A::div(A::scale(A::offset(s.ad_value(13), (-1.0)), p.p21), s.ad_value(15)));

        s.store_scale(92, 14, p.p23);

        s.store_scaled_exp(19, 68, p.p0);

        s.store_scaled_exp(93, 92, p.p2);

        s.store_div_ad_lhs(20, A::scale(A::exp(A::scale(s.ad_value(68), 1.0 / (p.p59))), p.p58), 18);

        s.store_div_ad_lhs(21, A::scale(A::exp(A::scale(s.ad_value(68), 1.0 / (p.p65))), p.p64), 18);

        s.store_scale_ad(28, A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), p.p7), 1.0), p.p47);

        s.store_scale_ad(30, A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), p.p6), 1.0), p.p5);

        s.store_scale_ad(31, A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), p.p10), 1.0), p.p9);

        s.store_scale_ad(29, A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), p.p55), 1.0), p.p56);

        s.v[32] = p.p16;

        s.v[33] = p.p69;

        s.v[34] = p.p74;

        s.v[81] = (s.v[11] / 300.15);

        s.store_scale(82, 10, 0.003331667499583542);

        s.store_sub_from_scalar_ad(83, 1.16, A::div(A::mul(A::scale(s.ad_value(10), 0.000702), s.ad_value(10)), A::offset(s.ad_value(10), 1108.0)));

        s.store_offset_ad(84, A::div(A::neg(s.ad_value(83)), A::scale(s.ad_value(10), (2.0 * 1.3806226e-23))), (1.1150877 / (1.3806226e-23 * (300.15 + 300.15))));

        s.store_mul_ad(85, A::neg(A::scale(s.ad_value(15), 2.0)), A::add(A::scale(A::ln(s.ad_value(82)), 1.5), A::scale(s.ad_value(84), 1.6021918e-19)));

        s.store_scale_ad(86, A::sub_from_scalar(p.p17, s.ad_value(85)), 1.0 / (s.v[81]));

        s.store_div_ad_lhs(87, A::sub_from_scalar(p.p17, s.ad_value(86)), 86);

        s.store_div_from_scalar_ad(89, s.v[32], A::offset(A::scale(A::sub_from_scalar((0.0004 * (s.v[11] - 300.15)), s.ad_value(87)), p.p18), 1.0));

        s.store_add_ad_lhs(25, A::mul(s.ad_value(82), s.ad_value(86)), 85);

        s.store_div_ad_lhs(88, A::sub(s.ad_value(25), s.ad_value(86)), 86);

        s.store_mul_ad_rhs(22, 89, A::offset(A::scale(A::sub(A::scale(A::offset(s.ad_value(10), (-300.15)), 0.0004), s.ad_value(88)), p.p18), 1.0));

        s.v[81] = (s.v[11] / 300.15);

        s.store_scale(82, 10, 0.003331667499583542);

        s.store_sub_from_scalar_ad(83, 1.16, A::div(A::mul(A::scale(s.ad_value(10), 0.000702), s.ad_value(10)), A::offset(s.ad_value(10), 1108.0)));

        s.store_offset_ad(84, A::div(A::neg(s.ad_value(83)), A::scale(s.ad_value(10), (2.0 * 1.3806226e-23))), (1.1150877 / (1.3806226e-23 * (300.15 + 300.15))));

        s.store_mul_ad(85, A::neg(A::scale(s.ad_value(15), 2.0)), A::add(A::scale(A::ln(s.ad_value(82)), 1.5), A::scale(s.ad_value(84), 1.6021918e-19)));

        s.store_scale_ad(86, A::sub_from_scalar(p.p70, s.ad_value(85)), 1.0 / (s.v[81]));

        s.store_div_ad_lhs(87, A::sub_from_scalar(p.p70, s.ad_value(86)), 86);

        s.store_div_from_scalar_ad(89, s.v[33], A::offset(A::scale(A::sub_from_scalar((0.0004 * (s.v[11] - 300.15)), s.ad_value(87)), p.p71), 1.0));

        s.store_add_ad_lhs(26, A::mul(s.ad_value(82), s.ad_value(86)), 85);

        s.store_div_ad_lhs(88, A::sub(s.ad_value(26), s.ad_value(86)), 86);

        s.store_mul_ad_rhs(23, 89, A::offset(A::scale(A::sub(A::scale(A::offset(s.ad_value(10), (-300.15)), 0.0004), s.ad_value(88)), p.p71), 1.0));

        s.v[81] = (s.v[11] / 300.15);

        s.store_scale(82, 10, 0.003331667499583542);

        s.store_sub_from_scalar_ad(83, 1.16, A::div(A::mul(A::scale(s.ad_value(10), 0.000702), s.ad_value(10)), A::offset(s.ad_value(10), 1108.0)));

        s.store_offset_ad(84, A::div(A::neg(s.ad_value(83)), A::scale(s.ad_value(10), (2.0 * 1.3806226e-23))), (1.1150877 / (1.3806226e-23 * (300.15 + 300.15))));

        s.store_mul_ad(85, A::neg(A::scale(s.ad_value(15), 2.0)), A::add(A::scale(A::ln(s.ad_value(82)), 1.5), A::scale(s.ad_value(84), 1.6021918e-19)));

        s.store_scale_ad(86, A::sub_from_scalar(p.p75, s.ad_value(85)), 1.0 / (s.v[81]));

        s.store_div_ad_lhs(87, A::sub_from_scalar(p.p75, s.ad_value(86)), 86);

        s.store_div_from_scalar_ad(89, s.v[34], A::offset(A::scale(A::sub_from_scalar((0.0004 * (s.v[11] - 300.15)), s.ad_value(87)), p.p76), 1.0));

        s.store_add_ad_lhs(27, A::mul(s.ad_value(82), s.ad_value(86)), 85);

        s.store_div_ad_lhs(88, A::sub(s.ad_value(27), s.ad_value(86)), 86);

        s.store_mul_ad_rhs(24, 89, A::offset(A::scale(A::sub(A::scale(A::offset(s.ad_value(10), (-300.15)), 0.0004), s.ad_value(88)), p.p76), 1.0));

        s.v[9] = p.p29;

        s.store_ad(75, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(4)), s.v[9]));

        s.store_ad(76, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(6)), s.v[9]));

        s.store_ad(77, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(4)), s.v[9]));

        s.store_ad(78, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(4)), s.v[9]));

        s.v[105] = if (s.v[19] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[105] != 0.0) {
            s.store_div_ad_rhs(0, 76, A::scale(s.ad_value(15), p.p1));
        }

        if (s.v[105] != 0.0) {
            s.store_div_ad(90, A::sub(A::neg(s.ad_value(76)), s.ad_value(30)), A::scale(s.ad_value(15), p.p11));
        }

        if (s.v[105] != 0.0) {
            s.store_div_ad(91, A::neg(s.ad_value(30)), A::scale(s.ad_value(15), p.p11));
        }

        s.v[106] = if (s.v[0] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[105] != 0.0) && (s.v[106] != 0.0)) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
        }

        if ((s.v[105] != 0.0) && (s.v[106] != 0.0)) {
            s.store_scalar(0, 80.0);
        }

        if ((s.v[105] != 0.0) && (!(s.v[106] != 0.0))) {
            s.store_scalar(1, 1.0);
        }

        if (s.v[105] != 0.0) {
            s.store_mul_ad_rhs(1, 1, A::exp(s.ad_value(0)));
        }

        if (s.v[105] != 0.0) {
            let assign800_ad_e1032: A = {
                if ((!(s.v[90] >= 37.0)) && (!(s.v[90] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(90)), 1.0))
                } else {
                    {
                        if ((!(s.v[90] >= 37.0)) && (s.v[90] <= (-37.0))) {
                            A::exp(s.ad_value(90))
                        } else {
                            {
                                if (s.v[90] >= 37.0) {
                                    s.ad_value(90)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign800_ad_e1066: A = {
                if ((!(s.v[91] >= 37.0)) && (!(s.v[91] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(91)), 1.0))
                } else {
                    {
                        if ((!(s.v[91] >= 37.0)) && (s.v[91] <= (-37.0))) {
                            A::exp(s.ad_value(91))
                        } else {
                            {
                                if (s.v[91] >= 37.0) {
                                    s.ad_value(91)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(2, assign800_ad_e1032, assign800_ad_e1066);
        }

        if (s.v[105] != 0.0) {
            s.store_sub_ad(35, A::mul(s.ad_value(19), A::offset(s.ad_value(1), (-1.0))), A::div(A::mul(s.ad_value(28), s.ad_value(2)), A::offset(A::scale(A::pow(A::abs(s.ad_value(76)), s.ad_value(31)), p.p8), 1.0)));
        }

        if (!(s.v[105] != 0.0)) {
            s.store_scalar(35, 0.0);
        }

        s.v[107] = if (s.v[93] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[107] != 0.0) {
            s.store_max_with_scalar_ad(101, A::sub_from_scalar(p.p4, s.ad_value(76)), 0.001);
        }

        if (s.v[107] != 0.0) {
            s.store_div_ad(0, A::scale(s.ad_value(76), ((-1.0) * p.p4)), A::mul(A::scale(s.ad_value(15), p.p3), s.ad_value(101)));
        }

        s.v[108] = if (s.v[0] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[107] != 0.0) && (s.v[108] != 0.0)) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
        }

        if ((s.v[107] != 0.0) && (s.v[108] != 0.0)) {
            s.store_scalar(0, 80.0);
        }

        if ((s.v[107] != 0.0) && (!(s.v[108] != 0.0))) {
            s.store_scalar(1, 1.0);
        }

        if (s.v[107] != 0.0) {
            s.store_mul_ad_rhs(1, 1, A::exp(s.ad_value(0)));
        }

        s.v[109] = if (s.v[20] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[109] != 0.0) {
            s.store_div_ad_rhs(0, 76, A::scale(s.ad_value(15), p.p59));
        }

        if (s.v[109] != 0.0) {
            s.store_div_ad(90, A::sub(A::neg(s.ad_value(76)), s.ad_value(30)), A::scale(s.ad_value(15), p.p57));
        }

        if (s.v[109] != 0.0) {
            s.store_div_ad(91, A::neg(s.ad_value(30)), A::scale(s.ad_value(15), p.p57));
        }

        s.v[110] = if (s.v[0] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[109] != 0.0) && (s.v[110] != 0.0)) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
        }

        if ((s.v[109] != 0.0) && (s.v[110] != 0.0)) {
            s.store_scalar(0, 80.0);
        }

        if ((s.v[109] != 0.0) && (!(s.v[110] != 0.0))) {
            s.store_scalar(1, 1.0);
        }

        if (s.v[109] != 0.0) {
            s.store_mul_ad_rhs(1, 1, A::exp(s.ad_value(0)));
        }

        if (s.v[109] != 0.0) {
            let assign1020_ad_e1266: A = {
                if ((!(s.v[90] >= 37.0)) && (!(s.v[90] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(90)), 1.0))
                } else {
                    {
                        if ((!(s.v[90] >= 37.0)) && (s.v[90] <= (-37.0))) {
                            A::exp(s.ad_value(90))
                        } else {
                            {
                                if (s.v[90] >= 37.0) {
                                    s.ad_value(90)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign1020_ad_e1300: A = {
                if ((!(s.v[91] >= 37.0)) && (!(s.v[91] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(91)), 1.0))
                } else {
                    {
                        if ((!(s.v[91] >= 37.0)) && (s.v[91] <= (-37.0))) {
                            A::exp(s.ad_value(91))
                        } else {
                            {
                                if (s.v[91] >= 37.0) {
                                    s.ad_value(91)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(2, assign1020_ad_e1266, assign1020_ad_e1300);
        }

        s.v[111] = if (s.v[19] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[111] != 0.0) {
            s.store_div_ad_rhs(0, 77, A::scale(s.ad_value(15), p.p61));
        }

        if (s.v[111] != 0.0) {
            s.store_div_ad(90, A::sub(A::neg(s.ad_value(77)), s.ad_value(30)), A::scale(s.ad_value(15), p.p57));
        }

        if (s.v[111] != 0.0) {
            s.store_div_ad(91, A::neg(s.ad_value(30)), A::scale(s.ad_value(15), p.p57));
        }

        s.v[112] = if (s.v[0] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[111] != 0.0) && (s.v[112] != 0.0)) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
        }

        if ((s.v[111] != 0.0) && (s.v[112] != 0.0)) {
            s.store_scalar(0, 80.0);
        }

        if ((s.v[111] != 0.0) && (!(s.v[112] != 0.0))) {
            s.store_scalar(1, 1.0);
        }

        if (s.v[111] != 0.0) {
            s.store_mul_ad_rhs(1, 1, A::exp(s.ad_value(0)));
        }

        if (s.v[111] != 0.0) {
            let assign1140_ad_e1428: A = {
                if ((!(s.v[90] >= 37.0)) && (!(s.v[90] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(90)), 1.0))
                } else {
                    {
                        if ((!(s.v[90] >= 37.0)) && (s.v[90] <= (-37.0))) {
                            A::exp(s.ad_value(90))
                        } else {
                            {
                                if (s.v[90] >= 37.0) {
                                    s.ad_value(90)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign1140_ad_e1462: A = {
                if ((!(s.v[91] >= 37.0)) && (!(s.v[91] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(91)), 1.0))
                } else {
                    {
                        if ((!(s.v[91] >= 37.0)) && (s.v[91] <= (-37.0))) {
                            A::exp(s.ad_value(91))
                        } else {
                            {
                                if (s.v[91] >= 37.0) {
                                    s.ad_value(91)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(2, assign1140_ad_e1428, assign1140_ad_e1462);
        }

        if (s.v[111] != 0.0) {
            s.store_sub_ad(38, A::mul(s.ad_value(19), A::offset(s.ad_value(1), (-1.0))), A::div(A::mul(s.ad_value(29), s.ad_value(2)), A::offset(A::scale(A::pow(A::abs(s.ad_value(77)), s.ad_value(31)), p.p8), 1.0)));
        }

        if (!(s.v[111] != 0.0)) {
            s.store_scalar(38, 0.0);
        }

        s.v[113] = if (s.v[21] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[113] != 0.0) {
            s.store_div_ad_rhs(0, 77, A::scale(s.ad_value(15), p.p65));
        }

        if (s.v[113] != 0.0) {
            s.store_div_ad(90, A::sub(A::neg(s.ad_value(77)), s.ad_value(30)), A::scale(s.ad_value(15), p.p57));
        }

        if (s.v[113] != 0.0) {
            s.store_div_ad(91, A::neg(s.ad_value(30)), A::scale(s.ad_value(15), p.p57));
        }

        s.v[114] = if (s.v[0] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[113] != 0.0) && (s.v[114] != 0.0)) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
        }

        if ((s.v[113] != 0.0) && (s.v[114] != 0.0)) {
            s.store_scalar(0, 80.0);
        }

        if ((s.v[113] != 0.0) && (!(s.v[114] != 0.0))) {
            s.store_scalar(1, 1.0);
        }

        if (s.v[113] != 0.0) {
            s.store_mul_ad_rhs(1, 1, A::exp(s.ad_value(0)));
        }

        if (s.v[113] != 0.0) {
            let assign1260_ad_e1590: A = {
                if ((!(s.v[90] >= 37.0)) && (!(s.v[90] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(90)), 1.0))
                } else {
                    {
                        if ((!(s.v[90] >= 37.0)) && (s.v[90] <= (-37.0))) {
                            A::exp(s.ad_value(90))
                        } else {
                            {
                                if (s.v[90] >= 37.0) {
                                    s.ad_value(90)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign1260_ad_e1624: A = {
                if ((!(s.v[91] >= 37.0)) && (!(s.v[91] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(91)), 1.0))
                } else {
                    {
                        if ((!(s.v[91] >= 37.0)) && (s.v[91] <= (-37.0))) {
                            A::exp(s.ad_value(91))
                        } else {
                            {
                                if (s.v[91] >= 37.0) {
                                    s.ad_value(91)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(2, assign1260_ad_e1590, assign1260_ad_e1624);
        }

        s.store_scale_ad(66, A::offset(A::scale(s.ad_value(77), p.p81), 1.0), s.v[66]);

        s.store_add_ad(42, A::mul(s.ad_value(35), s.ad_value(66)), A::scale(s.ad_value(38), s.v[67]));

        s.store_sub_ad(41, A::sub_from_scalar(1.0, A::scale(s.ad_value(76), s.v[65])), A::scale(s.ad_value(77), s.v[64]));

        s.store_offset_ad(96, A::powf(A::abs(A::offset(A::scale(s.ad_value(42), 4.0), 1.0)), p.p82), 1.0);

        s.store_div_ad_lhs(43, A::scale(s.ad_value(41), 2.0), 96);

        s.store_mul(45, 38, 43);

        s.store_mul(44, 35, 43);

        s.store_powf_ad(97, A::abs(A::scale(A::voltage(ctx, &nodes, Some(1), Some(2)), 1.0 / (p.p40))), p.p39);

        s.store_offset_ad(98, A::powf(A::offset(s.ad_value(97), 1.0), (1.0 / p.p39)), (-1.0));

        s.store_scale_ad(54, A::offset(A::scale(s.ad_value(98), p.p41), 1.0), p.p19);

        s.store_mul(55, 54, 35);

        s.store_scale(56, 45, p.p73);

        s.v[115] = if (p.p32 == 1.0) { 1.0 } else { 0.0 };

        s.v[117] = if (s.v[75] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[117] != 0.0) {
            s.store_scale_ad(57, A::mul(A::mul(s.ad_value(24), s.ad_value(27)), A::sub_from_scalar(1.0, A::exp(A::scale(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(75), s.ad_value(27)))), (1.0 - p.p76))))), 1.0 / ((1.0 - p.p76)));
        }

        if (!(s.v[117] != 0.0)) {
            s.store_mul_ad(57, A::mul(s.ad_value(24), s.ad_value(75)), A::offset(A::div(A::scale(s.ad_value(75), (0.5 * p.p76)), s.ad_value(27)), 1.0));
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
        s.store_scale_ad(4, A::neg(s.ad_value(25)), p.p24);

        s.store_add(5, 76, 4);

        s.v[118] = if (s.v[5] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[118] != 0.0) {
            s.store_scalar(6, (((((-1.0) - p.p18) * (((1.0 - p.p24)) as f64).ln())) as f64).exp());
        }

        if (s.v[118] != 0.0) {
            s.store_scale_ad(7, A::mul(s.ad_value(25), A::sub_from_scalar(1.0, A::scale(s.ad_value(6), ((1.0 - p.p24) * (1.0 - p.p24))))), 1.0 / ((1.0 - p.p18)));
        }

        if (s.v[118] != 0.0) {
            s.store_mul_ad_lhs(8, A::mul(s.ad_value(5), A::offset(A::div(A::scale(s.ad_value(5), (0.5 * p.p18)), s.ad_value(25)), (1.0 - p.p24))), 6);
        }

        if (!(s.v[118] != 0.0)) {
            s.store_scale_ad(7, A::mul(s.ad_value(25), A::sub_from_scalar(1.0, A::exp(A::scale(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(76), s.ad_value(25)))), (1.0 - p.p18))))), 1.0 / ((1.0 - p.p18)));
        }

        if (!(s.v[118] != 0.0)) {
            s.store_scalar(8, 0.0);
        }

        s.store_mul_ad_rhs(58, 22, A::add(s.ad_value(7), s.ad_value(8)));

        s.store_scale_ad(4, A::neg(s.ad_value(26)), p.p24);

        s.store_add(5, 78, 4);

        s.v[119] = if (s.v[5] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[119] != 0.0) {
            s.store_scalar(6, (((((-1.0) - p.p71) * (((1.0 - p.p24)) as f64).ln())) as f64).exp());
        }

        if (s.v[119] != 0.0) {
            s.store_scale_ad(7, A::mul(s.ad_value(26), A::sub_from_scalar(1.0, A::scale(s.ad_value(6), ((1.0 - p.p24) * (1.0 - p.p24))))), 1.0 / ((1.0 - p.p71)));
        }

        if (s.v[119] != 0.0) {
            s.store_mul_ad_lhs(8, A::mul(s.ad_value(5), A::offset(A::div(A::scale(s.ad_value(5), (0.5 * p.p71)), s.ad_value(26)), (1.0 - p.p24))), 6);
        }

        if (!(s.v[119] != 0.0)) {
            s.store_scale_ad(7, A::mul(s.ad_value(26), A::sub_from_scalar(1.0, A::exp(A::scale(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(78), s.ad_value(26)))), (1.0 - p.p71))))), 1.0 / ((1.0 - p.p71)));
        }

        if (!(s.v[119] != 0.0)) {
            s.store_scalar(8, 0.0);
        }

        s.store_mul_ad_rhs(59, 23, A::add(s.ad_value(7), s.ad_value(8)));

        s.store_scale(60, 59, (1.0 - p.p72));

        s.store_scale_ad(4, A::neg(s.ad_value(26)), p.p24);

        s.store_add(5, 77, 4);

        s.v[120] = if (s.v[5] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[120] != 0.0) {
            s.store_scalar(6, (((((-1.0) - p.p71) * (((1.0 - p.p24)) as f64).ln())) as f64).exp());
        }

        if (s.v[120] != 0.0) {
            s.store_scale_ad(7, A::mul(s.ad_value(26), A::sub_from_scalar(1.0, A::scale(s.ad_value(6), ((1.0 - p.p24) * (1.0 - p.p24))))), 1.0 / ((1.0 - p.p71)));
        }

        if (s.v[120] != 0.0) {
            s.store_mul_ad_lhs(8, A::mul(s.ad_value(5), A::offset(A::div(A::scale(s.ad_value(5), (0.5 * p.p71)), s.ad_value(26)), (1.0 - p.p24))), 6);
        }

        if (!(s.v[120] != 0.0)) {
            s.store_scale_ad(7, A::mul(s.ad_value(26), A::sub_from_scalar(1.0, A::exp(A::scale(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(26)))), (1.0 - p.p71))))), 1.0 / ((1.0 - p.p71)));
        }

        if (!(s.v[120] != 0.0)) {
            s.store_scalar(8, 0.0);
        }

        s.store_mul_ad_rhs(61, 23, A::add(s.ad_value(7), s.ad_value(8)));

        s.store_scale(62, 61, p.p72);

        s.v[121] = if ((p.p68 != 0.0) && (p.p19 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[121] != 0.0) {
            s.store_scale(63, 44, ((((s.v[9] * p.p68) * 3.141592653589793) / 180.0) * p.p19));
        }

        if (!(s.v[121] != 0.0)) {
            s.store_scalar(63, 0.0);
        }

        s.v[122] = if ((p.p30 == 1.0) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };

        s.v[123] = if (((p.p30 == 2.0) && (p.p33 > 0.0)) && (p.p35 > 0.0)) { 1.0 } else { 0.0 };

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
        let nv9 = ctx.node_voltage(nodes[9]);
        let eq0_e89: f64 = (s.v[76] - (nv9 - 0.0));
        let eq0_e89_d_n9: f64 = (s.dn[76][9] - 1.0);
        let eq0_e90: f64 = (-eq0_e89);
        let eq0_e90_d_n0: f64 = (-s.dn[76][0]);
        let eq0_e90_d_n1: f64 = (-s.dn[76][1]);
        let eq0_e90_d_n2: f64 = (-s.dn[76][2]);
        let eq0_e90_d_n3: f64 = (-s.dn[76][3]);
        let eq0_e90_d_n4: f64 = (-s.dn[76][4]);
        let eq0_e90_d_n5: f64 = (-s.dn[76][5]);
        let eq0_e90_d_n6: f64 = (-s.dn[76][6]);
        let eq0_e90_d_n7: f64 = (-s.dn[76][7]);
        let eq0_e90_d_n8: f64 = (-s.dn[76][8]);
        let eq0_e90_d_n9: f64 = (-eq0_e89_d_n9);
        let eq0_e90_d_b0: f64 = (-s.db[76][0]);
        let eq0_e90_d_b1: f64 = (-s.db[76][1]);
        let eq0_e90_d_b2: f64 = (-s.db[76][2]);
        let eq0_e90_d_b3: f64 = (-s.db[76][3]);
        let eq0_e90_d_b4: f64 = (-s.db[76][4]);
        let eq0_e90_d_b5: f64 = (-s.db[76][5]);
        let eq0_e90_d_b6: f64 = (-s.db[76][6]);
        let eq0_e90_d_b7: f64 = (-s.db[76][7]);
        let eq0_e92: f64 = eq0_e90;
        let eq0_value: f64 = eq0_e92;
        let eq0_node_derivatives: [f64; 10] = [eq0_e90_d_n0, eq0_e90_d_n1, eq0_e90_d_n2, eq0_e90_d_n3, eq0_e90_d_n4, eq0_e90_d_n5, eq0_e90_d_n6, eq0_e90_d_n7, eq0_e90_d_n8, eq0_e90_d_n9];
        let eq0_branch_derivatives: [f64; 8] = [eq0_e90_d_b0, eq0_e90_d_b1, eq0_e90_d_b2, eq0_e90_d_b3, eq0_e90_d_b4, eq0_e90_d_b5, eq0_e90_d_b6, eq0_e90_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            None,
            self.multiplicity * (eq0_value),
            &nodes,
            &eq0_node_derivatives,
            &branches,
            &eq0_branch_derivatives,
            self.multiplicity,
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
        let nv9 = ctx.node_voltage(nodes[9]);
        let eq1_e95: f64 = ((nv9 - 0.0) * 1e-6);
        let eq1_e95_d_n9: f64 = 1e-6;
        let eq1_value: f64 = eq1_e95;
        stamper.stamp_current(
            Some(nodes[9]),
            None,
            self.multiplicity * (eq1_value),
            &[
                GeneratedDerivative::node(nodes[9], self.multiplicity * eq1_e95_d_n9),
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
        let nv9 = ctx.node_voltage(nodes[9]);
        let eq2_e98: f64 = self.eval_ddt(0, (nv9 - 0.0));
        let eq2_e98_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq2_e98_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq2_e98_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq2_e98_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq2_e98_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq2_e98_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq2_e98_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq2_e98_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq2_e98_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq2_e98_d_n9: f64 = self.ddt_jacobian(1.0);
        let eq2_e98_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq2_e98_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq2_e98_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq2_e98_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq2_e98_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq2_e98_d_b5: f64 = self.ddt_jacobian(0.0);
        let eq2_e98_d_b6: f64 = self.ddt_jacobian(0.0);
        let eq2_e98_d_b7: f64 = self.ddt_jacobian(0.0);
        let eq2_e99: f64 = (p.p83 * eq2_e98);
        let eq2_e99_d_n0: f64 = (p.p83 * eq2_e98_d_n0);
        let eq2_e99_d_n1: f64 = (p.p83 * eq2_e98_d_n1);
        let eq2_e99_d_n2: f64 = (p.p83 * eq2_e98_d_n2);
        let eq2_e99_d_n3: f64 = (p.p83 * eq2_e98_d_n3);
        let eq2_e99_d_n4: f64 = (p.p83 * eq2_e98_d_n4);
        let eq2_e99_d_n5: f64 = (p.p83 * eq2_e98_d_n5);
        let eq2_e99_d_n6: f64 = (p.p83 * eq2_e98_d_n6);
        let eq2_e99_d_n7: f64 = (p.p83 * eq2_e98_d_n7);
        let eq2_e99_d_n8: f64 = (p.p83 * eq2_e98_d_n8);
        let eq2_e99_d_n9: f64 = (p.p83 * eq2_e98_d_n9);
        let eq2_e99_d_b0: f64 = (p.p83 * eq2_e98_d_b0);
        let eq2_e99_d_b1: f64 = (p.p83 * eq2_e98_d_b1);
        let eq2_e99_d_b2: f64 = (p.p83 * eq2_e98_d_b2);
        let eq2_e99_d_b3: f64 = (p.p83 * eq2_e98_d_b3);
        let eq2_e99_d_b4: f64 = (p.p83 * eq2_e98_d_b4);
        let eq2_e99_d_b5: f64 = (p.p83 * eq2_e98_d_b5);
        let eq2_e99_d_b6: f64 = (p.p83 * eq2_e98_d_b6);
        let eq2_e99_d_b7: f64 = (p.p83 * eq2_e98_d_b7);
        let eq2_value: f64 = eq2_e99;
        let eq2_node_derivatives: [f64; 10] = [eq2_e99_d_n0, eq2_e99_d_n1, eq2_e99_d_n2, eq2_e99_d_n3, eq2_e99_d_n4, eq2_e99_d_n5, eq2_e99_d_n6, eq2_e99_d_n7, eq2_e99_d_n8, eq2_e99_d_n9];
        let eq2_branch_derivatives: [f64; 8] = [eq2_e99_d_b0, eq2_e99_d_b1, eq2_e99_d_b2, eq2_e99_d_b3, eq2_e99_d_b4, eq2_e99_d_b5, eq2_e99_d_b6, eq2_e99_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            None,
            self.multiplicity * (eq2_value),
            &nodes,
            &eq2_node_derivatives,
            &branches,
            &eq2_branch_derivatives,
            self.multiplicity,
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
        let (eq3_e108, eq3_e108_d_n0, eq3_e108_d_n1, eq3_e108_d_n2, eq3_e108_d_n3, eq3_e108_d_n4, eq3_e108_d_n5, eq3_e108_d_n6, eq3_e108_d_n7, eq3_e108_d_n8, eq3_e108_d_n9, eq3_e108_d_b0, eq3_e108_d_b1, eq3_e108_d_b2, eq3_e108_d_b3, eq3_e108_d_b4, eq3_e108_d_b5, eq3_e108_d_b6, eq3_e108_d_b7,) = {
    if (s.v[115] != 0.0) {
        let eq3_e103: f64 = (s.v[35] / s.v[16]);
        let eq3_e103_d_n0: f64 = (((s.dn[35][0] * s.v[16]) - (s.v[35] * s.dn[16][0])) / (s.v[16] * s.v[16]));
        let eq3_e103_d_n1: f64 = (((s.dn[35][1] * s.v[16]) - (s.v[35] * s.dn[16][1])) / (s.v[16] * s.v[16]));
        let eq3_e103_d_n2: f64 = (((s.dn[35][2] * s.v[16]) - (s.v[35] * s.dn[16][2])) / (s.v[16] * s.v[16]));
        let eq3_e103_d_n3: f64 = (((s.dn[35][3] * s.v[16]) - (s.v[35] * s.dn[16][3])) / (s.v[16] * s.v[16]));
        let eq3_e103_d_n4: f64 = (((s.dn[35][4] * s.v[16]) - (s.v[35] * s.dn[16][4])) / (s.v[16] * s.v[16]));
        let eq3_e103_d_n5: f64 = (((s.dn[35][5] * s.v[16]) - (s.v[35] * s.dn[16][5])) / (s.v[16] * s.v[16]));
        let eq3_e103_d_n6: f64 = (((s.dn[35][6] * s.v[16]) - (s.v[35] * s.dn[16][6])) / (s.v[16] * s.v[16]));
        let eq3_e103_d_n7: f64 = (((s.dn[35][7] * s.v[16]) - (s.v[35] * s.dn[16][7])) / (s.v[16] * s.v[16]));
        let eq3_e103_d_n8: f64 = (((s.dn[35][8] * s.v[16]) - (s.v[35] * s.dn[16][8])) / (s.v[16] * s.v[16]));
        let eq3_e103_d_n9: f64 = (((s.dn[35][9] * s.v[16]) - (s.v[35] * s.dn[16][9])) / (s.v[16] * s.v[16]));
        let eq3_e103_d_b0: f64 = (((s.db[35][0] * s.v[16]) - (s.v[35] * s.db[16][0])) / (s.v[16] * s.v[16]));
        let eq3_e103_d_b1: f64 = (((s.db[35][1] * s.v[16]) - (s.v[35] * s.db[16][1])) / (s.v[16] * s.v[16]));
        let eq3_e103_d_b2: f64 = (((s.db[35][2] * s.v[16]) - (s.v[35] * s.db[16][2])) / (s.v[16] * s.v[16]));
        let eq3_e103_d_b3: f64 = (((s.db[35][3] * s.v[16]) - (s.v[35] * s.db[16][3])) / (s.v[16] * s.v[16]));
        let eq3_e103_d_b4: f64 = (((s.db[35][4] * s.v[16]) - (s.v[35] * s.db[16][4])) / (s.v[16] * s.v[16]));
        let eq3_e103_d_b5: f64 = (((s.db[35][5] * s.v[16]) - (s.v[35] * s.db[16][5])) / (s.v[16] * s.v[16]));
        let eq3_e103_d_b6: f64 = (((s.db[35][6] * s.v[16]) - (s.v[35] * s.db[16][6])) / (s.v[16] * s.v[16]));
        let eq3_e103_d_b7: f64 = (((s.db[35][7] * s.v[16]) - (s.v[35] * s.db[16][7])) / (s.v[16] * s.v[16]));
        let eq3_e104: f64 = (-eq3_e103);
        let eq3_e104_d_n0: f64 = (-eq3_e103_d_n0);
        let eq3_e104_d_n1: f64 = (-eq3_e103_d_n1);
        let eq3_e104_d_n2: f64 = (-eq3_e103_d_n2);
        let eq3_e104_d_n3: f64 = (-eq3_e103_d_n3);
        let eq3_e104_d_n4: f64 = (-eq3_e103_d_n4);
        let eq3_e104_d_n5: f64 = (-eq3_e103_d_n5);
        let eq3_e104_d_n6: f64 = (-eq3_e103_d_n6);
        let eq3_e104_d_n7: f64 = (-eq3_e103_d_n7);
        let eq3_e104_d_n8: f64 = (-eq3_e103_d_n8);
        let eq3_e104_d_n9: f64 = (-eq3_e103_d_n9);
        let eq3_e104_d_b0: f64 = (-eq3_e103_d_b0);
        let eq3_e104_d_b1: f64 = (-eq3_e103_d_b1);
        let eq3_e104_d_b2: f64 = (-eq3_e103_d_b2);
        let eq3_e104_d_b3: f64 = (-eq3_e103_d_b3);
        let eq3_e104_d_b4: f64 = (-eq3_e103_d_b4);
        let eq3_e104_d_b5: f64 = (-eq3_e103_d_b5);
        let eq3_e104_d_b6: f64 = (-eq3_e103_d_b6);
        let eq3_e104_d_b7: f64 = (-eq3_e103_d_b7);
        let eq3_e106: f64 = (eq3_e104 * s.v[54]);
        let eq3_e106_d_n0: f64 = ((eq3_e104_d_n0 * s.v[54]) + (eq3_e104 * s.dn[54][0]));
        let eq3_e106_d_n1: f64 = ((eq3_e104_d_n1 * s.v[54]) + (eq3_e104 * s.dn[54][1]));
        let eq3_e106_d_n2: f64 = ((eq3_e104_d_n2 * s.v[54]) + (eq3_e104 * s.dn[54][2]));
        let eq3_e106_d_n3: f64 = ((eq3_e104_d_n3 * s.v[54]) + (eq3_e104 * s.dn[54][3]));
        let eq3_e106_d_n4: f64 = ((eq3_e104_d_n4 * s.v[54]) + (eq3_e104 * s.dn[54][4]));
        let eq3_e106_d_n5: f64 = ((eq3_e104_d_n5 * s.v[54]) + (eq3_e104 * s.dn[54][5]));
        let eq3_e106_d_n6: f64 = ((eq3_e104_d_n6 * s.v[54]) + (eq3_e104 * s.dn[54][6]));
        let eq3_e106_d_n7: f64 = ((eq3_e104_d_n7 * s.v[54]) + (eq3_e104 * s.dn[54][7]));
        let eq3_e106_d_n8: f64 = ((eq3_e104_d_n8 * s.v[54]) + (eq3_e104 * s.dn[54][8]));
        let eq3_e106_d_n9: f64 = ((eq3_e104_d_n9 * s.v[54]) + (eq3_e104 * s.dn[54][9]));
        let eq3_e106_d_b0: f64 = ((eq3_e104_d_b0 * s.v[54]) + (eq3_e104 * s.db[54][0]));
        let eq3_e106_d_b1: f64 = ((eq3_e104_d_b1 * s.v[54]) + (eq3_e104 * s.db[54][1]));
        let eq3_e106_d_b2: f64 = ((eq3_e104_d_b2 * s.v[54]) + (eq3_e104 * s.db[54][2]));
        let eq3_e106_d_b3: f64 = ((eq3_e104_d_b3 * s.v[54]) + (eq3_e104 * s.db[54][3]));
        let eq3_e106_d_b4: f64 = ((eq3_e104_d_b4 * s.v[54]) + (eq3_e104 * s.db[54][4]));
        let eq3_e106_d_b5: f64 = ((eq3_e104_d_b5 * s.v[54]) + (eq3_e104 * s.db[54][5]));
        let eq3_e106_d_b6: f64 = ((eq3_e104_d_b6 * s.v[54]) + (eq3_e104 * s.db[54][6]));
        let eq3_e106_d_b7: f64 = ((eq3_e104_d_b7 * s.v[54]) + (eq3_e104 * s.db[54][7]));
        (eq3_e106, eq3_e106_d_n0, eq3_e106_d_n1, eq3_e106_d_n2, eq3_e106_d_n3, eq3_e106_d_n4, eq3_e106_d_n5, eq3_e106_d_n6, eq3_e106_d_n7, eq3_e106_d_n8, eq3_e106_d_n9, eq3_e106_d_b0, eq3_e106_d_b1, eq3_e106_d_b2, eq3_e106_d_b3, eq3_e106_d_b4, eq3_e106_d_b5, eq3_e106_d_b6, eq3_e106_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e108;
        let eq3_node_derivatives: [f64; 10] = [eq3_e108_d_n0, eq3_e108_d_n1, eq3_e108_d_n2, eq3_e108_d_n3, eq3_e108_d_n4, eq3_e108_d_n5, eq3_e108_d_n6, eq3_e108_d_n7, eq3_e108_d_n8, eq3_e108_d_n9];
        let eq3_branch_derivatives: [f64; 8] = [eq3_e108_d_b0, eq3_e108_d_b1, eq3_e108_d_b2, eq3_e108_d_b3, eq3_e108_d_b4, eq3_e108_d_b5, eq3_e108_d_b6, eq3_e108_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            None,
            self.multiplicity * (eq3_value),
            &nodes,
            &eq3_node_derivatives,
            &branches,
            &eq3_branch_derivatives,
            self.multiplicity,
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
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq4_e114, eq4_e114_d_n8,) = {
    if (s.v[115] != 0.0) {
        let eq4_e112: f64 = (nv8 - 0.0);
        (eq4_e112, 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e114;
        stamper.stamp_current(
            Some(nodes[8]),
            None,
            self.multiplicity * (eq4_value),
            &[
                GeneratedDerivative::node(nodes[8], self.multiplicity * eq4_e114_d_n8),
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
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq5_e121, eq5_e121_d_n0, eq5_e121_d_n1, eq5_e121_d_n2, eq5_e121_d_n3, eq5_e121_d_n4, eq5_e121_d_n5, eq5_e121_d_n6, eq5_e121_d_n7, eq5_e121_d_n8, eq5_e121_d_n9, eq5_e121_d_b0, eq5_e121_d_b1, eq5_e121_d_b2, eq5_e121_d_b3, eq5_e121_d_b4, eq5_e121_d_b5, eq5_e121_d_b6, eq5_e121_d_b7,) = {
    if (s.v[115] != 0.0) {
        let eq5_e118: f64 = self.eval_ddt(1, (nv8 - 0.0));
        let eq5_e118_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq5_e118_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq5_e118_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq5_e118_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq5_e118_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq5_e118_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq5_e118_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq5_e118_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq5_e118_d_n8: f64 = self.ddt_jacobian(1.0);
        let eq5_e118_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq5_e118_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq5_e118_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq5_e118_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq5_e118_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq5_e118_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq5_e118_d_b5: f64 = self.ddt_jacobian(0.0);
        let eq5_e118_d_b6: f64 = self.ddt_jacobian(0.0);
        let eq5_e118_d_b7: f64 = self.ddt_jacobian(0.0);
        let eq5_e119: f64 = (s.v[54] * eq5_e118);
        let eq5_e119_d_n0: f64 = ((s.dn[54][0] * eq5_e118) + (s.v[54] * eq5_e118_d_n0));
        let eq5_e119_d_n1: f64 = ((s.dn[54][1] * eq5_e118) + (s.v[54] * eq5_e118_d_n1));
        let eq5_e119_d_n2: f64 = ((s.dn[54][2] * eq5_e118) + (s.v[54] * eq5_e118_d_n2));
        let eq5_e119_d_n3: f64 = ((s.dn[54][3] * eq5_e118) + (s.v[54] * eq5_e118_d_n3));
        let eq5_e119_d_n4: f64 = ((s.dn[54][4] * eq5_e118) + (s.v[54] * eq5_e118_d_n4));
        let eq5_e119_d_n5: f64 = ((s.dn[54][5] * eq5_e118) + (s.v[54] * eq5_e118_d_n5));
        let eq5_e119_d_n6: f64 = ((s.dn[54][6] * eq5_e118) + (s.v[54] * eq5_e118_d_n6));
        let eq5_e119_d_n7: f64 = ((s.dn[54][7] * eq5_e118) + (s.v[54] * eq5_e118_d_n7));
        let eq5_e119_d_n8: f64 = ((s.dn[54][8] * eq5_e118) + (s.v[54] * eq5_e118_d_n8));
        let eq5_e119_d_n9: f64 = ((s.dn[54][9] * eq5_e118) + (s.v[54] * eq5_e118_d_n9));
        let eq5_e119_d_b0: f64 = ((s.db[54][0] * eq5_e118) + (s.v[54] * eq5_e118_d_b0));
        let eq5_e119_d_b1: f64 = ((s.db[54][1] * eq5_e118) + (s.v[54] * eq5_e118_d_b1));
        let eq5_e119_d_b2: f64 = ((s.db[54][2] * eq5_e118) + (s.v[54] * eq5_e118_d_b2));
        let eq5_e119_d_b3: f64 = ((s.db[54][3] * eq5_e118) + (s.v[54] * eq5_e118_d_b3));
        let eq5_e119_d_b4: f64 = ((s.db[54][4] * eq5_e118) + (s.v[54] * eq5_e118_d_b4));
        let eq5_e119_d_b5: f64 = ((s.db[54][5] * eq5_e118) + (s.v[54] * eq5_e118_d_b5));
        let eq5_e119_d_b6: f64 = ((s.db[54][6] * eq5_e118) + (s.v[54] * eq5_e118_d_b6));
        let eq5_e119_d_b7: f64 = ((s.db[54][7] * eq5_e118) + (s.v[54] * eq5_e118_d_b7));
        (eq5_e119, eq5_e119_d_n0, eq5_e119_d_n1, eq5_e119_d_n2, eq5_e119_d_n3, eq5_e119_d_n4, eq5_e119_d_n5, eq5_e119_d_n6, eq5_e119_d_n7, eq5_e119_d_n8, eq5_e119_d_n9, eq5_e119_d_b0, eq5_e119_d_b1, eq5_e119_d_b2, eq5_e119_d_b3, eq5_e119_d_b4, eq5_e119_d_b5, eq5_e119_d_b6, eq5_e119_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e121;
        let eq5_node_derivatives: [f64; 10] = [eq5_e121_d_n0, eq5_e121_d_n1, eq5_e121_d_n2, eq5_e121_d_n3, eq5_e121_d_n4, eq5_e121_d_n5, eq5_e121_d_n6, eq5_e121_d_n7, eq5_e121_d_n8, eq5_e121_d_n9];
        let eq5_branch_derivatives: [f64; 8] = [eq5_e121_d_b0, eq5_e121_d_b1, eq5_e121_d_b2, eq5_e121_d_b3, eq5_e121_d_b4, eq5_e121_d_b5, eq5_e121_d_b6, eq5_e121_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            None,
            self.multiplicity * (eq5_value),
            &nodes,
            &eq5_node_derivatives,
            &branches,
            &eq5_branch_derivatives,
            self.multiplicity,
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
        let (eq6_e126,) = {
    if (!(s.v[115] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq6_value: f64 = eq6_e126;
        stamper.stamp_potential(
            branches[0],
            eq6_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_7_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let (eq7_e141, eq7_e141_d_n0, eq7_e141_d_n1, eq7_e141_d_n2, eq7_e141_d_n3, eq7_e141_d_n4, eq7_e141_d_n5, eq7_e141_d_n6, eq7_e141_d_n7, eq7_e141_d_n8, eq7_e141_d_n9, eq7_e141_d_b0, eq7_e141_d_b1, eq7_e141_d_b2, eq7_e141_d_b3, eq7_e141_d_b4, eq7_e141_d_b5, eq7_e141_d_b6, eq7_e141_d_b7,) = {
    if (s.v[122] != 0.0) {
        let eq7_e129: f64 = (-1.0);
        let eq7_e132: f64 = (s.v[37] * (nv1 - nv2));
        let eq7_e132_d_n0: f64 = (s.dn[37][0] * (nv1 - nv2));
        let eq7_e132_d_n1: f64 = ((s.dn[37][1] * (nv1 - nv2)) + s.v[37]);
        let eq7_e132_d_n2: f64 = ((s.dn[37][2] * (nv1 - nv2)) + (-s.v[37]));
        let eq7_e132_d_n3: f64 = (s.dn[37][3] * (nv1 - nv2));
        let eq7_e132_d_n4: f64 = (s.dn[37][4] * (nv1 - nv2));
        let eq7_e132_d_n5: f64 = (s.dn[37][5] * (nv1 - nv2));
        let eq7_e132_d_n6: f64 = (s.dn[37][6] * (nv1 - nv2));
        let eq7_e132_d_n7: f64 = (s.dn[37][7] * (nv1 - nv2));
        let eq7_e132_d_n8: f64 = (s.dn[37][8] * (nv1 - nv2));
        let eq7_e132_d_n9: f64 = (s.dn[37][9] * (nv1 - nv2));
        let eq7_e132_d_b0: f64 = (s.db[37][0] * (nv1 - nv2));
        let eq7_e132_d_b1: f64 = (s.db[37][1] * (nv1 - nv2));
        let eq7_e132_d_b2: f64 = (s.db[37][2] * (nv1 - nv2));
        let eq7_e132_d_b3: f64 = (s.db[37][3] * (nv1 - nv2));
        let eq7_e132_d_b4: f64 = (s.db[37][4] * (nv1 - nv2));
        let eq7_e132_d_b5: f64 = (s.db[37][5] * (nv1 - nv2));
        let eq7_e132_d_b6: f64 = (s.db[37][6] * (nv1 - nv2));
        let eq7_e132_d_b7: f64 = (s.db[37][7] * (nv1 - nv2));
        let eq7_e133: f64 = (eq7_e132).abs();
        let eq7_e133_d_n0: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n0 } else { (-eq7_e132_d_n0) };
        let eq7_e133_d_n1: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n1 } else { (-eq7_e132_d_n1) };
        let eq7_e133_d_n2: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n2 } else { (-eq7_e132_d_n2) };
        let eq7_e133_d_n3: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n3 } else { (-eq7_e132_d_n3) };
        let eq7_e133_d_n4: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n4 } else { (-eq7_e132_d_n4) };
        let eq7_e133_d_n5: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n5 } else { (-eq7_e132_d_n5) };
        let eq7_e133_d_n6: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n6 } else { (-eq7_e132_d_n6) };
        let eq7_e133_d_n7: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n7 } else { (-eq7_e132_d_n7) };
        let eq7_e133_d_n8: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n8 } else { (-eq7_e132_d_n8) };
        let eq7_e133_d_n9: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_n9 } else { (-eq7_e132_d_n9) };
        let eq7_e133_d_b0: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_b0 } else { (-eq7_e132_d_b0) };
        let eq7_e133_d_b1: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_b1 } else { (-eq7_e132_d_b1) };
        let eq7_e133_d_b2: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_b2 } else { (-eq7_e132_d_b2) };
        let eq7_e133_d_b3: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_b3 } else { (-eq7_e132_d_b3) };
        let eq7_e133_d_b4: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_b4 } else { (-eq7_e132_d_b4) };
        let eq7_e133_d_b5: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_b5 } else { (-eq7_e132_d_b5) };
        let eq7_e133_d_b6: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_b6 } else { (-eq7_e132_d_b6) };
        let eq7_e133_d_b7: f64 = if eq7_e132 >= 0.0 { eq7_e132_d_b7 } else { (-eq7_e132_d_b7) };
        let eq7_e134: f64 = (eq7_e129 * eq7_e133);
        let eq7_e134_d_n0: f64 = (eq7_e129 * eq7_e133_d_n0);
        let eq7_e134_d_n1: f64 = (eq7_e129 * eq7_e133_d_n1);
        let eq7_e134_d_n2: f64 = (eq7_e129 * eq7_e133_d_n2);
        let eq7_e134_d_n3: f64 = (eq7_e129 * eq7_e133_d_n3);
        let eq7_e134_d_n4: f64 = (eq7_e129 * eq7_e133_d_n4);
        let eq7_e134_d_n5: f64 = (eq7_e129 * eq7_e133_d_n5);
        let eq7_e134_d_n6: f64 = (eq7_e129 * eq7_e133_d_n6);
        let eq7_e134_d_n7: f64 = (eq7_e129 * eq7_e133_d_n7);
        let eq7_e134_d_n8: f64 = (eq7_e129 * eq7_e133_d_n8);
        let eq7_e134_d_n9: f64 = (eq7_e129 * eq7_e133_d_n9);
        let eq7_e134_d_b0: f64 = (eq7_e129 * eq7_e133_d_b0);
        let eq7_e134_d_b1: f64 = (eq7_e129 * eq7_e133_d_b1);
        let eq7_e134_d_b2: f64 = (eq7_e129 * eq7_e133_d_b2);
        let eq7_e134_d_b3: f64 = (eq7_e129 * eq7_e133_d_b3);
        let eq7_e134_d_b4: f64 = (eq7_e129 * eq7_e133_d_b4);
        let eq7_e134_d_b5: f64 = (eq7_e129 * eq7_e133_d_b5);
        let eq7_e134_d_b6: f64 = (eq7_e129 * eq7_e133_d_b6);
        let eq7_e134_d_b7: f64 = (eq7_e129 * eq7_e133_d_b7);
        let eq7_e137: f64 = (s.v[40] * (nv1 - nv0));
        let eq7_e137_d_n0: f64 = ((s.dn[40][0] * (nv1 - nv0)) + (-s.v[40]));
        let eq7_e137_d_n1: f64 = ((s.dn[40][1] * (nv1 - nv0)) + s.v[40]);
        let eq7_e137_d_n2: f64 = (s.dn[40][2] * (nv1 - nv0));
        let eq7_e137_d_n3: f64 = (s.dn[40][3] * (nv1 - nv0));
        let eq7_e137_d_n4: f64 = (s.dn[40][4] * (nv1 - nv0));
        let eq7_e137_d_n5: f64 = (s.dn[40][5] * (nv1 - nv0));
        let eq7_e137_d_n6: f64 = (s.dn[40][6] * (nv1 - nv0));
        let eq7_e137_d_n7: f64 = (s.dn[40][7] * (nv1 - nv0));
        let eq7_e137_d_n8: f64 = (s.dn[40][8] * (nv1 - nv0));
        let eq7_e137_d_n9: f64 = (s.dn[40][9] * (nv1 - nv0));
        let eq7_e137_d_b0: f64 = (s.db[40][0] * (nv1 - nv0));
        let eq7_e137_d_b1: f64 = (s.db[40][1] * (nv1 - nv0));
        let eq7_e137_d_b2: f64 = (s.db[40][2] * (nv1 - nv0));
        let eq7_e137_d_b3: f64 = (s.db[40][3] * (nv1 - nv0));
        let eq7_e137_d_b4: f64 = (s.db[40][4] * (nv1 - nv0));
        let eq7_e137_d_b5: f64 = (s.db[40][5] * (nv1 - nv0));
        let eq7_e137_d_b6: f64 = (s.db[40][6] * (nv1 - nv0));
        let eq7_e137_d_b7: f64 = (s.db[40][7] * (nv1 - nv0));
        let eq7_e138: f64 = (eq7_e137).abs();
        let eq7_e138_d_n0: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n0 } else { (-eq7_e137_d_n0) };
        let eq7_e138_d_n1: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n1 } else { (-eq7_e137_d_n1) };
        let eq7_e138_d_n2: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n2 } else { (-eq7_e137_d_n2) };
        let eq7_e138_d_n3: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n3 } else { (-eq7_e137_d_n3) };
        let eq7_e138_d_n4: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n4 } else { (-eq7_e137_d_n4) };
        let eq7_e138_d_n5: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n5 } else { (-eq7_e137_d_n5) };
        let eq7_e138_d_n6: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n6 } else { (-eq7_e137_d_n6) };
        let eq7_e138_d_n7: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n7 } else { (-eq7_e137_d_n7) };
        let eq7_e138_d_n8: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n8 } else { (-eq7_e137_d_n8) };
        let eq7_e138_d_n9: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_n9 } else { (-eq7_e137_d_n9) };
        let eq7_e138_d_b0: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_b0 } else { (-eq7_e137_d_b0) };
        let eq7_e138_d_b1: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_b1 } else { (-eq7_e137_d_b1) };
        let eq7_e138_d_b2: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_b2 } else { (-eq7_e137_d_b2) };
        let eq7_e138_d_b3: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_b3 } else { (-eq7_e137_d_b3) };
        let eq7_e138_d_b4: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_b4 } else { (-eq7_e137_d_b4) };
        let eq7_e138_d_b5: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_b5 } else { (-eq7_e137_d_b5) };
        let eq7_e138_d_b6: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_b6 } else { (-eq7_e137_d_b6) };
        let eq7_e138_d_b7: f64 = if eq7_e137 >= 0.0 { eq7_e137_d_b7 } else { (-eq7_e137_d_b7) };
        let eq7_e139: f64 = (eq7_e134 - eq7_e138);
        let eq7_e139_d_n0: f64 = (eq7_e134_d_n0 - eq7_e138_d_n0);
        let eq7_e139_d_n1: f64 = (eq7_e134_d_n1 - eq7_e138_d_n1);
        let eq7_e139_d_n2: f64 = (eq7_e134_d_n2 - eq7_e138_d_n2);
        let eq7_e139_d_n3: f64 = (eq7_e134_d_n3 - eq7_e138_d_n3);
        let eq7_e139_d_n4: f64 = (eq7_e134_d_n4 - eq7_e138_d_n4);
        let eq7_e139_d_n5: f64 = (eq7_e134_d_n5 - eq7_e138_d_n5);
        let eq7_e139_d_n6: f64 = (eq7_e134_d_n6 - eq7_e138_d_n6);
        let eq7_e139_d_n7: f64 = (eq7_e134_d_n7 - eq7_e138_d_n7);
        let eq7_e139_d_n8: f64 = (eq7_e134_d_n8 - eq7_e138_d_n8);
        let eq7_e139_d_n9: f64 = (eq7_e134_d_n9 - eq7_e138_d_n9);
        let eq7_e139_d_b0: f64 = (eq7_e134_d_b0 - eq7_e138_d_b0);
        let eq7_e139_d_b1: f64 = (eq7_e134_d_b1 - eq7_e138_d_b1);
        let eq7_e139_d_b2: f64 = (eq7_e134_d_b2 - eq7_e138_d_b2);
        let eq7_e139_d_b3: f64 = (eq7_e134_d_b3 - eq7_e138_d_b3);
        let eq7_e139_d_b4: f64 = (eq7_e134_d_b4 - eq7_e138_d_b4);
        let eq7_e139_d_b5: f64 = (eq7_e134_d_b5 - eq7_e138_d_b5);
        let eq7_e139_d_b6: f64 = (eq7_e134_d_b6 - eq7_e138_d_b6);
        let eq7_e139_d_b7: f64 = (eq7_e134_d_b7 - eq7_e138_d_b7);
        (eq7_e139, eq7_e139_d_n0, eq7_e139_d_n1, eq7_e139_d_n2, eq7_e139_d_n3, eq7_e139_d_n4, eq7_e139_d_n5, eq7_e139_d_n6, eq7_e139_d_n7, eq7_e139_d_n8, eq7_e139_d_n9, eq7_e139_d_b0, eq7_e139_d_b1, eq7_e139_d_b2, eq7_e139_d_b3, eq7_e139_d_b4, eq7_e139_d_b5, eq7_e139_d_b6, eq7_e139_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e141;
        let eq7_node_derivatives: [f64; 10] = [eq7_e141_d_n0, eq7_e141_d_n1, eq7_e141_d_n2, eq7_e141_d_n3, eq7_e141_d_n4, eq7_e141_d_n5, eq7_e141_d_n6, eq7_e141_d_n7, eq7_e141_d_n8, eq7_e141_d_n9];
        let eq7_branch_derivatives: [f64; 8] = [eq7_e141_d_b0, eq7_e141_d_b1, eq7_e141_d_b2, eq7_e141_d_b3, eq7_e141_d_b4, eq7_e141_d_b5, eq7_e141_d_b6, eq7_e141_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            None,
            self.multiplicity * (eq7_value),
            &nodes,
            &eq7_node_derivatives,
            &branches,
            &eq7_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_8_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv3 = ctx.node_voltage(nodes[3]);
        let (eq8_e147, eq8_e147_d_n3,) = {
    if (s.v[122] != 0.0) {
        let eq8_e145: f64 = ((nv3 - 0.0) / p.p33);
        let eq8_e145_d_n3: f64 = (1.0 / p.p33);
        (eq8_e145, eq8_e145_d_n3,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e147;
        stamper.stamp_current(
            Some(nodes[3]),
            None,
            self.multiplicity * (eq8_value),
            &[
                GeneratedDerivative::node(nodes[3], self.multiplicity * eq8_e147_d_n3),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_9_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv3 = ctx.node_voltage(nodes[3]);
        let (eq9_e154, eq9_e154_d_n0, eq9_e154_d_n1, eq9_e154_d_n2, eq9_e154_d_n3, eq9_e154_d_n4, eq9_e154_d_n5, eq9_e154_d_n6, eq9_e154_d_n7, eq9_e154_d_n8, eq9_e154_d_n9, eq9_e154_d_b0, eq9_e154_d_b1, eq9_e154_d_b2, eq9_e154_d_b3, eq9_e154_d_b4, eq9_e154_d_b5, eq9_e154_d_b6, eq9_e154_d_b7,) = {
    if (s.v[122] != 0.0) {
        let eq9_e151: f64 = ((nv3 - 0.0) * p.p34);
        let eq9_e151_d_n3: f64 = p.p34;
        let eq9_e152: f64 = self.eval_ddt(2, eq9_e151);
        let eq9_e152_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq9_e152_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq9_e152_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq9_e152_d_n3: f64 = self.ddt_jacobian(eq9_e151_d_n3);
        let eq9_e152_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq9_e152_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq9_e152_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq9_e152_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq9_e152_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq9_e152_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq9_e152_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq9_e152_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq9_e152_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq9_e152_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq9_e152_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq9_e152_d_b5: f64 = self.ddt_jacobian(0.0);
        let eq9_e152_d_b6: f64 = self.ddt_jacobian(0.0);
        let eq9_e152_d_b7: f64 = self.ddt_jacobian(0.0);
        (eq9_e152, eq9_e152_d_n0, eq9_e152_d_n1, eq9_e152_d_n2, eq9_e152_d_n3, eq9_e152_d_n4, eq9_e152_d_n5, eq9_e152_d_n6, eq9_e152_d_n7, eq9_e152_d_n8, eq9_e152_d_n9, eq9_e152_d_b0, eq9_e152_d_b1, eq9_e152_d_b2, eq9_e152_d_b3, eq9_e152_d_b4, eq9_e152_d_b5, eq9_e152_d_b6, eq9_e152_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e154;
        let eq9_node_derivatives: [f64; 10] = [eq9_e154_d_n0, eq9_e154_d_n1, eq9_e154_d_n2, eq9_e154_d_n3, eq9_e154_d_n4, eq9_e154_d_n5, eq9_e154_d_n6, eq9_e154_d_n7, eq9_e154_d_n8, eq9_e154_d_n9];
        let eq9_branch_derivatives: [f64; 8] = [eq9_e154_d_b0, eq9_e154_d_b1, eq9_e154_d_b2, eq9_e154_d_b3, eq9_e154_d_b4, eq9_e154_d_b5, eq9_e154_d_b6, eq9_e154_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            None,
            self.multiplicity * (eq9_value),
            &nodes,
            &eq9_node_derivatives,
            &branches,
            &eq9_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_10_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq10_e158,) = {
    if (s.v[122] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq10_value: f64 = eq10_e158;
        stamper.stamp_potential(
            branches[1],
            eq10_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_11_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let (eq11_e176, eq11_e176_d_n0, eq11_e176_d_n1, eq11_e176_d_n2, eq11_e176_d_n3, eq11_e176_d_n4, eq11_e176_d_n5, eq11_e176_d_n6, eq11_e176_d_n7, eq11_e176_d_n8, eq11_e176_d_n9, eq11_e176_d_b0, eq11_e176_d_b1, eq11_e176_d_b2, eq11_e176_d_b3, eq11_e176_d_b4, eq11_e176_d_b5, eq11_e176_d_b6, eq11_e176_d_b7,) = {
    if ((!(s.v[122] != 0.0)) && (s.v[123] != 0.0)) {
        let eq11_e164: f64 = (-1.0);
        let eq11_e167: f64 = (s.v[37] * (nv1 - nv2));
        let eq11_e167_d_n0: f64 = (s.dn[37][0] * (nv1 - nv2));
        let eq11_e167_d_n1: f64 = ((s.dn[37][1] * (nv1 - nv2)) + s.v[37]);
        let eq11_e167_d_n2: f64 = ((s.dn[37][2] * (nv1 - nv2)) + (-s.v[37]));
        let eq11_e167_d_n3: f64 = (s.dn[37][3] * (nv1 - nv2));
        let eq11_e167_d_n4: f64 = (s.dn[37][4] * (nv1 - nv2));
        let eq11_e167_d_n5: f64 = (s.dn[37][5] * (nv1 - nv2));
        let eq11_e167_d_n6: f64 = (s.dn[37][6] * (nv1 - nv2));
        let eq11_e167_d_n7: f64 = (s.dn[37][7] * (nv1 - nv2));
        let eq11_e167_d_n8: f64 = (s.dn[37][8] * (nv1 - nv2));
        let eq11_e167_d_n9: f64 = (s.dn[37][9] * (nv1 - nv2));
        let eq11_e167_d_b0: f64 = (s.db[37][0] * (nv1 - nv2));
        let eq11_e167_d_b1: f64 = (s.db[37][1] * (nv1 - nv2));
        let eq11_e167_d_b2: f64 = (s.db[37][2] * (nv1 - nv2));
        let eq11_e167_d_b3: f64 = (s.db[37][3] * (nv1 - nv2));
        let eq11_e167_d_b4: f64 = (s.db[37][4] * (nv1 - nv2));
        let eq11_e167_d_b5: f64 = (s.db[37][5] * (nv1 - nv2));
        let eq11_e167_d_b6: f64 = (s.db[37][6] * (nv1 - nv2));
        let eq11_e167_d_b7: f64 = (s.db[37][7] * (nv1 - nv2));
        let eq11_e168: f64 = (eq11_e167).abs();
        let eq11_e168_d_n0: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n0 } else { (-eq11_e167_d_n0) };
        let eq11_e168_d_n1: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n1 } else { (-eq11_e167_d_n1) };
        let eq11_e168_d_n2: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n2 } else { (-eq11_e167_d_n2) };
        let eq11_e168_d_n3: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n3 } else { (-eq11_e167_d_n3) };
        let eq11_e168_d_n4: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n4 } else { (-eq11_e167_d_n4) };
        let eq11_e168_d_n5: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n5 } else { (-eq11_e167_d_n5) };
        let eq11_e168_d_n6: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n6 } else { (-eq11_e167_d_n6) };
        let eq11_e168_d_n7: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n7 } else { (-eq11_e167_d_n7) };
        let eq11_e168_d_n8: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n8 } else { (-eq11_e167_d_n8) };
        let eq11_e168_d_n9: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_n9 } else { (-eq11_e167_d_n9) };
        let eq11_e168_d_b0: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_b0 } else { (-eq11_e167_d_b0) };
        let eq11_e168_d_b1: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_b1 } else { (-eq11_e167_d_b1) };
        let eq11_e168_d_b2: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_b2 } else { (-eq11_e167_d_b2) };
        let eq11_e168_d_b3: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_b3 } else { (-eq11_e167_d_b3) };
        let eq11_e168_d_b4: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_b4 } else { (-eq11_e167_d_b4) };
        let eq11_e168_d_b5: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_b5 } else { (-eq11_e167_d_b5) };
        let eq11_e168_d_b6: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_b6 } else { (-eq11_e167_d_b6) };
        let eq11_e168_d_b7: f64 = if eq11_e167 >= 0.0 { eq11_e167_d_b7 } else { (-eq11_e167_d_b7) };
        let eq11_e169: f64 = (eq11_e164 * eq11_e168);
        let eq11_e169_d_n0: f64 = (eq11_e164 * eq11_e168_d_n0);
        let eq11_e169_d_n1: f64 = (eq11_e164 * eq11_e168_d_n1);
        let eq11_e169_d_n2: f64 = (eq11_e164 * eq11_e168_d_n2);
        let eq11_e169_d_n3: f64 = (eq11_e164 * eq11_e168_d_n3);
        let eq11_e169_d_n4: f64 = (eq11_e164 * eq11_e168_d_n4);
        let eq11_e169_d_n5: f64 = (eq11_e164 * eq11_e168_d_n5);
        let eq11_e169_d_n6: f64 = (eq11_e164 * eq11_e168_d_n6);
        let eq11_e169_d_n7: f64 = (eq11_e164 * eq11_e168_d_n7);
        let eq11_e169_d_n8: f64 = (eq11_e164 * eq11_e168_d_n8);
        let eq11_e169_d_n9: f64 = (eq11_e164 * eq11_e168_d_n9);
        let eq11_e169_d_b0: f64 = (eq11_e164 * eq11_e168_d_b0);
        let eq11_e169_d_b1: f64 = (eq11_e164 * eq11_e168_d_b1);
        let eq11_e169_d_b2: f64 = (eq11_e164 * eq11_e168_d_b2);
        let eq11_e169_d_b3: f64 = (eq11_e164 * eq11_e168_d_b3);
        let eq11_e169_d_b4: f64 = (eq11_e164 * eq11_e168_d_b4);
        let eq11_e169_d_b5: f64 = (eq11_e164 * eq11_e168_d_b5);
        let eq11_e169_d_b6: f64 = (eq11_e164 * eq11_e168_d_b6);
        let eq11_e169_d_b7: f64 = (eq11_e164 * eq11_e168_d_b7);
        let eq11_e172: f64 = (s.v[40] * (nv1 - nv0));
        let eq11_e172_d_n0: f64 = ((s.dn[40][0] * (nv1 - nv0)) + (-s.v[40]));
        let eq11_e172_d_n1: f64 = ((s.dn[40][1] * (nv1 - nv0)) + s.v[40]);
        let eq11_e172_d_n2: f64 = (s.dn[40][2] * (nv1 - nv0));
        let eq11_e172_d_n3: f64 = (s.dn[40][3] * (nv1 - nv0));
        let eq11_e172_d_n4: f64 = (s.dn[40][4] * (nv1 - nv0));
        let eq11_e172_d_n5: f64 = (s.dn[40][5] * (nv1 - nv0));
        let eq11_e172_d_n6: f64 = (s.dn[40][6] * (nv1 - nv0));
        let eq11_e172_d_n7: f64 = (s.dn[40][7] * (nv1 - nv0));
        let eq11_e172_d_n8: f64 = (s.dn[40][8] * (nv1 - nv0));
        let eq11_e172_d_n9: f64 = (s.dn[40][9] * (nv1 - nv0));
        let eq11_e172_d_b0: f64 = (s.db[40][0] * (nv1 - nv0));
        let eq11_e172_d_b1: f64 = (s.db[40][1] * (nv1 - nv0));
        let eq11_e172_d_b2: f64 = (s.db[40][2] * (nv1 - nv0));
        let eq11_e172_d_b3: f64 = (s.db[40][3] * (nv1 - nv0));
        let eq11_e172_d_b4: f64 = (s.db[40][4] * (nv1 - nv0));
        let eq11_e172_d_b5: f64 = (s.db[40][5] * (nv1 - nv0));
        let eq11_e172_d_b6: f64 = (s.db[40][6] * (nv1 - nv0));
        let eq11_e172_d_b7: f64 = (s.db[40][7] * (nv1 - nv0));
        let eq11_e173: f64 = (eq11_e172).abs();
        let eq11_e173_d_n0: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n0 } else { (-eq11_e172_d_n0) };
        let eq11_e173_d_n1: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n1 } else { (-eq11_e172_d_n1) };
        let eq11_e173_d_n2: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n2 } else { (-eq11_e172_d_n2) };
        let eq11_e173_d_n3: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n3 } else { (-eq11_e172_d_n3) };
        let eq11_e173_d_n4: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n4 } else { (-eq11_e172_d_n4) };
        let eq11_e173_d_n5: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n5 } else { (-eq11_e172_d_n5) };
        let eq11_e173_d_n6: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n6 } else { (-eq11_e172_d_n6) };
        let eq11_e173_d_n7: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n7 } else { (-eq11_e172_d_n7) };
        let eq11_e173_d_n8: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n8 } else { (-eq11_e172_d_n8) };
        let eq11_e173_d_n9: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_n9 } else { (-eq11_e172_d_n9) };
        let eq11_e173_d_b0: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_b0 } else { (-eq11_e172_d_b0) };
        let eq11_e173_d_b1: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_b1 } else { (-eq11_e172_d_b1) };
        let eq11_e173_d_b2: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_b2 } else { (-eq11_e172_d_b2) };
        let eq11_e173_d_b3: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_b3 } else { (-eq11_e172_d_b3) };
        let eq11_e173_d_b4: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_b4 } else { (-eq11_e172_d_b4) };
        let eq11_e173_d_b5: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_b5 } else { (-eq11_e172_d_b5) };
        let eq11_e173_d_b6: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_b6 } else { (-eq11_e172_d_b6) };
        let eq11_e173_d_b7: f64 = if eq11_e172 >= 0.0 { eq11_e172_d_b7 } else { (-eq11_e172_d_b7) };
        let eq11_e174: f64 = (eq11_e169 - eq11_e173);
        let eq11_e174_d_n0: f64 = (eq11_e169_d_n0 - eq11_e173_d_n0);
        let eq11_e174_d_n1: f64 = (eq11_e169_d_n1 - eq11_e173_d_n1);
        let eq11_e174_d_n2: f64 = (eq11_e169_d_n2 - eq11_e173_d_n2);
        let eq11_e174_d_n3: f64 = (eq11_e169_d_n3 - eq11_e173_d_n3);
        let eq11_e174_d_n4: f64 = (eq11_e169_d_n4 - eq11_e173_d_n4);
        let eq11_e174_d_n5: f64 = (eq11_e169_d_n5 - eq11_e173_d_n5);
        let eq11_e174_d_n6: f64 = (eq11_e169_d_n6 - eq11_e173_d_n6);
        let eq11_e174_d_n7: f64 = (eq11_e169_d_n7 - eq11_e173_d_n7);
        let eq11_e174_d_n8: f64 = (eq11_e169_d_n8 - eq11_e173_d_n8);
        let eq11_e174_d_n9: f64 = (eq11_e169_d_n9 - eq11_e173_d_n9);
        let eq11_e174_d_b0: f64 = (eq11_e169_d_b0 - eq11_e173_d_b0);
        let eq11_e174_d_b1: f64 = (eq11_e169_d_b1 - eq11_e173_d_b1);
        let eq11_e174_d_b2: f64 = (eq11_e169_d_b2 - eq11_e173_d_b2);
        let eq11_e174_d_b3: f64 = (eq11_e169_d_b3 - eq11_e173_d_b3);
        let eq11_e174_d_b4: f64 = (eq11_e169_d_b4 - eq11_e173_d_b4);
        let eq11_e174_d_b5: f64 = (eq11_e169_d_b5 - eq11_e173_d_b5);
        let eq11_e174_d_b6: f64 = (eq11_e169_d_b6 - eq11_e173_d_b6);
        let eq11_e174_d_b7: f64 = (eq11_e169_d_b7 - eq11_e173_d_b7);
        (eq11_e174, eq11_e174_d_n0, eq11_e174_d_n1, eq11_e174_d_n2, eq11_e174_d_n3, eq11_e174_d_n4, eq11_e174_d_n5, eq11_e174_d_n6, eq11_e174_d_n7, eq11_e174_d_n8, eq11_e174_d_n9, eq11_e174_d_b0, eq11_e174_d_b1, eq11_e174_d_b2, eq11_e174_d_b3, eq11_e174_d_b4, eq11_e174_d_b5, eq11_e174_d_b6, eq11_e174_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e176;
        let eq11_node_derivatives: [f64; 10] = [eq11_e176_d_n0, eq11_e176_d_n1, eq11_e176_d_n2, eq11_e176_d_n3, eq11_e176_d_n4, eq11_e176_d_n5, eq11_e176_d_n6, eq11_e176_d_n7, eq11_e176_d_n8, eq11_e176_d_n9];
        let eq11_branch_derivatives: [f64; 8] = [eq11_e176_d_b0, eq11_e176_d_b1, eq11_e176_d_b2, eq11_e176_d_b3, eq11_e176_d_b4, eq11_e176_d_b5, eq11_e176_d_b6, eq11_e176_d_b7];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            None,
            self.multiplicity * (eq11_value),
            &nodes,
            &eq11_node_derivatives,
            &branches,
            &eq11_branch_derivatives,
            self.multiplicity,
        );
    }
}
