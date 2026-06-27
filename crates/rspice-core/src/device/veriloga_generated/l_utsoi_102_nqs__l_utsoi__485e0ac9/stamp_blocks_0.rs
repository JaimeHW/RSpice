#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        s.v[7] = (273.15 + p.p15);

        s.v[0] = ((ctx_temp + p.p36)).min(1000.0);

        s.b[529] = (p.p10 == 1.0);
        s.v[529] = if s.b[529] { 1.0 } else { 0.0 };

        if s.b[529] {
            s.store_scalar(8, (0.5 * ((s.v[0] + (p.p17 + (p.p18 * s.v[0]))) + (((((s.v[0] - (p.p17 + (p.p18 * s.v[0]))) * (s.v[0] - (p.p17 + (p.p18 * s.v[0])))) + p.p19)) as f64).sqrt())));
            s.store_scaled_add_ad(225, A::offset(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), 600.0), A::sqrt(A::offset(A::mul_offset_lhs(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), (-600.0), A::offset(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), (-600.0))), 0.01)), 0.5);
        }

        if (!s.b[529]) {
            s.store_scalar(8, (0.5 * ((s.v[0] + 1.0) + (((((s.v[0] - 1.0) * (s.v[0] - 1.0)) + 0.001)) as f64).sqrt())));
            s.store_scalar(225, 600.0);
        }

        s.b[530] = (((p.p0 == 0.0) && (p.p172 > 0.0)) || ((p.p0 > 0.0) && (p.p443 > 0.0)));
        s.v[530] = if s.b[530] { 1.0 } else { 0.0 };

        let (assign80_e913,) = {
    if s.b[530] {
        (p.p5,)
    } else {
        (s.v[6],)
    }
};
        s.v[6] = assign80_e913;

        let (assign90_e918,) = {
    if (!s.b[530]) {
        (0.0,)
    } else {
        (s.v[6],)
    }
};
        s.v[6] = assign90_e918;

        s.v[219] = 0.0;

        s.copy_ad(217, 8);

        s.store_square(218, 217);

        s.store_offset(220, 217, (-s.v[7]));

        s.store_scale(221, 217, 1.0 / (s.v[7]));

        s.store_div_from_scalar(222, s.v[7], 217);

        s.store_scale(223, 217, 8.617332384961e-5);

        s.store_div_from_scalar(224, 1.0, 223);

        s.b[611] = (p.p0 == 0.0);
        s.v[611] = if s.b[611] { 1.0 } else { 0.0 };

        if s.b[611] {
            s.store_scalar(10, p.p23);
            s.store_scalar(9, p.p22);
            s.store_scalar(12, p.p25);
            s.store_scalar(11, p.p24);
            s.store_scalar(13, p.p30);
            s.store_scalar(533, p.p41);
            s.store_scalar(14, p.p42);
            s.store_scalar(15, p.p43);
            s.store_scalar(534, p.p44);
        }

        let (assign300_e986,) = {
    if s.b[611] {
        (1.0,)
    } else {
        (s.v[535],)
    }
};
        s.v[535] = assign300_e986;

        s.b[612] = (p.p45 < 0.0);
        s.v[612] = if s.b[612] { 1.0 } else { 0.0 };

        let (assign320_e996,) = {
    if (s.b[611] && s.b[612]) {
        let assign320_e994: f64 = (-1.0);
        (assign320_e994,)
    } else {
        (s.v[535],)
    }
};
        s.v[535] = assign320_e996;

        if s.b[611] {
            s.store_scalar(536, ((((p.p45) as f64).abs()).min(1e19) * 1000000.0));
            s.store_scalar(16, 1.0);
        }

        s.b[613] = (p.p46 < 0.0);
        s.v[613] = if s.b[613] { 1.0 } else { 0.0 };

        if (s.b[611] && s.b[613]) {
            s.store_scalar(16, (-1.0));
        }

        if s.b[611] {
            s.store_scalar(537, (((((p.p46) as f64).abs()).max(1e16)).min(1e21) * 1000000.0));
            s.store_scalar(17, p.p47);
            s.store_scalar(18, p.p48);
            s.store_scalar(19, (p.p49 * 1000000.0));
            s.store_scalar(20, (p.p50 * 1000000.0));
            s.store_scalar(183, p.p51);
            s.store_scalar(184, p.p52);
            s.store_scalar(23, p.p53);
            s.store_scalar(24, (p.p54 * 1000000.0));
            s.store_scalar(25, p.p55);
            s.store_scalar(26, p.p56);
            s.store_scalar(27, p.p57);
            s.store_div_scaled_product_indices(28, 27, 534, p.p58, 533, 1.0);
            s.store_scalar(29, (p.p59 * 1000000.0));
            s.store_scalar(30, p.p60);
            s.store_scalar(538, p.p61);
            s.store_scalar(187, p.p62);
            s.store_div_scaled_product_indices(188, 187, 534, p.p63, 533, 1.0);
            s.store_scalar(34, p.p64);
            s.store_scalar(35, p.p65);
            s.store_scalar(36, p.p66);
            s.store_scalar(37, p.p67);
            s.store_scalar(191, p.p68);
            s.store_scale(192, 191, p.p69);
            s.store_scalar(40, p.p70);
            s.store_scalar(195, p.p71);
            s.store_scalar(41, p.p72);
            s.store_scalar(42, p.p73);
            s.store_scalar(43, p.p74);
            s.store_scalar(196, p.p75);
            s.store_scalar(45, p.p76);
            s.store_scalar(539, p.p77);
            s.store_scalar(540, p.p78);
            s.store_scalar(193, p.p79);
            s.store_scalar(48, p.p80);
            s.store_scalar(194, p.p81);
            s.store_scalar(49, p.p82);
            s.store_scalar(197, p.p83);
            s.store_scalar(51, p.p84);
            s.store_scalar(52, p.p85);
            s.store_scalar(541, p.p86);
            s.store_scalar(198, p.p87);
            s.store_scalar(54, p.p88);
            s.store_scalar(55, p.p89);
            s.store_scalar(56, p.p90);
            s.store_scalar(57, p.p91);
            s.store_scalar(58, p.p92);
            s.store_scalar(199, p.p93);
            s.store_scalar(60, p.p94);
            s.store_scalar(61, p.p95);
            s.store_scalar(62, p.p96);
            s.store_scalar(542, p.p97);
            s.store_scalar(63, p.p98);
            s.store_scalar(64, p.p99);
            s.store_scalar(65, p.p100);
            s.store_scalar(66, p.p101);
            s.store_scalar(67, p.p102);
            s.store_scalar(75, p.p103);
            s.store_scalar(201, p.p104);
            s.store_scalar(202, p.p105);
            s.store_scalar(203, p.p106);
            s.store_scalar(206, p.p120);
            s.store_scalar(207, p.p121);
            s.store_scalar(204, p.p107);
            s.store_scalar(205, p.p108);
            s.store_scalar(76, p.p109);
            s.store_scalar(77, p.p123);
            s.store_scalar(78, p.p110);
            s.store_scalar(79, p.p111);
            s.store_scalar(80, p.p112);
            s.store_scalar(81, p.p122);
            s.store_scalar(82, p.p113);
            s.store_scalar(83, p.p114);
            s.store_scalar(84, p.p115);
            s.store_scalar(85, p.p116);
            s.store_scalar(86, p.p117);
            s.store_scalar(87, p.p118);
            s.store_scalar(88, p.p119);
            s.store_scalar(89, p.p124);
            s.store_scalar(90, p.p125);
            s.store_scalar(208, p.p126);
            s.store_scalar(209, p.p127);
            s.store_scalar(93, p.p128);
            s.store_scalar(94, p.p129);
            s.store_scalar(95, p.p130);
            s.store_scalar(96, p.p131);
            s.store_scalar(97, p.p132);
            s.store_scalar(98, p.p133);
            s.store_scalar(112, p.p147);
            s.store_scalar(210, p.p148);
            s.store_scalar(114, p.p149);
            s.store_scalar(115, p.p150);
            s.store_scalar(99, p.p134);
            s.store_scalar(211, p.p135);
        }

    }

    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[611] {
            s.store_scalar(212, p.p136);
            s.store_scalar(102, p.p137);
            s.store_scalar(103, p.p138);
            s.store_scalar(104, p.p139);
            s.store_scalar(105, p.p140);
            s.store_div_scaled_product_indices(106, 105, 534, p.p141, 533, 1.0);
            s.store_scalar(107, p.p142);
            s.store_div_scaled_product_indices(108, 107, 534, p.p143, 533, 1.0);
            s.store_scalar(109, p.p144);
            s.store_scalar(213, p.p145);
            s.store_scalar(111, p.p146);
            s.store_scalar(116, p.p151);
            s.store_scalar(117, p.p152);
            s.store_scalar(118, (p.p153 * 1000000.0));
            s.store_scalar(119, p.p154);
            s.store_scalar(120, p.p155);
            s.copy_ad(185, 183);
            s.copy_ad(186, 184);
            s.copy_ad(135, 27);
            s.copy_ad(136, 28);
            s.copy_ad(189, 187);
            s.copy_ad(190, 188);
            s.copy_ad(200, 199);
            s.copy_ad(543, 542);
            s.copy_ad(158, 63);
        }

        s.b[614] = (p.p11 > 0.0);
        s.v[614] = if s.b[614] { 1.0 } else { 0.0 };

        if (s.b[611] && s.b[614]) {
            s.store_scalar(185, p.p51);
        }

        s.b[615] = param_given[156];
        s.v[615] = if s.b[615] { 1.0 } else { 0.0 };

        if ((s.b[611] && s.b[614]) && s.b[615]) {
            s.store_scalar(185, p.p156);
        }

        if (s.b[611] && s.b[614]) {
            s.store_scalar(186, p.p52);
        }

        s.b[616] = param_given[157];
        s.v[616] = if s.b[616] { 1.0 } else { 0.0 };

        if ((s.b[611] && s.b[614]) && s.b[616]) {
            s.store_scalar(186, p.p157);
        }

        if (s.b[611] && s.b[614]) {
            s.store_scalar(135, p.p57);
        }

        s.b[617] = param_given[158];
        s.v[617] = if s.b[617] { 1.0 } else { 0.0 };

        if ((s.b[611] && s.b[614]) && s.b[617]) {
            s.store_scalar(135, p.p158);
        }

        if (s.b[611] && s.b[614]) {
            s.store_div_scaled_product_indices(136, 135, 534, p.p58, 533, 1.0);
            s.store_scalar(189, p.p62);
        }

        s.b[618] = param_given[159];
        s.v[618] = if s.b[618] { 1.0 } else { 0.0 };

        if ((s.b[611] && s.b[614]) && s.b[618]) {
            s.store_scalar(189, p.p159);
        }

        if (s.b[611] && s.b[614]) {
            s.store_div_scaled_product_indices(190, 189, 534, p.p63, 533, 1.0);
            s.store_scalar(200, p.p93);
        }

        s.b[619] = param_given[160];
        s.v[619] = if s.b[619] { 1.0 } else { 0.0 };

        if ((s.b[611] && s.b[614]) && s.b[619]) {
            s.store_scalar(200, p.p160);
        }

        if (s.b[611] && s.b[614]) {
            s.store_scalar(543, p.p97);
        }

        s.b[620] = param_given[161];
        s.v[620] = if s.b[620] { 1.0 } else { 0.0 };

        if ((s.b[611] && s.b[614]) && s.b[620]) {
            s.store_scalar(543, p.p161);
        }

        if (s.b[611] && s.b[614]) {
            s.store_scalar(158, p.p98);
        }

        s.b[621] = param_given[162];
        s.v[621] = if s.b[621] { 1.0 } else { 0.0 };

        if ((s.b[611] && s.b[614]) && s.b[621]) {
            s.store_scalar(158, p.p162);
        }

        if s.b[611] {
            s.store_scalar(159, p.p163);
            s.store_scalar(160, p.p164);
            s.store_scalar(161, p.p165);
            s.store_scalar(162, p.p166);
            s.store_scalar(163, p.p167);
            s.store_scalar(164, p.p168);
            s.store_scalar(165, p.p169);
            s.store_scalar(166, p.p170);
            s.store_scalar(167, p.p171);
            s.store_scalar(214, p.p172);
            s.store_scalar(169, p.p173);
            s.store_scalar(170, p.p174);
        }

        let (assign1920_e1743,) = {
    if s.b[611] {
        (p.p175,)
    } else {
        (s.v[171],)
    }
};
        s.v[171] = assign1920_e1743;

        let (assign1930_e1747,) = {
    if s.b[611] {
        (p.p176,)
    } else {
        (s.v[172],)
    }
};
        s.v[172] = assign1930_e1747;

        if s.b[611] {
            s.store_scalar(173, p.p177);
            s.store_scalar(174, p.p178);
            s.store_scalar(175, p.p179);
            s.store_scalar(176, p.p180);
            s.store_scalar(177, p.p181);
            s.store_scalar(179, p.p183);
            s.store_scalar(180, p.p184);
            s.store_scalar(181, p.p185);
            s.store_scalar(182, p.p186);
        }

        if (!s.b[611]) {
            s.store_scalar(588, (1.0 / p.p29));
            s.store_max_with_scalar_ad(532, A::scale(s.ad_value(588), p.p21), 1e-9);
            s.store_scale(10, 588, p.p23);
            s.store_scale(9, 588, p.p22);
            s.store_scale(12, 588, p.p25);
            s.store_scale(11, 588, p.p24);
            s.store_scalar(13, (p.p30 * p.p29));
            s.store_scalar(569, 1e-6);
            s.store_scalar(570, 1e-6);
            s.store_scale(571, 569, 1.0 / (p.p20));
            s.store_div(572, 570, 532);
            s.store_scaled_mul_scale_offset_inputs(573, 571, p.p192, 1.0, 572, p.p193, 1.0, p.p191);
            s.store_scaled_mul_scale_offset_inputs(574, 572, p.p197, 1.0, 571, p.p196, 1.0, p.p195);
            s.store_max_with_scalar_ad(575, A::offset(s.ad_value(573), ((p.p20) + ((-(2.0 * p.p194))))), 1e-9);
            s.store_max_with_scalar_ad(576, A::offset(A::add(s.ad_value(532), s.ad_value(574)), (-(2.0 * p.p198))), 1e-9);
            s.store_max_with_scalar_ad(577, A::offset(s.ad_value(573), ((((p.p20) + ((-(2.0 * p.p194))))) + (p.p199))), 1e-9);
            s.store_max_with_scalar_ad(578, A::offset(A::add(s.ad_value(532), s.ad_value(574)), (((-(2.0 * p.p198))) + (p.p200))), 1e-9);
            s.store_div(579, 569, 575);
            s.store_div(580, 570, 576);
            s.store_mul(581, 579, 580);
            s.store_max_with_scalar_ad(0, A::offset(s.ad_value(573), p.p20), 1e-9);
            s.store_div(582, 0, 569);
            s.store_max_with_scalar_ad(0, A::add(s.ad_value(532), s.ad_value(574)), 1e-9);
            s.store_div(583, 0, 570);
            s.store_scalar(533, p.p201);
            s.store_scalar(14, p.p202);
            s.store_scalar(15, p.p203);
            s.store_scalar(534, p.p204);
        }

        let (assign2400_e2084,) = {
    if (!s.b[611]) {
        (1.0,)
    } else {
        (s.v[535],)
    }
};
        s.v[535] = assign2400_e2084;

        s.b[622] = (p.p205 < 0.0);
        s.v[622] = if s.b[622] { 1.0 } else { 0.0 };

        let (assign2420_e2095,) = {
    if ((!s.b[611]) && s.b[622]) {
        let assign2420_e2093: f64 = (-1.0);
        (assign2420_e2093,)
    } else {
        (s.v[535],)
    }
};
        s.v[535] = assign2420_e2095;

        if (!s.b[611]) {
            s.store_scalar(536, ((((p.p205) as f64).abs()).min(1e19) * 1000000.0));
            s.store_scalar(16, 1.0);
        }

        s.b[623] = (p.p206 < 0.0);
        s.v[623] = if s.b[623] { 1.0 } else { 0.0 };

        if ((!s.b[611]) && s.b[623]) {
            s.store_scalar(16, (-1.0));
        }

        if (!s.b[611]) {
            s.store_scalar(537, (((((p.p206) as f64).abs()).max(1e16)).min(1e21) * 1000000.0));
            s.store_scalar(17, p.p207);
            s.store_scalar(18, p.p208);
            s.store_scalar(19, (p.p209 * 1000000.0));
            s.store_scalar(20, (p.p210 * 1000000.0));
            s.store_div_scaled_inputs(0, A::powf(s.ad_value(579), p.p213), p.p212, A::scale_offset(A::powf(s.ad_value(579), p.p215), p.p214, 1.0), 1.0);
            s.store_add_scaled_inputs3_offset_indices(183, 0, 1.0, 580, p.p216, 581, p.p217, p.p211);
            s.store_offset_mul_ad(184, A::div_scaled_inputs(s.ad_value(534), p.p219, s.ad_value(533), 1.0), s.ad_value(0), p.p218);
            s.store_mul3_ad_scaled_output(23, A::scale_offset(s.ad_value(579), p.p221, 1.0), A::scale_offset(s.ad_value(580), p.p222, 1.0), A::scale_offset(s.ad_value(581), p.p223, 1.0), p.p220);
            s.store_offset_scaled(607, 579, ((p.p225) * ((p.p224 * 1000000.0))), (p.p224 * 1000000.0));
            s.store_min_with_scalar_ad(24, A::max_with_scalar(s.ad_value(607), 1e25), 1e28);
            s.store_scalar(25, p.p226);
            s.store_scalar(26, p.p227);
            s.store_sub_from_scalar(228, 1.0, 15);
            s.store_add_scaled_inputs(229, 228, 1.04479e-10, 15, 1.43438e-10);
            s.store_div_ad_lhs(584, A::sqrt(A::mul3_scaled_output(s.ad_value(229), s.ad_value(14), A::offset(s.ad_value(533), 4e-10), 1.0 / (3.45313e-11))), 575);
            s.store_scaled_mul_scale_offset_rhs_ad(544, A::powf(s.ad_value(584), p.p229), 580, p.p230, 1.0, (p.p228 * 2.0));
            s.store_min_with_scalar_ad(27, A::max_with_scalar(s.ad_value(544), 0.0), 5.0);
        }

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[611]) {
            s.store_div_scaled_product_indices(28, 27, 534, p.p231, 533, 1.0);
            s.store_scalar(29, (p.p232 * 1000000.0));
            s.store_scalar(30, p.p233);
            s.store_scale(549, 580, p.p234);
            s.store_min_with_scalar_ad(538, A::max_with_scalar(s.ad_value(549), (-1.0)), 1.0);
            s.store_mul_ad(0, A::powf(s.ad_value(584), p.p236), A::scale_offset(s.ad_value(580), p.p237, 1.0));
            s.store_scale(546, 0, p.p235);
            s.store_max_with_scalar(187, 546, 0.0);
            s.store_div_scaled_product_indices(188, 187, 534, p.p238, 533, 1.0);
            s.store_scale(34, 0, p.p239);
            s.store_scalar(35, p.p240);
            s.store_div_scaled_inputs_mixed_ia(36, 579, p.p241, A::max_with_scalar(A::scale_offset(s.ad_value(580), p.p242, 1.0), 0.001), 1.0);
            s.store_scalar(37, p.p243);
            s.store_div_scaled_inputs_mixed_ia(2, 575, -1.0, A::max_with_scalar(A::scale_offset(s.ad_value(580), p.p248, 1.0), 0.001), p.p247);
        }

        s.b[624] = (s.v[2] > (-80.0));
        s.v[624] = if s.b[624] { 1.0 } else { 0.0 };

        if ((!s.b[611]) && s.b[624]) {
            s.store_exp(3, 2);
        }

        if ((!s.b[611]) && (!s.b[624])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(3, 1.80485e-35, A::neg(s.ad_value(2)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(2)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (!s.b[611]) {
            s.store_scale(4, 575, (-1.0 / (p.p250)));
        }

        s.b[625] = (s.v[4] > (-80.0));
        s.v[625] = if s.b[625] { 1.0 } else { 0.0 };

        if ((!s.b[611]) && s.b[625]) {
            s.store_exp(5, 4);
        }

        if ((!s.b[611]) && (!s.b[625])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(4)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (!s.b[611]) {
            s.store_max_with_scalar_ad(585, A::add(A::offset(A::div_scaled_product_offset_rhs(A::scale_offset(s.ad_value(580), p.p246, 1.0), s.ad_value(3), (-1.0), p.p245, s.ad_value(2), 1.0), 1.0), A::div_scaled_offset_numerator(s.ad_value(5), p.p249, ((-1.0) * p.p249), s.ad_value(4), 1.0)), 1e-6);
            s.store_max_with_scalar_ad(586, A::add_scaled_product(A::scale_offset(s.ad_value(580), p.p251, 1.0), 1.0, s.ad_value(580), A::ln(A::scale_offset(s.ad_value(576), 1.0 / (p.p253), 1.0)), p.p252), 1e-6);
            s.store_mul_div_from_scalar_lhs(587, p.p244, 585, 586);
            s.store_div_scaled_product_indices(548, 587, 576, 1.0, 575, 1.0);
            s.store_max_with_scalar(191, 548, 1e-10);
            s.store_scale(192, 191, p.p254);
            s.store_mul3_ad_scaled_output(40, A::scale_offset(s.ad_value(579), p.p256, 1.0), A::scale_offset(s.ad_value(580), p.p257, 1.0), A::scale_offset(s.ad_value(581), p.p258, 1.0), p.p255);
            s.store_mul3_ad(550, A::scale_offset(A::powf(s.ad_value(579), p.p261), p.p260, p.p259), A::scale_offset(s.ad_value(580), p.p262, 1.0), A::scale_offset(s.ad_value(581), p.p263, 1.0));
            s.store_max_with_scalar(195, 550, 0.0);
            s.store_scalar(41, p.p264);
            s.store_scalar(42, p.p265);
            s.store_mul3_ad_scaled_output(43, A::scale_offset(s.ad_value(579), p.p267, 1.0), A::scale_offset(s.ad_value(580), p.p268, 1.0), A::scale_offset(s.ad_value(581), p.p269, 1.0), p.p266);
            s.store_scalar(196, p.p270);
            s.store_scalar(45, p.p271);
            s.store_scalar(539, p.p272);
            s.store_scalar(540, p.p273);
            s.store_scalar(193, p.p274);
            s.store_scalar(48, p.p275);
            s.store_scalar(194, p.p276);
            s.store_scalar(49, p.p277);
            s.store_mul3_ad(197, A::scale_offset(A::powf(s.ad_value(579), p.p280), p.p279, p.p278), A::scale_offset(s.ad_value(580), p.p281, 1.0), A::scale_offset(s.ad_value(581), p.p282, 1.0));
            s.store_scalar(51, p.p283);
            s.store_scalar(52, p.p284);
            s.store_scalar(541, p.p285);
            s.store_mul_scale_offset_rhs(551, 580, 580, ((p.p287) * (p.p286)), p.p286);
            s.store_max_with_scalar(198, 551, 0.0);
            s.store_scalar(54, p.p288);
            s.store_scalar(55, p.p289);
            s.store_scalar(56, p.p290);
            s.store_scalar(57, p.p291);
            s.store_scalar(58, p.p292);
            s.store_mul_ad(552, A::mul3(s.ad_value(587), A::scale_offset(A::powf(s.ad_value(579), p.p295), p.p294, p.p293), A::scale_offset(s.ad_value(580), p.p296, 1.0)), A::scale_offset(s.ad_value(581), p.p297, 1.0));
            s.store_max_with_scalar(199, 552, 0.0);
            s.store_mul3_ad_scaled_output(60, A::scale_offset(s.ad_value(579), p.p299, 1.0), A::scale_offset(s.ad_value(580), p.p300, 1.0), A::scale_offset(s.ad_value(581), p.p301, 1.0), p.p298);
            s.store_scalar(61, p.p302);
            s.store_scalar(62, p.p303);
            s.store_div_from_scalar_offset_ad(554, p.p304, A::div_scaled_inputs(A::powf(s.ad_value(579), p.p306), p.p305, A::scale_offset(A::powf(s.ad_value(579), p.p308), p.p307, 1.0), 1.0), 1.0);
            s.store_min_with_scalar_ad(542, A::max_with_scalar(s.ad_value(554), 1.0), 16.0);
            s.store_div_scaled_product(557, A::powf(s.ad_value(579), p.p310), A::scale_offset(s.ad_value(580), p.p313, 1.0), p.p309, A::scale_offset(A::powf(s.ad_value(579), p.p312), p.p311, 1.0), 1.0);
            s.store_max_with_scalar(63, 557, 0.0);
            s.store_div_scaled_product(558, A::powf(s.ad_value(579), p.p315), A::scale_offset(s.ad_value(580), p.p318, 1.0), p.p314, A::scale_offset(A::powf(s.ad_value(579), p.p317), p.p316, 1.0), 1.0);
            s.store_max_with_scalar(64, 558, 0.0);
            s.store_scalar(65, p.p319);
            s.store_scalar(66, p.p320);
            s.store_scalar(67, p.p321);
            s.store_scalar(75, p.p322);
            s.store_div_from_scalar(201, p.p323, 581);
            s.store_div_from_scalar(202, p.p324, 580);
            s.store_div_from_scalar(203, p.p325, 580);
            s.store_div_from_scalar(206, p.p339, 580);
            s.store_div_from_scalar(207, p.p340, 580);
            s.store_div_from_scalar(204, p.p326, 580);
            s.store_div_from_scalar(205, p.p327, 580);
            s.store_scalar(76, p.p328);
            s.store_scalar(77, p.p342);
            s.store_scalar(78, p.p329);
            s.store_scalar(79, p.p330);
            s.store_scalar(80, p.p331);
            s.store_scalar(81, p.p341);
            s.store_scalar(82, p.p332);
            s.store_scalar(83, p.p333);
            s.store_scalar(84, p.p334);
            s.store_scale(85, 579, p.p335);
            s.store_scalar(86, p.p336);
            s.store_scalar(87, p.p337);
            s.store_scalar(88, p.p338);
            s.store_offset_div_from_scalar_ad(559, p.p345, s.ad_value(580), p.p343);
            s.store_max_with_scalar(89, 559, 0.0);
            s.store_offset_div_from_scalar_ad(560, p.p346, s.ad_value(580), p.p344);
            s.store_max_with_scalar(90, 560, 0.0);
            s.store_scalar(208, p.p347);
            s.store_scalar(209, p.p348);
            s.store_scalar(93, p.p349);
            s.store_scalar(94, p.p350);
            s.store_scalar(95, p.p351);
            s.store_scalar(96, p.p352);
            s.store_offset_scaled(97, 579, p.p355, p.p353);
            s.store_offset_scaled(98, 579, p.p356, p.p354);
            s.store_scaled_mul_scale_offset_inputs(561, 579, p.p389, 1.0, 580, p.p390, 1.0, p.p388);
            s.store_max_with_scalar(112, 561, 0.0);
            s.store_scalar(210, p.p391);
            s.store_scalar(114, p.p392);
            s.store_scaled_mul_scale_offset_inputs(562, 579, p.p394, 1.0, 580, p.p395, 1.0, p.p393);
            s.store_max_with_scalar(115, 562, 0.0);
            s.store_offset_scaled(589, 576, p.p358, (2.0 * p.p357));
            s.store_scalar(99, p.p359);
            s.store_scale_ad(0, A::powf(s.ad_value(579), p.p362), p.p361);
            s.store_add_scaled_inputs3_offset_indices(211, 0, 1.0, 580, p.p363, 581, p.p364, p.p360);
            s.store_scalar(212, p.p365);
            s.store_mul3_ad_scaled_output(102, A::scale_offset(s.ad_value(579), p.p367, 1.0), A::scale_offset(s.ad_value(580), p.p368, 1.0), A::scale_offset(s.ad_value(581), p.p369, 1.0), p.p366);
            s.store_scalar(103, p.p370);
            s.store_scalar(104, p.p371);
            s.store_scaled_mul_scale_offset_rhs_ad(0, A::powf(s.ad_value(584), p.p373), 580, p.p374, 1.0, (p.p372 * 2.0));
            s.store_min_with_scalar_ad(105, A::max_with_scalar(s.ad_value(0), 0.0), 5.0);
            s.store_div_scaled_product_indices(106, 105, 534, p.p375, 533, 1.0);
            s.store_mul_ad(0, A::powf(s.ad_value(584), p.p377), A::scale_offset(s.ad_value(580), p.p378, 1.0));
            s.store_scale(0, 0, p.p376);
            s.store_max_with_scalar(107, 0, 0.0);
            s.store_div_scaled_product_indices(108, 107, 534, p.p379, 533, 1.0);
            s.store_scalar(109, p.p380);
            s.store_offset_ad(0, A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p381 * p.p382), s.ad_value(575)), 1.0, A::exp_scaled_input(s.ad_value(575), (-1.0 / (p.p382)))), 1.0);
            s.store_max_with_scalar(0, 0, 1e-15);
            s.store_mul_ad(213, A::div_scaled_inputs(s.ad_value(589), p.p244, A::mul(s.ad_value(0), s.ad_value(575)), 1.0), A::scale_offset(s.ad_value(580), p.p383, 1.0));
            s.store_add_scaled_inputs_product_first_ad(111, A::scale_offset(s.ad_value(579), p.p385, p.p384), 1.0, 580, p.p386, 579, 580, p.p387);
            s.store_mul(116, 578, 577);
            s.store_offset_scaled(563, 582, p.p397, p.p396);
            s.store_max_with_scalar(117, 563, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (!s.b[611]) {
            s.store_scalar(118, (p.p398 * 1000000.0));
            s.store_div_scaled_inputs_indices(119, 578, p.p399, 570, 1.0);
            s.store_scalar(120, p.p400);
            s.copy_ad(185, 183);
            s.copy_ad(186, 184);
            s.copy_ad(135, 27);
            s.copy_ad(136, 28);
            s.copy_ad(547, 546);
            s.copy_ad(189, 187);
            s.copy_ad(190, 188);
            s.copy_ad(553, 552);
            s.copy_ad(200, 199);
            s.copy_ad(543, 542);
            s.copy_ad(158, 63);
        }

        s.b[626] = (p.p11 > 0.0);
        s.v[626] = if s.b[626] { 1.0 } else { 0.0 };

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(121, p.p211);
        }

        s.b[627] = param_given[401];
        s.v[627] = if s.b[627] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[627]) {
            s.store_scalar(121, p.p401);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(122, p.p212);
        }

        s.b[628] = param_given[402];
        s.v[628] = if s.b[628] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[628]) {
            s.store_scalar(122, p.p402);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(123, p.p213);
        }

        s.b[629] = param_given[403];
        s.v[629] = if s.b[629] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[629]) {
            s.store_scalar(123, p.p403);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(124, p.p216);
        }

        s.b[630] = param_given[406];
        s.v[630] = if s.b[630] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[630]) {
            s.store_scalar(124, p.p406);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(125, p.p217);
        }

        s.b[631] = param_given[407];
        s.v[631] = if s.b[631] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[631]) {
            s.store_scalar(125, p.p407);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(126, p.p214);
        }

        s.b[632] = param_given[404];
        s.v[632] = if s.b[632] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[632]) {
            s.store_scalar(126, p.p404);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(127, p.p215);
        }

        s.b[633] = param_given[405];
        s.v[633] = if s.b[633] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[633]) {
            s.store_scalar(127, p.p405);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_div_scaled_product_offset_denominator(0, s.ad_value(122), A::pow(s.ad_value(579), s.ad_value(123)), 1.0, A::mul(s.ad_value(126), A::pow(s.ad_value(579), s.ad_value(127))), 1.0, 1.0);
            s.store_add_scaled_inputs_products_indices(185, 121, 1.0, 0, 1.0, 124, 580, 1.0, 125, 581, 1.0);
            s.store_scalar(128, p.p218);
        }

        s.b[634] = param_given[408];
        s.v[634] = if s.b[634] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[634]) {
            s.store_scalar(128, p.p408);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(129, p.p219);
        }

        s.b[635] = param_given[409];
        s.v[635] = if s.b[635] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[635]) {
            s.store_scalar(129, p.p409);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_add_scaled_product_left_ad(186, 128, 1.0, A::div_scaled_product(s.ad_value(129), s.ad_value(534), 1.0, s.ad_value(533), 1.0), 0, 1.0);
            s.store_scalar(132, p.p228);
        }

        s.b[636] = param_given[410];
        s.v[636] = if s.b[636] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[636]) {
            s.store_scalar(132, p.p410);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(133, p.p229);
        }

        s.b[637] = param_given[411];
        s.v[637] = if s.b[637] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[637]) {
            s.store_scalar(133, p.p411);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(134, p.p230);
        }

        s.b[638] = param_given[412];
        s.v[638] = if s.b[638] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[638]) {
            s.store_scalar(134, p.p412);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_mul_ad_affine_product_rhs(545, 132, A::pow(s.ad_value(584), s.ad_value(133)), A::offset(A::mul(s.ad_value(134), s.ad_value(580)), 1.0), 2.0, 0.0);
            s.store_min_with_scalar_ad(135, A::max_with_scalar(s.ad_value(545), 0.0), 5.0);
            s.store_div_scaled_product_indices(136, 135, 534, p.p231, 533, 1.0);
            s.store_scalar(137, p.p235);
        }

        s.b[639] = param_given[413];
        s.v[639] = if s.b[639] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[639]) {
            s.store_scalar(137, p.p413);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(138, p.p236);
        }

        s.b[640] = param_given[414];
        s.v[640] = if s.b[640] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[640]) {
            s.store_scalar(138, p.p414);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(139, p.p237);
        }

        s.b[641] = param_given[415];
        s.v[641] = if s.b[641] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[641]) {
            s.store_scalar(139, p.p415);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_mul_offset_rhs_ad(0, A::pow(s.ad_value(584), s.ad_value(138)), A::mul(s.ad_value(139), s.ad_value(580)), 1.0);
            s.store_mul(547, 137, 0);
            s.store_max_with_scalar(189, 547, 0.0);
            s.store_div_scaled_product_indices(190, 189, 534, p.p238, 533, 1.0);
            s.store_scalar(142, p.p293);
        }

        s.b[642] = param_given[416];
        s.v[642] = if s.b[642] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[642]) {
            s.store_scalar(142, p.p416);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(143, p.p294);
        }

        s.b[643] = param_given[417];
        s.v[643] = if s.b[643] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[643]) {
            s.store_scalar(143, p.p417);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(144, p.p295);
        }

        s.b[644] = param_given[418];
        s.v[644] = if s.b[644] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[644]) {
            s.store_scalar(144, p.p418);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(145, p.p296);
        }

        s.b[645] = param_given[419];
        s.v[645] = if s.b[645] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[645]) {
            s.store_scalar(145, p.p419);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(146, p.p297);
        }

        s.b[646] = param_given[420];
        s.v[646] = if s.b[646] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[646]) {
            s.store_scalar(146, p.p420);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_mul_offset_rhs_ad(553, A::mul3(s.ad_value(587), A::add_scaled_product(s.ad_value(142), 1.0, s.ad_value(143), A::pow(s.ad_value(579), s.ad_value(144)), 1.0), A::offset(A::mul(s.ad_value(145), s.ad_value(580)), 1.0)), A::mul(s.ad_value(146), s.ad_value(581)), 1.0);
            s.store_max_with_scalar(200, 553, 0.0);
            s.store_scalar(148, p.p304);
        }

        s.b[647] = param_given[421];
        s.v[647] = if s.b[647] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[647]) {
            s.store_scalar(148, p.p421);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(149, p.p305);
        }

        s.b[648] = param_given[422];
        s.v[648] = if s.b[648] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[648]) {
            s.store_scalar(149, p.p422);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(150, p.p306);
        }

        s.b[649] = param_given[423];
        s.v[649] = if s.b[649] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[649]) {
            s.store_scalar(150, p.p423);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(151, p.p307);
        }

        s.b[650] = param_given[424];
        s.v[650] = if s.b[650] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[650]) {
            s.store_scalar(151, p.p424);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(152, p.p308);
        }

        s.b[651] = param_given[425];
        s.v[651] = if s.b[651] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[651]) {
            s.store_scalar(152, p.p425);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_div_scaled_value_offset_denominator(555, s.ad_value(148), 1.0, A::div_scaled_product_offset_denominator(s.ad_value(149), A::pow(s.ad_value(579), s.ad_value(150)), 1.0, A::mul(s.ad_value(151), A::pow(s.ad_value(579), s.ad_value(152))), 1.0, 1.0), 1.0, 1.0);
            s.store_min_with_scalar_ad(543, A::max_with_scalar(s.ad_value(555), 1.0), 16.0);
            s.store_scalar(153, p.p309);
        }

        s.b[652] = param_given[426];
        s.v[652] = if s.b[652] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[652]) {
            s.store_scalar(153, p.p426);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(154, p.p310);
        }

        s.b[653] = param_given[427];
        s.v[653] = if s.b[653] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[653]) {
            s.store_scalar(154, p.p427);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(155, p.p311);
        }

        s.b[654] = param_given[428];
        s.v[654] = if s.b[654] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[654]) {
            s.store_scalar(155, p.p428);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(156, p.p312);
        }

        s.b[655] = param_given[429];
        s.v[655] = if s.b[655] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[655]) {
            s.store_scalar(156, p.p429);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_scalar(157, p.p313);
        }

        s.b[656] = param_given[430];
        s.v[656] = if s.b[656] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[626]) && s.b[656]) {
            s.store_scalar(157, p.p430);
        }

        if ((!s.b[611]) && s.b[626]) {
            s.store_div_scaled_product3_mixed_iaaa(556, 153, A::pow(s.ad_value(579), s.ad_value(154)), A::offset(A::mul(s.ad_value(157), s.ad_value(580)), 1.0), 1.0, A::offset(A::mul(s.ad_value(155), A::pow(s.ad_value(579), s.ad_value(156))), 1.0), 1.0);
            s.store_max_with_scalar(158, 556, 0.0);
        }

        if (!s.b[611]) {
            s.store_mul_div_from_scalar_lhs(0, 3.45313e-11, 533, 578);
            s.store_scale(159, 0, p.p431);
            s.store_scale(160, 0, p.p432);
            s.store_div_from_scalar_ad(161, p.p433, A::max_with_scalar(A::offset(A::div_scaled_inputs(s.ad_value(570), p.p434, s.ad_value(578), 1.0), 1.0), 0.001));
            s.store_scalar(162, p.p435);
            s.store_scalar(163, p.p436);
            s.store_offset_scaled(564, 583, p.p439, p.p437);
            s.store_max_with_scalar(164, 564, 0.0);
            s.store_offset_scaled(565, 583, p.p440, p.p438);
            s.store_max_with_scalar(165, 565, 0.0);
            s.store_div_scaled_product3_indices(166, 229, 14, 576, p.p441, 575, 1.0);
            s.store_scalar(167, p.p442);
            s.store_max_with_scalar_ad(0, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(582), p.p444, 1.0), 1.0, s.ad_value(583), p.p445, s.ad_value(582), s.ad_value(583), p.p446), 1e-10);
            s.store_scalar(2, 0.0);
        }

        s.b[657] = ((p.p29 > 1.0) && (p.p28 > 0.0));
        s.v[657] = if s.b[657] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[611]) && s.b[657]) {
            s.store_scalar(3, ((-(p.p28 + p.p20)) / p.p449));
        }

        s.b[658] = (((s.v[3]) as f64).abs() < 80.0);
        s.v[658] = if s.b[658] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[657]) && s.b[658]) {
            s.store_exp(4, 3);
        }

        s.b[659] = (s.v[3] < (-80.0));
        s.v[659] = if s.b[659] { 1.0 } else { 0.0 };

        if ((((!s.b[611]) && s.b[657]) && (!s.b[658])) && s.b[659]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(4, 1.80485e-35, A::neg(s.ad_value(3)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(3)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(3)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((((!s.b[611]) && s.b[657]) && (!s.b[658])) && (!s.b[659])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(4, 3, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(3), (-80.0)), 0.5, A::scale_offset(s.ad_value(3), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if ((!s.b[611]) && s.b[657]) {
            s.store_sub_from_scalar(5, 1.0, 4);
            s.store_div_scaled_product_mixed_iaa(2, 4, A::sub(s.ad_value(5), A::scale_offset(A::powf(s.ad_value(4), p.p29), (-1.0 / (p.p29)), 1.0 / (p.p29))), (2.0 * p.p450), A::square(s.ad_value(5)), 1.0);
        }

        if (!s.b[611]) {
            s.store_div_scaled_value_offset_denominator(0, s.ad_value(0), 1.0, s.ad_value(2), 1.0, 1.0);
            s.store_div_from_scalar(566, p.p443, 0);
            s.store_max_with_scalar(214, 566, 1e-6);
            s.store_scalar(169, p.p447);
            s.store_scale(567, 0, p.p448);
            s.store_max_with_scalar(170, 567, 0.0);
        }

        let (assign5430_e4742,) = {
    if (!s.b[611]) {
        (p.p451,)
    } else {
        (s.v[171],)
    }
};
        s.v[171] = assign5430_e4742;

        let (assign5440_e4761,) = {
    if (!s.b[611]) {
        let assign5440_e4747: f64 = (p.p452 * s.v[548]);
        let assign5440_e4749: f64 = (assign5440_e4747 * s.v[548]);
        let assign5440_e4751: f64 = (assign5440_e4749 * s.v[580]);
        let assign5440_e4753: f64 = (assign5440_e4751 * s.v[580]);
        let assign5440_e4757: f64 = (p.p453 - 2.0);
        let assign5440_e4758: f64 = (s.v[579]).powf(assign5440_e4757);
        let assign5440_e4759: f64 = (assign5440_e4753 * assign5440_e4758);
        (assign5440_e4759,)
    } else {
        (s.v[172],)
    }
};
        s.v[172] = assign5440_e4761;

        if (!s.b[611]) {
            s.store_add_scaled_inputs(568, 581, p.p454, 580, p.p455);
            s.store_max_with_scalar(173, 568, 0.0);
            s.store_scale(174, 581, p.p456);
            s.store_scale(175, 581, p.p457);
            s.store_scalar(176, p.p458);
            s.store_scalar(177, p.p459);
            s.store_offset_scaled(0, 579, p.p490, p.p489);
            s.store_max_with_scalar(179, 0, 0.0);
            s.store_offset_scaled(0, 579, p.p492, p.p491);
            s.store_max_with_scalar(180, 0, 0.0);
            s.store_scalar(181, p.p493);
            s.store_scalar(182, p.p494);
        }

        s.b[661] = ((((p.p461 > 0.0) && (p.p26 > 0.0)) && (p.p27 > 0.0)) && ((p.p29 == 1.0) || ((p.p29 > 1.0) && (p.p28 > 0.0))));
        s.v[661] = if s.b[661] { 1.0 } else { 0.0 };

        s.b[662] = (p.p461 == 1.0);
        s.v[662] = if s.b[662] { 1.0 } else { 0.0 };

        if (((!s.b[611]) && s.b[661]) && s.b[662]) {
            s.store_scalar(592, 0.0);
            s.store_scalar(593, 0.0);
            s.store_scalar(594, 0.0);
        }

        let mut assign5720_loop_guard: usize = 0;
        while {
            let assign5720_cond_e4998: f64 = (p.p29 - 0.5);
            let assign5720_cond_e5000: f64 = if ((((!s.b[611]) && s.b[661]) && s.b[662]) && (s.v[594] < assign5720_cond_e4998)) { 1.0 } else { 0.0 };
            assign5720_cond_e5000 != 0.0
        } {
            assign5720_loop_guard += 1;
            assert!(assign5720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[611]) && s.b[661]) && s.b[662]) {
                s.store_add_ad_rhs(592, 592, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(594), (p.p28 + p.p20), (p.p26 + (0.5 * p.p20)))));
                s.store_add_ad_rhs(593, 593, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(594), (p.p28 + p.p20), (p.p27 + (0.5 * p.p20)))));
                s.store_offset(594, 594, 1.0);
            }
        }

        if (((!s.b[611]) && s.b[661]) && s.b[662]) {
            s.store_scale(595, 592, 1.0 / (p.p29));
            s.store_scale(596, 593, 1.0 / (p.p29));
            s.store_scalar(597, (1.0 / (p.p462 + (0.5 * p.p20))));
            s.store_scalar(598, (1.0 / (p.p463 + (0.5 * p.p20))));
            s.store_max_with_scalar_ad(599, A::offset(s.ad_value(573), p.p20), 1e-9);
            s.store_max_with_scalar_ad(600, A::offset(A::add(s.ad_value(532), s.ad_value(574)), p.p464), 1e-9);
            s.store_div_from_scalar_powf_ad(601, 1.0, s.ad_value(599), p.p471);
            s.store_div_from_scalar_powf_ad(602, 1.0, s.ad_value(600), p.p472);
            s.store_mul_ad(603, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(601), p.p468, 1.0), 1.0, s.ad_value(602), p.p469, s.ad_value(601), s.ad_value(602), p.p470), A::scale_offset(s.ad_value(221), p.p467, (((((-1.0)) * (p.p467))) + (1.0))));
            s.store_div_scaled_inputs2_indices(604, 595, p.p465, 596, p.p465, 603, 1.0);
            s.store_div_scaled_inputs2_indices(605, 597, p.p465, 598, p.p465, 603, 1.0);
            s.store_div_from_scalar_powf_ad(601, 1.0, s.ad_value(599), p.p477);
            s.store_div_from_scalar_powf_ad(602, 1.0, s.ad_value(600), p.p478);
            s.store_max_with_scalar_ad(606, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(601), p.p474, 1.0), 1.0, s.ad_value(602), p.p475, s.ad_value(601), s.ad_value(602), p.p476), 1e-20);
            s.store_add_scaled_inputs4_indices(607, 595, 1.0, 596, 1.0, 597, -1.0, 598, -1.0);
            s.store_div_scaled_product_offset_denominator(548, s.ad_value(548), A::offset(s.ad_value(604), 1.0), 1.0, s.ad_value(605), 1.0, 1.0);
            s.store_max_with_scalar(191, 548, 1e-10);
            s.store_scale(192, 191, p.p254);
            s.store_div_scaled_product_by_product(0, A::offset(s.ad_value(604), 1.0), A::scale_offset(s.ad_value(605), p.p466, 1.0), 1.0, A::offset(s.ad_value(605), 1.0), A::scale_offset(s.ad_value(604), p.p466, 1.0), 1.0);
            s.store_mul(552, 552, 0);
            s.store_max_with_scalar(199, 552, 0.0);
            s.store_mul(553, 553, 0);
            s.store_max_with_scalar(200, 553, 0.0);
            s.store_div_scaled_inputs_indices(0, 607, p.p473, 606, 1.0);
            s.store_add(183, 183, 0);
            s.store_add(184, 184, 0);
            s.store_add(185, 185, 0);
            s.store_add(186, 186, 0);
            s.store_div_scaled_inputs_mixed_ia(0, 607, p.p479, A::powf(s.ad_value(606), p.p480), 1.0);
            s.store_add(546, 546, 0);
            s.store_max_with_scalar(187, 546, 0.0);
            s.store_add(547, 547, 0);
            s.store_max_with_scalar(189, 547, 0.0);
            s.store_div_scaled_inputs_indices(0, 534, p.p238, 533, 1.0);
            s.store_mul(188, 187, 0);
            s.store_mul(190, 189, 0);
        }

        if (((!s.b[611]) && s.b[661]) && (!s.b[662])) {
            s.store_scalar(592, 0.0);
            s.store_scalar(594, 0.0);
            s.store_scalar(0, ((-1.0) / p.p482));
        }

        let mut assign6120_loop_guard: usize = 0;
        while {
            let assign6120_cond_e5595: f64 = (p.p29 - 0.5);
            let assign6120_cond_e5597: f64 = if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && (s.v[594] < assign6120_cond_e5595)) { 1.0 } else { 0.0 };
            assign6120_cond_e5597 != 0.0
        } {
            assign6120_loop_guard += 1;
            assert!(assign6120_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            s.b[663] = (((-((p.p26 + (0.5 * p.p20)) + (s.v[594] * (p.p28 + p.p20)))) / p.p481) > (-80.0));
            s.v[663] = if s.b[663] { 1.0 } else { 0.0 };
            if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && s.b[663]) {
                s.store_exp_scaled_input_ad(2, A::scale_offset(s.ad_value(594), (p.p28 + p.p20), (p.p26 + (0.5 * p.p20))), (-1.0 / (p.p481)));
            }
            if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && (!s.b[663])) {
                let assign6120_body2_ad_e5713: A = A::mul_offset_lhs(A::neg(A::scale(A::scale_offset(s.ad_value(594), (p.p28 + p.p20), (p.p26 + (0.5 * p.p20))), (-1.0 / (p.p481)))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::scale(A::scale_offset(s.ad_value(594), (p.p28 + p.p20), (p.p26 + (0.5 * p.p20))), (-1.0 / (p.p481)))), (-80.0)), 0.5, A::scale_offset(A::neg(A::scale(A::scale_offset(s.ad_value(594), (p.p28 + p.p20), (p.p26 + (0.5 * p.p20))), (-1.0 / (p.p481)))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
                s.store_div_from_scalar_offset_ad(2, 1.80485e-35, assign6120_body2_ad_e5713, 1.0);
            }
            s.b[664] = (((-((p.p27 + (0.5 * p.p20)) + (((p.p29 - 1.0) - s.v[594]) * (p.p28 + p.p20)))) / p.p481) > (-80.0));
            s.v[664] = if s.b[664] { 1.0 } else { 0.0 };
            if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && s.b[664]) {
                s.store_exp_scaled_input_ad(3, A::scale_offset(s.ad_value(594), (-(p.p28 + p.p20)), (((((p.p29 - 1.0)) * ((p.p28 + p.p20)))) + ((p.p27 + (0.5 * p.p20))))), (-1.0 / (p.p481)));
            }
            if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && (!s.b[664])) {
                let assign6120_body5_ad_e5851: A = A::mul_scaled_lhs(A::offset(A::neg(A::scale(A::scale_offset(s.ad_value(594), (-(p.p28 + p.p20)), (((((p.p29 - 1.0)) * ((p.p28 + p.p20)))) + ((p.p27 + (0.5 * p.p20))))), (-1.0 / (p.p481)))), (-80.0)), 0.5, A::scale_offset(A::neg(A::scale(A::scale_offset(s.ad_value(594), (-(p.p28 + p.p20)), (((((p.p29 - 1.0)) * ((p.p28 + p.p20)))) + ((p.p27 + (0.5 * p.p20))))), (-1.0 / (p.p481)))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0))));
                s.store_div_from_scalar_offset_mul_offset_lhs_ad(3, 1.80485e-35, A::neg(A::scale(A::scale_offset(s.ad_value(594), (-(p.p28 + p.p20)), (((((p.p29 - 1.0)) * ((p.p28 + p.p20)))) + ((p.p27 + (0.5 * p.p20))))), (-1.0 / (p.p481)))), (-80.0), A::offset(assign6120_body5_ad_e5851, 1.0), 1.0);
            }
            if (((!s.b[611]) && s.b[661]) && (!s.b[662])) {
                s.store_powf_ad(4, A::sub_from_scalar(1.0, s.ad_value(2)), (-p.p482));
                s.store_powf_ad(5, A::sub_from_scalar(1.0, s.ad_value(3)), (-p.p482));
                s.store_add_ad_rhs(592, 592, A::pow(A::add_scaled_inputs(s.ad_value(4), 0.5, s.ad_value(5), 0.5), s.ad_value(0)));
                s.store_offset(594, 594, 1.0);
            }
        }

        if (((!s.b[611]) && s.b[661]) && (!s.b[662])) {
            s.store_sub_from_scalar_scaled_input(608, 1.0, 592, 1.0 / (p.p29));
        }

        s.b[665] = (((-(p.p462 + (0.5 * p.p20))) / p.p481) > (-80.0));
        s.v[665] = if s.b[665] { 1.0 } else { 0.0 };

        if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && s.b[665]) {
            s.store_scalar(2, ((((-(p.p462 + (0.5 * p.p20))) / p.p481)) as f64).exp());
        }

        if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && (!s.b[665])) {
            s.store_scalar(2, (1.80485e-35 / (1.0 + (((-((-(p.p462 + (0.5 * p.p20))) / p.p481)) - 80.0) * (1.0 + ((0.5 * ((-((-(p.p462 + (0.5 * p.p20))) / p.p481)) - 80.0)) * (1.0 + (((-((-(p.p462 + (0.5 * p.p20))) / p.p481)) - 80.0) * 0.3333333333333))))))));
        }

        s.b[666] = (((-(p.p463 + (0.5 * p.p20))) / p.p481) > (-80.0));
        s.v[666] = if s.b[666] { 1.0 } else { 0.0 };

        if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && s.b[666]) {
            s.store_scalar(3, ((((-(p.p463 + (0.5 * p.p20))) / p.p481)) as f64).exp());
        }

        if ((((!s.b[611]) && s.b[661]) && (!s.b[662])) && (!s.b[666])) {
            s.store_scalar(3, (1.80485e-35 / (1.0 + (((-((-(p.p463 + (0.5 * p.p20))) / p.p481)) - 80.0) * (1.0 + ((0.5 * ((-((-(p.p463 + (0.5 * p.p20))) / p.p481)) - 80.0)) * (1.0 + (((-((-(p.p463 + (0.5 * p.p20))) / p.p481)) - 80.0) * 0.3333333333333))))))));
        }

        if (((!s.b[611]) && s.b[661]) && (!s.b[662])) {
            s.store_powf_ad(4, A::sub_from_scalar(1.0, s.ad_value(2)), (-p.p482));
            s.store_powf_ad(5, A::sub_from_scalar(1.0, s.ad_value(3)), (-p.p482));
            s.store_sub_from_scalar_ad(609, 1.0, A::pow(A::add_scaled_inputs(s.ad_value(4), 0.5, s.ad_value(5), 0.5), s.ad_value(0)));
            s.store_max_with_scalar_ad(600, A::offset(A::add(s.ad_value(532), s.ad_value(574)), p.p464), 1e-9);
            s.store_div_from_scalar_offset_scaled_input(610, p.p486, 221, p.p487, (((((-1.0)) * (p.p487))) + (1.0)));
            s.store_mul(604, 610, 608);
            s.store_mul(605, 610, 609);
            s.store_sub(607, 608, 609);
            s.store_max_with_scalar_ad(606, A::offset(A::div_scaled_inputs(s.ad_value(600), p.p484, s.ad_value(570), 1.0), 1.0), 1e-20);
            s.store_div_scaled_product_offset_denominator(548, s.ad_value(548), A::offset(s.ad_value(604), 1.0), 1.0, s.ad_value(605), 1.0, 1.0);
            s.store_max_with_scalar(191, 548, 1e-10);
            s.store_scale(192, 191, p.p254);
            s.store_div_scaled_product_by_product(0, A::offset(s.ad_value(604), 1.0), A::scale_offset(s.ad_value(605), p.p488, 1.0), 1.0, A::offset(s.ad_value(605), 1.0), A::scale_offset(s.ad_value(604), p.p488, 1.0), 1.0);
            s.store_mul(552, 552, 0);
            s.store_max_with_scalar(199, 552, 0.0);
            s.store_mul(553, 553, 0);
            s.store_max_with_scalar(200, 553, 0.0);
            s.store_div_scaled_inputs_indices(0, 607, p.p483, 606, 1.0);
            s.store_add(183, 183, 0);
            s.store_add(184, 184, 0);
            s.store_add(185, 185, 0);
            s.store_add(186, 186, 0);
            s.store_mul_ad_affine_product_rhs(0, 607, A::powf(s.ad_value(584), p.p236), A::scale_offset(s.ad_value(580), p.p237, 1.0), p.p485, 0.0);
            s.store_add(546, 546, 0);
            s.store_max_with_scalar(187, 546, 0.0);
            s.store_add(547, 547, 0);
            s.store_max_with_scalar(189, 547, 0.0);
            s.store_div_scaled_inputs_indices(0, 534, p.p238, 533, 1.0);
            s.store_mul(188, 187, 0);
            s.store_mul(190, 189, 0);
        }

        s.b[667] = (p.p7 == 0.0);
        s.v[667] = if s.b[667] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_5(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[667] {
            s.copy_ad(20, 19);
            s.copy_ad(203, 202);
            s.copy_ad(207, 206);
            s.copy_ad(205, 204);
            s.copy_ad(90, 89);
            s.copy_ad(209, 208);
            s.copy_ad(94, 93);
            s.copy_ad(96, 95);
            s.copy_ad(98, 97);
            s.copy_ad(160, 159);
            s.copy_ad(165, 164);
        }

        s.store_sub_from_scalar(228, 1.0, 15);

        s.store_add_scaled_inputs(229, 228, 1.04479e-10, 15, 1.43438e-10);

        s.store_sub_from_scalar_ad(230, 1.17, A::div_scaled_value_offset_denominator(s.ad_value(218), 0.000473, s.ad_value(217), 636.0, 1.0));

        s.store_sub_from_scalar_ad(231, 0.744, A::div_scaled_value_offset_denominator(s.ad_value(218), 0.0004774, s.ad_value(217), 235.0, 1.0));

        s.store_mul_add_scaled_inputs3_offset_rhs(232, 15, s.ad_value(231), 1.0, s.ad_value(230), (-1.0), s.ad_value(228), (-0.4), 0.0);

        s.store_add(233, 230, 232);

        s.store_scaled_mul(234, 233, 224, 0.5);

        s.copy_ad(235, 234);

        s.store_div_from_scalar_offset_ad(238, 1.0, A::sqrt_scaled_input(s.ad_value(15), 10.0), 1.0);

        s.store_sub_scaled_inputs(237, 15, 0.05, 232, 0.5);

        s.store_scaled_mul(0, 536, 14, ((1.602176565e-19 * 0.5) * 28959234086.17689));

        s.b[668] = (s.v[535] > 0.0);
        s.v[668] = if s.b[668] { 1.0 } else { 0.0 };

        if s.b[668] {
            s.store_mul_offset_rhs(243, 0, 533, (p.p13 * 4e-10));
            s.store_mul_offset_rhs(244, 0, 534, (p.p13 * 4e-10));
        }

        if (!s.b[668]) {
            s.store_mul_scaled_offset_rhs(243, 0, -1.0, 533, (p.p13 * 4e-10));
            s.store_mul_scaled_offset_rhs(244, 0, -1.0, 534, (p.p13 * 4e-10));
        }

        s.store_sqrt_scaled_input(0, 217, 0.0033333333333);

        s.store_mul3_affine_lhs(2, 0, 0, 4.05e25, 0.0, 0);

        s.store_mul(252, 2, 238);

        s.store_mul_exp_ad_rhs(251, 2, A::mul_scaled_lhs(s.ad_value(232), 0.5, s.ad_value(224)));

        s.store_mul_exp_ad_rhs(590, 2, A::mul_scaled_lhs(s.ad_value(232), 0.5, s.ad_value(224)));

        s.store_div_from_scalar(239, 3.45313e-11, 533);

        s.store_div_from_scalar(240, 3.45313e-11, 534);

        s.b[669] = (s.v[538] > 0.0);
        s.v[669] = if s.b[669] { 1.0 } else { 0.0 };

        if s.b[669] {
            s.store_mul_offset_rhs(241, 239, 538, 1.0);
            s.copy_ad(242, 240);
        }

        if (!s.b[669]) {
            s.copy_ad(241, 239);
            s.store_mul_sub_from_scalar_rhs(242, 240, 1.0, 538);
        }

        s.store_div(245, 229, 14);

        s.store_mul_offset_ad_rhs(226, 223, A::mul(s.ad_value(17), s.ad_value(222)), 1.0);

        s.store_div_from_scalar(227, 1.0, 226);

        s.store_scaled_mul(236, 233, 227, 0.5);

        s.store_div(246, 241, 245);

        s.store_div(247, 242, 245);

        s.store_div_from_scalar_add_ad(248, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(246)), 1.0), A::div_from_scalar(1.0, s.ad_value(247)));

        s.store_mul3_affine_lhs(253, 252, 229, (2.0 * 1.602176565e-19), 0.0, 227);

        s.store_offset_ln_ad(254, A::div_scaled_product(s.ad_value(245), s.ad_value(245), 1.0, s.ad_value(253), 1.0), (-0.6931471805599));

        s.store_mul_div_scaled_product_rhs(255, 227, s.ad_value(29), s.ad_value(14), (0.5 * 1.602176565e-19), A::add(s.ad_value(241), s.ad_value(242)), 1.0);

        s.store_mul(0, 34, 220);

        s.store_add(31, 187, 0);

        s.store_add(32, 188, 0);

        s.store_add(140, 189, 0);

        s.store_add(141, 190, 0);

        s.store_mul(329, 35, 227);

        s.store_div_ad_lhs(260, A::sqrt(A::mul_scaled_lhs(s.ad_value(537), ((2.0 * 1.602176565e-19) * 1.04479e-10), s.ad_value(224))), 242);

        s.store_square(261, 260);

        s.store_div_from_scalar(262, 1.0, 261);

        s.store_offset_scaled(263, 260, 0.707106781186545, 1.0);

        s.store_div_from_scalar(264, 1.0, 263);

        let assign7110_e6877: f64 = (1e-5 * s.v[263]);
        s.v[265] = assign7110_e6877;

        s.store_add_ad_lhs(591, A::ln(A::div(s.ad_value(537), s.ad_value(590))), 234);

        s.store_scale(266, 591, 2.0);

        s.b[670] = (p.p2 > 0.0);
        s.v[670] = if s.b[670] { 1.0 } else { 0.0 };

        if s.b[670] {
            s.store_add_ad_rhs(184, 184, A::mul3(s.ad_value(16), s.ad_value(223), s.ad_value(591)));
            s.store_add_ad_rhs(186, 186, A::mul3(s.ad_value(16), s.ad_value(223), s.ad_value(591)));
        }

        s.v[249] = 0.0;

        s.b[671] = (p.p9 > 0.0);
        s.v[671] = if s.b[671] { 1.0 } else { 0.0 };

        if s.b[671] {
            s.store_mul_add_ad_rhs(249, 223, A::ln(A::div(s.ad_value(24), s.ad_value(251))), s.ad_value(234));
        }

        s.store_div_ad_lhs(250, A::sqrt(A::mul_scaled_lhs(s.ad_value(229), (2.0 * 1.602176565e-19), s.ad_value(24))), 239);

        s.v[257] = 15.0;

        s.b[672] = (p.p10 == 1.0);
        s.v[672] = if s.b[672] { 1.0 } else { 0.0 };

        if s.b[672] {
            s.store_scaled_add_ad(257, A::offset(A::div_from_scalar(2970.0, s.ad_value(8)), 15.0), A::sqrt(A::offset(A::mul_sub_from_scalar_lhs(15.0, A::div_from_scalar(2970.0, s.ad_value(8)), A::sub_from_scalar(15.0, A::div_from_scalar(2970.0, s.ad_value(8)))), 1e-6)), 0.5);
        }

        s.v[256] = 0.0;

        s.v[258] = 0.0;

        s.store_scaled_mul(259, 14, 14, 1e18);

        s.b[673] = (p.p13 > 0.0);
        s.v[673] = if s.b[673] { 1.0 } else { 0.0 };

        s.b[674] = (p.p14 == 1.0);
        s.v[674] = if s.b[674] { 1.0 } else { 0.0 };

        if (s.b[673] && s.b[674]) {
            s.store_div_from_scalar(256, 0.409618895, 259);
            s.store_scale_ad(258, A::exp_scaled_input(A::ln(A::mul(s.ad_value(226), s.ad_value(259))), (-0.3333333333333)), ((0.4 * p.p13) * 1.27520989));
        }

        if (s.b[673] && (!s.b[674])) {
            s.store_div_from_scalar(256, 0.723134895, 259);
            s.store_scale_ad(258, A::exp_scaled_input(A::ln(A::mul(s.ad_value(226), s.ad_value(259))), (-0.3333333333333)), ((0.4 * p.p13) * 1.5412087));
        }

        s.store_add_scaled_product_indices(0, 256, 1.0, 23, 220, p.p14);

        s.store_sub_offset_lhs(2, 0, p.p34, 249);

        s.store_add_scaled_inputs4_indices(21, 183, p.p14, 237, p.p14, 243, p.p14, 2, 1.0);

        s.store_add_scaled_inputs4_indices(22, 184, p.p14, 237, p.p14, 244, p.p14, 0, 1.0);

        s.store_add_scaled_inputs4_indices(130, 185, p.p14, 237, p.p14, 243, p.p14, 2, 1.0);

        s.store_add_scaled_inputs4_indices(131, 186, p.p14, 237, p.p14, 244, p.p14, 0, 1.0);

        s.store_ln(295, 222);

        s.store_scaled_exp_ad(296, A::mul(s.ad_value(40), s.ad_value(295)), p.p35);

        s.store_mul(38, 191, 296);

        s.store_mul(39, 192, 296);

        s.store_exp_mul(297, 48, 295);

        s.store_mul(46, 193, 297);

        s.store_exp_mul(298, 49, 295);

        s.store_mul(47, 194, 298);

        s.store_exp_mul(299, 43, 295);

        s.store_mul(33, 195, 299);

        s.store_exp_mul(300, 45, 295);

        s.store_mul(44, 196, 300);

        s.store_exp_mul(301, 52, 295);

        s.store_mul(50, 197, 301);

        s.store_div_scaled_inputs_indices(0, 226, 1e-8, 14, 1.0);

        s.store_mul(267, 0, 46);

        s.store_div_from_scalar_scaled_input(268, 1.0, 539, 0.5);

        s.store_div(269, 268, 540);

        s.b[675] = (p.p14 == 1.0);
        s.v[675] = if s.b[675] { 1.0 } else { 0.0 };

        if s.b[675] {
            s.store_scale(270, 541, 0.5);
        }

        if (!s.b[675]) {
            s.store_scale(270, 541, 0.3333333333333);
        }

        s.store_sub_from_scalar(271, 1.0, 270);

        s.store_exp_mul(302, 55, 295);

        s.store_mul(53, 198, 302);

        s.store_scaled_mul(272, 53, 226, 2.0);

        s.store_offset_ad(215, A::exp_scaled_input(A::ln(A::offset(A::exp_scaled_input(A::div_from_scalar(16.0, s.ad_value(542)), 0.6931471805599), (-1.0))), 0.375), (-1.0));

        s.store_offset_ad(216, A::exp_scaled_input(A::ln(A::offset(A::exp_scaled_input(A::div_from_scalar(16.0, s.ad_value(543)), 0.6931471805599), (-1.0))), 0.375), (-1.0));

        s.store_exp_mul(303, 60, 295);

        s.store_mul3_lhs(59, 199, 303, 296);

        s.store_mul(273, 59, 226);

        s.store_mul3_lhs(147, 200, 303, 296);

        s.store_mul(274, 147, 226);

        s.store_mul(275, 64, 227);

        s.store_exp_mul_scaled_lhs_indices(304, 76, -1.0, 295);

        s.store_mul(68, 201, 304);

        s.store_mul(69, 202, 304);

        s.store_mul(70, 203, 304);

        s.store_mul(71, 204, 304);

        s.store_mul(72, 205, 304);

        s.store_exp_mul_scaled_lhs_indices(304, 77, -1.0, 295);

        s.store_mul(73, 206, 304);

        s.store_mul(74, 207, 304);

        s.store_div_from_scalar(276, 1.0, 87);

        s.store_scaled_sqrt_scaled_input(277, 87, ((2.0 * 1.602176565e-19) * 9.10938291e-31), ((4.0 * 0.3333333333333) * 9.482522386533242e33));

        s.store_mul(278, 277, 18);

        s.store_mul(279, 277, 18);

        s.v[280] = 0.0;

        s.b[676] = (s.v[79] < 0.0);
        s.v[676] = if s.b[676] { 1.0 } else { 0.0 };

        if s.b[676] {
            s.store_div_scaled_inputs_indices(280, 78, (-0.495), 79, 1.0);
        }

        s.v[281] = 0.0;

        s.b[677] = (s.v[82] < 0.0);
        s.v[677] = if s.b[677] { 1.0 } else { 0.0 };

        if s.b[677] {
            s.store_div_scaled_inputs_indices(281, 80, (-0.495), 82, 1.0);
        }

        s.v[282] = 0.0;

        s.b[678] = (s.v[84] < 0.0);
        s.v[678] = if s.b[678] { 1.0 } else { 0.0 };

        if s.b[678] {
            s.store_div_scaled_inputs_indices(282, 83, (-0.495), 84, 1.0);
        }

        s.store_scale(283, 233, 0.5);

        s.store_mul(284, 75, 226);

        s.store_mul(285, 75, 223);

        s.store_div_from_scalar_offset_ad(286, 1.0, A::mul(s.ad_value(88), s.ad_value(236)), 1.0);

        s.store_div_from_scalar_square_ad(0, 4e-18, s.ad_value(18));

        s.store_mul(89, 89, 0);

        s.store_mul(90, 90, 0);

        s.store_scale(0, 18, 500000000.0);

        s.store_scaled_add_ad(277, A::offset(A::mul(s.ad_value(93), s.ad_value(220)), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(93), s.ad_value(220)), 1.0, A::offset(A::mul(s.ad_value(93), s.ad_value(220)), 1.0)), 0.01)), 0.5);

        s.store_mul3_lhs(91, 208, 277, 0);

        s.store_scaled_add_ad(277, A::offset(A::mul(s.ad_value(94), s.ad_value(220)), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(94), s.ad_value(220)), 1.0, A::offset(A::mul(s.ad_value(94), s.ad_value(220)), 1.0)), 0.01)), 0.5);

        s.store_mul3_lhs(92, 209, 277, 0);

        s.store_mul_exp_ad_rhs(113, 210, A::mul_scaled_lhs(s.ad_value(114), -1.0, s.ad_value(295)));

        s.store_mul_offset_ad_rhs(288, 223, A::mul(s.ad_value(99), s.ad_value(222)), 1.0);

        s.store_div_from_scalar(289, 1.0, 288);

        s.store_mul3_affine_lhs(290, 252, 229, (2.0 * 1.602176565e-19), 0.0, 289);

        s.store_add_scaled_product_indices(0, 256, 1.0, 102, 220, p.p14);

        s.store_sub_offset_ad_lhs(100, A::add_scaled_inputs4(s.ad_value(211), p.p14, s.ad_value(237), p.p14, s.ad_value(243), p.p14, s.ad_value(0), 1.0), p.p34, 249);

        s.store_add_scaled_inputs4_indices(101, 212, p.p14, 237, p.p14, 244, p.p14, 0, 1.0);

        s.store_scaled_exp_ad(0, A::mul(s.ad_value(111), s.ad_value(295)), p.p35);

        s.store_mul(110, 213, 0);

        s.store_mul(287, 116, 226);

        s.store_div_scaled_inputs_mixed_ia(291, 118, (0.25 * 1.602176565e-19), A::mul(s.ad_value(229), s.ad_value(226)), 1.0);

        s.store_ln_div(292, 118, 252);

        s.store_scaled_mul(293, 119, 226, 1.25e-6);

        s.store_sqrt_ad(294, A::mul3_scaled_output(s.ad_value(229), s.ad_value(14), A::offset(s.ad_value(533), 4e-10), 1.0 / (3.45313e-11)));

        s.store_exp_mul(305, 169, 295);

        s.store_mul(168, 214, 305);

        let assign8220_e7523: f64 = (4.0 * 1.3806488e-23);
        let assign8220_e7525: f64 = (assign8220_e7523 * s.v[217]);
        s.v[306] = assign8220_e7525;

        let assign8230_e7528: f64 = (s.v[171] * s.v[306]);
        s.v[307] = assign8230_e7528;

        s.v[308] = s.v[307];

        let assign8250_e7532: f64 = (9.10938291e-31 * 1000000000000.0);
        let assign8250_e7534: f64 = (assign8250_e7532 * s.v[172]);
        s.v[309] = assign8250_e7534;

        s.b[785] = (s.v[6] > 0.0);
        s.v[785] = if s.b[785] { 1.0 } else { 0.0 };

        if s.b[785] {
            s.store_voltage(219, ctx, nodes, Some(4), None);
            s.store_add(217, 8, 219);
            s.store_square(218, 217);
            s.store_offset(220, 217, (-s.v[7]));
            s.store_scale(221, 217, 1.0 / (s.v[7]));
            s.store_div_from_scalar(222, s.v[7], 217);
            s.store_scale(223, 217, 8.617332384961e-5);
            s.store_div_from_scalar(224, 1.0, 223);
        }

        s.b[786] = (p.p10 == 1.0);
        s.v[786] = if s.b[786] { 1.0 } else { 0.0 };

        if (s.b[785] && s.b[786]) {
            s.store_scaled_add_ad(225, A::offset(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), 600.0), A::sqrt(A::offset(A::mul_offset_lhs(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), (-600.0), A::offset(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), (-600.0))), 0.01)), 0.5);
        }

        if (s.b[785] && (!s.b[786])) {
            s.store_scalar(225, 600.0);
        }

        if s.b[785] {
            s.store_sub_from_scalar_ad(230, 1.17, A::div_scaled_value_offset_denominator(s.ad_value(218), 0.000473, s.ad_value(217), 636.0, 1.0));
            s.store_sub_from_scalar_ad(231, 0.744, A::div_scaled_value_offset_denominator(s.ad_value(218), 0.0004774, s.ad_value(217), 235.0, 1.0));
            s.store_mul_add_scaled_inputs3_offset_rhs(232, 15, s.ad_value(231), 1.0, s.ad_value(230), (-1.0), s.ad_value(228), (-0.4), 0.0);
            s.store_add(233, 230, 232);
            s.store_scaled_mul(234, 233, 224, 0.5);
            s.store_sub_scaled_inputs(237, 15, 0.05, 232, 0.5);
            s.store_sqrt_scaled_input(0, 217, 0.0033333333333);
            s.store_mul3_affine_lhs(2, 0, 0, 4.05e25, 0.0, 0);
        }

    }

    pub(super) fn stamp_transient_block_6(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[785] {
            s.store_mul(252, 2, 238);
            s.store_mul_offset_ad_rhs(226, 223, A::mul(s.ad_value(17), s.ad_value(222)), 1.0);
            s.store_div_from_scalar(227, 1.0, 226);
            s.store_scaled_mul(236, 233, 227, 0.5);
            s.store_mul3_affine_lhs(253, 252, 229, (2.0 * 1.602176565e-19), 0.0, 227);
            s.store_offset_ln_ad(254, A::div_scaled_product(s.ad_value(245), s.ad_value(245), 1.0, s.ad_value(253), 1.0), (-0.6931471805599));
            s.store_mul_div_scaled_product_rhs(255, 227, s.ad_value(29), s.ad_value(14), (0.5 * 1.602176565e-19), A::add(s.ad_value(241), s.ad_value(242)), 1.0);
            s.store_mul(0, 34, 220);
            s.store_add(31, 187, 0);
            s.store_add(32, 188, 0);
            s.store_mul(329, 35, 227);
            s.store_add(140, 189, 0);
            s.store_add(141, 190, 0);
        }

        s.b[787] = (p.p9 > 0.0);
        s.v[787] = if s.b[787] { 1.0 } else { 0.0 };

        if (s.b[785] && s.b[787]) {
            s.store_mul_add_ad_rhs(249, 223, A::ln(A::div(s.ad_value(24), s.ad_value(251))), s.ad_value(235));
        }

        s.b[788] = (p.p10 == 1.0);
        s.v[788] = if s.b[788] { 1.0 } else { 0.0 };

        if (s.b[785] && s.b[788]) {
            s.store_scaled_add_ad(257, A::offset(A::div_from_scalar(2970.0, s.ad_value(8)), 15.0), A::sqrt(A::offset(A::mul_sub_from_scalar_lhs(15.0, A::div_from_scalar(2970.0, s.ad_value(8)), A::sub_from_scalar(15.0, A::div_from_scalar(2970.0, s.ad_value(8)))), 1e-6)), 0.5);
        }

        if s.b[785] {
            s.store_scalar(258, 0.0);
        }

        s.b[789] = (p.p13 > 0.0);
        s.v[789] = if s.b[789] { 1.0 } else { 0.0 };

        s.b[790] = (p.p14 == 1.0);
        s.v[790] = if s.b[790] { 1.0 } else { 0.0 };

        if ((s.b[785] && s.b[789]) && s.b[790]) {
            s.store_scale_ad(258, A::exp_scaled_input(A::ln(A::mul(s.ad_value(226), s.ad_value(259))), (-0.3333333333333)), ((0.4 * p.p13) * 1.27520989));
        }

        if ((s.b[785] && s.b[789]) && (!s.b[790])) {
            s.store_scale_ad(258, A::exp_scaled_input(A::ln(A::mul(s.ad_value(226), s.ad_value(259))), (-0.3333333333333)), ((0.4 * p.p13) * 1.5412087));
        }

        if s.b[785] {
            s.store_add_scaled_product_indices(0, 256, 1.0, 23, 220, p.p14);
            s.store_sub_offset_lhs(2, 0, p.p34, 249);
            s.store_add_scaled_inputs4_indices(21, 183, p.p14, 237, p.p14, 243, p.p14, 2, 1.0);
            s.store_add_scaled_inputs4_indices(22, 184, p.p14, 237, p.p14, 244, p.p14, 0, 1.0);
            s.store_add_scaled_inputs4_indices(130, 185, p.p14, 237, p.p14, 243, p.p14, 2, 1.0);
            s.store_add_scaled_inputs4_indices(131, 186, p.p14, 237, p.p14, 244, p.p14, 0, 1.0);
            s.store_ln(295, 222);
            s.store_scaled_exp_ad(296, A::mul(s.ad_value(40), s.ad_value(295)), p.p35);
            s.store_mul(38, 191, 296);
            s.store_mul(39, 192, 296);
            s.store_exp_mul(297, 48, 295);
            s.store_mul(46, 193, 297);
            s.store_exp_mul(298, 49, 295);
            s.store_mul(47, 194, 298);
            s.store_exp_mul(299, 43, 295);
            s.store_mul(33, 195, 299);
            s.store_exp_mul(300, 45, 295);
            s.store_mul(44, 196, 300);
            s.store_exp_mul(301, 52, 295);
            s.store_mul(50, 197, 301);
            s.store_div_scaled_inputs_indices(0, 226, 1e-8, 14, 1.0);
            s.store_mul(267, 0, 46);
            s.store_exp_mul(302, 55, 295);
            s.store_mul(53, 198, 302);
            s.store_scaled_mul(272, 53, 226, 2.0);
            s.store_exp_mul(303, 60, 295);
            s.store_mul3_lhs(59, 199, 303, 296);
            s.store_mul(273, 59, 226);
            s.store_mul3_lhs(147, 200, 303, 296);
            s.store_mul(274, 147, 226);
            s.store_mul(275, 64, 227);
            s.store_exp_mul_scaled_lhs_indices(304, 76, -1.0, 295);
            s.store_mul(68, 201, 304);
            s.store_mul(69, 202, 304);
            s.store_mul(70, 203, 304);
            s.store_mul(71, 204, 304);
            s.store_mul(72, 205, 304);
            s.store_exp_mul_scaled_lhs_indices(304, 77, -1.0, 295);
            s.store_mul(73, 206, 304);
            s.store_mul(74, 207, 304);
            s.store_scale(283, 233, 0.5);
            s.store_mul(284, 75, 226);
            s.store_mul(285, 75, 223);
            s.store_div_from_scalar_offset_ad(286, 1.0, A::mul(s.ad_value(88), s.ad_value(236)), 1.0);
            s.store_scale(0, 18, 500000000.0);
            s.store_scaled_add_ad(277, A::offset(A::mul(s.ad_value(93), s.ad_value(220)), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(93), s.ad_value(220)), 1.0, A::offset(A::mul(s.ad_value(93), s.ad_value(220)), 1.0)), 0.01)), 0.5);
            s.store_mul3_lhs(91, 208, 277, 0);
            s.store_scaled_add_ad(277, A::offset(A::mul(s.ad_value(94), s.ad_value(220)), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(94), s.ad_value(220)), 1.0, A::offset(A::mul(s.ad_value(94), s.ad_value(220)), 1.0)), 0.01)), 0.5);
            s.store_mul3_lhs(92, 209, 277, 0);
            s.store_mul_exp_ad_rhs(113, 210, A::mul_scaled_lhs(s.ad_value(114), -1.0, s.ad_value(295)));
            s.store_mul(287, 116, 226);
            s.store_div_scaled_inputs_mixed_ia(291, 118, (0.25 * 1.602176565e-19), A::mul(s.ad_value(229), s.ad_value(226)), 1.0);
            s.store_ln_div(292, 118, 252);
            s.store_scaled_mul(293, 119, 226, 1.25e-6);
            s.store_exp_mul(305, 169, 295);
            s.store_mul(168, 214, 305);
        }

        let (assign9360_e8432,) = {
    if s.b[785] {
        let assign9360_e8428: f64 = (4.0 * 1.3806488e-23);
        let assign9360_e8430: f64 = (assign9360_e8428 * s.v[217]);
        (assign9360_e8430,)
    } else {
        (s.v[306],)
    }
};
        s.v[306] = assign9360_e8432;

        let (assign9370_e8438,) = {
    if s.b[785] {
        let assign9370_e8436: f64 = (s.v[171] * s.v[306]);
        (assign9370_e8436,)
    } else {
        (s.v[307],)
    }
};
        s.v[307] = assign9370_e8438;

        s.b[791] = (p.p14 == 1.0);
        s.v[791] = if s.b[791] { 1.0 } else { 0.0 };

        if s.b[791] {
            s.store_voltage(330, ctx, nodes, Some(9), Some(6));
            s.store_voltage(702, ctx, nodes, Some(7), Some(6));
            s.store_voltage(331, ctx, nodes, Some(6), Some(8));
        }

        if (!s.b[791]) {
            s.store_scaled_voltage(330, ctx, nodes, Some(9), Some(6), -1.0);
            s.store_scaled_voltage(702, ctx, nodes, Some(7), Some(6), -1.0);
            s.store_scaled_voltage(331, ctx, nodes, Some(6), Some(8), -1.0);
        }

        s.store_neg(703, 702);

        s.store_add(332, 330, 703);

        s.store_add(333, 702, 331);

        s.b[792] = (s.v[702] < 0.0);
        s.v[792] = if s.b[792] { 1.0 } else { 0.0 };

        if s.b[792] {
            s.store_scalar(334, (-1.0));
            s.copy_ad(336, 703);
            s.copy_ad(335, 332);
            s.copy_ad(337, 333);
        }

        if (!s.b[792]) {
            s.store_scalar(334, 1.0);
            s.copy_ad(336, 702);
            s.copy_ad(335, 330);
            s.copy_ad(337, 331);
        }

        s.store_add(338, 335, 337);

        s.store_mul(339, 336, 227);

        s.store_mul_offset_ad_lhs(340, A::sqrt(A::offset(A::square(s.ad_value(336)), 0.01)), (-0.1), 227);

        s.store_scaled_sub(341, 339, 340, 0.5);

        s.copy_ad(869, 21);

        s.copy_ad(870, 22);

        s.copy_ad(871, 27);

        s.copy_ad(872, 28);

        s.copy_ad(873, 31);

        s.copy_ad(874, 32);

        s.copy_ad(875, 273);

        s.copy_ad(876, 215);

        s.copy_ad(877, 63);

        s.store_sub_ad_lhs(878, A::add_scaled_product(s.ad_value(341), (-1.0), A::sub(s.ad_value(335), s.ad_value(869)), s.ad_value(227), 1.0), 234);

        s.store_add_scaled_product_left_ad(879, 341, (-1.0), A::sub_scaled_inputs(s.ad_value(337), -1.0, s.ad_value(870), 1.0), 227, 1.0);

        s.store_sub(880, 879, 234);

        s.b[1059] = (p.p2 > 0.0);
        s.v[1059] = if s.b[1059] { 1.0 } else { 0.0 };

        if s.b[1059] {
            s.store_scale(0, 16, p.p14);
            s.store_div_scaled_offset_numerator(881, s.ad_value(246), 1.0, 1.0, A::offset(s.ad_value(247), 1.0), 1.0);
            s.store_ln(882, 881);
        }

        s.b[1060] = (s.v[882] > 1e-8);
        s.v[1060] = if s.b[1060] { 1.0 } else { 0.0 };

        if (s.b[1059] && s.b[1060]) {
            s.store_div_scaled_product_offset_denominator(883, s.ad_value(882), A::offset(s.ad_value(881), 1.0), 2.0, s.ad_value(881), (-1.0), 1.0);
        }

        if (s.b[1059] && (!s.b[1060])) {
            s.store_scaled_offset(883, 882, 2.0, 2.0);
        }

        if s.b[1059] {
            s.store_div_ad_rhs(884, 253, A::square(s.ad_value(245)));
            s.store_div_from_scalar(885, 1.0, 246);
            s.store_div_from_scalar(886, 1.0, 247);
            s.store_div_from_scalar_add_ad(913, 1.0, A::offset(s.ad_value(885), 1.0), s.ad_value(886));
            s.store_mul_sub_rhs(914, 913, 878, 880);
            s.store_add_scaled_product_indices(887, 878, 1.0, 914, 885, (-1.0));
            s.store_add_scaled_product_indices(888, 880, 1.0, 914, 886, 1.0);
            s.store_div_from_scalar_offset_input(793, 1.0, 246, 1.0);
            s.store_div_from_scalar_offset_input(794, 1.0, 247, 1.0);
            s.store_offset_ln_ad(796, A::div_scaled_product(A::add_scaled_product(s.ad_value(246), 1.0, s.ad_value(247), s.ad_value(794), 1.0), s.ad_value(883), 1.0, s.ad_value(884), 1.0), 1.5);
            s.store_offset_ln_ad(797, A::div_scaled_product(A::add_scaled_product(s.ad_value(247), 1.0, s.ad_value(246), s.ad_value(793), 1.0), s.ad_value(883), 1.0, s.ad_value(884), 1.0), 1.5);
        }

        s.b[1061] = (((s.v[796] - s.v[887]) / 1.5) < 80.0);
        s.v[1061] = if s.b[1061] { 1.0 } else { 0.0 };

        if (s.b[1059] && s.b[1061]) {
            s.store_ln_one_plus_exp_ad(795, A::sub_scaled_inputs(s.ad_value(796), 0.6666666666666666, s.ad_value(887), 0.6666666666666666));
        }

        if (s.b[1059] && (!s.b[1061])) {
            s.store_scaled_sub(795, 796, 887, 0.6666666666666666);
        }

    }

    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1059] {
            s.store_sub_scaled_inputs(800, 796, 1.0, 795, 1.5);
            s.store_mul_add_scaled_product_rhs(799, 794, s.ad_value(800), 1.0, s.ad_value(247), s.ad_value(880), 1.0);
        }

        s.b[1062] = (((s.v[797] - s.v[799]) / 1.5) < 80.0);
        s.v[1062] = if s.b[1062] { 1.0 } else { 0.0 };

        if (s.b[1059] && s.b[1062]) {
            s.store_ln_one_plus_exp_ad(795, A::sub_scaled_inputs(s.ad_value(797), 0.6666666666666666, s.ad_value(799), 0.6666666666666666));
        }

        if (s.b[1059] && (!s.b[1062])) {
            s.store_scaled_sub(795, 797, 799, 0.6666666666666666);
        }

        if s.b[1059] {
            s.store_sub_scaled_inputs(1, 797, 1.0, 795, 1.5);
            s.store_mul(2, 0, 1);
            s.store_mul(3, 0, 880);
            s.store_sub(845, 2, 3);
        }

        s.b[1063] = ((((-s.v[266])) as f64).abs() < 80.0);
        s.v[1063] = if s.b[1063] { 1.0 } else { 0.0 };

        if (s.b[1059] && s.b[1063]) {
            s.store_exp_neg_input(846, 266);
        }

        s.b[1064] = ((-s.v[266]) < (-80.0));
        s.v[1064] = if s.b[1064] { 1.0 } else { 0.0 };

        if ((s.b[1059] && (!s.b[1063])) && s.b[1064]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(846, 1.80485e-35, A::neg(A::neg(s.ad_value(266))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(266))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(266))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((s.b[1059] && (!s.b[1063])) && (!s.b[1064])) {
            s.store_scaled_offset_mul_offset_lhs_ad(846, A::neg(s.ad_value(266)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(266)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(266)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        s.b[1065] = (((s.v[845]) as f64).abs() <= s.v[265]);
        s.v[1065] = if s.b[1065] { 1.0 } else { 0.0 };

        if (s.b[1059] && s.b[1065]) {
            s.store_scaled_square(843, 264, (0.1666666666667 * 0.707106781186545));
            s.store_mul_ad_product_rhs(4, 845, s.ad_value(264), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(845), 1.0, s.ad_value(846)), s.ad_value(260), s.ad_value(843)), 1.0));
        }

        s.b[1066] = (s.v[845] < (-s.v[265]));
        s.v[1066] = if s.b[1066] { 1.0 } else { 0.0 };

        if ((s.b[1059] && (!s.b[1065])) && s.b[1066]) {
            s.store_neg(847, 845);
            s.store_scaled_mul(848, 847, 264, 1.25);
            s.store_scaled_sub_ad(849, A::offset(s.ad_value(848), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(848), (-6.0), A::offset(s.ad_value(848), (-6.0))), 64.0)), 0.5);
            s.store_sub(842, 847, 849);
            s.store_add_scaled_square_product_mixed_iia(850, 842, 1.0, 261, A::offset(s.ad_value(849), 1.0), 1.0);
            s.store_sub_scaled_inputs(852, 842, 2.0, 261, 1.0);
            s.store_sub_ad_lhs(853, A::ln(A::mul(s.ad_value(850), s.ad_value(262))), 849);
            s.store_add(840, 850, 852);
            s.store_add_scaled_square_product_mixed_iia(841, 840, 1.0, 853, A::add_scaled_product(s.ad_value(850), (-1.0), s.ad_value(852), s.ad_value(852), 0.5), 1.0);
            s.store_add_ad_rhs(854, 849, A::div_scaled_product3(s.ad_value(850), s.ad_value(840), s.ad_value(853), 1.0, A::add(s.ad_value(841), A::mul3(A::mul3(A::div(s.ad_value(840), s.ad_value(841)), s.ad_value(853), s.ad_value(853)), s.ad_value(852), A::sub_scaled_inputs(A::square(s.ad_value(852)), 0.3333333333333, s.ad_value(850), 1.0))), 1.0));
        }

        s.b[1067] = (s.v[854] < 80.0);
        s.v[1067] = if s.b[1067] { 1.0 } else { 0.0 };

        if (((s.b[1059] && (!s.b[1065])) && s.b[1066]) && s.b[1067]) {
            s.store_exp(855, 854);
        }

        if (((s.b[1059] && (!s.b[1065])) && s.b[1066]) && (!s.b[1067])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(855, 854, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(854), (-80.0)), 0.5, A::scale_offset(s.ad_value(854), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if ((s.b[1059] && (!s.b[1065])) && s.b[1066]) {
            s.store_div_from_scalar(856, 1.0, 855);
            s.store_div_from_scalar_offset_ad(842, 1.0, A::square(s.ad_value(854)), 2.0);
            s.store_mul_square_lhs(857, 854, 842);
            s.store_mul3_affine_lhs(858, 854, 842, 4.0, 0.0, 842);
            s.store_mul_ad_product_lhs(859, A::sub_scaled_inputs(s.ad_value(842), 8.0, s.ad_value(857), 12.0), s.ad_value(842), 842);
            s.store_sub(842, 847, 854);
            s.store_mul(843, 846, 856);
            s.store_add_scaled_product_right_ad(860, 842, 2.0, 261, A::add_scaled_inputs3_offset(s.ad_value(855), 1.0, s.ad_value(843), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(846), 1.0, s.ad_value(858)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(861, 842, 1.0, 261, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(855), 1.0, s.ad_value(854), (-1.0), s.ad_value(843), 1.0, (-1.0)), 1.0, s.ad_value(846), A::sub(A::offset(s.ad_value(854), (-1.0)), s.ad_value(857)), 1.0), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(842, 2.0, 261, A::add_scaled_inputs_product(s.ad_value(855), 1.0, s.ad_value(843), 1.0, s.ad_value(846), s.ad_value(859), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(842, 860, 1.0, 861, 842, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(4, 854, -1.0, A::div(s.ad_value(861), A::add(s.ad_value(860), A::sqrt(s.ad_value(842)))), 2.0);
        }

        if ((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) {
            s.store_div_from_scalar_offset_scaled_input(862, 1.0, 260, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(863, A::mul_scaled_lhs(s.ad_value(263), 1.25, s.ad_value(862)), (-1.0), 862);
            s.store_mul_ad_product_rhs(864, 845, s.ad_value(264), A::offset(A::mul(s.ad_value(863), s.ad_value(845)), 1.0));
        }

        s.b[1068] = ((-s.v[864]) > (-80.0));
        s.v[1068] = if s.b[1068] { 1.0 } else { 0.0 };

        if (((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) && s.b[1068]) {
            s.store_exp_neg_input(842, 864);
        }

        if (((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) && (!s.b[1068])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(842, 1.80485e-35, A::neg(A::neg(s.ad_value(864))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(864))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(864))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) {
            s.store_sub_from_scalar(865, 1.0, 842);
            s.store_add_scaled_inputs_product_right_ad(866, 845, 1.0, 261, 0.5, 260, A::sqrt(A::add_scaled_inputs3(s.ad_value(845), 1.0, s.ad_value(261), 0.25, s.ad_value(865), -1.0)), (-1.0));
            s.store_offset(867, 266, 3.0);
            s.store_sub_ad(849, A::add_scaled_inputs3(s.ad_value(866), 0.5, s.ad_value(867), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(866), s.ad_value(867)), A::sub(s.ad_value(866), s.ad_value(867))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(867), 0.5, A::sqrt(A::offset(A::square(s.ad_value(867)), 5.0)), 0.5));
            s.store_sub(842, 845, 849);
            s.store_exp_neg_input(843, 849);
            s.store_div_from_scalar_offset_ad(844, 1.0, A::square(s.ad_value(849)), 2.0);
            s.store_mul_square_lhs(857, 849, 844);
            s.store_mul3_affine_lhs(858, 849, 844, 4.0, 0.0, 844);
            s.store_mul_ad_product_lhs(859, A::sub_scaled_inputs(s.ad_value(844), 8.0, s.ad_value(857), 12.0), s.ad_value(844), 844);
            s.store_max_from_scalar_ad(850, 1e-40, A::add_scaled_square_product(s.ad_value(842), 1.0, s.ad_value(261), A::add_scaled_product(A::offset(A::add(s.ad_value(843), s.ad_value(849)), (-1.0)), 1.0, s.ad_value(846), A::add(A::offset(s.ad_value(849), 1.0), s.ad_value(857)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(851, 1.0, 261, A::add_scaled_product(s.ad_value(843), 1.0, s.ad_value(846), s.ad_value(859), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(852, 842, 2.0, 261, A::add_scaled_sub_value_product(1.0, s.ad_value(843), 1.0, s.ad_value(846), A::offset(s.ad_value(858), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(853, 266, 1.0, 849, (-1.0), A::ln(A::div(s.ad_value(850), s.ad_value(261))), 1.0);
            s.store_add(840, 850, 852);
            s.store_add_scaled_square_product_mixed_iia(841, 840, 1.0, 853, A::add_scaled_products(s.ad_value(852), s.ad_value(852), 0.5, s.ad_value(850), s.ad_value(851), (-1.0)), 1.0);
            s.store_add_ad_rhs(868, 849, A::div_scaled_product3(s.ad_value(850), s.ad_value(840), s.ad_value(853), 1.0, A::add(s.ad_value(841), A::mul3(A::mul3(A::div(s.ad_value(840), s.ad_value(841)), s.ad_value(853), s.ad_value(853)), s.ad_value(852), A::add_scaled_square_product(s.ad_value(852), 0.3333333333333, s.ad_value(850), s.ad_value(851), (-1.0)))), 1.0));
        }

        s.b[1069] = (s.v[868] < 80.0);
        s.v[1069] = if s.b[1069] { 1.0 } else { 0.0 };

        if (((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) && s.b[1069]) {
            s.store_exp(855, 868);
            s.store_div_from_scalar(856, 1.0, 855);
            s.store_mul(855, 846, 855);
        }

        s.b[1070] = (s.v[868] > (s.v[266] - 80.0));
        s.v[1070] = if s.b[1070] { 1.0 } else { 0.0 };

        if ((((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) && (!s.b[1069])) && s.b[1070]) {
            s.store_exp_sub(855, 868, 266);
            s.store_div(856, 846, 855);
        }

        if ((((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) && (!s.b[1069])) && (!s.b[1070])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(855, 1.80485e-35, A::sub(s.ad_value(266), s.ad_value(868)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(266), s.ad_value(868)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(266), s.ad_value(868)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(856, 1.80485e-35, 868, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(868), (-80.0)), 0.5, A::scale_offset(s.ad_value(868), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((s.b[1059] && (!s.b[1065])) && (!s.b[1066])) {
            s.store_div_from_scalar_offset_ad(842, 1.0, A::square(s.ad_value(868)), 2.0);
            s.store_mul_square_lhs(857, 868, 842);
            s.store_mul3_affine_lhs(858, 868, 842, 4.0, 0.0, 842);
            s.store_mul_ad_product_lhs(859, A::sub_scaled_inputs(s.ad_value(842), 8.0, s.ad_value(857), 12.0), s.ad_value(842), 842);
            s.store_sub(842, 845, 868);
            s.store_add_scaled_product_right_ad(860, 842, 2.0, 261, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(856)), 1.0, s.ad_value(855), 1.0, s.ad_value(846), A::offset(s.ad_value(858), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(861, 842, 1.0, 261, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(856), 1.0, s.ad_value(868), 1.0, s.ad_value(855), 1.0, (-1.0)), 1.0, s.ad_value(846), A::add(A::offset(s.ad_value(868), 1.0), s.ad_value(857)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(842, 2.0, 261, A::add_scaled_inputs_product(s.ad_value(856), 1.0, s.ad_value(855), 1.0, s.ad_value(846), s.ad_value(859), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(842, 860, 1.0, 861, 842, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(4, 868, 1.0, A::div(s.ad_value(861), A::add(s.ad_value(860), A::sqrt(s.ad_value(842)))), 2.0);
        }

        if s.b[1059] {
            s.store_mul_add_rhs(889, 0, 4, 3);
        }

        if (!s.b[1059]) {
            s.copy_ad(889, 880);
        }

        s.store_mul_sub_rhs(0, 248, 878, 889);

        s.b[1071] = (p.p13 > 0.0);
        s.v[1071] = if s.b[1071] { 1.0 } else { 0.0 };

        if s.b[1071] {
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(890, 0, 0.5, 257, 0.5, A::add_scaled_square_product(s.ad_value(257), 1.0, A::sub(s.ad_value(0), s.ad_value(257)), A::sub(s.ad_value(0), s.ad_value(257)), 1.0), 0.5);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(891, 257, 0.5, 0, ((-1.0) * 0.5), A::add_scaled_square_product(s.ad_value(257), 1.0, A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(257), 1.0), A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(257), 1.0), 1.0), 0.5);
            s.store_mul_ad_rhs(2, 258, A::exp_scaled_input(A::ln(s.ad_value(890)), (-0.3333333333333)));
            s.store_mul_ad_rhs(3, 258, A::exp_scaled_input(A::ln(s.ad_value(891)), (-0.3333333333333)));
            s.store_sub_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
            s.store_div(898, 245, 4);
            s.store_offset_mul(892, 246, 2, 1.0);
            s.store_offset_mul(893, 247, 3, 1.0);
            s.store_div_scaled_product_indices(894, 246, 4, 1.0, 892, 1.0);
            s.store_div_scaled_product_indices(895, 247, 4, 1.0, 893, 1.0);
            s.store_div_from_scalar_add_ad(896, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(894)), 1.0), A::div_from_scalar(1.0, s.ad_value(895)));
            s.store_offset_mul(892, 894, 2, 1.0);
            s.store_offset_mul(893, 895, 3, 1.0);
        }

        if (!s.b[1071]) {
            s.copy_ad(898, 245);
            s.copy_ad(894, 246);
            s.copy_ad(895, 247);
            s.copy_ad(896, 248);
            s.store_scalar(892, 1.0);
            s.store_scalar(893, 1.0);
        }

        s.store_mul_sub_rhs(897, 896, 878, 889);

        s.b[1072] = (s.v[897] > 0.0);
        s.v[1072] = if s.b[1072] { 1.0 } else { 0.0 };

        s.b[1073] = ((-s.v[897]) < 80.0);
        s.v[1073] = if s.b[1073] { 1.0 } else { 0.0 };

        if (s.b[1072] && s.b[1073]) {
            s.store_ln_one_plus_exp_neg_input(0, 897);
        }

        if (s.b[1072] && (!s.b[1073])) {
            s.store_neg(0, 897);
        }

        if s.b[1072] {
            s.store_add_scaled_inputs3_offset_mixed_iai(899, 878, 1.0, A::div(s.ad_value(897), s.ad_value(894)), (-1.0), 0, 1.0, (-0.6931471805599));
        }

        s.b[1074] = (s.v[897] < 80.0);
        s.v[1074] = if s.b[1074] { 1.0 } else { 0.0 };

        if ((!s.b[1072]) && s.b[1074]) {
            s.store_ln_one_plus_exp(0, 897);
        }

        if ((!s.b[1072]) && (!s.b[1074])) {
            s.copy_ad(0, 897);
        }

        if (!s.b[1072]) {
            s.store_add_scaled_inputs3_offset_mixed_iai(899, 889, 1.0, A::div(s.ad_value(897), s.ad_value(895)), 1.0, 0, 1.0, (-0.6931471805599));
        }

        s.store_add_scaled_inputs3_sqrt_third_mixed_iia(900, 899, 0.5, 254, 0.5, A::offset(A::mul(A::sub(s.ad_value(899), s.ad_value(254)), A::sub(s.ad_value(899), s.ad_value(254))), 4.0), (-0.5));

        s.store_offset_sqrt_ad(901, A::offset(A::div_scaled_inputs2(s.ad_value(254), 2.0, s.ad_value(900), (-2.0), s.ad_value(255), 1.0), 1.0), (-1.0));

        s.store_add_scaled_product_indices(902, 900, 1.0, 255, 901, 1.0);

        s.store_scaled_add_ad(0, A::offset(A::mul(s.ad_value(30), s.ad_value(879)), ((1.0) + (0.5))), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(30), s.ad_value(879)), ((1.0) + ((-0.5))), A::offset(A::mul(s.ad_value(30), s.ad_value(879)), ((1.0) + ((-0.5))))), 0.01)), 0.5);

        s.store_div_from_scalar_offset_ad(903, 1.0, A::mul(s.ad_value(871), s.ad_value(0)), 1.0);

        s.store_div_from_scalar_offset_ad(904, 1.0, A::mul(s.ad_value(872), s.ad_value(0)), 1.0);

        s.store_mul_offset_rhs_ad(0, A::mul3_scaled_output(s.ad_value(329), A::offset(A::sqrt(A::offset(A::div(s.ad_value(340), s.ad_value(329)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(901)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(879)), 1.0);

        s.store_mul(905, 873, 0);

        s.store_mul(906, 874, 0);

        s.store_add_ad_lhs(907, A::add_scaled_product(s.ad_value(902), 1.0, A::add_scaled_inputs3(s.ad_value(878), 1.0, s.ad_value(902), (-1.0), s.ad_value(905), 1.0), s.ad_value(903), 1.0), 341);

        s.store_add_ad_lhs(908, A::add_scaled_product(s.ad_value(902), 1.0, A::add_scaled_inputs3(s.ad_value(889), 1.0, s.ad_value(902), (-1.0), s.ad_value(906), 1.0), s.ad_value(904), 1.0), 341);

        let assign11230_ad_e10630: A = A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(908), 1.0, s.ad_value(25), A::sub(s.ad_value(907), s.ad_value(908)), 1.0), 1.0, s.ad_value(225), 1.0, A::sqrt(A::offset(A::mul(A::sub(A::add_scaled_product(s.ad_value(908), 1.0, s.ad_value(25), A::sub(s.ad_value(907), s.ad_value(908)), 1.0), s.ad_value(225)), A::sub(A::add_scaled_product(s.ad_value(908), 1.0, s.ad_value(25), A::sub(s.ad_value(907), s.ad_value(908)), 1.0), s.ad_value(225))), 0.01)), -1.0);
        s.store_scale_ad(909, assign11230_ad_e10630, 0.5);

        let assign11240_ad_e10664: A = A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(907), 1.0, s.ad_value(26), A::sub(s.ad_value(908), s.ad_value(907)), 1.0), 1.0, s.ad_value(225), 1.0, A::sqrt(A::offset(A::mul(A::sub(A::add_scaled_product(s.ad_value(907), 1.0, s.ad_value(26), A::sub(s.ad_value(908), s.ad_value(907)), 1.0), s.ad_value(225)), A::sub(A::add_scaled_product(s.ad_value(907), 1.0, s.ad_value(26), A::sub(s.ad_value(908), s.ad_value(907)), 1.0), s.ad_value(225))), 0.01)), -1.0);
        s.store_scale_ad(910, assign11240_ad_e10664, 0.5);

        s.store_div(911, 894, 903);

        s.store_div(912, 895, 904);

        s.store_div_from_scalar(885, 1.0, 911);

        s.store_div_from_scalar(886, 1.0, 912);

        s.store_div_from_scalar_add_ad(913, 1.0, A::offset(s.ad_value(885), 1.0), s.ad_value(886));

        s.store_div_ad_rhs(884, 253, A::square(s.ad_value(898)));

        s.store_div_scaled_offset_numerator(881, s.ad_value(911), 1.0, 1.0, A::offset(s.ad_value(912), 1.0), 1.0);

        s.store_ln(882, 881);

        s.b[1075] = (s.v[882] > 1e-8);
        s.v[1075] = if s.b[1075] { 1.0 } else { 0.0 };

        if s.b[1075] {
            s.store_div_scaled_product_offset_denominator(883, s.ad_value(882), A::offset(s.ad_value(881), 1.0), 2.0, s.ad_value(881), (-1.0), 1.0);
        }

        if (!s.b[1075]) {
            s.store_scaled_offset(883, 882, 2.0, 2.0);
        }

        s.store_mul_sub_rhs(914, 913, 909, 910);

    }

    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
    ) {
        s.store_square(915, 914);

        s.store_add_scaled_product_indices(887, 909, 1.0, 914, 885, (-1.0));

        s.store_add_scaled_product_indices(888, 910, 1.0, 914, 886, 1.0);

        s.store_div_from_scalar_offset_input(793, 1.0, 911, 1.0);

        s.store_div_from_scalar_offset_input(794, 1.0, 912, 1.0);

        s.store_offset_ln_ad(796, A::div_scaled_product(A::add_scaled_product(s.ad_value(911), 1.0, s.ad_value(912), s.ad_value(794), 1.0), s.ad_value(883), 1.0, s.ad_value(884), 1.0), 3.0);

        s.store_offset_ln_ad(797, A::div_scaled_product(A::add_scaled_product(s.ad_value(912), 1.0, s.ad_value(911), s.ad_value(793), 1.0), s.ad_value(883), 1.0, s.ad_value(884), 1.0), 3.0);

        s.b[1076] = (((s.v[796] - s.v[887]) * 0.3333333333333) < 80.0);
        s.v[1076] = if s.b[1076] { 1.0 } else { 0.0 };

        if s.b[1076] {
            s.store_ln_one_plus_exp_ad(795, A::sub_scaled_inputs(s.ad_value(796), 0.3333333333333, s.ad_value(887), 0.3333333333333));
        }

        if (!s.b[1076]) {
            s.store_scaled_sub(795, 796, 887, 0.3333333333333);
        }

        s.store_sub_scaled_inputs(800, 796, 1.0, 795, 3.0);

        s.b[1077] = (((s.v[797] - s.v[888]) * 0.3333333333333) < 80.0);
        s.v[1077] = if s.b[1077] { 1.0 } else { 0.0 };

        if s.b[1077] {
            s.store_ln_one_plus_exp_ad(795, A::sub_scaled_inputs(s.ad_value(797), 0.3333333333333, s.ad_value(888), 0.3333333333333));
        }

        if (!s.b[1077]) {
            s.store_scaled_sub(795, 797, 888, 0.3333333333333);
        }

        s.store_sub_scaled_inputs(801, 797, 1.0, 795, 3.0);

        s.store_mul_add_scaled_product_rhs(798, 793, s.ad_value(801), 1.0, s.ad_value(911), s.ad_value(909), 1.0);

        s.store_mul_add_scaled_product_rhs(799, 794, s.ad_value(800), 1.0, s.ad_value(912), s.ad_value(910), 1.0);

        s.b[1078] = (((s.v[796] - s.v[798]) * 0.3333333333333) < 80.0);
        s.v[1078] = if s.b[1078] { 1.0 } else { 0.0 };

        if s.b[1078] {
            s.store_ln_one_plus_exp_ad(795, A::sub_scaled_inputs(s.ad_value(796), 0.3333333333333, s.ad_value(798), 0.3333333333333));
        }

        if (!s.b[1078]) {
            s.store_scaled_sub(795, 796, 798, 0.3333333333333);
        }

        s.store_sub_scaled_inputs(800, 796, 1.0, 795, 3.0);

        s.b[1079] = (((s.v[797] - s.v[799]) * 0.3333333333333) < 80.0);
        s.v[1079] = if s.b[1079] { 1.0 } else { 0.0 };

        if s.b[1079] {
            s.store_ln_one_plus_exp_ad(795, A::sub_scaled_inputs(s.ad_value(797), 0.3333333333333, s.ad_value(799), 0.3333333333333));
        }

        if (!s.b[1079]) {
            s.store_scaled_sub(795, 797, 799, 0.3333333333333);
        }

        s.store_sub_scaled_inputs(801, 797, 1.0, 795, 3.0);

        s.store_sub(916, 909, 800);

        s.store_sub(920, 910, 801);

        s.v[807] = 0.0;

        s.v[810] = 0.0;

        s.store_mul(802, 911, 916);

        s.b[1080] = ((s.v[909] - s.v[916]) < 80.0);
        s.v[1080] = if s.b[1080] { 1.0 } else { 0.0 };

        if s.b[1080] {
            s.store_exp_sub(793, 909, 916);
        }

        if (!s.b[1080]) {
            s.store_scaled_offset_mul_offset_lhs_ad(793, A::sub(s.ad_value(909), s.ad_value(916)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(909), s.ad_value(916)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(909), s.ad_value(916)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        s.store_mul(803, 884, 793);

        s.store_sub_ad_lhs(804, A::square(s.ad_value(802)), 803);

        s.store_add_scaled_product_indices(805, 803, 1.0, 911, 802, 2.0);

        s.store_add_scaled_product_indices(806, 803, (-1.0), 911, 911, 2.0);

        s.b[1081] = (s.v[804] < (-0.005));
        s.v[1081] = if s.b[1081] { 1.0 } else { 0.0 };

        if s.b[1081] {
            s.store_sqrt_abs_ad(807, s.ad_value(804));
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
            s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);
            s.store_mul_add_ad_lhs(809, s.ad_value(804), A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 793);
            s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);
            s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);
            s.store_mul_div_lhs(814, 805, 804, 794);
            s.store_div_ad_lhs(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);
        }

        s.b[1082] = (s.v[804] > 0.005);
        s.v[1082] = if s.b[1082] { 1.0 } else { 0.0 };

        if ((!s.b[1081]) && s.b[1082]) {
            s.store_sqrt_abs_ad(807, s.ad_value(804));
            s.store_exp_neg_input(810, 807);
            s.store_div_scaled_product_offset_rhs(808, s.ad_value(807), s.ad_value(810), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);
            s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);
            s.store_mul_add_ad_lhs(809, s.ad_value(804), A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 793);
            s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);
            s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);
            s.store_mul_div_lhs(814, 805, 804, 794);
            s.store_div_ad_lhs(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);
        }

        if ((!s.b[1081]) && (!s.b[1082])) {
            s.store_offset_scaled_ad(795, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(808, 804, 795, 2.0);
            s.store_offset_scaled_ad(793, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_mul(809, 805, 793);
            s.store_offset_scaled_ad(794, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(811, 806, 793, 1.0, A::square(s.ad_value(805)), 794, (-1.0));
            s.store_scaled_mul(814, 805, 795, (-0.5));
            s.store_add_scaled_product_value_ad(815, A::mul3_scaled_output(s.ad_value(805), s.ad_value(805), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 2.0, A::scale(s.ad_value(804), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 806, 795, (-0.5));
        }

        s.b[1083] = (s.v[804] > 0.005);
        s.v[1083] = if s.b[1083] { 1.0 } else { 0.0 };

        if s.b[1083] {
            s.store_div_scaled_inputs_mixed_ia(794, 804, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0);
            s.store_mul(812, 794, 810);
            s.store_sub_ad_lhs(813, A::ln(s.ad_value(794)), 807);
        }

        s.b[1084] = (s.v[804] < (-0.005));
        s.v[1084] = if s.b[1084] { 1.0 } else { 0.0 };

        if ((!s.b[1083]) && s.b[1084]) {
            s.store_sin_scaled_input(794, 807, 0.5);
            s.store_div_scaled_inputs_mixed_ia(812, 804, -1.0, A::square(s.ad_value(794)), 1.0);
            s.store_ln(813, 812);
        }

        if ((!s.b[1083]) && (!s.b[1084])) {
            s.store_sub_from_scalar_ad(812, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(813, 812);
        }

        s.b[1085] = (((1.01 * s.v[802]) + s.v[808]) > 0.0);
        s.v[1085] = if s.b[1085] { 1.0 } else { 0.0 };

        if s.b[1085] {
            s.store_add(816, 802, 808);
            s.store_add(817, 911, 809);
            s.copy_ad(818, 811);
        }

        if (!s.b[1085]) {
            s.store_div_from_scalar_sub_ad(794, 1.0, s.ad_value(802), s.ad_value(808));
            s.store_sub(795, 809, 911);
            s.store_mul_sub_lhs(816, 803, 812, 794);
            s.store_mul_ad_lhs(817, A::add_scaled_value_products(s.ad_value(803), (-1.0), s.ad_value(795), s.ad_value(816), 1.0, s.ad_value(814), s.ad_value(812), (-1.0)), 794);
            s.store_mul_ad_lhs(818, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(811), s.ad_value(816), 1.0, s.ad_value(795), s.ad_value(817), 2.0), 1.0, s.ad_value(803), 1.0, A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812), (-1.0)), 794);
        }

        s.b[1086] = (s.v[816] > 0.0);
        s.v[1086] = if s.b[1086] { 1.0 } else { 0.0 };

        if s.b[1086] {
            s.store_ln(819, 816);
            s.store_div_from_scalar(793, 1.0, 816);
            s.store_mul(820, 817, 793);
            s.store_add_scaled_square_product_indices(821, 820, (-1.0), 818, 793, 1.0);
        }

        if (!s.b[1086]) {
            s.store_add_offset_lhs_ad_rhs(819, 802, 0.6931471805599, A::ln_scaled_input(s.ad_value(802), -1.0));
            s.store_div_from_scalar(793, 1.0, 916);
            s.store_add(820, 911, 793);
            s.store_mul_neg_lhs(821, 793, 793);
        }

        s.store_sub_ad_lhs(822, A::add_scaled_inputs4(s.ad_value(910), 1.0, s.ad_value(909), (-1.0), s.ad_value(916), 1.0, s.ad_value(819), 2.0), 813);

        s.store_sub_ad_lhs(823, A::scale_offset(s.ad_value(820), 2.0, 1.0), 814);

        s.store_sub_scaled_inputs(824, 821, 2.0, 815, 1.0);

        s.store_add_scaled_product_indices(825, 802, 1.0, 912, 822, 1.0);

        s.store_add_scaled_product_indices(826, 911, 1.0, 912, 823, 1.0);

        s.store_mul(827, 912, 824);

        s.store_add_scaled_product_indices(828, 803, (-1.0), 825, 816, 1.0);

        s.store_add_ad_lhs(829, A::add_scaled_products(s.ad_value(826), s.ad_value(816), 1.0, s.ad_value(825), s.ad_value(817), 1.0), 803);

        s.store_sub_ad_lhs(830, A::add_scaled_products3(s.ad_value(827), s.ad_value(816), 1.0, s.ad_value(826), s.ad_value(817), 2.0, s.ad_value(825), s.ad_value(818), 1.0), 803);

        s.store_add_scaled_square_product_indices(839, 829, 1.0, 828, 830, (-0.5));

        s.store_div_scaled_product3_mixed_iiia(831, 828, 829, 839, -1.0, A::offset(A::square(s.ad_value(839)), 1e-200), 1.0);

        s.store_add(916, 916, 831);

        s.store_mul(802, 911, 916);

        s.store_mul(832, 912, 920);

        s.store_add(825, 802, 832);

        s.store_offset_scaled(833, 825, 0.065345483024, 1.0);

        s.store_add_scaled_product_value_ad(834, A::scale_offset(s.ad_value(825), 8.5797362674, 39.478417604), 1.0, 802, 832, 1.0);

        s.store_add_scaled_product_indices(835, 825, (2.0 * 39.478417604), 802, 832, 39.478417604);

        s.store_sqrt_ad(836, A::add_scaled_square_product(s.ad_value(834), 1.0, s.ad_value(833), s.ad_value(835), (-4.0)));

        s.store_div_scaled_inputs2_indices(804, 836, 1.0, 834, (-1.0), 833, 2.0);

        s.store_sub_ad_lhs(837, A::square(s.ad_value(802)), 804);

        s.b[1087] = (s.v[837] > 0.0);
        s.v[1087] = if s.b[1087] { 1.0 } else { 0.0 };

        if s.b[1087] {
            s.store_mul_add_scaled_inputs3_offset_rhs(828, 837, A::ln(A::div(s.ad_value(837), s.ad_value(884))), 1.0, s.ad_value(909), (-1.0), s.ad_value(916), 1.0, 0.0);
            s.store_add_scaled_product_indices(829, 837, 1.0, 911, 802, 2.0);
        }

        let (assign12530_e11820,) = {
    if s.b[1087] {
        let assign12530_e11816: f64 = (s.v[909] - s.v[916]);
        let assign12530_e11818: f64 = (assign12530_e11816 - s.v[796]);
        (assign12530_e11818,)
    } else {
        (s.v[838],)
    }
};
        s.v[838] = assign12530_e11820;

        s.b[1088] = ((((s.v[828] < 0.0) && (s.v[829] > 0.0)) && (((s.v[838] + 2.3025850929941) + ((s.v[911]) as f64).ln()) > 0.0)) || (s.v[838] > 1.0));
        s.v[1088] = if s.b[1088] { 1.0 } else { 0.0 };

        if (s.b[1087] && s.b[1088]) {
            s.store_sub_div_rhs_indices(916, 916, 828, 829);
        }

        s.store_mul(802, 911, 916);

        s.store_mul(832, 912, 920);

        s.store_add(825, 802, 832);

        s.store_offset_scaled(833, 825, 0.065345483024, 1.0);

        s.store_add_scaled_product_value_ad(834, A::scale_offset(s.ad_value(825), 8.5797362674, 39.478417604), 1.0, 802, 832, 1.0);

        s.store_add_scaled_product_indices(835, 825, (2.0 * 39.478417604), 802, 832, 39.478417604);

        s.store_sqrt_ad(836, A::add_scaled_square_product(s.ad_value(834), 1.0, s.ad_value(833), s.ad_value(835), (-4.0)));

        s.store_div_scaled_inputs2_indices(804, 836, 1.0, 834, (-1.0), 833, 2.0);

        s.b[1089] = (s.v[804] < (-0.005));
        s.v[1089] = if s.b[1089] { 1.0 } else { 0.0 };

        if s.b[1089] {
            s.store_sqrt_abs_ad(807, s.ad_value(804));
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
            s.store_div_scaled_inputs2_mixed_iai(809, 804, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 0.25, 804, 1.0);
        }

        s.b[1090] = (s.v[804] > 0.005);
        s.v[1090] = if s.b[1090] { 1.0 } else { 0.0 };

        if ((!s.b[1089]) && s.b[1090]) {
            s.store_sqrt_abs_ad(807, s.ad_value(804));
            s.store_exp_neg_input(810, 807);
            s.store_div_scaled_product_offset_rhs(808, s.ad_value(807), s.ad_value(810), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);
            s.store_div_scaled_inputs2_mixed_iai(809, 804, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 0.25, 804, 1.0);
        }

        if ((!s.b[1089]) && (!s.b[1090])) {
            s.store_offset_ad(808, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);
            s.store_offset_scaled_ad(809, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
        }

        s.store_sub_ad_rhs(804, 804, A::div_scaled_inputs2(A::add_scaled_products(s.ad_value(825), s.ad_value(808), 1.0, s.ad_value(802), s.ad_value(832), 1.0), 1.0, s.ad_value(804), 1.0, A::offset(A::mul(s.ad_value(825), s.ad_value(809)), 1.0), 1.0));

        s.store_sub_ad_lhs(837, A::square(s.ad_value(802)), 804);

        s.b[1091] = (s.v[837] > 0.0);
        s.v[1091] = if s.b[1091] { 1.0 } else { 0.0 };

        if s.b[1091] {
            s.store_mul_add_scaled_inputs3_offset_rhs(828, 837, A::ln(A::div(s.ad_value(837), s.ad_value(884))), 1.0, s.ad_value(909), (-1.0), s.ad_value(916), 1.0, 0.0);
            s.store_add_scaled_product_indices(829, 837, 1.0, 911, 802, 2.0);
        }

        let (assign12800_e12093,) = {
    if s.b[1091] {
        let assign12800_e12089: f64 = (s.v[909] - s.v[916]);
        let assign12800_e12091: f64 = (assign12800_e12089 - s.v[796]);
        (assign12800_e12091,)
    } else {
        (s.v[838],)
    }
};
        s.v[838] = assign12800_e12093;

        s.b[1092] = ((((s.v[828] < 0.0) && (s.v[829] > 0.0)) && (((s.v[838] + 2.3025850929941) + ((s.v[911]) as f64).ln()) > 0.0)) || (s.v[838] > 1.0));
        s.v[1092] = if s.b[1092] { 1.0 } else { 0.0 };

        if (s.b[1091] && s.b[1092]) {
            s.store_sub_div_rhs_indices(916, 916, 828, 829);
        }

        s.store_mul(802, 911, 916);

        s.b[1093] = ((s.v[909] - s.v[916]) < 80.0);
        s.v[1093] = if s.b[1093] { 1.0 } else { 0.0 };

        if s.b[1093] {
            s.store_exp_sub(793, 909, 916);
        }

        if (!s.b[1093]) {
            s.store_scaled_offset_mul_offset_lhs_ad(793, A::sub(s.ad_value(909), s.ad_value(916)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(909), s.ad_value(916)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(909), s.ad_value(916)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        s.store_mul(803, 884, 793);

        s.store_sub_ad_lhs(804, A::square(s.ad_value(802)), 803);

        s.store_add_scaled_product_indices(805, 803, 1.0, 911, 802, 2.0);

        s.store_add_scaled_product_indices(806, 803, (-1.0), 911, 911, 2.0);

        s.b[1094] = (s.v[804] < (-0.005));
        s.v[1094] = if s.b[1094] { 1.0 } else { 0.0 };

        if s.b[1094] {
            s.store_sqrt_abs_ad(807, s.ad_value(804));
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
            s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1094] {
            s.store_mul_add_ad_lhs(809, s.ad_value(804), A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 793);
            s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);
            s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);
            s.store_mul_div_lhs(814, 805, 804, 794);
            s.store_div_ad_lhs(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);
        }

        s.b[1095] = (s.v[804] > 0.005);
        s.v[1095] = if s.b[1095] { 1.0 } else { 0.0 };

        if ((!s.b[1094]) && s.b[1095]) {
            s.store_sqrt_abs_ad(807, s.ad_value(804));
            s.store_exp_neg_input(810, 807);
            s.store_div_scaled_product_offset_rhs(808, s.ad_value(807), s.ad_value(810), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);
            s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);
            s.store_mul_add_ad_lhs(809, s.ad_value(804), A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 793);
            s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);
            s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);
            s.store_mul_div_lhs(814, 805, 804, 794);
            s.store_div_ad_lhs(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);
        }

        if ((!s.b[1094]) && (!s.b[1095])) {
            s.store_offset_scaled_ad(795, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(808, 804, 795, 2.0);
            s.store_offset_scaled_ad(793, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_mul(809, 805, 793);
            s.store_offset_scaled_ad(794, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(811, 806, 793, 1.0, A::square(s.ad_value(805)), 794, (-1.0));
            s.store_scaled_mul(814, 805, 795, (-0.5));
            s.store_add_scaled_product_value_ad(815, A::mul3_scaled_output(s.ad_value(805), s.ad_value(805), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 2.0, A::scale(s.ad_value(804), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 806, 795, (-0.5));
        }

        s.b[1096] = (s.v[804] > 0.005);
        s.v[1096] = if s.b[1096] { 1.0 } else { 0.0 };

        if s.b[1096] {
            s.store_div_scaled_inputs_mixed_ia(794, 804, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0);
            s.store_mul(812, 794, 810);
            s.store_sub_ad_lhs(813, A::ln(s.ad_value(794)), 807);
        }

        s.b[1097] = (s.v[804] < (-0.005));
        s.v[1097] = if s.b[1097] { 1.0 } else { 0.0 };

        if ((!s.b[1096]) && s.b[1097]) {
            s.store_sin_scaled_input(794, 807, 0.5);
            s.store_div_scaled_inputs_mixed_ia(812, 804, -1.0, A::square(s.ad_value(794)), 1.0);
            s.store_ln(813, 812);
        }

        if ((!s.b[1096]) && (!s.b[1097])) {
            s.store_sub_from_scalar_ad(812, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(813, 812);
        }

        s.b[1098] = (((1.01 * s.v[802]) + s.v[808]) > 0.0);
        s.v[1098] = if s.b[1098] { 1.0 } else { 0.0 };

        if s.b[1098] {
            s.store_add(816, 802, 808);
            s.store_add(817, 911, 809);
            s.copy_ad(818, 811);
        }

        if (!s.b[1098]) {
            s.store_div_from_scalar_sub_ad(794, 1.0, s.ad_value(802), s.ad_value(808));
            s.store_sub(795, 809, 911);
            s.store_mul_sub_lhs(816, 803, 812, 794);
            s.store_mul_ad_lhs(817, A::add_scaled_value_products(s.ad_value(803), (-1.0), s.ad_value(795), s.ad_value(816), 1.0, s.ad_value(814), s.ad_value(812), (-1.0)), 794);
            s.store_mul_ad_lhs(818, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(811), s.ad_value(816), 1.0, s.ad_value(795), s.ad_value(817), 2.0), 1.0, s.ad_value(803), 1.0, A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812), (-1.0)), 794);
        }

        s.b[1099] = (s.v[816] > 0.0);
        s.v[1099] = if s.b[1099] { 1.0 } else { 0.0 };

        if s.b[1099] {
            s.store_ln(819, 816);
            s.store_div_from_scalar(793, 1.0, 816);
            s.store_mul(820, 817, 793);
            s.store_add_scaled_square_product_indices(821, 820, (-1.0), 818, 793, 1.0);
        }

        if (!s.b[1099]) {
            s.store_add_offset_lhs_ad_rhs(819, 802, 0.6931471805599, A::ln_scaled_input(s.ad_value(802), -1.0));
            s.store_div_from_scalar(793, 1.0, 916);
            s.store_add(820, 911, 793);
            s.store_mul_neg_lhs(821, 793, 793);
        }

        s.store_sub_ad_lhs(822, A::add_scaled_inputs4(s.ad_value(910), 1.0, s.ad_value(909), (-1.0), s.ad_value(916), 1.0, s.ad_value(819), 2.0), 813);

        s.store_sub_ad_lhs(823, A::scale_offset(s.ad_value(820), 2.0, 1.0), 814);

        s.store_sub_scaled_inputs(824, 821, 2.0, 815, 1.0);

        s.store_add_scaled_product_indices(825, 802, 1.0, 912, 822, 1.0);

        s.store_add_scaled_product_indices(826, 911, 1.0, 912, 823, 1.0);

        s.store_mul(827, 912, 824);

        s.store_add_scaled_product_indices(828, 803, (-1.0), 825, 816, 1.0);

        s.store_add_ad_lhs(829, A::add_scaled_products(s.ad_value(826), s.ad_value(816), 1.0, s.ad_value(825), s.ad_value(817), 1.0), 803);

        s.store_sub_ad_lhs(830, A::add_scaled_products3(s.ad_value(827), s.ad_value(816), 1.0, s.ad_value(826), s.ad_value(817), 2.0, s.ad_value(825), s.ad_value(818), 1.0), 803);

        s.store_add_scaled_square_product_indices(839, 829, 1.0, 828, 830, (-0.5));

        s.store_div_scaled_product3_mixed_iiia(831, 828, 829, 839, -1.0, A::offset(A::square(s.ad_value(839)), 1e-200), 1.0);

        s.store_add(916, 916, 831);

        s.store_mul(802, 911, 916);

        s.b[1100] = ((s.v[909] - s.v[916]) < 80.0);
        s.v[1100] = if s.b[1100] { 1.0 } else { 0.0 };

        if s.b[1100] {
            s.store_exp_sub(793, 909, 916);
        }

        if (!s.b[1100]) {
            s.store_scaled_offset_mul_offset_lhs_ad(793, A::sub(s.ad_value(909), s.ad_value(916)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(909), s.ad_value(916)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(909), s.ad_value(916)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        s.store_mul(803, 884, 793);

        s.store_sub_ad_lhs(804, A::square(s.ad_value(802)), 803);

        s.store_add_scaled_product_indices(805, 803, 1.0, 911, 802, 2.0);

        s.store_add_scaled_product_indices(806, 803, (-1.0), 911, 911, 2.0);

        s.b[1101] = (s.v[804] < (-0.005));
        s.v[1101] = if s.b[1101] { 1.0 } else { 0.0 };

        if s.b[1101] {
            s.store_sqrt_abs_ad(807, s.ad_value(804));
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
            s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);
            s.store_mul_add_ad_lhs(809, s.ad_value(804), A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 793);
            s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);
            s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);
            s.store_mul_div_lhs(814, 805, 804, 794);
            s.store_div_ad_lhs(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);
        }

        s.b[1102] = (s.v[804] > 0.005);
        s.v[1102] = if s.b[1102] { 1.0 } else { 0.0 };

        if ((!s.b[1101]) && s.b[1102]) {
            s.store_sqrt_abs_ad(807, s.ad_value(804));
            s.store_exp_neg_input(810, 807);
            s.store_div_scaled_product_offset_rhs(808, s.ad_value(807), s.ad_value(810), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);
            s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);
            s.store_mul_add_ad_lhs(809, s.ad_value(804), A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 793);
            s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);
            s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);
            s.store_mul_div_lhs(814, 805, 804, 794);
            s.store_div_ad_lhs(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);
        }

        if ((!s.b[1101]) && (!s.b[1102])) {
            s.store_offset_scaled_ad(795, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(808, 804, 795, 2.0);
            s.store_offset_scaled_ad(793, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_mul(809, 805, 793);
            s.store_offset_scaled_ad(794, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(811, 806, 793, 1.0, A::square(s.ad_value(805)), 794, (-1.0));
            s.store_scaled_mul(814, 805, 795, (-0.5));
            s.store_add_scaled_product_value_ad(815, A::mul3_scaled_output(s.ad_value(805), s.ad_value(805), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 2.0, A::scale(s.ad_value(804), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 806, 795, (-0.5));
        }

        s.b[1103] = (s.v[804] > 0.005);
        s.v[1103] = if s.b[1103] { 1.0 } else { 0.0 };

        if s.b[1103] {
            s.store_div_scaled_inputs_mixed_ia(794, 804, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0);
            s.store_mul(812, 794, 810);
            s.store_sub_ad_lhs(813, A::ln(s.ad_value(794)), 807);
        }

        s.b[1104] = (s.v[804] < (-0.005));
        s.v[1104] = if s.b[1104] { 1.0 } else { 0.0 };

        if ((!s.b[1103]) && s.b[1104]) {
            s.store_sin_scaled_input(794, 807, 0.5);
            s.store_div_scaled_inputs_mixed_ia(812, 804, -1.0, A::square(s.ad_value(794)), 1.0);
            s.store_ln(813, 812);
        }

        if ((!s.b[1103]) && (!s.b[1104])) {
            s.store_sub_from_scalar_ad(812, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(813, 812);
        }

        s.b[1105] = (((1.01 * s.v[802]) + s.v[808]) > 0.0);
        s.v[1105] = if s.b[1105] { 1.0 } else { 0.0 };

        if s.b[1105] {
            s.store_add(816, 802, 808);
            s.store_add(817, 911, 809);
            s.copy_ad(818, 811);
        }

        if (!s.b[1105]) {
            s.store_div_from_scalar_sub_ad(794, 1.0, s.ad_value(802), s.ad_value(808));
            s.store_sub(795, 809, 911);
            s.store_mul_sub_lhs(816, 803, 812, 794);
            s.store_mul_ad_lhs(817, A::add_scaled_value_products(s.ad_value(803), (-1.0), s.ad_value(795), s.ad_value(816), 1.0, s.ad_value(814), s.ad_value(812), (-1.0)), 794);
            s.store_mul_ad_lhs(818, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(811), s.ad_value(816), 1.0, s.ad_value(795), s.ad_value(817), 2.0), 1.0, s.ad_value(803), 1.0, A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812), (-1.0)), 794);
        }

        s.b[1106] = (s.v[816] > 0.0);
        s.v[1106] = if s.b[1106] { 1.0 } else { 0.0 };

        if s.b[1106] {
            s.store_ln(819, 816);
            s.store_div_from_scalar(793, 1.0, 816);
            s.store_mul(820, 817, 793);
            s.store_add_scaled_square_product_indices(821, 820, (-1.0), 818, 793, 1.0);
        }

        if (!s.b[1106]) {
            s.store_add_offset_lhs_ad_rhs(819, 802, 0.6931471805599, A::ln_scaled_input(s.ad_value(802), -1.0));
            s.store_div_from_scalar(793, 1.0, 916);
            s.store_add(820, 911, 793);
            s.store_mul_neg_lhs(821, 793, 793);
        }

        s.store_sub_ad_lhs(822, A::add_scaled_inputs4(s.ad_value(910), 1.0, s.ad_value(909), (-1.0), s.ad_value(916), 1.0, s.ad_value(819), 2.0), 813);

        s.store_sub_ad_lhs(823, A::scale_offset(s.ad_value(820), 2.0, 1.0), 814);

        s.store_sub_scaled_inputs(824, 821, 2.0, 815, 1.0);

        s.store_add_scaled_product_indices(825, 802, 1.0, 912, 822, 1.0);

        s.store_add_scaled_product_indices(826, 911, 1.0, 912, 823, 1.0);

        s.store_mul(827, 912, 824);

        s.store_add_scaled_product_indices(828, 803, (-1.0), 825, 816, 1.0);

        s.store_add_ad_lhs(829, A::add_scaled_products(s.ad_value(826), s.ad_value(816), 1.0, s.ad_value(825), s.ad_value(817), 1.0), 803);

        s.store_sub_ad_lhs(830, A::add_scaled_products3(s.ad_value(827), s.ad_value(816), 1.0, s.ad_value(826), s.ad_value(817), 2.0, s.ad_value(825), s.ad_value(818), 1.0), 803);

        s.store_add_scaled_square_product_indices(839, 829, 1.0, 828, 830, (-0.5));

        s.store_div_scaled_product3_mixed_iiia(831, 828, 829, 839, -1.0, A::offset(A::square(s.ad_value(839)), 1e-200), 1.0);

        s.store_add(916, 916, 831);

        s.b[1107] = (p.p10 == 1.0);
        s.v[1107] = if s.b[1107] { 1.0 } else { 0.0 };

        s.b[1108] = (((s.v[831]) as f64).abs() > 0.01);
        s.v[1108] = if s.b[1108] { 1.0 } else { 0.0 };

        if (s.b[1107] && s.b[1108]) {
            s.store_mul(802, 911, 916);
        }

        s.b[1109] = ((s.v[909] - s.v[916]) < 80.0);
        s.v[1109] = if s.b[1109] { 1.0 } else { 0.0 };

        if ((s.b[1107] && s.b[1108]) && s.b[1109]) {
            s.store_exp_sub(793, 909, 916);
        }

        if ((s.b[1107] && s.b[1108]) && (!s.b[1109])) {
            s.store_scaled_offset_mul_offset_lhs_ad(793, A::sub(s.ad_value(909), s.ad_value(916)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(909), s.ad_value(916)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(909), s.ad_value(916)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if (s.b[1107] && s.b[1108]) {
            s.store_mul(803, 884, 793);
            s.store_sub_ad_lhs(804, A::square(s.ad_value(802)), 803);
        }

    }

    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
    ) {
        if (s.b[1107] && s.b[1108]) {
            s.store_add_scaled_product_indices(805, 803, 1.0, 911, 802, 2.0);
            s.store_add_scaled_product_indices(806, 803, (-1.0), 911, 911, 2.0);
        }

        s.b[1110] = (s.v[804] < (-0.005));
        s.v[1110] = if s.b[1110] { 1.0 } else { 0.0 };

        if ((s.b[1107] && s.b[1108]) && s.b[1110]) {
            s.store_sqrt_abs_ad(807, s.ad_value(804));
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
            s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);
            s.store_mul_add_ad_lhs(809, s.ad_value(804), A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 793);
            s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);
            s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);
            s.store_mul_div_lhs(814, 805, 804, 794);
            s.store_div_ad_lhs(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);
        }

        s.b[1111] = (s.v[804] > 0.005);
        s.v[1111] = if s.b[1111] { 1.0 } else { 0.0 };

        if (((s.b[1107] && s.b[1108]) && (!s.b[1110])) && s.b[1111]) {
            s.store_sqrt_abs_ad(807, s.ad_value(804));
            s.store_exp_neg_input(810, 807);
            s.store_div_scaled_product_offset_rhs(808, s.ad_value(807), s.ad_value(810), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);
            s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);
            s.store_mul_add_ad_lhs(809, s.ad_value(804), A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 793);
            s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);
            s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);
            s.store_mul_div_lhs(814, 805, 804, 794);
            s.store_div_ad_lhs(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);
        }

        if (((s.b[1107] && s.b[1108]) && (!s.b[1110])) && (!s.b[1111])) {
            s.store_offset_scaled_ad(795, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(808, 804, 795, 2.0);
            s.store_offset_scaled_ad(793, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_mul(809, 805, 793);
            s.store_offset_scaled_ad(794, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(811, 806, 793, 1.0, A::square(s.ad_value(805)), 794, (-1.0));
            s.store_scaled_mul(814, 805, 795, (-0.5));
            s.store_add_scaled_product_value_ad(815, A::mul3_scaled_output(s.ad_value(805), s.ad_value(805), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 2.0, A::scale(s.ad_value(804), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 806, 795, (-0.5));
        }

        s.b[1112] = (s.v[804] > 0.005);
        s.v[1112] = if s.b[1112] { 1.0 } else { 0.0 };

        if ((s.b[1107] && s.b[1108]) && s.b[1112]) {
            s.store_div_scaled_inputs_mixed_ia(794, 804, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0);
            s.store_mul(812, 794, 810);
            s.store_sub_ad_lhs(813, A::ln(s.ad_value(794)), 807);
        }

        s.b[1113] = (s.v[804] < (-0.005));
        s.v[1113] = if s.b[1113] { 1.0 } else { 0.0 };

        if (((s.b[1107] && s.b[1108]) && (!s.b[1112])) && s.b[1113]) {
            s.store_sin_scaled_input(794, 807, 0.5);
            s.store_div_scaled_inputs_mixed_ia(812, 804, -1.0, A::square(s.ad_value(794)), 1.0);
            s.store_ln(813, 812);
        }

        if (((s.b[1107] && s.b[1108]) && (!s.b[1112])) && (!s.b[1113])) {
            s.store_sub_from_scalar_ad(812, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(813, 812);
        }

        s.b[1114] = (((1.01 * s.v[802]) + s.v[808]) > 0.0);
        s.v[1114] = if s.b[1114] { 1.0 } else { 0.0 };

        if ((s.b[1107] && s.b[1108]) && s.b[1114]) {
            s.store_add(816, 802, 808);
            s.store_add(817, 911, 809);
            s.copy_ad(818, 811);
        }

        if ((s.b[1107] && s.b[1108]) && (!s.b[1114])) {
            s.store_div_from_scalar_sub_ad(794, 1.0, s.ad_value(802), s.ad_value(808));
            s.store_sub(795, 809, 911);
            s.store_mul_sub_lhs(816, 803, 812, 794);
            s.store_mul_ad_lhs(817, A::add_scaled_value_products(s.ad_value(803), (-1.0), s.ad_value(795), s.ad_value(816), 1.0, s.ad_value(814), s.ad_value(812), (-1.0)), 794);
            s.store_mul_ad_lhs(818, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(811), s.ad_value(816), 1.0, s.ad_value(795), s.ad_value(817), 2.0), 1.0, s.ad_value(803), 1.0, A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812), (-1.0)), 794);
        }

        s.b[1115] = (s.v[816] > 0.0);
        s.v[1115] = if s.b[1115] { 1.0 } else { 0.0 };

        if ((s.b[1107] && s.b[1108]) && s.b[1115]) {
            s.store_ln(819, 816);
            s.store_div_from_scalar(793, 1.0, 816);
            s.store_mul(820, 817, 793);
            s.store_add_scaled_square_product_indices(821, 820, (-1.0), 818, 793, 1.0);
        }

        if ((s.b[1107] && s.b[1108]) && (!s.b[1115])) {
            s.store_add_offset_lhs_ad_rhs(819, 802, 0.6931471805599, A::ln_scaled_input(s.ad_value(802), -1.0));
            s.store_div_from_scalar(793, 1.0, 916);
            s.store_add(820, 911, 793);
            s.store_mul_neg_lhs(821, 793, 793);
        }

        if (s.b[1107] && s.b[1108]) {
            s.store_sub_ad_lhs(822, A::add_scaled_inputs4(s.ad_value(910), 1.0, s.ad_value(909), (-1.0), s.ad_value(916), 1.0, s.ad_value(819), 2.0), 813);
            s.store_sub_ad_lhs(823, A::scale_offset(s.ad_value(820), 2.0, 1.0), 814);
            s.store_sub_scaled_inputs(824, 821, 2.0, 815, 1.0);
            s.store_add_scaled_product_indices(825, 802, 1.0, 912, 822, 1.0);
            s.store_add_scaled_product_indices(826, 911, 1.0, 912, 823, 1.0);
            s.store_mul(827, 912, 824);
            s.store_add_scaled_product_indices(828, 803, (-1.0), 825, 816, 1.0);
            s.store_add_ad_lhs(829, A::add_scaled_products(s.ad_value(826), s.ad_value(816), 1.0, s.ad_value(825), s.ad_value(817), 1.0), 803);
            s.store_sub_ad_lhs(830, A::add_scaled_products3(s.ad_value(827), s.ad_value(816), 1.0, s.ad_value(826), s.ad_value(817), 2.0, s.ad_value(825), s.ad_value(818), 1.0), 803);
            s.store_add_scaled_square_product_indices(839, 829, 1.0, 828, 830, (-0.5));
            s.store_div_scaled_product3_mixed_iiia(831, 828, 829, 839, -1.0, A::offset(A::square(s.ad_value(839)), 1e-200), 1.0);
            s.store_add(916, 916, 831);
        }

        s.store_mul(918, 911, 916);

        s.b[1116] = ((s.v[909] - s.v[916]) < 80.0);
        s.v[1116] = if s.b[1116] { 1.0 } else { 0.0 };

        if s.b[1116] {
            s.store_exp_sub(793, 909, 916);
        }

        if (!s.b[1116]) {
            s.store_scaled_offset_mul_offset_lhs_ad(793, A::sub(s.ad_value(909), s.ad_value(916)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(909), s.ad_value(916)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(909), s.ad_value(916)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        s.store_mul(922, 884, 793);

        s.store_sub_ad_lhs(921, A::square(s.ad_value(918)), 922);

        s.b[1117] = (s.v[922] <= 0.0);
        s.v[1117] = if s.b[1117] { 1.0 } else { 0.0 };

        if s.b[1117] {
            s.store_scalar(917, 1e-80);
            s.store_sub(919, 917, 918);
            s.store_div(920, 919, 912);
        }

        s.b[1118] = (s.v[921] < (-0.005));
        s.v[1118] = if s.b[1118] { 1.0 } else { 0.0 };

        if ((!s.b[1117]) && s.b[1118]) {
            s.store_sqrt_abs_ad(807, s.ad_value(921));
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
        }

        s.b[1119] = (s.v[921] > 0.005);
        s.v[1119] = if s.b[1119] { 1.0 } else { 0.0 };

        if (((!s.b[1117]) && (!s.b[1118])) && s.b[1119]) {
            s.store_sqrt_abs_ad(807, s.ad_value(921));
            s.store_exp_neg_input(810, 807);
            s.store_div_scaled_product_offset_rhs(808, s.ad_value(807), s.ad_value(810), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);
        }

        if (((!s.b[1117]) && (!s.b[1118])) && (!s.b[1119])) {
            s.store_offset_ad(808, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(921), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(921), 1.0, A::scale(s.ad_value(921), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);
        }

        s.b[1120] = (((1.01 * s.v[918]) + s.v[808]) > 0.0);
        s.v[1120] = if s.b[1120] { 1.0 } else { 0.0 };

        if ((!s.b[1117]) && s.b[1120]) {
            s.store_add(793, 918, 808);
        }

        s.b[1121] = ((s.v[922] * s.v[918]) < (((0.9 * s.v[918]) * s.v[918]) * s.v[793]));
        s.v[1121] = if s.b[1121] { 1.0 } else { 0.0 };

        if (((!s.b[1117]) && s.b[1120]) && s.b[1121]) {
            s.store_offset_div(917, 922, 793, 1e-80);
            s.store_sub(919, 917, 918);
            s.store_div(920, 919, 912);
        }

        s.b[1122] = (s.v[921] > 0.005);
        s.v[1122] = if s.b[1122] { 1.0 } else { 0.0 };

        if ((((!s.b[1117]) && s.b[1120]) && (!s.b[1121])) && s.b[1122]) {
            s.store_sub_ad_lhs(794, A::ln(A::div_scaled_inputs(s.ad_value(921), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0)), 807);
        }

        s.b[1123] = (s.v[921] < (-0.005));
        s.v[1123] = if s.b[1123] { 1.0 } else { 0.0 };

        if (((((!s.b[1117]) && s.b[1120]) && (!s.b[1121])) && (!s.b[1122])) && s.b[1123]) {
            s.store_sin_scaled_input(795, 807, 0.5);
            s.store_ln_div_scaled_input_square_denominator(794, 921, -1.0, 795, 1.0);
        }

        if (((((!s.b[1117]) && s.b[1120]) && (!s.b[1121])) && (!s.b[1122])) && (!s.b[1123])) {
            s.store_ln_ad(794, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(921), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(921), 1.0, A::scale(s.ad_value(921), 0.0396825396825397), 0.05), 0.3333333333333)));
        }

        if (((!s.b[1117]) && s.b[1120]) && (!s.b[1121])) {
            s.store_sub_ad_lhs(920, A::add_scaled_inputs4(s.ad_value(910), 1.0, s.ad_value(909), (-1.0), s.ad_value(916), 1.0, A::ln(s.ad_value(793)), 2.0), 794);
            s.store_mul(919, 912, 920);
            s.store_add(917, 918, 919);
        }

        s.b[1124] = (s.v[921] > 0.005);
        s.v[1124] = if s.b[1124] { 1.0 } else { 0.0 };

        s.b[1125] = (((s.v[916] - s.v[909]) - s.v[807]) < 80.0);
        s.v[1125] = if s.b[1125] { 1.0 } else { 0.0 };

        if ((((!s.b[1117]) && (!s.b[1120])) && s.b[1124]) && s.b[1125]) {
            s.store_exp_ad(795, A::add_scaled_inputs3(s.ad_value(916), 1.0, s.ad_value(909), (-1.0), s.ad_value(807), -1.0));
        }

        if ((((!s.b[1117]) && (!s.b[1120])) && s.b[1124]) && (!s.b[1125])) {
            let assign15460_ad_e15280: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(916), 1.0, s.ad_value(909), (-1.0), s.ad_value(807), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(916), 1.0, s.ad_value(909), (-1.0), s.ad_value(807), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(916), 1.0, s.ad_value(909), (-1.0), s.ad_value(807), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(795, assign15460_ad_e15280, 1.0, 5.54062e34);
        }

        if (((!s.b[1117]) && (!s.b[1120])) && s.b[1124]) {
            s.store_div(794, 795, 884);
            s.store_div_scaled_product_denominator_ad(793, 921, 794, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0);
        }

        s.b[1126] = (s.v[921] < (-0.005));
        s.v[1126] = if s.b[1126] { 1.0 } else { 0.0 };

        if ((((!s.b[1117]) && (!s.b[1120])) && (!s.b[1124])) && s.b[1126]) {
            s.store_sin_scaled_input(794, 807, 0.5);
            s.store_div_scaled_value_by_product(793, s.ad_value(921), -1.0, A::square(s.ad_value(794)), s.ad_value(922), 1.0);
        }

        if ((((!s.b[1117]) && (!s.b[1120])) && (!s.b[1124])) && (!s.b[1126])) {
            s.store_div_ad_lhs(793, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(921), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(921), 1.0, A::scale(s.ad_value(921), 0.0396825396825397), 0.05), 0.3333333333333)), 922);
        }

        if ((!s.b[1117]) && (!s.b[1120])) {
            s.store_offset_div_scaled_inputs2(917, s.ad_value(918), 1.0, s.ad_value(808), (-1.0), A::sub_from_scalar(1.0, s.ad_value(793)), 1.0, 1e-80);
            s.store_sub(919, 917, 918);
            s.store_div(920, 919, 912);
        }

        s.b[1127] = ((s.v[910] - s.v[920]) < 80.0);
        s.v[1127] = if s.b[1127] { 1.0 } else { 0.0 };

        if s.b[1127] {
            s.store_exp_sub(793, 910, 920);
        }

        if (!s.b[1127]) {
            s.store_scaled_offset_mul_offset_lhs_ad(793, A::sub(s.ad_value(910), s.ad_value(920)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(910), s.ad_value(920)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(910), s.ad_value(920)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        s.store_mul(923, 884, 793);

        s.v[926] = 0.0;

        s.v[927] = 0.0;

        s.v[924] = 0.0;

        s.v[925] = 0.0;

        s.v[928] = 0.0;

        s.v[929] = 0.0;

        s.b[1128] = (s.v[917] > 1e-6);
        s.v[1128] = if s.b[1128] { 1.0 } else { 0.0 };

        if s.b[1128] {
            s.store_mul(924, 922, 885);
            s.store_mul(925, 923, 886);
            s.store_add_scaled_inputs(926, 924, 1.0, 918, 2.0);
            s.store_add_scaled_inputs(927, 925, 1.0, 919, 2.0);
            s.store_add_scaled_inputs3_indices(928, 917, 2.0, 924, 1.0, 925, 1.0);
        }

        s.b[1129] = (((s.v[921]) as f64).abs() > 0.005);
        s.v[1129] = if s.b[1129] { 1.0 } else { 0.0 };

        if (s.b[1128] && s.b[1129]) {
            s.store_add_scaled_products3(2, s.ad_value(926), s.ad_value(927), 1.0, A::offset(s.ad_value(916), 2.0), s.ad_value(927), 2.0, A::offset(s.ad_value(920), 2.0), s.ad_value(926), 2.0);
            s.store_div_scaled_product_by_product(929, s.ad_value(921), s.ad_value(928), (-4.0), s.ad_value(917), s.ad_value(2), 1.0);
        }

        if (s.b[1128] && (!s.b[1129])) {
            s.store_offset_scaled_ad(2, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(921), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(921), 1.0, A::scale(s.ad_value(921), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_add_scaled_products3(3, s.ad_value(926), s.ad_value(922), 1.0, s.ad_value(927), s.ad_value(923), 1.0, A::mul3(s.ad_value(926), s.ad_value(927), s.ad_value(917)), A::offset(A::mul(s.ad_value(917), s.ad_value(2)), 1.0), 1.0);
            s.store_div_scaled_product3_by_product(929, s.ad_value(922), s.ad_value(923), s.ad_value(928), 1.0, s.ad_value(917), s.ad_value(3), 1.0);
        }

        s.store_ln(930, 917);

    }

    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
    ) {
        s.b[1130] = ((s.v[918] / 2.0) < 80.0);
        s.v[1130] = if s.b[1130] { 1.0 } else { 0.0 };

        if s.b[1130] {
            s.store_ln_one_plus_exp_scaled_input(2, 918, 0.5);
        }

        if (!s.b[1130]) {
            s.store_scale(2, 918, 0.5);
        }

        s.store_scale(931, 2, 2.0);

        s.b[1131] = ((s.v[919] / 2.0) < 80.0);
        s.v[1131] = if s.b[1131] { 1.0 } else { 0.0 };

        if s.b[1131] {
            s.store_ln_one_plus_exp_scaled_input(3, 919, 0.5);
        }

        if (!s.b[1131]) {
            s.store_scale(3, 919, 0.5);
        }

        s.store_scale(932, 3, 2.0);

        s.store_sub(933, 932, 919);

        s.store_sub(934, 931, 918);

        s.store_add_scaled_products_indices(935, 270, 931, 1.0, 271, 933, 1.0);

        s.store_add_scaled_products_indices(936, 270, 932, 1.0, 271, 934, 1.0);

        s.store_div_add_scaled_inputs_rhs_indices(0, 917, 931, 1.0, 932, 1.0);

        s.store_mul(937, 931, 0);

        s.store_mul(938, 932, 0);

        s.store_mul_ad_product_rhs(939, 931, s.ad_value(191), A::exp(A::mul(s.ad_value(40), s.ad_value(295))));

        s.store_mul_ad_product_rhs(940, 932, s.ad_value(192), A::exp(A::mul(s.ad_value(40), s.ad_value(295))));

        s.store_mul_add_scaled_product_rhs(2, 50, s.ad_value(933), 1.0, s.ad_value(51), s.ad_value(934), 1.0);

        s.store_scaled_add_offset_sqrt_square_offset(3, 2, 1.0, 1.0, 0.01, 0.5);

        s.store_scaled_add_ad(4, A::scale_offset(s.ad_value(2), 0.2, 1.0), A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(2), 0.2, 1.0), A::scale_offset(s.ad_value(2), 0.2, 1.0)), 0.01)), 0.5);

        s.store_div(941, 3, 4);

        s.store_mul_ad_product_rhs(942, 33, A::add_scaled_product(A::offset(A::mul(s.ad_value(41), s.ad_value(933)), 1.0), 1.0, s.ad_value(42), s.ad_value(934), 1.0), A::exp(A::mul_scaled_lhs(s.ad_value(44), -1.0, A::ln(A::add_scaled_product(A::offset(A::mul(s.ad_value(937), s.ad_value(268)), 1.0), 1.0, s.ad_value(938), s.ad_value(269), 1.0)))));

        s.b[1132] = (s.v[56] == 0.0);
        s.v[1132] = if s.b[1132] { 1.0 } else { 0.0 };

        if s.b[1132] {
            s.store_scalar(4, 1.0);
        }

        s.b[1133] = (s.v[56] < 0.0);
        s.v[1133] = if s.b[1133] { 1.0 } else { 0.0 };

        if ((!s.b[1132]) && s.b[1133]) {
            s.store_mul_exp_ad_rhs(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(917), 1e-12))));
            s.store_sub_from_scalar(4, 1.0, 2);
        }

        if ((!s.b[1132]) && (!s.b[1133])) {
            s.store_mul_exp_ad_rhs(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(917), 1e-12))));
            s.store_div_from_scalar_offset_input(4, 1.0, 2, 1.0);
        }

        s.store_mul_ad_affine_product_rhs(943, 272, s.ad_value(898), A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(879))), A::sqrt(A::offset(A::mul_sub_from_scalar_lhs(1.0, A::mul(s.ad_value(58), s.ad_value(879)), A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(879)))), 0.01))), 0.5, 0.0);

        s.store_mul_add_scaled_product_rhs(944, 943, s.ad_value(54), 1.0, s.ad_value(917), s.ad_value(4), 1.0);

        s.store_add_scaled_inputs_product_first_ad(945, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(935)), 1e-6)))), 1.0), 1.0, 942, 1.0, 38, 944, 1.0);

        s.store_add_scaled_inputs_product_first_ad(946, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(936)), 1e-6)))), 1.0), 1.0, 942, 1.0, 39, 944, 1.0);

        s.store_div_scaled_product_mixed_iaa(947, 941, A::add(s.ad_value(939), s.ad_value(940)), 1.0, A::add(A::div(s.ad_value(939), s.ad_value(945)), A::div(s.ad_value(940), s.ad_value(946))), 1.0);

        s.b[1134] = (((s.v[914]) as f64).abs() > 0.007);
        s.v[1134] = if s.b[1134] { 1.0 } else { 0.0 };

        s.b[1135] = (s.v[914] > 0.0);
        s.v[1135] = if s.b[1135] { 1.0 } else { 0.0 };

        if (s.b[1134] && s.b[1135]) {
            s.store_exp_neg_input(0, 914);
            s.store_div_ad_rhs(948, 914, A::sub_from_scalar(1.0, s.ad_value(0)));
            s.store_mul(949, 0, 948);
            s.store_add_offset_ad_lhs(950, A::ln(A::div(s.ad_value(884), A::mul(s.ad_value(917), s.ad_value(948)))), (-0.6931471805599), 887);
        }

        if (s.b[1134] && (!s.b[1135])) {
            s.store_exp(0, 914);
            s.store_div_scaled_value_offset_denominator(949, s.ad_value(914), 1.0, s.ad_value(0), (-1.0), 1.0);
            s.store_mul(948, 0, 949);
            s.store_add_offset_ad_lhs(950, A::ln(A::div(s.ad_value(884), A::mul(s.ad_value(917), s.ad_value(949)))), (-0.6931471805599), 888);
        }

        if s.b[1134] {
            s.store_div_scaled_inputs_mixed_ia(951, 914, -1.0, A::mul(s.ad_value(913), A::add_scaled_sub_value_product(1.0, s.ad_value(948), 1.0, s.ad_value(914), s.ad_value(886), (-1.0))), 1.0);
            s.store_div_ad_rhs(952, 914, A::mul(s.ad_value(913), A::add_scaled_sub_value_product(1.0, s.ad_value(949), 1.0, s.ad_value(914), s.ad_value(885), 1.0)));
            s.store_div_add_scaled_inputs_rhs_ad(953, 914, A::div_scaled_offset_numerator(A::mul(s.ad_value(949), s.ad_value(886)), 1.0, 0.5, s.ad_value(952), 1.0), 1.0, A::div_scaled_offset_numerator(A::mul(s.ad_value(948), s.ad_value(885)), 1.0, 0.5, s.ad_value(951), 1.0), -1.0);
        }

        if (!s.b[1134]) {
            s.store_scale(0, 915, (0.5 * 0.1666666666667));
            s.store_scale(2, 914, 0.5);
            s.store_add_offset_lhs(948, 2, 1.0, 0);
            s.store_add_ad_lhs(949, A::sub_from_scalar(1.0, s.ad_value(2)), 0);
            s.store_scale(3, 2, 0.1666666666667);
            s.store_div_from_scalar_mul_ad(951, 1.0, s.ad_value(913), A::add(A::offset(s.ad_value(886), 0.5), s.ad_value(3)));
            s.store_div_from_scalar_mul_ad(952, 1.0, s.ad_value(913), A::sub(A::offset(s.ad_value(885), 0.5), s.ad_value(3)));
            s.store_add_scaled_inputs3_offset_mixed_aii(950, A::ln(A::div(s.ad_value(884), A::mul_sub_from_scalar_rhs(s.ad_value(917), 1.0, A::scale(s.ad_value(0), 0.5)))), 1.0, 887, 0.5, 888, 0.5, (-0.6931471805599));
            s.store_div_from_scalar_ad(953, (-12.0), A::add_scaled_inputs4_offset(s.ad_value(913), ((-1.0) * 3.0), A::div_scaled_inputs(s.ad_value(913), 12.0, A::mul(s.ad_value(911), s.ad_value(912)), 1.0), 1.0, A::mul3(s.ad_value(913), A::sub(s.ad_value(885), s.ad_value(886)), s.ad_value(914)), 1.0, A::mul_sub_from_scalar_lhs_scaled_output(0.2, A::scale(s.ad_value(913), 0.25), s.ad_value(915), 0.3333333333333), 1.0, 4.0));
        }

        s.store_div_from_scalar(954, 1.0, 953);

        s.b[1136] = (s.v[917] > 1e-6);
        s.v[1136] = if s.b[1136] { 1.0 } else { 0.0 };

        if s.b[1136] {
            s.store_div_scaled_value_offset_denominator(955, s.ad_value(931), 100.0, s.ad_value(931), 100.0, 1.0);
        }

        s.b[1137] = (s.v[61] < 0.0);
        s.v[1137] = if s.b[1137] { 1.0 } else { 0.0 };

        if (s.b[1136] && s.b[1137]) {
            s.store_div_from_scalar_sub_from_scalar_ad(956, 1.0, 1.0, A::mul(s.ad_value(61), s.ad_value(955)));
        }

        if (s.b[1136] && (!s.b[1137])) {
            s.store_offset_mul(956, 61, 955, 1.0);
        }

        if s.b[1136] {
            s.store_div_scaled_value_offset_denominator(957, s.ad_value(932), 100.0, s.ad_value(932), 100.0, 1.0);
        }

        s.b[1138] = (s.v[62] < 0.0);
        s.v[1138] = if s.b[1138] { 1.0 } else { 0.0 };

        if (s.b[1136] && s.b[1138]) {
            s.store_div_from_scalar_sub_from_scalar_ad(958, 1.0, 1.0, A::mul(s.ad_value(62), s.ad_value(957)));
        }

        if (s.b[1136] && (!s.b[1138])) {
            s.store_offset_mul(958, 62, 957, 1.0);
        }

        if s.b[1136] {
            s.store_sub_ad(959, A::div_scaled_product_by_product(s.ad_value(929), s.ad_value(928), 1.0, s.ad_value(926), s.ad_value(927), 1.0), A::div_scaled_inputs2(A::div(s.ad_value(922), s.ad_value(926)), 1.0, A::div(s.ad_value(923), s.ad_value(927)), 1.0, s.ad_value(917), 1.0));
            s.store_div_scaled_product_offset_denominator(960, s.ad_value(959), s.ad_value(917), 1.0, s.ad_value(959), 1.0, 1.0);
            s.store_sub(2, 953, 960);
            s.store_div_scaled_add_product(961, s.ad_value(917), 1.0, s.ad_value(953), s.ad_value(950), 1.0, s.ad_value(2), 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(961, 961, 961, 1e-6, 0.5);
            s.store_scaled_mul_ad(962, A::div(s.ad_value(875), s.ad_value(947)), A::add(s.ad_value(956), s.ad_value(958)), 0.5);
            s.store_sub_from_scalar_div_indices(963, 1.0, 917, 960);
            s.store_offset(964, 950, 1.0);
            s.store_mul_sub_ad_lhs(965, A::offset(A::mul(A::sub_scaled_inputs(s.ad_value(960), 2.0, s.ad_value(917), 1.0), s.ad_value(954)), (-2.0)), s.ad_value(950), 961);
        }

        s.b[1139] = (s.v[962] > 1e-14);
        s.v[1139] = if s.b[1139] { 1.0 } else { 0.0 };

        if (s.b[1136] && s.b[1139]) {
            s.store_div_from_scalar_square_ad(966, 2.0, s.ad_value(962));
            s.store_mul(967, 966, 963);
            s.store_add(968, 966, 965);
            s.store_mul(969, 966, 964);
            s.store_sqrt_offset_ad(970, A::add(A::square(s.ad_value(967)), A::mul3_scaled_output(s.ad_value(966), s.ad_value(966), s.ad_value(966), 0.148148148148)), 1e-20);
            s.store_sqrt_offset_ad(971, A::add(A::square(s.ad_value(969)), A::mul3_scaled_output(s.ad_value(968), s.ad_value(968), s.ad_value(968), 0.148148148148)), 1e-20);
            s.store_sub_ad(972, A::exp_scaled_input(A::ln_scaled_input(A::add(s.ad_value(970), s.ad_value(967)), 0.5), 0.3333333333333), A::exp_scaled_input(A::ln_scaled_input(A::sub(s.ad_value(970), s.ad_value(967)), 0.5), 0.3333333333333));
            s.store_sub_ad(973, A::exp_scaled_input(A::ln_scaled_input(A::add(s.ad_value(971), s.ad_value(969)), 0.5), 0.3333333333333), A::exp_scaled_input(A::ln_scaled_input(A::sub(s.ad_value(971), s.ad_value(969)), 0.5), 0.3333333333333));
        }

        if (s.b[1136] && (!s.b[1139])) {
            s.copy_ad(972, 963);
            s.copy_ad(973, 964);
        }

        if s.b[1136] {
            s.store_square(4, 2);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(974, 972, (0.94 * 0.5), 973, (0.94 * 0.5), A::add_scaled_product(s.ad_value(4), 10.0, A::sub(s.ad_value(972), s.ad_value(973)), A::sub(s.ad_value(972), s.ad_value(973)), 1.0), (0.94 * 0.5));
            s.store_add_scaled_product_indices(975, 917, 1.0, 960, 974, 1.0);
            s.store_mul_sub_rhs(976, 953, 974, 950);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(977, 975, 0.5, 976, 0.5, A::add_scaled_product(s.ad_value(4), 36.0, A::sub(s.ad_value(975), s.ad_value(976)), A::sub(s.ad_value(975), s.ad_value(976)), 1.0), 0.5);
        }

        if (!s.b[1136]) {
            s.copy_ad(960, 953);
            s.store_scaled_offset(974, 950, 1.0, 0.94);
            s.store_add_scaled_product_right_ad(977, 917, 0.5, 953, A::sub_scaled_inputs(s.ad_value(974), 1.0, s.ad_value(950), 0.5), 1.0);
        }

        s.b[1140] = ((s.v[977] - 0.5) < 80.0);
        s.v[1140] = if s.b[1140] { 1.0 } else { 0.0 };

        if s.b[1140] {
            s.store_ln_one_plus_exp_ad(2, A::offset(s.ad_value(977), (-0.5)));
        }

        if (!s.b[1140]) {
            s.store_offset(2, 977, (-0.5));
        }

        s.store_offset(3, 2, 0.5);

        s.store_add_ad_rhs(4, 974, A::ln(A::div(s.ad_value(917), s.ad_value(3))));

        s.b[1141] = ((s.v[4] - 6.0) < 80.0);
        s.v[1141] = if s.b[1141] { 1.0 } else { 0.0 };

        if s.b[1141] {
            s.store_ln_one_plus_exp_ad(2, A::offset(s.ad_value(4), (-6.0)));
        }

        if (!s.b[1141]) {
            s.store_offset(2, 4, (-6.0));
        }

        s.store_offset(4, 2, 6.0);

        s.b[1142] = ((s.v[225] - s.v[4]) < 80.0);
        s.v[1142] = if s.b[1142] { 1.0 } else { 0.0 };

        if s.b[1142] {
            s.store_ln_one_plus_exp_ad(2, A::sub(s.ad_value(225), s.ad_value(4)));
        }

        if (!s.b[1142]) {
            s.store_sub(2, 225, 4);
        }

        s.store_sub(978, 225, 2);

        s.store_div(2, 339, 978);

        s.store_square(3, 2);

        s.store_square(4, 3);

        s.store_square(5, 4);

        s.store_exp_scaled_input_ad(0, A::ln(A::offset(A::mul(s.ad_value(876), s.ad_value(4)), 1.0)), 2.666666666667);

        s.store_mul_ad_rhs(979, 339, A::exp_scaled_input(A::ln(A::add(s.ad_value(0), A::square(s.ad_value(5)))), (-0.0625)));

        s.store_div_from_scalar_offset_input(793, 1.0, 911, 1.0);

        s.store_div_from_scalar_offset_input(794, 1.0, 912, 1.0);

        s.store_offset_add_ad(796, A::ln(A::div_scaled_product(A::add_scaled_product(s.ad_value(911), 1.0, s.ad_value(912), s.ad_value(794), 1.0), s.ad_value(883), 1.0, s.ad_value(884), 1.0)), s.ad_value(979), 3.0);

        s.store_offset_add_ad(797, A::ln(A::div_scaled_product(A::add_scaled_product(s.ad_value(912), 1.0, s.ad_value(911), s.ad_value(793), 1.0), s.ad_value(883), 1.0, s.ad_value(884), 1.0)), s.ad_value(979), 3.0);

        s.b[1143] = (((s.v[796] - s.v[887]) * 0.3333333333333) < 80.0);
        s.v[1143] = if s.b[1143] { 1.0 } else { 0.0 };

        if s.b[1143] {
            s.store_ln_one_plus_exp_ad(795, A::sub_scaled_inputs(s.ad_value(796), 0.3333333333333, s.ad_value(887), 0.3333333333333));
        }

        if (!s.b[1143]) {
            s.store_scaled_sub(795, 796, 887, 0.3333333333333);
        }

        s.store_sub_scaled_inputs(800, 796, 1.0, 795, 3.0);

        s.b[1144] = (((s.v[797] - s.v[888]) * 0.3333333333333) < 80.0);
        s.v[1144] = if s.b[1144] { 1.0 } else { 0.0 };

        if s.b[1144] {
            s.store_ln_one_plus_exp_ad(795, A::sub_scaled_inputs(s.ad_value(797), 0.3333333333333, s.ad_value(888), 0.3333333333333));
        }

        if (!s.b[1144]) {
            s.store_scaled_sub(795, 797, 888, 0.3333333333333);
        }

        s.store_sub_scaled_inputs(801, 797, 1.0, 795, 3.0);

        s.store_mul_add_scaled_product_rhs(798, 793, s.ad_value(801), 1.0, s.ad_value(911), s.ad_value(909), 1.0);

        s.store_mul_add_scaled_product_rhs(799, 794, s.ad_value(800), 1.0, s.ad_value(912), s.ad_value(910), 1.0);

        s.b[1145] = (((s.v[796] - s.v[798]) * 0.3333333333333) < 80.0);
        s.v[1145] = if s.b[1145] { 1.0 } else { 0.0 };

        if s.b[1145] {
            s.store_ln_one_plus_exp_ad(795, A::sub_scaled_inputs(s.ad_value(796), 0.3333333333333, s.ad_value(798), 0.3333333333333));
        }

        if (!s.b[1145]) {
            s.store_scaled_sub(795, 796, 798, 0.3333333333333);
        }

        s.store_sub_scaled_inputs(800, 796, 1.0, 795, 3.0);

        s.b[1146] = (((s.v[797] - s.v[799]) * 0.3333333333333) < 80.0);
        s.v[1146] = if s.b[1146] { 1.0 } else { 0.0 };

        if s.b[1146] {
            s.store_ln_one_plus_exp_ad(795, A::sub_scaled_inputs(s.ad_value(797), 0.3333333333333, s.ad_value(799), 0.3333333333333));
        }

        if (!s.b[1146]) {
            s.store_scaled_sub(795, 797, 799, 0.3333333333333);
        }

        s.store_sub_scaled_inputs(801, 797, 1.0, 795, 3.0);

        s.store_sub(980, 909, 800);

        s.store_sub(981, 910, 801);

        s.v[807] = 0.0;

        s.v[810] = 0.0;

        s.store_mul(802, 911, 980);

        s.b[1147] = (((s.v[909] - s.v[980]) - s.v[979]) < 80.0);
        s.v[1147] = if s.b[1147] { 1.0 } else { 0.0 };

        if s.b[1147] {
            s.store_exp_ad(793, A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0));
        }

        if (!s.b[1147]) {
            let assign17210_ad_e17011: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(793, assign17210_ad_e17011, 1.0, 5.54062e34);
        }

        s.store_mul(803, 884, 793);

        s.store_sub_ad_lhs(804, A::square(s.ad_value(802)), 803);

        s.store_add_scaled_product_indices(805, 803, 1.0, 911, 802, 2.0);

        s.store_add_scaled_product_indices(806, 803, (-1.0), 911, 911, 2.0);

        s.b[1148] = (s.v[804] < (-0.005));
        s.v[1148] = if s.b[1148] { 1.0 } else { 0.0 };

        if s.b[1148] {
            s.store_sqrt_abs_ad(807, s.ad_value(804));
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
            s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);
            s.store_mul_add_ad_lhs(809, s.ad_value(804), A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 793);
            s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);
            s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);
            s.store_mul_div_lhs(814, 805, 804, 794);
            s.store_div_ad_lhs(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);
        }

        s.b[1149] = (s.v[804] > 0.005);
        s.v[1149] = if s.b[1149] { 1.0 } else { 0.0 };

        if ((!s.b[1148]) && s.b[1149]) {
            s.store_sqrt_abs_ad(807, s.ad_value(804));
        }

    }

    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
    ) {
        if ((!s.b[1148]) && s.b[1149]) {
            s.store_exp_neg_input(810, 807);
            s.store_div_scaled_product_offset_rhs(808, s.ad_value(807), s.ad_value(810), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);
            s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);
            s.store_mul_add_ad_lhs(809, s.ad_value(804), A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 793);
            s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);
            s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);
            s.store_mul_div_lhs(814, 805, 804, 794);
            s.store_div_ad_lhs(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);
        }

        if ((!s.b[1148]) && (!s.b[1149])) {
            s.store_offset_scaled_ad(795, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(808, 804, 795, 2.0);
            s.store_offset_scaled_ad(793, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_mul(809, 805, 793);
            s.store_offset_scaled_ad(794, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(811, 806, 793, 1.0, A::square(s.ad_value(805)), 794, (-1.0));
            s.store_scaled_mul(814, 805, 795, (-0.5));
            s.store_add_scaled_product_value_ad(815, A::mul3_scaled_output(s.ad_value(805), s.ad_value(805), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 2.0, A::scale(s.ad_value(804), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 806, 795, (-0.5));
        }

        s.b[1150] = (s.v[804] > 0.005);
        s.v[1150] = if s.b[1150] { 1.0 } else { 0.0 };

        if s.b[1150] {
            s.store_div_scaled_inputs_mixed_ia(794, 804, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0);
            s.store_mul(812, 794, 810);
            s.store_sub_ad_lhs(813, A::ln(s.ad_value(794)), 807);
        }

        s.b[1151] = (s.v[804] < (-0.005));
        s.v[1151] = if s.b[1151] { 1.0 } else { 0.0 };

        if ((!s.b[1150]) && s.b[1151]) {
            s.store_sin_scaled_input(794, 807, 0.5);
            s.store_div_scaled_inputs_mixed_ia(812, 804, -1.0, A::square(s.ad_value(794)), 1.0);
            s.store_ln(813, 812);
        }

        if ((!s.b[1150]) && (!s.b[1151])) {
            s.store_sub_from_scalar_ad(812, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(813, 812);
        }

        s.b[1152] = (((1.01 * s.v[802]) + s.v[808]) > 0.0);
        s.v[1152] = if s.b[1152] { 1.0 } else { 0.0 };

        if s.b[1152] {
            s.store_add(816, 802, 808);
            s.store_add(817, 911, 809);
            s.copy_ad(818, 811);
        }

        if (!s.b[1152]) {
            s.store_div_from_scalar_sub_ad(794, 1.0, s.ad_value(802), s.ad_value(808));
            s.store_sub(795, 809, 911);
            s.store_mul_sub_lhs(816, 803, 812, 794);
            s.store_mul_ad_lhs(817, A::add_scaled_value_products(s.ad_value(803), (-1.0), s.ad_value(795), s.ad_value(816), 1.0, s.ad_value(814), s.ad_value(812), (-1.0)), 794);
            s.store_mul_ad_lhs(818, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(811), s.ad_value(816), 1.0, s.ad_value(795), s.ad_value(817), 2.0), 1.0, s.ad_value(803), 1.0, A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812), (-1.0)), 794);
        }

        s.b[1153] = (s.v[816] > 0.0);
        s.v[1153] = if s.b[1153] { 1.0 } else { 0.0 };

        if s.b[1153] {
            s.store_ln(819, 816);
            s.store_div_from_scalar(793, 1.0, 816);
            s.store_mul(820, 817, 793);
            s.store_add_scaled_square_product_indices(821, 820, (-1.0), 818, 793, 1.0);
        }

        if (!s.b[1153]) {
            s.store_add_offset_lhs_ad_rhs(819, 802, 0.6931471805599, A::ln_scaled_input(s.ad_value(802), -1.0));
            s.store_div_from_scalar(793, 1.0, 980);
            s.store_add(820, 911, 793);
            s.store_mul_neg_lhs(821, 793, 793);
        }

        s.store_sub_ad_lhs(822, A::add_scaled_inputs4(s.ad_value(910), 1.0, s.ad_value(909), (-1.0), s.ad_value(980), 1.0, s.ad_value(819), 2.0), 813);

        s.store_sub_ad_lhs(823, A::scale_offset(s.ad_value(820), 2.0, 1.0), 814);

        s.store_sub_scaled_inputs(824, 821, 2.0, 815, 1.0);

        s.store_add_scaled_product_indices(825, 802, 1.0, 912, 822, 1.0);

        s.store_add_scaled_product_indices(826, 911, 1.0, 912, 823, 1.0);

        s.store_mul(827, 912, 824);

        s.store_add_scaled_product_indices(828, 803, (-1.0), 825, 816, 1.0);

        s.store_add_ad_lhs(829, A::add_scaled_products(s.ad_value(826), s.ad_value(816), 1.0, s.ad_value(825), s.ad_value(817), 1.0), 803);

        s.store_sub_ad_lhs(830, A::add_scaled_products3(s.ad_value(827), s.ad_value(816), 1.0, s.ad_value(826), s.ad_value(817), 2.0, s.ad_value(825), s.ad_value(818), 1.0), 803);

        s.store_add_scaled_square_product_indices(839, 829, 1.0, 828, 830, (-0.5));

        s.store_div_scaled_product3_mixed_iiia(831, 828, 829, 839, -1.0, A::offset(A::square(s.ad_value(839)), 1e-200), 1.0);

        s.store_add(980, 980, 831);

        s.store_mul(802, 911, 980);

        s.store_mul(832, 912, 981);

        s.store_add(825, 802, 832);

        s.store_offset_scaled(833, 825, 0.065345483024, 1.0);

        s.store_add_scaled_product_value_ad(834, A::scale_offset(s.ad_value(825), 8.5797362674, 39.478417604), 1.0, 802, 832, 1.0);

        s.store_add_scaled_product_indices(835, 825, (2.0 * 39.478417604), 802, 832, 39.478417604);

        s.store_sqrt_ad(836, A::add_scaled_square_product(s.ad_value(834), 1.0, s.ad_value(833), s.ad_value(835), (-4.0)));

        s.store_div_scaled_inputs2_indices(804, 836, 1.0, 834, (-1.0), 833, 2.0);

        s.store_sub_ad_lhs(837, A::square(s.ad_value(802)), 804);

        s.b[1154] = (s.v[837] > 0.0);
        s.v[1154] = if s.b[1154] { 1.0 } else { 0.0 };

        if s.b[1154] {
            s.store_mul_ad_rhs(828, 837, A::add_scaled_inputs4(A::ln(A::div(s.ad_value(837), s.ad_value(884))), 1.0, s.ad_value(979), 1.0, s.ad_value(909), -1.0, s.ad_value(980), 1.0));
            s.store_add_scaled_product_indices(829, 837, 1.0, 911, 802, 2.0);
        }

        let (assign18050_e17843,) = {
    if s.b[1154] {
        let assign18050_e17839: f64 = (s.v[909] - s.v[980]);
        let assign18050_e17841: f64 = (assign18050_e17839 - s.v[796]);
        (assign18050_e17841,)
    } else {
        (s.v[838],)
    }
};
        s.v[838] = assign18050_e17843;

        s.b[1155] = ((((s.v[828] < 0.0) && (s.v[829] > 0.0)) && (((s.v[838] + 2.3025850929941) + ((s.v[911]) as f64).ln()) > 0.0)) || (s.v[838] > 1.0));
        s.v[1155] = if s.b[1155] { 1.0 } else { 0.0 };

        if (s.b[1154] && s.b[1155]) {
            s.store_sub_div_rhs_indices(980, 980, 828, 829);
        }

        s.store_mul(802, 911, 980);

        s.store_mul(832, 912, 981);

        s.store_add(825, 802, 832);

        s.store_offset_scaled(833, 825, 0.065345483024, 1.0);

        s.store_add_scaled_product_value_ad(834, A::scale_offset(s.ad_value(825), 8.5797362674, 39.478417604), 1.0, 802, 832, 1.0);

        s.store_add_scaled_product_indices(835, 825, (2.0 * 39.478417604), 802, 832, 39.478417604);

        s.store_sqrt_ad(836, A::add_scaled_square_product(s.ad_value(834), 1.0, s.ad_value(833), s.ad_value(835), (-4.0)));

        s.store_div_scaled_inputs2_indices(804, 836, 1.0, 834, (-1.0), 833, 2.0);

        s.b[1156] = (s.v[804] < (-0.005));
        s.v[1156] = if s.b[1156] { 1.0 } else { 0.0 };

        if s.b[1156] {
            s.store_sqrt_abs_ad(807, s.ad_value(804));
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
            s.store_div_scaled_inputs2_mixed_iai(809, 804, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 0.25, 804, 1.0);
        }

        s.b[1157] = (s.v[804] > 0.005);
        s.v[1157] = if s.b[1157] { 1.0 } else { 0.0 };

        if ((!s.b[1156]) && s.b[1157]) {
            s.store_sqrt_abs_ad(807, s.ad_value(804));
            s.store_exp_neg_input(810, 807);
            s.store_div_scaled_product_offset_rhs(808, s.ad_value(807), s.ad_value(810), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);
            s.store_div_scaled_inputs2_mixed_iai(809, 804, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 0.25, 804, 1.0);
        }

        if ((!s.b[1156]) && (!s.b[1157])) {
            s.store_offset_ad(808, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);
            s.store_offset_scaled_ad(809, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
        }

        s.store_sub_ad_rhs(804, 804, A::div_scaled_inputs2(A::add_scaled_products(s.ad_value(825), s.ad_value(808), 1.0, s.ad_value(802), s.ad_value(832), 1.0), 1.0, s.ad_value(804), 1.0, A::offset(A::mul(s.ad_value(825), s.ad_value(809)), 1.0), 1.0));

        s.store_sub_ad_lhs(837, A::square(s.ad_value(802)), 804);

        s.b[1158] = (s.v[837] > 0.0);
        s.v[1158] = if s.b[1158] { 1.0 } else { 0.0 };

        if s.b[1158] {
            s.store_mul_ad_rhs(828, 837, A::add_scaled_inputs4(A::ln(A::div(s.ad_value(837), s.ad_value(884))), 1.0, s.ad_value(979), 1.0, s.ad_value(909), -1.0, s.ad_value(980), 1.0));
            s.store_add_scaled_product_indices(829, 837, 1.0, 911, 802, 2.0);
        }

        let (assign18320_e18116,) = {
    if s.b[1158] {
        let assign18320_e18112: f64 = (s.v[909] - s.v[980]);
        let assign18320_e18114: f64 = (assign18320_e18112 - s.v[796]);
        (assign18320_e18114,)
    } else {
        (s.v[838],)
    }
};
        s.v[838] = assign18320_e18116;

        s.b[1159] = ((((s.v[828] < 0.0) && (s.v[829] > 0.0)) && (((s.v[838] + 2.3025850929941) + ((s.v[911]) as f64).ln()) > 0.0)) || (s.v[838] > 1.0));
        s.v[1159] = if s.b[1159] { 1.0 } else { 0.0 };

        if (s.b[1158] && s.b[1159]) {
            s.store_sub_div_rhs_indices(980, 980, 828, 829);
        }

        s.store_mul(802, 911, 980);

        s.b[1160] = (((s.v[909] - s.v[980]) - s.v[979]) < 80.0);
        s.v[1160] = if s.b[1160] { 1.0 } else { 0.0 };

        if s.b[1160] {
            s.store_exp_ad(793, A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0));
        }

        if (!s.b[1160]) {
            let assign18380_ad_e18200: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(793, assign18380_ad_e18200, 1.0, 5.54062e34);
        }

        s.store_mul(803, 884, 793);

        s.store_sub_ad_lhs(804, A::square(s.ad_value(802)), 803);

        s.store_add_scaled_product_indices(805, 803, 1.0, 911, 802, 2.0);

        s.store_add_scaled_product_indices(806, 803, (-1.0), 911, 911, 2.0);

        s.b[1161] = (s.v[804] < (-0.005));
        s.v[1161] = if s.b[1161] { 1.0 } else { 0.0 };

        if s.b[1161] {
            s.store_sqrt_abs_ad(807, s.ad_value(804));
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
            s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);
            s.store_mul_add_ad_lhs(809, s.ad_value(804), A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 793);
            s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);
            s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);
            s.store_mul_div_lhs(814, 805, 804, 794);
            s.store_div_ad_lhs(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);
        }

        s.b[1162] = (s.v[804] > 0.005);
        s.v[1162] = if s.b[1162] { 1.0 } else { 0.0 };

        if ((!s.b[1161]) && s.b[1162]) {
            s.store_sqrt_abs_ad(807, s.ad_value(804));
            s.store_exp_neg_input(810, 807);
            s.store_div_scaled_product_offset_rhs(808, s.ad_value(807), s.ad_value(810), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);
            s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);
            s.store_mul_add_ad_lhs(809, s.ad_value(804), A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 793);
            s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);
            s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);
            s.store_mul_div_lhs(814, 805, 804, 794);
            s.store_div_ad_lhs(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);
        }

        if ((!s.b[1161]) && (!s.b[1162])) {
            s.store_offset_scaled_ad(795, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(808, 804, 795, 2.0);
            s.store_offset_scaled_ad(793, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_mul(809, 805, 793);
            s.store_offset_scaled_ad(794, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(811, 806, 793, 1.0, A::square(s.ad_value(805)), 794, (-1.0));
            s.store_scaled_mul(814, 805, 795, (-0.5));
            s.store_add_scaled_product_value_ad(815, A::mul3_scaled_output(s.ad_value(805), s.ad_value(805), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 2.0, A::scale(s.ad_value(804), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 806, 795, (-0.5));
        }

        s.b[1163] = (s.v[804] > 0.005);
        s.v[1163] = if s.b[1163] { 1.0 } else { 0.0 };

        if s.b[1163] {
            s.store_div_scaled_inputs_mixed_ia(794, 804, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0);
            s.store_mul(812, 794, 810);
            s.store_sub_ad_lhs(813, A::ln(s.ad_value(794)), 807);
        }

        s.b[1164] = (s.v[804] < (-0.005));
        s.v[1164] = if s.b[1164] { 1.0 } else { 0.0 };

        if ((!s.b[1163]) && s.b[1164]) {
            s.store_sin_scaled_input(794, 807, 0.5);
            s.store_div_scaled_inputs_mixed_ia(812, 804, -1.0, A::square(s.ad_value(794)), 1.0);
            s.store_ln(813, 812);
        }

        if ((!s.b[1163]) && (!s.b[1164])) {
            s.store_sub_from_scalar_ad(812, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(813, 812);
        }

        s.b[1165] = (((1.01 * s.v[802]) + s.v[808]) > 0.0);
        s.v[1165] = if s.b[1165] { 1.0 } else { 0.0 };

        if s.b[1165] {
            s.store_add(816, 802, 808);
            s.store_add(817, 911, 809);
        }

    }

    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1165] {
            s.copy_ad(818, 811);
        }

        if (!s.b[1165]) {
            s.store_div_from_scalar_sub_ad(794, 1.0, s.ad_value(802), s.ad_value(808));
            s.store_sub(795, 809, 911);
            s.store_mul_sub_lhs(816, 803, 812, 794);
            s.store_mul_ad_lhs(817, A::add_scaled_value_products(s.ad_value(803), (-1.0), s.ad_value(795), s.ad_value(816), 1.0, s.ad_value(814), s.ad_value(812), (-1.0)), 794);
            s.store_mul_ad_lhs(818, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(811), s.ad_value(816), 1.0, s.ad_value(795), s.ad_value(817), 2.0), 1.0, s.ad_value(803), 1.0, A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812), (-1.0)), 794);
        }

        s.b[1166] = (s.v[816] > 0.0);
        s.v[1166] = if s.b[1166] { 1.0 } else { 0.0 };

        if s.b[1166] {
            s.store_ln(819, 816);
            s.store_div_from_scalar(793, 1.0, 816);
            s.store_mul(820, 817, 793);
            s.store_add_scaled_square_product_indices(821, 820, (-1.0), 818, 793, 1.0);
        }

        if (!s.b[1166]) {
            s.store_add_offset_lhs_ad_rhs(819, 802, 0.6931471805599, A::ln_scaled_input(s.ad_value(802), -1.0));
            s.store_div_from_scalar(793, 1.0, 980);
            s.store_add(820, 911, 793);
            s.store_mul_neg_lhs(821, 793, 793);
        }

        s.store_sub_ad_lhs(822, A::add_scaled_inputs4(s.ad_value(910), 1.0, s.ad_value(909), (-1.0), s.ad_value(980), 1.0, s.ad_value(819), 2.0), 813);

        s.store_sub_ad_lhs(823, A::scale_offset(s.ad_value(820), 2.0, 1.0), 814);

        s.store_sub_scaled_inputs(824, 821, 2.0, 815, 1.0);

        s.store_add_scaled_product_indices(825, 802, 1.0, 912, 822, 1.0);

        s.store_add_scaled_product_indices(826, 911, 1.0, 912, 823, 1.0);

        s.store_mul(827, 912, 824);

        s.store_add_scaled_product_indices(828, 803, (-1.0), 825, 816, 1.0);

        s.store_add_ad_lhs(829, A::add_scaled_products(s.ad_value(826), s.ad_value(816), 1.0, s.ad_value(825), s.ad_value(817), 1.0), 803);

        s.store_sub_ad_lhs(830, A::add_scaled_products3(s.ad_value(827), s.ad_value(816), 1.0, s.ad_value(826), s.ad_value(817), 2.0, s.ad_value(825), s.ad_value(818), 1.0), 803);

        s.store_add_scaled_square_product_indices(839, 829, 1.0, 828, 830, (-0.5));

        s.store_div_scaled_product3_mixed_iiia(831, 828, 829, 839, -1.0, A::offset(A::square(s.ad_value(839)), 1e-200), 1.0);

        s.store_add(980, 980, 831);

        s.store_mul(802, 911, 980);

        s.b[1167] = (((s.v[909] - s.v[980]) - s.v[979]) < 80.0);
        s.v[1167] = if s.b[1167] { 1.0 } else { 0.0 };

        if s.b[1167] {
            s.store_exp_ad(793, A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0));
        }

        if (!s.b[1167]) {
            let assign19130_ad_e18996: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(793, assign19130_ad_e18996, 1.0, 5.54062e34);
        }

        s.store_mul(803, 884, 793);

        s.store_sub_ad_lhs(804, A::square(s.ad_value(802)), 803);

        s.store_add_scaled_product_indices(805, 803, 1.0, 911, 802, 2.0);

        s.store_add_scaled_product_indices(806, 803, (-1.0), 911, 911, 2.0);

        s.b[1168] = (s.v[804] < (-0.005));
        s.v[1168] = if s.b[1168] { 1.0 } else { 0.0 };

        if s.b[1168] {
            s.store_sqrt_abs_ad(807, s.ad_value(804));
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
            s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);
            s.store_mul_add_ad_lhs(809, s.ad_value(804), A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 793);
            s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);
            s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);
            s.store_mul_div_lhs(814, 805, 804, 794);
            s.store_div_ad_lhs(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);
        }

        s.b[1169] = (s.v[804] > 0.005);
        s.v[1169] = if s.b[1169] { 1.0 } else { 0.0 };

        if ((!s.b[1168]) && s.b[1169]) {
            s.store_sqrt_abs_ad(807, s.ad_value(804));
            s.store_exp_neg_input(810, 807);
            s.store_div_scaled_product_offset_rhs(808, s.ad_value(807), s.ad_value(810), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);
            s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);
            s.store_mul_add_ad_lhs(809, s.ad_value(804), A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 793);
            s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);
            s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);
            s.store_mul_div_lhs(814, 805, 804, 794);
            s.store_div_ad_lhs(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);
        }

        if ((!s.b[1168]) && (!s.b[1169])) {
            s.store_offset_scaled_ad(795, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(808, 804, 795, 2.0);
            s.store_offset_scaled_ad(793, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_mul(809, 805, 793);
            s.store_offset_scaled_ad(794, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(811, 806, 793, 1.0, A::square(s.ad_value(805)), 794, (-1.0));
            s.store_scaled_mul(814, 805, 795, (-0.5));
            s.store_add_scaled_product_value_ad(815, A::mul3_scaled_output(s.ad_value(805), s.ad_value(805), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 2.0, A::scale(s.ad_value(804), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 806, 795, (-0.5));
        }

        s.b[1170] = (s.v[804] > 0.005);
        s.v[1170] = if s.b[1170] { 1.0 } else { 0.0 };

        if s.b[1170] {
            s.store_div_scaled_inputs_mixed_ia(794, 804, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0);
            s.store_mul(812, 794, 810);
            s.store_sub_ad_lhs(813, A::ln(s.ad_value(794)), 807);
        }

        s.b[1171] = (s.v[804] < (-0.005));
        s.v[1171] = if s.b[1171] { 1.0 } else { 0.0 };

        if ((!s.b[1170]) && s.b[1171]) {
            s.store_sin_scaled_input(794, 807, 0.5);
            s.store_div_scaled_inputs_mixed_ia(812, 804, -1.0, A::square(s.ad_value(794)), 1.0);
            s.store_ln(813, 812);
        }

        if ((!s.b[1170]) && (!s.b[1171])) {
            s.store_sub_from_scalar_ad(812, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(813, 812);
        }

        s.b[1172] = (((1.01 * s.v[802]) + s.v[808]) > 0.0);
        s.v[1172] = if s.b[1172] { 1.0 } else { 0.0 };

        if s.b[1172] {
            s.store_add(816, 802, 808);
            s.store_add(817, 911, 809);
            s.copy_ad(818, 811);
        }

        if (!s.b[1172]) {
            s.store_div_from_scalar_sub_ad(794, 1.0, s.ad_value(802), s.ad_value(808));
            s.store_sub(795, 809, 911);
            s.store_mul_sub_lhs(816, 803, 812, 794);
            s.store_mul_ad_lhs(817, A::add_scaled_value_products(s.ad_value(803), (-1.0), s.ad_value(795), s.ad_value(816), 1.0, s.ad_value(814), s.ad_value(812), (-1.0)), 794);
            s.store_mul_ad_lhs(818, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(811), s.ad_value(816), 1.0, s.ad_value(795), s.ad_value(817), 2.0), 1.0, s.ad_value(803), 1.0, A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812), (-1.0)), 794);
        }

        s.b[1173] = (s.v[816] > 0.0);
        s.v[1173] = if s.b[1173] { 1.0 } else { 0.0 };

        if s.b[1173] {
            s.store_ln(819, 816);
            s.store_div_from_scalar(793, 1.0, 816);
            s.store_mul(820, 817, 793);
            s.store_add_scaled_square_product_indices(821, 820, (-1.0), 818, 793, 1.0);
        }

        if (!s.b[1173]) {
            s.store_add_offset_lhs_ad_rhs(819, 802, 0.6931471805599, A::ln_scaled_input(s.ad_value(802), -1.0));
            s.store_div_from_scalar(793, 1.0, 980);
            s.store_add(820, 911, 793);
            s.store_mul_neg_lhs(821, 793, 793);
        }

        s.store_sub_ad_lhs(822, A::add_scaled_inputs4(s.ad_value(910), 1.0, s.ad_value(909), (-1.0), s.ad_value(980), 1.0, s.ad_value(819), 2.0), 813);

        s.store_sub_ad_lhs(823, A::scale_offset(s.ad_value(820), 2.0, 1.0), 814);

        s.store_sub_scaled_inputs(824, 821, 2.0, 815, 1.0);

        s.store_add_scaled_product_indices(825, 802, 1.0, 912, 822, 1.0);

        s.store_add_scaled_product_indices(826, 911, 1.0, 912, 823, 1.0);

        s.store_mul(827, 912, 824);

        s.store_add_scaled_product_indices(828, 803, (-1.0), 825, 816, 1.0);

        s.store_add_ad_lhs(829, A::add_scaled_products(s.ad_value(826), s.ad_value(816), 1.0, s.ad_value(825), s.ad_value(817), 1.0), 803);

        s.store_sub_ad_lhs(830, A::add_scaled_products3(s.ad_value(827), s.ad_value(816), 1.0, s.ad_value(826), s.ad_value(817), 2.0, s.ad_value(825), s.ad_value(818), 1.0), 803);

        s.store_add_scaled_square_product_indices(839, 829, 1.0, 828, 830, (-0.5));

        s.store_div_scaled_product3_mixed_iiia(831, 828, 829, 839, -1.0, A::offset(A::square(s.ad_value(839)), 1e-200), 1.0);

        s.store_add(980, 980, 831);

        s.b[1174] = (p.p10 == 1.0);
        s.v[1174] = if s.b[1174] { 1.0 } else { 0.0 };

        s.b[1175] = (((s.v[831]) as f64).abs() > 0.01);
        s.v[1175] = if s.b[1175] { 1.0 } else { 0.0 };

        if (s.b[1174] && s.b[1175]) {
            s.store_mul(802, 911, 980);
        }

        s.b[1176] = (((s.v[909] - s.v[980]) - s.v[979]) < 80.0);
        s.v[1176] = if s.b[1176] { 1.0 } else { 0.0 };

        if ((s.b[1174] && s.b[1175]) && s.b[1176]) {
            s.store_exp_ad(793, A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0));
        }

        if ((s.b[1174] && s.b[1175]) && (!s.b[1176])) {
            let assign19900_ad_e19812: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(793, assign19900_ad_e19812, 1.0, 5.54062e34);
        }

        if (s.b[1174] && s.b[1175]) {
            s.store_mul(803, 884, 793);
            s.store_sub_ad_lhs(804, A::square(s.ad_value(802)), 803);
            s.store_add_scaled_product_indices(805, 803, 1.0, 911, 802, 2.0);
            s.store_add_scaled_product_indices(806, 803, (-1.0), 911, 911, 2.0);
        }

        s.b[1177] = (s.v[804] < (-0.005));
        s.v[1177] = if s.b[1177] { 1.0 } else { 0.0 };

        if ((s.b[1174] && s.b[1175]) && s.b[1177]) {
            s.store_sqrt_abs_ad(807, s.ad_value(804));
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
            s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);
            s.store_mul_add_ad_lhs(809, s.ad_value(804), A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 793);
            s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);
            s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);
            s.store_mul_div_lhs(814, 805, 804, 794);
            s.store_div_ad_lhs(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);
        }

        s.b[1178] = (s.v[804] > 0.005);
        s.v[1178] = if s.b[1178] { 1.0 } else { 0.0 };

        if (((s.b[1174] && s.b[1175]) && (!s.b[1177])) && s.b[1178]) {
            s.store_sqrt_abs_ad(807, s.ad_value(804));
            s.store_exp_neg_input(810, 807);
            s.store_div_scaled_product_offset_rhs(808, s.ad_value(807), s.ad_value(810), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);
            s.store_div_scaled_inputs_indices(793, 805, 0.25, 804, 1.0);
            s.store_mul_add_ad_lhs(809, s.ad_value(804), A::mul_sub_from_scalar_rhs(s.ad_value(808), 2.0, s.ad_value(808)), 793);
            s.store_add_scaled_product_mixed_aai(811, A::div_scaled_product(s.ad_value(809), s.ad_value(806), 1.0, s.ad_value(805), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(805), 1.0, s.ad_value(809), s.ad_value(808), 1.0, (-2.0)), 793, 1.0);
            s.store_sub_from_scalar_scaled_input(794, 1.0, 808, 0.5);
            s.store_mul_div_lhs(814, 805, 804, 794);
            s.store_div_ad_lhs(815, A::add_scaled_products(s.ad_value(806), s.ad_value(794), 1.0, s.ad_value(805), A::add_scaled_inputs(s.ad_value(814), 1.0, s.ad_value(809), 0.5), (-1.0)), 804);
        }

        if (((s.b[1174] && s.b[1175]) && (!s.b[1177])) && (!s.b[1178])) {
            s.store_offset_scaled_ad(795, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(808, 804, 795, 2.0);
            s.store_offset_scaled_ad(793, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_mul(809, 805, 793);
            s.store_offset_scaled_ad(794, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(811, 806, 793, 1.0, A::square(s.ad_value(805)), 794, (-1.0));
            s.store_scaled_mul(814, 805, 795, (-0.5));
            s.store_add_scaled_product_value_ad(815, A::mul3_scaled_output(s.ad_value(805), s.ad_value(805), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 2.0, A::scale(s.ad_value(804), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 806, 795, (-0.5));
        }

        s.b[1179] = (s.v[804] > 0.005);
        s.v[1179] = if s.b[1179] { 1.0 } else { 0.0 };

        if ((s.b[1174] && s.b[1175]) && s.b[1179]) {
            s.store_div_scaled_inputs_mixed_ia(794, 804, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0);
            s.store_mul(812, 794, 810);
            s.store_sub_ad_lhs(813, A::ln(s.ad_value(794)), 807);
        }

        s.b[1180] = (s.v[804] < (-0.005));
        s.v[1180] = if s.b[1180] { 1.0 } else { 0.0 };

        if (((s.b[1174] && s.b[1175]) && (!s.b[1179])) && s.b[1180]) {
            s.store_sin_scaled_input(794, 807, 0.5);
        }

    }

    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1174] && s.b[1175]) && (!s.b[1179])) && s.b[1180]) {
            s.store_div_scaled_inputs_mixed_ia(812, 804, -1.0, A::square(s.ad_value(794)), 1.0);
            s.store_ln(813, 812);
        }

        if (((s.b[1174] && s.b[1175]) && (!s.b[1179])) && (!s.b[1180])) {
            s.store_sub_from_scalar_ad(812, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(804), 1.0, A::scale(s.ad_value(804), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(813, 812);
        }

        s.b[1181] = (((1.01 * s.v[802]) + s.v[808]) > 0.0);
        s.v[1181] = if s.b[1181] { 1.0 } else { 0.0 };

        if ((s.b[1174] && s.b[1175]) && s.b[1181]) {
            s.store_add(816, 802, 808);
            s.store_add(817, 911, 809);
            s.copy_ad(818, 811);
        }

        if ((s.b[1174] && s.b[1175]) && (!s.b[1181])) {
            s.store_div_from_scalar_sub_ad(794, 1.0, s.ad_value(802), s.ad_value(808));
            s.store_sub(795, 809, 911);
            s.store_mul_sub_lhs(816, 803, 812, 794);
            s.store_mul_ad_lhs(817, A::add_scaled_value_products(s.ad_value(803), (-1.0), s.ad_value(795), s.ad_value(816), 1.0, s.ad_value(814), s.ad_value(812), (-1.0)), 794);
            s.store_mul_ad_lhs(818, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(811), s.ad_value(816), 1.0, s.ad_value(795), s.ad_value(817), 2.0), 1.0, s.ad_value(803), 1.0, A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812), (-1.0)), 794);
        }

        s.b[1182] = (s.v[816] > 0.0);
        s.v[1182] = if s.b[1182] { 1.0 } else { 0.0 };

        if ((s.b[1174] && s.b[1175]) && s.b[1182]) {
            s.store_ln(819, 816);
            s.store_div_from_scalar(793, 1.0, 816);
            s.store_mul(820, 817, 793);
            s.store_add_scaled_square_product_indices(821, 820, (-1.0), 818, 793, 1.0);
        }

        if ((s.b[1174] && s.b[1175]) && (!s.b[1182])) {
            s.store_add_offset_lhs_ad_rhs(819, 802, 0.6931471805599, A::ln_scaled_input(s.ad_value(802), -1.0));
            s.store_div_from_scalar(793, 1.0, 980);
            s.store_add(820, 911, 793);
            s.store_mul_neg_lhs(821, 793, 793);
        }

        if (s.b[1174] && s.b[1175]) {
            s.store_sub_ad_lhs(822, A::add_scaled_inputs4(s.ad_value(910), 1.0, s.ad_value(909), (-1.0), s.ad_value(980), 1.0, s.ad_value(819), 2.0), 813);
            s.store_sub_ad_lhs(823, A::scale_offset(s.ad_value(820), 2.0, 1.0), 814);
            s.store_sub_scaled_inputs(824, 821, 2.0, 815, 1.0);
            s.store_add_scaled_product_indices(825, 802, 1.0, 912, 822, 1.0);
            s.store_add_scaled_product_indices(826, 911, 1.0, 912, 823, 1.0);
            s.store_mul(827, 912, 824);
            s.store_add_scaled_product_indices(828, 803, (-1.0), 825, 816, 1.0);
            s.store_add_ad_lhs(829, A::add_scaled_products(s.ad_value(826), s.ad_value(816), 1.0, s.ad_value(825), s.ad_value(817), 1.0), 803);
            s.store_sub_ad_lhs(830, A::add_scaled_products3(s.ad_value(827), s.ad_value(816), 1.0, s.ad_value(826), s.ad_value(817), 2.0, s.ad_value(825), s.ad_value(818), 1.0), 803);
            s.store_add_scaled_square_product_indices(839, 829, 1.0, 828, 830, (-0.5));
            s.store_div_scaled_product3_mixed_iiia(831, 828, 829, 839, -1.0, A::offset(A::square(s.ad_value(839)), 1e-200), 1.0);
            s.store_add(980, 980, 831);
        }

        s.store_mul(983, 911, 980);

        s.b[1183] = (((s.v[909] - s.v[980]) - s.v[979]) < 80.0);
        s.v[1183] = if s.b[1183] { 1.0 } else { 0.0 };

        if s.b[1183] {
            s.store_exp_ad(793, A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0));
        }

        if (!s.b[1183]) {
            let assign20650_ad_e20884: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(909), 1.0, s.ad_value(980), (-1.0), s.ad_value(979), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(793, assign20650_ad_e20884, 1.0, 5.54062e34);
        }

        s.store_mul(986, 884, 793);

        s.store_sub_ad_lhs(985, A::square(s.ad_value(983)), 986);

        s.b[1184] = (s.v[986] <= 0.0);
        s.v[1184] = if s.b[1184] { 1.0 } else { 0.0 };

        if s.b[1184] {
            s.store_scalar(982, 1e-80);
            s.store_sub(984, 982, 983);
            s.store_div(981, 984, 912);
        }

        s.b[1185] = (s.v[985] < (-0.005));
        s.v[1185] = if s.b[1185] { 1.0 } else { 0.0 };

        if ((!s.b[1184]) && s.b[1185]) {
            s.store_sqrt_abs_ad(807, s.ad_value(985));
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
        }

        s.b[1186] = (s.v[985] > 0.005);
        s.v[1186] = if s.b[1186] { 1.0 } else { 0.0 };

        if (((!s.b[1184]) && (!s.b[1185])) && s.b[1186]) {
            s.store_sqrt_abs_ad(807, s.ad_value(985));
            s.store_exp_neg_input(810, 807);
            s.store_div_scaled_product_offset_rhs(808, s.ad_value(807), s.ad_value(810), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(810)), 1.0);
        }

        if (((!s.b[1184]) && (!s.b[1185])) && (!s.b[1186])) {
            s.store_offset_ad(808, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(985), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(985), 1.0, A::scale(s.ad_value(985), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);
        }

        s.b[1187] = (((1.01 * s.v[983]) + s.v[808]) > 0.0);
        s.v[1187] = if s.b[1187] { 1.0 } else { 0.0 };

        if ((!s.b[1184]) && s.b[1187]) {
            s.store_add(793, 983, 808);
        }

        s.b[1188] = ((s.v[986] * s.v[983]) < (((0.9 * s.v[983]) * s.v[983]) * s.v[793]));
        s.v[1188] = if s.b[1188] { 1.0 } else { 0.0 };

        if (((!s.b[1184]) && s.b[1187]) && s.b[1188]) {
            s.store_offset_div(982, 986, 793, 1e-80);
            s.store_sub(984, 982, 983);
            s.store_div(981, 984, 912);
        }

        s.b[1189] = (s.v[985] > 0.005);
        s.v[1189] = if s.b[1189] { 1.0 } else { 0.0 };

        if ((((!s.b[1184]) && s.b[1187]) && (!s.b[1188])) && s.b[1189]) {
            s.store_sub_ad_lhs(794, A::ln(A::div_scaled_inputs(s.ad_value(985), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0)), 807);
        }

        s.b[1190] = (s.v[985] < (-0.005));
        s.v[1190] = if s.b[1190] { 1.0 } else { 0.0 };

        if (((((!s.b[1184]) && s.b[1187]) && (!s.b[1188])) && (!s.b[1189])) && s.b[1190]) {
            s.store_sin_scaled_input(795, 807, 0.5);
            s.store_ln_div_scaled_input_square_denominator(794, 985, -1.0, 795, 1.0);
        }

        if (((((!s.b[1184]) && s.b[1187]) && (!s.b[1188])) && (!s.b[1189])) && (!s.b[1190])) {
            s.store_ln_ad(794, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(985), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(985), 1.0, A::scale(s.ad_value(985), 0.0396825396825397), 0.05), 0.3333333333333)));
        }

        if (((!s.b[1184]) && s.b[1187]) && (!s.b[1188])) {
            s.store_sub_ad_lhs(981, A::add_scaled_inputs4(s.ad_value(910), 1.0, s.ad_value(909), (-1.0), s.ad_value(980), 1.0, A::ln(s.ad_value(793)), 2.0), 794);
            s.store_mul(984, 912, 981);
            s.store_add(982, 983, 984);
        }

        s.b[1191] = (s.v[985] > 0.005);
        s.v[1191] = if s.b[1191] { 1.0 } else { 0.0 };

        s.b[1192] = ((((s.v[980] + s.v[979]) - s.v[909]) - s.v[807]) < 80.0);
        s.v[1192] = if s.b[1192] { 1.0 } else { 0.0 };

        if ((((!s.b[1184]) && (!s.b[1187])) && s.b[1191]) && s.b[1192]) {
            s.store_exp_ad(795, A::add_scaled_inputs4(s.ad_value(980), 1.0, s.ad_value(979), 1.0, s.ad_value(909), -1.0, s.ad_value(807), -1.0));
        }

        if ((((!s.b[1184]) && (!s.b[1187])) && s.b[1191]) && (!s.b[1192])) {
            let assign20980_ad_e21303: A = A::mul_offset_lhs(A::add_scaled_inputs4(s.ad_value(980), 1.0, s.ad_value(979), 1.0, s.ad_value(909), -1.0, s.ad_value(807), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs4(s.ad_value(980), 1.0, s.ad_value(979), 1.0, s.ad_value(909), -1.0, s.ad_value(807), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs4(s.ad_value(980), 1.0, s.ad_value(979), 1.0, s.ad_value(909), -1.0, s.ad_value(807), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(795, assign20980_ad_e21303, 1.0, 5.54062e34);
        }

        if (((!s.b[1184]) && (!s.b[1187])) && s.b[1191]) {
            s.store_div(794, 795, 884);
            s.store_div_scaled_product_denominator_ad(793, 985, 794, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(810), 2.0, s.ad_value(810))), 1.0);
        }

        s.b[1193] = (s.v[985] < (-0.005));
        s.v[1193] = if s.b[1193] { 1.0 } else { 0.0 };

        if ((((!s.b[1184]) && (!s.b[1187])) && (!s.b[1191])) && s.b[1193]) {
            s.store_sin_scaled_input(794, 807, 0.5);
            s.store_div_scaled_value_by_product(793, s.ad_value(985), -1.0, A::square(s.ad_value(794)), s.ad_value(986), 1.0);
        }

        if ((((!s.b[1184]) && (!s.b[1187])) && (!s.b[1191])) && (!s.b[1193])) {
            s.store_div_ad_lhs(793, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(985), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(985), 1.0, A::scale(s.ad_value(985), 0.0396825396825397), 0.05), 0.3333333333333)), 986);
        }

        if ((!s.b[1184]) && (!s.b[1187])) {
            s.store_offset_div_scaled_inputs2(982, s.ad_value(983), 1.0, s.ad_value(808), (-1.0), A::sub_from_scalar(1.0, s.ad_value(793)), 1.0, 1e-80);
            s.store_sub(984, 982, 983);
            s.store_div(981, 984, 912);
        }

        s.b[1194] = (((s.v[910] - s.v[981]) - s.v[979]) < 80.0);
        s.v[1194] = if s.b[1194] { 1.0 } else { 0.0 };

        if s.b[1194] {
            s.store_exp_ad(793, A::add_scaled_inputs3(s.ad_value(910), 1.0, s.ad_value(981), (-1.0), s.ad_value(979), -1.0));
        }

        if (!s.b[1194]) {
            let assign21100_ad_e21500: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(910), 1.0, s.ad_value(981), (-1.0), s.ad_value(979), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(910), 1.0, s.ad_value(981), (-1.0), s.ad_value(979), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(910), 1.0, s.ad_value(981), (-1.0), s.ad_value(979), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(793, assign21100_ad_e21500, 1.0, 5.54062e34);
        }

        s.store_mul(987, 884, 793);

        s.v[990] = 0.0;

        s.v[991] = 0.0;

        s.v[988] = 0.0;

        s.v[989] = 0.0;

        s.v[992] = 0.0;

        s.v[993] = 0.0;

        s.b[1195] = (s.v[917] > 1e-6);
        s.v[1195] = if s.b[1195] { 1.0 } else { 0.0 };

        if s.b[1195] {
            s.store_mul(988, 986, 885);
            s.store_mul(989, 987, 886);
            s.store_add_scaled_inputs(990, 988, 1.0, 983, 2.0);
            s.store_add_scaled_inputs(991, 989, 1.0, 984, 2.0);
            s.store_add_scaled_inputs3_indices(992, 982, 2.0, 988, 1.0, 989, 1.0);
        }

        s.b[1196] = (((s.v[985]) as f64).abs() > 0.005);
        s.v[1196] = if s.b[1196] { 1.0 } else { 0.0 };

        if (s.b[1195] && s.b[1196]) {
            s.store_add_scaled_products3(2, s.ad_value(990), s.ad_value(991), 1.0, A::offset(s.ad_value(980), 2.0), s.ad_value(991), 2.0, A::offset(s.ad_value(981), 2.0), s.ad_value(990), 2.0);
            s.store_div_scaled_product_by_product(993, s.ad_value(985), s.ad_value(992), (-4.0), s.ad_value(982), s.ad_value(2), 1.0);
        }

        if (s.b[1195] && (!s.b[1196])) {
            s.store_offset_scaled_ad(2, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(985), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(985), 1.0, A::scale(s.ad_value(985), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_add_scaled_products3(3, s.ad_value(990), s.ad_value(986), 1.0, s.ad_value(991), s.ad_value(987), 1.0, A::mul3(s.ad_value(990), s.ad_value(991), s.ad_value(982)), A::offset(A::mul(s.ad_value(982), s.ad_value(2)), 1.0), 1.0);
            s.store_div_scaled_product3_by_product(993, s.ad_value(986), s.ad_value(987), s.ad_value(992), 1.0, s.ad_value(982), s.ad_value(3), 1.0);
        }

        s.store_add_ad_rhs(994, 979, A::ln(s.ad_value(982)));

        s.store_scaled_add(995, 917, 982, 0.5);

        s.store_sub(996, 994, 930);

        s.v[999] = 1.0;

        s.b[1197] = (p.p9 > 0.0);
        s.v[1197] = if s.b[1197] { 1.0 } else { 0.0 };

        if s.b[1197] {
            s.store_div_scaled_inputs2_indices(997, 918, 0.5, 983, 0.5, 911, 1.0);
            s.store_scaled_add_offset_sqrt_square_offset(997, 997, 1e-5, (-1e-5), 1.0, 0.5);
            s.store_sub_scaled_ad_lhs(1, A::sqrt(A::add_scaled_product(A::div(s.ad_value(997), s.ad_value(227)), 1.0, s.ad_value(250), s.ad_value(250), 0.25)), 250, 0.5);
            s.store_mul_powf_ad_lhs(998, s.ad_value(1), 2.0, 227);
            s.store_sub_from_scalar_div_indices(999, 1.0, 998, 997);
        }

        s.b[1198] = ((s.v[983] / 2.0) < 80.0);
        s.v[1198] = if s.b[1198] { 1.0 } else { 0.0 };

        if s.b[1198] {
            s.store_ln_one_plus_exp_scaled_input(2, 983, 0.5);
        }

        if (!s.b[1198]) {
            s.store_scale(2, 983, 0.5);
        }

        s.store_scale(1000, 2, 2.0);

        s.b[1199] = ((s.v[984] / 2.0) < 80.0);
        s.v[1199] = if s.b[1199] { 1.0 } else { 0.0 };

        if s.b[1199] {
            s.store_ln_one_plus_exp_scaled_input(3, 984, 0.5);
        }

        if (!s.b[1199]) {
            s.store_scale(3, 984, 0.5);
        }

        s.store_scale(1001, 3, 2.0);

        s.store_sub(1002, 1001, 984);

        s.store_sub(1003, 1000, 983);

        s.store_add_scaled_products_indices(1004, 270, 1000, 1.0, 271, 1002, 1.0);

        s.store_add_scaled_products_indices(1005, 270, 1001, 1.0, 271, 1003, 1.0);

        s.store_scaled_add(1006, 931, 1000, 0.5);

        s.store_scaled_add(1007, 932, 1001, 0.5);

        s.store_div_from_scalar_add_ad(0, 1.0, s.ad_value(1006), s.ad_value(1007));

        s.store_mul3_lhs(1008, 995, 1006, 0);

        s.store_mul3_lhs(1009, 995, 1007, 0);

        s.store_scaled_add(1010, 933, 1002, 0.5);

        s.store_scaled_add(1011, 934, 1003, 0.5);

        s.store_scaled_add(1012, 935, 1004, 0.5);

        s.store_scaled_add(1013, 936, 1005, 0.5);

        s.store_mul_product3_rhs(1014, 999, s.ad_value(1006), s.ad_value(191), A::exp(A::mul(s.ad_value(40), s.ad_value(295))), 1.0);

        s.store_mul_ad_product_rhs(1015, 1007, s.ad_value(192), A::exp(A::mul(s.ad_value(40), s.ad_value(295))));

        s.store_add(1016, 1014, 1015);

        s.store_mul_add_scaled_product_rhs(2, 50, s.ad_value(1010), 1.0, s.ad_value(51), s.ad_value(1011), 1.0);

        s.store_scaled_add_offset_sqrt_square_offset(3, 2, 1.0, 1.0, 0.01, 0.5);

        s.store_scaled_add_ad(4, A::scale_offset(s.ad_value(2), 0.2, 1.0), A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(2), 0.2, 1.0), A::scale_offset(s.ad_value(2), 0.2, 1.0)), 0.01)), 0.5);

        s.store_div(1017, 3, 4);

        s.store_mul_ad_product_rhs(1018, 33, A::add_scaled_product(A::offset(A::mul(s.ad_value(41), s.ad_value(1010)), 1.0), 1.0, s.ad_value(42), s.ad_value(1011), 1.0), A::exp(A::mul_scaled_lhs(s.ad_value(44), -1.0, A::ln(A::add_scaled_product(A::offset(A::mul(s.ad_value(1008), s.ad_value(268)), 1.0), 1.0, s.ad_value(1009), s.ad_value(269), 1.0)))));

        s.b[1200] = (s.v[56] == 0.0);
        s.v[1200] = if s.b[1200] { 1.0 } else { 0.0 };

        if s.b[1200] {
            s.store_scalar(4, 1.0);
        }

        s.b[1201] = (s.v[56] < 0.0);
        s.v[1201] = if s.b[1201] { 1.0 } else { 0.0 };

        if ((!s.b[1200]) && s.b[1201]) {
            s.store_mul_exp_ad_rhs(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(995), 1e-12))));
            s.store_sub_from_scalar(4, 1.0, 2);
        }

        if ((!s.b[1200]) && (!s.b[1201])) {
            s.store_mul_exp_ad_rhs(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(995), 1e-12))));
            s.store_div_from_scalar_offset_input(4, 1.0, 2, 1.0);
        }

        s.store_mul_add_scaled_product_rhs(1019, 943, s.ad_value(54), 1.0, s.ad_value(995), s.ad_value(4), 1.0);

        s.store_add_scaled_inputs_product_first_ad(1020, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1012)), 1e-6)))), 1.0), 1.0, 1018, 1.0, 38, 1019, 1.0);

        s.store_add_scaled_inputs_product_first_ad(1021, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1013)), 1e-6)))), 1.0), 1.0, 1018, 1.0, 39, 1019, 1.0);

        s.store_div_scaled_product_add_scaled_denominator(1022, 1017, 1016, 1.0, A::div(s.ad_value(1014), s.ad_value(1020)), 1.0, A::div(s.ad_value(1015), s.ad_value(1021)), 1.0, 1.0);

        s.store_div_from_scalar_offset_input(1023, 1.0, 995, 4.0);

    }

    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1202] = (s.v[65] > 0.0);
        s.v[1202] = if s.b[1202] { 1.0 } else { 0.0 };

        if s.b[1202] {
            s.store_div_from_scalar_offset_ad(0, 1.0, A::mul(s.ad_value(65), s.ad_value(1009)), 1.0);
        }

        if (!s.b[1202]) {
            s.store_sub_from_scalar_scaled_mul(0, 1.0, 65, 1009, 1.0);
        }

        s.store_mul3_lhs(1024, 995, 1023, 0);

        s.store_mul_ln_ad_lhs(1025, A::offset(A::div_scaled_inputs2(s.ad_value(339), 1.0, s.ad_value(979), (-1.0), A::add_scaled_product(A::mul3(s.ad_value(67), s.ad_value(995), s.ad_value(995)), 1.0, s.ad_value(66), s.ad_value(227), 1.0), 1.0), 1.0), 1024);

        s.store_mul(1026, 877, 1025);

        s.store_div_from_scalar_offset_ad(1027, 1.0, A::mul_offset_rhs(s.ad_value(1026), s.ad_value(1026), 1.0), 1.0);

        s.store_div_scaled_value_offset_denominator(955, s.ad_value(1006), 100.0, s.ad_value(1006), 100.0, 1.0);

        s.b[1203] = (s.v[61] < 0.0);
        s.v[1203] = if s.b[1203] { 1.0 } else { 0.0 };

        if s.b[1203] {
            s.store_div_from_scalar_sub_from_scalar_ad(956, 1.0, 1.0, A::mul(s.ad_value(61), s.ad_value(955)));
        }

        if (!s.b[1203]) {
            s.store_offset_mul(956, 61, 955, 1.0);
        }

        s.store_div_scaled_value_offset_denominator(957, s.ad_value(1007), 100.0, s.ad_value(1007), 100.0, 1.0);

        s.b[1204] = (s.v[62] < 0.0);
        s.v[1204] = if s.b[1204] { 1.0 } else { 0.0 };

        if s.b[1204] {
            s.store_div_from_scalar_sub_from_scalar_ad(958, 1.0, 1.0, A::mul(s.ad_value(62), s.ad_value(957)));
        }

        if (!s.b[1204]) {
            s.store_offset_mul(958, 62, 957, 1.0);
        }

        s.store_mul_ad_affine_product_rhs(1028, 875, s.ad_value(996), A::add(s.ad_value(956), s.ad_value(958)), 0.5, 0.0);

        s.store_div_ad_rhs(1029, 1028, A::mul(s.ad_value(1022), s.ad_value(1027)));

        s.store_square(1030, 1029);

        s.store_sqrt_offset_input(1031, 1030, 1.0);

        s.store_div_scaled_offset_numerator(1032, s.ad_value(1030), 1.5, 1.0, s.ad_value(1031), 1.0);

        s.b[1205] = (p.p13 > 0.0);
        s.v[1205] = if s.b[1205] { 1.0 } else { 0.0 };

        if s.b[1205] {
            s.store_mul_scaled_exp_ln_input_rhs(2, 258, 0.6, A::offset(A::square(s.ad_value(1006)), 60.0), (-0.1666666666667));
            s.store_mul_scaled_exp_ln_input_rhs(3, 258, 0.6, A::offset(A::square(s.ad_value(1007)), 60.0), (-0.1666666666667));
            s.store_div_scaled_offset_numerator(1033, A::mul(s.ad_value(911), s.ad_value(2)), 1.0, 1.0, s.ad_value(892), 1.0);
            s.store_div_scaled_offset_numerator(1034, A::mul(s.ad_value(912), s.ad_value(3)), 1.0, 1.0, s.ad_value(893), 1.0);
        }

        if (!s.b[1205]) {
            s.store_scalar(1033, 1.0);
            s.store_scalar(1034, 1.0);
        }

        s.b[1206] = (s.v[917] > 1e-6);
        s.v[1206] = if s.b[1206] { 1.0 } else { 0.0 };

        s.b[1207] = (s.v[982] > 1e-6);
        s.v[1207] = if s.b[1207] { 1.0 } else { 0.0 };

        s.b[1208] = (((s.v[991]) as f64).abs() < 0.01);
        s.v[1208] = if s.b[1208] { 1.0 } else { 0.0 };

        if ((s.b[1206] && s.b[1207]) && s.b[1208]) {
            s.store_div_scaled_inputs2_mixed_aia(0, A::offset(s.ad_value(980), 2.0), 1.0, 990, 0.5, A::mul_offset_lhs(s.ad_value(981), 2.0, s.ad_value(990)), 1.0);
            s.store_mul(2, 0, 991);
            s.store_square(3, 2);
            s.store_add_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
            s.store_add_scaled_product_indices(5, 4, 1.0, 2, 3, (-1.0));
            s.store_div_scaled_inputs2_mixed_iaa(2, 984, 1.0, A::mul3_scaled_output(s.ad_value(985), A::sub(s.ad_value(0), A::div_from_scalar(1.0, s.ad_value(990))), s.ad_value(5), 2.0), (-1.0), A::offset(s.ad_value(981), 2.0), 1.0);
            s.store_div_scaled_inputs2_mixed_aii(1035, A::div_scaled_add_product(s.ad_value(986), (-1.0), s.ad_value(993), s.ad_value(982), 1.0, s.ad_value(990), 1.0), 1.0, 2, (-1.0), 982, 1.0);
            s.store_div_scaled_product_offset_denominator(1036, s.ad_value(1035), s.ad_value(982), 1.0, s.ad_value(1035), 1.0, 1.0);
        }

        if ((s.b[1206] && s.b[1207]) && (!s.b[1208])) {
            s.store_sub_ad(1035, A::div_scaled_product_by_product(s.ad_value(993), s.ad_value(992), 1.0, s.ad_value(990), s.ad_value(991), 1.0), A::div_scaled_inputs2(A::div(s.ad_value(986), s.ad_value(990)), 1.0, A::div(s.ad_value(987), s.ad_value(991)), 1.0, s.ad_value(982), 1.0));
            s.store_div_scaled_product_offset_denominator(1036, s.ad_value(1035), s.ad_value(982), 1.0, s.ad_value(1035), 1.0, 1.0);
        }

        if (s.b[1206] && (!s.b[1207])) {
            s.copy_ad(1036, 953);
        }

        if s.b[1206] {
            s.store_sub(2, 1036, 960);
            s.store_offset_scaled_mul(3, 2, 2, 36.0, 1.0);
        }

        s.b[1209] = (((s.v[2]) as f64).abs() > 0.001);
        s.v[1209] = if s.b[1209] { 1.0 } else { 0.0 };

        if (s.b[1206] && s.b[1209]) {
            s.store_sub(4, 982, 917);
            s.store_add_scaled_product_indices(1037, 4, 1.0, 1036, 996, (-1.0));
            s.store_add_scaled_product_indices(1038, 4, 1.0, 960, 996, (-1.0));
            s.store_sqrt_square_add(1039, 1037, 3);
            s.store_sqrt_square_add(1040, 1038, 3);
            s.store_mul_ad(1041, A::div_from_scalar(0.25, s.ad_value(2)), A::add_scaled_products3(s.ad_value(1040), s.ad_value(1037), 1.0, s.ad_value(1039), s.ad_value(1038), (-1.0), s.ad_value(3), A::ln(A::div_scaled_inputs2(s.ad_value(1038), 1.0, s.ad_value(1040), 1.0, A::add(s.ad_value(1037), s.ad_value(1039)), 1.0)), 1.0));
        }

        if (s.b[1206] && (!s.b[1209])) {
            s.store_mul(4, 996, 2);
            s.store_div_scaled_product3_mixed_iiia(1041, 996, 4, 4, ((-0.25) * 0.1666666666667), A::sqrt(s.ad_value(3)), 1.0);
        }

        if (!s.b[1206]) {
            s.copy_ad(1036, 953);
            s.store_scalar(1041, 0.0);
        }

        s.store_add_scaled_inputs3_mixed_aii(1042, A::add_scaled_product(s.ad_value(1041), 1.0, s.ad_value(995), s.ad_value(996), 1.0), 1.0, 917, 1.0, 982, -1.0);

        s.b[1210] = (s.v[917] > 1e-6);
        s.v[1210] = if s.b[1210] { 1.0 } else { 0.0 };

        s.b[1211] = (s.v[1042] > 1e-30);
        s.v[1211] = if s.b[1211] { 1.0 } else { 0.0 };

        if (s.b[1210] && s.b[1211]) {
            s.store_div_add_scaled_inputs_rhs_mixed_ai(1043, 926, A::div(s.ad_value(922), s.ad_value(917)), 1.0, 929, -1.0);
            s.store_div_add_scaled_inputs_rhs_mixed_ai(1044, 990, A::div(s.ad_value(986), s.ad_value(982)), 1.0, 993, -1.0);
            s.store_div_scaled_inputs2_indices(1045, 1043, 1.0, 1044, (-1.0), 1042, 1.0);
            s.store_div_add_scaled_inputs_rhs_mixed_ai(1046, 927, A::div(s.ad_value(923), s.ad_value(917)), 1.0, 929, -1.0);
            s.store_div_add_scaled_inputs_rhs_mixed_ai(1047, 991, A::div(s.ad_value(987), s.ad_value(982)), 1.0, 993, -1.0);
            s.store_div_scaled_inputs2_indices(1048, 1046, 1.0, 1047, (-1.0), 1042, 1.0);
        }

        if (s.b[1210] && (!s.b[1211])) {
            s.store_scalar(1045, 0.0);
            s.store_scalar(1048, 0.0);
        }

        if (!s.b[1210]) {
            s.store_mul_add_scaled_inputs_rhs(1049, 948, A::div(s.ad_value(885), s.ad_value(951)), (-2.0), s.ad_value(954), (-2.0));
            s.store_mul_add_scaled_inputs_rhs(1050, 949, A::div(s.ad_value(886), s.ad_value(952)), (-2.0), s.ad_value(954), (-2.0));
            s.store_mul_sub_lhs(0, 1050, 1049, 954);
            s.store_mul(2, 1049, 885);
            s.store_mul(3, 1050, 886);
            s.store_add(4, 2, 3);
            s.store_offset_ad(5, A::add_scaled_products(s.ad_value(948), s.ad_value(885), 2.0, s.ad_value(949), s.ad_value(886), 2.0), 3.0);
            s.store_div_scaled_inputs3(1051, s.ad_value(3), 1.0, s.ad_value(0), 1.0, A::div(s.ad_value(4), s.ad_value(951)), -1.0, s.ad_value(5), 1.0);
            s.store_div_scaled_inputs3(1052, s.ad_value(2), 1.0, s.ad_value(0), (-1.0), A::div(s.ad_value(4), s.ad_value(952)), -1.0, s.ad_value(5), 1.0);
            s.store_mul_add_scaled_product_rhs(1045, 951, s.ad_value(954), -1.0, s.ad_value(1051), s.ad_value(951), -1.0);
            s.store_mul_add_scaled_product_rhs(1048, 952, s.ad_value(954), -1.0, s.ad_value(1052), s.ad_value(952), -1.0);
        }

        s.store_mul(1053, 1045, 1032);

        s.store_mul(1054, 1048, 1032);

        s.store_scaled_sub(1055, 983, 918, 0.5);

        s.store_scaled_sub(1056, 984, 919, 0.5);

        s.store_mul(1057, 1055, 1053);

        s.store_mul(1058, 1056, 1054);

        s.copy_ad(383, 879);

        s.copy_ad(384, 883);

        s.copy_ad(385, 884);

        s.copy_ad(386, 885);

        s.copy_ad(387, 886);

        s.copy_ad(388, 913);

        s.copy_ad(389, 914);

        s.copy_ad(390, 898);

        s.copy_ad(391, 897);

        s.copy_ad(392, 916);

        s.copy_ad(393, 901);

        s.copy_ad(394, 902);

        s.copy_ad(395, 903);

        s.copy_ad(396, 904);

        s.copy_ad(397, 905);

        s.copy_ad(398, 908);

        s.copy_ad(399, 910);

        s.copy_ad(400, 909);

        s.copy_ad(401, 911);

        s.copy_ad(402, 912);

        s.copy_ad(403, 917);

        s.copy_ad(404, 918);

        s.copy_ad(405, 919);

        s.copy_ad(406, 930);

        s.copy_ad(407, 960);

        s.copy_ad(408, 983);

        s.copy_ad(409, 984);

        s.copy_ad(411, 979);

        s.copy_ad(412, 980);

        s.copy_ad(413, 982);

        s.copy_ad(414, 994);

        s.copy_ad(415, 995);

        s.copy_ad(416, 999);

        s.copy_ad(417, 1006);

        s.copy_ad(418, 1007);

        s.copy_ad(419, 1008);

        s.copy_ad(420, 1009);

        s.copy_ad(421, 1016);

        s.copy_ad(422, 1022);

        s.copy_ad(423, 1023);

        s.copy_ad(424, 1025);

        s.copy_ad(425, 1027);

        s.copy_ad(426, 1031);

        s.v[427] = s.v[1028];

        s.copy_ad(428, 1030);

        s.copy_ad(429, 1032);

        s.copy_ad(430, 1033);

        s.copy_ad(431, 1034);

        s.copy_ad(432, 1036);

        s.copy_ad(433, 1042);

        s.copy_ad(434, 1053);

        s.copy_ad(435, 1045);

        s.copy_ad(436, 1055);

        s.copy_ad(437, 1056);

        s.copy_ad(438, 1057);

        s.copy_ad(439, 1058);

        s.store_div_scaled_inputs_mixed_ia(342, 421, p.p35, A::add(s.ad_value(417), s.ad_value(418)), 1.0);

        s.store_mul_add_scaled_product_rhs(343, 424, s.ad_value(63), 1.0, s.ad_value(275), s.ad_value(423), 1.0);

        s.store_mul_offset_ad_lhs(344, A::mul_offset_rhs(s.ad_value(343), s.ad_value(343), 1.0), 1.0, 425);

        s.store_mul3_lhs(345, 422, 425, 426);

        s.b[1212] = (p.p13 > 0.0);
        s.v[1212] = if s.b[1212] { 1.0 } else { 0.0 };

        if s.b[1212] {
            s.store_div_scaled_inputs2_mixed_iia(346, 417, 1.0, 418, 1.0, A::add(A::div(s.ad_value(417), s.ad_value(430)), A::div(s.ad_value(418), s.ad_value(431))), 1.0);
        }

        if (!s.b[1212]) {
            s.store_scalar(346, 1.0);
        }

        s.store_mul_square_lhs(347, 226, 342);

        s.store_div_scaled_product_by_product(348, A::mul3(s.ad_value(347), s.ad_value(390), s.ad_value(433)), s.ad_value(344), 1.0, s.ad_value(345), s.ad_value(346), 1.0);

        s.store_mul_neg_lhs(704, 330, 224);

        s.store_mul_neg_lhs(705, 332, 224);

        s.store_add_scaled_product_indices(0, 234, 1.0, 163, 224, p.p14);

        s.store_add(706, 704, 0);

        s.store_add(707, 705, 0);

        s.v[714] = 0.0;

        s.v[715] = 0.0;

        s.v[716] = 0.0;

        s.v[717] = 0.0;

        s.store_div_ad_lhs(708, A::sqrt(A::mul3_scaled_output(s.ad_value(19), s.ad_value(229), s.ad_value(224), (2.0 * 1.602176565e-19))), 241);

        s.store_square(709, 708);

        s.store_offset_scaled(710, 708, 0.707106781186545, 1.0);

        let assign23410_e23016: f64 = (1e-5 * s.v[710]);
        s.v[711] = assign23410_e23016;

        s.store_div_from_scalar(712, 1.0, 710);

        s.store_div_from_scalar_offset_scaled_input(713, 1.0, 708, 0.7324648775608221, 1.25);

        s.b[1213] = (((p.p3 > 0.0) && ((s.v[69] > 0.0) || (s.v[71] > 0.0))) || ((p.p4 > 0.0) && (s.v[89] > 0.0)));
        s.v[1213] = if s.b[1213] { 1.0 } else { 0.0 };

        s.b[1214] = (((s.v[704]) as f64).abs() <= s.v[711]);
        s.v[1214] = if s.b[1214] { 1.0 } else { 0.0 };

        if (s.b[1213] && s.b[1214]) {
            s.store_mul_neg_lhs(714, 704, 712);
        }

        s.b[1215] = (s.v[704] < (-s.v[711]));
        s.v[1215] = if s.b[1215] { 1.0 } else { 0.0 };

        if ((s.b[1213] && (!s.b[1214])) && s.b[1215]) {
            s.store_neg(683, 704);
            s.store_scaled_mul(684, 683, 712, 1.25);
            s.store_scaled_sub_ad(685, A::offset(s.ad_value(684), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(684), (-6.0), A::offset(s.ad_value(684), (-6.0))), 64.0)), 0.5);
            s.store_add_scaled_products_mixed_aaia(686, A::sub(s.ad_value(683), s.ad_value(685)), A::sub(s.ad_value(683), s.ad_value(685)), 1.0, 709, A::offset(s.ad_value(685), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(687, 683, 2.0, 685, (-2.0), 709, -1.0);
            s.store_sub_ad_lhs(688, A::ln(A::div(s.ad_value(686), s.ad_value(709))), 685);
            s.store_add(689, 686, 687);
            s.store_add_scaled_square_product_mixed_iia(690, 689, 1.0, 688, A::add_scaled_product(s.ad_value(686), (-1.0), s.ad_value(687), s.ad_value(687), 0.5), 1.0);
            s.store_add_ad_rhs(691, 690, A::mul3(A::mul3(A::div(s.ad_value(689), s.ad_value(690)), s.ad_value(688), s.ad_value(688)), s.ad_value(687), A::sub_scaled_inputs(A::square(s.ad_value(687)), 0.3333333333333, s.ad_value(686), 1.0)));
            s.store_add_ad_rhs(692, 685, A::div_scaled_product3(s.ad_value(686), s.ad_value(689), s.ad_value(688), 1.0, s.ad_value(691), 1.0));
        }

        s.b[1216] = (((s.v[692]) as f64).abs() < 80.0);
        s.v[1216] = if s.b[1216] { 1.0 } else { 0.0 };

        if (((s.b[1213] && (!s.b[1214])) && s.b[1215]) && s.b[1216]) {
            s.store_exp(693, 692);
        }

        s.b[1217] = (s.v[692] < (-80.0));
        s.v[1217] = if s.b[1217] { 1.0 } else { 0.0 };

    }
}
