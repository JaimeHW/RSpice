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
        let ctx_temp = ctx.temperature();s.store_scalar(769, (ctx_temp + p.p0));s.store_scalar(36, p.p34);s.store_scalar(37, p.p1);s.store_scalar(38, p.p2);s.store_scalar(39, p.p3);s.store_scalar(40, p.p4);s.store_scalar(41, p.p5);s.store_scalar(42, p.p6);s.store_scalar(43, p.p7);s.store_scalar(44, p.p8);s.store_scalar(45, p.p9);s.store_scalar(46, p.p10);s.store_scalar(47, p.p11);s.store_scalar(48, p.p12);s.store_scalar(49, p.p14);s.store_scalar(50, p.p16);s.store_scalar(51, p.p17);s.store_scalar(52, p.p18);s.store_scalar(53, p.p19);s.store_scalar(54, p.p20);s.store_scalar(55, p.p21);s.store_scalar(56, p.p22);s.store_scalar(57, p.p23);s.store_scalar(58, p.p24);s.store_scalar(59, p.p25);s.store_scalar(60, p.p26);s.store_scalar(61, p.p27);s.store_scalar(62, p.p28);s.store_scalar(63, p.p29);s.store_scalar(64, p.p30);s.store_scalar(65, p.p31);s.store_scalar(66, p.p37);s.store_scalar(67, p.p38);s.store_scalar(68, p.p39);s.store_scalar(69, p.p40);s.store_scalar(70, p.p41);s.store_scalar(71, p.p42);s.store_scalar(72, p.p43);s.store_scalar(73, p.p44);s.store_scalar(74, p.p45);s.store_scalar(75, p.p46);s.store_scalar(76, p.p47);s.store_scalar(77, p.p48);s.store_scalar(78, p.p49);s.store_scalar(79, p.p50);s.store_scalar(80, p.p51);s.store_scalar(81, p.p52);s.store_scalar(82, p.p53);s.store_scalar(83, p.p54);s.store_scalar(84, p.p55);s.store_scalar(85, p.p56);s.store_scalar(86, p.p57);s.store_scalar(87, p.p58);s.store_scalar(88, p.p59);s.store_scalar(89, p.p60);s.store_scalar(90, p.p63);s.store_scalar(91, p.p64);s.store_scalar(93, p.p66);s.store_scalar(94, p.p67);s.store_scalar(95, p.p68);s.store_scalar(96, p.p69);s.store_scalar(97, p.p70);s.store_scalar(98, p.p71);s.store_scalar(99, p.p72);s.store_scalar(100, p.p73);s.store_scalar(101, p.p74);s.store_scalar(102, p.p75);s.store_scalar(103, p.p76);s.store_scalar(104, p.p77);s.store_scalar(105, p.p78);s.store_scalar(106, p.p79);s.store_scalar(107, p.p80);s.store_scalar(108, p.p81);s.store_scalar(109, p.p82);s.store_scalar(110, p.p83);s.store_scalar(111, p.p84);s.store_scalar(112, p.p85);s.store_scalar(113, p.p86);s.store_scalar(114, p.p87);s.store_scalar(115, p.p88);s.store_scalar(116, p.p89);s.store_scalar(117, p.p90);s.store_scalar(118, p.p91);s.store_scalar(119, p.p92);s.store_scalar(120, p.p93);s.store_scalar(121, p.p94);s.store_scalar(122, p.p95);s.store_scalar(123, p.p96);s.store_scalar(124, p.p973);s.store_scalar(125, p.p97);s.store_scalar(126, p.p98);s.store_scalar(127, p.p99);s.store_scalar(128, p.p100);s.store_scalar(129, p.p101);s.store_scalar(130, p.p102);s.store_scalar(131, p.p103);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(132, p.p104);s.store_scalar(133, p.p105);s.store_scalar(134, p.p107);s.store_scalar(135, p.p108);s.store_scalar(136, p.p109);s.store_scalar(137, p.p110);s.store_scalar(138, p.p111);s.store_scalar(139, p.p112);s.store_scalar(140, p.p113);s.store_scalar(141, p.p114);s.store_scalar(142, p.p115);s.store_scalar(143, p.p116);s.store_scalar(144, p.p117);s.store_scalar(145, p.p118);s.store_scalar(146, p.p119);s.store_scalar(147, p.p120);s.store_scalar(148, p.p121);s.store_scalar(149, p.p122);s.store_scalar(150, (p.p123 + 273.15));s.store_scalar(153, p.p126);s.store_scalar(154, p.p127);s.store_scalar(155, p.p128);s.store_scalar(156, p.p129);s.store_scalar(157, p.p130);s.store_scalar(158, p.p131);s.store_scalar(159, p.p132);s.store_scalar(160, p.p133);s.store_scalar(161, p.p134);s.store_scalar(162, p.p135);s.store_scalar(163, p.p136);s.store_scalar(164, p.p137);s.store_scalar(165, p.p138);s.store_scalar(166, p.p139);s.store_scalar(167, p.p140);s.store_scalar(168, p.p141);s.store_scalar(169, p.p142);s.store_scalar(170, p.p143);s.store_scalar(171, p.p144);s.store_scalar(172, p.p145);s.store_scalar(173, p.p146);s.store_scalar(174, p.p147);s.store_scalar(175, p.p148);s.store_scalar(176, p.p149);s.store_scalar(177, p.p974);s.store_scalar(178, p.p150);s.store_scalar(179, p.p151);s.store_scalar(180, p.p152);s.store_scalar(181, p.p153);s.store_scalar(182, p.p154);s.store_scalar(183, p.p155);s.store_scalar(184, p.p975);s.store_scalar(185, p.p156);s.store_scalar(186, p.p157);s.store_scalar(187, p.p158);s.store_scalar(188, p.p159);s.store_scalar(189, p.p160);s.store_scalar(190, p.p161);s.store_scalar(191, p.p162);s.store_scalar(192, p.p163);s.store_scalar(193, p.p164);s.store_scalar(194, p.p165);s.store_scalar(195, p.p166);s.store_scalar(196, p.p167);s.store_scalar(197, p.p168);s.store_scalar(198, p.p169);s.store_scalar(199, p.p170);s.store_scalar(200, p.p171);s.store_scalar(201, p.p172);s.copy_ad(202, 1152);s.store_scalar(203, p.p174);s.store_scalar(204, p.p175);s.store_scalar(205, p.p176);s.store_scalar(206, p.p177);s.store_scalar(207, p.p178);s.store_scalar(208, p.p179);s.store_scalar(209, p.p180);s.store_scalar(210, p.p181);s.store_scalar(211, p.p182);s.store_scalar(212, p.p183);s.store_scalar(213, p.p184);s.store_scalar(214, p.p185);s.store_scalar(215, p.p186);s.store_scalar(216, p.p187);s.store_scalar(217, p.p188);s.store_scalar(218, p.p189);s.store_scalar(219, p.p190);s.store_scalar(220, p.p191);s.store_scalar(221, p.p192);s.store_scalar(222, p.p193);s.store_scalar(223, p.p194);s.store_scalar(224, p.p195);s.store_scalar(225, p.p196);s.store_scalar(226, p.p197);s.store_scalar(227, p.p198);s.store_scalar(228, p.p199);s.store_scalar(229, p.p200);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(230, p.p201);s.store_scalar(231, p.p202);s.store_scalar(233, p.p204);s.store_scalar(234, p.p205);s.store_scalar(235, p.p206);s.store_scalar(236, p.p207);s.store_scalar(237, p.p208);s.store_scalar(241, p.p214);s.store_scalar(243, p.p216);s.store_scalar(246, p.p219);s.store_scalar(247, p.p220);s.store_scalar(248, p.p221);s.store_scalar(249, p.p222);s.store_scalar(250, p.p223);s.store_scalar(251, p.p224);s.store_scalar(252, p.p225);s.store_scalar(253, p.p226);s.store_scalar(254, p.p227);s.store_scalar(255, p.p228);s.store_scalar(256, p.p229);s.store_scalar(257, p.p236);s.store_scalar(258, p.p237);s.store_scalar(259, p.p238);s.store_scalar(260, p.p239);s.store_scalar(261, p.p240);s.store_scalar(262, p.p241);s.store_scalar(263, p.p242);s.store_scalar(266, p.p245);s.store_scalar(267, p.p249);s.store_scalar(268, p.p253);s.store_scalar(269, p.p257);s.store_scalar(270, p.p261);s.store_scalar(271, p.p265);s.store_scalar(272, p.p269);s.store_scalar(273, p.p270);s.store_scalar(274, p.p271);s.store_scalar(275, p.p272);s.store_scalar(281, p.p287);s.store_scalar(282, p.p288);s.store_scalar(283, p.p289);s.store_scalar(284, p.p290);s.store_scalar(285, p.p291);s.store_scalar(286, p.p292);s.store_scalar(287, p.p293);s.store_scalar(288, p.p294);s.store_scalar(289, p.p295);s.store_scalar(290, p.p296);s.store_scalar(291, p.p297);s.store_scalar(292, p.p298);s.store_scalar(293, p.p299);s.store_scalar(294, p.p300);s.store_scalar(295, p.p301);s.store_scalar(296, p.p302);s.store_scalar(297, p.p303);s.store_scalar(298, p.p304);s.store_scalar(299, p.p305);s.store_scalar(300, p.p306);s.store_scalar(301, p.p307);s.store_scalar(302, p.p308);s.store_scalar(303, p.p309);s.store_scalar(304, p.p310);s.store_scalar(305, p.p311);s.store_scalar(306, p.p312);s.store_scalar(307, p.p313);s.store_scalar(308, p.p314);s.store_scalar(309, p.p315);s.store_scalar(310, p.p316);s.store_scalar(311, p.p317);s.store_scalar(312, p.p318);s.store_scalar(313, p.p319);s.store_scalar(314, p.p320);s.store_scalar(315, p.p321);s.store_scalar(316, p.p322);s.store_scalar(317, p.p323);s.store_scalar(318, p.p324);s.store_scalar(319, p.p325);s.store_scalar(320, p.p326);s.store_scalar(321, p.p327);s.store_scalar(322, p.p328);s.store_scalar(323, p.p329);s.store_scalar(324, p.p330);s.store_scalar(325, p.p331);s.store_scalar(326, p.p332);s.store_scalar(327, p.p333);s.store_scalar(328, p.p334);s.store_scalar(329, p.p335);s.store_scalar(330, p.p336);s.store_scalar(331, p.p337);s.store_scalar(332, p.p338);s.store_scalar(333, p.p339);s.store_scalar(334, p.p340);s.store_scalar(335, p.p341);s.store_scalar(336, p.p342);s.store_scalar(337, p.p343);s.store_scalar(338, p.p344);s.store_scalar(339, p.p345);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(340, p.p346);s.store_scalar(341, p.p347);s.store_scalar(342, p.p348);s.store_scalar(343, p.p349);s.store_scalar(344, p.p350);s.store_scalar(345, p.p351);s.store_scalar(346, p.p352);s.store_scalar(347, p.p353);s.store_scalar(348, p.p354);s.store_scalar(349, p.p355);s.store_scalar(350, p.p356);s.store_scalar(351, p.p357);s.store_scalar(352, p.p358);s.store_scalar(353, p.p359);s.store_scalar(354, p.p360);s.store_scalar(355, p.p362);s.store_scalar(356, p.p363);s.store_scalar(357, p.p364);s.store_scalar(358, p.p365);s.store_scalar(359, p.p366);s.store_scalar(360, p.p367);s.store_scalar(361, p.p368);s.store_scalar(362, p.p369);s.store_scalar(363, p.p370);s.store_scalar(364, p.p371);s.store_scalar(365, p.p372);s.store_scalar(366, p.p373);s.store_scalar(367, p.p374);s.store_scalar(368, p.p375);s.store_scalar(369, p.p376);s.store_scalar(370, p.p377);s.store_scalar(371, p.p378);s.store_scalar(372, p.p379);s.store_scalar(373, p.p380);s.store_scalar(374, p.p381);s.store_scalar(375, p.p382);s.store_scalar(376, p.p383);s.store_scalar(377, p.p384);s.store_scalar(378, p.p385);s.store_scalar(379, p.p386);s.store_scalar(380, p.p387);s.store_scalar(381, p.p388);s.store_scalar(382, p.p389);s.store_scalar(383, p.p390);s.store_scalar(384, p.p391);s.store_scalar(385, p.p392);s.store_scalar(388, p.p395);s.store_scalar(389, p.p396);s.store_scalar(390, p.p397);s.store_scalar(391, p.p398);s.store_scalar(392, p.p399);s.store_scalar(393, p.p400);s.store_scalar(394, p.p401);s.store_scalar(395, p.p402);s.store_scalar(396, p.p403);s.store_scalar(386, p.p393);s.store_scalar(387, p.p394);s.store_scalar(397, p.p404);s.store_scalar(398, p.p405);s.store_scalar(399, p.p406);s.store_scalar(400, p.p407);s.store_scalar(401, p.p408);s.store_scalar(402, p.p409);s.store_scalar(403, p.p410);s.store_scalar(404, p.p411);s.store_scalar(405, p.p412);s.store_scalar(406, p.p413);s.store_scalar(407, p.p414);s.store_scalar(408, p.p418);s.store_scalar(455, p.p985);s.store_scalar(456, p.p986);s.store_scalar(457, p.p987);s.store_scalar(458, p.p988);s.store_scalar(459, p.p989);s.store_scalar(460, p.p990);s.store_scalar(461, p.p991);s.store_scalar(462, p.p992);s.store_scalar(463, p.p993);s.store_scalar(464, p.p994);s.store_scalar(465, p.p995);
        if (s.v[68] != 0.0) {s.store_scalar(777, 3.9);s.store_scalar(776, s.v[72]);s.store_scalar(778, (8.85418e-12 * s.v[74]));s.store_primal_sqrt_scaled_input(780, 778, (2000000.0 * 1.60219e-19));s.store_primal_div_scaled_inputs_indices(757, 777, 8.85418e-12, 776, 1.0);s.store_scalar(781, s.v[455]);s.store_scalar(782, s.v[456]);s.store_scalar(784, s.v[457]);s.store_scalar(785, s.v[458]);s.store_scalar(786, s.v[459]);s.store_scalar(787, s.v[460]);s.store_scalar(788, s.v[461]);s.store_scalar(789, s.v[462]);s.store_scalar(790, s.v[463]);s.store_scalar(791, s.v[464]);}
        if (s.v[68] == 0.0) {s.store_scalar(777, s.v[73]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.v[68] == 0.0) {s.store_scalar(776, s.v[91]);s.store_scalar(778, 1.03594e-10);s.store_scalar(780, 5.753e-12);s.store_scalar(757, (3.453133e-11 / s.v[91]));s.store_scalar(781, s.v[455]);s.store_scalar(782, s.v[456]);s.store_scalar(784, s.v[457]);s.store_scalar(785, s.v[458]);s.store_scalar(786, s.v[459]);s.store_scalar(787, s.v[460]);s.store_scalar(788, s.v[461]);s.store_scalar(789, s.v[462]);s.store_scalar(790, s.v[463]);s.store_scalar(791, s.v[464]);}
        s.store_scalar(760, 0.0);s.b[807] = param_given[203];s.store_scalar(807, if s.b[807] { 1.0 } else { 0.0 });
        if s.b[807] {s.store_scalar(232, p.p203);}
        if (!s.b[807]) {s.store_scalar(232, (((2.0 * 3.453133e-11) / 3.141592653589793) * (((1.0 + (4e-7 / s.v[91]))) as f64).ln()));}
        s.b[808] = param_given[125];s.store_scalar(808, if s.b[808] { 1.0 } else { 0.0 });
        if s.b[808] {s.store_scalar(152, p.p125);}
        s.b[809] = (param_given[207] && (s.v[236] > 0.0));s.store_scalar(809, if s.b[809] { 1.0 } else { 0.0 });
        if ((!s.b[808]) && s.b[809]) {s.store_primal_offset_scaled(152, 757, s.v[236], (-s.v[230]));}
        if ((!s.b[808]) && (!s.b[809])) {s.store_primal_scale(152, 757, (0.6 * s.v[176]));}
        s.b[810] = param_given[124];s.store_scalar(810, if s.b[810] { 1.0 } else { 0.0 });
        if s.b[810] {s.store_scalar(151, p.p124);}
        s.b[811] = (param_given[207] && (s.v[236] > 0.0));s.store_scalar(811, if s.b[811] { 1.0 } else { 0.0 });
        if ((!s.b[810]) && s.b[811]) {s.store_primal_offset_scaled(151, 757, s.v[236], (-s.v[229]));}
        if ((!s.b[810]) && (!s.b[811])) {s.store_primal_scale(151, 757, (0.6 * s.v[176]));}
        s.b[885] = (s.v[200] < 0.1);s.store_scalar(885, if s.b[885] { 1.0 } else { 0.0 });
        if s.b[885] {s.store_scalar(200, 0.1);}
        s.b[886] = (s.v[201] < 0.1);s.store_scalar(886, if s.b[886] { 1.0 } else { 0.0 });
        if s.b[886] {s.store_scalar(201, 0.1);}
        s.store_scalar(832, s.v[150]);s.store_scalar(827, (s.v[769] / s.v[832]));
        if (s.v[68] != 0.0) {s.store_primal_sqrt_mul_ad(758, A::div_scaled_inputs(s.ad_value(778), 1.0, s.ad_value(777), 8.85418e-12), s.ad_value(776));}
        if (s.v[68] == 0.0) {s.store_scalar(758, ((((1.03594e-10 / 3.453133e-11) * s.v[91])) as f64).sqrt());}
        s.store_scalar(783, s.v[465]);s.b[887] = (s.v[68] == 0.0);s.store_scalar(887, if s.b[887] { 1.0 } else { 0.0 });
        if s.b[887] {s.store_scalar(831, (8.617087e-5 * s.v[832]));s.store_scalar(816, (1.16 - (((0.000702 * s.v[832]) * s.v[832]) / (s.v[832] + 1108.0))));s.copy_ad(755, 816);s.store_scalar(409, (8.617087e-5 * s.v[769]));s.store_scalar(815, (1.16 - (((0.000702 * s.v[769]) * s.v[769]) / (s.v[769] + 1108.0))));s.copy_ad(756, 815);s.store_scaled_exp_ad(817, A::sub_from_scalar(21.5565981, A::div_scaled_inputs(s.ad_value(815), 1.0, s.ad_value(409), 2.0)), ((14500000000.0 * (s.v[769] / 300.15)) * (((s.v[769] / 300.15)) as f64).sqrt()));}
        if (!s.b[887]) {s.store_scalar(831, (8.617087e-5 * s.v[832]));s.store_scalar(816, (s.v[76] - (((s.v[77] * s.v[832]) * s.v[832]) / (s.v[832] + s.v[78]))));s.copy_ad(755, 816);s.store_scalar(409, (8.617087e-5 * s.v[769]));s.store_scalar(815, (s.v[76] - (((s.v[77] * s.v[769]) * s.v[769]) / (s.v[769] + s.v[78]))));s.copy_ad(756, 815);s.store_scaled_exp_ad(817, A::sub(A::div_scaled_inputs(s.ad_value(816), 1.0, s.ad_value(831), 2.0), A::div_scaled_inputs(s.ad_value(815), 1.0, s.ad_value(409), 2.0)), ((s.v[75] * (s.v[769] / s.v[832])) * (((s.v[769] / s.v[832])) as f64).sqrt()));}
        s.store_scalar(427, (s.v[52] * s.v[330]));s.store_scalar(825, s.v[37]);s.store_scalar(826, (s.v[38] / s.v[39]));s.store_scalar(818, ((s.v[825]) as f64).powf(s.v[209]));s.store_scalar(819, ((s.v[826]) as f64).powf(s.v[212]));s.store_scalar(813, (((s.v[207] / s.v[818]) + (s.v[210] / s.v[819])) + (s.v[213] / (s.v[818] * s.v[819]))));s.store_scalar(687, (s.v[206] + s.v[813]));s.store_scalar(813, (((s.v[208] / s.v[818]) + (s.v[211] / s.v[819])) + (s.v[214] / (s.v[818] * s.v[819]))));s.store_scalar(691, (s.v[236] + s.v[813]));s.store_scalar(581, (s.v[385] + s.v[813]));s.b[888] = (s.v[581] < 0.0);s.store_scalar(888, if s.b[888] { 1.0 } else { 0.0 });
        if s.b[888] {s.store_scalar(581, 0.0);}
        s.store_scalar(820, ((s.v[825]) as f64).powf(s.v[221]));s.store_scalar(821, ((s.v[826]) as f64).powf(s.v[224]));s.store_scalar(814, (((s.v[219] / s.v[820]) + (s.v[222] / s.v[821])) + (s.v[225] / (s.v[820] * s.v[821]))));s.store_scalar(686, (s.v[216] + s.v[814]));s.store_scalar(814, (((s.v[220] / s.v[820]) + (s.v[223] / s.v[821])) + (s.v[226] / (s.v[820] * s.v[821]))));s.store_scalar(690, (s.v[235] + s.v[814]));s.store_scalar(688, (s.v[37] - (2.0 * s.v[687])));s.store_scalar(689, (((s.v[38] / s.v[39]) - (s.v[58] * s.v[284])) - ((2.0 - s.v[58]) * s.v[686])));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(709, ((s.v[689] / s.v[59]) + s.v[60]));s.store_scalar(708, ((s.v[689] / s.v[59]) + s.v[61]));s.store_scalar(692, (s.v[37] - (2.0 * s.v[691])));s.store_scalar(693, (((s.v[38] / s.v[39]) - (s.v[58] * s.v[284])) - ((2.0 - s.v[58]) * s.v[690])));s.store_scalar(710, ((s.v[693] / s.v[59]) + s.v[60]));s.store_scalar(711, ((s.v[693] / s.v[59]) + s.v[61]));s.store_scalar(726, ((s.v[37] - (2.0 * s.v[691])) - s.v[341]));s.store_scalar(727, (s.v[726] + (2.0 * s.v[353])));s.store_scalar(482, s.v[111]);s.store_scalar(483, s.v[112]);s.store_scalar(484, s.v[113]);s.store_scalar(486, s.v[114]);s.store_scalar(487, s.v[115]);s.copy_ad(605, 232);s.store_scalar(606, s.v[233]);s.store_scalar(607, s.v[234]);s.store_scalar(694, (1.0 + (((s.v[606] / s.v[688])) as f64).powf(s.v[607])));s.b[895] = (s.v[90] == 1.0);s.store_scalar(895, if s.b[895] { 1.0 } else { 0.0 });
        if s.b[895] {s.store_scalar(828, (1e-6 / s.v[688]));s.store_scalar(829, (1e-6 / s.v[689]));s.store_scalar(830, (1e-12 / (s.v[688] * s.v[689])));}
        if (!s.b[895]) {s.store_scalar(828, (1.0 / s.v[688]));s.store_scalar(829, (1.0 / s.v[689]));s.store_scalar(830, (1.0 / (s.v[688] * s.v[689])));}
        s.store_add_scaled_inputs3_offset_indices(478, 828, p.p461, 829, p.p642, 830, p.p823, s.v[108]);s.store_add_scaled_inputs3_offset_indices(479, 828, p.p462, 829, p.p643, 830, p.p824, s.v[107]);s.store_add_scaled_inputs3_offset_indices(480, 828, p.p463, 829, p.p644, 830, p.p826, s.v[109]);s.store_add_scaled_inputs3_offset_indices(481, 828, p.p464, 829, p.p645, 830, p.p825, s.v[110]);s.store_add_scaled_inputs3_offset_indices(507, 828, p.p465, 829, p.p646, 830, p.p827, s.v[134]);s.store_add_scaled_inputs3_offset_indices(522, 828, p.p466, 829, p.p647, 830, p.p828, s.v[135]);s.store_add_scaled_inputs3_offset_indices(490, 828, p.p467, 829, p.p648, 830, p.p829, s.v[116]);s.store_add_scaled_inputs3_offset_indices(494, 828, p.p470, 829, p.p651, 830, p.p832, s.v[120]);s.store_add_scaled_inputs3_offset_indices(627, 828, p.p468, 829, p.p649, 830, p.p830, s.v[281]);s.store_add_scaled_inputs3_offset_indices(628, 828, p.p469, 829, p.p650, 830, p.p831, s.v[282]);s.store_add_scaled_inputs3_offset_indices(495, 828, p.p471, 829, p.p652, 830, p.p833, s.v[121]);s.store_add_scaled_inputs3_offset_indices(496, 828, p.p472, 829, p.p653, 830, p.p834, s.v[122]);s.store_add_scaled_inputs3_offset_indices(626, 828, p.p473, 829, p.p654, 830, p.p835, s.v[352]);s.store_add_scaled_inputs3_offset_indices(497, 828, p.p474, 829, p.p655, 830, p.p836, s.v[123]);s.store_add_scaled_inputs3_offset_indices(498, 828, p.p976, 829, p.p979, 830, p.p982, s.v[124]);s.store_add_scaled_inputs3_offset_indices(738, 828, p.p475, 829, p.p656, 830, p.p837, s.v[125]);s.store_add_scaled_inputs3_offset_indices(499, 828, p.p476, 829, p.p657, 830, p.p838, s.v[126]);s.store_add_scaled_inputs3_offset_indices(500, 828, p.p477, 829, p.p658, 830, p.p839, s.v[127]);s.store_add_scaled_inputs3_offset_indices(501, 828, p.p478, 829, p.p659, 830, p.p840, s.v[128]);s.store_add_scaled_inputs3_offset_indices(502, 828, p.p479, 829, p.p660, 830, p.p841, s.v[129]);s.store_add_scaled_inputs3_offset_indices(503, 828, p.p480, 829, p.p661, 830, p.p842, s.v[130]);s.store_add_scaled_inputs3_offset_indices(504, 828, p.p481, 829, p.p662, 830, p.p843, s.v[131]);s.store_add_scaled_inputs3_offset_indices(514, 828, p.p482, 829, p.p663, 830, p.p844, s.v[142]);s.store_add_scaled_inputs3_offset_indices(508, 828, p.p484, 829, p.p665, 830, p.p846, s.v[136]);s.store_add_scaled_inputs3_offset_indices(510, 828, p.p485, 829, p.p666, 830, p.p847, s.v[138]);s.store_add_scaled_inputs3_offset_indices(512, 828, p.p486, 829, p.p667, 830, p.p848, s.v[140]);s.store_add_scaled_inputs3_offset_indices(471, 828, p.p491, 829, p.p672, 830, p.p853, s.v[100]);s.store_add_scaled_inputs3_offset_indices(473, 828, p.p492, 829, p.p673, 830, p.p854, s.v[102]);s.store_add_scaled_inputs3_offset_indices(474, 828, p.p493, 829, p.p674, 830, p.p855, s.v[103]);s.store_add_scaled_inputs3_offset_indices(568, 828, p.p494, 829, p.p675, 830, p.p856, s.v[227]);s.store_add_scaled_inputs3_offset_indices(569, 828, p.p495, 829, p.p676, 830, p.p857, s.v[228]);s.store_add_scaled_inputs3_offset_indices(477, 828, p.p496, 829, p.p677, 830, p.p858, s.v[106]);s.store_add_scaled_inputs3_offset_indices(629, 828, p.p497, 829, p.p678, 830, p.p859, s.v[283]);s.store_add_scaled_inputs3_offset_indices(475, 828, p.p498, 829, p.p679, 830, p.p860, s.v[104]);s.store_add_scaled_inputs3_offset_indices(476, 828, p.p499, 829, p.p680, 830, p.p861, s.v[105]);s.store_add_scaled_inputs3_offset_indices(551, 828, p.p500, 829, p.p681, 830, p.p862, s.v[156]);s.store_add_scaled_inputs3_offset_indices(540, 828, p.p501, 829, p.p682, 830, p.p863, s.v[157]);s.store_add_scaled_inputs3_offset_indices(539, 828, p.p502, 829, p.p683, 830, p.p864, s.v[158]);s.store_add_scaled_inputs3_offset_indices(554, 828, p.p503, 829, p.p684, 830, p.p865, s.v[162]);s.store_add_scaled_inputs3_offset_indices(553, 828, p.p504, 829, p.p685, 830, p.p866, s.v[161]);s.store_add_scaled_inputs3_offset_indices(565, 828, p.p505, 829, p.p686, 830, p.p867, s.v[215]);s.store_add_scaled_inputs3_offset_indices(470, 828, p.p506, 829, p.p687, 830, p.p868, s.v[99]);
        s.store_add_scaled_inputs3_offset_indices(566, 828, p.p507, 829, p.p688, 830, p.p869, s.v[217]);s.store_add_scaled_inputs3_offset_indices(567, 828, p.p508, 829, p.p689, 830, p.p870, s.v[218]);s.store_add_scaled_inputs3_offset_indices(521, 828, p.p509, 829, p.p690, 830, p.p871, s.v[149]);s.store_add_scaled_inputs3_offset_indices(556, 828, p.p510, 829, p.p691, 830, p.p872, s.v[164]);s.store_add_scaled_inputs3_offset_indices(557, 828, p.p511, 829, p.p692, 830, p.p873, s.v[165]);s.store_add_scaled_inputs3_offset_indices(558, 828, p.p512, 829, p.p693, 830, p.p874, s.v[166]);s.store_add_scaled_inputs3_offset_indices(559, 828, p.p513, 829, p.p694, 830, p.p875, s.v[167]);s.store_add_scaled_inputs3_offset_indices(506, 828, p.p514, 829, p.p695, 830, p.p876, s.v[133]);s.store_add_scaled_inputs3_offset_indices(469, 828, p.p515, 829, p.p696, 830, p.p877, s.v[98]);s.store_add_scaled_inputs3_offset_indices(466, 828, p.p516, 829, p.p697, 830, p.p878, s.v[95]);s.store_add_scaled_inputs3_offset_indices(467, 828, p.p517, 829, p.p698, 830, p.p879, s.v[96]);s.store_add_scaled_inputs3_offset_indices(468, 828, p.p518, 829, p.p699, 830, p.p880, s.v[97]);s.store_add_scaled_inputs3_offset_indices(560, 828, p.p519, 829, p.p700, 830, p.p881, s.v[168]);s.store_add_scaled_inputs3_offset_indices(561, 828, p.p520, 829, p.p701, 830, p.p882, s.v[169]);s.store_add_scaled_inputs3_offset_indices(562, 828, p.p521, 829, p.p702, 830, p.p883, s.v[170]);s.store_add_scaled_inputs3_offset_indices(563, 828, p.p522, 829, p.p703, 830, p.p884, s.v[171]);s.store_add_scaled_inputs3_offset_indices(505, 828, p.p523, 829, p.p704, 830, p.p885, s.v[132]);s.store_add_scaled_inputs3_offset_indices(564, 828, p.p524, 829, p.p705, 830, p.p886, s.v[172]);s.store_add_scaled_inputs3_offset_indices(550, 828, p.p525, 829, p.p706, 830, p.p887, s.v[154]);s.store_add_scaled_inputs3_offset_indices(570, 828, p.p526, 829, p.p707, 830, p.p888, s.v[237]);s.store_add_scaled_inputs3_offset_indices(630, 828, p.p527, 829, p.p708, 830, p.p889, s.v[295]);s.store_add_scaled_inputs3_offset_indices(631, 828, p.p530, 829, p.p711, 830, p.p892, s.v[296]);s.store_add_scaled_inputs3_offset_indices(632, 828, p.p529, 829, p.p710, 830, p.p891, s.v[297]);s.store_add_scaled_inputs3_offset_indices(633, 828, p.p532, 829, p.p713, 830, p.p894, s.v[298]);s.store_add_scaled_inputs3_offset_indices(634, 828, p.p528, 829, p.p709, 830, p.p890, s.v[299]);s.store_add_scaled_inputs3_offset_indices(635, 828, p.p531, 829, p.p712, 830, p.p893, s.v[300]);s.store_add_scaled_inputs3_offset_indices(571, 828, p.p533, 829, p.p714, 830, p.p895, s.v[285]);s.store_add_scaled_inputs3_offset_indices(636, 828, p.p534, 829, p.p715, 830, p.p896, s.v[286]);s.store_add_scaled_inputs3_offset_indices(637, 828, p.p535, 829, p.p716, 830, p.p897, s.v[287]);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_inputs3_offset_indices(638, 828, p.p536, 829, p.p717, 830, p.p898, s.v[288]);s.store_add_scaled_inputs3_offset_indices(639, 828, p.p537, 829, p.p718, 830, p.p899, s.v[290]);s.store_add_scaled_inputs3_offset_indices(640, 828, p.p538, 829, p.p719, 830, p.p900, s.v[302]);s.store_add_scaled_inputs3_offset_indices(641, 828, p.p539, 829, p.p720, 830, p.p901, s.v[291]);s.store_add_scaled_inputs3_offset_indices(642, 828, p.p540, 829, p.p721, 830, p.p902, s.v[292]);s.store_add_scaled_inputs3_offset_indices(643, 828, p.p541, 829, p.p722, 830, p.p903, s.v[293]);s.store_add_scaled_inputs3_offset_indices(644, 828, p.p542, 829, p.p723, 830, p.p904, s.v[294]);s.store_add_scaled_inputs3_offset_indices(645, 828, p.p543, 829, p.p724, 830, p.p905, s.v[178]);s.store_add_scaled_inputs3_offset_indices(646, 828, p.p544, 829, p.p725, 830, p.p906, s.v[179]);s.store_add_scaled_inputs3_offset_indices(647, 828, p.p545, 829, p.p726, 830, p.p907, s.v[180]);s.store_add_scaled_inputs3_offset_indices(648, 828, p.p977, 829, p.p980, 830, p.p983, s.v[177]);s.store_add_scaled_inputs3_offset_indices(649, 828, p.p546, 829, p.p727, 830, p.p908, s.v[181]);s.store_add_scaled_inputs3_offset_indices(650, 828, p.p547, 829, p.p728, 830, p.p909, s.v[182]);s.store_add_scaled_inputs3_offset_indices(651, 828, p.p548, 829, p.p729, 830, p.p910, s.v[183]);s.store_add_scaled_inputs3_offset_indices(652, 828, p.p549, 829, p.p730, 830, p.p911, s.v[185]);s.store_add_scaled_inputs3_offset_indices(653, 828, p.p550, 829, p.p731, 830, p.p912, s.v[186]);s.store_add_scaled_inputs3_offset_indices(654, 828, p.p551, 829, p.p732, 830, p.p913, s.v[187]);s.store_add_scaled_inputs3_offset_indices(655, 828, p.p978, 829, p.p981, 830, p.p984, s.v[184]);s.store_add_scaled_inputs3_offset_indices(656, 828, p.p552, 829, p.p733, 830, p.p914, s.v[188]);s.store_add_scaled_inputs3_offset_indices(657, 828, p.p553, 829, p.p734, 830, p.p915, s.v[189]);s.store_add_scaled_inputs3_offset_indices(658, 828, p.p554, 829, p.p735, 830, p.p916, s.v[190]);s.store_add_scaled_inputs3_offset_indices(659, 828, p.p555, 829, p.p736, 830, p.p917, s.v[303]);s.store_add_scaled_inputs3_offset_indices(660, 828, p.p556, 829, p.p737, 830, p.p918, s.v[304]);s.store_add_scaled_inputs3_offset_indices(661, 828, p.p557, 829, p.p738, 830, p.p919, s.v[191]);s.store_add_scaled_inputs3_offset_indices(662, 828, p.p558, 829, p.p739, 830, p.p920, s.v[192]);s.store_add_scaled_inputs3_offset_indices(663, 828, p.p559, 829, p.p740, 830, p.p921, s.v[305]);s.store_add_scaled_inputs3_offset_indices(664, 828, p.p560, 829, p.p741, 830, p.p922, s.v[306]);s.store_add_scaled_inputs3_offset_indices(665, 828, p.p561, 829, p.p742, 830, p.p923, s.v[307]);s.store_add_scaled_inputs3_offset_indices(666, 828, p.p562, 829, p.p743, 830, p.p924, s.v[308]);s.store_add_scaled_inputs3_offset_indices(667, 828, p.p563, 829, p.p744, 830, p.p925, s.v[309]);s.store_add_scaled_inputs3_offset_indices(668, 828, p.p564, 829, p.p745, 830, p.p926, s.v[310]);s.store_add_scaled_inputs3_offset_indices(669, 828, p.p565, 829, p.p746, 830, p.p927, s.v[311]);s.store_add_scaled_inputs3_offset_indices(670, 828, p.p566, 829, p.p747, 830, p.p928, s.v[312]);s.store_add_scaled_inputs3_offset_indices(671, 828, p.p567, 829, p.p748, 830, p.p929, s.v[313]);s.store_add_scaled_inputs3_offset_indices(673, 828, p.p569, 829, p.p750, 830, p.p931, s.v[315]);s.store_add_scaled_inputs3_offset_indices(672, 828, p.p568, 829, p.p749, 830, p.p930, s.v[314]);s.store_add_scaled_inputs3_offset_indices(674, 828, p.p570, 829, p.p751, 830, p.p932, s.v[316]);s.store_add_scaled_inputs3_offset_indices(675, 828, p.p571, 829, p.p752, 830, p.p933, s.v[318]);s.store_add_scaled_inputs3_offset_indices(676, 828, p.p572, 829, p.p753, 830, p.p934, s.v[319]);s.store_add_scaled_inputs3_offset_indices(677, 828, p.p573, 829, p.p754, 830, p.p935, s.v[320]);s.store_add_scaled_inputs3_offset_indices(678, 828, p.p574, 829, p.p755, 830, p.p936, s.v[321]);s.store_add_scaled_inputs3_offset_indices(679, 828, p.p575, 829, p.p756, 830, p.p937, s.v[322]);
        s.store_add_scaled_inputs3_offset_indices(680, 828, p.p576, 829, p.p757, 830, p.p938, s.v[323]);s.store_add_scaled_inputs3_offset_indices(681, 828, p.p577, 829, p.p758, 830, p.p939, s.v[325]);s.store_add_scaled_inputs3_offset_indices(682, 828, p.p578, 829, p.p759, 830, p.p940, s.v[326]);s.store_add_scaled_inputs3_offset_indices(716, 828, p.p579, 829, p.p760, 830, p.p941, s.v[327]);s.store_add_scaled_inputs3_offset_indices(717, 828, p.p580, 829, p.p761, 830, p.p942, s.v[328]);s.store_add_scaled_inputs3_offset_indices(608, 828, p.p422, 829, p.p603, 830, p.p784, s.v[176]);s.store_add_scaled_inputs3_offset_indices(609, 828, p.p423, 829, p.p604, 830, p.p785, s.v[364]);s.store_add_scaled_inputs3_offset_indices(611, 828, p.p425, 829, p.p606, 830, p.p787, s.v[368]);s.store_add_scaled_inputs3_offset_indices(610, 828, p.p424, 829, p.p605, 830, p.p786, s.v[365]);s.store_add_scaled_inputs3_offset_indices(612, 828, p.p426, 829, p.p607, 830, p.p788, s.v[369]);s.store_add_scaled_inputs3_offset_indices(616, 828, p.p433, 829, p.p614, 830, p.p795, s.v[333]);s.store_add_scaled_inputs3_offset_indices(617, 828, p.p443, 829, p.p624, 830, p.p805, s.v[339]);s.store_add_scaled_inputs3_offset_indices(618, 828, p.p444, 829, p.p625, 830, p.p806, s.v[340]);s.store_add_scaled_inputs3_offset_indices(619, 828, p.p445, 829, p.p626, 830, p.p807, s.v[193]);s.store_add_scaled_inputs3_offset_indices(620, 828, p.p446, 829, p.p627, 830, p.p808, s.v[194]);s.store_add_scaled_inputs3_offset_indices(621, 828, p.p447, 829, p.p628, 830, p.p809, s.v[195]);s.store_add_scaled_inputs3_offset_indices(622, 828, p.p448, 829, p.p629, 830, p.p810, s.v[196]);s.store_add_scaled_inputs3_offset_indices(623, 828, p.p449, 829, p.p630, 830, p.p811, s.v[197]);s.store_add_scaled_inputs3_offset_indices(624, 828, p.p450, 829, p.p631, 830, p.p812, s.v[198]);s.store_add_scaled_inputs3_offset_indices(625, 828, p.p451, 829, p.p632, 830, p.p813, s.v[199]);s.store_add_scaled_inputs3_offset_indices(603, 828, p.p431, 829, p.p612, 830, p.p793, s.v[230]);s.store_add_scaled_inputs3_offset_indices(602, 828, p.p430, 829, p.p611, 830, p.p792, s.v[229]);s.store_add_scaled_inputs3_offset_indices(604, 828, p.p432, 829, p.p613, 830, p.p794, s.v[231]);s.store_add_scaled_inputs3_offset_indices(515, 828, p.p434, 829, p.p615, 830, p.p796, s.v[144]);s.store_add_scaled_inputs3_offset_indices(516, 828, p.p487, 829, p.p668, 830, p.p849, s.v[147]);s.store_add_scaled_inputs3_offset_indices(517, 828, p.p488, 829, p.p669, 830, p.p850, s.v[148]);s.store_add_scaled_inputs3_offset_indices(518, 828, p.p483, 829, p.p664, 830, p.p845, s.v[143]);s.store_add_scaled_inputs3_offset_indices(519, 828, p.p490, 829, p.p671, 830, p.p852, s.v[145]);s.store_add_scaled_inputs3_offset_indices(520, 828, p.p489, 829, p.p670, 830, p.p851, s.v[146]);s.store_add_scaled_inputs3_offset_indices(491, 828, p.p435, 829, p.p616, 830, p.p797, s.v[117]);s.store_add_scaled_inputs3_offset_indices(493, 828, p.p437, 829, p.p618, 830, p.p799, s.v[119]);s.store_add_scaled_inputs3_offset_indices(492, 828, p.p436, 829, p.p617, 830, p.p798, s.v[118]);s.store_add_scaled_inputs3_offset_indices(509, 828, p.p438, 829, p.p619, 830, p.p800, s.v[137]);s.store_add_scaled_inputs3_offset_indices(511, 828, p.p439, 829, p.p620, 830, p.p801, s.v[139]);s.store_add_scaled_inputs3_offset_indices(513, 828, p.p440, 829, p.p621, 830, p.p802, s.v[141]);s.store_add_scaled_inputs3_offset_indices(472, 828, p.p441, 829, p.p622, 830, p.p803, s.v[101]);s.store_add_scaled_inputs3_offset_indices(555, 828, p.p442, 829, p.p623, 830, p.p804, s.v[163]);s.store_add_scaled_inputs3_offset_indices(578, 828, p.p458, 829, p.p639, 830, p.p820, s.v[382]);s.store_add_scaled_inputs3_offset_indices(572, 828, p.p452, 829, p.p633, 830, p.p814, s.v[376]);s.store_add_scaled_inputs3_offset_indices(573, 828, p.p453, 829, p.p634, 830, p.p815, s.v[377]);s.store_add_scaled_inputs3_offset_indices(574, 828, p.p454, 829, p.p635, 830, p.p816, s.v[378]);s.store_add_scaled_inputs3_offset_indices(575, 828, p.p455, 829, p.p636, 830, p.p817, s.v[379]);
        s.store_add_scaled_inputs3_offset_indices(576, 828, p.p456, 829, p.p637, 830, p.p818, s.v[380]);s.store_add_scaled_inputs3_offset_indices(577, 828, p.p457, 829, p.p638, 830, p.p819, s.v[381]);s.store_add_scaled_inputs3_offset_indices(579, 828, p.p459, 829, p.p640, 830, p.p821, s.v[383]);s.store_add_scaled_inputs3_offset_indices(580, 828, p.p460, 829, p.p641, 830, p.p822, s.v[384]);s.store_add_scaled_inputs3_offset_indices(595, 828, p.p588, 829, p.p769, 830, p.p950, s.v[397]);s.store_add_scaled_inputs3_offset_indices(596, 828, p.p589, 829, p.p770, 830, p.p951, s.v[398]);s.store_add_scaled_inputs3_offset_indices(582, 828, p.p590, 829, p.p771, 830, p.p952, s.v[388]);s.store_add_scaled_inputs3_offset_indices(583, 828, p.p591, 829, p.p772, 830, p.p953, s.v[405]);s.store_add_scaled_inputs3_offset_indices(584, 828, p.p592, 829, p.p773, 830, p.p954, s.v[406]);s.store_add_scaled_inputs3_offset_indices(585, 828, p.p593, 829, p.p774, 830, p.p955, s.v[389]);s.store_add_scaled_inputs3_offset_indices(586, 828, p.p594, 829, p.p775, 830, p.p956, s.v[390]);s.store_add_scaled_inputs3_offset_indices(587, 828, p.p595, 829, p.p776, 830, p.p957, s.v[391]);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_inputs3_offset_indices(588, 828, p.p596, 829, p.p777, 830, p.p958, s.v[392]);s.store_add_scaled_inputs3_offset_indices(589, 828, p.p597, 829, p.p778, 830, p.p959, s.v[393]);s.store_add_scaled_inputs3_offset_indices(590, 828, p.p598, 829, p.p779, 830, p.p960, s.v[394]);s.store_add_scaled_inputs3_offset_indices(591, 828, p.p599, 829, p.p780, 830, p.p961, s.v[395]);s.store_add_scaled_inputs3_offset_indices(592, 828, p.p600, 829, p.p781, 830, p.p962, s.v[396]);let t6: f64 = (p.p601 * s.v[828]);let t7: f64 = (s.v[386] + t6);let t8: f64 = (p.p782 * s.v[829]);let t9: f64 = (t7 + t8);let ta: f64 = (p.p963 * s.v[830]);let tb: f64 = (t9 + ta);s.store_scalar(593, tb);let tc: f64 = (p.p602 * s.v[828]);let td: f64 = (s.v[387] + tc);let te: f64 = (p.p783 * s.v[829]);let tf: f64 = (td + te);let t10: f64 = (p.p964 * s.v[830]);let t11: f64 = (tf + t10);s.store_scalar(594, t11);s.store_add_scaled_inputs3_offset_indices(683, 828, p.p581, 829, p.p762, 830, p.p943, s.v[334]);s.store_add_scaled_inputs3_offset_indices(684, 828, p.p582, 829, p.p763, 830, p.p944, s.v[335]);s.store_add_scaled_inputs3_offset_indices(685, 828, p.p583, 829, p.p764, 830, p.p945, s.v[351]);s.store_add_scaled_inputs3_offset_indices(722, 828, p.p584, 829, p.p765, 830, p.p946, s.v[347]);s.store_mul_powf_mixed_ia(722, 722, A::scale(s.ad_value(478), 5e-17), (-0.25));s.store_add_scaled_inputs3_offset_indices(723, 828, p.p585, 829, p.p766, 830, p.p947, s.v[348]);s.store_add_scaled_inputs3_offset_indices(724, 828, p.p586, 829, p.p767, 830, p.p948, s.v[349]);s.store_add_scaled_inputs3_offset_indices(725, 828, p.p587, 829, p.p768, 830, p.p949, s.v[350]);s.store_add_scaled_inputs3_offset_indices(739, 828, p.p246, 829, p.p247, 830, p.p248, s.v[266]);s.store_add_scaled_inputs3_offset_indices(740, 828, p.p250, 829, p.p251, 830, p.p252, s.v[267]);s.store_add_scaled_inputs3_offset_indices(741, 828, p.p254, 829, p.p255, 830, p.p256, s.v[268]);s.store_add_scaled_inputs3_offset_indices(742, 828, p.p258, 829, p.p259, 830, p.p260, s.v[269]);s.store_add_scaled_inputs3_offset_indices(743, 828, p.p262, 829, p.p263, 830, p.p264, s.v[270]);s.store_add_scaled_inputs3_offset_indices(744, 828, p.p266, 829, p.p267, 830, p.p268, s.v[271]);s.store_add_scaled_inputs3_offset_indices(750, 828, p.p415, 829, p.p416, 830, p.p417, s.v[407]);s.store_add_scaled_inputs3_offset_indices(751, 828, p.p419, 829, p.p420, 830, p.p421, s.v[408]);s.store_add_scaled_inputs3_offset_indices(746, 828, p.p273, 829, p.p276, 830, p.p279, s.v[275]);s.store_add_scaled_inputs3_offset_indices(747, 828, p.p274, 829, p.p277, 830, p.p280, s.v[272]);s.store_add_scaled_inputs3_offset_indices(748, 828, p.p275, 829, p.p278, 830, p.p281, s.v[274]);s.store_add_scaled_inputs3_offset_indices(613, 828, p.p427, 829, p.p608, 830, p.p789, s.v[371]);s.store_add_scaled_inputs3_offset_indices(614, 828, p.p428, 829, p.p609, 830, p.p790, s.v[372]);s.store_add_scaled_inputs3_offset_indices(615, 828, p.p429, 829, p.p610, 830, p.p791, s.v[373]);s.store_offset_scaled_ad(745, A::atan(s.ad_value(744)), 0.3183098861837907, 0.5);s.store_offset_scaled_ad(749, A::atan(s.ad_value(750)), 0.3183098861837907, 0.5);s.store_scalar(818, (s.v[827] - 1.0));s.copy_ad(523, 508);s.copy_ad(524, 510);s.copy_ad(525, 512);s.store_pow_from_scalar_ad(529, (s.v[689] * 1000000.0), s.ad_value(565));s.store_scalar(527, ((s.v[50] / (s.v[39] * (s.v[689] + s.v[358]))) * s.v[59]));s.store_scalar(528, ((s.v[51] * (s.v[39] * (s.v[689] + s.v[358]))) / s.v[59]));s.b[897] = (s.v[329] == 0.0);s.store_scalar(897, if s.b[897] { 1.0 } else { 0.0 });
        if s.b[897] {s.store_scalar(526, 0.0);}
        if (!s.b[897]) {s.store_scalar(526, ((((((s.v[53] * s.v[329]) * s.v[359]) / ((2.0 * s.v[329]) + (s.v[359] * s.v[688]))) * s.v[689]) / s.v[59]) / s.v[39]));}
        s.store_scalar(706, (((((s.v[361] / s.v[357])) as f64).powf(s.v[360]) / s.v[357]) / s.v[357]));s.store_add_scaled_inputs(508, 508, 1.0, 509, s.v[818]);s.store_add_scaled_inputs(510, 510, 1.0, 511, s.v[818]);s.store_add_scaled_inputs(512, 512, 1.0, 513, s.v[818]);s.b[898] = (s.v[514] > 1.0);s.store_scalar(898, if s.b[898] { 1.0 } else { 0.0 });
        if s.b[898] {s.store_scale(514, 514, 0.0001);}
        s.store_mul_mixed_ia(698, 514, A::pow_from_scalar(s.v[827], s.ad_value(515)));s.store_sub_scaled_inputs(699, 471, 1.0, 472, s.v[818]);s.store_div_scaled_inputs2_indices(552, 551, 1.0, 555, s.v[818], 529, 1.0);s.b[899] = (s.v[403] == 1.0);s.store_scalar(899, if s.b[899] { 1.0 } else { 0.0 });
        if s.b[899] {s.store_scale(848, 529, s.v[39]);s.store_scale(849, 555, s.v[818]);s.store_add(819, 539, 849);s.store_offset(820, 849, s.v[160]);}
        s.b[900] = (s.v[819] < 0.0);s.store_scalar(900, if s.b[900] { 1.0 } else { 0.0 });
        if (s.b[899] && s.b[900]) {s.store_scalar(819, 0.0);}
        s.b[901] = (s.v[820] < 0.0);s.store_scalar(901, if s.b[901] { 1.0 } else { 0.0 });
        if (s.b[899] && s.b[901]) {s.store_scalar(820, 0.0);}
        if s.b[899] {s.store_div(543, 819, 848);s.store_div(541, 820, 848);s.store_add(821, 540, 849);s.store_offset(822, 849, s.v[159]);}
        s.b[902] = (s.v[821] < 0.0);s.store_scalar(902, if s.b[902] { 1.0 } else { 0.0 });
        if (s.b[899] && s.b[902]) {s.store_scalar(821, 0.0);}
        s.b[903] = (s.v[822] < 0.0);s.store_scalar(903, if s.b[903] { 1.0 } else { 0.0 });
        if (s.b[899] && s.b[903]) {s.store_scalar(822, 0.0);}
        if s.b[899] {s.store_div(544, 821, 848);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[899] {s.store_div(542, 822, 848);}
        if (!s.b[899]) {s.store_scalar(543, 0.0);s.store_scalar(541, 0.0);s.store_scalar(544, 0.0);s.store_scalar(542, 0.0);}
        s.b[904] = (s.v[152] < 0.0);s.store_scalar(904, if s.b[904] { 1.0 } else { 0.0 });
        if s.b[904] {s.store_scalar(152, 0.0);}
        s.b[905] = (s.v[151] < 0.0);s.store_scalar(905, if s.b[905] { 1.0 } else { 0.0 });
        if s.b[905] {s.store_scalar(151, 0.0);}
        s.b[906] = (s.v[331] < 0.0);s.store_scalar(906, if s.b[906] { 1.0 } else { 0.0 });
        if s.b[906] {s.store_scalar(331, 0.0);}
        s.store_scaled_add(696, 152, 605, s.v[710]);s.store_scaled_add(695, 151, 605, s.v[711]);s.store_scale(697, 331, (s.v[692] * s.v[39]));s.b[907] = ((!param_given[81]) && param_given[84]);s.store_scalar(907, if s.b[907] { 1.0 } else { 0.0 });
        if s.b[907] {s.store_scale(818, 757, s.v[482]);s.store_scaled_mul(478, 818, 818, 3.021e22);}
        s.b[908] = (s.v[57] == 2.0);s.store_scalar(908, if s.b[908] { 1.0 } else { 0.0 });
        if (s.b[908] && (s.v[68] != 0.0)) {s.store_primal_scale(794, 778, ((((s.v[76] - 0.1) / 1.60219e-19) * 2e-6) * 1.0 / ((s.v[175] * s.v[175]))));}
        s.b[909] = (s.v[478] > s.v[794]);s.store_scalar(909, if s.b[909] { 1.0 } else { 0.0 });
        if ((s.b[908] && (s.v[68] != 0.0)) && s.b[909]) {s.copy_ad(478, 794);}
        if (s.b[908] && (s.v[68] == 0.0)) {s.store_primal_scale(794, 778, ((((1.12 - 0.1) / 1.60219e-19) * 2e-6) * 1.0 / ((s.v[174] * s.v[174]))));}
        s.b[910] = (s.v[478] > s.v[794]);s.store_scalar(910, if s.b[910] { 1.0 } else { 0.0 });
        if ((s.b[908] && (s.v[68] == 0.0)) && s.b[910]) {s.copy_ad(478, 794);}
        s.store_scalar(753, (3.453133e-11 / s.v[173]));
        if (s.v[68] != 0.0) {s.store_scalar(754, (1.03594e-10 / s.v[175]));}
        if (s.v[68] == 0.0) {s.store_scalar(754, (1.03594e-10 / s.v[174]));}
        let (t18,) = {
    if (s.v[68] != 0.0) {
        let t12: f64 = (1.60219e-19 * s.v[478]);let t13: f64 = (s.v[124] / s.v[37]);let t14: f64 = (1.0 + t13);let t15: f64 = (t12 * t14);let t16: f64 = (t15 * 1000000.0);let t17: f64 = (t16 * s.v[175]);
        (t17,)
    } else {
        (s.v[792],)
    }
};
        s.store_scalar(792, t18);
        let (t1f,) = {
    if (s.v[68] == 0.0) {
        let t19: f64 = (1.60219e-19 * s.v[478]);let t1a: f64 = (s.v[124] / s.v[37]);let t1b: f64 = (1.0 + t1a);let t1c: f64 = (t19 * t1b);let t1d: f64 = (t1c * 1000000.0);let t1e: f64 = (t1d * s.v[174]);
        (t1e,)
    } else {
        (s.v[792],)
    }
};
        s.store_scalar(792, t1f);let t20: f64 = (0.5 * s.v[792]);let t21: f64 = (t20 / s.v[754]);let t22: f64 = (0.8 - t21);let t23: f64 = (t22 + s.v[582]);s.store_scalar(793, t23);s.b[911] = (s.v[57] == 3.0);s.store_scalar(911, if s.b[911] { 1.0 } else { 0.0 });s.b[912] = (s.v[793] > s.v[594]);s.store_scalar(912, if s.b[912] { 1.0 } else { 0.0 });
        let (t24,) = {
    if (s.b[911] && s.b[912]) {
        (2.0,)
    } else {
        (s.v[57],)
    }
};
        s.store_scalar(57, t24);s.b[913] = (s.v[793] < s.v[593]);s.store_scalar(913, if s.b[913] { 1.0 } else { 0.0 });
        let (t25,) = {
    if ((s.b[911] && (!s.b[912])) && s.b[913]) {
        (0.0,)
    } else {
        (s.v[57],)
    }
};
        s.store_scalar(57, t25);
        let (t26,) = {
    if ((s.b[911] && (!s.b[912])) && (!s.b[913])) {
        (1.0,)
    } else {
        (s.v[57],)
    }
};
        s.store_scalar(57, t26);s.store_scale_ad(822, A::div_from_scalar(1.115, s.ad_value(409)), (s.v[827] - 1.0));s.store_div_scaled_product_indices(884, 619, 822, 1.0, 661, 1.0);s.b[914] = (s.v[884] > 100.0);s.store_scalar(914, if s.b[914] { 1.0 } else { 0.0 });
        if s.b[914] {s.store_scaled_offset(818, 884, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[915] = (s.v[884] < (-100.0));s.store_scalar(915, if s.b[915] { 1.0 } else { 0.0 });
        if ((!s.b[914]) && s.b[915]) {s.store_scalar(818, 3.720075976e-44);}
        if ((!s.b[914]) && (!s.b[915])) {s.store_exp(818, 884);}
        s.store_div_scaled_product_indices(884, 620, 822, 1.0, 661, 1.0);s.b[916] = (s.v[884] > 100.0);s.store_scalar(916, if s.b[916] { 1.0 } else { 0.0 });
        if s.b[916] {s.store_scaled_offset(819, 884, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[917] = (s.v[884] < (-100.0));s.store_scalar(917, if s.b[917] { 1.0 } else { 0.0 });
        if ((!s.b[916]) && s.b[917]) {s.store_scalar(819, 3.720075976e-44);}
        if ((!s.b[916]) && (!s.b[917])) {s.store_exp(819, 884);}
        s.store_div_scaled_product_indices(884, 621, 822, 1.0, 663, 1.0);s.b[918] = (s.v[884] > 100.0);s.store_scalar(918, if s.b[918] { 1.0 } else { 0.0 });
        if s.b[918] {s.store_scaled_offset(820, 884, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[919] = (s.v[884] < (-100.0));s.store_scalar(919, if s.b[919] { 1.0 } else { 0.0 });
        if ((!s.b[918]) && s.b[919]) {s.store_scalar(820, 3.720075976e-44);}
        if ((!s.b[918]) && (!s.b[919])) {s.store_exp(820, 884);}
        s.store_mul(718, 716, 818);s.store_mul(531, 667, 818);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_mul(533, 669, 819);s.store_mul(535, 671, 820);s.store_scale(884, 622, (s.v[827] - 1.0));s.b[920] = (s.v[884] > 100.0);s.store_scalar(920, if s.b[920] { 1.0 } else { 0.0 });
        if s.b[920] {s.store_scaled_offset(818, 884, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[921] = (s.v[884] < (-100.0));s.store_scalar(921, if s.b[921] { 1.0 } else { 0.0 });
        if ((!s.b[920]) && s.b[921]) {s.store_scalar(818, 3.720075976e-44);}
        if ((!s.b[920]) && (!s.b[921])) {s.store_exp(818, 884);}
        s.store_mul(537, 673, 818);s.store_div_scaled_product_indices(884, 619, 822, 1.0, 662, 1.0);s.b[922] = (s.v[884] > 100.0);s.store_scalar(922, if s.b[922] { 1.0 } else { 0.0 });
        if s.b[922] {s.store_scaled_offset(818, 884, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[923] = (s.v[884] < (-100.0));s.store_scalar(923, if s.b[923] { 1.0 } else { 0.0 });
        if ((!s.b[922]) && s.b[923]) {s.store_scalar(818, 3.720075976e-44);}
        if ((!s.b[922]) && (!s.b[923])) {s.store_exp(818, 884);}
        s.store_div_scaled_product_indices(884, 623, 822, 1.0, 662, 1.0);s.b[924] = (s.v[884] > 100.0);s.store_scalar(924, if s.b[924] { 1.0 } else { 0.0 });
        if s.b[924] {s.store_scaled_offset(819, 884, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[925] = (s.v[884] < (-100.0));s.store_scalar(925, if s.b[925] { 1.0 } else { 0.0 });
        if ((!s.b[924]) && s.b[925]) {s.store_scalar(819, 3.720075976e-44);}
        if ((!s.b[924]) && (!s.b[925])) {s.store_exp(819, 884);}
        s.store_div_scaled_product_indices(884, 624, 822, 1.0, 664, 1.0);s.b[926] = (s.v[884] > 100.0);s.store_scalar(926, if s.b[926] { 1.0 } else { 0.0 });
        if s.b[926] {s.store_scaled_offset(820, 884, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[927] = (s.v[884] < (-100.0));s.store_scalar(927, if s.b[927] { 1.0 } else { 0.0 });
        if ((!s.b[926]) && s.b[927]) {s.store_scalar(820, 3.720075976e-44);}
        if ((!s.b[926]) && (!s.b[927])) {s.store_exp(820, 884);}
        s.store_mul(719, 717, 818);s.store_mul(532, 668, 818);s.store_mul(534, 670, 819);s.store_mul(536, 672, 820);s.store_scale(884, 625, (s.v[827] - 1.0));s.b[928] = (s.v[884] > 100.0);s.store_scalar(928, if s.b[928] { 1.0 } else { 0.0 });
        if s.b[928] {s.store_scaled_offset(818, 884, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[929] = (s.v[884] < (-100.0));s.store_scalar(929, if s.b[929] { 1.0 } else { 0.0 });
        if ((!s.b[928]) && s.b[929]) {s.store_scalar(818, 3.720075976e-44);}
        if ((!s.b[928]) && (!s.b[929])) {s.store_exp(818, 884);}
        s.store_mul(538, 674, 818);s.b[930] = (s.v[479] > 0.0);s.store_scalar(930, if s.b[930] { 1.0 } else { 0.0 });
        if s.b[930] {
            s.store_mul_scale_offset_mixed_ia(530, 409, {
                if ((s.v[478] / s.v[479]) > 1e-38) {
                    A::ln(A::div(s.ad_value(478), s.ad_value(479)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (-s.v[36]), 0.0);
        }
        if (!s.b[930]) {
            s.store_mul_scale_offset_mixed_ia(530, 409, {
                if (((((-s.v[478]) * s.v[479]) / s.v[817]) / s.v[817]) > 1e-38) {
                    A::ln(A::div_scaled_product_by_product(s.ad_value(478), s.ad_value(479), -1.0, s.ad_value(817), s.ad_value(817), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (-s.v[36]), 0.0);
        }
        s.b[931] = (!param_given[340]);s.store_scalar(931, if s.b[931] { 1.0 } else { 0.0 });s.b[932] = (s.v[479] > 0.0);s.store_scalar(932, if s.b[932] { 1.0 } else { 0.0 });
        if (s.b[931] && s.b[932]) {
            s.store_scaled_offset_ad(683, A::mul(s.ad_value(409), {
                if ((((1e20 * s.v[479]) / s.v[817]) / s.v[817]) > 1e-38) {
                    A::ln(A::div_scaled_value_by_product(s.ad_value(479), 1e20, s.ad_value(817), s.ad_value(817), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }), (-0.3), (-s.v[36]));
        }
        s.b[933] = (s.v[479] < 0.0);s.store_scalar(933, if s.b[933] { 1.0 } else { 0.0 });
        if ((s.b[931] && (!s.b[932])) && s.b[933]) {
            s.store_scaled_offset_ad(683, A::mul(s.ad_value(409), {
                if (((-1e20) / s.v[479]) > 1e-38) {
                    A::ln(A::div_from_scalar((-1e20), s.ad_value(479)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }), 0.3, (-s.v[36]));
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_mul_scale_offset_mixed_ia(833, 409, {
            if ((((s.v[479]) as f64).abs() / s.v[817]) > 1e-38) {
                A::ln(A::div(A::abs(s.ad_value(479)), s.ad_value(817)))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, 2.0, 0.0);s.store_mul_scaled_sqrt_ad_rhs(834, 780, 1.0 / (s.v[753]), A::abs(s.ad_value(479)));s.b[934] = (!param_given[341]);s.store_scalar(934, if s.b[934] { 1.0 } else { 0.0 });s.b[935] = (((s.v[479] > 0.0) && (s.v[36] > 0.0)) || ((s.v[479] < 0.0) && (s.v[36] < 0.0)));s.store_scalar(935, if s.b[935] { 1.0 } else { 0.0 });
        if (s.b[934] && s.b[935]) {s.store_add_scaled_inputs_product_mixed_iiia(684, 683, 1.0, 833, 1.0, 834, A::sqrt(s.ad_value(833)), 1.0);}
        if (s.b[934] && (!s.b[935])) {s.store_add_scaled_inputs_product_mixed_iiia(684, 683, 1.0, 833, (-1.0), 834, A::sqrt(s.ad_value(833)), (-1.0));}
        s.b[936] = (!param_given[342]);s.store_scalar(936, if s.b[936] { 1.0 } else { 0.0 });
        if s.b[936] {s.store_sqrt_ad(812, A::div_scaled_product(s.ad_value(778), s.ad_value(833), 2.0, A::abs(s.ad_value(479)), (1.60219e-19 * 1000000.0)));s.store_div(813, 778, 812);s.store_div_scaled_value_offset_denominator(336, s.ad_value(813), s.v[753], s.ad_value(813), s.v[753], 1.0);}
        s.store_mul_scale_offset_mixed_ia(488, 409, {
            if ((s.v[478] / s.v[817]) > 1e-38) {
                A::ln(A::div(s.ad_value(478), s.ad_value(817)))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, 2.0, 0.0);s.store_sqrt(700, 488);s.store_mul_sqrt_mixed_ia(701, 700, A::div_scaled_inputs(s.ad_value(778), 2.0, s.ad_value(478), (1.60219e-19 * 1000000.0)));s.store_sqrt(702, 701);s.b[937] = (s.v[68] == 0.0);s.store_scalar(937, if s.b[937] { 1.0 } else { 0.0 });
        if s.b[937] {s.store_sqrt_scaled_input_ad(489, A::mul(A::div_from_scalar((3.0 * 3.9), s.ad_value(777)), s.ad_value(608)), s.v[91]);}
        if (!s.b[937]) {s.store_sqrt_ad(489, A::div_scaled_product3(s.ad_value(778), s.ad_value(608), s.ad_value(776), 1.0, s.ad_value(777), 8.85418e-12));}
        s.store_mul_mixed_ia(485, 409, {
                    if (((1e20 * s.v[478]) / (s.v[817] * s.v[817])) > 1e-38) {
                        A::ln(A::div_scaled_inputs(s.ad_value(478), 1e20, A::square(s.ad_value(817)), 1.0))
                    } else {
                        A::neg(A::constant(87.49823353377374))
                    }
                });
        s.store_sqrt_ad(728, A::div_scaled_product(s.ad_value(778), s.ad_value(478), (1.60219e-19 * (1000000.0 * 0.5)), s.ad_value(488), 1.0));s.b[938] = (s.v[68] == 0.0);s.store_scalar(938, if s.b[938] { 1.0 } else { 0.0 });s.b[939] = (s.v[480] > 0.0);s.store_scalar(939, if s.b[939] { 1.0 } else { 0.0 });
        if (s.b[938] && s.b[939]) {
            s.store_mul_mixed_ia(736, 831, {
                            if ((s.v[480] / 1e20) > 1e-38) {
                                A::ln_scaled_input(s.ad_value(480), 1.0 / (1e20))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if (s.b[938] && (!s.b[939])) {s.store_scalar(736, 0.0);}
        if (!s.b[938]) {
            s.store_mul_mixed_ia(818, 831, {
                            if ((s.v[481] / s.v[817]) > 1e-38) {
                                A::ln(A::div(s.ad_value(481), s.ad_value(817)))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if (!s.b[938]) {s.store_scale(819, 816, 0.5);}
        s.b[940] = (s.v[818] > s.v[819]);s.store_scalar(940, if s.b[940] { 1.0 } else { 0.0 });
        if ((!s.b[938]) && s.b[940]) {s.copy_ad(818, 819);}
        if (!s.b[938]) {s.store_sub_scaled_inputs_mixed_ai(820, A::offset(s.ad_value(819), s.v[80]), 1.0, 818, s.v[36]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (!s.b[938]) {s.store_sub_from_scalar(736, s.v[79], 820);}
        s.store_scalar(729, (((((s.v[360] * (if ((s.v[361] / s.v[357]) > 1e-38) { (((s.v[361] / s.v[357])) as f64).ln() } else { (-87.49823353377374) }))) as f64).exp() / s.v[357]) / s.v[357]));
        s.store_div_scaled_value_by_product_mixed_aii(732, A::exp_scaled_input({
            if ((s.v[361] / (s.v[357] * s.v[580])) > 1e-38) {
                A::ln(A::div_from_scalar(s.v[361], A::scale(s.ad_value(580), s.v[357])))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, s.v[360]), (1.0 / (s.v[357]) * 1.0 / (s.v[357])), 580, 580, 1.0);
        if (s.v[36] == 1.0) {
            s.copy_ad(730, 789);
        } else {
            s.copy_ad(730, 788);
        }
        if (s.v[36] == 1.0) {
            s.copy_ad(731, 791);
        } else {
            s.copy_ad(731, 790);
        }
        s.store_mul3_affine_lhs(733, 730, 581, ((s.v[689] / s.v[59]) + s.v[61]), 0.0, 732);s.store_mul3_affine_lhs(734, 730, 581, ((s.v[689] / s.v[59]) + s.v[60]), 0.0, 732);s.store_scaled_mul(735, 731, 580, (-s.v[357]));s.store_scale(730, 730, (s.v[729] * (((s.v[689] / s.v[59]) * s.v[688]) + (s.v[64] / s.v[39]))));s.store_primal_scale(731, 731, (-s.v[357]));s.b[941] = (param_given[89] || param_given[93]);s.store_scalar(941, if s.b[941] { 1.0 } else { 0.0 });s.b[942] = (!param_given[89]);s.store_scalar(942, if s.b[942] { 1.0 } else { 0.0 });
        if (s.b[941] && s.b[942]) {s.store_scalar(490, 0.53);}
        s.b[943] = (!param_given[93]);s.store_scalar(943, if s.b[943] { 1.0 } else { 0.0 });
        if (s.b[941] && s.b[943]) {s.store_scalar(494, (-0.0186));}
        s.b[949] = (!param_given[86]);s.store_scalar(949, if s.b[949] { 1.0 } else { 0.0 });
        if (((!s.b[941]) && s.b[949]) && (s.v[68] != 0.0)) {s.store_scaled_div_from_scalar_ad(818, 1.60219e-19, A::scale(s.ad_value(778), 2.0), 1000000.0);}
        if (((!s.b[941]) && s.b[949]) && (s.v[68] == 0.0)) {s.store_scalar(818, 0.00077348);}
        if ((!s.b[941]) && s.b[949]) {s.store_add_scaled_product_indices(484, 488, 1.0, 818, 478, (-(s.v[487] * s.v[487])));}
        s.b[950] = (s.v[484] > 0.0);s.store_scalar(950, if s.b[950] { 1.0 } else { 0.0 });
        if ((!s.b[941]) && s.b[950]) {s.store_neg(484, 484);}
        s.b[951] = (s.v[486] > 0.0);s.store_scalar(951, if s.b[951] { 1.0 } else { 0.0 });
        if ((!s.b[941]) && s.b[951]) {s.store_scalar(486, (-s.v[486]));}
        s.b[952] = (!param_given[84]);s.store_scalar(952, if s.b[952] { 1.0 } else { 0.0 });
        if ((!s.b[941]) && s.b[952]) {s.store_div_scaled_product_mixed_iai(482, 780, A::sqrt(s.ad_value(478)), 1.0, 757, 1.0);}
        s.b[953] = (!param_given[85]);s.store_scalar(953, if s.b[953] { 1.0 } else { 0.0 });
        if ((!s.b[941]) && s.b[953]) {s.store_div_scaled_product_mixed_iai(483, 780, A::sqrt(s.ad_value(479)), 1.0, 757, 1.0);}
        if (!s.b[941]) {s.store_sub(818, 482, 483);s.store_sub_mixed_ai(819, A::sqrt(A::sub(s.ad_value(488), s.ad_value(484))), 700);s.store_mul_sub_mixed_iai(820, 700, A::sqrt(A::sub(s.ad_value(488), s.ad_value(486))), 700);s.store_div_scaled_product_add_scaled_denominator_indices(494, 818, 819, 1.0, 820, 2.0, 486, 1.0, 1.0);s.store_add_scaled_product_mixed_iia(490, 483, 1.0, 494, A::sqrt(A::sub(s.ad_value(488), s.ad_value(486))), (-2.0));}
        s.store_offset(818, 628, s.v[689]);s.b[954] = (s.v[818] < 1e-8);s.store_scalar(954, if s.b[954] { 1.0 } else { 0.0 });
        if s.b[954] {s.store_scalar(818, 1e-8);}
        s.store_mul_scale_offset_mixed_ia(707, 490, A::div(s.ad_value(627), s.ad_value(818)), 1.0, 1.0);s.b[955] = (!param_given[108]);s.store_scalar(955, if s.b[955] { 1.0 } else { 0.0 });s.b[956] = (param_given[107] || param_given[106]);s.store_scalar(956, if s.b[956] { 1.0 } else { 0.0 });
        if (s.b[955] && s.b[956]) {s.store_add_scaled_inputs_product_indices(522, 507, s.v[36], 488, (-1.0), 707, 700, (-1.0));}
        if (s.b[955] && (!s.b[956])) {s.store_scalar(522, (-1.0));}
        s.b[957] = (!param_given[107]);s.store_scalar(957, if s.b[957] { 1.0 } else { 0.0 });
        if s.b[957] {s.store_add_scaled_inputs_product_indices(507, 522, s.v[36], 488, s.v[36], 707, 700, s.v[36]);}
        s.store_scale(737, 707, (s.v[91] * 1.0 / (s.v[93])));s.store_mul(819, 758, 702);s.store_ad_value(818, A::exp_div_scaled_inputs(s.ad_value(506), ((-0.5) * s.v[688]), s.ad_value(819), 1.0));s.store_add_scaled_product_indices(703, 818, 1.0, 818, 818, 2.0);s.store_ad_value(818, A::exp_div_scaled_inputs(s.ad_value(505), ((-0.5) * s.v[688]), s.ad_value(819), 1.0));s.store_add_scaled_product_indices(820, 818, 1.0, 818, 818, 2.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_product_indices(704, 562, 1.0, 561, 820, 1.0);s.store_div_mixed_ia(752, 741, A::exp_scaled_input(s.ad_value(742), (if (s.v[688] > 1e-38) { ((s.v[688]) as f64).ln() } else { (-87.49823353377374) })));s.b[958] = (s.v[248] < 0.0);s.store_scalar(958, if s.b[958] { 1.0 } else { 0.0 });
        if s.b[958] {s.store_scalar(248, 0.0);}
        s.store_scalar(818, ((s.v[825]) as f64).powf(s.v[253]));s.store_primal_offset(841, 248, s.v[826]);s.store_powf(819, 841, s.v[254]);s.store_add_ad(813, A::offset(A::div_from_scalar(p.p231, s.ad_value(819)), (p.p230 / s.v[818])), A::div_from_scalar(p.p232, A::scale(s.ad_value(819), s.v[818])));s.store_offset(597, 813, 1.0);s.store_scalar(818, ((s.v[825]) as f64).powf(s.v[255]));s.store_powf(819, 841, s.v[256]);s.store_add_ad(813, A::offset(A::div_from_scalar(p.p234, s.ad_value(819)), (p.p233 / s.v[818])), A::div_from_scalar(p.p235, A::scale(s.ad_value(819), s.v[818])));s.store_offset(598, 813, 1.0);s.store_sqrt_square_offset(598, 598, 1e-9);s.store_scalar(818, (s.v[827] - 1.0));s.store_offset_scaled(599, 597, (1.0 + (s.v[252] * s.v[818])), 1e-9);s.store_scalar(835, (1.0 / (s.v[246] + (0.5 * s.v[825]))));s.store_scalar(836, (1.0 / (s.v[247] + (0.5 * s.v[825]))));s.store_scalar(601, (s.v[835] + s.v[836]));s.store_scale_ad(600, A::div_from_scalar(s.v[249], s.ad_value(599)), s.v[601]);s.b[959] = (((s.v[40] > 0.0) && (s.v[41] > 0.0)) && ((s.v[39] == 1.0) || ((s.v[39] > 1.0) && (s.v[42] > 0.0))));s.store_scalar(959, if s.b[959] { 1.0 } else { 0.0 });
        if s.b[959] {s.store_scalar(837, 0.0);s.store_scalar(838, 0.0);}
        s.b[960] = (s.v[250] < (-1.0));s.store_scalar(960, if s.b[960] { 1.0 } else { 0.0 });
        if (s.b[959] && s.b[960]) {s.store_scalar(250, (-1.0));}
        s.b[961] = (s.v[250] > 1.0);s.store_scalar(961, if s.b[961] { 1.0 } else { 0.0 });
        if ((s.b[959] && (!s.b[960])) && s.b[961]) {s.store_scalar(250, 1.0);}
        if ((s.b[959] && (!s.b[960])) && (!s.b[961])) {
        }
        if s.b[959] {s.store_scalar(847, 0.0);}
        let mut t28: usize = 0;
        while {
            let t27: f64 = if (s.b[959] && (s.v[847] < s.v[39])) { 1.0 } else { 0.0 };
            t27 != 0.0
        } {
            t28 += 1;assert!(t28 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[959] {s.store_primal_div_from_scalar_offset_scaled_input(962, (1.0 / s.v[39]), 847, (s.v[42] + s.v[825]), (s.v[40] + (0.5 * s.v[825])));s.store_primal_div_from_scalar_offset_scaled_input(963, (1.0 / s.v[39]), 847, (s.v[42] + s.v[825]), (s.v[41] + (0.5 * s.v[825])));s.store_primal_add(837, 837, 962);s.store_primal_add(838, 838, 963);s.store_primal_offset(847, 847, 1.0);}
        }
        if s.b[959] {s.store_primal_add(842, 837, 838);s.copy_ad(414, 842);s.store_mul_div_from_scalar_lhs_ad_indices(839, s.v[249], 599, 842);s.store_div_scaled_offset_numerator_mixed_ia(818, 839, 1.0, 1.0, A::offset(s.ad_value(600), 1.0), 1.0);s.store_mul(765, 698, 818);s.store_div_scaled_offset_numerator(819, A::mul(s.ad_value(250), s.ad_value(839)), 1.0, 1.0, A::offset(A::mul(s.ad_value(250), s.ad_value(600)), 1.0), 1.0);s.store_mul(767, 699, 819);s.store_primal_offset(843, 842, (-s.v[601]));s.store_mul_div_from_scalar_lhs_ad_indices(840, s.v[251], 598, 843);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(844, s.v[257], A::powf(s.ad_value(598), s.v[258]), 843);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(845, s.v[259], A::powf(s.ad_value(598), s.v[260]), 843);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(846, s.v[261], A::powf(s.ad_value(598), s.v[262]), 843);s.store_add(768, 507, 840);s.store_add(763, 494, 844);s.store_add(761, 556, 845);s.store_add(762, 558, 846);}
        if (!s.b[959]) {s.copy_ad(765, 698);s.copy_ad(768, 507);s.copy_ad(767, 699);s.copy_ad(763, 494);s.copy_ad(761, 556);s.copy_ad(762, 558);s.store_scalar(414, 0.0);s.store_scalar(601, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
    ) {
        if (!s.b[959]) {s.store_scalar(250, 0.0);}
        s.store_scale(764, 763, (s.v[91] * 1.0 / (s.v[93])));s.store_offset(768, 768, s.v[56]);s.store_offset(766, 522, (s.v[36] * s.v[56]));s.store_scalar(430, (s.v[753] * s.v[44]));s.store_scale(432, 336, s.v[44]);s.store_scalar(431, (s.v[753] * s.v[43]));s.store_scale(433, 336, s.v[43]);s.b[964] = (s.v[336] > 0.0);s.store_scalar(964, if s.b[964] { 1.0 } else { 0.0 });s.b[965] = (((s.v[479] > 0.0) && (s.v[36] > 0.0)) || ((s.v[479] < 0.0) && (s.v[36] < 0.0)));s.store_scalar(965, if s.b[965] { 1.0 } else { 0.0 });
        if (s.b[964] && s.b[965]) {s.store_sub(818, 684, 683);}
        let (t2b,) = {
    if (s.b[964] && s.b[965]) {
        let t29: f64 = (s.v[337] * s.v[818]);let t2a: f64 = (s.v[683] + t29);
        (t2a,)
    } else {
        (s.v[545],)
    }
};
        s.store_scalar(545, t2b);
        if (s.b[964] && s.b[965]) {s.store_sub_from_scalar(819, s.v[430], 432);s.store_div_scaled_value_by_product_indices(820, 819, 1.0, 818, 818, 1.0);s.store_scale(546, 820, 1.0 / (s.v[337]));s.store_scale(547, 820, 1.0 / ((1.0 - s.v[337])));s.store_add_scaled_products_indices(434, 818, 819, ((1.0 + s.v[337]) * 0.3333333333333333), 432, 683, (-1.0));s.store_sub_from_scalar(819, s.v[431], 433);s.store_div_scaled_value_by_product_indices(820, 819, 1.0, 818, 818, 1.0);s.store_scale(548, 820, 1.0 / (s.v[337]));s.store_scale(549, 820, 1.0 / ((1.0 - s.v[337])));s.store_add_scaled_products_indices(435, 818, 819, ((1.0 + s.v[337]) * 0.3333333333333333), 433, 683, (-1.0));}
        if (s.b[964] && (!s.b[965])) {s.store_sub(818, 683, 684);}
        let (t2,) = {
    if (s.b[964] && (!s.b[965])) {
        let t0: f64 = (s.v[337] * s.v[818]);let t1: f64 = (s.v[684] + t0);
        (t1,)
    } else {
        (s.v[545],)
    }
};
        s.store_scalar(545, t2);
        if (s.b[964] && (!s.b[965])) {s.store_offset(819, 432, (-s.v[430]));s.store_div_scaled_value_by_product_indices(820, 819, 1.0, 818, 818, 1.0);s.store_scale(546, 820, 1.0 / (s.v[337]));s.store_scale(547, 820, 1.0 / ((1.0 - s.v[337])));s.store_add_scaled_product_indices(434, 684, (-s.v[430]), 818, 819, ((1.0 + s.v[337]) * 0.3333333333333333));s.store_offset(819, 433, (-s.v[431]));s.store_div_scaled_value_by_product_indices(820, 819, 1.0, 818, 818, 1.0);s.store_scale(548, 820, 1.0 / (s.v[337]));s.store_scale(549, 820, 1.0 / ((1.0 - s.v[337])));s.store_add_scaled_product_indices(435, 684, (-s.v[431]), 818, 819, ((1.0 + s.v[337]) * 0.3333333333333333));}
        let (t3,) = {
    if (!s.b[964]) {
        (0.0,)
    } else {
        (s.v[545],)
    }
};
        s.store_scalar(545, t3);
        if (!s.b[964]) {s.store_scalar(546, 0.0);s.store_scalar(547, 0.0);s.store_scalar(434, 0.0);s.store_scalar(548, 0.0);s.store_scalar(549, 0.0);s.store_scalar(435, 0.0);}
        s.b[966] = ((s.v[354] < 1.0) || (s.v[354] > 2.0));s.store_scalar(966, if s.b[966] { 1.0 } else { 0.0 });
        if s.b[966] {s.store_scalar(354, 1.0);}
        s.store_scale_ad(818, {
            if ((s.v[354] * (1.0 + (s.v[174] / s.v[173]))) > 1e-38) {
                A::ln_scaled_input(s.ad_value(354), (1.0 + (s.v[174] / s.v[173])))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, s.v[338]);s.store_scalar(819, (s.v[46] - s.v[38]));s.b[967] = (s.v[819] > 0.0);s.store_scalar(967, if s.b[967] { 1.0 } else { 0.0 });
        if s.b[967] {s.store_scale(428, 818, s.v[819]);}
        if (!s.b[967]) {s.store_scalar(428, 0.0);}
        s.store_scalar(819, (s.v[45] - s.v[38]));s.b[968] = (s.v[819] > 0.0);s.store_scalar(968, if s.b[968] { 1.0 } else { 0.0 });
        if s.b[968] {s.store_scale(429, 818, s.v[819]);}
        if (!s.b[968]) {s.store_scalar(429, 0.0);}
        s.store_scalar(423, (s.v[155] * s.v[47]));s.b[969] = (s.v[423] <= 0.001);s.store_scalar(969, if s.b[969] { 1.0 } else { 0.0 });
        if s.b[969] {s.store_scalar(423, 0.001);}
        s.store_scalar(422, (s.v[155] * s.v[48]));s.b[970] = (s.v[422] <= 0.001);s.store_scalar(970, if s.b[970] { 1.0 } else { 0.0 });
        if s.b[970] {s.store_scalar(422, 0.001);}
        s.b[971] = (s.v[317] < 1e-15);s.store_scalar(971, if s.b[971] { 1.0 } else { 0.0 });
        if s.b[971] {s.store_scalar(317, 1e-15);}
        s.store_div_scalar_by_product_indices(818, (((-0.5) * s.v[688]) * s.v[688]), 317, 317, 1.0);s.b[972] = (s.v[818] > 100.0);s.store_scalar(972, if s.b[972] { 1.0 } else { 0.0 });
        if s.b[972] {s.store_scaled_offset(819, 818, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[973] = (s.v[818] < (-100.0));s.store_scalar(973, if s.b[973] { 1.0 } else { 0.0 });
        if ((!s.b[972]) && s.b[973]) {s.store_scalar(819, 3.720075976e-44);}
        if ((!s.b[972]) && (!s.b[973])) {s.store_exp(819, 818);}
        s.copy_ad(712, 819);s.store_mul_scale_offset_mixed_ia(818, 680, A::div_from_scalar(1.0, s.ad_value(317)), 1.0, (1.0 / s.v[688]));s.store_pow_indices(713, 818, 679);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
    ) {
        s.store_offset_scaled_ad(714, A::pow(s.ad_value(818), s.ad_value(616)), s.v[324], 1.0);s.store_add_scaled_inputs(715, 681, 1.0, 682, s.v[688]);s.b[974] = (s.v[715] < 1.0);s.store_scalar(974, if s.b[974] { 1.0 } else { 0.0 });
        if s.b[974] {s.store_scalar(715, 1.0);}
        s.b[975] = (s.v[68] == 0.0);s.store_scalar(975, if s.b[975] { 1.0 } else { 0.0 });
        if s.b[975] {s.store_scalar(92, (s.v[91] - s.v[94]));}
        if (!s.b[975]) {s.store_scalar(850, (8.617087e-5 * s.v[84]));s.copy_ad(851, 850);}
        if (!s.b[975]) {
            s.store_mul_mixed_ia(852, 850, {
                            if (((1e20 * s.v[478]) / (s.v[817] * s.v[817])) > 1e-38) {
                                A::ln(A::div_scaled_inputs(s.ad_value(478), 1e20, A::square(s.ad_value(817)), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if (!s.b[975]) {
            s.store_mul_scale_offset_mixed_ia(853, 850, {
                if ((s.v[478] / s.v[817]) > 1e-38) {
                    A::ln(A::div(s.ad_value(478), s.ad_value(817)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 2.0, 0.0);
        }
        if (!s.b[975]) {s.store_sqrt(854, 853);s.store_add(814, 766, 853);s.store_scalar(855, (s.v[36] * s.v[83]));s.store_scalar(818, (s.v[87] * 8.85418e-12));}
        s.b[976] = ((((s.v[480] > 1e18) && (s.v[480] < 1e25)) && (s.v[855] > s.v[814])) && (s.v[818] != 0.0));s.store_scalar(976, if s.b[976] { 1.0 } else { 0.0 });
        if ((!s.b[975]) && s.b[976]) {s.store_div_scaled_product_mixed_iia(819, 778, 480, (1000000.0 * 1.60219e-19), A::square(s.ad_value(757)), 1.0);s.store_sqrt_offset_ad(822, A::div_scaled_inputs2(s.ad_value(855), 2.0, s.ad_value(818), (-2.0), s.ad_value(819), 1.0), 1.0);s.store_mul_scale_offset_indices(820, 819, 822, 1.0, (-1.0));s.store_div_scaled_product_indices(821, 820, 820, 0.5, 819, 1.0);s.store_offset_sub(884, 782, 821, (-0.05));s.store_sqrt_square_offset(824, 884, 0.224);s.store_add_scaled_inputs3_indices(823, 782, 1.0, 884, (-0.5), 824, (-0.5));s.store_sub(856, 855, 823);}
        if ((!s.b[975]) && (!s.b[976])) {s.copy_ad(856, 855);}
        if (!s.b[975]) {s.store_sub(858, 852, 853);s.copy_ad(821, 702);s.store_mul(861, 758, 821);s.store_mul(862, 758, 821);s.store_div_scaled_inputs_indices(818, 500, ((-0.5) * s.v[81]), 861, 1.0);}
        s.b[977] = (s.v[818] > (-100.0));s.store_scalar(977, if s.b[977] { 1.0 } else { 0.0 });
        if ((!s.b[975]) && s.b[977]) {s.store_exp(819, 818);s.store_mul_scale_offset_rhs(875, 819, 819, 2.0, 1.0);}
        if ((!s.b[975]) && (!s.b[977])) {s.store_scalar(819, 3.720075976e-44);s.store_mul_scale_offset_rhs(875, 819, 819, 2.0, 1.0);}
        if (!s.b[975]) {s.store_div_scaled_product_indices(820, 470, 778, 1.0, 701, 1.0);s.copy_ad(821, 466);s.store_div_scaled_inputs2_mixed_aii(822, A::add_scaled_product(s.ad_value(820), 1.0, s.ad_value(821), s.ad_value(875), 1.0), 1.0, 469, 1.0, 757, 1.0);}
        s.b[978] = (s.v[822] >= (-0.5));s.store_scalar(978, if s.b[978] { 1.0 } else { 0.0 });
        if ((!s.b[975]) && s.b[978]) {s.store_offset(864, 822, 1.0);}
        if ((!s.b[975]) && (!s.b[978])) {s.store_div_from_scalar_offset_scaled_input(818, 1.0, 822, 8.0, 3.0);s.store_mul_scale_offset_rhs(864, 818, 822, 3.0, 1.0);}
        s.b[979] = (s.v[739] > 0.0);s.store_scalar(979, if s.b[979] { 1.0 } else { 0.0 });
        if ((!s.b[975]) && s.b[979]) {s.store_offset_scaled(821, 739, 2.0, s.v[81]);}
        if ((!s.b[975]) && s.b[979]) {
            s.store_mul_mixed_ia(822, 851, {
                            if ((s.v[81] / s.v[821]) > 1e-38) {
                                A::ln(A::div_from_scalar(s.v[81], s.ad_value(821)))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if ((!s.b[975]) && s.b[979]) {s.store_mul(872, 864, 822);}
        if ((!s.b[975]) && (!s.b[979])) {s.store_scalar(872, 0.0);}
        if (!s.b[975]) {s.store_mul(411, 499, 875);s.store_mul(876, 411, 858);s.store_div_scaled_inputs_indices(818, 503, ((-0.5) * (s.v[82] * s.v[81])), 862, 1.0);}
        s.b[980] = (s.v[818] > (-100.0));s.store_scalar(980, if s.b[980] { 1.0 } else { 0.0 });
        if ((!s.b[975]) && s.b[980]) {s.store_exp(819, 818);s.store_mul_scale_offset_rhs(820, 819, 819, 2.0, 1.0);}
        if ((!s.b[975]) && (!s.b[980])) {s.store_scalar(819, 3.720075976e-44);s.store_mul_scale_offset_rhs(820, 819, 819, 2.0, 1.0);}
        if (!s.b[975]) {s.store_mul(818, 502, 820);s.store_mul(877, 818, 858);s.store_scalar(863, ((s.v[84] / s.v[150]) - 1.0));s.store_sqrt_offset_scaled_input(818, 498, 1.0 / (s.v[81]), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
    ) {
        if (!s.b[975]) {s.store_add_scaled_inputs(819, 491, 1.0, 492, 1.0 / (s.v[81]));s.store_add_scaled_product_mixed_aii(873, A::mul3(s.ad_value(737), A::offset(s.ad_value(818), (-1.0)), s.ad_value(854)), 1.0, 819, 863, 1.0);s.store_div_scaled_product_offset_denominator_indices(814, 776, 853, 1.0, 497, s.v[82], 1.0);s.store_scalar(870, 0.0);s.store_scalar(874, 0.0);s.store_sqrt_offset_scaled_input(871, 738, 1.0 / (s.v[81]), 1.0);s.copy_ad(867, 854);s.store_sub_add_scaled_inputs4_lhs_mixed_aiii(859, A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(768), s.v[36], A::add_scaled_products(s.ad_value(737), s.ad_value(867), 1.0, s.ad_value(707), s.ad_value(854), (-1.0)), s.ad_value(871), 1.0), 1.0, s.ad_value(876), (-1.0), s.ad_value(877), -1.0), 1.0, s.ad_value(495), s.ad_value(814), 1.0), 1.0, 873, 1.0, 870, -1.0, 872, -1.0, 874);s.store_sub(860, 856, 859);s.store_mul(849, 864, 851);s.store_div_scaled_product_indices(865, 745, 860, 1.0, 849, 1.0);s.store_div_scaled_inputs2_mixed_iai(866, 521, 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(745), s.ad_value(860)), (-1.0), 849, 1.0);}
        s.b[981] = (s.v[865] > 100.0);s.store_scalar(981, if s.b[981] { 1.0 } else { 0.0 });
        if ((!s.b[975]) && s.b[981]) {s.copy_ad(857, 860);}
        s.b[982] = (s.v[866] > 100.0);s.store_scalar(982, if s.b[982] { 1.0 } else { 0.0 });
        if (((!s.b[975]) && (!s.b[981])) && s.b[982]) {s.store_div_scaled_inputs2_by_product_indices(818, 860, 1.0, 521, (-1.0), 864, 851, 1.0);s.store_exp(868, 818);s.store_mul_div_scaled_product_indices(857, 868, 851, 728, 1.0, 757, 1.0);}
        if (((!s.b[975]) && (!s.b[981])) && (!s.b[982])) {s.store_exp(868, 865);}
        if (((!s.b[975]) && (!s.b[981])) && (!s.b[982])) {
            s.store_mul_mixed_ia(819, 849, {
                            if ((1.0 + s.v[868]) > 1e-38) {
                                A::ln(A::offset(s.ad_value(868), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if (((!s.b[975]) && (!s.b[981])) && (!s.b[982])) {s.store_mul3_ad(822, A::div_scaled_inputs(s.ad_value(757), -1.0, A::mul(s.ad_value(850), s.ad_value(728)), 1.0), A::exp(s.ad_value(866)), A::sub_from_scalar(1.0, s.ad_value(745)));s.store_sub_mixed_ia(820, 745, A::div_scaled_product(s.ad_value(849), s.ad_value(822), 1.0, A::sub_from_scalar(1.0, s.ad_value(745)), 1.0));s.store_div(857, 819, 820);}
        if (!s.b[975]) {s.store_add_scaled_inputs3_indices(821, 768, s.v[36], 766, (-1.0), 853, -1.0);s.store_scale(869, 821, 4.0);}
        s.b[983] = (s.v[869] < 0.0);s.store_scalar(983, if s.b[983] { 1.0 } else { 0.0 });
        if ((!s.b[975]) && s.b[983]) {s.store_scalar(869, 0.0);}
        let (t4,) = {
    if (!s.b[975]) {
        (0.0,)
    } else {
        (s.v[878],)
    }
};
        s.store_scalar(878, t4);
        if (!s.b[975]) {s.copy_ad(879, 776);}
        let (t5,) = {
    if (!s.b[975]) {
        (1000000.0,)
    } else {
        (s.v[880],)
    }
};
        s.store_scalar(880, t5);
    }
}
