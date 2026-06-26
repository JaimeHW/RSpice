#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv19 = ctx.node_voltage(nodes[19]);
        let nv21 = ctx.node_voltage(nodes[21]);
        s.v[0] = 0.0;

        s.v[109] = (p.p5 + 273.15);

        s.v[108] = ctx_temp;

        s.store_voltage(110, ctx, nodes, Some(4), None);

        s.store_offset(111, 110, (s.v[108] + p.p3));

        s.b[298] = (s.v[111] < ((-270.0) + 273.15));
        s.v[298] = if s.b[298] { 1.0 } else { 0.0 };

        if s.b[298] {
            s.store_scalar(111, ((-270.0) + 273.15));
        }

        s.b[299] = (s.v[111] > (1500.0 + 273.15));
        s.v[299] = if s.b[299] { 1.0 } else { 0.0 };

        if ((!s.b[298]) && s.b[299]) {
            s.store_scalar(111, (1500.0 + 273.15));
        }

        s.v[2] = 0.0;

        s.v[1] = 0.0;

        s.b[300] = (p.p50 == 0.0);
        s.v[300] = if s.b[300] { 1.0 } else { 0.0 };

        if s.b[300] {
            s.store_scalar(3, ((p.p30 / p.p0) / p.p2));
            s.store_scalar(4, ((p.p31 / p.p0) / p.p2));
        }

        if (!s.b[300]) {
            s.store_scalar(3, (((p.p30 / p.p0) + ((p.p29 * p.p54) / p.p0)) / p.p2));
            s.store_scalar(4, (((p.p31 / p.p0) + ((p.p29 * p.p66) / p.p0)) / p.p2));
        }

        s.b[301] = ((s.v[3] >= p.p353) && (s.v[3] > 0.0));
        s.v[301] = if s.b[301] { 1.0 } else { 0.0 };

        if s.b[301] {
            s.store_mul_ad_rhs(2, 3, A::add_scaled_offset_product_lhs(A::scale_offset(s.ad_value(111), p.p48, (((((-s.v[109])) * (p.p48))) + (1.0))), 1.0, s.ad_value(111), (-s.v[109]), A::offset(s.ad_value(111), (-s.v[109])), p.p49));
        }

        s.b[302] = (s.v[2] < (0.1 * s.v[3]));
        s.v[302] = if s.b[302] { 1.0 } else { 0.0 };

        if (s.b[301] && s.b[302]) {
            s.store_scale(2, 3, 0.1);
        }

        if (!s.b[301]) {
            s.store_scalar(2, 0.0);
        }

        s.b[303] = ((s.v[4] >= p.p353) && (s.v[4] > 0.0));
        s.v[303] = if s.b[303] { 1.0 } else { 0.0 };

        if s.b[303] {
            s.store_mul_ad_rhs(1, 4, A::add_scaled_offset_product_lhs(A::scale_offset(s.ad_value(111), p.p48, (((((-s.v[109])) * (p.p48))) + (1.0))), 1.0, s.ad_value(111), (-s.v[109]), A::offset(s.ad_value(111), (-s.v[109])), p.p49));
        }

        s.b[304] = (s.v[1] < (0.1 * s.v[4]));
        s.v[304] = if s.b[304] { 1.0 } else { 0.0 };

        if (s.b[303] && s.b[304]) {
            s.store_scale(1, 4, 0.1);
        }

        if (!s.b[303]) {
            s.store_scalar(1, 0.0);
        }

        s.v[5] = (((p.p324 / p.p2) / p.p325) * (p.p326 + ((p.p327 * p.p0) / p.p325)));

        s.v[6] = (((p.p324 / p.p2) / p.p325) * (((1.0 - p.p327) * p.p0) / p.p325));

        s.store_scale(113, 111, (1.38062e-23 * 6.241457005723417e18));

        s.store_offset_scaled(223, 111, p.p336, (((((-s.v[109])) * (p.p336))) + (1.0)));

        s.b[305] = (s.v[223] < 0.1);
        s.v[305] = if s.b[305] { 1.0 } else { 0.0 };

        if s.b[305] {
            s.store_scalar(223, 0.1);
        }

        s.store_powf_ad(112, A::scale(s.ad_value(111), 1.0 / (s.v[109])), 3.0);

        s.store_scale_ad(7, {
            if ((1.0 + (p.p21 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p21, (((((-s.v[109])) * (p.p21))) + (1.0)))
            }
        }, p.p9);

        s.store_scale_ad(8, {
            if ((1.0 + (p.p22 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p22, (((((-s.v[109])) * (p.p22))) + (1.0)))
            }
        }, p.p10);

        s.store_scale_ad(9, {
            if ((1.0 + (p.p23 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p23, (((((-s.v[109])) * (p.p23))) + (1.0)))
            }
        }, p.p11);

        s.store_scale_ad(10, {
            if ((1.0 + (p.p24 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p24, (((((-s.v[109])) * (p.p24))) + (1.0)))
            }
        }, p.p13);

        s.store_scale_ad(11, {
            if ((1.0 + (p.p25 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p25, (((((-s.v[109])) * (p.p25))) + (1.0)))
            }
        }, p.p12);

        s.store_scale_ad(12, {
            if ((1.0 + (p.p26 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p26, (((((-s.v[109])) * (p.p26))) + (1.0)))
            }
        }, p.p14);

        s.store_scale_ad(13, {
            if ((1.0 + (p.p21 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p21, (((((-s.v[109])) * (p.p21))) + (1.0)))
            }
        }, p.p15);

        s.store_scale_ad(14, {
            if ((1.0 + (p.p22 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p22, (((((-s.v[109])) * (p.p22))) + (1.0)))
            }
        }, p.p16);

        s.store_scale_ad(15, {
            if ((1.0 + (p.p23 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p23, (((((-s.v[109])) * (p.p23))) + (1.0)))
            }
        }, p.p17);

        s.store_scale_ad(16, {
            if ((1.0 + (p.p24 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p24, (((((-s.v[109])) * (p.p24))) + (1.0)))
            }
        }, p.p19);

        s.store_scale_ad(17, {
            if ((1.0 + (p.p25 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p25, (((((-s.v[109])) * (p.p25))) + (1.0)))
            }
        }, p.p18);

        s.store_scale_ad(18, {
            if ((1.0 + (p.p26 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p26, (((((-s.v[109])) * (p.p26))) + (1.0)))
            }
        }, p.p20);

        s.store_scale_ad(19, {
            if ((1.0 + (p.p8 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p8, (((((-s.v[109])) * (p.p8))) + (1.0)))
            }
        }, p.p7);

        s.store_scale_ad(20, {
            if ((1.0 + (p.p82 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p82, (((((-s.v[109])) * (p.p82))) + (1.0)))
            }
        }, p.p81);

        s.store_scale_ad(23, {
            if ((1.0 + (p.p104 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p104, (((((-s.v[109])) * (p.p104))) + (1.0)))
            }
        }, p.p103);

        s.store_scale_ad(26, {
            if ((1.0 + (p.p126 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p126, (((((-s.v[109])) * (p.p126))) + (1.0)))
            }
        }, p.p125);

        s.store_scale_ad(29, {
            if ((1.0 + (p.p148 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p148, (((((-s.v[109])) * (p.p148))) + (1.0)))
            }
        }, p.p147);

        s.store_scale_ad(21, {
            if ((1.0 + (p.p87 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p87, (((((-s.v[109])) * (p.p87))) + (1.0)))
            }
        }, p.p86);

        s.store_scale_ad(24, {
            if ((1.0 + (p.p109 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p109, (((((-s.v[109])) * (p.p109))) + (1.0)))
            }
        }, p.p108);

        s.store_scale_ad(27, {
            if ((1.0 + (p.p131 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p131, (((((-s.v[109])) * (p.p131))) + (1.0)))
            }
        }, p.p130);

        s.store_scale_ad(30, {
            if ((1.0 + (p.p153 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p153, (((((-s.v[109])) * (p.p153))) + (1.0)))
            }
        }, p.p152);

        s.store_scale_ad(22, {
            if ((1.0 + (p.p89 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p89, (((((-s.v[109])) * (p.p89))) + (1.0)))
            }
        }, p.p88);

        s.store_scale_ad(25, {
            if ((1.0 + (p.p111 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p111, (((((-s.v[109])) * (p.p111))) + (1.0)))
            }
        }, p.p110);

        s.store_scale_ad(28, {
            if ((1.0 + (p.p133 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p133, (((((-s.v[109])) * (p.p133))) + (1.0)))
            }
        }, p.p132);

        s.store_scale_ad(31, {
            if ((1.0 + (p.p155 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p155, (((((-s.v[109])) * (p.p155))) + (1.0)))
            }
        }, p.p154);

        s.store_scale_ad(32, {
            if ((1.0 + (p.p170 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p170, (((((-s.v[109])) * (p.p170))) + (1.0)))
            }
        }, p.p169);

        s.store_scale_ad(35, {
            if ((1.0 + (p.p192 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p192, (((((-s.v[109])) * (p.p192))) + (1.0)))
            }
        }, p.p191);

        s.store_scale_ad(38, {
            if ((1.0 + (p.p214 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p214, (((((-s.v[109])) * (p.p214))) + (1.0)))
            }
        }, p.p213);

        s.store_scale_ad(41, {
            if ((1.0 + (p.p236 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p236, (((((-s.v[109])) * (p.p236))) + (1.0)))
            }
        }, p.p235);

        s.store_scale_ad(33, {
            if ((1.0 + (p.p175 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p175, (((((-s.v[109])) * (p.p175))) + (1.0)))
            }
        }, p.p174);

        s.store_scale_ad(36, {
            if ((1.0 + (p.p197 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p197, (((((-s.v[109])) * (p.p197))) + (1.0)))
            }
        }, p.p196);

        s.store_scale_ad(39, {
            if ((1.0 + (p.p219 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p219, (((((-s.v[109])) * (p.p219))) + (1.0)))
            }
        }, p.p218);

        s.store_scale_ad(42, {
            if ((1.0 + (p.p241 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p241, (((((-s.v[109])) * (p.p241))) + (1.0)))
            }
        }, p.p240);

        s.store_scale_ad(34, {
            if ((1.0 + (p.p177 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p177, (((((-s.v[109])) * (p.p177))) + (1.0)))
            }
        }, p.p176);

        s.store_scale_ad(37, {
            if ((1.0 + (p.p199 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p199, (((((-s.v[109])) * (p.p199))) + (1.0)))
            }
        }, p.p198);

        s.store_scale_ad(40, {
            if ((1.0 + (p.p221 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p221, (((((-s.v[109])) * (p.p221))) + (1.0)))
            }
        }, p.p220);

        s.store_scale_ad(43, {
            if ((1.0 + (p.p243 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p243, (((((-s.v[109])) * (p.p243))) + (1.0)))
            }
        }, p.p242);

        s.store_scaled_voltage(44, ctx, nodes, Some(5), Some(9), p.p6);

        s.store_scaled_voltage(45, ctx, nodes, Some(8), Some(9), p.p6);

        s.b[306] = (p.p52 == 0.0);
        s.v[306] = if s.b[306] { 1.0 } else { 0.0 };

        s.b[307] = ((p.p6 * (nv19 - nv0)) <= (p.p6 * (nv19 - nv2)));
        s.v[307] = if s.b[307] { 1.0 } else { 0.0 };

        if (s.b[306] && s.b[307]) {
            s.store_scaled_voltage(48, ctx, nodes, Some(19), Some(2), p.p6);
        }

        if (s.b[306] && (!s.b[307])) {
            s.store_scaled_voltage(48, ctx, nodes, Some(19), Some(0), p.p6);
        }

        if (!s.b[306]) {
            let assign770_ad_e3265: A = {
                if (p.p52 != 0.0) {
                    let assign770_ad_e3230: A = A::add_scaled_inputs_product(A::voltage(ctx, nodes, Some(19), Some(0)), p.p6, A::voltage(ctx, nodes, Some(19), Some(2)), p.p6, A::sub_scaled_inputs(A::voltage(ctx, nodes, Some(19), Some(0)), p.p6, A::voltage(ctx, nodes, Some(19), Some(2)), p.p6), A::tanh_scaled_input(A::sub_scaled_inputs(A::voltage(ctx, nodes, Some(19), Some(0)), p.p6, A::voltage(ctx, nodes, Some(19), Some(2)), p.p6), (0.001 / p.p53)), 1.0);
                    A::scale(assign770_ad_e3230, 0.5)
                } else {
                    let assign770_ad_e3264: A = {
                        if (p.p52 == 0.0) {
                            let assign770_ad_e3261: A = A::add_scaled_inputs3(A::voltage(ctx, nodes, Some(19), Some(0)), p.p6, A::voltage(ctx, nodes, Some(19), Some(2)), p.p6, A::sqrt(A::offset(A::mul(A::sub_scaled_inputs(A::voltage(ctx, nodes, Some(19), Some(0)), p.p6, A::voltage(ctx, nodes, Some(19), Some(2)), p.p6), A::sub_scaled_inputs(A::voltage(ctx, nodes, Some(19), Some(0)), p.p6, A::voltage(ctx, nodes, Some(19), Some(2)), p.p6)), p.p53)), 1.0);
                            A::scale(assign770_ad_e3261, 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    };
                    assign770_ad_e3264
                }
            };
            s.store_ad_value(48, assign770_ad_e3265);
        }

        s.v[46] = (p.p55 + (1.0 / ((p.p29 * p.p56) * p.p33)));

        s.store_scaled_voltage(53, ctx, nodes, Some(13), Some(19), p.p6);

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

        s.b[308] = (p.p328 == 1.0);
        s.v[308] = if s.b[308] { 1.0 } else { 0.0 };

        if s.b[308] {
            let assign920_ad_e3395: A = {
                if ((!(((((nv0 - nv1) - p.p331) - (nv21 * p.p335)) / p.p334) > 50.0)) && (!(((((nv0 - nv1) - p.p331) - (nv21 * p.p335)) / p.p334) < (-50.0)))) {
                    A::exp_scaled_input(A::sub_scaled_inputs(A::offset(A::voltage(ctx, nodes, Some(0), Some(1)), (-p.p331)), 1.0, A::voltage(ctx, nodes, Some(21), None), p.p335), 1.0 / (p.p334))
                } else {
                    let assign920_ad_e3394: A = {
                        if ((!(((((nv0 - nv1) - p.p331) - (nv21 * p.p335)) / p.p334) > 50.0)) && (((((nv0 - nv1) - p.p331) - (nv21 * p.p335)) / p.p334) < (-50.0))) {
                            A::exp_scaled_input(A::constant(50.0), -1.0)
                        } else {
                            let assign920_ad_e3393: A = {
                                if (((((nv0 - nv1) - p.p331) - (nv21 * p.p335)) / p.p334) > 50.0) {
                                    A::scaled_offset(A::sub_scaled_inputs(A::offset(A::voltage(ctx, nodes, Some(0), Some(1)), (-p.p331)), 1.0 / (p.p334), A::voltage(ctx, nodes, Some(21), None), (p.p335 * 1.0 / (p.p334))), (((-50.0)) + (1.0)), ((50.0) as f64).exp())
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
            s.store_ad_value(222, A::add_scaled_inputs(A::abs(A::voltage(ctx, nodes, Some(0), Some(1))), p.p333, assign920_ad_e3395, 1.0));
        }

        if s.b[308] {
            s.store_voltage(221, ctx, nodes, Some(20), None);
            s.store_offset_mul(220, 221, 223, 1.0);
        }

        s.b[309] = (p.p328 == 2.0);
        s.v[309] = if s.b[309] { 1.0 } else { 0.0 };

        if ((!s.b[308]) && s.b[309]) {
            s.store_voltage(224, ctx, nodes, Some(22), None);
            s.store_voltage(225, ctx, nodes, Some(23), None);
            s.store_scaled_abs_ad(228, A::sub(s.ad_value(225), s.ad_value(224)), 1.0 / (p.p338));
        }

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv17 = ctx.node_voltage(nodes[17]);
        if ((!s.b[308]) && s.b[309]) {
            s.store_voltage(226, ctx, nodes, Some(25), None);
            s.store_voltage(227, ctx, nodes, Some(26), None);
            s.store_scaled_abs_ad(229, A::sub(s.ad_value(227), s.ad_value(226)), 1.0 / (p.p337));
            s.store_div_from_scalar_add_ad(230, 1.0, A::offset(s.ad_value(228), 1.0), s.ad_value(229));
        }

        s.b[310] = (p.p52 == 0.0);
        s.v[310] = if s.b[310] { 1.0 } else { 0.0 };

        s.b[311] = ((p.p6 * (nv17 - nv0)) <= (p.p6 * (nv17 - nv2)));
        s.v[311] = if s.b[311] { 1.0 } else { 0.0 };

        if (s.b[310] && s.b[311]) {
            s.store_scaled_voltage(49, ctx, nodes, Some(17), Some(2), p.p6);
        }

        if (s.b[310] && (!s.b[311])) {
            s.store_scaled_voltage(49, ctx, nodes, Some(17), Some(0), p.p6);
        }

        if (!s.b[310]) {
            let assign1070_ad_e3575: A = {
                if (p.p52 != 0.0) {
                    let assign1070_ad_e3540: A = A::add_scaled_inputs_product(A::voltage(ctx, nodes, Some(17), Some(0)), p.p6, A::voltage(ctx, nodes, Some(17), Some(2)), p.p6, A::sub_scaled_inputs(A::voltage(ctx, nodes, Some(17), Some(0)), p.p6, A::voltage(ctx, nodes, Some(17), Some(2)), p.p6), A::tanh_scaled_input(A::sub_scaled_inputs(A::voltage(ctx, nodes, Some(17), Some(0)), p.p6, A::voltage(ctx, nodes, Some(17), Some(2)), p.p6), (0.001 / p.p53)), 1.0);
                    A::scale(assign1070_ad_e3540, 0.5)
                } else {
                    let assign1070_ad_e3574: A = {
                        if (p.p52 == 0.0) {
                            let assign1070_ad_e3571: A = A::add_scaled_inputs3(A::voltage(ctx, nodes, Some(17), Some(0)), p.p6, A::voltage(ctx, nodes, Some(17), Some(2)), p.p6, A::sqrt(A::offset(A::mul(A::sub_scaled_inputs(A::voltage(ctx, nodes, Some(17), Some(0)), p.p6, A::voltage(ctx, nodes, Some(17), Some(2)), p.p6), A::sub_scaled_inputs(A::voltage(ctx, nodes, Some(17), Some(0)), p.p6, A::voltage(ctx, nodes, Some(17), Some(2)), p.p6)), p.p53)), 1.0);
                            A::scale(assign1070_ad_e3571, 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    };
                    assign1070_ad_e3574
                }
            };
            s.store_ad_value(49, assign1070_ad_e3575);
        }

        s.store_offset_div_from_scalar_ad(47, 1.0, A::scale(s.ad_value(220), (p.p29 * (p.p68 * p.p33))), p.p67);

        s.store_scaled_voltage(57, ctx, nodes, Some(18), Some(17), p.p6);

        s.store_sub(56, 47, 49);

        s.b[312] = (p.p78 == 1.0);
        s.v[312] = if s.b[312] { 1.0 } else { 0.0 };

        if s.b[312] {
            s.store_scaled_voltage(60, ctx, nodes, Some(7), Some(10), p.p6);
            s.store_scaled_voltage(62, ctx, nodes, Some(2), Some(10), p.p6);
        }

        if (!s.b[312]) {
            s.store_scaled_voltage(60, ctx, nodes, Some(2), Some(10), p.p6);
            s.store_scaled_voltage(62, ctx, nodes, Some(7), Some(10), p.p6);
        }

        s.store_scaled_voltage(61, ctx, nodes, Some(9), Some(10), p.p6);

        s.store_scaled_voltage(63, ctx, nodes, Some(3), Some(10), p.p6);

        s.b[313] = (p.p100 == 1.0);
        s.v[313] = if s.b[313] { 1.0 } else { 0.0 };

        if s.b[313] {
            s.store_scaled_voltage(66, ctx, nodes, Some(7), Some(11), p.p6);
            s.store_scaled_voltage(68, ctx, nodes, Some(2), Some(11), p.p6);
        }

        if (!s.b[313]) {
            s.store_scaled_voltage(66, ctx, nodes, Some(2), Some(11), p.p6);
            s.store_scaled_voltage(68, ctx, nodes, Some(7), Some(11), p.p6);
        }

        s.store_scaled_voltage(67, ctx, nodes, Some(10), Some(11), p.p6);

        s.store_scaled_voltage(69, ctx, nodes, Some(3), Some(11), p.p6);

        s.b[314] = (p.p122 == 1.0);
        s.v[314] = if s.b[314] { 1.0 } else { 0.0 };

        if s.b[314] {
            s.store_scaled_voltage(72, ctx, nodes, Some(7), Some(12), p.p6);
            s.store_scaled_voltage(74, ctx, nodes, Some(2), Some(12), p.p6);
        }

        if (!s.b[314]) {
            s.store_scaled_voltage(72, ctx, nodes, Some(2), Some(12), p.p6);
            s.store_scaled_voltage(74, ctx, nodes, Some(7), Some(12), p.p6);
        }

        s.store_scaled_voltage(73, ctx, nodes, Some(11), Some(12), p.p6);

        s.store_scaled_voltage(75, ctx, nodes, Some(3), Some(12), p.p6);

        s.b[315] = (p.p144 == 1.0);
        s.v[315] = if s.b[315] { 1.0 } else { 0.0 };

        if s.b[315] {
            s.store_scaled_voltage(78, ctx, nodes, Some(7), Some(13), p.p6);
            s.store_scaled_voltage(80, ctx, nodes, Some(2), Some(13), p.p6);
        }

        if (!s.b[315]) {
            s.store_scaled_voltage(78, ctx, nodes, Some(2), Some(13), p.p6);
            s.store_scaled_voltage(80, ctx, nodes, Some(7), Some(13), p.p6);
        }

        s.store_scaled_voltage(79, ctx, nodes, Some(12), Some(13), p.p6);

        s.store_scaled_voltage(81, ctx, nodes, Some(3), Some(13), p.p6);

        s.b[316] = (p.p166 == 1.0);
        s.v[316] = if s.b[316] { 1.0 } else { 0.0 };

        if s.b[316] {
            s.store_scaled_voltage(84, ctx, nodes, Some(7), Some(5), p.p6);
            s.store_scaled_voltage(86, ctx, nodes, Some(2), Some(5), p.p6);
        }

        if (!s.b[316]) {
            s.store_scaled_voltage(84, ctx, nodes, Some(2), Some(5), p.p6);
            s.store_scaled_voltage(86, ctx, nodes, Some(7), Some(5), p.p6);
        }

        s.store_scaled_voltage(85, ctx, nodes, Some(14), Some(5), p.p6);

        s.store_scaled_voltage(87, ctx, nodes, Some(3), Some(5), p.p6);

        s.b[317] = (p.p188 == 1.0);
        s.v[317] = if s.b[317] { 1.0 } else { 0.0 };

        if s.b[317] {
            s.store_scaled_voltage(90, ctx, nodes, Some(7), Some(14), p.p6);
            s.store_scaled_voltage(92, ctx, nodes, Some(2), Some(14), p.p6);
        }

        if (!s.b[317]) {
            s.store_scaled_voltage(90, ctx, nodes, Some(2), Some(14), p.p6);
            s.store_scaled_voltage(92, ctx, nodes, Some(7), Some(14), p.p6);
        }

        s.store_scaled_voltage(91, ctx, nodes, Some(15), Some(14), p.p6);

        s.store_scaled_voltage(93, ctx, nodes, Some(3), Some(14), p.p6);

        s.b[318] = (p.p210 == 1.0);
        s.v[318] = if s.b[318] { 1.0 } else { 0.0 };

        if s.b[318] {
            s.store_scaled_voltage(96, ctx, nodes, Some(7), Some(15), p.p6);
            s.store_scaled_voltage(98, ctx, nodes, Some(2), Some(15), p.p6);
        }

        if (!s.b[318]) {
            s.store_scaled_voltage(96, ctx, nodes, Some(2), Some(15), p.p6);
            s.store_scaled_voltage(98, ctx, nodes, Some(7), Some(15), p.p6);
        }

        s.store_scaled_voltage(97, ctx, nodes, Some(16), Some(15), p.p6);

        s.store_scaled_voltage(99, ctx, nodes, Some(3), Some(15), p.p6);

        s.b[319] = (p.p232 == 1.0);
        s.v[319] = if s.b[319] { 1.0 } else { 0.0 };

        if s.b[319] {
            s.store_scaled_voltage(102, ctx, nodes, Some(7), Some(16), p.p6);
            s.store_scaled_voltage(104, ctx, nodes, Some(2), Some(16), p.p6);
        }

        if (!s.b[319]) {
            s.store_scaled_voltage(102, ctx, nodes, Some(2), Some(16), p.p6);
            s.store_scaled_voltage(104, ctx, nodes, Some(7), Some(16), p.p6);
        }

        s.store_scaled_voltage(103, ctx, nodes, Some(17), Some(16), p.p6);

        s.store_scaled_voltage(105, ctx, nodes, Some(3), Some(16), p.p6);

        s.v[208] = 0.0;

        s.v[209] = 0.0;

        s.v[210] = 0.0;

        s.v[211] = 0.0;

        s.v[212] = 0.0;

        s.v[213] = 0.0;

        s.b[320] = (p.p233 > p.p354);
        s.v[320] = if s.b[320] { 1.0 } else { 0.0 };

        if s.b[320] {
            s.store_scalar(321, 0.0);
            s.store_scalar(322, 0.0);
            s.store_scalar(323, 0.0);
            s.store_scalar(324, 0.0);
            s.store_scalar(325, 0.0);
            s.store_scalar(326, 0.0);
            s.store_scalar(327, 0.0);
            s.store_scalar(328, 0.0);
            s.store_scalar(329, 0.0);
            s.copy_ad(330, 102);
            s.copy_ad(331, 103);
            s.store_scalar(332, p.p239);
            s.copy_ad(333, 104);
            s.copy_ad(334, 105);
            s.store_scalar(335, p.p237);
            s.copy_ad(336, 111);
            s.store_scalar(337, s.v[109]);
            s.copy_ad(338, 113);
            s.store_scalar(339, p.p0);
            s.store_scalar(340, p.p233);
            s.copy_ad(341, 41);
            s.store_scalar(342, p.p238);
            s.copy_ad(343, 42);
            s.copy_ad(344, 43);
            s.store_scalar(345, p.p234);
            s.store_scalar(346, p.p248);
            s.store_scalar(347, p.p247);
            s.store_scalar(348, 0.0);
            s.store_scalar(349, p.p249);
            s.store_scalar(350, p.p253);
            s.store_scalar(351, p.p244);
            s.store_scalar(352, p.p245);
            s.store_scalar(353, p.p246);
            s.store_scalar(354, p.p252);
            s.store_scalar(355, p.p251);
            s.store_scalar(356, p.p250);
            s.store_scalar(357, p.p39);
            s.store_scalar(358, p.p47);
            s.store_scalar(359, p.p45);
            s.store_scalar(360, p.p42);
            s.store_scalar(361, p.p2);
            s.store_scalar(362, p.p6);
            s.store_scalar(363, 1.0);
            s.store_scalar(364, 0.0);
            s.store_scalar(365, 0.0);
            s.store_scalar(366, 0.0);
            s.store_scalar(367, 0.0);
            s.store_scalar(368, 0.0);
            s.store_scalar(369, 0.0);
            s.store_scalar(370, 0.0);
            s.store_scalar(371, 0.0);
            s.store_scalar(372, 0.0);
            s.store_scalar(373, 0.0);
            s.store_scalar(374, 0.0);
            s.store_scalar(375, 0.0);
            s.store_scalar(376, 0.0);
            s.store_scalar(377, 0.0);
            s.store_scalar(378, 0.0);
            s.store_scalar(379, 0.0);
            s.store_scalar(380, 0.0);
            s.store_scalar(381, 0.0);
            s.store_scalar(382, 0.0);
            s.store_scalar(383, 0.0);
            s.store_scalar(384, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[320] {
            s.store_scalar(385, 0.0);
            s.store_scalar(386, 0.0);
            s.store_scalar(387, 0.0);
            s.store_scalar(388, 0.0);
            s.store_scalar(389, 0.0);
            s.store_scalar(390, 0.0);
            s.store_scalar(391, 0.0);
            s.store_scalar(392, 0.0);
            s.store_scalar(393, 0.0);
            s.store_scalar(394, 0.0);
            s.store_scalar(395, 0.0);
            s.store_scalar(396, 0.0);
            s.store_scalar(397, 0.0);
            s.store_scalar(398, 0.0);
            s.store_scalar(399, 0.0);
            s.store_scalar(400, 0.0);
            s.store_scalar(401, 0.0);
            s.store_scalar(402, 0.0);
            s.store_scalar(403, 0.0);
            s.store_scalar(404, 0.0);
            s.store_scalar(405, 0.0);
            s.store_scalar(406, 0.0);
            s.store_scalar(407, 0.0);
            s.store_scalar(408, 0.0);
            s.store_scalar(409, 0.0);
            s.store_scalar(410, 0.0);
            s.store_scalar(411, 0.0);
            s.store_scalar(412, 0.0);
            s.store_scalar(413, 0.0);
            s.store_scalar(414, 0.0);
            s.store_scalar(415, 0.0);
            s.store_scalar(416, 0.0);
            s.store_scalar(417, 0.0);
            s.store_scalar(418, 0.0);
            s.store_scalar(419, 0.0);
            s.store_scalar(420, 0.0);
            s.store_scalar(421, 0.0);
            s.store_scalar(422, 0.0);
            s.store_scalar(423, 0.0);
            s.store_scalar(424, 0.0);
            s.store_scalar(425, 0.0);
            s.store_scalar(426, 0.0);
            s.store_scalar(427, 0.0);
            s.store_scalar(428, 0.0);
            s.store_scalar(429, 0.0);
            s.store_scalar(430, 0.0);
            s.store_scalar(431, 0.0);
            s.store_scalar(432, 0.0);
        }

        if s.b[320] {
            s.store_ad_value(429, {
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(331), A::tanh_scaled_input(s.ad_value(331), (0.001 / p.p53)))
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

        if s.b[320] {
            s.store_sub(430, 330, 331);
            s.store_mul(364, 350, 338);
            s.store_ad_value(366, A::add_scaled_product(A::div_scaled_inputs(s.ad_value(346), 1.0, s.ad_value(338), 2.302585092994046), 1.0, s.ad_value(349), s.ad_value(429), 1.0));
            s.store_ad_value(367, A::add_scaled_product(s.ad_value(345), 1.0, s.ad_value(356), A::sub(s.ad_value(336), s.ad_value(337)), 1.0));
            s.store_pow_ad(385, A::div(s.ad_value(336), s.ad_value(337)), s.ad_value(358));
        }

        s.b[433] = (s.v[357] != 0.0);
        s.v[433] = if s.b[433] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[433]) {
            s.store_div_ad_rhs(368, 429, A::pow(A::offset(A::pow(A::div(s.ad_value(429), s.ad_value(357)), s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353))));
        }

        if (s.b[320] && (!s.b[433])) {
            s.store_scalar(368, 0.0);
        }

        if s.b[320] {
            s.store_mul_ad_lhs(365, A::add_scaled_product(s.ad_value(347), 1.0, s.ad_value(368), s.ad_value(348), (-1.0)), 429);
            s.store_sub(328, 367, 365);
            s.store_scaled_mul(370, 366, 338, 2.0);
            s.store_mul(371, 341, 370);
            s.store_sub_scaled_inputs(428, 328, 1.0, 364, (p.p51 * 0.5));
        }

        if s.b[320] {
            let assign3020_ad_e4515: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sub(s.ad_value(330), s.ad_value(430)), A::tanh_scaled_input(A::sub(s.ad_value(330), s.ad_value(430)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(330), s.ad_value(430)), A::sub(s.ad_value(330), s.ad_value(430))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(427, A::sub(assign3020_ad_e4515, s.ad_value(428)), 364);
        }

        s.b[434] = (s.v[427] > 50.0);
        s.v[434] = if s.b[434] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[434]) {
            s.store_scalar(386, 0.0);
        }

        s.b[435] = (s.v[427] < (-50.0));
        s.v[435] = if s.b[435] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[434])) && s.b[435]) {
            s.store_scalar(386, 1.0);
        }

        if ((s.b[320] && (!s.b[434])) && (!s.b[435])) {
            s.store_div_from_scalar_offset_ad(386, 1.0, A::exp(s.ad_value(427)), 1.0);
        }

        if s.b[320] {
            let assign3080_ad_e4603: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sub(s.ad_value(330), s.ad_value(430)), A::tanh_scaled_input(A::sub(s.ad_value(330), s.ad_value(430)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(330), s.ad_value(430)), A::sub(s.ad_value(330), s.ad_value(430))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(387, A::sub(assign3080_ad_e4603, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(364), s.ad_value(386), (-(p.p51 * 0.1)))), 370);
        }

        s.b[436] = (s.v[387] > 50.0);
        s.v[436] = if s.b[436] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[436]) {
            s.store_mul(388, 371, 387);
        }

        s.b[437] = (s.v[387] < (-50.0));
        s.v[437] = if s.b[437] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[436])) && s.b[437]) {
            s.store_mul_exp_rhs(388, 371, 387);
        }

        if ((s.b[320] && (!s.b[436])) && (!s.b[437])) {
            s.store_mul_ad_rhs(388, 371, A::ln_one_plus_exp(s.ad_value(387)));
        }

        if s.b[320] {
            s.store_div_ad_rhs(374, 352, A::mul_offset_rhs(s.ad_value(385), A::div_scaled_product(s.ad_value(354), s.ad_value(388), 1.0, s.ad_value(341), 1.0), 1.0));
            s.store_ad_value(375, A::div_scaled_product3_by_product(s.ad_value(351), A::offset(A::mul(s.ad_value(359), s.ad_value(337)), 1.0), A::offset(A::div_scaled_product(s.ad_value(360), s.ad_value(429), 1.0, s.ad_value(340), 1.0), 1.0), 1.0, A::offset(A::mul(s.ad_value(359), s.ad_value(336)), 1.0), A::offset(A::div_scaled_product(s.ad_value(355), s.ad_value(388), 1.0, s.ad_value(341), 1.0), 1.0), 1.0));
            s.store_add_ad(376, A::div_scaled_product3(s.ad_value(386), s.ad_value(338), s.ad_value(374), 2.0, s.ad_value(340), 1.0), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(386), s.ad_value(375)));
            s.store_ad_value(392, A::div_scaled_product(s.ad_value(375), s.ad_value(340), 1.0, s.ad_value(374), 1.0));
            s.store_ad_value(393, A::add_scaled_product(s.ad_value(392), (-1.0), s.ad_value(392), A::sqrt(A::offset(A::div(A::div_scaled_inputs(s.ad_value(388), 2.0, s.ad_value(341), 1.0), s.ad_value(392)), 1.0)), 1.0));
            s.store_ad_value(394, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(392), 1.0, s.ad_value(386)), 1.0, s.ad_value(370), s.ad_value(386), 1.0));
            s.store_ad_value(329, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(393), 1.0, s.ad_value(386)), 1.0, s.ad_value(370), s.ad_value(386), 1.0));
        }

        if s.b[320] {
            let assign3210_ad_e4832: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(331), s.ad_value(329)), 0.5, A::div(s.ad_value(331), s.ad_value(329)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(331), s.ad_value(329))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(331), s.ad_value(329)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(331), s.ad_value(329)), A::div(s.ad_value(331), s.ad_value(329)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(395, 1.0, A::offset(A::pow(assign3210_ad_e4832, s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353)));
        }

        if s.b[320] {
            s.store_mul(396, 331, 395);
        }

        if s.b[320] {
            let assign3230_ad_e4913: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(329), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(329), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(329), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(329), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(329), 1.0), A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(329), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(397, 1.0, A::offset(A::pow(assign3230_ad_e4913, s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353)));
        }

        if s.b[320] {
            s.store_mul_neg_lhs(398, 331, 397);
            s.store_div_ad_lhs(427, A::sub(s.ad_value(330), s.ad_value(428)), 364);
        }

        s.b[438] = (s.v[427] > 50.0);
        s.v[438] = if s.b[438] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[438]) {
            s.store_scalar(369, 0.0);
        }

        s.b[439] = (s.v[427] < (-50.0));
        s.v[439] = if s.b[439] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[438])) && s.b[439]) {
            s.store_scalar(369, 1.0);
        }

        if ((s.b[320] && (!s.b[438])) && (!s.b[439])) {
            s.store_div_from_scalar_offset_ad(369, 1.0, A::exp(s.ad_value(427)), 1.0);
        }

        if s.b[320] {
            s.store_ad_value(372, A::div_scaled_inputs3(s.ad_value(430), 1.0, s.ad_value(398), (-1.0), A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(364), s.ad_value(369), (-(p.p51 * 0.1))), -1.0, s.ad_value(370), 1.0));
        }

        s.b[440] = (s.v[372] > 50.0);
        s.v[440] = if s.b[440] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[440]) {
            s.store_mul(373, 371, 372);
        }

        s.b[441] = (s.v[372] < (-50.0));
        s.v[441] = if s.b[441] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[440])) && s.b[441]) {
            s.store_mul_exp_rhs(373, 371, 372);
        }

        if ((s.b[320] && (!s.b[440])) && (!s.b[441])) {
            s.store_mul_ad_rhs(373, 371, A::ln_one_plus_exp(s.ad_value(372)));
        }

        if s.b[320] {
            s.store_div_ad_lhs(427, A::sub(s.ad_value(430), s.ad_value(428)), 364);
        }

        s.b[442] = (s.v[427] > 50.0);
        s.v[442] = if s.b[442] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[442]) {
            s.store_scalar(399, 0.0);
        }

        s.b[443] = (s.v[427] < (-50.0));
        s.v[443] = if s.b[443] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[442])) && s.b[443]) {
            s.store_scalar(399, 1.0);
        }

        if ((s.b[320] && (!s.b[442])) && (!s.b[443])) {
            s.store_div_from_scalar_offset_ad(399, 1.0, A::exp(s.ad_value(427)), 1.0);
        }

        if s.b[320] {
            s.store_ad_value(400, A::div_scaled_inputs3(s.ad_value(330), 1.0, s.ad_value(396), (-1.0), A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(364), s.ad_value(399), (-(p.p51 * 0.1))), -1.0, s.ad_value(370), 1.0));
        }

        s.b[444] = (s.v[400] > 50.0);
        s.v[444] = if s.b[444] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[444]) {
            s.store_mul(401, 371, 400);
        }

        s.b[445] = (s.v[400] < (-50.0));
        s.v[445] = if s.b[445] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[444])) && s.b[445]) {
            s.store_mul_exp_rhs(401, 371, 400);
        }

        if ((s.b[320] && (!s.b[444])) && (!s.b[445])) {
            s.store_mul_ad_rhs(401, 371, A::ln_one_plus_exp(s.ad_value(400)));
        }

        if s.b[320] {
            s.store_div_ad_lhs(402, A::sub(s.ad_value(373), s.ad_value(401)), 341);
            s.store_div(428, 402, 394);
        }

        if s.b[320] {
            let assign3510_ad_e5190: A = A::pow(A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(428), A::tanh_scaled_input(s.ad_value(428), (0.001 / p.p53)))
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

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[320] {
            s.store_mul(404, 376, 403);
            s.store_mul_ad_lhs(322, A::mul3_scaled_output(A::mul3(s.ad_value(362), s.ad_value(339), s.ad_value(361)), A::add(s.ad_value(373), s.ad_value(401)), s.ad_value(404), 0.5), 363);
            s.store_scaled_div(377, 346, 338, (1.0 / (2.302585092994046)));
            s.store_scaled_mul(379, 377, 338, 2.0);
            s.store_mul(380, 341, 379);
            s.store_sub_scaled_inputs(432, 367, 1.0, 364, (p.p51 * 0.5));
        }

        if s.b[320] {
            let assign3580_ad_e5294: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sub(s.ad_value(330), s.ad_value(430)), A::tanh_scaled_input(A::sub(s.ad_value(330), s.ad_value(430)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(330), s.ad_value(430)), A::sub(s.ad_value(330), s.ad_value(430))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(431, A::sub(assign3580_ad_e5294, s.ad_value(432)), 364);
        }

        s.b[446] = (s.v[431] > 50.0);
        s.v[446] = if s.b[446] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[446]) {
            s.store_scalar(389, 0.0);
        }

        s.b[447] = (s.v[431] < (-50.0));
        s.v[447] = if s.b[447] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[446])) && s.b[447]) {
            s.store_scalar(389, 1.0);
        }

        if ((s.b[320] && (!s.b[446])) && (!s.b[447])) {
            s.store_div_from_scalar_offset_ad(389, 1.0, A::exp(s.ad_value(431)), 1.0);
        }

        if s.b[320] {
            let assign3640_ad_e5382: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sub(s.ad_value(330), s.ad_value(430)), A::tanh_scaled_input(A::sub(s.ad_value(330), s.ad_value(430)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(330), s.ad_value(430)), A::sub(s.ad_value(330), s.ad_value(430))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(390, A::sub(assign3640_ad_e5382, A::add_scaled_product(s.ad_value(367), 1.0, s.ad_value(364), s.ad_value(389), (-(p.p51 * 0.1)))), 379);
        }

        s.b[448] = (s.v[390] > 50.0);
        s.v[448] = if s.b[448] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[448]) {
            s.store_mul(391, 380, 390);
        }

        s.b[449] = (s.v[390] < (-50.0));
        s.v[449] = if s.b[449] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[448])) && s.b[449]) {
            s.store_mul_exp_rhs(391, 380, 390);
        }

        if ((s.b[320] && (!s.b[448])) && (!s.b[449])) {
            s.store_mul_ad_rhs(391, 380, A::ln_one_plus_exp(s.ad_value(390)));
        }

        if s.b[320] {
            s.store_div(383, 352, 385);
            s.store_mul_div_ad_rhs(384, 351, A::offset(A::mul(s.ad_value(359), s.ad_value(337)), 1.0), A::offset(A::mul(s.ad_value(359), s.ad_value(336)), 1.0));
            s.store_ad_value(405, A::div_scaled_product(s.ad_value(384), s.ad_value(340), 1.0, s.ad_value(383), 1.0));
            s.store_ad_value(406, A::add_scaled_product(s.ad_value(405), (-1.0), s.ad_value(405), A::sqrt(A::offset(A::div(A::div_scaled_inputs(s.ad_value(391), 2.0, s.ad_value(341), 1.0), s.ad_value(405)), 1.0)), 1.0));
            s.store_ad_value(407, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(406), 1.0, s.ad_value(389)), 1.0, s.ad_value(379), s.ad_value(389), 1.0));
        }

        if s.b[320] {
            let assign3750_ad_e5557: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(331), s.ad_value(407)), 0.5, A::div(s.ad_value(331), s.ad_value(407)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(331), s.ad_value(407))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(331), s.ad_value(407)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(331), s.ad_value(407)), A::div(s.ad_value(331), s.ad_value(407)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(408, 1.0, A::offset(A::pow(assign3750_ad_e5557, s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353)));
        }

        if s.b[320] {
            s.store_mul(409, 331, 408);
        }

        if s.b[320] {
            let assign3770_ad_e5638: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(407), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(407), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(407), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(407), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(407), 1.0), A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(407), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(410, 1.0, A::offset(A::pow(assign3770_ad_e5638, s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353)));
        }

        if s.b[320] {
            s.store_mul_neg_lhs(411, 331, 410);
            s.store_div_ad_lhs(431, A::sub(s.ad_value(330), s.ad_value(432)), 364);
        }

        s.b[450] = (s.v[431] > 50.0);
        s.v[450] = if s.b[450] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[450]) {
            s.store_scalar(378, 0.0);
        }

        s.b[451] = (s.v[431] < (-50.0));
        s.v[451] = if s.b[451] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[450])) && s.b[451]) {
            s.store_scalar(378, 1.0);
        }

        if ((s.b[320] && (!s.b[450])) && (!s.b[451])) {
            s.store_div_from_scalar_offset_ad(378, 1.0, A::exp(s.ad_value(431)), 1.0);
        }

        if s.b[320] {
            s.store_ad_value(381, A::div_scaled_inputs3(s.ad_value(430), 1.0, s.ad_value(411), (-1.0), A::add_scaled_product(s.ad_value(367), 1.0, s.ad_value(364), s.ad_value(378), (-(p.p51 * 0.1))), -1.0, s.ad_value(379), 1.0));
        }

        s.b[452] = (s.v[381] > 50.0);
        s.v[452] = if s.b[452] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[452]) {
            s.store_mul(382, 380, 381);
        }

        s.b[453] = (s.v[381] < (-50.0));
        s.v[453] = if s.b[453] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[452])) && s.b[453]) {
            s.store_mul_exp_rhs(382, 380, 381);
        }

        if ((s.b[320] && (!s.b[452])) && (!s.b[453])) {
            s.store_mul_ad_rhs(382, 380, A::ln_one_plus_exp(s.ad_value(381)));
        }

        if s.b[320] {
            s.store_div_ad_lhs(431, A::sub(s.ad_value(430), s.ad_value(432)), 364);
        }

        s.b[454] = (s.v[431] > 50.0);
        s.v[454] = if s.b[454] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[454]) {
            s.store_scalar(412, 0.0);
        }

        s.b[455] = (s.v[431] < (-50.0));
        s.v[455] = if s.b[455] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[454])) && s.b[455]) {
            s.store_scalar(412, 1.0);
        }

        if ((s.b[320] && (!s.b[454])) && (!s.b[455])) {
            s.store_div_from_scalar_offset_ad(412, 1.0, A::exp(s.ad_value(431)), 1.0);
        }

        if s.b[320] {
            s.store_ad_value(413, A::div_scaled_inputs3(s.ad_value(330), 1.0, s.ad_value(409), (-1.0), A::add_scaled_product(s.ad_value(367), 1.0, s.ad_value(364), s.ad_value(412), (-(p.p51 * 0.1))), -1.0, s.ad_value(379), 1.0));
        }

        s.b[456] = (s.v[413] > 50.0);
        s.v[456] = if s.b[456] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[456]) {
            s.store_mul(414, 380, 413);
        }

        s.b[457] = (s.v[413] < (-50.0));
        s.v[457] = if s.b[457] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[456])) && s.b[457]) {
            s.store_mul_exp_rhs(414, 380, 413);
        }

        if ((s.b[320] && (!s.b[456])) && (!s.b[457])) {
            s.store_mul_ad_rhs(414, 380, A::ln_one_plus_exp(s.ad_value(413)));
        }

        if s.b[320] {
            s.store_offset_square(415, 382, 1e-38);
            s.store_offset_mul(416, 415, 382, 1e-57);
            s.store_offset_square(417, 414, 1e-38);
            s.store_offset_mul(418, 417, 414, 1e-57);
            s.store_offset_mul(419, 382, 414, 1e-38);
            s.store_ad_value(420, A::div_scaled_inputs3(s.ad_value(415), (2.0 / 3.0), s.ad_value(417), (2.0 / 3.0), s.ad_value(419), (2.0 / 3.0), A::offset(A::add(s.ad_value(382), s.ad_value(414)), 2e-19), 1.0));
            s.store_div_ad(421, A::add_scaled_inputs_products(s.ad_value(416), (2.0 * 2.0), s.ad_value(418), (3.0 * 2.0), s.ad_value(415), s.ad_value(414), (4.0 * 2.0), s.ad_value(417), s.ad_value(382), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(415), 15.0, s.ad_value(417), 15.0, s.ad_value(419), (2.0 * 15.0)));
            s.store_sub(422, 420, 421);
            s.copy_ad(423, 421);
            s.store_mul_ad_lhs(323, A::mul3(A::mul3(s.ad_value(339), s.ad_value(361), s.ad_value(340)), s.ad_value(362), s.ad_value(422)), 363);
            s.store_mul_ad_lhs(324, A::mul3(A::mul3(s.ad_value(339), s.ad_value(361), s.ad_value(340)), s.ad_value(362), s.ad_value(423)), 363);
        }

        s.b[458] = (s.v[332] == 1.0);
        s.v[458] = if s.b[458] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[458]) {
            s.store_ad_value(424, A::div_scaled_inputs3(s.ad_value(333), 1.0, s.ad_value(367), -1.0, s.ad_value(364), (-(-(p.p51 * 0.5))), s.ad_value(379), 1.0));
        }

        s.b[459] = (s.v[424] > 50.0);
        s.v[459] = if s.b[459] { 1.0 } else { 0.0 };

        if ((s.b[320] && s.b[458]) && s.b[459]) {
            s.copy_ad(427, 424);
        }

        s.b[460] = (s.v[424] < (-50.0));
        s.v[460] = if s.b[460] { 1.0 } else { 0.0 };

        if (((s.b[320] && s.b[458]) && (!s.b[459])) && s.b[460]) {
            s.store_exp(427, 424);
        }

        if (((s.b[320] && s.b[458]) && (!s.b[459])) && (!s.b[460])) {
            s.store_ln_one_plus_exp(427, 424);
        }

        if (s.b[320] && s.b[458]) {
            s.store_mul_ad_product_lhs(325, A::mul3(A::mul3(s.ad_value(339), s.ad_value(361), s.ad_value(362)), s.ad_value(343), s.ad_value(379)), s.ad_value(427), 363);
            s.store_ad_value(425, A::div_scaled_inputs3(s.ad_value(334), 1.0, s.ad_value(367), -1.0, s.ad_value(364), (-(-(p.p51 * 0.5))), s.ad_value(379), 1.0));
        }

        s.b[461] = (s.v[425] > 50.0);
        s.v[461] = if s.b[461] { 1.0 } else { 0.0 };

        if ((s.b[320] && s.b[458]) && s.b[461]) {
            s.copy_ad(427, 425);
        }

        s.b[462] = (s.v[425] < (-50.0));
        s.v[462] = if s.b[462] { 1.0 } else { 0.0 };

        if (((s.b[320] && s.b[458]) && (!s.b[461])) && s.b[462]) {
            s.store_exp(427, 425);
        }

        if (((s.b[320] && s.b[458]) && (!s.b[461])) && (!s.b[462])) {
            s.store_ln_one_plus_exp(427, 425);
        }

        if (s.b[320] && s.b[458]) {
            s.store_mul_ad_product_lhs(326, A::mul3(A::mul3(s.ad_value(339), s.ad_value(361), s.ad_value(362)), s.ad_value(344), s.ad_value(379)), s.ad_value(427), 363);
        }

        if (s.b[320] && (!s.b[458])) {
            s.store_scalar(325, 0.0);
            s.store_scalar(326, 0.0);
        }

        s.b[463] = (s.v[335] == 1.0);
        s.v[463] = if s.b[463] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[463]) {
            s.store_ad_value(426, A::div_scaled_inputs3(s.ad_value(330), 1.0, s.ad_value(367), -1.0, s.ad_value(364), (-(-(p.p51 * 0.5))), s.ad_value(379), 1.0));
        }

        s.b[464] = (s.v[426] > 50.0);
        s.v[464] = if s.b[464] { 1.0 } else { 0.0 };

        if ((s.b[320] && s.b[463]) && s.b[464]) {
            s.copy_ad(427, 426);
        }

        s.b[465] = (s.v[426] < (-50.0));
        s.v[465] = if s.b[465] { 1.0 } else { 0.0 };

        if (((s.b[320] && s.b[463]) && (!s.b[464])) && s.b[465]) {
            s.store_exp(427, 426);
        }

        if (((s.b[320] && s.b[463]) && (!s.b[464])) && (!s.b[465])) {
            s.store_ln_one_plus_exp(427, 426);
        }

        if (s.b[320] && s.b[463]) {
            s.store_mul_ad_product_lhs(327, A::mul3(A::mul3(s.ad_value(339), s.ad_value(361), s.ad_value(362)), s.ad_value(342), s.ad_value(379)), s.ad_value(427), 363);
        }

        if (s.b[320] && (!s.b[463])) {
            s.store_scalar(327, 0.0);
        }

        if s.b[320] {
            s.copy_ad(321, 322);
            s.copy_ad(208, 322);
            s.copy_ad(209, 323);
            s.copy_ad(210, 324);
            s.copy_ad(211, 325);
            s.copy_ad(212, 326);
            s.copy_ad(213, 327);
            s.copy_ad(208, 321);
        }

        s.b[466] = (p.p232 == 1.0);
        s.v[466] = if s.b[466] { 1.0 } else { 0.0 };

        s.v[202] = 0.0;

        s.v[203] = 0.0;

        s.v[204] = 0.0;

        s.v[205] = 0.0;

        s.v[206] = 0.0;

        s.v[207] = 0.0;

        s.b[467] = (p.p211 > p.p354);
        s.v[467] = if s.b[467] { 1.0 } else { 0.0 };

        if s.b[467] {
            s.store_scalar(468, 0.0);
            s.store_scalar(469, 0.0);
            s.store_scalar(470, 0.0);
            s.store_scalar(471, 0.0);
            s.store_scalar(472, 0.0);
            s.store_scalar(473, 0.0);
            s.store_scalar(474, 0.0);
            s.store_scalar(475, 0.0);
            s.store_scalar(476, 0.0);
            s.copy_ad(477, 96);
            s.copy_ad(478, 97);
            s.store_scalar(479, p.p217);
            s.copy_ad(480, 98);
            s.copy_ad(481, 99);
            s.store_scalar(482, p.p215);
            s.copy_ad(483, 111);
            s.store_scalar(484, s.v[109]);
            s.copy_ad(485, 113);
            s.store_scalar(486, p.p0);
        }

    }

    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[467] {
            s.store_scalar(487, p.p211);
            s.copy_ad(488, 38);
            s.store_scalar(489, p.p216);
            s.copy_ad(490, 39);
            s.copy_ad(491, 40);
            s.store_scalar(492, p.p212);
            s.store_scalar(493, p.p226);
            s.store_scalar(494, p.p225);
            s.store_scalar(495, 0.0);
            s.store_scalar(496, p.p227);
            s.store_scalar(497, p.p231);
            s.store_scalar(498, p.p222);
            s.store_scalar(499, p.p223);
            s.store_scalar(500, p.p224);
            s.store_scalar(501, p.p230);
            s.store_scalar(502, p.p229);
            s.store_scalar(503, p.p228);
            s.store_scalar(504, p.p39);
            s.store_scalar(505, p.p47);
            s.store_scalar(506, p.p45);
            s.store_scalar(507, p.p42);
            s.store_scalar(508, p.p2);
            s.store_scalar(509, p.p6);
            s.store_scalar(510, 1.0);
            s.store_scalar(511, 0.0);
            s.store_scalar(512, 0.0);
            s.store_scalar(513, 0.0);
            s.store_scalar(514, 0.0);
            s.store_scalar(515, 0.0);
            s.store_scalar(516, 0.0);
            s.store_scalar(517, 0.0);
            s.store_scalar(518, 0.0);
            s.store_scalar(519, 0.0);
            s.store_scalar(520, 0.0);
            s.store_scalar(521, 0.0);
            s.store_scalar(522, 0.0);
            s.store_scalar(523, 0.0);
            s.store_scalar(524, 0.0);
            s.store_scalar(525, 0.0);
            s.store_scalar(526, 0.0);
            s.store_scalar(527, 0.0);
            s.store_scalar(528, 0.0);
            s.store_scalar(529, 0.0);
            s.store_scalar(530, 0.0);
            s.store_scalar(531, 0.0);
            s.store_scalar(532, 0.0);
            s.store_scalar(533, 0.0);
            s.store_scalar(534, 0.0);
            s.store_scalar(535, 0.0);
            s.store_scalar(536, 0.0);
            s.store_scalar(537, 0.0);
            s.store_scalar(538, 0.0);
            s.store_scalar(539, 0.0);
            s.store_scalar(540, 0.0);
            s.store_scalar(541, 0.0);
            s.store_scalar(542, 0.0);
            s.store_scalar(543, 0.0);
            s.store_scalar(544, 0.0);
            s.store_scalar(545, 0.0);
            s.store_scalar(546, 0.0);
            s.store_scalar(547, 0.0);
            s.store_scalar(548, 0.0);
            s.store_scalar(549, 0.0);
            s.store_scalar(550, 0.0);
            s.store_scalar(551, 0.0);
            s.store_scalar(552, 0.0);
            s.store_scalar(553, 0.0);
            s.store_scalar(554, 0.0);
            s.store_scalar(555, 0.0);
            s.store_scalar(556, 0.0);
            s.store_scalar(557, 0.0);
            s.store_scalar(558, 0.0);
            s.store_scalar(559, 0.0);
            s.store_scalar(560, 0.0);
            s.store_scalar(561, 0.0);
            s.store_scalar(562, 0.0);
            s.store_scalar(563, 0.0);
            s.store_scalar(564, 0.0);
            s.store_scalar(565, 0.0);
            s.store_scalar(566, 0.0);
            s.store_scalar(567, 0.0);
            s.store_scalar(568, 0.0);
            s.store_scalar(569, 0.0);
            s.store_scalar(570, 0.0);
            s.store_scalar(571, 0.0);
            s.store_scalar(572, 0.0);
            s.store_scalar(573, 0.0);
            s.store_scalar(574, 0.0);
            s.store_scalar(575, 0.0);
            s.store_scalar(576, 0.0);
            s.store_scalar(577, 0.0);
            s.store_scalar(578, 0.0);
            s.store_scalar(579, 0.0);
        }

        if s.b[467] {
            s.store_ad_value(576, {
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(478), A::tanh_scaled_input(s.ad_value(478), (0.001 / p.p53)))
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

        if s.b[467] {
            s.store_sub(577, 477, 478);
            s.store_mul(511, 497, 485);
            s.store_ad_value(513, A::add_scaled_product(A::div_scaled_inputs(s.ad_value(493), 1.0, s.ad_value(485), 2.302585092994046), 1.0, s.ad_value(496), s.ad_value(576), 1.0));
            s.store_ad_value(514, A::add_scaled_product(s.ad_value(492), 1.0, s.ad_value(503), A::sub(s.ad_value(483), s.ad_value(484)), 1.0));
            s.store_pow_ad(532, A::div(s.ad_value(483), s.ad_value(484)), s.ad_value(505));
        }

        s.b[580] = (s.v[504] != 0.0);
        s.v[580] = if s.b[580] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[580]) {
            s.store_div_ad_rhs(515, 576, A::pow(A::offset(A::pow(A::div(s.ad_value(576), s.ad_value(504)), s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500))));
        }

        if (s.b[467] && (!s.b[580])) {
            s.store_scalar(515, 0.0);
        }

        if s.b[467] {
            s.store_mul_ad_lhs(512, A::add_scaled_product(s.ad_value(494), 1.0, s.ad_value(515), s.ad_value(495), (-1.0)), 576);
            s.store_sub(475, 514, 512);
            s.store_scaled_mul(517, 513, 485, 2.0);
            s.store_mul(518, 488, 517);
            s.store_sub_scaled_inputs(575, 475, 1.0, 511, (p.p51 * 0.5));
        }

        if s.b[467] {
            let assign5860_ad_e6939: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sub(s.ad_value(477), s.ad_value(577)), A::tanh_scaled_input(A::sub(s.ad_value(477), s.ad_value(577)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(477), s.ad_value(577)), A::sub(s.ad_value(477), s.ad_value(577))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(574, A::sub(assign5860_ad_e6939, s.ad_value(575)), 511);
        }

        s.b[581] = (s.v[574] > 50.0);
        s.v[581] = if s.b[581] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[581]) {
            s.store_scalar(533, 0.0);
        }

        s.b[582] = (s.v[574] < (-50.0));
        s.v[582] = if s.b[582] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[581])) && s.b[582]) {
            s.store_scalar(533, 1.0);
        }

        if ((s.b[467] && (!s.b[581])) && (!s.b[582])) {
            s.store_div_from_scalar_offset_ad(533, 1.0, A::exp(s.ad_value(574)), 1.0);
        }

        if s.b[467] {
            let assign5920_ad_e7027: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sub(s.ad_value(477), s.ad_value(577)), A::tanh_scaled_input(A::sub(s.ad_value(477), s.ad_value(577)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(477), s.ad_value(577)), A::sub(s.ad_value(477), s.ad_value(577))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(534, A::sub(assign5920_ad_e7027, A::add_scaled_product(s.ad_value(475), 1.0, s.ad_value(511), s.ad_value(533), (-(p.p51 * 0.1)))), 517);
        }

        s.b[583] = (s.v[534] > 50.0);
        s.v[583] = if s.b[583] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[583]) {
            s.store_mul(535, 518, 534);
        }

        s.b[584] = (s.v[534] < (-50.0));
        s.v[584] = if s.b[584] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[583])) && s.b[584]) {
            s.store_mul_exp_rhs(535, 518, 534);
        }

        if ((s.b[467] && (!s.b[583])) && (!s.b[584])) {
            s.store_mul_ad_rhs(535, 518, A::ln_one_plus_exp(s.ad_value(534)));
        }

    }

    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[467] {
            s.store_div_ad_rhs(521, 499, A::mul_offset_rhs(s.ad_value(532), A::div_scaled_product(s.ad_value(501), s.ad_value(535), 1.0, s.ad_value(488), 1.0), 1.0));
            s.store_ad_value(522, A::div_scaled_product3_by_product(s.ad_value(498), A::offset(A::mul(s.ad_value(506), s.ad_value(484)), 1.0), A::offset(A::div_scaled_product(s.ad_value(507), s.ad_value(576), 1.0, s.ad_value(487), 1.0), 1.0), 1.0, A::offset(A::mul(s.ad_value(506), s.ad_value(483)), 1.0), A::offset(A::div_scaled_product(s.ad_value(502), s.ad_value(535), 1.0, s.ad_value(488), 1.0), 1.0), 1.0));
            s.store_add_ad(523, A::div_scaled_product3(s.ad_value(533), s.ad_value(485), s.ad_value(521), 2.0, s.ad_value(487), 1.0), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(533), s.ad_value(522)));
            s.store_ad_value(539, A::div_scaled_product(s.ad_value(522), s.ad_value(487), 1.0, s.ad_value(521), 1.0));
            s.store_ad_value(540, A::add_scaled_product(s.ad_value(539), (-1.0), s.ad_value(539), A::sqrt(A::offset(A::div(A::div_scaled_inputs(s.ad_value(535), 2.0, s.ad_value(488), 1.0), s.ad_value(539)), 1.0)), 1.0));
            s.store_ad_value(541, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(539), 1.0, s.ad_value(533)), 1.0, s.ad_value(517), s.ad_value(533), 1.0));
            s.store_ad_value(476, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(540), 1.0, s.ad_value(533)), 1.0, s.ad_value(517), s.ad_value(533), 1.0));
        }

        if s.b[467] {
            let assign6050_ad_e7256: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(478), s.ad_value(476)), 0.5, A::div(s.ad_value(478), s.ad_value(476)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(478), s.ad_value(476))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(478), s.ad_value(476)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(478), s.ad_value(476)), A::div(s.ad_value(478), s.ad_value(476)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(542, 1.0, A::offset(A::pow(assign6050_ad_e7256, s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500)));
        }

        if s.b[467] {
            s.store_mul(543, 478, 542);
        }

        if s.b[467] {
            let assign6070_ad_e7337: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(476), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(476), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(476), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(476), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(476), 1.0), A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(476), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(544, 1.0, A::offset(A::pow(assign6070_ad_e7337, s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500)));
        }

        if s.b[467] {
            s.store_mul_neg_lhs(545, 478, 544);
            s.store_div_ad_lhs(574, A::sub(s.ad_value(477), s.ad_value(575)), 511);
        }

        s.b[585] = (s.v[574] > 50.0);
        s.v[585] = if s.b[585] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[585]) {
            s.store_scalar(516, 0.0);
        }

        s.b[586] = (s.v[574] < (-50.0));
        s.v[586] = if s.b[586] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[585])) && s.b[586]) {
            s.store_scalar(516, 1.0);
        }

        if ((s.b[467] && (!s.b[585])) && (!s.b[586])) {
            s.store_div_from_scalar_offset_ad(516, 1.0, A::exp(s.ad_value(574)), 1.0);
        }

        if s.b[467] {
            s.store_ad_value(519, A::div_scaled_inputs3(s.ad_value(577), 1.0, s.ad_value(545), (-1.0), A::add_scaled_product(s.ad_value(475), 1.0, s.ad_value(511), s.ad_value(516), (-(p.p51 * 0.1))), -1.0, s.ad_value(517), 1.0));
        }

        s.b[587] = (s.v[519] > 50.0);
        s.v[587] = if s.b[587] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[587]) {
            s.store_mul(520, 518, 519);
        }

        s.b[588] = (s.v[519] < (-50.0));
        s.v[588] = if s.b[588] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[587])) && s.b[588]) {
            s.store_mul_exp_rhs(520, 518, 519);
        }

        if ((s.b[467] && (!s.b[587])) && (!s.b[588])) {
            s.store_mul_ad_rhs(520, 518, A::ln_one_plus_exp(s.ad_value(519)));
        }

        if s.b[467] {
            s.store_div_ad_lhs(574, A::sub(s.ad_value(577), s.ad_value(575)), 511);
        }

        s.b[589] = (s.v[574] > 50.0);
        s.v[589] = if s.b[589] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[589]) {
            s.store_scalar(546, 0.0);
        }

        s.b[590] = (s.v[574] < (-50.0));
        s.v[590] = if s.b[590] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[589])) && s.b[590]) {
            s.store_scalar(546, 1.0);
        }

        if ((s.b[467] && (!s.b[589])) && (!s.b[590])) {
            s.store_div_from_scalar_offset_ad(546, 1.0, A::exp(s.ad_value(574)), 1.0);
        }

        if s.b[467] {
            s.store_ad_value(547, A::div_scaled_inputs3(s.ad_value(477), 1.0, s.ad_value(543), (-1.0), A::add_scaled_product(s.ad_value(475), 1.0, s.ad_value(511), s.ad_value(546), (-(p.p51 * 0.1))), -1.0, s.ad_value(517), 1.0));
        }

        s.b[591] = (s.v[547] > 50.0);
        s.v[591] = if s.b[591] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[591]) {
            s.store_mul(548, 518, 547);
        }

        s.b[592] = (s.v[547] < (-50.0));
        s.v[592] = if s.b[592] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[591])) && s.b[592]) {
            s.store_mul_exp_rhs(548, 518, 547);
        }

        if ((s.b[467] && (!s.b[591])) && (!s.b[592])) {
            s.store_mul_ad_rhs(548, 518, A::ln_one_plus_exp(s.ad_value(547)));
        }

        if s.b[467] {
            s.store_div_ad_lhs(549, A::sub(s.ad_value(520), s.ad_value(548)), 488);
            s.store_div(575, 549, 541);
        }

        if s.b[467] {
            let assign6350_ad_e7614: A = A::pow(A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(575), A::tanh_scaled_input(s.ad_value(575), (0.001 / p.p53)))
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

        if s.b[467] {
            s.store_mul(551, 523, 550);
            s.store_mul_ad_lhs(469, A::mul3_scaled_output(A::mul3(s.ad_value(509), s.ad_value(486), s.ad_value(508)), A::add(s.ad_value(520), s.ad_value(548)), s.ad_value(551), 0.5), 510);
            s.store_scaled_div(524, 493, 485, (1.0 / (2.302585092994046)));
            s.store_scaled_mul(526, 524, 485, 2.0);
            s.store_mul(527, 488, 526);
            s.store_sub_scaled_inputs(579, 514, 1.0, 511, (p.p51 * 0.5));
        }

        if s.b[467] {
            let assign6420_ad_e7718: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sub(s.ad_value(477), s.ad_value(577)), A::tanh_scaled_input(A::sub(s.ad_value(477), s.ad_value(577)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(477), s.ad_value(577)), A::sub(s.ad_value(477), s.ad_value(577))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(578, A::sub(assign6420_ad_e7718, s.ad_value(579)), 511);
        }

        s.b[593] = (s.v[578] > 50.0);
        s.v[593] = if s.b[593] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[593]) {
            s.store_scalar(536, 0.0);
        }

        s.b[594] = (s.v[578] < (-50.0));
        s.v[594] = if s.b[594] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[593])) && s.b[594]) {
            s.store_scalar(536, 1.0);
        }

        if ((s.b[467] && (!s.b[593])) && (!s.b[594])) {
            s.store_div_from_scalar_offset_ad(536, 1.0, A::exp(s.ad_value(578)), 1.0);
        }

        if s.b[467] {
            let assign6480_ad_e7806: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sub(s.ad_value(477), s.ad_value(577)), A::tanh_scaled_input(A::sub(s.ad_value(477), s.ad_value(577)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(477), s.ad_value(577)), A::sub(s.ad_value(477), s.ad_value(577))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(537, A::sub(assign6480_ad_e7806, A::add_scaled_product(s.ad_value(514), 1.0, s.ad_value(511), s.ad_value(536), (-(p.p51 * 0.1)))), 526);
        }

        s.b[595] = (s.v[537] > 50.0);
        s.v[595] = if s.b[595] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[595]) {
            s.store_mul(538, 527, 537);
        }

        s.b[596] = (s.v[537] < (-50.0));
        s.v[596] = if s.b[596] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[595])) && s.b[596]) {
            s.store_mul_exp_rhs(538, 527, 537);
        }

        if ((s.b[467] && (!s.b[595])) && (!s.b[596])) {
            s.store_mul_ad_rhs(538, 527, A::ln_one_plus_exp(s.ad_value(537)));
        }

        if s.b[467] {
            s.store_div(530, 499, 532);
            s.store_mul_div_ad_rhs(531, 498, A::offset(A::mul(s.ad_value(506), s.ad_value(484)), 1.0), A::offset(A::mul(s.ad_value(506), s.ad_value(483)), 1.0));
            s.store_ad_value(552, A::div_scaled_product(s.ad_value(531), s.ad_value(487), 1.0, s.ad_value(530), 1.0));
            s.store_ad_value(553, A::add_scaled_product(s.ad_value(552), (-1.0), s.ad_value(552), A::sqrt(A::offset(A::div(A::div_scaled_inputs(s.ad_value(538), 2.0, s.ad_value(488), 1.0), s.ad_value(552)), 1.0)), 1.0));
            s.store_ad_value(554, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(553), 1.0, s.ad_value(536)), 1.0, s.ad_value(526), s.ad_value(536), 1.0));
        }

        if s.b[467] {
            let assign6590_ad_e7981: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(478), s.ad_value(554)), 0.5, A::div(s.ad_value(478), s.ad_value(554)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(478), s.ad_value(554))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(478), s.ad_value(554)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(478), s.ad_value(554)), A::div(s.ad_value(478), s.ad_value(554)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(555, 1.0, A::offset(A::pow(assign6590_ad_e7981, s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500)));
        }

        if s.b[467] {
            s.store_mul(556, 478, 555);
        }

        if s.b[467] {
            let assign6610_ad_e8062: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(554), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(554), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(554), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(554), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(554), 1.0), A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(554), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(557, 1.0, A::offset(A::pow(assign6610_ad_e8062, s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500)));
        }

        if s.b[467] {
            s.store_mul_neg_lhs(558, 478, 557);
            s.store_div_ad_lhs(578, A::sub(s.ad_value(477), s.ad_value(579)), 511);
        }

        s.b[597] = (s.v[578] > 50.0);
        s.v[597] = if s.b[597] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[597]) {
            s.store_scalar(525, 0.0);
        }

        s.b[598] = (s.v[578] < (-50.0));
        s.v[598] = if s.b[598] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[597])) && s.b[598]) {
            s.store_scalar(525, 1.0);
        }

        if ((s.b[467] && (!s.b[597])) && (!s.b[598])) {
            s.store_div_from_scalar_offset_ad(525, 1.0, A::exp(s.ad_value(578)), 1.0);
        }

        if s.b[467] {
            s.store_ad_value(528, A::div_scaled_inputs3(s.ad_value(577), 1.0, s.ad_value(558), (-1.0), A::add_scaled_product(s.ad_value(514), 1.0, s.ad_value(511), s.ad_value(525), (-(p.p51 * 0.1))), -1.0, s.ad_value(526), 1.0));
        }

        s.b[599] = (s.v[528] > 50.0);
        s.v[599] = if s.b[599] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[599]) {
            s.store_mul(529, 527, 528);
        }

        s.b[600] = (s.v[528] < (-50.0));
        s.v[600] = if s.b[600] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[599])) && s.b[600]) {
            s.store_mul_exp_rhs(529, 527, 528);
        }

        if ((s.b[467] && (!s.b[599])) && (!s.b[600])) {
            s.store_mul_ad_rhs(529, 527, A::ln_one_plus_exp(s.ad_value(528)));
        }

        if s.b[467] {
            s.store_div_ad_lhs(578, A::sub(s.ad_value(577), s.ad_value(579)), 511);
        }

        s.b[601] = (s.v[578] > 50.0);
        s.v[601] = if s.b[601] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[601]) {
            s.store_scalar(559, 0.0);
        }

        s.b[602] = (s.v[578] < (-50.0));
        s.v[602] = if s.b[602] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[601])) && s.b[602]) {
            s.store_scalar(559, 1.0);
        }

        if ((s.b[467] && (!s.b[601])) && (!s.b[602])) {
            s.store_div_from_scalar_offset_ad(559, 1.0, A::exp(s.ad_value(578)), 1.0);
        }

        if s.b[467] {
            s.store_ad_value(560, A::div_scaled_inputs3(s.ad_value(477), 1.0, s.ad_value(556), (-1.0), A::add_scaled_product(s.ad_value(514), 1.0, s.ad_value(511), s.ad_value(559), (-(p.p51 * 0.1))), -1.0, s.ad_value(526), 1.0));
        }

        s.b[603] = (s.v[560] > 50.0);
        s.v[603] = if s.b[603] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[603]) {
            s.store_mul(561, 527, 560);
        }

        s.b[604] = (s.v[560] < (-50.0));
        s.v[604] = if s.b[604] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[603])) && s.b[604]) {
            s.store_mul_exp_rhs(561, 527, 560);
        }

        if ((s.b[467] && (!s.b[603])) && (!s.b[604])) {
            s.store_mul_ad_rhs(561, 527, A::ln_one_plus_exp(s.ad_value(560)));
        }

        if s.b[467] {
            s.store_offset_square(562, 529, 1e-38);
            s.store_offset_mul(563, 562, 529, 1e-57);
            s.store_offset_square(564, 561, 1e-38);
            s.store_offset_mul(565, 564, 561, 1e-57);
            s.store_offset_mul(566, 529, 561, 1e-38);
            s.store_ad_value(567, A::div_scaled_inputs3(s.ad_value(562), (2.0 / 3.0), s.ad_value(564), (2.0 / 3.0), s.ad_value(566), (2.0 / 3.0), A::offset(A::add(s.ad_value(529), s.ad_value(561)), 2e-19), 1.0));
            s.store_div_ad(568, A::add_scaled_inputs_products(s.ad_value(563), (2.0 * 2.0), s.ad_value(565), (3.0 * 2.0), s.ad_value(562), s.ad_value(561), (4.0 * 2.0), s.ad_value(564), s.ad_value(529), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(562), 15.0, s.ad_value(564), 15.0, s.ad_value(566), (2.0 * 15.0)));
            s.store_sub(569, 567, 568);
            s.copy_ad(570, 568);
            s.store_mul_ad_lhs(470, A::mul3(A::mul3(s.ad_value(486), s.ad_value(508), s.ad_value(487)), s.ad_value(509), s.ad_value(569)), 510);
            s.store_mul_ad_lhs(471, A::mul3(A::mul3(s.ad_value(486), s.ad_value(508), s.ad_value(487)), s.ad_value(509), s.ad_value(570)), 510);
        }

        s.b[605] = (s.v[479] == 1.0);
        s.v[605] = if s.b[605] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[605]) {
            s.store_ad_value(571, A::div_scaled_inputs3(s.ad_value(480), 1.0, s.ad_value(514), -1.0, s.ad_value(511), (-(-(p.p51 * 0.5))), s.ad_value(526), 1.0));
        }

        s.b[606] = (s.v[571] > 50.0);
        s.v[606] = if s.b[606] { 1.0 } else { 0.0 };

        if ((s.b[467] && s.b[605]) && s.b[606]) {
            s.copy_ad(574, 571);
        }

        s.b[607] = (s.v[571] < (-50.0));
        s.v[607] = if s.b[607] { 1.0 } else { 0.0 };

        if (((s.b[467] && s.b[605]) && (!s.b[606])) && s.b[607]) {
            s.store_exp(574, 571);
        }

        if (((s.b[467] && s.b[605]) && (!s.b[606])) && (!s.b[607])) {
            s.store_ln_one_plus_exp(574, 571);
        }

        if (s.b[467] && s.b[605]) {
            s.store_mul_ad_product_lhs(472, A::mul3(A::mul3(s.ad_value(486), s.ad_value(508), s.ad_value(509)), s.ad_value(490), s.ad_value(526)), s.ad_value(574), 510);
            s.store_ad_value(572, A::div_scaled_inputs3(s.ad_value(481), 1.0, s.ad_value(514), -1.0, s.ad_value(511), (-(-(p.p51 * 0.5))), s.ad_value(526), 1.0));
        }

        s.b[608] = (s.v[572] > 50.0);
        s.v[608] = if s.b[608] { 1.0 } else { 0.0 };

        if ((s.b[467] && s.b[605]) && s.b[608]) {
            s.copy_ad(574, 572);
        }

    }

    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[609] = (s.v[572] < (-50.0));
        s.v[609] = if s.b[609] { 1.0 } else { 0.0 };

        if (((s.b[467] && s.b[605]) && (!s.b[608])) && s.b[609]) {
            s.store_exp(574, 572);
        }

        if (((s.b[467] && s.b[605]) && (!s.b[608])) && (!s.b[609])) {
            s.store_ln_one_plus_exp(574, 572);
        }

        if (s.b[467] && s.b[605]) {
            s.store_mul_ad_product_lhs(473, A::mul3(A::mul3(s.ad_value(486), s.ad_value(508), s.ad_value(509)), s.ad_value(491), s.ad_value(526)), s.ad_value(574), 510);
        }

        if (s.b[467] && (!s.b[605])) {
            s.store_scalar(472, 0.0);
            s.store_scalar(473, 0.0);
        }

        s.b[610] = (s.v[482] == 1.0);
        s.v[610] = if s.b[610] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[610]) {
            s.store_ad_value(573, A::div_scaled_inputs3(s.ad_value(477), 1.0, s.ad_value(514), -1.0, s.ad_value(511), (-(-(p.p51 * 0.5))), s.ad_value(526), 1.0));
        }

        s.b[611] = (s.v[573] > 50.0);
        s.v[611] = if s.b[611] { 1.0 } else { 0.0 };

        if ((s.b[467] && s.b[610]) && s.b[611]) {
            s.copy_ad(574, 573);
        }

        s.b[612] = (s.v[573] < (-50.0));
        s.v[612] = if s.b[612] { 1.0 } else { 0.0 };

        if (((s.b[467] && s.b[610]) && (!s.b[611])) && s.b[612]) {
            s.store_exp(574, 573);
        }

        if (((s.b[467] && s.b[610]) && (!s.b[611])) && (!s.b[612])) {
            s.store_ln_one_plus_exp(574, 573);
        }

        if (s.b[467] && s.b[610]) {
            s.store_mul_ad_product_lhs(474, A::mul3(A::mul3(s.ad_value(486), s.ad_value(508), s.ad_value(509)), s.ad_value(489), s.ad_value(526)), s.ad_value(574), 510);
        }

        if (s.b[467] && (!s.b[610])) {
            s.store_scalar(474, 0.0);
        }

        if s.b[467] {
            s.copy_ad(468, 469);
            s.copy_ad(202, 469);
            s.copy_ad(203, 470);
            s.copy_ad(204, 471);
            s.copy_ad(205, 472);
            s.copy_ad(206, 473);
            s.copy_ad(207, 474);
            s.copy_ad(202, 468);
        }

        s.b[613] = (p.p210 == 1.0);
        s.v[613] = if s.b[613] { 1.0 } else { 0.0 };

        s.v[196] = 0.0;

        s.v[197] = 0.0;

        s.v[198] = 0.0;

        s.v[199] = 0.0;

        s.v[200] = 0.0;

        s.v[201] = 0.0;

        s.b[614] = (p.p189 > p.p354);
        s.v[614] = if s.b[614] { 1.0 } else { 0.0 };

        if s.b[614] {
            s.store_scalar(615, 0.0);
            s.store_scalar(616, 0.0);
            s.store_scalar(617, 0.0);
            s.store_scalar(618, 0.0);
            s.store_scalar(619, 0.0);
            s.store_scalar(620, 0.0);
            s.store_scalar(621, 0.0);
            s.store_scalar(622, 0.0);
            s.store_scalar(623, 0.0);
            s.copy_ad(624, 90);
            s.copy_ad(625, 91);
            s.store_scalar(626, p.p195);
            s.copy_ad(627, 92);
            s.copy_ad(628, 93);
            s.store_scalar(629, p.p193);
            s.copy_ad(630, 111);
            s.store_scalar(631, s.v[109]);
            s.copy_ad(632, 113);
            s.store_scalar(633, p.p0);
            s.store_scalar(634, p.p189);
            s.copy_ad(635, 35);
            s.store_scalar(636, p.p194);
            s.copy_ad(637, 36);
            s.copy_ad(638, 37);
            s.store_scalar(639, p.p190);
            s.store_scalar(640, p.p204);
            s.store_scalar(641, p.p203);
            s.store_scalar(642, 0.0);
            s.store_scalar(643, p.p205);
            s.store_scalar(644, p.p209);
            s.store_scalar(645, p.p200);
            s.store_scalar(646, p.p201);
            s.store_scalar(647, p.p202);
            s.store_scalar(648, p.p208);
            s.store_scalar(649, p.p207);
            s.store_scalar(650, p.p206);
            s.store_scalar(651, p.p39);
            s.store_scalar(652, p.p47);
            s.store_scalar(653, p.p45);
            s.store_scalar(654, p.p42);
            s.store_scalar(655, p.p2);
            s.store_scalar(656, p.p6);
            s.store_scalar(657, 1.0);
            s.store_scalar(658, 0.0);
            s.store_scalar(659, 0.0);
            s.store_scalar(660, 0.0);
            s.store_scalar(661, 0.0);
            s.store_scalar(662, 0.0);
            s.store_scalar(663, 0.0);
            s.store_scalar(664, 0.0);
            s.store_scalar(665, 0.0);
            s.store_scalar(666, 0.0);
            s.store_scalar(667, 0.0);
            s.store_scalar(668, 0.0);
            s.store_scalar(669, 0.0);
            s.store_scalar(670, 0.0);
            s.store_scalar(671, 0.0);
            s.store_scalar(672, 0.0);
            s.store_scalar(673, 0.0);
            s.store_scalar(674, 0.0);
            s.store_scalar(675, 0.0);
            s.store_scalar(676, 0.0);
            s.store_scalar(677, 0.0);
            s.store_scalar(678, 0.0);
            s.store_scalar(679, 0.0);
            s.store_scalar(680, 0.0);
            s.store_scalar(681, 0.0);
            s.store_scalar(682, 0.0);
            s.store_scalar(683, 0.0);
            s.store_scalar(684, 0.0);
            s.store_scalar(685, 0.0);
            s.store_scalar(686, 0.0);
            s.store_scalar(687, 0.0);
            s.store_scalar(688, 0.0);
            s.store_scalar(689, 0.0);
            s.store_scalar(690, 0.0);
            s.store_scalar(691, 0.0);
            s.store_scalar(692, 0.0);
            s.store_scalar(693, 0.0);
            s.store_scalar(694, 0.0);
            s.store_scalar(695, 0.0);
            s.store_scalar(696, 0.0);
            s.store_scalar(697, 0.0);
            s.store_scalar(698, 0.0);
            s.store_scalar(699, 0.0);
            s.store_scalar(700, 0.0);
            s.store_scalar(701, 0.0);
            s.store_scalar(702, 0.0);
            s.store_scalar(703, 0.0);
            s.store_scalar(704, 0.0);
            s.store_scalar(705, 0.0);
            s.store_scalar(706, 0.0);
            s.store_scalar(707, 0.0);
            s.store_scalar(708, 0.0);
            s.store_scalar(709, 0.0);
            s.store_scalar(710, 0.0);
            s.store_scalar(711, 0.0);
            s.store_scalar(712, 0.0);
            s.store_scalar(713, 0.0);
            s.store_scalar(714, 0.0);
            s.store_scalar(715, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[614] {
            s.store_scalar(716, 0.0);
            s.store_scalar(717, 0.0);
            s.store_scalar(718, 0.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(721, 0.0);
            s.store_scalar(722, 0.0);
            s.store_scalar(723, 0.0);
            s.store_scalar(724, 0.0);
            s.store_scalar(725, 0.0);
            s.store_scalar(726, 0.0);
        }

        if s.b[614] {
            s.store_ad_value(723, {
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(625), A::tanh_scaled_input(s.ad_value(625), (0.001 / p.p53)))
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

        if s.b[614] {
            s.store_sub(724, 624, 625);
            s.store_mul(658, 644, 632);
            s.store_ad_value(660, A::add_scaled_product(A::div_scaled_inputs(s.ad_value(640), 1.0, s.ad_value(632), 2.302585092994046), 1.0, s.ad_value(643), s.ad_value(723), 1.0));
            s.store_ad_value(661, A::add_scaled_product(s.ad_value(639), 1.0, s.ad_value(650), A::sub(s.ad_value(630), s.ad_value(631)), 1.0));
            s.store_pow_ad(679, A::div(s.ad_value(630), s.ad_value(631)), s.ad_value(652));
        }

        s.b[727] = (s.v[651] != 0.0);
        s.v[727] = if s.b[727] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[727]) {
            s.store_div_ad_rhs(662, 723, A::pow(A::offset(A::pow(A::div(s.ad_value(723), s.ad_value(651)), s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647))));
        }

        if (s.b[614] && (!s.b[727])) {
            s.store_scalar(662, 0.0);
        }

        if s.b[614] {
            s.store_mul_ad_lhs(659, A::add_scaled_product(s.ad_value(641), 1.0, s.ad_value(662), s.ad_value(642), (-1.0)), 723);
            s.store_sub(622, 661, 659);
            s.store_scaled_mul(664, 660, 632, 2.0);
            s.store_mul(665, 635, 664);
            s.store_sub_scaled_inputs(722, 622, 1.0, 658, (p.p51 * 0.5));
        }

        if s.b[614] {
            let assign8700_ad_e9363: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sub(s.ad_value(624), s.ad_value(724)), A::tanh_scaled_input(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(624), s.ad_value(724)), A::sub(s.ad_value(624), s.ad_value(724))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(721, A::sub(assign8700_ad_e9363, s.ad_value(722)), 658);
        }

        s.b[728] = (s.v[721] > 50.0);
        s.v[728] = if s.b[728] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[728]) {
            s.store_scalar(680, 0.0);
        }

        s.b[729] = (s.v[721] < (-50.0));
        s.v[729] = if s.b[729] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[728])) && s.b[729]) {
            s.store_scalar(680, 1.0);
        }

        if ((s.b[614] && (!s.b[728])) && (!s.b[729])) {
            s.store_div_from_scalar_offset_ad(680, 1.0, A::exp(s.ad_value(721)), 1.0);
        }

        if s.b[614] {
            let assign8760_ad_e9451: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sub(s.ad_value(624), s.ad_value(724)), A::tanh_scaled_input(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(624), s.ad_value(724)), A::sub(s.ad_value(624), s.ad_value(724))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(681, A::sub(assign8760_ad_e9451, A::add_scaled_product(s.ad_value(622), 1.0, s.ad_value(658), s.ad_value(680), (-(p.p51 * 0.1)))), 664);
        }

        s.b[730] = (s.v[681] > 50.0);
        s.v[730] = if s.b[730] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[730]) {
            s.store_mul(682, 665, 681);
        }

        s.b[731] = (s.v[681] < (-50.0));
        s.v[731] = if s.b[731] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[730])) && s.b[731]) {
            s.store_mul_exp_rhs(682, 665, 681);
        }

        if ((s.b[614] && (!s.b[730])) && (!s.b[731])) {
            s.store_mul_ad_rhs(682, 665, A::ln_one_plus_exp(s.ad_value(681)));
        }

        if s.b[614] {
            s.store_div_ad_rhs(668, 646, A::mul_offset_rhs(s.ad_value(679), A::div_scaled_product(s.ad_value(648), s.ad_value(682), 1.0, s.ad_value(635), 1.0), 1.0));
            s.store_ad_value(669, A::div_scaled_product3_by_product(s.ad_value(645), A::offset(A::mul(s.ad_value(653), s.ad_value(631)), 1.0), A::offset(A::div_scaled_product(s.ad_value(654), s.ad_value(723), 1.0, s.ad_value(634), 1.0), 1.0), 1.0, A::offset(A::mul(s.ad_value(653), s.ad_value(630)), 1.0), A::offset(A::div_scaled_product(s.ad_value(649), s.ad_value(682), 1.0, s.ad_value(635), 1.0), 1.0), 1.0));
            s.store_add_ad(670, A::div_scaled_product3(s.ad_value(680), s.ad_value(632), s.ad_value(668), 2.0, s.ad_value(634), 1.0), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(680), s.ad_value(669)));
            s.store_ad_value(686, A::div_scaled_product(s.ad_value(669), s.ad_value(634), 1.0, s.ad_value(668), 1.0));
            s.store_ad_value(687, A::add_scaled_product(s.ad_value(686), (-1.0), s.ad_value(686), A::sqrt(A::offset(A::div(A::div_scaled_inputs(s.ad_value(682), 2.0, s.ad_value(635), 1.0), s.ad_value(686)), 1.0)), 1.0));
            s.store_ad_value(688, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(686), 1.0, s.ad_value(680)), 1.0, s.ad_value(664), s.ad_value(680), 1.0));
            s.store_ad_value(623, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(687), 1.0, s.ad_value(680)), 1.0, s.ad_value(664), s.ad_value(680), 1.0));
        }

        if s.b[614] {
            let assign8890_ad_e9680: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(625), s.ad_value(623)), 0.5, A::div(s.ad_value(625), s.ad_value(623)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(625), s.ad_value(623))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(625), s.ad_value(623)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(625), s.ad_value(623)), A::div(s.ad_value(625), s.ad_value(623)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(689, 1.0, A::offset(A::pow(assign8890_ad_e9680, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647)));
        }

        if s.b[614] {
            s.store_mul(690, 625, 689);
        }

        if s.b[614] {
            let assign8910_ad_e9761: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0), A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(691, 1.0, A::offset(A::pow(assign8910_ad_e9761, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647)));
        }

        if s.b[614] {
            s.store_mul_neg_lhs(692, 625, 691);
            s.store_div_ad_lhs(721, A::sub(s.ad_value(624), s.ad_value(722)), 658);
        }

        s.b[732] = (s.v[721] > 50.0);
        s.v[732] = if s.b[732] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[732]) {
            s.store_scalar(663, 0.0);
        }

        s.b[733] = (s.v[721] < (-50.0));
        s.v[733] = if s.b[733] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[732])) && s.b[733]) {
            s.store_scalar(663, 1.0);
        }

        if ((s.b[614] && (!s.b[732])) && (!s.b[733])) {
            s.store_div_from_scalar_offset_ad(663, 1.0, A::exp(s.ad_value(721)), 1.0);
        }

        if s.b[614] {
            s.store_ad_value(666, A::div_scaled_inputs3(s.ad_value(724), 1.0, s.ad_value(692), (-1.0), A::add_scaled_product(s.ad_value(622), 1.0, s.ad_value(658), s.ad_value(663), (-(p.p51 * 0.1))), -1.0, s.ad_value(664), 1.0));
        }

        s.b[734] = (s.v[666] > 50.0);
        s.v[734] = if s.b[734] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[734]) {
            s.store_mul(667, 665, 666);
        }

        s.b[735] = (s.v[666] < (-50.0));
        s.v[735] = if s.b[735] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[734])) && s.b[735]) {
            s.store_mul_exp_rhs(667, 665, 666);
        }

        if ((s.b[614] && (!s.b[734])) && (!s.b[735])) {
            s.store_mul_ad_rhs(667, 665, A::ln_one_plus_exp(s.ad_value(666)));
        }

        if s.b[614] {
            s.store_div_ad_lhs(721, A::sub(s.ad_value(724), s.ad_value(722)), 658);
        }

        s.b[736] = (s.v[721] > 50.0);
        s.v[736] = if s.b[736] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[736]) {
            s.store_scalar(693, 0.0);
        }

        s.b[737] = (s.v[721] < (-50.0));
        s.v[737] = if s.b[737] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[736])) && s.b[737]) {
            s.store_scalar(693, 1.0);
        }

        if ((s.b[614] && (!s.b[736])) && (!s.b[737])) {
            s.store_div_from_scalar_offset_ad(693, 1.0, A::exp(s.ad_value(721)), 1.0);
        }

        if s.b[614] {
            s.store_ad_value(694, A::div_scaled_inputs3(s.ad_value(624), 1.0, s.ad_value(690), (-1.0), A::add_scaled_product(s.ad_value(622), 1.0, s.ad_value(658), s.ad_value(693), (-(p.p51 * 0.1))), -1.0, s.ad_value(664), 1.0));
        }

        s.b[738] = (s.v[694] > 50.0);
        s.v[738] = if s.b[738] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[738]) {
            s.store_mul(695, 665, 694);
        }

        s.b[739] = (s.v[694] < (-50.0));
        s.v[739] = if s.b[739] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[738])) && s.b[739]) {
            s.store_mul_exp_rhs(695, 665, 694);
        }

        if ((s.b[614] && (!s.b[738])) && (!s.b[739])) {
            s.store_mul_ad_rhs(695, 665, A::ln_one_plus_exp(s.ad_value(694)));
        }

        if s.b[614] {
            s.store_div_ad_lhs(696, A::sub(s.ad_value(667), s.ad_value(695)), 635);
            s.store_div(722, 696, 688);
        }

        if s.b[614] {
            let assign9190_ad_e10038: A = A::pow(A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(722), A::tanh_scaled_input(s.ad_value(722), (0.001 / p.p53)))
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

        if s.b[614] {
            s.store_mul(698, 670, 697);
            s.store_mul_ad_lhs(616, A::mul3_scaled_output(A::mul3(s.ad_value(656), s.ad_value(633), s.ad_value(655)), A::add(s.ad_value(667), s.ad_value(695)), s.ad_value(698), 0.5), 657);
            s.store_scaled_div(671, 640, 632, (1.0 / (2.302585092994046)));
            s.store_scaled_mul(673, 671, 632, 2.0);
            s.store_mul(674, 635, 673);
            s.store_sub_scaled_inputs(726, 661, 1.0, 658, (p.p51 * 0.5));
        }

        if s.b[614] {
            let assign9260_ad_e10142: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sub(s.ad_value(624), s.ad_value(724)), A::tanh_scaled_input(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(624), s.ad_value(724)), A::sub(s.ad_value(624), s.ad_value(724))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(725, A::sub(assign9260_ad_e10142, s.ad_value(726)), 658);
        }

        s.b[740] = (s.v[725] > 50.0);
        s.v[740] = if s.b[740] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[740]) {
            s.store_scalar(683, 0.0);
        }

        s.b[741] = (s.v[725] < (-50.0));
        s.v[741] = if s.b[741] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[740])) && s.b[741]) {
            s.store_scalar(683, 1.0);
        }

        if ((s.b[614] && (!s.b[740])) && (!s.b[741])) {
            s.store_div_from_scalar_offset_ad(683, 1.0, A::exp(s.ad_value(725)), 1.0);
        }

        if s.b[614] {
            let assign9320_ad_e10230: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sub(s.ad_value(624), s.ad_value(724)), A::tanh_scaled_input(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(624), s.ad_value(724)), A::sub(s.ad_value(624), s.ad_value(724))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(684, A::sub(assign9320_ad_e10230, A::add_scaled_product(s.ad_value(661), 1.0, s.ad_value(658), s.ad_value(683), (-(p.p51 * 0.1)))), 673);
        }

        s.b[742] = (s.v[684] > 50.0);
        s.v[742] = if s.b[742] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[742]) {
            s.store_mul(685, 674, 684);
        }

        s.b[743] = (s.v[684] < (-50.0));
        s.v[743] = if s.b[743] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[742])) && s.b[743]) {
            s.store_mul_exp_rhs(685, 674, 684);
        }

        if ((s.b[614] && (!s.b[742])) && (!s.b[743])) {
            s.store_mul_ad_rhs(685, 674, A::ln_one_plus_exp(s.ad_value(684)));
        }

        if s.b[614] {
            s.store_div(677, 646, 679);
            s.store_mul_div_ad_rhs(678, 645, A::offset(A::mul(s.ad_value(653), s.ad_value(631)), 1.0), A::offset(A::mul(s.ad_value(653), s.ad_value(630)), 1.0));
            s.store_ad_value(699, A::div_scaled_product(s.ad_value(678), s.ad_value(634), 1.0, s.ad_value(677), 1.0));
            s.store_ad_value(700, A::add_scaled_product(s.ad_value(699), (-1.0), s.ad_value(699), A::sqrt(A::offset(A::div(A::div_scaled_inputs(s.ad_value(685), 2.0, s.ad_value(635), 1.0), s.ad_value(699)), 1.0)), 1.0));
            s.store_ad_value(701, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(700), 1.0, s.ad_value(683)), 1.0, s.ad_value(673), s.ad_value(683), 1.0));
        }

        if s.b[614] {
            let assign9430_ad_e10405: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(625), s.ad_value(701)), 0.5, A::div(s.ad_value(625), s.ad_value(701)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(625), s.ad_value(701))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(625), s.ad_value(701)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(625), s.ad_value(701)), A::div(s.ad_value(625), s.ad_value(701)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(702, 1.0, A::offset(A::pow(assign9430_ad_e10405, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647)));
        }

        if s.b[614] {
            s.store_mul(703, 625, 702);
        }

    }

    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[614] {
            let assign9450_ad_e10486: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0), A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(704, 1.0, A::offset(A::pow(assign9450_ad_e10486, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647)));
        }

        if s.b[614] {
            s.store_mul_neg_lhs(705, 625, 704);
            s.store_div_ad_lhs(725, A::sub(s.ad_value(624), s.ad_value(726)), 658);
        }

        s.b[744] = (s.v[725] > 50.0);
        s.v[744] = if s.b[744] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[744]) {
            s.store_scalar(672, 0.0);
        }

        s.b[745] = (s.v[725] < (-50.0));
        s.v[745] = if s.b[745] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[744])) && s.b[745]) {
            s.store_scalar(672, 1.0);
        }

        if ((s.b[614] && (!s.b[744])) && (!s.b[745])) {
            s.store_div_from_scalar_offset_ad(672, 1.0, A::exp(s.ad_value(725)), 1.0);
        }

        if s.b[614] {
            s.store_ad_value(675, A::div_scaled_inputs3(s.ad_value(724), 1.0, s.ad_value(705), (-1.0), A::add_scaled_product(s.ad_value(661), 1.0, s.ad_value(658), s.ad_value(672), (-(p.p51 * 0.1))), -1.0, s.ad_value(673), 1.0));
        }

        s.b[746] = (s.v[675] > 50.0);
        s.v[746] = if s.b[746] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[746]) {
            s.store_mul(676, 674, 675);
        }

        s.b[747] = (s.v[675] < (-50.0));
        s.v[747] = if s.b[747] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[746])) && s.b[747]) {
            s.store_mul_exp_rhs(676, 674, 675);
        }

        if ((s.b[614] && (!s.b[746])) && (!s.b[747])) {
            s.store_mul_ad_rhs(676, 674, A::ln_one_plus_exp(s.ad_value(675)));
        }

        if s.b[614] {
            s.store_div_ad_lhs(725, A::sub(s.ad_value(724), s.ad_value(726)), 658);
        }

        s.b[748] = (s.v[725] > 50.0);
        s.v[748] = if s.b[748] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[748]) {
            s.store_scalar(706, 0.0);
        }

        s.b[749] = (s.v[725] < (-50.0));
        s.v[749] = if s.b[749] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[748])) && s.b[749]) {
            s.store_scalar(706, 1.0);
        }

        if ((s.b[614] && (!s.b[748])) && (!s.b[749])) {
            s.store_div_from_scalar_offset_ad(706, 1.0, A::exp(s.ad_value(725)), 1.0);
        }

        if s.b[614] {
            s.store_ad_value(707, A::div_scaled_inputs3(s.ad_value(624), 1.0, s.ad_value(703), (-1.0), A::add_scaled_product(s.ad_value(661), 1.0, s.ad_value(658), s.ad_value(706), (-(p.p51 * 0.1))), -1.0, s.ad_value(673), 1.0));
        }

        s.b[750] = (s.v[707] > 50.0);
        s.v[750] = if s.b[750] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[750]) {
            s.store_mul(708, 674, 707);
        }

        s.b[751] = (s.v[707] < (-50.0));
        s.v[751] = if s.b[751] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[750])) && s.b[751]) {
            s.store_mul_exp_rhs(708, 674, 707);
        }

        if ((s.b[614] && (!s.b[750])) && (!s.b[751])) {
            s.store_mul_ad_rhs(708, 674, A::ln_one_plus_exp(s.ad_value(707)));
        }

        if s.b[614] {
            s.store_offset_square(709, 676, 1e-38);
            s.store_offset_mul(710, 709, 676, 1e-57);
            s.store_offset_square(711, 708, 1e-38);
            s.store_offset_mul(712, 711, 708, 1e-57);
            s.store_offset_mul(713, 676, 708, 1e-38);
            s.store_ad_value(714, A::div_scaled_inputs3(s.ad_value(709), (2.0 / 3.0), s.ad_value(711), (2.0 / 3.0), s.ad_value(713), (2.0 / 3.0), A::offset(A::add(s.ad_value(676), s.ad_value(708)), 2e-19), 1.0));
            s.store_div_ad(715, A::add_scaled_inputs_products(s.ad_value(710), (2.0 * 2.0), s.ad_value(712), (3.0 * 2.0), s.ad_value(709), s.ad_value(708), (4.0 * 2.0), s.ad_value(711), s.ad_value(676), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(709), 15.0, s.ad_value(711), 15.0, s.ad_value(713), (2.0 * 15.0)));
            s.store_sub(716, 714, 715);
            s.copy_ad(717, 715);
            s.store_mul_ad_lhs(617, A::mul3(A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(634)), s.ad_value(656), s.ad_value(716)), 657);
            s.store_mul_ad_lhs(618, A::mul3(A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(634)), s.ad_value(656), s.ad_value(717)), 657);
        }

        s.b[752] = (s.v[626] == 1.0);
        s.v[752] = if s.b[752] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[752]) {
            s.store_ad_value(718, A::div_scaled_inputs3(s.ad_value(627), 1.0, s.ad_value(661), -1.0, s.ad_value(658), (-(-(p.p51 * 0.5))), s.ad_value(673), 1.0));
        }

        s.b[753] = (s.v[718] > 50.0);
        s.v[753] = if s.b[753] { 1.0 } else { 0.0 };

        if ((s.b[614] && s.b[752]) && s.b[753]) {
            s.copy_ad(721, 718);
        }

        s.b[754] = (s.v[718] < (-50.0));
        s.v[754] = if s.b[754] { 1.0 } else { 0.0 };

        if (((s.b[614] && s.b[752]) && (!s.b[753])) && s.b[754]) {
            s.store_exp(721, 718);
        }

        if (((s.b[614] && s.b[752]) && (!s.b[753])) && (!s.b[754])) {
            s.store_ln_one_plus_exp(721, 718);
        }

        if (s.b[614] && s.b[752]) {
            s.store_mul_ad_product_lhs(619, A::mul3(A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(656)), s.ad_value(637), s.ad_value(673)), s.ad_value(721), 657);
            s.store_ad_value(719, A::div_scaled_inputs3(s.ad_value(628), 1.0, s.ad_value(661), -1.0, s.ad_value(658), (-(-(p.p51 * 0.5))), s.ad_value(673), 1.0));
        }

        s.b[755] = (s.v[719] > 50.0);
        s.v[755] = if s.b[755] { 1.0 } else { 0.0 };

        if ((s.b[614] && s.b[752]) && s.b[755]) {
            s.copy_ad(721, 719);
        }

        s.b[756] = (s.v[719] < (-50.0));
        s.v[756] = if s.b[756] { 1.0 } else { 0.0 };

        if (((s.b[614] && s.b[752]) && (!s.b[755])) && s.b[756]) {
            s.store_exp(721, 719);
        }

        if (((s.b[614] && s.b[752]) && (!s.b[755])) && (!s.b[756])) {
            s.store_ln_one_plus_exp(721, 719);
        }

        if (s.b[614] && s.b[752]) {
            s.store_mul_ad_product_lhs(620, A::mul3(A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(656)), s.ad_value(638), s.ad_value(673)), s.ad_value(721), 657);
        }

        if (s.b[614] && (!s.b[752])) {
            s.store_scalar(619, 0.0);
            s.store_scalar(620, 0.0);
        }

        s.b[757] = (s.v[629] == 1.0);
        s.v[757] = if s.b[757] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[757]) {
            s.store_ad_value(720, A::div_scaled_inputs3(s.ad_value(624), 1.0, s.ad_value(661), -1.0, s.ad_value(658), (-(-(p.p51 * 0.5))), s.ad_value(673), 1.0));
        }

        s.b[758] = (s.v[720] > 50.0);
        s.v[758] = if s.b[758] { 1.0 } else { 0.0 };

        if ((s.b[614] && s.b[757]) && s.b[758]) {
            s.copy_ad(721, 720);
        }

        s.b[759] = (s.v[720] < (-50.0));
        s.v[759] = if s.b[759] { 1.0 } else { 0.0 };

        if (((s.b[614] && s.b[757]) && (!s.b[758])) && s.b[759]) {
            s.store_exp(721, 720);
        }

        if (((s.b[614] && s.b[757]) && (!s.b[758])) && (!s.b[759])) {
            s.store_ln_one_plus_exp(721, 720);
        }

        if (s.b[614] && s.b[757]) {
            s.store_mul_ad_product_lhs(621, A::mul3(A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(656)), s.ad_value(636), s.ad_value(673)), s.ad_value(721), 657);
        }

        if (s.b[614] && (!s.b[757])) {
            s.store_scalar(621, 0.0);
        }

        if s.b[614] {
            s.copy_ad(615, 616);
            s.copy_ad(196, 616);
            s.copy_ad(197, 617);
            s.copy_ad(198, 618);
            s.copy_ad(199, 619);
            s.copy_ad(200, 620);
            s.copy_ad(201, 621);
            s.copy_ad(196, 615);
        }

        s.b[760] = (p.p188 == 1.0);
        s.v[760] = if s.b[760] { 1.0 } else { 0.0 };

        s.v[190] = 0.0;

        s.v[191] = 0.0;

        s.v[192] = 0.0;

        s.v[193] = 0.0;

        s.v[194] = 0.0;

        s.v[195] = 0.0;

        s.b[761] = (p.p167 > p.p354);
        s.v[761] = if s.b[761] { 1.0 } else { 0.0 };

        if s.b[761] {
            s.store_scalar(762, 0.0);
            s.store_scalar(763, 0.0);
            s.store_scalar(764, 0.0);
            s.store_scalar(765, 0.0);
            s.store_scalar(766, 0.0);
            s.store_scalar(767, 0.0);
            s.store_scalar(768, 0.0);
            s.store_scalar(769, 0.0);
            s.store_scalar(770, 0.0);
            s.copy_ad(771, 84);
            s.copy_ad(772, 85);
            s.store_scalar(773, p.p173);
            s.copy_ad(774, 86);
            s.copy_ad(775, 87);
            s.store_scalar(776, p.p171);
            s.copy_ad(777, 111);
            s.store_scalar(778, s.v[109]);
            s.copy_ad(779, 113);
            s.store_scalar(780, p.p0);
            s.store_scalar(781, p.p167);
            s.copy_ad(782, 32);
            s.store_scalar(783, p.p172);
            s.copy_ad(784, 33);
            s.copy_ad(785, 34);
            s.store_scalar(786, p.p168);
            s.store_scalar(787, p.p182);
            s.store_scalar(788, p.p181);
            s.store_scalar(789, 0.0);
            s.store_scalar(790, p.p183);
            s.store_scalar(791, p.p187);
            s.store_scalar(792, p.p178);
            s.store_scalar(793, p.p179);
            s.store_scalar(794, p.p180);
            s.store_scalar(795, p.p186);
            s.store_scalar(796, p.p185);
            s.store_scalar(797, p.p184);
            s.store_scalar(798, p.p39);
            s.store_scalar(799, p.p47);
            s.store_scalar(800, p.p45);
            s.store_scalar(801, p.p42);
            s.store_scalar(802, p.p2);
            s.store_scalar(803, p.p6);
            s.store_scalar(804, 1.0);
            s.store_scalar(805, 0.0);
            s.store_scalar(806, 0.0);
            s.store_scalar(807, 0.0);
            s.store_scalar(808, 0.0);
            s.store_scalar(809, 0.0);
            s.store_scalar(810, 0.0);
            s.store_scalar(811, 0.0);
            s.store_scalar(812, 0.0);
            s.store_scalar(813, 0.0);
            s.store_scalar(814, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[761] {
            s.store_scalar(815, 0.0);
            s.store_scalar(816, 0.0);
            s.store_scalar(817, 0.0);
            s.store_scalar(818, 0.0);
            s.store_scalar(819, 0.0);
            s.store_scalar(820, 0.0);
            s.store_scalar(821, 0.0);
            s.store_scalar(822, 0.0);
            s.store_scalar(823, 0.0);
            s.store_scalar(824, 0.0);
            s.store_scalar(825, 0.0);
            s.store_scalar(826, 0.0);
            s.store_scalar(827, 0.0);
            s.store_scalar(828, 0.0);
            s.store_scalar(829, 0.0);
            s.store_scalar(830, 0.0);
            s.store_scalar(831, 0.0);
            s.store_scalar(832, 0.0);
            s.store_scalar(833, 0.0);
            s.store_scalar(834, 0.0);
            s.store_scalar(835, 0.0);
            s.store_scalar(836, 0.0);
            s.store_scalar(837, 0.0);
            s.store_scalar(838, 0.0);
            s.store_scalar(839, 0.0);
            s.store_scalar(840, 0.0);
            s.store_scalar(841, 0.0);
            s.store_scalar(842, 0.0);
            s.store_scalar(843, 0.0);
            s.store_scalar(844, 0.0);
            s.store_scalar(845, 0.0);
            s.store_scalar(846, 0.0);
            s.store_scalar(847, 0.0);
            s.store_scalar(848, 0.0);
            s.store_scalar(849, 0.0);
            s.store_scalar(850, 0.0);
            s.store_scalar(851, 0.0);
            s.store_scalar(852, 0.0);
            s.store_scalar(853, 0.0);
            s.store_scalar(854, 0.0);
            s.store_scalar(855, 0.0);
            s.store_scalar(856, 0.0);
            s.store_scalar(857, 0.0);
            s.store_scalar(858, 0.0);
            s.store_scalar(859, 0.0);
            s.store_scalar(860, 0.0);
            s.store_scalar(861, 0.0);
            s.store_scalar(862, 0.0);
            s.store_scalar(863, 0.0);
            s.store_scalar(864, 0.0);
            s.store_scalar(865, 0.0);
            s.store_scalar(866, 0.0);
            s.store_scalar(867, 0.0);
            s.store_scalar(868, 0.0);
            s.store_scalar(869, 0.0);
            s.store_scalar(870, 0.0);
            s.store_scalar(871, 0.0);
            s.store_scalar(872, 0.0);
            s.store_scalar(873, 0.0);
        }

        if s.b[761] {
            s.store_ad_value(870, {
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(772), A::tanh_scaled_input(s.ad_value(772), (0.001 / p.p53)))
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

        if s.b[761] {
            s.store_sub(871, 771, 772);
            s.store_mul(805, 791, 779);
            s.store_ad_value(807, A::add_scaled_product(A::div_scaled_inputs(s.ad_value(787), 1.0, s.ad_value(779), 2.302585092994046), 1.0, s.ad_value(790), s.ad_value(870), 1.0));
            s.store_ad_value(808, A::add_scaled_product(s.ad_value(786), 1.0, s.ad_value(797), A::sub(s.ad_value(777), s.ad_value(778)), 1.0));
            s.store_pow_ad(826, A::div(s.ad_value(777), s.ad_value(778)), s.ad_value(799));
        }

        s.b[874] = (s.v[798] != 0.0);
        s.v[874] = if s.b[874] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[874]) {
            s.store_div_ad_rhs(809, 870, A::pow(A::offset(A::pow(A::div(s.ad_value(870), s.ad_value(798)), s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794))));
        }

        if (s.b[761] && (!s.b[874])) {
            s.store_scalar(809, 0.0);
        }

        if s.b[761] {
            s.store_mul_ad_lhs(806, A::add_scaled_product(s.ad_value(788), 1.0, s.ad_value(809), s.ad_value(789), (-1.0)), 870);
            s.store_sub(769, 808, 806);
            s.store_scaled_mul(811, 807, 779, 2.0);
            s.store_mul(812, 782, 811);
            s.store_sub_scaled_inputs(869, 769, 1.0, 805, (p.p51 * 0.5));
        }

        if s.b[761] {
            let assign11540_ad_e11787: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sub(s.ad_value(771), s.ad_value(871)), A::tanh_scaled_input(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(771), s.ad_value(871)), A::sub(s.ad_value(771), s.ad_value(871))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(868, A::sub(assign11540_ad_e11787, s.ad_value(869)), 805);
        }

        s.b[875] = (s.v[868] > 50.0);
        s.v[875] = if s.b[875] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[875]) {
            s.store_scalar(827, 0.0);
        }

        s.b[876] = (s.v[868] < (-50.0));
        s.v[876] = if s.b[876] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[875])) && s.b[876]) {
            s.store_scalar(827, 1.0);
        }

        if ((s.b[761] && (!s.b[875])) && (!s.b[876])) {
            s.store_div_from_scalar_offset_ad(827, 1.0, A::exp(s.ad_value(868)), 1.0);
        }

        if s.b[761] {
            let assign11600_ad_e11875: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sub(s.ad_value(771), s.ad_value(871)), A::tanh_scaled_input(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(771), s.ad_value(871)), A::sub(s.ad_value(771), s.ad_value(871))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(828, A::sub(assign11600_ad_e11875, A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(805), s.ad_value(827), (-(p.p51 * 0.1)))), 811);
        }

        s.b[877] = (s.v[828] > 50.0);
        s.v[877] = if s.b[877] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[877]) {
            s.store_mul(829, 812, 828);
        }

        s.b[878] = (s.v[828] < (-50.0));
        s.v[878] = if s.b[878] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[877])) && s.b[878]) {
            s.store_mul_exp_rhs(829, 812, 828);
        }

        if ((s.b[761] && (!s.b[877])) && (!s.b[878])) {
            s.store_mul_ad_rhs(829, 812, A::ln_one_plus_exp(s.ad_value(828)));
        }

        if s.b[761] {
            s.store_div_ad_rhs(815, 793, A::mul_offset_rhs(s.ad_value(826), A::div_scaled_product(s.ad_value(795), s.ad_value(829), 1.0, s.ad_value(782), 1.0), 1.0));
            s.store_ad_value(816, A::div_scaled_product3_by_product(s.ad_value(792), A::offset(A::mul(s.ad_value(800), s.ad_value(778)), 1.0), A::offset(A::div_scaled_product(s.ad_value(801), s.ad_value(870), 1.0, s.ad_value(781), 1.0), 1.0), 1.0, A::offset(A::mul(s.ad_value(800), s.ad_value(777)), 1.0), A::offset(A::div_scaled_product(s.ad_value(796), s.ad_value(829), 1.0, s.ad_value(782), 1.0), 1.0), 1.0));
            s.store_add_ad(817, A::div_scaled_product3(s.ad_value(827), s.ad_value(779), s.ad_value(815), 2.0, s.ad_value(781), 1.0), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(827), s.ad_value(816)));
            s.store_ad_value(833, A::div_scaled_product(s.ad_value(816), s.ad_value(781), 1.0, s.ad_value(815), 1.0));
            s.store_ad_value(834, A::add_scaled_product(s.ad_value(833), (-1.0), s.ad_value(833), A::sqrt(A::offset(A::div(A::div_scaled_inputs(s.ad_value(829), 2.0, s.ad_value(782), 1.0), s.ad_value(833)), 1.0)), 1.0));
            s.store_ad_value(835, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(833), 1.0, s.ad_value(827)), 1.0, s.ad_value(811), s.ad_value(827), 1.0));
            s.store_ad_value(770, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(834), 1.0, s.ad_value(827)), 1.0, s.ad_value(811), s.ad_value(827), 1.0));
        }

        if s.b[761] {
            let assign11730_ad_e12104: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(772), s.ad_value(770)), 0.5, A::div(s.ad_value(772), s.ad_value(770)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(772), s.ad_value(770))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(772), s.ad_value(770)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(772), s.ad_value(770)), A::div(s.ad_value(772), s.ad_value(770)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(836, 1.0, A::offset(A::pow(assign11730_ad_e12104, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794)));
        }

        if s.b[761] {
            s.store_mul(837, 772, 836);
        }

        if s.b[761] {
            let assign11750_ad_e12185: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0), A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(838, 1.0, A::offset(A::pow(assign11750_ad_e12185, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794)));
        }

        if s.b[761] {
            s.store_mul_neg_lhs(839, 772, 838);
            s.store_div_ad_lhs(868, A::sub(s.ad_value(771), s.ad_value(869)), 805);
        }

        s.b[879] = (s.v[868] > 50.0);
        s.v[879] = if s.b[879] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[879]) {
            s.store_scalar(810, 0.0);
        }

        s.b[880] = (s.v[868] < (-50.0));
        s.v[880] = if s.b[880] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[879])) && s.b[880]) {
            s.store_scalar(810, 1.0);
        }

        if ((s.b[761] && (!s.b[879])) && (!s.b[880])) {
            s.store_div_from_scalar_offset_ad(810, 1.0, A::exp(s.ad_value(868)), 1.0);
        }

        if s.b[761] {
            s.store_ad_value(813, A::div_scaled_inputs3(s.ad_value(871), 1.0, s.ad_value(839), (-1.0), A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(805), s.ad_value(810), (-(p.p51 * 0.1))), -1.0, s.ad_value(811), 1.0));
        }

        s.b[881] = (s.v[813] > 50.0);
        s.v[881] = if s.b[881] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[881]) {
            s.store_mul(814, 812, 813);
        }

        s.b[882] = (s.v[813] < (-50.0));
        s.v[882] = if s.b[882] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[881])) && s.b[882]) {
            s.store_mul_exp_rhs(814, 812, 813);
        }

        if ((s.b[761] && (!s.b[881])) && (!s.b[882])) {
            s.store_mul_ad_rhs(814, 812, A::ln_one_plus_exp(s.ad_value(813)));
        }

        if s.b[761] {
            s.store_div_ad_lhs(868, A::sub(s.ad_value(871), s.ad_value(869)), 805);
        }

        s.b[883] = (s.v[868] > 50.0);
        s.v[883] = if s.b[883] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[883]) {
            s.store_scalar(840, 0.0);
        }

        s.b[884] = (s.v[868] < (-50.0));
        s.v[884] = if s.b[884] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[883])) && s.b[884]) {
            s.store_scalar(840, 1.0);
        }

        if ((s.b[761] && (!s.b[883])) && (!s.b[884])) {
            s.store_div_from_scalar_offset_ad(840, 1.0, A::exp(s.ad_value(868)), 1.0);
        }

    }

    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[761] {
            s.store_ad_value(841, A::div_scaled_inputs3(s.ad_value(771), 1.0, s.ad_value(837), (-1.0), A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(805), s.ad_value(840), (-(p.p51 * 0.1))), -1.0, s.ad_value(811), 1.0));
        }

        s.b[885] = (s.v[841] > 50.0);
        s.v[885] = if s.b[885] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[885]) {
            s.store_mul(842, 812, 841);
        }

        s.b[886] = (s.v[841] < (-50.0));
        s.v[886] = if s.b[886] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[885])) && s.b[886]) {
            s.store_mul_exp_rhs(842, 812, 841);
        }

        if ((s.b[761] && (!s.b[885])) && (!s.b[886])) {
            s.store_mul_ad_rhs(842, 812, A::ln_one_plus_exp(s.ad_value(841)));
        }

        if s.b[761] {
            s.store_div_ad_lhs(843, A::sub(s.ad_value(814), s.ad_value(842)), 782);
            s.store_div(869, 843, 835);
        }

        if s.b[761] {
            let assign12030_ad_e12462: A = A::pow(A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(869), A::tanh_scaled_input(s.ad_value(869), (0.001 / p.p53)))
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

        if s.b[761] {
            s.store_mul(845, 817, 844);
            s.store_mul_ad_lhs(763, A::mul3_scaled_output(A::mul3(s.ad_value(803), s.ad_value(780), s.ad_value(802)), A::add(s.ad_value(814), s.ad_value(842)), s.ad_value(845), 0.5), 804);
            s.store_scaled_div(818, 787, 779, (1.0 / (2.302585092994046)));
            s.store_scaled_mul(820, 818, 779, 2.0);
            s.store_mul(821, 782, 820);
            s.store_sub_scaled_inputs(873, 808, 1.0, 805, (p.p51 * 0.5));
        }

        if s.b[761] {
            let assign12100_ad_e12566: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sub(s.ad_value(771), s.ad_value(871)), A::tanh_scaled_input(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(771), s.ad_value(871)), A::sub(s.ad_value(771), s.ad_value(871))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(872, A::sub(assign12100_ad_e12566, s.ad_value(873)), 805);
        }

        s.b[887] = (s.v[872] > 50.0);
        s.v[887] = if s.b[887] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[887]) {
            s.store_scalar(830, 0.0);
        }

        s.b[888] = (s.v[872] < (-50.0));
        s.v[888] = if s.b[888] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[887])) && s.b[888]) {
            s.store_scalar(830, 1.0);
        }

        if ((s.b[761] && (!s.b[887])) && (!s.b[888])) {
            s.store_div_from_scalar_offset_ad(830, 1.0, A::exp(s.ad_value(872)), 1.0);
        }

        if s.b[761] {
            let assign12160_ad_e12654: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sub(s.ad_value(771), s.ad_value(871)), A::tanh_scaled_input(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(771), s.ad_value(871)), A::sub(s.ad_value(771), s.ad_value(871))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(831, A::sub(assign12160_ad_e12654, A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(805), s.ad_value(830), (-(p.p51 * 0.1)))), 820);
        }

        s.b[889] = (s.v[831] > 50.0);
        s.v[889] = if s.b[889] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[889]) {
            s.store_mul(832, 821, 831);
        }

        s.b[890] = (s.v[831] < (-50.0));
        s.v[890] = if s.b[890] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[889])) && s.b[890]) {
            s.store_mul_exp_rhs(832, 821, 831);
        }

        if ((s.b[761] && (!s.b[889])) && (!s.b[890])) {
            s.store_mul_ad_rhs(832, 821, A::ln_one_plus_exp(s.ad_value(831)));
        }

        if s.b[761] {
            s.store_div(824, 793, 826);
            s.store_mul_div_ad_rhs(825, 792, A::offset(A::mul(s.ad_value(800), s.ad_value(778)), 1.0), A::offset(A::mul(s.ad_value(800), s.ad_value(777)), 1.0));
            s.store_ad_value(846, A::div_scaled_product(s.ad_value(825), s.ad_value(781), 1.0, s.ad_value(824), 1.0));
            s.store_ad_value(847, A::add_scaled_product(s.ad_value(846), (-1.0), s.ad_value(846), A::sqrt(A::offset(A::div(A::div_scaled_inputs(s.ad_value(832), 2.0, s.ad_value(782), 1.0), s.ad_value(846)), 1.0)), 1.0));
            s.store_ad_value(848, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(847), 1.0, s.ad_value(830)), 1.0, s.ad_value(820), s.ad_value(830), 1.0));
        }

        if s.b[761] {
            let assign12270_ad_e12829: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(772), s.ad_value(848)), 0.5, A::div(s.ad_value(772), s.ad_value(848)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(772), s.ad_value(848))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(772), s.ad_value(848)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(772), s.ad_value(848)), A::div(s.ad_value(772), s.ad_value(848)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(849, 1.0, A::offset(A::pow(assign12270_ad_e12829, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794)));
        }

        if s.b[761] {
            s.store_mul(850, 772, 849);
        }

        if s.b[761] {
            let assign12290_ad_e12910: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0), A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(851, 1.0, A::offset(A::pow(assign12290_ad_e12910, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794)));
        }

        if s.b[761] {
            s.store_mul_neg_lhs(852, 772, 851);
            s.store_div_ad_lhs(872, A::sub(s.ad_value(771), s.ad_value(873)), 805);
        }

        s.b[891] = (s.v[872] > 50.0);
        s.v[891] = if s.b[891] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[891]) {
            s.store_scalar(819, 0.0);
        }

        s.b[892] = (s.v[872] < (-50.0));
        s.v[892] = if s.b[892] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[891])) && s.b[892]) {
            s.store_scalar(819, 1.0);
        }

        if ((s.b[761] && (!s.b[891])) && (!s.b[892])) {
            s.store_div_from_scalar_offset_ad(819, 1.0, A::exp(s.ad_value(872)), 1.0);
        }

        if s.b[761] {
            s.store_ad_value(822, A::div_scaled_inputs3(s.ad_value(871), 1.0, s.ad_value(852), (-1.0), A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(805), s.ad_value(819), (-(p.p51 * 0.1))), -1.0, s.ad_value(820), 1.0));
        }

        s.b[893] = (s.v[822] > 50.0);
        s.v[893] = if s.b[893] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[893]) {
            s.store_mul(823, 821, 822);
        }

        s.b[894] = (s.v[822] < (-50.0));
        s.v[894] = if s.b[894] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[893])) && s.b[894]) {
            s.store_mul_exp_rhs(823, 821, 822);
        }

        if ((s.b[761] && (!s.b[893])) && (!s.b[894])) {
            s.store_mul_ad_rhs(823, 821, A::ln_one_plus_exp(s.ad_value(822)));
        }

        if s.b[761] {
            s.store_div_ad_lhs(872, A::sub(s.ad_value(871), s.ad_value(873)), 805);
        }

        s.b[895] = (s.v[872] > 50.0);
        s.v[895] = if s.b[895] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[895]) {
            s.store_scalar(853, 0.0);
        }

        s.b[896] = (s.v[872] < (-50.0));
        s.v[896] = if s.b[896] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[895])) && s.b[896]) {
            s.store_scalar(853, 1.0);
        }

        if ((s.b[761] && (!s.b[895])) && (!s.b[896])) {
            s.store_div_from_scalar_offset_ad(853, 1.0, A::exp(s.ad_value(872)), 1.0);
        }

        if s.b[761] {
            s.store_ad_value(854, A::div_scaled_inputs3(s.ad_value(771), 1.0, s.ad_value(850), (-1.0), A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(805), s.ad_value(853), (-(p.p51 * 0.1))), -1.0, s.ad_value(820), 1.0));
        }

        s.b[897] = (s.v[854] > 50.0);
        s.v[897] = if s.b[897] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[897]) {
            s.store_mul(855, 821, 854);
        }

        s.b[898] = (s.v[854] < (-50.0));
        s.v[898] = if s.b[898] { 1.0 } else { 0.0 };

        if ((s.b[761] && (!s.b[897])) && s.b[898]) {
            s.store_mul_exp_rhs(855, 821, 854);
        }

        if ((s.b[761] && (!s.b[897])) && (!s.b[898])) {
            s.store_mul_ad_rhs(855, 821, A::ln_one_plus_exp(s.ad_value(854)));
        }

        if s.b[761] {
            s.store_offset_square(856, 823, 1e-38);
            s.store_offset_mul(857, 856, 823, 1e-57);
            s.store_offset_square(858, 855, 1e-38);
            s.store_offset_mul(859, 858, 855, 1e-57);
            s.store_offset_mul(860, 823, 855, 1e-38);
            s.store_ad_value(861, A::div_scaled_inputs3(s.ad_value(856), (2.0 / 3.0), s.ad_value(858), (2.0 / 3.0), s.ad_value(860), (2.0 / 3.0), A::offset(A::add(s.ad_value(823), s.ad_value(855)), 2e-19), 1.0));
            s.store_div_ad(862, A::add_scaled_inputs_products(s.ad_value(857), (2.0 * 2.0), s.ad_value(859), (3.0 * 2.0), s.ad_value(856), s.ad_value(855), (4.0 * 2.0), s.ad_value(858), s.ad_value(823), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(856), 15.0, s.ad_value(858), 15.0, s.ad_value(860), (2.0 * 15.0)));
            s.store_sub(863, 861, 862);
            s.copy_ad(864, 862);
            s.store_mul_ad_lhs(764, A::mul3(A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(781)), s.ad_value(803), s.ad_value(863)), 804);
            s.store_mul_ad_lhs(765, A::mul3(A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(781)), s.ad_value(803), s.ad_value(864)), 804);
        }

        s.b[899] = (s.v[773] == 1.0);
        s.v[899] = if s.b[899] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[899]) {
            s.store_ad_value(865, A::div_scaled_inputs3(s.ad_value(774), 1.0, s.ad_value(808), -1.0, s.ad_value(805), (-(-(p.p51 * 0.5))), s.ad_value(820), 1.0));
        }

        s.b[900] = (s.v[865] > 50.0);
        s.v[900] = if s.b[900] { 1.0 } else { 0.0 };

        if ((s.b[761] && s.b[899]) && s.b[900]) {
            s.copy_ad(868, 865);
        }

        s.b[901] = (s.v[865] < (-50.0));
        s.v[901] = if s.b[901] { 1.0 } else { 0.0 };

        if (((s.b[761] && s.b[899]) && (!s.b[900])) && s.b[901]) {
            s.store_exp(868, 865);
        }

        if (((s.b[761] && s.b[899]) && (!s.b[900])) && (!s.b[901])) {
            s.store_ln_one_plus_exp(868, 865);
        }

        if (s.b[761] && s.b[899]) {
            s.store_mul_ad_product_lhs(766, A::mul3(A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(803)), s.ad_value(784), s.ad_value(820)), s.ad_value(868), 804);
            s.store_ad_value(866, A::div_scaled_inputs3(s.ad_value(775), 1.0, s.ad_value(808), -1.0, s.ad_value(805), (-(-(p.p51 * 0.5))), s.ad_value(820), 1.0));
        }

        s.b[902] = (s.v[866] > 50.0);
        s.v[902] = if s.b[902] { 1.0 } else { 0.0 };

        if ((s.b[761] && s.b[899]) && s.b[902]) {
            s.copy_ad(868, 866);
        }

        s.b[903] = (s.v[866] < (-50.0));
        s.v[903] = if s.b[903] { 1.0 } else { 0.0 };

        if (((s.b[761] && s.b[899]) && (!s.b[902])) && s.b[903]) {
            s.store_exp(868, 866);
        }

        if (((s.b[761] && s.b[899]) && (!s.b[902])) && (!s.b[903])) {
            s.store_ln_one_plus_exp(868, 866);
        }

        if (s.b[761] && s.b[899]) {
            s.store_mul_ad_product_lhs(767, A::mul3(A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(803)), s.ad_value(785), s.ad_value(820)), s.ad_value(868), 804);
        }

        if (s.b[761] && (!s.b[899])) {
            s.store_scalar(766, 0.0);
            s.store_scalar(767, 0.0);
        }

        s.b[904] = (s.v[776] == 1.0);
        s.v[904] = if s.b[904] { 1.0 } else { 0.0 };

        if (s.b[761] && s.b[904]) {
            s.store_ad_value(867, A::div_scaled_inputs3(s.ad_value(771), 1.0, s.ad_value(808), -1.0, s.ad_value(805), (-(-(p.p51 * 0.5))), s.ad_value(820), 1.0));
        }

        s.b[905] = (s.v[867] > 50.0);
        s.v[905] = if s.b[905] { 1.0 } else { 0.0 };

        if ((s.b[761] && s.b[904]) && s.b[905]) {
            s.copy_ad(868, 867);
        }

        s.b[906] = (s.v[867] < (-50.0));
        s.v[906] = if s.b[906] { 1.0 } else { 0.0 };

        if (((s.b[761] && s.b[904]) && (!s.b[905])) && s.b[906]) {
            s.store_exp(868, 867);
        }

        if (((s.b[761] && s.b[904]) && (!s.b[905])) && (!s.b[906])) {
            s.store_ln_one_plus_exp(868, 867);
        }

        if (s.b[761] && s.b[904]) {
            s.store_mul_ad_product_lhs(768, A::mul3(A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(803)), s.ad_value(783), s.ad_value(820)), s.ad_value(868), 804);
        }

        if (s.b[761] && (!s.b[904])) {
            s.store_scalar(768, 0.0);
        }

        if s.b[761] {
            s.copy_ad(762, 763);
            s.copy_ad(190, 763);
            s.copy_ad(191, 764);
            s.copy_ad(192, 765);
            s.copy_ad(193, 766);
            s.copy_ad(194, 767);
            s.copy_ad(195, 768);
            s.copy_ad(190, 762);
        }

        s.b[907] = (p.p166 == 1.0);
        s.v[907] = if s.b[907] { 1.0 } else { 0.0 };

        s.v[166] = 0.0;

        s.v[167] = 0.0;

        s.v[168] = 0.0;

        s.v[169] = 0.0;

        s.v[170] = 0.0;

        s.v[171] = 0.0;

        s.b[908] = (p.p79 > p.p354);
        s.v[908] = if s.b[908] { 1.0 } else { 0.0 };

        if s.b[908] {
            s.store_scalar(909, 0.0);
            s.store_scalar(910, 0.0);
            s.store_scalar(911, 0.0);
            s.store_scalar(912, 0.0);
            s.store_scalar(913, 0.0);
            s.store_scalar(914, 0.0);
            s.store_scalar(915, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[908] {
            s.store_scalar(916, 0.0);
            s.store_scalar(917, 0.0);
            s.copy_ad(918, 60);
            s.copy_ad(919, 61);
            s.store_scalar(920, p.p85);
            s.copy_ad(921, 62);
            s.copy_ad(922, 63);
            s.store_scalar(923, p.p83);
            s.copy_ad(924, 111);
            s.store_scalar(925, s.v[109]);
            s.copy_ad(926, 113);
            s.store_scalar(927, p.p0);
            s.store_scalar(928, p.p79);
            s.copy_ad(929, 20);
            s.store_scalar(930, p.p84);
            s.copy_ad(931, 21);
            s.copy_ad(932, 22);
            s.store_scalar(933, p.p80);
            s.store_scalar(934, p.p94);
            s.store_scalar(935, p.p93);
            s.store_scalar(936, 0.0);
            s.store_scalar(937, p.p95);
            s.store_scalar(938, p.p99);
            s.store_scalar(939, p.p90);
            s.store_scalar(940, p.p91);
            s.store_scalar(941, p.p92);
            s.store_scalar(942, p.p98);
            s.store_scalar(943, p.p97);
            s.store_scalar(944, p.p96);
            s.store_scalar(945, p.p39);
            s.store_scalar(946, p.p47);
            s.store_scalar(947, p.p45);
            s.store_scalar(948, p.p42);
            s.store_scalar(949, p.p2);
            s.store_scalar(950, p.p6);
            s.store_scalar(951, 1.0);
            s.store_scalar(952, 0.0);
            s.store_scalar(953, 0.0);
            s.store_scalar(954, 0.0);
            s.store_scalar(955, 0.0);
            s.store_scalar(956, 0.0);
            s.store_scalar(957, 0.0);
            s.store_scalar(958, 0.0);
            s.store_scalar(959, 0.0);
            s.store_scalar(960, 0.0);
            s.store_scalar(961, 0.0);
            s.store_scalar(962, 0.0);
            s.store_scalar(963, 0.0);
            s.store_scalar(964, 0.0);
            s.store_scalar(965, 0.0);
            s.store_scalar(966, 0.0);
            s.store_scalar(967, 0.0);
            s.store_scalar(968, 0.0);
            s.store_scalar(969, 0.0);
            s.store_scalar(970, 0.0);
            s.store_scalar(971, 0.0);
            s.store_scalar(972, 0.0);
            s.store_scalar(973, 0.0);
            s.store_scalar(974, 0.0);
            s.store_scalar(975, 0.0);
            s.store_scalar(976, 0.0);
            s.store_scalar(977, 0.0);
            s.store_scalar(978, 0.0);
            s.store_scalar(979, 0.0);
            s.store_scalar(980, 0.0);
            s.store_scalar(981, 0.0);
            s.store_scalar(982, 0.0);
            s.store_scalar(983, 0.0);
            s.store_scalar(984, 0.0);
            s.store_scalar(985, 0.0);
            s.store_scalar(986, 0.0);
            s.store_scalar(987, 0.0);
            s.store_scalar(988, 0.0);
            s.store_scalar(989, 0.0);
            s.store_scalar(990, 0.0);
            s.store_scalar(991, 0.0);
            s.store_scalar(992, 0.0);
            s.store_scalar(993, 0.0);
            s.store_scalar(994, 0.0);
            s.store_scalar(995, 0.0);
            s.store_scalar(996, 0.0);
            s.store_scalar(997, 0.0);
            s.store_scalar(998, 0.0);
            s.store_scalar(999, 0.0);
            s.store_scalar(1000, 0.0);
            s.store_scalar(1001, 0.0);
            s.store_scalar(1002, 0.0);
            s.store_scalar(1003, 0.0);
            s.store_scalar(1004, 0.0);
            s.store_scalar(1005, 0.0);
            s.store_scalar(1006, 0.0);
            s.store_scalar(1007, 0.0);
            s.store_scalar(1008, 0.0);
            s.store_scalar(1009, 0.0);
            s.store_scalar(1010, 0.0);
            s.store_scalar(1011, 0.0);
            s.store_scalar(1012, 0.0);
            s.store_scalar(1013, 0.0);
            s.store_scalar(1014, 0.0);
            s.store_scalar(1015, 0.0);
            s.store_scalar(1016, 0.0);
            s.store_scalar(1017, 0.0);
            s.store_scalar(1018, 0.0);
            s.store_scalar(1019, 0.0);
            s.store_scalar(1020, 0.0);
        }

        if s.b[908] {
            s.store_ad_value(1017, {
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(919), A::tanh_scaled_input(s.ad_value(919), (0.001 / p.p53)))
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

        if s.b[908] {
            s.store_sub(1018, 918, 919);
            s.store_mul(952, 938, 926);
            s.store_ad_value(954, A::add_scaled_product(A::div_scaled_inputs(s.ad_value(934), 1.0, s.ad_value(926), 2.302585092994046), 1.0, s.ad_value(937), s.ad_value(1017), 1.0));
            s.store_ad_value(955, A::add_scaled_product(s.ad_value(933), 1.0, s.ad_value(944), A::sub(s.ad_value(924), s.ad_value(925)), 1.0));
            s.store_pow_ad(973, A::div(s.ad_value(924), s.ad_value(925)), s.ad_value(946));
        }

        s.b[1021] = (s.v[945] != 0.0);
        s.v[1021] = if s.b[1021] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1021]) {
            s.store_div_ad_rhs(956, 1017, A::pow(A::offset(A::pow(A::div(s.ad_value(1017), s.ad_value(945)), s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941))));
        }

        if (s.b[908] && (!s.b[1021])) {
            s.store_scalar(956, 0.0);
        }

        if s.b[908] {
            s.store_mul_ad_lhs(953, A::add_scaled_product(s.ad_value(935), 1.0, s.ad_value(956), s.ad_value(936), (-1.0)), 1017);
            s.store_sub(916, 955, 953);
            s.store_scaled_mul(958, 954, 926, 2.0);
            s.store_mul(959, 929, 958);
            s.store_sub_scaled_inputs(1016, 916, 1.0, 952, (p.p51 * 0.5));
        }

        if s.b[908] {
            let assign14380_ad_e14211: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(918), s.ad_value(1018)), A::sub(s.ad_value(918), s.ad_value(1018))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1015, A::sub(assign14380_ad_e14211, s.ad_value(1016)), 952);
        }

        s.b[1022] = (s.v[1015] > 50.0);
        s.v[1022] = if s.b[1022] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1022]) {
            s.store_scalar(974, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1023] = (s.v[1015] < (-50.0));
        s.v[1023] = if s.b[1023] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1022])) && s.b[1023]) {
            s.store_scalar(974, 1.0);
        }

        if ((s.b[908] && (!s.b[1022])) && (!s.b[1023])) {
            s.store_div_from_scalar_offset_ad(974, 1.0, A::exp(s.ad_value(1015)), 1.0);
        }

        if s.b[908] {
            let assign14440_ad_e14299: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(918), s.ad_value(1018)), A::sub(s.ad_value(918), s.ad_value(1018))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(975, A::sub(assign14440_ad_e14299, A::add_scaled_product(s.ad_value(916), 1.0, s.ad_value(952), s.ad_value(974), (-(p.p51 * 0.1)))), 958);
        }

        s.b[1024] = (s.v[975] > 50.0);
        s.v[1024] = if s.b[1024] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1024]) {
            s.store_mul(976, 959, 975);
        }

        s.b[1025] = (s.v[975] < (-50.0));
        s.v[1025] = if s.b[1025] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1024])) && s.b[1025]) {
            s.store_mul_exp_rhs(976, 959, 975);
        }

        if ((s.b[908] && (!s.b[1024])) && (!s.b[1025])) {
            s.store_mul_ad_rhs(976, 959, A::ln_one_plus_exp(s.ad_value(975)));
        }

        if s.b[908] {
            s.store_div_ad_rhs(962, 940, A::mul_offset_rhs(s.ad_value(973), A::div_scaled_product(s.ad_value(942), s.ad_value(976), 1.0, s.ad_value(929), 1.0), 1.0));
            s.store_ad_value(963, A::div_scaled_product3_by_product(s.ad_value(939), A::offset(A::mul(s.ad_value(947), s.ad_value(925)), 1.0), A::offset(A::div_scaled_product(s.ad_value(948), s.ad_value(1017), 1.0, s.ad_value(928), 1.0), 1.0), 1.0, A::offset(A::mul(s.ad_value(947), s.ad_value(924)), 1.0), A::offset(A::div_scaled_product(s.ad_value(943), s.ad_value(976), 1.0, s.ad_value(929), 1.0), 1.0), 1.0));
            s.store_add_ad(964, A::div_scaled_product3(s.ad_value(974), s.ad_value(926), s.ad_value(962), 2.0, s.ad_value(928), 1.0), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(974), s.ad_value(963)));
            s.store_ad_value(980, A::div_scaled_product(s.ad_value(963), s.ad_value(928), 1.0, s.ad_value(962), 1.0));
            s.store_ad_value(981, A::add_scaled_product(s.ad_value(980), (-1.0), s.ad_value(980), A::sqrt(A::offset(A::div(A::div_scaled_inputs(s.ad_value(976), 2.0, s.ad_value(929), 1.0), s.ad_value(980)), 1.0)), 1.0));
            s.store_ad_value(982, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(980), 1.0, s.ad_value(974)), 1.0, s.ad_value(958), s.ad_value(974), 1.0));
            s.store_ad_value(917, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(981), 1.0, s.ad_value(974)), 1.0, s.ad_value(958), s.ad_value(974), 1.0));
        }

        if s.b[908] {
            let assign14570_ad_e14528: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(919), s.ad_value(917)), 0.5, A::div(s.ad_value(919), s.ad_value(917)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(919), s.ad_value(917))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(919), s.ad_value(917)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(919), s.ad_value(917)), A::div(s.ad_value(919), s.ad_value(917)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(983, 1.0, A::offset(A::pow(assign14570_ad_e14528, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul(984, 919, 983);
        }

        if s.b[908] {
            let assign14590_ad_e14609: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0), A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(985, 1.0, A::offset(A::pow(assign14590_ad_e14609, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul_neg_lhs(986, 919, 985);
            s.store_div_ad_lhs(1015, A::sub(s.ad_value(918), s.ad_value(1016)), 952);
        }

        s.b[1026] = (s.v[1015] > 50.0);
        s.v[1026] = if s.b[1026] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1026]) {
            s.store_scalar(957, 0.0);
        }

        s.b[1027] = (s.v[1015] < (-50.0));
        s.v[1027] = if s.b[1027] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1026])) && s.b[1027]) {
            s.store_scalar(957, 1.0);
        }

        if ((s.b[908] && (!s.b[1026])) && (!s.b[1027])) {
            s.store_div_from_scalar_offset_ad(957, 1.0, A::exp(s.ad_value(1015)), 1.0);
        }

        if s.b[908] {
            s.store_ad_value(960, A::div_scaled_inputs3(s.ad_value(1018), 1.0, s.ad_value(986), (-1.0), A::add_scaled_product(s.ad_value(916), 1.0, s.ad_value(952), s.ad_value(957), (-(p.p51 * 0.1))), -1.0, s.ad_value(958), 1.0));
        }

        s.b[1028] = (s.v[960] > 50.0);
        s.v[1028] = if s.b[1028] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1028]) {
            s.store_mul(961, 959, 960);
        }

        s.b[1029] = (s.v[960] < (-50.0));
        s.v[1029] = if s.b[1029] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1028])) && s.b[1029]) {
            s.store_mul_exp_rhs(961, 959, 960);
        }

        if ((s.b[908] && (!s.b[1028])) && (!s.b[1029])) {
            s.store_mul_ad_rhs(961, 959, A::ln_one_plus_exp(s.ad_value(960)));
        }

        if s.b[908] {
            s.store_div_ad_lhs(1015, A::sub(s.ad_value(1018), s.ad_value(1016)), 952);
        }

        s.b[1030] = (s.v[1015] > 50.0);
        s.v[1030] = if s.b[1030] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1030]) {
            s.store_scalar(987, 0.0);
        }

        s.b[1031] = (s.v[1015] < (-50.0));
        s.v[1031] = if s.b[1031] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1030])) && s.b[1031]) {
            s.store_scalar(987, 1.0);
        }

        if ((s.b[908] && (!s.b[1030])) && (!s.b[1031])) {
            s.store_div_from_scalar_offset_ad(987, 1.0, A::exp(s.ad_value(1015)), 1.0);
        }

        if s.b[908] {
            s.store_ad_value(988, A::div_scaled_inputs3(s.ad_value(918), 1.0, s.ad_value(984), (-1.0), A::add_scaled_product(s.ad_value(916), 1.0, s.ad_value(952), s.ad_value(987), (-(p.p51 * 0.1))), -1.0, s.ad_value(958), 1.0));
        }

        s.b[1032] = (s.v[988] > 50.0);
        s.v[1032] = if s.b[1032] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1032]) {
            s.store_mul(989, 959, 988);
        }

        s.b[1033] = (s.v[988] < (-50.0));
        s.v[1033] = if s.b[1033] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1032])) && s.b[1033]) {
            s.store_mul_exp_rhs(989, 959, 988);
        }

        if ((s.b[908] && (!s.b[1032])) && (!s.b[1033])) {
            s.store_mul_ad_rhs(989, 959, A::ln_one_plus_exp(s.ad_value(988)));
        }

        if s.b[908] {
            s.store_div_ad_lhs(990, A::sub(s.ad_value(961), s.ad_value(989)), 929);
            s.store_div(1016, 990, 982);
        }

        if s.b[908] {
            let assign14870_ad_e14886: A = A::pow(A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(1016), A::tanh_scaled_input(s.ad_value(1016), (0.001 / p.p53)))
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

        if s.b[908] {
            s.store_mul(992, 964, 991);
            s.store_mul_ad_lhs(910, A::mul3_scaled_output(A::mul3(s.ad_value(950), s.ad_value(927), s.ad_value(949)), A::add(s.ad_value(961), s.ad_value(989)), s.ad_value(992), 0.5), 951);
            s.store_scaled_div(965, 934, 926, (1.0 / (2.302585092994046)));
            s.store_scaled_mul(967, 965, 926, 2.0);
            s.store_mul(968, 929, 967);
            s.store_sub_scaled_inputs(1020, 955, 1.0, 952, (p.p51 * 0.5));
        }

        if s.b[908] {
            let assign14940_ad_e14990: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(918), s.ad_value(1018)), A::sub(s.ad_value(918), s.ad_value(1018))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1019, A::sub(assign14940_ad_e14990, s.ad_value(1020)), 952);
        }

        s.b[1034] = (s.v[1019] > 50.0);
        s.v[1034] = if s.b[1034] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1034]) {
            s.store_scalar(977, 0.0);
        }

        s.b[1035] = (s.v[1019] < (-50.0));
        s.v[1035] = if s.b[1035] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1034])) && s.b[1035]) {
            s.store_scalar(977, 1.0);
        }

        if ((s.b[908] && (!s.b[1034])) && (!s.b[1035])) {
            s.store_div_from_scalar_offset_ad(977, 1.0, A::exp(s.ad_value(1019)), 1.0);
        }

        if s.b[908] {
            let assign15000_ad_e15078: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(918), s.ad_value(1018)), A::sub(s.ad_value(918), s.ad_value(1018))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(978, A::sub(assign15000_ad_e15078, A::add_scaled_product(s.ad_value(955), 1.0, s.ad_value(952), s.ad_value(977), (-(p.p51 * 0.1)))), 967);
        }

        s.b[1036] = (s.v[978] > 50.0);
        s.v[1036] = if s.b[1036] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1036]) {
            s.store_mul(979, 968, 978);
        }

        s.b[1037] = (s.v[978] < (-50.0));
        s.v[1037] = if s.b[1037] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1036])) && s.b[1037]) {
            s.store_mul_exp_rhs(979, 968, 978);
        }

        if ((s.b[908] && (!s.b[1036])) && (!s.b[1037])) {
            s.store_mul_ad_rhs(979, 968, A::ln_one_plus_exp(s.ad_value(978)));
        }

        if s.b[908] {
            s.store_div(971, 940, 973);
            s.store_mul_div_ad_rhs(972, 939, A::offset(A::mul(s.ad_value(947), s.ad_value(925)), 1.0), A::offset(A::mul(s.ad_value(947), s.ad_value(924)), 1.0));
            s.store_ad_value(993, A::div_scaled_product(s.ad_value(972), s.ad_value(928), 1.0, s.ad_value(971), 1.0));
            s.store_ad_value(994, A::add_scaled_product(s.ad_value(993), (-1.0), s.ad_value(993), A::sqrt(A::offset(A::div(A::div_scaled_inputs(s.ad_value(979), 2.0, s.ad_value(929), 1.0), s.ad_value(993)), 1.0)), 1.0));
            s.store_ad_value(995, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(994), 1.0, s.ad_value(977)), 1.0, s.ad_value(967), s.ad_value(977), 1.0));
        }

        if s.b[908] {
            let assign15110_ad_e15253: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(919), s.ad_value(995)), 0.5, A::div(s.ad_value(919), s.ad_value(995)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(919), s.ad_value(995))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(919), s.ad_value(995)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(919), s.ad_value(995)), A::div(s.ad_value(919), s.ad_value(995)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(996, 1.0, A::offset(A::pow(assign15110_ad_e15253, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul(997, 919, 996);
        }

        if s.b[908] {
            let assign15130_ad_e15334: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0), A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(998, 1.0, A::offset(A::pow(assign15130_ad_e15334, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul_neg_lhs(999, 919, 998);
            s.store_div_ad_lhs(1019, A::sub(s.ad_value(918), s.ad_value(1020)), 952);
        }

        s.b[1038] = (s.v[1019] > 50.0);
        s.v[1038] = if s.b[1038] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1038]) {
            s.store_scalar(966, 0.0);
        }

        s.b[1039] = (s.v[1019] < (-50.0));
        s.v[1039] = if s.b[1039] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1038])) && s.b[1039]) {
            s.store_scalar(966, 1.0);
        }

        if ((s.b[908] && (!s.b[1038])) && (!s.b[1039])) {
            s.store_div_from_scalar_offset_ad(966, 1.0, A::exp(s.ad_value(1019)), 1.0);
        }

        if s.b[908] {
            s.store_ad_value(969, A::div_scaled_inputs3(s.ad_value(1018), 1.0, s.ad_value(999), (-1.0), A::add_scaled_product(s.ad_value(955), 1.0, s.ad_value(952), s.ad_value(966), (-(p.p51 * 0.1))), -1.0, s.ad_value(967), 1.0));
        }

        s.b[1040] = (s.v[969] > 50.0);
        s.v[1040] = if s.b[1040] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1040]) {
            s.store_mul(970, 968, 969);
        }

        s.b[1041] = (s.v[969] < (-50.0));
        s.v[1041] = if s.b[1041] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1040])) && s.b[1041]) {
            s.store_mul_exp_rhs(970, 968, 969);
        }

        if ((s.b[908] && (!s.b[1040])) && (!s.b[1041])) {
            s.store_mul_ad_rhs(970, 968, A::ln_one_plus_exp(s.ad_value(969)));
        }

        if s.b[908] {
            s.store_div_ad_lhs(1019, A::sub(s.ad_value(1018), s.ad_value(1020)), 952);
        }

        s.b[1042] = (s.v[1019] > 50.0);
        s.v[1042] = if s.b[1042] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1042]) {
            s.store_scalar(1000, 0.0);
        }

        s.b[1043] = (s.v[1019] < (-50.0));
        s.v[1043] = if s.b[1043] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1042])) && s.b[1043]) {
            s.store_scalar(1000, 1.0);
        }

        if ((s.b[908] && (!s.b[1042])) && (!s.b[1043])) {
            s.store_div_from_scalar_offset_ad(1000, 1.0, A::exp(s.ad_value(1019)), 1.0);
        }

        if s.b[908] {
            s.store_ad_value(1001, A::div_scaled_inputs3(s.ad_value(918), 1.0, s.ad_value(997), (-1.0), A::add_scaled_product(s.ad_value(955), 1.0, s.ad_value(952), s.ad_value(1000), (-(p.p51 * 0.1))), -1.0, s.ad_value(967), 1.0));
        }

        s.b[1044] = (s.v[1001] > 50.0);
        s.v[1044] = if s.b[1044] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1044]) {
            s.store_mul(1002, 968, 1001);
        }

        s.b[1045] = (s.v[1001] < (-50.0));
        s.v[1045] = if s.b[1045] { 1.0 } else { 0.0 };

        if ((s.b[908] && (!s.b[1044])) && s.b[1045]) {
            s.store_mul_exp_rhs(1002, 968, 1001);
        }

        if ((s.b[908] && (!s.b[1044])) && (!s.b[1045])) {
            s.store_mul_ad_rhs(1002, 968, A::ln_one_plus_exp(s.ad_value(1001)));
        }

        if s.b[908] {
            s.store_offset_square(1003, 970, 1e-38);
            s.store_offset_mul(1004, 1003, 970, 1e-57);
            s.store_offset_square(1005, 1002, 1e-38);
            s.store_offset_mul(1006, 1005, 1002, 1e-57);
            s.store_offset_mul(1007, 970, 1002, 1e-38);
            s.store_ad_value(1008, A::div_scaled_inputs3(s.ad_value(1003), (2.0 / 3.0), s.ad_value(1005), (2.0 / 3.0), s.ad_value(1007), (2.0 / 3.0), A::offset(A::add(s.ad_value(970), s.ad_value(1002)), 2e-19), 1.0));
            s.store_div_ad(1009, A::add_scaled_inputs_products(s.ad_value(1004), (2.0 * 2.0), s.ad_value(1006), (3.0 * 2.0), s.ad_value(1003), s.ad_value(1002), (4.0 * 2.0), s.ad_value(1005), s.ad_value(970), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1003), 15.0, s.ad_value(1005), 15.0, s.ad_value(1007), (2.0 * 15.0)));
            s.store_sub(1010, 1008, 1009);
            s.copy_ad(1011, 1009);
        }

    }

    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[908] {
            s.store_mul_ad_lhs(911, A::mul3(A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(928)), s.ad_value(950), s.ad_value(1010)), 951);
            s.store_mul_ad_lhs(912, A::mul3(A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(928)), s.ad_value(950), s.ad_value(1011)), 951);
        }

        s.b[1046] = (s.v[920] == 1.0);
        s.v[1046] = if s.b[1046] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1046]) {
            s.store_ad_value(1012, A::div_scaled_inputs3(s.ad_value(921), 1.0, s.ad_value(955), -1.0, s.ad_value(952), (-(-(p.p51 * 0.5))), s.ad_value(967), 1.0));
        }

        s.b[1047] = (s.v[1012] > 50.0);
        s.v[1047] = if s.b[1047] { 1.0 } else { 0.0 };

        if ((s.b[908] && s.b[1046]) && s.b[1047]) {
            s.copy_ad(1015, 1012);
        }

        s.b[1048] = (s.v[1012] < (-50.0));
        s.v[1048] = if s.b[1048] { 1.0 } else { 0.0 };

        if (((s.b[908] && s.b[1046]) && (!s.b[1047])) && s.b[1048]) {
            s.store_exp(1015, 1012);
        }

        if (((s.b[908] && s.b[1046]) && (!s.b[1047])) && (!s.b[1048])) {
            s.store_ln_one_plus_exp(1015, 1012);
        }

        if (s.b[908] && s.b[1046]) {
            s.store_mul_ad_product_lhs(913, A::mul3(A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(950)), s.ad_value(931), s.ad_value(967)), s.ad_value(1015), 951);
            s.store_ad_value(1013, A::div_scaled_inputs3(s.ad_value(922), 1.0, s.ad_value(955), -1.0, s.ad_value(952), (-(-(p.p51 * 0.5))), s.ad_value(967), 1.0));
        }

        s.b[1049] = (s.v[1013] > 50.0);
        s.v[1049] = if s.b[1049] { 1.0 } else { 0.0 };

        if ((s.b[908] && s.b[1046]) && s.b[1049]) {
            s.copy_ad(1015, 1013);
        }

        s.b[1050] = (s.v[1013] < (-50.0));
        s.v[1050] = if s.b[1050] { 1.0 } else { 0.0 };

        if (((s.b[908] && s.b[1046]) && (!s.b[1049])) && s.b[1050]) {
            s.store_exp(1015, 1013);
        }

        if (((s.b[908] && s.b[1046]) && (!s.b[1049])) && (!s.b[1050])) {
            s.store_ln_one_plus_exp(1015, 1013);
        }

        if (s.b[908] && s.b[1046]) {
            s.store_mul_ad_product_lhs(914, A::mul3(A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(950)), s.ad_value(932), s.ad_value(967)), s.ad_value(1015), 951);
        }

        if (s.b[908] && (!s.b[1046])) {
            s.store_scalar(913, 0.0);
            s.store_scalar(914, 0.0);
        }

        s.b[1051] = (s.v[923] == 1.0);
        s.v[1051] = if s.b[1051] { 1.0 } else { 0.0 };

        if (s.b[908] && s.b[1051]) {
            s.store_ad_value(1014, A::div_scaled_inputs3(s.ad_value(918), 1.0, s.ad_value(955), -1.0, s.ad_value(952), (-(-(p.p51 * 0.5))), s.ad_value(967), 1.0));
        }

        s.b[1052] = (s.v[1014] > 50.0);
        s.v[1052] = if s.b[1052] { 1.0 } else { 0.0 };

        if ((s.b[908] && s.b[1051]) && s.b[1052]) {
            s.copy_ad(1015, 1014);
        }

        s.b[1053] = (s.v[1014] < (-50.0));
        s.v[1053] = if s.b[1053] { 1.0 } else { 0.0 };

        if (((s.b[908] && s.b[1051]) && (!s.b[1052])) && s.b[1053]) {
            s.store_exp(1015, 1014);
        }

        if (((s.b[908] && s.b[1051]) && (!s.b[1052])) && (!s.b[1053])) {
            s.store_ln_one_plus_exp(1015, 1014);
        }

        if (s.b[908] && s.b[1051]) {
            s.store_mul_ad_product_lhs(915, A::mul3(A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(950)), s.ad_value(930), s.ad_value(967)), s.ad_value(1015), 951);
        }

        if (s.b[908] && (!s.b[1051])) {
            s.store_scalar(915, 0.0);
        }

        if s.b[908] {
            s.copy_ad(909, 910);
            s.copy_ad(166, 910);
            s.copy_ad(167, 911);
            s.copy_ad(168, 912);
            s.copy_ad(169, 913);
            s.copy_ad(170, 914);
            s.copy_ad(171, 915);
            s.copy_ad(166, 909);
        }

        s.b[1054] = (p.p78 == 1.0);
        s.v[1054] = if s.b[1054] { 1.0 } else { 0.0 };

        s.v[172] = 0.0;

        s.v[173] = 0.0;

        s.v[174] = 0.0;

        s.v[175] = 0.0;

        s.v[176] = 0.0;

        s.v[177] = 0.0;

        s.b[1055] = (p.p101 > p.p354);
        s.v[1055] = if s.b[1055] { 1.0 } else { 0.0 };

        if s.b[1055] {
            s.store_scalar(1056, 0.0);
            s.store_scalar(1057, 0.0);
            s.store_scalar(1058, 0.0);
            s.store_scalar(1059, 0.0);
            s.store_scalar(1060, 0.0);
            s.store_scalar(1061, 0.0);
            s.store_scalar(1062, 0.0);
            s.store_scalar(1063, 0.0);
            s.store_scalar(1064, 0.0);
            s.copy_ad(1065, 66);
            s.copy_ad(1066, 67);
            s.store_scalar(1067, p.p107);
            s.copy_ad(1068, 68);
            s.copy_ad(1069, 69);
            s.store_scalar(1070, p.p105);
            s.copy_ad(1071, 111);
            s.store_scalar(1072, s.v[109]);
            s.copy_ad(1073, 113);
            s.store_scalar(1074, p.p0);
            s.store_scalar(1075, p.p101);
            s.copy_ad(1076, 23);
            s.store_scalar(1077, p.p106);
            s.copy_ad(1078, 24);
            s.copy_ad(1079, 25);
            s.store_scalar(1080, p.p102);
            s.store_scalar(1081, p.p116);
            s.store_scalar(1082, p.p115);
            s.store_scalar(1083, 0.0);
            s.store_scalar(1084, p.p117);
            s.store_scalar(1085, p.p121);
            s.store_scalar(1086, p.p112);
            s.store_scalar(1087, p.p113);
            s.store_scalar(1088, p.p114);
            s.store_scalar(1089, p.p120);
            s.store_scalar(1090, p.p119);
            s.store_scalar(1091, p.p118);
            s.store_scalar(1092, p.p39);
            s.store_scalar(1093, p.p47);
            s.store_scalar(1094, p.p45);
            s.store_scalar(1095, p.p42);
            s.store_scalar(1096, p.p2);
            s.store_scalar(1097, p.p6);
            s.store_scalar(1098, 1.0);
            s.store_scalar(1099, 0.0);
            s.store_scalar(1100, 0.0);
            s.store_scalar(1101, 0.0);
            s.store_scalar(1102, 0.0);
            s.store_scalar(1103, 0.0);
            s.store_scalar(1104, 0.0);
            s.store_scalar(1105, 0.0);
            s.store_scalar(1106, 0.0);
            s.store_scalar(1107, 0.0);
            s.store_scalar(1108, 0.0);
            s.store_scalar(1109, 0.0);
            s.store_scalar(1110, 0.0);
            s.store_scalar(1111, 0.0);
            s.store_scalar(1112, 0.0);
            s.store_scalar(1113, 0.0);
            s.store_scalar(1114, 0.0);
            s.store_scalar(1115, 0.0);
            s.store_scalar(1116, 0.0);
            s.store_scalar(1117, 0.0);
            s.store_scalar(1118, 0.0);
            s.store_scalar(1119, 0.0);
            s.store_scalar(1120, 0.0);
            s.store_scalar(1121, 0.0);
            s.store_scalar(1122, 0.0);
            s.store_scalar(1123, 0.0);
            s.store_scalar(1124, 0.0);
            s.store_scalar(1125, 0.0);
            s.store_scalar(1126, 0.0);
            s.store_scalar(1127, 0.0);
            s.store_scalar(1128, 0.0);
            s.store_scalar(1129, 0.0);
            s.store_scalar(1130, 0.0);
            s.store_scalar(1131, 0.0);
            s.store_scalar(1132, 0.0);
            s.store_scalar(1133, 0.0);
            s.store_scalar(1134, 0.0);
            s.store_scalar(1135, 0.0);
            s.store_scalar(1136, 0.0);
            s.store_scalar(1137, 0.0);
            s.store_scalar(1138, 0.0);
            s.store_scalar(1139, 0.0);
            s.store_scalar(1140, 0.0);
            s.store_scalar(1141, 0.0);
            s.store_scalar(1142, 0.0);
            s.store_scalar(1143, 0.0);
            s.store_scalar(1144, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1055] {
            s.store_scalar(1145, 0.0);
            s.store_scalar(1146, 0.0);
            s.store_scalar(1147, 0.0);
            s.store_scalar(1148, 0.0);
            s.store_scalar(1149, 0.0);
            s.store_scalar(1150, 0.0);
            s.store_scalar(1151, 0.0);
            s.store_scalar(1152, 0.0);
            s.store_scalar(1153, 0.0);
            s.store_scalar(1154, 0.0);
            s.store_scalar(1155, 0.0);
            s.store_scalar(1156, 0.0);
            s.store_scalar(1157, 0.0);
            s.store_scalar(1158, 0.0);
            s.store_scalar(1159, 0.0);
            s.store_scalar(1160, 0.0);
            s.store_scalar(1161, 0.0);
            s.store_scalar(1162, 0.0);
            s.store_scalar(1163, 0.0);
            s.store_scalar(1164, 0.0);
            s.store_scalar(1165, 0.0);
            s.store_scalar(1166, 0.0);
            s.store_scalar(1167, 0.0);
        }

        if s.b[1055] {
            s.store_ad_value(1164, {
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(1066), A::tanh_scaled_input(s.ad_value(1066), (0.001 / p.p53)))
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

        if s.b[1055] {
            s.store_sub(1165, 1065, 1066);
            s.store_mul(1099, 1085, 1073);
            s.store_ad_value(1101, A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1081), 1.0, s.ad_value(1073), 2.302585092994046), 1.0, s.ad_value(1084), s.ad_value(1164), 1.0));
            s.store_ad_value(1102, A::add_scaled_product(s.ad_value(1080), 1.0, s.ad_value(1091), A::sub(s.ad_value(1071), s.ad_value(1072)), 1.0));
            s.store_pow_ad(1120, A::div(s.ad_value(1071), s.ad_value(1072)), s.ad_value(1093));
        }

        s.b[1168] = (s.v[1092] != 0.0);
        s.v[1168] = if s.b[1168] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1168]) {
            s.store_div_ad_rhs(1103, 1164, A::pow(A::offset(A::pow(A::div(s.ad_value(1164), s.ad_value(1092)), s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088))));
        }

        if (s.b[1055] && (!s.b[1168])) {
            s.store_scalar(1103, 0.0);
        }

        if s.b[1055] {
            s.store_mul_ad_lhs(1100, A::add_scaled_product(s.ad_value(1082), 1.0, s.ad_value(1103), s.ad_value(1083), (-1.0)), 1164);
            s.store_sub(1063, 1102, 1100);
            s.store_scaled_mul(1105, 1101, 1073, 2.0);
            s.store_mul(1106, 1076, 1105);
            s.store_sub_scaled_inputs(1163, 1063, 1.0, 1099, (p.p51 * 0.5));
        }

        if s.b[1055] {
            let assign17220_ad_e16635: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1065), s.ad_value(1165)), A::sub(s.ad_value(1065), s.ad_value(1165))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1162, A::sub(assign17220_ad_e16635, s.ad_value(1163)), 1099);
        }

        s.b[1169] = (s.v[1162] > 50.0);
        s.v[1169] = if s.b[1169] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1169]) {
            s.store_scalar(1121, 0.0);
        }

        s.b[1170] = (s.v[1162] < (-50.0));
        s.v[1170] = if s.b[1170] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1169])) && s.b[1170]) {
            s.store_scalar(1121, 1.0);
        }

        if ((s.b[1055] && (!s.b[1169])) && (!s.b[1170])) {
            s.store_div_from_scalar_offset_ad(1121, 1.0, A::exp(s.ad_value(1162)), 1.0);
        }

        if s.b[1055] {
            let assign17280_ad_e16723: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1065), s.ad_value(1165)), A::sub(s.ad_value(1065), s.ad_value(1165))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1122, A::sub(assign17280_ad_e16723, A::add_scaled_product(s.ad_value(1063), 1.0, s.ad_value(1099), s.ad_value(1121), (-(p.p51 * 0.1)))), 1105);
        }

        s.b[1171] = (s.v[1122] > 50.0);
        s.v[1171] = if s.b[1171] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1171]) {
            s.store_mul(1123, 1106, 1122);
        }

        s.b[1172] = (s.v[1122] < (-50.0));
        s.v[1172] = if s.b[1172] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1171])) && s.b[1172]) {
            s.store_mul_exp_rhs(1123, 1106, 1122);
        }

        if ((s.b[1055] && (!s.b[1171])) && (!s.b[1172])) {
            s.store_mul_ad_rhs(1123, 1106, A::ln_one_plus_exp(s.ad_value(1122)));
        }

        if s.b[1055] {
            s.store_div_ad_rhs(1109, 1087, A::mul_offset_rhs(s.ad_value(1120), A::div_scaled_product(s.ad_value(1089), s.ad_value(1123), 1.0, s.ad_value(1076), 1.0), 1.0));
            s.store_ad_value(1110, A::div_scaled_product3_by_product(s.ad_value(1086), A::offset(A::mul(s.ad_value(1094), s.ad_value(1072)), 1.0), A::offset(A::div_scaled_product(s.ad_value(1095), s.ad_value(1164), 1.0, s.ad_value(1075), 1.0), 1.0), 1.0, A::offset(A::mul(s.ad_value(1094), s.ad_value(1071)), 1.0), A::offset(A::div_scaled_product(s.ad_value(1090), s.ad_value(1123), 1.0, s.ad_value(1076), 1.0), 1.0), 1.0));
            s.store_add_ad(1111, A::div_scaled_product3(s.ad_value(1121), s.ad_value(1073), s.ad_value(1109), 2.0, s.ad_value(1075), 1.0), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(1121), s.ad_value(1110)));
            s.store_ad_value(1127, A::div_scaled_product(s.ad_value(1110), s.ad_value(1075), 1.0, s.ad_value(1109), 1.0));
            s.store_ad_value(1128, A::add_scaled_product(s.ad_value(1127), (-1.0), s.ad_value(1127), A::sqrt(A::offset(A::div(A::div_scaled_inputs(s.ad_value(1123), 2.0, s.ad_value(1076), 1.0), s.ad_value(1127)), 1.0)), 1.0));
            s.store_ad_value(1129, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(1127), 1.0, s.ad_value(1121)), 1.0, s.ad_value(1105), s.ad_value(1121), 1.0));
            s.store_ad_value(1064, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(1128), 1.0, s.ad_value(1121)), 1.0, s.ad_value(1105), s.ad_value(1121), 1.0));
        }

        if s.b[1055] {
            let assign17410_ad_e16952: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1066), s.ad_value(1064)), 0.5, A::div(s.ad_value(1066), s.ad_value(1064)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1066), s.ad_value(1064))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1066), s.ad_value(1064)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(1066), s.ad_value(1064)), A::div(s.ad_value(1066), s.ad_value(1064)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1130, 1.0, A::offset(A::pow(assign17410_ad_e16952, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul(1131, 1066, 1130);
        }

        if s.b[1055] {
            let assign17430_ad_e17033: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0), A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1132, 1.0, A::offset(A::pow(assign17430_ad_e17033, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul_neg_lhs(1133, 1066, 1132);
            s.store_div_ad_lhs(1162, A::sub(s.ad_value(1065), s.ad_value(1163)), 1099);
        }

        s.b[1173] = (s.v[1162] > 50.0);
        s.v[1173] = if s.b[1173] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1173]) {
            s.store_scalar(1104, 0.0);
        }

        s.b[1174] = (s.v[1162] < (-50.0));
        s.v[1174] = if s.b[1174] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1173])) && s.b[1174]) {
            s.store_scalar(1104, 1.0);
        }

        if ((s.b[1055] && (!s.b[1173])) && (!s.b[1174])) {
            s.store_div_from_scalar_offset_ad(1104, 1.0, A::exp(s.ad_value(1162)), 1.0);
        }

        if s.b[1055] {
            s.store_ad_value(1107, A::div_scaled_inputs3(s.ad_value(1165), 1.0, s.ad_value(1133), (-1.0), A::add_scaled_product(s.ad_value(1063), 1.0, s.ad_value(1099), s.ad_value(1104), (-(p.p51 * 0.1))), -1.0, s.ad_value(1105), 1.0));
        }

        s.b[1175] = (s.v[1107] > 50.0);
        s.v[1175] = if s.b[1175] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1175]) {
            s.store_mul(1108, 1106, 1107);
        }

        s.b[1176] = (s.v[1107] < (-50.0));
        s.v[1176] = if s.b[1176] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1175])) && s.b[1176]) {
            s.store_mul_exp_rhs(1108, 1106, 1107);
        }

        if ((s.b[1055] && (!s.b[1175])) && (!s.b[1176])) {
            s.store_mul_ad_rhs(1108, 1106, A::ln_one_plus_exp(s.ad_value(1107)));
        }

        if s.b[1055] {
            s.store_div_ad_lhs(1162, A::sub(s.ad_value(1165), s.ad_value(1163)), 1099);
        }

        s.b[1177] = (s.v[1162] > 50.0);
        s.v[1177] = if s.b[1177] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1177]) {
            s.store_scalar(1134, 0.0);
        }

        s.b[1178] = (s.v[1162] < (-50.0));
        s.v[1178] = if s.b[1178] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1177])) && s.b[1178]) {
            s.store_scalar(1134, 1.0);
        }

        if ((s.b[1055] && (!s.b[1177])) && (!s.b[1178])) {
            s.store_div_from_scalar_offset_ad(1134, 1.0, A::exp(s.ad_value(1162)), 1.0);
        }

        if s.b[1055] {
            s.store_ad_value(1135, A::div_scaled_inputs3(s.ad_value(1065), 1.0, s.ad_value(1131), (-1.0), A::add_scaled_product(s.ad_value(1063), 1.0, s.ad_value(1099), s.ad_value(1134), (-(p.p51 * 0.1))), -1.0, s.ad_value(1105), 1.0));
        }

        s.b[1179] = (s.v[1135] > 50.0);
        s.v[1179] = if s.b[1179] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1179]) {
            s.store_mul(1136, 1106, 1135);
        }

        s.b[1180] = (s.v[1135] < (-50.0));
        s.v[1180] = if s.b[1180] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1179])) && s.b[1180]) {
            s.store_mul_exp_rhs(1136, 1106, 1135);
        }

        if ((s.b[1055] && (!s.b[1179])) && (!s.b[1180])) {
            s.store_mul_ad_rhs(1136, 1106, A::ln_one_plus_exp(s.ad_value(1135)));
        }

        if s.b[1055] {
            s.store_div_ad_lhs(1137, A::sub(s.ad_value(1108), s.ad_value(1136)), 1076);
            s.store_div(1163, 1137, 1129);
        }

        if s.b[1055] {
            let assign17710_ad_e17310: A = A::pow(A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::mul(s.ad_value(1163), A::tanh_scaled_input(s.ad_value(1163), (0.001 / p.p53)))
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

        if s.b[1055] {
            s.store_mul(1139, 1111, 1138);
            s.store_mul_ad_lhs(1057, A::mul3_scaled_output(A::mul3(s.ad_value(1097), s.ad_value(1074), s.ad_value(1096)), A::add(s.ad_value(1108), s.ad_value(1136)), s.ad_value(1139), 0.5), 1098);
            s.store_scaled_div(1112, 1081, 1073, (1.0 / (2.302585092994046)));
            s.store_scaled_mul(1114, 1112, 1073, 2.0);
            s.store_mul(1115, 1076, 1114);
            s.store_sub_scaled_inputs(1167, 1102, 1.0, 1099, (p.p51 * 0.5));
        }

        if s.b[1055] {
            let assign17780_ad_e17414: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1065), s.ad_value(1165)), A::sub(s.ad_value(1065), s.ad_value(1165))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1166, A::sub(assign17780_ad_e17414, s.ad_value(1167)), 1099);
        }

        s.b[1181] = (s.v[1166] > 50.0);
        s.v[1181] = if s.b[1181] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1181]) {
            s.store_scalar(1124, 0.0);
        }

        s.b[1182] = (s.v[1166] < (-50.0));
        s.v[1182] = if s.b[1182] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1181])) && s.b[1182]) {
            s.store_scalar(1124, 1.0);
        }

        if ((s.b[1055] && (!s.b[1181])) && (!s.b[1182])) {
            s.store_div_from_scalar_offset_ad(1124, 1.0, A::exp(s.ad_value(1166)), 1.0);
        }

        if s.b[1055] {
            let assign17840_ad_e17502: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1065), s.ad_value(1165)), A::sub(s.ad_value(1065), s.ad_value(1165))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1125, A::sub(assign17840_ad_e17502, A::add_scaled_product(s.ad_value(1102), 1.0, s.ad_value(1099), s.ad_value(1124), (-(p.p51 * 0.1)))), 1114);
        }

        s.b[1183] = (s.v[1125] > 50.0);
        s.v[1183] = if s.b[1183] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1183]) {
            s.store_mul(1126, 1115, 1125);
        }

        s.b[1184] = (s.v[1125] < (-50.0));
        s.v[1184] = if s.b[1184] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1183])) && s.b[1184]) {
            s.store_mul_exp_rhs(1126, 1115, 1125);
        }

        if ((s.b[1055] && (!s.b[1183])) && (!s.b[1184])) {
            s.store_mul_ad_rhs(1126, 1115, A::ln_one_plus_exp(s.ad_value(1125)));
        }

        if s.b[1055] {
            s.store_div(1118, 1087, 1120);
        }

    }

    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1055] {
            s.store_mul_div_ad_rhs(1119, 1086, A::offset(A::mul(s.ad_value(1094), s.ad_value(1072)), 1.0), A::offset(A::mul(s.ad_value(1094), s.ad_value(1071)), 1.0));
            s.store_ad_value(1140, A::div_scaled_product(s.ad_value(1119), s.ad_value(1075), 1.0, s.ad_value(1118), 1.0));
            s.store_ad_value(1141, A::add_scaled_product(s.ad_value(1140), (-1.0), s.ad_value(1140), A::sqrt(A::offset(A::div(A::div_scaled_inputs(s.ad_value(1126), 2.0, s.ad_value(1076), 1.0), s.ad_value(1140)), 1.0)), 1.0));
            s.store_ad_value(1142, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(1141), 1.0, s.ad_value(1124)), 1.0, s.ad_value(1114), s.ad_value(1124), 1.0));
        }

        if s.b[1055] {
            let assign17950_ad_e17677: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1066), s.ad_value(1142)), 0.5, A::div(s.ad_value(1066), s.ad_value(1142)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1066), s.ad_value(1142))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1066), s.ad_value(1142)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(1066), s.ad_value(1142)), A::div(s.ad_value(1066), s.ad_value(1142)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1143, 1.0, A::offset(A::pow(assign17950_ad_e17677, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul(1144, 1066, 1143);
        }

        if s.b[1055] {
            let assign17970_ad_e17758: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0), A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(1145, 1.0, A::offset(A::pow(assign17970_ad_e17758, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul_neg_lhs(1146, 1066, 1145);
            s.store_div_ad_lhs(1166, A::sub(s.ad_value(1065), s.ad_value(1167)), 1099);
        }

        s.b[1185] = (s.v[1166] > 50.0);
        s.v[1185] = if s.b[1185] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1185]) {
            s.store_scalar(1113, 0.0);
        }

        s.b[1186] = (s.v[1166] < (-50.0));
        s.v[1186] = if s.b[1186] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1185])) && s.b[1186]) {
            s.store_scalar(1113, 1.0);
        }

        if ((s.b[1055] && (!s.b[1185])) && (!s.b[1186])) {
            s.store_div_from_scalar_offset_ad(1113, 1.0, A::exp(s.ad_value(1166)), 1.0);
        }

        if s.b[1055] {
            s.store_ad_value(1116, A::div_scaled_inputs3(s.ad_value(1165), 1.0, s.ad_value(1146), (-1.0), A::add_scaled_product(s.ad_value(1102), 1.0, s.ad_value(1099), s.ad_value(1113), (-(p.p51 * 0.1))), -1.0, s.ad_value(1114), 1.0));
        }

        s.b[1187] = (s.v[1116] > 50.0);
        s.v[1187] = if s.b[1187] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1187]) {
            s.store_mul(1117, 1115, 1116);
        }

        s.b[1188] = (s.v[1116] < (-50.0));
        s.v[1188] = if s.b[1188] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1187])) && s.b[1188]) {
            s.store_mul_exp_rhs(1117, 1115, 1116);
        }

        if ((s.b[1055] && (!s.b[1187])) && (!s.b[1188])) {
            s.store_mul_ad_rhs(1117, 1115, A::ln_one_plus_exp(s.ad_value(1116)));
        }

        if s.b[1055] {
            s.store_div_ad_lhs(1166, A::sub(s.ad_value(1165), s.ad_value(1167)), 1099);
        }

        s.b[1189] = (s.v[1166] > 50.0);
        s.v[1189] = if s.b[1189] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1189]) {
            s.store_scalar(1147, 0.0);
        }

        s.b[1190] = (s.v[1166] < (-50.0));
        s.v[1190] = if s.b[1190] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1189])) && s.b[1190]) {
            s.store_scalar(1147, 1.0);
        }

        if ((s.b[1055] && (!s.b[1189])) && (!s.b[1190])) {
            s.store_div_from_scalar_offset_ad(1147, 1.0, A::exp(s.ad_value(1166)), 1.0);
        }

        if s.b[1055] {
            s.store_ad_value(1148, A::div_scaled_inputs3(s.ad_value(1065), 1.0, s.ad_value(1144), (-1.0), A::add_scaled_product(s.ad_value(1102), 1.0, s.ad_value(1099), s.ad_value(1147), (-(p.p51 * 0.1))), -1.0, s.ad_value(1114), 1.0));
        }

        s.b[1191] = (s.v[1148] > 50.0);
        s.v[1191] = if s.b[1191] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1191]) {
            s.store_mul(1149, 1115, 1148);
        }

        s.b[1192] = (s.v[1148] < (-50.0));
        s.v[1192] = if s.b[1192] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1191])) && s.b[1192]) {
            s.store_mul_exp_rhs(1149, 1115, 1148);
        }

        if ((s.b[1055] && (!s.b[1191])) && (!s.b[1192])) {
            s.store_mul_ad_rhs(1149, 1115, A::ln_one_plus_exp(s.ad_value(1148)));
        }

        if s.b[1055] {
            s.store_offset_square(1150, 1117, 1e-38);
            s.store_offset_mul(1151, 1150, 1117, 1e-57);
            s.store_offset_square(1152, 1149, 1e-38);
            s.store_offset_mul(1153, 1152, 1149, 1e-57);
            s.store_offset_mul(1154, 1117, 1149, 1e-38);
            s.store_ad_value(1155, A::div_scaled_inputs3(s.ad_value(1150), (2.0 / 3.0), s.ad_value(1152), (2.0 / 3.0), s.ad_value(1154), (2.0 / 3.0), A::offset(A::add(s.ad_value(1117), s.ad_value(1149)), 2e-19), 1.0));
            s.store_div_ad(1156, A::add_scaled_inputs_products(s.ad_value(1151), (2.0 * 2.0), s.ad_value(1153), (3.0 * 2.0), s.ad_value(1150), s.ad_value(1149), (4.0 * 2.0), s.ad_value(1152), s.ad_value(1117), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1150), 15.0, s.ad_value(1152), 15.0, s.ad_value(1154), (2.0 * 15.0)));
            s.store_sub(1157, 1155, 1156);
            s.copy_ad(1158, 1156);
            s.store_mul_ad_lhs(1058, A::mul3(A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1075)), s.ad_value(1097), s.ad_value(1157)), 1098);
            s.store_mul_ad_lhs(1059, A::mul3(A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1075)), s.ad_value(1097), s.ad_value(1158)), 1098);
        }

        s.b[1193] = (s.v[1067] == 1.0);
        s.v[1193] = if s.b[1193] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1193]) {
            s.store_ad_value(1159, A::div_scaled_inputs3(s.ad_value(1068), 1.0, s.ad_value(1102), -1.0, s.ad_value(1099), (-(-(p.p51 * 0.5))), s.ad_value(1114), 1.0));
        }

        s.b[1194] = (s.v[1159] > 50.0);
        s.v[1194] = if s.b[1194] { 1.0 } else { 0.0 };

        if ((s.b[1055] && s.b[1193]) && s.b[1194]) {
            s.copy_ad(1162, 1159);
        }

        s.b[1195] = (s.v[1159] < (-50.0));
        s.v[1195] = if s.b[1195] { 1.0 } else { 0.0 };

        if (((s.b[1055] && s.b[1193]) && (!s.b[1194])) && s.b[1195]) {
            s.store_exp(1162, 1159);
        }

        if (((s.b[1055] && s.b[1193]) && (!s.b[1194])) && (!s.b[1195])) {
            s.store_ln_one_plus_exp(1162, 1159);
        }

        if (s.b[1055] && s.b[1193]) {
            s.store_mul_ad_product_lhs(1060, A::mul3(A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1097)), s.ad_value(1078), s.ad_value(1114)), s.ad_value(1162), 1098);
            s.store_ad_value(1160, A::div_scaled_inputs3(s.ad_value(1069), 1.0, s.ad_value(1102), -1.0, s.ad_value(1099), (-(-(p.p51 * 0.5))), s.ad_value(1114), 1.0));
        }

        s.b[1196] = (s.v[1160] > 50.0);
        s.v[1196] = if s.b[1196] { 1.0 } else { 0.0 };

        if ((s.b[1055] && s.b[1193]) && s.b[1196]) {
            s.copy_ad(1162, 1160);
        }

        s.b[1197] = (s.v[1160] < (-50.0));
        s.v[1197] = if s.b[1197] { 1.0 } else { 0.0 };

        if (((s.b[1055] && s.b[1193]) && (!s.b[1196])) && s.b[1197]) {
            s.store_exp(1162, 1160);
        }

        if (((s.b[1055] && s.b[1193]) && (!s.b[1196])) && (!s.b[1197])) {
            s.store_ln_one_plus_exp(1162, 1160);
        }

        if (s.b[1055] && s.b[1193]) {
            s.store_mul_ad_product_lhs(1061, A::mul3(A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1097)), s.ad_value(1079), s.ad_value(1114)), s.ad_value(1162), 1098);
        }

        if (s.b[1055] && (!s.b[1193])) {
            s.store_scalar(1060, 0.0);
            s.store_scalar(1061, 0.0);
        }

        s.b[1198] = (s.v[1070] == 1.0);
        s.v[1198] = if s.b[1198] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1198]) {
            s.store_ad_value(1161, A::div_scaled_inputs3(s.ad_value(1065), 1.0, s.ad_value(1102), -1.0, s.ad_value(1099), (-(-(p.p51 * 0.5))), s.ad_value(1114), 1.0));
        }

        s.b[1199] = (s.v[1161] > 50.0);
        s.v[1199] = if s.b[1199] { 1.0 } else { 0.0 };

        if ((s.b[1055] && s.b[1198]) && s.b[1199]) {
            s.copy_ad(1162, 1161);
        }

        s.b[1200] = (s.v[1161] < (-50.0));
        s.v[1200] = if s.b[1200] { 1.0 } else { 0.0 };

        if (((s.b[1055] && s.b[1198]) && (!s.b[1199])) && s.b[1200]) {
            s.store_exp(1162, 1161);
        }

        if (((s.b[1055] && s.b[1198]) && (!s.b[1199])) && (!s.b[1200])) {
            s.store_ln_one_plus_exp(1162, 1161);
        }

        if (s.b[1055] && s.b[1198]) {
            s.store_mul_ad_product_lhs(1062, A::mul3(A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1097)), s.ad_value(1077), s.ad_value(1114)), s.ad_value(1162), 1098);
        }

        if (s.b[1055] && (!s.b[1198])) {
            s.store_scalar(1062, 0.0);
        }

        if s.b[1055] {
            s.copy_ad(1056, 1057);
            s.copy_ad(172, 1057);
            s.copy_ad(173, 1058);
            s.copy_ad(174, 1059);
            s.copy_ad(175, 1060);
            s.copy_ad(176, 1061);
            s.copy_ad(177, 1062);
            s.copy_ad(172, 1056);
        }

        s.b[1201] = (p.p100 == 1.0);
        s.v[1201] = if s.b[1201] { 1.0 } else { 0.0 };

        s.v[178] = 0.0;

        s.v[179] = 0.0;

        s.v[180] = 0.0;

        s.v[181] = 0.0;

        s.v[182] = 0.0;

        s.v[183] = 0.0;

        s.b[1202] = (p.p123 > p.p354);
        s.v[1202] = if s.b[1202] { 1.0 } else { 0.0 };

        if s.b[1202] {
            s.store_scalar(1203, 0.0);
            s.store_scalar(1204, 0.0);
            s.store_scalar(1205, 0.0);
            s.store_scalar(1206, 0.0);
            s.store_scalar(1207, 0.0);
            s.store_scalar(1208, 0.0);
            s.store_scalar(1209, 0.0);
            s.store_scalar(1210, 0.0);
            s.store_scalar(1211, 0.0);
            s.copy_ad(1212, 72);
            s.copy_ad(1213, 73);
            s.store_scalar(1214, p.p129);
            s.copy_ad(1215, 74);
            s.copy_ad(1216, 75);
            s.store_scalar(1217, p.p127);
            s.copy_ad(1218, 111);
            s.store_scalar(1219, s.v[109]);
            s.copy_ad(1220, 113);
            s.store_scalar(1221, p.p0);
            s.store_scalar(1222, p.p123);
            s.copy_ad(1223, 26);
            s.store_scalar(1224, p.p128);
            s.copy_ad(1225, 27);
            s.copy_ad(1226, 28);
            s.store_scalar(1227, p.p124);
            s.store_scalar(1228, p.p138);
            s.store_scalar(1229, p.p137);
            s.store_scalar(1230, 0.0);
            s.store_scalar(1231, p.p139);
            s.store_scalar(1232, p.p143);
            s.store_scalar(1233, p.p134);
            s.store_scalar(1234, p.p135);
            s.store_scalar(1235, p.p136);
            s.store_scalar(1236, p.p142);
            s.store_scalar(1237, p.p141);
            s.store_scalar(1238, p.p140);
            s.store_scalar(1239, p.p39);
            s.store_scalar(1240, p.p47);
            s.store_scalar(1241, p.p45);
            s.store_scalar(1242, p.p42);
            s.store_scalar(1243, p.p2);
            s.store_scalar(1244, p.p6);
            s.store_scalar(1245, 1.0);
            s.store_scalar(1246, 0.0);
        }

    }
}
