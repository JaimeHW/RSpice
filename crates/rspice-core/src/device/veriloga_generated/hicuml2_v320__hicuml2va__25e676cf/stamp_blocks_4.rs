#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

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
        var_iavl: f64,
        var_iavl_db0: f64,
        var_iavl_db1: f64,
        var_iavl_db2: f64,
        var_iavl_db3: f64,
        var_iavl_db4: f64,
        var_iavl_db5: f64,
        var_iavl_dn0: f64,
        var_iavl_dn1: f64,
        var_iavl_dn10: f64,
        var_iavl_dn11: f64,
        var_iavl_dn12: f64,
        var_iavl_dn13: f64,
        var_iavl_dn14: f64,
        var_iavl_dn2: f64,
        var_iavl_dn3: f64,
        var_iavl_dn4: f64,
        var_iavl_dn5: f64,
        var_iavl_dn6: f64,
        var_iavl_dn7: f64,
        var_iavl_dn8: f64,
        var_iavl_dn9: f64,
        var_ibci: f64,
        var_ibci_db0: f64,
        var_ibci_db1: f64,
        var_ibci_db2: f64,
        var_ibci_db3: f64,
        var_ibci_db4: f64,
        var_ibci_db5: f64,
        var_ibci_dn0: f64,
        var_ibci_dn1: f64,
        var_ibci_dn10: f64,
        var_ibci_dn11: f64,
        var_ibci_dn12: f64,
        var_ibci_dn13: f64,
        var_ibci_dn14: f64,
        var_ibci_dn2: f64,
        var_ibci_dn3: f64,
        var_ibci_dn4: f64,
        var_ibci_dn5: f64,
        var_ibci_dn6: f64,
        var_ibci_dn7: f64,
        var_ibci_dn8: f64,
        var_ibci_dn9: f64,
        var_ibebtb: f64,
        var_ibebtb_db0: f64,
        var_ibebtb_db1: f64,
        var_ibebtb_db2: f64,
        var_ibebtb_db3: f64,
        var_ibebtb_db4: f64,
        var_ibebtb_db5: f64,
        var_ibebtb_dn0: f64,
        var_ibebtb_dn1: f64,
        var_ibebtb_dn10: f64,
        var_ibebtb_dn11: f64,
        var_ibebtb_dn12: f64,
        var_ibebtb_dn13: f64,
        var_ibebtb_dn14: f64,
        var_ibebtb_dn2: f64,
        var_ibebtb_dn3: f64,
        var_ibebtb_dn4: f64,
        var_ibebtb_dn5: f64,
        var_ibebtb_dn6: f64,
        var_ibebtb_dn7: f64,
        var_ibebtb_dn8: f64,
        var_ibebtb_dn9: f64,
        var_ibei: f64,
        var_ibei_db0: f64,
        var_ibei_db1: f64,
        var_ibei_db2: f64,
        var_ibei_db3: f64,
        var_ibei_db4: f64,
        var_ibei_db5: f64,
        var_ibei_dn0: f64,
        var_ibei_dn1: f64,
        var_ibei_dn10: f64,
        var_ibei_dn11: f64,
        var_ibei_dn12: f64,
        var_ibei_dn13: f64,
        var_ibei_dn14: f64,
        var_ibei_dn2: f64,
        var_ibei_dn3: f64,
        var_ibei_dn4: f64,
        var_ibei_dn5: f64,
        var_ibei_dn6: f64,
        var_ibei_dn7: f64,
        var_ibei_dn8: f64,
        var_ibei_dn9: f64,
        var_ibetat: f64,
        var_ibetat_db0: f64,
        var_ibetat_db1: f64,
        var_ibetat_db2: f64,
        var_ibetat_db3: f64,
        var_ibetat_db4: f64,
        var_ibetat_db5: f64,
        var_ibetat_dn0: f64,
        var_ibetat_dn1: f64,
        var_ibetat_dn10: f64,
        var_ibetat_dn11: f64,
        var_ibetat_dn12: f64,
        var_ibetat_dn13: f64,
        var_ibetat_dn14: f64,
        var_ibetat_dn2: f64,
        var_ibetat_dn3: f64,
        var_ibetat_dn4: f64,
        var_ibetat_dn5: f64,
        var_ibetat_dn6: f64,
        var_ibetat_dn7: f64,
        var_ibetat_dn8: f64,
        var_ibetat_dn9: f64,
        var_ibh_rec: f64,
        var_ibh_rec_db0: f64,
        var_ibh_rec_db1: f64,
        var_ibh_rec_db2: f64,
        var_ibh_rec_db3: f64,
        var_ibh_rec_db4: f64,
        var_ibh_rec_db5: f64,
        var_ibh_rec_dn0: f64,
        var_ibh_rec_dn1: f64,
        var_ibh_rec_dn10: f64,
        var_ibh_rec_dn11: f64,
        var_ibh_rec_dn12: f64,
        var_ibh_rec_dn13: f64,
        var_ibh_rec_dn14: f64,
        var_ibh_rec_dn2: f64,
        var_ibh_rec_dn3: f64,
        var_ibh_rec_dn4: f64,
        var_ibh_rec_dn5: f64,
        var_ibh_rec_dn6: f64,
        var_ibh_rec_dn7: f64,
        var_ibh_rec_dn8: f64,
        var_ibh_rec_dn9: f64,
        var_irei: f64,
        var_irei_db0: f64,
        var_irei_db1: f64,
        var_irei_db2: f64,
        var_irei_db3: f64,
        var_irei_db4: f64,
        var_irei_db5: f64,
        var_irei_dn0: f64,
        var_irei_dn1: f64,
        var_irei_dn10: f64,
        var_irei_dn11: f64,
        var_irei_dn12: f64,
        var_irei_dn13: f64,
        var_irei_dn14: f64,
        var_irei_dn2: f64,
        var_irei_dn3: f64,
        var_irei_dn4: f64,
        var_irei_dn5: f64,
        var_irei_dn6: f64,
        var_irei_dn7: f64,
        var_irei_dn8: f64,
        var_irei_dn9: f64,
        var_itr: f64,
        var_itr_db0: f64,
        var_itr_db1: f64,
        var_itr_db2: f64,
        var_itr_db3: f64,
        var_itr_db4: f64,
        var_itr_db5: f64,
        var_itr_dn0: f64,
        var_itr_dn1: f64,
        var_itr_dn10: f64,
        var_itr_dn11: f64,
        var_itr_dn12: f64,
        var_itr_dn13: f64,
        var_itr_dn14: f64,
        var_itr_dn2: f64,
        var_itr_dn3: f64,
        var_itr_dn4: f64,
        var_itr_dn5: f64,
        var_itr_dn6: f64,
        var_itr_dn7: f64,
        var_itr_dn8: f64,
        var_itr_dn9: f64,
        var_itxf: f64,
        var_itxf_db0: f64,
        var_itxf_db1: f64,
        var_itxf_db2: f64,
        var_itxf_db3: f64,
        var_itxf_db4: f64,
        var_itxf_db5: f64,
        var_itxf_dn0: f64,
        var_itxf_dn1: f64,
        var_itxf_dn10: f64,
        var_itxf_dn11: f64,
        var_itxf_dn12: f64,
        var_itxf_dn13: f64,
        var_itxf_dn14: f64,
        var_itxf_dn2: f64,
        var_itxf_dn3: f64,
        var_itxf_dn4: f64,
        var_itxf_dn5: f64,
        var_itxf_dn6: f64,
        var_itxf_dn7: f64,
        var_itxf_dn8: f64,
        var_itxf_dn9: f64,
        var_qdci: f64,
        var_qdci_db0: f64,
        var_qdci_db1: f64,
        var_qdci_db2: f64,
        var_qdci_db3: f64,
        var_qdci_db4: f64,
        var_qdci_db5: f64,
        var_qdci_dn0: f64,
        var_qdci_dn1: f64,
        var_qdci_dn10: f64,
        var_qdci_dn11: f64,
        var_qdci_dn12: f64,
        var_qdci_dn13: f64,
        var_qdci_dn14: f64,
        var_qdci_dn2: f64,
        var_qdci_dn3: f64,
        var_qdci_dn4: f64,
        var_qdci_dn5: f64,
        var_qdci_dn6: f64,
        var_qdci_dn7: f64,
        var_qdci_dn8: f64,
        var_qdci_dn9: f64,
        var_qdeix: f64,
        var_qdeix_db0: f64,
        var_qdeix_db1: f64,
        var_qdeix_db2: f64,
        var_qdeix_db3: f64,
        var_qdeix_db4: f64,
        var_qdeix_db5: f64,
        var_qdeix_dn0: f64,
        var_qdeix_dn1: f64,
        var_qdeix_dn10: f64,
        var_qdeix_dn11: f64,
        var_qdeix_dn12: f64,
        var_qdeix_dn13: f64,
        var_qdeix_dn14: f64,
        var_qdeix_dn2: f64,
        var_qdeix_dn3: f64,
        var_qdeix_dn4: f64,
        var_qdeix_dn5: f64,
        var_qdeix_dn6: f64,
        var_qdeix_dn7: f64,
        var_qdeix_dn8: f64,
        var_qdeix_dn9: f64,
        var_qjci: f64,
        var_qjci_db0: f64,
        var_qjci_db1: f64,
        var_qjci_db2: f64,
        var_qjci_db3: f64,
        var_qjci_db4: f64,
        var_qjci_db5: f64,
        var_qjci_dn0: f64,
        var_qjci_dn1: f64,
        var_qjci_dn10: f64,
        var_qjci_dn11: f64,
        var_qjci_dn12: f64,
        var_qjci_dn13: f64,
        var_qjci_dn14: f64,
        var_qjci_dn2: f64,
        var_qjci_dn3: f64,
        var_qjci_dn4: f64,
        var_qjci_dn5: f64,
        var_qjci_dn6: f64,
        var_qjci_dn7: f64,
        var_qjci_dn8: f64,
        var_qjci_dn9: f64,
        var_qjei: f64,
        var_qjei_db0: f64,
        var_qjei_db1: f64,
        var_qjei_db2: f64,
        var_qjei_db3: f64,
        var_qjei_db4: f64,
        var_qjei_db5: f64,
        var_qjei_dn0: f64,
        var_qjei_dn1: f64,
        var_qjei_dn10: f64,
        var_qjei_dn11: f64,
        var_qjei_dn12: f64,
        var_qjei_dn13: f64,
        var_qjei_dn14: f64,
        var_qjei_dn2: f64,
        var_qjei_dn3: f64,
        var_qjei_dn4: f64,
        var_qjei_dn5: f64,
        var_qjei_dn6: f64,
        var_qjei_dn7: f64,
        var_qjei_dn8: f64,
        var_qjei_dn9: f64,
        var_qrbi: f64,
        var_qrbi_db0: f64,
        var_qrbi_db1: f64,
        var_qrbi_db2: f64,
        var_qrbi_db3: f64,
        var_qrbi_db4: f64,
        var_qrbi_db5: f64,
        var_qrbi_dn0: f64,
        var_qrbi_dn1: f64,
        var_qrbi_dn10: f64,
        var_qrbi_dn11: f64,
        var_qrbi_dn12: f64,
        var_qrbi_dn13: f64,
        var_qrbi_dn14: f64,
        var_qrbi_dn2: f64,
        var_qrbi_dn3: f64,
        var_qrbi_dn4: f64,
        var_qrbi_dn5: f64,
        var_qrbi_dn6: f64,
        var_qrbi_dn7: f64,
        var_qrbi_dn8: f64,
        var_qrbi_dn9: f64,
        var_rbi: f64,
        var_rbi_db0: f64,
        var_rbi_db1: f64,
        var_rbi_db2: f64,
        var_rbi_db3: f64,
        var_rbi_db4: f64,
        var_rbi_db5: f64,
        var_rbi_dn0: f64,
        var_rbi_dn1: f64,
        var_rbi_dn10: f64,
        var_rbi_dn11: f64,
        var_rbi_dn12: f64,
        var_rbi_dn13: f64,
        var_rbi_dn14: f64,
        var_rbi_dn2: f64,
        var_rbi_dn3: f64,
        var_rbi_dn4: f64,
        var_rbi_dn5: f64,
        var_rbi_dn6: f64,
        var_rbi_dn7: f64,
        var_rbi_dn8: f64,
        var_rbi_dn9: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let eq0_e157: f64 = (var_ibei + var_irei);
        let eq0_e157_d_n0: f64 = (var_ibei_dn0 + var_irei_dn0);
        let eq0_e157_d_n1: f64 = (var_ibei_dn1 + var_irei_dn1);
        let eq0_e157_d_n2: f64 = (var_ibei_dn2 + var_irei_dn2);
        let eq0_e157_d_n3: f64 = (var_ibei_dn3 + var_irei_dn3);
        let eq0_e157_d_n4: f64 = (var_ibei_dn4 + var_irei_dn4);
        let eq0_e157_d_n5: f64 = (var_ibei_dn5 + var_irei_dn5);
        let eq0_e157_d_n6: f64 = (var_ibei_dn6 + var_irei_dn6);
        let eq0_e157_d_n7: f64 = (var_ibei_dn7 + var_irei_dn7);
        let eq0_e157_d_n8: f64 = (var_ibei_dn8 + var_irei_dn8);
        let eq0_e157_d_n9: f64 = (var_ibei_dn9 + var_irei_dn9);
        let eq0_e157_d_n10: f64 = (var_ibei_dn10 + var_irei_dn10);
        let eq0_e157_d_n11: f64 = (var_ibei_dn11 + var_irei_dn11);
        let eq0_e157_d_n12: f64 = (var_ibei_dn12 + var_irei_dn12);
        let eq0_e157_d_n13: f64 = (var_ibei_dn13 + var_irei_dn13);
        let eq0_e157_d_n14: f64 = (var_ibei_dn14 + var_irei_dn14);
        let eq0_e157_d_b0: f64 = (var_ibei_db0 + var_irei_db0);
        let eq0_e157_d_b1: f64 = (var_ibei_db1 + var_irei_db1);
        let eq0_e157_d_b2: f64 = (var_ibei_db2 + var_irei_db2);
        let eq0_e157_d_b3: f64 = (var_ibei_db3 + var_irei_db3);
        let eq0_e157_d_b4: f64 = (var_ibei_db4 + var_irei_db4);
        let eq0_e157_d_b5: f64 = (var_ibei_db5 + var_irei_db5);
        let eq0_e159: f64 = (eq0_e157 + var_ibetat);
        let eq0_e159_d_n0: f64 = (eq0_e157_d_n0 + var_ibetat_dn0);
        let eq0_e159_d_n1: f64 = (eq0_e157_d_n1 + var_ibetat_dn1);
        let eq0_e159_d_n2: f64 = (eq0_e157_d_n2 + var_ibetat_dn2);
        let eq0_e159_d_n3: f64 = (eq0_e157_d_n3 + var_ibetat_dn3);
        let eq0_e159_d_n4: f64 = (eq0_e157_d_n4 + var_ibetat_dn4);
        let eq0_e159_d_n5: f64 = (eq0_e157_d_n5 + var_ibetat_dn5);
        let eq0_e159_d_n6: f64 = (eq0_e157_d_n6 + var_ibetat_dn6);
        let eq0_e159_d_n7: f64 = (eq0_e157_d_n7 + var_ibetat_dn7);
        let eq0_e159_d_n8: f64 = (eq0_e157_d_n8 + var_ibetat_dn8);
        let eq0_e159_d_n9: f64 = (eq0_e157_d_n9 + var_ibetat_dn9);
        let eq0_e159_d_n10: f64 = (eq0_e157_d_n10 + var_ibetat_dn10);
        let eq0_e159_d_n11: f64 = (eq0_e157_d_n11 + var_ibetat_dn11);
        let eq0_e159_d_n12: f64 = (eq0_e157_d_n12 + var_ibetat_dn12);
        let eq0_e159_d_n13: f64 = (eq0_e157_d_n13 + var_ibetat_dn13);
        let eq0_e159_d_n14: f64 = (eq0_e157_d_n14 + var_ibetat_dn14);
        let eq0_e159_d_b0: f64 = (eq0_e157_d_b0 + var_ibetat_db0);
        let eq0_e159_d_b1: f64 = (eq0_e157_d_b1 + var_ibetat_db1);
        let eq0_e159_d_b2: f64 = (eq0_e157_d_b2 + var_ibetat_db2);
        let eq0_e159_d_b3: f64 = (eq0_e157_d_b3 + var_ibetat_db3);
        let eq0_e159_d_b4: f64 = (eq0_e157_d_b4 + var_ibetat_db4);
        let eq0_e159_d_b5: f64 = (eq0_e157_d_b5 + var_ibetat_db5);
        let eq0_e161: f64 = (eq0_e159 + var_ibh_rec);
        let eq0_e161_d_n0: f64 = (eq0_e159_d_n0 + var_ibh_rec_dn0);
        let eq0_e161_d_n1: f64 = (eq0_e159_d_n1 + var_ibh_rec_dn1);
        let eq0_e161_d_n2: f64 = (eq0_e159_d_n2 + var_ibh_rec_dn2);
        let eq0_e161_d_n3: f64 = (eq0_e159_d_n3 + var_ibh_rec_dn3);
        let eq0_e161_d_n4: f64 = (eq0_e159_d_n4 + var_ibh_rec_dn4);
        let eq0_e161_d_n5: f64 = (eq0_e159_d_n5 + var_ibh_rec_dn5);
        let eq0_e161_d_n6: f64 = (eq0_e159_d_n6 + var_ibh_rec_dn6);
        let eq0_e161_d_n7: f64 = (eq0_e159_d_n7 + var_ibh_rec_dn7);
        let eq0_e161_d_n8: f64 = (eq0_e159_d_n8 + var_ibh_rec_dn8);
        let eq0_e161_d_n9: f64 = (eq0_e159_d_n9 + var_ibh_rec_dn9);
        let eq0_e161_d_n10: f64 = (eq0_e159_d_n10 + var_ibh_rec_dn10);
        let eq0_e161_d_n11: f64 = (eq0_e159_d_n11 + var_ibh_rec_dn11);
        let eq0_e161_d_n12: f64 = (eq0_e159_d_n12 + var_ibh_rec_dn12);
        let eq0_e161_d_n13: f64 = (eq0_e159_d_n13 + var_ibh_rec_dn13);
        let eq0_e161_d_n14: f64 = (eq0_e159_d_n14 + var_ibh_rec_dn14);
        let eq0_e161_d_b0: f64 = (eq0_e159_d_b0 + var_ibh_rec_db0);
        let eq0_e161_d_b1: f64 = (eq0_e159_d_b1 + var_ibh_rec_db1);
        let eq0_e161_d_b2: f64 = (eq0_e159_d_b2 + var_ibh_rec_db2);
        let eq0_e161_d_b3: f64 = (eq0_e159_d_b3 + var_ibh_rec_db3);
        let eq0_e161_d_b4: f64 = (eq0_e159_d_b4 + var_ibh_rec_db4);
        let eq0_e161_d_b5: f64 = (eq0_e159_d_b5 + var_ibh_rec_db5);
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
        let eq0_e165: f64 = (var_gmin * (nv8 - nv6));
        let eq0_e166: f64 = (eq0_e162 + eq0_e165);
        let eq0_e166_d_n6: f64 = (eq0_e162_d_n6 + (-var_gmin));
        let eq0_e166_d_n8: f64 = (eq0_e162_d_n8 + var_gmin);
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
        let eq1_e170: f64 = (var_qdeix + var_qjei);
        let eq1_e170_d_n0: f64 = (var_qdeix_dn0 + var_qjei_dn0);
        let eq1_e170_d_n1: f64 = (var_qdeix_dn1 + var_qjei_dn1);
        let eq1_e170_d_n2: f64 = (var_qdeix_dn2 + var_qjei_dn2);
        let eq1_e170_d_n3: f64 = (var_qdeix_dn3 + var_qjei_dn3);
        let eq1_e170_d_n4: f64 = (var_qdeix_dn4 + var_qjei_dn4);
        let eq1_e170_d_n5: f64 = (var_qdeix_dn5 + var_qjei_dn5);
        let eq1_e170_d_n6: f64 = (var_qdeix_dn6 + var_qjei_dn6);
        let eq1_e170_d_n7: f64 = (var_qdeix_dn7 + var_qjei_dn7);
        let eq1_e170_d_n8: f64 = (var_qdeix_dn8 + var_qjei_dn8);
        let eq1_e170_d_n9: f64 = (var_qdeix_dn9 + var_qjei_dn9);
        let eq1_e170_d_n10: f64 = (var_qdeix_dn10 + var_qjei_dn10);
        let eq1_e170_d_n11: f64 = (var_qdeix_dn11 + var_qjei_dn11);
        let eq1_e170_d_n12: f64 = (var_qdeix_dn12 + var_qjei_dn12);
        let eq1_e170_d_n13: f64 = (var_qdeix_dn13 + var_qjei_dn13);
        let eq1_e170_d_n14: f64 = (var_qdeix_dn14 + var_qjei_dn14);
        let eq1_e170_d_b0: f64 = (var_qdeix_db0 + var_qjei_db0);
        let eq1_e170_d_b1: f64 = (var_qdeix_db1 + var_qjei_db1);
        let eq1_e170_d_b2: f64 = (var_qdeix_db2 + var_qjei_db2);
        let eq1_e170_d_b3: f64 = (var_qdeix_db3 + var_qjei_db3);
        let eq1_e170_d_b4: f64 = (var_qdeix_db4 + var_qjei_db4);
        let eq1_e170_d_b5: f64 = (var_qdeix_db5 + var_qjei_db5);
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
        let eq1_e172: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq1_e171);
        let eq1_value: f64 = eq1_e172;
        let eq1_node_derivatives: [f64; 15] = [(eq1_e171_d_n0 * ddt_scale), (eq1_e171_d_n1 * ddt_scale), (eq1_e171_d_n2 * ddt_scale), (eq1_e171_d_n3 * ddt_scale), (eq1_e171_d_n4 * ddt_scale), (eq1_e171_d_n5 * ddt_scale), (eq1_e171_d_n6 * ddt_scale), (eq1_e171_d_n7 * ddt_scale), (eq1_e171_d_n8 * ddt_scale), (eq1_e171_d_n9 * ddt_scale), (eq1_e171_d_n10 * ddt_scale), (eq1_e171_d_n11 * ddt_scale), (eq1_e171_d_n12 * ddt_scale), (eq1_e171_d_n13 * ddt_scale), (eq1_e171_d_n14 * ddt_scale)];
        let eq1_branch_derivatives: [f64; 6] = [(eq1_e171_d_b0 * ddt_scale), (eq1_e171_d_b1 * ddt_scale), (eq1_e171_d_b2 * ddt_scale), (eq1_e171_d_b3 * ddt_scale), (eq1_e171_d_b4 * ddt_scale), (eq1_e171_d_b5 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let eq2_e176: f64 = (var_ibci - var_iavl);
        let eq2_e176_d_n0: f64 = (var_ibci_dn0 - var_iavl_dn0);
        let eq2_e176_d_n1: f64 = (var_ibci_dn1 - var_iavl_dn1);
        let eq2_e176_d_n2: f64 = (var_ibci_dn2 - var_iavl_dn2);
        let eq2_e176_d_n3: f64 = (var_ibci_dn3 - var_iavl_dn3);
        let eq2_e176_d_n4: f64 = (var_ibci_dn4 - var_iavl_dn4);
        let eq2_e176_d_n5: f64 = (var_ibci_dn5 - var_iavl_dn5);
        let eq2_e176_d_n6: f64 = (var_ibci_dn6 - var_iavl_dn6);
        let eq2_e176_d_n7: f64 = (var_ibci_dn7 - var_iavl_dn7);
        let eq2_e176_d_n8: f64 = (var_ibci_dn8 - var_iavl_dn8);
        let eq2_e176_d_n9: f64 = (var_ibci_dn9 - var_iavl_dn9);
        let eq2_e176_d_n10: f64 = (var_ibci_dn10 - var_iavl_dn10);
        let eq2_e176_d_n11: f64 = (var_ibci_dn11 - var_iavl_dn11);
        let eq2_e176_d_n12: f64 = (var_ibci_dn12 - var_iavl_dn12);
        let eq2_e176_d_n13: f64 = (var_ibci_dn13 - var_iavl_dn13);
        let eq2_e176_d_n14: f64 = (var_ibci_dn14 - var_iavl_dn14);
        let eq2_e176_d_b0: f64 = (var_ibci_db0 - var_iavl_db0);
        let eq2_e176_d_b1: f64 = (var_ibci_db1 - var_iavl_db1);
        let eq2_e176_d_b2: f64 = (var_ibci_db2 - var_iavl_db2);
        let eq2_e176_d_b3: f64 = (var_ibci_db3 - var_iavl_db3);
        let eq2_e176_d_b4: f64 = (var_ibci_db4 - var_iavl_db4);
        let eq2_e176_d_b5: f64 = (var_ibci_db5 - var_iavl_db5);
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
        let eq2_e180: f64 = (var_gmin * (nv8 - nv5));
        let eq2_e181: f64 = (eq2_e177 + eq2_e180);
        let eq2_e181_d_n5: f64 = (eq2_e177_d_n5 + (-var_gmin));
        let eq2_e181_d_n8: f64 = (eq2_e177_d_n8 + var_gmin);
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
        let eq3_e185: f64 = (var_qdci + var_qjci);
        let eq3_e185_d_n0: f64 = (var_qdci_dn0 + var_qjci_dn0);
        let eq3_e185_d_n1: f64 = (var_qdci_dn1 + var_qjci_dn1);
        let eq3_e185_d_n2: f64 = (var_qdci_dn2 + var_qjci_dn2);
        let eq3_e185_d_n3: f64 = (var_qdci_dn3 + var_qjci_dn3);
        let eq3_e185_d_n4: f64 = (var_qdci_dn4 + var_qjci_dn4);
        let eq3_e185_d_n5: f64 = (var_qdci_dn5 + var_qjci_dn5);
        let eq3_e185_d_n6: f64 = (var_qdci_dn6 + var_qjci_dn6);
        let eq3_e185_d_n7: f64 = (var_qdci_dn7 + var_qjci_dn7);
        let eq3_e185_d_n8: f64 = (var_qdci_dn8 + var_qjci_dn8);
        let eq3_e185_d_n9: f64 = (var_qdci_dn9 + var_qjci_dn9);
        let eq3_e185_d_n10: f64 = (var_qdci_dn10 + var_qjci_dn10);
        let eq3_e185_d_n11: f64 = (var_qdci_dn11 + var_qjci_dn11);
        let eq3_e185_d_n12: f64 = (var_qdci_dn12 + var_qjci_dn12);
        let eq3_e185_d_n13: f64 = (var_qdci_dn13 + var_qjci_dn13);
        let eq3_e185_d_n14: f64 = (var_qdci_dn14 + var_qjci_dn14);
        let eq3_e185_d_b0: f64 = (var_qdci_db0 + var_qjci_db0);
        let eq3_e185_d_b1: f64 = (var_qdci_db1 + var_qjci_db1);
        let eq3_e185_d_b2: f64 = (var_qdci_db2 + var_qjci_db2);
        let eq3_e185_d_b3: f64 = (var_qdci_db3 + var_qjci_db3);
        let eq3_e185_d_b4: f64 = (var_qdci_db4 + var_qjci_db4);
        let eq3_e185_d_b5: f64 = (var_qdci_db5 + var_qjci_db5);
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
        let eq3_e187: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq3_e186);
        let eq3_value: f64 = eq3_e187;
        let eq3_node_derivatives: [f64; 15] = [(eq3_e186_d_n0 * ddt_scale), (eq3_e186_d_n1 * ddt_scale), (eq3_e186_d_n2 * ddt_scale), (eq3_e186_d_n3 * ddt_scale), (eq3_e186_d_n4 * ddt_scale), (eq3_e186_d_n5 * ddt_scale), (eq3_e186_d_n6 * ddt_scale), (eq3_e186_d_n7 * ddt_scale), (eq3_e186_d_n8 * ddt_scale), (eq3_e186_d_n9 * ddt_scale), (eq3_e186_d_n10 * ddt_scale), (eq3_e186_d_n11 * ddt_scale), (eq3_e186_d_n12 * ddt_scale), (eq3_e186_d_n13 * ddt_scale), (eq3_e186_d_n14 * ddt_scale)];
        let eq3_branch_derivatives: [f64; 6] = [(eq3_e186_d_b0 * ddt_scale), (eq3_e186_d_b1 * ddt_scale), (eq3_e186_d_b2 * ddt_scale), (eq3_e186_d_b3 * ddt_scale), (eq3_e186_d_b4 * ddt_scale), (eq3_e186_d_b5 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let eq4_e190: f64 = (p.p148 * var_itxf);
        let eq4_e190_d_n0: f64 = (p.p148 * var_itxf_dn0);
        let eq4_e190_d_n1: f64 = (p.p148 * var_itxf_dn1);
        let eq4_e190_d_n2: f64 = (p.p148 * var_itxf_dn2);
        let eq4_e190_d_n3: f64 = (p.p148 * var_itxf_dn3);
        let eq4_e190_d_n4: f64 = (p.p148 * var_itxf_dn4);
        let eq4_e190_d_n5: f64 = (p.p148 * var_itxf_dn5);
        let eq4_e190_d_n6: f64 = (p.p148 * var_itxf_dn6);
        let eq4_e190_d_n7: f64 = (p.p148 * var_itxf_dn7);
        let eq4_e190_d_n8: f64 = (p.p148 * var_itxf_dn8);
        let eq4_e190_d_n9: f64 = (p.p148 * var_itxf_dn9);
        let eq4_e190_d_n10: f64 = (p.p148 * var_itxf_dn10);
        let eq4_e190_d_n11: f64 = (p.p148 * var_itxf_dn11);
        let eq4_e190_d_n12: f64 = (p.p148 * var_itxf_dn12);
        let eq4_e190_d_n13: f64 = (p.p148 * var_itxf_dn13);
        let eq4_e190_d_n14: f64 = (p.p148 * var_itxf_dn14);
        let eq4_e190_d_b0: f64 = (p.p148 * var_itxf_db0);
        let eq4_e190_d_b1: f64 = (p.p148 * var_itxf_db1);
        let eq4_e190_d_b2: f64 = (p.p148 * var_itxf_db2);
        let eq4_e190_d_b3: f64 = (p.p148 * var_itxf_db3);
        let eq4_e190_d_b4: f64 = (p.p148 * var_itxf_db4);
        let eq4_e190_d_b5: f64 = (p.p148 * var_itxf_db5);
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
        let eq5_e193: f64 = (p.p148 * var_itr);
        let eq5_e193_d_n0: f64 = (p.p148 * var_itr_dn0);
        let eq5_e193_d_n1: f64 = (p.p148 * var_itr_dn1);
        let eq5_e193_d_n2: f64 = (p.p148 * var_itr_dn2);
        let eq5_e193_d_n3: f64 = (p.p148 * var_itr_dn3);
        let eq5_e193_d_n4: f64 = (p.p148 * var_itr_dn4);
        let eq5_e193_d_n5: f64 = (p.p148 * var_itr_dn5);
        let eq5_e193_d_n6: f64 = (p.p148 * var_itr_dn6);
        let eq5_e193_d_n7: f64 = (p.p148 * var_itr_dn7);
        let eq5_e193_d_n8: f64 = (p.p148 * var_itr_dn8);
        let eq5_e193_d_n9: f64 = (p.p148 * var_itr_dn9);
        let eq5_e193_d_n10: f64 = (p.p148 * var_itr_dn10);
        let eq5_e193_d_n11: f64 = (p.p148 * var_itr_dn11);
        let eq5_e193_d_n12: f64 = (p.p148 * var_itr_dn12);
        let eq5_e193_d_n13: f64 = (p.p148 * var_itr_dn13);
        let eq5_e193_d_n14: f64 = (p.p148 * var_itr_dn14);
        let eq5_e193_d_b0: f64 = (p.p148 * var_itr_db0);
        let eq5_e193_d_b1: f64 = (p.p148 * var_itr_db1);
        let eq5_e193_d_b2: f64 = (p.p148 * var_itr_db2);
        let eq5_e193_d_b3: f64 = (p.p148 * var_itr_db3);
        let eq5_e193_d_b4: f64 = (p.p148 * var_itr_db4);
        let eq5_e193_d_b5: f64 = (p.p148 * var_itr_db5);
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
    if (var_guard233 != 0.0) {
        let eq6_e197: f64 = ((nv7 - nv8) / var_rbi);
        let eq6_e197_d_n0: f64 = (-(((nv7 - nv8) * var_rbi_dn0) / (var_rbi * var_rbi)));
        let eq6_e197_d_n1: f64 = (-(((nv7 - nv8) * var_rbi_dn1) / (var_rbi * var_rbi)));
        let eq6_e197_d_n2: f64 = (-(((nv7 - nv8) * var_rbi_dn2) / (var_rbi * var_rbi)));
        let eq6_e197_d_n3: f64 = (-(((nv7 - nv8) * var_rbi_dn3) / (var_rbi * var_rbi)));
        let eq6_e197_d_n4: f64 = (-(((nv7 - nv8) * var_rbi_dn4) / (var_rbi * var_rbi)));
        let eq6_e197_d_n5: f64 = (-(((nv7 - nv8) * var_rbi_dn5) / (var_rbi * var_rbi)));
        let eq6_e197_d_n6: f64 = (-(((nv7 - nv8) * var_rbi_dn6) / (var_rbi * var_rbi)));
        let __rspice_inv_cse_0: f64 = 1.0 / (var_rbi * var_rbi);
        let eq6_e197_d_n7: f64 = ((var_rbi - ((nv7 - nv8) * var_rbi_dn7)) * __rspice_inv_cse_0);
        let eq6_e197_d_n8: f64 = (((-var_rbi) - ((nv7 - nv8) * var_rbi_dn8)) * __rspice_inv_cse_0);
        let eq6_e197_d_n9: f64 = (-(((nv7 - nv8) * var_rbi_dn9) / (var_rbi * var_rbi)));
        let eq6_e197_d_n10: f64 = (-(((nv7 - nv8) * var_rbi_dn10) / (var_rbi * var_rbi)));
        let eq6_e197_d_n11: f64 = (-(((nv7 - nv8) * var_rbi_dn11) / (var_rbi * var_rbi)));
        let eq6_e197_d_n12: f64 = (-(((nv7 - nv8) * var_rbi_dn12) / (var_rbi * var_rbi)));
        let eq6_e197_d_n13: f64 = (-(((nv7 - nv8) * var_rbi_dn13) / (var_rbi * var_rbi)));
        let eq6_e197_d_n14: f64 = (-(((nv7 - nv8) * var_rbi_dn14) / (var_rbi * var_rbi)));
        let eq6_e197_d_b0: f64 = (-(((nv7 - nv8) * var_rbi_db0) / (var_rbi * var_rbi)));
        let eq6_e197_d_b1: f64 = (-(((nv7 - nv8) * var_rbi_db1) / (var_rbi * var_rbi)));
        let eq6_e197_d_b2: f64 = (-(((nv7 - nv8) * var_rbi_db2) / (var_rbi * var_rbi)));
        let eq6_e197_d_b3: f64 = (-(((nv7 - nv8) * var_rbi_db3) / (var_rbi * var_rbi)));
        let eq6_e197_d_b4: f64 = (-(((nv7 - nv8) * var_rbi_db4) / (var_rbi * var_rbi)));
        let eq6_e197_d_b5: f64 = (-(((nv7 - nv8) * var_rbi_db5) / (var_rbi * var_rbi)));
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
    if ((var_guard233 != 0.0) && (var_guard234 != 0.0)) {
        let eq7_e204: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, var_qrbi);
        (eq7_e204, (var_qrbi_dn0 * ddt_scale), (var_qrbi_dn1 * ddt_scale), (var_qrbi_dn2 * ddt_scale), (var_qrbi_dn3 * ddt_scale), (var_qrbi_dn4 * ddt_scale), (var_qrbi_dn5 * ddt_scale), (var_qrbi_dn6 * ddt_scale), (var_qrbi_dn7 * ddt_scale), (var_qrbi_dn8 * ddt_scale), (var_qrbi_dn9 * ddt_scale), (var_qrbi_dn10 * ddt_scale), (var_qrbi_dn11 * ddt_scale), (var_qrbi_dn12 * ddt_scale), (var_qrbi_dn13 * ddt_scale), (var_qrbi_dn14 * ddt_scale), (var_qrbi_db0 * ddt_scale), (var_qrbi_db1 * ddt_scale), (var_qrbi_db2 * ddt_scale), (var_qrbi_db3 * ddt_scale), (var_qrbi_db4 * ddt_scale), (var_qrbi_db5 * ddt_scale),)
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
        let (eq9_e218, eq9_e218_d_n0, eq9_e218_d_n1, eq9_e218_d_n2, eq9_e218_d_n3, eq9_e218_d_n4, eq9_e218_d_n5, eq9_e218_d_n6, eq9_e218_d_n7, eq9_e218_d_n8, eq9_e218_d_n9, eq9_e218_d_n10, eq9_e218_d_n11, eq9_e218_d_n12, eq9_e218_d_n13, eq9_e218_d_n14, eq9_e218_d_b0, eq9_e218_d_b1, eq9_e218_d_b2, eq9_e218_d_b3, eq9_e218_d_b4, eq9_e218_d_b5,) = {
    if (var_guard235 != 0.0) {
        let eq9_e214: f64 = (-p.p148);
        let eq9_e216: f64 = (eq9_e214 * var_ibebtb);
        let eq9_e216_d_n0: f64 = (eq9_e214 * var_ibebtb_dn0);
        let eq9_e216_d_n1: f64 = (eq9_e214 * var_ibebtb_dn1);
        let eq9_e216_d_n2: f64 = (eq9_e214 * var_ibebtb_dn2);
        let eq9_e216_d_n3: f64 = (eq9_e214 * var_ibebtb_dn3);
        let eq9_e216_d_n4: f64 = (eq9_e214 * var_ibebtb_dn4);
        let eq9_e216_d_n5: f64 = (eq9_e214 * var_ibebtb_dn5);
        let eq9_e216_d_n6: f64 = (eq9_e214 * var_ibebtb_dn6);
        let eq9_e216_d_n7: f64 = (eq9_e214 * var_ibebtb_dn7);
        let eq9_e216_d_n8: f64 = (eq9_e214 * var_ibebtb_dn8);
        let eq9_e216_d_n9: f64 = (eq9_e214 * var_ibebtb_dn9);
        let eq9_e216_d_n10: f64 = (eq9_e214 * var_ibebtb_dn10);
        let eq9_e216_d_n11: f64 = (eq9_e214 * var_ibebtb_dn11);
        let eq9_e216_d_n12: f64 = (eq9_e214 * var_ibebtb_dn12);
        let eq9_e216_d_n13: f64 = (eq9_e214 * var_ibebtb_dn13);
        let eq9_e216_d_n14: f64 = (eq9_e214 * var_ibebtb_dn14);
        let eq9_e216_d_b0: f64 = (eq9_e214 * var_ibebtb_db0);
        let eq9_e216_d_b1: f64 = (eq9_e214 * var_ibebtb_db1);
        let eq9_e216_d_b2: f64 = (eq9_e214 * var_ibebtb_db2);
        let eq9_e216_d_b3: f64 = (eq9_e214 * var_ibebtb_db3);
        let eq9_e216_d_b4: f64 = (eq9_e214 * var_ibebtb_db4);
        let eq9_e216_d_b5: f64 = (eq9_e214 * var_ibebtb_db5);
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
    if (var_guard235 == 0.0) {
        let eq10_e222: f64 = (-p.p148);
        let eq10_e224: f64 = (eq10_e222 * var_ibebtb);
        let eq10_e224_d_n0: f64 = (eq10_e222 * var_ibebtb_dn0);
        let eq10_e224_d_n1: f64 = (eq10_e222 * var_ibebtb_dn1);
        let eq10_e224_d_n2: f64 = (eq10_e222 * var_ibebtb_dn2);
        let eq10_e224_d_n3: f64 = (eq10_e222 * var_ibebtb_dn3);
        let eq10_e224_d_n4: f64 = (eq10_e222 * var_ibebtb_dn4);
        let eq10_e224_d_n5: f64 = (eq10_e222 * var_ibebtb_dn5);
        let eq10_e224_d_n6: f64 = (eq10_e222 * var_ibebtb_dn6);
        let eq10_e224_d_n7: f64 = (eq10_e222 * var_ibebtb_dn7);
        let eq10_e224_d_n8: f64 = (eq10_e222 * var_ibebtb_dn8);
        let eq10_e224_d_n9: f64 = (eq10_e222 * var_ibebtb_dn9);
        let eq10_e224_d_n10: f64 = (eq10_e222 * var_ibebtb_dn10);
        let eq10_e224_d_n11: f64 = (eq10_e222 * var_ibebtb_dn11);
        let eq10_e224_d_n12: f64 = (eq10_e222 * var_ibebtb_dn12);
        let eq10_e224_d_n13: f64 = (eq10_e222 * var_ibebtb_dn13);
        let eq10_e224_d_n14: f64 = (eq10_e222 * var_ibebtb_dn14);
        let eq10_e224_d_b0: f64 = (eq10_e222 * var_ibebtb_db0);
        let eq10_e224_d_b1: f64 = (eq10_e222 * var_ibebtb_db1);
        let eq10_e224_d_b2: f64 = (eq10_e222 * var_ibebtb_db2);
        let eq10_e224_d_b3: f64 = (eq10_e222 * var_ibebtb_db3);
        let eq10_e224_d_b4: f64 = (eq10_e222 * var_ibebtb_db4);
        let eq10_e224_d_b5: f64 = (eq10_e222 * var_ibebtb_db5);
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
        var_guard236: f64,
        var_guard237: f64,
        var_guard238: f64,
        var_guard239: f64,
        var_guard240: f64,
        var_ibcbtb: f64,
        var_ibcbtb_db0: f64,
        var_ibcbtb_db1: f64,
        var_ibcbtb_db2: f64,
        var_ibcbtb_db3: f64,
        var_ibcbtb_db4: f64,
        var_ibcbtb_db5: f64,
        var_ibcbtb_dn0: f64,
        var_ibcbtb_dn1: f64,
        var_ibcbtb_dn10: f64,
        var_ibcbtb_dn11: f64,
        var_ibcbtb_dn12: f64,
        var_ibcbtb_dn13: f64,
        var_ibcbtb_dn14: f64,
        var_ibcbtb_dn2: f64,
        var_ibcbtb_dn3: f64,
        var_ibcbtb_dn4: f64,
        var_ibcbtb_dn5: f64,
        var_ibcbtb_dn6: f64,
        var_ibcbtb_dn7: f64,
        var_ibcbtb_dn8: f64,
        var_ibcbtb_dn9: f64,
        var_ibep: f64,
        var_ibep_db0: f64,
        var_ibep_db1: f64,
        var_ibep_db2: f64,
        var_ibep_db3: f64,
        var_ibep_db4: f64,
        var_ibep_db5: f64,
        var_ibep_dn0: f64,
        var_ibep_dn1: f64,
        var_ibep_dn10: f64,
        var_ibep_dn11: f64,
        var_ibep_dn12: f64,
        var_ibep_dn13: f64,
        var_ibep_dn14: f64,
        var_ibep_dn2: f64,
        var_ibep_dn3: f64,
        var_ibep_dn4: f64,
        var_ibep_dn5: f64,
        var_ibep_dn6: f64,
        var_ibep_dn7: f64,
        var_ibep_dn8: f64,
        var_ibep_dn9: f64,
        var_ijbcx: f64,
        var_ijbcx_db0: f64,
        var_ijbcx_db1: f64,
        var_ijbcx_db2: f64,
        var_ijbcx_db3: f64,
        var_ijbcx_db4: f64,
        var_ijbcx_db5: f64,
        var_ijbcx_dn0: f64,
        var_ijbcx_dn1: f64,
        var_ijbcx_dn10: f64,
        var_ijbcx_dn11: f64,
        var_ijbcx_dn12: f64,
        var_ijbcx_dn13: f64,
        var_ijbcx_dn14: f64,
        var_ijbcx_dn2: f64,
        var_ijbcx_dn3: f64,
        var_ijbcx_dn4: f64,
        var_ijbcx_dn5: f64,
        var_ijbcx_dn6: f64,
        var_ijbcx_dn7: f64,
        var_ijbcx_dn8: f64,
        var_ijbcx_dn9: f64,
        var_ijsc: f64,
        var_ijsc_db0: f64,
        var_ijsc_db1: f64,
        var_ijsc_db2: f64,
        var_ijsc_db3: f64,
        var_ijsc_db4: f64,
        var_ijsc_db5: f64,
        var_ijsc_dn0: f64,
        var_ijsc_dn1: f64,
        var_ijsc_dn10: f64,
        var_ijsc_dn11: f64,
        var_ijsc_dn12: f64,
        var_ijsc_dn13: f64,
        var_ijsc_dn14: f64,
        var_ijsc_dn2: f64,
        var_ijsc_dn3: f64,
        var_ijsc_dn4: f64,
        var_ijsc_dn5: f64,
        var_ijsc_dn6: f64,
        var_ijsc_dn7: f64,
        var_ijsc_dn8: f64,
        var_ijsc_dn9: f64,
        var_irep: f64,
        var_irep_db0: f64,
        var_irep_db1: f64,
        var_irep_db2: f64,
        var_irep_db3: f64,
        var_irep_db4: f64,
        var_irep_db5: f64,
        var_irep_dn0: f64,
        var_irep_dn1: f64,
        var_irep_dn10: f64,
        var_irep_dn11: f64,
        var_irep_dn12: f64,
        var_irep_dn13: f64,
        var_irep_dn14: f64,
        var_irep_dn2: f64,
        var_irep_dn3: f64,
        var_irep_dn4: f64,
        var_irep_dn5: f64,
        var_irep_dn6: f64,
        var_irep_dn7: f64,
        var_irep_dn8: f64,
        var_irep_dn9: f64,
        var_it_sub: f64,
        var_it_sub_db0: f64,
        var_it_sub_db1: f64,
        var_it_sub_db2: f64,
        var_it_sub_db3: f64,
        var_it_sub_db4: f64,
        var_it_sub_db5: f64,
        var_it_sub_dn0: f64,
        var_it_sub_dn1: f64,
        var_it_sub_dn10: f64,
        var_it_sub_dn11: f64,
        var_it_sub_dn12: f64,
        var_it_sub_dn13: f64,
        var_it_sub_dn14: f64,
        var_it_sub_dn2: f64,
        var_it_sub_dn3: f64,
        var_it_sub_dn4: f64,
        var_it_sub_dn5: f64,
        var_it_sub_dn6: f64,
        var_it_sub_dn7: f64,
        var_it_sub_dn8: f64,
        var_it_sub_dn9: f64,
        var_qdsu: f64,
        var_qdsu_db0: f64,
        var_qdsu_db1: f64,
        var_qdsu_db2: f64,
        var_qdsu_db3: f64,
        var_qdsu_db4: f64,
        var_qdsu_db5: f64,
        var_qdsu_dn0: f64,
        var_qdsu_dn1: f64,
        var_qdsu_dn10: f64,
        var_qdsu_dn11: f64,
        var_qdsu_dn12: f64,
        var_qdsu_dn13: f64,
        var_qdsu_dn14: f64,
        var_qdsu_dn2: f64,
        var_qdsu_dn3: f64,
        var_qdsu_dn4: f64,
        var_qdsu_dn5: f64,
        var_qdsu_dn6: f64,
        var_qdsu_dn7: f64,
        var_qdsu_dn8: f64,
        var_qdsu_dn9: f64,
        var_qjcx0_t_p: f64,
        var_qjcx0_t_p_db0: f64,
        var_qjcx0_t_p_db1: f64,
        var_qjcx0_t_p_db2: f64,
        var_qjcx0_t_p_db3: f64,
        var_qjcx0_t_p_db4: f64,
        var_qjcx0_t_p_db5: f64,
        var_qjcx0_t_p_dn0: f64,
        var_qjcx0_t_p_dn1: f64,
        var_qjcx0_t_p_dn10: f64,
        var_qjcx0_t_p_dn11: f64,
        var_qjcx0_t_p_dn12: f64,
        var_qjcx0_t_p_dn13: f64,
        var_qjcx0_t_p_dn14: f64,
        var_qjcx0_t_p_dn2: f64,
        var_qjcx0_t_p_dn3: f64,
        var_qjcx0_t_p_dn4: f64,
        var_qjcx0_t_p_dn5: f64,
        var_qjcx0_t_p_dn6: f64,
        var_qjcx0_t_p_dn7: f64,
        var_qjcx0_t_p_dn8: f64,
        var_qjcx0_t_p_dn9: f64,
        var_qjcx0_t_x: f64,
        var_qjcx0_t_x_db0: f64,
        var_qjcx0_t_x_db1: f64,
        var_qjcx0_t_x_db2: f64,
        var_qjcx0_t_x_db3: f64,
        var_qjcx0_t_x_db4: f64,
        var_qjcx0_t_x_db5: f64,
        var_qjcx0_t_x_dn0: f64,
        var_qjcx0_t_x_dn1: f64,
        var_qjcx0_t_x_dn10: f64,
        var_qjcx0_t_x_dn11: f64,
        var_qjcx0_t_x_dn12: f64,
        var_qjcx0_t_x_dn13: f64,
        var_qjcx0_t_x_dn14: f64,
        var_qjcx0_t_x_dn2: f64,
        var_qjcx0_t_x_dn3: f64,
        var_qjcx0_t_x_dn4: f64,
        var_qjcx0_t_x_dn5: f64,
        var_qjcx0_t_x_dn6: f64,
        var_qjcx0_t_x_dn7: f64,
        var_qjcx0_t_x_dn8: f64,
        var_qjcx0_t_x_dn9: f64,
        var_qjep: f64,
        var_qjep_db0: f64,
        var_qjep_db1: f64,
        var_qjep_db2: f64,
        var_qjep_db3: f64,
        var_qjep_db4: f64,
        var_qjep_db5: f64,
        var_qjep_dn0: f64,
        var_qjep_dn1: f64,
        var_qjep_dn10: f64,
        var_qjep_dn11: f64,
        var_qjep_dn12: f64,
        var_qjep_dn13: f64,
        var_qjep_dn14: f64,
        var_qjep_dn2: f64,
        var_qjep_dn3: f64,
        var_qjep_dn4: f64,
        var_qjep_dn5: f64,
        var_qjep_dn6: f64,
        var_qjep_dn7: f64,
        var_qjep_dn8: f64,
        var_qjep_dn9: f64,
        var_rbx_t: f64,
        var_rbx_t_db0: f64,
        var_rbx_t_db1: f64,
        var_rbx_t_db2: f64,
        var_rbx_t_db3: f64,
        var_rbx_t_db4: f64,
        var_rbx_t_db5: f64,
        var_rbx_t_dn0: f64,
        var_rbx_t_dn1: f64,
        var_rbx_t_dn10: f64,
        var_rbx_t_dn11: f64,
        var_rbx_t_dn12: f64,
        var_rbx_t_dn13: f64,
        var_rbx_t_dn14: f64,
        var_rbx_t_dn2: f64,
        var_rbx_t_dn3: f64,
        var_rbx_t_dn4: f64,
        var_rbx_t_dn5: f64,
        var_rbx_t_dn6: f64,
        var_rbx_t_dn7: f64,
        var_rbx_t_dn8: f64,
        var_rbx_t_dn9: f64,
        var_rcx_t: f64,
        var_rcx_t_db0: f64,
        var_rcx_t_db1: f64,
        var_rcx_t_db2: f64,
        var_rcx_t_db3: f64,
        var_rcx_t_db4: f64,
        var_rcx_t_db5: f64,
        var_rcx_t_dn0: f64,
        var_rcx_t_dn1: f64,
        var_rcx_t_dn10: f64,
        var_rcx_t_dn11: f64,
        var_rcx_t_dn12: f64,
        var_rcx_t_dn13: f64,
        var_rcx_t_dn14: f64,
        var_rcx_t_dn2: f64,
        var_rcx_t_dn3: f64,
        var_rcx_t_dn4: f64,
        var_rcx_t_dn5: f64,
        var_rcx_t_dn6: f64,
        var_rcx_t_dn7: f64,
        var_rcx_t_dn8: f64,
        var_rcx_t_dn9: f64,
        var_re_t: f64,
        var_re_t_db0: f64,
        var_re_t_db1: f64,
        var_re_t_db2: f64,
        var_re_t_db3: f64,
        var_re_t_db4: f64,
        var_re_t_db5: f64,
        var_re_t_dn0: f64,
        var_re_t_dn1: f64,
        var_re_t_dn10: f64,
        var_re_t_dn11: f64,
        var_re_t_dn12: f64,
        var_re_t_dn13: f64,
        var_re_t_dn14: f64,
        var_re_t_dn2: f64,
        var_re_t_dn3: f64,
        var_re_t_dn4: f64,
        var_re_t_dn5: f64,
        var_re_t_dn6: f64,
        var_re_t_dn7: f64,
        var_re_t_dn8: f64,
        var_re_t_dn9: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let eq11_e228: f64 = (-p.p148);
        let eq11_e230: f64 = (eq11_e228 * var_ibcbtb);
        let eq11_e230_d_n0: f64 = (eq11_e228 * var_ibcbtb_dn0);
        let eq11_e230_d_n1: f64 = (eq11_e228 * var_ibcbtb_dn1);
        let eq11_e230_d_n2: f64 = (eq11_e228 * var_ibcbtb_dn2);
        let eq11_e230_d_n3: f64 = (eq11_e228 * var_ibcbtb_dn3);
        let eq11_e230_d_n4: f64 = (eq11_e228 * var_ibcbtb_dn4);
        let eq11_e230_d_n5: f64 = (eq11_e228 * var_ibcbtb_dn5);
        let eq11_e230_d_n6: f64 = (eq11_e228 * var_ibcbtb_dn6);
        let eq11_e230_d_n7: f64 = (eq11_e228 * var_ibcbtb_dn7);
        let eq11_e230_d_n8: f64 = (eq11_e228 * var_ibcbtb_dn8);
        let eq11_e230_d_n9: f64 = (eq11_e228 * var_ibcbtb_dn9);
        let eq11_e230_d_n10: f64 = (eq11_e228 * var_ibcbtb_dn10);
        let eq11_e230_d_n11: f64 = (eq11_e228 * var_ibcbtb_dn11);
        let eq11_e230_d_n12: f64 = (eq11_e228 * var_ibcbtb_dn12);
        let eq11_e230_d_n13: f64 = (eq11_e228 * var_ibcbtb_dn13);
        let eq11_e230_d_n14: f64 = (eq11_e228 * var_ibcbtb_dn14);
        let eq11_e230_d_b0: f64 = (eq11_e228 * var_ibcbtb_db0);
        let eq11_e230_d_b1: f64 = (eq11_e228 * var_ibcbtb_db1);
        let eq11_e230_d_b2: f64 = (eq11_e228 * var_ibcbtb_db2);
        let eq11_e230_d_b3: f64 = (eq11_e228 * var_ibcbtb_db3);
        let eq11_e230_d_b4: f64 = (eq11_e228 * var_ibcbtb_db4);
        let eq11_e230_d_b5: f64 = (eq11_e228 * var_ibcbtb_db5);
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
        let eq12_e234: f64 = (var_ibep + var_irep);
        let eq12_e234_d_n0: f64 = (var_ibep_dn0 + var_irep_dn0);
        let eq12_e234_d_n1: f64 = (var_ibep_dn1 + var_irep_dn1);
        let eq12_e234_d_n2: f64 = (var_ibep_dn2 + var_irep_dn2);
        let eq12_e234_d_n3: f64 = (var_ibep_dn3 + var_irep_dn3);
        let eq12_e234_d_n4: f64 = (var_ibep_dn4 + var_irep_dn4);
        let eq12_e234_d_n5: f64 = (var_ibep_dn5 + var_irep_dn5);
        let eq12_e234_d_n6: f64 = (var_ibep_dn6 + var_irep_dn6);
        let eq12_e234_d_n7: f64 = (var_ibep_dn7 + var_irep_dn7);
        let eq12_e234_d_n8: f64 = (var_ibep_dn8 + var_irep_dn8);
        let eq12_e234_d_n9: f64 = (var_ibep_dn9 + var_irep_dn9);
        let eq12_e234_d_n10: f64 = (var_ibep_dn10 + var_irep_dn10);
        let eq12_e234_d_n11: f64 = (var_ibep_dn11 + var_irep_dn11);
        let eq12_e234_d_n12: f64 = (var_ibep_dn12 + var_irep_dn12);
        let eq12_e234_d_n13: f64 = (var_ibep_dn13 + var_irep_dn13);
        let eq12_e234_d_n14: f64 = (var_ibep_dn14 + var_irep_dn14);
        let eq12_e234_d_b0: f64 = (var_ibep_db0 + var_irep_db0);
        let eq12_e234_d_b1: f64 = (var_ibep_db1 + var_irep_db1);
        let eq12_e234_d_b2: f64 = (var_ibep_db2 + var_irep_db2);
        let eq12_e234_d_b3: f64 = (var_ibep_db3 + var_irep_db3);
        let eq12_e234_d_b4: f64 = (var_ibep_db4 + var_irep_db4);
        let eq12_e234_d_b5: f64 = (var_ibep_db5 + var_irep_db5);
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
        let eq13_e238: f64 = (p.p148 * var_qjep);
        let eq13_e238_d_n0: f64 = (p.p148 * var_qjep_dn0);
        let eq13_e238_d_n1: f64 = (p.p148 * var_qjep_dn1);
        let eq13_e238_d_n2: f64 = (p.p148 * var_qjep_dn2);
        let eq13_e238_d_n3: f64 = (p.p148 * var_qjep_dn3);
        let eq13_e238_d_n4: f64 = (p.p148 * var_qjep_dn4);
        let eq13_e238_d_n5: f64 = (p.p148 * var_qjep_dn5);
        let eq13_e238_d_n6: f64 = (p.p148 * var_qjep_dn6);
        let eq13_e238_d_n7: f64 = (p.p148 * var_qjep_dn7);
        let eq13_e238_d_n8: f64 = (p.p148 * var_qjep_dn8);
        let eq13_e238_d_n9: f64 = (p.p148 * var_qjep_dn9);
        let eq13_e238_d_n10: f64 = (p.p148 * var_qjep_dn10);
        let eq13_e238_d_n11: f64 = (p.p148 * var_qjep_dn11);
        let eq13_e238_d_n12: f64 = (p.p148 * var_qjep_dn12);
        let eq13_e238_d_n13: f64 = (p.p148 * var_qjep_dn13);
        let eq13_e238_d_n14: f64 = (p.p148 * var_qjep_dn14);
        let eq13_e238_d_b0: f64 = (p.p148 * var_qjep_db0);
        let eq13_e238_d_b1: f64 = (p.p148 * var_qjep_db1);
        let eq13_e238_d_b2: f64 = (p.p148 * var_qjep_db2);
        let eq13_e238_d_b3: f64 = (p.p148 * var_qjep_db3);
        let eq13_e238_d_b4: f64 = (p.p148 * var_qjep_db4);
        let eq13_e238_d_b5: f64 = (p.p148 * var_qjep_db5);
        let eq13_e239: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq13_e238);
        let eq13_value: f64 = eq13_e239;
        let eq13_node_derivatives: [f64; 15] = [(eq13_e238_d_n0 * ddt_scale), (eq13_e238_d_n1 * ddt_scale), (eq13_e238_d_n2 * ddt_scale), (eq13_e238_d_n3 * ddt_scale), (eq13_e238_d_n4 * ddt_scale), (eq13_e238_d_n5 * ddt_scale), (eq13_e238_d_n6 * ddt_scale), (eq13_e238_d_n7 * ddt_scale), (eq13_e238_d_n8 * ddt_scale), (eq13_e238_d_n9 * ddt_scale), (eq13_e238_d_n10 * ddt_scale), (eq13_e238_d_n11 * ddt_scale), (eq13_e238_d_n12 * ddt_scale), (eq13_e238_d_n13 * ddt_scale), (eq13_e238_d_n14 * ddt_scale)];
        let eq13_branch_derivatives: [f64; 6] = [(eq13_e238_d_b0 * ddt_scale), (eq13_e238_d_b1 * ddt_scale), (eq13_e238_d_b2 * ddt_scale), (eq13_e238_d_b3 * ddt_scale), (eq13_e238_d_b4 * ddt_scale), (eq13_e238_d_b5 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_e242: f64 = (p.p148 * var_ijbcx);
        let eq14_e242_d_n0: f64 = (p.p148 * var_ijbcx_dn0);
        let eq14_e242_d_n1: f64 = (p.p148 * var_ijbcx_dn1);
        let eq14_e242_d_n2: f64 = (p.p148 * var_ijbcx_dn2);
        let eq14_e242_d_n3: f64 = (p.p148 * var_ijbcx_dn3);
        let eq14_e242_d_n4: f64 = (p.p148 * var_ijbcx_dn4);
        let eq14_e242_d_n5: f64 = (p.p148 * var_ijbcx_dn5);
        let eq14_e242_d_n6: f64 = (p.p148 * var_ijbcx_dn6);
        let eq14_e242_d_n7: f64 = (p.p148 * var_ijbcx_dn7);
        let eq14_e242_d_n8: f64 = (p.p148 * var_ijbcx_dn8);
        let eq14_e242_d_n9: f64 = (p.p148 * var_ijbcx_dn9);
        let eq14_e242_d_n10: f64 = (p.p148 * var_ijbcx_dn10);
        let eq14_e242_d_n11: f64 = (p.p148 * var_ijbcx_dn11);
        let eq14_e242_d_n12: f64 = (p.p148 * var_ijbcx_dn12);
        let eq14_e242_d_n13: f64 = (p.p148 * var_ijbcx_dn13);
        let eq14_e242_d_n14: f64 = (p.p148 * var_ijbcx_dn14);
        let eq14_e242_d_b0: f64 = (p.p148 * var_ijbcx_db0);
        let eq14_e242_d_b1: f64 = (p.p148 * var_ijbcx_db1);
        let eq14_e242_d_b2: f64 = (p.p148 * var_ijbcx_db2);
        let eq14_e242_d_b3: f64 = (p.p148 * var_ijbcx_db3);
        let eq14_e242_d_b4: f64 = (p.p148 * var_ijbcx_db4);
        let eq14_e242_d_b5: f64 = (p.p148 * var_ijbcx_db5);
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
        let eq15_e246: f64 = (var_qjcx0_t_p + var_qdsu);
        let eq15_e246_d_n0: f64 = (var_qjcx0_t_p_dn0 + var_qdsu_dn0);
        let eq15_e246_d_n1: f64 = (var_qjcx0_t_p_dn1 + var_qdsu_dn1);
        let eq15_e246_d_n2: f64 = (var_qjcx0_t_p_dn2 + var_qdsu_dn2);
        let eq15_e246_d_n3: f64 = (var_qjcx0_t_p_dn3 + var_qdsu_dn3);
        let eq15_e246_d_n4: f64 = (var_qjcx0_t_p_dn4 + var_qdsu_dn4);
        let eq15_e246_d_n5: f64 = (var_qjcx0_t_p_dn5 + var_qdsu_dn5);
        let eq15_e246_d_n6: f64 = (var_qjcx0_t_p_dn6 + var_qdsu_dn6);
        let eq15_e246_d_n7: f64 = (var_qjcx0_t_p_dn7 + var_qdsu_dn7);
        let eq15_e246_d_n8: f64 = (var_qjcx0_t_p_dn8 + var_qdsu_dn8);
        let eq15_e246_d_n9: f64 = (var_qjcx0_t_p_dn9 + var_qdsu_dn9);
        let eq15_e246_d_n10: f64 = (var_qjcx0_t_p_dn10 + var_qdsu_dn10);
        let eq15_e246_d_n11: f64 = (var_qjcx0_t_p_dn11 + var_qdsu_dn11);
        let eq15_e246_d_n12: f64 = (var_qjcx0_t_p_dn12 + var_qdsu_dn12);
        let eq15_e246_d_n13: f64 = (var_qjcx0_t_p_dn13 + var_qdsu_dn13);
        let eq15_e246_d_n14: f64 = (var_qjcx0_t_p_dn14 + var_qdsu_dn14);
        let eq15_e246_d_b0: f64 = (var_qjcx0_t_p_db0 + var_qdsu_db0);
        let eq15_e246_d_b1: f64 = (var_qjcx0_t_p_db1 + var_qdsu_db1);
        let eq15_e246_d_b2: f64 = (var_qjcx0_t_p_db2 + var_qdsu_db2);
        let eq15_e246_d_b3: f64 = (var_qjcx0_t_p_db3 + var_qdsu_db3);
        let eq15_e246_d_b4: f64 = (var_qjcx0_t_p_db4 + var_qdsu_db4);
        let eq15_e246_d_b5: f64 = (var_qjcx0_t_p_db5 + var_qdsu_db5);
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
        let eq15_e248: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq15_e247);
        let eq15_value: f64 = eq15_e248;
        let eq15_node_derivatives: [f64; 15] = [(eq15_e247_d_n0 * ddt_scale), (eq15_e247_d_n1 * ddt_scale), (eq15_e247_d_n2 * ddt_scale), (eq15_e247_d_n3 * ddt_scale), (eq15_e247_d_n4 * ddt_scale), (eq15_e247_d_n5 * ddt_scale), (eq15_e247_d_n6 * ddt_scale), (eq15_e247_d_n7 * ddt_scale), (eq15_e247_d_n8 * ddt_scale), (eq15_e247_d_n9 * ddt_scale), (eq15_e247_d_n10 * ddt_scale), (eq15_e247_d_n11 * ddt_scale), (eq15_e247_d_n12 * ddt_scale), (eq15_e247_d_n13 * ddt_scale), (eq15_e247_d_n14 * ddt_scale)];
        let eq15_branch_derivatives: [f64; 6] = [(eq15_e247_d_b0 * ddt_scale), (eq15_e247_d_b1 * ddt_scale), (eq15_e247_d_b2 * ddt_scale), (eq15_e247_d_b3 * ddt_scale), (eq15_e247_d_b4 * ddt_scale), (eq15_e247_d_b5 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let eq17_e255: f64 = (p.p148 * var_qjcx0_t_x);
        let eq17_e255_d_n0: f64 = (p.p148 * var_qjcx0_t_x_dn0);
        let eq17_e255_d_n1: f64 = (p.p148 * var_qjcx0_t_x_dn1);
        let eq17_e255_d_n2: f64 = (p.p148 * var_qjcx0_t_x_dn2);
        let eq17_e255_d_n3: f64 = (p.p148 * var_qjcx0_t_x_dn3);
        let eq17_e255_d_n4: f64 = (p.p148 * var_qjcx0_t_x_dn4);
        let eq17_e255_d_n5: f64 = (p.p148 * var_qjcx0_t_x_dn5);
        let eq17_e255_d_n6: f64 = (p.p148 * var_qjcx0_t_x_dn6);
        let eq17_e255_d_n7: f64 = (p.p148 * var_qjcx0_t_x_dn7);
        let eq17_e255_d_n8: f64 = (p.p148 * var_qjcx0_t_x_dn8);
        let eq17_e255_d_n9: f64 = (p.p148 * var_qjcx0_t_x_dn9);
        let eq17_e255_d_n10: f64 = (p.p148 * var_qjcx0_t_x_dn10);
        let eq17_e255_d_n11: f64 = (p.p148 * var_qjcx0_t_x_dn11);
        let eq17_e255_d_n12: f64 = (p.p148 * var_qjcx0_t_x_dn12);
        let eq17_e255_d_n13: f64 = (p.p148 * var_qjcx0_t_x_dn13);
        let eq17_e255_d_n14: f64 = (p.p148 * var_qjcx0_t_x_dn14);
        let eq17_e255_d_b0: f64 = (p.p148 * var_qjcx0_t_x_db0);
        let eq17_e255_d_b1: f64 = (p.p148 * var_qjcx0_t_x_db1);
        let eq17_e255_d_b2: f64 = (p.p148 * var_qjcx0_t_x_db2);
        let eq17_e255_d_b3: f64 = (p.p148 * var_qjcx0_t_x_db3);
        let eq17_e255_d_b4: f64 = (p.p148 * var_qjcx0_t_x_db4);
        let eq17_e255_d_b5: f64 = (p.p148 * var_qjcx0_t_x_db5);
        let eq17_e256: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq17_e255);
        let eq17_value: f64 = eq17_e256;
        let eq17_node_derivatives: [f64; 15] = [(eq17_e255_d_n0 * ddt_scale), (eq17_e255_d_n1 * ddt_scale), (eq17_e255_d_n2 * ddt_scale), (eq17_e255_d_n3 * ddt_scale), (eq17_e255_d_n4 * ddt_scale), (eq17_e255_d_n5 * ddt_scale), (eq17_e255_d_n6 * ddt_scale), (eq17_e255_d_n7 * ddt_scale), (eq17_e255_d_n8 * ddt_scale), (eq17_e255_d_n9 * ddt_scale), (eq17_e255_d_n10 * ddt_scale), (eq17_e255_d_n11 * ddt_scale), (eq17_e255_d_n12 * ddt_scale), (eq17_e255_d_n13 * ddt_scale), (eq17_e255_d_n14 * ddt_scale)];
        let eq17_branch_derivatives: [f64; 6] = [(eq17_e255_d_b0 * ddt_scale), (eq17_e255_d_b1 * ddt_scale), (eq17_e255_d_b2 * ddt_scale), (eq17_e255_d_b3 * ddt_scale), (eq17_e255_d_b4 * ddt_scale), (eq17_e255_d_b5 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(5),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let (eq19_e266, eq19_e266_d_n0, eq19_e266_d_n1, eq19_e266_d_n2, eq19_e266_d_n3, eq19_e266_d_n4, eq19_e266_d_n5, eq19_e266_d_n6, eq19_e266_d_n7, eq19_e266_d_n8, eq19_e266_d_n9, eq19_e266_d_n10, eq19_e266_d_n11, eq19_e266_d_n12, eq19_e266_d_n13, eq19_e266_d_n14, eq19_e266_d_b0, eq19_e266_d_b1, eq19_e266_d_b2, eq19_e266_d_b3, eq19_e266_d_b4, eq19_e266_d_b5,) = {
    if (var_guard236 != 0.0) {
        let eq19_e264: f64 = ((nv1 - nv7) / var_rbx_t);
        let eq19_e264_d_n0: f64 = (-(((nv1 - nv7) * var_rbx_t_dn0) / (var_rbx_t * var_rbx_t)));
        let eq19_e264_d_n1: f64 = ((var_rbx_t - ((nv1 - nv7) * var_rbx_t_dn1)) / (var_rbx_t * var_rbx_t));
        let eq19_e264_d_n2: f64 = (-(((nv1 - nv7) * var_rbx_t_dn2) / (var_rbx_t * var_rbx_t)));
        let eq19_e264_d_n3: f64 = (-(((nv1 - nv7) * var_rbx_t_dn3) / (var_rbx_t * var_rbx_t)));
        let eq19_e264_d_n4: f64 = (-(((nv1 - nv7) * var_rbx_t_dn4) / (var_rbx_t * var_rbx_t)));
        let eq19_e264_d_n5: f64 = (-(((nv1 - nv7) * var_rbx_t_dn5) / (var_rbx_t * var_rbx_t)));
        let eq19_e264_d_n6: f64 = (-(((nv1 - nv7) * var_rbx_t_dn6) / (var_rbx_t * var_rbx_t)));
        let eq19_e264_d_n7: f64 = (((-var_rbx_t) - ((nv1 - nv7) * var_rbx_t_dn7)) / (var_rbx_t * var_rbx_t));
        let eq19_e264_d_n8: f64 = (-(((nv1 - nv7) * var_rbx_t_dn8) / (var_rbx_t * var_rbx_t)));
        let eq19_e264_d_n9: f64 = (-(((nv1 - nv7) * var_rbx_t_dn9) / (var_rbx_t * var_rbx_t)));
        let eq19_e264_d_n10: f64 = (-(((nv1 - nv7) * var_rbx_t_dn10) / (var_rbx_t * var_rbx_t)));
        let eq19_e264_d_n11: f64 = (-(((nv1 - nv7) * var_rbx_t_dn11) / (var_rbx_t * var_rbx_t)));
        let eq19_e264_d_n12: f64 = (-(((nv1 - nv7) * var_rbx_t_dn12) / (var_rbx_t * var_rbx_t)));
        let eq19_e264_d_n13: f64 = (-(((nv1 - nv7) * var_rbx_t_dn13) / (var_rbx_t * var_rbx_t)));
        let eq19_e264_d_n14: f64 = (-(((nv1 - nv7) * var_rbx_t_dn14) / (var_rbx_t * var_rbx_t)));
        let eq19_e264_d_b0: f64 = (-(((nv1 - nv7) * var_rbx_t_db0) / (var_rbx_t * var_rbx_t)));
        let eq19_e264_d_b1: f64 = (-(((nv1 - nv7) * var_rbx_t_db1) / (var_rbx_t * var_rbx_t)));
        let eq19_e264_d_b2: f64 = (-(((nv1 - nv7) * var_rbx_t_db2) / (var_rbx_t * var_rbx_t)));
        let eq19_e264_d_b3: f64 = (-(((nv1 - nv7) * var_rbx_t_db3) / (var_rbx_t * var_rbx_t)));
        let eq19_e264_d_b4: f64 = (-(((nv1 - nv7) * var_rbx_t_db4) / (var_rbx_t * var_rbx_t)));
        let eq19_e264_d_b5: f64 = (-(((nv1 - nv7) * var_rbx_t_db5) / (var_rbx_t * var_rbx_t)));
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
        let (eq21_e277, eq21_e277_d_n0, eq21_e277_d_n1, eq21_e277_d_n2, eq21_e277_d_n3, eq21_e277_d_n4, eq21_e277_d_n5, eq21_e277_d_n6, eq21_e277_d_n7, eq21_e277_d_n8, eq21_e277_d_n9, eq21_e277_d_n10, eq21_e277_d_n11, eq21_e277_d_n12, eq21_e277_d_n13, eq21_e277_d_n14, eq21_e277_d_b0, eq21_e277_d_b1, eq21_e277_d_b2, eq21_e277_d_b3, eq21_e277_d_b4, eq21_e277_d_b5,) = {
    if (var_guard237 != 0.0) {
        let eq21_e275: f64 = ((nv6 - nv2) / var_re_t);
        let eq21_e275_d_n0: f64 = (-(((nv6 - nv2) * var_re_t_dn0) / (var_re_t * var_re_t)));
        let eq21_e275_d_n1: f64 = (-(((nv6 - nv2) * var_re_t_dn1) / (var_re_t * var_re_t)));
        let eq21_e275_d_n2: f64 = (((-var_re_t) - ((nv6 - nv2) * var_re_t_dn2)) / (var_re_t * var_re_t));
        let eq21_e275_d_n3: f64 = (-(((nv6 - nv2) * var_re_t_dn3) / (var_re_t * var_re_t)));
        let eq21_e275_d_n4: f64 = (-(((nv6 - nv2) * var_re_t_dn4) / (var_re_t * var_re_t)));
        let eq21_e275_d_n5: f64 = (-(((nv6 - nv2) * var_re_t_dn5) / (var_re_t * var_re_t)));
        let eq21_e275_d_n6: f64 = ((var_re_t - ((nv6 - nv2) * var_re_t_dn6)) / (var_re_t * var_re_t));
        let eq21_e275_d_n7: f64 = (-(((nv6 - nv2) * var_re_t_dn7) / (var_re_t * var_re_t)));
        let eq21_e275_d_n8: f64 = (-(((nv6 - nv2) * var_re_t_dn8) / (var_re_t * var_re_t)));
        let eq21_e275_d_n9: f64 = (-(((nv6 - nv2) * var_re_t_dn9) / (var_re_t * var_re_t)));
        let eq21_e275_d_n10: f64 = (-(((nv6 - nv2) * var_re_t_dn10) / (var_re_t * var_re_t)));
        let eq21_e275_d_n11: f64 = (-(((nv6 - nv2) * var_re_t_dn11) / (var_re_t * var_re_t)));
        let eq21_e275_d_n12: f64 = (-(((nv6 - nv2) * var_re_t_dn12) / (var_re_t * var_re_t)));
        let eq21_e275_d_n13: f64 = (-(((nv6 - nv2) * var_re_t_dn13) / (var_re_t * var_re_t)));
        let eq21_e275_d_n14: f64 = (-(((nv6 - nv2) * var_re_t_dn14) / (var_re_t * var_re_t)));
        let eq21_e275_d_b0: f64 = (-(((nv6 - nv2) * var_re_t_db0) / (var_re_t * var_re_t)));
        let eq21_e275_d_b1: f64 = (-(((nv6 - nv2) * var_re_t_db1) / (var_re_t * var_re_t)));
        let eq21_e275_d_b2: f64 = (-(((nv6 - nv2) * var_re_t_db2) / (var_re_t * var_re_t)));
        let eq21_e275_d_b3: f64 = (-(((nv6 - nv2) * var_re_t_db3) / (var_re_t * var_re_t)));
        let eq21_e275_d_b4: f64 = (-(((nv6 - nv2) * var_re_t_db4) / (var_re_t * var_re_t)));
        let eq21_e275_d_b5: f64 = (-(((nv6 - nv2) * var_re_t_db5) / (var_re_t * var_re_t)));
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
        let (eq23_e288, eq23_e288_d_n0, eq23_e288_d_n1, eq23_e288_d_n2, eq23_e288_d_n3, eq23_e288_d_n4, eq23_e288_d_n5, eq23_e288_d_n6, eq23_e288_d_n7, eq23_e288_d_n8, eq23_e288_d_n9, eq23_e288_d_n10, eq23_e288_d_n11, eq23_e288_d_n12, eq23_e288_d_n13, eq23_e288_d_n14, eq23_e288_d_b0, eq23_e288_d_b1, eq23_e288_d_b2, eq23_e288_d_b3, eq23_e288_d_b4, eq23_e288_d_b5,) = {
    if (var_guard238 != 0.0) {
        let eq23_e286: f64 = ((nv5 - nv0) / var_rcx_t);
        let eq23_e286_d_n0: f64 = (((-var_rcx_t) - ((nv5 - nv0) * var_rcx_t_dn0)) / (var_rcx_t * var_rcx_t));
        let eq23_e286_d_n1: f64 = (-(((nv5 - nv0) * var_rcx_t_dn1) / (var_rcx_t * var_rcx_t)));
        let eq23_e286_d_n2: f64 = (-(((nv5 - nv0) * var_rcx_t_dn2) / (var_rcx_t * var_rcx_t)));
        let eq23_e286_d_n3: f64 = (-(((nv5 - nv0) * var_rcx_t_dn3) / (var_rcx_t * var_rcx_t)));
        let eq23_e286_d_n4: f64 = (-(((nv5 - nv0) * var_rcx_t_dn4) / (var_rcx_t * var_rcx_t)));
        let eq23_e286_d_n5: f64 = ((var_rcx_t - ((nv5 - nv0) * var_rcx_t_dn5)) / (var_rcx_t * var_rcx_t));
        let eq23_e286_d_n6: f64 = (-(((nv5 - nv0) * var_rcx_t_dn6) / (var_rcx_t * var_rcx_t)));
        let eq23_e286_d_n7: f64 = (-(((nv5 - nv0) * var_rcx_t_dn7) / (var_rcx_t * var_rcx_t)));
        let eq23_e286_d_n8: f64 = (-(((nv5 - nv0) * var_rcx_t_dn8) / (var_rcx_t * var_rcx_t)));
        let eq23_e286_d_n9: f64 = (-(((nv5 - nv0) * var_rcx_t_dn9) / (var_rcx_t * var_rcx_t)));
        let eq23_e286_d_n10: f64 = (-(((nv5 - nv0) * var_rcx_t_dn10) / (var_rcx_t * var_rcx_t)));
        let eq23_e286_d_n11: f64 = (-(((nv5 - nv0) * var_rcx_t_dn11) / (var_rcx_t * var_rcx_t)));
        let eq23_e286_d_n12: f64 = (-(((nv5 - nv0) * var_rcx_t_dn12) / (var_rcx_t * var_rcx_t)));
        let eq23_e286_d_n13: f64 = (-(((nv5 - nv0) * var_rcx_t_dn13) / (var_rcx_t * var_rcx_t)));
        let eq23_e286_d_n14: f64 = (-(((nv5 - nv0) * var_rcx_t_dn14) / (var_rcx_t * var_rcx_t)));
        let eq23_e286_d_b0: f64 = (-(((nv5 - nv0) * var_rcx_t_db0) / (var_rcx_t * var_rcx_t)));
        let eq23_e286_d_b1: f64 = (-(((nv5 - nv0) * var_rcx_t_db1) / (var_rcx_t * var_rcx_t)));
        let eq23_e286_d_b2: f64 = (-(((nv5 - nv0) * var_rcx_t_db2) / (var_rcx_t * var_rcx_t)));
        let eq23_e286_d_b3: f64 = (-(((nv5 - nv0) * var_rcx_t_db3) / (var_rcx_t * var_rcx_t)));
        let eq23_e286_d_b4: f64 = (-(((nv5 - nv0) * var_rcx_t_db4) / (var_rcx_t * var_rcx_t)));
        let eq23_e286_d_b5: f64 = (-(((nv5 - nv0) * var_rcx_t_db5) / (var_rcx_t * var_rcx_t)));
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
        let eq28_e308: f64 = (p.p148 * var_it_sub);
        let eq28_e308_d_n0: f64 = (p.p148 * var_it_sub_dn0);
        let eq28_e308_d_n1: f64 = (p.p148 * var_it_sub_dn1);
        let eq28_e308_d_n2: f64 = (p.p148 * var_it_sub_dn2);
        let eq28_e308_d_n3: f64 = (p.p148 * var_it_sub_dn3);
        let eq28_e308_d_n4: f64 = (p.p148 * var_it_sub_dn4);
        let eq28_e308_d_n5: f64 = (p.p148 * var_it_sub_dn5);
        let eq28_e308_d_n6: f64 = (p.p148 * var_it_sub_dn6);
        let eq28_e308_d_n7: f64 = (p.p148 * var_it_sub_dn7);
        let eq28_e308_d_n8: f64 = (p.p148 * var_it_sub_dn8);
        let eq28_e308_d_n9: f64 = (p.p148 * var_it_sub_dn9);
        let eq28_e308_d_n10: f64 = (p.p148 * var_it_sub_dn10);
        let eq28_e308_d_n11: f64 = (p.p148 * var_it_sub_dn11);
        let eq28_e308_d_n12: f64 = (p.p148 * var_it_sub_dn12);
        let eq28_e308_d_n13: f64 = (p.p148 * var_it_sub_dn13);
        let eq28_e308_d_n14: f64 = (p.p148 * var_it_sub_dn14);
        let eq28_e308_d_b0: f64 = (p.p148 * var_it_sub_db0);
        let eq28_e308_d_b1: f64 = (p.p148 * var_it_sub_db1);
        let eq28_e308_d_b2: f64 = (p.p148 * var_it_sub_db2);
        let eq28_e308_d_b3: f64 = (p.p148 * var_it_sub_db3);
        let eq28_e308_d_b4: f64 = (p.p148 * var_it_sub_db4);
        let eq28_e308_d_b5: f64 = (p.p148 * var_it_sub_db5);
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
    if ((var_guard239 != 0.0) && (var_guard240 != 0.0)) {
        let eq29_e314: f64 = (p.p148 * var_ijsc);
        let eq29_e314_d_n0: f64 = (p.p148 * var_ijsc_dn0);
        let eq29_e314_d_n1: f64 = (p.p148 * var_ijsc_dn1);
        let eq29_e314_d_n2: f64 = (p.p148 * var_ijsc_dn2);
        let eq29_e314_d_n3: f64 = (p.p148 * var_ijsc_dn3);
        let eq29_e314_d_n4: f64 = (p.p148 * var_ijsc_dn4);
        let eq29_e314_d_n5: f64 = (p.p148 * var_ijsc_dn5);
        let eq29_e314_d_n6: f64 = (p.p148 * var_ijsc_dn6);
        let eq29_e314_d_n7: f64 = (p.p148 * var_ijsc_dn7);
        let eq29_e314_d_n8: f64 = (p.p148 * var_ijsc_dn8);
        let eq29_e314_d_n9: f64 = (p.p148 * var_ijsc_dn9);
        let eq29_e314_d_n10: f64 = (p.p148 * var_ijsc_dn10);
        let eq29_e314_d_n11: f64 = (p.p148 * var_ijsc_dn11);
        let eq29_e314_d_n12: f64 = (p.p148 * var_ijsc_dn12);
        let eq29_e314_d_n13: f64 = (p.p148 * var_ijsc_dn13);
        let eq29_e314_d_n14: f64 = (p.p148 * var_ijsc_dn14);
        let eq29_e314_d_b0: f64 = (p.p148 * var_ijsc_db0);
        let eq29_e314_d_b1: f64 = (p.p148 * var_ijsc_db1);
        let eq29_e314_d_b2: f64 = (p.p148 * var_ijsc_db2);
        let eq29_e314_d_b3: f64 = (p.p148 * var_ijsc_db3);
        let eq29_e314_d_b4: f64 = (p.p148 * var_ijsc_db4);
        let eq29_e314_d_b5: f64 = (p.p148 * var_ijsc_db5);
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
    if (var_guard239 == 0.0) {
        let eq31_e329: f64 = (p.p148 * var_ijsc);
        let eq31_e329_d_n0: f64 = (p.p148 * var_ijsc_dn0);
        let eq31_e329_d_n1: f64 = (p.p148 * var_ijsc_dn1);
        let eq31_e329_d_n2: f64 = (p.p148 * var_ijsc_dn2);
        let eq31_e329_d_n3: f64 = (p.p148 * var_ijsc_dn3);
        let eq31_e329_d_n4: f64 = (p.p148 * var_ijsc_dn4);
        let eq31_e329_d_n5: f64 = (p.p148 * var_ijsc_dn5);
        let eq31_e329_d_n6: f64 = (p.p148 * var_ijsc_dn6);
        let eq31_e329_d_n7: f64 = (p.p148 * var_ijsc_dn7);
        let eq31_e329_d_n8: f64 = (p.p148 * var_ijsc_dn8);
        let eq31_e329_d_n9: f64 = (p.p148 * var_ijsc_dn9);
        let eq31_e329_d_n10: f64 = (p.p148 * var_ijsc_dn10);
        let eq31_e329_d_n11: f64 = (p.p148 * var_ijsc_dn11);
        let eq31_e329_d_n12: f64 = (p.p148 * var_ijsc_dn12);
        let eq31_e329_d_n13: f64 = (p.p148 * var_ijsc_dn13);
        let eq31_e329_d_n14: f64 = (p.p148 * var_ijsc_dn14);
        let eq31_e329_d_b0: f64 = (p.p148 * var_ijsc_db0);
        let eq31_e329_d_b1: f64 = (p.p148 * var_ijsc_db1);
        let eq31_e329_d_b2: f64 = (p.p148 * var_ijsc_db2);
        let eq31_e329_d_b3: f64 = (p.p148 * var_ijsc_db3);
        let eq31_e329_d_b4: f64 = (p.p148 * var_ijsc_db4);
        let eq31_e329_d_b5: f64 = (p.p148 * var_ijsc_db5);
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
    }

    pub(super) fn stamp_transient_equations_block_2(
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
        var_guard242: f64,
        var_guard243: f64,
        var_guard244: f64,
        var_guard245: f64,
        var_guard258: f64,
        var_ixf: f64,
        var_ixf1: f64,
        var_ixf1_db0: f64,
        var_ixf1_db1: f64,
        var_ixf1_db2: f64,
        var_ixf1_db3: f64,
        var_ixf1_db4: f64,
        var_ixf1_db5: f64,
        var_ixf1_dn0: f64,
        var_ixf1_dn1: f64,
        var_ixf1_dn10: f64,
        var_ixf1_dn11: f64,
        var_ixf1_dn12: f64,
        var_ixf1_dn13: f64,
        var_ixf1_dn14: f64,
        var_ixf1_dn2: f64,
        var_ixf1_dn3: f64,
        var_ixf1_dn4: f64,
        var_ixf1_dn5: f64,
        var_ixf1_dn6: f64,
        var_ixf1_dn7: f64,
        var_ixf1_dn8: f64,
        var_ixf1_dn9: f64,
        var_ixf2: f64,
        var_ixf2_db0: f64,
        var_ixf2_db1: f64,
        var_ixf2_db2: f64,
        var_ixf2_db3: f64,
        var_ixf2_db4: f64,
        var_ixf2_db5: f64,
        var_ixf2_dn0: f64,
        var_ixf2_dn1: f64,
        var_ixf2_dn10: f64,
        var_ixf2_dn11: f64,
        var_ixf2_dn12: f64,
        var_ixf2_dn13: f64,
        var_ixf2_dn14: f64,
        var_ixf2_dn2: f64,
        var_ixf2_dn3: f64,
        var_ixf2_dn4: f64,
        var_ixf2_dn5: f64,
        var_ixf2_dn6: f64,
        var_ixf2_dn7: f64,
        var_ixf2_dn8: f64,
        var_ixf2_dn9: f64,
        var_ixf_db0: f64,
        var_ixf_db1: f64,
        var_ixf_db2: f64,
        var_ixf_db3: f64,
        var_ixf_db4: f64,
        var_ixf_db5: f64,
        var_ixf_dn0: f64,
        var_ixf_dn1: f64,
        var_ixf_dn10: f64,
        var_ixf_dn11: f64,
        var_ixf_dn12: f64,
        var_ixf_dn13: f64,
        var_ixf_dn14: f64,
        var_ixf_dn2: f64,
        var_ixf_dn3: f64,
        var_ixf_dn4: f64,
        var_ixf_dn5: f64,
        var_ixf_dn6: f64,
        var_ixf_dn7: f64,
        var_ixf_dn8: f64,
        var_ixf_dn9: f64,
        var_n_1: f64,
        var_n_1_db0: f64,
        var_n_1_db1: f64,
        var_n_1_db2: f64,
        var_n_1_db3: f64,
        var_n_1_db4: f64,
        var_n_1_db5: f64,
        var_n_1_dn0: f64,
        var_n_1_dn1: f64,
        var_n_1_dn10: f64,
        var_n_1_dn11: f64,
        var_n_1_dn12: f64,
        var_n_1_dn13: f64,
        var_n_1_dn14: f64,
        var_n_1_dn2: f64,
        var_n_1_dn3: f64,
        var_n_1_dn4: f64,
        var_n_1_dn5: f64,
        var_n_1_dn6: f64,
        var_n_1_dn7: f64,
        var_n_1_dn8: f64,
        var_n_1_dn9: f64,
        var_n_2: f64,
        var_n_2_db0: f64,
        var_n_2_db1: f64,
        var_n_2_db2: f64,
        var_n_2_db3: f64,
        var_n_2_db4: f64,
        var_n_2_db5: f64,
        var_n_2_dn0: f64,
        var_n_2_dn1: f64,
        var_n_2_dn10: f64,
        var_n_2_dn11: f64,
        var_n_2_dn12: f64,
        var_n_2_dn13: f64,
        var_n_2_dn14: f64,
        var_n_2_dn2: f64,
        var_n_2_dn3: f64,
        var_n_2_dn4: f64,
        var_n_2_dn5: f64,
        var_n_2_dn6: f64,
        var_n_2_dn7: f64,
        var_n_2_dn8: f64,
        var_n_2_dn9: f64,
        var_n_w: f64,
        var_pterm: f64,
        var_pterm_db0: f64,
        var_pterm_db1: f64,
        var_pterm_db2: f64,
        var_pterm_db3: f64,
        var_pterm_db4: f64,
        var_pterm_db5: f64,
        var_pterm_dn0: f64,
        var_pterm_dn1: f64,
        var_pterm_dn10: f64,
        var_pterm_dn11: f64,
        var_pterm_dn12: f64,
        var_pterm_dn13: f64,
        var_pterm_dn14: f64,
        var_pterm_dn2: f64,
        var_pterm_dn3: f64,
        var_pterm_dn4: f64,
        var_pterm_dn5: f64,
        var_pterm_dn6: f64,
        var_pterm_dn7: f64,
        var_pterm_dn8: f64,
        var_pterm_dn9: f64,
        var_qjs: f64,
        var_qjs_db0: f64,
        var_qjs_db1: f64,
        var_qjs_db2: f64,
        var_qjs_db3: f64,
        var_qjs_db4: f64,
        var_qjs_db5: f64,
        var_qjs_dn0: f64,
        var_qjs_dn1: f64,
        var_qjs_dn10: f64,
        var_qjs_dn11: f64,
        var_qjs_dn12: f64,
        var_qjs_dn13: f64,
        var_qjs_dn14: f64,
        var_qjs_dn2: f64,
        var_qjs_dn3: f64,
        var_qjs_dn4: f64,
        var_qjs_dn5: f64,
        var_qjs_dn6: f64,
        var_qjs_dn7: f64,
        var_qjs_dn8: f64,
        var_qjs_dn9: f64,
        var_qscp: f64,
        var_qscp_db0: f64,
        var_qscp_db1: f64,
        var_qscp_db2: f64,
        var_qscp_db3: f64,
        var_qscp_db4: f64,
        var_qscp_db5: f64,
        var_qscp_dn0: f64,
        var_qscp_dn1: f64,
        var_qscp_dn10: f64,
        var_qscp_dn11: f64,
        var_qscp_dn12: f64,
        var_qscp_dn13: f64,
        var_qscp_dn14: f64,
        var_qscp_dn2: f64,
        var_qscp_dn3: f64,
        var_qscp_dn4: f64,
        var_qscp_dn5: f64,
        var_qscp_dn6: f64,
        var_qscp_dn7: f64,
        var_qscp_dn8: f64,
        var_qscp_dn9: f64,
        var_rth_t: f64,
        var_rth_t_db0: f64,
        var_rth_t_db1: f64,
        var_rth_t_db2: f64,
        var_rth_t_db3: f64,
        var_rth_t_db4: f64,
        var_rth_t_db5: f64,
        var_rth_t_dn0: f64,
        var_rth_t_dn1: f64,
        var_rth_t_dn10: f64,
        var_rth_t_dn11: f64,
        var_rth_t_dn12: f64,
        var_rth_t_dn13: f64,
        var_rth_t_dn14: f64,
        var_rth_t_dn2: f64,
        var_rth_t_dn3: f64,
        var_rth_t_dn4: f64,
        var_rth_t_dn5: f64,
        var_rth_t_dn6: f64,
        var_rth_t_dn7: f64,
        var_rth_t_dn8: f64,
        var_rth_t_dn9: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq33_e343: f64 = (p.p148 * var_qjs);
        let eq33_e343_d_n0: f64 = (p.p148 * var_qjs_dn0);
        let eq33_e343_d_n1: f64 = (p.p148 * var_qjs_dn1);
        let eq33_e343_d_n2: f64 = (p.p148 * var_qjs_dn2);
        let eq33_e343_d_n3: f64 = (p.p148 * var_qjs_dn3);
        let eq33_e343_d_n4: f64 = (p.p148 * var_qjs_dn4);
        let eq33_e343_d_n5: f64 = (p.p148 * var_qjs_dn5);
        let eq33_e343_d_n6: f64 = (p.p148 * var_qjs_dn6);
        let eq33_e343_d_n7: f64 = (p.p148 * var_qjs_dn7);
        let eq33_e343_d_n8: f64 = (p.p148 * var_qjs_dn8);
        let eq33_e343_d_n9: f64 = (p.p148 * var_qjs_dn9);
        let eq33_e343_d_n10: f64 = (p.p148 * var_qjs_dn10);
        let eq33_e343_d_n11: f64 = (p.p148 * var_qjs_dn11);
        let eq33_e343_d_n12: f64 = (p.p148 * var_qjs_dn12);
        let eq33_e343_d_n13: f64 = (p.p148 * var_qjs_dn13);
        let eq33_e343_d_n14: f64 = (p.p148 * var_qjs_dn14);
        let eq33_e343_d_b0: f64 = (p.p148 * var_qjs_db0);
        let eq33_e343_d_b1: f64 = (p.p148 * var_qjs_db1);
        let eq33_e343_d_b2: f64 = (p.p148 * var_qjs_db2);
        let eq33_e343_d_b3: f64 = (p.p148 * var_qjs_db3);
        let eq33_e343_d_b4: f64 = (p.p148 * var_qjs_db4);
        let eq33_e343_d_b5: f64 = (p.p148 * var_qjs_db5);
        let eq33_e344: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq33_e343);
        let eq33_value: f64 = eq33_e344;
        let eq33_node_derivatives: [f64; 15] = [(eq33_e343_d_n0 * ddt_scale), (eq33_e343_d_n1 * ddt_scale), (eq33_e343_d_n2 * ddt_scale), (eq33_e343_d_n3 * ddt_scale), (eq33_e343_d_n4 * ddt_scale), (eq33_e343_d_n5 * ddt_scale), (eq33_e343_d_n6 * ddt_scale), (eq33_e343_d_n7 * ddt_scale), (eq33_e343_d_n8 * ddt_scale), (eq33_e343_d_n9 * ddt_scale), (eq33_e343_d_n10 * ddt_scale), (eq33_e343_d_n11 * ddt_scale), (eq33_e343_d_n12 * ddt_scale), (eq33_e343_d_n13 * ddt_scale), (eq33_e343_d_n14 * ddt_scale)];
        let eq33_branch_derivatives: [f64; 6] = [(eq33_e343_d_b0 * ddt_scale), (eq33_e343_d_b1 * ddt_scale), (eq33_e343_d_b2 * ddt_scale), (eq33_e343_d_b3 * ddt_scale), (eq33_e343_d_b4 * ddt_scale), (eq33_e343_d_b5 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let eq34_e347: f64 = (p.p148 * var_qscp);
        let eq34_e347_d_n0: f64 = (p.p148 * var_qscp_dn0);
        let eq34_e347_d_n1: f64 = (p.p148 * var_qscp_dn1);
        let eq34_e347_d_n2: f64 = (p.p148 * var_qscp_dn2);
        let eq34_e347_d_n3: f64 = (p.p148 * var_qscp_dn3);
        let eq34_e347_d_n4: f64 = (p.p148 * var_qscp_dn4);
        let eq34_e347_d_n5: f64 = (p.p148 * var_qscp_dn5);
        let eq34_e347_d_n6: f64 = (p.p148 * var_qscp_dn6);
        let eq34_e347_d_n7: f64 = (p.p148 * var_qscp_dn7);
        let eq34_e347_d_n8: f64 = (p.p148 * var_qscp_dn8);
        let eq34_e347_d_n9: f64 = (p.p148 * var_qscp_dn9);
        let eq34_e347_d_n10: f64 = (p.p148 * var_qscp_dn10);
        let eq34_e347_d_n11: f64 = (p.p148 * var_qscp_dn11);
        let eq34_e347_d_n12: f64 = (p.p148 * var_qscp_dn12);
        let eq34_e347_d_n13: f64 = (p.p148 * var_qscp_dn13);
        let eq34_e347_d_n14: f64 = (p.p148 * var_qscp_dn14);
        let eq34_e347_d_b0: f64 = (p.p148 * var_qscp_db0);
        let eq34_e347_d_b1: f64 = (p.p148 * var_qscp_db1);
        let eq34_e347_d_b2: f64 = (p.p148 * var_qscp_db2);
        let eq34_e347_d_b3: f64 = (p.p148 * var_qscp_db3);
        let eq34_e347_d_b4: f64 = (p.p148 * var_qscp_db4);
        let eq34_e347_d_b5: f64 = (p.p148 * var_qscp_db5);
        let eq34_e348: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq34_e347);
        let eq34_value: f64 = eq34_e348;
        let eq34_node_derivatives: [f64; 15] = [(eq34_e347_d_n0 * ddt_scale), (eq34_e347_d_n1 * ddt_scale), (eq34_e347_d_n2 * ddt_scale), (eq34_e347_d_n3 * ddt_scale), (eq34_e347_d_n4 * ddt_scale), (eq34_e347_d_n5 * ddt_scale), (eq34_e347_d_n6 * ddt_scale), (eq34_e347_d_n7 * ddt_scale), (eq34_e347_d_n8 * ddt_scale), (eq34_e347_d_n9 * ddt_scale), (eq34_e347_d_n10 * ddt_scale), (eq34_e347_d_n11 * ddt_scale), (eq34_e347_d_n12 * ddt_scale), (eq34_e347_d_n13 * ddt_scale), (eq34_e347_d_n14 * ddt_scale)];
        let eq34_branch_derivatives: [f64; 6] = [(eq34_e347_d_b0 * ddt_scale), (eq34_e347_d_b1 * ddt_scale), (eq34_e347_d_b2 * ddt_scale), (eq34_e347_d_b3 * ddt_scale), (eq34_e347_d_b4 * ddt_scale), (eq34_e347_d_b5 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(0),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
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
        let (eq38_e376, eq38_e376_d_n0, eq38_e376_d_n1, eq38_e376_d_n2, eq38_e376_d_n3, eq38_e376_d_n4, eq38_e376_d_n5, eq38_e376_d_n6, eq38_e376_d_n7, eq38_e376_d_n8, eq38_e376_d_n9, eq38_e376_d_n10, eq38_e376_d_n11, eq38_e376_d_n12, eq38_e376_d_n13, eq38_e376_d_n14, eq38_e376_d_b0, eq38_e376_d_b1, eq38_e376_d_b2, eq38_e376_d_b3, eq38_e376_d_b4, eq38_e376_d_b5,) = {
    if (var_guard244 != 0.0) {
        let eq38_e372: f64 = ((nv4 - 0.0) / var_rth_t);
        let eq38_e372_d_n0: f64 = (-(((nv4 - 0.0) * var_rth_t_dn0) / (var_rth_t * var_rth_t)));
        let eq38_e372_d_n1: f64 = (-(((nv4 - 0.0) * var_rth_t_dn1) / (var_rth_t * var_rth_t)));
        let eq38_e372_d_n2: f64 = (-(((nv4 - 0.0) * var_rth_t_dn2) / (var_rth_t * var_rth_t)));
        let eq38_e372_d_n3: f64 = (-(((nv4 - 0.0) * var_rth_t_dn3) / (var_rth_t * var_rth_t)));
        let eq38_e372_d_n4: f64 = ((var_rth_t - ((nv4 - 0.0) * var_rth_t_dn4)) / (var_rth_t * var_rth_t));
        let eq38_e372_d_n5: f64 = (-(((nv4 - 0.0) * var_rth_t_dn5) / (var_rth_t * var_rth_t)));
        let eq38_e372_d_n6: f64 = (-(((nv4 - 0.0) * var_rth_t_dn6) / (var_rth_t * var_rth_t)));
        let eq38_e372_d_n7: f64 = (-(((nv4 - 0.0) * var_rth_t_dn7) / (var_rth_t * var_rth_t)));
        let eq38_e372_d_n8: f64 = (-(((nv4 - 0.0) * var_rth_t_dn8) / (var_rth_t * var_rth_t)));
        let eq38_e372_d_n9: f64 = (-(((nv4 - 0.0) * var_rth_t_dn9) / (var_rth_t * var_rth_t)));
        let eq38_e372_d_n10: f64 = (-(((nv4 - 0.0) * var_rth_t_dn10) / (var_rth_t * var_rth_t)));
        let eq38_e372_d_n11: f64 = (-(((nv4 - 0.0) * var_rth_t_dn11) / (var_rth_t * var_rth_t)));
        let eq38_e372_d_n12: f64 = (-(((nv4 - 0.0) * var_rth_t_dn12) / (var_rth_t * var_rth_t)));
        let eq38_e372_d_n13: f64 = (-(((nv4 - 0.0) * var_rth_t_dn13) / (var_rth_t * var_rth_t)));
        let eq38_e372_d_n14: f64 = (-(((nv4 - 0.0) * var_rth_t_dn14) / (var_rth_t * var_rth_t)));
        let eq38_e372_d_b0: f64 = (-(((nv4 - 0.0) * var_rth_t_db0) / (var_rth_t * var_rth_t)));
        let eq38_e372_d_b1: f64 = (-(((nv4 - 0.0) * var_rth_t_db1) / (var_rth_t * var_rth_t)));
        let eq38_e372_d_b2: f64 = (-(((nv4 - 0.0) * var_rth_t_db2) / (var_rth_t * var_rth_t)));
        let eq38_e372_d_b3: f64 = (-(((nv4 - 0.0) * var_rth_t_db3) / (var_rth_t * var_rth_t)));
        let eq38_e372_d_b4: f64 = (-(((nv4 - 0.0) * var_rth_t_db4) / (var_rth_t * var_rth_t)));
        let eq38_e372_d_b5: f64 = (-(((nv4 - 0.0) * var_rth_t_db5) / (var_rth_t * var_rth_t)));
        let eq38_e374: f64 = (eq38_e372 - var_pterm);
        let eq38_e374_d_n0: f64 = (eq38_e372_d_n0 - var_pterm_dn0);
        let eq38_e374_d_n1: f64 = (eq38_e372_d_n1 - var_pterm_dn1);
        let eq38_e374_d_n2: f64 = (eq38_e372_d_n2 - var_pterm_dn2);
        let eq38_e374_d_n3: f64 = (eq38_e372_d_n3 - var_pterm_dn3);
        let eq38_e374_d_n4: f64 = (eq38_e372_d_n4 - var_pterm_dn4);
        let eq38_e374_d_n5: f64 = (eq38_e372_d_n5 - var_pterm_dn5);
        let eq38_e374_d_n6: f64 = (eq38_e372_d_n6 - var_pterm_dn6);
        let eq38_e374_d_n7: f64 = (eq38_e372_d_n7 - var_pterm_dn7);
        let eq38_e374_d_n8: f64 = (eq38_e372_d_n8 - var_pterm_dn8);
        let eq38_e374_d_n9: f64 = (eq38_e372_d_n9 - var_pterm_dn9);
        let eq38_e374_d_n10: f64 = (eq38_e372_d_n10 - var_pterm_dn10);
        let eq38_e374_d_n11: f64 = (eq38_e372_d_n11 - var_pterm_dn11);
        let eq38_e374_d_n12: f64 = (eq38_e372_d_n12 - var_pterm_dn12);
        let eq38_e374_d_n13: f64 = (eq38_e372_d_n13 - var_pterm_dn13);
        let eq38_e374_d_n14: f64 = (eq38_e372_d_n14 - var_pterm_dn14);
        let eq38_e374_d_b0: f64 = (eq38_e372_d_b0 - var_pterm_db0);
        let eq38_e374_d_b1: f64 = (eq38_e372_d_b1 - var_pterm_db1);
        let eq38_e374_d_b2: f64 = (eq38_e372_d_b2 - var_pterm_db2);
        let eq38_e374_d_b3: f64 = (eq38_e372_d_b3 - var_pterm_db3);
        let eq38_e374_d_b4: f64 = (eq38_e372_d_b4 - var_pterm_db4);
        let eq38_e374_d_b5: f64 = (eq38_e372_d_b5 - var_pterm_db5);
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
        let eq41_node_derivatives: [f64; 15] = [var_ixf1_dn0, var_ixf1_dn1, var_ixf1_dn2, var_ixf1_dn3, var_ixf1_dn4, var_ixf1_dn5, var_ixf1_dn6, var_ixf1_dn7, var_ixf1_dn8, var_ixf1_dn9, var_ixf1_dn10, var_ixf1_dn11, var_ixf1_dn12, var_ixf1_dn13, var_ixf1_dn14];
        let eq41_branch_derivatives: [f64; 6] = [var_ixf1_db0, var_ixf1_db1, var_ixf1_db2, var_ixf1_db3, var_ixf1_db4, var_ixf1_db5];
        stamper.stamp_current_dense_local(
            Some(10),
            None,
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let eq43_value: f64 = var_ixf2;
        let eq43_node_derivatives: [f64; 15] = [var_ixf2_dn0, var_ixf2_dn1, var_ixf2_dn2, var_ixf2_dn3, var_ixf2_dn4, var_ixf2_dn5, var_ixf2_dn6, var_ixf2_dn7, var_ixf2_dn8, var_ixf2_dn9, var_ixf2_dn10, var_ixf2_dn11, var_ixf2_dn12, var_ixf2_dn13, var_ixf2_dn14];
        let eq43_branch_derivatives: [f64; 6] = [var_ixf2_db0, var_ixf2_db1, var_ixf2_db2, var_ixf2_db3, var_ixf2_db4, var_ixf2_db5];
        stamper.stamp_current_dense_local(
            Some(11),
            None,
            multiplicity * (eq43_value),
            &eq43_node_derivatives,
            &eq43_branch_derivatives,
            multiplicity,
        );
        let eq45_value: f64 = var_ixf;
        let eq45_node_derivatives: [f64; 15] = [var_ixf_dn0, var_ixf_dn1, var_ixf_dn2, var_ixf_dn3, var_ixf_dn4, var_ixf_dn5, var_ixf_dn6, var_ixf_dn7, var_ixf_dn8, var_ixf_dn9, var_ixf_dn10, var_ixf_dn11, var_ixf_dn12, var_ixf_dn13, var_ixf_dn14];
        let eq45_branch_derivatives: [f64; 6] = [var_ixf_db0, var_ixf_db1, var_ixf_db2, var_ixf_db3, var_ixf_db4, var_ixf_db5];
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq45_value),
            &eq45_node_derivatives,
            &eq45_branch_derivatives,
            multiplicity,
        );
        let (eq65_e534, eq65_e534_d_n0, eq65_e534_d_n1, eq65_e534_d_n2, eq65_e534_d_n3, eq65_e534_d_n4, eq65_e534_d_n5, eq65_e534_d_n6, eq65_e534_d_n7, eq65_e534_d_n8, eq65_e534_d_n9, eq65_e534_d_n10, eq65_e534_d_n11, eq65_e534_d_n12, eq65_e534_d_n13, eq65_e534_d_n14, eq65_e534_d_b0, eq65_e534_d_b1, eq65_e534_d_b2, eq65_e534_d_b3, eq65_e534_d_b4, eq65_e534_d_b5,) = {
    if (var_guard258 != 0.0) {
        let __rspice_inv_cse_0: f64 = 1.0 / var_n_w;
        let eq65_e527: f64 = (var_n_2 * __rspice_inv_cse_0);
        let eq65_e527_d_n0: f64 = (var_n_2_dn0 * __rspice_inv_cse_0);
        let eq65_e527_d_n1: f64 = (var_n_2_dn1 * __rspice_inv_cse_0);
        let eq65_e527_d_n2: f64 = (var_n_2_dn2 * __rspice_inv_cse_0);
        let eq65_e527_d_n3: f64 = (var_n_2_dn3 * __rspice_inv_cse_0);
        let eq65_e527_d_n4: f64 = (var_n_2_dn4 * __rspice_inv_cse_0);
        let eq65_e527_d_n5: f64 = (var_n_2_dn5 * __rspice_inv_cse_0);
        let eq65_e527_d_n6: f64 = (var_n_2_dn6 * __rspice_inv_cse_0);
        let eq65_e527_d_n7: f64 = (var_n_2_dn7 * __rspice_inv_cse_0);
        let eq65_e527_d_n8: f64 = (var_n_2_dn8 * __rspice_inv_cse_0);
        let eq65_e527_d_n9: f64 = (var_n_2_dn9 * __rspice_inv_cse_0);
        let eq65_e527_d_n10: f64 = (var_n_2_dn10 * __rspice_inv_cse_0);
        let eq65_e527_d_n11: f64 = (var_n_2_dn11 * __rspice_inv_cse_0);
        let eq65_e527_d_n12: f64 = (var_n_2_dn12 * __rspice_inv_cse_0);
        let eq65_e527_d_n13: f64 = (var_n_2_dn13 * __rspice_inv_cse_0);
        let eq65_e527_d_n14: f64 = (var_n_2_dn14 * __rspice_inv_cse_0);
        let eq65_e527_d_b0: f64 = (var_n_2_db0 * __rspice_inv_cse_0);
        let eq65_e527_d_b1: f64 = (var_n_2_db1 * __rspice_inv_cse_0);
        let eq65_e527_d_b2: f64 = (var_n_2_db2 * __rspice_inv_cse_0);
        let eq65_e527_d_b3: f64 = (var_n_2_db3 * __rspice_inv_cse_0);
        let eq65_e527_d_b4: f64 = (var_n_2_db4 * __rspice_inv_cse_0);
        let eq65_e527_d_b5: f64 = (var_n_2_db5 * __rspice_inv_cse_0);
        let eq65_e530: f64 = (var_n_w * (nv13 - 0.0));
        let eq65_e531: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq65_e530);
        let eq65_e532: f64 = (eq65_e527 * eq65_e531);
        let eq65_e532_d_n0: f64 = (eq65_e527_d_n0 * eq65_e531);
        let eq65_e532_d_n1: f64 = (eq65_e527_d_n1 * eq65_e531);
        let eq65_e532_d_n2: f64 = (eq65_e527_d_n2 * eq65_e531);
        let eq65_e532_d_n3: f64 = (eq65_e527_d_n3 * eq65_e531);
        let eq65_e532_d_n4: f64 = (eq65_e527_d_n4 * eq65_e531);
        let eq65_e532_d_n5: f64 = (eq65_e527_d_n5 * eq65_e531);
        let eq65_e532_d_n6: f64 = (eq65_e527_d_n6 * eq65_e531);
        let eq65_e532_d_n7: f64 = (eq65_e527_d_n7 * eq65_e531);
        let eq65_e532_d_n8: f64 = (eq65_e527_d_n8 * eq65_e531);
        let eq65_e532_d_n9: f64 = (eq65_e527_d_n9 * eq65_e531);
        let eq65_e532_d_n10: f64 = (eq65_e527_d_n10 * eq65_e531);
        let eq65_e532_d_n11: f64 = (eq65_e527_d_n11 * eq65_e531);
        let eq65_e532_d_n12: f64 = (eq65_e527_d_n12 * eq65_e531);
        let eq65_e532_d_n13: f64 = ((eq65_e527_d_n13 * eq65_e531) + (eq65_e527 * (var_n_w * ddt_scale)));
        let eq65_e532_d_n14: f64 = (eq65_e527_d_n14 * eq65_e531);
        let eq65_e532_d_b0: f64 = (eq65_e527_d_b0 * eq65_e531);
        let eq65_e532_d_b1: f64 = (eq65_e527_d_b1 * eq65_e531);
        let eq65_e532_d_b2: f64 = (eq65_e527_d_b2 * eq65_e531);
        let eq65_e532_d_b3: f64 = (eq65_e527_d_b3 * eq65_e531);
        let eq65_e532_d_b4: f64 = (eq65_e527_d_b4 * eq65_e531);
        let eq65_e532_d_b5: f64 = (eq65_e527_d_b5 * eq65_e531);
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
        let (eq66_e545, eq66_e545_d_n0, eq66_e545_d_n1, eq66_e545_d_n2, eq66_e545_d_n3, eq66_e545_d_n4, eq66_e545_d_n5, eq66_e545_d_n6, eq66_e545_d_n7, eq66_e545_d_n8, eq66_e545_d_n9, eq66_e545_d_n10, eq66_e545_d_n11, eq66_e545_d_n12, eq66_e545_d_n13, eq66_e545_d_n14, eq66_e545_d_b0, eq66_e545_d_b1, eq66_e545_d_b2, eq66_e545_d_b3, eq66_e545_d_b4, eq66_e545_d_b5,) = {
    if (var_guard258 != 0.0) {
        let __rspice_inv_cse_1: f64 = 1.0 / var_n_w;
        let eq66_e538: f64 = (var_n_1 * __rspice_inv_cse_1);
        let eq66_e538_d_n0: f64 = (var_n_1_dn0 * __rspice_inv_cse_1);
        let eq66_e538_d_n1: f64 = (var_n_1_dn1 * __rspice_inv_cse_1);
        let eq66_e538_d_n2: f64 = (var_n_1_dn2 * __rspice_inv_cse_1);
        let eq66_e538_d_n3: f64 = (var_n_1_dn3 * __rspice_inv_cse_1);
        let eq66_e538_d_n4: f64 = (var_n_1_dn4 * __rspice_inv_cse_1);
        let eq66_e538_d_n5: f64 = (var_n_1_dn5 * __rspice_inv_cse_1);
        let eq66_e538_d_n6: f64 = (var_n_1_dn6 * __rspice_inv_cse_1);
        let eq66_e538_d_n7: f64 = (var_n_1_dn7 * __rspice_inv_cse_1);
        let eq66_e538_d_n8: f64 = (var_n_1_dn8 * __rspice_inv_cse_1);
        let eq66_e538_d_n9: f64 = (var_n_1_dn9 * __rspice_inv_cse_1);
        let eq66_e538_d_n10: f64 = (var_n_1_dn10 * __rspice_inv_cse_1);
        let eq66_e538_d_n11: f64 = (var_n_1_dn11 * __rspice_inv_cse_1);
        let eq66_e538_d_n12: f64 = (var_n_1_dn12 * __rspice_inv_cse_1);
        let eq66_e538_d_n13: f64 = (var_n_1_dn13 * __rspice_inv_cse_1);
        let eq66_e538_d_n14: f64 = (var_n_1_dn14 * __rspice_inv_cse_1);
        let eq66_e538_d_b0: f64 = (var_n_1_db0 * __rspice_inv_cse_1);
        let eq66_e538_d_b1: f64 = (var_n_1_db1 * __rspice_inv_cse_1);
        let eq66_e538_d_b2: f64 = (var_n_1_db2 * __rspice_inv_cse_1);
        let eq66_e538_d_b3: f64 = (var_n_1_db3 * __rspice_inv_cse_1);
        let eq66_e538_d_b4: f64 = (var_n_1_db4 * __rspice_inv_cse_1);
        let eq66_e538_d_b5: f64 = (var_n_1_db5 * __rspice_inv_cse_1);
        let eq66_e541: f64 = (var_n_w * (nv14 - 0.0));
        let eq66_e542: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, eq66_e541);
        let eq66_e543: f64 = (eq66_e538 * eq66_e542);
        let eq66_e543_d_n0: f64 = (eq66_e538_d_n0 * eq66_e542);
        let eq66_e543_d_n1: f64 = (eq66_e538_d_n1 * eq66_e542);
        let eq66_e543_d_n2: f64 = (eq66_e538_d_n2 * eq66_e542);
        let eq66_e543_d_n3: f64 = (eq66_e538_d_n3 * eq66_e542);
        let eq66_e543_d_n4: f64 = (eq66_e538_d_n4 * eq66_e542);
        let eq66_e543_d_n5: f64 = (eq66_e538_d_n5 * eq66_e542);
        let eq66_e543_d_n6: f64 = (eq66_e538_d_n6 * eq66_e542);
        let eq66_e543_d_n7: f64 = (eq66_e538_d_n7 * eq66_e542);
        let eq66_e543_d_n8: f64 = (eq66_e538_d_n8 * eq66_e542);
        let eq66_e543_d_n9: f64 = (eq66_e538_d_n9 * eq66_e542);
        let eq66_e543_d_n10: f64 = (eq66_e538_d_n10 * eq66_e542);
        let eq66_e543_d_n11: f64 = (eq66_e538_d_n11 * eq66_e542);
        let eq66_e543_d_n12: f64 = (eq66_e538_d_n12 * eq66_e542);
        let eq66_e543_d_n13: f64 = (eq66_e538_d_n13 * eq66_e542);
        let eq66_e543_d_n14: f64 = ((eq66_e538_d_n14 * eq66_e542) + (eq66_e538 * (var_n_w * ddt_scale)));
        let eq66_e543_d_b0: f64 = (eq66_e538_d_b0 * eq66_e542);
        let eq66_e543_d_b1: f64 = (eq66_e538_d_b1 * eq66_e542);
        let eq66_e543_d_b2: f64 = (eq66_e538_d_b2 * eq66_e542);
        let eq66_e543_d_b3: f64 = (eq66_e538_d_b3 * eq66_e542);
        let eq66_e543_d_b4: f64 = (eq66_e538_d_b4 * eq66_e542);
        let eq66_e543_d_b5: f64 = (eq66_e538_d_b5 * eq66_e542);
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
