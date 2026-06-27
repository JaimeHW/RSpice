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
        s.store_scaled_voltage(202, ctx, nodes, Some(8), Some(6), p.p148);

        s.store_scaled_voltage(203, ctx, nodes, Some(8), Some(5), p.p148);

        s.store_sub(204, 202, 203);

        s.store_scaled_voltage(205, ctx, nodes, Some(7), Some(6), p.p148);

        s.store_scaled_voltage(206, ctx, nodes, Some(7), Some(5), p.p148);

        s.store_scaled_voltage(207, ctx, nodes, Some(1), Some(5), p.p148);

        s.store_scaled_voltage(208, ctx, nodes, Some(9), Some(5), p.p148);

        s.store_scaled_voltage(209, ctx, nodes, Some(3), Some(0), p.p148);

        s.b[279] = (p.p0 <= 310.0);
        s.v[279] = if s.b[279] { 1.0 } else { 0.0 };

        if s.b[279] {
            s.store_scalar(0, 1.6021918e-19);
            s.store_scalar(1, 1.3806226e-23);
        }

        if (!s.b[279]) {
            s.store_scalar(0, 1.602176634e-19);
            s.store_scalar(1, 1.380649e-23);
        }

        s.v[233] = 0.0;

        s.v[8] = (p.p146 + 273.15);

        s.v[9] = ctx_temp;

        s.store_div(2, 1, 0);

        s.store_scale(3, 2, 300.0);

        s.store_scale(6, 2, s.v[8]);

        s.store_div_from_scalar(7, 1.0, 6);

        s.v[276] = ((p.p121 * s.v[8]) * ((s.v[8]) as f64).ln());

        s.v[277] = (p.p122 * s.v[8]);

        s.v[56] = (p.p131 * s.v[8]);

        s.v[88] = ((p.p117 + s.v[276]) + s.v[277]);

        s.v[89] = ((p.p118 + s.v[276]) + s.v[277]);

        s.v[90] = ((p.p119 + s.v[276]) + s.v[277]);

        s.v[91] = ((s.v[88] + s.v[89]) * 0.5);

        s.v[92] = ((s.v[88] + s.v[90]) * 0.5);

        s.v[77] = ((p.p117 + p.p118) * 0.5);

        s.v[78] = ((p.p117 + p.p119) * 0.5);

        s.v[79] = ((p.p120 + p.p119) * 0.5);

        s.store_sub_from_scalar_ad(76, 3.0, A::div_from_scalar(p.p121, s.ad_value(2)));

        s.store_offset(80, 76, ((1.0) + ((-p.p130))));

        s.store_offset(81, 76, ((1.0) + ((-p.p138))));

        s.store_offset(82, 76, (-1.5));

        s.v[278] = ((1.0 - p.p107) * (p.p52 + p.p106));

        s.b[280] = (s.v[278] >= p.p106);
        s.v[280] = if s.b[280] { 1.0 } else { 0.0 };

        if s.b[280] {
            s.store_scalar(171, p.p106);
            s.store_scalar(172, 0.0);
            s.store_scalar(176, (s.v[278] - p.p106));
            s.store_sub_from_scalar(177, p.p52, 176);
        }

        if (!s.b[280]) {
            s.store_scalar(171, s.v[278]);
            s.store_sub_from_scalar(172, p.p106, 171);
            s.store_scalar(176, 0.0);
            s.store_scalar(177, p.p52);
        }

        s.v[174] = (p.p105 * p.p104);

        s.v[173] = (p.p104 - s.v[174]);

        s.b[281] = (p.p22 != 0.0);
        s.v[281] = if s.b[281] { 1.0 } else { 0.0 };

        if s.b[281] {
            s.store_scalar(175, (1.0 / p.p22));
        }

        if (!s.b[281]) {
            s.store_scalar(175, 0.0);
        }

        s.b[282] = (p.p0 <= 300.0);
        s.v[282] = if s.b[282] { 1.0 } else { 0.0 };

        let (assign510_e806,) = {
    if s.b[282] {
        (0.0,)
    } else {
        (s.v[223],)
    }
};
        s.v[223] = assign510_e806;

        let (assign520_e811,) = {
    if (!s.b[282]) {
        (0.7,)
    } else {
        (s.v[223],)
    }
};
        s.v[223] = assign520_e811;

        s.v[244] = 0.0;

        s.b[283] = ((p.p32 > 0.0) && (p.p47 > 0.0));
        s.v[283] = if s.b[283] { 1.0 } else { 0.0 };

        let (assign550_e823,) = {
    if s.b[283] {
        (1.0,)
    } else {
        (s.v[243],)
    }
};
        s.v[243] = assign550_e823;

        let (assign560_e828,) = {
    if (!s.b[283]) {
        (0.0,)
    } else {
        (s.v[243],)
    }
};
        s.v[243] = assign560_e828;

        s.v[234] = p.p86;

        s.b[284] = (p.p86 != 0.0);
        s.v[284] = if s.b[284] { 1.0 } else { 0.0 };

        s.b[285] = (((p.p88 == 0.0) && (p.p87 == 0.0)) || (p.p66 == 0.0));
        s.v[285] = if s.b[285] { 1.0 } else { 0.0 };

        let (assign600_e849,) = {
    if (s.b[284] && s.b[285]) {
        (0.0,)
    } else {
        (s.v[234],)
    }
};
        s.v[234] = assign600_e849;

        s.b[286] = ((p.p115 >= 0.01) || (p.p116 >= 0.01));
        s.v[286] = if s.b[286] { 1.0 } else { 0.0 };

        if s.b[286] {
            s.store_scalar(232, (0.5 * (p.p115 - p.p116)));
        }

        s.b[287] = (p.p116 < p.p115);
        s.v[287] = if s.b[287] { 1.0 } else { 0.0 };

        let (assign640_e873,) = {
    if (s.b[286] && s.b[287]) {
        (p.p116,)
    } else {
        (s.v[229],)
    }
};
        s.v[229] = assign640_e873;

        if (s.b[286] && s.b[287]) {
            s.store_scalar(230, p.p115);
        }

        let (assign660_e886,) = {
    if (s.b[286] && (!s.b[287])) {
        (p.p115,)
    } else {
        (s.v[229],)
    }
};
        s.v[229] = assign660_e886;

        if (s.b[286] && (!s.b[287])) {
            s.store_scalar(230, p.p116);
        }

        s.b[288] = (s.v[229] < 0.01);
        s.v[288] = if s.b[288] { 1.0 } else { 0.0 };

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
        }

        let (assign840_e1009,) = {
    if (!s.b[286]) {
        (p.p116,)
    } else {
        (s.v[229],)
    }
};
        s.v[229] = assign840_e1009;

        if (!s.b[286]) {
            s.store_scalar(230, p.p115);
            s.store_scalar(231, 0.0);
        }

        s.v[10] = (s.v[9] + p.p147);

        s.b[289] = (s.v[10] < ((-200.0) + 273.15));
        s.v[289] = if s.b[289] { 1.0 } else { 0.0 };

        if s.b[289] {
            s.store_scalar(10, ((-200.0) + 273.15));
        }

        s.b[290] = (s.v[10] > (326.85 + 273.15));
        s.v[290] = if s.b[290] { 1.0 } else { 0.0 };

        if ((!s.b[289]) && s.b[290]) {
            s.store_scalar(10, (326.85 + 273.15));
        }

        s.store_mul(4, 2, 10);

        s.store_div_from_scalar(5, 1.0, 4);

        s.store_offset(14, 10, (-s.v[8]));

        s.store_div_from_scalar(12, s.v[8], 10);

        s.store_scale(11, 10, 1.0 / (s.v[8]));

        s.store_ln(13, 11);

        s.store_mul_scaled_ad_rhs(74, 10, p.p121, A::ln(s.ad_value(10)));

        s.store_scale(75, 10, p.p122);

        s.store_add_ad_lhs(84, A::offset(s.ad_value(74), p.p117), 75);

        s.store_add_ad_lhs(83, A::offset(s.ad_value(74), p.p118), 75);

        s.store_add_ad_lhs(85, A::offset(s.ad_value(74), p.p119), 75);

        s.store_scaled_add(86, 84, 83, 0.5);

        s.store_scaled_add(87, 84, 85, 0.5);

        s.b[291] = (p.p39 > 0.0);
        s.v[291] = if s.b[291] { 1.0 } else { 0.0 };

        if s.b[291] {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p40 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p40)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[77]), s.v[77]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(27, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(26, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p40, s.ad_value(27))), p.p41), p.p39);
            s.store_scalar(28, ((p.p42) as f64).abs());
        }

        s.b[292] = (p.p42 > 0.0);
        s.v[292] = if s.b[292] { 1.0 } else { 0.0 };

        if (s.b[291] && s.b[292]) {
            s.store_scale(28, 27, (p.p42 * 1.0 / (p.p40)));
        }

        if (!s.b[291]) {
            s.store_scalar(26, p.p39);
            s.store_scalar(27, p.p40);
            s.store_scalar(28, p.p42);
        }

        s.store_scaled_exp_ad(22, A::add_scaled_inputs(s.ad_value(13), p.p124, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p.p118), 1.0), p.p14);

        s.store_scaled_exp_ad(24, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), (s.v[77] * 1.0 / (p.p17))), 1.0, s.ad_value(76), s.ad_value(13), 1.0 / (p.p17)), p.p16);

        s.b[293] = (p.p47 > 0.0);
        s.v[293] = if s.b[293] { 1.0 } else { 0.0 };

        if s.b[293] {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p48 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p48)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[78]), s.v[78]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(34, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(33, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p48, s.ad_value(34))), p.p49), p.p47);
            s.store_scalar(35, ((p.p50) as f64).abs());
        }

        s.b[294] = (p.p50 > 0.0);
        s.v[294] = if s.b[294] { 1.0 } else { 0.0 };

        if (s.b[293] && s.b[294]) {
            s.store_scale(35, 34, (p.p50 * 1.0 / (p.p48)));
        }

        if (!s.b[293]) {
            s.store_scalar(33, p.p47);
            s.store_scalar(34, p.p48);
            s.store_scalar(35, p.p50);
        }

        s.b[295] = (p.p0 <= 300.0);
        s.v[295] = if s.b[295] { 1.0 } else { 0.0 };

        if s.b[295] {
            s.store_scalar(35, 2.4);
        }

        s.store_scaled_exp_ad(32, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p.p119), 1.0, s.ad_value(80), s.ad_value(13), 1.0), p.p23);

        s.store_offset_scaled_ad(16, A::exp_scaled_input(A::ln_scaled_input(s.ad_value(27), 1.0 / (p.p40)), p.p41), (-p.p2), ((2.0) * (p.p2)));

        s.store_scaled_exp_ad(15, A::add_scaled_inputs(s.ad_value(13), p.p123, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p.p117), 1.0), p.p1);

        s.store_scaled_exp_scaled_input(18, 13, p.p126, p.p10);

        s.b[296] = ((p.p0 <= 300.0) && ((((p.p8 - 1.0)) as f64).abs() < 1e-5));
        s.v[296] = if s.b[296] { 1.0 } else { 0.0 };

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
        s.v[297] = if s.b[297] { 1.0 } else { 0.0 };

        if s.b[297] {
            s.store_offset_scaled_ad(58, A::scale(s.ad_value(14), p.p133), (-p.p79), p.p79);
            s.store_scalar(57, p.p78);
        }

        if (!s.b[297]) {
            s.store_offset_scaled(57, 14, ((p.p132) * (p.p78)), p.p78);
            s.store_scalar(58, p.p79);
        }

        s.store_add_scaled_product_value_ad(59, A::scale_offset(s.ad_value(14), p.p128, 1.0), p.p66, 14, 14, (p.p129 * p.p66));

        s.v[61] = p.p69;

        s.store_scaled_exp_scaled_input(60, 13, (p.p130 - 1.0), p.p71);

        s.b[298] = (s.v[243] == 1.0);
        s.v[298] = if s.b[298] { 1.0 } else { 0.0 };

        if s.b[298] {
            s.store_scaled_exp_scaled_input(63, 14, p.p139, p.p32);
            s.store_scaled_exp_scaled_input(62, 14, p.p140, p.p33);
        }

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (!s.b[298]) {
            s.store_scalar(63, p.p32);
            s.store_scalar(62, p.p33);
        }

        s.b[299] = ((p.p37 > 0.0) && (s.v[203] < 0.0));
        s.v[299] = if s.b[299] { 1.0 } else { 0.0 };

        if s.b[299] {
            s.store_scalar(67, p.p37);
            s.store_scalar(68, p.p38);
        }

        s.b[300] = ((p.p47 > 0.0) && (p.p48 > 0.0));
        s.v[300] = if s.b[300] { 1.0 } else { 0.0 };

        if (s.b[299] && s.b[300]) {
            s.store_div_from_scalar(169, s.v[92], 87);
            s.store_scale(170, 34, 1.0 / (p.p48));
            s.store_mul_ad_affine_product_lhs(168, A::sqrt(s.ad_value(169)), s.ad_value(170), 1.0 / (p.p47), 0.0, 33);
            s.store_scaled_mul(67, 168, 170, p.p37);
            s.store_div_from_scalar_mul_ad(68, p.p38, s.ad_value(168), s.ad_value(169));
        }

        if (!s.b[299]) {
            s.store_scalar(67, 0.0);
            s.store_scalar(68, 1.0);
        }

        s.store_scaled_exp_scaled_input(69, 13, p.p134, p.p89);

        s.b[301] = (p.p43 > 0.0);
        s.v[301] = if s.b[301] { 1.0 } else { 0.0 };

        if s.b[301] {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p44 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p44)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[77]), s.v[77]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(30, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(29, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p44, s.ad_value(30))), p.p45), p.p43);
            s.store_scalar(31, ((p.p46) as f64).abs());
        }

        s.b[302] = (p.p46 > 0.0);
        s.v[302] = if s.b[302] { 1.0 } else { 0.0 };

        if (s.b[301] && s.b[302]) {
            s.store_scale(31, 30, (p.p46 * 1.0 / (p.p44)));
        }

        if (!s.b[301]) {
            s.store_scalar(29, p.p43);
            s.store_scalar(30, p.p44);
            s.store_scalar(31, p.p46);
        }

        s.store_scaled_exp_ad(23, A::add_scaled_inputs(s.ad_value(13), p.p124, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p.p118), 1.0), p.p18);

        s.store_scaled_exp_ad(25, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), (s.v[77] * 1.0 / (p.p21))), 1.0, s.ad_value(76), s.ad_value(13), 1.0 / (p.p21)), p.p20);

        s.b[303] = ((p.p27 > 0.0) && ((s.v[205] < s.v[223]) || (s.v[202] < s.v[223])));
        s.v[303] = if s.b[303] { 1.0 } else { 0.0 };

        if s.b[303] {
            s.store_scalar(166, 1.0);
            s.store_scalar(167, 1.0);
            s.store_div_from_scalar(169, s.v[91], 86);
        }

        s.b[304] = (((p.p29 == 1.0) && (p.p43 > 0.0)) && (p.p44 > 0.0));
        s.v[304] = if s.b[304] { 1.0 } else { 0.0 };

        if (s.b[303] && s.b[304]) {
            s.store_scale(170, 30, 1.0 / (p.p44));
            s.store_mul_product3_rhs(167, 170, s.ad_value(29), A::sqrt(s.ad_value(169)), s.ad_value(170), 1.0 / (p.p43));
            s.store_div_scaled_value_by_product(166, A::powf(s.ad_value(169), (-1.5)), p.p43, s.ad_value(29), s.ad_value(170), 1.0);
        }

        s.b[305] = (((p.p29 == 0.0) && (p.p39 > 0.0)) && (p.p40 > 0.0));
        s.v[305] = if s.b[305] { 1.0 } else { 0.0 };

        if ((s.b[303] && (!s.b[304])) && s.b[305]) {
            s.store_scale(170, 27, 1.0 / (p.p40));
            s.store_mul_product3_rhs(167, 170, s.ad_value(26), A::sqrt(s.ad_value(169)), s.ad_value(170), 1.0 / (p.p39));
            s.store_div_scaled_value_by_product(166, A::powf(s.ad_value(169), (-1.5)), p.p39, s.ad_value(26), s.ad_value(170), 1.0);
        }

        if s.b[303] {
            s.store_scale(64, 167, p.p27);
            s.store_scale(65, 166, p.p28);
        }

        if (!s.b[303]) {
            s.store_scalar(64, 0.0);
            s.store_scalar(65, 1.0);
        }

        s.store_scale_ad(66, A::exp_scaled_input(A::offset(s.ad_value(27), (-p.p40)), (-1.0 / (p.p31))), p.p30);

        s.b[306] = (1.0 > 0.0);
        s.v[306] = if s.b[306] { 1.0 } else { 0.0 };

        if s.b[306] {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p53 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p53)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[78]), s.v[78]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(39, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_exp_scaled_input_ad(43, A::ln(A::div_from_scalar(p.p53, s.ad_value(39))), p.p54);
            s.store_scalar(40, ((p.p55) as f64).abs());
        }

        s.b[307] = (p.p55 > 0.0);
        s.v[307] = if s.b[307] { 1.0 } else { 0.0 };

        if (s.b[306] && s.b[307]) {
            s.store_scale(40, 39, (p.p55 * 1.0 / (p.p53)));
        }

        if (!s.b[306]) {
            s.store_scalar(43, 1.0);
            s.store_scalar(39, p.p53);
            s.store_scalar(40, p.p55);
        }

        s.b[308] = (p.p0 <= 300.0);
        s.v[308] = if s.b[308] { 1.0 } else { 0.0 };

        if s.b[308] {
            s.store_scalar(40, 2.4);
        }

        s.store_mul(37, 43, 176);

        s.store_mul(38, 43, 177);

        s.store_scaled_exp_ad(36, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p.p119), 1.0, s.ad_value(81), s.ad_value(13), 1.0), p.p25);

        s.b[309] = (p.p0 <= 300.0);
        s.v[309] = if s.b[309] { 1.0 } else { 0.0 };

        s.b[310] = (p.p57 > 0.0);
        s.v[310] = if s.b[310] { 1.0 } else { 0.0 };

        if (s.b[309] && s.b[310]) {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p58 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p58)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(47, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(46, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p58, s.ad_value(47))), p.p59), p.p57);
            s.store_scalar(48, (((-2.4)) as f64).abs());
        }

        s.b[311] = ((-2.4) > 0.0);
        s.v[311] = if s.b[311] { 1.0 } else { 0.0 };

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
        s.v[312] = if s.b[312] { 1.0 } else { 0.0 };

        if ((!s.b[309]) && s.b[312]) {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p58 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p58)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(47, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(46, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p58, s.ad_value(47))), p.p59), p.p57);
            s.store_scalar(48, (((-p.p60)) as f64).abs());
        }

        s.b[313] = ((-p.p60) > 0.0);
        s.v[313] = if s.b[313] { 1.0 } else { 0.0 };

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

        s.store_scaled_exp_ad(45, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p.p120), 1.0, s.ad_value(82), s.ad_value(13), 1.0), p.p99);

        s.store_scaled_exp_ad(44, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p.p119), 1.0, s.ad_value(82), s.ad_value(13), 1.0), p.p97);

        s.store_scaled_exp_scaled_input(52, 13, (p.p138 - 1.0), p.p101);

        s.b[314] = (p.p63 > 0.0);
        s.v[314] = if s.b[314] { 1.0 } else { 0.0 };

        s.b[315] = (p.p62 > 0.0);
        s.v[315] = if s.b[315] { 1.0 } else { 0.0 };

        if (s.b[314] && s.b[315]) {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p63 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p63)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(50, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(49, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p63, s.ad_value(50))), p.p64), p.p62);
            s.store_abs_scaled_input(51, 163, -1.0);
        }

        s.b[316] = ((-s.v[163]) > 0.0);
        s.v[316] = if s.b[316] { 1.0 } else { 0.0 };

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

        s.store_scaled_exp_scaled_input(72, 13, p.p136, p.p96);

        s.store_scaled_exp_scaled_input(71, 13, p.p135, p.p90);

        s.store_scaled_exp_scaled_input(73, 13, p.p137, p.p95);

        s.store_scaled_mul_scale_offset_rhs_ad(201, A::exp_scaled_input(s.ad_value(13), p.p143), 14, p.p144, 1.0, p.p142);

        s.b[317] = (((p.p141 != 0.0) && (p.p142 >= p.p149)) && (p.p142 > 0.0));
        s.v[317] = if s.b[317] { 1.0 } else { 0.0 };

        if s.b[317] {
            s.store_offset_voltage(10, ctx, nodes, Some(4), None, (s.v[9] + p.p147));
        }

        s.b[318] = (s.v[10] < ((-200.0) + 273.15));
        s.v[318] = if s.b[318] { 1.0 } else { 0.0 };

        if (s.b[317] && s.b[318]) {
            s.store_scalar(10, ((-200.0) + 273.15));
        }

        s.b[319] = (s.v[10] > (326.85 + 273.15));
        s.v[319] = if s.b[319] { 1.0 } else { 0.0 };

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
            s.store_mul_scaled_ad_rhs(74, 10, p.p121, A::ln(s.ad_value(10)));
            s.store_scale(75, 10, p.p122);
            s.store_add_ad_lhs(84, A::offset(s.ad_value(74), p.p117), 75);
            s.store_add_ad_lhs(83, A::offset(s.ad_value(74), p.p118), 75);
            s.store_add_ad_lhs(85, A::offset(s.ad_value(74), p.p119), 75);
            s.store_scaled_add(86, 84, 83, 0.5);
            s.store_scaled_add(87, 84, 85, 0.5);
        }

        s.b[320] = (p.p39 > 0.0);
        s.v[320] = if s.b[320] { 1.0 } else { 0.0 };

        if (s.b[317] && s.b[320]) {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p40 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p40)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[77]), s.v[77]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(27, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(26, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p40, s.ad_value(27))), p.p41), p.p39);
            s.store_scalar(28, ((p.p42) as f64).abs());
        }

        s.b[321] = (p.p42 > 0.0);
        s.v[321] = if s.b[321] { 1.0 } else { 0.0 };

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
            s.store_scaled_exp_ad(24, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), (s.v[77] * 1.0 / (p.p17))), 1.0, s.ad_value(76), s.ad_value(13), 1.0 / (p.p17)), p.p16);
        }

        s.b[322] = (p.p47 > 0.0);
        s.v[322] = if s.b[322] { 1.0 } else { 0.0 };

        if (s.b[317] && s.b[322]) {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p48 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p48)))));
        }

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[317] && s.b[322]) {
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[78]), s.v[78]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(34, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(33, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p48, s.ad_value(34))), p.p49), p.p47);
            s.store_scalar(35, ((p.p50) as f64).abs());
        }

        s.b[323] = (p.p50 > 0.0);
        s.v[323] = if s.b[323] { 1.0 } else { 0.0 };

        if ((s.b[317] && s.b[322]) && s.b[323]) {
            s.store_scale(35, 34, (p.p50 * 1.0 / (p.p48)));
        }

        if (s.b[317] && (!s.b[322])) {
            s.store_scalar(33, p.p47);
            s.store_scalar(34, p.p48);
            s.store_scalar(35, p.p50);
        }

        s.b[324] = (p.p0 <= 300.0);
        s.v[324] = if s.b[324] { 1.0 } else { 0.0 };

        if (s.b[317] && s.b[324]) {
            s.store_scalar(35, 2.4);
        }

        if s.b[317] {
            s.store_scaled_exp_ad(32, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p.p119), 1.0, s.ad_value(80), s.ad_value(13), 1.0), p.p23);
            s.store_offset_scaled_ad(16, A::exp_scaled_input(A::ln_scaled_input(s.ad_value(27), 1.0 / (p.p40)), p.p41), (-p.p2), ((2.0) * (p.p2)));
            s.store_scaled_exp_ad(15, A::add_scaled_inputs(s.ad_value(13), p.p123, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p.p117), 1.0), p.p1);
            s.store_scaled_exp_scaled_input(18, 13, p.p126, p.p10);
        }

        s.b[325] = ((p.p0 <= 300.0) && ((((p.p8 - 1.0)) as f64).abs() < 1e-5));
        s.v[325] = if s.b[325] { 1.0 } else { 0.0 };

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
        s.v[326] = if s.b[326] { 1.0 } else { 0.0 };

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

        s.b[327] = (s.v[243] == 1.0);
        s.v[327] = if s.b[327] { 1.0 } else { 0.0 };

        if (s.b[317] && s.b[327]) {
            s.store_scaled_exp_scaled_input(63, 14, p.p139, p.p32);
            s.store_scaled_exp_scaled_input(62, 14, p.p140, p.p33);
        }

        if (s.b[317] && (!s.b[327])) {
            s.store_scalar(63, p.p32);
            s.store_scalar(62, p.p33);
        }

        s.b[328] = ((p.p37 > 0.0) && (s.v[203] < 0.0));
        s.v[328] = if s.b[328] { 1.0 } else { 0.0 };

        if (s.b[317] && s.b[328]) {
            s.store_scalar(67, p.p37);
            s.store_scalar(68, p.p38);
        }

        s.b[329] = ((p.p47 > 0.0) && (p.p48 > 0.0));
        s.v[329] = if s.b[329] { 1.0 } else { 0.0 };

        if ((s.b[317] && s.b[328]) && s.b[329]) {
            s.store_div_from_scalar(169, s.v[92], 87);
            s.store_scale(170, 34, 1.0 / (p.p48));
            s.store_mul_ad_affine_product_lhs(168, A::sqrt(s.ad_value(169)), s.ad_value(170), 1.0 / (p.p47), 0.0, 33);
            s.store_scaled_mul(67, 168, 170, p.p37);
            s.store_div_from_scalar_mul_ad(68, p.p38, s.ad_value(168), s.ad_value(169));
        }

        if (s.b[317] && (!s.b[328])) {
            s.store_scalar(67, 0.0);
            s.store_scalar(68, 1.0);
        }

        if s.b[317] {
            s.store_scaled_exp_scaled_input(69, 13, p.p134, p.p89);
        }

        s.b[330] = (p.p43 > 0.0);
        s.v[330] = if s.b[330] { 1.0 } else { 0.0 };

        if (s.b[317] && s.b[330]) {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p44 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p44)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[77]), s.v[77]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(30, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(29, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p44, s.ad_value(30))), p.p45), p.p43);
            s.store_scalar(31, ((p.p46) as f64).abs());
        }

        s.b[331] = (p.p46 > 0.0);
        s.v[331] = if s.b[331] { 1.0 } else { 0.0 };

        if ((s.b[317] && s.b[330]) && s.b[331]) {
            s.store_scale(31, 30, (p.p46 * 1.0 / (p.p44)));
        }

        if (s.b[317] && (!s.b[330])) {
            s.store_scalar(29, p.p43);
            s.store_scalar(30, p.p44);
            s.store_scalar(31, p.p46);
        }

        if s.b[317] {
            s.store_scaled_exp_ad(23, A::add_scaled_inputs(s.ad_value(13), p.p124, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p.p118), 1.0), p.p18);
            s.store_scaled_exp_ad(25, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), (s.v[77] * 1.0 / (p.p21))), 1.0, s.ad_value(76), s.ad_value(13), 1.0 / (p.p21)), p.p20);
        }

        s.b[332] = ((p.p27 > 0.0) && ((s.v[205] < s.v[223]) || (s.v[202] < s.v[223])));
        s.v[332] = if s.b[332] { 1.0 } else { 0.0 };

        if (s.b[317] && s.b[332]) {
            s.store_scalar(166, 1.0);
            s.store_scalar(167, 1.0);
            s.store_div_from_scalar(169, s.v[91], 86);
        }

        s.b[333] = (((p.p29 == 1.0) && (p.p43 > 0.0)) && (p.p44 > 0.0));
        s.v[333] = if s.b[333] { 1.0 } else { 0.0 };

        if ((s.b[317] && s.b[332]) && s.b[333]) {
            s.store_scale(170, 30, 1.0 / (p.p44));
            s.store_mul_product3_rhs(167, 170, s.ad_value(29), A::sqrt(s.ad_value(169)), s.ad_value(170), 1.0 / (p.p43));
            s.store_div_scaled_value_by_product(166, A::powf(s.ad_value(169), (-1.5)), p.p43, s.ad_value(29), s.ad_value(170), 1.0);
        }

        s.b[334] = (((p.p29 == 0.0) && (p.p39 > 0.0)) && (p.p40 > 0.0));
        s.v[334] = if s.b[334] { 1.0 } else { 0.0 };

        if (((s.b[317] && s.b[332]) && (!s.b[333])) && s.b[334]) {
            s.store_scale(170, 27, 1.0 / (p.p40));
            s.store_mul_product3_rhs(167, 170, s.ad_value(26), A::sqrt(s.ad_value(169)), s.ad_value(170), 1.0 / (p.p39));
            s.store_div_scaled_value_by_product(166, A::powf(s.ad_value(169), (-1.5)), p.p39, s.ad_value(26), s.ad_value(170), 1.0);
        }

        if (s.b[317] && s.b[332]) {
            s.store_scale(64, 167, p.p27);
            s.store_scale(65, 166, p.p28);
        }

        if (s.b[317] && (!s.b[332])) {
            s.store_scalar(64, 0.0);
            s.store_scalar(65, 1.0);
        }

        if s.b[317] {
            s.store_scale_ad(66, A::exp_scaled_input(A::offset(s.ad_value(27), (-p.p40)), (-1.0 / (p.p31))), p.p30);
        }

        s.b[335] = (1.0 > 0.0);
        s.v[335] = if s.b[335] { 1.0 } else { 0.0 };

        if (s.b[317] && s.b[335]) {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p53 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p53)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[78]), s.v[78]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(39, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_exp_scaled_input_ad(43, A::ln(A::div_from_scalar(p.p53, s.ad_value(39))), p.p54);
            s.store_scalar(40, ((p.p55) as f64).abs());
        }

        s.b[336] = (p.p55 > 0.0);
        s.v[336] = if s.b[336] { 1.0 } else { 0.0 };

        if ((s.b[317] && s.b[335]) && s.b[336]) {
            s.store_scale(40, 39, (p.p55 * 1.0 / (p.p53)));
        }

        if (s.b[317] && (!s.b[335])) {
            s.store_scalar(43, 1.0);
            s.store_scalar(39, p.p53);
            s.store_scalar(40, p.p55);
        }

        s.b[337] = (p.p0 <= 300.0);
        s.v[337] = if s.b[337] { 1.0 } else { 0.0 };

        if (s.b[317] && s.b[337]) {
            s.store_scalar(40, 2.4);
        }

        if s.b[317] {
            s.store_mul(37, 43, 176);
            s.store_mul(38, 43, 177);
            s.store_scaled_exp_ad(36, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p.p119), 1.0, s.ad_value(81), s.ad_value(13), 1.0), p.p25);
        }

        s.b[338] = (p.p0 <= 300.0);
        s.v[338] = if s.b[338] { 1.0 } else { 0.0 };

        s.b[339] = (p.p57 > 0.0);
        s.v[339] = if s.b[339] { 1.0 } else { 0.0 };

        if ((s.b[317] && s.b[338]) && s.b[339]) {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p58 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p58)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(47, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(46, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p58, s.ad_value(47))), p.p59), p.p57);
            s.store_scalar(48, (((-2.4)) as f64).abs());
        }

        s.b[340] = ((-2.4) > 0.0);
        s.v[340] = if s.b[340] { 1.0 } else { 0.0 };

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
        s.v[341] = if s.b[341] { 1.0 } else { 0.0 };

        if ((s.b[317] && (!s.b[338])) && s.b[341]) {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p58 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p58)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(47, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(46, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p58, s.ad_value(47))), p.p59), p.p57);
            s.store_scalar(48, (((-p.p60)) as f64).abs());
        }

        s.b[342] = ((-p.p60) > 0.0);
        s.v[342] = if s.b[342] { 1.0 } else { 0.0 };

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
            s.store_scaled_exp_ad(45, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p.p120), 1.0, s.ad_value(82), s.ad_value(13), 1.0), p.p99);
            s.store_scaled_exp_ad(44, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p.p119), 1.0, s.ad_value(82), s.ad_value(13), 1.0), p.p97);
            s.store_scaled_exp_scaled_input(52, 13, (p.p138 - 1.0), p.p101);
        }

        s.b[343] = (p.p63 > 0.0);
        s.v[343] = if s.b[343] { 1.0 } else { 0.0 };

        s.b[344] = (p.p62 > 0.0);
        s.v[344] = if s.b[344] { 1.0 } else { 0.0 };

        if ((s.b[317] && s.b[343]) && s.b[344]) {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p63 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p63)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(50, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(49, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p63, s.ad_value(50))), p.p64), p.p62);
            s.store_abs_scaled_input(51, 163, -1.0);
        }

        s.b[345] = ((-s.v[163]) > 0.0);
        s.v[345] = if s.b[345] { 1.0 } else { 0.0 };

        if (((s.b[317] && s.b[343]) && s.b[344]) && s.b[345]) {
            s.store_scaled_mul(51, 163, 50, (-1.0 / (p.p63)));
        }

        if ((s.b[317] && s.b[343]) && (!s.b[344])) {
            s.store_scalar(49, p.p62);
        }

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[317] && s.b[343]) && (!s.b[344])) {
            s.store_scalar(50, p.p63);
            s.store_neg(51, 163);
        }

        if (s.b[317] && (!s.b[343])) {
            s.store_scalar(49, p.p62);
            s.store_scalar(50, p.p63);
            s.copy_ad(51, 163);
        }

        if s.b[317] {
            s.store_scaled_exp_scaled_input(72, 13, p.p136, p.p96);
            s.store_scaled_exp_scaled_input(71, 13, p.p135, p.p90);
            s.store_scaled_exp_scaled_input(73, 13, p.p137, p.p95);
            s.store_scaled_mul_scale_offset_rhs_ad(201, A::exp_scaled_input(s.ad_value(13), p.p143), 14, p.p144, 1.0, p.p142);
        }

        s.b[364] = (p.p14 > 0.0);
        s.v[364] = if s.b[364] { 1.0 } else { 0.0 };

        if s.b[364] {
            s.store_div_scaled_inputs(93, s.ad_value(202), 1.0, s.ad_value(4), p.p15);
        }

        s.b[365] = (s.v[93] > 80.0);
        s.v[365] = if s.b[365] { 1.0 } else { 0.0 };

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
        s.v[366] = if s.b[366] { 1.0 } else { 0.0 };

        if s.b[366] {
            s.store_div_scaled_inputs(93, s.ad_value(202), 1.0, s.ad_value(4), p.p17);
        }

        s.b[367] = (s.v[93] > 80.0);
        s.v[367] = if s.b[367] { 1.0 } else { 0.0 };

        if (s.b[366] && s.b[367]) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
            s.store_scalar(93, 80.0);
        }

        if (s.b[366] && (!s.b[367])) {
            s.store_scalar(94, 1.0);
        }

        if s.b[366] {
            s.store_mul_offset_ad_rhs(186, 24, A::mul(s.ad_value(94), A::limexp(s.ad_value(93))), (-1.0));
        }

        if (!s.b[366]) {
            s.store_scalar(186, 0.0);
        }

        s.store_mul_ad_rhs(350, 15, A::limexp_scaled_input(A::mul(s.ad_value(202), s.ad_value(5)), 1.0 / (p.p13)));

        s.store_mul_limexp_ad_rhs(351, 15, A::mul(s.ad_value(203), s.ad_value(5)));

        s.b[368] = (s.v[26] > 0.0);
        s.v[368] = if s.b[368] { 1.0 } else { 0.0 };

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
        s.v[369] = if s.b[369] { 1.0 } else { 0.0 };

        s.b[370] = (s.v[33] > 0.0);
        s.v[370] = if s.b[370] { 1.0 } else { 0.0 };

        if (s.b[369] && s.b[370]) {
            s.store_scalar(113, (p.p49 / 4.0));
            s.store_sub_from_scalar(114, p.p51, 34);
            s.store_mul_sub_from_scalar_ad_rhs(115, 34, 1.0, A::exp_scaled_input(A::ln(s.ad_value(35)), (-1.0 / (p.p49))));
            s.store_mul(116, 35, 33);
            s.store_mul_exp_ad_rhs(117, 33, A::mul_offset_lhs(s.ad_value(113), (-p.p49), A::ln(A::div_from_scalar(p.p51, s.ad_value(34)))));
            s.store_mul_sub_lhs(119, 115, 203, 5);
        }

        s.b[371] = (s.v[119] < 80.0);
        s.v[371] = if s.b[371] { 1.0 } else { 0.0 };

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
            s.store_div_scaled_inputs2(123, s.ad_value(114), 1.0, s.ad_value(122), 1.0, s.ad_value(118), 1.0);
        }

        s.b[372] = (s.v[123] < 80.0);
        s.v[372] = if s.b[372] { 1.0 } else { 0.0 };

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
            s.store_mul_product3_rhs(134, 124, s.ad_value(33), A::exp_scaled_input(s.ad_value(131), (-p.p49)), s.ad_value(121), 1.0);
            s.store_mul_ad_product_rhs(135, 117, A::exp(A::mul_scaled_rhs(s.ad_value(130), s.ad_value(113), -1.0)), A::sub_from_scalar(1.0, s.ad_value(124)));
            s.store_mul_sub_from_scalar_rhs(136, 116, 1.0, 121);
            s.store_add_scaled_inputs3(210, s.ad_value(134), 1.0, s.ad_value(135), 1.0, s.ad_value(136), 1.0);
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
        s.v[373] = if s.b[373] { 1.0 } else { 0.0 };

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
        s.v[374] = if s.b[374] { 1.0 } else { 0.0 };

        if s.b[374] {
            s.store_scale(375, 4, p.p11);
            s.store_div_scaled_inputs2(376, s.ad_value(27), 1.0, s.ad_value(202), (-1.0), s.ad_value(375), 1.0);
            s.store_add_scaled_product_right_ad(377, 27, 1.0, 375, A::add(s.ad_value(376), A::sqrt(A::offset(A::square(s.ad_value(376)), 1.921812))), (-0.5));
            s.store_mul_sub_from_scalar_ad_rhs(378, 18, 1.0, A::exp_scaled_input(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(377), s.ad_value(27)))), p.p41));
        }

        s.b[379] = (((s.v[378]) as f64).abs() > 0.001);
        s.v[379] = if s.b[379] { 1.0 } else { 0.0 };

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

        s.store_mul_offset_ad_rhs(352, 353, A::add_scaled_inputs(s.ad_value(347), 0.5, A::sqrt(A::offset(A::square(s.ad_value(347)), 1.921812)), 0.5), 1.0);

        s.store_scale(380, 34, (1.0 - ((((-((2.4) as f64).ln()) / p.p49)) as f64).exp()));

        s.store_mul_sub_lhs(381, 380, 203, 5);

        s.store_sqrt_square_offset(382, 381, 1.921812);

        s.store_scaled_add(383, 381, 382, 0.5);

        s.store_add_scaled_product_indices(384, 380, 1.0, 4, 383, (-1.0));

        s.store_div(385, 383, 382);

        s.store_add_scaled_product_mixed_aai(361, A::scale_offset(s.ad_value(385), (-2.4), 2.4), 1.0, A::exp_scaled_input(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(384), s.ad_value(34)))), (-p.p49)), 385, 1.0);

        s.store_add_scaled_inputs3_offset(357, s.ad_value(59), 1.0, A::div_from_scalar(1.0, s.ad_value(361)), p.p67, s.ad_value(361), p.p68, (((-1.0) * p.p67) + ((-1.0) * p.p68)));

        s.b[386] = (p.p79 > 0.0);
        s.v[386] = if s.b[386] { 1.0 } else { 0.0 };

        if s.b[386] {
            s.store_sub(363, 58, 203);
        }

        if (!s.b[386]) {
            s.store_sub(363, 204, 57);
        }

        s.b[394] = (p.p0 <= 300.0);
        s.v[394] = if s.b[394] { 1.0 } else { 0.0 };

        if s.b[394] {
            s.store_mul_sub_lhs(387, 363, 4, 5);
            s.store_add_scaled_product_right_ad(388, 4, 1.0, 4, A::add(s.ad_value(387), A::sqrt(A::offset(A::square(s.ad_value(387)), 1.921812))), 0.5);
        }

        if (!s.b[394]) {
            s.store_div(387, 363, 3);
            s.store_mul_scale_ad_rhs(388, 3, A::add(s.ad_value(387), A::sqrt(A::offset(A::square(s.ad_value(387)), p.p80))), 0.5);
        }

        s.store_div(389, 388, 55);

        s.store_mul(390, 388, 54);

        s.store_exp_scaled_input_ad(391, A::ln_one_plus_exp(A::scale(A::ln(s.ad_value(389)), p.p77)), 1.0 / (p.p77));

        s.store_div(392, 390, 391);

        s.store_scaled_sub(393, 388, 55, 1.0 / (p.p76));

        s.store_mul_offset_ad_rhs(362, 392, A::add_scaled_inputs(s.ad_value(393), 0.5, A::sqrt(A::offset(A::square(s.ad_value(393)), p.p81)), 0.5), 1.0);

        s.copy_ad(348, 352);

        s.b[395] = ((s.v[357] > 0.0) || (p.p85 > 0.0));
        s.v[395] = if s.b[395] { 1.0 } else { 0.0 };

        if s.b[395] {
            s.store_scale(396, 352, 0.5);
        }

        s.b[397] = (p.p0 <= 300.0);
        s.v[397] = if s.b[397] { 1.0 } else { 0.0 };

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
        s.v[398] = if s.b[398] { 1.0 } else { 0.0 };

        if s.b[398] {
            s.store_mul(359, 19, 59);
            s.store_mul(358, 359, 217);
        }

        if (!s.b[398]) {
            s.store_mul(358, 19, 355);
            s.store_mul(359, 19, 219);
        }

        s.v[354] = 0.0;

        s.b[399] = ((s.v[217] >= (1e-6 * s.v[362])) || (p.p0 >= 320.0));
        s.v[399] = if s.b[399] { 1.0 } else { 0.0 };

        if s.b[399] {
            s.store_div(96, 217, 362);
        }

    }

    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[399] {
            s.store_mul_ad_rhs(98, 61, A::exp_scaled_input(A::ln(s.ad_value(96)), p.p70));
            s.store_scaled_mul(97, 98, 217, 1.0 / ((1.0 + p.p70)));
        }

        s.b[400] = (p.p83 < (0.05 * (p.p75 / p.p74)));
        s.v[400] = if s.b[400] { 1.0 } else { 0.0 };

        if (s.b[399] && s.b[400]) {
            s.store_scalar(111, 0.0);
            s.store_scalar(112, 0.0);
        }

        if (s.b[399] && (!s.b[400])) {
            s.store_scaled_sub(107, 217, 362, 1.0 / (p.p83));
        }

        s.b[401] = (s.v[107] < (-10000000000.0));
        s.v[401] = if s.b[401] { 1.0 } else { 0.0 };

        if ((s.b[399] && (!s.b[400])) && s.b[401]) {
            s.store_scalar(107, (-10000000000.0));
        }

        if (s.b[399] && (!s.b[400])) {
            s.store_sqrt_square_offset(95, 107, p.p84);
            s.store_scaled_exp_ad(111, A::div_from_scalar((-2.0), A::add(s.ad_value(107), s.ad_value(95))), p.p82);
            s.store_div_scaled_inputs(112, s.ad_value(111), 2.0, A::mul_scaled_lhs(s.ad_value(95), p.p83, A::add(s.ad_value(107), s.ad_value(95))), 1.0);
        }

        if s.b[399] {
            s.store_mul_scaled_ad_rhs(99, 60, (1.0 - p.p73), A::offset(A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (-1.0)));
            s.store_add_ad_rhs(100, 99, A::mul3(A::mul3_scaled_output(s.ad_value(60), s.ad_value(217), A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (1.0 - p.p73)), s.ad_value(5), s.ad_value(112)));
            s.store_sub_from_scalar_ad(108, 1.0, A::div_from_scalar(1.0, s.ad_value(96)));
            s.store_scaled_add_sqrt_square_offset_rhs(109, 108, 108, p.p72, 1.0 / ((1.0 + (((1.0 + p.p72)) as f64).sqrt())));
            s.store_exp_ad(110, A::mul_offset_lhs(s.ad_value(111), (-p.p82), s.ad_value(5)));
            s.store_mul_product3_rhs(101, 110, s.ad_value(60), s.ad_value(109), s.ad_value(109), 1.0);
            s.store_mul_add_ad_rhs(102, 101, A::offset(A::div_from_scalar(2.0, A::mul(s.ad_value(96), A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)))), 1.0), A::mul3(s.ad_value(5), s.ad_value(217), s.ad_value(112)));
        }

        s.b[402] = ((((p.p115 < 0.01) && (p.p116 < 0.01)) && ((s.v[109] * p.p115) < 0.005)) && ((s.v[109] * p.p116) < 0.005));
        s.v[402] = if s.b[402] { 1.0 } else { 0.0 };

        if (s.b[399] && s.b[402]) {
            s.store_scaled_mul(105, 101, 217, p.p73);
            s.store_scale(106, 102, p.p73);
        }

        if (s.b[399] && (!s.b[402])) {
            s.store_sub_from_scalar(146, 1.0, 109);
            s.store_div_ad(147, A::mul_sub_from_scalar_rhs(A::offset(s.ad_value(146), (-1.0)), 1.0, s.ad_value(108)), A::mul(A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)), s.ad_value(217)));
        }

        s.b[403] = (((s.v[232]) as f64).abs() > 0.001);
        s.v[403] = if s.b[403] { 1.0 } else { 0.0 };

        if ((s.b[399] && (!s.b[402])) && s.b[403]) {
            s.store_exp_ad(151, A::mul_offset_lhs(s.ad_value(146), (-1.0), s.ad_value(231)));
        }

        s.b[404] = (s.v[229] < 0.01);
        s.v[404] = if s.b[404] { 1.0 } else { 0.0 };

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
            s.store_div_scaled_inputs2(154, s.ad_value(157), 1.0, s.ad_value(156), (-1.0), s.ad_value(232), 1.0);
            s.store_mul_product3_rhs(150, 147, A::div_scaled_inputs(s.ad_value(232), (-2.0), A::square(s.ad_value(152)), 1.0), s.ad_value(151), s.ad_value(231), 1.0);
            s.store_div_scaled_product_left_ad(155, A::sub(s.ad_value(159), s.ad_value(158)), 150, 1.0, 232, 1.0);
        }

        if ((s.b[399] && (!s.b[402])) && (!s.b[403])) {
            s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(146)), A::scale_offset(s.ad_value(146), p.p115, 1.0));
            s.store_offset_scaled(153, 149, p.p115, 1.0);
            s.store_div_scaled_product_offset_rhs(154, A::square(s.ad_value(149)), A::mul_scaled_lhs(s.ad_value(227), 2.0, s.ad_value(149)), 1.0, 1.0, s.ad_value(153), 1.0);
            s.store_div_scaled_product_denominator_ad(150, 147, 153, -1.0, A::scale_offset(s.ad_value(146), p.p115, 1.0), 1.0);
            s.store_mul_ad_product_lhs(155, s.ad_value(149), A::offset(A::div_from_scalar(1.0, A::square(s.ad_value(153))), 1.0), 150);
        }

        if (s.b[399] && (!s.b[402])) {
            s.store_scaled_mul(166, 60, 110, p.p73);
            s.store_mul(167, 166, 154);
            s.store_mul(105, 167, 217);
            s.store_add_scaled_inputs3(106, s.ad_value(167), 1.0, A::mul3(s.ad_value(105), s.ad_value(112), s.ad_value(5)), 1.0, A::mul3(s.ad_value(166), s.ad_value(217), s.ad_value(155)), 1.0);
        }

        if s.b[399] {
            s.store_scaled_mul(103, 101, 217, (1.0 - p.p73));
            s.store_scale(104, 102, (1.0 - p.p73));
            s.store_add_scaled_product_indices(354, 103, 1.0, 99, 217, 1.0);
        }

        s.b[405] = (p.p0 >= 310.0);
        s.v[405] = if s.b[405] { 1.0 } else { 0.0 };

        if (s.b[399] && s.b[405]) {
            s.store_add_scaled_inputs4(355, s.ad_value(355), 1.0, s.ad_value(354), 1.0, s.ad_value(97), 1.0, s.ad_value(105), 1.0);
            s.store_add_ad_lhs(219, A::add_scaled_inputs4(s.ad_value(219), 1.0, s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(98), 1.0), 106);
            s.store_add_scaled_inputs_products_indices(358, 358, 1.0, 354, p.p5, 20, 97, 1.0, 21, 105, 1.0);
            s.store_add_scaled_value_products(359, A::add_scaled_inputs3(s.ad_value(359), 1.0, s.ad_value(100), p.p5, s.ad_value(104), p.p5), 1.0, s.ad_value(20), s.ad_value(98), 1.0, s.ad_value(21), s.ad_value(106), 1.0);
        }

        if (s.b[399] && (!s.b[405])) {
            s.store_add_scaled_value_products3_indices(358, 354, 1.0, 19, 355, 1.0, 20, 97, 1.0, 21, 105, 1.0);
            s.store_add_scaled_inputs4(355, s.ad_value(355), 1.0, s.ad_value(354), 1.0, s.ad_value(97), 1.0, s.ad_value(105), 1.0);
            s.store_add_scaled_product_value_ad(359, A::add_scaled_inputs_products(s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(19), s.ad_value(219), 1.0, s.ad_value(20), s.ad_value(98), 1.0), 1.0, 21, 106, 1.0);
            s.store_add_ad_lhs(219, A::add_scaled_inputs4(s.ad_value(219), 1.0, s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(98), 1.0), 106);
        }

        s.store_scale(356, 218, p.p85);

        s.v[224] = 0.0;

        s.b[406] = (((p.p0 >= 310.0) && (s.v[358] > (1e-5 * s.v[348]))) || ((p.p0 <= 300.0) && (s.v[355] > (1e-5 * s.v[348]))));
        s.v[406] = if s.b[406] { 1.0 } else { 0.0 };

        if s.b[406] {
            s.store_sqrt_ad(355, A::mul3(s.ad_value(357), s.ad_value(217), s.ad_value(358)));
            s.store_add_scaled_inputs3(348, s.ad_value(352), 1.0, s.ad_value(355), 1.0, s.ad_value(356), p.p7);
            s.copy_ad(349, 348);
        }

    }

    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
    ) {
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
            s.v[408] = if s.b[408] { 1.0 } else { 0.0 };
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
            s.v[409] = if s.b[409] { 1.0 } else { 0.0 };
            if (s.b[406] && s.b[409]) {
                s.store_div(96, 217, 362);
                s.store_mul_ad_rhs(98, 61, A::exp_scaled_input(A::ln(s.ad_value(96)), p.p70));
                s.store_scaled_mul(97, 98, 217, 1.0 / ((1.0 + p.p70)));
            }
            s.b[410] = (p.p83 < (0.05 * (p.p75 / p.p74)));
            s.v[410] = if s.b[410] { 1.0 } else { 0.0 };
            if ((s.b[406] && s.b[409]) && s.b[410]) {
                s.store_scalar(111, 0.0);
                s.store_scalar(112, 0.0);
            }
            if ((s.b[406] && s.b[409]) && (!s.b[410])) {
                s.store_scaled_sub(107, 217, 362, 1.0 / (p.p83));
            }
            s.b[411] = (s.v[107] < (-10000000000.0));
            s.v[411] = if s.b[411] { 1.0 } else { 0.0 };
            if (((s.b[406] && s.b[409]) && (!s.b[410])) && s.b[411]) {
                s.store_scalar(107, (-10000000000.0));
            }
            if ((s.b[406] && s.b[409]) && (!s.b[410])) {
                s.store_sqrt_square_offset(95, 107, p.p84);
                s.store_scaled_exp_ad(111, A::div_from_scalar((-2.0), A::add(s.ad_value(107), s.ad_value(95))), p.p82);
                s.store_div_scaled_inputs(112, s.ad_value(111), 2.0, A::mul_scaled_lhs(s.ad_value(95), p.p83, A::add(s.ad_value(107), s.ad_value(95))), 1.0);
            }
            if (s.b[406] && s.b[409]) {
                s.store_mul_scaled_ad_rhs(99, 60, (1.0 - p.p73), A::offset(A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (-1.0)));
                s.store_add_ad_rhs(100, 99, A::mul3(A::mul3_scaled_output(s.ad_value(60), s.ad_value(217), A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (1.0 - p.p73)), s.ad_value(5), s.ad_value(112)));
                s.store_sub_from_scalar_ad(108, 1.0, A::div_from_scalar(1.0, s.ad_value(96)));
                s.store_scaled_add_sqrt_square_offset_rhs(109, 108, 108, p.p72, 1.0 / ((1.0 + (((1.0 + p.p72)) as f64).sqrt())));
                s.store_exp_ad(110, A::mul_offset_lhs(s.ad_value(111), (-p.p82), s.ad_value(5)));
                s.store_mul_product3_rhs(101, 110, s.ad_value(60), s.ad_value(109), s.ad_value(109), 1.0);
                s.store_mul_add_ad_rhs(102, 101, A::offset(A::div_from_scalar(2.0, A::mul(s.ad_value(96), A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)))), 1.0), A::mul3(s.ad_value(5), s.ad_value(217), s.ad_value(112)));
            }
            s.b[412] = ((((p.p115 < 0.01) && (p.p116 < 0.01)) && ((s.v[109] * p.p115) < 0.005)) && ((s.v[109] * p.p116) < 0.005));
            s.v[412] = if s.b[412] { 1.0 } else { 0.0 };
            if ((s.b[406] && s.b[409]) && s.b[412]) {
                s.store_scaled_mul(105, 101, 217, p.p73);
                s.store_scale(106, 102, p.p73);
            }
            if ((s.b[406] && s.b[409]) && (!s.b[412])) {
                s.store_sub_from_scalar(146, 1.0, 109);
                s.store_div_ad(147, A::mul_sub_from_scalar_rhs(A::offset(s.ad_value(146), (-1.0)), 1.0, s.ad_value(108)), A::mul(A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)), s.ad_value(217)));
            }
            s.b[413] = (((s.v[232]) as f64).abs() > 0.001);
            s.v[413] = if s.b[413] { 1.0 } else { 0.0 };
            if (((s.b[406] && s.b[409]) && (!s.b[412])) && s.b[413]) {
                s.store_exp_ad(151, A::mul_offset_lhs(s.ad_value(146), (-1.0), s.ad_value(231)));
            }
            s.b[414] = (s.v[229] < 0.01);
            s.v[414] = if s.b[414] { 1.0 } else { 0.0 };
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
                s.store_div_scaled_inputs2(154, s.ad_value(157), 1.0, s.ad_value(156), (-1.0), s.ad_value(232), 1.0);
                s.store_mul_product3_rhs(150, 147, A::div_scaled_inputs(s.ad_value(232), (-2.0), A::square(s.ad_value(152)), 1.0), s.ad_value(151), s.ad_value(231), 1.0);
                s.store_div_scaled_product_left_ad(155, A::sub(s.ad_value(159), s.ad_value(158)), 150, 1.0, 232, 1.0);
            }
            if (((s.b[406] && s.b[409]) && (!s.b[412])) && (!s.b[413])) {
                s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(146)), A::scale_offset(s.ad_value(146), p.p115, 1.0));
                s.store_offset_scaled(153, 149, p.p115, 1.0);
                s.store_div_scaled_product_offset_rhs(154, A::square(s.ad_value(149)), A::mul_scaled_lhs(s.ad_value(227), 2.0, s.ad_value(149)), 1.0, 1.0, s.ad_value(153), 1.0);
                s.store_div_scaled_product_denominator_ad(150, 147, 153, -1.0, A::scale_offset(s.ad_value(146), p.p115, 1.0), 1.0);
                s.store_mul_ad_product_lhs(155, s.ad_value(149), A::offset(A::div_from_scalar(1.0, A::square(s.ad_value(153))), 1.0), 150);
            }
            if ((s.b[406] && s.b[409]) && (!s.b[412])) {
                s.store_scaled_mul(166, 60, 110, p.p73);
                s.store_mul(167, 166, 154);
                s.store_mul(105, 167, 217);
                s.store_add_scaled_inputs3(106, s.ad_value(167), 1.0, A::mul3(s.ad_value(105), s.ad_value(112), s.ad_value(5)), 1.0, A::mul3(s.ad_value(166), s.ad_value(217), s.ad_value(155)), 1.0);
            }
            if (s.b[406] && s.b[409]) {
                s.store_scaled_mul(103, 101, 217, (1.0 - p.p73));
                s.store_scale(104, 102, (1.0 - p.p73));
                s.store_add_scaled_product_indices(354, 103, 1.0, 99, 217, 1.0);
            }
            s.b[415] = (p.p0 >= 310.0);
            s.v[415] = if s.b[415] { 1.0 } else { 0.0 };
            if ((s.b[406] && s.b[409]) && s.b[415]) {
                s.store_add_scaled_inputs4(355, s.ad_value(355), 1.0, s.ad_value(354), 1.0, s.ad_value(97), 1.0, s.ad_value(105), 1.0);
                s.store_add_ad_lhs(219, A::add_scaled_inputs4(s.ad_value(219), 1.0, s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(98), 1.0), 106);
                s.store_add_scaled_inputs_products_indices(358, 358, 1.0, 354, p.p5, 20, 97, 1.0, 21, 105, 1.0);
                s.store_add_scaled_value_products(359, A::add_scaled_inputs3(s.ad_value(359), 1.0, s.ad_value(100), p.p5, s.ad_value(104), p.p5), 1.0, s.ad_value(20), s.ad_value(98), 1.0, s.ad_value(21), s.ad_value(106), 1.0);
            }
            if ((s.b[406] && s.b[409]) && (!s.b[415])) {
                s.store_add_scaled_value_products3_indices(358, 354, 1.0, 19, 355, 1.0, 20, 97, 1.0, 21, 105, 1.0);
                s.store_add_scaled_inputs4(355, s.ad_value(355), 1.0, s.ad_value(354), 1.0, s.ad_value(97), 1.0, s.ad_value(105), 1.0);
                s.store_add_scaled_product_value_ad(359, A::add_scaled_inputs_products(s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(19), s.ad_value(219), 1.0, s.ad_value(20), s.ad_value(98), 1.0), 1.0, 21, 106, 1.0);
                s.store_add_ad_lhs(219, A::add_scaled_inputs4(s.ad_value(219), 1.0, s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(98), 1.0), 106);
            }
            if s.b[406] {
                s.store_scale(360, 218, (p.p7 * p.p85));
                s.store_div_scaled_inputs(349, A::add_scaled_inputs4(s.ad_value(348), 1.0, s.ad_value(352), -1.0, s.ad_value(358), -1.0, s.ad_value(360), -1.0), -1.0, A::offset(A::div_scaled_add_product(s.ad_value(360), 1.0, s.ad_value(359), s.ad_value(217), 1.0, s.ad_value(348), 1.0), 1.0), 1.0);
                s.store_abs_scaled_input(407, 348, 0.3);
            }
            s.b[416] = (((s.v[349]) as f64).abs() > s.v[407]);
            s.v[416] = if s.b[416] { 1.0 } else { 0.0 };
            s.b[417] = (s.v[349] >= 0.0);
            s.v[417] = if s.b[417] { 1.0 } else { 0.0 };
            if ((s.b[406] && s.b[416]) && s.b[417]) {
                s.copy_ad(349, 407);
            }
            if ((s.b[406] && s.b[416]) && (!s.b[417])) {
                s.store_neg(349, 407);
            }
            if s.b[406] {
                s.store_add(348, 348, 349);
            }
            let (assign6470_body87_e8091,) = {
    if s.b[406] {
        let assign6470_body87_e8089: f64 = (s.v[224] + 1.0);
        (assign6470_body87_e8089,)
    } else {
        (s.v[224],)
    }
};
            s.v[224] = assign6470_body87_e8091;
        }

        if s.b[406] {
            s.store_div(217, 350, 348);
            s.store_div(218, 351, 348);
            s.copy_ad(219, 357);
            s.store_mul(355, 357, 217);
        }

        s.b[418] = (p.p0 >= 310.0);
        s.v[418] = if s.b[418] { 1.0 } else { 0.0 };

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
        s.v[419] = if s.b[419] { 1.0 } else { 0.0 };

        if (s.b[406] && s.b[419]) {
            s.store_div(96, 217, 362);
            s.store_mul_ad_rhs(98, 61, A::exp_scaled_input(A::ln(s.ad_value(96)), p.p70));
            s.store_scaled_mul(97, 98, 217, 1.0 / ((1.0 + p.p70)));
        }

        s.b[420] = (p.p83 < (0.05 * (p.p75 / p.p74)));
        s.v[420] = if s.b[420] { 1.0 } else { 0.0 };

        if ((s.b[406] && s.b[419]) && s.b[420]) {
            s.store_scalar(111, 0.0);
            s.store_scalar(112, 0.0);
        }

        if ((s.b[406] && s.b[419]) && (!s.b[420])) {
            s.store_scaled_sub(107, 217, 362, 1.0 / (p.p83));
        }

        s.b[421] = (s.v[107] < (-10000000000.0));
        s.v[421] = if s.b[421] { 1.0 } else { 0.0 };

        if (((s.b[406] && s.b[419]) && (!s.b[420])) && s.b[421]) {
            s.store_scalar(107, (-10000000000.0));
        }

        if ((s.b[406] && s.b[419]) && (!s.b[420])) {
            s.store_sqrt_square_offset(95, 107, p.p84);
            s.store_scaled_exp_ad(111, A::div_from_scalar((-2.0), A::add(s.ad_value(107), s.ad_value(95))), p.p82);
            s.store_div_scaled_inputs(112, s.ad_value(111), 2.0, A::mul_scaled_lhs(s.ad_value(95), p.p83, A::add(s.ad_value(107), s.ad_value(95))), 1.0);
        }

        if (s.b[406] && s.b[419]) {
            s.store_mul_scaled_ad_rhs(99, 60, (1.0 - p.p73), A::offset(A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (-1.0)));
            s.store_add_ad_rhs(100, 99, A::mul3(A::mul3_scaled_output(s.ad_value(60), s.ad_value(217), A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (1.0 - p.p73)), s.ad_value(5), s.ad_value(112)));
            s.store_sub_from_scalar_ad(108, 1.0, A::div_from_scalar(1.0, s.ad_value(96)));
            s.store_scaled_add_sqrt_square_offset_rhs(109, 108, 108, p.p72, 1.0 / ((1.0 + (((1.0 + p.p72)) as f64).sqrt())));
            s.store_exp_ad(110, A::mul_offset_lhs(s.ad_value(111), (-p.p82), s.ad_value(5)));
            s.store_mul_product3_rhs(101, 110, s.ad_value(60), s.ad_value(109), s.ad_value(109), 1.0);
            s.store_mul_add_ad_rhs(102, 101, A::offset(A::div_from_scalar(2.0, A::mul(s.ad_value(96), A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)))), 1.0), A::mul3(s.ad_value(5), s.ad_value(217), s.ad_value(112)));
        }

        s.b[422] = ((((p.p115 < 0.01) && (p.p116 < 0.01)) && ((s.v[109] * p.p115) < 0.005)) && ((s.v[109] * p.p116) < 0.005));
        s.v[422] = if s.b[422] { 1.0 } else { 0.0 };

        if ((s.b[406] && s.b[419]) && s.b[422]) {
            s.store_scaled_mul(105, 101, 217, p.p73);
            s.store_scale(106, 102, p.p73);
        }

        if ((s.b[406] && s.b[419]) && (!s.b[422])) {
            s.store_sub_from_scalar(146, 1.0, 109);
            s.store_div_ad(147, A::mul_sub_from_scalar_rhs(A::offset(s.ad_value(146), (-1.0)), 1.0, s.ad_value(108)), A::mul(A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)), s.ad_value(217)));
        }

        s.b[423] = (((s.v[232]) as f64).abs() > 0.001);
        s.v[423] = if s.b[423] { 1.0 } else { 0.0 };

        if (((s.b[406] && s.b[419]) && (!s.b[422])) && s.b[423]) {
            s.store_exp_ad(151, A::mul_offset_lhs(s.ad_value(146), (-1.0), s.ad_value(231)));
        }

        s.b[424] = (s.v[229] < 0.01);
        s.v[424] = if s.b[424] { 1.0 } else { 0.0 };

        if ((((s.b[406] && s.b[419]) && (!s.b[422])) && s.b[423]) && s.b[424]) {
            s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(151)), A::mul(s.ad_value(151), s.ad_value(230)));
            s.store_offset_mul(148, 230, 149, 1.0);
            s.store_div_scaled_inputs2_by_product(154, A::mul3(s.ad_value(230), s.ad_value(149), A::offset(A::mul_scaled_lhs(s.ad_value(230), 0.25, s.ad_value(149)), 0.5)), 2.0, A::ln(s.ad_value(148)), (-(0.5 * 2.0)), s.ad_value(230), s.ad_value(230), 1.0);
            s.store_div_scaled_product_by_product(150, s.ad_value(231), s.ad_value(147), -1.0, s.ad_value(151), s.ad_value(230), 1.0);
            s.store_div_scaled_product3_mixed_aiii(155, A::offset(s.ad_value(148), 1.0), 149, 150, 1.0, 148, 1.0);
        }

        if ((((s.b[406] && s.b[419]) && (!s.b[422])) && s.b[423]) && (!s.b[424])) {
            s.store_sub_from_scalar_scaled_input(152, p.p116, 151, p.p115);
            s.store_div_scaled_offset_numerator(149, s.ad_value(151), 1.0, (-1.0), s.ad_value(152), 1.0);
        }

    }

    pub(super) fn stamp_transient_block_6(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((((s.b[406] && s.b[419]) && (!s.b[422])) && s.b[423]) && (!s.b[424])) {
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
            s.store_div_scaled_inputs2(154, s.ad_value(157), 1.0, s.ad_value(156), (-1.0), s.ad_value(232), 1.0);
            s.store_mul_product3_rhs(150, 147, A::div_scaled_inputs(s.ad_value(232), (-2.0), A::square(s.ad_value(152)), 1.0), s.ad_value(151), s.ad_value(231), 1.0);
            s.store_div_scaled_product_left_ad(155, A::sub(s.ad_value(159), s.ad_value(158)), 150, 1.0, 232, 1.0);
        }

        if (((s.b[406] && s.b[419]) && (!s.b[422])) && (!s.b[423])) {
            s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(146)), A::scale_offset(s.ad_value(146), p.p115, 1.0));
            s.store_offset_scaled(153, 149, p.p115, 1.0);
            s.store_div_scaled_product_offset_rhs(154, A::square(s.ad_value(149)), A::mul_scaled_lhs(s.ad_value(227), 2.0, s.ad_value(149)), 1.0, 1.0, s.ad_value(153), 1.0);
            s.store_div_scaled_product_denominator_ad(150, 147, 153, -1.0, A::scale_offset(s.ad_value(146), p.p115, 1.0), 1.0);
            s.store_mul_ad_product_lhs(155, s.ad_value(149), A::offset(A::div_from_scalar(1.0, A::square(s.ad_value(153))), 1.0), 150);
        }

        if ((s.b[406] && s.b[419]) && (!s.b[422])) {
            s.store_scaled_mul(166, 60, 110, p.p73);
            s.store_mul(167, 166, 154);
            s.store_mul(105, 167, 217);
            s.store_add_scaled_inputs3(106, s.ad_value(167), 1.0, A::mul3(s.ad_value(105), s.ad_value(112), s.ad_value(5)), 1.0, A::mul3(s.ad_value(166), s.ad_value(217), s.ad_value(155)), 1.0);
        }

        if (s.b[406] && s.b[419]) {
            s.store_scaled_mul(103, 101, 217, (1.0 - p.p73));
            s.store_scale(104, 102, (1.0 - p.p73));
            s.store_add_scaled_product_indices(354, 103, 1.0, 99, 217, 1.0);
        }

        s.b[425] = (p.p0 >= 310.0);
        s.v[425] = if s.b[425] { 1.0 } else { 0.0 };

        if ((s.b[406] && s.b[419]) && s.b[425]) {
            s.store_add_scaled_inputs4(355, s.ad_value(355), 1.0, s.ad_value(354), 1.0, s.ad_value(97), 1.0, s.ad_value(105), 1.0);
            s.store_add_ad_lhs(219, A::add_scaled_inputs4(s.ad_value(219), 1.0, s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(98), 1.0), 106);
            s.store_add_scaled_inputs_products_indices(358, 358, 1.0, 354, p.p5, 20, 97, 1.0, 21, 105, 1.0);
            s.store_add_scaled_value_products(359, A::add_scaled_inputs3(s.ad_value(359), 1.0, s.ad_value(100), p.p5, s.ad_value(104), p.p5), 1.0, s.ad_value(20), s.ad_value(98), 1.0, s.ad_value(21), s.ad_value(106), 1.0);
        }

        if ((s.b[406] && s.b[419]) && (!s.b[425])) {
            s.store_add_scaled_value_products3_indices(358, 354, 1.0, 19, 355, 1.0, 20, 97, 1.0, 21, 105, 1.0);
            s.store_add_scaled_inputs4(355, s.ad_value(355), 1.0, s.ad_value(354), 1.0, s.ad_value(97), 1.0, s.ad_value(105), 1.0);
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

        s.store_add_scaled_inputs4(222, s.ad_value(211), p.p93, s.ad_value(210), p.p93, s.ad_value(220), p.p93, s.ad_value(221), p.p93);

        s.store_mul_voltage_ad(183, s.ad_value(222), ctx, nodes, Some(7), Some(8));

        s.b[426] = (p.p23 > 0.0);
        s.v[426] = if s.b[426] { 1.0 } else { 0.0 };

        if s.b[426] {
            s.store_div_scaled_inputs(93, s.ad_value(203), 1.0, s.ad_value(4), p.p24);
        }

        s.b[427] = (s.v[93] > 80.0);
        s.v[427] = if s.b[427] { 1.0 } else { 0.0 };

        if (s.b[426] && s.b[427]) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
            s.store_scalar(93, 80.0);
        }

        if (s.b[426] && (!s.b[427])) {
            s.store_scalar(94, 1.0);
        }

        if s.b[426] {
            s.store_mul_offset_ad_rhs(187, 32, A::mul(s.ad_value(94), A::limexp(s.ad_value(93))), (-1.0));
        }

        if (!s.b[426]) {
            s.store_scalar(187, 0.0);
        }

        s.b[428] = ((p.p37 > 0.0) && (s.v[203] < 0.0));
        s.v[428] = if s.b[428] { 1.0 } else { 0.0 };

        s.b[429] = ((s.v[33] > 0.0) && (s.v[34] > 0.0));
        s.v[429] = if s.b[429] { 1.0 } else { 0.0 };

        if (s.b[428] && s.b[429]) {
            s.store_exp_scaled_input_ad(168, A::ln(A::div(s.ad_value(210), s.ad_value(33))), ((1.0 / p.p49) - 1.0));
            s.store_div_scaled_product_by_product(166, s.ad_value(67), s.ad_value(203), -1.0, s.ad_value(34), s.ad_value(168), 1.0);
            s.store_mul_exp_ad_rhs(193, 166, A::mul_scaled_lhs(s.ad_value(68), -1.0, s.ad_value(168)));
        }

        if (s.b[428] && (!s.b[429])) {
            s.store_scalar(193, 0.0);
        }

        if (!s.b[428]) {
            s.store_scalar(193, 0.0);
        }

        s.b[430] = (s.v[243] == 1.0);
        s.v[430] = if s.b[430] { 1.0 } else { 0.0 };

        if s.b[430] {
            s.store_sub(431, 34, 203);
        }

        s.b[437] = (s.v[431] > 0.0);
        s.v[437] = if s.b[437] { 1.0 } else { 0.0 };

        s.b[438] = (p.p35 > 0.0);
        s.v[438] = if s.b[438] { 1.0 } else { 0.0 };

        if ((s.b[430] && s.b[437]) && s.b[438]) {
            s.store_scalar(441, 0.1);
            s.store_div(440, 210, 33);
            s.store_add_scaled_product_indices(439, 217, p.p36, 55, 54, p.p35);
            s.store_sqrt_mul_ad(436, s.ad_value(441), A::ln(A::add_scaled_inputs(A::offset(A::exp(A::div(s.ad_value(440), s.ad_value(441))), (-2.0)), 1.0, A::cosh(A::div(A::sub_from_scalar(1.0, A::div(s.ad_value(217), s.ad_value(439))), s.ad_value(441))), 2.0)));
        }

        if ((s.b[430] && s.b[437]) && (!s.b[438])) {
            s.store_scalar(436, 1.0);
        }

        if (s.b[430] && s.b[437]) {
            s.store_div(432, 62, 210);
            s.store_div(433, 62, 33);
        }

        s.b[442] = (s.v[431] > s.v[433]);
        s.v[442] = if s.b[442] { 1.0 } else { 0.0 };

        if ((s.b[430] && s.b[437]) && s.b[442]) {
            s.store_mul_exp_ad_rhs(434, 63, A::div_scaled_inputs(s.ad_value(432), -1.0, A::mul(s.ad_value(433), s.ad_value(436)), 1.0));
            s.store_mul_ad_rhs(435, 434, A::add_scaled_offset_product_lhs(s.ad_value(433), 1.0, A::div(s.ad_value(432), s.ad_value(433)), 1.0, A::sub(s.ad_value(431), s.ad_value(433)), 1.0));
        }

        if ((s.b[430] && s.b[437]) && (!s.b[442])) {
            s.store_mul_ad_product_rhs(435, 63, s.ad_value(431), A::exp(A::div_scaled_inputs(s.ad_value(432), -1.0, A::mul(s.ad_value(431), s.ad_value(436)), 1.0)));
        }

        s.b[443] = (p.p34 > 0.0);
        s.v[443] = if s.b[443] { 1.0 } else { 0.0 };

        if ((s.b[430] && s.b[437]) && s.b[443]) {
            s.store_sub_from_scalar_scaled_input(444, 1.0, 435, p.p34);
            s.store_sqrt_square_offset(445, 444, 0.0001);
            s.store_scaled_add(446, 444, 445, 0.5);
            s.store_div_scaled_product_indices(244, 217, 435, 1.0, 446, 1.0);
        }

        if ((s.b[430] && s.b[437]) && (!s.b[443])) {
            s.store_mul(244, 217, 435);
        }

        if (s.b[430] && (!s.b[437])) {
            s.store_scalar(244, 0.0);
        }

        s.store_mul(190, 354, 175);

        s.b[447] = (s.v[69] > 0.0);
        s.v[447] = if s.b[447] { 1.0 } else { 0.0 };

        if s.b[447] {
            s.store_scale(449, 16, (1.0 + p.p92));
            s.store_add_scaled_inputs3(451, s.ad_value(179), 1.0, s.ad_value(178), 1.0, s.ad_value(355), 1.0);
            s.store_offset_div(448, 451, 449, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(452, 448, 448, 0.01, 0.5);
            s.store_div(70, 69, 452);
        }

        s.b[453] = (s.v[185] > 0.0);
        s.v[453] = if s.b[453] { 1.0 } else { 0.0 };

        if (s.b[447] && s.b[453]) {
            s.store_mul3_affine_lhs(450, 70, 185, p.p91, 0.0, 5);
        }

        s.b[454] = (s.v[450] < 1e-6);
        s.v[454] = if s.b[454] { 1.0 } else { 0.0 };

        if ((s.b[447] && s.b[453]) && s.b[454]) {
            s.store_mul_sub_from_scalar_ad_rhs(70, 70, 1.0, A::scale(s.ad_value(450), 0.5));
        }

        if ((s.b[447] && s.b[453]) && (!s.b[454])) {
            s.store_div_scaled_product_right_ad(70, 70, A::ln(A::offset(s.ad_value(450), 1.0)), 1.0, 450, 1.0);
        }

        s.b[455] = (s.v[355] > 0.0);
        s.v[455] = if s.b[455] { 1.0 } else { 0.0 };

        if (s.b[447] && s.b[455]) {
            s.store_div_scaled_product_mixed_iaa(70, 70, A::add_scaled_inputs(s.ad_value(179), 1.0, s.ad_value(355), p.p94), 1.0, A::add(s.ad_value(179), s.ad_value(355)), 1.0);
        }

        if (!s.b[447]) {
            s.store_scalar(70, 0.0);
        }

        s.b[456] = (p.p18 > 0.0);
        s.v[456] = if s.b[456] { 1.0 } else { 0.0 };

        if s.b[456] {
            s.store_div_scaled_inputs(93, s.ad_value(205), 1.0, s.ad_value(4), p.p19);
        }

        s.b[457] = (s.v[93] > 80.0);
        s.v[457] = if s.b[457] { 1.0 } else { 0.0 };

        if (s.b[456] && s.b[457]) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
            s.store_scalar(93, 80.0);
        }

        if (s.b[456] && (!s.b[457])) {
            s.store_scalar(94, 1.0);
        }

        if s.b[456] {
            s.store_mul_offset_ad_rhs(188, 23, A::mul(s.ad_value(94), A::limexp(s.ad_value(93))), (-1.0));
        }

        if (!s.b[456]) {
            s.store_scalar(188, 0.0);
        }

        s.b[458] = (p.p20 > 0.0);
        s.v[458] = if s.b[458] { 1.0 } else { 0.0 };

        if s.b[458] {
            s.store_div_scaled_inputs(93, s.ad_value(205), 1.0, s.ad_value(4), p.p21);
        }

        s.b[459] = (s.v[93] > 80.0);
        s.v[459] = if s.b[459] { 1.0 } else { 0.0 };

        if (s.b[458] && s.b[459]) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
            s.store_scalar(93, 80.0);
        }

        if (s.b[458] && (!s.b[459])) {
            s.store_scalar(94, 1.0);
        }

        if s.b[458] {
            s.store_mul_offset_ad_rhs(189, 25, A::mul(s.ad_value(94), A::limexp(s.ad_value(93))), (-1.0));
        }

        if (!s.b[458]) {
            s.store_scalar(189, 0.0);
        }

        s.b[460] = (s.v[29] > 0.0);
        s.v[460] = if s.b[460] { 1.0 } else { 0.0 };

        if s.b[460] {
            s.store_mul_sub_from_scalar_ad_rhs(137, 30, 1.0, A::exp_scaled_input(A::ln(s.ad_value(31)), (-1.0 / (p.p45))));
            s.store_mul_sub_lhs(141, 137, 205, 5);
            s.store_sqrt_square_offset(142, 141, 1.921812);
            s.store_scaled_add(143, 141, 142, 0.5);
            s.store_add_scaled_product_indices(138, 137, 1.0, 4, 143, (-1.0));
            s.store_div(144, 143, 142);
            s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(30))));
            s.store_mul_ad_lhs(145, A::exp_scaled_input(s.ad_value(139), (-p.p45)), 144);
            s.store_mul_add_ad_rhs(212, 29, s.ad_value(145), A::mul_sub_from_scalar_rhs(s.ad_value(31), 1.0, s.ad_value(144)));
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(140, 30, 1.0, A::exp_scaled_input(s.ad_value(139), (1.0 - p.p45)), 1.0 / ((1.0 - p.p45)));
            s.store_mul_add_scaled_product_rhs(180, 29, s.ad_value(140), 1.0, s.ad_value(31), A::sub(s.ad_value(205), s.ad_value(138)), 1.0);
        }

        if (!s.b[460]) {
            s.store_scalar(212, 0.0);
            s.store_scalar(180, 0.0);
        }

        s.b[461] = ((p.p27 > 0.0) && ((s.v[205] < s.v[223]) || (s.v[202] < s.v[223])));
        s.v[461] = if s.b[461] { 1.0 } else { 0.0 };

        s.b[464] = (((p.p29 == 1.0) && (s.v[29] > 0.0)) && (s.v[30] > 0.0));
        s.v[464] = if s.b[464] { 1.0 } else { 0.0 };

        if (s.b[461] && s.b[464]) {
            s.store_exp_scaled_input_ad(462, A::ln(A::div(s.ad_value(212), s.ad_value(29))), (1.0 - (1.0 / p.p45)));
            s.store_mul_ad_affine_product_lhs(463, A::div(s.ad_value(205), s.ad_value(30)), s.ad_value(64), -1.0, 0.0, 462);
            s.store_mul_exp_ad_rhs(191, 463, A::div_scaled_inputs(s.ad_value(65), -1.0, s.ad_value(462), 1.0));
        }

        s.b[465] = (((p.p29 == 0.0) && (s.v[26] > 0.0)) && (s.v[27] > 0.0));
        s.v[465] = if s.b[465] { 1.0 } else { 0.0 };

        if ((s.b[461] && (!s.b[464])) && s.b[465]) {
            s.store_exp_scaled_input_ad(462, A::ln(A::div(s.ad_value(211), s.ad_value(26))), (1.0 - (1.0 / p.p41)));
            s.store_mul_ad_affine_product_lhs(463, A::div(s.ad_value(202), s.ad_value(27)), s.ad_value(64), -1.0, 0.0, 462);
            s.store_mul_exp_ad_rhs(191, 463, A::div_scaled_inputs(s.ad_value(65), -1.0, s.ad_value(462), 1.0));
        }

        if ((s.b[461] && (!s.b[464])) && (!s.b[465])) {
            s.store_scalar(191, 0.0);
        }

        if (!s.b[461]) {
            s.store_scalar(191, 0.0);
        }

        s.store_mul_offset_ad_rhs(192, 66, A::exp_scaled_input(s.ad_value(202), 1.0 / (p.p31)), (-1.0));

        s.b[466] = (p.p56 < 100.0);
        s.v[466] = if s.b[466] { 1.0 } else { 0.0 };

        s.b[467] = (s.v[38] > 0.0);
        s.v[467] = if s.b[467] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[466] && s.b[467]) {
            s.store_scalar(113, (p.p54 / 4.0));
            s.store_sub_from_scalar(114, p.p56, 39);
            s.store_mul_sub_from_scalar_ad_rhs(115, 39, 1.0, A::exp_scaled_input(A::ln(s.ad_value(40)), (-1.0 / (p.p54))));
            s.store_mul(116, 40, 38);
            s.store_mul_exp_ad_rhs(117, 38, A::mul_offset_lhs(s.ad_value(113), (-p.p54), A::ln(A::div_from_scalar(p.p56, s.ad_value(39)))));
            s.store_mul_sub_lhs(119, 115, 206, 5);
        }

        s.b[468] = (s.v[119] < 80.0);
        s.v[468] = if s.b[468] { 1.0 } else { 0.0 };

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
            s.store_div_scaled_inputs2(123, s.ad_value(114), 1.0, s.ad_value(122), 1.0, s.ad_value(118), 1.0);
        }

        s.b[469] = (s.v[123] < 80.0);
        s.v[469] = if s.b[469] { 1.0 } else { 0.0 };

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
            s.store_mul_product3_rhs(134, 124, s.ad_value(38), A::exp_scaled_input(s.ad_value(131), (-p.p54)), s.ad_value(121), 1.0);
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
        s.v[470] = if s.b[470] { 1.0 } else { 0.0 };

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
        s.v[471] = if s.b[471] { 1.0 } else { 0.0 };

        if s.b[471] {
            s.store_div_scaled_inputs(93, s.ad_value(206), 1.0, s.ad_value(4), p.p26);
        }

        s.b[472] = (s.v[93] > 80.0);
        s.v[472] = if s.b[472] { 1.0 } else { 0.0 };

        if (s.b[471] && s.b[472]) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
            s.store_scalar(93, 80.0);
        }

        if (s.b[471] && (!s.b[472])) {
            s.store_scalar(94, 1.0);
        }

        if s.b[471] {
            s.store_mul_offset_ad_rhs(194, 36, A::mul(s.ad_value(94), A::limexp(s.ad_value(93))), (-1.0));
        }

        if (!s.b[471]) {
            s.store_scalar(194, 0.0);
        }

        s.b[473] = (p.p56 < 100.0);
        s.v[473] = if s.b[473] { 1.0 } else { 0.0 };

        s.b[474] = (s.v[37] > 0.0);
        s.v[474] = if s.b[474] { 1.0 } else { 0.0 };

        if (s.b[473] && s.b[474]) {
            s.store_scalar(113, (p.p54 / 4.0));
            s.store_sub_from_scalar(114, p.p56, 39);
            s.store_mul_sub_from_scalar_ad_rhs(115, 39, 1.0, A::exp_scaled_input(A::ln(s.ad_value(40)), (-1.0 / (p.p54))));
            s.store_mul(116, 40, 37);
            s.store_mul_exp_ad_rhs(117, 37, A::mul_offset_lhs(s.ad_value(113), (-p.p54), A::ln(A::div_from_scalar(p.p56, s.ad_value(39)))));
            s.store_mul_sub_lhs(119, 115, 207, 5);
        }

        s.b[475] = (s.v[119] < 80.0);
        s.v[475] = if s.b[475] { 1.0 } else { 0.0 };

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
            s.store_div_scaled_inputs2(123, s.ad_value(114), 1.0, s.ad_value(122), 1.0, s.ad_value(118), 1.0);
        }

        s.b[476] = (s.v[123] < 80.0);
        s.v[476] = if s.b[476] { 1.0 } else { 0.0 };

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
            s.store_mul_product3_rhs(134, 124, s.ad_value(37), A::exp_scaled_input(s.ad_value(131), (-p.p54)), s.ad_value(121), 1.0);
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
        s.v[477] = if s.b[477] { 1.0 } else { 0.0 };

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
        s.v[478] = if s.b[478] { 1.0 } else { 0.0 };

        s.b[479] = (s.v[46] > 0.0);
        s.v[479] = if s.b[479] { 1.0 } else { 0.0 };

        if (s.b[478] && s.b[479]) {
            s.store_scalar(113, (p.p59 / 4.0));
            s.store_sub_from_scalar(114, p.p61, 47);
            s.store_mul_sub_from_scalar_ad_rhs(115, 47, 1.0, A::exp_scaled_input(A::ln(s.ad_value(48)), (-1.0 / (p.p59))));
            s.store_mul(116, 48, 46);
            s.store_mul_exp_ad_rhs(117, 46, A::mul_offset_lhs(s.ad_value(113), (-p.p59), A::ln(A::div_from_scalar(p.p61, s.ad_value(47)))));
            s.store_mul_sub_lhs(119, 115, 208, 5);
        }

        s.b[480] = (s.v[119] < 80.0);
        s.v[480] = if s.b[480] { 1.0 } else { 0.0 };

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
            s.store_div_scaled_inputs2(123, s.ad_value(114), 1.0, s.ad_value(122), 1.0, s.ad_value(118), 1.0);
        }

        s.b[481] = (s.v[123] < 80.0);
        s.v[481] = if s.b[481] { 1.0 } else { 0.0 };

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
            s.store_mul_product3_rhs(134, 124, s.ad_value(46), A::exp_scaled_input(s.ad_value(131), (-p.p59)), s.ad_value(121), 1.0);
            s.store_mul_ad_product_rhs(135, 117, A::exp(A::mul_scaled_rhs(s.ad_value(130), s.ad_value(113), -1.0)), A::sub_from_scalar(1.0, s.ad_value(124)));
            s.store_mul_sub_from_scalar_rhs(136, 116, 1.0, 121);
            s.store_div_ad_lhs(127, A::mul_sub_from_scalar_rhs(s.ad_value(46), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132)))), 132);
        }

    }

    pub(super) fn stamp_transient_block_8(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[478] && s.b[479]) {
            s.store_div_ad_lhs(128, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133)))), 133);
            s.store_div_ad_lhs(129, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133)))), 133);
            s.store_add_scaled_products_left_left_ad(196, A::add_scaled_inputs3(s.ad_value(127), 1.0, s.ad_value(128), 1.0, s.ad_value(129), -1.0), 47, 1.0, 116, 126, 1.0);
        }

        if (s.b[478] && (!s.b[479])) {
            s.store_scalar(196, 0.0);
        }

        s.b[482] = (s.v[46] > 0.0);
        s.v[482] = if s.b[482] { 1.0 } else { 0.0 };

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
        s.v[483] = if s.b[483] { 1.0 } else { 0.0 };

        s.b[484] = (p.p65 < 100.0);
        s.v[484] = if s.b[484] { 1.0 } else { 0.0 };

        s.b[485] = (s.v[49] > 0.0);
        s.v[485] = if s.b[485] { 1.0 } else { 0.0 };

        if ((s.b[483] && s.b[484]) && s.b[485]) {
            s.store_scalar(113, (p.p64 / 4.0));
            s.store_sub_from_scalar(114, p.p65, 50);
            s.store_mul_sub_from_scalar_ad_rhs(115, 50, 1.0, A::exp_scaled_input(A::ln(s.ad_value(51)), (-1.0 / (p.p64))));
            s.store_mul(116, 51, 49);
            s.store_mul_exp_ad_rhs(117, 49, A::mul_offset_lhs(s.ad_value(113), (-p.p64), A::ln(A::div_from_scalar(p.p65, s.ad_value(50)))));
            s.store_mul_sub_lhs(119, 115, 209, 5);
        }

        s.b[486] = (s.v[119] < 80.0);
        s.v[486] = if s.b[486] { 1.0 } else { 0.0 };

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
            s.store_div_scaled_inputs2(123, s.ad_value(114), 1.0, s.ad_value(122), 1.0, s.ad_value(118), 1.0);
        }

        s.b[487] = (s.v[123] < 80.0);
        s.v[487] = if s.b[487] { 1.0 } else { 0.0 };

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
            s.store_mul_product3_rhs(134, 124, s.ad_value(49), A::exp_scaled_input(s.ad_value(131), (-p.p64)), s.ad_value(121), 1.0);
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
        s.v[488] = if s.b[488] { 1.0 } else { 0.0 };

        if ((s.b[483] && (!s.b[484])) && s.b[488]) {
            s.store_mul_sub_from_scalar_ad_rhs(137, 50, 1.0, A::exp_scaled_input(A::ln(s.ad_value(51)), (-1.0 / (p.p64))));
            s.store_mul_sub_lhs(141, 137, 209, 5);
            s.store_sqrt_square_offset(142, 141, 1.921812);
            s.store_scaled_add(143, 141, 142, 0.5);
            s.store_add_scaled_product_indices(138, 137, 1.0, 4, 143, (-1.0));
            s.store_div(144, 143, 142);
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
        s.v[489] = if s.b[489] { 1.0 } else { 0.0 };

        if s.b[489] {
            s.store_scale(490, 4, p.p98);
            s.store_limexp_div(491, 206, 490);
            s.store_limexp_div(492, 208, 490);
            s.store_mul_sub_rhs(198, 44, 491, 492);
        }

        s.b[493] = (p.p101 > 0.0);
        s.v[493] = if s.b[493] { 1.0 } else { 0.0 };

        if (s.b[489] && s.b[493]) {
            s.store_mul3_lhs(199, 52, 44, 491);
        }

        if (s.b[489] && (!s.b[493])) {
            s.store_scalar(199, 0.0);
        }

        if (!s.b[489]) {
            s.store_scalar(198, 0.0);
            s.store_scalar(199, 0.0);
        }

        s.b[494] = (p.p99 > 0.0);
        s.v[494] = if s.b[494] { 1.0 } else { 0.0 };

        if s.b[494] {
            s.store_div_scaled_inputs(93, s.ad_value(208), 1.0, s.ad_value(4), p.p100);
        }

        s.b[495] = (s.v[93] > 80.0);
        s.v[495] = if s.b[495] { 1.0 } else { 0.0 };

        if (s.b[494] && s.b[495]) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
            s.store_scalar(93, 80.0);
        }

        if (s.b[494] && (!s.b[495])) {
            s.store_scalar(94, 1.0);
        }

        if s.b[494] {
            s.store_mul_offset_ad_rhs(195, 45, A::mul(s.ad_value(94), A::limexp(s.ad_value(93))), (-1.0));
        }

        if (!s.b[494]) {
            s.store_scalar(195, 0.0);
        }

        s.b[496] = ((p.p142 >= p.p149) && (p.p142 > 0.0));
        s.v[496] = if s.b[496] { 1.0 } else { 0.0 };

        s.b[497] = (p.p141 == 1.0);
        s.v[497] = if s.b[497] { 1.0 } else { 0.0 };

        if (s.b[496] && s.b[497]) {
            s.store_add_scaled_products_right_left_ad(200, 204, 184, 1.0, A::sub(s.ad_value(34), s.ad_value(203)), 244, 1.0);
        }

        s.b[498] = (p.p141 == 2.0);
        s.v[498] = if s.b[498] { 1.0 } else { 0.0 };

        if ((s.b[496] && (!s.b[497])) && s.b[498]) {
            s.store_add_scaled_product_value_ad(200, A::add_scaled_value_products3(A::add_scaled_products3(s.ad_value(204), s.ad_value(184), 1.0, A::sub(s.ad_value(34), s.ad_value(203)), s.ad_value(244), 1.0, s.ad_value(185), s.ad_value(202), 1.0), 1.0, s.ad_value(187), s.ad_value(203), 1.0, s.ad_value(188), s.ad_value(205), 1.0, s.ad_value(194), s.ad_value(206), 1.0), 1.0, 195, 208, 1.0);
        }

        s.b[499] = ((s.v[70] >= p.p149) && (s.v[70] > 0.0));
        s.v[499] = if s.b[499] { 1.0 } else { 0.0 };

        if (((s.b[496] && (!s.b[497])) && s.b[498]) && s.b[499]) {
            s.store_add_ad_rhs(200, 200, A::div_scaled_product(A::voltage(ctx, nodes, Some(7), Some(8)), A::voltage(ctx, nodes, Some(7), Some(8)), 1.0, s.ad_value(70), 1.0));
        }

        s.b[500] = ((s.v[73] >= p.p149) && (s.v[73] > 0.0));
        s.v[500] = if s.b[500] { 1.0 } else { 0.0 };

        if (((s.b[496] && (!s.b[497])) && s.b[498]) && s.b[500]) {
            s.store_add_ad_rhs(200, 200, A::div_scaled_product(A::voltage(ctx, nodes, Some(6), Some(2)), A::voltage(ctx, nodes, Some(6), Some(2)), 1.0, s.ad_value(73), 1.0));
        }

        s.b[501] = ((s.v[72] >= p.p149) && (s.v[72] > 0.0));
        s.v[501] = if s.b[501] { 1.0 } else { 0.0 };

        if (((s.b[496] && (!s.b[497])) && s.b[498]) && s.b[501]) {
            s.store_add_ad_rhs(200, 200, A::div_scaled_product(A::voltage(ctx, nodes, Some(5), Some(0)), A::voltage(ctx, nodes, Some(5), Some(0)), 1.0, s.ad_value(72), 1.0));
        }

        s.b[502] = ((s.v[71] >= p.p149) && (s.v[71] > 0.0));
        s.v[502] = if s.b[502] { 1.0 } else { 0.0 };

        if (((s.b[496] && (!s.b[497])) && s.b[498]) && s.b[502]) {
            s.store_add_ad_rhs(200, 200, A::div_scaled_product(A::voltage(ctx, nodes, Some(1), Some(7)), A::voltage(ctx, nodes, Some(1), Some(7)), 1.0, s.ad_value(71), 1.0));
        }

        if ((s.b[496] && (!s.b[497])) && (!s.b[498])) {
            s.store_scalar(200, 0.0);
        }

        s.copy_ad(241, 217);

        s.copy_ad(242, 181);

        s.b[507] = (s.v[234] != 0.0);
        s.v[507] = if s.b[507] { 1.0 } else { 0.0 };

        if s.b[507] {
            s.store_voltage(504, ctx, nodes, Some(10), None);
            s.store_voltage(505, ctx, nodes, Some(11), None);
            s.store_scale_ad(237, A::div_scaled_inputs2(s.ad_value(505), 1.0, s.ad_value(217), (-1.0), s.ad_value(219), 1.0), p.p66);
            s.store_scale_ad(238, A::div_scaled_inputs2(s.ad_value(505), 1.0, s.ad_value(504), (-1.0), s.ad_value(219), 1.0), p.p66);
            s.store_scale(239, 504, (p.p88 * p.p66));
            s.store_scale(240, 505, ((p.p88 * 0.3333333333333333) * p.p66));
            s.copy_ad(241, 505);
            s.store_voltage(503, ctx, nodes, Some(12), None);
            s.store_div_from_scalar(506, p.p66, 219);
            s.store_mul_sub_lhs(235, 503, 181, 506);
            s.store_scale(236, 503, (p.p87 * p.p66));
            s.copy_ad(242, 503);
        }

        if (!s.b[507]) {
            s.store_voltage(237, ctx, nodes, Some(10), None);
            s.store_voltage(238, ctx, nodes, Some(11), None);
            s.store_scalar(239, 0.0);
            s.store_scalar(240, 0.0);
            s.store_voltage(235, ctx, nodes, Some(12), None);
            s.store_scalar(236, 0.0);
        }

        s.b[508] = ((p.p89 >= p.p149) && (p.p89 > 0.0));
        s.v[508] = if s.b[508] { 1.0 } else { 0.0 };

        s.b[509] = (p.p93 > 0.0);
        s.v[509] = if s.b[509] { 1.0 } else { 0.0 };

        s.b[510] = (p.p29 == 1.0);
        s.v[510] = if s.b[510] { 1.0 } else { 0.0 };

        s.b[511] = ((p.p90 >= p.p149) && (p.p90 > 0.0));
        s.v[511] = if s.b[511] { 1.0 } else { 0.0 };

        s.b[512] = ((p.p95 >= p.p149) && (p.p95 > 0.0));
        s.v[512] = if s.b[512] { 1.0 } else { 0.0 };

        s.b[513] = ((p.p96 >= p.p149) && (p.p96 > 0.0));
        s.v[513] = if s.b[513] { 1.0 } else { 0.0 };

        s.b[514] = (p.p0 >= 320.0);
        s.v[514] = if s.b[514] { 1.0 } else { 0.0 };

        s.b[515] = (p.p99 > 0.0);
        s.v[515] = if s.b[515] { 1.0 } else { 0.0 };

        s.b[517] = ((p.p102 >= p.p149) && (p.p102 > 0.0));
        s.v[517] = if s.b[517] { 1.0 } else { 0.0 };

        s.b[518] = (p.p103 > 0.0);
        s.v[518] = if s.b[518] { 1.0 } else { 0.0 };

        s.b[519] = (((p.p141 >= 1.0) && (p.p142 >= p.p149)) && (p.p142 > 0.0));
        s.v[519] = if s.b[519] { 1.0 } else { 0.0 };

        s.b[520] = (p.p145 > 0.0);
        s.v[520] = if s.b[520] { 1.0 } else { 0.0 };

        s.b[533] = ((p.p109 == 1.0) && ((p.p88 > 0.0) && (p.p87 > 0.0)));
        s.v[533] = if s.b[533] { 1.0 } else { 0.0 };

        s.b[539] = (s.v[185] > 0.0);
        s.v[539] = if s.b[539] { 1.0 } else { 0.0 };

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

    }

    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
    ) {
        s.b[540] = (s.v[538] > 0.0);
        s.v[540] = if s.b[540] { 1.0 } else { 0.0 };

        if (s.b[533] && s.b[540]) {
            s.store_mul_sqrt_rhs(537, 219, 538);
        }

        if (s.b[533] && (!s.b[540])) {
            s.store_scalar(537, 0.0);
        }

    }

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
        s.v[279] = if s.b[279] { 1.0 } else { 0.0 };

        if s.b[279] {
            s.store_scalar(0, 1.6021918e-19);
            s.store_scalar(1, 1.3806226e-23);
        }

        if (!s.b[279]) {
            s.store_scalar(0, 1.602176634e-19);
            s.store_scalar(1, 1.380649e-23);
        }

        s.v[8] = (p.p146 + 273.15);

        s.v[9] = ctx_temp;

        s.store_div(2, 1, 0);

        s.store_scale(3, 2, 300.0);

        s.store_scale(6, 2, s.v[8]);

        s.store_div_from_scalar(7, 1.0, 6);

        s.v[276] = ((p.p121 * s.v[8]) * ((s.v[8]) as f64).ln());

        s.v[277] = (p.p122 * s.v[8]);

        s.v[56] = (p.p131 * s.v[8]);

        s.v[88] = ((p.p117 + s.v[276]) + s.v[277]);

        s.v[89] = ((p.p118 + s.v[276]) + s.v[277]);

        s.v[90] = ((p.p119 + s.v[276]) + s.v[277]);

        s.v[91] = ((s.v[88] + s.v[89]) * 0.5);

        s.v[92] = ((s.v[88] + s.v[90]) * 0.5);

        s.v[77] = ((p.p117 + p.p118) * 0.5);

        s.v[78] = ((p.p117 + p.p119) * 0.5);

        s.v[79] = ((p.p120 + p.p119) * 0.5);

        s.store_sub_from_scalar_ad(76, 3.0, A::div_from_scalar(p.p121, s.ad_value(2)));

        s.store_offset(82, 76, (-1.5));

        s.v[278] = ((1.0 - p.p107) * (p.p52 + p.p106));

        s.b[280] = (s.v[278] >= p.p106);
        s.v[280] = if s.b[280] { 1.0 } else { 0.0 };

        if s.b[280] {
            s.store_scalar(171, p.p106);
            s.store_scalar(172, 0.0);
            s.store_scalar(176, (s.v[278] - p.p106));
            s.store_sub_from_scalar(177, p.p52, 176);
        }

        if (!s.b[280]) {
            s.store_scalar(171, s.v[278]);
            s.store_sub_from_scalar(172, p.p106, 171);
            s.store_scalar(176, 0.0);
            s.store_scalar(177, p.p52);
        }

        s.v[174] = (p.p105 * p.p104);

        s.v[173] = (p.p104 - s.v[174]);

        s.b[282] = (p.p0 <= 300.0);
        s.v[282] = if s.b[282] { 1.0 } else { 0.0 };

        if s.b[282] {
            s.store_scalar(223, 0.0);
        }

        if (!s.b[282]) {
            s.store_scalar(223, 0.7);
        }

        s.v[234] = p.p86;

        s.b[284] = (p.p86 != 0.0);
        s.v[284] = if s.b[284] { 1.0 } else { 0.0 };

        s.b[285] = (((p.p88 == 0.0) && (p.p87 == 0.0)) || (p.p66 == 0.0));
        s.v[285] = if s.b[285] { 1.0 } else { 0.0 };

        if (s.b[284] && s.b[285]) {
            s.store_scalar(234, 0.0);
        }

        s.b[286] = ((p.p115 >= 0.01) || (p.p116 >= 0.01));
        s.v[286] = if s.b[286] { 1.0 } else { 0.0 };

        if s.b[286] {
            s.store_scalar(232, (0.5 * (p.p115 - p.p116)));
        }

        s.b[287] = (p.p116 < p.p115);
        s.v[287] = if s.b[287] { 1.0 } else { 0.0 };

        if (s.b[286] && s.b[287]) {
            s.store_scalar(229, p.p116);
            s.store_scalar(230, p.p115);
        }

        if (s.b[286] && (!s.b[287])) {
            s.store_scalar(229, p.p115);
            s.store_scalar(230, p.p116);
        }

        s.b[288] = (s.v[229] < 0.01);
        s.v[288] = if s.b[288] { 1.0 } else { 0.0 };

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

        s.v[10] = (s.v[9] + p.p147);

        s.b[289] = (s.v[10] < ((-200.0) + 273.15));
        s.v[289] = if s.b[289] { 1.0 } else { 0.0 };

        if s.b[289] {
            s.store_scalar(10, ((-200.0) + 273.15));
        }

        s.b[290] = (s.v[10] > (326.85 + 273.15));
        s.v[290] = if s.b[290] { 1.0 } else { 0.0 };

        if ((!s.b[289]) && s.b[290]) {
            s.store_scalar(10, (326.85 + 273.15));
        }

        s.store_mul(4, 2, 10);

        s.store_div_from_scalar(5, 1.0, 4);

        s.store_offset(14, 10, (-s.v[8]));

        s.store_div_from_scalar(12, s.v[8], 10);

        s.store_scale(11, 10, 1.0 / (s.v[8]));

        s.store_ln(13, 11);

        s.store_mul_scaled_ad_rhs(74, 10, p.p121, A::ln(s.ad_value(10)));

        s.store_scale(75, 10, p.p122);

        s.store_add_ad_lhs(84, A::offset(s.ad_value(74), p.p117), 75);

        s.store_add_ad_lhs(83, A::offset(s.ad_value(74), p.p118), 75);

        s.store_add_ad_lhs(85, A::offset(s.ad_value(74), p.p119), 75);

        s.store_scaled_add(86, 84, 83, 0.5);

        s.store_scaled_add(87, 84, 85, 0.5);

        s.b[291] = (p.p39 > 0.0);
        s.v[291] = if s.b[291] { 1.0 } else { 0.0 };

        if s.b[291] {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p40 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p40)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[77]), s.v[77]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(27, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(26, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p40, s.ad_value(27))), p.p41), p.p39);
            s.store_scalar(28, ((p.p42) as f64).abs());
        }

        s.b[292] = (p.p42 > 0.0);
        s.v[292] = if s.b[292] { 1.0 } else { 0.0 };

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
        s.v[293] = if s.b[293] { 1.0 } else { 0.0 };

        if s.b[293] {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p48 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p48)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[78]), s.v[78]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(34, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(33, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p48, s.ad_value(34))), p.p49), p.p47);
            s.store_scalar(35, ((p.p50) as f64).abs());
        }

        s.b[294] = (p.p50 > 0.0);
        s.v[294] = if s.b[294] { 1.0 } else { 0.0 };

        if (s.b[293] && s.b[294]) {
            s.store_scale(35, 34, (p.p50 * 1.0 / (p.p48)));
        }

        if (!s.b[293]) {
            s.store_scalar(33, p.p47);
            s.store_scalar(34, p.p48);
            s.store_scalar(35, p.p50);
        }

        s.b[295] = (p.p0 <= 300.0);
        s.v[295] = if s.b[295] { 1.0 } else { 0.0 };

        if s.b[295] {
            s.store_scalar(35, 2.4);
        }

        s.store_offset_scaled_ad(16, A::exp_scaled_input(A::ln_scaled_input(s.ad_value(27), 1.0 / (p.p40)), p.p41), (-p.p2), ((2.0) * (p.p2)));

        s.store_scaled_exp_ad(15, A::add_scaled_inputs(s.ad_value(13), p.p123, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p.p117), 1.0), p.p1);

        s.store_scaled_exp_scaled_input(18, 13, p.p126, p.p10);

        s.b[296] = ((p.p0 <= 300.0) && ((((p.p8 - 1.0)) as f64).abs() < 1e-5));
        s.v[296] = if s.b[296] { 1.0 } else { 0.0 };

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
        s.v[297] = if s.b[297] { 1.0 } else { 0.0 };

        if s.b[297] {
            s.store_offset_scaled_ad(58, A::scale(s.ad_value(14), p.p133), (-p.p79), p.p79);
            s.store_scalar(57, p.p78);
        }

        if (!s.b[297]) {
            s.store_offset_scaled(57, 14, ((p.p132) * (p.p78)), p.p78);
            s.store_scalar(58, p.p79);
        }

        s.store_add_scaled_product_value_ad(59, A::scale_offset(s.ad_value(14), p.p128, 1.0), p.p66, 14, 14, (p.p129 * p.p66));

        s.v[61] = p.p69;

        s.store_scaled_exp_scaled_input(60, 13, (p.p130 - 1.0), p.p71);

        s.b[299] = ((p.p37 > 0.0) && (s.v[203] < 0.0));
        s.v[299] = if s.b[299] { 1.0 } else { 0.0 };

        if s.b[299] {
            s.store_scalar(67, p.p37);
        }

        s.b[300] = ((p.p47 > 0.0) && (p.p48 > 0.0));
        s.v[300] = if s.b[300] { 1.0 } else { 0.0 };

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
        s.v[301] = if s.b[301] { 1.0 } else { 0.0 };

        if s.b[301] {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p44 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p44)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[77]), s.v[77]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(30, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(29, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p44, s.ad_value(30))), p.p45), p.p43);
            s.store_scalar(31, ((p.p46) as f64).abs());
        }

        s.b[302] = (p.p46 > 0.0);
        s.v[302] = if s.b[302] { 1.0 } else { 0.0 };

        if (s.b[301] && s.b[302]) {
            s.store_scale(31, 30, (p.p46 * 1.0 / (p.p44)));
        }

        if (!s.b[301]) {
            s.store_scalar(29, p.p43);
            s.store_scalar(30, p.p44);
            s.store_scalar(31, p.p46);
        }

        s.b[303] = ((p.p27 > 0.0) && ((s.v[205] < s.v[223]) || (s.v[202] < s.v[223])));
        s.v[303] = if s.b[303] { 1.0 } else { 0.0 };

        if s.b[303] {
            s.store_scalar(166, 1.0);
            s.store_scalar(167, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[303] {
            s.store_div_from_scalar(169, s.v[91], 86);
        }

        s.b[304] = (((p.p29 == 1.0) && (p.p43 > 0.0)) && (p.p44 > 0.0));
        s.v[304] = if s.b[304] { 1.0 } else { 0.0 };

        if (s.b[303] && s.b[304]) {
            s.store_scale(170, 30, 1.0 / (p.p44));
            s.store_mul_product3_rhs(167, 170, s.ad_value(29), A::sqrt(s.ad_value(169)), s.ad_value(170), 1.0 / (p.p43));
            s.store_div_scaled_value_by_product(166, A::powf(s.ad_value(169), (-1.5)), p.p43, s.ad_value(29), s.ad_value(170), 1.0);
        }

        s.b[305] = (((p.p29 == 0.0) && (p.p39 > 0.0)) && (p.p40 > 0.0));
        s.v[305] = if s.b[305] { 1.0 } else { 0.0 };

        if ((s.b[303] && (!s.b[304])) && s.b[305]) {
            s.store_scale(170, 27, 1.0 / (p.p40));
            s.store_mul_product3_rhs(167, 170, s.ad_value(26), A::sqrt(s.ad_value(169)), s.ad_value(170), 1.0 / (p.p39));
            s.store_div_scaled_value_by_product(166, A::powf(s.ad_value(169), (-1.5)), p.p39, s.ad_value(26), s.ad_value(170), 1.0);
        }

        s.b[306] = (1.0 > 0.0);
        s.v[306] = if s.b[306] { 1.0 } else { 0.0 };

        if s.b[306] {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p53 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p53)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[78]), s.v[78]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(39, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_exp_scaled_input_ad(43, A::ln(A::div_from_scalar(p.p53, s.ad_value(39))), p.p54);
            s.store_scalar(40, ((p.p55) as f64).abs());
        }

        s.b[307] = (p.p55 > 0.0);
        s.v[307] = if s.b[307] { 1.0 } else { 0.0 };

        if (s.b[306] && s.b[307]) {
            s.store_scale(40, 39, (p.p55 * 1.0 / (p.p53)));
        }

        if (!s.b[306]) {
            s.store_scalar(43, 1.0);
            s.store_scalar(39, p.p53);
            s.store_scalar(40, p.p55);
        }

        s.b[308] = (p.p0 <= 300.0);
        s.v[308] = if s.b[308] { 1.0 } else { 0.0 };

        if s.b[308] {
            s.store_scalar(40, 2.4);
        }

        s.store_mul(37, 43, 176);

        s.store_mul(38, 43, 177);

        s.b[309] = (p.p0 <= 300.0);
        s.v[309] = if s.b[309] { 1.0 } else { 0.0 };

        s.b[310] = (p.p57 > 0.0);
        s.v[310] = if s.b[310] { 1.0 } else { 0.0 };

        if (s.b[309] && s.b[310]) {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p58 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p58)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(47, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(46, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p58, s.ad_value(47))), p.p59), p.p57);
            s.store_scalar(48, (((-2.4)) as f64).abs());
        }

        s.b[311] = ((-2.4) > 0.0);
        s.v[311] = if s.b[311] { 1.0 } else { 0.0 };

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
        s.v[312] = if s.b[312] { 1.0 } else { 0.0 };

        if ((!s.b[309]) && s.b[312]) {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p58 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p58)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(47, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(46, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p58, s.ad_value(47))), p.p59), p.p57);
            s.store_scalar(48, (((-p.p60)) as f64).abs());
        }

        s.b[313] = ((-p.p60) > 0.0);
        s.v[313] = if s.b[313] { 1.0 } else { 0.0 };

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
        s.v[314] = if s.b[314] { 1.0 } else { 0.0 };

        s.b[315] = (p.p62 > 0.0);
        s.v[315] = if s.b[315] { 1.0 } else { 0.0 };

        if (s.b[314] && s.b[315]) {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p63 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p63)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(50, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(49, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p63, s.ad_value(50))), p.p64), p.p62);
            s.store_abs_scaled_input(51, 163, -1.0);
        }

        s.b[316] = ((-s.v[163]) > 0.0);
        s.v[316] = if s.b[316] { 1.0 } else { 0.0 };

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
        s.v[317] = if s.b[317] { 1.0 } else { 0.0 };

        if s.b[317] {
            s.store_offset_voltage(10, ctx, nodes, Some(4), None, (s.v[9] + p.p147));
        }

        s.b[318] = (s.v[10] < ((-200.0) + 273.15));
        s.v[318] = if s.b[318] { 1.0 } else { 0.0 };

        if (s.b[317] && s.b[318]) {
            s.store_scalar(10, ((-200.0) + 273.15));
        }

        s.b[319] = (s.v[10] > (326.85 + 273.15));
        s.v[319] = if s.b[319] { 1.0 } else { 0.0 };

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
            s.store_mul_scaled_ad_rhs(74, 10, p.p121, A::ln(s.ad_value(10)));
            s.store_scale(75, 10, p.p122);
            s.store_add_ad_lhs(84, A::offset(s.ad_value(74), p.p117), 75);
            s.store_add_ad_lhs(83, A::offset(s.ad_value(74), p.p118), 75);
            s.store_add_ad_lhs(85, A::offset(s.ad_value(74), p.p119), 75);
            s.store_scaled_add(86, 84, 83, 0.5);
            s.store_scaled_add(87, 84, 85, 0.5);
        }

        s.b[320] = (p.p39 > 0.0);
        s.v[320] = if s.b[320] { 1.0 } else { 0.0 };

        if (s.b[317] && s.b[320]) {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p40 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p40)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[77]), s.v[77]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(27, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(26, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p40, s.ad_value(27))), p.p41), p.p39);
            s.store_scalar(28, ((p.p42) as f64).abs());
        }

        s.b[321] = (p.p42 > 0.0);
        s.v[321] = if s.b[321] { 1.0 } else { 0.0 };

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
        s.v[322] = if s.b[322] { 1.0 } else { 0.0 };

        if (s.b[317] && s.b[322]) {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p48 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p48)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[78]), s.v[78]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(34, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(33, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p48, s.ad_value(34))), p.p49), p.p47);
            s.store_scalar(35, ((p.p50) as f64).abs());
        }

        s.b[323] = (p.p50 > 0.0);
        s.v[323] = if s.b[323] { 1.0 } else { 0.0 };

        if ((s.b[317] && s.b[322]) && s.b[323]) {
            s.store_scale(35, 34, (p.p50 * 1.0 / (p.p48)));
        }

        if (s.b[317] && (!s.b[322])) {
            s.store_scalar(33, p.p47);
            s.store_scalar(34, p.p48);
            s.store_scalar(35, p.p50);
        }

        s.b[324] = (p.p0 <= 300.0);
        s.v[324] = if s.b[324] { 1.0 } else { 0.0 };

        if (s.b[317] && s.b[324]) {
            s.store_scalar(35, 2.4);
        }

        if s.b[317] {
            s.store_offset_scaled_ad(16, A::exp_scaled_input(A::ln_scaled_input(s.ad_value(27), 1.0 / (p.p40)), p.p41), (-p.p2), ((2.0) * (p.p2)));
            s.store_scaled_exp_ad(15, A::add_scaled_inputs(s.ad_value(13), p.p123, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p.p117), 1.0), p.p1);
            s.store_scaled_exp_scaled_input(18, 13, p.p126, p.p10);
        }

        s.b[325] = ((p.p0 <= 300.0) && ((((p.p8 - 1.0)) as f64).abs() < 1e-5));
        s.v[325] = if s.b[325] { 1.0 } else { 0.0 };

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
        s.v[326] = if s.b[326] { 1.0 } else { 0.0 };

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
        s.v[328] = if s.b[328] { 1.0 } else { 0.0 };

        if (s.b[317] && s.b[328]) {
            s.store_scalar(67, p.p37);
        }

        s.b[329] = ((p.p47 > 0.0) && (p.p48 > 0.0));
        s.v[329] = if s.b[329] { 1.0 } else { 0.0 };

        if ((s.b[317] && s.b[328]) && s.b[329]) {
            s.store_div_from_scalar(169, s.v[92], 87);
            s.store_scale(170, 34, 1.0 / (p.p48));
            s.store_mul_ad_affine_product_lhs(168, A::sqrt(s.ad_value(169)), s.ad_value(170), 1.0 / (p.p47), 0.0, 33);
        }

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[317] && s.b[328]) && s.b[329]) {
            s.store_scaled_mul(67, 168, 170, p.p37);
        }

        if (s.b[317] && (!s.b[328])) {
            s.store_scalar(67, 0.0);
        }

        s.b[330] = (p.p43 > 0.0);
        s.v[330] = if s.b[330] { 1.0 } else { 0.0 };

        if (s.b[317] && s.b[330]) {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p44 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p44)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[77]), s.v[77]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(30, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(29, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p44, s.ad_value(30))), p.p45), p.p43);
            s.store_scalar(31, ((p.p46) as f64).abs());
        }

        s.b[331] = (p.p46 > 0.0);
        s.v[331] = if s.b[331] { 1.0 } else { 0.0 };

        if ((s.b[317] && s.b[330]) && s.b[331]) {
            s.store_scale(31, 30, (p.p46 * 1.0 / (p.p44)));
        }

        if (s.b[317] && (!s.b[330])) {
            s.store_scalar(29, p.p43);
            s.store_scalar(30, p.p44);
            s.store_scalar(31, p.p46);
        }

        s.b[332] = ((p.p27 > 0.0) && ((s.v[205] < s.v[223]) || (s.v[202] < s.v[223])));
        s.v[332] = if s.b[332] { 1.0 } else { 0.0 };

        if (s.b[317] && s.b[332]) {
            s.store_scalar(166, 1.0);
            s.store_scalar(167, 1.0);
            s.store_div_from_scalar(169, s.v[91], 86);
        }

        s.b[333] = (((p.p29 == 1.0) && (p.p43 > 0.0)) && (p.p44 > 0.0));
        s.v[333] = if s.b[333] { 1.0 } else { 0.0 };

        if ((s.b[317] && s.b[332]) && s.b[333]) {
            s.store_scale(170, 30, 1.0 / (p.p44));
            s.store_mul_product3_rhs(167, 170, s.ad_value(29), A::sqrt(s.ad_value(169)), s.ad_value(170), 1.0 / (p.p43));
            s.store_div_scaled_value_by_product(166, A::powf(s.ad_value(169), (-1.5)), p.p43, s.ad_value(29), s.ad_value(170), 1.0);
        }

        s.b[334] = (((p.p29 == 0.0) && (p.p39 > 0.0)) && (p.p40 > 0.0));
        s.v[334] = if s.b[334] { 1.0 } else { 0.0 };

        if (((s.b[317] && s.b[332]) && (!s.b[333])) && s.b[334]) {
            s.store_scale(170, 27, 1.0 / (p.p40));
            s.store_mul_product3_rhs(167, 170, s.ad_value(26), A::sqrt(s.ad_value(169)), s.ad_value(170), 1.0 / (p.p39));
            s.store_div_scaled_value_by_product(166, A::powf(s.ad_value(169), (-1.5)), p.p39, s.ad_value(26), s.ad_value(170), 1.0);
        }

        s.b[335] = (1.0 > 0.0);
        s.v[335] = if s.b[335] { 1.0 } else { 0.0 };

        if (s.b[317] && s.b[335]) {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p53 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p53)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[78]), s.v[78]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(39, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_exp_scaled_input_ad(43, A::ln(A::div_from_scalar(p.p53, s.ad_value(39))), p.p54);
            s.store_scalar(40, ((p.p55) as f64).abs());
        }

        s.b[336] = (p.p55 > 0.0);
        s.v[336] = if s.b[336] { 1.0 } else { 0.0 };

        if ((s.b[317] && s.b[335]) && s.b[336]) {
            s.store_scale(40, 39, (p.p55 * 1.0 / (p.p53)));
        }

        if (s.b[317] && (!s.b[335])) {
            s.store_scalar(43, 1.0);
            s.store_scalar(39, p.p53);
            s.store_scalar(40, p.p55);
        }

        s.b[337] = (p.p0 <= 300.0);
        s.v[337] = if s.b[337] { 1.0 } else { 0.0 };

        if (s.b[317] && s.b[337]) {
            s.store_scalar(40, 2.4);
        }

        if s.b[317] {
            s.store_mul(37, 43, 176);
            s.store_mul(38, 43, 177);
        }

        s.b[338] = (p.p0 <= 300.0);
        s.v[338] = if s.b[338] { 1.0 } else { 0.0 };

        s.b[339] = (p.p57 > 0.0);
        s.v[339] = if s.b[339] { 1.0 } else { 0.0 };

        if ((s.b[317] && s.b[338]) && s.b[339]) {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p58 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p58)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(47, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(46, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p58, s.ad_value(47))), p.p59), p.p57);
            s.store_scalar(48, (((-2.4)) as f64).abs());
        }

        s.b[340] = ((-2.4) > 0.0);
        s.v[340] = if s.b[340] { 1.0 } else { 0.0 };

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
        s.v[341] = if s.b[341] { 1.0 } else { 0.0 };

        if ((s.b[317] && (!s.b[338])) && s.b[341]) {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p58 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p58)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(47, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(46, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p58, s.ad_value(47))), p.p59), p.p57);
            s.store_scalar(48, (((-p.p60)) as f64).abs());
        }

        s.b[342] = ((-p.p60) > 0.0);
        s.v[342] = if s.b[342] { 1.0 } else { 0.0 };

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
        s.v[343] = if s.b[343] { 1.0 } else { 0.0 };

        s.b[344] = (p.p62 > 0.0);
        s.v[344] = if s.b[344] { 1.0 } else { 0.0 };

        if ((s.b[317] && s.b[343]) && s.b[344]) {
            s.store_mul_scaled_ad_rhs(164, 6, 2.0, A::ln(A::sub(A::exp_scaled_input(s.ad_value(7), (p.p63 * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p.p63)))));
            s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));
            s.store_add_scaled_product_right_ad(50, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(49, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p63, s.ad_value(50))), p.p64), p.p62);
            s.store_abs_scaled_input(51, 163, -1.0);
        }

        s.b[345] = ((-s.v[163]) > 0.0);
        s.v[345] = if s.b[345] { 1.0 } else { 0.0 };

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
        s.v[364] = if s.b[364] { 1.0 } else { 0.0 };

        if s.b[364] {
            s.store_div_scaled_inputs(93, s.ad_value(202), 1.0, s.ad_value(4), p.p15);
        }

        s.b[365] = (s.v[93] > 80.0);
        s.v[365] = if s.b[365] { 1.0 } else { 0.0 };

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
        s.v[366] = if s.b[366] { 1.0 } else { 0.0 };

        if s.b[366] {
            s.store_div_scaled_inputs(93, s.ad_value(202), 1.0, s.ad_value(4), p.p17);
        }

        s.b[367] = (s.v[93] > 80.0);
        s.v[367] = if s.b[367] { 1.0 } else { 0.0 };

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
        s.v[368] = if s.b[368] { 1.0 } else { 0.0 };

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
        s.v[369] = if s.b[369] { 1.0 } else { 0.0 };

        s.b[370] = (s.v[33] > 0.0);
        s.v[370] = if s.b[370] { 1.0 } else { 0.0 };

        if (s.b[369] && s.b[370]) {
            s.store_scalar(113, (p.p49 / 4.0));
            s.store_sub_from_scalar(114, p.p51, 34);
            s.store_mul_sub_from_scalar_ad_rhs(115, 34, 1.0, A::exp_scaled_input(A::ln(s.ad_value(35)), (-1.0 / (p.p49))));
            s.store_mul(116, 35, 33);
            s.store_mul_exp_ad_rhs(117, 33, A::mul_offset_lhs(s.ad_value(113), (-p.p49), A::ln(A::div_from_scalar(p.p51, s.ad_value(34)))));
            s.store_mul_sub_lhs(119, 115, 203, 5);
        }

        s.b[371] = (s.v[119] < 80.0);
        s.v[371] = if s.b[371] { 1.0 } else { 0.0 };

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
            s.store_div_scaled_inputs2(123, s.ad_value(114), 1.0, s.ad_value(122), 1.0, s.ad_value(118), 1.0);
        }

        s.b[372] = (s.v[123] < 80.0);
        s.v[372] = if s.b[372] { 1.0 } else { 0.0 };

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
        }

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[369] && s.b[370]) {
            s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(34))));
            s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(34))));
            s.store_scalar(132, (1.0 - p.p49));
            s.store_sub_from_scalar(133, 1.0, 113);
            s.store_mul_product3_rhs(134, 124, s.ad_value(33), A::exp_scaled_input(s.ad_value(131), (-p.p49)), s.ad_value(121), 1.0);
            s.store_mul_ad_product_rhs(135, 117, A::exp(A::mul_scaled_rhs(s.ad_value(130), s.ad_value(113), -1.0)), A::sub_from_scalar(1.0, s.ad_value(124)));
            s.store_mul_sub_from_scalar_rhs(136, 116, 1.0, 121);
            s.store_add_scaled_inputs3(210, s.ad_value(134), 1.0, s.ad_value(135), 1.0, s.ad_value(136), 1.0);
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
        s.v[373] = if s.b[373] { 1.0 } else { 0.0 };

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
        s.v[374] = if s.b[374] { 1.0 } else { 0.0 };

        if s.b[374] {
            s.store_scale(375, 4, p.p11);
            s.store_div_scaled_inputs2(376, s.ad_value(27), 1.0, s.ad_value(202), (-1.0), s.ad_value(375), 1.0);
            s.store_add_scaled_product_right_ad(377, 27, 1.0, 375, A::add(s.ad_value(376), A::sqrt(A::offset(A::square(s.ad_value(376)), 1.921812))), (-0.5));
            s.store_mul_sub_from_scalar_ad_rhs(378, 18, 1.0, A::exp_scaled_input(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(377), s.ad_value(27)))), p.p41));
        }

        s.b[379] = (((s.v[378]) as f64).abs() > 0.001);
        s.v[379] = if s.b[379] { 1.0 } else { 0.0 };

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

        s.store_mul_offset_ad_rhs(352, 353, A::add_scaled_inputs(s.ad_value(347), 0.5, A::sqrt(A::offset(A::square(s.ad_value(347)), 1.921812)), 0.5), 1.0);

        s.store_scale(380, 34, (1.0 - ((((-((2.4) as f64).ln()) / p.p49)) as f64).exp()));

        s.store_mul_sub_lhs(381, 380, 203, 5);

        s.store_sqrt_square_offset(382, 381, 1.921812);

        s.store_scaled_add(383, 381, 382, 0.5);

        s.store_add_scaled_product_indices(384, 380, 1.0, 4, 383, (-1.0));

        s.store_div(385, 383, 382);

        s.store_add_scaled_product_mixed_aai(361, A::scale_offset(s.ad_value(385), (-2.4), 2.4), 1.0, A::exp_scaled_input(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(384), s.ad_value(34)))), (-p.p49)), 385, 1.0);

        s.store_add_scaled_inputs3_offset(357, s.ad_value(59), 1.0, A::div_from_scalar(1.0, s.ad_value(361)), p.p67, s.ad_value(361), p.p68, (((-1.0) * p.p67) + ((-1.0) * p.p68)));

        s.b[386] = (p.p79 > 0.0);
        s.v[386] = if s.b[386] { 1.0 } else { 0.0 };

        if s.b[386] {
            s.store_sub(363, 58, 203);
        }

        if (!s.b[386]) {
            s.store_sub(363, 204, 57);
        }

        s.b[394] = (p.p0 <= 300.0);
        s.v[394] = if s.b[394] { 1.0 } else { 0.0 };

        if s.b[394] {
            s.store_mul_sub_lhs(387, 363, 4, 5);
            s.store_add_scaled_product_right_ad(388, 4, 1.0, 4, A::add(s.ad_value(387), A::sqrt(A::offset(A::square(s.ad_value(387)), 1.921812))), 0.5);
        }

        if (!s.b[394]) {
            s.store_div(387, 363, 3);
            s.store_mul_scale_ad_rhs(388, 3, A::add(s.ad_value(387), A::sqrt(A::offset(A::square(s.ad_value(387)), p.p80))), 0.5);
        }

        s.store_div(389, 388, 55);

        s.store_mul(390, 388, 54);

        s.store_exp_scaled_input_ad(391, A::ln_one_plus_exp(A::scale(A::ln(s.ad_value(389)), p.p77)), 1.0 / (p.p77));

        s.store_div(392, 390, 391);

        s.store_scaled_sub(393, 388, 55, 1.0 / (p.p76));

        s.store_mul_offset_ad_rhs(362, 392, A::add_scaled_inputs(s.ad_value(393), 0.5, A::sqrt(A::offset(A::square(s.ad_value(393)), p.p81)), 0.5), 1.0);

        s.copy_ad(348, 352);

        s.b[395] = ((s.v[357] > 0.0) || (p.p85 > 0.0));
        s.v[395] = if s.b[395] { 1.0 } else { 0.0 };

        if s.b[395] {
            s.store_scale(396, 352, 0.5);
        }

        s.b[397] = (p.p0 <= 300.0);
        s.v[397] = if s.b[397] { 1.0 } else { 0.0 };

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
        s.v[398] = if s.b[398] { 1.0 } else { 0.0 };

        if s.b[398] {
            s.store_mul(359, 19, 59);
            s.store_mul(358, 359, 217);
        }

        if (!s.b[398]) {
            s.store_mul(358, 19, 355);
            s.store_mul(359, 19, 219);
        }

        s.v[354] = 0.0;

        s.b[399] = ((s.v[217] >= (1e-6 * s.v[362])) || (p.p0 >= 320.0));
        s.v[399] = if s.b[399] { 1.0 } else { 0.0 };

        if s.b[399] {
            s.store_div(96, 217, 362);
            s.store_mul_ad_rhs(98, 61, A::exp_scaled_input(A::ln(s.ad_value(96)), p.p70));
            s.store_scaled_mul(97, 98, 217, 1.0 / ((1.0 + p.p70)));
        }

        s.b[400] = (p.p83 < (0.05 * (p.p75 / p.p74)));
        s.v[400] = if s.b[400] { 1.0 } else { 0.0 };

        if (s.b[399] && s.b[400]) {
            s.store_scalar(111, 0.0);
            s.store_scalar(112, 0.0);
        }

        if (s.b[399] && (!s.b[400])) {
            s.store_scaled_sub(107, 217, 362, 1.0 / (p.p83));
        }

        s.b[401] = (s.v[107] < (-10000000000.0));
        s.v[401] = if s.b[401] { 1.0 } else { 0.0 };

        if ((s.b[399] && (!s.b[400])) && s.b[401]) {
            s.store_scalar(107, (-10000000000.0));
        }

        if (s.b[399] && (!s.b[400])) {
            s.store_sqrt_square_offset(95, 107, p.p84);
            s.store_scaled_exp_ad(111, A::div_from_scalar((-2.0), A::add(s.ad_value(107), s.ad_value(95))), p.p82);
            s.store_div_scaled_inputs(112, s.ad_value(111), 2.0, A::mul_scaled_lhs(s.ad_value(95), p.p83, A::add(s.ad_value(107), s.ad_value(95))), 1.0);
        }

        if s.b[399] {
            s.store_mul_scaled_ad_rhs(99, 60, (1.0 - p.p73), A::offset(A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (-1.0)));
            s.store_add_ad_rhs(100, 99, A::mul3(A::mul3_scaled_output(s.ad_value(60), s.ad_value(217), A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (1.0 - p.p73)), s.ad_value(5), s.ad_value(112)));
            s.store_sub_from_scalar_ad(108, 1.0, A::div_from_scalar(1.0, s.ad_value(96)));
            s.store_scaled_add_sqrt_square_offset_rhs(109, 108, 108, p.p72, 1.0 / ((1.0 + (((1.0 + p.p72)) as f64).sqrt())));
            s.store_exp_ad(110, A::mul_offset_lhs(s.ad_value(111), (-p.p82), s.ad_value(5)));
            s.store_mul_product3_rhs(101, 110, s.ad_value(60), s.ad_value(109), s.ad_value(109), 1.0);
            s.store_mul_add_ad_rhs(102, 101, A::offset(A::div_from_scalar(2.0, A::mul(s.ad_value(96), A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)))), 1.0), A::mul3(s.ad_value(5), s.ad_value(217), s.ad_value(112)));
        }

        s.b[402] = ((((p.p115 < 0.01) && (p.p116 < 0.01)) && ((s.v[109] * p.p115) < 0.005)) && ((s.v[109] * p.p116) < 0.005));
        s.v[402] = if s.b[402] { 1.0 } else { 0.0 };

        if (s.b[399] && s.b[402]) {
            s.store_scaled_mul(105, 101, 217, p.p73);
            s.store_scale(106, 102, p.p73);
        }

        if (s.b[399] && (!s.b[402])) {
            s.store_sub_from_scalar(146, 1.0, 109);
            s.store_div_ad(147, A::mul_sub_from_scalar_rhs(A::offset(s.ad_value(146), (-1.0)), 1.0, s.ad_value(108)), A::mul(A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)), s.ad_value(217)));
        }

        s.b[403] = (((s.v[232]) as f64).abs() > 0.001);
        s.v[403] = if s.b[403] { 1.0 } else { 0.0 };

        if ((s.b[399] && (!s.b[402])) && s.b[403]) {
            s.store_exp_ad(151, A::mul_offset_lhs(s.ad_value(146), (-1.0), s.ad_value(231)));
        }

        s.b[404] = (s.v[229] < 0.01);
        s.v[404] = if s.b[404] { 1.0 } else { 0.0 };

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
            s.store_div_scaled_inputs2(154, s.ad_value(157), 1.0, s.ad_value(156), (-1.0), s.ad_value(232), 1.0);
            s.store_mul_product3_rhs(150, 147, A::div_scaled_inputs(s.ad_value(232), (-2.0), A::square(s.ad_value(152)), 1.0), s.ad_value(151), s.ad_value(231), 1.0);
            s.store_div_scaled_product_left_ad(155, A::sub(s.ad_value(159), s.ad_value(158)), 150, 1.0, 232, 1.0);
        }

        if ((s.b[399] && (!s.b[402])) && (!s.b[403])) {
            s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(146)), A::scale_offset(s.ad_value(146), p.p115, 1.0));
            s.store_offset_scaled(153, 149, p.p115, 1.0);
            s.store_div_scaled_product_offset_rhs(154, A::square(s.ad_value(149)), A::mul_scaled_lhs(s.ad_value(227), 2.0, s.ad_value(149)), 1.0, 1.0, s.ad_value(153), 1.0);
            s.store_div_scaled_product_denominator_ad(150, 147, 153, -1.0, A::scale_offset(s.ad_value(146), p.p115, 1.0), 1.0);
            s.store_mul_ad_product_lhs(155, s.ad_value(149), A::offset(A::div_from_scalar(1.0, A::square(s.ad_value(153))), 1.0), 150);
        }

        if (s.b[399] && (!s.b[402])) {
            s.store_scaled_mul(166, 60, 110, p.p73);
            s.store_mul(167, 166, 154);
            s.store_mul(105, 167, 217);
            s.store_add_scaled_inputs3(106, s.ad_value(167), 1.0, A::mul3(s.ad_value(105), s.ad_value(112), s.ad_value(5)), 1.0, A::mul3(s.ad_value(166), s.ad_value(217), s.ad_value(155)), 1.0);
        }

        if s.b[399] {
            s.store_scaled_mul(103, 101, 217, (1.0 - p.p73));
            s.store_scale(104, 102, (1.0 - p.p73));
            s.store_add_scaled_product_indices(354, 103, 1.0, 99, 217, 1.0);
        }

        s.b[405] = (p.p0 >= 310.0);
        s.v[405] = if s.b[405] { 1.0 } else { 0.0 };

        if (s.b[399] && s.b[405]) {
            s.store_add_scaled_inputs4(355, s.ad_value(355), 1.0, s.ad_value(354), 1.0, s.ad_value(97), 1.0, s.ad_value(105), 1.0);
            s.store_add_ad_lhs(219, A::add_scaled_inputs4(s.ad_value(219), 1.0, s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(98), 1.0), 106);
            s.store_add_scaled_inputs_products_indices(358, 358, 1.0, 354, p.p5, 20, 97, 1.0, 21, 105, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[399] && s.b[405]) {
            s.store_add_scaled_value_products(359, A::add_scaled_inputs3(s.ad_value(359), 1.0, s.ad_value(100), p.p5, s.ad_value(104), p.p5), 1.0, s.ad_value(20), s.ad_value(98), 1.0, s.ad_value(21), s.ad_value(106), 1.0);
        }

        if (s.b[399] && (!s.b[405])) {
            s.store_add_scaled_value_products3_indices(358, 354, 1.0, 19, 355, 1.0, 20, 97, 1.0, 21, 105, 1.0);
            s.store_add_scaled_inputs4(355, s.ad_value(355), 1.0, s.ad_value(354), 1.0, s.ad_value(97), 1.0, s.ad_value(105), 1.0);
            s.store_add_scaled_product_value_ad(359, A::add_scaled_inputs_products(s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(19), s.ad_value(219), 1.0, s.ad_value(20), s.ad_value(98), 1.0), 1.0, 21, 106, 1.0);
            s.store_add_ad_lhs(219, A::add_scaled_inputs4(s.ad_value(219), 1.0, s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(98), 1.0), 106);
        }

        s.store_scale(356, 218, p.p85);

        s.v[224] = 0.0;

        s.b[406] = (((p.p0 >= 310.0) && (s.v[358] > (1e-5 * s.v[348]))) || ((p.p0 <= 300.0) && (s.v[355] > (1e-5 * s.v[348]))));
        s.v[406] = if s.b[406] { 1.0 } else { 0.0 };

        if s.b[406] {
            s.store_sqrt_ad(355, A::mul3(s.ad_value(357), s.ad_value(217), s.ad_value(358)));
            s.store_add_scaled_inputs3(348, s.ad_value(352), 1.0, s.ad_value(355), 1.0, s.ad_value(356), p.p7);
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
            s.v[408] = if s.b[408] { 1.0 } else { 0.0 };
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
            s.v[409] = if s.b[409] { 1.0 } else { 0.0 };
            if (s.b[406] && s.b[409]) {
                s.store_div(96, 217, 362);
                s.store_mul_ad_rhs(98, 61, A::exp_scaled_input(A::ln(s.ad_value(96)), p.p70));
                s.store_scaled_mul(97, 98, 217, 1.0 / ((1.0 + p.p70)));
            }
            s.b[410] = (p.p83 < (0.05 * (p.p75 / p.p74)));
            s.v[410] = if s.b[410] { 1.0 } else { 0.0 };
            if ((s.b[406] && s.b[409]) && s.b[410]) {
                s.store_scalar(111, 0.0);
                s.store_scalar(112, 0.0);
            }
            if ((s.b[406] && s.b[409]) && (!s.b[410])) {
                s.store_scaled_sub(107, 217, 362, 1.0 / (p.p83));
            }
            s.b[411] = (s.v[107] < (-10000000000.0));
            s.v[411] = if s.b[411] { 1.0 } else { 0.0 };
            if (((s.b[406] && s.b[409]) && (!s.b[410])) && s.b[411]) {
                s.store_scalar(107, (-10000000000.0));
            }
            if ((s.b[406] && s.b[409]) && (!s.b[410])) {
                s.store_sqrt_square_offset(95, 107, p.p84);
                s.store_scaled_exp_ad(111, A::div_from_scalar((-2.0), A::add(s.ad_value(107), s.ad_value(95))), p.p82);
                s.store_div_scaled_inputs(112, s.ad_value(111), 2.0, A::mul_scaled_lhs(s.ad_value(95), p.p83, A::add(s.ad_value(107), s.ad_value(95))), 1.0);
            }
            if (s.b[406] && s.b[409]) {
                s.store_mul_scaled_ad_rhs(99, 60, (1.0 - p.p73), A::offset(A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (-1.0)));
                s.store_add_ad_rhs(100, 99, A::mul3(A::mul3_scaled_output(s.ad_value(60), s.ad_value(217), A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (1.0 - p.p73)), s.ad_value(5), s.ad_value(112)));
                s.store_sub_from_scalar_ad(108, 1.0, A::div_from_scalar(1.0, s.ad_value(96)));
                s.store_scaled_add_sqrt_square_offset_rhs(109, 108, 108, p.p72, 1.0 / ((1.0 + (((1.0 + p.p72)) as f64).sqrt())));
                s.store_exp_ad(110, A::mul_offset_lhs(s.ad_value(111), (-p.p82), s.ad_value(5)));
                s.store_mul_product3_rhs(101, 110, s.ad_value(60), s.ad_value(109), s.ad_value(109), 1.0);
                s.store_mul_add_ad_rhs(102, 101, A::offset(A::div_from_scalar(2.0, A::mul(s.ad_value(96), A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)))), 1.0), A::mul3(s.ad_value(5), s.ad_value(217), s.ad_value(112)));
            }
            s.b[412] = ((((p.p115 < 0.01) && (p.p116 < 0.01)) && ((s.v[109] * p.p115) < 0.005)) && ((s.v[109] * p.p116) < 0.005));
            s.v[412] = if s.b[412] { 1.0 } else { 0.0 };
            if ((s.b[406] && s.b[409]) && s.b[412]) {
                s.store_scaled_mul(105, 101, 217, p.p73);
                s.store_scale(106, 102, p.p73);
            }
            if ((s.b[406] && s.b[409]) && (!s.b[412])) {
                s.store_sub_from_scalar(146, 1.0, 109);
                s.store_div_ad(147, A::mul_sub_from_scalar_rhs(A::offset(s.ad_value(146), (-1.0)), 1.0, s.ad_value(108)), A::mul(A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)), s.ad_value(217)));
            }
            s.b[413] = (((s.v[232]) as f64).abs() > 0.001);
            s.v[413] = if s.b[413] { 1.0 } else { 0.0 };
            if (((s.b[406] && s.b[409]) && (!s.b[412])) && s.b[413]) {
                s.store_exp_ad(151, A::mul_offset_lhs(s.ad_value(146), (-1.0), s.ad_value(231)));
            }
            s.b[414] = (s.v[229] < 0.01);
            s.v[414] = if s.b[414] { 1.0 } else { 0.0 };
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
                s.store_div_scaled_inputs2(154, s.ad_value(157), 1.0, s.ad_value(156), (-1.0), s.ad_value(232), 1.0);
                s.store_mul_product3_rhs(150, 147, A::div_scaled_inputs(s.ad_value(232), (-2.0), A::square(s.ad_value(152)), 1.0), s.ad_value(151), s.ad_value(231), 1.0);
                s.store_div_scaled_product_left_ad(155, A::sub(s.ad_value(159), s.ad_value(158)), 150, 1.0, 232, 1.0);
            }
            if (((s.b[406] && s.b[409]) && (!s.b[412])) && (!s.b[413])) {
                s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(146)), A::scale_offset(s.ad_value(146), p.p115, 1.0));
                s.store_offset_scaled(153, 149, p.p115, 1.0);
                s.store_div_scaled_product_offset_rhs(154, A::square(s.ad_value(149)), A::mul_scaled_lhs(s.ad_value(227), 2.0, s.ad_value(149)), 1.0, 1.0, s.ad_value(153), 1.0);
                s.store_div_scaled_product_denominator_ad(150, 147, 153, -1.0, A::scale_offset(s.ad_value(146), p.p115, 1.0), 1.0);
                s.store_mul_ad_product_lhs(155, s.ad_value(149), A::offset(A::div_from_scalar(1.0, A::square(s.ad_value(153))), 1.0), 150);
            }
            if ((s.b[406] && s.b[409]) && (!s.b[412])) {
                s.store_scaled_mul(166, 60, 110, p.p73);
                s.store_mul(167, 166, 154);
                s.store_mul(105, 167, 217);
                s.store_add_scaled_inputs3(106, s.ad_value(167), 1.0, A::mul3(s.ad_value(105), s.ad_value(112), s.ad_value(5)), 1.0, A::mul3(s.ad_value(166), s.ad_value(217), s.ad_value(155)), 1.0);
            }
            if (s.b[406] && s.b[409]) {
                s.store_scaled_mul(103, 101, 217, (1.0 - p.p73));
                s.store_scale(104, 102, (1.0 - p.p73));
                s.store_add_scaled_product_indices(354, 103, 1.0, 99, 217, 1.0);
            }
            s.b[415] = (p.p0 >= 310.0);
            s.v[415] = if s.b[415] { 1.0 } else { 0.0 };
            if ((s.b[406] && s.b[409]) && s.b[415]) {
                s.store_add_scaled_inputs4(355, s.ad_value(355), 1.0, s.ad_value(354), 1.0, s.ad_value(97), 1.0, s.ad_value(105), 1.0);
                s.store_add_ad_lhs(219, A::add_scaled_inputs4(s.ad_value(219), 1.0, s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(98), 1.0), 106);
                s.store_add_scaled_inputs_products_indices(358, 358, 1.0, 354, p.p5, 20, 97, 1.0, 21, 105, 1.0);
                s.store_add_scaled_value_products(359, A::add_scaled_inputs3(s.ad_value(359), 1.0, s.ad_value(100), p.p5, s.ad_value(104), p.p5), 1.0, s.ad_value(20), s.ad_value(98), 1.0, s.ad_value(21), s.ad_value(106), 1.0);
            }
            if ((s.b[406] && s.b[409]) && (!s.b[415])) {
                s.store_add_scaled_value_products3_indices(358, 354, 1.0, 19, 355, 1.0, 20, 97, 1.0, 21, 105, 1.0);
                s.store_add_scaled_inputs4(355, s.ad_value(355), 1.0, s.ad_value(354), 1.0, s.ad_value(97), 1.0, s.ad_value(105), 1.0);
                s.store_add_scaled_product_value_ad(359, A::add_scaled_inputs_products(s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(19), s.ad_value(219), 1.0, s.ad_value(20), s.ad_value(98), 1.0), 1.0, 21, 106, 1.0);
                s.store_add_ad_lhs(219, A::add_scaled_inputs4(s.ad_value(219), 1.0, s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(98), 1.0), 106);
            }
            if s.b[406] {
                s.store_scale(360, 218, (p.p7 * p.p85));
                s.store_div_scaled_inputs(349, A::add_scaled_inputs4(s.ad_value(348), 1.0, s.ad_value(352), -1.0, s.ad_value(358), -1.0, s.ad_value(360), -1.0), -1.0, A::offset(A::div_scaled_add_product(s.ad_value(360), 1.0, s.ad_value(359), s.ad_value(217), 1.0, s.ad_value(348), 1.0), 1.0), 1.0);
                s.store_abs_scaled_input(407, 348, 0.3);
            }
            s.b[416] = (((s.v[349]) as f64).abs() > s.v[407]);
            s.v[416] = if s.b[416] { 1.0 } else { 0.0 };
            s.b[417] = (s.v[349] >= 0.0);
            s.v[417] = if s.b[417] { 1.0 } else { 0.0 };
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
        s.v[418] = if s.b[418] { 1.0 } else { 0.0 };

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
        s.v[419] = if s.b[419] { 1.0 } else { 0.0 };

        if (s.b[406] && s.b[419]) {
            s.store_div(96, 217, 362);
            s.store_mul_ad_rhs(98, 61, A::exp_scaled_input(A::ln(s.ad_value(96)), p.p70));
            s.store_scaled_mul(97, 98, 217, 1.0 / ((1.0 + p.p70)));
        }

        s.b[420] = (p.p83 < (0.05 * (p.p75 / p.p74)));
        s.v[420] = if s.b[420] { 1.0 } else { 0.0 };

        if ((s.b[406] && s.b[419]) && s.b[420]) {
            s.store_scalar(111, 0.0);
            s.store_scalar(112, 0.0);
        }

        if ((s.b[406] && s.b[419]) && (!s.b[420])) {
            s.store_scaled_sub(107, 217, 362, 1.0 / (p.p83));
        }

        s.b[421] = (s.v[107] < (-10000000000.0));
        s.v[421] = if s.b[421] { 1.0 } else { 0.0 };

        if (((s.b[406] && s.b[419]) && (!s.b[420])) && s.b[421]) {
            s.store_scalar(107, (-10000000000.0));
        }

        if ((s.b[406] && s.b[419]) && (!s.b[420])) {
            s.store_sqrt_square_offset(95, 107, p.p84);
            s.store_scaled_exp_ad(111, A::div_from_scalar((-2.0), A::add(s.ad_value(107), s.ad_value(95))), p.p82);
            s.store_div_scaled_inputs(112, s.ad_value(111), 2.0, A::mul_scaled_lhs(s.ad_value(95), p.p83, A::add(s.ad_value(107), s.ad_value(95))), 1.0);
        }

        if (s.b[406] && s.b[419]) {
            s.store_mul_scaled_ad_rhs(99, 60, (1.0 - p.p73), A::offset(A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (-1.0)));
            s.store_add_ad_rhs(100, 99, A::mul3(A::mul3_scaled_output(s.ad_value(60), s.ad_value(217), A::exp(A::mul(s.ad_value(111), s.ad_value(5))), (1.0 - p.p73)), s.ad_value(5), s.ad_value(112)));
            s.store_sub_from_scalar_ad(108, 1.0, A::div_from_scalar(1.0, s.ad_value(96)));
            s.store_scaled_add_sqrt_square_offset_rhs(109, 108, 108, p.p72, 1.0 / ((1.0 + (((1.0 + p.p72)) as f64).sqrt())));
            s.store_exp_ad(110, A::mul_offset_lhs(s.ad_value(111), (-p.p82), s.ad_value(5)));
            s.store_mul_product3_rhs(101, 110, s.ad_value(60), s.ad_value(109), s.ad_value(109), 1.0);
            s.store_mul_add_ad_rhs(102, 101, A::offset(A::div_from_scalar(2.0, A::mul(s.ad_value(96), A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)))), 1.0), A::mul3(s.ad_value(5), s.ad_value(217), s.ad_value(112)));
        }

        s.b[422] = ((((p.p115 < 0.01) && (p.p116 < 0.01)) && ((s.v[109] * p.p115) < 0.005)) && ((s.v[109] * p.p116) < 0.005));
        s.v[422] = if s.b[422] { 1.0 } else { 0.0 };

        if ((s.b[406] && s.b[419]) && s.b[422]) {
            s.store_scaled_mul(105, 101, 217, p.p73);
            s.store_scale(106, 102, p.p73);
        }

        if ((s.b[406] && s.b[419]) && (!s.b[422])) {
            s.store_sub_from_scalar(146, 1.0, 109);
            s.store_div_ad(147, A::mul_sub_from_scalar_rhs(A::offset(s.ad_value(146), (-1.0)), 1.0, s.ad_value(108)), A::mul(A::sqrt(A::offset(A::square(s.ad_value(108)), p.p72)), s.ad_value(217)));
        }

        s.b[423] = (((s.v[232]) as f64).abs() > 0.001);
        s.v[423] = if s.b[423] { 1.0 } else { 0.0 };

        if (((s.b[406] && s.b[419]) && (!s.b[422])) && s.b[423]) {
            s.store_exp_ad(151, A::mul_offset_lhs(s.ad_value(146), (-1.0), s.ad_value(231)));
        }

    }

    pub(super) fn stamp_reactive_block_5(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[424] = (s.v[229] < 0.01);
        s.v[424] = if s.b[424] { 1.0 } else { 0.0 };

        if ((((s.b[406] && s.b[419]) && (!s.b[422])) && s.b[423]) && s.b[424]) {
            s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(151)), A::mul(s.ad_value(151), s.ad_value(230)));
            s.store_offset_mul(148, 230, 149, 1.0);
            s.store_div_scaled_inputs2_by_product(154, A::mul3(s.ad_value(230), s.ad_value(149), A::offset(A::mul_scaled_lhs(s.ad_value(230), 0.25, s.ad_value(149)), 0.5)), 2.0, A::ln(s.ad_value(148)), (-(0.5 * 2.0)), s.ad_value(230), s.ad_value(230), 1.0);
            s.store_div_scaled_product_by_product(150, s.ad_value(231), s.ad_value(147), -1.0, s.ad_value(151), s.ad_value(230), 1.0);
            s.store_div_scaled_product3_mixed_aiii(155, A::offset(s.ad_value(148), 1.0), 149, 150, 1.0, 148, 1.0);
        }

        if ((((s.b[406] && s.b[419]) && (!s.b[422])) && s.b[423]) && (!s.b[424])) {
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
            s.store_div_scaled_inputs2(154, s.ad_value(157), 1.0, s.ad_value(156), (-1.0), s.ad_value(232), 1.0);
            s.store_mul_product3_rhs(150, 147, A::div_scaled_inputs(s.ad_value(232), (-2.0), A::square(s.ad_value(152)), 1.0), s.ad_value(151), s.ad_value(231), 1.0);
            s.store_div_scaled_product_left_ad(155, A::sub(s.ad_value(159), s.ad_value(158)), 150, 1.0, 232, 1.0);
        }

        if (((s.b[406] && s.b[419]) && (!s.b[422])) && (!s.b[423])) {
            s.store_div_ad(149, A::sub_from_scalar(1.0, s.ad_value(146)), A::scale_offset(s.ad_value(146), p.p115, 1.0));
            s.store_offset_scaled(153, 149, p.p115, 1.0);
            s.store_div_scaled_product_offset_rhs(154, A::square(s.ad_value(149)), A::mul_scaled_lhs(s.ad_value(227), 2.0, s.ad_value(149)), 1.0, 1.0, s.ad_value(153), 1.0);
            s.store_div_scaled_product_denominator_ad(150, 147, 153, -1.0, A::scale_offset(s.ad_value(146), p.p115, 1.0), 1.0);
            s.store_mul_ad_product_lhs(155, s.ad_value(149), A::offset(A::div_from_scalar(1.0, A::square(s.ad_value(153))), 1.0), 150);
        }

        if ((s.b[406] && s.b[419]) && (!s.b[422])) {
            s.store_scaled_mul(166, 60, 110, p.p73);
            s.store_mul(167, 166, 154);
            s.store_mul(105, 167, 217);
            s.store_add_scaled_inputs3(106, s.ad_value(167), 1.0, A::mul3(s.ad_value(105), s.ad_value(112), s.ad_value(5)), 1.0, A::mul3(s.ad_value(166), s.ad_value(217), s.ad_value(155)), 1.0);
        }

        if (s.b[406] && s.b[419]) {
            s.store_scaled_mul(103, 101, 217, (1.0 - p.p73));
            s.store_scale(104, 102, (1.0 - p.p73));
            s.store_add_scaled_product_indices(354, 103, 1.0, 99, 217, 1.0);
        }

        s.b[425] = (p.p0 >= 310.0);
        s.v[425] = if s.b[425] { 1.0 } else { 0.0 };

        if ((s.b[406] && s.b[419]) && s.b[425]) {
            s.store_add_scaled_inputs4(355, s.ad_value(355), 1.0, s.ad_value(354), 1.0, s.ad_value(97), 1.0, s.ad_value(105), 1.0);
            s.store_add_ad_lhs(219, A::add_scaled_inputs4(s.ad_value(219), 1.0, s.ad_value(100), 1.0, s.ad_value(104), 1.0, s.ad_value(98), 1.0), 106);
            s.store_add_scaled_inputs_products_indices(358, 358, 1.0, 354, p.p5, 20, 97, 1.0, 21, 105, 1.0);
            s.store_add_scaled_value_products(359, A::add_scaled_inputs3(s.ad_value(359), 1.0, s.ad_value(100), p.p5, s.ad_value(104), p.p5), 1.0, s.ad_value(20), s.ad_value(98), 1.0, s.ad_value(21), s.ad_value(106), 1.0);
        }

        if ((s.b[406] && s.b[419]) && (!s.b[425])) {
            s.store_add_scaled_value_products3_indices(358, 354, 1.0, 19, 355, 1.0, 20, 97, 1.0, 21, 105, 1.0);
            s.store_add_scaled_inputs4(355, s.ad_value(355), 1.0, s.ad_value(354), 1.0, s.ad_value(97), 1.0, s.ad_value(105), 1.0);
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

        s.store_add_scaled_inputs4(222, s.ad_value(211), p.p93, s.ad_value(210), p.p93, s.ad_value(220), p.p93, s.ad_value(221), p.p93);

        s.store_mul_voltage_ad(183, s.ad_value(222), ctx, nodes, Some(7), Some(8));

        s.b[426] = (p.p23 > 0.0);
        s.v[426] = if s.b[426] { 1.0 } else { 0.0 };

        if s.b[426] {
            s.store_div_scaled_inputs(93, s.ad_value(203), 1.0, s.ad_value(4), p.p24);
        }

        s.b[427] = (s.v[93] > 80.0);
        s.v[427] = if s.b[427] { 1.0 } else { 0.0 };

        if (s.b[426] && s.b[427]) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
            s.store_scalar(93, 80.0);
        }

        if (s.b[426] && (!s.b[427])) {
            s.store_scalar(94, 1.0);
        }

        s.b[428] = ((p.p37 > 0.0) && (s.v[203] < 0.0));
        s.v[428] = if s.b[428] { 1.0 } else { 0.0 };

        s.b[429] = ((s.v[33] > 0.0) && (s.v[34] > 0.0));
        s.v[429] = if s.b[429] { 1.0 } else { 0.0 };

        if (s.b[428] && s.b[429]) {
            s.store_exp_scaled_input_ad(168, A::ln(A::div(s.ad_value(210), s.ad_value(33))), ((1.0 / p.p49) - 1.0));
            s.store_div_scaled_product_by_product(166, s.ad_value(67), s.ad_value(203), -1.0, s.ad_value(34), s.ad_value(168), 1.0);
        }

        s.b[456] = (p.p18 > 0.0);
        s.v[456] = if s.b[456] { 1.0 } else { 0.0 };

        if s.b[456] {
            s.store_div_scaled_inputs(93, s.ad_value(205), 1.0, s.ad_value(4), p.p19);
        }

        s.b[457] = (s.v[93] > 80.0);
        s.v[457] = if s.b[457] { 1.0 } else { 0.0 };

        if (s.b[456] && s.b[457]) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
            s.store_scalar(93, 80.0);
        }

        if (s.b[456] && (!s.b[457])) {
            s.store_scalar(94, 1.0);
        }

        s.b[458] = (p.p20 > 0.0);
        s.v[458] = if s.b[458] { 1.0 } else { 0.0 };

        if s.b[458] {
            s.store_div_scaled_inputs(93, s.ad_value(205), 1.0, s.ad_value(4), p.p21);
        }

        s.b[459] = (s.v[93] > 80.0);
        s.v[459] = if s.b[459] { 1.0 } else { 0.0 };

        if (s.b[458] && s.b[459]) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
            s.store_scalar(93, 80.0);
        }

        if (s.b[458] && (!s.b[459])) {
            s.store_scalar(94, 1.0);
        }

        s.b[460] = (s.v[29] > 0.0);
        s.v[460] = if s.b[460] { 1.0 } else { 0.0 };

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
        s.v[466] = if s.b[466] { 1.0 } else { 0.0 };

        s.b[467] = (s.v[38] > 0.0);
        s.v[467] = if s.b[467] { 1.0 } else { 0.0 };

        if (s.b[466] && s.b[467]) {
            s.store_scalar(113, (p.p54 / 4.0));
            s.store_sub_from_scalar(114, p.p56, 39);
            s.store_mul_sub_from_scalar_ad_rhs(115, 39, 1.0, A::exp_scaled_input(A::ln(s.ad_value(40)), (-1.0 / (p.p54))));
            s.store_mul(116, 40, 38);
            s.store_mul_exp_ad_rhs(117, 38, A::mul_offset_lhs(s.ad_value(113), (-p.p54), A::ln(A::div_from_scalar(p.p56, s.ad_value(39)))));
            s.store_mul_sub_lhs(119, 115, 206, 5);
        }

        s.b[468] = (s.v[119] < 80.0);
        s.v[468] = if s.b[468] { 1.0 } else { 0.0 };

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
            s.store_div_scaled_inputs2(123, s.ad_value(114), 1.0, s.ad_value(122), 1.0, s.ad_value(118), 1.0);
        }

        s.b[469] = (s.v[123] < 80.0);
        s.v[469] = if s.b[469] { 1.0 } else { 0.0 };

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
            s.store_mul_product3_rhs(134, 124, s.ad_value(38), A::exp_scaled_input(s.ad_value(131), (-p.p54)), s.ad_value(121), 1.0);
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
        s.v[470] = if s.b[470] { 1.0 } else { 0.0 };

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
        s.v[471] = if s.b[471] { 1.0 } else { 0.0 };

        if s.b[471] {
            s.store_div_scaled_inputs(93, s.ad_value(206), 1.0, s.ad_value(4), p.p26);
        }

        s.b[472] = (s.v[93] > 80.0);
        s.v[472] = if s.b[472] { 1.0 } else { 0.0 };

        if (s.b[471] && s.b[472]) {
            s.store_offset(94, 93, (((-80.0)) + (1.0)));
            s.store_scalar(93, 80.0);
        }

    }
}
