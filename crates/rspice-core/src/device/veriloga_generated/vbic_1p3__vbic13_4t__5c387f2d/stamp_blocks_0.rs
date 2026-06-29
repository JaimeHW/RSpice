#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.b[172] = ctx.analysis("static");
        s.v[172] = if s.b[172] { 1.0 } else { 0.0 };

        s.b[175] = param_given[10];
        s.v[175] = if s.b[175] { 1.0 } else { 0.0 };

        if (s.b[172] && s.b[175]) {
            s.store_scalar(165, p.p10);
        }

        if (s.b[172] && (!s.b[175])) {
            s.store_scalar(165, 1e-12);
        }

        s.b[176] = param_given[11];
        s.v[176] = if s.b[176] { 1.0 } else { 0.0 };

        if (s.b[172] && s.b[176]) {
            s.store_scalar(166, p.p11);
        }

        if (s.b[172] && (!s.b[176])) {
            s.store_scalar(166, 1.0);
        }

        s.b[177] = param_given[3];
        s.v[177] = if s.b[177] { 1.0 } else { 0.0 };

        if (s.b[172] && s.b[177]) {
            s.store_scalar(162, 1.0);
        }

        s.b[178] = param_given[4];
        s.v[178] = if s.b[178] { 1.0 } else { 0.0 };

        if ((s.b[172] && (!s.b[177])) && s.b[178]) {
            s.store_scalar(162, (-1.0));
        }

        s.b[179] = param_given[5];
        s.v[179] = if s.b[179] { 1.0 } else { 0.0 };

        if (((s.b[172] && (!s.b[177])) && (!s.b[178])) && s.b[179]) {
            s.store_scalar(162, p.p5);
        }

        if (((s.b[172] && (!s.b[177])) && (!s.b[178])) && (!s.b[179])) {
            s.store_scalar(162, 1.0);
        }

        if s.b[172] {
            s.store_scalar(113, ((p.p12) as f64).ln());
        }

        if s.b[172] {
            s.store_scalar(46, (if (p.p74 > 0.0) { (1.0 / p.p74) } else { 0.0 }));
        }

        if s.b[172] {
            s.store_scalar(47, (if (p.p75 > 0.0) { (1.0 / p.p75) } else { 0.0 }));
        }

        if s.b[172] {
            s.store_scalar(49, (if (p.p20 > 0.0) { (1.0 / p.p20) } else { 0.0 }));
        }

        if s.b[172] {
            s.store_scalar(50, (if (p.p79 > 0.0) { (1.0 / p.p79) } else { 0.0 }));
        }

        if s.b[172] {
            s.store_scalar(51, (if (p.p80 > 0.0) { (1.0 / p.p80) } else { 0.0 }));
        }

        if s.b[172] {
            s.store_scalar(52, (if (p.p80 > 0.0) { 0.0 } else { 1.0 }));
        }

        if s.b[172] {
            s.store_scalar(40, (273.15 + p.p13));
        }

        s.v[38] = ((ctx_temp + p.p0) - 273.15);

        s.b[182] = (s.v[38] < (p.p14 + 1.0));
        s.v[182] = if s.b[182] { 1.0 } else { 0.0 };

        if s.b[182] {
            s.store_scalar(38, (p.p14 + ((((s.v[38] - p.p14) - 1.0)) as f64).exp()));
        }

        s.b[183] = (s.v[38] > (p.p15 - 1.0));
        s.v[183] = if s.b[183] { 1.0 } else { 0.0 };

        if ((!s.b[182]) && s.b[183]) {
            s.store_sub_from_scalar_ad(38, p.p15, A::exp(A::offset(A::sub_from_scalar(p.p15, s.ad_value(38)), (-1.0))));
        }

        if ((!s.b[182]) && (!s.b[183])) {
        }

        s.store_offset(39, 38, 273.15);

        s.store_scale(73, 39, (1.380662e-23 * 6.241460901304403e18));

        s.store_div(41, 39, 40);

        s.b[184] = (p.p90 > 0.0);
        s.v[184] = if s.b[184] { 1.0 } else { 0.0 };

        if s.b[184] {
            s.store_mul_scaled_ln_ad_rhs(64, 73, p.p89, A::add_scaled_inputs(A::exp(A::div_from_scalar((-p.p88), A::scale(s.ad_value(73), p.p89))), 1.0, s.ad_value(166), 1.0 / (p.p90)));
        }

        if (!s.b[184]) {
            s.store_scalar(64, 0.0);
        }

        s.store_scaled_mul_ad(0, A::powf(s.ad_value(41), (p.p122 / p.p28)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p113)), (-p.p113), s.ad_value(73), p.p28)), p.p26);

        s.b[185] = (s.v[0] > 0.0);
        s.v[185] = if s.b[185] { 1.0 } else { 0.0 };

        s.b[186] = ((p.p72 > 0.0) && (s.v[166] > p.p72));
        s.v[186] = if s.b[186] { 1.0 } else { 0.0 };

        if (s.b[185] && s.b[186]) {
            s.store_mul_scaled_ln_ad_rhs(61, 73, p.p28, A::offset(A::div(A::powf(A::scale(s.ad_value(166), (0.5 * (((4.0 / p.p72)) as f64).powf(p.p73))), (1.0 / (1.0 - p.p73))), s.ad_value(0)), 1.0));
        }

        if (s.b[185] && (!s.b[186])) {
            s.store_mul_scaled_ln_ad_rhs(61, 73, p.p28, A::offset(A::div(s.ad_value(166), s.ad_value(0)), 1.0));
        }

        if (!s.b[185]) {
            s.store_scalar(61, 0.0);
        }

        s.store_scaled_mul_ad(1, A::powf(s.ad_value(41), (p.p125 / p.p29)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p121)), (-p.p121), s.ad_value(73), p.p29)), p.p27);

        s.b[187] = ((s.v[0] > 0.0) && (s.v[1] > 0.0));
        s.v[187] = if s.b[187] { 1.0 } else { 0.0 };

        s.b[188] = ((p.p74 > 0.0) && (s.v[166] > p.p74));
        s.v[188] = if s.b[188] { 1.0 } else { 0.0 };

        if (s.b[187] && s.b[188]) {
            s.store_mul_scaled_ln_ad_rhs(62, 73, p.p29, A::offset(A::div(A::powf(A::scale(s.ad_value(166), (0.5 * (((4.0 / p.p74)) as f64).powf(p.p73))), (1.0 / (1.0 - p.p73))), A::mul(s.ad_value(0), s.ad_value(1))), 1.0));
        }

        if (s.b[187] && (!s.b[188])) {
            s.store_mul_scaled_ln_ad_rhs(62, 73, p.p29, A::offset(A::div(s.ad_value(166), A::mul(s.ad_value(0), s.ad_value(1))), 1.0));
        }

        if (!s.b[187]) {
            s.store_scalar(62, 0.0);
        }

        s.store_scaled_mul_ad(5, A::powf(s.ad_value(41), (p.p122 / p.p33)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p120)), (-p.p120), s.ad_value(73), p.p33)), p.p31);

        s.b[189] = (s.v[5] > 0.0);
        s.v[189] = if s.b[189] { 1.0 } else { 0.0 };

        s.b[190] = ((p.p75 > 0.0) && (s.v[166] > p.p75));
        s.v[190] = if s.b[190] { 1.0 } else { 0.0 };

        if (s.b[189] && s.b[190]) {
            s.store_mul_scaled_ln_ad_rhs(63, 73, p.p33, A::offset(A::div_scaled_product(A::square(s.ad_value(166)), s.ad_value(47), 1.0, s.ad_value(5), 1.0), 1.0));
        }

        if (s.b[189] && (!s.b[190])) {
            s.store_mul_scaled_ln_ad_rhs(63, 73, p.p33, A::offset(A::div(s.ad_value(166), s.ad_value(5)), 1.0));
        }

        if (!s.b[189]) {
            s.store_scalar(63, 0.0);
        }

        s.store_scaled_mul_ad(3, A::powf(s.ad_value(41), (p.p123 / p.p56)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p114)), (-p.p114), s.ad_value(73), p.p56)), p.p54);

        s.b[191] = (s.v[3] > 0.0);
        s.v[191] = if s.b[191] { 1.0 } else { 0.0 };

        if s.b[191] {
            s.store_mul_scaled_ln_ad_rhs(65, 73, p.p56, A::offset(A::div(s.ad_value(166), s.ad_value(3)), 1.0));
        }

        if (!s.b[191]) {
            s.store_scalar(65, 0.0);
        }

        s.store_scaled_mul_ad(6, A::powf(s.ad_value(41), (p.p124 / p.p59)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p117)), (-p.p117), s.ad_value(73), p.p59)), p.p58);

        s.b[192] = (s.v[6] > 0.0);
        s.v[192] = if s.b[192] { 1.0 } else { 0.0 };

        if s.b[192] {
            s.store_mul_scaled_ln_ad_rhs(66, 73, p.p59, A::offset(A::div(s.ad_value(166), s.ad_value(6)), 1.0));
        }

        if (!s.b[192]) {
            s.store_scalar(66, 0.0);
        }

        s.store_scaled_mul_ad(4, A::powf(s.ad_value(41), (p.p123 / p.p61)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p115)), (-p.p115), s.ad_value(73), p.p61)), p.p60);

        s.b[193] = (s.v[4] > 0.0);
        s.v[193] = if s.b[193] { 1.0 } else { 0.0 };

        if s.b[193] {
            s.store_mul_scaled_ln_ad_rhs(67, 73, p.p61, A::offset(A::div(s.ad_value(166), s.ad_value(4)), 1.0));
        }

        if (!s.b[193]) {
            s.store_scalar(67, 0.0);
        }

        s.store_scaled_mul_ad(7, A::powf(s.ad_value(41), (p.p124 / p.p63)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p118)), (-p.p118), s.ad_value(73), p.p63)), p.p62);

        s.b[194] = (s.v[7] > 0.0);
        s.v[194] = if s.b[194] { 1.0 } else { 0.0 };

        if s.b[194] {
            s.store_mul_scaled_ln_ad_rhs(68, 73, p.p63, A::offset(A::div(s.ad_value(166), s.ad_value(7)), 1.0));
        }

        if (!s.b[194]) {
            s.store_scalar(68, 0.0);
        }

        s.store_scaled_mul_ad(8, A::powf(s.ad_value(41), (p.p123 / p.p61)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p115)), (-p.p115), s.ad_value(73), p.p61)), p.p64);

        s.b[195] = (s.v[8] > 0.0);
        s.v[195] = if s.b[195] { 1.0 } else { 0.0 };

        if s.b[195] {
            s.store_mul_scaled_ln_ad_rhs(69, 73, p.p61, A::offset(A::div(s.ad_value(166), s.ad_value(8)), 1.0));
        }

        if (!s.b[195]) {
            s.store_scalar(69, 0.0);
        }

        s.store_scaled_mul_ad(9, A::powf(s.ad_value(41), (p.p124 / p.p63)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p118)), (-p.p118), s.ad_value(73), p.p63)), p.p65);

        s.b[196] = (s.v[9] > 0.0);
        s.v[196] = if s.b[196] { 1.0 } else { 0.0 };

        if s.b[196] {
            s.store_mul_scaled_ln_ad_rhs(70, 73, p.p63, A::offset(A::div(s.ad_value(166), s.ad_value(9)), 1.0));
        }

        if (!s.b[196]) {
            s.store_scalar(70, 0.0);
        }

        s.store_scaled_mul_ad(10, A::powf(s.ad_value(41), (p.p123 / p.p67)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p116)), (-p.p116), s.ad_value(73), p.p67)), p.p66);

        s.b[197] = (s.v[10] > 0.0);
        s.v[197] = if s.b[197] { 1.0 } else { 0.0 };

        if s.b[197] {
            s.store_mul_scaled_ln_ad_rhs(71, 73, p.p67, A::offset(A::div(s.ad_value(166), s.ad_value(10)), 1.0));
        }

        if (!s.b[197]) {
            s.store_scalar(71, 0.0);
        }

        s.store_scaled_mul_ad(11, A::powf(s.ad_value(41), (p.p124 / p.p69)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p119)), (-p.p119), s.ad_value(73), p.p69)), p.p68);

        s.b[198] = (s.v[11] > 0.0);
        s.v[198] = if s.b[198] { 1.0 } else { 0.0 };

        if s.b[198] {
            s.store_mul_scaled_ln_ad_rhs(72, 73, p.p69, A::offset(A::div(s.ad_value(166), s.ad_value(11)), 1.0));
        }

        if (!s.b[198]) {
            s.store_scalar(72, 0.0);
        }

        s.store_voltage(138, ctx, nodes, Some(4), None);

        s.store_offset(38, 138, (((ctx_temp + p.p0)) + ((-273.15))));

        s.b[199] = (s.v[38] < (p.p14 + 1.0));
        s.v[199] = if s.b[199] { 1.0 } else { 0.0 };

        if s.b[199] {
            s.store_offset_exp_ad(38, A::offset(s.ad_value(38), (((-p.p14)) + ((-1.0)))), p.p14);
        }

        s.b[200] = (s.v[38] > (p.p15 - 1.0));
        s.v[200] = if s.b[200] { 1.0 } else { 0.0 };

        if ((!s.b[199]) && s.b[200]) {
            s.store_sub_from_scalar_ad(38, p.p15, A::exp(A::offset(A::sub_from_scalar(p.p15, s.ad_value(38)), (-1.0))));
        }

        if ((!s.b[199]) && (!s.b[200])) {
        }

        s.store_offset(39, 38, 273.15);

        s.store_scale(73, 39, (1.380662e-23 * 6.241460901304403e18));

        s.store_div(41, 39, 40);

        s.store_sub(42, 39, 40);

        s.store_scale_ad(2, A::powf(s.ad_value(41), p.p126), p.p72);

        s.b[201] = param_given[109];
        s.v[201] = if s.b[201] { 1.0 } else { 0.0 };

        if s.b[201] {
            s.store_scale_ad(12, A::powf(s.ad_value(41), p.p109), p.p16);
        }

        if (!s.b[201]) {
            s.store_scale_ad(12, A::powf(s.ad_value(41), p.p107), p.p16);
        }

        s.b[202] = param_given[108];
        s.v[202] = if s.b[202] { 1.0 } else { 0.0 };

        if s.b[202] {
            s.store_scale_ad(13, A::powf(s.ad_value(41), p.p108), p.p17);
        }

        if (!s.b[202]) {
            s.store_scale_ad(13, A::powf(s.ad_value(41), p.p107), p.p17);
        }

        s.b[203] = param_given[106];
        s.v[203] = if s.b[203] { 1.0 } else { 0.0 };

        if s.b[203] {
            s.store_scale_ad(14, A::powf(s.ad_value(41), p.p106), p.p21);
        }

        if (!s.b[203]) {
            s.store_scale_ad(14, A::powf(s.ad_value(41), p.p104), p.p21);
        }

        s.b[204] = param_given[105];
        s.v[204] = if s.b[204] { 1.0 } else { 0.0 };

        if s.b[204] {
            s.store_scale_ad(15, A::powf(s.ad_value(41), p.p105), p.p22);
        }

        if (!s.b[204]) {
            s.store_scale_ad(15, A::powf(s.ad_value(41), p.p104), p.p22);
        }

        s.store_scale_ad(16, A::powf(s.ad_value(41), p.p103), p.p23);

        s.store_scale_ad(17, A::powf(s.ad_value(41), p.p111), p.p24);

        s.b[205] = param_given[110];
        s.v[205] = if s.b[205] { 1.0 } else { 0.0 };

        if s.b[205] {
            s.store_scale_ad(18, A::powf(s.ad_value(41), p.p110), p.p25);
        }

        if (!s.b[205]) {
            s.store_scale_ad(18, A::powf(s.ad_value(41), p.p107), p.p25);
        }

        s.store_offset_scaled(19, 42, ((p.p132) * (p.p101)), p.p101);

        s.store_scaled_mul_ad(0, A::powf(s.ad_value(41), (p.p122 / p.p28)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p113)), (-p.p113), s.ad_value(73), p.p28)), p.p26);

        s.store_scaled_mul_ad(1, A::powf(s.ad_value(41), (p.p125 / p.p29)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p121)), (-p.p121), s.ad_value(73), p.p29)), p.p27);

        s.store_scaled_mul_ad(5, A::powf(s.ad_value(41), (p.p122 / p.p33)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p120)), (-p.p120), s.ad_value(73), p.p33)), p.p31);

        s.store_scaled_mul_ad(3, A::powf(s.ad_value(41), (p.p123 / p.p56)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p114)), (-p.p114), s.ad_value(73), p.p56)), p.p54);

        s.store_scaled_mul_ad(6, A::powf(s.ad_value(41), (p.p124 / p.p59)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p117)), (-p.p117), s.ad_value(73), p.p59)), p.p58);

        s.store_scaled_mul_ad(4, A::powf(s.ad_value(41), (p.p123 / p.p61)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p115)), (-p.p115), s.ad_value(73), p.p61)), p.p60);

        s.store_scaled_mul_ad(7, A::powf(s.ad_value(41), (p.p124 / p.p63)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p118)), (-p.p118), s.ad_value(73), p.p63)), p.p62);

        s.store_scaled_mul_ad(8, A::powf(s.ad_value(41), (p.p123 / p.p61)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p115)), (-p.p115), s.ad_value(73), p.p61)), p.p64);

        s.store_scaled_mul_ad(9, A::powf(s.ad_value(41), (p.p124 / p.p63)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p118)), (-p.p118), s.ad_value(73), p.p63)), p.p65);

        s.store_scaled_mul_ad(10, A::powf(s.ad_value(41), (p.p123 / p.p67)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p116)), (-p.p116), s.ad_value(73), p.p67)), p.p66);

        s.store_scaled_mul_ad(11, A::powf(s.ad_value(41), (p.p124 / p.p69)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p119)), (-p.p119), s.ad_value(73), p.p69)), p.p68);

        s.store_offset_scaled(27, 42, ((p.p129) * (p.p28)), p.p28);

        s.store_offset_scaled(28, 42, ((p.p129) * (p.p29)), p.p29);

        s.store_offset_scaled(29, 42, ((p.p127) * (p.p84)), p.p84);

        s.store_offset_scaled(30, 42, ((p.p128) * (p.p86)), p.p86);

        s.store_scaled_offset_ad(31, A::mul(s.ad_value(42), A::scale_offset(s.ad_value(42), p.p92, p.p91)), 1.0, p.p88);

        s.store_offset_scaled(32, 42, ((p.p93) * (p.p89)), p.p89);

        s.store_scaled_mul_ad(206, A::div(s.ad_value(73), s.ad_value(41)), A::ln(A::sub(A::exp_div_scaled_inputs(s.ad_value(41), (0.5 * p.p37), s.ad_value(73), 1.0), A::exp_div_scaled_inputs(s.ad_value(41), ((-0.5) * p.p37), s.ad_value(73), 1.0))), 2.0);

        s.store_sub_ad(207, A::add_scaled_products(s.ad_value(206), s.ad_value(41), 1.0, s.ad_value(73), A::ln(s.ad_value(41)), (-3.0)), A::scaled_offset(s.ad_value(41), (-1.0), p.p114));

        s.store_add_scaled_product_right_ad(20, 207, 1.0, 73, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp_div_scaled_inputs(s.ad_value(207), -1.0, s.ad_value(73), 1.0), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scaled_mul_ad(208, A::div(s.ad_value(73), s.ad_value(41)), A::ln(A::sub(A::exp_div_scaled_inputs(s.ad_value(41), (0.5 * p.p42), s.ad_value(73), 1.0), A::exp_div_scaled_inputs(s.ad_value(41), ((-0.5) * p.p42), s.ad_value(73), 1.0))), 2.0);

        s.store_sub_ad(209, A::add_scaled_products(s.ad_value(208), s.ad_value(41), 1.0, s.ad_value(73), A::ln(s.ad_value(41)), (-3.0)), A::scaled_offset(s.ad_value(41), (-1.0), p.p115));

        s.store_add_scaled_product_right_ad(21, 209, 1.0, 73, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp_div_scaled_inputs(s.ad_value(209), -1.0, s.ad_value(73), 1.0), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scaled_mul_ad(210, A::div(s.ad_value(73), s.ad_value(41)), A::ln(A::sub(A::exp_div_scaled_inputs(s.ad_value(41), (0.5 * p.p50), s.ad_value(73), 1.0), A::exp_div_scaled_inputs(s.ad_value(41), ((-0.5) * p.p50), s.ad_value(73), 1.0))), 2.0);

        s.store_sub_ad(211, A::add_scaled_products(s.ad_value(210), s.ad_value(41), 1.0, s.ad_value(73), A::ln(s.ad_value(41)), (-3.0)), A::scaled_offset(s.ad_value(41), (-1.0), p.p116));

        s.store_add_scaled_product_right_ad(22, 211, 1.0, 73, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp_div_scaled_inputs(s.ad_value(211), -1.0, s.ad_value(73), 1.0), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scaled_powf_ad(23, A::div_from_scalar(p.p37, s.ad_value(20)), p.p38, p.p36);

        s.store_scaled_powf_ad(24, A::div_from_scalar(p.p42, s.ad_value(21)), p.p43, p.p41);

        s.store_scaled_powf_ad(25, A::div_from_scalar(p.p42, s.ad_value(21)), p.p43, p.p48);

        s.store_scaled_powf_ad(26, A::div_from_scalar(p.p50, s.ad_value(22)), p.p51, p.p49);

        s.store_scaled_mul_ad(33, A::powf(s.ad_value(41), p.p122), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p113)), (-p.p113), s.ad_value(73), 1.0)), p.p19);

        s.store_scale_ad(34, A::powf(s.ad_value(41), p.p112), p.p18);

        s.store_ad_value(35, A::exp_div_scaled_inputs(s.ad_value(31), -1.0, A::mul(s.ad_value(32), s.ad_value(73)), 1.0));

        s.store_offset_scaled(36, 42, ((p.p130) * (p.p70)), p.p70);

        s.store_offset_scaled(37, 42, ((p.p131) * (p.p71)), p.p71);

        if (s.v[12] > 0.001) {
            s.store_div_from_scalar(53, 1.0, 12);
        } else {
            s.store_scalar(53, 1000.0);
        }

        if (s.v[13] > 0.001) {
            s.store_div_from_scalar(54, 1.0, 13);
        } else {
            s.store_scalar(54, 1000.0);
        }

        if (s.v[14] > 0.001) {
            s.store_div_from_scalar(55, 1.0, 14);
        } else {
            s.store_scalar(55, 1000.0);
        }

        if (s.v[15] > 0.001) {
            s.store_div_from_scalar(56, 1.0, 15);
        } else {
            s.store_scalar(56, 1000.0);
        }

        if (s.v[16] > 0.001) {
            s.store_div_from_scalar(57, 1.0, 16);
        } else {
            s.store_scalar(57, 1000.0);
        }

        if (s.v[18] > 0.001) {
            s.store_div_from_scalar(58, 1.0, 18);
        } else {
            s.store_scalar(58, 1000.0);
        }

        if (s.v[17] > 0.001) {
            s.store_div_from_scalar(59, 1.0, 17);
        } else {
            s.store_scalar(59, 1000.0);
        }

        if (s.v[19] > 0.001) {
            s.store_div_from_scalar(60, 1.0, 19);
        } else {
            s.store_scalar(60, 1000.0);
        }

        if (s.v[36] > 0.0) {
            s.store_div_from_scalar(43, 1.0, 36);
        } else {
            s.store_scalar(43, 0.0);
        }

        if (s.v[37] > 0.0) {
            s.store_div_from_scalar(44, 1.0, 37);
        } else {
            s.store_scalar(44, 0.0);
        }

        if (s.v[2] > 0.0) {
            s.store_div_from_scalar(45, 1.0, 2);
        } else {
            s.store_scalar(45, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.v[34] > 0.0) {
            s.store_div_from_scalar(48, 1.0, 34);
        } else {
            s.store_scalar(48, 0.0);
        }

        s.store_mul_voltage_ad(143, s.ad_value(162), ctx, nodes, Some(8), Some(9));

        s.store_mul_voltage_ad(145, s.ad_value(162), ctx, nodes, Some(7), Some(9));

        s.store_mul_voltage_ad(144, s.ad_value(162), ctx, nodes, Some(8), Some(6));

        s.store_mul_voltage_ad(148, s.ad_value(162), ctx, nodes, Some(8), Some(5));

        s.store_mul_voltage_ad(149, s.ad_value(162), ctx, nodes, Some(7), Some(5));

        s.store_mul_voltage_ad(146, s.ad_value(162), ctx, nodes, Some(7), Some(10));

        s.store_mul_voltage_ad(160, s.ad_value(162), ctx, nodes, Some(6), Some(9));

        s.store_voltage(153, ctx, nodes, Some(0), Some(5));

        s.store_mul_voltage_ad(154, s.ad_value(162), ctx, nodes, Some(5), Some(6));

        s.store_voltage(155, ctx, nodes, Some(1), Some(7));

        s.store_voltage(156, ctx, nodes, Some(7), Some(8));

        s.store_voltage(157, ctx, nodes, Some(2), Some(9));

        s.store_voltage(158, ctx, nodes, Some(10), Some(5));

        s.store_mul_voltage_ad(147, s.ad_value(162), ctx, nodes, Some(11), Some(10));

        s.store_mul_voltage_ad(161, s.ad_value(162), ctx, nodes, Some(7), Some(11));

        s.store_voltage(159, ctx, nodes, Some(3), Some(11));

        s.store_voltage(132, ctx, nodes, Some(13), None);

        s.store_scale(212, 20, (-p.p34));

        s.b[223] = (p.p39 <= 0.0);
        s.v[223] = if s.b[223] { 1.0 } else { 0.0 };

        if s.b[223] {
            s.store_add(213, 143, 212);
        }

        s.b[224] = (s.v[213] > 0.0);
        s.v[224] = if s.b[224] { 1.0 } else { 0.0 };

        if (s.b[223] && s.b[224]) {
            s.store_scalar(214, (((1.0 - p.p34)) as f64).powf((-p.p38)));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(215, 20, 1.0, 214, (1.0 - p.p34), 1.0 / ((1.0 - p.p38)));
            s.store_mul_ad_product_lhs_mixed_ia(216, 213, A::offset(A::div_scaled_inputs(s.ad_value(213), (0.5 * p.p38), s.ad_value(20), (1.0 - p.p34)), 1.0), 214);
        }

        if (s.b[223] && (!s.b[224])) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(215, 20, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(143), s.ad_value(20))), (1.0 - p.p38)), 1.0 / ((1.0 - p.p38)));
            s.store_scalar(216, 0.0);
        }

        if s.b[223] {
            s.store_add(114, 215, 216);
        }

        if (!s.b[223]) {
            s.store_sqrt_square_offset(217, 212, ((4.0 * p.p39) * p.p39));
            s.store_scaled_add(218, 212, 217, (-0.5));
            s.store_mul_scaled_powf_ad_rhs(219, 20, (-1.0 / ((1.0 - p.p38))), A::sub_from_scalar(1.0, A::div(s.ad_value(218), s.ad_value(20))), (1.0 - p.p38));
            s.store_add(220, 143, 212);
            s.store_sqrt_square_offset(221, 220, ((4.0 * p.p39) * p.p39));
            s.store_add_scaled_inputs3_indices(222, 220, 0.5, 221, (-0.5), 212, -1.0);
            s.store_mul_scaled_powf_ad_rhs(215, 20, (-1.0 / ((1.0 - p.p38))), A::sub_from_scalar(1.0, A::div(s.ad_value(222), s.ad_value(20))), (1.0 - p.p38));
            s.store_sub_ad_lhs(114, A::add_scaled_offset_product_rhs(s.ad_value(215), 1.0, A::add_scaled_inputs3(s.ad_value(143), 1.0, s.ad_value(222), (-1.0), s.ad_value(218), 1.0), A::div_scaled_inputs3(s.ad_value(143), (0.5 * p.p38), s.ad_value(222), ((-1.0) * (0.5 * p.p38)), s.ad_value(218), (0.5 * p.p38), s.ad_value(20), (1.0 - p.p34)), 1.0, (((1.0 - p.p34)) as f64).powf((-p.p38))), 219);
        }

        s.store_scale(225, 21, (-p.p34));

        s.b[246] = (p.p44 <= 0.0);
        s.v[246] = if s.b[246] { 1.0 } else { 0.0 };

        if s.b[246] {
            s.store_add(226, 144, 225);
        }

        s.b[247] = (s.v[226] > 0.0);
        s.v[247] = if s.b[247] { 1.0 } else { 0.0 };

        if (s.b[246] && s.b[247]) {
            s.store_scalar(227, (((1.0 - p.p34)) as f64).powf(((-1.0) - p.p43)));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(228, 21, 1.0, 227, ((1.0 - p.p34) * (1.0 - p.p34)), 1.0 / ((1.0 - p.p43)));
            s.store_mul_ad_product_lhs_mixed_ia(229, 226, A::offset(A::div_scaled_inputs(s.ad_value(226), (0.5 * p.p43), s.ad_value(21), 1.0), (1.0 - p.p34)), 227);
        }

        s.b[248] = ((p.p45 > 0.0) && (s.v[144] < (-p.p45)));
        s.v[248] = if s.b[248] { 1.0 } else { 0.0 };

        if ((s.b[246] && (!s.b[247])) && s.b[248]) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(228, 21, 1.0, A::mul_sub_from_scalar_rhs(A::powf(A::offset(A::div_from_scalar(p.p45, s.ad_value(21)), 1.0), (1.0 - p.p43)), 1.0, A::div_scaled_offset_numerator(s.ad_value(144), (1.0 - p.p43), (p.p45 * (1.0 - p.p43)), A::offset(s.ad_value(21), p.p45), 1.0)), 1.0 / ((1.0 - p.p43)));
        }

        if ((s.b[246] && (!s.b[247])) && (!s.b[248])) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(228, 21, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(144), s.ad_value(21))), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
        }

        if (s.b[246] && (!s.b[247])) {
            s.store_scalar(229, 0.0);
        }

        if s.b[246] {
            s.store_add(116, 228, 229);
        }

        s.b[249] = ((p.p45 > 0.0) && (p.p46 > 0.0));
        s.v[249] = if s.b[249] { 1.0 } else { 0.0 };

        if ((!s.b[246]) && s.b[249]) {
            s.store_div_scaled_offset_numerator(230, s.ad_value(225), 1.0, p.p45, A::sub_from_scalar(p.p45, s.ad_value(225)), 1.0);
            s.store_div_scaled_inputs_mixed_ia(231, 230, 2.0, A::add(A::sqrt_square_offset(A::offset(s.ad_value(230), (-1.0)), ((4.0 * p.p44) * p.p44)), A::sqrt_square_offset(A::offset(s.ad_value(230), 1.0), ((4.0 * p.p46) * p.p46))), 1.0);
            s.store_scaled_sub_ad_lhs(232, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(231), p.p45, s.ad_value(225)), (-p.p45)), 225, 0.5);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(233, 21, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(232), s.ad_value(21))), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
            s.store_div_scaled_inputs2_mixed_aia(234, A::scale_offset(s.ad_value(144), 2.0, p.p45), 1.0, 225, 1.0, A::sub_from_scalar(p.p45, s.ad_value(225)), 1.0);
            s.store_div_scaled_inputs_mixed_ia(235, 234, 2.0, A::add(A::sqrt_square_offset(A::offset(s.ad_value(234), (-1.0)), ((4.0 * p.p44) * p.p44)), A::sqrt_square_offset(A::offset(s.ad_value(234), 1.0), ((4.0 * p.p46) * p.p46))), 1.0);
            s.store_scaled_sub_ad_lhs(236, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(235), p.p45, s.ad_value(225)), (-p.p45)), 225, 0.5);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(228, 21, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(236), s.ad_value(21))), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
            s.store_scaled_offset(237, 235, 1.0, 0.5);
            s.store_powf_ad(238, A::offset(A::div_from_scalar(p.p45, s.ad_value(21)), 1.0), (-p.p43));
            s.store_powf_ad(239, A::offset(A::div(s.ad_value(225), s.ad_value(21)), 1.0), (-p.p43));
            s.store_add_scaled_product_value_ad(240, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(237), s.ad_value(238)), 1.0, 237, 239, 1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(241, 240, s.ad_value(144), 1.0, s.ad_value(236), (-1.0), s.ad_value(232), 1.0, 0.0);
            s.store_add_scaled_inputs3_indices(116, 241, 1.0, 228, 1.0, 233, -1.0);
        }

        if ((!s.b[246]) && (!s.b[249])) {
            s.store_sqrt_square_offset(242, 225, ((4.0 * p.p44) * p.p44));
            s.store_scaled_add(232, 225, 242, (-0.5));
            s.store_mul_scaled_powf_ad_rhs(243, 21, (-1.0 / ((1.0 - p.p43))), A::sub_from_scalar(1.0, A::div(s.ad_value(232), s.ad_value(21))), (1.0 - p.p43));
            s.store_add(244, 144, 225);
            s.store_sqrt_square_offset(245, 244, ((4.0 * p.p44) * p.p44));
            s.store_add_scaled_inputs3_indices(236, 244, 0.5, 245, (-0.5), 225, -1.0);
            s.store_mul_scaled_powf_ad_rhs(228, 21, (-1.0 / ((1.0 - p.p43))), A::sub_from_scalar(1.0, A::div(s.ad_value(236), s.ad_value(21))), (1.0 - p.p43));
            s.store_sub_ad_lhs(116, A::add_scaled_inputs4(s.ad_value(228), 1.0, s.ad_value(144), (((1.0 - p.p34)) as f64).powf((-p.p43)), s.ad_value(236), ((-1.0) * (((1.0 - p.p34)) as f64).powf((-p.p43))), s.ad_value(232), (((1.0 - p.p34)) as f64).powf((-p.p43))), 243);
        }

        s.store_div_from_scalar_mul_ad(112, 1.0, s.ad_value(27), s.ad_value(73));

        s.b[250] = (s.v[143] < s.v[61]);
        s.v[250] = if s.b[250] { 1.0 } else { 0.0 };

        if s.b[250] {
            s.store_exp_mul(109, 143, 112);
        }

        if (!s.b[250]) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(61), s.ad_value(112))), A::mul(A::sub(s.ad_value(143), s.ad_value(61)), s.ad_value(112)), 1.0);
        }

        s.store_mul_offset_rhs(74, 0, 109, (-1.0));

        s.store_div_from_scalar_mul_ad(112, 1.0, s.ad_value(28), s.ad_value(73));

        s.b[251] = (s.v[144] < s.v[62]);
        s.v[251] = if s.b[251] { 1.0 } else { 0.0 };

        if s.b[251] {
            s.store_exp_mul(109, 144, 112);
        }

        if (!s.b[251]) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(62), s.ad_value(112))), A::mul(A::sub(s.ad_value(144), s.ad_value(62)), s.ad_value(112)), 1.0);
        }

        s.store_mul_ad_product_rhs_mixed_ia(75, 0, 1, A::offset(s.ad_value(109), (-1.0)));

        s.store_offset_add_scaled_product(78, A::offset(A::mul(s.ad_value(114), s.ad_value(44)), 1.0), 1.0, s.ad_value(116), s.ad_value(43), 1.0, (-0.0001));

        s.store_offset_add_scaled_inputs_mixed_ai(79, A::sqrt_square_offset(s.ad_value(78), 1e-8), 0.5, 78, 0.5, 0.0001);

        s.store_add_scaled_products_indices(80, 74, 45, 1.0, 75, 46, 1.0);

        s.b[252] = (p.p30 < 0.5);
        s.v[252] = if s.b[252] { 1.0 } else { 0.0 };

        if s.b[252] {
            s.store_add_scaled_ad_lhs(108, A::powf(s.ad_value(79), (1.0 / p.p73)), 80, 4.0);
        }

        s.b[253] = (s.v[108] > 1e-8);
        s.v[253] = if s.b[253] { 1.0 } else { 0.0 };

        if (s.b[252] && s.b[253]) {
            s.store_scaled_add_ad_rhs(81, 79, A::powf(s.ad_value(108), p.p73), 0.5);
        }

        if (s.b[252] && (!s.b[253])) {
            s.store_scaled_offset(81, 79, ((1e-8) as f64).powf(p.p73), 0.5);
        }

        if (!s.b[252]) {
            s.store_offset_scaled(108, 80, 4.0, 1.0);
        }

        s.b[254] = (s.v[108] > 1e-8);
        s.v[254] = if s.b[254] { 1.0 } else { 0.0 };

        if ((!s.b[252]) && s.b[254]) {
            s.store_mul_scaled_offset_ad_rhs(81, 79, 0.5, A::powf(s.ad_value(108), p.p73), 1.0);
        }

        if ((!s.b[252]) && (!s.b[254])) {
            s.store_scale(81, 79, (0.5 * (1.0 + ((1e-8) as f64).powf(p.p73))));
        }

        s.store_div(77, 75, 81);

        s.store_div(76, 74, 81);

        s.copy_ad(137, 132);

        s.b[255] = (p.p31 > 0.0);
        s.v[255] = if s.b[255] { 1.0 } else { 0.0 };

        if s.b[255] {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p33);
        }

        s.b[256] = (s.v[146] < s.v[63]);
        s.v[256] = if s.b[256] { 1.0 } else { 0.0 };

        if (s.b[255] && s.b[256]) {
            s.store_exp_mul(109, 146, 112);
        }

        if (s.b[255] && (!s.b[256])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(63), s.ad_value(112))), A::mul(A::sub(s.ad_value(146), s.ad_value(63)), s.ad_value(112)), 1.0);
        }

        s.b[257] = (s.v[144] < s.v[63]);
        s.v[257] = if s.b[257] { 1.0 } else { 0.0 };

        if (s.b[255] && s.b[257]) {
            s.store_exp_mul(111, 144, 112);
        }

        if (s.b[255] && (!s.b[257])) {
            s.store_mul_offset_rhs_ad(111, A::exp(A::mul(s.ad_value(63), s.ad_value(112))), A::mul(A::sub(s.ad_value(144), s.ad_value(63)), s.ad_value(112)), 1.0);
        }

        if s.b[255] {
            s.store_mul_offset_ad_rhs(82, 5, A::add_scaled_inputs(s.ad_value(109), p.p32, s.ad_value(111), (1.0 - p.p32)), (-1.0));
            s.store_mul(85, 82, 47);
            s.store_offset_scaled(108, 85, 4.0, 1.0);
        }

        s.b[258] = (s.v[108] > 1e-8);
        s.v[258] = if s.b[258] { 1.0 } else { 0.0 };

        if (s.b[255] && s.b[258]) {
            s.store_scaled_offset_ad(86, A::sqrt(s.ad_value(108)), 1.0, 0.5);
        }

        if (s.b[255] && (!s.b[258])) {
            s.store_scalar(86, (0.5 * (1.0 + ((1e-8) as f64).sqrt())));
        }

        s.b[259] = (s.v[147] < s.v[63]);
        s.v[259] = if s.b[259] { 1.0 } else { 0.0 };

        if (s.b[255] && s.b[259]) {
            s.store_exp_mul(109, 147, 112);
        }

        if (s.b[255] && (!s.b[259])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(63), s.ad_value(112))), A::mul(A::sub(s.ad_value(147), s.ad_value(63)), s.ad_value(112)), 1.0);
        }

        if s.b[255] {
            s.store_mul_offset_rhs(83, 5, 109, (-1.0));
            s.store_div_scaled_inputs2_indices(84, 82, 1.0, 83, (-1.0), 86, 1.0);
        }

        if (!s.b[255]) {
            s.store_scalar(82, 0.0);
            s.store_scalar(86, 1.0);
            s.store_scalar(84, 0.0);
        }

        s.b[260] = (p.p55 == 1.0);
        s.v[260] = if s.b[260] { 1.0 } else { 0.0 };

        if s.b[260] {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p56);
        }

        s.b[261] = (s.v[143] < s.v[65]);
        s.v[261] = if s.b[261] { 1.0 } else { 0.0 };

        if (s.b[260] && s.b[261]) {
            s.store_exp_mul(109, 143, 112);
        }

        if (s.b[260] && (!s.b[261])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(65), s.ad_value(112))), A::mul(A::sub(s.ad_value(143), s.ad_value(65)), s.ad_value(112)), 1.0);
        }

        if s.b[260] {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p59);
        }

        s.b[262] = (s.v[143] < s.v[66]);
        s.v[262] = if s.b[262] { 1.0 } else { 0.0 };

        if (s.b[260] && s.b[262]) {
            s.store_exp_mul(110, 143, 112);
        }

        if (s.b[260] && (!s.b[262])) {
            s.store_mul_offset_rhs_ad(110, A::exp(A::mul(s.ad_value(66), s.ad_value(112))), A::mul(A::sub(s.ad_value(143), s.ad_value(66)), s.ad_value(112)), 1.0);
        }

        s.b[263] = (p.p57 > 0.0);
        s.v[263] = if s.b[263] { 1.0 } else { 0.0 };

        if (s.b[260] && s.b[263]) {
            s.store_add_scaled_offset_product_rhs_mixed_aii(87, A::mul3(s.ad_value(3), A::scale_offset(s.ad_value(79), p.p57, (((((-1.0)) * (p.p57))) + (1.0))), A::offset(s.ad_value(109), (-1.0))), 1.0, 6, 110, (-1.0), 1.0);
        }

        if (s.b[260] && (!s.b[263])) {
            s.store_add_scaled_products_mixed_iaia(87, 3, A::offset(s.ad_value(109), (-1.0)), 1.0, 6, A::offset(s.ad_value(110), (-1.0)), 1.0);
        }

        s.b[264] = (p.p88 > 0.0);
        s.v[264] = if s.b[264] { 1.0 } else { 0.0 };

        if (s.b[260] && s.b[264]) {
            s.store_sub_scaled_inputs(150, 31, -1.0, 143, 1.0);
            s.store_div_from_scalar_mul_ad(112, 1.0, s.ad_value(32), s.ad_value(73));
        }

        s.b[265] = (s.v[150] < s.v[64]);
        s.v[265] = if s.b[265] { 1.0 } else { 0.0 };

        if ((s.b[260] && s.b[264]) && s.b[265]) {
            s.store_exp_mul(111, 150, 112);
        }

        if ((s.b[260] && s.b[264]) && (!s.b[265])) {
            s.store_mul_offset_rhs_ad(111, A::exp(A::mul(s.ad_value(64), s.ad_value(112))), A::mul(A::sub(s.ad_value(150), s.ad_value(64)), s.ad_value(112)), 1.0);
        }

        if (s.b[260] && s.b[264]) {
            s.store_add_scaled_inputs3_indices(87, 87, 1.0, 111, (-p.p90), 35, (-(-p.p90)));
        }

        if s.b[260] {
            s.store_scalar(88, 0.0);
        }

        s.b[266] = (p.p55 == 0.0);
        s.v[266] = if s.b[266] { 1.0 } else { 0.0 };

        if ((!s.b[260]) && s.b[266]) {
            s.store_scalar(87, 0.0);
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p56);
        }

        s.b[267] = (s.v[145] < s.v[65]);
        s.v[267] = if s.b[267] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && s.b[266]) && s.b[267]) {
            s.store_exp_mul(109, 145, 112);
        }

        if (((!s.b[260]) && s.b[266]) && (!s.b[267])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(65), s.ad_value(112))), A::mul(A::sub(s.ad_value(145), s.ad_value(65)), s.ad_value(112)), 1.0);
        }

        if ((!s.b[260]) && s.b[266]) {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p59);
        }

        s.b[268] = (s.v[145] < s.v[66]);
        s.v[268] = if s.b[268] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && s.b[266]) && s.b[268]) {
            s.store_exp_mul(110, 145, 112);
        }

        if (((!s.b[260]) && s.b[266]) && (!s.b[268])) {
            s.store_mul_offset_rhs_ad(110, A::exp(A::mul(s.ad_value(66), s.ad_value(112))), A::mul(A::sub(s.ad_value(145), s.ad_value(66)), s.ad_value(112)), 1.0);
        }

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[260]) && s.b[266]) {
            s.store_add_scaled_products_mixed_iaia(88, 3, A::offset(s.ad_value(109), (-1.0)), 1.0, 6, A::offset(s.ad_value(110), (-1.0)), 1.0);
        }

        s.b[269] = (p.p88 > 0.0);
        s.v[269] = if s.b[269] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && s.b[266]) && s.b[269]) {
            s.store_sub_scaled_inputs(150, 31, -1.0, 143, 1.0);
            s.store_div_from_scalar_mul_ad(112, 1.0, s.ad_value(32), s.ad_value(73));
        }

        s.b[270] = (s.v[150] < s.v[64]);
        s.v[270] = if s.b[270] { 1.0 } else { 0.0 };

        if ((((!s.b[260]) && s.b[266]) && s.b[269]) && s.b[270]) {
            s.store_exp_mul(111, 150, 112);
        }

        if ((((!s.b[260]) && s.b[266]) && s.b[269]) && (!s.b[270])) {
            s.store_mul_offset_rhs_ad(111, A::exp(A::mul(s.ad_value(64), s.ad_value(112))), A::mul(A::sub(s.ad_value(150), s.ad_value(64)), s.ad_value(112)), 1.0);
        }

        if (((!s.b[260]) && s.b[266]) && s.b[269]) {
            s.store_add_scaled_inputs3_indices(88, 88, 1.0, 111, (-p.p90), 35, (-(-p.p90)));
        }

        if ((!s.b[260]) && (!s.b[266])) {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p56);
        }

        s.b[271] = (s.v[143] < s.v[65]);
        s.v[271] = if s.b[271] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && (!s.b[266])) && s.b[271]) {
            s.store_exp_mul(109, 143, 112);
        }

        if (((!s.b[260]) && (!s.b[266])) && (!s.b[271])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(65), s.ad_value(112))), A::mul(A::sub(s.ad_value(143), s.ad_value(65)), s.ad_value(112)), 1.0);
        }

        if ((!s.b[260]) && (!s.b[266])) {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p59);
        }

        s.b[272] = (s.v[143] < s.v[66]);
        s.v[272] = if s.b[272] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && (!s.b[266])) && s.b[272]) {
            s.store_exp_mul(110, 143, 112);
        }

        if (((!s.b[260]) && (!s.b[266])) && (!s.b[272])) {
            s.store_mul_offset_rhs_ad(110, A::exp(A::mul(s.ad_value(66), s.ad_value(112))), A::mul(A::sub(s.ad_value(143), s.ad_value(66)), s.ad_value(112)), 1.0);
        }

        s.b[273] = (p.p57 > 0.0);
        s.v[273] = if s.b[273] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && (!s.b[266])) && s.b[273]) {
            s.store_add_scaled_offset_product_rhs_mixed_aii(87, A::mul3(s.ad_value(3), A::scale_offset(s.ad_value(79), p.p57, (((((-1.0)) * (p.p57))) + (1.0))), A::offset(s.ad_value(109), (-1.0))), p.p55, 6, 110, (-1.0), p.p55);
        }

        if (((!s.b[260]) && (!s.b[266])) && (!s.b[273])) {
            s.store_add_scaled_products_mixed_iaia(87, 3, A::offset(s.ad_value(109), (-1.0)), p.p55, 6, A::offset(s.ad_value(110), (-1.0)), p.p55);
        }

        s.b[274] = (p.p88 > 0.0);
        s.v[274] = if s.b[274] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && (!s.b[266])) && s.b[274]) {
            s.store_sub_scaled_inputs(150, 31, -1.0, 143, 1.0);
            s.store_div_from_scalar_mul_ad(112, 1.0, s.ad_value(32), s.ad_value(73));
        }

        s.b[275] = (s.v[150] < s.v[64]);
        s.v[275] = if s.b[275] { 1.0 } else { 0.0 };

        if ((((!s.b[260]) && (!s.b[266])) && s.b[274]) && s.b[275]) {
            s.store_exp_mul(111, 150, 112);
        }

        if ((((!s.b[260]) && (!s.b[266])) && s.b[274]) && (!s.b[275])) {
            s.store_mul_offset_rhs_ad(111, A::exp(A::mul(s.ad_value(64), s.ad_value(112))), A::mul(A::sub(s.ad_value(150), s.ad_value(64)), s.ad_value(112)), 1.0);
        }

        if (((!s.b[260]) && (!s.b[266])) && s.b[274]) {
            s.store_add_scaled_inputs3_indices(87, 87, 1.0, 111, (-(p.p55 * p.p90)), 35, (-(-(p.p55 * p.p90))));
        }

        if ((!s.b[260]) && (!s.b[266])) {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p56);
        }

        s.b[276] = (s.v[145] < s.v[65]);
        s.v[276] = if s.b[276] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && (!s.b[266])) && s.b[276]) {
            s.store_exp_mul(109, 145, 112);
        }

        if (((!s.b[260]) && (!s.b[266])) && (!s.b[276])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(65), s.ad_value(112))), A::mul(A::sub(s.ad_value(145), s.ad_value(65)), s.ad_value(112)), 1.0);
        }

        if ((!s.b[260]) && (!s.b[266])) {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p59);
        }

        s.b[277] = (s.v[145] < s.v[66]);
        s.v[277] = if s.b[277] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && (!s.b[266])) && s.b[277]) {
            s.store_exp_mul(110, 145, 112);
        }

        if (((!s.b[260]) && (!s.b[266])) && (!s.b[277])) {
            s.store_mul_offset_rhs_ad(110, A::exp(A::mul(s.ad_value(66), s.ad_value(112))), A::mul(A::sub(s.ad_value(145), s.ad_value(66)), s.ad_value(112)), 1.0);
        }

        if ((!s.b[260]) && (!s.b[266])) {
            s.store_add_scaled_products_mixed_iaia(88, 3, A::offset(s.ad_value(109), (-1.0)), (1.0 - p.p55), 6, A::offset(s.ad_value(110), (-1.0)), (1.0 - p.p55));
        }

        s.b[278] = (p.p88 > 0.0);
        s.v[278] = if s.b[278] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && (!s.b[266])) && s.b[278]) {
            s.store_sub_scaled_inputs(150, 31, -1.0, 143, 1.0);
            s.store_div_from_scalar_mul_ad(112, 1.0, s.ad_value(32), s.ad_value(73));
        }

        s.b[279] = (s.v[150] < s.v[64]);
        s.v[279] = if s.b[279] { 1.0 } else { 0.0 };

        if ((((!s.b[260]) && (!s.b[266])) && s.b[278]) && s.b[279]) {
            s.store_exp_mul(111, 150, 112);
        }

        if ((((!s.b[260]) && (!s.b[266])) && s.b[278]) && (!s.b[279])) {
            s.store_mul_offset_rhs_ad(111, A::exp(A::mul(s.ad_value(64), s.ad_value(112))), A::mul(A::sub(s.ad_value(150), s.ad_value(64)), s.ad_value(112)), 1.0);
        }

        if (((!s.b[260]) && (!s.b[266])) && s.b[278]) {
            s.store_add_scaled_inputs3_indices(88, 88, 1.0, 111, (-((1.0 - p.p55) * p.p90)), 35, (-(-((1.0 - p.p55) * p.p90))));
        }

        s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p61);

        s.b[280] = (s.v[144] < s.v[67]);
        s.v[280] = if s.b[280] { 1.0 } else { 0.0 };

        if s.b[280] {
            s.store_exp_mul(109, 144, 112);
        }

        if (!s.b[280]) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(67), s.ad_value(112))), A::mul(A::sub(s.ad_value(144), s.ad_value(67)), s.ad_value(112)), 1.0);
        }

        s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p63);

        s.b[281] = (s.v[144] < s.v[68]);
        s.v[281] = if s.b[281] { 1.0 } else { 0.0 };

        if s.b[281] {
            s.store_exp_mul(110, 144, 112);
        }

        if (!s.b[281]) {
            s.store_mul_offset_rhs_ad(110, A::exp(A::mul(s.ad_value(68), s.ad_value(112))), A::mul(A::sub(s.ad_value(144), s.ad_value(68)), s.ad_value(112)), 1.0);
        }

        s.store_add_scaled_products_mixed_iaia(89, 4, A::offset(s.ad_value(109), (-1.0)), 1.0, 7, A::offset(s.ad_value(110), (-1.0)), 1.0);

        s.b[282] = ((p.p64 > 0.0) || (p.p65 > 0.0));
        s.v[282] = if s.b[282] { 1.0 } else { 0.0 };

        if s.b[282] {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p61);
        }

        s.b[283] = (s.v[146] < s.v[69]);
        s.v[283] = if s.b[283] { 1.0 } else { 0.0 };

        if (s.b[282] && s.b[283]) {
            s.store_exp_mul(109, 146, 112);
        }

        if (s.b[282] && (!s.b[283])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(69), s.ad_value(112))), A::mul(A::sub(s.ad_value(146), s.ad_value(69)), s.ad_value(112)), 1.0);
        }

        if s.b[282] {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p63);
        }

        s.b[284] = (s.v[146] < s.v[70]);
        s.v[284] = if s.b[284] { 1.0 } else { 0.0 };

        if (s.b[282] && s.b[284]) {
            s.store_exp_mul(110, 146, 112);
        }

        if (s.b[282] && (!s.b[284])) {
            s.store_mul_offset_rhs_ad(110, A::exp(A::mul(s.ad_value(70), s.ad_value(112))), A::mul(A::sub(s.ad_value(146), s.ad_value(70)), s.ad_value(112)), 1.0);
        }

        if s.b[282] {
            s.store_add_scaled_products_mixed_iaia(91, 8, A::offset(s.ad_value(109), (-1.0)), 1.0, 9, A::offset(s.ad_value(110), (-1.0)), 1.0);
        }

        if (!s.b[282]) {
            s.store_scalar(91, 0.0);
        }

        s.store_div(108, 144, 73);

        s.b[285] = (s.v[108] < s.v[113]);
        s.v[285] = if s.b[285] { 1.0 } else { 0.0 };

        if s.b[285] {
            s.store_exp(109, 108);
        }

        if (!s.b[285]) {
            s.store_mul_offset_rhs_ad(109, A::exp(s.ad_value(113)), A::sub(s.ad_value(108), s.ad_value(113)), 1.0);
        }

        s.store_div(108, 148, 73);

        s.b[286] = (s.v[108] < s.v[113]);
        s.v[286] = if s.b[286] { 1.0 } else { 0.0 };

        if s.b[286] {
            s.store_exp(111, 108);
        }

        if (!s.b[286]) {
            s.store_mul_offset_rhs_ad(111, A::exp(s.ad_value(113)), A::sub(s.ad_value(108), s.ad_value(113)), 1.0);
        }

        s.store_sqrt_offset_ad(103, A::mul(s.ad_value(33), s.ad_value(109)), 1.0);

        s.store_sqrt_offset_ad(104, A::mul(s.ad_value(33), s.ad_value(111)), 1.0);

        s.store_mul(96, 153, 53);

        s.store_div_scaled_offset_numerator(105, s.ad_value(103), 1.0, 1.0, A::offset(s.ad_value(104), 1.0), 1.0);

        s.store_mul_add_scaled_product_rhs(106, 54, s.ad_value(154), 1.0, s.ad_value(73), A::add_scaled_inputs3(s.ad_value(103), 1.0, s.ad_value(104), (-1.0), A::ln(s.ad_value(105)), -1.0), 1.0);

        s.store_div_scaled_product_by_product(107, s.ad_value(48), s.ad_value(106), 1.0, s.ad_value(54), A::offset(A::mul3_scaled_output(s.ad_value(48), s.ad_value(49), A::sqrt_square_offset(s.ad_value(154), 0.01), 0.5), 1.0), 1.0);

        s.store_div_ad_rhs(97, 106, A::sqrt_square_offset(s.ad_value(107), 1.0));

        s.store_mul(98, 155, 55);

        s.store_mul3_lhs(99, 156, 81, 56);

        s.store_mul(100, 157, 57);

        s.store_mul3_lhs(101, 158, 86, 58);

        s.store_mul(102, 159, 59);

        s.b[287] = (p.p83 > 0.0);
        s.v[287] = if s.b[287] { 1.0 } else { 0.0 };

        if s.b[287] {
            s.store_powf_ad(288, A::scaled_offset(s.ad_value(29), 1.0, 0.02), (1.0 / (1.01 - p.p43)));
            s.store_add_ad_lhs(289, A::add_scaled_inputs4(A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(21), 1.0, s.ad_value(144), (-1.0), s.ad_value(288), -1.0), 0.01), 0.5, s.ad_value(21), 0.5, s.ad_value(144), ((-1.0) * 0.5), s.ad_value(288), (-0.5)), 288);
            s.store_mul_scaled_powf_rhs(290, 29, -1.0, 289, (p.p43 - 1.0));
        }

        s.b[293] = (s.v[290] < s.v[113]);
        s.v[293] = if s.b[293] { 1.0 } else { 0.0 };

        if (s.b[287] && s.b[293]) {
            s.store_exp(291, 290);
        }

        if (s.b[287] && (!s.b[293])) {
            s.store_exp(292, 113);
            s.store_mul_offset_ad_rhs(291, 292, A::sub(s.ad_value(290), s.ad_value(113)), 1.0);
        }

        if s.b[287] {
            s.store_scaled_mul(95, 289, 291, p.p83);
            s.store_mul_add_scaled_inputs3_offset_rhs(93, 95, s.ad_value(137), 1.0, s.ad_value(77), (-1.0), s.ad_value(89), -1.0, 0.0);
        }

        if (!s.b[287]) {
            s.store_scalar(93, 0.0);
        }

        s.b[294] = (p.p85 > 0.0);
        s.v[294] = if s.b[294] { 1.0 } else { 0.0 };

        if s.b[294] {
            s.store_powf_ad(295, A::scaled_offset(s.ad_value(30), 1.0, 0.02), (1.0 / (1.01 - p.p87)));
            s.store_add_scaled_inputs4_mixed_aiii(296, A::sqrt_square_offset(A::sub_scaled_inputs(s.ad_value(149), -1.0, s.ad_value(295), 1.0), 0.01), 0.5, 149, (-0.5), 295, ((-1.0) * 0.5), 295, 1.0);
            s.store_mul_scaled_powf_rhs(297, 30, -1.0, 296, (p.p87 - 1.0));
        }

        s.b[300] = (s.v[297] < s.v[113]);
        s.v[300] = if s.b[300] { 1.0 } else { 0.0 };

        if (s.b[294] && s.b[300]) {
            s.store_exp(298, 297);
        }

        if (s.b[294] && (!s.b[300])) {
            s.store_exp(299, 113);
            s.store_mul_offset_ad_rhs(298, 299, A::sub(s.ad_value(297), s.ad_value(113)), 1.0);
        }

        if s.b[294] {
            s.store_scaled_mul(95, 296, 298, p.p85);
            s.store_mul_neg_lhs(94, 96, 95);
        }

        if (!s.b[294]) {
            s.store_scalar(94, 0.0);
        }

        s.b[301] = ((p.p97 > 0.0) && (p.p95 > 0.0));
        s.v[301] = if s.b[301] { 1.0 } else { 0.0 };

        s.b[302] = (p.p94 > 0.0);
        s.v[302] = if s.b[302] { 1.0 } else { 0.0 };

        if (s.b[301] && s.b[302]) {
            s.store_offset_sub_from_scalar_ad(170, 1.0, A::scale(s.ad_value(144), 1.0 / (p.p94)), (-0.1));
            s.store_offset_add_scaled_inputs_mixed_ia(170, 170, 0.5, A::sqrt_square_offset(s.ad_value(170), 0.0001), 0.5, 0.1);
            s.store_scale(168, 170, p.p95);
        }

        if (s.b[301] && (!s.b[302])) {
            s.store_scalar(168, p.p95);
        }

        if s.b[301] {
            s.store_scaled_powf_ad(169, A::offset(A::div(s.ad_value(76), s.ad_value(168)), (-1.0)), p.p96, p.p97);
        }

        if (!s.b[301]) {
            s.store_scalar(169, 0.0);
        }

        s.store_add_scaled_inputs3_indices(90, 89, 1.0, 93, (-1.0), 169, -1.0);

        s.b[303] = ((p.p66 > 0.0) || (p.p68 > 0.0));
        s.v[303] = if s.b[303] { 1.0 } else { 0.0 };

        if s.b[303] {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p67);
        }

        s.b[304] = (s.v[147] < s.v[71]);
        s.v[304] = if s.b[304] { 1.0 } else { 0.0 };

        if (s.b[303] && s.b[304]) {
            s.store_exp_mul(109, 147, 112);
        }

        if (s.b[303] && (!s.b[304])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(71), s.ad_value(112))), A::mul(A::sub(s.ad_value(147), s.ad_value(71)), s.ad_value(112)), 1.0);
        }

        if s.b[303] {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p69);
        }

        s.b[305] = (s.v[147] < s.v[72]);
        s.v[305] = if s.b[305] { 1.0 } else { 0.0 };

        if (s.b[303] && s.b[305]) {
            s.store_exp_mul(110, 147, 112);
        }

        if (s.b[303] && (!s.b[305])) {
            s.store_mul_offset_rhs_ad(110, A::exp(A::mul(s.ad_value(72), s.ad_value(112))), A::mul(A::sub(s.ad_value(147), s.ad_value(72)), s.ad_value(112)), 1.0);
        }

        if s.b[303] {
            s.store_add_scaled_products_mixed_iaia(92, 10, A::offset(s.ad_value(109), (-1.0)), 1.0, 11, A::offset(s.ad_value(110), (-1.0)), 1.0);
        }

        if (!s.b[303]) {
            s.store_scalar(92, 0.0);
        }

        s.store_add_scaled_value_products(140, A::add_scaled_value_products3(A::add_scaled_value_products3(A::add_scaled_value_products3(A::add_scaled_products3(s.ad_value(87), s.ad_value(143), 1.0, s.ad_value(90), s.ad_value(144), 1.0, A::sub(s.ad_value(137), s.ad_value(77)), s.ad_value(160), 1.0), 1.0, s.ad_value(88), s.ad_value(145), 1.0, s.ad_value(91), s.ad_value(146), 1.0, s.ad_value(102), s.ad_value(159), 1.0), 1.0, s.ad_value(92), s.ad_value(147), 1.0, s.ad_value(84), s.ad_value(161), 1.0, s.ad_value(96), s.ad_value(153), 1.0), 1.0, s.ad_value(97), s.ad_value(154), 1.0, s.ad_value(98), s.ad_value(155), 1.0, s.ad_value(99), s.ad_value(156), 1.0), 1.0, s.ad_value(100), s.ad_value(157), 1.0, s.ad_value(101), s.ad_value(158), 1.0);

        s.store_scale(139, 140, (-p.p2));

        s.store_mul(141, 138, 60);

        s.store_sub(133, 132, 76);

        s.store_add_scaled_product_indices(87, 87, 1.0, 165, 143, 1.0);

        s.store_add_scaled_product_indices(88, 88, 1.0, 165, 145, 1.0);

        s.store_add_scaled_product_indices(91, 91, 1.0, 165, 146, 1.0);

        s.store_add_scaled_product_indices(90, 90, 1.0, 165, 144, 1.0);

        s.store_add_scaled_product_indices(94, 94, 1.0, 165, 149, 1.0);

        s.store_add_scaled_product_indices(92, 92, 1.0, 165, 147, 1.0);

        s.store_scaled_mul(87, 162, 87, 1.0);

        s.store_scaled_mul(88, 162, 88, 1.0);

        s.store_scaled_mul(76, 162, 76, 1.0);

        s.store_scaled_mul(137, 162, 137, 1.0);

        s.store_scaled_mul(77, 162, 77, 1.0);

        s.store_scaled_mul(90, 162, 90, 1.0);

        s.store_scaled_mul(94, 162, 94, 1.0);

        s.store_scaled_mul(91, 162, 91, 1.0);

        s.store_scaled_mul(97, 162, 97, 1.0);

        s.store_scaled_mul(92, 162, 92, 1.0);

        s.store_scaled_mul(84, 162, 84, 1.0);

        s.b[306] = (p.p49 > 0.0);
        s.v[306] = if s.b[306] { 1.0 } else { 0.0 };

        if s.b[306] {
            s.store_scale(307, 22, (-p.p34));
        }

        s.b[318] = (p.p52 <= 0.0);
        s.v[318] = if s.b[318] { 1.0 } else { 0.0 };

        if (s.b[306] && s.b[318]) {
            s.store_add(308, 147, 307);
        }

        s.b[319] = (s.v[308] > 0.0);
        s.v[319] = if s.b[319] { 1.0 } else { 0.0 };

        if ((s.b[306] && s.b[318]) && s.b[319]) {
            s.store_scalar(309, (((1.0 - p.p34)) as f64).powf((-p.p51)));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(310, 22, 1.0, 309, (1.0 - p.p34), 1.0 / ((1.0 - p.p51)));
            s.store_mul_ad_product_lhs_mixed_ia(311, 308, A::offset(A::div_scaled_inputs(s.ad_value(308), (0.5 * p.p51), s.ad_value(22), (1.0 - p.p34)), 1.0), 309);
        }

        if ((s.b[306] && s.b[318]) && (!s.b[319])) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(310, 22, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(147), s.ad_value(22))), (1.0 - p.p51)), 1.0 / ((1.0 - p.p51)));
            s.store_scalar(311, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[306] && s.b[318]) {
            s.store_add(118, 310, 311);
        }

        if (s.b[306] && (!s.b[318])) {
            s.store_sqrt_square_offset(312, 307, ((4.0 * p.p52) * p.p52));
            s.store_scaled_add(313, 307, 312, (-0.5));
            s.store_mul_scaled_powf_ad_rhs(314, 22, (-1.0 / ((1.0 - p.p51))), A::sub_from_scalar(1.0, A::div(s.ad_value(313), s.ad_value(22))), (1.0 - p.p51));
            s.store_add(315, 147, 307);
            s.store_sqrt_square_offset(316, 315, ((4.0 * p.p52) * p.p52));
            s.store_add_scaled_inputs3_indices(317, 315, 0.5, 316, (-0.5), 307, -1.0);
            s.store_mul_scaled_powf_ad_rhs(310, 22, (-1.0 / ((1.0 - p.p51))), A::sub_from_scalar(1.0, A::div(s.ad_value(317), s.ad_value(22))), (1.0 - p.p51));
            s.store_sub_ad_lhs(118, A::add_scaled_offset_product_rhs(s.ad_value(310), 1.0, A::add_scaled_inputs3(s.ad_value(147), 1.0, s.ad_value(317), (-1.0), s.ad_value(313), 1.0), A::div_scaled_inputs3(s.ad_value(147), (0.5 * p.p51), s.ad_value(317), ((-1.0) * (0.5 * p.p51)), s.ad_value(313), (0.5 * p.p51), s.ad_value(22), (1.0 - p.p34)), 1.0, (((1.0 - p.p34)) as f64).powf((-p.p51))), 314);
        }

        if (!s.b[306]) {
            s.store_scalar(118, 0.0);
        }

        s.store_scale(320, 20, (-p.p34));

        s.b[331] = (p.p39 <= 0.0);
        s.v[331] = if s.b[331] { 1.0 } else { 0.0 };

        if s.b[331] {
            s.store_add(321, 145, 320);
        }

        s.b[332] = (s.v[321] > 0.0);
        s.v[332] = if s.b[332] { 1.0 } else { 0.0 };

        if (s.b[331] && s.b[332]) {
            s.store_scalar(322, (((1.0 - p.p34)) as f64).powf((-p.p38)));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(323, 20, 1.0, 322, (1.0 - p.p34), 1.0 / ((1.0 - p.p38)));
            s.store_mul_ad_product_lhs_mixed_ia(324, 321, A::offset(A::div_scaled_inputs(s.ad_value(321), (0.5 * p.p38), s.ad_value(20), (1.0 - p.p34)), 1.0), 322);
        }

        if (s.b[331] && (!s.b[332])) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(323, 20, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(145), s.ad_value(20))), (1.0 - p.p38)), 1.0 / ((1.0 - p.p38)));
            s.store_scalar(324, 0.0);
        }

        if s.b[331] {
            s.store_add(115, 323, 324);
        }

        if (!s.b[331]) {
            s.store_sqrt_square_offset(325, 320, ((4.0 * p.p39) * p.p39));
            s.store_scaled_add(326, 320, 325, (-0.5));
            s.store_mul_scaled_powf_ad_rhs(327, 20, (-1.0 / ((1.0 - p.p38))), A::sub_from_scalar(1.0, A::div(s.ad_value(326), s.ad_value(20))), (1.0 - p.p38));
            s.store_add(328, 145, 320);
            s.store_sqrt_square_offset(329, 328, ((4.0 * p.p39) * p.p39));
            s.store_add_scaled_inputs3_indices(330, 328, 0.5, 329, (-0.5), 320, -1.0);
            s.store_mul_scaled_powf_ad_rhs(323, 20, (-1.0 / ((1.0 - p.p38))), A::sub_from_scalar(1.0, A::div(s.ad_value(330), s.ad_value(20))), (1.0 - p.p38));
            s.store_sub_ad_lhs(115, A::add_scaled_offset_product_rhs(s.ad_value(323), 1.0, A::add_scaled_inputs3(s.ad_value(145), 1.0, s.ad_value(330), (-1.0), s.ad_value(326), 1.0), A::div_scaled_inputs3(s.ad_value(145), (0.5 * p.p38), s.ad_value(330), ((-1.0) * (0.5 * p.p38)), s.ad_value(326), (0.5 * p.p38), s.ad_value(20), (1.0 - p.p34)), 1.0, (((1.0 - p.p34)) as f64).powf((-p.p38))), 327);
        }

        s.store_scale(333, 21, (-p.p34));

        s.b[354] = (p.p44 <= 0.0);
        s.v[354] = if s.b[354] { 1.0 } else { 0.0 };

        if s.b[354] {
            s.store_add(334, 146, 333);
        }

        s.b[355] = (s.v[334] > 0.0);
        s.v[355] = if s.b[355] { 1.0 } else { 0.0 };

        if (s.b[354] && s.b[355]) {
            s.store_scalar(335, (((1.0 - p.p34)) as f64).powf(((-1.0) - p.p43)));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(336, 21, 1.0, 335, ((1.0 - p.p34) * (1.0 - p.p34)), 1.0 / ((1.0 - p.p43)));
            s.store_mul_ad_product_lhs_mixed_ia(337, 334, A::offset(A::div_scaled_inputs(s.ad_value(334), (0.5 * p.p43), s.ad_value(21), 1.0), (1.0 - p.p34)), 335);
        }

        s.b[356] = ((p.p45 > 0.0) && (s.v[146] < (-p.p45)));
        s.v[356] = if s.b[356] { 1.0 } else { 0.0 };

        if ((s.b[354] && (!s.b[355])) && s.b[356]) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(336, 21, 1.0, A::mul_sub_from_scalar_rhs(A::powf(A::offset(A::div_from_scalar(p.p45, s.ad_value(21)), 1.0), (1.0 - p.p43)), 1.0, A::div_scaled_offset_numerator(s.ad_value(146), (1.0 - p.p43), (p.p45 * (1.0 - p.p43)), A::offset(s.ad_value(21), p.p45), 1.0)), 1.0 / ((1.0 - p.p43)));
        }

        if ((s.b[354] && (!s.b[355])) && (!s.b[356])) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(336, 21, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(146), s.ad_value(21))), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
        }

        if (s.b[354] && (!s.b[355])) {
            s.store_scalar(337, 0.0);
        }

        if s.b[354] {
            s.store_add(117, 336, 337);
        }

        s.b[357] = ((p.p45 > 0.0) && (p.p46 > 0.0));
        s.v[357] = if s.b[357] { 1.0 } else { 0.0 };

        if ((!s.b[354]) && s.b[357]) {
            s.store_div_scaled_offset_numerator(338, s.ad_value(333), 1.0, p.p45, A::sub_from_scalar(p.p45, s.ad_value(333)), 1.0);
            s.store_div_scaled_inputs_mixed_ia(339, 338, 2.0, A::add(A::sqrt_square_offset(A::offset(s.ad_value(338), (-1.0)), ((4.0 * p.p44) * p.p44)), A::sqrt_square_offset(A::offset(s.ad_value(338), 1.0), ((4.0 * p.p46) * p.p46))), 1.0);
            s.store_scaled_sub_ad_lhs(340, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(339), p.p45, s.ad_value(333)), (-p.p45)), 333, 0.5);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(341, 21, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(340), s.ad_value(21))), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
            s.store_div_scaled_inputs2_mixed_aia(342, A::scale_offset(s.ad_value(146), 2.0, p.p45), 1.0, 333, 1.0, A::sub_from_scalar(p.p45, s.ad_value(333)), 1.0);
            s.store_div_scaled_inputs_mixed_ia(343, 342, 2.0, A::add(A::sqrt_square_offset(A::offset(s.ad_value(342), (-1.0)), ((4.0 * p.p44) * p.p44)), A::sqrt_square_offset(A::offset(s.ad_value(342), 1.0), ((4.0 * p.p46) * p.p46))), 1.0);
            s.store_scaled_sub_ad_lhs(344, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(343), p.p45, s.ad_value(333)), (-p.p45)), 333, 0.5);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(336, 21, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(344), s.ad_value(21))), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
            s.store_scaled_offset(345, 343, 1.0, 0.5);
            s.store_powf_ad(346, A::offset(A::div_from_scalar(p.p45, s.ad_value(21)), 1.0), (-p.p43));
            s.store_powf_ad(347, A::offset(A::div(s.ad_value(333), s.ad_value(21)), 1.0), (-p.p43));
            s.store_add_scaled_product_value_ad(348, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(345), s.ad_value(346)), 1.0, 345, 347, 1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(349, 348, s.ad_value(146), 1.0, s.ad_value(344), (-1.0), s.ad_value(340), 1.0, 0.0);
            s.store_add_scaled_inputs3_indices(117, 349, 1.0, 336, 1.0, 341, -1.0);
        }

        if ((!s.b[354]) && (!s.b[357])) {
            s.store_sqrt_square_offset(350, 333, ((4.0 * p.p44) * p.p44));
            s.store_scaled_add(340, 333, 350, (-0.5));
            s.store_mul_scaled_powf_ad_rhs(351, 21, (-1.0 / ((1.0 - p.p43))), A::sub_from_scalar(1.0, A::div(s.ad_value(340), s.ad_value(21))), (1.0 - p.p43));
            s.store_add(352, 146, 333);
            s.store_sqrt_square_offset(353, 352, ((4.0 * p.p44) * p.p44));
            s.store_add_scaled_inputs3_indices(344, 352, 0.5, 353, (-0.5), 333, -1.0);
            s.store_mul_scaled_powf_ad_rhs(336, 21, (-1.0 / ((1.0 - p.p43))), A::sub_from_scalar(1.0, A::div(s.ad_value(344), s.ad_value(21))), (1.0 - p.p43));
            s.store_sub_ad_lhs(117, A::add_scaled_inputs4(s.ad_value(336), 1.0, s.ad_value(146), (((1.0 - p.p34)) as f64).powf((-p.p43)), s.ad_value(344), ((-1.0) * (((1.0 - p.p34)) as f64).powf((-p.p43))), s.ad_value(340), (((1.0 - p.p34)) as f64).powf((-p.p43))), 351);
        }

        s.b[119] = (s.v[74] > 0.0);
        s.v[119] = if s.b[119] { 1.0 } else { 0.0 };

        s.store_scaled_mul(120, 74, 51, s.v[119]);

        s.store_div_scaled_value_offset_denominator(121, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);

        s.store_scaled_mul(108, 144, 50, 0.6944444444444444);

        s.b[358] = (s.v[108] < s.v[113]);
        s.v[358] = if s.b[358] { 1.0 } else { 0.0 };

        if s.b[358] {
            s.store_exp(109, 108);
        }

        if (!s.b[358]) {
            s.store_mul_offset_rhs_ad(109, A::exp(s.ad_value(113)), A::sub(s.ad_value(108), s.ad_value(113)), 1.0);
        }

        s.store_scaled_mul_scale_offset_rhs_ad(122, A::offset(A::mul_scaled_output(s.ad_value(109), A::add(s.ad_value(52), A::square(s.ad_value(121))), (p.p78 * s.v[119])), 1.0), 79, p.p77, 1.0, p.p76);

        s.store_add_scaled_product_value_ad(123, A::div_scaled_product(s.ad_value(122), s.ad_value(74), 1.0, s.ad_value(81), 1.0), 1.0, 23, 114, p.p55);

        s.store_scaled_mul(124, 23, 115, (1.0 - p.p55));

        s.store_add_scaled_ad_lhs(125, A::add_scaled_product(s.ad_value(75), p.p81, s.ad_value(24), s.ad_value(116), 1.0), 103, p.p47);

        s.store_scale(126, 104, p.p47);

        s.store_add_scaled_product_indices(127, 82, p.p81, 25, 117, 1.0);

        s.store_add_scaled_product_indices(128, 147, p.p53, 26, 118, 1.0);

        s.store_scale(142, 138, p.p102);

        s.store_scaled_mul(123, 162, 123, 1.0);

        s.store_scaled_mul(124, 162, 124, 1.0);

        s.store_scaled_mul(125, 162, 125, 1.0);

        s.store_scaled_mul(126, 162, 126, 1.0);

        s.store_scaled_mul(127, 162, 127, 1.0);

        s.store_scaled_mul(128, 162, 128, 1.0);

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.b[172] = ctx.analysis("static");
        s.v[172] = if s.b[172] { 1.0 } else { 0.0 };

        s.b[176] = param_given[11];
        s.v[176] = if s.b[176] { 1.0 } else { 0.0 };

        if (s.b[172] && s.b[176]) {
            s.store_scalar(166, p.p11);
        }

        if (s.b[172] && (!s.b[176])) {
            s.store_scalar(166, 1.0);
        }

        s.b[177] = param_given[3];
        s.v[177] = if s.b[177] { 1.0 } else { 0.0 };

        if (s.b[172] && s.b[177]) {
            s.store_scalar(162, 1.0);
        }

        s.b[178] = param_given[4];
        s.v[178] = if s.b[178] { 1.0 } else { 0.0 };

        if ((s.b[172] && (!s.b[177])) && s.b[178]) {
            s.store_scalar(162, (-1.0));
        }

        s.b[179] = param_given[5];
        s.v[179] = if s.b[179] { 1.0 } else { 0.0 };

        if (((s.b[172] && (!s.b[177])) && (!s.b[178])) && s.b[179]) {
            s.store_scalar(162, p.p5);
        }

        if (((s.b[172] && (!s.b[177])) && (!s.b[178])) && (!s.b[179])) {
            s.store_scalar(162, 1.0);
        }

        if s.b[172] {
            s.store_scalar(113, ((p.p12) as f64).ln());
        }

        if s.b[172] {
            s.store_scalar(46, (if (p.p74 > 0.0) { (1.0 / p.p74) } else { 0.0 }));
        }

        if s.b[172] {
            s.store_scalar(47, (if (p.p75 > 0.0) { (1.0 / p.p75) } else { 0.0 }));
        }

        if s.b[172] {
            s.store_scalar(50, (if (p.p79 > 0.0) { (1.0 / p.p79) } else { 0.0 }));
        }

        if s.b[172] {
            s.store_scalar(51, (if (p.p80 > 0.0) { (1.0 / p.p80) } else { 0.0 }));
        }

        if s.b[172] {
            s.store_scalar(52, (if (p.p80 > 0.0) { 0.0 } else { 1.0 }));
        }

        if s.b[172] {
            s.store_scalar(40, (273.15 + p.p13));
        }

        s.v[38] = ((ctx_temp + p.p0) - 273.15);

        s.b[182] = (s.v[38] < (p.p14 + 1.0));
        s.v[182] = if s.b[182] { 1.0 } else { 0.0 };

        if s.b[182] {
            s.store_scalar(38, (p.p14 + ((((s.v[38] - p.p14) - 1.0)) as f64).exp()));
        }

        s.b[183] = (s.v[38] > (p.p15 - 1.0));
        s.v[183] = if s.b[183] { 1.0 } else { 0.0 };

        if ((!s.b[182]) && s.b[183]) {
            s.store_sub_from_scalar_ad(38, p.p15, A::exp(A::offset(A::sub_from_scalar(p.p15, s.ad_value(38)), (-1.0))));
        }

        if ((!s.b[182]) && (!s.b[183])) {
        }

        s.store_offset(39, 38, 273.15);

        s.store_scale(73, 39, (1.380662e-23 * 6.241460901304403e18));

        s.store_div(41, 39, 40);

        s.b[184] = (p.p90 > 0.0);
        s.v[184] = if s.b[184] { 1.0 } else { 0.0 };

        if s.b[184] {
            s.store_mul_scaled_ln_ad_rhs(64, 73, p.p89, A::add_scaled_inputs(A::exp(A::div_from_scalar((-p.p88), A::scale(s.ad_value(73), p.p89))), 1.0, s.ad_value(166), 1.0 / (p.p90)));
        }

        if (!s.b[184]) {
            s.store_scalar(64, 0.0);
        }

        s.store_scaled_mul_ad(0, A::powf(s.ad_value(41), (p.p122 / p.p28)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p113)), (-p.p113), s.ad_value(73), p.p28)), p.p26);

        s.b[185] = (s.v[0] > 0.0);
        s.v[185] = if s.b[185] { 1.0 } else { 0.0 };

        s.b[186] = ((p.p72 > 0.0) && (s.v[166] > p.p72));
        s.v[186] = if s.b[186] { 1.0 } else { 0.0 };

        if (s.b[185] && s.b[186]) {
            s.store_mul_scaled_ln_ad_rhs(61, 73, p.p28, A::offset(A::div(A::powf(A::scale(s.ad_value(166), (0.5 * (((4.0 / p.p72)) as f64).powf(p.p73))), (1.0 / (1.0 - p.p73))), s.ad_value(0)), 1.0));
        }

        if (s.b[185] && (!s.b[186])) {
            s.store_mul_scaled_ln_ad_rhs(61, 73, p.p28, A::offset(A::div(s.ad_value(166), s.ad_value(0)), 1.0));
        }

        if (!s.b[185]) {
            s.store_scalar(61, 0.0);
        }

        s.store_scaled_mul_ad(1, A::powf(s.ad_value(41), (p.p125 / p.p29)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p121)), (-p.p121), s.ad_value(73), p.p29)), p.p27);

        s.b[187] = ((s.v[0] > 0.0) && (s.v[1] > 0.0));
        s.v[187] = if s.b[187] { 1.0 } else { 0.0 };

        s.b[188] = ((p.p74 > 0.0) && (s.v[166] > p.p74));
        s.v[188] = if s.b[188] { 1.0 } else { 0.0 };

        if (s.b[187] && s.b[188]) {
            s.store_mul_scaled_ln_ad_rhs(62, 73, p.p29, A::offset(A::div(A::powf(A::scale(s.ad_value(166), (0.5 * (((4.0 / p.p74)) as f64).powf(p.p73))), (1.0 / (1.0 - p.p73))), A::mul(s.ad_value(0), s.ad_value(1))), 1.0));
        }

        if (s.b[187] && (!s.b[188])) {
            s.store_mul_scaled_ln_ad_rhs(62, 73, p.p29, A::offset(A::div(s.ad_value(166), A::mul(s.ad_value(0), s.ad_value(1))), 1.0));
        }

        if (!s.b[187]) {
            s.store_scalar(62, 0.0);
        }

        s.store_scaled_mul_ad(5, A::powf(s.ad_value(41), (p.p122 / p.p33)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p120)), (-p.p120), s.ad_value(73), p.p33)), p.p31);

        s.b[189] = (s.v[5] > 0.0);
        s.v[189] = if s.b[189] { 1.0 } else { 0.0 };

        s.b[190] = ((p.p75 > 0.0) && (s.v[166] > p.p75));
        s.v[190] = if s.b[190] { 1.0 } else { 0.0 };

        if (s.b[189] && s.b[190]) {
            s.store_mul_scaled_ln_ad_rhs(63, 73, p.p33, A::offset(A::div_scaled_product(A::square(s.ad_value(166)), s.ad_value(47), 1.0, s.ad_value(5), 1.0), 1.0));
        }

        if (s.b[189] && (!s.b[190])) {
            s.store_mul_scaled_ln_ad_rhs(63, 73, p.p33, A::offset(A::div(s.ad_value(166), s.ad_value(5)), 1.0));
        }

        if (!s.b[189]) {
            s.store_scalar(63, 0.0);
        }

        s.store_scaled_mul_ad(3, A::powf(s.ad_value(41), (p.p123 / p.p56)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p114)), (-p.p114), s.ad_value(73), p.p56)), p.p54);

        s.b[191] = (s.v[3] > 0.0);
        s.v[191] = if s.b[191] { 1.0 } else { 0.0 };

        if s.b[191] {
            s.store_mul_scaled_ln_ad_rhs(65, 73, p.p56, A::offset(A::div(s.ad_value(166), s.ad_value(3)), 1.0));
        }

        if (!s.b[191]) {
            s.store_scalar(65, 0.0);
        }

        s.store_scaled_mul_ad(4, A::powf(s.ad_value(41), (p.p123 / p.p61)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p115)), (-p.p115), s.ad_value(73), p.p61)), p.p60);

        s.b[193] = (s.v[4] > 0.0);
        s.v[193] = if s.b[193] { 1.0 } else { 0.0 };

        if s.b[193] {
            s.store_mul_scaled_ln_ad_rhs(67, 73, p.p61, A::offset(A::div(s.ad_value(166), s.ad_value(4)), 1.0));
        }

        if (!s.b[193]) {
            s.store_scalar(67, 0.0);
        }

        s.store_scaled_mul_ad(8, A::powf(s.ad_value(41), (p.p123 / p.p61)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p115)), (-p.p115), s.ad_value(73), p.p61)), p.p64);

        s.b[195] = (s.v[8] > 0.0);
        s.v[195] = if s.b[195] { 1.0 } else { 0.0 };

        if s.b[195] {
            s.store_mul_scaled_ln_ad_rhs(69, 73, p.p61, A::offset(A::div(s.ad_value(166), s.ad_value(8)), 1.0));
        }

        if (!s.b[195]) {
            s.store_scalar(69, 0.0);
        }

        s.store_scaled_mul_ad(10, A::powf(s.ad_value(41), (p.p123 / p.p67)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p116)), (-p.p116), s.ad_value(73), p.p67)), p.p66);

        s.b[197] = (s.v[10] > 0.0);
        s.v[197] = if s.b[197] { 1.0 } else { 0.0 };

        if s.b[197] {
            s.store_mul_scaled_ln_ad_rhs(71, 73, p.p67, A::offset(A::div(s.ad_value(166), s.ad_value(10)), 1.0));
        }

        if (!s.b[197]) {
            s.store_scalar(71, 0.0);
        }

        s.store_voltage(138, ctx, nodes, Some(4), None);

        s.store_offset(38, 138, (((ctx_temp + p.p0)) + ((-273.15))));

        s.b[199] = (s.v[38] < (p.p14 + 1.0));
        s.v[199] = if s.b[199] { 1.0 } else { 0.0 };

        if s.b[199] {
            s.store_offset_exp_ad(38, A::offset(s.ad_value(38), (((-p.p14)) + ((-1.0)))), p.p14);
        }

        s.b[200] = (s.v[38] > (p.p15 - 1.0));
        s.v[200] = if s.b[200] { 1.0 } else { 0.0 };

        if ((!s.b[199]) && s.b[200]) {
            s.store_sub_from_scalar_ad(38, p.p15, A::exp(A::offset(A::sub_from_scalar(p.p15, s.ad_value(38)), (-1.0))));
        }

        if ((!s.b[199]) && (!s.b[200])) {
        }

        s.store_offset(39, 38, 273.15);

        s.store_scale(73, 39, (1.380662e-23 * 6.241460901304403e18));

        s.store_div(41, 39, 40);

        s.store_sub(42, 39, 40);

        s.store_scale_ad(2, A::powf(s.ad_value(41), p.p126), p.p72);

        s.store_scaled_mul_ad(0, A::powf(s.ad_value(41), (p.p122 / p.p28)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p113)), (-p.p113), s.ad_value(73), p.p28)), p.p26);

        s.store_scaled_mul_ad(1, A::powf(s.ad_value(41), (p.p125 / p.p29)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p121)), (-p.p121), s.ad_value(73), p.p29)), p.p27);

        s.store_scaled_mul_ad(5, A::powf(s.ad_value(41), (p.p122 / p.p33)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p120)), (-p.p120), s.ad_value(73), p.p33)), p.p31);

        s.store_scaled_mul_ad(3, A::powf(s.ad_value(41), (p.p123 / p.p56)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p114)), (-p.p114), s.ad_value(73), p.p56)), p.p54);

        s.store_scaled_mul_ad(4, A::powf(s.ad_value(41), (p.p123 / p.p61)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p115)), (-p.p115), s.ad_value(73), p.p61)), p.p60);

        s.store_scaled_mul_ad(8, A::powf(s.ad_value(41), (p.p123 / p.p61)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p115)), (-p.p115), s.ad_value(73), p.p61)), p.p64);

        s.store_scaled_mul_ad(10, A::powf(s.ad_value(41), (p.p123 / p.p67)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p116)), (-p.p116), s.ad_value(73), p.p67)), p.p66);

        s.store_offset_scaled(27, 42, ((p.p129) * (p.p28)), p.p28);

        s.store_offset_scaled(28, 42, ((p.p129) * (p.p29)), p.p29);

        s.store_scaled_offset_ad(31, A::mul(s.ad_value(42), A::scale_offset(s.ad_value(42), p.p92, p.p91)), 1.0, p.p88);

        s.store_offset_scaled(32, 42, ((p.p93) * (p.p89)), p.p89);

        s.store_scaled_mul_ad(206, A::div(s.ad_value(73), s.ad_value(41)), A::ln(A::sub(A::exp_div_scaled_inputs(s.ad_value(41), (0.5 * p.p37), s.ad_value(73), 1.0), A::exp_div_scaled_inputs(s.ad_value(41), ((-0.5) * p.p37), s.ad_value(73), 1.0))), 2.0);

        s.store_sub_ad(207, A::add_scaled_products(s.ad_value(206), s.ad_value(41), 1.0, s.ad_value(73), A::ln(s.ad_value(41)), (-3.0)), A::scaled_offset(s.ad_value(41), (-1.0), p.p114));

        s.store_add_scaled_product_right_ad(20, 207, 1.0, 73, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp_div_scaled_inputs(s.ad_value(207), -1.0, s.ad_value(73), 1.0), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scaled_mul_ad(208, A::div(s.ad_value(73), s.ad_value(41)), A::ln(A::sub(A::exp_div_scaled_inputs(s.ad_value(41), (0.5 * p.p42), s.ad_value(73), 1.0), A::exp_div_scaled_inputs(s.ad_value(41), ((-0.5) * p.p42), s.ad_value(73), 1.0))), 2.0);

        s.store_sub_ad(209, A::add_scaled_products(s.ad_value(208), s.ad_value(41), 1.0, s.ad_value(73), A::ln(s.ad_value(41)), (-3.0)), A::scaled_offset(s.ad_value(41), (-1.0), p.p115));

        s.store_add_scaled_product_right_ad(21, 209, 1.0, 73, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp_div_scaled_inputs(s.ad_value(209), -1.0, s.ad_value(73), 1.0), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scaled_mul_ad(210, A::div(s.ad_value(73), s.ad_value(41)), A::ln(A::sub(A::exp_div_scaled_inputs(s.ad_value(41), (0.5 * p.p50), s.ad_value(73), 1.0), A::exp_div_scaled_inputs(s.ad_value(41), ((-0.5) * p.p50), s.ad_value(73), 1.0))), 2.0);

        s.store_sub_ad(211, A::add_scaled_products(s.ad_value(210), s.ad_value(41), 1.0, s.ad_value(73), A::ln(s.ad_value(41)), (-3.0)), A::scaled_offset(s.ad_value(41), (-1.0), p.p116));

        s.store_add_scaled_product_right_ad(22, 211, 1.0, 73, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp_div_scaled_inputs(s.ad_value(211), -1.0, s.ad_value(73), 1.0), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scaled_powf_ad(23, A::div_from_scalar(p.p37, s.ad_value(20)), p.p38, p.p36);

        s.store_scaled_powf_ad(24, A::div_from_scalar(p.p42, s.ad_value(21)), p.p43, p.p41);

        s.store_scaled_powf_ad(25, A::div_from_scalar(p.p42, s.ad_value(21)), p.p43, p.p48);

        s.store_scaled_powf_ad(26, A::div_from_scalar(p.p50, s.ad_value(22)), p.p51, p.p49);

        s.store_scaled_mul_ad(33, A::powf(s.ad_value(41), p.p122), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p113)), (-p.p113), s.ad_value(73), 1.0)), p.p19);

        s.store_offset_scaled(36, 42, ((p.p130) * (p.p70)), p.p70);

        s.store_offset_scaled(37, 42, ((p.p131) * (p.p71)), p.p71);

        if (s.v[36] > 0.0) {
            s.store_div_from_scalar(43, 1.0, 36);
        } else {
            s.store_scalar(43, 0.0);
        }

        if (s.v[37] > 0.0) {
            s.store_div_from_scalar(44, 1.0, 37);
        } else {
            s.store_scalar(44, 0.0);
        }

        if (s.v[2] > 0.0) {
            s.store_div_from_scalar(45, 1.0, 2);
        } else {
            s.store_scalar(45, 0.0);
        }

        s.store_mul_voltage_ad(143, s.ad_value(162), ctx, nodes, Some(8), Some(9));

        s.store_mul_voltage_ad(145, s.ad_value(162), ctx, nodes, Some(7), Some(9));

        s.store_mul_voltage_ad(144, s.ad_value(162), ctx, nodes, Some(8), Some(6));

        s.store_mul_voltage_ad(148, s.ad_value(162), ctx, nodes, Some(8), Some(5));

        s.store_mul_voltage_ad(146, s.ad_value(162), ctx, nodes, Some(7), Some(10));

        s.store_mul_voltage_ad(147, s.ad_value(162), ctx, nodes, Some(11), Some(10));

        s.store_scale(212, 20, (-p.p34));

        s.b[223] = (p.p39 <= 0.0);
        s.v[223] = if s.b[223] { 1.0 } else { 0.0 };

        if s.b[223] {
            s.store_add(213, 143, 212);
        }

        s.b[224] = (s.v[213] > 0.0);
        s.v[224] = if s.b[224] { 1.0 } else { 0.0 };

        if (s.b[223] && s.b[224]) {
            s.store_scalar(214, (((1.0 - p.p34)) as f64).powf((-p.p38)));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(215, 20, 1.0, 214, (1.0 - p.p34), 1.0 / ((1.0 - p.p38)));
            s.store_mul_ad_product_lhs_mixed_ia(216, 213, A::offset(A::div_scaled_inputs(s.ad_value(213), (0.5 * p.p38), s.ad_value(20), (1.0 - p.p34)), 1.0), 214);
        }

        if (s.b[223] && (!s.b[224])) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(215, 20, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(143), s.ad_value(20))), (1.0 - p.p38)), 1.0 / ((1.0 - p.p38)));
            s.store_scalar(216, 0.0);
        }

        if s.b[223] {
            s.store_add(114, 215, 216);
        }

        if (!s.b[223]) {
            s.store_sqrt_square_offset(217, 212, ((4.0 * p.p39) * p.p39));
            s.store_scaled_add(218, 212, 217, (-0.5));
            s.store_mul_scaled_powf_ad_rhs(219, 20, (-1.0 / ((1.0 - p.p38))), A::sub_from_scalar(1.0, A::div(s.ad_value(218), s.ad_value(20))), (1.0 - p.p38));
            s.store_add(220, 143, 212);
            s.store_sqrt_square_offset(221, 220, ((4.0 * p.p39) * p.p39));
            s.store_add_scaled_inputs3_indices(222, 220, 0.5, 221, (-0.5), 212, -1.0);
            s.store_mul_scaled_powf_ad_rhs(215, 20, (-1.0 / ((1.0 - p.p38))), A::sub_from_scalar(1.0, A::div(s.ad_value(222), s.ad_value(20))), (1.0 - p.p38));
            s.store_sub_ad_lhs(114, A::add_scaled_offset_product_rhs(s.ad_value(215), 1.0, A::add_scaled_inputs3(s.ad_value(143), 1.0, s.ad_value(222), (-1.0), s.ad_value(218), 1.0), A::div_scaled_inputs3(s.ad_value(143), (0.5 * p.p38), s.ad_value(222), ((-1.0) * (0.5 * p.p38)), s.ad_value(218), (0.5 * p.p38), s.ad_value(20), (1.0 - p.p34)), 1.0, (((1.0 - p.p34)) as f64).powf((-p.p38))), 219);
        }

        s.store_scale(225, 21, (-p.p34));

        s.b[246] = (p.p44 <= 0.0);
        s.v[246] = if s.b[246] { 1.0 } else { 0.0 };

        if s.b[246] {
            s.store_add(226, 144, 225);
        }

        s.b[247] = (s.v[226] > 0.0);
        s.v[247] = if s.b[247] { 1.0 } else { 0.0 };

        if (s.b[246] && s.b[247]) {
            s.store_scalar(227, (((1.0 - p.p34)) as f64).powf(((-1.0) - p.p43)));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(228, 21, 1.0, 227, ((1.0 - p.p34) * (1.0 - p.p34)), 1.0 / ((1.0 - p.p43)));
            s.store_mul_ad_product_lhs_mixed_ia(229, 226, A::offset(A::div_scaled_inputs(s.ad_value(226), (0.5 * p.p43), s.ad_value(21), 1.0), (1.0 - p.p34)), 227);
        }

        s.b[248] = ((p.p45 > 0.0) && (s.v[144] < (-p.p45)));
        s.v[248] = if s.b[248] { 1.0 } else { 0.0 };

        if ((s.b[246] && (!s.b[247])) && s.b[248]) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(228, 21, 1.0, A::mul_sub_from_scalar_rhs(A::powf(A::offset(A::div_from_scalar(p.p45, s.ad_value(21)), 1.0), (1.0 - p.p43)), 1.0, A::div_scaled_offset_numerator(s.ad_value(144), (1.0 - p.p43), (p.p45 * (1.0 - p.p43)), A::offset(s.ad_value(21), p.p45), 1.0)), 1.0 / ((1.0 - p.p43)));
        }

        if ((s.b[246] && (!s.b[247])) && (!s.b[248])) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(228, 21, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(144), s.ad_value(21))), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
        }

        if (s.b[246] && (!s.b[247])) {
            s.store_scalar(229, 0.0);
        }

        if s.b[246] {
            s.store_add(116, 228, 229);
        }

        s.b[249] = ((p.p45 > 0.0) && (p.p46 > 0.0));
        s.v[249] = if s.b[249] { 1.0 } else { 0.0 };

        if ((!s.b[246]) && s.b[249]) {
            s.store_div_scaled_offset_numerator(230, s.ad_value(225), 1.0, p.p45, A::sub_from_scalar(p.p45, s.ad_value(225)), 1.0);
            s.store_div_scaled_inputs_mixed_ia(231, 230, 2.0, A::add(A::sqrt_square_offset(A::offset(s.ad_value(230), (-1.0)), ((4.0 * p.p44) * p.p44)), A::sqrt_square_offset(A::offset(s.ad_value(230), 1.0), ((4.0 * p.p46) * p.p46))), 1.0);
            s.store_scaled_sub_ad_lhs(232, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(231), p.p45, s.ad_value(225)), (-p.p45)), 225, 0.5);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(233, 21, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(232), s.ad_value(21))), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
            s.store_div_scaled_inputs2_mixed_aia(234, A::scale_offset(s.ad_value(144), 2.0, p.p45), 1.0, 225, 1.0, A::sub_from_scalar(p.p45, s.ad_value(225)), 1.0);
            s.store_div_scaled_inputs_mixed_ia(235, 234, 2.0, A::add(A::sqrt_square_offset(A::offset(s.ad_value(234), (-1.0)), ((4.0 * p.p44) * p.p44)), A::sqrt_square_offset(A::offset(s.ad_value(234), 1.0), ((4.0 * p.p46) * p.p46))), 1.0);
            s.store_scaled_sub_ad_lhs(236, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(235), p.p45, s.ad_value(225)), (-p.p45)), 225, 0.5);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(228, 21, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(236), s.ad_value(21))), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
            s.store_scaled_offset(237, 235, 1.0, 0.5);
            s.store_powf_ad(238, A::offset(A::div_from_scalar(p.p45, s.ad_value(21)), 1.0), (-p.p43));
            s.store_powf_ad(239, A::offset(A::div(s.ad_value(225), s.ad_value(21)), 1.0), (-p.p43));
            s.store_add_scaled_product_value_ad(240, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(237), s.ad_value(238)), 1.0, 237, 239, 1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(241, 240, s.ad_value(144), 1.0, s.ad_value(236), (-1.0), s.ad_value(232), 1.0, 0.0);
            s.store_add_scaled_inputs3_indices(116, 241, 1.0, 228, 1.0, 233, -1.0);
        }

        if ((!s.b[246]) && (!s.b[249])) {
            s.store_sqrt_square_offset(242, 225, ((4.0 * p.p44) * p.p44));
            s.store_scaled_add(232, 225, 242, (-0.5));
            s.store_mul_scaled_powf_ad_rhs(243, 21, (-1.0 / ((1.0 - p.p43))), A::sub_from_scalar(1.0, A::div(s.ad_value(232), s.ad_value(21))), (1.0 - p.p43));
        }

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[246]) && (!s.b[249])) {
            s.store_add(244, 144, 225);
            s.store_sqrt_square_offset(245, 244, ((4.0 * p.p44) * p.p44));
            s.store_add_scaled_inputs3_indices(236, 244, 0.5, 245, (-0.5), 225, -1.0);
            s.store_mul_scaled_powf_ad_rhs(228, 21, (-1.0 / ((1.0 - p.p43))), A::sub_from_scalar(1.0, A::div(s.ad_value(236), s.ad_value(21))), (1.0 - p.p43));
            s.store_sub_ad_lhs(116, A::add_scaled_inputs4(s.ad_value(228), 1.0, s.ad_value(144), (((1.0 - p.p34)) as f64).powf((-p.p43)), s.ad_value(236), ((-1.0) * (((1.0 - p.p34)) as f64).powf((-p.p43))), s.ad_value(232), (((1.0 - p.p34)) as f64).powf((-p.p43))), 243);
        }

        s.store_div_from_scalar_mul_ad(112, 1.0, s.ad_value(27), s.ad_value(73));

        s.b[250] = (s.v[143] < s.v[61]);
        s.v[250] = if s.b[250] { 1.0 } else { 0.0 };

        if s.b[250] {
            s.store_exp_mul(109, 143, 112);
        }

        if (!s.b[250]) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(61), s.ad_value(112))), A::mul(A::sub(s.ad_value(143), s.ad_value(61)), s.ad_value(112)), 1.0);
        }

        s.store_mul_offset_rhs(74, 0, 109, (-1.0));

        s.store_div_from_scalar_mul_ad(112, 1.0, s.ad_value(28), s.ad_value(73));

        s.b[251] = (s.v[144] < s.v[62]);
        s.v[251] = if s.b[251] { 1.0 } else { 0.0 };

        if s.b[251] {
            s.store_exp_mul(109, 144, 112);
        }

        if (!s.b[251]) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(62), s.ad_value(112))), A::mul(A::sub(s.ad_value(144), s.ad_value(62)), s.ad_value(112)), 1.0);
        }

        s.store_mul_ad_product_rhs_mixed_ia(75, 0, 1, A::offset(s.ad_value(109), (-1.0)));

        s.store_offset_add_scaled_product(78, A::offset(A::mul(s.ad_value(114), s.ad_value(44)), 1.0), 1.0, s.ad_value(116), s.ad_value(43), 1.0, (-0.0001));

        s.store_offset_add_scaled_inputs_mixed_ai(79, A::sqrt_square_offset(s.ad_value(78), 1e-8), 0.5, 78, 0.5, 0.0001);

        s.store_add_scaled_products_indices(80, 74, 45, 1.0, 75, 46, 1.0);

        s.b[252] = (p.p30 < 0.5);
        s.v[252] = if s.b[252] { 1.0 } else { 0.0 };

        if s.b[252] {
            s.store_add_scaled_ad_lhs(108, A::powf(s.ad_value(79), (1.0 / p.p73)), 80, 4.0);
        }

        s.b[253] = (s.v[108] > 1e-8);
        s.v[253] = if s.b[253] { 1.0 } else { 0.0 };

        if (s.b[252] && s.b[253]) {
            s.store_scaled_add_ad_rhs(81, 79, A::powf(s.ad_value(108), p.p73), 0.5);
        }

        if (s.b[252] && (!s.b[253])) {
            s.store_scaled_offset(81, 79, ((1e-8) as f64).powf(p.p73), 0.5);
        }

        if (!s.b[252]) {
            s.store_offset_scaled(108, 80, 4.0, 1.0);
        }

        s.b[254] = (s.v[108] > 1e-8);
        s.v[254] = if s.b[254] { 1.0 } else { 0.0 };

        if ((!s.b[252]) && s.b[254]) {
            s.store_mul_scaled_offset_ad_rhs(81, 79, 0.5, A::powf(s.ad_value(108), p.p73), 1.0);
        }

        if ((!s.b[252]) && (!s.b[254])) {
            s.store_scale(81, 79, (0.5 * (1.0 + ((1e-8) as f64).powf(p.p73))));
        }

        s.b[255] = (p.p31 > 0.0);
        s.v[255] = if s.b[255] { 1.0 } else { 0.0 };

        if s.b[255] {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p33);
        }

        s.b[256] = (s.v[146] < s.v[63]);
        s.v[256] = if s.b[256] { 1.0 } else { 0.0 };

        if (s.b[255] && s.b[256]) {
            s.store_exp_mul(109, 146, 112);
        }

        if (s.b[255] && (!s.b[256])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(63), s.ad_value(112))), A::mul(A::sub(s.ad_value(146), s.ad_value(63)), s.ad_value(112)), 1.0);
        }

        s.b[257] = (s.v[144] < s.v[63]);
        s.v[257] = if s.b[257] { 1.0 } else { 0.0 };

        if (s.b[255] && s.b[257]) {
            s.store_exp_mul(111, 144, 112);
        }

        if (s.b[255] && (!s.b[257])) {
            s.store_mul_offset_rhs_ad(111, A::exp(A::mul(s.ad_value(63), s.ad_value(112))), A::mul(A::sub(s.ad_value(144), s.ad_value(63)), s.ad_value(112)), 1.0);
        }

        if s.b[255] {
            s.store_mul_offset_ad_rhs(82, 5, A::add_scaled_inputs(s.ad_value(109), p.p32, s.ad_value(111), (1.0 - p.p32)), (-1.0));
            s.store_mul(85, 82, 47);
            s.store_offset_scaled(108, 85, 4.0, 1.0);
        }

        s.b[259] = (s.v[147] < s.v[63]);
        s.v[259] = if s.b[259] { 1.0 } else { 0.0 };

        if (s.b[255] && s.b[259]) {
            s.store_exp_mul(109, 147, 112);
        }

        if (s.b[255] && (!s.b[259])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(63), s.ad_value(112))), A::mul(A::sub(s.ad_value(147), s.ad_value(63)), s.ad_value(112)), 1.0);
        }

        if (!s.b[255]) {
            s.store_scalar(82, 0.0);
        }

        s.b[260] = (p.p55 == 1.0);
        s.v[260] = if s.b[260] { 1.0 } else { 0.0 };

        if s.b[260] {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p56);
        }

        s.b[261] = (s.v[143] < s.v[65]);
        s.v[261] = if s.b[261] { 1.0 } else { 0.0 };

        if (s.b[260] && s.b[261]) {
            s.store_exp_mul(109, 143, 112);
        }

        if (s.b[260] && (!s.b[261])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(65), s.ad_value(112))), A::mul(A::sub(s.ad_value(143), s.ad_value(65)), s.ad_value(112)), 1.0);
        }

        if s.b[260] {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p59);
        }

        s.b[264] = (p.p88 > 0.0);
        s.v[264] = if s.b[264] { 1.0 } else { 0.0 };

        if (s.b[260] && s.b[264]) {
            s.store_sub_scaled_inputs(150, 31, -1.0, 143, 1.0);
            s.store_div_from_scalar_mul_ad(112, 1.0, s.ad_value(32), s.ad_value(73));
        }

        s.b[265] = (s.v[150] < s.v[64]);
        s.v[265] = if s.b[265] { 1.0 } else { 0.0 };

        if ((s.b[260] && s.b[264]) && s.b[265]) {
            s.store_exp_mul(111, 150, 112);
        }

        if ((s.b[260] && s.b[264]) && (!s.b[265])) {
            s.store_mul_offset_rhs_ad(111, A::exp(A::mul(s.ad_value(64), s.ad_value(112))), A::mul(A::sub(s.ad_value(150), s.ad_value(64)), s.ad_value(112)), 1.0);
        }

        s.b[266] = (p.p55 == 0.0);
        s.v[266] = if s.b[266] { 1.0 } else { 0.0 };

        if ((!s.b[260]) && s.b[266]) {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p56);
        }

        s.b[267] = (s.v[145] < s.v[65]);
        s.v[267] = if s.b[267] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && s.b[266]) && s.b[267]) {
            s.store_exp_mul(109, 145, 112);
        }

        if (((!s.b[260]) && s.b[266]) && (!s.b[267])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(65), s.ad_value(112))), A::mul(A::sub(s.ad_value(145), s.ad_value(65)), s.ad_value(112)), 1.0);
        }

        if ((!s.b[260]) && s.b[266]) {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p59);
        }

        s.b[269] = (p.p88 > 0.0);
        s.v[269] = if s.b[269] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && s.b[266]) && s.b[269]) {
            s.store_sub_scaled_inputs(150, 31, -1.0, 143, 1.0);
            s.store_div_from_scalar_mul_ad(112, 1.0, s.ad_value(32), s.ad_value(73));
        }

        s.b[270] = (s.v[150] < s.v[64]);
        s.v[270] = if s.b[270] { 1.0 } else { 0.0 };

        if ((((!s.b[260]) && s.b[266]) && s.b[269]) && s.b[270]) {
            s.store_exp_mul(111, 150, 112);
        }

        if ((((!s.b[260]) && s.b[266]) && s.b[269]) && (!s.b[270])) {
            s.store_mul_offset_rhs_ad(111, A::exp(A::mul(s.ad_value(64), s.ad_value(112))), A::mul(A::sub(s.ad_value(150), s.ad_value(64)), s.ad_value(112)), 1.0);
        }

        if ((!s.b[260]) && (!s.b[266])) {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p56);
        }

        s.b[271] = (s.v[143] < s.v[65]);
        s.v[271] = if s.b[271] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && (!s.b[266])) && s.b[271]) {
            s.store_exp_mul(109, 143, 112);
        }

        if (((!s.b[260]) && (!s.b[266])) && (!s.b[271])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(65), s.ad_value(112))), A::mul(A::sub(s.ad_value(143), s.ad_value(65)), s.ad_value(112)), 1.0);
        }

        if ((!s.b[260]) && (!s.b[266])) {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p59);
        }

        s.b[274] = (p.p88 > 0.0);
        s.v[274] = if s.b[274] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && (!s.b[266])) && s.b[274]) {
            s.store_sub_scaled_inputs(150, 31, -1.0, 143, 1.0);
            s.store_div_from_scalar_mul_ad(112, 1.0, s.ad_value(32), s.ad_value(73));
        }

        s.b[275] = (s.v[150] < s.v[64]);
        s.v[275] = if s.b[275] { 1.0 } else { 0.0 };

        if ((((!s.b[260]) && (!s.b[266])) && s.b[274]) && s.b[275]) {
            s.store_exp_mul(111, 150, 112);
        }

        if ((((!s.b[260]) && (!s.b[266])) && s.b[274]) && (!s.b[275])) {
            s.store_mul_offset_rhs_ad(111, A::exp(A::mul(s.ad_value(64), s.ad_value(112))), A::mul(A::sub(s.ad_value(150), s.ad_value(64)), s.ad_value(112)), 1.0);
        }

        if ((!s.b[260]) && (!s.b[266])) {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p56);
        }

        s.b[276] = (s.v[145] < s.v[65]);
        s.v[276] = if s.b[276] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && (!s.b[266])) && s.b[276]) {
            s.store_exp_mul(109, 145, 112);
        }

        if (((!s.b[260]) && (!s.b[266])) && (!s.b[276])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(65), s.ad_value(112))), A::mul(A::sub(s.ad_value(145), s.ad_value(65)), s.ad_value(112)), 1.0);
        }

        if ((!s.b[260]) && (!s.b[266])) {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p59);
        }

        s.b[278] = (p.p88 > 0.0);
        s.v[278] = if s.b[278] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && (!s.b[266])) && s.b[278]) {
            s.store_sub_scaled_inputs(150, 31, -1.0, 143, 1.0);
            s.store_div_from_scalar_mul_ad(112, 1.0, s.ad_value(32), s.ad_value(73));
        }

        s.b[279] = (s.v[150] < s.v[64]);
        s.v[279] = if s.b[279] { 1.0 } else { 0.0 };

        if ((((!s.b[260]) && (!s.b[266])) && s.b[278]) && s.b[279]) {
            s.store_exp_mul(111, 150, 112);
        }

        if ((((!s.b[260]) && (!s.b[266])) && s.b[278]) && (!s.b[279])) {
            s.store_mul_offset_rhs_ad(111, A::exp(A::mul(s.ad_value(64), s.ad_value(112))), A::mul(A::sub(s.ad_value(150), s.ad_value(64)), s.ad_value(112)), 1.0);
        }

        s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p61);

        s.b[280] = (s.v[144] < s.v[67]);
        s.v[280] = if s.b[280] { 1.0 } else { 0.0 };

        if s.b[280] {
            s.store_exp_mul(109, 144, 112);
        }

        if (!s.b[280]) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(67), s.ad_value(112))), A::mul(A::sub(s.ad_value(144), s.ad_value(67)), s.ad_value(112)), 1.0);
        }

        s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p63);

        s.b[282] = ((p.p64 > 0.0) || (p.p65 > 0.0));
        s.v[282] = if s.b[282] { 1.0 } else { 0.0 };

        if s.b[282] {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p61);
        }

        s.b[283] = (s.v[146] < s.v[69]);
        s.v[283] = if s.b[283] { 1.0 } else { 0.0 };

        if (s.b[282] && s.b[283]) {
            s.store_exp_mul(109, 146, 112);
        }

        if (s.b[282] && (!s.b[283])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(69), s.ad_value(112))), A::mul(A::sub(s.ad_value(146), s.ad_value(69)), s.ad_value(112)), 1.0);
        }

        if s.b[282] {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p63);
        }

        s.store_div(108, 144, 73);

        s.b[285] = (s.v[108] < s.v[113]);
        s.v[285] = if s.b[285] { 1.0 } else { 0.0 };

        if s.b[285] {
            s.store_exp(109, 108);
        }

        if (!s.b[285]) {
            s.store_mul_offset_rhs_ad(109, A::exp(s.ad_value(113)), A::sub(s.ad_value(108), s.ad_value(113)), 1.0);
        }

        s.store_div(108, 148, 73);

        s.b[286] = (s.v[108] < s.v[113]);
        s.v[286] = if s.b[286] { 1.0 } else { 0.0 };

        if s.b[286] {
            s.store_exp(111, 108);
        }

        if (!s.b[286]) {
            s.store_mul_offset_rhs_ad(111, A::exp(s.ad_value(113)), A::sub(s.ad_value(108), s.ad_value(113)), 1.0);
        }

        s.store_sqrt_offset_ad(103, A::mul(s.ad_value(33), s.ad_value(109)), 1.0);

        s.store_sqrt_offset_ad(104, A::mul(s.ad_value(33), s.ad_value(111)), 1.0);

        s.b[303] = ((p.p66 > 0.0) || (p.p68 > 0.0));
        s.v[303] = if s.b[303] { 1.0 } else { 0.0 };

        if s.b[303] {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p67);
        }

        s.b[304] = (s.v[147] < s.v[71]);
        s.v[304] = if s.b[304] { 1.0 } else { 0.0 };

        if (s.b[303] && s.b[304]) {
            s.store_exp_mul(109, 147, 112);
        }

        if (s.b[303] && (!s.b[304])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(71), s.ad_value(112))), A::mul(A::sub(s.ad_value(147), s.ad_value(71)), s.ad_value(112)), 1.0);
        }

        if s.b[303] {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p69);
        }

        s.b[306] = (p.p49 > 0.0);
        s.v[306] = if s.b[306] { 1.0 } else { 0.0 };

        if s.b[306] {
            s.store_scale(307, 22, (-p.p34));
        }

        s.b[318] = (p.p52 <= 0.0);
        s.v[318] = if s.b[318] { 1.0 } else { 0.0 };

        if (s.b[306] && s.b[318]) {
            s.store_add(308, 147, 307);
        }

        s.b[319] = (s.v[308] > 0.0);
        s.v[319] = if s.b[319] { 1.0 } else { 0.0 };

        if ((s.b[306] && s.b[318]) && s.b[319]) {
            s.store_scalar(309, (((1.0 - p.p34)) as f64).powf((-p.p51)));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(310, 22, 1.0, 309, (1.0 - p.p34), 1.0 / ((1.0 - p.p51)));
            s.store_mul_ad_product_lhs_mixed_ia(311, 308, A::offset(A::div_scaled_inputs(s.ad_value(308), (0.5 * p.p51), s.ad_value(22), (1.0 - p.p34)), 1.0), 309);
        }

        if ((s.b[306] && s.b[318]) && (!s.b[319])) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(310, 22, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(147), s.ad_value(22))), (1.0 - p.p51)), 1.0 / ((1.0 - p.p51)));
            s.store_scalar(311, 0.0);
        }

        if (s.b[306] && s.b[318]) {
            s.store_add(118, 310, 311);
        }

        if (s.b[306] && (!s.b[318])) {
            s.store_sqrt_square_offset(312, 307, ((4.0 * p.p52) * p.p52));
            s.store_scaled_add(313, 307, 312, (-0.5));
            s.store_mul_scaled_powf_ad_rhs(314, 22, (-1.0 / ((1.0 - p.p51))), A::sub_from_scalar(1.0, A::div(s.ad_value(313), s.ad_value(22))), (1.0 - p.p51));
            s.store_add(315, 147, 307);
            s.store_sqrt_square_offset(316, 315, ((4.0 * p.p52) * p.p52));
            s.store_add_scaled_inputs3_indices(317, 315, 0.5, 316, (-0.5), 307, -1.0);
            s.store_mul_scaled_powf_ad_rhs(310, 22, (-1.0 / ((1.0 - p.p51))), A::sub_from_scalar(1.0, A::div(s.ad_value(317), s.ad_value(22))), (1.0 - p.p51));
            s.store_sub_ad_lhs(118, A::add_scaled_offset_product_rhs(s.ad_value(310), 1.0, A::add_scaled_inputs3(s.ad_value(147), 1.0, s.ad_value(317), (-1.0), s.ad_value(313), 1.0), A::div_scaled_inputs3(s.ad_value(147), (0.5 * p.p51), s.ad_value(317), ((-1.0) * (0.5 * p.p51)), s.ad_value(313), (0.5 * p.p51), s.ad_value(22), (1.0 - p.p34)), 1.0, (((1.0 - p.p34)) as f64).powf((-p.p51))), 314);
        }

        if (!s.b[306]) {
            s.store_scalar(118, 0.0);
        }

        s.store_scale(320, 20, (-p.p34));

        s.b[331] = (p.p39 <= 0.0);
        s.v[331] = if s.b[331] { 1.0 } else { 0.0 };

        if s.b[331] {
            s.store_add(321, 145, 320);
        }

        s.b[332] = (s.v[321] > 0.0);
        s.v[332] = if s.b[332] { 1.0 } else { 0.0 };

        if (s.b[331] && s.b[332]) {
            s.store_scalar(322, (((1.0 - p.p34)) as f64).powf((-p.p38)));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(323, 20, 1.0, 322, (1.0 - p.p34), 1.0 / ((1.0 - p.p38)));
            s.store_mul_ad_product_lhs_mixed_ia(324, 321, A::offset(A::div_scaled_inputs(s.ad_value(321), (0.5 * p.p38), s.ad_value(20), (1.0 - p.p34)), 1.0), 322);
        }

        if (s.b[331] && (!s.b[332])) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(323, 20, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(145), s.ad_value(20))), (1.0 - p.p38)), 1.0 / ((1.0 - p.p38)));
        }

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[331] && (!s.b[332])) {
            s.store_scalar(324, 0.0);
        }

        if s.b[331] {
            s.store_add(115, 323, 324);
        }

        if (!s.b[331]) {
            s.store_sqrt_square_offset(325, 320, ((4.0 * p.p39) * p.p39));
            s.store_scaled_add(326, 320, 325, (-0.5));
            s.store_mul_scaled_powf_ad_rhs(327, 20, (-1.0 / ((1.0 - p.p38))), A::sub_from_scalar(1.0, A::div(s.ad_value(326), s.ad_value(20))), (1.0 - p.p38));
            s.store_add(328, 145, 320);
            s.store_sqrt_square_offset(329, 328, ((4.0 * p.p39) * p.p39));
            s.store_add_scaled_inputs3_indices(330, 328, 0.5, 329, (-0.5), 320, -1.0);
            s.store_mul_scaled_powf_ad_rhs(323, 20, (-1.0 / ((1.0 - p.p38))), A::sub_from_scalar(1.0, A::div(s.ad_value(330), s.ad_value(20))), (1.0 - p.p38));
            s.store_sub_ad_lhs(115, A::add_scaled_offset_product_rhs(s.ad_value(323), 1.0, A::add_scaled_inputs3(s.ad_value(145), 1.0, s.ad_value(330), (-1.0), s.ad_value(326), 1.0), A::div_scaled_inputs3(s.ad_value(145), (0.5 * p.p38), s.ad_value(330), ((-1.0) * (0.5 * p.p38)), s.ad_value(326), (0.5 * p.p38), s.ad_value(20), (1.0 - p.p34)), 1.0, (((1.0 - p.p34)) as f64).powf((-p.p38))), 327);
        }

        s.store_scale(333, 21, (-p.p34));

        s.b[354] = (p.p44 <= 0.0);
        s.v[354] = if s.b[354] { 1.0 } else { 0.0 };

        if s.b[354] {
            s.store_add(334, 146, 333);
        }

        s.b[355] = (s.v[334] > 0.0);
        s.v[355] = if s.b[355] { 1.0 } else { 0.0 };

        if (s.b[354] && s.b[355]) {
            s.store_scalar(335, (((1.0 - p.p34)) as f64).powf(((-1.0) - p.p43)));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(336, 21, 1.0, 335, ((1.0 - p.p34) * (1.0 - p.p34)), 1.0 / ((1.0 - p.p43)));
            s.store_mul_ad_product_lhs_mixed_ia(337, 334, A::offset(A::div_scaled_inputs(s.ad_value(334), (0.5 * p.p43), s.ad_value(21), 1.0), (1.0 - p.p34)), 335);
        }

        s.b[356] = ((p.p45 > 0.0) && (s.v[146] < (-p.p45)));
        s.v[356] = if s.b[356] { 1.0 } else { 0.0 };

        if ((s.b[354] && (!s.b[355])) && s.b[356]) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(336, 21, 1.0, A::mul_sub_from_scalar_rhs(A::powf(A::offset(A::div_from_scalar(p.p45, s.ad_value(21)), 1.0), (1.0 - p.p43)), 1.0, A::div_scaled_offset_numerator(s.ad_value(146), (1.0 - p.p43), (p.p45 * (1.0 - p.p43)), A::offset(s.ad_value(21), p.p45), 1.0)), 1.0 / ((1.0 - p.p43)));
        }

        if ((s.b[354] && (!s.b[355])) && (!s.b[356])) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(336, 21, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(146), s.ad_value(21))), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
        }

        if (s.b[354] && (!s.b[355])) {
            s.store_scalar(337, 0.0);
        }

        if s.b[354] {
            s.store_add(117, 336, 337);
        }

        s.b[357] = ((p.p45 > 0.0) && (p.p46 > 0.0));
        s.v[357] = if s.b[357] { 1.0 } else { 0.0 };

        if ((!s.b[354]) && s.b[357]) {
            s.store_div_scaled_offset_numerator(338, s.ad_value(333), 1.0, p.p45, A::sub_from_scalar(p.p45, s.ad_value(333)), 1.0);
            s.store_div_scaled_inputs_mixed_ia(339, 338, 2.0, A::add(A::sqrt_square_offset(A::offset(s.ad_value(338), (-1.0)), ((4.0 * p.p44) * p.p44)), A::sqrt_square_offset(A::offset(s.ad_value(338), 1.0), ((4.0 * p.p46) * p.p46))), 1.0);
            s.store_scaled_sub_ad_lhs(340, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(339), p.p45, s.ad_value(333)), (-p.p45)), 333, 0.5);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(341, 21, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(340), s.ad_value(21))), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
            s.store_div_scaled_inputs2_mixed_aia(342, A::scale_offset(s.ad_value(146), 2.0, p.p45), 1.0, 333, 1.0, A::sub_from_scalar(p.p45, s.ad_value(333)), 1.0);
            s.store_div_scaled_inputs_mixed_ia(343, 342, 2.0, A::add(A::sqrt_square_offset(A::offset(s.ad_value(342), (-1.0)), ((4.0 * p.p44) * p.p44)), A::sqrt_square_offset(A::offset(s.ad_value(342), 1.0), ((4.0 * p.p46) * p.p46))), 1.0);
            s.store_scaled_sub_ad_lhs(344, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(343), p.p45, s.ad_value(333)), (-p.p45)), 333, 0.5);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(336, 21, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(344), s.ad_value(21))), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
            s.store_scaled_offset(345, 343, 1.0, 0.5);
            s.store_powf_ad(346, A::offset(A::div_from_scalar(p.p45, s.ad_value(21)), 1.0), (-p.p43));
            s.store_powf_ad(347, A::offset(A::div(s.ad_value(333), s.ad_value(21)), 1.0), (-p.p43));
            s.store_add_scaled_product_value_ad(348, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(345), s.ad_value(346)), 1.0, 345, 347, 1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(349, 348, s.ad_value(146), 1.0, s.ad_value(344), (-1.0), s.ad_value(340), 1.0, 0.0);
            s.store_add_scaled_inputs3_indices(117, 349, 1.0, 336, 1.0, 341, -1.0);
        }

        if ((!s.b[354]) && (!s.b[357])) {
            s.store_sqrt_square_offset(350, 333, ((4.0 * p.p44) * p.p44));
            s.store_scaled_add(340, 333, 350, (-0.5));
            s.store_mul_scaled_powf_ad_rhs(351, 21, (-1.0 / ((1.0 - p.p43))), A::sub_from_scalar(1.0, A::div(s.ad_value(340), s.ad_value(21))), (1.0 - p.p43));
            s.store_add(352, 146, 333);
            s.store_sqrt_square_offset(353, 352, ((4.0 * p.p44) * p.p44));
            s.store_add_scaled_inputs3_indices(344, 352, 0.5, 353, (-0.5), 333, -1.0);
            s.store_mul_scaled_powf_ad_rhs(336, 21, (-1.0 / ((1.0 - p.p43))), A::sub_from_scalar(1.0, A::div(s.ad_value(344), s.ad_value(21))), (1.0 - p.p43));
            s.store_sub_ad_lhs(117, A::add_scaled_inputs4(s.ad_value(336), 1.0, s.ad_value(146), (((1.0 - p.p34)) as f64).powf((-p.p43)), s.ad_value(344), ((-1.0) * (((1.0 - p.p34)) as f64).powf((-p.p43))), s.ad_value(340), (((1.0 - p.p34)) as f64).powf((-p.p43))), 351);
        }

        s.b[119] = (s.v[74] > 0.0);
        s.v[119] = if s.b[119] { 1.0 } else { 0.0 };

        s.store_scaled_mul(120, 74, 51, s.v[119]);

        s.store_div_scaled_value_offset_denominator(121, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);

        s.store_scaled_mul(108, 144, 50, 0.6944444444444444);

        s.b[358] = (s.v[108] < s.v[113]);
        s.v[358] = if s.b[358] { 1.0 } else { 0.0 };

        if s.b[358] {
            s.store_exp(109, 108);
        }

        if (!s.b[358]) {
            s.store_mul_offset_rhs_ad(109, A::exp(s.ad_value(113)), A::sub(s.ad_value(108), s.ad_value(113)), 1.0);
        }

        s.store_scaled_mul_scale_offset_rhs_ad(122, A::offset(A::mul_scaled_output(s.ad_value(109), A::add(s.ad_value(52), A::square(s.ad_value(121))), (p.p78 * s.v[119])), 1.0), 79, p.p77, 1.0, p.p76);

        s.store_add_scaled_product_value_ad(123, A::div_scaled_product(s.ad_value(122), s.ad_value(74), 1.0, s.ad_value(81), 1.0), 1.0, 23, 114, p.p55);

        s.store_scaled_mul(124, 23, 115, (1.0 - p.p55));

        s.store_add_scaled_ad_lhs(125, A::add_scaled_product(s.ad_value(75), p.p81, s.ad_value(24), s.ad_value(116), 1.0), 103, p.p47);

        s.store_scale(126, 104, p.p47);

        s.store_add_scaled_product_indices(127, 82, p.p81, 25, 117, 1.0);

        s.store_add_scaled_product_indices(128, 147, p.p53, 26, 118, 1.0);

        s.store_scale(142, 138, p.p102);

        s.store_scaled_mul(123, 162, 123, 1.0);

        s.store_scaled_mul(124, 162, 124, 1.0);

        s.store_scaled_mul(125, 162, 125, 1.0);

        s.store_scaled_mul(126, 162, 126, 1.0);

        s.store_scaled_mul(127, 162, 127, 1.0);

        s.store_scaled_mul(128, 162, 128, 1.0);

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
        let eq0_value: f64 = s.v[87];
        let eq0_node_derivatives: [f64; 14] = [s.dn[87][0], s.dn[87][1], s.dn[87][2], s.dn[87][3], s.dn[87][4], s.dn[87][5], s.dn[87][6], s.dn[87][7], s.dn[87][8], s.dn[87][9], s.dn[87][10], s.dn[87][11], s.dn[87][12], s.dn[87][13]];
        let eq0_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let eq1_value: f64 = s.v[88];
        let eq1_node_derivatives: [f64; 14] = [s.dn[88][0], s.dn[88][1], s.dn[88][2], s.dn[88][3], s.dn[88][4], s.dn[88][5], s.dn[88][6], s.dn[88][7], s.dn[88][8], s.dn[88][9], s.dn[88][10], s.dn[88][11], s.dn[88][12], s.dn[88][13]];
        let eq1_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let eq2_value: f64 = s.v[137];
        let eq2_node_derivatives: [f64; 14] = [s.dn[137][0], s.dn[137][1], s.dn[137][2], s.dn[137][3], s.dn[137][4], s.dn[137][5], s.dn[137][6], s.dn[137][7], s.dn[137][8], s.dn[137][9], s.dn[137][10], s.dn[137][11], s.dn[137][12], s.dn[137][13]];
        let eq2_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(9),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let eq3_value: f64 = s.v[77];
        let eq3_node_derivatives: [f64; 14] = [s.dn[77][0], s.dn[77][1], s.dn[77][2], s.dn[77][3], s.dn[77][4], s.dn[77][5], s.dn[77][6], s.dn[77][7], s.dn[77][8], s.dn[77][9], s.dn[77][10], s.dn[77][11], s.dn[77][12], s.dn[77][13]];
        let eq3_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let eq4_value: f64 = s.v[90];
        let eq4_node_derivatives: [f64; 14] = [s.dn[90][0], s.dn[90][1], s.dn[90][2], s.dn[90][3], s.dn[90][4], s.dn[90][5], s.dn[90][6], s.dn[90][7], s.dn[90][8], s.dn[90][9], s.dn[90][10], s.dn[90][11], s.dn[90][12], s.dn[90][13]];
        let eq4_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let eq5_value: f64 = s.v[94];
        let eq5_node_derivatives: [f64; 14] = [s.dn[94][0], s.dn[94][1], s.dn[94][2], s.dn[94][3], s.dn[94][4], s.dn[94][5], s.dn[94][6], s.dn[94][7], s.dn[94][8], s.dn[94][9], s.dn[94][10], s.dn[94][11], s.dn[94][12], s.dn[94][13]];
        let eq5_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let eq6_value: f64 = s.v[91];
        let eq6_node_derivatives: [f64; 14] = [s.dn[91][0], s.dn[91][1], s.dn[91][2], s.dn[91][3], s.dn[91][4], s.dn[91][5], s.dn[91][6], s.dn[91][7], s.dn[91][8], s.dn[91][9], s.dn[91][10], s.dn[91][11], s.dn[91][12], s.dn[91][13]];
        let eq6_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(10),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let eq7_value: f64 = s.v[96];
        let eq7_node_derivatives: [f64; 14] = [s.dn[96][0], s.dn[96][1], s.dn[96][2], s.dn[96][3], s.dn[96][4], s.dn[96][5], s.dn[96][6], s.dn[96][7], s.dn[96][8], s.dn[96][9], s.dn[96][10], s.dn[96][11], s.dn[96][12], s.dn[96][13]];
        let eq7_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(5),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let eq8_value: f64 = s.v[97];
        let eq8_node_derivatives: [f64; 14] = [s.dn[97][0], s.dn[97][1], s.dn[97][2], s.dn[97][3], s.dn[97][4], s.dn[97][5], s.dn[97][6], s.dn[97][7], s.dn[97][8], s.dn[97][9], s.dn[97][10], s.dn[97][11], s.dn[97][12], s.dn[97][13]];
        let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let eq9_value: f64 = s.v[98];
        let eq9_node_derivatives: [f64; 14] = [s.dn[98][0], s.dn[98][1], s.dn[98][2], s.dn[98][3], s.dn[98][4], s.dn[98][5], s.dn[98][6], s.dn[98][7], s.dn[98][8], s.dn[98][9], s.dn[98][10], s.dn[98][11], s.dn[98][12], s.dn[98][13]];
        let eq9_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(7),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_value: f64 = s.v[99];
        let eq10_node_derivatives: [f64; 14] = [s.dn[99][0], s.dn[99][1], s.dn[99][2], s.dn[99][3], s.dn[99][4], s.dn[99][5], s.dn[99][6], s.dn[99][7], s.dn[99][8], s.dn[99][9], s.dn[99][10], s.dn[99][11], s.dn[99][12], s.dn[99][13]];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_value: f64 = s.v[100];
        let eq11_node_derivatives: [f64; 14] = [s.dn[100][0], s.dn[100][1], s.dn[100][2], s.dn[100][3], s.dn[100][4], s.dn[100][5], s.dn[100][6], s.dn[100][7], s.dn[100][8], s.dn[100][9], s.dn[100][10], s.dn[100][11], s.dn[100][12], s.dn[100][13]];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(9),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_value: f64 = s.v[101];
        let eq12_node_derivatives: [f64; 14] = [s.dn[101][0], s.dn[101][1], s.dn[101][2], s.dn[101][3], s.dn[101][4], s.dn[101][5], s.dn[101][6], s.dn[101][7], s.dn[101][8], s.dn[101][9], s.dn[101][10], s.dn[101][11], s.dn[101][12], s.dn[101][13]];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(5),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_value: f64 = s.v[92];
        let eq13_node_derivatives: [f64; 14] = [s.dn[92][0], s.dn[92][1], s.dn[92][2], s.dn[92][3], s.dn[92][4], s.dn[92][5], s.dn[92][6], s.dn[92][7], s.dn[92][8], s.dn[92][9], s.dn[92][10], s.dn[92][11], s.dn[92][12], s.dn[92][13]];
        let eq13_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(10),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_value: f64 = s.v[84];
        let eq14_node_derivatives: [f64; 14] = [s.dn[84][0], s.dn[84][1], s.dn[84][2], s.dn[84][3], s.dn[84][4], s.dn[84][5], s.dn[84][6], s.dn[84][7], s.dn[84][8], s.dn[84][9], s.dn[84][10], s.dn[84][11], s.dn[84][12], s.dn[84][13]];
        let eq14_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(11),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let eq15_value: f64 = s.v[102];
        let eq15_node_derivatives: [f64; 14] = [s.dn[102][0], s.dn[102][1], s.dn[102][2], s.dn[102][3], s.dn[102][4], s.dn[102][5], s.dn[102][6], s.dn[102][7], s.dn[102][8], s.dn[102][9], s.dn[102][10], s.dn[102][11], s.dn[102][12], s.dn[102][13]];
        let eq15_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(11),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let eq16_value: f64 = s.v[133];
        let eq16_node_derivatives: [f64; 14] = [s.dn[133][0], s.dn[133][1], s.dn[133][2], s.dn[133][3], s.dn[133][4], s.dn[133][5], s.dn[133][6], s.dn[133][7], s.dn[133][8], s.dn[133][9], s.dn[133][10], s.dn[133][11], s.dn[133][12], s.dn[133][13]];
        let eq16_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let eq18_value: f64 = s.v[141];
        let eq18_node_derivatives: [f64; 14] = [s.dn[141][0], s.dn[141][1], s.dn[141][2], s.dn[141][3], s.dn[141][4], s.dn[141][5], s.dn[141][6], s.dn[141][7], s.dn[141][8], s.dn[141][9], s.dn[141][10], s.dn[141][11], s.dn[141][12], s.dn[141][13]];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let eq19_value: f64 = s.v[139];
        let eq19_node_derivatives: [f64; 14] = [s.dn[139][0], s.dn[139][1], s.dn[139][2], s.dn[139][3], s.dn[139][4], s.dn[139][5], s.dn[139][6], s.dn[139][7], s.dn[139][8], s.dn[139][9], s.dn[139][10], s.dn[139][11], s.dn[139][12], s.dn[139][13]];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let eq20_e159: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, s.v[123]);
        let eq20_value: f64 = eq20_e159;
        let eq20_node_derivatives: [f64; 14] = [(s.dn[123][0] * ddt_scale), (s.dn[123][1] * ddt_scale), (s.dn[123][2] * ddt_scale), (s.dn[123][3] * ddt_scale), (s.dn[123][4] * ddt_scale), (s.dn[123][5] * ddt_scale), (s.dn[123][6] * ddt_scale), (s.dn[123][7] * ddt_scale), (s.dn[123][8] * ddt_scale), (s.dn[123][9] * ddt_scale), (s.dn[123][10] * ddt_scale), (s.dn[123][11] * ddt_scale), (s.dn[123][12] * ddt_scale), (s.dn[123][13] * ddt_scale)];
        let eq20_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let eq21_e161: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, s.v[124]);
        let eq21_value: f64 = eq21_e161;
        let eq21_node_derivatives: [f64; 14] = [(s.dn[124][0] * ddt_scale), (s.dn[124][1] * ddt_scale), (s.dn[124][2] * ddt_scale), (s.dn[124][3] * ddt_scale), (s.dn[124][4] * ddt_scale), (s.dn[124][5] * ddt_scale), (s.dn[124][6] * ddt_scale), (s.dn[124][7] * ddt_scale), (s.dn[124][8] * ddt_scale), (s.dn[124][9] * ddt_scale), (s.dn[124][10] * ddt_scale), (s.dn[124][11] * ddt_scale), (s.dn[124][12] * ddt_scale), (s.dn[124][13] * ddt_scale)];
        let eq21_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let eq22_e163: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, s.v[125]);
        let eq22_value: f64 = eq22_e163;
        let eq22_node_derivatives: [f64; 14] = [(s.dn[125][0] * ddt_scale), (s.dn[125][1] * ddt_scale), (s.dn[125][2] * ddt_scale), (s.dn[125][3] * ddt_scale), (s.dn[125][4] * ddt_scale), (s.dn[125][5] * ddt_scale), (s.dn[125][6] * ddt_scale), (s.dn[125][7] * ddt_scale), (s.dn[125][8] * ddt_scale), (s.dn[125][9] * ddt_scale), (s.dn[125][10] * ddt_scale), (s.dn[125][11] * ddt_scale), (s.dn[125][12] * ddt_scale), (s.dn[125][13] * ddt_scale)];
        let eq22_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq22_value),
            &eq22_node_derivatives,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let eq23_e165: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, s.v[126]);
        let eq23_value: f64 = eq23_e165;
        let eq23_node_derivatives: [f64; 14] = [(s.dn[126][0] * ddt_scale), (s.dn[126][1] * ddt_scale), (s.dn[126][2] * ddt_scale), (s.dn[126][3] * ddt_scale), (s.dn[126][4] * ddt_scale), (s.dn[126][5] * ddt_scale), (s.dn[126][6] * ddt_scale), (s.dn[126][7] * ddt_scale), (s.dn[126][8] * ddt_scale), (s.dn[126][9] * ddt_scale), (s.dn[126][10] * ddt_scale), (s.dn[126][11] * ddt_scale), (s.dn[126][12] * ddt_scale), (s.dn[126][13] * ddt_scale)];
        let eq23_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let eq24_e167: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, s.v[127]);
        let eq24_value: f64 = eq24_e167;
        let eq24_node_derivatives: [f64; 14] = [(s.dn[127][0] * ddt_scale), (s.dn[127][1] * ddt_scale), (s.dn[127][2] * ddt_scale), (s.dn[127][3] * ddt_scale), (s.dn[127][4] * ddt_scale), (s.dn[127][5] * ddt_scale), (s.dn[127][6] * ddt_scale), (s.dn[127][7] * ddt_scale), (s.dn[127][8] * ddt_scale), (s.dn[127][9] * ddt_scale), (s.dn[127][10] * ddt_scale), (s.dn[127][11] * ddt_scale), (s.dn[127][12] * ddt_scale), (s.dn[127][13] * ddt_scale)];
        let eq24_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(10),
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let eq27_e173: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, s.v[128]);
        let eq27_value: f64 = eq27_e173;
        let eq27_node_derivatives: [f64; 14] = [(s.dn[128][0] * ddt_scale), (s.dn[128][1] * ddt_scale), (s.dn[128][2] * ddt_scale), (s.dn[128][3] * ddt_scale), (s.dn[128][4] * ddt_scale), (s.dn[128][5] * ddt_scale), (s.dn[128][6] * ddt_scale), (s.dn[128][7] * ddt_scale), (s.dn[128][8] * ddt_scale), (s.dn[128][9] * ddt_scale), (s.dn[128][10] * ddt_scale), (s.dn[128][11] * ddt_scale), (s.dn[128][12] * ddt_scale), (s.dn[128][13] * ddt_scale)];
        let eq27_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(10),
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let eq30_e179: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, s.v[142]);
        let eq30_value: f64 = eq30_e179;
        let eq30_node_derivatives: [f64; 14] = [(s.dn[142][0] * ddt_scale), (s.dn[142][1] * ddt_scale), (s.dn[142][2] * ddt_scale), (s.dn[142][3] * ddt_scale), (s.dn[142][4] * ddt_scale), (s.dn[142][5] * ddt_scale), (s.dn[142][6] * ddt_scale), (s.dn[142][7] * ddt_scale), (s.dn[142][8] * ddt_scale), (s.dn[142][9] * ddt_scale), (s.dn[142][10] * ddt_scale), (s.dn[142][11] * ddt_scale), (s.dn[142][12] * ddt_scale), (s.dn[142][13] * ddt_scale)];
        let eq30_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
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
        let eq20_e159_q: f64 = s.v[123];
        let eq20_reactive_node_derivatives: [f64; 14] = [s.dn[123][0], s.dn[123][1], s.dn[123][2], s.dn[123][3], s.dn[123][4], s.dn[123][5], s.dn[123][6], s.dn[123][7], s.dn[123][8], s.dn[123][9], s.dn[123][10], s.dn[123][11], s.dn[123][12], s.dn[123][13]];
        let eq20_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[9]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );
        let eq21_e161_q: f64 = s.v[124];
        let eq21_reactive_node_derivatives: [f64; 14] = [s.dn[124][0], s.dn[124][1], s.dn[124][2], s.dn[124][3], s.dn[124][4], s.dn[124][5], s.dn[124][6], s.dn[124][7], s.dn[124][8], s.dn[124][9], s.dn[124][10], s.dn[124][11], s.dn[124][12], s.dn[124][13]];
        let eq21_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq21_reactive_node_derivatives,
            branches,
            &eq21_reactive_branch_derivatives,
            multiplicity,
        );
        let eq22_e163_q: f64 = s.v[125];
        let eq22_reactive_node_derivatives: [f64; 14] = [s.dn[125][0], s.dn[125][1], s.dn[125][2], s.dn[125][3], s.dn[125][4], s.dn[125][5], s.dn[125][6], s.dn[125][7], s.dn[125][8], s.dn[125][9], s.dn[125][10], s.dn[125][11], s.dn[125][12], s.dn[125][13]];
        let eq22_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq22_reactive_node_derivatives,
            branches,
            &eq22_reactive_branch_derivatives,
            multiplicity,
        );
        let eq23_e165_q: f64 = s.v[126];
        let eq23_reactive_node_derivatives: [f64; 14] = [s.dn[126][0], s.dn[126][1], s.dn[126][2], s.dn[126][3], s.dn[126][4], s.dn[126][5], s.dn[126][6], s.dn[126][7], s.dn[126][8], s.dn[126][9], s.dn[126][10], s.dn[126][11], s.dn[126][12], s.dn[126][13]];
        let eq23_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
        let eq24_e167_q: f64 = s.v[127];
        let eq24_reactive_node_derivatives: [f64; 14] = [s.dn[127][0], s.dn[127][1], s.dn[127][2], s.dn[127][3], s.dn[127][4], s.dn[127][5], s.dn[127][6], s.dn[127][7], s.dn[127][8], s.dn[127][9], s.dn[127][10], s.dn[127][11], s.dn[127][12], s.dn[127][13]];
        let eq24_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            nodes,
            &eq24_reactive_node_derivatives,
            branches,
            &eq24_reactive_branch_derivatives,
            multiplicity,
        );
        let eq27_e173_q: f64 = s.v[128];
        let eq27_reactive_node_derivatives: [f64; 14] = [s.dn[128][0], s.dn[128][1], s.dn[128][2], s.dn[128][3], s.dn[128][4], s.dn[128][5], s.dn[128][6], s.dn[128][7], s.dn[128][8], s.dn[128][9], s.dn[128][10], s.dn[128][11], s.dn[128][12], s.dn[128][13]];
        let eq27_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[10]),
            nodes,
            &eq27_reactive_node_derivatives,
            branches,
            &eq27_reactive_branch_derivatives,
            multiplicity,
        );
        let eq30_e179_q: f64 = s.v[142];
        let eq30_reactive_node_derivatives: [f64; 14] = [s.dn[142][0], s.dn[142][1], s.dn[142][2], s.dn[142][3], s.dn[142][4], s.dn[142][5], s.dn[142][6], s.dn[142][7], s.dn[142][8], s.dn[142][9], s.dn[142][10], s.dn[142][11], s.dn[142][12], s.dn[142][13]];
        let eq30_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq30_reactive_node_derivatives,
            branches,
            &eq30_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
