#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.store_scaled_voltage(202, ctx, nodes, Some(8), Some(6), p.p148);

        s.store_scaled_voltage(203, ctx, nodes, Some(8), Some(5), p.p148);

        s.store_sub(204, 202, 203);

        s.store_scaled_voltage(205, ctx, nodes, Some(7), Some(6), p.p148);

        s.store_scaled_voltage(206, ctx, nodes, Some(7), Some(5), p.p148);

        s.store_scaled_voltage(207, ctx, nodes, Some(1), Some(5), p.p148);

        s.store_scaled_voltage(208, ctx, nodes, Some(9), Some(5), p.p148);

        s.store_scaled_voltage(209, ctx, nodes, Some(3), Some(0), p.p148);

        s.b[279] = (p.p0 <= 310.0);
        s.store_scalar(279, if s.b[279] { 1.0 } else { 0.0 });

        if s.b[279] {
            s.store_scalar(0, 1.6021918e-19);
            s.store_scalar(1, 1.3806226e-23);
        }

        if (!s.b[279]) {
            s.store_scalar(0, 1.602176634e-19);
            s.store_scalar(1, 1.380649e-23);
        }

        s.store_scalar(8, (p.p146 + 273.15));

        s.store_scalar(9, ctx_temp);

        s.store_div(2, 1, 0);

        s.store_scale(3, 2, 300.0);

        s.store_scale(6, 2, s.v[8]);

        s.store_div_from_scalar(7, 1.0, 6);

        s.store_scalar(276, ((p.p121 * s.v[8]) * ((s.v[8]) as f64).ln()));

        s.store_scalar(277, (p.p122 * s.v[8]));

        s.store_scalar(56, (p.p131 * s.v[8]));

        s.store_scalar(88, ((p.p117 + s.v[276]) + s.v[277]));

        s.store_scalar(89, ((p.p118 + s.v[276]) + s.v[277]));

        s.store_scalar(90, ((p.p119 + s.v[276]) + s.v[277]));

        s.store_scalar(91, ((s.v[88] + s.v[89]) * 0.5));

        s.store_scalar(92, ((s.v[88] + s.v[90]) * 0.5));

        s.store_scalar(77, ((p.p117 + p.p118) * 0.5));

        s.store_scalar(78, ((p.p117 + p.p119) * 0.5));

        s.store_scalar(79, ((p.p120 + p.p119) * 0.5));

        s.store_sub_from_scalar_ad(76, 3.0, A::div_from_scalar(p.p121, s.ad_value(2)));

        s.store_offset(82, 76, (-1.5));

        s.store_scalar(278, ((1.0 - p.p107) * (p.p52 + p.p106)));

        s.b[280] = (s.v[278] >= p.p106);
        s.store_scalar(280, if s.b[280] { 1.0 } else { 0.0 });

        if s.b[280] {
            s.store_scalar(176, (s.v[278] - p.p106));
            s.store_sub_from_scalar(177, p.p52, 176);
        }

        if (!s.b[280]) {
            s.store_scalar(176, 0.0);
            s.store_scalar(177, p.p52);
        }

        s.b[282] = (p.p0 <= 300.0);
        s.store_scalar(282, if s.b[282] { 1.0 } else { 0.0 });

        if s.b[282] {
            s.store_scalar(223, 0.0);
        }

        if (!s.b[282]) {
            s.store_scalar(223, 0.7);
        }

        s.store_scalar(234, p.p86);

        s.b[284] = (p.p86 != 0.0);
        s.store_scalar(284, if s.b[284] { 1.0 } else { 0.0 });

        s.b[285] = (((p.p88 == 0.0) && (p.p87 == 0.0)) || (p.p66 == 0.0));
        s.store_scalar(285, if s.b[285] { 1.0 } else { 0.0 });

        if (s.b[284] && s.b[285]) {
            s.store_scalar(234, 0.0);
        }

        s.b[286] = ((p.p115 >= 0.01) || (p.p116 >= 0.01));
        s.store_scalar(286, if s.b[286] { 1.0 } else { 0.0 });

        if s.b[286] {
            s.store_scalar(232, (0.5 * (p.p115 - p.p116)));
        }

        s.b[287] = (p.p116 < p.p115);
        s.store_scalar(287, if s.b[287] { 1.0 } else { 0.0 });

        if (s.b[286] && s.b[287]) {
            s.store_scalar(229, p.p116);
            s.store_scalar(230, p.p115);
        }

        if (s.b[286] && (!s.b[287])) {
            s.store_scalar(229, p.p115);
            s.store_scalar(230, p.p116);
        }

        s.b[288] = (s.v[229] < 0.01);
        s.store_scalar(288, if s.b[288] { 1.0 } else { 0.0 });

        if (s.b[286] && s.b[288]) {
            s.store_scalar(225, 1000000000.0);
            s.store_scalar(226, 1000000000.0);
            s.store_scalar(227, 170000000.0);
            s.store_scalar(228, 170000000.0);
            s.store_ln_offset_input(231, 230, 1.0);
        }

        if (s.b[286] && (!s.b[288])) {
            s.store_scalar(225, (1.0 / p.p115));
            s.store_scalar(226, (1.0 / p.p116));
            s.store_scalar(227, (p.p115 / 6.0));
            s.store_scalar(228, (p.p116 / 6.0));
            s.store_scalar(231, ((((1.0 + p.p115) / (1.0 + p.p116))) as f64).ln());
        }

        if (!s.b[286]) {
            s.store_scalar(232, 0.0);
            s.store_scalar(225, 1000000000.0);
            s.store_scalar(226, 1000000000.0);
            s.store_scalar(227, 170000000.0);
            s.store_scalar(228, 170000000.0);
            s.store_scalar(229, p.p116);
            s.store_scalar(230, p.p115);
            s.store_scalar(231, 0.0);
        }

        s.store_scalar(10, (s.v[9] + p.p147));

        s.b[289] = (s.v[10] < ((-200.0) + 273.15));
        s.store_scalar(289, if s.b[289] { 1.0 } else { 0.0 });

        if s.b[289] {
            s.store_scalar(10, ((-200.0) + 273.15));
        }

        s.b[290] = (s.v[10] > (326.85 + 273.15));
        s.store_scalar(290, if s.b[290] { 1.0 } else { 0.0 });

        if ((!s.b[289]) && s.b[290]) {
            s.store_scalar(10, (326.85 + 273.15));
        }

        s.store_mul(4, 2, 10);

        s.store_div_from_scalar(5, 1.0, 4);

        s.store_offset(14, 10, (-s.v[8]));

        s.store_div_from_scalar(12, s.v[8], 10);

        s.store_scale(11, 10, 1.0 / (s.v[8]));

        s.store_ln(13, 11);

        s.store_mul_scaled_ln_rhs(74, 10, p.p121, 10);

        s.store_scale(75, 10, p.p122);

        s.store_add_offset_lhs(84, 74, p.p117, 75);

        s.store_add_offset_lhs(83, 74, p.p118, 75);

        s.store_add_offset_lhs(85, 74, p.p119, 75);

        s.store_scaled_add(86, 84, 83, 0.5);

        s.store_scaled_add(87, 84, 85, 0.5);

        s.b[291] = (p.p39 > 0.0);
        s.store_scalar(291, if s.b[291] { 1.0 } else { 0.0 });

        if s.b[291] {
            s.store_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p.p40 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p40))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[77]), s.v[77]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(27, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(26, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p40, s.ad_value(27))), p.p41), p.p39);
            s.store_scalar(28, ((p.p42) as f64).abs());
        }

        s.b[292] = (p.p42 > 0.0);
        s.store_scalar(292, if s.b[292] { 1.0 } else { 0.0 });

        if (s.b[291] && s.b[292]) {
            s.store_scale(28, 27, (p.p42 * 1.0 / (p.p40)));
        }

        if (!s.b[291]) {
            s.store_scalar(26, p.p39);
            s.store_scalar(27, p.p40);
            s.store_scalar(28, p.p42);
        }

        s.store_scaled_exp_ad(22, A::add_scaled_inputs(s.ad_value(13), p.p124, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p.p118), 1.0), p.p14);

        s.b[293] = (p.p47 > 0.0);
        s.store_scalar(293, if s.b[293] { 1.0 } else { 0.0 });

        if s.b[293] {
            s.store_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p.p48 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p48))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[78]), s.v[78]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(34, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(33, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p48, s.ad_value(34))), p.p49), p.p47);
            s.store_scalar(35, ((p.p50) as f64).abs());
        }

        s.b[294] = (p.p50 > 0.0);
        s.store_scalar(294, if s.b[294] { 1.0 } else { 0.0 });

        if (s.b[293] && s.b[294]) {
            s.store_scale(35, 34, (p.p50 * 1.0 / (p.p48)));
        }

        if (!s.b[293]) {
            s.store_scalar(33, p.p47);
            s.store_scalar(34, p.p48);
            s.store_scalar(35, p.p50);
        }

        s.b[295] = (p.p0 <= 300.0);
        s.store_scalar(295, if s.b[295] { 1.0 } else { 0.0 });

        if s.b[295] {
            s.store_scalar(35, 2.4);
        }

        s.store_offset_scaled_ad(16, A::exp_scaled_input(A::ln_scaled_input(s.ad_value(27), 1.0 / (p.p40)), p.p41), (-p.p2), ((2.0) * (p.p2)));

        s.store_scaled_exp_ad(15, A::add_scaled_inputs(s.ad_value(13), p.p123, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p.p117), 1.0), p.p1);

        s.store_scaled_exp_scaled_input(18, 13, p.p126, p.p10);

        s.b[296] = ((p.p0 <= 300.0) && ((((p.p8 - 1.0)) as f64).abs() < 1e-5));
        s.store_scalar(296, if s.b[296] { 1.0 } else { 0.0 });

        if s.b[296] {
            s.store_scaled_exp_ad(17, A::mul_scaled_lhs(s.ad_value(5), p.p125, A::offset(A::exp_scaled_input(s.ad_value(13), p.p127), (-1.0))), p.p9);
        }

        if (!s.b[296]) {
            s.store_scaled_exp_ad(17, A::mul_scaled_lhs(s.ad_value(5), p.p125, A::offset(A::exp_scaled_input(s.ad_value(13), p.p127), (-1.0))), p.p8);
        }

        s.store_scaled_exp_ad(19, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p.p125), p.p3);

        s.store_scaled_exp_ad(20, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), (p.p117 - p.p118)), p.p4);

        s.store_scaled_exp_ad(21, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), (p.p117 - p.p119)), p.p6);

        s.store_scaled_exp_scaled_input(55, 13, (p.p130 - s.v[56]), p.p75);

        s.store_scaled_exp_scaled_input(53, 13, p.p130, p.p74);

        s.store_div_from_scalar(54, 1.0, 53);

        s.b[297] = (p.p79 > 0.0);
        s.store_scalar(297, if s.b[297] { 1.0 } else { 0.0 });

        if s.b[297] {
            s.store_offset_scaled_ad(58, A::scale(s.ad_value(14), p.p133), (-p.p79), p.p79);
            s.store_scalar(57, p.p78);
        }

        if (!s.b[297]) {
            s.store_offset_scaled(57, 14, ((p.p132) * (p.p78)), p.p78);
            s.store_scalar(58, p.p79);
        }

        s.store_add_scaled_product_value_ad(59, A::scale_offset(s.ad_value(14), p.p128, 1.0), p.p66, 14, 14, (p.p129 * p.p66));

        s.store_scalar(61, p.p69);

        s.store_scaled_exp_scaled_input(60, 13, (p.p130 - 1.0), p.p71);

        s.b[299] = ((p.p37 > 0.0) && (s.v[203] < 0.0));
        s.store_scalar(299, if s.b[299] { 1.0 } else { 0.0 });

        if s.b[299] {
            s.store_scalar(67, p.p37);
        }

        s.b[300] = ((p.p47 > 0.0) && (p.p48 > 0.0));
        s.store_scalar(300, if s.b[300] { 1.0 } else { 0.0 });

        if (s.b[299] && s.b[300]) {
            s.store_div_from_scalar(169, s.v[92], 87);
            s.store_scale(170, 34, 1.0 / (p.p48));
            s.store_mul_ad_affine_product_lhs(168, A::sqrt(s.ad_value(169)), s.ad_value(170), 1.0 / (p.p47), 0.0, 33);
            s.store_scaled_mul(67, 168, 170, p.p37);
        }

        if (!s.b[299]) {
            s.store_scalar(67, 0.0);
        }

        s.b[301] = (p.p43 > 0.0);
        s.store_scalar(301, if s.b[301] { 1.0 } else { 0.0 });

        if s.b[301] {
            s.store_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p.p44 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p44))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[77]), s.v[77]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(30, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(29, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p44, s.ad_value(30))), p.p45), p.p43);
            s.store_scalar(31, ((p.p46) as f64).abs());
        }

        s.b[302] = (p.p46 > 0.0);
        s.store_scalar(302, if s.b[302] { 1.0 } else { 0.0 });

        if (s.b[301] && s.b[302]) {
            s.store_scale(31, 30, (p.p46 * 1.0 / (p.p44)));
        }

        if (!s.b[301]) {
            s.store_scalar(29, p.p43);
            s.store_scalar(30, p.p44);
            s.store_scalar(31, p.p46);
        }

        s.b[303] = ((p.p27 > 0.0) && ((s.v[205] < s.v[223]) || (s.v[202] < s.v[223])));
        s.store_scalar(303, if s.b[303] { 1.0 } else { 0.0 });

        if s.b[303] {
            s.store_scalar(166, 1.0);
            s.store_scalar(167, 1.0);
            s.store_div_from_scalar(169, s.v[91], 86);
        }

        s.b[304] = (((p.p29 == 1.0) && (p.p43 > 0.0)) && (p.p44 > 0.0));
        s.store_scalar(304, if s.b[304] { 1.0 } else { 0.0 });

        if (s.b[303] && s.b[304]) {
            s.store_scale(170, 30, 1.0 / (p.p44));
            s.store_mul_product3_mixed_iiai(167, 170, 29, A::sqrt(s.ad_value(169)), 170, 1.0 / (p.p43));
            s.store_div_scaled_value_by_product(166, A::powf(s.ad_value(169), (-1.5)), p.p43, s.ad_value(29), s.ad_value(170), 1.0);
        }

        s.b[305] = (((p.p29 == 0.0) && (p.p39 > 0.0)) && (p.p40 > 0.0));
        s.store_scalar(305, if s.b[305] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[303] && (!s.b[304])) && s.b[305]) {
            s.store_scale(170, 27, 1.0 / (p.p40));
            s.store_mul_product3_mixed_iiai(167, 170, 26, A::sqrt(s.ad_value(169)), 170, 1.0 / (p.p39));
            s.store_div_scaled_value_by_product(166, A::powf(s.ad_value(169), (-1.5)), p.p39, s.ad_value(26), s.ad_value(170), 1.0);
        }

        s.b[306] = (1.0 > 0.0);
        s.store_scalar(306, if s.b[306] { 1.0 } else { 0.0 });

        if s.b[306] {
            s.store_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p.p53 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p53))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[78]), s.v[78]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(39, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_exp_scaled_input_ad(43, A::ln(A::div_from_scalar(p.p53, s.ad_value(39))), p.p54);
            s.store_scalar(40, ((p.p55) as f64).abs());
        }

        s.b[307] = (p.p55 > 0.0);
        s.store_scalar(307, if s.b[307] { 1.0 } else { 0.0 });

        if (s.b[306] && s.b[307]) {
            s.store_scale(40, 39, (p.p55 * 1.0 / (p.p53)));
        }

        if (!s.b[306]) {
            s.store_scalar(43, 1.0);
            s.store_scalar(39, p.p53);
            s.store_scalar(40, p.p55);
        }

        s.b[308] = (p.p0 <= 300.0);
        s.store_scalar(308, if s.b[308] { 1.0 } else { 0.0 });

        if s.b[308] {
            s.store_scalar(40, 2.4);
        }

        s.store_mul(37, 43, 176);

        s.store_mul(38, 43, 177);

        s.b[309] = (p.p0 <= 300.0);
        s.store_scalar(309, if s.b[309] { 1.0 } else { 0.0 });

        s.b[310] = (p.p57 > 0.0);
        s.store_scalar(310, if s.b[310] { 1.0 } else { 0.0 });

        if (s.b[309] && s.b[310]) {
            s.store_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p.p58 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p58))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(47, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(46, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p58, s.ad_value(47))), p.p59), p.p57);
            s.store_scalar(48, (((-2.4)) as f64).abs());
        }

        s.b[311] = ((-2.4) > 0.0);
        s.store_scalar(311, if s.b[311] { 1.0 } else { 0.0 });

        if ((s.b[309] && s.b[310]) && s.b[311]) {
            s.store_scale(48, 47, ((-2.4) * 1.0 / (p.p58)));
        }

        if (s.b[309] && (!s.b[310])) {
            s.store_scalar(46, p.p57);
            s.store_scalar(47, p.p58);
            s.store_scalar(48, (-2.4));
        }

        if s.b[309] {
            s.store_scalar(163, 2.4);
        }

        s.b[312] = (p.p57 > 0.0);
        s.store_scalar(312, if s.b[312] { 1.0 } else { 0.0 });

        if ((!s.b[309]) && s.b[312]) {
            s.store_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p.p58 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p58))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(47, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(46, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p58, s.ad_value(47))), p.p59), p.p57);
            s.store_scalar(48, (((-p.p60)) as f64).abs());
        }

        s.b[313] = ((-p.p60) > 0.0);
        s.store_scalar(313, if s.b[313] { 1.0 } else { 0.0 });

        if (((!s.b[309]) && s.b[312]) && s.b[313]) {
            s.store_scale(48, 47, ((-p.p60) * 1.0 / (p.p58)));
        }

        if ((!s.b[309]) && (!s.b[312])) {
            s.store_scalar(46, p.p57);
            s.store_scalar(47, p.p58);
            s.store_scalar(48, (-p.p60));
        }

        if (!s.b[309]) {
            s.store_scalar(163, p.p60);
        }

        s.store_scaled_exp_ad(44, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p.p119), 1.0, s.ad_value(82), s.ad_value(13), 1.0), p.p97);

        s.store_scaled_exp_scaled_input(52, 13, (p.p138 - 1.0), p.p101);

        s.b[314] = (p.p63 > 0.0);
        s.store_scalar(314, if s.b[314] { 1.0 } else { 0.0 });

        s.b[315] = (p.p62 > 0.0);
        s.store_scalar(315, if s.b[315] { 1.0 } else { 0.0 });

        if (s.b[314] && s.b[315]) {
            s.store_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p.p63 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p63))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(50, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(49, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p63, s.ad_value(50))), p.p64), p.p62);
            s.store_abs_scaled_input(51, 163, -1.0);
        }

        s.b[316] = ((-s.v[163]) > 0.0);
        s.store_scalar(316, if s.b[316] { 1.0 } else { 0.0 });

        if ((s.b[314] && s.b[315]) && s.b[316]) {
            s.store_scaled_mul(51, 163, 50, (-1.0 / (p.p63)));
        }

        if (s.b[314] && (!s.b[315])) {
            s.store_scalar(49, p.p62);
            s.store_scalar(50, p.p63);
            s.store_neg(51, 163);
        }

        if (!s.b[314]) {
            s.store_scalar(49, p.p62);
            s.store_scalar(50, p.p63);
            s.copy_ad(51, 163);
        }

        s.b[317] = (((p.p141 != 0.0) && (p.p142 >= p.p149)) && (p.p142 > 0.0));
        s.store_scalar(317, if s.b[317] { 1.0 } else { 0.0 });

        if s.b[317] {
            s.store_offset_voltage(10, ctx, nodes, Some(4), None, (s.v[9] + p.p147));
        }

        s.b[318] = (s.v[10] < ((-200.0) + 273.15));
        s.store_scalar(318, if s.b[318] { 1.0 } else { 0.0 });

        if (s.b[317] && s.b[318]) {
            s.store_scalar(10, ((-200.0) + 273.15));
        }

        s.b[319] = (s.v[10] > (326.85 + 273.15));
        s.store_scalar(319, if s.b[319] { 1.0 } else { 0.0 });

        if ((s.b[317] && (!s.b[318])) && s.b[319]) {
            s.store_scalar(10, (326.85 + 273.15));
        }

        if s.b[317] {
            s.store_mul(4, 2, 10);
            s.store_div_from_scalar(5, 1.0, 4);
            s.store_offset(14, 10, (-s.v[8]));
            s.store_div_from_scalar(12, s.v[8], 10);
            s.store_scale(11, 10, 1.0 / (s.v[8]));
            s.store_ln(13, 11);
            s.store_mul_scaled_ln_rhs(74, 10, p.p121, 10);
            s.store_scale(75, 10, p.p122);
            s.store_add_offset_lhs(84, 74, p.p117, 75);
            s.store_add_offset_lhs(83, 74, p.p118, 75);
            s.store_add_offset_lhs(85, 74, p.p119, 75);
            s.store_scaled_add(86, 84, 83, 0.5);
            s.store_scaled_add(87, 84, 85, 0.5);
        }

        s.b[320] = (p.p39 > 0.0);
        s.store_scalar(320, if s.b[320] { 1.0 } else { 0.0 });

        if (s.b[317] && s.b[320]) {
            s.store_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p.p40 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p40))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[77]), s.v[77]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(27, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(26, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p40, s.ad_value(27))), p.p41), p.p39);
            s.store_scalar(28, ((p.p42) as f64).abs());
        }

        s.b[321] = (p.p42 > 0.0);
        s.store_scalar(321, if s.b[321] { 1.0 } else { 0.0 });

        if ((s.b[317] && s.b[320]) && s.b[321]) {
            s.store_scale(28, 27, (p.p42 * 1.0 / (p.p40)));
        }

        if (s.b[317] && (!s.b[320])) {
            s.store_scalar(26, p.p39);
            s.store_scalar(27, p.p40);
            s.store_scalar(28, p.p42);
        }

        if s.b[317] {
            s.store_scaled_exp_ad(22, A::add_scaled_inputs(s.ad_value(13), p.p124, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p.p118), 1.0), p.p14);
        }

        s.b[322] = (p.p47 > 0.0);
        s.store_scalar(322, if s.b[322] { 1.0 } else { 0.0 });

        if (s.b[317] && s.b[322]) {
            s.store_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p.p48 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p48))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[78]), s.v[78]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(34, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(33, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p48, s.ad_value(34))), p.p49), p.p47);
            s.store_scalar(35, ((p.p50) as f64).abs());
        }

        s.b[323] = (p.p50 > 0.0);
        s.store_scalar(323, if s.b[323] { 1.0 } else { 0.0 });

        if ((s.b[317] && s.b[322]) && s.b[323]) {
            s.store_scale(35, 34, (p.p50 * 1.0 / (p.p48)));
        }

        if (s.b[317] && (!s.b[322])) {
            s.store_scalar(33, p.p47);
            s.store_scalar(34, p.p48);
            s.store_scalar(35, p.p50);
        }

        s.b[324] = (p.p0 <= 300.0);
        s.store_scalar(324, if s.b[324] { 1.0 } else { 0.0 });

        if (s.b[317] && s.b[324]) {
            s.store_scalar(35, 2.4);
        }

        if s.b[317] {
            s.store_offset_scaled_ad(16, A::exp_scaled_input(A::ln_scaled_input(s.ad_value(27), 1.0 / (p.p40)), p.p41), (-p.p2), ((2.0) * (p.p2)));
            s.store_scaled_exp_ad(15, A::add_scaled_inputs(s.ad_value(13), p.p123, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p.p117), 1.0), p.p1);
            s.store_scaled_exp_scaled_input(18, 13, p.p126, p.p10);
        }

        s.b[325] = ((p.p0 <= 300.0) && ((((p.p8 - 1.0)) as f64).abs() < 1e-5));
        s.store_scalar(325, if s.b[325] { 1.0 } else { 0.0 });

        if (s.b[317] && s.b[325]) {
            s.store_scaled_exp_ad(17, A::mul_scaled_lhs(s.ad_value(5), p.p125, A::offset(A::exp_scaled_input(s.ad_value(13), p.p127), (-1.0))), p.p9);
        }

        if (s.b[317] && (!s.b[325])) {
            s.store_scaled_exp_ad(17, A::mul_scaled_lhs(s.ad_value(5), p.p125, A::offset(A::exp_scaled_input(s.ad_value(13), p.p127), (-1.0))), p.p8);
        }

        if s.b[317] {
            s.store_scaled_exp_ad(19, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p.p125), p.p3);
            s.store_scaled_exp_ad(20, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), (p.p117 - p.p118)), p.p4);
            s.store_scaled_exp_ad(21, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), (p.p117 - p.p119)), p.p6);
            s.store_scaled_exp_scaled_input(55, 13, (p.p130 - s.v[56]), p.p75);
            s.store_scaled_exp_scaled_input(53, 13, p.p130, p.p74);
            s.store_div_from_scalar(54, 1.0, 53);
        }

        s.b[326] = (p.p79 > 0.0);
        s.store_scalar(326, if s.b[326] { 1.0 } else { 0.0 });

        if (s.b[317] && s.b[326]) {
            s.store_offset_scaled_ad(58, A::scale(s.ad_value(14), p.p133), (-p.p79), p.p79);
            s.store_scalar(57, p.p78);
        }

        if (s.b[317] && (!s.b[326])) {
            s.store_offset_scaled(57, 14, ((p.p132) * (p.p78)), p.p78);
            s.store_scalar(58, p.p79);
        }

        if s.b[317] {
            s.store_add_scaled_product_value_ad(59, A::scale_offset(s.ad_value(14), p.p128, 1.0), p.p66, 14, 14, (p.p129 * p.p66));
            s.store_scalar(61, p.p69);
            s.store_scaled_exp_scaled_input(60, 13, (p.p130 - 1.0), p.p71);
        }

        s.b[328] = ((p.p37 > 0.0) && (s.v[203] < 0.0));
        s.store_scalar(328, if s.b[328] { 1.0 } else { 0.0 });

        if (s.b[317] && s.b[328]) {
            s.store_scalar(67, p.p37);
        }

        s.b[329] = ((p.p47 > 0.0) && (p.p48 > 0.0));
        s.store_scalar(329, if s.b[329] { 1.0 } else { 0.0 });

        if ((s.b[317] && s.b[328]) && s.b[329]) {
            s.store_div_from_scalar(169, s.v[92], 87);
            s.store_scale(170, 34, 1.0 / (p.p48));
            s.store_mul_ad_affine_product_lhs(168, A::sqrt(s.ad_value(169)), s.ad_value(170), 1.0 / (p.p47), 0.0, 33);
            s.store_scaled_mul(67, 168, 170, p.p37);
        }

        if (s.b[317] && (!s.b[328])) {
            s.store_scalar(67, 0.0);
        }

        s.b[330] = (p.p43 > 0.0);
        s.store_scalar(330, if s.b[330] { 1.0 } else { 0.0 });

        if (s.b[317] && s.b[330]) {
            s.store_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p.p44 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p44))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[77]), s.v[77]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(30, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
        }

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[317] && s.b[330]) {
            s.store_scale_ad(29, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p44, s.ad_value(30))), p.p45), p.p43);
            s.store_scalar(31, ((p.p46) as f64).abs());
        }

        s.b[331] = (p.p46 > 0.0);
        s.store_scalar(331, if s.b[331] { 1.0 } else { 0.0 });

        if ((s.b[317] && s.b[330]) && s.b[331]) {
            s.store_scale(31, 30, (p.p46 * 1.0 / (p.p44)));
        }

        if (s.b[317] && (!s.b[330])) {
            s.store_scalar(29, p.p43);
            s.store_scalar(30, p.p44);
            s.store_scalar(31, p.p46);
        }

        s.b[332] = ((p.p27 > 0.0) && ((s.v[205] < s.v[223]) || (s.v[202] < s.v[223])));
        s.store_scalar(332, if s.b[332] { 1.0 } else { 0.0 });

        if (s.b[317] && s.b[332]) {
            s.store_scalar(166, 1.0);
            s.store_scalar(167, 1.0);
            s.store_div_from_scalar(169, s.v[91], 86);
        }

        s.b[333] = (((p.p29 == 1.0) && (p.p43 > 0.0)) && (p.p44 > 0.0));
        s.store_scalar(333, if s.b[333] { 1.0 } else { 0.0 });

        if ((s.b[317] && s.b[332]) && s.b[333]) {
            s.store_scale(170, 30, 1.0 / (p.p44));
            s.store_mul_product3_mixed_iiai(167, 170, 29, A::sqrt(s.ad_value(169)), 170, 1.0 / (p.p43));
            s.store_div_scaled_value_by_product(166, A::powf(s.ad_value(169), (-1.5)), p.p43, s.ad_value(29), s.ad_value(170), 1.0);
        }

        s.b[334] = (((p.p29 == 0.0) && (p.p39 > 0.0)) && (p.p40 > 0.0));
        s.store_scalar(334, if s.b[334] { 1.0 } else { 0.0 });

        if (((s.b[317] && s.b[332]) && (!s.b[333])) && s.b[334]) {
            s.store_scale(170, 27, 1.0 / (p.p40));
            s.store_mul_product3_mixed_iiai(167, 170, 26, A::sqrt(s.ad_value(169)), 170, 1.0 / (p.p39));
            s.store_div_scaled_value_by_product(166, A::powf(s.ad_value(169), (-1.5)), p.p39, s.ad_value(26), s.ad_value(170), 1.0);
        }

        s.b[335] = (1.0 > 0.0);
        s.store_scalar(335, if s.b[335] { 1.0 } else { 0.0 });

        if (s.b[317] && s.b[335]) {
            s.store_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p.p53 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p53))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[78]), s.v[78]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(39, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_exp_scaled_input_ad(43, A::ln(A::div_from_scalar(p.p53, s.ad_value(39))), p.p54);
            s.store_scalar(40, ((p.p55) as f64).abs());
        }

        s.b[336] = (p.p55 > 0.0);
        s.store_scalar(336, if s.b[336] { 1.0 } else { 0.0 });

        if ((s.b[317] && s.b[335]) && s.b[336]) {
            s.store_scale(40, 39, (p.p55 * 1.0 / (p.p53)));
        }

        if (s.b[317] && (!s.b[335])) {
            s.store_scalar(43, 1.0);
            s.store_scalar(39, p.p53);
            s.store_scalar(40, p.p55);
        }

        s.b[337] = (p.p0 <= 300.0);
        s.store_scalar(337, if s.b[337] { 1.0 } else { 0.0 });

        if (s.b[317] && s.b[337]) {
            s.store_scalar(40, 2.4);
        }

        if s.b[317] {
            s.store_mul(37, 43, 176);
            s.store_mul(38, 43, 177);
        }

        s.b[338] = (p.p0 <= 300.0);
        s.store_scalar(338, if s.b[338] { 1.0 } else { 0.0 });

        s.b[339] = (p.p57 > 0.0);
        s.store_scalar(339, if s.b[339] { 1.0 } else { 0.0 });

        if ((s.b[317] && s.b[338]) && s.b[339]) {
            s.store_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p.p58 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p58))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(47, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(46, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p58, s.ad_value(47))), p.p59), p.p57);
            s.store_scalar(48, (((-2.4)) as f64).abs());
        }

        s.b[340] = ((-2.4) > 0.0);
        s.store_scalar(340, if s.b[340] { 1.0 } else { 0.0 });

        if (((s.b[317] && s.b[338]) && s.b[339]) && s.b[340]) {
            s.store_scale(48, 47, ((-2.4) * 1.0 / (p.p58)));
        }

        if ((s.b[317] && s.b[338]) && (!s.b[339])) {
            s.store_scalar(46, p.p57);
            s.store_scalar(47, p.p58);
            s.store_scalar(48, (-2.4));
        }

        if (s.b[317] && s.b[338]) {
            s.store_scalar(163, 2.4);
        }

        s.b[341] = (p.p57 > 0.0);
        s.store_scalar(341, if s.b[341] { 1.0 } else { 0.0 });

        if ((s.b[317] && (!s.b[338])) && s.b[341]) {
            s.store_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p.p58 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p58))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(47, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(46, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p58, s.ad_value(47))), p.p59), p.p57);
            s.store_scalar(48, (((-p.p60)) as f64).abs());
        }

        s.b[342] = ((-p.p60) > 0.0);
        s.store_scalar(342, if s.b[342] { 1.0 } else { 0.0 });

        if (((s.b[317] && (!s.b[338])) && s.b[341]) && s.b[342]) {
            s.store_scale(48, 47, ((-p.p60) * 1.0 / (p.p58)));
        }

        if ((s.b[317] && (!s.b[338])) && (!s.b[341])) {
            s.store_scalar(46, p.p57);
            s.store_scalar(47, p.p58);
            s.store_scalar(48, (-p.p60));
        }

        if (s.b[317] && (!s.b[338])) {
            s.store_scalar(163, p.p60);
        }

        if s.b[317] {
            s.store_scaled_exp_ad(44, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p.p119), 1.0, s.ad_value(82), s.ad_value(13), 1.0), p.p97);
            s.store_scaled_exp_scaled_input(52, 13, (p.p138 - 1.0), p.p101);
        }

        s.b[343] = (p.p63 > 0.0);
        s.store_scalar(343, if s.b[343] { 1.0 } else { 0.0 });

        s.b[344] = (p.p62 > 0.0);
        s.store_scalar(344, if s.b[344] { 1.0 } else { 0.0 });

        if ((s.b[317] && s.b[343]) && s.b[344]) {
            s.store_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p.p63 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p63))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(50, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(49, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p63, s.ad_value(50))), p.p64), p.p62);
            s.store_abs_scaled_input(51, 163, -1.0);
        }

        s.b[345] = ((-s.v[163]) > 0.0);
        s.store_scalar(345, if s.b[345] { 1.0 } else { 0.0 });

        if (((s.b[317] && s.b[343]) && s.b[344]) && s.b[345]) {
            s.store_scaled_mul(51, 163, 50, (-1.0 / (p.p63)));
        }

        if ((s.b[317] && s.b[343]) && (!s.b[344])) {
            s.store_scalar(49, p.p62);
            s.store_scalar(50, p.p63);
            s.store_neg(51, 163);
        }

        if (s.b[317] && (!s.b[343])) {
            s.store_scalar(49, p.p62);
            s.store_scalar(50, p.p63);
            s.copy_ad(51, 163);
        }

        s.b[364] = (p.p14 > 0.0);
        s.store_scalar(364, if s.b[364] { 1.0 } else { 0.0 });

        if s.b[364] {
            s.store_div_scaled_inputs_indices(93, 202, 1.0, 4, p.p15);
        }

        s.b[365] = (s.v[93] > 80.0);
        s.store_scalar(365, if s.b[365] { 1.0 } else { 0.0 });

        if (s.b[364] && s.b[365]) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
            s.store_scalar(93, 80.0);
        }

        if (s.b[364] && (!s.b[365])) {
            s.store_scalar(94, 1.0);
        }

        if s.b[364] {
            s.store_mul_offset_ad_rhs(185, 22, A::mul(s.ad_value(94), A::limexp(s.ad_value(93))), (-1.0));
        }

        if (!s.b[364]) {
            s.store_scalar(185, 0.0);
        }

        s.b[366] = (p.p16 > 0.0);
        s.store_scalar(366, if s.b[366] { 1.0 } else { 0.0 });

        if s.b[366] {
            s.store_div_scaled_inputs_indices(93, 202, 1.0, 4, p.p17);
        }

        s.b[367] = (s.v[93] > 80.0);
        s.store_scalar(367, if s.b[367] { 1.0 } else { 0.0 });

        if (s.b[366] && s.b[367]) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
            s.store_scalar(93, 80.0);
        }

        if (s.b[366] && (!s.b[367])) {
            s.store_scalar(94, 1.0);
        }

        s.store_mul_ad_rhs(350, 15, A::limexp_scaled_input(A::mul(s.ad_value(202), s.ad_value(5)), 1.0 / (p.p13)));

        s.store_mul_limexp_ad_rhs(351, 15, A::mul(s.ad_value(203), s.ad_value(5)));

        s.b[368] = (s.v[26] > 0.0);
        s.store_scalar(368, if s.b[368] { 1.0 } else { 0.0 });

        if s.b[368] {
            s.store_mul_sub_from_scalar_ad_rhs(137, 27, 1.0, A::exp_scaled_input(A::ln(s.ad_value(28)), (-1.0 / (p.p41))));
            s.store_mul_sub_lhs(141, 137, 202, 5);
            s.store_sqrt_square_offset(142, 141, 1.921812);
            s.store_scaled_add(143, 141, 142, 0.5);
            s.store_add_scaled_product_indices(138, 137, 1.0, 4, 143, (-1.0));
            s.store_div(144, 143, 142);
            s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(27))));
            s.store_mul_ad_lhs(145, A::exp_scaled_input(s.ad_value(139), (-p.p41)), 144);
            s.store_mul_add_ad_rhs(211, 26, s.ad_value(145), A::mul_sub_from_scalar_rhs(s.ad_value(28), 1.0, s.ad_value(144)));
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(140, 27, 1.0, A::exp_scaled_input(s.ad_value(139), (1.0 - p.p41)), 1.0 / ((1.0 - p.p41)));
            s.store_mul_add_scaled_product_rhs(179, 26, s.ad_value(140), 1.0, s.ad_value(28), A::sub(s.ad_value(202), s.ad_value(138)), 1.0);
        }

        if (!s.b[368]) {
            s.store_scalar(211, 0.0);
            s.store_scalar(179, 0.0);
        }

        s.b[369] = (p.p51 < 100.0);
        s.store_scalar(369, if s.b[369] { 1.0 } else { 0.0 });

        s.b[370] = (s.v[33] > 0.0);
        s.store_scalar(370, if s.b[370] { 1.0 } else { 0.0 });

        if (s.b[369] && s.b[370]) {
            s.store_scalar(113, (p.p49 / 4.0));
            s.store_sub_from_scalar(114, p.p51, 34);
            s.store_mul_sub_from_scalar_ad_rhs(115, 34, 1.0, A::exp_scaled_input(A::ln(s.ad_value(35)), (-1.0 / (p.p49))));
            s.store_mul(116, 35, 33);
            s.store_mul_exp_ad_rhs(117, 33, A::mul_offset_lhs(s.ad_value(113), (-p.p49), A::ln(A::div_from_scalar(p.p51, s.ad_value(34)))));
            s.store_mul_sub_lhs(119, 115, 203, 5);
        }

        s.b[371] = (s.v[119] < 80.0);
        s.store_scalar(371, if s.b[371] { 1.0 } else { 0.0 });

        if ((s.b[369] && s.b[370]) && s.b[371]) {
            s.store_exp(120, 119);
            s.store_div_scaled_value_offset_denominator(121, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);
            s.store_add_scaled_product_right_ad(122, 115, 1.0, 4, A::ln(A::offset(s.ad_value(120), 1.0)), (-1.0));
        }

        if ((s.b[369] && s.b[370]) && (!s.b[371])) {
            s.store_scalar(121, 1.0);
            s.copy_ad(122, 203);
        }

        if (s.b[369] && s.b[370]) {
            s.store_add_scaled_inputs(118, 114, 0.1, 4, 4.0);
            s.store_div_scaled_inputs2_indices(123, 114, 1.0, 122, 1.0, 118, 1.0);
        }

        s.b[372] = (s.v[123] < 80.0);
        s.store_scalar(372, if s.b[372] { 1.0 } else { 0.0 });

        if ((s.b[369] && s.b[370]) && s.b[372]) {
            s.store_exp(120, 123);
            s.store_div_scaled_value_offset_denominator(124, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);
            s.store_sub_ad_lhs(125, A::mul(s.ad_value(118), A::sub(A::ln(A::offset(s.ad_value(120), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(114), -1.0, s.ad_value(115), -1.0, s.ad_value(118), 1.0)))), 114);
        }

        if ((s.b[369] && s.b[370]) && (!s.b[372])) {
            s.store_scalar(124, 1.0);
            s.copy_ad(125, 122);
        }

        if (s.b[369] && s.b[370]) {
            s.store_sub(126, 203, 122);
            s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(34))));
            s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(34))));
            s.store_scalar(132, (1.0 - p.p49));
            s.store_sub_from_scalar(133, 1.0, 113);
            s.store_mul_product3_mixed_iiai(134, 124, 33, A::exp_scaled_input(s.ad_value(131), (-p.p49)), 121, 1.0);
            s.store_mul_ad_product_rhs(135, 117, A::exp(A::mul_scaled_rhs(s.ad_value(130), s.ad_value(113), -1.0)), A::sub_from_scalar(1.0, s.ad_value(124)));
        }

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[369] && s.b[370]) {
            s.store_mul_sub_from_scalar_rhs(136, 116, 1.0, 121);
            s.store_add_scaled_inputs3_indices(210, 134, 1.0, 135, 1.0, 136, 1.0);
            s.store_div_ad_lhs(127, A::mul_sub_from_scalar_rhs(s.ad_value(33), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132)))), 132);
            s.store_div_ad_lhs(128, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133)))), 133);
            s.store_div_ad_lhs(129, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133)))), 133);
            s.store_add_scaled_products_left_left_ad(178, A::add_scaled_inputs3(s.ad_value(127), 1.0, s.ad_value(128), 1.0, s.ad_value(129), -1.0), 34, 1.0, 116, 126, 1.0);
        }

        if (s.b[369] && (!s.b[370])) {
            s.store_scalar(210, 0.0);
            s.store_scalar(178, 0.0);
        }

        s.b[373] = (s.v[33] > 0.0);
        s.store_scalar(373, if s.b[373] { 1.0 } else { 0.0 });

        if ((!s.b[369]) && s.b[373]) {
            s.store_mul_sub_from_scalar_ad_rhs(137, 34, 1.0, A::exp_scaled_input(A::ln(s.ad_value(35)), (-1.0 / (p.p49))));
            s.store_mul_sub_lhs(141, 137, 203, 5);
            s.store_sqrt_square_offset(142, 141, 1.921812);
            s.store_scaled_add(143, 141, 142, 0.5);
            s.store_add_scaled_product_indices(138, 137, 1.0, 4, 143, (-1.0));
            s.store_div(144, 143, 142);
            s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(34))));
            s.store_mul_ad_lhs(145, A::exp_scaled_input(s.ad_value(139), (-p.p49)), 144);
            s.store_mul_add_ad_rhs(210, 33, s.ad_value(145), A::mul_sub_from_scalar_rhs(s.ad_value(35), 1.0, s.ad_value(144)));
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(140, 34, 1.0, A::exp_scaled_input(s.ad_value(139), (1.0 - p.p49)), 1.0 / ((1.0 - p.p49)));
            s.store_mul_add_scaled_product_rhs(178, 33, s.ad_value(140), 1.0, s.ad_value(35), A::sub(s.ad_value(203), s.ad_value(138)), 1.0);
        }

        if ((!s.b[369]) && (!s.b[373])) {
            s.store_scalar(210, 0.0);
            s.store_scalar(178, 0.0);
        }

        s.b[374] = (p.p10 > 0.0);
        s.store_scalar(374, if s.b[374] { 1.0 } else { 0.0 });

        if s.b[374] {
            s.store_scale(375, 4, p.p11);
            s.store_div_scaled_inputs2_indices(376, 27, 1.0, 202, (-1.0), 375, 1.0);
            s.store_add_scaled_product_right_ad(377, 27, 1.0, 375, A::add(s.ad_value(376), A::sqrt_square_offset(s.ad_value(376), 1.921812)), (-0.5));
            s.store_mul_sub_from_scalar_ad_rhs(378, 18, 1.0, A::exp_scaled_input(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(377), s.ad_value(27)))), p.p41));
        }

        s.b[379] = (((s.v[378]) as f64).abs() > 0.001);
        s.store_scalar(379, if s.b[379] { 1.0 } else { 0.0 });

        if (s.b[374] && s.b[379]) {
            s.store_div_scaled_product_offset_rhs(346, s.ad_value(17), A::exp(s.ad_value(378)), (-1.0), 1.0, s.ad_value(378), 1.0);
        }

        if (s.b[374] && (!s.b[379])) {
            s.store_mul_scale_offset_rhs(346, 17, 378, 0.5, 1.0);
        }

        if (!s.b[374]) {
            s.copy_ad(346, 17);
        }

        s.store_add_scaled_ad_lhs(352, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(346), s.ad_value(179), 1.0), 178, p.p12);

        s.store_scale(353, 16, 0.05);

        s.store_offset_div(347, 352, 353, (-1.0));

        s.store_mul_offset_ad_rhs(352, 353, A::add_scaled_inputs(s.ad_value(347), 0.5, A::sqrt_square_offset(s.ad_value(347), 1.921812), 0.5), 1.0);

        s.store_scale(380, 34, (1.0 - ((((-((2.4) as f64).ln()) / p.p49)) as f64).exp()));

        s.store_mul_sub_lhs(381, 380, 203, 5);

        s.store_sqrt_square_offset(382, 381, 1.921812);

        s.store_scaled_add(383, 381, 382, 0.5);

        s.store_add_scaled_product_indices(384, 380, 1.0, 4, 383, (-1.0));

        s.store_div(385, 383, 382);

        s.store_add_scaled_product_mixed_aai(361, A::scale_offset(s.ad_value(385), (-2.4), 2.4), 1.0, A::exp_scaled_input(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(384), s.ad_value(34)))), (-p.p49)), 385, 1.0);

        s.store_add_scaled_inputs3_offset_mixed_iai(357, 59, 1.0, A::div_from_scalar(1.0, s.ad_value(361)), p.p67, 361, p.p68, (((-1.0) * p.p67) + ((-1.0) * p.p68)));

        s.b[386] = (p.p79 > 0.0);
        s.store_scalar(386, if s.b[386] { 1.0 } else { 0.0 });

        if s.b[386] {
            s.store_sub(363, 58, 203);
        }

        if (!s.b[386]) {
            s.store_sub(363, 204, 57);
        }

        s.b[394] = (p.p0 <= 300.0);
        s.store_scalar(394, if s.b[394] { 1.0 } else { 0.0 });

        if s.b[394] {
            s.store_mul_sub_lhs(387, 363, 4, 5);
            s.store_add_scaled_product_right_ad(388, 4, 1.0, 4, A::add(s.ad_value(387), A::sqrt_square_offset(s.ad_value(387), 1.921812)), 0.5);
        }

        if (!s.b[394]) {
            s.store_div(387, 363, 3);
            s.store_mul_add_scaled_inputs_rhs(388, 3, s.ad_value(387), 0.5, A::sqrt_square_offset(s.ad_value(387), p.p80), 0.5);
        }

        s.store_div(389, 388, 55);

        s.store_mul(390, 388, 54);

        s.store_exp_scaled_input_ad(391, A::ln_one_plus_exp(A::scale(A::ln(s.ad_value(389)), p.p77)), 1.0 / (p.p77));

        s.store_div(392, 390, 391);

        s.store_scaled_sub(393, 388, 55, 1.0 / (p.p76));

        s.store_mul_offset_ad_rhs(362, 392, A::add_scaled_inputs(s.ad_value(393), 0.5, A::sqrt_square_offset(s.ad_value(393), p.p81), 0.5), 1.0);

        s.copy_ad(348, 352);

        s.b[395] = ((s.v[357] > 0.0) || (p.p85 > 0.0));
        s.store_scalar(395, if s.b[395] { 1.0 } else { 0.0 });

        if s.b[395] {
            s.store_scale(396, 352, 0.5);
        }

        s.b[397] = (p.p0 <= 300.0);
        s.store_scalar(397, if s.b[397] { 1.0 } else { 0.0 });

        if (s.b[395] && s.b[397]) {
            s.store_add_ad_rhs(348, 396, A::sqrt(A::add_scaled_inputs(A::add_scaled_square_product(s.ad_value(396), 1.0, s.ad_value(357), s.ad_value(350), 1.0), 1.0, s.ad_value(351), p.p85)));
        }

        if (s.b[395] && (!s.b[397])) {
            s.store_add_ad_rhs(348, 396, A::sqrt(A::add_scaled_inputs3(A::square(s.ad_value(396)), 1.0, A::mul3(s.ad_value(19), s.ad_value(59), s.ad_value(350)), 1.0, s.ad_value(351), p.p85)));
        }

        s.store_div(217, 350, 348);

        s.store_div(218, 351, 348);

        s.copy_ad(219, 357);

        s.store_mul(355, 357, 217);

        s.b[398] = (p.p0 >= 310.0);
        s.store_scalar(398, if s.b[398] { 1.0 } else { 0.0 });

        if s.b[398] {
            s.store_mul(359, 19, 59);
            s.store_mul(358, 359, 217);
        }

        if (!s.b[398]) {
            s.store_mul(358, 19, 355);
            s.store_mul(359, 19, 219);
        }

        s.store_scalar(354, 0.0);

        s.b[399] = ((s.v[217] >= (1e-6 * s.v[362])) || (p.p0 >= 320.0));
        s.store_scalar(399, if s.b[399] { 1.0 } else { 0.0 });

        if s.b[399] {
            s.store_div(96, 217, 362);
            s.store_mul_ad_rhs(98, 61, A::exp_scaled_input(A::ln(s.ad_value(96)), p.p70));
            s.store_scaled_mul(97, 98, 217, 1.0 / ((1.0 + p.p70)));
        }

        s.b[400] = (p.p83 < (0.05 * (p.p75 / p.p74)));
        s.store_scalar(400, if s.b[400] { 1.0 } else { 0.0 });

        if (s.b[399] && s.b[400]) {
            s.store_scalar(111, 0.0);
            s.store_scalar(112, 0.0);
        }

        if (s.b[399] && (!s.b[400])) {
            s.store_scaled_sub(107, 217, 362, 1.0 / (p.p83));
        }

        s.b[401] = (s.v[107] < (-10000000000.0));
        s.store_scalar(401, if s.b[401] { 1.0 } else { 0.0 });

        if ((s.b[399] && (!s.b[400])) && s.b[401]) {
            s.store_scalar(107, (-10000000000.0));
        }

        if (s.b[399] && (!s.b[400])) {
            s.store_sqrt_square_offset(95, 107, p.p84);
            s.store_scaled_exp_ad(111, A::div_from_scalar((-2.0), A::add(s.ad_value(107), s.ad_value(95))), p.p82);
            s.store_div_scaled_inputs_mixed_ia(112, 111, 2.0, A::mul_scaled_lhs(s.ad_value(95), p.p83, A::add(s.ad_value(107), s.ad_value(95))), 1.0);
        }

        if s.b[399] {
            s.store_mul_scaled_offset_ad_rhs(99, 60, (1.0 - p.p73), A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (-1.0));
            s.store_add_product3_rhs_mixed_aii(100, 99, A::mul3_scaled_output(s.ad_value(60), s.ad_value(217), A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (1.0 - p.p73)), 5, 112, 1.0);
            s.store_sub_from_scalar_ad(108, 1.0, A::div_from_scalar(1.0, s.ad_value(96)));
            s.store_scaled_add_sqrt_square_offset_rhs(109, 108, 108, p.p72, 1.0 / ((1.0 + (((1.0 + p.p72)) as f64).sqrt())));
            s.store_exp_ad(110, A::mul_offset_lhs(s.ad_value(111), (-p.p82), s.ad_value(5)));
            s.store_mul_product3_indices(101, 110, 60, 109, 109, 1.0);
            s.store_mul_add_ad_rhs(102, 101, A::offset(A::div_from_scalar(2.0, A::mul(s.ad_value(96), A::sqrt_square_offset(s.ad_value(108), p.p72))), 1.0), A::mul3(s.ad_value(5), s.ad_value(217), s.ad_value(112)));
        }

        s.b[402] = ((((p.p115 < 0.01) && (p.p116 < 0.01)) && ((s.v[109] * p.p115) < 0.005)) && ((s.v[109] * p.p116) < 0.005));
        s.store_scalar(402, if s.b[402] { 1.0 } else { 0.0 });

        if (s.b[399] && s.b[402]) {
            s.store_scaled_mul(105, 101, 217, p.p73);
            s.store_scale(106, 102, p.p73);
        }

        if (s.b[399] && (!s.b[402])) {
            s.store_sub_from_scalar(146, 1.0, 109);
            s.store_div_ad(147, A::mul_sub_from_scalar_rhs(A::offset(s.ad_value(146), (-1.0)), 1.0, s.ad_value(108)), A::mul(A::sqrt_square_offset(s.ad_value(108), p.p72), s.ad_value(217)));
        }

        s.b[403] = (((s.v[232]) as f64).abs() > 0.001);
        s.store_scalar(403, if s.b[403] { 1.0 } else { 0.0 });

        if ((s.b[399] && (!s.b[402])) && s.b[403]) {
            s.store_exp_ad(151, A::mul_offset_lhs(s.ad_value(146), (-1.0), s.ad_value(231)));
        }

        s.b[404] = (s.v[229] < 0.01);
        s.store_scalar(404, if s.b[404] { 1.0 } else { 0.0 });

        if (((s.b[399] && (!s.b[402])) && s.b[403]) && s.b[404]) {
            s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(151)), A::mul(s.ad_value(151), s.ad_value(230)));
            s.store_offset_mul(148, 230, 149, 1.0);
            s.store_div_scaled_inputs2_by_product(154, A::mul3(s.ad_value(230), s.ad_value(149), A::offset(A::mul_scaled_lhs(s.ad_value(230), 0.25, s.ad_value(149)), 0.5)), 2.0, A::ln(s.ad_value(148)), (-(0.5 * 2.0)), s.ad_value(230), s.ad_value(230), 1.0);
            s.store_div_scaled_product_by_product(150, s.ad_value(231), s.ad_value(147), -1.0, s.ad_value(151), s.ad_value(230), 1.0);
            s.store_div_scaled_product3_mixed_aiii(155, A::offset(s.ad_value(148), 1.0), 149, 150, 1.0, 148, 1.0);
        }

        if (((s.b[399] && (!s.b[402])) && s.b[403]) && (!s.b[404])) {
            s.store_sub_from_scalar_scaled_input(152, p.p116, 151, p.p115);
            s.store_div_scaled_offset_numerator(149, s.ad_value(151), 1.0, (-1.0), s.ad_value(152), 1.0);
            s.store_offset_scaled(160, 149, p.p116, 1.0);
            s.store_ln(161, 160);
            s.store_mul(162, 227, 226);
            s.store_add_scaled_products_mixed_aiai(157, A::mul_sub_from_scalar_rhs(s.ad_value(161), 0.5, s.ad_value(162)), 226, 1.0, A::add_scaled_product(s.ad_value(162), 1.0, s.ad_value(227), s.ad_value(149), 1.0), 149, 1.0);
            s.store_add_scaled_inputs_product_first_ad(159, A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), 1.0, 162, 1.0, 149, 227, 2.0);
            s.store_offset_scaled(160, 149, p.p115, 1.0);
            s.store_ln(161, 160);
            s.store_mul(162, 228, 225);
            s.store_add_scaled_products_mixed_aiai(156, A::mul_sub_from_scalar_rhs(s.ad_value(161), 0.5, s.ad_value(162)), 225, 1.0, A::add_scaled_product(s.ad_value(162), 1.0, s.ad_value(228), s.ad_value(149), 1.0), 149, 1.0);
            s.store_add_scaled_inputs_product_first_ad(158, A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), 1.0, 162, 1.0, 149, 228, 2.0);
            s.store_div_scaled_inputs2_indices(154, 157, 1.0, 156, (-1.0), 232, 1.0);
            s.store_mul_product3_mixed_iaii(150, 147, A::div_scaled_inputs(s.ad_value(232), (-2.0), A::square(s.ad_value(152)), 1.0), 151, 231, 1.0);
            s.store_div_scaled_product_left_ad(155, A::sub(s.ad_value(159), s.ad_value(158)), 150, 1.0, 232, 1.0);
        }

        if ((s.b[399] && (!s.b[402])) && (!s.b[403])) {
            s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(146)), A::scale_offset(s.ad_value(146), p.p115, 1.0));
            s.store_offset_scaled(153, 149, p.p115, 1.0);
            s.store_div_scaled_product_offset_rhs(154, A::square(s.ad_value(149)), A::mul_scaled_lhs(s.ad_value(227), 2.0, s.ad_value(149)), 1.0, 1.0, s.ad_value(153), 1.0);
            s.store_div_scaled_product_denominator_ad(150, 147, 153, -1.0, A::scale_offset(s.ad_value(146), p.p115, 1.0), 1.0);
            s.store_mul_ad_product_lhs_mixed_ia(155, 149, A::offset(A::div_from_scalar(1.0, A::square(s.ad_value(153))), 1.0), 150);
        }

        if (s.b[399] && (!s.b[402])) {
            s.store_scaled_mul(166, 60, 110, p.p73);
            s.store_mul(167, 166, 154);
            s.store_mul(105, 167, 217);
            s.store_add_scaled_inputs3_mixed_iaa(106, 167, 1.0, A::mul3(s.ad_value(105), s.ad_value(112), s.ad_value(5)), 1.0, A::mul3(s.ad_value(166), s.ad_value(217), s.ad_value(155)), 1.0);
        }

        if s.b[399] {
            s.store_scaled_mul(103, 101, 217, (1.0 - p.p73));
            s.store_scale(104, 102, (1.0 - p.p73));
            s.store_add_scaled_product_indices(354, 103, 1.0, 99, 217, 1.0);
        }

        s.b[405] = (p.p0 >= 310.0);
        s.store_scalar(405, if s.b[405] { 1.0 } else { 0.0 });

        if (s.b[399] && s.b[405]) {
            s.store_add_scaled_inputs4_indices(355, 355, 1.0, 354, 1.0, 97, 1.0, 105, 1.0);
            s.store_add_ad_lhs(219, A::add_scaled_inputs4(s.ad_value(219), 1.0, s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(98), 1.0), 106);
            s.store_add_scaled_inputs_products_indices(358, 358, 1.0, 354, p.p5, 20, 97, 1.0, 21, 105, 1.0);
            s.store_add_scaled_value_products(359, A::add_scaled_inputs3(s.ad_value(359), 1.0, s.ad_value(100), p.p5, s.ad_value(104), p.p5), 1.0, s.ad_value(20), s.ad_value(98), 1.0, s.ad_value(21), s.ad_value(106), 1.0);
        }

        if (s.b[399] && (!s.b[405])) {
            s.store_add_scaled_value_products3_indices(358, 354, 1.0, 19, 355, 1.0, 20, 97, 1.0, 21, 105, 1.0);
            s.store_add_scaled_inputs4_indices(355, 355, 1.0, 354, 1.0, 97, 1.0, 105, 1.0);
            s.store_add_scaled_product_value_ad(359, A::add_scaled_inputs_products(s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(19), s.ad_value(219), 1.0, s.ad_value(20), s.ad_value(98), 1.0), 1.0, 21, 106, 1.0);
            s.store_add_ad_lhs(219, A::add_scaled_inputs4(s.ad_value(219), 1.0, s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(98), 1.0), 106);
        }

        s.store_scale(356, 218, p.p85);

        s.store_scalar(224, 0.0);

        s.b[406] = (((p.p0 >= 310.0) && (s.v[358] > (1e-5 * s.v[348]))) || ((p.p0 <= 300.0) && (s.v[355] > (1e-5 * s.v[348]))));
        s.store_scalar(406, if s.b[406] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[406] {
            s.store_sqrt_ad(355, A::mul3(s.ad_value(357), s.ad_value(217), s.ad_value(358)));
            s.store_add_scaled_inputs3_indices(348, 352, 1.0, 355, 1.0, 356, p.p7);
            s.copy_ad(349, 348);
        }

        let mut assign6470_loop_guard: usize = 0;
        while {
            let assign6470_cond_e6823: f64 = (s.v[349]).abs();
            let assign6470_cond_e6823_d_n0: f64 = if s.v[349] >= 0.0 { s.dn[349][0] } else { (-s.dn[349][0]) };
            let assign6470_cond_e6823_d_n1: f64 = if s.v[349] >= 0.0 { s.dn[349][1] } else { (-s.dn[349][1]) };
            let assign6470_cond_e6823_d_n2: f64 = if s.v[349] >= 0.0 { s.dn[349][2] } else { (-s.dn[349][2]) };
            let assign6470_cond_e6823_d_n3: f64 = if s.v[349] >= 0.0 { s.dn[349][3] } else { (-s.dn[349][3]) };
            let assign6470_cond_e6823_d_n4: f64 = if s.v[349] >= 0.0 { s.dn[349][4] } else { (-s.dn[349][4]) };
            let assign6470_cond_e6823_d_n5: f64 = if s.v[349] >= 0.0 { s.dn[349][5] } else { (-s.dn[349][5]) };
            let assign6470_cond_e6823_d_n6: f64 = if s.v[349] >= 0.0 { s.dn[349][6] } else { (-s.dn[349][6]) };
            let assign6470_cond_e6823_d_n7: f64 = if s.v[349] >= 0.0 { s.dn[349][7] } else { (-s.dn[349][7]) };
            let assign6470_cond_e6823_d_n8: f64 = if s.v[349] >= 0.0 { s.dn[349][8] } else { (-s.dn[349][8]) };
            let assign6470_cond_e6823_d_n9: f64 = if s.v[349] >= 0.0 { s.dn[349][9] } else { (-s.dn[349][9]) };
            let assign6470_cond_e6823_d_n10: f64 = if s.v[349] >= 0.0 { s.dn[349][10] } else { (-s.dn[349][10]) };
            let assign6470_cond_e6823_d_n11: f64 = if s.v[349] >= 0.0 { s.dn[349][11] } else { (-s.dn[349][11]) };
            let assign6470_cond_e6823_d_n12: f64 = if s.v[349] >= 0.0 { s.dn[349][12] } else { (-s.dn[349][12]) };
            let assign6470_cond_e6823_d_n13: f64 = if s.v[349] >= 0.0 { s.dn[349][13] } else { (-s.dn[349][13]) };
            let assign6470_cond_e6823_d_n14: f64 = if s.v[349] >= 0.0 { s.dn[349][14] } else { (-s.dn[349][14]) };
            let assign6470_cond_e6823_d_b0: f64 = if s.v[349] >= 0.0 { s.db[349][0] } else { (-s.db[349][0]) };
            let assign6470_cond_e6823_d_b1: f64 = if s.v[349] >= 0.0 { s.db[349][1] } else { (-s.db[349][1]) };
            let assign6470_cond_e6823_d_b2: f64 = if s.v[349] >= 0.0 { s.db[349][2] } else { (-s.db[349][2]) };
            let assign6470_cond_e6823_d_b3: f64 = if s.v[349] >= 0.0 { s.db[349][3] } else { (-s.db[349][3]) };
            let assign6470_cond_e6823_d_b4: f64 = if s.v[349] >= 0.0 { s.db[349][4] } else { (-s.db[349][4]) };
            let assign6470_cond_e6823_d_b5: f64 = if s.v[349] >= 0.0 { s.db[349][5] } else { (-s.db[349][5]) };
            let assign6470_cond_e6826: f64 = 1e-5;
            let assign6470_cond_e6828: f64 = (s.v[348]).abs();
            let assign6470_cond_e6828_d_n0: f64 = if s.v[348] >= 0.0 { s.dn[348][0] } else { (-s.dn[348][0]) };
            let assign6470_cond_e6828_d_n1: f64 = if s.v[348] >= 0.0 { s.dn[348][1] } else { (-s.dn[348][1]) };
            let assign6470_cond_e6828_d_n2: f64 = if s.v[348] >= 0.0 { s.dn[348][2] } else { (-s.dn[348][2]) };
            let assign6470_cond_e6828_d_n3: f64 = if s.v[348] >= 0.0 { s.dn[348][3] } else { (-s.dn[348][3]) };
            let assign6470_cond_e6828_d_n4: f64 = if s.v[348] >= 0.0 { s.dn[348][4] } else { (-s.dn[348][4]) };
            let assign6470_cond_e6828_d_n5: f64 = if s.v[348] >= 0.0 { s.dn[348][5] } else { (-s.dn[348][5]) };
            let assign6470_cond_e6828_d_n6: f64 = if s.v[348] >= 0.0 { s.dn[348][6] } else { (-s.dn[348][6]) };
            let assign6470_cond_e6828_d_n7: f64 = if s.v[348] >= 0.0 { s.dn[348][7] } else { (-s.dn[348][7]) };
            let assign6470_cond_e6828_d_n8: f64 = if s.v[348] >= 0.0 { s.dn[348][8] } else { (-s.dn[348][8]) };
            let assign6470_cond_e6828_d_n9: f64 = if s.v[348] >= 0.0 { s.dn[348][9] } else { (-s.dn[348][9]) };
            let assign6470_cond_e6828_d_n10: f64 = if s.v[348] >= 0.0 { s.dn[348][10] } else { (-s.dn[348][10]) };
            let assign6470_cond_e6828_d_n11: f64 = if s.v[348] >= 0.0 { s.dn[348][11] } else { (-s.dn[348][11]) };
            let assign6470_cond_e6828_d_n12: f64 = if s.v[348] >= 0.0 { s.dn[348][12] } else { (-s.dn[348][12]) };
            let assign6470_cond_e6828_d_n13: f64 = if s.v[348] >= 0.0 { s.dn[348][13] } else { (-s.dn[348][13]) };
            let assign6470_cond_e6828_d_n14: f64 = if s.v[348] >= 0.0 { s.dn[348][14] } else { (-s.dn[348][14]) };
            let assign6470_cond_e6828_d_b0: f64 = if s.v[348] >= 0.0 { s.db[348][0] } else { (-s.db[348][0]) };
            let assign6470_cond_e6828_d_b1: f64 = if s.v[348] >= 0.0 { s.db[348][1] } else { (-s.db[348][1]) };
            let assign6470_cond_e6828_d_b2: f64 = if s.v[348] >= 0.0 { s.db[348][2] } else { (-s.db[348][2]) };
            let assign6470_cond_e6828_d_b3: f64 = if s.v[348] >= 0.0 { s.db[348][3] } else { (-s.db[348][3]) };
            let assign6470_cond_e6828_d_b4: f64 = if s.v[348] >= 0.0 { s.db[348][4] } else { (-s.db[348][4]) };
            let assign6470_cond_e6828_d_b5: f64 = if s.v[348] >= 0.0 { s.db[348][5] } else { (-s.db[348][5]) };
            let assign6470_cond_e6829: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828);
            let assign6470_cond_e6829_d_n0: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n0);
            let assign6470_cond_e6829_d_n1: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n1);
            let assign6470_cond_e6829_d_n2: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n2);
            let assign6470_cond_e6829_d_n3: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n3);
            let assign6470_cond_e6829_d_n4: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n4);
            let assign6470_cond_e6829_d_n5: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n5);
            let assign6470_cond_e6829_d_n6: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n6);
            let assign6470_cond_e6829_d_n7: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n7);
            let assign6470_cond_e6829_d_n8: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n8);
            let assign6470_cond_e6829_d_n9: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n9);
            let assign6470_cond_e6829_d_n10: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n10);
            let assign6470_cond_e6829_d_n11: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n11);
            let assign6470_cond_e6829_d_n12: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n12);
            let assign6470_cond_e6829_d_n13: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n13);
            let assign6470_cond_e6829_d_n14: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n14);
            let assign6470_cond_e6829_d_b0: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_b0);
            let assign6470_cond_e6829_d_b1: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_b1);
            let assign6470_cond_e6829_d_b2: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_b2);
            let assign6470_cond_e6829_d_b3: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_b3);
            let assign6470_cond_e6829_d_b4: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_b4);
            let assign6470_cond_e6829_d_b5: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_b5);
            let assign6470_cond_e6835: f64 = if (s.b[406] && ((assign6470_cond_e6823 >= assign6470_cond_e6829) && (s.v[224] <= 100.0))) { 1.0 } else { 0.0 };
            assign6470_cond_e6835 != 0.0
        } {
            assign6470_loop_guard += 1;
            assert!(assign6470_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[406] {
                s.store_div(217, 350, 348);
                s.store_div(218, 351, 348);
                s.copy_ad(219, 357);
                s.store_mul(355, 357, 217);
            }
            s.b[408] = (p.p0 >= 310.0);
            s.store_scalar(408, if s.b[408] { 1.0 } else { 0.0 });
            if (s.b[406] && s.b[408]) {
                s.store_mul(359, 19, 59);
                s.store_mul(358, 359, 217);
            }
            if (s.b[406] && (!s.b[408])) {
                s.store_mul(358, 19, 355);
                s.store_mul(359, 19, 219);
            }
            if s.b[406] {
                s.store_scalar(354, 0.0);
            }
            s.b[409] = ((s.v[217] >= (1e-6 * s.v[362])) || (p.p0 >= 320.0));
            s.store_scalar(409, if s.b[409] { 1.0 } else { 0.0 });
            if (s.b[406] && s.b[409]) {
                s.store_div(96, 217, 362);
                s.store_mul_ad_rhs(98, 61, A::exp_scaled_input(A::ln(s.ad_value(96)), p.p70));
                s.store_scaled_mul(97, 98, 217, 1.0 / ((1.0 + p.p70)));
            }
            s.b[410] = (p.p83 < (0.05 * (p.p75 / p.p74)));
            s.store_scalar(410, if s.b[410] { 1.0 } else { 0.0 });
            if ((s.b[406] && s.b[409]) && s.b[410]) {
                s.store_scalar(111, 0.0);
                s.store_scalar(112, 0.0);
            }
            if ((s.b[406] && s.b[409]) && (!s.b[410])) {
                s.store_scaled_sub(107, 217, 362, 1.0 / (p.p83));
            }
            s.b[411] = (s.v[107] < (-10000000000.0));
            s.store_scalar(411, if s.b[411] { 1.0 } else { 0.0 });
            if (((s.b[406] && s.b[409]) && (!s.b[410])) && s.b[411]) {
                s.store_scalar(107, (-10000000000.0));
            }
            if ((s.b[406] && s.b[409]) && (!s.b[410])) {
                s.store_sqrt_square_offset(95, 107, p.p84);
                s.store_scaled_exp_ad(111, A::div_from_scalar((-2.0), A::add(s.ad_value(107), s.ad_value(95))), p.p82);
                s.store_div_scaled_inputs_mixed_ia(112, 111, 2.0, A::mul_scaled_lhs(s.ad_value(95), p.p83, A::add(s.ad_value(107), s.ad_value(95))), 1.0);
            }
            if (s.b[406] && s.b[409]) {
                s.store_mul_scaled_offset_ad_rhs(99, 60, (1.0 - p.p73), A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (-1.0));
                s.store_add_product3_rhs_mixed_aii(100, 99, A::mul3_scaled_output(s.ad_value(60), s.ad_value(217), A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (1.0 - p.p73)), 5, 112, 1.0);
                s.store_sub_from_scalar_ad(108, 1.0, A::div_from_scalar(1.0, s.ad_value(96)));
                s.store_scaled_add_sqrt_square_offset_rhs(109, 108, 108, p.p72, 1.0 / ((1.0 + (((1.0 + p.p72)) as f64).sqrt())));
                s.store_exp_ad(110, A::mul_offset_lhs(s.ad_value(111), (-p.p82), s.ad_value(5)));
                s.store_mul_product3_indices(101, 110, 60, 109, 109, 1.0);
                s.store_mul_add_ad_rhs(102, 101, A::offset(A::div_from_scalar(2.0, A::mul(s.ad_value(96), A::sqrt_square_offset(s.ad_value(108), p.p72))), 1.0), A::mul3(s.ad_value(5), s.ad_value(217), s.ad_value(112)));
            }
            s.b[412] = ((((p.p115 < 0.01) && (p.p116 < 0.01)) && ((s.v[109] * p.p115) < 0.005)) && ((s.v[109] * p.p116) < 0.005));
            s.store_scalar(412, if s.b[412] { 1.0 } else { 0.0 });
            if ((s.b[406] && s.b[409]) && s.b[412]) {
                s.store_scaled_mul(105, 101, 217, p.p73);
                s.store_scale(106, 102, p.p73);
            }
            if ((s.b[406] && s.b[409]) && (!s.b[412])) {
                s.store_sub_from_scalar(146, 1.0, 109);
                s.store_div_ad(147, A::mul_sub_from_scalar_rhs(A::offset(s.ad_value(146), (-1.0)), 1.0, s.ad_value(108)), A::mul(A::sqrt_square_offset(s.ad_value(108), p.p72), s.ad_value(217)));
            }
            s.b[413] = (((s.v[232]) as f64).abs() > 0.001);
            s.store_scalar(413, if s.b[413] { 1.0 } else { 0.0 });
            if (((s.b[406] && s.b[409]) && (!s.b[412])) && s.b[413]) {
                s.store_exp_ad(151, A::mul_offset_lhs(s.ad_value(146), (-1.0), s.ad_value(231)));
            }
            s.b[414] = (s.v[229] < 0.01);
            s.store_scalar(414, if s.b[414] { 1.0 } else { 0.0 });
            if ((((s.b[406] && s.b[409]) && (!s.b[412])) && s.b[413]) && s.b[414]) {
                s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(151)), A::mul(s.ad_value(151), s.ad_value(230)));
                s.store_offset_mul(148, 230, 149, 1.0);
                s.store_div_scaled_inputs2_by_product(154, A::mul3(s.ad_value(230), s.ad_value(149), A::offset(A::mul_scaled_lhs(s.ad_value(230), 0.25, s.ad_value(149)), 0.5)), 2.0, A::ln(s.ad_value(148)), (-(0.5 * 2.0)), s.ad_value(230), s.ad_value(230), 1.0);
                s.store_div_scaled_product_by_product(150, s.ad_value(231), s.ad_value(147), -1.0, s.ad_value(151), s.ad_value(230), 1.0);
                s.store_div_scaled_product3_mixed_aiii(155, A::offset(s.ad_value(148), 1.0), 149, 150, 1.0, 148, 1.0);
            }
            if ((((s.b[406] && s.b[409]) && (!s.b[412])) && s.b[413]) && (!s.b[414])) {
                s.store_sub_from_scalar_scaled_input(152, p.p116, 151, p.p115);
                s.store_div_scaled_offset_numerator(149, s.ad_value(151), 1.0, (-1.0), s.ad_value(152), 1.0);
                s.store_offset_scaled(160, 149, p.p116, 1.0);
                s.store_ln(161, 160);
                s.store_mul(162, 227, 226);
                s.store_add_scaled_products_mixed_aiai(157, A::mul_sub_from_scalar_rhs(s.ad_value(161), 0.5, s.ad_value(162)), 226, 1.0, A::add_scaled_product(s.ad_value(162), 1.0, s.ad_value(227), s.ad_value(149), 1.0), 149, 1.0);
                s.store_add_scaled_inputs_product_first_ad(159, A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), 1.0, 162, 1.0, 149, 227, 2.0);
                s.store_offset_scaled(160, 149, p.p115, 1.0);
                s.store_ln(161, 160);
                s.store_mul(162, 228, 225);
                s.store_add_scaled_products_mixed_aiai(156, A::mul_sub_from_scalar_rhs(s.ad_value(161), 0.5, s.ad_value(162)), 225, 1.0, A::add_scaled_product(s.ad_value(162), 1.0, s.ad_value(228), s.ad_value(149), 1.0), 149, 1.0);
                s.store_add_scaled_inputs_product_first_ad(158, A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), 1.0, 162, 1.0, 149, 228, 2.0);
                s.store_div_scaled_inputs2_indices(154, 157, 1.0, 156, (-1.0), 232, 1.0);
                s.store_mul_product3_mixed_iaii(150, 147, A::div_scaled_inputs(s.ad_value(232), (-2.0), A::square(s.ad_value(152)), 1.0), 151, 231, 1.0);
                s.store_div_scaled_product_left_ad(155, A::sub(s.ad_value(159), s.ad_value(158)), 150, 1.0, 232, 1.0);
            }
            if (((s.b[406] && s.b[409]) && (!s.b[412])) && (!s.b[413])) {
                s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(146)), A::scale_offset(s.ad_value(146), p.p115, 1.0));
                s.store_offset_scaled(153, 149, p.p115, 1.0);
                s.store_div_scaled_product_offset_rhs(154, A::square(s.ad_value(149)), A::mul_scaled_lhs(s.ad_value(227), 2.0, s.ad_value(149)), 1.0, 1.0, s.ad_value(153), 1.0);
                s.store_div_scaled_product_denominator_ad(150, 147, 153, -1.0, A::scale_offset(s.ad_value(146), p.p115, 1.0), 1.0);
                s.store_mul_ad_product_lhs_mixed_ia(155, 149, A::offset(A::div_from_scalar(1.0, A::square(s.ad_value(153))), 1.0), 150);
            }
            if ((s.b[406] && s.b[409]) && (!s.b[412])) {
                s.store_scaled_mul(166, 60, 110, p.p73);
                s.store_mul(167, 166, 154);
                s.store_mul(105, 167, 217);
                s.store_add_scaled_inputs3_mixed_iaa(106, 167, 1.0, A::mul3(s.ad_value(105), s.ad_value(112), s.ad_value(5)), 1.0, A::mul3(s.ad_value(166), s.ad_value(217), s.ad_value(155)), 1.0);
            }
            if (s.b[406] && s.b[409]) {
                s.store_scaled_mul(103, 101, 217, (1.0 - p.p73));
                s.store_scale(104, 102, (1.0 - p.p73));
                s.store_add_scaled_product_indices(354, 103, 1.0, 99, 217, 1.0);
            }
            s.b[415] = (p.p0 >= 310.0);
            s.store_scalar(415, if s.b[415] { 1.0 } else { 0.0 });
            if ((s.b[406] && s.b[409]) && s.b[415]) {
                s.store_add_scaled_inputs4_indices(355, 355, 1.0, 354, 1.0, 97, 1.0, 105, 1.0);
                s.store_add_ad_lhs(219, A::add_scaled_inputs4(s.ad_value(219), 1.0, s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(98), 1.0), 106);
                s.store_add_scaled_inputs_products_indices(358, 358, 1.0, 354, p.p5, 20, 97, 1.0, 21, 105, 1.0);
                s.store_add_scaled_value_products(359, A::add_scaled_inputs3(s.ad_value(359), 1.0, s.ad_value(100), p.p5, s.ad_value(104), p.p5), 1.0, s.ad_value(20), s.ad_value(98), 1.0, s.ad_value(21), s.ad_value(106), 1.0);
            }
            if ((s.b[406] && s.b[409]) && (!s.b[415])) {
                s.store_add_scaled_value_products3_indices(358, 354, 1.0, 19, 355, 1.0, 20, 97, 1.0, 21, 105, 1.0);
                s.store_add_scaled_inputs4_indices(355, 355, 1.0, 354, 1.0, 97, 1.0, 105, 1.0);
                s.store_add_scaled_product_value_ad(359, A::add_scaled_inputs_products(s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(19), s.ad_value(219), 1.0, s.ad_value(20), s.ad_value(98), 1.0), 1.0, 21, 106, 1.0);
                s.store_add_ad_lhs(219, A::add_scaled_inputs4(s.ad_value(219), 1.0, s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(98), 1.0), 106);
            }
            if s.b[406] {
                s.store_scale(360, 218, (p.p7 * p.p85));
                s.store_div_scaled_inputs(349, A::add_scaled_inputs4(s.ad_value(348), 1.0, s.ad_value(352), -1.0, s.ad_value(358), -1.0, s.ad_value(360), -1.0), -1.0, A::offset(A::div_scaled_add_product(s.ad_value(360), 1.0, s.ad_value(359), s.ad_value(217), 1.0, s.ad_value(348), 1.0), 1.0), 1.0);
                s.store_abs_scaled_input(407, 348, 0.3);
            }
            s.b[416] = (((s.v[349]) as f64).abs() > s.v[407]);
            s.store_scalar(416, if s.b[416] { 1.0 } else { 0.0 });
            s.b[417] = (s.v[349] >= 0.0);
            s.store_scalar(417, if s.b[417] { 1.0 } else { 0.0 });
            if ((s.b[406] && s.b[416]) && s.b[417]) {
                s.copy_ad(349, 407);
            }
            if ((s.b[406] && s.b[416]) && (!s.b[417])) {
                s.store_neg(349, 407);
            }
            if s.b[406] {
                s.store_add(348, 348, 349);
                s.store_scalar(224, (s.v[224] + 1.0));
            }
        }

        if s.b[406] {
            s.store_div(217, 350, 348);
            s.store_div(218, 351, 348);
            s.copy_ad(219, 357);
            s.store_mul(355, 357, 217);
        }

        s.b[418] = (p.p0 >= 310.0);
        s.store_scalar(418, if s.b[418] { 1.0 } else { 0.0 });

        if (s.b[406] && s.b[418]) {
            s.store_mul(359, 19, 59);
            s.store_mul(358, 359, 217);
        }

        if (s.b[406] && (!s.b[418])) {
            s.store_mul(358, 19, 355);
            s.store_mul(359, 19, 219);
        }

        if s.b[406] {
            s.store_scalar(354, 0.0);
        }

        s.b[419] = ((s.v[217] >= (1e-6 * s.v[362])) || (p.p0 >= 320.0));
        s.store_scalar(419, if s.b[419] { 1.0 } else { 0.0 });

        if (s.b[406] && s.b[419]) {
            s.store_div(96, 217, 362);
            s.store_mul_ad_rhs(98, 61, A::exp_scaled_input(A::ln(s.ad_value(96)), p.p70));
            s.store_scaled_mul(97, 98, 217, 1.0 / ((1.0 + p.p70)));
        }

        s.b[420] = (p.p83 < (0.05 * (p.p75 / p.p74)));
        s.store_scalar(420, if s.b[420] { 1.0 } else { 0.0 });

        if ((s.b[406] && s.b[419]) && s.b[420]) {
            s.store_scalar(111, 0.0);
            s.store_scalar(112, 0.0);
        }

        if ((s.b[406] && s.b[419]) && (!s.b[420])) {
            s.store_scaled_sub(107, 217, 362, 1.0 / (p.p83));
        }

        s.b[421] = (s.v[107] < (-10000000000.0));
        s.store_scalar(421, if s.b[421] { 1.0 } else { 0.0 });

        if (((s.b[406] && s.b[419]) && (!s.b[420])) && s.b[421]) {
            s.store_scalar(107, (-10000000000.0));
        }

        if ((s.b[406] && s.b[419]) && (!s.b[420])) {
            s.store_sqrt_square_offset(95, 107, p.p84);
            s.store_scaled_exp_ad(111, A::div_from_scalar((-2.0), A::add(s.ad_value(107), s.ad_value(95))), p.p82);
            s.store_div_scaled_inputs_mixed_ia(112, 111, 2.0, A::mul_scaled_lhs(s.ad_value(95), p.p83, A::add(s.ad_value(107), s.ad_value(95))), 1.0);
        }

        if (s.b[406] && s.b[419]) {
            s.store_mul_scaled_offset_ad_rhs(99, 60, (1.0 - p.p73), A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (-1.0));
            s.store_add_product3_rhs_mixed_aii(100, 99, A::mul3_scaled_output(s.ad_value(60), s.ad_value(217), A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (1.0 - p.p73)), 5, 112, 1.0);
            s.store_sub_from_scalar_ad(108, 1.0, A::div_from_scalar(1.0, s.ad_value(96)));
            s.store_scaled_add_sqrt_square_offset_rhs(109, 108, 108, p.p72, 1.0 / ((1.0 + (((1.0 + p.p72)) as f64).sqrt())));
            s.store_exp_ad(110, A::mul_offset_lhs(s.ad_value(111), (-p.p82), s.ad_value(5)));
            s.store_mul_product3_indices(101, 110, 60, 109, 109, 1.0);
            s.store_mul_add_ad_rhs(102, 101, A::offset(A::div_from_scalar(2.0, A::mul(s.ad_value(96), A::sqrt_square_offset(s.ad_value(108), p.p72))), 1.0), A::mul3(s.ad_value(5), s.ad_value(217), s.ad_value(112)));
        }

        s.b[422] = ((((p.p115 < 0.01) && (p.p116 < 0.01)) && ((s.v[109] * p.p115) < 0.005)) && ((s.v[109] * p.p116) < 0.005));
        s.store_scalar(422, if s.b[422] { 1.0 } else { 0.0 });

        if ((s.b[406] && s.b[419]) && s.b[422]) {
            s.store_scaled_mul(105, 101, 217, p.p73);
            s.store_scale(106, 102, p.p73);
        }

        if ((s.b[406] && s.b[419]) && (!s.b[422])) {
            s.store_sub_from_scalar(146, 1.0, 109);
            s.store_div_ad(147, A::mul_sub_from_scalar_rhs(A::offset(s.ad_value(146), (-1.0)), 1.0, s.ad_value(108)), A::mul(A::sqrt_square_offset(s.ad_value(108), p.p72), s.ad_value(217)));
        }

        s.b[423] = (((s.v[232]) as f64).abs() > 0.001);
        s.store_scalar(423, if s.b[423] { 1.0 } else { 0.0 });

        if (((s.b[406] && s.b[419]) && (!s.b[422])) && s.b[423]) {
            s.store_exp_ad(151, A::mul_offset_lhs(s.ad_value(146), (-1.0), s.ad_value(231)));
        }

        s.b[424] = (s.v[229] < 0.01);
        s.store_scalar(424, if s.b[424] { 1.0 } else { 0.0 });

        if ((((s.b[406] && s.b[419]) && (!s.b[422])) && s.b[423]) && s.b[424]) {
            s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(151)), A::mul(s.ad_value(151), s.ad_value(230)));
            s.store_offset_mul(148, 230, 149, 1.0);
            s.store_div_scaled_inputs2_by_product(154, A::mul3(s.ad_value(230), s.ad_value(149), A::offset(A::mul_scaled_lhs(s.ad_value(230), 0.25, s.ad_value(149)), 0.5)), 2.0, A::ln(s.ad_value(148)), (-(0.5 * 2.0)), s.ad_value(230), s.ad_value(230), 1.0);
            s.store_div_scaled_product_by_product(150, s.ad_value(231), s.ad_value(147), -1.0, s.ad_value(151), s.ad_value(230), 1.0);
            s.store_div_scaled_product3_mixed_aiii(155, A::offset(s.ad_value(148), 1.0), 149, 150, 1.0, 148, 1.0);
        }

        if ((((s.b[406] && s.b[419]) && (!s.b[422])) && s.b[423]) && (!s.b[424])) {
            s.store_sub_from_scalar_scaled_input(152, p.p116, 151, p.p115);
        }

    }

    pub(super) fn stamp_reactive_block_5(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((((s.b[406] && s.b[419]) && (!s.b[422])) && s.b[423]) && (!s.b[424])) {
            s.store_div_scaled_offset_numerator(149, s.ad_value(151), 1.0, (-1.0), s.ad_value(152), 1.0);
            s.store_offset_scaled(160, 149, p.p116, 1.0);
            s.store_ln(161, 160);
            s.store_mul(162, 227, 226);
            s.store_add_scaled_products_mixed_aiai(157, A::mul_sub_from_scalar_rhs(s.ad_value(161), 0.5, s.ad_value(162)), 226, 1.0, A::add_scaled_product(s.ad_value(162), 1.0, s.ad_value(227), s.ad_value(149), 1.0), 149, 1.0);
            s.store_add_scaled_inputs_product_first_ad(159, A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), 1.0, 162, 1.0, 149, 227, 2.0);
            s.store_offset_scaled(160, 149, p.p115, 1.0);
            s.store_ln(161, 160);
            s.store_mul(162, 228, 225);
            s.store_add_scaled_products_mixed_aiai(156, A::mul_sub_from_scalar_rhs(s.ad_value(161), 0.5, s.ad_value(162)), 225, 1.0, A::add_scaled_product(s.ad_value(162), 1.0, s.ad_value(228), s.ad_value(149), 1.0), 149, 1.0);
            s.store_add_scaled_inputs_product_first_ad(158, A::div(A::sub_from_scalar(0.5, s.ad_value(162)), s.ad_value(160)), 1.0, 162, 1.0, 149, 228, 2.0);
            s.store_div_scaled_inputs2_indices(154, 157, 1.0, 156, (-1.0), 232, 1.0);
            s.store_mul_product3_mixed_iaii(150, 147, A::div_scaled_inputs(s.ad_value(232), (-2.0), A::square(s.ad_value(152)), 1.0), 151, 231, 1.0);
            s.store_div_scaled_product_left_ad(155, A::sub(s.ad_value(159), s.ad_value(158)), 150, 1.0, 232, 1.0);
        }

        if (((s.b[406] && s.b[419]) && (!s.b[422])) && (!s.b[423])) {
            s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(146)), A::scale_offset(s.ad_value(146), p.p115, 1.0));
            s.store_offset_scaled(153, 149, p.p115, 1.0);
            s.store_div_scaled_product_offset_rhs(154, A::square(s.ad_value(149)), A::mul_scaled_lhs(s.ad_value(227), 2.0, s.ad_value(149)), 1.0, 1.0, s.ad_value(153), 1.0);
            s.store_div_scaled_product_denominator_ad(150, 147, 153, -1.0, A::scale_offset(s.ad_value(146), p.p115, 1.0), 1.0);
            s.store_mul_ad_product_lhs_mixed_ia(155, 149, A::offset(A::div_from_scalar(1.0, A::square(s.ad_value(153))), 1.0), 150);
        }

        if ((s.b[406] && s.b[419]) && (!s.b[422])) {
            s.store_scaled_mul(166, 60, 110, p.p73);
            s.store_mul(167, 166, 154);
            s.store_mul(105, 167, 217);
            s.store_add_scaled_inputs3_mixed_iaa(106, 167, 1.0, A::mul3(s.ad_value(105), s.ad_value(112), s.ad_value(5)), 1.0, A::mul3(s.ad_value(166), s.ad_value(217), s.ad_value(155)), 1.0);
        }

        if (s.b[406] && s.b[419]) {
            s.store_scaled_mul(103, 101, 217, (1.0 - p.p73));
            s.store_scale(104, 102, (1.0 - p.p73));
            s.store_add_scaled_product_indices(354, 103, 1.0, 99, 217, 1.0);
        }

        s.b[425] = (p.p0 >= 310.0);
        s.store_scalar(425, if s.b[425] { 1.0 } else { 0.0 });

        if ((s.b[406] && s.b[419]) && s.b[425]) {
            s.store_add_scaled_inputs4_indices(355, 355, 1.0, 354, 1.0, 97, 1.0, 105, 1.0);
            s.store_add_ad_lhs(219, A::add_scaled_inputs4(s.ad_value(219), 1.0, s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(98), 1.0), 106);
            s.store_add_scaled_inputs_products_indices(358, 358, 1.0, 354, p.p5, 20, 97, 1.0, 21, 105, 1.0);
            s.store_add_scaled_value_products(359, A::add_scaled_inputs3(s.ad_value(359), 1.0, s.ad_value(100), p.p5, s.ad_value(104), p.p5), 1.0, s.ad_value(20), s.ad_value(98), 1.0, s.ad_value(21), s.ad_value(106), 1.0);
        }

        if ((s.b[406] && s.b[419]) && (!s.b[425])) {
            s.store_add_scaled_value_products3_indices(358, 354, 1.0, 19, 355, 1.0, 20, 97, 1.0, 21, 105, 1.0);
            s.store_add_scaled_inputs4_indices(355, 355, 1.0, 354, 1.0, 97, 1.0, 105, 1.0);
            s.store_add_scaled_product_value_ad(359, A::add_scaled_inputs_products(s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(19), s.ad_value(219), 1.0, s.ad_value(20), s.ad_value(98), 1.0), 1.0, 21, 106, 1.0);
            s.store_add_ad_lhs(219, A::add_scaled_inputs4(s.ad_value(219), 1.0, s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(98), 1.0), 106);
        }

        if s.b[406] {
            s.store_scale(356, 218, p.p85);
        }

        s.store_sub(184, 217, 218);

        s.copy_ad(181, 355);

        s.copy_ad(182, 356);

        s.store_mul3_lhs(220, 357, 217, 5);

        s.store_scaled_mul(221, 218, 5, p.p85);

        s.store_add_scaled_inputs4_indices(222, 211, p.p93, 210, p.p93, 220, p.p93, 221, p.p93);

        s.store_mul_voltage_ad(183, s.ad_value(222), ctx, nodes, Some(7), Some(8));

        s.b[426] = (p.p23 > 0.0);
        s.store_scalar(426, if s.b[426] { 1.0 } else { 0.0 });

        if s.b[426] {
            s.store_div_scaled_inputs_indices(93, 203, 1.0, 4, p.p24);
        }

        s.b[427] = (s.v[93] > 80.0);
        s.store_scalar(427, if s.b[427] { 1.0 } else { 0.0 });

        if (s.b[426] && s.b[427]) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
            s.store_scalar(93, 80.0);
        }

        if (s.b[426] && (!s.b[427])) {
            s.store_scalar(94, 1.0);
        }

        s.b[428] = ((p.p37 > 0.0) && (s.v[203] < 0.0));
        s.store_scalar(428, if s.b[428] { 1.0 } else { 0.0 });

        s.b[429] = ((s.v[33] > 0.0) && (s.v[34] > 0.0));
        s.store_scalar(429, if s.b[429] { 1.0 } else { 0.0 });

        if (s.b[428] && s.b[429]) {
            s.store_exp_scaled_input_ad(168, A::ln(A::div(s.ad_value(210), s.ad_value(33))), ((1.0 / p.p49) - 1.0));
            s.store_div_scaled_product_by_product(166, s.ad_value(67), s.ad_value(203), -1.0, s.ad_value(34), s.ad_value(168), 1.0);
        }

        s.b[456] = (p.p18 > 0.0);
        s.store_scalar(456, if s.b[456] { 1.0 } else { 0.0 });

        if s.b[456] {
            s.store_div_scaled_inputs_indices(93, 205, 1.0, 4, p.p19);
        }

        s.b[457] = (s.v[93] > 80.0);
        s.store_scalar(457, if s.b[457] { 1.0 } else { 0.0 });

        if (s.b[456] && s.b[457]) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
            s.store_scalar(93, 80.0);
        }

        if (s.b[456] && (!s.b[457])) {
            s.store_scalar(94, 1.0);
        }

        s.b[458] = (p.p20 > 0.0);
        s.store_scalar(458, if s.b[458] { 1.0 } else { 0.0 });

        if s.b[458] {
            s.store_div_scaled_inputs_indices(93, 205, 1.0, 4, p.p21);
        }

        s.b[459] = (s.v[93] > 80.0);
        s.store_scalar(459, if s.b[459] { 1.0 } else { 0.0 });

        if (s.b[458] && s.b[459]) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
            s.store_scalar(93, 80.0);
        }

        if (s.b[458] && (!s.b[459])) {
            s.store_scalar(94, 1.0);
        }

        s.b[460] = (s.v[29] > 0.0);
        s.store_scalar(460, if s.b[460] { 1.0 } else { 0.0 });

        if s.b[460] {
            s.store_mul_sub_from_scalar_ad_rhs(137, 30, 1.0, A::exp_scaled_input(A::ln(s.ad_value(31)), (-1.0 / (p.p45))));
            s.store_mul_sub_lhs(141, 137, 205, 5);
            s.store_sqrt_square_offset(142, 141, 1.921812);
            s.store_scaled_add(143, 141, 142, 0.5);
            s.store_add_scaled_product_indices(138, 137, 1.0, 4, 143, (-1.0));
            s.store_div(144, 143, 142);
            s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(30))));
            s.store_mul_ad_lhs(145, A::exp_scaled_input(s.ad_value(139), (-p.p45)), 144);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(140, 30, 1.0, A::exp_scaled_input(s.ad_value(139), (1.0 - p.p45)), 1.0 / ((1.0 - p.p45)));
            s.store_mul_add_scaled_product_rhs(180, 29, s.ad_value(140), 1.0, s.ad_value(31), A::sub(s.ad_value(205), s.ad_value(138)), 1.0);
        }

        if (!s.b[460]) {
            s.store_scalar(180, 0.0);
        }

        s.b[466] = (p.p56 < 100.0);
        s.store_scalar(466, if s.b[466] { 1.0 } else { 0.0 });

        s.b[467] = (s.v[38] > 0.0);
        s.store_scalar(467, if s.b[467] { 1.0 } else { 0.0 });

        if (s.b[466] && s.b[467]) {
            s.store_scalar(113, (p.p54 / 4.0));
            s.store_sub_from_scalar(114, p.p56, 39);
            s.store_mul_sub_from_scalar_ad_rhs(115, 39, 1.0, A::exp_scaled_input(A::ln(s.ad_value(40)), (-1.0 / (p.p54))));
            s.store_mul(116, 40, 38);
            s.store_mul_exp_ad_rhs(117, 38, A::mul_offset_lhs(s.ad_value(113), (-p.p54), A::ln(A::div_from_scalar(p.p56, s.ad_value(39)))));
            s.store_mul_sub_lhs(119, 115, 206, 5);
        }

        s.b[468] = (s.v[119] < 80.0);
        s.store_scalar(468, if s.b[468] { 1.0 } else { 0.0 });

        if ((s.b[466] && s.b[467]) && s.b[468]) {
            s.store_exp(120, 119);
            s.store_div_scaled_value_offset_denominator(121, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);
            s.store_add_scaled_product_right_ad(122, 115, 1.0, 4, A::ln(A::offset(s.ad_value(120), 1.0)), (-1.0));
        }

        if ((s.b[466] && s.b[467]) && (!s.b[468])) {
            s.store_scalar(121, 1.0);
            s.copy_ad(122, 206);
        }

        if (s.b[466] && s.b[467]) {
            s.store_add_scaled_inputs(118, 114, 0.1, 4, 4.0);
            s.store_div_scaled_inputs2_indices(123, 114, 1.0, 122, 1.0, 118, 1.0);
        }

        s.b[469] = (s.v[123] < 80.0);
        s.store_scalar(469, if s.b[469] { 1.0 } else { 0.0 });

        if ((s.b[466] && s.b[467]) && s.b[469]) {
            s.store_exp(120, 123);
            s.store_div_scaled_value_offset_denominator(124, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);
            s.store_sub_ad_lhs(125, A::mul(s.ad_value(118), A::sub(A::ln(A::offset(s.ad_value(120), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(114), -1.0, s.ad_value(115), -1.0, s.ad_value(118), 1.0)))), 114);
        }

        if ((s.b[466] && s.b[467]) && (!s.b[469])) {
            s.store_scalar(124, 1.0);
            s.copy_ad(125, 122);
        }

        if (s.b[466] && s.b[467]) {
            s.store_sub(126, 206, 122);
            s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(39))));
            s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(39))));
            s.store_scalar(132, (1.0 - p.p54));
            s.store_sub_from_scalar(133, 1.0, 113);
            s.store_mul_product3_mixed_iiai(134, 124, 38, A::exp_scaled_input(s.ad_value(131), (-p.p54)), 121, 1.0);
            s.store_mul_ad_product_rhs(135, 117, A::exp(A::mul_scaled_rhs(s.ad_value(130), s.ad_value(113), -1.0)), A::sub_from_scalar(1.0, s.ad_value(124)));
            s.store_mul_sub_from_scalar_rhs(136, 116, 1.0, 121);
            s.store_div_ad_lhs(127, A::mul_sub_from_scalar_rhs(s.ad_value(38), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132)))), 132);
            s.store_div_ad_lhs(128, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133)))), 133);
            s.store_div_ad_lhs(129, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133)))), 133);
            s.store_add_scaled_products_left_left_ad(42, A::add_scaled_inputs3(s.ad_value(127), 1.0, s.ad_value(128), 1.0, s.ad_value(129), -1.0), 39, 1.0, 116, 126, 1.0);
        }

        if (s.b[466] && (!s.b[467])) {
            s.store_scalar(42, 0.0);
        }

        s.b[470] = (s.v[38] > 0.0);
        s.store_scalar(470, if s.b[470] { 1.0 } else { 0.0 });

        if ((!s.b[466]) && s.b[470]) {
            s.store_mul_sub_from_scalar_ad_rhs(137, 39, 1.0, A::exp_scaled_input(A::ln(s.ad_value(40)), (-1.0 / (p.p54))));
            s.store_mul_sub_lhs(141, 137, 206, 5);
            s.store_sqrt_square_offset(142, 141, 1.921812);
            s.store_scaled_add(143, 141, 142, 0.5);
            s.store_add_scaled_product_indices(138, 137, 1.0, 4, 143, (-1.0));
            s.store_div(144, 143, 142);
            s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(39))));
            s.store_mul_ad_lhs(145, A::exp_scaled_input(s.ad_value(139), (-p.p54)), 144);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(140, 39, 1.0, A::exp_scaled_input(s.ad_value(139), (1.0 - p.p54)), 1.0 / ((1.0 - p.p54)));
            s.store_mul_add_scaled_product_rhs(42, 38, s.ad_value(140), 1.0, s.ad_value(40), A::sub(s.ad_value(206), s.ad_value(138)), 1.0);
        }

        if ((!s.b[466]) && (!s.b[470])) {
            s.store_scalar(42, 0.0);
        }

        s.b[471] = (p.p25 > 0.0);
        s.store_scalar(471, if s.b[471] { 1.0 } else { 0.0 });

        if s.b[471] {
            s.store_div_scaled_inputs_indices(93, 206, 1.0, 4, p.p26);
        }

        s.b[472] = (s.v[93] > 80.0);
        s.store_scalar(472, if s.b[472] { 1.0 } else { 0.0 });

        if (s.b[471] && s.b[472]) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
            s.store_scalar(93, 80.0);
        }

        if (s.b[471] && (!s.b[472])) {
            s.store_scalar(94, 1.0);
        }

        s.b[473] = (p.p56 < 100.0);
        s.store_scalar(473, if s.b[473] { 1.0 } else { 0.0 });

        s.b[474] = (s.v[37] > 0.0);
        s.store_scalar(474, if s.b[474] { 1.0 } else { 0.0 });

        if (s.b[473] && s.b[474]) {
            s.store_scalar(113, (p.p54 / 4.0));
            s.store_sub_from_scalar(114, p.p56, 39);
            s.store_mul_sub_from_scalar_ad_rhs(115, 39, 1.0, A::exp_scaled_input(A::ln(s.ad_value(40)), (-1.0 / (p.p54))));
            s.store_mul(116, 40, 37);
        }

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[473] && s.b[474]) {
            s.store_mul_exp_ad_rhs(117, 37, A::mul_offset_lhs(s.ad_value(113), (-p.p54), A::ln(A::div_from_scalar(p.p56, s.ad_value(39)))));
            s.store_mul_sub_lhs(119, 115, 207, 5);
        }

        s.b[475] = (s.v[119] < 80.0);
        s.store_scalar(475, if s.b[475] { 1.0 } else { 0.0 });

        if ((s.b[473] && s.b[474]) && s.b[475]) {
            s.store_exp(120, 119);
            s.store_div_scaled_value_offset_denominator(121, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);
            s.store_add_scaled_product_right_ad(122, 115, 1.0, 4, A::ln(A::offset(s.ad_value(120), 1.0)), (-1.0));
        }

        if ((s.b[473] && s.b[474]) && (!s.b[475])) {
            s.store_scalar(121, 1.0);
            s.copy_ad(122, 207);
        }

        if (s.b[473] && s.b[474]) {
            s.store_add_scaled_inputs(118, 114, 0.1, 4, 4.0);
            s.store_div_scaled_inputs2_indices(123, 114, 1.0, 122, 1.0, 118, 1.0);
        }

        s.b[476] = (s.v[123] < 80.0);
        s.store_scalar(476, if s.b[476] { 1.0 } else { 0.0 });

        if ((s.b[473] && s.b[474]) && s.b[476]) {
            s.store_exp(120, 123);
            s.store_div_scaled_value_offset_denominator(124, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);
            s.store_sub_ad_lhs(125, A::mul(s.ad_value(118), A::sub(A::ln(A::offset(s.ad_value(120), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(114), -1.0, s.ad_value(115), -1.0, s.ad_value(118), 1.0)))), 114);
        }

        if ((s.b[473] && s.b[474]) && (!s.b[476])) {
            s.store_scalar(124, 1.0);
            s.copy_ad(125, 122);
        }

        if (s.b[473] && s.b[474]) {
            s.store_sub(126, 207, 122);
            s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(39))));
            s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(39))));
            s.store_scalar(132, (1.0 - p.p54));
            s.store_sub_from_scalar(133, 1.0, 113);
            s.store_mul_product3_mixed_iiai(134, 124, 37, A::exp_scaled_input(s.ad_value(131), (-p.p54)), 121, 1.0);
            s.store_mul_ad_product_rhs(135, 117, A::exp(A::mul_scaled_rhs(s.ad_value(130), s.ad_value(113), -1.0)), A::sub_from_scalar(1.0, s.ad_value(124)));
            s.store_mul_sub_from_scalar_rhs(136, 116, 1.0, 121);
            s.store_div_ad_lhs(127, A::mul_sub_from_scalar_rhs(s.ad_value(37), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132)))), 132);
            s.store_div_ad_lhs(128, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133)))), 133);
            s.store_div_ad_lhs(129, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133)))), 133);
            s.store_add_scaled_products_left_left_ad(41, A::add_scaled_inputs3(s.ad_value(127), 1.0, s.ad_value(128), 1.0, s.ad_value(129), -1.0), 39, 1.0, 116, 126, 1.0);
        }

        if (s.b[473] && (!s.b[474])) {
            s.store_scalar(41, 0.0);
        }

        s.b[477] = (s.v[37] > 0.0);
        s.store_scalar(477, if s.b[477] { 1.0 } else { 0.0 });

        if ((!s.b[473]) && s.b[477]) {
            s.store_mul_sub_from_scalar_ad_rhs(137, 39, 1.0, A::exp_scaled_input(A::ln(s.ad_value(40)), (-1.0 / (p.p54))));
            s.store_mul_sub_lhs(141, 137, 207, 5);
            s.store_sqrt_square_offset(142, 141, 1.921812);
            s.store_scaled_add(143, 141, 142, 0.5);
            s.store_add_scaled_product_indices(138, 137, 1.0, 4, 143, (-1.0));
            s.store_div(144, 143, 142);
            s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(39))));
            s.store_mul_ad_lhs(145, A::exp_scaled_input(s.ad_value(139), (-p.p54)), 144);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(140, 39, 1.0, A::exp_scaled_input(s.ad_value(139), (1.0 - p.p54)), 1.0 / ((1.0 - p.p54)));
            s.store_mul_add_scaled_product_rhs(41, 37, s.ad_value(140), 1.0, s.ad_value(40), A::sub(s.ad_value(207), s.ad_value(138)), 1.0);
        }

        if ((!s.b[473]) && (!s.b[477])) {
            s.store_scalar(41, 0.0);
        }

        s.b[478] = (p.p61 < 100.0);
        s.store_scalar(478, if s.b[478] { 1.0 } else { 0.0 });

        s.b[479] = (s.v[46] > 0.0);
        s.store_scalar(479, if s.b[479] { 1.0 } else { 0.0 });

        if (s.b[478] && s.b[479]) {
            s.store_scalar(113, (p.p59 / 4.0));
            s.store_sub_from_scalar(114, p.p61, 47);
            s.store_mul_sub_from_scalar_ad_rhs(115, 47, 1.0, A::exp_scaled_input(A::ln(s.ad_value(48)), (-1.0 / (p.p59))));
            s.store_mul(116, 48, 46);
            s.store_mul_exp_ad_rhs(117, 46, A::mul_offset_lhs(s.ad_value(113), (-p.p59), A::ln(A::div_from_scalar(p.p61, s.ad_value(47)))));
            s.store_mul_sub_lhs(119, 115, 208, 5);
        }

        s.b[480] = (s.v[119] < 80.0);
        s.store_scalar(480, if s.b[480] { 1.0 } else { 0.0 });

        if ((s.b[478] && s.b[479]) && s.b[480]) {
            s.store_exp(120, 119);
            s.store_div_scaled_value_offset_denominator(121, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);
            s.store_add_scaled_product_right_ad(122, 115, 1.0, 4, A::ln(A::offset(s.ad_value(120), 1.0)), (-1.0));
        }

        if ((s.b[478] && s.b[479]) && (!s.b[480])) {
            s.store_scalar(121, 1.0);
            s.copy_ad(122, 208);
        }

        if (s.b[478] && s.b[479]) {
            s.store_add_scaled_inputs(118, 114, 0.1, 4, 4.0);
            s.store_div_scaled_inputs2_indices(123, 114, 1.0, 122, 1.0, 118, 1.0);
        }

        s.b[481] = (s.v[123] < 80.0);
        s.store_scalar(481, if s.b[481] { 1.0 } else { 0.0 });

        if ((s.b[478] && s.b[479]) && s.b[481]) {
            s.store_exp(120, 123);
            s.store_div_scaled_value_offset_denominator(124, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);
            s.store_sub_ad_lhs(125, A::mul(s.ad_value(118), A::sub(A::ln(A::offset(s.ad_value(120), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(114), -1.0, s.ad_value(115), -1.0, s.ad_value(118), 1.0)))), 114);
        }

        if ((s.b[478] && s.b[479]) && (!s.b[481])) {
            s.store_scalar(124, 1.0);
            s.copy_ad(125, 122);
        }

        if (s.b[478] && s.b[479]) {
            s.store_sub(126, 208, 122);
            s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(47))));
            s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(47))));
            s.store_scalar(132, (1.0 - p.p59));
            s.store_sub_from_scalar(133, 1.0, 113);
            s.store_mul_product3_mixed_iiai(134, 124, 46, A::exp_scaled_input(s.ad_value(131), (-p.p59)), 121, 1.0);
            s.store_mul_ad_product_rhs(135, 117, A::exp(A::mul_scaled_rhs(s.ad_value(130), s.ad_value(113), -1.0)), A::sub_from_scalar(1.0, s.ad_value(124)));
            s.store_mul_sub_from_scalar_rhs(136, 116, 1.0, 121);
            s.store_div_ad_lhs(127, A::mul_sub_from_scalar_rhs(s.ad_value(46), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132)))), 132);
            s.store_div_ad_lhs(128, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133)))), 133);
            s.store_div_ad_lhs(129, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133)))), 133);
            s.store_add_scaled_products_left_left_ad(196, A::add_scaled_inputs3(s.ad_value(127), 1.0, s.ad_value(128), 1.0, s.ad_value(129), -1.0), 47, 1.0, 116, 126, 1.0);
        }

        if (s.b[478] && (!s.b[479])) {
            s.store_scalar(196, 0.0);
        }

        s.b[482] = (s.v[46] > 0.0);
        s.store_scalar(482, if s.b[482] { 1.0 } else { 0.0 });

        if ((!s.b[478]) && s.b[482]) {
            s.store_mul_sub_from_scalar_ad_rhs(137, 47, 1.0, A::exp_scaled_input(A::ln(s.ad_value(48)), (-1.0 / (p.p59))));
            s.store_mul_sub_lhs(141, 137, 208, 5);
            s.store_sqrt_square_offset(142, 141, 1.921812);
            s.store_scaled_add(143, 141, 142, 0.5);
            s.store_add_scaled_product_indices(138, 137, 1.0, 4, 143, (-1.0));
            s.store_div(144, 143, 142);
            s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(47))));
            s.store_mul_ad_lhs(145, A::exp_scaled_input(s.ad_value(139), (-p.p59)), 144);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(140, 47, 1.0, A::exp_scaled_input(s.ad_value(139), (1.0 - p.p59)), 1.0 / ((1.0 - p.p59)));
            s.store_mul_add_scaled_product_rhs(196, 46, s.ad_value(140), 1.0, s.ad_value(48), A::sub(s.ad_value(208), s.ad_value(138)), 1.0);
        }

        if ((!s.b[478]) && (!s.b[482])) {
            s.store_scalar(196, 0.0);
        }

        s.b[483] = (p.p63 > 0.0);
        s.store_scalar(483, if s.b[483] { 1.0 } else { 0.0 });

        s.b[484] = (p.p65 < 100.0);
        s.store_scalar(484, if s.b[484] { 1.0 } else { 0.0 });

        s.b[485] = (s.v[49] > 0.0);
        s.store_scalar(485, if s.b[485] { 1.0 } else { 0.0 });

        if ((s.b[483] && s.b[484]) && s.b[485]) {
            s.store_scalar(113, (p.p64 / 4.0));
            s.store_sub_from_scalar(114, p.p65, 50);
            s.store_mul_sub_from_scalar_ad_rhs(115, 50, 1.0, A::exp_scaled_input(A::ln(s.ad_value(51)), (-1.0 / (p.p64))));
            s.store_mul(116, 51, 49);
            s.store_mul_exp_ad_rhs(117, 49, A::mul_offset_lhs(s.ad_value(113), (-p.p64), A::ln(A::div_from_scalar(p.p65, s.ad_value(50)))));
            s.store_mul_sub_lhs(119, 115, 209, 5);
        }

        s.b[486] = (s.v[119] < 80.0);
        s.store_scalar(486, if s.b[486] { 1.0 } else { 0.0 });

        if (((s.b[483] && s.b[484]) && s.b[485]) && s.b[486]) {
            s.store_exp(120, 119);
            s.store_div_scaled_value_offset_denominator(121, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);
            s.store_add_scaled_product_right_ad(122, 115, 1.0, 4, A::ln(A::offset(s.ad_value(120), 1.0)), (-1.0));
        }

        if (((s.b[483] && s.b[484]) && s.b[485]) && (!s.b[486])) {
            s.store_scalar(121, 1.0);
            s.copy_ad(122, 209);
        }

        if ((s.b[483] && s.b[484]) && s.b[485]) {
            s.store_add_scaled_inputs(118, 114, 0.1, 4, 4.0);
            s.store_div_scaled_inputs2_indices(123, 114, 1.0, 122, 1.0, 118, 1.0);
        }

        s.b[487] = (s.v[123] < 80.0);
        s.store_scalar(487, if s.b[487] { 1.0 } else { 0.0 });

        if (((s.b[483] && s.b[484]) && s.b[485]) && s.b[487]) {
            s.store_exp(120, 123);
            s.store_div_scaled_value_offset_denominator(124, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);
            s.store_sub_ad_lhs(125, A::mul(s.ad_value(118), A::sub(A::ln(A::offset(s.ad_value(120), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(114), -1.0, s.ad_value(115), -1.0, s.ad_value(118), 1.0)))), 114);
        }

        if (((s.b[483] && s.b[484]) && s.b[485]) && (!s.b[487])) {
            s.store_scalar(124, 1.0);
            s.copy_ad(125, 122);
        }

        if ((s.b[483] && s.b[484]) && s.b[485]) {
            s.store_sub(126, 209, 122);
            s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(50))));
            s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(50))));
            s.store_scalar(132, (1.0 - p.p64));
            s.store_sub_from_scalar(133, 1.0, 113);
            s.store_mul_product3_mixed_iiai(134, 124, 49, A::exp_scaled_input(s.ad_value(131), (-p.p64)), 121, 1.0);
            s.store_mul_ad_product_rhs(135, 117, A::exp(A::mul_scaled_rhs(s.ad_value(130), s.ad_value(113), -1.0)), A::sub_from_scalar(1.0, s.ad_value(124)));
            s.store_mul_sub_from_scalar_rhs(136, 116, 1.0, 121);
            s.store_div_ad_lhs(127, A::mul_sub_from_scalar_rhs(s.ad_value(49), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132)))), 132);
            s.store_div_ad_lhs(128, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133)))), 133);
            s.store_div_ad_lhs(129, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133)))), 133);
            s.store_add_scaled_products_left_left_ad(197, A::add_scaled_inputs3(s.ad_value(127), 1.0, s.ad_value(128), 1.0, s.ad_value(129), -1.0), 50, 1.0, 116, 126, 1.0);
        }

        if ((s.b[483] && s.b[484]) && (!s.b[485])) {
            s.store_scalar(197, 0.0);
        }

        s.b[488] = (s.v[49] > 0.0);
        s.store_scalar(488, if s.b[488] { 1.0 } else { 0.0 });

        if ((s.b[483] && (!s.b[484])) && s.b[488]) {
            s.store_mul_sub_from_scalar_ad_rhs(137, 50, 1.0, A::exp_scaled_input(A::ln(s.ad_value(51)), (-1.0 / (p.p64))));
            s.store_mul_sub_lhs(141, 137, 209, 5);
            s.store_sqrt_square_offset(142, 141, 1.921812);
            s.store_scaled_add(143, 141, 142, 0.5);
            s.store_add_scaled_product_indices(138, 137, 1.0, 4, 143, (-1.0));
            s.store_div(144, 143, 142);
        }

    }

    pub(super) fn stamp_reactive_block_7(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[483] && (!s.b[484])) && s.b[488]) {
            s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(50))));
            s.store_mul_ad_lhs(145, A::exp_scaled_input(s.ad_value(139), (-p.p64)), 144);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(140, 50, 1.0, A::exp_scaled_input(s.ad_value(139), (1.0 - p.p64)), 1.0 / ((1.0 - p.p64)));
            s.store_mul_add_scaled_product_rhs(197, 49, s.ad_value(140), 1.0, s.ad_value(51), A::sub(s.ad_value(209), s.ad_value(138)), 1.0);
        }

        if ((s.b[483] && (!s.b[484])) && (!s.b[488])) {
            s.store_scalar(197, 0.0);
        }

        if (!s.b[483]) {
            s.store_scale(197, 209, p.p62);
        }

        s.b[489] = (p.p97 > 0.0);
        s.store_scalar(489, if s.b[489] { 1.0 } else { 0.0 });

        if s.b[489] {
            s.store_scale(490, 4, p.p98);
            s.store_limexp_div(491, 206, 490);
        }

        s.b[493] = (p.p101 > 0.0);
        s.store_scalar(493, if s.b[493] { 1.0 } else { 0.0 });

        if (s.b[489] && s.b[493]) {
            s.store_mul3_lhs(199, 52, 44, 491);
        }

        if (s.b[489] && (!s.b[493])) {
            s.store_scalar(199, 0.0);
        }

        if (!s.b[489]) {
            s.store_scalar(199, 0.0);
        }

        s.b[494] = (p.p99 > 0.0);
        s.store_scalar(494, if s.b[494] { 1.0 } else { 0.0 });

        if s.b[494] {
            s.store_div_scaled_inputs_indices(93, 208, 1.0, 4, p.p100);
        }

        s.b[495] = (s.v[93] > 80.0);
        s.store_scalar(495, if s.b[495] { 1.0 } else { 0.0 });

        if (s.b[494] && s.b[495]) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
            s.store_scalar(93, 80.0);
        }

        if (s.b[494] && (!s.b[495])) {
            s.store_scalar(94, 1.0);
        }

        s.copy_ad(242, 181);

        s.b[507] = (s.v[234] != 0.0);
        s.store_scalar(507, if s.b[507] { 1.0 } else { 0.0 });

        if s.b[507] {
            s.store_voltage(503, ctx, nodes, Some(12), None);
            s.copy_ad(242, 503);
        }

        s.b[508] = ((p.p89 >= p.p149) && (p.p89 > 0.0));
        s.store_scalar(508, if s.b[508] { 1.0 } else { 0.0 });

        s.b[509] = (p.p93 > 0.0);
        s.store_scalar(509, if s.b[509] { 1.0 } else { 0.0 });

        s.b[517] = ((p.p102 >= p.p149) && (p.p102 > 0.0));
        s.store_scalar(517, if s.b[517] { 1.0 } else { 0.0 });

        s.b[518] = (p.p103 > 0.0);
        s.store_scalar(518, if s.b[518] { 1.0 } else { 0.0 });

        s.b[519] = (((p.p141 >= 1.0) && (p.p142 >= p.p149)) && (p.p142 > 0.0));
        s.store_scalar(519, if s.b[519] { 1.0 } else { 0.0 });

        s.b[520] = (p.p145 > 0.0);
        s.store_scalar(520, if s.b[520] { 1.0 } else { 0.0 });

        s.b[533] = ((p.p109 == 1.0) && ((p.p88 > 0.0) && (p.p87 > 0.0)));
        s.store_scalar(533, if s.b[533] { 1.0 } else { 0.0 });

        s.b[539] = (s.v[185] > 0.0);
        s.store_scalar(539, if s.b[539] { 1.0 } else { 0.0 });

        if (s.b[533] && s.b[539]) {
            s.store_div(534, 184, 185);
        }

        if (s.b[533] && (!s.b[539])) {
            s.store_scalar(534, 1000000000.0);
        }

        if s.b[533] {
            s.store_scalar(535, 1.0);
            s.store_scale(536, 219, p.p88);
            s.store_scale(538, 534, ((2.0 * p.p87) - (p.p88 * p.p88)));
        }

        s.b[540] = (s.v[538] > 0.0);
        s.store_scalar(540, if s.b[540] { 1.0 } else { 0.0 });

        if (s.b[533] && s.b[540]) {
            s.store_mul_sqrt_rhs(537, 219, 538);
        }

        if (s.b[533] && (!s.b[540])) {
            s.store_scalar(537, 0.0);
        }

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_gmin: f64,
        var_guard233: f64,
        var_guard234: f64,
        var_guard235: f64,
        var_guard236: f64,
        var_guard237: f64,
        var_guard238: f64,
        var_iavl: f64,
        var_iavl_dn0: f64,
        var_iavl_dn1: f64,
        var_iavl_dn3: f64,
        var_iavl_dn4: f64,
        var_iavl_dn5: f64,
        var_iavl_dn6: f64,
        var_iavl_dn7: f64,
        var_iavl_dn8: f64,
        var_iavl_dn9: f64,
        var_ibcbtb: f64,
        var_ibcbtb_dn0: f64,
        var_ibcbtb_dn1: f64,
        var_ibcbtb_dn3: f64,
        var_ibcbtb_dn4: f64,
        var_ibcbtb_dn5: f64,
        var_ibcbtb_dn6: f64,
        var_ibcbtb_dn7: f64,
        var_ibcbtb_dn8: f64,
        var_ibcbtb_dn9: f64,
        var_ibci: f64,
        var_ibci_dn4: f64,
        var_ibci_dn5: f64,
        var_ibci_dn6: f64,
        var_ibci_dn7: f64,
        var_ibci_dn8: f64,
        var_ibci_dn9: f64,
        var_ibebtb: f64,
        var_ibebtb_dn0: f64,
        var_ibebtb_dn1: f64,
        var_ibebtb_dn3: f64,
        var_ibebtb_dn4: f64,
        var_ibebtb_dn5: f64,
        var_ibebtb_dn6: f64,
        var_ibebtb_dn7: f64,
        var_ibebtb_dn8: f64,
        var_ibebtb_dn9: f64,
        var_ibei: f64,
        var_ibei_dn4: f64,
        var_ibei_dn5: f64,
        var_ibei_dn6: f64,
        var_ibei_dn7: f64,
        var_ibei_dn8: f64,
        var_ibei_dn9: f64,
        var_ibep: f64,
        var_ibep_dn4: f64,
        var_ibep_dn5: f64,
        var_ibep_dn6: f64,
        var_ibep_dn7: f64,
        var_ibep_dn8: f64,
        var_ibep_dn9: f64,
        var_ibetat: f64,
        var_ibetat_dn4: f64,
        var_ibetat_dn6: f64,
        var_ibetat_dn8: f64,
        var_ibh_rec: f64,
        var_ibh_rec_dn0: f64,
        var_ibh_rec_dn1: f64,
        var_ibh_rec_dn3: f64,
        var_ibh_rec_dn4: f64,
        var_ibh_rec_dn5: f64,
        var_ibh_rec_dn6: f64,
        var_ibh_rec_dn7: f64,
        var_ibh_rec_dn8: f64,
        var_ibh_rec_dn9: f64,
        var_ijbcx: f64,
        var_ijbcx_dn4: f64,
        var_ijbcx_dn5: f64,
        var_ijbcx_dn6: f64,
        var_ijbcx_dn7: f64,
        var_ijbcx_dn8: f64,
        var_ijbcx_dn9: f64,
        var_irei: f64,
        var_irei_dn4: f64,
        var_irei_dn5: f64,
        var_irei_dn6: f64,
        var_irei_dn7: f64,
        var_irei_dn8: f64,
        var_irei_dn9: f64,
        var_irep: f64,
        var_irep_dn4: f64,
        var_irep_dn5: f64,
        var_irep_dn6: f64,
        var_irep_dn7: f64,
        var_irep_dn8: f64,
        var_irep_dn9: f64,
        var_it_sub: f64,
        var_it_sub_dn4: f64,
        var_it_sub_dn5: f64,
        var_it_sub_dn7: f64,
        var_it_sub_dn9: f64,
        var_itr: f64,
        var_itr_dn0: f64,
        var_itr_dn1: f64,
        var_itr_dn3: f64,
        var_itr_dn4: f64,
        var_itr_dn5: f64,
        var_itr_dn6: f64,
        var_itr_dn7: f64,
        var_itr_dn8: f64,
        var_itr_dn9: f64,
        var_itxf: f64,
        var_itxf_dn0: f64,
        var_itxf_dn1: f64,
        var_itxf_dn11: f64,
        var_itxf_dn3: f64,
        var_itxf_dn4: f64,
        var_itxf_dn5: f64,
        var_itxf_dn6: f64,
        var_itxf_dn7: f64,
        var_itxf_dn8: f64,
        var_itxf_dn9: f64,
        var_qdci: f64,
        var_qdci_dn0: f64,
        var_qdci_dn1: f64,
        var_qdci_dn3: f64,
        var_qdci_dn4: f64,
        var_qdci_dn5: f64,
        var_qdci_dn6: f64,
        var_qdci_dn7: f64,
        var_qdci_dn8: f64,
        var_qdci_dn9: f64,
        var_qdeix: f64,
        var_qdeix_dn0: f64,
        var_qdeix_dn1: f64,
        var_qdeix_dn12: f64,
        var_qdeix_dn3: f64,
        var_qdeix_dn4: f64,
        var_qdeix_dn5: f64,
        var_qdeix_dn6: f64,
        var_qdeix_dn7: f64,
        var_qdeix_dn8: f64,
        var_qdeix_dn9: f64,
        var_qdsu: f64,
        var_qdsu_dn4: f64,
        var_qdsu_dn5: f64,
        var_qdsu_dn7: f64,
        var_qjci: f64,
        var_qjci_dn0: f64,
        var_qjci_dn1: f64,
        var_qjci_dn3: f64,
        var_qjci_dn4: f64,
        var_qjci_dn5: f64,
        var_qjci_dn6: f64,
        var_qjci_dn7: f64,
        var_qjci_dn8: f64,
        var_qjci_dn9: f64,
        var_qjcx0_t_p: f64,
        var_qjcx0_t_p_dn0: f64,
        var_qjcx0_t_p_dn1: f64,
        var_qjcx0_t_p_dn3: f64,
        var_qjcx0_t_p_dn4: f64,
        var_qjcx0_t_p_dn5: f64,
        var_qjcx0_t_p_dn6: f64,
        var_qjcx0_t_p_dn7: f64,
        var_qjcx0_t_p_dn8: f64,
        var_qjcx0_t_p_dn9: f64,
        var_qjcx0_t_x: f64,
        var_qjcx0_t_x_dn0: f64,
        var_qjcx0_t_x_dn1: f64,
        var_qjcx0_t_x_dn3: f64,
        var_qjcx0_t_x_dn4: f64,
        var_qjcx0_t_x_dn5: f64,
        var_qjcx0_t_x_dn6: f64,
        var_qjcx0_t_x_dn7: f64,
        var_qjcx0_t_x_dn8: f64,
        var_qjcx0_t_x_dn9: f64,
        var_qjei: f64,
        var_qjei_dn0: f64,
        var_qjei_dn1: f64,
        var_qjei_dn3: f64,
        var_qjei_dn4: f64,
        var_qjei_dn5: f64,
        var_qjei_dn6: f64,
        var_qjei_dn7: f64,
        var_qjei_dn8: f64,
        var_qjei_dn9: f64,
        var_qjep: f64,
        var_qjep_dn0: f64,
        var_qjep_dn1: f64,
        var_qjep_dn3: f64,
        var_qjep_dn4: f64,
        var_qjep_dn5: f64,
        var_qjep_dn6: f64,
        var_qjep_dn7: f64,
        var_qjep_dn8: f64,
        var_qjep_dn9: f64,
        var_qrbi: f64,
        var_qrbi_dn0: f64,
        var_qrbi_dn1: f64,
        var_qrbi_dn3: f64,
        var_qrbi_dn4: f64,
        var_qrbi_dn5: f64,
        var_qrbi_dn6: f64,
        var_qrbi_dn7: f64,
        var_qrbi_dn8: f64,
        var_qrbi_dn9: f64,
        var_rbi: f64,
        var_rbi_dn0: f64,
        var_rbi_dn1: f64,
        var_rbi_dn3: f64,
        var_rbi_dn4: f64,
        var_rbi_dn5: f64,
        var_rbi_dn6: f64,
        var_rbi_dn7: f64,
        var_rbi_dn8: f64,
        var_rbi_dn9: f64,
        var_rbx_t: f64,
        var_rbx_t_dn4: f64,
        var_rcx_t: f64,
        var_rcx_t_dn4: f64,
        var_re_t: f64,
        var_re_t_dn4: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let eq0_e157: f64 = (var_ibei + var_irei);
        let eq0_e157_d_n4: f64 = (var_ibei_dn4 + var_irei_dn4);
        let eq0_e157_d_n5: f64 = (var_ibei_dn5 + var_irei_dn5);
        let eq0_e157_d_n6: f64 = (var_ibei_dn6 + var_irei_dn6);
        let eq0_e157_d_n7: f64 = (var_ibei_dn7 + var_irei_dn7);
        let eq0_e157_d_n8: f64 = (var_ibei_dn8 + var_irei_dn8);
        let eq0_e157_d_n9: f64 = (var_ibei_dn9 + var_irei_dn9);
        let eq0_e159: f64 = (eq0_e157 + var_ibetat);
        let eq0_e159_d_n4: f64 = (eq0_e157_d_n4 + var_ibetat_dn4);
        let eq0_e159_d_n6: f64 = (eq0_e157_d_n6 + var_ibetat_dn6);
        let eq0_e159_d_n8: f64 = (eq0_e157_d_n8 + var_ibetat_dn8);
        let eq0_e161: f64 = (eq0_e159 + var_ibh_rec);
        let eq0_e161_d_n4: f64 = (eq0_e159_d_n4 + var_ibh_rec_dn4);
        let eq0_e161_d_n5: f64 = (eq0_e157_d_n5 + var_ibh_rec_dn5);
        let eq0_e161_d_n6: f64 = (eq0_e159_d_n6 + var_ibh_rec_dn6);
        let eq0_e161_d_n7: f64 = (eq0_e157_d_n7 + var_ibh_rec_dn7);
        let eq0_e161_d_n8: f64 = (eq0_e159_d_n8 + var_ibh_rec_dn8);
        let eq0_e161_d_n9: f64 = (eq0_e157_d_n9 + var_ibh_rec_dn9);
        let eq0_e162: f64 = (p.p148 * eq0_e161);
        let eq0_e162_d_n0: f64 = (p.p148 * var_ibh_rec_dn0);
        let eq0_e162_d_n1: f64 = (p.p148 * var_ibh_rec_dn1);
        let eq0_e162_d_n3: f64 = (p.p148 * var_ibh_rec_dn3);
        let eq0_e162_d_n4: f64 = (p.p148 * eq0_e161_d_n4);
        let eq0_e162_d_n5: f64 = (p.p148 * eq0_e161_d_n5);
        let eq0_e162_d_n6: f64 = (p.p148 * eq0_e161_d_n6);
        let eq0_e162_d_n7: f64 = (p.p148 * eq0_e161_d_n7);
        let eq0_e162_d_n8: f64 = (p.p148 * eq0_e161_d_n8);
        let eq0_e162_d_n9: f64 = (p.p148 * eq0_e161_d_n9);
        let eq0_e165: f64 = (var_gmin * (nv8 - nv6));
        let eq0_e166: f64 = (eq0_e162 + eq0_e165);
        let eq0_e166_d_n6: f64 = (eq0_e162_d_n6 + (-var_gmin));
        let eq0_e166_d_n8: f64 = (eq0_e162_d_n8 + var_gmin);
        let eq0_value: f64 = eq0_e166;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq0_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq0_e162_d_n0), multiplicity * (eq0_e162_d_n1), multiplicity * (eq0_e162_d_n3), multiplicity * (eq0_e162_d_n4), multiplicity * (eq0_e162_d_n5), multiplicity * (eq0_e166_d_n6), multiplicity * (eq0_e162_d_n7), multiplicity * (eq0_e166_d_n8), multiplicity * (eq0_e162_d_n9)],
            [],
            [],
            1.0,
        );
        let eq1_e170: f64 = (var_qdeix + var_qjei);
        let eq1_e170_d_n0: f64 = (var_qdeix_dn0 + var_qjei_dn0);
        let eq1_e170_d_n1: f64 = (var_qdeix_dn1 + var_qjei_dn1);
        let eq1_e170_d_n3: f64 = (var_qdeix_dn3 + var_qjei_dn3);
        let eq1_e170_d_n4: f64 = (var_qdeix_dn4 + var_qjei_dn4);
        let eq1_e170_d_n5: f64 = (var_qdeix_dn5 + var_qjei_dn5);
        let eq1_e170_d_n6: f64 = (var_qdeix_dn6 + var_qjei_dn6);
        let eq1_e170_d_n7: f64 = (var_qdeix_dn7 + var_qjei_dn7);
        let eq1_e170_d_n8: f64 = (var_qdeix_dn8 + var_qjei_dn8);
        let eq1_e170_d_n9: f64 = (var_qdeix_dn9 + var_qjei_dn9);
        let eq1_e171: f64 = (p.p148 * eq1_e170);
        let eq1_e171_d_n0: f64 = (p.p148 * eq1_e170_d_n0);
        let eq1_e171_d_n1: f64 = (p.p148 * eq1_e170_d_n1);
        let eq1_e171_d_n3: f64 = (p.p148 * eq1_e170_d_n3);
        let eq1_e171_d_n4: f64 = (p.p148 * eq1_e170_d_n4);
        let eq1_e171_d_n5: f64 = (p.p148 * eq1_e170_d_n5);
        let eq1_e171_d_n6: f64 = (p.p148 * eq1_e170_d_n6);
        let eq1_e171_d_n7: f64 = (p.p148 * eq1_e170_d_n7);
        let eq1_e171_d_n8: f64 = (p.p148 * eq1_e170_d_n8);
        let eq1_e171_d_n9: f64 = (p.p148 * eq1_e170_d_n9);
        let eq1_e171_d_n12: f64 = (p.p148 * var_qdeix_dn12);
        let eq1_e172: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq1_e171);
        let eq1_value: f64 = eq1_e172;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq1_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 12],
            [multiplicity * ((eq1_e171_d_n0 * ddt_scale)), multiplicity * ((eq1_e171_d_n1 * ddt_scale)), multiplicity * ((eq1_e171_d_n3 * ddt_scale)), multiplicity * ((eq1_e171_d_n4 * ddt_scale)), multiplicity * ((eq1_e171_d_n5 * ddt_scale)), multiplicity * ((eq1_e171_d_n6 * ddt_scale)), multiplicity * ((eq1_e171_d_n7 * ddt_scale)), multiplicity * ((eq1_e171_d_n8 * ddt_scale)), multiplicity * ((eq1_e171_d_n9 * ddt_scale)), multiplicity * ((eq1_e171_d_n12 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq2_e176: f64 = (var_ibci - var_iavl);
        let eq2_e176_d_n4: f64 = (var_ibci_dn4 - var_iavl_dn4);
        let eq2_e176_d_n5: f64 = (var_ibci_dn5 - var_iavl_dn5);
        let eq2_e176_d_n6: f64 = (var_ibci_dn6 - var_iavl_dn6);
        let eq2_e176_d_n7: f64 = (var_ibci_dn7 - var_iavl_dn7);
        let eq2_e176_d_n8: f64 = (var_ibci_dn8 - var_iavl_dn8);
        let eq2_e176_d_n9: f64 = (var_ibci_dn9 - var_iavl_dn9);
        let eq2_e177: f64 = (p.p148 * eq2_e176);
        let eq2_e177_d_n0: f64 = (p.p148 * (-var_iavl_dn0));
        let eq2_e177_d_n1: f64 = (p.p148 * (-var_iavl_dn1));
        let eq2_e177_d_n3: f64 = (p.p148 * (-var_iavl_dn3));
        let eq2_e177_d_n4: f64 = (p.p148 * eq2_e176_d_n4);
        let eq2_e177_d_n5: f64 = (p.p148 * eq2_e176_d_n5);
        let eq2_e177_d_n6: f64 = (p.p148 * eq2_e176_d_n6);
        let eq2_e177_d_n7: f64 = (p.p148 * eq2_e176_d_n7);
        let eq2_e177_d_n8: f64 = (p.p148 * eq2_e176_d_n8);
        let eq2_e177_d_n9: f64 = (p.p148 * eq2_e176_d_n9);
        let eq2_e180: f64 = (var_gmin * (nv8 - nv5));
        let eq2_e181: f64 = (eq2_e177 + eq2_e180);
        let eq2_e181_d_n5: f64 = (eq2_e177_d_n5 + (-var_gmin));
        let eq2_e181_d_n8: f64 = (eq2_e177_d_n8 + var_gmin);
        let eq2_value: f64 = eq2_e181;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq2_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq2_e177_d_n0), multiplicity * (eq2_e177_d_n1), multiplicity * (eq2_e177_d_n3), multiplicity * (eq2_e177_d_n4), multiplicity * (eq2_e181_d_n5), multiplicity * (eq2_e177_d_n6), multiplicity * (eq2_e177_d_n7), multiplicity * (eq2_e181_d_n8), multiplicity * (eq2_e177_d_n9)],
            [],
            [],
            1.0,
        );
        let eq3_e185: f64 = (var_qdci + var_qjci);
        let eq3_e185_d_n0: f64 = (var_qdci_dn0 + var_qjci_dn0);
        let eq3_e185_d_n1: f64 = (var_qdci_dn1 + var_qjci_dn1);
        let eq3_e185_d_n3: f64 = (var_qdci_dn3 + var_qjci_dn3);
        let eq3_e185_d_n4: f64 = (var_qdci_dn4 + var_qjci_dn4);
        let eq3_e185_d_n5: f64 = (var_qdci_dn5 + var_qjci_dn5);
        let eq3_e185_d_n6: f64 = (var_qdci_dn6 + var_qjci_dn6);
        let eq3_e185_d_n7: f64 = (var_qdci_dn7 + var_qjci_dn7);
        let eq3_e185_d_n8: f64 = (var_qdci_dn8 + var_qjci_dn8);
        let eq3_e185_d_n9: f64 = (var_qdci_dn9 + var_qjci_dn9);
        let eq3_e186: f64 = (p.p148 * eq3_e185);
        let eq3_e186_d_n0: f64 = (p.p148 * eq3_e185_d_n0);
        let eq3_e186_d_n1: f64 = (p.p148 * eq3_e185_d_n1);
        let eq3_e186_d_n3: f64 = (p.p148 * eq3_e185_d_n3);
        let eq3_e186_d_n4: f64 = (p.p148 * eq3_e185_d_n4);
        let eq3_e186_d_n5: f64 = (p.p148 * eq3_e185_d_n5);
        let eq3_e186_d_n6: f64 = (p.p148 * eq3_e185_d_n6);
        let eq3_e186_d_n7: f64 = (p.p148 * eq3_e185_d_n7);
        let eq3_e186_d_n8: f64 = (p.p148 * eq3_e185_d_n8);
        let eq3_e186_d_n9: f64 = (p.p148 * eq3_e185_d_n9);
        let eq3_e187: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq3_e186);
        let eq3_value: f64 = eq3_e187;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq3_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq3_e186_d_n0 * ddt_scale)), multiplicity * ((eq3_e186_d_n1 * ddt_scale)), multiplicity * ((eq3_e186_d_n3 * ddt_scale)), multiplicity * ((eq3_e186_d_n4 * ddt_scale)), multiplicity * ((eq3_e186_d_n5 * ddt_scale)), multiplicity * ((eq3_e186_d_n6 * ddt_scale)), multiplicity * ((eq3_e186_d_n7 * ddt_scale)), multiplicity * ((eq3_e186_d_n8 * ddt_scale)), multiplicity * ((eq3_e186_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq4_e190: f64 = (p.p148 * var_itxf);
        let eq4_e190_d_n0: f64 = (p.p148 * var_itxf_dn0);
        let eq4_e190_d_n1: f64 = (p.p148 * var_itxf_dn1);
        let eq4_e190_d_n3: f64 = (p.p148 * var_itxf_dn3);
        let eq4_e190_d_n4: f64 = (p.p148 * var_itxf_dn4);
        let eq4_e190_d_n5: f64 = (p.p148 * var_itxf_dn5);
        let eq4_e190_d_n6: f64 = (p.p148 * var_itxf_dn6);
        let eq4_e190_d_n7: f64 = (p.p148 * var_itxf_dn7);
        let eq4_e190_d_n8: f64 = (p.p148 * var_itxf_dn8);
        let eq4_e190_d_n9: f64 = (p.p148 * var_itxf_dn9);
        let eq4_e190_d_n11: f64 = (p.p148 * var_itxf_dn11);
        let eq4_value: f64 = eq4_e190;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq4_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 11],
            [multiplicity * (eq4_e190_d_n0), multiplicity * (eq4_e190_d_n1), multiplicity * (eq4_e190_d_n3), multiplicity * (eq4_e190_d_n4), multiplicity * (eq4_e190_d_n5), multiplicity * (eq4_e190_d_n6), multiplicity * (eq4_e190_d_n7), multiplicity * (eq4_e190_d_n8), multiplicity * (eq4_e190_d_n9), multiplicity * (eq4_e190_d_n11)],
            [],
            [],
            1.0,
        );
        let eq5_e193: f64 = (p.p148 * var_itr);
        let eq5_e193_d_n0: f64 = (p.p148 * var_itr_dn0);
        let eq5_e193_d_n1: f64 = (p.p148 * var_itr_dn1);
        let eq5_e193_d_n3: f64 = (p.p148 * var_itr_dn3);
        let eq5_e193_d_n4: f64 = (p.p148 * var_itr_dn4);
        let eq5_e193_d_n5: f64 = (p.p148 * var_itr_dn5);
        let eq5_e193_d_n6: f64 = (p.p148 * var_itr_dn6);
        let eq5_e193_d_n7: f64 = (p.p148 * var_itr_dn7);
        let eq5_e193_d_n8: f64 = (p.p148 * var_itr_dn8);
        let eq5_e193_d_n9: f64 = (p.p148 * var_itr_dn9);
        let eq5_value: f64 = eq5_e193;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(5),
            multiplicity * (eq5_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq5_e193_d_n0), multiplicity * (eq5_e193_d_n1), multiplicity * (eq5_e193_d_n3), multiplicity * (eq5_e193_d_n4), multiplicity * (eq5_e193_d_n5), multiplicity * (eq5_e193_d_n6), multiplicity * (eq5_e193_d_n7), multiplicity * (eq5_e193_d_n8), multiplicity * (eq5_e193_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq6_e199, eq6_e199_d_n0, eq6_e199_d_n1, eq6_e199_d_n3, eq6_e199_d_n4, eq6_e199_d_n5, eq6_e199_d_n6, eq6_e199_d_n7, eq6_e199_d_n8, eq6_e199_d_n9,) = {
    if (var_guard233 != 0.0) {
        let eq6_e197: f64 = ((nv7 - nv8) / var_rbi);
        let eq6_e197_d_n0: f64 = (-(((nv7 - nv8) * var_rbi_dn0) / (var_rbi * var_rbi)));
        let eq6_e197_d_n1: f64 = (-(((nv7 - nv8) * var_rbi_dn1) / (var_rbi * var_rbi)));
        let eq6_e197_d_n3: f64 = (-(((nv7 - nv8) * var_rbi_dn3) / (var_rbi * var_rbi)));
        let eq6_e197_d_n4: f64 = (-(((nv7 - nv8) * var_rbi_dn4) / (var_rbi * var_rbi)));
        let eq6_e197_d_n5: f64 = (-(((nv7 - nv8) * var_rbi_dn5) / (var_rbi * var_rbi)));
        let eq6_e197_d_n6: f64 = (-(((nv7 - nv8) * var_rbi_dn6) / (var_rbi * var_rbi)));
        let __rspice_inv_cse_0: f64 = 1.0 / (var_rbi * var_rbi);
        let eq6_e197_d_n7: f64 = ((var_rbi - ((nv7 - nv8) * var_rbi_dn7)) * __rspice_inv_cse_0);
        let eq6_e197_d_n8: f64 = (((-var_rbi) - ((nv7 - nv8) * var_rbi_dn8)) * __rspice_inv_cse_0);
        let eq6_e197_d_n9: f64 = (-(((nv7 - nv8) * var_rbi_dn9) / (var_rbi * var_rbi)));
        (eq6_e197, eq6_e197_d_n0, eq6_e197_d_n1, eq6_e197_d_n3, eq6_e197_d_n4, eq6_e197_d_n5, eq6_e197_d_n6, eq6_e197_d_n7, eq6_e197_d_n8, eq6_e197_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e199;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(7),
            Some(8),
            multiplicity * (eq6_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq6_e199_d_n0), multiplicity * (eq6_e199_d_n1), multiplicity * (eq6_e199_d_n3), multiplicity * (eq6_e199_d_n4), multiplicity * (eq6_e199_d_n5), multiplicity * (eq6_e199_d_n6), multiplicity * (eq6_e199_d_n7), multiplicity * (eq6_e199_d_n8), multiplicity * (eq6_e199_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq7_e206, eq7_e206_d_n0, eq7_e206_d_n1, eq7_e206_d_n3, eq7_e206_d_n4, eq7_e206_d_n5, eq7_e206_d_n6, eq7_e206_d_n7, eq7_e206_d_n8, eq7_e206_d_n9,) = {
    if ((var_guard233 != 0.0) && (var_guard234 != 0.0)) {
        let eq7_e204: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, var_qrbi);
        (eq7_e204, (var_qrbi_dn0 * ddt_scale), (var_qrbi_dn1 * ddt_scale), (var_qrbi_dn3 * ddt_scale), (var_qrbi_dn4 * ddt_scale), (var_qrbi_dn5 * ddt_scale), (var_qrbi_dn6 * ddt_scale), (var_qrbi_dn7 * ddt_scale), (var_qrbi_dn8 * ddt_scale), (var_qrbi_dn9 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e206;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(7),
            Some(8),
            multiplicity * (eq7_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq7_e206_d_n0), multiplicity * (eq7_e206_d_n1), multiplicity * (eq7_e206_d_n3), multiplicity * (eq7_e206_d_n4), multiplicity * (eq7_e206_d_n5), multiplicity * (eq7_e206_d_n6), multiplicity * (eq7_e206_d_n7), multiplicity * (eq7_e206_d_n8), multiplicity * (eq7_e206_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq9_e218, eq9_e218_d_n0, eq9_e218_d_n1, eq9_e218_d_n3, eq9_e218_d_n4, eq9_e218_d_n5, eq9_e218_d_n6, eq9_e218_d_n7, eq9_e218_d_n8, eq9_e218_d_n9,) = {
    if (var_guard235 != 0.0) {
        let eq9_e214: f64 = (-p.p148);
        let eq9_e216: f64 = (eq9_e214 * var_ibebtb);
        let eq9_e216_d_n0: f64 = (eq9_e214 * var_ibebtb_dn0);
        let eq9_e216_d_n1: f64 = (eq9_e214 * var_ibebtb_dn1);
        let eq9_e216_d_n3: f64 = (eq9_e214 * var_ibebtb_dn3);
        let eq9_e216_d_n4: f64 = (eq9_e214 * var_ibebtb_dn4);
        let eq9_e216_d_n5: f64 = (eq9_e214 * var_ibebtb_dn5);
        let eq9_e216_d_n6: f64 = (eq9_e214 * var_ibebtb_dn6);
        let eq9_e216_d_n7: f64 = (eq9_e214 * var_ibebtb_dn7);
        let eq9_e216_d_n8: f64 = (eq9_e214 * var_ibebtb_dn8);
        let eq9_e216_d_n9: f64 = (eq9_e214 * var_ibebtb_dn9);
        (eq9_e216, eq9_e216_d_n0, eq9_e216_d_n1, eq9_e216_d_n3, eq9_e216_d_n4, eq9_e216_d_n5, eq9_e216_d_n6, eq9_e216_d_n7, eq9_e216_d_n8, eq9_e216_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e218;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq9_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq9_e218_d_n0), multiplicity * (eq9_e218_d_n1), multiplicity * (eq9_e218_d_n3), multiplicity * (eq9_e218_d_n4), multiplicity * (eq9_e218_d_n5), multiplicity * (eq9_e218_d_n6), multiplicity * (eq9_e218_d_n7), multiplicity * (eq9_e218_d_n8), multiplicity * (eq9_e218_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq10_e226, eq10_e226_d_n0, eq10_e226_d_n1, eq10_e226_d_n3, eq10_e226_d_n4, eq10_e226_d_n5, eq10_e226_d_n6, eq10_e226_d_n7, eq10_e226_d_n8, eq10_e226_d_n9,) = {
    if (var_guard235 == 0.0) {
        let eq10_e222: f64 = (-p.p148);
        let eq10_e224: f64 = (eq10_e222 * var_ibebtb);
        let eq10_e224_d_n0: f64 = (eq10_e222 * var_ibebtb_dn0);
        let eq10_e224_d_n1: f64 = (eq10_e222 * var_ibebtb_dn1);
        let eq10_e224_d_n3: f64 = (eq10_e222 * var_ibebtb_dn3);
        let eq10_e224_d_n4: f64 = (eq10_e222 * var_ibebtb_dn4);
        let eq10_e224_d_n5: f64 = (eq10_e222 * var_ibebtb_dn5);
        let eq10_e224_d_n6: f64 = (eq10_e222 * var_ibebtb_dn6);
        let eq10_e224_d_n7: f64 = (eq10_e222 * var_ibebtb_dn7);
        let eq10_e224_d_n8: f64 = (eq10_e222 * var_ibebtb_dn8);
        let eq10_e224_d_n9: f64 = (eq10_e222 * var_ibebtb_dn9);
        (eq10_e224, eq10_e224_d_n0, eq10_e224_d_n1, eq10_e224_d_n3, eq10_e224_d_n4, eq10_e224_d_n5, eq10_e224_d_n6, eq10_e224_d_n7, eq10_e224_d_n8, eq10_e224_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e226;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq10_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq10_e226_d_n0), multiplicity * (eq10_e226_d_n1), multiplicity * (eq10_e226_d_n3), multiplicity * (eq10_e226_d_n4), multiplicity * (eq10_e226_d_n5), multiplicity * (eq10_e226_d_n6), multiplicity * (eq10_e226_d_n7), multiplicity * (eq10_e226_d_n8), multiplicity * (eq10_e226_d_n9)],
            [],
            [],
            1.0,
        );
        let eq11_e228: f64 = (-p.p148);
        let eq11_e230: f64 = (eq11_e228 * var_ibcbtb);
        let eq11_e230_d_n0: f64 = (eq11_e228 * var_ibcbtb_dn0);
        let eq11_e230_d_n1: f64 = (eq11_e228 * var_ibcbtb_dn1);
        let eq11_e230_d_n3: f64 = (eq11_e228 * var_ibcbtb_dn3);
        let eq11_e230_d_n4: f64 = (eq11_e228 * var_ibcbtb_dn4);
        let eq11_e230_d_n5: f64 = (eq11_e228 * var_ibcbtb_dn5);
        let eq11_e230_d_n6: f64 = (eq11_e228 * var_ibcbtb_dn6);
        let eq11_e230_d_n7: f64 = (eq11_e228 * var_ibcbtb_dn7);
        let eq11_e230_d_n8: f64 = (eq11_e228 * var_ibcbtb_dn8);
        let eq11_e230_d_n9: f64 = (eq11_e228 * var_ibcbtb_dn9);
        let eq11_value: f64 = eq11_e230;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq11_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq11_e230_d_n0), multiplicity * (eq11_e230_d_n1), multiplicity * (eq11_e230_d_n3), multiplicity * (eq11_e230_d_n4), multiplicity * (eq11_e230_d_n5), multiplicity * (eq11_e230_d_n6), multiplicity * (eq11_e230_d_n7), multiplicity * (eq11_e230_d_n8), multiplicity * (eq11_e230_d_n9)],
            [],
            [],
            1.0,
        );
        let eq12_e234: f64 = (var_ibep + var_irep);
        let eq12_e234_d_n4: f64 = (var_ibep_dn4 + var_irep_dn4);
        let eq12_e234_d_n5: f64 = (var_ibep_dn5 + var_irep_dn5);
        let eq12_e234_d_n6: f64 = (var_ibep_dn6 + var_irep_dn6);
        let eq12_e234_d_n7: f64 = (var_ibep_dn7 + var_irep_dn7);
        let eq12_e234_d_n8: f64 = (var_ibep_dn8 + var_irep_dn8);
        let eq12_e234_d_n9: f64 = (var_ibep_dn9 + var_irep_dn9);
        let eq12_e235: f64 = (p.p148 * eq12_e234);
        let eq12_e235_d_n4: f64 = (p.p148 * eq12_e234_d_n4);
        let eq12_e235_d_n5: f64 = (p.p148 * eq12_e234_d_n5);
        let eq12_e235_d_n6: f64 = (p.p148 * eq12_e234_d_n6);
        let eq12_e235_d_n7: f64 = (p.p148 * eq12_e234_d_n7);
        let eq12_e235_d_n8: f64 = (p.p148 * eq12_e234_d_n8);
        let eq12_e235_d_n9: f64 = (p.p148 * eq12_e234_d_n9);
        let eq12_value: f64 = eq12_e235;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq12_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq12_e235_d_n4), multiplicity * (eq12_e235_d_n5), multiplicity * (eq12_e235_d_n6), multiplicity * (eq12_e235_d_n7), multiplicity * (eq12_e235_d_n8), multiplicity * (eq12_e235_d_n9)],
            [],
            [],
            1.0,
        );
        let eq13_e238: f64 = (p.p148 * var_qjep);
        let eq13_e238_d_n0: f64 = (p.p148 * var_qjep_dn0);
        let eq13_e238_d_n1: f64 = (p.p148 * var_qjep_dn1);
        let eq13_e238_d_n3: f64 = (p.p148 * var_qjep_dn3);
        let eq13_e238_d_n4: f64 = (p.p148 * var_qjep_dn4);
        let eq13_e238_d_n5: f64 = (p.p148 * var_qjep_dn5);
        let eq13_e238_d_n6: f64 = (p.p148 * var_qjep_dn6);
        let eq13_e238_d_n7: f64 = (p.p148 * var_qjep_dn7);
        let eq13_e238_d_n8: f64 = (p.p148 * var_qjep_dn8);
        let eq13_e238_d_n9: f64 = (p.p148 * var_qjep_dn9);
        let eq13_e239: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq13_e238);
        let eq13_value: f64 = eq13_e239;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq13_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq13_e238_d_n0 * ddt_scale)), multiplicity * ((eq13_e238_d_n1 * ddt_scale)), multiplicity * ((eq13_e238_d_n3 * ddt_scale)), multiplicity * ((eq13_e238_d_n4 * ddt_scale)), multiplicity * ((eq13_e238_d_n5 * ddt_scale)), multiplicity * ((eq13_e238_d_n6 * ddt_scale)), multiplicity * ((eq13_e238_d_n7 * ddt_scale)), multiplicity * ((eq13_e238_d_n8 * ddt_scale)), multiplicity * ((eq13_e238_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq14_e242: f64 = (p.p148 * var_ijbcx);
        let eq14_e242_d_n4: f64 = (p.p148 * var_ijbcx_dn4);
        let eq14_e242_d_n5: f64 = (p.p148 * var_ijbcx_dn5);
        let eq14_e242_d_n6: f64 = (p.p148 * var_ijbcx_dn6);
        let eq14_e242_d_n7: f64 = (p.p148 * var_ijbcx_dn7);
        let eq14_e242_d_n8: f64 = (p.p148 * var_ijbcx_dn8);
        let eq14_e242_d_n9: f64 = (p.p148 * var_ijbcx_dn9);
        let eq14_value: f64 = eq14_e242;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(5),
            multiplicity * (eq14_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq14_e242_d_n4), multiplicity * (eq14_e242_d_n5), multiplicity * (eq14_e242_d_n6), multiplicity * (eq14_e242_d_n7), multiplicity * (eq14_e242_d_n8), multiplicity * (eq14_e242_d_n9)],
            [],
            [],
            1.0,
        );
        let eq15_e246: f64 = (var_qjcx0_t_p + var_qdsu);
        let eq15_e246_d_n4: f64 = (var_qjcx0_t_p_dn4 + var_qdsu_dn4);
        let eq15_e246_d_n5: f64 = (var_qjcx0_t_p_dn5 + var_qdsu_dn5);
        let eq15_e246_d_n7: f64 = (var_qjcx0_t_p_dn7 + var_qdsu_dn7);
        let eq15_e247: f64 = (p.p148 * eq15_e246);
        let eq15_e247_d_n0: f64 = (p.p148 * var_qjcx0_t_p_dn0);
        let eq15_e247_d_n1: f64 = (p.p148 * var_qjcx0_t_p_dn1);
        let eq15_e247_d_n3: f64 = (p.p148 * var_qjcx0_t_p_dn3);
        let eq15_e247_d_n4: f64 = (p.p148 * eq15_e246_d_n4);
        let eq15_e247_d_n5: f64 = (p.p148 * eq15_e246_d_n5);
        let eq15_e247_d_n6: f64 = (p.p148 * var_qjcx0_t_p_dn6);
        let eq15_e247_d_n7: f64 = (p.p148 * eq15_e246_d_n7);
        let eq15_e247_d_n8: f64 = (p.p148 * var_qjcx0_t_p_dn8);
        let eq15_e247_d_n9: f64 = (p.p148 * var_qjcx0_t_p_dn9);
        let eq15_e248: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq15_e247);
        let eq15_value: f64 = eq15_e248;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(7),
            Some(5),
            multiplicity * (eq15_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq15_e247_d_n0 * ddt_scale)), multiplicity * ((eq15_e247_d_n1 * ddt_scale)), multiplicity * ((eq15_e247_d_n3 * ddt_scale)), multiplicity * ((eq15_e247_d_n4 * ddt_scale)), multiplicity * ((eq15_e247_d_n5 * ddt_scale)), multiplicity * ((eq15_e247_d_n6 * ddt_scale)), multiplicity * ((eq15_e247_d_n7 * ddt_scale)), multiplicity * ((eq15_e247_d_n8 * ddt_scale)), multiplicity * ((eq15_e247_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq17_e255: f64 = (p.p148 * var_qjcx0_t_x);
        let eq17_e255_d_n0: f64 = (p.p148 * var_qjcx0_t_x_dn0);
        let eq17_e255_d_n1: f64 = (p.p148 * var_qjcx0_t_x_dn1);
        let eq17_e255_d_n3: f64 = (p.p148 * var_qjcx0_t_x_dn3);
        let eq17_e255_d_n4: f64 = (p.p148 * var_qjcx0_t_x_dn4);
        let eq17_e255_d_n5: f64 = (p.p148 * var_qjcx0_t_x_dn5);
        let eq17_e255_d_n6: f64 = (p.p148 * var_qjcx0_t_x_dn6);
        let eq17_e255_d_n7: f64 = (p.p148 * var_qjcx0_t_x_dn7);
        let eq17_e255_d_n8: f64 = (p.p148 * var_qjcx0_t_x_dn8);
        let eq17_e255_d_n9: f64 = (p.p148 * var_qjcx0_t_x_dn9);
        let eq17_e256: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq17_e255);
        let eq17_value: f64 = eq17_e256;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(5),
            multiplicity * (eq17_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq17_e255_d_n0 * ddt_scale)), multiplicity * ((eq17_e255_d_n1 * ddt_scale)), multiplicity * ((eq17_e255_d_n3 * ddt_scale)), multiplicity * ((eq17_e255_d_n4 * ddt_scale)), multiplicity * ((eq17_e255_d_n5 * ddt_scale)), multiplicity * ((eq17_e255_d_n6 * ddt_scale)), multiplicity * ((eq17_e255_d_n7 * ddt_scale)), multiplicity * ((eq17_e255_d_n8 * ddt_scale)), multiplicity * ((eq17_e255_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let (eq19_e266, eq19_e266_d_n1, eq19_e266_d_n4, eq19_e266_d_n7,) = {
    if (var_guard236 != 0.0) {
        let __rspice_inv_cse_1: f64 = 1.0 / var_rbx_t;
        let eq19_e264: f64 = ((nv1 - nv7) * __rspice_inv_cse_1);
        let eq19_e264_d_n1: f64 = (1.0 * __rspice_inv_cse_1);
        let eq19_e264_d_n4: f64 = (-(((nv1 - nv7) * var_rbx_t_dn4) / (var_rbx_t * var_rbx_t)));
        let eq19_e264_d_n7: f64 = (-1.0 / var_rbx_t);
        (eq19_e264, eq19_e264_d_n1, eq19_e264_d_n4, eq19_e264_d_n7,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e266;
        stamper.stamp_current_node3_local(
            Some(1),
            Some(7),
            multiplicity * (eq19_value),
            1,
            multiplicity * (eq19_e266_d_n1),
            4,
            multiplicity * (eq19_e266_d_n4),
            7,
            multiplicity * (eq19_e266_d_n7),
        );
        let (eq21_e277, eq21_e277_d_n2, eq21_e277_d_n4, eq21_e277_d_n6,) = {
    if (var_guard237 != 0.0) {
        let __rspice_inv_cse_2: f64 = 1.0 / var_re_t;
        let eq21_e275: f64 = ((nv6 - nv2) * __rspice_inv_cse_2);
        let eq21_e275_d_n2: f64 = ((-1.0) * __rspice_inv_cse_2);
        let eq21_e275_d_n4: f64 = (-(((nv6 - nv2) * var_re_t_dn4) / (var_re_t * var_re_t)));
        let eq21_e275_d_n6: f64 = (1.0 / var_re_t);
        (eq21_e275, eq21_e275_d_n2, eq21_e275_d_n4, eq21_e275_d_n6,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e277;
        stamper.stamp_current_node3_local(
            Some(6),
            Some(2),
            multiplicity * (eq21_value),
            2,
            multiplicity * (eq21_e277_d_n2),
            4,
            multiplicity * (eq21_e277_d_n4),
            6,
            multiplicity * (eq21_e277_d_n6),
        );
        let (eq23_e288, eq23_e288_d_n0, eq23_e288_d_n4, eq23_e288_d_n5,) = {
    if (var_guard238 != 0.0) {
        let __rspice_inv_cse_3: f64 = 1.0 / var_rcx_t;
        let eq23_e286: f64 = ((nv5 - nv0) * __rspice_inv_cse_3);
        let eq23_e286_d_n0: f64 = ((-1.0) * __rspice_inv_cse_3);
        let eq23_e286_d_n4: f64 = (-(((nv5 - nv0) * var_rcx_t_dn4) / (var_rcx_t * var_rcx_t)));
        let eq23_e286_d_n5: f64 = (1.0 / var_rcx_t);
        (eq23_e286, eq23_e286_d_n0, eq23_e286_d_n4, eq23_e286_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e288;
        stamper.stamp_current_node3_local(
            Some(5),
            Some(0),
            multiplicity * (eq23_value),
            0,
            multiplicity * (eq23_e288_d_n0),
            4,
            multiplicity * (eq23_e288_d_n4),
            5,
            multiplicity * (eq23_e288_d_n5),
        );
        let eq28_e308: f64 = (p.p148 * var_it_sub);
        let eq28_e308_d_n4: f64 = (p.p148 * var_it_sub_dn4);
        let eq28_e308_d_n5: f64 = (p.p148 * var_it_sub_dn5);
        let eq28_e308_d_n7: f64 = (p.p148 * var_it_sub_dn7);
        let eq28_e308_d_n9: f64 = (p.p148 * var_it_sub_dn9);
        let eq28_value: f64 = eq28_e308;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq28_value),
            [4, 5, 7, 9],
            [multiplicity * (eq28_e308_d_n4), multiplicity * (eq28_e308_d_n5), multiplicity * (eq28_e308_d_n7), multiplicity * (eq28_e308_d_n9)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_guard239: f64,
        var_guard240: f64,
        var_guard242: f64,
        var_guard243: f64,
        var_guard244: f64,
        var_guard245: f64,
        var_guard258: f64,
        var_ijsc: f64,
        var_ijsc_dn4: f64,
        var_ijsc_dn5: f64,
        var_ijsc_dn6: f64,
        var_ijsc_dn7: f64,
        var_ijsc_dn8: f64,
        var_ijsc_dn9: f64,
        var_ixf: f64,
        var_ixf1: f64,
        var_ixf1_dn0: f64,
        var_ixf1_dn1: f64,
        var_ixf1_dn10: f64,
        var_ixf1_dn11: f64,
        var_ixf1_dn3: f64,
        var_ixf1_dn4: f64,
        var_ixf1_dn5: f64,
        var_ixf1_dn6: f64,
        var_ixf1_dn7: f64,
        var_ixf1_dn8: f64,
        var_ixf1_dn9: f64,
        var_ixf2: f64,
        var_ixf2_dn0: f64,
        var_ixf2_dn1: f64,
        var_ixf2_dn10: f64,
        var_ixf2_dn11: f64,
        var_ixf2_dn3: f64,
        var_ixf2_dn4: f64,
        var_ixf2_dn5: f64,
        var_ixf2_dn6: f64,
        var_ixf2_dn7: f64,
        var_ixf2_dn8: f64,
        var_ixf2_dn9: f64,
        var_ixf_dn0: f64,
        var_ixf_dn1: f64,
        var_ixf_dn12: f64,
        var_ixf_dn3: f64,
        var_ixf_dn4: f64,
        var_ixf_dn5: f64,
        var_ixf_dn6: f64,
        var_ixf_dn7: f64,
        var_ixf_dn8: f64,
        var_ixf_dn9: f64,
        var_n_1: f64,
        var_n_1_dn0: f64,
        var_n_1_dn1: f64,
        var_n_1_dn3: f64,
        var_n_1_dn4: f64,
        var_n_1_dn5: f64,
        var_n_1_dn6: f64,
        var_n_1_dn7: f64,
        var_n_1_dn8: f64,
        var_n_1_dn9: f64,
        var_n_2: f64,
        var_n_2_dn0: f64,
        var_n_2_dn1: f64,
        var_n_2_dn3: f64,
        var_n_2_dn4: f64,
        var_n_2_dn5: f64,
        var_n_2_dn6: f64,
        var_n_2_dn7: f64,
        var_n_2_dn8: f64,
        var_n_2_dn9: f64,
        var_n_w: f64,
        var_pterm: f64,
        var_pterm_dn0: f64,
        var_pterm_dn1: f64,
        var_pterm_dn2: f64,
        var_pterm_dn3: f64,
        var_pterm_dn4: f64,
        var_pterm_dn5: f64,
        var_pterm_dn6: f64,
        var_pterm_dn7: f64,
        var_pterm_dn8: f64,
        var_pterm_dn9: f64,
        var_qjs: f64,
        var_qjs_dn0: f64,
        var_qjs_dn1: f64,
        var_qjs_dn3: f64,
        var_qjs_dn4: f64,
        var_qjs_dn5: f64,
        var_qjs_dn6: f64,
        var_qjs_dn7: f64,
        var_qjs_dn8: f64,
        var_qjs_dn9: f64,
        var_qscp: f64,
        var_qscp_dn0: f64,
        var_qscp_dn1: f64,
        var_qscp_dn3: f64,
        var_qscp_dn4: f64,
        var_qscp_dn5: f64,
        var_qscp_dn6: f64,
        var_qscp_dn7: f64,
        var_qscp_dn8: f64,
        var_qscp_dn9: f64,
        var_rth_t: f64,
        var_rth_t_dn4: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq29_e316, eq29_e316_d_n4, eq29_e316_d_n5, eq29_e316_d_n6, eq29_e316_d_n7, eq29_e316_d_n8, eq29_e316_d_n9,) = {
    if ((var_guard239 != 0.0) && (var_guard240 != 0.0)) {
        let eq29_e314: f64 = (p.p148 * var_ijsc);
        let eq29_e314_d_n4: f64 = (p.p148 * var_ijsc_dn4);
        let eq29_e314_d_n5: f64 = (p.p148 * var_ijsc_dn5);
        let eq29_e314_d_n6: f64 = (p.p148 * var_ijsc_dn6);
        let eq29_e314_d_n7: f64 = (p.p148 * var_ijsc_dn7);
        let eq29_e314_d_n8: f64 = (p.p148 * var_ijsc_dn8);
        let eq29_e314_d_n9: f64 = (p.p148 * var_ijsc_dn9);
        (eq29_e314, eq29_e314_d_n4, eq29_e314_d_n5, eq29_e314_d_n6, eq29_e314_d_n7, eq29_e314_d_n8, eq29_e314_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e316;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(5),
            multiplicity * (eq29_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq29_e316_d_n4), multiplicity * (eq29_e316_d_n5), multiplicity * (eq29_e316_d_n6), multiplicity * (eq29_e316_d_n7), multiplicity * (eq29_e316_d_n8), multiplicity * (eq29_e316_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq31_e331, eq31_e331_d_n4, eq31_e331_d_n5, eq31_e331_d_n6, eq31_e331_d_n7, eq31_e331_d_n8, eq31_e331_d_n9,) = {
    if (var_guard239 == 0.0) {
        let eq31_e329: f64 = (p.p148 * var_ijsc);
        let eq31_e329_d_n4: f64 = (p.p148 * var_ijsc_dn4);
        let eq31_e329_d_n5: f64 = (p.p148 * var_ijsc_dn5);
        let eq31_e329_d_n6: f64 = (p.p148 * var_ijsc_dn6);
        let eq31_e329_d_n7: f64 = (p.p148 * var_ijsc_dn7);
        let eq31_e329_d_n8: f64 = (p.p148 * var_ijsc_dn8);
        let eq31_e329_d_n9: f64 = (p.p148 * var_ijsc_dn9);
        (eq31_e329, eq31_e329_d_n4, eq31_e329_d_n5, eq31_e329_d_n6, eq31_e329_d_n7, eq31_e329_d_n8, eq31_e329_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e331;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(5),
            multiplicity * (eq31_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq31_e331_d_n4), multiplicity * (eq31_e331_d_n5), multiplicity * (eq31_e331_d_n6), multiplicity * (eq31_e331_d_n7), multiplicity * (eq31_e331_d_n8), multiplicity * (eq31_e331_d_n9)],
            [],
            [],
            1.0,
        );
        let eq33_e343: f64 = (p.p148 * var_qjs);
        let eq33_e343_d_n0: f64 = (p.p148 * var_qjs_dn0);
        let eq33_e343_d_n1: f64 = (p.p148 * var_qjs_dn1);
        let eq33_e343_d_n3: f64 = (p.p148 * var_qjs_dn3);
        let eq33_e343_d_n4: f64 = (p.p148 * var_qjs_dn4);
        let eq33_e343_d_n5: f64 = (p.p148 * var_qjs_dn5);
        let eq33_e343_d_n6: f64 = (p.p148 * var_qjs_dn6);
        let eq33_e343_d_n7: f64 = (p.p148 * var_qjs_dn7);
        let eq33_e343_d_n8: f64 = (p.p148 * var_qjs_dn8);
        let eq33_e343_d_n9: f64 = (p.p148 * var_qjs_dn9);
        let eq33_e344: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq33_e343);
        let eq33_value: f64 = eq33_e344;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(9),
            Some(5),
            multiplicity * (eq33_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq33_e343_d_n0 * ddt_scale)), multiplicity * ((eq33_e343_d_n1 * ddt_scale)), multiplicity * ((eq33_e343_d_n3 * ddt_scale)), multiplicity * ((eq33_e343_d_n4 * ddt_scale)), multiplicity * ((eq33_e343_d_n5 * ddt_scale)), multiplicity * ((eq33_e343_d_n6 * ddt_scale)), multiplicity * ((eq33_e343_d_n7 * ddt_scale)), multiplicity * ((eq33_e343_d_n8 * ddt_scale)), multiplicity * ((eq33_e343_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq34_e347: f64 = (p.p148 * var_qscp);
        let eq34_e347_d_n0: f64 = (p.p148 * var_qscp_dn0);
        let eq34_e347_d_n1: f64 = (p.p148 * var_qscp_dn1);
        let eq34_e347_d_n3: f64 = (p.p148 * var_qscp_dn3);
        let eq34_e347_d_n4: f64 = (p.p148 * var_qscp_dn4);
        let eq34_e347_d_n5: f64 = (p.p148 * var_qscp_dn5);
        let eq34_e347_d_n6: f64 = (p.p148 * var_qscp_dn6);
        let eq34_e347_d_n7: f64 = (p.p148 * var_qscp_dn7);
        let eq34_e347_d_n8: f64 = (p.p148 * var_qscp_dn8);
        let eq34_e347_d_n9: f64 = (p.p148 * var_qscp_dn9);
        let eq34_e348: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq34_e347);
        let eq34_value: f64 = eq34_e348;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(3),
            Some(0),
            multiplicity * (eq34_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq34_e347_d_n0 * ddt_scale)), multiplicity * ((eq34_e347_d_n1 * ddt_scale)), multiplicity * ((eq34_e347_d_n3 * ddt_scale)), multiplicity * ((eq34_e347_d_n4 * ddt_scale)), multiplicity * ((eq34_e347_d_n5 * ddt_scale)), multiplicity * ((eq34_e347_d_n6 * ddt_scale)), multiplicity * ((eq34_e347_d_n7 * ddt_scale)), multiplicity * ((eq34_e347_d_n8 * ddt_scale)), multiplicity * ((eq34_e347_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let (eq36_e363, eq36_e363_d_n3, eq36_e363_d_n9,) = {
    if ((var_guard242 != 0.0) && (var_guard243 != 0.0)) {
        let eq36_e360: f64 = (p.p103 * (nv9 - nv3));
        let eq36_e361: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq36_e360);
        (eq36_e361, ((-p.p103) * ddt_scale), (p.p103 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e363;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(3),
            multiplicity * (eq36_value),
            3,
            multiplicity * (eq36_e363_d_n3),
            9,
            multiplicity * (eq36_e363_d_n9),
        );
        let (eq38_e376, eq38_e376_d_n0, eq38_e376_d_n1, eq38_e376_d_n2, eq38_e376_d_n3, eq38_e376_d_n4, eq38_e376_d_n5, eq38_e376_d_n6, eq38_e376_d_n7, eq38_e376_d_n8, eq38_e376_d_n9,) = {
    if (var_guard244 != 0.0) {
        let eq38_e372: f64 = ((nv4 - 0.0) / var_rth_t);
        let eq38_e372_d_n4: f64 = ((var_rth_t - ((nv4 - 0.0) * var_rth_t_dn4)) / (var_rth_t * var_rth_t));
        let eq38_e374: f64 = (eq38_e372 - var_pterm);
        let eq38_e374_d_n4: f64 = (eq38_e372_d_n4 - var_pterm_dn4);
        (eq38_e374, (-var_pterm_dn0), (-var_pterm_dn1), (-var_pterm_dn2), (-var_pterm_dn3), eq38_e374_d_n4, (-var_pterm_dn5), (-var_pterm_dn6), (-var_pterm_dn7), (-var_pterm_dn8), (-var_pterm_dn9),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e376;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(4),
            None,
            multiplicity * (eq38_value),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq38_e376_d_n0), multiplicity * (eq38_e376_d_n1), multiplicity * (eq38_e376_d_n2), multiplicity * (eq38_e376_d_n3), multiplicity * (eq38_e376_d_n4), multiplicity * (eq38_e376_d_n5), multiplicity * (eq38_e376_d_n6), multiplicity * (eq38_e376_d_n7), multiplicity * (eq38_e376_d_n8), multiplicity * (eq38_e376_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq39_e385, eq39_e385_d_n4,) = {
    if ((var_guard244 != 0.0) && (var_guard245 != 0.0)) {
        let eq39_e382: f64 = (p.p145 * (nv4 - 0.0));
        let eq39_e383: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq39_e382);
        (eq39_e383, (p.p145 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e385;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq39_value),
            4,
            multiplicity * (eq39_e385_d_n4),
        );
        let eq41_value: f64 = var_ixf1;
        let eq41_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq41_node_derivatives: [f64; 11] = [var_ixf1_dn0, var_ixf1_dn1, var_ixf1_dn3, var_ixf1_dn4, var_ixf1_dn5, var_ixf1_dn6, var_ixf1_dn7, var_ixf1_dn8, var_ixf1_dn9, var_ixf1_dn10, var_ixf1_dn11];
        let eq41_branch_derivative_indices: [usize; 0] = [];
        let eq41_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            None,
            multiplicity * (eq41_value),
            &eq41_node_derivative_indices,
            &eq41_node_derivatives,
            &eq41_branch_derivative_indices,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let eq43_value: f64 = var_ixf2;
        let eq43_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq43_node_derivatives: [f64; 11] = [var_ixf2_dn0, var_ixf2_dn1, var_ixf2_dn3, var_ixf2_dn4, var_ixf2_dn5, var_ixf2_dn6, var_ixf2_dn7, var_ixf2_dn8, var_ixf2_dn9, var_ixf2_dn10, var_ixf2_dn11];
        let eq43_branch_derivative_indices: [usize; 0] = [];
        let eq43_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            None,
            multiplicity * (eq43_value),
            &eq43_node_derivative_indices,
            &eq43_node_derivatives,
            &eq43_branch_derivative_indices,
            &eq43_branch_derivatives,
            multiplicity,
        );
        let eq45_value: f64 = var_ixf;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(12),
            None,
            multiplicity * (eq45_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 12],
            [multiplicity * (var_ixf_dn0), multiplicity * (var_ixf_dn1), multiplicity * (var_ixf_dn3), multiplicity * (var_ixf_dn4), multiplicity * (var_ixf_dn5), multiplicity * (var_ixf_dn6), multiplicity * (var_ixf_dn7), multiplicity * (var_ixf_dn8), multiplicity * (var_ixf_dn9), multiplicity * (var_ixf_dn12)],
            [],
            [],
            1.0,
        );
        let (eq65_e534, eq65_e534_d_n0, eq65_e534_d_n1, eq65_e534_d_n3, eq65_e534_d_n4, eq65_e534_d_n5, eq65_e534_d_n6, eq65_e534_d_n7, eq65_e534_d_n8, eq65_e534_d_n9, eq65_e534_d_n13,) = {
    if (var_guard258 != 0.0) {
        let __rspice_inv_cse_0: f64 = 1.0 / var_n_w;
        let eq65_e527: f64 = (var_n_2 * __rspice_inv_cse_0);
        let eq65_e527_d_n0: f64 = (var_n_2_dn0 * __rspice_inv_cse_0);
        let eq65_e527_d_n1: f64 = (var_n_2_dn1 * __rspice_inv_cse_0);
        let eq65_e527_d_n3: f64 = (var_n_2_dn3 * __rspice_inv_cse_0);
        let eq65_e527_d_n4: f64 = (var_n_2_dn4 * __rspice_inv_cse_0);
        let eq65_e527_d_n5: f64 = (var_n_2_dn5 * __rspice_inv_cse_0);
        let eq65_e527_d_n6: f64 = (var_n_2_dn6 * __rspice_inv_cse_0);
        let eq65_e527_d_n7: f64 = (var_n_2_dn7 * __rspice_inv_cse_0);
        let eq65_e527_d_n8: f64 = (var_n_2_dn8 * __rspice_inv_cse_0);
        let eq65_e527_d_n9: f64 = (var_n_2_dn9 * __rspice_inv_cse_0);
        let eq65_e530: f64 = (var_n_w * (nv13 - 0.0));
        let eq65_e531: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq65_e530);
        let eq65_e532: f64 = (eq65_e527 * eq65_e531);
        let eq65_e532_d_n0: f64 = (eq65_e527_d_n0 * eq65_e531);
        let eq65_e532_d_n1: f64 = (eq65_e527_d_n1 * eq65_e531);
        let eq65_e532_d_n3: f64 = (eq65_e527_d_n3 * eq65_e531);
        let eq65_e532_d_n4: f64 = (eq65_e527_d_n4 * eq65_e531);
        let eq65_e532_d_n5: f64 = (eq65_e527_d_n5 * eq65_e531);
        let eq65_e532_d_n6: f64 = (eq65_e527_d_n6 * eq65_e531);
        let eq65_e532_d_n7: f64 = (eq65_e527_d_n7 * eq65_e531);
        let eq65_e532_d_n8: f64 = (eq65_e527_d_n8 * eq65_e531);
        let eq65_e532_d_n9: f64 = (eq65_e527_d_n9 * eq65_e531);
        let eq65_e532_d_n13: f64 = (eq65_e527 * (var_n_w * ddt_scale));
        (eq65_e532, eq65_e532_d_n0, eq65_e532_d_n1, eq65_e532_d_n3, eq65_e532_d_n4, eq65_e532_d_n5, eq65_e532_d_n6, eq65_e532_d_n7, eq65_e532_d_n8, eq65_e532_d_n9, eq65_e532_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e534;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq65_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 13],
            [multiplicity * (eq65_e534_d_n0), multiplicity * (eq65_e534_d_n1), multiplicity * (eq65_e534_d_n3), multiplicity * (eq65_e534_d_n4), multiplicity * (eq65_e534_d_n5), multiplicity * (eq65_e534_d_n6), multiplicity * (eq65_e534_d_n7), multiplicity * (eq65_e534_d_n8), multiplicity * (eq65_e534_d_n9), multiplicity * (eq65_e534_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq66_e545, eq66_e545_d_n0, eq66_e545_d_n1, eq66_e545_d_n3, eq66_e545_d_n4, eq66_e545_d_n5, eq66_e545_d_n6, eq66_e545_d_n7, eq66_e545_d_n8, eq66_e545_d_n9, eq66_e545_d_n14,) = {
    if (var_guard258 != 0.0) {
        let __rspice_inv_cse_1: f64 = 1.0 / var_n_w;
        let eq66_e538: f64 = (var_n_1 * __rspice_inv_cse_1);
        let eq66_e538_d_n0: f64 = (var_n_1_dn0 * __rspice_inv_cse_1);
        let eq66_e538_d_n1: f64 = (var_n_1_dn1 * __rspice_inv_cse_1);
        let eq66_e538_d_n3: f64 = (var_n_1_dn3 * __rspice_inv_cse_1);
        let eq66_e538_d_n4: f64 = (var_n_1_dn4 * __rspice_inv_cse_1);
        let eq66_e538_d_n5: f64 = (var_n_1_dn5 * __rspice_inv_cse_1);
        let eq66_e538_d_n6: f64 = (var_n_1_dn6 * __rspice_inv_cse_1);
        let eq66_e538_d_n7: f64 = (var_n_1_dn7 * __rspice_inv_cse_1);
        let eq66_e538_d_n8: f64 = (var_n_1_dn8 * __rspice_inv_cse_1);
        let eq66_e538_d_n9: f64 = (var_n_1_dn9 * __rspice_inv_cse_1);
        let eq66_e541: f64 = (var_n_w * (nv14 - 0.0));
        let eq66_e542: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, eq66_e541);
        let eq66_e543: f64 = (eq66_e538 * eq66_e542);
        let eq66_e543_d_n0: f64 = (eq66_e538_d_n0 * eq66_e542);
        let eq66_e543_d_n1: f64 = (eq66_e538_d_n1 * eq66_e542);
        let eq66_e543_d_n3: f64 = (eq66_e538_d_n3 * eq66_e542);
        let eq66_e543_d_n4: f64 = (eq66_e538_d_n4 * eq66_e542);
        let eq66_e543_d_n5: f64 = (eq66_e538_d_n5 * eq66_e542);
        let eq66_e543_d_n6: f64 = (eq66_e538_d_n6 * eq66_e542);
        let eq66_e543_d_n7: f64 = (eq66_e538_d_n7 * eq66_e542);
        let eq66_e543_d_n8: f64 = (eq66_e538_d_n8 * eq66_e542);
        let eq66_e543_d_n9: f64 = (eq66_e538_d_n9 * eq66_e542);
        let eq66_e543_d_n14: f64 = (eq66_e538 * (var_n_w * ddt_scale));
        (eq66_e543, eq66_e543_d_n0, eq66_e543_d_n1, eq66_e543_d_n3, eq66_e543_d_n4, eq66_e543_d_n5, eq66_e543_d_n6, eq66_e543_d_n7, eq66_e543_d_n8, eq66_e543_d_n9, eq66_e543_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e545;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq66_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 14],
            [multiplicity * (eq66_e545_d_n0), multiplicity * (eq66_e545_d_n1), multiplicity * (eq66_e545_d_n3), multiplicity * (eq66_e545_d_n4), multiplicity * (eq66_e545_d_n5), multiplicity * (eq66_e545_d_n6), multiplicity * (eq66_e545_d_n7), multiplicity * (eq66_e545_d_n8), multiplicity * (eq66_e545_d_n9), multiplicity * (eq66_e545_d_n14)],
            [],
            [],
            1.0,
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq1_e170: f64 = (s.v[242] + s.v[179]);
        let eq1_e170_d_n0: f64 = (s.dn[242][0] + s.dn[179][0]);
        let eq1_e170_d_n1: f64 = (s.dn[242][1] + s.dn[179][1]);
        let eq1_e170_d_n2: f64 = (s.dn[242][2] + s.dn[179][2]);
        let eq1_e170_d_n3: f64 = (s.dn[242][3] + s.dn[179][3]);
        let eq1_e170_d_n4: f64 = (s.dn[242][4] + s.dn[179][4]);
        let eq1_e170_d_n5: f64 = (s.dn[242][5] + s.dn[179][5]);
        let eq1_e170_d_n6: f64 = (s.dn[242][6] + s.dn[179][6]);
        let eq1_e170_d_n7: f64 = (s.dn[242][7] + s.dn[179][7]);
        let eq1_e170_d_n8: f64 = (s.dn[242][8] + s.dn[179][8]);
        let eq1_e170_d_n9: f64 = (s.dn[242][9] + s.dn[179][9]);
        let eq1_e170_d_n10: f64 = (s.dn[242][10] + s.dn[179][10]);
        let eq1_e170_d_n11: f64 = (s.dn[242][11] + s.dn[179][11]);
        let eq1_e170_d_n12: f64 = (s.dn[242][12] + s.dn[179][12]);
        let eq1_e170_d_n13: f64 = (s.dn[242][13] + s.dn[179][13]);
        let eq1_e170_d_n14: f64 = (s.dn[242][14] + s.dn[179][14]);
        let eq1_e170_d_b0: f64 = (s.db[242][0] + s.db[179][0]);
        let eq1_e170_d_b1: f64 = (s.db[242][1] + s.db[179][1]);
        let eq1_e170_d_b2: f64 = (s.db[242][2] + s.db[179][2]);
        let eq1_e170_d_b3: f64 = (s.db[242][3] + s.db[179][3]);
        let eq1_e170_d_b4: f64 = (s.db[242][4] + s.db[179][4]);
        let eq1_e170_d_b5: f64 = (s.db[242][5] + s.db[179][5]);
        let eq1_e171: f64 = (p.p148 * eq1_e170);
        let eq1_e171_d_n0: f64 = (p.p148 * eq1_e170_d_n0);
        let eq1_e171_d_n1: f64 = (p.p148 * eq1_e170_d_n1);
        let eq1_e171_d_n2: f64 = (p.p148 * eq1_e170_d_n2);
        let eq1_e171_d_n3: f64 = (p.p148 * eq1_e170_d_n3);
        let eq1_e171_d_n4: f64 = (p.p148 * eq1_e170_d_n4);
        let eq1_e171_d_n5: f64 = (p.p148 * eq1_e170_d_n5);
        let eq1_e171_d_n6: f64 = (p.p148 * eq1_e170_d_n6);
        let eq1_e171_d_n7: f64 = (p.p148 * eq1_e170_d_n7);
        let eq1_e171_d_n8: f64 = (p.p148 * eq1_e170_d_n8);
        let eq1_e171_d_n9: f64 = (p.p148 * eq1_e170_d_n9);
        let eq1_e171_d_n10: f64 = (p.p148 * eq1_e170_d_n10);
        let eq1_e171_d_n11: f64 = (p.p148 * eq1_e170_d_n11);
        let eq1_e171_d_n12: f64 = (p.p148 * eq1_e170_d_n12);
        let eq1_e171_d_n13: f64 = (p.p148 * eq1_e170_d_n13);
        let eq1_e171_d_n14: f64 = (p.p148 * eq1_e170_d_n14);
        let eq1_e171_d_b0: f64 = (p.p148 * eq1_e170_d_b0);
        let eq1_e171_d_b1: f64 = (p.p148 * eq1_e170_d_b1);
        let eq1_e171_d_b2: f64 = (p.p148 * eq1_e170_d_b2);
        let eq1_e171_d_b3: f64 = (p.p148 * eq1_e170_d_b3);
        let eq1_e171_d_b4: f64 = (p.p148 * eq1_e170_d_b4);
        let eq1_e171_d_b5: f64 = (p.p148 * eq1_e170_d_b5);
        let eq1_e172_q: f64 = eq1_e171;
        let eq1_reactive_node_derivatives: [f64; 15] = [eq1_e171_d_n0, eq1_e171_d_n1, eq1_e171_d_n2, eq1_e171_d_n3, eq1_e171_d_n4, eq1_e171_d_n5, eq1_e171_d_n6, eq1_e171_d_n7, eq1_e171_d_n8, eq1_e171_d_n9, eq1_e171_d_n10, eq1_e171_d_n11, eq1_e171_d_n12, eq1_e171_d_n13, eq1_e171_d_n14];
        let eq1_reactive_branch_derivatives: [f64; 6] = [eq1_e171_d_b0, eq1_e171_d_b1, eq1_e171_d_b2, eq1_e171_d_b3, eq1_e171_d_b4, eq1_e171_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq1_reactive_node_derivatives,
            branches,
            &eq1_reactive_branch_derivatives,
            multiplicity,
        );
        let eq3_e185: f64 = (s.v[182] + s.v[178]);
        let eq3_e185_d_n0: f64 = (s.dn[182][0] + s.dn[178][0]);
        let eq3_e185_d_n1: f64 = (s.dn[182][1] + s.dn[178][1]);
        let eq3_e185_d_n2: f64 = (s.dn[182][2] + s.dn[178][2]);
        let eq3_e185_d_n3: f64 = (s.dn[182][3] + s.dn[178][3]);
        let eq3_e185_d_n4: f64 = (s.dn[182][4] + s.dn[178][4]);
        let eq3_e185_d_n5: f64 = (s.dn[182][5] + s.dn[178][5]);
        let eq3_e185_d_n6: f64 = (s.dn[182][6] + s.dn[178][6]);
        let eq3_e185_d_n7: f64 = (s.dn[182][7] + s.dn[178][7]);
        let eq3_e185_d_n8: f64 = (s.dn[182][8] + s.dn[178][8]);
        let eq3_e185_d_n9: f64 = (s.dn[182][9] + s.dn[178][9]);
        let eq3_e185_d_n10: f64 = (s.dn[182][10] + s.dn[178][10]);
        let eq3_e185_d_n11: f64 = (s.dn[182][11] + s.dn[178][11]);
        let eq3_e185_d_n12: f64 = (s.dn[182][12] + s.dn[178][12]);
        let eq3_e185_d_n13: f64 = (s.dn[182][13] + s.dn[178][13]);
        let eq3_e185_d_n14: f64 = (s.dn[182][14] + s.dn[178][14]);
        let eq3_e185_d_b0: f64 = (s.db[182][0] + s.db[178][0]);
        let eq3_e185_d_b1: f64 = (s.db[182][1] + s.db[178][1]);
        let eq3_e185_d_b2: f64 = (s.db[182][2] + s.db[178][2]);
        let eq3_e185_d_b3: f64 = (s.db[182][3] + s.db[178][3]);
        let eq3_e185_d_b4: f64 = (s.db[182][4] + s.db[178][4]);
        let eq3_e185_d_b5: f64 = (s.db[182][5] + s.db[178][5]);
        let eq3_e186: f64 = (p.p148 * eq3_e185);
        let eq3_e186_d_n0: f64 = (p.p148 * eq3_e185_d_n0);
        let eq3_e186_d_n1: f64 = (p.p148 * eq3_e185_d_n1);
        let eq3_e186_d_n2: f64 = (p.p148 * eq3_e185_d_n2);
        let eq3_e186_d_n3: f64 = (p.p148 * eq3_e185_d_n3);
        let eq3_e186_d_n4: f64 = (p.p148 * eq3_e185_d_n4);
        let eq3_e186_d_n5: f64 = (p.p148 * eq3_e185_d_n5);
        let eq3_e186_d_n6: f64 = (p.p148 * eq3_e185_d_n6);
        let eq3_e186_d_n7: f64 = (p.p148 * eq3_e185_d_n7);
        let eq3_e186_d_n8: f64 = (p.p148 * eq3_e185_d_n8);
        let eq3_e186_d_n9: f64 = (p.p148 * eq3_e185_d_n9);
        let eq3_e186_d_n10: f64 = (p.p148 * eq3_e185_d_n10);
        let eq3_e186_d_n11: f64 = (p.p148 * eq3_e185_d_n11);
        let eq3_e186_d_n12: f64 = (p.p148 * eq3_e185_d_n12);
        let eq3_e186_d_n13: f64 = (p.p148 * eq3_e185_d_n13);
        let eq3_e186_d_n14: f64 = (p.p148 * eq3_e185_d_n14);
        let eq3_e186_d_b0: f64 = (p.p148 * eq3_e185_d_b0);
        let eq3_e186_d_b1: f64 = (p.p148 * eq3_e185_d_b1);
        let eq3_e186_d_b2: f64 = (p.p148 * eq3_e185_d_b2);
        let eq3_e186_d_b3: f64 = (p.p148 * eq3_e185_d_b3);
        let eq3_e186_d_b4: f64 = (p.p148 * eq3_e185_d_b4);
        let eq3_e186_d_b5: f64 = (p.p148 * eq3_e185_d_b5);
        let eq3_e187_q: f64 = eq3_e186;
        let eq3_reactive_node_derivatives: [f64; 15] = [eq3_e186_d_n0, eq3_e186_d_n1, eq3_e186_d_n2, eq3_e186_d_n3, eq3_e186_d_n4, eq3_e186_d_n5, eq3_e186_d_n6, eq3_e186_d_n7, eq3_e186_d_n8, eq3_e186_d_n9, eq3_e186_d_n10, eq3_e186_d_n11, eq3_e186_d_n12, eq3_e186_d_n13, eq3_e186_d_n14];
        let eq3_reactive_branch_derivatives: [f64; 6] = [eq3_e186_d_b0, eq3_e186_d_b1, eq3_e186_d_b2, eq3_e186_d_b3, eq3_e186_d_b4, eq3_e186_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq3_reactive_node_derivatives,
            branches,
            &eq3_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq7_e206, eq7_e206_d_n0, eq7_e206_d_n1, eq7_e206_d_n2, eq7_e206_d_n3, eq7_e206_d_n4, eq7_e206_d_n5, eq7_e206_d_n6, eq7_e206_d_n7, eq7_e206_d_n8, eq7_e206_d_n9, eq7_e206_d_n10, eq7_e206_d_n11, eq7_e206_d_n12, eq7_e206_d_n13, eq7_e206_d_n14, eq7_e206_d_b0, eq7_e206_d_b1, eq7_e206_d_b2, eq7_e206_d_b3, eq7_e206_d_b4, eq7_e206_d_b5, eq7_e206_q,) = {
    if (s.b[508] && s.b[509]) {
        let eq7_e204_q: f64 = s.v[183];
        (s.v[183], s.dn[183][0], s.dn[183][1], s.dn[183][2], s.dn[183][3], s.dn[183][4], s.dn[183][5], s.dn[183][6], s.dn[183][7], s.dn[183][8], s.dn[183][9], s.dn[183][10], s.dn[183][11], s.dn[183][12], s.dn[183][13], s.dn[183][14], s.db[183][0], s.db[183][1], s.db[183][2], s.db[183][3], s.db[183][4], s.db[183][5], eq7_e204_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 15] = [eq7_e206_d_n0, eq7_e206_d_n1, eq7_e206_d_n2, eq7_e206_d_n3, eq7_e206_d_n4, eq7_e206_d_n5, eq7_e206_d_n6, eq7_e206_d_n7, eq7_e206_d_n8, eq7_e206_d_n9, eq7_e206_d_n10, eq7_e206_d_n11, eq7_e206_d_n12, eq7_e206_d_n13, eq7_e206_d_n14];
        let eq7_reactive_branch_derivatives: [f64; 6] = [eq7_e206_d_b0, eq7_e206_d_b1, eq7_e206_d_b2, eq7_e206_d_b3, eq7_e206_d_b4, eq7_e206_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            nodes,
            &eq7_reactive_node_derivatives,
            branches,
            &eq7_reactive_branch_derivatives,
            multiplicity,
        );
        let eq13_e238: f64 = (p.p148 * s.v[180]);
        let eq13_e239_q: f64 = eq13_e238;
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            nodes,
            &s.dn[180],
            branches,
            &s.db[180],
            (multiplicity) * (p.p148),
        );
        let eq15_e246: f64 = (s.v[42] + s.v[199]);
        let eq15_e246_d_n0: f64 = (s.dn[42][0] + s.dn[199][0]);
        let eq15_e246_d_n1: f64 = (s.dn[42][1] + s.dn[199][1]);
        let eq15_e246_d_n2: f64 = (s.dn[42][2] + s.dn[199][2]);
        let eq15_e246_d_n3: f64 = (s.dn[42][3] + s.dn[199][3]);
        let eq15_e246_d_n4: f64 = (s.dn[42][4] + s.dn[199][4]);
        let eq15_e246_d_n5: f64 = (s.dn[42][5] + s.dn[199][5]);
        let eq15_e246_d_n6: f64 = (s.dn[42][6] + s.dn[199][6]);
        let eq15_e246_d_n7: f64 = (s.dn[42][7] + s.dn[199][7]);
        let eq15_e246_d_n8: f64 = (s.dn[42][8] + s.dn[199][8]);
        let eq15_e246_d_n9: f64 = (s.dn[42][9] + s.dn[199][9]);
        let eq15_e246_d_n10: f64 = (s.dn[42][10] + s.dn[199][10]);
        let eq15_e246_d_n11: f64 = (s.dn[42][11] + s.dn[199][11]);
        let eq15_e246_d_n12: f64 = (s.dn[42][12] + s.dn[199][12]);
        let eq15_e246_d_n13: f64 = (s.dn[42][13] + s.dn[199][13]);
        let eq15_e246_d_n14: f64 = (s.dn[42][14] + s.dn[199][14]);
        let eq15_e246_d_b0: f64 = (s.db[42][0] + s.db[199][0]);
        let eq15_e246_d_b1: f64 = (s.db[42][1] + s.db[199][1]);
        let eq15_e246_d_b2: f64 = (s.db[42][2] + s.db[199][2]);
        let eq15_e246_d_b3: f64 = (s.db[42][3] + s.db[199][3]);
        let eq15_e246_d_b4: f64 = (s.db[42][4] + s.db[199][4]);
        let eq15_e246_d_b5: f64 = (s.db[42][5] + s.db[199][5]);
        let eq15_e247: f64 = (p.p148 * eq15_e246);
        let eq15_e247_d_n0: f64 = (p.p148 * eq15_e246_d_n0);
        let eq15_e247_d_n1: f64 = (p.p148 * eq15_e246_d_n1);
        let eq15_e247_d_n2: f64 = (p.p148 * eq15_e246_d_n2);
        let eq15_e247_d_n3: f64 = (p.p148 * eq15_e246_d_n3);
        let eq15_e247_d_n4: f64 = (p.p148 * eq15_e246_d_n4);
        let eq15_e247_d_n5: f64 = (p.p148 * eq15_e246_d_n5);
        let eq15_e247_d_n6: f64 = (p.p148 * eq15_e246_d_n6);
        let eq15_e247_d_n7: f64 = (p.p148 * eq15_e246_d_n7);
        let eq15_e247_d_n8: f64 = (p.p148 * eq15_e246_d_n8);
        let eq15_e247_d_n9: f64 = (p.p148 * eq15_e246_d_n9);
        let eq15_e247_d_n10: f64 = (p.p148 * eq15_e246_d_n10);
        let eq15_e247_d_n11: f64 = (p.p148 * eq15_e246_d_n11);
        let eq15_e247_d_n12: f64 = (p.p148 * eq15_e246_d_n12);
        let eq15_e247_d_n13: f64 = (p.p148 * eq15_e246_d_n13);
        let eq15_e247_d_n14: f64 = (p.p148 * eq15_e246_d_n14);
        let eq15_e247_d_b0: f64 = (p.p148 * eq15_e246_d_b0);
        let eq15_e247_d_b1: f64 = (p.p148 * eq15_e246_d_b1);
        let eq15_e247_d_b2: f64 = (p.p148 * eq15_e246_d_b2);
        let eq15_e247_d_b3: f64 = (p.p148 * eq15_e246_d_b3);
        let eq15_e247_d_b4: f64 = (p.p148 * eq15_e246_d_b4);
        let eq15_e247_d_b5: f64 = (p.p148 * eq15_e246_d_b5);
        let eq15_e248_q: f64 = eq15_e247;
        let eq15_reactive_node_derivatives: [f64; 15] = [eq15_e247_d_n0, eq15_e247_d_n1, eq15_e247_d_n2, eq15_e247_d_n3, eq15_e247_d_n4, eq15_e247_d_n5, eq15_e247_d_n6, eq15_e247_d_n7, eq15_e247_d_n8, eq15_e247_d_n9, eq15_e247_d_n10, eq15_e247_d_n11, eq15_e247_d_n12, eq15_e247_d_n13, eq15_e247_d_n14];
        let eq15_reactive_branch_derivatives: [f64; 6] = [eq15_e247_d_b0, eq15_e247_d_b1, eq15_e247_d_b2, eq15_e247_d_b3, eq15_e247_d_b4, eq15_e247_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq17_e255: f64 = (p.p148 * s.v[41]);
        let eq17_e256_q: f64 = eq17_e255;
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            nodes,
            &s.dn[41],
            branches,
            &s.db[41],
            (multiplicity) * (p.p148),
        );
        let eq33_e343: f64 = (p.p148 * s.v[196]);
        let eq33_e344_q: f64 = eq33_e343;
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[5]),
            nodes,
            &s.dn[196],
            branches,
            &s.db[196],
            (multiplicity) * (p.p148),
        );
        let eq34_e347: f64 = (p.p148 * s.v[197]);
        let eq34_e348_q: f64 = eq34_e347;
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[0]),
            nodes,
            &s.dn[197],
            branches,
            &s.db[197],
            (multiplicity) * (p.p148),
        );
        let (eq36_e363, eq36_e363_d_n3, eq36_e363_d_n9, eq36_e363_q,) = {
    if (s.b[517] && s.b[518]) {
        let eq36_e360: f64 = (p.p103 * (nv9 - nv3));
        let eq36_e361_q: f64 = eq36_e360;
        (eq36_e360, (-p.p103), p.p103, eq36_e361_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2(
            Some(nodes[9]),
            Some(nodes[3]),
            nodes[3],
            multiplicity * (eq36_e363_d_n3),
            nodes[9],
            multiplicity * (eq36_e363_d_n9),
        );
        let (eq39_e385, eq39_e385_d_n4, eq39_e385_q,) = {
    if (s.b[519] && s.b[520]) {
        let eq39_e382: f64 = (p.p145 * (nv4 - 0.0));
        let eq39_e383_q: f64 = eq39_e382;
        (eq39_e382, p.p145, eq39_e383_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (eq39_e385_d_n4),
        );
        let (eq65_e534, eq65_e534_d_n0, eq65_e534_d_n1, eq65_e534_d_n2, eq65_e534_d_n3, eq65_e534_d_n4, eq65_e534_d_n5, eq65_e534_d_n6, eq65_e534_d_n7, eq65_e534_d_n8, eq65_e534_d_n9, eq65_e534_d_n10, eq65_e534_d_n11, eq65_e534_d_n12, eq65_e534_d_n13, eq65_e534_d_n14, eq65_e534_d_b0, eq65_e534_d_b1, eq65_e534_d_b2, eq65_e534_d_b3, eq65_e534_d_b4, eq65_e534_d_b5, eq65_e534_q, eq65_e534_q_d_n0, eq65_e534_q_d_n1, eq65_e534_q_d_n2, eq65_e534_q_d_n3, eq65_e534_q_d_n4, eq65_e534_q_d_n5, eq65_e534_q_d_n6, eq65_e534_q_d_n7, eq65_e534_q_d_n8, eq65_e534_q_d_n9, eq65_e534_q_d_n10, eq65_e534_q_d_n11, eq65_e534_q_d_n12, eq65_e534_q_d_n13, eq65_e534_q_d_n14, eq65_e534_q_d_b0, eq65_e534_q_d_b1, eq65_e534_q_d_b2, eq65_e534_q_d_b3, eq65_e534_q_d_b4, eq65_e534_q_d_b5,) = {
    if s.b[533] {
        let eq65_e527: f64 = (s.v[537] / s.v[535]);
        let __rspice_inv_cse_0: f64 = 1.0 / (s.v[535] * s.v[535]);
        let eq65_e527_d_n0: f64 = (((s.dn[537][0] * s.v[535]) - (s.v[537] * s.dn[535][0])) * __rspice_inv_cse_0);
        let eq65_e527_d_n1: f64 = (((s.dn[537][1] * s.v[535]) - (s.v[537] * s.dn[535][1])) * __rspice_inv_cse_0);
        let eq65_e527_d_n2: f64 = (((s.dn[537][2] * s.v[535]) - (s.v[537] * s.dn[535][2])) * __rspice_inv_cse_0);
        let eq65_e527_d_n3: f64 = (((s.dn[537][3] * s.v[535]) - (s.v[537] * s.dn[535][3])) * __rspice_inv_cse_0);
        let eq65_e527_d_n4: f64 = (((s.dn[537][4] * s.v[535]) - (s.v[537] * s.dn[535][4])) * __rspice_inv_cse_0);
        let eq65_e527_d_n5: f64 = (((s.dn[537][5] * s.v[535]) - (s.v[537] * s.dn[535][5])) * __rspice_inv_cse_0);
        let eq65_e527_d_n6: f64 = (((s.dn[537][6] * s.v[535]) - (s.v[537] * s.dn[535][6])) * __rspice_inv_cse_0);
        let eq65_e527_d_n7: f64 = (((s.dn[537][7] * s.v[535]) - (s.v[537] * s.dn[535][7])) * __rspice_inv_cse_0);
        let eq65_e527_d_n8: f64 = (((s.dn[537][8] * s.v[535]) - (s.v[537] * s.dn[535][8])) * __rspice_inv_cse_0);
        let eq65_e527_d_n9: f64 = (((s.dn[537][9] * s.v[535]) - (s.v[537] * s.dn[535][9])) * __rspice_inv_cse_0);
        let eq65_e527_d_n10: f64 = (((s.dn[537][10] * s.v[535]) - (s.v[537] * s.dn[535][10])) * __rspice_inv_cse_0);
        let eq65_e527_d_n11: f64 = (((s.dn[537][11] * s.v[535]) - (s.v[537] * s.dn[535][11])) * __rspice_inv_cse_0);
        let eq65_e527_d_n12: f64 = (((s.dn[537][12] * s.v[535]) - (s.v[537] * s.dn[535][12])) * __rspice_inv_cse_0);
        let eq65_e527_d_n13: f64 = (((s.dn[537][13] * s.v[535]) - (s.v[537] * s.dn[535][13])) * __rspice_inv_cse_0);
        let eq65_e527_d_n14: f64 = (((s.dn[537][14] * s.v[535]) - (s.v[537] * s.dn[535][14])) * __rspice_inv_cse_0);
        let eq65_e527_d_b0: f64 = (((s.db[537][0] * s.v[535]) - (s.v[537] * s.db[535][0])) * __rspice_inv_cse_0);
        let eq65_e527_d_b1: f64 = (((s.db[537][1] * s.v[535]) - (s.v[537] * s.db[535][1])) * __rspice_inv_cse_0);
        let eq65_e527_d_b2: f64 = (((s.db[537][2] * s.v[535]) - (s.v[537] * s.db[535][2])) * __rspice_inv_cse_0);
        let eq65_e527_d_b3: f64 = (((s.db[537][3] * s.v[535]) - (s.v[537] * s.db[535][3])) * __rspice_inv_cse_0);
        let eq65_e527_d_b4: f64 = (((s.db[537][4] * s.v[535]) - (s.v[537] * s.db[535][4])) * __rspice_inv_cse_0);
        let eq65_e527_d_b5: f64 = (((s.db[537][5] * s.v[535]) - (s.v[537] * s.db[535][5])) * __rspice_inv_cse_0);
        let eq65_e530: f64 = (s.v[535] * (nv13 - 0.0));
        let eq65_e530_d_n0: f64 = (s.dn[535][0] * (nv13 - 0.0));
        let eq65_e530_d_n1: f64 = (s.dn[535][1] * (nv13 - 0.0));
        let eq65_e530_d_n2: f64 = (s.dn[535][2] * (nv13 - 0.0));
        let eq65_e530_d_n3: f64 = (s.dn[535][3] * (nv13 - 0.0));
        let eq65_e530_d_n4: f64 = (s.dn[535][4] * (nv13 - 0.0));
        let eq65_e530_d_n5: f64 = (s.dn[535][5] * (nv13 - 0.0));
        let eq65_e530_d_n6: f64 = (s.dn[535][6] * (nv13 - 0.0));
        let eq65_e530_d_n7: f64 = (s.dn[535][7] * (nv13 - 0.0));
        let eq65_e530_d_n8: f64 = (s.dn[535][8] * (nv13 - 0.0));
        let eq65_e530_d_n9: f64 = (s.dn[535][9] * (nv13 - 0.0));
        let eq65_e530_d_n10: f64 = (s.dn[535][10] * (nv13 - 0.0));
        let eq65_e530_d_n11: f64 = (s.dn[535][11] * (nv13 - 0.0));
        let eq65_e530_d_n12: f64 = (s.dn[535][12] * (nv13 - 0.0));
        let eq65_e530_d_n13: f64 = ((s.dn[535][13] * (nv13 - 0.0)) + s.v[535]);
        let eq65_e530_d_n14: f64 = (s.dn[535][14] * (nv13 - 0.0));
        let eq65_e530_d_b0: f64 = (s.db[535][0] * (nv13 - 0.0));
        let eq65_e530_d_b1: f64 = (s.db[535][1] * (nv13 - 0.0));
        let eq65_e530_d_b2: f64 = (s.db[535][2] * (nv13 - 0.0));
        let eq65_e530_d_b3: f64 = (s.db[535][3] * (nv13 - 0.0));
        let eq65_e530_d_b4: f64 = (s.db[535][4] * (nv13 - 0.0));
        let eq65_e530_d_b5: f64 = (s.db[535][5] * (nv13 - 0.0));
        let eq65_e531_q: f64 = eq65_e530;
        let eq65_e532: f64 = (eq65_e527 * eq65_e530);
        let eq65_e532_d_n0: f64 = ((eq65_e527_d_n0 * eq65_e530) + (eq65_e527 * eq65_e530_d_n0));
        let eq65_e532_d_n1: f64 = ((eq65_e527_d_n1 * eq65_e530) + (eq65_e527 * eq65_e530_d_n1));
        let eq65_e532_d_n2: f64 = ((eq65_e527_d_n2 * eq65_e530) + (eq65_e527 * eq65_e530_d_n2));
        let eq65_e532_d_n3: f64 = ((eq65_e527_d_n3 * eq65_e530) + (eq65_e527 * eq65_e530_d_n3));
        let eq65_e532_d_n4: f64 = ((eq65_e527_d_n4 * eq65_e530) + (eq65_e527 * eq65_e530_d_n4));
        let eq65_e532_d_n5: f64 = ((eq65_e527_d_n5 * eq65_e530) + (eq65_e527 * eq65_e530_d_n5));
        let eq65_e532_d_n6: f64 = ((eq65_e527_d_n6 * eq65_e530) + (eq65_e527 * eq65_e530_d_n6));
        let eq65_e532_d_n7: f64 = ((eq65_e527_d_n7 * eq65_e530) + (eq65_e527 * eq65_e530_d_n7));
        let eq65_e532_d_n8: f64 = ((eq65_e527_d_n8 * eq65_e530) + (eq65_e527 * eq65_e530_d_n8));
        let eq65_e532_d_n9: f64 = ((eq65_e527_d_n9 * eq65_e530) + (eq65_e527 * eq65_e530_d_n9));
        let eq65_e532_d_n10: f64 = ((eq65_e527_d_n10 * eq65_e530) + (eq65_e527 * eq65_e530_d_n10));
        let eq65_e532_d_n11: f64 = ((eq65_e527_d_n11 * eq65_e530) + (eq65_e527 * eq65_e530_d_n11));
        let eq65_e532_d_n12: f64 = ((eq65_e527_d_n12 * eq65_e530) + (eq65_e527 * eq65_e530_d_n12));
        let eq65_e532_d_n13: f64 = ((eq65_e527_d_n13 * eq65_e530) + (eq65_e527 * eq65_e530_d_n13));
        let eq65_e532_d_n14: f64 = ((eq65_e527_d_n14 * eq65_e530) + (eq65_e527 * eq65_e530_d_n14));
        let eq65_e532_d_b0: f64 = ((eq65_e527_d_b0 * eq65_e530) + (eq65_e527 * eq65_e530_d_b0));
        let eq65_e532_d_b1: f64 = ((eq65_e527_d_b1 * eq65_e530) + (eq65_e527 * eq65_e530_d_b1));
        let eq65_e532_d_b2: f64 = ((eq65_e527_d_b2 * eq65_e530) + (eq65_e527 * eq65_e530_d_b2));
        let eq65_e532_d_b3: f64 = ((eq65_e527_d_b3 * eq65_e530) + (eq65_e527 * eq65_e530_d_b3));
        let eq65_e532_d_b4: f64 = ((eq65_e527_d_b4 * eq65_e530) + (eq65_e527 * eq65_e530_d_b4));
        let eq65_e532_d_b5: f64 = ((eq65_e527_d_b5 * eq65_e530) + (eq65_e527 * eq65_e530_d_b5));
        let eq65_e532_q: f64 = (eq65_e527 * eq65_e531_q);
        let eq65_e532_q_d_n0: f64 = ((eq65_e527_d_n0 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n0));
        let eq65_e532_q_d_n1: f64 = ((eq65_e527_d_n1 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n1));
        let eq65_e532_q_d_n2: f64 = ((eq65_e527_d_n2 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n2));
        let eq65_e532_q_d_n3: f64 = ((eq65_e527_d_n3 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n3));
        let eq65_e532_q_d_n4: f64 = ((eq65_e527_d_n4 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n4));
        let eq65_e532_q_d_n5: f64 = ((eq65_e527_d_n5 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n5));
        let eq65_e532_q_d_n6: f64 = ((eq65_e527_d_n6 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n6));
        let eq65_e532_q_d_n7: f64 = ((eq65_e527_d_n7 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n7));
        let eq65_e532_q_d_n8: f64 = ((eq65_e527_d_n8 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n8));
        let eq65_e532_q_d_n9: f64 = ((eq65_e527_d_n9 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n9));
        let eq65_e532_q_d_n10: f64 = ((eq65_e527_d_n10 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n10));
        let eq65_e532_q_d_n11: f64 = ((eq65_e527_d_n11 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n11));
        let eq65_e532_q_d_n12: f64 = ((eq65_e527_d_n12 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n12));
        let eq65_e532_q_d_n13: f64 = ((eq65_e527_d_n13 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n13));
        let eq65_e532_q_d_n14: f64 = ((eq65_e527_d_n14 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n14));
        let eq65_e532_q_d_b0: f64 = ((eq65_e527_d_b0 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_b0));
        let eq65_e532_q_d_b1: f64 = ((eq65_e527_d_b1 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_b1));
        let eq65_e532_q_d_b2: f64 = ((eq65_e527_d_b2 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_b2));
        let eq65_e532_q_d_b3: f64 = ((eq65_e527_d_b3 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_b3));
        let eq65_e532_q_d_b4: f64 = ((eq65_e527_d_b4 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_b4));
        let eq65_e532_q_d_b5: f64 = ((eq65_e527_d_b5 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_b5));
        (eq65_e532, eq65_e532_d_n0, eq65_e532_d_n1, eq65_e532_d_n2, eq65_e532_d_n3, eq65_e532_d_n4, eq65_e532_d_n5, eq65_e532_d_n6, eq65_e532_d_n7, eq65_e532_d_n8, eq65_e532_d_n9, eq65_e532_d_n10, eq65_e532_d_n11, eq65_e532_d_n12, eq65_e532_d_n13, eq65_e532_d_n14, eq65_e532_d_b0, eq65_e532_d_b1, eq65_e532_d_b2, eq65_e532_d_b3, eq65_e532_d_b4, eq65_e532_d_b5, eq65_e532_q, eq65_e532_q_d_n0, eq65_e532_q_d_n1, eq65_e532_q_d_n2, eq65_e532_q_d_n3, eq65_e532_q_d_n4, eq65_e532_q_d_n5, eq65_e532_q_d_n6, eq65_e532_q_d_n7, eq65_e532_q_d_n8, eq65_e532_q_d_n9, eq65_e532_q_d_n10, eq65_e532_q_d_n11, eq65_e532_q_d_n12, eq65_e532_q_d_n13, eq65_e532_q_d_n14, eq65_e532_q_d_b0, eq65_e532_q_d_b1, eq65_e532_q_d_b2, eq65_e532_q_d_b3, eq65_e532_q_d_b4, eq65_e532_q_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_reactive_node_derivatives: [f64; 15] = [eq65_e534_q_d_n0, eq65_e534_q_d_n1, eq65_e534_q_d_n2, eq65_e534_q_d_n3, eq65_e534_q_d_n4, eq65_e534_q_d_n5, eq65_e534_q_d_n6, eq65_e534_q_d_n7, eq65_e534_q_d_n8, eq65_e534_q_d_n9, eq65_e534_q_d_n10, eq65_e534_q_d_n11, eq65_e534_q_d_n12, eq65_e534_q_d_n13, eq65_e534_q_d_n14];
        let eq65_reactive_branch_derivatives: [f64; 6] = [eq65_e534_q_d_b0, eq65_e534_q_d_b1, eq65_e534_q_d_b2, eq65_e534_q_d_b3, eq65_e534_q_d_b4, eq65_e534_q_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq65_reactive_node_derivatives,
            branches,
            &eq65_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq66_e545, eq66_e545_d_n0, eq66_e545_d_n1, eq66_e545_d_n2, eq66_e545_d_n3, eq66_e545_d_n4, eq66_e545_d_n5, eq66_e545_d_n6, eq66_e545_d_n7, eq66_e545_d_n8, eq66_e545_d_n9, eq66_e545_d_n10, eq66_e545_d_n11, eq66_e545_d_n12, eq66_e545_d_n13, eq66_e545_d_n14, eq66_e545_d_b0, eq66_e545_d_b1, eq66_e545_d_b2, eq66_e545_d_b3, eq66_e545_d_b4, eq66_e545_d_b5, eq66_e545_q, eq66_e545_q_d_n0, eq66_e545_q_d_n1, eq66_e545_q_d_n2, eq66_e545_q_d_n3, eq66_e545_q_d_n4, eq66_e545_q_d_n5, eq66_e545_q_d_n6, eq66_e545_q_d_n7, eq66_e545_q_d_n8, eq66_e545_q_d_n9, eq66_e545_q_d_n10, eq66_e545_q_d_n11, eq66_e545_q_d_n12, eq66_e545_q_d_n13, eq66_e545_q_d_n14, eq66_e545_q_d_b0, eq66_e545_q_d_b1, eq66_e545_q_d_b2, eq66_e545_q_d_b3, eq66_e545_q_d_b4, eq66_e545_q_d_b5,) = {
    if s.b[533] {
        let eq66_e538: f64 = (s.v[536] / s.v[535]);
        let __rspice_inv_cse_1: f64 = 1.0 / (s.v[535] * s.v[535]);
        let eq66_e538_d_n0: f64 = (((s.dn[536][0] * s.v[535]) - (s.v[536] * s.dn[535][0])) * __rspice_inv_cse_1);
        let eq66_e538_d_n1: f64 = (((s.dn[536][1] * s.v[535]) - (s.v[536] * s.dn[535][1])) * __rspice_inv_cse_1);
        let eq66_e538_d_n2: f64 = (((s.dn[536][2] * s.v[535]) - (s.v[536] * s.dn[535][2])) * __rspice_inv_cse_1);
        let eq66_e538_d_n3: f64 = (((s.dn[536][3] * s.v[535]) - (s.v[536] * s.dn[535][3])) * __rspice_inv_cse_1);
        let eq66_e538_d_n4: f64 = (((s.dn[536][4] * s.v[535]) - (s.v[536] * s.dn[535][4])) * __rspice_inv_cse_1);
        let eq66_e538_d_n5: f64 = (((s.dn[536][5] * s.v[535]) - (s.v[536] * s.dn[535][5])) * __rspice_inv_cse_1);
        let eq66_e538_d_n6: f64 = (((s.dn[536][6] * s.v[535]) - (s.v[536] * s.dn[535][6])) * __rspice_inv_cse_1);
        let eq66_e538_d_n7: f64 = (((s.dn[536][7] * s.v[535]) - (s.v[536] * s.dn[535][7])) * __rspice_inv_cse_1);
        let eq66_e538_d_n8: f64 = (((s.dn[536][8] * s.v[535]) - (s.v[536] * s.dn[535][8])) * __rspice_inv_cse_1);
        let eq66_e538_d_n9: f64 = (((s.dn[536][9] * s.v[535]) - (s.v[536] * s.dn[535][9])) * __rspice_inv_cse_1);
        let eq66_e538_d_n10: f64 = (((s.dn[536][10] * s.v[535]) - (s.v[536] * s.dn[535][10])) * __rspice_inv_cse_1);
        let eq66_e538_d_n11: f64 = (((s.dn[536][11] * s.v[535]) - (s.v[536] * s.dn[535][11])) * __rspice_inv_cse_1);
        let eq66_e538_d_n12: f64 = (((s.dn[536][12] * s.v[535]) - (s.v[536] * s.dn[535][12])) * __rspice_inv_cse_1);
        let eq66_e538_d_n13: f64 = (((s.dn[536][13] * s.v[535]) - (s.v[536] * s.dn[535][13])) * __rspice_inv_cse_1);
        let eq66_e538_d_n14: f64 = (((s.dn[536][14] * s.v[535]) - (s.v[536] * s.dn[535][14])) * __rspice_inv_cse_1);
        let eq66_e538_d_b0: f64 = (((s.db[536][0] * s.v[535]) - (s.v[536] * s.db[535][0])) * __rspice_inv_cse_1);
        let eq66_e538_d_b1: f64 = (((s.db[536][1] * s.v[535]) - (s.v[536] * s.db[535][1])) * __rspice_inv_cse_1);
        let eq66_e538_d_b2: f64 = (((s.db[536][2] * s.v[535]) - (s.v[536] * s.db[535][2])) * __rspice_inv_cse_1);
        let eq66_e538_d_b3: f64 = (((s.db[536][3] * s.v[535]) - (s.v[536] * s.db[535][3])) * __rspice_inv_cse_1);
        let eq66_e538_d_b4: f64 = (((s.db[536][4] * s.v[535]) - (s.v[536] * s.db[535][4])) * __rspice_inv_cse_1);
        let eq66_e538_d_b5: f64 = (((s.db[536][5] * s.v[535]) - (s.v[536] * s.db[535][5])) * __rspice_inv_cse_1);
        let eq66_e541: f64 = (s.v[535] * (nv14 - 0.0));
        let eq66_e541_d_n0: f64 = (s.dn[535][0] * (nv14 - 0.0));
        let eq66_e541_d_n1: f64 = (s.dn[535][1] * (nv14 - 0.0));
        let eq66_e541_d_n2: f64 = (s.dn[535][2] * (nv14 - 0.0));
        let eq66_e541_d_n3: f64 = (s.dn[535][3] * (nv14 - 0.0));
        let eq66_e541_d_n4: f64 = (s.dn[535][4] * (nv14 - 0.0));
        let eq66_e541_d_n5: f64 = (s.dn[535][5] * (nv14 - 0.0));
        let eq66_e541_d_n6: f64 = (s.dn[535][6] * (nv14 - 0.0));
        let eq66_e541_d_n7: f64 = (s.dn[535][7] * (nv14 - 0.0));
        let eq66_e541_d_n8: f64 = (s.dn[535][8] * (nv14 - 0.0));
        let eq66_e541_d_n9: f64 = (s.dn[535][9] * (nv14 - 0.0));
        let eq66_e541_d_n10: f64 = (s.dn[535][10] * (nv14 - 0.0));
        let eq66_e541_d_n11: f64 = (s.dn[535][11] * (nv14 - 0.0));
        let eq66_e541_d_n12: f64 = (s.dn[535][12] * (nv14 - 0.0));
        let eq66_e541_d_n13: f64 = (s.dn[535][13] * (nv14 - 0.0));
        let eq66_e541_d_n14: f64 = ((s.dn[535][14] * (nv14 - 0.0)) + s.v[535]);
        let eq66_e541_d_b0: f64 = (s.db[535][0] * (nv14 - 0.0));
        let eq66_e541_d_b1: f64 = (s.db[535][1] * (nv14 - 0.0));
        let eq66_e541_d_b2: f64 = (s.db[535][2] * (nv14 - 0.0));
        let eq66_e541_d_b3: f64 = (s.db[535][3] * (nv14 - 0.0));
        let eq66_e541_d_b4: f64 = (s.db[535][4] * (nv14 - 0.0));
        let eq66_e541_d_b5: f64 = (s.db[535][5] * (nv14 - 0.0));
        let eq66_e542_q: f64 = eq66_e541;
        let eq66_e543: f64 = (eq66_e538 * eq66_e541);
        let eq66_e543_d_n0: f64 = ((eq66_e538_d_n0 * eq66_e541) + (eq66_e538 * eq66_e541_d_n0));
        let eq66_e543_d_n1: f64 = ((eq66_e538_d_n1 * eq66_e541) + (eq66_e538 * eq66_e541_d_n1));
        let eq66_e543_d_n2: f64 = ((eq66_e538_d_n2 * eq66_e541) + (eq66_e538 * eq66_e541_d_n2));
        let eq66_e543_d_n3: f64 = ((eq66_e538_d_n3 * eq66_e541) + (eq66_e538 * eq66_e541_d_n3));
        let eq66_e543_d_n4: f64 = ((eq66_e538_d_n4 * eq66_e541) + (eq66_e538 * eq66_e541_d_n4));
        let eq66_e543_d_n5: f64 = ((eq66_e538_d_n5 * eq66_e541) + (eq66_e538 * eq66_e541_d_n5));
        let eq66_e543_d_n6: f64 = ((eq66_e538_d_n6 * eq66_e541) + (eq66_e538 * eq66_e541_d_n6));
        let eq66_e543_d_n7: f64 = ((eq66_e538_d_n7 * eq66_e541) + (eq66_e538 * eq66_e541_d_n7));
        let eq66_e543_d_n8: f64 = ((eq66_e538_d_n8 * eq66_e541) + (eq66_e538 * eq66_e541_d_n8));
        let eq66_e543_d_n9: f64 = ((eq66_e538_d_n9 * eq66_e541) + (eq66_e538 * eq66_e541_d_n9));
        let eq66_e543_d_n10: f64 = ((eq66_e538_d_n10 * eq66_e541) + (eq66_e538 * eq66_e541_d_n10));
        let eq66_e543_d_n11: f64 = ((eq66_e538_d_n11 * eq66_e541) + (eq66_e538 * eq66_e541_d_n11));
        let eq66_e543_d_n12: f64 = ((eq66_e538_d_n12 * eq66_e541) + (eq66_e538 * eq66_e541_d_n12));
        let eq66_e543_d_n13: f64 = ((eq66_e538_d_n13 * eq66_e541) + (eq66_e538 * eq66_e541_d_n13));
        let eq66_e543_d_n14: f64 = ((eq66_e538_d_n14 * eq66_e541) + (eq66_e538 * eq66_e541_d_n14));
        let eq66_e543_d_b0: f64 = ((eq66_e538_d_b0 * eq66_e541) + (eq66_e538 * eq66_e541_d_b0));
        let eq66_e543_d_b1: f64 = ((eq66_e538_d_b1 * eq66_e541) + (eq66_e538 * eq66_e541_d_b1));
        let eq66_e543_d_b2: f64 = ((eq66_e538_d_b2 * eq66_e541) + (eq66_e538 * eq66_e541_d_b2));
        let eq66_e543_d_b3: f64 = ((eq66_e538_d_b3 * eq66_e541) + (eq66_e538 * eq66_e541_d_b3));
        let eq66_e543_d_b4: f64 = ((eq66_e538_d_b4 * eq66_e541) + (eq66_e538 * eq66_e541_d_b4));
        let eq66_e543_d_b5: f64 = ((eq66_e538_d_b5 * eq66_e541) + (eq66_e538 * eq66_e541_d_b5));
        let eq66_e543_q: f64 = (eq66_e538 * eq66_e542_q);
        let eq66_e543_q_d_n0: f64 = ((eq66_e538_d_n0 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n0));
        let eq66_e543_q_d_n1: f64 = ((eq66_e538_d_n1 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n1));
        let eq66_e543_q_d_n2: f64 = ((eq66_e538_d_n2 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n2));
        let eq66_e543_q_d_n3: f64 = ((eq66_e538_d_n3 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n3));
        let eq66_e543_q_d_n4: f64 = ((eq66_e538_d_n4 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n4));
        let eq66_e543_q_d_n5: f64 = ((eq66_e538_d_n5 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n5));
        let eq66_e543_q_d_n6: f64 = ((eq66_e538_d_n6 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n6));
        let eq66_e543_q_d_n7: f64 = ((eq66_e538_d_n7 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n7));
        let eq66_e543_q_d_n8: f64 = ((eq66_e538_d_n8 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n8));
        let eq66_e543_q_d_n9: f64 = ((eq66_e538_d_n9 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n9));
        let eq66_e543_q_d_n10: f64 = ((eq66_e538_d_n10 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n10));
        let eq66_e543_q_d_n11: f64 = ((eq66_e538_d_n11 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n11));
        let eq66_e543_q_d_n12: f64 = ((eq66_e538_d_n12 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n12));
        let eq66_e543_q_d_n13: f64 = ((eq66_e538_d_n13 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n13));
        let eq66_e543_q_d_n14: f64 = ((eq66_e538_d_n14 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n14));
        let eq66_e543_q_d_b0: f64 = ((eq66_e538_d_b0 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_b0));
        let eq66_e543_q_d_b1: f64 = ((eq66_e538_d_b1 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_b1));
        let eq66_e543_q_d_b2: f64 = ((eq66_e538_d_b2 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_b2));
        let eq66_e543_q_d_b3: f64 = ((eq66_e538_d_b3 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_b3));
        let eq66_e543_q_d_b4: f64 = ((eq66_e538_d_b4 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_b4));
        let eq66_e543_q_d_b5: f64 = ((eq66_e538_d_b5 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_b5));
        (eq66_e543, eq66_e543_d_n0, eq66_e543_d_n1, eq66_e543_d_n2, eq66_e543_d_n3, eq66_e543_d_n4, eq66_e543_d_n5, eq66_e543_d_n6, eq66_e543_d_n7, eq66_e543_d_n8, eq66_e543_d_n9, eq66_e543_d_n10, eq66_e543_d_n11, eq66_e543_d_n12, eq66_e543_d_n13, eq66_e543_d_n14, eq66_e543_d_b0, eq66_e543_d_b1, eq66_e543_d_b2, eq66_e543_d_b3, eq66_e543_d_b4, eq66_e543_d_b5, eq66_e543_q, eq66_e543_q_d_n0, eq66_e543_q_d_n1, eq66_e543_q_d_n2, eq66_e543_q_d_n3, eq66_e543_q_d_n4, eq66_e543_q_d_n5, eq66_e543_q_d_n6, eq66_e543_q_d_n7, eq66_e543_q_d_n8, eq66_e543_q_d_n9, eq66_e543_q_d_n10, eq66_e543_q_d_n11, eq66_e543_q_d_n12, eq66_e543_q_d_n13, eq66_e543_q_d_n14, eq66_e543_q_d_b0, eq66_e543_q_d_b1, eq66_e543_q_d_b2, eq66_e543_q_d_b3, eq66_e543_q_d_b4, eq66_e543_q_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_reactive_node_derivatives: [f64; 15] = [eq66_e545_q_d_n0, eq66_e545_q_d_n1, eq66_e545_q_d_n2, eq66_e545_q_d_n3, eq66_e545_q_d_n4, eq66_e545_q_d_n5, eq66_e545_q_d_n6, eq66_e545_q_d_n7, eq66_e545_q_d_n8, eq66_e545_q_d_n9, eq66_e545_q_d_n10, eq66_e545_q_d_n11, eq66_e545_q_d_n12, eq66_e545_q_d_n13, eq66_e545_q_d_n14];
        let eq66_reactive_branch_derivatives: [f64; 6] = [eq66_e545_q_d_b0, eq66_e545_q_d_b1, eq66_e545_q_d_b2, eq66_e545_q_d_b3, eq66_e545_q_d_b4, eq66_e545_q_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq66_reactive_node_derivatives,
            branches,
            &eq66_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
