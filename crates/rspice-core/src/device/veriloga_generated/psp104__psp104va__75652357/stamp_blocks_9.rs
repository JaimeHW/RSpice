#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1010] {s.store_primal_scale(465, 463, 2.0);}
        s.store_primal_offset(551, 508, s.v[370]);s.store_primal_offset(552, 509, s.v[370]);s.store_primal_offset(553, 510, s.v[370]);s.store_primal_scale_ad(554, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(548), s.v[363], s.ad_value(551), s.v[365]), 0.5), ((s.v[360]) as f64).powf(1.5));s.store_primal_scale_ad(555, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(549), s.v[363], s.ad_value(552), s.v[365]), 0.5), ((s.v[360]) as f64).powf(1.5));s.store_primal_scale_ad(556, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(550), s.v[363], s.ad_value(553), s.v[365]), 0.5), ((s.v[360]) as f64).powf(1.5));s.store_primal_mul3_lhs(557, 511, 554, 554);s.store_primal_mul3_lhs(558, 512, 555, 555);s.store_primal_mul3_lhs(559, 513, 556, 556);s.store_primal_sub_scaled_inputs_ln_rhs(560, 502, s.v[360], 554, (2.0 * s.v[364]));s.store_primal_sub_scaled_inputs_ln_rhs(561, 503, s.v[360], 555, (2.0 * s.v[364]));s.store_primal_sub_scaled_inputs_ln_rhs(562, 504, s.v[360], 556, (2.0 * s.v[364]));s.store_primal_add_scaled_inputs_mixed_ia(563, 560, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(560), (-s.v[365]), ((0.05) * (s.v[365])))), s.v[364]);s.store_primal_add_scaled_inputs_mixed_ia(564, 561, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(561), (-s.v[365]), ((0.05) * (s.v[365])))), s.v[364]);s.store_primal_add_scaled_inputs_mixed_ia(565, 562, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(562), (-s.v[365]), ((0.05) * (s.v[365])))), s.v[364]);s.store_primal_div_from_scalar(566, 1.0, 563);s.store_primal_div_from_scalar(567, 1.0, 564);s.store_primal_div_from_scalar(568, 1.0, 565);s.store_primal_mul_pow_mixed_iai(575, 499, A::mul(s.ad_value(502), s.ad_value(566)), 505);s.store_primal_mul_pow_mixed_iai(576, 500, A::mul(s.ad_value(503), s.ad_value(567)), 506);s.store_primal_mul_pow_mixed_iai(577, 501, A::mul(s.ad_value(504), s.ad_value(568)), 507);s.store_primal_mul3_lhs(578, 575, 563, 572);s.store_primal_mul3_lhs(579, 576, 564, 573);s.store_primal_mul3_lhs(580, 577, 565, 574);s.store_primal_scale(581, 575, 2.0);s.store_primal_scale(582, 576, 2.0);s.store_primal_scale(583, 577, 2.0);s.store_primal_max_with_scalar_ad(593, A::scale(s.ad_value(551), 0.5), s.v[364]);s.store_primal_max_with_scalar_ad(594, A::scale(s.ad_value(552), 0.5), s.v[364]);s.store_primal_max_with_scalar_ad(595, A::scale(s.ad_value(553), 0.5), s.v[364]);s.store_primal_scale(596, 593, s.v[365]);s.store_primal_scale(597, 594, s.v[365]);s.store_primal_scale(598, 595, s.v[365]);s.store_primal_scaled_sqrt_ad(599, A::mul3_scaled_output(s.ad_value(522), A::square(s.ad_value(593)), s.ad_value(593), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));s.store_primal_scaled_sqrt_ad(600, A::mul3_scaled_output(s.ad_value(523), A::square(s.ad_value(594)), s.ad_value(594), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));s.store_primal_scaled_sqrt_ad(601, A::mul3_scaled_output(s.ad_value(524), A::square(s.ad_value(595)), s.ad_value(595), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));s.store_primal_mul_scale_offset_rhs(602, 528, 531, (s.v[359] - s.v[358]), 1.0);s.store_primal_mul_scale_offset_rhs(603, 529, 532, (s.v[359] - s.v[358]), 1.0);s.store_mul_scale_offset_rhs(604, 530, 533, (s.v[359] - s.v[358]), 1.0);
        if (!(s.v[602] > 0.0)) {s.store_scalar(602, 0.0);}
        if (!(s.v[603] > 0.0)) {s.store_scalar(603, 0.0);}
        if (!(s.v[604] > 0.0)) {s.store_scalar(604, 0.0);}
        s.b[1011] = (s.v[629] == 1.0);s.store_scalar(1011, if s.b[1011] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1011] {s.store_primal_offset(618, 617, s.v[370]);s.store_primal_scale_ad(620, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(619), s.v[363], s.ad_value(618), s.v[365]), 0.5), ((s.v[360]) as f64).powf(1.5));s.store_primal_sub_scaled_inputs_ln_rhs(621, 615, s.v[360], 620, (2.0 * s.v[364]));s.store_primal_add_scaled_inputs_mixed_ia(622, 621, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(621), (-s.v[365]), ((0.05) * (s.v[365])))), s.v[364]);s.store_primal_div_from_scalar(623, 1.0, 622);s.store_primal_mul_pow_mixed_iai(626, 614, A::mul(s.ad_value(615), s.ad_value(623)), 616);s.store_primal_mul3_lhs(627, 626, 622, 625);s.store_primal_scale(628, 626, 2.0);}
        s.store_scalar(1, 1.0);s.store_scalar(2, 1.0);s.store_scalar(306, 0.0);s.store_scalar(307, 0.0);s.store_scalar(3, p.p0);s.store_scalar(4, p.p1);s.store_scalar(5, p.p2);s.store_scalar(6, p.p3);s.store_scalar(7, p.p4);s.store_scalar(8, p.p8);s.store_scalar(640, p.p19);s.store_scalar(641, p.p20);s.store_scalar(642, p.p21);s.store_scalar(667, p.p22);s.store_scalar(668, p.p23);s.store_scalar(669, p.p24);s.store_scalar(643, p.p25);s.store_scalar(644, p.p26);s.store_scalar(670, p.p27);s.store_scalar(671, p.p28);s.store_scalar(10, p.p14);s.b[1012] = (p.p39 > 0.0);s.store_scalar(1012, if s.b[1012] { 1.0 } else { 0.0 });
        if s.b[1012] {s.store_scalar(1, (if (p.p9 > 1.0) { p.p9 } else { 1.0 }));}
        if s.b[1012] {s.store_primal_floor_ad(1, A::offset(s.ad_value(1), 0.5));s.store_primal_div_from_scalar(2, 1.0, 1);}
        if ((s.v[4] * s.v[2]) > 1e-9) {
            s.store_primal_scale(4, 2, s.v[4]);
        } else {
            s.store_scalar(4, 1e-9);
        }
        s.store_scalar(11, p.p5);s.store_scalar(12, p.p6);s.store_scalar(13, p.p7);s.store_scalar(302, (1e-6 / s.v[3]));s.store_primal_div_from_scalar(303, 1e-6, 4);s.store_primal_offset_scaled(304, 303, ((p.p188) * ((p.p186 * (1.0 + (p.p187 * s.v[302]))))), (p.p186 * (1.0 + (p.p187 * s.v[302]))));s.store_primal_offset_scaled(305, 303, ((p.p192) * ((p.p190 * (1.0 + (p.p191 * s.v[302]))))), (p.p190 * (1.0 + (p.p191 * s.v[302]))));
        if (((s.v[3] + s.v[304]) - (2.0 * p.p189)) > 1e-9) {
            s.store_primal_offset(306, 304, ((s.v[3]) + ((-(2.0 * p.p189)))));
        } else {
            s.store_scalar(306, 1e-9);
        }
        if (((s.v[4] + s.v[305]) - (2.0 * p.p193)) > 1e-9) {
            s.store_primal_offset_add(307, 4, 305, (-(2.0 * p.p193)));
        } else {
            s.store_scalar(307, 1e-9);
        }
        s.store_primal_div_from_scalar(308, 1e-6, 306);s.store_primal_square(309, 308);s.store_primal_div_from_scalar(310, 1e-6, 307);s.store_primal_div_from_scalar(311, 1.0, 310);s.store_primal_mul(312, 308, 310);s.store_primal_div_from_scalar(313, 1.0, 312);
        if ((((s.v[3] + s.v[304]) - (2.0 * p.p189)) + p.p194) > 1e-9) {
            s.store_primal_offset(314, 304, ((((s.v[3]) + ((-(2.0 * p.p189))))) + (p.p194)));
        } else {
            s.store_scalar(314, 1e-9);
        }
        if ((((s.v[4] + s.v[305]) - (2.0 * p.p193)) + p.p195) > 1e-9) {
            s.store_primal_offset_add(315, 4, 305, (((-(2.0 * p.p193))) + (p.p195)));
        } else {
            s.store_scalar(315, 1e-9);
        }
        s.store_primal_scale(316, 315, 1000000.0);
        if (((s.v[3] + s.v[304]) + p.p194) > 1e-9) {
            s.store_primal_offset(317, 304, ((s.v[3]) + (p.p194)));
        } else {
            s.store_scalar(317, 1e-9);
        }
        if (((s.v[4] + s.v[305]) + p.p195) > 1e-9) {
            s.store_primal_offset_add(318, 4, 305, p.p195);
        } else {
            s.store_scalar(318, 1e-9);
        }
        s.store_primal_scale(319, 317, 1000000.0);s.store_primal_scale(320, 318, 1000000.0);s.store_scalar(40, p.p56);s.store_scalar(41, p.p57);s.store_scalar(42, p.p58);s.store_scalar(43, p.p59);s.store_scalar(44, p.p60);s.store_scalar(45, p.p61);s.store_scalar(46, p.p62);s.store_scalar(47, p.p63);s.store_scalar(48, p.p64);s.store_scalar(49, p.p65);s.store_scalar(50, p.p66);s.store_scalar(55, p.p67);s.store_scalar(56, p.p68);s.store_scalar(57, p.p69);s.store_scalar(58, p.p70);s.store_scalar(51, p.p71);s.store_scalar(52, p.p73);s.store_scalar(53, p.p72);s.store_scalar(54, p.p74);s.store_scalar(59, p.p78);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_scalar(60, p.p80);s.store_scalar(61, p.p79);s.store_scalar(62, p.p75);s.store_scalar(63, p.p77);s.store_scalar(64, p.p76);s.store_scalar(65, p.p81);s.store_scalar(66, p.p82);s.store_scalar(67, p.p83);s.store_scalar(68, p.p84);s.store_scalar(69, p.p85);s.store_scalar(70, p.p86);s.store_scalar(71, p.p87);s.store_scalar(72, p.p88);s.store_scalar(73, p.p89);s.store_scalar(74, p.p90);s.store_scalar(75, p.p91);s.store_scalar(76, p.p92);s.store_scalar(77, p.p93);s.store_scalar(78, p.p94);s.store_scalar(79, p.p95);s.store_scalar(80, p.p96);s.store_scalar(81, p.p97);s.store_scalar(82, p.p98);s.store_scalar(83, p.p99);s.store_scalar(84, p.p100);s.store_scalar(85, p.p101);s.store_scalar(86, p.p102);s.store_scalar(87, p.p103);s.store_scalar(88, p.p104);s.store_scalar(89, p.p105);s.store_scalar(90, p.p106);s.store_scalar(91, p.p107);s.store_scalar(92, p.p108);s.store_scalar(93, p.p109);s.store_scalar(94, p.p110);s.store_scalar(95, p.p111);s.store_scalar(96, p.p112);s.store_scalar(97, p.p113);s.store_scalar(98, p.p114);s.store_scalar(99, p.p115);s.store_scalar(100, p.p116);s.store_scalar(101, p.p117);s.store_scalar(102, p.p118);s.store_scalar(103, p.p119);s.store_scalar(104, p.p120);s.store_scalar(105, p.p119);s.b[1013] = param_given[121];s.store_scalar(1013, if s.b[1013] { 1.0 } else { 0.0 });
        if s.b[1013] {s.store_scalar(105, p.p121);}
        s.store_scalar(106, p.p120);s.b[1014] = param_given[122];s.store_scalar(1014, if s.b[1014] { 1.0 } else { 0.0 });
        if s.b[1014] {s.store_scalar(106, p.p122);}
        s.copy_ad(107, 105);s.b[1015] = param_given[123];s.store_scalar(1015, if s.b[1015] { 1.0 } else { 0.0 });
        if s.b[1015] {s.store_scalar(107, p.p123);}
        s.copy_ad(108, 106);s.b[1016] = param_given[124];s.store_scalar(1016, if s.b[1016] { 1.0 } else { 0.0 });
        if s.b[1016] {s.store_scalar(108, p.p124);}
        s.store_scalar(109, p.p125);s.store_scalar(110, p.p126);s.store_scalar(111, p.p127);s.store_scalar(112, p.p128);s.store_scalar(113, p.p129);s.store_scalar(114, p.p130);s.store_scalar(115, p.p131);s.store_scalar(116, p.p132);s.store_scalar(117, p.p133);s.store_scalar(118, p.p134);s.store_scalar(119, p.p135);s.store_scalar(120, p.p136);s.store_scalar(121, p.p98);s.b[1017] = param_given[137];s.store_scalar(1017, if s.b[1017] { 1.0 } else { 0.0 });
        if s.b[1017] {s.store_scalar(121, p.p137);}
        s.store_scalar(122, p.p103);s.b[1018] = param_given[138];s.store_scalar(1018, if s.b[1018] { 1.0 } else { 0.0 });
        if s.b[1018] {s.store_scalar(122, p.p138);}
        s.store_scalar(123, p.p139);s.store_scalar(124, p.p140);s.store_scalar(125, p.p141);s.store_scalar(126, p.p142);s.store_scalar(127, p.p143);s.store_scalar(128, p.p144);s.store_scalar(129, p.p145);s.store_scalar(130, p.p146);s.store_scalar(131, p.p147);s.store_scalar(132, p.p148);s.store_scalar(133, p.p149);s.store_scalar(134, p.p150);s.store_scalar(135, p.p151);s.store_scalar(136, p.p152);s.store_scalar(137, p.p153);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scalar(138, p.p154);s.store_scalar(139, p.p155);s.store_scalar(145, p.p161);s.store_scalar(146, p.p162);s.store_scalar(147, p.p163);s.store_scalar(148, p.p164);s.store_scalar(149, p.p165);s.store_scalar(150, p.p166);s.store_scalar(151, p.p167);s.store_scalar(152, p.p168);s.store_scalar(153, p.p169);s.store_scalar(154, p.p170);s.store_scalar(155, p.p171);s.store_scalar(156, p.p173);s.store_scalar(157, p.p172);s.b[1019] = (p.p39 > 0.0);s.store_scalar(1019, if s.b[1019] { 1.0 } else { 0.0 });
        if s.b[1019] {s.store_primal_add_scaled_inputs3_offset_mixed_aii(40, A::powf(s.ad_value(308), p.p198), p.p197, 310, p.p199, 312, p.p200, p.p196);s.store_primal_add_scaled_inputs3_offset_indices(41, 308, p.p202, 310, p.p203, 312, p.p204, p.p201);s.store_scalar(42, p.p205);s.store_scalar(43, p.p206);s.store_scalar(44, p.p207);}
        if s.b[1019] {
            s.store_primal_scale_ad(325, {
                if ((1.0 + ((p.p209 * s.v[310]) * (((1.0 + (s.v[307] / p.p210))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(310), p.p209, A::ln(A::scale_offset(s.ad_value(307), 1.0 / (p.p210), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p208);
        }
        if s.b[1019] {
            s.store_primal_scale_ad(326, {
                if ((1.0 + ((p.p212 * s.v[310]) * (((1.0 + (s.v[307] / p.p213))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(310), p.p212, A::ln(A::scale_offset(s.ad_value(307), 1.0 / (p.p213), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p211);
        }
        if s.b[1019] {
            s.store_primal_scale_ad(327, {
                if ((1.0 + ((p.p215 * s.v[310]) * (((1.0 + (s.v[307] / p.p213))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(310), p.p215, A::ln(A::scale_offset(s.ad_value(307), 1.0 / (p.p213), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p214);
        }
        s.b[1020] = (s.v[306] > (2.0 * s.v[327]));s.store_scalar(1020, if s.b[1020] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1020]) {s.store_scalar(328, 75000000000.0);s.store_primal_sub_ad(329, A::sqrt(A::add_scaled_inputs(s.ad_value(325), 1.0, s.ad_value(326), 0.5)), A::sqrt(s.ad_value(325)));s.store_primal_add_scaled_product_mixed_aia(330, A::sqrt(s.ad_value(325)), 1.0, 328, A::ln(A::offset(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(327), 2.0, s.ad_value(306), 1.0), A::exp(A::div(s.ad_value(329), s.ad_value(328))), (-1.0)), 1.0)), 1.0);s.store_primal_square(330, 330);}
        s.b[1021] = (s.v[306] >= s.v[327]);s.store_scalar(1021, if s.b[1021] { 1.0 } else { 0.0 });
        if ((s.b[1019] && (!s.b[1020])) && s.b[1021]) {s.store_primal_add_mixed_ia(330, 325, A::div_scaled_product(s.ad_value(326), s.ad_value(327), 1.0, s.ad_value(306), 1.0));}
        if ((s.b[1019] && (!s.b[1020])) && (!s.b[1021])) {s.store_primal_add_mixed_ia(330, 325, A::mul_sub_from_scalar_rhs(s.ad_value(326), 2.0, A::div(s.ad_value(306), s.ad_value(327))));}
        if s.b[1019] {s.store_primal_mul_sub_scaled_inputs_rhs_mixed_ai(45, 330, A::sub_from_scalar(1.0, A::scale(s.ad_value(308), p.p216)), 1.0, 309, p.p217);s.store_primal_add_scaled_inputs3_offset_mixed_aii(46, A::powf(s.ad_value(308), p.p220), p.p219, 310, p.p221, 312, p.p222, p.p218);s.store_scalar(47, p.p223);s.store_scalar(48, p.p224);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1019] {s.store_primal_add_scaled_inputs3_offset_mixed_aii(49, A::powf(s.ad_value(308), p.p227), p.p226, 310, p.p228, 312, p.p229, p.p225);}
        if s.b[1019] {
            s.store_primal_scale_ad(50, {
                if (1e-6 > (1.0 + (p.p231 * s.v[308]))) {
                    A::constant(1e-6)
                } else {
                    A::scale_offset(s.ad_value(308), p.p231, 1.0)
                }
            }, p.p230);
        }
        if s.b[1019] {s.store_scalar(55, p.p232);s.store_scalar(56, p.p233);s.store_scalar(57, p.p236);s.store_scalar(58, p.p237);s.store_primal_mul3_ad(51, A::scale_offset(A::powf(s.ad_value(308), p.p240), p.p239, p.p238), A::scale_offset(s.ad_value(310), p.p241, 1.0), A::scale_offset(s.ad_value(312), p.p242, 1.0));s.store_scalar(52, p.p244);s.store_scalar(53, p.p243);s.store_scalar(54, p.p245);s.store_primal_mul_powf_scale_offset_lhs(62, 308, 310, p.p247, (p.p248) * (p.p246), (1.0) * (p.p246));s.store_scalar(63, p.p250);s.store_scalar(64, p.p249);s.store_primal_mul_powf_scale_offset_lhs(59, 308, 310, p.p252, (p.p253) * (p.p251), (1.0) * (p.p251));s.store_scalar(60, p.p255);s.store_scalar(61, p.p254);s.store_primal_offset_scaled(331, 310, ((p.p258) * (p.p257)), p.p257);}
        if s.b[1019] {
            s.store_primal_scale_ad(332, {
                if ((1.0 + (p.p260 * s.v[310])) > 0.001) {
                    A::scale_offset(s.ad_value(310), p.p260, 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p259);
        }
        if s.b[1019] {s.store_primal_add_ad(333, A::offset(A::mul_sub_from_scalar_rhs(A::div_scaled_product(s.ad_value(331), s.ad_value(332), 1.0, s.ad_value(306), 1.0), 1.0, A::exp_div_scaled_inputs(s.ad_value(306), -1.0, s.ad_value(332), 1.0)), 1.0), A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p261 * p.p262), s.ad_value(306)), 1.0, A::exp_scaled_input(s.ad_value(306), (-1.0 / (p.p262)))));}
        if s.b[1019] {
            if (s.v[333] > 1e-15) {
            } else {
                s.store_scalar(333, 1e-15);
            }
        }
        if s.b[1019] {s.store_primal_add_scaled_product_mixed_aia(334, A::scale_offset(s.ad_value(310), p.p263, 1.0), 1.0, 310, A::ln(A::scale_offset(s.ad_value(307), 1.0 / (p.p265), 1.0)), p.p264);s.store_primal_mul_div_scaled_inputs_mixed_iia(65, 334, 307, p.p256, A::mul(s.ad_value(333), s.ad_value(306)), 1.0);s.store_primal_add_scaled_inputs3_offset_indices(66, 308, p.p267, 310, p.p268, 312, p.p269, p.p266);s.store_primal_offset_scaled(67, 310, ((p.p271) * (p.p270)), p.p270);s.store_scalar(68, p.p272);s.store_scalar(69, p.p273);s.store_scalar(70, p.p274);s.store_primal_mul3_ad(71, A::scale_offset(A::powf(s.ad_value(308), p.p277), p.p276, p.p275), A::scale_offset(s.ad_value(310), p.p278, 1.0), A::scale_offset(s.ad_value(312), p.p279, 1.0));s.store_scalar(72, p.p280);s.store_scalar(73, p.p281);s.store_scalar(74, p.p282);s.store_primal_mul3_ad_scaled_output(75, A::scale_offset(s.ad_value(308), p.p284, 1.0), A::scale_offset(s.ad_value(310), p.p285, 1.0), A::scale_offset(s.ad_value(312), p.p286, 1.0), p.p283);s.store_scalar(76, p.p287);s.store_scalar(77, p.p288);s.store_primal_mul_scale_offset_rhs(78, 310, 310, ((p.p290) * (p.p289)), p.p289);s.store_scalar(79, p.p291);s.store_scalar(80, p.p292);s.store_scalar(81, p.p293);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1019] {s.store_primal_mul3_ad(82, A::offset(A::mul(A::div_scaled_inputs(s.ad_value(334), p.p295, s.ad_value(333), 1.0), A::powf(s.ad_value(308), p.p296)), p.p294), A::scale_offset(s.ad_value(310), p.p297, 1.0), A::scale_offset(s.ad_value(312), p.p298, 1.0));s.store_primal_add_scaled_inputs3_offset_indices(83, 308, p.p300, 310, p.p301, 312, p.p302, p.p299);s.store_scalar(84, p.p303);s.store_scalar(85, p.p304);s.store_scalar(86, p.p305);s.store_primal_div_from_scalar_offset_scaled_input(87, p.p306, 308, p.p307, 1.0);s.store_primal_mul_powf_scale_offset_lhs(88, 308, 310, p.p309, (p.p310) * (p.p308), (1.0) * (p.p308));s.store_primal_powf(335, 308, p.p312);s.store_primal_div_scaled_product_offset_denominator_mixed_iaa(89, 335, A::scale_offset(s.ad_value(310), p.p314, 1.0), p.p311, A::mul_scaled_lhs(s.ad_value(308), p.p313, s.ad_value(335)), 1.0, 1.0);s.store_primal_powf(335, 308, p.p316);s.store_primal_div_scaled_product_offset_denominator_mixed_iaa(90, 335, A::scale_offset(s.ad_value(310), p.p318, 1.0), p.p315, A::mul_scaled_lhs(s.ad_value(308), p.p317, s.ad_value(335)), 1.0, 1.0);s.store_scalar(91, p.p319);s.store_primal_scaled_mul_scale_offset_inputs(92, 308, p.p321, 1.0, 310, p.p322, 1.0, p.p320);s.store_scalar(93, p.p323);s.store_scalar(94, p.p324);s.store_primal_scaled_mul_scale_offset_inputs(95, 308, p.p326, 1.0, 310, p.p327, 1.0, p.p325);s.store_primal_scaled_mul_scale_offset_inputs(96, 308, p.p329, 1.0, 310, p.p330, 1.0, p.p328);s.store_scalar(97, p.p331);s.store_scalar(98, p.p332);s.store_primal_div_from_scalar(99, p.p333, 312);s.store_primal_div_from_scalar_scaled_input(100, (p.p334 * p.p234), 310, 1e-6);s.store_primal_div_from_scalar_scaled_input(101, (p.p335 * p.p235), 310, 1e-6);s.store_scalar(102, p.p336);s.store_scalar(103, p.p337);s.store_scalar(104, p.p338);s.store_scalar(105, p.p337);}
        s.b[1022] = param_given[339];s.store_scalar(1022, if s.b[1022] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1022]) {s.store_scalar(105, p.p339);}
        if s.b[1019] {s.store_scalar(106, p.p338);}
        s.b[1023] = param_given[340];s.store_scalar(1023, if s.b[1023] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1023]) {s.store_scalar(106, p.p340);}
        if s.b[1019] {s.copy_ad(107, 105);}
        s.b[1024] = param_given[341];s.store_scalar(1024, if s.b[1024] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1024]) {s.store_scalar(107, p.p341);}
        if s.b[1019] {s.copy_ad(108, 106);}
        s.b[1025] = param_given[342];s.store_scalar(1025, if s.b[1025] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1025]) {s.store_scalar(108, p.p342);}
        if s.b[1019] {s.store_scalar(109, p.p343);s.store_primal_div_from_scalar_scaled_input(110, (p.p344 * p.p234), 310, 1e-6);s.store_primal_div_from_scalar_scaled_input(111, (p.p345 * p.p235), 310, 1e-6);s.store_scalar(112, p.p346);s.store_scalar(113, p.p347);s.store_scalar(114, p.p348);s.store_scalar(115, p.p349);s.store_scalar(116, p.p350);s.store_scalar(117, p.p351);s.store_primal_scaled_mul(118, 315, 314, ((8.8541878176e-12 * p.p207) * 1.0 / (p.p206)));s.store_primal_scale(125, 315, ((8.8541878176e-12 * p.p207) * (p.p234 * 1.0 / (p.p232))));s.store_primal_scale(126, 315, ((8.8541878176e-12 * p.p207) * (p.p235 * 1.0 / (p.p233))));s.store_primal_add_scaled_inputs3_offset_mixed_aii(119, A::powf(s.ad_value(308), p.p354), p.p353, 310, p.p355, 312, p.p356, p.p352);s.store_primal_add_scaled_inputs3_offset_indices(120, 308, p.p358, 310, p.p359, 312, p.p360, p.p357);s.store_scalar(32, p.p294);}
        s.b[1026] = param_given[361];s.store_scalar(1026, if s.b[1026] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1026]) {s.store_scalar(32, p.p361);}
        if s.b[1019] {s.store_scalar(33, p.p295);}
        s.b[1027] = param_given[362];s.store_scalar(1027, if s.b[1027] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1027]) {s.store_scalar(33, p.p362);}
        if s.b[1019] {s.store_scalar(34, p.p296);}
        s.b[1028] = param_given[363];s.store_scalar(1028, if s.b[1028] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1028]) {s.store_scalar(34, p.p363);}
        if s.b[1019] {s.store_scalar(35, p.p297);}
        s.b[1029] = param_given[364];s.store_scalar(1029, if s.b[1029] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1029]) {s.store_scalar(35, p.p364);}
        if s.b[1019] {s.store_scalar(36, p.p298);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1030] = param_given[365];s.store_scalar(1030, if s.b[1030] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1030]) {s.store_scalar(36, p.p365);}
        if s.b[1019] {s.store_primal_mul3_ad(121, A::add_scaled_product(s.ad_value(32), 1.0, A::div_scaled_product(s.ad_value(33), s.ad_value(334), 1.0, s.ad_value(333), 1.0), A::pow(s.ad_value(308), s.ad_value(34)), 1.0), A::offset(A::mul(s.ad_value(35), s.ad_value(310)), 1.0), A::offset(A::mul(s.ad_value(36), s.ad_value(312)), 1.0));s.store_scalar(37, p.p306);}
        s.b[1031] = param_given[366];s.store_scalar(1031, if s.b[1031] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1031]) {s.store_scalar(37, p.p366);}
        if s.b[1019] {s.store_scalar(38, p.p307);}
        s.b[1032] = param_given[367];s.store_scalar(1032, if s.b[1032] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1032]) {s.store_scalar(38, p.p367);}
        if s.b[1019] {s.store_primal_div_scaled_value_offset_denominator(122, s.ad_value(37), 1.0, A::mul(s.ad_value(38), s.ad_value(308)), 1.0, 1.0);s.store_primal_mul_powf_scale_offset_lhs(123, 308, 310, p.p369, (p.p370) * (p.p368), (1.0) * (p.p368));s.store_primal_powf(335, 308, p.p372);s.store_primal_div_scaled_product_offset_denominator_mixed_iaa(124, 335, A::scale_offset(s.ad_value(310), p.p374, 1.0), p.p371, A::mul_scaled_lhs(s.ad_value(308), p.p373, s.ad_value(335)), 1.0, 1.0);s.store_scalar(127, p.p375);s.store_scalar(128, p.p376);s.store_scalar(129, p.p377);s.store_primal_scale(130, 319, p.p378);s.store_primal_scale(131, 316, p.p379);s.store_primal_scale(132, 316, p.p380);s.store_scalar(133, p.p381);s.store_scalar(134, p.p382);s.store_scalar(135, p.p383);s.store_scalar(136, p.p384);s.store_primal_scale(137, 320, p.p385);s.store_primal_scale(138, 320, p.p386);s.store_primal_sub_from_scalar_ad(1001, 1.0, A::div_from_scalar((2.0 * p.p393), s.ad_value(306)));s.store_scalar(139, p.p387);s.store_primal_offset_scaled(338, 307, p.p396, (2.0 * p.p395));s.store_scalar(145, p.p397);s.store_primal_add_scaled_inputs3_offset_indices(146, 308, p.p399, 310, p.p400, 312, p.p401, p.p398);s.store_primal_add_scaled_inputs3_offset_mixed_aii(147, A::powf(s.ad_value(308), p.p404), p.p403, 310, p.p405, 312, p.p406, p.p402);s.store_primal_mul3_ad_scaled_output(148, A::scale_offset(A::powf(s.ad_value(308), p.p409), p.p408, 1.0), A::scale_offset(s.ad_value(310), p.p410, 1.0), A::scale_offset(s.ad_value(312), p.p411, 1.0), p.p407);s.store_primal_offset_scaled_ad(149, A::powf(s.ad_value(308), p.p414), p.p413, p.p412);s.store_primal_offset_ad(341, A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p415 * p.p416), s.ad_value(306)), 1.0, A::exp_scaled_input(s.ad_value(306), (-1.0 / (p.p416)))), 1.0);}
        if s.b[1019] {
            if (s.v[341] > 1e-15) {
            } else {
                s.store_scalar(341, 1e-15);
            }
        }
        if s.b[1019] {s.store_primal_mul_div_scaled_inputs_mixed_aia(150, A::scale_offset(s.ad_value(310), p.p417, 1.0), 338, p.p256, A::mul(s.ad_value(341), s.ad_value(306)), 1.0);s.store_primal_add_scaled_inputs3_offset_indices(151, 308, p.p419, 310, p.p420, 312, p.p421, p.p418);s.store_primal_mul_powf_scale_offset_lhs(152, 308, 310, p.p423, (p.p424) * (p.p422), (1.0) * (p.p422));s.store_scalar(153, p.p425);s.store_scalar(154, p.p426);s.store_primal_mul_powf_scale_offset_lhs(155, 308, 310, p.p428, (p.p429) * (p.p427), (1.0) * (p.p427));s.store_scalar(156, p.p431);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1019] {s.store_scalar(157, p.p430);s.store_primal_add_scaled_inputs3_offset_indices(342, 308, p.p808, 310, p.p809, 312, p.p810, p.p807);s.store_primal_add_scaled_inputs3_offset_indices(343, 308, p.p812, 310, p.p813, 312, p.p814, p.p811);}
        s.b[1034] = (((param_given[448] || param_given[449]) || param_given[450]) || param_given[451]);s.store_scalar(1034, if s.b[1034] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1034]) {s.store_primal_add_scaled_inputs3_offset_indices(40, 308, p.p449, 310, p.p450, 312, p.p451, p.p448);}
        s.b[1035] = (((param_given[452] || param_given[453]) || param_given[454]) || param_given[455]);s.store_scalar(1035, if s.b[1035] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1035]) {s.store_primal_add_scaled_inputs3_offset_indices(41, 308, p.p453, 310, p.p454, 312, p.p455, p.p452);}
        s.b[1036] = (((param_given[456] || param_given[457]) || param_given[458]) || param_given[459]);s.store_scalar(1036, if s.b[1036] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1036]) {s.store_primal_add_scaled_inputs3_offset_indices(45, 308, p.p457, 310, p.p458, 312, p.p459, p.p456);}
        s.b[1037] = (((param_given[460] || param_given[461]) || param_given[462]) || param_given[463]);s.store_scalar(1037, if s.b[1037] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1037]) {s.store_primal_add_scaled_inputs3_offset_indices(46, 308, p.p461, 310, p.p462, 312, p.p463, p.p460);}
        s.b[1038] = (((param_given[464] || param_given[465]) || param_given[466]) || param_given[467]);s.store_scalar(1038, if s.b[1038] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1038]) {s.store_primal_add_scaled_inputs3_offset_indices(47, 308, p.p465, 310, p.p466, 312, p.p467, p.p464);}
        s.b[1039] = (((param_given[468] || param_given[469]) || param_given[470]) || param_given[471]);s.store_scalar(1039, if s.b[1039] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1039]) {s.store_primal_add_scaled_inputs3_offset_indices(49, 308, p.p469, 310, p.p470, 312, p.p471, p.p468);}
        s.b[1040] = (((param_given[472] || param_given[473]) || param_given[474]) || param_given[475]);s.store_scalar(1040, if s.b[1040] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1040]) {s.store_primal_add_scaled_inputs3_offset_indices(50, 308, p.p473, 310, p.p474, 312, p.p475, p.p472);}
        s.b[1041] = (((param_given[476] || param_given[477]) || param_given[478]) || param_given[479]);s.store_scalar(1041, if s.b[1041] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1041]) {s.store_primal_add_scaled_inputs3_offset_indices(57, 308, p.p477, 310, p.p478, 312, p.p479, p.p476);}
        s.b[1042] = (((param_given[480] || param_given[481]) || param_given[482]) || param_given[483]);s.store_scalar(1042, if s.b[1042] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1042]) {s.store_primal_add_scaled_inputs3_offset_indices(58, 308, p.p481, 310, p.p482, 312, p.p483, p.p480);}
        s.b[1043] = (((param_given[484] || param_given[485]) || param_given[486]) || param_given[487]);s.store_scalar(1043, if s.b[1043] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1043]) {s.store_primal_add_scaled_inputs3_offset_indices(51, 308, p.p485, 310, p.p486, 312, p.p487, p.p484);}
        s.b[1044] = (((param_given[492] || param_given[493]) || param_given[494]) || param_given[495]);s.store_scalar(1044, if s.b[1044] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1044]) {s.store_primal_add_scaled_inputs3_offset_indices(52, 308, p.p493, 310, p.p494, 312, p.p495, p.p492);}
        s.b[1045] = (((param_given[488] || param_given[489]) || param_given[490]) || param_given[491]);s.store_scalar(1045, if s.b[1045] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1045]) {s.store_primal_add_scaled_inputs3_offset_indices(53, 308, p.p489, 310, p.p490, 312, p.p491, p.p488);}
        s.b[1046] = (((param_given[496] || param_given[497]) || param_given[498]) || param_given[499]);s.store_scalar(1046, if s.b[1046] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1046]) {s.store_primal_add_scaled_inputs3_offset_indices(54, 308, p.p497, 310, p.p498, 312, p.p499, p.p496);}
        s.b[1047] = (((param_given[500] || param_given[501]) || param_given[502]) || param_given[503]);s.store_scalar(1047, if s.b[1047] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1047]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(62, 309, 308, p.p501, 310, p.p502, 312, p.p503, p.p500);}
        s.b[1048] = (((param_given[508] || param_given[509]) || param_given[510]) || param_given[511]);s.store_scalar(1048, if s.b[1048] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1048]) {s.store_primal_add_scaled_inputs3_offset_indices(63, 308, p.p509, 310, p.p510, 312, p.p511, p.p508);}
        s.b[1049] = (((param_given[504] || param_given[505]) || param_given[506]) || param_given[507]);s.store_scalar(1049, if s.b[1049] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1049]) {s.store_primal_add_scaled_inputs3_offset_indices(64, 308, p.p505, 310, p.p506, 312, p.p507, p.p504);}
        s.b[1050] = (((param_given[512] || param_given[513]) || param_given[514]) || param_given[515]);s.store_scalar(1050, if s.b[1050] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1050]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(59, 309, 308, p.p513, 310, p.p514, 312, p.p515, p.p512);}
        s.b[1051] = (((param_given[520] || param_given[521]) || param_given[522]) || param_given[523]);s.store_scalar(1051, if s.b[1051] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1051]) {s.store_primal_add_scaled_inputs3_offset_indices(60, 308, p.p521, 310, p.p522, 312, p.p523, p.p520);}
        s.b[1052] = (((param_given[516] || param_given[517]) || param_given[518]) || param_given[519]);s.store_scalar(1052, if s.b[1052] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1052]) {s.store_primal_add_scaled_inputs3_offset_indices(61, 308, p.p517, 310, p.p518, 312, p.p519, p.p516);}
        s.b[1053] = (((param_given[524] || param_given[525]) || param_given[526]) || param_given[527]);s.store_scalar(1053, if s.b[1053] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1053]) {s.store_primal_mul_div_scaled_inputs_mixed_aii(65, A::add_scaled_inputs3_offset(s.ad_value(308), p.p525, s.ad_value(310), p.p526, s.ad_value(312), p.p527, p.p524), 307, 1.0, 306, 1.0);}
        s.b[1054] = (((param_given[528] || param_given[529]) || param_given[530]) || param_given[531]);s.store_scalar(1054, if s.b[1054] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1054]) {s.store_primal_add_scaled_inputs3_offset_indices(66, 308, p.p529, 310, p.p530, 312, p.p531, p.p528);}
        s.b[1055] = (((param_given[532] || param_given[533]) || param_given[534]) || param_given[535]);s.store_scalar(1055, if s.b[1055] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1055]) {s.store_primal_add_scaled_inputs3_offset_indices(67, 308, p.p533, 310, p.p534, 312, p.p535, p.p532);}
        s.b[1056] = (((param_given[536] || param_given[537]) || param_given[538]) || param_given[539]);s.store_scalar(1056, if s.b[1056] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1056]) {s.store_primal_add_scaled_inputs3_offset_indices(69, 308, p.p537, 310, p.p538, 312, p.p539, p.p536);}
        s.b[1057] = (((param_given[540] || param_given[541]) || param_given[542]) || param_given[543]);s.store_scalar(1057, if s.b[1057] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1057]) {s.store_primal_add_scaled_inputs3_offset_indices(71, 308, p.p541, 310, p.p542, 312, p.p543, p.p540);}
        s.b[1058] = (((param_given[544] || param_given[545]) || param_given[546]) || param_given[547]);s.store_scalar(1058, if s.b[1058] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1058]) {s.store_primal_add_scaled_inputs3_offset_indices(73, 308, p.p545, 310, p.p546, 312, p.p547, p.p544);}
        s.b[1059] = (((param_given[548] || param_given[549]) || param_given[550]) || param_given[551]);s.store_scalar(1059, if s.b[1059] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1059]) {s.store_primal_add_scaled_inputs3_offset_indices(75, 308, p.p549, 310, p.p550, 312, p.p551, p.p548);}
        s.b[1060] = (((param_given[552] || param_given[553]) || param_given[554]) || param_given[555]);s.store_scalar(1060, if s.b[1060] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1060]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(78, 310, 308, p.p553, 310, p.p554, 312, p.p555, p.p552);}
        s.b[1061] = (((param_given[556] || param_given[557]) || param_given[558]) || param_given[559]);s.store_scalar(1061, if s.b[1061] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1061]) {s.store_primal_add_scaled_inputs3_offset_indices(79, 308, p.p557, 310, p.p558, 312, p.p559, p.p556);}
        s.b[1062] = (((param_given[560] || param_given[561]) || param_given[562]) || param_given[563]);s.store_scalar(1062, if s.b[1062] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1019] && s.b[1062]) {s.store_primal_add_scaled_inputs3_offset_indices(80, 308, p.p561, 310, p.p562, 312, p.p563, p.p560);}
        s.b[1063] = (((param_given[564] || param_given[565]) || param_given[566]) || param_given[567]);s.store_scalar(1063, if s.b[1063] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1063]) {s.store_primal_add_scaled_inputs3_offset_indices(81, 308, p.p565, 310, p.p566, 312, p.p567, p.p564);}
        s.b[1064] = (((param_given[568] || param_given[569]) || param_given[570]) || param_given[571]);s.store_scalar(1064, if s.b[1064] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1064]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(82, 308, 308, p.p569, 310, p.p570, 312, p.p571, p.p568);}
        s.b[1065] = (((param_given[572] || param_given[573]) || param_given[574]) || param_given[575]);s.store_scalar(1065, if s.b[1065] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1065]) {s.store_primal_add_scaled_inputs3_offset_indices(83, 308, p.p573, 310, p.p574, 312, p.p575, p.p572);}
        s.b[1066] = (((param_given[576] || param_given[577]) || param_given[578]) || param_given[579]);s.store_scalar(1066, if s.b[1066] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1066]) {s.store_primal_add_scaled_inputs3_offset_indices(84, 308, p.p577, 310, p.p578, 312, p.p579, p.p576);}
        s.b[1067] = (((param_given[580] || param_given[581]) || param_given[582]) || param_given[583]);s.store_scalar(1067, if s.b[1067] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1067]) {s.store_primal_add_scaled_inputs3_offset_indices(85, 308, p.p581, 310, p.p582, 312, p.p583, p.p580);}
        s.b[1068] = (((param_given[584] || param_given[585]) || param_given[586]) || param_given[587]);s.store_scalar(1068, if s.b[1068] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1068]) {s.store_primal_add_scaled_inputs3_offset_indices(87, 308, p.p585, 310, p.p586, 312, p.p587, p.p584);}
        s.b[1069] = (((param_given[588] || param_given[589]) || param_given[590]) || param_given[591]);s.store_scalar(1069, if s.b[1069] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1069]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(88, 308, 308, p.p589, 310, p.p590, 312, p.p591, p.p588);}
        s.b[1070] = (((param_given[592] || param_given[593]) || param_given[594]) || param_given[595]);s.store_scalar(1070, if s.b[1070] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1070]) {s.store_primal_add_scaled_inputs3_offset_indices(89, 308, p.p593, 310, p.p594, 312, p.p595, p.p592);}
        s.b[1071] = (((param_given[596] || param_given[597]) || param_given[598]) || param_given[599]);s.store_scalar(1071, if s.b[1071] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1071]) {s.store_primal_add_scaled_inputs3_offset_indices(90, 308, p.p597, 310, p.p598, 312, p.p599, p.p596);}
        s.b[1072] = (((param_given[600] || param_given[601]) || param_given[602]) || param_given[603]);s.store_scalar(1072, if s.b[1072] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1072]) {s.store_primal_add_scaled_inputs3_offset_indices(92, 308, p.p601, 310, p.p602, 312, p.p603, p.p600);}
        s.b[1073] = (((param_given[604] || param_given[605]) || param_given[606]) || param_given[607]);s.store_scalar(1073, if s.b[1073] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1073]) {s.store_primal_add_scaled_inputs3_offset_indices(94, 308, p.p605, 310, p.p606, 312, p.p607, p.p604);}
        s.b[1074] = (((param_given[608] || param_given[609]) || param_given[610]) || param_given[611]);s.store_scalar(1074, if s.b[1074] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1074]) {s.store_primal_add_scaled_inputs3_offset_indices(95, 308, p.p609, 310, p.p610, 312, p.p611, p.p608);}
        s.b[1075] = (((param_given[612] || param_given[613]) || param_given[614]) || param_given[615]);s.store_scalar(1075, if s.b[1075] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1075]) {s.store_primal_add_scaled_inputs3_offset_indices(96, 308, p.p613, 310, p.p614, 312, p.p615, p.p612);}
        s.b[1076] = (((param_given[616] || param_given[617]) || param_given[618]) || param_given[619]);s.store_scalar(1076, if s.b[1076] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1076]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(99, 313, 308, p.p617, 310, p.p618, 312, p.p619, p.p616);}
        s.b[1077] = (((param_given[620] || param_given[621]) || param_given[622]) || param_given[623]);s.store_scalar(1077, if s.b[1077] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1077]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(100, 311, 308, p.p621, 310, p.p622, 312, p.p623, p.p620);}
        s.b[1078] = (((param_given[624] || param_given[625]) || param_given[626]) || param_given[627]);s.store_scalar(1078, if s.b[1078] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1078]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(101, 311, 308, p.p625, 310, p.p626, 312, p.p627, p.p624);}
        s.b[1079] = (((param_given[628] || param_given[629]) || param_given[630]) || param_given[631]);s.store_scalar(1079, if s.b[1079] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1079]) {s.store_primal_add_scaled_inputs3_offset_indices(102, 308, p.p629, 310, p.p630, 312, p.p631, p.p628);}
        s.b[1080] = (((param_given[632] || param_given[633]) || param_given[634]) || param_given[635]);s.store_scalar(1080, if s.b[1080] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1080]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(110, 311, 308, p.p633, 310, p.p634, 312, p.p635, p.p632);}
        s.b[1081] = (((param_given[636] || param_given[637]) || param_given[638]) || param_given[639]);s.store_scalar(1081, if s.b[1081] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1081]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(111, 311, 308, p.p637, 310, p.p638, 312, p.p639, p.p636);}
        s.b[1082] = (((param_given[640] || param_given[641]) || param_given[642]) || param_given[643]);s.store_scalar(1082, if s.b[1082] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1082]) {s.store_primal_add_scaled_inputs3_offset_indices(114, 308, p.p641, 310, p.p642, 312, p.p643, p.p640);}
        s.b[1083] = (((param_given[644] || param_given[645]) || param_given[646]) || param_given[647]);s.store_scalar(1083, if s.b[1083] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1083]) {s.store_primal_add_scaled_inputs3_offset_indices(115, 308, p.p645, 310, p.p646, 312, p.p647, p.p644);}
        s.b[1084] = (((param_given[648] || param_given[649]) || param_given[650]) || param_given[651]);s.store_scalar(1084, if s.b[1084] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1084]) {s.store_primal_mul_ad_affine_product_rhs(118, 316, s.ad_value(314), A::add_scaled_inputs3_offset(s.ad_value(308), p.p649, s.ad_value(310), p.p650, s.ad_value(312), p.p651, p.p648), 1.0 / (1e-6), 0.0);}
        s.b[1085] = (((param_given[652] || param_given[653]) || param_given[654]) || param_given[655]);s.store_scalar(1085, if s.b[1085] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1085]) {s.store_primal_add_scaled_inputs3_offset_indices(119, 308, p.p653, 310, p.p654, 312, p.p655, p.p652);}
        s.b[1086] = (((param_given[656] || param_given[657]) || param_given[658]) || param_given[659]);s.store_scalar(1086, if s.b[1086] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1086]) {s.store_primal_add_scaled_inputs3_offset_indices(120, 308, p.p657, 310, p.p658, 312, p.p659, p.p656);}
        s.b[1087] = (((((((param_given[660] || param_given[661]) || param_given[662]) || param_given[663]) || param_given[568]) || param_given[569]) || param_given[570]) || param_given[571]);s.store_scalar(1087, if s.b[1087] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1087]) {s.store_scalar(28, p.p568);}
        s.b[1088] = param_given[660];s.store_scalar(1088, if s.b[1088] { 1.0 } else { 0.0 });
        if ((s.b[1019] && s.b[1087]) && s.b[1088]) {s.store_scalar(28, p.p660);}
        if (s.b[1019] && s.b[1087]) {s.store_scalar(29, p.p569);}
        s.b[1089] = param_given[661];s.store_scalar(1089, if s.b[1089] { 1.0 } else { 0.0 });
        if ((s.b[1019] && s.b[1087]) && s.b[1089]) {s.store_scalar(29, p.p661);}
        if (s.b[1019] && s.b[1087]) {s.store_scalar(30, p.p570);}
        s.b[1090] = param_given[662];s.store_scalar(1090, if s.b[1090] { 1.0 } else { 0.0 });
        if ((s.b[1019] && s.b[1087]) && s.b[1090]) {s.store_scalar(30, p.p662);}
        if (s.b[1019] && s.b[1087]) {s.store_scalar(31, p.p571);}
        s.b[1091] = param_given[663];s.store_scalar(1091, if s.b[1091] { 1.0 } else { 0.0 });
        if ((s.b[1019] && s.b[1087]) && s.b[1091]) {s.store_scalar(31, p.p663);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1019] && s.b[1087]) {s.store_primal_mul_mixed_ia(121, 308, A::add_scaled_value_products3(s.ad_value(28), 1.0, s.ad_value(29), s.ad_value(308), 1.0, s.ad_value(30), s.ad_value(310), 1.0, s.ad_value(31), s.ad_value(312), 1.0));}
        s.b[1092] = (((((((param_given[664] || param_given[665]) || param_given[666]) || param_given[667]) || param_given[584]) || param_given[585]) || param_given[586]) || param_given[587]);s.store_scalar(1092, if s.b[1092] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1092]) {s.store_scalar(28, p.p584);}
        s.b[1093] = param_given[664];s.store_scalar(1093, if s.b[1093] { 1.0 } else { 0.0 });
        if ((s.b[1019] && s.b[1092]) && s.b[1093]) {s.store_scalar(28, p.p664);}
        if (s.b[1019] && s.b[1092]) {s.store_scalar(29, p.p585);}
        s.b[1094] = param_given[665];s.store_scalar(1094, if s.b[1094] { 1.0 } else { 0.0 });
        if ((s.b[1019] && s.b[1092]) && s.b[1094]) {s.store_scalar(29, p.p665);}
        if (s.b[1019] && s.b[1092]) {s.store_scalar(30, p.p586);}
        s.b[1095] = param_given[666];s.store_scalar(1095, if s.b[1095] { 1.0 } else { 0.0 });
        if ((s.b[1019] && s.b[1092]) && s.b[1095]) {s.store_scalar(30, p.p666);}
        if (s.b[1019] && s.b[1092]) {s.store_scalar(31, p.p587);}
        s.b[1096] = param_given[667];s.store_scalar(1096, if s.b[1096] { 1.0 } else { 0.0 });
        if ((s.b[1019] && s.b[1092]) && s.b[1096]) {s.store_scalar(31, p.p667);}
        if (s.b[1019] && s.b[1092]) {s.store_primal_add_scaled_value_products3_indices(122, 28, 1.0, 29, 308, 1.0, 30, 310, 1.0, 31, 312, 1.0);}
        s.b[1097] = (((param_given[668] || param_given[669]) || param_given[670]) || param_given[671]);s.store_scalar(1097, if s.b[1097] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1097]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(123, 308, 308, p.p669, 310, p.p670, 312, p.p671, p.p668);}
        s.b[1098] = (((param_given[672] || param_given[673]) || param_given[674]) || param_given[675]);s.store_scalar(1098, if s.b[1098] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1098]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(124, 308, 308, p.p673, 310, p.p674, 312, p.p675, p.p672);}
        s.b[1099] = (((param_given[676] || param_given[677]) || param_given[678]) || param_given[679]);s.store_scalar(1099, if s.b[1099] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1099]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(125, 316, 308, p.p677, 310, p.p678, 312, p.p679, p.p676);}
        s.b[1100] = (((param_given[680] || param_given[681]) || param_given[682]) || param_given[683]);s.store_scalar(1100, if s.b[1100] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1100]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(126, 316, 308, p.p681, 310, p.p682, 312, p.p683, p.p680);}
        s.b[1101] = (((param_given[684] || param_given[685]) || param_given[686]) || param_given[687]);s.store_scalar(1101, if s.b[1101] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1101]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(130, 319, 308, p.p685, 310, p.p686, 312, p.p687, p.p684);}
        s.b[1102] = (((param_given[688] || param_given[689]) || param_given[690]) || param_given[691]);s.store_scalar(1102, if s.b[1102] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1102]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(131, 316, 308, p.p689, 310, p.p690, 312, p.p691, p.p688);}
        s.b[1103] = (((param_given[692] || param_given[693]) || param_given[694]) || param_given[695]);s.store_scalar(1103, if s.b[1103] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1103]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(132, 316, 308, p.p693, 310, p.p694, 312, p.p695, p.p692);}
        s.b[1104] = (((param_given[696] || param_given[697]) || param_given[698]) || param_given[699]);s.store_scalar(1104, if s.b[1104] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1104]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(137, 320, 308, p.p697, 310, p.p698, 312, p.p699, p.p696);}
        s.b[1105] = (((param_given[700] || param_given[701]) || param_given[702]) || param_given[703]);s.store_scalar(1105, if s.b[1105] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1105]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(138, 320, 308, p.p701, 310, p.p702, 312, p.p703, p.p700);}
        s.b[1110] = (((param_given[720] || param_given[721]) || param_given[722]) || param_given[723]);s.store_scalar(1110, if s.b[1110] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1110]) {s.store_primal_add_scaled_inputs3_offset_indices(145, 308, p.p721, 310, p.p722, 312, p.p723, p.p720);}
        s.b[1111] = (((param_given[724] || param_given[725]) || param_given[726]) || param_given[727]);s.store_scalar(1111, if s.b[1111] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1111]) {s.store_primal_add_scaled_inputs3_offset_indices(146, 308, p.p725, 310, p.p726, 312, p.p727, p.p724);}
        s.b[1112] = (((param_given[728] || param_given[729]) || param_given[730]) || param_given[731]);s.store_scalar(1112, if s.b[1112] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1112]) {s.store_primal_add_scaled_inputs3_offset_indices(147, 308, p.p729, 310, p.p730, 312, p.p731, p.p728);}
        s.b[1113] = (((param_given[732] || param_given[733]) || param_given[734]) || param_given[735]);s.store_scalar(1113, if s.b[1113] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1113]) {s.store_primal_add_scaled_inputs3_offset_indices(148, 308, p.p733, 310, p.p734, 312, p.p735, p.p732);}
        s.b[1114] = (((param_given[736] || param_given[737]) || param_given[738]) || param_given[739]);s.store_scalar(1114, if s.b[1114] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1114]) {s.store_primal_add_scaled_inputs3_offset_indices(149, 308, p.p737, 310, p.p738, 312, p.p739, p.p736);}
        s.b[1115] = (((param_given[740] || param_given[741]) || param_given[742]) || param_given[743]);s.store_scalar(1115, if s.b[1115] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1115]) {s.store_primal_mul_div_scaled_inputs_mixed_aii(150, A::add_scaled_inputs3_offset(s.ad_value(308), p.p741, s.ad_value(310), p.p742, s.ad_value(312), p.p743, p.p740), 338, 1.0, 306, 1.0);}
        s.b[1116] = (((param_given[744] || param_given[745]) || param_given[746]) || param_given[747]);s.store_scalar(1116, if s.b[1116] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1116]) {s.store_primal_add_scaled_inputs3_offset_indices(151, 308, p.p745, 310, p.p746, 312, p.p747, p.p744);}
        s.b[1117] = (((param_given[748] || param_given[749]) || param_given[750]) || param_given[751]);s.store_scalar(1117, if s.b[1117] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1117]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(152, 309, 308, p.p749, 310, p.p750, 312, p.p751, p.p748);}
        s.b[1118] = (((param_given[752] || param_given[753]) || param_given[754]) || param_given[755]);s.store_scalar(1118, if s.b[1118] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1118]) {s.store_primal_add_scaled_inputs3_offset_indices(153, 308, p.p753, 310, p.p754, 312, p.p755, p.p752);}
        s.b[1119] = (((param_given[756] || param_given[757]) || param_given[758]) || param_given[759]);s.store_scalar(1119, if s.b[1119] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1119]) {s.store_primal_add_scaled_inputs3_offset_indices(154, 308, p.p757, 310, p.p758, 312, p.p759, p.p756);}
        s.b[1120] = (((param_given[760] || param_given[761]) || param_given[762]) || param_given[763]);s.store_scalar(1120, if s.b[1120] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1120]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(155, 309, 308, p.p761, 310, p.p762, 312, p.p763, p.p760);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1121] = (((param_given[768] || param_given[769]) || param_given[770]) || param_given[771]);s.store_scalar(1121, if s.b[1121] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1121]) {s.store_primal_add_scaled_inputs3_offset_indices(156, 308, p.p769, 310, p.p770, 312, p.p771, p.p768);}
        s.b[1122] = (((param_given[764] || param_given[765]) || param_given[766]) || param_given[767]);s.store_scalar(1122, if s.b[1122] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1122]) {s.store_primal_add_scaled_inputs3_offset_indices(157, 308, p.p765, 310, p.p766, 312, p.p767, p.p764);}
        if s.b[1019] {s.store_scalar(1008, 0.0);s.store_scalar(1009, 0.0);s.store_scalar(1007, 0.0);s.store_scalar(39, p.p788);}
        s.b[1126] = param_given[789];s.store_scalar(1126, if s.b[1126] { 1.0 } else { 0.0 });
        if (s.b[1019] && s.b[1126]) {s.store_scalar(39, p.p789);}
        s.b[1127] = (((s.v[5] > 0.0) && (s.v[6] > 0.0)) && ((s.v[1] == 1.0) || ((s.v[1] > 1.0) && (s.v[7] > 0.0))));s.store_scalar(1127, if s.b[1127] { 1.0 } else { 0.0 });let mut t2: usize = 0;
        while {
            let t0: f64 = (s.v[1] - 0.5);let t1: f64 = if ((s.b[1019] && s.b[1127]) && (s.v[1007] < t0)) { 1.0 } else { 0.0 };
            t1 != 0.0
        } {
            t2 += 1;assert!(t2 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1019] && s.b[1127]) {s.store_primal_add_mixed_ia(1008, 1008, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1007), (s.v[7] + s.v[3]), (s.v[5] + (0.5 * s.v[3])))));s.store_primal_add_mixed_ia(1009, 1009, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1007), (s.v[7] + s.v[3]), (s.v[6] + (0.5 * s.v[3])))));s.store_primal_offset(1007, 1007, 1.0);}
        }
        if (s.b[1019] && s.b[1127]) {s.store_primal_mul(992, 1008, 2);s.store_primal_mul(993, 1009, 2);s.store_scalar(994, (1.0 / (p.p784 + (0.5 * s.v[3]))));s.store_scalar(995, (1.0 / (p.p785 + (0.5 * s.v[3]))));}
        if (s.b[1019] && s.b[1127]) {
            if ((s.v[3] + s.v[304]) > 1e-9) {
                s.store_primal_offset(1005, 304, s.v[3]);
            } else {
                s.store_scalar(1005, 1e-9);
            }
        }
        if (s.b[1019] && s.b[1127]) {
            if (((s.v[4] + s.v[305]) + p.p786) > 1e-9) {
                s.store_primal_offset_add(1006, 4, 305, p.p786);
            } else {
                s.store_scalar(1006, 1e-9);
            }
        }
        if (s.b[1019] && s.b[1127]) {s.store_primal_div_from_scalar_powf_ad(1003, 1.0, s.ad_value(1005), p.p794);s.store_primal_div_from_scalar_powf_ad(1004, 1.0, s.ad_value(1006), p.p795);s.store_primal_add_scaled_inputs_product_mixed_aiii(996, A::scale_offset(s.ad_value(1003), p.p791, 1.0), (1.0 + (p.p790 * (s.v[346] - 1.0))), 1004, (p.p792 * (1.0 + (p.p790 * (s.v[346] - 1.0)))), 1003, 1004, (p.p793 * (1.0 + (p.p790 * (s.v[346] - 1.0)))));s.store_primal_div_scaled_inputs2_indices(997, 992, p.p787, 993, p.p787, 996, 1.0);s.store_primal_div_scaled_inputs2_indices(998, 994, p.p787, 995, p.p787, 996, 1.0);s.store_primal_div_from_scalar_powf_ad(1003, 1.0, s.ad_value(1005), p.p800);s.store_primal_div_from_scalar_powf_ad(1004, 1.0, s.ad_value(1006), p.p801);s.store_primal_add_scaled_inputs_product_mixed_aiii(999, A::scale_offset(s.ad_value(1003), p.p797, 1.0), 1.0, 1004, p.p798, 1003, 1004, p.p799);s.store_primal_add_scaled_inputs4_indices(1001, 992, 1.0, 993, 1.0, 994, -1.0, 995, -1.0);s.store_primal_div_scaled_offset_numerator_mixed_ia(1002, 997, 1.0, 1.0, A::offset(s.ad_value(998), 1.0), 1.0);s.store_primal_mul(65, 65, 1002);s.store_primal_div_scaled_product3_mixed_iiaa(82, 82, 1002, A::scale_offset(s.ad_value(998), p.p788, 1.0), 1.0, A::scale_offset(s.ad_value(997), p.p788, 1.0), 1.0);s.store_primal_div_scaled_product3_mixed_iiaa(121, 121, 1002, A::offset(A::mul(s.ad_value(39), s.ad_value(998)), 1.0), 1.0, A::offset(A::mul(s.ad_value(39), s.ad_value(997)), 1.0), 1.0);s.store_primal_mul(150, 150, 1002);s.store_primal_div_scaled_inputs_indices(1002, 1001, p.p796, 999, 1.0);s.store_primal_add(40, 40, 1002);s.store_primal_add(145, 145, 1002);s.store_primal_div_scaled_inputs_mixed_ia(1002, 1001, p.p802, A::powf(s.ad_value(999), p.p803), 1.0);s.store_primal_add(62, 62, 1002);s.store_primal_add(155, 155, 1002);}
        s.b[1128] = ((((s.v[11] > 0.0) || (s.v[12] > 0.0)) || (s.v[13] > 0.0)) || (s.v[8] > 0.0));s.store_scalar(1128, if s.b[1128] { 1.0 } else { 0.0 });s.b[1129] = (((s.v[11] == 0.0) && (s.v[12] == 0.0)) && (s.v[13] == 0.0));s.store_scalar(1129, if s.b[1129] { 1.0 } else { 0.0 });
        if ((s.b[1019] && s.b[1128]) && s.b[1129]) {s.store_primal_offset(1001, 4, s.v[8]);s.store_scalar(1002, (1.0 / p.p804));s.store_primal_div_from_scalar_scaled_input(11, (p.p804 * p.p804), 1001, s.v[8]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1019] && s.b[1128]) && s.b[1129]) {s.store_primal_div_scaled_add_product_mixed_aaai(12, A::exp_scaled_input(s.ad_value(1002), ((-10.0) * s.v[8])), ((0.1 * s.v[8]) + (0.01 * p.p804)), A::scale_offset(s.ad_value(1001), 0.1, (0.01 * p.p804)), A::exp(A::mul_scaled_lhs(s.ad_value(1001), (-10.0), s.ad_value(1002))), (-1.0), 4, 1.0);s.store_primal_div_scaled_add_product_mixed_aaai(13, A::exp_scaled_input(s.ad_value(1002), ((-20.0) * s.v[8])), ((0.05 * s.v[8]) + (0.0025 * p.p804)), A::scale_offset(s.ad_value(1001), 0.05, (0.0025 * p.p804)), A::exp(A::mul_scaled_lhs(s.ad_value(1001), (-20.0), s.ad_value(1002))), (-1.0), 4, 1.0);}
        if (s.b[1019] && s.b[1128]) {s.store_primal_add_scaled_inputs3_indices(1001, 11, 1.0, 12, p.p805, 13, p.p806);s.store_primal_add_scaled_product_indices(40, 40, 1.0, 342, 1001, 1.0);s.store_primal_mul_scale_offset_mixed_ia(65, 65, A::mul(s.ad_value(343), s.ad_value(1001)), 1.0, 1.0);s.store_primal_add_scaled_product_indices(145, 145, 1.0, 342, 1001, 1.0);s.store_primal_mul_scale_offset_mixed_ia(150, 150, A::mul(s.ad_value(343), s.ad_value(1001)), 1.0, 1.0);}
        s.copy_ad(172, 40);s.copy_ad(173, 41);s.copy_ad(174, 42);s.copy_ad(176, 43);s.copy_ad(177, 44);
        if (s.v[45] > 1e20) {
            if (s.v[45] < 1e26) {
                s.copy_ad(178, 45);
            } else {
                s.store_scalar(178, 1e26);
            }
        } else {
            s.store_scalar(178, 1e20);
        }
        if (s.v[46] > 0.01) {
            s.copy_ad(179, 46);
        } else {
            s.store_scalar(179, 0.01);
        }
        if (s.v[47] > 0.0) {
            s.copy_ad(180, 47);
        } else {
            s.store_scalar(180, 0.0);
        }
        s.copy_ad(181, 48);s.copy_ad(182, 49);
        if (s.v[50] > 0.0) {
            s.copy_ad(183, 50);
        } else {
            s.store_scalar(183, 0.0);
        }
        s.copy_ad(187, 55);s.copy_ad(188, 56);
        if (s.v[57] > 1e23) {
            if (s.v[57] < 1e27) {
                s.copy_ad(189, 57);
            } else {
                s.store_scalar(189, 1e27);
            }
        } else {
            s.store_scalar(189, 1e23);
        }
        if (s.v[58] > 1e23) {
            if (s.v[58] < 1e27) {
                s.copy_ad(190, 58);
            } else {
                s.store_scalar(190, 1e27);
            }
        } else {
            s.store_scalar(190, 1e23);
        }
        if (s.v[51] > 0.0) {
            s.copy_ad(184, 51);
        } else {
            s.store_scalar(184, 0.0);
        }
        if (s.v[53] > 0.0) {
            if (s.v[53] < 0.5) {
                s.copy_ad(186, 53);
            } else {
                s.store_scalar(186, 0.5);
            }
        } else {
            s.store_scalar(186, 0.0);
        }
        if (s.v[52] > 0.0) {
            if (s.v[52] < 1.0) {
                s.copy_ad(185, 52);
            } else {
                s.store_scalar(185, 1.0);
            }
        } else {
            s.store_scalar(185, 0.0);
        }
        s.copy_ad(175, 54);
        if (s.v[62] > 0.0) {
            s.copy_ad(191, 62);
        } else {
            s.store_scalar(191, 0.0);
        }
        if (s.v[64] > 0.0) {
            if (s.v[64] < 1.0) {
                s.copy_ad(193, 64);
            } else {
                s.store_scalar(193, 1.0);
            }
        } else {
            s.store_scalar(193, 0.0);
        }
        if (s.v[63] > 0.0) {
            s.copy_ad(192, 63);
        } else {
            s.store_scalar(192, 0.0);
        }
        if (s.v[59] > 0.0) {
            s.copy_ad(194, 59);
        } else {
            s.store_scalar(194, 0.0);
        }
        if (s.v[61] > 0.0) {
            if (s.v[61] < 1.0) {
                s.copy_ad(195, 61);
            } else {
                s.store_scalar(195, 1.0);
            }
        } else {
            s.store_scalar(195, 0.0);
        }
        if (s.v[60] > 0.0) {
            s.copy_ad(196, 60);
        } else {
            s.store_scalar(196, 0.0);
        }
        if (s.v[65] > 0.0) {
            s.copy_ad(197, 65);
        } else {
            s.store_scalar(197, 0.0);
        }
        s.copy_ad(198, 66);
        if (s.v[67] > 0.0) {
            s.copy_ad(199, 67);
        } else {
            s.store_scalar(199, 0.0);
        }
        s.copy_ad(200, 68);
        if (s.v[69] > 0.0) {
            s.copy_ad(201, 69);
        } else {
            s.store_scalar(201, 0.0);
        }
        s.copy_ad(202, 70);
        if (s.v[71] > 0.0) {
            s.copy_ad(203, 71);
        } else {
            s.store_scalar(203, 0.0);
        }
        s.copy_ad(204, 72);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
    ) {
        if (s.v[73] > 0.0) {
            s.copy_ad(205, 73);
        } else {
            s.store_scalar(205, 0.0);
        }
        s.copy_ad(206, 74);
        if (s.v[75] > 0.0) {
            s.copy_ad(207, 75);
        } else {
            s.store_scalar(207, 0.0);
        }
        s.copy_ad(208, 76);s.copy_ad(209, 77);
        if (s.v[78] > 0.0) {
            s.copy_ad(210, 78);
        } else {
            s.store_scalar(210, 0.0);
        }
        s.copy_ad(211, 79);
        if (s.v[80] > (-0.5)) {
            if (s.v[80] < 1.0) {
                s.copy_ad(212, 80);
            } else {
                s.store_scalar(212, 1.0);
            }
        } else {
            s.store_scalar(212, (-0.5));
        }
        if (s.v[81] > (-0.5)) {
            s.copy_ad(213, 81);
        } else {
            s.store_scalar(213, (-0.5));
        }
        if (s.v[82] > 0.0) {
            s.copy_ad(214, 82);
        } else {
            s.store_scalar(214, 0.0);
        }
        s.copy_ad(215, 83);
        if (s.v[84] > (-0.5)) {
            if (s.v[84] < 1.0) {
                s.copy_ad(216, 84);
            } else {
                s.store_scalar(216, 1.0);
            }
        } else {
            s.store_scalar(216, (-0.5));
        }
        if (s.v[85] > (-0.5)) {
            s.copy_ad(217, 85);
        } else {
            s.store_scalar(217, (-0.5));
        }
        if (s.v[86] > 0.01) {
            s.copy_ad(218, 86);
        } else {
            s.store_scalar(218, 0.01);
        }
        if (s.v[87] > 2.0) {
            s.copy_ad(219, 87);
        } else {
            s.store_scalar(219, 2.0);
        }
        if (s.v[88] > 0.0) {
            s.copy_ad(220, 88);
        } else {
            s.store_scalar(220, 0.0);
        }
        if (s.v[89] > 0.0) {
            s.copy_ad(221, 89);
        } else {
            s.store_scalar(221, 0.0);
        }
        if (s.v[90] > 0.0) {
            s.copy_ad(222, 90);
        } else {
            s.store_scalar(222, 0.0);
        }
        s.copy_ad(223, 91);
        if (s.v[92] > 0.0) {
            s.copy_ad(224, 92);
        } else {
            s.store_scalar(224, 0.0);
        }
        s.copy_ad(225, 93);s.copy_ad(226, 94);
        if (s.v[95] > 0.0) {
            s.copy_ad(227, 95);
        } else {
            s.store_scalar(227, 0.0);
        }
        if (s.v[96] > 0.0) {
            s.copy_ad(228, 96);
        } else {
            s.store_scalar(228, 0.0);
        }
        if (s.v[97] > 1e-12) {
            s.copy_ad(229, 97);
        } else {
            s.store_scalar(229, 1e-12);
        }
        s.copy_ad(230, 98);
        if (s.v[99] > 0.0) {
            s.copy_ad(231, 99);
        } else {
            s.store_scalar(231, 0.0);
        }
        if (s.v[100] > 0.0) {
            s.copy_ad(232, 100);
        } else {
            s.store_scalar(232, 0.0);
        }
        if (s.v[101] > 0.0) {
            s.copy_ad(233, 101);
        } else {
            s.store_scalar(233, 0.0);
        }
        s.copy_ad(234, 102);s.copy_ad(235, 103);s.copy_ad(236, 104);s.copy_ad(237, 105);s.copy_ad(238, 106);s.copy_ad(239, 107);s.copy_ad(240, 108);s.copy_ad(241, 109);
        if (s.v[110] > 0.0) {
            s.copy_ad(242, 110);
        } else {
            s.store_scalar(242, 0.0);
        }
        if (s.v[111] > 0.0) {
            s.copy_ad(243, 111);
        } else {
            s.store_scalar(243, 0.0);
        }
        s.copy_ad(244, 112);s.copy_ad(245, 113);s.copy_ad(246, 114);s.copy_ad(247, 115);s.copy_ad(248, 116);s.copy_ad(249, 117);
        if (s.v[118] > 0.0) {
            s.copy_ad(250, 118);
        } else {
            s.store_scalar(250, 0.0);
        }
        s.copy_ad(251, 119);
        if (s.v[120] > 0.0) {
            s.copy_ad(252, 120);
        } else {
            s.store_scalar(252, 0.0);
        }
        if (s.v[121] > 0.0) {
            s.copy_ad(253, 121);
        } else {
            s.store_scalar(253, 0.0);
        }
        if (s.v[122] > 2.0) {
            s.copy_ad(254, 122);
        } else {
            s.store_scalar(254, 2.0);
        }
        s.copy_ad(255, 123);
        if (s.v[124] > 0.0) {
            s.copy_ad(256, 124);
        } else {
            s.store_scalar(256, 0.0);
        }
        if (s.v[125] > 0.0) {
            s.copy_ad(257, 125);
        } else {
            s.store_scalar(257, 0.0);
        }
        if (s.v[126] > 0.0) {
            s.copy_ad(258, 126);
        } else {
            s.store_scalar(258, 0.0);
        }
        s.copy_ad(259, 127);s.copy_ad(260, 128);s.copy_ad(261, 129);
        if (s.v[130] > 0.0) {
            s.copy_ad(262, 130);
        } else {
            s.store_scalar(262, 0.0);
        }
        if (s.v[131] > 0.0) {
            s.copy_ad(263, 131);
        } else {
            s.store_scalar(263, 0.0);
        }
        if (s.v[132] > 0.0) {
            s.copy_ad(264, 132);
        } else {
            s.store_scalar(264, 0.0);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.copy_ad(265, 133);s.copy_ad(266, 134);s.copy_ad(267, 135);s.copy_ad(268, 136);
        if (s.v[137] > 0.0) {
            s.copy_ad(269, 137);
        } else {
            s.store_scalar(269, 0.0);
        }
        if (s.v[138] > 0.0) {
            s.copy_ad(270, 138);
        } else {
            s.store_scalar(270, 0.0);
        }
        s.copy_ad(271, 139);s.copy_ad(277, 145);s.copy_ad(278, 146);s.copy_ad(279, 147);
        if (s.v[148] > 1e20) {
            if (s.v[148] < 1e26) {
                s.copy_ad(280, 148);
            } else {
                s.store_scalar(280, 1e26);
            }
        } else {
            s.store_scalar(280, 1e20);
        }
        if (s.v[149] > 0.0) {
            s.copy_ad(281, 149);
        } else {
            s.store_scalar(281, 0.0);
        }
        if (s.v[150] > 0.0) {
            s.copy_ad(282, 150);
        } else {
            s.store_scalar(282, 0.0);
        }
        s.copy_ad(283, 151);
        if (s.v[152] > 0.0) {
            s.copy_ad(284, 152);
        } else {
            s.store_scalar(284, 0.0);
        }
        if (s.v[153] > 0.0) {
            if (s.v[153] < 1.0) {
                s.copy_ad(285, 153);
            } else {
                s.store_scalar(285, 1.0);
            }
        } else {
            s.store_scalar(285, 0.0);
        }
        if (s.v[154] > 0.0) {
            s.copy_ad(286, 154);
        } else {
            s.store_scalar(286, 0.0);
        }
        if (s.v[155] > 0.0) {
            s.copy_ad(287, 155);
        } else {
            s.store_scalar(287, 0.0);
        }
        if (s.v[157] > 0.0) {
            if (s.v[157] < 1.0) {
                s.copy_ad(289, 157);
            } else {
                s.store_scalar(289, 1.0);
            }
        } else {
            s.store_scalar(289, 0.0);
        }
        if (s.v[156] > 0.0) {
            s.copy_ad(288, 156);
        } else {
            s.store_scalar(288, 0.0);
        }
        if ((p.p31 * s.v[1]) > 0.0) {
            s.store_primal_scale(15, 1, p.p31);
        } else {
            s.store_scalar(15, 0.0);
        }
        s.store_scalar(16, p.p16);s.store_scalar(17, p.p15);s.store_scalar(18, p.p18);s.store_scalar(19, p.p17);s.b[1130] = (p.p44 == 0.0);s.store_scalar(1130, if s.b[1130] { 1.0 } else { 0.0 });
        if s.b[1130] {s.copy_ad(188, 187);s.copy_ad(190, 189);s.copy_ad(243, 242);s.copy_ad(245, 244);s.copy_ad(247, 246);s.copy_ad(249, 248);s.copy_ad(233, 232);s.copy_ad(239, 237);s.copy_ad(240, 238);s.copy_ad(258, 257);s.copy_ad(260, 259);s.copy_ad(264, 263);s.copy_ad(270, 269);}
        s.store_primal_scale(762, 177, 8.8541878176e-12);s.store_primal_div(763, 762, 176);s.store_primal_square(764, 176);s.store_primal_scale(765, 763, 6.241449993689894e18);s.store_primal_mul(766, 252, 178);
        if (s.v[766] > 1e20) {
            if (s.v[766] < 1e26) {
            } else {
                s.store_scalar(766, 1e26);
            }
        } else {
            s.store_scalar(766, 1e20);
        }
        s.store_scalar(767, 0.0);s.b[1131] = (p.p51 > 0.0);s.store_scalar(1131, if s.b[1131] { 1.0 } else { 0.0 });
        if s.b[1131] {s.store_primal_scale_ad(767, A::powf(s.ad_value(763), 0.6666666666666666), ((0.4 * 5.951993) * p.p51));}
        s.b[1132] = (s.v[0] == (-1.0));s.store_scalar(1132, if s.b[1132] { 1.0 } else { 0.0 });
        if (s.b[1131] && s.b[1132]) {s.store_primal_scale(767, 767, (7.448711 / 5.951993));}
        s.store_primal_scale(768, 763, (1e-8 * 1.0 / (s.v[761])));s.store_primal_scale(769, 209, 0.5);s.store_scalar(770, 0.5);s.b[1133] = (s.v[0] == (-1.0));s.store_scalar(1133, if s.b[1133] { 1.0 } else { 0.0 });
        if s.b[1133] {s.store_primal_scale(769, 209, 0.3333333333333333);s.store_scalar(770, 0.3333333333333333);}
        s.store_primal_offset_pow_from_scalar_ad(1000, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(219)), 1.0), (-1.0));
        s.store_primal_div_scaled_product_offset_lhs_mixed_iaa(771, 1000, (-1.0), A::offset(s.ad_value(1000), (-1.0)), 1.0, {
            if ((4.0 * s.v[1000]) > 0.0001) {
                A::scale(s.ad_value(1000), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
    ) {
        s.store_primal_offset_pow_from_scalar_ad(1000, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(254)), 1.0), (-1.0));
        s.store_primal_div_scaled_product_offset_lhs_mixed_iaa(772, 1000, (-1.0), A::offset(s.ad_value(1000), (-1.0)), 1.0, {
            if ((4.0 * s.v[1000]) > 0.0001) {
                A::scale(s.ad_value(1000), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);s.store_primal_div_from_scalar(773, 1.0, 223);s.store_primal_div(774, 762, 187);s.store_primal_div(775, 762, 188);s.store_primal_div_mixed_ai(776, A::sqrt_scaled_input(s.ad_value(189), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[349])), 774);s.store_primal_div_mixed_ai(777, A::sqrt_scaled_input(s.ad_value(190), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[349])), 775);s.store_primal_square(778, 776);s.store_primal_square(779, 777);s.store_primal_offset_div_ad(780, A::ln(A::offset(A::exp_scaled_input(s.ad_value(261), (0.005 * s.v[349])), (-1.0))), s.ad_value(261), (-((((((0.005 * s.v[349])) as f64).exp() - 1.0)) as f64).ln()));s.store_primal_add_mixed_ai(781, A::ln_scaled_input(s.ad_value(776), 0.5), 780);s.store_primal_add_mixed_ai(782, A::ln_scaled_input(s.ad_value(777), 0.5), 780);s.store_primal_div_from_scalar(814, 1.0, 776);s.store_primal_offset_scaled(815, 776, 3.1, 8.5);s.store_primal_square(783, 815);s.store_primal_scale(816, 815, 0.5);s.b[1134] = (s.v[814] < 0.06);s.store_scalar(1134, if s.b[1134] { 1.0 } else { 0.0 });
        if s.b[1134] {s.store_primal_scale(784, 814, 64.0);}
        s.b[1135] = (s.v[814] <= 0.45);s.store_scalar(1135, if s.b[1135] { 1.0 } else { 0.0 });
        if ((!s.b[1134]) && s.b[1135]) {s.store_primal_offset_scaled(784, 814, 22.0, 3.0);}
        s.b[1136] = (s.v[814] <= 1.6);s.store_scalar(1136, if s.b[1136] { 1.0 } else { 0.0 });
        if (((!s.b[1134]) && (!s.b[1135])) && s.b[1136]) {s.store_primal_offset_scaled(784, 814, (-7.2), 15.5);}
        if (((!s.b[1134]) && (!s.b[1135])) && (!s.b[1136])) {s.copy_ad(784, 776);}
        s.store_primal_add_scaled_inputs_product_mixed_iiia(785, 816, 1.0, 778, 0.5, 776, A::sqrt(A::add_scaled_inputs3(s.ad_value(816), 1.0, s.ad_value(778), 0.25, s.ad_value(784), 1.0)), (-1.0));s.store_primal_div_from_scalar(814, 1.0, 777);s.store_primal_offset_scaled(815, 777, 3.1, 8.5);s.store_primal_square(786, 815);s.store_primal_scale(816, 815, 0.5);s.b[1137] = (s.v[814] < 0.06);s.store_scalar(1137, if s.b[1137] { 1.0 } else { 0.0 });
        if s.b[1137] {s.store_primal_scale(787, 814, 64.0);}
        s.b[1138] = (s.v[814] <= 0.45);s.store_scalar(1138, if s.b[1138] { 1.0 } else { 0.0 });
        if ((!s.b[1137]) && s.b[1138]) {s.store_primal_offset_scaled(787, 814, 22.0, 3.0);}
        s.b[1139] = (s.v[814] <= 1.6);s.store_scalar(1139, if s.b[1139] { 1.0 } else { 0.0 });
        if (((!s.b[1137]) && (!s.b[1138])) && s.b[1139]) {s.store_primal_offset_scaled(787, 814, (-7.2), 15.5);}
        if (((!s.b[1137]) && (!s.b[1138])) && (!s.b[1139])) {s.copy_ad(787, 777);}
        s.store_primal_add_scaled_inputs_product_mixed_iiia(788, 816, 1.0, 779, 0.5, 777, A::sqrt(A::add_scaled_inputs3(s.ad_value(816), 1.0, s.ad_value(779), 0.25, s.ad_value(787), 1.0)), (-1.0));s.store_primal_add_scaled_inputs_ad(722, A::offset(s.ad_value(182), s.v[356]), 1.0, A::ln_scaled_input(A::mul(s.ad_value(178), A::powf(s.ad_value(357), (-0.75))), 4e-26), (2.0 * s.v[709]));
        if (!(s.v[722] > 0.05)) {s.store_scalar(722, 0.05);}
        s.store_primal_div_mixed_ai(723, A::sqrt_scaled_input(s.ad_value(178), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[355])), 763);s.store_scalar(724, 0.0);s.store_scalar(725, 0.0);s.b[1140] = (s.v[183] > 0.0);s.store_scalar(1140, if s.b[1140] { 1.0 } else { 0.0 });
        if s.b[1140] {s.store_primal_div_from_scalar(726, 80000000.0, 764);}
        if s.b[1140] {
            if (s.v[183] > s.v[726]) {
                s.copy_ad(725, 183);
            } else {
                s.copy_ad(725, 726);
            }
        }
        if s.b[1140] {
            if (5e24 > s.v[725]) {
                s.store_scalar(725, 5e24);
            } else {
            }
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1140] {s.store_primal_div_scaled_product_indices(724, 763, 763, (2.0 * s.v[709]), 725, (1.6021918e-19 * s.v[761]));}
        s.store_scalar(727, ((100.0 * s.v[709]) * s.v[709]));s.b[1141] = (p.p51 > 0.0);s.store_scalar(1141, if s.b[1141] { 1.0 } else { 0.0 });
        if s.b[1141] {s.store_primal_sqrt_ad(728, A::mul3_scaled_output(s.ad_value(723), s.ad_value(723), s.ad_value(722), s.v[709]));s.store_primal_mul_scaled_powf_rhs(729, 767, 0.75, 728, 0.6666666666666666);s.store_primal_add(722, 722, 729);s.store_primal_mul_scale_offset_mixed_ia(723, 723, A::div_scaled_inputs(s.ad_value(729), (2.0 * 0.6666666666666666), s.ad_value(728), 1.0), 1.0, 1.0);}
        s.store_primal_sqrt(730, 722);s.store_primal_scale(731, 722, 0.95);s.store_primal_scaled_mul(732, 722, 722, 0.0025);s.copy_ad(733, 732);s.store_primal_scaled_sqrt(734, 733, 0.5);s.store_primal_add_scaled_inputs3_sqrt_third_mixed_iia(735, 731, 0.5, 734, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(731), s.ad_value(734))), s.ad_value(732)), (-0.5));s.store_primal_scaled_offset(736, 722, s.v[356], 0.5);s.store_primal_sub_mixed_ai(737, A::sqrt(A::add(s.ad_value(180), s.ad_value(722))), 730);s.store_primal_add_scaled_inputs3_sqrt_first_mixed_aii(738, A::add_scaled_inputs3(s.ad_value(180), 1.0, s.ad_value(181), 1.0, s.ad_value(722), 1.0), 1.0, 730, (-1.0), 737, -1.0);s.store_primal_add_scaled_inputs3_offset_mixed_iia(739, 182, 1.0, 251, 1.0, A::ln_scaled_input(A::mul(s.ad_value(766), A::powf(s.ad_value(357), (-0.75))), 4e-26), (2.0 * s.v[709]), s.v[356]);
        if (!(s.v[739] > 0.05)) {s.store_scalar(739, 0.05);}
        s.store_primal_div_mixed_ai(740, A::sqrt_scaled_input(s.ad_value(766), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[355])), 763);s.b[1142] = (p.p51 > 0.0);s.store_scalar(1142, if s.b[1142] { 1.0 } else { 0.0 });
        if s.b[1142] {s.store_primal_sqrt_ad(728, A::mul3_scaled_output(s.ad_value(740), s.ad_value(740), s.ad_value(739), s.v[709]));s.store_primal_mul_scaled_powf_rhs(729, 767, 0.75, 728, 0.6666666666666666);s.store_primal_add(739, 739, 729);s.store_primal_mul_scale_offset_mixed_ia(740, 740, A::div_scaled_inputs(s.ad_value(729), (2.0 * 0.6666666666666666), s.ad_value(728), 1.0), 1.0, 1.0);}
        s.store_primal_scale(741, 739, 0.95);s.store_primal_scaled_mul(742, 739, 739, 0.0025);s.copy_ad(743, 742);s.store_primal_scaled_sqrt(734, 743, 0.5);s.store_primal_add_scaled_inputs3_sqrt_third_mixed_iia(744, 741, 0.5, 734, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(741), s.ad_value(734))), s.ad_value(742)), (-0.5));s.store_primal_offset_add_scaled_product_mixed_iia(694, 172, 1.0, 173, A::scale_offset(s.ad_value(174), s.v[352], 1.0), s.v[352], s.v[17]);s.store_primal_exp_scaled_input(745, 175, s.v[354]);s.store_primal_mul(695, 184, 745);s.store_primal_scale(696, 185, 1.0 / (s.v[353]));s.store_primal_exp_scaled_input(746, 198, s.v[354]);s.store_primal_mul(697, 197, 746);s.store_primal_scaled_mul(710, 697, 763, s.v[16]);s.store_primal_mul_mixed_ia(699, 201, A::exp_scaled_input(s.ad_value(202), s.v[354]));s.store_primal_exp_scaled_input(747, 200, s.v[354]);s.store_primal_mul(698, 199, 747);s.store_primal_mul_mixed_ia(701, 205, A::exp_scaled_input(s.ad_value(206), s.v[354]));s.store_primal_exp_scaled_input(748, 204, s.v[354]);s.store_primal_mul(700, 203, 748);s.store_primal_exp_scaled_input(749, 208, s.v[354]);s.store_primal_mul(702, 207, 749);s.store_primal_exp_scaled_input(750, 211, s.v[354]);s.store_primal_mul(703, 210, 750);s.store_primal_scaled_mul(751, 710, 703, 2.0);
    }
}
