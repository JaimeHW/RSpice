#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.store_offset_voltage(12, ctx, nodes, Some(3), None, ((ctx_temp) + (p.p45)));

        if ((1026.85 + 273.15) < (if (s.v[12] > ((-100.0) + 273.15)) { s.v[12] } else { ((-100.0) + 273.15) })) {
            s.store_scalar(10, (1026.85 + 273.15));
        } else {
            s.store_ad_value(10, {
                if (s.v[12] > ((-100.0) + 273.15)) {
                    s.ad_value(12)
                } else {
                    A::constant(((-100.0) + 273.15))
                }
            });
        }

        s.v[3] = (p.p43 * p.p42);

        s.store_scaled_voltage(95, ctx, nodes, Some(5), Some(4), p.p29);

        s.store_offset_scaled_ad(94, A::powf(A::neg(A::min_with_scalar(s.ad_value(95), 0.0)), p.p80), p.p79, 1.0);

        s.v[11] = (p.p25 + 273.15);

        s.store_scale(15, 10, 8.6170869e-5);

        s.store_scale(13, 10, 1.0 / (s.v[11]));

        s.store_ln(14, 13);

        s.store_exp_scaled_input(18, 14, p.p77);

        s.store_scaled_mul(16, 18, 94, p.p52);

        s.store_scale(17, 18, p.p60);

        s.v[64] = (if (p.p53 > 0.0) { (1.0 / p.p53) } else { 0.0 });

        s.v[65] = (if (p.p62 > 0.0) { (1.0 / p.p62) } else { 0.0 });

        s.v[66] = (if (p.p54 > 0.0) { (1.0 / p.p54) } else { 0.0 });

        s.v[67] = (if (p.p63 > 0.0) { (1.0 / p.p63) } else { 0.0 });

        s.store_add_scaled_ad_rhs(68, 14, p.p22, A::div_scaled_offset_numerator(s.ad_value(13), p.p21, ((-1.0) * p.p21), s.ad_value(15), 1.0));

        s.store_scale(92, 14, p.p23);

        s.store_scaled_exp(19, 68, p.p0);

        s.store_scaled_exp(93, 92, p.p2);

        s.store_div_scaled_inputs(20, A::exp_scaled_input(s.ad_value(68), 1.0 / (p.p59)), p.p58, s.ad_value(18), 1.0);

        s.store_div_scaled_inputs(21, A::exp_scaled_input(s.ad_value(68), 1.0 / (p.p65)), p.p64, s.ad_value(18), 1.0);

        s.store_offset_scaled(28, 13, ((p.p7) * (p.p47)), (((((((-1.0)) * (p.p7))) + (1.0))) * (p.p47)));

        s.store_offset_scaled(30, 13, ((p.p6) * (p.p5)), (((((((-1.0)) * (p.p6))) + (1.0))) * (p.p5)));

        s.store_offset_scaled(31, 13, ((p.p10) * (p.p9)), (((((((-1.0)) * (p.p10))) + (1.0))) * (p.p9)));

        s.store_offset_scaled(29, 13, ((p.p55) * (p.p56)), (((((((-1.0)) * (p.p55))) + (1.0))) * (p.p56)));

        s.v[32] = p.p16;

        s.v[33] = p.p69;

        s.v[34] = p.p74;

        s.v[81] = (s.v[11] / 300.15);

        s.store_scale(82, 10, 0.003331667499583542);

        s.store_sub_from_scalar_ad(83, 1.16, A::div_scaled_product_offset_denominator(s.ad_value(10), s.ad_value(10), 0.000702, s.ad_value(10), 1108.0, 1.0));

        s.store_offset_ad(84, A::div_scaled_inputs(s.ad_value(83), -1.0, s.ad_value(10), (2.0 * 1.3806226e-23)), (1.1150877 / (1.3806226e-23 * (300.15 + 300.15))));

        s.store_scaled_mul_ad(85, A::scale(s.ad_value(15), 2.0), A::add_scaled_inputs(A::ln(s.ad_value(82)), 1.5, s.ad_value(84), 1.6021918e-19), -1.0);

        s.store_offset_scaled(86, 85, (-1.0 / (s.v[81])), ((p.p17) * (1.0 / (s.v[81]))));

        s.store_div_ad_lhs(87, A::sub_from_scalar(p.p17, s.ad_value(86)), 86);

        s.store_div_from_scalar_offset_scaled_input(89, s.v[32], 87, (-p.p18), (((((0.0004 * (s.v[11] - 300.15))) * (p.p18))) + (1.0)));

        s.store_add_scaled_product(25, s.ad_value(85), 1.0, s.ad_value(82), s.ad_value(86), 1.0);

        s.store_div_scaled_inputs2(88, s.ad_value(25), 1.0, s.ad_value(86), (-1.0), s.ad_value(86), 1.0);

        s.store_mul_offset_ad_rhs(22, 89, A::sub_scaled_inputs(A::scaled_offset(s.ad_value(10), (-300.15), 0.0004), p.p18, s.ad_value(88), p.p18), 1.0);

        s.v[81] = (s.v[11] / 300.15);

        s.store_scale(82, 10, 0.003331667499583542);

        s.store_sub_from_scalar_ad(83, 1.16, A::div_scaled_product_offset_denominator(s.ad_value(10), s.ad_value(10), 0.000702, s.ad_value(10), 1108.0, 1.0));

        s.store_offset_ad(84, A::div_scaled_inputs(s.ad_value(83), -1.0, s.ad_value(10), (2.0 * 1.3806226e-23)), (1.1150877 / (1.3806226e-23 * (300.15 + 300.15))));

        s.store_scaled_mul_ad(85, A::scale(s.ad_value(15), 2.0), A::add_scaled_inputs(A::ln(s.ad_value(82)), 1.5, s.ad_value(84), 1.6021918e-19), -1.0);

        s.store_offset_scaled(86, 85, (-1.0 / (s.v[81])), ((p.p70) * (1.0 / (s.v[81]))));

        s.store_div_ad_lhs(87, A::sub_from_scalar(p.p70, s.ad_value(86)), 86);

        s.store_div_from_scalar_offset_scaled_input(89, s.v[33], 87, (-p.p71), (((((0.0004 * (s.v[11] - 300.15))) * (p.p71))) + (1.0)));

        s.store_add_scaled_product(26, s.ad_value(85), 1.0, s.ad_value(82), s.ad_value(86), 1.0);

        s.store_div_scaled_inputs2(88, s.ad_value(26), 1.0, s.ad_value(86), (-1.0), s.ad_value(86), 1.0);

        s.store_mul_offset_ad_rhs(23, 89, A::sub_scaled_inputs(A::scaled_offset(s.ad_value(10), (-300.15), 0.0004), p.p71, s.ad_value(88), p.p71), 1.0);

        s.v[81] = (s.v[11] / 300.15);

        s.store_scale(82, 10, 0.003331667499583542);

        s.store_sub_from_scalar_ad(83, 1.16, A::div_scaled_product_offset_denominator(s.ad_value(10), s.ad_value(10), 0.000702, s.ad_value(10), 1108.0, 1.0));

        s.store_offset_ad(84, A::div_scaled_inputs(s.ad_value(83), -1.0, s.ad_value(10), (2.0 * 1.3806226e-23)), (1.1150877 / (1.3806226e-23 * (300.15 + 300.15))));

        s.store_scaled_mul_ad(85, A::scale(s.ad_value(15), 2.0), A::add_scaled_inputs(A::ln(s.ad_value(82)), 1.5, s.ad_value(84), 1.6021918e-19), -1.0);

        s.store_offset_scaled(86, 85, (-1.0 / (s.v[81])), ((p.p75) * (1.0 / (s.v[81]))));

        s.store_div_ad_lhs(87, A::sub_from_scalar(p.p75, s.ad_value(86)), 86);

        s.store_div_from_scalar_offset_scaled_input(89, s.v[34], 87, (-p.p76), (((((0.0004 * (s.v[11] - 300.15))) * (p.p76))) + (1.0)));

        s.store_add_scaled_product(27, s.ad_value(85), 1.0, s.ad_value(82), s.ad_value(86), 1.0);

        s.store_div_scaled_inputs2(88, s.ad_value(27), 1.0, s.ad_value(86), (-1.0), s.ad_value(86), 1.0);

        s.store_mul_offset_ad_rhs(24, 89, A::sub_scaled_inputs(A::scaled_offset(s.ad_value(10), (-300.15), 0.0004), p.p76, s.ad_value(88), p.p76), 1.0);

        s.v[9] = p.p29;

        s.store_scaled_voltage(75, ctx, nodes, Some(2), Some(4), s.v[9]);

        s.store_scaled_voltage(76, ctx, nodes, Some(5), Some(6), s.v[9]);

        s.store_scaled_voltage(77, ctx, nodes, Some(5), Some(4), s.v[9]);

        s.store_scaled_voltage(78, ctx, nodes, Some(1), Some(4), s.v[9]);

        s.store_scaled_voltage(79, ctx, nodes, Some(1), Some(5), s.v[9]);

        s.store_scaled_voltage(80, ctx, nodes, Some(2), Some(6), s.v[9]);

        s.b[105] = (s.v[19] > 0.0);
        s.v[105] = if s.b[105] { 1.0 } else { 0.0 };

        if s.b[105] {
            s.store_div_scaled_inputs(0, s.ad_value(76), 1.0, s.ad_value(15), p.p1);
            s.store_div_scaled_inputs2(90, s.ad_value(76), -1.0, s.ad_value(30), (-1.0), s.ad_value(15), p.p11);
            s.store_div_scaled_inputs(91, s.ad_value(30), -1.0, s.ad_value(15), p.p11);
        }

        s.b[106] = (s.v[0] > 80.0);
        s.v[106] = if s.b[106] { 1.0 } else { 0.0 };

        if (s.b[105] && s.b[106]) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
            s.store_scalar(0, 80.0);
        }

        if (s.b[105] && (!s.b[106])) {
            s.store_scalar(1, 1.0);
        }

        if s.b[105] {
            s.store_mul_exp_rhs(1, 1, 0);
        }

        if s.b[105] {
            let assign800_ad_e1032: A = {
                if ((!(s.v[90] >= 37.0)) && (!(s.v[90] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(90))
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
                    A::ln_one_plus_exp(s.ad_value(91))
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

        if s.b[105] {
            s.store_ad_value(35, A::add_scaled_offset_product_rhs(A::div_scaled_product(s.ad_value(28), s.ad_value(2), 1.0, A::scale_offset(A::pow(A::abs(s.ad_value(76)), s.ad_value(31)), p.p8, 1.0), 1.0), (-1.0), s.ad_value(19), s.ad_value(1), (-1.0), 1.0));
        }

        if (!s.b[105]) {
            s.store_scalar(35, 0.0);
        }

        s.b[107] = (s.v[93] > 0.0);
        s.v[107] = if s.b[107] { 1.0 } else { 0.0 };

        if s.b[107] {
            s.store_max_with_scalar_ad(101, A::sub_from_scalar(p.p4, s.ad_value(76)), 0.001);
            s.store_div_scaled_inputs(0, s.ad_value(76), ((-1.0) * p.p4), A::mul_scaled_lhs(s.ad_value(15), p.p3, s.ad_value(101)), 1.0);
        }

        s.b[108] = (s.v[0] > 80.0);
        s.v[108] = if s.b[108] { 1.0 } else { 0.0 };

        if (s.b[107] && s.b[108]) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
            s.store_scalar(0, 80.0);
        }

        if (s.b[107] && (!s.b[108])) {
            s.store_scalar(1, 1.0);
        }

        if s.b[107] {
            s.store_mul_exp_rhs(1, 1, 0);
            s.store_mul_offset_rhs(47, 93, 1, (-1.0));
        }

        if (!s.b[107]) {
            s.store_scalar(47, 0.0);
        }

        s.b[109] = (s.v[20] > 0.0);
        s.v[109] = if s.b[109] { 1.0 } else { 0.0 };

        if s.b[109] {
            s.store_div_scaled_inputs(0, s.ad_value(76), 1.0, s.ad_value(15), p.p59);
            s.store_div_scaled_inputs2(90, s.ad_value(76), -1.0, s.ad_value(30), (-1.0), s.ad_value(15), p.p57);
            s.store_div_scaled_inputs(91, s.ad_value(30), -1.0, s.ad_value(15), p.p57);
        }

        s.b[110] = (s.v[0] > 80.0);
        s.v[110] = if s.b[110] { 1.0 } else { 0.0 };

        if (s.b[109] && s.b[110]) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
            s.store_scalar(0, 80.0);
        }

        if (s.b[109] && (!s.b[110])) {
            s.store_scalar(1, 1.0);
        }

        if s.b[109] {
            s.store_mul_exp_rhs(1, 1, 0);
        }

        if s.b[109] {
            let assign1020_ad_e1266: A = {
                if ((!(s.v[90] >= 37.0)) && (!(s.v[90] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(90))
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
                    A::ln_one_plus_exp(s.ad_value(91))
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

        if s.b[109] {
            s.store_ad_value(36, A::add_scaled_offset_product_rhs(A::div_scaled_inputs(s.ad_value(2), 0.0, A::scale_offset(A::pow(A::abs(s.ad_value(76)), s.ad_value(31)), p.p8, 1.0), 1.0), (-1.0), s.ad_value(20), s.ad_value(1), (-1.0), 1.0));
        }

        if (!s.b[109]) {
            s.store_scalar(36, 0.0);
        }

        s.b[111] = (s.v[19] > 0.0);
        s.v[111] = if s.b[111] { 1.0 } else { 0.0 };

        if s.b[111] {
            s.store_div_scaled_inputs(0, s.ad_value(77), 1.0, s.ad_value(15), p.p61);
            s.store_div_scaled_inputs2(90, s.ad_value(77), -1.0, s.ad_value(30), (-1.0), s.ad_value(15), p.p57);
            s.store_div_scaled_inputs(91, s.ad_value(30), -1.0, s.ad_value(15), p.p57);
        }

        s.b[112] = (s.v[0] > 80.0);
        s.v[112] = if s.b[112] { 1.0 } else { 0.0 };

        if (s.b[111] && s.b[112]) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
            s.store_scalar(0, 80.0);
        }

        if (s.b[111] && (!s.b[112])) {
            s.store_scalar(1, 1.0);
        }

        if s.b[111] {
            s.store_mul_exp_rhs(1, 1, 0);
        }

        if s.b[111] {
            let assign1140_ad_e1428: A = {
                if ((!(s.v[90] >= 37.0)) && (!(s.v[90] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(90))
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
                    A::ln_one_plus_exp(s.ad_value(91))
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

        if s.b[111] {
            s.store_ad_value(38, A::add_scaled_offset_product_rhs(A::div_scaled_product(s.ad_value(29), s.ad_value(2), 1.0, A::scale_offset(A::pow(A::abs(s.ad_value(77)), s.ad_value(31)), p.p8, 1.0), 1.0), (-1.0), s.ad_value(19), s.ad_value(1), (-1.0), 1.0));
        }

        if (!s.b[111]) {
            s.store_scalar(38, 0.0);
        }

        s.b[113] = (s.v[21] > 0.0);
        s.v[113] = if s.b[113] { 1.0 } else { 0.0 };

        if s.b[113] {
            s.store_div_scaled_inputs(0, s.ad_value(77), 1.0, s.ad_value(15), p.p65);
            s.store_div_scaled_inputs2(90, s.ad_value(77), -1.0, s.ad_value(30), (-1.0), s.ad_value(15), p.p57);
            s.store_div_scaled_inputs(91, s.ad_value(30), -1.0, s.ad_value(15), p.p57);
        }

        s.b[114] = (s.v[0] > 80.0);
        s.v[114] = if s.b[114] { 1.0 } else { 0.0 };

        if (s.b[113] && s.b[114]) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
            s.store_scalar(0, 80.0);
        }

        if (s.b[113] && (!s.b[114])) {
            s.store_scalar(1, 1.0);
        }

        if s.b[113] {
            s.store_mul_exp_rhs(1, 1, 0);
        }

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[113] {
            let assign1260_ad_e1590: A = {
                if ((!(s.v[90] >= 37.0)) && (!(s.v[90] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(90))
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
                    A::ln_one_plus_exp(s.ad_value(91))
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

        if s.b[113] {
            s.store_ad_value(39, A::add_scaled_offset_product_rhs(A::div_scaled_inputs(s.ad_value(2), 0.0, A::scale_offset(A::powf(A::abs(s.ad_value(77)), p.p9), p.p8, 1.0), 1.0), (-1.0), s.ad_value(21), s.ad_value(1), (-1.0), 1.0));
        }

        if (!s.b[113]) {
            s.store_scalar(39, 0.0);
        }

        s.store_abs_ad(102, A::div(A::min(A::voltage(ctx, nodes, Some(9), None), s.ad_value(76)), A::max_with_scalar(A::abs(s.ad_value(76)), 1e-9)));

        s.store_add_ad_lhs(37, A::div_scaled_inputs2(s.ad_value(35), 1.0, s.ad_value(47), (-1.0), s.ad_value(16), 1.0), 36);

        s.store_add_ad_lhs(40, A::div(s.ad_value(38), s.ad_value(17)), 39);

        s.store_offset_scaled(66, 77, ((p.p81) * (s.v[66])), s.v[66]);

        s.store_add_scaled_product(42, s.ad_value(38), s.v[67], s.ad_value(35), s.ad_value(66), 1.0);

        s.store_sub_scaled_ad_lhs(41, A::sub_from_scalar(1.0, A::scale(s.ad_value(76), s.v[65])), 77, s.v[64]);

        s.store_offset_powf_ad(96, A::abs(A::scale_offset(s.ad_value(42), 4.0, 1.0)), p.p82, 1.0);

        s.store_div_scaled_inputs(43, s.ad_value(41), 2.0, s.ad_value(96), 1.0);

        s.store_mul(45, 38, 43);

        s.store_mul(44, 35, 43);

        s.store_add_scaled_product(46, A::mul3_scaled_output(s.ad_value(35), s.ad_value(43), s.ad_value(102), p.p84), 1.0, s.ad_value(35), s.ad_value(43), (1.0 - p.p84));

        s.store_offset_powf_ad(99, A::abs_scaled_input(s.ad_value(79), 1.0 / (p.p48)), p.p49, 1.0);

        s.store_offset_powf_ad(100, A::abs_scaled_input(s.ad_value(80), 1.0 / (p.p50)), p.p51, 1.0);

        s.store_scaled_mul_ad(51, A::exp_scaled_input(s.ad_value(14), p.p37), A::powf(s.ad_value(99), (1.0 / p.p49)), p.p12);

        s.store_scaled_exp_scaled_input(52, 14, p.p78, p.p66);

        s.store_scaled_mul_ad(53, A::exp_scaled_input(s.ad_value(14), p.p38), A::powf(s.ad_value(100), (1.0 / p.p51)), p.p14);

        s.store_powf_ad(97, A::abs_scaled_input(A::voltage(ctx, nodes, Some(1), Some(2)), 1.0 / (p.p40)), p.p39);

        s.store_offset_powf_ad(98, A::offset(s.ad_value(97), 1.0), (1.0 / p.p39), (-1.0));

        s.store_offset_scaled(54, 98, ((p.p41) * (p.p19)), p.p19);

        s.store_mul(55, 54, 35);

        s.store_scale(56, 45, p.p73);

        s.b[115] = (p.p32 == 1.0);
        s.v[115] = if s.b[115] { 1.0 } else { 0.0 };

        if s.b[115] {
            s.store_div_scaled_value_offset_denominator(51, s.ad_value(51), 1.0, A::powf(A::scale(A::abs(A::voltage(ctx, nodes, Some(8), None)), 1.0 / (p.p20)), p.p44), 1.0, 1.0);
        }

        if (!s.b[115]) {
        }

        s.b[116] = (p.p31 == 1.0);
        s.v[116] = if s.b[116] { 1.0 } else { 0.0 };

        if s.b[116] {
            s.store_offset(51, 51, p.p13);
            s.store_offset(52, 52, p.p67);
            s.store_offset(53, 53, p.p15);
        }

        s.b[117] = (s.v[75] <= 0.0);
        s.v[117] = if s.b[117] { 1.0 } else { 0.0 };

        if s.b[117] {
            s.store_mul_ad_affine_product_rhs(57, 24, s.ad_value(27), A::sub_from_scalar(1.0, A::exp_scaled_input(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(75), s.ad_value(27)))), (1.0 - p.p76))), 1.0 / ((1.0 - p.p76)), 0.0);
        }

        if (!s.b[117]) {
            s.store_mul_ad_product_rhs(57, 24, s.ad_value(75), A::offset(A::div_scaled_inputs(s.ad_value(75), (0.5 * p.p76), s.ad_value(27), 1.0), 1.0));
        }

        s.store_scale(4, 25, (-p.p24));

        s.store_add(5, 76, 4);

        s.b[118] = (s.v[5] > 0.0);
        s.v[118] = if s.b[118] { 1.0 } else { 0.0 };

        if s.b[118] {
            s.store_scalar(6, (((((-1.0) - p.p18) * (((1.0 - p.p24)) as f64).ln())) as f64).exp());
            s.store_ad_value(7, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(25), 1.0, A::scale(s.ad_value(6), ((1.0 - p.p24) * (1.0 - p.p24))), 1.0 / ((1.0 - p.p18))));
            s.store_mul_ad_product_lhs(8, s.ad_value(5), A::offset(A::div_scaled_inputs(s.ad_value(5), (0.5 * p.p18), s.ad_value(25), 1.0), (1.0 - p.p24)), 6);
        }

        if (!s.b[118]) {
            s.store_ad_value(7, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(25), 1.0, A::exp_scaled_input(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(76), s.ad_value(25)))), (1.0 - p.p18)), 1.0 / ((1.0 - p.p18))));
            s.store_scalar(8, 0.0);
        }

        s.store_mul_add_rhs(58, 22, 7, 8);

        s.store_scale(4, 26, (-p.p24));

        s.store_add(5, 78, 4);

        s.b[119] = (s.v[5] > 0.0);
        s.v[119] = if s.b[119] { 1.0 } else { 0.0 };

        if s.b[119] {
            s.store_scalar(6, (((((-1.0) - p.p71) * (((1.0 - p.p24)) as f64).ln())) as f64).exp());
            s.store_ad_value(7, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(26), 1.0, A::scale(s.ad_value(6), ((1.0 - p.p24) * (1.0 - p.p24))), 1.0 / ((1.0 - p.p71))));
            s.store_mul_ad_product_lhs(8, s.ad_value(5), A::offset(A::div_scaled_inputs(s.ad_value(5), (0.5 * p.p71), s.ad_value(26), 1.0), (1.0 - p.p24)), 6);
        }

        if (!s.b[119]) {
            s.store_ad_value(7, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(26), 1.0, A::exp_scaled_input(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(78), s.ad_value(26)))), (1.0 - p.p71)), 1.0 / ((1.0 - p.p71))));
            s.store_scalar(8, 0.0);
        }

        s.store_mul_add_rhs(59, 23, 7, 8);

        s.store_scale(60, 59, (1.0 - p.p72));

        s.store_scale(4, 26, (-p.p24));

        s.store_add(5, 77, 4);

        s.b[120] = (s.v[5] > 0.0);
        s.v[120] = if s.b[120] { 1.0 } else { 0.0 };

        if s.b[120] {
            s.store_scalar(6, (((((-1.0) - p.p71) * (((1.0 - p.p24)) as f64).ln())) as f64).exp());
            s.store_ad_value(7, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(26), 1.0, A::scale(s.ad_value(6), ((1.0 - p.p24) * (1.0 - p.p24))), 1.0 / ((1.0 - p.p71))));
            s.store_mul_ad_product_lhs(8, s.ad_value(5), A::offset(A::div_scaled_inputs(s.ad_value(5), (0.5 * p.p71), s.ad_value(26), 1.0), (1.0 - p.p24)), 6);
        }

        if (!s.b[120]) {
            s.store_ad_value(7, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(26), 1.0, A::exp_scaled_input(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(26)))), (1.0 - p.p71)), 1.0 / ((1.0 - p.p71))));
            s.store_scalar(8, 0.0);
        }

        s.store_mul_add_rhs(61, 23, 7, 8);

        s.store_scale(62, 61, p.p72);

        s.b[121] = ((p.p68 != 0.0) && (p.p19 != 0.0));
        s.v[121] = if s.b[121] { 1.0 } else { 0.0 };

        if s.b[121] {
            s.store_scale(63, 44, ((((s.v[9] * p.p68) * 3.141592653589793) / 180.0) * p.p19));
        }

        if (!s.b[121]) {
            s.store_scalar(63, 0.0);
        }

        s.b[122] = ((p.p30 == 1.0) && (p.p33 > 0.0));
        s.v[122] = if s.b[122] { 1.0 } else { 0.0 };

        s.b[123] = (((p.p30 == 2.0) && (p.p33 > 0.0)) && (p.p35 > 0.0));
        s.v[123] = if s.b[123] { 1.0 } else { 0.0 };

        s.b[124] = (p.p30 == (-1.0));
        s.v[124] = if s.b[124] { 1.0 } else { 0.0 };

        s.store_scale(69, 10, (4.0 * 1.3806226e-23));

        s.v[50] = ((p.p12 + (p.p31 * p.p13)) / s.v[3]);

        s.v[48] = ((p.p14 + (p.p31 * p.p15)) / s.v[3]);

        s.v[49] = ((p.p66 + (p.p31 * p.p67)) / s.v[3]);

        s.b[125] = ((s.v[50] > 0.0) && (s.v[50] >= p.p46));
        s.v[125] = if s.b[125] { 1.0 } else { 0.0 };

        if s.b[125] {
            s.store_ad_value(72, {
                if ((s.v[51] / s.v[3]) >= p.p46) {
                    A::div_scaled_inputs(s.ad_value(69), 1.0, s.ad_value(51), 1.0 / (s.v[3]))
                } else {
                    A::constant(0.0)
                }
            });
        }

        s.b[126] = ((s.v[48] > 0.0) && (s.v[48] >= p.p46));
        s.v[126] = if s.b[126] { 1.0 } else { 0.0 };

        if s.b[126] {
            s.store_ad_value(73, {
                if ((s.v[53] / s.v[3]) >= p.p46) {
                    A::div_scaled_inputs(s.ad_value(69), 1.0, s.ad_value(53), 1.0 / (s.v[3]))
                } else {
                    A::constant(0.0)
                }
            });
        }

        s.b[127] = ((s.v[49] > 0.0) && (s.v[49] >= p.p46));
        s.v[127] = if s.b[127] { 1.0 } else { 0.0 };

        if s.b[127] {
            s.store_ad_value(74, {
                if ((s.v[52] / s.v[3]) >= p.p46) {
                    A::div_scaled_inputs(s.ad_value(69), 1.0, s.ad_value(52), 1.0 / (s.v[3]))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (if ((p.p28 > 0.0) && (p.p27 > 0.0)) { 1.0 } else { 0.0 } > 0.0) {
            s.store_scaled_powf_ad(71, A::abs(s.ad_value(37)), p.p28, p.p27);
        } else {
            s.store_scalar(71, 0.0);
        }

        s.v[70] = (2.0 * 1.6021918e-19);

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.store_offset_voltage(12, ctx, nodes, Some(3), None, ((ctx_temp) + (p.p45)));

        if ((1026.85 + 273.15) < (if (s.v[12] > ((-100.0) + 273.15)) { s.v[12] } else { ((-100.0) + 273.15) })) {
            s.store_scalar(10, (1026.85 + 273.15));
        } else {
            s.store_ad_value(10, {
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

        s.store_exp_scaled_input(18, 14, p.p77);

        s.v[64] = (if (p.p53 > 0.0) { (1.0 / p.p53) } else { 0.0 });

        s.v[65] = (if (p.p62 > 0.0) { (1.0 / p.p62) } else { 0.0 });

        s.v[66] = (if (p.p54 > 0.0) { (1.0 / p.p54) } else { 0.0 });

        s.v[67] = (if (p.p63 > 0.0) { (1.0 / p.p63) } else { 0.0 });

        s.store_add_scaled_ad_rhs(68, 14, p.p22, A::div_scaled_offset_numerator(s.ad_value(13), p.p21, ((-1.0) * p.p21), s.ad_value(15), 1.0));

        s.store_scale(92, 14, p.p23);

        s.store_scaled_exp(19, 68, p.p0);

        s.store_scaled_exp(93, 92, p.p2);

        s.store_div_scaled_inputs(20, A::exp_scaled_input(s.ad_value(68), 1.0 / (p.p59)), p.p58, s.ad_value(18), 1.0);

        s.store_div_scaled_inputs(21, A::exp_scaled_input(s.ad_value(68), 1.0 / (p.p65)), p.p64, s.ad_value(18), 1.0);

        s.store_offset_scaled(28, 13, ((p.p7) * (p.p47)), (((((((-1.0)) * (p.p7))) + (1.0))) * (p.p47)));

        s.store_offset_scaled(30, 13, ((p.p6) * (p.p5)), (((((((-1.0)) * (p.p6))) + (1.0))) * (p.p5)));

        s.store_offset_scaled(31, 13, ((p.p10) * (p.p9)), (((((((-1.0)) * (p.p10))) + (1.0))) * (p.p9)));

        s.store_offset_scaled(29, 13, ((p.p55) * (p.p56)), (((((((-1.0)) * (p.p55))) + (1.0))) * (p.p56)));

        s.v[32] = p.p16;

        s.v[33] = p.p69;

        s.v[34] = p.p74;

        s.v[81] = (s.v[11] / 300.15);

        s.store_scale(82, 10, 0.003331667499583542);

        s.store_sub_from_scalar_ad(83, 1.16, A::div_scaled_product_offset_denominator(s.ad_value(10), s.ad_value(10), 0.000702, s.ad_value(10), 1108.0, 1.0));

        s.store_offset_ad(84, A::div_scaled_inputs(s.ad_value(83), -1.0, s.ad_value(10), (2.0 * 1.3806226e-23)), (1.1150877 / (1.3806226e-23 * (300.15 + 300.15))));

        s.store_scaled_mul_ad(85, A::scale(s.ad_value(15), 2.0), A::add_scaled_inputs(A::ln(s.ad_value(82)), 1.5, s.ad_value(84), 1.6021918e-19), -1.0);

        s.store_offset_scaled(86, 85, (-1.0 / (s.v[81])), ((p.p17) * (1.0 / (s.v[81]))));

        s.store_div_ad_lhs(87, A::sub_from_scalar(p.p17, s.ad_value(86)), 86);

        s.store_div_from_scalar_offset_scaled_input(89, s.v[32], 87, (-p.p18), (((((0.0004 * (s.v[11] - 300.15))) * (p.p18))) + (1.0)));

        s.store_add_scaled_product(25, s.ad_value(85), 1.0, s.ad_value(82), s.ad_value(86), 1.0);

        s.store_div_scaled_inputs2(88, s.ad_value(25), 1.0, s.ad_value(86), (-1.0), s.ad_value(86), 1.0);

        s.store_mul_offset_ad_rhs(22, 89, A::sub_scaled_inputs(A::scaled_offset(s.ad_value(10), (-300.15), 0.0004), p.p18, s.ad_value(88), p.p18), 1.0);

        s.v[81] = (s.v[11] / 300.15);

        s.store_scale(82, 10, 0.003331667499583542);

        s.store_sub_from_scalar_ad(83, 1.16, A::div_scaled_product_offset_denominator(s.ad_value(10), s.ad_value(10), 0.000702, s.ad_value(10), 1108.0, 1.0));

        s.store_offset_ad(84, A::div_scaled_inputs(s.ad_value(83), -1.0, s.ad_value(10), (2.0 * 1.3806226e-23)), (1.1150877 / (1.3806226e-23 * (300.15 + 300.15))));

        s.store_scaled_mul_ad(85, A::scale(s.ad_value(15), 2.0), A::add_scaled_inputs(A::ln(s.ad_value(82)), 1.5, s.ad_value(84), 1.6021918e-19), -1.0);

        s.store_offset_scaled(86, 85, (-1.0 / (s.v[81])), ((p.p70) * (1.0 / (s.v[81]))));

        s.store_div_ad_lhs(87, A::sub_from_scalar(p.p70, s.ad_value(86)), 86);

        s.store_div_from_scalar_offset_scaled_input(89, s.v[33], 87, (-p.p71), (((((0.0004 * (s.v[11] - 300.15))) * (p.p71))) + (1.0)));

        s.store_add_scaled_product(26, s.ad_value(85), 1.0, s.ad_value(82), s.ad_value(86), 1.0);

        s.store_div_scaled_inputs2(88, s.ad_value(26), 1.0, s.ad_value(86), (-1.0), s.ad_value(86), 1.0);

        s.store_mul_offset_ad_rhs(23, 89, A::sub_scaled_inputs(A::scaled_offset(s.ad_value(10), (-300.15), 0.0004), p.p71, s.ad_value(88), p.p71), 1.0);

        s.v[81] = (s.v[11] / 300.15);

        s.store_scale(82, 10, 0.003331667499583542);

        s.store_sub_from_scalar_ad(83, 1.16, A::div_scaled_product_offset_denominator(s.ad_value(10), s.ad_value(10), 0.000702, s.ad_value(10), 1108.0, 1.0));

        s.store_offset_ad(84, A::div_scaled_inputs(s.ad_value(83), -1.0, s.ad_value(10), (2.0 * 1.3806226e-23)), (1.1150877 / (1.3806226e-23 * (300.15 + 300.15))));

        s.store_scaled_mul_ad(85, A::scale(s.ad_value(15), 2.0), A::add_scaled_inputs(A::ln(s.ad_value(82)), 1.5, s.ad_value(84), 1.6021918e-19), -1.0);

        s.store_offset_scaled(86, 85, (-1.0 / (s.v[81])), ((p.p75) * (1.0 / (s.v[81]))));

        s.store_div_ad_lhs(87, A::sub_from_scalar(p.p75, s.ad_value(86)), 86);

        s.store_div_from_scalar_offset_scaled_input(89, s.v[34], 87, (-p.p76), (((((0.0004 * (s.v[11] - 300.15))) * (p.p76))) + (1.0)));

        s.store_add_scaled_product(27, s.ad_value(85), 1.0, s.ad_value(82), s.ad_value(86), 1.0);

        s.store_div_scaled_inputs2(88, s.ad_value(27), 1.0, s.ad_value(86), (-1.0), s.ad_value(86), 1.0);

        s.store_mul_offset_ad_rhs(24, 89, A::sub_scaled_inputs(A::scaled_offset(s.ad_value(10), (-300.15), 0.0004), p.p76, s.ad_value(88), p.p76), 1.0);

        s.v[9] = p.p29;

        s.store_scaled_voltage(75, ctx, nodes, Some(2), Some(4), s.v[9]);

        s.store_scaled_voltage(76, ctx, nodes, Some(5), Some(6), s.v[9]);

        s.store_scaled_voltage(77, ctx, nodes, Some(5), Some(4), s.v[9]);

        s.store_scaled_voltage(78, ctx, nodes, Some(1), Some(4), s.v[9]);

        s.b[105] = (s.v[19] > 0.0);
        s.v[105] = if s.b[105] { 1.0 } else { 0.0 };

        if s.b[105] {
            s.store_div_scaled_inputs(0, s.ad_value(76), 1.0, s.ad_value(15), p.p1);
            s.store_div_scaled_inputs2(90, s.ad_value(76), -1.0, s.ad_value(30), (-1.0), s.ad_value(15), p.p11);
            s.store_div_scaled_inputs(91, s.ad_value(30), -1.0, s.ad_value(15), p.p11);
        }

        s.b[106] = (s.v[0] > 80.0);
        s.v[106] = if s.b[106] { 1.0 } else { 0.0 };

        if (s.b[105] && s.b[106]) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
            s.store_scalar(0, 80.0);
        }

        if (s.b[105] && (!s.b[106])) {
            s.store_scalar(1, 1.0);
        }

        if s.b[105] {
            s.store_mul_exp_rhs(1, 1, 0);
        }

        if s.b[105] {
            let assign800_ad_e1032: A = {
                if ((!(s.v[90] >= 37.0)) && (!(s.v[90] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(90))
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
                    A::ln_one_plus_exp(s.ad_value(91))
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

        if s.b[105] {
            s.store_ad_value(35, A::add_scaled_offset_product_rhs(A::div_scaled_product(s.ad_value(28), s.ad_value(2), 1.0, A::scale_offset(A::pow(A::abs(s.ad_value(76)), s.ad_value(31)), p.p8, 1.0), 1.0), (-1.0), s.ad_value(19), s.ad_value(1), (-1.0), 1.0));
        }

        if (!s.b[105]) {
            s.store_scalar(35, 0.0);
        }

        s.b[107] = (s.v[93] > 0.0);
        s.v[107] = if s.b[107] { 1.0 } else { 0.0 };

        if s.b[107] {
            s.store_max_with_scalar_ad(101, A::sub_from_scalar(p.p4, s.ad_value(76)), 0.001);
            s.store_div_scaled_inputs(0, s.ad_value(76), ((-1.0) * p.p4), A::mul_scaled_lhs(s.ad_value(15), p.p3, s.ad_value(101)), 1.0);
        }

        s.b[108] = (s.v[0] > 80.0);
        s.v[108] = if s.b[108] { 1.0 } else { 0.0 };

        if (s.b[107] && s.b[108]) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
            s.store_scalar(0, 80.0);
        }

        if (s.b[107] && (!s.b[108])) {
            s.store_scalar(1, 1.0);
        }

        if s.b[107] {
            s.store_mul_exp_rhs(1, 1, 0);
        }

        s.b[109] = (s.v[20] > 0.0);
        s.v[109] = if s.b[109] { 1.0 } else { 0.0 };

        if s.b[109] {
            s.store_div_scaled_inputs(0, s.ad_value(76), 1.0, s.ad_value(15), p.p59);
            s.store_div_scaled_inputs2(90, s.ad_value(76), -1.0, s.ad_value(30), (-1.0), s.ad_value(15), p.p57);
            s.store_div_scaled_inputs(91, s.ad_value(30), -1.0, s.ad_value(15), p.p57);
        }

        s.b[110] = (s.v[0] > 80.0);
        s.v[110] = if s.b[110] { 1.0 } else { 0.0 };

        if (s.b[109] && s.b[110]) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
            s.store_scalar(0, 80.0);
        }

        if (s.b[109] && (!s.b[110])) {
            s.store_scalar(1, 1.0);
        }

        if s.b[109] {
            s.store_mul_exp_rhs(1, 1, 0);
        }

        if s.b[109] {
            let assign1020_ad_e1266: A = {
                if ((!(s.v[90] >= 37.0)) && (!(s.v[90] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(90))
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
                    A::ln_one_plus_exp(s.ad_value(91))
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

        s.b[111] = (s.v[19] > 0.0);
        s.v[111] = if s.b[111] { 1.0 } else { 0.0 };

        if s.b[111] {
            s.store_div_scaled_inputs(0, s.ad_value(77), 1.0, s.ad_value(15), p.p61);
            s.store_div_scaled_inputs2(90, s.ad_value(77), -1.0, s.ad_value(30), (-1.0), s.ad_value(15), p.p57);
            s.store_div_scaled_inputs(91, s.ad_value(30), -1.0, s.ad_value(15), p.p57);
        }

        s.b[112] = (s.v[0] > 80.0);
        s.v[112] = if s.b[112] { 1.0 } else { 0.0 };

        if (s.b[111] && s.b[112]) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
            s.store_scalar(0, 80.0);
        }

        if (s.b[111] && (!s.b[112])) {
            s.store_scalar(1, 1.0);
        }

        if s.b[111] {
            s.store_mul_exp_rhs(1, 1, 0);
        }

        if s.b[111] {
            let assign1140_ad_e1428: A = {
                if ((!(s.v[90] >= 37.0)) && (!(s.v[90] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(90))
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
                    A::ln_one_plus_exp(s.ad_value(91))
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

        if s.b[111] {
            s.store_ad_value(38, A::add_scaled_offset_product_rhs(A::div_scaled_product(s.ad_value(29), s.ad_value(2), 1.0, A::scale_offset(A::pow(A::abs(s.ad_value(77)), s.ad_value(31)), p.p8, 1.0), 1.0), (-1.0), s.ad_value(19), s.ad_value(1), (-1.0), 1.0));
        }

        if (!s.b[111]) {
            s.store_scalar(38, 0.0);
        }

        s.b[113] = (s.v[21] > 0.0);
        s.v[113] = if s.b[113] { 1.0 } else { 0.0 };

        if s.b[113] {
            s.store_div_scaled_inputs(0, s.ad_value(77), 1.0, s.ad_value(15), p.p65);
            s.store_div_scaled_inputs2(90, s.ad_value(77), -1.0, s.ad_value(30), (-1.0), s.ad_value(15), p.p57);
            s.store_div_scaled_inputs(91, s.ad_value(30), -1.0, s.ad_value(15), p.p57);
        }

        s.b[114] = (s.v[0] > 80.0);
        s.v[114] = if s.b[114] { 1.0 } else { 0.0 };

        if (s.b[113] && s.b[114]) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
            s.store_scalar(0, 80.0);
        }

        if (s.b[113] && (!s.b[114])) {
            s.store_scalar(1, 1.0);
        }

        if s.b[113] {
            s.store_mul_exp_rhs(1, 1, 0);
        }

        if s.b[113] {
            let assign1260_ad_e1590: A = {
                if ((!(s.v[90] >= 37.0)) && (!(s.v[90] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(90))
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
                    A::ln_one_plus_exp(s.ad_value(91))
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

        s.store_offset_scaled(66, 77, ((p.p81) * (s.v[66])), s.v[66]);

        s.store_add_scaled_product(42, s.ad_value(38), s.v[67], s.ad_value(35), s.ad_value(66), 1.0);

        s.store_sub_scaled_ad_lhs(41, A::sub_from_scalar(1.0, A::scale(s.ad_value(76), s.v[65])), 77, s.v[64]);

        s.store_offset_powf_ad(96, A::abs(A::scale_offset(s.ad_value(42), 4.0, 1.0)), p.p82, 1.0);

        s.store_div_scaled_inputs(43, s.ad_value(41), 2.0, s.ad_value(96), 1.0);

        s.store_mul(45, 38, 43);

        s.store_mul(44, 35, 43);

        s.store_powf_ad(97, A::abs_scaled_input(A::voltage(ctx, nodes, Some(1), Some(2)), 1.0 / (p.p40)), p.p39);

        s.store_offset_powf_ad(98, A::offset(s.ad_value(97), 1.0), (1.0 / p.p39), (-1.0));

        s.store_offset_scaled(54, 98, ((p.p41) * (p.p19)), p.p19);

        s.store_mul(55, 54, 35);

        s.store_scale(56, 45, p.p73);

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[115] = (p.p32 == 1.0);
        s.v[115] = if s.b[115] { 1.0 } else { 0.0 };

        s.b[117] = (s.v[75] <= 0.0);
        s.v[117] = if s.b[117] { 1.0 } else { 0.0 };

        if s.b[117] {
            s.store_mul_ad_affine_product_rhs(57, 24, s.ad_value(27), A::sub_from_scalar(1.0, A::exp_scaled_input(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(75), s.ad_value(27)))), (1.0 - p.p76))), 1.0 / ((1.0 - p.p76)), 0.0);
        }

        if (!s.b[117]) {
            s.store_mul_ad_product_rhs(57, 24, s.ad_value(75), A::offset(A::div_scaled_inputs(s.ad_value(75), (0.5 * p.p76), s.ad_value(27), 1.0), 1.0));
        }

        s.store_scale(4, 25, (-p.p24));

        s.store_add(5, 76, 4);

        s.b[118] = (s.v[5] > 0.0);
        s.v[118] = if s.b[118] { 1.0 } else { 0.0 };

        if s.b[118] {
            s.store_scalar(6, (((((-1.0) - p.p18) * (((1.0 - p.p24)) as f64).ln())) as f64).exp());
            s.store_ad_value(7, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(25), 1.0, A::scale(s.ad_value(6), ((1.0 - p.p24) * (1.0 - p.p24))), 1.0 / ((1.0 - p.p18))));
            s.store_mul_ad_product_lhs(8, s.ad_value(5), A::offset(A::div_scaled_inputs(s.ad_value(5), (0.5 * p.p18), s.ad_value(25), 1.0), (1.0 - p.p24)), 6);
        }

        if (!s.b[118]) {
            s.store_ad_value(7, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(25), 1.0, A::exp_scaled_input(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(76), s.ad_value(25)))), (1.0 - p.p18)), 1.0 / ((1.0 - p.p18))));
            s.store_scalar(8, 0.0);
        }

        s.store_mul_add_rhs(58, 22, 7, 8);

        s.store_scale(4, 26, (-p.p24));

        s.store_add(5, 78, 4);

        s.b[119] = (s.v[5] > 0.0);
        s.v[119] = if s.b[119] { 1.0 } else { 0.0 };

        if s.b[119] {
            s.store_scalar(6, (((((-1.0) - p.p71) * (((1.0 - p.p24)) as f64).ln())) as f64).exp());
            s.store_ad_value(7, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(26), 1.0, A::scale(s.ad_value(6), ((1.0 - p.p24) * (1.0 - p.p24))), 1.0 / ((1.0 - p.p71))));
            s.store_mul_ad_product_lhs(8, s.ad_value(5), A::offset(A::div_scaled_inputs(s.ad_value(5), (0.5 * p.p71), s.ad_value(26), 1.0), (1.0 - p.p24)), 6);
        }

        if (!s.b[119]) {
            s.store_ad_value(7, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(26), 1.0, A::exp_scaled_input(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(78), s.ad_value(26)))), (1.0 - p.p71)), 1.0 / ((1.0 - p.p71))));
            s.store_scalar(8, 0.0);
        }

        s.store_mul_add_rhs(59, 23, 7, 8);

        s.store_scale(60, 59, (1.0 - p.p72));

        s.store_scale(4, 26, (-p.p24));

        s.store_add(5, 77, 4);

        s.b[120] = (s.v[5] > 0.0);
        s.v[120] = if s.b[120] { 1.0 } else { 0.0 };

        if s.b[120] {
            s.store_scalar(6, (((((-1.0) - p.p71) * (((1.0 - p.p24)) as f64).ln())) as f64).exp());
            s.store_ad_value(7, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(26), 1.0, A::scale(s.ad_value(6), ((1.0 - p.p24) * (1.0 - p.p24))), 1.0 / ((1.0 - p.p71))));
            s.store_mul_ad_product_lhs(8, s.ad_value(5), A::offset(A::div_scaled_inputs(s.ad_value(5), (0.5 * p.p71), s.ad_value(26), 1.0), (1.0 - p.p24)), 6);
        }

        if (!s.b[120]) {
            s.store_ad_value(7, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(26), 1.0, A::exp_scaled_input(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(26)))), (1.0 - p.p71)), 1.0 / ((1.0 - p.p71))));
            s.store_scalar(8, 0.0);
        }

        s.store_mul_add_rhs(61, 23, 7, 8);

        s.store_scale(62, 61, p.p72);

        s.b[121] = ((p.p68 != 0.0) && (p.p19 != 0.0));
        s.v[121] = if s.b[121] { 1.0 } else { 0.0 };

        if s.b[121] {
            s.store_scale(63, 44, ((((s.v[9] * p.p68) * 3.141592653589793) / 180.0) * p.p19));
        }

        if (!s.b[121]) {
            s.store_scalar(63, 0.0);
        }

        s.b[122] = ((p.p30 == 1.0) && (p.p33 > 0.0));
        s.v[122] = if s.b[122] { 1.0 } else { 0.0 };

        s.b[123] = (((p.p30 == 2.0) && (p.p33 > 0.0)) && (p.p35 > 0.0));
        s.v[123] = if s.b[123] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv8 = ctx.node_voltage(nodes[8]);
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
        stamper.stamp_current_dense_local(
            Some(9),
            None,
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let eq1_e95: f64 = ((nv9 - 0.0) * 1e-6);
        let eq1_e95_d_n9: f64 = 1e-6;
        let eq1_value: f64 = eq1_e95;
        stamper.stamp_current_node1_local(
            Some(9),
            None,
            multiplicity * (eq1_value),
            9,
            multiplicity * (eq1_e95_d_n9),
        );
        let eq2_e98: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, (nv9 - 0.0));
        let eq2_e99: f64 = (p.p83 * eq2_e98);
        let eq2_e99_d_n9: f64 = (p.p83 * ddt_scale);
        let eq2_value: f64 = eq2_e99;
        stamper.stamp_current_node1_local(
            Some(9),
            None,
            multiplicity * (eq2_value),
            9,
            multiplicity * (eq2_e99_d_n9),
        );
        let (eq3_e108, eq3_e108_d_n0, eq3_e108_d_n1, eq3_e108_d_n2, eq3_e108_d_n3, eq3_e108_d_n4, eq3_e108_d_n5, eq3_e108_d_n6, eq3_e108_d_n7, eq3_e108_d_n8, eq3_e108_d_n9, eq3_e108_d_b0, eq3_e108_d_b1, eq3_e108_d_b2, eq3_e108_d_b3, eq3_e108_d_b4, eq3_e108_d_b5, eq3_e108_d_b6, eq3_e108_d_b7,) = {
    if s.b[115] {
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
        stamper.stamp_current_dense_local(
            Some(8),
            None,
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e114, eq4_e114_d_n8,) = {
    if s.b[115] {
        let eq4_e112: f64 = (nv8 - 0.0);
        (eq4_e112, 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e114;
        stamper.stamp_current_node1_local(
            Some(8),
            None,
            multiplicity * (eq4_value),
            8,
            multiplicity * (eq4_e114_d_n8),
        );
        let (eq5_e121, eq5_e121_d_n0, eq5_e121_d_n1, eq5_e121_d_n2, eq5_e121_d_n3, eq5_e121_d_n4, eq5_e121_d_n5, eq5_e121_d_n6, eq5_e121_d_n7, eq5_e121_d_n8, eq5_e121_d_n9, eq5_e121_d_b0, eq5_e121_d_b1, eq5_e121_d_b2, eq5_e121_d_b3, eq5_e121_d_b4, eq5_e121_d_b5, eq5_e121_d_b6, eq5_e121_d_b7,) = {
    if s.b[115] {
        let eq5_e118: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, (nv8 - 0.0));
        let eq5_e119: f64 = (s.v[54] * eq5_e118);
        let eq5_e119_d_n0: f64 = (s.dn[54][0] * eq5_e118);
        let eq5_e119_d_n1: f64 = (s.dn[54][1] * eq5_e118);
        let eq5_e119_d_n2: f64 = (s.dn[54][2] * eq5_e118);
        let eq5_e119_d_n3: f64 = (s.dn[54][3] * eq5_e118);
        let eq5_e119_d_n4: f64 = (s.dn[54][4] * eq5_e118);
        let eq5_e119_d_n5: f64 = (s.dn[54][5] * eq5_e118);
        let eq5_e119_d_n6: f64 = (s.dn[54][6] * eq5_e118);
        let eq5_e119_d_n7: f64 = (s.dn[54][7] * eq5_e118);
        let eq5_e119_d_n8: f64 = ((s.dn[54][8] * eq5_e118) + (s.v[54] * ddt_scale));
        let eq5_e119_d_n9: f64 = (s.dn[54][9] * eq5_e118);
        let eq5_e119_d_b0: f64 = (s.db[54][0] * eq5_e118);
        let eq5_e119_d_b1: f64 = (s.db[54][1] * eq5_e118);
        let eq5_e119_d_b2: f64 = (s.db[54][2] * eq5_e118);
        let eq5_e119_d_b3: f64 = (s.db[54][3] * eq5_e118);
        let eq5_e119_d_b4: f64 = (s.db[54][4] * eq5_e118);
        let eq5_e119_d_b5: f64 = (s.db[54][5] * eq5_e118);
        let eq5_e119_d_b6: f64 = (s.db[54][6] * eq5_e118);
        let eq5_e119_d_b7: f64 = (s.db[54][7] * eq5_e118);
        (eq5_e119, eq5_e119_d_n0, eq5_e119_d_n1, eq5_e119_d_n2, eq5_e119_d_n3, eq5_e119_d_n4, eq5_e119_d_n5, eq5_e119_d_n6, eq5_e119_d_n7, eq5_e119_d_n8, eq5_e119_d_n9, eq5_e119_d_b0, eq5_e119_d_b1, eq5_e119_d_b2, eq5_e119_d_b3, eq5_e119_d_b4, eq5_e119_d_b5, eq5_e119_d_b6, eq5_e119_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e121;
        let eq5_node_derivatives: [f64; 10] = [eq5_e121_d_n0, eq5_e121_d_n1, eq5_e121_d_n2, eq5_e121_d_n3, eq5_e121_d_n4, eq5_e121_d_n5, eq5_e121_d_n6, eq5_e121_d_n7, eq5_e121_d_n8, eq5_e121_d_n9];
        let eq5_branch_derivatives: [f64; 8] = [eq5_e121_d_b0, eq5_e121_d_b1, eq5_e121_d_b2, eq5_e121_d_b3, eq5_e121_d_b4, eq5_e121_d_b5, eq5_e121_d_b6, eq5_e121_d_b7];
        stamper.stamp_current_dense_local(
            Some(8),
            None,
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq6_e126,) = {
    if (!s.b[115]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq6_value: f64 = eq6_e126;
        stamper.stamp_potential_const_local(
            0,
            eq6_value,
        );
        let (eq7_e141, eq7_e141_d_n0, eq7_e141_d_n1, eq7_e141_d_n2, eq7_e141_d_n3, eq7_e141_d_n4, eq7_e141_d_n5, eq7_e141_d_n6, eq7_e141_d_n7, eq7_e141_d_n8, eq7_e141_d_n9, eq7_e141_d_b0, eq7_e141_d_b1, eq7_e141_d_b2, eq7_e141_d_b3, eq7_e141_d_b4, eq7_e141_d_b5, eq7_e141_d_b6, eq7_e141_d_b7,) = {
    if s.b[122] {
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
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let (eq8_e147, eq8_e147_d_n3,) = {
    if s.b[122] {
        let eq8_e145: f64 = ((nv3 - 0.0) / p.p33);
        let eq8_e145_d_n3: f64 = (1.0 / p.p33);
        (eq8_e145, eq8_e145_d_n3,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e147;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq8_value),
            3,
            multiplicity * (eq8_e147_d_n3),
        );
        let (eq9_e154, eq9_e154_d_n3,) = {
    if s.b[122] {
        let eq9_e151: f64 = ((nv3 - 0.0) * p.p34);
        let eq9_e151_d_n3: f64 = p.p34;
        let eq9_e152: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, eq9_e151);
        let eq9_e152_d_n3: f64 = (eq9_e151_d_n3 * ddt_scale);
        (eq9_e152, eq9_e152_d_n3,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e154;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq9_value),
            3,
            multiplicity * (eq9_e154_d_n3),
        );
        let (eq10_e158,) = {
    if s.b[122] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq10_value: f64 = eq10_e158;
        stamper.stamp_potential_const_local(
            1,
            eq10_value,
        );
        let (eq11_e176, eq11_e176_d_n0, eq11_e176_d_n1, eq11_e176_d_n2, eq11_e176_d_n3, eq11_e176_d_n4, eq11_e176_d_n5, eq11_e176_d_n6, eq11_e176_d_n7, eq11_e176_d_n8, eq11_e176_d_n9, eq11_e176_d_b0, eq11_e176_d_b1, eq11_e176_d_b2, eq11_e176_d_b3, eq11_e176_d_b4, eq11_e176_d_b5, eq11_e176_d_b6, eq11_e176_d_b7,) = {
    if ((!s.b[122]) && s.b[123]) {
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
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq12_e185, eq12_e185_d_n3, eq12_e185_d_n7,) = {
    if ((!s.b[122]) && s.b[123]) {
        let eq12_e183: f64 = ((nv3 - nv7) / p.p33);
        let eq12_e183_d_n3: f64 = (1.0 / p.p33);
        let eq12_e183_d_n7: f64 = (-1.0 / p.p33);
        (eq12_e183, eq12_e183_d_n3, eq12_e183_d_n7,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e185;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(7),
            multiplicity * (eq12_value),
            3,
            multiplicity * (eq12_e185_d_n3),
            7,
            multiplicity * (eq12_e185_d_n7),
        );
        let (eq13_e195, eq13_e195_d_n3,) = {
    if ((!s.b[122]) && s.b[123]) {
        let eq13_e192: f64 = (p.p34 * (nv3 - 0.0));
        let eq13_e192_d_n3: f64 = p.p34;
        let eq13_e193: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, eq13_e192);
        let eq13_e193_d_n3: f64 = (eq13_e192_d_n3 * ddt_scale);
        (eq13_e193, eq13_e193_d_n3,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e195;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq13_value),
            3,
            multiplicity * (eq13_e195_d_n3),
        );
        let (eq14_e204, eq14_e204_d_n7,) = {
    if ((!s.b[122]) && s.b[123]) {
        let eq14_e202: f64 = ((nv7 - 0.0) / p.p35);
        let eq14_e202_d_n7: f64 = (1.0 / p.p35);
        (eq14_e202, eq14_e202_d_n7,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e204;
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * (eq14_value),
            7,
            multiplicity * (eq14_e204_d_n7),
        );
        let (eq15_e214, eq15_e214_d_n7,) = {
    if ((!s.b[122]) && s.b[123]) {
        let eq15_e211: f64 = (p.p36 * (nv7 - 0.0));
        let eq15_e211_d_n7: f64 = p.p36;
        let eq15_e212: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, eq15_e211);
        let eq15_e212_d_n7: f64 = (eq15_e211_d_n7 * ddt_scale);
        (eq15_e212, eq15_e212_d_n7,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e214;
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * (eq15_value),
            7,
            multiplicity * (eq15_e214_d_n7),
        );
        let (eq16_e235, eq16_e235_d_n0, eq16_e235_d_n1, eq16_e235_d_n2, eq16_e235_d_n3, eq16_e235_d_n4, eq16_e235_d_n5, eq16_e235_d_n6, eq16_e235_d_n7, eq16_e235_d_n8, eq16_e235_d_n9, eq16_e235_d_b0, eq16_e235_d_b1, eq16_e235_d_b2, eq16_e235_d_b3, eq16_e235_d_b4, eq16_e235_d_b5, eq16_e235_d_b6, eq16_e235_d_b7,) = {
    if (((!s.b[122]) && (!s.b[123])) && s.b[124]) {
        let eq16_e223: f64 = (-1.0);
        let eq16_e226: f64 = (s.v[37] * (nv1 - nv2));
        let eq16_e226_d_n0: f64 = (s.dn[37][0] * (nv1 - nv2));
        let eq16_e226_d_n1: f64 = ((s.dn[37][1] * (nv1 - nv2)) + s.v[37]);
        let eq16_e226_d_n2: f64 = ((s.dn[37][2] * (nv1 - nv2)) + (-s.v[37]));
        let eq16_e226_d_n3: f64 = (s.dn[37][3] * (nv1 - nv2));
        let eq16_e226_d_n4: f64 = (s.dn[37][4] * (nv1 - nv2));
        let eq16_e226_d_n5: f64 = (s.dn[37][5] * (nv1 - nv2));
        let eq16_e226_d_n6: f64 = (s.dn[37][6] * (nv1 - nv2));
        let eq16_e226_d_n7: f64 = (s.dn[37][7] * (nv1 - nv2));
        let eq16_e226_d_n8: f64 = (s.dn[37][8] * (nv1 - nv2));
        let eq16_e226_d_n9: f64 = (s.dn[37][9] * (nv1 - nv2));
        let eq16_e226_d_b0: f64 = (s.db[37][0] * (nv1 - nv2));
        let eq16_e226_d_b1: f64 = (s.db[37][1] * (nv1 - nv2));
        let eq16_e226_d_b2: f64 = (s.db[37][2] * (nv1 - nv2));
        let eq16_e226_d_b3: f64 = (s.db[37][3] * (nv1 - nv2));
        let eq16_e226_d_b4: f64 = (s.db[37][4] * (nv1 - nv2));
        let eq16_e226_d_b5: f64 = (s.db[37][5] * (nv1 - nv2));
        let eq16_e226_d_b6: f64 = (s.db[37][6] * (nv1 - nv2));
        let eq16_e226_d_b7: f64 = (s.db[37][7] * (nv1 - nv2));
        let eq16_e227: f64 = (eq16_e226).abs();
        let eq16_e227_d_n0: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n0 } else { (-eq16_e226_d_n0) };
        let eq16_e227_d_n1: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n1 } else { (-eq16_e226_d_n1) };
        let eq16_e227_d_n2: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n2 } else { (-eq16_e226_d_n2) };
        let eq16_e227_d_n3: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n3 } else { (-eq16_e226_d_n3) };
        let eq16_e227_d_n4: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n4 } else { (-eq16_e226_d_n4) };
        let eq16_e227_d_n5: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n5 } else { (-eq16_e226_d_n5) };
        let eq16_e227_d_n6: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n6 } else { (-eq16_e226_d_n6) };
        let eq16_e227_d_n7: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n7 } else { (-eq16_e226_d_n7) };
        let eq16_e227_d_n8: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n8 } else { (-eq16_e226_d_n8) };
        let eq16_e227_d_n9: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_n9 } else { (-eq16_e226_d_n9) };
        let eq16_e227_d_b0: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b0 } else { (-eq16_e226_d_b0) };
        let eq16_e227_d_b1: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b1 } else { (-eq16_e226_d_b1) };
        let eq16_e227_d_b2: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b2 } else { (-eq16_e226_d_b2) };
        let eq16_e227_d_b3: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b3 } else { (-eq16_e226_d_b3) };
        let eq16_e227_d_b4: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b4 } else { (-eq16_e226_d_b4) };
        let eq16_e227_d_b5: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b5 } else { (-eq16_e226_d_b5) };
        let eq16_e227_d_b6: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b6 } else { (-eq16_e226_d_b6) };
        let eq16_e227_d_b7: f64 = if eq16_e226 >= 0.0 { eq16_e226_d_b7 } else { (-eq16_e226_d_b7) };
        let eq16_e228: f64 = (eq16_e223 * eq16_e227);
        let eq16_e228_d_n0: f64 = (eq16_e223 * eq16_e227_d_n0);
        let eq16_e228_d_n1: f64 = (eq16_e223 * eq16_e227_d_n1);
        let eq16_e228_d_n2: f64 = (eq16_e223 * eq16_e227_d_n2);
        let eq16_e228_d_n3: f64 = (eq16_e223 * eq16_e227_d_n3);
        let eq16_e228_d_n4: f64 = (eq16_e223 * eq16_e227_d_n4);
        let eq16_e228_d_n5: f64 = (eq16_e223 * eq16_e227_d_n5);
        let eq16_e228_d_n6: f64 = (eq16_e223 * eq16_e227_d_n6);
        let eq16_e228_d_n7: f64 = (eq16_e223 * eq16_e227_d_n7);
        let eq16_e228_d_n8: f64 = (eq16_e223 * eq16_e227_d_n8);
        let eq16_e228_d_n9: f64 = (eq16_e223 * eq16_e227_d_n9);
        let eq16_e228_d_b0: f64 = (eq16_e223 * eq16_e227_d_b0);
        let eq16_e228_d_b1: f64 = (eq16_e223 * eq16_e227_d_b1);
        let eq16_e228_d_b2: f64 = (eq16_e223 * eq16_e227_d_b2);
        let eq16_e228_d_b3: f64 = (eq16_e223 * eq16_e227_d_b3);
        let eq16_e228_d_b4: f64 = (eq16_e223 * eq16_e227_d_b4);
        let eq16_e228_d_b5: f64 = (eq16_e223 * eq16_e227_d_b5);
        let eq16_e228_d_b6: f64 = (eq16_e223 * eq16_e227_d_b6);
        let eq16_e228_d_b7: f64 = (eq16_e223 * eq16_e227_d_b7);
        let eq16_e231: f64 = (s.v[40] * (nv1 - nv0));
        let eq16_e231_d_n0: f64 = ((s.dn[40][0] * (nv1 - nv0)) + (-s.v[40]));
        let eq16_e231_d_n1: f64 = ((s.dn[40][1] * (nv1 - nv0)) + s.v[40]);
        let eq16_e231_d_n2: f64 = (s.dn[40][2] * (nv1 - nv0));
        let eq16_e231_d_n3: f64 = (s.dn[40][3] * (nv1 - nv0));
        let eq16_e231_d_n4: f64 = (s.dn[40][4] * (nv1 - nv0));
        let eq16_e231_d_n5: f64 = (s.dn[40][5] * (nv1 - nv0));
        let eq16_e231_d_n6: f64 = (s.dn[40][6] * (nv1 - nv0));
        let eq16_e231_d_n7: f64 = (s.dn[40][7] * (nv1 - nv0));
        let eq16_e231_d_n8: f64 = (s.dn[40][8] * (nv1 - nv0));
        let eq16_e231_d_n9: f64 = (s.dn[40][9] * (nv1 - nv0));
        let eq16_e231_d_b0: f64 = (s.db[40][0] * (nv1 - nv0));
        let eq16_e231_d_b1: f64 = (s.db[40][1] * (nv1 - nv0));
        let eq16_e231_d_b2: f64 = (s.db[40][2] * (nv1 - nv0));
        let eq16_e231_d_b3: f64 = (s.db[40][3] * (nv1 - nv0));
        let eq16_e231_d_b4: f64 = (s.db[40][4] * (nv1 - nv0));
        let eq16_e231_d_b5: f64 = (s.db[40][5] * (nv1 - nv0));
        let eq16_e231_d_b6: f64 = (s.db[40][6] * (nv1 - nv0));
        let eq16_e231_d_b7: f64 = (s.db[40][7] * (nv1 - nv0));
        let eq16_e232: f64 = (eq16_e231).abs();
        let eq16_e232_d_n0: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n0 } else { (-eq16_e231_d_n0) };
        let eq16_e232_d_n1: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n1 } else { (-eq16_e231_d_n1) };
        let eq16_e232_d_n2: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n2 } else { (-eq16_e231_d_n2) };
        let eq16_e232_d_n3: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n3 } else { (-eq16_e231_d_n3) };
        let eq16_e232_d_n4: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n4 } else { (-eq16_e231_d_n4) };
        let eq16_e232_d_n5: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n5 } else { (-eq16_e231_d_n5) };
        let eq16_e232_d_n6: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n6 } else { (-eq16_e231_d_n6) };
        let eq16_e232_d_n7: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n7 } else { (-eq16_e231_d_n7) };
        let eq16_e232_d_n8: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n8 } else { (-eq16_e231_d_n8) };
        let eq16_e232_d_n9: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_n9 } else { (-eq16_e231_d_n9) };
        let eq16_e232_d_b0: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b0 } else { (-eq16_e231_d_b0) };
        let eq16_e232_d_b1: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b1 } else { (-eq16_e231_d_b1) };
        let eq16_e232_d_b2: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b2 } else { (-eq16_e231_d_b2) };
        let eq16_e232_d_b3: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b3 } else { (-eq16_e231_d_b3) };
        let eq16_e232_d_b4: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b4 } else { (-eq16_e231_d_b4) };
        let eq16_e232_d_b5: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b5 } else { (-eq16_e231_d_b5) };
        let eq16_e232_d_b6: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b6 } else { (-eq16_e231_d_b6) };
        let eq16_e232_d_b7: f64 = if eq16_e231 >= 0.0 { eq16_e231_d_b7 } else { (-eq16_e231_d_b7) };
        let eq16_e233: f64 = (eq16_e228 - eq16_e232);
        let eq16_e233_d_n0: f64 = (eq16_e228_d_n0 - eq16_e232_d_n0);
        let eq16_e233_d_n1: f64 = (eq16_e228_d_n1 - eq16_e232_d_n1);
        let eq16_e233_d_n2: f64 = (eq16_e228_d_n2 - eq16_e232_d_n2);
        let eq16_e233_d_n3: f64 = (eq16_e228_d_n3 - eq16_e232_d_n3);
        let eq16_e233_d_n4: f64 = (eq16_e228_d_n4 - eq16_e232_d_n4);
        let eq16_e233_d_n5: f64 = (eq16_e228_d_n5 - eq16_e232_d_n5);
        let eq16_e233_d_n6: f64 = (eq16_e228_d_n6 - eq16_e232_d_n6);
        let eq16_e233_d_n7: f64 = (eq16_e228_d_n7 - eq16_e232_d_n7);
        let eq16_e233_d_n8: f64 = (eq16_e228_d_n8 - eq16_e232_d_n8);
        let eq16_e233_d_n9: f64 = (eq16_e228_d_n9 - eq16_e232_d_n9);
        let eq16_e233_d_b0: f64 = (eq16_e228_d_b0 - eq16_e232_d_b0);
        let eq16_e233_d_b1: f64 = (eq16_e228_d_b1 - eq16_e232_d_b1);
        let eq16_e233_d_b2: f64 = (eq16_e228_d_b2 - eq16_e232_d_b2);
        let eq16_e233_d_b3: f64 = (eq16_e228_d_b3 - eq16_e232_d_b3);
        let eq16_e233_d_b4: f64 = (eq16_e228_d_b4 - eq16_e232_d_b4);
        let eq16_e233_d_b5: f64 = (eq16_e228_d_b5 - eq16_e232_d_b5);
        let eq16_e233_d_b6: f64 = (eq16_e228_d_b6 - eq16_e232_d_b6);
        let eq16_e233_d_b7: f64 = (eq16_e228_d_b7 - eq16_e232_d_b7);
        (eq16_e233, eq16_e233_d_n0, eq16_e233_d_n1, eq16_e233_d_n2, eq16_e233_d_n3, eq16_e233_d_n4, eq16_e233_d_n5, eq16_e233_d_n6, eq16_e233_d_n7, eq16_e233_d_n8, eq16_e233_d_n9, eq16_e233_d_b0, eq16_e233_d_b1, eq16_e233_d_b2, eq16_e233_d_b3, eq16_e233_d_b4, eq16_e233_d_b5, eq16_e233_d_b6, eq16_e233_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e235;
        let eq16_node_derivatives: [f64; 10] = [eq16_e235_d_n0, eq16_e235_d_n1, eq16_e235_d_n2, eq16_e235_d_n3, eq16_e235_d_n4, eq16_e235_d_n5, eq16_e235_d_n6, eq16_e235_d_n7, eq16_e235_d_n8, eq16_e235_d_n9];
        let eq16_branch_derivatives: [f64; 8] = [eq16_e235_d_b0, eq16_e235_d_b1, eq16_e235_d_b2, eq16_e235_d_b3, eq16_e235_d_b4, eq16_e235_d_b5, eq16_e235_d_b6, eq16_e235_d_b7];
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let (eq17_e245,) = {
    if (((!s.b[122]) && (!s.b[123])) && s.b[124]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e245;
        stamper.stamp_potential_const_local(
            2,
            eq17_value,
        );
        let (eq18_e256,) = {
    if (((!s.b[122]) && (!s.b[123])) && (!s.b[124])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq18_value: f64 = eq18_e256;
        stamper.stamp_potential_const_local(
            3,
            eq18_value,
        );
        let (eq19_e267,) = {
    if (((!s.b[122]) && (!s.b[123])) && (!s.b[124])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e267;
        stamper.stamp_potential_const_local(
            4,
            eq19_value,
        );
        let eq20_e270: f64 = 0.0;
        let eq20_e272: f64 = (eq20_e270 * (nv5 - nv6));
        let eq20_e272_d_n6: f64 = (-eq20_e270);
        let eq20_value: f64 = eq20_e272;
        stamper.stamp_current_node2_local(
            Some(5),
            Some(6),
            multiplicity * (eq20_value),
            5,
            multiplicity * (eq20_e270),
            6,
            multiplicity * (eq20_e272_d_n6),
        );
        let eq21_e275: f64 = 0.0;
        let eq21_e277: f64 = (eq21_e275 * (nv5 - nv4));
        let eq21_e277_d_n4: f64 = (-eq21_e275);
        let eq21_value: f64 = eq21_e277;
        stamper.stamp_current_node2_local(
            Some(5),
            Some(4),
            multiplicity * (eq21_value),
            4,
            multiplicity * (eq21_e277_d_n4),
            5,
            multiplicity * (eq21_e275),
        );
        let eq22_e280: f64 = 0.0;
        let eq22_e282: f64 = (eq22_e280 * (nv4 - nv6));
        let eq22_e282_d_n6: f64 = (-eq22_e280);
        let eq22_value: f64 = eq22_e282;
        stamper.stamp_current_node2_local(
            Some(4),
            Some(6),
            multiplicity * (eq22_value),
            4,
            multiplicity * (eq22_e280),
            6,
            multiplicity * (eq22_e282_d_n6),
        );
        let (eq23_e297, eq23_e297_d_n0, eq23_e297_d_n1, eq23_e297_d_n2, eq23_e297_d_n3, eq23_e297_d_n4, eq23_e297_d_n5, eq23_e297_d_n6, eq23_e297_d_n7, eq23_e297_d_n8, eq23_e297_d_n9, eq23_e297_d_b0, eq23_e297_d_b1, eq23_e297_d_b2, eq23_e297_d_b3, eq23_e297_d_b4, eq23_e297_d_b5, eq23_e297_d_b6, eq23_e297_d_b7,) = {
    if s.b[125] {
        let eq23_e287: f64 = (s.v[51] / s.v[3]);
        let eq23_e287_d_n0: f64 = (s.dn[51][0] / s.v[3]);
        let eq23_e287_d_n1: f64 = (s.dn[51][1] / s.v[3]);
        let eq23_e287_d_n2: f64 = (s.dn[51][2] / s.v[3]);
        let eq23_e287_d_n3: f64 = (s.dn[51][3] / s.v[3]);
        let eq23_e287_d_n4: f64 = (s.dn[51][4] / s.v[3]);
        let eq23_e287_d_n5: f64 = (s.dn[51][5] / s.v[3]);
        let eq23_e287_d_n6: f64 = (s.dn[51][6] / s.v[3]);
        let eq23_e287_d_n7: f64 = (s.dn[51][7] / s.v[3]);
        let eq23_e287_d_n8: f64 = (s.dn[51][8] / s.v[3]);
        let eq23_e287_d_n9: f64 = (s.dn[51][9] / s.v[3]);
        let eq23_e287_d_b0: f64 = (s.db[51][0] / s.v[3]);
        let eq23_e287_d_b1: f64 = (s.db[51][1] / s.v[3]);
        let eq23_e287_d_b2: f64 = (s.db[51][2] / s.v[3]);
        let eq23_e287_d_b3: f64 = (s.db[51][3] / s.v[3]);
        let eq23_e287_d_b4: f64 = (s.db[51][4] / s.v[3]);
        let eq23_e287_d_b5: f64 = (s.db[51][5] / s.v[3]);
        let eq23_e287_d_b6: f64 = (s.db[51][6] / s.v[3]);
        let eq23_e287_d_b7: f64 = (s.db[51][7] / s.v[3]);
        let (eq23_e294, eq23_e294_d_n0, eq23_e294_d_n1, eq23_e294_d_n2, eq23_e294_d_n3, eq23_e294_d_n4, eq23_e294_d_n5, eq23_e294_d_n6, eq23_e294_d_n7, eq23_e294_d_n8, eq23_e294_d_n9, eq23_e294_d_b0, eq23_e294_d_b1, eq23_e294_d_b2, eq23_e294_d_b3, eq23_e294_d_b4, eq23_e294_d_b5, eq23_e294_d_b6, eq23_e294_d_b7,) = {
            if (eq23_e287 > p.p46) {
                let eq23_e292: f64 = (s.v[51] / s.v[3]);
                let eq23_e292_d_n0: f64 = (s.dn[51][0] / s.v[3]);
                let eq23_e292_d_n1: f64 = (s.dn[51][1] / s.v[3]);
                let eq23_e292_d_n2: f64 = (s.dn[51][2] / s.v[3]);
                let eq23_e292_d_n3: f64 = (s.dn[51][3] / s.v[3]);
                let eq23_e292_d_n4: f64 = (s.dn[51][4] / s.v[3]);
                let eq23_e292_d_n5: f64 = (s.dn[51][5] / s.v[3]);
                let eq23_e292_d_n6: f64 = (s.dn[51][6] / s.v[3]);
                let eq23_e292_d_n7: f64 = (s.dn[51][7] / s.v[3]);
                let eq23_e292_d_n8: f64 = (s.dn[51][8] / s.v[3]);
                let eq23_e292_d_n9: f64 = (s.dn[51][9] / s.v[3]);
                let eq23_e292_d_b0: f64 = (s.db[51][0] / s.v[3]);
                let eq23_e292_d_b1: f64 = (s.db[51][1] / s.v[3]);
                let eq23_e292_d_b2: f64 = (s.db[51][2] / s.v[3]);
                let eq23_e292_d_b3: f64 = (s.db[51][3] / s.v[3]);
                let eq23_e292_d_b4: f64 = (s.db[51][4] / s.v[3]);
                let eq23_e292_d_b5: f64 = (s.db[51][5] / s.v[3]);
                let eq23_e292_d_b6: f64 = (s.db[51][6] / s.v[3]);
                let eq23_e292_d_b7: f64 = (s.db[51][7] / s.v[3]);
                (eq23_e292, eq23_e292_d_n0, eq23_e292_d_n1, eq23_e292_d_n2, eq23_e292_d_n3, eq23_e292_d_n4, eq23_e292_d_n5, eq23_e292_d_n6, eq23_e292_d_n7, eq23_e292_d_n8, eq23_e292_d_n9, eq23_e292_d_b0, eq23_e292_d_b1, eq23_e292_d_b2, eq23_e292_d_b3, eq23_e292_d_b4, eq23_e292_d_b5, eq23_e292_d_b6, eq23_e292_d_b7,)
            } else {
                (p.p46, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let eq23_e295: f64 = ((nv1 - nv5) / eq23_e294);
        let eq23_e295_d_n0: f64 = (-(((nv1 - nv5) * eq23_e294_d_n0) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_n1: f64 = ((eq23_e294 - ((nv1 - nv5) * eq23_e294_d_n1)) / (eq23_e294 * eq23_e294));
        let eq23_e295_d_n2: f64 = (-(((nv1 - nv5) * eq23_e294_d_n2) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_n3: f64 = (-(((nv1 - nv5) * eq23_e294_d_n3) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_n4: f64 = (-(((nv1 - nv5) * eq23_e294_d_n4) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_n5: f64 = (((-eq23_e294) - ((nv1 - nv5) * eq23_e294_d_n5)) / (eq23_e294 * eq23_e294));
        let eq23_e295_d_n6: f64 = (-(((nv1 - nv5) * eq23_e294_d_n6) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_n7: f64 = (-(((nv1 - nv5) * eq23_e294_d_n7) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_n8: f64 = (-(((nv1 - nv5) * eq23_e294_d_n8) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_n9: f64 = (-(((nv1 - nv5) * eq23_e294_d_n9) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b0: f64 = (-(((nv1 - nv5) * eq23_e294_d_b0) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b1: f64 = (-(((nv1 - nv5) * eq23_e294_d_b1) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b2: f64 = (-(((nv1 - nv5) * eq23_e294_d_b2) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b3: f64 = (-(((nv1 - nv5) * eq23_e294_d_b3) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b4: f64 = (-(((nv1 - nv5) * eq23_e294_d_b4) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b5: f64 = (-(((nv1 - nv5) * eq23_e294_d_b5) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b6: f64 = (-(((nv1 - nv5) * eq23_e294_d_b6) / (eq23_e294 * eq23_e294)));
        let eq23_e295_d_b7: f64 = (-(((nv1 - nv5) * eq23_e294_d_b7) / (eq23_e294 * eq23_e294)));
        (eq23_e295, eq23_e295_d_n0, eq23_e295_d_n1, eq23_e295_d_n2, eq23_e295_d_n3, eq23_e295_d_n4, eq23_e295_d_n5, eq23_e295_d_n6, eq23_e295_d_n7, eq23_e295_d_n8, eq23_e295_d_n9, eq23_e295_d_b0, eq23_e295_d_b1, eq23_e295_d_b2, eq23_e295_d_b3, eq23_e295_d_b4, eq23_e295_d_b5, eq23_e295_d_b6, eq23_e295_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e297;
        let eq23_node_derivatives: [f64; 10] = [eq23_e297_d_n0, eq23_e297_d_n1, eq23_e297_d_n2, eq23_e297_d_n3, eq23_e297_d_n4, eq23_e297_d_n5, eq23_e297_d_n6, eq23_e297_d_n7, eq23_e297_d_n8, eq23_e297_d_n9];
        let eq23_branch_derivatives: [f64; 8] = [eq23_e297_d_b0, eq23_e297_d_b1, eq23_e297_d_b2, eq23_e297_d_b3, eq23_e297_d_b4, eq23_e297_d_b5, eq23_e297_d_b6, eq23_e297_d_b7];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(5),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let (eq24_e303,) = {
    if s.b[125] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq24_value: f64 = eq24_e303;
        stamper.stamp_current_const_local(
            Some(1),
            Some(5),
            multiplicity * (eq24_value),
        );
        let (eq25_e308,) = {
    if (!s.b[125]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq25_value: f64 = eq25_e308;
        stamper.stamp_potential_const_local(
            5,
            eq25_value,
        );
        let (eq26_e323, eq26_e323_d_n0, eq26_e323_d_n1, eq26_e323_d_n2, eq26_e323_d_n3, eq26_e323_d_n4, eq26_e323_d_n5, eq26_e323_d_n6, eq26_e323_d_n7, eq26_e323_d_n8, eq26_e323_d_n9, eq26_e323_d_b0, eq26_e323_d_b1, eq26_e323_d_b2, eq26_e323_d_b3, eq26_e323_d_b4, eq26_e323_d_b5, eq26_e323_d_b6, eq26_e323_d_b7,) = {
    if s.b[126] {
        let eq26_e313: f64 = (s.v[53] / s.v[3]);
        let eq26_e313_d_n0: f64 = (s.dn[53][0] / s.v[3]);
        let eq26_e313_d_n1: f64 = (s.dn[53][1] / s.v[3]);
        let eq26_e313_d_n2: f64 = (s.dn[53][2] / s.v[3]);
        let eq26_e313_d_n3: f64 = (s.dn[53][3] / s.v[3]);
        let eq26_e313_d_n4: f64 = (s.dn[53][4] / s.v[3]);
        let eq26_e313_d_n5: f64 = (s.dn[53][5] / s.v[3]);
        let eq26_e313_d_n6: f64 = (s.dn[53][6] / s.v[3]);
        let eq26_e313_d_n7: f64 = (s.dn[53][7] / s.v[3]);
        let eq26_e313_d_n8: f64 = (s.dn[53][8] / s.v[3]);
        let eq26_e313_d_n9: f64 = (s.dn[53][9] / s.v[3]);
        let eq26_e313_d_b0: f64 = (s.db[53][0] / s.v[3]);
        let eq26_e313_d_b1: f64 = (s.db[53][1] / s.v[3]);
        let eq26_e313_d_b2: f64 = (s.db[53][2] / s.v[3]);
        let eq26_e313_d_b3: f64 = (s.db[53][3] / s.v[3]);
        let eq26_e313_d_b4: f64 = (s.db[53][4] / s.v[3]);
        let eq26_e313_d_b5: f64 = (s.db[53][5] / s.v[3]);
        let eq26_e313_d_b6: f64 = (s.db[53][6] / s.v[3]);
        let eq26_e313_d_b7: f64 = (s.db[53][7] / s.v[3]);
        let (eq26_e320, eq26_e320_d_n0, eq26_e320_d_n1, eq26_e320_d_n2, eq26_e320_d_n3, eq26_e320_d_n4, eq26_e320_d_n5, eq26_e320_d_n6, eq26_e320_d_n7, eq26_e320_d_n8, eq26_e320_d_n9, eq26_e320_d_b0, eq26_e320_d_b1, eq26_e320_d_b2, eq26_e320_d_b3, eq26_e320_d_b4, eq26_e320_d_b5, eq26_e320_d_b6, eq26_e320_d_b7,) = {
            if (eq26_e313 > p.p46) {
                let eq26_e318: f64 = (s.v[53] / s.v[3]);
                let eq26_e318_d_n0: f64 = (s.dn[53][0] / s.v[3]);
                let eq26_e318_d_n1: f64 = (s.dn[53][1] / s.v[3]);
                let eq26_e318_d_n2: f64 = (s.dn[53][2] / s.v[3]);
                let eq26_e318_d_n3: f64 = (s.dn[53][3] / s.v[3]);
                let eq26_e318_d_n4: f64 = (s.dn[53][4] / s.v[3]);
                let eq26_e318_d_n5: f64 = (s.dn[53][5] / s.v[3]);
                let eq26_e318_d_n6: f64 = (s.dn[53][6] / s.v[3]);
                let eq26_e318_d_n7: f64 = (s.dn[53][7] / s.v[3]);
                let eq26_e318_d_n8: f64 = (s.dn[53][8] / s.v[3]);
                let eq26_e318_d_n9: f64 = (s.dn[53][9] / s.v[3]);
                let eq26_e318_d_b0: f64 = (s.db[53][0] / s.v[3]);
                let eq26_e318_d_b1: f64 = (s.db[53][1] / s.v[3]);
                let eq26_e318_d_b2: f64 = (s.db[53][2] / s.v[3]);
                let eq26_e318_d_b3: f64 = (s.db[53][3] / s.v[3]);
                let eq26_e318_d_b4: f64 = (s.db[53][4] / s.v[3]);
                let eq26_e318_d_b5: f64 = (s.db[53][5] / s.v[3]);
                let eq26_e318_d_b6: f64 = (s.db[53][6] / s.v[3]);
                let eq26_e318_d_b7: f64 = (s.db[53][7] / s.v[3]);
                (eq26_e318, eq26_e318_d_n0, eq26_e318_d_n1, eq26_e318_d_n2, eq26_e318_d_n3, eq26_e318_d_n4, eq26_e318_d_n5, eq26_e318_d_n6, eq26_e318_d_n7, eq26_e318_d_n8, eq26_e318_d_n9, eq26_e318_d_b0, eq26_e318_d_b1, eq26_e318_d_b2, eq26_e318_d_b3, eq26_e318_d_b4, eq26_e318_d_b5, eq26_e318_d_b6, eq26_e318_d_b7,)
            } else {
                (p.p46, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let eq26_e321: f64 = ((nv2 - nv6) / eq26_e320);
        let eq26_e321_d_n0: f64 = (-(((nv2 - nv6) * eq26_e320_d_n0) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_n1: f64 = (-(((nv2 - nv6) * eq26_e320_d_n1) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_n2: f64 = ((eq26_e320 - ((nv2 - nv6) * eq26_e320_d_n2)) / (eq26_e320 * eq26_e320));
        let eq26_e321_d_n3: f64 = (-(((nv2 - nv6) * eq26_e320_d_n3) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_n4: f64 = (-(((nv2 - nv6) * eq26_e320_d_n4) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_n5: f64 = (-(((nv2 - nv6) * eq26_e320_d_n5) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_n6: f64 = (((-eq26_e320) - ((nv2 - nv6) * eq26_e320_d_n6)) / (eq26_e320 * eq26_e320));
        let eq26_e321_d_n7: f64 = (-(((nv2 - nv6) * eq26_e320_d_n7) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_n8: f64 = (-(((nv2 - nv6) * eq26_e320_d_n8) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_n9: f64 = (-(((nv2 - nv6) * eq26_e320_d_n9) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b0: f64 = (-(((nv2 - nv6) * eq26_e320_d_b0) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b1: f64 = (-(((nv2 - nv6) * eq26_e320_d_b1) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b2: f64 = (-(((nv2 - nv6) * eq26_e320_d_b2) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b3: f64 = (-(((nv2 - nv6) * eq26_e320_d_b3) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b4: f64 = (-(((nv2 - nv6) * eq26_e320_d_b4) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b5: f64 = (-(((nv2 - nv6) * eq26_e320_d_b5) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b6: f64 = (-(((nv2 - nv6) * eq26_e320_d_b6) / (eq26_e320 * eq26_e320)));
        let eq26_e321_d_b7: f64 = (-(((nv2 - nv6) * eq26_e320_d_b7) / (eq26_e320 * eq26_e320)));
        (eq26_e321, eq26_e321_d_n0, eq26_e321_d_n1, eq26_e321_d_n2, eq26_e321_d_n3, eq26_e321_d_n4, eq26_e321_d_n5, eq26_e321_d_n6, eq26_e321_d_n7, eq26_e321_d_n8, eq26_e321_d_n9, eq26_e321_d_b0, eq26_e321_d_b1, eq26_e321_d_b2, eq26_e321_d_b3, eq26_e321_d_b4, eq26_e321_d_b5, eq26_e321_d_b6, eq26_e321_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e323;
        let eq26_node_derivatives: [f64; 10] = [eq26_e323_d_n0, eq26_e323_d_n1, eq26_e323_d_n2, eq26_e323_d_n3, eq26_e323_d_n4, eq26_e323_d_n5, eq26_e323_d_n6, eq26_e323_d_n7, eq26_e323_d_n8, eq26_e323_d_n9];
        let eq26_branch_derivatives: [f64; 8] = [eq26_e323_d_b0, eq26_e323_d_b1, eq26_e323_d_b2, eq26_e323_d_b3, eq26_e323_d_b4, eq26_e323_d_b5, eq26_e323_d_b6, eq26_e323_d_b7];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(6),
            multiplicity * (eq26_value),
            &eq26_node_derivatives,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let (eq27_e329,) = {
    if s.b[126] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq27_value: f64 = eq27_e329;
        stamper.stamp_current_const_local(
            Some(2),
            Some(6),
            multiplicity * (eq27_value),
        );
        let (eq28_e334,) = {
    if (!s.b[126]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq28_value: f64 = eq28_e334;
        stamper.stamp_potential_const_local(
            6,
            eq28_value,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq29_e349, eq29_e349_d_n0, eq29_e349_d_n1, eq29_e349_d_n2, eq29_e349_d_n3, eq29_e349_d_n4, eq29_e349_d_n5, eq29_e349_d_n6, eq29_e349_d_n7, eq29_e349_d_n8, eq29_e349_d_n9, eq29_e349_d_b0, eq29_e349_d_b1, eq29_e349_d_b2, eq29_e349_d_b3, eq29_e349_d_b4, eq29_e349_d_b5, eq29_e349_d_b6, eq29_e349_d_b7,) = {
    if s.b[127] {
        let eq29_e339: f64 = (s.v[52] / s.v[3]);
        let eq29_e339_d_n0: f64 = (s.dn[52][0] / s.v[3]);
        let eq29_e339_d_n1: f64 = (s.dn[52][1] / s.v[3]);
        let eq29_e339_d_n2: f64 = (s.dn[52][2] / s.v[3]);
        let eq29_e339_d_n3: f64 = (s.dn[52][3] / s.v[3]);
        let eq29_e339_d_n4: f64 = (s.dn[52][4] / s.v[3]);
        let eq29_e339_d_n5: f64 = (s.dn[52][5] / s.v[3]);
        let eq29_e339_d_n6: f64 = (s.dn[52][6] / s.v[3]);
        let eq29_e339_d_n7: f64 = (s.dn[52][7] / s.v[3]);
        let eq29_e339_d_n8: f64 = (s.dn[52][8] / s.v[3]);
        let eq29_e339_d_n9: f64 = (s.dn[52][9] / s.v[3]);
        let eq29_e339_d_b0: f64 = (s.db[52][0] / s.v[3]);
        let eq29_e339_d_b1: f64 = (s.db[52][1] / s.v[3]);
        let eq29_e339_d_b2: f64 = (s.db[52][2] / s.v[3]);
        let eq29_e339_d_b3: f64 = (s.db[52][3] / s.v[3]);
        let eq29_e339_d_b4: f64 = (s.db[52][4] / s.v[3]);
        let eq29_e339_d_b5: f64 = (s.db[52][5] / s.v[3]);
        let eq29_e339_d_b6: f64 = (s.db[52][6] / s.v[3]);
        let eq29_e339_d_b7: f64 = (s.db[52][7] / s.v[3]);
        let (eq29_e346, eq29_e346_d_n0, eq29_e346_d_n1, eq29_e346_d_n2, eq29_e346_d_n3, eq29_e346_d_n4, eq29_e346_d_n5, eq29_e346_d_n6, eq29_e346_d_n7, eq29_e346_d_n8, eq29_e346_d_n9, eq29_e346_d_b0, eq29_e346_d_b1, eq29_e346_d_b2, eq29_e346_d_b3, eq29_e346_d_b4, eq29_e346_d_b5, eq29_e346_d_b6, eq29_e346_d_b7,) = {
            if (eq29_e339 > p.p46) {
                let eq29_e344: f64 = (s.v[52] / s.v[3]);
                let eq29_e344_d_n0: f64 = (s.dn[52][0] / s.v[3]);
                let eq29_e344_d_n1: f64 = (s.dn[52][1] / s.v[3]);
                let eq29_e344_d_n2: f64 = (s.dn[52][2] / s.v[3]);
                let eq29_e344_d_n3: f64 = (s.dn[52][3] / s.v[3]);
                let eq29_e344_d_n4: f64 = (s.dn[52][4] / s.v[3]);
                let eq29_e344_d_n5: f64 = (s.dn[52][5] / s.v[3]);
                let eq29_e344_d_n6: f64 = (s.dn[52][6] / s.v[3]);
                let eq29_e344_d_n7: f64 = (s.dn[52][7] / s.v[3]);
                let eq29_e344_d_n8: f64 = (s.dn[52][8] / s.v[3]);
                let eq29_e344_d_n9: f64 = (s.dn[52][9] / s.v[3]);
                let eq29_e344_d_b0: f64 = (s.db[52][0] / s.v[3]);
                let eq29_e344_d_b1: f64 = (s.db[52][1] / s.v[3]);
                let eq29_e344_d_b2: f64 = (s.db[52][2] / s.v[3]);
                let eq29_e344_d_b3: f64 = (s.db[52][3] / s.v[3]);
                let eq29_e344_d_b4: f64 = (s.db[52][4] / s.v[3]);
                let eq29_e344_d_b5: f64 = (s.db[52][5] / s.v[3]);
                let eq29_e344_d_b6: f64 = (s.db[52][6] / s.v[3]);
                let eq29_e344_d_b7: f64 = (s.db[52][7] / s.v[3]);
                (eq29_e344, eq29_e344_d_n0, eq29_e344_d_n1, eq29_e344_d_n2, eq29_e344_d_n3, eq29_e344_d_n4, eq29_e344_d_n5, eq29_e344_d_n6, eq29_e344_d_n7, eq29_e344_d_n8, eq29_e344_d_n9, eq29_e344_d_b0, eq29_e344_d_b1, eq29_e344_d_b2, eq29_e344_d_b3, eq29_e344_d_b4, eq29_e344_d_b5, eq29_e344_d_b6, eq29_e344_d_b7,)
            } else {
                (p.p46, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let eq29_e347: f64 = ((nv0 - nv4) / eq29_e346);
        let eq29_e347_d_n0: f64 = ((eq29_e346 - ((nv0 - nv4) * eq29_e346_d_n0)) / (eq29_e346 * eq29_e346));
        let eq29_e347_d_n1: f64 = (-(((nv0 - nv4) * eq29_e346_d_n1) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_n2: f64 = (-(((nv0 - nv4) * eq29_e346_d_n2) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_n3: f64 = (-(((nv0 - nv4) * eq29_e346_d_n3) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_n4: f64 = (((-eq29_e346) - ((nv0 - nv4) * eq29_e346_d_n4)) / (eq29_e346 * eq29_e346));
        let eq29_e347_d_n5: f64 = (-(((nv0 - nv4) * eq29_e346_d_n5) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_n6: f64 = (-(((nv0 - nv4) * eq29_e346_d_n6) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_n7: f64 = (-(((nv0 - nv4) * eq29_e346_d_n7) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_n8: f64 = (-(((nv0 - nv4) * eq29_e346_d_n8) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_n9: f64 = (-(((nv0 - nv4) * eq29_e346_d_n9) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b0: f64 = (-(((nv0 - nv4) * eq29_e346_d_b0) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b1: f64 = (-(((nv0 - nv4) * eq29_e346_d_b1) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b2: f64 = (-(((nv0 - nv4) * eq29_e346_d_b2) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b3: f64 = (-(((nv0 - nv4) * eq29_e346_d_b3) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b4: f64 = (-(((nv0 - nv4) * eq29_e346_d_b4) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b5: f64 = (-(((nv0 - nv4) * eq29_e346_d_b5) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b6: f64 = (-(((nv0 - nv4) * eq29_e346_d_b6) / (eq29_e346 * eq29_e346)));
        let eq29_e347_d_b7: f64 = (-(((nv0 - nv4) * eq29_e346_d_b7) / (eq29_e346 * eq29_e346)));
        (eq29_e347, eq29_e347_d_n0, eq29_e347_d_n1, eq29_e347_d_n2, eq29_e347_d_n3, eq29_e347_d_n4, eq29_e347_d_n5, eq29_e347_d_n6, eq29_e347_d_n7, eq29_e347_d_n8, eq29_e347_d_n9, eq29_e347_d_b0, eq29_e347_d_b1, eq29_e347_d_b2, eq29_e347_d_b3, eq29_e347_d_b4, eq29_e347_d_b5, eq29_e347_d_b6, eq29_e347_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e349;
        let eq29_node_derivatives: [f64; 10] = [eq29_e349_d_n0, eq29_e349_d_n1, eq29_e349_d_n2, eq29_e349_d_n3, eq29_e349_d_n4, eq29_e349_d_n5, eq29_e349_d_n6, eq29_e349_d_n7, eq29_e349_d_n8, eq29_e349_d_n9];
        let eq29_branch_derivatives: [f64; 8] = [eq29_e349_d_b0, eq29_e349_d_b1, eq29_e349_d_b2, eq29_e349_d_b3, eq29_e349_d_b4, eq29_e349_d_b5, eq29_e349_d_b6, eq29_e349_d_b7];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(4),
            multiplicity * (eq29_value),
            &eq29_node_derivatives,
            &eq29_branch_derivatives,
            multiplicity,
        );
        let (eq30_e355,) = {
    if s.b[127] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq30_value: f64 = eq30_e355;
        stamper.stamp_current_const_local(
            Some(0),
            Some(4),
            multiplicity * (eq30_value),
        );
        let (eq31_e360,) = {
    if (!s.b[127]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq31_value: f64 = eq31_e360;
        stamper.stamp_potential_const_local(
            7,
            eq31_value,
        );
        let eq32_e363: f64 = (s.v[9] * s.v[37]);
        let eq32_e363_d_n0: f64 = (s.v[9] * s.dn[37][0]);
        let eq32_e363_d_n1: f64 = (s.v[9] * s.dn[37][1]);
        let eq32_e363_d_n2: f64 = (s.v[9] * s.dn[37][2]);
        let eq32_e363_d_n3: f64 = (s.v[9] * s.dn[37][3]);
        let eq32_e363_d_n4: f64 = (s.v[9] * s.dn[37][4]);
        let eq32_e363_d_n5: f64 = (s.v[9] * s.dn[37][5]);
        let eq32_e363_d_n6: f64 = (s.v[9] * s.dn[37][6]);
        let eq32_e363_d_n7: f64 = (s.v[9] * s.dn[37][7]);
        let eq32_e363_d_n8: f64 = (s.v[9] * s.dn[37][8]);
        let eq32_e363_d_n9: f64 = (s.v[9] * s.dn[37][9]);
        let eq32_e363_d_b0: f64 = (s.v[9] * s.db[37][0]);
        let eq32_e363_d_b1: f64 = (s.v[9] * s.db[37][1]);
        let eq32_e363_d_b2: f64 = (s.v[9] * s.db[37][2]);
        let eq32_e363_d_b3: f64 = (s.v[9] * s.db[37][3]);
        let eq32_e363_d_b4: f64 = (s.v[9] * s.db[37][4]);
        let eq32_e363_d_b5: f64 = (s.v[9] * s.db[37][5]);
        let eq32_e363_d_b6: f64 = (s.v[9] * s.db[37][6]);
        let eq32_e363_d_b7: f64 = (s.v[9] * s.db[37][7]);
        let eq32_e365: f64 = (eq32_e363 * s.v[3]);
        let eq32_e365_d_n0: f64 = (eq32_e363_d_n0 * s.v[3]);
        let eq32_e365_d_n1: f64 = (eq32_e363_d_n1 * s.v[3]);
        let eq32_e365_d_n2: f64 = (eq32_e363_d_n2 * s.v[3]);
        let eq32_e365_d_n3: f64 = (eq32_e363_d_n3 * s.v[3]);
        let eq32_e365_d_n4: f64 = (eq32_e363_d_n4 * s.v[3]);
        let eq32_e365_d_n5: f64 = (eq32_e363_d_n5 * s.v[3]);
        let eq32_e365_d_n6: f64 = (eq32_e363_d_n6 * s.v[3]);
        let eq32_e365_d_n7: f64 = (eq32_e363_d_n7 * s.v[3]);
        let eq32_e365_d_n8: f64 = (eq32_e363_d_n8 * s.v[3]);
        let eq32_e365_d_n9: f64 = (eq32_e363_d_n9 * s.v[3]);
        let eq32_e365_d_b0: f64 = (eq32_e363_d_b0 * s.v[3]);
        let eq32_e365_d_b1: f64 = (eq32_e363_d_b1 * s.v[3]);
        let eq32_e365_d_b2: f64 = (eq32_e363_d_b2 * s.v[3]);
        let eq32_e365_d_b3: f64 = (eq32_e363_d_b3 * s.v[3]);
        let eq32_e365_d_b4: f64 = (eq32_e363_d_b4 * s.v[3]);
        let eq32_e365_d_b5: f64 = (eq32_e363_d_b5 * s.v[3]);
        let eq32_e365_d_b6: f64 = (eq32_e363_d_b6 * s.v[3]);
        let eq32_e365_d_b7: f64 = (eq32_e363_d_b7 * s.v[3]);
        let eq32_value: f64 = eq32_e365;
        let eq32_node_derivatives: [f64; 10] = [eq32_e365_d_n0, eq32_e365_d_n1, eq32_e365_d_n2, eq32_e365_d_n3, eq32_e365_d_n4, eq32_e365_d_n5, eq32_e365_d_n6, eq32_e365_d_n7, eq32_e365_d_n8, eq32_e365_d_n9];
        let eq32_branch_derivatives: [f64; 8] = [eq32_e365_d_b0, eq32_e365_d_b1, eq32_e365_d_b2, eq32_e365_d_b3, eq32_e365_d_b4, eq32_e365_d_b5, eq32_e365_d_b6, eq32_e365_d_b7];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let eq33_e368: f64 = (s.v[9] * s.v[40]);
        let eq33_e368_d_n0: f64 = (s.v[9] * s.dn[40][0]);
        let eq33_e368_d_n1: f64 = (s.v[9] * s.dn[40][1]);
        let eq33_e368_d_n2: f64 = (s.v[9] * s.dn[40][2]);
        let eq33_e368_d_n3: f64 = (s.v[9] * s.dn[40][3]);
        let eq33_e368_d_n4: f64 = (s.v[9] * s.dn[40][4]);
        let eq33_e368_d_n5: f64 = (s.v[9] * s.dn[40][5]);
        let eq33_e368_d_n6: f64 = (s.v[9] * s.dn[40][6]);
        let eq33_e368_d_n7: f64 = (s.v[9] * s.dn[40][7]);
        let eq33_e368_d_n8: f64 = (s.v[9] * s.dn[40][8]);
        let eq33_e368_d_n9: f64 = (s.v[9] * s.dn[40][9]);
        let eq33_e368_d_b0: f64 = (s.v[9] * s.db[40][0]);
        let eq33_e368_d_b1: f64 = (s.v[9] * s.db[40][1]);
        let eq33_e368_d_b2: f64 = (s.v[9] * s.db[40][2]);
        let eq33_e368_d_b3: f64 = (s.v[9] * s.db[40][3]);
        let eq33_e368_d_b4: f64 = (s.v[9] * s.db[40][4]);
        let eq33_e368_d_b5: f64 = (s.v[9] * s.db[40][5]);
        let eq33_e368_d_b6: f64 = (s.v[9] * s.db[40][6]);
        let eq33_e368_d_b7: f64 = (s.v[9] * s.db[40][7]);
        let eq33_e370: f64 = (eq33_e368 * s.v[3]);
        let eq33_e370_d_n0: f64 = (eq33_e368_d_n0 * s.v[3]);
        let eq33_e370_d_n1: f64 = (eq33_e368_d_n1 * s.v[3]);
        let eq33_e370_d_n2: f64 = (eq33_e368_d_n2 * s.v[3]);
        let eq33_e370_d_n3: f64 = (eq33_e368_d_n3 * s.v[3]);
        let eq33_e370_d_n4: f64 = (eq33_e368_d_n4 * s.v[3]);
        let eq33_e370_d_n5: f64 = (eq33_e368_d_n5 * s.v[3]);
        let eq33_e370_d_n6: f64 = (eq33_e368_d_n6 * s.v[3]);
        let eq33_e370_d_n7: f64 = (eq33_e368_d_n7 * s.v[3]);
        let eq33_e370_d_n8: f64 = (eq33_e368_d_n8 * s.v[3]);
        let eq33_e370_d_n9: f64 = (eq33_e368_d_n9 * s.v[3]);
        let eq33_e370_d_b0: f64 = (eq33_e368_d_b0 * s.v[3]);
        let eq33_e370_d_b1: f64 = (eq33_e368_d_b1 * s.v[3]);
        let eq33_e370_d_b2: f64 = (eq33_e368_d_b2 * s.v[3]);
        let eq33_e370_d_b3: f64 = (eq33_e368_d_b3 * s.v[3]);
        let eq33_e370_d_b4: f64 = (eq33_e368_d_b4 * s.v[3]);
        let eq33_e370_d_b5: f64 = (eq33_e368_d_b5 * s.v[3]);
        let eq33_e370_d_b6: f64 = (eq33_e368_d_b6 * s.v[3]);
        let eq33_e370_d_b7: f64 = (eq33_e368_d_b7 * s.v[3]);
        let eq33_value: f64 = eq33_e370;
        let eq33_node_derivatives: [f64; 10] = [eq33_e370_d_n0, eq33_e370_d_n1, eq33_e370_d_n2, eq33_e370_d_n3, eq33_e370_d_n4, eq33_e370_d_n5, eq33_e370_d_n6, eq33_e370_d_n7, eq33_e370_d_n8, eq33_e370_d_n9];
        let eq33_branch_derivatives: [f64; 8] = [eq33_e370_d_b0, eq33_e370_d_b1, eq33_e370_d_b2, eq33_e370_d_b3, eq33_e370_d_b4, eq33_e370_d_b5, eq33_e370_d_b6, eq33_e370_d_b7];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(4),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let eq34_e373: f64 = (-s.v[45]);
        let eq34_e373_d_n0: f64 = (-s.dn[45][0]);
        let eq34_e373_d_n1: f64 = (-s.dn[45][1]);
        let eq34_e373_d_n2: f64 = (-s.dn[45][2]);
        let eq34_e373_d_n3: f64 = (-s.dn[45][3]);
        let eq34_e373_d_n4: f64 = (-s.dn[45][4]);
        let eq34_e373_d_n5: f64 = (-s.dn[45][5]);
        let eq34_e373_d_n6: f64 = (-s.dn[45][6]);
        let eq34_e373_d_n7: f64 = (-s.dn[45][7]);
        let eq34_e373_d_n8: f64 = (-s.dn[45][8]);
        let eq34_e373_d_n9: f64 = (-s.dn[45][9]);
        let eq34_e373_d_b0: f64 = (-s.db[45][0]);
        let eq34_e373_d_b1: f64 = (-s.db[45][1]);
        let eq34_e373_d_b2: f64 = (-s.db[45][2]);
        let eq34_e373_d_b3: f64 = (-s.db[45][3]);
        let eq34_e373_d_b4: f64 = (-s.db[45][4]);
        let eq34_e373_d_b5: f64 = (-s.db[45][5]);
        let eq34_e373_d_b6: f64 = (-s.db[45][6]);
        let eq34_e373_d_b7: f64 = (-s.db[45][7]);
        let eq34_e375: f64 = (eq34_e373 * s.v[3]);
        let eq34_e375_d_n0: f64 = (eq34_e373_d_n0 * s.v[3]);
        let eq34_e375_d_n1: f64 = (eq34_e373_d_n1 * s.v[3]);
        let eq34_e375_d_n2: f64 = (eq34_e373_d_n2 * s.v[3]);
        let eq34_e375_d_n3: f64 = (eq34_e373_d_n3 * s.v[3]);
        let eq34_e375_d_n4: f64 = (eq34_e373_d_n4 * s.v[3]);
        let eq34_e375_d_n5: f64 = (eq34_e373_d_n5 * s.v[3]);
        let eq34_e375_d_n6: f64 = (eq34_e373_d_n6 * s.v[3]);
        let eq34_e375_d_n7: f64 = (eq34_e373_d_n7 * s.v[3]);
        let eq34_e375_d_n8: f64 = (eq34_e373_d_n8 * s.v[3]);
        let eq34_e375_d_n9: f64 = (eq34_e373_d_n9 * s.v[3]);
        let eq34_e375_d_b0: f64 = (eq34_e373_d_b0 * s.v[3]);
        let eq34_e375_d_b1: f64 = (eq34_e373_d_b1 * s.v[3]);
        let eq34_e375_d_b2: f64 = (eq34_e373_d_b2 * s.v[3]);
        let eq34_e375_d_b3: f64 = (eq34_e373_d_b3 * s.v[3]);
        let eq34_e375_d_b4: f64 = (eq34_e373_d_b4 * s.v[3]);
        let eq34_e375_d_b5: f64 = (eq34_e373_d_b5 * s.v[3]);
        let eq34_e375_d_b6: f64 = (eq34_e373_d_b6 * s.v[3]);
        let eq34_e375_d_b7: f64 = (eq34_e373_d_b7 * s.v[3]);
        let eq34_e376: f64 = (s.v[9] * eq34_e375);
        let eq34_e376_d_n0: f64 = (s.v[9] * eq34_e375_d_n0);
        let eq34_e376_d_n1: f64 = (s.v[9] * eq34_e375_d_n1);
        let eq34_e376_d_n2: f64 = (s.v[9] * eq34_e375_d_n2);
        let eq34_e376_d_n3: f64 = (s.v[9] * eq34_e375_d_n3);
        let eq34_e376_d_n4: f64 = (s.v[9] * eq34_e375_d_n4);
        let eq34_e376_d_n5: f64 = (s.v[9] * eq34_e375_d_n5);
        let eq34_e376_d_n6: f64 = (s.v[9] * eq34_e375_d_n6);
        let eq34_e376_d_n7: f64 = (s.v[9] * eq34_e375_d_n7);
        let eq34_e376_d_n8: f64 = (s.v[9] * eq34_e375_d_n8);
        let eq34_e376_d_n9: f64 = (s.v[9] * eq34_e375_d_n9);
        let eq34_e376_d_b0: f64 = (s.v[9] * eq34_e375_d_b0);
        let eq34_e376_d_b1: f64 = (s.v[9] * eq34_e375_d_b1);
        let eq34_e376_d_b2: f64 = (s.v[9] * eq34_e375_d_b2);
        let eq34_e376_d_b3: f64 = (s.v[9] * eq34_e375_d_b3);
        let eq34_e376_d_b4: f64 = (s.v[9] * eq34_e375_d_b4);
        let eq34_e376_d_b5: f64 = (s.v[9] * eq34_e375_d_b5);
        let eq34_e376_d_b6: f64 = (s.v[9] * eq34_e375_d_b6);
        let eq34_e376_d_b7: f64 = (s.v[9] * eq34_e375_d_b7);
        let eq34_value: f64 = eq34_e376;
        let eq34_node_derivatives: [f64; 10] = [eq34_e376_d_n0, eq34_e376_d_n1, eq34_e376_d_n2, eq34_e376_d_n3, eq34_e376_d_n4, eq34_e376_d_n5, eq34_e376_d_n6, eq34_e376_d_n7, eq34_e376_d_n8, eq34_e376_d_n9];
        let eq34_branch_derivatives: [f64; 8] = [eq34_e376_d_b0, eq34_e376_d_b1, eq34_e376_d_b2, eq34_e376_d_b3, eq34_e376_d_b4, eq34_e376_d_b5, eq34_e376_d_b6, eq34_e376_d_b7];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(6),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let eq35_e379: f64 = (s.v[9] * s.v[46]);
        let eq35_e379_d_n0: f64 = (s.v[9] * s.dn[46][0]);
        let eq35_e379_d_n1: f64 = (s.v[9] * s.dn[46][1]);
        let eq35_e379_d_n2: f64 = (s.v[9] * s.dn[46][2]);
        let eq35_e379_d_n3: f64 = (s.v[9] * s.dn[46][3]);
        let eq35_e379_d_n4: f64 = (s.v[9] * s.dn[46][4]);
        let eq35_e379_d_n5: f64 = (s.v[9] * s.dn[46][5]);
        let eq35_e379_d_n6: f64 = (s.v[9] * s.dn[46][6]);
        let eq35_e379_d_n7: f64 = (s.v[9] * s.dn[46][7]);
        let eq35_e379_d_n8: f64 = (s.v[9] * s.dn[46][8]);
        let eq35_e379_d_n9: f64 = (s.v[9] * s.dn[46][9]);
        let eq35_e379_d_b0: f64 = (s.v[9] * s.db[46][0]);
        let eq35_e379_d_b1: f64 = (s.v[9] * s.db[46][1]);
        let eq35_e379_d_b2: f64 = (s.v[9] * s.db[46][2]);
        let eq35_e379_d_b3: f64 = (s.v[9] * s.db[46][3]);
        let eq35_e379_d_b4: f64 = (s.v[9] * s.db[46][4]);
        let eq35_e379_d_b5: f64 = (s.v[9] * s.db[46][5]);
        let eq35_e379_d_b6: f64 = (s.v[9] * s.db[46][6]);
        let eq35_e379_d_b7: f64 = (s.v[9] * s.db[46][7]);
        let eq35_e381: f64 = (eq35_e379 * s.v[3]);
        let eq35_e381_d_n0: f64 = (eq35_e379_d_n0 * s.v[3]);
        let eq35_e381_d_n1: f64 = (eq35_e379_d_n1 * s.v[3]);
        let eq35_e381_d_n2: f64 = (eq35_e379_d_n2 * s.v[3]);
        let eq35_e381_d_n3: f64 = (eq35_e379_d_n3 * s.v[3]);
        let eq35_e381_d_n4: f64 = (eq35_e379_d_n4 * s.v[3]);
        let eq35_e381_d_n5: f64 = (eq35_e379_d_n5 * s.v[3]);
        let eq35_e381_d_n6: f64 = (eq35_e379_d_n6 * s.v[3]);
        let eq35_e381_d_n7: f64 = (eq35_e379_d_n7 * s.v[3]);
        let eq35_e381_d_n8: f64 = (eq35_e379_d_n8 * s.v[3]);
        let eq35_e381_d_n9: f64 = (eq35_e379_d_n9 * s.v[3]);
        let eq35_e381_d_b0: f64 = (eq35_e379_d_b0 * s.v[3]);
        let eq35_e381_d_b1: f64 = (eq35_e379_d_b1 * s.v[3]);
        let eq35_e381_d_b2: f64 = (eq35_e379_d_b2 * s.v[3]);
        let eq35_e381_d_b3: f64 = (eq35_e379_d_b3 * s.v[3]);
        let eq35_e381_d_b4: f64 = (eq35_e379_d_b4 * s.v[3]);
        let eq35_e381_d_b5: f64 = (eq35_e379_d_b5 * s.v[3]);
        let eq35_e381_d_b6: f64 = (eq35_e379_d_b6 * s.v[3]);
        let eq35_e381_d_b7: f64 = (eq35_e379_d_b7 * s.v[3]);
        let eq35_value: f64 = eq35_e381;
        let eq35_node_derivatives: [f64; 10] = [eq35_e381_d_n0, eq35_e381_d_n1, eq35_e381_d_n2, eq35_e381_d_n3, eq35_e381_d_n4, eq35_e381_d_n5, eq35_e381_d_n6, eq35_e381_d_n7, eq35_e381_d_n8, eq35_e381_d_n9];
        let eq35_branch_derivatives: [f64; 8] = [eq35_e381_d_b0, eq35_e381_d_b1, eq35_e381_d_b2, eq35_e381_d_b3, eq35_e381_d_b4, eq35_e381_d_b5, eq35_e381_d_b6, eq35_e381_d_b7];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(6),
            multiplicity * (eq35_value),
            &eq35_node_derivatives,
            &eq35_branch_derivatives,
            multiplicity,
        );
        let eq36_e384: f64 = (s.v[9] * s.v[58]);
        let eq36_e384_d_n0: f64 = (s.v[9] * s.dn[58][0]);
        let eq36_e384_d_n1: f64 = (s.v[9] * s.dn[58][1]);
        let eq36_e384_d_n2: f64 = (s.v[9] * s.dn[58][2]);
        let eq36_e384_d_n3: f64 = (s.v[9] * s.dn[58][3]);
        let eq36_e384_d_n4: f64 = (s.v[9] * s.dn[58][4]);
        let eq36_e384_d_n5: f64 = (s.v[9] * s.dn[58][5]);
        let eq36_e384_d_n6: f64 = (s.v[9] * s.dn[58][6]);
        let eq36_e384_d_n7: f64 = (s.v[9] * s.dn[58][7]);
        let eq36_e384_d_n8: f64 = (s.v[9] * s.dn[58][8]);
        let eq36_e384_d_n9: f64 = (s.v[9] * s.dn[58][9]);
        let eq36_e384_d_b0: f64 = (s.v[9] * s.db[58][0]);
        let eq36_e384_d_b1: f64 = (s.v[9] * s.db[58][1]);
        let eq36_e384_d_b2: f64 = (s.v[9] * s.db[58][2]);
        let eq36_e384_d_b3: f64 = (s.v[9] * s.db[58][3]);
        let eq36_e384_d_b4: f64 = (s.v[9] * s.db[58][4]);
        let eq36_e384_d_b5: f64 = (s.v[9] * s.db[58][5]);
        let eq36_e384_d_b6: f64 = (s.v[9] * s.db[58][6]);
        let eq36_e384_d_b7: f64 = (s.v[9] * s.db[58][7]);
        let eq36_e386: f64 = (eq36_e384 * s.v[3]);
        let eq36_e386_d_n0: f64 = (eq36_e384_d_n0 * s.v[3]);
        let eq36_e386_d_n1: f64 = (eq36_e384_d_n1 * s.v[3]);
        let eq36_e386_d_n2: f64 = (eq36_e384_d_n2 * s.v[3]);
        let eq36_e386_d_n3: f64 = (eq36_e384_d_n3 * s.v[3]);
        let eq36_e386_d_n4: f64 = (eq36_e384_d_n4 * s.v[3]);
        let eq36_e386_d_n5: f64 = (eq36_e384_d_n5 * s.v[3]);
        let eq36_e386_d_n6: f64 = (eq36_e384_d_n6 * s.v[3]);
        let eq36_e386_d_n7: f64 = (eq36_e384_d_n7 * s.v[3]);
        let eq36_e386_d_n8: f64 = (eq36_e384_d_n8 * s.v[3]);
        let eq36_e386_d_n9: f64 = (eq36_e384_d_n9 * s.v[3]);
        let eq36_e386_d_b0: f64 = (eq36_e384_d_b0 * s.v[3]);
        let eq36_e386_d_b1: f64 = (eq36_e384_d_b1 * s.v[3]);
        let eq36_e386_d_b2: f64 = (eq36_e384_d_b2 * s.v[3]);
        let eq36_e386_d_b3: f64 = (eq36_e384_d_b3 * s.v[3]);
        let eq36_e386_d_b4: f64 = (eq36_e384_d_b4 * s.v[3]);
        let eq36_e386_d_b5: f64 = (eq36_e384_d_b5 * s.v[3]);
        let eq36_e386_d_b6: f64 = (eq36_e384_d_b6 * s.v[3]);
        let eq36_e386_d_b7: f64 = (eq36_e384_d_b7 * s.v[3]);
        let eq36_e387: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq36_e386);
        let eq36_e387_d_n0: f64 = (eq36_e386_d_n0 * ddt_scale);
        let eq36_e387_d_n1: f64 = (eq36_e386_d_n1 * ddt_scale);
        let eq36_e387_d_n2: f64 = (eq36_e386_d_n2 * ddt_scale);
        let eq36_e387_d_n3: f64 = (eq36_e386_d_n3 * ddt_scale);
        let eq36_e387_d_n4: f64 = (eq36_e386_d_n4 * ddt_scale);
        let eq36_e387_d_n5: f64 = (eq36_e386_d_n5 * ddt_scale);
        let eq36_e387_d_n6: f64 = (eq36_e386_d_n6 * ddt_scale);
        let eq36_e387_d_n7: f64 = (eq36_e386_d_n7 * ddt_scale);
        let eq36_e387_d_n8: f64 = (eq36_e386_d_n8 * ddt_scale);
        let eq36_e387_d_n9: f64 = (eq36_e386_d_n9 * ddt_scale);
        let eq36_e387_d_b0: f64 = (eq36_e386_d_b0 * ddt_scale);
        let eq36_e387_d_b1: f64 = (eq36_e386_d_b1 * ddt_scale);
        let eq36_e387_d_b2: f64 = (eq36_e386_d_b2 * ddt_scale);
        let eq36_e387_d_b3: f64 = (eq36_e386_d_b3 * ddt_scale);
        let eq36_e387_d_b4: f64 = (eq36_e386_d_b4 * ddt_scale);
        let eq36_e387_d_b5: f64 = (eq36_e386_d_b5 * ddt_scale);
        let eq36_e387_d_b6: f64 = (eq36_e386_d_b6 * ddt_scale);
        let eq36_e387_d_b7: f64 = (eq36_e386_d_b7 * ddt_scale);
        let eq36_value: f64 = eq36_e387;
        let eq36_node_derivatives: [f64; 10] = [eq36_e387_d_n0, eq36_e387_d_n1, eq36_e387_d_n2, eq36_e387_d_n3, eq36_e387_d_n4, eq36_e387_d_n5, eq36_e387_d_n6, eq36_e387_d_n7, eq36_e387_d_n8, eq36_e387_d_n9];
        let eq36_branch_derivatives: [f64; 8] = [eq36_e387_d_b0, eq36_e387_d_b1, eq36_e387_d_b2, eq36_e387_d_b3, eq36_e387_d_b4, eq36_e387_d_b5, eq36_e387_d_b6, eq36_e387_d_b7];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq36_value),
            &eq36_node_derivatives,
            &eq36_branch_derivatives,
            multiplicity,
        );
        let eq37_e390: f64 = (s.v[9] * s.v[55]);
        let eq37_e390_d_n0: f64 = (s.v[9] * s.dn[55][0]);
        let eq37_e390_d_n1: f64 = (s.v[9] * s.dn[55][1]);
        let eq37_e390_d_n2: f64 = (s.v[9] * s.dn[55][2]);
        let eq37_e390_d_n3: f64 = (s.v[9] * s.dn[55][3]);
        let eq37_e390_d_n4: f64 = (s.v[9] * s.dn[55][4]);
        let eq37_e390_d_n5: f64 = (s.v[9] * s.dn[55][5]);
        let eq37_e390_d_n6: f64 = (s.v[9] * s.dn[55][6]);
        let eq37_e390_d_n7: f64 = (s.v[9] * s.dn[55][7]);
        let eq37_e390_d_n8: f64 = (s.v[9] * s.dn[55][8]);
        let eq37_e390_d_n9: f64 = (s.v[9] * s.dn[55][9]);
        let eq37_e390_d_b0: f64 = (s.v[9] * s.db[55][0]);
        let eq37_e390_d_b1: f64 = (s.v[9] * s.db[55][1]);
        let eq37_e390_d_b2: f64 = (s.v[9] * s.db[55][2]);
        let eq37_e390_d_b3: f64 = (s.v[9] * s.db[55][3]);
        let eq37_e390_d_b4: f64 = (s.v[9] * s.db[55][4]);
        let eq37_e390_d_b5: f64 = (s.v[9] * s.db[55][5]);
        let eq37_e390_d_b6: f64 = (s.v[9] * s.db[55][6]);
        let eq37_e390_d_b7: f64 = (s.v[9] * s.db[55][7]);
        let eq37_e392: f64 = (eq37_e390 * s.v[3]);
        let eq37_e392_d_n0: f64 = (eq37_e390_d_n0 * s.v[3]);
        let eq37_e392_d_n1: f64 = (eq37_e390_d_n1 * s.v[3]);
        let eq37_e392_d_n2: f64 = (eq37_e390_d_n2 * s.v[3]);
        let eq37_e392_d_n3: f64 = (eq37_e390_d_n3 * s.v[3]);
        let eq37_e392_d_n4: f64 = (eq37_e390_d_n4 * s.v[3]);
        let eq37_e392_d_n5: f64 = (eq37_e390_d_n5 * s.v[3]);
        let eq37_e392_d_n6: f64 = (eq37_e390_d_n6 * s.v[3]);
        let eq37_e392_d_n7: f64 = (eq37_e390_d_n7 * s.v[3]);
        let eq37_e392_d_n8: f64 = (eq37_e390_d_n8 * s.v[3]);
        let eq37_e392_d_n9: f64 = (eq37_e390_d_n9 * s.v[3]);
        let eq37_e392_d_b0: f64 = (eq37_e390_d_b0 * s.v[3]);
        let eq37_e392_d_b1: f64 = (eq37_e390_d_b1 * s.v[3]);
        let eq37_e392_d_b2: f64 = (eq37_e390_d_b2 * s.v[3]);
        let eq37_e392_d_b3: f64 = (eq37_e390_d_b3 * s.v[3]);
        let eq37_e392_d_b4: f64 = (eq37_e390_d_b4 * s.v[3]);
        let eq37_e392_d_b5: f64 = (eq37_e390_d_b5 * s.v[3]);
        let eq37_e392_d_b6: f64 = (eq37_e390_d_b6 * s.v[3]);
        let eq37_e392_d_b7: f64 = (eq37_e390_d_b7 * s.v[3]);
        let eq37_e393: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, eq37_e392);
        let eq37_e393_d_n0: f64 = (eq37_e392_d_n0 * ddt_scale);
        let eq37_e393_d_n1: f64 = (eq37_e392_d_n1 * ddt_scale);
        let eq37_e393_d_n2: f64 = (eq37_e392_d_n2 * ddt_scale);
        let eq37_e393_d_n3: f64 = (eq37_e392_d_n3 * ddt_scale);
        let eq37_e393_d_n4: f64 = (eq37_e392_d_n4 * ddt_scale);
        let eq37_e393_d_n5: f64 = (eq37_e392_d_n5 * ddt_scale);
        let eq37_e393_d_n6: f64 = (eq37_e392_d_n6 * ddt_scale);
        let eq37_e393_d_n7: f64 = (eq37_e392_d_n7 * ddt_scale);
        let eq37_e393_d_n8: f64 = (eq37_e392_d_n8 * ddt_scale);
        let eq37_e393_d_n9: f64 = (eq37_e392_d_n9 * ddt_scale);
        let eq37_e393_d_b0: f64 = (eq37_e392_d_b0 * ddt_scale);
        let eq37_e393_d_b1: f64 = (eq37_e392_d_b1 * ddt_scale);
        let eq37_e393_d_b2: f64 = (eq37_e392_d_b2 * ddt_scale);
        let eq37_e393_d_b3: f64 = (eq37_e392_d_b3 * ddt_scale);
        let eq37_e393_d_b4: f64 = (eq37_e392_d_b4 * ddt_scale);
        let eq37_e393_d_b5: f64 = (eq37_e392_d_b5 * ddt_scale);
        let eq37_e393_d_b6: f64 = (eq37_e392_d_b6 * ddt_scale);
        let eq37_e393_d_b7: f64 = (eq37_e392_d_b7 * ddt_scale);
        let eq37_value: f64 = eq37_e393;
        let eq37_node_derivatives: [f64; 10] = [eq37_e393_d_n0, eq37_e393_d_n1, eq37_e393_d_n2, eq37_e393_d_n3, eq37_e393_d_n4, eq37_e393_d_n5, eq37_e393_d_n6, eq37_e393_d_n7, eq37_e393_d_n8, eq37_e393_d_n9];
        let eq37_branch_derivatives: [f64; 8] = [eq37_e393_d_b0, eq37_e393_d_b1, eq37_e393_d_b2, eq37_e393_d_b3, eq37_e393_d_b4, eq37_e393_d_b5, eq37_e393_d_b6, eq37_e393_d_b7];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq37_value),
            &eq37_node_derivatives,
            &eq37_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let eq38_e396: f64 = (s.v[9] * s.v[60]);
        let eq38_e396_d_n0: f64 = (s.v[9] * s.dn[60][0]);
        let eq38_e396_d_n1: f64 = (s.v[9] * s.dn[60][1]);
        let eq38_e396_d_n2: f64 = (s.v[9] * s.dn[60][2]);
        let eq38_e396_d_n3: f64 = (s.v[9] * s.dn[60][3]);
        let eq38_e396_d_n4: f64 = (s.v[9] * s.dn[60][4]);
        let eq38_e396_d_n5: f64 = (s.v[9] * s.dn[60][5]);
        let eq38_e396_d_n6: f64 = (s.v[9] * s.dn[60][6]);
        let eq38_e396_d_n7: f64 = (s.v[9] * s.dn[60][7]);
        let eq38_e396_d_n8: f64 = (s.v[9] * s.dn[60][8]);
        let eq38_e396_d_n9: f64 = (s.v[9] * s.dn[60][9]);
        let eq38_e396_d_b0: f64 = (s.v[9] * s.db[60][0]);
        let eq38_e396_d_b1: f64 = (s.v[9] * s.db[60][1]);
        let eq38_e396_d_b2: f64 = (s.v[9] * s.db[60][2]);
        let eq38_e396_d_b3: f64 = (s.v[9] * s.db[60][3]);
        let eq38_e396_d_b4: f64 = (s.v[9] * s.db[60][4]);
        let eq38_e396_d_b5: f64 = (s.v[9] * s.db[60][5]);
        let eq38_e396_d_b6: f64 = (s.v[9] * s.db[60][6]);
        let eq38_e396_d_b7: f64 = (s.v[9] * s.db[60][7]);
        let eq38_e398: f64 = (eq38_e396 * s.v[3]);
        let eq38_e398_d_n0: f64 = (eq38_e396_d_n0 * s.v[3]);
        let eq38_e398_d_n1: f64 = (eq38_e396_d_n1 * s.v[3]);
        let eq38_e398_d_n2: f64 = (eq38_e396_d_n2 * s.v[3]);
        let eq38_e398_d_n3: f64 = (eq38_e396_d_n3 * s.v[3]);
        let eq38_e398_d_n4: f64 = (eq38_e396_d_n4 * s.v[3]);
        let eq38_e398_d_n5: f64 = (eq38_e396_d_n5 * s.v[3]);
        let eq38_e398_d_n6: f64 = (eq38_e396_d_n6 * s.v[3]);
        let eq38_e398_d_n7: f64 = (eq38_e396_d_n7 * s.v[3]);
        let eq38_e398_d_n8: f64 = (eq38_e396_d_n8 * s.v[3]);
        let eq38_e398_d_n9: f64 = (eq38_e396_d_n9 * s.v[3]);
        let eq38_e398_d_b0: f64 = (eq38_e396_d_b0 * s.v[3]);
        let eq38_e398_d_b1: f64 = (eq38_e396_d_b1 * s.v[3]);
        let eq38_e398_d_b2: f64 = (eq38_e396_d_b2 * s.v[3]);
        let eq38_e398_d_b3: f64 = (eq38_e396_d_b3 * s.v[3]);
        let eq38_e398_d_b4: f64 = (eq38_e396_d_b4 * s.v[3]);
        let eq38_e398_d_b5: f64 = (eq38_e396_d_b5 * s.v[3]);
        let eq38_e398_d_b6: f64 = (eq38_e396_d_b6 * s.v[3]);
        let eq38_e398_d_b7: f64 = (eq38_e396_d_b7 * s.v[3]);
        let eq38_e399: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, eq38_e398);
        let eq38_e399_d_n0: f64 = (eq38_e398_d_n0 * ddt_scale);
        let eq38_e399_d_n1: f64 = (eq38_e398_d_n1 * ddt_scale);
        let eq38_e399_d_n2: f64 = (eq38_e398_d_n2 * ddt_scale);
        let eq38_e399_d_n3: f64 = (eq38_e398_d_n3 * ddt_scale);
        let eq38_e399_d_n4: f64 = (eq38_e398_d_n4 * ddt_scale);
        let eq38_e399_d_n5: f64 = (eq38_e398_d_n5 * ddt_scale);
        let eq38_e399_d_n6: f64 = (eq38_e398_d_n6 * ddt_scale);
        let eq38_e399_d_n7: f64 = (eq38_e398_d_n7 * ddt_scale);
        let eq38_e399_d_n8: f64 = (eq38_e398_d_n8 * ddt_scale);
        let eq38_e399_d_n9: f64 = (eq38_e398_d_n9 * ddt_scale);
        let eq38_e399_d_b0: f64 = (eq38_e398_d_b0 * ddt_scale);
        let eq38_e399_d_b1: f64 = (eq38_e398_d_b1 * ddt_scale);
        let eq38_e399_d_b2: f64 = (eq38_e398_d_b2 * ddt_scale);
        let eq38_e399_d_b3: f64 = (eq38_e398_d_b3 * ddt_scale);
        let eq38_e399_d_b4: f64 = (eq38_e398_d_b4 * ddt_scale);
        let eq38_e399_d_b5: f64 = (eq38_e398_d_b5 * ddt_scale);
        let eq38_e399_d_b6: f64 = (eq38_e398_d_b6 * ddt_scale);
        let eq38_e399_d_b7: f64 = (eq38_e398_d_b7 * ddt_scale);
        let eq38_value: f64 = eq38_e399;
        let eq38_node_derivatives: [f64; 10] = [eq38_e399_d_n0, eq38_e399_d_n1, eq38_e399_d_n2, eq38_e399_d_n3, eq38_e399_d_n4, eq38_e399_d_n5, eq38_e399_d_n6, eq38_e399_d_n7, eq38_e399_d_n8, eq38_e399_d_n9];
        let eq38_branch_derivatives: [f64; 8] = [eq38_e399_d_b0, eq38_e399_d_b1, eq38_e399_d_b2, eq38_e399_d_b3, eq38_e399_d_b4, eq38_e399_d_b5, eq38_e399_d_b6, eq38_e399_d_b7];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(4),
            multiplicity * (eq38_value),
            &eq38_node_derivatives,
            &eq38_branch_derivatives,
            multiplicity,
        );
        let eq39_e402: f64 = (s.v[9] * s.v[62]);
        let eq39_e402_d_n0: f64 = (s.v[9] * s.dn[62][0]);
        let eq39_e402_d_n1: f64 = (s.v[9] * s.dn[62][1]);
        let eq39_e402_d_n2: f64 = (s.v[9] * s.dn[62][2]);
        let eq39_e402_d_n3: f64 = (s.v[9] * s.dn[62][3]);
        let eq39_e402_d_n4: f64 = (s.v[9] * s.dn[62][4]);
        let eq39_e402_d_n5: f64 = (s.v[9] * s.dn[62][5]);
        let eq39_e402_d_n6: f64 = (s.v[9] * s.dn[62][6]);
        let eq39_e402_d_n7: f64 = (s.v[9] * s.dn[62][7]);
        let eq39_e402_d_n8: f64 = (s.v[9] * s.dn[62][8]);
        let eq39_e402_d_n9: f64 = (s.v[9] * s.dn[62][9]);
        let eq39_e402_d_b0: f64 = (s.v[9] * s.db[62][0]);
        let eq39_e402_d_b1: f64 = (s.v[9] * s.db[62][1]);
        let eq39_e402_d_b2: f64 = (s.v[9] * s.db[62][2]);
        let eq39_e402_d_b3: f64 = (s.v[9] * s.db[62][3]);
        let eq39_e402_d_b4: f64 = (s.v[9] * s.db[62][4]);
        let eq39_e402_d_b5: f64 = (s.v[9] * s.db[62][5]);
        let eq39_e402_d_b6: f64 = (s.v[9] * s.db[62][6]);
        let eq39_e402_d_b7: f64 = (s.v[9] * s.db[62][7]);
        let eq39_e404: f64 = (eq39_e402 * s.v[3]);
        let eq39_e404_d_n0: f64 = (eq39_e402_d_n0 * s.v[3]);
        let eq39_e404_d_n1: f64 = (eq39_e402_d_n1 * s.v[3]);
        let eq39_e404_d_n2: f64 = (eq39_e402_d_n2 * s.v[3]);
        let eq39_e404_d_n3: f64 = (eq39_e402_d_n3 * s.v[3]);
        let eq39_e404_d_n4: f64 = (eq39_e402_d_n4 * s.v[3]);
        let eq39_e404_d_n5: f64 = (eq39_e402_d_n5 * s.v[3]);
        let eq39_e404_d_n6: f64 = (eq39_e402_d_n6 * s.v[3]);
        let eq39_e404_d_n7: f64 = (eq39_e402_d_n7 * s.v[3]);
        let eq39_e404_d_n8: f64 = (eq39_e402_d_n8 * s.v[3]);
        let eq39_e404_d_n9: f64 = (eq39_e402_d_n9 * s.v[3]);
        let eq39_e404_d_b0: f64 = (eq39_e402_d_b0 * s.v[3]);
        let eq39_e404_d_b1: f64 = (eq39_e402_d_b1 * s.v[3]);
        let eq39_e404_d_b2: f64 = (eq39_e402_d_b2 * s.v[3]);
        let eq39_e404_d_b3: f64 = (eq39_e402_d_b3 * s.v[3]);
        let eq39_e404_d_b4: f64 = (eq39_e402_d_b4 * s.v[3]);
        let eq39_e404_d_b5: f64 = (eq39_e402_d_b5 * s.v[3]);
        let eq39_e404_d_b6: f64 = (eq39_e402_d_b6 * s.v[3]);
        let eq39_e404_d_b7: f64 = (eq39_e402_d_b7 * s.v[3]);
        let eq39_e405: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, eq39_e404);
        let eq39_e405_d_n0: f64 = (eq39_e404_d_n0 * ddt_scale);
        let eq39_e405_d_n1: f64 = (eq39_e404_d_n1 * ddt_scale);
        let eq39_e405_d_n2: f64 = (eq39_e404_d_n2 * ddt_scale);
        let eq39_e405_d_n3: f64 = (eq39_e404_d_n3 * ddt_scale);
        let eq39_e405_d_n4: f64 = (eq39_e404_d_n4 * ddt_scale);
        let eq39_e405_d_n5: f64 = (eq39_e404_d_n5 * ddt_scale);
        let eq39_e405_d_n6: f64 = (eq39_e404_d_n6 * ddt_scale);
        let eq39_e405_d_n7: f64 = (eq39_e404_d_n7 * ddt_scale);
        let eq39_e405_d_n8: f64 = (eq39_e404_d_n8 * ddt_scale);
        let eq39_e405_d_n9: f64 = (eq39_e404_d_n9 * ddt_scale);
        let eq39_e405_d_b0: f64 = (eq39_e404_d_b0 * ddt_scale);
        let eq39_e405_d_b1: f64 = (eq39_e404_d_b1 * ddt_scale);
        let eq39_e405_d_b2: f64 = (eq39_e404_d_b2 * ddt_scale);
        let eq39_e405_d_b3: f64 = (eq39_e404_d_b3 * ddt_scale);
        let eq39_e405_d_b4: f64 = (eq39_e404_d_b4 * ddt_scale);
        let eq39_e405_d_b5: f64 = (eq39_e404_d_b5 * ddt_scale);
        let eq39_e405_d_b6: f64 = (eq39_e404_d_b6 * ddt_scale);
        let eq39_e405_d_b7: f64 = (eq39_e404_d_b7 * ddt_scale);
        let eq39_value: f64 = eq39_e405;
        let eq39_node_derivatives: [f64; 10] = [eq39_e405_d_n0, eq39_e405_d_n1, eq39_e405_d_n2, eq39_e405_d_n3, eq39_e405_d_n4, eq39_e405_d_n5, eq39_e405_d_n6, eq39_e405_d_n7, eq39_e405_d_n8, eq39_e405_d_n9];
        let eq39_branch_derivatives: [f64; 8] = [eq39_e405_d_b0, eq39_e405_d_b1, eq39_e405_d_b2, eq39_e405_d_b3, eq39_e405_d_b4, eq39_e405_d_b5, eq39_e405_d_b6, eq39_e405_d_b7];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(4),
            multiplicity * (eq39_value),
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let eq40_e408: f64 = (s.v[9] * s.v[56]);
        let eq40_e408_d_n0: f64 = (s.v[9] * s.dn[56][0]);
        let eq40_e408_d_n1: f64 = (s.v[9] * s.dn[56][1]);
        let eq40_e408_d_n2: f64 = (s.v[9] * s.dn[56][2]);
        let eq40_e408_d_n3: f64 = (s.v[9] * s.dn[56][3]);
        let eq40_e408_d_n4: f64 = (s.v[9] * s.dn[56][4]);
        let eq40_e408_d_n5: f64 = (s.v[9] * s.dn[56][5]);
        let eq40_e408_d_n6: f64 = (s.v[9] * s.dn[56][6]);
        let eq40_e408_d_n7: f64 = (s.v[9] * s.dn[56][7]);
        let eq40_e408_d_n8: f64 = (s.v[9] * s.dn[56][8]);
        let eq40_e408_d_n9: f64 = (s.v[9] * s.dn[56][9]);
        let eq40_e408_d_b0: f64 = (s.v[9] * s.db[56][0]);
        let eq40_e408_d_b1: f64 = (s.v[9] * s.db[56][1]);
        let eq40_e408_d_b2: f64 = (s.v[9] * s.db[56][2]);
        let eq40_e408_d_b3: f64 = (s.v[9] * s.db[56][3]);
        let eq40_e408_d_b4: f64 = (s.v[9] * s.db[56][4]);
        let eq40_e408_d_b5: f64 = (s.v[9] * s.db[56][5]);
        let eq40_e408_d_b6: f64 = (s.v[9] * s.db[56][6]);
        let eq40_e408_d_b7: f64 = (s.v[9] * s.db[56][7]);
        let eq40_e410: f64 = (eq40_e408 * s.v[3]);
        let eq40_e410_d_n0: f64 = (eq40_e408_d_n0 * s.v[3]);
        let eq40_e410_d_n1: f64 = (eq40_e408_d_n1 * s.v[3]);
        let eq40_e410_d_n2: f64 = (eq40_e408_d_n2 * s.v[3]);
        let eq40_e410_d_n3: f64 = (eq40_e408_d_n3 * s.v[3]);
        let eq40_e410_d_n4: f64 = (eq40_e408_d_n4 * s.v[3]);
        let eq40_e410_d_n5: f64 = (eq40_e408_d_n5 * s.v[3]);
        let eq40_e410_d_n6: f64 = (eq40_e408_d_n6 * s.v[3]);
        let eq40_e410_d_n7: f64 = (eq40_e408_d_n7 * s.v[3]);
        let eq40_e410_d_n8: f64 = (eq40_e408_d_n8 * s.v[3]);
        let eq40_e410_d_n9: f64 = (eq40_e408_d_n9 * s.v[3]);
        let eq40_e410_d_b0: f64 = (eq40_e408_d_b0 * s.v[3]);
        let eq40_e410_d_b1: f64 = (eq40_e408_d_b1 * s.v[3]);
        let eq40_e410_d_b2: f64 = (eq40_e408_d_b2 * s.v[3]);
        let eq40_e410_d_b3: f64 = (eq40_e408_d_b3 * s.v[3]);
        let eq40_e410_d_b4: f64 = (eq40_e408_d_b4 * s.v[3]);
        let eq40_e410_d_b5: f64 = (eq40_e408_d_b5 * s.v[3]);
        let eq40_e410_d_b6: f64 = (eq40_e408_d_b6 * s.v[3]);
        let eq40_e410_d_b7: f64 = (eq40_e408_d_b7 * s.v[3]);
        let eq40_e411: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, eq40_e410);
        let eq40_e411_d_n0: f64 = (eq40_e410_d_n0 * ddt_scale);
        let eq40_e411_d_n1: f64 = (eq40_e410_d_n1 * ddt_scale);
        let eq40_e411_d_n2: f64 = (eq40_e410_d_n2 * ddt_scale);
        let eq40_e411_d_n3: f64 = (eq40_e410_d_n3 * ddt_scale);
        let eq40_e411_d_n4: f64 = (eq40_e410_d_n4 * ddt_scale);
        let eq40_e411_d_n5: f64 = (eq40_e410_d_n5 * ddt_scale);
        let eq40_e411_d_n6: f64 = (eq40_e410_d_n6 * ddt_scale);
        let eq40_e411_d_n7: f64 = (eq40_e410_d_n7 * ddt_scale);
        let eq40_e411_d_n8: f64 = (eq40_e410_d_n8 * ddt_scale);
        let eq40_e411_d_n9: f64 = (eq40_e410_d_n9 * ddt_scale);
        let eq40_e411_d_b0: f64 = (eq40_e410_d_b0 * ddt_scale);
        let eq40_e411_d_b1: f64 = (eq40_e410_d_b1 * ddt_scale);
        let eq40_e411_d_b2: f64 = (eq40_e410_d_b2 * ddt_scale);
        let eq40_e411_d_b3: f64 = (eq40_e410_d_b3 * ddt_scale);
        let eq40_e411_d_b4: f64 = (eq40_e410_d_b4 * ddt_scale);
        let eq40_e411_d_b5: f64 = (eq40_e410_d_b5 * ddt_scale);
        let eq40_e411_d_b6: f64 = (eq40_e410_d_b6 * ddt_scale);
        let eq40_e411_d_b7: f64 = (eq40_e410_d_b7 * ddt_scale);
        let eq40_value: f64 = eq40_e411;
        let eq40_node_derivatives: [f64; 10] = [eq40_e411_d_n0, eq40_e411_d_n1, eq40_e411_d_n2, eq40_e411_d_n3, eq40_e411_d_n4, eq40_e411_d_n5, eq40_e411_d_n6, eq40_e411_d_n7, eq40_e411_d_n8, eq40_e411_d_n9];
        let eq40_branch_derivatives: [f64; 8] = [eq40_e411_d_b0, eq40_e411_d_b1, eq40_e411_d_b2, eq40_e411_d_b3, eq40_e411_d_b4, eq40_e411_d_b5, eq40_e411_d_b6, eq40_e411_d_b7];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(4),
            multiplicity * (eq40_value),
            &eq40_node_derivatives,
            &eq40_branch_derivatives,
            multiplicity,
        );
        let eq41_e414: f64 = (s.v[9] * s.v[57]);
        let eq41_e414_d_n0: f64 = (s.v[9] * s.dn[57][0]);
        let eq41_e414_d_n1: f64 = (s.v[9] * s.dn[57][1]);
        let eq41_e414_d_n2: f64 = (s.v[9] * s.dn[57][2]);
        let eq41_e414_d_n3: f64 = (s.v[9] * s.dn[57][3]);
        let eq41_e414_d_n4: f64 = (s.v[9] * s.dn[57][4]);
        let eq41_e414_d_n5: f64 = (s.v[9] * s.dn[57][5]);
        let eq41_e414_d_n6: f64 = (s.v[9] * s.dn[57][6]);
        let eq41_e414_d_n7: f64 = (s.v[9] * s.dn[57][7]);
        let eq41_e414_d_n8: f64 = (s.v[9] * s.dn[57][8]);
        let eq41_e414_d_n9: f64 = (s.v[9] * s.dn[57][9]);
        let eq41_e414_d_b0: f64 = (s.v[9] * s.db[57][0]);
        let eq41_e414_d_b1: f64 = (s.v[9] * s.db[57][1]);
        let eq41_e414_d_b2: f64 = (s.v[9] * s.db[57][2]);
        let eq41_e414_d_b3: f64 = (s.v[9] * s.db[57][3]);
        let eq41_e414_d_b4: f64 = (s.v[9] * s.db[57][4]);
        let eq41_e414_d_b5: f64 = (s.v[9] * s.db[57][5]);
        let eq41_e414_d_b6: f64 = (s.v[9] * s.db[57][6]);
        let eq41_e414_d_b7: f64 = (s.v[9] * s.db[57][7]);
        let eq41_e416: f64 = (eq41_e414 * s.v[3]);
        let eq41_e416_d_n0: f64 = (eq41_e414_d_n0 * s.v[3]);
        let eq41_e416_d_n1: f64 = (eq41_e414_d_n1 * s.v[3]);
        let eq41_e416_d_n2: f64 = (eq41_e414_d_n2 * s.v[3]);
        let eq41_e416_d_n3: f64 = (eq41_e414_d_n3 * s.v[3]);
        let eq41_e416_d_n4: f64 = (eq41_e414_d_n4 * s.v[3]);
        let eq41_e416_d_n5: f64 = (eq41_e414_d_n5 * s.v[3]);
        let eq41_e416_d_n6: f64 = (eq41_e414_d_n6 * s.v[3]);
        let eq41_e416_d_n7: f64 = (eq41_e414_d_n7 * s.v[3]);
        let eq41_e416_d_n8: f64 = (eq41_e414_d_n8 * s.v[3]);
        let eq41_e416_d_n9: f64 = (eq41_e414_d_n9 * s.v[3]);
        let eq41_e416_d_b0: f64 = (eq41_e414_d_b0 * s.v[3]);
        let eq41_e416_d_b1: f64 = (eq41_e414_d_b1 * s.v[3]);
        let eq41_e416_d_b2: f64 = (eq41_e414_d_b2 * s.v[3]);
        let eq41_e416_d_b3: f64 = (eq41_e414_d_b3 * s.v[3]);
        let eq41_e416_d_b4: f64 = (eq41_e414_d_b4 * s.v[3]);
        let eq41_e416_d_b5: f64 = (eq41_e414_d_b5 * s.v[3]);
        let eq41_e416_d_b6: f64 = (eq41_e414_d_b6 * s.v[3]);
        let eq41_e416_d_b7: f64 = (eq41_e414_d_b7 * s.v[3]);
        let eq41_e417: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, eq41_e416);
        let eq41_e417_d_n0: f64 = (eq41_e416_d_n0 * ddt_scale);
        let eq41_e417_d_n1: f64 = (eq41_e416_d_n1 * ddt_scale);
        let eq41_e417_d_n2: f64 = (eq41_e416_d_n2 * ddt_scale);
        let eq41_e417_d_n3: f64 = (eq41_e416_d_n3 * ddt_scale);
        let eq41_e417_d_n4: f64 = (eq41_e416_d_n4 * ddt_scale);
        let eq41_e417_d_n5: f64 = (eq41_e416_d_n5 * ddt_scale);
        let eq41_e417_d_n6: f64 = (eq41_e416_d_n6 * ddt_scale);
        let eq41_e417_d_n7: f64 = (eq41_e416_d_n7 * ddt_scale);
        let eq41_e417_d_n8: f64 = (eq41_e416_d_n8 * ddt_scale);
        let eq41_e417_d_n9: f64 = (eq41_e416_d_n9 * ddt_scale);
        let eq41_e417_d_b0: f64 = (eq41_e416_d_b0 * ddt_scale);
        let eq41_e417_d_b1: f64 = (eq41_e416_d_b1 * ddt_scale);
        let eq41_e417_d_b2: f64 = (eq41_e416_d_b2 * ddt_scale);
        let eq41_e417_d_b3: f64 = (eq41_e416_d_b3 * ddt_scale);
        let eq41_e417_d_b4: f64 = (eq41_e416_d_b4 * ddt_scale);
        let eq41_e417_d_b5: f64 = (eq41_e416_d_b5 * ddt_scale);
        let eq41_e417_d_b6: f64 = (eq41_e416_d_b6 * ddt_scale);
        let eq41_e417_d_b7: f64 = (eq41_e416_d_b7 * ddt_scale);
        let eq41_value: f64 = eq41_e417;
        let eq41_node_derivatives: [f64; 10] = [eq41_e417_d_n0, eq41_e417_d_n1, eq41_e417_d_n2, eq41_e417_d_n3, eq41_e417_d_n4, eq41_e417_d_n5, eq41_e417_d_n6, eq41_e417_d_n7, eq41_e417_d_n8, eq41_e417_d_n9];
        let eq41_branch_derivatives: [f64; 8] = [eq41_e417_d_b0, eq41_e417_d_b1, eq41_e417_d_b2, eq41_e417_d_b3, eq41_e417_d_b4, eq41_e417_d_b5, eq41_e417_d_b6, eq41_e417_d_b7];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(4),
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let eq42_e419: f64 = (-s.v[63]);
        let eq42_e419_d_n0: f64 = (-s.dn[63][0]);
        let eq42_e419_d_n1: f64 = (-s.dn[63][1]);
        let eq42_e419_d_n2: f64 = (-s.dn[63][2]);
        let eq42_e419_d_n3: f64 = (-s.dn[63][3]);
        let eq42_e419_d_n4: f64 = (-s.dn[63][4]);
        let eq42_e419_d_n5: f64 = (-s.dn[63][5]);
        let eq42_e419_d_n6: f64 = (-s.dn[63][6]);
        let eq42_e419_d_n7: f64 = (-s.dn[63][7]);
        let eq42_e419_d_n8: f64 = (-s.dn[63][8]);
        let eq42_e419_d_n9: f64 = (-s.dn[63][9]);
        let eq42_e419_d_b0: f64 = (-s.db[63][0]);
        let eq42_e419_d_b1: f64 = (-s.db[63][1]);
        let eq42_e419_d_b2: f64 = (-s.db[63][2]);
        let eq42_e419_d_b3: f64 = (-s.db[63][3]);
        let eq42_e419_d_b4: f64 = (-s.db[63][4]);
        let eq42_e419_d_b5: f64 = (-s.db[63][5]);
        let eq42_e419_d_b6: f64 = (-s.db[63][6]);
        let eq42_e419_d_b7: f64 = (-s.db[63][7]);
        let eq42_e421: f64 = (eq42_e419 * s.v[3]);
        let eq42_e421_d_n0: f64 = (eq42_e419_d_n0 * s.v[3]);
        let eq42_e421_d_n1: f64 = (eq42_e419_d_n1 * s.v[3]);
        let eq42_e421_d_n2: f64 = (eq42_e419_d_n2 * s.v[3]);
        let eq42_e421_d_n3: f64 = (eq42_e419_d_n3 * s.v[3]);
        let eq42_e421_d_n4: f64 = (eq42_e419_d_n4 * s.v[3]);
        let eq42_e421_d_n5: f64 = (eq42_e419_d_n5 * s.v[3]);
        let eq42_e421_d_n6: f64 = (eq42_e419_d_n6 * s.v[3]);
        let eq42_e421_d_n7: f64 = (eq42_e419_d_n7 * s.v[3]);
        let eq42_e421_d_n8: f64 = (eq42_e419_d_n8 * s.v[3]);
        let eq42_e421_d_n9: f64 = (eq42_e419_d_n9 * s.v[3]);
        let eq42_e421_d_b0: f64 = (eq42_e419_d_b0 * s.v[3]);
        let eq42_e421_d_b1: f64 = (eq42_e419_d_b1 * s.v[3]);
        let eq42_e421_d_b2: f64 = (eq42_e419_d_b2 * s.v[3]);
        let eq42_e421_d_b3: f64 = (eq42_e419_d_b3 * s.v[3]);
        let eq42_e421_d_b4: f64 = (eq42_e419_d_b4 * s.v[3]);
        let eq42_e421_d_b5: f64 = (eq42_e419_d_b5 * s.v[3]);
        let eq42_e421_d_b6: f64 = (eq42_e419_d_b6 * s.v[3]);
        let eq42_e421_d_b7: f64 = (eq42_e419_d_b7 * s.v[3]);
        let eq42_e422: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, eq42_e421);
        let eq42_e422_d_n0: f64 = (eq42_e421_d_n0 * ddt_scale);
        let eq42_e422_d_n1: f64 = (eq42_e421_d_n1 * ddt_scale);
        let eq42_e422_d_n2: f64 = (eq42_e421_d_n2 * ddt_scale);
        let eq42_e422_d_n3: f64 = (eq42_e421_d_n3 * ddt_scale);
        let eq42_e422_d_n4: f64 = (eq42_e421_d_n4 * ddt_scale);
        let eq42_e422_d_n5: f64 = (eq42_e421_d_n5 * ddt_scale);
        let eq42_e422_d_n6: f64 = (eq42_e421_d_n6 * ddt_scale);
        let eq42_e422_d_n7: f64 = (eq42_e421_d_n7 * ddt_scale);
        let eq42_e422_d_n8: f64 = (eq42_e421_d_n8 * ddt_scale);
        let eq42_e422_d_n9: f64 = (eq42_e421_d_n9 * ddt_scale);
        let eq42_e422_d_b0: f64 = (eq42_e421_d_b0 * ddt_scale);
        let eq42_e422_d_b1: f64 = (eq42_e421_d_b1 * ddt_scale);
        let eq42_e422_d_b2: f64 = (eq42_e421_d_b2 * ddt_scale);
        let eq42_e422_d_b3: f64 = (eq42_e421_d_b3 * ddt_scale);
        let eq42_e422_d_b4: f64 = (eq42_e421_d_b4 * ddt_scale);
        let eq42_e422_d_b5: f64 = (eq42_e421_d_b5 * ddt_scale);
        let eq42_e422_d_b6: f64 = (eq42_e421_d_b6 * ddt_scale);
        let eq42_e422_d_b7: f64 = (eq42_e421_d_b7 * ddt_scale);
        let eq42_value: f64 = eq42_e422;
        let eq42_node_derivatives: [f64; 10] = [eq42_e422_d_n0, eq42_e422_d_n1, eq42_e422_d_n2, eq42_e422_d_n3, eq42_e422_d_n4, eq42_e422_d_n5, eq42_e422_d_n6, eq42_e422_d_n7, eq42_e422_d_n8, eq42_e422_d_n9];
        let eq42_branch_derivatives: [f64; 8] = [eq42_e422_d_b0, eq42_e422_d_b1, eq42_e422_d_b2, eq42_e422_d_b3, eq42_e422_d_b4, eq42_e422_d_b5, eq42_e422_d_b6, eq42_e422_d_b7];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq42_value),
            &eq42_node_derivatives,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let eq43_e425: f64 = (s.v[63] * s.v[3]);
        let eq43_e425_d_n0: f64 = (s.dn[63][0] * s.v[3]);
        let eq43_e425_d_n1: f64 = (s.dn[63][1] * s.v[3]);
        let eq43_e425_d_n2: f64 = (s.dn[63][2] * s.v[3]);
        let eq43_e425_d_n3: f64 = (s.dn[63][3] * s.v[3]);
        let eq43_e425_d_n4: f64 = (s.dn[63][4] * s.v[3]);
        let eq43_e425_d_n5: f64 = (s.dn[63][5] * s.v[3]);
        let eq43_e425_d_n6: f64 = (s.dn[63][6] * s.v[3]);
        let eq43_e425_d_n7: f64 = (s.dn[63][7] * s.v[3]);
        let eq43_e425_d_n8: f64 = (s.dn[63][8] * s.v[3]);
        let eq43_e425_d_n9: f64 = (s.dn[63][9] * s.v[3]);
        let eq43_e425_d_b0: f64 = (s.db[63][0] * s.v[3]);
        let eq43_e425_d_b1: f64 = (s.db[63][1] * s.v[3]);
        let eq43_e425_d_b2: f64 = (s.db[63][2] * s.v[3]);
        let eq43_e425_d_b3: f64 = (s.db[63][3] * s.v[3]);
        let eq43_e425_d_b4: f64 = (s.db[63][4] * s.v[3]);
        let eq43_e425_d_b5: f64 = (s.db[63][5] * s.v[3]);
        let eq43_e425_d_b6: f64 = (s.db[63][6] * s.v[3]);
        let eq43_e425_d_b7: f64 = (s.db[63][7] * s.v[3]);
        let eq43_e426: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 12, eq43_e425);
        let eq43_e426_d_n0: f64 = (eq43_e425_d_n0 * ddt_scale);
        let eq43_e426_d_n1: f64 = (eq43_e425_d_n1 * ddt_scale);
        let eq43_e426_d_n2: f64 = (eq43_e425_d_n2 * ddt_scale);
        let eq43_e426_d_n3: f64 = (eq43_e425_d_n3 * ddt_scale);
        let eq43_e426_d_n4: f64 = (eq43_e425_d_n4 * ddt_scale);
        let eq43_e426_d_n5: f64 = (eq43_e425_d_n5 * ddt_scale);
        let eq43_e426_d_n6: f64 = (eq43_e425_d_n6 * ddt_scale);
        let eq43_e426_d_n7: f64 = (eq43_e425_d_n7 * ddt_scale);
        let eq43_e426_d_n8: f64 = (eq43_e425_d_n8 * ddt_scale);
        let eq43_e426_d_n9: f64 = (eq43_e425_d_n9 * ddt_scale);
        let eq43_e426_d_b0: f64 = (eq43_e425_d_b0 * ddt_scale);
        let eq43_e426_d_b1: f64 = (eq43_e425_d_b1 * ddt_scale);
        let eq43_e426_d_b2: f64 = (eq43_e425_d_b2 * ddt_scale);
        let eq43_e426_d_b3: f64 = (eq43_e425_d_b3 * ddt_scale);
        let eq43_e426_d_b4: f64 = (eq43_e425_d_b4 * ddt_scale);
        let eq43_e426_d_b5: f64 = (eq43_e425_d_b5 * ddt_scale);
        let eq43_e426_d_b6: f64 = (eq43_e425_d_b6 * ddt_scale);
        let eq43_e426_d_b7: f64 = (eq43_e425_d_b7 * ddt_scale);
        let eq43_value: f64 = eq43_e426;
        let eq43_node_derivatives: [f64; 10] = [eq43_e426_d_n0, eq43_e426_d_n1, eq43_e426_d_n2, eq43_e426_d_n3, eq43_e426_d_n4, eq43_e426_d_n5, eq43_e426_d_n6, eq43_e426_d_n7, eq43_e426_d_n8, eq43_e426_d_n9];
        let eq43_branch_derivatives: [f64; 8] = [eq43_e426_d_b0, eq43_e426_d_b1, eq43_e426_d_b2, eq43_e426_d_b3, eq43_e426_d_b4, eq43_e426_d_b5, eq43_e426_d_b6, eq43_e426_d_b7];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(4),
            multiplicity * (eq43_value),
            &eq43_node_derivatives,
            &eq43_branch_derivatives,
            multiplicity,
        );
        let eq44_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (eq44_value),
        );
        let eq45_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (eq45_value),
        );
        let eq46_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(4),
            Some(6),
            multiplicity * (eq46_value),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let eq2_e98_q: f64 = (nv9 - 0.0);
        let eq2_e99: f64 = (p.p83 * (nv9 - 0.0));
        let eq2_e99_d_n9: f64 = p.p83;
        let eq2_e99_q: f64 = (p.p83 * eq2_e98_q);
        let eq2_e99_q_d_n9: f64 = p.p83;
        stamper.stamp_current_reactive_node1(
            Some(nodes[9]),
            None,
            nodes[9],
            multiplicity * (eq2_e99_q_d_n9),
        );
        let (eq5_e121, eq5_e121_d_n0, eq5_e121_d_n1, eq5_e121_d_n2, eq5_e121_d_n3, eq5_e121_d_n4, eq5_e121_d_n5, eq5_e121_d_n6, eq5_e121_d_n7, eq5_e121_d_n8, eq5_e121_d_n9, eq5_e121_d_b0, eq5_e121_d_b1, eq5_e121_d_b2, eq5_e121_d_b3, eq5_e121_d_b4, eq5_e121_d_b5, eq5_e121_d_b6, eq5_e121_d_b7, eq5_e121_q, eq5_e121_q_d_n0, eq5_e121_q_d_n1, eq5_e121_q_d_n2, eq5_e121_q_d_n3, eq5_e121_q_d_n4, eq5_e121_q_d_n5, eq5_e121_q_d_n6, eq5_e121_q_d_n7, eq5_e121_q_d_n8, eq5_e121_q_d_n9, eq5_e121_q_d_b0, eq5_e121_q_d_b1, eq5_e121_q_d_b2, eq5_e121_q_d_b3, eq5_e121_q_d_b4, eq5_e121_q_d_b5, eq5_e121_q_d_b6, eq5_e121_q_d_b7,) = {
    if s.b[115] {
        let eq5_e118_q: f64 = (nv8 - 0.0);
        let eq5_e119: f64 = (s.v[54] * (nv8 - 0.0));
        let eq5_e119_d_n0: f64 = (s.dn[54][0] * (nv8 - 0.0));
        let eq5_e119_d_n1: f64 = (s.dn[54][1] * (nv8 - 0.0));
        let eq5_e119_d_n2: f64 = (s.dn[54][2] * (nv8 - 0.0));
        let eq5_e119_d_n3: f64 = (s.dn[54][3] * (nv8 - 0.0));
        let eq5_e119_d_n4: f64 = (s.dn[54][4] * (nv8 - 0.0));
        let eq5_e119_d_n5: f64 = (s.dn[54][5] * (nv8 - 0.0));
        let eq5_e119_d_n6: f64 = (s.dn[54][6] * (nv8 - 0.0));
        let eq5_e119_d_n7: f64 = (s.dn[54][7] * (nv8 - 0.0));
        let eq5_e119_d_n8: f64 = ((s.dn[54][8] * (nv8 - 0.0)) + s.v[54]);
        let eq5_e119_d_n9: f64 = (s.dn[54][9] * (nv8 - 0.0));
        let eq5_e119_d_b0: f64 = (s.db[54][0] * (nv8 - 0.0));
        let eq5_e119_d_b1: f64 = (s.db[54][1] * (nv8 - 0.0));
        let eq5_e119_d_b2: f64 = (s.db[54][2] * (nv8 - 0.0));
        let eq5_e119_d_b3: f64 = (s.db[54][3] * (nv8 - 0.0));
        let eq5_e119_d_b4: f64 = (s.db[54][4] * (nv8 - 0.0));
        let eq5_e119_d_b5: f64 = (s.db[54][5] * (nv8 - 0.0));
        let eq5_e119_d_b6: f64 = (s.db[54][6] * (nv8 - 0.0));
        let eq5_e119_d_b7: f64 = (s.db[54][7] * (nv8 - 0.0));
        let eq5_e119_q: f64 = (s.v[54] * eq5_e118_q);
        let eq5_e119_q_d_n0: f64 = (s.dn[54][0] * eq5_e118_q);
        let eq5_e119_q_d_n1: f64 = (s.dn[54][1] * eq5_e118_q);
        let eq5_e119_q_d_n2: f64 = (s.dn[54][2] * eq5_e118_q);
        let eq5_e119_q_d_n3: f64 = (s.dn[54][3] * eq5_e118_q);
        let eq5_e119_q_d_n4: f64 = (s.dn[54][4] * eq5_e118_q);
        let eq5_e119_q_d_n5: f64 = (s.dn[54][5] * eq5_e118_q);
        let eq5_e119_q_d_n6: f64 = (s.dn[54][6] * eq5_e118_q);
        let eq5_e119_q_d_n7: f64 = (s.dn[54][7] * eq5_e118_q);
        let eq5_e119_q_d_n8: f64 = ((s.dn[54][8] * eq5_e118_q) + s.v[54]);
        let eq5_e119_q_d_n9: f64 = (s.dn[54][9] * eq5_e118_q);
        let eq5_e119_q_d_b0: f64 = (s.db[54][0] * eq5_e118_q);
        let eq5_e119_q_d_b1: f64 = (s.db[54][1] * eq5_e118_q);
        let eq5_e119_q_d_b2: f64 = (s.db[54][2] * eq5_e118_q);
        let eq5_e119_q_d_b3: f64 = (s.db[54][3] * eq5_e118_q);
        let eq5_e119_q_d_b4: f64 = (s.db[54][4] * eq5_e118_q);
        let eq5_e119_q_d_b5: f64 = (s.db[54][5] * eq5_e118_q);
        let eq5_e119_q_d_b6: f64 = (s.db[54][6] * eq5_e118_q);
        let eq5_e119_q_d_b7: f64 = (s.db[54][7] * eq5_e118_q);
        (eq5_e119, eq5_e119_d_n0, eq5_e119_d_n1, eq5_e119_d_n2, eq5_e119_d_n3, eq5_e119_d_n4, eq5_e119_d_n5, eq5_e119_d_n6, eq5_e119_d_n7, eq5_e119_d_n8, eq5_e119_d_n9, eq5_e119_d_b0, eq5_e119_d_b1, eq5_e119_d_b2, eq5_e119_d_b3, eq5_e119_d_b4, eq5_e119_d_b5, eq5_e119_d_b6, eq5_e119_d_b7, eq5_e119_q, eq5_e119_q_d_n0, eq5_e119_q_d_n1, eq5_e119_q_d_n2, eq5_e119_q_d_n3, eq5_e119_q_d_n4, eq5_e119_q_d_n5, eq5_e119_q_d_n6, eq5_e119_q_d_n7, eq5_e119_q_d_n8, eq5_e119_q_d_n9, eq5_e119_q_d_b0, eq5_e119_q_d_b1, eq5_e119_q_d_b2, eq5_e119_q_d_b3, eq5_e119_q_d_b4, eq5_e119_q_d_b5, eq5_e119_q_d_b6, eq5_e119_q_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_reactive_node_derivatives: [f64; 10] = [eq5_e121_q_d_n0, eq5_e121_q_d_n1, eq5_e121_q_d_n2, eq5_e121_q_d_n3, eq5_e121_q_d_n4, eq5_e121_q_d_n5, eq5_e121_q_d_n6, eq5_e121_q_d_n7, eq5_e121_q_d_n8, eq5_e121_q_d_n9];
        let eq5_reactive_branch_derivatives: [f64; 8] = [eq5_e121_q_d_b0, eq5_e121_q_d_b1, eq5_e121_q_d_b2, eq5_e121_q_d_b3, eq5_e121_q_d_b4, eq5_e121_q_d_b5, eq5_e121_q_d_b6, eq5_e121_q_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            None,
            nodes,
            &eq5_reactive_node_derivatives,
            branches,
            &eq5_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq9_e154, eq9_e154_d_n3, eq9_e154_q, eq9_e154_q_d_n3,) = {
    if s.b[122] {
        let eq9_e151: f64 = ((nv3 - 0.0) * p.p34);
        let eq9_e151_d_n3: f64 = p.p34;
        let eq9_e152_q: f64 = eq9_e151;
        (eq9_e151, eq9_e151_d_n3, eq9_e152_q, eq9_e151_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (eq9_e154_q_d_n3),
        );
        let (eq13_e195, eq13_e195_d_n3, eq13_e195_q, eq13_e195_q_d_n3,) = {
    if ((!s.b[122]) && s.b[123]) {
        let eq13_e192: f64 = (p.p34 * (nv3 - 0.0));
        let eq13_e192_d_n3: f64 = p.p34;
        let eq13_e193_q: f64 = eq13_e192;
        (eq13_e192, eq13_e192_d_n3, eq13_e193_q, eq13_e192_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (eq13_e195_q_d_n3),
        );
        let (eq15_e214, eq15_e214_d_n7, eq15_e214_q, eq15_e214_q_d_n7,) = {
    if ((!s.b[122]) && s.b[123]) {
        let eq15_e211: f64 = (p.p36 * (nv7 - 0.0));
        let eq15_e211_d_n7: f64 = p.p36;
        let eq15_e212_q: f64 = eq15_e211;
        (eq15_e211, eq15_e211_d_n7, eq15_e212_q, eq15_e211_d_n7,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[7]),
            None,
            nodes[7],
            multiplicity * (eq15_e214_q_d_n7),
        );
        let eq36_e384: f64 = (s.v[9] * s.v[58]);
        let eq36_e384_d_n0: f64 = (s.v[9] * s.dn[58][0]);
        let eq36_e384_d_n1: f64 = (s.v[9] * s.dn[58][1]);
        let eq36_e384_d_n2: f64 = (s.v[9] * s.dn[58][2]);
        let eq36_e384_d_n3: f64 = (s.v[9] * s.dn[58][3]);
        let eq36_e384_d_n4: f64 = (s.v[9] * s.dn[58][4]);
        let eq36_e384_d_n5: f64 = (s.v[9] * s.dn[58][5]);
        let eq36_e384_d_n6: f64 = (s.v[9] * s.dn[58][6]);
        let eq36_e384_d_n7: f64 = (s.v[9] * s.dn[58][7]);
        let eq36_e384_d_n8: f64 = (s.v[9] * s.dn[58][8]);
        let eq36_e384_d_n9: f64 = (s.v[9] * s.dn[58][9]);
        let eq36_e384_d_b0: f64 = (s.v[9] * s.db[58][0]);
        let eq36_e384_d_b1: f64 = (s.v[9] * s.db[58][1]);
        let eq36_e384_d_b2: f64 = (s.v[9] * s.db[58][2]);
        let eq36_e384_d_b3: f64 = (s.v[9] * s.db[58][3]);
        let eq36_e384_d_b4: f64 = (s.v[9] * s.db[58][4]);
        let eq36_e384_d_b5: f64 = (s.v[9] * s.db[58][5]);
        let eq36_e384_d_b6: f64 = (s.v[9] * s.db[58][6]);
        let eq36_e384_d_b7: f64 = (s.v[9] * s.db[58][7]);
        let eq36_e386: f64 = (eq36_e384 * s.v[3]);
        let eq36_e386_d_n0: f64 = (eq36_e384_d_n0 * s.v[3]);
        let eq36_e386_d_n1: f64 = (eq36_e384_d_n1 * s.v[3]);
        let eq36_e386_d_n2: f64 = (eq36_e384_d_n2 * s.v[3]);
        let eq36_e386_d_n3: f64 = (eq36_e384_d_n3 * s.v[3]);
        let eq36_e386_d_n4: f64 = (eq36_e384_d_n4 * s.v[3]);
        let eq36_e386_d_n5: f64 = (eq36_e384_d_n5 * s.v[3]);
        let eq36_e386_d_n6: f64 = (eq36_e384_d_n6 * s.v[3]);
        let eq36_e386_d_n7: f64 = (eq36_e384_d_n7 * s.v[3]);
        let eq36_e386_d_n8: f64 = (eq36_e384_d_n8 * s.v[3]);
        let eq36_e386_d_n9: f64 = (eq36_e384_d_n9 * s.v[3]);
        let eq36_e386_d_b0: f64 = (eq36_e384_d_b0 * s.v[3]);
        let eq36_e386_d_b1: f64 = (eq36_e384_d_b1 * s.v[3]);
        let eq36_e386_d_b2: f64 = (eq36_e384_d_b2 * s.v[3]);
        let eq36_e386_d_b3: f64 = (eq36_e384_d_b3 * s.v[3]);
        let eq36_e386_d_b4: f64 = (eq36_e384_d_b4 * s.v[3]);
        let eq36_e386_d_b5: f64 = (eq36_e384_d_b5 * s.v[3]);
        let eq36_e386_d_b6: f64 = (eq36_e384_d_b6 * s.v[3]);
        let eq36_e386_d_b7: f64 = (eq36_e384_d_b7 * s.v[3]);
        let eq36_e387_q: f64 = eq36_e386;
        let eq36_reactive_node_derivatives: [f64; 10] = [eq36_e386_d_n0, eq36_e386_d_n1, eq36_e386_d_n2, eq36_e386_d_n3, eq36_e386_d_n4, eq36_e386_d_n5, eq36_e386_d_n6, eq36_e386_d_n7, eq36_e386_d_n8, eq36_e386_d_n9];
        let eq36_reactive_branch_derivatives: [f64; 8] = [eq36_e386_d_b0, eq36_e386_d_b1, eq36_e386_d_b2, eq36_e386_d_b3, eq36_e386_d_b4, eq36_e386_d_b5, eq36_e386_d_b6, eq36_e386_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq36_reactive_node_derivatives,
            branches,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let eq37_e390: f64 = (s.v[9] * s.v[55]);
        let eq37_e390_d_n0: f64 = (s.v[9] * s.dn[55][0]);
        let eq37_e390_d_n1: f64 = (s.v[9] * s.dn[55][1]);
        let eq37_e390_d_n2: f64 = (s.v[9] * s.dn[55][2]);
        let eq37_e390_d_n3: f64 = (s.v[9] * s.dn[55][3]);
        let eq37_e390_d_n4: f64 = (s.v[9] * s.dn[55][4]);
        let eq37_e390_d_n5: f64 = (s.v[9] * s.dn[55][5]);
        let eq37_e390_d_n6: f64 = (s.v[9] * s.dn[55][6]);
        let eq37_e390_d_n7: f64 = (s.v[9] * s.dn[55][7]);
        let eq37_e390_d_n8: f64 = (s.v[9] * s.dn[55][8]);
        let eq37_e390_d_n9: f64 = (s.v[9] * s.dn[55][9]);
        let eq37_e390_d_b0: f64 = (s.v[9] * s.db[55][0]);
        let eq37_e390_d_b1: f64 = (s.v[9] * s.db[55][1]);
        let eq37_e390_d_b2: f64 = (s.v[9] * s.db[55][2]);
        let eq37_e390_d_b3: f64 = (s.v[9] * s.db[55][3]);
        let eq37_e390_d_b4: f64 = (s.v[9] * s.db[55][4]);
        let eq37_e390_d_b5: f64 = (s.v[9] * s.db[55][5]);
        let eq37_e390_d_b6: f64 = (s.v[9] * s.db[55][6]);
        let eq37_e390_d_b7: f64 = (s.v[9] * s.db[55][7]);
        let eq37_e392: f64 = (eq37_e390 * s.v[3]);
        let eq37_e392_d_n0: f64 = (eq37_e390_d_n0 * s.v[3]);
        let eq37_e392_d_n1: f64 = (eq37_e390_d_n1 * s.v[3]);
        let eq37_e392_d_n2: f64 = (eq37_e390_d_n2 * s.v[3]);
        let eq37_e392_d_n3: f64 = (eq37_e390_d_n3 * s.v[3]);
        let eq37_e392_d_n4: f64 = (eq37_e390_d_n4 * s.v[3]);
        let eq37_e392_d_n5: f64 = (eq37_e390_d_n5 * s.v[3]);
        let eq37_e392_d_n6: f64 = (eq37_e390_d_n6 * s.v[3]);
        let eq37_e392_d_n7: f64 = (eq37_e390_d_n7 * s.v[3]);
        let eq37_e392_d_n8: f64 = (eq37_e390_d_n8 * s.v[3]);
        let eq37_e392_d_n9: f64 = (eq37_e390_d_n9 * s.v[3]);
        let eq37_e392_d_b0: f64 = (eq37_e390_d_b0 * s.v[3]);
        let eq37_e392_d_b1: f64 = (eq37_e390_d_b1 * s.v[3]);
        let eq37_e392_d_b2: f64 = (eq37_e390_d_b2 * s.v[3]);
        let eq37_e392_d_b3: f64 = (eq37_e390_d_b3 * s.v[3]);
        let eq37_e392_d_b4: f64 = (eq37_e390_d_b4 * s.v[3]);
        let eq37_e392_d_b5: f64 = (eq37_e390_d_b5 * s.v[3]);
        let eq37_e392_d_b6: f64 = (eq37_e390_d_b6 * s.v[3]);
        let eq37_e392_d_b7: f64 = (eq37_e390_d_b7 * s.v[3]);
        let eq37_e393_q: f64 = eq37_e392;
        let eq37_reactive_node_derivatives: [f64; 10] = [eq37_e392_d_n0, eq37_e392_d_n1, eq37_e392_d_n2, eq37_e392_d_n3, eq37_e392_d_n4, eq37_e392_d_n5, eq37_e392_d_n6, eq37_e392_d_n7, eq37_e392_d_n8, eq37_e392_d_n9];
        let eq37_reactive_branch_derivatives: [f64; 8] = [eq37_e392_d_b0, eq37_e392_d_b1, eq37_e392_d_b2, eq37_e392_d_b3, eq37_e392_d_b4, eq37_e392_d_b5, eq37_e392_d_b6, eq37_e392_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq37_reactive_node_derivatives,
            branches,
            &eq37_reactive_branch_derivatives,
            multiplicity,
        );
        let eq38_e396: f64 = (s.v[9] * s.v[60]);
        let eq38_e396_d_n0: f64 = (s.v[9] * s.dn[60][0]);
        let eq38_e396_d_n1: f64 = (s.v[9] * s.dn[60][1]);
        let eq38_e396_d_n2: f64 = (s.v[9] * s.dn[60][2]);
        let eq38_e396_d_n3: f64 = (s.v[9] * s.dn[60][3]);
        let eq38_e396_d_n4: f64 = (s.v[9] * s.dn[60][4]);
        let eq38_e396_d_n5: f64 = (s.v[9] * s.dn[60][5]);
        let eq38_e396_d_n6: f64 = (s.v[9] * s.dn[60][6]);
        let eq38_e396_d_n7: f64 = (s.v[9] * s.dn[60][7]);
        let eq38_e396_d_n8: f64 = (s.v[9] * s.dn[60][8]);
        let eq38_e396_d_n9: f64 = (s.v[9] * s.dn[60][9]);
        let eq38_e396_d_b0: f64 = (s.v[9] * s.db[60][0]);
        let eq38_e396_d_b1: f64 = (s.v[9] * s.db[60][1]);
        let eq38_e396_d_b2: f64 = (s.v[9] * s.db[60][2]);
        let eq38_e396_d_b3: f64 = (s.v[9] * s.db[60][3]);
        let eq38_e396_d_b4: f64 = (s.v[9] * s.db[60][4]);
        let eq38_e396_d_b5: f64 = (s.v[9] * s.db[60][5]);
        let eq38_e396_d_b6: f64 = (s.v[9] * s.db[60][6]);
        let eq38_e396_d_b7: f64 = (s.v[9] * s.db[60][7]);
        let eq38_e398: f64 = (eq38_e396 * s.v[3]);
        let eq38_e398_d_n0: f64 = (eq38_e396_d_n0 * s.v[3]);
        let eq38_e398_d_n1: f64 = (eq38_e396_d_n1 * s.v[3]);
        let eq38_e398_d_n2: f64 = (eq38_e396_d_n2 * s.v[3]);
        let eq38_e398_d_n3: f64 = (eq38_e396_d_n3 * s.v[3]);
        let eq38_e398_d_n4: f64 = (eq38_e396_d_n4 * s.v[3]);
        let eq38_e398_d_n5: f64 = (eq38_e396_d_n5 * s.v[3]);
        let eq38_e398_d_n6: f64 = (eq38_e396_d_n6 * s.v[3]);
        let eq38_e398_d_n7: f64 = (eq38_e396_d_n7 * s.v[3]);
        let eq38_e398_d_n8: f64 = (eq38_e396_d_n8 * s.v[3]);
        let eq38_e398_d_n9: f64 = (eq38_e396_d_n9 * s.v[3]);
        let eq38_e398_d_b0: f64 = (eq38_e396_d_b0 * s.v[3]);
        let eq38_e398_d_b1: f64 = (eq38_e396_d_b1 * s.v[3]);
        let eq38_e398_d_b2: f64 = (eq38_e396_d_b2 * s.v[3]);
        let eq38_e398_d_b3: f64 = (eq38_e396_d_b3 * s.v[3]);
        let eq38_e398_d_b4: f64 = (eq38_e396_d_b4 * s.v[3]);
        let eq38_e398_d_b5: f64 = (eq38_e396_d_b5 * s.v[3]);
        let eq38_e398_d_b6: f64 = (eq38_e396_d_b6 * s.v[3]);
        let eq38_e398_d_b7: f64 = (eq38_e396_d_b7 * s.v[3]);
        let eq38_e399_q: f64 = eq38_e398;
        let eq38_reactive_node_derivatives: [f64; 10] = [eq38_e398_d_n0, eq38_e398_d_n1, eq38_e398_d_n2, eq38_e398_d_n3, eq38_e398_d_n4, eq38_e398_d_n5, eq38_e398_d_n6, eq38_e398_d_n7, eq38_e398_d_n8, eq38_e398_d_n9];
        let eq38_reactive_branch_derivatives: [f64; 8] = [eq38_e398_d_b0, eq38_e398_d_b1, eq38_e398_d_b2, eq38_e398_d_b3, eq38_e398_d_b4, eq38_e398_d_b5, eq38_e398_d_b6, eq38_e398_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[4]),
            nodes,
            &eq38_reactive_node_derivatives,
            branches,
            &eq38_reactive_branch_derivatives,
            multiplicity,
        );
        let eq39_e402: f64 = (s.v[9] * s.v[62]);
        let eq39_e402_d_n0: f64 = (s.v[9] * s.dn[62][0]);
        let eq39_e402_d_n1: f64 = (s.v[9] * s.dn[62][1]);
        let eq39_e402_d_n2: f64 = (s.v[9] * s.dn[62][2]);
        let eq39_e402_d_n3: f64 = (s.v[9] * s.dn[62][3]);
        let eq39_e402_d_n4: f64 = (s.v[9] * s.dn[62][4]);
        let eq39_e402_d_n5: f64 = (s.v[9] * s.dn[62][5]);
        let eq39_e402_d_n6: f64 = (s.v[9] * s.dn[62][6]);
        let eq39_e402_d_n7: f64 = (s.v[9] * s.dn[62][7]);
        let eq39_e402_d_n8: f64 = (s.v[9] * s.dn[62][8]);
        let eq39_e402_d_n9: f64 = (s.v[9] * s.dn[62][9]);
        let eq39_e402_d_b0: f64 = (s.v[9] * s.db[62][0]);
        let eq39_e402_d_b1: f64 = (s.v[9] * s.db[62][1]);
        let eq39_e402_d_b2: f64 = (s.v[9] * s.db[62][2]);
        let eq39_e402_d_b3: f64 = (s.v[9] * s.db[62][3]);
        let eq39_e402_d_b4: f64 = (s.v[9] * s.db[62][4]);
        let eq39_e402_d_b5: f64 = (s.v[9] * s.db[62][5]);
        let eq39_e402_d_b6: f64 = (s.v[9] * s.db[62][6]);
        let eq39_e402_d_b7: f64 = (s.v[9] * s.db[62][7]);
        let eq39_e404: f64 = (eq39_e402 * s.v[3]);
        let eq39_e404_d_n0: f64 = (eq39_e402_d_n0 * s.v[3]);
        let eq39_e404_d_n1: f64 = (eq39_e402_d_n1 * s.v[3]);
        let eq39_e404_d_n2: f64 = (eq39_e402_d_n2 * s.v[3]);
        let eq39_e404_d_n3: f64 = (eq39_e402_d_n3 * s.v[3]);
        let eq39_e404_d_n4: f64 = (eq39_e402_d_n4 * s.v[3]);
        let eq39_e404_d_n5: f64 = (eq39_e402_d_n5 * s.v[3]);
        let eq39_e404_d_n6: f64 = (eq39_e402_d_n6 * s.v[3]);
        let eq39_e404_d_n7: f64 = (eq39_e402_d_n7 * s.v[3]);
        let eq39_e404_d_n8: f64 = (eq39_e402_d_n8 * s.v[3]);
        let eq39_e404_d_n9: f64 = (eq39_e402_d_n9 * s.v[3]);
        let eq39_e404_d_b0: f64 = (eq39_e402_d_b0 * s.v[3]);
        let eq39_e404_d_b1: f64 = (eq39_e402_d_b1 * s.v[3]);
        let eq39_e404_d_b2: f64 = (eq39_e402_d_b2 * s.v[3]);
        let eq39_e404_d_b3: f64 = (eq39_e402_d_b3 * s.v[3]);
        let eq39_e404_d_b4: f64 = (eq39_e402_d_b4 * s.v[3]);
        let eq39_e404_d_b5: f64 = (eq39_e402_d_b5 * s.v[3]);
        let eq39_e404_d_b6: f64 = (eq39_e402_d_b6 * s.v[3]);
        let eq39_e404_d_b7: f64 = (eq39_e402_d_b7 * s.v[3]);
        let eq39_e405_q: f64 = eq39_e404;
        let eq39_reactive_node_derivatives: [f64; 10] = [eq39_e404_d_n0, eq39_e404_d_n1, eq39_e404_d_n2, eq39_e404_d_n3, eq39_e404_d_n4, eq39_e404_d_n5, eq39_e404_d_n6, eq39_e404_d_n7, eq39_e404_d_n8, eq39_e404_d_n9];
        let eq39_reactive_branch_derivatives: [f64; 8] = [eq39_e404_d_b0, eq39_e404_d_b1, eq39_e404_d_b2, eq39_e404_d_b3, eq39_e404_d_b4, eq39_e404_d_b5, eq39_e404_d_b6, eq39_e404_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes,
            &eq39_reactive_node_derivatives,
            branches,
            &eq39_reactive_branch_derivatives,
            multiplicity,
        );
        let eq40_e408: f64 = (s.v[9] * s.v[56]);
        let eq40_e408_d_n0: f64 = (s.v[9] * s.dn[56][0]);
        let eq40_e408_d_n1: f64 = (s.v[9] * s.dn[56][1]);
        let eq40_e408_d_n2: f64 = (s.v[9] * s.dn[56][2]);
        let eq40_e408_d_n3: f64 = (s.v[9] * s.dn[56][3]);
        let eq40_e408_d_n4: f64 = (s.v[9] * s.dn[56][4]);
        let eq40_e408_d_n5: f64 = (s.v[9] * s.dn[56][5]);
        let eq40_e408_d_n6: f64 = (s.v[9] * s.dn[56][6]);
        let eq40_e408_d_n7: f64 = (s.v[9] * s.dn[56][7]);
        let eq40_e408_d_n8: f64 = (s.v[9] * s.dn[56][8]);
        let eq40_e408_d_n9: f64 = (s.v[9] * s.dn[56][9]);
        let eq40_e408_d_b0: f64 = (s.v[9] * s.db[56][0]);
        let eq40_e408_d_b1: f64 = (s.v[9] * s.db[56][1]);
        let eq40_e408_d_b2: f64 = (s.v[9] * s.db[56][2]);
        let eq40_e408_d_b3: f64 = (s.v[9] * s.db[56][3]);
        let eq40_e408_d_b4: f64 = (s.v[9] * s.db[56][4]);
        let eq40_e408_d_b5: f64 = (s.v[9] * s.db[56][5]);
        let eq40_e408_d_b6: f64 = (s.v[9] * s.db[56][6]);
        let eq40_e408_d_b7: f64 = (s.v[9] * s.db[56][7]);
        let eq40_e410: f64 = (eq40_e408 * s.v[3]);
        let eq40_e410_d_n0: f64 = (eq40_e408_d_n0 * s.v[3]);
        let eq40_e410_d_n1: f64 = (eq40_e408_d_n1 * s.v[3]);
        let eq40_e410_d_n2: f64 = (eq40_e408_d_n2 * s.v[3]);
        let eq40_e410_d_n3: f64 = (eq40_e408_d_n3 * s.v[3]);
        let eq40_e410_d_n4: f64 = (eq40_e408_d_n4 * s.v[3]);
        let eq40_e410_d_n5: f64 = (eq40_e408_d_n5 * s.v[3]);
        let eq40_e410_d_n6: f64 = (eq40_e408_d_n6 * s.v[3]);
        let eq40_e410_d_n7: f64 = (eq40_e408_d_n7 * s.v[3]);
        let eq40_e410_d_n8: f64 = (eq40_e408_d_n8 * s.v[3]);
        let eq40_e410_d_n9: f64 = (eq40_e408_d_n9 * s.v[3]);
        let eq40_e410_d_b0: f64 = (eq40_e408_d_b0 * s.v[3]);
        let eq40_e410_d_b1: f64 = (eq40_e408_d_b1 * s.v[3]);
        let eq40_e410_d_b2: f64 = (eq40_e408_d_b2 * s.v[3]);
        let eq40_e410_d_b3: f64 = (eq40_e408_d_b3 * s.v[3]);
        let eq40_e410_d_b4: f64 = (eq40_e408_d_b4 * s.v[3]);
        let eq40_e410_d_b5: f64 = (eq40_e408_d_b5 * s.v[3]);
        let eq40_e410_d_b6: f64 = (eq40_e408_d_b6 * s.v[3]);
        let eq40_e410_d_b7: f64 = (eq40_e408_d_b7 * s.v[3]);
        let eq40_e411_q: f64 = eq40_e410;
        let eq40_reactive_node_derivatives: [f64; 10] = [eq40_e410_d_n0, eq40_e410_d_n1, eq40_e410_d_n2, eq40_e410_d_n3, eq40_e410_d_n4, eq40_e410_d_n5, eq40_e410_d_n6, eq40_e410_d_n7, eq40_e410_d_n8, eq40_e410_d_n9];
        let eq40_reactive_branch_derivatives: [f64; 8] = [eq40_e410_d_b0, eq40_e410_d_b1, eq40_e410_d_b2, eq40_e410_d_b3, eq40_e410_d_b4, eq40_e410_d_b5, eq40_e410_d_b6, eq40_e410_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes,
            &eq40_reactive_node_derivatives,
            branches,
            &eq40_reactive_branch_derivatives,
            multiplicity,
        );
        let eq41_e414: f64 = (s.v[9] * s.v[57]);
        let eq41_e414_d_n0: f64 = (s.v[9] * s.dn[57][0]);
        let eq41_e414_d_n1: f64 = (s.v[9] * s.dn[57][1]);
        let eq41_e414_d_n2: f64 = (s.v[9] * s.dn[57][2]);
        let eq41_e414_d_n3: f64 = (s.v[9] * s.dn[57][3]);
        let eq41_e414_d_n4: f64 = (s.v[9] * s.dn[57][4]);
        let eq41_e414_d_n5: f64 = (s.v[9] * s.dn[57][5]);
        let eq41_e414_d_n6: f64 = (s.v[9] * s.dn[57][6]);
        let eq41_e414_d_n7: f64 = (s.v[9] * s.dn[57][7]);
        let eq41_e414_d_n8: f64 = (s.v[9] * s.dn[57][8]);
        let eq41_e414_d_n9: f64 = (s.v[9] * s.dn[57][9]);
        let eq41_e414_d_b0: f64 = (s.v[9] * s.db[57][0]);
        let eq41_e414_d_b1: f64 = (s.v[9] * s.db[57][1]);
        let eq41_e414_d_b2: f64 = (s.v[9] * s.db[57][2]);
        let eq41_e414_d_b3: f64 = (s.v[9] * s.db[57][3]);
        let eq41_e414_d_b4: f64 = (s.v[9] * s.db[57][4]);
        let eq41_e414_d_b5: f64 = (s.v[9] * s.db[57][5]);
        let eq41_e414_d_b6: f64 = (s.v[9] * s.db[57][6]);
        let eq41_e414_d_b7: f64 = (s.v[9] * s.db[57][7]);
        let eq41_e416: f64 = (eq41_e414 * s.v[3]);
        let eq41_e416_d_n0: f64 = (eq41_e414_d_n0 * s.v[3]);
        let eq41_e416_d_n1: f64 = (eq41_e414_d_n1 * s.v[3]);
        let eq41_e416_d_n2: f64 = (eq41_e414_d_n2 * s.v[3]);
        let eq41_e416_d_n3: f64 = (eq41_e414_d_n3 * s.v[3]);
        let eq41_e416_d_n4: f64 = (eq41_e414_d_n4 * s.v[3]);
        let eq41_e416_d_n5: f64 = (eq41_e414_d_n5 * s.v[3]);
        let eq41_e416_d_n6: f64 = (eq41_e414_d_n6 * s.v[3]);
        let eq41_e416_d_n7: f64 = (eq41_e414_d_n7 * s.v[3]);
        let eq41_e416_d_n8: f64 = (eq41_e414_d_n8 * s.v[3]);
        let eq41_e416_d_n9: f64 = (eq41_e414_d_n9 * s.v[3]);
        let eq41_e416_d_b0: f64 = (eq41_e414_d_b0 * s.v[3]);
        let eq41_e416_d_b1: f64 = (eq41_e414_d_b1 * s.v[3]);
        let eq41_e416_d_b2: f64 = (eq41_e414_d_b2 * s.v[3]);
        let eq41_e416_d_b3: f64 = (eq41_e414_d_b3 * s.v[3]);
        let eq41_e416_d_b4: f64 = (eq41_e414_d_b4 * s.v[3]);
        let eq41_e416_d_b5: f64 = (eq41_e414_d_b5 * s.v[3]);
        let eq41_e416_d_b6: f64 = (eq41_e414_d_b6 * s.v[3]);
        let eq41_e416_d_b7: f64 = (eq41_e414_d_b7 * s.v[3]);
        let eq41_e417_q: f64 = eq41_e416;
        let eq41_reactive_node_derivatives: [f64; 10] = [eq41_e416_d_n0, eq41_e416_d_n1, eq41_e416_d_n2, eq41_e416_d_n3, eq41_e416_d_n4, eq41_e416_d_n5, eq41_e416_d_n6, eq41_e416_d_n7, eq41_e416_d_n8, eq41_e416_d_n9];
        let eq41_reactive_branch_derivatives: [f64; 8] = [eq41_e416_d_b0, eq41_e416_d_b1, eq41_e416_d_b2, eq41_e416_d_b3, eq41_e416_d_b4, eq41_e416_d_b5, eq41_e416_d_b6, eq41_e416_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[4]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e419: f64 = (-s.v[63]);
        let eq42_e419_d_n0: f64 = (-s.dn[63][0]);
        let eq42_e419_d_n1: f64 = (-s.dn[63][1]);
        let eq42_e419_d_n2: f64 = (-s.dn[63][2]);
        let eq42_e419_d_n3: f64 = (-s.dn[63][3]);
        let eq42_e419_d_n4: f64 = (-s.dn[63][4]);
        let eq42_e419_d_n5: f64 = (-s.dn[63][5]);
        let eq42_e419_d_n6: f64 = (-s.dn[63][6]);
        let eq42_e419_d_n7: f64 = (-s.dn[63][7]);
        let eq42_e419_d_n8: f64 = (-s.dn[63][8]);
        let eq42_e419_d_n9: f64 = (-s.dn[63][9]);
        let eq42_e419_d_b0: f64 = (-s.db[63][0]);
        let eq42_e419_d_b1: f64 = (-s.db[63][1]);
        let eq42_e419_d_b2: f64 = (-s.db[63][2]);
        let eq42_e419_d_b3: f64 = (-s.db[63][3]);
        let eq42_e419_d_b4: f64 = (-s.db[63][4]);
        let eq42_e419_d_b5: f64 = (-s.db[63][5]);
        let eq42_e419_d_b6: f64 = (-s.db[63][6]);
        let eq42_e419_d_b7: f64 = (-s.db[63][7]);
        let eq42_e421: f64 = (eq42_e419 * s.v[3]);
        let eq42_e421_d_n0: f64 = (eq42_e419_d_n0 * s.v[3]);
        let eq42_e421_d_n1: f64 = (eq42_e419_d_n1 * s.v[3]);
        let eq42_e421_d_n2: f64 = (eq42_e419_d_n2 * s.v[3]);
        let eq42_e421_d_n3: f64 = (eq42_e419_d_n3 * s.v[3]);
        let eq42_e421_d_n4: f64 = (eq42_e419_d_n4 * s.v[3]);
        let eq42_e421_d_n5: f64 = (eq42_e419_d_n5 * s.v[3]);
        let eq42_e421_d_n6: f64 = (eq42_e419_d_n6 * s.v[3]);
        let eq42_e421_d_n7: f64 = (eq42_e419_d_n7 * s.v[3]);
        let eq42_e421_d_n8: f64 = (eq42_e419_d_n8 * s.v[3]);
        let eq42_e421_d_n9: f64 = (eq42_e419_d_n9 * s.v[3]);
        let eq42_e421_d_b0: f64 = (eq42_e419_d_b0 * s.v[3]);
        let eq42_e421_d_b1: f64 = (eq42_e419_d_b1 * s.v[3]);
        let eq42_e421_d_b2: f64 = (eq42_e419_d_b2 * s.v[3]);
        let eq42_e421_d_b3: f64 = (eq42_e419_d_b3 * s.v[3]);
        let eq42_e421_d_b4: f64 = (eq42_e419_d_b4 * s.v[3]);
        let eq42_e421_d_b5: f64 = (eq42_e419_d_b5 * s.v[3]);
        let eq42_e421_d_b6: f64 = (eq42_e419_d_b6 * s.v[3]);
        let eq42_e421_d_b7: f64 = (eq42_e419_d_b7 * s.v[3]);
        let eq42_e422_q: f64 = eq42_e421;
        let eq42_reactive_node_derivatives: [f64; 10] = [eq42_e421_d_n0, eq42_e421_d_n1, eq42_e421_d_n2, eq42_e421_d_n3, eq42_e421_d_n4, eq42_e421_d_n5, eq42_e421_d_n6, eq42_e421_d_n7, eq42_e421_d_n8, eq42_e421_d_n9];
        let eq42_reactive_branch_derivatives: [f64; 8] = [eq42_e421_d_b0, eq42_e421_d_b1, eq42_e421_d_b2, eq42_e421_d_b3, eq42_e421_d_b4, eq42_e421_d_b5, eq42_e421_d_b6, eq42_e421_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let eq43_e425: f64 = (s.v[63] * s.v[3]);
        let eq43_e425_d_n0: f64 = (s.dn[63][0] * s.v[3]);
        let eq43_e425_d_n1: f64 = (s.dn[63][1] * s.v[3]);
        let eq43_e425_d_n2: f64 = (s.dn[63][2] * s.v[3]);
        let eq43_e425_d_n3: f64 = (s.dn[63][3] * s.v[3]);
        let eq43_e425_d_n4: f64 = (s.dn[63][4] * s.v[3]);
        let eq43_e425_d_n5: f64 = (s.dn[63][5] * s.v[3]);
        let eq43_e425_d_n6: f64 = (s.dn[63][6] * s.v[3]);
        let eq43_e425_d_n7: f64 = (s.dn[63][7] * s.v[3]);
        let eq43_e425_d_n8: f64 = (s.dn[63][8] * s.v[3]);
        let eq43_e425_d_n9: f64 = (s.dn[63][9] * s.v[3]);
        let eq43_e425_d_b0: f64 = (s.db[63][0] * s.v[3]);
        let eq43_e425_d_b1: f64 = (s.db[63][1] * s.v[3]);
        let eq43_e425_d_b2: f64 = (s.db[63][2] * s.v[3]);
        let eq43_e425_d_b3: f64 = (s.db[63][3] * s.v[3]);
        let eq43_e425_d_b4: f64 = (s.db[63][4] * s.v[3]);
        let eq43_e425_d_b5: f64 = (s.db[63][5] * s.v[3]);
        let eq43_e425_d_b6: f64 = (s.db[63][6] * s.v[3]);
        let eq43_e425_d_b7: f64 = (s.db[63][7] * s.v[3]);
        let eq43_e426_q: f64 = eq43_e425;
        let eq43_reactive_node_derivatives: [f64; 10] = [eq43_e425_d_n0, eq43_e425_d_n1, eq43_e425_d_n2, eq43_e425_d_n3, eq43_e425_d_n4, eq43_e425_d_n5, eq43_e425_d_n6, eq43_e425_d_n7, eq43_e425_d_n8, eq43_e425_d_n9];
        let eq43_reactive_branch_derivatives: [f64; 8] = [eq43_e425_d_b0, eq43_e425_d_b1, eq43_e425_d_b2, eq43_e425_d_b3, eq43_e425_d_b4, eq43_e425_d_b5, eq43_e425_d_b6, eq43_e425_d_b7];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
