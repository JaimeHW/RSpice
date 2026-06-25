#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv19 = ctx.node_voltage(nodes[19]);
        let nv21 = ctx.node_voltage(nodes[21]);
        s.v[0] = 0.0;

        s.v[109] = (p.p5 + 273.15);

        s.v[108] = ctx.temperature();

        s.store_ad(110, &A::voltage(ctx, &nodes, Some(4), None));

        s.store_offset(111, 110, (s.v[108] + p.p3));

        s.v[298] = if (s.v[111] < ((-270.0) + 273.15)) { 1.0 } else { 0.0 };

        if (s.v[298] != 0.0) {
            s.store_scalar(111, ((-270.0) + 273.15));
        }

        s.v[299] = if (s.v[111] > (1500.0 + 273.15)) { 1.0 } else { 0.0 };

        if ((!(s.v[298] != 0.0)) && (s.v[299] != 0.0)) {
            s.store_scalar(111, (1500.0 + 273.15));
        }

        s.v[2] = 0.0;

        s.v[1] = 0.0;

        s.v[300] = if (p.p50 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[300] != 0.0) {
            s.store_scalar(3, ((p.p30 / p.p0) / p.p2));
        }

        if (s.v[300] != 0.0) {
            s.store_scalar(4, ((p.p31 / p.p0) / p.p2));
        }

        if (!(s.v[300] != 0.0)) {
            s.store_scalar(3, (((p.p30 / p.p0) + ((p.p29 * p.p54) / p.p0)) / p.p2));
        }

        if (!(s.v[300] != 0.0)) {
            s.store_scalar(4, (((p.p31 / p.p0) + ((p.p29 * p.p66) / p.p0)) / p.p2));
        }

        s.v[301] = if ((s.v[3] >= p.p353) && (s.v[3] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[301] != 0.0) {
            s.store_mul_ad_rhs(2, 3, A::add(A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p48), 1.0), A::mul(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p49), A::offset(s.ad_value(111), (-s.v[109])))));
        }

        s.v[302] = if (s.v[2] < (0.1 * s.v[3])) { 1.0 } else { 0.0 };

        if ((s.v[301] != 0.0) && (s.v[302] != 0.0)) {
            s.store_scale(2, 3, 0.1);
        }

        if (!(s.v[301] != 0.0)) {
            s.store_scalar(2, 0.0);
        }

        s.v[303] = if ((s.v[4] >= p.p353) && (s.v[4] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[303] != 0.0) {
            s.store_mul_ad_rhs(1, 4, A::add(A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p48), 1.0), A::mul(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p49), A::offset(s.ad_value(111), (-s.v[109])))));
        }

        s.v[304] = if (s.v[1] < (0.1 * s.v[4])) { 1.0 } else { 0.0 };

        if ((s.v[303] != 0.0) && (s.v[304] != 0.0)) {
            s.store_scale(1, 4, 0.1);
        }

        if (!(s.v[303] != 0.0)) {
            s.store_scalar(1, 0.0);
        }

        s.v[5] = (((p.p324 / p.p2) / p.p325) * (p.p326 + ((p.p327 * p.p0) / p.p325)));

        s.v[6] = (((p.p324 / p.p2) / p.p325) * (((1.0 - p.p327) * p.p0) / p.p325));

        s.store_scale(113, 111, (1.38062e-23 * 6.241457005723417e18));

        s.store_offset_ad(223, A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p336), 1.0);

        s.v[305] = if (s.v[223] < 0.1) { 1.0 } else { 0.0 };

        if (s.v[305] != 0.0) {
            s.store_scalar(223, 0.1);
        }

        s.store_powf_ad(112, A::scale(s.ad_value(111), 1.0 / (s.v[109])), 3.0);

        s.store_ad(7, &A::scale({
            if ((1.0 + (p.p21 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p21), 1.0)
            }
        }, p.p9));

        s.store_ad(8, &A::scale({
            if ((1.0 + (p.p22 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p22), 1.0)
            }
        }, p.p10));

        s.store_ad(9, &A::scale({
            if ((1.0 + (p.p23 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p23), 1.0)
            }
        }, p.p11));

        s.store_ad(10, &A::scale({
            if ((1.0 + (p.p24 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p24), 1.0)
            }
        }, p.p13));

        s.store_ad(11, &A::scale({
            if ((1.0 + (p.p25 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p25), 1.0)
            }
        }, p.p12));

        s.store_ad(12, &A::scale({
            if ((1.0 + (p.p26 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p26), 1.0)
            }
        }, p.p14));

        s.store_ad(13, &A::scale({
            if ((1.0 + (p.p21 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p21), 1.0)
            }
        }, p.p15));

        s.store_ad(14, &A::scale({
            if ((1.0 + (p.p22 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p22), 1.0)
            }
        }, p.p16));

        s.store_ad(15, &A::scale({
            if ((1.0 + (p.p23 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p23), 1.0)
            }
        }, p.p17));

        s.store_ad(16, &A::scale({
            if ((1.0 + (p.p24 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p24), 1.0)
            }
        }, p.p19));

        s.store_ad(17, &A::scale({
            if ((1.0 + (p.p25 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p25), 1.0)
            }
        }, p.p18));

        s.store_ad(18, &A::scale({
            if ((1.0 + (p.p26 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p26), 1.0)
            }
        }, p.p20));

        s.store_ad(19, &A::scale({
            if ((1.0 + (p.p8 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p8), 1.0)
            }
        }, p.p7));

        s.store_ad(20, &A::scale({
            if ((1.0 + (p.p82 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p82), 1.0)
            }
        }, p.p81));

        s.store_ad(23, &A::scale({
            if ((1.0 + (p.p104 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p104), 1.0)
            }
        }, p.p103));

        s.store_ad(26, &A::scale({
            if ((1.0 + (p.p126 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p126), 1.0)
            }
        }, p.p125));

        s.store_ad(29, &A::scale({
            if ((1.0 + (p.p148 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p148), 1.0)
            }
        }, p.p147));

        s.store_ad(21, &A::scale({
            if ((1.0 + (p.p87 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p87), 1.0)
            }
        }, p.p86));

        s.store_ad(24, &A::scale({
            if ((1.0 + (p.p109 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p109), 1.0)
            }
        }, p.p108));

        s.store_ad(27, &A::scale({
            if ((1.0 + (p.p131 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p131), 1.0)
            }
        }, p.p130));

        s.store_ad(30, &A::scale({
            if ((1.0 + (p.p153 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p153), 1.0)
            }
        }, p.p152));

        s.store_ad(22, &A::scale({
            if ((1.0 + (p.p89 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p89), 1.0)
            }
        }, p.p88));

        s.store_ad(25, &A::scale({
            if ((1.0 + (p.p111 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p111), 1.0)
            }
        }, p.p110));

        s.store_ad(28, &A::scale({
            if ((1.0 + (p.p133 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p133), 1.0)
            }
        }, p.p132));

        s.store_ad(31, &A::scale({
            if ((1.0 + (p.p155 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p155), 1.0)
            }
        }, p.p154));

        s.store_ad(32, &A::scale({
            if ((1.0 + (p.p170 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p170), 1.0)
            }
        }, p.p169));

        s.store_ad(35, &A::scale({
            if ((1.0 + (p.p192 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p192), 1.0)
            }
        }, p.p191));

        s.store_ad(38, &A::scale({
            if ((1.0 + (p.p214 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p214), 1.0)
            }
        }, p.p213));

        s.store_ad(41, &A::scale({
            if ((1.0 + (p.p236 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p236), 1.0)
            }
        }, p.p235));

        s.store_ad(33, &A::scale({
            if ((1.0 + (p.p175 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p175), 1.0)
            }
        }, p.p174));

        s.store_ad(36, &A::scale({
            if ((1.0 + (p.p197 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p197), 1.0)
            }
        }, p.p196));

        s.store_ad(39, &A::scale({
            if ((1.0 + (p.p219 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p219), 1.0)
            }
        }, p.p218));

        s.store_ad(42, &A::scale({
            if ((1.0 + (p.p241 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p241), 1.0)
            }
        }, p.p240));

        s.store_ad(34, &A::scale({
            if ((1.0 + (p.p177 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p177), 1.0)
            }
        }, p.p176));

        s.store_ad(37, &A::scale({
            if ((1.0 + (p.p199 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p199), 1.0)
            }
        }, p.p198));

        s.store_ad(40, &A::scale({
            if ((1.0 + (p.p221 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p221), 1.0)
            }
        }, p.p220));

        s.store_ad(43, &A::scale({
            if ((1.0 + (p.p243 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::offset(A::scale(A::offset(s.ad_value(111), (-s.v[109])), p.p243), 1.0)
            }
        }, p.p242));

        s.store_ad(44, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(9)), p.p6));

        s.store_ad(45, &A::scale(A::voltage(ctx, &nodes, Some(8), Some(9)), p.p6));

        s.v[306] = if (p.p52 == 0.0) { 1.0 } else { 0.0 };

        s.v[307] = if ((p.p6 * (nv19 - nv0)) <= (p.p6 * (nv19 - nv2))) { 1.0 } else { 0.0 };

        if ((s.v[306] != 0.0) && (s.v[307] != 0.0)) {
            s.store_ad(48, &A::scale(A::voltage(ctx, &nodes, Some(19), Some(2)), p.p6));
        }

        if ((s.v[306] != 0.0) && (!(s.v[307] != 0.0))) {
            s.store_ad(48, &A::scale(A::voltage(ctx, &nodes, Some(19), Some(0)), p.p6));
        }

        if (!(s.v[306] != 0.0)) {
            let assign770_ad_e3265: A = {
                if (!(p.p52 == 0.0)) {
                    let assign770_ad_e3230: A = A::add(A::add(A::scale(A::voltage(ctx, &nodes, Some(19), Some(0)), p.p6), A::scale(A::voltage(ctx, &nodes, Some(19), Some(2)), p.p6)), A::mul(A::sub(A::scale(A::voltage(ctx, &nodes, Some(19), Some(0)), p.p6), A::scale(A::voltage(ctx, &nodes, Some(19), Some(2)), p.p6)), A::tanh(A::scale(A::sub(A::scale(A::voltage(ctx, &nodes, Some(19), Some(0)), p.p6), A::scale(A::voltage(ctx, &nodes, Some(19), Some(2)), p.p6)), (0.001 / p.p53)))));
                    A::scale(assign770_ad_e3230, 0.5)
                } else {
                    let assign770_ad_e3264: A = {
                        if (p.p52 == 0.0) {
                            let assign770_ad_e3261: A = A::add(A::add(A::scale(A::voltage(ctx, &nodes, Some(19), Some(0)), p.p6), A::scale(A::voltage(ctx, &nodes, Some(19), Some(2)), p.p6)), A::sqrt(A::offset(A::mul(A::sub(A::scale(A::voltage(ctx, &nodes, Some(19), Some(0)), p.p6), A::scale(A::voltage(ctx, &nodes, Some(19), Some(2)), p.p6)), A::sub(A::scale(A::voltage(ctx, &nodes, Some(19), Some(0)), p.p6), A::scale(A::voltage(ctx, &nodes, Some(19), Some(2)), p.p6))), p.p53)));
                            A::scale(assign770_ad_e3261, 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    };
                    assign770_ad_e3264
                }
            };
            s.store_ad(48, &assign770_ad_e3265);
        }

        s.v[46] = (p.p55 + (1.0 / ((p.p29 * p.p56) * p.p33)));

        s.store_ad(53, &A::scale(A::voltage(ctx, &nodes, Some(13), Some(19)), p.p6));

        s.store_sub_from_scalar(52, s.v[46], 48);

        s.v[222] = 0.0;

        s.v[221] = 0.0;

        s.v[220] = 1.0;

        s.v[224] = 0.0;

        s.v[226] = 0.0;

        s.v[225] = 0.0;

        s.v[227] = 0.0;

        s.v[228] = 0.0;

        s.v[229] = 0.0;

        s.v[230] = 1.0;

        s.v[308] = if (p.p328 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[308] != 0.0) {
            let assign920_ad_e3395: A = {
                if ((!(((((nv0 - nv1) - p.p331) - (nv21 * p.p335)) / p.p334) > 50.0)) && (!(((((nv0 - nv1) - p.p331) - (nv21 * p.p335)) / p.p334) < (-50.0)))) {
                    A::exp(A::scale(A::sub(A::offset(A::voltage(ctx, &nodes, Some(0), Some(1)), (-p.p331)), A::scale(A::voltage(ctx, &nodes, Some(21), None), p.p335)), 1.0 / (p.p334)))
                } else {
                    let assign920_ad_e3394: A = {
                        if ((!(((((nv0 - nv1) - p.p331) - (nv21 * p.p335)) / p.p334) > 50.0)) && (((((nv0 - nv1) - p.p331) - (nv21 * p.p335)) / p.p334) < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            let assign920_ad_e3393: A = {
                                if (((((nv0 - nv1) - p.p331) - (nv21 * p.p335)) / p.p334) > 50.0) {
                                    A::scale(A::offset(A::offset(A::scale(A::sub(A::offset(A::voltage(ctx, &nodes, Some(0), Some(1)), (-p.p331)), A::scale(A::voltage(ctx, &nodes, Some(21), None), p.p335)), 1.0 / (p.p334)), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            };
                            assign920_ad_e3393
                        }
                    };
                    assign920_ad_e3394
                }
            };
            s.store_add_ad(222, A::scale(A::abs(A::voltage(ctx, &nodes, Some(0), Some(1))), p.p333), assign920_ad_e3395);
        }

        if (s.v[308] != 0.0) {
            s.store_ad(221, &A::voltage(ctx, &nodes, Some(20), None));
        }

        if (s.v[308] != 0.0) {
            s.store_offset_ad(220, A::mul(s.ad_value(221), s.ad_value(223)), 1.0);
        }

        s.v[309] = if (p.p328 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[308] != 0.0)) && (s.v[309] != 0.0)) {
            s.store_ad(224, &A::voltage(ctx, &nodes, Some(22), None));
        }

        if ((!(s.v[308] != 0.0)) && (s.v[309] != 0.0)) {
            s.store_ad(225, &A::voltage(ctx, &nodes, Some(23), None));
        }

        if ((!(s.v[308] != 0.0)) && (s.v[309] != 0.0)) {
            s.store_scale_ad(228, A::abs(A::sub(s.ad_value(225), s.ad_value(224))), 1.0 / (p.p338));
        }

        if ((!(s.v[308] != 0.0)) && (s.v[309] != 0.0)) {
            s.store_ad(226, &A::voltage(ctx, &nodes, Some(25), None));
        }

        if ((!(s.v[308] != 0.0)) && (s.v[309] != 0.0)) {
            s.store_ad(227, &A::voltage(ctx, &nodes, Some(26), None));
        }

        if ((!(s.v[308] != 0.0)) && (s.v[309] != 0.0)) {
            s.store_scale_ad(229, A::abs(A::sub(s.ad_value(227), s.ad_value(226))), 1.0 / (p.p337));
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv17 = ctx.node_voltage(nodes[17]);
        if ((!(s.v[308] != 0.0)) && (s.v[309] != 0.0)) {
            s.store_div_from_scalar_ad(230, 1.0, A::add(A::offset(s.ad_value(228), 1.0), s.ad_value(229)));
        }

        s.v[310] = if (p.p52 == 0.0) { 1.0 } else { 0.0 };

        s.v[311] = if ((p.p6 * (nv17 - nv0)) <= (p.p6 * (nv17 - nv2))) { 1.0 } else { 0.0 };

        if ((s.v[310] != 0.0) && (s.v[311] != 0.0)) {
            s.store_ad(49, &A::scale(A::voltage(ctx, &nodes, Some(17), Some(2)), p.p6));
        }

        if ((s.v[310] != 0.0) && (!(s.v[311] != 0.0))) {
            s.store_ad(49, &A::scale(A::voltage(ctx, &nodes, Some(17), Some(0)), p.p6));
        }

        if (!(s.v[310] != 0.0)) {
            let assign1070_ad_e3575: A = {
                if (!(p.p52 == 0.0)) {
                    let assign1070_ad_e3540: A = A::add(A::add(A::scale(A::voltage(ctx, &nodes, Some(17), Some(0)), p.p6), A::scale(A::voltage(ctx, &nodes, Some(17), Some(2)), p.p6)), A::mul(A::sub(A::scale(A::voltage(ctx, &nodes, Some(17), Some(0)), p.p6), A::scale(A::voltage(ctx, &nodes, Some(17), Some(2)), p.p6)), A::tanh(A::scale(A::sub(A::scale(A::voltage(ctx, &nodes, Some(17), Some(0)), p.p6), A::scale(A::voltage(ctx, &nodes, Some(17), Some(2)), p.p6)), (0.001 / p.p53)))));
                    A::scale(assign1070_ad_e3540, 0.5)
                } else {
                    let assign1070_ad_e3574: A = {
                        if (p.p52 == 0.0) {
                            let assign1070_ad_e3571: A = A::add(A::add(A::scale(A::voltage(ctx, &nodes, Some(17), Some(0)), p.p6), A::scale(A::voltage(ctx, &nodes, Some(17), Some(2)), p.p6)), A::sqrt(A::offset(A::mul(A::sub(A::scale(A::voltage(ctx, &nodes, Some(17), Some(0)), p.p6), A::scale(A::voltage(ctx, &nodes, Some(17), Some(2)), p.p6)), A::sub(A::scale(A::voltage(ctx, &nodes, Some(17), Some(0)), p.p6), A::scale(A::voltage(ctx, &nodes, Some(17), Some(2)), p.p6))), p.p53)));
                            A::scale(assign1070_ad_e3571, 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    };
                    assign1070_ad_e3574
                }
            };
            s.store_ad(49, &assign1070_ad_e3575);
        }

        s.store_offset_ad(47, A::div_from_scalar(1.0, A::scale(s.ad_value(220), (p.p29 * (p.p68 * p.p33)))), p.p67);

        s.store_ad(57, &A::scale(A::voltage(ctx, &nodes, Some(18), Some(17)), p.p6));

        s.store_sub(56, 47, 49);

        s.v[312] = if (p.p78 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[312] != 0.0) {
            s.store_ad(60, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(10)), p.p6));
        }

        if (s.v[312] != 0.0) {
            s.store_ad(62, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(10)), p.p6));
        }

        if (!(s.v[312] != 0.0)) {
            s.store_ad(60, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(10)), p.p6));
        }

        if (!(s.v[312] != 0.0)) {
            s.store_ad(62, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(10)), p.p6));
        }

        s.store_ad(61, &A::scale(A::voltage(ctx, &nodes, Some(9), Some(10)), p.p6));

        s.store_ad(63, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(10)), p.p6));

        s.v[313] = if (p.p100 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[313] != 0.0) {
            s.store_ad(66, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(11)), p.p6));
        }

        if (s.v[313] != 0.0) {
            s.store_ad(68, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(11)), p.p6));
        }

        if (!(s.v[313] != 0.0)) {
            s.store_ad(66, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(11)), p.p6));
        }

        if (!(s.v[313] != 0.0)) {
            s.store_ad(68, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(11)), p.p6));
        }

        s.store_ad(67, &A::scale(A::voltage(ctx, &nodes, Some(10), Some(11)), p.p6));

        s.store_ad(69, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(11)), p.p6));

        s.v[314] = if (p.p122 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[314] != 0.0) {
            s.store_ad(72, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(12)), p.p6));
        }

        if (s.v[314] != 0.0) {
            s.store_ad(74, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(12)), p.p6));
        }

        if (!(s.v[314] != 0.0)) {
            s.store_ad(72, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(12)), p.p6));
        }

        if (!(s.v[314] != 0.0)) {
            s.store_ad(74, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(12)), p.p6));
        }

        s.store_ad(73, &A::scale(A::voltage(ctx, &nodes, Some(11), Some(12)), p.p6));

        s.store_ad(75, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(12)), p.p6));

        s.v[315] = if (p.p144 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[315] != 0.0) {
            s.store_ad(78, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(13)), p.p6));
        }

        if (s.v[315] != 0.0) {
            s.store_ad(80, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(13)), p.p6));
        }

        if (!(s.v[315] != 0.0)) {
            s.store_ad(78, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(13)), p.p6));
        }

        if (!(s.v[315] != 0.0)) {
            s.store_ad(80, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(13)), p.p6));
        }

        s.store_ad(79, &A::scale(A::voltage(ctx, &nodes, Some(12), Some(13)), p.p6));

        s.store_ad(81, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(13)), p.p6));

        s.v[316] = if (p.p166 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[316] != 0.0) {
            s.store_ad(84, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(5)), p.p6));
        }

        if (s.v[316] != 0.0) {
            s.store_ad(86, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(5)), p.p6));
        }

        if (!(s.v[316] != 0.0)) {
            s.store_ad(84, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(5)), p.p6));
        }

        if (!(s.v[316] != 0.0)) {
            s.store_ad(86, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(5)), p.p6));
        }

        s.store_ad(85, &A::scale(A::voltage(ctx, &nodes, Some(14), Some(5)), p.p6));

        s.store_ad(87, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(5)), p.p6));

        s.v[317] = if (p.p188 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[317] != 0.0) {
            s.store_ad(90, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(14)), p.p6));
        }

        if (s.v[317] != 0.0) {
            s.store_ad(92, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(14)), p.p6));
        }

        if (!(s.v[317] != 0.0)) {
            s.store_ad(90, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(14)), p.p6));
        }

        if (!(s.v[317] != 0.0)) {
            s.store_ad(92, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(14)), p.p6));
        }

        s.store_ad(91, &A::scale(A::voltage(ctx, &nodes, Some(15), Some(14)), p.p6));

        s.store_ad(93, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(14)), p.p6));

        s.v[318] = if (p.p210 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[318] != 0.0) {
            s.store_ad(96, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(15)), p.p6));
        }

        if (s.v[318] != 0.0) {
            s.store_ad(98, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(15)), p.p6));
        }

        if (!(s.v[318] != 0.0)) {
            s.store_ad(96, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(15)), p.p6));
        }

        if (!(s.v[318] != 0.0)) {
            s.store_ad(98, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(15)), p.p6));
        }

        s.store_ad(97, &A::scale(A::voltage(ctx, &nodes, Some(16), Some(15)), p.p6));

        s.store_ad(99, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(15)), p.p6));

        s.v[319] = if (p.p232 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[319] != 0.0) {
            s.store_ad(102, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(16)), p.p6));
        }

        if (s.v[319] != 0.0) {
            s.store_ad(104, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(16)), p.p6));
        }

        if (!(s.v[319] != 0.0)) {
            s.store_ad(102, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(16)), p.p6));
        }

        if (!(s.v[319] != 0.0)) {
            s.store_ad(104, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(16)), p.p6));
        }

        s.store_ad(103, &A::scale(A::voltage(ctx, &nodes, Some(17), Some(16)), p.p6));

        s.store_ad(105, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(16)), p.p6));

        s.v[208] = 0.0;

        s.v[209] = 0.0;

        s.v[210] = 0.0;

        s.v[211] = 0.0;

        s.v[212] = 0.0;

        s.v[213] = 0.0;

        s.v[320] = if (p.p233 > p.p354) { 1.0 } else { 0.0 };

        if (s.v[320] != 0.0) {
            s.store_scalar(321, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(322, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(323, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(324, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(325, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(326, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(327, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(328, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(329, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.copy_ad(330, 102);
        }

        if (s.v[320] != 0.0) {
            s.copy_ad(331, 103);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(332, p.p239);
        }

        if (s.v[320] != 0.0) {
            s.copy_ad(333, 104);
        }

        if (s.v[320] != 0.0) {
            s.copy_ad(334, 105);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(335, p.p237);
        }

        if (s.v[320] != 0.0) {
            s.copy_ad(336, 111);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(337, s.v[109]);
        }

        if (s.v[320] != 0.0) {
            s.copy_ad(338, 113);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(339, p.p0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(340, p.p233);
        }

        if (s.v[320] != 0.0) {
            s.copy_ad(341, 41);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(342, p.p238);
        }

        if (s.v[320] != 0.0) {
            s.copy_ad(343, 42);
        }

        if (s.v[320] != 0.0) {
            s.copy_ad(344, 43);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(345, p.p234);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(346, p.p248);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(347, p.p247);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(348, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(349, p.p249);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(350, p.p253);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(351, p.p244);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(352, p.p245);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(353, p.p246);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(354, p.p252);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(355, p.p251);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(356, p.p250);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(357, p.p39);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(358, p.p47);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(359, p.p45);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(360, p.p42);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(361, p.p2);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(362, p.p6);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(363, 1.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(364, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(365, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(366, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(367, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(368, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(369, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(370, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(371, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(372, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(373, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(374, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(375, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(376, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(377, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(378, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(379, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(380, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(381, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(382, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(383, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(384, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(385, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(386, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(387, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(388, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(389, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(390, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_2(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[320] != 0.0) {
            s.store_scalar(391, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(392, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(393, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(394, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(395, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(396, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(397, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(398, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(399, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(400, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(401, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(402, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(403, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(404, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(405, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(406, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(407, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(408, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(409, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(410, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(411, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(412, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(413, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(414, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(415, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(416, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(417, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(418, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(419, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(420, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(421, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(422, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(423, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(424, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(425, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(426, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(427, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(428, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(429, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(430, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(431, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_scalar(432, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_ad(429, &{
                if (!(p.p52 == 0.0)) {
                    A::mul(s.ad_value(331), A::tanh(A::scale(s.ad_value(331), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(331)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.v[320] != 0.0) {
            s.store_sub(430, 330, 331);
        }

        if (s.v[320] != 0.0) {
            s.store_mul(364, 350, 338);
        }

        if (s.v[320] != 0.0) {
            s.store_add_ad(366, A::div(s.ad_value(346), A::scale(s.ad_value(338), 2.302585092994046)), A::mul(s.ad_value(349), s.ad_value(429)));
        }

        if (s.v[320] != 0.0) {
            s.store_add_ad_rhs(367, 345, A::mul(s.ad_value(356), A::sub(s.ad_value(336), s.ad_value(337))));
        }

        if (s.v[320] != 0.0) {
            s.store_ad(385, &A::pow(A::div(s.ad_value(336), s.ad_value(337)), s.ad_value(358)));
        }

        s.v[433] = if (s.v[357] != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[320] != 0.0) && (s.v[433] != 0.0)) {
            s.store_div_ad_rhs(368, 429, A::pow(A::offset(A::pow(A::div(s.ad_value(429), s.ad_value(357)), s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353))));
        }

        if ((s.v[320] != 0.0) && (!(s.v[433] != 0.0))) {
            s.store_scalar(368, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.store_mul_ad_lhs(365, A::sub(s.ad_value(347), A::mul(s.ad_value(368), s.ad_value(348))), 429);
        }

        if (s.v[320] != 0.0) {
            s.store_sub(328, 367, 365);
        }

        if (s.v[320] != 0.0) {
            s.store_mul_ad_lhs(370, A::scale(s.ad_value(366), 2.0), 338);
        }

        if (s.v[320] != 0.0) {
            s.store_mul(371, 341, 370);
        }

        if (s.v[320] != 0.0) {
            s.store_sub_ad_rhs(428, 328, A::scale(s.ad_value(364), (p.p51 * 0.5)));
        }

        if (s.v[320] != 0.0) {
            let assign3020_ad_e4515: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(330), s.ad_value(430)), A::mul(A::sub(s.ad_value(330), s.ad_value(430)), A::tanh(A::scale(A::sub(s.ad_value(330), s.ad_value(430)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(330), s.ad_value(430)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(330), s.ad_value(430)), A::sub(s.ad_value(330), s.ad_value(430))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(427, A::sub(assign3020_ad_e4515, s.ad_value(428)), 364);
        }

        s.v[434] = if (s.v[427] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[320] != 0.0) && (s.v[434] != 0.0)) {
            s.store_scalar(386, 0.0);
        }

        s.v[435] = if (s.v[427] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[320] != 0.0) && (!(s.v[434] != 0.0))) && (s.v[435] != 0.0)) {
            s.store_scalar(386, 1.0);
        }

        if (((s.v[320] != 0.0) && (!(s.v[434] != 0.0))) && (!(s.v[435] != 0.0))) {
            s.store_div_from_scalar_ad(386, 1.0, A::offset(A::exp(s.ad_value(427)), 1.0));
        }

        if (s.v[320] != 0.0) {
            let assign3080_ad_e4603: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(330), s.ad_value(430)), A::mul(A::sub(s.ad_value(330), s.ad_value(430)), A::tanh(A::scale(A::sub(s.ad_value(330), s.ad_value(430)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(330), s.ad_value(430)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(330), s.ad_value(430)), A::sub(s.ad_value(330), s.ad_value(430))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(387, A::sub(assign3080_ad_e4603, A::sub(s.ad_value(328), A::mul(A::scale(s.ad_value(364), (p.p51 * 0.1)), s.ad_value(386)))), 370);
        }

        s.v[436] = if (s.v[387] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[320] != 0.0) && (s.v[436] != 0.0)) {
            s.store_mul(388, 371, 387);
        }

        s.v[437] = if (s.v[387] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[320] != 0.0) && (!(s.v[436] != 0.0))) && (s.v[437] != 0.0)) {
            s.store_mul_ad_rhs(388, 371, A::exp(s.ad_value(387)));
        }

        if (((s.v[320] != 0.0) && (!(s.v[436] != 0.0))) && (!(s.v[437] != 0.0))) {
            s.store_mul_ad_rhs(388, 371, A::ln(A::offset(A::exp(s.ad_value(387)), 1.0)));
        }

        if (s.v[320] != 0.0) {
            s.store_div_ad_rhs(374, 352, A::mul(s.ad_value(385), A::offset(A::div(A::mul(s.ad_value(354), s.ad_value(388)), s.ad_value(341)), 1.0)));
        }

        if (s.v[320] != 0.0) {
            s.store_div_ad(375, A::mul(A::mul(s.ad_value(351), A::div(A::offset(A::mul(s.ad_value(359), s.ad_value(337)), 1.0), A::offset(A::mul(s.ad_value(359), s.ad_value(336)), 1.0))), A::offset(A::div(A::mul(s.ad_value(360), s.ad_value(429)), s.ad_value(340)), 1.0)), A::offset(A::div(A::mul(s.ad_value(355), s.ad_value(388)), s.ad_value(341)), 1.0));
        }

        if (s.v[320] != 0.0) {
            s.store_add_ad(376, A::div(A::mul(A::mul(A::scale(s.ad_value(386), 2.0), s.ad_value(338)), s.ad_value(374)), s.ad_value(340)), A::mul(A::sub_from_scalar(1.0, s.ad_value(386)), s.ad_value(375)));
        }

        if (s.v[320] != 0.0) {
            s.store_div_ad_lhs(392, A::mul(s.ad_value(375), s.ad_value(340)), 374);
        }

        if (s.v[320] != 0.0) {
            s.store_sub_ad_lhs(393, A::mul(s.ad_value(392), A::sqrt(A::offset(A::div(A::div(A::scale(s.ad_value(388), 2.0), s.ad_value(341)), s.ad_value(392)), 1.0))), 392);
        }

        if (s.v[320] != 0.0) {
            s.store_add_ad(394, A::mul(s.ad_value(392), A::sub_from_scalar(1.0, s.ad_value(386))), A::mul(s.ad_value(370), s.ad_value(386)));
        }

        if (s.v[320] != 0.0) {
            s.store_add_ad(329, A::mul(s.ad_value(393), A::sub_from_scalar(1.0, s.ad_value(386))), A::mul(s.ad_value(370), s.ad_value(386)));
        }

        if (s.v[320] != 0.0) {
            let assign3210_ad_e4832: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(s.ad_value(331), s.ad_value(329)), A::mul(A::neg(A::div(s.ad_value(331), s.ad_value(329))), A::tanh(A::scale(A::neg(A::div(s.ad_value(331), s.ad_value(329))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(s.ad_value(331), s.ad_value(329)), A::sqrt(A::offset(A::mul(A::neg(A::div(s.ad_value(331), s.ad_value(329))), A::neg(A::div(s.ad_value(331), s.ad_value(329)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(395, 1.0, A::pow(A::offset(A::pow(assign3210_ad_e4832, s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353))));
        }

        if (s.v[320] != 0.0) {
            s.store_mul(396, 331, 395);
        }

        if (s.v[320] != 0.0) {
            let assign3230_ad_e4913: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(A::neg(s.ad_value(331)), s.ad_value(329)), A::mul(A::neg(A::div(A::neg(s.ad_value(331)), s.ad_value(329))), A::tanh(A::scale(A::neg(A::div(A::neg(s.ad_value(331)), s.ad_value(329))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(A::neg(s.ad_value(331)), s.ad_value(329)), A::sqrt(A::offset(A::mul(A::neg(A::div(A::neg(s.ad_value(331)), s.ad_value(329))), A::neg(A::div(A::neg(s.ad_value(331)), s.ad_value(329)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(397, 1.0, A::pow(A::offset(A::pow(assign3230_ad_e4913, s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353))));
        }

        if (s.v[320] != 0.0) {
            s.store_mul_ad_lhs(398, A::neg(s.ad_value(331)), 397);
        }

        if (s.v[320] != 0.0) {
            s.store_div_ad_lhs(427, A::sub(s.ad_value(330), s.ad_value(428)), 364);
        }

        s.v[438] = if (s.v[427] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[320] != 0.0) && (s.v[438] != 0.0)) {
            s.store_scalar(369, 0.0);
        }

        s.v[439] = if (s.v[427] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[320] != 0.0) && (!(s.v[438] != 0.0))) && (s.v[439] != 0.0)) {
            s.store_scalar(369, 1.0);
        }

        if (((s.v[320] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[439] != 0.0))) {
            s.store_div_from_scalar_ad(369, 1.0, A::offset(A::exp(s.ad_value(427)), 1.0));
        }

        if (s.v[320] != 0.0) {
            s.store_div_ad_lhs(372, A::sub(A::sub(s.ad_value(430), s.ad_value(398)), A::sub(s.ad_value(328), A::mul(A::scale(s.ad_value(364), (p.p51 * 0.1)), s.ad_value(369)))), 370);
        }

        s.v[440] = if (s.v[372] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[320] != 0.0) && (s.v[440] != 0.0)) {
            s.store_mul(373, 371, 372);
        }

        s.v[441] = if (s.v[372] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[320] != 0.0) && (!(s.v[440] != 0.0))) && (s.v[441] != 0.0)) {
            s.store_mul_ad_rhs(373, 371, A::exp(s.ad_value(372)));
        }

        if (((s.v[320] != 0.0) && (!(s.v[440] != 0.0))) && (!(s.v[441] != 0.0))) {
            s.store_mul_ad_rhs(373, 371, A::ln(A::offset(A::exp(s.ad_value(372)), 1.0)));
        }

        if (s.v[320] != 0.0) {
            s.store_div_ad_lhs(427, A::sub(s.ad_value(430), s.ad_value(428)), 364);
        }

        s.v[442] = if (s.v[427] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[320] != 0.0) && (s.v[442] != 0.0)) {
            s.store_scalar(399, 0.0);
        }

        s.v[443] = if (s.v[427] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[320] != 0.0) && (!(s.v[442] != 0.0))) && (s.v[443] != 0.0)) {
            s.store_scalar(399, 1.0);
        }

        if (((s.v[320] != 0.0) && (!(s.v[442] != 0.0))) && (!(s.v[443] != 0.0))) {
            s.store_div_from_scalar_ad(399, 1.0, A::offset(A::exp(s.ad_value(427)), 1.0));
        }

        if (s.v[320] != 0.0) {
            s.store_div_ad_lhs(400, A::sub(A::sub(s.ad_value(330), s.ad_value(396)), A::sub(s.ad_value(328), A::mul(A::scale(s.ad_value(364), (p.p51 * 0.1)), s.ad_value(399)))), 370);
        }

        s.v[444] = if (s.v[400] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[320] != 0.0) && (s.v[444] != 0.0)) {
            s.store_mul(401, 371, 400);
        }

        s.v[445] = if (s.v[400] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[320] != 0.0) && (!(s.v[444] != 0.0))) && (s.v[445] != 0.0)) {
            s.store_mul_ad_rhs(401, 371, A::exp(s.ad_value(400)));
        }

        if (((s.v[320] != 0.0) && (!(s.v[444] != 0.0))) && (!(s.v[445] != 0.0))) {
            s.store_mul_ad_rhs(401, 371, A::ln(A::offset(A::exp(s.ad_value(400)), 1.0)));
        }

        if (s.v[320] != 0.0) {
            s.store_div_ad_lhs(402, A::sub(s.ad_value(373), s.ad_value(401)), 341);
        }

        if (s.v[320] != 0.0) {
            s.store_div(428, 402, 394);
        }

        if (s.v[320] != 0.0) {
            let assign3510_ad_e5190: A = A::pow(A::offset(A::pow({
                if (!(p.p52 == 0.0)) {
                    A::mul(s.ad_value(428), A::tanh(A::scale(s.ad_value(428), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(428)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353)));
            s.store_div_ad_rhs(403, 428, assign3510_ad_e5190);
        }

        if (s.v[320] != 0.0) {
            s.store_mul(404, 376, 403);
        }

        if (s.v[320] != 0.0) {
            s.store_mul_ad_lhs(322, A::mul(A::mul(A::scale(A::mul(A::mul(s.ad_value(362), s.ad_value(339)), s.ad_value(361)), 0.5), A::add(s.ad_value(373), s.ad_value(401))), s.ad_value(404)), 363);
        }

        if (s.v[320] != 0.0) {
            s.store_div_ad_rhs(377, 346, A::scale(s.ad_value(338), 2.302585092994046));
        }

        if (s.v[320] != 0.0) {
            s.store_mul_ad_lhs(379, A::scale(s.ad_value(377), 2.0), 338);
        }

        if (s.v[320] != 0.0) {
            s.store_mul(380, 341, 379);
        }

        if (s.v[320] != 0.0) {
            s.store_sub_ad_rhs(432, 367, A::scale(s.ad_value(364), (p.p51 * 0.5)));
        }

    }

    pub(super) fn stamp_transient_block_3(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[320] != 0.0) {
            let assign3580_ad_e5294: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(330), s.ad_value(430)), A::mul(A::sub(s.ad_value(330), s.ad_value(430)), A::tanh(A::scale(A::sub(s.ad_value(330), s.ad_value(430)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(330), s.ad_value(430)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(330), s.ad_value(430)), A::sub(s.ad_value(330), s.ad_value(430))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(431, A::sub(assign3580_ad_e5294, s.ad_value(432)), 364);
        }

        s.v[446] = if (s.v[431] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[320] != 0.0) && (s.v[446] != 0.0)) {
            s.store_scalar(389, 0.0);
        }

        s.v[447] = if (s.v[431] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[320] != 0.0) && (!(s.v[446] != 0.0))) && (s.v[447] != 0.0)) {
            s.store_scalar(389, 1.0);
        }

        if (((s.v[320] != 0.0) && (!(s.v[446] != 0.0))) && (!(s.v[447] != 0.0))) {
            s.store_div_from_scalar_ad(389, 1.0, A::offset(A::exp(s.ad_value(431)), 1.0));
        }

        if (s.v[320] != 0.0) {
            let assign3640_ad_e5382: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(330), s.ad_value(430)), A::mul(A::sub(s.ad_value(330), s.ad_value(430)), A::tanh(A::scale(A::sub(s.ad_value(330), s.ad_value(430)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(330), s.ad_value(430)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(330), s.ad_value(430)), A::sub(s.ad_value(330), s.ad_value(430))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(390, A::sub(assign3640_ad_e5382, A::sub(s.ad_value(367), A::mul(A::scale(s.ad_value(364), (p.p51 * 0.1)), s.ad_value(389)))), 379);
        }

        s.v[448] = if (s.v[390] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[320] != 0.0) && (s.v[448] != 0.0)) {
            s.store_mul(391, 380, 390);
        }

        s.v[449] = if (s.v[390] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[320] != 0.0) && (!(s.v[448] != 0.0))) && (s.v[449] != 0.0)) {
            s.store_mul_ad_rhs(391, 380, A::exp(s.ad_value(390)));
        }

        if (((s.v[320] != 0.0) && (!(s.v[448] != 0.0))) && (!(s.v[449] != 0.0))) {
            s.store_mul_ad_rhs(391, 380, A::ln(A::offset(A::exp(s.ad_value(390)), 1.0)));
        }

        if (s.v[320] != 0.0) {
            s.store_div(383, 352, 385);
        }

        if (s.v[320] != 0.0) {
            s.store_mul_ad_rhs(384, 351, A::div(A::offset(A::mul(s.ad_value(359), s.ad_value(337)), 1.0), A::offset(A::mul(s.ad_value(359), s.ad_value(336)), 1.0)));
        }

        if (s.v[320] != 0.0) {
            s.store_div_ad_lhs(405, A::mul(s.ad_value(384), s.ad_value(340)), 383);
        }

        if (s.v[320] != 0.0) {
            s.store_sub_ad_lhs(406, A::mul(s.ad_value(405), A::sqrt(A::offset(A::div(A::div(A::scale(s.ad_value(391), 2.0), s.ad_value(341)), s.ad_value(405)), 1.0))), 405);
        }

        if (s.v[320] != 0.0) {
            s.store_add_ad(407, A::mul(s.ad_value(406), A::sub_from_scalar(1.0, s.ad_value(389))), A::mul(s.ad_value(379), s.ad_value(389)));
        }

        if (s.v[320] != 0.0) {
            let assign3750_ad_e5557: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(s.ad_value(331), s.ad_value(407)), A::mul(A::neg(A::div(s.ad_value(331), s.ad_value(407))), A::tanh(A::scale(A::neg(A::div(s.ad_value(331), s.ad_value(407))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(s.ad_value(331), s.ad_value(407)), A::sqrt(A::offset(A::mul(A::neg(A::div(s.ad_value(331), s.ad_value(407))), A::neg(A::div(s.ad_value(331), s.ad_value(407)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(408, 1.0, A::pow(A::offset(A::pow(assign3750_ad_e5557, s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353))));
        }

        if (s.v[320] != 0.0) {
            s.store_mul(409, 331, 408);
        }

        if (s.v[320] != 0.0) {
            let assign3770_ad_e5638: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(A::neg(s.ad_value(331)), s.ad_value(407)), A::mul(A::neg(A::div(A::neg(s.ad_value(331)), s.ad_value(407))), A::tanh(A::scale(A::neg(A::div(A::neg(s.ad_value(331)), s.ad_value(407))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(A::neg(s.ad_value(331)), s.ad_value(407)), A::sqrt(A::offset(A::mul(A::neg(A::div(A::neg(s.ad_value(331)), s.ad_value(407))), A::neg(A::div(A::neg(s.ad_value(331)), s.ad_value(407)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(410, 1.0, A::pow(A::offset(A::pow(assign3770_ad_e5638, s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353))));
        }

        if (s.v[320] != 0.0) {
            s.store_mul_ad_lhs(411, A::neg(s.ad_value(331)), 410);
        }

        if (s.v[320] != 0.0) {
            s.store_div_ad_lhs(431, A::sub(s.ad_value(330), s.ad_value(432)), 364);
        }

        s.v[450] = if (s.v[431] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[320] != 0.0) && (s.v[450] != 0.0)) {
            s.store_scalar(378, 0.0);
        }

        s.v[451] = if (s.v[431] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[320] != 0.0) && (!(s.v[450] != 0.0))) && (s.v[451] != 0.0)) {
            s.store_scalar(378, 1.0);
        }

        if (((s.v[320] != 0.0) && (!(s.v[450] != 0.0))) && (!(s.v[451] != 0.0))) {
            s.store_div_from_scalar_ad(378, 1.0, A::offset(A::exp(s.ad_value(431)), 1.0));
        }

        if (s.v[320] != 0.0) {
            s.store_div_ad_lhs(381, A::sub(A::sub(s.ad_value(430), s.ad_value(411)), A::sub(s.ad_value(367), A::mul(A::scale(s.ad_value(364), (p.p51 * 0.1)), s.ad_value(378)))), 379);
        }

        s.v[452] = if (s.v[381] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[320] != 0.0) && (s.v[452] != 0.0)) {
            s.store_mul(382, 380, 381);
        }

        s.v[453] = if (s.v[381] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[320] != 0.0) && (!(s.v[452] != 0.0))) && (s.v[453] != 0.0)) {
            s.store_mul_ad_rhs(382, 380, A::exp(s.ad_value(381)));
        }

        if (((s.v[320] != 0.0) && (!(s.v[452] != 0.0))) && (!(s.v[453] != 0.0))) {
            s.store_mul_ad_rhs(382, 380, A::ln(A::offset(A::exp(s.ad_value(381)), 1.0)));
        }

        if (s.v[320] != 0.0) {
            s.store_div_ad_lhs(431, A::sub(s.ad_value(430), s.ad_value(432)), 364);
        }

        s.v[454] = if (s.v[431] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[320] != 0.0) && (s.v[454] != 0.0)) {
            s.store_scalar(412, 0.0);
        }

        s.v[455] = if (s.v[431] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[320] != 0.0) && (!(s.v[454] != 0.0))) && (s.v[455] != 0.0)) {
            s.store_scalar(412, 1.0);
        }

        if (((s.v[320] != 0.0) && (!(s.v[454] != 0.0))) && (!(s.v[455] != 0.0))) {
            s.store_div_from_scalar_ad(412, 1.0, A::offset(A::exp(s.ad_value(431)), 1.0));
        }

        if (s.v[320] != 0.0) {
            s.store_div_ad_lhs(413, A::sub(A::sub(s.ad_value(330), s.ad_value(409)), A::sub(s.ad_value(367), A::mul(A::scale(s.ad_value(364), (p.p51 * 0.1)), s.ad_value(412)))), 379);
        }

        s.v[456] = if (s.v[413] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[320] != 0.0) && (s.v[456] != 0.0)) {
            s.store_mul(414, 380, 413);
        }

        s.v[457] = if (s.v[413] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[320] != 0.0) && (!(s.v[456] != 0.0))) && (s.v[457] != 0.0)) {
            s.store_mul_ad_rhs(414, 380, A::exp(s.ad_value(413)));
        }

        if (((s.v[320] != 0.0) && (!(s.v[456] != 0.0))) && (!(s.v[457] != 0.0))) {
            s.store_mul_ad_rhs(414, 380, A::ln(A::offset(A::exp(s.ad_value(413)), 1.0)));
        }

        if (s.v[320] != 0.0) {
            s.store_offset_ad(415, A::square(s.ad_value(382)), 1e-38);
        }

        if (s.v[320] != 0.0) {
            s.store_offset_ad(416, A::mul(s.ad_value(415), s.ad_value(382)), 1e-57);
        }

        if (s.v[320] != 0.0) {
            s.store_offset_ad(417, A::square(s.ad_value(414)), 1e-38);
        }

        if (s.v[320] != 0.0) {
            s.store_offset_ad(418, A::mul(s.ad_value(417), s.ad_value(414)), 1e-57);
        }

        if (s.v[320] != 0.0) {
            s.store_offset_ad(419, A::mul(s.ad_value(382), s.ad_value(414)), 1e-38);
        }

        if (s.v[320] != 0.0) {
            s.store_div_ad(420, A::scale(A::add(A::add(s.ad_value(415), s.ad_value(417)), s.ad_value(419)), (2.0 / 3.0)), A::offset(A::add(s.ad_value(382), s.ad_value(414)), 2e-19));
        }

        if (s.v[320] != 0.0) {
            s.store_div_ad(421, A::scale(A::add(A::add(A::add(A::scale(s.ad_value(416), 2.0), A::scale(s.ad_value(418), 3.0)), A::mul(A::scale(s.ad_value(415), 4.0), s.ad_value(414))), A::mul(A::scale(s.ad_value(417), 6.0), s.ad_value(382))), 2.0), A::scale(A::add(A::add(s.ad_value(415), s.ad_value(417)), A::scale(s.ad_value(419), 2.0)), 15.0));
        }

        if (s.v[320] != 0.0) {
            s.store_sub(422, 420, 421);
        }

        if (s.v[320] != 0.0) {
            s.copy_ad(423, 421);
        }

        if (s.v[320] != 0.0) {
            s.store_mul_ad_lhs(323, A::mul(A::mul(A::mul(A::mul(s.ad_value(339), s.ad_value(361)), s.ad_value(340)), s.ad_value(362)), s.ad_value(422)), 363);
        }

        if (s.v[320] != 0.0) {
            s.store_mul_ad_lhs(324, A::mul(A::mul(A::mul(A::mul(s.ad_value(339), s.ad_value(361)), s.ad_value(340)), s.ad_value(362)), s.ad_value(423)), 363);
        }

        s.v[458] = if (s.v[332] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[320] != 0.0) && (s.v[458] != 0.0)) {
            s.store_div_ad_lhs(424, A::sub(s.ad_value(333), A::sub(s.ad_value(367), A::scale(s.ad_value(364), (p.p51 * 0.5)))), 379);
        }

        s.v[459] = if (s.v[424] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[320] != 0.0) && (s.v[458] != 0.0)) && (s.v[459] != 0.0)) {
            s.copy_ad(427, 424);
        }

        s.v[460] = if (s.v[424] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[320] != 0.0) && (s.v[458] != 0.0)) && (!(s.v[459] != 0.0))) && (s.v[460] != 0.0)) {
            s.store_exp(427, 424);
        }

        if ((((s.v[320] != 0.0) && (s.v[458] != 0.0)) && (!(s.v[459] != 0.0))) && (!(s.v[460] != 0.0))) {
            s.store_ln_ad(427, A::offset(A::exp(s.ad_value(424)), 1.0));
        }

        if ((s.v[320] != 0.0) && (s.v[458] != 0.0)) {
            s.store_mul_ad_lhs(325, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(339), s.ad_value(361)), s.ad_value(362)), s.ad_value(343)), s.ad_value(379)), s.ad_value(427)), 363);
        }

        if ((s.v[320] != 0.0) && (s.v[458] != 0.0)) {
            s.store_div_ad_lhs(425, A::sub(s.ad_value(334), A::sub(s.ad_value(367), A::scale(s.ad_value(364), (p.p51 * 0.5)))), 379);
        }

        s.v[461] = if (s.v[425] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[320] != 0.0) && (s.v[458] != 0.0)) && (s.v[461] != 0.0)) {
            s.copy_ad(427, 425);
        }

        s.v[462] = if (s.v[425] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[320] != 0.0) && (s.v[458] != 0.0)) && (!(s.v[461] != 0.0))) && (s.v[462] != 0.0)) {
            s.store_exp(427, 425);
        }

        if ((((s.v[320] != 0.0) && (s.v[458] != 0.0)) && (!(s.v[461] != 0.0))) && (!(s.v[462] != 0.0))) {
            s.store_ln_ad(427, A::offset(A::exp(s.ad_value(425)), 1.0));
        }

        if ((s.v[320] != 0.0) && (s.v[458] != 0.0)) {
            s.store_mul_ad_lhs(326, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(339), s.ad_value(361)), s.ad_value(362)), s.ad_value(344)), s.ad_value(379)), s.ad_value(427)), 363);
        }

        if ((s.v[320] != 0.0) && (!(s.v[458] != 0.0))) {
            s.store_scalar(325, 0.0);
        }

        if ((s.v[320] != 0.0) && (!(s.v[458] != 0.0))) {
            s.store_scalar(326, 0.0);
        }

        s.v[463] = if (s.v[335] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[320] != 0.0) && (s.v[463] != 0.0)) {
            s.store_div_ad_lhs(426, A::sub(s.ad_value(330), A::sub(s.ad_value(367), A::scale(s.ad_value(364), (p.p51 * 0.5)))), 379);
        }

        s.v[464] = if (s.v[426] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[320] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) {
            s.copy_ad(427, 426);
        }

        s.v[465] = if (s.v[426] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[320] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[465] != 0.0)) {
            s.store_exp(427, 426);
        }

        if ((((s.v[320] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[465] != 0.0))) {
            s.store_ln_ad(427, A::offset(A::exp(s.ad_value(426)), 1.0));
        }

        if ((s.v[320] != 0.0) && (s.v[463] != 0.0)) {
            s.store_mul_ad_lhs(327, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(339), s.ad_value(361)), s.ad_value(362)), s.ad_value(342)), s.ad_value(379)), s.ad_value(427)), 363);
        }

        if ((s.v[320] != 0.0) && (!(s.v[463] != 0.0))) {
            s.store_scalar(327, 0.0);
        }

        if (s.v[320] != 0.0) {
            s.copy_ad(321, 322);
        }

        if (s.v[320] != 0.0) {
            s.copy_ad(208, 322);
        }

        if (s.v[320] != 0.0) {
            s.copy_ad(209, 323);
        }

        if (s.v[320] != 0.0) {
            s.copy_ad(210, 324);
        }

        if (s.v[320] != 0.0) {
            s.copy_ad(211, 325);
        }

        if (s.v[320] != 0.0) {
            s.copy_ad(212, 326);
        }

        if (s.v[320] != 0.0) {
            s.copy_ad(213, 327);
        }

        if (s.v[320] != 0.0) {
            s.copy_ad(208, 321);
        }

        s.v[466] = if (p.p232 == 1.0) { 1.0 } else { 0.0 };

        s.v[202] = 0.0;

        s.v[203] = 0.0;

        s.v[204] = 0.0;

        s.v[205] = 0.0;

        s.v[206] = 0.0;

        s.v[207] = 0.0;

        s.v[467] = if (p.p211 > p.p354) { 1.0 } else { 0.0 };

        if (s.v[467] != 0.0) {
            s.store_scalar(468, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(469, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(470, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(471, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(472, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(473, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(474, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(475, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(476, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.copy_ad(477, 96);
        }

        if (s.v[467] != 0.0) {
            s.copy_ad(478, 97);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(479, p.p217);
        }

        if (s.v[467] != 0.0) {
            s.copy_ad(480, 98);
        }

        if (s.v[467] != 0.0) {
            s.copy_ad(481, 99);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(482, p.p215);
        }

        if (s.v[467] != 0.0) {
            s.copy_ad(483, 111);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(484, s.v[109]);
        }

        if (s.v[467] != 0.0) {
            s.copy_ad(485, 113);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(486, p.p0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(487, p.p211);
        }

        if (s.v[467] != 0.0) {
            s.copy_ad(488, 38);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(489, p.p216);
        }

        if (s.v[467] != 0.0) {
            s.copy_ad(490, 39);
        }

        if (s.v[467] != 0.0) {
            s.copy_ad(491, 40);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(492, p.p212);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(493, p.p226);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(494, p.p225);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(495, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(496, p.p227);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(497, p.p231);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(498, p.p222);
        }

    }

    pub(super) fn stamp_transient_block_4(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[467] != 0.0) {
            s.store_scalar(499, p.p223);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(500, p.p224);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(501, p.p230);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(502, p.p229);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(503, p.p228);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(504, p.p39);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(505, p.p47);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(506, p.p45);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(507, p.p42);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(508, p.p2);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(509, p.p6);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(510, 1.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(511, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(512, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(513, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(514, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(515, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(516, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(517, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(518, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(519, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(520, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(521, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(522, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(523, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(524, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(525, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(526, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(527, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(528, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(529, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(530, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(531, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(532, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(533, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(534, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(535, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(536, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(537, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(538, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(539, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(540, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(541, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(542, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(543, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(544, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(545, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(546, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(547, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(548, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(549, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(550, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(551, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(552, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(553, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(554, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(555, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(556, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(557, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(558, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(559, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(560, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(561, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(562, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(563, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(564, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(565, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(566, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(567, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(568, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(569, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(570, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(571, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(572, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(573, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(574, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(575, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(576, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(577, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(578, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_scalar(579, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_ad(576, &{
                if (!(p.p52 == 0.0)) {
                    A::mul(s.ad_value(478), A::tanh(A::scale(s.ad_value(478), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(478)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.v[467] != 0.0) {
            s.store_sub(577, 477, 478);
        }

        if (s.v[467] != 0.0) {
            s.store_mul(511, 497, 485);
        }

        if (s.v[467] != 0.0) {
            s.store_add_ad(513, A::div(s.ad_value(493), A::scale(s.ad_value(485), 2.302585092994046)), A::mul(s.ad_value(496), s.ad_value(576)));
        }

        if (s.v[467] != 0.0) {
            s.store_add_ad_rhs(514, 492, A::mul(s.ad_value(503), A::sub(s.ad_value(483), s.ad_value(484))));
        }

        if (s.v[467] != 0.0) {
            s.store_ad(532, &A::pow(A::div(s.ad_value(483), s.ad_value(484)), s.ad_value(505)));
        }

        s.v[580] = if (s.v[504] != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[467] != 0.0) && (s.v[580] != 0.0)) {
            s.store_div_ad_rhs(515, 576, A::pow(A::offset(A::pow(A::div(s.ad_value(576), s.ad_value(504)), s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500))));
        }

        if ((s.v[467] != 0.0) && (!(s.v[580] != 0.0))) {
            s.store_scalar(515, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.store_mul_ad_lhs(512, A::sub(s.ad_value(494), A::mul(s.ad_value(515), s.ad_value(495))), 576);
        }

        if (s.v[467] != 0.0) {
            s.store_sub(475, 514, 512);
        }

        if (s.v[467] != 0.0) {
            s.store_mul_ad_lhs(517, A::scale(s.ad_value(513), 2.0), 485);
        }

        if (s.v[467] != 0.0) {
            s.store_mul(518, 488, 517);
        }

        if (s.v[467] != 0.0) {
            s.store_sub_ad_rhs(575, 475, A::scale(s.ad_value(511), (p.p51 * 0.5)));
        }

        if (s.v[467] != 0.0) {
            let assign5860_ad_e6939: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(477), s.ad_value(577)), A::mul(A::sub(s.ad_value(477), s.ad_value(577)), A::tanh(A::scale(A::sub(s.ad_value(477), s.ad_value(577)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(477), s.ad_value(577)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(477), s.ad_value(577)), A::sub(s.ad_value(477), s.ad_value(577))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(574, A::sub(assign5860_ad_e6939, s.ad_value(575)), 511);
        }

        s.v[581] = if (s.v[574] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[467] != 0.0) && (s.v[581] != 0.0)) {
            s.store_scalar(533, 0.0);
        }

        s.v[582] = if (s.v[574] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[467] != 0.0) && (!(s.v[581] != 0.0))) && (s.v[582] != 0.0)) {
            s.store_scalar(533, 1.0);
        }

        if (((s.v[467] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[582] != 0.0))) {
            s.store_div_from_scalar_ad(533, 1.0, A::offset(A::exp(s.ad_value(574)), 1.0));
        }

        if (s.v[467] != 0.0) {
            let assign5920_ad_e7027: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(477), s.ad_value(577)), A::mul(A::sub(s.ad_value(477), s.ad_value(577)), A::tanh(A::scale(A::sub(s.ad_value(477), s.ad_value(577)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(477), s.ad_value(577)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(477), s.ad_value(577)), A::sub(s.ad_value(477), s.ad_value(577))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(534, A::sub(assign5920_ad_e7027, A::sub(s.ad_value(475), A::mul(A::scale(s.ad_value(511), (p.p51 * 0.1)), s.ad_value(533)))), 517);
        }

        s.v[583] = if (s.v[534] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[467] != 0.0) && (s.v[583] != 0.0)) {
            s.store_mul(535, 518, 534);
        }

        s.v[584] = if (s.v[534] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[467] != 0.0) && (!(s.v[583] != 0.0))) && (s.v[584] != 0.0)) {
            s.store_mul_ad_rhs(535, 518, A::exp(s.ad_value(534)));
        }

        if (((s.v[467] != 0.0) && (!(s.v[583] != 0.0))) && (!(s.v[584] != 0.0))) {
            s.store_mul_ad_rhs(535, 518, A::ln(A::offset(A::exp(s.ad_value(534)), 1.0)));
        }

        if (s.v[467] != 0.0) {
            s.store_div_ad_rhs(521, 499, A::mul(s.ad_value(532), A::offset(A::div(A::mul(s.ad_value(501), s.ad_value(535)), s.ad_value(488)), 1.0)));
        }

        if (s.v[467] != 0.0) {
            s.store_div_ad(522, A::mul(A::mul(s.ad_value(498), A::div(A::offset(A::mul(s.ad_value(506), s.ad_value(484)), 1.0), A::offset(A::mul(s.ad_value(506), s.ad_value(483)), 1.0))), A::offset(A::div(A::mul(s.ad_value(507), s.ad_value(576)), s.ad_value(487)), 1.0)), A::offset(A::div(A::mul(s.ad_value(502), s.ad_value(535)), s.ad_value(488)), 1.0));
        }

        if (s.v[467] != 0.0) {
            s.store_add_ad(523, A::div(A::mul(A::mul(A::scale(s.ad_value(533), 2.0), s.ad_value(485)), s.ad_value(521)), s.ad_value(487)), A::mul(A::sub_from_scalar(1.0, s.ad_value(533)), s.ad_value(522)));
        }

        if (s.v[467] != 0.0) {
            s.store_div_ad_lhs(539, A::mul(s.ad_value(522), s.ad_value(487)), 521);
        }

        if (s.v[467] != 0.0) {
            s.store_sub_ad_lhs(540, A::mul(s.ad_value(539), A::sqrt(A::offset(A::div(A::div(A::scale(s.ad_value(535), 2.0), s.ad_value(488)), s.ad_value(539)), 1.0))), 539);
        }

        if (s.v[467] != 0.0) {
            s.store_add_ad(541, A::mul(s.ad_value(539), A::sub_from_scalar(1.0, s.ad_value(533))), A::mul(s.ad_value(517), s.ad_value(533)));
        }

        if (s.v[467] != 0.0) {
            s.store_add_ad(476, A::mul(s.ad_value(540), A::sub_from_scalar(1.0, s.ad_value(533))), A::mul(s.ad_value(517), s.ad_value(533)));
        }

        if (s.v[467] != 0.0) {
            let assign6050_ad_e7256: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(s.ad_value(478), s.ad_value(476)), A::mul(A::neg(A::div(s.ad_value(478), s.ad_value(476))), A::tanh(A::scale(A::neg(A::div(s.ad_value(478), s.ad_value(476))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(s.ad_value(478), s.ad_value(476)), A::sqrt(A::offset(A::mul(A::neg(A::div(s.ad_value(478), s.ad_value(476))), A::neg(A::div(s.ad_value(478), s.ad_value(476)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(542, 1.0, A::pow(A::offset(A::pow(assign6050_ad_e7256, s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500))));
        }

        if (s.v[467] != 0.0) {
            s.store_mul(543, 478, 542);
        }

    }

    pub(super) fn stamp_transient_block_5(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[467] != 0.0) {
            let assign6070_ad_e7337: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(A::neg(s.ad_value(478)), s.ad_value(476)), A::mul(A::neg(A::div(A::neg(s.ad_value(478)), s.ad_value(476))), A::tanh(A::scale(A::neg(A::div(A::neg(s.ad_value(478)), s.ad_value(476))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(A::neg(s.ad_value(478)), s.ad_value(476)), A::sqrt(A::offset(A::mul(A::neg(A::div(A::neg(s.ad_value(478)), s.ad_value(476))), A::neg(A::div(A::neg(s.ad_value(478)), s.ad_value(476)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(544, 1.0, A::pow(A::offset(A::pow(assign6070_ad_e7337, s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500))));
        }

        if (s.v[467] != 0.0) {
            s.store_mul_ad_lhs(545, A::neg(s.ad_value(478)), 544);
        }

        if (s.v[467] != 0.0) {
            s.store_div_ad_lhs(574, A::sub(s.ad_value(477), s.ad_value(575)), 511);
        }

        s.v[585] = if (s.v[574] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[467] != 0.0) && (s.v[585] != 0.0)) {
            s.store_scalar(516, 0.0);
        }

        s.v[586] = if (s.v[574] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[467] != 0.0) && (!(s.v[585] != 0.0))) && (s.v[586] != 0.0)) {
            s.store_scalar(516, 1.0);
        }

        if (((s.v[467] != 0.0) && (!(s.v[585] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_div_from_scalar_ad(516, 1.0, A::offset(A::exp(s.ad_value(574)), 1.0));
        }

        if (s.v[467] != 0.0) {
            s.store_div_ad_lhs(519, A::sub(A::sub(s.ad_value(577), s.ad_value(545)), A::sub(s.ad_value(475), A::mul(A::scale(s.ad_value(511), (p.p51 * 0.1)), s.ad_value(516)))), 517);
        }

        s.v[587] = if (s.v[519] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[467] != 0.0) && (s.v[587] != 0.0)) {
            s.store_mul(520, 518, 519);
        }

        s.v[588] = if (s.v[519] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[467] != 0.0) && (!(s.v[587] != 0.0))) && (s.v[588] != 0.0)) {
            s.store_mul_ad_rhs(520, 518, A::exp(s.ad_value(519)));
        }

        if (((s.v[467] != 0.0) && (!(s.v[587] != 0.0))) && (!(s.v[588] != 0.0))) {
            s.store_mul_ad_rhs(520, 518, A::ln(A::offset(A::exp(s.ad_value(519)), 1.0)));
        }

        if (s.v[467] != 0.0) {
            s.store_div_ad_lhs(574, A::sub(s.ad_value(577), s.ad_value(575)), 511);
        }

        s.v[589] = if (s.v[574] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[467] != 0.0) && (s.v[589] != 0.0)) {
            s.store_scalar(546, 0.0);
        }

        s.v[590] = if (s.v[574] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[467] != 0.0) && (!(s.v[589] != 0.0))) && (s.v[590] != 0.0)) {
            s.store_scalar(546, 1.0);
        }

        if (((s.v[467] != 0.0) && (!(s.v[589] != 0.0))) && (!(s.v[590] != 0.0))) {
            s.store_div_from_scalar_ad(546, 1.0, A::offset(A::exp(s.ad_value(574)), 1.0));
        }

        if (s.v[467] != 0.0) {
            s.store_div_ad_lhs(547, A::sub(A::sub(s.ad_value(477), s.ad_value(543)), A::sub(s.ad_value(475), A::mul(A::scale(s.ad_value(511), (p.p51 * 0.1)), s.ad_value(546)))), 517);
        }

        s.v[591] = if (s.v[547] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[467] != 0.0) && (s.v[591] != 0.0)) {
            s.store_mul(548, 518, 547);
        }

        s.v[592] = if (s.v[547] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[467] != 0.0) && (!(s.v[591] != 0.0))) && (s.v[592] != 0.0)) {
            s.store_mul_ad_rhs(548, 518, A::exp(s.ad_value(547)));
        }

        if (((s.v[467] != 0.0) && (!(s.v[591] != 0.0))) && (!(s.v[592] != 0.0))) {
            s.store_mul_ad_rhs(548, 518, A::ln(A::offset(A::exp(s.ad_value(547)), 1.0)));
        }

        if (s.v[467] != 0.0) {
            s.store_div_ad_lhs(549, A::sub(s.ad_value(520), s.ad_value(548)), 488);
        }

        if (s.v[467] != 0.0) {
            s.store_div(575, 549, 541);
        }

        if (s.v[467] != 0.0) {
            let assign6350_ad_e7614: A = A::pow(A::offset(A::pow({
                if (!(p.p52 == 0.0)) {
                    A::mul(s.ad_value(575), A::tanh(A::scale(s.ad_value(575), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(575)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500)));
            s.store_div_ad_rhs(550, 575, assign6350_ad_e7614);
        }

        if (s.v[467] != 0.0) {
            s.store_mul(551, 523, 550);
        }

        if (s.v[467] != 0.0) {
            s.store_mul_ad_lhs(469, A::mul(A::mul(A::scale(A::mul(A::mul(s.ad_value(509), s.ad_value(486)), s.ad_value(508)), 0.5), A::add(s.ad_value(520), s.ad_value(548))), s.ad_value(551)), 510);
        }

        if (s.v[467] != 0.0) {
            s.store_div_ad_rhs(524, 493, A::scale(s.ad_value(485), 2.302585092994046));
        }

        if (s.v[467] != 0.0) {
            s.store_mul_ad_lhs(526, A::scale(s.ad_value(524), 2.0), 485);
        }

        if (s.v[467] != 0.0) {
            s.store_mul(527, 488, 526);
        }

        if (s.v[467] != 0.0) {
            s.store_sub_ad_rhs(579, 514, A::scale(s.ad_value(511), (p.p51 * 0.5)));
        }

        if (s.v[467] != 0.0) {
            let assign6420_ad_e7718: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(477), s.ad_value(577)), A::mul(A::sub(s.ad_value(477), s.ad_value(577)), A::tanh(A::scale(A::sub(s.ad_value(477), s.ad_value(577)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(477), s.ad_value(577)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(477), s.ad_value(577)), A::sub(s.ad_value(477), s.ad_value(577))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(578, A::sub(assign6420_ad_e7718, s.ad_value(579)), 511);
        }

        s.v[593] = if (s.v[578] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[467] != 0.0) && (s.v[593] != 0.0)) {
            s.store_scalar(536, 0.0);
        }

        s.v[594] = if (s.v[578] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[467] != 0.0) && (!(s.v[593] != 0.0))) && (s.v[594] != 0.0)) {
            s.store_scalar(536, 1.0);
        }

        if (((s.v[467] != 0.0) && (!(s.v[593] != 0.0))) && (!(s.v[594] != 0.0))) {
            s.store_div_from_scalar_ad(536, 1.0, A::offset(A::exp(s.ad_value(578)), 1.0));
        }

        if (s.v[467] != 0.0) {
            let assign6480_ad_e7806: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(477), s.ad_value(577)), A::mul(A::sub(s.ad_value(477), s.ad_value(577)), A::tanh(A::scale(A::sub(s.ad_value(477), s.ad_value(577)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(477), s.ad_value(577)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(477), s.ad_value(577)), A::sub(s.ad_value(477), s.ad_value(577))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(537, A::sub(assign6480_ad_e7806, A::sub(s.ad_value(514), A::mul(A::scale(s.ad_value(511), (p.p51 * 0.1)), s.ad_value(536)))), 526);
        }

        s.v[595] = if (s.v[537] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[467] != 0.0) && (s.v[595] != 0.0)) {
            s.store_mul(538, 527, 537);
        }

        s.v[596] = if (s.v[537] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[467] != 0.0) && (!(s.v[595] != 0.0))) && (s.v[596] != 0.0)) {
            s.store_mul_ad_rhs(538, 527, A::exp(s.ad_value(537)));
        }

        if (((s.v[467] != 0.0) && (!(s.v[595] != 0.0))) && (!(s.v[596] != 0.0))) {
            s.store_mul_ad_rhs(538, 527, A::ln(A::offset(A::exp(s.ad_value(537)), 1.0)));
        }

        if (s.v[467] != 0.0) {
            s.store_div(530, 499, 532);
        }

        if (s.v[467] != 0.0) {
            s.store_mul_ad_rhs(531, 498, A::div(A::offset(A::mul(s.ad_value(506), s.ad_value(484)), 1.0), A::offset(A::mul(s.ad_value(506), s.ad_value(483)), 1.0)));
        }

        if (s.v[467] != 0.0) {
            s.store_div_ad_lhs(552, A::mul(s.ad_value(531), s.ad_value(487)), 530);
        }

        if (s.v[467] != 0.0) {
            s.store_sub_ad_lhs(553, A::mul(s.ad_value(552), A::sqrt(A::offset(A::div(A::div(A::scale(s.ad_value(538), 2.0), s.ad_value(488)), s.ad_value(552)), 1.0))), 552);
        }

        if (s.v[467] != 0.0) {
            s.store_add_ad(554, A::mul(s.ad_value(553), A::sub_from_scalar(1.0, s.ad_value(536))), A::mul(s.ad_value(526), s.ad_value(536)));
        }

        if (s.v[467] != 0.0) {
            let assign6590_ad_e7981: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(s.ad_value(478), s.ad_value(554)), A::mul(A::neg(A::div(s.ad_value(478), s.ad_value(554))), A::tanh(A::scale(A::neg(A::div(s.ad_value(478), s.ad_value(554))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(s.ad_value(478), s.ad_value(554)), A::sqrt(A::offset(A::mul(A::neg(A::div(s.ad_value(478), s.ad_value(554))), A::neg(A::div(s.ad_value(478), s.ad_value(554)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(555, 1.0, A::pow(A::offset(A::pow(assign6590_ad_e7981, s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500))));
        }

        if (s.v[467] != 0.0) {
            s.store_mul(556, 478, 555);
        }

        if (s.v[467] != 0.0) {
            let assign6610_ad_e8062: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(A::neg(s.ad_value(478)), s.ad_value(554)), A::mul(A::neg(A::div(A::neg(s.ad_value(478)), s.ad_value(554))), A::tanh(A::scale(A::neg(A::div(A::neg(s.ad_value(478)), s.ad_value(554))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(A::neg(s.ad_value(478)), s.ad_value(554)), A::sqrt(A::offset(A::mul(A::neg(A::div(A::neg(s.ad_value(478)), s.ad_value(554))), A::neg(A::div(A::neg(s.ad_value(478)), s.ad_value(554)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(557, 1.0, A::pow(A::offset(A::pow(assign6610_ad_e8062, s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500))));
        }

        if (s.v[467] != 0.0) {
            s.store_mul_ad_lhs(558, A::neg(s.ad_value(478)), 557);
        }

        if (s.v[467] != 0.0) {
            s.store_div_ad_lhs(578, A::sub(s.ad_value(477), s.ad_value(579)), 511);
        }

        s.v[597] = if (s.v[578] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[467] != 0.0) && (s.v[597] != 0.0)) {
            s.store_scalar(525, 0.0);
        }

        s.v[598] = if (s.v[578] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[467] != 0.0) && (!(s.v[597] != 0.0))) && (s.v[598] != 0.0)) {
            s.store_scalar(525, 1.0);
        }

        if (((s.v[467] != 0.0) && (!(s.v[597] != 0.0))) && (!(s.v[598] != 0.0))) {
            s.store_div_from_scalar_ad(525, 1.0, A::offset(A::exp(s.ad_value(578)), 1.0));
        }

        if (s.v[467] != 0.0) {
            s.store_div_ad_lhs(528, A::sub(A::sub(s.ad_value(577), s.ad_value(558)), A::sub(s.ad_value(514), A::mul(A::scale(s.ad_value(511), (p.p51 * 0.1)), s.ad_value(525)))), 526);
        }

        s.v[599] = if (s.v[528] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[467] != 0.0) && (s.v[599] != 0.0)) {
            s.store_mul(529, 527, 528);
        }

        s.v[600] = if (s.v[528] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[467] != 0.0) && (!(s.v[599] != 0.0))) && (s.v[600] != 0.0)) {
            s.store_mul_ad_rhs(529, 527, A::exp(s.ad_value(528)));
        }

        if (((s.v[467] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[600] != 0.0))) {
            s.store_mul_ad_rhs(529, 527, A::ln(A::offset(A::exp(s.ad_value(528)), 1.0)));
        }

        if (s.v[467] != 0.0) {
            s.store_div_ad_lhs(578, A::sub(s.ad_value(577), s.ad_value(579)), 511);
        }

        s.v[601] = if (s.v[578] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[467] != 0.0) && (s.v[601] != 0.0)) {
            s.store_scalar(559, 0.0);
        }

        s.v[602] = if (s.v[578] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[467] != 0.0) && (!(s.v[601] != 0.0))) && (s.v[602] != 0.0)) {
            s.store_scalar(559, 1.0);
        }

        if (((s.v[467] != 0.0) && (!(s.v[601] != 0.0))) && (!(s.v[602] != 0.0))) {
            s.store_div_from_scalar_ad(559, 1.0, A::offset(A::exp(s.ad_value(578)), 1.0));
        }

        if (s.v[467] != 0.0) {
            s.store_div_ad_lhs(560, A::sub(A::sub(s.ad_value(477), s.ad_value(556)), A::sub(s.ad_value(514), A::mul(A::scale(s.ad_value(511), (p.p51 * 0.1)), s.ad_value(559)))), 526);
        }

        s.v[603] = if (s.v[560] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[467] != 0.0) && (s.v[603] != 0.0)) {
            s.store_mul(561, 527, 560);
        }

        s.v[604] = if (s.v[560] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[467] != 0.0) && (!(s.v[603] != 0.0))) && (s.v[604] != 0.0)) {
            s.store_mul_ad_rhs(561, 527, A::exp(s.ad_value(560)));
        }

        if (((s.v[467] != 0.0) && (!(s.v[603] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_mul_ad_rhs(561, 527, A::ln(A::offset(A::exp(s.ad_value(560)), 1.0)));
        }

        if (s.v[467] != 0.0) {
            s.store_offset_ad(562, A::square(s.ad_value(529)), 1e-38);
        }

        if (s.v[467] != 0.0) {
            s.store_offset_ad(563, A::mul(s.ad_value(562), s.ad_value(529)), 1e-57);
        }

        if (s.v[467] != 0.0) {
            s.store_offset_ad(564, A::square(s.ad_value(561)), 1e-38);
        }

        if (s.v[467] != 0.0) {
            s.store_offset_ad(565, A::mul(s.ad_value(564), s.ad_value(561)), 1e-57);
        }

        if (s.v[467] != 0.0) {
            s.store_offset_ad(566, A::mul(s.ad_value(529), s.ad_value(561)), 1e-38);
        }

        if (s.v[467] != 0.0) {
            s.store_div_ad(567, A::scale(A::add(A::add(s.ad_value(562), s.ad_value(564)), s.ad_value(566)), (2.0 / 3.0)), A::offset(A::add(s.ad_value(529), s.ad_value(561)), 2e-19));
        }

        if (s.v[467] != 0.0) {
            s.store_div_ad(568, A::scale(A::add(A::add(A::add(A::scale(s.ad_value(563), 2.0), A::scale(s.ad_value(565), 3.0)), A::mul(A::scale(s.ad_value(562), 4.0), s.ad_value(561))), A::mul(A::scale(s.ad_value(564), 6.0), s.ad_value(529))), 2.0), A::scale(A::add(A::add(s.ad_value(562), s.ad_value(564)), A::scale(s.ad_value(566), 2.0)), 15.0));
        }

        if (s.v[467] != 0.0) {
            s.store_sub(569, 567, 568);
        }

        if (s.v[467] != 0.0) {
            s.copy_ad(570, 568);
        }

        if (s.v[467] != 0.0) {
            s.store_mul_ad_lhs(470, A::mul(A::mul(A::mul(A::mul(s.ad_value(486), s.ad_value(508)), s.ad_value(487)), s.ad_value(509)), s.ad_value(569)), 510);
        }

        if (s.v[467] != 0.0) {
            s.store_mul_ad_lhs(471, A::mul(A::mul(A::mul(A::mul(s.ad_value(486), s.ad_value(508)), s.ad_value(487)), s.ad_value(509)), s.ad_value(570)), 510);
        }

        s.v[605] = if (s.v[479] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[467] != 0.0) && (s.v[605] != 0.0)) {
            s.store_div_ad_lhs(571, A::sub(s.ad_value(480), A::sub(s.ad_value(514), A::scale(s.ad_value(511), (p.p51 * 0.5)))), 526);
        }

        s.v[606] = if (s.v[571] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[467] != 0.0) && (s.v[605] != 0.0)) && (s.v[606] != 0.0)) {
            s.copy_ad(574, 571);
        }

        s.v[607] = if (s.v[571] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[467] != 0.0) && (s.v[605] != 0.0)) && (!(s.v[606] != 0.0))) && (s.v[607] != 0.0)) {
            s.store_exp(574, 571);
        }

        if ((((s.v[467] != 0.0) && (s.v[605] != 0.0)) && (!(s.v[606] != 0.0))) && (!(s.v[607] != 0.0))) {
            s.store_ln_ad(574, A::offset(A::exp(s.ad_value(571)), 1.0));
        }

        if ((s.v[467] != 0.0) && (s.v[605] != 0.0)) {
            s.store_mul_ad_lhs(472, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(486), s.ad_value(508)), s.ad_value(509)), s.ad_value(490)), s.ad_value(526)), s.ad_value(574)), 510);
        }

        if ((s.v[467] != 0.0) && (s.v[605] != 0.0)) {
            s.store_div_ad_lhs(572, A::sub(s.ad_value(481), A::sub(s.ad_value(514), A::scale(s.ad_value(511), (p.p51 * 0.5)))), 526);
        }

        s.v[608] = if (s.v[572] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[467] != 0.0) && (s.v[605] != 0.0)) && (s.v[608] != 0.0)) {
            s.copy_ad(574, 572);
        }

        s.v[609] = if (s.v[572] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[467] != 0.0) && (s.v[605] != 0.0)) && (!(s.v[608] != 0.0))) && (s.v[609] != 0.0)) {
            s.store_exp(574, 572);
        }

        if ((((s.v[467] != 0.0) && (s.v[605] != 0.0)) && (!(s.v[608] != 0.0))) && (!(s.v[609] != 0.0))) {
            s.store_ln_ad(574, A::offset(A::exp(s.ad_value(572)), 1.0));
        }

        if ((s.v[467] != 0.0) && (s.v[605] != 0.0)) {
            s.store_mul_ad_lhs(473, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(486), s.ad_value(508)), s.ad_value(509)), s.ad_value(491)), s.ad_value(526)), s.ad_value(574)), 510);
        }

        if ((s.v[467] != 0.0) && (!(s.v[605] != 0.0))) {
            s.store_scalar(472, 0.0);
        }

        if ((s.v[467] != 0.0) && (!(s.v[605] != 0.0))) {
            s.store_scalar(473, 0.0);
        }

        s.v[610] = if (s.v[482] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[467] != 0.0) && (s.v[610] != 0.0)) {
            s.store_div_ad_lhs(573, A::sub(s.ad_value(477), A::sub(s.ad_value(514), A::scale(s.ad_value(511), (p.p51 * 0.5)))), 526);
        }

        s.v[611] = if (s.v[573] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[467] != 0.0) && (s.v[610] != 0.0)) && (s.v[611] != 0.0)) {
            s.copy_ad(574, 573);
        }

        s.v[612] = if (s.v[573] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[467] != 0.0) && (s.v[610] != 0.0)) && (!(s.v[611] != 0.0))) && (s.v[612] != 0.0)) {
            s.store_exp(574, 573);
        }

        if ((((s.v[467] != 0.0) && (s.v[610] != 0.0)) && (!(s.v[611] != 0.0))) && (!(s.v[612] != 0.0))) {
            s.store_ln_ad(574, A::offset(A::exp(s.ad_value(573)), 1.0));
        }

        if ((s.v[467] != 0.0) && (s.v[610] != 0.0)) {
            s.store_mul_ad_lhs(474, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(486), s.ad_value(508)), s.ad_value(509)), s.ad_value(489)), s.ad_value(526)), s.ad_value(574)), 510);
        }

        if ((s.v[467] != 0.0) && (!(s.v[610] != 0.0))) {
            s.store_scalar(474, 0.0);
        }

        if (s.v[467] != 0.0) {
            s.copy_ad(468, 469);
        }

        if (s.v[467] != 0.0) {
            s.copy_ad(202, 469);
        }

        if (s.v[467] != 0.0) {
            s.copy_ad(203, 470);
        }

        if (s.v[467] != 0.0) {
            s.copy_ad(204, 471);
        }

        if (s.v[467] != 0.0) {
            s.copy_ad(205, 472);
        }

    }

    pub(super) fn stamp_transient_block_6(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[467] != 0.0) {
            s.copy_ad(206, 473);
        }

        if (s.v[467] != 0.0) {
            s.copy_ad(207, 474);
        }

        if (s.v[467] != 0.0) {
            s.copy_ad(202, 468);
        }

        s.v[613] = if (p.p210 == 1.0) { 1.0 } else { 0.0 };

        s.v[196] = 0.0;

        s.v[197] = 0.0;

        s.v[198] = 0.0;

        s.v[199] = 0.0;

        s.v[200] = 0.0;

        s.v[201] = 0.0;

        s.v[614] = if (p.p189 > p.p354) { 1.0 } else { 0.0 };

        if (s.v[614] != 0.0) {
            s.store_scalar(615, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(616, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(617, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(618, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(619, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(620, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(621, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(622, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(623, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.copy_ad(624, 90);
        }

        if (s.v[614] != 0.0) {
            s.copy_ad(625, 91);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(626, p.p195);
        }

        if (s.v[614] != 0.0) {
            s.copy_ad(627, 92);
        }

        if (s.v[614] != 0.0) {
            s.copy_ad(628, 93);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(629, p.p193);
        }

        if (s.v[614] != 0.0) {
            s.copy_ad(630, 111);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(631, s.v[109]);
        }

        if (s.v[614] != 0.0) {
            s.copy_ad(632, 113);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(633, p.p0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(634, p.p189);
        }

        if (s.v[614] != 0.0) {
            s.copy_ad(635, 35);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(636, p.p194);
        }

        if (s.v[614] != 0.0) {
            s.copy_ad(637, 36);
        }

        if (s.v[614] != 0.0) {
            s.copy_ad(638, 37);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(639, p.p190);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(640, p.p204);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(641, p.p203);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(642, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(643, p.p205);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(644, p.p209);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(645, p.p200);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(646, p.p201);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(647, p.p202);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(648, p.p208);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(649, p.p207);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(650, p.p206);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(651, p.p39);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(652, p.p47);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(653, p.p45);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(654, p.p42);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(655, p.p2);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(656, p.p6);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(657, 1.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(658, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(659, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(660, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(661, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(662, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(663, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(664, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(665, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(666, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(667, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(668, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(669, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(670, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(671, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(672, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(673, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(674, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(675, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(676, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(677, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(678, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(679, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(680, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(681, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(682, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(683, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(684, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(685, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(686, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(687, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(688, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(689, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(690, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(691, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(692, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(693, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(694, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(695, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(696, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(697, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(698, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(699, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(700, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(701, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(702, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(703, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(704, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(705, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(706, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(707, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(708, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(709, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(710, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(711, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(712, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(713, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(714, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(715, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(716, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(717, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(718, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(719, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(720, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(721, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(722, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(723, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(724, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(725, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_scalar(726, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_ad(723, &{
                if (!(p.p52 == 0.0)) {
                    A::mul(s.ad_value(625), A::tanh(A::scale(s.ad_value(625), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(625)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.v[614] != 0.0) {
            s.store_sub(724, 624, 625);
        }

        if (s.v[614] != 0.0) {
            s.store_mul(658, 644, 632);
        }

        if (s.v[614] != 0.0) {
            s.store_add_ad(660, A::div(s.ad_value(640), A::scale(s.ad_value(632), 2.302585092994046)), A::mul(s.ad_value(643), s.ad_value(723)));
        }

        if (s.v[614] != 0.0) {
            s.store_add_ad_rhs(661, 639, A::mul(s.ad_value(650), A::sub(s.ad_value(630), s.ad_value(631))));
        }

        if (s.v[614] != 0.0) {
            s.store_ad(679, &A::pow(A::div(s.ad_value(630), s.ad_value(631)), s.ad_value(652)));
        }

    }

    pub(super) fn stamp_transient_block_7(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[727] = if (s.v[651] != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[614] != 0.0) && (s.v[727] != 0.0)) {
            s.store_div_ad_rhs(662, 723, A::pow(A::offset(A::pow(A::div(s.ad_value(723), s.ad_value(651)), s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647))));
        }

        if ((s.v[614] != 0.0) && (!(s.v[727] != 0.0))) {
            s.store_scalar(662, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.store_mul_ad_lhs(659, A::sub(s.ad_value(641), A::mul(s.ad_value(662), s.ad_value(642))), 723);
        }

        if (s.v[614] != 0.0) {
            s.store_sub(622, 661, 659);
        }

        if (s.v[614] != 0.0) {
            s.store_mul_ad_lhs(664, A::scale(s.ad_value(660), 2.0), 632);
        }

        if (s.v[614] != 0.0) {
            s.store_mul(665, 635, 664);
        }

        if (s.v[614] != 0.0) {
            s.store_sub_ad_rhs(722, 622, A::scale(s.ad_value(658), (p.p51 * 0.5)));
        }

        if (s.v[614] != 0.0) {
            let assign8700_ad_e9363: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(624), s.ad_value(724)), A::mul(A::sub(s.ad_value(624), s.ad_value(724)), A::tanh(A::scale(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(624), s.ad_value(724)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(624), s.ad_value(724)), A::sub(s.ad_value(624), s.ad_value(724))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(721, A::sub(assign8700_ad_e9363, s.ad_value(722)), 658);
        }

        s.v[728] = if (s.v[721] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[614] != 0.0) && (s.v[728] != 0.0)) {
            s.store_scalar(680, 0.0);
        }

        s.v[729] = if (s.v[721] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[614] != 0.0) && (!(s.v[728] != 0.0))) && (s.v[729] != 0.0)) {
            s.store_scalar(680, 1.0);
        }

        if (((s.v[614] != 0.0) && (!(s.v[728] != 0.0))) && (!(s.v[729] != 0.0))) {
            s.store_div_from_scalar_ad(680, 1.0, A::offset(A::exp(s.ad_value(721)), 1.0));
        }

        if (s.v[614] != 0.0) {
            let assign8760_ad_e9451: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(624), s.ad_value(724)), A::mul(A::sub(s.ad_value(624), s.ad_value(724)), A::tanh(A::scale(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(624), s.ad_value(724)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(624), s.ad_value(724)), A::sub(s.ad_value(624), s.ad_value(724))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(681, A::sub(assign8760_ad_e9451, A::sub(s.ad_value(622), A::mul(A::scale(s.ad_value(658), (p.p51 * 0.1)), s.ad_value(680)))), 664);
        }

        s.v[730] = if (s.v[681] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[614] != 0.0) && (s.v[730] != 0.0)) {
            s.store_mul(682, 665, 681);
        }

        s.v[731] = if (s.v[681] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[614] != 0.0) && (!(s.v[730] != 0.0))) && (s.v[731] != 0.0)) {
            s.store_mul_ad_rhs(682, 665, A::exp(s.ad_value(681)));
        }

        if (((s.v[614] != 0.0) && (!(s.v[730] != 0.0))) && (!(s.v[731] != 0.0))) {
            s.store_mul_ad_rhs(682, 665, A::ln(A::offset(A::exp(s.ad_value(681)), 1.0)));
        }

        if (s.v[614] != 0.0) {
            s.store_div_ad_rhs(668, 646, A::mul(s.ad_value(679), A::offset(A::div(A::mul(s.ad_value(648), s.ad_value(682)), s.ad_value(635)), 1.0)));
        }

        if (s.v[614] != 0.0) {
            s.store_div_ad(669, A::mul(A::mul(s.ad_value(645), A::div(A::offset(A::mul(s.ad_value(653), s.ad_value(631)), 1.0), A::offset(A::mul(s.ad_value(653), s.ad_value(630)), 1.0))), A::offset(A::div(A::mul(s.ad_value(654), s.ad_value(723)), s.ad_value(634)), 1.0)), A::offset(A::div(A::mul(s.ad_value(649), s.ad_value(682)), s.ad_value(635)), 1.0));
        }

        if (s.v[614] != 0.0) {
            s.store_add_ad(670, A::div(A::mul(A::mul(A::scale(s.ad_value(680), 2.0), s.ad_value(632)), s.ad_value(668)), s.ad_value(634)), A::mul(A::sub_from_scalar(1.0, s.ad_value(680)), s.ad_value(669)));
        }

        if (s.v[614] != 0.0) {
            s.store_div_ad_lhs(686, A::mul(s.ad_value(669), s.ad_value(634)), 668);
        }

        if (s.v[614] != 0.0) {
            s.store_sub_ad_lhs(687, A::mul(s.ad_value(686), A::sqrt(A::offset(A::div(A::div(A::scale(s.ad_value(682), 2.0), s.ad_value(635)), s.ad_value(686)), 1.0))), 686);
        }

        if (s.v[614] != 0.0) {
            s.store_add_ad(688, A::mul(s.ad_value(686), A::sub_from_scalar(1.0, s.ad_value(680))), A::mul(s.ad_value(664), s.ad_value(680)));
        }

        if (s.v[614] != 0.0) {
            s.store_add_ad(623, A::mul(s.ad_value(687), A::sub_from_scalar(1.0, s.ad_value(680))), A::mul(s.ad_value(664), s.ad_value(680)));
        }

        if (s.v[614] != 0.0) {
            let assign8890_ad_e9680: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(s.ad_value(625), s.ad_value(623)), A::mul(A::neg(A::div(s.ad_value(625), s.ad_value(623))), A::tanh(A::scale(A::neg(A::div(s.ad_value(625), s.ad_value(623))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(s.ad_value(625), s.ad_value(623)), A::sqrt(A::offset(A::mul(A::neg(A::div(s.ad_value(625), s.ad_value(623))), A::neg(A::div(s.ad_value(625), s.ad_value(623)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(689, 1.0, A::pow(A::offset(A::pow(assign8890_ad_e9680, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647))));
        }

        if (s.v[614] != 0.0) {
            s.store_mul(690, 625, 689);
        }

        if (s.v[614] != 0.0) {
            let assign8910_ad_e9761: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(A::neg(s.ad_value(625)), s.ad_value(623)), A::mul(A::neg(A::div(A::neg(s.ad_value(625)), s.ad_value(623))), A::tanh(A::scale(A::neg(A::div(A::neg(s.ad_value(625)), s.ad_value(623))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(A::neg(s.ad_value(625)), s.ad_value(623)), A::sqrt(A::offset(A::mul(A::neg(A::div(A::neg(s.ad_value(625)), s.ad_value(623))), A::neg(A::div(A::neg(s.ad_value(625)), s.ad_value(623)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(691, 1.0, A::pow(A::offset(A::pow(assign8910_ad_e9761, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647))));
        }

        if (s.v[614] != 0.0) {
            s.store_mul_ad_lhs(692, A::neg(s.ad_value(625)), 691);
        }

        if (s.v[614] != 0.0) {
            s.store_div_ad_lhs(721, A::sub(s.ad_value(624), s.ad_value(722)), 658);
        }

        s.v[732] = if (s.v[721] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[614] != 0.0) && (s.v[732] != 0.0)) {
            s.store_scalar(663, 0.0);
        }

        s.v[733] = if (s.v[721] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[614] != 0.0) && (!(s.v[732] != 0.0))) && (s.v[733] != 0.0)) {
            s.store_scalar(663, 1.0);
        }

        if (((s.v[614] != 0.0) && (!(s.v[732] != 0.0))) && (!(s.v[733] != 0.0))) {
            s.store_div_from_scalar_ad(663, 1.0, A::offset(A::exp(s.ad_value(721)), 1.0));
        }

        if (s.v[614] != 0.0) {
            s.store_div_ad_lhs(666, A::sub(A::sub(s.ad_value(724), s.ad_value(692)), A::sub(s.ad_value(622), A::mul(A::scale(s.ad_value(658), (p.p51 * 0.1)), s.ad_value(663)))), 664);
        }

        s.v[734] = if (s.v[666] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[614] != 0.0) && (s.v[734] != 0.0)) {
            s.store_mul(667, 665, 666);
        }

        s.v[735] = if (s.v[666] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[614] != 0.0) && (!(s.v[734] != 0.0))) && (s.v[735] != 0.0)) {
            s.store_mul_ad_rhs(667, 665, A::exp(s.ad_value(666)));
        }

        if (((s.v[614] != 0.0) && (!(s.v[734] != 0.0))) && (!(s.v[735] != 0.0))) {
            s.store_mul_ad_rhs(667, 665, A::ln(A::offset(A::exp(s.ad_value(666)), 1.0)));
        }

        if (s.v[614] != 0.0) {
            s.store_div_ad_lhs(721, A::sub(s.ad_value(724), s.ad_value(722)), 658);
        }

        s.v[736] = if (s.v[721] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[614] != 0.0) && (s.v[736] != 0.0)) {
            s.store_scalar(693, 0.0);
        }

        s.v[737] = if (s.v[721] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[614] != 0.0) && (!(s.v[736] != 0.0))) && (s.v[737] != 0.0)) {
            s.store_scalar(693, 1.0);
        }

        if (((s.v[614] != 0.0) && (!(s.v[736] != 0.0))) && (!(s.v[737] != 0.0))) {
            s.store_div_from_scalar_ad(693, 1.0, A::offset(A::exp(s.ad_value(721)), 1.0));
        }

        if (s.v[614] != 0.0) {
            s.store_div_ad_lhs(694, A::sub(A::sub(s.ad_value(624), s.ad_value(690)), A::sub(s.ad_value(622), A::mul(A::scale(s.ad_value(658), (p.p51 * 0.1)), s.ad_value(693)))), 664);
        }

        s.v[738] = if (s.v[694] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[614] != 0.0) && (s.v[738] != 0.0)) {
            s.store_mul(695, 665, 694);
        }

        s.v[739] = if (s.v[694] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[614] != 0.0) && (!(s.v[738] != 0.0))) && (s.v[739] != 0.0)) {
            s.store_mul_ad_rhs(695, 665, A::exp(s.ad_value(694)));
        }

        if (((s.v[614] != 0.0) && (!(s.v[738] != 0.0))) && (!(s.v[739] != 0.0))) {
            s.store_mul_ad_rhs(695, 665, A::ln(A::offset(A::exp(s.ad_value(694)), 1.0)));
        }

        if (s.v[614] != 0.0) {
            s.store_div_ad_lhs(696, A::sub(s.ad_value(667), s.ad_value(695)), 635);
        }

        if (s.v[614] != 0.0) {
            s.store_div(722, 696, 688);
        }

        if (s.v[614] != 0.0) {
            let assign9190_ad_e10038: A = A::pow(A::offset(A::pow({
                if (!(p.p52 == 0.0)) {
                    A::mul(s.ad_value(722), A::tanh(A::scale(s.ad_value(722), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(722)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647)));
            s.store_div_ad_rhs(697, 722, assign9190_ad_e10038);
        }

        if (s.v[614] != 0.0) {
            s.store_mul(698, 670, 697);
        }

        if (s.v[614] != 0.0) {
            s.store_mul_ad_lhs(616, A::mul(A::mul(A::scale(A::mul(A::mul(s.ad_value(656), s.ad_value(633)), s.ad_value(655)), 0.5), A::add(s.ad_value(667), s.ad_value(695))), s.ad_value(698)), 657);
        }

        if (s.v[614] != 0.0) {
            s.store_div_ad_rhs(671, 640, A::scale(s.ad_value(632), 2.302585092994046));
        }

        if (s.v[614] != 0.0) {
            s.store_mul_ad_lhs(673, A::scale(s.ad_value(671), 2.0), 632);
        }

        if (s.v[614] != 0.0) {
            s.store_mul(674, 635, 673);
        }

        if (s.v[614] != 0.0) {
            s.store_sub_ad_rhs(726, 661, A::scale(s.ad_value(658), (p.p51 * 0.5)));
        }

        if (s.v[614] != 0.0) {
            let assign9260_ad_e10142: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(624), s.ad_value(724)), A::mul(A::sub(s.ad_value(624), s.ad_value(724)), A::tanh(A::scale(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(624), s.ad_value(724)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(624), s.ad_value(724)), A::sub(s.ad_value(624), s.ad_value(724))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(725, A::sub(assign9260_ad_e10142, s.ad_value(726)), 658);
        }

        s.v[740] = if (s.v[725] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[614] != 0.0) && (s.v[740] != 0.0)) {
            s.store_scalar(683, 0.0);
        }

        s.v[741] = if (s.v[725] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[614] != 0.0) && (!(s.v[740] != 0.0))) && (s.v[741] != 0.0)) {
            s.store_scalar(683, 1.0);
        }

        if (((s.v[614] != 0.0) && (!(s.v[740] != 0.0))) && (!(s.v[741] != 0.0))) {
            s.store_div_from_scalar_ad(683, 1.0, A::offset(A::exp(s.ad_value(725)), 1.0));
        }

        if (s.v[614] != 0.0) {
            let assign9320_ad_e10230: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(624), s.ad_value(724)), A::mul(A::sub(s.ad_value(624), s.ad_value(724)), A::tanh(A::scale(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(624), s.ad_value(724)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(624), s.ad_value(724)), A::sub(s.ad_value(624), s.ad_value(724))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(684, A::sub(assign9320_ad_e10230, A::sub(s.ad_value(661), A::mul(A::scale(s.ad_value(658), (p.p51 * 0.1)), s.ad_value(683)))), 673);
        }

        s.v[742] = if (s.v[684] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[614] != 0.0) && (s.v[742] != 0.0)) {
            s.store_mul(685, 674, 684);
        }

        s.v[743] = if (s.v[684] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[614] != 0.0) && (!(s.v[742] != 0.0))) && (s.v[743] != 0.0)) {
            s.store_mul_ad_rhs(685, 674, A::exp(s.ad_value(684)));
        }

        if (((s.v[614] != 0.0) && (!(s.v[742] != 0.0))) && (!(s.v[743] != 0.0))) {
            s.store_mul_ad_rhs(685, 674, A::ln(A::offset(A::exp(s.ad_value(684)), 1.0)));
        }

        if (s.v[614] != 0.0) {
            s.store_div(677, 646, 679);
        }

        if (s.v[614] != 0.0) {
            s.store_mul_ad_rhs(678, 645, A::div(A::offset(A::mul(s.ad_value(653), s.ad_value(631)), 1.0), A::offset(A::mul(s.ad_value(653), s.ad_value(630)), 1.0)));
        }

        if (s.v[614] != 0.0) {
            s.store_div_ad_lhs(699, A::mul(s.ad_value(678), s.ad_value(634)), 677);
        }

        if (s.v[614] != 0.0) {
            s.store_sub_ad_lhs(700, A::mul(s.ad_value(699), A::sqrt(A::offset(A::div(A::div(A::scale(s.ad_value(685), 2.0), s.ad_value(635)), s.ad_value(699)), 1.0))), 699);
        }

        if (s.v[614] != 0.0) {
            s.store_add_ad(701, A::mul(s.ad_value(700), A::sub_from_scalar(1.0, s.ad_value(683))), A::mul(s.ad_value(673), s.ad_value(683)));
        }

        if (s.v[614] != 0.0) {
            let assign9430_ad_e10405: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(s.ad_value(625), s.ad_value(701)), A::mul(A::neg(A::div(s.ad_value(625), s.ad_value(701))), A::tanh(A::scale(A::neg(A::div(s.ad_value(625), s.ad_value(701))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(s.ad_value(625), s.ad_value(701)), A::sqrt(A::offset(A::mul(A::neg(A::div(s.ad_value(625), s.ad_value(701))), A::neg(A::div(s.ad_value(625), s.ad_value(701)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(702, 1.0, A::pow(A::offset(A::pow(assign9430_ad_e10405, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647))));
        }

        if (s.v[614] != 0.0) {
            s.store_mul(703, 625, 702);
        }

        if (s.v[614] != 0.0) {
            let assign9450_ad_e10486: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(A::neg(s.ad_value(625)), s.ad_value(701)), A::mul(A::neg(A::div(A::neg(s.ad_value(625)), s.ad_value(701))), A::tanh(A::scale(A::neg(A::div(A::neg(s.ad_value(625)), s.ad_value(701))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(A::neg(s.ad_value(625)), s.ad_value(701)), A::sqrt(A::offset(A::mul(A::neg(A::div(A::neg(s.ad_value(625)), s.ad_value(701))), A::neg(A::div(A::neg(s.ad_value(625)), s.ad_value(701)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(704, 1.0, A::pow(A::offset(A::pow(assign9450_ad_e10486, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647))));
        }

        if (s.v[614] != 0.0) {
            s.store_mul_ad_lhs(705, A::neg(s.ad_value(625)), 704);
        }

        if (s.v[614] != 0.0) {
            s.store_div_ad_lhs(725, A::sub(s.ad_value(624), s.ad_value(726)), 658);
        }

        s.v[744] = if (s.v[725] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[614] != 0.0) && (s.v[744] != 0.0)) {
            s.store_scalar(672, 0.0);
        }

        s.v[745] = if (s.v[725] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[614] != 0.0) && (!(s.v[744] != 0.0))) && (s.v[745] != 0.0)) {
            s.store_scalar(672, 1.0);
        }

        if (((s.v[614] != 0.0) && (!(s.v[744] != 0.0))) && (!(s.v[745] != 0.0))) {
            s.store_div_from_scalar_ad(672, 1.0, A::offset(A::exp(s.ad_value(725)), 1.0));
        }

        if (s.v[614] != 0.0) {
            s.store_div_ad_lhs(675, A::sub(A::sub(s.ad_value(724), s.ad_value(705)), A::sub(s.ad_value(661), A::mul(A::scale(s.ad_value(658), (p.p51 * 0.1)), s.ad_value(672)))), 673);
        }

        s.v[746] = if (s.v[675] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[614] != 0.0) && (s.v[746] != 0.0)) {
            s.store_mul(676, 674, 675);
        }

        s.v[747] = if (s.v[675] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[614] != 0.0) && (!(s.v[746] != 0.0))) && (s.v[747] != 0.0)) {
            s.store_mul_ad_rhs(676, 674, A::exp(s.ad_value(675)));
        }

        if (((s.v[614] != 0.0) && (!(s.v[746] != 0.0))) && (!(s.v[747] != 0.0))) {
            s.store_mul_ad_rhs(676, 674, A::ln(A::offset(A::exp(s.ad_value(675)), 1.0)));
        }

        if (s.v[614] != 0.0) {
            s.store_div_ad_lhs(725, A::sub(s.ad_value(724), s.ad_value(726)), 658);
        }

        s.v[748] = if (s.v[725] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[614] != 0.0) && (s.v[748] != 0.0)) {
            s.store_scalar(706, 0.0);
        }

        s.v[749] = if (s.v[725] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[614] != 0.0) && (!(s.v[748] != 0.0))) && (s.v[749] != 0.0)) {
            s.store_scalar(706, 1.0);
        }

        if (((s.v[614] != 0.0) && (!(s.v[748] != 0.0))) && (!(s.v[749] != 0.0))) {
            s.store_div_from_scalar_ad(706, 1.0, A::offset(A::exp(s.ad_value(725)), 1.0));
        }

        if (s.v[614] != 0.0) {
            s.store_div_ad_lhs(707, A::sub(A::sub(s.ad_value(624), s.ad_value(703)), A::sub(s.ad_value(661), A::mul(A::scale(s.ad_value(658), (p.p51 * 0.1)), s.ad_value(706)))), 673);
        }

        s.v[750] = if (s.v[707] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[614] != 0.0) && (s.v[750] != 0.0)) {
            s.store_mul(708, 674, 707);
        }

        s.v[751] = if (s.v[707] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[614] != 0.0) && (!(s.v[750] != 0.0))) && (s.v[751] != 0.0)) {
            s.store_mul_ad_rhs(708, 674, A::exp(s.ad_value(707)));
        }

        if (((s.v[614] != 0.0) && (!(s.v[750] != 0.0))) && (!(s.v[751] != 0.0))) {
            s.store_mul_ad_rhs(708, 674, A::ln(A::offset(A::exp(s.ad_value(707)), 1.0)));
        }

        if (s.v[614] != 0.0) {
            s.store_offset_ad(709, A::square(s.ad_value(676)), 1e-38);
        }

        if (s.v[614] != 0.0) {
            s.store_offset_ad(710, A::mul(s.ad_value(709), s.ad_value(676)), 1e-57);
        }

    }

    pub(super) fn stamp_transient_block_8(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[614] != 0.0) {
            s.store_offset_ad(711, A::square(s.ad_value(708)), 1e-38);
        }

        if (s.v[614] != 0.0) {
            s.store_offset_ad(712, A::mul(s.ad_value(711), s.ad_value(708)), 1e-57);
        }

        if (s.v[614] != 0.0) {
            s.store_offset_ad(713, A::mul(s.ad_value(676), s.ad_value(708)), 1e-38);
        }

        if (s.v[614] != 0.0) {
            s.store_div_ad(714, A::scale(A::add(A::add(s.ad_value(709), s.ad_value(711)), s.ad_value(713)), (2.0 / 3.0)), A::offset(A::add(s.ad_value(676), s.ad_value(708)), 2e-19));
        }

        if (s.v[614] != 0.0) {
            s.store_div_ad(715, A::scale(A::add(A::add(A::add(A::scale(s.ad_value(710), 2.0), A::scale(s.ad_value(712), 3.0)), A::mul(A::scale(s.ad_value(709), 4.0), s.ad_value(708))), A::mul(A::scale(s.ad_value(711), 6.0), s.ad_value(676))), 2.0), A::scale(A::add(A::add(s.ad_value(709), s.ad_value(711)), A::scale(s.ad_value(713), 2.0)), 15.0));
        }

        if (s.v[614] != 0.0) {
            s.store_sub(716, 714, 715);
        }

        if (s.v[614] != 0.0) {
            s.copy_ad(717, 715);
        }

        if (s.v[614] != 0.0) {
            s.store_mul_ad_lhs(617, A::mul(A::mul(A::mul(A::mul(s.ad_value(633), s.ad_value(655)), s.ad_value(634)), s.ad_value(656)), s.ad_value(716)), 657);
        }

        if (s.v[614] != 0.0) {
            s.store_mul_ad_lhs(618, A::mul(A::mul(A::mul(A::mul(s.ad_value(633), s.ad_value(655)), s.ad_value(634)), s.ad_value(656)), s.ad_value(717)), 657);
        }

        s.v[752] = if (s.v[626] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[614] != 0.0) && (s.v[752] != 0.0)) {
            s.store_div_ad_lhs(718, A::sub(s.ad_value(627), A::sub(s.ad_value(661), A::scale(s.ad_value(658), (p.p51 * 0.5)))), 673);
        }

        s.v[753] = if (s.v[718] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[614] != 0.0) && (s.v[752] != 0.0)) && (s.v[753] != 0.0)) {
            s.copy_ad(721, 718);
        }

        s.v[754] = if (s.v[718] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[614] != 0.0) && (s.v[752] != 0.0)) && (!(s.v[753] != 0.0))) && (s.v[754] != 0.0)) {
            s.store_exp(721, 718);
        }

        if ((((s.v[614] != 0.0) && (s.v[752] != 0.0)) && (!(s.v[753] != 0.0))) && (!(s.v[754] != 0.0))) {
            s.store_ln_ad(721, A::offset(A::exp(s.ad_value(718)), 1.0));
        }

        if ((s.v[614] != 0.0) && (s.v[752] != 0.0)) {
            s.store_mul_ad_lhs(619, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(633), s.ad_value(655)), s.ad_value(656)), s.ad_value(637)), s.ad_value(673)), s.ad_value(721)), 657);
        }

        if ((s.v[614] != 0.0) && (s.v[752] != 0.0)) {
            s.store_div_ad_lhs(719, A::sub(s.ad_value(628), A::sub(s.ad_value(661), A::scale(s.ad_value(658), (p.p51 * 0.5)))), 673);
        }

        s.v[755] = if (s.v[719] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[614] != 0.0) && (s.v[752] != 0.0)) && (s.v[755] != 0.0)) {
            s.copy_ad(721, 719);
        }

        s.v[756] = if (s.v[719] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[614] != 0.0) && (s.v[752] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[756] != 0.0)) {
            s.store_exp(721, 719);
        }

        if ((((s.v[614] != 0.0) && (s.v[752] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[756] != 0.0))) {
            s.store_ln_ad(721, A::offset(A::exp(s.ad_value(719)), 1.0));
        }

        if ((s.v[614] != 0.0) && (s.v[752] != 0.0)) {
            s.store_mul_ad_lhs(620, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(633), s.ad_value(655)), s.ad_value(656)), s.ad_value(638)), s.ad_value(673)), s.ad_value(721)), 657);
        }

        if ((s.v[614] != 0.0) && (!(s.v[752] != 0.0))) {
            s.store_scalar(619, 0.0);
        }

        if ((s.v[614] != 0.0) && (!(s.v[752] != 0.0))) {
            s.store_scalar(620, 0.0);
        }

        s.v[757] = if (s.v[629] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[614] != 0.0) && (s.v[757] != 0.0)) {
            s.store_div_ad_lhs(720, A::sub(s.ad_value(624), A::sub(s.ad_value(661), A::scale(s.ad_value(658), (p.p51 * 0.5)))), 673);
        }

        s.v[758] = if (s.v[720] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[614] != 0.0) && (s.v[757] != 0.0)) && (s.v[758] != 0.0)) {
            s.copy_ad(721, 720);
        }

        s.v[759] = if (s.v[720] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[614] != 0.0) && (s.v[757] != 0.0)) && (!(s.v[758] != 0.0))) && (s.v[759] != 0.0)) {
            s.store_exp(721, 720);
        }

        if ((((s.v[614] != 0.0) && (s.v[757] != 0.0)) && (!(s.v[758] != 0.0))) && (!(s.v[759] != 0.0))) {
            s.store_ln_ad(721, A::offset(A::exp(s.ad_value(720)), 1.0));
        }

        if ((s.v[614] != 0.0) && (s.v[757] != 0.0)) {
            s.store_mul_ad_lhs(621, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(633), s.ad_value(655)), s.ad_value(656)), s.ad_value(636)), s.ad_value(673)), s.ad_value(721)), 657);
        }

        if ((s.v[614] != 0.0) && (!(s.v[757] != 0.0))) {
            s.store_scalar(621, 0.0);
        }

        if (s.v[614] != 0.0) {
            s.copy_ad(615, 616);
        }

        if (s.v[614] != 0.0) {
            s.copy_ad(196, 616);
        }

        if (s.v[614] != 0.0) {
            s.copy_ad(197, 617);
        }

        if (s.v[614] != 0.0) {
            s.copy_ad(198, 618);
        }

        if (s.v[614] != 0.0) {
            s.copy_ad(199, 619);
        }

        if (s.v[614] != 0.0) {
            s.copy_ad(200, 620);
        }

        if (s.v[614] != 0.0) {
            s.copy_ad(201, 621);
        }

        if (s.v[614] != 0.0) {
            s.copy_ad(196, 615);
        }

        s.v[760] = if (p.p188 == 1.0) { 1.0 } else { 0.0 };

        s.v[190] = 0.0;

        s.v[191] = 0.0;

        s.v[192] = 0.0;

        s.v[193] = 0.0;

        s.v[194] = 0.0;

        s.v[195] = 0.0;

        s.v[761] = if (p.p167 > p.p354) { 1.0 } else { 0.0 };

        if (s.v[761] != 0.0) {
            s.store_scalar(762, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(763, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(764, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(765, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(766, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(767, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(768, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(769, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(770, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.copy_ad(771, 84);
        }

        if (s.v[761] != 0.0) {
            s.copy_ad(772, 85);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(773, p.p173);
        }

        if (s.v[761] != 0.0) {
            s.copy_ad(774, 86);
        }

        if (s.v[761] != 0.0) {
            s.copy_ad(775, 87);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(776, p.p171);
        }

        if (s.v[761] != 0.0) {
            s.copy_ad(777, 111);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(778, s.v[109]);
        }

        if (s.v[761] != 0.0) {
            s.copy_ad(779, 113);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(780, p.p0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(781, p.p167);
        }

        if (s.v[761] != 0.0) {
            s.copy_ad(782, 32);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(783, p.p172);
        }

        if (s.v[761] != 0.0) {
            s.copy_ad(784, 33);
        }

        if (s.v[761] != 0.0) {
            s.copy_ad(785, 34);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(786, p.p168);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(787, p.p182);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(788, p.p181);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(789, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(790, p.p183);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(791, p.p187);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(792, p.p178);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(793, p.p179);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(794, p.p180);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(795, p.p186);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(796, p.p185);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(797, p.p184);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(798, p.p39);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(799, p.p47);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(800, p.p45);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(801, p.p42);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(802, p.p2);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(803, p.p6);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(804, 1.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(805, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(806, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(807, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(808, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(809, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(810, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(811, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(812, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(813, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(814, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(815, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(816, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(817, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(818, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(819, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(820, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(821, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(822, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(823, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(824, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(825, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(826, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(827, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(828, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(829, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(830, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(831, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(832, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(833, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(834, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(835, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(836, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(837, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(838, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(839, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(840, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(841, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(842, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(843, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(844, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(845, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(846, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_9(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[761] != 0.0) {
            s.store_scalar(847, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(848, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(849, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(850, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(851, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(852, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(853, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(854, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(855, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(856, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(857, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(858, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(859, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(860, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(861, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(862, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(863, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(864, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(865, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(866, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(867, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(868, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(869, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(870, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(871, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(872, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_scalar(873, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_ad(870, &{
                if (!(p.p52 == 0.0)) {
                    A::mul(s.ad_value(772), A::tanh(A::scale(s.ad_value(772), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(772)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.v[761] != 0.0) {
            s.store_sub(871, 771, 772);
        }

        if (s.v[761] != 0.0) {
            s.store_mul(805, 791, 779);
        }

        if (s.v[761] != 0.0) {
            s.store_add_ad(807, A::div(s.ad_value(787), A::scale(s.ad_value(779), 2.302585092994046)), A::mul(s.ad_value(790), s.ad_value(870)));
        }

        if (s.v[761] != 0.0) {
            s.store_add_ad_rhs(808, 786, A::mul(s.ad_value(797), A::sub(s.ad_value(777), s.ad_value(778))));
        }

        if (s.v[761] != 0.0) {
            s.store_ad(826, &A::pow(A::div(s.ad_value(777), s.ad_value(778)), s.ad_value(799)));
        }

        s.v[874] = if (s.v[798] != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[761] != 0.0) && (s.v[874] != 0.0)) {
            s.store_div_ad_rhs(809, 870, A::pow(A::offset(A::pow(A::div(s.ad_value(870), s.ad_value(798)), s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794))));
        }

        if ((s.v[761] != 0.0) && (!(s.v[874] != 0.0))) {
            s.store_scalar(809, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.store_mul_ad_lhs(806, A::sub(s.ad_value(788), A::mul(s.ad_value(809), s.ad_value(789))), 870);
        }

        if (s.v[761] != 0.0) {
            s.store_sub(769, 808, 806);
        }

        if (s.v[761] != 0.0) {
            s.store_mul_ad_lhs(811, A::scale(s.ad_value(807), 2.0), 779);
        }

        if (s.v[761] != 0.0) {
            s.store_mul(812, 782, 811);
        }

        if (s.v[761] != 0.0) {
            s.store_sub_ad_rhs(869, 769, A::scale(s.ad_value(805), (p.p51 * 0.5)));
        }

        if (s.v[761] != 0.0) {
            let assign11540_ad_e11787: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(771), s.ad_value(871)), A::mul(A::sub(s.ad_value(771), s.ad_value(871)), A::tanh(A::scale(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(771), s.ad_value(871)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(771), s.ad_value(871)), A::sub(s.ad_value(771), s.ad_value(871))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(868, A::sub(assign11540_ad_e11787, s.ad_value(869)), 805);
        }

        s.v[875] = if (s.v[868] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[761] != 0.0) && (s.v[875] != 0.0)) {
            s.store_scalar(827, 0.0);
        }

        s.v[876] = if (s.v[868] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[761] != 0.0) && (!(s.v[875] != 0.0))) && (s.v[876] != 0.0)) {
            s.store_scalar(827, 1.0);
        }

        if (((s.v[761] != 0.0) && (!(s.v[875] != 0.0))) && (!(s.v[876] != 0.0))) {
            s.store_div_from_scalar_ad(827, 1.0, A::offset(A::exp(s.ad_value(868)), 1.0));
        }

        if (s.v[761] != 0.0) {
            let assign11600_ad_e11875: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(771), s.ad_value(871)), A::mul(A::sub(s.ad_value(771), s.ad_value(871)), A::tanh(A::scale(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(771), s.ad_value(871)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(771), s.ad_value(871)), A::sub(s.ad_value(771), s.ad_value(871))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(828, A::sub(assign11600_ad_e11875, A::sub(s.ad_value(769), A::mul(A::scale(s.ad_value(805), (p.p51 * 0.1)), s.ad_value(827)))), 811);
        }

        s.v[877] = if (s.v[828] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[761] != 0.0) && (s.v[877] != 0.0)) {
            s.store_mul(829, 812, 828);
        }

        s.v[878] = if (s.v[828] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[761] != 0.0) && (!(s.v[877] != 0.0))) && (s.v[878] != 0.0)) {
            s.store_mul_ad_rhs(829, 812, A::exp(s.ad_value(828)));
        }

        if (((s.v[761] != 0.0) && (!(s.v[877] != 0.0))) && (!(s.v[878] != 0.0))) {
            s.store_mul_ad_rhs(829, 812, A::ln(A::offset(A::exp(s.ad_value(828)), 1.0)));
        }

        if (s.v[761] != 0.0) {
            s.store_div_ad_rhs(815, 793, A::mul(s.ad_value(826), A::offset(A::div(A::mul(s.ad_value(795), s.ad_value(829)), s.ad_value(782)), 1.0)));
        }

        if (s.v[761] != 0.0) {
            s.store_div_ad(816, A::mul(A::mul(s.ad_value(792), A::div(A::offset(A::mul(s.ad_value(800), s.ad_value(778)), 1.0), A::offset(A::mul(s.ad_value(800), s.ad_value(777)), 1.0))), A::offset(A::div(A::mul(s.ad_value(801), s.ad_value(870)), s.ad_value(781)), 1.0)), A::offset(A::div(A::mul(s.ad_value(796), s.ad_value(829)), s.ad_value(782)), 1.0));
        }

        if (s.v[761] != 0.0) {
            s.store_add_ad(817, A::div(A::mul(A::mul(A::scale(s.ad_value(827), 2.0), s.ad_value(779)), s.ad_value(815)), s.ad_value(781)), A::mul(A::sub_from_scalar(1.0, s.ad_value(827)), s.ad_value(816)));
        }

        if (s.v[761] != 0.0) {
            s.store_div_ad_lhs(833, A::mul(s.ad_value(816), s.ad_value(781)), 815);
        }

        if (s.v[761] != 0.0) {
            s.store_sub_ad_lhs(834, A::mul(s.ad_value(833), A::sqrt(A::offset(A::div(A::div(A::scale(s.ad_value(829), 2.0), s.ad_value(782)), s.ad_value(833)), 1.0))), 833);
        }

        if (s.v[761] != 0.0) {
            s.store_add_ad(835, A::mul(s.ad_value(833), A::sub_from_scalar(1.0, s.ad_value(827))), A::mul(s.ad_value(811), s.ad_value(827)));
        }

        if (s.v[761] != 0.0) {
            s.store_add_ad(770, A::mul(s.ad_value(834), A::sub_from_scalar(1.0, s.ad_value(827))), A::mul(s.ad_value(811), s.ad_value(827)));
        }

        if (s.v[761] != 0.0) {
            let assign11730_ad_e12104: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(s.ad_value(772), s.ad_value(770)), A::mul(A::neg(A::div(s.ad_value(772), s.ad_value(770))), A::tanh(A::scale(A::neg(A::div(s.ad_value(772), s.ad_value(770))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(s.ad_value(772), s.ad_value(770)), A::sqrt(A::offset(A::mul(A::neg(A::div(s.ad_value(772), s.ad_value(770))), A::neg(A::div(s.ad_value(772), s.ad_value(770)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(836, 1.0, A::pow(A::offset(A::pow(assign11730_ad_e12104, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794))));
        }

        if (s.v[761] != 0.0) {
            s.store_mul(837, 772, 836);
        }

        if (s.v[761] != 0.0) {
            let assign11750_ad_e12185: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(A::neg(s.ad_value(772)), s.ad_value(770)), A::mul(A::neg(A::div(A::neg(s.ad_value(772)), s.ad_value(770))), A::tanh(A::scale(A::neg(A::div(A::neg(s.ad_value(772)), s.ad_value(770))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(A::neg(s.ad_value(772)), s.ad_value(770)), A::sqrt(A::offset(A::mul(A::neg(A::div(A::neg(s.ad_value(772)), s.ad_value(770))), A::neg(A::div(A::neg(s.ad_value(772)), s.ad_value(770)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(838, 1.0, A::pow(A::offset(A::pow(assign11750_ad_e12185, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794))));
        }

        if (s.v[761] != 0.0) {
            s.store_mul_ad_lhs(839, A::neg(s.ad_value(772)), 838);
        }

        if (s.v[761] != 0.0) {
            s.store_div_ad_lhs(868, A::sub(s.ad_value(771), s.ad_value(869)), 805);
        }

        s.v[879] = if (s.v[868] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[761] != 0.0) && (s.v[879] != 0.0)) {
            s.store_scalar(810, 0.0);
        }

        s.v[880] = if (s.v[868] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[761] != 0.0) && (!(s.v[879] != 0.0))) && (s.v[880] != 0.0)) {
            s.store_scalar(810, 1.0);
        }

        if (((s.v[761] != 0.0) && (!(s.v[879] != 0.0))) && (!(s.v[880] != 0.0))) {
            s.store_div_from_scalar_ad(810, 1.0, A::offset(A::exp(s.ad_value(868)), 1.0));
        }

        if (s.v[761] != 0.0) {
            s.store_div_ad_lhs(813, A::sub(A::sub(s.ad_value(871), s.ad_value(839)), A::sub(s.ad_value(769), A::mul(A::scale(s.ad_value(805), (p.p51 * 0.1)), s.ad_value(810)))), 811);
        }

        s.v[881] = if (s.v[813] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[761] != 0.0) && (s.v[881] != 0.0)) {
            s.store_mul(814, 812, 813);
        }

        s.v[882] = if (s.v[813] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[761] != 0.0) && (!(s.v[881] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_mul_ad_rhs(814, 812, A::exp(s.ad_value(813)));
        }

        if (((s.v[761] != 0.0) && (!(s.v[881] != 0.0))) && (!(s.v[882] != 0.0))) {
            s.store_mul_ad_rhs(814, 812, A::ln(A::offset(A::exp(s.ad_value(813)), 1.0)));
        }

        if (s.v[761] != 0.0) {
            s.store_div_ad_lhs(868, A::sub(s.ad_value(871), s.ad_value(869)), 805);
        }

        s.v[883] = if (s.v[868] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[761] != 0.0) && (s.v[883] != 0.0)) {
            s.store_scalar(840, 0.0);
        }

        s.v[884] = if (s.v[868] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[761] != 0.0) && (!(s.v[883] != 0.0))) && (s.v[884] != 0.0)) {
            s.store_scalar(840, 1.0);
        }

        if (((s.v[761] != 0.0) && (!(s.v[883] != 0.0))) && (!(s.v[884] != 0.0))) {
            s.store_div_from_scalar_ad(840, 1.0, A::offset(A::exp(s.ad_value(868)), 1.0));
        }

        if (s.v[761] != 0.0) {
            s.store_div_ad_lhs(841, A::sub(A::sub(s.ad_value(771), s.ad_value(837)), A::sub(s.ad_value(769), A::mul(A::scale(s.ad_value(805), (p.p51 * 0.1)), s.ad_value(840)))), 811);
        }

        s.v[885] = if (s.v[841] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[761] != 0.0) && (s.v[885] != 0.0)) {
            s.store_mul(842, 812, 841);
        }

        s.v[886] = if (s.v[841] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[761] != 0.0) && (!(s.v[885] != 0.0))) && (s.v[886] != 0.0)) {
            s.store_mul_ad_rhs(842, 812, A::exp(s.ad_value(841)));
        }

        if (((s.v[761] != 0.0) && (!(s.v[885] != 0.0))) && (!(s.v[886] != 0.0))) {
            s.store_mul_ad_rhs(842, 812, A::ln(A::offset(A::exp(s.ad_value(841)), 1.0)));
        }

        if (s.v[761] != 0.0) {
            s.store_div_ad_lhs(843, A::sub(s.ad_value(814), s.ad_value(842)), 782);
        }

        if (s.v[761] != 0.0) {
            s.store_div(869, 843, 835);
        }

        if (s.v[761] != 0.0) {
            let assign12030_ad_e12462: A = A::pow(A::offset(A::pow({
                if (!(p.p52 == 0.0)) {
                    A::mul(s.ad_value(869), A::tanh(A::scale(s.ad_value(869), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(869)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794)));
            s.store_div_ad_rhs(844, 869, assign12030_ad_e12462);
        }

        if (s.v[761] != 0.0) {
            s.store_mul(845, 817, 844);
        }

        if (s.v[761] != 0.0) {
            s.store_mul_ad_lhs(763, A::mul(A::mul(A::scale(A::mul(A::mul(s.ad_value(803), s.ad_value(780)), s.ad_value(802)), 0.5), A::add(s.ad_value(814), s.ad_value(842))), s.ad_value(845)), 804);
        }

        if (s.v[761] != 0.0) {
            s.store_div_ad_rhs(818, 787, A::scale(s.ad_value(779), 2.302585092994046));
        }

        if (s.v[761] != 0.0) {
            s.store_mul_ad_lhs(820, A::scale(s.ad_value(818), 2.0), 779);
        }

        if (s.v[761] != 0.0) {
            s.store_mul(821, 782, 820);
        }

        if (s.v[761] != 0.0) {
            s.store_sub_ad_rhs(873, 808, A::scale(s.ad_value(805), (p.p51 * 0.5)));
        }

        if (s.v[761] != 0.0) {
            let assign12100_ad_e12566: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(771), s.ad_value(871)), A::mul(A::sub(s.ad_value(771), s.ad_value(871)), A::tanh(A::scale(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(771), s.ad_value(871)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(771), s.ad_value(871)), A::sub(s.ad_value(771), s.ad_value(871))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(872, A::sub(assign12100_ad_e12566, s.ad_value(873)), 805);
        }

        s.v[887] = if (s.v[872] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[761] != 0.0) && (s.v[887] != 0.0)) {
            s.store_scalar(830, 0.0);
        }

        s.v[888] = if (s.v[872] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[761] != 0.0) && (!(s.v[887] != 0.0))) && (s.v[888] != 0.0)) {
            s.store_scalar(830, 1.0);
        }

        if (((s.v[761] != 0.0) && (!(s.v[887] != 0.0))) && (!(s.v[888] != 0.0))) {
            s.store_div_from_scalar_ad(830, 1.0, A::offset(A::exp(s.ad_value(872)), 1.0));
        }

        if (s.v[761] != 0.0) {
            let assign12160_ad_e12654: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(771), s.ad_value(871)), A::mul(A::sub(s.ad_value(771), s.ad_value(871)), A::tanh(A::scale(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(771), s.ad_value(871)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(771), s.ad_value(871)), A::sub(s.ad_value(771), s.ad_value(871))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(831, A::sub(assign12160_ad_e12654, A::sub(s.ad_value(808), A::mul(A::scale(s.ad_value(805), (p.p51 * 0.1)), s.ad_value(830)))), 820);
        }

        s.v[889] = if (s.v[831] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[761] != 0.0) && (s.v[889] != 0.0)) {
            s.store_mul(832, 821, 831);
        }

        s.v[890] = if (s.v[831] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[761] != 0.0) && (!(s.v[889] != 0.0))) && (s.v[890] != 0.0)) {
            s.store_mul_ad_rhs(832, 821, A::exp(s.ad_value(831)));
        }

        if (((s.v[761] != 0.0) && (!(s.v[889] != 0.0))) && (!(s.v[890] != 0.0))) {
            s.store_mul_ad_rhs(832, 821, A::ln(A::offset(A::exp(s.ad_value(831)), 1.0)));
        }

        if (s.v[761] != 0.0) {
            s.store_div(824, 793, 826);
        }

    }

    pub(super) fn stamp_transient_block_10(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[761] != 0.0) {
            s.store_mul_ad_rhs(825, 792, A::div(A::offset(A::mul(s.ad_value(800), s.ad_value(778)), 1.0), A::offset(A::mul(s.ad_value(800), s.ad_value(777)), 1.0)));
        }

        if (s.v[761] != 0.0) {
            s.store_div_ad_lhs(846, A::mul(s.ad_value(825), s.ad_value(781)), 824);
        }

        if (s.v[761] != 0.0) {
            s.store_sub_ad_lhs(847, A::mul(s.ad_value(846), A::sqrt(A::offset(A::div(A::div(A::scale(s.ad_value(832), 2.0), s.ad_value(782)), s.ad_value(846)), 1.0))), 846);
        }

        if (s.v[761] != 0.0) {
            s.store_add_ad(848, A::mul(s.ad_value(847), A::sub_from_scalar(1.0, s.ad_value(830))), A::mul(s.ad_value(820), s.ad_value(830)));
        }

        if (s.v[761] != 0.0) {
            let assign12270_ad_e12829: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(s.ad_value(772), s.ad_value(848)), A::mul(A::neg(A::div(s.ad_value(772), s.ad_value(848))), A::tanh(A::scale(A::neg(A::div(s.ad_value(772), s.ad_value(848))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(s.ad_value(772), s.ad_value(848)), A::sqrt(A::offset(A::mul(A::neg(A::div(s.ad_value(772), s.ad_value(848))), A::neg(A::div(s.ad_value(772), s.ad_value(848)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(849, 1.0, A::pow(A::offset(A::pow(assign12270_ad_e12829, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794))));
        }

        if (s.v[761] != 0.0) {
            s.store_mul(850, 772, 849);
        }

        if (s.v[761] != 0.0) {
            let assign12290_ad_e12910: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(A::neg(s.ad_value(772)), s.ad_value(848)), A::mul(A::neg(A::div(A::neg(s.ad_value(772)), s.ad_value(848))), A::tanh(A::scale(A::neg(A::div(A::neg(s.ad_value(772)), s.ad_value(848))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(A::neg(s.ad_value(772)), s.ad_value(848)), A::sqrt(A::offset(A::mul(A::neg(A::div(A::neg(s.ad_value(772)), s.ad_value(848))), A::neg(A::div(A::neg(s.ad_value(772)), s.ad_value(848)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(851, 1.0, A::pow(A::offset(A::pow(assign12290_ad_e12910, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794))));
        }

        if (s.v[761] != 0.0) {
            s.store_mul_ad_lhs(852, A::neg(s.ad_value(772)), 851);
        }

        if (s.v[761] != 0.0) {
            s.store_div_ad_lhs(872, A::sub(s.ad_value(771), s.ad_value(873)), 805);
        }

        s.v[891] = if (s.v[872] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[761] != 0.0) && (s.v[891] != 0.0)) {
            s.store_scalar(819, 0.0);
        }

        s.v[892] = if (s.v[872] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[761] != 0.0) && (!(s.v[891] != 0.0))) && (s.v[892] != 0.0)) {
            s.store_scalar(819, 1.0);
        }

        if (((s.v[761] != 0.0) && (!(s.v[891] != 0.0))) && (!(s.v[892] != 0.0))) {
            s.store_div_from_scalar_ad(819, 1.0, A::offset(A::exp(s.ad_value(872)), 1.0));
        }

        if (s.v[761] != 0.0) {
            s.store_div_ad_lhs(822, A::sub(A::sub(s.ad_value(871), s.ad_value(852)), A::sub(s.ad_value(808), A::mul(A::scale(s.ad_value(805), (p.p51 * 0.1)), s.ad_value(819)))), 820);
        }

        s.v[893] = if (s.v[822] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[761] != 0.0) && (s.v[893] != 0.0)) {
            s.store_mul(823, 821, 822);
        }

        s.v[894] = if (s.v[822] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[761] != 0.0) && (!(s.v[893] != 0.0))) && (s.v[894] != 0.0)) {
            s.store_mul_ad_rhs(823, 821, A::exp(s.ad_value(822)));
        }

        if (((s.v[761] != 0.0) && (!(s.v[893] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_mul_ad_rhs(823, 821, A::ln(A::offset(A::exp(s.ad_value(822)), 1.0)));
        }

        if (s.v[761] != 0.0) {
            s.store_div_ad_lhs(872, A::sub(s.ad_value(871), s.ad_value(873)), 805);
        }

        s.v[895] = if (s.v[872] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[761] != 0.0) && (s.v[895] != 0.0)) {
            s.store_scalar(853, 0.0);
        }

        s.v[896] = if (s.v[872] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[761] != 0.0) && (!(s.v[895] != 0.0))) && (s.v[896] != 0.0)) {
            s.store_scalar(853, 1.0);
        }

        if (((s.v[761] != 0.0) && (!(s.v[895] != 0.0))) && (!(s.v[896] != 0.0))) {
            s.store_div_from_scalar_ad(853, 1.0, A::offset(A::exp(s.ad_value(872)), 1.0));
        }

        if (s.v[761] != 0.0) {
            s.store_div_ad_lhs(854, A::sub(A::sub(s.ad_value(771), s.ad_value(850)), A::sub(s.ad_value(808), A::mul(A::scale(s.ad_value(805), (p.p51 * 0.1)), s.ad_value(853)))), 820);
        }

        s.v[897] = if (s.v[854] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[761] != 0.0) && (s.v[897] != 0.0)) {
            s.store_mul(855, 821, 854);
        }

        s.v[898] = if (s.v[854] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[761] != 0.0) && (!(s.v[897] != 0.0))) && (s.v[898] != 0.0)) {
            s.store_mul_ad_rhs(855, 821, A::exp(s.ad_value(854)));
        }

        if (((s.v[761] != 0.0) && (!(s.v[897] != 0.0))) && (!(s.v[898] != 0.0))) {
            s.store_mul_ad_rhs(855, 821, A::ln(A::offset(A::exp(s.ad_value(854)), 1.0)));
        }

        if (s.v[761] != 0.0) {
            s.store_offset_ad(856, A::square(s.ad_value(823)), 1e-38);
        }

        if (s.v[761] != 0.0) {
            s.store_offset_ad(857, A::mul(s.ad_value(856), s.ad_value(823)), 1e-57);
        }

        if (s.v[761] != 0.0) {
            s.store_offset_ad(858, A::square(s.ad_value(855)), 1e-38);
        }

        if (s.v[761] != 0.0) {
            s.store_offset_ad(859, A::mul(s.ad_value(858), s.ad_value(855)), 1e-57);
        }

        if (s.v[761] != 0.0) {
            s.store_offset_ad(860, A::mul(s.ad_value(823), s.ad_value(855)), 1e-38);
        }

        if (s.v[761] != 0.0) {
            s.store_div_ad(861, A::scale(A::add(A::add(s.ad_value(856), s.ad_value(858)), s.ad_value(860)), (2.0 / 3.0)), A::offset(A::add(s.ad_value(823), s.ad_value(855)), 2e-19));
        }

        if (s.v[761] != 0.0) {
            s.store_div_ad(862, A::scale(A::add(A::add(A::add(A::scale(s.ad_value(857), 2.0), A::scale(s.ad_value(859), 3.0)), A::mul(A::scale(s.ad_value(856), 4.0), s.ad_value(855))), A::mul(A::scale(s.ad_value(858), 6.0), s.ad_value(823))), 2.0), A::scale(A::add(A::add(s.ad_value(856), s.ad_value(858)), A::scale(s.ad_value(860), 2.0)), 15.0));
        }

        if (s.v[761] != 0.0) {
            s.store_sub(863, 861, 862);
        }

        if (s.v[761] != 0.0) {
            s.copy_ad(864, 862);
        }

        if (s.v[761] != 0.0) {
            s.store_mul_ad_lhs(764, A::mul(A::mul(A::mul(A::mul(s.ad_value(780), s.ad_value(802)), s.ad_value(781)), s.ad_value(803)), s.ad_value(863)), 804);
        }

        if (s.v[761] != 0.0) {
            s.store_mul_ad_lhs(765, A::mul(A::mul(A::mul(A::mul(s.ad_value(780), s.ad_value(802)), s.ad_value(781)), s.ad_value(803)), s.ad_value(864)), 804);
        }

        s.v[899] = if (s.v[773] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[761] != 0.0) && (s.v[899] != 0.0)) {
            s.store_div_ad_lhs(865, A::sub(s.ad_value(774), A::sub(s.ad_value(808), A::scale(s.ad_value(805), (p.p51 * 0.5)))), 820);
        }

        s.v[900] = if (s.v[865] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[761] != 0.0) && (s.v[899] != 0.0)) && (s.v[900] != 0.0)) {
            s.copy_ad(868, 865);
        }

        s.v[901] = if (s.v[865] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[761] != 0.0) && (s.v[899] != 0.0)) && (!(s.v[900] != 0.0))) && (s.v[901] != 0.0)) {
            s.store_exp(868, 865);
        }

        if ((((s.v[761] != 0.0) && (s.v[899] != 0.0)) && (!(s.v[900] != 0.0))) && (!(s.v[901] != 0.0))) {
            s.store_ln_ad(868, A::offset(A::exp(s.ad_value(865)), 1.0));
        }

        if ((s.v[761] != 0.0) && (s.v[899] != 0.0)) {
            s.store_mul_ad_lhs(766, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(780), s.ad_value(802)), s.ad_value(803)), s.ad_value(784)), s.ad_value(820)), s.ad_value(868)), 804);
        }

        if ((s.v[761] != 0.0) && (s.v[899] != 0.0)) {
            s.store_div_ad_lhs(866, A::sub(s.ad_value(775), A::sub(s.ad_value(808), A::scale(s.ad_value(805), (p.p51 * 0.5)))), 820);
        }

        s.v[902] = if (s.v[866] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[761] != 0.0) && (s.v[899] != 0.0)) && (s.v[902] != 0.0)) {
            s.copy_ad(868, 866);
        }

        s.v[903] = if (s.v[866] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[761] != 0.0) && (s.v[899] != 0.0)) && (!(s.v[902] != 0.0))) && (s.v[903] != 0.0)) {
            s.store_exp(868, 866);
        }

        if ((((s.v[761] != 0.0) && (s.v[899] != 0.0)) && (!(s.v[902] != 0.0))) && (!(s.v[903] != 0.0))) {
            s.store_ln_ad(868, A::offset(A::exp(s.ad_value(866)), 1.0));
        }

        if ((s.v[761] != 0.0) && (s.v[899] != 0.0)) {
            s.store_mul_ad_lhs(767, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(780), s.ad_value(802)), s.ad_value(803)), s.ad_value(785)), s.ad_value(820)), s.ad_value(868)), 804);
        }

        if ((s.v[761] != 0.0) && (!(s.v[899] != 0.0))) {
            s.store_scalar(766, 0.0);
        }

        if ((s.v[761] != 0.0) && (!(s.v[899] != 0.0))) {
            s.store_scalar(767, 0.0);
        }

        s.v[904] = if (s.v[776] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[761] != 0.0) && (s.v[904] != 0.0)) {
            s.store_div_ad_lhs(867, A::sub(s.ad_value(771), A::sub(s.ad_value(808), A::scale(s.ad_value(805), (p.p51 * 0.5)))), 820);
        }

        s.v[905] = if (s.v[867] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[761] != 0.0) && (s.v[904] != 0.0)) && (s.v[905] != 0.0)) {
            s.copy_ad(868, 867);
        }

        s.v[906] = if (s.v[867] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[761] != 0.0) && (s.v[904] != 0.0)) && (!(s.v[905] != 0.0))) && (s.v[906] != 0.0)) {
            s.store_exp(868, 867);
        }

        if ((((s.v[761] != 0.0) && (s.v[904] != 0.0)) && (!(s.v[905] != 0.0))) && (!(s.v[906] != 0.0))) {
            s.store_ln_ad(868, A::offset(A::exp(s.ad_value(867)), 1.0));
        }

        if ((s.v[761] != 0.0) && (s.v[904] != 0.0)) {
            s.store_mul_ad_lhs(768, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(780), s.ad_value(802)), s.ad_value(803)), s.ad_value(783)), s.ad_value(820)), s.ad_value(868)), 804);
        }

        if ((s.v[761] != 0.0) && (!(s.v[904] != 0.0))) {
            s.store_scalar(768, 0.0);
        }

        if (s.v[761] != 0.0) {
            s.copy_ad(762, 763);
        }

        if (s.v[761] != 0.0) {
            s.copy_ad(190, 763);
        }

        if (s.v[761] != 0.0) {
            s.copy_ad(191, 764);
        }

        if (s.v[761] != 0.0) {
            s.copy_ad(192, 765);
        }

        if (s.v[761] != 0.0) {
            s.copy_ad(193, 766);
        }

        if (s.v[761] != 0.0) {
            s.copy_ad(194, 767);
        }

        if (s.v[761] != 0.0) {
            s.copy_ad(195, 768);
        }

        if (s.v[761] != 0.0) {
            s.copy_ad(190, 762);
        }

        s.v[907] = if (p.p166 == 1.0) { 1.0 } else { 0.0 };

        s.v[166] = 0.0;

        s.v[167] = 0.0;

        s.v[168] = 0.0;

        s.v[169] = 0.0;

        s.v[170] = 0.0;

        s.v[171] = 0.0;

        s.v[908] = if (p.p79 > p.p354) { 1.0 } else { 0.0 };

        if (s.v[908] != 0.0) {
            s.store_scalar(909, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(910, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(911, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(912, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(913, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(914, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(915, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(916, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(917, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.copy_ad(918, 60);
        }

        if (s.v[908] != 0.0) {
            s.copy_ad(919, 61);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(920, p.p85);
        }

        if (s.v[908] != 0.0) {
            s.copy_ad(921, 62);
        }

        if (s.v[908] != 0.0) {
            s.copy_ad(922, 63);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(923, p.p83);
        }

        if (s.v[908] != 0.0) {
            s.copy_ad(924, 111);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(925, s.v[109]);
        }

        if (s.v[908] != 0.0) {
            s.copy_ad(926, 113);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(927, p.p0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(928, p.p79);
        }

        if (s.v[908] != 0.0) {
            s.copy_ad(929, 20);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(930, p.p84);
        }

        if (s.v[908] != 0.0) {
            s.copy_ad(931, 21);
        }

        if (s.v[908] != 0.0) {
            s.copy_ad(932, 22);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(933, p.p80);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(934, p.p94);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(935, p.p93);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(936, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(937, p.p95);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(938, p.p99);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(939, p.p90);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(940, p.p91);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(941, p.p92);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(942, p.p98);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(943, p.p97);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(944, p.p96);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(945, p.p39);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(946, p.p47);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(947, p.p45);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(948, p.p42);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(949, p.p2);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(950, p.p6);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(951, 1.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(952, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(953, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(954, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(955, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(956, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_11(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[908] != 0.0) {
            s.store_scalar(957, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(958, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(959, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(960, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(961, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(962, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(963, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(964, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(965, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(966, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(967, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(968, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(969, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(970, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(971, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(972, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(973, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(974, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(975, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(976, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(977, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(978, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(979, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(980, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(981, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(982, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(983, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(984, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(985, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(986, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(987, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(988, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(989, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(990, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(991, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(992, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(993, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(994, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(995, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(996, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(997, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(998, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(999, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(1000, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(1001, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(1002, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(1003, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(1004, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(1005, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(1006, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(1007, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(1008, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(1009, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(1010, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(1011, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(1012, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(1013, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(1014, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(1015, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(1016, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(1017, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(1018, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(1019, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_scalar(1020, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_ad(1017, &{
                if (!(p.p52 == 0.0)) {
                    A::mul(s.ad_value(919), A::tanh(A::scale(s.ad_value(919), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(919)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.v[908] != 0.0) {
            s.store_sub(1018, 918, 919);
        }

        if (s.v[908] != 0.0) {
            s.store_mul(952, 938, 926);
        }

        if (s.v[908] != 0.0) {
            s.store_add_ad(954, A::div(s.ad_value(934), A::scale(s.ad_value(926), 2.302585092994046)), A::mul(s.ad_value(937), s.ad_value(1017)));
        }

        if (s.v[908] != 0.0) {
            s.store_add_ad_rhs(955, 933, A::mul(s.ad_value(944), A::sub(s.ad_value(924), s.ad_value(925))));
        }

        if (s.v[908] != 0.0) {
            s.store_ad(973, &A::pow(A::div(s.ad_value(924), s.ad_value(925)), s.ad_value(946)));
        }

        s.v[1021] = if (s.v[945] != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[908] != 0.0) && (s.v[1021] != 0.0)) {
            s.store_div_ad_rhs(956, 1017, A::pow(A::offset(A::pow(A::div(s.ad_value(1017), s.ad_value(945)), s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941))));
        }

        if ((s.v[908] != 0.0) && (!(s.v[1021] != 0.0))) {
            s.store_scalar(956, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.store_mul_ad_lhs(953, A::sub(s.ad_value(935), A::mul(s.ad_value(956), s.ad_value(936))), 1017);
        }

        if (s.v[908] != 0.0) {
            s.store_sub(916, 955, 953);
        }

        if (s.v[908] != 0.0) {
            s.store_mul_ad_lhs(958, A::scale(s.ad_value(954), 2.0), 926);
        }

        if (s.v[908] != 0.0) {
            s.store_mul(959, 929, 958);
        }

        if (s.v[908] != 0.0) {
            s.store_sub_ad_rhs(1016, 916, A::scale(s.ad_value(952), (p.p51 * 0.5)));
        }

        if (s.v[908] != 0.0) {
            let assign14380_ad_e14211: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(918), s.ad_value(1018)), A::mul(A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh(A::scale(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(918), s.ad_value(1018)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(918), s.ad_value(1018)), A::sub(s.ad_value(918), s.ad_value(1018))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1015, A::sub(assign14380_ad_e14211, s.ad_value(1016)), 952);
        }

        s.v[1022] = if (s.v[1015] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[908] != 0.0) && (s.v[1022] != 0.0)) {
            s.store_scalar(974, 0.0);
        }

        s.v[1023] = if (s.v[1015] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[908] != 0.0) && (!(s.v[1022] != 0.0))) && (s.v[1023] != 0.0)) {
            s.store_scalar(974, 1.0);
        }

        if (((s.v[908] != 0.0) && (!(s.v[1022] != 0.0))) && (!(s.v[1023] != 0.0))) {
            s.store_div_from_scalar_ad(974, 1.0, A::offset(A::exp(s.ad_value(1015)), 1.0));
        }

        if (s.v[908] != 0.0) {
            let assign14440_ad_e14299: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(918), s.ad_value(1018)), A::mul(A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh(A::scale(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(918), s.ad_value(1018)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(918), s.ad_value(1018)), A::sub(s.ad_value(918), s.ad_value(1018))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(975, A::sub(assign14440_ad_e14299, A::sub(s.ad_value(916), A::mul(A::scale(s.ad_value(952), (p.p51 * 0.1)), s.ad_value(974)))), 958);
        }

        s.v[1024] = if (s.v[975] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[908] != 0.0) && (s.v[1024] != 0.0)) {
            s.store_mul(976, 959, 975);
        }

        s.v[1025] = if (s.v[975] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[908] != 0.0) && (!(s.v[1024] != 0.0))) && (s.v[1025] != 0.0)) {
            s.store_mul_ad_rhs(976, 959, A::exp(s.ad_value(975)));
        }

        if (((s.v[908] != 0.0) && (!(s.v[1024] != 0.0))) && (!(s.v[1025] != 0.0))) {
            s.store_mul_ad_rhs(976, 959, A::ln(A::offset(A::exp(s.ad_value(975)), 1.0)));
        }

        if (s.v[908] != 0.0) {
            s.store_div_ad_rhs(962, 940, A::mul(s.ad_value(973), A::offset(A::div(A::mul(s.ad_value(942), s.ad_value(976)), s.ad_value(929)), 1.0)));
        }

        if (s.v[908] != 0.0) {
            s.store_div_ad(963, A::mul(A::mul(s.ad_value(939), A::div(A::offset(A::mul(s.ad_value(947), s.ad_value(925)), 1.0), A::offset(A::mul(s.ad_value(947), s.ad_value(924)), 1.0))), A::offset(A::div(A::mul(s.ad_value(948), s.ad_value(1017)), s.ad_value(928)), 1.0)), A::offset(A::div(A::mul(s.ad_value(943), s.ad_value(976)), s.ad_value(929)), 1.0));
        }

        if (s.v[908] != 0.0) {
            s.store_add_ad(964, A::div(A::mul(A::mul(A::scale(s.ad_value(974), 2.0), s.ad_value(926)), s.ad_value(962)), s.ad_value(928)), A::mul(A::sub_from_scalar(1.0, s.ad_value(974)), s.ad_value(963)));
        }

        if (s.v[908] != 0.0) {
            s.store_div_ad_lhs(980, A::mul(s.ad_value(963), s.ad_value(928)), 962);
        }

        if (s.v[908] != 0.0) {
            s.store_sub_ad_lhs(981, A::mul(s.ad_value(980), A::sqrt(A::offset(A::div(A::div(A::scale(s.ad_value(976), 2.0), s.ad_value(929)), s.ad_value(980)), 1.0))), 980);
        }

        if (s.v[908] != 0.0) {
            s.store_add_ad(982, A::mul(s.ad_value(980), A::sub_from_scalar(1.0, s.ad_value(974))), A::mul(s.ad_value(958), s.ad_value(974)));
        }

        if (s.v[908] != 0.0) {
            s.store_add_ad(917, A::mul(s.ad_value(981), A::sub_from_scalar(1.0, s.ad_value(974))), A::mul(s.ad_value(958), s.ad_value(974)));
        }

        if (s.v[908] != 0.0) {
            let assign14570_ad_e14528: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(s.ad_value(919), s.ad_value(917)), A::mul(A::neg(A::div(s.ad_value(919), s.ad_value(917))), A::tanh(A::scale(A::neg(A::div(s.ad_value(919), s.ad_value(917))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(s.ad_value(919), s.ad_value(917)), A::sqrt(A::offset(A::mul(A::neg(A::div(s.ad_value(919), s.ad_value(917))), A::neg(A::div(s.ad_value(919), s.ad_value(917)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(983, 1.0, A::pow(A::offset(A::pow(assign14570_ad_e14528, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941))));
        }

        if (s.v[908] != 0.0) {
            s.store_mul(984, 919, 983);
        }

        if (s.v[908] != 0.0) {
            let assign14590_ad_e14609: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(A::neg(s.ad_value(919)), s.ad_value(917)), A::mul(A::neg(A::div(A::neg(s.ad_value(919)), s.ad_value(917))), A::tanh(A::scale(A::neg(A::div(A::neg(s.ad_value(919)), s.ad_value(917))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(A::neg(s.ad_value(919)), s.ad_value(917)), A::sqrt(A::offset(A::mul(A::neg(A::div(A::neg(s.ad_value(919)), s.ad_value(917))), A::neg(A::div(A::neg(s.ad_value(919)), s.ad_value(917)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(985, 1.0, A::pow(A::offset(A::pow(assign14590_ad_e14609, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941))));
        }

        if (s.v[908] != 0.0) {
            s.store_mul_ad_lhs(986, A::neg(s.ad_value(919)), 985);
        }

        if (s.v[908] != 0.0) {
            s.store_div_ad_lhs(1015, A::sub(s.ad_value(918), s.ad_value(1016)), 952);
        }

        s.v[1026] = if (s.v[1015] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[908] != 0.0) && (s.v[1026] != 0.0)) {
            s.store_scalar(957, 0.0);
        }

        s.v[1027] = if (s.v[1015] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[908] != 0.0) && (!(s.v[1026] != 0.0))) && (s.v[1027] != 0.0)) {
            s.store_scalar(957, 1.0);
        }

        if (((s.v[908] != 0.0) && (!(s.v[1026] != 0.0))) && (!(s.v[1027] != 0.0))) {
            s.store_div_from_scalar_ad(957, 1.0, A::offset(A::exp(s.ad_value(1015)), 1.0));
        }

        if (s.v[908] != 0.0) {
            s.store_div_ad_lhs(960, A::sub(A::sub(s.ad_value(1018), s.ad_value(986)), A::sub(s.ad_value(916), A::mul(A::scale(s.ad_value(952), (p.p51 * 0.1)), s.ad_value(957)))), 958);
        }

        s.v[1028] = if (s.v[960] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[908] != 0.0) && (s.v[1028] != 0.0)) {
            s.store_mul(961, 959, 960);
        }

        s.v[1029] = if (s.v[960] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[908] != 0.0) && (!(s.v[1028] != 0.0))) && (s.v[1029] != 0.0)) {
            s.store_mul_ad_rhs(961, 959, A::exp(s.ad_value(960)));
        }

        if (((s.v[908] != 0.0) && (!(s.v[1028] != 0.0))) && (!(s.v[1029] != 0.0))) {
            s.store_mul_ad_rhs(961, 959, A::ln(A::offset(A::exp(s.ad_value(960)), 1.0)));
        }

        if (s.v[908] != 0.0) {
            s.store_div_ad_lhs(1015, A::sub(s.ad_value(1018), s.ad_value(1016)), 952);
        }

        s.v[1030] = if (s.v[1015] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[908] != 0.0) && (s.v[1030] != 0.0)) {
            s.store_scalar(987, 0.0);
        }

        s.v[1031] = if (s.v[1015] < (-50.0)) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_12(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[908] != 0.0) && (!(s.v[1030] != 0.0))) && (s.v[1031] != 0.0)) {
            s.store_scalar(987, 1.0);
        }

        if (((s.v[908] != 0.0) && (!(s.v[1030] != 0.0))) && (!(s.v[1031] != 0.0))) {
            s.store_div_from_scalar_ad(987, 1.0, A::offset(A::exp(s.ad_value(1015)), 1.0));
        }

        if (s.v[908] != 0.0) {
            s.store_div_ad_lhs(988, A::sub(A::sub(s.ad_value(918), s.ad_value(984)), A::sub(s.ad_value(916), A::mul(A::scale(s.ad_value(952), (p.p51 * 0.1)), s.ad_value(987)))), 958);
        }

        s.v[1032] = if (s.v[988] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[908] != 0.0) && (s.v[1032] != 0.0)) {
            s.store_mul(989, 959, 988);
        }

        s.v[1033] = if (s.v[988] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[908] != 0.0) && (!(s.v[1032] != 0.0))) && (s.v[1033] != 0.0)) {
            s.store_mul_ad_rhs(989, 959, A::exp(s.ad_value(988)));
        }

        if (((s.v[908] != 0.0) && (!(s.v[1032] != 0.0))) && (!(s.v[1033] != 0.0))) {
            s.store_mul_ad_rhs(989, 959, A::ln(A::offset(A::exp(s.ad_value(988)), 1.0)));
        }

        if (s.v[908] != 0.0) {
            s.store_div_ad_lhs(990, A::sub(s.ad_value(961), s.ad_value(989)), 929);
        }

        if (s.v[908] != 0.0) {
            s.store_div(1016, 990, 982);
        }

        if (s.v[908] != 0.0) {
            let assign14870_ad_e14886: A = A::pow(A::offset(A::pow({
                if (!(p.p52 == 0.0)) {
                    A::mul(s.ad_value(1016), A::tanh(A::scale(s.ad_value(1016), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(1016)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
            s.store_div_ad_rhs(991, 1016, assign14870_ad_e14886);
        }

        if (s.v[908] != 0.0) {
            s.store_mul(992, 964, 991);
        }

        if (s.v[908] != 0.0) {
            s.store_mul_ad_lhs(910, A::mul(A::mul(A::scale(A::mul(A::mul(s.ad_value(950), s.ad_value(927)), s.ad_value(949)), 0.5), A::add(s.ad_value(961), s.ad_value(989))), s.ad_value(992)), 951);
        }

        if (s.v[908] != 0.0) {
            s.store_div_ad_rhs(965, 934, A::scale(s.ad_value(926), 2.302585092994046));
        }

        if (s.v[908] != 0.0) {
            s.store_mul_ad_lhs(967, A::scale(s.ad_value(965), 2.0), 926);
        }

        if (s.v[908] != 0.0) {
            s.store_mul(968, 929, 967);
        }

        if (s.v[908] != 0.0) {
            s.store_sub_ad_rhs(1020, 955, A::scale(s.ad_value(952), (p.p51 * 0.5)));
        }

        if (s.v[908] != 0.0) {
            let assign14940_ad_e14990: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(918), s.ad_value(1018)), A::mul(A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh(A::scale(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(918), s.ad_value(1018)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(918), s.ad_value(1018)), A::sub(s.ad_value(918), s.ad_value(1018))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1019, A::sub(assign14940_ad_e14990, s.ad_value(1020)), 952);
        }

        s.v[1034] = if (s.v[1019] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[908] != 0.0) && (s.v[1034] != 0.0)) {
            s.store_scalar(977, 0.0);
        }

        s.v[1035] = if (s.v[1019] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[908] != 0.0) && (!(s.v[1034] != 0.0))) && (s.v[1035] != 0.0)) {
            s.store_scalar(977, 1.0);
        }

        if (((s.v[908] != 0.0) && (!(s.v[1034] != 0.0))) && (!(s.v[1035] != 0.0))) {
            s.store_div_from_scalar_ad(977, 1.0, A::offset(A::exp(s.ad_value(1019)), 1.0));
        }

        if (s.v[908] != 0.0) {
            let assign15000_ad_e15078: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(918), s.ad_value(1018)), A::mul(A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh(A::scale(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(918), s.ad_value(1018)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(918), s.ad_value(1018)), A::sub(s.ad_value(918), s.ad_value(1018))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(978, A::sub(assign15000_ad_e15078, A::sub(s.ad_value(955), A::mul(A::scale(s.ad_value(952), (p.p51 * 0.1)), s.ad_value(977)))), 967);
        }

        s.v[1036] = if (s.v[978] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[908] != 0.0) && (s.v[1036] != 0.0)) {
            s.store_mul(979, 968, 978);
        }

        s.v[1037] = if (s.v[978] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[908] != 0.0) && (!(s.v[1036] != 0.0))) && (s.v[1037] != 0.0)) {
            s.store_mul_ad_rhs(979, 968, A::exp(s.ad_value(978)));
        }

        if (((s.v[908] != 0.0) && (!(s.v[1036] != 0.0))) && (!(s.v[1037] != 0.0))) {
            s.store_mul_ad_rhs(979, 968, A::ln(A::offset(A::exp(s.ad_value(978)), 1.0)));
        }

        if (s.v[908] != 0.0) {
            s.store_div(971, 940, 973);
        }

        if (s.v[908] != 0.0) {
            s.store_mul_ad_rhs(972, 939, A::div(A::offset(A::mul(s.ad_value(947), s.ad_value(925)), 1.0), A::offset(A::mul(s.ad_value(947), s.ad_value(924)), 1.0)));
        }

        if (s.v[908] != 0.0) {
            s.store_div_ad_lhs(993, A::mul(s.ad_value(972), s.ad_value(928)), 971);
        }

        if (s.v[908] != 0.0) {
            s.store_sub_ad_lhs(994, A::mul(s.ad_value(993), A::sqrt(A::offset(A::div(A::div(A::scale(s.ad_value(979), 2.0), s.ad_value(929)), s.ad_value(993)), 1.0))), 993);
        }

        if (s.v[908] != 0.0) {
            s.store_add_ad(995, A::mul(s.ad_value(994), A::sub_from_scalar(1.0, s.ad_value(977))), A::mul(s.ad_value(967), s.ad_value(977)));
        }

        if (s.v[908] != 0.0) {
            let assign15110_ad_e15253: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(s.ad_value(919), s.ad_value(995)), A::mul(A::neg(A::div(s.ad_value(919), s.ad_value(995))), A::tanh(A::scale(A::neg(A::div(s.ad_value(919), s.ad_value(995))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(s.ad_value(919), s.ad_value(995)), A::sqrt(A::offset(A::mul(A::neg(A::div(s.ad_value(919), s.ad_value(995))), A::neg(A::div(s.ad_value(919), s.ad_value(995)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(996, 1.0, A::pow(A::offset(A::pow(assign15110_ad_e15253, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941))));
        }

        if (s.v[908] != 0.0) {
            s.store_mul(997, 919, 996);
        }

        if (s.v[908] != 0.0) {
            let assign15130_ad_e15334: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(A::neg(s.ad_value(919)), s.ad_value(995)), A::mul(A::neg(A::div(A::neg(s.ad_value(919)), s.ad_value(995))), A::tanh(A::scale(A::neg(A::div(A::neg(s.ad_value(919)), s.ad_value(995))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(A::neg(s.ad_value(919)), s.ad_value(995)), A::sqrt(A::offset(A::mul(A::neg(A::div(A::neg(s.ad_value(919)), s.ad_value(995))), A::neg(A::div(A::neg(s.ad_value(919)), s.ad_value(995)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(998, 1.0, A::pow(A::offset(A::pow(assign15130_ad_e15334, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941))));
        }

        if (s.v[908] != 0.0) {
            s.store_mul_ad_lhs(999, A::neg(s.ad_value(919)), 998);
        }

        if (s.v[908] != 0.0) {
            s.store_div_ad_lhs(1019, A::sub(s.ad_value(918), s.ad_value(1020)), 952);
        }

        s.v[1038] = if (s.v[1019] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[908] != 0.0) && (s.v[1038] != 0.0)) {
            s.store_scalar(966, 0.0);
        }

        s.v[1039] = if (s.v[1019] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[908] != 0.0) && (!(s.v[1038] != 0.0))) && (s.v[1039] != 0.0)) {
            s.store_scalar(966, 1.0);
        }

        if (((s.v[908] != 0.0) && (!(s.v[1038] != 0.0))) && (!(s.v[1039] != 0.0))) {
            s.store_div_from_scalar_ad(966, 1.0, A::offset(A::exp(s.ad_value(1019)), 1.0));
        }

        if (s.v[908] != 0.0) {
            s.store_div_ad_lhs(969, A::sub(A::sub(s.ad_value(1018), s.ad_value(999)), A::sub(s.ad_value(955), A::mul(A::scale(s.ad_value(952), (p.p51 * 0.1)), s.ad_value(966)))), 967);
        }

        s.v[1040] = if (s.v[969] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[908] != 0.0) && (s.v[1040] != 0.0)) {
            s.store_mul(970, 968, 969);
        }

        s.v[1041] = if (s.v[969] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[908] != 0.0) && (!(s.v[1040] != 0.0))) && (s.v[1041] != 0.0)) {
            s.store_mul_ad_rhs(970, 968, A::exp(s.ad_value(969)));
        }

        if (((s.v[908] != 0.0) && (!(s.v[1040] != 0.0))) && (!(s.v[1041] != 0.0))) {
            s.store_mul_ad_rhs(970, 968, A::ln(A::offset(A::exp(s.ad_value(969)), 1.0)));
        }

        if (s.v[908] != 0.0) {
            s.store_div_ad_lhs(1019, A::sub(s.ad_value(1018), s.ad_value(1020)), 952);
        }

        s.v[1042] = if (s.v[1019] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[908] != 0.0) && (s.v[1042] != 0.0)) {
            s.store_scalar(1000, 0.0);
        }

        s.v[1043] = if (s.v[1019] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[908] != 0.0) && (!(s.v[1042] != 0.0))) && (s.v[1043] != 0.0)) {
            s.store_scalar(1000, 1.0);
        }

        if (((s.v[908] != 0.0) && (!(s.v[1042] != 0.0))) && (!(s.v[1043] != 0.0))) {
            s.store_div_from_scalar_ad(1000, 1.0, A::offset(A::exp(s.ad_value(1019)), 1.0));
        }

        if (s.v[908] != 0.0) {
            s.store_div_ad_lhs(1001, A::sub(A::sub(s.ad_value(918), s.ad_value(997)), A::sub(s.ad_value(955), A::mul(A::scale(s.ad_value(952), (p.p51 * 0.1)), s.ad_value(1000)))), 967);
        }

        s.v[1044] = if (s.v[1001] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[908] != 0.0) && (s.v[1044] != 0.0)) {
            s.store_mul(1002, 968, 1001);
        }

        s.v[1045] = if (s.v[1001] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[908] != 0.0) && (!(s.v[1044] != 0.0))) && (s.v[1045] != 0.0)) {
            s.store_mul_ad_rhs(1002, 968, A::exp(s.ad_value(1001)));
        }

        if (((s.v[908] != 0.0) && (!(s.v[1044] != 0.0))) && (!(s.v[1045] != 0.0))) {
            s.store_mul_ad_rhs(1002, 968, A::ln(A::offset(A::exp(s.ad_value(1001)), 1.0)));
        }

        if (s.v[908] != 0.0) {
            s.store_offset_ad(1003, A::square(s.ad_value(970)), 1e-38);
        }

        if (s.v[908] != 0.0) {
            s.store_offset_ad(1004, A::mul(s.ad_value(1003), s.ad_value(970)), 1e-57);
        }

        if (s.v[908] != 0.0) {
            s.store_offset_ad(1005, A::square(s.ad_value(1002)), 1e-38);
        }

        if (s.v[908] != 0.0) {
            s.store_offset_ad(1006, A::mul(s.ad_value(1005), s.ad_value(1002)), 1e-57);
        }

        if (s.v[908] != 0.0) {
            s.store_offset_ad(1007, A::mul(s.ad_value(970), s.ad_value(1002)), 1e-38);
        }

        if (s.v[908] != 0.0) {
            s.store_div_ad(1008, A::scale(A::add(A::add(s.ad_value(1003), s.ad_value(1005)), s.ad_value(1007)), (2.0 / 3.0)), A::offset(A::add(s.ad_value(970), s.ad_value(1002)), 2e-19));
        }

        if (s.v[908] != 0.0) {
            s.store_div_ad(1009, A::scale(A::add(A::add(A::add(A::scale(s.ad_value(1004), 2.0), A::scale(s.ad_value(1006), 3.0)), A::mul(A::scale(s.ad_value(1003), 4.0), s.ad_value(1002))), A::mul(A::scale(s.ad_value(1005), 6.0), s.ad_value(970))), 2.0), A::scale(A::add(A::add(s.ad_value(1003), s.ad_value(1005)), A::scale(s.ad_value(1007), 2.0)), 15.0));
        }

        if (s.v[908] != 0.0) {
            s.store_sub(1010, 1008, 1009);
        }

        if (s.v[908] != 0.0) {
            s.copy_ad(1011, 1009);
        }

        if (s.v[908] != 0.0) {
            s.store_mul_ad_lhs(911, A::mul(A::mul(A::mul(A::mul(s.ad_value(927), s.ad_value(949)), s.ad_value(928)), s.ad_value(950)), s.ad_value(1010)), 951);
        }

        if (s.v[908] != 0.0) {
            s.store_mul_ad_lhs(912, A::mul(A::mul(A::mul(A::mul(s.ad_value(927), s.ad_value(949)), s.ad_value(928)), s.ad_value(950)), s.ad_value(1011)), 951);
        }

        s.v[1046] = if (s.v[920] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[908] != 0.0) && (s.v[1046] != 0.0)) {
            s.store_div_ad_lhs(1012, A::sub(s.ad_value(921), A::sub(s.ad_value(955), A::scale(s.ad_value(952), (p.p51 * 0.5)))), 967);
        }

        s.v[1047] = if (s.v[1012] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[908] != 0.0) && (s.v[1046] != 0.0)) && (s.v[1047] != 0.0)) {
            s.copy_ad(1015, 1012);
        }

        s.v[1048] = if (s.v[1012] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[908] != 0.0) && (s.v[1046] != 0.0)) && (!(s.v[1047] != 0.0))) && (s.v[1048] != 0.0)) {
            s.store_exp(1015, 1012);
        }

        if ((((s.v[908] != 0.0) && (s.v[1046] != 0.0)) && (!(s.v[1047] != 0.0))) && (!(s.v[1048] != 0.0))) {
            s.store_ln_ad(1015, A::offset(A::exp(s.ad_value(1012)), 1.0));
        }

        if ((s.v[908] != 0.0) && (s.v[1046] != 0.0)) {
            s.store_mul_ad_lhs(913, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(927), s.ad_value(949)), s.ad_value(950)), s.ad_value(931)), s.ad_value(967)), s.ad_value(1015)), 951);
        }

        if ((s.v[908] != 0.0) && (s.v[1046] != 0.0)) {
            s.store_div_ad_lhs(1013, A::sub(s.ad_value(922), A::sub(s.ad_value(955), A::scale(s.ad_value(952), (p.p51 * 0.5)))), 967);
        }

        s.v[1049] = if (s.v[1013] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[908] != 0.0) && (s.v[1046] != 0.0)) && (s.v[1049] != 0.0)) {
            s.copy_ad(1015, 1013);
        }

        s.v[1050] = if (s.v[1013] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[908] != 0.0) && (s.v[1046] != 0.0)) && (!(s.v[1049] != 0.0))) && (s.v[1050] != 0.0)) {
            s.store_exp(1015, 1013);
        }

        if ((((s.v[908] != 0.0) && (s.v[1046] != 0.0)) && (!(s.v[1049] != 0.0))) && (!(s.v[1050] != 0.0))) {
            s.store_ln_ad(1015, A::offset(A::exp(s.ad_value(1013)), 1.0));
        }

        if ((s.v[908] != 0.0) && (s.v[1046] != 0.0)) {
            s.store_mul_ad_lhs(914, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(927), s.ad_value(949)), s.ad_value(950)), s.ad_value(932)), s.ad_value(967)), s.ad_value(1015)), 951);
        }

        if ((s.v[908] != 0.0) && (!(s.v[1046] != 0.0))) {
            s.store_scalar(913, 0.0);
        }

        if ((s.v[908] != 0.0) && (!(s.v[1046] != 0.0))) {
            s.store_scalar(914, 0.0);
        }

        s.v[1051] = if (s.v[923] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[908] != 0.0) && (s.v[1051] != 0.0)) {
            s.store_div_ad_lhs(1014, A::sub(s.ad_value(918), A::sub(s.ad_value(955), A::scale(s.ad_value(952), (p.p51 * 0.5)))), 967);
        }

        s.v[1052] = if (s.v[1014] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[908] != 0.0) && (s.v[1051] != 0.0)) && (s.v[1052] != 0.0)) {
            s.copy_ad(1015, 1014);
        }

        s.v[1053] = if (s.v[1014] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[908] != 0.0) && (s.v[1051] != 0.0)) && (!(s.v[1052] != 0.0))) && (s.v[1053] != 0.0)) {
            s.store_exp(1015, 1014);
        }

        if ((((s.v[908] != 0.0) && (s.v[1051] != 0.0)) && (!(s.v[1052] != 0.0))) && (!(s.v[1053] != 0.0))) {
            s.store_ln_ad(1015, A::offset(A::exp(s.ad_value(1014)), 1.0));
        }

        if ((s.v[908] != 0.0) && (s.v[1051] != 0.0)) {
            s.store_mul_ad_lhs(915, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(927), s.ad_value(949)), s.ad_value(950)), s.ad_value(930)), s.ad_value(967)), s.ad_value(1015)), 951);
        }

        if ((s.v[908] != 0.0) && (!(s.v[1051] != 0.0))) {
            s.store_scalar(915, 0.0);
        }

        if (s.v[908] != 0.0) {
            s.copy_ad(909, 910);
        }

        if (s.v[908] != 0.0) {
            s.copy_ad(166, 910);
        }

        if (s.v[908] != 0.0) {
            s.copy_ad(167, 911);
        }

        if (s.v[908] != 0.0) {
            s.copy_ad(168, 912);
        }

        if (s.v[908] != 0.0) {
            s.copy_ad(169, 913);
        }

        if (s.v[908] != 0.0) {
            s.copy_ad(170, 914);
        }

        if (s.v[908] != 0.0) {
            s.copy_ad(171, 915);
        }

        if (s.v[908] != 0.0) {
            s.copy_ad(166, 909);
        }

        s.v[1054] = if (p.p78 == 1.0) { 1.0 } else { 0.0 };

        s.v[172] = 0.0;

        s.v[173] = 0.0;

        s.v[174] = 0.0;

        s.v[175] = 0.0;

        s.v[176] = 0.0;

        s.v[177] = 0.0;

        s.v[1055] = if (p.p101 > p.p354) { 1.0 } else { 0.0 };

        if (s.v[1055] != 0.0) {
            s.store_scalar(1056, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1057, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1058, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1059, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1060, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1061, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1062, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1063, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1064, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.copy_ad(1065, 66);
        }

        if (s.v[1055] != 0.0) {
            s.copy_ad(1066, 67);
        }

    }

    pub(super) fn stamp_transient_block_13(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1055] != 0.0) {
            s.store_scalar(1067, p.p107);
        }

        if (s.v[1055] != 0.0) {
            s.copy_ad(1068, 68);
        }

        if (s.v[1055] != 0.0) {
            s.copy_ad(1069, 69);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1070, p.p105);
        }

        if (s.v[1055] != 0.0) {
            s.copy_ad(1071, 111);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1072, s.v[109]);
        }

        if (s.v[1055] != 0.0) {
            s.copy_ad(1073, 113);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1074, p.p0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1075, p.p101);
        }

        if (s.v[1055] != 0.0) {
            s.copy_ad(1076, 23);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1077, p.p106);
        }

        if (s.v[1055] != 0.0) {
            s.copy_ad(1078, 24);
        }

        if (s.v[1055] != 0.0) {
            s.copy_ad(1079, 25);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1080, p.p102);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1081, p.p116);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1082, p.p115);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1083, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1084, p.p117);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1085, p.p121);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1086, p.p112);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1087, p.p113);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1088, p.p114);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1089, p.p120);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1090, p.p119);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1091, p.p118);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1092, p.p39);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1093, p.p47);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1094, p.p45);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1095, p.p42);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1096, p.p2);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1097, p.p6);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1098, 1.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1099, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1100, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1101, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1102, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1103, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1104, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1105, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1106, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1107, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1108, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1109, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1110, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1111, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1112, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1113, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1114, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1115, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1116, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1117, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1118, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1119, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1120, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1121, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1122, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1123, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1124, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1125, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1126, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1127, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1128, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1129, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1130, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1131, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1132, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1133, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1134, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1135, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1136, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1137, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1138, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1139, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1140, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1141, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1142, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1143, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1144, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1145, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1146, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1147, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1148, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1149, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1150, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1151, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1152, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1153, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1154, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1155, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1156, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1157, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1158, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1159, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1160, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1161, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1162, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1163, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1164, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1165, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1166, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_scalar(1167, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_ad(1164, &{
                if (!(p.p52 == 0.0)) {
                    A::mul(s.ad_value(1066), A::tanh(A::scale(s.ad_value(1066), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(1066)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.v[1055] != 0.0) {
            s.store_sub(1165, 1065, 1066);
        }

        if (s.v[1055] != 0.0) {
            s.store_mul(1099, 1085, 1073);
        }

        if (s.v[1055] != 0.0) {
            s.store_add_ad(1101, A::div(s.ad_value(1081), A::scale(s.ad_value(1073), 2.302585092994046)), A::mul(s.ad_value(1084), s.ad_value(1164)));
        }

        if (s.v[1055] != 0.0) {
            s.store_add_ad_rhs(1102, 1080, A::mul(s.ad_value(1091), A::sub(s.ad_value(1071), s.ad_value(1072))));
        }

        if (s.v[1055] != 0.0) {
            s.store_ad(1120, &A::pow(A::div(s.ad_value(1071), s.ad_value(1072)), s.ad_value(1093)));
        }

        s.v[1168] = if (s.v[1092] != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1055] != 0.0) && (s.v[1168] != 0.0)) {
            s.store_div_ad_rhs(1103, 1164, A::pow(A::offset(A::pow(A::div(s.ad_value(1164), s.ad_value(1092)), s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088))));
        }

        if ((s.v[1055] != 0.0) && (!(s.v[1168] != 0.0))) {
            s.store_scalar(1103, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.store_mul_ad_lhs(1100, A::sub(s.ad_value(1082), A::mul(s.ad_value(1103), s.ad_value(1083))), 1164);
        }

        if (s.v[1055] != 0.0) {
            s.store_sub(1063, 1102, 1100);
        }

        if (s.v[1055] != 0.0) {
            s.store_mul_ad_lhs(1105, A::scale(s.ad_value(1101), 2.0), 1073);
        }

        if (s.v[1055] != 0.0) {
            s.store_mul(1106, 1076, 1105);
        }

        if (s.v[1055] != 0.0) {
            s.store_sub_ad_rhs(1163, 1063, A::scale(s.ad_value(1099), (p.p51 * 0.5)));
        }

        if (s.v[1055] != 0.0) {
            let assign17220_ad_e16635: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(1065), s.ad_value(1165)), A::mul(A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh(A::scale(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(1065), s.ad_value(1165)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1065), s.ad_value(1165)), A::sub(s.ad_value(1065), s.ad_value(1165))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1162, A::sub(assign17220_ad_e16635, s.ad_value(1163)), 1099);
        }

        s.v[1169] = if (s.v[1162] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1055] != 0.0) && (s.v[1169] != 0.0)) {
            s.store_scalar(1121, 0.0);
        }

        s.v[1170] = if (s.v[1162] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1055] != 0.0) && (!(s.v[1169] != 0.0))) && (s.v[1170] != 0.0)) {
            s.store_scalar(1121, 1.0);
        }

        if (((s.v[1055] != 0.0) && (!(s.v[1169] != 0.0))) && (!(s.v[1170] != 0.0))) {
            s.store_div_from_scalar_ad(1121, 1.0, A::offset(A::exp(s.ad_value(1162)), 1.0));
        }

    }

    pub(super) fn stamp_transient_block_14(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1055] != 0.0) {
            let assign17280_ad_e16723: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(1065), s.ad_value(1165)), A::mul(A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh(A::scale(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(1065), s.ad_value(1165)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1065), s.ad_value(1165)), A::sub(s.ad_value(1065), s.ad_value(1165))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1122, A::sub(assign17280_ad_e16723, A::sub(s.ad_value(1063), A::mul(A::scale(s.ad_value(1099), (p.p51 * 0.1)), s.ad_value(1121)))), 1105);
        }

        s.v[1171] = if (s.v[1122] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1055] != 0.0) && (s.v[1171] != 0.0)) {
            s.store_mul(1123, 1106, 1122);
        }

        s.v[1172] = if (s.v[1122] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1055] != 0.0) && (!(s.v[1171] != 0.0))) && (s.v[1172] != 0.0)) {
            s.store_mul_ad_rhs(1123, 1106, A::exp(s.ad_value(1122)));
        }

        if (((s.v[1055] != 0.0) && (!(s.v[1171] != 0.0))) && (!(s.v[1172] != 0.0))) {
            s.store_mul_ad_rhs(1123, 1106, A::ln(A::offset(A::exp(s.ad_value(1122)), 1.0)));
        }

        if (s.v[1055] != 0.0) {
            s.store_div_ad_rhs(1109, 1087, A::mul(s.ad_value(1120), A::offset(A::div(A::mul(s.ad_value(1089), s.ad_value(1123)), s.ad_value(1076)), 1.0)));
        }

        if (s.v[1055] != 0.0) {
            s.store_div_ad(1110, A::mul(A::mul(s.ad_value(1086), A::div(A::offset(A::mul(s.ad_value(1094), s.ad_value(1072)), 1.0), A::offset(A::mul(s.ad_value(1094), s.ad_value(1071)), 1.0))), A::offset(A::div(A::mul(s.ad_value(1095), s.ad_value(1164)), s.ad_value(1075)), 1.0)), A::offset(A::div(A::mul(s.ad_value(1090), s.ad_value(1123)), s.ad_value(1076)), 1.0));
        }

        if (s.v[1055] != 0.0) {
            s.store_add_ad(1111, A::div(A::mul(A::mul(A::scale(s.ad_value(1121), 2.0), s.ad_value(1073)), s.ad_value(1109)), s.ad_value(1075)), A::mul(A::sub_from_scalar(1.0, s.ad_value(1121)), s.ad_value(1110)));
        }

        if (s.v[1055] != 0.0) {
            s.store_div_ad_lhs(1127, A::mul(s.ad_value(1110), s.ad_value(1075)), 1109);
        }

        if (s.v[1055] != 0.0) {
            s.store_sub_ad_lhs(1128, A::mul(s.ad_value(1127), A::sqrt(A::offset(A::div(A::div(A::scale(s.ad_value(1123), 2.0), s.ad_value(1076)), s.ad_value(1127)), 1.0))), 1127);
        }

        if (s.v[1055] != 0.0) {
            s.store_add_ad(1129, A::mul(s.ad_value(1127), A::sub_from_scalar(1.0, s.ad_value(1121))), A::mul(s.ad_value(1105), s.ad_value(1121)));
        }

        if (s.v[1055] != 0.0) {
            s.store_add_ad(1064, A::mul(s.ad_value(1128), A::sub_from_scalar(1.0, s.ad_value(1121))), A::mul(s.ad_value(1105), s.ad_value(1121)));
        }

        if (s.v[1055] != 0.0) {
            let assign17410_ad_e16952: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(s.ad_value(1066), s.ad_value(1064)), A::mul(A::neg(A::div(s.ad_value(1066), s.ad_value(1064))), A::tanh(A::scale(A::neg(A::div(s.ad_value(1066), s.ad_value(1064))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(s.ad_value(1066), s.ad_value(1064)), A::sqrt(A::offset(A::mul(A::neg(A::div(s.ad_value(1066), s.ad_value(1064))), A::neg(A::div(s.ad_value(1066), s.ad_value(1064)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(1130, 1.0, A::pow(A::offset(A::pow(assign17410_ad_e16952, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088))));
        }

        if (s.v[1055] != 0.0) {
            s.store_mul(1131, 1066, 1130);
        }

        if (s.v[1055] != 0.0) {
            let assign17430_ad_e17033: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(A::neg(s.ad_value(1066)), s.ad_value(1064)), A::mul(A::neg(A::div(A::neg(s.ad_value(1066)), s.ad_value(1064))), A::tanh(A::scale(A::neg(A::div(A::neg(s.ad_value(1066)), s.ad_value(1064))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(A::neg(s.ad_value(1066)), s.ad_value(1064)), A::sqrt(A::offset(A::mul(A::neg(A::div(A::neg(s.ad_value(1066)), s.ad_value(1064))), A::neg(A::div(A::neg(s.ad_value(1066)), s.ad_value(1064)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(1132, 1.0, A::pow(A::offset(A::pow(assign17430_ad_e17033, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088))));
        }

        if (s.v[1055] != 0.0) {
            s.store_mul_ad_lhs(1133, A::neg(s.ad_value(1066)), 1132);
        }

        if (s.v[1055] != 0.0) {
            s.store_div_ad_lhs(1162, A::sub(s.ad_value(1065), s.ad_value(1163)), 1099);
        }

        s.v[1173] = if (s.v[1162] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1055] != 0.0) && (s.v[1173] != 0.0)) {
            s.store_scalar(1104, 0.0);
        }

        s.v[1174] = if (s.v[1162] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1055] != 0.0) && (!(s.v[1173] != 0.0))) && (s.v[1174] != 0.0)) {
            s.store_scalar(1104, 1.0);
        }

        if (((s.v[1055] != 0.0) && (!(s.v[1173] != 0.0))) && (!(s.v[1174] != 0.0))) {
            s.store_div_from_scalar_ad(1104, 1.0, A::offset(A::exp(s.ad_value(1162)), 1.0));
        }

        if (s.v[1055] != 0.0) {
            s.store_div_ad_lhs(1107, A::sub(A::sub(s.ad_value(1165), s.ad_value(1133)), A::sub(s.ad_value(1063), A::mul(A::scale(s.ad_value(1099), (p.p51 * 0.1)), s.ad_value(1104)))), 1105);
        }

        s.v[1175] = if (s.v[1107] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1055] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_mul(1108, 1106, 1107);
        }

        s.v[1176] = if (s.v[1107] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1055] != 0.0) && (!(s.v[1175] != 0.0))) && (s.v[1176] != 0.0)) {
            s.store_mul_ad_rhs(1108, 1106, A::exp(s.ad_value(1107)));
        }

        if (((s.v[1055] != 0.0) && (!(s.v[1175] != 0.0))) && (!(s.v[1176] != 0.0))) {
            s.store_mul_ad_rhs(1108, 1106, A::ln(A::offset(A::exp(s.ad_value(1107)), 1.0)));
        }

        if (s.v[1055] != 0.0) {
            s.store_div_ad_lhs(1162, A::sub(s.ad_value(1165), s.ad_value(1163)), 1099);
        }

        s.v[1177] = if (s.v[1162] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1055] != 0.0) && (s.v[1177] != 0.0)) {
            s.store_scalar(1134, 0.0);
        }

        s.v[1178] = if (s.v[1162] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1055] != 0.0) && (!(s.v[1177] != 0.0))) && (s.v[1178] != 0.0)) {
            s.store_scalar(1134, 1.0);
        }

        if (((s.v[1055] != 0.0) && (!(s.v[1177] != 0.0))) && (!(s.v[1178] != 0.0))) {
            s.store_div_from_scalar_ad(1134, 1.0, A::offset(A::exp(s.ad_value(1162)), 1.0));
        }

        if (s.v[1055] != 0.0) {
            s.store_div_ad_lhs(1135, A::sub(A::sub(s.ad_value(1065), s.ad_value(1131)), A::sub(s.ad_value(1063), A::mul(A::scale(s.ad_value(1099), (p.p51 * 0.1)), s.ad_value(1134)))), 1105);
        }

        s.v[1179] = if (s.v[1135] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1055] != 0.0) && (s.v[1179] != 0.0)) {
            s.store_mul(1136, 1106, 1135);
        }

        s.v[1180] = if (s.v[1135] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1055] != 0.0) && (!(s.v[1179] != 0.0))) && (s.v[1180] != 0.0)) {
            s.store_mul_ad_rhs(1136, 1106, A::exp(s.ad_value(1135)));
        }

        if (((s.v[1055] != 0.0) && (!(s.v[1179] != 0.0))) && (!(s.v[1180] != 0.0))) {
            s.store_mul_ad_rhs(1136, 1106, A::ln(A::offset(A::exp(s.ad_value(1135)), 1.0)));
        }

        if (s.v[1055] != 0.0) {
            s.store_div_ad_lhs(1137, A::sub(s.ad_value(1108), s.ad_value(1136)), 1076);
        }

        if (s.v[1055] != 0.0) {
            s.store_div(1163, 1137, 1129);
        }

        if (s.v[1055] != 0.0) {
            let assign17710_ad_e17310: A = A::pow(A::offset(A::pow({
                if (!(p.p52 == 0.0)) {
                    A::mul(s.ad_value(1163), A::tanh(A::scale(s.ad_value(1163), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(1163)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
            s.store_div_ad_rhs(1138, 1163, assign17710_ad_e17310);
        }

        if (s.v[1055] != 0.0) {
            s.store_mul(1139, 1111, 1138);
        }

        if (s.v[1055] != 0.0) {
            s.store_mul_ad_lhs(1057, A::mul(A::mul(A::scale(A::mul(A::mul(s.ad_value(1097), s.ad_value(1074)), s.ad_value(1096)), 0.5), A::add(s.ad_value(1108), s.ad_value(1136))), s.ad_value(1139)), 1098);
        }

        if (s.v[1055] != 0.0) {
            s.store_div_ad_rhs(1112, 1081, A::scale(s.ad_value(1073), 2.302585092994046));
        }

        if (s.v[1055] != 0.0) {
            s.store_mul_ad_lhs(1114, A::scale(s.ad_value(1112), 2.0), 1073);
        }

        if (s.v[1055] != 0.0) {
            s.store_mul(1115, 1076, 1114);
        }

        if (s.v[1055] != 0.0) {
            s.store_sub_ad_rhs(1167, 1102, A::scale(s.ad_value(1099), (p.p51 * 0.5)));
        }

        if (s.v[1055] != 0.0) {
            let assign17780_ad_e17414: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(1065), s.ad_value(1165)), A::mul(A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh(A::scale(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(1065), s.ad_value(1165)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1065), s.ad_value(1165)), A::sub(s.ad_value(1065), s.ad_value(1165))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1166, A::sub(assign17780_ad_e17414, s.ad_value(1167)), 1099);
        }

        s.v[1181] = if (s.v[1166] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1055] != 0.0) && (s.v[1181] != 0.0)) {
            s.store_scalar(1124, 0.0);
        }

        s.v[1182] = if (s.v[1166] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1055] != 0.0) && (!(s.v[1181] != 0.0))) && (s.v[1182] != 0.0)) {
            s.store_scalar(1124, 1.0);
        }

        if (((s.v[1055] != 0.0) && (!(s.v[1181] != 0.0))) && (!(s.v[1182] != 0.0))) {
            s.store_div_from_scalar_ad(1124, 1.0, A::offset(A::exp(s.ad_value(1166)), 1.0));
        }

        if (s.v[1055] != 0.0) {
            let assign17840_ad_e17502: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(1065), s.ad_value(1165)), A::mul(A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh(A::scale(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(1065), s.ad_value(1165)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1065), s.ad_value(1165)), A::sub(s.ad_value(1065), s.ad_value(1165))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1125, A::sub(assign17840_ad_e17502, A::sub(s.ad_value(1102), A::mul(A::scale(s.ad_value(1099), (p.p51 * 0.1)), s.ad_value(1124)))), 1114);
        }

        s.v[1183] = if (s.v[1125] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1055] != 0.0) && (s.v[1183] != 0.0)) {
            s.store_mul(1126, 1115, 1125);
        }

        s.v[1184] = if (s.v[1125] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1055] != 0.0) && (!(s.v[1183] != 0.0))) && (s.v[1184] != 0.0)) {
            s.store_mul_ad_rhs(1126, 1115, A::exp(s.ad_value(1125)));
        }

        if (((s.v[1055] != 0.0) && (!(s.v[1183] != 0.0))) && (!(s.v[1184] != 0.0))) {
            s.store_mul_ad_rhs(1126, 1115, A::ln(A::offset(A::exp(s.ad_value(1125)), 1.0)));
        }

        if (s.v[1055] != 0.0) {
            s.store_div(1118, 1087, 1120);
        }

        if (s.v[1055] != 0.0) {
            s.store_mul_ad_rhs(1119, 1086, A::div(A::offset(A::mul(s.ad_value(1094), s.ad_value(1072)), 1.0), A::offset(A::mul(s.ad_value(1094), s.ad_value(1071)), 1.0)));
        }

        if (s.v[1055] != 0.0) {
            s.store_div_ad_lhs(1140, A::mul(s.ad_value(1119), s.ad_value(1075)), 1118);
        }

        if (s.v[1055] != 0.0) {
            s.store_sub_ad_lhs(1141, A::mul(s.ad_value(1140), A::sqrt(A::offset(A::div(A::div(A::scale(s.ad_value(1126), 2.0), s.ad_value(1076)), s.ad_value(1140)), 1.0))), 1140);
        }

        if (s.v[1055] != 0.0) {
            s.store_add_ad(1142, A::mul(s.ad_value(1141), A::sub_from_scalar(1.0, s.ad_value(1124))), A::mul(s.ad_value(1114), s.ad_value(1124)));
        }

        if (s.v[1055] != 0.0) {
            let assign17950_ad_e17677: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(s.ad_value(1066), s.ad_value(1142)), A::mul(A::neg(A::div(s.ad_value(1066), s.ad_value(1142))), A::tanh(A::scale(A::neg(A::div(s.ad_value(1066), s.ad_value(1142))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(s.ad_value(1066), s.ad_value(1142)), A::sqrt(A::offset(A::mul(A::neg(A::div(s.ad_value(1066), s.ad_value(1142))), A::neg(A::div(s.ad_value(1066), s.ad_value(1142)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(1143, 1.0, A::pow(A::offset(A::pow(assign17950_ad_e17677, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088))));
        }

        if (s.v[1055] != 0.0) {
            s.store_mul(1144, 1066, 1143);
        }

        if (s.v[1055] != 0.0) {
            let assign17970_ad_e17758: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(A::neg(s.ad_value(1066)), s.ad_value(1142)), A::mul(A::neg(A::div(A::neg(s.ad_value(1066)), s.ad_value(1142))), A::tanh(A::scale(A::neg(A::div(A::neg(s.ad_value(1066)), s.ad_value(1142))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(A::neg(s.ad_value(1066)), s.ad_value(1142)), A::sqrt(A::offset(A::mul(A::neg(A::div(A::neg(s.ad_value(1066)), s.ad_value(1142))), A::neg(A::div(A::neg(s.ad_value(1066)), s.ad_value(1142)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(1145, 1.0, A::pow(A::offset(A::pow(assign17970_ad_e17758, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088))));
        }

        if (s.v[1055] != 0.0) {
            s.store_mul_ad_lhs(1146, A::neg(s.ad_value(1066)), 1145);
        }

        if (s.v[1055] != 0.0) {
            s.store_div_ad_lhs(1166, A::sub(s.ad_value(1065), s.ad_value(1167)), 1099);
        }

        s.v[1185] = if (s.v[1166] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1055] != 0.0) && (s.v[1185] != 0.0)) {
            s.store_scalar(1113, 0.0);
        }

        s.v[1186] = if (s.v[1166] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1055] != 0.0) && (!(s.v[1185] != 0.0))) && (s.v[1186] != 0.0)) {
            s.store_scalar(1113, 1.0);
        }

        if (((s.v[1055] != 0.0) && (!(s.v[1185] != 0.0))) && (!(s.v[1186] != 0.0))) {
            s.store_div_from_scalar_ad(1113, 1.0, A::offset(A::exp(s.ad_value(1166)), 1.0));
        }

        if (s.v[1055] != 0.0) {
            s.store_div_ad_lhs(1116, A::sub(A::sub(s.ad_value(1165), s.ad_value(1146)), A::sub(s.ad_value(1102), A::mul(A::scale(s.ad_value(1099), (p.p51 * 0.1)), s.ad_value(1113)))), 1114);
        }

        s.v[1187] = if (s.v[1116] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1055] != 0.0) && (s.v[1187] != 0.0)) {
            s.store_mul(1117, 1115, 1116);
        }

        s.v[1188] = if (s.v[1116] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1055] != 0.0) && (!(s.v[1187] != 0.0))) && (s.v[1188] != 0.0)) {
            s.store_mul_ad_rhs(1117, 1115, A::exp(s.ad_value(1116)));
        }

        if (((s.v[1055] != 0.0) && (!(s.v[1187] != 0.0))) && (!(s.v[1188] != 0.0))) {
            s.store_mul_ad_rhs(1117, 1115, A::ln(A::offset(A::exp(s.ad_value(1116)), 1.0)));
        }

        if (s.v[1055] != 0.0) {
            s.store_div_ad_lhs(1166, A::sub(s.ad_value(1165), s.ad_value(1167)), 1099);
        }

        s.v[1189] = if (s.v[1166] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1055] != 0.0) && (s.v[1189] != 0.0)) {
            s.store_scalar(1147, 0.0);
        }

        s.v[1190] = if (s.v[1166] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1055] != 0.0) && (!(s.v[1189] != 0.0))) && (s.v[1190] != 0.0)) {
            s.store_scalar(1147, 1.0);
        }

        if (((s.v[1055] != 0.0) && (!(s.v[1189] != 0.0))) && (!(s.v[1190] != 0.0))) {
            s.store_div_from_scalar_ad(1147, 1.0, A::offset(A::exp(s.ad_value(1166)), 1.0));
        }

        if (s.v[1055] != 0.0) {
            s.store_div_ad_lhs(1148, A::sub(A::sub(s.ad_value(1065), s.ad_value(1144)), A::sub(s.ad_value(1102), A::mul(A::scale(s.ad_value(1099), (p.p51 * 0.1)), s.ad_value(1147)))), 1114);
        }

        s.v[1191] = if (s.v[1148] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1055] != 0.0) && (s.v[1191] != 0.0)) {
            s.store_mul(1149, 1115, 1148);
        }

        s.v[1192] = if (s.v[1148] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1055] != 0.0) && (!(s.v[1191] != 0.0))) && (s.v[1192] != 0.0)) {
            s.store_mul_ad_rhs(1149, 1115, A::exp(s.ad_value(1148)));
        }

        if (((s.v[1055] != 0.0) && (!(s.v[1191] != 0.0))) && (!(s.v[1192] != 0.0))) {
            s.store_mul_ad_rhs(1149, 1115, A::ln(A::offset(A::exp(s.ad_value(1148)), 1.0)));
        }

        if (s.v[1055] != 0.0) {
            s.store_offset_ad(1150, A::square(s.ad_value(1117)), 1e-38);
        }

        if (s.v[1055] != 0.0) {
            s.store_offset_ad(1151, A::mul(s.ad_value(1150), s.ad_value(1117)), 1e-57);
        }

        if (s.v[1055] != 0.0) {
            s.store_offset_ad(1152, A::square(s.ad_value(1149)), 1e-38);
        }

        if (s.v[1055] != 0.0) {
            s.store_offset_ad(1153, A::mul(s.ad_value(1152), s.ad_value(1149)), 1e-57);
        }

        if (s.v[1055] != 0.0) {
            s.store_offset_ad(1154, A::mul(s.ad_value(1117), s.ad_value(1149)), 1e-38);
        }

        if (s.v[1055] != 0.0) {
            s.store_div_ad(1155, A::scale(A::add(A::add(s.ad_value(1150), s.ad_value(1152)), s.ad_value(1154)), (2.0 / 3.0)), A::offset(A::add(s.ad_value(1117), s.ad_value(1149)), 2e-19));
        }

        if (s.v[1055] != 0.0) {
            s.store_div_ad(1156, A::scale(A::add(A::add(A::add(A::scale(s.ad_value(1151), 2.0), A::scale(s.ad_value(1153), 3.0)), A::mul(A::scale(s.ad_value(1150), 4.0), s.ad_value(1149))), A::mul(A::scale(s.ad_value(1152), 6.0), s.ad_value(1117))), 2.0), A::scale(A::add(A::add(s.ad_value(1150), s.ad_value(1152)), A::scale(s.ad_value(1154), 2.0)), 15.0));
        }

        if (s.v[1055] != 0.0) {
            s.store_sub(1157, 1155, 1156);
        }

        if (s.v[1055] != 0.0) {
            s.copy_ad(1158, 1156);
        }

        if (s.v[1055] != 0.0) {
            s.store_mul_ad_lhs(1058, A::mul(A::mul(A::mul(A::mul(s.ad_value(1074), s.ad_value(1096)), s.ad_value(1075)), s.ad_value(1097)), s.ad_value(1157)), 1098);
        }

        if (s.v[1055] != 0.0) {
            s.store_mul_ad_lhs(1059, A::mul(A::mul(A::mul(A::mul(s.ad_value(1074), s.ad_value(1096)), s.ad_value(1075)), s.ad_value(1097)), s.ad_value(1158)), 1098);
        }

        s.v[1193] = if (s.v[1067] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1055] != 0.0) && (s.v[1193] != 0.0)) {
            s.store_div_ad_lhs(1159, A::sub(s.ad_value(1068), A::sub(s.ad_value(1102), A::scale(s.ad_value(1099), (p.p51 * 0.5)))), 1114);
        }

        s.v[1194] = if (s.v[1159] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[1055] != 0.0) && (s.v[1193] != 0.0)) && (s.v[1194] != 0.0)) {
            s.copy_ad(1162, 1159);
        }

        s.v[1195] = if (s.v[1159] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1055] != 0.0) && (s.v[1193] != 0.0)) && (!(s.v[1194] != 0.0))) && (s.v[1195] != 0.0)) {
            s.store_exp(1162, 1159);
        }

        if ((((s.v[1055] != 0.0) && (s.v[1193] != 0.0)) && (!(s.v[1194] != 0.0))) && (!(s.v[1195] != 0.0))) {
            s.store_ln_ad(1162, A::offset(A::exp(s.ad_value(1159)), 1.0));
        }

        if ((s.v[1055] != 0.0) && (s.v[1193] != 0.0)) {
            s.store_mul_ad_lhs(1060, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(1074), s.ad_value(1096)), s.ad_value(1097)), s.ad_value(1078)), s.ad_value(1114)), s.ad_value(1162)), 1098);
        }

    }

    pub(super) fn stamp_transient_block_15(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[1055] != 0.0) && (s.v[1193] != 0.0)) {
            s.store_div_ad_lhs(1160, A::sub(s.ad_value(1069), A::sub(s.ad_value(1102), A::scale(s.ad_value(1099), (p.p51 * 0.5)))), 1114);
        }

        s.v[1196] = if (s.v[1160] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[1055] != 0.0) && (s.v[1193] != 0.0)) && (s.v[1196] != 0.0)) {
            s.copy_ad(1162, 1160);
        }

        s.v[1197] = if (s.v[1160] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1055] != 0.0) && (s.v[1193] != 0.0)) && (!(s.v[1196] != 0.0))) && (s.v[1197] != 0.0)) {
            s.store_exp(1162, 1160);
        }

        if ((((s.v[1055] != 0.0) && (s.v[1193] != 0.0)) && (!(s.v[1196] != 0.0))) && (!(s.v[1197] != 0.0))) {
            s.store_ln_ad(1162, A::offset(A::exp(s.ad_value(1160)), 1.0));
        }

        if ((s.v[1055] != 0.0) && (s.v[1193] != 0.0)) {
            s.store_mul_ad_lhs(1061, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(1074), s.ad_value(1096)), s.ad_value(1097)), s.ad_value(1079)), s.ad_value(1114)), s.ad_value(1162)), 1098);
        }

        if ((s.v[1055] != 0.0) && (!(s.v[1193] != 0.0))) {
            s.store_scalar(1060, 0.0);
        }

        if ((s.v[1055] != 0.0) && (!(s.v[1193] != 0.0))) {
            s.store_scalar(1061, 0.0);
        }

        s.v[1198] = if (s.v[1070] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1055] != 0.0) && (s.v[1198] != 0.0)) {
            s.store_div_ad_lhs(1161, A::sub(s.ad_value(1065), A::sub(s.ad_value(1102), A::scale(s.ad_value(1099), (p.p51 * 0.5)))), 1114);
        }

        s.v[1199] = if (s.v[1161] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[1055] != 0.0) && (s.v[1198] != 0.0)) && (s.v[1199] != 0.0)) {
            s.copy_ad(1162, 1161);
        }

        s.v[1200] = if (s.v[1161] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1055] != 0.0) && (s.v[1198] != 0.0)) && (!(s.v[1199] != 0.0))) && (s.v[1200] != 0.0)) {
            s.store_exp(1162, 1161);
        }

        if ((((s.v[1055] != 0.0) && (s.v[1198] != 0.0)) && (!(s.v[1199] != 0.0))) && (!(s.v[1200] != 0.0))) {
            s.store_ln_ad(1162, A::offset(A::exp(s.ad_value(1161)), 1.0));
        }

        if ((s.v[1055] != 0.0) && (s.v[1198] != 0.0)) {
            s.store_mul_ad_lhs(1062, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(1074), s.ad_value(1096)), s.ad_value(1097)), s.ad_value(1077)), s.ad_value(1114)), s.ad_value(1162)), 1098);
        }

        if ((s.v[1055] != 0.0) && (!(s.v[1198] != 0.0))) {
            s.store_scalar(1062, 0.0);
        }

        if (s.v[1055] != 0.0) {
            s.copy_ad(1056, 1057);
        }

        if (s.v[1055] != 0.0) {
            s.copy_ad(172, 1057);
        }

        if (s.v[1055] != 0.0) {
            s.copy_ad(173, 1058);
        }

        if (s.v[1055] != 0.0) {
            s.copy_ad(174, 1059);
        }

        if (s.v[1055] != 0.0) {
            s.copy_ad(175, 1060);
        }

        if (s.v[1055] != 0.0) {
            s.copy_ad(176, 1061);
        }

        if (s.v[1055] != 0.0) {
            s.copy_ad(177, 1062);
        }

        if (s.v[1055] != 0.0) {
            s.copy_ad(172, 1056);
        }

        s.v[1201] = if (p.p100 == 1.0) { 1.0 } else { 0.0 };

        s.v[178] = 0.0;

        s.v[179] = 0.0;

        s.v[180] = 0.0;

        s.v[181] = 0.0;

        s.v[182] = 0.0;

        s.v[183] = 0.0;

        s.v[1202] = if (p.p123 > p.p354) { 1.0 } else { 0.0 };

        if (s.v[1202] != 0.0) {
            s.store_scalar(1203, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1204, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1205, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1206, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1207, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1208, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1209, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1210, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1211, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.copy_ad(1212, 72);
        }

        if (s.v[1202] != 0.0) {
            s.copy_ad(1213, 73);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1214, p.p129);
        }

        if (s.v[1202] != 0.0) {
            s.copy_ad(1215, 74);
        }

        if (s.v[1202] != 0.0) {
            s.copy_ad(1216, 75);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1217, p.p127);
        }

        if (s.v[1202] != 0.0) {
            s.copy_ad(1218, 111);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1219, s.v[109]);
        }

        if (s.v[1202] != 0.0) {
            s.copy_ad(1220, 113);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1221, p.p0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1222, p.p123);
        }

        if (s.v[1202] != 0.0) {
            s.copy_ad(1223, 26);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1224, p.p128);
        }

        if (s.v[1202] != 0.0) {
            s.copy_ad(1225, 27);
        }

        if (s.v[1202] != 0.0) {
            s.copy_ad(1226, 28);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1227, p.p124);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1228, p.p138);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1229, p.p137);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1230, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1231, p.p139);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1232, p.p143);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1233, p.p134);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1234, p.p135);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1235, p.p136);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1236, p.p142);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1237, p.p141);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1238, p.p140);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1239, p.p39);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1240, p.p47);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1241, p.p45);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1242, p.p42);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1243, p.p2);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1244, p.p6);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1245, 1.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1246, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1247, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1248, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1249, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1250, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1251, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1252, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1253, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1254, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1255, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1256, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1257, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1258, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1259, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1260, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1261, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1262, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1263, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1264, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1265, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1266, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1267, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1268, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1269, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1270, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1271, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1272, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1273, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1274, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1275, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1276, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1277, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1278, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1279, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1280, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1281, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1282, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1283, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1284, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1285, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1286, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1287, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1288, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1289, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1290, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1291, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1292, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1293, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1294, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1295, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1296, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1297, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1298, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1299, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1300, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1301, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1302, 0.0);
        }

    }
}
