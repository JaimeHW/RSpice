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

        s.b[525] = (p.p10 == 1.0);
        s.v[525] = if s.b[525] { 1.0 } else { 0.0 };

        if s.b[525] {
            s.store_scalar(8, (0.5 * ((s.v[0] + (p.p17 + (p.p18 * s.v[0]))) + (((((s.v[0] - (p.p17 + (p.p18 * s.v[0]))) * (s.v[0] - (p.p17 + (p.p18 * s.v[0])))) + p.p19)) as f64).sqrt())));
            s.store_scaled_add_ad(221, A::offset(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), 600.0), A::sqrt(A::offset(A::mul_offset_lhs(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), (-600.0), A::offset(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), (-600.0))), 0.01)), 0.5);
        }

        if (!s.b[525]) {
            s.store_scalar(8, (0.5 * ((s.v[0] + 1.0) + (((((s.v[0] - 1.0) * (s.v[0] - 1.0)) + 0.001)) as f64).sqrt())));
            s.store_scalar(221, 600.0);
        }

        s.b[526] = (((p.p0 == 0.0) && (p.p172 > 0.0)) || ((p.p0 > 0.0) && (p.p439 > 0.0)));
        s.v[526] = if s.b[526] { 1.0 } else { 0.0 };

        let (assign80_e841,) = {
    if s.b[526] {
        (p.p5,)
    } else {
        (s.v[6],)
    }
};
        s.v[6] = assign80_e841;

        let (assign90_e846,) = {
    if (!s.b[526]) {
        (0.0,)
    } else {
        (s.v[6],)
    }
};
        s.v[6] = assign90_e846;

        s.v[215] = 0.0;

        s.copy_ad(213, 8);

        s.store_square(214, 213);

        s.store_offset(216, 213, (-s.v[7]));

        s.store_scale(217, 213, 1.0 / (s.v[7]));

        s.store_div_from_scalar(218, s.v[7], 213);

        s.store_scale(219, 213, 8.617332384961e-5);

        s.store_div_from_scalar(220, 1.0, 219);

        s.b[607] = (p.p0 == 0.0);
        s.v[607] = if s.b[607] { 1.0 } else { 0.0 };

        if s.b[607] {
            s.store_scalar(10, p.p23);
            s.store_scalar(9, p.p22);
            s.store_scalar(12, p.p25);
            s.store_scalar(11, p.p24);
            s.store_scalar(13, p.p30);
            s.store_scalar(529, p.p41);
            s.store_scalar(14, p.p42);
            s.store_scalar(15, p.p43);
            s.store_scalar(530, p.p44);
        }

        let (assign300_e914,) = {
    if s.b[607] {
        (1.0,)
    } else {
        (s.v[531],)
    }
};
        s.v[531] = assign300_e914;

        s.b[608] = (p.p45 < 0.0);
        s.v[608] = if s.b[608] { 1.0 } else { 0.0 };

        let (assign320_e924,) = {
    if (s.b[607] && s.b[608]) {
        let assign320_e922: f64 = (-1.0);
        (assign320_e922,)
    } else {
        (s.v[531],)
    }
};
        s.v[531] = assign320_e924;

        if s.b[607] {
            s.store_scalar(532, ((((p.p45) as f64).abs()).min(1e19) * 1000000.0));
            s.store_scalar(16, 1.0);
        }

        s.b[609] = (p.p46 < 0.0);
        s.v[609] = if s.b[609] { 1.0 } else { 0.0 };

        if (s.b[607] && s.b[609]) {
            s.store_scalar(16, (-1.0));
        }

        if s.b[607] {
            s.store_scalar(533, (((((p.p46) as f64).abs()).max(1e16)).min(1e21) * 1000000.0));
            s.store_scalar(17, p.p47);
            s.store_scalar(18, p.p48);
            s.store_scalar(19, (p.p49 * 1000000.0));
            s.store_scalar(20, (p.p50 * 1000000.0));
            s.store_scalar(179, p.p51);
            s.store_scalar(180, p.p52);
            s.store_scalar(23, p.p53);
            s.store_scalar(24, (p.p54 * 1000000.0));
            s.store_scalar(25, p.p55);
            s.store_scalar(26, p.p56);
            s.store_scalar(27, p.p57);
            s.store_div_scaled_product_indices(28, 27, 530, p.p58, 529, 1.0);
            s.store_scalar(29, (p.p59 * 1000000.0));
            s.store_scalar(30, p.p60);
            s.store_scalar(534, p.p61);
            s.store_scalar(183, p.p62);
            s.store_div_scaled_product_indices(184, 183, 530, p.p63, 529, 1.0);
            s.store_scalar(34, p.p64);
            s.store_scalar(35, p.p65);
            s.store_scalar(36, p.p66);
            s.store_scalar(37, p.p67);
            s.store_scalar(187, p.p68);
            s.store_scale(188, 187, p.p69);
            s.store_scalar(40, p.p70);
            s.store_scalar(191, p.p71);
            s.store_scalar(41, p.p72);
            s.store_scalar(42, p.p73);
            s.store_scalar(43, p.p74);
            s.store_scalar(192, p.p75);
            s.store_scalar(45, p.p76);
            s.store_scalar(535, p.p77);
            s.store_scalar(536, p.p78);
            s.store_scalar(189, p.p79);
            s.store_scalar(48, p.p80);
            s.store_scalar(190, p.p81);
            s.store_scalar(49, p.p82);
            s.store_scalar(193, p.p83);
            s.store_scalar(51, p.p84);
            s.store_scalar(52, p.p85);
            s.store_scalar(537, p.p86);
            s.store_scalar(194, p.p87);
            s.store_scalar(54, p.p88);
            s.store_scalar(55, p.p89);
            s.store_scalar(56, p.p90);
            s.store_scalar(57, p.p91);
            s.store_scalar(58, p.p92);
            s.store_scalar(195, p.p93);
            s.store_scalar(60, p.p94);
            s.store_scalar(61, p.p95);
            s.store_scalar(62, p.p96);
            s.store_scalar(538, p.p97);
            s.store_scalar(63, p.p98);
            s.store_scalar(64, p.p99);
            s.store_scalar(65, p.p100);
            s.store_scalar(66, p.p101);
            s.store_scalar(67, p.p102);
            s.store_scalar(75, p.p103);
            s.store_scalar(197, p.p104);
            s.store_scalar(198, p.p105);
            s.store_scalar(199, p.p106);
            s.store_scalar(202, p.p120);
            s.store_scalar(203, p.p121);
            s.store_scalar(200, p.p107);
            s.store_scalar(201, p.p108);
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
            s.store_scalar(204, p.p126);
            s.store_scalar(205, p.p127);
            s.store_scalar(93, p.p128);
            s.store_scalar(94, p.p129);
            s.store_scalar(95, p.p130);
            s.store_scalar(96, p.p131);
            s.store_scalar(97, p.p132);
            s.store_scalar(98, p.p133);
            s.store_scalar(112, p.p147);
            s.store_scalar(206, p.p148);
            s.store_scalar(114, p.p149);
            s.store_scalar(115, p.p150);
            s.store_scalar(99, p.p134);
            s.store_scalar(207, p.p135);
        }

    }

    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[607] {
            s.store_scalar(208, p.p136);
            s.store_scalar(102, p.p137);
            s.store_scalar(103, p.p138);
            s.store_scalar(104, p.p139);
            s.store_scalar(105, p.p140);
            s.store_div_scaled_product_indices(106, 105, 530, p.p141, 529, 1.0);
            s.store_scalar(107, p.p142);
            s.store_div_scaled_product_indices(108, 107, 530, p.p143, 529, 1.0);
            s.store_scalar(109, p.p144);
            s.store_scalar(209, p.p145);
            s.store_scalar(111, p.p146);
            s.store_scalar(116, p.p151);
            s.store_scalar(117, p.p152);
            s.store_scalar(118, (p.p153 * 1000000.0));
            s.store_scalar(119, p.p154);
            s.store_scalar(120, p.p155);
            s.copy_ad(181, 179);
            s.copy_ad(182, 180);
            s.copy_ad(135, 27);
            s.copy_ad(136, 28);
            s.copy_ad(185, 183);
            s.copy_ad(186, 184);
            s.copy_ad(196, 195);
            s.copy_ad(539, 538);
            s.copy_ad(158, 63);
        }

        s.b[610] = (p.p11 > 0.0);
        s.v[610] = if s.b[610] { 1.0 } else { 0.0 };

        if (s.b[607] && s.b[610]) {
            s.store_scalar(181, p.p51);
        }

        s.b[611] = param_given[156];
        s.v[611] = if s.b[611] { 1.0 } else { 0.0 };

        if ((s.b[607] && s.b[610]) && s.b[611]) {
            s.store_scalar(181, p.p156);
        }

        if (s.b[607] && s.b[610]) {
            s.store_scalar(182, p.p52);
        }

        s.b[612] = param_given[157];
        s.v[612] = if s.b[612] { 1.0 } else { 0.0 };

        if ((s.b[607] && s.b[610]) && s.b[612]) {
            s.store_scalar(182, p.p157);
        }

        if (s.b[607] && s.b[610]) {
            s.store_scalar(135, p.p57);
        }

        s.b[613] = param_given[158];
        s.v[613] = if s.b[613] { 1.0 } else { 0.0 };

        if ((s.b[607] && s.b[610]) && s.b[613]) {
            s.store_scalar(135, p.p158);
        }

        if (s.b[607] && s.b[610]) {
            s.store_div_scaled_product_indices(136, 135, 530, p.p58, 529, 1.0);
            s.store_scalar(185, p.p62);
        }

        s.b[614] = param_given[159];
        s.v[614] = if s.b[614] { 1.0 } else { 0.0 };

        if ((s.b[607] && s.b[610]) && s.b[614]) {
            s.store_scalar(185, p.p159);
        }

        if (s.b[607] && s.b[610]) {
            s.store_div_scaled_product_indices(186, 185, 530, p.p63, 529, 1.0);
            s.store_scalar(196, p.p93);
        }

        s.b[615] = param_given[160];
        s.v[615] = if s.b[615] { 1.0 } else { 0.0 };

        if ((s.b[607] && s.b[610]) && s.b[615]) {
            s.store_scalar(196, p.p160);
        }

        if (s.b[607] && s.b[610]) {
            s.store_scalar(539, p.p97);
        }

        s.b[616] = param_given[161];
        s.v[616] = if s.b[616] { 1.0 } else { 0.0 };

        if ((s.b[607] && s.b[610]) && s.b[616]) {
            s.store_scalar(539, p.p161);
        }

        if (s.b[607] && s.b[610]) {
            s.store_scalar(158, p.p98);
        }

        s.b[617] = param_given[162];
        s.v[617] = if s.b[617] { 1.0 } else { 0.0 };

        if ((s.b[607] && s.b[610]) && s.b[617]) {
            s.store_scalar(158, p.p162);
        }

        if s.b[607] {
            s.store_scalar(159, p.p163);
            s.store_scalar(160, p.p164);
            s.store_scalar(161, p.p165);
            s.store_scalar(162, p.p166);
            s.store_scalar(163, p.p167);
            s.store_scalar(164, p.p168);
            s.store_scalar(165, p.p169);
            s.store_scalar(166, p.p170);
            s.store_scalar(167, p.p171);
            s.store_scalar(210, p.p172);
            s.store_scalar(169, p.p173);
            s.store_scalar(170, p.p174);
        }

        let (assign1920_e1671,) = {
    if s.b[607] {
        (p.p175,)
    } else {
        (s.v[171],)
    }
};
        s.v[171] = assign1920_e1671;

        let (assign1930_e1675,) = {
    if s.b[607] {
        (p.p176,)
    } else {
        (s.v[172],)
    }
};
        s.v[172] = assign1930_e1675;

        if s.b[607] {
            s.store_scalar(173, p.p177);
            s.store_scalar(174, p.p178);
            s.store_scalar(175, p.p179);
            s.store_scalar(176, p.p180);
            s.store_scalar(177, p.p181);
        }

        if (!s.b[607]) {
            s.store_scalar(584, (1.0 / p.p29));
            s.store_max_with_scalar_ad(528, A::scale(s.ad_value(584), p.p21), 1e-9);
            s.store_scale(10, 584, p.p23);
            s.store_scale(9, 584, p.p22);
            s.store_scale(12, 584, p.p25);
            s.store_scale(11, 584, p.p24);
            s.store_scalar(13, (p.p30 * p.p29));
            s.store_scalar(565, 1e-6);
            s.store_scalar(566, 1e-6);
            s.store_scale(567, 565, 1.0 / (p.p20));
            s.store_div(568, 566, 528);
            s.store_scaled_mul_scale_offset_inputs(569, 567, p.p188, 1.0, 568, p.p189, 1.0, p.p187);
            s.store_scaled_mul_scale_offset_inputs(570, 568, p.p193, 1.0, 567, p.p192, 1.0, p.p191);
            s.store_max_with_scalar_ad(571, A::offset(s.ad_value(569), ((p.p20) + ((-(2.0 * p.p190))))), 1e-9);
            s.store_max_with_scalar_ad(572, A::offset(A::add(s.ad_value(528), s.ad_value(570)), (-(2.0 * p.p194))), 1e-9);
            s.store_max_with_scalar_ad(573, A::offset(s.ad_value(569), ((((p.p20) + ((-(2.0 * p.p190))))) + (p.p195))), 1e-9);
            s.store_max_with_scalar_ad(574, A::offset(A::add(s.ad_value(528), s.ad_value(570)), (((-(2.0 * p.p194))) + (p.p196))), 1e-9);
            s.store_div(575, 565, 571);
            s.store_div(576, 566, 572);
            s.store_mul(577, 575, 576);
            s.store_max_with_scalar_ad(0, A::offset(s.ad_value(569), p.p20), 1e-9);
            s.store_div(578, 0, 565);
            s.store_max_with_scalar_ad(0, A::add(s.ad_value(528), s.ad_value(570)), 1e-9);
            s.store_div(579, 0, 566);
            s.store_scalar(529, p.p197);
            s.store_scalar(14, p.p198);
            s.store_scalar(15, p.p199);
            s.store_scalar(530, p.p200);
        }

        let (assign2360_e1996,) = {
    if (!s.b[607]) {
        (1.0,)
    } else {
        (s.v[531],)
    }
};
        s.v[531] = assign2360_e1996;

        s.b[618] = (p.p201 < 0.0);
        s.v[618] = if s.b[618] { 1.0 } else { 0.0 };

        let (assign2380_e2007,) = {
    if ((!s.b[607]) && s.b[618]) {
        let assign2380_e2005: f64 = (-1.0);
        (assign2380_e2005,)
    } else {
        (s.v[531],)
    }
};
        s.v[531] = assign2380_e2007;

        if (!s.b[607]) {
            s.store_scalar(532, ((((p.p201) as f64).abs()).min(1e19) * 1000000.0));
            s.store_scalar(16, 1.0);
        }

        s.b[619] = (p.p202 < 0.0);
        s.v[619] = if s.b[619] { 1.0 } else { 0.0 };

        if ((!s.b[607]) && s.b[619]) {
            s.store_scalar(16, (-1.0));
        }

        if (!s.b[607]) {
            s.store_scalar(533, (((((p.p202) as f64).abs()).max(1e16)).min(1e21) * 1000000.0));
            s.store_scalar(17, p.p203);
            s.store_scalar(18, p.p204);
            s.store_scalar(19, (p.p205 * 1000000.0));
            s.store_scalar(20, (p.p206 * 1000000.0));
            s.store_div_scaled_inputs(0, A::powf(s.ad_value(575), p.p209), p.p208, A::scale_offset(A::powf(s.ad_value(575), p.p211), p.p210, 1.0), 1.0);
            s.store_add_scaled_inputs3_offset_indices(179, 0, 1.0, 576, p.p212, 577, p.p213, p.p207);
            s.store_offset_mul_ad(180, A::div_scaled_inputs(s.ad_value(530), p.p215, s.ad_value(529), 1.0), s.ad_value(0), p.p214);
            s.store_mul3_ad_scaled_output(23, A::scale_offset(s.ad_value(575), p.p217, 1.0), A::scale_offset(s.ad_value(576), p.p218, 1.0), A::scale_offset(s.ad_value(577), p.p219, 1.0), p.p216);
            s.store_offset_scaled(603, 575, ((p.p221) * ((p.p220 * 1000000.0))), (p.p220 * 1000000.0));
            s.store_min_with_scalar_ad(24, A::max_with_scalar(s.ad_value(603), 1e25), 1e28);
            s.store_scalar(25, p.p222);
            s.store_scalar(26, p.p223);
            s.store_sub_from_scalar(224, 1.0, 15);
            s.store_add_scaled_inputs(225, 224, 1.04479e-10, 15, 1.43438e-10);
            s.store_div_ad_lhs(580, A::sqrt(A::mul3_scaled_output(s.ad_value(225), s.ad_value(14), A::offset(s.ad_value(529), 4e-10), 1.0 / (3.45313e-11))), 571);
            s.store_scaled_mul_scale_offset_rhs_ad(540, A::powf(s.ad_value(580), p.p225), 576, p.p226, 1.0, (p.p224 * 2.0));
            s.store_min_with_scalar_ad(27, A::max_with_scalar(s.ad_value(540), 0.0), 5.0);
            s.store_div_scaled_product_indices(28, 27, 530, p.p227, 529, 1.0);
            s.store_scalar(29, (p.p228 * 1000000.0));
            s.store_scalar(30, p.p229);
            s.store_scale(545, 576, p.p230);
        }

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[607]) {
            s.store_min_with_scalar_ad(534, A::max_with_scalar(s.ad_value(545), (-1.0)), 1.0);
            s.store_mul_ad(0, A::powf(s.ad_value(580), p.p232), A::scale_offset(s.ad_value(576), p.p233, 1.0));
            s.store_scale(542, 0, p.p231);
            s.store_max_with_scalar(183, 542, 0.0);
            s.store_div_scaled_product_indices(184, 183, 530, p.p234, 529, 1.0);
            s.store_scale(34, 0, p.p235);
            s.store_scalar(35, p.p236);
            s.store_div_scaled_inputs_mixed_ia(36, 575, p.p237, A::max_with_scalar(A::scale_offset(s.ad_value(576), p.p238, 1.0), 0.001), 1.0);
            s.store_scalar(37, p.p239);
            s.store_div_scaled_inputs_mixed_ia(2, 571, -1.0, A::max_with_scalar(A::scale_offset(s.ad_value(576), p.p244, 1.0), 0.001), p.p243);
        }

        s.b[620] = (s.v[2] > (-80.0));
        s.v[620] = if s.b[620] { 1.0 } else { 0.0 };

        if ((!s.b[607]) && s.b[620]) {
            s.store_exp(3, 2);
        }

        if ((!s.b[607]) && (!s.b[620])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(3, 1.80485e-35, A::neg(s.ad_value(2)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(2)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (!s.b[607]) {
            s.store_scale(4, 571, (-1.0 / (p.p246)));
        }

        s.b[621] = (s.v[4] > (-80.0));
        s.v[621] = if s.b[621] { 1.0 } else { 0.0 };

        if ((!s.b[607]) && s.b[621]) {
            s.store_exp(5, 4);
        }

        if ((!s.b[607]) && (!s.b[621])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(4)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if (!s.b[607]) {
            s.store_max_with_scalar_ad(581, A::add(A::offset(A::div_scaled_product_offset_rhs(A::scale_offset(s.ad_value(576), p.p242, 1.0), s.ad_value(3), (-1.0), p.p241, s.ad_value(2), 1.0), 1.0), A::div_scaled_offset_numerator(s.ad_value(5), p.p245, ((-1.0) * p.p245), s.ad_value(4), 1.0)), 1e-6);
            s.store_max_with_scalar_ad(582, A::add_scaled_product(A::scale_offset(s.ad_value(576), p.p247, 1.0), 1.0, s.ad_value(576), A::ln(A::scale_offset(s.ad_value(572), 1.0 / (p.p249), 1.0)), p.p248), 1e-6);
            s.store_mul_div_from_scalar_lhs(583, p.p240, 581, 582);
            s.store_div_scaled_product_indices(544, 583, 572, 1.0, 571, 1.0);
            s.store_max_with_scalar(187, 544, 1e-10);
            s.store_scale(188, 187, p.p250);
            s.store_mul3_ad_scaled_output(40, A::scale_offset(s.ad_value(575), p.p252, 1.0), A::scale_offset(s.ad_value(576), p.p253, 1.0), A::scale_offset(s.ad_value(577), p.p254, 1.0), p.p251);
            s.store_mul3_ad(546, A::scale_offset(A::powf(s.ad_value(575), p.p257), p.p256, p.p255), A::scale_offset(s.ad_value(576), p.p258, 1.0), A::scale_offset(s.ad_value(577), p.p259, 1.0));
            s.store_max_with_scalar(191, 546, 0.0);
            s.store_scalar(41, p.p260);
            s.store_scalar(42, p.p261);
            s.store_mul3_ad_scaled_output(43, A::scale_offset(s.ad_value(575), p.p263, 1.0), A::scale_offset(s.ad_value(576), p.p264, 1.0), A::scale_offset(s.ad_value(577), p.p265, 1.0), p.p262);
            s.store_scalar(192, p.p266);
            s.store_scalar(45, p.p267);
            s.store_scalar(535, p.p268);
            s.store_scalar(536, p.p269);
            s.store_scalar(189, p.p270);
            s.store_scalar(48, p.p271);
            s.store_scalar(190, p.p272);
            s.store_scalar(49, p.p273);
            s.store_mul3_ad(193, A::scale_offset(A::powf(s.ad_value(575), p.p276), p.p275, p.p274), A::scale_offset(s.ad_value(576), p.p277, 1.0), A::scale_offset(s.ad_value(577), p.p278, 1.0));
            s.store_scalar(51, p.p279);
            s.store_scalar(52, p.p280);
            s.store_scalar(537, p.p281);
            s.store_mul_scale_offset_rhs(547, 576, 576, ((p.p283) * (p.p282)), p.p282);
            s.store_max_with_scalar(194, 547, 0.0);
            s.store_scalar(54, p.p284);
            s.store_scalar(55, p.p285);
            s.store_scalar(56, p.p286);
            s.store_scalar(57, p.p287);
            s.store_scalar(58, p.p288);
            s.store_mul_ad(548, A::mul3(s.ad_value(583), A::scale_offset(A::powf(s.ad_value(575), p.p291), p.p290, p.p289), A::scale_offset(s.ad_value(576), p.p292, 1.0)), A::scale_offset(s.ad_value(577), p.p293, 1.0));
            s.store_max_with_scalar(195, 548, 0.0);
            s.store_mul3_ad_scaled_output(60, A::scale_offset(s.ad_value(575), p.p295, 1.0), A::scale_offset(s.ad_value(576), p.p296, 1.0), A::scale_offset(s.ad_value(577), p.p297, 1.0), p.p294);
            s.store_scalar(61, p.p298);
            s.store_scalar(62, p.p299);
            s.store_div_from_scalar_offset_ad(550, p.p300, A::div_scaled_inputs(A::powf(s.ad_value(575), p.p302), p.p301, A::scale_offset(A::powf(s.ad_value(575), p.p304), p.p303, 1.0), 1.0), 1.0);
            s.store_min_with_scalar_ad(538, A::max_with_scalar(s.ad_value(550), 1.0), 16.0);
            s.store_div_scaled_product(553, A::powf(s.ad_value(575), p.p306), A::scale_offset(s.ad_value(576), p.p309, 1.0), p.p305, A::scale_offset(A::powf(s.ad_value(575), p.p308), p.p307, 1.0), 1.0);
            s.store_max_with_scalar(63, 553, 0.0);
            s.store_div_scaled_product(554, A::powf(s.ad_value(575), p.p311), A::scale_offset(s.ad_value(576), p.p314, 1.0), p.p310, A::scale_offset(A::powf(s.ad_value(575), p.p313), p.p312, 1.0), 1.0);
            s.store_max_with_scalar(64, 554, 0.0);
            s.store_scalar(65, p.p315);
            s.store_scalar(66, p.p316);
            s.store_scalar(67, p.p317);
            s.store_scalar(75, p.p318);
            s.store_div_from_scalar(197, p.p319, 577);
            s.store_div_from_scalar(198, p.p320, 576);
            s.store_div_from_scalar(199, p.p321, 576);
            s.store_div_from_scalar(202, p.p335, 576);
            s.store_div_from_scalar(203, p.p336, 576);
            s.store_div_from_scalar(200, p.p322, 576);
            s.store_div_from_scalar(201, p.p323, 576);
            s.store_scalar(76, p.p324);
            s.store_scalar(77, p.p338);
            s.store_scalar(78, p.p325);
            s.store_scalar(79, p.p326);
            s.store_scalar(80, p.p327);
            s.store_scalar(81, p.p337);
            s.store_scalar(82, p.p328);
            s.store_scalar(83, p.p329);
            s.store_scalar(84, p.p330);
            s.store_scale(85, 575, p.p331);
            s.store_scalar(86, p.p332);
            s.store_scalar(87, p.p333);
            s.store_scalar(88, p.p334);
            s.store_offset_div_from_scalar_ad(555, p.p341, s.ad_value(576), p.p339);
            s.store_max_with_scalar(89, 555, 0.0);
            s.store_offset_div_from_scalar_ad(556, p.p342, s.ad_value(576), p.p340);
            s.store_max_with_scalar(90, 556, 0.0);
            s.store_scalar(204, p.p343);
            s.store_scalar(205, p.p344);
            s.store_scalar(93, p.p345);
            s.store_scalar(94, p.p346);
            s.store_scalar(95, p.p347);
            s.store_scalar(96, p.p348);
            s.store_offset_scaled(97, 575, p.p351, p.p349);
            s.store_offset_scaled(98, 575, p.p352, p.p350);
            s.store_scaled_mul_scale_offset_inputs(557, 575, p.p385, 1.0, 576, p.p386, 1.0, p.p384);
            s.store_max_with_scalar(112, 557, 0.0);
            s.store_scalar(206, p.p387);
            s.store_scalar(114, p.p388);
            s.store_scaled_mul_scale_offset_inputs(558, 575, p.p390, 1.0, 576, p.p391, 1.0, p.p389);
            s.store_max_with_scalar(115, 558, 0.0);
            s.store_offset_scaled(585, 572, p.p354, (2.0 * p.p353));
            s.store_scalar(99, p.p355);
            s.store_scale_ad(0, A::powf(s.ad_value(575), p.p358), p.p357);
            s.store_add_scaled_inputs3_offset_indices(207, 0, 1.0, 576, p.p359, 577, p.p360, p.p356);
            s.store_scalar(208, p.p361);
            s.store_mul3_ad_scaled_output(102, A::scale_offset(s.ad_value(575), p.p363, 1.0), A::scale_offset(s.ad_value(576), p.p364, 1.0), A::scale_offset(s.ad_value(577), p.p365, 1.0), p.p362);
            s.store_scalar(103, p.p366);
            s.store_scalar(104, p.p367);
            s.store_scaled_mul_scale_offset_rhs_ad(0, A::powf(s.ad_value(580), p.p369), 576, p.p370, 1.0, (p.p368 * 2.0));
            s.store_min_with_scalar_ad(105, A::max_with_scalar(s.ad_value(0), 0.0), 5.0);
            s.store_div_scaled_product_indices(106, 105, 530, p.p371, 529, 1.0);
            s.store_mul_ad(0, A::powf(s.ad_value(580), p.p373), A::scale_offset(s.ad_value(576), p.p374, 1.0));
            s.store_scale(0, 0, p.p372);
            s.store_max_with_scalar(107, 0, 0.0);
            s.store_div_scaled_product_indices(108, 107, 530, p.p375, 529, 1.0);
            s.store_scalar(109, p.p376);
            s.store_offset_ad(0, A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p377 * p.p378), s.ad_value(571)), 1.0, A::exp_scaled_input(s.ad_value(571), (-1.0 / (p.p378)))), 1.0);
            s.store_max_with_scalar(0, 0, 1e-15);
            s.store_mul_ad(209, A::div_scaled_inputs(s.ad_value(585), p.p240, A::mul(s.ad_value(0), s.ad_value(571)), 1.0), A::scale_offset(s.ad_value(576), p.p379, 1.0));
            s.store_add_scaled_inputs_product_first_ad(111, A::scale_offset(s.ad_value(575), p.p381, p.p380), 1.0, 576, p.p382, 575, 576, p.p383);
            s.store_mul(116, 574, 573);
            s.store_offset_scaled(559, 578, p.p393, p.p392);
            s.store_max_with_scalar(117, 559, 0.0);
            s.store_scalar(118, (p.p394 * 1000000.0));
            s.store_div_scaled_inputs_indices(119, 574, p.p395, 566, 1.0);
            s.store_scalar(120, p.p396);
            s.copy_ad(181, 179);
        }

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (!s.b[607]) {
            s.copy_ad(182, 180);
            s.copy_ad(135, 27);
            s.copy_ad(136, 28);
            s.copy_ad(543, 542);
            s.copy_ad(185, 183);
            s.copy_ad(186, 184);
            s.copy_ad(549, 548);
            s.copy_ad(196, 195);
            s.copy_ad(539, 538);
            s.copy_ad(158, 63);
        }

        s.b[622] = (p.p11 > 0.0);
        s.v[622] = if s.b[622] { 1.0 } else { 0.0 };

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(121, p.p207);
        }

        s.b[623] = param_given[397];
        s.v[623] = if s.b[623] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[623]) {
            s.store_scalar(121, p.p397);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(122, p.p208);
        }

        s.b[624] = param_given[398];
        s.v[624] = if s.b[624] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[624]) {
            s.store_scalar(122, p.p398);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(123, p.p209);
        }

        s.b[625] = param_given[399];
        s.v[625] = if s.b[625] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[625]) {
            s.store_scalar(123, p.p399);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(124, p.p212);
        }

        s.b[626] = param_given[402];
        s.v[626] = if s.b[626] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[626]) {
            s.store_scalar(124, p.p402);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(125, p.p213);
        }

        s.b[627] = param_given[403];
        s.v[627] = if s.b[627] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[627]) {
            s.store_scalar(125, p.p403);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(126, p.p210);
        }

        s.b[628] = param_given[400];
        s.v[628] = if s.b[628] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[628]) {
            s.store_scalar(126, p.p400);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(127, p.p211);
        }

        s.b[629] = param_given[401];
        s.v[629] = if s.b[629] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[629]) {
            s.store_scalar(127, p.p401);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_div_scaled_product_offset_denominator(0, s.ad_value(122), A::pow(s.ad_value(575), s.ad_value(123)), 1.0, A::mul(s.ad_value(126), A::pow(s.ad_value(575), s.ad_value(127))), 1.0, 1.0);
            s.store_add_scaled_inputs_products_indices(181, 121, 1.0, 0, 1.0, 124, 576, 1.0, 125, 577, 1.0);
            s.store_scalar(128, p.p214);
        }

        s.b[630] = param_given[404];
        s.v[630] = if s.b[630] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[630]) {
            s.store_scalar(128, p.p404);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(129, p.p215);
        }

        s.b[631] = param_given[405];
        s.v[631] = if s.b[631] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[631]) {
            s.store_scalar(129, p.p405);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_add_scaled_product_left_ad(182, 128, 1.0, A::div_scaled_product(s.ad_value(129), s.ad_value(530), 1.0, s.ad_value(529), 1.0), 0, 1.0);
            s.store_scalar(132, p.p224);
        }

        s.b[632] = param_given[406];
        s.v[632] = if s.b[632] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[632]) {
            s.store_scalar(132, p.p406);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(133, p.p225);
        }

        s.b[633] = param_given[407];
        s.v[633] = if s.b[633] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[633]) {
            s.store_scalar(133, p.p407);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(134, p.p226);
        }

        s.b[634] = param_given[408];
        s.v[634] = if s.b[634] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[634]) {
            s.store_scalar(134, p.p408);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_mul_ad_affine_product_rhs(541, 132, A::pow(s.ad_value(580), s.ad_value(133)), A::offset(A::mul(s.ad_value(134), s.ad_value(576)), 1.0), 2.0, 0.0);
            s.store_min_with_scalar_ad(135, A::max_with_scalar(s.ad_value(541), 0.0), 5.0);
            s.store_div_scaled_product_indices(136, 135, 530, p.p227, 529, 1.0);
            s.store_scalar(137, p.p231);
        }

        s.b[635] = param_given[409];
        s.v[635] = if s.b[635] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[635]) {
            s.store_scalar(137, p.p409);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(138, p.p232);
        }

        s.b[636] = param_given[410];
        s.v[636] = if s.b[636] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[636]) {
            s.store_scalar(138, p.p410);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(139, p.p233);
        }

        s.b[637] = param_given[411];
        s.v[637] = if s.b[637] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[637]) {
            s.store_scalar(139, p.p411);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_mul_offset_rhs_ad(0, A::pow(s.ad_value(580), s.ad_value(138)), A::mul(s.ad_value(139), s.ad_value(576)), 1.0);
            s.store_mul(543, 137, 0);
            s.store_max_with_scalar(185, 543, 0.0);
            s.store_div_scaled_product_indices(186, 185, 530, p.p234, 529, 1.0);
            s.store_scalar(142, p.p289);
        }

        s.b[638] = param_given[412];
        s.v[638] = if s.b[638] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[638]) {
            s.store_scalar(142, p.p412);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(143, p.p290);
        }

        s.b[639] = param_given[413];
        s.v[639] = if s.b[639] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[639]) {
            s.store_scalar(143, p.p413);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(144, p.p291);
        }

        s.b[640] = param_given[414];
        s.v[640] = if s.b[640] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[640]) {
            s.store_scalar(144, p.p414);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(145, p.p292);
        }

        s.b[641] = param_given[415];
        s.v[641] = if s.b[641] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[641]) {
            s.store_scalar(145, p.p415);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(146, p.p293);
        }

        s.b[642] = param_given[416];
        s.v[642] = if s.b[642] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[642]) {
            s.store_scalar(146, p.p416);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_mul_offset_rhs_ad(549, A::mul3(s.ad_value(583), A::add_scaled_product(s.ad_value(142), 1.0, s.ad_value(143), A::pow(s.ad_value(575), s.ad_value(144)), 1.0), A::offset(A::mul(s.ad_value(145), s.ad_value(576)), 1.0)), A::mul(s.ad_value(146), s.ad_value(577)), 1.0);
            s.store_max_with_scalar(196, 549, 0.0);
            s.store_scalar(148, p.p300);
        }

        s.b[643] = param_given[417];
        s.v[643] = if s.b[643] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[643]) {
            s.store_scalar(148, p.p417);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(149, p.p301);
        }

        s.b[644] = param_given[418];
        s.v[644] = if s.b[644] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[644]) {
            s.store_scalar(149, p.p418);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(150, p.p302);
        }

        s.b[645] = param_given[419];
        s.v[645] = if s.b[645] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[645]) {
            s.store_scalar(150, p.p419);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(151, p.p303);
        }

        s.b[646] = param_given[420];
        s.v[646] = if s.b[646] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[646]) {
            s.store_scalar(151, p.p420);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(152, p.p304);
        }

        s.b[647] = param_given[421];
        s.v[647] = if s.b[647] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[647]) {
            s.store_scalar(152, p.p421);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_div_scaled_value_offset_denominator(551, s.ad_value(148), 1.0, A::div_scaled_product_offset_denominator(s.ad_value(149), A::pow(s.ad_value(575), s.ad_value(150)), 1.0, A::mul(s.ad_value(151), A::pow(s.ad_value(575), s.ad_value(152))), 1.0, 1.0), 1.0, 1.0);
            s.store_min_with_scalar_ad(539, A::max_with_scalar(s.ad_value(551), 1.0), 16.0);
            s.store_scalar(153, p.p305);
        }

        s.b[648] = param_given[422];
        s.v[648] = if s.b[648] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[648]) {
            s.store_scalar(153, p.p422);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(154, p.p306);
        }

        s.b[649] = param_given[423];
        s.v[649] = if s.b[649] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[649]) {
            s.store_scalar(154, p.p423);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(155, p.p307);
        }

        s.b[650] = param_given[424];
        s.v[650] = if s.b[650] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[650]) {
            s.store_scalar(155, p.p424);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(156, p.p308);
        }

        s.b[651] = param_given[425];
        s.v[651] = if s.b[651] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[651]) {
            s.store_scalar(156, p.p425);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(157, p.p309);
        }

        s.b[652] = param_given[426];
        s.v[652] = if s.b[652] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[622]) && s.b[652]) {
            s.store_scalar(157, p.p426);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_div_scaled_product3_mixed_iaaa(552, 153, A::pow(s.ad_value(575), s.ad_value(154)), A::offset(A::mul(s.ad_value(157), s.ad_value(576)), 1.0), 1.0, A::offset(A::mul(s.ad_value(155), A::pow(s.ad_value(575), s.ad_value(156))), 1.0), 1.0);
            s.store_max_with_scalar(158, 552, 0.0);
        }

        if (!s.b[607]) {
            s.store_mul_div_from_scalar_lhs(0, 3.45313e-11, 529, 574);
            s.store_scale(159, 0, p.p427);
            s.store_scale(160, 0, p.p428);
            s.store_div_from_scalar_ad(161, p.p429, A::max_with_scalar(A::offset(A::div_scaled_inputs(s.ad_value(566), p.p430, s.ad_value(574), 1.0), 1.0), 0.001));
            s.store_scalar(162, p.p431);
            s.store_scalar(163, p.p432);
            s.store_offset_scaled(560, 579, p.p435, p.p433);
            s.store_max_with_scalar(164, 560, 0.0);
            s.store_offset_scaled(561, 579, p.p436, p.p434);
            s.store_max_with_scalar(165, 561, 0.0);
            s.store_div_scaled_product3_indices(166, 225, 14, 572, p.p437, 571, 1.0);
            s.store_scalar(167, p.p438);
            s.store_max_with_scalar_ad(0, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(578), p.p440, 1.0), 1.0, s.ad_value(579), p.p441, s.ad_value(578), s.ad_value(579), p.p442), 1e-10);
            s.store_scalar(2, 0.0);
        }

        s.b[653] = ((p.p29 > 1.0) && (p.p28 > 0.0));
        s.v[653] = if s.b[653] { 1.0 } else { 0.0 };

        if ((!s.b[607]) && s.b[653]) {
            s.store_scalar(3, ((-(p.p28 + p.p20)) / p.p445));
        }

        s.b[654] = (((s.v[3]) as f64).abs() < 80.0);
        s.v[654] = if s.b[654] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[653]) && s.b[654]) {
            s.store_exp(4, 3);
        }

        s.b[655] = (s.v[3] < (-80.0));
        s.v[655] = if s.b[655] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((!s.b[607]) && s.b[653]) && (!s.b[654])) && s.b[655]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(4, 1.80485e-35, A::neg(s.ad_value(3)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(3)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(3)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((((!s.b[607]) && s.b[653]) && (!s.b[654])) && (!s.b[655])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(4, 3, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(3), (-80.0)), 0.5, A::scale_offset(s.ad_value(3), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if ((!s.b[607]) && s.b[653]) {
            s.store_sub_from_scalar(5, 1.0, 4);
            s.store_div_scaled_product_mixed_iaa(2, 4, A::sub(s.ad_value(5), A::scale_offset(A::powf(s.ad_value(4), p.p29), (-1.0 / (p.p29)), 1.0 / (p.p29))), (2.0 * p.p446), A::square(s.ad_value(5)), 1.0);
        }

        if (!s.b[607]) {
            s.store_div_scaled_value_offset_denominator(0, s.ad_value(0), 1.0, s.ad_value(2), 1.0, 1.0);
            s.store_div_from_scalar(562, p.p439, 0);
            s.store_max_with_scalar(210, 562, 1e-6);
            s.store_scalar(169, p.p443);
            s.store_scale(563, 0, p.p444);
            s.store_max_with_scalar(170, 563, 0.0);
        }

        let (assign5390_e4654,) = {
    if (!s.b[607]) {
        (p.p447,)
    } else {
        (s.v[171],)
    }
};
        s.v[171] = assign5390_e4654;

        let (assign5400_e4673,) = {
    if (!s.b[607]) {
        let assign5400_e4659: f64 = (p.p448 * s.v[544]);
        let assign5400_e4661: f64 = (assign5400_e4659 * s.v[544]);
        let assign5400_e4663: f64 = (assign5400_e4661 * s.v[576]);
        let assign5400_e4665: f64 = (assign5400_e4663 * s.v[576]);
        let assign5400_e4669: f64 = (p.p449 - 2.0);
        let assign5400_e4670: f64 = (s.v[575]).powf(assign5400_e4669);
        let assign5400_e4671: f64 = (assign5400_e4665 * assign5400_e4670);
        (assign5400_e4671,)
    } else {
        (s.v[172],)
    }
};
        s.v[172] = assign5400_e4673;

        if (!s.b[607]) {
            s.store_add_scaled_inputs(564, 577, p.p450, 576, p.p451);
            s.store_max_with_scalar(173, 564, 0.0);
            s.store_scale(174, 577, p.p452);
            s.store_scale(175, 577, p.p453);
            s.store_scalar(176, p.p454);
            s.store_scalar(177, p.p455);
        }

        s.b[657] = ((((p.p457 > 0.0) && (p.p26 > 0.0)) && (p.p27 > 0.0)) && ((p.p29 == 1.0) || ((p.p29 > 1.0) && (p.p28 > 0.0))));
        s.v[657] = if s.b[657] { 1.0 } else { 0.0 };

        s.b[658] = (p.p457 == 1.0);
        s.v[658] = if s.b[658] { 1.0 } else { 0.0 };

        if (((!s.b[607]) && s.b[657]) && s.b[658]) {
            s.store_scalar(588, 0.0);
            s.store_scalar(589, 0.0);
            s.store_scalar(590, 0.0);
        }

        let mut assign5620_loop_guard: usize = 0;
        while {
            let assign5620_cond_e4868: f64 = (p.p29 - 0.5);
            let assign5620_cond_e4870: f64 = if ((((!s.b[607]) && s.b[657]) && s.b[658]) && (s.v[590] < assign5620_cond_e4868)) { 1.0 } else { 0.0 };
            assign5620_cond_e4870 != 0.0
        } {
            assign5620_loop_guard += 1;
            assert!(assign5620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[607]) && s.b[657]) && s.b[658]) {
                s.store_add_ad_rhs(588, 588, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(590), (p.p28 + p.p20), (p.p26 + (0.5 * p.p20)))));
                s.store_add_ad_rhs(589, 589, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(590), (p.p28 + p.p20), (p.p27 + (0.5 * p.p20)))));
                s.store_offset(590, 590, 1.0);
            }
        }

        if (((!s.b[607]) && s.b[657]) && s.b[658]) {
            s.store_scale(591, 588, 1.0 / (p.p29));
            s.store_scale(592, 589, 1.0 / (p.p29));
            s.store_scalar(593, (1.0 / (p.p458 + (0.5 * p.p20))));
            s.store_scalar(594, (1.0 / (p.p459 + (0.5 * p.p20))));
            s.store_max_with_scalar_ad(595, A::offset(s.ad_value(569), p.p20), 1e-9);
            s.store_max_with_scalar_ad(596, A::offset(A::add(s.ad_value(528), s.ad_value(570)), p.p460), 1e-9);
            s.store_div_from_scalar_powf_ad(597, 1.0, s.ad_value(595), p.p467);
            s.store_div_from_scalar_powf_ad(598, 1.0, s.ad_value(596), p.p468);
            s.store_mul_ad(599, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(597), p.p464, 1.0), 1.0, s.ad_value(598), p.p465, s.ad_value(597), s.ad_value(598), p.p466), A::scale_offset(s.ad_value(217), p.p463, (((((-1.0)) * (p.p463))) + (1.0))));
            s.store_div_scaled_inputs2_indices(600, 591, p.p461, 592, p.p461, 599, 1.0);
            s.store_div_scaled_inputs2_indices(601, 593, p.p461, 594, p.p461, 599, 1.0);
            s.store_div_from_scalar_powf_ad(597, 1.0, s.ad_value(595), p.p473);
            s.store_div_from_scalar_powf_ad(598, 1.0, s.ad_value(596), p.p474);
            s.store_max_with_scalar_ad(602, A::add_scaled_inputs_product(A::scale_offset(s.ad_value(597), p.p470, 1.0), 1.0, s.ad_value(598), p.p471, s.ad_value(597), s.ad_value(598), p.p472), 1e-20);
            s.store_add_scaled_inputs4_indices(603, 591, 1.0, 592, 1.0, 593, -1.0, 594, -1.0);
            s.store_div_scaled_product_offset_denominator(544, s.ad_value(544), A::offset(s.ad_value(600), 1.0), 1.0, s.ad_value(601), 1.0, 1.0);
            s.store_max_with_scalar(187, 544, 1e-10);
            s.store_scale(188, 187, p.p250);
            s.store_div_scaled_product_by_product(0, A::offset(s.ad_value(600), 1.0), A::scale_offset(s.ad_value(601), p.p462, 1.0), 1.0, A::offset(s.ad_value(601), 1.0), A::scale_offset(s.ad_value(600), p.p462, 1.0), 1.0);
            s.store_mul(548, 548, 0);
            s.store_max_with_scalar(195, 548, 0.0);
            s.store_mul(549, 549, 0);
            s.store_max_with_scalar(196, 549, 0.0);
            s.store_div_scaled_inputs_indices(0, 603, p.p469, 602, 1.0);
            s.store_add(179, 179, 0);
            s.store_add(180, 180, 0);
            s.store_add(181, 181, 0);
            s.store_add(182, 182, 0);
            s.store_div_scaled_inputs_mixed_ia(0, 603, p.p475, A::powf(s.ad_value(602), p.p476), 1.0);
            s.store_add(542, 542, 0);
            s.store_max_with_scalar(183, 542, 0.0);
            s.store_add(543, 543, 0);
            s.store_max_with_scalar(185, 543, 0.0);
            s.store_div_scaled_inputs_indices(0, 530, p.p234, 529, 1.0);
            s.store_mul(184, 183, 0);
            s.store_mul(186, 185, 0);
        }

        if (((!s.b[607]) && s.b[657]) && (!s.b[658])) {
            s.store_scalar(588, 0.0);
            s.store_scalar(590, 0.0);
            s.store_scalar(0, ((-1.0) / p.p478));
        }

        let mut assign6020_loop_guard: usize = 0;
        while {
            let assign6020_cond_e5465: f64 = (p.p29 - 0.5);
            let assign6020_cond_e5467: f64 = if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && (s.v[590] < assign6020_cond_e5465)) { 1.0 } else { 0.0 };
            assign6020_cond_e5467 != 0.0
        } {
            assign6020_loop_guard += 1;
            assert!(assign6020_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            s.b[659] = (((-((p.p26 + (0.5 * p.p20)) + (s.v[590] * (p.p28 + p.p20)))) / p.p477) > (-80.0));
            s.v[659] = if s.b[659] { 1.0 } else { 0.0 };
            if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && s.b[659]) {
                s.store_exp_scaled_input_ad(2, A::scale_offset(s.ad_value(590), (p.p28 + p.p20), (p.p26 + (0.5 * p.p20))), (-1.0 / (p.p477)));
            }
            if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && (!s.b[659])) {
                let assign6020_body2_ad_e5583: A = A::mul_offset_lhs(A::neg(A::scale(A::scale_offset(s.ad_value(590), (p.p28 + p.p20), (p.p26 + (0.5 * p.p20))), (-1.0 / (p.p477)))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::scale(A::scale_offset(s.ad_value(590), (p.p28 + p.p20), (p.p26 + (0.5 * p.p20))), (-1.0 / (p.p477)))), (-80.0)), 0.5, A::scale_offset(A::neg(A::scale(A::scale_offset(s.ad_value(590), (p.p28 + p.p20), (p.p26 + (0.5 * p.p20))), (-1.0 / (p.p477)))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
                s.store_div_from_scalar_offset_ad(2, 1.80485e-35, assign6020_body2_ad_e5583, 1.0);
            }
            s.b[660] = (((-((p.p27 + (0.5 * p.p20)) + (((p.p29 - 1.0) - s.v[590]) * (p.p28 + p.p20)))) / p.p477) > (-80.0));
            s.v[660] = if s.b[660] { 1.0 } else { 0.0 };
            if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && s.b[660]) {
                s.store_exp_scaled_input_ad(3, A::scale_offset(s.ad_value(590), (-(p.p28 + p.p20)), (((((p.p29 - 1.0)) * ((p.p28 + p.p20)))) + ((p.p27 + (0.5 * p.p20))))), (-1.0 / (p.p477)));
            }
            if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && (!s.b[660])) {
                let assign6020_body5_ad_e5721: A = A::mul_scaled_lhs(A::offset(A::neg(A::scale(A::scale_offset(s.ad_value(590), (-(p.p28 + p.p20)), (((((p.p29 - 1.0)) * ((p.p28 + p.p20)))) + ((p.p27 + (0.5 * p.p20))))), (-1.0 / (p.p477)))), (-80.0)), 0.5, A::scale_offset(A::neg(A::scale(A::scale_offset(s.ad_value(590), (-(p.p28 + p.p20)), (((((p.p29 - 1.0)) * ((p.p28 + p.p20)))) + ((p.p27 + (0.5 * p.p20))))), (-1.0 / (p.p477)))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0))));
                s.store_div_from_scalar_offset_mul_offset_lhs_ad(3, 1.80485e-35, A::neg(A::scale(A::scale_offset(s.ad_value(590), (-(p.p28 + p.p20)), (((((p.p29 - 1.0)) * ((p.p28 + p.p20)))) + ((p.p27 + (0.5 * p.p20))))), (-1.0 / (p.p477)))), (-80.0), A::offset(assign6020_body5_ad_e5721, 1.0), 1.0);
            }
            if (((!s.b[607]) && s.b[657]) && (!s.b[658])) {
                s.store_powf_ad(4, A::sub_from_scalar(1.0, s.ad_value(2)), (-p.p478));
                s.store_powf_ad(5, A::sub_from_scalar(1.0, s.ad_value(3)), (-p.p478));
                s.store_add_ad_rhs(588, 588, A::pow(A::add_scaled_inputs(s.ad_value(4), 0.5, s.ad_value(5), 0.5), s.ad_value(0)));
                s.store_offset(590, 590, 1.0);
            }
        }

        if (((!s.b[607]) && s.b[657]) && (!s.b[658])) {
            s.store_sub_from_scalar_scaled_input(604, 1.0, 588, 1.0 / (p.p29));
        }

        s.b[661] = (((-(p.p458 + (0.5 * p.p20))) / p.p477) > (-80.0));
        s.v[661] = if s.b[661] { 1.0 } else { 0.0 };

        if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && s.b[661]) {
            s.store_scalar(2, ((((-(p.p458 + (0.5 * p.p20))) / p.p477)) as f64).exp());
        }

        if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && (!s.b[661])) {
            s.store_scalar(2, (1.80485e-35 / (1.0 + (((-((-(p.p458 + (0.5 * p.p20))) / p.p477)) - 80.0) * (1.0 + ((0.5 * ((-((-(p.p458 + (0.5 * p.p20))) / p.p477)) - 80.0)) * (1.0 + (((-((-(p.p458 + (0.5 * p.p20))) / p.p477)) - 80.0) * 0.3333333333333))))))));
        }

        s.b[662] = (((-(p.p459 + (0.5 * p.p20))) / p.p477) > (-80.0));
        s.v[662] = if s.b[662] { 1.0 } else { 0.0 };

        if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && s.b[662]) {
            s.store_scalar(3, ((((-(p.p459 + (0.5 * p.p20))) / p.p477)) as f64).exp());
        }

        if ((((!s.b[607]) && s.b[657]) && (!s.b[658])) && (!s.b[662])) {
            s.store_scalar(3, (1.80485e-35 / (1.0 + (((-((-(p.p459 + (0.5 * p.p20))) / p.p477)) - 80.0) * (1.0 + ((0.5 * ((-((-(p.p459 + (0.5 * p.p20))) / p.p477)) - 80.0)) * (1.0 + (((-((-(p.p459 + (0.5 * p.p20))) / p.p477)) - 80.0) * 0.3333333333333))))))));
        }

        if (((!s.b[607]) && s.b[657]) && (!s.b[658])) {
            s.store_powf_ad(4, A::sub_from_scalar(1.0, s.ad_value(2)), (-p.p478));
            s.store_powf_ad(5, A::sub_from_scalar(1.0, s.ad_value(3)), (-p.p478));
            s.store_sub_from_scalar_ad(605, 1.0, A::pow(A::add_scaled_inputs(s.ad_value(4), 0.5, s.ad_value(5), 0.5), s.ad_value(0)));
            s.store_max_with_scalar_ad(596, A::offset(A::add(s.ad_value(528), s.ad_value(570)), p.p460), 1e-9);
            s.store_div_from_scalar_offset_scaled_input(606, p.p482, 217, p.p483, (((((-1.0)) * (p.p483))) + (1.0)));
            s.store_mul(600, 606, 604);
            s.store_mul(601, 606, 605);
            s.store_sub(603, 604, 605);
            s.store_max_with_scalar_ad(602, A::offset(A::div_scaled_inputs(s.ad_value(596), p.p480, s.ad_value(566), 1.0), 1.0), 1e-20);
            s.store_div_scaled_product_offset_denominator(544, s.ad_value(544), A::offset(s.ad_value(600), 1.0), 1.0, s.ad_value(601), 1.0, 1.0);
            s.store_max_with_scalar(187, 544, 1e-10);
            s.store_scale(188, 187, p.p250);
            s.store_div_scaled_product_by_product(0, A::offset(s.ad_value(600), 1.0), A::scale_offset(s.ad_value(601), p.p484, 1.0), 1.0, A::offset(s.ad_value(601), 1.0), A::scale_offset(s.ad_value(600), p.p484, 1.0), 1.0);
            s.store_mul(548, 548, 0);
            s.store_max_with_scalar(195, 548, 0.0);
            s.store_mul(549, 549, 0);
            s.store_max_with_scalar(196, 549, 0.0);
            s.store_div_scaled_inputs_indices(0, 603, p.p479, 602, 1.0);
            s.store_add(179, 179, 0);
            s.store_add(180, 180, 0);
            s.store_add(181, 181, 0);
            s.store_add(182, 182, 0);
            s.store_mul_ad_affine_product_rhs(0, 603, A::powf(s.ad_value(580), p.p232), A::scale_offset(s.ad_value(576), p.p233, 1.0), p.p481, 0.0);
            s.store_add(542, 542, 0);
            s.store_max_with_scalar(183, 542, 0.0);
            s.store_add(543, 543, 0);
            s.store_max_with_scalar(185, 543, 0.0);
            s.store_div_scaled_inputs_indices(0, 530, p.p234, 529, 1.0);
            s.store_mul(184, 183, 0);
            s.store_mul(186, 185, 0);
        }

        s.b[663] = (p.p7 == 0.0);
        s.v[663] = if s.b[663] { 1.0 } else { 0.0 };

        if s.b[663] {
            s.copy_ad(20, 19);
            s.copy_ad(199, 198);
            s.copy_ad(203, 202);
            s.copy_ad(201, 200);
            s.copy_ad(90, 89);
            s.copy_ad(205, 204);
            s.copy_ad(94, 93);
            s.copy_ad(96, 95);
            s.copy_ad(98, 97);
            s.copy_ad(160, 159);
        }

    }

    pub(super) fn stamp_transient_block_5(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[663] {
            s.copy_ad(165, 164);
        }

        s.store_sub_from_scalar(224, 1.0, 15);

        s.store_add_scaled_inputs(225, 224, 1.04479e-10, 15, 1.43438e-10);

        s.store_sub_from_scalar_ad(226, 1.17, A::div_scaled_value_offset_denominator(s.ad_value(214), 0.000473, s.ad_value(213), 636.0, 1.0));

        s.store_sub_from_scalar_ad(227, 0.744, A::div_scaled_value_offset_denominator(s.ad_value(214), 0.0004774, s.ad_value(213), 235.0, 1.0));

        s.store_mul_add_scaled_inputs3_offset_rhs(228, 15, s.ad_value(227), 1.0, s.ad_value(226), (-1.0), s.ad_value(224), (-0.4), 0.0);

        s.store_add(229, 226, 228);

        s.store_scaled_mul(230, 229, 220, 0.5);

        s.copy_ad(231, 230);

        s.store_div_from_scalar_offset_ad(234, 1.0, A::sqrt_scaled_input(s.ad_value(15), 10.0), 1.0);

        s.store_sub_scaled_inputs(233, 15, 0.05, 228, 0.5);

        s.store_scaled_mul(0, 532, 14, ((1.602176565e-19 * 0.5) * 28959234086.17689));

        s.b[664] = (s.v[531] > 0.0);
        s.v[664] = if s.b[664] { 1.0 } else { 0.0 };

        if s.b[664] {
            s.store_mul_offset_rhs(239, 0, 529, (p.p13 * 4e-10));
            s.store_mul_offset_rhs(240, 0, 530, (p.p13 * 4e-10));
        }

        if (!s.b[664]) {
            s.store_mul_scaled_offset_rhs(239, 0, -1.0, 529, (p.p13 * 4e-10));
            s.store_mul_scaled_offset_rhs(240, 0, -1.0, 530, (p.p13 * 4e-10));
        }

        s.store_sqrt_scaled_input(0, 213, 0.0033333333333);

        s.store_mul3_affine_lhs(2, 0, 0, 4.05e25, 0.0, 0);

        s.store_mul(248, 2, 234);

        s.store_mul_exp_ad_rhs(247, 2, A::mul_scaled_lhs(s.ad_value(228), 0.5, s.ad_value(220)));

        s.store_mul_exp_ad_rhs(586, 2, A::mul_scaled_lhs(s.ad_value(228), 0.5, s.ad_value(220)));

        s.store_div_from_scalar(235, 3.45313e-11, 529);

        s.store_div_from_scalar(236, 3.45313e-11, 530);

        s.b[665] = (s.v[534] > 0.0);
        s.v[665] = if s.b[665] { 1.0 } else { 0.0 };

        if s.b[665] {
            s.store_mul_offset_rhs(237, 235, 534, 1.0);
            s.copy_ad(238, 236);
        }

        if (!s.b[665]) {
            s.copy_ad(237, 235);
            s.store_mul_sub_from_scalar_rhs(238, 236, 1.0, 534);
        }

        s.store_div(241, 225, 14);

        s.store_mul_offset_ad_rhs(222, 219, A::mul(s.ad_value(17), s.ad_value(218)), 1.0);

        s.store_div_from_scalar(223, 1.0, 222);

        s.store_scaled_mul(232, 229, 223, 0.5);

        s.store_div(242, 237, 241);

        s.store_div(243, 238, 241);

        s.store_div_from_scalar_add_ad(244, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(242)), 1.0), A::div_from_scalar(1.0, s.ad_value(243)));

        s.store_mul3_affine_lhs(249, 248, 225, (2.0 * 1.602176565e-19), 0.0, 223);

        s.store_offset_ln_ad(250, A::div_scaled_product(s.ad_value(241), s.ad_value(241), 1.0, s.ad_value(249), 1.0), (-0.6931471805599));

        s.store_mul_div_scaled_product_rhs(251, 223, s.ad_value(29), s.ad_value(14), (0.5 * 1.602176565e-19), A::add(s.ad_value(237), s.ad_value(238)), 1.0);

        s.store_mul(0, 34, 216);

        s.store_add(31, 183, 0);

        s.store_add(32, 184, 0);

        s.store_add(140, 185, 0);

        s.store_add(141, 186, 0);

        s.store_mul(325, 35, 223);

        s.store_div_ad_lhs(256, A::sqrt(A::mul_scaled_lhs(s.ad_value(533), ((2.0 * 1.602176565e-19) * 1.04479e-10), s.ad_value(220))), 238);

        s.store_square(257, 256);

        s.store_div_from_scalar(258, 1.0, 257);

        s.store_offset_scaled(259, 256, 0.707106781186545, 1.0);

        s.store_div_from_scalar(260, 1.0, 259);

        let assign7010_e6747: f64 = (1e-5 * s.v[259]);
        s.v[261] = assign7010_e6747;

        s.store_add_ad_lhs(587, A::ln(A::div(s.ad_value(533), s.ad_value(586))), 230);

        s.store_scale(262, 587, 2.0);

        s.b[666] = (p.p2 > 0.0);
        s.v[666] = if s.b[666] { 1.0 } else { 0.0 };

        if s.b[666] {
            s.store_add_ad_rhs(180, 180, A::mul3(s.ad_value(16), s.ad_value(219), s.ad_value(587)));
            s.store_add_ad_rhs(182, 182, A::mul3(s.ad_value(16), s.ad_value(219), s.ad_value(587)));
        }

        s.v[245] = 0.0;

        s.b[667] = (p.p9 > 0.0);
        s.v[667] = if s.b[667] { 1.0 } else { 0.0 };

        if s.b[667] {
            s.store_mul_add_ad_rhs(245, 219, A::ln(A::div(s.ad_value(24), s.ad_value(247))), s.ad_value(230));
        }

        s.store_div_ad_lhs(246, A::sqrt(A::mul_scaled_lhs(s.ad_value(225), (2.0 * 1.602176565e-19), s.ad_value(24))), 235);

        s.v[253] = 15.0;

        s.b[668] = (p.p10 == 1.0);
        s.v[668] = if s.b[668] { 1.0 } else { 0.0 };

        if s.b[668] {
            s.store_scaled_add_ad(253, A::offset(A::div_from_scalar(2970.0, s.ad_value(8)), 15.0), A::sqrt(A::offset(A::mul_sub_from_scalar_lhs(15.0, A::div_from_scalar(2970.0, s.ad_value(8)), A::sub_from_scalar(15.0, A::div_from_scalar(2970.0, s.ad_value(8)))), 1e-6)), 0.5);
        }

        s.v[252] = 0.0;

        s.v[254] = 0.0;

        s.store_scaled_mul(255, 14, 14, 1e18);

        s.b[669] = (p.p13 > 0.0);
        s.v[669] = if s.b[669] { 1.0 } else { 0.0 };

        s.b[670] = (p.p14 == 1.0);
        s.v[670] = if s.b[670] { 1.0 } else { 0.0 };

        if (s.b[669] && s.b[670]) {
            s.store_div_from_scalar(252, 0.409618895, 255);
            s.store_scale_ad(254, A::exp_scaled_input(A::ln(A::mul(s.ad_value(222), s.ad_value(255))), (-0.3333333333333)), ((0.4 * p.p13) * 1.27520989));
        }

        if (s.b[669] && (!s.b[670])) {
            s.store_div_from_scalar(252, 0.723134895, 255);
            s.store_scale_ad(254, A::exp_scaled_input(A::ln(A::mul(s.ad_value(222), s.ad_value(255))), (-0.3333333333333)), ((0.4 * p.p13) * 1.5412087));
        }

        s.store_add_scaled_product_indices(0, 252, 1.0, 23, 216, p.p14);

        s.store_sub_offset_lhs(2, 0, p.p34, 245);

        s.store_add_scaled_inputs4_indices(21, 179, p.p14, 233, p.p14, 239, p.p14, 2, 1.0);

        s.store_add_scaled_inputs4_indices(22, 180, p.p14, 233, p.p14, 240, p.p14, 0, 1.0);

        s.store_add_scaled_inputs4_indices(130, 181, p.p14, 233, p.p14, 239, p.p14, 2, 1.0);

        s.store_add_scaled_inputs4_indices(131, 182, p.p14, 233, p.p14, 240, p.p14, 0, 1.0);

        s.store_ln(291, 218);

        s.store_scaled_exp_ad(292, A::mul(s.ad_value(40), s.ad_value(291)), p.p35);

        s.store_mul(38, 187, 292);

        s.store_mul(39, 188, 292);

        s.store_exp_mul(293, 48, 291);

        s.store_mul(46, 189, 293);

        s.store_exp_mul(294, 49, 291);

        s.store_mul(47, 190, 294);

        s.store_exp_mul(295, 43, 291);

        s.store_mul(33, 191, 295);

        s.store_exp_mul(296, 45, 291);

        s.store_mul(44, 192, 296);

        s.store_exp_mul(297, 52, 291);

        s.store_mul(50, 193, 297);

        s.store_div_scaled_inputs_indices(0, 222, 1e-8, 14, 1.0);

        s.store_mul(263, 0, 46);

        s.store_div_from_scalar_scaled_input(264, 1.0, 535, 0.5);

        s.store_div(265, 264, 536);

        s.b[671] = (p.p14 == 1.0);
        s.v[671] = if s.b[671] { 1.0 } else { 0.0 };

        if s.b[671] {
            s.store_scale(266, 537, 0.5);
        }

        if (!s.b[671]) {
            s.store_scale(266, 537, 0.3333333333333);
        }

        s.store_sub_from_scalar(267, 1.0, 266);

        s.store_exp_mul(298, 55, 291);

        s.store_mul(53, 194, 298);

        s.store_scaled_mul(268, 53, 222, 2.0);

        s.store_offset_ad(211, A::exp_scaled_input(A::ln(A::offset(A::exp_scaled_input(A::div_from_scalar(16.0, s.ad_value(538)), 0.6931471805599), (-1.0))), 0.375), (-1.0));

        s.store_offset_ad(212, A::exp_scaled_input(A::ln(A::offset(A::exp_scaled_input(A::div_from_scalar(16.0, s.ad_value(539)), 0.6931471805599), (-1.0))), 0.375), (-1.0));

        s.store_exp_mul(299, 60, 291);

        s.store_mul3_lhs(59, 195, 299, 292);

        s.store_mul(269, 59, 222);

        s.store_mul3_lhs(147, 196, 299, 292);

        s.store_mul(270, 147, 222);

        s.store_mul(271, 64, 223);

        s.store_exp_mul_scaled_lhs_indices(300, 76, -1.0, 291);

        s.store_mul(68, 197, 300);

        s.store_mul(69, 198, 300);

        s.store_mul(70, 199, 300);

        s.store_mul(71, 200, 300);

        s.store_mul(72, 201, 300);

        s.store_exp_mul_scaled_lhs_indices(300, 77, -1.0, 291);

        s.store_mul(73, 202, 300);

        s.store_mul(74, 203, 300);

        s.store_div_from_scalar(272, 1.0, 87);

        s.store_scaled_sqrt_scaled_input(273, 87, ((2.0 * 1.602176565e-19) * 9.10938291e-31), ((4.0 * 0.3333333333333) * 9.482522386533242e33));

        s.store_mul(274, 273, 18);

        s.store_mul(275, 273, 18);

        s.v[276] = 0.0;

        s.b[672] = (s.v[79] < 0.0);
        s.v[672] = if s.b[672] { 1.0 } else { 0.0 };

        if s.b[672] {
            s.store_div_scaled_inputs_indices(276, 78, (-0.495), 79, 1.0);
        }

        s.v[277] = 0.0;

        s.b[673] = (s.v[82] < 0.0);
        s.v[673] = if s.b[673] { 1.0 } else { 0.0 };

        if s.b[673] {
            s.store_div_scaled_inputs_indices(277, 80, (-0.495), 82, 1.0);
        }

        s.v[278] = 0.0;

        s.b[674] = (s.v[84] < 0.0);
        s.v[674] = if s.b[674] { 1.0 } else { 0.0 };

        if s.b[674] {
            s.store_div_scaled_inputs_indices(278, 83, (-0.495), 84, 1.0);
        }

        s.store_scale(279, 229, 0.5);

        s.store_mul(280, 75, 222);

        s.store_mul(281, 75, 219);

        s.store_div_from_scalar_offset_ad(282, 1.0, A::mul(s.ad_value(88), s.ad_value(232)), 1.0);

        s.store_div_from_scalar_square_ad(0, 4e-18, s.ad_value(18));

        s.store_mul(89, 89, 0);

        s.store_mul(90, 90, 0);

        s.store_scale(0, 18, 500000000.0);

        s.store_scaled_add_ad(273, A::offset(A::mul(s.ad_value(93), s.ad_value(216)), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(93), s.ad_value(216)), 1.0, A::offset(A::mul(s.ad_value(93), s.ad_value(216)), 1.0)), 0.01)), 0.5);

        s.store_mul3_lhs(91, 204, 273, 0);

        s.store_scaled_add_ad(273, A::offset(A::mul(s.ad_value(94), s.ad_value(216)), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(94), s.ad_value(216)), 1.0, A::offset(A::mul(s.ad_value(94), s.ad_value(216)), 1.0)), 0.01)), 0.5);

        s.store_mul3_lhs(92, 205, 273, 0);

        s.store_mul_exp_ad_rhs(113, 206, A::mul_scaled_lhs(s.ad_value(114), -1.0, s.ad_value(291)));

        s.store_mul_offset_ad_rhs(284, 219, A::mul(s.ad_value(99), s.ad_value(218)), 1.0);

        s.store_div_from_scalar(285, 1.0, 284);

        s.store_mul3_affine_lhs(286, 248, 225, (2.0 * 1.602176565e-19), 0.0, 285);

        s.store_add_scaled_product_indices(0, 252, 1.0, 102, 216, p.p14);

        s.store_sub_offset_ad_lhs(100, A::add_scaled_inputs4(s.ad_value(207), p.p14, s.ad_value(233), p.p14, s.ad_value(239), p.p14, s.ad_value(0), 1.0), p.p34, 245);

        s.store_add_scaled_inputs4_indices(101, 208, p.p14, 233, p.p14, 240, p.p14, 0, 1.0);

        s.store_scaled_exp_ad(0, A::mul(s.ad_value(111), s.ad_value(291)), p.p35);

        s.store_mul(110, 209, 0);

        s.store_mul(283, 116, 222);

        s.store_div_scaled_inputs_mixed_ia(287, 118, (0.25 * 1.602176565e-19), A::mul(s.ad_value(225), s.ad_value(222)), 1.0);

        s.store_ln_div(288, 118, 248);

        s.store_scaled_mul(289, 119, 222, 1.25e-6);

        s.store_sqrt_ad(290, A::mul3_scaled_output(s.ad_value(225), s.ad_value(14), A::offset(s.ad_value(529), 4e-10), 1.0 / (3.45313e-11)));

        s.store_exp_mul(301, 169, 291);

        s.store_mul(168, 210, 301);

        let assign8120_e7393: f64 = (4.0 * 1.3806488e-23);
        let assign8120_e7395: f64 = (assign8120_e7393 * s.v[213]);
        s.v[302] = assign8120_e7395;

        let assign8130_e7398: f64 = (s.v[171] * s.v[302]);
        s.v[303] = assign8130_e7398;

        s.v[304] = s.v[303];

        let assign8150_e7402: f64 = (9.10938291e-31 * 1000000000000.0);
        let assign8150_e7404: f64 = (assign8150_e7402 * s.v[172]);
        s.v[305] = assign8150_e7404;

        s.b[781] = (s.v[6] > 0.0);
        s.v[781] = if s.b[781] { 1.0 } else { 0.0 };

        if s.b[781] {
            s.store_voltage(215, ctx, nodes, Some(4), None);
            s.store_add(213, 8, 215);
            s.store_square(214, 213);
            s.store_offset(216, 213, (-s.v[7]));
            s.store_scale(217, 213, 1.0 / (s.v[7]));
            s.store_div_from_scalar(218, s.v[7], 213);
            s.store_scale(219, 213, 8.617332384961e-5);
            s.store_div_from_scalar(220, 1.0, 219);
        }

        s.b[782] = (p.p10 == 1.0);
        s.v[782] = if s.b[782] { 1.0 } else { 0.0 };

        if (s.b[781] && s.b[782]) {
            s.store_scaled_add_ad(221, A::offset(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), 600.0), A::sqrt(A::offset(A::mul_offset_lhs(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), (-600.0), A::offset(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), (-600.0))), 0.01)), 0.5);
        }

        if (s.b[781] && (!s.b[782])) {
            s.store_scalar(221, 600.0);
        }

        if s.b[781] {
            s.store_sub_from_scalar_ad(226, 1.17, A::div_scaled_value_offset_denominator(s.ad_value(214), 0.000473, s.ad_value(213), 636.0, 1.0));
            s.store_sub_from_scalar_ad(227, 0.744, A::div_scaled_value_offset_denominator(s.ad_value(214), 0.0004774, s.ad_value(213), 235.0, 1.0));
            s.store_mul_add_scaled_inputs3_offset_rhs(228, 15, s.ad_value(227), 1.0, s.ad_value(226), (-1.0), s.ad_value(224), (-0.4), 0.0);
            s.store_add(229, 226, 228);
            s.store_scaled_mul(230, 229, 220, 0.5);
            s.store_sub_scaled_inputs(233, 15, 0.05, 228, 0.5);
            s.store_sqrt_scaled_input(0, 213, 0.0033333333333);
            s.store_mul3_affine_lhs(2, 0, 0, 4.05e25, 0.0, 0);
            s.store_mul(248, 2, 234);
            s.store_mul_offset_ad_rhs(222, 219, A::mul(s.ad_value(17), s.ad_value(218)), 1.0);
            s.store_div_from_scalar(223, 1.0, 222);
            s.store_scaled_mul(232, 229, 223, 0.5);
            s.store_mul3_affine_lhs(249, 248, 225, (2.0 * 1.602176565e-19), 0.0, 223);
            s.store_offset_ln_ad(250, A::div_scaled_product(s.ad_value(241), s.ad_value(241), 1.0, s.ad_value(249), 1.0), (-0.6931471805599));
            s.store_mul_div_scaled_product_rhs(251, 223, s.ad_value(29), s.ad_value(14), (0.5 * 1.602176565e-19), A::add(s.ad_value(237), s.ad_value(238)), 1.0);
            s.store_mul(0, 34, 216);
            s.store_add(31, 183, 0);
            s.store_add(32, 184, 0);
        }

    }

    pub(super) fn stamp_transient_block_6(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[781] {
            s.store_mul(325, 35, 223);
            s.store_add(140, 185, 0);
            s.store_add(141, 186, 0);
        }

        s.b[783] = (p.p9 > 0.0);
        s.v[783] = if s.b[783] { 1.0 } else { 0.0 };

        if (s.b[781] && s.b[783]) {
            s.store_mul_add_ad_rhs(245, 219, A::ln(A::div(s.ad_value(24), s.ad_value(247))), s.ad_value(231));
        }

        s.b[784] = (p.p10 == 1.0);
        s.v[784] = if s.b[784] { 1.0 } else { 0.0 };

        if (s.b[781] && s.b[784]) {
            s.store_scaled_add_ad(253, A::offset(A::div_from_scalar(2970.0, s.ad_value(8)), 15.0), A::sqrt(A::offset(A::mul_sub_from_scalar_lhs(15.0, A::div_from_scalar(2970.0, s.ad_value(8)), A::sub_from_scalar(15.0, A::div_from_scalar(2970.0, s.ad_value(8)))), 1e-6)), 0.5);
        }

        if s.b[781] {
            s.store_scalar(254, 0.0);
        }

        s.b[785] = (p.p13 > 0.0);
        s.v[785] = if s.b[785] { 1.0 } else { 0.0 };

        s.b[786] = (p.p14 == 1.0);
        s.v[786] = if s.b[786] { 1.0 } else { 0.0 };

        if ((s.b[781] && s.b[785]) && s.b[786]) {
            s.store_scale_ad(254, A::exp_scaled_input(A::ln(A::mul(s.ad_value(222), s.ad_value(255))), (-0.3333333333333)), ((0.4 * p.p13) * 1.27520989));
        }

        if ((s.b[781] && s.b[785]) && (!s.b[786])) {
            s.store_scale_ad(254, A::exp_scaled_input(A::ln(A::mul(s.ad_value(222), s.ad_value(255))), (-0.3333333333333)), ((0.4 * p.p13) * 1.5412087));
        }

        if s.b[781] {
            s.store_add_scaled_product_indices(0, 252, 1.0, 23, 216, p.p14);
            s.store_sub_offset_lhs(2, 0, p.p34, 245);
            s.store_add_scaled_inputs4_indices(21, 179, p.p14, 233, p.p14, 239, p.p14, 2, 1.0);
            s.store_add_scaled_inputs4_indices(22, 180, p.p14, 233, p.p14, 240, p.p14, 0, 1.0);
            s.store_add_scaled_inputs4_indices(130, 181, p.p14, 233, p.p14, 239, p.p14, 2, 1.0);
            s.store_add_scaled_inputs4_indices(131, 182, p.p14, 233, p.p14, 240, p.p14, 0, 1.0);
            s.store_ln(291, 218);
            s.store_scaled_exp_ad(292, A::mul(s.ad_value(40), s.ad_value(291)), p.p35);
            s.store_mul(38, 187, 292);
            s.store_mul(39, 188, 292);
            s.store_exp_mul(293, 48, 291);
            s.store_mul(46, 189, 293);
            s.store_exp_mul(294, 49, 291);
            s.store_mul(47, 190, 294);
            s.store_exp_mul(295, 43, 291);
            s.store_mul(33, 191, 295);
            s.store_exp_mul(296, 45, 291);
            s.store_mul(44, 192, 296);
            s.store_exp_mul(297, 52, 291);
            s.store_mul(50, 193, 297);
            s.store_div_scaled_inputs_indices(0, 222, 1e-8, 14, 1.0);
            s.store_mul(263, 0, 46);
            s.store_exp_mul(298, 55, 291);
            s.store_mul(53, 194, 298);
            s.store_scaled_mul(268, 53, 222, 2.0);
            s.store_exp_mul(299, 60, 291);
            s.store_mul3_lhs(59, 195, 299, 292);
            s.store_mul(269, 59, 222);
            s.store_mul3_lhs(147, 196, 299, 292);
            s.store_mul(270, 147, 222);
            s.store_mul(271, 64, 223);
            s.store_exp_mul_scaled_lhs_indices(300, 76, -1.0, 291);
            s.store_mul(68, 197, 300);
            s.store_mul(69, 198, 300);
            s.store_mul(70, 199, 300);
            s.store_mul(71, 200, 300);
            s.store_mul(72, 201, 300);
            s.store_exp_mul_scaled_lhs_indices(300, 77, -1.0, 291);
            s.store_mul(73, 202, 300);
            s.store_mul(74, 203, 300);
            s.store_scale(279, 229, 0.5);
            s.store_mul(280, 75, 222);
            s.store_mul(281, 75, 219);
            s.store_div_from_scalar_offset_ad(282, 1.0, A::mul(s.ad_value(88), s.ad_value(232)), 1.0);
            s.store_scale(0, 18, 500000000.0);
            s.store_scaled_add_ad(273, A::offset(A::mul(s.ad_value(93), s.ad_value(216)), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(93), s.ad_value(216)), 1.0, A::offset(A::mul(s.ad_value(93), s.ad_value(216)), 1.0)), 0.01)), 0.5);
            s.store_mul3_lhs(91, 204, 273, 0);
            s.store_scaled_add_ad(273, A::offset(A::mul(s.ad_value(94), s.ad_value(216)), 1.0), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(94), s.ad_value(216)), 1.0, A::offset(A::mul(s.ad_value(94), s.ad_value(216)), 1.0)), 0.01)), 0.5);
            s.store_mul3_lhs(92, 205, 273, 0);
            s.store_mul_exp_ad_rhs(113, 206, A::mul_scaled_lhs(s.ad_value(114), -1.0, s.ad_value(291)));
            s.store_mul(283, 116, 222);
            s.store_div_scaled_inputs_mixed_ia(287, 118, (0.25 * 1.602176565e-19), A::mul(s.ad_value(225), s.ad_value(222)), 1.0);
            s.store_ln_div(288, 118, 248);
            s.store_scaled_mul(289, 119, 222, 1.25e-6);
            s.store_exp_mul(301, 169, 291);
            s.store_mul(168, 210, 301);
        }

        let (assign9260_e8302,) = {
    if s.b[781] {
        let assign9260_e8298: f64 = (4.0 * 1.3806488e-23);
        let assign9260_e8300: f64 = (assign9260_e8298 * s.v[213]);
        (assign9260_e8300,)
    } else {
        (s.v[302],)
    }
};
        s.v[302] = assign9260_e8302;

        let (assign9270_e8308,) = {
    if s.b[781] {
        let assign9270_e8306: f64 = (s.v[171] * s.v[302]);
        (assign9270_e8306,)
    } else {
        (s.v[303],)
    }
};
        s.v[303] = assign9270_e8308;

        s.b[787] = (p.p14 == 1.0);
        s.v[787] = if s.b[787] { 1.0 } else { 0.0 };

        if s.b[787] {
            s.store_voltage(326, ctx, nodes, Some(9), Some(6));
            s.store_voltage(698, ctx, nodes, Some(7), Some(6));
            s.store_voltage(327, ctx, nodes, Some(6), Some(8));
        }

        if (!s.b[787]) {
            s.store_scaled_voltage(326, ctx, nodes, Some(9), Some(6), -1.0);
            s.store_scaled_voltage(698, ctx, nodes, Some(7), Some(6), -1.0);
            s.store_scaled_voltage(327, ctx, nodes, Some(6), Some(8), -1.0);
        }

        s.store_neg(699, 698);

        s.store_add(328, 326, 699);

        s.store_add(329, 698, 327);

        s.b[788] = (s.v[698] < 0.0);
        s.v[788] = if s.b[788] { 1.0 } else { 0.0 };

        if s.b[788] {
            s.store_scalar(330, (-1.0));
            s.copy_ad(332, 699);
            s.copy_ad(331, 328);
            s.copy_ad(333, 329);
        }

        if (!s.b[788]) {
            s.store_scalar(330, 1.0);
            s.copy_ad(332, 698);
            s.copy_ad(331, 326);
            s.copy_ad(333, 327);
        }

        s.store_add(334, 331, 333);

        s.store_mul(335, 332, 223);

        s.store_mul_offset_ad_lhs(336, A::sqrt(A::offset(A::square(s.ad_value(332)), 0.01)), (-0.1), 223);

        s.store_scaled_sub(337, 335, 336, 0.5);

        s.copy_ad(865, 21);

        s.copy_ad(866, 22);

        s.copy_ad(867, 27);

        s.copy_ad(868, 28);

        s.copy_ad(869, 31);

        s.copy_ad(870, 32);

        s.copy_ad(871, 269);

        s.copy_ad(872, 211);

        s.copy_ad(873, 63);

        s.store_sub_ad_lhs(874, A::add_scaled_product(s.ad_value(337), (-1.0), A::sub(s.ad_value(331), s.ad_value(865)), s.ad_value(223), 1.0), 230);

        s.store_add_scaled_product_left_ad(875, 337, (-1.0), A::sub_scaled_inputs(s.ad_value(333), -1.0, s.ad_value(866), 1.0), 223, 1.0);

        s.store_sub(876, 875, 230);

        s.b[1055] = (p.p2 > 0.0);
        s.v[1055] = if s.b[1055] { 1.0 } else { 0.0 };

        if s.b[1055] {
            s.store_scale(0, 16, p.p14);
            s.store_div_scaled_offset_numerator(877, s.ad_value(242), 1.0, 1.0, A::offset(s.ad_value(243), 1.0), 1.0);
            s.store_ln(878, 877);
        }

        s.b[1056] = (s.v[878] > 1e-8);
        s.v[1056] = if s.b[1056] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1056]) {
            s.store_div_scaled_product_offset_denominator(879, s.ad_value(878), A::offset(s.ad_value(877), 1.0), 2.0, s.ad_value(877), (-1.0), 1.0);
        }

        if (s.b[1055] && (!s.b[1056])) {
            s.store_scaled_offset(879, 878, 2.0, 2.0);
        }

        if s.b[1055] {
            s.store_div_ad_rhs(880, 249, A::square(s.ad_value(241)));
            s.store_div_from_scalar(881, 1.0, 242);
            s.store_div_from_scalar(882, 1.0, 243);
            s.store_div_from_scalar_add_ad(909, 1.0, A::offset(s.ad_value(881), 1.0), s.ad_value(882));
            s.store_mul_sub_rhs(910, 909, 874, 876);
            s.store_add_scaled_product_indices(883, 874, 1.0, 910, 881, (-1.0));
            s.store_add_scaled_product_indices(884, 876, 1.0, 910, 882, 1.0);
            s.store_div_from_scalar_offset_input(789, 1.0, 242, 1.0);
            s.store_div_from_scalar_offset_input(790, 1.0, 243, 1.0);
            s.store_offset_ln_ad(792, A::div_scaled_product(A::add_scaled_product(s.ad_value(242), 1.0, s.ad_value(243), s.ad_value(790), 1.0), s.ad_value(879), 1.0, s.ad_value(880), 1.0), 1.5);
            s.store_offset_ln_ad(793, A::div_scaled_product(A::add_scaled_product(s.ad_value(243), 1.0, s.ad_value(242), s.ad_value(789), 1.0), s.ad_value(879), 1.0, s.ad_value(880), 1.0), 1.5);
        }

        s.b[1057] = (((s.v[792] - s.v[883]) / 1.5) < 80.0);
        s.v[1057] = if s.b[1057] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1057]) {
            s.store_ln_one_plus_exp_ad(791, A::sub_scaled_inputs(s.ad_value(792), 0.6666666666666666, s.ad_value(883), 0.6666666666666666));
        }

        if (s.b[1055] && (!s.b[1057])) {
            s.store_scaled_sub(791, 792, 883, 0.6666666666666666);
        }

        if s.b[1055] {
            s.store_sub_scaled_inputs(796, 792, 1.0, 791, 1.5);
            s.store_mul_add_scaled_product_rhs(795, 790, s.ad_value(796), 1.0, s.ad_value(243), s.ad_value(876), 1.0);
        }

        s.b[1058] = (((s.v[793] - s.v[795]) / 1.5) < 80.0);
        s.v[1058] = if s.b[1058] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1058]) {
            s.store_ln_one_plus_exp_ad(791, A::sub_scaled_inputs(s.ad_value(793), 0.6666666666666666, s.ad_value(795), 0.6666666666666666));
        }

        if (s.b[1055] && (!s.b[1058])) {
            s.store_scaled_sub(791, 793, 795, 0.6666666666666666);
        }

        if s.b[1055] {
            s.store_sub_scaled_inputs(1, 793, 1.0, 791, 1.5);
            s.store_mul(2, 0, 1);
            s.store_mul(3, 0, 876);
            s.store_sub(841, 2, 3);
        }

        s.b[1059] = ((((-s.v[262])) as f64).abs() < 80.0);
        s.v[1059] = if s.b[1059] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1059]) {
            s.store_exp_neg_input(842, 262);
        }

    }

    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1060] = ((-s.v[262]) < (-80.0));
        s.v[1060] = if s.b[1060] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1059])) && s.b[1060]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(842, 1.80485e-35, A::neg(A::neg(s.ad_value(262))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(262))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(262))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((s.b[1055] && (!s.b[1059])) && (!s.b[1060])) {
            s.store_scaled_offset_mul_offset_lhs_ad(842, A::neg(s.ad_value(262)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(262)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(262)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        s.b[1061] = (((s.v[841]) as f64).abs() <= s.v[261]);
        s.v[1061] = if s.b[1061] { 1.0 } else { 0.0 };

        if (s.b[1055] && s.b[1061]) {
            s.store_scaled_square(839, 260, (0.1666666666667 * 0.707106781186545));
            s.store_mul_ad_product_rhs(4, 841, s.ad_value(260), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(841), 1.0, s.ad_value(842)), s.ad_value(256), s.ad_value(839)), 1.0));
        }

        s.b[1062] = (s.v[841] < (-s.v[261]));
        s.v[1062] = if s.b[1062] { 1.0 } else { 0.0 };

        if ((s.b[1055] && (!s.b[1061])) && s.b[1062]) {
            s.store_neg(843, 841);
            s.store_scaled_mul(844, 843, 260, 1.25);
            s.store_scaled_sub_ad(845, A::offset(s.ad_value(844), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(844), (-6.0), A::offset(s.ad_value(844), (-6.0))), 64.0)), 0.5);
            s.store_sub(838, 843, 845);
            s.store_add_scaled_square_product_mixed_iia(846, 838, 1.0, 257, A::offset(s.ad_value(845), 1.0), 1.0);
            s.store_sub_scaled_inputs(848, 838, 2.0, 257, 1.0);
            s.store_sub_ad_lhs(849, A::ln(A::mul(s.ad_value(846), s.ad_value(258))), 845);
            s.store_add(836, 846, 848);
            s.store_add_scaled_square_product_mixed_iia(837, 836, 1.0, 849, A::add_scaled_product(s.ad_value(846), (-1.0), s.ad_value(848), s.ad_value(848), 0.5), 1.0);
            s.store_add_ad_rhs(850, 845, A::div_scaled_product3(s.ad_value(846), s.ad_value(836), s.ad_value(849), 1.0, A::add(s.ad_value(837), A::mul3(A::mul3(A::div(s.ad_value(836), s.ad_value(837)), s.ad_value(849), s.ad_value(849)), s.ad_value(848), A::sub_scaled_inputs(A::square(s.ad_value(848)), 0.3333333333333, s.ad_value(846), 1.0))), 1.0));
        }

        s.b[1063] = (s.v[850] < 80.0);
        s.v[1063] = if s.b[1063] { 1.0 } else { 0.0 };

        if (((s.b[1055] && (!s.b[1061])) && s.b[1062]) && s.b[1063]) {
            s.store_exp(851, 850);
        }

        if (((s.b[1055] && (!s.b[1061])) && s.b[1062]) && (!s.b[1063])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(851, 850, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(850), (-80.0)), 0.5, A::scale_offset(s.ad_value(850), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if ((s.b[1055] && (!s.b[1061])) && s.b[1062]) {
            s.store_div_from_scalar(852, 1.0, 851);
            s.store_div_from_scalar_offset_ad(838, 1.0, A::square(s.ad_value(850)), 2.0);
            s.store_mul_square_lhs(853, 850, 838);
            s.store_mul3_affine_lhs(854, 850, 838, 4.0, 0.0, 838);
            s.store_mul_ad_product_lhs(855, A::sub_scaled_inputs(s.ad_value(838), 8.0, s.ad_value(853), 12.0), s.ad_value(838), 838);
            s.store_sub(838, 843, 850);
            s.store_mul(839, 842, 852);
            s.store_add_scaled_product_right_ad(856, 838, 2.0, 257, A::add_scaled_inputs3_offset(s.ad_value(851), 1.0, s.ad_value(839), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(842), 1.0, s.ad_value(854)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(857, 838, 1.0, 257, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(851), 1.0, s.ad_value(850), (-1.0), s.ad_value(839), 1.0, (-1.0)), 1.0, s.ad_value(842), A::sub(A::offset(s.ad_value(850), (-1.0)), s.ad_value(853)), 1.0), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(838, 2.0, 257, A::add_scaled_inputs_product(s.ad_value(851), 1.0, s.ad_value(839), 1.0, s.ad_value(842), s.ad_value(855), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(838, 856, 1.0, 857, 838, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(4, 850, -1.0, A::div(s.ad_value(857), A::add(s.ad_value(856), A::sqrt(s.ad_value(838)))), 2.0);
        }

        if ((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) {
            s.store_div_from_scalar_offset_scaled_input(858, 1.0, 256, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(859, A::mul_scaled_lhs(s.ad_value(259), 1.25, s.ad_value(858)), (-1.0), 858);
            s.store_mul_ad_product_rhs(860, 841, s.ad_value(260), A::offset(A::mul(s.ad_value(859), s.ad_value(841)), 1.0));
        }

        s.b[1064] = ((-s.v[860]) > (-80.0));
        s.v[1064] = if s.b[1064] { 1.0 } else { 0.0 };

        if (((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) && s.b[1064]) {
            s.store_exp_neg_input(838, 860);
        }

        if (((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) && (!s.b[1064])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(838, 1.80485e-35, A::neg(A::neg(s.ad_value(860))), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(A::neg(s.ad_value(860))), (-80.0)), 0.5, A::scale_offset(A::neg(A::neg(s.ad_value(860))), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) {
            s.store_sub_from_scalar(861, 1.0, 838);
            s.store_add_scaled_inputs_product_right_ad(862, 841, 1.0, 257, 0.5, 256, A::sqrt(A::add_scaled_inputs3(s.ad_value(841), 1.0, s.ad_value(257), 0.25, s.ad_value(861), -1.0)), (-1.0));
            s.store_offset(863, 262, 3.0);
            s.store_sub_ad(845, A::add_scaled_inputs3(s.ad_value(862), 0.5, s.ad_value(863), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(862), s.ad_value(863)), A::sub(s.ad_value(862), s.ad_value(863))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(863), 0.5, A::sqrt(A::offset(A::square(s.ad_value(863)), 5.0)), 0.5));
            s.store_sub(838, 841, 845);
            s.store_exp_neg_input(839, 845);
            s.store_div_from_scalar_offset_ad(840, 1.0, A::square(s.ad_value(845)), 2.0);
            s.store_mul_square_lhs(853, 845, 840);
            s.store_mul3_affine_lhs(854, 845, 840, 4.0, 0.0, 840);
            s.store_mul_ad_product_lhs(855, A::sub_scaled_inputs(s.ad_value(840), 8.0, s.ad_value(853), 12.0), s.ad_value(840), 840);
            s.store_max_from_scalar_ad(846, 1e-40, A::add_scaled_square_product(s.ad_value(838), 1.0, s.ad_value(257), A::add_scaled_product(A::offset(A::add(s.ad_value(839), s.ad_value(845)), (-1.0)), 1.0, s.ad_value(842), A::add(A::offset(s.ad_value(845), 1.0), s.ad_value(853)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(847, 1.0, 257, A::add_scaled_product(s.ad_value(839), 1.0, s.ad_value(842), s.ad_value(855), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(848, 838, 2.0, 257, A::add_scaled_sub_value_product(1.0, s.ad_value(839), 1.0, s.ad_value(842), A::offset(s.ad_value(854), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(849, 262, 1.0, 845, (-1.0), A::ln(A::div(s.ad_value(846), s.ad_value(257))), 1.0);
            s.store_add(836, 846, 848);
            s.store_add_scaled_square_product_mixed_iia(837, 836, 1.0, 849, A::add_scaled_products(s.ad_value(848), s.ad_value(848), 0.5, s.ad_value(846), s.ad_value(847), (-1.0)), 1.0);
            s.store_add_ad_rhs(864, 845, A::div_scaled_product3(s.ad_value(846), s.ad_value(836), s.ad_value(849), 1.0, A::add(s.ad_value(837), A::mul3(A::mul3(A::div(s.ad_value(836), s.ad_value(837)), s.ad_value(849), s.ad_value(849)), s.ad_value(848), A::add_scaled_square_product(s.ad_value(848), 0.3333333333333, s.ad_value(846), s.ad_value(847), (-1.0)))), 1.0));
        }

        s.b[1065] = (s.v[864] < 80.0);
        s.v[1065] = if s.b[1065] { 1.0 } else { 0.0 };

        if (((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) && s.b[1065]) {
            s.store_exp(851, 864);
            s.store_div_from_scalar(852, 1.0, 851);
            s.store_mul(851, 842, 851);
        }

        s.b[1066] = (s.v[864] > (s.v[262] - 80.0));
        s.v[1066] = if s.b[1066] { 1.0 } else { 0.0 };

        if ((((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) && (!s.b[1065])) && s.b[1066]) {
            s.store_exp_sub(851, 864, 262);
            s.store_div(852, 842, 851);
        }

        if ((((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) && (!s.b[1065])) && (!s.b[1066])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(851, 1.80485e-35, A::sub(s.ad_value(262), s.ad_value(864)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(262), s.ad_value(864)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(262), s.ad_value(864)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(852, 1.80485e-35, 864, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(864), (-80.0)), 0.5, A::scale_offset(s.ad_value(864), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((s.b[1055] && (!s.b[1061])) && (!s.b[1062])) {
            s.store_div_from_scalar_offset_ad(838, 1.0, A::square(s.ad_value(864)), 2.0);
            s.store_mul_square_lhs(853, 864, 838);
            s.store_mul3_affine_lhs(854, 864, 838, 4.0, 0.0, 838);
            s.store_mul_ad_product_lhs(855, A::sub_scaled_inputs(s.ad_value(838), 8.0, s.ad_value(853), 12.0), s.ad_value(838), 838);
            s.store_sub(838, 841, 864);
            s.store_add_scaled_product_right_ad(856, 838, 2.0, 257, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(852)), 1.0, s.ad_value(851), 1.0, s.ad_value(842), A::offset(s.ad_value(854), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(857, 838, 1.0, 257, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(852), 1.0, s.ad_value(864), 1.0, s.ad_value(851), 1.0, (-1.0)), 1.0, s.ad_value(842), A::add(A::offset(s.ad_value(864), 1.0), s.ad_value(853)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(838, 2.0, 257, A::add_scaled_inputs_product(s.ad_value(852), 1.0, s.ad_value(851), 1.0, s.ad_value(842), s.ad_value(855), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(838, 856, 1.0, 857, 838, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(4, 864, 1.0, A::div(s.ad_value(857), A::add(s.ad_value(856), A::sqrt(s.ad_value(838)))), 2.0);
        }

        if s.b[1055] {
            s.store_mul_add_rhs(885, 0, 4, 3);
        }

        if (!s.b[1055]) {
            s.copy_ad(885, 876);
        }

        s.store_mul_sub_rhs(0, 244, 874, 885);

        s.b[1067] = (p.p13 > 0.0);
        s.v[1067] = if s.b[1067] { 1.0 } else { 0.0 };

        if s.b[1067] {
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(886, 0, 0.5, 253, 0.5, A::add_scaled_square_product(s.ad_value(253), 1.0, A::sub(s.ad_value(0), s.ad_value(253)), A::sub(s.ad_value(0), s.ad_value(253)), 1.0), 0.5);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(887, 253, 0.5, 0, ((-1.0) * 0.5), A::add_scaled_square_product(s.ad_value(253), 1.0, A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(253), 1.0), A::sub_scaled_inputs(s.ad_value(0), -1.0, s.ad_value(253), 1.0), 1.0), 0.5);
            s.store_mul_ad_rhs(2, 254, A::exp_scaled_input(A::ln(s.ad_value(886)), (-0.3333333333333)));
            s.store_mul_ad_rhs(3, 254, A::exp_scaled_input(A::ln(s.ad_value(887)), (-0.3333333333333)));
            s.store_sub_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
            s.store_div(894, 241, 4);
            s.store_offset_mul(888, 242, 2, 1.0);
            s.store_offset_mul(889, 243, 3, 1.0);
            s.store_div_scaled_product_indices(890, 242, 4, 1.0, 888, 1.0);
            s.store_div_scaled_product_indices(891, 243, 4, 1.0, 889, 1.0);
            s.store_div_from_scalar_add_ad(892, 1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(890)), 1.0), A::div_from_scalar(1.0, s.ad_value(891)));
            s.store_offset_mul(888, 890, 2, 1.0);
            s.store_offset_mul(889, 891, 3, 1.0);
        }

        if (!s.b[1067]) {
            s.copy_ad(894, 241);
            s.copy_ad(890, 242);
            s.copy_ad(891, 243);
            s.copy_ad(892, 244);
            s.store_scalar(888, 1.0);
            s.store_scalar(889, 1.0);
        }

        s.store_mul_sub_rhs(893, 892, 874, 885);

        s.b[1068] = (s.v[893] > 0.0);
        s.v[1068] = if s.b[1068] { 1.0 } else { 0.0 };

        s.b[1069] = ((-s.v[893]) < 80.0);
        s.v[1069] = if s.b[1069] { 1.0 } else { 0.0 };

        if (s.b[1068] && s.b[1069]) {
            s.store_ln_one_plus_exp_neg_input(0, 893);
        }

        if (s.b[1068] && (!s.b[1069])) {
            s.store_neg(0, 893);
        }

        if s.b[1068] {
            s.store_add_scaled_inputs3_offset_mixed_iai(895, 874, 1.0, A::div(s.ad_value(893), s.ad_value(890)), (-1.0), 0, 1.0, (-0.6931471805599));
        }

        s.b[1070] = (s.v[893] < 80.0);
        s.v[1070] = if s.b[1070] { 1.0 } else { 0.0 };

        if ((!s.b[1068]) && s.b[1070]) {
            s.store_ln_one_plus_exp(0, 893);
        }

        if ((!s.b[1068]) && (!s.b[1070])) {
            s.copy_ad(0, 893);
        }

        if (!s.b[1068]) {
            s.store_add_scaled_inputs3_offset_mixed_iai(895, 885, 1.0, A::div(s.ad_value(893), s.ad_value(891)), 1.0, 0, 1.0, (-0.6931471805599));
        }

        s.store_add_scaled_inputs3_sqrt_third_mixed_iia(896, 895, 0.5, 250, 0.5, A::offset(A::mul(A::sub(s.ad_value(895), s.ad_value(250)), A::sub(s.ad_value(895), s.ad_value(250))), 4.0), (-0.5));

        s.store_offset_sqrt_ad(897, A::offset(A::div_scaled_inputs2(s.ad_value(250), 2.0, s.ad_value(896), (-2.0), s.ad_value(251), 1.0), 1.0), (-1.0));

        s.store_add_scaled_product_indices(898, 896, 1.0, 251, 897, 1.0);

        s.store_scaled_add_ad(0, A::offset(A::mul(s.ad_value(30), s.ad_value(875)), ((1.0) + (0.5))), A::sqrt(A::offset(A::mul_offset_lhs(A::mul(s.ad_value(30), s.ad_value(875)), ((1.0) + ((-0.5))), A::offset(A::mul(s.ad_value(30), s.ad_value(875)), ((1.0) + ((-0.5))))), 0.01)), 0.5);

        s.store_div_from_scalar_offset_ad(899, 1.0, A::mul(s.ad_value(867), s.ad_value(0)), 1.0);

        s.store_div_from_scalar_offset_ad(900, 1.0, A::mul(s.ad_value(868), s.ad_value(0)), 1.0);

        s.store_mul_offset_rhs_ad(0, A::mul3_scaled_output(s.ad_value(325), A::offset(A::sqrt(A::offset(A::div(s.ad_value(336), s.ad_value(325)), 1.0)), (-1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(897)), 1.0), 2.0), A::mul(s.ad_value(37), s.ad_value(875)), 1.0);

        s.store_mul(901, 869, 0);

        s.store_mul(902, 870, 0);

        s.store_add_ad_lhs(903, A::add_scaled_product(s.ad_value(898), 1.0, A::add_scaled_inputs3(s.ad_value(874), 1.0, s.ad_value(898), (-1.0), s.ad_value(901), 1.0), s.ad_value(899), 1.0), 337);

        s.store_add_ad_lhs(904, A::add_scaled_product(s.ad_value(898), 1.0, A::add_scaled_inputs3(s.ad_value(885), 1.0, s.ad_value(898), (-1.0), s.ad_value(902), 1.0), s.ad_value(900), 1.0), 337);

        let assign11130_ad_e10500: A = A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(904), 1.0, s.ad_value(25), A::sub(s.ad_value(903), s.ad_value(904)), 1.0), 1.0, s.ad_value(221), 1.0, A::sqrt(A::offset(A::mul(A::sub(A::add_scaled_product(s.ad_value(904), 1.0, s.ad_value(25), A::sub(s.ad_value(903), s.ad_value(904)), 1.0), s.ad_value(221)), A::sub(A::add_scaled_product(s.ad_value(904), 1.0, s.ad_value(25), A::sub(s.ad_value(903), s.ad_value(904)), 1.0), s.ad_value(221))), 0.01)), -1.0);
        s.store_scale_ad(905, assign11130_ad_e10500, 0.5);

        let assign11140_ad_e10534: A = A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(903), 1.0, s.ad_value(26), A::sub(s.ad_value(904), s.ad_value(903)), 1.0), 1.0, s.ad_value(221), 1.0, A::sqrt(A::offset(A::mul(A::sub(A::add_scaled_product(s.ad_value(903), 1.0, s.ad_value(26), A::sub(s.ad_value(904), s.ad_value(903)), 1.0), s.ad_value(221)), A::sub(A::add_scaled_product(s.ad_value(903), 1.0, s.ad_value(26), A::sub(s.ad_value(904), s.ad_value(903)), 1.0), s.ad_value(221))), 0.01)), -1.0);
        s.store_scale_ad(906, assign11140_ad_e10534, 0.5);

        s.store_div(907, 890, 899);

        s.store_div(908, 891, 900);

        s.store_div_from_scalar(881, 1.0, 907);

        s.store_div_from_scalar(882, 1.0, 908);

        s.store_div_from_scalar_add_ad(909, 1.0, A::offset(s.ad_value(881), 1.0), s.ad_value(882));

        s.store_div_ad_rhs(880, 249, A::square(s.ad_value(894)));

        s.store_div_scaled_offset_numerator(877, s.ad_value(907), 1.0, 1.0, A::offset(s.ad_value(908), 1.0), 1.0);

        s.store_ln(878, 877);

        s.b[1071] = (s.v[878] > 1e-8);
        s.v[1071] = if s.b[1071] { 1.0 } else { 0.0 };

        if s.b[1071] {
            s.store_div_scaled_product_offset_denominator(879, s.ad_value(878), A::offset(s.ad_value(877), 1.0), 2.0, s.ad_value(877), (-1.0), 1.0);
        }

        if (!s.b[1071]) {
            s.store_scaled_offset(879, 878, 2.0, 2.0);
        }

        s.store_mul_sub_rhs(910, 909, 905, 906);

        s.store_square(911, 910);

        s.store_add_scaled_product_indices(883, 905, 1.0, 910, 881, (-1.0));

        s.store_add_scaled_product_indices(884, 906, 1.0, 910, 882, 1.0);

        s.store_div_from_scalar_offset_input(789, 1.0, 907, 1.0);

        s.store_div_from_scalar_offset_input(790, 1.0, 908, 1.0);

        s.store_offset_ln_ad(792, A::div_scaled_product(A::add_scaled_product(s.ad_value(907), 1.0, s.ad_value(908), s.ad_value(790), 1.0), s.ad_value(879), 1.0, s.ad_value(880), 1.0), 3.0);

        s.store_offset_ln_ad(793, A::div_scaled_product(A::add_scaled_product(s.ad_value(908), 1.0, s.ad_value(907), s.ad_value(789), 1.0), s.ad_value(879), 1.0, s.ad_value(880), 1.0), 3.0);

        s.b[1072] = (((s.v[792] - s.v[883]) * 0.3333333333333) < 80.0);
        s.v[1072] = if s.b[1072] { 1.0 } else { 0.0 };

        if s.b[1072] {
            s.store_ln_one_plus_exp_ad(791, A::sub_scaled_inputs(s.ad_value(792), 0.3333333333333, s.ad_value(883), 0.3333333333333));
        }

        if (!s.b[1072]) {
            s.store_scaled_sub(791, 792, 883, 0.3333333333333);
        }

        s.store_sub_scaled_inputs(796, 792, 1.0, 791, 3.0);

        s.b[1073] = (((s.v[793] - s.v[884]) * 0.3333333333333) < 80.0);
        s.v[1073] = if s.b[1073] { 1.0 } else { 0.0 };

        if s.b[1073] {
            s.store_ln_one_plus_exp_ad(791, A::sub_scaled_inputs(s.ad_value(793), 0.3333333333333, s.ad_value(884), 0.3333333333333));
        }

        if (!s.b[1073]) {
            s.store_scaled_sub(791, 793, 884, 0.3333333333333);
        }

        s.store_sub_scaled_inputs(797, 793, 1.0, 791, 3.0);

        s.store_mul_add_scaled_product_rhs(794, 789, s.ad_value(797), 1.0, s.ad_value(907), s.ad_value(905), 1.0);

    }

    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
    ) {
        s.store_mul_add_scaled_product_rhs(795, 790, s.ad_value(796), 1.0, s.ad_value(908), s.ad_value(906), 1.0);

        s.b[1074] = (((s.v[792] - s.v[794]) * 0.3333333333333) < 80.0);
        s.v[1074] = if s.b[1074] { 1.0 } else { 0.0 };

        if s.b[1074] {
            s.store_ln_one_plus_exp_ad(791, A::sub_scaled_inputs(s.ad_value(792), 0.3333333333333, s.ad_value(794), 0.3333333333333));
        }

        if (!s.b[1074]) {
            s.store_scaled_sub(791, 792, 794, 0.3333333333333);
        }

        s.store_sub_scaled_inputs(796, 792, 1.0, 791, 3.0);

        s.b[1075] = (((s.v[793] - s.v[795]) * 0.3333333333333) < 80.0);
        s.v[1075] = if s.b[1075] { 1.0 } else { 0.0 };

        if s.b[1075] {
            s.store_ln_one_plus_exp_ad(791, A::sub_scaled_inputs(s.ad_value(793), 0.3333333333333, s.ad_value(795), 0.3333333333333));
        }

        if (!s.b[1075]) {
            s.store_scaled_sub(791, 793, 795, 0.3333333333333);
        }

        s.store_sub_scaled_inputs(797, 793, 1.0, 791, 3.0);

        s.store_sub(912, 905, 796);

        s.store_sub(916, 906, 797);

        s.v[803] = 0.0;

        s.v[806] = 0.0;

        s.store_mul(798, 907, 912);

        s.b[1076] = ((s.v[905] - s.v[912]) < 80.0);
        s.v[1076] = if s.b[1076] { 1.0 } else { 0.0 };

        if s.b[1076] {
            s.store_exp_sub(789, 905, 912);
        }

        if (!s.b[1076]) {
            s.store_scaled_offset_mul_offset_lhs_ad(789, A::sub(s.ad_value(905), s.ad_value(912)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(905), s.ad_value(912)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(905), s.ad_value(912)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        s.store_mul(799, 880, 789);

        s.store_sub_ad_lhs(800, A::square(s.ad_value(798)), 799);

        s.store_add_scaled_product_indices(801, 799, 1.0, 907, 798, 2.0);

        s.store_add_scaled_product_indices(802, 799, (-1.0), 907, 907, 2.0);

        s.b[1077] = (s.v[800] < (-0.005));
        s.v[1077] = if s.b[1077] { 1.0 } else { 0.0 };

        if s.b[1077] {
            s.store_sqrt_abs_ad(803, s.ad_value(800));
            s.store_div_ad_rhs(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));
            s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);
            s.store_mul_add_ad_lhs(805, s.ad_value(800), A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 789);
            s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);
            s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);
            s.store_mul_div_lhs(810, 801, 800, 790);
            s.store_div_ad_lhs(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);
        }

        s.b[1078] = (s.v[800] > 0.005);
        s.v[1078] = if s.b[1078] { 1.0 } else { 0.0 };

        if ((!s.b[1077]) && s.b[1078]) {
            s.store_sqrt_abs_ad(803, s.ad_value(800));
            s.store_exp_neg_input(806, 803);
            s.store_div_scaled_product_offset_rhs(804, s.ad_value(803), s.ad_value(806), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);
            s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);
            s.store_mul_add_ad_lhs(805, s.ad_value(800), A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 789);
            s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);
            s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);
            s.store_mul_div_lhs(810, 801, 800, 790);
            s.store_div_ad_lhs(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);
        }

        if ((!s.b[1077]) && (!s.b[1078])) {
            s.store_offset_scaled_ad(791, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(804, 800, 791, 2.0);
            s.store_offset_scaled_ad(789, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_mul(805, 801, 789);
            s.store_offset_scaled_ad(790, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(807, 802, 789, 1.0, A::square(s.ad_value(801)), 790, (-1.0));
            s.store_scaled_mul(810, 801, 791, (-0.5));
            s.store_add_scaled_product_value_ad(811, A::mul3_scaled_output(s.ad_value(801), s.ad_value(801), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 2.0, A::scale(s.ad_value(800), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 802, 791, (-0.5));
        }

        s.b[1079] = (s.v[800] > 0.005);
        s.v[1079] = if s.b[1079] { 1.0 } else { 0.0 };

        if s.b[1079] {
            s.store_div_scaled_inputs_mixed_ia(790, 800, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0);
            s.store_mul(808, 790, 806);
            s.store_sub_ad_lhs(809, A::ln(s.ad_value(790)), 803);
        }

        s.b[1080] = (s.v[800] < (-0.005));
        s.v[1080] = if s.b[1080] { 1.0 } else { 0.0 };

        if ((!s.b[1079]) && s.b[1080]) {
            s.store_sin_scaled_input(790, 803, 0.5);
            s.store_div_scaled_inputs_mixed_ia(808, 800, -1.0, A::square(s.ad_value(790)), 1.0);
            s.store_ln(809, 808);
        }

        if ((!s.b[1079]) && (!s.b[1080])) {
            s.store_sub_from_scalar_ad(808, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(809, 808);
        }

        s.b[1081] = (((1.01 * s.v[798]) + s.v[804]) > 0.0);
        s.v[1081] = if s.b[1081] { 1.0 } else { 0.0 };

        if s.b[1081] {
            s.store_add(812, 798, 804);
            s.store_add(813, 907, 805);
            s.copy_ad(814, 807);
        }

        if (!s.b[1081]) {
            s.store_div_from_scalar_sub_ad(790, 1.0, s.ad_value(798), s.ad_value(804));
            s.store_sub(791, 805, 907);
            s.store_mul_sub_lhs(812, 799, 808, 790);
            s.store_mul_ad_lhs(813, A::add_scaled_value_products(s.ad_value(799), (-1.0), s.ad_value(791), s.ad_value(812), 1.0, s.ad_value(810), s.ad_value(808), (-1.0)), 790);
            s.store_mul_ad_lhs(814, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(807), s.ad_value(812), 1.0, s.ad_value(791), s.ad_value(813), 2.0), 1.0, s.ad_value(799), 1.0, A::add(s.ad_value(811), A::square(s.ad_value(810))), s.ad_value(808), (-1.0)), 790);
        }

        s.b[1082] = (s.v[812] > 0.0);
        s.v[1082] = if s.b[1082] { 1.0 } else { 0.0 };

        if s.b[1082] {
            s.store_ln(815, 812);
            s.store_div_from_scalar(789, 1.0, 812);
            s.store_mul(816, 813, 789);
            s.store_add_scaled_square_product_indices(817, 816, (-1.0), 814, 789, 1.0);
        }

        if (!s.b[1082]) {
            s.store_add_offset_lhs_ad_rhs(815, 798, 0.6931471805599, A::ln_scaled_input(s.ad_value(798), -1.0));
            s.store_div_from_scalar(789, 1.0, 912);
            s.store_add(816, 907, 789);
            s.store_mul_neg_lhs(817, 789, 789);
        }

        s.store_sub_ad_lhs(818, A::add_scaled_inputs4(s.ad_value(906), 1.0, s.ad_value(905), (-1.0), s.ad_value(912), 1.0, s.ad_value(815), 2.0), 809);

        s.store_sub_ad_lhs(819, A::scale_offset(s.ad_value(816), 2.0, 1.0), 810);

        s.store_sub_scaled_inputs(820, 817, 2.0, 811, 1.0);

        s.store_add_scaled_product_indices(821, 798, 1.0, 908, 818, 1.0);

        s.store_add_scaled_product_indices(822, 907, 1.0, 908, 819, 1.0);

        s.store_mul(823, 908, 820);

        s.store_add_scaled_product_indices(824, 799, (-1.0), 821, 812, 1.0);

        s.store_add_ad_lhs(825, A::add_scaled_products(s.ad_value(822), s.ad_value(812), 1.0, s.ad_value(821), s.ad_value(813), 1.0), 799);

        s.store_sub_ad_lhs(826, A::add_scaled_products3(s.ad_value(823), s.ad_value(812), 1.0, s.ad_value(822), s.ad_value(813), 2.0, s.ad_value(821), s.ad_value(814), 1.0), 799);

        s.store_add_scaled_square_product_indices(835, 825, 1.0, 824, 826, (-0.5));

        s.store_div_scaled_product3_mixed_iiia(827, 824, 825, 835, -1.0, A::offset(A::square(s.ad_value(835)), 1e-200), 1.0);

        s.store_add(912, 912, 827);

        s.store_mul(798, 907, 912);

        s.store_mul(828, 908, 916);

        s.store_add(821, 798, 828);

        s.store_offset_scaled(829, 821, 0.065345483024, 1.0);

        s.store_add_scaled_product_value_ad(830, A::scale_offset(s.ad_value(821), 8.5797362674, 39.478417604), 1.0, 798, 828, 1.0);

        s.store_add_scaled_product_indices(831, 821, (2.0 * 39.478417604), 798, 828, 39.478417604);

        s.store_sqrt_ad(832, A::add_scaled_square_product(s.ad_value(830), 1.0, s.ad_value(829), s.ad_value(831), (-4.0)));

        s.store_div_scaled_inputs2_indices(800, 832, 1.0, 830, (-1.0), 829, 2.0);

        s.store_sub_ad_lhs(833, A::square(s.ad_value(798)), 800);

        s.b[1083] = (s.v[833] > 0.0);
        s.v[1083] = if s.b[1083] { 1.0 } else { 0.0 };

        if s.b[1083] {
            s.store_mul_add_scaled_inputs3_offset_rhs(824, 833, A::ln(A::div(s.ad_value(833), s.ad_value(880))), 1.0, s.ad_value(905), (-1.0), s.ad_value(912), 1.0, 0.0);
            s.store_add_scaled_product_indices(825, 833, 1.0, 907, 798, 2.0);
        }

        let (assign12430_e11690,) = {
    if s.b[1083] {
        let assign12430_e11686: f64 = (s.v[905] - s.v[912]);
        let assign12430_e11688: f64 = (assign12430_e11686 - s.v[792]);
        (assign12430_e11688,)
    } else {
        (s.v[834],)
    }
};
        s.v[834] = assign12430_e11690;

        s.b[1084] = ((((s.v[824] < 0.0) && (s.v[825] > 0.0)) && (((s.v[834] + 2.3025850929941) + ((s.v[907]) as f64).ln()) > 0.0)) || (s.v[834] > 1.0));
        s.v[1084] = if s.b[1084] { 1.0 } else { 0.0 };

        if (s.b[1083] && s.b[1084]) {
            s.store_sub_div_rhs_indices(912, 912, 824, 825);
        }

        s.store_mul(798, 907, 912);

        s.store_mul(828, 908, 916);

        s.store_add(821, 798, 828);

        s.store_offset_scaled(829, 821, 0.065345483024, 1.0);

        s.store_add_scaled_product_value_ad(830, A::scale_offset(s.ad_value(821), 8.5797362674, 39.478417604), 1.0, 798, 828, 1.0);

        s.store_add_scaled_product_indices(831, 821, (2.0 * 39.478417604), 798, 828, 39.478417604);

        s.store_sqrt_ad(832, A::add_scaled_square_product(s.ad_value(830), 1.0, s.ad_value(829), s.ad_value(831), (-4.0)));

        s.store_div_scaled_inputs2_indices(800, 832, 1.0, 830, (-1.0), 829, 2.0);

        s.b[1085] = (s.v[800] < (-0.005));
        s.v[1085] = if s.b[1085] { 1.0 } else { 0.0 };

        if s.b[1085] {
            s.store_sqrt_abs_ad(803, s.ad_value(800));
            s.store_div_ad_rhs(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));
            s.store_div_scaled_inputs2_mixed_iai(805, 800, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 0.25, 800, 1.0);
        }

        s.b[1086] = (s.v[800] > 0.005);
        s.v[1086] = if s.b[1086] { 1.0 } else { 0.0 };

        if ((!s.b[1085]) && s.b[1086]) {
            s.store_sqrt_abs_ad(803, s.ad_value(800));
            s.store_exp_neg_input(806, 803);
            s.store_div_scaled_product_offset_rhs(804, s.ad_value(803), s.ad_value(806), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);
            s.store_div_scaled_inputs2_mixed_iai(805, 800, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 0.25, 800, 1.0);
        }

        if ((!s.b[1085]) && (!s.b[1086])) {
            s.store_offset_ad(804, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);
            s.store_offset_scaled_ad(805, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
        }

        s.store_sub_ad_rhs(800, 800, A::div_scaled_inputs2(A::add_scaled_products(s.ad_value(821), s.ad_value(804), 1.0, s.ad_value(798), s.ad_value(828), 1.0), 1.0, s.ad_value(800), 1.0, A::offset(A::mul(s.ad_value(821), s.ad_value(805)), 1.0), 1.0));

        s.store_sub_ad_lhs(833, A::square(s.ad_value(798)), 800);

        s.b[1087] = (s.v[833] > 0.0);
        s.v[1087] = if s.b[1087] { 1.0 } else { 0.0 };

        if s.b[1087] {
            s.store_mul_add_scaled_inputs3_offset_rhs(824, 833, A::ln(A::div(s.ad_value(833), s.ad_value(880))), 1.0, s.ad_value(905), (-1.0), s.ad_value(912), 1.0, 0.0);
            s.store_add_scaled_product_indices(825, 833, 1.0, 907, 798, 2.0);
        }

        let (assign12700_e11963,) = {
    if s.b[1087] {
        let assign12700_e11959: f64 = (s.v[905] - s.v[912]);
        let assign12700_e11961: f64 = (assign12700_e11959 - s.v[792]);
        (assign12700_e11961,)
    } else {
        (s.v[834],)
    }
};
        s.v[834] = assign12700_e11963;

        s.b[1088] = ((((s.v[824] < 0.0) && (s.v[825] > 0.0)) && (((s.v[834] + 2.3025850929941) + ((s.v[907]) as f64).ln()) > 0.0)) || (s.v[834] > 1.0));
        s.v[1088] = if s.b[1088] { 1.0 } else { 0.0 };

        if (s.b[1087] && s.b[1088]) {
            s.store_sub_div_rhs_indices(912, 912, 824, 825);
        }

        s.store_mul(798, 907, 912);

        s.b[1089] = ((s.v[905] - s.v[912]) < 80.0);
        s.v[1089] = if s.b[1089] { 1.0 } else { 0.0 };

        if s.b[1089] {
            s.store_exp_sub(789, 905, 912);
        }

        if (!s.b[1089]) {
            s.store_scaled_offset_mul_offset_lhs_ad(789, A::sub(s.ad_value(905), s.ad_value(912)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(905), s.ad_value(912)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(905), s.ad_value(912)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        s.store_mul(799, 880, 789);

        s.store_sub_ad_lhs(800, A::square(s.ad_value(798)), 799);

        s.store_add_scaled_product_indices(801, 799, 1.0, 907, 798, 2.0);

        s.store_add_scaled_product_indices(802, 799, (-1.0), 907, 907, 2.0);

        s.b[1090] = (s.v[800] < (-0.005));
        s.v[1090] = if s.b[1090] { 1.0 } else { 0.0 };

        if s.b[1090] {
            s.store_sqrt_abs_ad(803, s.ad_value(800));
            s.store_div_ad_rhs(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));
            s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);
            s.store_mul_add_ad_lhs(805, s.ad_value(800), A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 789);
            s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);
            s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);
            s.store_mul_div_lhs(810, 801, 800, 790);
            s.store_div_ad_lhs(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);
        }

        s.b[1091] = (s.v[800] > 0.005);
        s.v[1091] = if s.b[1091] { 1.0 } else { 0.0 };

        if ((!s.b[1090]) && s.b[1091]) {
            s.store_sqrt_abs_ad(803, s.ad_value(800));
            s.store_exp_neg_input(806, 803);
            s.store_div_scaled_product_offset_rhs(804, s.ad_value(803), s.ad_value(806), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);
            s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);
            s.store_mul_add_ad_lhs(805, s.ad_value(800), A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 789);
        }

    }

    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1090]) && s.b[1091]) {
            s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);
            s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);
            s.store_mul_div_lhs(810, 801, 800, 790);
            s.store_div_ad_lhs(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);
        }

        if ((!s.b[1090]) && (!s.b[1091])) {
            s.store_offset_scaled_ad(791, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(804, 800, 791, 2.0);
            s.store_offset_scaled_ad(789, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_mul(805, 801, 789);
            s.store_offset_scaled_ad(790, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(807, 802, 789, 1.0, A::square(s.ad_value(801)), 790, (-1.0));
            s.store_scaled_mul(810, 801, 791, (-0.5));
            s.store_add_scaled_product_value_ad(811, A::mul3_scaled_output(s.ad_value(801), s.ad_value(801), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 2.0, A::scale(s.ad_value(800), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 802, 791, (-0.5));
        }

        s.b[1092] = (s.v[800] > 0.005);
        s.v[1092] = if s.b[1092] { 1.0 } else { 0.0 };

        if s.b[1092] {
            s.store_div_scaled_inputs_mixed_ia(790, 800, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0);
            s.store_mul(808, 790, 806);
            s.store_sub_ad_lhs(809, A::ln(s.ad_value(790)), 803);
        }

        s.b[1093] = (s.v[800] < (-0.005));
        s.v[1093] = if s.b[1093] { 1.0 } else { 0.0 };

        if ((!s.b[1092]) && s.b[1093]) {
            s.store_sin_scaled_input(790, 803, 0.5);
            s.store_div_scaled_inputs_mixed_ia(808, 800, -1.0, A::square(s.ad_value(790)), 1.0);
            s.store_ln(809, 808);
        }

        if ((!s.b[1092]) && (!s.b[1093])) {
            s.store_sub_from_scalar_ad(808, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(809, 808);
        }

        s.b[1094] = (((1.01 * s.v[798]) + s.v[804]) > 0.0);
        s.v[1094] = if s.b[1094] { 1.0 } else { 0.0 };

        if s.b[1094] {
            s.store_add(812, 798, 804);
            s.store_add(813, 907, 805);
            s.copy_ad(814, 807);
        }

        if (!s.b[1094]) {
            s.store_div_from_scalar_sub_ad(790, 1.0, s.ad_value(798), s.ad_value(804));
            s.store_sub(791, 805, 907);
            s.store_mul_sub_lhs(812, 799, 808, 790);
            s.store_mul_ad_lhs(813, A::add_scaled_value_products(s.ad_value(799), (-1.0), s.ad_value(791), s.ad_value(812), 1.0, s.ad_value(810), s.ad_value(808), (-1.0)), 790);
            s.store_mul_ad_lhs(814, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(807), s.ad_value(812), 1.0, s.ad_value(791), s.ad_value(813), 2.0), 1.0, s.ad_value(799), 1.0, A::add(s.ad_value(811), A::square(s.ad_value(810))), s.ad_value(808), (-1.0)), 790);
        }

        s.b[1095] = (s.v[812] > 0.0);
        s.v[1095] = if s.b[1095] { 1.0 } else { 0.0 };

        if s.b[1095] {
            s.store_ln(815, 812);
            s.store_div_from_scalar(789, 1.0, 812);
            s.store_mul(816, 813, 789);
            s.store_add_scaled_square_product_indices(817, 816, (-1.0), 814, 789, 1.0);
        }

        if (!s.b[1095]) {
            s.store_add_offset_lhs_ad_rhs(815, 798, 0.6931471805599, A::ln_scaled_input(s.ad_value(798), -1.0));
            s.store_div_from_scalar(789, 1.0, 912);
            s.store_add(816, 907, 789);
            s.store_mul_neg_lhs(817, 789, 789);
        }

        s.store_sub_ad_lhs(818, A::add_scaled_inputs4(s.ad_value(906), 1.0, s.ad_value(905), (-1.0), s.ad_value(912), 1.0, s.ad_value(815), 2.0), 809);

        s.store_sub_ad_lhs(819, A::scale_offset(s.ad_value(816), 2.0, 1.0), 810);

        s.store_sub_scaled_inputs(820, 817, 2.0, 811, 1.0);

        s.store_add_scaled_product_indices(821, 798, 1.0, 908, 818, 1.0);

        s.store_add_scaled_product_indices(822, 907, 1.0, 908, 819, 1.0);

        s.store_mul(823, 908, 820);

        s.store_add_scaled_product_indices(824, 799, (-1.0), 821, 812, 1.0);

        s.store_add_ad_lhs(825, A::add_scaled_products(s.ad_value(822), s.ad_value(812), 1.0, s.ad_value(821), s.ad_value(813), 1.0), 799);

        s.store_sub_ad_lhs(826, A::add_scaled_products3(s.ad_value(823), s.ad_value(812), 1.0, s.ad_value(822), s.ad_value(813), 2.0, s.ad_value(821), s.ad_value(814), 1.0), 799);

        s.store_add_scaled_square_product_indices(835, 825, 1.0, 824, 826, (-0.5));

        s.store_div_scaled_product3_mixed_iiia(827, 824, 825, 835, -1.0, A::offset(A::square(s.ad_value(835)), 1e-200), 1.0);

        s.store_add(912, 912, 827);

        s.store_mul(798, 907, 912);

        s.b[1096] = ((s.v[905] - s.v[912]) < 80.0);
        s.v[1096] = if s.b[1096] { 1.0 } else { 0.0 };

        if s.b[1096] {
            s.store_exp_sub(789, 905, 912);
        }

        if (!s.b[1096]) {
            s.store_scaled_offset_mul_offset_lhs_ad(789, A::sub(s.ad_value(905), s.ad_value(912)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(905), s.ad_value(912)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(905), s.ad_value(912)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        s.store_mul(799, 880, 789);

        s.store_sub_ad_lhs(800, A::square(s.ad_value(798)), 799);

        s.store_add_scaled_product_indices(801, 799, 1.0, 907, 798, 2.0);

        s.store_add_scaled_product_indices(802, 799, (-1.0), 907, 907, 2.0);

        s.b[1097] = (s.v[800] < (-0.005));
        s.v[1097] = if s.b[1097] { 1.0 } else { 0.0 };

        if s.b[1097] {
            s.store_sqrt_abs_ad(803, s.ad_value(800));
            s.store_div_ad_rhs(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));
            s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);
            s.store_mul_add_ad_lhs(805, s.ad_value(800), A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 789);
            s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);
            s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);
            s.store_mul_div_lhs(810, 801, 800, 790);
            s.store_div_ad_lhs(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);
        }

        s.b[1098] = (s.v[800] > 0.005);
        s.v[1098] = if s.b[1098] { 1.0 } else { 0.0 };

        if ((!s.b[1097]) && s.b[1098]) {
            s.store_sqrt_abs_ad(803, s.ad_value(800));
            s.store_exp_neg_input(806, 803);
            s.store_div_scaled_product_offset_rhs(804, s.ad_value(803), s.ad_value(806), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);
            s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);
            s.store_mul_add_ad_lhs(805, s.ad_value(800), A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 789);
            s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);
            s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);
            s.store_mul_div_lhs(810, 801, 800, 790);
            s.store_div_ad_lhs(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);
        }

        if ((!s.b[1097]) && (!s.b[1098])) {
            s.store_offset_scaled_ad(791, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(804, 800, 791, 2.0);
            s.store_offset_scaled_ad(789, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_mul(805, 801, 789);
            s.store_offset_scaled_ad(790, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(807, 802, 789, 1.0, A::square(s.ad_value(801)), 790, (-1.0));
            s.store_scaled_mul(810, 801, 791, (-0.5));
            s.store_add_scaled_product_value_ad(811, A::mul3_scaled_output(s.ad_value(801), s.ad_value(801), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 2.0, A::scale(s.ad_value(800), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 802, 791, (-0.5));
        }

        s.b[1099] = (s.v[800] > 0.005);
        s.v[1099] = if s.b[1099] { 1.0 } else { 0.0 };

        if s.b[1099] {
            s.store_div_scaled_inputs_mixed_ia(790, 800, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0);
            s.store_mul(808, 790, 806);
            s.store_sub_ad_lhs(809, A::ln(s.ad_value(790)), 803);
        }

        s.b[1100] = (s.v[800] < (-0.005));
        s.v[1100] = if s.b[1100] { 1.0 } else { 0.0 };

        if ((!s.b[1099]) && s.b[1100]) {
            s.store_sin_scaled_input(790, 803, 0.5);
            s.store_div_scaled_inputs_mixed_ia(808, 800, -1.0, A::square(s.ad_value(790)), 1.0);
            s.store_ln(809, 808);
        }

        if ((!s.b[1099]) && (!s.b[1100])) {
            s.store_sub_from_scalar_ad(808, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(809, 808);
        }

        s.b[1101] = (((1.01 * s.v[798]) + s.v[804]) > 0.0);
        s.v[1101] = if s.b[1101] { 1.0 } else { 0.0 };

        if s.b[1101] {
            s.store_add(812, 798, 804);
            s.store_add(813, 907, 805);
            s.copy_ad(814, 807);
        }

        if (!s.b[1101]) {
            s.store_div_from_scalar_sub_ad(790, 1.0, s.ad_value(798), s.ad_value(804));
            s.store_sub(791, 805, 907);
            s.store_mul_sub_lhs(812, 799, 808, 790);
            s.store_mul_ad_lhs(813, A::add_scaled_value_products(s.ad_value(799), (-1.0), s.ad_value(791), s.ad_value(812), 1.0, s.ad_value(810), s.ad_value(808), (-1.0)), 790);
            s.store_mul_ad_lhs(814, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(807), s.ad_value(812), 1.0, s.ad_value(791), s.ad_value(813), 2.0), 1.0, s.ad_value(799), 1.0, A::add(s.ad_value(811), A::square(s.ad_value(810))), s.ad_value(808), (-1.0)), 790);
        }

        s.b[1102] = (s.v[812] > 0.0);
        s.v[1102] = if s.b[1102] { 1.0 } else { 0.0 };

        if s.b[1102] {
            s.store_ln(815, 812);
            s.store_div_from_scalar(789, 1.0, 812);
            s.store_mul(816, 813, 789);
            s.store_add_scaled_square_product_indices(817, 816, (-1.0), 814, 789, 1.0);
        }

        if (!s.b[1102]) {
            s.store_add_offset_lhs_ad_rhs(815, 798, 0.6931471805599, A::ln_scaled_input(s.ad_value(798), -1.0));
            s.store_div_from_scalar(789, 1.0, 912);
            s.store_add(816, 907, 789);
            s.store_mul_neg_lhs(817, 789, 789);
        }

        s.store_sub_ad_lhs(818, A::add_scaled_inputs4(s.ad_value(906), 1.0, s.ad_value(905), (-1.0), s.ad_value(912), 1.0, s.ad_value(815), 2.0), 809);

        s.store_sub_ad_lhs(819, A::scale_offset(s.ad_value(816), 2.0, 1.0), 810);

        s.store_sub_scaled_inputs(820, 817, 2.0, 811, 1.0);

        s.store_add_scaled_product_indices(821, 798, 1.0, 908, 818, 1.0);

        s.store_add_scaled_product_indices(822, 907, 1.0, 908, 819, 1.0);

        s.store_mul(823, 908, 820);

        s.store_add_scaled_product_indices(824, 799, (-1.0), 821, 812, 1.0);

        s.store_add_ad_lhs(825, A::add_scaled_products(s.ad_value(822), s.ad_value(812), 1.0, s.ad_value(821), s.ad_value(813), 1.0), 799);

        s.store_sub_ad_lhs(826, A::add_scaled_products3(s.ad_value(823), s.ad_value(812), 1.0, s.ad_value(822), s.ad_value(813), 2.0, s.ad_value(821), s.ad_value(814), 1.0), 799);

        s.store_add_scaled_square_product_indices(835, 825, 1.0, 824, 826, (-0.5));

        s.store_div_scaled_product3_mixed_iiia(827, 824, 825, 835, -1.0, A::offset(A::square(s.ad_value(835)), 1e-200), 1.0);

        s.store_add(912, 912, 827);

        s.b[1103] = (p.p10 == 1.0);
        s.v[1103] = if s.b[1103] { 1.0 } else { 0.0 };

        s.b[1104] = (((s.v[827]) as f64).abs() > 0.01);
        s.v[1104] = if s.b[1104] { 1.0 } else { 0.0 };

        if (s.b[1103] && s.b[1104]) {
            s.store_mul(798, 907, 912);
        }

        s.b[1105] = ((s.v[905] - s.v[912]) < 80.0);
        s.v[1105] = if s.b[1105] { 1.0 } else { 0.0 };

        if ((s.b[1103] && s.b[1104]) && s.b[1105]) {
            s.store_exp_sub(789, 905, 912);
        }

        if ((s.b[1103] && s.b[1104]) && (!s.b[1105])) {
            s.store_scaled_offset_mul_offset_lhs_ad(789, A::sub(s.ad_value(905), s.ad_value(912)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(905), s.ad_value(912)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(905), s.ad_value(912)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if (s.b[1103] && s.b[1104]) {
            s.store_mul(799, 880, 789);
            s.store_sub_ad_lhs(800, A::square(s.ad_value(798)), 799);
            s.store_add_scaled_product_indices(801, 799, 1.0, 907, 798, 2.0);
            s.store_add_scaled_product_indices(802, 799, (-1.0), 907, 907, 2.0);
        }

        s.b[1106] = (s.v[800] < (-0.005));
        s.v[1106] = if s.b[1106] { 1.0 } else { 0.0 };

        if ((s.b[1103] && s.b[1104]) && s.b[1106]) {
            s.store_sqrt_abs_ad(803, s.ad_value(800));
            s.store_div_ad_rhs(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));
            s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);
            s.store_mul_add_ad_lhs(805, s.ad_value(800), A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 789);
            s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);
            s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);
            s.store_mul_div_lhs(810, 801, 800, 790);
            s.store_div_ad_lhs(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);
        }

    }

    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
    ) {
        s.b[1107] = (s.v[800] > 0.005);
        s.v[1107] = if s.b[1107] { 1.0 } else { 0.0 };

        if (((s.b[1103] && s.b[1104]) && (!s.b[1106])) && s.b[1107]) {
            s.store_sqrt_abs_ad(803, s.ad_value(800));
            s.store_exp_neg_input(806, 803);
            s.store_div_scaled_product_offset_rhs(804, s.ad_value(803), s.ad_value(806), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);
            s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);
            s.store_mul_add_ad_lhs(805, s.ad_value(800), A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 789);
            s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);
            s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);
            s.store_mul_div_lhs(810, 801, 800, 790);
            s.store_div_ad_lhs(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);
        }

        if (((s.b[1103] && s.b[1104]) && (!s.b[1106])) && (!s.b[1107])) {
            s.store_offset_scaled_ad(791, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(804, 800, 791, 2.0);
            s.store_offset_scaled_ad(789, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_mul(805, 801, 789);
            s.store_offset_scaled_ad(790, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(807, 802, 789, 1.0, A::square(s.ad_value(801)), 790, (-1.0));
            s.store_scaled_mul(810, 801, 791, (-0.5));
            s.store_add_scaled_product_value_ad(811, A::mul3_scaled_output(s.ad_value(801), s.ad_value(801), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 2.0, A::scale(s.ad_value(800), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 802, 791, (-0.5));
        }

        s.b[1108] = (s.v[800] > 0.005);
        s.v[1108] = if s.b[1108] { 1.0 } else { 0.0 };

        if ((s.b[1103] && s.b[1104]) && s.b[1108]) {
            s.store_div_scaled_inputs_mixed_ia(790, 800, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0);
            s.store_mul(808, 790, 806);
            s.store_sub_ad_lhs(809, A::ln(s.ad_value(790)), 803);
        }

        s.b[1109] = (s.v[800] < (-0.005));
        s.v[1109] = if s.b[1109] { 1.0 } else { 0.0 };

        if (((s.b[1103] && s.b[1104]) && (!s.b[1108])) && s.b[1109]) {
            s.store_sin_scaled_input(790, 803, 0.5);
            s.store_div_scaled_inputs_mixed_ia(808, 800, -1.0, A::square(s.ad_value(790)), 1.0);
            s.store_ln(809, 808);
        }

        if (((s.b[1103] && s.b[1104]) && (!s.b[1108])) && (!s.b[1109])) {
            s.store_sub_from_scalar_ad(808, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(809, 808);
        }

        s.b[1110] = (((1.01 * s.v[798]) + s.v[804]) > 0.0);
        s.v[1110] = if s.b[1110] { 1.0 } else { 0.0 };

        if ((s.b[1103] && s.b[1104]) && s.b[1110]) {
            s.store_add(812, 798, 804);
            s.store_add(813, 907, 805);
            s.copy_ad(814, 807);
        }

        if ((s.b[1103] && s.b[1104]) && (!s.b[1110])) {
            s.store_div_from_scalar_sub_ad(790, 1.0, s.ad_value(798), s.ad_value(804));
            s.store_sub(791, 805, 907);
            s.store_mul_sub_lhs(812, 799, 808, 790);
            s.store_mul_ad_lhs(813, A::add_scaled_value_products(s.ad_value(799), (-1.0), s.ad_value(791), s.ad_value(812), 1.0, s.ad_value(810), s.ad_value(808), (-1.0)), 790);
            s.store_mul_ad_lhs(814, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(807), s.ad_value(812), 1.0, s.ad_value(791), s.ad_value(813), 2.0), 1.0, s.ad_value(799), 1.0, A::add(s.ad_value(811), A::square(s.ad_value(810))), s.ad_value(808), (-1.0)), 790);
        }

        s.b[1111] = (s.v[812] > 0.0);
        s.v[1111] = if s.b[1111] { 1.0 } else { 0.0 };

        if ((s.b[1103] && s.b[1104]) && s.b[1111]) {
            s.store_ln(815, 812);
            s.store_div_from_scalar(789, 1.0, 812);
            s.store_mul(816, 813, 789);
            s.store_add_scaled_square_product_indices(817, 816, (-1.0), 814, 789, 1.0);
        }

        if ((s.b[1103] && s.b[1104]) && (!s.b[1111])) {
            s.store_add_offset_lhs_ad_rhs(815, 798, 0.6931471805599, A::ln_scaled_input(s.ad_value(798), -1.0));
            s.store_div_from_scalar(789, 1.0, 912);
            s.store_add(816, 907, 789);
            s.store_mul_neg_lhs(817, 789, 789);
        }

        if (s.b[1103] && s.b[1104]) {
            s.store_sub_ad_lhs(818, A::add_scaled_inputs4(s.ad_value(906), 1.0, s.ad_value(905), (-1.0), s.ad_value(912), 1.0, s.ad_value(815), 2.0), 809);
            s.store_sub_ad_lhs(819, A::scale_offset(s.ad_value(816), 2.0, 1.0), 810);
            s.store_sub_scaled_inputs(820, 817, 2.0, 811, 1.0);
            s.store_add_scaled_product_indices(821, 798, 1.0, 908, 818, 1.0);
            s.store_add_scaled_product_indices(822, 907, 1.0, 908, 819, 1.0);
            s.store_mul(823, 908, 820);
            s.store_add_scaled_product_indices(824, 799, (-1.0), 821, 812, 1.0);
            s.store_add_ad_lhs(825, A::add_scaled_products(s.ad_value(822), s.ad_value(812), 1.0, s.ad_value(821), s.ad_value(813), 1.0), 799);
            s.store_sub_ad_lhs(826, A::add_scaled_products3(s.ad_value(823), s.ad_value(812), 1.0, s.ad_value(822), s.ad_value(813), 2.0, s.ad_value(821), s.ad_value(814), 1.0), 799);
            s.store_add_scaled_square_product_indices(835, 825, 1.0, 824, 826, (-0.5));
            s.store_div_scaled_product3_mixed_iiia(827, 824, 825, 835, -1.0, A::offset(A::square(s.ad_value(835)), 1e-200), 1.0);
            s.store_add(912, 912, 827);
        }

        s.store_mul(914, 907, 912);

        s.b[1112] = ((s.v[905] - s.v[912]) < 80.0);
        s.v[1112] = if s.b[1112] { 1.0 } else { 0.0 };

        if s.b[1112] {
            s.store_exp_sub(789, 905, 912);
        }

        if (!s.b[1112]) {
            s.store_scaled_offset_mul_offset_lhs_ad(789, A::sub(s.ad_value(905), s.ad_value(912)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(905), s.ad_value(912)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(905), s.ad_value(912)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        s.store_mul(918, 880, 789);

        s.store_sub_ad_lhs(917, A::square(s.ad_value(914)), 918);

        s.b[1113] = (s.v[918] <= 0.0);
        s.v[1113] = if s.b[1113] { 1.0 } else { 0.0 };

        if s.b[1113] {
            s.store_scalar(913, 1e-80);
            s.store_sub(915, 913, 914);
            s.store_div(916, 915, 908);
        }

        s.b[1114] = (s.v[917] < (-0.005));
        s.v[1114] = if s.b[1114] { 1.0 } else { 0.0 };

        if ((!s.b[1113]) && s.b[1114]) {
            s.store_sqrt_abs_ad(803, s.ad_value(917));
            s.store_div_ad_rhs(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));
        }

        s.b[1115] = (s.v[917] > 0.005);
        s.v[1115] = if s.b[1115] { 1.0 } else { 0.0 };

        if (((!s.b[1113]) && (!s.b[1114])) && s.b[1115]) {
            s.store_sqrt_abs_ad(803, s.ad_value(917));
            s.store_exp_neg_input(806, 803);
            s.store_div_scaled_product_offset_rhs(804, s.ad_value(803), s.ad_value(806), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);
        }

        if (((!s.b[1113]) && (!s.b[1114])) && (!s.b[1115])) {
            s.store_offset_ad(804, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(917), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(917), 1.0, A::scale(s.ad_value(917), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);
        }

        s.b[1116] = (((1.01 * s.v[914]) + s.v[804]) > 0.0);
        s.v[1116] = if s.b[1116] { 1.0 } else { 0.0 };

        if ((!s.b[1113]) && s.b[1116]) {
            s.store_add(789, 914, 804);
        }

        s.b[1117] = ((s.v[918] * s.v[914]) < (((0.9 * s.v[914]) * s.v[914]) * s.v[789]));
        s.v[1117] = if s.b[1117] { 1.0 } else { 0.0 };

        if (((!s.b[1113]) && s.b[1116]) && s.b[1117]) {
            s.store_offset_div(913, 918, 789, 1e-80);
            s.store_sub(915, 913, 914);
            s.store_div(916, 915, 908);
        }

        s.b[1118] = (s.v[917] > 0.005);
        s.v[1118] = if s.b[1118] { 1.0 } else { 0.0 };

        if ((((!s.b[1113]) && s.b[1116]) && (!s.b[1117])) && s.b[1118]) {
            s.store_sub_ad_lhs(790, A::ln(A::div_scaled_inputs(s.ad_value(917), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0)), 803);
        }

        s.b[1119] = (s.v[917] < (-0.005));
        s.v[1119] = if s.b[1119] { 1.0 } else { 0.0 };

        if (((((!s.b[1113]) && s.b[1116]) && (!s.b[1117])) && (!s.b[1118])) && s.b[1119]) {
            s.store_sin_scaled_input(791, 803, 0.5);
            s.store_ln_div_scaled_input_square_denominator(790, 917, -1.0, 791, 1.0);
        }

        if (((((!s.b[1113]) && s.b[1116]) && (!s.b[1117])) && (!s.b[1118])) && (!s.b[1119])) {
            s.store_ln_ad(790, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(917), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(917), 1.0, A::scale(s.ad_value(917), 0.0396825396825397), 0.05), 0.3333333333333)));
        }

        if (((!s.b[1113]) && s.b[1116]) && (!s.b[1117])) {
            s.store_sub_ad_lhs(916, A::add_scaled_inputs4(s.ad_value(906), 1.0, s.ad_value(905), (-1.0), s.ad_value(912), 1.0, A::ln(s.ad_value(789)), 2.0), 790);
            s.store_mul(915, 908, 916);
            s.store_add(913, 914, 915);
        }

        s.b[1120] = (s.v[917] > 0.005);
        s.v[1120] = if s.b[1120] { 1.0 } else { 0.0 };

        s.b[1121] = (((s.v[912] - s.v[905]) - s.v[803]) < 80.0);
        s.v[1121] = if s.b[1121] { 1.0 } else { 0.0 };

        if ((((!s.b[1113]) && (!s.b[1116])) && s.b[1120]) && s.b[1121]) {
            s.store_exp_ad(791, A::add_scaled_inputs3(s.ad_value(912), 1.0, s.ad_value(905), (-1.0), s.ad_value(803), -1.0));
        }

        if ((((!s.b[1113]) && (!s.b[1116])) && s.b[1120]) && (!s.b[1121])) {
            let assign15360_ad_e15150: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(912), 1.0, s.ad_value(905), (-1.0), s.ad_value(803), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(912), 1.0, s.ad_value(905), (-1.0), s.ad_value(803), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(912), 1.0, s.ad_value(905), (-1.0), s.ad_value(803), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(791, assign15360_ad_e15150, 1.0, 5.54062e34);
        }

        if (((!s.b[1113]) && (!s.b[1116])) && s.b[1120]) {
            s.store_div(790, 791, 880);
            s.store_div_scaled_product_denominator_ad(789, 917, 790, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0);
        }

        s.b[1122] = (s.v[917] < (-0.005));
        s.v[1122] = if s.b[1122] { 1.0 } else { 0.0 };

        if ((((!s.b[1113]) && (!s.b[1116])) && (!s.b[1120])) && s.b[1122]) {
            s.store_sin_scaled_input(790, 803, 0.5);
            s.store_div_scaled_value_by_product(789, s.ad_value(917), -1.0, A::square(s.ad_value(790)), s.ad_value(918), 1.0);
        }

        if ((((!s.b[1113]) && (!s.b[1116])) && (!s.b[1120])) && (!s.b[1122])) {
            s.store_div_ad_lhs(789, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(917), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(917), 1.0, A::scale(s.ad_value(917), 0.0396825396825397), 0.05), 0.3333333333333)), 918);
        }

        if ((!s.b[1113]) && (!s.b[1116])) {
            s.store_offset_div_scaled_inputs2(913, s.ad_value(914), 1.0, s.ad_value(804), (-1.0), A::sub_from_scalar(1.0, s.ad_value(789)), 1.0, 1e-80);
            s.store_sub(915, 913, 914);
            s.store_div(916, 915, 908);
        }

        s.b[1123] = ((s.v[906] - s.v[916]) < 80.0);
        s.v[1123] = if s.b[1123] { 1.0 } else { 0.0 };

        if s.b[1123] {
            s.store_exp_sub(789, 906, 916);
        }

        if (!s.b[1123]) {
            s.store_scaled_offset_mul_offset_lhs_ad(789, A::sub(s.ad_value(906), s.ad_value(916)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(906), s.ad_value(916)), (-80.0)), 0.5, A::scale_offset(A::sub(s.ad_value(906), s.ad_value(916)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        s.store_mul(919, 880, 789);

        s.v[922] = 0.0;

        s.v[923] = 0.0;

        s.v[920] = 0.0;

        s.v[921] = 0.0;

        s.v[924] = 0.0;

        s.v[925] = 0.0;

        s.b[1124] = (s.v[913] > 1e-6);
        s.v[1124] = if s.b[1124] { 1.0 } else { 0.0 };

        if s.b[1124] {
            s.store_mul(920, 918, 881);
            s.store_mul(921, 919, 882);
            s.store_add_scaled_inputs(922, 920, 1.0, 914, 2.0);
            s.store_add_scaled_inputs(923, 921, 1.0, 915, 2.0);
            s.store_add_scaled_inputs3_indices(924, 913, 2.0, 920, 1.0, 921, 1.0);
        }

        s.b[1125] = (((s.v[917]) as f64).abs() > 0.005);
        s.v[1125] = if s.b[1125] { 1.0 } else { 0.0 };

        if (s.b[1124] && s.b[1125]) {
            s.store_add_scaled_products3(2, s.ad_value(922), s.ad_value(923), 1.0, A::offset(s.ad_value(912), 2.0), s.ad_value(923), 2.0, A::offset(s.ad_value(916), 2.0), s.ad_value(922), 2.0);
            s.store_div_scaled_product_by_product(925, s.ad_value(917), s.ad_value(924), (-4.0), s.ad_value(913), s.ad_value(2), 1.0);
        }

        if (s.b[1124] && (!s.b[1125])) {
            s.store_offset_scaled_ad(2, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(917), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(917), 1.0, A::scale(s.ad_value(917), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_add_scaled_products3(3, s.ad_value(922), s.ad_value(918), 1.0, s.ad_value(923), s.ad_value(919), 1.0, A::mul3(s.ad_value(922), s.ad_value(923), s.ad_value(913)), A::offset(A::mul(s.ad_value(913), s.ad_value(2)), 1.0), 1.0);
            s.store_div_scaled_product3_by_product(925, s.ad_value(918), s.ad_value(919), s.ad_value(924), 1.0, s.ad_value(913), s.ad_value(3), 1.0);
        }

        s.store_ln(926, 913);

        s.b[1126] = ((s.v[914] / 2.0) < 80.0);
        s.v[1126] = if s.b[1126] { 1.0 } else { 0.0 };

        if s.b[1126] {
            s.store_ln_one_plus_exp_scaled_input(2, 914, 0.5);
        }

        if (!s.b[1126]) {
            s.store_scale(2, 914, 0.5);
        }

        s.store_scale(927, 2, 2.0);

        s.b[1127] = ((s.v[915] / 2.0) < 80.0);
        s.v[1127] = if s.b[1127] { 1.0 } else { 0.0 };

        if s.b[1127] {
            s.store_ln_one_plus_exp_scaled_input(3, 915, 0.5);
        }

        if (!s.b[1127]) {
            s.store_scale(3, 915, 0.5);
        }

        s.store_scale(928, 3, 2.0);

        s.store_sub(929, 928, 915);

        s.store_sub(930, 927, 914);

        s.store_add_scaled_products_indices(931, 266, 927, 1.0, 267, 929, 1.0);

        s.store_add_scaled_products_indices(932, 266, 928, 1.0, 267, 930, 1.0);

        s.store_div_add_scaled_inputs_rhs_indices(0, 913, 927, 1.0, 928, 1.0);

        s.store_mul(933, 927, 0);

        s.store_mul(934, 928, 0);

        s.store_mul_ad_product_rhs(935, 927, s.ad_value(187), A::exp(A::mul(s.ad_value(40), s.ad_value(291))));

        s.store_mul_ad_product_rhs(936, 928, s.ad_value(188), A::exp(A::mul(s.ad_value(40), s.ad_value(291))));

    }

    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
    ) {
        s.store_mul_add_scaled_product_rhs(2, 50, s.ad_value(929), 1.0, s.ad_value(51), s.ad_value(930), 1.0);

        s.store_scaled_add_offset_sqrt_square_offset(3, 2, 1.0, 1.0, 0.01, 0.5);

        s.store_scaled_add_ad(4, A::scale_offset(s.ad_value(2), 0.2, 1.0), A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(2), 0.2, 1.0), A::scale_offset(s.ad_value(2), 0.2, 1.0)), 0.01)), 0.5);

        s.store_div(937, 3, 4);

        s.store_mul_ad_product_rhs(938, 33, A::add_scaled_product(A::offset(A::mul(s.ad_value(41), s.ad_value(929)), 1.0), 1.0, s.ad_value(42), s.ad_value(930), 1.0), A::exp(A::mul_scaled_lhs(s.ad_value(44), -1.0, A::ln(A::add_scaled_product(A::offset(A::mul(s.ad_value(933), s.ad_value(264)), 1.0), 1.0, s.ad_value(934), s.ad_value(265), 1.0)))));

        s.b[1128] = (s.v[56] == 0.0);
        s.v[1128] = if s.b[1128] { 1.0 } else { 0.0 };

        if s.b[1128] {
            s.store_scalar(4, 1.0);
        }

        s.b[1129] = (s.v[56] < 0.0);
        s.v[1129] = if s.b[1129] { 1.0 } else { 0.0 };

        if ((!s.b[1128]) && s.b[1129]) {
            s.store_mul_exp_ad_rhs(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(913), 1e-12))));
            s.store_sub_from_scalar(4, 1.0, 2);
        }

        if ((!s.b[1128]) && (!s.b[1129])) {
            s.store_mul_exp_ad_rhs(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(913), 1e-12))));
            s.store_div_from_scalar_offset_input(4, 1.0, 2, 1.0);
        }

        s.store_mul_ad_affine_product_rhs(939, 268, s.ad_value(894), A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(875))), A::sqrt(A::offset(A::mul_sub_from_scalar_lhs(1.0, A::mul(s.ad_value(58), s.ad_value(875)), A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(875)))), 0.01))), 0.5, 0.0);

        s.store_mul_add_scaled_product_rhs(940, 939, s.ad_value(54), 1.0, s.ad_value(913), s.ad_value(4), 1.0);

        s.store_add_scaled_inputs_product_first_ad(941, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(931)), 1e-6)))), 1.0), 1.0, 938, 1.0, 38, 940, 1.0);

        s.store_add_scaled_inputs_product_first_ad(942, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(932)), 1e-6)))), 1.0), 1.0, 938, 1.0, 39, 940, 1.0);

        s.store_div_scaled_product_mixed_iaa(943, 937, A::add(s.ad_value(935), s.ad_value(936)), 1.0, A::add(A::div(s.ad_value(935), s.ad_value(941)), A::div(s.ad_value(936), s.ad_value(942))), 1.0);

        s.b[1130] = (((s.v[910]) as f64).abs() > 0.007);
        s.v[1130] = if s.b[1130] { 1.0 } else { 0.0 };

        s.b[1131] = (s.v[910] > 0.0);
        s.v[1131] = if s.b[1131] { 1.0 } else { 0.0 };

        if (s.b[1130] && s.b[1131]) {
            s.store_exp_neg_input(0, 910);
            s.store_div_ad_rhs(944, 910, A::sub_from_scalar(1.0, s.ad_value(0)));
            s.store_mul(945, 0, 944);
            s.store_add_offset_ad_lhs(946, A::ln(A::div(s.ad_value(880), A::mul(s.ad_value(913), s.ad_value(944)))), (-0.6931471805599), 883);
        }

        if (s.b[1130] && (!s.b[1131])) {
            s.store_exp(0, 910);
            s.store_div_scaled_value_offset_denominator(945, s.ad_value(910), 1.0, s.ad_value(0), (-1.0), 1.0);
            s.store_mul(944, 0, 945);
            s.store_add_offset_ad_lhs(946, A::ln(A::div(s.ad_value(880), A::mul(s.ad_value(913), s.ad_value(945)))), (-0.6931471805599), 884);
        }

        if s.b[1130] {
            s.store_div_scaled_inputs_mixed_ia(947, 910, -1.0, A::mul(s.ad_value(909), A::add_scaled_sub_value_product(1.0, s.ad_value(944), 1.0, s.ad_value(910), s.ad_value(882), (-1.0))), 1.0);
            s.store_div_ad_rhs(948, 910, A::mul(s.ad_value(909), A::add_scaled_sub_value_product(1.0, s.ad_value(945), 1.0, s.ad_value(910), s.ad_value(881), 1.0)));
            s.store_div_add_scaled_inputs_rhs_ad(949, 910, A::div_scaled_offset_numerator(A::mul(s.ad_value(945), s.ad_value(882)), 1.0, 0.5, s.ad_value(948), 1.0), 1.0, A::div_scaled_offset_numerator(A::mul(s.ad_value(944), s.ad_value(881)), 1.0, 0.5, s.ad_value(947), 1.0), -1.0);
        }

        if (!s.b[1130]) {
            s.store_scale(0, 911, (0.5 * 0.1666666666667));
            s.store_scale(2, 910, 0.5);
            s.store_add_offset_lhs(944, 2, 1.0, 0);
            s.store_add_ad_lhs(945, A::sub_from_scalar(1.0, s.ad_value(2)), 0);
            s.store_scale(3, 2, 0.1666666666667);
            s.store_div_from_scalar_mul_ad(947, 1.0, s.ad_value(909), A::add(A::offset(s.ad_value(882), 0.5), s.ad_value(3)));
            s.store_div_from_scalar_mul_ad(948, 1.0, s.ad_value(909), A::sub(A::offset(s.ad_value(881), 0.5), s.ad_value(3)));
            s.store_add_scaled_inputs3_offset_mixed_aii(946, A::ln(A::div(s.ad_value(880), A::mul_sub_from_scalar_rhs(s.ad_value(913), 1.0, A::scale(s.ad_value(0), 0.5)))), 1.0, 883, 0.5, 884, 0.5, (-0.6931471805599));
            s.store_div_from_scalar_ad(949, (-12.0), A::add_scaled_inputs4_offset(s.ad_value(909), ((-1.0) * 3.0), A::div_scaled_inputs(s.ad_value(909), 12.0, A::mul(s.ad_value(907), s.ad_value(908)), 1.0), 1.0, A::mul3(s.ad_value(909), A::sub(s.ad_value(881), s.ad_value(882)), s.ad_value(910)), 1.0, A::mul_sub_from_scalar_lhs_scaled_output(0.2, A::scale(s.ad_value(909), 0.25), s.ad_value(911), 0.3333333333333), 1.0, 4.0));
        }

        s.store_div_from_scalar(950, 1.0, 949);

        s.b[1132] = (s.v[913] > 1e-6);
        s.v[1132] = if s.b[1132] { 1.0 } else { 0.0 };

        if s.b[1132] {
            s.store_div_scaled_value_offset_denominator(951, s.ad_value(927), 100.0, s.ad_value(927), 100.0, 1.0);
        }

        s.b[1133] = (s.v[61] < 0.0);
        s.v[1133] = if s.b[1133] { 1.0 } else { 0.0 };

        if (s.b[1132] && s.b[1133]) {
            s.store_div_from_scalar_sub_from_scalar_ad(952, 1.0, 1.0, A::mul(s.ad_value(61), s.ad_value(951)));
        }

        if (s.b[1132] && (!s.b[1133])) {
            s.store_offset_mul(952, 61, 951, 1.0);
        }

        if s.b[1132] {
            s.store_div_scaled_value_offset_denominator(953, s.ad_value(928), 100.0, s.ad_value(928), 100.0, 1.0);
        }

        s.b[1134] = (s.v[62] < 0.0);
        s.v[1134] = if s.b[1134] { 1.0 } else { 0.0 };

        if (s.b[1132] && s.b[1134]) {
            s.store_div_from_scalar_sub_from_scalar_ad(954, 1.0, 1.0, A::mul(s.ad_value(62), s.ad_value(953)));
        }

        if (s.b[1132] && (!s.b[1134])) {
            s.store_offset_mul(954, 62, 953, 1.0);
        }

        if s.b[1132] {
            s.store_sub_ad(955, A::div_scaled_product_by_product(s.ad_value(925), s.ad_value(924), 1.0, s.ad_value(922), s.ad_value(923), 1.0), A::div_scaled_inputs2(A::div(s.ad_value(918), s.ad_value(922)), 1.0, A::div(s.ad_value(919), s.ad_value(923)), 1.0, s.ad_value(913), 1.0));
            s.store_div_scaled_product_offset_denominator(956, s.ad_value(955), s.ad_value(913), 1.0, s.ad_value(955), 1.0, 1.0);
            s.store_sub(2, 949, 956);
            s.store_div_scaled_add_product(957, s.ad_value(913), 1.0, s.ad_value(949), s.ad_value(946), 1.0, s.ad_value(2), 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(957, 957, 957, 1e-6, 0.5);
            s.store_scaled_mul_ad(958, A::div(s.ad_value(871), s.ad_value(943)), A::add(s.ad_value(952), s.ad_value(954)), 0.5);
            s.store_sub_from_scalar_div_indices(959, 1.0, 913, 956);
            s.store_offset(960, 946, 1.0);
            s.store_mul_sub_ad_lhs(961, A::offset(A::mul(A::sub_scaled_inputs(s.ad_value(956), 2.0, s.ad_value(913), 1.0), s.ad_value(950)), (-2.0)), s.ad_value(946), 957);
        }

        s.b[1135] = (s.v[958] > 1e-14);
        s.v[1135] = if s.b[1135] { 1.0 } else { 0.0 };

        if (s.b[1132] && s.b[1135]) {
            s.store_div_from_scalar_square_ad(962, 2.0, s.ad_value(958));
            s.store_mul(963, 962, 959);
            s.store_add(964, 962, 961);
            s.store_mul(965, 962, 960);
            s.store_sqrt_offset_ad(966, A::add(A::square(s.ad_value(963)), A::mul3_scaled_output(s.ad_value(962), s.ad_value(962), s.ad_value(962), 0.148148148148)), 1e-20);
            s.store_sqrt_offset_ad(967, A::add(A::square(s.ad_value(965)), A::mul3_scaled_output(s.ad_value(964), s.ad_value(964), s.ad_value(964), 0.148148148148)), 1e-20);
            s.store_sub_ad(968, A::exp_scaled_input(A::ln_scaled_input(A::add(s.ad_value(966), s.ad_value(963)), 0.5), 0.3333333333333), A::exp_scaled_input(A::ln_scaled_input(A::sub(s.ad_value(966), s.ad_value(963)), 0.5), 0.3333333333333));
            s.store_sub_ad(969, A::exp_scaled_input(A::ln_scaled_input(A::add(s.ad_value(967), s.ad_value(965)), 0.5), 0.3333333333333), A::exp_scaled_input(A::ln_scaled_input(A::sub(s.ad_value(967), s.ad_value(965)), 0.5), 0.3333333333333));
        }

        if (s.b[1132] && (!s.b[1135])) {
            s.copy_ad(968, 959);
            s.copy_ad(969, 960);
        }

        if s.b[1132] {
            s.store_square(4, 2);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(970, 968, (0.94 * 0.5), 969, (0.94 * 0.5), A::add_scaled_product(s.ad_value(4), 10.0, A::sub(s.ad_value(968), s.ad_value(969)), A::sub(s.ad_value(968), s.ad_value(969)), 1.0), (0.94 * 0.5));
            s.store_add_scaled_product_indices(971, 913, 1.0, 956, 970, 1.0);
            s.store_mul_sub_rhs(972, 949, 970, 946);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(973, 971, 0.5, 972, 0.5, A::add_scaled_product(s.ad_value(4), 36.0, A::sub(s.ad_value(971), s.ad_value(972)), A::sub(s.ad_value(971), s.ad_value(972)), 1.0), 0.5);
        }

        if (!s.b[1132]) {
            s.copy_ad(956, 949);
            s.store_scaled_offset(970, 946, 1.0, 0.94);
            s.store_add_scaled_product_right_ad(973, 913, 0.5, 949, A::sub_scaled_inputs(s.ad_value(970), 1.0, s.ad_value(946), 0.5), 1.0);
        }

        s.b[1136] = ((s.v[973] - 0.5) < 80.0);
        s.v[1136] = if s.b[1136] { 1.0 } else { 0.0 };

        if s.b[1136] {
            s.store_ln_one_plus_exp_ad(2, A::offset(s.ad_value(973), (-0.5)));
        }

        if (!s.b[1136]) {
            s.store_offset(2, 973, (-0.5));
        }

        s.store_offset(3, 2, 0.5);

        s.store_add_ad_rhs(4, 970, A::ln(A::div(s.ad_value(913), s.ad_value(3))));

        s.b[1137] = ((s.v[4] - 6.0) < 80.0);
        s.v[1137] = if s.b[1137] { 1.0 } else { 0.0 };

        if s.b[1137] {
            s.store_ln_one_plus_exp_ad(2, A::offset(s.ad_value(4), (-6.0)));
        }

        if (!s.b[1137]) {
            s.store_offset(2, 4, (-6.0));
        }

        s.store_offset(4, 2, 6.0);

        s.b[1138] = ((s.v[221] - s.v[4]) < 80.0);
        s.v[1138] = if s.b[1138] { 1.0 } else { 0.0 };

        if s.b[1138] {
            s.store_ln_one_plus_exp_ad(2, A::sub(s.ad_value(221), s.ad_value(4)));
        }

        if (!s.b[1138]) {
            s.store_sub(2, 221, 4);
        }

        s.store_sub(974, 221, 2);

        s.store_div(2, 335, 974);

        s.store_square(3, 2);

        s.store_square(4, 3);

        s.store_square(5, 4);

        s.store_exp_scaled_input_ad(0, A::ln(A::offset(A::mul(s.ad_value(872), s.ad_value(4)), 1.0)), 2.666666666667);

        s.store_mul_ad_rhs(975, 335, A::exp_scaled_input(A::ln(A::add(s.ad_value(0), A::square(s.ad_value(5)))), (-0.0625)));

        s.store_div_from_scalar_offset_input(789, 1.0, 907, 1.0);

        s.store_div_from_scalar_offset_input(790, 1.0, 908, 1.0);

        s.store_offset_add_ad(792, A::ln(A::div_scaled_product(A::add_scaled_product(s.ad_value(907), 1.0, s.ad_value(908), s.ad_value(790), 1.0), s.ad_value(879), 1.0, s.ad_value(880), 1.0)), s.ad_value(975), 3.0);

        s.store_offset_add_ad(793, A::ln(A::div_scaled_product(A::add_scaled_product(s.ad_value(908), 1.0, s.ad_value(907), s.ad_value(789), 1.0), s.ad_value(879), 1.0, s.ad_value(880), 1.0)), s.ad_value(975), 3.0);

        s.b[1139] = (((s.v[792] - s.v[883]) * 0.3333333333333) < 80.0);
        s.v[1139] = if s.b[1139] { 1.0 } else { 0.0 };

        if s.b[1139] {
            s.store_ln_one_plus_exp_ad(791, A::sub_scaled_inputs(s.ad_value(792), 0.3333333333333, s.ad_value(883), 0.3333333333333));
        }

        if (!s.b[1139]) {
            s.store_scaled_sub(791, 792, 883, 0.3333333333333);
        }

        s.store_sub_scaled_inputs(796, 792, 1.0, 791, 3.0);

        s.b[1140] = (((s.v[793] - s.v[884]) * 0.3333333333333) < 80.0);
        s.v[1140] = if s.b[1140] { 1.0 } else { 0.0 };

        if s.b[1140] {
            s.store_ln_one_plus_exp_ad(791, A::sub_scaled_inputs(s.ad_value(793), 0.3333333333333, s.ad_value(884), 0.3333333333333));
        }

        if (!s.b[1140]) {
            s.store_scaled_sub(791, 793, 884, 0.3333333333333);
        }

        s.store_sub_scaled_inputs(797, 793, 1.0, 791, 3.0);

        s.store_mul_add_scaled_product_rhs(794, 789, s.ad_value(797), 1.0, s.ad_value(907), s.ad_value(905), 1.0);

        s.store_mul_add_scaled_product_rhs(795, 790, s.ad_value(796), 1.0, s.ad_value(908), s.ad_value(906), 1.0);

        s.b[1141] = (((s.v[792] - s.v[794]) * 0.3333333333333) < 80.0);
        s.v[1141] = if s.b[1141] { 1.0 } else { 0.0 };

        if s.b[1141] {
            s.store_ln_one_plus_exp_ad(791, A::sub_scaled_inputs(s.ad_value(792), 0.3333333333333, s.ad_value(794), 0.3333333333333));
        }

        if (!s.b[1141]) {
            s.store_scaled_sub(791, 792, 794, 0.3333333333333);
        }

        s.store_sub_scaled_inputs(796, 792, 1.0, 791, 3.0);

        s.b[1142] = (((s.v[793] - s.v[795]) * 0.3333333333333) < 80.0);
        s.v[1142] = if s.b[1142] { 1.0 } else { 0.0 };

        if s.b[1142] {
            s.store_ln_one_plus_exp_ad(791, A::sub_scaled_inputs(s.ad_value(793), 0.3333333333333, s.ad_value(795), 0.3333333333333));
        }

        if (!s.b[1142]) {
            s.store_scaled_sub(791, 793, 795, 0.3333333333333);
        }

        s.store_sub_scaled_inputs(797, 793, 1.0, 791, 3.0);

        s.store_sub(976, 905, 796);

        s.store_sub(977, 906, 797);

        s.v[803] = 0.0;

        s.v[806] = 0.0;

        s.store_mul(798, 907, 976);

        s.b[1143] = (((s.v[905] - s.v[976]) - s.v[975]) < 80.0);
        s.v[1143] = if s.b[1143] { 1.0 } else { 0.0 };

        if s.b[1143] {
            s.store_exp_ad(789, A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0));
        }

        if (!s.b[1143]) {
            let assign17110_ad_e16881: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(789, assign17110_ad_e16881, 1.0, 5.54062e34);
        }

        s.store_mul(799, 880, 789);

        s.store_sub_ad_lhs(800, A::square(s.ad_value(798)), 799);

        s.store_add_scaled_product_indices(801, 799, 1.0, 907, 798, 2.0);

        s.store_add_scaled_product_indices(802, 799, (-1.0), 907, 907, 2.0);

        s.b[1144] = (s.v[800] < (-0.005));
        s.v[1144] = if s.b[1144] { 1.0 } else { 0.0 };

        if s.b[1144] {
            s.store_sqrt_abs_ad(803, s.ad_value(800));
            s.store_div_ad_rhs(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));
            s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);
            s.store_mul_add_ad_lhs(805, s.ad_value(800), A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 789);
            s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);
            s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);
            s.store_mul_div_lhs(810, 801, 800, 790);
            s.store_div_ad_lhs(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);
        }

        s.b[1145] = (s.v[800] > 0.005);
        s.v[1145] = if s.b[1145] { 1.0 } else { 0.0 };

        if ((!s.b[1144]) && s.b[1145]) {
            s.store_sqrt_abs_ad(803, s.ad_value(800));
            s.store_exp_neg_input(806, 803);
            s.store_div_scaled_product_offset_rhs(804, s.ad_value(803), s.ad_value(806), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);
            s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);
            s.store_mul_add_ad_lhs(805, s.ad_value(800), A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 789);
            s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);
            s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);
            s.store_mul_div_lhs(810, 801, 800, 790);
            s.store_div_ad_lhs(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);
        }

        if ((!s.b[1144]) && (!s.b[1145])) {
            s.store_offset_scaled_ad(791, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(804, 800, 791, 2.0);
            s.store_offset_scaled_ad(789, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
        }

    }

    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
    ) {
        if ((!s.b[1144]) && (!s.b[1145])) {
            s.store_mul(805, 801, 789);
            s.store_offset_scaled_ad(790, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(807, 802, 789, 1.0, A::square(s.ad_value(801)), 790, (-1.0));
            s.store_scaled_mul(810, 801, 791, (-0.5));
            s.store_add_scaled_product_value_ad(811, A::mul3_scaled_output(s.ad_value(801), s.ad_value(801), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 2.0, A::scale(s.ad_value(800), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 802, 791, (-0.5));
        }

        s.b[1146] = (s.v[800] > 0.005);
        s.v[1146] = if s.b[1146] { 1.0 } else { 0.0 };

        if s.b[1146] {
            s.store_div_scaled_inputs_mixed_ia(790, 800, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0);
            s.store_mul(808, 790, 806);
            s.store_sub_ad_lhs(809, A::ln(s.ad_value(790)), 803);
        }

        s.b[1147] = (s.v[800] < (-0.005));
        s.v[1147] = if s.b[1147] { 1.0 } else { 0.0 };

        if ((!s.b[1146]) && s.b[1147]) {
            s.store_sin_scaled_input(790, 803, 0.5);
            s.store_div_scaled_inputs_mixed_ia(808, 800, -1.0, A::square(s.ad_value(790)), 1.0);
            s.store_ln(809, 808);
        }

        if ((!s.b[1146]) && (!s.b[1147])) {
            s.store_sub_from_scalar_ad(808, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(809, 808);
        }

        s.b[1148] = (((1.01 * s.v[798]) + s.v[804]) > 0.0);
        s.v[1148] = if s.b[1148] { 1.0 } else { 0.0 };

        if s.b[1148] {
            s.store_add(812, 798, 804);
            s.store_add(813, 907, 805);
            s.copy_ad(814, 807);
        }

        if (!s.b[1148]) {
            s.store_div_from_scalar_sub_ad(790, 1.0, s.ad_value(798), s.ad_value(804));
            s.store_sub(791, 805, 907);
            s.store_mul_sub_lhs(812, 799, 808, 790);
            s.store_mul_ad_lhs(813, A::add_scaled_value_products(s.ad_value(799), (-1.0), s.ad_value(791), s.ad_value(812), 1.0, s.ad_value(810), s.ad_value(808), (-1.0)), 790);
            s.store_mul_ad_lhs(814, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(807), s.ad_value(812), 1.0, s.ad_value(791), s.ad_value(813), 2.0), 1.0, s.ad_value(799), 1.0, A::add(s.ad_value(811), A::square(s.ad_value(810))), s.ad_value(808), (-1.0)), 790);
        }

        s.b[1149] = (s.v[812] > 0.0);
        s.v[1149] = if s.b[1149] { 1.0 } else { 0.0 };

        if s.b[1149] {
            s.store_ln(815, 812);
            s.store_div_from_scalar(789, 1.0, 812);
            s.store_mul(816, 813, 789);
            s.store_add_scaled_square_product_indices(817, 816, (-1.0), 814, 789, 1.0);
        }

        if (!s.b[1149]) {
            s.store_add_offset_lhs_ad_rhs(815, 798, 0.6931471805599, A::ln_scaled_input(s.ad_value(798), -1.0));
            s.store_div_from_scalar(789, 1.0, 976);
            s.store_add(816, 907, 789);
            s.store_mul_neg_lhs(817, 789, 789);
        }

        s.store_sub_ad_lhs(818, A::add_scaled_inputs4(s.ad_value(906), 1.0, s.ad_value(905), (-1.0), s.ad_value(976), 1.0, s.ad_value(815), 2.0), 809);

        s.store_sub_ad_lhs(819, A::scale_offset(s.ad_value(816), 2.0, 1.0), 810);

        s.store_sub_scaled_inputs(820, 817, 2.0, 811, 1.0);

        s.store_add_scaled_product_indices(821, 798, 1.0, 908, 818, 1.0);

        s.store_add_scaled_product_indices(822, 907, 1.0, 908, 819, 1.0);

        s.store_mul(823, 908, 820);

        s.store_add_scaled_product_indices(824, 799, (-1.0), 821, 812, 1.0);

        s.store_add_ad_lhs(825, A::add_scaled_products(s.ad_value(822), s.ad_value(812), 1.0, s.ad_value(821), s.ad_value(813), 1.0), 799);

        s.store_sub_ad_lhs(826, A::add_scaled_products3(s.ad_value(823), s.ad_value(812), 1.0, s.ad_value(822), s.ad_value(813), 2.0, s.ad_value(821), s.ad_value(814), 1.0), 799);

        s.store_add_scaled_square_product_indices(835, 825, 1.0, 824, 826, (-0.5));

        s.store_div_scaled_product3_mixed_iiia(827, 824, 825, 835, -1.0, A::offset(A::square(s.ad_value(835)), 1e-200), 1.0);

        s.store_add(976, 976, 827);

        s.store_mul(798, 907, 976);

        s.store_mul(828, 908, 977);

        s.store_add(821, 798, 828);

        s.store_offset_scaled(829, 821, 0.065345483024, 1.0);

        s.store_add_scaled_product_value_ad(830, A::scale_offset(s.ad_value(821), 8.5797362674, 39.478417604), 1.0, 798, 828, 1.0);

        s.store_add_scaled_product_indices(831, 821, (2.0 * 39.478417604), 798, 828, 39.478417604);

        s.store_sqrt_ad(832, A::add_scaled_square_product(s.ad_value(830), 1.0, s.ad_value(829), s.ad_value(831), (-4.0)));

        s.store_div_scaled_inputs2_indices(800, 832, 1.0, 830, (-1.0), 829, 2.0);

        s.store_sub_ad_lhs(833, A::square(s.ad_value(798)), 800);

        s.b[1150] = (s.v[833] > 0.0);
        s.v[1150] = if s.b[1150] { 1.0 } else { 0.0 };

        if s.b[1150] {
            s.store_mul_ad_rhs(824, 833, A::add_scaled_inputs4(A::ln(A::div(s.ad_value(833), s.ad_value(880))), 1.0, s.ad_value(975), 1.0, s.ad_value(905), -1.0, s.ad_value(976), 1.0));
            s.store_add_scaled_product_indices(825, 833, 1.0, 907, 798, 2.0);
        }

        let (assign17950_e17713,) = {
    if s.b[1150] {
        let assign17950_e17709: f64 = (s.v[905] - s.v[976]);
        let assign17950_e17711: f64 = (assign17950_e17709 - s.v[792]);
        (assign17950_e17711,)
    } else {
        (s.v[834],)
    }
};
        s.v[834] = assign17950_e17713;

        s.b[1151] = ((((s.v[824] < 0.0) && (s.v[825] > 0.0)) && (((s.v[834] + 2.3025850929941) + ((s.v[907]) as f64).ln()) > 0.0)) || (s.v[834] > 1.0));
        s.v[1151] = if s.b[1151] { 1.0 } else { 0.0 };

        if (s.b[1150] && s.b[1151]) {
            s.store_sub_div_rhs_indices(976, 976, 824, 825);
        }

        s.store_mul(798, 907, 976);

        s.store_mul(828, 908, 977);

        s.store_add(821, 798, 828);

        s.store_offset_scaled(829, 821, 0.065345483024, 1.0);

        s.store_add_scaled_product_value_ad(830, A::scale_offset(s.ad_value(821), 8.5797362674, 39.478417604), 1.0, 798, 828, 1.0);

        s.store_add_scaled_product_indices(831, 821, (2.0 * 39.478417604), 798, 828, 39.478417604);

        s.store_sqrt_ad(832, A::add_scaled_square_product(s.ad_value(830), 1.0, s.ad_value(829), s.ad_value(831), (-4.0)));

        s.store_div_scaled_inputs2_indices(800, 832, 1.0, 830, (-1.0), 829, 2.0);

        s.b[1152] = (s.v[800] < (-0.005));
        s.v[1152] = if s.b[1152] { 1.0 } else { 0.0 };

        if s.b[1152] {
            s.store_sqrt_abs_ad(803, s.ad_value(800));
            s.store_div_ad_rhs(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));
            s.store_div_scaled_inputs2_mixed_iai(805, 800, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 0.25, 800, 1.0);
        }

        s.b[1153] = (s.v[800] > 0.005);
        s.v[1153] = if s.b[1153] { 1.0 } else { 0.0 };

        if ((!s.b[1152]) && s.b[1153]) {
            s.store_sqrt_abs_ad(803, s.ad_value(800));
            s.store_exp_neg_input(806, 803);
            s.store_div_scaled_product_offset_rhs(804, s.ad_value(803), s.ad_value(806), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);
            s.store_div_scaled_inputs2_mixed_iai(805, 800, 0.25, A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 0.25, 800, 1.0);
        }

        if ((!s.b[1152]) && (!s.b[1153])) {
            s.store_offset_ad(804, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);
            s.store_offset_scaled_ad(805, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
        }

        s.store_sub_ad_rhs(800, 800, A::div_scaled_inputs2(A::add_scaled_products(s.ad_value(821), s.ad_value(804), 1.0, s.ad_value(798), s.ad_value(828), 1.0), 1.0, s.ad_value(800), 1.0, A::offset(A::mul(s.ad_value(821), s.ad_value(805)), 1.0), 1.0));

        s.store_sub_ad_lhs(833, A::square(s.ad_value(798)), 800);

        s.b[1154] = (s.v[833] > 0.0);
        s.v[1154] = if s.b[1154] { 1.0 } else { 0.0 };

        if s.b[1154] {
            s.store_mul_ad_rhs(824, 833, A::add_scaled_inputs4(A::ln(A::div(s.ad_value(833), s.ad_value(880))), 1.0, s.ad_value(975), 1.0, s.ad_value(905), -1.0, s.ad_value(976), 1.0));
            s.store_add_scaled_product_indices(825, 833, 1.0, 907, 798, 2.0);
        }

        let (assign18220_e17986,) = {
    if s.b[1154] {
        let assign18220_e17982: f64 = (s.v[905] - s.v[976]);
        let assign18220_e17984: f64 = (assign18220_e17982 - s.v[792]);
        (assign18220_e17984,)
    } else {
        (s.v[834],)
    }
};
        s.v[834] = assign18220_e17986;

        s.b[1155] = ((((s.v[824] < 0.0) && (s.v[825] > 0.0)) && (((s.v[834] + 2.3025850929941) + ((s.v[907]) as f64).ln()) > 0.0)) || (s.v[834] > 1.0));
        s.v[1155] = if s.b[1155] { 1.0 } else { 0.0 };

        if (s.b[1154] && s.b[1155]) {
            s.store_sub_div_rhs_indices(976, 976, 824, 825);
        }

        s.store_mul(798, 907, 976);

        s.b[1156] = (((s.v[905] - s.v[976]) - s.v[975]) < 80.0);
        s.v[1156] = if s.b[1156] { 1.0 } else { 0.0 };

        if s.b[1156] {
            s.store_exp_ad(789, A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0));
        }

        if (!s.b[1156]) {
            let assign18280_ad_e18070: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(789, assign18280_ad_e18070, 1.0, 5.54062e34);
        }

        s.store_mul(799, 880, 789);

        s.store_sub_ad_lhs(800, A::square(s.ad_value(798)), 799);

        s.store_add_scaled_product_indices(801, 799, 1.0, 907, 798, 2.0);

        s.store_add_scaled_product_indices(802, 799, (-1.0), 907, 907, 2.0);

        s.b[1157] = (s.v[800] < (-0.005));
        s.v[1157] = if s.b[1157] { 1.0 } else { 0.0 };

        if s.b[1157] {
            s.store_sqrt_abs_ad(803, s.ad_value(800));
            s.store_div_ad_rhs(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));
            s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);
            s.store_mul_add_ad_lhs(805, s.ad_value(800), A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 789);
            s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);
            s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);
            s.store_mul_div_lhs(810, 801, 800, 790);
            s.store_div_ad_lhs(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);
        }

        s.b[1158] = (s.v[800] > 0.005);
        s.v[1158] = if s.b[1158] { 1.0 } else { 0.0 };

        if ((!s.b[1157]) && s.b[1158]) {
            s.store_sqrt_abs_ad(803, s.ad_value(800));
            s.store_exp_neg_input(806, 803);
            s.store_div_scaled_product_offset_rhs(804, s.ad_value(803), s.ad_value(806), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);
            s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);
            s.store_mul_add_ad_lhs(805, s.ad_value(800), A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 789);
            s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);
            s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);
            s.store_mul_div_lhs(810, 801, 800, 790);
            s.store_div_ad_lhs(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);
        }

        if ((!s.b[1157]) && (!s.b[1158])) {
            s.store_offset_scaled_ad(791, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(804, 800, 791, 2.0);
            s.store_offset_scaled_ad(789, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_mul(805, 801, 789);
            s.store_offset_scaled_ad(790, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(807, 802, 789, 1.0, A::square(s.ad_value(801)), 790, (-1.0));
            s.store_scaled_mul(810, 801, 791, (-0.5));
            s.store_add_scaled_product_value_ad(811, A::mul3_scaled_output(s.ad_value(801), s.ad_value(801), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 2.0, A::scale(s.ad_value(800), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 802, 791, (-0.5));
        }

        s.b[1159] = (s.v[800] > 0.005);
        s.v[1159] = if s.b[1159] { 1.0 } else { 0.0 };

        if s.b[1159] {
            s.store_div_scaled_inputs_mixed_ia(790, 800, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0);
            s.store_mul(808, 790, 806);
            s.store_sub_ad_lhs(809, A::ln(s.ad_value(790)), 803);
        }

        s.b[1160] = (s.v[800] < (-0.005));
        s.v[1160] = if s.b[1160] { 1.0 } else { 0.0 };

        if ((!s.b[1159]) && s.b[1160]) {
            s.store_sin_scaled_input(790, 803, 0.5);
            s.store_div_scaled_inputs_mixed_ia(808, 800, -1.0, A::square(s.ad_value(790)), 1.0);
            s.store_ln(809, 808);
        }

        if ((!s.b[1159]) && (!s.b[1160])) {
            s.store_sub_from_scalar_ad(808, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(809, 808);
        }

        s.b[1161] = (((1.01 * s.v[798]) + s.v[804]) > 0.0);
        s.v[1161] = if s.b[1161] { 1.0 } else { 0.0 };

        if s.b[1161] {
            s.store_add(812, 798, 804);
            s.store_add(813, 907, 805);
            s.copy_ad(814, 807);
        }

        if (!s.b[1161]) {
            s.store_div_from_scalar_sub_ad(790, 1.0, s.ad_value(798), s.ad_value(804));
            s.store_sub(791, 805, 907);
            s.store_mul_sub_lhs(812, 799, 808, 790);
            s.store_mul_ad_lhs(813, A::add_scaled_value_products(s.ad_value(799), (-1.0), s.ad_value(791), s.ad_value(812), 1.0, s.ad_value(810), s.ad_value(808), (-1.0)), 790);
            s.store_mul_ad_lhs(814, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(807), s.ad_value(812), 1.0, s.ad_value(791), s.ad_value(813), 2.0), 1.0, s.ad_value(799), 1.0, A::add(s.ad_value(811), A::square(s.ad_value(810))), s.ad_value(808), (-1.0)), 790);
        }

        s.b[1162] = (s.v[812] > 0.0);
        s.v[1162] = if s.b[1162] { 1.0 } else { 0.0 };

        if s.b[1162] {
            s.store_ln(815, 812);
            s.store_div_from_scalar(789, 1.0, 812);
            s.store_mul(816, 813, 789);
            s.store_add_scaled_square_product_indices(817, 816, (-1.0), 814, 789, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[1162]) {
            s.store_add_offset_lhs_ad_rhs(815, 798, 0.6931471805599, A::ln_scaled_input(s.ad_value(798), -1.0));
            s.store_div_from_scalar(789, 1.0, 976);
            s.store_add(816, 907, 789);
            s.store_mul_neg_lhs(817, 789, 789);
        }

        s.store_sub_ad_lhs(818, A::add_scaled_inputs4(s.ad_value(906), 1.0, s.ad_value(905), (-1.0), s.ad_value(976), 1.0, s.ad_value(815), 2.0), 809);

        s.store_sub_ad_lhs(819, A::scale_offset(s.ad_value(816), 2.0, 1.0), 810);

        s.store_sub_scaled_inputs(820, 817, 2.0, 811, 1.0);

        s.store_add_scaled_product_indices(821, 798, 1.0, 908, 818, 1.0);

        s.store_add_scaled_product_indices(822, 907, 1.0, 908, 819, 1.0);

        s.store_mul(823, 908, 820);

        s.store_add_scaled_product_indices(824, 799, (-1.0), 821, 812, 1.0);

        s.store_add_ad_lhs(825, A::add_scaled_products(s.ad_value(822), s.ad_value(812), 1.0, s.ad_value(821), s.ad_value(813), 1.0), 799);

        s.store_sub_ad_lhs(826, A::add_scaled_products3(s.ad_value(823), s.ad_value(812), 1.0, s.ad_value(822), s.ad_value(813), 2.0, s.ad_value(821), s.ad_value(814), 1.0), 799);

        s.store_add_scaled_square_product_indices(835, 825, 1.0, 824, 826, (-0.5));

        s.store_div_scaled_product3_mixed_iiia(827, 824, 825, 835, -1.0, A::offset(A::square(s.ad_value(835)), 1e-200), 1.0);

        s.store_add(976, 976, 827);

        s.store_mul(798, 907, 976);

        s.b[1163] = (((s.v[905] - s.v[976]) - s.v[975]) < 80.0);
        s.v[1163] = if s.b[1163] { 1.0 } else { 0.0 };

        if s.b[1163] {
            s.store_exp_ad(789, A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0));
        }

        if (!s.b[1163]) {
            let assign19030_ad_e18866: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(789, assign19030_ad_e18866, 1.0, 5.54062e34);
        }

        s.store_mul(799, 880, 789);

        s.store_sub_ad_lhs(800, A::square(s.ad_value(798)), 799);

        s.store_add_scaled_product_indices(801, 799, 1.0, 907, 798, 2.0);

        s.store_add_scaled_product_indices(802, 799, (-1.0), 907, 907, 2.0);

        s.b[1164] = (s.v[800] < (-0.005));
        s.v[1164] = if s.b[1164] { 1.0 } else { 0.0 };

        if s.b[1164] {
            s.store_sqrt_abs_ad(803, s.ad_value(800));
            s.store_div_ad_rhs(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));
            s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);
            s.store_mul_add_ad_lhs(805, s.ad_value(800), A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 789);
            s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);
            s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);
            s.store_mul_div_lhs(810, 801, 800, 790);
            s.store_div_ad_lhs(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);
        }

        s.b[1165] = (s.v[800] > 0.005);
        s.v[1165] = if s.b[1165] { 1.0 } else { 0.0 };

        if ((!s.b[1164]) && s.b[1165]) {
            s.store_sqrt_abs_ad(803, s.ad_value(800));
            s.store_exp_neg_input(806, 803);
            s.store_div_scaled_product_offset_rhs(804, s.ad_value(803), s.ad_value(806), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);
            s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);
            s.store_mul_add_ad_lhs(805, s.ad_value(800), A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 789);
            s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);
            s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);
            s.store_mul_div_lhs(810, 801, 800, 790);
            s.store_div_ad_lhs(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);
        }

        if ((!s.b[1164]) && (!s.b[1165])) {
            s.store_offset_scaled_ad(791, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(804, 800, 791, 2.0);
            s.store_offset_scaled_ad(789, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_mul(805, 801, 789);
            s.store_offset_scaled_ad(790, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(807, 802, 789, 1.0, A::square(s.ad_value(801)), 790, (-1.0));
            s.store_scaled_mul(810, 801, 791, (-0.5));
            s.store_add_scaled_product_value_ad(811, A::mul3_scaled_output(s.ad_value(801), s.ad_value(801), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 2.0, A::scale(s.ad_value(800), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 802, 791, (-0.5));
        }

        s.b[1166] = (s.v[800] > 0.005);
        s.v[1166] = if s.b[1166] { 1.0 } else { 0.0 };

        if s.b[1166] {
            s.store_div_scaled_inputs_mixed_ia(790, 800, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0);
            s.store_mul(808, 790, 806);
            s.store_sub_ad_lhs(809, A::ln(s.ad_value(790)), 803);
        }

        s.b[1167] = (s.v[800] < (-0.005));
        s.v[1167] = if s.b[1167] { 1.0 } else { 0.0 };

        if ((!s.b[1166]) && s.b[1167]) {
            s.store_sin_scaled_input(790, 803, 0.5);
            s.store_div_scaled_inputs_mixed_ia(808, 800, -1.0, A::square(s.ad_value(790)), 1.0);
            s.store_ln(809, 808);
        }

        if ((!s.b[1166]) && (!s.b[1167])) {
            s.store_sub_from_scalar_ad(808, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(809, 808);
        }

        s.b[1168] = (((1.01 * s.v[798]) + s.v[804]) > 0.0);
        s.v[1168] = if s.b[1168] { 1.0 } else { 0.0 };

        if s.b[1168] {
            s.store_add(812, 798, 804);
            s.store_add(813, 907, 805);
            s.copy_ad(814, 807);
        }

        if (!s.b[1168]) {
            s.store_div_from_scalar_sub_ad(790, 1.0, s.ad_value(798), s.ad_value(804));
            s.store_sub(791, 805, 907);
            s.store_mul_sub_lhs(812, 799, 808, 790);
            s.store_mul_ad_lhs(813, A::add_scaled_value_products(s.ad_value(799), (-1.0), s.ad_value(791), s.ad_value(812), 1.0, s.ad_value(810), s.ad_value(808), (-1.0)), 790);
            s.store_mul_ad_lhs(814, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(807), s.ad_value(812), 1.0, s.ad_value(791), s.ad_value(813), 2.0), 1.0, s.ad_value(799), 1.0, A::add(s.ad_value(811), A::square(s.ad_value(810))), s.ad_value(808), (-1.0)), 790);
        }

        s.b[1169] = (s.v[812] > 0.0);
        s.v[1169] = if s.b[1169] { 1.0 } else { 0.0 };

        if s.b[1169] {
            s.store_ln(815, 812);
            s.store_div_from_scalar(789, 1.0, 812);
            s.store_mul(816, 813, 789);
            s.store_add_scaled_square_product_indices(817, 816, (-1.0), 814, 789, 1.0);
        }

        if (!s.b[1169]) {
            s.store_add_offset_lhs_ad_rhs(815, 798, 0.6931471805599, A::ln_scaled_input(s.ad_value(798), -1.0));
            s.store_div_from_scalar(789, 1.0, 976);
            s.store_add(816, 907, 789);
            s.store_mul_neg_lhs(817, 789, 789);
        }

        s.store_sub_ad_lhs(818, A::add_scaled_inputs4(s.ad_value(906), 1.0, s.ad_value(905), (-1.0), s.ad_value(976), 1.0, s.ad_value(815), 2.0), 809);

        s.store_sub_ad_lhs(819, A::scale_offset(s.ad_value(816), 2.0, 1.0), 810);

        s.store_sub_scaled_inputs(820, 817, 2.0, 811, 1.0);

        s.store_add_scaled_product_indices(821, 798, 1.0, 908, 818, 1.0);

        s.store_add_scaled_product_indices(822, 907, 1.0, 908, 819, 1.0);

        s.store_mul(823, 908, 820);

        s.store_add_scaled_product_indices(824, 799, (-1.0), 821, 812, 1.0);

        s.store_add_ad_lhs(825, A::add_scaled_products(s.ad_value(822), s.ad_value(812), 1.0, s.ad_value(821), s.ad_value(813), 1.0), 799);

        s.store_sub_ad_lhs(826, A::add_scaled_products3(s.ad_value(823), s.ad_value(812), 1.0, s.ad_value(822), s.ad_value(813), 2.0, s.ad_value(821), s.ad_value(814), 1.0), 799);

        s.store_add_scaled_square_product_indices(835, 825, 1.0, 824, 826, (-0.5));

        s.store_div_scaled_product3_mixed_iiia(827, 824, 825, 835, -1.0, A::offset(A::square(s.ad_value(835)), 1e-200), 1.0);

        s.store_add(976, 976, 827);

        s.b[1170] = (p.p10 == 1.0);
        s.v[1170] = if s.b[1170] { 1.0 } else { 0.0 };

        s.b[1171] = (((s.v[827]) as f64).abs() > 0.01);
        s.v[1171] = if s.b[1171] { 1.0 } else { 0.0 };

        if (s.b[1170] && s.b[1171]) {
            s.store_mul(798, 907, 976);
        }

        s.b[1172] = (((s.v[905] - s.v[976]) - s.v[975]) < 80.0);
        s.v[1172] = if s.b[1172] { 1.0 } else { 0.0 };

        if ((s.b[1170] && s.b[1171]) && s.b[1172]) {
            s.store_exp_ad(789, A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0));
        }

        if ((s.b[1170] && s.b[1171]) && (!s.b[1172])) {
            let assign19800_ad_e19682: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(789, assign19800_ad_e19682, 1.0, 5.54062e34);
        }

        if (s.b[1170] && s.b[1171]) {
            s.store_mul(799, 880, 789);
            s.store_sub_ad_lhs(800, A::square(s.ad_value(798)), 799);
            s.store_add_scaled_product_indices(801, 799, 1.0, 907, 798, 2.0);
            s.store_add_scaled_product_indices(802, 799, (-1.0), 907, 907, 2.0);
        }

        s.b[1173] = (s.v[800] < (-0.005));
        s.v[1173] = if s.b[1173] { 1.0 } else { 0.0 };

        if ((s.b[1170] && s.b[1171]) && s.b[1173]) {
            s.store_sqrt_abs_ad(803, s.ad_value(800));
            s.store_div_ad_rhs(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));
            s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);
            s.store_mul_add_ad_lhs(805, s.ad_value(800), A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 789);
            s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);
            s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);
            s.store_mul_div_lhs(810, 801, 800, 790);
            s.store_div_ad_lhs(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);
        }

        s.b[1174] = (s.v[800] > 0.005);
        s.v[1174] = if s.b[1174] { 1.0 } else { 0.0 };

        if (((s.b[1170] && s.b[1171]) && (!s.b[1173])) && s.b[1174]) {
            s.store_sqrt_abs_ad(803, s.ad_value(800));
            s.store_exp_neg_input(806, 803);
            s.store_div_scaled_product_offset_rhs(804, s.ad_value(803), s.ad_value(806), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);
            s.store_div_scaled_inputs_indices(789, 801, 0.25, 800, 1.0);
            s.store_mul_add_ad_lhs(805, s.ad_value(800), A::mul_sub_from_scalar_rhs(s.ad_value(804), 2.0, s.ad_value(804)), 789);
            s.store_add_scaled_product_mixed_aai(807, A::div_scaled_product(s.ad_value(805), s.ad_value(802), 1.0, s.ad_value(801), 1.0), 1.0, A::add_scaled_offset_product_rhs(s.ad_value(801), 1.0, s.ad_value(805), s.ad_value(804), 1.0, (-2.0)), 789, 1.0);
            s.store_sub_from_scalar_scaled_input(790, 1.0, 804, 0.5);
            s.store_mul_div_lhs(810, 801, 800, 790);
            s.store_div_ad_lhs(811, A::add_scaled_products(s.ad_value(802), s.ad_value(790), 1.0, s.ad_value(801), A::add_scaled_inputs(s.ad_value(810), 1.0, s.ad_value(805), 0.5), (-1.0)), 800);
        }

        if (((s.b[1170] && s.b[1171]) && (!s.b[1173])) && (!s.b[1174])) {
            s.store_offset_scaled_ad(791, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.025), 0.0238095238095), 0.0166666666667), (-0.1666666666667), 0.1666666666667);
            s.store_offset_mul(804, 800, 791, 2.0);
            s.store_offset_scaled_ad(789, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_mul(805, 801, 789);
            s.store_offset_scaled_ad(790, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0420875420875421), 0.05), 0.0714285714286), (-0.0055555555556), 0.0055555555556);
            s.store_add_scaled_products_right_left_ad(807, 802, 789, 1.0, A::square(s.ad_value(801)), 790, (-1.0));
            s.store_scaled_mul(810, 801, 791, (-0.5));
            s.store_add_scaled_product_value_ad(811, A::mul3_scaled_output(s.ad_value(801), s.ad_value(801), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 2.0, A::scale(s.ad_value(800), 0.075), 0.0238095238095)), (0.25 * 0.0055555555556)), 1.0, 802, 791, (-0.5));
        }

        s.b[1175] = (s.v[800] > 0.005);
        s.v[1175] = if s.b[1175] { 1.0 } else { 0.0 };

        if ((s.b[1170] && s.b[1171]) && s.b[1175]) {
            s.store_div_scaled_inputs_mixed_ia(790, 800, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0);
            s.store_mul(808, 790, 806);
            s.store_sub_ad_lhs(809, A::ln(s.ad_value(790)), 803);
        }

        s.b[1176] = (s.v[800] < (-0.005));
        s.v[1176] = if s.b[1176] { 1.0 } else { 0.0 };

        if (((s.b[1170] && s.b[1171]) && (!s.b[1175])) && s.b[1176]) {
            s.store_sin_scaled_input(790, 803, 0.5);
            s.store_div_scaled_inputs_mixed_ia(808, 800, -1.0, A::square(s.ad_value(790)), 1.0);
            s.store_ln(809, 808);
        }

        if (((s.b[1170] && s.b[1171]) && (!s.b[1175])) && (!s.b[1176])) {
            s.store_sub_from_scalar_ad(808, 4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(800), 1.0, A::scale(s.ad_value(800), 0.0396825396825397), 0.05), 0.3333333333333));
            s.store_ln(809, 808);
        }

        s.b[1177] = (((1.01 * s.v[798]) + s.v[804]) > 0.0);
        s.v[1177] = if s.b[1177] { 1.0 } else { 0.0 };

        if ((s.b[1170] && s.b[1171]) && s.b[1177]) {
            s.store_add(812, 798, 804);
            s.store_add(813, 907, 805);
            s.copy_ad(814, 807);
        }

        if ((s.b[1170] && s.b[1171]) && (!s.b[1177])) {
            s.store_div_from_scalar_sub_ad(790, 1.0, s.ad_value(798), s.ad_value(804));
            s.store_sub(791, 805, 907);
            s.store_mul_sub_lhs(812, 799, 808, 790);
        }

    }

    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1170] && s.b[1171]) && (!s.b[1177])) {
            s.store_mul_ad_lhs(813, A::add_scaled_value_products(s.ad_value(799), (-1.0), s.ad_value(791), s.ad_value(812), 1.0, s.ad_value(810), s.ad_value(808), (-1.0)), 790);
            s.store_mul_ad_lhs(814, A::add_scaled_inputs_product(A::add_scaled_products(s.ad_value(807), s.ad_value(812), 1.0, s.ad_value(791), s.ad_value(813), 2.0), 1.0, s.ad_value(799), 1.0, A::add(s.ad_value(811), A::square(s.ad_value(810))), s.ad_value(808), (-1.0)), 790);
        }

        s.b[1178] = (s.v[812] > 0.0);
        s.v[1178] = if s.b[1178] { 1.0 } else { 0.0 };

        if ((s.b[1170] && s.b[1171]) && s.b[1178]) {
            s.store_ln(815, 812);
            s.store_div_from_scalar(789, 1.0, 812);
            s.store_mul(816, 813, 789);
            s.store_add_scaled_square_product_indices(817, 816, (-1.0), 814, 789, 1.0);
        }

        if ((s.b[1170] && s.b[1171]) && (!s.b[1178])) {
            s.store_add_offset_lhs_ad_rhs(815, 798, 0.6931471805599, A::ln_scaled_input(s.ad_value(798), -1.0));
            s.store_div_from_scalar(789, 1.0, 976);
            s.store_add(816, 907, 789);
            s.store_mul_neg_lhs(817, 789, 789);
        }

        if (s.b[1170] && s.b[1171]) {
            s.store_sub_ad_lhs(818, A::add_scaled_inputs4(s.ad_value(906), 1.0, s.ad_value(905), (-1.0), s.ad_value(976), 1.0, s.ad_value(815), 2.0), 809);
            s.store_sub_ad_lhs(819, A::scale_offset(s.ad_value(816), 2.0, 1.0), 810);
            s.store_sub_scaled_inputs(820, 817, 2.0, 811, 1.0);
            s.store_add_scaled_product_indices(821, 798, 1.0, 908, 818, 1.0);
            s.store_add_scaled_product_indices(822, 907, 1.0, 908, 819, 1.0);
            s.store_mul(823, 908, 820);
            s.store_add_scaled_product_indices(824, 799, (-1.0), 821, 812, 1.0);
            s.store_add_ad_lhs(825, A::add_scaled_products(s.ad_value(822), s.ad_value(812), 1.0, s.ad_value(821), s.ad_value(813), 1.0), 799);
            s.store_sub_ad_lhs(826, A::add_scaled_products3(s.ad_value(823), s.ad_value(812), 1.0, s.ad_value(822), s.ad_value(813), 2.0, s.ad_value(821), s.ad_value(814), 1.0), 799);
            s.store_add_scaled_square_product_indices(835, 825, 1.0, 824, 826, (-0.5));
            s.store_div_scaled_product3_mixed_iiia(827, 824, 825, 835, -1.0, A::offset(A::square(s.ad_value(835)), 1e-200), 1.0);
            s.store_add(976, 976, 827);
        }

        s.store_mul(979, 907, 976);

        s.b[1179] = (((s.v[905] - s.v[976]) - s.v[975]) < 80.0);
        s.v[1179] = if s.b[1179] { 1.0 } else { 0.0 };

        if s.b[1179] {
            s.store_exp_ad(789, A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0));
        }

        if (!s.b[1179]) {
            let assign20550_ad_e20754: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(905), 1.0, s.ad_value(976), (-1.0), s.ad_value(975), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(789, assign20550_ad_e20754, 1.0, 5.54062e34);
        }

        s.store_mul(982, 880, 789);

        s.store_sub_ad_lhs(981, A::square(s.ad_value(979)), 982);

        s.b[1180] = (s.v[982] <= 0.0);
        s.v[1180] = if s.b[1180] { 1.0 } else { 0.0 };

        if s.b[1180] {
            s.store_scalar(978, 1e-80);
            s.store_sub(980, 978, 979);
            s.store_div(977, 980, 908);
        }

        s.b[1181] = (s.v[981] < (-0.005));
        s.v[1181] = if s.b[1181] { 1.0 } else { 0.0 };

        if ((!s.b[1180]) && s.b[1181]) {
            s.store_sqrt_abs_ad(803, s.ad_value(981));
            s.store_div_ad_rhs(804, 803, A::tan(A::scale(s.ad_value(803), 0.5)));
        }

        s.b[1182] = (s.v[981] > 0.005);
        s.v[1182] = if s.b[1182] { 1.0 } else { 0.0 };

        if (((!s.b[1180]) && (!s.b[1181])) && s.b[1182]) {
            s.store_sqrt_abs_ad(803, s.ad_value(981));
            s.store_exp_neg_input(806, 803);
            s.store_div_scaled_product_offset_rhs(804, s.ad_value(803), s.ad_value(806), 1.0, 1.0, A::sub_from_scalar(1.0, s.ad_value(806)), 1.0);
        }

        if (((!s.b[1180]) && (!s.b[1181])) && (!s.b[1182])) {
            s.store_offset_ad(804, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(981), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(981), 1.0, A::scale(s.ad_value(981), 0.0238095238095), 0.0166666666667), 0.1666666666667), 2.0);
        }

        s.b[1183] = (((1.01 * s.v[979]) + s.v[804]) > 0.0);
        s.v[1183] = if s.b[1183] { 1.0 } else { 0.0 };

        if ((!s.b[1180]) && s.b[1183]) {
            s.store_add(789, 979, 804);
        }

        s.b[1184] = ((s.v[982] * s.v[979]) < (((0.9 * s.v[979]) * s.v[979]) * s.v[789]));
        s.v[1184] = if s.b[1184] { 1.0 } else { 0.0 };

        if (((!s.b[1180]) && s.b[1183]) && s.b[1184]) {
            s.store_offset_div(978, 982, 789, 1e-80);
            s.store_sub(980, 978, 979);
            s.store_div(977, 980, 908);
        }

        s.b[1185] = (s.v[981] > 0.005);
        s.v[1185] = if s.b[1185] { 1.0 } else { 0.0 };

        if ((((!s.b[1180]) && s.b[1183]) && (!s.b[1184])) && s.b[1185]) {
            s.store_sub_ad_lhs(790, A::ln(A::div_scaled_inputs(s.ad_value(981), 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0)), 803);
        }

        s.b[1186] = (s.v[981] < (-0.005));
        s.v[1186] = if s.b[1186] { 1.0 } else { 0.0 };

        if (((((!s.b[1180]) && s.b[1183]) && (!s.b[1184])) && (!s.b[1185])) && s.b[1186]) {
            s.store_sin_scaled_input(791, 803, 0.5);
            s.store_ln_div_scaled_input_square_denominator(790, 981, -1.0, 791, 1.0);
        }

        if (((((!s.b[1180]) && s.b[1183]) && (!s.b[1184])) && (!s.b[1185])) && (!s.b[1186])) {
            s.store_ln_ad(790, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(981), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(981), 1.0, A::scale(s.ad_value(981), 0.0396825396825397), 0.05), 0.3333333333333)));
        }

        if (((!s.b[1180]) && s.b[1183]) && (!s.b[1184])) {
            s.store_sub_ad_lhs(977, A::add_scaled_inputs4(s.ad_value(906), 1.0, s.ad_value(905), (-1.0), s.ad_value(976), 1.0, A::ln(s.ad_value(789)), 2.0), 790);
            s.store_mul(980, 908, 977);
            s.store_add(978, 979, 980);
        }

        s.b[1187] = (s.v[981] > 0.005);
        s.v[1187] = if s.b[1187] { 1.0 } else { 0.0 };

        s.b[1188] = ((((s.v[976] + s.v[975]) - s.v[905]) - s.v[803]) < 80.0);
        s.v[1188] = if s.b[1188] { 1.0 } else { 0.0 };

        if ((((!s.b[1180]) && (!s.b[1183])) && s.b[1187]) && s.b[1188]) {
            s.store_exp_ad(791, A::add_scaled_inputs4(s.ad_value(976), 1.0, s.ad_value(975), 1.0, s.ad_value(905), -1.0, s.ad_value(803), -1.0));
        }

        if ((((!s.b[1180]) && (!s.b[1183])) && s.b[1187]) && (!s.b[1188])) {
            let assign20880_ad_e21173: A = A::mul_offset_lhs(A::add_scaled_inputs4(s.ad_value(976), 1.0, s.ad_value(975), 1.0, s.ad_value(905), -1.0, s.ad_value(803), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs4(s.ad_value(976), 1.0, s.ad_value(975), 1.0, s.ad_value(905), -1.0, s.ad_value(803), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs4(s.ad_value(976), 1.0, s.ad_value(975), 1.0, s.ad_value(905), -1.0, s.ad_value(803), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(791, assign20880_ad_e21173, 1.0, 5.54062e34);
        }

        if (((!s.b[1180]) && (!s.b[1183])) && s.b[1187]) {
            s.store_div(790, 791, 880);
            s.store_div_scaled_product_denominator_ad(789, 981, 790, 4.0, A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(806), 2.0, s.ad_value(806))), 1.0);
        }

        s.b[1189] = (s.v[981] < (-0.005));
        s.v[1189] = if s.b[1189] { 1.0 } else { 0.0 };

        if ((((!s.b[1180]) && (!s.b[1183])) && (!s.b[1187])) && s.b[1189]) {
            s.store_sin_scaled_input(790, 803, 0.5);
            s.store_div_scaled_value_by_product(789, s.ad_value(981), -1.0, A::square(s.ad_value(790)), s.ad_value(982), 1.0);
        }

        if ((((!s.b[1180]) && (!s.b[1183])) && (!s.b[1187])) && (!s.b[1189])) {
            s.store_div_ad_lhs(789, A::sub_from_scalar(4.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(981), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(981), 1.0, A::scale(s.ad_value(981), 0.0396825396825397), 0.05), 0.3333333333333)), 982);
        }

        if ((!s.b[1180]) && (!s.b[1183])) {
            s.store_offset_div_scaled_inputs2(978, s.ad_value(979), 1.0, s.ad_value(804), (-1.0), A::sub_from_scalar(1.0, s.ad_value(789)), 1.0, 1e-80);
            s.store_sub(980, 978, 979);
            s.store_div(977, 980, 908);
        }

        s.b[1190] = (((s.v[906] - s.v[977]) - s.v[975]) < 80.0);
        s.v[1190] = if s.b[1190] { 1.0 } else { 0.0 };

        if s.b[1190] {
            s.store_exp_ad(789, A::add_scaled_inputs3(s.ad_value(906), 1.0, s.ad_value(977), (-1.0), s.ad_value(975), -1.0));
        }

        if (!s.b[1190]) {
            let assign21000_ad_e21370: A = A::mul_offset_lhs(A::add_scaled_inputs3(s.ad_value(906), 1.0, s.ad_value(977), (-1.0), s.ad_value(975), -1.0), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::add_scaled_inputs3(s.ad_value(906), 1.0, s.ad_value(977), (-1.0), s.ad_value(975), -1.0), (-80.0)), 0.5, A::scale_offset(A::add_scaled_inputs3(s.ad_value(906), 1.0, s.ad_value(977), (-1.0), s.ad_value(975), -1.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0));
            s.store_scaled_offset_ad(789, assign21000_ad_e21370, 1.0, 5.54062e34);
        }

        s.store_mul(983, 880, 789);

        s.v[986] = 0.0;

        s.v[987] = 0.0;

        s.v[984] = 0.0;

        s.v[985] = 0.0;

        s.v[988] = 0.0;

        s.v[989] = 0.0;

        s.b[1191] = (s.v[913] > 1e-6);
        s.v[1191] = if s.b[1191] { 1.0 } else { 0.0 };

        if s.b[1191] {
            s.store_mul(984, 982, 881);
            s.store_mul(985, 983, 882);
            s.store_add_scaled_inputs(986, 984, 1.0, 979, 2.0);
            s.store_add_scaled_inputs(987, 985, 1.0, 980, 2.0);
            s.store_add_scaled_inputs3_indices(988, 978, 2.0, 984, 1.0, 985, 1.0);
        }

        s.b[1192] = (((s.v[981]) as f64).abs() > 0.005);
        s.v[1192] = if s.b[1192] { 1.0 } else { 0.0 };

        if (s.b[1191] && s.b[1192]) {
            s.store_add_scaled_products3(2, s.ad_value(986), s.ad_value(987), 1.0, A::offset(s.ad_value(976), 2.0), s.ad_value(987), 2.0, A::offset(s.ad_value(977), 2.0), s.ad_value(986), 2.0);
            s.store_div_scaled_product_by_product(989, s.ad_value(981), s.ad_value(988), (-4.0), s.ad_value(978), s.ad_value(2), 1.0);
        }

        if (s.b[1191] && (!s.b[1192])) {
            s.store_offset_scaled_ad(2, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(981), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(981), 1.0, A::scale(s.ad_value(981), 0.0333333333333), 0.0357142857143), 0.0333333333333), (-0.1666666666667), 0.1666666666667);
            s.store_add_scaled_products3(3, s.ad_value(986), s.ad_value(982), 1.0, s.ad_value(987), s.ad_value(983), 1.0, A::mul3(s.ad_value(986), s.ad_value(987), s.ad_value(978)), A::offset(A::mul(s.ad_value(978), s.ad_value(2)), 1.0), 1.0);
            s.store_div_scaled_product3_by_product(989, s.ad_value(982), s.ad_value(983), s.ad_value(988), 1.0, s.ad_value(978), s.ad_value(3), 1.0);
        }

        s.store_add_ad_rhs(990, 975, A::ln(s.ad_value(978)));

        s.store_scaled_add(991, 913, 978, 0.5);

        s.store_sub(992, 990, 926);

        s.v[995] = 1.0;

        s.b[1193] = (p.p9 > 0.0);
        s.v[1193] = if s.b[1193] { 1.0 } else { 0.0 };

        if s.b[1193] {
            s.store_div_scaled_inputs2_indices(993, 914, 0.5, 979, 0.5, 907, 1.0);
            s.store_scaled_add_offset_sqrt_square_offset(993, 993, 1e-5, (-1e-5), 1.0, 0.5);
            s.store_sub_scaled_ad_lhs(1, A::sqrt(A::add_scaled_product(A::div(s.ad_value(993), s.ad_value(223)), 1.0, s.ad_value(246), s.ad_value(246), 0.25)), 246, 0.5);
            s.store_mul_powf_ad_lhs(994, s.ad_value(1), 2.0, 223);
            s.store_sub_from_scalar_div_indices(995, 1.0, 994, 993);
        }

        s.b[1194] = ((s.v[979] / 2.0) < 80.0);
        s.v[1194] = if s.b[1194] { 1.0 } else { 0.0 };

        if s.b[1194] {
            s.store_ln_one_plus_exp_scaled_input(2, 979, 0.5);
        }

        if (!s.b[1194]) {
            s.store_scale(2, 979, 0.5);
        }

        s.store_scale(996, 2, 2.0);

        s.b[1195] = ((s.v[980] / 2.0) < 80.0);
        s.v[1195] = if s.b[1195] { 1.0 } else { 0.0 };

        if s.b[1195] {
            s.store_ln_one_plus_exp_scaled_input(3, 980, 0.5);
        }

        if (!s.b[1195]) {
            s.store_scale(3, 980, 0.5);
        }

        s.store_scale(997, 3, 2.0);

        s.store_sub(998, 997, 980);

        s.store_sub(999, 996, 979);

        s.store_add_scaled_products_indices(1000, 266, 996, 1.0, 267, 998, 1.0);

        s.store_add_scaled_products_indices(1001, 266, 997, 1.0, 267, 999, 1.0);

        s.store_scaled_add(1002, 927, 996, 0.5);

        s.store_scaled_add(1003, 928, 997, 0.5);

        s.store_div_from_scalar_add_ad(0, 1.0, s.ad_value(1002), s.ad_value(1003));

        s.store_mul3_lhs(1004, 991, 1002, 0);

        s.store_mul3_lhs(1005, 991, 1003, 0);

        s.store_scaled_add(1006, 929, 998, 0.5);

        s.store_scaled_add(1007, 930, 999, 0.5);

        s.store_scaled_add(1008, 931, 1000, 0.5);

        s.store_scaled_add(1009, 932, 1001, 0.5);

        s.store_mul_product3_rhs(1010, 995, s.ad_value(1002), s.ad_value(187), A::exp(A::mul(s.ad_value(40), s.ad_value(291))), 1.0);

        s.store_mul_ad_product_rhs(1011, 1003, s.ad_value(188), A::exp(A::mul(s.ad_value(40), s.ad_value(291))));

        s.store_add(1012, 1010, 1011);

        s.store_mul_add_scaled_product_rhs(2, 50, s.ad_value(1006), 1.0, s.ad_value(51), s.ad_value(1007), 1.0);

        s.store_scaled_add_offset_sqrt_square_offset(3, 2, 1.0, 1.0, 0.01, 0.5);

        s.store_scaled_add_ad(4, A::scale_offset(s.ad_value(2), 0.2, 1.0), A::sqrt(A::offset(A::mul(A::scale_offset(s.ad_value(2), 0.2, 1.0), A::scale_offset(s.ad_value(2), 0.2, 1.0)), 0.01)), 0.5);

        s.store_div(1013, 3, 4);

        s.store_mul_ad_product_rhs(1014, 33, A::add_scaled_product(A::offset(A::mul(s.ad_value(41), s.ad_value(1006)), 1.0), 1.0, s.ad_value(42), s.ad_value(1007), 1.0), A::exp(A::mul_scaled_lhs(s.ad_value(44), -1.0, A::ln(A::add_scaled_product(A::offset(A::mul(s.ad_value(1004), s.ad_value(264)), 1.0), 1.0, s.ad_value(1005), s.ad_value(265), 1.0)))));

        s.b[1196] = (s.v[56] == 0.0);
        s.v[1196] = if s.b[1196] { 1.0 } else { 0.0 };

        if s.b[1196] {
            s.store_scalar(4, 1.0);
        }

        s.b[1197] = (s.v[56] < 0.0);
        s.v[1197] = if s.b[1197] { 1.0 } else { 0.0 };

        if ((!s.b[1196]) && s.b[1197]) {
            s.store_mul_exp_ad_rhs(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(991), 1e-12))));
            s.store_sub_from_scalar(4, 1.0, 2);
        }

        if ((!s.b[1196]) && (!s.b[1197])) {
            s.store_mul_exp_ad_rhs(2, 56, A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(991), 1e-12))));
            s.store_div_from_scalar_offset_input(4, 1.0, 2, 1.0);
        }

        s.store_mul_add_scaled_product_rhs(1015, 939, s.ad_value(54), 1.0, s.ad_value(991), s.ad_value(4), 1.0);

        s.store_add_scaled_inputs_product_first_ad(1016, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(1008)), 1e-6)))), 1.0), 1.0, 1014, 1.0, 38, 1015, 1.0);

        s.store_add_scaled_inputs_product_first_ad(1017, A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(263), s.ad_value(1009)), 1e-6)))), 1.0), 1.0, 1014, 1.0, 39, 1015, 1.0);

        s.store_div_scaled_product_add_scaled_denominator(1018, 1013, 1012, 1.0, A::div(s.ad_value(1010), s.ad_value(1016)), 1.0, A::div(s.ad_value(1011), s.ad_value(1017)), 1.0, 1.0);

        s.store_div_from_scalar_offset_input(1019, 1.0, 991, 4.0);

        s.b[1198] = (s.v[65] > 0.0);
        s.v[1198] = if s.b[1198] { 1.0 } else { 0.0 };

        if s.b[1198] {
            s.store_div_from_scalar_offset_ad(0, 1.0, A::mul(s.ad_value(65), s.ad_value(1005)), 1.0);
        }

        if (!s.b[1198]) {
            s.store_sub_from_scalar_scaled_mul(0, 1.0, 65, 1005, 1.0);
        }

        s.store_mul3_lhs(1020, 991, 1019, 0);

        s.store_mul_ln_ad_lhs(1021, A::offset(A::div_scaled_inputs2(s.ad_value(335), 1.0, s.ad_value(975), (-1.0), A::add_scaled_product(A::mul3(s.ad_value(67), s.ad_value(991), s.ad_value(991)), 1.0, s.ad_value(66), s.ad_value(223), 1.0), 1.0), 1.0), 1020);

        s.store_mul(1022, 873, 1021);

        s.store_div_from_scalar_offset_ad(1023, 1.0, A::mul_offset_rhs(s.ad_value(1022), s.ad_value(1022), 1.0), 1.0);

        s.store_div_scaled_value_offset_denominator(951, s.ad_value(1002), 100.0, s.ad_value(1002), 100.0, 1.0);

        s.b[1199] = (s.v[61] < 0.0);
        s.v[1199] = if s.b[1199] { 1.0 } else { 0.0 };

        if s.b[1199] {
            s.store_div_from_scalar_sub_from_scalar_ad(952, 1.0, 1.0, A::mul(s.ad_value(61), s.ad_value(951)));
        }

        if (!s.b[1199]) {
            s.store_offset_mul(952, 61, 951, 1.0);
        }

        s.store_div_scaled_value_offset_denominator(953, s.ad_value(1003), 100.0, s.ad_value(1003), 100.0, 1.0);

        s.b[1200] = (s.v[62] < 0.0);
        s.v[1200] = if s.b[1200] { 1.0 } else { 0.0 };

        if s.b[1200] {
            s.store_div_from_scalar_sub_from_scalar_ad(954, 1.0, 1.0, A::mul(s.ad_value(62), s.ad_value(953)));
        }

        if (!s.b[1200]) {
            s.store_offset_mul(954, 62, 953, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_mul_ad_affine_product_rhs(1024, 871, s.ad_value(992), A::add(s.ad_value(952), s.ad_value(954)), 0.5, 0.0);

        s.store_div_ad_rhs(1025, 1024, A::mul(s.ad_value(1018), s.ad_value(1023)));

        s.store_square(1026, 1025);

        s.store_sqrt_offset_input(1027, 1026, 1.0);

        s.store_div_scaled_offset_numerator(1028, s.ad_value(1026), 1.5, 1.0, s.ad_value(1027), 1.0);

        s.b[1201] = (p.p13 > 0.0);
        s.v[1201] = if s.b[1201] { 1.0 } else { 0.0 };

        if s.b[1201] {
            s.store_mul_scaled_exp_ln_input_rhs(2, 254, 0.6, A::offset(A::square(s.ad_value(1002)), 60.0), (-0.1666666666667));
            s.store_mul_scaled_exp_ln_input_rhs(3, 254, 0.6, A::offset(A::square(s.ad_value(1003)), 60.0), (-0.1666666666667));
            s.store_div_scaled_offset_numerator(1029, A::mul(s.ad_value(907), s.ad_value(2)), 1.0, 1.0, s.ad_value(888), 1.0);
            s.store_div_scaled_offset_numerator(1030, A::mul(s.ad_value(908), s.ad_value(3)), 1.0, 1.0, s.ad_value(889), 1.0);
        }

        if (!s.b[1201]) {
            s.store_scalar(1029, 1.0);
            s.store_scalar(1030, 1.0);
        }

        s.b[1202] = (s.v[913] > 1e-6);
        s.v[1202] = if s.b[1202] { 1.0 } else { 0.0 };

        s.b[1203] = (s.v[978] > 1e-6);
        s.v[1203] = if s.b[1203] { 1.0 } else { 0.0 };

        s.b[1204] = (((s.v[987]) as f64).abs() < 0.01);
        s.v[1204] = if s.b[1204] { 1.0 } else { 0.0 };

        if ((s.b[1202] && s.b[1203]) && s.b[1204]) {
            s.store_div_scaled_inputs2_mixed_aia(0, A::offset(s.ad_value(976), 2.0), 1.0, 986, 0.5, A::mul_offset_lhs(s.ad_value(977), 2.0, s.ad_value(986)), 1.0);
            s.store_mul(2, 0, 987);
            s.store_square(3, 2);
            s.store_add_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
            s.store_add_scaled_product_indices(5, 4, 1.0, 2, 3, (-1.0));
            s.store_div_scaled_inputs2_mixed_iaa(2, 980, 1.0, A::mul3_scaled_output(s.ad_value(981), A::sub(s.ad_value(0), A::div_from_scalar(1.0, s.ad_value(986))), s.ad_value(5), 2.0), (-1.0), A::offset(s.ad_value(977), 2.0), 1.0);
            s.store_div_scaled_inputs2_mixed_aii(1031, A::div_scaled_add_product(s.ad_value(982), (-1.0), s.ad_value(989), s.ad_value(978), 1.0, s.ad_value(986), 1.0), 1.0, 2, (-1.0), 978, 1.0);
            s.store_div_scaled_product_offset_denominator(1032, s.ad_value(1031), s.ad_value(978), 1.0, s.ad_value(1031), 1.0, 1.0);
        }

        if ((s.b[1202] && s.b[1203]) && (!s.b[1204])) {
            s.store_sub_ad(1031, A::div_scaled_product_by_product(s.ad_value(989), s.ad_value(988), 1.0, s.ad_value(986), s.ad_value(987), 1.0), A::div_scaled_inputs2(A::div(s.ad_value(982), s.ad_value(986)), 1.0, A::div(s.ad_value(983), s.ad_value(987)), 1.0, s.ad_value(978), 1.0));
            s.store_div_scaled_product_offset_denominator(1032, s.ad_value(1031), s.ad_value(978), 1.0, s.ad_value(1031), 1.0, 1.0);
        }

        if (s.b[1202] && (!s.b[1203])) {
            s.copy_ad(1032, 949);
        }

        if s.b[1202] {
            s.store_sub(2, 1032, 956);
            s.store_offset_scaled_mul(3, 2, 2, 36.0, 1.0);
        }

        s.b[1205] = (((s.v[2]) as f64).abs() > 0.001);
        s.v[1205] = if s.b[1205] { 1.0 } else { 0.0 };

        if (s.b[1202] && s.b[1205]) {
            s.store_sub(4, 978, 913);
            s.store_add_scaled_product_indices(1033, 4, 1.0, 1032, 992, (-1.0));
            s.store_add_scaled_product_indices(1034, 4, 1.0, 956, 992, (-1.0));
            s.store_sqrt_square_add(1035, 1033, 3);
            s.store_sqrt_square_add(1036, 1034, 3);
            s.store_mul_ad(1037, A::div_from_scalar(0.25, s.ad_value(2)), A::add_scaled_products3(s.ad_value(1036), s.ad_value(1033), 1.0, s.ad_value(1035), s.ad_value(1034), (-1.0), s.ad_value(3), A::ln(A::div_scaled_inputs2(s.ad_value(1034), 1.0, s.ad_value(1036), 1.0, A::add(s.ad_value(1033), s.ad_value(1035)), 1.0)), 1.0));
        }

        if (s.b[1202] && (!s.b[1205])) {
            s.store_mul(4, 992, 2);
            s.store_div_scaled_product3_mixed_iiia(1037, 992, 4, 4, ((-0.25) * 0.1666666666667), A::sqrt(s.ad_value(3)), 1.0);
        }

        if (!s.b[1202]) {
            s.copy_ad(1032, 949);
            s.store_scalar(1037, 0.0);
        }

        s.store_add_scaled_inputs3_mixed_aii(1038, A::add_scaled_product(s.ad_value(1037), 1.0, s.ad_value(991), s.ad_value(992), 1.0), 1.0, 913, 1.0, 978, -1.0);

        s.b[1206] = (s.v[913] > 1e-6);
        s.v[1206] = if s.b[1206] { 1.0 } else { 0.0 };

        s.b[1207] = (s.v[1038] > 1e-30);
        s.v[1207] = if s.b[1207] { 1.0 } else { 0.0 };

        if (s.b[1206] && s.b[1207]) {
            s.store_div_add_scaled_inputs_rhs_mixed_ai(1039, 922, A::div(s.ad_value(918), s.ad_value(913)), 1.0, 925, -1.0);
            s.store_div_add_scaled_inputs_rhs_mixed_ai(1040, 986, A::div(s.ad_value(982), s.ad_value(978)), 1.0, 989, -1.0);
            s.store_div_scaled_inputs2_indices(1041, 1039, 1.0, 1040, (-1.0), 1038, 1.0);
            s.store_div_add_scaled_inputs_rhs_mixed_ai(1042, 923, A::div(s.ad_value(919), s.ad_value(913)), 1.0, 925, -1.0);
            s.store_div_add_scaled_inputs_rhs_mixed_ai(1043, 987, A::div(s.ad_value(983), s.ad_value(978)), 1.0, 989, -1.0);
            s.store_div_scaled_inputs2_indices(1044, 1042, 1.0, 1043, (-1.0), 1038, 1.0);
        }

        if (s.b[1206] && (!s.b[1207])) {
            s.store_scalar(1041, 0.0);
            s.store_scalar(1044, 0.0);
        }

        if (!s.b[1206]) {
            s.store_mul_add_scaled_inputs_rhs(1045, 944, A::div(s.ad_value(881), s.ad_value(947)), (-2.0), s.ad_value(950), (-2.0));
            s.store_mul_add_scaled_inputs_rhs(1046, 945, A::div(s.ad_value(882), s.ad_value(948)), (-2.0), s.ad_value(950), (-2.0));
            s.store_mul_sub_lhs(0, 1046, 1045, 950);
            s.store_mul(2, 1045, 881);
            s.store_mul(3, 1046, 882);
            s.store_add(4, 2, 3);
            s.store_offset_ad(5, A::add_scaled_products(s.ad_value(944), s.ad_value(881), 2.0, s.ad_value(945), s.ad_value(882), 2.0), 3.0);
            s.store_div_scaled_inputs3(1047, s.ad_value(3), 1.0, s.ad_value(0), 1.0, A::div(s.ad_value(4), s.ad_value(947)), -1.0, s.ad_value(5), 1.0);
            s.store_div_scaled_inputs3(1048, s.ad_value(2), 1.0, s.ad_value(0), (-1.0), A::div(s.ad_value(4), s.ad_value(948)), -1.0, s.ad_value(5), 1.0);
            s.store_mul_add_scaled_product_rhs(1041, 947, s.ad_value(950), -1.0, s.ad_value(1047), s.ad_value(947), -1.0);
            s.store_mul_add_scaled_product_rhs(1044, 948, s.ad_value(950), -1.0, s.ad_value(1048), s.ad_value(948), -1.0);
        }

        s.store_mul(1049, 1041, 1028);

        s.store_mul(1050, 1044, 1028);

        s.store_scaled_sub(1051, 979, 914, 0.5);

        s.store_scaled_sub(1052, 980, 915, 0.5);

        s.store_mul(1053, 1051, 1049);

        s.store_mul(1054, 1052, 1050);

        s.copy_ad(379, 875);

        s.copy_ad(380, 879);

        s.copy_ad(381, 880);

        s.copy_ad(382, 881);

        s.copy_ad(383, 882);

        s.copy_ad(384, 909);

        s.copy_ad(385, 910);

        s.copy_ad(386, 894);

        s.copy_ad(387, 893);

        s.copy_ad(388, 912);

        s.copy_ad(389, 897);

        s.copy_ad(390, 898);

        s.copy_ad(391, 899);

        s.copy_ad(392, 900);

        s.copy_ad(393, 901);

        s.copy_ad(394, 904);

        s.copy_ad(395, 906);

        s.copy_ad(396, 905);

        s.copy_ad(397, 907);

        s.copy_ad(398, 908);

        s.copy_ad(399, 913);

        s.copy_ad(400, 914);

        s.copy_ad(401, 915);

        s.copy_ad(402, 926);

        s.copy_ad(403, 956);

        s.copy_ad(404, 979);

        s.copy_ad(405, 980);

        s.copy_ad(407, 975);

        s.copy_ad(408, 976);

        s.copy_ad(409, 978);

        s.copy_ad(410, 990);

        s.copy_ad(411, 991);

        s.copy_ad(412, 995);

        s.copy_ad(413, 1002);

        s.copy_ad(414, 1003);

        s.copy_ad(415, 1004);

        s.copy_ad(416, 1005);

        s.copy_ad(417, 1012);

        s.copy_ad(418, 1018);

        s.copy_ad(419, 1019);

        s.copy_ad(420, 1021);

        s.copy_ad(421, 1023);

        s.copy_ad(422, 1027);

        s.v[423] = s.v[1024];

        s.copy_ad(424, 1026);

        s.copy_ad(425, 1028);

        s.copy_ad(426, 1029);

        s.copy_ad(427, 1030);

        s.copy_ad(428, 1032);

        s.copy_ad(429, 1038);

        s.copy_ad(430, 1049);

        s.copy_ad(431, 1041);

        s.copy_ad(432, 1051);

        s.copy_ad(433, 1052);

        s.copy_ad(434, 1053);

        s.copy_ad(435, 1054);

        s.store_div_scaled_inputs_mixed_ia(338, 417, p.p35, A::add(s.ad_value(413), s.ad_value(414)), 1.0);

        s.store_mul_add_scaled_product_rhs(339, 420, s.ad_value(63), 1.0, s.ad_value(271), s.ad_value(419), 1.0);

        s.store_mul_offset_ad_lhs(340, A::mul_offset_rhs(s.ad_value(339), s.ad_value(339), 1.0), 1.0, 421);

        s.store_mul3_lhs(341, 418, 421, 422);

        s.b[1208] = (p.p13 > 0.0);
        s.v[1208] = if s.b[1208] { 1.0 } else { 0.0 };

        if s.b[1208] {
            s.store_div_scaled_inputs2_mixed_iia(342, 413, 1.0, 414, 1.0, A::add(A::div(s.ad_value(413), s.ad_value(426)), A::div(s.ad_value(414), s.ad_value(427))), 1.0);
        }

        if (!s.b[1208]) {
            s.store_scalar(342, 1.0);
        }

        s.store_mul_square_lhs(343, 222, 338);

        s.store_div_scaled_product_by_product(344, A::mul3(s.ad_value(343), s.ad_value(386), s.ad_value(429)), s.ad_value(340), 1.0, s.ad_value(341), s.ad_value(342), 1.0);

        s.store_mul_neg_lhs(700, 326, 220);

        s.store_mul_neg_lhs(701, 328, 220);

        s.store_add_scaled_product_indices(0, 230, 1.0, 163, 220, p.p14);

        s.store_add(702, 700, 0);

        s.store_add(703, 701, 0);

        s.v[710] = 0.0;

        s.v[711] = 0.0;

        s.v[712] = 0.0;

        s.v[713] = 0.0;

        s.store_div_ad_lhs(704, A::sqrt(A::mul3_scaled_output(s.ad_value(19), s.ad_value(225), s.ad_value(220), (2.0 * 1.602176565e-19))), 237);

        s.store_square(705, 704);

        s.store_offset_scaled(706, 704, 0.707106781186545, 1.0);

        let assign23310_e22886: f64 = (1e-5 * s.v[706]);
        s.v[707] = assign23310_e22886;

        s.store_div_from_scalar(708, 1.0, 706);

        s.store_div_from_scalar_offset_scaled_input(709, 1.0, 704, 0.7324648775608221, 1.25);

        s.b[1209] = (((p.p3 > 0.0) && ((s.v[69] > 0.0) || (s.v[71] > 0.0))) || ((p.p4 > 0.0) && (s.v[89] > 0.0)));
        s.v[1209] = if s.b[1209] { 1.0 } else { 0.0 };

        s.b[1210] = (((s.v[700]) as f64).abs() <= s.v[707]);
        s.v[1210] = if s.b[1210] { 1.0 } else { 0.0 };

        if (s.b[1209] && s.b[1210]) {
            s.store_mul_neg_lhs(710, 700, 708);
        }

        s.b[1211] = (s.v[700] < (-s.v[707]));
        s.v[1211] = if s.b[1211] { 1.0 } else { 0.0 };

        if ((s.b[1209] && (!s.b[1210])) && s.b[1211]) {
            s.store_neg(679, 700);
            s.store_scaled_mul(680, 679, 708, 1.25);
            s.store_scaled_sub_ad(681, A::offset(s.ad_value(680), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(680), (-6.0), A::offset(s.ad_value(680), (-6.0))), 64.0)), 0.5);
            s.store_add_scaled_products_mixed_aaia(682, A::sub(s.ad_value(679), s.ad_value(681)), A::sub(s.ad_value(679), s.ad_value(681)), 1.0, 705, A::offset(s.ad_value(681), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(683, 679, 2.0, 681, (-2.0), 705, -1.0);
            s.store_sub_ad_lhs(684, A::ln(A::div(s.ad_value(682), s.ad_value(705))), 681);
            s.store_add(685, 682, 683);
            s.store_add_scaled_square_product_mixed_iia(686, 685, 1.0, 684, A::add_scaled_product(s.ad_value(682), (-1.0), s.ad_value(683), s.ad_value(683), 0.5), 1.0);
            s.store_add_ad_rhs(687, 686, A::mul3(A::mul3(A::div(s.ad_value(685), s.ad_value(686)), s.ad_value(684), s.ad_value(684)), s.ad_value(683), A::sub_scaled_inputs(A::square(s.ad_value(683)), 0.3333333333333, s.ad_value(682), 1.0)));
            s.store_add_ad_rhs(688, 681, A::div_scaled_product3(s.ad_value(682), s.ad_value(685), s.ad_value(684), 1.0, s.ad_value(687), 1.0));
        }

        s.b[1212] = (((s.v[688]) as f64).abs() < 80.0);
        s.v[1212] = if s.b[1212] { 1.0 } else { 0.0 };

        if (((s.b[1209] && (!s.b[1210])) && s.b[1211]) && s.b[1212]) {
            s.store_exp(689, 688);
        }

        s.b[1213] = (s.v[688] < (-80.0));
        s.v[1213] = if s.b[1213] { 1.0 } else { 0.0 };

        if ((((s.b[1209] && (!s.b[1210])) && s.b[1211]) && (!s.b[1212])) && s.b[1213]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(689, 1.80485e-35, A::neg(s.ad_value(688)), (-80.0), A::offset(A::mul_scaled_lhs(A::offset(A::neg(s.ad_value(688)), (-80.0)), 0.5, A::scale_offset(A::neg(s.ad_value(688)), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0);
        }

        if ((((s.b[1209] && (!s.b[1210])) && s.b[1211]) && (!s.b[1212])) && (!s.b[1213])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(689, 688, (-80.0), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(688), (-80.0)), 0.5, A::scale_offset(s.ad_value(688), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)))), 1.0), 1.0, 5.54062e34);
        }

        if ((s.b[1209] && (!s.b[1210])) && s.b[1211]) {
            s.store_sub(687, 679, 688);
            s.store_add_scaled_offset_product_rhs(690, 687, 2.0, 705, 689, (-1.0), 1.0);
            s.store_add_scaled_square_product_mixed_iia(691, 687, 1.0, 705, A::sub(A::offset(s.ad_value(688), 1.0), s.ad_value(689)), 1.0);
            s.store_sub_from_scalar_scaled_mul(692, 1.0, 705, 689, 0.5);
            s.store_add_scaled_square_product_indices(687, 690, 1.0, 692, 691, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(693, 691, 2.0, A::add(s.ad_value(690), A::sqrt(s.ad_value(687))), 1.0);
            s.store_neg_ad(710, A::add(s.ad_value(688), s.ad_value(693)));
        }

        if ((s.b[1209] && (!s.b[1210])) && (!s.b[1211])) {
            s.store_mul_offset_ad_lhs(694, A::mul_scaled_lhs(s.ad_value(706), 1.25, s.ad_value(709)), (-1.0), 709);
            s.store_mul_ad_product_rhs(695, 700, s.ad_value(708), A::offset(A::mul(s.ad_value(694), s.ad_value(700)), 1.0));
        }

    }
}
