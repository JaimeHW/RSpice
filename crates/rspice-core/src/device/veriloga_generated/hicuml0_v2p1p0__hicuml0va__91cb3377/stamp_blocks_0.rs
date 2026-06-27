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
        s.store_scaled_voltage(183, ctx, nodes, Some(1), Some(5), p.p110);

        s.store_scaled_voltage(184, ctx, nodes, Some(6), Some(5), p.p110);

        s.store_scaled_voltage(185, ctx, nodes, Some(6), Some(7), p.p110);

        s.store_sub(186, 185, 184);

        s.store_scaled_voltage(187, ctx, nodes, Some(3), Some(5), p.p110);

        s.store_scaled_voltage(191, ctx, nodes, Some(1), Some(2), p.p110);

        s.v[8] = (p.p108 + 273.15);

        s.v[9] = ctx_temp;

        s.v[177] = ((1.3806226e-23 * s.v[8]) / 1.602176462e-19);

        s.v[172] = (p.p88 * s.v[8]);

        s.v[173] = (0.5 * (p.p76 + p.p77));

        s.v[174] = (0.5 * (p.p76 + p.p78));

        s.v[175] = (0.5 * (p.p79 + p.p78));

        s.v[168] = (3.0 - ((1.602176462e-19 * p.p80) / 1.3806226e-23));

        s.v[171] = ((p.p82 - p.p81) - 0.5);

        s.v[176] = (p.p76 - p.p77);

        s.v[27] = p.p34;

        s.v[4] = (s.v[9] + p.p109);

        s.b[247] = (s.v[4] < ((-100.0) + 273.15));
        s.v[247] = if s.b[247] { 1.0 } else { 0.0 };

        if s.b[247] {
            s.store_scalar(4, ((-100.0) + 273.15));
        }

        s.b[248] = (s.v[4] > (326.85 + 273.15));
        s.v[248] = if s.b[248] { 1.0 } else { 0.0 };

        if ((!s.b[247]) && s.b[248]) {
            s.store_scalar(4, (326.85 + 273.15));
        }

        s.store_scale(2, 4, (1.3806226e-23 * 6.241509744511525e18));

        s.store_div_from_scalar(3, 1.0, 2);

        s.store_offset(7, 4, (-s.v[8]));

        s.store_scale(5, 4, 1.0 / (s.v[8]));

        s.store_ln(6, 5);

        s.store_mul_offset_rhs(10, 3, 5, (-1.0));

        s.v[178] = ((0.5 * p.p35) / s.v[177]);

        s.v[96] = ((2.0 * s.v[177]) * (((((s.v[178]) as f64).exp() - (((-s.v[178])) as f64).exp())) as f64).ln());

        s.store_add_scaled_inputs_product_second_ad(97, 5, s.v[96], A::scale_offset(s.ad_value(5), (-s.v[173]), s.v[173]), 1.0, 2, 6, (-s.v[168]));

        s.store_add_scaled_product_right_ad(16, 97, 1.0, 2, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(97), -1.0, s.ad_value(3))), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scale_ad(23, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p35, s.ad_value(16))), p.p36), p.p34);

        s.store_scale(43, 16, (p.p37 * 1.0 / (p.p35)));

        s.v[178] = ((0.5 * p.p38) / s.v[177]);

        s.v[96] = ((2.0 * s.v[177]) * (((((s.v[178]) as f64).exp() - (((-s.v[178])) as f64).exp())) as f64).ln());

        s.store_add_scaled_inputs_product_second_ad(97, 5, s.v[96], A::scale_offset(s.ad_value(5), (-s.v[173]), s.v[173]), 1.0, 2, 6, (-s.v[168]));

        s.store_add_scaled_product_right_ad(22, 97, 1.0, 2, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(97), -1.0, s.ad_value(3))), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scale_ad(26, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p38, s.ad_value(22))), p.p39), s.v[27]);

        s.store_scale(44, 22, (p.p40 * 1.0 / (p.p38)));

        s.v[178] = ((0.5 * p.p42) / s.v[177]);

        s.v[96] = ((2.0 * s.v[177]) * (((((s.v[178]) as f64).exp() - (((-s.v[178])) as f64).exp())) as f64).ln());

        s.store_add_scaled_inputs_product_second_ad(97, 5, s.v[96], A::scale_offset(s.ad_value(5), (-s.v[174]), s.v[174]), 1.0, 2, 6, (-s.v[168]));

        s.store_add_scaled_product_right_ad(17, 97, 1.0, 2, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(97), -1.0, s.ad_value(3))), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scale_ad(24, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p42, s.ad_value(17))), p.p43), p.p41);

        s.store_scaled_exp_ad(11, A::add_scaled_inputs(s.ad_value(6), p.p81, s.ad_value(10), p.p76), p.p1);

        s.store_scaled_exp_ad(15, A::sub_scaled_inputs(s.ad_value(6), p.p95, s.ad_value(10), p.p83), p.p9);

        s.store_scaled_exp_scaled_input(33, 6, (p.p87 - s.v[172]), p.p62);

        s.store_scaled_exp_scaled_input(31, 6, p.p87, p.p61);

        s.store_div_from_scalar(32, 1.0, 31);

        s.store_offset_scaled(34, 7, ((p.p89) * (p.p64)), p.p64);

        s.b[249] = (p.p65 > 0.0);
        s.v[249] = if s.b[249] { 1.0 } else { 0.0 };

        if s.b[249] {
            s.store_offset_scaled_ad(38, A::scale(s.ad_value(7), p.p90), (-p.p65), p.p65);
            s.store_scalar(34, p.p64);
        }

        if (!s.b[249]) {
            s.store_offset_scaled(34, 7, ((p.p89) * (p.p64)), p.p64);
            s.store_scalar(38, p.p65);
        }

        s.store_add_scaled_product_value_ad(42, A::scale_offset(s.ad_value(7), p.p85, 1.0), p.p54, 7, 7, (p.p86 * p.p54));

        s.b[250] = (p.p96 == 1.0);
        s.v[250] = if s.b[250] { 1.0 } else { 0.0 };

        if s.b[250] {
            s.store_scaled_exp_ad(36, A::sub_scaled_inputs(s.ad_value(6), s.v[171], s.ad_value(10), s.v[176]), p.p57);
        }

        if (!s.b[250]) {
            s.store_scalar(36, p.p57);
        }

        s.store_scaled_exp_scaled_input(35, 6, (p.p87 - 1.0), p.p59);

        s.v[178] = ((0.5 * p.p46) / s.v[177]);

        s.v[96] = ((2.0 * s.v[177]) * (((((s.v[178]) as f64).exp() - (((-s.v[178])) as f64).exp())) as f64).ln());

        s.store_add_scaled_inputs_product_second_ad(97, 5, s.v[96], A::scale_offset(s.ad_value(5), (-s.v[174]), s.v[174]), 1.0, 2, 6, (-s.v[168]));

        s.store_add_scaled_product_right_ad(18, 97, 1.0, 2, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(97), -1.0, s.ad_value(3))), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scale_ad(25, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p46, s.ad_value(18))), p.p47), p.p45);

        s.v[178] = ((0.5 * p.p51) / s.v[177]);

        s.v[96] = ((2.0 * s.v[177]) * (((((s.v[178]) as f64).exp() - (((-s.v[178])) as f64).exp())) as f64).ln());

        s.store_add_scaled_inputs_product_second_ad(97, 5, s.v[96], A::scale_offset(s.ad_value(5), (-s.v[175]), s.v[175]), 1.0, 2, 6, (-s.v[168]));

        s.store_add_scaled_product_right_ad(19, 97, 1.0, 2, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(97), -1.0, s.ad_value(3))), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scale_ad(30, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p51, s.ad_value(19))), p.p52), p.p50);

        s.store_scaled_exp_scaled_input(200, 6, p.p97, p.p7);

        s.store_div_from_scalar_exp_ad(202, p.p6, A::mul_scaled_lhs(s.ad_value(3), p.p83, A::offset(A::exp_scaled_input(s.ad_value(6), p.p84), (-1.0))));

        s.b[252] = (p.p0 <= 200.0);
        s.v[252] = if s.b[252] { 1.0 } else { 0.0 };

        if s.b[252] {
            s.store_offset_mul_ad(204, s.ad_value(7), A::scale_offset(s.ad_value(7), p.p102, p.p101), 1.0);
        }

        if (!s.b[252]) {
            s.store_exp_scaled_input(204, 6, p.p98);
        }

        s.store_scale(203, 204, p.p12);

        s.store_mul_scaled_ad_rhs(205, 204, p.p13, A::exp_scaled_input(s.ad_value(10), s.v[176]));

        s.v[206] = p.p14;

        s.b[253] = ((p.p103 != 0.0) && (p.p104 >= p.p111));
        s.v[253] = if s.b[253] { 1.0 } else { 0.0 };

        if s.b[253] {
            s.store_offset_voltage(4, ctx, nodes, Some(4), None, (s.v[9] + p.p109));
        }

        s.b[254] = (s.v[4] < ((-100.0) + 273.15));
        s.v[254] = if s.b[254] { 1.0 } else { 0.0 };

        if (s.b[253] && s.b[254]) {
            s.store_scalar(4, ((-100.0) + 273.15));
        }

        s.b[255] = (s.v[4] > (326.85 + 273.15));
        s.v[255] = if s.b[255] { 1.0 } else { 0.0 };

        if ((s.b[253] && (!s.b[254])) && s.b[255]) {
            s.store_scalar(4, (326.85 + 273.15));
        }

        if s.b[253] {
            s.store_scale(2, 4, (1.3806226e-23 * 6.241509744511525e18));
            s.store_div_from_scalar(3, 1.0, 2);
            s.store_offset(7, 4, (-s.v[8]));
            s.store_scale(5, 4, 1.0 / (s.v[8]));
            s.store_ln(6, 5);
            s.store_mul_offset_rhs(10, 3, 5, (-1.0));
            s.store_scalar(178, ((0.5 * p.p35) / s.v[177]));
            s.store_scaled_ln_ad(96, A::sub(A::exp(s.ad_value(178)), A::exp_scaled_input(s.ad_value(178), -1.0)), (2.0 * s.v[177]));
            s.store_add_scaled_value_products(97, A::scale_offset(s.ad_value(5), (-s.v[173]), s.v[173]), 1.0, s.ad_value(96), s.ad_value(5), 1.0, s.ad_value(2), s.ad_value(6), (-s.v[168]));
            s.store_add_scaled_product_right_ad(16, 97, 1.0, 2, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(97), -1.0, s.ad_value(3))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(23, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p35, s.ad_value(16))), p.p36), p.p34);
            s.store_scale(43, 16, (p.p37 * 1.0 / (p.p35)));
            s.store_scalar(178, ((0.5 * p.p38) / s.v[177]));
            s.store_scaled_ln_ad(96, A::sub(A::exp(s.ad_value(178)), A::exp_scaled_input(s.ad_value(178), -1.0)), (2.0 * s.v[177]));
            s.store_add_scaled_value_products(97, A::scale_offset(s.ad_value(5), (-s.v[173]), s.v[173]), 1.0, s.ad_value(96), s.ad_value(5), 1.0, s.ad_value(2), s.ad_value(6), (-s.v[168]));
            s.store_add_scaled_product_right_ad(22, 97, 1.0, 2, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(97), -1.0, s.ad_value(3))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(26, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p38, s.ad_value(22))), p.p39), s.v[27]);
            s.store_scale(44, 22, (p.p40 * 1.0 / (p.p38)));
            s.store_scalar(178, ((0.5 * p.p42) / s.v[177]));
            s.store_scaled_ln_ad(96, A::sub(A::exp(s.ad_value(178)), A::exp_scaled_input(s.ad_value(178), -1.0)), (2.0 * s.v[177]));
            s.store_add_scaled_value_products(97, A::scale_offset(s.ad_value(5), (-s.v[174]), s.v[174]), 1.0, s.ad_value(96), s.ad_value(5), 1.0, s.ad_value(2), s.ad_value(6), (-s.v[168]));
            s.store_add_scaled_product_right_ad(17, 97, 1.0, 2, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(97), -1.0, s.ad_value(3))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(24, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p42, s.ad_value(17))), p.p43), p.p41);
            s.store_scaled_exp_ad(11, A::add_scaled_inputs(s.ad_value(6), p.p81, s.ad_value(10), p.p76), p.p1);
            s.store_scaled_exp_ad(15, A::sub_scaled_inputs(s.ad_value(6), p.p95, s.ad_value(10), p.p83), p.p9);
            s.store_scaled_exp_scaled_input(33, 6, (p.p87 - s.v[172]), p.p62);
            s.store_scaled_exp_scaled_input(31, 6, p.p87, p.p61);
            s.store_div_from_scalar(32, 1.0, 31);
            s.store_offset_scaled(34, 7, ((p.p89) * (p.p64)), p.p64);
        }

        s.b[256] = (p.p65 > 0.0);
        s.v[256] = if s.b[256] { 1.0 } else { 0.0 };

        if (s.b[253] && s.b[256]) {
            s.store_offset_scaled_ad(38, A::scale(s.ad_value(7), p.p90), (-p.p65), p.p65);
            s.store_scalar(34, p.p64);
        }

        if (s.b[253] && (!s.b[256])) {
            s.store_offset_scaled(34, 7, ((p.p89) * (p.p64)), p.p64);
            s.store_scalar(38, p.p65);
        }

        if s.b[253] {
            s.store_add_scaled_product_value_ad(42, A::scale_offset(s.ad_value(7), p.p85, 1.0), p.p54, 7, 7, (p.p86 * p.p54));
        }

        s.b[257] = (p.p96 == 1.0);
        s.v[257] = if s.b[257] { 1.0 } else { 0.0 };

        if (s.b[253] && s.b[257]) {
            s.store_scaled_exp_ad(36, A::sub_scaled_inputs(s.ad_value(6), s.v[171], s.ad_value(10), s.v[176]), p.p57);
        }

        if (s.b[253] && (!s.b[257])) {
            s.store_scalar(36, p.p57);
        }

        if s.b[253] {
            s.store_scaled_exp_scaled_input(35, 6, (p.p87 - 1.0), p.p59);
            s.store_scalar(178, ((0.5 * p.p46) / s.v[177]));
            s.store_scaled_ln_ad(96, A::sub(A::exp(s.ad_value(178)), A::exp_scaled_input(s.ad_value(178), -1.0)), (2.0 * s.v[177]));
            s.store_add_scaled_value_products(97, A::scale_offset(s.ad_value(5), (-s.v[174]), s.v[174]), 1.0, s.ad_value(96), s.ad_value(5), 1.0, s.ad_value(2), s.ad_value(6), (-s.v[168]));
            s.store_add_scaled_product_right_ad(18, 97, 1.0, 2, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(97), -1.0, s.ad_value(3))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(25, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p46, s.ad_value(18))), p.p47), p.p45);
            s.store_scalar(178, ((0.5 * p.p51) / s.v[177]));
            s.store_scaled_ln_ad(96, A::sub(A::exp(s.ad_value(178)), A::exp_scaled_input(s.ad_value(178), -1.0)), (2.0 * s.v[177]));
            s.store_add_scaled_value_products(97, A::scale_offset(s.ad_value(5), (-s.v[175]), s.v[175]), 1.0, s.ad_value(96), s.ad_value(5), 1.0, s.ad_value(2), s.ad_value(6), (-s.v[168]));
            s.store_add_scaled_product_right_ad(19, 97, 1.0, 2, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(97), -1.0, s.ad_value(3))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(30, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p51, s.ad_value(19))), p.p52), p.p50);
            s.store_scaled_exp_scaled_input(200, 6, p.p97, p.p7);
            s.store_div_from_scalar_exp_ad(202, p.p6, A::mul_scaled_lhs(s.ad_value(3), p.p83, A::offset(A::exp_scaled_input(s.ad_value(6), p.p84), (-1.0))));
        }

        s.b[259] = (p.p0 <= 200.0);
        s.v[259] = if s.b[259] { 1.0 } else { 0.0 };

        if (s.b[253] && s.b[259]) {
            s.store_offset_mul_ad(204, s.ad_value(7), A::scale_offset(s.ad_value(7), p.p102, p.p101), 1.0);
        }

        if (s.b[253] && (!s.b[259])) {
            s.store_exp_scaled_input(204, 6, p.p98);
        }

        if s.b[253] {
            s.store_scale(203, 204, p.p12);
            s.store_mul_scaled_ad_rhs(205, 204, p.p13, A::exp_scaled_input(s.ad_value(10), s.v[176]));
            s.store_scalar(206, p.p14);
        }

        s.b[260] = (s.v[25] <= 1e-30);
        s.v[260] = if s.b[260] { 1.0 } else { 0.0 };

        if s.b[260] {
            s.store_scale(111, 24, p.p49);
            s.store_scalar(108, 0.0);
            s.store_scale(113, 24, (1.0 - p.p49));
        }

        s.b[261] = (p.p44 < 100.0);
        s.v[261] = if s.b[261] { 1.0 } else { 0.0 };

        s.b[262] = (s.v[113] > 0.0);
        s.v[262] = if s.b[262] { 1.0 } else { 0.0 };

        if ((s.b[260] && s.b[261]) && s.b[262]) {
            s.store_scalar(50, (p.p43 / 4.0));
            s.store_sub_from_scalar(51, p.p44, 17);
            s.store_scale(52, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
            s.store_scale(53, 113, 2.4);
            s.store_mul_exp_ad_rhs(54, 113, A::mul_offset_lhs(s.ad_value(50), (-p.p43), A::ln(A::div_from_scalar(p.p44, s.ad_value(17)))));
            s.store_mul_sub_lhs(56, 52, 183, 3);
        }

        s.b[263] = (s.v[56] < 80.0);
        s.v[263] = if s.b[263] { 1.0 } else { 0.0 };

        if (((s.b[260] && s.b[261]) && s.b[262]) && s.b[263]) {
            s.store_exp(57, 56);
            s.store_add_scaled_product_right_ad(58, 52, 1.0, 2, A::ln(A::offset(s.ad_value(57), 1.0)), (-1.0));
        }

        if (((s.b[260] && s.b[261]) && s.b[262]) && (!s.b[263])) {
            s.copy_ad(58, 183);
        }

        if ((s.b[260] && s.b[261]) && s.b[262]) {
            s.store_add_scaled_inputs(55, 51, 0.1, 2, 4.0);
            s.store_div_scaled_inputs2(59, s.ad_value(51), 1.0, s.ad_value(58), 1.0, s.ad_value(55), 1.0);
        }

        s.b[264] = (s.v[59] < 80.0);
        s.v[264] = if s.b[264] { 1.0 } else { 0.0 };

        if (((s.b[260] && s.b[261]) && s.b[262]) && s.b[264]) {
            s.store_exp(57, 59);
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(51), -1.0, s.ad_value(52), -1.0, s.ad_value(55), 1.0)))), 51);
        }

        if (((s.b[260] && s.b[261]) && s.b[262]) && (!s.b[264])) {
            s.copy_ad(60, 58);
        }

    }

    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[260] && s.b[261]) && s.b[262]) {
            s.store_sub(61, 183, 58);
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(17))));
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(17))));
            s.store_scalar(67, (1.0 - p.p43));
            s.store_sub_from_scalar(68, 1.0, 50);
            s.store_div_ad_lhs(62, A::mul_sub_from_scalar_rhs(s.ad_value(113), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67)))), 67);
            s.store_div_ad_lhs(63, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68)))), 68);
            s.store_div_ad_lhs(64, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68)))), 68);
            s.store_add_scaled_products_left_left_ad(105, A::add_scaled_inputs3(s.ad_value(62), 1.0, s.ad_value(63), 1.0, s.ad_value(64), -1.0), 17, 1.0, 53, 61, 1.0);
        }

        if ((s.b[260] && s.b[261]) && (!s.b[262])) {
            s.store_scalar(105, 0.0);
        }

        s.b[265] = (s.v[113] > 0.0);
        s.v[265] = if s.b[265] { 1.0 } else { 0.0 };

        if ((s.b[260] && (!s.b[261])) && s.b[265]) {
            s.store_scale(76, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
            s.store_mul_sub_lhs(80, 76, 183, 3);
            s.store_sqrt_square_offset(81, 80, 1.921812);
            s.store_scaled_add(82, 80, 81, 0.5);
            s.store_add_scaled_product_indices(77, 76, 1.0, 2, 82, (-1.0));
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(17))));
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(79, 17, 1.0, A::exp_scaled_input(s.ad_value(78), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
            s.store_mul_add_scaled_inputs3_offset_rhs(105, 113, s.ad_value(79), 1.0, s.ad_value(183), 2.4, s.ad_value(77), (-2.4), 0.0);
        }

        if ((s.b[260] && (!s.b[261])) && (!s.b[265])) {
            s.store_scalar(105, 0.0);
        }

        if (!s.b[260]) {
            s.copy_ad(111, 24);
            s.store_scale(112, 25, p.p49);
        }

        s.b[266] = (p.p48 < 100.0);
        s.v[266] = if s.b[266] { 1.0 } else { 0.0 };

        s.b[267] = (s.v[112] > 0.0);
        s.v[267] = if s.b[267] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && s.b[266]) && s.b[267]) {
            s.store_scalar(50, (p.p47 / 4.0));
            s.store_sub_from_scalar(51, p.p48, 18);
            s.store_scale(52, 18, (1.0 - ((((-((2.4) as f64).ln()) / p.p47)) as f64).exp()));
            s.store_scale(53, 112, 2.4);
            s.store_mul_exp_ad_rhs(54, 112, A::mul_offset_lhs(s.ad_value(50), (-p.p47), A::ln(A::div_from_scalar(p.p48, s.ad_value(18)))));
            s.store_mul_sub_lhs(56, 52, 184, 3);
        }

        s.b[268] = (s.v[56] < 80.0);
        s.v[268] = if s.b[268] { 1.0 } else { 0.0 };

        if ((((!s.b[260]) && s.b[266]) && s.b[267]) && s.b[268]) {
            s.store_exp(57, 56);
            s.store_add_scaled_product_right_ad(58, 52, 1.0, 2, A::ln(A::offset(s.ad_value(57), 1.0)), (-1.0));
        }

        if ((((!s.b[260]) && s.b[266]) && s.b[267]) && (!s.b[268])) {
            s.copy_ad(58, 184);
        }

        if (((!s.b[260]) && s.b[266]) && s.b[267]) {
            s.store_add_scaled_inputs(55, 51, 0.1, 2, 4.0);
            s.store_div_scaled_inputs2(59, s.ad_value(51), 1.0, s.ad_value(58), 1.0, s.ad_value(55), 1.0);
        }

        s.b[269] = (s.v[59] < 80.0);
        s.v[269] = if s.b[269] { 1.0 } else { 0.0 };

        if ((((!s.b[260]) && s.b[266]) && s.b[267]) && s.b[269]) {
            s.store_exp(57, 59);
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(51), -1.0, s.ad_value(52), -1.0, s.ad_value(55), 1.0)))), 51);
        }

        if ((((!s.b[260]) && s.b[266]) && s.b[267]) && (!s.b[269])) {
            s.copy_ad(60, 58);
        }

        if (((!s.b[260]) && s.b[266]) && s.b[267]) {
            s.store_sub(61, 184, 58);
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(18))));
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(18))));
            s.store_scalar(67, (1.0 - p.p47));
            s.store_sub_from_scalar(68, 1.0, 50);
            s.store_div_ad_lhs(62, A::mul_sub_from_scalar_rhs(s.ad_value(112), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67)))), 67);
            s.store_div_ad_lhs(63, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68)))), 68);
            s.store_div_ad_lhs(64, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68)))), 68);
            s.store_add_scaled_products_left_left_ad(108, A::add_scaled_inputs3(s.ad_value(62), 1.0, s.ad_value(63), 1.0, s.ad_value(64), -1.0), 18, 1.0, 53, 61, 1.0);
        }

        if (((!s.b[260]) && s.b[266]) && (!s.b[267])) {
            s.store_scalar(108, 0.0);
        }

        s.b[270] = (s.v[112] > 0.0);
        s.v[270] = if s.b[270] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && (!s.b[266])) && s.b[270]) {
            s.store_scale(76, 18, (1.0 - ((((-((2.4) as f64).ln()) / p.p47)) as f64).exp()));
            s.store_mul_sub_lhs(80, 76, 184, 3);
            s.store_sqrt_square_offset(81, 80, 1.921812);
            s.store_scaled_add(82, 80, 81, 0.5);
            s.store_add_scaled_product_indices(77, 76, 1.0, 2, 82, (-1.0));
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(18))));
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(79, 18, 1.0, A::exp_scaled_input(s.ad_value(78), (1.0 - p.p47)), 1.0 / ((1.0 - p.p47)));
            s.store_mul_add_scaled_inputs3_offset_rhs(108, 112, s.ad_value(79), 1.0, s.ad_value(184), 2.4, s.ad_value(77), (-2.4), 0.0);
        }

        if (((!s.b[260]) && (!s.b[266])) && (!s.b[270])) {
            s.store_scalar(108, 0.0);
        }

        if (!s.b[260]) {
            s.store_scale(113, 25, (1.0 - p.p49));
        }

        s.b[271] = (p.p48 < 100.0);
        s.v[271] = if s.b[271] { 1.0 } else { 0.0 };

        s.b[272] = (s.v[113] > 0.0);
        s.v[272] = if s.b[272] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && s.b[271]) && s.b[272]) {
            s.store_scalar(50, (p.p47 / 4.0));
            s.store_sub_from_scalar(51, p.p48, 18);
            s.store_scale(52, 18, (1.0 - ((((-((2.4) as f64).ln()) / p.p47)) as f64).exp()));
            s.store_scale(53, 113, 2.4);
            s.store_mul_exp_ad_rhs(54, 113, A::mul_offset_lhs(s.ad_value(50), (-p.p47), A::ln(A::div_from_scalar(p.p48, s.ad_value(18)))));
            s.store_mul_sub_lhs(56, 52, 183, 3);
        }

        s.b[273] = (s.v[56] < 80.0);
        s.v[273] = if s.b[273] { 1.0 } else { 0.0 };

        if ((((!s.b[260]) && s.b[271]) && s.b[272]) && s.b[273]) {
            s.store_exp(57, 56);
            s.store_add_scaled_product_right_ad(58, 52, 1.0, 2, A::ln(A::offset(s.ad_value(57), 1.0)), (-1.0));
        }

        if ((((!s.b[260]) && s.b[271]) && s.b[272]) && (!s.b[273])) {
            s.copy_ad(58, 183);
        }

        if (((!s.b[260]) && s.b[271]) && s.b[272]) {
            s.store_add_scaled_inputs(55, 51, 0.1, 2, 4.0);
            s.store_div_scaled_inputs2(59, s.ad_value(51), 1.0, s.ad_value(58), 1.0, s.ad_value(55), 1.0);
        }

        s.b[274] = (s.v[59] < 80.0);
        s.v[274] = if s.b[274] { 1.0 } else { 0.0 };

        if ((((!s.b[260]) && s.b[271]) && s.b[272]) && s.b[274]) {
            s.store_exp(57, 59);
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(51), -1.0, s.ad_value(52), -1.0, s.ad_value(55), 1.0)))), 51);
        }

        if ((((!s.b[260]) && s.b[271]) && s.b[272]) && (!s.b[274])) {
            s.copy_ad(60, 58);
        }

        if (((!s.b[260]) && s.b[271]) && s.b[272]) {
            s.store_sub(61, 183, 58);
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(18))));
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(18))));
            s.store_scalar(67, (1.0 - p.p47));
            s.store_sub_from_scalar(68, 1.0, 50);
            s.store_div_ad_lhs(62, A::mul_sub_from_scalar_rhs(s.ad_value(113), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67)))), 67);
            s.store_div_ad_lhs(63, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68)))), 68);
            s.store_div_ad_lhs(64, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68)))), 68);
            s.store_add_scaled_products_left_left_ad(105, A::add_scaled_inputs3(s.ad_value(62), 1.0, s.ad_value(63), 1.0, s.ad_value(64), -1.0), 18, 1.0, 53, 61, 1.0);
        }

        if (((!s.b[260]) && s.b[271]) && (!s.b[272])) {
            s.store_scalar(105, 0.0);
        }

        s.b[275] = (s.v[113] > 0.0);
        s.v[275] = if s.b[275] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && (!s.b[271])) && s.b[275]) {
            s.store_scale(76, 18, (1.0 - ((((-((2.4) as f64).ln()) / p.p47)) as f64).exp()));
            s.store_mul_sub_lhs(80, 76, 183, 3);
            s.store_sqrt_square_offset(81, 80, 1.921812);
            s.store_scaled_add(82, 80, 81, 0.5);
            s.store_add_scaled_product_indices(77, 76, 1.0, 2, 82, (-1.0));
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(18))));
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(79, 18, 1.0, A::exp_scaled_input(s.ad_value(78), (1.0 - p.p47)), 1.0 / ((1.0 - p.p47)));
            s.store_mul_add_scaled_inputs3_offset_rhs(105, 113, s.ad_value(79), 1.0, s.ad_value(183), 2.4, s.ad_value(77), (-2.4), 0.0);
        }

        if (((!s.b[260]) && (!s.b[271])) && (!s.b[275])) {
            s.store_scalar(105, 0.0);
        }

        s.b[276] = (p.p44 < 100.0);
        s.v[276] = if s.b[276] { 1.0 } else { 0.0 };

        s.b[277] = (s.v[111] > 0.0);
        s.v[277] = if s.b[277] { 1.0 } else { 0.0 };

        if (s.b[276] && s.b[277]) {
            s.store_scalar(50, (p.p43 / 4.0));
            s.store_sub_from_scalar(51, p.p44, 17);
            s.store_scale(52, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
            s.store_scale(53, 111, 2.4);
            s.store_mul_exp_ad_rhs(54, 111, A::mul_offset_lhs(s.ad_value(50), (-p.p43), A::ln(A::div_from_scalar(p.p44, s.ad_value(17)))));
            s.store_mul_sub_lhs(56, 52, 184, 3);
        }

        s.b[278] = (s.v[56] < 80.0);
        s.v[278] = if s.b[278] { 1.0 } else { 0.0 };

        if ((s.b[276] && s.b[277]) && s.b[278]) {
            s.store_exp(57, 56);
            s.store_add_scaled_product_right_ad(58, 52, 1.0, 2, A::ln(A::offset(s.ad_value(57), 1.0)), (-1.0));
        }

        if ((s.b[276] && s.b[277]) && (!s.b[278])) {
            s.copy_ad(58, 184);
        }

        if (s.b[276] && s.b[277]) {
            s.store_add_scaled_inputs(55, 51, 0.1, 2, 4.0);
            s.store_div_scaled_inputs2(59, s.ad_value(51), 1.0, s.ad_value(58), 1.0, s.ad_value(55), 1.0);
        }

        s.b[279] = (s.v[59] < 80.0);
        s.v[279] = if s.b[279] { 1.0 } else { 0.0 };

        if ((s.b[276] && s.b[277]) && s.b[279]) {
            s.store_exp(57, 59);
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(51), -1.0, s.ad_value(52), -1.0, s.ad_value(55), 1.0)))), 51);
        }

        if ((s.b[276] && s.b[277]) && (!s.b[279])) {
            s.copy_ad(60, 58);
        }

        if (s.b[276] && s.b[277]) {
            s.store_sub(61, 184, 58);
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(17))));
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(17))));
            s.store_scalar(67, (1.0 - p.p43));
            s.store_sub_from_scalar(68, 1.0, 50);
            s.store_div_ad_lhs(62, A::mul_sub_from_scalar_rhs(s.ad_value(111), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67)))), 67);
            s.store_div_ad_lhs(63, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68)))), 68);
            s.store_div_ad_lhs(64, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68)))), 68);
            s.store_add_scaled_products_left_left_ad(103, A::add_scaled_inputs3(s.ad_value(62), 1.0, s.ad_value(63), 1.0, s.ad_value(64), -1.0), 17, 1.0, 53, 61, 1.0);
        }

        if (s.b[276] && (!s.b[277])) {
            s.store_scalar(103, 0.0);
        }

        s.b[280] = (s.v[111] > 0.0);
        s.v[280] = if s.b[280] { 1.0 } else { 0.0 };

        if ((!s.b[276]) && s.b[280]) {
            s.store_scale(76, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
            s.store_mul_sub_lhs(80, 76, 184, 3);
            s.store_sqrt_square_offset(81, 80, 1.921812);
            s.store_scaled_add(82, 80, 81, 0.5);
        }

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[276]) && s.b[280]) {
            s.store_add_scaled_product_indices(77, 76, 1.0, 2, 82, (-1.0));
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(17))));
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(79, 17, 1.0, A::exp_scaled_input(s.ad_value(78), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
            s.store_mul_add_scaled_inputs3_offset_rhs(103, 111, s.ad_value(79), 1.0, s.ad_value(184), 2.4, s.ad_value(77), (-2.4), 0.0);
        }

        if ((!s.b[276]) && (!s.b[280])) {
            s.store_scalar(103, 0.0);
        }

        s.store_add(106, 103, 108);

        s.b[281] = (s.v[111] > 0.0);
        s.v[281] = if s.b[281] { 1.0 } else { 0.0 };

        if s.b[281] {
            s.store_scale(282, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
            s.store_mul_sub_lhs(283, 282, 184, 3);
            s.store_sqrt_square_offset(284, 283, 1.921812);
            s.store_scaled_add(285, 283, 284, 0.5);
            s.store_add_scaled_product_indices(286, 282, 1.0, 2, 285, (-1.0));
            s.store_div(287, 285, 284);
            s.store_add_ad(107, A::mul3(s.ad_value(111), A::exp_scaled_input(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(286), s.ad_value(17)))), (-p.p43)), s.ad_value(287)), A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(111), 1.0, s.ad_value(287), 2.4));
        }

        if (!s.b[281]) {
            s.store_scalar(107, 0.0);
        }

        s.b[288] = (p.p65 > 0.0);
        s.v[288] = if s.b[288] { 1.0 } else { 0.0 };

        if s.b[288] {
            s.store_sub(143, 38, 184);
        }

        if (!s.b[288]) {
            s.store_sub(143, 186, 34);
        }

        s.store_offset_mul(289, 143, 3, (-1.0));

        s.store_mul_offset_ad_lhs(290, A::add_scaled_inputs(s.ad_value(289), 0.5, A::sqrt(A::offset(A::square(s.ad_value(289)), 1.921812)), 0.5), 1.0, 2);

        s.store_div(291, 290, 33);

        s.store_mul(292, 290, 32);

        s.store_exp_scaled_input_ad(293, A::ln_one_plus_exp(A::scale(A::ln(s.ad_value(291)), p.p67)), 1.0 / (p.p67));

        s.store_div(294, 292, 293);

        s.store_scaled_sub(295, 290, 33, 1.0 / (p.p63));

        s.store_mul_offset_ad_rhs(142, 294, A::add_scaled_inputs(s.ad_value(295), 0.5, A::sqrt(A::offset(A::square(s.ad_value(295)), p.p66)), 0.5), 1.0);

        s.b[296] = ((s.v[107] > 0.0) && (s.v[111] > 0.0));
        s.v[296] = if s.b[296] { 1.0 } else { 0.0 };

        if s.b[296] {
            s.store_div(114, 111, 107);
            s.store_div(103, 103, 111);
        }

        if (!s.b[296]) {
            s.store_scalar(114, 1.0);
            s.store_scalar(103, 0.0);
        }

        s.b[297] = (s.v[23] > 0.0);
        s.v[297] = if s.b[297] { 1.0 } else { 0.0 };

        if s.b[297] {
            s.store_mul_sub_from_scalar_ad_rhs(76, 16, 1.0, A::exp_scaled_input(A::ln(s.ad_value(43)), (-1.0 / (p.p36))));
            s.store_mul_sub_lhs(80, 76, 185, 3);
            s.store_sqrt_square_offset(81, 80, 1.921812);
            s.store_scaled_add(82, 80, 81, 0.5);
            s.store_add_scaled_product_indices(77, 76, 1.0, 2, 82, (-1.0));
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(16))));
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(79, 16, 1.0, A::exp_scaled_input(s.ad_value(78), (1.0 - p.p36)), 1.0 / ((1.0 - p.p36)));
            s.store_mul_add_scaled_product_rhs(98, 23, s.ad_value(79), 1.0, s.ad_value(43), A::sub(s.ad_value(185), s.ad_value(77)), 1.0);
        }

        if (!s.b[297]) {
            s.store_scalar(98, 0.0);
        }

        s.store_div(102, 98, 23);

        s.b[298] = (p.p0 <= 200.0);
        s.v[298] = if s.b[298] { 1.0 } else { 0.0 };

        s.b[299] = (s.v[26] > 0.0);
        s.v[299] = if s.b[299] { 1.0 } else { 0.0 };

        if (s.b[298] && s.b[299]) {
            s.store_mul_sub_from_scalar_ad_rhs(76, 22, 1.0, A::exp_scaled_input(A::ln(s.ad_value(44)), (-1.0 / (p.p39))));
            s.store_mul_sub_lhs(80, 76, 185, 3);
            s.store_sqrt_square_offset(81, 80, 1.921812);
            s.store_scaled_add(82, 80, 81, 0.5);
            s.store_add_scaled_product_indices(77, 76, 1.0, 2, 82, (-1.0));
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(22))));
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(79, 22, 1.0, A::exp_scaled_input(s.ad_value(78), (1.0 - p.p39)), 1.0 / ((1.0 - p.p39)));
            s.store_mul_add_scaled_product_rhs(100, 26, s.ad_value(79), 1.0, s.ad_value(44), A::sub(s.ad_value(185), s.ad_value(77)), 1.0);
        }

        if (s.b[298] && (!s.b[299])) {
            s.store_scalar(100, 0.0);
        }

        if s.b[298] {
            s.store_div(101, 100, 26);
            s.copy_ad(20, 22);
            s.store_scalar(21, p.p39);
        }

        if (!s.b[298]) {
            s.copy_ad(101, 102);
            s.copy_ad(20, 16);
            s.store_scalar(21, p.p36);
        }

        s.b[300] = (p.p7 == 0.0);
        s.v[300] = if s.b[300] { 1.0 } else { 0.0 };

        if s.b[300] {
            s.store_scalar(201, 1.0);
        }

        if (!s.b[300]) {
            s.store_scale(301, 2, p.p8);
            s.store_div_scaled_inputs2(302, s.ad_value(20), 1.0, s.ad_value(185), (-1.0), s.ad_value(301), 1.0);
            s.store_add_scaled_product_right_ad(303, 20, 1.0, 301, A::add(s.ad_value(302), A::sqrt(A::offset(A::square(s.ad_value(302)), 1.921812))), (-0.5));
            s.store_mul_sub_from_scalar_ad_rhs(304, 200, 1.0, A::exp(A::mul(s.ad_value(21), A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(303), s.ad_value(20)))))));
        }

        s.b[305] = (((s.v[304]) as f64).abs() >= 0.001);
        s.v[305] = if s.b[305] { 1.0 } else { 0.0 };

        if ((!s.b[300]) && s.b[305]) {
            s.store_div_scaled_offset_numerator(201, A::exp(s.ad_value(304)), 1.0, (-1.0), s.ad_value(304), 1.0);
        }

        if ((!s.b[300]) && (!s.b[305])) {
            s.store_offset_scaled(201, 304, 0.5, 1.0);
        }

        s.store_mul(159, 201, 101);

        s.store_add_scaled_ad_lhs(116, A::offset(A::div(s.ad_value(159), s.ad_value(202)), 1.0), 103, 1.0 / (p.p5));

        s.store_offset_scaled(131, 116, 20.0, (-1.0));

        s.store_scaled_offset_ad(115, A::add_scaled_inputs(s.ad_value(131), 0.5, A::sqrt(A::offset(A::square(s.ad_value(131)), 1.921812)), 0.5), 1.0, 0.025);

        s.store_add_scaled_inputs3_offset(117, s.ad_value(42), 1.0, s.ad_value(114), p.p55, A::div_from_scalar(1.0, s.ad_value(114)), p.p56, (((-1.0) * p.p55) + ((-1.0) * p.p56)));

        s.b[306] = (p.p10 == 1.0);
        s.v[306] = if s.b[306] { 1.0 } else { 0.0 };

        if s.b[306] {
            s.store_offset_div(130, 117, 42, (-1.0));
            s.store_div_scaled_value_offset_denominator(118, s.ad_value(15), 1.0, s.ad_value(130), 1.0, 1.0);
        }

        if (!s.b[306]) {
            s.copy_ad(118, 15);
        }

        s.v[119] = p.p11;

        s.store_div_scaled_inputs(180, s.ad_value(185), 1.0, s.ad_value(2), p.p3);

        s.b[307] = (s.v[180] > 80.0);
        s.v[307] = if s.b[307] { 1.0 } else { 0.0 };

        if s.b[307] {
            s.store_offset(179, 180, (((-80.0)) + (1.0)));
            s.store_scalar(180, 80.0);
        }

        if (!s.b[307]) {
            s.store_scalar(179, 1.0);
        }

        s.store_mul_limexp_rhs(179, 179, 180);

        s.store_mul(120, 11, 179);

        s.store_div_scaled_inputs(182, s.ad_value(184), 1.0, s.ad_value(2), p.p4);

        s.b[308] = (s.v[182] > 80.0);
        s.v[308] = if s.b[308] { 1.0 } else { 0.0 };

        if s.b[308] {
            s.store_offset(181, 182, (((-80.0)) + (1.0)));
            s.store_scalar(182, 80.0);
        }

        if (!s.b[308]) {
            s.store_scalar(181, 1.0);
        }

        s.store_mul_limexp_rhs(181, 181, 182);

        s.store_mul(121, 11, 181);

        s.b[309] = (p.p13 != 0.0);
        s.v[309] = if s.b[309] { 1.0 } else { 0.0 };

        if s.b[309] {
            s.store_add_scaled_inputs3(123, A::div(s.ad_value(120), s.ad_value(118)), 1.0, s.ad_value(121), 1.0 / (s.v[119]), A::exp_scaled_input(A::ln(A::mul3(s.ad_value(120), A::div(s.ad_value(120), s.ad_value(142)), A::div(s.ad_value(205), s.ad_value(203)))), 0.6666), 1.0);
            s.store_add_scaled_inputs4(124, A::div(s.ad_value(120), s.ad_value(118)), 1.0, s.ad_value(121), 1.0 / (s.v[119]), A::div(s.ad_value(120), s.ad_value(203)), 1.0, A::exp_scaled_input(A::ln(A::mul3(s.ad_value(120), A::div(s.ad_value(120), s.ad_value(142)), A::div(s.ad_value(205), s.ad_value(203)))), 0.6666), 1.0);
        }

        if (!s.b[309]) {
            s.store_add_scaled_ad_lhs(123, A::div(s.ad_value(120), s.ad_value(118)), 121, 1.0 / (s.v[119]));
            s.store_add_scaled_inputs3(124, A::div(s.ad_value(120), s.ad_value(118)), 1.0, s.ad_value(121), 1.0 / (s.v[119]), A::div(s.ad_value(120), s.ad_value(203)), 1.0);
        }

        s.store_add_ad_rhs(128, 115, A::sqrt(A::add(A::square(s.ad_value(115)), s.ad_value(123))));

        s.store_add_ad_rhs(129, 115, A::sqrt(A::add(A::square(s.ad_value(115)), s.ad_value(124))));

        let assign4950_e5324: f64 = (s.v[124] - s.v[123]);
        s.v[207] = assign4950_e5324;

        s.b[310] = (((s.v[207]) as f64).abs() > 1e-8);
        s.v[310] = if s.b[310] { 1.0 } else { 0.0 };

        if s.b[310] {
            s.store_sub_from_scalar_ad(150, 1.0, A::mul(A::div(A::div_scaled_value_offset_denominator(s.ad_value(142), 1.0, s.ad_value(206), 1.0, 1.0), s.ad_value(120)), s.ad_value(128)));
            s.store_offset_mul_ad(151, A::div(A::div_scaled_value_offset_denominator(s.ad_value(142), 1.0, s.ad_value(206), 1.0, 1.0), s.ad_value(120)), A::sub(s.ad_value(129), s.ad_value(128)), 1.0);
            s.store_div(149, 150, 151);
            s.store_scaled_add_ad_lhs(146, A::sqrt(A::offset(A::square(s.ad_value(149)), 0.01)), 149, 1.0 / ((1.0 + (((1.0 + 0.01)) as f64).sqrt())));
        }

        if (!s.b[310]) {
            s.store_scalar(146, 0.0);
        }

        s.b[311] = (p.p2 == 0.0);
        s.v[311] = if s.b[311] { 1.0 } else { 0.0 };

        s.b[312] = (p.p13 != 0.0);
        s.v[312] = if s.b[312] { 1.0 } else { 0.0 };

        if (s.b[311] && s.b[312]) {
            s.store_add_scaled_inputs4(122, A::div(s.ad_value(120), s.ad_value(118)), 1.0, s.ad_value(121), 1.0 / (s.v[119]), A::mul3(A::div(s.ad_value(120), s.ad_value(203)), s.ad_value(146), s.ad_value(146)), 1.0, A::exp_scaled_input(A::ln(A::mul3(s.ad_value(120), A::div(s.ad_value(120), s.ad_value(142)), A::div(s.ad_value(205), s.ad_value(203)))), 0.6666), 1.0);
        }

        if (s.b[311] && (!s.b[312])) {
            s.store_add_scaled_inputs3(122, A::div(s.ad_value(120), s.ad_value(118)), 1.0, s.ad_value(121), 1.0 / (s.v[119]), A::mul3(A::div(s.ad_value(120), s.ad_value(203)), s.ad_value(146), s.ad_value(146)), 1.0);
        }

        if s.b[311] {
            s.store_add_ad_rhs(125, 115, A::sqrt(A::add(A::square(s.ad_value(115)), s.ad_value(122))));
        }

        if (!s.b[311]) {
            s.store_scalar(83, (1.0 / 3.0));
            s.store_scale(84, 115, (-2.0));
        }

        s.b[313] = ((p.p9 == 1000000.0) && (p.p12 == 1000000.0));
        s.v[313] = if s.b[313] { 1.0 } else { 0.0 };

        if ((!s.b[311]) && s.b[313]) {
            s.store_scalar(85, 0.0);
        }

        if ((!s.b[311]) && (!s.b[313])) {
            s.store_neg_ad(85, A::add_scaled_inputs3(A::div(s.ad_value(120), s.ad_value(118)), 1.0, s.ad_value(121), 1.0 / (s.v[119]), A::mul3(A::div(s.ad_value(120), s.ad_value(203)), s.ad_value(146), s.ad_value(146)), 1.0));
        }

        if (!s.b[311]) {
            s.store_div_scaled_product_left_ad(86, A::div_scaled_product(s.ad_value(120), s.ad_value(120), -1.0, s.ad_value(142), 1.0), 205, 1.0, 203, 1.0);
            s.store_square(87, 84);
            s.store_add_scaled_product_indices(88, 85, 1.0, 87, 83, (-1.0));
            s.store_add_ad_lhs(89, A::add_scaled_product(A::mul3(s.ad_value(84), s.ad_value(85), s.ad_value(83)), (-1.0), s.ad_value(84), s.ad_value(87), (2.0 * 0.037037037037037035)), 86);
            s.store_add_scaled_square_product_mixed_iai(90, 89, 0.25, A::square(s.ad_value(88)), 88, 0.037037037037037035);
        }

        s.b[314] = (((s.v[90]) as f64).abs() < 1e-10);
        s.v[314] = if s.b[314] { 1.0 } else { 0.0 };

        if ((!s.b[311]) && s.b[314]) {
            s.store_add_scaled_product_value_ad(91, A::div_scaled_inputs(s.ad_value(89), 3.0, s.ad_value(88), 1.0), 1.0, 84, 83, (-1.0));
        }

        s.b[315] = (s.v[90] > 0.0);
        s.v[315] = if s.b[315] { 1.0 } else { 0.0 };

        if (((!s.b[311]) && (!s.b[314])) && s.b[315]) {
            s.store_scale(92, 89, (-0.5));
            s.store_sqrt(93, 90);
            s.store_add(87, 92, 93);
        }

        s.b[316] = (s.v[87] > 0.0);
        s.v[316] = if s.b[316] { 1.0 } else { 0.0 };

        if ((((!s.b[311]) && (!s.b[314])) && s.b[315]) && s.b[316]) {
            s.store_exp_ad(94, A::mul(s.ad_value(83), A::ln(s.ad_value(87))));
        }

        if ((((!s.b[311]) && (!s.b[314])) && s.b[315]) && (!s.b[316])) {
            s.store_neg_ad(94, A::exp(A::mul(s.ad_value(83), A::ln_scaled_input(s.ad_value(87), -1.0))));
        }

        if (((!s.b[311]) && (!s.b[314])) && s.b[315]) {
            s.store_sub(87, 92, 93);
        }

        s.b[317] = (s.v[87] > 0.0);
        s.v[317] = if s.b[317] { 1.0 } else { 0.0 };

        if ((((!s.b[311]) && (!s.b[314])) && s.b[315]) && s.b[317]) {
            s.store_exp_ad(95, A::mul(s.ad_value(83), A::ln(s.ad_value(87))));
        }

        if ((((!s.b[311]) && (!s.b[314])) && s.b[315]) && (!s.b[317])) {
            s.store_neg_ad(95, A::exp(A::mul(s.ad_value(83), A::ln_scaled_input(s.ad_value(87), -1.0))));
        }

        if (((!s.b[311]) && (!s.b[314])) && s.b[315]) {
            s.store_add_scaled_inputs_product_indices(91, 94, 1.0, 95, 1.0, 84, 83, (-1.0));
        }

        if (((!s.b[311]) && (!s.b[314])) && (!s.b[315])) {
            s.store_mul_scaled_ad_rhs(87, 89, (-0.5), A::sqrt(A::div_from_scalar((-27.0), A::mul(A::square(s.ad_value(88)), s.ad_value(88)))));
            s.store_square(92, 87);
        }

        s.b[318] = (s.v[87] >= 0.0);
        s.v[318] = if s.b[318] { 1.0 } else { 0.0 };

        if ((((!s.b[311]) && (!s.b[314])) && (!s.b[315])) && s.b[318]) {
            s.store_sub_from_scalar_ad(87, (3.141592653589793 / 2.0), A::atan(A::sqrt(A::div(s.ad_value(92), A::sub_from_scalar(1.0, s.ad_value(92))))));
        }

        if ((((!s.b[311]) && (!s.b[314])) && (!s.b[315])) && (!s.b[318])) {
            s.store_offset_atan_ad(87, A::sqrt(A::div(s.ad_value(92), A::sub_from_scalar(1.0, s.ad_value(92)))), (3.141592653589793 / 2.0));
        }

        if (((!s.b[311]) && (!s.b[314])) && (!s.b[315])) {
            s.store_add_scaled_products_mixed_aaii(87, A::sqrt(A::mul_scaled_lhs(s.ad_value(88), (-4.0), s.ad_value(83))), A::cos(A::mul(s.ad_value(83), s.ad_value(87))), 1.0, 84, 83, (-1.0));
            s.copy_ad(91, 87);
        }

        if (!s.b[311]) {
            s.copy_ad(125, 91);
        }

        s.b[319] = (s.v[125] < 1e-20);
        s.v[319] = if s.b[319] { 1.0 } else { 0.0 };

        if s.b[319] {
            s.store_scalar(125, 1e-20);
        }

        s.store_div(126, 120, 125);

    }

    pub(super) fn stamp_transient_block_3(
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
        let nv4 = ctx.node_voltage(nodes[4]);
        s.store_div(127, 121, 125);

        s.b[320] = (s.v[126] < 1e-20);
        s.v[320] = if s.b[320] { 1.0 } else { 0.0 };

        if s.b[320] {
            s.store_scalar(126, 1e-20);
        }

        s.store_mul(138, 117, 126);

        s.store_sub_from_scalar_ad(147, 1.0, A::div(s.ad_value(142), s.ad_value(126)));

        s.store_sqrt_square_offset(144, 147, p.p60);

        s.store_scaled_add(145, 147, 144, 1.0 / ((1.0 + (((1.0 + p.p60)) as f64).sqrt())));

        s.store_mul3_lhs(148, 35, 145, 145);

        s.store_mul(139, 148, 126);

        s.store_mul_ad_rhs(141, 36, A::exp_scaled_input(A::ln(A::div(s.ad_value(126), s.ad_value(142))), p.p58));

        s.store_scaled_mul(140, 141, 126, 1.0 / ((p.p58 + 1.0)));

        s.store_add_scaled_inputs3(137, s.ad_value(138), 1.0, s.ad_value(140), 1.0, s.ad_value(139), 1.0);

        s.store_scale(152, 127, p.p68);

        s.v[47] = p.p44;

        s.b[327] = (s.v[47] < 100.0);
        s.v[327] = if s.b[327] { 1.0 } else { 0.0 };

        s.b[328] = (s.v[24] > 0.0);
        s.v[328] = if s.b[328] { 1.0 } else { 0.0 };

        if (s.b[327] && s.b[328]) {
            s.store_scalar(50, (p.p43 / 4.0));
            s.store_sub_from_scalar(51, s.v[47], 17);
            s.store_scale(52, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
            s.store_scale(53, 24, 2.4);
            s.store_mul_exp_ad_rhs(54, 24, A::mul_offset_lhs(s.ad_value(50), (-p.p43), A::ln(A::div_from_scalar(s.v[47], s.ad_value(17)))));
            s.store_mul_sub_lhs(56, 52, 184, 3);
        }

        s.b[329] = (s.v[56] < 80.0);
        s.v[329] = if s.b[329] { 1.0 } else { 0.0 };

        if ((s.b[327] && s.b[328]) && s.b[329]) {
            s.store_exp(57, 56);
            s.store_add_scaled_product_right_ad(58, 52, 1.0, 2, A::ln(A::offset(s.ad_value(57), 1.0)), (-1.0));
        }

        if ((s.b[327] && s.b[328]) && (!s.b[329])) {
            s.copy_ad(58, 184);
        }

        if (s.b[327] && s.b[328]) {
            s.store_add_scaled_inputs(55, 51, 0.1, 2, 4.0);
            s.store_div_scaled_inputs2(59, s.ad_value(51), 1.0, s.ad_value(58), 1.0, s.ad_value(55), 1.0);
        }

        s.b[330] = (s.v[59] < 80.0);
        s.v[330] = if s.b[330] { 1.0 } else { 0.0 };

        if ((s.b[327] && s.b[328]) && s.b[330]) {
            s.store_exp(57, 59);
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(51), -1.0, s.ad_value(52), -1.0, s.ad_value(55), 1.0)))), 51);
        }

        if ((s.b[327] && s.b[328]) && (!s.b[330])) {
            s.copy_ad(60, 58);
        }

        if (s.b[327] && s.b[328]) {
            s.store_sub(61, 184, 58);
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(17))));
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(17))));
            s.store_scalar(67, (1.0 - p.p43));
            s.store_sub_from_scalar(68, 1.0, 50);
            s.store_div_ad_lhs(62, A::mul_sub_from_scalar_rhs(s.ad_value(24), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67)))), 67);
            s.store_div_ad_lhs(63, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68)))), 68);
            s.store_div_ad_lhs(64, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68)))), 68);
        }

        s.b[331] = (s.v[24] > 0.0);
        s.v[331] = if s.b[331] { 1.0 } else { 0.0 };

        if ((!s.b[327]) && s.b[331]) {
            s.store_scale(76, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
            s.store_mul_sub_lhs(80, 76, 184, 3);
            s.store_sqrt_square_offset(81, 80, 1.921812);
            s.store_scaled_add(82, 80, 81, 0.5);
            s.store_add_scaled_product_indices(77, 76, 1.0, 2, 82, (-1.0));
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(17))));
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(79, 17, 1.0, A::exp_scaled_input(s.ad_value(78), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
        }

        s.b[349] = (p.p53 < 100.0);
        s.v[349] = if s.b[349] { 1.0 } else { 0.0 };

        s.b[350] = (s.v[30] > 0.0);
        s.v[350] = if s.b[350] { 1.0 } else { 0.0 };

        if (s.b[349] && s.b[350]) {
            s.store_scalar(50, (p.p52 / 4.0));
            s.store_sub_from_scalar(51, p.p53, 19);
            s.store_scale(52, 19, (1.0 - ((((-((2.4) as f64).ln()) / p.p52)) as f64).exp()));
            s.store_scale(53, 30, 2.4);
            s.store_mul_exp_ad_rhs(54, 30, A::mul_offset_lhs(s.ad_value(50), (-p.p52), A::ln(A::div_from_scalar(p.p53, s.ad_value(19)))));
            s.store_mul_sub_lhs(56, 52, 187, 3);
        }

        s.b[351] = (s.v[56] < 80.0);
        s.v[351] = if s.b[351] { 1.0 } else { 0.0 };

        if ((s.b[349] && s.b[350]) && s.b[351]) {
            s.store_exp(57, 56);
            s.store_add_scaled_product_right_ad(58, 52, 1.0, 2, A::ln(A::offset(s.ad_value(57), 1.0)), (-1.0));
        }

        if ((s.b[349] && s.b[350]) && (!s.b[351])) {
            s.copy_ad(58, 187);
        }

        if (s.b[349] && s.b[350]) {
            s.store_add_scaled_inputs(55, 51, 0.1, 2, 4.0);
            s.store_div_scaled_inputs2(59, s.ad_value(51), 1.0, s.ad_value(58), 1.0, s.ad_value(55), 1.0);
        }

        s.b[352] = (s.v[59] < 80.0);
        s.v[352] = if s.b[352] { 1.0 } else { 0.0 };

        if ((s.b[349] && s.b[350]) && s.b[352]) {
            s.store_exp(57, 59);
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(51), -1.0, s.ad_value(52), -1.0, s.ad_value(55), 1.0)))), 51);
        }

        if ((s.b[349] && s.b[350]) && (!s.b[352])) {
            s.copy_ad(60, 58);
        }

        if (s.b[349] && s.b[350]) {
            s.store_sub(61, 187, 58);
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(19))));
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(19))));
            s.store_scalar(67, (1.0 - p.p52));
            s.store_sub_from_scalar(68, 1.0, 50);
            s.store_div_ad_lhs(62, A::mul_sub_from_scalar_rhs(s.ad_value(30), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67)))), 67);
            s.store_div_ad_lhs(63, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68)))), 68);
            s.store_div_ad_lhs(64, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68)))), 68);
            s.store_add_scaled_products_left_left_ad(162, A::add_scaled_inputs3(s.ad_value(62), 1.0, s.ad_value(63), 1.0, s.ad_value(64), -1.0), 19, 1.0, 53, 61, 1.0);
        }

        if (s.b[349] && (!s.b[350])) {
            s.store_scalar(162, 0.0);
        }

        s.b[353] = (s.v[30] > 0.0);
        s.v[353] = if s.b[353] { 1.0 } else { 0.0 };

        if ((!s.b[349]) && s.b[353]) {
            s.store_scale(76, 19, (1.0 - ((((-((2.4) as f64).ln()) / p.p52)) as f64).exp()));
            s.store_mul_sub_lhs(80, 76, 187, 3);
            s.store_sqrt_square_offset(81, 80, 1.921812);
            s.store_scaled_add(82, 80, 81, 0.5);
            s.store_add_scaled_product_indices(77, 76, 1.0, 2, 82, (-1.0));
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(19))));
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(79, 19, 1.0, A::exp_scaled_input(s.ad_value(78), (1.0 - p.p52)), 1.0 / ((1.0 - p.p52)));
            s.store_mul_add_scaled_inputs3_offset_rhs(162, 30, s.ad_value(79), 1.0, s.ad_value(187), 2.4, s.ad_value(77), (-2.4), 0.0);
        }

        if ((!s.b[349]) && (!s.b[353])) {
            s.store_scalar(162, 0.0);
        }

        s.copy_ad(208, 137);

        s.copy_ad(211, 126);

        s.b[355] = ((p.p73 != 0.0) && (p.p54 != 0.0));
        s.v[355] = if s.b[355] { 1.0 } else { 0.0 };

        if s.b[355] {
            s.store_voltage(208, ctx, nodes, Some(8), None);
            s.store_scale(210, 208, (p.p71 * p.p54));
            s.store_voltage(211, ctx, nodes, Some(9), None);
            s.store_scale(213, 211, (p.p72 * p.p54));
        }

        if (!s.b[355]) {
            s.store_scalar(210, 0.0);
            s.store_scalar(213, 0.0);
        }

        s.store_scale(196, 183, p.p70);

        s.store_scale(197, 191, p.p69);

        s.store_add(198, 106, 152);

        s.store_add(199, 98, 208);

        s.store_scale(162, 162, p.p110);

        s.store_scale(105, 105, p.p110);

        s.store_scale(196, 196, p.p110);

        s.store_scale(197, 197, p.p110);

        s.store_scale(198, 198, p.p110);

        s.store_scale(199, 199, p.p110);

        s.b[359] = ((p.p103 == 0.0) || (p.p107 == 0.0));
        s.v[359] = if s.b[359] { 1.0 } else { 0.0 };

        if s.b[359] {
            s.store_scalar(167, 0.0);
        }

        if (!s.b[359]) {
            s.store_ddt_scaled_voltage(167, Some(4), None, p.p107, ddt_scale, eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, ((nv4) * (p.p107))));
        }

        s.b[360] = ((p.p103 == 0.0) || (p.p104 < p.p111));
        s.v[360] = if s.b[360] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.store_scaled_voltage(183, ctx, nodes, Some(1), Some(5), p.p110);

        s.store_scaled_voltage(184, ctx, nodes, Some(6), Some(5), p.p110);

        s.store_scaled_voltage(185, ctx, nodes, Some(6), Some(7), p.p110);

        s.store_sub(186, 185, 184);

        s.store_scaled_voltage(187, ctx, nodes, Some(3), Some(5), p.p110);

        s.store_scaled_voltage(191, ctx, nodes, Some(1), Some(2), p.p110);

        s.v[8] = (p.p108 + 273.15);

        s.v[9] = ctx_temp;

        s.v[177] = ((1.3806226e-23 * s.v[8]) / 1.602176462e-19);

        s.v[172] = (p.p88 * s.v[8]);

        s.v[173] = (0.5 * (p.p76 + p.p77));

        s.v[174] = (0.5 * (p.p76 + p.p78));

        s.v[175] = (0.5 * (p.p79 + p.p78));

        s.v[168] = (3.0 - ((1.602176462e-19 * p.p80) / 1.3806226e-23));

        s.v[171] = ((p.p82 - p.p81) - 0.5);

        s.v[176] = (p.p76 - p.p77);

        s.v[27] = p.p34;

        s.v[4] = (s.v[9] + p.p109);

        s.b[247] = (s.v[4] < ((-100.0) + 273.15));
        s.v[247] = if s.b[247] { 1.0 } else { 0.0 };

        if s.b[247] {
            s.store_scalar(4, ((-100.0) + 273.15));
        }

        s.b[248] = (s.v[4] > (326.85 + 273.15));
        s.v[248] = if s.b[248] { 1.0 } else { 0.0 };

        if ((!s.b[247]) && s.b[248]) {
            s.store_scalar(4, (326.85 + 273.15));
        }

        s.store_scale(2, 4, (1.3806226e-23 * 6.241509744511525e18));

        s.store_div_from_scalar(3, 1.0, 2);

        s.store_offset(7, 4, (-s.v[8]));

        s.store_scale(5, 4, 1.0 / (s.v[8]));

        s.store_ln(6, 5);

        s.store_mul_offset_rhs(10, 3, 5, (-1.0));

        s.v[178] = ((0.5 * p.p35) / s.v[177]);

        s.v[96] = ((2.0 * s.v[177]) * (((((s.v[178]) as f64).exp() - (((-s.v[178])) as f64).exp())) as f64).ln());

        s.store_add_scaled_inputs_product_second_ad(97, 5, s.v[96], A::scale_offset(s.ad_value(5), (-s.v[173]), s.v[173]), 1.0, 2, 6, (-s.v[168]));

        s.store_add_scaled_product_right_ad(16, 97, 1.0, 2, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(97), -1.0, s.ad_value(3))), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scale_ad(23, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p35, s.ad_value(16))), p.p36), p.p34);

        s.store_scale(43, 16, (p.p37 * 1.0 / (p.p35)));

        s.v[178] = ((0.5 * p.p38) / s.v[177]);

        s.v[96] = ((2.0 * s.v[177]) * (((((s.v[178]) as f64).exp() - (((-s.v[178])) as f64).exp())) as f64).ln());

        s.store_add_scaled_inputs_product_second_ad(97, 5, s.v[96], A::scale_offset(s.ad_value(5), (-s.v[173]), s.v[173]), 1.0, 2, 6, (-s.v[168]));

        s.store_add_scaled_product_right_ad(22, 97, 1.0, 2, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(97), -1.0, s.ad_value(3))), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scale_ad(26, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p38, s.ad_value(22))), p.p39), s.v[27]);

        s.store_scale(44, 22, (p.p40 * 1.0 / (p.p38)));

        s.v[178] = ((0.5 * p.p42) / s.v[177]);

        s.v[96] = ((2.0 * s.v[177]) * (((((s.v[178]) as f64).exp() - (((-s.v[178])) as f64).exp())) as f64).ln());

        s.store_add_scaled_inputs_product_second_ad(97, 5, s.v[96], A::scale_offset(s.ad_value(5), (-s.v[174]), s.v[174]), 1.0, 2, 6, (-s.v[168]));

        s.store_add_scaled_product_right_ad(17, 97, 1.0, 2, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(97), -1.0, s.ad_value(3))), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scale_ad(24, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p42, s.ad_value(17))), p.p43), p.p41);

        s.store_scaled_exp_ad(11, A::add_scaled_inputs(s.ad_value(6), p.p81, s.ad_value(10), p.p76), p.p1);

        s.store_scaled_exp_ad(15, A::sub_scaled_inputs(s.ad_value(6), p.p95, s.ad_value(10), p.p83), p.p9);

        s.store_scaled_exp_scaled_input(33, 6, (p.p87 - s.v[172]), p.p62);

        s.store_scaled_exp_scaled_input(31, 6, p.p87, p.p61);

        s.store_div_from_scalar(32, 1.0, 31);

        s.store_offset_scaled(34, 7, ((p.p89) * (p.p64)), p.p64);

        s.b[249] = (p.p65 > 0.0);
        s.v[249] = if s.b[249] { 1.0 } else { 0.0 };

        if s.b[249] {
            s.store_offset_scaled_ad(38, A::scale(s.ad_value(7), p.p90), (-p.p65), p.p65);
            s.store_scalar(34, p.p64);
        }

        if (!s.b[249]) {
            s.store_offset_scaled(34, 7, ((p.p89) * (p.p64)), p.p64);
            s.store_scalar(38, p.p65);
        }

        s.store_add_scaled_product_value_ad(42, A::scale_offset(s.ad_value(7), p.p85, 1.0), p.p54, 7, 7, (p.p86 * p.p54));

        s.b[250] = (p.p96 == 1.0);
        s.v[250] = if s.b[250] { 1.0 } else { 0.0 };

        if s.b[250] {
            s.store_scaled_exp_ad(36, A::sub_scaled_inputs(s.ad_value(6), s.v[171], s.ad_value(10), s.v[176]), p.p57);
        }

        if (!s.b[250]) {
            s.store_scalar(36, p.p57);
        }

        s.store_scaled_exp_scaled_input(35, 6, (p.p87 - 1.0), p.p59);

        s.v[178] = ((0.5 * p.p46) / s.v[177]);

        s.v[96] = ((2.0 * s.v[177]) * (((((s.v[178]) as f64).exp() - (((-s.v[178])) as f64).exp())) as f64).ln());

        s.store_add_scaled_inputs_product_second_ad(97, 5, s.v[96], A::scale_offset(s.ad_value(5), (-s.v[174]), s.v[174]), 1.0, 2, 6, (-s.v[168]));

        s.store_add_scaled_product_right_ad(18, 97, 1.0, 2, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(97), -1.0, s.ad_value(3))), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scale_ad(25, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p46, s.ad_value(18))), p.p47), p.p45);

        s.v[178] = ((0.5 * p.p51) / s.v[177]);

        s.v[96] = ((2.0 * s.v[177]) * (((((s.v[178]) as f64).exp() - (((-s.v[178])) as f64).exp())) as f64).ln());

        s.store_add_scaled_inputs_product_second_ad(97, 5, s.v[96], A::scale_offset(s.ad_value(5), (-s.v[175]), s.v[175]), 1.0, 2, 6, (-s.v[168]));

        s.store_add_scaled_product_right_ad(19, 97, 1.0, 2, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(97), -1.0, s.ad_value(3))), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scale_ad(30, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p51, s.ad_value(19))), p.p52), p.p50);

        s.store_scaled_exp_scaled_input(200, 6, p.p97, p.p7);

        s.store_div_from_scalar_exp_ad(202, p.p6, A::mul_scaled_lhs(s.ad_value(3), p.p83, A::offset(A::exp_scaled_input(s.ad_value(6), p.p84), (-1.0))));

        s.b[252] = (p.p0 <= 200.0);
        s.v[252] = if s.b[252] { 1.0 } else { 0.0 };

        if s.b[252] {
            s.store_offset_mul_ad(204, s.ad_value(7), A::scale_offset(s.ad_value(7), p.p102, p.p101), 1.0);
        }

        if (!s.b[252]) {
            s.store_exp_scaled_input(204, 6, p.p98);
        }

        s.store_scale(203, 204, p.p12);

        s.store_mul_scaled_ad_rhs(205, 204, p.p13, A::exp_scaled_input(s.ad_value(10), s.v[176]));

        s.v[206] = p.p14;

        s.b[253] = ((p.p103 != 0.0) && (p.p104 >= p.p111));
        s.v[253] = if s.b[253] { 1.0 } else { 0.0 };

        if s.b[253] {
            s.store_offset_voltage(4, ctx, nodes, Some(4), None, (s.v[9] + p.p109));
        }

        s.b[254] = (s.v[4] < ((-100.0) + 273.15));
        s.v[254] = if s.b[254] { 1.0 } else { 0.0 };

        if (s.b[253] && s.b[254]) {
            s.store_scalar(4, ((-100.0) + 273.15));
        }

        s.b[255] = (s.v[4] > (326.85 + 273.15));
        s.v[255] = if s.b[255] { 1.0 } else { 0.0 };

        if ((s.b[253] && (!s.b[254])) && s.b[255]) {
            s.store_scalar(4, (326.85 + 273.15));
        }

        if s.b[253] {
            s.store_scale(2, 4, (1.3806226e-23 * 6.241509744511525e18));
            s.store_div_from_scalar(3, 1.0, 2);
            s.store_offset(7, 4, (-s.v[8]));
            s.store_scale(5, 4, 1.0 / (s.v[8]));
            s.store_ln(6, 5);
            s.store_mul_offset_rhs(10, 3, 5, (-1.0));
            s.store_scalar(178, ((0.5 * p.p35) / s.v[177]));
            s.store_scaled_ln_ad(96, A::sub(A::exp(s.ad_value(178)), A::exp_scaled_input(s.ad_value(178), -1.0)), (2.0 * s.v[177]));
            s.store_add_scaled_value_products(97, A::scale_offset(s.ad_value(5), (-s.v[173]), s.v[173]), 1.0, s.ad_value(96), s.ad_value(5), 1.0, s.ad_value(2), s.ad_value(6), (-s.v[168]));
            s.store_add_scaled_product_right_ad(16, 97, 1.0, 2, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(97), -1.0, s.ad_value(3))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(23, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p35, s.ad_value(16))), p.p36), p.p34);
            s.store_scale(43, 16, (p.p37 * 1.0 / (p.p35)));
            s.store_scalar(178, ((0.5 * p.p38) / s.v[177]));
            s.store_scaled_ln_ad(96, A::sub(A::exp(s.ad_value(178)), A::exp_scaled_input(s.ad_value(178), -1.0)), (2.0 * s.v[177]));
            s.store_add_scaled_value_products(97, A::scale_offset(s.ad_value(5), (-s.v[173]), s.v[173]), 1.0, s.ad_value(96), s.ad_value(5), 1.0, s.ad_value(2), s.ad_value(6), (-s.v[168]));
            s.store_add_scaled_product_right_ad(22, 97, 1.0, 2, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(97), -1.0, s.ad_value(3))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(26, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p38, s.ad_value(22))), p.p39), s.v[27]);
            s.store_scale(44, 22, (p.p40 * 1.0 / (p.p38)));
            s.store_scalar(178, ((0.5 * p.p42) / s.v[177]));
            s.store_scaled_ln_ad(96, A::sub(A::exp(s.ad_value(178)), A::exp_scaled_input(s.ad_value(178), -1.0)), (2.0 * s.v[177]));
            s.store_add_scaled_value_products(97, A::scale_offset(s.ad_value(5), (-s.v[174]), s.v[174]), 1.0, s.ad_value(96), s.ad_value(5), 1.0, s.ad_value(2), s.ad_value(6), (-s.v[168]));
            s.store_add_scaled_product_right_ad(17, 97, 1.0, 2, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(97), -1.0, s.ad_value(3))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(24, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p42, s.ad_value(17))), p.p43), p.p41);
            s.store_scaled_exp_ad(11, A::add_scaled_inputs(s.ad_value(6), p.p81, s.ad_value(10), p.p76), p.p1);
            s.store_scaled_exp_ad(15, A::sub_scaled_inputs(s.ad_value(6), p.p95, s.ad_value(10), p.p83), p.p9);
            s.store_scaled_exp_scaled_input(33, 6, (p.p87 - s.v[172]), p.p62);
            s.store_scaled_exp_scaled_input(31, 6, p.p87, p.p61);
            s.store_div_from_scalar(32, 1.0, 31);
            s.store_offset_scaled(34, 7, ((p.p89) * (p.p64)), p.p64);
        }

        s.b[256] = (p.p65 > 0.0);
        s.v[256] = if s.b[256] { 1.0 } else { 0.0 };

        if (s.b[253] && s.b[256]) {
            s.store_offset_scaled_ad(38, A::scale(s.ad_value(7), p.p90), (-p.p65), p.p65);
            s.store_scalar(34, p.p64);
        }

        if (s.b[253] && (!s.b[256])) {
            s.store_offset_scaled(34, 7, ((p.p89) * (p.p64)), p.p64);
            s.store_scalar(38, p.p65);
        }

        if s.b[253] {
            s.store_add_scaled_product_value_ad(42, A::scale_offset(s.ad_value(7), p.p85, 1.0), p.p54, 7, 7, (p.p86 * p.p54));
        }

        s.b[257] = (p.p96 == 1.0);
        s.v[257] = if s.b[257] { 1.0 } else { 0.0 };

        if (s.b[253] && s.b[257]) {
            s.store_scaled_exp_ad(36, A::sub_scaled_inputs(s.ad_value(6), s.v[171], s.ad_value(10), s.v[176]), p.p57);
        }

        if (s.b[253] && (!s.b[257])) {
            s.store_scalar(36, p.p57);
        }

        if s.b[253] {
            s.store_scaled_exp_scaled_input(35, 6, (p.p87 - 1.0), p.p59);
            s.store_scalar(178, ((0.5 * p.p46) / s.v[177]));
            s.store_scaled_ln_ad(96, A::sub(A::exp(s.ad_value(178)), A::exp_scaled_input(s.ad_value(178), -1.0)), (2.0 * s.v[177]));
            s.store_add_scaled_value_products(97, A::scale_offset(s.ad_value(5), (-s.v[174]), s.v[174]), 1.0, s.ad_value(96), s.ad_value(5), 1.0, s.ad_value(2), s.ad_value(6), (-s.v[168]));
            s.store_add_scaled_product_right_ad(18, 97, 1.0, 2, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(97), -1.0, s.ad_value(3))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(25, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p46, s.ad_value(18))), p.p47), p.p45);
            s.store_scalar(178, ((0.5 * p.p51) / s.v[177]));
            s.store_scaled_ln_ad(96, A::sub(A::exp(s.ad_value(178)), A::exp_scaled_input(s.ad_value(178), -1.0)), (2.0 * s.v[177]));
            s.store_add_scaled_value_products(97, A::scale_offset(s.ad_value(5), (-s.v[175]), s.v[175]), 1.0, s.ad_value(96), s.ad_value(5), 1.0, s.ad_value(2), s.ad_value(6), (-s.v[168]));
            s.store_add_scaled_product_right_ad(19, 97, 1.0, 2, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(97), -1.0, s.ad_value(3))), 4.0, 1.0)), 1.0), 0.5), 2.0);
            s.store_scale_ad(30, A::exp_scaled_input(A::ln(A::div_from_scalar(p.p51, s.ad_value(19))), p.p52), p.p50);
            s.store_scaled_exp_scaled_input(200, 6, p.p97, p.p7);
            s.store_div_from_scalar_exp_ad(202, p.p6, A::mul_scaled_lhs(s.ad_value(3), p.p83, A::offset(A::exp_scaled_input(s.ad_value(6), p.p84), (-1.0))));
        }

        s.b[259] = (p.p0 <= 200.0);
        s.v[259] = if s.b[259] { 1.0 } else { 0.0 };

        if (s.b[253] && s.b[259]) {
            s.store_offset_mul_ad(204, s.ad_value(7), A::scale_offset(s.ad_value(7), p.p102, p.p101), 1.0);
        }

        if (s.b[253] && (!s.b[259])) {
            s.store_exp_scaled_input(204, 6, p.p98);
        }

        if s.b[253] {
            s.store_scale(203, 204, p.p12);
            s.store_mul_scaled_ad_rhs(205, 204, p.p13, A::exp_scaled_input(s.ad_value(10), s.v[176]));
            s.store_scalar(206, p.p14);
        }

        s.b[260] = (s.v[25] <= 1e-30);
        s.v[260] = if s.b[260] { 1.0 } else { 0.0 };

        if s.b[260] {
            s.store_scale(111, 24, p.p49);
            s.store_scalar(108, 0.0);
            s.store_scale(113, 24, (1.0 - p.p49));
        }

        s.b[261] = (p.p44 < 100.0);
        s.v[261] = if s.b[261] { 1.0 } else { 0.0 };

        s.b[262] = (s.v[113] > 0.0);
        s.v[262] = if s.b[262] { 1.0 } else { 0.0 };

        if ((s.b[260] && s.b[261]) && s.b[262]) {
            s.store_scalar(50, (p.p43 / 4.0));
            s.store_sub_from_scalar(51, p.p44, 17);
            s.store_scale(52, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
            s.store_scale(53, 113, 2.4);
            s.store_mul_exp_ad_rhs(54, 113, A::mul_offset_lhs(s.ad_value(50), (-p.p43), A::ln(A::div_from_scalar(p.p44, s.ad_value(17)))));
            s.store_mul_sub_lhs(56, 52, 183, 3);
        }

        s.b[263] = (s.v[56] < 80.0);
        s.v[263] = if s.b[263] { 1.0 } else { 0.0 };

        if (((s.b[260] && s.b[261]) && s.b[262]) && s.b[263]) {
            s.store_exp(57, 56);
            s.store_add_scaled_product_right_ad(58, 52, 1.0, 2, A::ln(A::offset(s.ad_value(57), 1.0)), (-1.0));
        }

        if (((s.b[260] && s.b[261]) && s.b[262]) && (!s.b[263])) {
            s.copy_ad(58, 183);
        }

        if ((s.b[260] && s.b[261]) && s.b[262]) {
            s.store_add_scaled_inputs(55, 51, 0.1, 2, 4.0);
            s.store_div_scaled_inputs2(59, s.ad_value(51), 1.0, s.ad_value(58), 1.0, s.ad_value(55), 1.0);
        }

        s.b[264] = (s.v[59] < 80.0);
        s.v[264] = if s.b[264] { 1.0 } else { 0.0 };

        if (((s.b[260] && s.b[261]) && s.b[262]) && s.b[264]) {
            s.store_exp(57, 59);
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(51), -1.0, s.ad_value(52), -1.0, s.ad_value(55), 1.0)))), 51);
        }

        if (((s.b[260] && s.b[261]) && s.b[262]) && (!s.b[264])) {
            s.copy_ad(60, 58);
        }

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[260] && s.b[261]) && s.b[262]) {
            s.store_sub(61, 183, 58);
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(17))));
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(17))));
            s.store_scalar(67, (1.0 - p.p43));
            s.store_sub_from_scalar(68, 1.0, 50);
            s.store_div_ad_lhs(62, A::mul_sub_from_scalar_rhs(s.ad_value(113), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67)))), 67);
            s.store_div_ad_lhs(63, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68)))), 68);
            s.store_div_ad_lhs(64, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68)))), 68);
            s.store_add_scaled_products_left_left_ad(105, A::add_scaled_inputs3(s.ad_value(62), 1.0, s.ad_value(63), 1.0, s.ad_value(64), -1.0), 17, 1.0, 53, 61, 1.0);
        }

        if ((s.b[260] && s.b[261]) && (!s.b[262])) {
            s.store_scalar(105, 0.0);
        }

        s.b[265] = (s.v[113] > 0.0);
        s.v[265] = if s.b[265] { 1.0 } else { 0.0 };

        if ((s.b[260] && (!s.b[261])) && s.b[265]) {
            s.store_scale(76, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
            s.store_mul_sub_lhs(80, 76, 183, 3);
            s.store_sqrt_square_offset(81, 80, 1.921812);
            s.store_scaled_add(82, 80, 81, 0.5);
            s.store_add_scaled_product_indices(77, 76, 1.0, 2, 82, (-1.0));
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(17))));
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(79, 17, 1.0, A::exp_scaled_input(s.ad_value(78), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
            s.store_mul_add_scaled_inputs3_offset_rhs(105, 113, s.ad_value(79), 1.0, s.ad_value(183), 2.4, s.ad_value(77), (-2.4), 0.0);
        }

        if ((s.b[260] && (!s.b[261])) && (!s.b[265])) {
            s.store_scalar(105, 0.0);
        }

        if (!s.b[260]) {
            s.copy_ad(111, 24);
            s.store_scale(112, 25, p.p49);
        }

        s.b[266] = (p.p48 < 100.0);
        s.v[266] = if s.b[266] { 1.0 } else { 0.0 };

        s.b[267] = (s.v[112] > 0.0);
        s.v[267] = if s.b[267] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && s.b[266]) && s.b[267]) {
            s.store_scalar(50, (p.p47 / 4.0));
            s.store_sub_from_scalar(51, p.p48, 18);
            s.store_scale(52, 18, (1.0 - ((((-((2.4) as f64).ln()) / p.p47)) as f64).exp()));
            s.store_scale(53, 112, 2.4);
            s.store_mul_exp_ad_rhs(54, 112, A::mul_offset_lhs(s.ad_value(50), (-p.p47), A::ln(A::div_from_scalar(p.p48, s.ad_value(18)))));
            s.store_mul_sub_lhs(56, 52, 184, 3);
        }

        s.b[268] = (s.v[56] < 80.0);
        s.v[268] = if s.b[268] { 1.0 } else { 0.0 };

        if ((((!s.b[260]) && s.b[266]) && s.b[267]) && s.b[268]) {
            s.store_exp(57, 56);
            s.store_add_scaled_product_right_ad(58, 52, 1.0, 2, A::ln(A::offset(s.ad_value(57), 1.0)), (-1.0));
        }

        if ((((!s.b[260]) && s.b[266]) && s.b[267]) && (!s.b[268])) {
            s.copy_ad(58, 184);
        }

        if (((!s.b[260]) && s.b[266]) && s.b[267]) {
            s.store_add_scaled_inputs(55, 51, 0.1, 2, 4.0);
            s.store_div_scaled_inputs2(59, s.ad_value(51), 1.0, s.ad_value(58), 1.0, s.ad_value(55), 1.0);
        }

        s.b[269] = (s.v[59] < 80.0);
        s.v[269] = if s.b[269] { 1.0 } else { 0.0 };

        if ((((!s.b[260]) && s.b[266]) && s.b[267]) && s.b[269]) {
            s.store_exp(57, 59);
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(51), -1.0, s.ad_value(52), -1.0, s.ad_value(55), 1.0)))), 51);
        }

        if ((((!s.b[260]) && s.b[266]) && s.b[267]) && (!s.b[269])) {
            s.copy_ad(60, 58);
        }

        if (((!s.b[260]) && s.b[266]) && s.b[267]) {
            s.store_sub(61, 184, 58);
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(18))));
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(18))));
            s.store_scalar(67, (1.0 - p.p47));
            s.store_sub_from_scalar(68, 1.0, 50);
            s.store_div_ad_lhs(62, A::mul_sub_from_scalar_rhs(s.ad_value(112), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67)))), 67);
            s.store_div_ad_lhs(63, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68)))), 68);
            s.store_div_ad_lhs(64, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68)))), 68);
            s.store_add_scaled_products_left_left_ad(108, A::add_scaled_inputs3(s.ad_value(62), 1.0, s.ad_value(63), 1.0, s.ad_value(64), -1.0), 18, 1.0, 53, 61, 1.0);
        }

        if (((!s.b[260]) && s.b[266]) && (!s.b[267])) {
            s.store_scalar(108, 0.0);
        }

        s.b[270] = (s.v[112] > 0.0);
        s.v[270] = if s.b[270] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && (!s.b[266])) && s.b[270]) {
            s.store_scale(76, 18, (1.0 - ((((-((2.4) as f64).ln()) / p.p47)) as f64).exp()));
            s.store_mul_sub_lhs(80, 76, 184, 3);
            s.store_sqrt_square_offset(81, 80, 1.921812);
            s.store_scaled_add(82, 80, 81, 0.5);
            s.store_add_scaled_product_indices(77, 76, 1.0, 2, 82, (-1.0));
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(18))));
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(79, 18, 1.0, A::exp_scaled_input(s.ad_value(78), (1.0 - p.p47)), 1.0 / ((1.0 - p.p47)));
            s.store_mul_add_scaled_inputs3_offset_rhs(108, 112, s.ad_value(79), 1.0, s.ad_value(184), 2.4, s.ad_value(77), (-2.4), 0.0);
        }

        if (((!s.b[260]) && (!s.b[266])) && (!s.b[270])) {
            s.store_scalar(108, 0.0);
        }

        if (!s.b[260]) {
            s.store_scale(113, 25, (1.0 - p.p49));
        }

        s.b[271] = (p.p48 < 100.0);
        s.v[271] = if s.b[271] { 1.0 } else { 0.0 };

        s.b[272] = (s.v[113] > 0.0);
        s.v[272] = if s.b[272] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && s.b[271]) && s.b[272]) {
            s.store_scalar(50, (p.p47 / 4.0));
            s.store_sub_from_scalar(51, p.p48, 18);
            s.store_scale(52, 18, (1.0 - ((((-((2.4) as f64).ln()) / p.p47)) as f64).exp()));
            s.store_scale(53, 113, 2.4);
            s.store_mul_exp_ad_rhs(54, 113, A::mul_offset_lhs(s.ad_value(50), (-p.p47), A::ln(A::div_from_scalar(p.p48, s.ad_value(18)))));
            s.store_mul_sub_lhs(56, 52, 183, 3);
        }

        s.b[273] = (s.v[56] < 80.0);
        s.v[273] = if s.b[273] { 1.0 } else { 0.0 };

        if ((((!s.b[260]) && s.b[271]) && s.b[272]) && s.b[273]) {
            s.store_exp(57, 56);
            s.store_add_scaled_product_right_ad(58, 52, 1.0, 2, A::ln(A::offset(s.ad_value(57), 1.0)), (-1.0));
        }

        if ((((!s.b[260]) && s.b[271]) && s.b[272]) && (!s.b[273])) {
            s.copy_ad(58, 183);
        }

        if (((!s.b[260]) && s.b[271]) && s.b[272]) {
            s.store_add_scaled_inputs(55, 51, 0.1, 2, 4.0);
            s.store_div_scaled_inputs2(59, s.ad_value(51), 1.0, s.ad_value(58), 1.0, s.ad_value(55), 1.0);
        }

        s.b[274] = (s.v[59] < 80.0);
        s.v[274] = if s.b[274] { 1.0 } else { 0.0 };

        if ((((!s.b[260]) && s.b[271]) && s.b[272]) && s.b[274]) {
            s.store_exp(57, 59);
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(51), -1.0, s.ad_value(52), -1.0, s.ad_value(55), 1.0)))), 51);
        }

        if ((((!s.b[260]) && s.b[271]) && s.b[272]) && (!s.b[274])) {
            s.copy_ad(60, 58);
        }

        if (((!s.b[260]) && s.b[271]) && s.b[272]) {
            s.store_sub(61, 183, 58);
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(18))));
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(18))));
            s.store_scalar(67, (1.0 - p.p47));
            s.store_sub_from_scalar(68, 1.0, 50);
            s.store_div_ad_lhs(62, A::mul_sub_from_scalar_rhs(s.ad_value(113), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67)))), 67);
            s.store_div_ad_lhs(63, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68)))), 68);
            s.store_div_ad_lhs(64, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68)))), 68);
            s.store_add_scaled_products_left_left_ad(105, A::add_scaled_inputs3(s.ad_value(62), 1.0, s.ad_value(63), 1.0, s.ad_value(64), -1.0), 18, 1.0, 53, 61, 1.0);
        }

        if (((!s.b[260]) && s.b[271]) && (!s.b[272])) {
            s.store_scalar(105, 0.0);
        }

        s.b[275] = (s.v[113] > 0.0);
        s.v[275] = if s.b[275] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && (!s.b[271])) && s.b[275]) {
            s.store_scale(76, 18, (1.0 - ((((-((2.4) as f64).ln()) / p.p47)) as f64).exp()));
            s.store_mul_sub_lhs(80, 76, 183, 3);
            s.store_sqrt_square_offset(81, 80, 1.921812);
            s.store_scaled_add(82, 80, 81, 0.5);
            s.store_add_scaled_product_indices(77, 76, 1.0, 2, 82, (-1.0));
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(18))));
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(79, 18, 1.0, A::exp_scaled_input(s.ad_value(78), (1.0 - p.p47)), 1.0 / ((1.0 - p.p47)));
            s.store_mul_add_scaled_inputs3_offset_rhs(105, 113, s.ad_value(79), 1.0, s.ad_value(183), 2.4, s.ad_value(77), (-2.4), 0.0);
        }

        if (((!s.b[260]) && (!s.b[271])) && (!s.b[275])) {
            s.store_scalar(105, 0.0);
        }

        s.b[276] = (p.p44 < 100.0);
        s.v[276] = if s.b[276] { 1.0 } else { 0.0 };

        s.b[277] = (s.v[111] > 0.0);
        s.v[277] = if s.b[277] { 1.0 } else { 0.0 };

        if (s.b[276] && s.b[277]) {
            s.store_scalar(50, (p.p43 / 4.0));
            s.store_sub_from_scalar(51, p.p44, 17);
            s.store_scale(52, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
            s.store_scale(53, 111, 2.4);
            s.store_mul_exp_ad_rhs(54, 111, A::mul_offset_lhs(s.ad_value(50), (-p.p43), A::ln(A::div_from_scalar(p.p44, s.ad_value(17)))));
            s.store_mul_sub_lhs(56, 52, 184, 3);
        }

        s.b[278] = (s.v[56] < 80.0);
        s.v[278] = if s.b[278] { 1.0 } else { 0.0 };

        if ((s.b[276] && s.b[277]) && s.b[278]) {
            s.store_exp(57, 56);
            s.store_add_scaled_product_right_ad(58, 52, 1.0, 2, A::ln(A::offset(s.ad_value(57), 1.0)), (-1.0));
        }

        if ((s.b[276] && s.b[277]) && (!s.b[278])) {
            s.copy_ad(58, 184);
        }

        if (s.b[276] && s.b[277]) {
            s.store_add_scaled_inputs(55, 51, 0.1, 2, 4.0);
            s.store_div_scaled_inputs2(59, s.ad_value(51), 1.0, s.ad_value(58), 1.0, s.ad_value(55), 1.0);
        }

        s.b[279] = (s.v[59] < 80.0);
        s.v[279] = if s.b[279] { 1.0 } else { 0.0 };

        if ((s.b[276] && s.b[277]) && s.b[279]) {
            s.store_exp(57, 59);
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(51), -1.0, s.ad_value(52), -1.0, s.ad_value(55), 1.0)))), 51);
        }

        if ((s.b[276] && s.b[277]) && (!s.b[279])) {
            s.copy_ad(60, 58);
        }

        if (s.b[276] && s.b[277]) {
            s.store_sub(61, 184, 58);
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(17))));
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(17))));
            s.store_scalar(67, (1.0 - p.p43));
            s.store_sub_from_scalar(68, 1.0, 50);
            s.store_div_ad_lhs(62, A::mul_sub_from_scalar_rhs(s.ad_value(111), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67)))), 67);
            s.store_div_ad_lhs(63, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68)))), 68);
            s.store_div_ad_lhs(64, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68)))), 68);
            s.store_add_scaled_products_left_left_ad(103, A::add_scaled_inputs3(s.ad_value(62), 1.0, s.ad_value(63), 1.0, s.ad_value(64), -1.0), 17, 1.0, 53, 61, 1.0);
        }

        if (s.b[276] && (!s.b[277])) {
            s.store_scalar(103, 0.0);
        }

        s.b[280] = (s.v[111] > 0.0);
        s.v[280] = if s.b[280] { 1.0 } else { 0.0 };

        if ((!s.b[276]) && s.b[280]) {
            s.store_scale(76, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
            s.store_mul_sub_lhs(80, 76, 184, 3);
            s.store_sqrt_square_offset(81, 80, 1.921812);
            s.store_scaled_add(82, 80, 81, 0.5);
        }

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[276]) && s.b[280]) {
            s.store_add_scaled_product_indices(77, 76, 1.0, 2, 82, (-1.0));
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(17))));
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(79, 17, 1.0, A::exp_scaled_input(s.ad_value(78), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
            s.store_mul_add_scaled_inputs3_offset_rhs(103, 111, s.ad_value(79), 1.0, s.ad_value(184), 2.4, s.ad_value(77), (-2.4), 0.0);
        }

        if ((!s.b[276]) && (!s.b[280])) {
            s.store_scalar(103, 0.0);
        }

        s.store_add(106, 103, 108);

        s.b[281] = (s.v[111] > 0.0);
        s.v[281] = if s.b[281] { 1.0 } else { 0.0 };

        if s.b[281] {
            s.store_scale(282, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
            s.store_mul_sub_lhs(283, 282, 184, 3);
            s.store_sqrt_square_offset(284, 283, 1.921812);
            s.store_scaled_add(285, 283, 284, 0.5);
            s.store_add_scaled_product_indices(286, 282, 1.0, 2, 285, (-1.0));
            s.store_div(287, 285, 284);
            s.store_add_ad(107, A::mul3(s.ad_value(111), A::exp_scaled_input(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(286), s.ad_value(17)))), (-p.p43)), s.ad_value(287)), A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(111), 1.0, s.ad_value(287), 2.4));
        }

        if (!s.b[281]) {
            s.store_scalar(107, 0.0);
        }

        s.b[288] = (p.p65 > 0.0);
        s.v[288] = if s.b[288] { 1.0 } else { 0.0 };

        if s.b[288] {
            s.store_sub(143, 38, 184);
        }

        if (!s.b[288]) {
            s.store_sub(143, 186, 34);
        }

        s.store_offset_mul(289, 143, 3, (-1.0));

        s.store_mul_offset_ad_lhs(290, A::add_scaled_inputs(s.ad_value(289), 0.5, A::sqrt(A::offset(A::square(s.ad_value(289)), 1.921812)), 0.5), 1.0, 2);

        s.store_div(291, 290, 33);

        s.store_mul(292, 290, 32);

        s.store_exp_scaled_input_ad(293, A::ln_one_plus_exp(A::scale(A::ln(s.ad_value(291)), p.p67)), 1.0 / (p.p67));

        s.store_div(294, 292, 293);

        s.store_scaled_sub(295, 290, 33, 1.0 / (p.p63));

        s.store_mul_offset_ad_rhs(142, 294, A::add_scaled_inputs(s.ad_value(295), 0.5, A::sqrt(A::offset(A::square(s.ad_value(295)), p.p66)), 0.5), 1.0);

        s.b[296] = ((s.v[107] > 0.0) && (s.v[111] > 0.0));
        s.v[296] = if s.b[296] { 1.0 } else { 0.0 };

        if s.b[296] {
            s.store_div(114, 111, 107);
            s.store_div(103, 103, 111);
        }

        if (!s.b[296]) {
            s.store_scalar(114, 1.0);
            s.store_scalar(103, 0.0);
        }

        s.b[297] = (s.v[23] > 0.0);
        s.v[297] = if s.b[297] { 1.0 } else { 0.0 };

        if s.b[297] {
            s.store_mul_sub_from_scalar_ad_rhs(76, 16, 1.0, A::exp_scaled_input(A::ln(s.ad_value(43)), (-1.0 / (p.p36))));
            s.store_mul_sub_lhs(80, 76, 185, 3);
            s.store_sqrt_square_offset(81, 80, 1.921812);
            s.store_scaled_add(82, 80, 81, 0.5);
            s.store_add_scaled_product_indices(77, 76, 1.0, 2, 82, (-1.0));
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(16))));
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(79, 16, 1.0, A::exp_scaled_input(s.ad_value(78), (1.0 - p.p36)), 1.0 / ((1.0 - p.p36)));
            s.store_mul_add_scaled_product_rhs(98, 23, s.ad_value(79), 1.0, s.ad_value(43), A::sub(s.ad_value(185), s.ad_value(77)), 1.0);
        }

        if (!s.b[297]) {
            s.store_scalar(98, 0.0);
        }

        s.store_div(102, 98, 23);

        s.b[298] = (p.p0 <= 200.0);
        s.v[298] = if s.b[298] { 1.0 } else { 0.0 };

        s.b[299] = (s.v[26] > 0.0);
        s.v[299] = if s.b[299] { 1.0 } else { 0.0 };

        if (s.b[298] && s.b[299]) {
            s.store_mul_sub_from_scalar_ad_rhs(76, 22, 1.0, A::exp_scaled_input(A::ln(s.ad_value(44)), (-1.0 / (p.p39))));
            s.store_mul_sub_lhs(80, 76, 185, 3);
            s.store_sqrt_square_offset(81, 80, 1.921812);
            s.store_scaled_add(82, 80, 81, 0.5);
            s.store_add_scaled_product_indices(77, 76, 1.0, 2, 82, (-1.0));
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(22))));
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(79, 22, 1.0, A::exp_scaled_input(s.ad_value(78), (1.0 - p.p39)), 1.0 / ((1.0 - p.p39)));
            s.store_mul_add_scaled_product_rhs(100, 26, s.ad_value(79), 1.0, s.ad_value(44), A::sub(s.ad_value(185), s.ad_value(77)), 1.0);
        }

        if (s.b[298] && (!s.b[299])) {
            s.store_scalar(100, 0.0);
        }

        if s.b[298] {
            s.store_div(101, 100, 26);
            s.copy_ad(20, 22);
            s.store_scalar(21, p.p39);
        }

        if (!s.b[298]) {
            s.copy_ad(101, 102);
            s.copy_ad(20, 16);
            s.store_scalar(21, p.p36);
        }

        s.b[300] = (p.p7 == 0.0);
        s.v[300] = if s.b[300] { 1.0 } else { 0.0 };

        if s.b[300] {
            s.store_scalar(201, 1.0);
        }

        if (!s.b[300]) {
            s.store_scale(301, 2, p.p8);
            s.store_div_scaled_inputs2(302, s.ad_value(20), 1.0, s.ad_value(185), (-1.0), s.ad_value(301), 1.0);
            s.store_add_scaled_product_right_ad(303, 20, 1.0, 301, A::add(s.ad_value(302), A::sqrt(A::offset(A::square(s.ad_value(302)), 1.921812))), (-0.5));
            s.store_mul_sub_from_scalar_ad_rhs(304, 200, 1.0, A::exp(A::mul(s.ad_value(21), A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(303), s.ad_value(20)))))));
        }

        s.b[305] = (((s.v[304]) as f64).abs() >= 0.001);
        s.v[305] = if s.b[305] { 1.0 } else { 0.0 };

        if ((!s.b[300]) && s.b[305]) {
            s.store_div_scaled_offset_numerator(201, A::exp(s.ad_value(304)), 1.0, (-1.0), s.ad_value(304), 1.0);
        }

        if ((!s.b[300]) && (!s.b[305])) {
            s.store_offset_scaled(201, 304, 0.5, 1.0);
        }

        s.store_mul(159, 201, 101);

        s.store_add_scaled_ad_lhs(116, A::offset(A::div(s.ad_value(159), s.ad_value(202)), 1.0), 103, 1.0 / (p.p5));

        s.store_offset_scaled(131, 116, 20.0, (-1.0));

        s.store_scaled_offset_ad(115, A::add_scaled_inputs(s.ad_value(131), 0.5, A::sqrt(A::offset(A::square(s.ad_value(131)), 1.921812)), 0.5), 1.0, 0.025);

        s.store_add_scaled_inputs3_offset(117, s.ad_value(42), 1.0, s.ad_value(114), p.p55, A::div_from_scalar(1.0, s.ad_value(114)), p.p56, (((-1.0) * p.p55) + ((-1.0) * p.p56)));

        s.b[306] = (p.p10 == 1.0);
        s.v[306] = if s.b[306] { 1.0 } else { 0.0 };

        if s.b[306] {
            s.store_offset_div(130, 117, 42, (-1.0));
            s.store_div_scaled_value_offset_denominator(118, s.ad_value(15), 1.0, s.ad_value(130), 1.0, 1.0);
        }

        if (!s.b[306]) {
            s.copy_ad(118, 15);
        }

        s.v[119] = p.p11;

        s.store_div_scaled_inputs(180, s.ad_value(185), 1.0, s.ad_value(2), p.p3);

        s.b[307] = (s.v[180] > 80.0);
        s.v[307] = if s.b[307] { 1.0 } else { 0.0 };

        if s.b[307] {
            s.store_offset(179, 180, (((-80.0)) + (1.0)));
            s.store_scalar(180, 80.0);
        }

        if (!s.b[307]) {
            s.store_scalar(179, 1.0);
        }

        s.store_mul_limexp_rhs(179, 179, 180);

        s.store_mul(120, 11, 179);

        s.store_div_scaled_inputs(182, s.ad_value(184), 1.0, s.ad_value(2), p.p4);

        s.b[308] = (s.v[182] > 80.0);
        s.v[308] = if s.b[308] { 1.0 } else { 0.0 };

        if s.b[308] {
            s.store_offset(181, 182, (((-80.0)) + (1.0)));
            s.store_scalar(182, 80.0);
        }

        if (!s.b[308]) {
            s.store_scalar(181, 1.0);
        }

        s.store_mul_limexp_rhs(181, 181, 182);

        s.store_mul(121, 11, 181);

        s.b[309] = (p.p13 != 0.0);
        s.v[309] = if s.b[309] { 1.0 } else { 0.0 };

        if s.b[309] {
            s.store_add_scaled_inputs3(123, A::div(s.ad_value(120), s.ad_value(118)), 1.0, s.ad_value(121), 1.0 / (s.v[119]), A::exp_scaled_input(A::ln(A::mul3(s.ad_value(120), A::div(s.ad_value(120), s.ad_value(142)), A::div(s.ad_value(205), s.ad_value(203)))), 0.6666), 1.0);
            s.store_add_scaled_inputs4(124, A::div(s.ad_value(120), s.ad_value(118)), 1.0, s.ad_value(121), 1.0 / (s.v[119]), A::div(s.ad_value(120), s.ad_value(203)), 1.0, A::exp_scaled_input(A::ln(A::mul3(s.ad_value(120), A::div(s.ad_value(120), s.ad_value(142)), A::div(s.ad_value(205), s.ad_value(203)))), 0.6666), 1.0);
        }

        if (!s.b[309]) {
            s.store_add_scaled_ad_lhs(123, A::div(s.ad_value(120), s.ad_value(118)), 121, 1.0 / (s.v[119]));
            s.store_add_scaled_inputs3(124, A::div(s.ad_value(120), s.ad_value(118)), 1.0, s.ad_value(121), 1.0 / (s.v[119]), A::div(s.ad_value(120), s.ad_value(203)), 1.0);
        }

        s.store_add_ad_rhs(128, 115, A::sqrt(A::add(A::square(s.ad_value(115)), s.ad_value(123))));

        s.store_add_ad_rhs(129, 115, A::sqrt(A::add(A::square(s.ad_value(115)), s.ad_value(124))));

        s.store_sub(207, 124, 123);

        s.b[310] = (((s.v[207]) as f64).abs() > 1e-8);
        s.v[310] = if s.b[310] { 1.0 } else { 0.0 };

        if s.b[310] {
            s.store_sub_from_scalar_ad(150, 1.0, A::mul(A::div(A::div_scaled_value_offset_denominator(s.ad_value(142), 1.0, s.ad_value(206), 1.0, 1.0), s.ad_value(120)), s.ad_value(128)));
            s.store_offset_mul_ad(151, A::div(A::div_scaled_value_offset_denominator(s.ad_value(142), 1.0, s.ad_value(206), 1.0, 1.0), s.ad_value(120)), A::sub(s.ad_value(129), s.ad_value(128)), 1.0);
            s.store_div(149, 150, 151);
            s.store_scaled_add_ad_lhs(146, A::sqrt(A::offset(A::square(s.ad_value(149)), 0.01)), 149, 1.0 / ((1.0 + (((1.0 + 0.01)) as f64).sqrt())));
        }

        if (!s.b[310]) {
            s.store_scalar(146, 0.0);
        }

        s.b[311] = (p.p2 == 0.0);
        s.v[311] = if s.b[311] { 1.0 } else { 0.0 };

        s.b[312] = (p.p13 != 0.0);
        s.v[312] = if s.b[312] { 1.0 } else { 0.0 };

        if (s.b[311] && s.b[312]) {
            s.store_add_scaled_inputs4(122, A::div(s.ad_value(120), s.ad_value(118)), 1.0, s.ad_value(121), 1.0 / (s.v[119]), A::mul3(A::div(s.ad_value(120), s.ad_value(203)), s.ad_value(146), s.ad_value(146)), 1.0, A::exp_scaled_input(A::ln(A::mul3(s.ad_value(120), A::div(s.ad_value(120), s.ad_value(142)), A::div(s.ad_value(205), s.ad_value(203)))), 0.6666), 1.0);
        }

        if (s.b[311] && (!s.b[312])) {
            s.store_add_scaled_inputs3(122, A::div(s.ad_value(120), s.ad_value(118)), 1.0, s.ad_value(121), 1.0 / (s.v[119]), A::mul3(A::div(s.ad_value(120), s.ad_value(203)), s.ad_value(146), s.ad_value(146)), 1.0);
        }

        if s.b[311] {
            s.store_add_ad_rhs(125, 115, A::sqrt(A::add(A::square(s.ad_value(115)), s.ad_value(122))));
        }

        if (!s.b[311]) {
            s.store_scalar(83, (1.0 / 3.0));
            s.store_scale(84, 115, (-2.0));
        }

        s.b[313] = ((p.p9 == 1000000.0) && (p.p12 == 1000000.0));
        s.v[313] = if s.b[313] { 1.0 } else { 0.0 };

        if ((!s.b[311]) && s.b[313]) {
            s.store_scalar(85, 0.0);
        }

        if ((!s.b[311]) && (!s.b[313])) {
            s.store_neg_ad(85, A::add_scaled_inputs3(A::div(s.ad_value(120), s.ad_value(118)), 1.0, s.ad_value(121), 1.0 / (s.v[119]), A::mul3(A::div(s.ad_value(120), s.ad_value(203)), s.ad_value(146), s.ad_value(146)), 1.0));
        }

        if (!s.b[311]) {
            s.store_div_scaled_product_left_ad(86, A::div_scaled_product(s.ad_value(120), s.ad_value(120), -1.0, s.ad_value(142), 1.0), 205, 1.0, 203, 1.0);
            s.store_square(87, 84);
            s.store_add_scaled_product_indices(88, 85, 1.0, 87, 83, (-1.0));
            s.store_add_ad_lhs(89, A::add_scaled_product(A::mul3(s.ad_value(84), s.ad_value(85), s.ad_value(83)), (-1.0), s.ad_value(84), s.ad_value(87), (2.0 * 0.037037037037037035)), 86);
            s.store_add_scaled_square_product_mixed_iai(90, 89, 0.25, A::square(s.ad_value(88)), 88, 0.037037037037037035);
        }

        s.b[314] = (((s.v[90]) as f64).abs() < 1e-10);
        s.v[314] = if s.b[314] { 1.0 } else { 0.0 };

        if ((!s.b[311]) && s.b[314]) {
            s.store_add_scaled_product_value_ad(91, A::div_scaled_inputs(s.ad_value(89), 3.0, s.ad_value(88), 1.0), 1.0, 84, 83, (-1.0));
        }

        s.b[315] = (s.v[90] > 0.0);
        s.v[315] = if s.b[315] { 1.0 } else { 0.0 };

        if (((!s.b[311]) && (!s.b[314])) && s.b[315]) {
            s.store_scale(92, 89, (-0.5));
            s.store_sqrt(93, 90);
            s.store_add(87, 92, 93);
        }

        s.b[316] = (s.v[87] > 0.0);
        s.v[316] = if s.b[316] { 1.0 } else { 0.0 };

        if ((((!s.b[311]) && (!s.b[314])) && s.b[315]) && s.b[316]) {
            s.store_exp_ad(94, A::mul(s.ad_value(83), A::ln(s.ad_value(87))));
        }

        if ((((!s.b[311]) && (!s.b[314])) && s.b[315]) && (!s.b[316])) {
            s.store_neg_ad(94, A::exp(A::mul(s.ad_value(83), A::ln_scaled_input(s.ad_value(87), -1.0))));
        }

        if (((!s.b[311]) && (!s.b[314])) && s.b[315]) {
            s.store_sub(87, 92, 93);
        }

        s.b[317] = (s.v[87] > 0.0);
        s.v[317] = if s.b[317] { 1.0 } else { 0.0 };

        if ((((!s.b[311]) && (!s.b[314])) && s.b[315]) && s.b[317]) {
            s.store_exp_ad(95, A::mul(s.ad_value(83), A::ln(s.ad_value(87))));
        }

        if ((((!s.b[311]) && (!s.b[314])) && s.b[315]) && (!s.b[317])) {
            s.store_neg_ad(95, A::exp(A::mul(s.ad_value(83), A::ln_scaled_input(s.ad_value(87), -1.0))));
        }

        if (((!s.b[311]) && (!s.b[314])) && s.b[315]) {
            s.store_add_scaled_inputs_product_indices(91, 94, 1.0, 95, 1.0, 84, 83, (-1.0));
        }

        if (((!s.b[311]) && (!s.b[314])) && (!s.b[315])) {
            s.store_mul_scaled_ad_rhs(87, 89, (-0.5), A::sqrt(A::div_from_scalar((-27.0), A::mul(A::square(s.ad_value(88)), s.ad_value(88)))));
            s.store_square(92, 87);
        }

        s.b[318] = (s.v[87] >= 0.0);
        s.v[318] = if s.b[318] { 1.0 } else { 0.0 };

        if ((((!s.b[311]) && (!s.b[314])) && (!s.b[315])) && s.b[318]) {
            s.store_sub_from_scalar_ad(87, (3.141592653589793 / 2.0), A::atan(A::sqrt(A::div(s.ad_value(92), A::sub_from_scalar(1.0, s.ad_value(92))))));
        }

        if ((((!s.b[311]) && (!s.b[314])) && (!s.b[315])) && (!s.b[318])) {
            s.store_offset_atan_ad(87, A::sqrt(A::div(s.ad_value(92), A::sub_from_scalar(1.0, s.ad_value(92)))), (3.141592653589793 / 2.0));
        }

        if (((!s.b[311]) && (!s.b[314])) && (!s.b[315])) {
            s.store_add_scaled_products_mixed_aaii(87, A::sqrt(A::mul_scaled_lhs(s.ad_value(88), (-4.0), s.ad_value(83))), A::cos(A::mul(s.ad_value(83), s.ad_value(87))), 1.0, 84, 83, (-1.0));
            s.copy_ad(91, 87);
        }

        if (!s.b[311]) {
            s.copy_ad(125, 91);
        }

        s.b[319] = (s.v[125] < 1e-20);
        s.v[319] = if s.b[319] { 1.0 } else { 0.0 };

        if s.b[319] {
            s.store_scalar(125, 1e-20);
        }

        s.store_div(126, 120, 125);

        s.store_div(127, 121, 125);

    }

    pub(super) fn stamp_reactive_block_3(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        s.b[320] = (s.v[126] < 1e-20);
        s.v[320] = if s.b[320] { 1.0 } else { 0.0 };

        if s.b[320] {
            s.store_scalar(126, 1e-20);
        }

        s.store_mul(138, 117, 126);

        s.store_sub_from_scalar_ad(147, 1.0, A::div(s.ad_value(142), s.ad_value(126)));

        s.store_sqrt_square_offset(144, 147, p.p60);

        s.store_scaled_add(145, 147, 144, 1.0 / ((1.0 + (((1.0 + p.p60)) as f64).sqrt())));

        s.store_mul3_lhs(148, 35, 145, 145);

        s.store_mul(139, 148, 126);

        s.store_mul_ad_rhs(141, 36, A::exp_scaled_input(A::ln(A::div(s.ad_value(126), s.ad_value(142))), p.p58));

        s.store_scaled_mul(140, 141, 126, 1.0 / ((p.p58 + 1.0)));

        s.store_add_scaled_inputs3(137, s.ad_value(138), 1.0, s.ad_value(140), 1.0, s.ad_value(139), 1.0);

        s.store_scale(152, 127, p.p68);

        s.v[47] = p.p44;

        s.b[327] = (s.v[47] < 100.0);
        s.v[327] = if s.b[327] { 1.0 } else { 0.0 };

        s.b[328] = (s.v[24] > 0.0);
        s.v[328] = if s.b[328] { 1.0 } else { 0.0 };

        if (s.b[327] && s.b[328]) {
            s.store_scalar(50, (p.p43 / 4.0));
            s.store_sub_from_scalar(51, s.v[47], 17);
            s.store_scale(52, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
            s.store_scale(53, 24, 2.4);
            s.store_mul_exp_ad_rhs(54, 24, A::mul_offset_lhs(s.ad_value(50), (-p.p43), A::ln(A::div_from_scalar(s.v[47], s.ad_value(17)))));
            s.store_mul_sub_lhs(56, 52, 184, 3);
        }

        s.b[329] = (s.v[56] < 80.0);
        s.v[329] = if s.b[329] { 1.0 } else { 0.0 };

        if ((s.b[327] && s.b[328]) && s.b[329]) {
            s.store_exp(57, 56);
            s.store_add_scaled_product_right_ad(58, 52, 1.0, 2, A::ln(A::offset(s.ad_value(57), 1.0)), (-1.0));
        }

        if ((s.b[327] && s.b[328]) && (!s.b[329])) {
            s.copy_ad(58, 184);
        }

        if (s.b[327] && s.b[328]) {
            s.store_add_scaled_inputs(55, 51, 0.1, 2, 4.0);
            s.store_div_scaled_inputs2(59, s.ad_value(51), 1.0, s.ad_value(58), 1.0, s.ad_value(55), 1.0);
        }

        s.b[330] = (s.v[59] < 80.0);
        s.v[330] = if s.b[330] { 1.0 } else { 0.0 };

        if ((s.b[327] && s.b[328]) && s.b[330]) {
            s.store_exp(57, 59);
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(51), -1.0, s.ad_value(52), -1.0, s.ad_value(55), 1.0)))), 51);
        }

        if ((s.b[327] && s.b[328]) && (!s.b[330])) {
            s.copy_ad(60, 58);
        }

        if (s.b[327] && s.b[328]) {
            s.store_sub(61, 184, 58);
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(17))));
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(17))));
            s.store_scalar(67, (1.0 - p.p43));
            s.store_sub_from_scalar(68, 1.0, 50);
            s.store_div_ad_lhs(62, A::mul_sub_from_scalar_rhs(s.ad_value(24), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67)))), 67);
            s.store_div_ad_lhs(63, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68)))), 68);
            s.store_div_ad_lhs(64, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68)))), 68);
        }

        s.b[331] = (s.v[24] > 0.0);
        s.v[331] = if s.b[331] { 1.0 } else { 0.0 };

        if ((!s.b[327]) && s.b[331]) {
            s.store_scale(76, 17, (1.0 - ((((-((2.4) as f64).ln()) / p.p43)) as f64).exp()));
            s.store_mul_sub_lhs(80, 76, 184, 3);
            s.store_sqrt_square_offset(81, 80, 1.921812);
            s.store_scaled_add(82, 80, 81, 0.5);
            s.store_add_scaled_product_indices(77, 76, 1.0, 2, 82, (-1.0));
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(17))));
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(79, 17, 1.0, A::exp_scaled_input(s.ad_value(78), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
        }

        s.b[349] = (p.p53 < 100.0);
        s.v[349] = if s.b[349] { 1.0 } else { 0.0 };

        s.b[350] = (s.v[30] > 0.0);
        s.v[350] = if s.b[350] { 1.0 } else { 0.0 };

        if (s.b[349] && s.b[350]) {
            s.store_scalar(50, (p.p52 / 4.0));
            s.store_sub_from_scalar(51, p.p53, 19);
            s.store_scale(52, 19, (1.0 - ((((-((2.4) as f64).ln()) / p.p52)) as f64).exp()));
            s.store_scale(53, 30, 2.4);
            s.store_mul_exp_ad_rhs(54, 30, A::mul_offset_lhs(s.ad_value(50), (-p.p52), A::ln(A::div_from_scalar(p.p53, s.ad_value(19)))));
            s.store_mul_sub_lhs(56, 52, 187, 3);
        }

        s.b[351] = (s.v[56] < 80.0);
        s.v[351] = if s.b[351] { 1.0 } else { 0.0 };

        if ((s.b[349] && s.b[350]) && s.b[351]) {
            s.store_exp(57, 56);
            s.store_add_scaled_product_right_ad(58, 52, 1.0, 2, A::ln(A::offset(s.ad_value(57), 1.0)), (-1.0));
        }

        if ((s.b[349] && s.b[350]) && (!s.b[351])) {
            s.copy_ad(58, 187);
        }

        if (s.b[349] && s.b[350]) {
            s.store_add_scaled_inputs(55, 51, 0.1, 2, 4.0);
            s.store_div_scaled_inputs2(59, s.ad_value(51), 1.0, s.ad_value(58), 1.0, s.ad_value(55), 1.0);
        }

        s.b[352] = (s.v[59] < 80.0);
        s.v[352] = if s.b[352] { 1.0 } else { 0.0 };

        if ((s.b[349] && s.b[350]) && s.b[352]) {
            s.store_exp(57, 59);
            s.store_sub_ad_lhs(60, A::mul(s.ad_value(55), A::sub(A::ln(A::offset(s.ad_value(57), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(51), -1.0, s.ad_value(52), -1.0, s.ad_value(55), 1.0)))), 51);
        }

        if ((s.b[349] && s.b[350]) && (!s.b[352])) {
            s.copy_ad(60, 58);
        }

        if (s.b[349] && s.b[350]) {
            s.store_sub(61, 187, 58);
            s.store_ln_ad(65, A::sub_from_scalar(1.0, A::div(s.ad_value(58), s.ad_value(19))));
            s.store_ln_ad(66, A::sub_from_scalar(1.0, A::div(s.ad_value(60), s.ad_value(19))));
            s.store_scalar(67, (1.0 - p.p52));
            s.store_sub_from_scalar(68, 1.0, 50);
            s.store_div_ad_lhs(62, A::mul_sub_from_scalar_rhs(s.ad_value(30), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(67)))), 67);
            s.store_div_ad_lhs(63, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(65), s.ad_value(68)))), 68);
            s.store_div_ad_lhs(64, A::mul_sub_from_scalar_rhs(s.ad_value(54), 1.0, A::exp(A::mul(s.ad_value(66), s.ad_value(68)))), 68);
            s.store_add_scaled_products_left_left_ad(162, A::add_scaled_inputs3(s.ad_value(62), 1.0, s.ad_value(63), 1.0, s.ad_value(64), -1.0), 19, 1.0, 53, 61, 1.0);
        }

        if (s.b[349] && (!s.b[350])) {
            s.store_scalar(162, 0.0);
        }

        s.b[353] = (s.v[30] > 0.0);
        s.v[353] = if s.b[353] { 1.0 } else { 0.0 };

        if ((!s.b[349]) && s.b[353]) {
            s.store_scale(76, 19, (1.0 - ((((-((2.4) as f64).ln()) / p.p52)) as f64).exp()));
            s.store_mul_sub_lhs(80, 76, 187, 3);
            s.store_sqrt_square_offset(81, 80, 1.921812);
            s.store_scaled_add(82, 80, 81, 0.5);
            s.store_add_scaled_product_indices(77, 76, 1.0, 2, 82, (-1.0));
            s.store_ln_ad(78, A::sub_from_scalar(1.0, A::div(s.ad_value(77), s.ad_value(19))));
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(79, 19, 1.0, A::exp_scaled_input(s.ad_value(78), (1.0 - p.p52)), 1.0 / ((1.0 - p.p52)));
            s.store_mul_add_scaled_inputs3_offset_rhs(162, 30, s.ad_value(79), 1.0, s.ad_value(187), 2.4, s.ad_value(77), (-2.4), 0.0);
        }

        if ((!s.b[349]) && (!s.b[353])) {
            s.store_scalar(162, 0.0);
        }

        s.copy_ad(208, 137);

        s.copy_ad(211, 126);

        s.b[355] = ((p.p73 != 0.0) && (p.p54 != 0.0));
        s.v[355] = if s.b[355] { 1.0 } else { 0.0 };

        if s.b[355] {
            s.store_voltage(208, ctx, nodes, Some(8), None);
            s.store_scale(210, 208, (p.p71 * p.p54));
            s.store_voltage(211, ctx, nodes, Some(9), None);
            s.store_scale(213, 211, (p.p72 * p.p54));
        }

        if (!s.b[355]) {
            s.store_scalar(210, 0.0);
            s.store_scalar(213, 0.0);
        }

        s.store_scale(196, 183, p.p70);

        s.store_scale(197, 191, p.p69);

        s.store_add(198, 106, 152);

        s.store_add(199, 98, 208);

        s.store_scale(162, 162, p.p110);

        s.store_scale(105, 105, p.p110);

        s.store_scale(196, 196, p.p110);

        s.store_scale(197, 197, p.p110);

        s.store_scale(198, 198, p.p110);

        s.store_scale(199, 199, p.p110);

        s.b[359] = ((p.p103 == 0.0) || (p.p107 == 0.0));
        s.v[359] = if s.b[359] { 1.0 } else { 0.0 };

        if s.b[359] {
            s.store_scalar(167, 0.0);
        }

        let (assign7590_e7673, assign7590_e7673_d_n0, assign7590_e7673_d_n1, assign7590_e7673_d_n2, assign7590_e7673_d_n3, assign7590_e7673_d_n4, assign7590_e7673_d_n5, assign7590_e7673_d_n6, assign7590_e7673_d_n7, assign7590_e7673_d_n8, assign7590_e7673_d_n9, assign7590_e7673_d_b0, assign7590_e7673_d_b1, assign7590_e7673_d_b2, assign7590_e7673_d_b3, assign7590_e7673_q, assign7590_e7673_q_d_n4,) = {
    if (!s.b[359]) {
        let assign7590_e7670: f64 = (p.p107 * (nv4 - 0.0));
        let assign7590_e7671_q: f64 = assign7590_e7670;
        (assign7590_e7670, 0.0, 0.0, 0.0, 0.0, p.p107, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, assign7590_e7671_q, p.p107,)
    } else {
        (s.v[167], s.dn[167][0], s.dn[167][1], s.dn[167][2], s.dn[167][3], s.dn[167][4], s.dn[167][5], s.dn[167][6], s.dn[167][7], s.dn[167][8], s.dn[167][9], s.db[167][0], s.db[167][1], s.db[167][2], s.db[167][3], 0.0, 0.0,)
    }
};
        s.v[167] = assign7590_e7673;
        s.dn[167][0] = assign7590_e7673_d_n0;
        s.dn[167][1] = assign7590_e7673_d_n1;
        s.dn[167][2] = assign7590_e7673_d_n2;
        s.dn[167][3] = assign7590_e7673_d_n3;
        s.dn[167][4] = assign7590_e7673_d_n4;
        s.dn[167][5] = assign7590_e7673_d_n5;
        s.dn[167][6] = assign7590_e7673_d_n6;
        s.dn[167][7] = assign7590_e7673_d_n7;
        s.dn[167][8] = assign7590_e7673_d_n8;
        s.dn[167][9] = assign7590_e7673_d_n9;
        s.db[167][0] = assign7590_e7673_d_b0;
        s.db[167][1] = assign7590_e7673_d_b1;
        s.db[167][2] = assign7590_e7673_d_b2;
        s.db[167][3] = assign7590_e7673_d_b3;
        s.rv[167] = assign7590_e7673_q;
        s.rdn[167][4] = assign7590_e7673_q_d_n4;

        s.b[360] = ((p.p103 == 0.0) || (p.p104 < p.p111));
        s.v[360] = if s.b[360] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let eq4_e146: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, s.v[162]);
        let eq4_e146_d_n0: f64 = (s.dn[162][0] * ddt_scale);
        let eq4_e146_d_n1: f64 = (s.dn[162][1] * ddt_scale);
        let eq4_e146_d_n2: f64 = (s.dn[162][2] * ddt_scale);
        let eq4_e146_d_n3: f64 = (s.dn[162][3] * ddt_scale);
        let eq4_e146_d_n4: f64 = (s.dn[162][4] * ddt_scale);
        let eq4_e146_d_n5: f64 = (s.dn[162][5] * ddt_scale);
        let eq4_e146_d_n6: f64 = (s.dn[162][6] * ddt_scale);
        let eq4_e146_d_n7: f64 = (s.dn[162][7] * ddt_scale);
        let eq4_e146_d_n8: f64 = (s.dn[162][8] * ddt_scale);
        let eq4_e146_d_n9: f64 = (s.dn[162][9] * ddt_scale);
        let eq4_e146_d_b0: f64 = (s.db[162][0] * ddt_scale);
        let eq4_e146_d_b1: f64 = (s.db[162][1] * ddt_scale);
        let eq4_e146_d_b2: f64 = (s.db[162][2] * ddt_scale);
        let eq4_e146_d_b3: f64 = (s.db[162][3] * ddt_scale);
        let eq4_value: f64 = eq4_e146;
        let eq4_node_derivatives: [f64; 10] = [eq4_e146_d_n0, eq4_e146_d_n1, eq4_e146_d_n2, eq4_e146_d_n3, eq4_e146_d_n4, eq4_e146_d_n5, eq4_e146_d_n6, eq4_e146_d_n7, eq4_e146_d_n8, eq4_e146_d_n9];
        let eq4_branch_derivatives: [f64; 4] = [eq4_e146_d_b0, eq4_e146_d_b1, eq4_e146_d_b2, eq4_e146_d_b3];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(5),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let eq5_e148: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, s.v[105]);
        let eq5_e148_d_n0: f64 = (s.dn[105][0] * ddt_scale);
        let eq5_e148_d_n1: f64 = (s.dn[105][1] * ddt_scale);
        let eq5_e148_d_n2: f64 = (s.dn[105][2] * ddt_scale);
        let eq5_e148_d_n3: f64 = (s.dn[105][3] * ddt_scale);
        let eq5_e148_d_n4: f64 = (s.dn[105][4] * ddt_scale);
        let eq5_e148_d_n5: f64 = (s.dn[105][5] * ddt_scale);
        let eq5_e148_d_n6: f64 = (s.dn[105][6] * ddt_scale);
        let eq5_e148_d_n7: f64 = (s.dn[105][7] * ddt_scale);
        let eq5_e148_d_n8: f64 = (s.dn[105][8] * ddt_scale);
        let eq5_e148_d_n9: f64 = (s.dn[105][9] * ddt_scale);
        let eq5_e148_d_b0: f64 = (s.db[105][0] * ddt_scale);
        let eq5_e148_d_b1: f64 = (s.db[105][1] * ddt_scale);
        let eq5_e148_d_b2: f64 = (s.db[105][2] * ddt_scale);
        let eq5_e148_d_b3: f64 = (s.db[105][3] * ddt_scale);
        let eq5_value: f64 = eq5_e148;
        let eq5_node_derivatives: [f64; 10] = [eq5_e148_d_n0, eq5_e148_d_n1, eq5_e148_d_n2, eq5_e148_d_n3, eq5_e148_d_n4, eq5_e148_d_n5, eq5_e148_d_n6, eq5_e148_d_n7, eq5_e148_d_n8, eq5_e148_d_n9];
        let eq5_branch_derivatives: [f64; 4] = [eq5_e148_d_b0, eq5_e148_d_b1, eq5_e148_d_b2, eq5_e148_d_b3];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(5),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let eq6_e150: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, s.v[196]);
        let eq6_e150_d_n0: f64 = (s.dn[196][0] * ddt_scale);
        let eq6_e150_d_n1: f64 = (s.dn[196][1] * ddt_scale);
        let eq6_e150_d_n2: f64 = (s.dn[196][2] * ddt_scale);
        let eq6_e150_d_n3: f64 = (s.dn[196][3] * ddt_scale);
        let eq6_e150_d_n4: f64 = (s.dn[196][4] * ddt_scale);
        let eq6_e150_d_n5: f64 = (s.dn[196][5] * ddt_scale);
        let eq6_e150_d_n6: f64 = (s.dn[196][6] * ddt_scale);
        let eq6_e150_d_n7: f64 = (s.dn[196][7] * ddt_scale);
        let eq6_e150_d_n8: f64 = (s.dn[196][8] * ddt_scale);
        let eq6_e150_d_n9: f64 = (s.dn[196][9] * ddt_scale);
        let eq6_e150_d_b0: f64 = (s.db[196][0] * ddt_scale);
        let eq6_e150_d_b1: f64 = (s.db[196][1] * ddt_scale);
        let eq6_e150_d_b2: f64 = (s.db[196][2] * ddt_scale);
        let eq6_e150_d_b3: f64 = (s.db[196][3] * ddt_scale);
        let eq6_value: f64 = eq6_e150;
        let eq6_node_derivatives: [f64; 10] = [eq6_e150_d_n0, eq6_e150_d_n1, eq6_e150_d_n2, eq6_e150_d_n3, eq6_e150_d_n4, eq6_e150_d_n5, eq6_e150_d_n6, eq6_e150_d_n7, eq6_e150_d_n8, eq6_e150_d_n9];
        let eq6_branch_derivatives: [f64; 4] = [eq6_e150_d_b0, eq6_e150_d_b1, eq6_e150_d_b2, eq6_e150_d_b3];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(5),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let eq7_e152: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, s.v[197]);
        let eq7_e152_d_n0: f64 = (s.dn[197][0] * ddt_scale);
        let eq7_e152_d_n1: f64 = (s.dn[197][1] * ddt_scale);
        let eq7_e152_d_n2: f64 = (s.dn[197][2] * ddt_scale);
        let eq7_e152_d_n3: f64 = (s.dn[197][3] * ddt_scale);
        let eq7_e152_d_n4: f64 = (s.dn[197][4] * ddt_scale);
        let eq7_e152_d_n5: f64 = (s.dn[197][5] * ddt_scale);
        let eq7_e152_d_n6: f64 = (s.dn[197][6] * ddt_scale);
        let eq7_e152_d_n7: f64 = (s.dn[197][7] * ddt_scale);
        let eq7_e152_d_n8: f64 = (s.dn[197][8] * ddt_scale);
        let eq7_e152_d_n9: f64 = (s.dn[197][9] * ddt_scale);
        let eq7_e152_d_b0: f64 = (s.db[197][0] * ddt_scale);
        let eq7_e152_d_b1: f64 = (s.db[197][1] * ddt_scale);
        let eq7_e152_d_b2: f64 = (s.db[197][2] * ddt_scale);
        let eq7_e152_d_b3: f64 = (s.db[197][3] * ddt_scale);
        let eq7_value: f64 = eq7_e152;
        let eq7_node_derivatives: [f64; 10] = [eq7_e152_d_n0, eq7_e152_d_n1, eq7_e152_d_n2, eq7_e152_d_n3, eq7_e152_d_n4, eq7_e152_d_n5, eq7_e152_d_n6, eq7_e152_d_n7, eq7_e152_d_n8, eq7_e152_d_n9];
        let eq7_branch_derivatives: [f64; 4] = [eq7_e152_d_b0, eq7_e152_d_b1, eq7_e152_d_b2, eq7_e152_d_b3];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(2),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let (eq9_e163,) = {
    if (s.v[356] == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq9_value: f64 = eq9_e163;
        stamper.stamp_potential_const_local(
            0,
            eq9_value,
        );
        let (eq11_e174,) = {
    if (s.v[357] == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq11_value: f64 = eq11_e174;
        stamper.stamp_potential_const_local(
            1,
            eq11_value,
        );
        let (eq13_e185,) = {
    if (s.v[358] == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq13_value: f64 = eq13_e185;
        stamper.stamp_potential_const_local(
            2,
            eq13_value,
        );
        let eq15_e188: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, s.v[198]);
        let eq15_e188_d_n0: f64 = (s.dn[198][0] * ddt_scale);
        let eq15_e188_d_n1: f64 = (s.dn[198][1] * ddt_scale);
        let eq15_e188_d_n2: f64 = (s.dn[198][2] * ddt_scale);
        let eq15_e188_d_n3: f64 = (s.dn[198][3] * ddt_scale);
        let eq15_e188_d_n4: f64 = (s.dn[198][4] * ddt_scale);
        let eq15_e188_d_n5: f64 = (s.dn[198][5] * ddt_scale);
        let eq15_e188_d_n6: f64 = (s.dn[198][6] * ddt_scale);
        let eq15_e188_d_n7: f64 = (s.dn[198][7] * ddt_scale);
        let eq15_e188_d_n8: f64 = (s.dn[198][8] * ddt_scale);
        let eq15_e188_d_n9: f64 = (s.dn[198][9] * ddt_scale);
        let eq15_e188_d_b0: f64 = (s.db[198][0] * ddt_scale);
        let eq15_e188_d_b1: f64 = (s.db[198][1] * ddt_scale);
        let eq15_e188_d_b2: f64 = (s.db[198][2] * ddt_scale);
        let eq15_e188_d_b3: f64 = (s.db[198][3] * ddt_scale);
        let eq15_value: f64 = eq15_e188;
        let eq15_node_derivatives: [f64; 10] = [eq15_e188_d_n0, eq15_e188_d_n1, eq15_e188_d_n2, eq15_e188_d_n3, eq15_e188_d_n4, eq15_e188_d_n5, eq15_e188_d_n6, eq15_e188_d_n7, eq15_e188_d_n8, eq15_e188_d_n9];
        let eq15_branch_derivatives: [f64; 4] = [eq15_e188_d_b0, eq15_e188_d_b1, eq15_e188_d_b2, eq15_e188_d_b3];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let eq17_e191: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, s.v[199]);
        let eq17_e191_d_n0: f64 = (s.dn[199][0] * ddt_scale);
        let eq17_e191_d_n1: f64 = (s.dn[199][1] * ddt_scale);
        let eq17_e191_d_n2: f64 = (s.dn[199][2] * ddt_scale);
        let eq17_e191_d_n3: f64 = (s.dn[199][3] * ddt_scale);
        let eq17_e191_d_n4: f64 = (s.dn[199][4] * ddt_scale);
        let eq17_e191_d_n5: f64 = (s.dn[199][5] * ddt_scale);
        let eq17_e191_d_n6: f64 = (s.dn[199][6] * ddt_scale);
        let eq17_e191_d_n7: f64 = (s.dn[199][7] * ddt_scale);
        let eq17_e191_d_n8: f64 = (s.dn[199][8] * ddt_scale);
        let eq17_e191_d_n9: f64 = (s.dn[199][9] * ddt_scale);
        let eq17_e191_d_b0: f64 = (s.db[199][0] * ddt_scale);
        let eq17_e191_d_b1: f64 = (s.db[199][1] * ddt_scale);
        let eq17_e191_d_b2: f64 = (s.db[199][2] * ddt_scale);
        let eq17_e191_d_b3: f64 = (s.db[199][3] * ddt_scale);
        let eq17_value: f64 = eq17_e191;
        let eq17_node_derivatives: [f64; 10] = [eq17_e191_d_n0, eq17_e191_d_n1, eq17_e191_d_n2, eq17_e191_d_n3, eq17_e191_d_n4, eq17_e191_d_n5, eq17_e191_d_n6, eq17_e191_d_n7, eq17_e191_d_n8, eq17_e191_d_n9];
        let eq17_branch_derivatives: [f64; 4] = [eq17_e191_d_b0, eq17_e191_d_b1, eq17_e191_d_b2, eq17_e191_d_b3];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let (eq19_e196,) = {
    if s.b[360] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e196;
        stamper.stamp_potential_const_local(
            3,
            eq19_value,
        );
        let (eq21_e210, eq21_e210_d_n0, eq21_e210_d_n1, eq21_e210_d_n2, eq21_e210_d_n3, eq21_e210_d_n4, eq21_e210_d_n5, eq21_e210_d_n6, eq21_e210_d_n7, eq21_e210_d_n8, eq21_e210_d_n9, eq21_e210_d_b0, eq21_e210_d_b1, eq21_e210_d_b2, eq21_e210_d_b3,) = {
    if (!s.b[360]) {
        (s.v[167], s.dn[167][0], s.dn[167][1], s.dn[167][2], s.dn[167][3], s.dn[167][4], s.dn[167][5], s.dn[167][6], s.dn[167][7], s.dn[167][8], s.dn[167][9], s.db[167][0], s.db[167][1], s.db[167][2], s.db[167][3],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e210;
        let eq21_node_derivatives: [f64; 10] = [eq21_e210_d_n0, eq21_e210_d_n1, eq21_e210_d_n2, eq21_e210_d_n3, eq21_e210_d_n4, eq21_e210_d_n5, eq21_e210_d_n6, eq21_e210_d_n7, eq21_e210_d_n8, eq21_e210_d_n9];
        let eq21_branch_derivatives: [f64; 4] = [eq21_e210_d_b0, eq21_e210_d_b1, eq21_e210_d_b2, eq21_e210_d_b3];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let eq23_e213: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, s.v[210]);
        let eq23_e213_d_n0: f64 = (s.dn[210][0] * ddt_scale);
        let eq23_e213_d_n1: f64 = (s.dn[210][1] * ddt_scale);
        let eq23_e213_d_n2: f64 = (s.dn[210][2] * ddt_scale);
        let eq23_e213_d_n3: f64 = (s.dn[210][3] * ddt_scale);
        let eq23_e213_d_n4: f64 = (s.dn[210][4] * ddt_scale);
        let eq23_e213_d_n5: f64 = (s.dn[210][5] * ddt_scale);
        let eq23_e213_d_n6: f64 = (s.dn[210][6] * ddt_scale);
        let eq23_e213_d_n7: f64 = (s.dn[210][7] * ddt_scale);
        let eq23_e213_d_n8: f64 = (s.dn[210][8] * ddt_scale);
        let eq23_e213_d_n9: f64 = (s.dn[210][9] * ddt_scale);
        let eq23_e213_d_b0: f64 = (s.db[210][0] * ddt_scale);
        let eq23_e213_d_b1: f64 = (s.db[210][1] * ddt_scale);
        let eq23_e213_d_b2: f64 = (s.db[210][2] * ddt_scale);
        let eq23_e213_d_b3: f64 = (s.db[210][3] * ddt_scale);
        let eq23_value: f64 = eq23_e213;
        let eq23_node_derivatives: [f64; 10] = [eq23_e213_d_n0, eq23_e213_d_n1, eq23_e213_d_n2, eq23_e213_d_n3, eq23_e213_d_n4, eq23_e213_d_n5, eq23_e213_d_n6, eq23_e213_d_n7, eq23_e213_d_n8, eq23_e213_d_n9];
        let eq23_branch_derivatives: [f64; 4] = [eq23_e213_d_b0, eq23_e213_d_b1, eq23_e213_d_b2, eq23_e213_d_b3];
        stamper.stamp_current_dense_local(
            Some(8),
            None,
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let eq25_e216: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, s.v[213]);
        let eq25_e216_d_n0: f64 = (s.dn[213][0] * ddt_scale);
        let eq25_e216_d_n1: f64 = (s.dn[213][1] * ddt_scale);
        let eq25_e216_d_n2: f64 = (s.dn[213][2] * ddt_scale);
        let eq25_e216_d_n3: f64 = (s.dn[213][3] * ddt_scale);
        let eq25_e216_d_n4: f64 = (s.dn[213][4] * ddt_scale);
        let eq25_e216_d_n5: f64 = (s.dn[213][5] * ddt_scale);
        let eq25_e216_d_n6: f64 = (s.dn[213][6] * ddt_scale);
        let eq25_e216_d_n7: f64 = (s.dn[213][7] * ddt_scale);
        let eq25_e216_d_n8: f64 = (s.dn[213][8] * ddt_scale);
        let eq25_e216_d_n9: f64 = (s.dn[213][9] * ddt_scale);
        let eq25_e216_d_b0: f64 = (s.db[213][0] * ddt_scale);
        let eq25_e216_d_b1: f64 = (s.db[213][1] * ddt_scale);
        let eq25_e216_d_b2: f64 = (s.db[213][2] * ddt_scale);
        let eq25_e216_d_b3: f64 = (s.db[213][3] * ddt_scale);
        let eq25_value: f64 = eq25_e216;
        let eq25_node_derivatives: [f64; 10] = [eq25_e216_d_n0, eq25_e216_d_n1, eq25_e216_d_n2, eq25_e216_d_n3, eq25_e216_d_n4, eq25_e216_d_n5, eq25_e216_d_n6, eq25_e216_d_n7, eq25_e216_d_n8, eq25_e216_d_n9];
        let eq25_branch_derivatives: [f64; 4] = [eq25_e216_d_b0, eq25_e216_d_b1, eq25_e216_d_b2, eq25_e216_d_b3];
        stamper.stamp_current_dense_local(
            Some(9),
            None,
            multiplicity * (eq25_value),
            &eq25_node_derivatives,
            &eq25_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let eq4_e146_q: f64 = s.v[162];
        let eq4_reactive_node_derivatives: [f64; 10] = [s.dn[162][0], s.dn[162][1], s.dn[162][2], s.dn[162][3], s.dn[162][4], s.dn[162][5], s.dn[162][6], s.dn[162][7], s.dn[162][8], s.dn[162][9]];
        let eq4_reactive_branch_derivatives: [f64; 4] = [s.db[162][0], s.db[162][1], s.db[162][2], s.db[162][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            nodes,
            &eq4_reactive_node_derivatives,
            branches,
            &eq4_reactive_branch_derivatives,
            multiplicity,
        );
        let eq5_e148_q: f64 = s.v[105];
        let eq5_reactive_node_derivatives: [f64; 10] = [s.dn[105][0], s.dn[105][1], s.dn[105][2], s.dn[105][3], s.dn[105][4], s.dn[105][5], s.dn[105][6], s.dn[105][7], s.dn[105][8], s.dn[105][9]];
        let eq5_reactive_branch_derivatives: [f64; 4] = [s.db[105][0], s.db[105][1], s.db[105][2], s.db[105][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            nodes,
            &eq5_reactive_node_derivatives,
            branches,
            &eq5_reactive_branch_derivatives,
            multiplicity,
        );
        let eq6_e150_q: f64 = s.v[196];
        let eq6_reactive_node_derivatives: [f64; 10] = [s.dn[196][0], s.dn[196][1], s.dn[196][2], s.dn[196][3], s.dn[196][4], s.dn[196][5], s.dn[196][6], s.dn[196][7], s.dn[196][8], s.dn[196][9]];
        let eq6_reactive_branch_derivatives: [f64; 4] = [s.db[196][0], s.db[196][1], s.db[196][2], s.db[196][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            nodes,
            &eq6_reactive_node_derivatives,
            branches,
            &eq6_reactive_branch_derivatives,
            multiplicity,
        );
        let eq7_e152_q: f64 = s.v[197];
        let eq7_reactive_node_derivatives: [f64; 10] = [s.dn[197][0], s.dn[197][1], s.dn[197][2], s.dn[197][3], s.dn[197][4], s.dn[197][5], s.dn[197][6], s.dn[197][7], s.dn[197][8], s.dn[197][9]];
        let eq7_reactive_branch_derivatives: [f64; 4] = [s.db[197][0], s.db[197][1], s.db[197][2], s.db[197][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes,
            &eq7_reactive_node_derivatives,
            branches,
            &eq7_reactive_branch_derivatives,
            multiplicity,
        );
        let eq15_e188_q: f64 = s.v[198];
        let eq15_reactive_node_derivatives: [f64; 10] = [s.dn[198][0], s.dn[198][1], s.dn[198][2], s.dn[198][3], s.dn[198][4], s.dn[198][5], s.dn[198][6], s.dn[198][7], s.dn[198][8], s.dn[198][9]];
        let eq15_reactive_branch_derivatives: [f64; 4] = [s.db[198][0], s.db[198][1], s.db[198][2], s.db[198][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[5]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq17_e191_q: f64 = s.v[199];
        let eq17_reactive_node_derivatives: [f64; 10] = [s.dn[199][0], s.dn[199][1], s.dn[199][2], s.dn[199][3], s.dn[199][4], s.dn[199][5], s.dn[199][6], s.dn[199][7], s.dn[199][8], s.dn[199][9]];
        let eq17_reactive_branch_derivatives: [f64; 4] = [s.db[199][0], s.db[199][1], s.db[199][2], s.db[199][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq17_reactive_node_derivatives,
            branches,
            &eq17_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq21_e210, eq21_e210_d_n0, eq21_e210_d_n1, eq21_e210_d_n2, eq21_e210_d_n3, eq21_e210_d_n4, eq21_e210_d_n5, eq21_e210_d_n6, eq21_e210_d_n7, eq21_e210_d_n8, eq21_e210_d_n9, eq21_e210_d_b0, eq21_e210_d_b1, eq21_e210_d_b2, eq21_e210_d_b3, eq21_e210_q, eq21_e210_q_d_n4,) = {
    if (!s.b[360]) {
        let eq21_e208_q: f64 = s.rv[167];
        (s.v[167], s.dn[167][0], s.dn[167][1], s.dn[167][2], s.dn[167][3], s.dn[167][4], s.dn[167][5], s.dn[167][6], s.dn[167][7], s.dn[167][8], s.dn[167][9], s.db[167][0], s.db[167][1], s.db[167][2], s.db[167][3], eq21_e208_q, s.rdn[167][4],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (eq21_e210_q_d_n4),
        );
        let eq23_e213_q: f64 = s.v[210];
        let eq23_reactive_node_derivatives: [f64; 10] = [s.dn[210][0], s.dn[210][1], s.dn[210][2], s.dn[210][3], s.dn[210][4], s.dn[210][5], s.dn[210][6], s.dn[210][7], s.dn[210][8], s.dn[210][9]];
        let eq23_reactive_branch_derivatives: [f64; 4] = [s.db[210][0], s.db[210][1], s.db[210][2], s.db[210][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            None,
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
        let eq25_e216_q: f64 = s.v[213];
        let eq25_reactive_node_derivatives: [f64; 10] = [s.dn[213][0], s.dn[213][1], s.dn[213][2], s.dn[213][3], s.dn[213][4], s.dn[213][5], s.dn[213][6], s.dn[213][7], s.dn[213][8], s.dn[213][9]];
        let eq25_reactive_branch_derivatives: [f64; 4] = [s.db[213][0], s.db[213][1], s.db[213][2], s.db[213][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            None,
            nodes,
            &eq25_reactive_node_derivatives,
            branches,
            &eq25_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
