#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();s.store_scaled_voltage(202, ctx, nodes, Some(8), Some(6), p[148]);s.store_scaled_voltage(203, ctx, nodes, Some(8), Some(5), p[148]);s.store_sub(204, 202, 203);s.store_scaled_voltage(205, ctx, nodes, Some(7), Some(6), p[148]);s.store_scaled_voltage(206, ctx, nodes, Some(7), Some(5), p[148]);s.store_scaled_voltage(207, ctx, nodes, Some(1), Some(5), p[148]);s.store_scaled_voltage(208, ctx, nodes, Some(9), Some(5), p[148]);s.store_scaled_voltage(209, ctx, nodes, Some(3), Some(0), p[148]);s.b[279] = (p[0] <= 310.0);s.store_scalar(279, if s.b[279] { 1.0 } else { 0.0 });
        if s.b[279] {s.store_scalar(0, 1.6021918e-19);s.store_scalar(1, 1.3806226e-23);}
        if (!s.b[279]) {s.store_scalar(0, 1.602176634e-19);s.store_scalar(1, 1.380649e-23);}
        s.store_scalar(233, 0.0);s.store_scalar(8, (p[146] + 273.15));s.store_scalar(9, ctx_temp);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_primal_div(2, 1, 0);s.store_primal_scale(3, 2, 300.0);s.store_primal_scale(6, 2, s.v[8]);s.store_primal_div_from_scalar(7, 1.0, 6);s.store_scalar(276, ((p[121] * s.v[8]) * ((s.v[8]) as f64).ln()));s.store_scalar(277, (p[122] * s.v[8]));s.store_scalar(56, (p[131] * s.v[8]));s.store_scalar(88, ((p[117] + s.v[276]) + s.v[277]));s.store_scalar(89, ((p[118] + s.v[276]) + s.v[277]));s.store_scalar(90, ((p[119] + s.v[276]) + s.v[277]));s.store_scalar(91, ((s.v[88] + s.v[89]) * 0.5));s.store_scalar(92, ((s.v[88] + s.v[90]) * 0.5));s.store_scalar(77, ((p[117] + p[118]) * 0.5));s.store_scalar(78, ((p[117] + p[119]) * 0.5));s.store_scalar(79, ((p[120] + p[119]) * 0.5));s.store_primal_sub_from_scalar_ad(76, 3.0, A::div_from_scalar(p[121], s.ad_value(2)));s.store_primal_offset(80, 76, ((1.0) + ((-p[130]))));s.store_primal_offset(81, 76, ((1.0) + ((-p[138]))));s.store_primal_offset(82, 76, (-1.5));s.store_scalar(278, ((1.0 - p[107]) * (p[52] + p[106])));s.b[280] = (s.v[278] >= p[106]);s.store_scalar(280, if s.b[280] { 1.0 } else { 0.0 });
        if s.b[280] {s.store_scalar(171, p[106]);s.store_scalar(172, 0.0);s.store_scalar(176, (s.v[278] - p[106]));s.store_primal_sub_from_scalar(177, p[52], 176);}
        if (!s.b[280]) {s.store_scalar(171, s.v[278]);s.store_primal_sub_from_scalar(172, p[106], 171);s.store_scalar(176, 0.0);s.store_scalar(177, p[52]);}
        s.store_scalar(174, (p[105] * p[104]));s.store_scalar(173, (p[104] - s.v[174]));s.b[281] = (p[22] != 0.0);s.store_scalar(281, if s.b[281] { 1.0 } else { 0.0 });
        if s.b[281] {s.store_scalar(175, (1.0 / p[22]));}
        if (!s.b[281]) {s.store_scalar(175, 0.0);}
        s.b[282] = (p[0] <= 300.0);s.store_scalar(282, if s.b[282] { 1.0 } else { 0.0 });
        let (t0,) = {
    if s.b[282] {
        (0.0,)
    } else {
        (s.v[223],)
    }
};
        s.store_scalar(223, t0);
        let (t1,) = {
    if (!s.b[282]) {
        (0.7,)
    } else {
        (s.v[223],)
    }
};
        s.store_scalar(223, t1);s.store_scalar(244, 0.0);s.b[283] = ((p[32] > 0.0) && (p[47] > 0.0));s.store_scalar(283, if s.b[283] { 1.0 } else { 0.0 });
        let (t2,) = {
    if s.b[283] {
        (1.0,)
    } else {
        (s.v[243],)
    }
};
        s.store_scalar(243, t2);
        let (t3,) = {
    if (!s.b[283]) {
        (0.0,)
    } else {
        (s.v[243],)
    }
};
        s.store_scalar(243, t3);s.store_scalar(234, p[86]);s.b[284] = (p[86] != 0.0);s.store_scalar(284, if s.b[284] { 1.0 } else { 0.0 });s.b[285] = (((p[88] == 0.0) && (p[87] == 0.0)) || (p[66] == 0.0));s.store_scalar(285, if s.b[285] { 1.0 } else { 0.0 });
        let (t4,) = {
    if (s.b[284] && s.b[285]) {
        (0.0,)
    } else {
        (s.v[234],)
    }
};
        s.store_scalar(234, t4);s.b[286] = ((p[115] >= 0.01) || (p[116] >= 0.01));s.store_scalar(286, if s.b[286] { 1.0 } else { 0.0 });
        if s.b[286] {s.store_scalar(232, (0.5 * (p[115] - p[116])));}
        s.b[287] = (p[116] < p[115]);s.store_scalar(287, if s.b[287] { 1.0 } else { 0.0 });
        let (t5,) = {
    if (s.b[286] && s.b[287]) {
        (p[116],)
    } else {
        (s.v[229],)
    }
};
        s.store_scalar(229, t5);
        if (s.b[286] && s.b[287]) {s.store_scalar(230, p[115]);}
        let (t6,) = {
    if (s.b[286] && (!s.b[287])) {
        (p[115],)
    } else {
        (s.v[229],)
    }
};
        s.store_scalar(229, t6);
        if (s.b[286] && (!s.b[287])) {s.store_scalar(230, p[116]);}
        s.b[288] = (s.v[229] < 0.01);s.store_scalar(288, if s.b[288] { 1.0 } else { 0.0 });
        if (s.b[286] && s.b[288]) {s.store_scalar(225, 1000000000.0);s.store_scalar(226, 1000000000.0);s.store_scalar(227, 170000000.0);s.store_scalar(228, 170000000.0);s.store_primal_ln_offset_input(231, 230, 1.0);}
        if (s.b[286] && (!s.b[288])) {s.store_scalar(225, (1.0 / p[115]));s.store_scalar(226, (1.0 / p[116]));s.store_scalar(227, (p[115] / 6.0));s.store_scalar(228, (p[116] / 6.0));s.store_scalar(231, ((((1.0 + p[115]) / (1.0 + p[116]))) as f64).ln());}
        if (!s.b[286]) {s.store_scalar(232, 0.0);s.store_scalar(225, 1000000000.0);s.store_scalar(226, 1000000000.0);s.store_scalar(227, 170000000.0);s.store_scalar(228, 170000000.0);}
        let (t7,) = {
    if (!s.b[286]) {
        (p[116],)
    } else {
        (s.v[229],)
    }
};
        s.store_scalar(229, t7);
        if (!s.b[286]) {s.store_scalar(230, p[115]);s.store_scalar(231, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(10, (s.v[9] + p[147]));s.b[289] = (s.v[10] < ((-200.0) + 273.15));s.store_scalar(289, if s.b[289] { 1.0 } else { 0.0 });
        if s.b[289] {s.store_scalar(10, ((-200.0) + 273.15));}
        s.b[290] = (s.v[10] > (326.85 + 273.15));s.store_scalar(290, if s.b[290] { 1.0 } else { 0.0 });
        if ((!s.b[289]) && s.b[290]) {s.store_scalar(10, (326.85 + 273.15));}
        s.store_mul(4, 2, 10);s.store_div_from_scalar(5, 1.0, 4);s.store_offset(14, 10, (-s.v[8]));s.store_div_from_scalar(12, s.v[8], 10);s.store_scale(11, 10, 1.0 / (s.v[8]));s.store_ln(13, 11);s.store_mul_scaled_ln_rhs(74, 10, p[121], 10);s.store_scale(75, 10, p[122]);s.store_add_offset_lhs(84, 74, p[117], 75);s.store_add_offset_lhs(83, 74, p[118], 75);s.store_add_offset_lhs(85, 74, p[119], 75);s.store_scaled_add(86, 84, 83, 0.5);s.store_scaled_add(87, 84, 85, 0.5);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[291] = (p[39] > 0.0);s.store_scalar(291, if s.b[291] { 1.0 } else { 0.0 });
        if s.b[291] {s.store_primal_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p[40] * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p[40]))));s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[77]), s.v[77]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));s.store_add_scaled_product_mixed_iia(27, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);s.store_scale_ad(26, A::exp_scaled_input(A::ln(A::div_from_scalar(p[40], s.ad_value(27))), p[41]), p[39]);s.store_scalar(28, ((p[42]) as f64).abs());}
        s.b[292] = (p[42] > 0.0);s.store_scalar(292, if s.b[292] { 1.0 } else { 0.0 });
        if (s.b[291] && s.b[292]) {s.store_scale(28, 27, (p[42] * 1.0 / (p[40])));}
        if (!s.b[291]) {s.store_scalar(26, p[39]);s.store_scalar(27, p[40]);s.store_scalar(28, p[42]);}
        s.store_scaled_exp_ad(22, A::add_scaled_inputs(s.ad_value(13), p[124], A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p[118]), 1.0), p[14]);s.store_scaled_exp_ad(24, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), (s.v[77] * 1.0 / (p[17]))), 1.0, s.ad_value(76), s.ad_value(13), 1.0 / (p[17])), p[16]);s.b[293] = (p[47] > 0.0);s.store_scalar(293, if s.b[293] { 1.0 } else { 0.0 });
        if s.b[293] {s.store_primal_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p[48] * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p[48]))));s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[78]), s.v[78]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));s.store_add_scaled_product_mixed_iia(34, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);s.store_scale_ad(33, A::exp_scaled_input(A::ln(A::div_from_scalar(p[48], s.ad_value(34))), p[49]), p[47]);s.store_scalar(35, ((p[50]) as f64).abs());}
        s.b[294] = (p[50] > 0.0);s.store_scalar(294, if s.b[294] { 1.0 } else { 0.0 });
        if (s.b[293] && s.b[294]) {s.store_scale(35, 34, (p[50] * 1.0 / (p[48])));}
        if (!s.b[293]) {s.store_scalar(33, p[47]);s.store_scalar(34, p[48]);s.store_scalar(35, p[50]);}
        s.b[295] = (p[0] <= 300.0);s.store_scalar(295, if s.b[295] { 1.0 } else { 0.0 });
        if s.b[295] {s.store_scalar(35, 2.4);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scaled_exp_ad(32, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p[119]), 1.0, s.ad_value(80), s.ad_value(13), 1.0), p[23]);s.store_offset_scaled_ad(16, A::exp_scaled_input(A::ln_scaled_input(s.ad_value(27), 1.0 / (p[40])), p[41]), (-p[2]), ((2.0) * (p[2])));s.store_scaled_exp_ad(15, A::add_scaled_inputs(s.ad_value(13), p[123], A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p[117]), 1.0), p[1]);s.store_scaled_exp_scaled_input(18, 13, p[126], p[10]);s.b[296] = ((p[0] <= 300.0) && ((((p[8] - 1.0)) as f64).abs() < 1e-5));s.store_scalar(296, if s.b[296] { 1.0 } else { 0.0 });
        if s.b[296] {s.store_scaled_exp_ad(17, A::mul_scaled_lhs(s.ad_value(5), p[125], A::offset(A::exp_scaled_input(s.ad_value(13), p[127]), (-1.0))), p[9]);}
        if (!s.b[296]) {s.store_scaled_exp_ad(17, A::mul_scaled_lhs(s.ad_value(5), p[125], A::offset(A::exp_scaled_input(s.ad_value(13), p[127]), (-1.0))), p[8]);}
        s.store_scaled_exp_ad(19, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p[125]), p[3]);s.store_scaled_exp_ad(20, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), (p[117] - p[118])), p[4]);s.store_scaled_exp_ad(21, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), (p[117] - p[119])), p[6]);s.store_scaled_exp_scaled_input(55, 13, (p[130] - s.v[56]), p[75]);s.store_scaled_exp_scaled_input(53, 13, p[130], p[74]);s.store_div_from_scalar(54, 1.0, 53);s.b[297] = (p[79] > 0.0);s.store_scalar(297, if s.b[297] { 1.0 } else { 0.0 });
        if s.b[297] {s.store_offset_scaled_ad(58, A::scale(s.ad_value(14), p[133]), (-p[79]), p[79]);s.store_scalar(57, p[78]);}
        if (!s.b[297]) {s.store_offset_scaled(57, 14, ((p[132]) * (p[78])), p[78]);s.store_scalar(58, p[79]);}
        s.store_add_scaled_product_mixed_aii(59, A::scale_offset(s.ad_value(14), p[128], 1.0), p[66], 14, 14, (p[129] * p[66]));s.store_scalar(61, p[69]);s.store_scaled_exp_scaled_input(60, 13, (p[130] - 1.0), p[71]);s.b[298] = (s.v[243] == 1.0);s.store_scalar(298, if s.b[298] { 1.0 } else { 0.0 });
        if s.b[298] {s.store_scaled_exp_scaled_input(63, 14, p[139], p[32]);s.store_scaled_exp_scaled_input(62, 14, p[140], p[33]);}
        if (!s.b[298]) {s.store_scalar(63, p[32]);s.store_scalar(62, p[33]);}
        s.b[299] = ((p[37] > 0.0) && (s.v[203] < 0.0));s.store_scalar(299, if s.b[299] { 1.0 } else { 0.0 });
        if s.b[299] {s.store_scalar(67, p[37]);s.store_scalar(68, p[38]);}
        s.b[300] = ((p[47] > 0.0) && (p[48] > 0.0));s.store_scalar(300, if s.b[300] { 1.0 } else { 0.0 });
        if (s.b[299] && s.b[300]) {s.store_div_from_scalar(169, s.v[92], 87);s.store_scale(170, 34, 1.0 / (p[48]));s.store_mul_ad_affine_product_lhs(168, A::sqrt(s.ad_value(169)), s.ad_value(170), 1.0 / (p[47]), 0.0, 33);s.store_scaled_mul(67, 168, 170, p[37]);s.store_div_scalar_by_product_indices(68, p[38], 168, 169, 1.0);}
        if (!s.b[299]) {s.store_scalar(67, 0.0);s.store_scalar(68, 1.0);}
        s.store_scaled_exp_scaled_input(69, 13, p[134], p[89]);s.b[301] = (p[43] > 0.0);s.store_scalar(301, if s.b[301] { 1.0 } else { 0.0 });
        if s.b[301] {s.store_primal_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p[44] * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p[44]))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[301] {s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[77]), s.v[77]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));s.store_add_scaled_product_mixed_iia(30, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);s.store_scale_ad(29, A::exp_scaled_input(A::ln(A::div_from_scalar(p[44], s.ad_value(30))), p[45]), p[43]);s.store_scalar(31, ((p[46]) as f64).abs());}
        s.b[302] = (p[46] > 0.0);s.store_scalar(302, if s.b[302] { 1.0 } else { 0.0 });
        if (s.b[301] && s.b[302]) {s.store_scale(31, 30, (p[46] * 1.0 / (p[44])));}
        if (!s.b[301]) {s.store_scalar(29, p[43]);s.store_scalar(30, p[44]);s.store_scalar(31, p[46]);}
        s.store_scaled_exp_ad(23, A::add_scaled_inputs(s.ad_value(13), p[124], A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p[118]), 1.0), p[18]);s.store_scaled_exp_ad(25, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), (s.v[77] * 1.0 / (p[21]))), 1.0, s.ad_value(76), s.ad_value(13), 1.0 / (p[21])), p[20]);s.b[303] = ((p[27] > 0.0) && ((s.v[205] < s.v[223]) || (s.v[202] < s.v[223])));s.store_scalar(303, if s.b[303] { 1.0 } else { 0.0 });
        if s.b[303] {s.store_scalar(166, 1.0);s.store_scalar(167, 1.0);s.store_div_from_scalar(169, s.v[91], 86);}
        s.b[304] = (((p[29] == 1.0) && (p[43] > 0.0)) && (p[44] > 0.0));s.store_scalar(304, if s.b[304] { 1.0 } else { 0.0 });
        if (s.b[303] && s.b[304]) {s.store_scale(170, 30, 1.0 / (p[44]));s.store_mul_product3_mixed_iiai(167, 170, 29, A::sqrt(s.ad_value(169)), 170, 1.0 / (p[43]));s.store_div_scaled_value_by_product_mixed_aii(166, A::powf(s.ad_value(169), (-1.5)), p[43], 29, 170, 1.0);}
        s.b[305] = (((p[29] == 0.0) && (p[39] > 0.0)) && (p[40] > 0.0));s.store_scalar(305, if s.b[305] { 1.0 } else { 0.0 });
        if ((s.b[303] && (!s.b[304])) && s.b[305]) {s.store_scale(170, 27, 1.0 / (p[40]));s.store_mul_product3_mixed_iiai(167, 170, 26, A::sqrt(s.ad_value(169)), 170, 1.0 / (p[39]));s.store_div_scaled_value_by_product_mixed_aii(166, A::powf(s.ad_value(169), (-1.5)), p[39], 26, 170, 1.0);}
        if s.b[303] {s.store_scale(64, 167, p[27]);s.store_scale(65, 166, p[28]);}
        if (!s.b[303]) {s.store_scalar(64, 0.0);s.store_scalar(65, 1.0);}
        s.store_scale_ad(66, A::exp_scaled_input(A::offset(s.ad_value(27), (-p[40])), (-1.0 / (p[31]))), p[30]);s.b[306] = (1.0 > 0.0);s.store_scalar(306, if s.b[306] { 1.0 } else { 0.0 });
        if s.b[306] {s.store_primal_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p[53] * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p[53]))));s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[78]), s.v[78]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[306] {s.store_add_scaled_product_mixed_iia(39, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);s.store_exp_scaled_input_ad(43, A::ln(A::div_from_scalar(p[53], s.ad_value(39))), p[54]);s.store_scalar(40, ((p[55]) as f64).abs());}
        s.b[307] = (p[55] > 0.0);s.store_scalar(307, if s.b[307] { 1.0 } else { 0.0 });
        if (s.b[306] && s.b[307]) {s.store_scale(40, 39, (p[55] * 1.0 / (p[53])));}
        if (!s.b[306]) {s.store_scalar(43, 1.0);s.store_scalar(39, p[53]);s.store_scalar(40, p[55]);}
        s.b[308] = (p[0] <= 300.0);s.store_scalar(308, if s.b[308] { 1.0 } else { 0.0 });
        if s.b[308] {s.store_scalar(40, 2.4);}
        s.store_mul(37, 43, 176);s.store_mul(38, 43, 177);s.store_scaled_exp_ad(36, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p[119]), 1.0, s.ad_value(81), s.ad_value(13), 1.0), p[25]);s.b[309] = (p[0] <= 300.0);s.store_scalar(309, if s.b[309] { 1.0 } else { 0.0 });s.b[310] = (p[57] > 0.0);s.store_scalar(310, if s.b[310] { 1.0 } else { 0.0 });
        if (s.b[309] && s.b[310]) {s.store_primal_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p[58] * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p[58]))));s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));s.store_add_scaled_product_mixed_iia(47, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);s.store_scale_ad(46, A::exp_scaled_input(A::ln(A::div_from_scalar(p[58], s.ad_value(47))), p[59]), p[57]);s.store_scalar(48, (((-2.4)) as f64).abs());}
        s.b[311] = ((-2.4) > 0.0);s.store_scalar(311, if s.b[311] { 1.0 } else { 0.0 });
        if ((s.b[309] && s.b[310]) && s.b[311]) {s.store_scale(48, 47, ((-2.4) * 1.0 / (p[58])));}
        if (s.b[309] && (!s.b[310])) {s.store_scalar(46, p[57]);s.store_scalar(47, p[58]);s.store_scalar(48, (-2.4));}
        if s.b[309] {s.store_scalar(163, 2.4);}
        s.b[312] = (p[57] > 0.0);s.store_scalar(312, if s.b[312] { 1.0 } else { 0.0 });
        if ((!s.b[309]) && s.b[312]) {s.store_primal_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p[58] * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p[58]))));s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_7(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((!s.b[309]) && s.b[312]) {s.store_add_scaled_product_mixed_iia(47, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);s.store_scale_ad(46, A::exp_scaled_input(A::ln(A::div_from_scalar(p[58], s.ad_value(47))), p[59]), p[57]);s.store_scalar(48, (((-p[60])) as f64).abs());}
        s.b[313] = ((-p[60]) > 0.0);s.store_scalar(313, if s.b[313] { 1.0 } else { 0.0 });
        if (((!s.b[309]) && s.b[312]) && s.b[313]) {s.store_scale(48, 47, ((-p[60]) * 1.0 / (p[58])));}
        if ((!s.b[309]) && (!s.b[312])) {s.store_scalar(46, p[57]);s.store_scalar(47, p[58]);s.store_scalar(48, (-p[60]));}
        if (!s.b[309]) {s.store_scalar(163, p[60]);}
        s.store_scaled_exp_ad(45, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p[120]), 1.0, s.ad_value(82), s.ad_value(13), 1.0), p[99]);s.store_scaled_exp_ad(44, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p[119]), 1.0, s.ad_value(82), s.ad_value(13), 1.0), p[97]);s.store_scaled_exp_scaled_input(52, 13, (p[138] - 1.0), p[101]);s.b[314] = (p[63] > 0.0);s.store_scalar(314, if s.b[314] { 1.0 } else { 0.0 });s.b[315] = (p[62] > 0.0);s.store_scalar(315, if s.b[315] { 1.0 } else { 0.0 });
        if (s.b[314] && s.b[315]) {s.store_primal_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p[63] * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p[63]))));s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));s.store_add_scaled_product_mixed_iia(50, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);s.store_scale_ad(49, A::exp_scaled_input(A::ln(A::div_from_scalar(p[63], s.ad_value(50))), p[64]), p[62]);s.store_abs_scaled_input(51, 163, -1.0);}
        s.b[316] = ((-s.v[163]) > 0.0);s.store_scalar(316, if s.b[316] { 1.0 } else { 0.0 });
        if ((s.b[314] && s.b[315]) && s.b[316]) {s.store_scaled_mul(51, 163, 50, (-1.0 / (p[63])));}
        if (s.b[314] && (!s.b[315])) {s.store_scalar(49, p[62]);s.store_scalar(50, p[63]);s.store_neg(51, 163);}
        if (!s.b[314]) {s.store_scalar(49, p[62]);s.store_scalar(50, p[63]);s.copy_ad(51, 163);}
        s.store_scaled_exp_scaled_input(72, 13, p[136], p[96]);s.store_scaled_exp_scaled_input(71, 13, p[135], p[90]);s.store_scaled_exp_scaled_input(73, 13, p[137], p[95]);s.store_mul_scale_offset_mixed_ai(201, A::exp_scaled_input(s.ad_value(13), p[143]), 14, (p[144]) * (p[142]), (1.0) * (p[142]));s.b[317] = (((p[141] != 0.0) && (p[142] >= p[149])) && (p[142] > 0.0));s.store_scalar(317, if s.b[317] { 1.0 } else { 0.0 });
        if s.b[317] {s.store_offset_voltage(10, ctx, nodes, Some(4), None, (s.v[9] + p[147]));}
        s.b[318] = (s.v[10] < ((-200.0) + 273.15));s.store_scalar(318, if s.b[318] { 1.0 } else { 0.0 });
        if (s.b[317] && s.b[318]) {s.store_scalar(10, ((-200.0) + 273.15));}
        s.b[319] = (s.v[10] > (326.85 + 273.15));s.store_scalar(319, if s.b[319] { 1.0 } else { 0.0 });
        if ((s.b[317] && (!s.b[318])) && s.b[319]) {s.store_scalar(10, (326.85 + 273.15));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[317] {s.store_mul(4, 2, 10);s.store_div_from_scalar(5, 1.0, 4);s.store_offset(14, 10, (-s.v[8]));s.store_div_from_scalar(12, s.v[8], 10);s.store_scale(11, 10, 1.0 / (s.v[8]));s.store_ln(13, 11);s.store_mul_scaled_ln_rhs(74, 10, p[121], 10);s.store_scale(75, 10, p[122]);s.store_add_offset_lhs(84, 74, p[117], 75);s.store_add_offset_lhs(83, 74, p[118], 75);s.store_add_offset_lhs(85, 74, p[119], 75);s.store_scaled_add(86, 84, 83, 0.5);s.store_scaled_add(87, 84, 85, 0.5);}
        s.b[320] = (p[39] > 0.0);s.store_scalar(320, if s.b[320] { 1.0 } else { 0.0 });
        if (s.b[317] && s.b[320]) {s.store_primal_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p[40] * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p[40]))));s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[77]), s.v[77]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));s.store_add_scaled_product_mixed_iia(27, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);s.store_scale_ad(26, A::exp_scaled_input(A::ln(A::div_from_scalar(p[40], s.ad_value(27))), p[41]), p[39]);s.store_scalar(28, ((p[42]) as f64).abs());}
        s.b[321] = (p[42] > 0.0);s.store_scalar(321, if s.b[321] { 1.0 } else { 0.0 });
        if ((s.b[317] && s.b[320]) && s.b[321]) {s.store_scale(28, 27, (p[42] * 1.0 / (p[40])));}
        if (s.b[317] && (!s.b[320])) {s.store_scalar(26, p[39]);s.store_scalar(27, p[40]);s.store_scalar(28, p[42]);}
        if s.b[317] {s.store_scaled_exp_ad(22, A::add_scaled_inputs(s.ad_value(13), p[124], A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p[118]), 1.0), p[14]);s.store_scaled_exp_ad(24, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), (s.v[77] * 1.0 / (p[17]))), 1.0, s.ad_value(76), s.ad_value(13), 1.0 / (p[17])), p[16]);}
        s.b[322] = (p[47] > 0.0);s.store_scalar(322, if s.b[322] { 1.0 } else { 0.0 });
        if (s.b[317] && s.b[322]) {s.store_primal_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p[48] * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p[48]))));s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[78]), s.v[78]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));s.store_add_scaled_product_mixed_iia(34, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[317] && s.b[322]) {s.store_scale_ad(33, A::exp_scaled_input(A::ln(A::div_from_scalar(p[48], s.ad_value(34))), p[49]), p[47]);s.store_scalar(35, ((p[50]) as f64).abs());}
        s.b[323] = (p[50] > 0.0);s.store_scalar(323, if s.b[323] { 1.0 } else { 0.0 });
        if ((s.b[317] && s.b[322]) && s.b[323]) {s.store_scale(35, 34, (p[50] * 1.0 / (p[48])));}
        if (s.b[317] && (!s.b[322])) {s.store_scalar(33, p[47]);s.store_scalar(34, p[48]);s.store_scalar(35, p[50]);}
        s.b[324] = (p[0] <= 300.0);s.store_scalar(324, if s.b[324] { 1.0 } else { 0.0 });
        if (s.b[317] && s.b[324]) {s.store_scalar(35, 2.4);}
        if s.b[317] {s.store_scaled_exp_ad(32, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p[119]), 1.0, s.ad_value(80), s.ad_value(13), 1.0), p[23]);s.store_offset_scaled_ad(16, A::exp_scaled_input(A::ln_scaled_input(s.ad_value(27), 1.0 / (p[40])), p[41]), (-p[2]), ((2.0) * (p[2])));s.store_scaled_exp_ad(15, A::add_scaled_inputs(s.ad_value(13), p[123], A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p[117]), 1.0), p[1]);s.store_scaled_exp_scaled_input(18, 13, p[126], p[10]);}
        s.b[325] = ((p[0] <= 300.0) && ((((p[8] - 1.0)) as f64).abs() < 1e-5));s.store_scalar(325, if s.b[325] { 1.0 } else { 0.0 });
        if (s.b[317] && s.b[325]) {s.store_scaled_exp_ad(17, A::mul_scaled_lhs(s.ad_value(5), p[125], A::offset(A::exp_scaled_input(s.ad_value(13), p[127]), (-1.0))), p[9]);}
        if (s.b[317] && (!s.b[325])) {s.store_scaled_exp_ad(17, A::mul_scaled_lhs(s.ad_value(5), p[125], A::offset(A::exp_scaled_input(s.ad_value(13), p[127]), (-1.0))), p[8]);}
        if s.b[317] {s.store_scaled_exp_ad(19, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p[125]), p[3]);s.store_scaled_exp_ad(20, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), (p[117] - p[118])), p[4]);s.store_scaled_exp_ad(21, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), (p[117] - p[119])), p[6]);s.store_scaled_exp_scaled_input(55, 13, (p[130] - s.v[56]), p[75]);s.store_scaled_exp_scaled_input(53, 13, p[130], p[74]);s.store_div_from_scalar(54, 1.0, 53);}
        s.b[326] = (p[79] > 0.0);s.store_scalar(326, if s.b[326] { 1.0 } else { 0.0 });
        if (s.b[317] && s.b[326]) {s.store_offset_scaled_ad(58, A::scale(s.ad_value(14), p[133]), (-p[79]), p[79]);s.store_scalar(57, p[78]);}
        if (s.b[317] && (!s.b[326])) {s.store_offset_scaled(57, 14, ((p[132]) * (p[78])), p[78]);s.store_scalar(58, p[79]);}
        if s.b[317] {s.store_add_scaled_product_mixed_aii(59, A::scale_offset(s.ad_value(14), p[128], 1.0), p[66], 14, 14, (p[129] * p[66]));s.store_scalar(61, p[69]);s.store_scaled_exp_scaled_input(60, 13, (p[130] - 1.0), p[71]);}
        s.b[327] = (s.v[243] == 1.0);s.store_scalar(327, if s.b[327] { 1.0 } else { 0.0 });
        if (s.b[317] && s.b[327]) {s.store_scaled_exp_scaled_input(63, 14, p[139], p[32]);s.store_scaled_exp_scaled_input(62, 14, p[140], p[33]);}
        if (s.b[317] && (!s.b[327])) {s.store_scalar(63, p[32]);s.store_scalar(62, p[33]);}
        s.b[328] = ((p[37] > 0.0) && (s.v[203] < 0.0));s.store_scalar(328, if s.b[328] { 1.0 } else { 0.0 });
        if (s.b[317] && s.b[328]) {s.store_scalar(67, p[37]);s.store_scalar(68, p[38]);}
        s.b[329] = ((p[47] > 0.0) && (p[48] > 0.0));s.store_scalar(329, if s.b[329] { 1.0 } else { 0.0 });
        if ((s.b[317] && s.b[328]) && s.b[329]) {s.store_div_from_scalar(169, s.v[92], 87);s.store_scale(170, 34, 1.0 / (p[48]));s.store_mul_ad_affine_product_lhs(168, A::sqrt(s.ad_value(169)), s.ad_value(170), 1.0 / (p[47]), 0.0, 33);s.store_scaled_mul(67, 168, 170, p[37]);s.store_div_scalar_by_product_indices(68, p[38], 168, 169, 1.0);}
        if (s.b[317] && (!s.b[328])) {s.store_scalar(67, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[317] && (!s.b[328])) {s.store_scalar(68, 1.0);}
        if s.b[317] {s.store_scaled_exp_scaled_input(69, 13, p[134], p[89]);}
        s.b[330] = (p[43] > 0.0);s.store_scalar(330, if s.b[330] { 1.0 } else { 0.0 });
        if (s.b[317] && s.b[330]) {s.store_primal_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p[44] * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p[44]))));s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[77]), s.v[77]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));s.store_add_scaled_product_mixed_iia(30, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);s.store_scale_ad(29, A::exp_scaled_input(A::ln(A::div_from_scalar(p[44], s.ad_value(30))), p[45]), p[43]);s.store_scalar(31, ((p[46]) as f64).abs());}
        s.b[331] = (p[46] > 0.0);s.store_scalar(331, if s.b[331] { 1.0 } else { 0.0 });
        if ((s.b[317] && s.b[330]) && s.b[331]) {s.store_scale(31, 30, (p[46] * 1.0 / (p[44])));}
        if (s.b[317] && (!s.b[330])) {s.store_scalar(29, p[43]);s.store_scalar(30, p[44]);s.store_scalar(31, p[46]);}
        if s.b[317] {s.store_scaled_exp_ad(23, A::add_scaled_inputs(s.ad_value(13), p[124], A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p[118]), 1.0), p[18]);s.store_scaled_exp_ad(25, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), (s.v[77] * 1.0 / (p[21]))), 1.0, s.ad_value(76), s.ad_value(13), 1.0 / (p[21])), p[20]);}
        s.b[332] = ((p[27] > 0.0) && ((s.v[205] < s.v[223]) || (s.v[202] < s.v[223])));s.store_scalar(332, if s.b[332] { 1.0 } else { 0.0 });
        if (s.b[317] && s.b[332]) {s.store_scalar(166, 1.0);s.store_scalar(167, 1.0);s.store_div_from_scalar(169, s.v[91], 86);}
        s.b[333] = (((p[29] == 1.0) && (p[43] > 0.0)) && (p[44] > 0.0));s.store_scalar(333, if s.b[333] { 1.0 } else { 0.0 });
        if ((s.b[317] && s.b[332]) && s.b[333]) {s.store_scale(170, 30, 1.0 / (p[44]));s.store_mul_product3_mixed_iiai(167, 170, 29, A::sqrt(s.ad_value(169)), 170, 1.0 / (p[43]));s.store_div_scaled_value_by_product_mixed_aii(166, A::powf(s.ad_value(169), (-1.5)), p[43], 29, 170, 1.0);}
        s.b[334] = (((p[29] == 0.0) && (p[39] > 0.0)) && (p[40] > 0.0));s.store_scalar(334, if s.b[334] { 1.0 } else { 0.0 });
        if (((s.b[317] && s.b[332]) && (!s.b[333])) && s.b[334]) {s.store_scale(170, 27, 1.0 / (p[40]));s.store_mul_product3_mixed_iiai(167, 170, 26, A::sqrt(s.ad_value(169)), 170, 1.0 / (p[39]));s.store_div_scaled_value_by_product_mixed_aii(166, A::powf(s.ad_value(169), (-1.5)), p[39], 26, 170, 1.0);}
        if (s.b[317] && s.b[332]) {s.store_scale(64, 167, p[27]);s.store_scale(65, 166, p[28]);}
        if (s.b[317] && (!s.b[332])) {s.store_scalar(64, 0.0);s.store_scalar(65, 1.0);}
        if s.b[317] {s.store_scale_ad(66, A::exp_scaled_input(A::offset(s.ad_value(27), (-p[40])), (-1.0 / (p[31]))), p[30]);}
        s.b[335] = (1.0 > 0.0);s.store_scalar(335, if s.b[335] { 1.0 } else { 0.0 });
        if (s.b[317] && s.b[335]) {s.store_primal_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p[53] * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p[53]))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[317] && s.b[335]) {s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[78]), s.v[78]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));s.store_add_scaled_product_mixed_iia(39, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);s.store_exp_scaled_input_ad(43, A::ln(A::div_from_scalar(p[53], s.ad_value(39))), p[54]);s.store_scalar(40, ((p[55]) as f64).abs());}
        s.b[336] = (p[55] > 0.0);s.store_scalar(336, if s.b[336] { 1.0 } else { 0.0 });
        if ((s.b[317] && s.b[335]) && s.b[336]) {s.store_scale(40, 39, (p[55] * 1.0 / (p[53])));}
        if (s.b[317] && (!s.b[335])) {s.store_scalar(43, 1.0);s.store_scalar(39, p[53]);s.store_scalar(40, p[55]);}
        s.b[337] = (p[0] <= 300.0);s.store_scalar(337, if s.b[337] { 1.0 } else { 0.0 });
        if (s.b[317] && s.b[337]) {s.store_scalar(40, 2.4);}
        if s.b[317] {s.store_mul(37, 43, 176);s.store_mul(38, 43, 177);s.store_scaled_exp_ad(36, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p[119]), 1.0, s.ad_value(81), s.ad_value(13), 1.0), p[25]);}
        s.b[338] = (p[0] <= 300.0);s.store_scalar(338, if s.b[338] { 1.0 } else { 0.0 });s.b[339] = (p[57] > 0.0);s.store_scalar(339, if s.b[339] { 1.0 } else { 0.0 });
        if ((s.b[317] && s.b[338]) && s.b[339]) {s.store_primal_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p[58] * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p[58]))));s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));s.store_add_scaled_product_mixed_iia(47, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);s.store_scale_ad(46, A::exp_scaled_input(A::ln(A::div_from_scalar(p[58], s.ad_value(47))), p[59]), p[57]);s.store_scalar(48, (((-2.4)) as f64).abs());}
        s.b[340] = ((-2.4) > 0.0);s.store_scalar(340, if s.b[340] { 1.0 } else { 0.0 });
        if (((s.b[317] && s.b[338]) && s.b[339]) && s.b[340]) {s.store_scale(48, 47, ((-2.4) * 1.0 / (p[58])));}
        if ((s.b[317] && s.b[338]) && (!s.b[339])) {s.store_scalar(46, p[57]);s.store_scalar(47, p[58]);s.store_scalar(48, (-2.4));}
        if (s.b[317] && s.b[338]) {s.store_scalar(163, 2.4);}
        s.b[341] = (p[57] > 0.0);s.store_scalar(341, if s.b[341] { 1.0 } else { 0.0 });
        if ((s.b[317] && (!s.b[338])) && s.b[341]) {s.store_primal_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p[58] * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p[58]))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[317] && (!s.b[338])) && s.b[341]) {s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));s.store_add_scaled_product_mixed_iia(47, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);s.store_scale_ad(46, A::exp_scaled_input(A::ln(A::div_from_scalar(p[58], s.ad_value(47))), p[59]), p[57]);s.store_scalar(48, (((-p[60])) as f64).abs());}
        s.b[342] = ((-p[60]) > 0.0);s.store_scalar(342, if s.b[342] { 1.0 } else { 0.0 });
        if (((s.b[317] && (!s.b[338])) && s.b[341]) && s.b[342]) {s.store_scale(48, 47, ((-p[60]) * 1.0 / (p[58])));}
        if ((s.b[317] && (!s.b[338])) && (!s.b[341])) {s.store_scalar(46, p[57]);s.store_scalar(47, p[58]);s.store_scalar(48, (-p[60]));}
        if (s.b[317] && (!s.b[338])) {s.store_scalar(163, p[60]);}
        if s.b[317] {s.store_scaled_exp_ad(45, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p[120]), 1.0, s.ad_value(82), s.ad_value(13), 1.0), p[99]);s.store_scaled_exp_ad(44, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(7), 1.0, s.ad_value(12), p[119]), 1.0, s.ad_value(82), s.ad_value(13), 1.0), p[97]);s.store_scaled_exp_scaled_input(52, 13, (p[138] - 1.0), p[101]);}
        s.b[343] = (p[63] > 0.0);s.store_scalar(343, if s.b[343] { 1.0 } else { 0.0 });s.b[344] = (p[62] > 0.0);s.store_scalar(344, if s.b[344] { 1.0 } else { 0.0 });
        if ((s.b[317] && s.b[343]) && s.b[344]) {s.store_primal_mul_scaled_ln_ad_rhs(164, 6, 2.0, A::sub(A::exp_scaled_input(s.ad_value(7), (p[63] * 0.5)), A::exp_scaled_input(s.ad_value(7), ((-0.5) * p[63]))));s.store_sub_ad(165, A::add_scaled_product(A::scale_offset(s.ad_value(11), (-s.v[79]), s.v[79]), 1.0, s.ad_value(164), s.ad_value(11), 1.0), A::mul3(s.ad_value(76), s.ad_value(4), s.ad_value(13)));s.store_add_scaled_product_mixed_iia(50, 165, 1.0, 4, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp(A::mul_scaled_lhs(s.ad_value(165), -1.0, s.ad_value(5))), 4.0, 1.0)), 1.0), 0.5), 2.0);s.store_scale_ad(49, A::exp_scaled_input(A::ln(A::div_from_scalar(p[63], s.ad_value(50))), p[64]), p[62]);s.store_abs_scaled_input(51, 163, -1.0);}
        s.b[345] = ((-s.v[163]) > 0.0);s.store_scalar(345, if s.b[345] { 1.0 } else { 0.0 });
        if (((s.b[317] && s.b[343]) && s.b[344]) && s.b[345]) {s.store_scaled_mul(51, 163, 50, (-1.0 / (p[63])));}
        if ((s.b[317] && s.b[343]) && (!s.b[344])) {s.store_scalar(49, p[62]);s.store_scalar(50, p[63]);s.store_neg(51, 163);}
        if (s.b[317] && (!s.b[343])) {s.store_scalar(49, p[62]);s.store_scalar(50, p[63]);s.copy_ad(51, 163);}
        if s.b[317] {s.store_scaled_exp_scaled_input(72, 13, p[136], p[96]);s.store_scaled_exp_scaled_input(71, 13, p[135], p[90]);s.store_scaled_exp_scaled_input(73, 13, p[137], p[95]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[317] {s.store_mul_scale_offset_mixed_ai(201, A::exp_scaled_input(s.ad_value(13), p[143]), 14, (p[144]) * (p[142]), (1.0) * (p[142]));}
        s.b[364] = (p[14] > 0.0);s.store_scalar(364, if s.b[364] { 1.0 } else { 0.0 });
        if s.b[364] {s.store_div_scaled_inputs_indices(93, 202, 1.0, 4, p[15]);}
        s.b[365] = (s.v[93] > 80.0);s.store_scalar(365, if s.b[365] { 1.0 } else { 0.0 });
        if (s.b[364] && s.b[365]) {s.store_offset(94, 93, (((-80.0)) + (1.0)));s.store_scalar(93, 80.0);}
        if (s.b[364] && (!s.b[365])) {s.store_scalar(94, 1.0);}
        if s.b[364] {s.store_mul_scale_offset_mixed_ia(185, 22, A::mul(s.ad_value(94), A::limexp(s.ad_value(93))), 1.0, (-1.0));}
        if (!s.b[364]) {s.store_scalar(185, 0.0);}
        s.b[366] = (p[16] > 0.0);s.store_scalar(366, if s.b[366] { 1.0 } else { 0.0 });
        if s.b[366] {s.store_div_scaled_inputs_indices(93, 202, 1.0, 4, p[17]);}
        s.b[367] = (s.v[93] > 80.0);s.store_scalar(367, if s.b[367] { 1.0 } else { 0.0 });
        if (s.b[366] && s.b[367]) {s.store_offset(94, 93, (((-80.0)) + (1.0)));s.store_scalar(93, 80.0);}
        if (s.b[366] && (!s.b[367])) {s.store_scalar(94, 1.0);}
        if s.b[366] {s.store_mul_scale_offset_mixed_ia(186, 24, A::mul(s.ad_value(94), A::limexp(s.ad_value(93))), 1.0, (-1.0));}
        if (!s.b[366]) {s.store_scalar(186, 0.0);}
        s.store_mul_mixed_ia(350, 15, A::limexp_scaled_input(A::mul(s.ad_value(202), s.ad_value(5)), 1.0 / (p[13])));s.store_mul_limexp_mixed_ia(351, 15, A::mul(s.ad_value(203), s.ad_value(5)));s.b[368] = (s.v[26] > 0.0);s.store_scalar(368, if s.b[368] { 1.0 } else { 0.0 });
        if s.b[368] {s.store_mul_scale_offset_mixed_ia(137, 27, A::exp_scaled_input(A::ln(s.ad_value(28)), (-1.0 / (p[41]))), -1.0, 1.0);s.store_mul_sub_lhs(141, 137, 202, 5);s.store_sqrt_square_offset(142, 141, 1.921812);s.store_scaled_add(143, 141, 142, 0.5);s.store_add_scaled_product_indices(138, 137, 1.0, 4, 143, (-1.0));s.store_div(144, 143, 142);s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(27))));s.store_mul_mixed_ai(145, A::exp_scaled_input(s.ad_value(139), (-p[41])), 144);s.store_mul_add_mixed_iia(211, 26, 145, A::mul_sub_from_scalar_rhs(s.ad_value(28), 1.0, s.ad_value(144)));s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(140, 27, 1.0, A::exp_scaled_input(s.ad_value(139), (1.0 - p[41])), 1.0, 1.0 / ((1.0 - p[41])));s.store_mul_add_scaled_product_rhs_mixed_iia(179, 26, 140, 1.0, 28, A::sub(s.ad_value(202), s.ad_value(138)), 1.0);}
        if (!s.b[368]) {s.store_scalar(211, 0.0);s.store_scalar(179, 0.0);}
        s.b[369] = (p[51] < 100.0);s.store_scalar(369, if s.b[369] { 1.0 } else { 0.0 });s.b[370] = (s.v[33] > 0.0);s.store_scalar(370, if s.b[370] { 1.0 } else { 0.0 });
        if (s.b[369] && s.b[370]) {s.store_scalar(113, (p[49] / 4.0));s.store_sub_from_scalar(114, p[51], 34);s.store_mul_scale_offset_mixed_ia(115, 34, A::exp_scaled_input(A::ln(s.ad_value(35)), (-1.0 / (p[49]))), -1.0, 1.0);s.store_mul(116, 35, 33);s.store_mul_exp_mixed_ia(117, 33, A::mul_offset_lhs(s.ad_value(113), (-p[49]), A::ln(A::div_from_scalar(p[51], s.ad_value(34)))));s.store_mul_sub_lhs(119, 115, 203, 5);}
        s.b[371] = (s.v[119] < 80.0);s.store_scalar(371, if s.b[371] { 1.0 } else { 0.0 });
        if ((s.b[369] && s.b[370]) && s.b[371]) {s.store_exp(120, 119);s.store_div_scaled_value_offset_denominator(121, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[369] && s.b[370]) && s.b[371]) {s.store_add_scaled_product_mixed_iia(122, 115, 1.0, 4, A::ln(A::offset(s.ad_value(120), 1.0)), (-1.0));}
        if ((s.b[369] && s.b[370]) && (!s.b[371])) {s.store_scalar(121, 1.0);s.copy_ad(122, 203);}
        if (s.b[369] && s.b[370]) {s.store_add_scaled_inputs(118, 114, 0.1, 4, 4.0);s.store_div_scaled_inputs2_indices(123, 114, 1.0, 122, 1.0, 118, 1.0);}
        s.b[372] = (s.v[123] < 80.0);s.store_scalar(372, if s.b[372] { 1.0 } else { 0.0 });
        if ((s.b[369] && s.b[370]) && s.b[372]) {s.store_exp(120, 123);s.store_div_scaled_value_offset_denominator(124, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);s.store_sub_mixed_ai(125, A::mul(s.ad_value(118), A::sub(A::ln(A::offset(s.ad_value(120), 1.0)), A::exp(A::div_scaled_inputs2(s.ad_value(114), -1.0, s.ad_value(115), -1.0, s.ad_value(118), 1.0)))), 114);}
        if ((s.b[369] && s.b[370]) && (!s.b[372])) {s.store_scalar(124, 1.0);s.copy_ad(125, 122);}
        if (s.b[369] && s.b[370]) {s.store_sub(126, 203, 122);s.store_ln_ad(130, A::sub_from_scalar(1.0, A::div(s.ad_value(122), s.ad_value(34))));s.store_ln_ad(131, A::sub_from_scalar(1.0, A::div(s.ad_value(125), s.ad_value(34))));s.store_scalar(132, (1.0 - p[49]));s.store_primal_sub_from_scalar(133, 1.0, 113);s.store_mul_product3_mixed_iiai(134, 124, 33, A::exp_scaled_input(s.ad_value(131), (-p[49])), 121, 1.0);s.store_mul_ad_product_rhs(135, 117, A::exp(A::mul_scaled_rhs(s.ad_value(130), s.ad_value(113), -1.0)), A::sub_from_scalar(1.0, s.ad_value(124)));s.store_mul_scale_offset_indices(136, 116, 121, -1.0, 1.0);s.store_add_scaled_inputs3_indices(210, 134, 1.0, 135, 1.0, 136, 1.0);s.store_div_mixed_ai(127, A::mul_sub_from_scalar_rhs(s.ad_value(33), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(132)))), 132);s.store_div_mixed_ai(128, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(130), s.ad_value(133)))), 133);s.store_div_mixed_ai(129, A::mul_sub_from_scalar_rhs(s.ad_value(117), 1.0, A::exp(A::mul(s.ad_value(131), s.ad_value(133)))), 133);s.store_add_scaled_products_mixed_aiii(178, A::add_scaled_inputs3(s.ad_value(127), 1.0, s.ad_value(128), 1.0, s.ad_value(129), -1.0), 34, 1.0, 116, 126, 1.0);}
        if (s.b[369] && (!s.b[370])) {s.store_scalar(210, 0.0);s.store_scalar(178, 0.0);}
        s.b[373] = (s.v[33] > 0.0);s.store_scalar(373, if s.b[373] { 1.0 } else { 0.0 });
        if ((!s.b[369]) && s.b[373]) {s.store_mul_scale_offset_mixed_ia(137, 34, A::exp_scaled_input(A::ln(s.ad_value(35)), (-1.0 / (p[49]))), -1.0, 1.0);s.store_mul_sub_lhs(141, 137, 203, 5);s.store_sqrt_square_offset(142, 141, 1.921812);s.store_scaled_add(143, 141, 142, 0.5);s.store_add_scaled_product_indices(138, 137, 1.0, 4, 143, (-1.0));s.store_div(144, 143, 142);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[369]) && s.b[373]) {s.store_ln_ad(139, A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(34))));s.store_mul_mixed_ai(145, A::exp_scaled_input(s.ad_value(139), (-p[49])), 144);s.store_mul_add_mixed_iia(210, 33, 145, A::mul_sub_from_scalar_rhs(s.ad_value(35), 1.0, s.ad_value(144)));s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(140, 34, 1.0, A::exp_scaled_input(s.ad_value(139), (1.0 - p[49])), 1.0, 1.0 / ((1.0 - p[49])));s.store_mul_add_scaled_product_rhs_mixed_iia(178, 33, 140, 1.0, 35, A::sub(s.ad_value(203), s.ad_value(138)), 1.0);}
        if ((!s.b[369]) && (!s.b[373])) {s.store_scalar(210, 0.0);s.store_scalar(178, 0.0);}
        s.b[374] = (p[10] > 0.0);s.store_scalar(374, if s.b[374] { 1.0 } else { 0.0 });
        if s.b[374] {s.store_scale(375, 4, p[11]);s.store_div_scaled_inputs2_indices(376, 27, 1.0, 202, (-1.0), 375, 1.0);s.store_add_scaled_product_mixed_iia(377, 27, 1.0, 375, A::add(s.ad_value(376), A::sqrt_square_offset(s.ad_value(376), 1.921812)), (-0.5));s.store_mul_scale_offset_mixed_ia(378, 18, A::exp_scaled_input(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(377), s.ad_value(27)))), p[41]), -1.0, 1.0);}
        s.b[379] = (((s.v[378]) as f64).abs() > 0.001);s.store_scalar(379, if s.b[379] { 1.0 } else { 0.0 });
        if (s.b[374] && s.b[379]) {s.store_div_scaled_product_offset_rhs_mixed_iai(346, 17, A::exp(s.ad_value(378)), (-1.0), 1.0, 378, 1.0);}
        if (s.b[374] && (!s.b[379])) {s.store_mul_scale_offset_rhs(346, 17, 378, 0.5, 1.0);}
        if (!s.b[374]) {s.copy_ad(346, 17);}
        s.store_add_scaled_inputs_mixed_ai(352, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(346), s.ad_value(179), 1.0), 1.0, 178, p[12]);s.store_scale(353, 16, 0.05);s.store_offset_div(347, 352, 353, (-1.0));s.store_mul_scale_offset_mixed_ia(352, 353, A::add_scaled_inputs(s.ad_value(347), 0.5, A::sqrt_square_offset(s.ad_value(347), 1.921812), 0.5), 1.0, 1.0);s.store_scale(380, 34, (1.0 - ((((-((2.4) as f64).ln()) / p[49])) as f64).exp()));s.store_mul_sub_lhs(381, 380, 203, 5);s.store_sqrt_square_offset(382, 381, 1.921812);s.store_scaled_add(383, 381, 382, 0.5);s.store_add_scaled_product_indices(384, 380, 1.0, 4, 383, (-1.0));s.store_div(385, 383, 382);s.store_add_scaled_product_mixed_aai(361, A::scale_offset(s.ad_value(385), (-2.4), 2.4), 1.0, A::exp_scaled_input(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(384), s.ad_value(34)))), (-p[49])), 385, 1.0);s.store_add_scaled_inputs3_offset_mixed_iai(357, 59, 1.0, A::div_from_scalar(1.0, s.ad_value(361)), p[67], 361, p[68], (((-1.0) * p[67]) + ((-1.0) * p[68])));s.b[386] = (p[79] > 0.0);s.store_scalar(386, if s.b[386] { 1.0 } else { 0.0 });
        if s.b[386] {s.store_sub(363, 58, 203);}
        if (!s.b[386]) {s.store_sub(363, 204, 57);}
        s.b[394] = (p[0] <= 300.0);s.store_scalar(394, if s.b[394] { 1.0 } else { 0.0 });
        if s.b[394] {s.store_mul_sub_lhs(387, 363, 4, 5);s.store_add_scaled_product_mixed_iia(388, 4, 1.0, 4, A::add(s.ad_value(387), A::sqrt_square_offset(s.ad_value(387), 1.921812)), 0.5);}
        if (!s.b[394]) {s.store_div(387, 363, 3);}
    }
}
