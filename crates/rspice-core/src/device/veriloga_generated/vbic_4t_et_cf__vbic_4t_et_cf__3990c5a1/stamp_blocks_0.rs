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
        s.v[35] = (273.15 + p.p0);

        s.store_offset_voltage(34, ctx, nodes, Some(4), None, (ctx_temp + p.p105));

        s.store_scale(48, 34, (1.3806503e-23 * 6.241509744511525e18));

        s.store_scale(36, 34, 1.0 / (s.v[35]));

        s.store_offset(37, 34, (-s.v[35]));

        s.store_scale_ad(2, A::powf(s.ad_value(36), p.p90), p.p53);

        s.store_scale_ad(12, A::powf(s.ad_value(36), p.p91), p.p1);

        s.store_scale_ad(13, A::powf(s.ad_value(36), p.p68), p.p2);

        s.store_scale_ad(14, A::powf(s.ad_value(36), p.p92), p.p6);

        s.store_scale_ad(15, A::powf(s.ad_value(36), p.p67), p.p7);

        s.store_scale_ad(16, A::powf(s.ad_value(36), p.p66), p.p8);

        s.store_scale_ad(17, A::powf(s.ad_value(36), p.p69), p.p9);

        s.store_scale_ad(18, A::powf(s.ad_value(36), p.p93), p.p10);

        s.store_scaled_powf_ad(0, A::mul(A::powf(s.ad_value(36), p.p78), A::exp(A::div_scaled_offset_numerator(s.ad_value(36), (-(-p.p71)), (-p.p71), s.ad_value(48), 1.0))), (1.0 / p.p12), p.p11);

        s.store_scaled_powf_ad(1, A::mul(A::powf(s.ad_value(36), p.p95), A::exp(A::div_scaled_offset_numerator(s.ad_value(36), (-(-p.p96)), (-p.p96), s.ad_value(48), 1.0))), (1.0 / p.p13), p.p94);

        s.store_scaled_powf_ad(5, A::mul(A::powf(s.ad_value(36), p.p78), A::exp(A::div_scaled_offset_numerator(s.ad_value(36), (-(-p.p97)), (-p.p97), s.ad_value(48), 1.0))), (1.0 / p.p44), p.p42);

        s.store_scaled_powf_ad(3, A::mul(A::powf(s.ad_value(36), p.p79), A::exp(A::div_scaled_offset_numerator(s.ad_value(36), (-(-p.p72)), (-p.p72), s.ad_value(48), 1.0))), (1.0 / p.p33), p.p31);

        s.store_scaled_powf_ad(6, A::mul(A::powf(s.ad_value(36), p.p80), A::exp(A::div_scaled_offset_numerator(s.ad_value(36), (-(-p.p75)), (-p.p75), s.ad_value(48), 1.0))), (1.0 / p.p35), p.p34);

        s.store_scaled_powf_ad(4, A::mul(A::powf(s.ad_value(36), p.p79), A::exp(A::div_scaled_offset_numerator(s.ad_value(36), (-(-p.p73)), (-p.p73), s.ad_value(48), 1.0))), (1.0 / p.p37), p.p36);

        s.store_scaled_powf_ad(7, A::mul(A::powf(s.ad_value(36), p.p80), A::exp(A::div_scaled_offset_numerator(s.ad_value(36), (-(-p.p76)), (-p.p76), s.ad_value(48), 1.0))), (1.0 / p.p39), p.p38);

        s.store_scaled_powf_ad(8, A::mul(A::powf(s.ad_value(36), p.p79), A::exp(A::div_scaled_offset_numerator(s.ad_value(36), (-(-p.p73)), (-p.p73), s.ad_value(48), 1.0))), (1.0 / p.p37), p.p45);

        s.store_scaled_powf_ad(9, A::mul(A::powf(s.ad_value(36), p.p80), A::exp(A::div_scaled_offset_numerator(s.ad_value(36), (-(-p.p76)), (-p.p76), s.ad_value(48), 1.0))), (1.0 / p.p39), p.p46);

        s.store_scaled_powf_ad(10, A::mul(A::powf(s.ad_value(36), p.p79), A::exp(A::div_scaled_offset_numerator(s.ad_value(36), (-(-p.p74)), (-p.p74), s.ad_value(48), 1.0))), (1.0 / p.p48), p.p47);

        s.store_scaled_powf_ad(11, A::mul(A::powf(s.ad_value(36), p.p80), A::exp(A::div_scaled_offset_numerator(s.ad_value(36), (-(-p.p77)), (-p.p77), s.ad_value(48), 1.0))), (1.0 / p.p50), p.p49);

        s.store_offset_scaled(26, 37, ((p.p81) * (p.p12)), p.p12);

        s.store_offset_scaled(27, 37, ((p.p81) * (p.p13)), p.p13);

        s.store_offset_scaled(28, 37, ((p.p82) * (p.p41)), p.p41);

        s.store_scaled_offset_ad(29, A::mul(s.ad_value(37), A::scale_offset(s.ad_value(37), p.p102, p.p101)), 1.0, p.p98);

        s.store_offset_scaled(30, 37, ((p.p103) * (p.p99)), p.p99);

        s.store_scaled_mul_ad(108, A::div(s.ad_value(48), s.ad_value(36)), A::ln(A::sub(A::exp(A::div_scaled_inputs(s.ad_value(36), (0.5 * p.p17), s.ad_value(48), 1.0)), A::exp(A::div_scaled_inputs(s.ad_value(36), ((-0.5) * p.p17), s.ad_value(48), 1.0)))), 2.0);

        s.store_sub_ad(109, A::add_scaled_products(s.ad_value(108), s.ad_value(36), 1.0, s.ad_value(48), A::ln(s.ad_value(36)), (-3.0)), A::scaled_offset(s.ad_value(36), (-1.0), p.p72));

        s.store_add_scaled_product(19, s.ad_value(109), 1.0, s.ad_value(48), A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::div_scaled_inputs(s.ad_value(109), -1.0, s.ad_value(48), 1.0)), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scaled_mul_ad(108, A::div(s.ad_value(48), s.ad_value(36)), A::ln(A::sub(A::exp(A::div_scaled_inputs(s.ad_value(36), (0.5 * p.p24), s.ad_value(48), 1.0)), A::exp(A::div_scaled_inputs(s.ad_value(36), ((-0.5) * p.p24), s.ad_value(48), 1.0)))), 2.0);

        s.store_sub_ad(109, A::add_scaled_products(s.ad_value(108), s.ad_value(36), 1.0, s.ad_value(48), A::ln(s.ad_value(36)), (-3.0)), A::scaled_offset(s.ad_value(36), (-1.0), p.p73));

        s.store_add_scaled_product(20, s.ad_value(109), 1.0, s.ad_value(48), A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::div_scaled_inputs(s.ad_value(109), -1.0, s.ad_value(48), 1.0)), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scaled_mul_ad(108, A::div(s.ad_value(48), s.ad_value(36)), A::ln(A::sub(A::exp(A::div_scaled_inputs(s.ad_value(36), (0.5 * p.p28), s.ad_value(48), 1.0)), A::exp(A::div_scaled_inputs(s.ad_value(36), ((-0.5) * p.p28), s.ad_value(48), 1.0)))), 2.0);

        s.store_sub_ad(109, A::add_scaled_products(s.ad_value(108), s.ad_value(36), 1.0, s.ad_value(48), A::ln(s.ad_value(36)), (-3.0)), A::scaled_offset(s.ad_value(36), (-1.0), p.p74));

        s.store_add_scaled_product(21, s.ad_value(109), 1.0, s.ad_value(48), A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::div_scaled_inputs(s.ad_value(109), -1.0, s.ad_value(48), 1.0)), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scaled_powf_ad(22, A::div_from_scalar(p.p17, s.ad_value(19)), p.p18, p.p16);

        s.store_scaled_powf_ad(23, A::div_from_scalar(p.p24, s.ad_value(20)), p.p25, p.p21);

        s.store_scaled_powf_ad(24, A::div_from_scalar(p.p24, s.ad_value(20)), p.p25, p.p23);

        s.store_scaled_powf_ad(25, A::div_from_scalar(p.p28, s.ad_value(21)), p.p29, p.p27);

        s.store_scaled_mul_ad(31, A::powf(s.ad_value(36), p.p78), A::exp(A::div_scaled_offset_numerator(s.ad_value(36), (-(-p.p71)), (-p.p71), s.ad_value(48), 1.0)), p.p4);

        s.store_scale_ad(32, A::powf(s.ad_value(36), p.p70), p.p3);

        s.store_exp_ad(33, A::div_scaled_inputs(s.ad_value(29), -1.0, A::mul(s.ad_value(30), s.ad_value(48)), 1.0));

        s.v[38] = (if (p.p51 > 0.0) { (1.0 / p.p51) } else { 0.0 });

        s.v[39] = (if (p.p52 > 0.0) { (1.0 / p.p52) } else { 0.0 });

        if (p.p53 > 0.0) {
            s.store_div_from_scalar(40, 1.0, 2);
        } else {
            s.store_scalar(40, 0.0);
        }

        s.v[41] = (if (p.p54 > 0.0) { (1.0 / p.p54) } else { 0.0 });

        s.v[42] = (if (p.p55 > 0.0) { (1.0 / p.p55) } else { 0.0 });

        if (p.p3 > 0.0) {
            s.store_div_from_scalar(43, 1.0, 32);
        } else {
            s.store_scalar(43, 0.0);
        }

        s.v[44] = (if (p.p5 > 0.0) { (1.0 / p.p5) } else { 0.0 });

        s.v[45] = (if (p.p59 > 0.0) { (1.0 / p.p59) } else { 0.0 });

        s.v[46] = (if (p.p60 > 0.0) { (1.0 / p.p60) } else { 0.0 });

        s.b[47] = (!(p.p60 > 0.0));
        s.v[47] = if s.b[47] { 1.0 } else { 0.0 };

        s.store_voltage(132, ctx, nodes, Some(8), Some(9));

        s.store_voltage(133, ctx, nodes, Some(7), Some(9));

        s.store_voltage(134, ctx, nodes, Some(8), Some(6));

        s.store_voltage(135, ctx, nodes, Some(8), Some(5));

        s.store_voltage(136, ctx, nodes, Some(7), Some(10));

        s.store_scale(110, 19, (-p.p14));

        s.b[137] = (p.p19 <= 0.0);
        s.v[137] = if s.b[137] { 1.0 } else { 0.0 };

        if s.b[137] {
            s.store_add(111, 132, 110);
        }

        s.b[138] = (s.v[111] > 0.0);
        s.v[138] = if s.b[138] { 1.0 } else { 0.0 };

        if (s.b[137] && s.b[138]) {
            s.store_scalar(112, (((1.0 - p.p14)) as f64).powf(((-1.0) - p.p18)));
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(19), 1.0, A::scale(s.ad_value(112), ((1.0 - p.p14) * (1.0 - p.p14))), 1.0 / ((1.0 - p.p18))));
            s.store_mul_ad_product_lhs(114, s.ad_value(111), A::offset(A::div_scaled_inputs(s.ad_value(111), (0.5 * p.p18), s.ad_value(19), 1.0), (1.0 - p.p14)), 112);
        }

        if (s.b[137] && (!s.b[138])) {
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(19), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(132), s.ad_value(19))), (1.0 - p.p18)), 1.0 / ((1.0 - p.p18))));
            s.store_scalar(114, 0.0);
        }

        if s.b[137] {
            s.store_add(88, 113, 114);
        }

        if (!s.b[137]) {
            s.store_sqrt_square_offset(115, 110, ((4.0 * p.p19) * p.p19));
            s.store_scaled_add(116, 110, 115, (-0.5));
            s.store_mul_scaled_ad_rhs(117, 19, (-1.0 / ((1.0 - p.p18))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(116), s.ad_value(19))), (1.0 - p.p18)));
            s.store_add(118, 132, 110);
            s.store_sqrt_square_offset(119, 118, ((4.0 * p.p19) * p.p19));
            s.store_add_scaled_inputs3(120, s.ad_value(118), 0.5, s.ad_value(119), (-0.5), s.ad_value(110), -1.0);
            s.store_mul_scaled_ad_rhs(113, 19, (-1.0 / ((1.0 - p.p18))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(120), s.ad_value(19))), (1.0 - p.p18)));
            s.store_sub_ad_lhs(88, A::add_scaled_inputs4(s.ad_value(113), 1.0, s.ad_value(132), (((1.0 - p.p14)) as f64).powf((-p.p18)), s.ad_value(120), ((-1.0) * (((1.0 - p.p14)) as f64).powf((-p.p18))), s.ad_value(116), (((1.0 - p.p14)) as f64).powf((-p.p18))), 117);
        }

        s.store_scale(110, 19, (-p.p14));

        s.b[139] = (p.p19 <= 0.0);
        s.v[139] = if s.b[139] { 1.0 } else { 0.0 };

        if s.b[139] {
            s.store_add(111, 133, 110);
        }

        s.b[140] = (s.v[111] > 0.0);
        s.v[140] = if s.b[140] { 1.0 } else { 0.0 };

        if (s.b[139] && s.b[140]) {
            s.store_scalar(112, (((1.0 - p.p14)) as f64).powf(((-1.0) - p.p18)));
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(19), 1.0, A::scale(s.ad_value(112), ((1.0 - p.p14) * (1.0 - p.p14))), 1.0 / ((1.0 - p.p18))));
            s.store_mul_ad_product_lhs(114, s.ad_value(111), A::offset(A::div_scaled_inputs(s.ad_value(111), (0.5 * p.p18), s.ad_value(19), 1.0), (1.0 - p.p14)), 112);
        }

        if (s.b[139] && (!s.b[140])) {
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(19), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(133), s.ad_value(19))), (1.0 - p.p18)), 1.0 / ((1.0 - p.p18))));
            s.store_scalar(114, 0.0);
        }

        if s.b[139] {
            s.store_add(89, 113, 114);
        }

        if (!s.b[139]) {
            s.store_sqrt_square_offset(115, 110, ((4.0 * p.p19) * p.p19));
            s.store_scaled_add(116, 110, 115, (-0.5));
            s.store_mul_scaled_ad_rhs(117, 19, (-1.0 / ((1.0 - p.p18))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(116), s.ad_value(19))), (1.0 - p.p18)));
            s.store_add(118, 133, 110);
            s.store_sqrt_square_offset(119, 118, ((4.0 * p.p19) * p.p19));
            s.store_add_scaled_inputs3(120, s.ad_value(118), 0.5, s.ad_value(119), (-0.5), s.ad_value(110), -1.0);
            s.store_mul_scaled_ad_rhs(113, 19, (-1.0 / ((1.0 - p.p18))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(120), s.ad_value(19))), (1.0 - p.p18)));
            s.store_sub_ad_lhs(89, A::add_scaled_inputs4(s.ad_value(113), 1.0, s.ad_value(133), (((1.0 - p.p14)) as f64).powf((-p.p18)), s.ad_value(120), ((-1.0) * (((1.0 - p.p14)) as f64).powf((-p.p18))), s.ad_value(116), (((1.0 - p.p14)) as f64).powf((-p.p18))), 117);
        }

        s.store_scale(110, 20, (-p.p14));

        s.b[141] = (p.p26 <= 0.0);
        s.v[141] = if s.b[141] { 1.0 } else { 0.0 };

        if s.b[141] {
            s.store_add(111, 134, 110);
        }

        s.b[142] = (s.v[111] > 0.0);
        s.v[142] = if s.b[142] { 1.0 } else { 0.0 };

        if (s.b[141] && s.b[142]) {
            s.store_scalar(112, (((1.0 - p.p14)) as f64).powf(((-1.0) - p.p25)));
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(20), 1.0, A::scale(s.ad_value(112), ((1.0 - p.p14) * (1.0 - p.p14))), 1.0 / ((1.0 - p.p25))));
            s.store_mul_ad_product_lhs(114, s.ad_value(111), A::offset(A::div_scaled_inputs(s.ad_value(111), (0.5 * p.p25), s.ad_value(20), 1.0), (1.0 - p.p14)), 112);
        }

        s.b[143] = ((p.p85 > 0.0) && (s.v[134] < (-p.p85)));
        s.v[143] = if s.b[143] { 1.0 } else { 0.0 };

        if ((s.b[141] && (!s.b[142])) && s.b[143]) {
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(20), 1.0, A::mul_sub_from_scalar_rhs(A::powf(A::offset(A::div_from_scalar(p.p85, s.ad_value(20)), 1.0), (1.0 - p.p25)), 1.0, A::div_scaled_offset_numerator(s.ad_value(134), (1.0 - p.p25), (p.p85 * (1.0 - p.p25)), A::offset(s.ad_value(20), p.p85), 1.0)), 1.0 / ((1.0 - p.p25))));
        }

        if ((s.b[141] && (!s.b[142])) && (!s.b[143])) {
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(20), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(134), s.ad_value(20))), (1.0 - p.p25)), 1.0 / ((1.0 - p.p25))));
        }

        if (s.b[141] && (!s.b[142])) {
            s.store_scalar(114, 0.0);
        }

        if s.b[141] {
            s.store_add(90, 113, 114);
        }

        s.b[144] = ((p.p85 > 0.0) && (p.p86 > 0.0));
        s.v[144] = if s.b[144] { 1.0 } else { 0.0 };

        if ((!s.b[141]) && s.b[144]) {
            s.store_ad_value(121, A::div_scaled_offset_numerator(s.ad_value(110), 1.0, p.p85, A::sub_from_scalar(p.p85, s.ad_value(110)), 1.0));
            s.store_ad_value(122, A::div_scaled_inputs(s.ad_value(121), 2.0, A::add(A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(121), (-1.0), A::offset(s.ad_value(121), (-1.0))), ((4.0 * p.p26) * p.p26))), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(121), 1.0, A::offset(s.ad_value(121), 1.0)), ((4.0 * p.p86) * p.p86)))), 1.0));
            s.store_scaled_sub_ad_lhs(116, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(122), p.p85, s.ad_value(110)), (-p.p85)), 110, 0.5);
            s.store_ad_value(123, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(20), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(116), s.ad_value(20))), (1.0 - p.p25)), 1.0 / ((1.0 - p.p25))));
            s.store_ad_value(124, A::div_scaled_inputs2(A::scale_offset(s.ad_value(134), 2.0, p.p85), 1.0, s.ad_value(110), 1.0, A::sub_from_scalar(p.p85, s.ad_value(110)), 1.0));
            s.store_ad_value(125, A::div_scaled_inputs(s.ad_value(124), 2.0, A::add(A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(124), (-1.0), A::offset(s.ad_value(124), (-1.0))), ((4.0 * p.p26) * p.p26))), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(124), 1.0, A::offset(s.ad_value(124), 1.0)), ((4.0 * p.p86) * p.p86)))), 1.0));
            s.store_scaled_sub_ad_lhs(120, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(125), p.p85, s.ad_value(110)), (-p.p85)), 110, 0.5);
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(20), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(120), s.ad_value(20))), (1.0 - p.p25)), 1.0 / ((1.0 - p.p25))));
            s.store_scaled_offset(126, 125, 1.0, 0.5);
            s.store_powf_ad(127, A::offset(A::div_from_scalar(p.p85, s.ad_value(20)), 1.0), (-p.p25));
            s.store_powf_ad(128, A::offset(A::div(s.ad_value(110), s.ad_value(20)), 1.0), (-p.p25));
            s.store_add_scaled_product(129, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(126), s.ad_value(127)), 1.0, s.ad_value(126), s.ad_value(128), 1.0);
            s.store_mul_ad_lhs(130, A::add_scaled_inputs3(s.ad_value(134), 1.0, s.ad_value(120), (-1.0), s.ad_value(116), 1.0), 129);
            s.store_add_scaled_inputs3(90, s.ad_value(130), 1.0, s.ad_value(113), 1.0, s.ad_value(123), -1.0);
        }

        if ((!s.b[141]) && (!s.b[144])) {
            s.store_sqrt_square_offset(115, 110, ((4.0 * p.p26) * p.p26));
            s.store_scaled_add(116, 110, 115, (-0.5));
            s.store_mul_scaled_ad_rhs(117, 20, (-1.0 / ((1.0 - p.p25))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(116), s.ad_value(20))), (1.0 - p.p25)));
            s.store_add(118, 134, 110);
            s.store_sqrt_square_offset(119, 118, ((4.0 * p.p26) * p.p26));
            s.store_add_scaled_inputs3(120, s.ad_value(118), 0.5, s.ad_value(119), (-0.5), s.ad_value(110), -1.0);
            s.store_mul_scaled_ad_rhs(113, 20, (-1.0 / ((1.0 - p.p25))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(120), s.ad_value(20))), (1.0 - p.p25)));
            s.store_sub_ad_lhs(90, A::add_scaled_inputs4(s.ad_value(113), 1.0, s.ad_value(134), (((1.0 - p.p14)) as f64).powf((-p.p25)), s.ad_value(120), ((-1.0) * (((1.0 - p.p14)) as f64).powf((-p.p25))), s.ad_value(116), (((1.0 - p.p14)) as f64).powf((-p.p25))), 117);
        }

        s.store_scale(110, 20, (-p.p14));

        s.b[145] = (p.p26 <= 0.0);
        s.v[145] = if s.b[145] { 1.0 } else { 0.0 };

        if s.b[145] {
            s.store_add(111, 136, 110);
        }

        s.b[146] = (s.v[111] > 0.0);
        s.v[146] = if s.b[146] { 1.0 } else { 0.0 };

        if (s.b[145] && s.b[146]) {
            s.store_scalar(112, (((1.0 - p.p14)) as f64).powf(((-1.0) - p.p25)));
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(20), 1.0, A::scale(s.ad_value(112), ((1.0 - p.p14) * (1.0 - p.p14))), 1.0 / ((1.0 - p.p25))));
            s.store_mul_ad_product_lhs(114, s.ad_value(111), A::offset(A::div_scaled_inputs(s.ad_value(111), (0.5 * p.p25), s.ad_value(20), 1.0), (1.0 - p.p14)), 112);
        }

        s.b[147] = ((p.p85 > 0.0) && (s.v[136] < (-p.p85)));
        s.v[147] = if s.b[147] { 1.0 } else { 0.0 };

        if ((s.b[145] && (!s.b[146])) && s.b[147]) {
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(20), 1.0, A::mul_sub_from_scalar_rhs(A::powf(A::offset(A::div_from_scalar(p.p85, s.ad_value(20)), 1.0), (1.0 - p.p25)), 1.0, A::div_scaled_offset_numerator(s.ad_value(136), (1.0 - p.p25), (p.p85 * (1.0 - p.p25)), A::offset(s.ad_value(20), p.p85), 1.0)), 1.0 / ((1.0 - p.p25))));
        }

        if ((s.b[145] && (!s.b[146])) && (!s.b[147])) {
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(20), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(136), s.ad_value(20))), (1.0 - p.p25)), 1.0 / ((1.0 - p.p25))));
        }

        if (s.b[145] && (!s.b[146])) {
            s.store_scalar(114, 0.0);
        }

        if s.b[145] {
            s.store_add(91, 113, 114);
        }

        s.b[148] = ((p.p85 > 0.0) && (p.p86 > 0.0));
        s.v[148] = if s.b[148] { 1.0 } else { 0.0 };

        if ((!s.b[145]) && s.b[148]) {
            s.store_ad_value(121, A::div_scaled_offset_numerator(s.ad_value(110), 1.0, p.p85, A::sub_from_scalar(p.p85, s.ad_value(110)), 1.0));
            s.store_ad_value(122, A::div_scaled_inputs(s.ad_value(121), 2.0, A::add(A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(121), (-1.0), A::offset(s.ad_value(121), (-1.0))), ((4.0 * p.p26) * p.p26))), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(121), 1.0, A::offset(s.ad_value(121), 1.0)), ((4.0 * p.p86) * p.p86)))), 1.0));
            s.store_scaled_sub_ad_lhs(116, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(122), p.p85, s.ad_value(110)), (-p.p85)), 110, 0.5);
            s.store_ad_value(123, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(20), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(116), s.ad_value(20))), (1.0 - p.p25)), 1.0 / ((1.0 - p.p25))));
            s.store_ad_value(124, A::div_scaled_inputs2(A::scale_offset(s.ad_value(136), 2.0, p.p85), 1.0, s.ad_value(110), 1.0, A::sub_from_scalar(p.p85, s.ad_value(110)), 1.0));
            s.store_ad_value(125, A::div_scaled_inputs(s.ad_value(124), 2.0, A::add(A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(124), (-1.0), A::offset(s.ad_value(124), (-1.0))), ((4.0 * p.p26) * p.p26))), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(124), 1.0, A::offset(s.ad_value(124), 1.0)), ((4.0 * p.p86) * p.p86)))), 1.0));
            s.store_scaled_sub_ad_lhs(120, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(125), p.p85, s.ad_value(110)), (-p.p85)), 110, 0.5);
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(20), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(120), s.ad_value(20))), (1.0 - p.p25)), 1.0 / ((1.0 - p.p25))));
            s.store_scaled_offset(126, 125, 1.0, 0.5);
            s.store_powf_ad(127, A::offset(A::div_from_scalar(p.p85, s.ad_value(20)), 1.0), (-p.p25));
            s.store_powf_ad(128, A::offset(A::div(s.ad_value(110), s.ad_value(20)), 1.0), (-p.p25));
            s.store_add_scaled_product(129, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(126), s.ad_value(127)), 1.0, s.ad_value(126), s.ad_value(128), 1.0);
            s.store_mul_ad_lhs(130, A::add_scaled_inputs3(s.ad_value(136), 1.0, s.ad_value(120), (-1.0), s.ad_value(116), 1.0), 129);
            s.store_add_scaled_inputs3(91, s.ad_value(130), 1.0, s.ad_value(113), 1.0, s.ad_value(123), -1.0);
        }

        if ((!s.b[145]) && (!s.b[148])) {
            s.store_sqrt_square_offset(115, 110, ((4.0 * p.p26) * p.p26));
            s.store_scaled_add(116, 110, 115, (-0.5));
        }

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((!s.b[145]) && (!s.b[148])) {
            s.store_mul_scaled_ad_rhs(117, 20, (-1.0 / ((1.0 - p.p25))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(116), s.ad_value(20))), (1.0 - p.p25)));
            s.store_add(118, 136, 110);
            s.store_sqrt_square_offset(119, 118, ((4.0 * p.p26) * p.p26));
            s.store_add_scaled_inputs3(120, s.ad_value(118), 0.5, s.ad_value(119), (-0.5), s.ad_value(110), -1.0);
            s.store_mul_scaled_ad_rhs(113, 20, (-1.0 / ((1.0 - p.p25))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(120), s.ad_value(20))), (1.0 - p.p25)));
            s.store_sub_ad_lhs(91, A::add_scaled_inputs4(s.ad_value(113), 1.0, s.ad_value(136), (((1.0 - p.p14)) as f64).powf((-p.p25)), s.ad_value(120), ((-1.0) * (((1.0 - p.p14)) as f64).powf((-p.p25))), s.ad_value(116), (((1.0 - p.p14)) as f64).powf((-p.p25))), 117);
        }

        s.b[149] = (p.p27 > 0.0);
        s.v[149] = if s.b[149] { 1.0 } else { 0.0 };

        if s.b[149] {
            s.store_scale(110, 21, (-p.p14));
        }

        s.b[150] = (p.p30 <= 0.0);
        s.v[150] = if s.b[150] { 1.0 } else { 0.0 };

        if (s.b[149] && s.b[150]) {
            s.store_add_ad_lhs(111, A::voltage(ctx, nodes, Some(11), Some(10)), 110);
        }

        s.b[151] = (s.v[111] > 0.0);
        s.v[151] = if s.b[151] { 1.0 } else { 0.0 };

        if ((s.b[149] && s.b[150]) && s.b[151]) {
            s.store_scalar(112, (((1.0 - p.p14)) as f64).powf(((-1.0) - p.p29)));
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(21), 1.0, A::scale(s.ad_value(112), ((1.0 - p.p14) * (1.0 - p.p14))), 1.0 / ((1.0 - p.p29))));
            s.store_mul_ad_product_lhs(114, s.ad_value(111), A::offset(A::div_scaled_inputs(s.ad_value(111), (0.5 * p.p29), s.ad_value(21), 1.0), (1.0 - p.p14)), 112);
        }

        if ((s.b[149] && s.b[150]) && (!s.b[151])) {
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(21), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(A::voltage(ctx, nodes, Some(11), Some(10)), s.ad_value(21))), (1.0 - p.p29)), 1.0 / ((1.0 - p.p29))));
            s.store_scalar(114, 0.0);
        }

        if (s.b[149] && s.b[150]) {
            s.store_add(92, 113, 114);
        }

        if (s.b[149] && (!s.b[150])) {
            s.store_sqrt_square_offset(115, 110, ((4.0 * p.p30) * p.p30));
            s.store_scaled_add(116, 110, 115, (-0.5));
            s.store_mul_scaled_ad_rhs(117, 21, (-1.0 / ((1.0 - p.p29))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(116), s.ad_value(21))), (1.0 - p.p29)));
            s.store_add_ad_lhs(118, A::voltage(ctx, nodes, Some(11), Some(10)), 110);
            s.store_sqrt_square_offset(119, 118, ((4.0 * p.p30) * p.p30));
            s.store_add_scaled_inputs3(120, s.ad_value(118), 0.5, s.ad_value(119), (-0.5), s.ad_value(110), -1.0);
            s.store_mul_scaled_ad_rhs(113, 21, (-1.0 / ((1.0 - p.p29))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(120), s.ad_value(21))), (1.0 - p.p29)));
            s.store_sub_ad_lhs(92, A::add_scaled_inputs4(s.ad_value(113), 1.0, A::voltage(ctx, nodes, Some(11), Some(10)), (((1.0 - p.p14)) as f64).powf((-p.p29)), s.ad_value(120), ((-1.0) * (((1.0 - p.p14)) as f64).powf((-p.p29))), s.ad_value(116), (((1.0 - p.p14)) as f64).powf((-p.p29))), 117);
        }

        if (!s.b[149]) {
            s.store_scalar(92, 0.0);
        }

        s.store_div_ad_rhs(82, 132, A::mul(s.ad_value(26), s.ad_value(48)));

        s.store_limexp(83, 82);

        s.store_mul_offset_rhs(49, 0, 83, (-1.0));

        s.store_div_ad_rhs(82, 134, A::mul(s.ad_value(27), s.ad_value(48)));

        s.store_limexp(83, 82);

        s.store_mul_ad_product_rhs(50, 0, s.ad_value(1), A::offset(s.ad_value(83), (-1.0)));

        s.store_add_scaled_ad_lhs(53, A::scale_offset(s.ad_value(88), s.v[39], 1.0), 90, s.v[38]);

        s.store_offset_scaled_ad(54, A::add(A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(53), (-0.0001), A::offset(s.ad_value(53), (-0.0001))), 1e-8)), s.ad_value(53)), 0.5, (((((-0.0001)) * (0.5))) + (0.0001)));

        s.store_add_scaled_product(55, s.ad_value(50), s.v[41], s.ad_value(49), s.ad_value(40), 1.0);

        s.b[152] = (p.p88 < 0.5);
        s.v[152] = if s.b[152] { 1.0 } else { 0.0 };

        if s.b[152] {
            s.store_scaled_add_ad_rhs(56, 54, A::powf(A::add_scaled_inputs(A::powf(s.ad_value(54), (1.0 / p.p89)), 1.0, s.ad_value(55), 4.0), p.p89), 0.5);
        }

        if (!s.b[152]) {
            s.store_mul_scaled_ad_rhs(56, 54, 0.5, A::offset(A::powf(A::scale_offset(s.ad_value(55), 4.0, 1.0), p.p89), 1.0));
        }

        s.store_div(52, 50, 56);

        s.store_div(51, 49, 56);

        s.b[153] = (p.p42 > 0.0);
        s.v[153] = if s.b[153] { 1.0 } else { 0.0 };

        if s.b[153] {
            s.store_scaled_div(82, 136, 48, (1.0 / (p.p44)));
            s.store_limexp(83, 82);
            s.store_scaled_div(86, 134, 48, (1.0 / (p.p44)));
            s.store_limexp(87, 86);
            s.store_mul_offset_ad_rhs(57, 5, A::add_scaled_inputs(s.ad_value(83), p.p43, s.ad_value(87), (1.0 - p.p43)), (-1.0));
            s.store_scale(60, 57, s.v[42]);
            s.store_scaled_offset_ad(61, A::sqrt(A::scale_offset(s.ad_value(60), 4.0, 1.0)), 1.0, 0.5);
            s.store_ad_value(82, A::div_scaled_inputs(A::voltage(ctx, nodes, Some(11), Some(10)), 1.0, s.ad_value(48), p.p44));
            s.store_limexp(83, 82);
            s.store_mul_offset_rhs(58, 5, 83, (-1.0));
            s.store_ad_value(59, A::div_scaled_inputs2(s.ad_value(57), 1.0, s.ad_value(58), (-1.0), s.ad_value(61), 1.0));
        }

        if (!s.b[153]) {
            s.store_scalar(57, 0.0);
            s.store_scalar(61, 1.0);
            s.store_scalar(59, 0.0);
        }

        s.b[154] = (p.p32 == 1.0);
        s.v[154] = if s.b[154] { 1.0 } else { 0.0 };

        if s.b[154] {
            s.store_scaled_div(82, 132, 48, (1.0 / (p.p33)));
            s.store_limexp(83, 82);
            s.store_scaled_div(84, 132, 48, (1.0 / (p.p35)));
            s.store_limexp(85, 84);
        }

        s.b[155] = (p.p98 > 0.0);
        s.v[155] = if s.b[155] { 1.0 } else { 0.0 };

        if (s.b[154] && s.b[155]) {
            s.store_ad_value(86, A::div_scaled_inputs2(s.ad_value(29), -1.0, s.ad_value(132), (-1.0), A::mul(s.ad_value(30), s.ad_value(48)), 1.0));
            s.store_limexp(87, 86);
            s.store_add_scaled_inputs3(62, A::add_scaled_products(s.ad_value(3), A::offset(s.ad_value(83), (-1.0)), 1.0, s.ad_value(6), A::offset(s.ad_value(85), (-1.0)), 1.0), 1.0, s.ad_value(87), (-p.p100), s.ad_value(33), (-(-p.p100)));
        }

        if (s.b[154] && (!s.b[155])) {
            s.store_add_scaled_products(62, s.ad_value(3), A::offset(s.ad_value(83), (-1.0)), 1.0, s.ad_value(6), A::offset(s.ad_value(85), (-1.0)), 1.0);
        }

        if s.b[154] {
            s.store_scalar(63, 0.0);
        }

        s.b[156] = (p.p32 == 0.0);
        s.v[156] = if s.b[156] { 1.0 } else { 0.0 };

        if ((!s.b[154]) && s.b[156]) {
            s.store_scalar(62, 0.0);
            s.store_scaled_div(82, 133, 48, (1.0 / (p.p33)));
            s.store_limexp(83, 82);
            s.store_scaled_div(84, 133, 48, (1.0 / (p.p35)));
            s.store_limexp(85, 84);
        }

        s.b[157] = (p.p98 > 0.0);
        s.v[157] = if s.b[157] { 1.0 } else { 0.0 };

        if (((!s.b[154]) && s.b[156]) && s.b[157]) {
            s.store_ad_value(86, A::div_scaled_inputs2(s.ad_value(29), -1.0, s.ad_value(133), (-1.0), A::mul(s.ad_value(30), s.ad_value(48)), 1.0));
            s.store_limexp(87, 86);
            s.store_add_scaled_inputs3(63, A::add_scaled_products(s.ad_value(3), A::offset(s.ad_value(83), (-1.0)), 1.0, s.ad_value(6), A::offset(s.ad_value(85), (-1.0)), 1.0), 1.0, s.ad_value(87), (-p.p100), s.ad_value(33), (-(-p.p100)));
        }

        if (((!s.b[154]) && s.b[156]) && (!s.b[157])) {
            s.store_add_scaled_products(63, s.ad_value(3), A::offset(s.ad_value(83), (-1.0)), 1.0, s.ad_value(6), A::offset(s.ad_value(85), (-1.0)), 1.0);
        }

        if ((!s.b[154]) && (!s.b[156])) {
            s.store_scaled_div(82, 132, 48, (1.0 / (p.p33)));
            s.store_limexp(83, 82);
            s.store_scaled_div(84, 132, 48, (1.0 / (p.p35)));
            s.store_limexp(85, 84);
        }

        s.b[158] = (p.p98 > 0.0);
        s.v[158] = if s.b[158] { 1.0 } else { 0.0 };

        if (((!s.b[154]) && (!s.b[156])) && s.b[158]) {
            s.store_ad_value(86, A::div_scaled_inputs2(s.ad_value(29), -1.0, s.ad_value(132), (-1.0), A::mul(s.ad_value(30), s.ad_value(48)), 1.0));
            s.store_limexp(87, 86);
            s.store_add_scaled_inputs3(62, A::add_scaled_products(s.ad_value(3), A::offset(s.ad_value(83), (-1.0)), 1.0, s.ad_value(6), A::offset(s.ad_value(85), (-1.0)), 1.0), p.p32, s.ad_value(87), ((-p.p100) * p.p32), s.ad_value(33), ((-(-p.p100)) * p.p32));
        }

        if (((!s.b[154]) && (!s.b[156])) && (!s.b[158])) {
            s.store_add_scaled_products(62, s.ad_value(3), A::offset(s.ad_value(83), (-1.0)), p.p32, s.ad_value(6), A::offset(s.ad_value(85), (-1.0)), p.p32);
        }

        if ((!s.b[154]) && (!s.b[156])) {
            s.store_scaled_div(82, 133, 48, (1.0 / (p.p33)));
            s.store_limexp(83, 82);
            s.store_scaled_div(84, 133, 48, (1.0 / (p.p35)));
            s.store_limexp(85, 84);
        }

        s.b[159] = (p.p98 > 0.0);
        s.v[159] = if s.b[159] { 1.0 } else { 0.0 };

        if (((!s.b[154]) && (!s.b[156])) && s.b[159]) {
            s.store_ad_value(86, A::div_scaled_inputs2(s.ad_value(29), -1.0, s.ad_value(133), (-1.0), A::mul(s.ad_value(30), s.ad_value(48)), 1.0));
            s.store_limexp(87, 86);
            s.store_add_scaled_inputs3(63, A::add_scaled_products(s.ad_value(3), A::offset(s.ad_value(83), (-1.0)), 1.0, s.ad_value(6), A::offset(s.ad_value(85), (-1.0)), 1.0), (1.0 - p.p32), s.ad_value(87), ((-p.p100) * (1.0 - p.p32)), s.ad_value(33), ((-(-p.p100)) * (1.0 - p.p32)));
        }

        if (((!s.b[154]) && (!s.b[156])) && (!s.b[159])) {
            s.store_add_scaled_products(63, s.ad_value(3), A::offset(s.ad_value(83), (-1.0)), (1.0 - p.p32), s.ad_value(6), A::offset(s.ad_value(85), (-1.0)), (1.0 - p.p32));
        }

        s.store_scaled_div(82, 134, 48, (1.0 / (p.p37)));

        s.store_limexp(83, 82);

        s.store_scaled_div(84, 134, 48, (1.0 / (p.p39)));

        s.store_limexp(85, 84);

        s.store_add_scaled_products(64, s.ad_value(4), A::offset(s.ad_value(83), (-1.0)), 1.0, s.ad_value(7), A::offset(s.ad_value(85), (-1.0)), 1.0);

        s.b[160] = ((p.p45 > 0.0) || (p.p46 > 0.0));
        s.v[160] = if s.b[160] { 1.0 } else { 0.0 };

        if s.b[160] {
            s.store_scaled_div(82, 136, 48, (1.0 / (p.p37)));
            s.store_limexp(83, 82);
            s.store_scaled_div(84, 136, 48, (1.0 / (p.p39)));
            s.store_limexp(85, 84);
            s.store_add_scaled_products(66, s.ad_value(8), A::offset(s.ad_value(83), (-1.0)), 1.0, s.ad_value(9), A::offset(s.ad_value(85), (-1.0)), 1.0);
        }

        if (!s.b[160]) {
            s.store_scalar(66, 0.0);
        }

        s.b[161] = (p.p40 > 0.0);
        s.v[161] = if s.b[161] { 1.0 } else { 0.0 };

        if s.b[161] {
            s.store_add_scaled_inputs3(120, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(20), s.ad_value(134)), A::sub(s.ad_value(20), s.ad_value(134))), 0.01)), 0.5, s.ad_value(20), 0.5, s.ad_value(134), ((-1.0) * 0.5));
            s.store_mul_scaled_ad_rhs(69, 120, p.p40, A::limexp(A::mul_scaled_lhs(s.ad_value(28), -1.0, A::powf(s.ad_value(120), (p.p25 - 1.0)))));
            s.store_mul_ad_lhs(68, A::add_scaled_inputs3(s.ad_value(51), 1.0, s.ad_value(52), (-1.0), s.ad_value(64), -1.0), 69);
        }

        if (!s.b[161]) {
            s.store_scalar(68, 0.0);
        }

        s.store_sub(65, 64, 68);

        s.b[162] = (p.p1 > 0.0);
        s.v[162] = if s.b[162] { 1.0 } else { 0.0 };

        if s.b[162] {
            s.store_div_voltage_by_ad(70, ctx, nodes, Some(0), Some(5), s.ad_value(12));
        }

        if (!s.b[162]) {
            s.store_scalar(70, 0.0);
        }

        s.store_div(82, 134, 48);

        s.store_limexp(83, 82);

        s.store_div(86, 135, 48);

        s.store_limexp(87, 86);

        s.store_sqrt_offset_ad(77, A::mul(s.ad_value(31), s.ad_value(83)), 1.0);

        s.store_sqrt_offset_ad(78, A::mul(s.ad_value(31), s.ad_value(87)), 1.0);

        s.b[163] = (p.p2 > 0.0);
        s.v[163] = if s.b[163] { 1.0 } else { 0.0 };

        if s.b[163] {
            s.store_ad_value(79, A::div_scaled_offset_numerator(s.ad_value(77), 1.0, 1.0, A::offset(s.ad_value(78), 1.0), 1.0));
            s.store_ad_value(80, A::div_scaled_add_product(A::voltage(ctx, nodes, Some(5), Some(6)), 1.0, s.ad_value(48), A::add_scaled_inputs3(s.ad_value(77), 1.0, s.ad_value(78), (-1.0), A::ln(s.ad_value(79)), -1.0), 1.0, s.ad_value(13), 1.0));
            s.store_ad_value(81, A::div_scaled_product3(s.ad_value(43), s.ad_value(13), s.ad_value(80), 1.0, A::offset(A::mul_scaled_lhs(s.ad_value(43), (0.5 * s.v[44]), A::sqrt(A::offset(A::square(A::voltage(ctx, nodes, Some(5), Some(6))), 0.01))), 1.0), 1.0));
            s.store_div_ad_rhs(71, 80, A::sqrt(A::offset(A::square(s.ad_value(81)), 1.0)));
        }

        if (!s.b[163]) {
            s.store_scalar(71, 0.0);
        }

        s.b[164] = (p.p6 > 0.0);
        s.v[164] = if s.b[164] { 1.0 } else { 0.0 };

        if s.b[164] {
            s.store_div_voltage_by_ad(72, ctx, nodes, Some(1), Some(7), s.ad_value(14));
        }

        if (!s.b[164]) {
            s.store_scalar(72, 0.0);
        }

        s.b[165] = (p.p7 > 0.0);
        s.v[165] = if s.b[165] { 1.0 } else { 0.0 };

        if s.b[165] {
            s.store_div_scaled_product(73, A::voltage(ctx, nodes, Some(7), Some(8)), s.ad_value(56), 1.0, s.ad_value(15), 1.0);
        }

        if (!s.b[165]) {
            s.store_scalar(73, 0.0);
        }

        s.b[166] = (p.p8 > 0.0);
        s.v[166] = if s.b[166] { 1.0 } else { 0.0 };

        if s.b[166] {
            s.store_div_voltage_by_ad(74, ctx, nodes, Some(2), Some(9), s.ad_value(16));
        }

        if (!s.b[166]) {
            s.store_scalar(74, 0.0);
        }

        s.b[167] = (p.p10 > 0.0);
        s.v[167] = if s.b[167] { 1.0 } else { 0.0 };

        if s.b[167] {
            s.store_div_scaled_product(75, A::voltage(ctx, nodes, Some(10), Some(5)), s.ad_value(61), 1.0, s.ad_value(18), 1.0);
        }

        if (!s.b[167]) {
            s.store_scalar(75, 0.0);
        }

        s.b[168] = ((p.p47 > 0.0) || (p.p49 > 0.0));
        s.v[168] = if s.b[168] { 1.0 } else { 0.0 };

        if s.b[168] {
            s.store_ad_value(82, A::div_scaled_inputs(A::voltage(ctx, nodes, Some(11), Some(10)), 1.0, s.ad_value(48), p.p48));
            s.store_limexp(83, 82);
            s.store_ad_value(84, A::div_scaled_inputs(A::voltage(ctx, nodes, Some(11), Some(10)), 1.0, s.ad_value(48), p.p50));
        }

    }

    pub(super) fn stamp_transient_block_2(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[168] {
            s.store_limexp(85, 84);
            s.store_add_scaled_products(67, s.ad_value(10), A::offset(s.ad_value(83), (-1.0)), 1.0, s.ad_value(11), A::offset(s.ad_value(85), (-1.0)), 1.0);
        }

        if (!s.b[168]) {
            s.store_scalar(67, 0.0);
        }

        s.b[169] = (p.p9 > 0.0);
        s.v[169] = if s.b[169] { 1.0 } else { 0.0 };

        if s.b[169] {
            s.store_div_voltage_by_ad(76, ctx, nodes, Some(3), Some(11), s.ad_value(17));
        }

        if (!s.b[169]) {
            s.store_scalar(76, 0.0);
        }

        s.b[93] = (s.v[49] > 0.0);
        s.v[93] = if s.b[93] { 1.0 } else { 0.0 };

        s.store_scale(94, 49, (s.v[93] * s.v[46]));

        s.store_ad_value(95, A::div_scaled_value_offset_denominator(s.ad_value(94), 1.0, s.ad_value(94), 1.0, 1.0));

        s.store_scaled_mul_ad(96, A::scale_offset(s.ad_value(54), p.p57, 1.0), A::offset(A::mul_offset_rhs_scaled_output(A::limexp_scaled_input(s.ad_value(134), (s.v[45] * 1.0 / (1.44))), A::square(s.ad_value(95)), s.v[47], (p.p58 * s.v[93])), 1.0), p.p56);

        s.store_add_scaled_product(97, A::div_scaled_product(s.ad_value(96), s.ad_value(49), 1.0, s.ad_value(56), 1.0), 1.0, s.ad_value(22), s.ad_value(88), p.p32);

        s.store_scaled_mul(98, 22, 89, (1.0 - p.p32));

        s.store_add_scaled_ad_lhs(99, A::add_scaled_product(s.ad_value(50), p.p61, s.ad_value(23), s.ad_value(90), 1.0), 77, p.p22);

        s.store_scale(100, 78, p.p22);

        s.store_add_scaled_product(101, s.ad_value(57), p.p61, s.ad_value(24), s.ad_value(91), 1.0);

        s.store_add_scaled_product(102, A::voltage(ctx, nodes, Some(11), Some(10)), p.p87, s.ad_value(25), s.ad_value(92), 1.0);

        s.store_scaled_voltage(103, ctx, nodes, Some(1), Some(2), p.p15);

        s.store_scaled_voltage(104, ctx, nodes, Some(1), Some(0), p.p20);

        let assign3230_ad_e3802: A = A::add_scaled_product(A::add_scaled_value_products3(A::add_scaled_products3(s.ad_value(62), s.ad_value(132), 1.0, s.ad_value(65), s.ad_value(134), 1.0, A::sub(s.ad_value(51), s.ad_value(52)), A::voltage(ctx, nodes, Some(6), Some(9)), 1.0), 1.0, s.ad_value(63), s.ad_value(133), 1.0, s.ad_value(66), s.ad_value(136), 1.0, s.ad_value(76), A::voltage(ctx, nodes, Some(3), Some(11)), 1.0), 1.0, s.ad_value(67), A::voltage(ctx, nodes, Some(11), Some(10)), 1.0);
        let assign3230_ad_e3826: A = A::add_scaled_value_products3(A::add_scaled_value_products3(assign3230_ad_e3802, 1.0, s.ad_value(59), A::voltage(ctx, nodes, Some(7), Some(11)), 1.0, s.ad_value(70), A::voltage(ctx, nodes, Some(0), Some(5)), 1.0, s.ad_value(71), A::voltage(ctx, nodes, Some(5), Some(6)), 1.0), 1.0, s.ad_value(72), A::voltage(ctx, nodes, Some(1), Some(7)), 1.0, s.ad_value(73), A::voltage(ctx, nodes, Some(7), Some(8)), 1.0, s.ad_value(74), A::voltage(ctx, nodes, Some(2), Some(9)), 1.0);
        s.store_neg_ad(105, A::add_scaled_product(assign3230_ad_e3826, 1.0, s.ad_value(75), A::voltage(ctx, nodes, Some(10), Some(5)), 1.0));

        s.b[170] = (p.p83 > 0.0);
        s.v[170] = if s.b[170] { 1.0 } else { 0.0 };

        if s.b[170] {
            s.store_scaled_voltage(106, ctx, nodes, Some(4), None, 1.0 / (p.p83));
        }

        if (!s.b[170]) {
            s.store_scalar(106, 0.0);
        }

        s.store_scaled_voltage(107, ctx, nodes, Some(4), None, p.p84);

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.v[35] = (273.15 + p.p0);

        s.store_offset_voltage(34, ctx, nodes, Some(4), None, (ctx_temp + p.p105));

        s.store_scale(48, 34, (1.3806503e-23 * 6.241509744511525e18));

        s.store_scale(36, 34, 1.0 / (s.v[35]));

        s.store_offset(37, 34, (-s.v[35]));

        s.store_scale_ad(2, A::powf(s.ad_value(36), p.p90), p.p53);

        s.store_scaled_powf_ad(0, A::mul(A::powf(s.ad_value(36), p.p78), A::exp(A::div_scaled_offset_numerator(s.ad_value(36), (-(-p.p71)), (-p.p71), s.ad_value(48), 1.0))), (1.0 / p.p12), p.p11);

        s.store_scaled_powf_ad(1, A::mul(A::powf(s.ad_value(36), p.p95), A::exp(A::div_scaled_offset_numerator(s.ad_value(36), (-(-p.p96)), (-p.p96), s.ad_value(48), 1.0))), (1.0 / p.p13), p.p94);

        s.store_scaled_powf_ad(5, A::mul(A::powf(s.ad_value(36), p.p78), A::exp(A::div_scaled_offset_numerator(s.ad_value(36), (-(-p.p97)), (-p.p97), s.ad_value(48), 1.0))), (1.0 / p.p44), p.p42);

        s.store_offset_scaled(26, 37, ((p.p81) * (p.p12)), p.p12);

        s.store_offset_scaled(27, 37, ((p.p81) * (p.p13)), p.p13);

        s.store_scaled_offset_ad(29, A::mul(s.ad_value(37), A::scale_offset(s.ad_value(37), p.p102, p.p101)), 1.0, p.p98);

        s.store_offset_scaled(30, 37, ((p.p103) * (p.p99)), p.p99);

        s.store_scaled_mul_ad(108, A::div(s.ad_value(48), s.ad_value(36)), A::ln(A::sub(A::exp(A::div_scaled_inputs(s.ad_value(36), (0.5 * p.p17), s.ad_value(48), 1.0)), A::exp(A::div_scaled_inputs(s.ad_value(36), ((-0.5) * p.p17), s.ad_value(48), 1.0)))), 2.0);

        s.store_sub_ad(109, A::add_scaled_products(s.ad_value(108), s.ad_value(36), 1.0, s.ad_value(48), A::ln(s.ad_value(36)), (-3.0)), A::scaled_offset(s.ad_value(36), (-1.0), p.p72));

        s.store_add_scaled_product(19, s.ad_value(109), 1.0, s.ad_value(48), A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::div_scaled_inputs(s.ad_value(109), -1.0, s.ad_value(48), 1.0)), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scaled_mul_ad(108, A::div(s.ad_value(48), s.ad_value(36)), A::ln(A::sub(A::exp(A::div_scaled_inputs(s.ad_value(36), (0.5 * p.p24), s.ad_value(48), 1.0)), A::exp(A::div_scaled_inputs(s.ad_value(36), ((-0.5) * p.p24), s.ad_value(48), 1.0)))), 2.0);

        s.store_sub_ad(109, A::add_scaled_products(s.ad_value(108), s.ad_value(36), 1.0, s.ad_value(48), A::ln(s.ad_value(36)), (-3.0)), A::scaled_offset(s.ad_value(36), (-1.0), p.p73));

        s.store_add_scaled_product(20, s.ad_value(109), 1.0, s.ad_value(48), A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::div_scaled_inputs(s.ad_value(109), -1.0, s.ad_value(48), 1.0)), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scaled_mul_ad(108, A::div(s.ad_value(48), s.ad_value(36)), A::ln(A::sub(A::exp(A::div_scaled_inputs(s.ad_value(36), (0.5 * p.p28), s.ad_value(48), 1.0)), A::exp(A::div_scaled_inputs(s.ad_value(36), ((-0.5) * p.p28), s.ad_value(48), 1.0)))), 2.0);

        s.store_sub_ad(109, A::add_scaled_products(s.ad_value(108), s.ad_value(36), 1.0, s.ad_value(48), A::ln(s.ad_value(36)), (-3.0)), A::scaled_offset(s.ad_value(36), (-1.0), p.p74));

        s.store_add_scaled_product(21, s.ad_value(109), 1.0, s.ad_value(48), A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::div_scaled_inputs(s.ad_value(109), -1.0, s.ad_value(48), 1.0)), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scaled_powf_ad(22, A::div_from_scalar(p.p17, s.ad_value(19)), p.p18, p.p16);

        s.store_scaled_powf_ad(23, A::div_from_scalar(p.p24, s.ad_value(20)), p.p25, p.p21);

        s.store_scaled_powf_ad(24, A::div_from_scalar(p.p24, s.ad_value(20)), p.p25, p.p23);

        s.store_scaled_powf_ad(25, A::div_from_scalar(p.p28, s.ad_value(21)), p.p29, p.p27);

        s.store_scaled_mul_ad(31, A::powf(s.ad_value(36), p.p78), A::exp(A::div_scaled_offset_numerator(s.ad_value(36), (-(-p.p71)), (-p.p71), s.ad_value(48), 1.0)), p.p4);

        s.v[38] = (if (p.p51 > 0.0) { (1.0 / p.p51) } else { 0.0 });

        s.v[39] = (if (p.p52 > 0.0) { (1.0 / p.p52) } else { 0.0 });

        if (p.p53 > 0.0) {
            s.store_div_from_scalar(40, 1.0, 2);
        } else {
            s.store_scalar(40, 0.0);
        }

        s.v[41] = (if (p.p54 > 0.0) { (1.0 / p.p54) } else { 0.0 });

        s.v[45] = (if (p.p59 > 0.0) { (1.0 / p.p59) } else { 0.0 });

        s.v[46] = (if (p.p60 > 0.0) { (1.0 / p.p60) } else { 0.0 });

        s.b[47] = (!(p.p60 > 0.0));
        s.v[47] = if s.b[47] { 1.0 } else { 0.0 };

        s.store_voltage(132, ctx, nodes, Some(8), Some(9));

        s.store_voltage(133, ctx, nodes, Some(7), Some(9));

        s.store_voltage(134, ctx, nodes, Some(8), Some(6));

        s.store_voltage(135, ctx, nodes, Some(8), Some(5));

        s.store_voltage(136, ctx, nodes, Some(7), Some(10));

        s.store_scale(110, 19, (-p.p14));

        s.b[137] = (p.p19 <= 0.0);
        s.v[137] = if s.b[137] { 1.0 } else { 0.0 };

        if s.b[137] {
            s.store_add(111, 132, 110);
        }

        s.b[138] = (s.v[111] > 0.0);
        s.v[138] = if s.b[138] { 1.0 } else { 0.0 };

        if (s.b[137] && s.b[138]) {
            s.store_scalar(112, (((1.0 - p.p14)) as f64).powf(((-1.0) - p.p18)));
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(19), 1.0, A::scale(s.ad_value(112), ((1.0 - p.p14) * (1.0 - p.p14))), 1.0 / ((1.0 - p.p18))));
            s.store_mul_ad_product_lhs(114, s.ad_value(111), A::offset(A::div_scaled_inputs(s.ad_value(111), (0.5 * p.p18), s.ad_value(19), 1.0), (1.0 - p.p14)), 112);
        }

        if (s.b[137] && (!s.b[138])) {
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(19), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(132), s.ad_value(19))), (1.0 - p.p18)), 1.0 / ((1.0 - p.p18))));
            s.store_scalar(114, 0.0);
        }

        if s.b[137] {
            s.store_add(88, 113, 114);
        }

        if (!s.b[137]) {
            s.store_sqrt_square_offset(115, 110, ((4.0 * p.p19) * p.p19));
            s.store_scaled_add(116, 110, 115, (-0.5));
            s.store_mul_scaled_ad_rhs(117, 19, (-1.0 / ((1.0 - p.p18))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(116), s.ad_value(19))), (1.0 - p.p18)));
            s.store_add(118, 132, 110);
            s.store_sqrt_square_offset(119, 118, ((4.0 * p.p19) * p.p19));
            s.store_add_scaled_inputs3(120, s.ad_value(118), 0.5, s.ad_value(119), (-0.5), s.ad_value(110), -1.0);
            s.store_mul_scaled_ad_rhs(113, 19, (-1.0 / ((1.0 - p.p18))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(120), s.ad_value(19))), (1.0 - p.p18)));
            s.store_sub_ad_lhs(88, A::add_scaled_inputs4(s.ad_value(113), 1.0, s.ad_value(132), (((1.0 - p.p14)) as f64).powf((-p.p18)), s.ad_value(120), ((-1.0) * (((1.0 - p.p14)) as f64).powf((-p.p18))), s.ad_value(116), (((1.0 - p.p14)) as f64).powf((-p.p18))), 117);
        }

        s.store_scale(110, 19, (-p.p14));

        s.b[139] = (p.p19 <= 0.0);
        s.v[139] = if s.b[139] { 1.0 } else { 0.0 };

        if s.b[139] {
            s.store_add(111, 133, 110);
        }

        s.b[140] = (s.v[111] > 0.0);
        s.v[140] = if s.b[140] { 1.0 } else { 0.0 };

        if (s.b[139] && s.b[140]) {
            s.store_scalar(112, (((1.0 - p.p14)) as f64).powf(((-1.0) - p.p18)));
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(19), 1.0, A::scale(s.ad_value(112), ((1.0 - p.p14) * (1.0 - p.p14))), 1.0 / ((1.0 - p.p18))));
            s.store_mul_ad_product_lhs(114, s.ad_value(111), A::offset(A::div_scaled_inputs(s.ad_value(111), (0.5 * p.p18), s.ad_value(19), 1.0), (1.0 - p.p14)), 112);
        }

        if (s.b[139] && (!s.b[140])) {
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(19), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(133), s.ad_value(19))), (1.0 - p.p18)), 1.0 / ((1.0 - p.p18))));
            s.store_scalar(114, 0.0);
        }

        if s.b[139] {
            s.store_add(89, 113, 114);
        }

        if (!s.b[139]) {
            s.store_sqrt_square_offset(115, 110, ((4.0 * p.p19) * p.p19));
            s.store_scaled_add(116, 110, 115, (-0.5));
            s.store_mul_scaled_ad_rhs(117, 19, (-1.0 / ((1.0 - p.p18))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(116), s.ad_value(19))), (1.0 - p.p18)));
            s.store_add(118, 133, 110);
            s.store_sqrt_square_offset(119, 118, ((4.0 * p.p19) * p.p19));
            s.store_add_scaled_inputs3(120, s.ad_value(118), 0.5, s.ad_value(119), (-0.5), s.ad_value(110), -1.0);
            s.store_mul_scaled_ad_rhs(113, 19, (-1.0 / ((1.0 - p.p18))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(120), s.ad_value(19))), (1.0 - p.p18)));
            s.store_sub_ad_lhs(89, A::add_scaled_inputs4(s.ad_value(113), 1.0, s.ad_value(133), (((1.0 - p.p14)) as f64).powf((-p.p18)), s.ad_value(120), ((-1.0) * (((1.0 - p.p14)) as f64).powf((-p.p18))), s.ad_value(116), (((1.0 - p.p14)) as f64).powf((-p.p18))), 117);
        }

        s.store_scale(110, 20, (-p.p14));

        s.b[141] = (p.p26 <= 0.0);
        s.v[141] = if s.b[141] { 1.0 } else { 0.0 };

        if s.b[141] {
            s.store_add(111, 134, 110);
        }

        s.b[142] = (s.v[111] > 0.0);
        s.v[142] = if s.b[142] { 1.0 } else { 0.0 };

        if (s.b[141] && s.b[142]) {
            s.store_scalar(112, (((1.0 - p.p14)) as f64).powf(((-1.0) - p.p25)));
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(20), 1.0, A::scale(s.ad_value(112), ((1.0 - p.p14) * (1.0 - p.p14))), 1.0 / ((1.0 - p.p25))));
            s.store_mul_ad_product_lhs(114, s.ad_value(111), A::offset(A::div_scaled_inputs(s.ad_value(111), (0.5 * p.p25), s.ad_value(20), 1.0), (1.0 - p.p14)), 112);
        }

        s.b[143] = ((p.p85 > 0.0) && (s.v[134] < (-p.p85)));
        s.v[143] = if s.b[143] { 1.0 } else { 0.0 };

        if ((s.b[141] && (!s.b[142])) && s.b[143]) {
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(20), 1.0, A::mul_sub_from_scalar_rhs(A::powf(A::offset(A::div_from_scalar(p.p85, s.ad_value(20)), 1.0), (1.0 - p.p25)), 1.0, A::div_scaled_offset_numerator(s.ad_value(134), (1.0 - p.p25), (p.p85 * (1.0 - p.p25)), A::offset(s.ad_value(20), p.p85), 1.0)), 1.0 / ((1.0 - p.p25))));
        }

        if ((s.b[141] && (!s.b[142])) && (!s.b[143])) {
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(20), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(134), s.ad_value(20))), (1.0 - p.p25)), 1.0 / ((1.0 - p.p25))));
        }

        if (s.b[141] && (!s.b[142])) {
            s.store_scalar(114, 0.0);
        }

        if s.b[141] {
            s.store_add(90, 113, 114);
        }

        s.b[144] = ((p.p85 > 0.0) && (p.p86 > 0.0));
        s.v[144] = if s.b[144] { 1.0 } else { 0.0 };

        if ((!s.b[141]) && s.b[144]) {
            s.store_ad_value(121, A::div_scaled_offset_numerator(s.ad_value(110), 1.0, p.p85, A::sub_from_scalar(p.p85, s.ad_value(110)), 1.0));
            s.store_ad_value(122, A::div_scaled_inputs(s.ad_value(121), 2.0, A::add(A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(121), (-1.0), A::offset(s.ad_value(121), (-1.0))), ((4.0 * p.p26) * p.p26))), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(121), 1.0, A::offset(s.ad_value(121), 1.0)), ((4.0 * p.p86) * p.p86)))), 1.0));
            s.store_scaled_sub_ad_lhs(116, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(122), p.p85, s.ad_value(110)), (-p.p85)), 110, 0.5);
            s.store_ad_value(123, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(20), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(116), s.ad_value(20))), (1.0 - p.p25)), 1.0 / ((1.0 - p.p25))));
            s.store_ad_value(124, A::div_scaled_inputs2(A::scale_offset(s.ad_value(134), 2.0, p.p85), 1.0, s.ad_value(110), 1.0, A::sub_from_scalar(p.p85, s.ad_value(110)), 1.0));
            s.store_ad_value(125, A::div_scaled_inputs(s.ad_value(124), 2.0, A::add(A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(124), (-1.0), A::offset(s.ad_value(124), (-1.0))), ((4.0 * p.p26) * p.p26))), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(124), 1.0, A::offset(s.ad_value(124), 1.0)), ((4.0 * p.p86) * p.p86)))), 1.0));
            s.store_scaled_sub_ad_lhs(120, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(125), p.p85, s.ad_value(110)), (-p.p85)), 110, 0.5);
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(20), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(120), s.ad_value(20))), (1.0 - p.p25)), 1.0 / ((1.0 - p.p25))));
            s.store_scaled_offset(126, 125, 1.0, 0.5);
            s.store_powf_ad(127, A::offset(A::div_from_scalar(p.p85, s.ad_value(20)), 1.0), (-p.p25));
            s.store_powf_ad(128, A::offset(A::div(s.ad_value(110), s.ad_value(20)), 1.0), (-p.p25));
            s.store_add_scaled_product(129, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(126), s.ad_value(127)), 1.0, s.ad_value(126), s.ad_value(128), 1.0);
            s.store_mul_ad_lhs(130, A::add_scaled_inputs3(s.ad_value(134), 1.0, s.ad_value(120), (-1.0), s.ad_value(116), 1.0), 129);
            s.store_add_scaled_inputs3(90, s.ad_value(130), 1.0, s.ad_value(113), 1.0, s.ad_value(123), -1.0);
        }

        if ((!s.b[141]) && (!s.b[144])) {
            s.store_sqrt_square_offset(115, 110, ((4.0 * p.p26) * p.p26));
            s.store_scaled_add(116, 110, 115, (-0.5));
            s.store_mul_scaled_ad_rhs(117, 20, (-1.0 / ((1.0 - p.p25))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(116), s.ad_value(20))), (1.0 - p.p25)));
            s.store_add(118, 134, 110);
            s.store_sqrt_square_offset(119, 118, ((4.0 * p.p26) * p.p26));
            s.store_add_scaled_inputs3(120, s.ad_value(118), 0.5, s.ad_value(119), (-0.5), s.ad_value(110), -1.0);
            s.store_mul_scaled_ad_rhs(113, 20, (-1.0 / ((1.0 - p.p25))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(120), s.ad_value(20))), (1.0 - p.p25)));
            s.store_sub_ad_lhs(90, A::add_scaled_inputs4(s.ad_value(113), 1.0, s.ad_value(134), (((1.0 - p.p14)) as f64).powf((-p.p25)), s.ad_value(120), ((-1.0) * (((1.0 - p.p14)) as f64).powf((-p.p25))), s.ad_value(116), (((1.0 - p.p14)) as f64).powf((-p.p25))), 117);
        }

        s.store_scale(110, 20, (-p.p14));

        s.b[145] = (p.p26 <= 0.0);
        s.v[145] = if s.b[145] { 1.0 } else { 0.0 };

        if s.b[145] {
            s.store_add(111, 136, 110);
        }

        s.b[146] = (s.v[111] > 0.0);
        s.v[146] = if s.b[146] { 1.0 } else { 0.0 };

        if (s.b[145] && s.b[146]) {
            s.store_scalar(112, (((1.0 - p.p14)) as f64).powf(((-1.0) - p.p25)));
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(20), 1.0, A::scale(s.ad_value(112), ((1.0 - p.p14) * (1.0 - p.p14))), 1.0 / ((1.0 - p.p25))));
            s.store_mul_ad_product_lhs(114, s.ad_value(111), A::offset(A::div_scaled_inputs(s.ad_value(111), (0.5 * p.p25), s.ad_value(20), 1.0), (1.0 - p.p14)), 112);
        }

        s.b[147] = ((p.p85 > 0.0) && (s.v[136] < (-p.p85)));
        s.v[147] = if s.b[147] { 1.0 } else { 0.0 };

        if ((s.b[145] && (!s.b[146])) && s.b[147]) {
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(20), 1.0, A::mul_sub_from_scalar_rhs(A::powf(A::offset(A::div_from_scalar(p.p85, s.ad_value(20)), 1.0), (1.0 - p.p25)), 1.0, A::div_scaled_offset_numerator(s.ad_value(136), (1.0 - p.p25), (p.p85 * (1.0 - p.p25)), A::offset(s.ad_value(20), p.p85), 1.0)), 1.0 / ((1.0 - p.p25))));
        }

        if ((s.b[145] && (!s.b[146])) && (!s.b[147])) {
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(20), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(136), s.ad_value(20))), (1.0 - p.p25)), 1.0 / ((1.0 - p.p25))));
        }

        if (s.b[145] && (!s.b[146])) {
            s.store_scalar(114, 0.0);
        }

        if s.b[145] {
            s.store_add(91, 113, 114);
        }

        s.b[148] = ((p.p85 > 0.0) && (p.p86 > 0.0));
        s.v[148] = if s.b[148] { 1.0 } else { 0.0 };

        if ((!s.b[145]) && s.b[148]) {
            s.store_ad_value(121, A::div_scaled_offset_numerator(s.ad_value(110), 1.0, p.p85, A::sub_from_scalar(p.p85, s.ad_value(110)), 1.0));
            s.store_ad_value(122, A::div_scaled_inputs(s.ad_value(121), 2.0, A::add(A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(121), (-1.0), A::offset(s.ad_value(121), (-1.0))), ((4.0 * p.p26) * p.p26))), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(121), 1.0, A::offset(s.ad_value(121), 1.0)), ((4.0 * p.p86) * p.p86)))), 1.0));
            s.store_scaled_sub_ad_lhs(116, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(122), p.p85, s.ad_value(110)), (-p.p85)), 110, 0.5);
            s.store_ad_value(123, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(20), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(116), s.ad_value(20))), (1.0 - p.p25)), 1.0 / ((1.0 - p.p25))));
            s.store_ad_value(124, A::div_scaled_inputs2(A::scale_offset(s.ad_value(136), 2.0, p.p85), 1.0, s.ad_value(110), 1.0, A::sub_from_scalar(p.p85, s.ad_value(110)), 1.0));
            s.store_ad_value(125, A::div_scaled_inputs(s.ad_value(124), 2.0, A::add(A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(124), (-1.0), A::offset(s.ad_value(124), (-1.0))), ((4.0 * p.p26) * p.p26))), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(124), 1.0, A::offset(s.ad_value(124), 1.0)), ((4.0 * p.p86) * p.p86)))), 1.0));
            s.store_scaled_sub_ad_lhs(120, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(125), p.p85, s.ad_value(110)), (-p.p85)), 110, 0.5);
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(20), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(120), s.ad_value(20))), (1.0 - p.p25)), 1.0 / ((1.0 - p.p25))));
            s.store_scaled_offset(126, 125, 1.0, 0.5);
            s.store_powf_ad(127, A::offset(A::div_from_scalar(p.p85, s.ad_value(20)), 1.0), (-p.p25));
            s.store_powf_ad(128, A::offset(A::div(s.ad_value(110), s.ad_value(20)), 1.0), (-p.p25));
            s.store_add_scaled_product(129, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(126), s.ad_value(127)), 1.0, s.ad_value(126), s.ad_value(128), 1.0);
            s.store_mul_ad_lhs(130, A::add_scaled_inputs3(s.ad_value(136), 1.0, s.ad_value(120), (-1.0), s.ad_value(116), 1.0), 129);
            s.store_add_scaled_inputs3(91, s.ad_value(130), 1.0, s.ad_value(113), 1.0, s.ad_value(123), -1.0);
        }

        if ((!s.b[145]) && (!s.b[148])) {
            s.store_sqrt_square_offset(115, 110, ((4.0 * p.p26) * p.p26));
            s.store_scaled_add(116, 110, 115, (-0.5));
            s.store_mul_scaled_ad_rhs(117, 20, (-1.0 / ((1.0 - p.p25))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(116), s.ad_value(20))), (1.0 - p.p25)));
            s.store_add(118, 136, 110);
            s.store_sqrt_square_offset(119, 118, ((4.0 * p.p26) * p.p26));
            s.store_add_scaled_inputs3(120, s.ad_value(118), 0.5, s.ad_value(119), (-0.5), s.ad_value(110), -1.0);
            s.store_mul_scaled_ad_rhs(113, 20, (-1.0 / ((1.0 - p.p25))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(120), s.ad_value(20))), (1.0 - p.p25)));
            s.store_sub_ad_lhs(91, A::add_scaled_inputs4(s.ad_value(113), 1.0, s.ad_value(136), (((1.0 - p.p14)) as f64).powf((-p.p25)), s.ad_value(120), ((-1.0) * (((1.0 - p.p14)) as f64).powf((-p.p25))), s.ad_value(116), (((1.0 - p.p14)) as f64).powf((-p.p25))), 117);
        }

        s.b[149] = (p.p27 > 0.0);
        s.v[149] = if s.b[149] { 1.0 } else { 0.0 };

        if s.b[149] {
            s.store_scale(110, 21, (-p.p14));
        }

        s.b[150] = (p.p30 <= 0.0);
        s.v[150] = if s.b[150] { 1.0 } else { 0.0 };

        if (s.b[149] && s.b[150]) {
            s.store_add_ad_lhs(111, A::voltage(ctx, nodes, Some(11), Some(10)), 110);
        }

        s.b[151] = (s.v[111] > 0.0);
        s.v[151] = if s.b[151] { 1.0 } else { 0.0 };

        if ((s.b[149] && s.b[150]) && s.b[151]) {
            s.store_scalar(112, (((1.0 - p.p14)) as f64).powf(((-1.0) - p.p29)));
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(21), 1.0, A::scale(s.ad_value(112), ((1.0 - p.p14) * (1.0 - p.p14))), 1.0 / ((1.0 - p.p29))));
        }

    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[149] && s.b[150]) && s.b[151]) {
            s.store_mul_ad_product_lhs(114, s.ad_value(111), A::offset(A::div_scaled_inputs(s.ad_value(111), (0.5 * p.p29), s.ad_value(21), 1.0), (1.0 - p.p14)), 112);
        }

        if ((s.b[149] && s.b[150]) && (!s.b[151])) {
            s.store_ad_value(113, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(21), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(A::voltage(ctx, nodes, Some(11), Some(10)), s.ad_value(21))), (1.0 - p.p29)), 1.0 / ((1.0 - p.p29))));
            s.store_scalar(114, 0.0);
        }

        if (s.b[149] && s.b[150]) {
            s.store_add(92, 113, 114);
        }

        if (s.b[149] && (!s.b[150])) {
            s.store_sqrt_square_offset(115, 110, ((4.0 * p.p30) * p.p30));
            s.store_scaled_add(116, 110, 115, (-0.5));
            s.store_mul_scaled_ad_rhs(117, 21, (-1.0 / ((1.0 - p.p29))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(116), s.ad_value(21))), (1.0 - p.p29)));
            s.store_add_ad_lhs(118, A::voltage(ctx, nodes, Some(11), Some(10)), 110);
            s.store_sqrt_square_offset(119, 118, ((4.0 * p.p30) * p.p30));
            s.store_add_scaled_inputs3(120, s.ad_value(118), 0.5, s.ad_value(119), (-0.5), s.ad_value(110), -1.0);
            s.store_mul_scaled_ad_rhs(113, 21, (-1.0 / ((1.0 - p.p29))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(120), s.ad_value(21))), (1.0 - p.p29)));
            s.store_sub_ad_lhs(92, A::add_scaled_inputs4(s.ad_value(113), 1.0, A::voltage(ctx, nodes, Some(11), Some(10)), (((1.0 - p.p14)) as f64).powf((-p.p29)), s.ad_value(120), ((-1.0) * (((1.0 - p.p14)) as f64).powf((-p.p29))), s.ad_value(116), (((1.0 - p.p14)) as f64).powf((-p.p29))), 117);
        }

        if (!s.b[149]) {
            s.store_scalar(92, 0.0);
        }

        s.store_div_ad_rhs(82, 132, A::mul(s.ad_value(26), s.ad_value(48)));

        s.store_limexp(83, 82);

        s.store_mul_offset_rhs(49, 0, 83, (-1.0));

        s.store_div_ad_rhs(82, 134, A::mul(s.ad_value(27), s.ad_value(48)));

        s.store_limexp(83, 82);

        s.store_mul_ad_product_rhs(50, 0, s.ad_value(1), A::offset(s.ad_value(83), (-1.0)));

        s.store_add_scaled_ad_lhs(53, A::scale_offset(s.ad_value(88), s.v[39], 1.0), 90, s.v[38]);

        s.store_offset_scaled_ad(54, A::add(A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(53), (-0.0001), A::offset(s.ad_value(53), (-0.0001))), 1e-8)), s.ad_value(53)), 0.5, (((((-0.0001)) * (0.5))) + (0.0001)));

        s.store_add_scaled_product(55, s.ad_value(50), s.v[41], s.ad_value(49), s.ad_value(40), 1.0);

        s.b[152] = (p.p88 < 0.5);
        s.v[152] = if s.b[152] { 1.0 } else { 0.0 };

        if s.b[152] {
            s.store_scaled_add_ad_rhs(56, 54, A::powf(A::add_scaled_inputs(A::powf(s.ad_value(54), (1.0 / p.p89)), 1.0, s.ad_value(55), 4.0), p.p89), 0.5);
        }

        if (!s.b[152]) {
            s.store_mul_scaled_ad_rhs(56, 54, 0.5, A::offset(A::powf(A::scale_offset(s.ad_value(55), 4.0, 1.0), p.p89), 1.0));
        }

        s.b[153] = (p.p42 > 0.0);
        s.v[153] = if s.b[153] { 1.0 } else { 0.0 };

        if s.b[153] {
            s.store_scaled_div(82, 136, 48, (1.0 / (p.p44)));
            s.store_limexp(83, 82);
            s.store_scaled_div(86, 134, 48, (1.0 / (p.p44)));
            s.store_limexp(87, 86);
            s.store_mul_offset_ad_rhs(57, 5, A::add_scaled_inputs(s.ad_value(83), p.p43, s.ad_value(87), (1.0 - p.p43)), (-1.0));
            s.store_ad_value(82, A::div_scaled_inputs(A::voltage(ctx, nodes, Some(11), Some(10)), 1.0, s.ad_value(48), p.p44));
            s.store_limexp(83, 82);
        }

        if (!s.b[153]) {
            s.store_scalar(57, 0.0);
        }

        s.b[154] = (p.p32 == 1.0);
        s.v[154] = if s.b[154] { 1.0 } else { 0.0 };

        if s.b[154] {
            s.store_scaled_div(82, 132, 48, (1.0 / (p.p33)));
            s.store_limexp(83, 82);
        }

        s.b[155] = (p.p98 > 0.0);
        s.v[155] = if s.b[155] { 1.0 } else { 0.0 };

        if (s.b[154] && s.b[155]) {
            s.store_ad_value(86, A::div_scaled_inputs2(s.ad_value(29), -1.0, s.ad_value(132), (-1.0), A::mul(s.ad_value(30), s.ad_value(48)), 1.0));
            s.store_limexp(87, 86);
        }

        s.b[156] = (p.p32 == 0.0);
        s.v[156] = if s.b[156] { 1.0 } else { 0.0 };

        if ((!s.b[154]) && s.b[156]) {
            s.store_scaled_div(82, 133, 48, (1.0 / (p.p33)));
            s.store_limexp(83, 82);
        }

        s.b[157] = (p.p98 > 0.0);
        s.v[157] = if s.b[157] { 1.0 } else { 0.0 };

        if (((!s.b[154]) && s.b[156]) && s.b[157]) {
            s.store_ad_value(86, A::div_scaled_inputs2(s.ad_value(29), -1.0, s.ad_value(133), (-1.0), A::mul(s.ad_value(30), s.ad_value(48)), 1.0));
            s.store_limexp(87, 86);
        }

        if ((!s.b[154]) && (!s.b[156])) {
            s.store_scaled_div(82, 132, 48, (1.0 / (p.p33)));
            s.store_limexp(83, 82);
        }

        s.b[158] = (p.p98 > 0.0);
        s.v[158] = if s.b[158] { 1.0 } else { 0.0 };

        if (((!s.b[154]) && (!s.b[156])) && s.b[158]) {
            s.store_ad_value(86, A::div_scaled_inputs2(s.ad_value(29), -1.0, s.ad_value(132), (-1.0), A::mul(s.ad_value(30), s.ad_value(48)), 1.0));
            s.store_limexp(87, 86);
        }

        if ((!s.b[154]) && (!s.b[156])) {
            s.store_scaled_div(82, 133, 48, (1.0 / (p.p33)));
            s.store_limexp(83, 82);
        }

        s.b[159] = (p.p98 > 0.0);
        s.v[159] = if s.b[159] { 1.0 } else { 0.0 };

        if (((!s.b[154]) && (!s.b[156])) && s.b[159]) {
            s.store_ad_value(86, A::div_scaled_inputs2(s.ad_value(29), -1.0, s.ad_value(133), (-1.0), A::mul(s.ad_value(30), s.ad_value(48)), 1.0));
            s.store_limexp(87, 86);
        }

        s.store_scaled_div(82, 134, 48, (1.0 / (p.p37)));

        s.store_limexp(83, 82);

        s.b[160] = ((p.p45 > 0.0) || (p.p46 > 0.0));
        s.v[160] = if s.b[160] { 1.0 } else { 0.0 };

        if s.b[160] {
            s.store_scaled_div(82, 136, 48, (1.0 / (p.p37)));
            s.store_limexp(83, 82);
        }

        s.b[161] = (p.p40 > 0.0);
        s.v[161] = if s.b[161] { 1.0 } else { 0.0 };

        if s.b[161] {
            s.store_add_scaled_inputs3(120, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(20), s.ad_value(134)), A::sub(s.ad_value(20), s.ad_value(134))), 0.01)), 0.5, s.ad_value(20), 0.5, s.ad_value(134), ((-1.0) * 0.5));
        }

        s.store_div(82, 134, 48);

        s.store_limexp(83, 82);

        s.store_div(86, 135, 48);

        s.store_limexp(87, 86);

        s.store_sqrt_offset_ad(77, A::mul(s.ad_value(31), s.ad_value(83)), 1.0);

        s.store_sqrt_offset_ad(78, A::mul(s.ad_value(31), s.ad_value(87)), 1.0);

        s.b[168] = ((p.p47 > 0.0) || (p.p49 > 0.0));
        s.v[168] = if s.b[168] { 1.0 } else { 0.0 };

        if s.b[168] {
            s.store_ad_value(82, A::div_scaled_inputs(A::voltage(ctx, nodes, Some(11), Some(10)), 1.0, s.ad_value(48), p.p48));
            s.store_limexp(83, 82);
        }

        s.b[93] = (s.v[49] > 0.0);
        s.v[93] = if s.b[93] { 1.0 } else { 0.0 };

        s.store_scale(94, 49, (s.v[93] * s.v[46]));

        s.store_ad_value(95, A::div_scaled_value_offset_denominator(s.ad_value(94), 1.0, s.ad_value(94), 1.0, 1.0));

        s.store_scaled_mul_ad(96, A::scale_offset(s.ad_value(54), p.p57, 1.0), A::offset(A::mul_offset_rhs_scaled_output(A::limexp_scaled_input(s.ad_value(134), (s.v[45] * 1.0 / (1.44))), A::square(s.ad_value(95)), s.v[47], (p.p58 * s.v[93])), 1.0), p.p56);

        s.store_add_scaled_product(97, A::div_scaled_product(s.ad_value(96), s.ad_value(49), 1.0, s.ad_value(56), 1.0), 1.0, s.ad_value(22), s.ad_value(88), p.p32);

        s.store_scaled_mul(98, 22, 89, (1.0 - p.p32));

        s.store_add_scaled_ad_lhs(99, A::add_scaled_product(s.ad_value(50), p.p61, s.ad_value(23), s.ad_value(90), 1.0), 77, p.p22);

        s.store_scale(100, 78, p.p22);

        s.store_add_scaled_product(101, s.ad_value(57), p.p61, s.ad_value(24), s.ad_value(91), 1.0);

        s.store_add_scaled_product(102, A::voltage(ctx, nodes, Some(11), Some(10)), p.p87, s.ad_value(25), s.ad_value(92), 1.0);

        s.store_scaled_voltage(103, ctx, nodes, Some(1), Some(2), p.p15);

        s.store_scaled_voltage(104, ctx, nodes, Some(1), Some(0), p.p20);

        s.store_scaled_voltage(107, ctx, nodes, Some(4), None, p.p84);

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
        let eq0_value: f64 = s.v[62];
        let eq0_node_derivatives: [f64; 12] = [s.dn[62][0], s.dn[62][1], s.dn[62][2], s.dn[62][3], s.dn[62][4], s.dn[62][5], s.dn[62][6], s.dn[62][7], s.dn[62][8], s.dn[62][9], s.dn[62][10], s.dn[62][11]];
        let eq0_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let eq1_value: f64 = s.v[63];
        let eq1_node_derivatives: [f64; 12] = [s.dn[63][0], s.dn[63][1], s.dn[63][2], s.dn[63][3], s.dn[63][4], s.dn[63][5], s.dn[63][6], s.dn[63][7], s.dn[63][8], s.dn[63][9], s.dn[63][10], s.dn[63][11]];
        let eq1_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let eq2_value: f64 = s.v[51];
        let eq2_node_derivatives: [f64; 12] = [s.dn[51][0], s.dn[51][1], s.dn[51][2], s.dn[51][3], s.dn[51][4], s.dn[51][5], s.dn[51][6], s.dn[51][7], s.dn[51][8], s.dn[51][9], s.dn[51][10], s.dn[51][11]];
        let eq2_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(9),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let eq3_value: f64 = s.v[52];
        let eq3_node_derivatives: [f64; 12] = [s.dn[52][0], s.dn[52][1], s.dn[52][2], s.dn[52][3], s.dn[52][4], s.dn[52][5], s.dn[52][6], s.dn[52][7], s.dn[52][8], s.dn[52][9], s.dn[52][10], s.dn[52][11]];
        let eq3_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let eq4_value: f64 = s.v[65];
        let eq4_node_derivatives: [f64; 12] = [s.dn[65][0], s.dn[65][1], s.dn[65][2], s.dn[65][3], s.dn[65][4], s.dn[65][5], s.dn[65][6], s.dn[65][7], s.dn[65][8], s.dn[65][9], s.dn[65][10], s.dn[65][11]];
        let eq4_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let eq5_value: f64 = s.v[66];
        let eq5_node_derivatives: [f64; 12] = [s.dn[66][0], s.dn[66][1], s.dn[66][2], s.dn[66][3], s.dn[66][4], s.dn[66][5], s.dn[66][6], s.dn[66][7], s.dn[66][8], s.dn[66][9], s.dn[66][10], s.dn[66][11]];
        let eq5_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(10),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let eq6_value: f64 = s.v[70];
        let eq6_node_derivatives: [f64; 12] = [s.dn[70][0], s.dn[70][1], s.dn[70][2], s.dn[70][3], s.dn[70][4], s.dn[70][5], s.dn[70][6], s.dn[70][7], s.dn[70][8], s.dn[70][9], s.dn[70][10], s.dn[70][11]];
        let eq6_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(5),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let eq7_value: f64 = s.v[71];
        let eq7_node_derivatives: [f64; 12] = [s.dn[71][0], s.dn[71][1], s.dn[71][2], s.dn[71][3], s.dn[71][4], s.dn[71][5], s.dn[71][6], s.dn[71][7], s.dn[71][8], s.dn[71][9], s.dn[71][10], s.dn[71][11]];
        let eq7_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let eq8_value: f64 = s.v[72];
        let eq8_node_derivatives: [f64; 12] = [s.dn[72][0], s.dn[72][1], s.dn[72][2], s.dn[72][3], s.dn[72][4], s.dn[72][5], s.dn[72][6], s.dn[72][7], s.dn[72][8], s.dn[72][9], s.dn[72][10], s.dn[72][11]];
        let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(7),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let eq9_value: f64 = s.v[73];
        let eq9_node_derivatives: [f64; 12] = [s.dn[73][0], s.dn[73][1], s.dn[73][2], s.dn[73][3], s.dn[73][4], s.dn[73][5], s.dn[73][6], s.dn[73][7], s.dn[73][8], s.dn[73][9], s.dn[73][10], s.dn[73][11]];
        let eq9_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_value: f64 = s.v[74];
        let eq10_node_derivatives: [f64; 12] = [s.dn[74][0], s.dn[74][1], s.dn[74][2], s.dn[74][3], s.dn[74][4], s.dn[74][5], s.dn[74][6], s.dn[74][7], s.dn[74][8], s.dn[74][9], s.dn[74][10], s.dn[74][11]];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(9),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_value: f64 = s.v[75];
        let eq11_node_derivatives: [f64; 12] = [s.dn[75][0], s.dn[75][1], s.dn[75][2], s.dn[75][3], s.dn[75][4], s.dn[75][5], s.dn[75][6], s.dn[75][7], s.dn[75][8], s.dn[75][9], s.dn[75][10], s.dn[75][11]];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(5),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e124: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, s.v[97]);
        let eq12_e124_d_n0: f64 = (s.dn[97][0] * ddt_scale);
        let eq12_e124_d_n1: f64 = (s.dn[97][1] * ddt_scale);
        let eq12_e124_d_n2: f64 = (s.dn[97][2] * ddt_scale);
        let eq12_e124_d_n3: f64 = (s.dn[97][3] * ddt_scale);
        let eq12_e124_d_n4: f64 = (s.dn[97][4] * ddt_scale);
        let eq12_e124_d_n5: f64 = (s.dn[97][5] * ddt_scale);
        let eq12_e124_d_n6: f64 = (s.dn[97][6] * ddt_scale);
        let eq12_e124_d_n7: f64 = (s.dn[97][7] * ddt_scale);
        let eq12_e124_d_n8: f64 = (s.dn[97][8] * ddt_scale);
        let eq12_e124_d_n9: f64 = (s.dn[97][9] * ddt_scale);
        let eq12_e124_d_n10: f64 = (s.dn[97][10] * ddt_scale);
        let eq12_e124_d_n11: f64 = (s.dn[97][11] * ddt_scale);
        let eq12_value: f64 = eq12_e124;
        let eq12_node_derivatives: [f64; 12] = [eq12_e124_d_n0, eq12_e124_d_n1, eq12_e124_d_n2, eq12_e124_d_n3, eq12_e124_d_n4, eq12_e124_d_n5, eq12_e124_d_n6, eq12_e124_d_n7, eq12_e124_d_n8, eq12_e124_d_n9, eq12_e124_d_n10, eq12_e124_d_n11];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e126: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, s.v[98]);
        let eq13_e126_d_n0: f64 = (s.dn[98][0] * ddt_scale);
        let eq13_e126_d_n1: f64 = (s.dn[98][1] * ddt_scale);
        let eq13_e126_d_n2: f64 = (s.dn[98][2] * ddt_scale);
        let eq13_e126_d_n3: f64 = (s.dn[98][3] * ddt_scale);
        let eq13_e126_d_n4: f64 = (s.dn[98][4] * ddt_scale);
        let eq13_e126_d_n5: f64 = (s.dn[98][5] * ddt_scale);
        let eq13_e126_d_n6: f64 = (s.dn[98][6] * ddt_scale);
        let eq13_e126_d_n7: f64 = (s.dn[98][7] * ddt_scale);
        let eq13_e126_d_n8: f64 = (s.dn[98][8] * ddt_scale);
        let eq13_e126_d_n9: f64 = (s.dn[98][9] * ddt_scale);
        let eq13_e126_d_n10: f64 = (s.dn[98][10] * ddt_scale);
        let eq13_e126_d_n11: f64 = (s.dn[98][11] * ddt_scale);
        let eq13_value: f64 = eq13_e126;
        let eq13_node_derivatives: [f64; 12] = [eq13_e126_d_n0, eq13_e126_d_n1, eq13_e126_d_n2, eq13_e126_d_n3, eq13_e126_d_n4, eq13_e126_d_n5, eq13_e126_d_n6, eq13_e126_d_n7, eq13_e126_d_n8, eq13_e126_d_n9, eq13_e126_d_n10, eq13_e126_d_n11];
        let eq13_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_e128: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, s.v[99]);
        let eq14_e128_d_n0: f64 = (s.dn[99][0] * ddt_scale);
        let eq14_e128_d_n1: f64 = (s.dn[99][1] * ddt_scale);
        let eq14_e128_d_n2: f64 = (s.dn[99][2] * ddt_scale);
        let eq14_e128_d_n3: f64 = (s.dn[99][3] * ddt_scale);
        let eq14_e128_d_n4: f64 = (s.dn[99][4] * ddt_scale);
        let eq14_e128_d_n5: f64 = (s.dn[99][5] * ddt_scale);
        let eq14_e128_d_n6: f64 = (s.dn[99][6] * ddt_scale);
        let eq14_e128_d_n7: f64 = (s.dn[99][7] * ddt_scale);
        let eq14_e128_d_n8: f64 = (s.dn[99][8] * ddt_scale);
        let eq14_e128_d_n9: f64 = (s.dn[99][9] * ddt_scale);
        let eq14_e128_d_n10: f64 = (s.dn[99][10] * ddt_scale);
        let eq14_e128_d_n11: f64 = (s.dn[99][11] * ddt_scale);
        let eq14_value: f64 = eq14_e128;
        let eq14_node_derivatives: [f64; 12] = [eq14_e128_d_n0, eq14_e128_d_n1, eq14_e128_d_n2, eq14_e128_d_n3, eq14_e128_d_n4, eq14_e128_d_n5, eq14_e128_d_n6, eq14_e128_d_n7, eq14_e128_d_n8, eq14_e128_d_n9, eq14_e128_d_n10, eq14_e128_d_n11];
        let eq14_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let eq15_e130: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, s.v[100]);
        let eq15_e130_d_n0: f64 = (s.dn[100][0] * ddt_scale);
        let eq15_e130_d_n1: f64 = (s.dn[100][1] * ddt_scale);
        let eq15_e130_d_n2: f64 = (s.dn[100][2] * ddt_scale);
        let eq15_e130_d_n3: f64 = (s.dn[100][3] * ddt_scale);
        let eq15_e130_d_n4: f64 = (s.dn[100][4] * ddt_scale);
        let eq15_e130_d_n5: f64 = (s.dn[100][5] * ddt_scale);
        let eq15_e130_d_n6: f64 = (s.dn[100][6] * ddt_scale);
        let eq15_e130_d_n7: f64 = (s.dn[100][7] * ddt_scale);
        let eq15_e130_d_n8: f64 = (s.dn[100][8] * ddt_scale);
        let eq15_e130_d_n9: f64 = (s.dn[100][9] * ddt_scale);
        let eq15_e130_d_n10: f64 = (s.dn[100][10] * ddt_scale);
        let eq15_e130_d_n11: f64 = (s.dn[100][11] * ddt_scale);
        let eq15_value: f64 = eq15_e130;
        let eq15_node_derivatives: [f64; 12] = [eq15_e130_d_n0, eq15_e130_d_n1, eq15_e130_d_n2, eq15_e130_d_n3, eq15_e130_d_n4, eq15_e130_d_n5, eq15_e130_d_n6, eq15_e130_d_n7, eq15_e130_d_n8, eq15_e130_d_n9, eq15_e130_d_n10, eq15_e130_d_n11];
        let eq15_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let eq16_e132: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, s.v[101]);
        let eq16_e132_d_n0: f64 = (s.dn[101][0] * ddt_scale);
        let eq16_e132_d_n1: f64 = (s.dn[101][1] * ddt_scale);
        let eq16_e132_d_n2: f64 = (s.dn[101][2] * ddt_scale);
        let eq16_e132_d_n3: f64 = (s.dn[101][3] * ddt_scale);
        let eq16_e132_d_n4: f64 = (s.dn[101][4] * ddt_scale);
        let eq16_e132_d_n5: f64 = (s.dn[101][5] * ddt_scale);
        let eq16_e132_d_n6: f64 = (s.dn[101][6] * ddt_scale);
        let eq16_e132_d_n7: f64 = (s.dn[101][7] * ddt_scale);
        let eq16_e132_d_n8: f64 = (s.dn[101][8] * ddt_scale);
        let eq16_e132_d_n9: f64 = (s.dn[101][9] * ddt_scale);
        let eq16_e132_d_n10: f64 = (s.dn[101][10] * ddt_scale);
        let eq16_e132_d_n11: f64 = (s.dn[101][11] * ddt_scale);
        let eq16_value: f64 = eq16_e132;
        let eq16_node_derivatives: [f64; 12] = [eq16_e132_d_n0, eq16_e132_d_n1, eq16_e132_d_n2, eq16_e132_d_n3, eq16_e132_d_n4, eq16_e132_d_n5, eq16_e132_d_n6, eq16_e132_d_n7, eq16_e132_d_n8, eq16_e132_d_n9, eq16_e132_d_n10, eq16_e132_d_n11];
        let eq16_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(10),
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let eq17_e134: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, s.v[103]);
        let eq17_e134_d_n0: f64 = (s.dn[103][0] * ddt_scale);
        let eq17_e134_d_n1: f64 = (s.dn[103][1] * ddt_scale);
        let eq17_e134_d_n2: f64 = (s.dn[103][2] * ddt_scale);
        let eq17_e134_d_n3: f64 = (s.dn[103][3] * ddt_scale);
        let eq17_e134_d_n4: f64 = (s.dn[103][4] * ddt_scale);
        let eq17_e134_d_n5: f64 = (s.dn[103][5] * ddt_scale);
        let eq17_e134_d_n6: f64 = (s.dn[103][6] * ddt_scale);
        let eq17_e134_d_n7: f64 = (s.dn[103][7] * ddt_scale);
        let eq17_e134_d_n8: f64 = (s.dn[103][8] * ddt_scale);
        let eq17_e134_d_n9: f64 = (s.dn[103][9] * ddt_scale);
        let eq17_e134_d_n10: f64 = (s.dn[103][10] * ddt_scale);
        let eq17_e134_d_n11: f64 = (s.dn[103][11] * ddt_scale);
        let eq17_value: f64 = eq17_e134;
        let eq17_node_derivatives: [f64; 12] = [eq17_e134_d_n0, eq17_e134_d_n1, eq17_e134_d_n2, eq17_e134_d_n3, eq17_e134_d_n4, eq17_e134_d_n5, eq17_e134_d_n6, eq17_e134_d_n7, eq17_e134_d_n8, eq17_e134_d_n9, eq17_e134_d_n10, eq17_e134_d_n11];
        let eq17_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(2),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let eq18_e136: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, s.v[104]);
        let eq18_e136_d_n0: f64 = (s.dn[104][0] * ddt_scale);
        let eq18_e136_d_n1: f64 = (s.dn[104][1] * ddt_scale);
        let eq18_e136_d_n2: f64 = (s.dn[104][2] * ddt_scale);
        let eq18_e136_d_n3: f64 = (s.dn[104][3] * ddt_scale);
        let eq18_e136_d_n4: f64 = (s.dn[104][4] * ddt_scale);
        let eq18_e136_d_n5: f64 = (s.dn[104][5] * ddt_scale);
        let eq18_e136_d_n6: f64 = (s.dn[104][6] * ddt_scale);
        let eq18_e136_d_n7: f64 = (s.dn[104][7] * ddt_scale);
        let eq18_e136_d_n8: f64 = (s.dn[104][8] * ddt_scale);
        let eq18_e136_d_n9: f64 = (s.dn[104][9] * ddt_scale);
        let eq18_e136_d_n10: f64 = (s.dn[104][10] * ddt_scale);
        let eq18_e136_d_n11: f64 = (s.dn[104][11] * ddt_scale);
        let eq18_value: f64 = eq18_e136;
        let eq18_node_derivatives: [f64; 12] = [eq18_e136_d_n0, eq18_e136_d_n1, eq18_e136_d_n2, eq18_e136_d_n3, eq18_e136_d_n4, eq18_e136_d_n5, eq18_e136_d_n6, eq18_e136_d_n7, eq18_e136_d_n8, eq18_e136_d_n9, eq18_e136_d_n10, eq18_e136_d_n11];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(0),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let eq19_value: f64 = s.v[67];
        let eq19_node_derivatives: [f64; 12] = [s.dn[67][0], s.dn[67][1], s.dn[67][2], s.dn[67][3], s.dn[67][4], s.dn[67][5], s.dn[67][6], s.dn[67][7], s.dn[67][8], s.dn[67][9], s.dn[67][10], s.dn[67][11]];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(10),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let eq20_value: f64 = s.v[59];
        let eq20_node_derivatives: [f64; 12] = [s.dn[59][0], s.dn[59][1], s.dn[59][2], s.dn[59][3], s.dn[59][4], s.dn[59][5], s.dn[59][6], s.dn[59][7], s.dn[59][8], s.dn[59][9], s.dn[59][10], s.dn[59][11]];
        let eq20_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(11),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let eq21_value: f64 = s.v[76];
        let eq21_node_derivatives: [f64; 12] = [s.dn[76][0], s.dn[76][1], s.dn[76][2], s.dn[76][3], s.dn[76][4], s.dn[76][5], s.dn[76][6], s.dn[76][7], s.dn[76][8], s.dn[76][9], s.dn[76][10], s.dn[76][11]];
        let eq21_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(11),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let eq22_e141: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, s.v[102]);
        let eq22_e141_d_n0: f64 = (s.dn[102][0] * ddt_scale);
        let eq22_e141_d_n1: f64 = (s.dn[102][1] * ddt_scale);
        let eq22_e141_d_n2: f64 = (s.dn[102][2] * ddt_scale);
        let eq22_e141_d_n3: f64 = (s.dn[102][3] * ddt_scale);
        let eq22_e141_d_n4: f64 = (s.dn[102][4] * ddt_scale);
        let eq22_e141_d_n5: f64 = (s.dn[102][5] * ddt_scale);
        let eq22_e141_d_n6: f64 = (s.dn[102][6] * ddt_scale);
        let eq22_e141_d_n7: f64 = (s.dn[102][7] * ddt_scale);
        let eq22_e141_d_n8: f64 = (s.dn[102][8] * ddt_scale);
        let eq22_e141_d_n9: f64 = (s.dn[102][9] * ddt_scale);
        let eq22_e141_d_n10: f64 = (s.dn[102][10] * ddt_scale);
        let eq22_e141_d_n11: f64 = (s.dn[102][11] * ddt_scale);
        let eq22_value: f64 = eq22_e141;
        let eq22_node_derivatives: [f64; 12] = [eq22_e141_d_n0, eq22_e141_d_n1, eq22_e141_d_n2, eq22_e141_d_n3, eq22_e141_d_n4, eq22_e141_d_n5, eq22_e141_d_n6, eq22_e141_d_n7, eq22_e141_d_n8, eq22_e141_d_n9, eq22_e141_d_n10, eq22_e141_d_n11];
        let eq22_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(10),
            multiplicity * (eq22_value),
            &eq22_node_derivatives,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let eq23_value: f64 = s.v[106];
        let eq23_node_derivatives: [f64; 12] = [s.dn[106][0], s.dn[106][1], s.dn[106][2], s.dn[106][3], s.dn[106][4], s.dn[106][5], s.dn[106][6], s.dn[106][7], s.dn[106][8], s.dn[106][9], s.dn[106][10], s.dn[106][11]];
        let eq23_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let eq24_value: f64 = s.v[105];
        let eq24_node_derivatives: [f64; 12] = [s.dn[105][0], s.dn[105][1], s.dn[105][2], s.dn[105][3], s.dn[105][4], s.dn[105][5], s.dn[105][6], s.dn[105][7], s.dn[105][8], s.dn[105][9], s.dn[105][10], s.dn[105][11]];
        let eq24_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let eq25_e145: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, s.v[107]);
        let eq25_e145_d_n0: f64 = (s.dn[107][0] * ddt_scale);
        let eq25_e145_d_n1: f64 = (s.dn[107][1] * ddt_scale);
        let eq25_e145_d_n2: f64 = (s.dn[107][2] * ddt_scale);
        let eq25_e145_d_n3: f64 = (s.dn[107][3] * ddt_scale);
        let eq25_e145_d_n4: f64 = (s.dn[107][4] * ddt_scale);
        let eq25_e145_d_n5: f64 = (s.dn[107][5] * ddt_scale);
        let eq25_e145_d_n6: f64 = (s.dn[107][6] * ddt_scale);
        let eq25_e145_d_n7: f64 = (s.dn[107][7] * ddt_scale);
        let eq25_e145_d_n8: f64 = (s.dn[107][8] * ddt_scale);
        let eq25_e145_d_n9: f64 = (s.dn[107][9] * ddt_scale);
        let eq25_e145_d_n10: f64 = (s.dn[107][10] * ddt_scale);
        let eq25_e145_d_n11: f64 = (s.dn[107][11] * ddt_scale);
        let eq25_value: f64 = eq25_e145;
        let eq25_node_derivatives: [f64; 12] = [eq25_e145_d_n0, eq25_e145_d_n1, eq25_e145_d_n2, eq25_e145_d_n3, eq25_e145_d_n4, eq25_e145_d_n5, eq25_e145_d_n6, eq25_e145_d_n7, eq25_e145_d_n8, eq25_e145_d_n9, eq25_e145_d_n10, eq25_e145_d_n11];
        let eq25_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense_local(
            Some(4),
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
        let eq12_e124_q: f64 = s.v[97];
        let eq12_reactive_node_derivatives: [f64; 12] = [s.dn[97][0], s.dn[97][1], s.dn[97][2], s.dn[97][3], s.dn[97][4], s.dn[97][5], s.dn[97][6], s.dn[97][7], s.dn[97][8], s.dn[97][9], s.dn[97][10], s.dn[97][11]];
        let eq12_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[9]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq13_e126_q: f64 = s.v[98];
        let eq13_reactive_node_derivatives: [f64; 12] = [s.dn[98][0], s.dn[98][1], s.dn[98][2], s.dn[98][3], s.dn[98][4], s.dn[98][5], s.dn[98][6], s.dn[98][7], s.dn[98][8], s.dn[98][9], s.dn[98][10], s.dn[98][11]];
        let eq13_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let eq14_e128_q: f64 = s.v[99];
        let eq14_reactive_node_derivatives: [f64; 12] = [s.dn[99][0], s.dn[99][1], s.dn[99][2], s.dn[99][3], s.dn[99][4], s.dn[99][5], s.dn[99][6], s.dn[99][7], s.dn[99][8], s.dn[99][9], s.dn[99][10], s.dn[99][11]];
        let eq14_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
        let eq15_e130_q: f64 = s.v[100];
        let eq15_reactive_node_derivatives: [f64; 12] = [s.dn[100][0], s.dn[100][1], s.dn[100][2], s.dn[100][3], s.dn[100][4], s.dn[100][5], s.dn[100][6], s.dn[100][7], s.dn[100][8], s.dn[100][9], s.dn[100][10], s.dn[100][11]];
        let eq15_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq16_e132_q: f64 = s.v[101];
        let eq16_reactive_node_derivatives: [f64; 12] = [s.dn[101][0], s.dn[101][1], s.dn[101][2], s.dn[101][3], s.dn[101][4], s.dn[101][5], s.dn[101][6], s.dn[101][7], s.dn[101][8], s.dn[101][9], s.dn[101][10], s.dn[101][11]];
        let eq16_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            nodes,
            &eq16_reactive_node_derivatives,
            branches,
            &eq16_reactive_branch_derivatives,
            multiplicity,
        );
        let eq17_e134_q: f64 = s.v[103];
        let eq17_reactive_node_derivatives: [f64; 12] = [s.dn[103][0], s.dn[103][1], s.dn[103][2], s.dn[103][3], s.dn[103][4], s.dn[103][5], s.dn[103][6], s.dn[103][7], s.dn[103][8], s.dn[103][9], s.dn[103][10], s.dn[103][11]];
        let eq17_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes,
            &eq17_reactive_node_derivatives,
            branches,
            &eq17_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e136_q: f64 = s.v[104];
        let eq18_reactive_node_derivatives: [f64; 12] = [s.dn[104][0], s.dn[104][1], s.dn[104][2], s.dn[104][3], s.dn[104][4], s.dn[104][5], s.dn[104][6], s.dn[104][7], s.dn[104][8], s.dn[104][9], s.dn[104][10], s.dn[104][11]];
        let eq18_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq22_e141_q: f64 = s.v[102];
        let eq22_reactive_node_derivatives: [f64; 12] = [s.dn[102][0], s.dn[102][1], s.dn[102][2], s.dn[102][3], s.dn[102][4], s.dn[102][5], s.dn[102][6], s.dn[102][7], s.dn[102][8], s.dn[102][9], s.dn[102][10], s.dn[102][11]];
        let eq22_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[10]),
            nodes,
            &eq22_reactive_node_derivatives,
            branches,
            &eq22_reactive_branch_derivatives,
            multiplicity,
        );
        let eq25_e145_q: f64 = s.v[107];
        let eq25_reactive_node_derivatives: [f64; 12] = [s.dn[107][0], s.dn[107][1], s.dn[107][2], s.dn[107][3], s.dn[107][4], s.dn[107][5], s.dn[107][6], s.dn[107][7], s.dn[107][8], s.dn[107][9], s.dn[107][10], s.dn[107][11]];
        let eq25_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq25_reactive_node_derivatives,
            branches,
            &eq25_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
