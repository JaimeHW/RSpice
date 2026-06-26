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
        s.v[199] = (11.7 * 8.8541879239442e-12);

        s.v[157] = 0.0;

        s.v[6] = 0.0;

        s.v[175] = 0.0;

        s.v[31] = (s.v[199] / p.p13);

        s.v[34] = (((s.v[31] * p.p14)) as f64).sqrt();

        s.v[35] = (s.v[34] * p.p25);

        s.v[32] = ((3.0 * s.v[31]) * p.p28);

        s.v[33] = (s.v[31] * p.p29);

        s.v[36] = (p.p35 + p.p35);

        s.v[37] = (p.p13 / (s.v[199] * p.p22));

        s.v[182] = ((p.p30 + p.p30) / p.p13);

        s.v[39] = (if (p.p0 > 0.0) { 0.5 } else { 0.3333333333333 });

        s.b[238] = (p.p3 == (-(-1e21)));
        s.v[238] = if s.b[238] { 1.0 } else { 0.0 };

        if s.b[238] {
            s.store_scalar(49, (ctx_temp + p.p2));
        }

        if (!s.b[238]) {
            s.store_scalar(49, (p.p3 + 273.15));
        }

        s.b[239] = (p.p4 == (-(-1e21)));
        s.v[239] = if s.b[239] { 1.0 } else { 0.0 };

        if s.b[239] {
            s.store_scalar(55, (25.0 + 273.15));
        }

        if (!s.b[239]) {
            s.store_scalar(55, (p.p4 + 273.15));
        }

        s.store_scale(17, 49, THERMAL_VOLTAGE_PER_K);

        s.store_scale(25, 17, 0.1);

        s.store_div_from_scalar(24, 1.0, 17);

        s.store_scale(26, 17, 2.0);

        s.store_scale(27, 26, 2.0);

        s.store_square(28, 17);

        s.store_scale(29, 28, 2.0);

        s.store_scale(30, 28, 16.0);

        s.store_sub_from_scalar_ad(51, 1.16, A::div(A::mul_scaled_lhs(s.ad_value(49), 0.000702, s.ad_value(49)), A::offset(s.ad_value(49), 1108.0)));

        s.store_sub_from_scalar_ad(52, 1.16, A::div(A::mul_scaled_lhs(s.ad_value(55), 0.000702, s.ad_value(55)), A::offset(s.ad_value(55), 1108.0)));

        s.store_sub(53, 49, 55);

        s.store_div(54, 49, 55);

        s.store_sub_from_scalar_ad(56, p.p15, A::scale(s.ad_value(53), p.p16));

        s.store_scale_ad(58, A::powf(s.ad_value(54), p.p20), p.p19);

        s.store_scale_ad(59, A::powf(s.ad_value(54), p.p24), p.p23);

        s.store_offset_scaled(60, 53, ((p.p34) * (p.p33)), p.p33);

        s.store_add_ad_lhs(61, A::sub(A::sub_scaled_inputs(s.ad_value(54), p.p18, A::mul_scaled_lhs(s.ad_value(17), 3.0, A::ln(s.ad_value(54))), 1.0), A::mul(s.ad_value(52), s.ad_value(54))), 51);

        s.v[0] = 0.2;

        s.store_offset(1, 61, (-s.v[0]));

        s.store_offset_ad(61, A::add_scaled_inputs(s.ad_value(1), 0.5, A::sqrt(A::add(A::square(s.ad_value(1)), A::square(s.ad_value(17)))), 0.5), s.v[0]);

        s.store_sqrt(71, 61);

        s.store_div_from_scalar(40, 1.0, 59);

        s.store_scale(41, 59, s.v[34]);

        s.store_scale(42, 60, s.v[34]);

        s.store_div_from_scalar(43, p.p32, 60);

        s.v[191] = (p.p5 + p.p26);

        s.v[192] = (p.p6 + p.p27);

        s.store_scale(158, 59, s.v[191]);

        s.store_mul_offset_ad_rhs(173, 17, A::ln(A::mul_scaled_lhs(s.ad_value(158), 0.5, s.ad_value(24))), (-0.6));

        s.v[48] = (1.0 / (((s.v[192] * s.v[191])) as f64).sqrt());

        s.b[240] = (p.p0 > 0.0);
        s.v[240] = if s.b[240] { 1.0 } else { 0.0 };

        if s.b[240] {
            s.store_ad_value(57, {
                if (p.p38 != 1e-6) {
                    A::offset(s.ad_value(56), (s.v[48] * (p.p38 - 1e-6)))
                } else {
                    s.ad_value(56)
                }
            });
        }

        if (!s.b[240]) {
            s.store_ad_value(57, {
                if (p.p38 != 1e-6) {
                    A::sub_from_scalar((s.v[48] * (1e-6 - p.p38)), s.ad_value(56))
                } else {
                    A::neg(s.ad_value(56))
                }
            });
        }

        s.store_scale_ad(50, {
            if (p.p39 != 1e-6) {
                A::scale(s.ad_value(58), (1.0 + ((p.p39 - 1e-6) * s.v[48])))
            } else {
                s.ad_value(58)
            }
        }, s.v[192]);

        s.v[62] = (if (p.p40 != 1e-6) { (p.p17 + ((p.p40 - 1e-6) * s.v[48])) } else { p.p17 });

        s.store_scale(153, 71, s.v[62]);

        s.b[241] = (s.v[182] == 0.0);
        s.v[241] = if s.b[241] { 1.0 } else { 0.0 };

        if s.b[241] {
            s.store_scalar(183, 0.0);
        }

        if (!s.b[241]) {
            s.store_scalar(184, (0.28 * ((s.v[191] / (p.p31 * p.p8)) - 0.1)));
            s.store_div_from_scalar_offset_ad(242, 1.0, A::add_scaled_inputs(s.ad_value(184), 0.5, A::sqrt(A::offset(A::square(s.ad_value(184)), 0.001936)), 0.5), 1.0);
            s.store_scaled_mul(183, 242, 242, s.v[182]);
        }

        s.store_scaled_voltage(145, ctx, nodes, Some(1), Some(3), p.p0);

        s.store_scaled_voltage(147, ctx, nodes, Some(2), Some(3), p.p0);

        s.store_scaled_voltage(146, ctx, nodes, Some(0), Some(3), p.p0);

        s.b[243] = ((s.v[146] - s.v[147]) < 0.0);
        s.v[243] = if s.b[243] { 1.0 } else { 0.0 };

        if s.b[243] {
            s.store_scalar(44, (-1.0));
            s.copy_ad(38, 147);
            s.copy_ad(147, 146);
            s.copy_ad(146, 38);
        }

        if (!s.b[243]) {
            s.store_scalar(44, 1.0);
        }

        s.store_add_ad_lhs(143, A::add(A::sub(A::sub(s.ad_value(145), s.ad_value(57)), s.ad_value(183)), s.ad_value(61)), 153);

        s.store_sqrt_ad(144, A::add_scaled_inputs(A::square(s.ad_value(143)), 1.0, s.ad_value(30), 2.0));

        s.store_scaled_add(3, 143, 144, 0.5);

        s.store_add(70, 61, 147);

        s.store_sqrt_square_add(76, 70, 30);

        s.store_sqrt_scaled_ad(74, A::add(s.ad_value(70), s.ad_value(76)), 0.5);

        s.store_add(69, 61, 146);

        s.store_sqrt_square_add(75, 69, 30);

        s.store_sqrt_scaled_ad(73, A::add(s.ad_value(69), s.ad_value(75)), 0.5);

        s.v[45] = ((s.v[32] * p.p7) / s.v[192]);

        s.v[46] = ((s.v[33] * p.p8) / s.v[191]);

        s.store_sqrt_offset_input(67, 3, ((0.25 * s.v[62]) * s.v[62]));

        s.store_sub_ad(68, A::sub(s.ad_value(3), s.ad_value(61)), A::scaled_offset(s.ad_value(67), (-(0.5 * s.v[62])), s.v[62]));

        s.store_sqrt_add_ad(174, A::add(s.ad_value(68), s.ad_value(61)), s.ad_value(25));

        s.store_add_scaled_ad_lhs(64, A::sub_from_scalar(s.v[62], A::add_scaled_inputs(s.ad_value(74), s.v[46], s.ad_value(73), s.v[46])), 174, s.v[45]);

        s.store_sqrt_square_add(65, 64, 25);

        s.store_scaled_add(4, 64, 65, 0.5);

        s.store_sqrt_add_ad(66, s.ad_value(3), A::mul_scaled_lhs(s.ad_value(4), 0.25, s.ad_value(4)));

        s.store_sub_ad(5, A::sub(s.ad_value(3), s.ad_value(61)), A::mul(s.ad_value(4), A::sub_scaled_inputs(s.ad_value(66), 1.0, s.ad_value(4), 0.5)));

        s.store_mul_sub_lhs(0, 5, 147, 24);

        s.b[244] = (s.v[0] > (-0.35));
        s.v[244] = if s.b[244] { 1.0 } else { 0.0 };

        if s.b[244] {
            s.store_div_from_scalar_sub_ad(196, 2.0, A::offset(s.ad_value(0), 1.3), A::ln(A::offset(s.ad_value(0), 1.6)));
            s.store_div_ad(197, A::offset(s.ad_value(196), 2.0), A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(196))));
            s.store_div_ad(195, A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(197))), A::offset(s.ad_value(197), 2.0));
        }

        s.b[245] = (s.v[0] > (-15.0));
        s.v[245] = if s.b[245] { 1.0 } else { 0.0 };

        if ((!s.b[244]) && s.b[245]) {
            s.store_offset_ad(196, A::exp_scaled_input(s.ad_value(0), -1.0), 1.55);
            s.store_div_ad(197, A::offset(s.ad_value(196), 2.0), A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(196))));
            s.store_div_ad(195, A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(197))), A::offset(s.ad_value(197), 2.0));
        }

        s.b[246] = (s.v[0] > (-23.0));
        s.v[246] = if s.b[246] { 1.0 } else { 0.0 };

        if (((!s.b[244]) && (!s.b[245])) && s.b[246]) {
            s.store_div_from_scalar_offset_ad(195, 1.0, A::exp_scaled_input(s.ad_value(0), -1.0), 2.0);
        }

        if (((!s.b[244]) && (!s.b[245])) && (!s.b[246])) {
            s.store_offset_exp(195, 0, 1e-64);
        }

        s.store_mul_offset_rhs(7, 195, 195, 1.0);

        s.store_sqrt(87, 7);

        s.copy_ad(90, 195);

        s.store_div(160, 17, 158);

        s.store_sqrt_offset_ad(80, A::mul(s.ad_value(87), s.ad_value(160)), 0.25);

        s.store_mul_offset_rhs(10, 158, 80, (-0.5));

        s.store_scaled_sub(77, 146, 147, 0.5);

        s.store_mul_offset_ad_rhs(78, 30, A::sub_scaled_inputs(s.ad_value(87), p.p25, A::mul(s.ad_value(10), s.ad_value(24)), p.p25), 0.015625);

        s.store_sqrt_square_add(81, 10, 78);

        s.store_sqrt_add_ad(82, A::mul(A::sub(s.ad_value(77), s.ad_value(10)), A::sub(s.ad_value(77), s.ad_value(10))), s.ad_value(78));

        s.store_sub(79, 81, 82);

        s.store_sqrt_offset_ad(83, A::mul(A::sub_scaled_inputs(s.ad_value(87), 1.0, A::ln(s.ad_value(7)), 0.75), s.ad_value(160)), 0.25);

        s.store_add_ad_lhs(11, A::mul(s.ad_value(158), A::offset(s.ad_value(83), (-0.5))), 173);

        s.store_sub(159, 77, 11);

        s.store_sqrt_square_add(84, 11, 78);

        s.store_sqrt_square_add(85, 159, 78);

        s.store_mul_add_ad_lhs(0, A::sub(A::sub(A::sub(s.ad_value(5), s.ad_value(77)), s.ad_value(147)), s.ad_value(84)), s.ad_value(85), 24);

        s.b[247] = (s.v[0] > (-0.35));
        s.v[247] = if s.b[247] { 1.0 } else { 0.0 };

        if s.b[247] {
            s.store_div_from_scalar_sub_ad(196, 2.0, A::offset(s.ad_value(0), 1.3), A::ln(A::offset(s.ad_value(0), 1.6)));
            s.store_div_ad(197, A::offset(s.ad_value(196), 2.0), A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(196))));
            s.store_div_ad(195, A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(197))), A::offset(s.ad_value(197), 2.0));
        }

        s.b[248] = (s.v[0] > (-15.0));
        s.v[248] = if s.b[248] { 1.0 } else { 0.0 };

        if ((!s.b[247]) && s.b[248]) {
            s.store_offset_ad(196, A::exp_scaled_input(s.ad_value(0), -1.0), 1.55);
            s.store_div_ad(197, A::offset(s.ad_value(196), 2.0), A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(196))));
            s.store_div_ad(195, A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(197))), A::offset(s.ad_value(197), 2.0));
        }

        s.b[249] = (s.v[0] > (-23.0));
        s.v[249] = if s.b[249] { 1.0 } else { 0.0 };

        if (((!s.b[247]) && (!s.b[248])) && s.b[249]) {
            s.store_div_from_scalar_offset_ad(195, 1.0, A::exp_scaled_input(s.ad_value(0), -1.0), 2.0);
        }

        if (((!s.b[247]) && (!s.b[248])) && (!s.b[249])) {
            s.store_offset_exp(195, 0, 1e-64);
        }

        s.store_mul_offset_rhs(9, 195, 195, 1.0);

        s.copy_ad(92, 195);

        s.store_scaled_ln_ad(12, A::offset(A::div(A::sub(s.ad_value(77), s.ad_value(79)), s.ad_value(41)), 1.0), s.v[35]);

        s.store_add_ad(155, A::sub_from_scalar(s.v[191], s.ad_value(12)), A::mul(A::add(s.ad_value(77), s.ad_value(79)), s.ad_value(40)));

        s.v[154] = (0.1 * s.v[191]);

        s.store_sqrt_square_offset(63, 155, (s.v[154] * s.v[154]));

        s.store_scaled_add(13, 155, 63, 0.5);

        s.store_mul_sub_lhs(0, 5, 146, 24);

        s.b[250] = (s.v[0] > (-0.35));
        s.v[250] = if s.b[250] { 1.0 } else { 0.0 };

        if s.b[250] {
            s.store_div_from_scalar_sub_ad(196, 2.0, A::offset(s.ad_value(0), 1.3), A::ln(A::offset(s.ad_value(0), 1.6)));
            s.store_div_ad(197, A::offset(s.ad_value(196), 2.0), A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(196))));
            s.store_div_ad(195, A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(197))), A::offset(s.ad_value(197), 2.0));
        }

        s.b[251] = (s.v[0] > (-15.0));
        s.v[251] = if s.b[251] { 1.0 } else { 0.0 };

        if ((!s.b[250]) && s.b[251]) {
            s.store_offset_ad(196, A::exp_scaled_input(s.ad_value(0), -1.0), 1.55);
            s.store_div_ad(197, A::offset(s.ad_value(196), 2.0), A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(196))));
            s.store_div_ad(195, A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(197))), A::offset(s.ad_value(197), 2.0));
        }

        s.b[252] = (s.v[0] > (-23.0));
        s.v[252] = if s.b[252] { 1.0 } else { 0.0 };

        if (((!s.b[250]) && (!s.b[251])) && s.b[252]) {
            s.store_div_from_scalar_offset_ad(195, 1.0, A::exp_scaled_input(s.ad_value(0), -1.0), 2.0);
        }

        if (((!s.b[250]) && (!s.b[251])) && (!s.b[252])) {
            s.store_offset_exp(195, 0, 1e-64);
        }

        s.store_mul_offset_rhs(8, 195, 195, 1.0);

        s.copy_ad(91, 195);

        s.store_offset(95, 7, 0.25);

        s.store_offset(96, 8, 0.25);

        s.store_sqrt(93, 95);

        s.store_sqrt(94, 96);

        s.store_mul_ad(99, A::add(s.ad_value(93), s.ad_value(94)), A::add(s.ad_value(93), s.ad_value(94)));

        s.store_offset_add(107, 5, 61, 1e-6);

        s.store_scaled_sqrt(108, 107, 2.0);

        s.store_div_from_scalar(111, s.v[62], 108);

        s.store_div_from_scalar_offset_input(112, s.v[62], 108, s.v[62]);

        s.store_mul_ad(100, A::mul_scaled_lhs(A::offset(s.ad_value(111), 1.0), -1.0, s.ad_value(17)), A::offset(A::div(A::add_scaled_inputs(A::add(s.ad_value(96), A::mul(s.ad_value(94), s.ad_value(93))), (0.66666666 + 0.66666666), s.ad_value(95), (0.66666666 + 0.66666666)), A::add(s.ad_value(93), s.ad_value(94))), (-1.0)));

        s.store_sub_scaled_ad_rhs(101, 108, ((-0.5) * s.v[62]), A::mul(s.ad_value(112), s.ad_value(100)));

        s.b[253] = (p.p22 == 0.0);
        s.v[253] = if s.b[253] { 1.0 } else { 0.0 };

        if s.b[253] {
            s.store_sqrt_square_add(175, 5, 29);
            s.store_scaled_add(6, 5, 175, 0.5);
            s.store_offset_scaled(157, 6, p.p21, 1.0);
            s.store_div_ad_rhs(14, 50, A::mul(s.ad_value(13), s.ad_value(157)));
        }

        s.b[254] = ((s.v[101] + (s.v[39] * s.v[100])) > 0.0);
        s.v[254] = if s.b[254] { 1.0 } else { 0.0 };

        if ((!s.b[253]) && s.b[254]) {
            s.store_offset_ad(47, A::add_scaled_inputs(s.ad_value(101), s.v[37], s.ad_value(100), (s.v[39] * s.v[37])), 1.0);
        }

        if ((!s.b[253]) && (!s.b[254])) {
            s.store_sub_from_scalar_ad(47, 1.0, A::add_scaled_inputs(s.ad_value(101), s.v[37], s.ad_value(100), (s.v[39] * s.v[37])));
        }

        if (!s.b[253]) {
            s.store_offset_scaled(156, 153, s.v[37], 1.0);
            s.store_div_ad(14, A::mul(s.ad_value(50), s.ad_value(156)), A::mul(s.ad_value(13), s.ad_value(47)));
        }

        s.store_sqrt_add_ad(72, A::add(s.ad_value(61), s.ad_value(5)), s.ad_value(27));

        s.store_offset_div_from_scalar_ad(15, s.v[62], A::scale(s.ad_value(72), 2.0), 1.0);

        s.store_sub(86, 7, 9);

        s.store_mul3_lhs(16, 29, 15, 14);

        s.store_mul(150, 16, 86);

        s.store_mul_abs_rhs(152, 14, 100);

        s.store_scaled_div(0, 4, 65, (1.0 / (2.0)));

        s.store_div(1, 3, 144);

        s.store_div_ad_lhs(161, A::mul_scaled_lhs(s.ad_value(0), (-s.v[46]), s.ad_value(73)), 75);

        s.store_div_ad_lhs(163, A::mul_scaled_lhs(s.ad_value(0), (-s.v[46]), s.ad_value(74)), 76);

        s.store_mul_div_ad_lhs(162, A::mul_scaled_lhs(s.ad_value(0), s.v[45], A::offset(s.ad_value(67), (-(0.5 * s.v[62])))), A::mul(s.ad_value(67), s.ad_value(174)), 1);

        s.store_div_ad_lhs(2, A::add(s.ad_value(5), s.ad_value(61)), 66);

        s.store_mul_neg_lhs(113, 2, 161);

        s.store_mul_neg_lhs(115, 2, 163);

        s.store_add_ad(114, A::mul_scaled_lhs(s.ad_value(2), -1.0, s.ad_value(162)), A::mul(A::sub_from_scalar(1.0, A::div(s.ad_value(4), A::scale(s.ad_value(66), 2.0))), s.ad_value(1)));

        s.store_mul(0, 90, 24);

        s.store_mul(116, 0, 113);

        s.store_mul_offset_rhs(117, 0, 115, (-1.0));

        s.store_mul(118, 0, 114);

        s.store_div_ad_rhs(0, 17, A::mul_scaled_lhs(s.ad_value(80), 4.0, s.ad_value(87)));

        s.store_mul(122, 0, 116);

        s.store_mul(124, 0, 117);

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        s.store_mul(123, 0, 118);

        s.store_scale(0, 27, (2.0 * p.p25));

        s.store_scaled_div(1, 17, 87, (1.0 / (2.0)));

        s.store_mul_sub_ad_rhs(125, 0, A::mul(s.ad_value(116), s.ad_value(1)), s.ad_value(122));

        s.store_mul_sub_ad_rhs(127, 0, A::mul(s.ad_value(117), s.ad_value(1)), s.ad_value(124));

        s.store_mul_sub_ad_rhs(126, 0, A::mul(s.ad_value(118), s.ad_value(1)), s.ad_value(123));

        s.store_div_from_scalar(0, 1.0, 81);

        s.store_div_from_scalar(1, 1.0, 82);

        s.store_sub(2, 77, 10);

        s.store_sub_ad(128, A::mul(A::add(A::mul(s.ad_value(10), s.ad_value(122)), s.ad_value(125)), s.ad_value(0)), A::mul(A::add(A::mul(s.ad_value(2), A::sub_from_scalar(0.5, s.ad_value(122))), s.ad_value(125)), s.ad_value(1)));

        s.store_sub_ad(130, A::mul(A::add(A::mul(s.ad_value(10), s.ad_value(124)), s.ad_value(127)), s.ad_value(0)), A::mul(A::add(A::mul(s.ad_value(2), A::sub_from_scalar((-0.5), s.ad_value(124))), s.ad_value(127)), s.ad_value(1)));

        s.store_sub_ad(129, A::mul(A::add(A::mul(s.ad_value(10), s.ad_value(123)), s.ad_value(126)), s.ad_value(0)), A::mul(A::add(A::mul_scaled_rhs(s.ad_value(2), s.ad_value(123), -1.0), s.ad_value(126)), s.ad_value(1)));

        s.store_div_ad(0, A::mul(s.ad_value(17), A::offset(s.ad_value(87), (-1.5))), A::mul_scaled_lhs(s.ad_value(83), 4.0, s.ad_value(7)));

        s.store_mul(131, 0, 116);

        s.store_mul(133, 0, 117);

        s.store_mul(132, 0, 118);

        s.store_mul(0, 92, 24);

        s.store_div_from_scalar(1, 1.0, 84);

        s.store_div_from_scalar(2, 1.0, 85);

        s.store_mul_add_ad_rhs(134, 0, A::sub(A::offset(s.ad_value(113), (-0.5)), A::mul(A::add(A::mul(s.ad_value(11), s.ad_value(131)), s.ad_value(125)), s.ad_value(1))), A::mul(A::add(A::mul(s.ad_value(159), A::sub_from_scalar(0.5, s.ad_value(131))), s.ad_value(125)), s.ad_value(2)));

        s.store_mul_add_ad_rhs(136, 0, A::sub(A::offset(s.ad_value(115), (-0.5)), A::mul(A::add(A::mul(s.ad_value(11), s.ad_value(133)), s.ad_value(127)), s.ad_value(1))), A::mul(A::add(A::mul(s.ad_value(159), A::sub_from_scalar((-0.5), s.ad_value(133))), s.ad_value(127)), s.ad_value(2)));

        s.store_mul_add_ad_rhs(135, 0, A::sub(s.ad_value(114), A::mul(A::add(A::mul(s.ad_value(11), s.ad_value(132)), s.ad_value(126)), s.ad_value(1))), A::mul(A::add(A::mul_scaled_rhs(s.ad_value(159), s.ad_value(132), -1.0), s.ad_value(126)), s.ad_value(2)));

        s.store_div_from_scalar_sub_ad(0, s.v[35], A::add(s.ad_value(41), s.ad_value(77)), s.ad_value(79));

        s.store_mul_sub_from_scalar_rhs(167, 0, 0.5, 128);

        s.store_mul_sub_from_scalar_rhs(169, 0, (-0.5), 130);

        s.store_mul_neg_lhs(168, 0, 129);

        s.store_div_from_scalar(0, 1.0, 63);

        s.store_mul_sub_ad_rhs(137, 0, A::mul(A::offset(s.ad_value(128), 0.5), s.ad_value(40)), s.ad_value(167));

        s.store_mul_sub_ad_rhs(139, 0, A::mul(A::offset(s.ad_value(130), (-0.5)), s.ad_value(40)), s.ad_value(169));

        s.store_mul_sub_ad_rhs(138, 0, A::mul(s.ad_value(129), s.ad_value(40)), s.ad_value(168));

        s.store_mul(0, 91, 24);

        s.store_mul_offset_rhs(119, 0, 113, (-1.0));

        s.store_mul(120, 0, 115);

        s.store_mul(121, 0, 114);

        s.store_div_ad_lhs(0, A::mul_scaled_output(A::offset(s.ad_value(111), 1.0), s.ad_value(17), (-0.66666666)), 99);

        s.store_mul_ad_rhs(1, 0, A::add_scaled_inputs(s.ad_value(93), 1.0, s.ad_value(94), 2.0));

        s.store_mul_ad_rhs(2, 0, A::add_scaled_inputs(s.ad_value(94), 1.0, s.ad_value(93), 2.0));

        s.store_div_ad(0, A::mul_scaled_lhs(s.ad_value(111), -1.0, s.ad_value(100)), A::mul(A::add(A::offset(s.ad_value(111), 2.0), s.ad_value(111)), s.ad_value(107)));

        s.store_add_ad(185, A::add(A::mul(s.ad_value(0), s.ad_value(113)), A::mul(s.ad_value(1), s.ad_value(116))), A::mul(s.ad_value(2), s.ad_value(119)));

        s.store_add_ad(186, A::add(A::mul(s.ad_value(0), s.ad_value(115)), A::mul(s.ad_value(1), s.ad_value(117))), A::mul(s.ad_value(2), s.ad_value(120)));

        s.store_add_ad(187, A::add(A::mul(s.ad_value(0), s.ad_value(114)), A::mul(s.ad_value(1), s.ad_value(118))), A::mul(s.ad_value(2), s.ad_value(121)));

        s.store_sub_ad(0, A::offset(s.ad_value(111), 1.0), A::div(s.ad_value(100), A::mul_scaled_lhs(A::offset(s.ad_value(111), 1.0), 2.0, s.ad_value(107))));

        s.store_mul_scaled_ad_rhs(188, 112, -1.0, A::add(A::mul(s.ad_value(0), s.ad_value(113)), s.ad_value(185)));

        s.store_mul_scaled_ad_rhs(189, 112, -1.0, A::add(A::mul(s.ad_value(0), s.ad_value(115)), s.ad_value(186)));

        s.store_mul_scaled_ad_rhs(190, 112, -1.0, A::add(A::mul(s.ad_value(0), s.ad_value(114)), s.ad_value(187)));

        s.b[255] = (p.p22 == 0.0);
        s.v[255] = if s.b[255] { 1.0 } else { 0.0 };

        if s.b[255] {
            s.store_div_ad(0, A::scale(s.ad_value(6), p.p21), A::mul(s.ad_value(157), s.ad_value(175)));
            s.store_mul(164, 0, 113);
            s.store_mul(166, 0, 115);
            s.store_mul(165, 0, 114);
            s.store_sub_scaled_inputs(140, 137, -1.0, 164, 1.0);
            s.store_sub_scaled_inputs(142, 139, -1.0, 166, 1.0);
            s.store_sub_scaled_inputs(141, 138, -1.0, 165, 1.0);
        }

        if (!s.b[255]) {
            s.store_div_from_scalar(0, s.v[37], 47);
            s.store_sub_ad_lhs(140, A::mul(s.ad_value(0), A::add_scaled_inputs(s.ad_value(188), 1.0, s.ad_value(185), s.v[39])), 137);
            s.store_sub_ad_lhs(142, A::mul(s.ad_value(0), A::add_scaled_inputs(s.ad_value(189), 1.0, s.ad_value(186), s.v[39])), 139);
            s.store_sub_ad_lhs(141, A::mul(s.ad_value(0), A::add_scaled_inputs(s.ad_value(190), 1.0, s.ad_value(187), s.v[39])), 138);
        }

        s.store_div_from_scalar_mul_ad(0, (-s.v[62]), A::mul_scaled_lhs(s.ad_value(15), 4.0, s.ad_value(72)), A::add(A::add(s.ad_value(61), s.ad_value(5)), s.ad_value(27)));

        s.store_mul(170, 0, 113);

        s.store_mul(172, 0, 115);

        s.store_mul(171, 0, 114);

        s.store_mul_sub_ad_rhs(21, 16, A::add(A::mul(A::add(s.ad_value(170), s.ad_value(140)), s.ad_value(86)), s.ad_value(116)), s.ad_value(134));

        s.store_mul_scaled_ad_rhs(19, 16, -1.0, A::sub(A::add(A::mul(A::add(s.ad_value(172), s.ad_value(142)), s.ad_value(86)), s.ad_value(117)), s.ad_value(136)));

        s.store_mul_sub_ad_rhs(18, 16, A::add(A::mul(A::add(s.ad_value(171), s.ad_value(141)), s.ad_value(86)), s.ad_value(118)), s.ad_value(135));

        s.v[193] = ((p.p36 * p.p37) / (s.v[192] - p.p27));

        s.v[194] = ((p.p36 * p.p37) / (s.v[192] - p.p27));

        s.store_div_from_scalar_ad(0, 1.0, A::add_scaled_inputs(A::scale_offset(s.ad_value(19), s.v[193], 1.0), 1.0, s.ad_value(21), s.v[194]));

        s.store_mul(150, 150, 0);

        s.store_sub_scaled_ad_lhs(177, A::sub(s.ad_value(146), s.ad_value(147)), 10, s.v[36]);

        s.b[256] = ((s.v[177] > 0.0) && (s.v[43] > 0.0));
        s.v[256] = if s.b[256] { 1.0 } else { 0.0 };

        if s.b[256] {
            s.store_div_from_scalar(180, 1.0, 177);
            s.store_mul_neg_lhs(176, 42, 180);
        }

        s.b[257] = (s.v[176] < (-35.0));
        s.v[257] = if s.b[257] { 1.0 } else { 0.0 };

        if (s.b[256] && s.b[257]) {
            s.store_scalar(176, (-35.0));
        }

        if s.b[256] {
            s.store_exp(179, 176);
            s.store_mul3_lhs(22, 43, 177, 179);
            s.store_mul(23, 22, 150);
        }

        if (!s.b[256]) {
            s.store_scalar(176, 0.0);
            s.store_scalar(23, 0.0);
        }

        s.v[109] = ((s.v[192] * s.v[191]) * p.p13);

        s.store_mul(97, 93, 95);

        s.store_mul(98, 94, 96);

        s.store_sqrt_ad(0, A::add_scaled_inputs(s.ad_value(61), 1.0, s.ad_value(5), 0.5));

        s.store_scale(181, 0, 2.0);

        s.store_mul_scaled_ad_lhs(110, A::offset(A::div(s.ad_value(4), s.ad_value(181)), 1.0), 17, s.v[109]);

        s.store_mul_scaled_ad_rhs(102, 110, -1.0, A::offset(A::div(A::add_scaled_inputs(A::add(A::add_scaled_inputs(s.ad_value(98), 3.0, A::mul_scaled_lhs(s.ad_value(96), 6.0, s.ad_value(93)), 1.0), A::mul_scaled_lhs(s.ad_value(94), 4.0, s.ad_value(95))), 0.266666666, s.ad_value(97), (2.0 * 0.266666666)), s.ad_value(99)), (-0.5)));

        s.store_mul_scaled_ad_rhs(103, 110, -1.0, A::offset(A::div(A::add_scaled_inputs(A::add(A::add_scaled_inputs(s.ad_value(97), 3.0, A::mul_scaled_lhs(s.ad_value(95), 6.0, s.ad_value(94)), 1.0), A::mul_scaled_lhs(s.ad_value(93), 4.0, s.ad_value(96))), 0.266666666, s.ad_value(98), (2.0 * 0.266666666)), s.ad_value(99)), (-0.5)));

        s.store_add(104, 103, 102);

        s.store_sub_ad(105, A::sub_scaled_inputs(A::add(A::mul_scaled_lhs(s.ad_value(4), (-0.5), s.ad_value(108)), s.ad_value(3)), s.v[109], s.ad_value(143), s.v[109]), A::div(A::mul(s.ad_value(104), s.ad_value(4)), A::add(s.ad_value(4), s.ad_value(181))));

        s.store_sub_scaled_inputs(106, 104, -1.0, 105, 1.0);

        let assign2910_ad_e2274: A = A::ddt(s.ad_value(102), ddt_scale, eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, s.ad_value(102).value));
        s.store_ad_value(200, assign2910_ad_e2274);

        let assign2920_ad_e2276: A = A::ddt(s.ad_value(103), ddt_scale, eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, s.ad_value(103).value));
        s.store_ad_value(201, assign2920_ad_e2276);

        s.b[258] = (s.v[44] == 1.0);
        s.v[258] = if s.b[258] { 1.0 } else { 0.0 };

        if (p.p1 != 0.0) {
            s.store_scaled_mul(260, 49, 152, (4.0 * 1.3806226e-23));
            s.store_scaled_mul(259, 18, 18, (p.p42 * 1.0 / ((((s.v[192] * p.p8) * s.v[191]) * p.p13))));
        }

        s.b[261] = ((p.p9 == 0.0) && (p.p37 > 0.0));
        s.v[261] = if s.b[261] { 1.0 } else { 0.0 };

        if s.b[261] {
            s.store_scalar(202, ((2.0 * p.p37) * s.v[192]));
        }

        if (!s.b[261]) {
            s.store_scalar(202, p.p9);
        }

        s.b[262] = ((p.p11 == 0.0) && (p.p37 > 0.0));
        s.v[262] = if s.b[262] { 1.0 } else { 0.0 };

        if s.b[262] {
            s.store_scalar(204, ((4.0 * p.p37) + s.v[192]));
        }

        if (!s.b[262]) {
            s.store_scalar(204, p.p11);
        }

        s.b[263] = ((p.p10 == 0.0) && (p.p37 > 0.0));
        s.v[263] = if s.b[263] { 1.0 } else { 0.0 };

        if s.b[263] {
            s.store_scalar(203, ((2.0 * p.p37) * s.v[192]));
        }

        if (!s.b[263]) {
            s.store_scalar(203, p.p10);
        }

        s.b[264] = ((p.p12 == 0.0) && (p.p37 > 0.0));
        s.v[264] = if s.b[264] { 1.0 } else { 0.0 };

        if s.b[264] {
            s.store_scalar(205, ((4.0 * p.p37) + s.v[192]));
        }

        if (!s.b[264]) {
            s.store_scalar(205, p.p12);
        }

        s.store_ad_value(208, A::exp_scaled_input(A::add_scaled_inputs(A::sub(A::div(s.ad_value(52), A::scale(s.ad_value(55), THERMAL_VOLTAGE_PER_K)), A::div(s.ad_value(51), s.ad_value(17))), 1.0, A::ln(s.ad_value(54)), p.p65), 1.0 / (p.p43)));

        s.store_scale(210, 208, p.p44);

        s.store_scale(211, 208, p.p45);

        s.store_scale(212, 208, p.p46);

        s.store_sub_from_scalar_ad(213, p.p50, A::scale(s.ad_value(53), p.p69));

        s.store_sub_from_scalar_ad(214, p.p51, A::scale(s.ad_value(53), p.p70));

        s.store_sub_from_scalar_ad(215, p.p52, A::scale(s.ad_value(53), p.p71));

        s.store_offset_scaled(216, 53, ((p.p66) * (p.p53)), p.p53);

        s.store_offset_scaled(217, 53, ((p.p67) * (p.p54)), p.p54);

        s.store_offset_scaled(218, 53, ((p.p68) * (p.p55)), p.p55);

        s.store_offset_scaled(219, 54, ((p.p72) * (p.p59)), (((((((-1.0)) * (p.p72))) + (1.0))) * (p.p59)));

        s.store_offset_scaled(220, 54, ((p.p73) * (p.p60)), (((((((-1.0)) * (p.p73))) + (1.0))) * (p.p60)));

        s.store_offset_scaled(221, 54, ((p.p74) * (p.p61)), (((((((-1.0)) * (p.p74))) + (1.0))) * (p.p61)));

        s.store_scaled_voltage(206, ctx, nodes, Some(0), Some(3), p.p0);

        s.store_scaled_voltage(207, ctx, nodes, Some(2), Some(3), p.p0);

        s.store_add_scaled_ad_lhs(222, A::add(A::mul(s.ad_value(210), s.ad_value(203)), A::mul(s.ad_value(211), s.ad_value(205))), 212, s.v[192]);

        s.store_div_ad(223, A::mul_scaled_lhs(s.ad_value(206), -1.0, s.ad_value(54)), A::scale(s.ad_value(17), p.p43));

        s.b[265] = (s.v[223] < (-40.0));
        s.v[265] = if s.b[265] { 1.0 } else { 0.0 };

        if s.b[265] {
            s.store_scalar(223, (-40.0));
        }

        s.store_div_ad(209, A::mul(A::sub_from_scalar(p.p58, s.ad_value(206)), s.ad_value(54)), A::scale(s.ad_value(17), p.p43));

        s.b[266] = (s.v[209] > 70.0);
        s.v[266] = if s.b[266] { 1.0 } else { 0.0 };

        if s.b[266] {
            s.store_scalar(226, 1.0);
        }

        if (!s.b[266]) {
            s.store_offset_scaled_ad(226, A::exp_scaled_input(s.ad_value(209), -1.0), p.p57, 1.0);
        }

        s.store_mul_scaled_ad_rhs(228, 212, (-s.v[192]), A::offset(A::exp(A::div(A::scale(A::div(A::mul(s.ad_value(206), s.ad_value(54)), A::mul(s.ad_value(17), s.ad_value(221))), p.p64), A::max_with_scalar(A::offset(s.ad_value(206), p.p64), 0.001))), (-1.0)));

        s.store_sub_ad_rhs(228, 228, A::mul(A::mul(s.ad_value(205), s.ad_value(211)), A::offset(A::exp(A::div(A::scale(A::div(A::mul(s.ad_value(206), s.ad_value(54)), A::mul(s.ad_value(17), s.ad_value(220))), p.p63), A::max_with_scalar(A::offset(s.ad_value(206), p.p63), 0.001))), (-1.0))));

        s.store_sub_ad_rhs(228, 228, A::mul(A::mul(s.ad_value(203), s.ad_value(210)), A::offset(A::exp(A::div(A::scale(A::div(A::mul(s.ad_value(206), s.ad_value(54)), A::mul(s.ad_value(17), s.ad_value(219))), p.p62), A::max_with_scalar(A::offset(s.ad_value(206), p.p62), 0.001))), (-1.0))));

        s.store_add_scaled_ad_lhs(224, A::add(A::mul(s.ad_value(210), s.ad_value(202)), A::mul(s.ad_value(211), s.ad_value(204))), 212, s.v[192]);

        s.store_div_ad(225, A::mul_scaled_lhs(s.ad_value(207), -1.0, s.ad_value(54)), A::scale(s.ad_value(17), p.p43));

        s.b[267] = (s.v[225] < (-40.0));
        s.v[267] = if s.b[267] { 1.0 } else { 0.0 };

        if s.b[267] {
            s.store_scalar(225, (-40.0));
        }

        s.store_div_ad(209, A::mul(A::sub_from_scalar(p.p58, s.ad_value(207)), s.ad_value(54)), A::scale(s.ad_value(17), p.p43));

        s.b[268] = (s.v[209] > 70.0);
        s.v[268] = if s.b[268] { 1.0 } else { 0.0 };

        if s.b[268] {
            s.store_scalar(227, 1.0);
        }

        if (!s.b[268]) {
            s.store_offset_scaled_ad(227, A::exp_scaled_input(s.ad_value(209), -1.0), p.p57, 1.0);
        }

        s.store_mul_scaled_ad_rhs(229, 212, (-s.v[192]), A::offset(A::exp(A::div(A::scale(A::div(A::mul(s.ad_value(207), s.ad_value(54)), A::mul(s.ad_value(17), s.ad_value(221))), p.p64), A::max_with_scalar(A::offset(s.ad_value(207), p.p64), 0.001))), (-1.0)));

        s.store_sub_ad_rhs(229, 229, A::mul(A::mul(s.ad_value(204), s.ad_value(211)), A::offset(A::exp(A::div(A::scale(A::div(A::mul(s.ad_value(207), s.ad_value(54)), A::mul(s.ad_value(17), s.ad_value(220))), p.p63), A::max_with_scalar(A::offset(s.ad_value(207), p.p63), 0.001))), (-1.0))));

        s.store_sub_ad_rhs(229, 229, A::mul(A::mul(s.ad_value(202), s.ad_value(210)), A::offset(A::exp(A::div(A::scale(A::div(A::mul(s.ad_value(207), s.ad_value(54)), A::mul(s.ad_value(17), s.ad_value(219))), p.p62), A::max_with_scalar(A::offset(s.ad_value(207), p.p62), 0.001))), (-1.0))));

        s.b[269] = (s.v[206] > 0.0);
        s.v[269] = if s.b[269] { 1.0 } else { 0.0 };

        if s.b[269] {
            s.store_mul_ad(230, A::mul(s.ad_value(216), s.ad_value(203)), A::exp_scaled_input(A::ln(A::offset(A::div(s.ad_value(206), s.ad_value(213)), 1.0)), (-p.p47)));
            s.store_mul_ad(231, A::mul(s.ad_value(217), s.ad_value(205)), A::exp_scaled_input(A::ln(A::offset(A::div(s.ad_value(206), s.ad_value(214)), 1.0)), (-p.p48)));
            s.store_mul_scaled_ad_rhs(232, 218, s.v[192], A::exp_scaled_input(A::ln(A::offset(A::div(s.ad_value(206), s.ad_value(215)), 1.0)), (-p.p49)));
        }

        if (!s.b[269]) {
            s.store_mul_ad(230, A::mul(s.ad_value(216), s.ad_value(203)), A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(206), p.p47), s.ad_value(213))));
            s.store_mul_ad(231, A::mul(s.ad_value(217), s.ad_value(205)), A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(206), p.p48), s.ad_value(214))));
            s.store_mul_scaled_ad_rhs(232, 218, s.v[192], A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(206), p.p49), s.ad_value(215))));
        }

        s.store_mul_add_ad_lhs(236, A::add(s.ad_value(230), s.ad_value(231)), s.ad_value(232), 206);

        s.b[270] = (s.v[207] > 0.0);
        s.v[270] = if s.b[270] { 1.0 } else { 0.0 };

        if s.b[270] {
            s.store_mul_ad(233, A::mul(s.ad_value(216), s.ad_value(202)), A::exp_scaled_input(A::ln(A::offset(A::div(s.ad_value(207), s.ad_value(213)), 1.0)), (-p.p47)));
            s.store_mul_ad(234, A::mul(s.ad_value(217), s.ad_value(204)), A::exp_scaled_input(A::ln(A::offset(A::div(s.ad_value(207), s.ad_value(214)), 1.0)), (-p.p48)));
            s.store_mul_scaled_ad_rhs(235, 218, s.v[192], A::exp_scaled_input(A::ln(A::offset(A::div(s.ad_value(207), s.ad_value(215)), 1.0)), (-p.p49)));
        }

        if (!s.b[270]) {
            s.store_mul_ad(233, A::mul(s.ad_value(216), s.ad_value(202)), A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(207), p.p47), s.ad_value(213))));
            s.store_mul_ad(234, A::mul(s.ad_value(217), s.ad_value(204)), A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(207), p.p48), s.ad_value(214))));
            s.store_mul_scaled_ad_rhs(235, 218, s.v[192], A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(207), p.p49), s.ad_value(215))));
        }

        s.store_mul_add_ad_lhs(237, A::add(s.ad_value(233), s.ad_value(234)), s.ad_value(235), 207);

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.v[199] = (11.7 * 8.8541879239442e-12);

        s.v[157] = 0.0;

        s.v[6] = 0.0;

        s.v[175] = 0.0;

        s.v[31] = (s.v[199] / p.p13);

        s.v[34] = (((s.v[31] * p.p14)) as f64).sqrt();

        s.v[35] = (s.v[34] * p.p25);

        s.v[32] = ((3.0 * s.v[31]) * p.p28);

        s.v[33] = (s.v[31] * p.p29);

        s.v[37] = (p.p13 / (s.v[199] * p.p22));

        s.v[182] = ((p.p30 + p.p30) / p.p13);

        s.v[39] = (if (p.p0 > 0.0) { 0.5 } else { 0.3333333333333 });

        s.b[238] = (p.p3 == (-(-1e21)));
        s.v[238] = if s.b[238] { 1.0 } else { 0.0 };

        if s.b[238] {
            s.store_scalar(49, (ctx_temp + p.p2));
        }

        if (!s.b[238]) {
            s.store_scalar(49, (p.p3 + 273.15));
        }

        s.b[239] = (p.p4 == (-(-1e21)));
        s.v[239] = if s.b[239] { 1.0 } else { 0.0 };

        if s.b[239] {
            s.store_scalar(55, (25.0 + 273.15));
        }

        if (!s.b[239]) {
            s.store_scalar(55, (p.p4 + 273.15));
        }

        s.store_scale(17, 49, THERMAL_VOLTAGE_PER_K);

        s.store_scale(25, 17, 0.1);

        s.store_div_from_scalar(24, 1.0, 17);

        s.store_scale(26, 17, 2.0);

        s.store_scale(27, 26, 2.0);

        s.store_square(28, 17);

        s.store_scale(29, 28, 2.0);

        s.store_scale(30, 28, 16.0);

        s.store_sub_from_scalar_ad(51, 1.16, A::div(A::mul_scaled_lhs(s.ad_value(49), 0.000702, s.ad_value(49)), A::offset(s.ad_value(49), 1108.0)));

        s.store_sub_from_scalar_ad(52, 1.16, A::div(A::mul_scaled_lhs(s.ad_value(55), 0.000702, s.ad_value(55)), A::offset(s.ad_value(55), 1108.0)));

        s.store_sub(53, 49, 55);

        s.store_div(54, 49, 55);

        s.store_sub_from_scalar_ad(56, p.p15, A::scale(s.ad_value(53), p.p16));

        s.store_scale_ad(58, A::powf(s.ad_value(54), p.p20), p.p19);

        s.store_scale_ad(59, A::powf(s.ad_value(54), p.p24), p.p23);

        s.store_add_ad_lhs(61, A::sub(A::sub_scaled_inputs(s.ad_value(54), p.p18, A::mul_scaled_lhs(s.ad_value(17), 3.0, A::ln(s.ad_value(54))), 1.0), A::mul(s.ad_value(52), s.ad_value(54))), 51);

        s.v[0] = 0.2;

        s.store_offset(1, 61, (-s.v[0]));

        s.store_offset_ad(61, A::add_scaled_inputs(s.ad_value(1), 0.5, A::sqrt(A::add(A::square(s.ad_value(1)), A::square(s.ad_value(17)))), 0.5), s.v[0]);

        s.store_sqrt(71, 61);

        s.store_div_from_scalar(40, 1.0, 59);

        s.store_scale(41, 59, s.v[34]);

        s.v[191] = (p.p5 + p.p26);

        s.v[192] = (p.p6 + p.p27);

        s.store_scale(158, 59, s.v[191]);

        s.store_mul_offset_ad_rhs(173, 17, A::ln(A::mul_scaled_lhs(s.ad_value(158), 0.5, s.ad_value(24))), (-0.6));

        s.v[48] = (1.0 / (((s.v[192] * s.v[191])) as f64).sqrt());

        s.b[240] = (p.p0 > 0.0);
        s.v[240] = if s.b[240] { 1.0 } else { 0.0 };

        if s.b[240] {
            s.store_ad_value(57, {
                if (p.p38 != 1e-6) {
                    A::offset(s.ad_value(56), (s.v[48] * (p.p38 - 1e-6)))
                } else {
                    s.ad_value(56)
                }
            });
        }

        if (!s.b[240]) {
            s.store_ad_value(57, {
                if (p.p38 != 1e-6) {
                    A::sub_from_scalar((s.v[48] * (1e-6 - p.p38)), s.ad_value(56))
                } else {
                    A::neg(s.ad_value(56))
                }
            });
        }

        s.store_scale_ad(50, {
            if (p.p39 != 1e-6) {
                A::scale(s.ad_value(58), (1.0 + ((p.p39 - 1e-6) * s.v[48])))
            } else {
                s.ad_value(58)
            }
        }, s.v[192]);

        s.v[62] = (if (p.p40 != 1e-6) { (p.p17 + ((p.p40 - 1e-6) * s.v[48])) } else { p.p17 });

        s.store_scale(153, 71, s.v[62]);

        s.b[241] = (s.v[182] == 0.0);
        s.v[241] = if s.b[241] { 1.0 } else { 0.0 };

        if s.b[241] {
            s.store_scalar(183, 0.0);
        }

        if (!s.b[241]) {
            s.store_scalar(184, (0.28 * ((s.v[191] / (p.p31 * p.p8)) - 0.1)));
            s.store_div_from_scalar_offset_ad(242, 1.0, A::add_scaled_inputs(s.ad_value(184), 0.5, A::sqrt(A::offset(A::square(s.ad_value(184)), 0.001936)), 0.5), 1.0);
            s.store_scaled_mul(183, 242, 242, s.v[182]);
        }

        s.store_scaled_voltage(145, ctx, nodes, Some(1), Some(3), p.p0);

        s.store_scaled_voltage(147, ctx, nodes, Some(2), Some(3), p.p0);

        s.store_scaled_voltage(146, ctx, nodes, Some(0), Some(3), p.p0);

        s.b[243] = ((s.v[146] - s.v[147]) < 0.0);
        s.v[243] = if s.b[243] { 1.0 } else { 0.0 };

        if s.b[243] {
            s.store_scalar(44, (-1.0));
            s.copy_ad(38, 147);
            s.copy_ad(147, 146);
            s.copy_ad(146, 38);
        }

        if (!s.b[243]) {
            s.store_scalar(44, 1.0);
        }

        s.store_add_ad_lhs(143, A::add(A::sub(A::sub(s.ad_value(145), s.ad_value(57)), s.ad_value(183)), s.ad_value(61)), 153);

        s.store_sqrt_ad(144, A::add_scaled_inputs(A::square(s.ad_value(143)), 1.0, s.ad_value(30), 2.0));

        s.store_scaled_add(3, 143, 144, 0.5);

        s.store_add(70, 61, 147);

        s.store_sqrt_square_add(76, 70, 30);

        s.store_sqrt_scaled_ad(74, A::add(s.ad_value(70), s.ad_value(76)), 0.5);

        s.store_add(69, 61, 146);

        s.store_sqrt_square_add(75, 69, 30);

        s.store_sqrt_scaled_ad(73, A::add(s.ad_value(69), s.ad_value(75)), 0.5);

        s.v[45] = ((s.v[32] * p.p7) / s.v[192]);

        s.v[46] = ((s.v[33] * p.p8) / s.v[191]);

        s.store_sqrt_offset_input(67, 3, ((0.25 * s.v[62]) * s.v[62]));

        s.store_sub_ad(68, A::sub(s.ad_value(3), s.ad_value(61)), A::scaled_offset(s.ad_value(67), (-(0.5 * s.v[62])), s.v[62]));

        s.store_sqrt_add_ad(174, A::add(s.ad_value(68), s.ad_value(61)), s.ad_value(25));

        s.store_add_scaled_ad_lhs(64, A::sub_from_scalar(s.v[62], A::add_scaled_inputs(s.ad_value(74), s.v[46], s.ad_value(73), s.v[46])), 174, s.v[45]);

        s.store_sqrt_square_add(65, 64, 25);

        s.store_scaled_add(4, 64, 65, 0.5);

        s.store_sqrt_add_ad(66, s.ad_value(3), A::mul_scaled_lhs(s.ad_value(4), 0.25, s.ad_value(4)));

        s.store_sub_ad(5, A::sub(s.ad_value(3), s.ad_value(61)), A::mul(s.ad_value(4), A::sub_scaled_inputs(s.ad_value(66), 1.0, s.ad_value(4), 0.5)));

        s.store_mul_sub_lhs(0, 5, 147, 24);

        s.b[244] = (s.v[0] > (-0.35));
        s.v[244] = if s.b[244] { 1.0 } else { 0.0 };

        if s.b[244] {
            s.store_div_from_scalar_sub_ad(196, 2.0, A::offset(s.ad_value(0), 1.3), A::ln(A::offset(s.ad_value(0), 1.6)));
            s.store_div_ad(197, A::offset(s.ad_value(196), 2.0), A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(196))));
            s.store_div_ad(195, A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(197))), A::offset(s.ad_value(197), 2.0));
        }

        s.b[245] = (s.v[0] > (-15.0));
        s.v[245] = if s.b[245] { 1.0 } else { 0.0 };

        if ((!s.b[244]) && s.b[245]) {
            s.store_offset_ad(196, A::exp_scaled_input(s.ad_value(0), -1.0), 1.55);
            s.store_div_ad(197, A::offset(s.ad_value(196), 2.0), A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(196))));
            s.store_div_ad(195, A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(197))), A::offset(s.ad_value(197), 2.0));
        }

        s.b[246] = (s.v[0] > (-23.0));
        s.v[246] = if s.b[246] { 1.0 } else { 0.0 };

        if (((!s.b[244]) && (!s.b[245])) && s.b[246]) {
            s.store_div_from_scalar_offset_ad(195, 1.0, A::exp_scaled_input(s.ad_value(0), -1.0), 2.0);
        }

        if (((!s.b[244]) && (!s.b[245])) && (!s.b[246])) {
            s.store_offset_exp(195, 0, 1e-64);
        }

        s.store_mul_offset_rhs(7, 195, 195, 1.0);

        s.store_sqrt(87, 7);

        s.copy_ad(90, 195);

        s.store_div(160, 17, 158);

        s.store_sqrt_offset_ad(80, A::mul(s.ad_value(87), s.ad_value(160)), 0.25);

        s.store_mul_offset_rhs(10, 158, 80, (-0.5));

        s.store_scaled_sub(77, 146, 147, 0.5);

        s.store_mul_offset_ad_rhs(78, 30, A::sub_scaled_inputs(s.ad_value(87), p.p25, A::mul(s.ad_value(10), s.ad_value(24)), p.p25), 0.015625);

        s.store_sqrt_square_add(81, 10, 78);

        s.store_sqrt_add_ad(82, A::mul(A::sub(s.ad_value(77), s.ad_value(10)), A::sub(s.ad_value(77), s.ad_value(10))), s.ad_value(78));

        s.store_sub(79, 81, 82);

        s.store_sqrt_offset_ad(83, A::mul(A::sub_scaled_inputs(s.ad_value(87), 1.0, A::ln(s.ad_value(7)), 0.75), s.ad_value(160)), 0.25);

        s.store_add_ad_lhs(11, A::mul(s.ad_value(158), A::offset(s.ad_value(83), (-0.5))), 173);

        s.store_sub(159, 77, 11);

        s.store_sqrt_square_add(84, 11, 78);

        s.store_sqrt_square_add(85, 159, 78);

        s.store_mul_add_ad_lhs(0, A::sub(A::sub(A::sub(s.ad_value(5), s.ad_value(77)), s.ad_value(147)), s.ad_value(84)), s.ad_value(85), 24);

        s.b[247] = (s.v[0] > (-0.35));
        s.v[247] = if s.b[247] { 1.0 } else { 0.0 };

        if s.b[247] {
            s.store_div_from_scalar_sub_ad(196, 2.0, A::offset(s.ad_value(0), 1.3), A::ln(A::offset(s.ad_value(0), 1.6)));
            s.store_div_ad(197, A::offset(s.ad_value(196), 2.0), A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(196))));
            s.store_div_ad(195, A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(197))), A::offset(s.ad_value(197), 2.0));
        }

        s.b[248] = (s.v[0] > (-15.0));
        s.v[248] = if s.b[248] { 1.0 } else { 0.0 };

        if ((!s.b[247]) && s.b[248]) {
            s.store_offset_ad(196, A::exp_scaled_input(s.ad_value(0), -1.0), 1.55);
            s.store_div_ad(197, A::offset(s.ad_value(196), 2.0), A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(196))));
            s.store_div_ad(195, A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(197))), A::offset(s.ad_value(197), 2.0));
        }

        s.b[249] = (s.v[0] > (-23.0));
        s.v[249] = if s.b[249] { 1.0 } else { 0.0 };

        if (((!s.b[247]) && (!s.b[248])) && s.b[249]) {
            s.store_div_from_scalar_offset_ad(195, 1.0, A::exp_scaled_input(s.ad_value(0), -1.0), 2.0);
        }

        if (((!s.b[247]) && (!s.b[248])) && (!s.b[249])) {
            s.store_offset_exp(195, 0, 1e-64);
        }

        s.store_mul_offset_rhs(9, 195, 195, 1.0);

        s.copy_ad(92, 195);

        s.store_scaled_ln_ad(12, A::offset(A::div(A::sub(s.ad_value(77), s.ad_value(79)), s.ad_value(41)), 1.0), s.v[35]);

        s.store_add_ad(155, A::sub_from_scalar(s.v[191], s.ad_value(12)), A::mul(A::add(s.ad_value(77), s.ad_value(79)), s.ad_value(40)));

        s.v[154] = (0.1 * s.v[191]);

        s.store_sqrt_square_offset(63, 155, (s.v[154] * s.v[154]));

        s.store_scaled_add(13, 155, 63, 0.5);

        s.store_mul_sub_lhs(0, 5, 146, 24);

        s.b[250] = (s.v[0] > (-0.35));
        s.v[250] = if s.b[250] { 1.0 } else { 0.0 };

        if s.b[250] {
            s.store_div_from_scalar_sub_ad(196, 2.0, A::offset(s.ad_value(0), 1.3), A::ln(A::offset(s.ad_value(0), 1.6)));
            s.store_div_ad(197, A::offset(s.ad_value(196), 2.0), A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(196))));
            s.store_div_ad(195, A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(197))), A::offset(s.ad_value(197), 2.0));
        }

        s.b[251] = (s.v[0] > (-15.0));
        s.v[251] = if s.b[251] { 1.0 } else { 0.0 };

        if ((!s.b[250]) && s.b[251]) {
            s.store_offset_ad(196, A::exp_scaled_input(s.ad_value(0), -1.0), 1.55);
            s.store_div_ad(197, A::offset(s.ad_value(196), 2.0), A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(196))));
            s.store_div_ad(195, A::add(A::offset(s.ad_value(0), 1.0), A::ln(s.ad_value(197))), A::offset(s.ad_value(197), 2.0));
        }

        s.b[252] = (s.v[0] > (-23.0));
        s.v[252] = if s.b[252] { 1.0 } else { 0.0 };

        if (((!s.b[250]) && (!s.b[251])) && s.b[252]) {
            s.store_div_from_scalar_offset_ad(195, 1.0, A::exp_scaled_input(s.ad_value(0), -1.0), 2.0);
        }

        if (((!s.b[250]) && (!s.b[251])) && (!s.b[252])) {
            s.store_offset_exp(195, 0, 1e-64);
        }

        s.store_mul_offset_rhs(8, 195, 195, 1.0);

        s.copy_ad(91, 195);

        s.store_offset(95, 7, 0.25);

        s.store_offset(96, 8, 0.25);

        s.store_sqrt(93, 95);

        s.store_sqrt(94, 96);

        s.store_mul_ad(99, A::add(s.ad_value(93), s.ad_value(94)), A::add(s.ad_value(93), s.ad_value(94)));

        s.store_offset_add(107, 5, 61, 1e-6);

        s.store_scaled_sqrt(108, 107, 2.0);

        s.store_div_from_scalar(111, s.v[62], 108);

        s.store_div_from_scalar_offset_input(112, s.v[62], 108, s.v[62]);

        s.store_mul_ad(100, A::mul_scaled_lhs(A::offset(s.ad_value(111), 1.0), -1.0, s.ad_value(17)), A::offset(A::div(A::add_scaled_inputs(A::add(s.ad_value(96), A::mul(s.ad_value(94), s.ad_value(93))), (0.66666666 + 0.66666666), s.ad_value(95), (0.66666666 + 0.66666666)), A::add(s.ad_value(93), s.ad_value(94))), (-1.0)));

        s.store_sub_scaled_ad_rhs(101, 108, ((-0.5) * s.v[62]), A::mul(s.ad_value(112), s.ad_value(100)));

        s.b[253] = (p.p22 == 0.0);
        s.v[253] = if s.b[253] { 1.0 } else { 0.0 };

        if s.b[253] {
            s.store_sqrt_square_add(175, 5, 29);
            s.store_scaled_add(6, 5, 175, 0.5);
            s.store_offset_scaled(157, 6, p.p21, 1.0);
            s.store_div_ad_rhs(14, 50, A::mul(s.ad_value(13), s.ad_value(157)));
        }

        s.b[254] = ((s.v[101] + (s.v[39] * s.v[100])) > 0.0);
        s.v[254] = if s.b[254] { 1.0 } else { 0.0 };

        if ((!s.b[253]) && s.b[254]) {
            s.store_offset_ad(47, A::add_scaled_inputs(s.ad_value(101), s.v[37], s.ad_value(100), (s.v[39] * s.v[37])), 1.0);
        }

        if ((!s.b[253]) && (!s.b[254])) {
            s.store_sub_from_scalar_ad(47, 1.0, A::add_scaled_inputs(s.ad_value(101), s.v[37], s.ad_value(100), (s.v[39] * s.v[37])));
        }

        if (!s.b[253]) {
            s.store_offset_scaled(156, 153, s.v[37], 1.0);
            s.store_div_ad(14, A::mul(s.ad_value(50), s.ad_value(156)), A::mul(s.ad_value(13), s.ad_value(47)));
        }

        s.store_sqrt_add_ad(72, A::add(s.ad_value(61), s.ad_value(5)), s.ad_value(27));

        s.store_offset_div_from_scalar_ad(15, s.v[62], A::scale(s.ad_value(72), 2.0), 1.0);

        s.store_sub(86, 7, 9);

        s.store_mul3_lhs(16, 29, 15, 14);

        s.store_scaled_div(0, 4, 65, (1.0 / (2.0)));

        s.store_div(1, 3, 144);

        s.store_div_ad_lhs(161, A::mul_scaled_lhs(s.ad_value(0), (-s.v[46]), s.ad_value(73)), 75);

        s.store_div_ad_lhs(163, A::mul_scaled_lhs(s.ad_value(0), (-s.v[46]), s.ad_value(74)), 76);

        s.store_div_ad_lhs(2, A::add(s.ad_value(5), s.ad_value(61)), 66);

        s.store_mul_neg_lhs(113, 2, 161);

        s.store_mul_neg_lhs(115, 2, 163);

        s.store_mul(0, 90, 24);

        s.store_mul(116, 0, 113);

        s.store_mul_offset_rhs(117, 0, 115, (-1.0));

        s.store_div_ad_rhs(0, 17, A::mul_scaled_lhs(s.ad_value(80), 4.0, s.ad_value(87)));

        s.store_mul(122, 0, 116);

        s.store_mul(124, 0, 117);

        s.store_scale(0, 27, (2.0 * p.p25));

        s.store_scaled_div(1, 17, 87, (1.0 / (2.0)));

        s.store_mul_sub_ad_rhs(125, 0, A::mul(s.ad_value(116), s.ad_value(1)), s.ad_value(122));

        s.store_mul_sub_ad_rhs(127, 0, A::mul(s.ad_value(117), s.ad_value(1)), s.ad_value(124));

        s.store_div_from_scalar(0, 1.0, 81);

        s.store_div_from_scalar(1, 1.0, 82);

        s.store_sub(2, 77, 10);

        s.store_sub_ad(128, A::mul(A::add(A::mul(s.ad_value(10), s.ad_value(122)), s.ad_value(125)), s.ad_value(0)), A::mul(A::add(A::mul(s.ad_value(2), A::sub_from_scalar(0.5, s.ad_value(122))), s.ad_value(125)), s.ad_value(1)));

        s.store_sub_ad(130, A::mul(A::add(A::mul(s.ad_value(10), s.ad_value(124)), s.ad_value(127)), s.ad_value(0)), A::mul(A::add(A::mul(s.ad_value(2), A::sub_from_scalar((-0.5), s.ad_value(124))), s.ad_value(127)), s.ad_value(1)));

    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.store_div_ad(0, A::mul(s.ad_value(17), A::offset(s.ad_value(87), (-1.5))), A::mul_scaled_lhs(s.ad_value(83), 4.0, s.ad_value(7)));

        s.store_mul(131, 0, 116);

        s.store_mul(133, 0, 117);

        s.store_mul(0, 92, 24);

        s.store_div_from_scalar(1, 1.0, 84);

        s.store_div_from_scalar(2, 1.0, 85);

        s.store_mul_add_ad_rhs(134, 0, A::sub(A::offset(s.ad_value(113), (-0.5)), A::mul(A::add(A::mul(s.ad_value(11), s.ad_value(131)), s.ad_value(125)), s.ad_value(1))), A::mul(A::add(A::mul(s.ad_value(159), A::sub_from_scalar(0.5, s.ad_value(131))), s.ad_value(125)), s.ad_value(2)));

        s.store_mul_add_ad_rhs(136, 0, A::sub(A::offset(s.ad_value(115), (-0.5)), A::mul(A::add(A::mul(s.ad_value(11), s.ad_value(133)), s.ad_value(127)), s.ad_value(1))), A::mul(A::add(A::mul(s.ad_value(159), A::sub_from_scalar((-0.5), s.ad_value(133))), s.ad_value(127)), s.ad_value(2)));

        s.store_div_from_scalar_sub_ad(0, s.v[35], A::add(s.ad_value(41), s.ad_value(77)), s.ad_value(79));

        s.store_mul_sub_from_scalar_rhs(167, 0, 0.5, 128);

        s.store_mul_sub_from_scalar_rhs(169, 0, (-0.5), 130);

        s.store_div_from_scalar(0, 1.0, 63);

        s.store_mul_sub_ad_rhs(137, 0, A::mul(A::offset(s.ad_value(128), 0.5), s.ad_value(40)), s.ad_value(167));

        s.store_mul_sub_ad_rhs(139, 0, A::mul(A::offset(s.ad_value(130), (-0.5)), s.ad_value(40)), s.ad_value(169));

        s.store_mul(0, 91, 24);

        s.store_mul_offset_rhs(119, 0, 113, (-1.0));

        s.store_mul(120, 0, 115);

        s.store_div_ad_lhs(0, A::mul_scaled_output(A::offset(s.ad_value(111), 1.0), s.ad_value(17), (-0.66666666)), 99);

        s.store_mul_ad_rhs(1, 0, A::add_scaled_inputs(s.ad_value(93), 1.0, s.ad_value(94), 2.0));

        s.store_mul_ad_rhs(2, 0, A::add_scaled_inputs(s.ad_value(94), 1.0, s.ad_value(93), 2.0));

        s.store_div_ad(0, A::mul_scaled_lhs(s.ad_value(111), -1.0, s.ad_value(100)), A::mul(A::add(A::offset(s.ad_value(111), 2.0), s.ad_value(111)), s.ad_value(107)));

        s.store_add_ad(185, A::add(A::mul(s.ad_value(0), s.ad_value(113)), A::mul(s.ad_value(1), s.ad_value(116))), A::mul(s.ad_value(2), s.ad_value(119)));

        s.store_add_ad(186, A::add(A::mul(s.ad_value(0), s.ad_value(115)), A::mul(s.ad_value(1), s.ad_value(117))), A::mul(s.ad_value(2), s.ad_value(120)));

        s.store_sub_ad(0, A::offset(s.ad_value(111), 1.0), A::div(s.ad_value(100), A::mul_scaled_lhs(A::offset(s.ad_value(111), 1.0), 2.0, s.ad_value(107))));

        s.store_mul_scaled_ad_rhs(188, 112, -1.0, A::add(A::mul(s.ad_value(0), s.ad_value(113)), s.ad_value(185)));

        s.store_mul_scaled_ad_rhs(189, 112, -1.0, A::add(A::mul(s.ad_value(0), s.ad_value(115)), s.ad_value(186)));

        s.b[255] = (p.p22 == 0.0);
        s.v[255] = if s.b[255] { 1.0 } else { 0.0 };

        if s.b[255] {
            s.store_div_ad(0, A::scale(s.ad_value(6), p.p21), A::mul(s.ad_value(157), s.ad_value(175)));
            s.store_mul(164, 0, 113);
            s.store_mul(166, 0, 115);
            s.store_sub_scaled_inputs(140, 137, -1.0, 164, 1.0);
            s.store_sub_scaled_inputs(142, 139, -1.0, 166, 1.0);
        }

        if (!s.b[255]) {
            s.store_div_from_scalar(0, s.v[37], 47);
            s.store_sub_ad_lhs(140, A::mul(s.ad_value(0), A::add_scaled_inputs(s.ad_value(188), 1.0, s.ad_value(185), s.v[39])), 137);
            s.store_sub_ad_lhs(142, A::mul(s.ad_value(0), A::add_scaled_inputs(s.ad_value(189), 1.0, s.ad_value(186), s.v[39])), 139);
        }

        s.store_div_from_scalar_mul_ad(0, (-s.v[62]), A::mul_scaled_lhs(s.ad_value(15), 4.0, s.ad_value(72)), A::add(A::add(s.ad_value(61), s.ad_value(5)), s.ad_value(27)));

        s.store_mul(170, 0, 113);

        s.store_mul(172, 0, 115);

        s.store_mul_sub_ad_rhs(21, 16, A::add(A::mul(A::add(s.ad_value(170), s.ad_value(140)), s.ad_value(86)), s.ad_value(116)), s.ad_value(134));

        s.store_mul_scaled_ad_rhs(19, 16, -1.0, A::sub(A::add(A::mul(A::add(s.ad_value(172), s.ad_value(142)), s.ad_value(86)), s.ad_value(117)), s.ad_value(136)));

        s.v[193] = ((p.p36 * p.p37) / (s.v[192] - p.p27));

        s.v[194] = ((p.p36 * p.p37) / (s.v[192] - p.p27));

        s.store_div_from_scalar_ad(0, 1.0, A::add_scaled_inputs(A::scale_offset(s.ad_value(19), s.v[193], 1.0), 1.0, s.ad_value(21), s.v[194]));

        s.v[109] = ((s.v[192] * s.v[191]) * p.p13);

        s.store_mul(97, 93, 95);

        s.store_mul(98, 94, 96);

        s.store_sqrt_ad(0, A::add_scaled_inputs(s.ad_value(61), 1.0, s.ad_value(5), 0.5));

        s.store_scale(181, 0, 2.0);

        s.store_mul_scaled_ad_lhs(110, A::offset(A::div(s.ad_value(4), s.ad_value(181)), 1.0), 17, s.v[109]);

        s.store_mul_scaled_ad_rhs(102, 110, -1.0, A::offset(A::div(A::add_scaled_inputs(A::add(A::add_scaled_inputs(s.ad_value(98), 3.0, A::mul_scaled_lhs(s.ad_value(96), 6.0, s.ad_value(93)), 1.0), A::mul_scaled_lhs(s.ad_value(94), 4.0, s.ad_value(95))), 0.266666666, s.ad_value(97), (2.0 * 0.266666666)), s.ad_value(99)), (-0.5)));

        s.store_mul_scaled_ad_rhs(103, 110, -1.0, A::offset(A::div(A::add_scaled_inputs(A::add(A::add_scaled_inputs(s.ad_value(97), 3.0, A::mul_scaled_lhs(s.ad_value(95), 6.0, s.ad_value(94)), 1.0), A::mul_scaled_lhs(s.ad_value(93), 4.0, s.ad_value(96))), 0.266666666, s.ad_value(98), (2.0 * 0.266666666)), s.ad_value(99)), (-0.5)));

        s.store_add(104, 103, 102);

        s.store_sub_ad(105, A::sub_scaled_inputs(A::add(A::mul_scaled_lhs(s.ad_value(4), (-0.5), s.ad_value(108)), s.ad_value(3)), s.v[109], s.ad_value(143), s.v[109]), A::div(A::mul(s.ad_value(104), s.ad_value(4)), A::add(s.ad_value(4), s.ad_value(181))));

        s.store_sub_scaled_inputs(106, 104, -1.0, 105, 1.0);

        let assign2910_e2274_q: f64 = s.v[102];
        s.v[200] = s.v[102];
        s.dn[200][0] = s.dn[102][0];
        s.dn[200][1] = s.dn[102][1];
        s.dn[200][2] = s.dn[102][2];
        s.dn[200][3] = s.dn[102][3];
        s.rv[200] = assign2910_e2274_q;
        s.rdn[200][0] = s.dn[102][0];
        s.rdn[200][1] = s.dn[102][1];
        s.rdn[200][2] = s.dn[102][2];
        s.rdn[200][3] = s.dn[102][3];

        let assign2920_e2276_q: f64 = s.v[103];
        s.v[201] = s.v[103];
        s.dn[201][0] = s.dn[103][0];
        s.dn[201][1] = s.dn[103][1];
        s.dn[201][2] = s.dn[103][2];
        s.dn[201][3] = s.dn[103][3];
        s.rv[201] = assign2920_e2276_q;
        s.rdn[201][0] = s.dn[103][0];
        s.rdn[201][1] = s.dn[103][1];
        s.rdn[201][2] = s.dn[103][2];
        s.rdn[201][3] = s.dn[103][3];

        s.b[258] = (s.v[44] == 1.0);
        s.v[258] = if s.b[258] { 1.0 } else { 0.0 };

        s.b[261] = ((p.p9 == 0.0) && (p.p37 > 0.0));
        s.v[261] = if s.b[261] { 1.0 } else { 0.0 };

        if s.b[261] {
            s.store_scalar(202, ((2.0 * p.p37) * s.v[192]));
        }

        if (!s.b[261]) {
            s.store_scalar(202, p.p9);
        }

        s.b[262] = ((p.p11 == 0.0) && (p.p37 > 0.0));
        s.v[262] = if s.b[262] { 1.0 } else { 0.0 };

        if s.b[262] {
            s.store_scalar(204, ((4.0 * p.p37) + s.v[192]));
        }

        if (!s.b[262]) {
            s.store_scalar(204, p.p11);
        }

        s.b[263] = ((p.p10 == 0.0) && (p.p37 > 0.0));
        s.v[263] = if s.b[263] { 1.0 } else { 0.0 };

        if s.b[263] {
            s.store_scalar(203, ((2.0 * p.p37) * s.v[192]));
        }

        if (!s.b[263]) {
            s.store_scalar(203, p.p10);
        }

        s.b[264] = ((p.p12 == 0.0) && (p.p37 > 0.0));
        s.v[264] = if s.b[264] { 1.0 } else { 0.0 };

        if s.b[264] {
            s.store_scalar(205, ((4.0 * p.p37) + s.v[192]));
        }

        if (!s.b[264]) {
            s.store_scalar(205, p.p12);
        }

        s.store_sub_from_scalar_ad(213, p.p50, A::scale(s.ad_value(53), p.p69));

        s.store_sub_from_scalar_ad(214, p.p51, A::scale(s.ad_value(53), p.p70));

        s.store_sub_from_scalar_ad(215, p.p52, A::scale(s.ad_value(53), p.p71));

        s.store_offset_scaled(216, 53, ((p.p66) * (p.p53)), p.p53);

        s.store_offset_scaled(217, 53, ((p.p67) * (p.p54)), p.p54);

        s.store_offset_scaled(218, 53, ((p.p68) * (p.p55)), p.p55);

        s.store_scaled_voltage(206, ctx, nodes, Some(0), Some(3), p.p0);

        s.store_scaled_voltage(207, ctx, nodes, Some(2), Some(3), p.p0);

        s.b[269] = (s.v[206] > 0.0);
        s.v[269] = if s.b[269] { 1.0 } else { 0.0 };

        if s.b[269] {
            s.store_mul_ad(230, A::mul(s.ad_value(216), s.ad_value(203)), A::exp_scaled_input(A::ln(A::offset(A::div(s.ad_value(206), s.ad_value(213)), 1.0)), (-p.p47)));
            s.store_mul_ad(231, A::mul(s.ad_value(217), s.ad_value(205)), A::exp_scaled_input(A::ln(A::offset(A::div(s.ad_value(206), s.ad_value(214)), 1.0)), (-p.p48)));
            s.store_mul_scaled_ad_rhs(232, 218, s.v[192], A::exp_scaled_input(A::ln(A::offset(A::div(s.ad_value(206), s.ad_value(215)), 1.0)), (-p.p49)));
        }

        if (!s.b[269]) {
            s.store_mul_ad(230, A::mul(s.ad_value(216), s.ad_value(203)), A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(206), p.p47), s.ad_value(213))));
            s.store_mul_ad(231, A::mul(s.ad_value(217), s.ad_value(205)), A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(206), p.p48), s.ad_value(214))));
            s.store_mul_scaled_ad_rhs(232, 218, s.v[192], A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(206), p.p49), s.ad_value(215))));
        }

        s.store_mul_add_ad_lhs(236, A::add(s.ad_value(230), s.ad_value(231)), s.ad_value(232), 206);

        s.b[270] = (s.v[207] > 0.0);
        s.v[270] = if s.b[270] { 1.0 } else { 0.0 };

        if s.b[270] {
            s.store_mul_ad(233, A::mul(s.ad_value(216), s.ad_value(202)), A::exp_scaled_input(A::ln(A::offset(A::div(s.ad_value(207), s.ad_value(213)), 1.0)), (-p.p47)));
            s.store_mul_ad(234, A::mul(s.ad_value(217), s.ad_value(204)), A::exp_scaled_input(A::ln(A::offset(A::div(s.ad_value(207), s.ad_value(214)), 1.0)), (-p.p48)));
            s.store_mul_scaled_ad_rhs(235, 218, s.v[192], A::exp_scaled_input(A::ln(A::offset(A::div(s.ad_value(207), s.ad_value(215)), 1.0)), (-p.p49)));
        }

        if (!s.b[270]) {
            s.store_mul_ad(233, A::mul(s.ad_value(216), s.ad_value(202)), A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(207), p.p47), s.ad_value(213))));
            s.store_mul_ad(234, A::mul(s.ad_value(217), s.ad_value(204)), A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(207), p.p48), s.ad_value(214))));
            s.store_mul_scaled_ad_rhs(235, 218, s.v[192], A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(207), p.p49), s.ad_value(215))));
        }

        s.store_mul_add_ad_lhs(237, A::add(s.ad_value(233), s.ad_value(234)), s.ad_value(235), 207);

    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let eq0_e84: f64 = (p.p0 * s.v[44]);
        let eq0_e84_d_n0: f64 = (p.p0 * s.dn[44][0]);
        let eq0_e84_d_n1: f64 = (p.p0 * s.dn[44][1]);
        let eq0_e84_d_n2: f64 = (p.p0 * s.dn[44][2]);
        let eq0_e84_d_n3: f64 = (p.p0 * s.dn[44][3]);
        let eq0_e86: f64 = (eq0_e84 * s.v[150]);
        let eq0_e86_d_n0: f64 = ((eq0_e84_d_n0 * s.v[150]) + (eq0_e84 * s.dn[150][0]));
        let eq0_e86_d_n1: f64 = ((eq0_e84_d_n1 * s.v[150]) + (eq0_e84 * s.dn[150][1]));
        let eq0_e86_d_n2: f64 = ((eq0_e84_d_n2 * s.v[150]) + (eq0_e84 * s.dn[150][2]));
        let eq0_e86_d_n3: f64 = ((eq0_e84_d_n3 * s.v[150]) + (eq0_e84 * s.dn[150][3]));
        let eq0_value: f64 = eq0_e86;
        stamper.stamp_current_local(
            Some(0),
            Some(2),
            multiplicity * (eq0_value),
            &[
                GeneratedDerivative::node(0, multiplicity * eq0_e86_d_n0),
                GeneratedDerivative::node(1, multiplicity * eq0_e86_d_n1),
                GeneratedDerivative::node(2, multiplicity * eq0_e86_d_n2),
                GeneratedDerivative::node(3, multiplicity * eq0_e86_d_n3),
            ],
        );
        let (eq1_e92, eq1_e92_d_n0, eq1_e92_d_n1, eq1_e92_d_n2, eq1_e92_d_n3,) = {
    if s.b[258] {
        let eq1_e90: f64 = (p.p0 * s.v[200]);
        let eq1_e90_d_n0: f64 = (p.p0 * s.dn[200][0]);
        let eq1_e90_d_n1: f64 = (p.p0 * s.dn[200][1]);
        let eq1_e90_d_n2: f64 = (p.p0 * s.dn[200][2]);
        let eq1_e90_d_n3: f64 = (p.p0 * s.dn[200][3]);
        (eq1_e90, eq1_e90_d_n0, eq1_e90_d_n1, eq1_e90_d_n2, eq1_e90_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e92;
        stamper.stamp_current_local(
            Some(0),
            Some(3),
            multiplicity * (eq1_value),
            &[
                GeneratedDerivative::node(0, multiplicity * eq1_e92_d_n0),
                GeneratedDerivative::node(1, multiplicity * eq1_e92_d_n1),
                GeneratedDerivative::node(2, multiplicity * eq1_e92_d_n2),
                GeneratedDerivative::node(3, multiplicity * eq1_e92_d_n3),
            ],
        );
        let (eq2_e98, eq2_e98_d_n0, eq2_e98_d_n1, eq2_e98_d_n2, eq2_e98_d_n3,) = {
    if s.b[258] {
        let eq2_e96: f64 = (p.p0 * s.v[201]);
        let eq2_e96_d_n0: f64 = (p.p0 * s.dn[201][0]);
        let eq2_e96_d_n1: f64 = (p.p0 * s.dn[201][1]);
        let eq2_e96_d_n2: f64 = (p.p0 * s.dn[201][2]);
        let eq2_e96_d_n3: f64 = (p.p0 * s.dn[201][3]);
        (eq2_e96, eq2_e96_d_n0, eq2_e96_d_n1, eq2_e96_d_n2, eq2_e96_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e98;
        stamper.stamp_current_local(
            Some(2),
            Some(3),
            multiplicity * (eq2_value),
            &[
                GeneratedDerivative::node(0, multiplicity * eq2_e98_d_n0),
                GeneratedDerivative::node(1, multiplicity * eq2_e98_d_n1),
                GeneratedDerivative::node(2, multiplicity * eq2_e98_d_n2),
                GeneratedDerivative::node(3, multiplicity * eq2_e98_d_n3),
            ],
        );
        let (eq3_e104, eq3_e104_d_n0, eq3_e104_d_n1, eq3_e104_d_n2, eq3_e104_d_n3,) = {
    if s.b[258] {
        let eq3_e102: f64 = (p.p0 * s.v[23]);
        let eq3_e102_d_n0: f64 = (p.p0 * s.dn[23][0]);
        let eq3_e102_d_n1: f64 = (p.p0 * s.dn[23][1]);
        let eq3_e102_d_n2: f64 = (p.p0 * s.dn[23][2]);
        let eq3_e102_d_n3: f64 = (p.p0 * s.dn[23][3]);
        (eq3_e102, eq3_e102_d_n0, eq3_e102_d_n1, eq3_e102_d_n2, eq3_e102_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e104;
        stamper.stamp_current_local(
            Some(0),
            Some(3),
            multiplicity * (eq3_value),
            &[
                GeneratedDerivative::node(0, multiplicity * eq3_e104_d_n0),
                GeneratedDerivative::node(1, multiplicity * eq3_e104_d_n1),
                GeneratedDerivative::node(2, multiplicity * eq3_e104_d_n2),
                GeneratedDerivative::node(3, multiplicity * eq3_e104_d_n3),
            ],
        );
        let (eq4_e111, eq4_e111_d_n0, eq4_e111_d_n1, eq4_e111_d_n2, eq4_e111_d_n3,) = {
    if (!s.b[258]) {
        let eq4_e109: f64 = (p.p0 * s.v[200]);
        let eq4_e109_d_n0: f64 = (p.p0 * s.dn[200][0]);
        let eq4_e109_d_n1: f64 = (p.p0 * s.dn[200][1]);
        let eq4_e109_d_n2: f64 = (p.p0 * s.dn[200][2]);
        let eq4_e109_d_n3: f64 = (p.p0 * s.dn[200][3]);
        (eq4_e109, eq4_e109_d_n0, eq4_e109_d_n1, eq4_e109_d_n2, eq4_e109_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e111;
        stamper.stamp_current_local(
            Some(2),
            Some(3),
            multiplicity * (eq4_value),
            &[
                GeneratedDerivative::node(0, multiplicity * eq4_e111_d_n0),
                GeneratedDerivative::node(1, multiplicity * eq4_e111_d_n1),
                GeneratedDerivative::node(2, multiplicity * eq4_e111_d_n2),
                GeneratedDerivative::node(3, multiplicity * eq4_e111_d_n3),
            ],
        );
        let (eq5_e118, eq5_e118_d_n0, eq5_e118_d_n1, eq5_e118_d_n2, eq5_e118_d_n3,) = {
    if (!s.b[258]) {
        let eq5_e116: f64 = (p.p0 * s.v[201]);
        let eq5_e116_d_n0: f64 = (p.p0 * s.dn[201][0]);
        let eq5_e116_d_n1: f64 = (p.p0 * s.dn[201][1]);
        let eq5_e116_d_n2: f64 = (p.p0 * s.dn[201][2]);
        let eq5_e116_d_n3: f64 = (p.p0 * s.dn[201][3]);
        (eq5_e116, eq5_e116_d_n0, eq5_e116_d_n1, eq5_e116_d_n2, eq5_e116_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e118;
        stamper.stamp_current_local(
            Some(0),
            Some(3),
            multiplicity * (eq5_value),
            &[
                GeneratedDerivative::node(0, multiplicity * eq5_e118_d_n0),
                GeneratedDerivative::node(1, multiplicity * eq5_e118_d_n1),
                GeneratedDerivative::node(2, multiplicity * eq5_e118_d_n2),
                GeneratedDerivative::node(3, multiplicity * eq5_e118_d_n3),
            ],
        );
        let (eq6_e125, eq6_e125_d_n0, eq6_e125_d_n1, eq6_e125_d_n2, eq6_e125_d_n3,) = {
    if (!s.b[258]) {
        let eq6_e123: f64 = (p.p0 * s.v[23]);
        let eq6_e123_d_n0: f64 = (p.p0 * s.dn[23][0]);
        let eq6_e123_d_n1: f64 = (p.p0 * s.dn[23][1]);
        let eq6_e123_d_n2: f64 = (p.p0 * s.dn[23][2]);
        let eq6_e123_d_n3: f64 = (p.p0 * s.dn[23][3]);
        (eq6_e123, eq6_e123_d_n0, eq6_e123_d_n1, eq6_e123_d_n2, eq6_e123_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e125;
        stamper.stamp_current_local(
            Some(2),
            Some(3),
            multiplicity * (eq6_value),
            &[
                GeneratedDerivative::node(0, multiplicity * eq6_e125_d_n0),
                GeneratedDerivative::node(1, multiplicity * eq6_e125_d_n1),
                GeneratedDerivative::node(2, multiplicity * eq6_e125_d_n2),
                GeneratedDerivative::node(3, multiplicity * eq6_e125_d_n3),
            ],
        );
        let eq7_e128: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, s.v[106]);
        let eq7_e128_d_n0: f64 = (s.dn[106][0] * ddt_scale);
        let eq7_e128_d_n1: f64 = (s.dn[106][1] * ddt_scale);
        let eq7_e128_d_n2: f64 = (s.dn[106][2] * ddt_scale);
        let eq7_e128_d_n3: f64 = (s.dn[106][3] * ddt_scale);
        let eq7_e129: f64 = (p.p0 * eq7_e128);
        let eq7_e129_d_n0: f64 = (p.p0 * eq7_e128_d_n0);
        let eq7_e129_d_n1: f64 = (p.p0 * eq7_e128_d_n1);
        let eq7_e129_d_n2: f64 = (p.p0 * eq7_e128_d_n2);
        let eq7_e129_d_n3: f64 = (p.p0 * eq7_e128_d_n3);
        let eq7_value: f64 = eq7_e129;
        stamper.stamp_current_local(
            Some(1),
            Some(3),
            multiplicity * (eq7_value),
            &[
                GeneratedDerivative::node(0, multiplicity * eq7_e129_d_n0),
                GeneratedDerivative::node(1, multiplicity * eq7_e129_d_n1),
                GeneratedDerivative::node(2, multiplicity * eq7_e129_d_n2),
                GeneratedDerivative::node(3, multiplicity * eq7_e129_d_n3),
            ],
        );
        let (eq8_e140,) = {
    if (p.p1 != 0.0) {
        let eq8_e138: f64 = 0.0;
        (eq8_e138,)
    } else {
        (0.0,)
    }
};
        let eq8_value: f64 = eq8_e140;
        stamper.stamp_current_const_local(
            Some(0),
            Some(2),
            multiplicity * (eq8_value),
        );
        let eq9_e144: f64 = (s.v[223]).exp();
        let eq9_e144_d_n0: f64 = (eq9_e144 * s.dn[223][0]);
        let eq9_e144_d_n1: f64 = (eq9_e144 * s.dn[223][1]);
        let eq9_e144_d_n2: f64 = (eq9_e144 * s.dn[223][2]);
        let eq9_e144_d_n3: f64 = (eq9_e144 * s.dn[223][3]);
        let eq9_e145: f64 = (1.0 - eq9_e144);
        let eq9_e145_d_n0: f64 = (-eq9_e144_d_n0);
        let eq9_e145_d_n1: f64 = (-eq9_e144_d_n1);
        let eq9_e145_d_n2: f64 = (-eq9_e144_d_n2);
        let eq9_e145_d_n3: f64 = (-eq9_e144_d_n3);
        let eq9_e146: f64 = (s.v[222] * eq9_e145);
        let eq9_e146_d_n0: f64 = ((s.dn[222][0] * eq9_e145) + (s.v[222] * eq9_e145_d_n0));
        let eq9_e146_d_n1: f64 = ((s.dn[222][1] * eq9_e145) + (s.v[222] * eq9_e145_d_n1));
        let eq9_e146_d_n2: f64 = ((s.dn[222][2] * eq9_e145) + (s.v[222] * eq9_e145_d_n2));
        let eq9_e146_d_n3: f64 = ((s.dn[222][3] * eq9_e145) + (s.v[222] * eq9_e145_d_n3));
        let eq9_e148: f64 = (eq9_e146 * s.v[226]);
        let eq9_e148_d_n0: f64 = ((eq9_e146_d_n0 * s.v[226]) + (eq9_e146 * s.dn[226][0]));
        let eq9_e148_d_n1: f64 = ((eq9_e146_d_n1 * s.v[226]) + (eq9_e146 * s.dn[226][1]));
        let eq9_e148_d_n2: f64 = ((eq9_e146_d_n2 * s.v[226]) + (eq9_e146 * s.dn[226][2]));
        let eq9_e148_d_n3: f64 = ((eq9_e146_d_n3 * s.v[226]) + (eq9_e146 * s.dn[226][3]));
        let eq9_e151: f64 = (s.v[206] * p.p56);
        let eq9_e151_d_n0: f64 = (s.dn[206][0] * p.p56);
        let eq9_e151_d_n1: f64 = (s.dn[206][1] * p.p56);
        let eq9_e151_d_n2: f64 = (s.dn[206][2] * p.p56);
        let eq9_e151_d_n3: f64 = (s.dn[206][3] * p.p56);
        let eq9_e152: f64 = (eq9_e148 + eq9_e151);
        let eq9_e152_d_n0: f64 = (eq9_e148_d_n0 + eq9_e151_d_n0);
        let eq9_e152_d_n1: f64 = (eq9_e148_d_n1 + eq9_e151_d_n1);
        let eq9_e152_d_n2: f64 = (eq9_e148_d_n2 + eq9_e151_d_n2);
        let eq9_e152_d_n3: f64 = (eq9_e148_d_n3 + eq9_e151_d_n3);
        let eq9_e154: f64 = (eq9_e152 + s.v[228]);
        let eq9_e154_d_n0: f64 = (eq9_e152_d_n0 + s.dn[228][0]);
        let eq9_e154_d_n1: f64 = (eq9_e152_d_n1 + s.dn[228][1]);
        let eq9_e154_d_n2: f64 = (eq9_e152_d_n2 + s.dn[228][2]);
        let eq9_e154_d_n3: f64 = (eq9_e152_d_n3 + s.dn[228][3]);
        let eq9_e156: f64 = (eq9_e154 * p.p0);
        let eq9_e156_d_n0: f64 = (eq9_e154_d_n0 * p.p0);
        let eq9_e156_d_n1: f64 = (eq9_e154_d_n1 * p.p0);
        let eq9_e156_d_n2: f64 = (eq9_e154_d_n2 * p.p0);
        let eq9_e156_d_n3: f64 = (eq9_e154_d_n3 * p.p0);
        let eq9_e158: f64 = (eq9_e156 * p.p7);
        let eq9_e158_d_n0: f64 = (eq9_e156_d_n0 * p.p7);
        let eq9_e158_d_n1: f64 = (eq9_e156_d_n1 * p.p7);
        let eq9_e158_d_n2: f64 = (eq9_e156_d_n2 * p.p7);
        let eq9_e158_d_n3: f64 = (eq9_e156_d_n3 * p.p7);
        let eq9_value: f64 = eq9_e158;
        stamper.stamp_current_local(
            Some(0),
            Some(3),
            multiplicity * (eq9_value),
            &[
                GeneratedDerivative::node(0, multiplicity * eq9_e158_d_n0),
                GeneratedDerivative::node(1, multiplicity * eq9_e158_d_n1),
                GeneratedDerivative::node(2, multiplicity * eq9_e158_d_n2),
                GeneratedDerivative::node(3, multiplicity * eq9_e158_d_n3),
            ],
        );
        let eq10_e162: f64 = (s.v[225]).exp();
        let eq10_e162_d_n0: f64 = (eq10_e162 * s.dn[225][0]);
        let eq10_e162_d_n1: f64 = (eq10_e162 * s.dn[225][1]);
        let eq10_e162_d_n2: f64 = (eq10_e162 * s.dn[225][2]);
        let eq10_e162_d_n3: f64 = (eq10_e162 * s.dn[225][3]);
        let eq10_e163: f64 = (1.0 - eq10_e162);
        let eq10_e163_d_n0: f64 = (-eq10_e162_d_n0);
        let eq10_e163_d_n1: f64 = (-eq10_e162_d_n1);
        let eq10_e163_d_n2: f64 = (-eq10_e162_d_n2);
        let eq10_e163_d_n3: f64 = (-eq10_e162_d_n3);
        let eq10_e164: f64 = (s.v[224] * eq10_e163);
        let eq10_e164_d_n0: f64 = ((s.dn[224][0] * eq10_e163) + (s.v[224] * eq10_e163_d_n0));
        let eq10_e164_d_n1: f64 = ((s.dn[224][1] * eq10_e163) + (s.v[224] * eq10_e163_d_n1));
        let eq10_e164_d_n2: f64 = ((s.dn[224][2] * eq10_e163) + (s.v[224] * eq10_e163_d_n2));
        let eq10_e164_d_n3: f64 = ((s.dn[224][3] * eq10_e163) + (s.v[224] * eq10_e163_d_n3));
        let eq10_e166: f64 = (eq10_e164 * s.v[227]);
        let eq10_e166_d_n0: f64 = ((eq10_e164_d_n0 * s.v[227]) + (eq10_e164 * s.dn[227][0]));
        let eq10_e166_d_n1: f64 = ((eq10_e164_d_n1 * s.v[227]) + (eq10_e164 * s.dn[227][1]));
        let eq10_e166_d_n2: f64 = ((eq10_e164_d_n2 * s.v[227]) + (eq10_e164 * s.dn[227][2]));
        let eq10_e166_d_n3: f64 = ((eq10_e164_d_n3 * s.v[227]) + (eq10_e164 * s.dn[227][3]));
        let eq10_e169: f64 = (s.v[207] * p.p56);
        let eq10_e169_d_n0: f64 = (s.dn[207][0] * p.p56);
        let eq10_e169_d_n1: f64 = (s.dn[207][1] * p.p56);
        let eq10_e169_d_n2: f64 = (s.dn[207][2] * p.p56);
        let eq10_e169_d_n3: f64 = (s.dn[207][3] * p.p56);
        let eq10_e170: f64 = (eq10_e166 + eq10_e169);
        let eq10_e170_d_n0: f64 = (eq10_e166_d_n0 + eq10_e169_d_n0);
        let eq10_e170_d_n1: f64 = (eq10_e166_d_n1 + eq10_e169_d_n1);
        let eq10_e170_d_n2: f64 = (eq10_e166_d_n2 + eq10_e169_d_n2);
        let eq10_e170_d_n3: f64 = (eq10_e166_d_n3 + eq10_e169_d_n3);
        let eq10_e172: f64 = (eq10_e170 + s.v[229]);
        let eq10_e172_d_n0: f64 = (eq10_e170_d_n0 + s.dn[229][0]);
        let eq10_e172_d_n1: f64 = (eq10_e170_d_n1 + s.dn[229][1]);
        let eq10_e172_d_n2: f64 = (eq10_e170_d_n2 + s.dn[229][2]);
        let eq10_e172_d_n3: f64 = (eq10_e170_d_n3 + s.dn[229][3]);
        let eq10_e174: f64 = (eq10_e172 * p.p0);
        let eq10_e174_d_n0: f64 = (eq10_e172_d_n0 * p.p0);
        let eq10_e174_d_n1: f64 = (eq10_e172_d_n1 * p.p0);
        let eq10_e174_d_n2: f64 = (eq10_e172_d_n2 * p.p0);
        let eq10_e174_d_n3: f64 = (eq10_e172_d_n3 * p.p0);
        let eq10_e176: f64 = (eq10_e174 * p.p7);
        let eq10_e176_d_n0: f64 = (eq10_e174_d_n0 * p.p7);
        let eq10_e176_d_n1: f64 = (eq10_e174_d_n1 * p.p7);
        let eq10_e176_d_n2: f64 = (eq10_e174_d_n2 * p.p7);
        let eq10_e176_d_n3: f64 = (eq10_e174_d_n3 * p.p7);
        let eq10_value: f64 = eq10_e176;
        stamper.stamp_current_local(
            Some(2),
            Some(3),
            multiplicity * (eq10_value),
            &[
                GeneratedDerivative::node(0, multiplicity * eq10_e176_d_n0),
                GeneratedDerivative::node(1, multiplicity * eq10_e176_d_n1),
                GeneratedDerivative::node(2, multiplicity * eq10_e176_d_n2),
                GeneratedDerivative::node(3, multiplicity * eq10_e176_d_n3),
            ],
        );
        let eq11_e178: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, s.v[236]);
        let eq11_e178_d_n0: f64 = (s.dn[236][0] * ddt_scale);
        let eq11_e178_d_n1: f64 = (s.dn[236][1] * ddt_scale);
        let eq11_e178_d_n2: f64 = (s.dn[236][2] * ddt_scale);
        let eq11_e178_d_n3: f64 = (s.dn[236][3] * ddt_scale);
        let eq11_e180: f64 = (eq11_e178 * p.p0);
        let eq11_e180_d_n0: f64 = (eq11_e178_d_n0 * p.p0);
        let eq11_e180_d_n1: f64 = (eq11_e178_d_n1 * p.p0);
        let eq11_e180_d_n2: f64 = (eq11_e178_d_n2 * p.p0);
        let eq11_e180_d_n3: f64 = (eq11_e178_d_n3 * p.p0);
        let eq11_e182: f64 = (eq11_e180 * p.p7);
        let eq11_e182_d_n0: f64 = (eq11_e180_d_n0 * p.p7);
        let eq11_e182_d_n1: f64 = (eq11_e180_d_n1 * p.p7);
        let eq11_e182_d_n2: f64 = (eq11_e180_d_n2 * p.p7);
        let eq11_e182_d_n3: f64 = (eq11_e180_d_n3 * p.p7);
        let eq11_value: f64 = eq11_e182;
        stamper.stamp_current_local(
            Some(0),
            Some(3),
            multiplicity * (eq11_value),
            &[
                GeneratedDerivative::node(0, multiplicity * eq11_e182_d_n0),
                GeneratedDerivative::node(1, multiplicity * eq11_e182_d_n1),
                GeneratedDerivative::node(2, multiplicity * eq11_e182_d_n2),
                GeneratedDerivative::node(3, multiplicity * eq11_e182_d_n3),
            ],
        );
        let eq12_e184: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, s.v[237]);
        let eq12_e184_d_n0: f64 = (s.dn[237][0] * ddt_scale);
        let eq12_e184_d_n1: f64 = (s.dn[237][1] * ddt_scale);
        let eq12_e184_d_n2: f64 = (s.dn[237][2] * ddt_scale);
        let eq12_e184_d_n3: f64 = (s.dn[237][3] * ddt_scale);
        let eq12_e186: f64 = (eq12_e184 * p.p0);
        let eq12_e186_d_n0: f64 = (eq12_e184_d_n0 * p.p0);
        let eq12_e186_d_n1: f64 = (eq12_e184_d_n1 * p.p0);
        let eq12_e186_d_n2: f64 = (eq12_e184_d_n2 * p.p0);
        let eq12_e186_d_n3: f64 = (eq12_e184_d_n3 * p.p0);
        let eq12_e188: f64 = (eq12_e186 * p.p7);
        let eq12_e188_d_n0: f64 = (eq12_e186_d_n0 * p.p7);
        let eq12_e188_d_n1: f64 = (eq12_e186_d_n1 * p.p7);
        let eq12_e188_d_n2: f64 = (eq12_e186_d_n2 * p.p7);
        let eq12_e188_d_n3: f64 = (eq12_e186_d_n3 * p.p7);
        let eq12_value: f64 = eq12_e188;
        stamper.stamp_current_local(
            Some(2),
            Some(3),
            multiplicity * (eq12_value),
            &[
                GeneratedDerivative::node(0, multiplicity * eq12_e188_d_n0),
                GeneratedDerivative::node(1, multiplicity * eq12_e188_d_n1),
                GeneratedDerivative::node(2, multiplicity * eq12_e188_d_n2),
                GeneratedDerivative::node(3, multiplicity * eq12_e188_d_n3),
            ],
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let (eq1_e92, eq1_e92_d_n0, eq1_e92_d_n1, eq1_e92_d_n2, eq1_e92_d_n3, eq1_e92_q, eq1_e92_q_d_n0, eq1_e92_q_d_n1, eq1_e92_q_d_n2, eq1_e92_q_d_n3,) = {
    if s.b[258] {
        let eq1_e89_q: f64 = s.rv[200];
        let eq1_e90: f64 = (p.p0 * s.v[200]);
        let eq1_e90_d_n0: f64 = (p.p0 * s.dn[200][0]);
        let eq1_e90_d_n1: f64 = (p.p0 * s.dn[200][1]);
        let eq1_e90_d_n2: f64 = (p.p0 * s.dn[200][2]);
        let eq1_e90_d_n3: f64 = (p.p0 * s.dn[200][3]);
        let eq1_e90_q: f64 = (p.p0 * eq1_e89_q);
        let eq1_e90_q_d_n0: f64 = (p.p0 * s.rdn[200][0]);
        let eq1_e90_q_d_n1: f64 = (p.p0 * s.rdn[200][1]);
        let eq1_e90_q_d_n2: f64 = (p.p0 * s.rdn[200][2]);
        let eq1_e90_q_d_n3: f64 = (p.p0 * s.rdn[200][3]);
        (eq1_e90, eq1_e90_d_n0, eq1_e90_d_n1, eq1_e90_d_n2, eq1_e90_d_n3, eq1_e90_q, eq1_e90_q_d_n0, eq1_e90_q_d_n1, eq1_e90_q_d_n2, eq1_e90_q_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[0]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq1_e92_q_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq1_e92_q_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq1_e92_q_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq1_e92_q_d_n3)),
            ],
        );
        let (eq2_e98, eq2_e98_d_n0, eq2_e98_d_n1, eq2_e98_d_n2, eq2_e98_d_n3, eq2_e98_q, eq2_e98_q_d_n0, eq2_e98_q_d_n1, eq2_e98_q_d_n2, eq2_e98_q_d_n3,) = {
    if s.b[258] {
        let eq2_e95_q: f64 = s.rv[201];
        let eq2_e96: f64 = (p.p0 * s.v[201]);
        let eq2_e96_d_n0: f64 = (p.p0 * s.dn[201][0]);
        let eq2_e96_d_n1: f64 = (p.p0 * s.dn[201][1]);
        let eq2_e96_d_n2: f64 = (p.p0 * s.dn[201][2]);
        let eq2_e96_d_n3: f64 = (p.p0 * s.dn[201][3]);
        let eq2_e96_q: f64 = (p.p0 * eq2_e95_q);
        let eq2_e96_q_d_n0: f64 = (p.p0 * s.rdn[201][0]);
        let eq2_e96_q_d_n1: f64 = (p.p0 * s.rdn[201][1]);
        let eq2_e96_q_d_n2: f64 = (p.p0 * s.rdn[201][2]);
        let eq2_e96_q_d_n3: f64 = (p.p0 * s.rdn[201][3]);
        (eq2_e96, eq2_e96_d_n0, eq2_e96_d_n1, eq2_e96_d_n2, eq2_e96_d_n3, eq2_e96_q, eq2_e96_q_d_n0, eq2_e96_q_d_n1, eq2_e96_q_d_n2, eq2_e96_q_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[2]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq2_e98_q_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq2_e98_q_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq2_e98_q_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq2_e98_q_d_n3)),
            ],
        );
        let (eq4_e111, eq4_e111_d_n0, eq4_e111_d_n1, eq4_e111_d_n2, eq4_e111_d_n3, eq4_e111_q, eq4_e111_q_d_n0, eq4_e111_q_d_n1, eq4_e111_q_d_n2, eq4_e111_q_d_n3,) = {
    if (!s.b[258]) {
        let eq4_e108_q: f64 = s.rv[200];
        let eq4_e109: f64 = (p.p0 * s.v[200]);
        let eq4_e109_d_n0: f64 = (p.p0 * s.dn[200][0]);
        let eq4_e109_d_n1: f64 = (p.p0 * s.dn[200][1]);
        let eq4_e109_d_n2: f64 = (p.p0 * s.dn[200][2]);
        let eq4_e109_d_n3: f64 = (p.p0 * s.dn[200][3]);
        let eq4_e109_q: f64 = (p.p0 * eq4_e108_q);
        let eq4_e109_q_d_n0: f64 = (p.p0 * s.rdn[200][0]);
        let eq4_e109_q_d_n1: f64 = (p.p0 * s.rdn[200][1]);
        let eq4_e109_q_d_n2: f64 = (p.p0 * s.rdn[200][2]);
        let eq4_e109_q_d_n3: f64 = (p.p0 * s.rdn[200][3]);
        (eq4_e109, eq4_e109_d_n0, eq4_e109_d_n1, eq4_e109_d_n2, eq4_e109_d_n3, eq4_e109_q, eq4_e109_q_d_n0, eq4_e109_q_d_n1, eq4_e109_q_d_n2, eq4_e109_q_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[2]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq4_e111_q_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq4_e111_q_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq4_e111_q_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq4_e111_q_d_n3)),
            ],
        );
        let (eq5_e118, eq5_e118_d_n0, eq5_e118_d_n1, eq5_e118_d_n2, eq5_e118_d_n3, eq5_e118_q, eq5_e118_q_d_n0, eq5_e118_q_d_n1, eq5_e118_q_d_n2, eq5_e118_q_d_n3,) = {
    if (!s.b[258]) {
        let eq5_e115_q: f64 = s.rv[201];
        let eq5_e116: f64 = (p.p0 * s.v[201]);
        let eq5_e116_d_n0: f64 = (p.p0 * s.dn[201][0]);
        let eq5_e116_d_n1: f64 = (p.p0 * s.dn[201][1]);
        let eq5_e116_d_n2: f64 = (p.p0 * s.dn[201][2]);
        let eq5_e116_d_n3: f64 = (p.p0 * s.dn[201][3]);
        let eq5_e116_q: f64 = (p.p0 * eq5_e115_q);
        let eq5_e116_q_d_n0: f64 = (p.p0 * s.rdn[201][0]);
        let eq5_e116_q_d_n1: f64 = (p.p0 * s.rdn[201][1]);
        let eq5_e116_q_d_n2: f64 = (p.p0 * s.rdn[201][2]);
        let eq5_e116_q_d_n3: f64 = (p.p0 * s.rdn[201][3]);
        (eq5_e116, eq5_e116_d_n0, eq5_e116_d_n1, eq5_e116_d_n2, eq5_e116_d_n3, eq5_e116_q, eq5_e116_q_d_n0, eq5_e116_q_d_n1, eq5_e116_q_d_n2, eq5_e116_q_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[0]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq5_e118_q_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq5_e118_q_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq5_e118_q_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq5_e118_q_d_n3)),
            ],
        );
        let eq7_e128_q: f64 = s.v[106];
        let eq7_e129: f64 = (p.p0 * s.v[106]);
        let eq7_e129_d_n0: f64 = (p.p0 * s.dn[106][0]);
        let eq7_e129_d_n1: f64 = (p.p0 * s.dn[106][1]);
        let eq7_e129_d_n2: f64 = (p.p0 * s.dn[106][2]);
        let eq7_e129_d_n3: f64 = (p.p0 * s.dn[106][3]);
        let eq7_e129_q: f64 = (p.p0 * eq7_e128_q);
        let eq7_e129_q_d_n0: f64 = (p.p0 * s.dn[106][0]);
        let eq7_e129_q_d_n1: f64 = (p.p0 * s.dn[106][1]);
        let eq7_e129_q_d_n2: f64 = (p.p0 * s.dn[106][2]);
        let eq7_e129_q_d_n3: f64 = (p.p0 * s.dn[106][3]);
        stamper.stamp_current_reactive(
            Some(nodes[1]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq7_e129_q_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq7_e129_q_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq7_e129_q_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq7_e129_q_d_n3)),
            ],
        );
        let eq11_e178_q: f64 = s.v[236];
        let eq11_e180: f64 = (s.v[236] * p.p0);
        let eq11_e180_d_n0: f64 = (s.dn[236][0] * p.p0);
        let eq11_e180_d_n1: f64 = (s.dn[236][1] * p.p0);
        let eq11_e180_d_n2: f64 = (s.dn[236][2] * p.p0);
        let eq11_e180_d_n3: f64 = (s.dn[236][3] * p.p0);
        let eq11_e180_q: f64 = (eq11_e178_q * p.p0);
        let eq11_e180_q_d_n0: f64 = (s.dn[236][0] * p.p0);
        let eq11_e180_q_d_n1: f64 = (s.dn[236][1] * p.p0);
        let eq11_e180_q_d_n2: f64 = (s.dn[236][2] * p.p0);
        let eq11_e180_q_d_n3: f64 = (s.dn[236][3] * p.p0);
        let eq11_e182: f64 = (eq11_e180 * p.p7);
        let eq11_e182_d_n0: f64 = (eq11_e180_d_n0 * p.p7);
        let eq11_e182_d_n1: f64 = (eq11_e180_d_n1 * p.p7);
        let eq11_e182_d_n2: f64 = (eq11_e180_d_n2 * p.p7);
        let eq11_e182_d_n3: f64 = (eq11_e180_d_n3 * p.p7);
        let eq11_e182_q: f64 = (eq11_e180_q * p.p7);
        let eq11_e182_q_d_n0: f64 = (eq11_e180_q_d_n0 * p.p7);
        let eq11_e182_q_d_n1: f64 = (eq11_e180_q_d_n1 * p.p7);
        let eq11_e182_q_d_n2: f64 = (eq11_e180_q_d_n2 * p.p7);
        let eq11_e182_q_d_n3: f64 = (eq11_e180_q_d_n3 * p.p7);
        stamper.stamp_current_reactive(
            Some(nodes[0]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq11_e182_q_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq11_e182_q_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq11_e182_q_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq11_e182_q_d_n3)),
            ],
        );
        let eq12_e184_q: f64 = s.v[237];
        let eq12_e186: f64 = (s.v[237] * p.p0);
        let eq12_e186_d_n0: f64 = (s.dn[237][0] * p.p0);
        let eq12_e186_d_n1: f64 = (s.dn[237][1] * p.p0);
        let eq12_e186_d_n2: f64 = (s.dn[237][2] * p.p0);
        let eq12_e186_d_n3: f64 = (s.dn[237][3] * p.p0);
        let eq12_e186_q: f64 = (eq12_e184_q * p.p0);
        let eq12_e186_q_d_n0: f64 = (s.dn[237][0] * p.p0);
        let eq12_e186_q_d_n1: f64 = (s.dn[237][1] * p.p0);
        let eq12_e186_q_d_n2: f64 = (s.dn[237][2] * p.p0);
        let eq12_e186_q_d_n3: f64 = (s.dn[237][3] * p.p0);
        let eq12_e188: f64 = (eq12_e186 * p.p7);
        let eq12_e188_d_n0: f64 = (eq12_e186_d_n0 * p.p7);
        let eq12_e188_d_n1: f64 = (eq12_e186_d_n1 * p.p7);
        let eq12_e188_d_n2: f64 = (eq12_e186_d_n2 * p.p7);
        let eq12_e188_d_n3: f64 = (eq12_e186_d_n3 * p.p7);
        let eq12_e188_q: f64 = (eq12_e186_q * p.p7);
        let eq12_e188_q_d_n0: f64 = (eq12_e186_q_d_n0 * p.p7);
        let eq12_e188_q_d_n1: f64 = (eq12_e186_q_d_n1 * p.p7);
        let eq12_e188_q_d_n2: f64 = (eq12_e186_q_d_n2 * p.p7);
        let eq12_e188_q_d_n3: f64 = (eq12_e186_q_d_n3 * p.p7);
        stamper.stamp_current_reactive(
            Some(nodes[2]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq12_e188_q_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq12_e188_q_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq12_e188_q_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq12_e188_q_d_n3)),
            ],
        );
    }
}
