#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[471] && (!s.b[472])) {
            s.store_scalar(94, 1.0);
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

    }

    pub(super) fn stamp_reactive_block_7(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
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

        s.copy_ad(242, 181);

        s.b[507] = (s.v[234] != 0.0);
        s.v[507] = if s.b[507] { 1.0 } else { 0.0 };

        if s.b[507] {
            s.store_voltage(504, ctx, nodes, Some(10), None);
            s.store_voltage(505, ctx, nodes, Some(11), None);
            s.store_scale(239, 504, (p.p88 * p.p66));
            s.store_scale(240, 505, ((p.p88 * 0.3333333333333333) * p.p66));
            s.store_voltage(503, ctx, nodes, Some(12), None);
            s.store_scale(236, 503, (p.p87 * p.p66));
            s.copy_ad(242, 503);
        }

        if (!s.b[507]) {
            s.store_scalar(239, 0.0);
            s.store_scalar(240, 0.0);
            s.store_scalar(236, 0.0);
        }

        s.b[508] = ((p.p89 >= p.p149) && (p.p89 > 0.0));
        s.v[508] = if s.b[508] { 1.0 } else { 0.0 };

        s.b[509] = (p.p93 > 0.0);
        s.v[509] = if s.b[509] { 1.0 } else { 0.0 };

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

        s.b[540] = (s.v[538] > 0.0);
        s.v[540] = if s.b[540] { 1.0 } else { 0.0 };

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
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let eq0_e157: f64 = (s.v[185] + s.v[186]);
        let eq0_e157_d_n0: f64 = (s.dn[185][0] + s.dn[186][0]);
        let eq0_e157_d_n1: f64 = (s.dn[185][1] + s.dn[186][1]);
        let eq0_e157_d_n2: f64 = (s.dn[185][2] + s.dn[186][2]);
        let eq0_e157_d_n3: f64 = (s.dn[185][3] + s.dn[186][3]);
        let eq0_e157_d_n4: f64 = (s.dn[185][4] + s.dn[186][4]);
        let eq0_e157_d_n5: f64 = (s.dn[185][5] + s.dn[186][5]);
        let eq0_e157_d_n6: f64 = (s.dn[185][6] + s.dn[186][6]);
        let eq0_e157_d_n7: f64 = (s.dn[185][7] + s.dn[186][7]);
        let eq0_e157_d_n8: f64 = (s.dn[185][8] + s.dn[186][8]);
        let eq0_e157_d_n9: f64 = (s.dn[185][9] + s.dn[186][9]);
        let eq0_e157_d_n10: f64 = (s.dn[185][10] + s.dn[186][10]);
        let eq0_e157_d_n11: f64 = (s.dn[185][11] + s.dn[186][11]);
        let eq0_e157_d_n12: f64 = (s.dn[185][12] + s.dn[186][12]);
        let eq0_e157_d_n13: f64 = (s.dn[185][13] + s.dn[186][13]);
        let eq0_e157_d_n14: f64 = (s.dn[185][14] + s.dn[186][14]);
        let eq0_e157_d_b0: f64 = (s.db[185][0] + s.db[186][0]);
        let eq0_e157_d_b1: f64 = (s.db[185][1] + s.db[186][1]);
        let eq0_e157_d_b2: f64 = (s.db[185][2] + s.db[186][2]);
        let eq0_e157_d_b3: f64 = (s.db[185][3] + s.db[186][3]);
        let eq0_e157_d_b4: f64 = (s.db[185][4] + s.db[186][4]);
        let eq0_e157_d_b5: f64 = (s.db[185][5] + s.db[186][5]);
        let eq0_e159: f64 = (eq0_e157 + s.v[192]);
        let eq0_e159_d_n0: f64 = (eq0_e157_d_n0 + s.dn[192][0]);
        let eq0_e159_d_n1: f64 = (eq0_e157_d_n1 + s.dn[192][1]);
        let eq0_e159_d_n2: f64 = (eq0_e157_d_n2 + s.dn[192][2]);
        let eq0_e159_d_n3: f64 = (eq0_e157_d_n3 + s.dn[192][3]);
        let eq0_e159_d_n4: f64 = (eq0_e157_d_n4 + s.dn[192][4]);
        let eq0_e159_d_n5: f64 = (eq0_e157_d_n5 + s.dn[192][5]);
        let eq0_e159_d_n6: f64 = (eq0_e157_d_n6 + s.dn[192][6]);
        let eq0_e159_d_n7: f64 = (eq0_e157_d_n7 + s.dn[192][7]);
        let eq0_e159_d_n8: f64 = (eq0_e157_d_n8 + s.dn[192][8]);
        let eq0_e159_d_n9: f64 = (eq0_e157_d_n9 + s.dn[192][9]);
        let eq0_e159_d_n10: f64 = (eq0_e157_d_n10 + s.dn[192][10]);
        let eq0_e159_d_n11: f64 = (eq0_e157_d_n11 + s.dn[192][11]);
        let eq0_e159_d_n12: f64 = (eq0_e157_d_n12 + s.dn[192][12]);
        let eq0_e159_d_n13: f64 = (eq0_e157_d_n13 + s.dn[192][13]);
        let eq0_e159_d_n14: f64 = (eq0_e157_d_n14 + s.dn[192][14]);
        let eq0_e159_d_b0: f64 = (eq0_e157_d_b0 + s.db[192][0]);
        let eq0_e159_d_b1: f64 = (eq0_e157_d_b1 + s.db[192][1]);
        let eq0_e159_d_b2: f64 = (eq0_e157_d_b2 + s.db[192][2]);
        let eq0_e159_d_b3: f64 = (eq0_e157_d_b3 + s.db[192][3]);
        let eq0_e159_d_b4: f64 = (eq0_e157_d_b4 + s.db[192][4]);
        let eq0_e159_d_b5: f64 = (eq0_e157_d_b5 + s.db[192][5]);
        let eq0_e161: f64 = (eq0_e159 + s.v[190]);
        let eq0_e161_d_n0: f64 = (eq0_e159_d_n0 + s.dn[190][0]);
        let eq0_e161_d_n1: f64 = (eq0_e159_d_n1 + s.dn[190][1]);
        let eq0_e161_d_n2: f64 = (eq0_e159_d_n2 + s.dn[190][2]);
        let eq0_e161_d_n3: f64 = (eq0_e159_d_n3 + s.dn[190][3]);
        let eq0_e161_d_n4: f64 = (eq0_e159_d_n4 + s.dn[190][4]);
        let eq0_e161_d_n5: f64 = (eq0_e159_d_n5 + s.dn[190][5]);
        let eq0_e161_d_n6: f64 = (eq0_e159_d_n6 + s.dn[190][6]);
        let eq0_e161_d_n7: f64 = (eq0_e159_d_n7 + s.dn[190][7]);
        let eq0_e161_d_n8: f64 = (eq0_e159_d_n8 + s.dn[190][8]);
        let eq0_e161_d_n9: f64 = (eq0_e159_d_n9 + s.dn[190][9]);
        let eq0_e161_d_n10: f64 = (eq0_e159_d_n10 + s.dn[190][10]);
        let eq0_e161_d_n11: f64 = (eq0_e159_d_n11 + s.dn[190][11]);
        let eq0_e161_d_n12: f64 = (eq0_e159_d_n12 + s.dn[190][12]);
        let eq0_e161_d_n13: f64 = (eq0_e159_d_n13 + s.dn[190][13]);
        let eq0_e161_d_n14: f64 = (eq0_e159_d_n14 + s.dn[190][14]);
        let eq0_e161_d_b0: f64 = (eq0_e159_d_b0 + s.db[190][0]);
        let eq0_e161_d_b1: f64 = (eq0_e159_d_b1 + s.db[190][1]);
        let eq0_e161_d_b2: f64 = (eq0_e159_d_b2 + s.db[190][2]);
        let eq0_e161_d_b3: f64 = (eq0_e159_d_b3 + s.db[190][3]);
        let eq0_e161_d_b4: f64 = (eq0_e159_d_b4 + s.db[190][4]);
        let eq0_e161_d_b5: f64 = (eq0_e159_d_b5 + s.db[190][5]);
        let eq0_e162: f64 = (p.p148 * eq0_e161);
        let eq0_e162_d_n0: f64 = (p.p148 * eq0_e161_d_n0);
        let eq0_e162_d_n1: f64 = (p.p148 * eq0_e161_d_n1);
        let eq0_e162_d_n2: f64 = (p.p148 * eq0_e161_d_n2);
        let eq0_e162_d_n3: f64 = (p.p148 * eq0_e161_d_n3);
        let eq0_e162_d_n4: f64 = (p.p148 * eq0_e161_d_n4);
        let eq0_e162_d_n5: f64 = (p.p148 * eq0_e161_d_n5);
        let eq0_e162_d_n6: f64 = (p.p148 * eq0_e161_d_n6);
        let eq0_e162_d_n7: f64 = (p.p148 * eq0_e161_d_n7);
        let eq0_e162_d_n8: f64 = (p.p148 * eq0_e161_d_n8);
        let eq0_e162_d_n9: f64 = (p.p148 * eq0_e161_d_n9);
        let eq0_e162_d_n10: f64 = (p.p148 * eq0_e161_d_n10);
        let eq0_e162_d_n11: f64 = (p.p148 * eq0_e161_d_n11);
        let eq0_e162_d_n12: f64 = (p.p148 * eq0_e161_d_n12);
        let eq0_e162_d_n13: f64 = (p.p148 * eq0_e161_d_n13);
        let eq0_e162_d_n14: f64 = (p.p148 * eq0_e161_d_n14);
        let eq0_e162_d_b0: f64 = (p.p148 * eq0_e161_d_b0);
        let eq0_e162_d_b1: f64 = (p.p148 * eq0_e161_d_b1);
        let eq0_e162_d_b2: f64 = (p.p148 * eq0_e161_d_b2);
        let eq0_e162_d_b3: f64 = (p.p148 * eq0_e161_d_b3);
        let eq0_e162_d_b4: f64 = (p.p148 * eq0_e161_d_b4);
        let eq0_e162_d_b5: f64 = (p.p148 * eq0_e161_d_b5);
        let eq0_e165: f64 = (s.v[233] * (nv8 - nv6));
        let eq0_e165_d_n6: f64 = (-s.v[233]);
        let eq0_e165_d_n8: f64 = s.v[233];
        let eq0_e166: f64 = (eq0_e162 + eq0_e165);
        let eq0_e166_d_n6: f64 = (eq0_e162_d_n6 + eq0_e165_d_n6);
        let eq0_e166_d_n8: f64 = (eq0_e162_d_n8 + eq0_e165_d_n8);
        let eq0_value: f64 = eq0_e166;
        let eq0_node_derivatives: [f64; 15] = [eq0_e162_d_n0, eq0_e162_d_n1, eq0_e162_d_n2, eq0_e162_d_n3, eq0_e162_d_n4, eq0_e162_d_n5, eq0_e166_d_n6, eq0_e162_d_n7, eq0_e166_d_n8, eq0_e162_d_n9, eq0_e162_d_n10, eq0_e162_d_n11, eq0_e162_d_n12, eq0_e162_d_n13, eq0_e162_d_n14];
        let eq0_branch_derivatives: [f64; 6] = [eq0_e162_d_b0, eq0_e162_d_b1, eq0_e162_d_b2, eq0_e162_d_b3, eq0_e162_d_b4, eq0_e162_d_b5];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
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
        let eq1_e172: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, eq1_e171);
        let eq1_e172_d_n0: f64 = (eq1_e171_d_n0 * ddt_scale);
        let eq1_e172_d_n1: f64 = (eq1_e171_d_n1 * ddt_scale);
        let eq1_e172_d_n2: f64 = (eq1_e171_d_n2 * ddt_scale);
        let eq1_e172_d_n3: f64 = (eq1_e171_d_n3 * ddt_scale);
        let eq1_e172_d_n4: f64 = (eq1_e171_d_n4 * ddt_scale);
        let eq1_e172_d_n5: f64 = (eq1_e171_d_n5 * ddt_scale);
        let eq1_e172_d_n6: f64 = (eq1_e171_d_n6 * ddt_scale);
        let eq1_e172_d_n7: f64 = (eq1_e171_d_n7 * ddt_scale);
        let eq1_e172_d_n8: f64 = (eq1_e171_d_n8 * ddt_scale);
        let eq1_e172_d_n9: f64 = (eq1_e171_d_n9 * ddt_scale);
        let eq1_e172_d_n10: f64 = (eq1_e171_d_n10 * ddt_scale);
        let eq1_e172_d_n11: f64 = (eq1_e171_d_n11 * ddt_scale);
        let eq1_e172_d_n12: f64 = (eq1_e171_d_n12 * ddt_scale);
        let eq1_e172_d_n13: f64 = (eq1_e171_d_n13 * ddt_scale);
        let eq1_e172_d_n14: f64 = (eq1_e171_d_n14 * ddt_scale);
        let eq1_e172_d_b0: f64 = (eq1_e171_d_b0 * ddt_scale);
        let eq1_e172_d_b1: f64 = (eq1_e171_d_b1 * ddt_scale);
        let eq1_e172_d_b2: f64 = (eq1_e171_d_b2 * ddt_scale);
        let eq1_e172_d_b3: f64 = (eq1_e171_d_b3 * ddt_scale);
        let eq1_e172_d_b4: f64 = (eq1_e171_d_b4 * ddt_scale);
        let eq1_e172_d_b5: f64 = (eq1_e171_d_b5 * ddt_scale);
        let eq1_value: f64 = eq1_e172;
        let eq1_node_derivatives: [f64; 15] = [eq1_e172_d_n0, eq1_e172_d_n1, eq1_e172_d_n2, eq1_e172_d_n3, eq1_e172_d_n4, eq1_e172_d_n5, eq1_e172_d_n6, eq1_e172_d_n7, eq1_e172_d_n8, eq1_e172_d_n9, eq1_e172_d_n10, eq1_e172_d_n11, eq1_e172_d_n12, eq1_e172_d_n13, eq1_e172_d_n14];
        let eq1_branch_derivatives: [f64; 6] = [eq1_e172_d_b0, eq1_e172_d_b1, eq1_e172_d_b2, eq1_e172_d_b3, eq1_e172_d_b4, eq1_e172_d_b5];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let eq2_e176: f64 = (s.v[187] - s.v[244]);
        let eq2_e176_d_n0: f64 = (s.dn[187][0] - s.dn[244][0]);
        let eq2_e176_d_n1: f64 = (s.dn[187][1] - s.dn[244][1]);
        let eq2_e176_d_n2: f64 = (s.dn[187][2] - s.dn[244][2]);
        let eq2_e176_d_n3: f64 = (s.dn[187][3] - s.dn[244][3]);
        let eq2_e176_d_n4: f64 = (s.dn[187][4] - s.dn[244][4]);
        let eq2_e176_d_n5: f64 = (s.dn[187][5] - s.dn[244][5]);
        let eq2_e176_d_n6: f64 = (s.dn[187][6] - s.dn[244][6]);
        let eq2_e176_d_n7: f64 = (s.dn[187][7] - s.dn[244][7]);
        let eq2_e176_d_n8: f64 = (s.dn[187][8] - s.dn[244][8]);
        let eq2_e176_d_n9: f64 = (s.dn[187][9] - s.dn[244][9]);
        let eq2_e176_d_n10: f64 = (s.dn[187][10] - s.dn[244][10]);
        let eq2_e176_d_n11: f64 = (s.dn[187][11] - s.dn[244][11]);
        let eq2_e176_d_n12: f64 = (s.dn[187][12] - s.dn[244][12]);
        let eq2_e176_d_n13: f64 = (s.dn[187][13] - s.dn[244][13]);
        let eq2_e176_d_n14: f64 = (s.dn[187][14] - s.dn[244][14]);
        let eq2_e176_d_b0: f64 = (s.db[187][0] - s.db[244][0]);
        let eq2_e176_d_b1: f64 = (s.db[187][1] - s.db[244][1]);
        let eq2_e176_d_b2: f64 = (s.db[187][2] - s.db[244][2]);
        let eq2_e176_d_b3: f64 = (s.db[187][3] - s.db[244][3]);
        let eq2_e176_d_b4: f64 = (s.db[187][4] - s.db[244][4]);
        let eq2_e176_d_b5: f64 = (s.db[187][5] - s.db[244][5]);
        let eq2_e177: f64 = (p.p148 * eq2_e176);
        let eq2_e177_d_n0: f64 = (p.p148 * eq2_e176_d_n0);
        let eq2_e177_d_n1: f64 = (p.p148 * eq2_e176_d_n1);
        let eq2_e177_d_n2: f64 = (p.p148 * eq2_e176_d_n2);
        let eq2_e177_d_n3: f64 = (p.p148 * eq2_e176_d_n3);
        let eq2_e177_d_n4: f64 = (p.p148 * eq2_e176_d_n4);
        let eq2_e177_d_n5: f64 = (p.p148 * eq2_e176_d_n5);
        let eq2_e177_d_n6: f64 = (p.p148 * eq2_e176_d_n6);
        let eq2_e177_d_n7: f64 = (p.p148 * eq2_e176_d_n7);
        let eq2_e177_d_n8: f64 = (p.p148 * eq2_e176_d_n8);
        let eq2_e177_d_n9: f64 = (p.p148 * eq2_e176_d_n9);
        let eq2_e177_d_n10: f64 = (p.p148 * eq2_e176_d_n10);
        let eq2_e177_d_n11: f64 = (p.p148 * eq2_e176_d_n11);
        let eq2_e177_d_n12: f64 = (p.p148 * eq2_e176_d_n12);
        let eq2_e177_d_n13: f64 = (p.p148 * eq2_e176_d_n13);
        let eq2_e177_d_n14: f64 = (p.p148 * eq2_e176_d_n14);
        let eq2_e177_d_b0: f64 = (p.p148 * eq2_e176_d_b0);
        let eq2_e177_d_b1: f64 = (p.p148 * eq2_e176_d_b1);
        let eq2_e177_d_b2: f64 = (p.p148 * eq2_e176_d_b2);
        let eq2_e177_d_b3: f64 = (p.p148 * eq2_e176_d_b3);
        let eq2_e177_d_b4: f64 = (p.p148 * eq2_e176_d_b4);
        let eq2_e177_d_b5: f64 = (p.p148 * eq2_e176_d_b5);
        let eq2_e180: f64 = (s.v[233] * (nv8 - nv5));
        let eq2_e180_d_n5: f64 = (-s.v[233]);
        let eq2_e180_d_n8: f64 = s.v[233];
        let eq2_e181: f64 = (eq2_e177 + eq2_e180);
        let eq2_e181_d_n5: f64 = (eq2_e177_d_n5 + eq2_e180_d_n5);
        let eq2_e181_d_n8: f64 = (eq2_e177_d_n8 + eq2_e180_d_n8);
        let eq2_value: f64 = eq2_e181;
        let eq2_node_derivatives: [f64; 15] = [eq2_e177_d_n0, eq2_e177_d_n1, eq2_e177_d_n2, eq2_e177_d_n3, eq2_e177_d_n4, eq2_e181_d_n5, eq2_e177_d_n6, eq2_e177_d_n7, eq2_e181_d_n8, eq2_e177_d_n9, eq2_e177_d_n10, eq2_e177_d_n11, eq2_e177_d_n12, eq2_e177_d_n13, eq2_e177_d_n14];
        let eq2_branch_derivatives: [f64; 6] = [eq2_e177_d_b0, eq2_e177_d_b1, eq2_e177_d_b2, eq2_e177_d_b3, eq2_e177_d_b4, eq2_e177_d_b5];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
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
        let eq3_e187: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, eq3_e186);
        let eq3_e187_d_n0: f64 = (eq3_e186_d_n0 * ddt_scale);
        let eq3_e187_d_n1: f64 = (eq3_e186_d_n1 * ddt_scale);
        let eq3_e187_d_n2: f64 = (eq3_e186_d_n2 * ddt_scale);
        let eq3_e187_d_n3: f64 = (eq3_e186_d_n3 * ddt_scale);
        let eq3_e187_d_n4: f64 = (eq3_e186_d_n4 * ddt_scale);
        let eq3_e187_d_n5: f64 = (eq3_e186_d_n5 * ddt_scale);
        let eq3_e187_d_n6: f64 = (eq3_e186_d_n6 * ddt_scale);
        let eq3_e187_d_n7: f64 = (eq3_e186_d_n7 * ddt_scale);
        let eq3_e187_d_n8: f64 = (eq3_e186_d_n8 * ddt_scale);
        let eq3_e187_d_n9: f64 = (eq3_e186_d_n9 * ddt_scale);
        let eq3_e187_d_n10: f64 = (eq3_e186_d_n10 * ddt_scale);
        let eq3_e187_d_n11: f64 = (eq3_e186_d_n11 * ddt_scale);
        let eq3_e187_d_n12: f64 = (eq3_e186_d_n12 * ddt_scale);
        let eq3_e187_d_n13: f64 = (eq3_e186_d_n13 * ddt_scale);
        let eq3_e187_d_n14: f64 = (eq3_e186_d_n14 * ddt_scale);
        let eq3_e187_d_b0: f64 = (eq3_e186_d_b0 * ddt_scale);
        let eq3_e187_d_b1: f64 = (eq3_e186_d_b1 * ddt_scale);
        let eq3_e187_d_b2: f64 = (eq3_e186_d_b2 * ddt_scale);
        let eq3_e187_d_b3: f64 = (eq3_e186_d_b3 * ddt_scale);
        let eq3_e187_d_b4: f64 = (eq3_e186_d_b4 * ddt_scale);
        let eq3_e187_d_b5: f64 = (eq3_e186_d_b5 * ddt_scale);
        let eq3_value: f64 = eq3_e187;
        let eq3_node_derivatives: [f64; 15] = [eq3_e187_d_n0, eq3_e187_d_n1, eq3_e187_d_n2, eq3_e187_d_n3, eq3_e187_d_n4, eq3_e187_d_n5, eq3_e187_d_n6, eq3_e187_d_n7, eq3_e187_d_n8, eq3_e187_d_n9, eq3_e187_d_n10, eq3_e187_d_n11, eq3_e187_d_n12, eq3_e187_d_n13, eq3_e187_d_n14];
        let eq3_branch_derivatives: [f64; 6] = [eq3_e187_d_b0, eq3_e187_d_b1, eq3_e187_d_b2, eq3_e187_d_b3, eq3_e187_d_b4, eq3_e187_d_b5];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let eq4_e190: f64 = (p.p148 * s.v[241]);
        let eq4_e190_d_n0: f64 = (p.p148 * s.dn[241][0]);
        let eq4_e190_d_n1: f64 = (p.p148 * s.dn[241][1]);
        let eq4_e190_d_n2: f64 = (p.p148 * s.dn[241][2]);
        let eq4_e190_d_n3: f64 = (p.p148 * s.dn[241][3]);
        let eq4_e190_d_n4: f64 = (p.p148 * s.dn[241][4]);
        let eq4_e190_d_n5: f64 = (p.p148 * s.dn[241][5]);
        let eq4_e190_d_n6: f64 = (p.p148 * s.dn[241][6]);
        let eq4_e190_d_n7: f64 = (p.p148 * s.dn[241][7]);
        let eq4_e190_d_n8: f64 = (p.p148 * s.dn[241][8]);
        let eq4_e190_d_n9: f64 = (p.p148 * s.dn[241][9]);
        let eq4_e190_d_n10: f64 = (p.p148 * s.dn[241][10]);
        let eq4_e190_d_n11: f64 = (p.p148 * s.dn[241][11]);
        let eq4_e190_d_n12: f64 = (p.p148 * s.dn[241][12]);
        let eq4_e190_d_n13: f64 = (p.p148 * s.dn[241][13]);
        let eq4_e190_d_n14: f64 = (p.p148 * s.dn[241][14]);
        let eq4_e190_d_b0: f64 = (p.p148 * s.db[241][0]);
        let eq4_e190_d_b1: f64 = (p.p148 * s.db[241][1]);
        let eq4_e190_d_b2: f64 = (p.p148 * s.db[241][2]);
        let eq4_e190_d_b3: f64 = (p.p148 * s.db[241][3]);
        let eq4_e190_d_b4: f64 = (p.p148 * s.db[241][4]);
        let eq4_e190_d_b5: f64 = (p.p148 * s.db[241][5]);
        let eq4_value: f64 = eq4_e190;
        let eq4_node_derivatives: [f64; 15] = [eq4_e190_d_n0, eq4_e190_d_n1, eq4_e190_d_n2, eq4_e190_d_n3, eq4_e190_d_n4, eq4_e190_d_n5, eq4_e190_d_n6, eq4_e190_d_n7, eq4_e190_d_n8, eq4_e190_d_n9, eq4_e190_d_n10, eq4_e190_d_n11, eq4_e190_d_n12, eq4_e190_d_n13, eq4_e190_d_n14];
        let eq4_branch_derivatives: [f64; 6] = [eq4_e190_d_b0, eq4_e190_d_b1, eq4_e190_d_b2, eq4_e190_d_b3, eq4_e190_d_b4, eq4_e190_d_b5];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let eq5_e193: f64 = (p.p148 * s.v[218]);
        let eq5_e193_d_n0: f64 = (p.p148 * s.dn[218][0]);
        let eq5_e193_d_n1: f64 = (p.p148 * s.dn[218][1]);
        let eq5_e193_d_n2: f64 = (p.p148 * s.dn[218][2]);
        let eq5_e193_d_n3: f64 = (p.p148 * s.dn[218][3]);
        let eq5_e193_d_n4: f64 = (p.p148 * s.dn[218][4]);
        let eq5_e193_d_n5: f64 = (p.p148 * s.dn[218][5]);
        let eq5_e193_d_n6: f64 = (p.p148 * s.dn[218][6]);
        let eq5_e193_d_n7: f64 = (p.p148 * s.dn[218][7]);
        let eq5_e193_d_n8: f64 = (p.p148 * s.dn[218][8]);
        let eq5_e193_d_n9: f64 = (p.p148 * s.dn[218][9]);
        let eq5_e193_d_n10: f64 = (p.p148 * s.dn[218][10]);
        let eq5_e193_d_n11: f64 = (p.p148 * s.dn[218][11]);
        let eq5_e193_d_n12: f64 = (p.p148 * s.dn[218][12]);
        let eq5_e193_d_n13: f64 = (p.p148 * s.dn[218][13]);
        let eq5_e193_d_n14: f64 = (p.p148 * s.dn[218][14]);
        let eq5_e193_d_b0: f64 = (p.p148 * s.db[218][0]);
        let eq5_e193_d_b1: f64 = (p.p148 * s.db[218][1]);
        let eq5_e193_d_b2: f64 = (p.p148 * s.db[218][2]);
        let eq5_e193_d_b3: f64 = (p.p148 * s.db[218][3]);
        let eq5_e193_d_b4: f64 = (p.p148 * s.db[218][4]);
        let eq5_e193_d_b5: f64 = (p.p148 * s.db[218][5]);
        let eq5_value: f64 = eq5_e193;
        let eq5_node_derivatives: [f64; 15] = [eq5_e193_d_n0, eq5_e193_d_n1, eq5_e193_d_n2, eq5_e193_d_n3, eq5_e193_d_n4, eq5_e193_d_n5, eq5_e193_d_n6, eq5_e193_d_n7, eq5_e193_d_n8, eq5_e193_d_n9, eq5_e193_d_n10, eq5_e193_d_n11, eq5_e193_d_n12, eq5_e193_d_n13, eq5_e193_d_n14];
        let eq5_branch_derivatives: [f64; 6] = [eq5_e193_d_b0, eq5_e193_d_b1, eq5_e193_d_b2, eq5_e193_d_b3, eq5_e193_d_b4, eq5_e193_d_b5];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq6_e199, eq6_e199_d_n0, eq6_e199_d_n1, eq6_e199_d_n2, eq6_e199_d_n3, eq6_e199_d_n4, eq6_e199_d_n5, eq6_e199_d_n6, eq6_e199_d_n7, eq6_e199_d_n8, eq6_e199_d_n9, eq6_e199_d_n10, eq6_e199_d_n11, eq6_e199_d_n12, eq6_e199_d_n13, eq6_e199_d_n14, eq6_e199_d_b0, eq6_e199_d_b1, eq6_e199_d_b2, eq6_e199_d_b3, eq6_e199_d_b4, eq6_e199_d_b5,) = {
    if s.b[508] {
        let eq6_e197: f64 = ((nv7 - nv8) / s.v[70]);
        let eq6_e197_d_n0: f64 = (-(((nv7 - nv8) * s.dn[70][0]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n1: f64 = (-(((nv7 - nv8) * s.dn[70][1]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n2: f64 = (-(((nv7 - nv8) * s.dn[70][2]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n3: f64 = (-(((nv7 - nv8) * s.dn[70][3]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n4: f64 = (-(((nv7 - nv8) * s.dn[70][4]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n5: f64 = (-(((nv7 - nv8) * s.dn[70][5]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n6: f64 = (-(((nv7 - nv8) * s.dn[70][6]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n7: f64 = ((s.v[70] - ((nv7 - nv8) * s.dn[70][7])) / (s.v[70] * s.v[70]));
        let eq6_e197_d_n8: f64 = (((-s.v[70]) - ((nv7 - nv8) * s.dn[70][8])) / (s.v[70] * s.v[70]));
        let eq6_e197_d_n9: f64 = (-(((nv7 - nv8) * s.dn[70][9]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n10: f64 = (-(((nv7 - nv8) * s.dn[70][10]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n11: f64 = (-(((nv7 - nv8) * s.dn[70][11]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n12: f64 = (-(((nv7 - nv8) * s.dn[70][12]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n13: f64 = (-(((nv7 - nv8) * s.dn[70][13]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n14: f64 = (-(((nv7 - nv8) * s.dn[70][14]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_b0: f64 = (-(((nv7 - nv8) * s.db[70][0]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_b1: f64 = (-(((nv7 - nv8) * s.db[70][1]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_b2: f64 = (-(((nv7 - nv8) * s.db[70][2]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_b3: f64 = (-(((nv7 - nv8) * s.db[70][3]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_b4: f64 = (-(((nv7 - nv8) * s.db[70][4]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_b5: f64 = (-(((nv7 - nv8) * s.db[70][5]) / (s.v[70] * s.v[70])));
        (eq6_e197, eq6_e197_d_n0, eq6_e197_d_n1, eq6_e197_d_n2, eq6_e197_d_n3, eq6_e197_d_n4, eq6_e197_d_n5, eq6_e197_d_n6, eq6_e197_d_n7, eq6_e197_d_n8, eq6_e197_d_n9, eq6_e197_d_n10, eq6_e197_d_n11, eq6_e197_d_n12, eq6_e197_d_n13, eq6_e197_d_n14, eq6_e197_d_b0, eq6_e197_d_b1, eq6_e197_d_b2, eq6_e197_d_b3, eq6_e197_d_b4, eq6_e197_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e199;
        let eq6_node_derivatives: [f64; 15] = [eq6_e199_d_n0, eq6_e199_d_n1, eq6_e199_d_n2, eq6_e199_d_n3, eq6_e199_d_n4, eq6_e199_d_n5, eq6_e199_d_n6, eq6_e199_d_n7, eq6_e199_d_n8, eq6_e199_d_n9, eq6_e199_d_n10, eq6_e199_d_n11, eq6_e199_d_n12, eq6_e199_d_n13, eq6_e199_d_n14];
        let eq6_branch_derivatives: [f64; 6] = [eq6_e199_d_b0, eq6_e199_d_b1, eq6_e199_d_b2, eq6_e199_d_b3, eq6_e199_d_b4, eq6_e199_d_b5];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq7_e206, eq7_e206_d_n0, eq7_e206_d_n1, eq7_e206_d_n2, eq7_e206_d_n3, eq7_e206_d_n4, eq7_e206_d_n5, eq7_e206_d_n6, eq7_e206_d_n7, eq7_e206_d_n8, eq7_e206_d_n9, eq7_e206_d_n10, eq7_e206_d_n11, eq7_e206_d_n12, eq7_e206_d_n13, eq7_e206_d_n14, eq7_e206_d_b0, eq7_e206_d_b1, eq7_e206_d_b2, eq7_e206_d_b3, eq7_e206_d_b4, eq7_e206_d_b5,) = {
    if (s.b[508] && s.b[509]) {
        let eq7_e204: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, s.v[183]);
        let eq7_e204_d_n0: f64 = (s.dn[183][0] * ddt_scale);
        let eq7_e204_d_n1: f64 = (s.dn[183][1] * ddt_scale);
        let eq7_e204_d_n2: f64 = (s.dn[183][2] * ddt_scale);
        let eq7_e204_d_n3: f64 = (s.dn[183][3] * ddt_scale);
        let eq7_e204_d_n4: f64 = (s.dn[183][4] * ddt_scale);
        let eq7_e204_d_n5: f64 = (s.dn[183][5] * ddt_scale);
        let eq7_e204_d_n6: f64 = (s.dn[183][6] * ddt_scale);
        let eq7_e204_d_n7: f64 = (s.dn[183][7] * ddt_scale);
        let eq7_e204_d_n8: f64 = (s.dn[183][8] * ddt_scale);
        let eq7_e204_d_n9: f64 = (s.dn[183][9] * ddt_scale);
        let eq7_e204_d_n10: f64 = (s.dn[183][10] * ddt_scale);
        let eq7_e204_d_n11: f64 = (s.dn[183][11] * ddt_scale);
        let eq7_e204_d_n12: f64 = (s.dn[183][12] * ddt_scale);
        let eq7_e204_d_n13: f64 = (s.dn[183][13] * ddt_scale);
        let eq7_e204_d_n14: f64 = (s.dn[183][14] * ddt_scale);
        let eq7_e204_d_b0: f64 = (s.db[183][0] * ddt_scale);
        let eq7_e204_d_b1: f64 = (s.db[183][1] * ddt_scale);
        let eq7_e204_d_b2: f64 = (s.db[183][2] * ddt_scale);
        let eq7_e204_d_b3: f64 = (s.db[183][3] * ddt_scale);
        let eq7_e204_d_b4: f64 = (s.db[183][4] * ddt_scale);
        let eq7_e204_d_b5: f64 = (s.db[183][5] * ddt_scale);
        (eq7_e204, eq7_e204_d_n0, eq7_e204_d_n1, eq7_e204_d_n2, eq7_e204_d_n3, eq7_e204_d_n4, eq7_e204_d_n5, eq7_e204_d_n6, eq7_e204_d_n7, eq7_e204_d_n8, eq7_e204_d_n9, eq7_e204_d_n10, eq7_e204_d_n11, eq7_e204_d_n12, eq7_e204_d_n13, eq7_e204_d_n14, eq7_e204_d_b0, eq7_e204_d_b1, eq7_e204_d_b2, eq7_e204_d_b3, eq7_e204_d_b4, eq7_e204_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e206;
        let eq7_node_derivatives: [f64; 15] = [eq7_e206_d_n0, eq7_e206_d_n1, eq7_e206_d_n2, eq7_e206_d_n3, eq7_e206_d_n4, eq7_e206_d_n5, eq7_e206_d_n6, eq7_e206_d_n7, eq7_e206_d_n8, eq7_e206_d_n9, eq7_e206_d_n10, eq7_e206_d_n11, eq7_e206_d_n12, eq7_e206_d_n13, eq7_e206_d_n14];
        let eq7_branch_derivatives: [f64; 6] = [eq7_e206_d_b0, eq7_e206_d_b1, eq7_e206_d_b2, eq7_e206_d_b3, eq7_e206_d_b4, eq7_e206_d_b5];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let (eq8_e211,) = {
    if (!s.b[508]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq8_value: f64 = eq8_e211;
        stamper.stamp_potential_const_local(
            0,
            eq8_value,
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
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq9_e218, eq9_e218_d_n0, eq9_e218_d_n1, eq9_e218_d_n2, eq9_e218_d_n3, eq9_e218_d_n4, eq9_e218_d_n5, eq9_e218_d_n6, eq9_e218_d_n7, eq9_e218_d_n8, eq9_e218_d_n9, eq9_e218_d_n10, eq9_e218_d_n11, eq9_e218_d_n12, eq9_e218_d_n13, eq9_e218_d_n14, eq9_e218_d_b0, eq9_e218_d_b1, eq9_e218_d_b2, eq9_e218_d_b3, eq9_e218_d_b4, eq9_e218_d_b5,) = {
    if s.b[510] {
        let eq9_e214: f64 = (-p.p148);
        let eq9_e216: f64 = (eq9_e214 * s.v[191]);
        let eq9_e216_d_n0: f64 = (eq9_e214 * s.dn[191][0]);
        let eq9_e216_d_n1: f64 = (eq9_e214 * s.dn[191][1]);
        let eq9_e216_d_n2: f64 = (eq9_e214 * s.dn[191][2]);
        let eq9_e216_d_n3: f64 = (eq9_e214 * s.dn[191][3]);
        let eq9_e216_d_n4: f64 = (eq9_e214 * s.dn[191][4]);
        let eq9_e216_d_n5: f64 = (eq9_e214 * s.dn[191][5]);
        let eq9_e216_d_n6: f64 = (eq9_e214 * s.dn[191][6]);
        let eq9_e216_d_n7: f64 = (eq9_e214 * s.dn[191][7]);
        let eq9_e216_d_n8: f64 = (eq9_e214 * s.dn[191][8]);
        let eq9_e216_d_n9: f64 = (eq9_e214 * s.dn[191][9]);
        let eq9_e216_d_n10: f64 = (eq9_e214 * s.dn[191][10]);
        let eq9_e216_d_n11: f64 = (eq9_e214 * s.dn[191][11]);
        let eq9_e216_d_n12: f64 = (eq9_e214 * s.dn[191][12]);
        let eq9_e216_d_n13: f64 = (eq9_e214 * s.dn[191][13]);
        let eq9_e216_d_n14: f64 = (eq9_e214 * s.dn[191][14]);
        let eq9_e216_d_b0: f64 = (eq9_e214 * s.db[191][0]);
        let eq9_e216_d_b1: f64 = (eq9_e214 * s.db[191][1]);
        let eq9_e216_d_b2: f64 = (eq9_e214 * s.db[191][2]);
        let eq9_e216_d_b3: f64 = (eq9_e214 * s.db[191][3]);
        let eq9_e216_d_b4: f64 = (eq9_e214 * s.db[191][4]);
        let eq9_e216_d_b5: f64 = (eq9_e214 * s.db[191][5]);
        (eq9_e216, eq9_e216_d_n0, eq9_e216_d_n1, eq9_e216_d_n2, eq9_e216_d_n3, eq9_e216_d_n4, eq9_e216_d_n5, eq9_e216_d_n6, eq9_e216_d_n7, eq9_e216_d_n8, eq9_e216_d_n9, eq9_e216_d_n10, eq9_e216_d_n11, eq9_e216_d_n12, eq9_e216_d_n13, eq9_e216_d_n14, eq9_e216_d_b0, eq9_e216_d_b1, eq9_e216_d_b2, eq9_e216_d_b3, eq9_e216_d_b4, eq9_e216_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e218;
        let eq9_node_derivatives: [f64; 15] = [eq9_e218_d_n0, eq9_e218_d_n1, eq9_e218_d_n2, eq9_e218_d_n3, eq9_e218_d_n4, eq9_e218_d_n5, eq9_e218_d_n6, eq9_e218_d_n7, eq9_e218_d_n8, eq9_e218_d_n9, eq9_e218_d_n10, eq9_e218_d_n11, eq9_e218_d_n12, eq9_e218_d_n13, eq9_e218_d_n14];
        let eq9_branch_derivatives: [f64; 6] = [eq9_e218_d_b0, eq9_e218_d_b1, eq9_e218_d_b2, eq9_e218_d_b3, eq9_e218_d_b4, eq9_e218_d_b5];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let (eq10_e226, eq10_e226_d_n0, eq10_e226_d_n1, eq10_e226_d_n2, eq10_e226_d_n3, eq10_e226_d_n4, eq10_e226_d_n5, eq10_e226_d_n6, eq10_e226_d_n7, eq10_e226_d_n8, eq10_e226_d_n9, eq10_e226_d_n10, eq10_e226_d_n11, eq10_e226_d_n12, eq10_e226_d_n13, eq10_e226_d_n14, eq10_e226_d_b0, eq10_e226_d_b1, eq10_e226_d_b2, eq10_e226_d_b3, eq10_e226_d_b4, eq10_e226_d_b5,) = {
    if (!s.b[510]) {
        let eq10_e222: f64 = (-p.p148);
        let eq10_e224: f64 = (eq10_e222 * s.v[191]);
        let eq10_e224_d_n0: f64 = (eq10_e222 * s.dn[191][0]);
        let eq10_e224_d_n1: f64 = (eq10_e222 * s.dn[191][1]);
        let eq10_e224_d_n2: f64 = (eq10_e222 * s.dn[191][2]);
        let eq10_e224_d_n3: f64 = (eq10_e222 * s.dn[191][3]);
        let eq10_e224_d_n4: f64 = (eq10_e222 * s.dn[191][4]);
        let eq10_e224_d_n5: f64 = (eq10_e222 * s.dn[191][5]);
        let eq10_e224_d_n6: f64 = (eq10_e222 * s.dn[191][6]);
        let eq10_e224_d_n7: f64 = (eq10_e222 * s.dn[191][7]);
        let eq10_e224_d_n8: f64 = (eq10_e222 * s.dn[191][8]);
        let eq10_e224_d_n9: f64 = (eq10_e222 * s.dn[191][9]);
        let eq10_e224_d_n10: f64 = (eq10_e222 * s.dn[191][10]);
        let eq10_e224_d_n11: f64 = (eq10_e222 * s.dn[191][11]);
        let eq10_e224_d_n12: f64 = (eq10_e222 * s.dn[191][12]);
        let eq10_e224_d_n13: f64 = (eq10_e222 * s.dn[191][13]);
        let eq10_e224_d_n14: f64 = (eq10_e222 * s.dn[191][14]);
        let eq10_e224_d_b0: f64 = (eq10_e222 * s.db[191][0]);
        let eq10_e224_d_b1: f64 = (eq10_e222 * s.db[191][1]);
        let eq10_e224_d_b2: f64 = (eq10_e222 * s.db[191][2]);
        let eq10_e224_d_b3: f64 = (eq10_e222 * s.db[191][3]);
        let eq10_e224_d_b4: f64 = (eq10_e222 * s.db[191][4]);
        let eq10_e224_d_b5: f64 = (eq10_e222 * s.db[191][5]);
        (eq10_e224, eq10_e224_d_n0, eq10_e224_d_n1, eq10_e224_d_n2, eq10_e224_d_n3, eq10_e224_d_n4, eq10_e224_d_n5, eq10_e224_d_n6, eq10_e224_d_n7, eq10_e224_d_n8, eq10_e224_d_n9, eq10_e224_d_n10, eq10_e224_d_n11, eq10_e224_d_n12, eq10_e224_d_n13, eq10_e224_d_n14, eq10_e224_d_b0, eq10_e224_d_b1, eq10_e224_d_b2, eq10_e224_d_b3, eq10_e224_d_b4, eq10_e224_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e226;
        let eq10_node_derivatives: [f64; 15] = [eq10_e226_d_n0, eq10_e226_d_n1, eq10_e226_d_n2, eq10_e226_d_n3, eq10_e226_d_n4, eq10_e226_d_n5, eq10_e226_d_n6, eq10_e226_d_n7, eq10_e226_d_n8, eq10_e226_d_n9, eq10_e226_d_n10, eq10_e226_d_n11, eq10_e226_d_n12, eq10_e226_d_n13, eq10_e226_d_n14];
        let eq10_branch_derivatives: [f64; 6] = [eq10_e226_d_b0, eq10_e226_d_b1, eq10_e226_d_b2, eq10_e226_d_b3, eq10_e226_d_b4, eq10_e226_d_b5];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e228: f64 = (-p.p148);
        let eq11_e230: f64 = (eq11_e228 * s.v[193]);
        let eq11_e230_d_n0: f64 = (eq11_e228 * s.dn[193][0]);
        let eq11_e230_d_n1: f64 = (eq11_e228 * s.dn[193][1]);
        let eq11_e230_d_n2: f64 = (eq11_e228 * s.dn[193][2]);
        let eq11_e230_d_n3: f64 = (eq11_e228 * s.dn[193][3]);
        let eq11_e230_d_n4: f64 = (eq11_e228 * s.dn[193][4]);
        let eq11_e230_d_n5: f64 = (eq11_e228 * s.dn[193][5]);
        let eq11_e230_d_n6: f64 = (eq11_e228 * s.dn[193][6]);
        let eq11_e230_d_n7: f64 = (eq11_e228 * s.dn[193][7]);
        let eq11_e230_d_n8: f64 = (eq11_e228 * s.dn[193][8]);
        let eq11_e230_d_n9: f64 = (eq11_e228 * s.dn[193][9]);
        let eq11_e230_d_n10: f64 = (eq11_e228 * s.dn[193][10]);
        let eq11_e230_d_n11: f64 = (eq11_e228 * s.dn[193][11]);
        let eq11_e230_d_n12: f64 = (eq11_e228 * s.dn[193][12]);
        let eq11_e230_d_n13: f64 = (eq11_e228 * s.dn[193][13]);
        let eq11_e230_d_n14: f64 = (eq11_e228 * s.dn[193][14]);
        let eq11_e230_d_b0: f64 = (eq11_e228 * s.db[193][0]);
        let eq11_e230_d_b1: f64 = (eq11_e228 * s.db[193][1]);
        let eq11_e230_d_b2: f64 = (eq11_e228 * s.db[193][2]);
        let eq11_e230_d_b3: f64 = (eq11_e228 * s.db[193][3]);
        let eq11_e230_d_b4: f64 = (eq11_e228 * s.db[193][4]);
        let eq11_e230_d_b5: f64 = (eq11_e228 * s.db[193][5]);
        let eq11_value: f64 = eq11_e230;
        let eq11_node_derivatives: [f64; 15] = [eq11_e230_d_n0, eq11_e230_d_n1, eq11_e230_d_n2, eq11_e230_d_n3, eq11_e230_d_n4, eq11_e230_d_n5, eq11_e230_d_n6, eq11_e230_d_n7, eq11_e230_d_n8, eq11_e230_d_n9, eq11_e230_d_n10, eq11_e230_d_n11, eq11_e230_d_n12, eq11_e230_d_n13, eq11_e230_d_n14];
        let eq11_branch_derivatives: [f64; 6] = [eq11_e230_d_b0, eq11_e230_d_b1, eq11_e230_d_b2, eq11_e230_d_b3, eq11_e230_d_b4, eq11_e230_d_b5];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e234: f64 = (s.v[188] + s.v[189]);
        let eq12_e234_d_n0: f64 = (s.dn[188][0] + s.dn[189][0]);
        let eq12_e234_d_n1: f64 = (s.dn[188][1] + s.dn[189][1]);
        let eq12_e234_d_n2: f64 = (s.dn[188][2] + s.dn[189][2]);
        let eq12_e234_d_n3: f64 = (s.dn[188][3] + s.dn[189][3]);
        let eq12_e234_d_n4: f64 = (s.dn[188][4] + s.dn[189][4]);
        let eq12_e234_d_n5: f64 = (s.dn[188][5] + s.dn[189][5]);
        let eq12_e234_d_n6: f64 = (s.dn[188][6] + s.dn[189][6]);
        let eq12_e234_d_n7: f64 = (s.dn[188][7] + s.dn[189][7]);
        let eq12_e234_d_n8: f64 = (s.dn[188][8] + s.dn[189][8]);
        let eq12_e234_d_n9: f64 = (s.dn[188][9] + s.dn[189][9]);
        let eq12_e234_d_n10: f64 = (s.dn[188][10] + s.dn[189][10]);
        let eq12_e234_d_n11: f64 = (s.dn[188][11] + s.dn[189][11]);
        let eq12_e234_d_n12: f64 = (s.dn[188][12] + s.dn[189][12]);
        let eq12_e234_d_n13: f64 = (s.dn[188][13] + s.dn[189][13]);
        let eq12_e234_d_n14: f64 = (s.dn[188][14] + s.dn[189][14]);
        let eq12_e234_d_b0: f64 = (s.db[188][0] + s.db[189][0]);
        let eq12_e234_d_b1: f64 = (s.db[188][1] + s.db[189][1]);
        let eq12_e234_d_b2: f64 = (s.db[188][2] + s.db[189][2]);
        let eq12_e234_d_b3: f64 = (s.db[188][3] + s.db[189][3]);
        let eq12_e234_d_b4: f64 = (s.db[188][4] + s.db[189][4]);
        let eq12_e234_d_b5: f64 = (s.db[188][5] + s.db[189][5]);
        let eq12_e235: f64 = (p.p148 * eq12_e234);
        let eq12_e235_d_n0: f64 = (p.p148 * eq12_e234_d_n0);
        let eq12_e235_d_n1: f64 = (p.p148 * eq12_e234_d_n1);
        let eq12_e235_d_n2: f64 = (p.p148 * eq12_e234_d_n2);
        let eq12_e235_d_n3: f64 = (p.p148 * eq12_e234_d_n3);
        let eq12_e235_d_n4: f64 = (p.p148 * eq12_e234_d_n4);
        let eq12_e235_d_n5: f64 = (p.p148 * eq12_e234_d_n5);
        let eq12_e235_d_n6: f64 = (p.p148 * eq12_e234_d_n6);
        let eq12_e235_d_n7: f64 = (p.p148 * eq12_e234_d_n7);
        let eq12_e235_d_n8: f64 = (p.p148 * eq12_e234_d_n8);
        let eq12_e235_d_n9: f64 = (p.p148 * eq12_e234_d_n9);
        let eq12_e235_d_n10: f64 = (p.p148 * eq12_e234_d_n10);
        let eq12_e235_d_n11: f64 = (p.p148 * eq12_e234_d_n11);
        let eq12_e235_d_n12: f64 = (p.p148 * eq12_e234_d_n12);
        let eq12_e235_d_n13: f64 = (p.p148 * eq12_e234_d_n13);
        let eq12_e235_d_n14: f64 = (p.p148 * eq12_e234_d_n14);
        let eq12_e235_d_b0: f64 = (p.p148 * eq12_e234_d_b0);
        let eq12_e235_d_b1: f64 = (p.p148 * eq12_e234_d_b1);
        let eq12_e235_d_b2: f64 = (p.p148 * eq12_e234_d_b2);
        let eq12_e235_d_b3: f64 = (p.p148 * eq12_e234_d_b3);
        let eq12_e235_d_b4: f64 = (p.p148 * eq12_e234_d_b4);
        let eq12_e235_d_b5: f64 = (p.p148 * eq12_e234_d_b5);
        let eq12_value: f64 = eq12_e235;
        let eq12_node_derivatives: [f64; 15] = [eq12_e235_d_n0, eq12_e235_d_n1, eq12_e235_d_n2, eq12_e235_d_n3, eq12_e235_d_n4, eq12_e235_d_n5, eq12_e235_d_n6, eq12_e235_d_n7, eq12_e235_d_n8, eq12_e235_d_n9, eq12_e235_d_n10, eq12_e235_d_n11, eq12_e235_d_n12, eq12_e235_d_n13, eq12_e235_d_n14];
        let eq12_branch_derivatives: [f64; 6] = [eq12_e235_d_b0, eq12_e235_d_b1, eq12_e235_d_b2, eq12_e235_d_b3, eq12_e235_d_b4, eq12_e235_d_b5];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e238: f64 = (p.p148 * s.v[180]);
        let eq13_e238_d_n0: f64 = (p.p148 * s.dn[180][0]);
        let eq13_e238_d_n1: f64 = (p.p148 * s.dn[180][1]);
        let eq13_e238_d_n2: f64 = (p.p148 * s.dn[180][2]);
        let eq13_e238_d_n3: f64 = (p.p148 * s.dn[180][3]);
        let eq13_e238_d_n4: f64 = (p.p148 * s.dn[180][4]);
        let eq13_e238_d_n5: f64 = (p.p148 * s.dn[180][5]);
        let eq13_e238_d_n6: f64 = (p.p148 * s.dn[180][6]);
        let eq13_e238_d_n7: f64 = (p.p148 * s.dn[180][7]);
        let eq13_e238_d_n8: f64 = (p.p148 * s.dn[180][8]);
        let eq13_e238_d_n9: f64 = (p.p148 * s.dn[180][9]);
        let eq13_e238_d_n10: f64 = (p.p148 * s.dn[180][10]);
        let eq13_e238_d_n11: f64 = (p.p148 * s.dn[180][11]);
        let eq13_e238_d_n12: f64 = (p.p148 * s.dn[180][12]);
        let eq13_e238_d_n13: f64 = (p.p148 * s.dn[180][13]);
        let eq13_e238_d_n14: f64 = (p.p148 * s.dn[180][14]);
        let eq13_e238_d_b0: f64 = (p.p148 * s.db[180][0]);
        let eq13_e238_d_b1: f64 = (p.p148 * s.db[180][1]);
        let eq13_e238_d_b2: f64 = (p.p148 * s.db[180][2]);
        let eq13_e238_d_b3: f64 = (p.p148 * s.db[180][3]);
        let eq13_e238_d_b4: f64 = (p.p148 * s.db[180][4]);
        let eq13_e238_d_b5: f64 = (p.p148 * s.db[180][5]);
        let eq13_e239: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, eq13_e238);
        let eq13_e239_d_n0: f64 = (eq13_e238_d_n0 * ddt_scale);
        let eq13_e239_d_n1: f64 = (eq13_e238_d_n1 * ddt_scale);
        let eq13_e239_d_n2: f64 = (eq13_e238_d_n2 * ddt_scale);
        let eq13_e239_d_n3: f64 = (eq13_e238_d_n3 * ddt_scale);
        let eq13_e239_d_n4: f64 = (eq13_e238_d_n4 * ddt_scale);
        let eq13_e239_d_n5: f64 = (eq13_e238_d_n5 * ddt_scale);
        let eq13_e239_d_n6: f64 = (eq13_e238_d_n6 * ddt_scale);
        let eq13_e239_d_n7: f64 = (eq13_e238_d_n7 * ddt_scale);
        let eq13_e239_d_n8: f64 = (eq13_e238_d_n8 * ddt_scale);
        let eq13_e239_d_n9: f64 = (eq13_e238_d_n9 * ddt_scale);
        let eq13_e239_d_n10: f64 = (eq13_e238_d_n10 * ddt_scale);
        let eq13_e239_d_n11: f64 = (eq13_e238_d_n11 * ddt_scale);
        let eq13_e239_d_n12: f64 = (eq13_e238_d_n12 * ddt_scale);
        let eq13_e239_d_n13: f64 = (eq13_e238_d_n13 * ddt_scale);
        let eq13_e239_d_n14: f64 = (eq13_e238_d_n14 * ddt_scale);
        let eq13_e239_d_b0: f64 = (eq13_e238_d_b0 * ddt_scale);
        let eq13_e239_d_b1: f64 = (eq13_e238_d_b1 * ddt_scale);
        let eq13_e239_d_b2: f64 = (eq13_e238_d_b2 * ddt_scale);
        let eq13_e239_d_b3: f64 = (eq13_e238_d_b3 * ddt_scale);
        let eq13_e239_d_b4: f64 = (eq13_e238_d_b4 * ddt_scale);
        let eq13_e239_d_b5: f64 = (eq13_e238_d_b5 * ddt_scale);
        let eq13_value: f64 = eq13_e239;
        let eq13_node_derivatives: [f64; 15] = [eq13_e239_d_n0, eq13_e239_d_n1, eq13_e239_d_n2, eq13_e239_d_n3, eq13_e239_d_n4, eq13_e239_d_n5, eq13_e239_d_n6, eq13_e239_d_n7, eq13_e239_d_n8, eq13_e239_d_n9, eq13_e239_d_n10, eq13_e239_d_n11, eq13_e239_d_n12, eq13_e239_d_n13, eq13_e239_d_n14];
        let eq13_branch_derivatives: [f64; 6] = [eq13_e239_d_b0, eq13_e239_d_b1, eq13_e239_d_b2, eq13_e239_d_b3, eq13_e239_d_b4, eq13_e239_d_b5];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_e242: f64 = (p.p148 * s.v[194]);
        let eq14_e242_d_n0: f64 = (p.p148 * s.dn[194][0]);
        let eq14_e242_d_n1: f64 = (p.p148 * s.dn[194][1]);
        let eq14_e242_d_n2: f64 = (p.p148 * s.dn[194][2]);
        let eq14_e242_d_n3: f64 = (p.p148 * s.dn[194][3]);
        let eq14_e242_d_n4: f64 = (p.p148 * s.dn[194][4]);
        let eq14_e242_d_n5: f64 = (p.p148 * s.dn[194][5]);
        let eq14_e242_d_n6: f64 = (p.p148 * s.dn[194][6]);
        let eq14_e242_d_n7: f64 = (p.p148 * s.dn[194][7]);
        let eq14_e242_d_n8: f64 = (p.p148 * s.dn[194][8]);
        let eq14_e242_d_n9: f64 = (p.p148 * s.dn[194][9]);
        let eq14_e242_d_n10: f64 = (p.p148 * s.dn[194][10]);
        let eq14_e242_d_n11: f64 = (p.p148 * s.dn[194][11]);
        let eq14_e242_d_n12: f64 = (p.p148 * s.dn[194][12]);
        let eq14_e242_d_n13: f64 = (p.p148 * s.dn[194][13]);
        let eq14_e242_d_n14: f64 = (p.p148 * s.dn[194][14]);
        let eq14_e242_d_b0: f64 = (p.p148 * s.db[194][0]);
        let eq14_e242_d_b1: f64 = (p.p148 * s.db[194][1]);
        let eq14_e242_d_b2: f64 = (p.p148 * s.db[194][2]);
        let eq14_e242_d_b3: f64 = (p.p148 * s.db[194][3]);
        let eq14_e242_d_b4: f64 = (p.p148 * s.db[194][4]);
        let eq14_e242_d_b5: f64 = (p.p148 * s.db[194][5]);
        let eq14_value: f64 = eq14_e242;
        let eq14_node_derivatives: [f64; 15] = [eq14_e242_d_n0, eq14_e242_d_n1, eq14_e242_d_n2, eq14_e242_d_n3, eq14_e242_d_n4, eq14_e242_d_n5, eq14_e242_d_n6, eq14_e242_d_n7, eq14_e242_d_n8, eq14_e242_d_n9, eq14_e242_d_n10, eq14_e242_d_n11, eq14_e242_d_n12, eq14_e242_d_n13, eq14_e242_d_n14];
        let eq14_branch_derivatives: [f64; 6] = [eq14_e242_d_b0, eq14_e242_d_b1, eq14_e242_d_b2, eq14_e242_d_b3, eq14_e242_d_b4, eq14_e242_d_b5];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
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
        let eq15_e248: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, eq15_e247);
        let eq15_e248_d_n0: f64 = (eq15_e247_d_n0 * ddt_scale);
        let eq15_e248_d_n1: f64 = (eq15_e247_d_n1 * ddt_scale);
        let eq15_e248_d_n2: f64 = (eq15_e247_d_n2 * ddt_scale);
        let eq15_e248_d_n3: f64 = (eq15_e247_d_n3 * ddt_scale);
        let eq15_e248_d_n4: f64 = (eq15_e247_d_n4 * ddt_scale);
        let eq15_e248_d_n5: f64 = (eq15_e247_d_n5 * ddt_scale);
        let eq15_e248_d_n6: f64 = (eq15_e247_d_n6 * ddt_scale);
        let eq15_e248_d_n7: f64 = (eq15_e247_d_n7 * ddt_scale);
        let eq15_e248_d_n8: f64 = (eq15_e247_d_n8 * ddt_scale);
        let eq15_e248_d_n9: f64 = (eq15_e247_d_n9 * ddt_scale);
        let eq15_e248_d_n10: f64 = (eq15_e247_d_n10 * ddt_scale);
        let eq15_e248_d_n11: f64 = (eq15_e247_d_n11 * ddt_scale);
        let eq15_e248_d_n12: f64 = (eq15_e247_d_n12 * ddt_scale);
        let eq15_e248_d_n13: f64 = (eq15_e247_d_n13 * ddt_scale);
        let eq15_e248_d_n14: f64 = (eq15_e247_d_n14 * ddt_scale);
        let eq15_e248_d_b0: f64 = (eq15_e247_d_b0 * ddt_scale);
        let eq15_e248_d_b1: f64 = (eq15_e247_d_b1 * ddt_scale);
        let eq15_e248_d_b2: f64 = (eq15_e247_d_b2 * ddt_scale);
        let eq15_e248_d_b3: f64 = (eq15_e247_d_b3 * ddt_scale);
        let eq15_e248_d_b4: f64 = (eq15_e247_d_b4 * ddt_scale);
        let eq15_e248_d_b5: f64 = (eq15_e247_d_b5 * ddt_scale);
        let eq15_value: f64 = eq15_e248;
        let eq15_node_derivatives: [f64; 15] = [eq15_e248_d_n0, eq15_e248_d_n1, eq15_e248_d_n2, eq15_e248_d_n3, eq15_e248_d_n4, eq15_e248_d_n5, eq15_e248_d_n6, eq15_e248_d_n7, eq15_e248_d_n8, eq15_e248_d_n9, eq15_e248_d_n10, eq15_e248_d_n11, eq15_e248_d_n12, eq15_e248_d_n13, eq15_e248_d_n14];
        let eq15_branch_derivatives: [f64; 6] = [eq15_e248_d_b0, eq15_e248_d_b1, eq15_e248_d_b2, eq15_e248_d_b3, eq15_e248_d_b4, eq15_e248_d_b5];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let eq16_e251: f64 = (s.v[172] * (nv7 - nv5));
        let eq16_e251_d_n0: f64 = (s.dn[172][0] * (nv7 - nv5));
        let eq16_e251_d_n1: f64 = (s.dn[172][1] * (nv7 - nv5));
        let eq16_e251_d_n2: f64 = (s.dn[172][2] * (nv7 - nv5));
        let eq16_e251_d_n3: f64 = (s.dn[172][3] * (nv7 - nv5));
        let eq16_e251_d_n4: f64 = (s.dn[172][4] * (nv7 - nv5));
        let eq16_e251_d_n5: f64 = ((s.dn[172][5] * (nv7 - nv5)) + (-s.v[172]));
        let eq16_e251_d_n6: f64 = (s.dn[172][6] * (nv7 - nv5));
        let eq16_e251_d_n7: f64 = ((s.dn[172][7] * (nv7 - nv5)) + s.v[172]);
        let eq16_e251_d_n8: f64 = (s.dn[172][8] * (nv7 - nv5));
        let eq16_e251_d_n9: f64 = (s.dn[172][9] * (nv7 - nv5));
        let eq16_e251_d_n10: f64 = (s.dn[172][10] * (nv7 - nv5));
        let eq16_e251_d_n11: f64 = (s.dn[172][11] * (nv7 - nv5));
        let eq16_e251_d_n12: f64 = (s.dn[172][12] * (nv7 - nv5));
        let eq16_e251_d_n13: f64 = (s.dn[172][13] * (nv7 - nv5));
        let eq16_e251_d_n14: f64 = (s.dn[172][14] * (nv7 - nv5));
        let eq16_e251_d_b0: f64 = (s.db[172][0] * (nv7 - nv5));
        let eq16_e251_d_b1: f64 = (s.db[172][1] * (nv7 - nv5));
        let eq16_e251_d_b2: f64 = (s.db[172][2] * (nv7 - nv5));
        let eq16_e251_d_b3: f64 = (s.db[172][3] * (nv7 - nv5));
        let eq16_e251_d_b4: f64 = (s.db[172][4] * (nv7 - nv5));
        let eq16_e251_d_b5: f64 = (s.db[172][5] * (nv7 - nv5));
        let eq16_e252: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq16_e251);
        let eq16_e252_d_n0: f64 = (eq16_e251_d_n0 * ddt_scale);
        let eq16_e252_d_n1: f64 = (eq16_e251_d_n1 * ddt_scale);
        let eq16_e252_d_n2: f64 = (eq16_e251_d_n2 * ddt_scale);
        let eq16_e252_d_n3: f64 = (eq16_e251_d_n3 * ddt_scale);
        let eq16_e252_d_n4: f64 = (eq16_e251_d_n4 * ddt_scale);
        let eq16_e252_d_n5: f64 = (eq16_e251_d_n5 * ddt_scale);
        let eq16_e252_d_n6: f64 = (eq16_e251_d_n6 * ddt_scale);
        let eq16_e252_d_n7: f64 = (eq16_e251_d_n7 * ddt_scale);
        let eq16_e252_d_n8: f64 = (eq16_e251_d_n8 * ddt_scale);
        let eq16_e252_d_n9: f64 = (eq16_e251_d_n9 * ddt_scale);
        let eq16_e252_d_n10: f64 = (eq16_e251_d_n10 * ddt_scale);
        let eq16_e252_d_n11: f64 = (eq16_e251_d_n11 * ddt_scale);
        let eq16_e252_d_n12: f64 = (eq16_e251_d_n12 * ddt_scale);
        let eq16_e252_d_n13: f64 = (eq16_e251_d_n13 * ddt_scale);
        let eq16_e252_d_n14: f64 = (eq16_e251_d_n14 * ddt_scale);
        let eq16_e252_d_b0: f64 = (eq16_e251_d_b0 * ddt_scale);
        let eq16_e252_d_b1: f64 = (eq16_e251_d_b1 * ddt_scale);
        let eq16_e252_d_b2: f64 = (eq16_e251_d_b2 * ddt_scale);
        let eq16_e252_d_b3: f64 = (eq16_e251_d_b3 * ddt_scale);
        let eq16_e252_d_b4: f64 = (eq16_e251_d_b4 * ddt_scale);
        let eq16_e252_d_b5: f64 = (eq16_e251_d_b5 * ddt_scale);
        let eq16_value: f64 = eq16_e252;
        let eq16_node_derivatives: [f64; 15] = [eq16_e252_d_n0, eq16_e252_d_n1, eq16_e252_d_n2, eq16_e252_d_n3, eq16_e252_d_n4, eq16_e252_d_n5, eq16_e252_d_n6, eq16_e252_d_n7, eq16_e252_d_n8, eq16_e252_d_n9, eq16_e252_d_n10, eq16_e252_d_n11, eq16_e252_d_n12, eq16_e252_d_n13, eq16_e252_d_n14];
        let eq16_branch_derivatives: [f64; 6] = [eq16_e252_d_b0, eq16_e252_d_b1, eq16_e252_d_b2, eq16_e252_d_b3, eq16_e252_d_b4, eq16_e252_d_b5];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let eq17_e255: f64 = (p.p148 * s.v[41]);
        let eq17_e255_d_n0: f64 = (p.p148 * s.dn[41][0]);
        let eq17_e255_d_n1: f64 = (p.p148 * s.dn[41][1]);
        let eq17_e255_d_n2: f64 = (p.p148 * s.dn[41][2]);
        let eq17_e255_d_n3: f64 = (p.p148 * s.dn[41][3]);
        let eq17_e255_d_n4: f64 = (p.p148 * s.dn[41][4]);
        let eq17_e255_d_n5: f64 = (p.p148 * s.dn[41][5]);
        let eq17_e255_d_n6: f64 = (p.p148 * s.dn[41][6]);
        let eq17_e255_d_n7: f64 = (p.p148 * s.dn[41][7]);
        let eq17_e255_d_n8: f64 = (p.p148 * s.dn[41][8]);
        let eq17_e255_d_n9: f64 = (p.p148 * s.dn[41][9]);
        let eq17_e255_d_n10: f64 = (p.p148 * s.dn[41][10]);
        let eq17_e255_d_n11: f64 = (p.p148 * s.dn[41][11]);
        let eq17_e255_d_n12: f64 = (p.p148 * s.dn[41][12]);
        let eq17_e255_d_n13: f64 = (p.p148 * s.dn[41][13]);
        let eq17_e255_d_n14: f64 = (p.p148 * s.dn[41][14]);
        let eq17_e255_d_b0: f64 = (p.p148 * s.db[41][0]);
        let eq17_e255_d_b1: f64 = (p.p148 * s.db[41][1]);
        let eq17_e255_d_b2: f64 = (p.p148 * s.db[41][2]);
        let eq17_e255_d_b3: f64 = (p.p148 * s.db[41][3]);
        let eq17_e255_d_b4: f64 = (p.p148 * s.db[41][4]);
        let eq17_e255_d_b5: f64 = (p.p148 * s.db[41][5]);
        let eq17_e256: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, eq17_e255);
        let eq17_e256_d_n0: f64 = (eq17_e255_d_n0 * ddt_scale);
        let eq17_e256_d_n1: f64 = (eq17_e255_d_n1 * ddt_scale);
        let eq17_e256_d_n2: f64 = (eq17_e255_d_n2 * ddt_scale);
        let eq17_e256_d_n3: f64 = (eq17_e255_d_n3 * ddt_scale);
        let eq17_e256_d_n4: f64 = (eq17_e255_d_n4 * ddt_scale);
        let eq17_e256_d_n5: f64 = (eq17_e255_d_n5 * ddt_scale);
        let eq17_e256_d_n6: f64 = (eq17_e255_d_n6 * ddt_scale);
        let eq17_e256_d_n7: f64 = (eq17_e255_d_n7 * ddt_scale);
        let eq17_e256_d_n8: f64 = (eq17_e255_d_n8 * ddt_scale);
        let eq17_e256_d_n9: f64 = (eq17_e255_d_n9 * ddt_scale);
        let eq17_e256_d_n10: f64 = (eq17_e255_d_n10 * ddt_scale);
        let eq17_e256_d_n11: f64 = (eq17_e255_d_n11 * ddt_scale);
        let eq17_e256_d_n12: f64 = (eq17_e255_d_n12 * ddt_scale);
        let eq17_e256_d_n13: f64 = (eq17_e255_d_n13 * ddt_scale);
        let eq17_e256_d_n14: f64 = (eq17_e255_d_n14 * ddt_scale);
        let eq17_e256_d_b0: f64 = (eq17_e255_d_b0 * ddt_scale);
        let eq17_e256_d_b1: f64 = (eq17_e255_d_b1 * ddt_scale);
        let eq17_e256_d_b2: f64 = (eq17_e255_d_b2 * ddt_scale);
        let eq17_e256_d_b3: f64 = (eq17_e255_d_b3 * ddt_scale);
        let eq17_e256_d_b4: f64 = (eq17_e255_d_b4 * ddt_scale);
        let eq17_e256_d_b5: f64 = (eq17_e255_d_b5 * ddt_scale);
        let eq17_value: f64 = eq17_e256;
        let eq17_node_derivatives: [f64; 15] = [eq17_e256_d_n0, eq17_e256_d_n1, eq17_e256_d_n2, eq17_e256_d_n3, eq17_e256_d_n4, eq17_e256_d_n5, eq17_e256_d_n6, eq17_e256_d_n7, eq17_e256_d_n8, eq17_e256_d_n9, eq17_e256_d_n10, eq17_e256_d_n11, eq17_e256_d_n12, eq17_e256_d_n13, eq17_e256_d_n14];
        let eq17_branch_derivatives: [f64; 6] = [eq17_e256_d_b0, eq17_e256_d_b1, eq17_e256_d_b2, eq17_e256_d_b3, eq17_e256_d_b4, eq17_e256_d_b5];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(5),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let eq18_e259: f64 = (s.v[171] * (nv1 - nv5));
        let eq18_e259_d_n0: f64 = (s.dn[171][0] * (nv1 - nv5));
        let eq18_e259_d_n1: f64 = ((s.dn[171][1] * (nv1 - nv5)) + s.v[171]);
        let eq18_e259_d_n2: f64 = (s.dn[171][2] * (nv1 - nv5));
        let eq18_e259_d_n3: f64 = (s.dn[171][3] * (nv1 - nv5));
        let eq18_e259_d_n4: f64 = (s.dn[171][4] * (nv1 - nv5));
        let eq18_e259_d_n5: f64 = ((s.dn[171][5] * (nv1 - nv5)) + (-s.v[171]));
        let eq18_e259_d_n6: f64 = (s.dn[171][6] * (nv1 - nv5));
        let eq18_e259_d_n7: f64 = (s.dn[171][7] * (nv1 - nv5));
        let eq18_e259_d_n8: f64 = (s.dn[171][8] * (nv1 - nv5));
        let eq18_e259_d_n9: f64 = (s.dn[171][9] * (nv1 - nv5));
        let eq18_e259_d_n10: f64 = (s.dn[171][10] * (nv1 - nv5));
        let eq18_e259_d_n11: f64 = (s.dn[171][11] * (nv1 - nv5));
        let eq18_e259_d_n12: f64 = (s.dn[171][12] * (nv1 - nv5));
        let eq18_e259_d_n13: f64 = (s.dn[171][13] * (nv1 - nv5));
        let eq18_e259_d_n14: f64 = (s.dn[171][14] * (nv1 - nv5));
        let eq18_e259_d_b0: f64 = (s.db[171][0] * (nv1 - nv5));
        let eq18_e259_d_b1: f64 = (s.db[171][1] * (nv1 - nv5));
        let eq18_e259_d_b2: f64 = (s.db[171][2] * (nv1 - nv5));
        let eq18_e259_d_b3: f64 = (s.db[171][3] * (nv1 - nv5));
        let eq18_e259_d_b4: f64 = (s.db[171][4] * (nv1 - nv5));
        let eq18_e259_d_b5: f64 = (s.db[171][5] * (nv1 - nv5));
        let eq18_e260: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, eq18_e259);
        let eq18_e260_d_n0: f64 = (eq18_e259_d_n0 * ddt_scale);
        let eq18_e260_d_n1: f64 = (eq18_e259_d_n1 * ddt_scale);
        let eq18_e260_d_n2: f64 = (eq18_e259_d_n2 * ddt_scale);
        let eq18_e260_d_n3: f64 = (eq18_e259_d_n3 * ddt_scale);
        let eq18_e260_d_n4: f64 = (eq18_e259_d_n4 * ddt_scale);
        let eq18_e260_d_n5: f64 = (eq18_e259_d_n5 * ddt_scale);
        let eq18_e260_d_n6: f64 = (eq18_e259_d_n6 * ddt_scale);
        let eq18_e260_d_n7: f64 = (eq18_e259_d_n7 * ddt_scale);
        let eq18_e260_d_n8: f64 = (eq18_e259_d_n8 * ddt_scale);
        let eq18_e260_d_n9: f64 = (eq18_e259_d_n9 * ddt_scale);
        let eq18_e260_d_n10: f64 = (eq18_e259_d_n10 * ddt_scale);
        let eq18_e260_d_n11: f64 = (eq18_e259_d_n11 * ddt_scale);
        let eq18_e260_d_n12: f64 = (eq18_e259_d_n12 * ddt_scale);
        let eq18_e260_d_n13: f64 = (eq18_e259_d_n13 * ddt_scale);
        let eq18_e260_d_n14: f64 = (eq18_e259_d_n14 * ddt_scale);
        let eq18_e260_d_b0: f64 = (eq18_e259_d_b0 * ddt_scale);
        let eq18_e260_d_b1: f64 = (eq18_e259_d_b1 * ddt_scale);
        let eq18_e260_d_b2: f64 = (eq18_e259_d_b2 * ddt_scale);
        let eq18_e260_d_b3: f64 = (eq18_e259_d_b3 * ddt_scale);
        let eq18_e260_d_b4: f64 = (eq18_e259_d_b4 * ddt_scale);
        let eq18_e260_d_b5: f64 = (eq18_e259_d_b5 * ddt_scale);
        let eq18_value: f64 = eq18_e260;
        let eq18_node_derivatives: [f64; 15] = [eq18_e260_d_n0, eq18_e260_d_n1, eq18_e260_d_n2, eq18_e260_d_n3, eq18_e260_d_n4, eq18_e260_d_n5, eq18_e260_d_n6, eq18_e260_d_n7, eq18_e260_d_n8, eq18_e260_d_n9, eq18_e260_d_n10, eq18_e260_d_n11, eq18_e260_d_n12, eq18_e260_d_n13, eq18_e260_d_n14];
        let eq18_branch_derivatives: [f64; 6] = [eq18_e260_d_b0, eq18_e260_d_b1, eq18_e260_d_b2, eq18_e260_d_b3, eq18_e260_d_b4, eq18_e260_d_b5];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(5),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
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
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq19_e266, eq19_e266_d_n0, eq19_e266_d_n1, eq19_e266_d_n2, eq19_e266_d_n3, eq19_e266_d_n4, eq19_e266_d_n5, eq19_e266_d_n6, eq19_e266_d_n7, eq19_e266_d_n8, eq19_e266_d_n9, eq19_e266_d_n10, eq19_e266_d_n11, eq19_e266_d_n12, eq19_e266_d_n13, eq19_e266_d_n14, eq19_e266_d_b0, eq19_e266_d_b1, eq19_e266_d_b2, eq19_e266_d_b3, eq19_e266_d_b4, eq19_e266_d_b5,) = {
    if s.b[511] {
        let eq19_e264: f64 = ((nv1 - nv7) / s.v[71]);
        let eq19_e264_d_n0: f64 = (-(((nv1 - nv7) * s.dn[71][0]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n1: f64 = ((s.v[71] - ((nv1 - nv7) * s.dn[71][1])) / (s.v[71] * s.v[71]));
        let eq19_e264_d_n2: f64 = (-(((nv1 - nv7) * s.dn[71][2]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n3: f64 = (-(((nv1 - nv7) * s.dn[71][3]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n4: f64 = (-(((nv1 - nv7) * s.dn[71][4]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n5: f64 = (-(((nv1 - nv7) * s.dn[71][5]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n6: f64 = (-(((nv1 - nv7) * s.dn[71][6]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n7: f64 = (((-s.v[71]) - ((nv1 - nv7) * s.dn[71][7])) / (s.v[71] * s.v[71]));
        let eq19_e264_d_n8: f64 = (-(((nv1 - nv7) * s.dn[71][8]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n9: f64 = (-(((nv1 - nv7) * s.dn[71][9]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n10: f64 = (-(((nv1 - nv7) * s.dn[71][10]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n11: f64 = (-(((nv1 - nv7) * s.dn[71][11]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n12: f64 = (-(((nv1 - nv7) * s.dn[71][12]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n13: f64 = (-(((nv1 - nv7) * s.dn[71][13]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n14: f64 = (-(((nv1 - nv7) * s.dn[71][14]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_b0: f64 = (-(((nv1 - nv7) * s.db[71][0]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_b1: f64 = (-(((nv1 - nv7) * s.db[71][1]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_b2: f64 = (-(((nv1 - nv7) * s.db[71][2]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_b3: f64 = (-(((nv1 - nv7) * s.db[71][3]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_b4: f64 = (-(((nv1 - nv7) * s.db[71][4]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_b5: f64 = (-(((nv1 - nv7) * s.db[71][5]) / (s.v[71] * s.v[71])));
        (eq19_e264, eq19_e264_d_n0, eq19_e264_d_n1, eq19_e264_d_n2, eq19_e264_d_n3, eq19_e264_d_n4, eq19_e264_d_n5, eq19_e264_d_n6, eq19_e264_d_n7, eq19_e264_d_n8, eq19_e264_d_n9, eq19_e264_d_n10, eq19_e264_d_n11, eq19_e264_d_n12, eq19_e264_d_n13, eq19_e264_d_n14, eq19_e264_d_b0, eq19_e264_d_b1, eq19_e264_d_b2, eq19_e264_d_b3, eq19_e264_d_b4, eq19_e264_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e266;
        let eq19_node_derivatives: [f64; 15] = [eq19_e266_d_n0, eq19_e266_d_n1, eq19_e266_d_n2, eq19_e266_d_n3, eq19_e266_d_n4, eq19_e266_d_n5, eq19_e266_d_n6, eq19_e266_d_n7, eq19_e266_d_n8, eq19_e266_d_n9, eq19_e266_d_n10, eq19_e266_d_n11, eq19_e266_d_n12, eq19_e266_d_n13, eq19_e266_d_n14];
        let eq19_branch_derivatives: [f64; 6] = [eq19_e266_d_b0, eq19_e266_d_b1, eq19_e266_d_b2, eq19_e266_d_b3, eq19_e266_d_b4, eq19_e266_d_b5];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(7),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let (eq20_e271,) = {
    if (!s.b[511]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e271;
        stamper.stamp_potential_const_local(
            1,
            eq20_value,
        );
        let (eq21_e277, eq21_e277_d_n0, eq21_e277_d_n1, eq21_e277_d_n2, eq21_e277_d_n3, eq21_e277_d_n4, eq21_e277_d_n5, eq21_e277_d_n6, eq21_e277_d_n7, eq21_e277_d_n8, eq21_e277_d_n9, eq21_e277_d_n10, eq21_e277_d_n11, eq21_e277_d_n12, eq21_e277_d_n13, eq21_e277_d_n14, eq21_e277_d_b0, eq21_e277_d_b1, eq21_e277_d_b2, eq21_e277_d_b3, eq21_e277_d_b4, eq21_e277_d_b5,) = {
    if s.b[512] {
        let eq21_e275: f64 = ((nv6 - nv2) / s.v[73]);
        let eq21_e275_d_n0: f64 = (-(((nv6 - nv2) * s.dn[73][0]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n1: f64 = (-(((nv6 - nv2) * s.dn[73][1]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n2: f64 = (((-s.v[73]) - ((nv6 - nv2) * s.dn[73][2])) / (s.v[73] * s.v[73]));
        let eq21_e275_d_n3: f64 = (-(((nv6 - nv2) * s.dn[73][3]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n4: f64 = (-(((nv6 - nv2) * s.dn[73][4]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n5: f64 = (-(((nv6 - nv2) * s.dn[73][5]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n6: f64 = ((s.v[73] - ((nv6 - nv2) * s.dn[73][6])) / (s.v[73] * s.v[73]));
        let eq21_e275_d_n7: f64 = (-(((nv6 - nv2) * s.dn[73][7]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n8: f64 = (-(((nv6 - nv2) * s.dn[73][8]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n9: f64 = (-(((nv6 - nv2) * s.dn[73][9]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n10: f64 = (-(((nv6 - nv2) * s.dn[73][10]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n11: f64 = (-(((nv6 - nv2) * s.dn[73][11]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n12: f64 = (-(((nv6 - nv2) * s.dn[73][12]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n13: f64 = (-(((nv6 - nv2) * s.dn[73][13]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n14: f64 = (-(((nv6 - nv2) * s.dn[73][14]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_b0: f64 = (-(((nv6 - nv2) * s.db[73][0]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_b1: f64 = (-(((nv6 - nv2) * s.db[73][1]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_b2: f64 = (-(((nv6 - nv2) * s.db[73][2]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_b3: f64 = (-(((nv6 - nv2) * s.db[73][3]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_b4: f64 = (-(((nv6 - nv2) * s.db[73][4]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_b5: f64 = (-(((nv6 - nv2) * s.db[73][5]) / (s.v[73] * s.v[73])));
        (eq21_e275, eq21_e275_d_n0, eq21_e275_d_n1, eq21_e275_d_n2, eq21_e275_d_n3, eq21_e275_d_n4, eq21_e275_d_n5, eq21_e275_d_n6, eq21_e275_d_n7, eq21_e275_d_n8, eq21_e275_d_n9, eq21_e275_d_n10, eq21_e275_d_n11, eq21_e275_d_n12, eq21_e275_d_n13, eq21_e275_d_n14, eq21_e275_d_b0, eq21_e275_d_b1, eq21_e275_d_b2, eq21_e275_d_b3, eq21_e275_d_b4, eq21_e275_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e277;
        let eq21_node_derivatives: [f64; 15] = [eq21_e277_d_n0, eq21_e277_d_n1, eq21_e277_d_n2, eq21_e277_d_n3, eq21_e277_d_n4, eq21_e277_d_n5, eq21_e277_d_n6, eq21_e277_d_n7, eq21_e277_d_n8, eq21_e277_d_n9, eq21_e277_d_n10, eq21_e277_d_n11, eq21_e277_d_n12, eq21_e277_d_n13, eq21_e277_d_n14];
        let eq21_branch_derivatives: [f64; 6] = [eq21_e277_d_b0, eq21_e277_d_b1, eq21_e277_d_b2, eq21_e277_d_b3, eq21_e277_d_b4, eq21_e277_d_b5];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(2),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq22_e282,) = {
    if (!s.b[512]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e282;
        stamper.stamp_potential_const_local(
            2,
            eq22_value,
        );
        let (eq23_e288, eq23_e288_d_n0, eq23_e288_d_n1, eq23_e288_d_n2, eq23_e288_d_n3, eq23_e288_d_n4, eq23_e288_d_n5, eq23_e288_d_n6, eq23_e288_d_n7, eq23_e288_d_n8, eq23_e288_d_n9, eq23_e288_d_n10, eq23_e288_d_n11, eq23_e288_d_n12, eq23_e288_d_n13, eq23_e288_d_n14, eq23_e288_d_b0, eq23_e288_d_b1, eq23_e288_d_b2, eq23_e288_d_b3, eq23_e288_d_b4, eq23_e288_d_b5,) = {
    if s.b[513] {
        let eq23_e286: f64 = ((nv5 - nv0) / s.v[72]);
        let eq23_e286_d_n0: f64 = (((-s.v[72]) - ((nv5 - nv0) * s.dn[72][0])) / (s.v[72] * s.v[72]));
        let eq23_e286_d_n1: f64 = (-(((nv5 - nv0) * s.dn[72][1]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n2: f64 = (-(((nv5 - nv0) * s.dn[72][2]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n3: f64 = (-(((nv5 - nv0) * s.dn[72][3]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n4: f64 = (-(((nv5 - nv0) * s.dn[72][4]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n5: f64 = ((s.v[72] - ((nv5 - nv0) * s.dn[72][5])) / (s.v[72] * s.v[72]));
        let eq23_e286_d_n6: f64 = (-(((nv5 - nv0) * s.dn[72][6]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n7: f64 = (-(((nv5 - nv0) * s.dn[72][7]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n8: f64 = (-(((nv5 - nv0) * s.dn[72][8]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n9: f64 = (-(((nv5 - nv0) * s.dn[72][9]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n10: f64 = (-(((nv5 - nv0) * s.dn[72][10]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n11: f64 = (-(((nv5 - nv0) * s.dn[72][11]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n12: f64 = (-(((nv5 - nv0) * s.dn[72][12]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n13: f64 = (-(((nv5 - nv0) * s.dn[72][13]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n14: f64 = (-(((nv5 - nv0) * s.dn[72][14]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_b0: f64 = (-(((nv5 - nv0) * s.db[72][0]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_b1: f64 = (-(((nv5 - nv0) * s.db[72][1]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_b2: f64 = (-(((nv5 - nv0) * s.db[72][2]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_b3: f64 = (-(((nv5 - nv0) * s.db[72][3]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_b4: f64 = (-(((nv5 - nv0) * s.db[72][4]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_b5: f64 = (-(((nv5 - nv0) * s.db[72][5]) / (s.v[72] * s.v[72])));
        (eq23_e286, eq23_e286_d_n0, eq23_e286_d_n1, eq23_e286_d_n2, eq23_e286_d_n3, eq23_e286_d_n4, eq23_e286_d_n5, eq23_e286_d_n6, eq23_e286_d_n7, eq23_e286_d_n8, eq23_e286_d_n9, eq23_e286_d_n10, eq23_e286_d_n11, eq23_e286_d_n12, eq23_e286_d_n13, eq23_e286_d_n14, eq23_e286_d_b0, eq23_e286_d_b1, eq23_e286_d_b2, eq23_e286_d_b3, eq23_e286_d_b4, eq23_e286_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e288;
        let eq23_node_derivatives: [f64; 15] = [eq23_e288_d_n0, eq23_e288_d_n1, eq23_e288_d_n2, eq23_e288_d_n3, eq23_e288_d_n4, eq23_e288_d_n5, eq23_e288_d_n6, eq23_e288_d_n7, eq23_e288_d_n8, eq23_e288_d_n9, eq23_e288_d_n10, eq23_e288_d_n11, eq23_e288_d_n12, eq23_e288_d_n13, eq23_e288_d_n14];
        let eq23_branch_derivatives: [f64; 6] = [eq23_e288_d_b0, eq23_e288_d_b1, eq23_e288_d_b2, eq23_e288_d_b3, eq23_e288_d_b4, eq23_e288_d_b5];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(0),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let (eq24_e293,) = {
    if (!s.b[513]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq24_value: f64 = eq24_e293;
        stamper.stamp_potential_const_local(
            3,
            eq24_value,
        );
        let eq25_e296: f64 = (s.v[174] * (nv7 - nv2));
        let eq25_e296_d_n2: f64 = (-s.v[174]);
        let eq25_e296_d_n7: f64 = s.v[174];
        let eq25_e297: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, eq25_e296);
        let eq25_e297_d_n2: f64 = (eq25_e296_d_n2 * ddt_scale);
        let eq25_e297_d_n7: f64 = (eq25_e296_d_n7 * ddt_scale);
        let eq25_value: f64 = eq25_e297;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(2),
            multiplicity * (eq25_value),
            2,
            multiplicity * (eq25_e297_d_n2),
            7,
            multiplicity * (eq25_e297_d_n7),
        );
        let eq26_e300: f64 = (s.v[173] * (nv1 - nv2));
        let eq26_e300_d_n1: f64 = s.v[173];
        let eq26_e300_d_n2: f64 = (-s.v[173]);
        let eq26_e301: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, eq26_e300);
        let eq26_e301_d_n1: f64 = (eq26_e300_d_n1 * ddt_scale);
        let eq26_e301_d_n2: f64 = (eq26_e300_d_n2 * ddt_scale);
        let eq26_value: f64 = eq26_e301;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (eq26_value),
            1,
            multiplicity * (eq26_e301_d_n1),
            2,
            multiplicity * (eq26_e301_d_n2),
        );
        let eq27_e304: f64 = (p.p108 * (nv0 - nv2));
        let eq27_e304_d_n0: f64 = p.p108;
        let eq27_e304_d_n2: f64 = (-p.p108);
        let eq27_e305: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, eq27_e304);
        let eq27_e305_d_n0: f64 = (eq27_e304_d_n0 * ddt_scale);
        let eq27_e305_d_n2: f64 = (eq27_e304_d_n2 * ddt_scale);
        let eq27_value: f64 = eq27_e305;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(2),
            multiplicity * (eq27_value),
            0,
            multiplicity * (eq27_e305_d_n0),
            2,
            multiplicity * (eq27_e305_d_n2),
        );
        let eq28_e308: f64 = (p.p148 * s.v[198]);
        let eq28_e308_d_n0: f64 = (p.p148 * s.dn[198][0]);
        let eq28_e308_d_n1: f64 = (p.p148 * s.dn[198][1]);
        let eq28_e308_d_n2: f64 = (p.p148 * s.dn[198][2]);
        let eq28_e308_d_n3: f64 = (p.p148 * s.dn[198][3]);
        let eq28_e308_d_n4: f64 = (p.p148 * s.dn[198][4]);
        let eq28_e308_d_n5: f64 = (p.p148 * s.dn[198][5]);
        let eq28_e308_d_n6: f64 = (p.p148 * s.dn[198][6]);
        let eq28_e308_d_n7: f64 = (p.p148 * s.dn[198][7]);
        let eq28_e308_d_n8: f64 = (p.p148 * s.dn[198][8]);
        let eq28_e308_d_n9: f64 = (p.p148 * s.dn[198][9]);
        let eq28_e308_d_n10: f64 = (p.p148 * s.dn[198][10]);
        let eq28_e308_d_n11: f64 = (p.p148 * s.dn[198][11]);
        let eq28_e308_d_n12: f64 = (p.p148 * s.dn[198][12]);
        let eq28_e308_d_n13: f64 = (p.p148 * s.dn[198][13]);
        let eq28_e308_d_n14: f64 = (p.p148 * s.dn[198][14]);
        let eq28_e308_d_b0: f64 = (p.p148 * s.db[198][0]);
        let eq28_e308_d_b1: f64 = (p.p148 * s.db[198][1]);
        let eq28_e308_d_b2: f64 = (p.p148 * s.db[198][2]);
        let eq28_e308_d_b3: f64 = (p.p148 * s.db[198][3]);
        let eq28_e308_d_b4: f64 = (p.p148 * s.db[198][4]);
        let eq28_e308_d_b5: f64 = (p.p148 * s.db[198][5]);
        let eq28_value: f64 = eq28_e308;
        let eq28_node_derivatives: [f64; 15] = [eq28_e308_d_n0, eq28_e308_d_n1, eq28_e308_d_n2, eq28_e308_d_n3, eq28_e308_d_n4, eq28_e308_d_n5, eq28_e308_d_n6, eq28_e308_d_n7, eq28_e308_d_n8, eq28_e308_d_n9, eq28_e308_d_n10, eq28_e308_d_n11, eq28_e308_d_n12, eq28_e308_d_n13, eq28_e308_d_n14];
        let eq28_branch_derivatives: [f64; 6] = [eq28_e308_d_b0, eq28_e308_d_b1, eq28_e308_d_b2, eq28_e308_d_b3, eq28_e308_d_b4, eq28_e308_d_b5];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq28_value),
            &eq28_node_derivatives,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let (eq29_e316, eq29_e316_d_n0, eq29_e316_d_n1, eq29_e316_d_n2, eq29_e316_d_n3, eq29_e316_d_n4, eq29_e316_d_n5, eq29_e316_d_n6, eq29_e316_d_n7, eq29_e316_d_n8, eq29_e316_d_n9, eq29_e316_d_n10, eq29_e316_d_n11, eq29_e316_d_n12, eq29_e316_d_n13, eq29_e316_d_n14, eq29_e316_d_b0, eq29_e316_d_b1, eq29_e316_d_b2, eq29_e316_d_b3, eq29_e316_d_b4, eq29_e316_d_b5,) = {
    if (s.b[514] && s.b[515]) {
        let eq29_e314: f64 = (p.p148 * s.v[195]);
        let eq29_e314_d_n0: f64 = (p.p148 * s.dn[195][0]);
        let eq29_e314_d_n1: f64 = (p.p148 * s.dn[195][1]);
        let eq29_e314_d_n2: f64 = (p.p148 * s.dn[195][2]);
        let eq29_e314_d_n3: f64 = (p.p148 * s.dn[195][3]);
        let eq29_e314_d_n4: f64 = (p.p148 * s.dn[195][4]);
        let eq29_e314_d_n5: f64 = (p.p148 * s.dn[195][5]);
        let eq29_e314_d_n6: f64 = (p.p148 * s.dn[195][6]);
        let eq29_e314_d_n7: f64 = (p.p148 * s.dn[195][7]);
        let eq29_e314_d_n8: f64 = (p.p148 * s.dn[195][8]);
        let eq29_e314_d_n9: f64 = (p.p148 * s.dn[195][9]);
        let eq29_e314_d_n10: f64 = (p.p148 * s.dn[195][10]);
        let eq29_e314_d_n11: f64 = (p.p148 * s.dn[195][11]);
        let eq29_e314_d_n12: f64 = (p.p148 * s.dn[195][12]);
        let eq29_e314_d_n13: f64 = (p.p148 * s.dn[195][13]);
        let eq29_e314_d_n14: f64 = (p.p148 * s.dn[195][14]);
        let eq29_e314_d_b0: f64 = (p.p148 * s.db[195][0]);
        let eq29_e314_d_b1: f64 = (p.p148 * s.db[195][1]);
        let eq29_e314_d_b2: f64 = (p.p148 * s.db[195][2]);
        let eq29_e314_d_b3: f64 = (p.p148 * s.db[195][3]);
        let eq29_e314_d_b4: f64 = (p.p148 * s.db[195][4]);
        let eq29_e314_d_b5: f64 = (p.p148 * s.db[195][5]);
        (eq29_e314, eq29_e314_d_n0, eq29_e314_d_n1, eq29_e314_d_n2, eq29_e314_d_n3, eq29_e314_d_n4, eq29_e314_d_n5, eq29_e314_d_n6, eq29_e314_d_n7, eq29_e314_d_n8, eq29_e314_d_n9, eq29_e314_d_n10, eq29_e314_d_n11, eq29_e314_d_n12, eq29_e314_d_n13, eq29_e314_d_n14, eq29_e314_d_b0, eq29_e314_d_b1, eq29_e314_d_b2, eq29_e314_d_b3, eq29_e314_d_b4, eq29_e314_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e316;
        let eq29_node_derivatives: [f64; 15] = [eq29_e316_d_n0, eq29_e316_d_n1, eq29_e316_d_n2, eq29_e316_d_n3, eq29_e316_d_n4, eq29_e316_d_n5, eq29_e316_d_n6, eq29_e316_d_n7, eq29_e316_d_n8, eq29_e316_d_n9, eq29_e316_d_n10, eq29_e316_d_n11, eq29_e316_d_n12, eq29_e316_d_n13, eq29_e316_d_n14];
        let eq29_branch_derivatives: [f64; 6] = [eq29_e316_d_b0, eq29_e316_d_b1, eq29_e316_d_b2, eq29_e316_d_b3, eq29_e316_d_b4, eq29_e316_d_b5];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq29_value),
            &eq29_node_derivatives,
            &eq29_branch_derivatives,
            multiplicity,
        );
        let (eq31_e331, eq31_e331_d_n0, eq31_e331_d_n1, eq31_e331_d_n2, eq31_e331_d_n3, eq31_e331_d_n4, eq31_e331_d_n5, eq31_e331_d_n6, eq31_e331_d_n7, eq31_e331_d_n8, eq31_e331_d_n9, eq31_e331_d_n10, eq31_e331_d_n11, eq31_e331_d_n12, eq31_e331_d_n13, eq31_e331_d_n14, eq31_e331_d_b0, eq31_e331_d_b1, eq31_e331_d_b2, eq31_e331_d_b3, eq31_e331_d_b4, eq31_e331_d_b5,) = {
    if (!s.b[514]) {
        let eq31_e329: f64 = (p.p148 * s.v[195]);
        let eq31_e329_d_n0: f64 = (p.p148 * s.dn[195][0]);
        let eq31_e329_d_n1: f64 = (p.p148 * s.dn[195][1]);
        let eq31_e329_d_n2: f64 = (p.p148 * s.dn[195][2]);
        let eq31_e329_d_n3: f64 = (p.p148 * s.dn[195][3]);
        let eq31_e329_d_n4: f64 = (p.p148 * s.dn[195][4]);
        let eq31_e329_d_n5: f64 = (p.p148 * s.dn[195][5]);
        let eq31_e329_d_n6: f64 = (p.p148 * s.dn[195][6]);
        let eq31_e329_d_n7: f64 = (p.p148 * s.dn[195][7]);
        let eq31_e329_d_n8: f64 = (p.p148 * s.dn[195][8]);
        let eq31_e329_d_n9: f64 = (p.p148 * s.dn[195][9]);
        let eq31_e329_d_n10: f64 = (p.p148 * s.dn[195][10]);
        let eq31_e329_d_n11: f64 = (p.p148 * s.dn[195][11]);
        let eq31_e329_d_n12: f64 = (p.p148 * s.dn[195][12]);
        let eq31_e329_d_n13: f64 = (p.p148 * s.dn[195][13]);
        let eq31_e329_d_n14: f64 = (p.p148 * s.dn[195][14]);
        let eq31_e329_d_b0: f64 = (p.p148 * s.db[195][0]);
        let eq31_e329_d_b1: f64 = (p.p148 * s.db[195][1]);
        let eq31_e329_d_b2: f64 = (p.p148 * s.db[195][2]);
        let eq31_e329_d_b3: f64 = (p.p148 * s.db[195][3]);
        let eq31_e329_d_b4: f64 = (p.p148 * s.db[195][4]);
        let eq31_e329_d_b5: f64 = (p.p148 * s.db[195][5]);
        (eq31_e329, eq31_e329_d_n0, eq31_e329_d_n1, eq31_e329_d_n2, eq31_e329_d_n3, eq31_e329_d_n4, eq31_e329_d_n5, eq31_e329_d_n6, eq31_e329_d_n7, eq31_e329_d_n8, eq31_e329_d_n9, eq31_e329_d_n10, eq31_e329_d_n11, eq31_e329_d_n12, eq31_e329_d_n13, eq31_e329_d_n14, eq31_e329_d_b0, eq31_e329_d_b1, eq31_e329_d_b2, eq31_e329_d_b3, eq31_e329_d_b4, eq31_e329_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e331;
        let eq31_node_derivatives: [f64; 15] = [eq31_e331_d_n0, eq31_e331_d_n1, eq31_e331_d_n2, eq31_e331_d_n3, eq31_e331_d_n4, eq31_e331_d_n5, eq31_e331_d_n6, eq31_e331_d_n7, eq31_e331_d_n8, eq31_e331_d_n9, eq31_e331_d_n10, eq31_e331_d_n11, eq31_e331_d_n12, eq31_e331_d_n13, eq31_e331_d_n14];
        let eq31_branch_derivatives: [f64; 6] = [eq31_e331_d_b0, eq31_e331_d_b1, eq31_e331_d_b2, eq31_e331_d_b3, eq31_e331_d_b4, eq31_e331_d_b5];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let eq33_e343: f64 = (p.p148 * s.v[196]);
        let eq33_e343_d_n0: f64 = (p.p148 * s.dn[196][0]);
        let eq33_e343_d_n1: f64 = (p.p148 * s.dn[196][1]);
        let eq33_e343_d_n2: f64 = (p.p148 * s.dn[196][2]);
        let eq33_e343_d_n3: f64 = (p.p148 * s.dn[196][3]);
        let eq33_e343_d_n4: f64 = (p.p148 * s.dn[196][4]);
        let eq33_e343_d_n5: f64 = (p.p148 * s.dn[196][5]);
        let eq33_e343_d_n6: f64 = (p.p148 * s.dn[196][6]);
        let eq33_e343_d_n7: f64 = (p.p148 * s.dn[196][7]);
        let eq33_e343_d_n8: f64 = (p.p148 * s.dn[196][8]);
        let eq33_e343_d_n9: f64 = (p.p148 * s.dn[196][9]);
        let eq33_e343_d_n10: f64 = (p.p148 * s.dn[196][10]);
        let eq33_e343_d_n11: f64 = (p.p148 * s.dn[196][11]);
        let eq33_e343_d_n12: f64 = (p.p148 * s.dn[196][12]);
        let eq33_e343_d_n13: f64 = (p.p148 * s.dn[196][13]);
        let eq33_e343_d_n14: f64 = (p.p148 * s.dn[196][14]);
        let eq33_e343_d_b0: f64 = (p.p148 * s.db[196][0]);
        let eq33_e343_d_b1: f64 = (p.p148 * s.db[196][1]);
        let eq33_e343_d_b2: f64 = (p.p148 * s.db[196][2]);
        let eq33_e343_d_b3: f64 = (p.p148 * s.db[196][3]);
        let eq33_e343_d_b4: f64 = (p.p148 * s.db[196][4]);
        let eq33_e343_d_b5: f64 = (p.p148 * s.db[196][5]);
        let eq33_e344: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, eq33_e343);
        let eq33_e344_d_n0: f64 = (eq33_e343_d_n0 * ddt_scale);
        let eq33_e344_d_n1: f64 = (eq33_e343_d_n1 * ddt_scale);
        let eq33_e344_d_n2: f64 = (eq33_e343_d_n2 * ddt_scale);
        let eq33_e344_d_n3: f64 = (eq33_e343_d_n3 * ddt_scale);
        let eq33_e344_d_n4: f64 = (eq33_e343_d_n4 * ddt_scale);
        let eq33_e344_d_n5: f64 = (eq33_e343_d_n5 * ddt_scale);
        let eq33_e344_d_n6: f64 = (eq33_e343_d_n6 * ddt_scale);
        let eq33_e344_d_n7: f64 = (eq33_e343_d_n7 * ddt_scale);
        let eq33_e344_d_n8: f64 = (eq33_e343_d_n8 * ddt_scale);
        let eq33_e344_d_n9: f64 = (eq33_e343_d_n9 * ddt_scale);
        let eq33_e344_d_n10: f64 = (eq33_e343_d_n10 * ddt_scale);
        let eq33_e344_d_n11: f64 = (eq33_e343_d_n11 * ddt_scale);
        let eq33_e344_d_n12: f64 = (eq33_e343_d_n12 * ddt_scale);
        let eq33_e344_d_n13: f64 = (eq33_e343_d_n13 * ddt_scale);
        let eq33_e344_d_n14: f64 = (eq33_e343_d_n14 * ddt_scale);
        let eq33_e344_d_b0: f64 = (eq33_e343_d_b0 * ddt_scale);
        let eq33_e344_d_b1: f64 = (eq33_e343_d_b1 * ddt_scale);
        let eq33_e344_d_b2: f64 = (eq33_e343_d_b2 * ddt_scale);
        let eq33_e344_d_b3: f64 = (eq33_e343_d_b3 * ddt_scale);
        let eq33_e344_d_b4: f64 = (eq33_e343_d_b4 * ddt_scale);
        let eq33_e344_d_b5: f64 = (eq33_e343_d_b5 * ddt_scale);
        let eq33_value: f64 = eq33_e344;
        let eq33_node_derivatives: [f64; 15] = [eq33_e344_d_n0, eq33_e344_d_n1, eq33_e344_d_n2, eq33_e344_d_n3, eq33_e344_d_n4, eq33_e344_d_n5, eq33_e344_d_n6, eq33_e344_d_n7, eq33_e344_d_n8, eq33_e344_d_n9, eq33_e344_d_n10, eq33_e344_d_n11, eq33_e344_d_n12, eq33_e344_d_n13, eq33_e344_d_n14];
        let eq33_branch_derivatives: [f64; 6] = [eq33_e344_d_b0, eq33_e344_d_b1, eq33_e344_d_b2, eq33_e344_d_b3, eq33_e344_d_b4, eq33_e344_d_b5];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let eq34_e347: f64 = (p.p148 * s.v[197]);
        let eq34_e347_d_n0: f64 = (p.p148 * s.dn[197][0]);
        let eq34_e347_d_n1: f64 = (p.p148 * s.dn[197][1]);
        let eq34_e347_d_n2: f64 = (p.p148 * s.dn[197][2]);
        let eq34_e347_d_n3: f64 = (p.p148 * s.dn[197][3]);
        let eq34_e347_d_n4: f64 = (p.p148 * s.dn[197][4]);
        let eq34_e347_d_n5: f64 = (p.p148 * s.dn[197][5]);
        let eq34_e347_d_n6: f64 = (p.p148 * s.dn[197][6]);
        let eq34_e347_d_n7: f64 = (p.p148 * s.dn[197][7]);
        let eq34_e347_d_n8: f64 = (p.p148 * s.dn[197][8]);
        let eq34_e347_d_n9: f64 = (p.p148 * s.dn[197][9]);
        let eq34_e347_d_n10: f64 = (p.p148 * s.dn[197][10]);
        let eq34_e347_d_n11: f64 = (p.p148 * s.dn[197][11]);
        let eq34_e347_d_n12: f64 = (p.p148 * s.dn[197][12]);
        let eq34_e347_d_n13: f64 = (p.p148 * s.dn[197][13]);
        let eq34_e347_d_n14: f64 = (p.p148 * s.dn[197][14]);
        let eq34_e347_d_b0: f64 = (p.p148 * s.db[197][0]);
        let eq34_e347_d_b1: f64 = (p.p148 * s.db[197][1]);
        let eq34_e347_d_b2: f64 = (p.p148 * s.db[197][2]);
        let eq34_e347_d_b3: f64 = (p.p148 * s.db[197][3]);
        let eq34_e347_d_b4: f64 = (p.p148 * s.db[197][4]);
        let eq34_e347_d_b5: f64 = (p.p148 * s.db[197][5]);
        let eq34_e348: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 12, eq34_e347);
        let eq34_e348_d_n0: f64 = (eq34_e347_d_n0 * ddt_scale);
        let eq34_e348_d_n1: f64 = (eq34_e347_d_n1 * ddt_scale);
        let eq34_e348_d_n2: f64 = (eq34_e347_d_n2 * ddt_scale);
        let eq34_e348_d_n3: f64 = (eq34_e347_d_n3 * ddt_scale);
        let eq34_e348_d_n4: f64 = (eq34_e347_d_n4 * ddt_scale);
        let eq34_e348_d_n5: f64 = (eq34_e347_d_n5 * ddt_scale);
        let eq34_e348_d_n6: f64 = (eq34_e347_d_n6 * ddt_scale);
        let eq34_e348_d_n7: f64 = (eq34_e347_d_n7 * ddt_scale);
        let eq34_e348_d_n8: f64 = (eq34_e347_d_n8 * ddt_scale);
        let eq34_e348_d_n9: f64 = (eq34_e347_d_n9 * ddt_scale);
        let eq34_e348_d_n10: f64 = (eq34_e347_d_n10 * ddt_scale);
        let eq34_e348_d_n11: f64 = (eq34_e347_d_n11 * ddt_scale);
        let eq34_e348_d_n12: f64 = (eq34_e347_d_n12 * ddt_scale);
        let eq34_e348_d_n13: f64 = (eq34_e347_d_n13 * ddt_scale);
        let eq34_e348_d_n14: f64 = (eq34_e347_d_n14 * ddt_scale);
        let eq34_e348_d_b0: f64 = (eq34_e347_d_b0 * ddt_scale);
        let eq34_e348_d_b1: f64 = (eq34_e347_d_b1 * ddt_scale);
        let eq34_e348_d_b2: f64 = (eq34_e347_d_b2 * ddt_scale);
        let eq34_e348_d_b3: f64 = (eq34_e347_d_b3 * ddt_scale);
        let eq34_e348_d_b4: f64 = (eq34_e347_d_b4 * ddt_scale);
        let eq34_e348_d_b5: f64 = (eq34_e347_d_b5 * ddt_scale);
        let eq34_value: f64 = eq34_e348;
        let eq34_node_derivatives: [f64; 15] = [eq34_e348_d_n0, eq34_e348_d_n1, eq34_e348_d_n2, eq34_e348_d_n3, eq34_e348_d_n4, eq34_e348_d_n5, eq34_e348_d_n6, eq34_e348_d_n7, eq34_e348_d_n8, eq34_e348_d_n9, eq34_e348_d_n10, eq34_e348_d_n11, eq34_e348_d_n12, eq34_e348_d_n13, eq34_e348_d_n14];
        let eq34_branch_derivatives: [f64; 6] = [eq34_e348_d_b0, eq34_e348_d_b1, eq34_e348_d_b2, eq34_e348_d_b3, eq34_e348_d_b4, eq34_e348_d_b5];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(0),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let (eq36_e363, eq36_e363_d_n3, eq36_e363_d_n9,) = {
    if (s.b[517] && s.b[518]) {
        let eq36_e360: f64 = (p.p103 * (nv9 - nv3));
        let eq36_e360_d_n3: f64 = (-p.p103);
        let eq36_e360_d_n9: f64 = p.p103;
        let eq36_e361: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 13, eq36_e360);
        let eq36_e361_d_n3: f64 = (eq36_e360_d_n3 * ddt_scale);
        let eq36_e361_d_n9: f64 = (eq36_e360_d_n9 * ddt_scale);
        (eq36_e361, eq36_e361_d_n3, eq36_e361_d_n9,)
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
        let (eq37_e368,) = {
    if (!s.b[517]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq37_value: f64 = eq37_e368;
        stamper.stamp_potential_const_local(
            4,
            eq37_value,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq38_e376, eq38_e376_d_n0, eq38_e376_d_n1, eq38_e376_d_n2, eq38_e376_d_n3, eq38_e376_d_n4, eq38_e376_d_n5, eq38_e376_d_n6, eq38_e376_d_n7, eq38_e376_d_n8, eq38_e376_d_n9, eq38_e376_d_n10, eq38_e376_d_n11, eq38_e376_d_n12, eq38_e376_d_n13, eq38_e376_d_n14, eq38_e376_d_b0, eq38_e376_d_b1, eq38_e376_d_b2, eq38_e376_d_b3, eq38_e376_d_b4, eq38_e376_d_b5,) = {
    if s.b[519] {
        let eq38_e372: f64 = ((nv4 - 0.0) / s.v[201]);
        let eq38_e372_d_n0: f64 = (-(((nv4 - 0.0) * s.dn[201][0]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n1: f64 = (-(((nv4 - 0.0) * s.dn[201][1]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n2: f64 = (-(((nv4 - 0.0) * s.dn[201][2]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n3: f64 = (-(((nv4 - 0.0) * s.dn[201][3]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n4: f64 = ((s.v[201] - ((nv4 - 0.0) * s.dn[201][4])) / (s.v[201] * s.v[201]));
        let eq38_e372_d_n5: f64 = (-(((nv4 - 0.0) * s.dn[201][5]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n6: f64 = (-(((nv4 - 0.0) * s.dn[201][6]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n7: f64 = (-(((nv4 - 0.0) * s.dn[201][7]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n8: f64 = (-(((nv4 - 0.0) * s.dn[201][8]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n9: f64 = (-(((nv4 - 0.0) * s.dn[201][9]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n10: f64 = (-(((nv4 - 0.0) * s.dn[201][10]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n11: f64 = (-(((nv4 - 0.0) * s.dn[201][11]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n12: f64 = (-(((nv4 - 0.0) * s.dn[201][12]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n13: f64 = (-(((nv4 - 0.0) * s.dn[201][13]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_n14: f64 = (-(((nv4 - 0.0) * s.dn[201][14]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_b0: f64 = (-(((nv4 - 0.0) * s.db[201][0]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_b1: f64 = (-(((nv4 - 0.0) * s.db[201][1]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_b2: f64 = (-(((nv4 - 0.0) * s.db[201][2]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_b3: f64 = (-(((nv4 - 0.0) * s.db[201][3]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_b4: f64 = (-(((nv4 - 0.0) * s.db[201][4]) / (s.v[201] * s.v[201])));
        let eq38_e372_d_b5: f64 = (-(((nv4 - 0.0) * s.db[201][5]) / (s.v[201] * s.v[201])));
        let eq38_e374: f64 = (eq38_e372 - s.v[200]);
        let eq38_e374_d_n0: f64 = (eq38_e372_d_n0 - s.dn[200][0]);
        let eq38_e374_d_n1: f64 = (eq38_e372_d_n1 - s.dn[200][1]);
        let eq38_e374_d_n2: f64 = (eq38_e372_d_n2 - s.dn[200][2]);
        let eq38_e374_d_n3: f64 = (eq38_e372_d_n3 - s.dn[200][3]);
        let eq38_e374_d_n4: f64 = (eq38_e372_d_n4 - s.dn[200][4]);
        let eq38_e374_d_n5: f64 = (eq38_e372_d_n5 - s.dn[200][5]);
        let eq38_e374_d_n6: f64 = (eq38_e372_d_n6 - s.dn[200][6]);
        let eq38_e374_d_n7: f64 = (eq38_e372_d_n7 - s.dn[200][7]);
        let eq38_e374_d_n8: f64 = (eq38_e372_d_n8 - s.dn[200][8]);
        let eq38_e374_d_n9: f64 = (eq38_e372_d_n9 - s.dn[200][9]);
        let eq38_e374_d_n10: f64 = (eq38_e372_d_n10 - s.dn[200][10]);
        let eq38_e374_d_n11: f64 = (eq38_e372_d_n11 - s.dn[200][11]);
        let eq38_e374_d_n12: f64 = (eq38_e372_d_n12 - s.dn[200][12]);
        let eq38_e374_d_n13: f64 = (eq38_e372_d_n13 - s.dn[200][13]);
        let eq38_e374_d_n14: f64 = (eq38_e372_d_n14 - s.dn[200][14]);
        let eq38_e374_d_b0: f64 = (eq38_e372_d_b0 - s.db[200][0]);
        let eq38_e374_d_b1: f64 = (eq38_e372_d_b1 - s.db[200][1]);
        let eq38_e374_d_b2: f64 = (eq38_e372_d_b2 - s.db[200][2]);
        let eq38_e374_d_b3: f64 = (eq38_e372_d_b3 - s.db[200][3]);
        let eq38_e374_d_b4: f64 = (eq38_e372_d_b4 - s.db[200][4]);
        let eq38_e374_d_b5: f64 = (eq38_e372_d_b5 - s.db[200][5]);
        (eq38_e374, eq38_e374_d_n0, eq38_e374_d_n1, eq38_e374_d_n2, eq38_e374_d_n3, eq38_e374_d_n4, eq38_e374_d_n5, eq38_e374_d_n6, eq38_e374_d_n7, eq38_e374_d_n8, eq38_e374_d_n9, eq38_e374_d_n10, eq38_e374_d_n11, eq38_e374_d_n12, eq38_e374_d_n13, eq38_e374_d_n14, eq38_e374_d_b0, eq38_e374_d_b1, eq38_e374_d_b2, eq38_e374_d_b3, eq38_e374_d_b4, eq38_e374_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e376;
        let eq38_node_derivatives: [f64; 15] = [eq38_e376_d_n0, eq38_e376_d_n1, eq38_e376_d_n2, eq38_e376_d_n3, eq38_e376_d_n4, eq38_e376_d_n5, eq38_e376_d_n6, eq38_e376_d_n7, eq38_e376_d_n8, eq38_e376_d_n9, eq38_e376_d_n10, eq38_e376_d_n11, eq38_e376_d_n12, eq38_e376_d_n13, eq38_e376_d_n14];
        let eq38_branch_derivatives: [f64; 6] = [eq38_e376_d_b0, eq38_e376_d_b1, eq38_e376_d_b2, eq38_e376_d_b3, eq38_e376_d_b4, eq38_e376_d_b5];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq38_value),
            &eq38_node_derivatives,
            &eq38_branch_derivatives,
            multiplicity,
        );
        let (eq39_e385, eq39_e385_d_n4,) = {
    if (s.b[519] && s.b[520]) {
        let eq39_e382: f64 = (p.p145 * (nv4 - 0.0));
        let eq39_e382_d_n4: f64 = p.p145;
        let eq39_e383: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 14, eq39_e382);
        let eq39_e383_d_n4: f64 = (eq39_e382_d_n4 * ddt_scale);
        (eq39_e383, eq39_e383_d_n4,)
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
        let (eq40_e390,) = {
    if (!s.b[519]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq40_value: f64 = eq40_e390;
        stamper.stamp_potential_const_local(
            5,
            eq40_value,
        );
        let eq41_value: f64 = s.v[237];
        let eq41_node_derivatives: [f64; 15] = [s.dn[237][0], s.dn[237][1], s.dn[237][2], s.dn[237][3], s.dn[237][4], s.dn[237][5], s.dn[237][6], s.dn[237][7], s.dn[237][8], s.dn[237][9], s.dn[237][10], s.dn[237][11], s.dn[237][12], s.dn[237][13], s.dn[237][14]];
        let eq41_branch_derivatives: [f64; 6] = [s.db[237][0], s.db[237][1], s.db[237][2], s.db[237][3], s.db[237][4], s.db[237][5]];
        stamper.stamp_current_dense_local(
            Some(10),
            None,
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let eq42_e393: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 15, s.v[239]);
        let eq42_e393_d_n0: f64 = (s.dn[239][0] * ddt_scale);
        let eq42_e393_d_n1: f64 = (s.dn[239][1] * ddt_scale);
        let eq42_e393_d_n2: f64 = (s.dn[239][2] * ddt_scale);
        let eq42_e393_d_n3: f64 = (s.dn[239][3] * ddt_scale);
        let eq42_e393_d_n4: f64 = (s.dn[239][4] * ddt_scale);
        let eq42_e393_d_n5: f64 = (s.dn[239][5] * ddt_scale);
        let eq42_e393_d_n6: f64 = (s.dn[239][6] * ddt_scale);
        let eq42_e393_d_n7: f64 = (s.dn[239][7] * ddt_scale);
        let eq42_e393_d_n8: f64 = (s.dn[239][8] * ddt_scale);
        let eq42_e393_d_n9: f64 = (s.dn[239][9] * ddt_scale);
        let eq42_e393_d_n10: f64 = (s.dn[239][10] * ddt_scale);
        let eq42_e393_d_n11: f64 = (s.dn[239][11] * ddt_scale);
        let eq42_e393_d_n12: f64 = (s.dn[239][12] * ddt_scale);
        let eq42_e393_d_n13: f64 = (s.dn[239][13] * ddt_scale);
        let eq42_e393_d_n14: f64 = (s.dn[239][14] * ddt_scale);
        let eq42_e393_d_b0: f64 = (s.db[239][0] * ddt_scale);
        let eq42_e393_d_b1: f64 = (s.db[239][1] * ddt_scale);
        let eq42_e393_d_b2: f64 = (s.db[239][2] * ddt_scale);
        let eq42_e393_d_b3: f64 = (s.db[239][3] * ddt_scale);
        let eq42_e393_d_b4: f64 = (s.db[239][4] * ddt_scale);
        let eq42_e393_d_b5: f64 = (s.db[239][5] * ddt_scale);
        let eq42_value: f64 = eq42_e393;
        let eq42_node_derivatives: [f64; 15] = [eq42_e393_d_n0, eq42_e393_d_n1, eq42_e393_d_n2, eq42_e393_d_n3, eq42_e393_d_n4, eq42_e393_d_n5, eq42_e393_d_n6, eq42_e393_d_n7, eq42_e393_d_n8, eq42_e393_d_n9, eq42_e393_d_n10, eq42_e393_d_n11, eq42_e393_d_n12, eq42_e393_d_n13, eq42_e393_d_n14];
        let eq42_branch_derivatives: [f64; 6] = [eq42_e393_d_b0, eq42_e393_d_b1, eq42_e393_d_b2, eq42_e393_d_b3, eq42_e393_d_b4, eq42_e393_d_b5];
        stamper.stamp_current_dense_local(
            Some(10),
            None,
            multiplicity * (eq42_value),
            &eq42_node_derivatives,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let eq43_value: f64 = s.v[238];
        let eq43_node_derivatives: [f64; 15] = [s.dn[238][0], s.dn[238][1], s.dn[238][2], s.dn[238][3], s.dn[238][4], s.dn[238][5], s.dn[238][6], s.dn[238][7], s.dn[238][8], s.dn[238][9], s.dn[238][10], s.dn[238][11], s.dn[238][12], s.dn[238][13], s.dn[238][14]];
        let eq43_branch_derivatives: [f64; 6] = [s.db[238][0], s.db[238][1], s.db[238][2], s.db[238][3], s.db[238][4], s.db[238][5]];
        stamper.stamp_current_dense_local(
            Some(11),
            None,
            multiplicity * (eq43_value),
            &eq43_node_derivatives,
            &eq43_branch_derivatives,
            multiplicity,
        );
        let eq44_e396: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 16, s.v[240]);
        let eq44_e396_d_n0: f64 = (s.dn[240][0] * ddt_scale);
        let eq44_e396_d_n1: f64 = (s.dn[240][1] * ddt_scale);
        let eq44_e396_d_n2: f64 = (s.dn[240][2] * ddt_scale);
        let eq44_e396_d_n3: f64 = (s.dn[240][3] * ddt_scale);
        let eq44_e396_d_n4: f64 = (s.dn[240][4] * ddt_scale);
        let eq44_e396_d_n5: f64 = (s.dn[240][5] * ddt_scale);
        let eq44_e396_d_n6: f64 = (s.dn[240][6] * ddt_scale);
        let eq44_e396_d_n7: f64 = (s.dn[240][7] * ddt_scale);
        let eq44_e396_d_n8: f64 = (s.dn[240][8] * ddt_scale);
        let eq44_e396_d_n9: f64 = (s.dn[240][9] * ddt_scale);
        let eq44_e396_d_n10: f64 = (s.dn[240][10] * ddt_scale);
        let eq44_e396_d_n11: f64 = (s.dn[240][11] * ddt_scale);
        let eq44_e396_d_n12: f64 = (s.dn[240][12] * ddt_scale);
        let eq44_e396_d_n13: f64 = (s.dn[240][13] * ddt_scale);
        let eq44_e396_d_n14: f64 = (s.dn[240][14] * ddt_scale);
        let eq44_e396_d_b0: f64 = (s.db[240][0] * ddt_scale);
        let eq44_e396_d_b1: f64 = (s.db[240][1] * ddt_scale);
        let eq44_e396_d_b2: f64 = (s.db[240][2] * ddt_scale);
        let eq44_e396_d_b3: f64 = (s.db[240][3] * ddt_scale);
        let eq44_e396_d_b4: f64 = (s.db[240][4] * ddt_scale);
        let eq44_e396_d_b5: f64 = (s.db[240][5] * ddt_scale);
        let eq44_value: f64 = eq44_e396;
        let eq44_node_derivatives: [f64; 15] = [eq44_e396_d_n0, eq44_e396_d_n1, eq44_e396_d_n2, eq44_e396_d_n3, eq44_e396_d_n4, eq44_e396_d_n5, eq44_e396_d_n6, eq44_e396_d_n7, eq44_e396_d_n8, eq44_e396_d_n9, eq44_e396_d_n10, eq44_e396_d_n11, eq44_e396_d_n12, eq44_e396_d_n13, eq44_e396_d_n14];
        let eq44_branch_derivatives: [f64; 6] = [eq44_e396_d_b0, eq44_e396_d_b1, eq44_e396_d_b2, eq44_e396_d_b3, eq44_e396_d_b4, eq44_e396_d_b5];
        stamper.stamp_current_dense_local(
            Some(11),
            None,
            multiplicity * (eq44_value),
            &eq44_node_derivatives,
            &eq44_branch_derivatives,
            multiplicity,
        );
        let eq45_value: f64 = s.v[235];
        let eq45_node_derivatives: [f64; 15] = [s.dn[235][0], s.dn[235][1], s.dn[235][2], s.dn[235][3], s.dn[235][4], s.dn[235][5], s.dn[235][6], s.dn[235][7], s.dn[235][8], s.dn[235][9], s.dn[235][10], s.dn[235][11], s.dn[235][12], s.dn[235][13], s.dn[235][14]];
        let eq45_branch_derivatives: [f64; 6] = [s.db[235][0], s.db[235][1], s.db[235][2], s.db[235][3], s.db[235][4], s.db[235][5]];
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq45_value),
            &eq45_node_derivatives,
            &eq45_branch_derivatives,
            multiplicity,
        );
        let eq46_e399: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 17, s.v[236]);
        let eq46_e399_d_n0: f64 = (s.dn[236][0] * ddt_scale);
        let eq46_e399_d_n1: f64 = (s.dn[236][1] * ddt_scale);
        let eq46_e399_d_n2: f64 = (s.dn[236][2] * ddt_scale);
        let eq46_e399_d_n3: f64 = (s.dn[236][3] * ddt_scale);
        let eq46_e399_d_n4: f64 = (s.dn[236][4] * ddt_scale);
        let eq46_e399_d_n5: f64 = (s.dn[236][5] * ddt_scale);
        let eq46_e399_d_n6: f64 = (s.dn[236][6] * ddt_scale);
        let eq46_e399_d_n7: f64 = (s.dn[236][7] * ddt_scale);
        let eq46_e399_d_n8: f64 = (s.dn[236][8] * ddt_scale);
        let eq46_e399_d_n9: f64 = (s.dn[236][9] * ddt_scale);
        let eq46_e399_d_n10: f64 = (s.dn[236][10] * ddt_scale);
        let eq46_e399_d_n11: f64 = (s.dn[236][11] * ddt_scale);
        let eq46_e399_d_n12: f64 = (s.dn[236][12] * ddt_scale);
        let eq46_e399_d_n13: f64 = (s.dn[236][13] * ddt_scale);
        let eq46_e399_d_n14: f64 = (s.dn[236][14] * ddt_scale);
        let eq46_e399_d_b0: f64 = (s.db[236][0] * ddt_scale);
        let eq46_e399_d_b1: f64 = (s.db[236][1] * ddt_scale);
        let eq46_e399_d_b2: f64 = (s.db[236][2] * ddt_scale);
        let eq46_e399_d_b3: f64 = (s.db[236][3] * ddt_scale);
        let eq46_e399_d_b4: f64 = (s.db[236][4] * ddt_scale);
        let eq46_e399_d_b5: f64 = (s.db[236][5] * ddt_scale);
        let eq46_value: f64 = eq46_e399;
        let eq46_node_derivatives: [f64; 15] = [eq46_e399_d_n0, eq46_e399_d_n1, eq46_e399_d_n2, eq46_e399_d_n3, eq46_e399_d_n4, eq46_e399_d_n5, eq46_e399_d_n6, eq46_e399_d_n7, eq46_e399_d_n8, eq46_e399_d_n9, eq46_e399_d_n10, eq46_e399_d_n11, eq46_e399_d_n12, eq46_e399_d_n13, eq46_e399_d_n14];
        let eq46_branch_derivatives: [f64; 6] = [eq46_e399_d_b0, eq46_e399_d_b1, eq46_e399_d_b2, eq46_e399_d_b3, eq46_e399_d_b4, eq46_e399_d_b5];
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq46_value),
            &eq46_node_derivatives,
            &eq46_branch_derivatives,
            multiplicity,
        );
        let (eq47_e407,) = {
    if s.b[525] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq47_value: f64 = eq47_e407;
        stamper.stamp_current_const_local(
            Some(1),
            Some(7),
            multiplicity * (eq47_value),
        );
        let (eq48_e415,) = {
    if s.b[526] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq48_value: f64 = eq48_e415;
        stamper.stamp_current_const_local(
            Some(7),
            Some(8),
            multiplicity * (eq48_value),
        );
        let (eq49_e423,) = {
    if s.b[527] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e423;
        stamper.stamp_current_const_local(
            Some(5),
            Some(0),
            multiplicity * (eq49_value),
        );
        let (eq50_e431,) = {
    if s.b[528] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq50_value: f64 = eq50_e431;
        stamper.stamp_current_const_local(
            Some(6),
            Some(2),
            multiplicity * (eq50_value),
        );
        let (eq51_e439,) = {
    if s.b[529] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq51_value: f64 = eq51_e439;
        stamper.stamp_current_const_local(
            Some(9),
            Some(3),
            multiplicity * (eq51_value),
        );
        let (eq52_e446,) = {
    if s.b[530] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq52_value: f64 = eq52_e446;
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (eq52_value),
        );
        let (eq53_e454,) = {
    if (!s.b[530]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq53_value: f64 = eq53_e454;
        stamper.stamp_current_const_local(
            Some(7),
            Some(6),
            multiplicity * (eq53_value),
        );
        let (eq54_e461,) = {
    if s.b[531] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq54_value: f64 = eq54_e461;
        stamper.stamp_current_const_local(
            Some(6),
            Some(2),
            multiplicity * (eq54_value),
        );
        let (eq55_e470,) = {
    if s.b[532] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq55_value: f64 = eq55_e470;
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (eq55_value),
        );
        let eq56_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(7),
            Some(6),
            multiplicity * (eq56_value),
        );
        let eq57_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(8),
            multiplicity * (eq57_value),
        );
        let eq58_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(8),
            Some(5),
            multiplicity * (eq58_value),
        );
        let eq59_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(8),
            Some(5),
            multiplicity * (eq59_value),
        );
        let eq60_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(7),
            Some(5),
            multiplicity * (eq60_value),
        );
        let eq61_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(9),
            Some(5),
            multiplicity * (eq61_value),
        );
        let (eq62_e514,) = {
    if s.b[533] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq62_value: f64 = eq62_e514;
        stamper.stamp_current_const_local(
            Some(13),
            None,
            multiplicity * (eq62_value),
        );
        let (eq65_e534, eq65_e534_d_n0, eq65_e534_d_n1, eq65_e534_d_n2, eq65_e534_d_n3, eq65_e534_d_n4, eq65_e534_d_n5, eq65_e534_d_n6, eq65_e534_d_n7, eq65_e534_d_n8, eq65_e534_d_n9, eq65_e534_d_n10, eq65_e534_d_n11, eq65_e534_d_n12, eq65_e534_d_n13, eq65_e534_d_n14, eq65_e534_d_b0, eq65_e534_d_b1, eq65_e534_d_b2, eq65_e534_d_b3, eq65_e534_d_b4, eq65_e534_d_b5,) = {
    if s.b[533] {
        let eq65_e527: f64 = (s.v[537] / s.v[535]);
        let eq65_e527_d_n0: f64 = (((s.dn[537][0] * s.v[535]) - (s.v[537] * s.dn[535][0])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n1: f64 = (((s.dn[537][1] * s.v[535]) - (s.v[537] * s.dn[535][1])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n2: f64 = (((s.dn[537][2] * s.v[535]) - (s.v[537] * s.dn[535][2])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n3: f64 = (((s.dn[537][3] * s.v[535]) - (s.v[537] * s.dn[535][3])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n4: f64 = (((s.dn[537][4] * s.v[535]) - (s.v[537] * s.dn[535][4])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n5: f64 = (((s.dn[537][5] * s.v[535]) - (s.v[537] * s.dn[535][5])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n6: f64 = (((s.dn[537][6] * s.v[535]) - (s.v[537] * s.dn[535][6])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n7: f64 = (((s.dn[537][7] * s.v[535]) - (s.v[537] * s.dn[535][7])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n8: f64 = (((s.dn[537][8] * s.v[535]) - (s.v[537] * s.dn[535][8])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n9: f64 = (((s.dn[537][9] * s.v[535]) - (s.v[537] * s.dn[535][9])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n10: f64 = (((s.dn[537][10] * s.v[535]) - (s.v[537] * s.dn[535][10])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n11: f64 = (((s.dn[537][11] * s.v[535]) - (s.v[537] * s.dn[535][11])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n12: f64 = (((s.dn[537][12] * s.v[535]) - (s.v[537] * s.dn[535][12])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n13: f64 = (((s.dn[537][13] * s.v[535]) - (s.v[537] * s.dn[535][13])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n14: f64 = (((s.dn[537][14] * s.v[535]) - (s.v[537] * s.dn[535][14])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_b0: f64 = (((s.db[537][0] * s.v[535]) - (s.v[537] * s.db[535][0])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_b1: f64 = (((s.db[537][1] * s.v[535]) - (s.v[537] * s.db[535][1])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_b2: f64 = (((s.db[537][2] * s.v[535]) - (s.v[537] * s.db[535][2])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_b3: f64 = (((s.db[537][3] * s.v[535]) - (s.v[537] * s.db[535][3])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_b4: f64 = (((s.db[537][4] * s.v[535]) - (s.v[537] * s.db[535][4])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_b5: f64 = (((s.db[537][5] * s.v[535]) - (s.v[537] * s.db[535][5])) / (s.v[535] * s.v[535]));
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
        let eq65_e531: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 18, eq65_e530);
        let eq65_e531_d_n0: f64 = (eq65_e530_d_n0 * ddt_scale);
        let eq65_e531_d_n1: f64 = (eq65_e530_d_n1 * ddt_scale);
        let eq65_e531_d_n2: f64 = (eq65_e530_d_n2 * ddt_scale);
        let eq65_e531_d_n3: f64 = (eq65_e530_d_n3 * ddt_scale);
        let eq65_e531_d_n4: f64 = (eq65_e530_d_n4 * ddt_scale);
        let eq65_e531_d_n5: f64 = (eq65_e530_d_n5 * ddt_scale);
        let eq65_e531_d_n6: f64 = (eq65_e530_d_n6 * ddt_scale);
        let eq65_e531_d_n7: f64 = (eq65_e530_d_n7 * ddt_scale);
        let eq65_e531_d_n8: f64 = (eq65_e530_d_n8 * ddt_scale);
        let eq65_e531_d_n9: f64 = (eq65_e530_d_n9 * ddt_scale);
        let eq65_e531_d_n10: f64 = (eq65_e530_d_n10 * ddt_scale);
        let eq65_e531_d_n11: f64 = (eq65_e530_d_n11 * ddt_scale);
        let eq65_e531_d_n12: f64 = (eq65_e530_d_n12 * ddt_scale);
        let eq65_e531_d_n13: f64 = (eq65_e530_d_n13 * ddt_scale);
        let eq65_e531_d_n14: f64 = (eq65_e530_d_n14 * ddt_scale);
        let eq65_e531_d_b0: f64 = (eq65_e530_d_b0 * ddt_scale);
        let eq65_e531_d_b1: f64 = (eq65_e530_d_b1 * ddt_scale);
        let eq65_e531_d_b2: f64 = (eq65_e530_d_b2 * ddt_scale);
        let eq65_e531_d_b3: f64 = (eq65_e530_d_b3 * ddt_scale);
        let eq65_e531_d_b4: f64 = (eq65_e530_d_b4 * ddt_scale);
        let eq65_e531_d_b5: f64 = (eq65_e530_d_b5 * ddt_scale);
        let eq65_e532: f64 = (eq65_e527 * eq65_e531);
        let eq65_e532_d_n0: f64 = ((eq65_e527_d_n0 * eq65_e531) + (eq65_e527 * eq65_e531_d_n0));
        let eq65_e532_d_n1: f64 = ((eq65_e527_d_n1 * eq65_e531) + (eq65_e527 * eq65_e531_d_n1));
        let eq65_e532_d_n2: f64 = ((eq65_e527_d_n2 * eq65_e531) + (eq65_e527 * eq65_e531_d_n2));
        let eq65_e532_d_n3: f64 = ((eq65_e527_d_n3 * eq65_e531) + (eq65_e527 * eq65_e531_d_n3));
        let eq65_e532_d_n4: f64 = ((eq65_e527_d_n4 * eq65_e531) + (eq65_e527 * eq65_e531_d_n4));
        let eq65_e532_d_n5: f64 = ((eq65_e527_d_n5 * eq65_e531) + (eq65_e527 * eq65_e531_d_n5));
        let eq65_e532_d_n6: f64 = ((eq65_e527_d_n6 * eq65_e531) + (eq65_e527 * eq65_e531_d_n6));
        let eq65_e532_d_n7: f64 = ((eq65_e527_d_n7 * eq65_e531) + (eq65_e527 * eq65_e531_d_n7));
        let eq65_e532_d_n8: f64 = ((eq65_e527_d_n8 * eq65_e531) + (eq65_e527 * eq65_e531_d_n8));
        let eq65_e532_d_n9: f64 = ((eq65_e527_d_n9 * eq65_e531) + (eq65_e527 * eq65_e531_d_n9));
        let eq65_e532_d_n10: f64 = ((eq65_e527_d_n10 * eq65_e531) + (eq65_e527 * eq65_e531_d_n10));
        let eq65_e532_d_n11: f64 = ((eq65_e527_d_n11 * eq65_e531) + (eq65_e527 * eq65_e531_d_n11));
        let eq65_e532_d_n12: f64 = ((eq65_e527_d_n12 * eq65_e531) + (eq65_e527 * eq65_e531_d_n12));
        let eq65_e532_d_n13: f64 = ((eq65_e527_d_n13 * eq65_e531) + (eq65_e527 * eq65_e531_d_n13));
        let eq65_e532_d_n14: f64 = ((eq65_e527_d_n14 * eq65_e531) + (eq65_e527 * eq65_e531_d_n14));
        let eq65_e532_d_b0: f64 = ((eq65_e527_d_b0 * eq65_e531) + (eq65_e527 * eq65_e531_d_b0));
        let eq65_e532_d_b1: f64 = ((eq65_e527_d_b1 * eq65_e531) + (eq65_e527 * eq65_e531_d_b1));
        let eq65_e532_d_b2: f64 = ((eq65_e527_d_b2 * eq65_e531) + (eq65_e527 * eq65_e531_d_b2));
        let eq65_e532_d_b3: f64 = ((eq65_e527_d_b3 * eq65_e531) + (eq65_e527 * eq65_e531_d_b3));
        let eq65_e532_d_b4: f64 = ((eq65_e527_d_b4 * eq65_e531) + (eq65_e527 * eq65_e531_d_b4));
        let eq65_e532_d_b5: f64 = ((eq65_e527_d_b5 * eq65_e531) + (eq65_e527 * eq65_e531_d_b5));
        (eq65_e532, eq65_e532_d_n0, eq65_e532_d_n1, eq65_e532_d_n2, eq65_e532_d_n3, eq65_e532_d_n4, eq65_e532_d_n5, eq65_e532_d_n6, eq65_e532_d_n7, eq65_e532_d_n8, eq65_e532_d_n9, eq65_e532_d_n10, eq65_e532_d_n11, eq65_e532_d_n12, eq65_e532_d_n13, eq65_e532_d_n14, eq65_e532_d_b0, eq65_e532_d_b1, eq65_e532_d_b2, eq65_e532_d_b3, eq65_e532_d_b4, eq65_e532_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e534;
        let eq65_node_derivatives: [f64; 15] = [eq65_e534_d_n0, eq65_e534_d_n1, eq65_e534_d_n2, eq65_e534_d_n3, eq65_e534_d_n4, eq65_e534_d_n5, eq65_e534_d_n6, eq65_e534_d_n7, eq65_e534_d_n8, eq65_e534_d_n9, eq65_e534_d_n10, eq65_e534_d_n11, eq65_e534_d_n12, eq65_e534_d_n13, eq65_e534_d_n14];
        let eq65_branch_derivatives: [f64; 6] = [eq65_e534_d_b0, eq65_e534_d_b1, eq65_e534_d_b2, eq65_e534_d_b3, eq65_e534_d_b4, eq65_e534_d_b5];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq65_value),
            &eq65_node_derivatives,
            &eq65_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq66_e545, eq66_e545_d_n0, eq66_e545_d_n1, eq66_e545_d_n2, eq66_e545_d_n3, eq66_e545_d_n4, eq66_e545_d_n5, eq66_e545_d_n6, eq66_e545_d_n7, eq66_e545_d_n8, eq66_e545_d_n9, eq66_e545_d_n10, eq66_e545_d_n11, eq66_e545_d_n12, eq66_e545_d_n13, eq66_e545_d_n14, eq66_e545_d_b0, eq66_e545_d_b1, eq66_e545_d_b2, eq66_e545_d_b3, eq66_e545_d_b4, eq66_e545_d_b5,) = {
    if s.b[533] {
        let eq66_e538: f64 = (s.v[536] / s.v[535]);
        let eq66_e538_d_n0: f64 = (((s.dn[536][0] * s.v[535]) - (s.v[536] * s.dn[535][0])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n1: f64 = (((s.dn[536][1] * s.v[535]) - (s.v[536] * s.dn[535][1])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n2: f64 = (((s.dn[536][2] * s.v[535]) - (s.v[536] * s.dn[535][2])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n3: f64 = (((s.dn[536][3] * s.v[535]) - (s.v[536] * s.dn[535][3])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n4: f64 = (((s.dn[536][4] * s.v[535]) - (s.v[536] * s.dn[535][4])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n5: f64 = (((s.dn[536][5] * s.v[535]) - (s.v[536] * s.dn[535][5])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n6: f64 = (((s.dn[536][6] * s.v[535]) - (s.v[536] * s.dn[535][6])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n7: f64 = (((s.dn[536][7] * s.v[535]) - (s.v[536] * s.dn[535][7])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n8: f64 = (((s.dn[536][8] * s.v[535]) - (s.v[536] * s.dn[535][8])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n9: f64 = (((s.dn[536][9] * s.v[535]) - (s.v[536] * s.dn[535][9])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n10: f64 = (((s.dn[536][10] * s.v[535]) - (s.v[536] * s.dn[535][10])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n11: f64 = (((s.dn[536][11] * s.v[535]) - (s.v[536] * s.dn[535][11])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n12: f64 = (((s.dn[536][12] * s.v[535]) - (s.v[536] * s.dn[535][12])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n13: f64 = (((s.dn[536][13] * s.v[535]) - (s.v[536] * s.dn[535][13])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n14: f64 = (((s.dn[536][14] * s.v[535]) - (s.v[536] * s.dn[535][14])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_b0: f64 = (((s.db[536][0] * s.v[535]) - (s.v[536] * s.db[535][0])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_b1: f64 = (((s.db[536][1] * s.v[535]) - (s.v[536] * s.db[535][1])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_b2: f64 = (((s.db[536][2] * s.v[535]) - (s.v[536] * s.db[535][2])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_b3: f64 = (((s.db[536][3] * s.v[535]) - (s.v[536] * s.db[535][3])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_b4: f64 = (((s.db[536][4] * s.v[535]) - (s.v[536] * s.db[535][4])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_b5: f64 = (((s.db[536][5] * s.v[535]) - (s.v[536] * s.db[535][5])) / (s.v[535] * s.v[535]));
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
        let eq66_e542: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 19, eq66_e541);
        let eq66_e542_d_n0: f64 = (eq66_e541_d_n0 * ddt_scale);
        let eq66_e542_d_n1: f64 = (eq66_e541_d_n1 * ddt_scale);
        let eq66_e542_d_n2: f64 = (eq66_e541_d_n2 * ddt_scale);
        let eq66_e542_d_n3: f64 = (eq66_e541_d_n3 * ddt_scale);
        let eq66_e542_d_n4: f64 = (eq66_e541_d_n4 * ddt_scale);
        let eq66_e542_d_n5: f64 = (eq66_e541_d_n5 * ddt_scale);
        let eq66_e542_d_n6: f64 = (eq66_e541_d_n6 * ddt_scale);
        let eq66_e542_d_n7: f64 = (eq66_e541_d_n7 * ddt_scale);
        let eq66_e542_d_n8: f64 = (eq66_e541_d_n8 * ddt_scale);
        let eq66_e542_d_n9: f64 = (eq66_e541_d_n9 * ddt_scale);
        let eq66_e542_d_n10: f64 = (eq66_e541_d_n10 * ddt_scale);
        let eq66_e542_d_n11: f64 = (eq66_e541_d_n11 * ddt_scale);
        let eq66_e542_d_n12: f64 = (eq66_e541_d_n12 * ddt_scale);
        let eq66_e542_d_n13: f64 = (eq66_e541_d_n13 * ddt_scale);
        let eq66_e542_d_n14: f64 = (eq66_e541_d_n14 * ddt_scale);
        let eq66_e542_d_b0: f64 = (eq66_e541_d_b0 * ddt_scale);
        let eq66_e542_d_b1: f64 = (eq66_e541_d_b1 * ddt_scale);
        let eq66_e542_d_b2: f64 = (eq66_e541_d_b2 * ddt_scale);
        let eq66_e542_d_b3: f64 = (eq66_e541_d_b3 * ddt_scale);
        let eq66_e542_d_b4: f64 = (eq66_e541_d_b4 * ddt_scale);
        let eq66_e542_d_b5: f64 = (eq66_e541_d_b5 * ddt_scale);
        let eq66_e543: f64 = (eq66_e538 * eq66_e542);
        let eq66_e543_d_n0: f64 = ((eq66_e538_d_n0 * eq66_e542) + (eq66_e538 * eq66_e542_d_n0));
        let eq66_e543_d_n1: f64 = ((eq66_e538_d_n1 * eq66_e542) + (eq66_e538 * eq66_e542_d_n1));
        let eq66_e543_d_n2: f64 = ((eq66_e538_d_n2 * eq66_e542) + (eq66_e538 * eq66_e542_d_n2));
        let eq66_e543_d_n3: f64 = ((eq66_e538_d_n3 * eq66_e542) + (eq66_e538 * eq66_e542_d_n3));
        let eq66_e543_d_n4: f64 = ((eq66_e538_d_n4 * eq66_e542) + (eq66_e538 * eq66_e542_d_n4));
        let eq66_e543_d_n5: f64 = ((eq66_e538_d_n5 * eq66_e542) + (eq66_e538 * eq66_e542_d_n5));
        let eq66_e543_d_n6: f64 = ((eq66_e538_d_n6 * eq66_e542) + (eq66_e538 * eq66_e542_d_n6));
        let eq66_e543_d_n7: f64 = ((eq66_e538_d_n7 * eq66_e542) + (eq66_e538 * eq66_e542_d_n7));
        let eq66_e543_d_n8: f64 = ((eq66_e538_d_n8 * eq66_e542) + (eq66_e538 * eq66_e542_d_n8));
        let eq66_e543_d_n9: f64 = ((eq66_e538_d_n9 * eq66_e542) + (eq66_e538 * eq66_e542_d_n9));
        let eq66_e543_d_n10: f64 = ((eq66_e538_d_n10 * eq66_e542) + (eq66_e538 * eq66_e542_d_n10));
        let eq66_e543_d_n11: f64 = ((eq66_e538_d_n11 * eq66_e542) + (eq66_e538 * eq66_e542_d_n11));
        let eq66_e543_d_n12: f64 = ((eq66_e538_d_n12 * eq66_e542) + (eq66_e538 * eq66_e542_d_n12));
        let eq66_e543_d_n13: f64 = ((eq66_e538_d_n13 * eq66_e542) + (eq66_e538 * eq66_e542_d_n13));
        let eq66_e543_d_n14: f64 = ((eq66_e538_d_n14 * eq66_e542) + (eq66_e538 * eq66_e542_d_n14));
        let eq66_e543_d_b0: f64 = ((eq66_e538_d_b0 * eq66_e542) + (eq66_e538 * eq66_e542_d_b0));
        let eq66_e543_d_b1: f64 = ((eq66_e538_d_b1 * eq66_e542) + (eq66_e538 * eq66_e542_d_b1));
        let eq66_e543_d_b2: f64 = ((eq66_e538_d_b2 * eq66_e542) + (eq66_e538 * eq66_e542_d_b2));
        let eq66_e543_d_b3: f64 = ((eq66_e538_d_b3 * eq66_e542) + (eq66_e538 * eq66_e542_d_b3));
        let eq66_e543_d_b4: f64 = ((eq66_e538_d_b4 * eq66_e542) + (eq66_e538 * eq66_e542_d_b4));
        let eq66_e543_d_b5: f64 = ((eq66_e538_d_b5 * eq66_e542) + (eq66_e538 * eq66_e542_d_b5));
        (eq66_e543, eq66_e543_d_n0, eq66_e543_d_n1, eq66_e543_d_n2, eq66_e543_d_n3, eq66_e543_d_n4, eq66_e543_d_n5, eq66_e543_d_n6, eq66_e543_d_n7, eq66_e543_d_n8, eq66_e543_d_n9, eq66_e543_d_n10, eq66_e543_d_n11, eq66_e543_d_n12, eq66_e543_d_n13, eq66_e543_d_n14, eq66_e543_d_b0, eq66_e543_d_b1, eq66_e543_d_b2, eq66_e543_d_b3, eq66_e543_d_b4, eq66_e543_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e545;
        let eq66_node_derivatives: [f64; 15] = [eq66_e545_d_n0, eq66_e545_d_n1, eq66_e545_d_n2, eq66_e545_d_n3, eq66_e545_d_n4, eq66_e545_d_n5, eq66_e545_d_n6, eq66_e545_d_n7, eq66_e545_d_n8, eq66_e545_d_n9, eq66_e545_d_n10, eq66_e545_d_n11, eq66_e545_d_n12, eq66_e545_d_n13, eq66_e545_d_n14];
        let eq66_branch_derivatives: [f64; 6] = [eq66_e545_d_b0, eq66_e545_d_b1, eq66_e545_d_b2, eq66_e545_d_b3, eq66_e545_d_b4, eq66_e545_d_b5];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq66_value),
            &eq66_node_derivatives,
            &eq66_branch_derivatives,
            multiplicity,
        );
        let (eq67_e554,) = {
    if s.b[533] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq67_value: f64 = eq67_e554;
        stamper.stamp_current_const_local(
            Some(14),
            None,
            multiplicity * (eq67_value),
        );
        let (eq70_e573,) = {
    if (!s.b[533]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq70_value: f64 = eq70_e573;
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (eq70_value),
        );
        let (eq71_e583,) = {
    if (!s.b[533]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq71_value: f64 = eq71_e583;
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (eq71_value),
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
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
        let (eq7_e206, eq7_e206_d_n0, eq7_e206_d_n1, eq7_e206_d_n2, eq7_e206_d_n3, eq7_e206_d_n4, eq7_e206_d_n5, eq7_e206_d_n6, eq7_e206_d_n7, eq7_e206_d_n8, eq7_e206_d_n9, eq7_e206_d_n10, eq7_e206_d_n11, eq7_e206_d_n12, eq7_e206_d_n13, eq7_e206_d_n14, eq7_e206_d_b0, eq7_e206_d_b1, eq7_e206_d_b2, eq7_e206_d_b3, eq7_e206_d_b4, eq7_e206_d_b5, eq7_e206_q, eq7_e206_q_d_n0, eq7_e206_q_d_n1, eq7_e206_q_d_n2, eq7_e206_q_d_n3, eq7_e206_q_d_n4, eq7_e206_q_d_n5, eq7_e206_q_d_n6, eq7_e206_q_d_n7, eq7_e206_q_d_n8, eq7_e206_q_d_n9, eq7_e206_q_d_n10, eq7_e206_q_d_n11, eq7_e206_q_d_n12, eq7_e206_q_d_n13, eq7_e206_q_d_n14, eq7_e206_q_d_b0, eq7_e206_q_d_b1, eq7_e206_q_d_b2, eq7_e206_q_d_b3, eq7_e206_q_d_b4, eq7_e206_q_d_b5,) = {
    if (s.b[508] && s.b[509]) {
        let eq7_e204_q: f64 = s.v[183];
        (s.v[183], s.dn[183][0], s.dn[183][1], s.dn[183][2], s.dn[183][3], s.dn[183][4], s.dn[183][5], s.dn[183][6], s.dn[183][7], s.dn[183][8], s.dn[183][9], s.dn[183][10], s.dn[183][11], s.dn[183][12], s.dn[183][13], s.dn[183][14], s.db[183][0], s.db[183][1], s.db[183][2], s.db[183][3], s.db[183][4], s.db[183][5], eq7_e204_q, s.dn[183][0], s.dn[183][1], s.dn[183][2], s.dn[183][3], s.dn[183][4], s.dn[183][5], s.dn[183][6], s.dn[183][7], s.dn[183][8], s.dn[183][9], s.dn[183][10], s.dn[183][11], s.dn[183][12], s.dn[183][13], s.dn[183][14], s.db[183][0], s.db[183][1], s.db[183][2], s.db[183][3], s.db[183][4], s.db[183][5],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 15] = [eq7_e206_q_d_n0, eq7_e206_q_d_n1, eq7_e206_q_d_n2, eq7_e206_q_d_n3, eq7_e206_q_d_n4, eq7_e206_q_d_n5, eq7_e206_q_d_n6, eq7_e206_q_d_n7, eq7_e206_q_d_n8, eq7_e206_q_d_n9, eq7_e206_q_d_n10, eq7_e206_q_d_n11, eq7_e206_q_d_n12, eq7_e206_q_d_n13, eq7_e206_q_d_n14];
        let eq7_reactive_branch_derivatives: [f64; 6] = [eq7_e206_q_d_b0, eq7_e206_q_d_b1, eq7_e206_q_d_b2, eq7_e206_q_d_b3, eq7_e206_q_d_b4, eq7_e206_q_d_b5];
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
        let eq13_e238_d_n0: f64 = (p.p148 * s.dn[180][0]);
        let eq13_e238_d_n1: f64 = (p.p148 * s.dn[180][1]);
        let eq13_e238_d_n2: f64 = (p.p148 * s.dn[180][2]);
        let eq13_e238_d_n3: f64 = (p.p148 * s.dn[180][3]);
        let eq13_e238_d_n4: f64 = (p.p148 * s.dn[180][4]);
        let eq13_e238_d_n5: f64 = (p.p148 * s.dn[180][5]);
        let eq13_e238_d_n6: f64 = (p.p148 * s.dn[180][6]);
        let eq13_e238_d_n7: f64 = (p.p148 * s.dn[180][7]);
        let eq13_e238_d_n8: f64 = (p.p148 * s.dn[180][8]);
        let eq13_e238_d_n9: f64 = (p.p148 * s.dn[180][9]);
        let eq13_e238_d_n10: f64 = (p.p148 * s.dn[180][10]);
        let eq13_e238_d_n11: f64 = (p.p148 * s.dn[180][11]);
        let eq13_e238_d_n12: f64 = (p.p148 * s.dn[180][12]);
        let eq13_e238_d_n13: f64 = (p.p148 * s.dn[180][13]);
        let eq13_e238_d_n14: f64 = (p.p148 * s.dn[180][14]);
        let eq13_e238_d_b0: f64 = (p.p148 * s.db[180][0]);
        let eq13_e238_d_b1: f64 = (p.p148 * s.db[180][1]);
        let eq13_e238_d_b2: f64 = (p.p148 * s.db[180][2]);
        let eq13_e238_d_b3: f64 = (p.p148 * s.db[180][3]);
        let eq13_e238_d_b4: f64 = (p.p148 * s.db[180][4]);
        let eq13_e238_d_b5: f64 = (p.p148 * s.db[180][5]);
        let eq13_e239_q: f64 = eq13_e238;
        let eq13_reactive_node_derivatives: [f64; 15] = [eq13_e238_d_n0, eq13_e238_d_n1, eq13_e238_d_n2, eq13_e238_d_n3, eq13_e238_d_n4, eq13_e238_d_n5, eq13_e238_d_n6, eq13_e238_d_n7, eq13_e238_d_n8, eq13_e238_d_n9, eq13_e238_d_n10, eq13_e238_d_n11, eq13_e238_d_n12, eq13_e238_d_n13, eq13_e238_d_n14];
        let eq13_reactive_branch_derivatives: [f64; 6] = [eq13_e238_d_b0, eq13_e238_d_b1, eq13_e238_d_b2, eq13_e238_d_b3, eq13_e238_d_b4, eq13_e238_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
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
        let eq16_e251: f64 = (s.v[172] * (nv7 - nv5));
        let eq16_e251_d_n0: f64 = (s.dn[172][0] * (nv7 - nv5));
        let eq16_e251_d_n1: f64 = (s.dn[172][1] * (nv7 - nv5));
        let eq16_e251_d_n2: f64 = (s.dn[172][2] * (nv7 - nv5));
        let eq16_e251_d_n3: f64 = (s.dn[172][3] * (nv7 - nv5));
        let eq16_e251_d_n4: f64 = (s.dn[172][4] * (nv7 - nv5));
        let eq16_e251_d_n5: f64 = ((s.dn[172][5] * (nv7 - nv5)) + (-s.v[172]));
        let eq16_e251_d_n6: f64 = (s.dn[172][6] * (nv7 - nv5));
        let eq16_e251_d_n7: f64 = ((s.dn[172][7] * (nv7 - nv5)) + s.v[172]);
        let eq16_e251_d_n8: f64 = (s.dn[172][8] * (nv7 - nv5));
        let eq16_e251_d_n9: f64 = (s.dn[172][9] * (nv7 - nv5));
        let eq16_e251_d_n10: f64 = (s.dn[172][10] * (nv7 - nv5));
        let eq16_e251_d_n11: f64 = (s.dn[172][11] * (nv7 - nv5));
        let eq16_e251_d_n12: f64 = (s.dn[172][12] * (nv7 - nv5));
        let eq16_e251_d_n13: f64 = (s.dn[172][13] * (nv7 - nv5));
        let eq16_e251_d_n14: f64 = (s.dn[172][14] * (nv7 - nv5));
        let eq16_e251_d_b0: f64 = (s.db[172][0] * (nv7 - nv5));
        let eq16_e251_d_b1: f64 = (s.db[172][1] * (nv7 - nv5));
        let eq16_e251_d_b2: f64 = (s.db[172][2] * (nv7 - nv5));
        let eq16_e251_d_b3: f64 = (s.db[172][3] * (nv7 - nv5));
        let eq16_e251_d_b4: f64 = (s.db[172][4] * (nv7 - nv5));
        let eq16_e251_d_b5: f64 = (s.db[172][5] * (nv7 - nv5));
        let eq16_e252_q: f64 = eq16_e251;
        let eq16_reactive_node_derivatives: [f64; 15] = [eq16_e251_d_n0, eq16_e251_d_n1, eq16_e251_d_n2, eq16_e251_d_n3, eq16_e251_d_n4, eq16_e251_d_n5, eq16_e251_d_n6, eq16_e251_d_n7, eq16_e251_d_n8, eq16_e251_d_n9, eq16_e251_d_n10, eq16_e251_d_n11, eq16_e251_d_n12, eq16_e251_d_n13, eq16_e251_d_n14];
        let eq16_reactive_branch_derivatives: [f64; 6] = [eq16_e251_d_b0, eq16_e251_d_b1, eq16_e251_d_b2, eq16_e251_d_b3, eq16_e251_d_b4, eq16_e251_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq16_reactive_node_derivatives,
            branches,
            &eq16_reactive_branch_derivatives,
            multiplicity,
        );
        let eq17_e255: f64 = (p.p148 * s.v[41]);
        let eq17_e255_d_n0: f64 = (p.p148 * s.dn[41][0]);
        let eq17_e255_d_n1: f64 = (p.p148 * s.dn[41][1]);
        let eq17_e255_d_n2: f64 = (p.p148 * s.dn[41][2]);
        let eq17_e255_d_n3: f64 = (p.p148 * s.dn[41][3]);
        let eq17_e255_d_n4: f64 = (p.p148 * s.dn[41][4]);
        let eq17_e255_d_n5: f64 = (p.p148 * s.dn[41][5]);
        let eq17_e255_d_n6: f64 = (p.p148 * s.dn[41][6]);
        let eq17_e255_d_n7: f64 = (p.p148 * s.dn[41][7]);
        let eq17_e255_d_n8: f64 = (p.p148 * s.dn[41][8]);
        let eq17_e255_d_n9: f64 = (p.p148 * s.dn[41][9]);
        let eq17_e255_d_n10: f64 = (p.p148 * s.dn[41][10]);
        let eq17_e255_d_n11: f64 = (p.p148 * s.dn[41][11]);
        let eq17_e255_d_n12: f64 = (p.p148 * s.dn[41][12]);
        let eq17_e255_d_n13: f64 = (p.p148 * s.dn[41][13]);
        let eq17_e255_d_n14: f64 = (p.p148 * s.dn[41][14]);
        let eq17_e255_d_b0: f64 = (p.p148 * s.db[41][0]);
        let eq17_e255_d_b1: f64 = (p.p148 * s.db[41][1]);
        let eq17_e255_d_b2: f64 = (p.p148 * s.db[41][2]);
        let eq17_e255_d_b3: f64 = (p.p148 * s.db[41][3]);
        let eq17_e255_d_b4: f64 = (p.p148 * s.db[41][4]);
        let eq17_e255_d_b5: f64 = (p.p148 * s.db[41][5]);
        let eq17_e256_q: f64 = eq17_e255;
        let eq17_reactive_node_derivatives: [f64; 15] = [eq17_e255_d_n0, eq17_e255_d_n1, eq17_e255_d_n2, eq17_e255_d_n3, eq17_e255_d_n4, eq17_e255_d_n5, eq17_e255_d_n6, eq17_e255_d_n7, eq17_e255_d_n8, eq17_e255_d_n9, eq17_e255_d_n10, eq17_e255_d_n11, eq17_e255_d_n12, eq17_e255_d_n13, eq17_e255_d_n14];
        let eq17_reactive_branch_derivatives: [f64; 6] = [eq17_e255_d_b0, eq17_e255_d_b1, eq17_e255_d_b2, eq17_e255_d_b3, eq17_e255_d_b4, eq17_e255_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            nodes,
            &eq17_reactive_node_derivatives,
            branches,
            &eq17_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e259: f64 = (s.v[171] * (nv1 - nv5));
        let eq18_e259_d_n0: f64 = (s.dn[171][0] * (nv1 - nv5));
        let eq18_e259_d_n1: f64 = ((s.dn[171][1] * (nv1 - nv5)) + s.v[171]);
        let eq18_e259_d_n2: f64 = (s.dn[171][2] * (nv1 - nv5));
        let eq18_e259_d_n3: f64 = (s.dn[171][3] * (nv1 - nv5));
        let eq18_e259_d_n4: f64 = (s.dn[171][4] * (nv1 - nv5));
        let eq18_e259_d_n5: f64 = ((s.dn[171][5] * (nv1 - nv5)) + (-s.v[171]));
        let eq18_e259_d_n6: f64 = (s.dn[171][6] * (nv1 - nv5));
        let eq18_e259_d_n7: f64 = (s.dn[171][7] * (nv1 - nv5));
        let eq18_e259_d_n8: f64 = (s.dn[171][8] * (nv1 - nv5));
        let eq18_e259_d_n9: f64 = (s.dn[171][9] * (nv1 - nv5));
        let eq18_e259_d_n10: f64 = (s.dn[171][10] * (nv1 - nv5));
        let eq18_e259_d_n11: f64 = (s.dn[171][11] * (nv1 - nv5));
        let eq18_e259_d_n12: f64 = (s.dn[171][12] * (nv1 - nv5));
        let eq18_e259_d_n13: f64 = (s.dn[171][13] * (nv1 - nv5));
        let eq18_e259_d_n14: f64 = (s.dn[171][14] * (nv1 - nv5));
        let eq18_e259_d_b0: f64 = (s.db[171][0] * (nv1 - nv5));
        let eq18_e259_d_b1: f64 = (s.db[171][1] * (nv1 - nv5));
        let eq18_e259_d_b2: f64 = (s.db[171][2] * (nv1 - nv5));
        let eq18_e259_d_b3: f64 = (s.db[171][3] * (nv1 - nv5));
        let eq18_e259_d_b4: f64 = (s.db[171][4] * (nv1 - nv5));
        let eq18_e259_d_b5: f64 = (s.db[171][5] * (nv1 - nv5));
        let eq18_e260_q: f64 = eq18_e259;
        let eq18_reactive_node_derivatives: [f64; 15] = [eq18_e259_d_n0, eq18_e259_d_n1, eq18_e259_d_n2, eq18_e259_d_n3, eq18_e259_d_n4, eq18_e259_d_n5, eq18_e259_d_n6, eq18_e259_d_n7, eq18_e259_d_n8, eq18_e259_d_n9, eq18_e259_d_n10, eq18_e259_d_n11, eq18_e259_d_n12, eq18_e259_d_n13, eq18_e259_d_n14];
        let eq18_reactive_branch_derivatives: [f64; 6] = [eq18_e259_d_b0, eq18_e259_d_b1, eq18_e259_d_b2, eq18_e259_d_b3, eq18_e259_d_b4, eq18_e259_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq25_e296: f64 = (s.v[174] * (nv7 - nv2));
        let eq25_e296_d_n2: f64 = (-s.v[174]);
        let eq25_e296_d_n7: f64 = s.v[174];
        let eq25_e297_q: f64 = eq25_e296;
        stamper.stamp_current_reactive_node2(
            Some(nodes[7]),
            Some(nodes[2]),
            nodes[2],
            multiplicity * (eq25_e296_d_n2),
            nodes[7],
            multiplicity * (eq25_e296_d_n7),
        );
        let eq26_e300: f64 = (s.v[173] * (nv1 - nv2));
        let eq26_e300_d_n1: f64 = s.v[173];
        let eq26_e300_d_n2: f64 = (-s.v[173]);
        let eq26_e301_q: f64 = eq26_e300;
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (eq26_e300_d_n1),
            nodes[2],
            multiplicity * (eq26_e300_d_n2),
        );
        let eq27_e304: f64 = (p.p108 * (nv0 - nv2));
        let eq27_e304_d_n0: f64 = p.p108;
        let eq27_e304_d_n2: f64 = (-p.p108);
        let eq27_e305_q: f64 = eq27_e304;
        stamper.stamp_current_reactive_node2(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes[0],
            multiplicity * (eq27_e304_d_n0),
            nodes[2],
            multiplicity * (eq27_e304_d_n2),
        );
        let eq33_e343: f64 = (p.p148 * s.v[196]);
        let eq33_e343_d_n0: f64 = (p.p148 * s.dn[196][0]);
        let eq33_e343_d_n1: f64 = (p.p148 * s.dn[196][1]);
        let eq33_e343_d_n2: f64 = (p.p148 * s.dn[196][2]);
        let eq33_e343_d_n3: f64 = (p.p148 * s.dn[196][3]);
        let eq33_e343_d_n4: f64 = (p.p148 * s.dn[196][4]);
        let eq33_e343_d_n5: f64 = (p.p148 * s.dn[196][5]);
        let eq33_e343_d_n6: f64 = (p.p148 * s.dn[196][6]);
        let eq33_e343_d_n7: f64 = (p.p148 * s.dn[196][7]);
        let eq33_e343_d_n8: f64 = (p.p148 * s.dn[196][8]);
        let eq33_e343_d_n9: f64 = (p.p148 * s.dn[196][9]);
        let eq33_e343_d_n10: f64 = (p.p148 * s.dn[196][10]);
        let eq33_e343_d_n11: f64 = (p.p148 * s.dn[196][11]);
        let eq33_e343_d_n12: f64 = (p.p148 * s.dn[196][12]);
        let eq33_e343_d_n13: f64 = (p.p148 * s.dn[196][13]);
        let eq33_e343_d_n14: f64 = (p.p148 * s.dn[196][14]);
        let eq33_e343_d_b0: f64 = (p.p148 * s.db[196][0]);
        let eq33_e343_d_b1: f64 = (p.p148 * s.db[196][1]);
        let eq33_e343_d_b2: f64 = (p.p148 * s.db[196][2]);
        let eq33_e343_d_b3: f64 = (p.p148 * s.db[196][3]);
        let eq33_e343_d_b4: f64 = (p.p148 * s.db[196][4]);
        let eq33_e343_d_b5: f64 = (p.p148 * s.db[196][5]);
        let eq33_e344_q: f64 = eq33_e343;
        let eq33_reactive_node_derivatives: [f64; 15] = [eq33_e343_d_n0, eq33_e343_d_n1, eq33_e343_d_n2, eq33_e343_d_n3, eq33_e343_d_n4, eq33_e343_d_n5, eq33_e343_d_n6, eq33_e343_d_n7, eq33_e343_d_n8, eq33_e343_d_n9, eq33_e343_d_n10, eq33_e343_d_n11, eq33_e343_d_n12, eq33_e343_d_n13, eq33_e343_d_n14];
        let eq33_reactive_branch_derivatives: [f64; 6] = [eq33_e343_d_b0, eq33_e343_d_b1, eq33_e343_d_b2, eq33_e343_d_b3, eq33_e343_d_b4, eq33_e343_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[5]),
            nodes,
            &eq33_reactive_node_derivatives,
            branches,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );
        let eq34_e347: f64 = (p.p148 * s.v[197]);
        let eq34_e347_d_n0: f64 = (p.p148 * s.dn[197][0]);
        let eq34_e347_d_n1: f64 = (p.p148 * s.dn[197][1]);
        let eq34_e347_d_n2: f64 = (p.p148 * s.dn[197][2]);
        let eq34_e347_d_n3: f64 = (p.p148 * s.dn[197][3]);
        let eq34_e347_d_n4: f64 = (p.p148 * s.dn[197][4]);
        let eq34_e347_d_n5: f64 = (p.p148 * s.dn[197][5]);
        let eq34_e347_d_n6: f64 = (p.p148 * s.dn[197][6]);
        let eq34_e347_d_n7: f64 = (p.p148 * s.dn[197][7]);
        let eq34_e347_d_n8: f64 = (p.p148 * s.dn[197][8]);
        let eq34_e347_d_n9: f64 = (p.p148 * s.dn[197][9]);
        let eq34_e347_d_n10: f64 = (p.p148 * s.dn[197][10]);
        let eq34_e347_d_n11: f64 = (p.p148 * s.dn[197][11]);
        let eq34_e347_d_n12: f64 = (p.p148 * s.dn[197][12]);
        let eq34_e347_d_n13: f64 = (p.p148 * s.dn[197][13]);
        let eq34_e347_d_n14: f64 = (p.p148 * s.dn[197][14]);
        let eq34_e347_d_b0: f64 = (p.p148 * s.db[197][0]);
        let eq34_e347_d_b1: f64 = (p.p148 * s.db[197][1]);
        let eq34_e347_d_b2: f64 = (p.p148 * s.db[197][2]);
        let eq34_e347_d_b3: f64 = (p.p148 * s.db[197][3]);
        let eq34_e347_d_b4: f64 = (p.p148 * s.db[197][4]);
        let eq34_e347_d_b5: f64 = (p.p148 * s.db[197][5]);
        let eq34_e348_q: f64 = eq34_e347;
        let eq34_reactive_node_derivatives: [f64; 15] = [eq34_e347_d_n0, eq34_e347_d_n1, eq34_e347_d_n2, eq34_e347_d_n3, eq34_e347_d_n4, eq34_e347_d_n5, eq34_e347_d_n6, eq34_e347_d_n7, eq34_e347_d_n8, eq34_e347_d_n9, eq34_e347_d_n10, eq34_e347_d_n11, eq34_e347_d_n12, eq34_e347_d_n13, eq34_e347_d_n14];
        let eq34_reactive_branch_derivatives: [f64; 6] = [eq34_e347_d_b0, eq34_e347_d_b1, eq34_e347_d_b2, eq34_e347_d_b3, eq34_e347_d_b4, eq34_e347_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[0]),
            nodes,
            &eq34_reactive_node_derivatives,
            branches,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq36_e363, eq36_e363_d_n3, eq36_e363_d_n9, eq36_e363_q, eq36_e363_q_d_n3, eq36_e363_q_d_n9,) = {
    if (s.b[517] && s.b[518]) {
        let eq36_e360: f64 = (p.p103 * (nv9 - nv3));
        let eq36_e360_d_n3: f64 = (-p.p103);
        let eq36_e360_d_n9: f64 = p.p103;
        let eq36_e361_q: f64 = eq36_e360;
        (eq36_e360, eq36_e360_d_n3, eq36_e360_d_n9, eq36_e361_q, eq36_e360_d_n3, eq36_e360_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2(
            Some(nodes[9]),
            Some(nodes[3]),
            nodes[3],
            multiplicity * (eq36_e363_q_d_n3),
            nodes[9],
            multiplicity * (eq36_e363_q_d_n9),
        );
        let (eq39_e385, eq39_e385_d_n4, eq39_e385_q, eq39_e385_q_d_n4,) = {
    if (s.b[519] && s.b[520]) {
        let eq39_e382: f64 = (p.p145 * (nv4 - 0.0));
        let eq39_e382_d_n4: f64 = p.p145;
        let eq39_e383_q: f64 = eq39_e382;
        (eq39_e382, eq39_e382_d_n4, eq39_e383_q, eq39_e382_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (eq39_e385_q_d_n4),
        );
        let eq42_e393_q: f64 = s.v[239];
        let eq42_reactive_node_derivatives: [f64; 15] = [s.dn[239][0], s.dn[239][1], s.dn[239][2], s.dn[239][3], s.dn[239][4], s.dn[239][5], s.dn[239][6], s.dn[239][7], s.dn[239][8], s.dn[239][9], s.dn[239][10], s.dn[239][11], s.dn[239][12], s.dn[239][13], s.dn[239][14]];
        let eq42_reactive_branch_derivatives: [f64; 6] = [s.db[239][0], s.db[239][1], s.db[239][2], s.db[239][3], s.db[239][4], s.db[239][5]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            None,
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let eq44_e396_q: f64 = s.v[240];
        let eq44_reactive_node_derivatives: [f64; 15] = [s.dn[240][0], s.dn[240][1], s.dn[240][2], s.dn[240][3], s.dn[240][4], s.dn[240][5], s.dn[240][6], s.dn[240][7], s.dn[240][8], s.dn[240][9], s.dn[240][10], s.dn[240][11], s.dn[240][12], s.dn[240][13], s.dn[240][14]];
        let eq44_reactive_branch_derivatives: [f64; 6] = [s.db[240][0], s.db[240][1], s.db[240][2], s.db[240][3], s.db[240][4], s.db[240][5]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            None,
            nodes,
            &eq44_reactive_node_derivatives,
            branches,
            &eq44_reactive_branch_derivatives,
            multiplicity,
        );
        let eq46_e399_q: f64 = s.v[236];
        let eq46_reactive_node_derivatives: [f64; 15] = [s.dn[236][0], s.dn[236][1], s.dn[236][2], s.dn[236][3], s.dn[236][4], s.dn[236][5], s.dn[236][6], s.dn[236][7], s.dn[236][8], s.dn[236][9], s.dn[236][10], s.dn[236][11], s.dn[236][12], s.dn[236][13], s.dn[236][14]];
        let eq46_reactive_branch_derivatives: [f64; 6] = [s.db[236][0], s.db[236][1], s.db[236][2], s.db[236][3], s.db[236][4], s.db[236][5]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            None,
            nodes,
            &eq46_reactive_node_derivatives,
            branches,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq65_e534, eq65_e534_d_n0, eq65_e534_d_n1, eq65_e534_d_n2, eq65_e534_d_n3, eq65_e534_d_n4, eq65_e534_d_n5, eq65_e534_d_n6, eq65_e534_d_n7, eq65_e534_d_n8, eq65_e534_d_n9, eq65_e534_d_n10, eq65_e534_d_n11, eq65_e534_d_n12, eq65_e534_d_n13, eq65_e534_d_n14, eq65_e534_d_b0, eq65_e534_d_b1, eq65_e534_d_b2, eq65_e534_d_b3, eq65_e534_d_b4, eq65_e534_d_b5, eq65_e534_q, eq65_e534_q_d_n0, eq65_e534_q_d_n1, eq65_e534_q_d_n2, eq65_e534_q_d_n3, eq65_e534_q_d_n4, eq65_e534_q_d_n5, eq65_e534_q_d_n6, eq65_e534_q_d_n7, eq65_e534_q_d_n8, eq65_e534_q_d_n9, eq65_e534_q_d_n10, eq65_e534_q_d_n11, eq65_e534_q_d_n12, eq65_e534_q_d_n13, eq65_e534_q_d_n14, eq65_e534_q_d_b0, eq65_e534_q_d_b1, eq65_e534_q_d_b2, eq65_e534_q_d_b3, eq65_e534_q_d_b4, eq65_e534_q_d_b5,) = {
    if s.b[533] {
        let eq65_e527: f64 = (s.v[537] / s.v[535]);
        let eq65_e527_d_n0: f64 = (((s.dn[537][0] * s.v[535]) - (s.v[537] * s.dn[535][0])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n1: f64 = (((s.dn[537][1] * s.v[535]) - (s.v[537] * s.dn[535][1])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n2: f64 = (((s.dn[537][2] * s.v[535]) - (s.v[537] * s.dn[535][2])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n3: f64 = (((s.dn[537][3] * s.v[535]) - (s.v[537] * s.dn[535][3])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n4: f64 = (((s.dn[537][4] * s.v[535]) - (s.v[537] * s.dn[535][4])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n5: f64 = (((s.dn[537][5] * s.v[535]) - (s.v[537] * s.dn[535][5])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n6: f64 = (((s.dn[537][6] * s.v[535]) - (s.v[537] * s.dn[535][6])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n7: f64 = (((s.dn[537][7] * s.v[535]) - (s.v[537] * s.dn[535][7])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n8: f64 = (((s.dn[537][8] * s.v[535]) - (s.v[537] * s.dn[535][8])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n9: f64 = (((s.dn[537][9] * s.v[535]) - (s.v[537] * s.dn[535][9])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n10: f64 = (((s.dn[537][10] * s.v[535]) - (s.v[537] * s.dn[535][10])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n11: f64 = (((s.dn[537][11] * s.v[535]) - (s.v[537] * s.dn[535][11])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n12: f64 = (((s.dn[537][12] * s.v[535]) - (s.v[537] * s.dn[535][12])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n13: f64 = (((s.dn[537][13] * s.v[535]) - (s.v[537] * s.dn[535][13])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n14: f64 = (((s.dn[537][14] * s.v[535]) - (s.v[537] * s.dn[535][14])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_b0: f64 = (((s.db[537][0] * s.v[535]) - (s.v[537] * s.db[535][0])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_b1: f64 = (((s.db[537][1] * s.v[535]) - (s.v[537] * s.db[535][1])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_b2: f64 = (((s.db[537][2] * s.v[535]) - (s.v[537] * s.db[535][2])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_b3: f64 = (((s.db[537][3] * s.v[535]) - (s.v[537] * s.db[535][3])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_b4: f64 = (((s.db[537][4] * s.v[535]) - (s.v[537] * s.db[535][4])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_b5: f64 = (((s.db[537][5] * s.v[535]) - (s.v[537] * s.db[535][5])) / (s.v[535] * s.v[535]));
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
        let eq66_e538_d_n0: f64 = (((s.dn[536][0] * s.v[535]) - (s.v[536] * s.dn[535][0])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n1: f64 = (((s.dn[536][1] * s.v[535]) - (s.v[536] * s.dn[535][1])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n2: f64 = (((s.dn[536][2] * s.v[535]) - (s.v[536] * s.dn[535][2])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n3: f64 = (((s.dn[536][3] * s.v[535]) - (s.v[536] * s.dn[535][3])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n4: f64 = (((s.dn[536][4] * s.v[535]) - (s.v[536] * s.dn[535][4])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n5: f64 = (((s.dn[536][5] * s.v[535]) - (s.v[536] * s.dn[535][5])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n6: f64 = (((s.dn[536][6] * s.v[535]) - (s.v[536] * s.dn[535][6])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n7: f64 = (((s.dn[536][7] * s.v[535]) - (s.v[536] * s.dn[535][7])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n8: f64 = (((s.dn[536][8] * s.v[535]) - (s.v[536] * s.dn[535][8])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n9: f64 = (((s.dn[536][9] * s.v[535]) - (s.v[536] * s.dn[535][9])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n10: f64 = (((s.dn[536][10] * s.v[535]) - (s.v[536] * s.dn[535][10])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n11: f64 = (((s.dn[536][11] * s.v[535]) - (s.v[536] * s.dn[535][11])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n12: f64 = (((s.dn[536][12] * s.v[535]) - (s.v[536] * s.dn[535][12])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n13: f64 = (((s.dn[536][13] * s.v[535]) - (s.v[536] * s.dn[535][13])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n14: f64 = (((s.dn[536][14] * s.v[535]) - (s.v[536] * s.dn[535][14])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_b0: f64 = (((s.db[536][0] * s.v[535]) - (s.v[536] * s.db[535][0])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_b1: f64 = (((s.db[536][1] * s.v[535]) - (s.v[536] * s.db[535][1])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_b2: f64 = (((s.db[536][2] * s.v[535]) - (s.v[536] * s.db[535][2])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_b3: f64 = (((s.db[536][3] * s.v[535]) - (s.v[536] * s.db[535][3])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_b4: f64 = (((s.db[536][4] * s.v[535]) - (s.v[536] * s.db[535][4])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_b5: f64 = (((s.db[536][5] * s.v[535]) - (s.v[536] * s.db[535][5])) / (s.v[535] * s.v[535]));
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
