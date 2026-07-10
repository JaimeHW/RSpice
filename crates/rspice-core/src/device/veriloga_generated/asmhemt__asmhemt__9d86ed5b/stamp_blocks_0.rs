#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_0(
        s: &mut Scratch,
    ) {
        s.store_scalar(192, 0.0);s.store_scalar(193, 0.0);s.store_scalar(194, 0.0);s.store_scalar(195, 0.0);s.store_scalar(196, 0.0);s.store_scalar(197, 0.0);s.store_scalar(186, 1.0);s.store_scalar(213, 0.0);s.store_scalar(214, 0.0);s.store_scalar(215, 0.0);s.store_scalar(216, 0.0);s.store_scalar(94, 0.0);s.store_scalar(209, 0.0);s.store_scalar(210, 0.0);s.store_scalar(211, 0.0);s.store_scalar(212, 0.0);s.store_scalar(185, 0.0);s.store_scalar(222, 0.0);s.store_scalar(223, 0.0);s.store_scalar(224, 0.0);s.store_scalar(225, 0.0);s.store_scalar(226, 0.0);s.store_scalar(227, 0.0);s.store_scalar(228, 0.0);s.store_scalar(229, 0.0);s.store_scalar(230, 0.0);s.store_scalar(231, 0.0);s.store_scalar(233, 0.0);s.store_scalar(234, 0.0);s.store_scalar(235, 0.0);s.store_scalar(236, 0.0);s.store_scalar(237, 0.0);s.store_scalar(238, 0.0);s.store_scalar(239, 0.0);s.store_scalar(240, 0.0);s.store_scalar(241, 0.0);s.store_scalar(242, 0.0);s.store_scalar(243, 0.0);s.store_scalar(245, 0.0);s.store_scalar(246, 0.0);s.store_scalar(247, 0.0);s.store_scalar(248, 0.0);s.store_scalar(249, 0.0);s.store_scalar(250, 0.0);s.store_scalar(251, 0.0);s.store_scalar(252, 0.0);s.store_scalar(253, 0.0);s.store_scalar(254, 0.0);s.store_scalar(255, 0.0);s.store_scalar(257, 0.0);s.store_scalar(258, 0.0);s.store_scalar(259, 0.0);s.store_scalar(260, 0.0);s.store_scalar(261, 0.0);s.store_scalar(262, 0.0);s.store_scalar(263, 0.0);s.store_scalar(264, 0.0);s.store_scalar(265, 0.0);s.store_scalar(266, 0.0);s.store_scalar(267, 0.0);s.store_scalar(269, 0.0);s.store_scalar(270, 0.0);s.store_scalar(271, 0.0);s.store_scalar(272, 0.0);s.store_scalar(273, 0.0);s.store_scalar(274, 0.0);s.store_scalar(275, 0.0);s.store_scalar(276, 0.0);s.store_scalar(277, 0.0);s.store_scalar(278, 0.0);s.store_scalar(279, 0.0);s.store_scalar(281, 0.0);s.store_scalar(282, 0.0);s.store_scalar(283, 0.0);s.store_scalar(284, 0.0);s.store_scalar(285, 0.0);s.store_scalar(286, 0.0);s.store_scalar(287, 0.0);s.store_scalar(288, 0.0);s.store_scalar(289, 0.0);s.store_scalar(290, 0.0);s.store_scalar(291, 0.0);s.store_scalar(293, 0.0);s.store_scalar(294, 0.0);s.store_scalar(295, 0.0);s.store_scalar(296, 0.0);s.store_scalar(297, 0.0);s.store_scalar(298, 0.0);s.store_scalar(299, 0.0);s.store_scalar(300, 0.0);s.store_scalar(301, 0.0);s.store_scalar(302, 0.0);s.store_scalar(303, 0.0);s.store_scalar(305, 0.0);s.store_scalar(306, 0.0);s.store_scalar(307, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();s.store_scalar(308, 0.0);s.store_scalar(309, 0.0);s.store_scalar(310, 0.0);s.store_scalar(311, 0.0);s.store_scalar(312, 0.0);s.store_scalar(313, 0.0);s.store_scalar(314, 0.0);s.store_scalar(315, 0.0);s.store_scalar(317, 0.0);s.store_scalar(206, 0.0);s.store_scalar(207, 0.0);s.store_scalar(182, 0.01);s.store_scalar(183, 0.01);s.store_scalar(144, 0.0);s.store_scalar(145, 0.0);s.store_scalar(142, 0.0);s.store_scalar(143, 0.0);s.store_scalar(48, 1.0);s.store_scalar(56, 1.0);s.store_scalar(64, 1.0);s.store_scalar(72, 1.0);s.store_scalar(52, 1.0);s.store_scalar(60, 1.0);s.store_scalar(68, 1.0);s.store_scalar(76, 1.0);s.store_scalar(321, 0.0);s.store_scalar(323, 0.0);s.store_scalar(322, 0.0);s.store_scalar(324, 0.0);s.store_scalar(325, 0.0);s.store_scalar(326, 0.0);s.store_scalar(327, 0.0);s.store_scalar(328, 1.0);s.store_scalar(329, 1.0);s.store_scalar(318, 1000.0);s.store_scalar(319, 1000.0);s.store_scalar(320, 1000.0);s.store_scalar(339, 0.0);s.store_scalar(344, 0.0);s.store_scalar(345, 0.0);s.store_scalar(341, 0.0);s.store_scalar(340, 0.0);s.store_scalar(346, 0.0);s.store_scalar(366, 0.0);s.store_scalar(365, 0.0);s.b[382] = (!true);s.store_scalar(382, if s.b[382] { 1.0 } else { 0.0 });s.b[383] = ((p.p31 == 0.0) || (p.p32 == 0.0));s.store_scalar(383, if s.b[383] { 1.0 } else { 0.0 });s.store_scalar(361, p.p34);s.b[384] = (p.p149 == 1.0);s.store_scalar(384, if s.b[384] { 1.0 } else { 0.0 });s.b[385] = (s.v[361] == 0.0);s.store_scalar(385, if s.b[385] { 1.0 } else { 0.0 });
        let (t0,) = {
    if (s.b[384] && s.b[385]) {
        (1.0,)
    } else {
        (s.v[361],)
    }
};
        s.store_scalar(361, t0);s.store_scalar(35, (p.p0 + 273.15));s.store_voltage(42, ctx, nodes, Some(7), Some(8));s.store_voltage(43, ctx, nodes, Some(9), Some(8));s.store_voltage(44, ctx, nodes, Some(9), Some(7));s.store_voltage(46, ctx, nodes, Some(3), Some(8));s.store_voltage(47, ctx, nodes, Some(3), Some(7));s.store_scalar(41, 1.0);s.b[386] = (s.v[42] < 0.0);s.store_scalar(386, if s.b[386] { 1.0 } else { 0.0 });
        if s.b[386] {s.store_scalar(41, (-1.0));s.store_mul(38, 41, 42);s.copy_ad(40, 44);s.copy_ad(45, 47);}
        if (!s.b[386]) {s.copy_ad(38, 42);s.copy_ad(40, 43);s.copy_ad(45, 46);}
        s.store_offset_sqrt_ad(140, A::offset(A::square(s.ad_value(38)), 0.01), (-0.1));s.store_offset_sqrt_ad(141, A::offset(A::square(A::voltage(ctx, nodes, Some(0), Some(2))), 0.01), (-0.1));s.store_offset_voltage(82, ctx, nodes, Some(4), None, ((ctx_temp) + (p.p274)));s.store_scale(36, 82, 8.617087e-5);s.b[387] = (p.p81 == 0.0);s.store_scalar(387, if s.b[387] { 1.0 } else { 0.0 });s.b[388] = (p.p81 == 1.0);s.store_scalar(388, if s.b[388] { 1.0 } else { 0.0 });s.b[389] = (p.p81 == 2.0);s.store_scalar(389, if s.b[389] { 1.0 } else { 0.0 });s.b[390] = (p.p81 == 3.0);s.store_scalar(390, if s.b[390] { 1.0 } else { 0.0 });s.b[391] = (p.p81 == 4.0);s.store_scalar(391, if s.b[391] { 1.0 } else { 0.0 });s.b[392] = (p.p81 == 5.0);s.store_scalar(392, if s.b[392] { 1.0 } else { 0.0 });
        if (s.b[388] && (!s.b[387])) {s.store_voltage(186, ctx, nodes, Some(5), None);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(186, 186, 0.5, 36, 0.5, 186, 36, ((0.25 * p.p128) * p.p128), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_2(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[388] && (!s.b[387])) {s.store_offset_scaled_ad(213, A::limited_exp(A::div_from_scalar((-1.0), s.ad_value(186))), p.p101, p.p100);s.store_offset_scaled_ad(214, A::limited_exp(A::div_from_scalar((-1.0), s.ad_value(186))), p.p105, p.p104);s.store_offset_scaled_ad(215, A::limited_exp(A::div_from_scalar((-1.0), s.ad_value(186))), p.p107, p.p106);s.store_offset_scaled_ad(216, A::limited_exp(A::div_from_scalar((-1.0), s.ad_value(186))), p.p103, p.p102);}
        if (s.b[389] && (!(s.b[387] || s.b[388]))) {s.store_limited_exp_scaled_voltage(208, ctx, nodes, Some(1), Some(2), (-p.p112));s.store_scaled_voltage(209, ctx, nodes, Some(6), None, p.p113);s.store_offset_add_scaled_inputs(210, A::voltage(ctx, nodes, Some(5), None), (-p.p116), A::voltage(ctx, nodes, Some(6), None), p.p117, p.p118);s.store_scaled_voltage(211, ctx, nodes, Some(6), None, p.p114);s.store_scaled_voltage(212, ctx, nodes, Some(6), None, p.p115);}
        if (s.b[390] && (!((s.b[387] || s.b[388]) || s.b[389]))) {s.store_voltage(147, ctx, nodes, Some(0), Some(1));s.store_mul_div_from_scalar_lhs_ad_mixed_ai(90, p.p124, A::scale_offset(s.ad_value(147), p.p123, 1.0), 147);s.store_scaled_offset(91, 147, (-p.p127), p.p125);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(148, 90, 0.5, 91, 0.5, 90, 91, ((0.25 * p.p128) * p.p128), 0.5);s.store_exp_scaled_input_ad(136, A::offset(A::voltage(ctx, nodes, Some(1), Some(2)), (-p.p10)), ((-2.0) * 1.0 / (p.p122)));s.store_offset_scaled_ad(149, A::div(A::sub_from_scalar(1.0, s.ad_value(136)), A::offset(s.ad_value(136), 1.0)), ((p.p120 - 1e-9) * 0.5), ((((p.p120 - 1e-9) * 0.5)) + (1e-9)));s.store_scaled_voltage(184, ctx, nodes, Some(5), None, 1.0 / (p.p121));s.store_mul_powf_mixed_ia(185, 184, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p126);}
        if (s.b[391] && (!(((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]))) {s.store_abs_voltage(136, ctx, nodes, Some(0), Some(2));s.store_div_from_scalar_ad(338, p.p82, A::scale_offset(A::exp_scaled_input(A::voltage(ctx, nodes, Some(11), Some(12)), 1.0 / (p.p86)), p.p85, 1.0));s.store_abs_voltage(90, ctx, nodes, Some(1), Some(2));s.store_div_from_scalar_ad(343, p.p84, A::scale_offset(A::exp_scaled_input(A::voltage(ctx, nodes, Some(13), Some(14)), 1.0 / (p.p88)), p.p87, 1.0));s.store_sub_voltage_abs_voltage(337, ctx, nodes, Some(12), None, Some(0), Some(2));s.store_scaled_add_mixed_ia(337, 337, A::sqrt_square_offset(s.ad_value(337), ((0.25 * 1e-30) * 1e-30)), 0.5);s.store_sub_voltage_abs_voltage(342, ctx, nodes, Some(14), None, Some(1), Some(2));s.store_scaled_add_mixed_ia(342, 342, A::sqrt_square_offset(s.ad_value(342), ((0.25 * 1e-30) * 1e-30)), 0.5);s.store_scale(136, 337, p.p89);s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));s.store_scaled_div(339, 136, 90, (((p.p91 * p.p10)) as f64).abs());s.store_scale(136, 342, p.p90);s.store_sqrt_square_offset(90, 342, (p.p90 * p.p90));s.store_scaled_div(344, 136, 90, (((p.p92 * p.p10)) as f64).abs());s.store_scale(136, 342, p.p90);s.store_sqrt_square_offset(90, 342, (p.p90 * p.p90));s.store_scaled_div(345, 136, 90, (((p.p93 * p.p13)) as f64).abs());s.store_scale(136, 342, p.p90);s.store_sqrt_square_offset(90, 342, (p.p90 * p.p90));s.store_scaled_div(346, 136, 90, (((p.p94 * p.p17)) as f64).abs());s.store_scale(136, 337, p.p89);s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));s.store_scaled_div(340, 136, 90, (((p.p95 * p.p36)) as f64).abs());s.store_scale(136, 337, p.p89);s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));s.store_scaled_div(341, 136, 90, (((p.p96 * p.p37)) as f64).abs());}
        if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {s.store_ln_ad(362, A::offset(A::exp(A::offset(A::add_scaled_inputs3(A::voltage(ctx, nodes, Some(1), Some(2)), p.p129, A::voltage(ctx, nodes, Some(1), Some(0)), p.p130, A::abs(A::voltage(ctx, nodes, Some(0), Some(2))), p.p131), p.p132)), p.p133));s.store_scaled_exp_ad(363, A::offset(A::div_from_scalar(p.p137, A::scale(s.ad_value(82), 8.617087e-5)), (-(p.p137 / (8.617087e-5 * s.v[35])))), p.p134);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_3(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {s.store_ln_ad(368, A::offset(A::exp(A::offset(A::add_scaled_inputs3(A::voltage(ctx, nodes, Some(1), Some(2)), p.p138, A::voltage(ctx, nodes, Some(1), Some(0)), p.p139, A::abs(A::voltage(ctx, nodes, Some(0), Some(2))), p.p140), p.p141)), p.p142));s.store_scaled_exp_ad(367, A::offset(A::div_from_scalar(p.p146, A::scale(s.ad_value(82), 8.617087e-5)), (-(p.p146 / (8.617087e-5 * s.v[35])))), p.p143);s.store_voltage(337, ctx, nodes, Some(5), None);s.store_voltage(364, ctx, nodes, Some(6), None);s.store_scale(136, 337, p.p89);s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));s.store_scaled_div(339, 136, 90, (((p.p91 * p.p10)) as f64).abs());s.store_scale(136, 337, p.p89);s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));s.store_scaled_div(340, 136, 90, (((p.p95 * p.p36)) as f64).abs());s.store_scale(136, 337, p.p89);s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));s.store_scaled_div(341, 136, 90, (((p.p96 * p.p37)) as f64).abs());s.store_scale(136, 364, p.p90);s.store_sqrt_square_offset(90, 364, (p.p90 * p.p90));s.store_scaled_div(344, 136, 90, (((p.p92 * p.p10)) as f64).abs());s.store_scale(136, 364, p.p90);s.store_sqrt_square_offset(90, 364, (p.p90 * p.p90));s.store_scaled_div(365, 136, 90, (((p.p147 * p.p36)) as f64).abs());s.store_scale(136, 364, p.p90);s.store_sqrt_square_offset(90, 364, (p.p90 * p.p90));s.store_scaled_div(366, 136, 90, (((p.p148 * p.p37)) as f64).abs());}
        s.store_scalar(80, (p.p9 / p.p1));s.store_scalar(81, (p.p9 / p.p2));s.store_offset_ad(146, A::mul_offset_lhs(s.ad_value(211), p.p27, s.ad_value(140)), (1.0 + p.p26));s.store_scaled_mul(83, 82, 146, 8.617087e-5);s.store_add_scaled_inputs3_offset_mixed_iia(87, 339, 1.0, 344, 1.0, A::div_scaled_product(A::sub(A::offset(s.ad_value(212), p.p22), s.ad_value(216)), s.ad_value(140), p.p23, A::sqrt_square_offset(s.ad_value(140), (p.p23 * p.p23)), 1.0), -1.0, p.p10);s.store_scale(334, 82, 1.0 / (s.v[35]));s.store_sub_from_scalar_ad(379, p.p266, A::scaled_offset(s.ad_value(334), (-1.0), p.p267));s.store_add_scaled_inputs_mixed_ai(88, A::add_scaled_inputs4_offset(s.ad_value(87), 1.0, s.ad_value(334), ((-1.0) * p.p24), s.ad_value(209), 1.0, s.ad_value(213), 1.0, ((-1.0) * ((-1.0) * p.p24))), 1.0, 45, ((s.v[81] / (s.v[81] + s.v[80])) * p.p11));s.store_div_scalar_by_product_indices(136, p.p3, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));s.store_add_scaled_product_mixed_iia(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p30), 1.0);s.store_add_scaled_inputs4_mixed_iiai(160, 40, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(40), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);s.store_sub(37, 160, 88);s.store_div_from_scalar_scaled_input(84, s.v[80], 83, (1.602176634e-19 * 3.24e17));s.store_div_from_scalar(150, 2.718281828459045, 84);s.store_div_from_scalar(151, 1.0, 84);s.store_scalar(99, (s.v[80] / 1.602176634e-19));s.store_scaled_add_mixed_ia(154, 37, A::sqrt_square_offset(s.ad_value(37), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let t1: A = A::powf(A::scale(s.ad_value(154), s.v[99]), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, t1, (-(p.p28 / 3.0)), A::add_scaled_offset_product_rhs(t1, ((2.0 * p.p28) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 37, 1.0, 83, 2.0);s.b[393] = (s.v[136] < 200.0);s.store_scalar(393, if s.b[393] { 1.0 } else { 0.0 });
        if s.b[393] {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product_mixed_iaa(153, 83, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), (2.0 * s.v[99]), A::add_scaled_inputs(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, A::limited_exp_div_scaled_inputs(s.ad_value(37), (-1.0), s.ad_value(83), 2.0), (s.v[99] / 3.24e17)), 1.0);}
        if (!s.b[393]) {s.store_div_scaled_product_add_scaled_denominator(153, 83, 136, ((2.0 * s.v[99]) * 1.0 / (1.0)), A::div_from_scalar(1.0, s.ad_value(152)), 1.0, A::limited_exp_div_scaled_inputs(s.ad_value(37), (-1.0), s.ad_value(83), 2.0), (s.v[99] / 3.24e17), 1.0);}
        s.store_sub_scaled_inputs(100, 37, 1.0, 153, 1.0 / (s.v[99]));s.b[394] = ((((s.v[100] - s.v[37])) as f64).abs() > 1e-19);s.store_scalar(394, if s.b[394] { 1.0 } else { 0.0 });
        if s.b[394] {s.store_sub(101, 37, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_scalar(136, ((s.v[99]) as f64).powf(0.6666666666666666));s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p.p28);s.store_scaled_mul(103, 136, 90, p.p29);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
        if s.b[394] {
            s.store_add_scaled_value_products_mixed_iiaia(106, 101, s.v[99], 83, {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), 83, {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(105))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }
        if s.b[394] {s.store_scaled_mul(107, 136, 91, p.p28);s.store_scaled_mul(108, 136, 91, p.p29);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_sub_ad(113, A::sub_from_scalar(((-1.0) * s.v[99]), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 37, 114);s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(137, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(116, 136, p.p28, 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p.p29, 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[394] {
            s.store_add_scaled_value_products_mixed_iiaia(120, 115, s.v[99], 83, {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), 83, {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(119))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }
        if s.b[394] {s.store_scaled_mul(121, 136, 137, p.p28);s.store_scaled_mul(122, 136, 137, p.p29);s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_sub_ad(127, A::sub_from_scalar(((-1.0) * s.v[99]), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));s.store_sub_div_rhs_indices(128, 114, 120, 127);s.copy_ad(129, 128);}
        if (!s.b[394]) {s.copy_ad(129, 100);}
        s.store_sub_from_scalar(347, p.p13, 345);s.store_sub_from_scalar(348, p.p17, 346);s.store_mul_powf_mixed_ia(97, 347, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20);s.store_mul_powf_mixed_ia(89, 348, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19);s.store_scaled_abs_ad(136, A::sub(s.ad_value(37), s.ad_value(129)), (s.v[80] / p.p9));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));s.store_div_mixed_ia(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);s.store_scaled_add_mixed_ia(90, 37, A::sqrt_square_offset(s.ad_value(37), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p3, 136, p.p3, 90, 1.0, 1.0);s.store_powf_ad(136, A::div(s.ad_value(38), s.ad_value(85)), p.p18);s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));s.store_mul(86, 38, 90);s.store_sub(39, 37, 86);s.copy_ad(130, 39);s.store_scaled_add_mixed_ia(131, 130, A::sqrt_square_offset(s.ad_value(130), ((4.0 * 0.3) * 0.3)), 0.5);s.copy_ad(154, 131);s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);let t2: A = A::powf(A::scale(s.ad_value(154), s.v[99]), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, t2, (-(p.p28 / 3.0)), A::add_scaled_offset_product_rhs(t2, ((2.0 * p.p28) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);s.b[395] = (s.v[136] < 200.0);s.store_scalar(395, if s.b[395] { 1.0 } else { 0.0 });
        if s.b[395] {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[395] {s.store_div_scaled_product_mixed_iaa(156, 83, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), (2.0 * s.v[99]), A::add_scaled_inputs(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), (s.v[99] / 3.24e17)), 1.0);}
        if (!s.b[395]) {s.store_div_scaled_product_add_scaled_denominator(156, 83, 136, ((2.0 * s.v[99]) * 1.0 / (1.0)), A::div_from_scalar(1.0, s.ad_value(152)), 1.0, A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), (s.v[99] / 3.24e17), 1.0);}
        s.store_sub_scaled_inputs(100, 130, 1.0, 156, 1.0 / (s.v[99]));s.b[396] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);s.store_scalar(396, if s.b[396] { 1.0 } else { 0.0 });
        if s.b[396] {s.store_sub(101, 130, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_scalar(136, ((s.v[99]) as f64).powf(0.6666666666666666));s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p.p28);s.store_scaled_mul(103, 136, 90, p.p29);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
        if s.b[396] {
            s.store_add_scaled_value_products_mixed_iiaia(106, 101, s.v[99], 83, {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), 83, {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(105))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }
        if s.b[396] {s.store_scaled_mul(107, 136, 91, p.p28);s.store_scaled_mul(108, 136, 91, p.p29);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_sub_ad(113, A::sub_from_scalar(((-1.0) * s.v[99]), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 130, 114);s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_mul_scaled_powf_rhs(116, 136, p.p28, 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p.p29, 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
        if s.b[396] {
            s.store_add_scaled_value_products_mixed_iiaia(120, 115, s.v[99], 83, {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), 83, {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(119))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }
        if s.b[396] {s.store_mul_scaled_powf_rhs(121, 136, p.p28, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(122, 136, p.p29, 115, (-0.3333333333333333));s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);s.store_sub_ad(127, A::sub_from_scalar(((-1.0) * s.v[99]), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[396] {s.store_sub_div_rhs_indices(128, 114, 120, 127);s.store_add(132, 128, 86);}
        if (!s.b[396]) {s.store_add(132, 100, 86);}
        s.store_scaled_add(133, 129, 132, 0.5);s.store_sub(134, 132, 129);s.store_mul_add_scaled_inputs3_offset_rhs_indices(135, 134, 37, 1.0, 133, (-1.0), 83, 1.0, 0.0);s.store_scaled_abs_ad(136, A::sub(s.ad_value(37), s.ad_value(133)), (s.v[80] / p.p9));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);s.store_scale(96, 95, (s.v[80] * (p.p4 * (p.p5 * 1.0 / (p.p3)))));s.store_mul_scale_offset_mixed_ia(98, 96, A::sub_scaled_inputs(s.ad_value(140), p.p21, s.ad_value(86), p.p21), 1.0, 1.0);s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(134), (p.p25 * p.p25), s.ad_value(134)), 1.0);s.store_div(93, 98, 92);s.store_mul(94, 93, 135);s.store_offset_scaled(333, 334, ((p.p271) * (p.p269)), (((((((-1.0)) * (p.p271))) + (1.0))) * (p.p269)));s.store_offset_scaled(335, 334, ((p.p272) * (p.p270)), (((((((-1.0)) * (p.p272))) + (1.0))) * (p.p270)));s.store_offset_scaled(336, 334, ((p.p273) * (p.p268)), (((((((-1.0)) * (p.p273))) + (1.0))) * (p.p268)));s.b[397] = (s.v[333] > 0.0);s.store_scalar(397, if s.b[397] { 1.0 } else { 0.0 });s.b[398] = ((s.v[141] - s.v[336]) > 0.0);s.store_scalar(398, if s.b[398] { 1.0 } else { 0.0 });
        if (s.b[397] && s.b[398]) {s.store_div_scaled_inputs2_by_product_indices(354, 141, 1.0, 336, (-1.0), 335, 36, 1.0);}
        s.b[399] = (s.v[354] > 80.0);s.store_scalar(399, if s.b[399] { 1.0 } else { 0.0 });
        if ((s.b[397] && s.b[398]) && s.b[399]) {s.store_offset(355, 354, (((-80.0)) + (1.0)));s.store_scalar(354, 80.0);}
        if ((s.b[397] && s.b[398]) && (!s.b[399])) {s.store_scalar(355, 1.0);}
        if (s.b[397] && s.b[398]) {s.store_mul_exp_rhs(355, 355, 354);s.store_mul_scale_offset_indices(332, 333, 355, 1.0, (-1.0));}
        if (s.b[397] && (!s.b[398])) {s.store_div_scaled_inputs2_by_product_indices(354, 141, 1.0, 336, (-1.0), 335, 36, 1.0);}
        s.b[400] = (s.v[354] > 80.0);s.store_scalar(400, if s.b[400] { 1.0 } else { 0.0 });
        if ((s.b[397] && (!s.b[398])) && s.b[400]) {s.store_offset(355, 354, (((-80.0)) + (1.0)));s.store_scalar(354, 80.0);}
        if ((s.b[397] && (!s.b[398])) && (!s.b[400])) {s.store_scalar(355, 1.0);}
        if (s.b[397] && (!s.b[398])) {s.store_mul_exp_rhs(355, 355, 354);s.store_mul_scale_offset_indices(332, 333, 355, 1.0, (-1.0));}
        if (!s.b[397]) {s.store_scalar(332, 0.0);}
        s.store_sub(90, 132, 129);s.store_add_scaled_inputs3_indices(91, 37, 1.0, 83, 1.0, 133, -1.0);s.store_add_scaled_inputs3_mixed_iia(137, 37, (((s.v[80] * p.p4) * p.p5) * p.p3), 133, ((-1.0) * (((s.v[80] * p.p4) * p.p5) * p.p3)), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), (((s.v[80] * p.p4) * p.p5) * p.p3));s.store_scale(188, 137, (1.0 / (p.p233) * 1e26));s.store_offset_powf_ad(189, s.ad_value(188), p.p232, 1.0);s.store_div_from_scalar(190, p.p231, 189);s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p1);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(161, 191, 37, ((p.p4 * p.p5) * p.p3), 133, (((-1.0)) * (((p.p4 * p.p5) * p.p3))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p3), 0.0);s.store_add_scaled_inputs3_indices(136, 37, 1.0, 83, 1.0, 133, -1.0);s.store_add_scaled_inputs(90, 129, 0.3333333333333333, 132, (2.0 * 0.3333333333333333));s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(134)), (1.0 / 12.0), 136, 1.0);s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(134)), 134, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);s.store_mul_add_scaled_inputs4_indices_rhs(165, 191, 37, (-(((p.p4 * p.p3) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p3) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p3) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p3) * p.p5) * 0.5)));s.store_sub_scaled_inputs(166, 161, (-1.0), 165, 1.0);s.b[401] = (s.v[41] < 0.0);s.store_scalar(401, if s.b[401] { 1.0 } else { 0.0 });
        if s.b[401] {s.copy_ad(90, 166);s.copy_ad(166, 165);s.copy_ad(165, 90);}
        s.b[402] = (p.p56 == 0.0);s.store_scalar(402, if s.b[402] { 1.0 } else { 0.0 });s.b[403] = (p.p56 == 1.0);s.store_scalar(403, if s.b[403] { 1.0 } else { 0.0 });s.b[404] = (p.p56 == 2.0);s.store_scalar(404, if s.b[404] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_8(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[405] = (p.p56 == 3.0);s.store_scalar(405, if s.b[405] { 1.0 } else { 0.0 });s.b[406] = (p.p56 == 4.0);s.store_scalar(406, if s.b[406] { 1.0 } else { 0.0 });
        if s.b[402] {s.store_scalar(206, 0.0);s.store_scalar(207, 0.0);}
        if (s.b[403] && (!s.b[402])) {s.store_div_scaled_inputs_mixed_ai(136, A::voltage(ctx, nodes, Some(9), Some(8)), 1.0, 82, (p.p57 * 8.617087e-5));s.store_offset_scaled(137, 82, ((1.0 / (s.v[35])) * (p.p71)), (((((-1.0)) * (p.p71))) + (p.p63)));s.store_scaled_mul_ad(206, A::abs(s.ad_value(137)), A::offset(A::limited_exp(s.ad_value(136)), (-1.0)), ((p.p4 * p.p3) * p.p5));s.store_div_scaled_inputs_mixed_ai(136, A::voltage(ctx, nodes, Some(9), Some(7)), 1.0, 82, (p.p60 * 8.617087e-5));s.store_offset_scaled(137, 82, ((1.0 / (s.v[35])) * (p.p72)), (((((-1.0)) * (p.p72))) + (p.p64)));s.store_scaled_mul_ad(207, A::abs(s.ad_value(137)), A::offset(A::limited_exp(s.ad_value(136)), (-1.0)), ((p.p4 * p.p3) * p.p5));}
        if (s.b[404] && (!(s.b[402] || s.b[403]))) {s.store_offset_scaled(326, 82, ((1.0 / (s.v[35])) * (p.p75)), (((((-1.0)) * (p.p75))) + (p.p67)));s.store_offset_scaled(328, 82, ((1.0 / (s.v[35])) * (p.p77)), (((((-1.0)) * (p.p77))) + (p.p57)));s.store_offset_scaled(330, 82, ((1.0 / (s.v[35])) * (p.p79)), (((((-1.0)) * (p.p79))) + (p.p61)));s.store_div_scaled_inputs2_mixed_aii(136, A::voltage(ctx, nodes, Some(9), Some(8)), 1.0, 326, (-1.0), 328, (8.617087e-5 * s.v[35]));s.store_scale_ad(137, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p71), p.p63);s.store_scaled_mul_ad(206, A::abs(s.ad_value(137)), A::offset(A::limited_exp(s.ad_value(136)), (-1.0)), ((p.p4 * p.p3) * p.p5));s.store_add_scaled_inputs3_sqrt_third_ad(321, A::voltage(ctx, nodes, Some(9), Some(8)), -1.0, A::voltage(ctx, nodes, Some(9), Some(8)), (-(-0.5)), A::offset(A::square(A::neg(A::voltage(ctx, nodes, Some(9), Some(8)))), 0.001), (-(-0.5)));s.store_scale(322, 321, 1.0 / (p.p1));s.store_offset_sqrt(136, 321, p.p69);s.store_div_scaled_inputs_indices(90, 136, 1.0, 330, (8.617087e-5 * s.v[35]));s.store_scale_ad(324, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p73), p.p65);s.store_mul_scale_offset_mixed_ia(206, 206, A::mul3(s.ad_value(322), s.ad_value(324), A::limited_exp(s.ad_value(90))), 1.0, 1.0);s.store_offset_scaled(327, 82, ((1.0 / (s.v[35])) * (p.p76)), (((((-1.0)) * (p.p76))) + (p.p68)));s.store_offset_scaled(329, 82, ((1.0 / (s.v[35])) * (p.p78)), (((((-1.0)) * (p.p78))) + (p.p60)));s.store_offset_scaled(331, 82, ((1.0 / (s.v[35])) * (p.p80)), (((((-1.0)) * (p.p80))) + (p.p62)));s.store_div_scaled_inputs2_mixed_aii(136, A::voltage(ctx, nodes, Some(9), Some(7)), 1.0, 327, (-1.0), 329, (8.617087e-5 * s.v[35]));s.store_scale_ad(137, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p72), p.p64);s.store_scaled_mul_ad(207, A::abs(s.ad_value(137)), A::offset(A::limited_exp(s.ad_value(136)), (-1.0)), ((p.p4 * p.p3) * p.p5));s.store_add_scaled_inputs3_sqrt_third_ad(323, A::voltage(ctx, nodes, Some(9), Some(7)), -1.0, A::voltage(ctx, nodes, Some(9), Some(7)), (-(-0.5)), A::offset(A::square(A::neg(A::voltage(ctx, nodes, Some(9), Some(7)))), 0.001), (-(-0.5)));s.store_scale(322, 323, 1.0 / (p.p1));s.store_offset_sqrt(136, 323, p.p70);s.store_div_scaled_inputs_indices(136, 136, 1.0, 331, (8.617087e-5 * s.v[35]));s.store_scale_ad(325, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p74), p.p66);s.store_mul_scale_offset_mixed_ia(207, 207, A::mul3(s.ad_value(322), s.ad_value(325), A::limited_exp(s.ad_value(136))), 1.0, 1.0);}
        if (s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) {s.store_offset_scaled(326, 82, ((1.0 / (s.v[35])) * (p.p75)), (((((-1.0)) * (p.p75))) + (p.p67)));s.store_offset_scaled(328, 82, ((1.0 / (s.v[35])) * (p.p77)), (((((-1.0)) * (p.p77))) + (p.p57)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_9(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);let nv9 = ctx.node_voltage(nodes[9]);
        if (s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) {s.store_offset_scaled(330, 82, ((1.0 / (s.v[35])) * (p.p79)), (((((-1.0)) * (p.p79))) + (p.p61)));s.store_scale_ad(324, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p73), p.p65);s.store_scale_ad(137, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p71), (((p.p4 * p.p3) * p.p5) * p.p63));}
        s.b[407] = (s.v[137] > 0.0);s.store_scalar(407, if s.b[407] { 1.0 } else { 0.0 });s.b[408] = ((nv9 - nv8) > 0.0);s.store_scalar(408, if s.b[408] { 1.0 } else { 0.0 });
        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[407]) && s.b[408]) {s.store_div_scaled_value_by_product_mixed_aii(354, A::powf(A::voltage(ctx, nodes, Some(9), Some(8)), p.p58), 1.0, 328, 36, 1.0);}
        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[407]) && (!s.b[408])) {s.store_div_voltage_by_ad(354, ctx, nodes, Some(9), Some(8), A::mul(s.ad_value(328), s.ad_value(36)));}
        s.b[409] = (s.v[354] > 80.0);s.store_scalar(409, if s.b[409] { 1.0 } else { 0.0 });
        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[407]) && s.b[409]) {s.store_offset(355, 354, (((-80.0)) + (1.0)));s.store_scalar(354, 80.0);}
        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[407]) && (!s.b[409])) {s.store_scalar(355, 1.0);}
        if ((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[407]) {s.store_mul_exp_rhs(355, 355, 354);s.store_mul_ad_product_rhs(206, 137, A::offset(s.ad_value(355), (-1.0)), A::exp_div_scaled_inputs(s.ad_value(326), -1.0, A::mul(s.ad_value(328), s.ad_value(36)), 1.0));s.store_add_scaled_inputs3_sqrt_third_ad(356, A::voltage(ctx, nodes, Some(9), Some(8)), -1.0, A::voltage(ctx, nodes, Some(9), Some(8)), (-(-0.5)), A::offset(A::square(A::neg(A::voltage(ctx, nodes, Some(9), Some(8)))), 0.001), (-(-0.5)));s.store_div_scaled_offset_numerator(357, A::sqrt(s.ad_value(356)), 1.0, p.p69, A::mul(s.ad_value(330), s.ad_value(36)), 1.0);}
        s.b[410] = (s.v[357] > 80.0);s.store_scalar(410, if s.b[410] { 1.0 } else { 0.0 });
        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[407]) && s.b[410]) {s.store_offset(358, 357, (((-80.0)) + (1.0)));s.store_scalar(357, 80.0);}
        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[407]) && (!s.b[410])) {s.store_scalar(358, 1.0);}
        if ((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[407]) {s.store_offset_mul_ad(358, A::mul3(s.ad_value(356), s.ad_value(324), s.ad_value(358)), A::exp(s.ad_value(357)), 1.0);s.store_mul(206, 206, 358);}
        if ((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && (!s.b[407])) {s.store_scalar(206, 0.0);}
        if (s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) {s.store_offset_scaled(327, 82, ((1.0 / (s.v[35])) * (p.p76)), (((((-1.0)) * (p.p76))) + (p.p68)));s.store_offset_scaled(329, 82, ((1.0 / (s.v[35])) * (p.p78)), (((((-1.0)) * (p.p78))) + (p.p60)));s.store_offset_scaled(331, 82, ((1.0 / (s.v[35])) * (p.p80)), (((((-1.0)) * (p.p80))) + (p.p62)));s.store_scale_ad(325, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p74), p.p66);s.store_scale_ad(137, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p72), (((p.p4 * p.p3) * p.p5) * p.p64));}
        s.b[411] = (s.v[137] > 0.0);s.store_scalar(411, if s.b[411] { 1.0 } else { 0.0 });s.b[412] = ((nv9 - nv7) > 0.0);s.store_scalar(412, if s.b[412] { 1.0 } else { 0.0 });
        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[411]) && s.b[412]) {s.store_div_scaled_value_by_product_mixed_aii(354, A::powf(A::voltage(ctx, nodes, Some(9), Some(7)), p.p59), 1.0, 329, 36, 1.0);}
        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[411]) && (!s.b[412])) {s.store_div_voltage_by_ad(354, ctx, nodes, Some(9), Some(7), A::mul(s.ad_value(329), s.ad_value(36)));}
        s.b[413] = (s.v[354] > 80.0);s.store_scalar(413, if s.b[413] { 1.0 } else { 0.0 });
        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[411]) && s.b[413]) {s.store_offset(355, 354, (((-80.0)) + (1.0)));s.store_scalar(354, 80.0);}
        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[411]) && (!s.b[413])) {s.store_scalar(355, 1.0);}
        if ((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[411]) {s.store_mul_exp_rhs(355, 355, 354);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_10(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);let nv9 = ctx.node_voltage(nodes[9]);
        if ((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[411]) {s.store_mul_ad_product_rhs(207, 137, A::offset(s.ad_value(355), (-1.0)), A::exp_div_scaled_inputs(s.ad_value(327), -1.0, A::mul(s.ad_value(329), s.ad_value(36)), 1.0));s.store_add_scaled_inputs3_sqrt_third_ad(356, A::voltage(ctx, nodes, Some(9), Some(7)), -1.0, A::voltage(ctx, nodes, Some(9), Some(7)), (-(-0.5)), A::offset(A::square(A::neg(A::voltage(ctx, nodes, Some(9), Some(7)))), 0.001), (-(-0.5)));s.store_div_scaled_offset_numerator(357, A::sqrt(s.ad_value(356)), 1.0, p.p70, A::mul(s.ad_value(331), s.ad_value(36)), 1.0);}
        s.b[414] = (s.v[357] > 80.0);s.store_scalar(414, if s.b[414] { 1.0 } else { 0.0 });
        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[411]) && s.b[414]) {s.store_offset(358, 357, (((-80.0)) + (1.0)));s.store_scalar(357, 80.0);}
        if (((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[411]) && (!s.b[414])) {s.store_scalar(358, 1.0);}
        if ((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && s.b[411]) {s.store_offset_mul_ad(358, A::mul3(s.ad_value(356), s.ad_value(325), s.ad_value(358)), A::exp(s.ad_value(357)), 1.0);s.store_mul(207, 207, 358);}
        if ((s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) && (!s.b[411])) {s.store_scalar(207, 0.0);}
        if (s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) {s.store_offset_scaled(326, 82, ((1.0 / (s.v[35])) * (p.p75)), (((((-1.0)) * (p.p75))) + (p.p67)));s.store_offset_scaled(328, 82, ((1.0 / (s.v[35])) * (p.p77)), (((((-1.0)) * (p.p77))) + (p.p57)));s.store_offset_scaled(330, 82, ((1.0 / (s.v[35])) * (p.p79)), (((((-1.0)) * (p.p79))) + (p.p61)));s.store_scale_ad(324, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p73), (((p.p4 * p.p3) * p.p5) * p.p65));s.store_scale_ad(137, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p71), (((p.p4 * p.p3) * p.p5) * p.p63));}
        s.b[415] = (s.v[137] > 0.0);s.store_scalar(415, if s.b[415] { 1.0 } else { 0.0 });s.b[416] = ((nv9 - nv8) > 0.0);s.store_scalar(416, if s.b[416] { 1.0 } else { 0.0 });
        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[415]) && s.b[416]) {s.store_div_scaled_value_by_product_mixed_aii(354, A::powf(A::voltage(ctx, nodes, Some(9), Some(8)), p.p58), 1.0, 328, 36, 1.0);}
        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[415]) && (!s.b[416])) {s.store_div_voltage_by_ad(354, ctx, nodes, Some(9), Some(8), A::mul(s.ad_value(328), s.ad_value(36)));}
        s.b[417] = (s.v[354] > 80.0);s.store_scalar(417, if s.b[417] { 1.0 } else { 0.0 });
        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[415]) && s.b[417]) {s.store_offset(355, 354, (((-80.0)) + (1.0)));s.store_scalar(354, 80.0);}
        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[415]) && (!s.b[417])) {s.store_scalar(355, 1.0);}
        if ((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[415]) {s.store_mul_exp_rhs(355, 355, 354);s.store_mul_ad_product_rhs(380, 137, A::offset(s.ad_value(355), (-1.0)), A::exp_div_scaled_inputs(s.ad_value(326), -1.0, A::mul(s.ad_value(328), s.ad_value(36)), 1.0));s.store_add_scaled_inputs3_sqrt_third_ad(356, A::voltage(ctx, nodes, Some(9), Some(8)), -1.0, A::voltage(ctx, nodes, Some(9), Some(8)), (-(-0.5)), A::square(A::neg(A::voltage(ctx, nodes, Some(9), Some(8)))), (-(-0.5)));s.store_div_scaled_offset_numerator(357, A::sqrt(s.ad_value(356)), 1.0, p.p69, A::mul(s.ad_value(330), s.ad_value(36)), 1.0);}
        s.b[418] = (s.v[357] > 80.0);s.store_scalar(418, if s.b[418] { 1.0 } else { 0.0 });
        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[415]) && s.b[418]) {s.store_offset(358, 357, (((-80.0)) + (1.0)));s.store_scalar(357, 80.0);}
        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[415]) && (!s.b[418])) {s.store_scalar(358, 1.0);}
        if ((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[415]) {s.store_mul_exp_rhs(358, 358, 357);s.store_mul_sub_mixed_iia(381, 324, 358, A::exp(A::div_from_scalar(p.p69, A::mul(s.ad_value(330), s.ad_value(36)))));s.store_sub(206, 380, 381);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_11(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);let nv9 = ctx.node_voltage(nodes[9]);
        if ((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && (!s.b[415])) {s.store_scalar(206, 0.0);}
        if (s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) {s.store_offset_scaled(327, 82, ((1.0 / (s.v[35])) * (p.p76)), (((((-1.0)) * (p.p76))) + (p.p68)));s.store_offset_scaled(329, 82, ((1.0 / (s.v[35])) * (p.p78)), (((((-1.0)) * (p.p78))) + (p.p60)));s.store_offset_scaled(331, 82, ((1.0 / (s.v[35])) * (p.p80)), (((((-1.0)) * (p.p80))) + (p.p62)));s.store_scale_ad(325, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p74), (((p.p4 * p.p3) * p.p5) * p.p66));s.store_scale_ad(137, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p72), (((p.p4 * p.p3) * p.p5) * p.p64));}
        s.b[419] = (s.v[137] > 0.0);s.store_scalar(419, if s.b[419] { 1.0 } else { 0.0 });s.b[420] = ((nv9 - nv7) > 0.0);s.store_scalar(420, if s.b[420] { 1.0 } else { 0.0 });
        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[419]) && s.b[420]) {s.store_div_scaled_value_by_product_mixed_aii(354, A::powf(A::voltage(ctx, nodes, Some(9), Some(7)), p.p59), 1.0, 329, 36, 1.0);}
        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[419]) && (!s.b[420])) {s.store_div_voltage_by_ad(354, ctx, nodes, Some(9), Some(7), A::mul(s.ad_value(329), s.ad_value(36)));}
        s.b[421] = (s.v[354] > 80.0);s.store_scalar(421, if s.b[421] { 1.0 } else { 0.0 });
        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[419]) && s.b[421]) {s.store_offset(355, 354, (((-80.0)) + (1.0)));s.store_scalar(354, 80.0);}
        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[419]) && (!s.b[421])) {s.store_scalar(355, 1.0);}
        if ((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[419]) {s.store_mul_exp_rhs(355, 355, 354);s.store_mul_ad_product_rhs(380, 137, A::offset(s.ad_value(355), (-1.0)), A::exp_div_scaled_inputs(s.ad_value(327), -1.0, A::mul(s.ad_value(329), s.ad_value(36)), 1.0));s.store_add_scaled_inputs3_sqrt_third_ad(356, A::voltage(ctx, nodes, Some(9), Some(7)), -1.0, A::voltage(ctx, nodes, Some(9), Some(7)), (-(-0.5)), A::square(A::neg(A::voltage(ctx, nodes, Some(9), Some(7)))), (-(-0.5)));s.store_div_scaled_offset_numerator(357, A::sqrt(s.ad_value(356)), 1.0, p.p70, A::mul(s.ad_value(331), s.ad_value(36)), 1.0);}
        s.b[422] = (s.v[357] > 80.0);s.store_scalar(422, if s.b[422] { 1.0 } else { 0.0 });
        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[419]) && s.b[422]) {s.store_offset(358, 357, (((-80.0)) + (1.0)));s.store_scalar(357, 80.0);}
        if (((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[419]) && (!s.b[422])) {s.store_scalar(358, 1.0);}
        if ((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && s.b[419]) {s.store_mul_exp_rhs(358, 358, 357);s.store_mul_sub_mixed_iia(381, 325, 358, A::exp(A::div_from_scalar(p.p70, A::mul(s.ad_value(331), s.ad_value(36)))));s.store_sub(207, 380, 381);}
        if ((s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) && (!s.b[419])) {s.store_scalar(207, 0.0);}
        s.b[423] = (p.p56 == 0.0);s.store_scalar(423, if s.b[423] { 1.0 } else { 0.0 });s.b[359] = param_given[45];s.store_scalar(359, if s.b[359] { 1.0 } else { 0.0 });s.b[360] = param_given[44];s.store_scalar(360, if s.b[360] { 1.0 } else { 0.0 });s.copy_ad(187, 154);s.b[424] = (s.v[361] == 1.0);s.store_scalar(424, if s.b[424] { 1.0 } else { 0.0 });
        if s.b[424] {s.store_add_scaled_inputs4_offset_indices(177, 82, ((-p.p36) * ((1.0 / (s.v[35])) * (p.p50))), 340, (-1.0), 365, -1.0, 45, ((p.p12 / 1.602176634e-19) * s.v[81]), (p.p36 + ((-p.p36) * (((-1.0)) * (p.p50)))));s.store_add_scaled_inputs3_offset_mixed_iia(177, 177, 1.0, 177, (-0.5), A::sqrt_square_offset(A::offset(s.ad_value(177), (-1.0)), 0.001), (-(-0.5)), (1.0 + (-0.5)));s.store_mul_scale_offset_rhs(172, 177, 187, ((p.p38) * (1.602176634e-19)), 1.602176634e-19);s.store_scaled_powf_ad(176, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p51, p.p35);s.store_scaled_mul(173, 172, 176, (p.p4 * p.p5));s.store_scaled_powf_ad(180, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p52, p.p40);s.store_div_scalar_by_product_indices(175, p.p46, 172, 180, (p.p4 * p.p5));}
        s.b[425] = s.b[359];s.store_scalar(425, if s.b[425] { 1.0 } else { 0.0 });
        if (s.b[424] && s.b[425]) {s.store_scalar(350, (1.0 + p.p45));s.store_mul_sqrt_lhs(351, 350, 94);s.store_div(352, 351, 173);s.store_scale(353, 352, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[424] && s.b[425]) {s.store_add_mixed_ia(350, 350, A::square(s.ad_value(352)));s.store_add_ad(350, A::sqrt(A::sub(s.ad_value(350), s.ad_value(353))), A::sqrt(A::add(s.ad_value(350), s.ad_value(353))));s.store_div_scaled_inputs_indices(349, 351, 2.0, 350, 1.0);s.store_sub_from_scalar_div_indices(91, 1.0, 349, 173);}
        if (s.b[424] && (!s.b[425])) {s.store_abs_ad(182, A::div(s.ad_value(94), s.ad_value(173)));s.store_scaled_offset_ad(183, A::sub(A::offset(s.ad_value(182), 0.9), A::sqrt_square_offset(A::offset(s.ad_value(182), (-0.9)), (0.1 * 0.1))), (-(0.9 - ((((0.9 * 0.9) + (0.1 * 0.1))) as f64).sqrt())), 0.5);s.store_powf(136, 183, p.p42);s.store_sub_from_scalar(90, 1.0, 136);s.store_powf(91, 90, (1.0 / p.p42));}
        if s.b[424] {s.store_div(170, 175, 91);s.store_offset_scaled(178, 82, ((((1.0 / (s.v[35])) * (p.p54))) * (p.p48)), (((((((-1.0)) * (p.p54))) + (1.0))) * (p.p48)));s.store_add_scaled_inputs3_indices(145, 178, 1.0 / ((p.p4 * p.p5)), 170, 1.0, 214, 1.0);s.store_add_scaled_inputs4_offset_indices(177, 82, ((-p.p37) * ((1.0 / (s.v[35])) * (p.p50))), 341, (-1.0), 366, -1.0, 45, ((p.p12 / 1.602176634e-19) * s.v[81]), (p.p37 + ((-p.p37) * (((-1.0)) * (p.p50)))));s.store_add_scaled_inputs3_offset_mixed_iia(177, 177, 1.0, 177, (-0.5), A::sqrt_square_offset(A::offset(s.ad_value(177), (-1.0)), 0.001), (-(-0.5)), (1.0 + (-0.5)));s.store_mul_scale_offset_rhs(172, 177, 187, ((p.p39) * (1.602176634e-19)), 1.602176634e-19);s.store_scaled_mul(173, 172, 176, (p.p4 * p.p5));s.store_scaled_powf_ad(181, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p53, p.p41);s.store_div_scalar_by_product_indices(174, p.p47, 172, 181, (p.p4 * p.p5));}
        s.b[426] = s.b[360];s.store_scalar(426, if s.b[426] { 1.0 } else { 0.0 });
        if (s.b[424] && s.b[426]) {s.store_scalar(350, (1.0 + p.p44));s.store_mul_sqrt_lhs(351, 350, 94);s.store_div(352, 351, 173);s.store_scale(353, 352, 2.0);s.store_add_mixed_ia(350, 350, A::square(s.ad_value(352)));s.store_add_ad(350, A::sqrt(A::sub(s.ad_value(350), s.ad_value(353))), A::sqrt(A::add(s.ad_value(350), s.ad_value(353))));s.store_div_scaled_inputs_indices(349, 351, 2.0, 350, 1.0);s.store_sub_from_scalar_div_indices(91, 1.0, 349, 173);}
        if (s.b[424] && (!s.b[426])) {s.store_abs_ad(182, A::div(s.ad_value(94), s.ad_value(173)));s.store_scaled_offset_ad(183, A::sub(A::offset(s.ad_value(182), 0.9), A::sqrt_square_offset(A::offset(s.ad_value(182), (-0.9)), (0.1 * 0.1))), (-(0.9 - ((((0.9 * 0.9) + (0.1 * 0.1))) as f64).sqrt())), 0.5);s.store_powf(136, 183, p.p43);s.store_sub_from_scalar(90, 1.0, 136);s.store_powf(91, 90, (1.0 / p.p43));}
        if s.b[424] {s.store_div(171, 174, 91);s.store_offset_scaled(179, 82, ((((1.0 / (s.v[35])) * (p.p55))) * (p.p49)), (((((((-1.0)) * (p.p55))) + (1.0))) * (p.p49)));s.store_add_mixed_ai(144, A::add_scaled_inputs4(s.ad_value(179), 1.0 / ((p.p4 * p.p5)), s.ad_value(171), 1.0, s.ad_value(185), 1.0, s.ad_value(210), 1.0), 215);s.store_div_from_scalar(142, 1.0, 144);s.store_div_from_scalar(143, 1.0, 145);}
        s.b[427] = (p.p149 == 0.0);s.store_scalar(427, if s.b[427] { 1.0 } else { 0.0 });s.b[428] = (p.p149 == 0.0);s.store_scalar(428, if s.b[428] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_13(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[429] = (p.p260 == 1.0);s.store_scalar(429, if s.b[429] { 1.0 } else { 0.0 });s.b[430] = (s.v[361] == 1.0);s.store_scalar(430, if s.b[430] { 1.0 } else { 0.0 });s.b[431] = (p.p149 == 0.0);s.store_scalar(431, if s.b[431] { 1.0 } else { 0.0 });s.b[432] = (p.p56 != 0.0);s.store_scalar(432, if s.b[432] { 1.0 } else { 0.0 });s.b[433] = (p.p149 == 0.0);s.store_scalar(433, if s.b[433] { 1.0 } else { 0.0 });s.b[434] = (p.p150 != 0.0);s.store_scalar(434, if s.b[434] { 1.0 } else { 0.0 });
        if (s.b[433] && s.b[434]) {s.store_voltage(49, ctx, nodes, Some(15), Some(7));}
        s.b[435] = (p.p150 == 1.0);s.store_scalar(435, if s.b[435] { 1.0 } else { 0.0 });
        if ((s.b[433] && s.b[434]) && s.b[435]) {s.store_voltage(50, ctx, nodes, Some(9), Some(7));s.store_voltage(51, ctx, nodes, Some(9), Some(15));}
        if ((s.b[433] && s.b[434]) && (!s.b[435])) {s.store_voltage(50, ctx, nodes, Some(2), Some(7));s.store_voltage(51, ctx, nodes, Some(2), Some(15));}
        if (s.b[433] && s.b[434]) {s.store_scalar(48, 1.0);}
        s.b[436] = (s.v[49] < 0.0);s.store_scalar(436, if s.b[436] { 1.0 } else { 0.0 });
        if ((s.b[433] && s.b[434]) && s.b[436]) {s.store_scalar(48, (-1.0));s.store_mul(231, 48, 49);s.copy_ad(230, 51);}
        if ((s.b[433] && s.b[434]) && (!s.b[436])) {s.copy_ad(231, 49);s.copy_ad(230, 50);}
        if (s.b[433] && s.b[434]) {s.store_offset_sqrt_ad(232, A::offset(A::square(s.ad_value(231)), 0.01), (-0.1));s.store_offset_scaled(146, 232, p.p166, (1.0 + p.p165));s.store_scaled_mul(83, 82, 146, 8.617087e-5);s.store_sub_ad(88, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p162)), (((((-1.0)) * (p.p162))) + (p.p159))), A::div_scaled_inputs(s.ad_value(232), (p.p168 * p.p167), A::sqrt_square_offset(s.ad_value(232), (p.p168 * p.p168)), 1.0));s.store_scalar(223, (p.p9 / p.p160));s.store_div_scalar_by_product_indices(136, p.p161, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));s.store_add_scaled_product_mixed_iia(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p158), 1.0);s.store_add_scaled_inputs4_mixed_iiai(160, 230, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(230), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);s.store_sub(222, 160, 88);s.store_div_scaled_inputs_indices(84, 223, 1.0, 83, (1.602176634e-19 * 3.24e17));s.store_div_from_scalar(150, 2.718281828459045, 84);s.store_div_from_scalar(151, 1.0, 84);s.store_primal_scale(99, 223, 6.241509074460763e18);s.store_scaled_add_mixed_ia(154, 222, A::sqrt_square_offset(s.ad_value(222), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);let t3: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, t3, (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(t3, ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 222, 1.0, 83, 2.0);}
        s.b[437] = (s.v[136] < 200.0);s.store_scalar(437, if s.b[437] { 1.0 } else { 0.0 });
        if ((s.b[433] && s.b[434]) && s.b[437]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[433] && s.b[434]) && s.b[437]) {s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(222), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if ((s.b[433] && s.b[434]) && (!s.b[437])) {s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(222), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
        if (s.b[433] && s.b[434]) {s.store_sub_div_rhs_indices(100, 222, 153, 99);}
        s.b[438] = ((((s.v[100] - s.v[222])) as f64).abs() > 1e-19);s.store_scalar(438, if s.b[438] { 1.0 } else { 0.0 });
        if ((s.b[433] && s.b[434]) && s.b[438]) {s.store_sub(101, 222, 100);s.store_scaled_add_mixed_ia(101, 101, A::sqrt_square_offset(s.ad_value(101), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(136, 99, 0.6666666666666666);s.store_powf(90, 101, 0.6666666666666666);s.store_powf(91, 101, (-0.3333333333333333));s.store_scaled_mul(102, 136, 90, p.p169);s.store_scaled_mul(103, 136, 90, p.p170);s.store_sub_div_same_denominator(104, 100, 102, 83);s.store_sub_div_same_denominator(105, 100, 103, 83);}
        if ((s.b[433] && s.b[434]) && s.b[438]) {
            s.store_add_scaled_products3_mixed_iiiaia(106, 99, 101, 1.0, 83, {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), 83, {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(105))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }
        if ((s.b[433] && s.b[434]) && s.b[438]) {s.store_scaled_mul(107, 136, 91, p.p169);s.store_scaled_mul(108, 136, 91, p.p170);s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(110, 104, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(112, 105, 1.0);s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);s.store_sub_div_rhs_indices(114, 100, 106, 113);s.store_sub(115, 222, 114);s.store_scaled_add_mixed_ia(115, 115, A::sqrt_square_offset(s.ad_value(115), ((4.0 * 1e-9) * 1e-9)), 0.5);s.store_powf(137, 115, (-0.3333333333333333));s.store_mul_scaled_powf_rhs(116, 136, p.p169, 115, 0.6666666666666666);s.store_mul_scaled_powf_rhs(117, 136, p.p170, 115, 0.6666666666666666);s.store_sub_div_same_denominator(118, 114, 116, 83);s.store_sub_div_same_denominator(119, 114, 117, 83);}
        if ((s.b[433] && s.b[434]) && s.b[438]) {
            s.store_add_scaled_products3_mixed_iiiaia(120, 99, 115, 1.0, 83, {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), 83, {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(119))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }
        if ((s.b[433] && s.b[434]) && s.b[438]) {s.store_scaled_mul(121, 136, 137, p.p169);s.store_scaled_mul(122, 136, 137, p.p170);s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(124, 118, 1.0);s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);s.store_offset_limited_exp(126, 119, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[433] && s.b[434]) && s.b[438]) {s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);s.store_sub_div_rhs_indices(128, 114, 120, 127);s.copy_ad(224, 128);}
        if ((s.b[433] && s.b[434]) && (!s.b[438])) {s.copy_ad(224, 100);}
        if (s.b[433] && s.b[434]) {s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p163);s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p164);s.store_mul_scaled_abs_ad_rhs(136, 223, 1.0 / (p.p9), A::sub(s.ad_value(222), s.ad_value(224)));s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(224)), (s.v[81] / p.p9));s.store_div_mixed_ia(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);s.store_scaled_add_mixed_ia(90, 222, A::sqrt_square_offset(s.ad_value(222), ((4.0 * 0.3) * 0.3)), 0.5);s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p161, 136, p.p161, 90, 1.0, 1.0);s.store_powf_ad(136, A::div(s.ad_value(231), s.ad_value(85)), p.p18);s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));s.store_mul(86, 231, 90);s.store_sub(39, 222, 86);s.copy_ad(130, 39);s.store_scaled_add_mixed_ia(131, 130, A::sqrt_square_offset(s.ad_value(130), ((4.0 * 0.3) * 0.3)), 0.5);s.copy_ad(154, 131);s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);let t4: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, t4, (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(t4, ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);}
        s.b[439] = (s.v[136] < 200.0);s.store_scalar(439, if s.b[439] { 1.0 } else { 0.0 });
        if ((s.b[433] && s.b[434]) && s.b[439]) {s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);}
    }
}
