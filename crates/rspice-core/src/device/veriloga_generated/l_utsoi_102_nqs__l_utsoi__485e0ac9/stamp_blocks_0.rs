#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();s.store_scalar(7, (273.15 + p.p15));s.store_scalar(0, ((ctx_temp + p.p36)).min(1000.0));s.b[529] = (p.p10 == 1.0);s.store_scalar(529, if s.b[529] { 1.0 } else { 0.0 });
        if s.b[529] {s.store_scalar(8, (0.5 * ((s.v[0] + (p.p17 + (p.p18 * s.v[0]))) + (((((s.v[0] - (p.p17 + (p.p18 * s.v[0]))) * (s.v[0] - (p.p17 + (p.p18 * s.v[0])))) + p.p19)) as f64).sqrt())));s.store_scaled_add_offset_sqrt_square_offset_ad(225, A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), 600.0, (-600.0), 0.01, 0.5);}
        if (!s.b[529]) {s.store_scalar(8, (0.5 * ((s.v[0] + 1.0) + (((((s.v[0] - 1.0) * (s.v[0] - 1.0)) + 0.001)) as f64).sqrt())));s.store_scalar(225, 600.0);}
        s.b[530] = (((p.p0 == 0.0) && (p.p172 > 0.0)) || ((p.p0 > 0.0) && (p.p443 > 0.0)));s.store_scalar(530, if s.b[530] { 1.0 } else { 0.0 });
        let (t18,) = {
    if s.b[530] {
        (p.p5,)
    } else {
        (s.v[6],)
    }
};
        s.store_scalar(6, t18);
        let (t1e,) = {
    if (!s.b[530]) {
        (0.0,)
    } else {
        (s.v[6],)
    }
};
        s.store_scalar(6, t1e);s.store_scalar(475, 0.0);s.store_scalar(219, 0.0);s.copy_ad(217, 8);s.store_square(218, 217);s.store_offset(220, 217, (-s.v[7]));s.store_scale(221, 217, 1.0 / (s.v[7]));s.store_div_from_scalar(222, s.v[7], 217);s.store_scale(223, 217, 8.617332384961e-5);s.store_div_from_scalar(224, 1.0, 223);s.b[611] = (p.p0 == 0.0);s.store_scalar(611, if s.b[611] { 1.0 } else { 0.0 });
        if s.b[611] {s.store_scalar(10, p.p23);s.store_scalar(9, p.p22);s.store_scalar(12, p.p25);s.store_scalar(11, p.p24);s.store_scalar(13, p.p30);s.store_scalar(533, p.p41);s.store_scalar(14, p.p42);s.store_scalar(15, p.p43);s.store_scalar(534, p.p44);}
        let (t5,) = {
    if s.b[611] {
        (1.0,)
    } else {
        (s.v[535],)
    }
};
        s.store_scalar(535, t5);s.b[612] = (p.p45 < 0.0);s.store_scalar(612, if s.b[612] { 1.0 } else { 0.0 });
        let (t7,) = {
    if (s.b[611] && s.b[612]) {
        let t6: f64 = (-1.0);
        (t6,)
    } else {
        (s.v[535],)
    }
};
        s.store_scalar(535, t7);
        if s.b[611] {s.store_scalar(536, ((((p.p45) as f64).abs()).min(1e19) * 1000000.0));s.store_scalar(16, 1.0);}
        s.b[613] = (p.p46 < 0.0);s.store_scalar(613, if s.b[613] { 1.0 } else { 0.0 });
        if (s.b[611] && s.b[613]) {s.store_scalar(16, (-1.0));}
        if s.b[611] {s.store_scalar(537, (((((p.p46) as f64).abs()).max(1e16)).min(1e21) * 1000000.0));s.store_scalar(17, p.p47);s.store_scalar(18, p.p48);s.store_scalar(19, (p.p49 * 1000000.0));s.store_scalar(20, (p.p50 * 1000000.0));s.store_scalar(183, p.p51);s.store_scalar(184, p.p52);s.store_scalar(23, p.p53);s.store_scalar(24, (p.p54 * 1000000.0));s.store_scalar(25, p.p55);s.store_scalar(26, p.p56);s.store_scalar(27, p.p57);s.store_primal_div_scaled_product_indices(28, 27, 534, p.p58, 533, 1.0);s.store_scalar(29, (p.p59 * 1000000.0));s.store_scalar(30, p.p60);s.store_scalar(538, p.p61);s.store_scalar(187, p.p62);s.store_div_scaled_product_indices(188, 187, 534, p.p63, 533, 1.0);s.store_scalar(34, p.p64);s.store_scalar(35, p.p65);s.store_scalar(36, p.p66);s.store_scalar(37, p.p67);s.store_scalar(191, p.p68);s.store_scale(192, 191, p.p69);s.store_scalar(40, p.p70);s.store_scalar(195, p.p71);s.store_scalar(41, p.p72);s.store_scalar(42, p.p73);s.store_scalar(43, p.p74);s.store_scalar(196, p.p75);s.store_scalar(45, p.p76);s.store_scalar(539, p.p77);s.store_scalar(540, p.p78);s.store_scalar(193, p.p79);s.store_scalar(48, p.p80);s.store_scalar(194, p.p81);s.store_scalar(49, p.p82);s.store_scalar(197, p.p83);s.store_scalar(51, p.p84);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[611] {s.store_scalar(52, p.p85);s.store_scalar(541, p.p86);s.store_scalar(198, p.p87);s.store_scalar(54, p.p88);s.store_scalar(55, p.p89);s.store_scalar(56, p.p90);s.store_scalar(57, p.p91);s.store_scalar(58, p.p92);s.store_scalar(199, p.p93);s.store_scalar(60, p.p94);s.store_scalar(61, p.p95);s.store_scalar(62, p.p96);s.store_scalar(542, p.p97);s.store_scalar(63, p.p98);s.store_scalar(64, p.p99);s.store_scalar(65, p.p100);s.store_scalar(66, p.p101);s.store_scalar(67, p.p102);s.store_scalar(75, p.p103);s.store_scalar(201, p.p104);s.store_scalar(202, p.p105);s.store_scalar(203, p.p106);s.store_scalar(206, p.p120);s.store_scalar(207, p.p121);s.store_scalar(204, p.p107);s.store_scalar(205, p.p108);s.store_scalar(76, p.p109);s.store_scalar(77, p.p123);s.store_scalar(78, p.p110);s.store_scalar(79, p.p111);s.store_scalar(80, p.p112);s.store_scalar(81, p.p122);s.store_scalar(82, p.p113);s.store_scalar(83, p.p114);s.store_scalar(84, p.p115);s.store_scalar(85, p.p116);s.store_scalar(86, p.p117);s.store_scalar(87, p.p118);s.store_scalar(88, p.p119);s.store_scalar(89, p.p124);s.store_scalar(90, p.p125);s.store_scalar(208, p.p126);s.store_scalar(209, p.p127);s.store_scalar(93, p.p128);s.store_scalar(94, p.p129);s.store_scalar(95, p.p130);s.store_scalar(96, p.p131);s.store_scalar(97, p.p132);s.store_scalar(98, p.p133);s.store_scalar(112, p.p147);s.store_scalar(210, p.p148);s.store_scalar(114, p.p149);s.store_scalar(115, p.p150);s.store_scalar(99, p.p134);s.store_scalar(211, p.p135);s.store_scalar(212, p.p136);s.store_scalar(102, p.p137);s.store_scalar(103, p.p138);s.store_scalar(104, p.p139);s.store_scalar(105, p.p140);s.store_div_scaled_product_indices(106, 105, 534, p.p141, 533, 1.0);s.store_scalar(107, p.p142);s.store_div_scaled_product_indices(108, 107, 534, p.p143, 533, 1.0);s.store_scalar(109, p.p144);s.store_scalar(213, p.p145);s.store_scalar(111, p.p146);s.store_scalar(116, p.p151);s.store_scalar(117, p.p152);s.store_scalar(118, (p.p153 * 1000000.0));s.store_scalar(119, p.p154);s.store_scalar(120, p.p155);s.copy_ad(185, 183);s.copy_ad(186, 184);s.copy_ad(135, 27);s.copy_ad(136, 28);s.copy_ad(189, 187);s.copy_ad(190, 188);s.copy_ad(200, 199);s.copy_ad(543, 542);s.copy_ad(158, 63);}
        s.b[614] = (p.p11 > 0.0);s.store_scalar(614, if s.b[614] { 1.0 } else { 0.0 });
        if (s.b[611] && s.b[614]) {s.store_scalar(185, p.p51);}
        s.b[615] = param_given[156];s.store_scalar(615, if s.b[615] { 1.0 } else { 0.0 });
        if ((s.b[611] && s.b[614]) && s.b[615]) {s.store_scalar(185, p.p156);}
        if (s.b[611] && s.b[614]) {s.store_scalar(186, p.p52);}
        s.b[616] = param_given[157];s.store_scalar(616, if s.b[616] { 1.0 } else { 0.0 });
        if ((s.b[611] && s.b[614]) && s.b[616]) {s.store_scalar(186, p.p157);}
        if (s.b[611] && s.b[614]) {s.store_scalar(135, p.p57);}
        s.b[617] = param_given[158];s.store_scalar(617, if s.b[617] { 1.0 } else { 0.0 });
        if ((s.b[611] && s.b[614]) && s.b[617]) {s.store_scalar(135, p.p158);}
        if (s.b[611] && s.b[614]) {s.store_primal_div_scaled_product_indices(136, 135, 534, p.p58, 533, 1.0);s.store_scalar(189, p.p62);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[618] = param_given[159];s.store_scalar(618, if s.b[618] { 1.0 } else { 0.0 });
        if ((s.b[611] && s.b[614]) && s.b[618]) {s.store_scalar(189, p.p159);}
        if (s.b[611] && s.b[614]) {s.store_div_scaled_product_indices(190, 189, 534, p.p63, 533, 1.0);s.store_scalar(200, p.p93);}
        s.b[619] = param_given[160];s.store_scalar(619, if s.b[619] { 1.0 } else { 0.0 });
        if ((s.b[611] && s.b[614]) && s.b[619]) {s.store_scalar(200, p.p160);}
        if (s.b[611] && s.b[614]) {s.store_scalar(543, p.p97);}
        s.b[620] = param_given[161];s.store_scalar(620, if s.b[620] { 1.0 } else { 0.0 });
        if ((s.b[611] && s.b[614]) && s.b[620]) {s.store_scalar(543, p.p161);}
        if (s.b[611] && s.b[614]) {s.store_scalar(158, p.p98);}
        s.b[621] = param_given[162];s.store_scalar(621, if s.b[621] { 1.0 } else { 0.0 });
        if ((s.b[611] && s.b[614]) && s.b[621]) {s.store_scalar(158, p.p162);}
        if s.b[611] {s.store_scalar(159, p.p163);s.store_scalar(160, p.p164);s.store_scalar(161, p.p165);s.store_scalar(162, p.p166);s.store_scalar(163, p.p167);s.store_scalar(164, p.p168);s.store_scalar(165, p.p169);s.store_scalar(166, p.p170);s.store_scalar(167, p.p171);s.store_scalar(214, p.p172);s.store_scalar(169, p.p173);s.store_scalar(170, p.p174);}
        let (t0,) = {
    if s.b[611] {
        (p.p175,)
    } else {
        (s.v[171],)
    }
};
        s.store_scalar(171, t0);
        let (t1,) = {
    if s.b[611] {
        (p.p176,)
    } else {
        (s.v[172],)
    }
};
        s.store_scalar(172, t1);
        if s.b[611] {s.store_scalar(173, p.p177);s.store_scalar(174, p.p178);s.store_scalar(175, p.p179);s.store_scalar(176, p.p180);s.store_scalar(177, p.p181);s.store_scalar(179, p.p183);s.store_scalar(180, p.p184);s.store_scalar(181, p.p185);s.store_scalar(182, p.p186);s.store_scalar(311, p.p187);s.store_scalar(318, p.p188);s.store_scalar(322, p.p189);s.store_scalar(326, p.p190);}
        if (!s.b[611]) {s.store_scalar(588, (1.0 / p.p29));s.store_primal_max_with_scalar_ad(532, A::scale(s.ad_value(588), p.p21), 1e-9);s.store_primal_scale(10, 588, p.p23);s.store_primal_scale(9, 588, p.p22);s.store_primal_scale(12, 588, p.p25);s.store_primal_scale(11, 588, p.p24);s.store_scalar(13, (p.p30 * p.p29));s.store_scalar(569, 1e-6);s.store_scalar(570, 1e-6);s.store_primal_scale(571, 569, 1.0 / (p.p20));s.store_primal_div(572, 570, 532);s.store_primal_scaled_mul_scale_offset_inputs(573, 571, p.p192, 1.0, 572, p.p193, 1.0, p.p191);s.store_primal_scaled_mul_scale_offset_inputs(574, 572, p.p197, 1.0, 571, p.p196, 1.0, p.p195);s.store_primal_max_with_scalar_ad(575, A::offset(s.ad_value(573), ((p.p20) + ((-(2.0 * p.p194))))), 1e-9);s.store_primal_max_with_scalar_ad(576, A::offset(A::add(s.ad_value(532), s.ad_value(574)), (-(2.0 * p.p198))), 1e-9);s.store_primal_max_with_scalar_ad(577, A::offset(s.ad_value(573), ((((p.p20) + ((-(2.0 * p.p194))))) + (p.p199))), 1e-9);s.store_primal_max_with_scalar_ad(578, A::offset(A::add(s.ad_value(532), s.ad_value(574)), (((-(2.0 * p.p198))) + (p.p200))), 1e-9);s.store_primal_div(579, 569, 575);s.store_primal_div(580, 570, 576);s.store_primal_mul(581, 579, 580);s.store_max_with_scalar_ad(0, A::offset(s.ad_value(573), p.p20), 1e-9);s.store_div(582, 0, 569);s.store_max_with_scalar_ad(0, A::add(s.ad_value(532), s.ad_value(574)), 1e-9);s.store_div(583, 0, 570);s.store_primal_max_with_scalar_ad(312, A::offset(s.ad_value(573), p.p20), 1e-9);s.store_primal_max_with_scalar_ad(313, A::offset(s.ad_value(312), p.p499), 1e-9);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[611]) {s.store_primal_max_with_scalar_ad(314, A::add(s.ad_value(532), s.ad_value(574)), 1e-9);s.store_primal_max_with_scalar_ad(315, A::sub_from_scalar(p.p38, A::scale(s.ad_value(574), 0.5)), 1e-9);s.store_scalar(533, p.p201);s.store_scalar(14, p.p202);s.store_scalar(15, p.p203);s.store_scalar(534, p.p204);}
        let (t2,) = {
    if (!s.b[611]) {
        (1.0,)
    } else {
        (s.v[535],)
    }
};
        s.store_scalar(535, t2);s.b[622] = (p.p205 < 0.0);s.store_scalar(622, if s.b[622] { 1.0 } else { 0.0 });
        let (t4,) = {
    if ((!s.b[611]) && s.b[622]) {
        let t3: f64 = (-1.0);
        (t3,)
    } else {
        (s.v[535],)
    }
};
        s.store_scalar(535, t4);
        if (!s.b[611]) {s.store_scalar(536, ((((p.p205) as f64).abs()).min(1e19) * 1000000.0));s.store_scalar(16, 1.0);}
        s.b[623] = (p.p206 < 0.0);s.store_scalar(623, if s.b[623] { 1.0 } else { 0.0 });
        if ((!s.b[611]) && s.b[623]) {s.store_scalar(16, (-1.0));}
        if (!s.b[611]) {s.store_scalar(537, (((((p.p206) as f64).abs()).max(1e16)).min(1e21) * 1000000.0));s.store_scalar(17, p.p207);s.store_scalar(18, p.p208);s.store_scalar(19, (p.p209 * 1000000.0));s.store_scalar(20, (p.p210 * 1000000.0));s.store_div_scaled_inputs(0, A::powf(s.ad_value(579), p.p213), p.p212, A::scale_offset(A::powf(s.ad_value(579), p.p215), p.p214, 1.0), 1.0);s.store_add_scaled_inputs3_offset_indices(183, 0, 1.0, 580, p.p216, 581, p.p217, p.p211);s.store_offset_mul_ad(184, A::div_scaled_inputs(s.ad_value(534), p.p219, s.ad_value(533), 1.0), s.ad_value(0), p.p218);s.store_primal_mul3_ad_scaled_output(23, A::scale_offset(s.ad_value(579), p.p221, 1.0), A::scale_offset(s.ad_value(580), p.p222, 1.0), A::scale_offset(s.ad_value(581), p.p223, 1.0), p.p220);s.store_offset_scaled(607, 579, ((p.p225) * ((p.p224 * 1000000.0))), (p.p224 * 1000000.0));s.store_min_with_scalar_ad(24, A::max_with_scalar(s.ad_value(607), 1e25), 1e28);s.store_scalar(25, p.p226);s.store_scalar(26, p.p227);s.store_primal_sub_from_scalar(228, 1.0, 15);s.store_primal_add_scaled_inputs(229, 228, 1.04479e-10, 15, 1.43438e-10);s.store_primal_div_mixed_ai(584, A::sqrt(A::mul3_scaled_output(s.ad_value(229), s.ad_value(14), A::offset(s.ad_value(533), 4e-10), 1.0 / (3.45313e-11))), 575);s.store_primal_mul_powf_scale_offset_lhs(544, 584, 580, p.p229, (p.p230) * ((p.p228 * 2.0)), (1.0) * ((p.p228 * 2.0)));s.store_primal_min_with_scalar_ad(27, A::max_with_scalar(s.ad_value(544), 0.0), 5.0);s.store_primal_div_scaled_product_indices(28, 27, 534, p.p231, 533, 1.0);s.store_scalar(29, (p.p232 * 1000000.0));s.store_scalar(30, p.p233);s.store_primal_scale(549, 580, p.p234);s.store_primal_min_with_scalar_ad(538, A::max_with_scalar(s.ad_value(549), (-1.0)), 1.0);s.store_mul_powf_scale_offset_lhs(0, 584, 580, p.p236, p.p237, 1.0);s.store_scale(546, 0, p.p235);s.store_max_with_scalar(187, 546, 0.0);s.store_div_scaled_product_indices(188, 187, 534, p.p238, 533, 1.0);s.store_scale(34, 0, p.p239);s.store_scalar(35, p.p240);s.store_primal_div_scaled_inputs_mixed_ia(36, 579, p.p241, A::max_with_scalar(A::scale_offset(s.ad_value(580), p.p242, 1.0), 0.001), 1.0);s.store_scalar(37, p.p243);s.store_div_scaled_inputs_mixed_ia(2, 575, -1.0, A::max_with_scalar(A::scale_offset(s.ad_value(580), p.p248, 1.0), 0.001), p.p247);}
        s.b[624] = (s.v[2] > (-80.0));s.store_scalar(624, if s.b[624] { 1.0 } else { 0.0 });
        if ((!s.b[611]) && s.b[624]) {s.store_exp(3, 2);}
        if ((!s.b[611]) && (!s.b[624])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(s.ad_value(2)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (!s.b[611]) {s.store_scale(4, 575, (-1.0 / (p.p250)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[625] = (s.v[4] > (-80.0));s.store_scalar(625, if s.b[625] { 1.0 } else { 0.0 });
        if ((!s.b[611]) && s.b[625]) {s.store_exp(5, 4);}
        if ((!s.b[611]) && (!s.b[625])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (!s.b[611]) {s.store_max_with_scalar_ad(585, A::add(A::offset(A::div_scaled_product_offset_rhs(A::scale_offset(s.ad_value(580), p.p246, 1.0), s.ad_value(3), (-1.0), p.p245, s.ad_value(2), 1.0), 1.0), A::div_scaled_offset_numerator(s.ad_value(5), p.p249, ((-1.0) * p.p249), s.ad_value(4), 1.0)), 1e-6);s.store_primal_max_with_scalar_ad(586, A::add_scaled_product(A::scale_offset(s.ad_value(580), p.p251, 1.0), 1.0, s.ad_value(580), A::ln(A::scale_offset(s.ad_value(576), 1.0 / (p.p253), 1.0)), p.p252), 1e-6);s.store_mul_div_from_scalar_lhs_ad_indices(587, p.p244, 585, 586);s.store_div_scaled_product_indices(548, 587, 576, 1.0, 575, 1.0);s.store_max_with_scalar(191, 548, 1e-10);s.store_scale(192, 191, p.p254);s.store_primal_mul3_ad_scaled_output(40, A::scale_offset(s.ad_value(579), p.p256, 1.0), A::scale_offset(s.ad_value(580), p.p257, 1.0), A::scale_offset(s.ad_value(581), p.p258, 1.0), p.p255);s.store_primal_mul3_ad(550, A::scale_offset(A::powf(s.ad_value(579), p.p261), p.p260, p.p259), A::scale_offset(s.ad_value(580), p.p262, 1.0), A::scale_offset(s.ad_value(581), p.p263, 1.0));s.store_primal_max_with_scalar(195, 550, 0.0);s.store_scalar(41, p.p264);s.store_scalar(42, p.p265);s.store_primal_mul3_ad_scaled_output(43, A::scale_offset(s.ad_value(579), p.p267, 1.0), A::scale_offset(s.ad_value(580), p.p268, 1.0), A::scale_offset(s.ad_value(581), p.p269, 1.0), p.p266);s.store_scalar(196, p.p270);s.store_scalar(45, p.p271);s.store_scalar(539, p.p272);s.store_scalar(540, p.p273);s.store_scalar(193, p.p274);s.store_scalar(48, p.p275);s.store_scalar(194, p.p276);s.store_scalar(49, p.p277);s.store_primal_mul3_ad(197, A::scale_offset(A::powf(s.ad_value(579), p.p280), p.p279, p.p278), A::scale_offset(s.ad_value(580), p.p281, 1.0), A::scale_offset(s.ad_value(581), p.p282, 1.0));s.store_scalar(51, p.p283);s.store_scalar(52, p.p284);s.store_scalar(541, p.p285);s.store_primal_mul_scale_offset_rhs(551, 580, 580, ((p.p287) * (p.p286)), p.p286);s.store_primal_max_with_scalar(198, 551, 0.0);s.store_scalar(54, p.p288);s.store_scalar(55, p.p289);s.store_scalar(56, p.p290);s.store_scalar(57, p.p291);s.store_scalar(58, p.p292);s.store_mul_scale_offset_mixed_ai(552, A::mul3(s.ad_value(587), A::scale_offset(A::powf(s.ad_value(579), p.p295), p.p294, p.p293), A::scale_offset(s.ad_value(580), p.p296, 1.0)), 581, p.p297, 1.0);s.store_max_with_scalar(199, 552, 0.0);s.store_primal_mul3_ad_scaled_output(60, A::scale_offset(s.ad_value(579), p.p299, 1.0), A::scale_offset(s.ad_value(580), p.p300, 1.0), A::scale_offset(s.ad_value(581), p.p301, 1.0), p.p298);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[611]) {s.store_scalar(61, p.p302);s.store_scalar(62, p.p303);s.store_primal_div_from_scalar_offset_ad(554, p.p304, A::div_scaled_inputs(A::powf(s.ad_value(579), p.p306), p.p305, A::scale_offset(A::powf(s.ad_value(579), p.p308), p.p307, 1.0), 1.0), 1.0);s.store_primal_min_with_scalar_ad(542, A::max_with_scalar(s.ad_value(554), 1.0), 16.0);s.store_primal_div_scaled_product(557, A::powf(s.ad_value(579), p.p310), A::scale_offset(s.ad_value(580), p.p313, 1.0), p.p309, A::scale_offset(A::powf(s.ad_value(579), p.p312), p.p311, 1.0), 1.0);s.store_primal_max_with_scalar(63, 557, 0.0);s.store_primal_div_scaled_product(558, A::powf(s.ad_value(579), p.p315), A::scale_offset(s.ad_value(580), p.p318, 1.0), p.p314, A::scale_offset(A::powf(s.ad_value(579), p.p317), p.p316, 1.0), 1.0);s.store_primal_max_with_scalar(64, 558, 0.0);s.store_scalar(65, p.p319);s.store_scalar(66, p.p320);s.store_scalar(67, p.p321);s.store_scalar(75, p.p322);s.store_primal_div_from_scalar(201, p.p323, 581);s.store_primal_div_from_scalar(202, p.p324, 580);s.store_primal_div_from_scalar(203, p.p325, 580);s.store_primal_div_from_scalar(206, p.p339, 580);s.store_primal_div_from_scalar(207, p.p340, 580);s.store_primal_div_from_scalar(204, p.p326, 580);s.store_primal_div_from_scalar(205, p.p327, 580);s.store_scalar(76, p.p328);s.store_scalar(77, p.p342);s.store_scalar(78, p.p329);s.store_scalar(79, p.p330);s.store_scalar(80, p.p331);s.store_scalar(81, p.p341);s.store_scalar(82, p.p332);s.store_scalar(83, p.p333);s.store_scalar(84, p.p334);s.store_primal_scale(85, 579, p.p335);s.store_scalar(86, p.p336);s.store_scalar(87, p.p337);s.store_scalar(88, p.p338);s.store_primal_offset_div_from_scalar_ad(559, p.p345, s.ad_value(580), p.p343);s.store_max_with_scalar(89, 559, 0.0);s.store_primal_offset_div_from_scalar_ad(560, p.p346, s.ad_value(580), p.p344);s.store_max_with_scalar(90, 560, 0.0);s.store_scalar(208, p.p347);s.store_scalar(209, p.p348);s.store_scalar(93, p.p349);s.store_scalar(94, p.p350);s.store_scalar(95, p.p351);s.store_scalar(96, p.p352);s.store_primal_offset_scaled(97, 579, p.p355, p.p353);s.store_primal_offset_scaled(98, 579, p.p356, p.p354);s.store_primal_scaled_mul_scale_offset_inputs(561, 579, p.p389, 1.0, 580, p.p390, 1.0, p.p388);s.store_primal_max_with_scalar(112, 561, 0.0);s.store_scalar(210, p.p391);s.store_scalar(114, p.p392);s.store_primal_scaled_mul_scale_offset_inputs(562, 579, p.p394, 1.0, 580, p.p395, 1.0, p.p393);s.store_primal_max_with_scalar(115, 562, 0.0);s.store_primal_offset_scaled(589, 576, p.p358, (2.0 * p.p357));s.store_scalar(99, p.p359);s.store_scale_ad(0, A::powf(s.ad_value(579), p.p362), p.p361);s.store_add_scaled_inputs3_offset_indices(211, 0, 1.0, 580, p.p363, 581, p.p364, p.p360);s.store_scalar(212, p.p365);s.store_primal_mul3_ad_scaled_output(102, A::scale_offset(s.ad_value(579), p.p367, 1.0), A::scale_offset(s.ad_value(580), p.p368, 1.0), A::scale_offset(s.ad_value(581), p.p369, 1.0), p.p366);s.store_scalar(103, p.p370);s.store_scalar(104, p.p371);s.store_mul_powf_scale_offset_lhs(0, 584, 580, p.p373, (p.p374) * ((p.p372 * 2.0)), (1.0) * ((p.p372 * 2.0)));s.store_min_with_scalar_ad(105, A::max_with_scalar(s.ad_value(0), 0.0), 5.0);s.store_div_scaled_product_indices(106, 105, 534, p.p375, 533, 1.0);s.store_mul_powf_scale_offset_lhs(0, 584, 580, p.p377, p.p378, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (!s.b[611]) {s.store_scale(0, 0, p.p376);s.store_max_with_scalar(107, 0, 0.0);s.store_div_scaled_product_indices(108, 107, 534, p.p379, 533, 1.0);s.store_scalar(109, p.p380);s.store_offset_ad(0, A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p381 * p.p382), s.ad_value(575)), 1.0, A::exp_scaled_input(s.ad_value(575), (-1.0 / (p.p382)))), 1.0);s.store_max_with_scalar(0, 0, 1e-15);s.store_mul_div_scaled_inputs_mixed_aia(213, A::scale_offset(s.ad_value(580), p.p383, 1.0), 589, p.p244, A::mul(s.ad_value(0), s.ad_value(575)), 1.0);s.store_primal_add_scaled_inputs_product_mixed_aiii(111, A::scale_offset(s.ad_value(579), p.p385, p.p384), 1.0, 580, p.p386, 579, 580, p.p387);s.store_primal_mul(116, 578, 577);s.store_offset_scaled(563, 582, p.p397, p.p396);s.store_max_with_scalar(117, 563, 0.0);s.store_scalar(118, (p.p398 * 1000000.0));s.store_primal_div_scaled_inputs_indices(119, 578, p.p399, 570, 1.0);s.store_scalar(120, p.p400);s.copy_ad(185, 183);s.copy_ad(186, 184);s.copy_ad(135, 27);s.copy_ad(136, 28);s.copy_ad(547, 546);s.copy_ad(189, 187);s.copy_ad(190, 188);s.copy_ad(553, 552);s.copy_ad(200, 199);s.copy_ad(543, 542);s.copy_ad(158, 63);}
        s.b[626] = (p.p11 > 0.0);s.store_scalar(626, if s.b[626] { 1.0 } else { 0.0 });
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(121, p.p211);}
        s.b[627] = param_given[401];s.store_scalar(627, if s.b[627] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[627]) {s.store_scalar(121, p.p401);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(122, p.p212);}
        s.b[628] = param_given[402];s.store_scalar(628, if s.b[628] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[628]) {s.store_scalar(122, p.p402);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(123, p.p213);}
        s.b[629] = param_given[403];s.store_scalar(629, if s.b[629] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[629]) {s.store_scalar(123, p.p403);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(124, p.p216);}
        s.b[630] = param_given[406];s.store_scalar(630, if s.b[630] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[630]) {s.store_scalar(124, p.p406);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(125, p.p217);}
        s.b[631] = param_given[407];s.store_scalar(631, if s.b[631] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[631]) {s.store_scalar(125, p.p407);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(126, p.p214);}
        s.b[632] = param_given[404];s.store_scalar(632, if s.b[632] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[632]) {s.store_scalar(126, p.p404);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(127, p.p215);}
        s.b[633] = param_given[405];s.store_scalar(633, if s.b[633] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[633]) {s.store_scalar(127, p.p405);}
        if ((!s.b[611]) && s.b[626]) {s.store_div_scaled_product_offset_denominator_mixed_iaa(0, 122, A::pow(s.ad_value(579), s.ad_value(123)), 1.0, A::mul(s.ad_value(126), A::pow(s.ad_value(579), s.ad_value(127))), 1.0, 1.0);s.store_add_scaled_inputs_products_indices(185, 121, 1.0, 0, 1.0, 124, 580, 1.0, 125, 581, 1.0);s.store_scalar(128, p.p218);}
        s.b[634] = param_given[408];s.store_scalar(634, if s.b[634] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[634]) {s.store_scalar(128, p.p408);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(129, p.p219);}
        s.b[635] = param_given[409];s.store_scalar(635, if s.b[635] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[635]) {s.store_scalar(129, p.p409);}
        if ((!s.b[611]) && s.b[626]) {s.store_add_scaled_product_mixed_iai(186, 128, 1.0, A::div_scaled_product(s.ad_value(129), s.ad_value(534), 1.0, s.ad_value(533), 1.0), 0, 1.0);s.store_scalar(132, p.p228);}
        s.b[636] = param_given[410];s.store_scalar(636, if s.b[636] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[636]) {s.store_scalar(132, p.p410);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(133, p.p229);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[637] = param_given[411];s.store_scalar(637, if s.b[637] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[637]) {s.store_scalar(133, p.p411);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(134, p.p230);}
        s.b[638] = param_given[412];s.store_scalar(638, if s.b[638] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[638]) {s.store_scalar(134, p.p412);}
        if ((!s.b[611]) && s.b[626]) {s.store_primal_mul_ad_affine_product_rhs(545, 132, A::pow(s.ad_value(584), s.ad_value(133)), A::offset(A::mul(s.ad_value(134), s.ad_value(580)), 1.0), 2.0, 0.0);s.store_primal_min_with_scalar_ad(135, A::max_with_scalar(s.ad_value(545), 0.0), 5.0);s.store_primal_div_scaled_product_indices(136, 135, 534, p.p231, 533, 1.0);s.store_scalar(137, p.p235);}
        s.b[639] = param_given[413];s.store_scalar(639, if s.b[639] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[639]) {s.store_scalar(137, p.p413);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(138, p.p236);}
        s.b[640] = param_given[414];s.store_scalar(640, if s.b[640] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[640]) {s.store_scalar(138, p.p414);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(139, p.p237);}
        s.b[641] = param_given[415];s.store_scalar(641, if s.b[641] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[641]) {s.store_scalar(139, p.p415);}
        if ((!s.b[611]) && s.b[626]) {s.store_mul_scale_offset(0, A::pow(s.ad_value(584), s.ad_value(138)), A::mul(s.ad_value(139), s.ad_value(580)), 1.0, 1.0);s.store_mul(547, 137, 0);s.store_max_with_scalar(189, 547, 0.0);s.store_div_scaled_product_indices(190, 189, 534, p.p238, 533, 1.0);s.store_scalar(142, p.p293);}
        s.b[642] = param_given[416];s.store_scalar(642, if s.b[642] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[642]) {s.store_scalar(142, p.p416);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(143, p.p294);}
        s.b[643] = param_given[417];s.store_scalar(643, if s.b[643] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[643]) {s.store_scalar(143, p.p417);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(144, p.p295);}
        s.b[644] = param_given[418];s.store_scalar(644, if s.b[644] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[644]) {s.store_scalar(144, p.p418);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(145, p.p296);}
        s.b[645] = param_given[419];s.store_scalar(645, if s.b[645] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[645]) {s.store_scalar(145, p.p419);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(146, p.p297);}
        s.b[646] = param_given[420];s.store_scalar(646, if s.b[646] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[646]) {s.store_scalar(146, p.p420);}
        if ((!s.b[611]) && s.b[626]) {s.store_mul_scale_offset(553, A::mul3(s.ad_value(587), A::add_scaled_product(s.ad_value(142), 1.0, s.ad_value(143), A::pow(s.ad_value(579), s.ad_value(144)), 1.0), A::offset(A::mul(s.ad_value(145), s.ad_value(580)), 1.0)), A::mul(s.ad_value(146), s.ad_value(581)), 1.0, 1.0);s.store_max_with_scalar(200, 553, 0.0);s.store_scalar(148, p.p304);}
        s.b[647] = param_given[421];s.store_scalar(647, if s.b[647] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[647]) {s.store_scalar(148, p.p421);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(149, p.p305);}
        s.b[648] = param_given[422];s.store_scalar(648, if s.b[648] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[648]) {s.store_scalar(149, p.p422);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(150, p.p306);}
        s.b[649] = param_given[423];s.store_scalar(649, if s.b[649] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[649]) {s.store_scalar(150, p.p423);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(151, p.p307);}
        s.b[650] = param_given[424];s.store_scalar(650, if s.b[650] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[650]) {s.store_scalar(151, p.p424);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(152, p.p308);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[651] = param_given[425];s.store_scalar(651, if s.b[651] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[651]) {s.store_scalar(152, p.p425);}
        if ((!s.b[611]) && s.b[626]) {s.store_primal_div_scaled_value_offset_denominator(555, s.ad_value(148), 1.0, A::div_scaled_product_offset_denominator(s.ad_value(149), A::pow(s.ad_value(579), s.ad_value(150)), 1.0, A::mul(s.ad_value(151), A::pow(s.ad_value(579), s.ad_value(152))), 1.0, 1.0), 1.0, 1.0);s.store_primal_min_with_scalar_ad(543, A::max_with_scalar(s.ad_value(555), 1.0), 16.0);s.store_scalar(153, p.p309);}
        s.b[652] = param_given[426];s.store_scalar(652, if s.b[652] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[652]) {s.store_scalar(153, p.p426);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(154, p.p310);}
        s.b[653] = param_given[427];s.store_scalar(653, if s.b[653] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[653]) {s.store_scalar(154, p.p427);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(155, p.p311);}
        s.b[654] = param_given[428];s.store_scalar(654, if s.b[654] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[654]) {s.store_scalar(155, p.p428);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(156, p.p312);}
        s.b[655] = param_given[429];s.store_scalar(655, if s.b[655] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[655]) {s.store_scalar(156, p.p429);}
        if ((!s.b[611]) && s.b[626]) {s.store_scalar(157, p.p313);}
        s.b[656] = param_given[430];s.store_scalar(656, if s.b[656] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[626]) && s.b[656]) {s.store_scalar(157, p.p430);}
        if ((!s.b[611]) && s.b[626]) {s.store_primal_div_scaled_product3_mixed_iaaa(556, 153, A::pow(s.ad_value(579), s.ad_value(154)), A::offset(A::mul(s.ad_value(157), s.ad_value(580)), 1.0), 1.0, A::offset(A::mul(s.ad_value(155), A::pow(s.ad_value(579), s.ad_value(156))), 1.0), 1.0);s.store_primal_max_with_scalar(158, 556, 0.0);}
        if (!s.b[611]) {s.store_mul_div_from_scalar_lhs_ad_indices(0, 3.45313e-11, 533, 578);s.store_scale(159, 0, p.p431);s.store_scale(160, 0, p.p432);s.store_primal_div_from_scalar_ad(161, p.p433, A::max_with_scalar(A::offset(A::div_scaled_inputs(s.ad_value(570), p.p434, s.ad_value(578), 1.0), 1.0), 0.001));s.store_scalar(162, p.p435);s.store_scalar(163, p.p436);s.store_offset_scaled(564, 583, p.p439, p.p437);s.store_max_with_scalar(164, 564, 0.0);s.store_offset_scaled(565, 583, p.p440, p.p438);s.store_max_with_scalar(165, 565, 0.0);s.store_primal_div_scaled_product3_indices(166, 229, 14, 576, p.p441, 575, 1.0);s.store_scalar(167, p.p442);s.store_max_with_scalar_ad(0, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(582), p.p444, 1.0), 1.0, s.ad_value(583), p.p445, s.ad_value(582), s.ad_value(583), p.p446), 1e-10);s.store_scalar(2, 0.0);}
        s.b[657] = ((p.p29 > 1.0) && (p.p28 > 0.0));s.store_scalar(657, if s.b[657] { 1.0 } else { 0.0 });
        if ((!s.b[611]) && s.b[657]) {s.store_scalar(3, ((-(p.p28 + p.p20)) / p.p449));}
        s.b[658] = (((s.v[3]) as f64).abs() < 80.0);s.store_scalar(658, if s.b[658] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[657]) && s.b[658]) {s.store_exp(4, 3);}
        s.b[659] = (s.v[3] < (-80.0));s.store_scalar(659, if s.b[659] { 1.0 } else { 0.0 });
        if ((((!s.b[611]) && s.b[657]) && (!s.b[658])) && s.b[659]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(4, 1.80485e-35, A::neg(s.ad_value(3)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((((!s.b[611]) && s.b[657]) && (!s.b[658])) && (!s.b[659])) {s.store_scaled_softlimit_poly_offset_lhs_mul_scaled_ad_rhs(4, 3, (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0, 5.54062e34);}
        if ((!s.b[611]) && s.b[657]) {s.store_sub_from_scalar(5, 1.0, 4);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[611]) && s.b[657]) {s.store_div_scaled_product_mixed_iaa(2, 4, A::sub(s.ad_value(5), A::scale_offset(A::powi(s.ad_value(4), (p.p29 as i32)), (-1.0 / (p.p29)), 1.0 / (p.p29))), (2.0 * p.p450), A::square(s.ad_value(5)), 1.0);}
        if (!s.b[611]) {s.store_div_scaled_value_offset_denominator(0, s.ad_value(0), 1.0, s.ad_value(2), 1.0, 1.0);s.store_div_from_scalar(566, p.p443, 0);s.store_max_with_scalar(214, 566, 1e-6);s.store_scalar(169, p.p447);s.store_scale(567, 0, p.p448);s.store_max_with_scalar(170, 567, 0.0);}
        let (t8,) = {
    if (!s.b[611]) {
        (p.p451,)
    } else {
        (s.v[171],)
    }
};
        s.store_scalar(171, t8);
        let (t10,) = {
    if (!s.b[611]) {
        let t9: f64 = (p.p452 * s.v[548]);let ta: f64 = (t9 * s.v[548]);let tb: f64 = (ta * s.v[580]);let tc: f64 = (tb * s.v[580]);let td: f64 = (p.p453 - 2.0);let te: f64 = (s.v[579]).powf(td);let tf: f64 = (tc * te);
        (tf,)
    } else {
        (s.v[172],)
    }
};
        s.store_scalar(172, t10);
        if (!s.b[611]) {s.store_primal_add_scaled_inputs(568, 581, p.p454, 580, p.p455);s.store_primal_max_with_scalar(173, 568, 0.0);s.store_primal_scale(174, 581, p.p456);s.store_primal_scale(175, 581, p.p457);s.store_scalar(176, p.p458);s.store_scalar(177, p.p459);s.store_offset_scaled(0, 579, p.p490, p.p489);s.store_max_with_scalar(179, 0, 0.0);s.store_offset_scaled(0, 579, p.p492, p.p491);s.store_max_with_scalar(180, 0, 0.0);s.store_scalar(181, p.p493);s.store_scalar(182, p.p494);s.store_primal_offset_add_ad(310, A::div_scaled_inputs2(s.ad_value(314), ((0.3333333333333 * 1.0 / (p.p37)) * p.p498), s.ad_value(315), p.p498, s.ad_value(313), p.p37), A::div_from_scalar((p.p496 + p.p497), A::mul(s.ad_value(314), s.ad_value(312))), (p.p29 * p.p495));s.store_primal_max_with_scalar(311, 310, 0.0);s.store_scalar(319, (p.p500).max(0.0));s.store_scalar(323, (p.p501).max(0.0));}
        s.b[660] = (p.p7 == 0.0);s.store_scalar(660, if s.b[660] { 1.0 } else { 0.0 });
        if ((!s.b[611]) && s.b[660]) {s.copy_ad(323, 319);}
        if (!s.b[611]) {s.store_primal_scale(318, 319, (p.p29 * p.p39));s.store_primal_scale(322, 323, (p.p29 * p.p40));s.store_scalar(326, (p.p29 * p.p502));}
        s.b[661] = ((((p.p461 > 0.0) && (p.p26 > 0.0)) && (p.p27 > 0.0)) && ((p.p29 == 1.0) || ((p.p29 > 1.0) && (p.p28 > 0.0))));s.store_scalar(661, if s.b[661] { 1.0 } else { 0.0 });s.b[662] = (p.p461 == 1.0);s.store_scalar(662, if s.b[662] { 1.0 } else { 0.0 });
        if (((!s.b[611]) && s.b[661]) && s.b[662]) {s.store_scalar(592, 0.0);s.store_scalar(593, 0.0);s.store_scalar(594, 0.0);}
        let mut t13: usize = 0;
        while {
            let t11: f64 = (p.p29 - 0.5);let t12: f64 = if ((((!s.b[611]) && s.b[661]) && s.b[662]) && (s.v[594] < t11)) { 1.0 } else { 0.0 };
            t12 != 0.0
        } {
            t13 += 1;assert!(t13 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[611]) && s.b[661]) && s.b[662]) {s.store_add_mixed_ia(592, 592, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(594), (p.p28 + p.p20), (p.p26 + (0.5 * p.p20)))));s.store_primal_add_mixed_ia(593, 593, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(594), (p.p28 + p.p20), (p.p27 + (0.5 * p.p20)))));s.store_primal_offset(594, 594, 1.0);}
        }
        if (((!s.b[611]) && s.b[661]) && s.b[662]) {s.store_scale(595, 592, 1.0 / (p.p29));s.store_primal_scale(596, 593, 1.0 / (p.p29));s.store_scalar(597, (1.0 / (p.p462 + (0.5 * p.p20))));s.store_scalar(598, (1.0 / (p.p463 + (0.5 * p.p20))));s.store_primal_max_with_scalar_ad(599, A::offset(s.ad_value(573), p.p20), 1e-9);s.store_primal_max_with_scalar_ad(600, A::offset(A::add(s.ad_value(532), s.ad_value(574)), p.p464), 1e-9);s.store_primal_div_from_scalar_powf_ad(601, 1.0, s.ad_value(599), p.p471);s.store_primal_div_from_scalar_powf_ad(602, 1.0, s.ad_value(600), p.p472);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[611]) && s.b[661]) && s.b[662]) {s.store_mul_scale_offset_mixed_ai(603, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(601), p.p468, 1.0), 1.0, s.ad_value(602), p.p469, s.ad_value(601), s.ad_value(602), p.p470), 221, p.p467, (((((-1.0)) * (p.p467))) + (1.0)));s.store_div_scaled_inputs2_indices(604, 595, p.p465, 596, p.p465, 603, 1.0);s.store_div_scaled_inputs2_indices(605, 597, p.p465, 598, p.p465, 603, 1.0);s.store_primal_div_from_scalar_powf_ad(601, 1.0, s.ad_value(599), p.p477);s.store_primal_div_from_scalar_powf_ad(602, 1.0, s.ad_value(600), p.p478);s.store_primal_max_with_scalar_ad(606, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(601), p.p474, 1.0), 1.0, s.ad_value(602), p.p475, s.ad_value(601), s.ad_value(602), p.p476), 1e-20);s.store_add_scaled_inputs4_indices(607, 595, 1.0, 596, 1.0, 597, -1.0, 598, -1.0);s.store_div_scaled_product_offset_denominator_mixed_iai(548, 548, A::offset(s.ad_value(604), 1.0), 1.0, 605, 1.0, 1.0);s.store_max_with_scalar(191, 548, 1e-10);s.store_scale(192, 191, p.p254);s.store_div_scaled_product_by_product(0, A::offset(s.ad_value(604), 1.0), A::scale_offset(s.ad_value(605), p.p466, 1.0), 1.0, A::offset(s.ad_value(605), 1.0), A::scale_offset(s.ad_value(604), p.p466, 1.0), 1.0);s.store_mul(552, 552, 0);s.store_max_with_scalar(199, 552, 0.0);s.store_mul(553, 553, 0);s.store_max_with_scalar(200, 553, 0.0);s.store_div_scaled_inputs_indices(0, 607, p.p473, 606, 1.0);s.store_add(183, 183, 0);s.store_add(184, 184, 0);s.store_add(185, 185, 0);s.store_add(186, 186, 0);s.store_div_scaled_inputs_mixed_ia(0, 607, p.p479, A::powf(s.ad_value(606), p.p480), 1.0);s.store_add(546, 546, 0);s.store_max_with_scalar(187, 546, 0.0);s.store_add(547, 547, 0);s.store_max_with_scalar(189, 547, 0.0);s.store_div_scaled_inputs_indices(0, 534, p.p238, 533, 1.0);s.store_mul(188, 187, 0);s.store_mul(190, 189, 0);}
        if (((!s.b[611]) && s.b[661]) && (!s.b[662])) {s.store_scalar(592, 0.0);s.store_scalar(594, 0.0);s.store_scalar(0, ((-1.0) / p.p482));}
        let mut t16: usize = 0;
        while {
            let t14: f64 = (p.p29 - 0.5);let t15: f64 = if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && (s.v[594] < t14)) { 1.0 } else { 0.0 };
            t15 != 0.0
        } {
            t16 += 1;assert!(t16 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");s.b[663] = (((-((p.p26 + (0.5 * p.p20)) + (s.v[594] * (p.p28 + p.p20)))) / p.p481) > (-80.0));s.store_scalar(663, if s.b[663] { 1.0 } else { 0.0 });
            if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && s.b[663]) {s.store_exp_scaled_input_ad(2, A::scale_offset(s.ad_value(594), (p.p28 + p.p20), (p.p26 + (0.5 * p.p20))), (-1.0 / (p.p481)));}
            if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && (!s.b[663])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(2, 1.80485e-35, A::neg(A::scale(A::scale_offset(s.ad_value(594), (p.p28 + p.p20), (p.p26 + (0.5 * p.p20))), (-1.0 / (p.p481)))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
            s.b[664] = (((-((p.p27 + (0.5 * p.p20)) + (((p.p29 - 1.0) - s.v[594]) * (p.p28 + p.p20)))) / p.p481) > (-80.0));s.store_scalar(664, if s.b[664] { 1.0 } else { 0.0 });
            if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && s.b[664]) {s.store_exp_scaled_input_ad(3, A::scale_offset(s.ad_value(594), (-(p.p28 + p.p20)), (((((p.p29 - 1.0)) * ((p.p28 + p.p20)))) + ((p.p27 + (0.5 * p.p20))))), (-1.0 / (p.p481)));}
            if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && (!s.b[664])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(A::scale(A::scale_offset(s.ad_value(594), (-(p.p28 + p.p20)), (((((p.p29 - 1.0)) * ((p.p28 + p.p20)))) + ((p.p27 + (0.5 * p.p20))))), (-1.0 / (p.p481)))), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
            if (((!s.b[611]) && s.b[661]) && (!s.b[662])) {s.store_powf_ad(4, A::sub_from_scalar(1.0, s.ad_value(2)), (-p.p482));s.store_powf_ad(5, A::sub_from_scalar(1.0, s.ad_value(3)), (-p.p482));s.store_add_mixed_ia(592, 592, A::pow(A::add_scaled_inputs(s.ad_value(4), 0.5, s.ad_value(5), 0.5), s.ad_value(0)));s.store_primal_offset(594, 594, 1.0);}
        }
        if (((!s.b[611]) && s.b[661]) && (!s.b[662])) {s.store_sub_from_scalar_scaled_input(608, 1.0, 592, 1.0 / (p.p29));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[665] = (((-(p.p462 + (0.5 * p.p20))) / p.p481) > (-80.0));s.store_scalar(665, if s.b[665] { 1.0 } else { 0.0 });
        if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && s.b[665]) {s.store_scalar(2, ((((-(p.p462 + (0.5 * p.p20))) / p.p481)) as f64).exp());}
        if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && (!s.b[665])) {s.store_scalar(2, (1.80485e-35 / (1.0 + (((-((-(p.p462 + (0.5 * p.p20))) / p.p481)) - 80.0) * (1.0 + ((0.5 * ((-((-(p.p462 + (0.5 * p.p20))) / p.p481)) - 80.0)) * (1.0 + (((-((-(p.p462 + (0.5 * p.p20))) / p.p481)) - 80.0) * 0.3333333333333))))))));}
        s.b[666] = (((-(p.p463 + (0.5 * p.p20))) / p.p481) > (-80.0));s.store_scalar(666, if s.b[666] { 1.0 } else { 0.0 });
        if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && s.b[666]) {s.store_scalar(3, ((((-(p.p463 + (0.5 * p.p20))) / p.p481)) as f64).exp());}
        if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && (!s.b[666])) {s.store_scalar(3, (1.80485e-35 / (1.0 + (((-((-(p.p463 + (0.5 * p.p20))) / p.p481)) - 80.0) * (1.0 + ((0.5 * ((-((-(p.p463 + (0.5 * p.p20))) / p.p481)) - 80.0)) * (1.0 + (((-((-(p.p463 + (0.5 * p.p20))) / p.p481)) - 80.0) * 0.3333333333333))))))));}
        if (((!s.b[611]) && s.b[661]) && (!s.b[662])) {s.store_powf_ad(4, A::sub_from_scalar(1.0, s.ad_value(2)), (-p.p482));s.store_powf_ad(5, A::sub_from_scalar(1.0, s.ad_value(3)), (-p.p482));s.store_sub_from_scalar_ad(609, 1.0, A::pow(A::add_scaled_inputs(s.ad_value(4), 0.5, s.ad_value(5), 0.5), s.ad_value(0)));s.store_primal_max_with_scalar_ad(600, A::offset(A::add(s.ad_value(532), s.ad_value(574)), p.p464), 1e-9);s.store_div_from_scalar_offset_scaled_input(610, p.p486, 221, p.p487, (((((-1.0)) * (p.p487))) + (1.0)));s.store_mul(604, 610, 608);s.store_mul(605, 610, 609);s.store_sub(607, 608, 609);s.store_primal_max_with_scalar_ad(606, A::offset(A::div_scaled_inputs(s.ad_value(600), p.p484, s.ad_value(570), 1.0), 1.0), 1e-20);s.store_div_scaled_product_offset_denominator_mixed_iai(548, 548, A::offset(s.ad_value(604), 1.0), 1.0, 605, 1.0, 1.0);s.store_max_with_scalar(191, 548, 1e-10);s.store_scale(192, 191, p.p254);s.store_div_scaled_product_by_product(0, A::offset(s.ad_value(604), 1.0), A::scale_offset(s.ad_value(605), p.p488, 1.0), 1.0, A::offset(s.ad_value(605), 1.0), A::scale_offset(s.ad_value(604), p.p488, 1.0), 1.0);s.store_mul(552, 552, 0);s.store_max_with_scalar(199, 552, 0.0);s.store_mul(553, 553, 0);s.store_max_with_scalar(200, 553, 0.0);s.store_div_scaled_inputs_indices(0, 607, p.p483, 606, 1.0);s.store_add(183, 183, 0);s.store_add(184, 184, 0);s.store_add(185, 185, 0);s.store_add(186, 186, 0);s.store_mul_ad_affine_product_rhs(0, 607, A::powf(s.ad_value(584), p.p236), A::scale_offset(s.ad_value(580), p.p237, 1.0), p.p485, 0.0);s.store_add(546, 546, 0);s.store_max_with_scalar(187, 546, 0.0);s.store_add(547, 547, 0);s.store_max_with_scalar(189, 547, 0.0);s.store_div_scaled_inputs_indices(0, 534, p.p238, 533, 1.0);s.store_mul(188, 187, 0);s.store_mul(190, 189, 0);}
        s.b[667] = (p.p7 == 0.0);s.store_scalar(667, if s.b[667] { 1.0 } else { 0.0 });
        if s.b[667] {s.copy_ad(20, 19);s.copy_ad(203, 202);s.copy_ad(207, 206);s.copy_ad(205, 204);s.copy_ad(90, 89);s.copy_ad(209, 208);s.copy_ad(94, 93);s.copy_ad(96, 95);s.copy_ad(98, 97);s.copy_ad(160, 159);s.copy_ad(165, 164);}
        s.store_primal_sub_from_scalar(228, 1.0, 15);s.store_primal_add_scaled_inputs(229, 228, 1.04479e-10, 15, 1.43438e-10);s.store_sub_from_scalar_ad(230, 1.17, A::div_scaled_value_offset_denominator(s.ad_value(218), 0.000473, s.ad_value(217), 636.0, 1.0));s.store_sub_from_scalar_ad(231, 0.744, A::div_scaled_value_offset_denominator(s.ad_value(218), 0.0004774, s.ad_value(217), 235.0, 1.0));s.store_mul_add_scaled_inputs3_offset_rhs_indices(232, 15, 231, 1.0, 230, (-1.0), 228, (-0.4), 0.0);s.store_add(233, 230, 232);s.store_scaled_mul(234, 233, 224, 0.5);s.copy_ad(235, 234);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_primal_div_from_scalar_offset_ad(238, 1.0, A::sqrt_scaled_input(s.ad_value(15), 10.0), 1.0);s.store_sub_scaled_inputs(237, 15, 0.05, 232, 0.5);s.store_scaled_mul(0, 536, 14, ((1.602176565e-19 * 0.5) * 28959234086.17689));s.b[668] = (s.v[535] > 0.0);s.store_scalar(668, if s.b[668] { 1.0 } else { 0.0 });
        if s.b[668] {s.store_mul_scale_offset_indices(243, 0, 533, 1.0, (p.p13 * 4e-10));s.store_mul_scale_offset_indices(244, 0, 534, 1.0, (p.p13 * 4e-10));}
        if (!s.b[668]) {s.store_mul_scaled_offset_rhs(243, 0, -1.0, 533, (p.p13 * 4e-10));s.store_mul_scaled_offset_rhs(244, 0, -1.0, 534, (p.p13 * 4e-10));}
        s.store_sqrt_scaled_input(0, 217, 0.0033333333333);s.store_mul3_affine_lhs(2, 0, 0, 4.05e25, 0.0, 0);s.store_mul(252, 2, 238);s.store_mul_exp_mixed_ia(251, 2, A::mul_scaled_lhs(s.ad_value(232), 0.5, s.ad_value(224)));s.store_mul_exp_mixed_ia(590, 2, A::mul_scaled_lhs(s.ad_value(232), 0.5, s.ad_value(224)));s.store_primal_div_from_scalar(239, 3.45313e-11, 533);s.store_primal_div_from_scalar(240, 3.45313e-11, 534);s.b[669] = (s.v[538] > 0.0);s.store_scalar(669, if s.b[669] { 1.0 } else { 0.0 });
        if s.b[669] {s.store_primal_mul_scale_offset_indices(241, 239, 538, 1.0, 1.0);s.copy_ad(242, 240);}
        if (!s.b[669]) {s.copy_ad(241, 239);s.store_primal_mul_scale_offset_indices(242, 240, 538, -1.0, 1.0);}
        s.store_primal_div(245, 229, 14);s.store_mul_scale_offset_mixed_ia(226, 223, A::mul(s.ad_value(17), s.ad_value(222)), 1.0, 1.0);s.store_div_from_scalar(227, 1.0, 226);s.store_scaled_mul(236, 233, 227, 0.5);s.store_primal_div(246, 241, 245);s.store_primal_div(247, 242, 245);s.store_primal_div_from_scalar_add_ad(248, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(246)), 1.0), A::div_from_scalar(1.0, s.ad_value(247)));s.store_mul3_affine_lhs(253, 252, 229, (2.0 * 1.602176565e-19), 0.0, 227);s.store_offset_ln_ad(254, A::div_scaled_product(s.ad_value(245), s.ad_value(245), 1.0, s.ad_value(253), 1.0), (-0.6931471805599));s.store_mul_div_scaled_product_mixed_iiia(255, 227, 29, 14, (0.5 * 1.602176565e-19), A::add(s.ad_value(241), s.ad_value(242)), 1.0);s.store_mul(0, 34, 220);s.store_add(31, 187, 0);s.store_add(32, 188, 0);s.store_add(140, 189, 0);s.store_add(141, 190, 0);s.store_mul(329, 35, 227);s.store_div_mixed_ai(260, A::sqrt(A::mul_scaled_lhs(s.ad_value(537), ((2.0 * 1.602176565e-19) * 1.04479e-10), s.ad_value(224))), 242);s.store_square(261, 260);s.store_div_from_scalar(262, 1.0, 261);s.store_offset_scaled(263, 260, 0.707106781186545, 1.0);s.store_div_from_scalar(264, 1.0, 263);let t17: f64 = (1e-5 * s.v[263]);s.store_scalar(265, t17);s.store_add_ln_div_lhs(591, 537, 590, 234);s.store_scale(266, 591, 2.0);s.b[670] = (p.p2 > 0.0);s.store_scalar(670, if s.b[670] { 1.0 } else { 0.0 });
        if s.b[670] {s.store_add_product3_rhs_indices(184, 184, 16, 223, 591, 1.0);s.store_add_product3_rhs_indices(186, 186, 16, 223, 591, 1.0);}
        s.store_scalar(249, 0.0);s.b[671] = (p.p9 > 0.0);s.store_scalar(671, if s.b[671] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[671] {s.store_mul_add_mixed_iai(249, 223, A::ln(A::div(s.ad_value(24), s.ad_value(251))), 234);}
        s.store_div_mixed_ai(250, A::sqrt(A::mul_scaled_lhs(s.ad_value(229), (2.0 * 1.602176565e-19), s.ad_value(24))), 239);s.store_scalar(257, 15.0);s.b[672] = (p.p10 == 1.0);s.store_scalar(672, if s.b[672] { 1.0 } else { 0.0 });
        if s.b[672] {s.store_scaled_add_ad(257, A::offset(A::div_from_scalar(2970.0, s.ad_value(8)), 15.0), A::sqrt_square_offset(A::sub_from_scalar(15.0, A::div_from_scalar(2970.0, s.ad_value(8))), 1e-6), 0.5);}
        s.store_scalar(256, 0.0);s.store_scalar(258, 0.0);s.store_primal_scaled_mul(259, 14, 14, 1e18);s.b[673] = (p.p13 > 0.0);s.store_scalar(673, if s.b[673] { 1.0 } else { 0.0 });s.b[674] = (p.p14 == 1.0);s.store_scalar(674, if s.b[674] { 1.0 } else { 0.0 });
        if (s.b[673] && s.b[674]) {s.store_primal_div_from_scalar(256, 0.409618895, 259);s.store_scale_ad(258, A::exp_scaled_input(A::ln(A::mul(s.ad_value(226), s.ad_value(259))), (-0.3333333333333)), ((0.4 * p.p13) * 1.27520989));}
        if (s.b[673] && (!s.b[674])) {s.store_primal_div_from_scalar(256, 0.723134895, 259);s.store_scale_ad(258, A::exp_scaled_input(A::ln(A::mul(s.ad_value(226), s.ad_value(259))), (-0.3333333333333)), ((0.4 * p.p13) * 1.5412087));}
        s.store_add_scaled_product_indices(0, 256, 1.0, 23, 220, p.p14);s.store_sub_offset_lhs(2, 0, p.p34, 249);s.store_add_scaled_inputs4_indices(21, 183, p.p14, 237, p.p14, 243, p.p14, 2, 1.0);s.store_add_scaled_inputs4_indices(22, 184, p.p14, 237, p.p14, 244, p.p14, 0, 1.0);s.store_add_scaled_inputs4_indices(130, 185, p.p14, 237, p.p14, 243, p.p14, 2, 1.0);s.store_add_scaled_inputs4_indices(131, 186, p.p14, 237, p.p14, 244, p.p14, 0, 1.0);s.store_ln(295, 222);s.store_scaled_exp_ad(296, A::mul(s.ad_value(40), s.ad_value(295)), p.p35);s.store_mul(38, 191, 296);s.store_mul(39, 192, 296);s.store_exp_mul(297, 48, 295);s.store_mul(46, 193, 297);s.store_exp_mul(298, 49, 295);s.store_mul(47, 194, 298);s.store_exp_mul(299, 43, 295);s.store_mul(33, 195, 299);s.store_exp_mul(300, 45, 295);s.store_mul(44, 196, 300);s.store_exp_mul(301, 52, 295);s.store_mul(50, 197, 301);s.store_div_scaled_inputs_indices(0, 226, 1e-8, 14, 1.0);s.store_mul(267, 0, 46);s.store_primal_div_from_scalar_scaled_input(268, 1.0, 539, 0.5);s.store_primal_div(269, 268, 540);s.b[675] = (p.p14 == 1.0);s.store_scalar(675, if s.b[675] { 1.0 } else { 0.0 });
        if s.b[675] {s.store_primal_scale(270, 541, 0.5);}
        if (!s.b[675]) {s.store_primal_scale(270, 541, 0.3333333333333);}
        s.store_primal_sub_from_scalar(271, 1.0, 270);s.store_exp_mul(302, 55, 295);s.store_mul(53, 198, 302);s.store_scaled_mul(272, 53, 226, 2.0);s.store_primal_offset_ad(215, A::exp_scaled_input(A::ln(A::offset(A::exp_scaled_input(A::div_from_scalar(16.0, s.ad_value(542)), 0.6931471805599), (-1.0))), 0.375), (-1.0));s.store_primal_offset_ad(216, A::exp_scaled_input(A::ln(A::offset(A::exp_scaled_input(A::div_from_scalar(16.0, s.ad_value(543)), 0.6931471805599), (-1.0))), 0.375), (-1.0));s.store_exp_mul(303, 60, 295);s.store_mul3_lhs(59, 199, 303, 296);s.store_mul(273, 59, 226);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_mul3_lhs(147, 200, 303, 296);s.store_mul(274, 147, 226);s.store_mul(275, 64, 227);s.store_exp_mul_scaled_lhs_indices(304, 76, -1.0, 295);s.store_mul(68, 201, 304);s.store_mul(69, 202, 304);s.store_mul(70, 203, 304);s.store_mul(71, 204, 304);s.store_mul(72, 205, 304);s.store_exp_mul_scaled_lhs_indices(304, 77, -1.0, 295);s.store_mul(73, 206, 304);s.store_mul(74, 207, 304);s.store_primal_div_from_scalar(276, 1.0, 87);s.store_scaled_sqrt_scaled_input(277, 87, ((2.0 * 1.602176565e-19) * 9.10938291e-31), ((4.0 * 0.3333333333333) * 9.482522386533242e33));s.store_mul(278, 277, 18);s.store_mul(279, 277, 18);s.store_scalar(280, 0.0);s.b[676] = (s.v[79] < 0.0);s.store_scalar(676, if s.b[676] { 1.0 } else { 0.0 });
        if s.b[676] {s.store_primal_div_scaled_inputs_indices(280, 78, (-0.495), 79, 1.0);}
        s.store_scalar(281, 0.0);s.b[677] = (s.v[82] < 0.0);s.store_scalar(677, if s.b[677] { 1.0 } else { 0.0 });
        if s.b[677] {s.store_primal_div_scaled_inputs_indices(281, 80, (-0.495), 82, 1.0);}
        s.store_scalar(282, 0.0);s.b[678] = (s.v[84] < 0.0);s.store_scalar(678, if s.b[678] { 1.0 } else { 0.0 });
        if s.b[678] {s.store_primal_div_scaled_inputs_indices(282, 83, (-0.495), 84, 1.0);}
        s.store_scale(283, 233, 0.5);s.store_mul(284, 75, 226);s.store_mul(285, 75, 223);s.store_div_from_scalar_offset_product(286, 1.0, 88, 236, 1.0);s.store_div_from_scalar_square_ad(0, 4e-18, s.ad_value(18));s.store_mul(89, 89, 0);s.store_mul(90, 90, 0);s.store_scale(0, 18, 500000000.0);s.store_scaled_add_sqrt_square_offset_ad(277, A::offset(A::mul(s.ad_value(93), s.ad_value(220)), 1.0), 0.01, 0.5);s.store_mul3_lhs(91, 208, 277, 0);s.store_scaled_add_sqrt_square_offset_ad(277, A::offset(A::mul(s.ad_value(94), s.ad_value(220)), 1.0), 0.01, 0.5);s.store_mul3_lhs(92, 209, 277, 0);s.store_mul_exp_mixed_ia(113, 210, A::mul_scaled_lhs(s.ad_value(114), -1.0, s.ad_value(295)));s.store_mul_scale_offset_mixed_ia(288, 223, A::mul(s.ad_value(99), s.ad_value(222)), 1.0, 1.0);s.store_div_from_scalar(289, 1.0, 288);s.store_mul3_affine_lhs(290, 252, 229, (2.0 * 1.602176565e-19), 0.0, 289);s.store_add_scaled_product_indices(0, 256, 1.0, 102, 220, p.p14);s.store_sub_offset_lhs_mixed_ai(100, A::add_scaled_inputs4(s.ad_value(211), p.p14, s.ad_value(237), p.p14, s.ad_value(243), p.p14, s.ad_value(0), 1.0), p.p34, 249);s.store_add_scaled_inputs4_indices(101, 212, p.p14, 237, p.p14, 244, p.p14, 0, 1.0);s.store_scaled_exp_ad(0, A::mul(s.ad_value(111), s.ad_value(295)), p.p35);s.store_mul(110, 213, 0);s.store_mul(287, 116, 226);s.store_div_scaled_inputs_mixed_ia(291, 118, (0.25 * 1.602176565e-19), A::mul(s.ad_value(229), s.ad_value(226)), 1.0);s.store_ln_div(292, 118, 252);s.store_scaled_mul(293, 119, 226, 1.25e-6);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_15(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.store_primal_sqrt_ad(294, A::mul3_scaled_output(s.ad_value(229), s.ad_value(14), A::offset(s.ad_value(533), 4e-10), 1.0 / (3.45313e-11)));s.store_exp_mul(305, 169, 295);s.store_mul(168, 214, 305);let t19: f64 = (4.0 * 1.3806488e-23);let t1a: f64 = (t19 * s.v[217]);s.store_scalar(306, t1a);let t1b: f64 = (s.v[171] * s.v[306]);s.store_scalar(307, t1b);s.store_scalar(308, s.v[307]);let t1c: f64 = (9.10938291e-31 * 1000000000000.0);let t1d: f64 = (t1c * s.v[172]);s.store_scalar(309, t1d);s.b[679] = (s.v[311] > 0.0);s.store_scalar(679, if s.b[679] { 1.0 } else { 0.0 });
        if s.b[679] {s.store_primal_div_from_scalar(316, 1.0, 311);}
        if (!s.b[679]) {s.store_scalar(316, 0.0);}
        s.b[680] = (s.v[318] > 0.0);s.store_scalar(680, if s.b[680] { 1.0 } else { 0.0 });
        if s.b[680] {s.store_primal_div_from_scalar(320, 1.0, 318);}
        if (!s.b[680]) {s.store_scalar(320, 0.0);}
        s.b[681] = (s.v[322] > 0.0);s.store_scalar(681, if s.b[681] { 1.0 } else { 0.0 });
        if s.b[681] {s.store_primal_div_from_scalar(324, 1.0, 322);}
        if (!s.b[681]) {s.store_scalar(324, 0.0);}
        s.b[682] = (s.v[326] > 0.0);s.store_scalar(682, if s.b[682] { 1.0 } else { 0.0 });
        if s.b[682] {s.store_primal_div_from_scalar(327, 1.0, 326);}
        if (!s.b[682]) {s.store_scalar(327, 0.0);}
        s.b[785] = (s.v[6] > 0.0);s.store_scalar(785, if s.b[785] { 1.0 } else { 0.0 });
        if s.b[785] {s.store_voltage(219, ctx, nodes, Some(4), None);s.store_add(217, 8, 219);s.store_square(218, 217);s.store_offset(220, 217, (-s.v[7]));s.store_scale(221, 217, 1.0 / (s.v[7]));s.store_div_from_scalar(222, s.v[7], 217);s.store_scale(223, 217, 8.617332384961e-5);s.store_div_from_scalar(224, 1.0, 223);}
        s.b[786] = (p.p10 == 1.0);s.store_scalar(786, if s.b[786] { 1.0 } else { 0.0 });
        if (s.b[785] && s.b[786]) {s.store_scaled_add_offset_sqrt_square_offset_ad(225, A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), 600.0, (-600.0), 0.01, 0.5);}
        if (s.b[785] && (!s.b[786])) {s.store_scalar(225, 600.0);}
        if s.b[785] {s.store_sub_from_scalar_ad(230, 1.17, A::div_scaled_value_offset_denominator(s.ad_value(218), 0.000473, s.ad_value(217), 636.0, 1.0));s.store_sub_from_scalar_ad(231, 0.744, A::div_scaled_value_offset_denominator(s.ad_value(218), 0.0004774, s.ad_value(217), 235.0, 1.0));s.store_mul_add_scaled_inputs3_offset_rhs_indices(232, 15, 231, 1.0, 230, (-1.0), 228, (-0.4), 0.0);s.store_add(233, 230, 232);s.store_scaled_mul(234, 233, 224, 0.5);s.store_sub_scaled_inputs(237, 15, 0.05, 232, 0.5);s.store_sqrt_scaled_input(0, 217, 0.0033333333333);s.store_mul3_affine_lhs(2, 0, 0, 4.05e25, 0.0, 0);s.store_mul(252, 2, 238);s.store_mul_scale_offset_mixed_ia(226, 223, A::mul(s.ad_value(17), s.ad_value(222)), 1.0, 1.0);s.store_div_from_scalar(227, 1.0, 226);s.store_scaled_mul(236, 233, 227, 0.5);s.store_mul3_affine_lhs(253, 252, 229, (2.0 * 1.602176565e-19), 0.0, 227);s.store_offset_ln_ad(254, A::div_scaled_product(s.ad_value(245), s.ad_value(245), 1.0, s.ad_value(253), 1.0), (-0.6931471805599));s.store_mul_div_scaled_product_mixed_iiia(255, 227, 29, 14, (0.5 * 1.602176565e-19), A::add(s.ad_value(241), s.ad_value(242)), 1.0);s.store_mul(0, 34, 220);s.store_add(31, 187, 0);s.store_add(32, 188, 0);s.store_mul(329, 35, 227);s.store_add(140, 189, 0);s.store_add(141, 190, 0);}
    }
}
