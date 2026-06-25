#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[7] = (273.15 + p.p15);

        s.v[0] = ((ctx.temperature() + p.p36)).min(1000.0);

        s.v[529] = if (p.p10 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[529] != 0.0) {
            s.store_scalar(8, (0.5 * ((s.v[0] + (p.p17 + (p.p18 * s.v[0]))) + (((((s.v[0] - (p.p17 + (p.p18 * s.v[0]))) * (s.v[0] - (p.p17 + (p.p18 * s.v[0])))) + p.p19)) as f64).sqrt())));
        }

        if (s.v[529] != 0.0) {
            s.store_scale_ad(225, A::add(A::offset(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), 600.0), A::sqrt(A::offset(A::mul(A::offset(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), (-600.0)), A::offset(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), (-600.0))), 0.01))), 0.5);
        }

        if (!(s.v[529] != 0.0)) {
            s.store_scalar(8, (0.5 * ((s.v[0] + 1.0) + (((((s.v[0] - 1.0) * (s.v[0] - 1.0)) + 0.001)) as f64).sqrt())));
        }

        if (!(s.v[529] != 0.0)) {
            s.store_scalar(225, 600.0);
        }

        s.v[530] = if (((p.p0 == 0.0) && (p.p172 > 0.0)) || ((p.p0 > 0.0) && (p.p443 > 0.0))) { 1.0 } else { 0.0 };

        if (s.v[530] != 0.0) {
            s.store_scalar(6, p.p5);
        }

        if (!(s.v[530] != 0.0)) {
            s.store_scalar(6, 0.0);
        }

        s.v[475] = 0.0;

        s.v[219] = 0.0;

        s.copy_ad(217, 8);

        s.store_square(218, 217);

        s.store_offset(220, 217, (-s.v[7]));

        s.store_scale(221, 217, 1.0 / (s.v[7]));

        s.store_div_from_scalar(222, s.v[7], 217);

        s.store_scale(223, 217, 8.617332384961e-5);

        s.store_div_from_scalar(224, 1.0, 223);

        s.v[611] = if (p.p0 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[611] != 0.0) {
            s.store_scalar(10, p.p23);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(9, p.p22);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(12, p.p25);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(11, p.p24);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(13, p.p30);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(533, p.p41);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(14, p.p42);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(15, p.p43);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(534, p.p44);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(535, 1.0);
        }

        s.v[612] = if (p.p45 < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[611] != 0.0) && (s.v[612] != 0.0)) {
            s.store_scalar(535, (-1.0));
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(536, ((((p.p45) as f64).abs()).min(1e19) * 1000000.0));
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(16, 1.0);
        }

        s.v[613] = if (p.p46 < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[611] != 0.0) && (s.v[613] != 0.0)) {
            s.store_scalar(16, (-1.0));
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(537, (((((p.p46) as f64).abs()).max(1e16)).min(1e21) * 1000000.0));
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(17, p.p47);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(18, p.p48);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(19, (p.p49 * 1000000.0));
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(20, (p.p50 * 1000000.0));
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(183, p.p51);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(184, p.p52);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(23, p.p53);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(24, (p.p54 * 1000000.0));
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(25, p.p55);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(26, p.p56);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(27, p.p57);
        }

        if (s.v[611] != 0.0) {
            s.store_div_ad_lhs(28, A::mul(A::scale(s.ad_value(27), p.p58), s.ad_value(534)), 533);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(29, (p.p59 * 1000000.0));
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(30, p.p60);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(538, p.p61);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(187, p.p62);
        }

        if (s.v[611] != 0.0) {
            s.store_div_ad_lhs(188, A::mul(A::scale(s.ad_value(187), p.p63), s.ad_value(534)), 533);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(34, p.p64);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(35, p.p65);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(36, p.p66);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(37, p.p67);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(191, p.p68);
        }

        if (s.v[611] != 0.0) {
            s.store_scale(192, 191, p.p69);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(40, p.p70);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(195, p.p71);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(41, p.p72);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(42, p.p73);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(43, p.p74);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(196, p.p75);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(45, p.p76);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(539, p.p77);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(540, p.p78);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(193, p.p79);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(48, p.p80);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(194, p.p81);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(49, p.p82);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(197, p.p83);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(51, p.p84);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(52, p.p85);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(541, p.p86);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(198, p.p87);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(54, p.p88);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(55, p.p89);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(56, p.p90);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(57, p.p91);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(58, p.p92);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(199, p.p93);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(60, p.p94);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(61, p.p95);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(62, p.p96);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(542, p.p97);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(63, p.p98);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(64, p.p99);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(65, p.p100);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(66, p.p101);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(67, p.p102);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(75, p.p103);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(201, p.p104);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(202, p.p105);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(203, p.p106);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(206, p.p120);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(207, p.p121);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(204, p.p107);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(205, p.p108);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(76, p.p109);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(77, p.p123);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(78, p.p110);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(79, p.p111);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(80, p.p112);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(81, p.p122);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(82, p.p113);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(83, p.p114);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(84, p.p115);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(85, p.p116);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(86, p.p117);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(87, p.p118);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(88, p.p119);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(89, p.p124);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(90, p.p125);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(208, p.p126);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(209, p.p127);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(93, p.p128);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(94, p.p129);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(95, p.p130);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(96, p.p131);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(97, p.p132);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(98, p.p133);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(112, p.p147);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(210, p.p148);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(114, p.p149);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(115, p.p150);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(99, p.p134);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(211, p.p135);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(212, p.p136);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(102, p.p137);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(103, p.p138);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(104, p.p139);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(105, p.p140);
        }

        if (s.v[611] != 0.0) {
            s.store_div_ad_lhs(106, A::mul(A::scale(s.ad_value(105), p.p141), s.ad_value(534)), 533);
        }

    }

    pub(super) fn stamp_transient_block_1(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[611] != 0.0) {
            s.store_scalar(107, p.p142);
        }

        if (s.v[611] != 0.0) {
            s.store_div_ad_lhs(108, A::mul(A::scale(s.ad_value(107), p.p143), s.ad_value(534)), 533);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(109, p.p144);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(213, p.p145);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(111, p.p146);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(116, p.p151);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(117, p.p152);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(118, (p.p153 * 1000000.0));
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(119, p.p154);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(120, p.p155);
        }

        if (s.v[611] != 0.0) {
            s.copy_ad(185, 183);
        }

        if (s.v[611] != 0.0) {
            s.copy_ad(186, 184);
        }

        if (s.v[611] != 0.0) {
            s.copy_ad(135, 27);
        }

        if (s.v[611] != 0.0) {
            s.copy_ad(136, 28);
        }

        if (s.v[611] != 0.0) {
            s.copy_ad(189, 187);
        }

        if (s.v[611] != 0.0) {
            s.copy_ad(190, 188);
        }

        if (s.v[611] != 0.0) {
            s.copy_ad(200, 199);
        }

        if (s.v[611] != 0.0) {
            s.copy_ad(543, 542);
        }

        if (s.v[611] != 0.0) {
            s.copy_ad(158, 63);
        }

        s.v[614] = if (p.p11 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[611] != 0.0) && (s.v[614] != 0.0)) {
            s.store_scalar(185, p.p51);
        }

        s.v[615] = if (if self.param_given[156] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[611] != 0.0) && (s.v[614] != 0.0)) && (s.v[615] != 0.0)) {
            s.store_scalar(185, p.p156);
        }

        if ((s.v[611] != 0.0) && (s.v[614] != 0.0)) {
            s.store_scalar(186, p.p52);
        }

        s.v[616] = if (if self.param_given[157] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[611] != 0.0) && (s.v[614] != 0.0)) && (s.v[616] != 0.0)) {
            s.store_scalar(186, p.p157);
        }

        if ((s.v[611] != 0.0) && (s.v[614] != 0.0)) {
            s.store_scalar(135, p.p57);
        }

        s.v[617] = if (if self.param_given[158] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[611] != 0.0) && (s.v[614] != 0.0)) && (s.v[617] != 0.0)) {
            s.store_scalar(135, p.p158);
        }

        if ((s.v[611] != 0.0) && (s.v[614] != 0.0)) {
            s.store_div_ad_lhs(136, A::mul(A::scale(s.ad_value(135), p.p58), s.ad_value(534)), 533);
        }

        if ((s.v[611] != 0.0) && (s.v[614] != 0.0)) {
            s.store_scalar(189, p.p62);
        }

        s.v[618] = if (if self.param_given[159] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[611] != 0.0) && (s.v[614] != 0.0)) && (s.v[618] != 0.0)) {
            s.store_scalar(189, p.p159);
        }

        if ((s.v[611] != 0.0) && (s.v[614] != 0.0)) {
            s.store_div_ad_lhs(190, A::mul(A::scale(s.ad_value(189), p.p63), s.ad_value(534)), 533);
        }

        if ((s.v[611] != 0.0) && (s.v[614] != 0.0)) {
            s.store_scalar(200, p.p93);
        }

        s.v[619] = if (if self.param_given[160] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[611] != 0.0) && (s.v[614] != 0.0)) && (s.v[619] != 0.0)) {
            s.store_scalar(200, p.p160);
        }

        if ((s.v[611] != 0.0) && (s.v[614] != 0.0)) {
            s.store_scalar(543, p.p97);
        }

        s.v[620] = if (if self.param_given[161] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[611] != 0.0) && (s.v[614] != 0.0)) && (s.v[620] != 0.0)) {
            s.store_scalar(543, p.p161);
        }

        if ((s.v[611] != 0.0) && (s.v[614] != 0.0)) {
            s.store_scalar(158, p.p98);
        }

        s.v[621] = if (if self.param_given[162] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[611] != 0.0) && (s.v[614] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_scalar(158, p.p162);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(159, p.p163);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(160, p.p164);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(161, p.p165);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(162, p.p166);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(163, p.p167);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(164, p.p168);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(165, p.p169);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(166, p.p170);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(167, p.p171);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(214, p.p172);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(169, p.p173);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(170, p.p174);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(171, p.p175);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(172, p.p176);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(173, p.p177);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(174, p.p178);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(175, p.p179);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(176, p.p180);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(177, p.p181);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(178, p.p182);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(179, p.p183);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(180, p.p184);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(181, p.p185);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(182, p.p186);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(311, p.p187);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(318, p.p188);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(322, p.p189);
        }

        if (s.v[611] != 0.0) {
            s.store_scalar(326, p.p190);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(588, (1.0 / p.p29));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_max_with_scalar_ad(532, A::scale(s.ad_value(588), p.p21), 1e-9);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scale(10, 588, p.p23);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scale(9, 588, p.p22);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scale(12, 588, p.p25);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scale(11, 588, p.p24);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(13, (p.p30 * p.p29));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(569, 1e-6);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(570, 1e-6);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scale(571, 569, 1.0 / (p.p20));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div(572, 570, 532);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_mul_ad(573, A::scale(A::offset(A::scale(s.ad_value(571), p.p192), 1.0), p.p191), A::offset(A::scale(s.ad_value(572), p.p193), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_mul_ad(574, A::scale(A::offset(A::scale(s.ad_value(572), p.p197), 1.0), p.p195), A::offset(A::scale(s.ad_value(571), p.p196), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_max_with_scalar_ad(575, A::offset(A::offset(s.ad_value(573), p.p20), (-(2.0 * p.p194))), 1e-9);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_max_with_scalar_ad(576, A::offset(A::add(s.ad_value(532), s.ad_value(574)), (-(2.0 * p.p198))), 1e-9);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_max_with_scalar_ad(577, A::offset(A::offset(A::offset(s.ad_value(573), p.p20), (-(2.0 * p.p194))), p.p199), 1e-9);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_max_with_scalar_ad(578, A::offset(A::offset(A::add(s.ad_value(532), s.ad_value(574)), (-(2.0 * p.p198))), p.p200), 1e-9);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div(579, 569, 575);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div(580, 570, 576);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_mul(581, 579, 580);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_max_with_scalar_ad(0, A::offset(s.ad_value(573), p.p20), 1e-9);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div(582, 0, 569);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_max_with_scalar_ad(0, A::add(s.ad_value(532), s.ad_value(574)), 1e-9);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div(583, 0, 570);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_max_with_scalar_ad(312, A::offset(s.ad_value(573), p.p20), 1e-9);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_max_with_scalar_ad(313, A::offset(s.ad_value(312), p.p499), 1e-9);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_max_with_scalar_ad(314, A::add(s.ad_value(532), s.ad_value(574)), 1e-9);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_max_with_scalar_ad(315, A::sub_from_scalar(p.p38, A::scale(s.ad_value(574), 0.5)), 1e-9);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(533, p.p201);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(14, p.p202);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(15, p.p203);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(534, p.p204);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(535, 1.0);
        }

        s.v[622] = if (p.p205 < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[611] != 0.0)) && (s.v[622] != 0.0)) {
            s.store_scalar(535, (-1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(536, ((((p.p205) as f64).abs()).min(1e19) * 1000000.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(16, 1.0);
        }

        s.v[623] = if (p.p206 < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[611] != 0.0)) && (s.v[623] != 0.0)) {
            s.store_scalar(16, (-1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(537, (((((p.p206) as f64).abs()).max(1e16)).min(1e21) * 1000000.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(17, p.p207);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(18, p.p208);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(19, (p.p209 * 1000000.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(20, (p.p210 * 1000000.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_ad(0, A::scale(A::powf(s.ad_value(579), p.p213), p.p212), A::offset(A::scale(A::powf(s.ad_value(579), p.p215), p.p214), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_add_ad(183, A::add(A::offset(s.ad_value(0), p.p211), A::scale(s.ad_value(580), p.p216)), A::scale(s.ad_value(581), p.p217));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_offset_ad(184, A::mul(A::div(A::scale(s.ad_value(534), p.p219), s.ad_value(533)), s.ad_value(0)), p.p218);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_mul_ad(23, A::mul(A::scale(A::offset(A::scale(s.ad_value(579), p.p221), 1.0), p.p220), A::offset(A::scale(s.ad_value(580), p.p222), 1.0)), A::offset(A::scale(s.ad_value(581), p.p223), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scale_ad(607, A::offset(A::scale(s.ad_value(579), p.p225), 1.0), (p.p224 * 1000000.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_min_with_scalar_ad(24, A::max_with_scalar(s.ad_value(607), 1e25), 1e28);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(25, p.p226);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(26, p.p227);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_sub_from_scalar(228, 1.0, 15);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_add_ad(229, A::scale(s.ad_value(228), 1.04479e-10), A::scale(s.ad_value(15), 1.43438e-10));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_ad_lhs(584, A::sqrt(A::mul(A::mul(A::scale(s.ad_value(229), 28959234086.17689), s.ad_value(14)), A::offset(s.ad_value(533), 4e-10))), 575);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_mul_ad(544, A::scale(A::powf(s.ad_value(584), p.p229), (p.p228 * 2.0)), A::offset(A::scale(s.ad_value(580), p.p230), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_min_with_scalar_ad(27, A::max_with_scalar(s.ad_value(544), 0.0), 5.0);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_ad_lhs(28, A::mul(A::scale(s.ad_value(27), p.p231), s.ad_value(534)), 533);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(29, (p.p232 * 1000000.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(30, p.p233);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scale(549, 580, p.p234);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_min_with_scalar_ad(538, A::max_with_scalar(s.ad_value(549), (-1.0)), 1.0);
        }

    }

    pub(super) fn stamp_transient_block_2(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (!(s.v[611] != 0.0)) {
            s.store_mul_ad(0, A::powf(s.ad_value(584), p.p236), A::offset(A::scale(s.ad_value(580), p.p237), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scale(546, 0, p.p235);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_ad(187, &A::max_with_scalar(s.ad_value(546), 0.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_ad_lhs(188, A::mul(A::scale(s.ad_value(187), p.p238), s.ad_value(534)), 533);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scale(34, 0, p.p239);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(35, p.p240);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_ad(36, A::scale(s.ad_value(579), p.p241), A::max_with_scalar(A::offset(A::scale(s.ad_value(580), p.p242), 1.0), 0.001));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(37, p.p243);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_ad(2, A::neg(s.ad_value(575)), A::scale(A::max_with_scalar(A::offset(A::scale(s.ad_value(580), p.p248), 1.0), 0.001), p.p247));
        }

        s.v[624] = if (s.v[2] > (-80.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[611] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_exp(3, 2);
        }

        if ((!(s.v[611] != 0.0)) && (!(s.v[624] != 0.0))) {
            s.store_div_from_scalar_ad(3, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(2)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(2)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scale_ad(4, A::neg(s.ad_value(575)), 1.0 / (p.p250));
        }

        s.v[625] = if (s.v[4] > (-80.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[611] != 0.0)) && (s.v[625] != 0.0)) {
            s.store_exp(5, 4);
        }

        if ((!(s.v[611] != 0.0)) && (!(s.v[625] != 0.0))) {
            s.store_div_from_scalar_ad(5, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(4)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(4)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_max_with_scalar_ad(585, A::add(A::offset(A::div(A::mul(A::scale(A::offset(A::scale(s.ad_value(580), p.p246), 1.0), p.p245), A::offset(s.ad_value(3), (-1.0))), s.ad_value(2)), 1.0), A::div(A::scale(A::offset(s.ad_value(5), (-1.0)), p.p249), s.ad_value(4))), 1e-6);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_max_with_scalar_ad(586, A::add(A::offset(A::scale(s.ad_value(580), p.p251), 1.0), A::mul(A::scale(s.ad_value(580), p.p252), A::ln(A::offset(A::scale(s.ad_value(576), 1.0 / (p.p253)), 1.0)))), 1e-6);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_mul_ad_lhs(587, A::div_from_scalar(p.p244, s.ad_value(585)), 586);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_ad_lhs(548, A::mul(s.ad_value(587), s.ad_value(576)), 575);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_ad(191, &A::max_with_scalar(s.ad_value(548), 1e-10));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scale(192, 191, p.p254);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_mul_ad(40, A::mul(A::scale(A::offset(A::scale(s.ad_value(579), p.p256), 1.0), p.p255), A::offset(A::scale(s.ad_value(580), p.p257), 1.0)), A::offset(A::scale(s.ad_value(581), p.p258), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_mul_ad(550, A::mul(A::offset(A::scale(A::powf(s.ad_value(579), p.p261), p.p260), p.p259), A::offset(A::scale(s.ad_value(580), p.p262), 1.0)), A::offset(A::scale(s.ad_value(581), p.p263), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_ad(195, &A::max_with_scalar(s.ad_value(550), 0.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(41, p.p264);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(42, p.p265);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_mul_ad(43, A::mul(A::scale(A::offset(A::scale(s.ad_value(579), p.p267), 1.0), p.p266), A::offset(A::scale(s.ad_value(580), p.p268), 1.0)), A::offset(A::scale(s.ad_value(581), p.p269), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(196, p.p270);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(45, p.p271);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(539, p.p272);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(540, p.p273);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(193, p.p274);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(48, p.p275);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(194, p.p276);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(49, p.p277);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_mul_ad(197, A::mul(A::offset(A::scale(A::powf(s.ad_value(579), p.p280), p.p279), p.p278), A::offset(A::scale(s.ad_value(580), p.p281), 1.0)), A::offset(A::scale(s.ad_value(581), p.p282), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(51, p.p283);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(52, p.p284);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(541, p.p285);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_mul_ad(551, A::scale(s.ad_value(580), p.p286), A::offset(A::scale(s.ad_value(580), p.p287), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_ad(198, &A::max_with_scalar(s.ad_value(551), 0.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(54, p.p288);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(55, p.p289);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(56, p.p290);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(57, p.p291);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(58, p.p292);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_mul_ad(552, A::mul(A::mul(s.ad_value(587), A::offset(A::scale(A::powf(s.ad_value(579), p.p295), p.p294), p.p293)), A::offset(A::scale(s.ad_value(580), p.p296), 1.0)), A::offset(A::scale(s.ad_value(581), p.p297), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_ad(199, &A::max_with_scalar(s.ad_value(552), 0.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_mul_ad(60, A::mul(A::scale(A::offset(A::scale(s.ad_value(579), p.p299), 1.0), p.p298), A::offset(A::scale(s.ad_value(580), p.p300), 1.0)), A::offset(A::scale(s.ad_value(581), p.p301), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(61, p.p302);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(62, p.p303);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_from_scalar_ad(554, p.p304, A::offset(A::div(A::scale(A::powf(s.ad_value(579), p.p306), p.p305), A::offset(A::scale(A::powf(s.ad_value(579), p.p308), p.p307), 1.0)), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_min_with_scalar_ad(542, A::max_with_scalar(s.ad_value(554), 1.0), 16.0);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_ad(557, A::mul(A::scale(A::powf(s.ad_value(579), p.p310), p.p309), A::offset(A::scale(s.ad_value(580), p.p313), 1.0)), A::offset(A::scale(A::powf(s.ad_value(579), p.p312), p.p311), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_ad(63, &A::max_with_scalar(s.ad_value(557), 0.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_ad(558, A::mul(A::scale(A::powf(s.ad_value(579), p.p315), p.p314), A::offset(A::scale(s.ad_value(580), p.p318), 1.0)), A::offset(A::scale(A::powf(s.ad_value(579), p.p317), p.p316), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_ad(64, &A::max_with_scalar(s.ad_value(558), 0.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(65, p.p319);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(66, p.p320);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(67, p.p321);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(75, p.p322);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_from_scalar(201, p.p323, 581);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_from_scalar(202, p.p324, 580);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_from_scalar(203, p.p325, 580);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_from_scalar(206, p.p339, 580);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_from_scalar(207, p.p340, 580);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_from_scalar(204, p.p326, 580);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_from_scalar(205, p.p327, 580);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(76, p.p328);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(77, p.p342);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(78, p.p329);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(79, p.p330);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(80, p.p331);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(81, p.p341);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(82, p.p332);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(83, p.p333);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(84, p.p334);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scale(85, 579, p.p335);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(86, p.p336);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(87, p.p337);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(88, p.p338);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_offset_ad(559, A::div_from_scalar(p.p345, s.ad_value(580)), p.p343);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_ad(89, &A::max_with_scalar(s.ad_value(559), 0.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_offset_ad(560, A::div_from_scalar(p.p346, s.ad_value(580)), p.p344);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_ad(90, &A::max_with_scalar(s.ad_value(560), 0.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(208, p.p347);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(209, p.p348);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(93, p.p349);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(94, p.p350);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(95, p.p351);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(96, p.p352);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_offset_scaled(97, 579, p.p355, p.p353);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_offset_scaled(98, 579, p.p356, p.p354);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_mul_ad(561, A::scale(A::offset(A::scale(s.ad_value(579), p.p389), 1.0), p.p388), A::offset(A::scale(s.ad_value(580), p.p390), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_ad(112, &A::max_with_scalar(s.ad_value(561), 0.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(210, p.p391);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(114, p.p392);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_mul_ad(562, A::scale(A::offset(A::scale(s.ad_value(579), p.p394), 1.0), p.p393), A::offset(A::scale(s.ad_value(580), p.p395), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_ad(115, &A::max_with_scalar(s.ad_value(562), 0.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_offset_scaled(589, 576, p.p358, (2.0 * p.p357));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(99, p.p359);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scale_ad(0, A::powf(s.ad_value(579), p.p362), p.p361);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_add_ad(211, A::add(A::offset(s.ad_value(0), p.p360), A::scale(s.ad_value(580), p.p363)), A::scale(s.ad_value(581), p.p364));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(212, p.p365);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_mul_ad(102, A::mul(A::scale(A::offset(A::scale(s.ad_value(579), p.p367), 1.0), p.p366), A::offset(A::scale(s.ad_value(580), p.p368), 1.0)), A::offset(A::scale(s.ad_value(581), p.p369), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(103, p.p370);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(104, p.p371);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_mul_ad(0, A::scale(A::powf(s.ad_value(584), p.p373), (p.p372 * 2.0)), A::offset(A::scale(s.ad_value(580), p.p374), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_min_with_scalar_ad(105, A::max_with_scalar(s.ad_value(0), 0.0), 5.0);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_ad_lhs(106, A::mul(A::scale(s.ad_value(105), p.p375), s.ad_value(534)), 533);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_mul_ad(0, A::powf(s.ad_value(584), p.p377), A::offset(A::scale(s.ad_value(580), p.p378), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scale(0, 0, p.p376);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_ad(107, &A::max_with_scalar(s.ad_value(0), 0.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_ad_lhs(108, A::mul(A::scale(s.ad_value(107), p.p379), s.ad_value(534)), 533);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(109, p.p380);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_offset_ad(0, A::mul(A::div_from_scalar((p.p381 * p.p382), s.ad_value(575)), A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(s.ad_value(575)), 1.0 / (p.p382))))), 1.0);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_ad(0, &A::max_with_scalar(s.ad_value(0), 1e-15));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_mul_ad(213, A::div(A::scale(s.ad_value(589), p.p244), A::mul(s.ad_value(0), s.ad_value(575))), A::offset(A::scale(s.ad_value(580), p.p383), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_add_ad(111, A::add(A::offset(A::scale(s.ad_value(579), p.p385), p.p384), A::scale(s.ad_value(580), p.p386)), A::mul(A::scale(s.ad_value(579), p.p387), s.ad_value(580)));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_mul(116, 578, 577);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_offset_scaled(563, 582, p.p397, p.p396);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_ad(117, &A::max_with_scalar(s.ad_value(563), 0.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(118, (p.p398 * 1000000.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_ad_lhs(119, A::scale(s.ad_value(578), p.p399), 570);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(120, p.p400);
        }

        if (!(s.v[611] != 0.0)) {
            s.copy_ad(185, 183);
        }

        if (!(s.v[611] != 0.0)) {
            s.copy_ad(186, 184);
        }

        if (!(s.v[611] != 0.0)) {
            s.copy_ad(135, 27);
        }

    }

    pub(super) fn stamp_transient_block_3(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (!(s.v[611] != 0.0)) {
            s.copy_ad(136, 28);
        }

        if (!(s.v[611] != 0.0)) {
            s.copy_ad(547, 546);
        }

        if (!(s.v[611] != 0.0)) {
            s.copy_ad(189, 187);
        }

        if (!(s.v[611] != 0.0)) {
            s.copy_ad(190, 188);
        }

        if (!(s.v[611] != 0.0)) {
            s.copy_ad(553, 552);
        }

        if (!(s.v[611] != 0.0)) {
            s.copy_ad(200, 199);
        }

        if (!(s.v[611] != 0.0)) {
            s.copy_ad(543, 542);
        }

        if (!(s.v[611] != 0.0)) {
            s.copy_ad(158, 63);
        }

        s.v[626] = if (p.p11 > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(121, p.p211);
        }

        s.v[627] = if (if self.param_given[401] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_scalar(121, p.p401);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(122, p.p212);
        }

        s.v[628] = if (if self.param_given[402] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[628] != 0.0)) {
            s.store_scalar(122, p.p402);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(123, p.p213);
        }

        s.v[629] = if (if self.param_given[403] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[629] != 0.0)) {
            s.store_scalar(123, p.p403);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(124, p.p216);
        }

        s.v[630] = if (if self.param_given[406] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[630] != 0.0)) {
            s.store_scalar(124, p.p406);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(125, p.p217);
        }

        s.v[631] = if (if self.param_given[407] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[631] != 0.0)) {
            s.store_scalar(125, p.p407);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(126, p.p214);
        }

        s.v[632] = if (if self.param_given[404] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[632] != 0.0)) {
            s.store_scalar(126, p.p404);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(127, p.p215);
        }

        s.v[633] = if (if self.param_given[405] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[633] != 0.0)) {
            s.store_scalar(127, p.p405);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_div_ad(0, A::mul(s.ad_value(122), A::pow(s.ad_value(579), s.ad_value(123))), A::offset(A::mul(s.ad_value(126), A::pow(s.ad_value(579), s.ad_value(127))), 1.0));
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_add_ad(185, A::add(A::add(s.ad_value(121), s.ad_value(0)), A::mul(s.ad_value(124), s.ad_value(580))), A::mul(s.ad_value(125), s.ad_value(581)));
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(128, p.p218);
        }

        s.v[634] = if (if self.param_given[408] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[634] != 0.0)) {
            s.store_scalar(128, p.p408);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(129, p.p219);
        }

        s.v[635] = if (if self.param_given[409] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[635] != 0.0)) {
            s.store_scalar(129, p.p409);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_add_ad_rhs(186, 128, A::mul(A::div(A::mul(s.ad_value(129), s.ad_value(534)), s.ad_value(533)), s.ad_value(0)));
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(132, p.p228);
        }

        s.v[636] = if (if self.param_given[410] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[636] != 0.0)) {
            s.store_scalar(132, p.p410);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(133, p.p229);
        }

        s.v[637] = if (if self.param_given[411] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[637] != 0.0)) {
            s.store_scalar(133, p.p411);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(134, p.p230);
        }

        s.v[638] = if (if self.param_given[412] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[638] != 0.0)) {
            s.store_scalar(134, p.p412);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_mul_ad(545, A::mul(A::scale(s.ad_value(132), 2.0), A::pow(s.ad_value(584), s.ad_value(133))), A::offset(A::mul(s.ad_value(134), s.ad_value(580)), 1.0));
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_min_with_scalar_ad(135, A::max_with_scalar(s.ad_value(545), 0.0), 5.0);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_div_ad_lhs(136, A::mul(A::scale(s.ad_value(135), p.p231), s.ad_value(534)), 533);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(137, p.p235);
        }

        s.v[639] = if (if self.param_given[413] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[639] != 0.0)) {
            s.store_scalar(137, p.p413);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(138, p.p236);
        }

        s.v[640] = if (if self.param_given[414] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[640] != 0.0)) {
            s.store_scalar(138, p.p414);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(139, p.p237);
        }

        s.v[641] = if (if self.param_given[415] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[641] != 0.0)) {
            s.store_scalar(139, p.p415);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_mul_ad(0, A::pow(s.ad_value(584), s.ad_value(138)), A::offset(A::mul(s.ad_value(139), s.ad_value(580)), 1.0));
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_mul(547, 137, 0);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_ad(189, &A::max_with_scalar(s.ad_value(547), 0.0));
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_div_ad_lhs(190, A::mul(A::scale(s.ad_value(189), p.p238), s.ad_value(534)), 533);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(142, p.p293);
        }

        s.v[642] = if (if self.param_given[416] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[642] != 0.0)) {
            s.store_scalar(142, p.p416);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(143, p.p294);
        }

        s.v[643] = if (if self.param_given[417] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[643] != 0.0)) {
            s.store_scalar(143, p.p417);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(144, p.p295);
        }

        s.v[644] = if (if self.param_given[418] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[644] != 0.0)) {
            s.store_scalar(144, p.p418);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(145, p.p296);
        }

        s.v[645] = if (if self.param_given[419] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[645] != 0.0)) {
            s.store_scalar(145, p.p419);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(146, p.p297);
        }

        s.v[646] = if (if self.param_given[420] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[646] != 0.0)) {
            s.store_scalar(146, p.p420);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_mul_ad(553, A::mul(A::mul(s.ad_value(587), A::add(s.ad_value(142), A::mul(s.ad_value(143), A::pow(s.ad_value(579), s.ad_value(144))))), A::offset(A::mul(s.ad_value(145), s.ad_value(580)), 1.0)), A::offset(A::mul(s.ad_value(146), s.ad_value(581)), 1.0));
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_ad(200, &A::max_with_scalar(s.ad_value(553), 0.0));
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(148, p.p304);
        }

        s.v[647] = if (if self.param_given[421] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[647] != 0.0)) {
            s.store_scalar(148, p.p421);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(149, p.p305);
        }

        s.v[648] = if (if self.param_given[422] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[648] != 0.0)) {
            s.store_scalar(149, p.p422);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(150, p.p306);
        }

        s.v[649] = if (if self.param_given[423] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[649] != 0.0)) {
            s.store_scalar(150, p.p423);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(151, p.p307);
        }

        s.v[650] = if (if self.param_given[424] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[650] != 0.0)) {
            s.store_scalar(151, p.p424);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(152, p.p308);
        }

        s.v[651] = if (if self.param_given[425] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[651] != 0.0)) {
            s.store_scalar(152, p.p425);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_div_ad_rhs(555, 148, A::offset(A::div(A::mul(s.ad_value(149), A::pow(s.ad_value(579), s.ad_value(150))), A::offset(A::mul(s.ad_value(151), A::pow(s.ad_value(579), s.ad_value(152))), 1.0)), 1.0));
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_min_with_scalar_ad(543, A::max_with_scalar(s.ad_value(555), 1.0), 16.0);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(153, p.p309);
        }

        s.v[652] = if (if self.param_given[426] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[652] != 0.0)) {
            s.store_scalar(153, p.p426);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(154, p.p310);
        }

        s.v[653] = if (if self.param_given[427] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[653] != 0.0)) {
            s.store_scalar(154, p.p427);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(155, p.p311);
        }

        s.v[654] = if (if self.param_given[428] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[654] != 0.0)) {
            s.store_scalar(155, p.p428);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(156, p.p312);
        }

        s.v[655] = if (if self.param_given[429] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[655] != 0.0)) {
            s.store_scalar(156, p.p429);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_scalar(157, p.p313);
        }

        s.v[656] = if (if self.param_given[430] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) && (s.v[656] != 0.0)) {
            s.store_scalar(157, p.p430);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_div_ad(556, A::mul(A::mul(s.ad_value(153), A::pow(s.ad_value(579), s.ad_value(154))), A::offset(A::mul(s.ad_value(157), s.ad_value(580)), 1.0)), A::offset(A::mul(s.ad_value(155), A::pow(s.ad_value(579), s.ad_value(156))), 1.0));
        }

        if ((!(s.v[611] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_ad(158, &A::max_with_scalar(s.ad_value(556), 0.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_mul_ad_lhs(0, A::div_from_scalar(3.45313e-11, s.ad_value(533)), 578);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scale(159, 0, p.p431);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scale(160, 0, p.p432);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_from_scalar_ad(161, p.p433, A::max_with_scalar(A::offset(A::div(A::scale(s.ad_value(570), p.p434), s.ad_value(578)), 1.0), 0.001));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(162, p.p435);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(163, p.p436);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_offset_scaled(564, 583, p.p439, p.p437);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_ad(164, &A::max_with_scalar(s.ad_value(564), 0.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_offset_scaled(565, 583, p.p440, p.p438);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_ad(165, &A::max_with_scalar(s.ad_value(565), 0.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_ad_lhs(166, A::mul(A::mul(A::scale(s.ad_value(229), p.p441), s.ad_value(14)), s.ad_value(576)), 575);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(167, p.p442);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_max_with_scalar_ad(0, A::add(A::add(A::offset(A::scale(s.ad_value(582), p.p444), 1.0), A::scale(s.ad_value(583), p.p445)), A::mul(A::scale(s.ad_value(582), p.p446), s.ad_value(583))), 1e-10);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(2, 0.0);
        }

        s.v[657] = if ((p.p29 > 1.0) && (p.p28 > 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[611] != 0.0)) && (s.v[657] != 0.0)) {
            s.store_scalar(3, ((-(p.p28 + p.p20)) / p.p449));
        }

        s.v[658] = if (((s.v[3]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[657] != 0.0)) && (s.v[658] != 0.0)) {
            s.store_exp(4, 3);
        }

        s.v[659] = if (s.v[3] < (-80.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[611] != 0.0)) && (s.v[657] != 0.0)) && (!(s.v[658] != 0.0))) && (s.v[659] != 0.0)) {
            s.store_div_from_scalar_ad(4, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(3)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(3)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(3)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((!(s.v[611] != 0.0)) && (s.v[657] != 0.0)) && (!(s.v[658] != 0.0))) && (!(s.v[659] != 0.0))) {
            s.store_scale_ad(4, A::offset(A::mul(A::offset(s.ad_value(3), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(3), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(3), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[657] != 0.0)) {
            s.store_sub_from_scalar(5, 1.0, 4);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[657] != 0.0)) {
            s.store_div_ad(2, A::mul(A::scale(s.ad_value(4), (2.0 * p.p450)), A::sub(s.ad_value(5), A::scale(A::sub_from_scalar(1.0, A::powf(s.ad_value(4), p.p29)), 1.0 / (p.p29)))), A::square(s.ad_value(5)));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_ad_rhs(0, 0, A::offset(s.ad_value(2), 1.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_div_from_scalar(566, p.p443, 0);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_ad(214, &A::max_with_scalar(s.ad_value(566), 1e-6));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(169, p.p447);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scale(567, 0, p.p448);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_ad(170, &A::max_with_scalar(s.ad_value(567), 0.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(171, p.p451);
        }

    }

    pub(super) fn stamp_transient_block_4(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (!(s.v[611] != 0.0)) {
            s.store_mul_ad(172, A::mul(A::mul(A::mul(A::scale(s.ad_value(548), p.p452), s.ad_value(548)), s.ad_value(580)), s.ad_value(580)), A::powf(s.ad_value(579), (p.p453 - 2.0)));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_add_ad(568, A::scale(s.ad_value(581), p.p454), A::scale(s.ad_value(580), p.p455));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_ad(173, &A::max_with_scalar(s.ad_value(568), 0.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scale(174, 581, p.p456);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scale(175, 581, p.p457);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(176, p.p458);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(177, p.p459);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(178, p.p460);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_offset_scaled(0, 579, p.p490, p.p489);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_ad(179, &A::max_with_scalar(s.ad_value(0), 0.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_offset_scaled(0, 579, p.p492, p.p491);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_ad(180, &A::max_with_scalar(s.ad_value(0), 0.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(181, p.p493);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(182, p.p494);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_offset_ad(310, A::add(A::div(A::scale(A::add(A::scale(s.ad_value(314), (0.3333333333333 * 1.0 / (p.p37))), s.ad_value(315)), p.p498), A::scale(s.ad_value(313), p.p37)), A::div_from_scalar((p.p496 + p.p497), A::mul(s.ad_value(314), s.ad_value(312)))), (p.p29 * p.p495));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_ad(311, &A::max_with_scalar(s.ad_value(310), 0.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(319, (p.p500).max(0.0));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(323, (p.p501).max(0.0));
        }

        s.v[660] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[611] != 0.0)) && (s.v[660] != 0.0)) {
            s.copy_ad(323, 319);
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scale(318, 319, (p.p29 * p.p39));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scale(322, 323, (p.p29 * p.p40));
        }

        if (!(s.v[611] != 0.0)) {
            s.store_scalar(326, (p.p29 * p.p502));
        }

        s.v[661] = if ((((p.p461 > 0.0) && (p.p26 > 0.0)) && (p.p27 > 0.0)) && ((p.p29 == 1.0) || ((p.p29 > 1.0) && (p.p28 > 0.0)))) { 1.0 } else { 0.0 };

        s.v[662] = if (p.p461 == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_scalar(592, 0.0);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_scalar(593, 0.0);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_scalar(594, 0.0);
        }

        let mut assign5720_loop_guard: usize = 0;
        while {
            let assign5720_cond_e4998: f64 = (p.p29 - 0.5);
            let assign5720_cond_e5000: f64 = if ((((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) && (s.v[594] < assign5720_cond_e4998)) { 1.0 } else { 0.0 };
            assign5720_cond_e5000 != 0.0
        } {
            assign5720_loop_guard += 1;
            assert!(assign5720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
                s.store_add_ad_rhs(592, 592, A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(594), (p.p28 + p.p20)), (p.p26 + (0.5 * p.p20)))));
            }
            if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
                s.store_add_ad_rhs(593, 593, A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(594), (p.p28 + p.p20)), (p.p27 + (0.5 * p.p20)))));
            }
            if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
                s.store_offset(594, 594, 1.0);
            }
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_scale(595, 592, 1.0 / (p.p29));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_scale(596, 593, 1.0 / (p.p29));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_scalar(597, (1.0 / (p.p462 + (0.5 * p.p20))));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_scalar(598, (1.0 / (p.p463 + (0.5 * p.p20))));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_max_with_scalar_ad(599, A::offset(s.ad_value(573), p.p20), 1e-9);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_max_with_scalar_ad(600, A::offset(A::add(s.ad_value(532), s.ad_value(574)), p.p464), 1e-9);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_div_from_scalar_ad(601, 1.0, A::powf(s.ad_value(599), p.p471));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_div_from_scalar_ad(602, 1.0, A::powf(s.ad_value(600), p.p472));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_mul_ad(603, A::add(A::add(A::offset(A::scale(s.ad_value(601), p.p468), 1.0), A::scale(s.ad_value(602), p.p469)), A::mul(A::scale(s.ad_value(601), p.p470), s.ad_value(602))), A::offset(A::scale(A::offset(s.ad_value(221), (-1.0)), p.p467), 1.0));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_div_ad_lhs(604, A::scale(A::add(s.ad_value(595), s.ad_value(596)), p.p465), 603);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_div_ad_lhs(605, A::scale(A::add(s.ad_value(597), s.ad_value(598)), p.p465), 603);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_div_from_scalar_ad(601, 1.0, A::powf(s.ad_value(599), p.p477));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_div_from_scalar_ad(602, 1.0, A::powf(s.ad_value(600), p.p478));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_max_with_scalar_ad(606, A::add(A::add(A::offset(A::scale(s.ad_value(601), p.p474), 1.0), A::scale(s.ad_value(602), p.p475)), A::mul(A::scale(s.ad_value(601), p.p476), s.ad_value(602))), 1e-20);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_sub_ad_lhs(607, A::sub(A::add(s.ad_value(595), s.ad_value(596)), s.ad_value(597)), 598);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_div_ad(548, A::mul(s.ad_value(548), A::offset(s.ad_value(604), 1.0)), A::offset(s.ad_value(605), 1.0));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_ad(191, &A::max_with_scalar(s.ad_value(548), 1e-10));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_scale(192, 191, p.p254);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_div_ad(0, A::mul(A::offset(s.ad_value(604), 1.0), A::offset(A::scale(s.ad_value(605), p.p466), 1.0)), A::mul(A::offset(s.ad_value(605), 1.0), A::offset(A::scale(s.ad_value(604), p.p466), 1.0)));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_mul(552, 552, 0);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_ad(199, &A::max_with_scalar(s.ad_value(552), 0.0));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_mul(553, 553, 0);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_ad(200, &A::max_with_scalar(s.ad_value(553), 0.0));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_div_ad_lhs(0, A::scale(s.ad_value(607), p.p473), 606);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_add(183, 183, 0);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_add(184, 184, 0);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_add(185, 185, 0);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_add(186, 186, 0);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_div_ad(0, A::scale(s.ad_value(607), p.p479), A::powf(s.ad_value(606), p.p480));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_add(546, 546, 0);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_ad(187, &A::max_with_scalar(s.ad_value(546), 0.0));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_add(547, 547, 0);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_ad(189, &A::max_with_scalar(s.ad_value(547), 0.0));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_div_ad_lhs(0, A::scale(s.ad_value(534), p.p238), 533);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_mul(188, 187, 0);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_mul(190, 189, 0);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_scalar(592, 0.0);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_scalar(594, 0.0);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_scalar(0, ((-1.0) / p.p482));
        }

        let mut assign6120_loop_guard: usize = 0;
        while {
            let assign6120_cond_e5595: f64 = (p.p29 - 0.5);
            let assign6120_cond_e5597: f64 = if ((((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) && (s.v[594] < assign6120_cond_e5595)) { 1.0 } else { 0.0 };
            assign6120_cond_e5597 != 0.0
        } {
            assign6120_loop_guard += 1;
            assert!(assign6120_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            s.v[663] = if (((-((p.p26 + (0.5 * p.p20)) + (s.v[594] * (p.p28 + p.p20)))) / p.p481) > (-80.0)) { 1.0 } else { 0.0 };
            if ((((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) && (s.v[663] != 0.0)) {
                s.store_exp_ad(2, A::scale(A::neg(A::offset(A::scale(s.ad_value(594), (p.p28 + p.p20)), (p.p26 + (0.5 * p.p20)))), 1.0 / (p.p481)));
            }
            if ((((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) && (!(s.v[663] != 0.0))) {
                let assign6120_body2_ad_e5712: A = A::offset(A::mul(A::scale(A::offset(A::neg(A::scale(A::neg(A::offset(A::scale(s.ad_value(594), (p.p28 + p.p20)), (p.p26 + (0.5 * p.p20)))), 1.0 / (p.p481))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::scale(A::neg(A::offset(A::scale(s.ad_value(594), (p.p28 + p.p20)), (p.p26 + (0.5 * p.p20)))), 1.0 / (p.p481))), (-80.0)), 0.3333333333333), 1.0)), 1.0);
                s.store_div_from_scalar_ad(2, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::scale(A::neg(A::offset(A::scale(s.ad_value(594), (p.p28 + p.p20)), (p.p26 + (0.5 * p.p20)))), 1.0 / (p.p481))), (-80.0)), assign6120_body2_ad_e5712), 1.0));
            }
            s.v[664] = if (((-((p.p27 + (0.5 * p.p20)) + (((p.p29 - 1.0) - s.v[594]) * (p.p28 + p.p20)))) / p.p481) > (-80.0)) { 1.0 } else { 0.0 };
            if ((((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) && (s.v[664] != 0.0)) {
                s.store_exp_ad(3, A::scale(A::neg(A::offset(A::scale(A::sub_from_scalar((p.p29 - 1.0), s.ad_value(594)), (p.p28 + p.p20)), (p.p27 + (0.5 * p.p20)))), 1.0 / (p.p481)));
            }
            if ((((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) && (!(s.v[664] != 0.0))) {
                let assign6120_body5_ad_e5851: A = A::mul(A::scale(A::offset(A::neg(A::scale(A::neg(A::offset(A::scale(A::sub_from_scalar((p.p29 - 1.0), s.ad_value(594)), (p.p28 + p.p20)), (p.p27 + (0.5 * p.p20)))), 1.0 / (p.p481))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::scale(A::neg(A::offset(A::scale(A::sub_from_scalar((p.p29 - 1.0), s.ad_value(594)), (p.p28 + p.p20)), (p.p27 + (0.5 * p.p20)))), 1.0 / (p.p481))), (-80.0)), 0.3333333333333), 1.0));
                s.store_div_from_scalar_ad(3, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::scale(A::neg(A::offset(A::scale(A::sub_from_scalar((p.p29 - 1.0), s.ad_value(594)), (p.p28 + p.p20)), (p.p27 + (0.5 * p.p20)))), 1.0 / (p.p481))), (-80.0)), A::offset(assign6120_body5_ad_e5851, 1.0)), 1.0));
            }
            if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
                s.store_powf_ad(4, A::sub_from_scalar(1.0, s.ad_value(2)), (-p.p482));
            }
            if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
                s.store_powf_ad(5, A::sub_from_scalar(1.0, s.ad_value(3)), (-p.p482));
            }
            if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
                s.store_add_ad_rhs(592, 592, A::pow(A::scale(A::add(s.ad_value(4), s.ad_value(5)), 0.5), s.ad_value(0)));
            }
            if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
                s.store_offset(594, 594, 1.0);
            }
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_sub_from_scalar_ad(608, 1.0, A::scale(s.ad_value(592), 1.0 / (p.p29)));
        }

        s.v[665] = if (((-(p.p462 + (0.5 * p.p20))) / p.p481) > (-80.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) && (s.v[665] != 0.0)) {
            s.store_scalar(2, ((((-(p.p462 + (0.5 * p.p20))) / p.p481)) as f64).exp());
        }

        if ((((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) && (!(s.v[665] != 0.0))) {
            s.store_scalar(2, (1.80485e-35 / (1.0 + (((-((-(p.p462 + (0.5 * p.p20))) / p.p481)) - 80.0) * (1.0 + ((0.5 * ((-((-(p.p462 + (0.5 * p.p20))) / p.p481)) - 80.0)) * (1.0 + (((-((-(p.p462 + (0.5 * p.p20))) / p.p481)) - 80.0) * 0.3333333333333))))))));
        }

        s.v[666] = if (((-(p.p463 + (0.5 * p.p20))) / p.p481) > (-80.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) && (s.v[666] != 0.0)) {
            s.store_scalar(3, ((((-(p.p463 + (0.5 * p.p20))) / p.p481)) as f64).exp());
        }

        if ((((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) && (!(s.v[666] != 0.0))) {
            s.store_scalar(3, (1.80485e-35 / (1.0 + (((-((-(p.p463 + (0.5 * p.p20))) / p.p481)) - 80.0) * (1.0 + ((0.5 * ((-((-(p.p463 + (0.5 * p.p20))) / p.p481)) - 80.0)) * (1.0 + (((-((-(p.p463 + (0.5 * p.p20))) / p.p481)) - 80.0) * 0.3333333333333))))))));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_powf_ad(4, A::sub_from_scalar(1.0, s.ad_value(2)), (-p.p482));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_powf_ad(5, A::sub_from_scalar(1.0, s.ad_value(3)), (-p.p482));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_sub_from_scalar_ad(609, 1.0, A::pow(A::scale(A::add(s.ad_value(4), s.ad_value(5)), 0.5), s.ad_value(0)));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_max_with_scalar_ad(600, A::offset(A::add(s.ad_value(532), s.ad_value(574)), p.p464), 1e-9);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_div_from_scalar_ad(610, p.p486, A::offset(A::scale(A::offset(s.ad_value(221), (-1.0)), p.p487), 1.0));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_mul(604, 610, 608);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_mul(605, 610, 609);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_sub(607, 608, 609);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_max_with_scalar_ad(606, A::offset(A::div(A::scale(s.ad_value(600), p.p484), s.ad_value(570)), 1.0), 1e-20);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_div_ad(548, A::mul(s.ad_value(548), A::offset(s.ad_value(604), 1.0)), A::offset(s.ad_value(605), 1.0));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_ad(191, &A::max_with_scalar(s.ad_value(548), 1e-10));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_scale(192, 191, p.p254);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_div_ad(0, A::mul(A::offset(s.ad_value(604), 1.0), A::offset(A::scale(s.ad_value(605), p.p488), 1.0)), A::mul(A::offset(s.ad_value(605), 1.0), A::offset(A::scale(s.ad_value(604), p.p488), 1.0)));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_mul(552, 552, 0);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_ad(199, &A::max_with_scalar(s.ad_value(552), 0.0));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_mul(553, 553, 0);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_ad(200, &A::max_with_scalar(s.ad_value(553), 0.0));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_div_ad_lhs(0, A::scale(s.ad_value(607), p.p483), 606);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_add(183, 183, 0);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_add(184, 184, 0);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_add(185, 185, 0);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_add(186, 186, 0);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_mul_ad(0, A::mul(A::scale(s.ad_value(607), p.p485), A::powf(s.ad_value(584), p.p236)), A::offset(A::scale(s.ad_value(580), p.p237), 1.0));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_add(546, 546, 0);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_ad(187, &A::max_with_scalar(s.ad_value(546), 0.0));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_add(547, 547, 0);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_ad(189, &A::max_with_scalar(s.ad_value(547), 0.0));
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_div_ad_lhs(0, A::scale(s.ad_value(534), p.p238), 533);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_mul(188, 187, 0);
        }

        if (((!(s.v[611] != 0.0)) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_mul(190, 189, 0);
        }

        s.v[667] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[667] != 0.0) {
            s.copy_ad(20, 19);
        }

        if (s.v[667] != 0.0) {
            s.copy_ad(203, 202);
        }

        if (s.v[667] != 0.0) {
            s.copy_ad(207, 206);
        }

        if (s.v[667] != 0.0) {
            s.copy_ad(205, 204);
        }

        if (s.v[667] != 0.0) {
            s.copy_ad(90, 89);
        }

        if (s.v[667] != 0.0) {
            s.copy_ad(209, 208);
        }

        if (s.v[667] != 0.0) {
            s.copy_ad(94, 93);
        }

        if (s.v[667] != 0.0) {
            s.copy_ad(96, 95);
        }

        if (s.v[667] != 0.0) {
            s.copy_ad(98, 97);
        }

        if (s.v[667] != 0.0) {
            s.copy_ad(160, 159);
        }

        if (s.v[667] != 0.0) {
            s.copy_ad(165, 164);
        }

        s.store_sub_from_scalar(228, 1.0, 15);

    }

    pub(super) fn stamp_transient_block_5(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.store_add_ad(229, A::scale(s.ad_value(228), 1.04479e-10), A::scale(s.ad_value(15), 1.43438e-10));

        s.store_sub_from_scalar_ad(230, 1.17, A::div(A::scale(s.ad_value(218), 0.000473), A::offset(s.ad_value(217), 636.0)));

        s.store_sub_from_scalar_ad(231, 0.744, A::div(A::scale(s.ad_value(218), 0.0004774), A::offset(s.ad_value(217), 235.0)));

        s.store_mul_ad_lhs(232, A::add(A::sub(s.ad_value(231), s.ad_value(230)), A::scale(s.ad_value(228), (-0.4))), 15);

        s.store_add(233, 230, 232);

        s.store_mul_ad_lhs(234, A::scale(s.ad_value(233), 0.5), 224);

        s.copy_ad(235, 234);

        s.store_div_from_scalar_ad(238, 1.0, A::offset(A::sqrt(A::scale(s.ad_value(15), 10.0)), 1.0));

        s.store_sub_ad(237, A::scale(s.ad_value(15), 0.05), A::scale(s.ad_value(232), 0.5));

        s.store_scale_ad(0, A::mul(A::scale(s.ad_value(536), (1.602176565e-19 * 0.5)), s.ad_value(14)), 28959234086.17689);

        s.v[668] = if (s.v[535] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[668] != 0.0) {
            s.store_mul_ad_rhs(243, 0, A::offset(s.ad_value(533), (p.p13 * 4e-10)));
        }

        if (s.v[668] != 0.0) {
            s.store_mul_ad_rhs(244, 0, A::offset(s.ad_value(534), (p.p13 * 4e-10)));
        }

        if (!(s.v[668] != 0.0)) {
            s.store_mul_ad(243, A::neg(s.ad_value(0)), A::offset(s.ad_value(533), (p.p13 * 4e-10)));
        }

        if (!(s.v[668] != 0.0)) {
            s.store_mul_ad(244, A::neg(s.ad_value(0)), A::offset(s.ad_value(534), (p.p13 * 4e-10)));
        }

        s.store_sqrt_ad(0, A::scale(s.ad_value(217), 0.0033333333333));

        s.store_mul_ad_lhs(2, A::mul(A::scale(s.ad_value(0), 4.05e25), s.ad_value(0)), 0);

        s.store_mul(252, 2, 238);

        s.store_mul_ad_rhs(251, 2, A::exp(A::mul(A::scale(s.ad_value(232), 0.5), s.ad_value(224))));

        s.store_mul_ad_rhs(590, 2, A::exp(A::mul(A::scale(s.ad_value(232), 0.5), s.ad_value(224))));

        s.store_div_from_scalar(239, 3.45313e-11, 533);

        s.store_div_from_scalar(240, 3.45313e-11, 534);

        s.v[669] = if (s.v[538] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[669] != 0.0) {
            s.store_mul_ad_rhs(241, 239, A::offset(s.ad_value(538), 1.0));
        }

        if (s.v[669] != 0.0) {
            s.copy_ad(242, 240);
        }

        if (!(s.v[669] != 0.0)) {
            s.copy_ad(241, 239);
        }

        if (!(s.v[669] != 0.0)) {
            s.store_mul_ad_rhs(242, 240, A::sub_from_scalar(1.0, s.ad_value(538)));
        }

        s.store_div(245, 229, 14);

        s.store_mul_ad_rhs(226, 223, A::offset(A::mul(s.ad_value(17), s.ad_value(222)), 1.0));

        s.store_div_from_scalar(227, 1.0, 226);

        s.store_mul_ad_lhs(236, A::scale(s.ad_value(233), 0.5), 227);

        s.store_div(246, 241, 245);

        s.store_div(247, 242, 245);

        s.store_div_from_scalar_ad(248, 1.0, A::add(A::offset(A::div_from_scalar(1.0, s.ad_value(246)), 1.0), A::div_from_scalar(1.0, s.ad_value(247))));

        s.store_mul_ad_lhs(253, A::mul(A::scale(s.ad_value(252), (2.0 * 1.602176565e-19)), s.ad_value(229)), 227);

        s.store_offset_ad(254, A::ln(A::div(A::square(s.ad_value(245)), s.ad_value(253))), (-0.6931471805599));

        s.store_mul_ad_lhs(255, A::div(A::mul(A::scale(s.ad_value(29), (0.5 * 1.602176565e-19)), s.ad_value(14)), A::add(s.ad_value(241), s.ad_value(242))), 227);

        s.store_mul(0, 34, 220);

        s.store_add(31, 187, 0);

        s.store_add(32, 188, 0);

        s.store_add(140, 189, 0);

        s.store_add(141, 190, 0);

        s.store_mul(329, 35, 227);

        s.store_div_ad_lhs(260, A::sqrt(A::mul(A::scale(s.ad_value(537), ((2.0 * 1.602176565e-19) * 1.04479e-10)), s.ad_value(224))), 242);

        s.store_square(261, 260);

        s.store_div_from_scalar(262, 1.0, 261);

        s.store_offset_scaled(263, 260, 0.707106781186545, 1.0);

        s.store_div_from_scalar(264, 1.0, 263);

        s.store_scale(265, 263, 1e-5);

        s.store_add_ad_lhs(591, A::ln(A::div(s.ad_value(537), s.ad_value(590))), 234);

        s.store_scale(266, 591, 2.0);

        s.v[670] = if (p.p2 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[670] != 0.0) {
            s.store_add_ad_rhs(184, 184, A::mul(A::mul(s.ad_value(16), s.ad_value(223)), s.ad_value(591)));
        }

        if (s.v[670] != 0.0) {
            s.store_add_ad_rhs(186, 186, A::mul(A::mul(s.ad_value(16), s.ad_value(223)), s.ad_value(591)));
        }

        s.v[249] = 0.0;

        s.v[671] = if (p.p9 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[671] != 0.0) {
            s.store_mul_ad_rhs(249, 223, A::add(A::ln(A::div(s.ad_value(24), s.ad_value(251))), s.ad_value(234)));
        }

        s.store_div_ad_lhs(250, A::sqrt(A::mul(A::scale(s.ad_value(229), (2.0 * 1.602176565e-19)), s.ad_value(24))), 239);

        s.v[257] = 15.0;

        s.v[672] = if (p.p10 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[672] != 0.0) {
            s.store_scale_ad(257, A::add(A::offset(A::div_from_scalar(2970.0, s.ad_value(8)), 15.0), A::sqrt(A::offset(A::mul(A::sub_from_scalar(15.0, A::div_from_scalar(2970.0, s.ad_value(8))), A::sub_from_scalar(15.0, A::div_from_scalar(2970.0, s.ad_value(8)))), 1e-6))), 0.5);
        }

        s.v[256] = 0.0;

        s.v[258] = 0.0;

        s.store_mul_ad_lhs(259, A::scale(s.ad_value(14), 1e18), 14);

        s.v[673] = if (p.p13 > 0.0) { 1.0 } else { 0.0 };

        s.v[674] = if (p.p14 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[673] != 0.0) && (s.v[674] != 0.0)) {
            s.store_div_from_scalar(256, 0.409618895, 259);
        }

        if ((s.v[673] != 0.0) && (s.v[674] != 0.0)) {
            s.store_scale_ad(258, A::exp(A::scale(A::ln(A::mul(s.ad_value(226), s.ad_value(259))), (-0.3333333333333))), ((0.4 * p.p13) * 1.27520989));
        }

        if ((s.v[673] != 0.0) && (!(s.v[674] != 0.0))) {
            s.store_div_from_scalar(256, 0.723134895, 259);
        }

        if ((s.v[673] != 0.0) && (!(s.v[674] != 0.0))) {
            s.store_scale_ad(258, A::exp(A::scale(A::ln(A::mul(s.ad_value(226), s.ad_value(259))), (-0.3333333333333))), ((0.4 * p.p13) * 1.5412087));
        }

        s.store_add_ad_lhs(0, A::mul(A::scale(s.ad_value(23), p.p14), s.ad_value(220)), 256);

        s.store_sub_ad_lhs(2, A::offset(s.ad_value(0), p.p34), 249);

        s.store_add_ad_lhs(21, A::scale(A::add(A::add(s.ad_value(183), s.ad_value(237)), s.ad_value(243)), p.p14), 2);

        s.store_add_ad_lhs(22, A::scale(A::add(A::add(s.ad_value(184), s.ad_value(237)), s.ad_value(244)), p.p14), 0);

        s.store_add_ad_lhs(130, A::scale(A::add(A::add(s.ad_value(185), s.ad_value(237)), s.ad_value(243)), p.p14), 2);

        s.store_add_ad_lhs(131, A::scale(A::add(A::add(s.ad_value(186), s.ad_value(237)), s.ad_value(244)), p.p14), 0);

        s.store_ln(295, 222);

        s.store_scale_ad(296, A::exp(A::mul(s.ad_value(40), s.ad_value(295))), p.p35);

        s.store_mul(38, 191, 296);

        s.store_mul(39, 192, 296);

        s.store_exp_ad(297, A::mul(s.ad_value(48), s.ad_value(295)));

        s.store_mul(46, 193, 297);

        s.store_exp_ad(298, A::mul(s.ad_value(49), s.ad_value(295)));

        s.store_mul(47, 194, 298);

        s.store_exp_ad(299, A::mul(s.ad_value(43), s.ad_value(295)));

        s.store_mul(33, 195, 299);

        s.store_exp_ad(300, A::mul(s.ad_value(45), s.ad_value(295)));

        s.store_mul(44, 196, 300);

        s.store_exp_ad(301, A::mul(s.ad_value(52), s.ad_value(295)));

        s.store_mul(50, 197, 301);

        s.store_div_ad_lhs(0, A::scale(s.ad_value(226), 1e-8), 14);

        s.store_mul(267, 0, 46);

        s.store_div_from_scalar_ad(268, 1.0, A::scale(s.ad_value(539), 0.5));

        s.store_div(269, 268, 540);

        s.v[675] = if (p.p14 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[675] != 0.0) {
            s.store_scale(270, 541, 0.5);
        }

        if (!(s.v[675] != 0.0)) {
            s.store_scale(270, 541, 0.3333333333333);
        }

        s.store_sub_from_scalar(271, 1.0, 270);

        s.store_exp_ad(302, A::mul(s.ad_value(55), s.ad_value(295)));

        s.store_mul(53, 198, 302);

        s.store_mul_ad_lhs(272, A::scale(s.ad_value(53), 2.0), 226);

        s.store_offset_ad(215, A::exp(A::scale(A::ln(A::offset(A::exp(A::scale(A::div_from_scalar(16.0, s.ad_value(542)), 0.6931471805599)), (-1.0))), 0.375)), (-1.0));

        s.store_offset_ad(216, A::exp(A::scale(A::ln(A::offset(A::exp(A::scale(A::div_from_scalar(16.0, s.ad_value(543)), 0.6931471805599)), (-1.0))), 0.375)), (-1.0));

        s.store_exp_ad(303, A::mul(s.ad_value(60), s.ad_value(295)));

        s.store_mul_ad_lhs(59, A::mul(s.ad_value(199), s.ad_value(303)), 296);

        s.store_mul(273, 59, 226);

        s.store_mul_ad_lhs(147, A::mul(s.ad_value(200), s.ad_value(303)), 296);

        s.store_mul(274, 147, 226);

        s.store_mul(275, 64, 227);

        s.store_exp_ad(304, A::mul(A::neg(s.ad_value(76)), s.ad_value(295)));

        s.store_mul(68, 201, 304);

        s.store_mul(69, 202, 304);

        s.store_mul(70, 203, 304);

        s.store_mul(71, 204, 304);

        s.store_mul(72, 205, 304);

        s.store_exp_ad(304, A::mul(A::neg(s.ad_value(77)), s.ad_value(295)));

        s.store_mul(73, 206, 304);

        s.store_mul(74, 207, 304);

        s.store_div_from_scalar(276, 1.0, 87);

        s.store_scale_ad(277, A::sqrt(A::scale(s.ad_value(87), ((2.0 * 1.602176565e-19) * 9.10938291e-31))), ((4.0 * 0.3333333333333) * 9.482522386533242e33));

        s.store_mul(278, 277, 18);

        s.store_mul(279, 277, 18);

        s.v[280] = 0.0;

        s.v[676] = if (s.v[79] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[676] != 0.0) {
            s.store_div_ad_lhs(280, A::scale(s.ad_value(78), (-0.495)), 79);
        }

        s.v[281] = 0.0;

        s.v[677] = if (s.v[82] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[677] != 0.0) {
            s.store_div_ad_lhs(281, A::scale(s.ad_value(80), (-0.495)), 82);
        }

        s.v[282] = 0.0;

        s.v[678] = if (s.v[84] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[678] != 0.0) {
            s.store_div_ad_lhs(282, A::scale(s.ad_value(83), (-0.495)), 84);
        }

        s.store_scale(283, 233, 0.5);

        s.store_mul(284, 75, 226);

        s.store_mul(285, 75, 223);

        s.store_div_from_scalar_ad(286, 1.0, A::offset(A::mul(s.ad_value(88), s.ad_value(236)), 1.0));

        s.store_div_from_scalar_ad(0, 4e-18, A::square(s.ad_value(18)));

        s.store_mul(89, 89, 0);

        s.store_mul(90, 90, 0);

        s.store_scale(0, 18, 500000000.0);

        s.store_scale_ad(277, A::add(A::offset(A::mul(s.ad_value(93), s.ad_value(220)), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(93), s.ad_value(220)), 1.0), A::offset(A::mul(s.ad_value(93), s.ad_value(220)), 1.0)), 0.01))), 0.5);

        s.store_mul_ad_lhs(91, A::mul(s.ad_value(208), s.ad_value(277)), 0);

        s.store_scale_ad(277, A::add(A::offset(A::mul(s.ad_value(94), s.ad_value(220)), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(94), s.ad_value(220)), 1.0), A::offset(A::mul(s.ad_value(94), s.ad_value(220)), 1.0)), 0.01))), 0.5);

        s.store_mul_ad_lhs(92, A::mul(s.ad_value(209), s.ad_value(277)), 0);

        s.store_mul_ad_rhs(113, 210, A::exp(A::mul(A::neg(s.ad_value(114)), s.ad_value(295))));

        s.store_mul_ad_rhs(288, 223, A::offset(A::mul(s.ad_value(99), s.ad_value(222)), 1.0));

        s.store_div_from_scalar(289, 1.0, 288);

        s.store_mul_ad_lhs(290, A::mul(A::scale(s.ad_value(252), (2.0 * 1.602176565e-19)), s.ad_value(229)), 289);

        s.store_add_ad_lhs(0, A::mul(A::scale(s.ad_value(102), p.p14), s.ad_value(220)), 256);

        s.store_sub_ad_lhs(100, A::offset(A::add(A::scale(A::add(A::add(s.ad_value(211), s.ad_value(237)), s.ad_value(243)), p.p14), s.ad_value(0)), p.p34), 249);

        s.store_add_ad_lhs(101, A::scale(A::add(A::add(s.ad_value(212), s.ad_value(237)), s.ad_value(244)), p.p14), 0);

        s.store_scale_ad(0, A::exp(A::mul(s.ad_value(111), s.ad_value(295))), p.p35);

        s.store_mul(110, 213, 0);

        s.store_mul(287, 116, 226);

        s.store_div_ad(291, A::scale(s.ad_value(118), (0.25 * 1.602176565e-19)), A::mul(s.ad_value(229), s.ad_value(226)));

        s.store_ln_ad(292, A::div(s.ad_value(118), s.ad_value(252)));

        s.store_mul_ad_lhs(293, A::scale(s.ad_value(119), 1.25e-6), 226);

        s.store_sqrt_ad(294, A::mul(A::mul(A::scale(s.ad_value(229), 28959234086.17689), s.ad_value(14)), A::offset(s.ad_value(533), 4e-10)));

        s.store_exp_ad(305, A::mul(s.ad_value(169), s.ad_value(295)));

        s.store_mul(168, 214, 305);

        s.store_scale(306, 217, (4.0 * 1.3806488e-23));

        s.store_mul(307, 171, 306);

        s.copy_ad(308, 307);

        s.store_scale(309, 172, (9.10938291e-31 * 1000000000000.0));

        s.v[679] = if (s.v[311] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[679] != 0.0) {
            s.store_div_from_scalar(316, 1.0, 311);
        }

        if (!(s.v[679] != 0.0)) {
            s.store_scalar(316, 0.0);
        }

        s.v[680] = if (s.v[318] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[680] != 0.0) {
            s.store_div_from_scalar(320, 1.0, 318);
        }

        if (!(s.v[680] != 0.0)) {
            s.store_scalar(320, 0.0);
        }

        s.v[681] = if (s.v[322] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[681] != 0.0) {
            s.store_div_from_scalar(324, 1.0, 322);
        }

        if (!(s.v[681] != 0.0)) {
            s.store_scalar(324, 0.0);
        }

        s.v[682] = if (s.v[326] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[682] != 0.0) {
            s.store_div_from_scalar(327, 1.0, 326);
        }

        if (!(s.v[682] != 0.0)) {
            s.store_scalar(327, 0.0);
        }

        s.v[785] = if (s.v[6] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[785] != 0.0) {
            s.store_ad(219, &A::voltage(ctx, &nodes, Some(4), None));
        }

        if (s.v[785] != 0.0) {
            s.store_add(217, 8, 219);
        }

        if (s.v[785] != 0.0) {
            s.store_square(218, 217);
        }

        if (s.v[785] != 0.0) {
            s.store_offset(220, 217, (-s.v[7]));
        }

        if (s.v[785] != 0.0) {
            s.store_scale(221, 217, 1.0 / (s.v[7]));
        }

        if (s.v[785] != 0.0) {
            s.store_div_from_scalar(222, s.v[7], 217);
        }

        if (s.v[785] != 0.0) {
            s.store_scale(223, 217, 8.617332384961e-5);
        }

        if (s.v[785] != 0.0) {
            s.store_div_from_scalar(224, 1.0, 223);
        }

        s.v[786] = if (p.p10 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[785] != 0.0) && (s.v[786] != 0.0)) {
            s.store_scale_ad(225, A::add(A::offset(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), 600.0), A::sqrt(A::offset(A::mul(A::offset(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), (-600.0)), A::offset(A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), (-600.0))), 0.01))), 0.5);
        }

        if ((s.v[785] != 0.0) && (!(s.v[786] != 0.0))) {
            s.store_scalar(225, 600.0);
        }

        if (s.v[785] != 0.0) {
            s.store_sub_from_scalar_ad(230, 1.17, A::div(A::scale(s.ad_value(218), 0.000473), A::offset(s.ad_value(217), 636.0)));
        }

        if (s.v[785] != 0.0) {
            s.store_sub_from_scalar_ad(231, 0.744, A::div(A::scale(s.ad_value(218), 0.0004774), A::offset(s.ad_value(217), 235.0)));
        }

        if (s.v[785] != 0.0) {
            s.store_mul_ad_lhs(232, A::add(A::sub(s.ad_value(231), s.ad_value(230)), A::scale(s.ad_value(228), (-0.4))), 15);
        }

        if (s.v[785] != 0.0) {
            s.store_add(233, 230, 232);
        }

        if (s.v[785] != 0.0) {
            s.store_mul_ad_lhs(234, A::scale(s.ad_value(233), 0.5), 224);
        }

        if (s.v[785] != 0.0) {
            s.store_sub_ad(237, A::scale(s.ad_value(15), 0.05), A::scale(s.ad_value(232), 0.5));
        }

        if (s.v[785] != 0.0) {
            s.store_sqrt_ad(0, A::scale(s.ad_value(217), 0.0033333333333));
        }

        if (s.v[785] != 0.0) {
            s.store_mul_ad_lhs(2, A::mul(A::scale(s.ad_value(0), 4.05e25), s.ad_value(0)), 0);
        }

        if (s.v[785] != 0.0) {
            s.store_mul(252, 2, 238);
        }

        if (s.v[785] != 0.0) {
            s.store_mul_ad_rhs(226, 223, A::offset(A::mul(s.ad_value(17), s.ad_value(222)), 1.0));
        }

        if (s.v[785] != 0.0) {
            s.store_div_from_scalar(227, 1.0, 226);
        }

        if (s.v[785] != 0.0) {
            s.store_mul_ad_lhs(236, A::scale(s.ad_value(233), 0.5), 227);
        }

        if (s.v[785] != 0.0) {
            s.store_mul_ad_lhs(253, A::mul(A::scale(s.ad_value(252), (2.0 * 1.602176565e-19)), s.ad_value(229)), 227);
        }

        if (s.v[785] != 0.0) {
            s.store_offset_ad(254, A::ln(A::div(A::square(s.ad_value(245)), s.ad_value(253))), (-0.6931471805599));
        }

        if (s.v[785] != 0.0) {
            s.store_mul_ad_lhs(255, A::div(A::mul(A::scale(s.ad_value(29), (0.5 * 1.602176565e-19)), s.ad_value(14)), A::add(s.ad_value(241), s.ad_value(242))), 227);
        }

    }

    pub(super) fn stamp_transient_block_6(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[785] != 0.0) {
            s.store_mul(0, 34, 220);
        }

        if (s.v[785] != 0.0) {
            s.store_add(31, 187, 0);
        }

        if (s.v[785] != 0.0) {
            s.store_add(32, 188, 0);
        }

        if (s.v[785] != 0.0) {
            s.store_mul(329, 35, 227);
        }

        if (s.v[785] != 0.0) {
            s.store_add(140, 189, 0);
        }

        if (s.v[785] != 0.0) {
            s.store_add(141, 190, 0);
        }

        s.v[787] = if (p.p9 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[785] != 0.0) && (s.v[787] != 0.0)) {
            s.store_mul_ad_rhs(249, 223, A::add(A::ln(A::div(s.ad_value(24), s.ad_value(251))), s.ad_value(235)));
        }

        s.v[788] = if (p.p10 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[785] != 0.0) && (s.v[788] != 0.0)) {
            s.store_scale_ad(257, A::add(A::offset(A::div_from_scalar(2970.0, s.ad_value(8)), 15.0), A::sqrt(A::offset(A::mul(A::sub_from_scalar(15.0, A::div_from_scalar(2970.0, s.ad_value(8))), A::sub_from_scalar(15.0, A::div_from_scalar(2970.0, s.ad_value(8)))), 1e-6))), 0.5);
        }

        if (s.v[785] != 0.0) {
            s.store_scalar(258, 0.0);
        }

        s.v[789] = if (p.p13 > 0.0) { 1.0 } else { 0.0 };

        s.v[790] = if (p.p14 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[785] != 0.0) && (s.v[789] != 0.0)) && (s.v[790] != 0.0)) {
            s.store_scale_ad(258, A::exp(A::scale(A::ln(A::mul(s.ad_value(226), s.ad_value(259))), (-0.3333333333333))), ((0.4 * p.p13) * 1.27520989));
        }

        if (((s.v[785] != 0.0) && (s.v[789] != 0.0)) && (!(s.v[790] != 0.0))) {
            s.store_scale_ad(258, A::exp(A::scale(A::ln(A::mul(s.ad_value(226), s.ad_value(259))), (-0.3333333333333))), ((0.4 * p.p13) * 1.5412087));
        }

        if (s.v[785] != 0.0) {
            s.store_add_ad_lhs(0, A::mul(A::scale(s.ad_value(23), p.p14), s.ad_value(220)), 256);
        }

        if (s.v[785] != 0.0) {
            s.store_sub_ad_lhs(2, A::offset(s.ad_value(0), p.p34), 249);
        }

        if (s.v[785] != 0.0) {
            s.store_add_ad_lhs(21, A::scale(A::add(A::add(s.ad_value(183), s.ad_value(237)), s.ad_value(243)), p.p14), 2);
        }

        if (s.v[785] != 0.0) {
            s.store_add_ad_lhs(22, A::scale(A::add(A::add(s.ad_value(184), s.ad_value(237)), s.ad_value(244)), p.p14), 0);
        }

        if (s.v[785] != 0.0) {
            s.store_add_ad_lhs(130, A::scale(A::add(A::add(s.ad_value(185), s.ad_value(237)), s.ad_value(243)), p.p14), 2);
        }

        if (s.v[785] != 0.0) {
            s.store_add_ad_lhs(131, A::scale(A::add(A::add(s.ad_value(186), s.ad_value(237)), s.ad_value(244)), p.p14), 0);
        }

        if (s.v[785] != 0.0) {
            s.store_ln(295, 222);
        }

        if (s.v[785] != 0.0) {
            s.store_scale_ad(296, A::exp(A::mul(s.ad_value(40), s.ad_value(295))), p.p35);
        }

        if (s.v[785] != 0.0) {
            s.store_mul(38, 191, 296);
        }

        if (s.v[785] != 0.0) {
            s.store_mul(39, 192, 296);
        }

        if (s.v[785] != 0.0) {
            s.store_exp_ad(297, A::mul(s.ad_value(48), s.ad_value(295)));
        }

        if (s.v[785] != 0.0) {
            s.store_mul(46, 193, 297);
        }

        if (s.v[785] != 0.0) {
            s.store_exp_ad(298, A::mul(s.ad_value(49), s.ad_value(295)));
        }

        if (s.v[785] != 0.0) {
            s.store_mul(47, 194, 298);
        }

        if (s.v[785] != 0.0) {
            s.store_exp_ad(299, A::mul(s.ad_value(43), s.ad_value(295)));
        }

        if (s.v[785] != 0.0) {
            s.store_mul(33, 195, 299);
        }

        if (s.v[785] != 0.0) {
            s.store_exp_ad(300, A::mul(s.ad_value(45), s.ad_value(295)));
        }

        if (s.v[785] != 0.0) {
            s.store_mul(44, 196, 300);
        }

        if (s.v[785] != 0.0) {
            s.store_exp_ad(301, A::mul(s.ad_value(52), s.ad_value(295)));
        }

        if (s.v[785] != 0.0) {
            s.store_mul(50, 197, 301);
        }

        if (s.v[785] != 0.0) {
            s.store_div_ad_lhs(0, A::scale(s.ad_value(226), 1e-8), 14);
        }

        if (s.v[785] != 0.0) {
            s.store_mul(267, 0, 46);
        }

        if (s.v[785] != 0.0) {
            s.store_exp_ad(302, A::mul(s.ad_value(55), s.ad_value(295)));
        }

        if (s.v[785] != 0.0) {
            s.store_mul(53, 198, 302);
        }

        if (s.v[785] != 0.0) {
            s.store_mul_ad_lhs(272, A::scale(s.ad_value(53), 2.0), 226);
        }

        if (s.v[785] != 0.0) {
            s.store_exp_ad(303, A::mul(s.ad_value(60), s.ad_value(295)));
        }

        if (s.v[785] != 0.0) {
            s.store_mul_ad_lhs(59, A::mul(s.ad_value(199), s.ad_value(303)), 296);
        }

        if (s.v[785] != 0.0) {
            s.store_mul(273, 59, 226);
        }

        if (s.v[785] != 0.0) {
            s.store_mul_ad_lhs(147, A::mul(s.ad_value(200), s.ad_value(303)), 296);
        }

        if (s.v[785] != 0.0) {
            s.store_mul(274, 147, 226);
        }

        if (s.v[785] != 0.0) {
            s.store_mul(275, 64, 227);
        }

        if (s.v[785] != 0.0) {
            s.store_exp_ad(304, A::mul(A::neg(s.ad_value(76)), s.ad_value(295)));
        }

        if (s.v[785] != 0.0) {
            s.store_mul(68, 201, 304);
        }

        if (s.v[785] != 0.0) {
            s.store_mul(69, 202, 304);
        }

        if (s.v[785] != 0.0) {
            s.store_mul(70, 203, 304);
        }

        if (s.v[785] != 0.0) {
            s.store_mul(71, 204, 304);
        }

        if (s.v[785] != 0.0) {
            s.store_mul(72, 205, 304);
        }

        if (s.v[785] != 0.0) {
            s.store_exp_ad(304, A::mul(A::neg(s.ad_value(77)), s.ad_value(295)));
        }

        if (s.v[785] != 0.0) {
            s.store_mul(73, 206, 304);
        }

        if (s.v[785] != 0.0) {
            s.store_mul(74, 207, 304);
        }

        if (s.v[785] != 0.0) {
            s.store_scale(283, 233, 0.5);
        }

        if (s.v[785] != 0.0) {
            s.store_mul(284, 75, 226);
        }

        if (s.v[785] != 0.0) {
            s.store_mul(285, 75, 223);
        }

        if (s.v[785] != 0.0) {
            s.store_div_from_scalar_ad(286, 1.0, A::offset(A::mul(s.ad_value(88), s.ad_value(236)), 1.0));
        }

        if (s.v[785] != 0.0) {
            s.store_scale(0, 18, 500000000.0);
        }

        if (s.v[785] != 0.0) {
            s.store_scale_ad(277, A::add(A::offset(A::mul(s.ad_value(93), s.ad_value(220)), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(93), s.ad_value(220)), 1.0), A::offset(A::mul(s.ad_value(93), s.ad_value(220)), 1.0)), 0.01))), 0.5);
        }

        if (s.v[785] != 0.0) {
            s.store_mul_ad_lhs(91, A::mul(s.ad_value(208), s.ad_value(277)), 0);
        }

        if (s.v[785] != 0.0) {
            s.store_scale_ad(277, A::add(A::offset(A::mul(s.ad_value(94), s.ad_value(220)), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(94), s.ad_value(220)), 1.0), A::offset(A::mul(s.ad_value(94), s.ad_value(220)), 1.0)), 0.01))), 0.5);
        }

        if (s.v[785] != 0.0) {
            s.store_mul_ad_lhs(92, A::mul(s.ad_value(209), s.ad_value(277)), 0);
        }

        if (s.v[785] != 0.0) {
            s.store_mul_ad_rhs(113, 210, A::exp(A::mul(A::neg(s.ad_value(114)), s.ad_value(295))));
        }

        if (s.v[785] != 0.0) {
            s.store_mul(287, 116, 226);
        }

        if (s.v[785] != 0.0) {
            s.store_div_ad(291, A::scale(s.ad_value(118), (0.25 * 1.602176565e-19)), A::mul(s.ad_value(229), s.ad_value(226)));
        }

        if (s.v[785] != 0.0) {
            s.store_ln_ad(292, A::div(s.ad_value(118), s.ad_value(252)));
        }

        if (s.v[785] != 0.0) {
            s.store_mul_ad_lhs(293, A::scale(s.ad_value(119), 1.25e-6), 226);
        }

        if (s.v[785] != 0.0) {
            s.store_exp_ad(305, A::mul(s.ad_value(169), s.ad_value(295)));
        }

        if (s.v[785] != 0.0) {
            s.store_mul(168, 214, 305);
        }

        if (s.v[785] != 0.0) {
            s.store_scale(306, 217, (4.0 * 1.3806488e-23));
        }

        if (s.v[785] != 0.0) {
            s.store_mul(307, 171, 306);
        }

        s.v[791] = if (p.p14 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[791] != 0.0) {
            s.store_ad(330, &A::voltage(ctx, &nodes, Some(9), Some(6)));
        }

        if (s.v[791] != 0.0) {
            s.store_ad(702, &A::voltage(ctx, &nodes, Some(7), Some(6)));
        }

        if (s.v[791] != 0.0) {
            s.store_ad(331, &A::voltage(ctx, &nodes, Some(6), Some(8)));
        }

        if (!(s.v[791] != 0.0)) {
            s.store_ad(330, &A::neg(A::voltage(ctx, &nodes, Some(9), Some(6))));
        }

        if (!(s.v[791] != 0.0)) {
            s.store_ad(702, &A::neg(A::voltage(ctx, &nodes, Some(7), Some(6))));
        }

        if (!(s.v[791] != 0.0)) {
            s.store_ad(331, &A::neg(A::voltage(ctx, &nodes, Some(6), Some(8))));
        }

        s.store_neg(703, 702);

        s.store_add(332, 330, 703);

        s.store_add(333, 702, 331);

        s.v[792] = if (s.v[702] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[792] != 0.0) {
            s.store_scalar(334, (-1.0));
        }

        if (s.v[792] != 0.0) {
            s.copy_ad(336, 703);
        }

        if (s.v[792] != 0.0) {
            s.copy_ad(335, 332);
        }

        if (s.v[792] != 0.0) {
            s.copy_ad(337, 333);
        }

        if (!(s.v[792] != 0.0)) {
            s.store_scalar(334, 1.0);
        }

        if (!(s.v[792] != 0.0)) {
            s.copy_ad(336, 702);
        }

        if (!(s.v[792] != 0.0)) {
            s.copy_ad(335, 330);
        }

        if (!(s.v[792] != 0.0)) {
            s.copy_ad(337, 331);
        }

        s.store_add(338, 335, 337);

        s.store_mul(339, 336, 227);

        s.store_mul_ad_lhs(340, A::offset(A::sqrt(A::offset(A::square(s.ad_value(336)), 0.01)), (-0.1)), 227);

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

        s.store_sub_ad_lhs(878, A::sub(A::mul(A::sub(s.ad_value(335), s.ad_value(869)), s.ad_value(227)), s.ad_value(341)), 234);

        s.store_sub_ad_lhs(879, A::mul(A::sub(A::neg(s.ad_value(337)), s.ad_value(870)), s.ad_value(227)), 341);

        s.store_sub(880, 879, 234);

        s.v[1059] = if (p.p2 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1059] != 0.0) {
            s.store_scale(0, 16, p.p14);
        }

        if (s.v[1059] != 0.0) {
            s.store_div_ad(881, A::offset(s.ad_value(246), 1.0), A::offset(s.ad_value(247), 1.0));
        }

        if (s.v[1059] != 0.0) {
            s.store_ln(882, 881);
        }

        s.v[1060] = if (s.v[882] > 1e-8) { 1.0 } else { 0.0 };

        if ((s.v[1059] != 0.0) && (s.v[1060] != 0.0)) {
            s.store_div_ad(883, A::mul(A::scale(s.ad_value(882), 2.0), A::offset(s.ad_value(881), 1.0)), A::offset(s.ad_value(881), (-1.0)));
        }

        if ((s.v[1059] != 0.0) && (!(s.v[1060] != 0.0))) {
            s.store_scaled_offset(883, 882, 2.0, 2.0);
        }

        if (s.v[1059] != 0.0) {
            s.store_div_ad_rhs(884, 253, A::square(s.ad_value(245)));
        }

        if (s.v[1059] != 0.0) {
            s.store_div_from_scalar(885, 1.0, 246);
        }

        if (s.v[1059] != 0.0) {
            s.store_div_from_scalar(886, 1.0, 247);
        }

        if (s.v[1059] != 0.0) {
            s.store_div_from_scalar_ad(913, 1.0, A::add(A::offset(s.ad_value(885), 1.0), s.ad_value(886)));
        }

        if (s.v[1059] != 0.0) {
            s.store_mul_ad_rhs(914, 913, A::sub(s.ad_value(878), s.ad_value(880)));
        }

        if (s.v[1059] != 0.0) {
            s.store_sub_ad_rhs(887, 878, A::mul(s.ad_value(914), s.ad_value(885)));
        }

        if (s.v[1059] != 0.0) {
            s.store_add_ad_rhs(888, 880, A::mul(s.ad_value(914), s.ad_value(886)));
        }

        if (s.v[1059] != 0.0) {
            s.store_div_from_scalar_ad(793, 1.0, A::offset(s.ad_value(246), 1.0));
        }

        if (s.v[1059] != 0.0) {
            s.store_div_from_scalar_ad(794, 1.0, A::offset(s.ad_value(247), 1.0));
        }

        if (s.v[1059] != 0.0) {
            s.store_offset_ad(796, A::ln(A::div(A::mul(A::add(s.ad_value(246), A::mul(s.ad_value(247), s.ad_value(794))), s.ad_value(883)), s.ad_value(884))), 1.5);
        }

        if (s.v[1059] != 0.0) {
            s.store_offset_ad(797, A::ln(A::div(A::mul(A::add(s.ad_value(247), A::mul(s.ad_value(246), s.ad_value(793))), s.ad_value(883)), s.ad_value(884))), 1.5);
        }

        s.v[1061] = if (((s.v[796] - s.v[887]) / 1.5) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1059] != 0.0) && (s.v[1061] != 0.0)) {
            s.store_ln_ad(795, A::offset(A::exp(A::scale(A::sub(s.ad_value(796), s.ad_value(887)), 0.6666666666666666)), 1.0));
        }

        if ((s.v[1059] != 0.0) && (!(s.v[1061] != 0.0))) {
            s.store_scaled_sub(795, 796, 887, 0.6666666666666666);
        }

        if (s.v[1059] != 0.0) {
            s.store_sub_ad_rhs(800, 796, A::scale(s.ad_value(795), 1.5));
        }

        if (s.v[1059] != 0.0) {
            s.store_mul_ad_lhs(799, A::add(A::mul(s.ad_value(247), s.ad_value(880)), s.ad_value(800)), 794);
        }

        s.v[1062] = if (((s.v[797] - s.v[799]) / 1.5) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1059] != 0.0) && (s.v[1062] != 0.0)) {
            s.store_ln_ad(795, A::offset(A::exp(A::scale(A::sub(s.ad_value(797), s.ad_value(799)), 0.6666666666666666)), 1.0));
        }

        if ((s.v[1059] != 0.0) && (!(s.v[1062] != 0.0))) {
            s.store_scaled_sub(795, 797, 799, 0.6666666666666666);
        }

        if (s.v[1059] != 0.0) {
            s.store_sub_ad_rhs(1, 797, A::scale(s.ad_value(795), 1.5));
        }

        if (s.v[1059] != 0.0) {
            s.store_mul(2, 0, 1);
        }

        if (s.v[1059] != 0.0) {
            s.store_mul(3, 0, 880);
        }

        if (s.v[1059] != 0.0) {
            s.store_sub(845, 2, 3);
        }

        s.v[1063] = if ((((-s.v[266])) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1059] != 0.0) && (s.v[1063] != 0.0)) {
            s.store_exp_ad(846, A::neg(s.ad_value(266)));
        }

        s.v[1064] = if ((-s.v[266]) < (-80.0)) { 1.0 } else { 0.0 };

        if (((s.v[1059] != 0.0) && (!(s.v[1063] != 0.0))) && (s.v[1064] != 0.0)) {
            s.store_div_from_scalar_ad(846, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::neg(s.ad_value(266))), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::neg(s.ad_value(266))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(266))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1063] != 0.0))) && (!(s.v[1064] != 0.0))) {
            s.store_scale_ad(846, A::offset(A::mul(A::offset(A::neg(s.ad_value(266)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(266)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(266)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        s.v[1065] = if (((s.v[845]) as f64).abs() <= s.v[265]) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_7(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[1059] != 0.0) && (s.v[1065] != 0.0)) {
            s.store_scale_ad(843, A::square(s.ad_value(264)), (0.1666666666667 * 0.707106781186545));
        }

        if ((s.v[1059] != 0.0) && (s.v[1065] != 0.0)) {
            s.store_mul_ad(4, A::mul(s.ad_value(845), s.ad_value(264)), A::offset(A::mul(A::mul(A::mul(s.ad_value(845), A::sub_from_scalar(1.0, s.ad_value(846))), s.ad_value(260)), s.ad_value(843)), 1.0));
        }

        s.v[1066] = if (s.v[845] < (-s.v[265])) { 1.0 } else { 0.0 };

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) {
            s.store_neg(847, 845);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) {
            s.store_scaled_mul(848, 847, 264, 1.25);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) {
            s.store_scale_ad(849, A::sub(A::offset(s.ad_value(848), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(848), (-6.0)), A::offset(s.ad_value(848), (-6.0))), 64.0))), 0.5);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) {
            s.store_sub(842, 847, 849);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) {
            s.store_add_ad(850, A::square(s.ad_value(842)), A::mul(s.ad_value(261), A::offset(s.ad_value(849), 1.0)));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) {
            s.store_sub_ad_lhs(852, A::scale(s.ad_value(842), 2.0), 261);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) {
            s.store_sub_ad_lhs(853, A::ln(A::mul(s.ad_value(850), s.ad_value(262))), 849);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) {
            s.store_add(840, 850, 852);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) {
            s.store_add_ad(841, A::square(s.ad_value(840)), A::mul(s.ad_value(853), A::sub(A::mul(A::scale(s.ad_value(852), 0.5), s.ad_value(852)), s.ad_value(850))));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) {
            s.store_add_ad_rhs(854, 849, A::div(A::mul(A::mul(s.ad_value(850), s.ad_value(840)), s.ad_value(853)), A::add(s.ad_value(841), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(840), s.ad_value(841)), s.ad_value(853)), s.ad_value(853)), s.ad_value(852)), A::sub(A::scale(A::square(s.ad_value(852)), 0.3333333333333), s.ad_value(850))))));
        }

        s.v[1067] = if (s.v[854] < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) && (s.v[1067] != 0.0)) {
            s.store_exp(855, 854);
        }

        if ((((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) && (!(s.v[1067] != 0.0))) {
            s.store_scale_ad(855, A::offset(A::mul(A::offset(s.ad_value(854), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(854), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(854), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) {
            s.store_div_from_scalar(856, 1.0, 855);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) {
            s.store_div_from_scalar_ad(842, 1.0, A::offset(A::square(s.ad_value(854)), 2.0));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) {
            s.store_mul_ad_lhs(857, A::square(s.ad_value(854)), 842);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) {
            s.store_scale_ad(858, A::mul(A::mul(s.ad_value(854), s.ad_value(842)), s.ad_value(842)), 4.0);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) {
            s.store_mul_ad_lhs(859, A::mul(A::sub(A::scale(s.ad_value(842), 8.0), A::scale(s.ad_value(857), 12.0)), s.ad_value(842)), 842);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) {
            s.store_sub(842, 847, 854);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) {
            s.store_mul(843, 846, 856);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) {
            s.store_add_ad(860, A::scale(s.ad_value(842), 2.0), A::mul(s.ad_value(261), A::add(A::sub(A::offset(s.ad_value(855), (-1.0)), s.ad_value(843)), A::mul(s.ad_value(846), A::sub_from_scalar(1.0, s.ad_value(858))))));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) {
            s.store_sub_ad(861, A::square(s.ad_value(842)), A::mul(s.ad_value(261), A::add(A::add(A::offset(A::sub(s.ad_value(855), s.ad_value(854)), (-1.0)), s.ad_value(843)), A::mul(s.ad_value(846), A::sub(A::offset(s.ad_value(854), (-1.0)), s.ad_value(857))))));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) {
            s.store_sub_from_scalar_ad(842, 2.0, A::mul(s.ad_value(261), A::sub(A::add(s.ad_value(855), s.ad_value(843)), A::mul(s.ad_value(846), s.ad_value(859)))));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) {
            s.store_sub_ad(842, A::square(s.ad_value(860)), A::scale(A::mul(s.ad_value(861), s.ad_value(842)), 2.0));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (s.v[1066] != 0.0)) {
            s.store_sub_ad(4, A::neg(s.ad_value(854)), A::scale(A::div(s.ad_value(861), A::add(s.ad_value(860), A::sqrt(s.ad_value(842)))), 2.0));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_div_from_scalar_ad(862, 1.0, A::offset(A::scale(s.ad_value(260), 0.732464877560822), 1.25));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_mul_ad_lhs(863, A::offset(A::mul(A::scale(s.ad_value(263), 1.25), s.ad_value(862)), (-1.0)), 862);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_mul_ad(864, A::mul(s.ad_value(845), s.ad_value(264)), A::offset(A::mul(s.ad_value(863), s.ad_value(845)), 1.0));
        }

        s.v[1068] = if ((-s.v[864]) > (-80.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) && (s.v[1068] != 0.0)) {
            s.store_exp_ad(842, A::neg(s.ad_value(864)));
        }

        if ((((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) && (!(s.v[1068] != 0.0))) {
            s.store_div_from_scalar_ad(842, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::neg(s.ad_value(864))), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::neg(s.ad_value(864))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(864))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_sub_from_scalar(865, 1.0, 842);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_sub_ad(866, A::add(s.ad_value(845), A::scale(s.ad_value(261), 0.5)), A::mul(s.ad_value(260), A::sqrt(A::sub(A::add(s.ad_value(845), A::scale(s.ad_value(261), 0.25)), s.ad_value(865)))));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_offset(867, 266, 3.0);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_sub_ad(849, A::scale(A::sub(A::add(s.ad_value(866), s.ad_value(867)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(866), s.ad_value(867)), A::sub(s.ad_value(866), s.ad_value(867))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(867), A::sqrt(A::offset(A::square(s.ad_value(867)), 5.0))), 0.5));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_sub(842, 845, 849);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_exp_ad(843, A::neg(s.ad_value(849)));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_div_from_scalar_ad(844, 1.0, A::offset(A::square(s.ad_value(849)), 2.0));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_mul_ad_lhs(857, A::square(s.ad_value(849)), 844);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_scale_ad(858, A::mul(A::mul(s.ad_value(849), s.ad_value(844)), s.ad_value(844)), 4.0);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_mul_ad_lhs(859, A::mul(A::sub(A::scale(s.ad_value(844), 8.0), A::scale(s.ad_value(857), 12.0)), s.ad_value(844)), 844);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_max_from_scalar_ad(850, 1e-40, A::sub(A::square(s.ad_value(842)), A::mul(s.ad_value(261), A::sub(A::offset(A::add(s.ad_value(843), s.ad_value(849)), (-1.0)), A::mul(s.ad_value(846), A::add(A::offset(s.ad_value(849), 1.0), s.ad_value(857)))))));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_sub_from_scalar_ad(851, 1.0, A::scale(A::mul(s.ad_value(261), A::sub(s.ad_value(843), A::mul(s.ad_value(846), s.ad_value(859)))), 0.5));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_add_ad(852, A::scale(s.ad_value(842), 2.0), A::mul(s.ad_value(261), A::sub(A::sub_from_scalar(1.0, s.ad_value(843)), A::mul(s.ad_value(846), A::offset(s.ad_value(858), 1.0)))));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_add_ad(853, A::sub(s.ad_value(266), s.ad_value(849)), A::ln(A::div(s.ad_value(850), s.ad_value(261))));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_add(840, 850, 852);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_add_ad(841, A::square(s.ad_value(840)), A::mul(s.ad_value(853), A::sub(A::mul(A::scale(s.ad_value(852), 0.5), s.ad_value(852)), A::mul(s.ad_value(850), s.ad_value(851)))));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            let assign10590_ad_e9839: A = A::add(s.ad_value(849), A::div(A::mul(A::mul(s.ad_value(850), s.ad_value(840)), s.ad_value(853)), A::add(s.ad_value(841), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(840), s.ad_value(841)), s.ad_value(853)), s.ad_value(853)), s.ad_value(852)), A::sub(A::scale(A::square(s.ad_value(852)), 0.3333333333333), A::mul(s.ad_value(850), s.ad_value(851)))))));
            s.store_ad(868, &assign10590_ad_e9839);
        }

        s.v[1069] = if (s.v[868] < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) && (s.v[1069] != 0.0)) {
            s.store_exp(855, 868);
        }

        if ((((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) && (s.v[1069] != 0.0)) {
            s.store_div_from_scalar(856, 1.0, 855);
        }

        if ((((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) && (s.v[1069] != 0.0)) {
            s.store_mul(855, 846, 855);
        }

        s.v[1070] = if (s.v[868] > (s.v[266] - 80.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) && (!(s.v[1069] != 0.0))) && (s.v[1070] != 0.0)) {
            s.store_exp_ad(855, A::sub(s.ad_value(868), s.ad_value(266)));
        }

        if (((((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) && (!(s.v[1069] != 0.0))) && (s.v[1070] != 0.0)) {
            s.store_div(856, 846, 855);
        }

        if (((((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) && (!(s.v[1069] != 0.0))) && (!(s.v[1070] != 0.0))) {
            s.store_div_from_scalar_ad(855, 1.80485e-35, A::offset(A::mul(A::offset(A::sub(s.ad_value(266), s.ad_value(868)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(266), s.ad_value(868)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(266), s.ad_value(868)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) && (!(s.v[1069] != 0.0))) && (!(s.v[1070] != 0.0))) {
            s.store_div_from_scalar_ad(856, 1.80485e-35, A::offset(A::mul(A::offset(s.ad_value(868), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(868), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(868), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_div_from_scalar_ad(842, 1.0, A::offset(A::square(s.ad_value(868)), 2.0));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_mul_ad_lhs(857, A::square(s.ad_value(868)), 842);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_scale_ad(858, A::mul(A::mul(s.ad_value(868), s.ad_value(842)), s.ad_value(842)), 4.0);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_mul_ad_lhs(859, A::mul(A::sub(A::scale(s.ad_value(842), 8.0), A::scale(s.ad_value(857), 12.0)), s.ad_value(842)), 842);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_sub(842, 845, 868);
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_add_ad(860, A::scale(s.ad_value(842), 2.0), A::mul(s.ad_value(261), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(856)), s.ad_value(855)), A::mul(s.ad_value(846), A::offset(s.ad_value(858), 1.0)))));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_sub_ad(861, A::square(s.ad_value(842)), A::mul(s.ad_value(261), A::sub(A::add(A::offset(A::add(s.ad_value(856), s.ad_value(868)), (-1.0)), s.ad_value(855)), A::mul(s.ad_value(846), A::add(A::offset(s.ad_value(868), 1.0), s.ad_value(857))))));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_sub_from_scalar_ad(842, 2.0, A::mul(s.ad_value(261), A::sub(A::add(s.ad_value(856), s.ad_value(855)), A::mul(s.ad_value(846), s.ad_value(859)))));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_sub_ad(842, A::square(s.ad_value(860)), A::scale(A::mul(s.ad_value(861), s.ad_value(842)), 2.0));
        }

        if (((s.v[1059] != 0.0) && (!(s.v[1065] != 0.0))) && (!(s.v[1066] != 0.0))) {
            s.store_add_ad_rhs(4, 868, A::scale(A::div(s.ad_value(861), A::add(s.ad_value(860), A::sqrt(s.ad_value(842)))), 2.0));
        }

        if (s.v[1059] != 0.0) {
            s.store_mul_ad_rhs(889, 0, A::add(s.ad_value(4), s.ad_value(3)));
        }

        if (!(s.v[1059] != 0.0)) {
            s.copy_ad(889, 880);
        }

        s.store_mul_ad_rhs(0, 248, A::sub(s.ad_value(878), s.ad_value(889)));

        s.v[1071] = if (p.p13 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1071] != 0.0) {
            s.store_scale_ad(890, A::add(A::add(s.ad_value(0), s.ad_value(257)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(0), s.ad_value(257)), A::sub(s.ad_value(0), s.ad_value(257))), A::square(s.ad_value(257))))), 0.5);
        }

        if (s.v[1071] != 0.0) {
            s.store_scale_ad(891, A::add(A::sub(s.ad_value(257), s.ad_value(0)), A::sqrt(A::add(A::mul(A::sub(A::neg(s.ad_value(0)), s.ad_value(257)), A::sub(A::neg(s.ad_value(0)), s.ad_value(257))), A::square(s.ad_value(257))))), 0.5);
        }

        if (s.v[1071] != 0.0) {
            s.store_mul_ad_rhs(2, 258, A::exp(A::scale(A::ln(s.ad_value(890)), (-0.3333333333333))));
        }

        if (s.v[1071] != 0.0) {
            s.store_mul_ad_rhs(3, 258, A::exp(A::scale(A::ln(s.ad_value(891)), (-0.3333333333333))));
        }

        if (s.v[1071] != 0.0) {
            s.store_sub_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
        }

        if (s.v[1071] != 0.0) {
            s.store_div(898, 245, 4);
        }

        if (s.v[1071] != 0.0) {
            s.store_offset_ad(892, A::mul(s.ad_value(246), s.ad_value(2)), 1.0);
        }

        if (s.v[1071] != 0.0) {
            s.store_offset_ad(893, A::mul(s.ad_value(247), s.ad_value(3)), 1.0);
        }

        if (s.v[1071] != 0.0) {
            s.store_div_ad_lhs(894, A::mul(s.ad_value(246), s.ad_value(4)), 892);
        }

        if (s.v[1071] != 0.0) {
            s.store_div_ad_lhs(895, A::mul(s.ad_value(247), s.ad_value(4)), 893);
        }

        if (s.v[1071] != 0.0) {
            s.store_div_from_scalar_ad(896, 1.0, A::add(A::offset(A::div_from_scalar(1.0, s.ad_value(894)), 1.0), A::div_from_scalar(1.0, s.ad_value(895))));
        }

        if (s.v[1071] != 0.0) {
            s.store_offset_ad(892, A::mul(s.ad_value(894), s.ad_value(2)), 1.0);
        }

        if (s.v[1071] != 0.0) {
            s.store_offset_ad(893, A::mul(s.ad_value(895), s.ad_value(3)), 1.0);
        }

        if (!(s.v[1071] != 0.0)) {
            s.copy_ad(898, 245);
        }

        if (!(s.v[1071] != 0.0)) {
            s.copy_ad(894, 246);
        }

        if (!(s.v[1071] != 0.0)) {
            s.copy_ad(895, 247);
        }

        if (!(s.v[1071] != 0.0)) {
            s.copy_ad(896, 248);
        }

        if (!(s.v[1071] != 0.0)) {
            s.store_scalar(892, 1.0);
        }

        if (!(s.v[1071] != 0.0)) {
            s.store_scalar(893, 1.0);
        }

        s.store_mul_ad_rhs(897, 896, A::sub(s.ad_value(878), s.ad_value(889)));

        s.v[1072] = if (s.v[897] > 0.0) { 1.0 } else { 0.0 };

        s.v[1073] = if ((-s.v[897]) < 80.0) { 1.0 } else { 0.0 };

        if ((s.v[1072] != 0.0) && (s.v[1073] != 0.0)) {
            s.store_ln_ad(0, A::offset(A::exp(A::neg(s.ad_value(897))), 1.0));
        }

        if ((s.v[1072] != 0.0) && (!(s.v[1073] != 0.0))) {
            s.store_neg(0, 897);
        }

        if (s.v[1072] != 0.0) {
            s.store_offset_ad(899, A::add(A::sub(s.ad_value(878), A::div(s.ad_value(897), s.ad_value(894))), s.ad_value(0)), (-0.6931471805599));
        }

        s.v[1074] = if (s.v[897] < 80.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1072] != 0.0)) && (s.v[1074] != 0.0)) {
            s.store_ln_ad(0, A::offset(A::exp(s.ad_value(897)), 1.0));
        }

        if ((!(s.v[1072] != 0.0)) && (!(s.v[1074] != 0.0))) {
            s.copy_ad(0, 897);
        }

        if (!(s.v[1072] != 0.0)) {
            s.store_offset_ad(899, A::add(A::add(s.ad_value(889), A::div(s.ad_value(897), s.ad_value(895))), s.ad_value(0)), (-0.6931471805599));
        }

        s.store_scale_ad(900, A::sub(A::add(s.ad_value(899), s.ad_value(254)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(899), s.ad_value(254)), A::sub(s.ad_value(899), s.ad_value(254))), 4.0))), 0.5);

        s.store_offset_ad(901, A::sqrt(A::offset(A::div(A::scale(A::sub(s.ad_value(254), s.ad_value(900)), 2.0), s.ad_value(255)), 1.0)), (-1.0));

        s.store_add_ad_rhs(902, 900, A::mul(s.ad_value(255), s.ad_value(901)));

        s.store_scale_ad(0, A::add(A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(879)), 1.0), 0.5), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(879)), 1.0), (-0.5)), A::offset(A::offset(A::mul(s.ad_value(30), s.ad_value(879)), 1.0), (-0.5))), 0.01))), 0.5);

        s.store_div_from_scalar_ad(903, 1.0, A::offset(A::mul(s.ad_value(871), s.ad_value(0)), 1.0));

        s.store_div_from_scalar_ad(904, 1.0, A::offset(A::mul(s.ad_value(872), s.ad_value(0)), 1.0));

        s.store_mul_ad(0, A::mul(A::mul(A::scale(s.ad_value(329), 2.0), A::offset(A::sqrt(A::offset(A::div(s.ad_value(340), s.ad_value(329)), 1.0)), (-1.0))), A::offset(A::mul(s.ad_value(36), s.ad_value(901)), 1.0)), A::offset(A::mul(s.ad_value(37), s.ad_value(879)), 1.0));

        s.store_mul(905, 873, 0);

        s.store_mul(906, 874, 0);

        s.store_add_ad_lhs(907, A::add(A::mul(A::add(A::sub(s.ad_value(878), s.ad_value(902)), s.ad_value(905)), s.ad_value(903)), s.ad_value(902)), 341);

        s.store_add_ad_lhs(908, A::add(A::mul(A::add(A::sub(s.ad_value(889), s.ad_value(902)), s.ad_value(906)), s.ad_value(904)), s.ad_value(902)), 341);

        let assign11230_ad_e10630: A = A::sub(A::add(A::add(s.ad_value(908), A::mul(s.ad_value(25), A::sub(s.ad_value(907), s.ad_value(908)))), s.ad_value(225)), A::sqrt(A::offset(A::mul(A::sub(A::add(s.ad_value(908), A::mul(s.ad_value(25), A::sub(s.ad_value(907), s.ad_value(908)))), s.ad_value(225)), A::sub(A::add(s.ad_value(908), A::mul(s.ad_value(25), A::sub(s.ad_value(907), s.ad_value(908)))), s.ad_value(225))), 0.01)));
        s.store_scale_ad(909, assign11230_ad_e10630, 0.5);

        let assign11240_ad_e10664: A = A::sub(A::add(A::add(s.ad_value(907), A::mul(s.ad_value(26), A::sub(s.ad_value(908), s.ad_value(907)))), s.ad_value(225)), A::sqrt(A::offset(A::mul(A::sub(A::add(s.ad_value(907), A::mul(s.ad_value(26), A::sub(s.ad_value(908), s.ad_value(907)))), s.ad_value(225)), A::sub(A::add(s.ad_value(907), A::mul(s.ad_value(26), A::sub(s.ad_value(908), s.ad_value(907)))), s.ad_value(225))), 0.01)));
        s.store_scale_ad(910, assign11240_ad_e10664, 0.5);

        s.store_div(911, 894, 903);

        s.store_div(912, 895, 904);

        s.store_div_from_scalar(885, 1.0, 911);

        s.store_div_from_scalar(886, 1.0, 912);

        s.store_div_from_scalar_ad(913, 1.0, A::add(A::offset(s.ad_value(885), 1.0), s.ad_value(886)));

        s.store_div_ad_rhs(884, 253, A::square(s.ad_value(898)));

        s.store_div_ad(881, A::offset(s.ad_value(911), 1.0), A::offset(s.ad_value(912), 1.0));

        s.store_ln(882, 881);

        s.v[1075] = if (s.v[882] > 1e-8) { 1.0 } else { 0.0 };

        if (s.v[1075] != 0.0) {
            s.store_div_ad(883, A::mul(A::scale(s.ad_value(882), 2.0), A::offset(s.ad_value(881), 1.0)), A::offset(s.ad_value(881), (-1.0)));
        }

        if (!(s.v[1075] != 0.0)) {
            s.store_scaled_offset(883, 882, 2.0, 2.0);
        }

        s.store_mul_ad_rhs(914, 913, A::sub(s.ad_value(909), s.ad_value(910)));

        s.store_square(915, 914);

        s.store_sub_ad_rhs(887, 909, A::mul(s.ad_value(914), s.ad_value(885)));

        s.store_add_ad_rhs(888, 910, A::mul(s.ad_value(914), s.ad_value(886)));

        s.store_div_from_scalar_ad(793, 1.0, A::offset(s.ad_value(911), 1.0));

        s.store_div_from_scalar_ad(794, 1.0, A::offset(s.ad_value(912), 1.0));

        s.store_offset_ad(796, A::ln(A::div(A::mul(A::add(s.ad_value(911), A::mul(s.ad_value(912), s.ad_value(794))), s.ad_value(883)), s.ad_value(884))), 3.0);

        s.store_offset_ad(797, A::ln(A::div(A::mul(A::add(s.ad_value(912), A::mul(s.ad_value(911), s.ad_value(793))), s.ad_value(883)), s.ad_value(884))), 3.0);

        s.v[1076] = if (((s.v[796] - s.v[887]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1076] != 0.0) {
            s.store_ln_ad(795, A::offset(A::exp(A::scale(A::sub(s.ad_value(796), s.ad_value(887)), 0.3333333333333)), 1.0));
        }

        if (!(s.v[1076] != 0.0)) {
            s.store_scaled_sub(795, 796, 887, 0.3333333333333);
        }

        s.store_sub_ad_rhs(800, 796, A::scale(s.ad_value(795), 3.0));

        s.v[1077] = if (((s.v[797] - s.v[888]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1077] != 0.0) {
            s.store_ln_ad(795, A::offset(A::exp(A::scale(A::sub(s.ad_value(797), s.ad_value(888)), 0.3333333333333)), 1.0));
        }

        if (!(s.v[1077] != 0.0)) {
            s.store_scaled_sub(795, 797, 888, 0.3333333333333);
        }

        s.store_sub_ad_rhs(801, 797, A::scale(s.ad_value(795), 3.0));

        s.store_mul_ad_lhs(798, A::add(A::mul(s.ad_value(911), s.ad_value(909)), s.ad_value(801)), 793);

        s.store_mul_ad_lhs(799, A::add(A::mul(s.ad_value(912), s.ad_value(910)), s.ad_value(800)), 794);

        s.v[1078] = if (((s.v[796] - s.v[798]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1078] != 0.0) {
            s.store_ln_ad(795, A::offset(A::exp(A::scale(A::sub(s.ad_value(796), s.ad_value(798)), 0.3333333333333)), 1.0));
        }

        if (!(s.v[1078] != 0.0)) {
            s.store_scaled_sub(795, 796, 798, 0.3333333333333);
        }

        s.store_sub_ad_rhs(800, 796, A::scale(s.ad_value(795), 3.0));

        s.v[1079] = if (((s.v[797] - s.v[799]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1079] != 0.0) {
            s.store_ln_ad(795, A::offset(A::exp(A::scale(A::sub(s.ad_value(797), s.ad_value(799)), 0.3333333333333)), 1.0));
        }

        if (!(s.v[1079] != 0.0)) {
            s.store_scaled_sub(795, 797, 799, 0.3333333333333);
        }

    }

    pub(super) fn stamp_transient_block_8(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.store_sub_ad_rhs(801, 797, A::scale(s.ad_value(795), 3.0));

        s.store_sub(916, 909, 800);

        s.store_sub(920, 910, 801);

        s.v[807] = 0.0;

        s.v[810] = 0.0;

        s.store_mul(802, 911, 916);

        s.v[1080] = if ((s.v[909] - s.v[916]) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1080] != 0.0) {
            s.store_exp_ad(793, A::sub(s.ad_value(909), s.ad_value(916)));
        }

        if (!(s.v[1080] != 0.0)) {
            s.store_scale_ad(793, A::offset(A::mul(A::offset(A::sub(s.ad_value(909), s.ad_value(916)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(909), s.ad_value(916)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(909), s.ad_value(916)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        s.store_mul(803, 884, 793);

        s.store_sub_ad_lhs(804, A::square(s.ad_value(802)), 803);

        s.store_add_ad_lhs(805, A::mul(A::scale(s.ad_value(911), 2.0), s.ad_value(802)), 803);

        s.store_sub_ad_lhs(806, A::mul(A::scale(s.ad_value(911), 2.0), s.ad_value(911)), 803);

        s.v[1081] = if (s.v[804] < (-0.005)) { 1.0 } else { 0.0 };

        if (s.v[1081] != 0.0) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(804)));
        }

        if (s.v[1081] != 0.0) {
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
        }

        if (s.v[1081] != 0.0) {
            s.store_div_ad_lhs(793, A::scale(s.ad_value(805), 0.25), 804);
        }

        if (s.v[1081] != 0.0) {
            s.store_mul_ad_lhs(809, A::add(s.ad_value(804), A::mul(s.ad_value(808), A::sub_from_scalar(2.0, s.ad_value(808)))), 793);
        }

        if (s.v[1081] != 0.0) {
            s.store_add_ad(811, A::mul(A::sub(s.ad_value(805), A::mul(A::scale(s.ad_value(809), 2.0), A::offset(s.ad_value(808), 1.0))), s.ad_value(793)), A::div(A::mul(s.ad_value(809), s.ad_value(806)), s.ad_value(805)));
        }

        if (s.v[1081] != 0.0) {
            s.store_sub_from_scalar_ad(794, 1.0, A::scale(s.ad_value(808), 0.5));
        }

        if (s.v[1081] != 0.0) {
            s.store_mul_ad_lhs(814, A::div(s.ad_value(805), s.ad_value(804)), 794);
        }

        if (s.v[1081] != 0.0) {
            s.store_div_ad_lhs(815, A::sub(A::mul(s.ad_value(806), s.ad_value(794)), A::mul(s.ad_value(805), A::add(s.ad_value(814), A::scale(s.ad_value(809), 0.5)))), 804);
        }

        s.v[1082] = if (s.v[804] > 0.005) { 1.0 } else { 0.0 };

        if ((!(s.v[1081] != 0.0)) && (s.v[1082] != 0.0)) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(804)));
        }

        if ((!(s.v[1081] != 0.0)) && (s.v[1082] != 0.0)) {
            s.store_exp_ad(810, A::neg(s.ad_value(807)));
        }

        if ((!(s.v[1081] != 0.0)) && (s.v[1082] != 0.0)) {
            s.store_div_ad(808, A::mul(s.ad_value(807), A::offset(s.ad_value(810), 1.0)), A::sub_from_scalar(1.0, s.ad_value(810)));
        }

        if ((!(s.v[1081] != 0.0)) && (s.v[1082] != 0.0)) {
            s.store_div_ad_lhs(793, A::scale(s.ad_value(805), 0.25), 804);
        }

        if ((!(s.v[1081] != 0.0)) && (s.v[1082] != 0.0)) {
            s.store_mul_ad_lhs(809, A::add(s.ad_value(804), A::mul(s.ad_value(808), A::sub_from_scalar(2.0, s.ad_value(808)))), 793);
        }

        if ((!(s.v[1081] != 0.0)) && (s.v[1082] != 0.0)) {
            s.store_add_ad(811, A::mul(A::sub(s.ad_value(805), A::mul(A::scale(s.ad_value(809), 2.0), A::offset(s.ad_value(808), 1.0))), s.ad_value(793)), A::div(A::mul(s.ad_value(809), s.ad_value(806)), s.ad_value(805)));
        }

        if ((!(s.v[1081] != 0.0)) && (s.v[1082] != 0.0)) {
            s.store_sub_from_scalar_ad(794, 1.0, A::scale(s.ad_value(808), 0.5));
        }

        if ((!(s.v[1081] != 0.0)) && (s.v[1082] != 0.0)) {
            s.store_mul_ad_lhs(814, A::div(s.ad_value(805), s.ad_value(804)), 794);
        }

        if ((!(s.v[1081] != 0.0)) && (s.v[1082] != 0.0)) {
            s.store_div_ad_lhs(815, A::sub(A::mul(s.ad_value(806), s.ad_value(794)), A::mul(s.ad_value(805), A::add(s.ad_value(814), A::scale(s.ad_value(809), 0.5)))), 804);
        }

        if ((!(s.v[1081] != 0.0)) && (!(s.v[1082] != 0.0))) {
            s.store_scale_ad(795, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.025)))))), 0.1666666666667);
        }

        if ((!(s.v[1081] != 0.0)) && (!(s.v[1082] != 0.0))) {
            s.store_offset_ad(808, A::mul(s.ad_value(804), s.ad_value(795)), 2.0);
        }

        if ((!(s.v[1081] != 0.0)) && (!(s.v[1082] != 0.0))) {
            s.store_scale_ad(793, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0333333333333)))))), 0.1666666666667);
        }

        if ((!(s.v[1081] != 0.0)) && (!(s.v[1082] != 0.0))) {
            s.store_mul(809, 805, 793);
        }

        if ((!(s.v[1081] != 0.0)) && (!(s.v[1082] != 0.0))) {
            s.store_scale_ad(794, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0420875420875421)))))), 0.0055555555556);
        }

        if ((!(s.v[1081] != 0.0)) && (!(s.v[1082] != 0.0))) {
            s.store_sub_ad(811, A::mul(s.ad_value(806), s.ad_value(793)), A::mul(A::square(s.ad_value(805)), s.ad_value(794)));
        }

        if ((!(s.v[1081] != 0.0)) && (!(s.v[1082] != 0.0))) {
            s.store_mul_ad_lhs(814, A::scale(s.ad_value(805), (-0.5)), 795);
        }

        if ((!(s.v[1081] != 0.0)) && (!(s.v[1082] != 0.0))) {
            s.store_add_ad(815, A::mul(A::scale(s.ad_value(806), (-0.5)), s.ad_value(795)), A::mul(A::mul(A::scale(s.ad_value(805), (0.25 * 0.0055555555556)), s.ad_value(805)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(804), 0.075))))));
        }

        s.v[1083] = if (s.v[804] > 0.005) { 1.0 } else { 0.0 };

        if (s.v[1083] != 0.0) {
            s.store_div_ad(794, A::scale(s.ad_value(804), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(810), A::sub_from_scalar(2.0, s.ad_value(810)))));
        }

        if (s.v[1083] != 0.0) {
            s.store_mul(812, 794, 810);
        }

        if (s.v[1083] != 0.0) {
            s.store_sub_ad_lhs(813, A::ln(s.ad_value(794)), 807);
        }

        s.v[1084] = if (s.v[804] < (-0.005)) { 1.0 } else { 0.0 };

        if ((!(s.v[1083] != 0.0)) && (s.v[1084] != 0.0)) {
            s.store_sin_ad(794, A::scale(s.ad_value(807), 0.5));
        }

        if ((!(s.v[1083] != 0.0)) && (s.v[1084] != 0.0)) {
            s.store_div_ad(812, A::neg(s.ad_value(804)), A::square(s.ad_value(794)));
        }

        if ((!(s.v[1083] != 0.0)) && (s.v[1084] != 0.0)) {
            s.store_ln(813, 812);
        }

        if ((!(s.v[1083] != 0.0)) && (!(s.v[1084] != 0.0))) {
            s.store_sub_from_scalar_ad(812, 4.0, A::mul(A::scale(s.ad_value(804), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0396825396825397))))));
        }

        if ((!(s.v[1083] != 0.0)) && (!(s.v[1084] != 0.0))) {
            s.store_ln(813, 812);
        }

        s.v[1085] = if (((1.01 * s.v[802]) + s.v[808]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1085] != 0.0) {
            s.store_add(816, 802, 808);
        }

        if (s.v[1085] != 0.0) {
            s.store_add(817, 911, 809);
        }

        if (s.v[1085] != 0.0) {
            s.copy_ad(818, 811);
        }

        if (!(s.v[1085] != 0.0)) {
            s.store_div_from_scalar_ad(794, 1.0, A::sub(s.ad_value(802), s.ad_value(808)));
        }

        if (!(s.v[1085] != 0.0)) {
            s.store_sub(795, 809, 911);
        }

        if (!(s.v[1085] != 0.0)) {
            s.store_mul_ad_lhs(816, A::sub(s.ad_value(803), s.ad_value(812)), 794);
        }

        if (!(s.v[1085] != 0.0)) {
            s.store_mul_ad_lhs(817, A::sub(A::sub(A::mul(s.ad_value(795), s.ad_value(816)), s.ad_value(803)), A::mul(s.ad_value(814), s.ad_value(812))), 794);
        }

        if (!(s.v[1085] != 0.0)) {
            s.store_mul_ad_lhs(818, A::sub(A::add(A::add(A::mul(s.ad_value(811), s.ad_value(816)), A::mul(A::scale(s.ad_value(795), 2.0), s.ad_value(817))), s.ad_value(803)), A::mul(A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812))), 794);
        }

        s.v[1086] = if (s.v[816] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1086] != 0.0) {
            s.store_ln(819, 816);
        }

        if (s.v[1086] != 0.0) {
            s.store_div_from_scalar(793, 1.0, 816);
        }

        if (s.v[1086] != 0.0) {
            s.store_mul(820, 817, 793);
        }

        if (s.v[1086] != 0.0) {
            s.store_sub_ad(821, A::mul(s.ad_value(818), s.ad_value(793)), A::square(s.ad_value(820)));
        }

        if (!(s.v[1086] != 0.0)) {
            s.store_add_ad(819, A::offset(s.ad_value(802), 0.6931471805599), A::ln(A::neg(s.ad_value(802))));
        }

        if (!(s.v[1086] != 0.0)) {
            s.store_div_from_scalar(793, 1.0, 916);
        }

        if (!(s.v[1086] != 0.0)) {
            s.store_add(820, 911, 793);
        }

        if (!(s.v[1086] != 0.0)) {
            s.store_mul_ad_lhs(821, A::neg(s.ad_value(793)), 793);
        }

        s.store_sub_ad_lhs(822, A::add(A::add(A::sub(s.ad_value(910), s.ad_value(909)), s.ad_value(916)), A::scale(s.ad_value(819), 2.0)), 813);

        s.store_sub_ad_lhs(823, A::offset(A::scale(s.ad_value(820), 2.0), 1.0), 814);

        s.store_sub_ad_lhs(824, A::scale(s.ad_value(821), 2.0), 815);

        s.store_add_ad_rhs(825, 802, A::mul(s.ad_value(912), s.ad_value(822)));

        s.store_add_ad_rhs(826, 911, A::mul(s.ad_value(912), s.ad_value(823)));

        s.store_mul(827, 912, 824);

        s.store_sub_ad_lhs(828, A::mul(s.ad_value(825), s.ad_value(816)), 803);

        s.store_add_ad_lhs(829, A::add(A::mul(s.ad_value(826), s.ad_value(816)), A::mul(s.ad_value(825), s.ad_value(817))), 803);

        s.store_sub_ad_lhs(830, A::add(A::add(A::mul(s.ad_value(827), s.ad_value(816)), A::mul(A::scale(s.ad_value(826), 2.0), s.ad_value(817))), A::mul(s.ad_value(825), s.ad_value(818))), 803);

        s.store_sub_ad(839, A::square(s.ad_value(829)), A::mul(A::scale(s.ad_value(828), 0.5), s.ad_value(830)));

        s.store_div_ad(831, A::mul(A::mul(A::neg(s.ad_value(828)), s.ad_value(829)), s.ad_value(839)), A::offset(A::square(s.ad_value(839)), 1e-200));

        s.store_add(916, 916, 831);

        s.store_mul(802, 911, 916);

        s.store_mul(832, 912, 920);

        s.store_add(825, 802, 832);

        s.store_offset_scaled(833, 825, 0.065345483024, 1.0);

        s.store_add_ad(834, A::offset(A::scale(s.ad_value(825), 8.5797362674), 39.478417604), A::mul(s.ad_value(802), s.ad_value(832)));

        s.store_scale_ad(835, A::add(A::scale(s.ad_value(825), 2.0), A::mul(s.ad_value(802), s.ad_value(832))), 39.478417604);

        s.store_sqrt_ad(836, A::sub(A::square(s.ad_value(834)), A::mul(A::scale(s.ad_value(833), 4.0), s.ad_value(835))));

        s.store_div_ad(804, A::sub(s.ad_value(836), s.ad_value(834)), A::scale(s.ad_value(833), 2.0));

        s.store_sub_ad_lhs(837, A::square(s.ad_value(802)), 804);

        s.v[1087] = if (s.v[837] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1087] != 0.0) {
            s.store_mul_ad_rhs(828, 837, A::add(A::sub(A::ln(A::div(s.ad_value(837), s.ad_value(884))), s.ad_value(909)), s.ad_value(916)));
        }

        if (s.v[1087] != 0.0) {
            s.store_add_ad_lhs(829, A::mul(A::scale(s.ad_value(911), 2.0), s.ad_value(802)), 837);
        }

        if (s.v[1087] != 0.0) {
            s.store_sub_ad_lhs(838, A::sub(s.ad_value(909), s.ad_value(916)), 796);
        }

        s.v[1088] = if ((((s.v[828] < 0.0) && (s.v[829] > 0.0)) && (((s.v[838] + 2.3025850929941) + ((s.v[911]) as f64).ln()) > 0.0)) || (s.v[838] > 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1087] != 0.0) && (s.v[1088] != 0.0)) {
            s.store_sub_ad_rhs(916, 916, A::div(s.ad_value(828), s.ad_value(829)));
        }

        s.store_mul(802, 911, 916);

        s.store_mul(832, 912, 920);

        s.store_add(825, 802, 832);

        s.store_offset_scaled(833, 825, 0.065345483024, 1.0);

        s.store_add_ad(834, A::offset(A::scale(s.ad_value(825), 8.5797362674), 39.478417604), A::mul(s.ad_value(802), s.ad_value(832)));

        s.store_scale_ad(835, A::add(A::scale(s.ad_value(825), 2.0), A::mul(s.ad_value(802), s.ad_value(832))), 39.478417604);

        s.store_sqrt_ad(836, A::sub(A::square(s.ad_value(834)), A::mul(A::scale(s.ad_value(833), 4.0), s.ad_value(835))));

        s.store_div_ad(804, A::sub(s.ad_value(836), s.ad_value(834)), A::scale(s.ad_value(833), 2.0));

        s.v[1089] = if (s.v[804] < (-0.005)) { 1.0 } else { 0.0 };

        if (s.v[1089] != 0.0) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(804)));
        }

        if (s.v[1089] != 0.0) {
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
        }

        if (s.v[1089] != 0.0) {
            s.store_div_ad_lhs(809, A::scale(A::add(s.ad_value(804), A::mul(s.ad_value(808), A::sub_from_scalar(2.0, s.ad_value(808)))), 0.25), 804);
        }

        s.v[1090] = if (s.v[804] > 0.005) { 1.0 } else { 0.0 };

        if ((!(s.v[1089] != 0.0)) && (s.v[1090] != 0.0)) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(804)));
        }

        if ((!(s.v[1089] != 0.0)) && (s.v[1090] != 0.0)) {
            s.store_exp_ad(810, A::neg(s.ad_value(807)));
        }

        if ((!(s.v[1089] != 0.0)) && (s.v[1090] != 0.0)) {
            s.store_div_ad(808, A::mul(s.ad_value(807), A::offset(s.ad_value(810), 1.0)), A::sub_from_scalar(1.0, s.ad_value(810)));
        }

        if ((!(s.v[1089] != 0.0)) && (s.v[1090] != 0.0)) {
            s.store_div_ad_lhs(809, A::scale(A::add(s.ad_value(804), A::mul(s.ad_value(808), A::sub_from_scalar(2.0, s.ad_value(808)))), 0.25), 804);
        }

        if ((!(s.v[1089] != 0.0)) && (!(s.v[1090] != 0.0))) {
            s.store_offset_ad(808, A::mul(A::scale(s.ad_value(804), 0.1666666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0166666666667), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0238095238095))))), 2.0);
        }

        if ((!(s.v[1089] != 0.0)) && (!(s.v[1090] != 0.0))) {
            s.store_scale_ad(809, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0333333333333)))))), 0.1666666666667);
        }

        s.store_sub_ad_rhs(804, 804, A::div(A::add(A::add(A::mul(s.ad_value(825), s.ad_value(808)), A::mul(s.ad_value(802), s.ad_value(832))), s.ad_value(804)), A::offset(A::mul(s.ad_value(825), s.ad_value(809)), 1.0)));

        s.store_sub_ad_lhs(837, A::square(s.ad_value(802)), 804);

        s.v[1091] = if (s.v[837] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1091] != 0.0) {
            s.store_mul_ad_rhs(828, 837, A::add(A::sub(A::ln(A::div(s.ad_value(837), s.ad_value(884))), s.ad_value(909)), s.ad_value(916)));
        }

        if (s.v[1091] != 0.0) {
            s.store_add_ad_lhs(829, A::mul(A::scale(s.ad_value(911), 2.0), s.ad_value(802)), 837);
        }

        if (s.v[1091] != 0.0) {
            s.store_sub_ad_lhs(838, A::sub(s.ad_value(909), s.ad_value(916)), 796);
        }

        s.v[1092] = if ((((s.v[828] < 0.0) && (s.v[829] > 0.0)) && (((s.v[838] + 2.3025850929941) + ((s.v[911]) as f64).ln()) > 0.0)) || (s.v[838] > 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1091] != 0.0) && (s.v[1092] != 0.0)) {
            s.store_sub_ad_rhs(916, 916, A::div(s.ad_value(828), s.ad_value(829)));
        }

        s.store_mul(802, 911, 916);

        s.v[1093] = if ((s.v[909] - s.v[916]) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1093] != 0.0) {
            s.store_exp_ad(793, A::sub(s.ad_value(909), s.ad_value(916)));
        }

        if (!(s.v[1093] != 0.0)) {
            s.store_scale_ad(793, A::offset(A::mul(A::offset(A::sub(s.ad_value(909), s.ad_value(916)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(909), s.ad_value(916)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(909), s.ad_value(916)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        s.store_mul(803, 884, 793);

        s.store_sub_ad_lhs(804, A::square(s.ad_value(802)), 803);

        s.store_add_ad_lhs(805, A::mul(A::scale(s.ad_value(911), 2.0), s.ad_value(802)), 803);

        s.store_sub_ad_lhs(806, A::mul(A::scale(s.ad_value(911), 2.0), s.ad_value(911)), 803);

        s.v[1094] = if (s.v[804] < (-0.005)) { 1.0 } else { 0.0 };

        if (s.v[1094] != 0.0) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(804)));
        }

        if (s.v[1094] != 0.0) {
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
        }

        if (s.v[1094] != 0.0) {
            s.store_div_ad_lhs(793, A::scale(s.ad_value(805), 0.25), 804);
        }

        if (s.v[1094] != 0.0) {
            s.store_mul_ad_lhs(809, A::add(s.ad_value(804), A::mul(s.ad_value(808), A::sub_from_scalar(2.0, s.ad_value(808)))), 793);
        }

        if (s.v[1094] != 0.0) {
            s.store_add_ad(811, A::mul(A::sub(s.ad_value(805), A::mul(A::scale(s.ad_value(809), 2.0), A::offset(s.ad_value(808), 1.0))), s.ad_value(793)), A::div(A::mul(s.ad_value(809), s.ad_value(806)), s.ad_value(805)));
        }

        if (s.v[1094] != 0.0) {
            s.store_sub_from_scalar_ad(794, 1.0, A::scale(s.ad_value(808), 0.5));
        }

        if (s.v[1094] != 0.0) {
            s.store_mul_ad_lhs(814, A::div(s.ad_value(805), s.ad_value(804)), 794);
        }

        if (s.v[1094] != 0.0) {
            s.store_div_ad_lhs(815, A::sub(A::mul(s.ad_value(806), s.ad_value(794)), A::mul(s.ad_value(805), A::add(s.ad_value(814), A::scale(s.ad_value(809), 0.5)))), 804);
        }

        s.v[1095] = if (s.v[804] > 0.005) { 1.0 } else { 0.0 };

        if ((!(s.v[1094] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(804)));
        }

        if ((!(s.v[1094] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_exp_ad(810, A::neg(s.ad_value(807)));
        }

        if ((!(s.v[1094] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_div_ad(808, A::mul(s.ad_value(807), A::offset(s.ad_value(810), 1.0)), A::sub_from_scalar(1.0, s.ad_value(810)));
        }

        if ((!(s.v[1094] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_div_ad_lhs(793, A::scale(s.ad_value(805), 0.25), 804);
        }

        if ((!(s.v[1094] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_mul_ad_lhs(809, A::add(s.ad_value(804), A::mul(s.ad_value(808), A::sub_from_scalar(2.0, s.ad_value(808)))), 793);
        }

        if ((!(s.v[1094] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_add_ad(811, A::mul(A::sub(s.ad_value(805), A::mul(A::scale(s.ad_value(809), 2.0), A::offset(s.ad_value(808), 1.0))), s.ad_value(793)), A::div(A::mul(s.ad_value(809), s.ad_value(806)), s.ad_value(805)));
        }

        if ((!(s.v[1094] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_sub_from_scalar_ad(794, 1.0, A::scale(s.ad_value(808), 0.5));
        }

        if ((!(s.v[1094] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_mul_ad_lhs(814, A::div(s.ad_value(805), s.ad_value(804)), 794);
        }

        if ((!(s.v[1094] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_div_ad_lhs(815, A::sub(A::mul(s.ad_value(806), s.ad_value(794)), A::mul(s.ad_value(805), A::add(s.ad_value(814), A::scale(s.ad_value(809), 0.5)))), 804);
        }

        if ((!(s.v[1094] != 0.0)) && (!(s.v[1095] != 0.0))) {
            s.store_scale_ad(795, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.025)))))), 0.1666666666667);
        }

        if ((!(s.v[1094] != 0.0)) && (!(s.v[1095] != 0.0))) {
            s.store_offset_ad(808, A::mul(s.ad_value(804), s.ad_value(795)), 2.0);
        }

        if ((!(s.v[1094] != 0.0)) && (!(s.v[1095] != 0.0))) {
            s.store_scale_ad(793, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0333333333333)))))), 0.1666666666667);
        }

        if ((!(s.v[1094] != 0.0)) && (!(s.v[1095] != 0.0))) {
            s.store_mul(809, 805, 793);
        }

        if ((!(s.v[1094] != 0.0)) && (!(s.v[1095] != 0.0))) {
            s.store_scale_ad(794, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0420875420875421)))))), 0.0055555555556);
        }

        if ((!(s.v[1094] != 0.0)) && (!(s.v[1095] != 0.0))) {
            s.store_sub_ad(811, A::mul(s.ad_value(806), s.ad_value(793)), A::mul(A::square(s.ad_value(805)), s.ad_value(794)));
        }

        if ((!(s.v[1094] != 0.0)) && (!(s.v[1095] != 0.0))) {
            s.store_mul_ad_lhs(814, A::scale(s.ad_value(805), (-0.5)), 795);
        }

        if ((!(s.v[1094] != 0.0)) && (!(s.v[1095] != 0.0))) {
            s.store_add_ad(815, A::mul(A::scale(s.ad_value(806), (-0.5)), s.ad_value(795)), A::mul(A::mul(A::scale(s.ad_value(805), (0.25 * 0.0055555555556)), s.ad_value(805)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(804), 0.075))))));
        }

        s.v[1096] = if (s.v[804] > 0.005) { 1.0 } else { 0.0 };

        if (s.v[1096] != 0.0) {
            s.store_div_ad(794, A::scale(s.ad_value(804), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(810), A::sub_from_scalar(2.0, s.ad_value(810)))));
        }

    }

    pub(super) fn stamp_transient_block_9(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1096] != 0.0) {
            s.store_mul(812, 794, 810);
        }

        if (s.v[1096] != 0.0) {
            s.store_sub_ad_lhs(813, A::ln(s.ad_value(794)), 807);
        }

        s.v[1097] = if (s.v[804] < (-0.005)) { 1.0 } else { 0.0 };

        if ((!(s.v[1096] != 0.0)) && (s.v[1097] != 0.0)) {
            s.store_sin_ad(794, A::scale(s.ad_value(807), 0.5));
        }

        if ((!(s.v[1096] != 0.0)) && (s.v[1097] != 0.0)) {
            s.store_div_ad(812, A::neg(s.ad_value(804)), A::square(s.ad_value(794)));
        }

        if ((!(s.v[1096] != 0.0)) && (s.v[1097] != 0.0)) {
            s.store_ln(813, 812);
        }

        if ((!(s.v[1096] != 0.0)) && (!(s.v[1097] != 0.0))) {
            s.store_sub_from_scalar_ad(812, 4.0, A::mul(A::scale(s.ad_value(804), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0396825396825397))))));
        }

        if ((!(s.v[1096] != 0.0)) && (!(s.v[1097] != 0.0))) {
            s.store_ln(813, 812);
        }

        s.v[1098] = if (((1.01 * s.v[802]) + s.v[808]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1098] != 0.0) {
            s.store_add(816, 802, 808);
        }

        if (s.v[1098] != 0.0) {
            s.store_add(817, 911, 809);
        }

        if (s.v[1098] != 0.0) {
            s.copy_ad(818, 811);
        }

        if (!(s.v[1098] != 0.0)) {
            s.store_div_from_scalar_ad(794, 1.0, A::sub(s.ad_value(802), s.ad_value(808)));
        }

        if (!(s.v[1098] != 0.0)) {
            s.store_sub(795, 809, 911);
        }

        if (!(s.v[1098] != 0.0)) {
            s.store_mul_ad_lhs(816, A::sub(s.ad_value(803), s.ad_value(812)), 794);
        }

        if (!(s.v[1098] != 0.0)) {
            s.store_mul_ad_lhs(817, A::sub(A::sub(A::mul(s.ad_value(795), s.ad_value(816)), s.ad_value(803)), A::mul(s.ad_value(814), s.ad_value(812))), 794);
        }

        if (!(s.v[1098] != 0.0)) {
            s.store_mul_ad_lhs(818, A::sub(A::add(A::add(A::mul(s.ad_value(811), s.ad_value(816)), A::mul(A::scale(s.ad_value(795), 2.0), s.ad_value(817))), s.ad_value(803)), A::mul(A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812))), 794);
        }

        s.v[1099] = if (s.v[816] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1099] != 0.0) {
            s.store_ln(819, 816);
        }

        if (s.v[1099] != 0.0) {
            s.store_div_from_scalar(793, 1.0, 816);
        }

        if (s.v[1099] != 0.0) {
            s.store_mul(820, 817, 793);
        }

        if (s.v[1099] != 0.0) {
            s.store_sub_ad(821, A::mul(s.ad_value(818), s.ad_value(793)), A::square(s.ad_value(820)));
        }

        if (!(s.v[1099] != 0.0)) {
            s.store_add_ad(819, A::offset(s.ad_value(802), 0.6931471805599), A::ln(A::neg(s.ad_value(802))));
        }

        if (!(s.v[1099] != 0.0)) {
            s.store_div_from_scalar(793, 1.0, 916);
        }

        if (!(s.v[1099] != 0.0)) {
            s.store_add(820, 911, 793);
        }

        if (!(s.v[1099] != 0.0)) {
            s.store_mul_ad_lhs(821, A::neg(s.ad_value(793)), 793);
        }

        s.store_sub_ad_lhs(822, A::add(A::add(A::sub(s.ad_value(910), s.ad_value(909)), s.ad_value(916)), A::scale(s.ad_value(819), 2.0)), 813);

        s.store_sub_ad_lhs(823, A::offset(A::scale(s.ad_value(820), 2.0), 1.0), 814);

        s.store_sub_ad_lhs(824, A::scale(s.ad_value(821), 2.0), 815);

        s.store_add_ad_rhs(825, 802, A::mul(s.ad_value(912), s.ad_value(822)));

        s.store_add_ad_rhs(826, 911, A::mul(s.ad_value(912), s.ad_value(823)));

        s.store_mul(827, 912, 824);

        s.store_sub_ad_lhs(828, A::mul(s.ad_value(825), s.ad_value(816)), 803);

        s.store_add_ad_lhs(829, A::add(A::mul(s.ad_value(826), s.ad_value(816)), A::mul(s.ad_value(825), s.ad_value(817))), 803);

        s.store_sub_ad_lhs(830, A::add(A::add(A::mul(s.ad_value(827), s.ad_value(816)), A::mul(A::scale(s.ad_value(826), 2.0), s.ad_value(817))), A::mul(s.ad_value(825), s.ad_value(818))), 803);

        s.store_sub_ad(839, A::square(s.ad_value(829)), A::mul(A::scale(s.ad_value(828), 0.5), s.ad_value(830)));

        s.store_div_ad(831, A::mul(A::mul(A::neg(s.ad_value(828)), s.ad_value(829)), s.ad_value(839)), A::offset(A::square(s.ad_value(839)), 1e-200));

        s.store_add(916, 916, 831);

        s.store_mul(802, 911, 916);

        s.v[1100] = if ((s.v[909] - s.v[916]) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1100] != 0.0) {
            s.store_exp_ad(793, A::sub(s.ad_value(909), s.ad_value(916)));
        }

        if (!(s.v[1100] != 0.0)) {
            s.store_scale_ad(793, A::offset(A::mul(A::offset(A::sub(s.ad_value(909), s.ad_value(916)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(909), s.ad_value(916)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(909), s.ad_value(916)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        s.store_mul(803, 884, 793);

        s.store_sub_ad_lhs(804, A::square(s.ad_value(802)), 803);

        s.store_add_ad_lhs(805, A::mul(A::scale(s.ad_value(911), 2.0), s.ad_value(802)), 803);

        s.store_sub_ad_lhs(806, A::mul(A::scale(s.ad_value(911), 2.0), s.ad_value(911)), 803);

        s.v[1101] = if (s.v[804] < (-0.005)) { 1.0 } else { 0.0 };

        if (s.v[1101] != 0.0) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(804)));
        }

        if (s.v[1101] != 0.0) {
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
        }

        if (s.v[1101] != 0.0) {
            s.store_div_ad_lhs(793, A::scale(s.ad_value(805), 0.25), 804);
        }

        if (s.v[1101] != 0.0) {
            s.store_mul_ad_lhs(809, A::add(s.ad_value(804), A::mul(s.ad_value(808), A::sub_from_scalar(2.0, s.ad_value(808)))), 793);
        }

        if (s.v[1101] != 0.0) {
            s.store_add_ad(811, A::mul(A::sub(s.ad_value(805), A::mul(A::scale(s.ad_value(809), 2.0), A::offset(s.ad_value(808), 1.0))), s.ad_value(793)), A::div(A::mul(s.ad_value(809), s.ad_value(806)), s.ad_value(805)));
        }

        if (s.v[1101] != 0.0) {
            s.store_sub_from_scalar_ad(794, 1.0, A::scale(s.ad_value(808), 0.5));
        }

        if (s.v[1101] != 0.0) {
            s.store_mul_ad_lhs(814, A::div(s.ad_value(805), s.ad_value(804)), 794);
        }

        if (s.v[1101] != 0.0) {
            s.store_div_ad_lhs(815, A::sub(A::mul(s.ad_value(806), s.ad_value(794)), A::mul(s.ad_value(805), A::add(s.ad_value(814), A::scale(s.ad_value(809), 0.5)))), 804);
        }

        s.v[1102] = if (s.v[804] > 0.005) { 1.0 } else { 0.0 };

        if ((!(s.v[1101] != 0.0)) && (s.v[1102] != 0.0)) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(804)));
        }

        if ((!(s.v[1101] != 0.0)) && (s.v[1102] != 0.0)) {
            s.store_exp_ad(810, A::neg(s.ad_value(807)));
        }

        if ((!(s.v[1101] != 0.0)) && (s.v[1102] != 0.0)) {
            s.store_div_ad(808, A::mul(s.ad_value(807), A::offset(s.ad_value(810), 1.0)), A::sub_from_scalar(1.0, s.ad_value(810)));
        }

        if ((!(s.v[1101] != 0.0)) && (s.v[1102] != 0.0)) {
            s.store_div_ad_lhs(793, A::scale(s.ad_value(805), 0.25), 804);
        }

        if ((!(s.v[1101] != 0.0)) && (s.v[1102] != 0.0)) {
            s.store_mul_ad_lhs(809, A::add(s.ad_value(804), A::mul(s.ad_value(808), A::sub_from_scalar(2.0, s.ad_value(808)))), 793);
        }

        if ((!(s.v[1101] != 0.0)) && (s.v[1102] != 0.0)) {
            s.store_add_ad(811, A::mul(A::sub(s.ad_value(805), A::mul(A::scale(s.ad_value(809), 2.0), A::offset(s.ad_value(808), 1.0))), s.ad_value(793)), A::div(A::mul(s.ad_value(809), s.ad_value(806)), s.ad_value(805)));
        }

        if ((!(s.v[1101] != 0.0)) && (s.v[1102] != 0.0)) {
            s.store_sub_from_scalar_ad(794, 1.0, A::scale(s.ad_value(808), 0.5));
        }

        if ((!(s.v[1101] != 0.0)) && (s.v[1102] != 0.0)) {
            s.store_mul_ad_lhs(814, A::div(s.ad_value(805), s.ad_value(804)), 794);
        }

        if ((!(s.v[1101] != 0.0)) && (s.v[1102] != 0.0)) {
            s.store_div_ad_lhs(815, A::sub(A::mul(s.ad_value(806), s.ad_value(794)), A::mul(s.ad_value(805), A::add(s.ad_value(814), A::scale(s.ad_value(809), 0.5)))), 804);
        }

        if ((!(s.v[1101] != 0.0)) && (!(s.v[1102] != 0.0))) {
            s.store_scale_ad(795, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.025)))))), 0.1666666666667);
        }

        if ((!(s.v[1101] != 0.0)) && (!(s.v[1102] != 0.0))) {
            s.store_offset_ad(808, A::mul(s.ad_value(804), s.ad_value(795)), 2.0);
        }

        if ((!(s.v[1101] != 0.0)) && (!(s.v[1102] != 0.0))) {
            s.store_scale_ad(793, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0333333333333)))))), 0.1666666666667);
        }

        if ((!(s.v[1101] != 0.0)) && (!(s.v[1102] != 0.0))) {
            s.store_mul(809, 805, 793);
        }

        if ((!(s.v[1101] != 0.0)) && (!(s.v[1102] != 0.0))) {
            s.store_scale_ad(794, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0420875420875421)))))), 0.0055555555556);
        }

        if ((!(s.v[1101] != 0.0)) && (!(s.v[1102] != 0.0))) {
            s.store_sub_ad(811, A::mul(s.ad_value(806), s.ad_value(793)), A::mul(A::square(s.ad_value(805)), s.ad_value(794)));
        }

        if ((!(s.v[1101] != 0.0)) && (!(s.v[1102] != 0.0))) {
            s.store_mul_ad_lhs(814, A::scale(s.ad_value(805), (-0.5)), 795);
        }

        if ((!(s.v[1101] != 0.0)) && (!(s.v[1102] != 0.0))) {
            s.store_add_ad(815, A::mul(A::scale(s.ad_value(806), (-0.5)), s.ad_value(795)), A::mul(A::mul(A::scale(s.ad_value(805), (0.25 * 0.0055555555556)), s.ad_value(805)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(804), 0.075))))));
        }

        s.v[1103] = if (s.v[804] > 0.005) { 1.0 } else { 0.0 };

        if (s.v[1103] != 0.0) {
            s.store_div_ad(794, A::scale(s.ad_value(804), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(810), A::sub_from_scalar(2.0, s.ad_value(810)))));
        }

        if (s.v[1103] != 0.0) {
            s.store_mul(812, 794, 810);
        }

        if (s.v[1103] != 0.0) {
            s.store_sub_ad_lhs(813, A::ln(s.ad_value(794)), 807);
        }

        s.v[1104] = if (s.v[804] < (-0.005)) { 1.0 } else { 0.0 };

        if ((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) {
            s.store_sin_ad(794, A::scale(s.ad_value(807), 0.5));
        }

        if ((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) {
            s.store_div_ad(812, A::neg(s.ad_value(804)), A::square(s.ad_value(794)));
        }

        if ((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) {
            s.store_ln(813, 812);
        }

        if ((!(s.v[1103] != 0.0)) && (!(s.v[1104] != 0.0))) {
            s.store_sub_from_scalar_ad(812, 4.0, A::mul(A::scale(s.ad_value(804), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0396825396825397))))));
        }

        if ((!(s.v[1103] != 0.0)) && (!(s.v[1104] != 0.0))) {
            s.store_ln(813, 812);
        }

        s.v[1105] = if (((1.01 * s.v[802]) + s.v[808]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1105] != 0.0) {
            s.store_add(816, 802, 808);
        }

        if (s.v[1105] != 0.0) {
            s.store_add(817, 911, 809);
        }

        if (s.v[1105] != 0.0) {
            s.copy_ad(818, 811);
        }

        if (!(s.v[1105] != 0.0)) {
            s.store_div_from_scalar_ad(794, 1.0, A::sub(s.ad_value(802), s.ad_value(808)));
        }

        if (!(s.v[1105] != 0.0)) {
            s.store_sub(795, 809, 911);
        }

        if (!(s.v[1105] != 0.0)) {
            s.store_mul_ad_lhs(816, A::sub(s.ad_value(803), s.ad_value(812)), 794);
        }

        if (!(s.v[1105] != 0.0)) {
            s.store_mul_ad_lhs(817, A::sub(A::sub(A::mul(s.ad_value(795), s.ad_value(816)), s.ad_value(803)), A::mul(s.ad_value(814), s.ad_value(812))), 794);
        }

        if (!(s.v[1105] != 0.0)) {
            s.store_mul_ad_lhs(818, A::sub(A::add(A::add(A::mul(s.ad_value(811), s.ad_value(816)), A::mul(A::scale(s.ad_value(795), 2.0), s.ad_value(817))), s.ad_value(803)), A::mul(A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812))), 794);
        }

        s.v[1106] = if (s.v[816] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1106] != 0.0) {
            s.store_ln(819, 816);
        }

        if (s.v[1106] != 0.0) {
            s.store_div_from_scalar(793, 1.0, 816);
        }

        if (s.v[1106] != 0.0) {
            s.store_mul(820, 817, 793);
        }

        if (s.v[1106] != 0.0) {
            s.store_sub_ad(821, A::mul(s.ad_value(818), s.ad_value(793)), A::square(s.ad_value(820)));
        }

        if (!(s.v[1106] != 0.0)) {
            s.store_add_ad(819, A::offset(s.ad_value(802), 0.6931471805599), A::ln(A::neg(s.ad_value(802))));
        }

        if (!(s.v[1106] != 0.0)) {
            s.store_div_from_scalar(793, 1.0, 916);
        }

        if (!(s.v[1106] != 0.0)) {
            s.store_add(820, 911, 793);
        }

        if (!(s.v[1106] != 0.0)) {
            s.store_mul_ad_lhs(821, A::neg(s.ad_value(793)), 793);
        }

        s.store_sub_ad_lhs(822, A::add(A::add(A::sub(s.ad_value(910), s.ad_value(909)), s.ad_value(916)), A::scale(s.ad_value(819), 2.0)), 813);

        s.store_sub_ad_lhs(823, A::offset(A::scale(s.ad_value(820), 2.0), 1.0), 814);

        s.store_sub_ad_lhs(824, A::scale(s.ad_value(821), 2.0), 815);

        s.store_add_ad_rhs(825, 802, A::mul(s.ad_value(912), s.ad_value(822)));

        s.store_add_ad_rhs(826, 911, A::mul(s.ad_value(912), s.ad_value(823)));

        s.store_mul(827, 912, 824);

        s.store_sub_ad_lhs(828, A::mul(s.ad_value(825), s.ad_value(816)), 803);

        s.store_add_ad_lhs(829, A::add(A::mul(s.ad_value(826), s.ad_value(816)), A::mul(s.ad_value(825), s.ad_value(817))), 803);

        s.store_sub_ad_lhs(830, A::add(A::add(A::mul(s.ad_value(827), s.ad_value(816)), A::mul(A::scale(s.ad_value(826), 2.0), s.ad_value(817))), A::mul(s.ad_value(825), s.ad_value(818))), 803);

        s.store_sub_ad(839, A::square(s.ad_value(829)), A::mul(A::scale(s.ad_value(828), 0.5), s.ad_value(830)));

        s.store_div_ad(831, A::mul(A::mul(A::neg(s.ad_value(828)), s.ad_value(829)), s.ad_value(839)), A::offset(A::square(s.ad_value(839)), 1e-200));

        s.store_add(916, 916, 831);

        s.v[1107] = if (p.p10 == 1.0) { 1.0 } else { 0.0 };

        s.v[1108] = if (((s.v[831]) as f64).abs() > 0.01) { 1.0 } else { 0.0 };

        if ((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) {
            s.store_mul(802, 911, 916);
        }

        s.v[1109] = if ((s.v[909] - s.v[916]) < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (s.v[1109] != 0.0)) {
            s.store_exp_ad(793, A::sub(s.ad_value(909), s.ad_value(916)));
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1109] != 0.0))) {
            s.store_scale_ad(793, A::offset(A::mul(A::offset(A::sub(s.ad_value(909), s.ad_value(916)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(909), s.ad_value(916)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(909), s.ad_value(916)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if ((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) {
            s.store_mul(803, 884, 793);
        }

        if ((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) {
            s.store_sub_ad_lhs(804, A::square(s.ad_value(802)), 803);
        }

        if ((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) {
            s.store_add_ad_lhs(805, A::mul(A::scale(s.ad_value(911), 2.0), s.ad_value(802)), 803);
        }

        if ((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) {
            s.store_sub_ad_lhs(806, A::mul(A::scale(s.ad_value(911), 2.0), s.ad_value(911)), 803);
        }

        s.v[1110] = if (s.v[804] < (-0.005)) { 1.0 } else { 0.0 };

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (s.v[1110] != 0.0)) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(804)));
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (s.v[1110] != 0.0)) {
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (s.v[1110] != 0.0)) {
            s.store_div_ad_lhs(793, A::scale(s.ad_value(805), 0.25), 804);
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (s.v[1110] != 0.0)) {
            s.store_mul_ad_lhs(809, A::add(s.ad_value(804), A::mul(s.ad_value(808), A::sub_from_scalar(2.0, s.ad_value(808)))), 793);
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (s.v[1110] != 0.0)) {
            s.store_add_ad(811, A::mul(A::sub(s.ad_value(805), A::mul(A::scale(s.ad_value(809), 2.0), A::offset(s.ad_value(808), 1.0))), s.ad_value(793)), A::div(A::mul(s.ad_value(809), s.ad_value(806)), s.ad_value(805)));
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (s.v[1110] != 0.0)) {
            s.store_sub_from_scalar_ad(794, 1.0, A::scale(s.ad_value(808), 0.5));
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (s.v[1110] != 0.0)) {
            s.store_mul_ad_lhs(814, A::div(s.ad_value(805), s.ad_value(804)), 794);
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (s.v[1110] != 0.0)) {
            s.store_div_ad_lhs(815, A::sub(A::mul(s.ad_value(806), s.ad_value(794)), A::mul(s.ad_value(805), A::add(s.ad_value(814), A::scale(s.ad_value(809), 0.5)))), 804);
        }

        s.v[1111] = if (s.v[804] > 0.005) { 1.0 } else { 0.0 };

        if ((((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1110] != 0.0))) && (s.v[1111] != 0.0)) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(804)));
        }

        if ((((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1110] != 0.0))) && (s.v[1111] != 0.0)) {
            s.store_exp_ad(810, A::neg(s.ad_value(807)));
        }

        if ((((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1110] != 0.0))) && (s.v[1111] != 0.0)) {
            s.store_div_ad(808, A::mul(s.ad_value(807), A::offset(s.ad_value(810), 1.0)), A::sub_from_scalar(1.0, s.ad_value(810)));
        }

        if ((((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1110] != 0.0))) && (s.v[1111] != 0.0)) {
            s.store_div_ad_lhs(793, A::scale(s.ad_value(805), 0.25), 804);
        }

        if ((((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1110] != 0.0))) && (s.v[1111] != 0.0)) {
            s.store_mul_ad_lhs(809, A::add(s.ad_value(804), A::mul(s.ad_value(808), A::sub_from_scalar(2.0, s.ad_value(808)))), 793);
        }

        if ((((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1110] != 0.0))) && (s.v[1111] != 0.0)) {
            s.store_add_ad(811, A::mul(A::sub(s.ad_value(805), A::mul(A::scale(s.ad_value(809), 2.0), A::offset(s.ad_value(808), 1.0))), s.ad_value(793)), A::div(A::mul(s.ad_value(809), s.ad_value(806)), s.ad_value(805)));
        }

        if ((((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1110] != 0.0))) && (s.v[1111] != 0.0)) {
            s.store_sub_from_scalar_ad(794, 1.0, A::scale(s.ad_value(808), 0.5));
        }

        if ((((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1110] != 0.0))) && (s.v[1111] != 0.0)) {
            s.store_mul_ad_lhs(814, A::div(s.ad_value(805), s.ad_value(804)), 794);
        }

        if ((((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1110] != 0.0))) && (s.v[1111] != 0.0)) {
            s.store_div_ad_lhs(815, A::sub(A::mul(s.ad_value(806), s.ad_value(794)), A::mul(s.ad_value(805), A::add(s.ad_value(814), A::scale(s.ad_value(809), 0.5)))), 804);
        }

        if ((((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1110] != 0.0))) && (!(s.v[1111] != 0.0))) {
            s.store_scale_ad(795, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.025)))))), 0.1666666666667);
        }

        if ((((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1110] != 0.0))) && (!(s.v[1111] != 0.0))) {
            s.store_offset_ad(808, A::mul(s.ad_value(804), s.ad_value(795)), 2.0);
        }

        if ((((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1110] != 0.0))) && (!(s.v[1111] != 0.0))) {
            s.store_scale_ad(793, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0333333333333)))))), 0.1666666666667);
        }

        if ((((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1110] != 0.0))) && (!(s.v[1111] != 0.0))) {
            s.store_mul(809, 805, 793);
        }

        if ((((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1110] != 0.0))) && (!(s.v[1111] != 0.0))) {
            s.store_scale_ad(794, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0420875420875421)))))), 0.0055555555556);
        }

        if ((((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1110] != 0.0))) && (!(s.v[1111] != 0.0))) {
            s.store_sub_ad(811, A::mul(s.ad_value(806), s.ad_value(793)), A::mul(A::square(s.ad_value(805)), s.ad_value(794)));
        }

        if ((((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1110] != 0.0))) && (!(s.v[1111] != 0.0))) {
            s.store_mul_ad_lhs(814, A::scale(s.ad_value(805), (-0.5)), 795);
        }

        if ((((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1110] != 0.0))) && (!(s.v[1111] != 0.0))) {
            s.store_add_ad(815, A::mul(A::scale(s.ad_value(806), (-0.5)), s.ad_value(795)), A::mul(A::mul(A::scale(s.ad_value(805), (0.25 * 0.0055555555556)), s.ad_value(805)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(804), 0.075))))));
        }

    }

    pub(super) fn stamp_transient_block_10(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[1112] = if (s.v[804] > 0.005) { 1.0 } else { 0.0 };

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (s.v[1112] != 0.0)) {
            s.store_div_ad(794, A::scale(s.ad_value(804), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(810), A::sub_from_scalar(2.0, s.ad_value(810)))));
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (s.v[1112] != 0.0)) {
            s.store_mul(812, 794, 810);
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (s.v[1112] != 0.0)) {
            s.store_sub_ad_lhs(813, A::ln(s.ad_value(794)), 807);
        }

        s.v[1113] = if (s.v[804] < (-0.005)) { 1.0 } else { 0.0 };

        if ((((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1112] != 0.0))) && (s.v[1113] != 0.0)) {
            s.store_sin_ad(794, A::scale(s.ad_value(807), 0.5));
        }

        if ((((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1112] != 0.0))) && (s.v[1113] != 0.0)) {
            s.store_div_ad(812, A::neg(s.ad_value(804)), A::square(s.ad_value(794)));
        }

        if ((((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1112] != 0.0))) && (s.v[1113] != 0.0)) {
            s.store_ln(813, 812);
        }

        if ((((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1112] != 0.0))) && (!(s.v[1113] != 0.0))) {
            s.store_sub_from_scalar_ad(812, 4.0, A::mul(A::scale(s.ad_value(804), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0396825396825397))))));
        }

        if ((((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1112] != 0.0))) && (!(s.v[1113] != 0.0))) {
            s.store_ln(813, 812);
        }

        s.v[1114] = if (((1.01 * s.v[802]) + s.v[808]) > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (s.v[1114] != 0.0)) {
            s.store_add(816, 802, 808);
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (s.v[1114] != 0.0)) {
            s.store_add(817, 911, 809);
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (s.v[1114] != 0.0)) {
            s.copy_ad(818, 811);
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1114] != 0.0))) {
            s.store_div_from_scalar_ad(794, 1.0, A::sub(s.ad_value(802), s.ad_value(808)));
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1114] != 0.0))) {
            s.store_sub(795, 809, 911);
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1114] != 0.0))) {
            s.store_mul_ad_lhs(816, A::sub(s.ad_value(803), s.ad_value(812)), 794);
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1114] != 0.0))) {
            s.store_mul_ad_lhs(817, A::sub(A::sub(A::mul(s.ad_value(795), s.ad_value(816)), s.ad_value(803)), A::mul(s.ad_value(814), s.ad_value(812))), 794);
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1114] != 0.0))) {
            s.store_mul_ad_lhs(818, A::sub(A::add(A::add(A::mul(s.ad_value(811), s.ad_value(816)), A::mul(A::scale(s.ad_value(795), 2.0), s.ad_value(817))), s.ad_value(803)), A::mul(A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812))), 794);
        }

        s.v[1115] = if (s.v[816] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (s.v[1115] != 0.0)) {
            s.store_ln(819, 816);
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (s.v[1115] != 0.0)) {
            s.store_div_from_scalar(793, 1.0, 816);
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (s.v[1115] != 0.0)) {
            s.store_mul(820, 817, 793);
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (s.v[1115] != 0.0)) {
            s.store_sub_ad(821, A::mul(s.ad_value(818), s.ad_value(793)), A::square(s.ad_value(820)));
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1115] != 0.0))) {
            s.store_add_ad(819, A::offset(s.ad_value(802), 0.6931471805599), A::ln(A::neg(s.ad_value(802))));
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1115] != 0.0))) {
            s.store_div_from_scalar(793, 1.0, 916);
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1115] != 0.0))) {
            s.store_add(820, 911, 793);
        }

        if (((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) && (!(s.v[1115] != 0.0))) {
            s.store_mul_ad_lhs(821, A::neg(s.ad_value(793)), 793);
        }

        if ((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) {
            s.store_sub_ad_lhs(822, A::add(A::add(A::sub(s.ad_value(910), s.ad_value(909)), s.ad_value(916)), A::scale(s.ad_value(819), 2.0)), 813);
        }

        if ((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) {
            s.store_sub_ad_lhs(823, A::offset(A::scale(s.ad_value(820), 2.0), 1.0), 814);
        }

        if ((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) {
            s.store_sub_ad_lhs(824, A::scale(s.ad_value(821), 2.0), 815);
        }

        if ((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) {
            s.store_add_ad_rhs(825, 802, A::mul(s.ad_value(912), s.ad_value(822)));
        }

        if ((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) {
            s.store_add_ad_rhs(826, 911, A::mul(s.ad_value(912), s.ad_value(823)));
        }

        if ((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) {
            s.store_mul(827, 912, 824);
        }

        if ((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) {
            s.store_sub_ad_lhs(828, A::mul(s.ad_value(825), s.ad_value(816)), 803);
        }

        if ((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) {
            s.store_add_ad_lhs(829, A::add(A::mul(s.ad_value(826), s.ad_value(816)), A::mul(s.ad_value(825), s.ad_value(817))), 803);
        }

        if ((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) {
            s.store_sub_ad_lhs(830, A::add(A::add(A::mul(s.ad_value(827), s.ad_value(816)), A::mul(A::scale(s.ad_value(826), 2.0), s.ad_value(817))), A::mul(s.ad_value(825), s.ad_value(818))), 803);
        }

        if ((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) {
            s.store_sub_ad(839, A::square(s.ad_value(829)), A::mul(A::scale(s.ad_value(828), 0.5), s.ad_value(830)));
        }

        if ((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) {
            s.store_div_ad(831, A::mul(A::mul(A::neg(s.ad_value(828)), s.ad_value(829)), s.ad_value(839)), A::offset(A::square(s.ad_value(839)), 1e-200));
        }

        if ((s.v[1107] != 0.0) && (s.v[1108] != 0.0)) {
            s.store_add(916, 916, 831);
        }

        s.store_mul(918, 911, 916);

        s.v[1116] = if ((s.v[909] - s.v[916]) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1116] != 0.0) {
            s.store_exp_ad(793, A::sub(s.ad_value(909), s.ad_value(916)));
        }

        if (!(s.v[1116] != 0.0)) {
            s.store_scale_ad(793, A::offset(A::mul(A::offset(A::sub(s.ad_value(909), s.ad_value(916)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(909), s.ad_value(916)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(909), s.ad_value(916)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        s.store_mul(922, 884, 793);

        s.store_sub_ad_lhs(921, A::square(s.ad_value(918)), 922);

        s.v[1117] = if (s.v[922] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1117] != 0.0) {
            s.store_scalar(917, 1e-80);
        }

        if (s.v[1117] != 0.0) {
            s.store_sub(919, 917, 918);
        }

        if (s.v[1117] != 0.0) {
            s.store_div(920, 919, 912);
        }

        s.v[1118] = if (s.v[921] < (-0.005)) { 1.0 } else { 0.0 };

        if ((!(s.v[1117] != 0.0)) && (s.v[1118] != 0.0)) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(921)));
        }

        if ((!(s.v[1117] != 0.0)) && (s.v[1118] != 0.0)) {
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
        }

        s.v[1119] = if (s.v[921] > 0.005) { 1.0 } else { 0.0 };

        if (((!(s.v[1117] != 0.0)) && (!(s.v[1118] != 0.0))) && (s.v[1119] != 0.0)) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(921)));
        }

        if (((!(s.v[1117] != 0.0)) && (!(s.v[1118] != 0.0))) && (s.v[1119] != 0.0)) {
            s.store_exp_ad(810, A::neg(s.ad_value(807)));
        }

        if (((!(s.v[1117] != 0.0)) && (!(s.v[1118] != 0.0))) && (s.v[1119] != 0.0)) {
            s.store_div_ad(808, A::mul(s.ad_value(807), A::offset(s.ad_value(810), 1.0)), A::sub_from_scalar(1.0, s.ad_value(810)));
        }

        if (((!(s.v[1117] != 0.0)) && (!(s.v[1118] != 0.0))) && (!(s.v[1119] != 0.0))) {
            s.store_offset_ad(808, A::mul(A::scale(s.ad_value(921), 0.1666666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(921), 0.0166666666667), A::sub_from_scalar(1.0, A::scale(s.ad_value(921), 0.0238095238095))))), 2.0);
        }

        s.v[1120] = if (((1.01 * s.v[918]) + s.v[808]) > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1117] != 0.0)) && (s.v[1120] != 0.0)) {
            s.store_add(793, 918, 808);
        }

        s.v[1121] = if ((s.v[922] * s.v[918]) < (((0.9 * s.v[918]) * s.v[918]) * s.v[793])) { 1.0 } else { 0.0 };

        if (((!(s.v[1117] != 0.0)) && (s.v[1120] != 0.0)) && (s.v[1121] != 0.0)) {
            s.store_offset_ad(917, A::div(s.ad_value(922), s.ad_value(793)), 1e-80);
        }

        if (((!(s.v[1117] != 0.0)) && (s.v[1120] != 0.0)) && (s.v[1121] != 0.0)) {
            s.store_sub(919, 917, 918);
        }

        if (((!(s.v[1117] != 0.0)) && (s.v[1120] != 0.0)) && (s.v[1121] != 0.0)) {
            s.store_div(920, 919, 912);
        }

        s.v[1122] = if (s.v[921] > 0.005) { 1.0 } else { 0.0 };

        if ((((!(s.v[1117] != 0.0)) && (s.v[1120] != 0.0)) && (!(s.v[1121] != 0.0))) && (s.v[1122] != 0.0)) {
            s.store_sub_ad_lhs(794, A::ln(A::div(A::scale(s.ad_value(921), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(810), A::sub_from_scalar(2.0, s.ad_value(810)))))), 807);
        }

        s.v[1123] = if (s.v[921] < (-0.005)) { 1.0 } else { 0.0 };

        if (((((!(s.v[1117] != 0.0)) && (s.v[1120] != 0.0)) && (!(s.v[1121] != 0.0))) && (!(s.v[1122] != 0.0))) && (s.v[1123] != 0.0)) {
            s.store_sin_ad(795, A::scale(s.ad_value(807), 0.5));
        }

        if (((((!(s.v[1117] != 0.0)) && (s.v[1120] != 0.0)) && (!(s.v[1121] != 0.0))) && (!(s.v[1122] != 0.0))) && (s.v[1123] != 0.0)) {
            s.store_ln_ad(794, A::div(A::neg(s.ad_value(921)), A::square(s.ad_value(795))));
        }

        if (((((!(s.v[1117] != 0.0)) && (s.v[1120] != 0.0)) && (!(s.v[1121] != 0.0))) && (!(s.v[1122] != 0.0))) && (!(s.v[1123] != 0.0))) {
            s.store_ln_ad(794, A::sub_from_scalar(4.0, A::mul(A::scale(s.ad_value(921), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(921), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(921), 0.0396825396825397)))))));
        }

        if (((!(s.v[1117] != 0.0)) && (s.v[1120] != 0.0)) && (!(s.v[1121] != 0.0))) {
            s.store_sub_ad_lhs(920, A::add(A::add(A::sub(s.ad_value(910), s.ad_value(909)), s.ad_value(916)), A::scale(A::ln(s.ad_value(793)), 2.0)), 794);
        }

        if (((!(s.v[1117] != 0.0)) && (s.v[1120] != 0.0)) && (!(s.v[1121] != 0.0))) {
            s.store_mul(919, 912, 920);
        }

        if (((!(s.v[1117] != 0.0)) && (s.v[1120] != 0.0)) && (!(s.v[1121] != 0.0))) {
            s.store_add(917, 918, 919);
        }

        s.v[1124] = if (s.v[921] > 0.005) { 1.0 } else { 0.0 };

        s.v[1125] = if (((s.v[916] - s.v[909]) - s.v[807]) < 80.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1117] != 0.0)) && (!(s.v[1120] != 0.0))) && (s.v[1124] != 0.0)) && (s.v[1125] != 0.0)) {
            s.store_exp_ad(795, A::sub(A::sub(s.ad_value(916), s.ad_value(909)), s.ad_value(807)));
        }

        if ((((!(s.v[1117] != 0.0)) && (!(s.v[1120] != 0.0))) && (s.v[1124] != 0.0)) && (!(s.v[1125] != 0.0))) {
            let assign15460_ad_e15281: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(916), s.ad_value(909)), s.ad_value(807)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(s.ad_value(916), s.ad_value(909)), s.ad_value(807)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(916), s.ad_value(909)), s.ad_value(807)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(795, assign15460_ad_e15281, 5.54062e34);
        }

        if (((!(s.v[1117] != 0.0)) && (!(s.v[1120] != 0.0))) && (s.v[1124] != 0.0)) {
            s.store_div(794, 795, 884);
        }

        if (((!(s.v[1117] != 0.0)) && (!(s.v[1120] != 0.0))) && (s.v[1124] != 0.0)) {
            s.store_div_ad(793, A::mul(A::scale(s.ad_value(921), 4.0), s.ad_value(794)), A::sub_from_scalar(1.0, A::mul(s.ad_value(810), A::sub_from_scalar(2.0, s.ad_value(810)))));
        }

        s.v[1126] = if (s.v[921] < (-0.005)) { 1.0 } else { 0.0 };

        if ((((!(s.v[1117] != 0.0)) && (!(s.v[1120] != 0.0))) && (!(s.v[1124] != 0.0))) && (s.v[1126] != 0.0)) {
            s.store_sin_ad(794, A::scale(s.ad_value(807), 0.5));
        }

        if ((((!(s.v[1117] != 0.0)) && (!(s.v[1120] != 0.0))) && (!(s.v[1124] != 0.0))) && (s.v[1126] != 0.0)) {
            s.store_div_ad_lhs(793, A::div(A::neg(s.ad_value(921)), A::square(s.ad_value(794))), 922);
        }

        if ((((!(s.v[1117] != 0.0)) && (!(s.v[1120] != 0.0))) && (!(s.v[1124] != 0.0))) && (!(s.v[1126] != 0.0))) {
            s.store_div_ad_lhs(793, A::sub_from_scalar(4.0, A::mul(A::scale(s.ad_value(921), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(921), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(921), 0.0396825396825397)))))), 922);
        }

        if ((!(s.v[1117] != 0.0)) && (!(s.v[1120] != 0.0))) {
            s.store_offset_ad(917, A::div(A::sub(s.ad_value(918), s.ad_value(808)), A::sub_from_scalar(1.0, s.ad_value(793))), 1e-80);
        }

        if ((!(s.v[1117] != 0.0)) && (!(s.v[1120] != 0.0))) {
            s.store_sub(919, 917, 918);
        }

        if ((!(s.v[1117] != 0.0)) && (!(s.v[1120] != 0.0))) {
            s.store_div(920, 919, 912);
        }

        s.v[1127] = if ((s.v[910] - s.v[920]) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1127] != 0.0) {
            s.store_exp_ad(793, A::sub(s.ad_value(910), s.ad_value(920)));
        }

        if (!(s.v[1127] != 0.0)) {
            s.store_scale_ad(793, A::offset(A::mul(A::offset(A::sub(s.ad_value(910), s.ad_value(920)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(910), s.ad_value(920)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(910), s.ad_value(920)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        s.store_mul(923, 884, 793);

        s.v[926] = 0.0;

        s.v[927] = 0.0;

        s.v[924] = 0.0;

        s.v[925] = 0.0;

        s.v[928] = 0.0;

        s.v[929] = 0.0;

        s.v[1128] = if (s.v[917] > 1e-6) { 1.0 } else { 0.0 };

        if (s.v[1128] != 0.0) {
            s.store_mul(924, 922, 885);
        }

        if (s.v[1128] != 0.0) {
            s.store_mul(925, 923, 886);
        }

        if (s.v[1128] != 0.0) {
            s.store_add_ad_rhs(926, 924, A::scale(s.ad_value(918), 2.0));
        }

        if (s.v[1128] != 0.0) {
            s.store_add_ad_rhs(927, 925, A::scale(s.ad_value(919), 2.0));
        }

        if (s.v[1128] != 0.0) {
            s.store_add_ad_lhs(928, A::add(A::scale(s.ad_value(917), 2.0), s.ad_value(924)), 925);
        }

        s.v[1129] = if (((s.v[921]) as f64).abs() > 0.005) { 1.0 } else { 0.0 };

        if ((s.v[1128] != 0.0) && (s.v[1129] != 0.0)) {
            s.store_add_ad(2, A::add(A::mul(s.ad_value(926), s.ad_value(927)), A::mul(A::scale(A::offset(s.ad_value(916), 2.0), 2.0), s.ad_value(927))), A::mul(A::scale(A::offset(s.ad_value(920), 2.0), 2.0), s.ad_value(926)));
        }

        if ((s.v[1128] != 0.0) && (s.v[1129] != 0.0)) {
            s.store_div_ad(929, A::mul(A::scale(s.ad_value(921), (-4.0)), s.ad_value(928)), A::mul(s.ad_value(917), s.ad_value(2)));
        }

        if ((s.v[1128] != 0.0) && (!(s.v[1129] != 0.0))) {
            s.store_scale_ad(2, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(921), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(921), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(921), 0.0333333333333)))))), 0.1666666666667);
        }

        if ((s.v[1128] != 0.0) && (!(s.v[1129] != 0.0))) {
            s.store_add_ad(3, A::add(A::mul(s.ad_value(926), s.ad_value(922)), A::mul(s.ad_value(927), s.ad_value(923))), A::mul(A::mul(A::mul(s.ad_value(926), s.ad_value(927)), s.ad_value(917)), A::offset(A::mul(s.ad_value(917), s.ad_value(2)), 1.0)));
        }

        if ((s.v[1128] != 0.0) && (!(s.v[1129] != 0.0))) {
            s.store_div_ad(929, A::mul(A::mul(s.ad_value(922), s.ad_value(923)), s.ad_value(928)), A::mul(s.ad_value(917), s.ad_value(3)));
        }

        s.store_ln(930, 917);

        s.v[1130] = if ((s.v[918] / 2.0) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1130] != 0.0) {
            s.store_ln_ad(2, A::offset(A::exp(A::scale(s.ad_value(918), 0.5)), 1.0));
        }

        if (!(s.v[1130] != 0.0)) {
            s.store_scale(2, 918, 0.5);
        }

        s.store_scale(931, 2, 2.0);

        s.v[1131] = if ((s.v[919] / 2.0) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1131] != 0.0) {
            s.store_ln_ad(3, A::offset(A::exp(A::scale(s.ad_value(919), 0.5)), 1.0));
        }

        if (!(s.v[1131] != 0.0)) {
            s.store_scale(3, 919, 0.5);
        }

        s.store_scale(932, 3, 2.0);

        s.store_sub(933, 932, 919);

        s.store_sub(934, 931, 918);

        s.store_add_ad(935, A::mul(s.ad_value(270), s.ad_value(931)), A::mul(s.ad_value(271), s.ad_value(933)));

        s.store_add_ad(936, A::mul(s.ad_value(270), s.ad_value(932)), A::mul(s.ad_value(271), s.ad_value(934)));

        s.store_div_ad_rhs(0, 917, A::add(s.ad_value(931), s.ad_value(932)));

        s.store_mul(937, 931, 0);

        s.store_mul(938, 932, 0);

        s.store_mul_ad(939, A::mul(s.ad_value(931), s.ad_value(191)), A::exp(A::mul(s.ad_value(40), s.ad_value(295))));

        s.store_mul_ad(940, A::mul(s.ad_value(932), s.ad_value(192)), A::exp(A::mul(s.ad_value(40), s.ad_value(295))));

        s.store_mul_ad_rhs(2, 50, A::add(s.ad_value(933), A::mul(s.ad_value(51), s.ad_value(934))));

        s.store_scale_ad(3, A::add(A::offset(s.ad_value(2), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2), 1.0), A::offset(s.ad_value(2), 1.0)), 0.01))), 0.5);

        s.store_scale_ad(4, A::add(A::offset(A::scale(s.ad_value(2), 0.2), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::scale(s.ad_value(2), 0.2), 1.0), A::offset(A::scale(s.ad_value(2), 0.2), 1.0)), 0.01))), 0.5);

        s.store_div(941, 3, 4);

        s.store_mul_ad(942, A::mul(s.ad_value(33), A::add(A::offset(A::mul(s.ad_value(41), s.ad_value(933)), 1.0), A::mul(s.ad_value(42), s.ad_value(934)))), A::exp(A::mul(A::neg(s.ad_value(44)), A::ln(A::add(A::offset(A::mul(s.ad_value(937), s.ad_value(268)), 1.0), A::mul(s.ad_value(938), s.ad_value(269)))))));

        s.v[1132] = if (s.v[56] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1132] != 0.0) {
            s.store_scalar(4, 1.0);
        }

        s.v[1133] = if (s.v[56] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1132] != 0.0)) && (s.v[1133] != 0.0)) {
            s.store_mul_ad_rhs(2, 56, A::exp(A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(917), 1e-12)))));
        }

        if ((!(s.v[1132] != 0.0)) && (s.v[1133] != 0.0)) {
            s.store_sub_from_scalar(4, 1.0, 2);
        }

        if ((!(s.v[1132] != 0.0)) && (!(s.v[1133] != 0.0))) {
            s.store_mul_ad_rhs(2, 56, A::exp(A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(917), 1e-12)))));
        }

        if ((!(s.v[1132] != 0.0)) && (!(s.v[1133] != 0.0))) {
            s.store_div_from_scalar_ad(4, 1.0, A::offset(s.ad_value(2), 1.0));
        }

        s.store_mul_ad(943, A::scale(A::mul(s.ad_value(272), s.ad_value(898)), 0.5), A::add(A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(879))), A::sqrt(A::offset(A::mul(A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(879))), A::sub_from_scalar(1.0, A::mul(s.ad_value(58), s.ad_value(879)))), 0.01))));

        s.store_mul_ad_rhs(944, 943, A::add(A::mul(s.ad_value(917), s.ad_value(4)), s.ad_value(54)));

        s.store_add_ad(945, A::add(A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(935)), 1e-6)))), 1.0), s.ad_value(942)), A::mul(s.ad_value(38), s.ad_value(944)));

        s.store_add_ad(946, A::add(A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(936)), 1e-6)))), 1.0), s.ad_value(942)), A::mul(s.ad_value(39), s.ad_value(944)));

        s.store_div_ad(947, A::mul(s.ad_value(941), A::add(s.ad_value(939), s.ad_value(940))), A::add(A::div(s.ad_value(939), s.ad_value(945)), A::div(s.ad_value(940), s.ad_value(946))));

        s.v[1134] = if (((s.v[914]) as f64).abs() > 0.007) { 1.0 } else { 0.0 };

        s.v[1135] = if (s.v[914] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1134] != 0.0) && (s.v[1135] != 0.0)) {
            s.store_exp_ad(0, A::neg(s.ad_value(914)));
        }

        if ((s.v[1134] != 0.0) && (s.v[1135] != 0.0)) {
            s.store_div_ad_rhs(948, 914, A::sub_from_scalar(1.0, s.ad_value(0)));
        }

        if ((s.v[1134] != 0.0) && (s.v[1135] != 0.0)) {
            s.store_mul(949, 0, 948);
        }

        if ((s.v[1134] != 0.0) && (s.v[1135] != 0.0)) {
            s.store_add_ad_lhs(950, A::offset(A::ln(A::div(s.ad_value(884), A::mul(s.ad_value(917), s.ad_value(948)))), (-0.6931471805599)), 887);
        }

        if ((s.v[1134] != 0.0) && (!(s.v[1135] != 0.0))) {
            s.store_exp(0, 914);
        }

        if ((s.v[1134] != 0.0) && (!(s.v[1135] != 0.0))) {
            s.store_div_ad_rhs(949, 914, A::offset(s.ad_value(0), (-1.0)));
        }

        if ((s.v[1134] != 0.0) && (!(s.v[1135] != 0.0))) {
            s.store_mul(948, 0, 949);
        }

        if ((s.v[1134] != 0.0) && (!(s.v[1135] != 0.0))) {
            s.store_add_ad_lhs(950, A::offset(A::ln(A::div(s.ad_value(884), A::mul(s.ad_value(917), s.ad_value(949)))), (-0.6931471805599)), 888);
        }

        if (s.v[1134] != 0.0) {
            s.store_div_ad(951, A::neg(s.ad_value(914)), A::mul(s.ad_value(913), A::sub(A::sub_from_scalar(1.0, s.ad_value(948)), A::mul(s.ad_value(914), s.ad_value(886)))));
        }

        if (s.v[1134] != 0.0) {
            s.store_div_ad_rhs(952, 914, A::mul(s.ad_value(913), A::add(A::sub_from_scalar(1.0, s.ad_value(949)), A::mul(s.ad_value(914), s.ad_value(885)))));
        }

    }

    pub(super) fn stamp_transient_block_11(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1134] != 0.0) {
            s.store_div_ad_rhs(953, 914, A::sub(A::div(A::offset(A::mul(s.ad_value(949), s.ad_value(886)), 0.5), s.ad_value(952)), A::div(A::offset(A::mul(s.ad_value(948), s.ad_value(885)), 0.5), s.ad_value(951))));
        }

        if (!(s.v[1134] != 0.0)) {
            s.store_scale(0, 915, (0.5 * 0.1666666666667));
        }

        if (!(s.v[1134] != 0.0)) {
            s.store_scale(2, 914, 0.5);
        }

        if (!(s.v[1134] != 0.0)) {
            s.store_add_ad_lhs(948, A::offset(s.ad_value(2), 1.0), 0);
        }

        if (!(s.v[1134] != 0.0)) {
            s.store_add_ad_lhs(949, A::sub_from_scalar(1.0, s.ad_value(2)), 0);
        }

        if (!(s.v[1134] != 0.0)) {
            s.store_scale(3, 2, 0.1666666666667);
        }

        if (!(s.v[1134] != 0.0)) {
            s.store_div_from_scalar_ad(951, 1.0, A::mul(s.ad_value(913), A::add(A::offset(s.ad_value(886), 0.5), s.ad_value(3))));
        }

        if (!(s.v[1134] != 0.0)) {
            s.store_div_from_scalar_ad(952, 1.0, A::mul(s.ad_value(913), A::sub(A::offset(s.ad_value(885), 0.5), s.ad_value(3))));
        }

        if (!(s.v[1134] != 0.0)) {
            s.store_add_ad(950, A::offset(A::ln(A::div(s.ad_value(884), A::mul(s.ad_value(917), A::sub_from_scalar(1.0, A::scale(s.ad_value(0), 0.5))))), (-0.6931471805599)), A::scale(A::add(s.ad_value(887), s.ad_value(888)), 0.5));
        }

        if (!(s.v[1134] != 0.0)) {
            let assign16340_ad_e16237: A = A::add(A::add(A::add(A::sub_from_scalar(4.0, A::scale(s.ad_value(913), 3.0)), A::div(A::scale(s.ad_value(913), 12.0), A::mul(s.ad_value(911), s.ad_value(912)))), A::mul(A::mul(s.ad_value(913), A::sub(s.ad_value(885), s.ad_value(886))), s.ad_value(914))), A::mul(A::scale(A::sub_from_scalar(0.2, A::scale(s.ad_value(913), 0.25)), 0.3333333333333), s.ad_value(915)));
            s.store_div_from_scalar_ad(953, (-12.0), assign16340_ad_e16237);
        }

        s.store_div_from_scalar(954, 1.0, 953);

        s.v[1136] = if (s.v[917] > 1e-6) { 1.0 } else { 0.0 };

        if (s.v[1136] != 0.0) {
            s.store_div_ad(955, A::scale(s.ad_value(931), 100.0), A::offset(s.ad_value(931), 100.0));
        }

        s.v[1137] = if (s.v[61] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1136] != 0.0) && (s.v[1137] != 0.0)) {
            s.store_div_from_scalar_ad(956, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(61), s.ad_value(955))));
        }

        if ((s.v[1136] != 0.0) && (!(s.v[1137] != 0.0))) {
            s.store_offset_ad(956, A::mul(s.ad_value(61), s.ad_value(955)), 1.0);
        }

        if (s.v[1136] != 0.0) {
            s.store_div_ad(957, A::scale(s.ad_value(932), 100.0), A::offset(s.ad_value(932), 100.0));
        }

        s.v[1138] = if (s.v[62] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1136] != 0.0) && (s.v[1138] != 0.0)) {
            s.store_div_from_scalar_ad(958, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(62), s.ad_value(957))));
        }

        if ((s.v[1136] != 0.0) && (!(s.v[1138] != 0.0))) {
            s.store_offset_ad(958, A::mul(s.ad_value(62), s.ad_value(957)), 1.0);
        }

        if (s.v[1136] != 0.0) {
            s.store_sub_ad(959, A::div(A::mul(s.ad_value(929), s.ad_value(928)), A::mul(s.ad_value(926), s.ad_value(927))), A::div(A::add(A::div(s.ad_value(922), s.ad_value(926)), A::div(s.ad_value(923), s.ad_value(927))), s.ad_value(917)));
        }

        if (s.v[1136] != 0.0) {
            s.store_div_ad(960, A::mul(s.ad_value(959), s.ad_value(917)), A::offset(s.ad_value(959), 1.0));
        }

        if (s.v[1136] != 0.0) {
            s.store_sub(2, 953, 960);
        }

        if (s.v[1136] != 0.0) {
            s.store_div_ad_lhs(961, A::add(s.ad_value(917), A::mul(s.ad_value(953), s.ad_value(950))), 2);
        }

        if (s.v[1136] != 0.0) {
            s.store_scale_ad(961, A::add(s.ad_value(961), A::sqrt(A::offset(A::square(s.ad_value(961)), 1e-6))), 0.5);
        }

        if (s.v[1136] != 0.0) {
            s.store_mul_ad(962, A::scale(A::div(s.ad_value(875), s.ad_value(947)), 0.5), A::add(s.ad_value(956), s.ad_value(958)));
        }

        if (s.v[1136] != 0.0) {
            s.store_sub_from_scalar_ad(963, 1.0, A::div(s.ad_value(917), s.ad_value(960)));
        }

        if (s.v[1136] != 0.0) {
            s.store_offset(964, 950, 1.0);
        }

        if (s.v[1136] != 0.0) {
            s.store_mul_ad_lhs(965, A::sub(A::offset(A::mul(A::sub(A::scale(s.ad_value(960), 2.0), s.ad_value(917)), s.ad_value(954)), (-2.0)), s.ad_value(950)), 961);
        }

        s.v[1139] = if (s.v[962] > 1e-14) { 1.0 } else { 0.0 };

        if ((s.v[1136] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_div_from_scalar_ad(966, 2.0, A::square(s.ad_value(962)));
        }

        if ((s.v[1136] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_mul(967, 966, 963);
        }

        if ((s.v[1136] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_add(968, 966, 965);
        }

        if ((s.v[1136] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_mul(969, 966, 964);
        }

        if ((s.v[1136] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_sqrt_ad(970, A::offset(A::add(A::square(s.ad_value(967)), A::mul(A::mul(A::scale(s.ad_value(966), 0.148148148148), s.ad_value(966)), s.ad_value(966))), 1e-20));
        }

        if ((s.v[1136] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_sqrt_ad(971, A::offset(A::add(A::square(s.ad_value(969)), A::mul(A::mul(A::scale(s.ad_value(968), 0.148148148148), s.ad_value(968)), s.ad_value(968))), 1e-20));
        }

        if ((s.v[1136] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_sub_ad(972, A::exp(A::scale(A::ln(A::scale(A::add(s.ad_value(970), s.ad_value(967)), 0.5)), 0.3333333333333)), A::exp(A::scale(A::ln(A::scale(A::sub(s.ad_value(970), s.ad_value(967)), 0.5)), 0.3333333333333)));
        }

        if ((s.v[1136] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_sub_ad(973, A::exp(A::scale(A::ln(A::scale(A::add(s.ad_value(971), s.ad_value(969)), 0.5)), 0.3333333333333)), A::exp(A::scale(A::ln(A::scale(A::sub(s.ad_value(971), s.ad_value(969)), 0.5)), 0.3333333333333)));
        }

        if ((s.v[1136] != 0.0) && (!(s.v[1139] != 0.0))) {
            s.copy_ad(972, 963);
        }

        if ((s.v[1136] != 0.0) && (!(s.v[1139] != 0.0))) {
            s.copy_ad(973, 964);
        }

        if (s.v[1136] != 0.0) {
            s.store_square(4, 2);
        }

        if (s.v[1136] != 0.0) {
            s.store_scale_ad(974, A::add(A::add(s.ad_value(972), s.ad_value(973)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(972), s.ad_value(973)), A::sub(s.ad_value(972), s.ad_value(973))), A::scale(s.ad_value(4), 10.0)))), (0.94 * 0.5));
        }

        if (s.v[1136] != 0.0) {
            s.store_add_ad_rhs(975, 917, A::mul(s.ad_value(960), s.ad_value(974)));
        }

        if (s.v[1136] != 0.0) {
            s.store_mul_ad_rhs(976, 953, A::sub(s.ad_value(974), s.ad_value(950)));
        }

        if (s.v[1136] != 0.0) {
            s.store_scale_ad(977, A::add(A::add(s.ad_value(975), s.ad_value(976)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(975), s.ad_value(976)), A::sub(s.ad_value(975), s.ad_value(976))), A::scale(s.ad_value(4), 36.0)))), 0.5);
        }

        if (!(s.v[1136] != 0.0)) {
            s.copy_ad(960, 953);
        }

        if (!(s.v[1136] != 0.0)) {
            s.store_scaled_offset(974, 950, 1.0, 0.94);
        }

        if (!(s.v[1136] != 0.0)) {
            s.store_add_ad(977, A::scale(s.ad_value(917), 0.5), A::mul(s.ad_value(953), A::sub(s.ad_value(974), A::scale(s.ad_value(950), 0.5))));
        }

        s.v[1140] = if ((s.v[977] - 0.5) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1140] != 0.0) {
            s.store_ln_ad(2, A::offset(A::exp(A::offset(s.ad_value(977), (-0.5))), 1.0));
        }

        if (!(s.v[1140] != 0.0)) {
            s.store_offset(2, 977, (-0.5));
        }

        s.store_offset(3, 2, 0.5);

        s.store_add_ad_rhs(4, 974, A::ln(A::div(s.ad_value(917), s.ad_value(3))));

        s.v[1141] = if ((s.v[4] - 6.0) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1141] != 0.0) {
            s.store_ln_ad(2, A::offset(A::exp(A::offset(s.ad_value(4), (-6.0))), 1.0));
        }

        if (!(s.v[1141] != 0.0)) {
            s.store_offset(2, 4, (-6.0));
        }

        s.store_offset(4, 2, 6.0);

        s.v[1142] = if ((s.v[225] - s.v[4]) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1142] != 0.0) {
            s.store_ln_ad(2, A::offset(A::exp(A::sub(s.ad_value(225), s.ad_value(4))), 1.0));
        }

        if (!(s.v[1142] != 0.0)) {
            s.store_sub(2, 225, 4);
        }

        s.store_sub(978, 225, 2);

        s.store_div(2, 339, 978);

        s.store_square(3, 2);

        s.store_square(4, 3);

        s.store_square(5, 4);

        s.store_exp_ad(0, A::scale(A::ln(A::offset(A::mul(s.ad_value(876), s.ad_value(4)), 1.0)), 2.666666666667));

        s.store_mul_ad_rhs(979, 339, A::exp(A::scale(A::ln(A::add(s.ad_value(0), A::square(s.ad_value(5)))), (-0.0625))));

        s.store_div_from_scalar_ad(793, 1.0, A::offset(s.ad_value(911), 1.0));

        s.store_div_from_scalar_ad(794, 1.0, A::offset(s.ad_value(912), 1.0));

        s.store_offset_ad(796, A::add(A::ln(A::div(A::mul(A::add(s.ad_value(911), A::mul(s.ad_value(912), s.ad_value(794))), s.ad_value(883)), s.ad_value(884))), s.ad_value(979)), 3.0);

        s.store_offset_ad(797, A::add(A::ln(A::div(A::mul(A::add(s.ad_value(912), A::mul(s.ad_value(911), s.ad_value(793))), s.ad_value(883)), s.ad_value(884))), s.ad_value(979)), 3.0);

        s.v[1143] = if (((s.v[796] - s.v[887]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1143] != 0.0) {
            s.store_ln_ad(795, A::offset(A::exp(A::scale(A::sub(s.ad_value(796), s.ad_value(887)), 0.3333333333333)), 1.0));
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scaled_sub(795, 796, 887, 0.3333333333333);
        }

        s.store_sub_ad_rhs(800, 796, A::scale(s.ad_value(795), 3.0));

        s.v[1144] = if (((s.v[797] - s.v[888]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1144] != 0.0) {
            s.store_ln_ad(795, A::offset(A::exp(A::scale(A::sub(s.ad_value(797), s.ad_value(888)), 0.3333333333333)), 1.0));
        }

        if (!(s.v[1144] != 0.0)) {
            s.store_scaled_sub(795, 797, 888, 0.3333333333333);
        }

        s.store_sub_ad_rhs(801, 797, A::scale(s.ad_value(795), 3.0));

        s.store_mul_ad_lhs(798, A::add(A::mul(s.ad_value(911), s.ad_value(909)), s.ad_value(801)), 793);

        s.store_mul_ad_lhs(799, A::add(A::mul(s.ad_value(912), s.ad_value(910)), s.ad_value(800)), 794);

        s.v[1145] = if (((s.v[796] - s.v[798]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1145] != 0.0) {
            s.store_ln_ad(795, A::offset(A::exp(A::scale(A::sub(s.ad_value(796), s.ad_value(798)), 0.3333333333333)), 1.0));
        }

        if (!(s.v[1145] != 0.0)) {
            s.store_scaled_sub(795, 796, 798, 0.3333333333333);
        }

        s.store_sub_ad_rhs(800, 796, A::scale(s.ad_value(795), 3.0));

        s.v[1146] = if (((s.v[797] - s.v[799]) * 0.3333333333333) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1146] != 0.0) {
            s.store_ln_ad(795, A::offset(A::exp(A::scale(A::sub(s.ad_value(797), s.ad_value(799)), 0.3333333333333)), 1.0));
        }

        if (!(s.v[1146] != 0.0)) {
            s.store_scaled_sub(795, 797, 799, 0.3333333333333);
        }

        s.store_sub_ad_rhs(801, 797, A::scale(s.ad_value(795), 3.0));

        s.store_sub(980, 909, 800);

        s.store_sub(981, 910, 801);

        s.v[807] = 0.0;

        s.v[810] = 0.0;

        s.store_mul(802, 911, 980);

        s.v[1147] = if (((s.v[909] - s.v[980]) - s.v[979]) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1147] != 0.0) {
            s.store_exp_ad(793, A::sub(A::sub(s.ad_value(909), s.ad_value(980)), s.ad_value(979)));
        }

        if (!(s.v[1147] != 0.0)) {
            let assign17210_ad_e17012: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(909), s.ad_value(980)), s.ad_value(979)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(s.ad_value(909), s.ad_value(980)), s.ad_value(979)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(909), s.ad_value(980)), s.ad_value(979)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(793, assign17210_ad_e17012, 5.54062e34);
        }

        s.store_mul(803, 884, 793);

        s.store_sub_ad_lhs(804, A::square(s.ad_value(802)), 803);

        s.store_add_ad_lhs(805, A::mul(A::scale(s.ad_value(911), 2.0), s.ad_value(802)), 803);

        s.store_sub_ad_lhs(806, A::mul(A::scale(s.ad_value(911), 2.0), s.ad_value(911)), 803);

        s.v[1148] = if (s.v[804] < (-0.005)) { 1.0 } else { 0.0 };

        if (s.v[1148] != 0.0) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(804)));
        }

        if (s.v[1148] != 0.0) {
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
        }

        if (s.v[1148] != 0.0) {
            s.store_div_ad_lhs(793, A::scale(s.ad_value(805), 0.25), 804);
        }

        if (s.v[1148] != 0.0) {
            s.store_mul_ad_lhs(809, A::add(s.ad_value(804), A::mul(s.ad_value(808), A::sub_from_scalar(2.0, s.ad_value(808)))), 793);
        }

        if (s.v[1148] != 0.0) {
            s.store_add_ad(811, A::mul(A::sub(s.ad_value(805), A::mul(A::scale(s.ad_value(809), 2.0), A::offset(s.ad_value(808), 1.0))), s.ad_value(793)), A::div(A::mul(s.ad_value(809), s.ad_value(806)), s.ad_value(805)));
        }

        if (s.v[1148] != 0.0) {
            s.store_sub_from_scalar_ad(794, 1.0, A::scale(s.ad_value(808), 0.5));
        }

        if (s.v[1148] != 0.0) {
            s.store_mul_ad_lhs(814, A::div(s.ad_value(805), s.ad_value(804)), 794);
        }

        if (s.v[1148] != 0.0) {
            s.store_div_ad_lhs(815, A::sub(A::mul(s.ad_value(806), s.ad_value(794)), A::mul(s.ad_value(805), A::add(s.ad_value(814), A::scale(s.ad_value(809), 0.5)))), 804);
        }

        s.v[1149] = if (s.v[804] > 0.005) { 1.0 } else { 0.0 };

        if ((!(s.v[1148] != 0.0)) && (s.v[1149] != 0.0)) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(804)));
        }

        if ((!(s.v[1148] != 0.0)) && (s.v[1149] != 0.0)) {
            s.store_exp_ad(810, A::neg(s.ad_value(807)));
        }

        if ((!(s.v[1148] != 0.0)) && (s.v[1149] != 0.0)) {
            s.store_div_ad(808, A::mul(s.ad_value(807), A::offset(s.ad_value(810), 1.0)), A::sub_from_scalar(1.0, s.ad_value(810)));
        }

        if ((!(s.v[1148] != 0.0)) && (s.v[1149] != 0.0)) {
            s.store_div_ad_lhs(793, A::scale(s.ad_value(805), 0.25), 804);
        }

        if ((!(s.v[1148] != 0.0)) && (s.v[1149] != 0.0)) {
            s.store_mul_ad_lhs(809, A::add(s.ad_value(804), A::mul(s.ad_value(808), A::sub_from_scalar(2.0, s.ad_value(808)))), 793);
        }

        if ((!(s.v[1148] != 0.0)) && (s.v[1149] != 0.0)) {
            s.store_add_ad(811, A::mul(A::sub(s.ad_value(805), A::mul(A::scale(s.ad_value(809), 2.0), A::offset(s.ad_value(808), 1.0))), s.ad_value(793)), A::div(A::mul(s.ad_value(809), s.ad_value(806)), s.ad_value(805)));
        }

        if ((!(s.v[1148] != 0.0)) && (s.v[1149] != 0.0)) {
            s.store_sub_from_scalar_ad(794, 1.0, A::scale(s.ad_value(808), 0.5));
        }

        if ((!(s.v[1148] != 0.0)) && (s.v[1149] != 0.0)) {
            s.store_mul_ad_lhs(814, A::div(s.ad_value(805), s.ad_value(804)), 794);
        }

        if ((!(s.v[1148] != 0.0)) && (s.v[1149] != 0.0)) {
            s.store_div_ad_lhs(815, A::sub(A::mul(s.ad_value(806), s.ad_value(794)), A::mul(s.ad_value(805), A::add(s.ad_value(814), A::scale(s.ad_value(809), 0.5)))), 804);
        }

        if ((!(s.v[1148] != 0.0)) && (!(s.v[1149] != 0.0))) {
            s.store_scale_ad(795, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.025)))))), 0.1666666666667);
        }

        if ((!(s.v[1148] != 0.0)) && (!(s.v[1149] != 0.0))) {
            s.store_offset_ad(808, A::mul(s.ad_value(804), s.ad_value(795)), 2.0);
        }

        if ((!(s.v[1148] != 0.0)) && (!(s.v[1149] != 0.0))) {
            s.store_scale_ad(793, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0333333333333)))))), 0.1666666666667);
        }

        if ((!(s.v[1148] != 0.0)) && (!(s.v[1149] != 0.0))) {
            s.store_mul(809, 805, 793);
        }

        if ((!(s.v[1148] != 0.0)) && (!(s.v[1149] != 0.0))) {
            s.store_scale_ad(794, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0420875420875421)))))), 0.0055555555556);
        }

        if ((!(s.v[1148] != 0.0)) && (!(s.v[1149] != 0.0))) {
            s.store_sub_ad(811, A::mul(s.ad_value(806), s.ad_value(793)), A::mul(A::square(s.ad_value(805)), s.ad_value(794)));
        }

        if ((!(s.v[1148] != 0.0)) && (!(s.v[1149] != 0.0))) {
            s.store_mul_ad_lhs(814, A::scale(s.ad_value(805), (-0.5)), 795);
        }

        if ((!(s.v[1148] != 0.0)) && (!(s.v[1149] != 0.0))) {
            s.store_add_ad(815, A::mul(A::scale(s.ad_value(806), (-0.5)), s.ad_value(795)), A::mul(A::mul(A::scale(s.ad_value(805), (0.25 * 0.0055555555556)), s.ad_value(805)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(804), 0.075))))));
        }

        s.v[1150] = if (s.v[804] > 0.005) { 1.0 } else { 0.0 };

        if (s.v[1150] != 0.0) {
            s.store_div_ad(794, A::scale(s.ad_value(804), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(810), A::sub_from_scalar(2.0, s.ad_value(810)))));
        }

        if (s.v[1150] != 0.0) {
            s.store_mul(812, 794, 810);
        }

        if (s.v[1150] != 0.0) {
            s.store_sub_ad_lhs(813, A::ln(s.ad_value(794)), 807);
        }

        s.v[1151] = if (s.v[804] < (-0.005)) { 1.0 } else { 0.0 };

        if ((!(s.v[1150] != 0.0)) && (s.v[1151] != 0.0)) {
            s.store_sin_ad(794, A::scale(s.ad_value(807), 0.5));
        }

        if ((!(s.v[1150] != 0.0)) && (s.v[1151] != 0.0)) {
            s.store_div_ad(812, A::neg(s.ad_value(804)), A::square(s.ad_value(794)));
        }

        if ((!(s.v[1150] != 0.0)) && (s.v[1151] != 0.0)) {
            s.store_ln(813, 812);
        }

        if ((!(s.v[1150] != 0.0)) && (!(s.v[1151] != 0.0))) {
            s.store_sub_from_scalar_ad(812, 4.0, A::mul(A::scale(s.ad_value(804), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0396825396825397))))));
        }

        if ((!(s.v[1150] != 0.0)) && (!(s.v[1151] != 0.0))) {
            s.store_ln(813, 812);
        }

        s.v[1152] = if (((1.01 * s.v[802]) + s.v[808]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1152] != 0.0) {
            s.store_add(816, 802, 808);
        }

        if (s.v[1152] != 0.0) {
            s.store_add(817, 911, 809);
        }

        if (s.v[1152] != 0.0) {
            s.copy_ad(818, 811);
        }

        if (!(s.v[1152] != 0.0)) {
            s.store_div_from_scalar_ad(794, 1.0, A::sub(s.ad_value(802), s.ad_value(808)));
        }

        if (!(s.v[1152] != 0.0)) {
            s.store_sub(795, 809, 911);
        }

        if (!(s.v[1152] != 0.0)) {
            s.store_mul_ad_lhs(816, A::sub(s.ad_value(803), s.ad_value(812)), 794);
        }

        if (!(s.v[1152] != 0.0)) {
            s.store_mul_ad_lhs(817, A::sub(A::sub(A::mul(s.ad_value(795), s.ad_value(816)), s.ad_value(803)), A::mul(s.ad_value(814), s.ad_value(812))), 794);
        }

        if (!(s.v[1152] != 0.0)) {
            s.store_mul_ad_lhs(818, A::sub(A::add(A::add(A::mul(s.ad_value(811), s.ad_value(816)), A::mul(A::scale(s.ad_value(795), 2.0), s.ad_value(817))), s.ad_value(803)), A::mul(A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812))), 794);
        }

        s.v[1153] = if (s.v[816] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1153] != 0.0) {
            s.store_ln(819, 816);
        }

        if (s.v[1153] != 0.0) {
            s.store_div_from_scalar(793, 1.0, 816);
        }

        if (s.v[1153] != 0.0) {
            s.store_mul(820, 817, 793);
        }

    }

    pub(super) fn stamp_transient_block_12(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1153] != 0.0) {
            s.store_sub_ad(821, A::mul(s.ad_value(818), s.ad_value(793)), A::square(s.ad_value(820)));
        }

        if (!(s.v[1153] != 0.0)) {
            s.store_add_ad(819, A::offset(s.ad_value(802), 0.6931471805599), A::ln(A::neg(s.ad_value(802))));
        }

        if (!(s.v[1153] != 0.0)) {
            s.store_div_from_scalar(793, 1.0, 980);
        }

        if (!(s.v[1153] != 0.0)) {
            s.store_add(820, 911, 793);
        }

        if (!(s.v[1153] != 0.0)) {
            s.store_mul_ad_lhs(821, A::neg(s.ad_value(793)), 793);
        }

        s.store_sub_ad_lhs(822, A::add(A::add(A::sub(s.ad_value(910), s.ad_value(909)), s.ad_value(980)), A::scale(s.ad_value(819), 2.0)), 813);

        s.store_sub_ad_lhs(823, A::offset(A::scale(s.ad_value(820), 2.0), 1.0), 814);

        s.store_sub_ad_lhs(824, A::scale(s.ad_value(821), 2.0), 815);

        s.store_add_ad_rhs(825, 802, A::mul(s.ad_value(912), s.ad_value(822)));

        s.store_add_ad_rhs(826, 911, A::mul(s.ad_value(912), s.ad_value(823)));

        s.store_mul(827, 912, 824);

        s.store_sub_ad_lhs(828, A::mul(s.ad_value(825), s.ad_value(816)), 803);

        s.store_add_ad_lhs(829, A::add(A::mul(s.ad_value(826), s.ad_value(816)), A::mul(s.ad_value(825), s.ad_value(817))), 803);

        s.store_sub_ad_lhs(830, A::add(A::add(A::mul(s.ad_value(827), s.ad_value(816)), A::mul(A::scale(s.ad_value(826), 2.0), s.ad_value(817))), A::mul(s.ad_value(825), s.ad_value(818))), 803);

        s.store_sub_ad(839, A::square(s.ad_value(829)), A::mul(A::scale(s.ad_value(828), 0.5), s.ad_value(830)));

        s.store_div_ad(831, A::mul(A::mul(A::neg(s.ad_value(828)), s.ad_value(829)), s.ad_value(839)), A::offset(A::square(s.ad_value(839)), 1e-200));

        s.store_add(980, 980, 831);

        s.store_mul(802, 911, 980);

        s.store_mul(832, 912, 981);

        s.store_add(825, 802, 832);

        s.store_offset_scaled(833, 825, 0.065345483024, 1.0);

        s.store_add_ad(834, A::offset(A::scale(s.ad_value(825), 8.5797362674), 39.478417604), A::mul(s.ad_value(802), s.ad_value(832)));

        s.store_scale_ad(835, A::add(A::scale(s.ad_value(825), 2.0), A::mul(s.ad_value(802), s.ad_value(832))), 39.478417604);

        s.store_sqrt_ad(836, A::sub(A::square(s.ad_value(834)), A::mul(A::scale(s.ad_value(833), 4.0), s.ad_value(835))));

        s.store_div_ad(804, A::sub(s.ad_value(836), s.ad_value(834)), A::scale(s.ad_value(833), 2.0));

        s.store_sub_ad_lhs(837, A::square(s.ad_value(802)), 804);

        s.v[1154] = if (s.v[837] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1154] != 0.0) {
            s.store_mul_ad_rhs(828, 837, A::add(A::sub(A::add(A::ln(A::div(s.ad_value(837), s.ad_value(884))), s.ad_value(979)), s.ad_value(909)), s.ad_value(980)));
        }

        if (s.v[1154] != 0.0) {
            s.store_add_ad_lhs(829, A::mul(A::scale(s.ad_value(911), 2.0), s.ad_value(802)), 837);
        }

        if (s.v[1154] != 0.0) {
            s.store_sub_ad_lhs(838, A::sub(s.ad_value(909), s.ad_value(980)), 796);
        }

        s.v[1155] = if ((((s.v[828] < 0.0) && (s.v[829] > 0.0)) && (((s.v[838] + 2.3025850929941) + ((s.v[911]) as f64).ln()) > 0.0)) || (s.v[838] > 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1154] != 0.0) && (s.v[1155] != 0.0)) {
            s.store_sub_ad_rhs(980, 980, A::div(s.ad_value(828), s.ad_value(829)));
        }

        s.store_mul(802, 911, 980);

        s.store_mul(832, 912, 981);

        s.store_add(825, 802, 832);

        s.store_offset_scaled(833, 825, 0.065345483024, 1.0);

        s.store_add_ad(834, A::offset(A::scale(s.ad_value(825), 8.5797362674), 39.478417604), A::mul(s.ad_value(802), s.ad_value(832)));

        s.store_scale_ad(835, A::add(A::scale(s.ad_value(825), 2.0), A::mul(s.ad_value(802), s.ad_value(832))), 39.478417604);

        s.store_sqrt_ad(836, A::sub(A::square(s.ad_value(834)), A::mul(A::scale(s.ad_value(833), 4.0), s.ad_value(835))));

        s.store_div_ad(804, A::sub(s.ad_value(836), s.ad_value(834)), A::scale(s.ad_value(833), 2.0));

        s.v[1156] = if (s.v[804] < (-0.005)) { 1.0 } else { 0.0 };

        if (s.v[1156] != 0.0) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(804)));
        }

        if (s.v[1156] != 0.0) {
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
        }

        if (s.v[1156] != 0.0) {
            s.store_div_ad_lhs(809, A::scale(A::add(s.ad_value(804), A::mul(s.ad_value(808), A::sub_from_scalar(2.0, s.ad_value(808)))), 0.25), 804);
        }

        s.v[1157] = if (s.v[804] > 0.005) { 1.0 } else { 0.0 };

        if ((!(s.v[1156] != 0.0)) && (s.v[1157] != 0.0)) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(804)));
        }

        if ((!(s.v[1156] != 0.0)) && (s.v[1157] != 0.0)) {
            s.store_exp_ad(810, A::neg(s.ad_value(807)));
        }

        if ((!(s.v[1156] != 0.0)) && (s.v[1157] != 0.0)) {
            s.store_div_ad(808, A::mul(s.ad_value(807), A::offset(s.ad_value(810), 1.0)), A::sub_from_scalar(1.0, s.ad_value(810)));
        }

        if ((!(s.v[1156] != 0.0)) && (s.v[1157] != 0.0)) {
            s.store_div_ad_lhs(809, A::scale(A::add(s.ad_value(804), A::mul(s.ad_value(808), A::sub_from_scalar(2.0, s.ad_value(808)))), 0.25), 804);
        }

        if ((!(s.v[1156] != 0.0)) && (!(s.v[1157] != 0.0))) {
            s.store_offset_ad(808, A::mul(A::scale(s.ad_value(804), 0.1666666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0166666666667), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0238095238095))))), 2.0);
        }

        if ((!(s.v[1156] != 0.0)) && (!(s.v[1157] != 0.0))) {
            s.store_scale_ad(809, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0333333333333)))))), 0.1666666666667);
        }

        s.store_sub_ad_rhs(804, 804, A::div(A::add(A::add(A::mul(s.ad_value(825), s.ad_value(808)), A::mul(s.ad_value(802), s.ad_value(832))), s.ad_value(804)), A::offset(A::mul(s.ad_value(825), s.ad_value(809)), 1.0)));

        s.store_sub_ad_lhs(837, A::square(s.ad_value(802)), 804);

        s.v[1158] = if (s.v[837] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1158] != 0.0) {
            s.store_mul_ad_rhs(828, 837, A::add(A::sub(A::add(A::ln(A::div(s.ad_value(837), s.ad_value(884))), s.ad_value(979)), s.ad_value(909)), s.ad_value(980)));
        }

        if (s.v[1158] != 0.0) {
            s.store_add_ad_lhs(829, A::mul(A::scale(s.ad_value(911), 2.0), s.ad_value(802)), 837);
        }

        if (s.v[1158] != 0.0) {
            s.store_sub_ad_lhs(838, A::sub(s.ad_value(909), s.ad_value(980)), 796);
        }

        s.v[1159] = if ((((s.v[828] < 0.0) && (s.v[829] > 0.0)) && (((s.v[838] + 2.3025850929941) + ((s.v[911]) as f64).ln()) > 0.0)) || (s.v[838] > 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1158] != 0.0) && (s.v[1159] != 0.0)) {
            s.store_sub_ad_rhs(980, 980, A::div(s.ad_value(828), s.ad_value(829)));
        }

        s.store_mul(802, 911, 980);

        s.v[1160] = if (((s.v[909] - s.v[980]) - s.v[979]) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1160] != 0.0) {
            s.store_exp_ad(793, A::sub(A::sub(s.ad_value(909), s.ad_value(980)), s.ad_value(979)));
        }

        if (!(s.v[1160] != 0.0)) {
            let assign18380_ad_e18201: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(909), s.ad_value(980)), s.ad_value(979)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(s.ad_value(909), s.ad_value(980)), s.ad_value(979)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(909), s.ad_value(980)), s.ad_value(979)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(793, assign18380_ad_e18201, 5.54062e34);
        }

        s.store_mul(803, 884, 793);

        s.store_sub_ad_lhs(804, A::square(s.ad_value(802)), 803);

        s.store_add_ad_lhs(805, A::mul(A::scale(s.ad_value(911), 2.0), s.ad_value(802)), 803);

        s.store_sub_ad_lhs(806, A::mul(A::scale(s.ad_value(911), 2.0), s.ad_value(911)), 803);

        s.v[1161] = if (s.v[804] < (-0.005)) { 1.0 } else { 0.0 };

        if (s.v[1161] != 0.0) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(804)));
        }

        if (s.v[1161] != 0.0) {
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
        }

        if (s.v[1161] != 0.0) {
            s.store_div_ad_lhs(793, A::scale(s.ad_value(805), 0.25), 804);
        }

        if (s.v[1161] != 0.0) {
            s.store_mul_ad_lhs(809, A::add(s.ad_value(804), A::mul(s.ad_value(808), A::sub_from_scalar(2.0, s.ad_value(808)))), 793);
        }

        if (s.v[1161] != 0.0) {
            s.store_add_ad(811, A::mul(A::sub(s.ad_value(805), A::mul(A::scale(s.ad_value(809), 2.0), A::offset(s.ad_value(808), 1.0))), s.ad_value(793)), A::div(A::mul(s.ad_value(809), s.ad_value(806)), s.ad_value(805)));
        }

        if (s.v[1161] != 0.0) {
            s.store_sub_from_scalar_ad(794, 1.0, A::scale(s.ad_value(808), 0.5));
        }

        if (s.v[1161] != 0.0) {
            s.store_mul_ad_lhs(814, A::div(s.ad_value(805), s.ad_value(804)), 794);
        }

        if (s.v[1161] != 0.0) {
            s.store_div_ad_lhs(815, A::sub(A::mul(s.ad_value(806), s.ad_value(794)), A::mul(s.ad_value(805), A::add(s.ad_value(814), A::scale(s.ad_value(809), 0.5)))), 804);
        }

        s.v[1162] = if (s.v[804] > 0.005) { 1.0 } else { 0.0 };

        if ((!(s.v[1161] != 0.0)) && (s.v[1162] != 0.0)) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(804)));
        }

        if ((!(s.v[1161] != 0.0)) && (s.v[1162] != 0.0)) {
            s.store_exp_ad(810, A::neg(s.ad_value(807)));
        }

        if ((!(s.v[1161] != 0.0)) && (s.v[1162] != 0.0)) {
            s.store_div_ad(808, A::mul(s.ad_value(807), A::offset(s.ad_value(810), 1.0)), A::sub_from_scalar(1.0, s.ad_value(810)));
        }

        if ((!(s.v[1161] != 0.0)) && (s.v[1162] != 0.0)) {
            s.store_div_ad_lhs(793, A::scale(s.ad_value(805), 0.25), 804);
        }

        if ((!(s.v[1161] != 0.0)) && (s.v[1162] != 0.0)) {
            s.store_mul_ad_lhs(809, A::add(s.ad_value(804), A::mul(s.ad_value(808), A::sub_from_scalar(2.0, s.ad_value(808)))), 793);
        }

        if ((!(s.v[1161] != 0.0)) && (s.v[1162] != 0.0)) {
            s.store_add_ad(811, A::mul(A::sub(s.ad_value(805), A::mul(A::scale(s.ad_value(809), 2.0), A::offset(s.ad_value(808), 1.0))), s.ad_value(793)), A::div(A::mul(s.ad_value(809), s.ad_value(806)), s.ad_value(805)));
        }

        if ((!(s.v[1161] != 0.0)) && (s.v[1162] != 0.0)) {
            s.store_sub_from_scalar_ad(794, 1.0, A::scale(s.ad_value(808), 0.5));
        }

        if ((!(s.v[1161] != 0.0)) && (s.v[1162] != 0.0)) {
            s.store_mul_ad_lhs(814, A::div(s.ad_value(805), s.ad_value(804)), 794);
        }

        if ((!(s.v[1161] != 0.0)) && (s.v[1162] != 0.0)) {
            s.store_div_ad_lhs(815, A::sub(A::mul(s.ad_value(806), s.ad_value(794)), A::mul(s.ad_value(805), A::add(s.ad_value(814), A::scale(s.ad_value(809), 0.5)))), 804);
        }

        if ((!(s.v[1161] != 0.0)) && (!(s.v[1162] != 0.0))) {
            s.store_scale_ad(795, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.025)))))), 0.1666666666667);
        }

        if ((!(s.v[1161] != 0.0)) && (!(s.v[1162] != 0.0))) {
            s.store_offset_ad(808, A::mul(s.ad_value(804), s.ad_value(795)), 2.0);
        }

        if ((!(s.v[1161] != 0.0)) && (!(s.v[1162] != 0.0))) {
            s.store_scale_ad(793, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0333333333333)))))), 0.1666666666667);
        }

        if ((!(s.v[1161] != 0.0)) && (!(s.v[1162] != 0.0))) {
            s.store_mul(809, 805, 793);
        }

        if ((!(s.v[1161] != 0.0)) && (!(s.v[1162] != 0.0))) {
            s.store_scale_ad(794, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0420875420875421)))))), 0.0055555555556);
        }

        if ((!(s.v[1161] != 0.0)) && (!(s.v[1162] != 0.0))) {
            s.store_sub_ad(811, A::mul(s.ad_value(806), s.ad_value(793)), A::mul(A::square(s.ad_value(805)), s.ad_value(794)));
        }

        if ((!(s.v[1161] != 0.0)) && (!(s.v[1162] != 0.0))) {
            s.store_mul_ad_lhs(814, A::scale(s.ad_value(805), (-0.5)), 795);
        }

        if ((!(s.v[1161] != 0.0)) && (!(s.v[1162] != 0.0))) {
            s.store_add_ad(815, A::mul(A::scale(s.ad_value(806), (-0.5)), s.ad_value(795)), A::mul(A::mul(A::scale(s.ad_value(805), (0.25 * 0.0055555555556)), s.ad_value(805)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(804), 0.075))))));
        }

        s.v[1163] = if (s.v[804] > 0.005) { 1.0 } else { 0.0 };

        if (s.v[1163] != 0.0) {
            s.store_div_ad(794, A::scale(s.ad_value(804), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(810), A::sub_from_scalar(2.0, s.ad_value(810)))));
        }

        if (s.v[1163] != 0.0) {
            s.store_mul(812, 794, 810);
        }

        if (s.v[1163] != 0.0) {
            s.store_sub_ad_lhs(813, A::ln(s.ad_value(794)), 807);
        }

        s.v[1164] = if (s.v[804] < (-0.005)) { 1.0 } else { 0.0 };

        if ((!(s.v[1163] != 0.0)) && (s.v[1164] != 0.0)) {
            s.store_sin_ad(794, A::scale(s.ad_value(807), 0.5));
        }

        if ((!(s.v[1163] != 0.0)) && (s.v[1164] != 0.0)) {
            s.store_div_ad(812, A::neg(s.ad_value(804)), A::square(s.ad_value(794)));
        }

        if ((!(s.v[1163] != 0.0)) && (s.v[1164] != 0.0)) {
            s.store_ln(813, 812);
        }

        if ((!(s.v[1163] != 0.0)) && (!(s.v[1164] != 0.0))) {
            s.store_sub_from_scalar_ad(812, 4.0, A::mul(A::scale(s.ad_value(804), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0396825396825397))))));
        }

        if ((!(s.v[1163] != 0.0)) && (!(s.v[1164] != 0.0))) {
            s.store_ln(813, 812);
        }

        s.v[1165] = if (((1.01 * s.v[802]) + s.v[808]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1165] != 0.0) {
            s.store_add(816, 802, 808);
        }

        if (s.v[1165] != 0.0) {
            s.store_add(817, 911, 809);
        }

        if (s.v[1165] != 0.0) {
            s.copy_ad(818, 811);
        }

        if (!(s.v[1165] != 0.0)) {
            s.store_div_from_scalar_ad(794, 1.0, A::sub(s.ad_value(802), s.ad_value(808)));
        }

        if (!(s.v[1165] != 0.0)) {
            s.store_sub(795, 809, 911);
        }

        if (!(s.v[1165] != 0.0)) {
            s.store_mul_ad_lhs(816, A::sub(s.ad_value(803), s.ad_value(812)), 794);
        }

        if (!(s.v[1165] != 0.0)) {
            s.store_mul_ad_lhs(817, A::sub(A::sub(A::mul(s.ad_value(795), s.ad_value(816)), s.ad_value(803)), A::mul(s.ad_value(814), s.ad_value(812))), 794);
        }

        if (!(s.v[1165] != 0.0)) {
            s.store_mul_ad_lhs(818, A::sub(A::add(A::add(A::mul(s.ad_value(811), s.ad_value(816)), A::mul(A::scale(s.ad_value(795), 2.0), s.ad_value(817))), s.ad_value(803)), A::mul(A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812))), 794);
        }

        s.v[1166] = if (s.v[816] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1166] != 0.0) {
            s.store_ln(819, 816);
        }

        if (s.v[1166] != 0.0) {
            s.store_div_from_scalar(793, 1.0, 816);
        }

        if (s.v[1166] != 0.0) {
            s.store_mul(820, 817, 793);
        }

        if (s.v[1166] != 0.0) {
            s.store_sub_ad(821, A::mul(s.ad_value(818), s.ad_value(793)), A::square(s.ad_value(820)));
        }

        if (!(s.v[1166] != 0.0)) {
            s.store_add_ad(819, A::offset(s.ad_value(802), 0.6931471805599), A::ln(A::neg(s.ad_value(802))));
        }

        if (!(s.v[1166] != 0.0)) {
            s.store_div_from_scalar(793, 1.0, 980);
        }

        if (!(s.v[1166] != 0.0)) {
            s.store_add(820, 911, 793);
        }

        if (!(s.v[1166] != 0.0)) {
            s.store_mul_ad_lhs(821, A::neg(s.ad_value(793)), 793);
        }

        s.store_sub_ad_lhs(822, A::add(A::add(A::sub(s.ad_value(910), s.ad_value(909)), s.ad_value(980)), A::scale(s.ad_value(819), 2.0)), 813);

        s.store_sub_ad_lhs(823, A::offset(A::scale(s.ad_value(820), 2.0), 1.0), 814);

        s.store_sub_ad_lhs(824, A::scale(s.ad_value(821), 2.0), 815);

        s.store_add_ad_rhs(825, 802, A::mul(s.ad_value(912), s.ad_value(822)));

        s.store_add_ad_rhs(826, 911, A::mul(s.ad_value(912), s.ad_value(823)));

        s.store_mul(827, 912, 824);

        s.store_sub_ad_lhs(828, A::mul(s.ad_value(825), s.ad_value(816)), 803);

        s.store_add_ad_lhs(829, A::add(A::mul(s.ad_value(826), s.ad_value(816)), A::mul(s.ad_value(825), s.ad_value(817))), 803);

        s.store_sub_ad_lhs(830, A::add(A::add(A::mul(s.ad_value(827), s.ad_value(816)), A::mul(A::scale(s.ad_value(826), 2.0), s.ad_value(817))), A::mul(s.ad_value(825), s.ad_value(818))), 803);

        s.store_sub_ad(839, A::square(s.ad_value(829)), A::mul(A::scale(s.ad_value(828), 0.5), s.ad_value(830)));

        s.store_div_ad(831, A::mul(A::mul(A::neg(s.ad_value(828)), s.ad_value(829)), s.ad_value(839)), A::offset(A::square(s.ad_value(839)), 1e-200));

        s.store_add(980, 980, 831);

        s.store_mul(802, 911, 980);

        s.v[1167] = if (((s.v[909] - s.v[980]) - s.v[979]) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1167] != 0.0) {
            s.store_exp_ad(793, A::sub(A::sub(s.ad_value(909), s.ad_value(980)), s.ad_value(979)));
        }

        if (!(s.v[1167] != 0.0)) {
            let assign19130_ad_e18997: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(909), s.ad_value(980)), s.ad_value(979)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(s.ad_value(909), s.ad_value(980)), s.ad_value(979)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(909), s.ad_value(980)), s.ad_value(979)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(793, assign19130_ad_e18997, 5.54062e34);
        }

        s.store_mul(803, 884, 793);

        s.store_sub_ad_lhs(804, A::square(s.ad_value(802)), 803);

        s.store_add_ad_lhs(805, A::mul(A::scale(s.ad_value(911), 2.0), s.ad_value(802)), 803);

        s.store_sub_ad_lhs(806, A::mul(A::scale(s.ad_value(911), 2.0), s.ad_value(911)), 803);

        s.v[1168] = if (s.v[804] < (-0.005)) { 1.0 } else { 0.0 };

        if (s.v[1168] != 0.0) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(804)));
        }

        if (s.v[1168] != 0.0) {
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
        }

        if (s.v[1168] != 0.0) {
            s.store_div_ad_lhs(793, A::scale(s.ad_value(805), 0.25), 804);
        }

        if (s.v[1168] != 0.0) {
            s.store_mul_ad_lhs(809, A::add(s.ad_value(804), A::mul(s.ad_value(808), A::sub_from_scalar(2.0, s.ad_value(808)))), 793);
        }

        if (s.v[1168] != 0.0) {
            s.store_add_ad(811, A::mul(A::sub(s.ad_value(805), A::mul(A::scale(s.ad_value(809), 2.0), A::offset(s.ad_value(808), 1.0))), s.ad_value(793)), A::div(A::mul(s.ad_value(809), s.ad_value(806)), s.ad_value(805)));
        }

        if (s.v[1168] != 0.0) {
            s.store_sub_from_scalar_ad(794, 1.0, A::scale(s.ad_value(808), 0.5));
        }

        if (s.v[1168] != 0.0) {
            s.store_mul_ad_lhs(814, A::div(s.ad_value(805), s.ad_value(804)), 794);
        }

        if (s.v[1168] != 0.0) {
            s.store_div_ad_lhs(815, A::sub(A::mul(s.ad_value(806), s.ad_value(794)), A::mul(s.ad_value(805), A::add(s.ad_value(814), A::scale(s.ad_value(809), 0.5)))), 804);
        }

        s.v[1169] = if (s.v[804] > 0.005) { 1.0 } else { 0.0 };

        if ((!(s.v[1168] != 0.0)) && (s.v[1169] != 0.0)) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(804)));
        }

        if ((!(s.v[1168] != 0.0)) && (s.v[1169] != 0.0)) {
            s.store_exp_ad(810, A::neg(s.ad_value(807)));
        }

        if ((!(s.v[1168] != 0.0)) && (s.v[1169] != 0.0)) {
            s.store_div_ad(808, A::mul(s.ad_value(807), A::offset(s.ad_value(810), 1.0)), A::sub_from_scalar(1.0, s.ad_value(810)));
        }

        if ((!(s.v[1168] != 0.0)) && (s.v[1169] != 0.0)) {
            s.store_div_ad_lhs(793, A::scale(s.ad_value(805), 0.25), 804);
        }

        if ((!(s.v[1168] != 0.0)) && (s.v[1169] != 0.0)) {
            s.store_mul_ad_lhs(809, A::add(s.ad_value(804), A::mul(s.ad_value(808), A::sub_from_scalar(2.0, s.ad_value(808)))), 793);
        }

        if ((!(s.v[1168] != 0.0)) && (s.v[1169] != 0.0)) {
            s.store_add_ad(811, A::mul(A::sub(s.ad_value(805), A::mul(A::scale(s.ad_value(809), 2.0), A::offset(s.ad_value(808), 1.0))), s.ad_value(793)), A::div(A::mul(s.ad_value(809), s.ad_value(806)), s.ad_value(805)));
        }

        if ((!(s.v[1168] != 0.0)) && (s.v[1169] != 0.0)) {
            s.store_sub_from_scalar_ad(794, 1.0, A::scale(s.ad_value(808), 0.5));
        }

        if ((!(s.v[1168] != 0.0)) && (s.v[1169] != 0.0)) {
            s.store_mul_ad_lhs(814, A::div(s.ad_value(805), s.ad_value(804)), 794);
        }

        if ((!(s.v[1168] != 0.0)) && (s.v[1169] != 0.0)) {
            s.store_div_ad_lhs(815, A::sub(A::mul(s.ad_value(806), s.ad_value(794)), A::mul(s.ad_value(805), A::add(s.ad_value(814), A::scale(s.ad_value(809), 0.5)))), 804);
        }

        if ((!(s.v[1168] != 0.0)) && (!(s.v[1169] != 0.0))) {
            s.store_scale_ad(795, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.025)))))), 0.1666666666667);
        }

    }

    pub(super) fn stamp_transient_block_13(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((!(s.v[1168] != 0.0)) && (!(s.v[1169] != 0.0))) {
            s.store_offset_ad(808, A::mul(s.ad_value(804), s.ad_value(795)), 2.0);
        }

        if ((!(s.v[1168] != 0.0)) && (!(s.v[1169] != 0.0))) {
            s.store_scale_ad(793, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0333333333333)))))), 0.1666666666667);
        }

        if ((!(s.v[1168] != 0.0)) && (!(s.v[1169] != 0.0))) {
            s.store_mul(809, 805, 793);
        }

        if ((!(s.v[1168] != 0.0)) && (!(s.v[1169] != 0.0))) {
            s.store_scale_ad(794, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0420875420875421)))))), 0.0055555555556);
        }

        if ((!(s.v[1168] != 0.0)) && (!(s.v[1169] != 0.0))) {
            s.store_sub_ad(811, A::mul(s.ad_value(806), s.ad_value(793)), A::mul(A::square(s.ad_value(805)), s.ad_value(794)));
        }

        if ((!(s.v[1168] != 0.0)) && (!(s.v[1169] != 0.0))) {
            s.store_mul_ad_lhs(814, A::scale(s.ad_value(805), (-0.5)), 795);
        }

        if ((!(s.v[1168] != 0.0)) && (!(s.v[1169] != 0.0))) {
            s.store_add_ad(815, A::mul(A::scale(s.ad_value(806), (-0.5)), s.ad_value(795)), A::mul(A::mul(A::scale(s.ad_value(805), (0.25 * 0.0055555555556)), s.ad_value(805)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(804), 0.075))))));
        }

        s.v[1170] = if (s.v[804] > 0.005) { 1.0 } else { 0.0 };

        if (s.v[1170] != 0.0) {
            s.store_div_ad(794, A::scale(s.ad_value(804), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(810), A::sub_from_scalar(2.0, s.ad_value(810)))));
        }

        if (s.v[1170] != 0.0) {
            s.store_mul(812, 794, 810);
        }

        if (s.v[1170] != 0.0) {
            s.store_sub_ad_lhs(813, A::ln(s.ad_value(794)), 807);
        }

        s.v[1171] = if (s.v[804] < (-0.005)) { 1.0 } else { 0.0 };

        if ((!(s.v[1170] != 0.0)) && (s.v[1171] != 0.0)) {
            s.store_sin_ad(794, A::scale(s.ad_value(807), 0.5));
        }

        if ((!(s.v[1170] != 0.0)) && (s.v[1171] != 0.0)) {
            s.store_div_ad(812, A::neg(s.ad_value(804)), A::square(s.ad_value(794)));
        }

        if ((!(s.v[1170] != 0.0)) && (s.v[1171] != 0.0)) {
            s.store_ln(813, 812);
        }

        if ((!(s.v[1170] != 0.0)) && (!(s.v[1171] != 0.0))) {
            s.store_sub_from_scalar_ad(812, 4.0, A::mul(A::scale(s.ad_value(804), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0396825396825397))))));
        }

        if ((!(s.v[1170] != 0.0)) && (!(s.v[1171] != 0.0))) {
            s.store_ln(813, 812);
        }

        s.v[1172] = if (((1.01 * s.v[802]) + s.v[808]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1172] != 0.0) {
            s.store_add(816, 802, 808);
        }

        if (s.v[1172] != 0.0) {
            s.store_add(817, 911, 809);
        }

        if (s.v[1172] != 0.0) {
            s.copy_ad(818, 811);
        }

        if (!(s.v[1172] != 0.0)) {
            s.store_div_from_scalar_ad(794, 1.0, A::sub(s.ad_value(802), s.ad_value(808)));
        }

        if (!(s.v[1172] != 0.0)) {
            s.store_sub(795, 809, 911);
        }

        if (!(s.v[1172] != 0.0)) {
            s.store_mul_ad_lhs(816, A::sub(s.ad_value(803), s.ad_value(812)), 794);
        }

        if (!(s.v[1172] != 0.0)) {
            s.store_mul_ad_lhs(817, A::sub(A::sub(A::mul(s.ad_value(795), s.ad_value(816)), s.ad_value(803)), A::mul(s.ad_value(814), s.ad_value(812))), 794);
        }

        if (!(s.v[1172] != 0.0)) {
            s.store_mul_ad_lhs(818, A::sub(A::add(A::add(A::mul(s.ad_value(811), s.ad_value(816)), A::mul(A::scale(s.ad_value(795), 2.0), s.ad_value(817))), s.ad_value(803)), A::mul(A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812))), 794);
        }

        s.v[1173] = if (s.v[816] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1173] != 0.0) {
            s.store_ln(819, 816);
        }

        if (s.v[1173] != 0.0) {
            s.store_div_from_scalar(793, 1.0, 816);
        }

        if (s.v[1173] != 0.0) {
            s.store_mul(820, 817, 793);
        }

        if (s.v[1173] != 0.0) {
            s.store_sub_ad(821, A::mul(s.ad_value(818), s.ad_value(793)), A::square(s.ad_value(820)));
        }

        if (!(s.v[1173] != 0.0)) {
            s.store_add_ad(819, A::offset(s.ad_value(802), 0.6931471805599), A::ln(A::neg(s.ad_value(802))));
        }

        if (!(s.v[1173] != 0.0)) {
            s.store_div_from_scalar(793, 1.0, 980);
        }

        if (!(s.v[1173] != 0.0)) {
            s.store_add(820, 911, 793);
        }

        if (!(s.v[1173] != 0.0)) {
            s.store_mul_ad_lhs(821, A::neg(s.ad_value(793)), 793);
        }

        s.store_sub_ad_lhs(822, A::add(A::add(A::sub(s.ad_value(910), s.ad_value(909)), s.ad_value(980)), A::scale(s.ad_value(819), 2.0)), 813);

        s.store_sub_ad_lhs(823, A::offset(A::scale(s.ad_value(820), 2.0), 1.0), 814);

        s.store_sub_ad_lhs(824, A::scale(s.ad_value(821), 2.0), 815);

        s.store_add_ad_rhs(825, 802, A::mul(s.ad_value(912), s.ad_value(822)));

        s.store_add_ad_rhs(826, 911, A::mul(s.ad_value(912), s.ad_value(823)));

        s.store_mul(827, 912, 824);

        s.store_sub_ad_lhs(828, A::mul(s.ad_value(825), s.ad_value(816)), 803);

        s.store_add_ad_lhs(829, A::add(A::mul(s.ad_value(826), s.ad_value(816)), A::mul(s.ad_value(825), s.ad_value(817))), 803);

        s.store_sub_ad_lhs(830, A::add(A::add(A::mul(s.ad_value(827), s.ad_value(816)), A::mul(A::scale(s.ad_value(826), 2.0), s.ad_value(817))), A::mul(s.ad_value(825), s.ad_value(818))), 803);

        s.store_sub_ad(839, A::square(s.ad_value(829)), A::mul(A::scale(s.ad_value(828), 0.5), s.ad_value(830)));

        s.store_div_ad(831, A::mul(A::mul(A::neg(s.ad_value(828)), s.ad_value(829)), s.ad_value(839)), A::offset(A::square(s.ad_value(839)), 1e-200));

        s.store_add(980, 980, 831);

        s.v[1174] = if (p.p10 == 1.0) { 1.0 } else { 0.0 };

        s.v[1175] = if (((s.v[831]) as f64).abs() > 0.01) { 1.0 } else { 0.0 };

        if ((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_mul(802, 911, 980);
        }

        s.v[1176] = if (((s.v[909] - s.v[980]) - s.v[979]) < 80.0) { 1.0 } else { 0.0 };

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (s.v[1176] != 0.0)) {
            s.store_exp_ad(793, A::sub(A::sub(s.ad_value(909), s.ad_value(980)), s.ad_value(979)));
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1176] != 0.0))) {
            let assign19900_ad_e19813: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(909), s.ad_value(980)), s.ad_value(979)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(s.ad_value(909), s.ad_value(980)), s.ad_value(979)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(909), s.ad_value(980)), s.ad_value(979)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(793, assign19900_ad_e19813, 5.54062e34);
        }

        if ((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_mul(803, 884, 793);
        }

        if ((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_sub_ad_lhs(804, A::square(s.ad_value(802)), 803);
        }

        if ((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_add_ad_lhs(805, A::mul(A::scale(s.ad_value(911), 2.0), s.ad_value(802)), 803);
        }

        if ((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_sub_ad_lhs(806, A::mul(A::scale(s.ad_value(911), 2.0), s.ad_value(911)), 803);
        }

        s.v[1177] = if (s.v[804] < (-0.005)) { 1.0 } else { 0.0 };

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (s.v[1177] != 0.0)) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(804)));
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (s.v[1177] != 0.0)) {
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (s.v[1177] != 0.0)) {
            s.store_div_ad_lhs(793, A::scale(s.ad_value(805), 0.25), 804);
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (s.v[1177] != 0.0)) {
            s.store_mul_ad_lhs(809, A::add(s.ad_value(804), A::mul(s.ad_value(808), A::sub_from_scalar(2.0, s.ad_value(808)))), 793);
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (s.v[1177] != 0.0)) {
            s.store_add_ad(811, A::mul(A::sub(s.ad_value(805), A::mul(A::scale(s.ad_value(809), 2.0), A::offset(s.ad_value(808), 1.0))), s.ad_value(793)), A::div(A::mul(s.ad_value(809), s.ad_value(806)), s.ad_value(805)));
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (s.v[1177] != 0.0)) {
            s.store_sub_from_scalar_ad(794, 1.0, A::scale(s.ad_value(808), 0.5));
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (s.v[1177] != 0.0)) {
            s.store_mul_ad_lhs(814, A::div(s.ad_value(805), s.ad_value(804)), 794);
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (s.v[1177] != 0.0)) {
            s.store_div_ad_lhs(815, A::sub(A::mul(s.ad_value(806), s.ad_value(794)), A::mul(s.ad_value(805), A::add(s.ad_value(814), A::scale(s.ad_value(809), 0.5)))), 804);
        }

        s.v[1178] = if (s.v[804] > 0.005) { 1.0 } else { 0.0 };

        if ((((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1177] != 0.0))) && (s.v[1178] != 0.0)) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(804)));
        }

        if ((((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1177] != 0.0))) && (s.v[1178] != 0.0)) {
            s.store_exp_ad(810, A::neg(s.ad_value(807)));
        }

        if ((((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1177] != 0.0))) && (s.v[1178] != 0.0)) {
            s.store_div_ad(808, A::mul(s.ad_value(807), A::offset(s.ad_value(810), 1.0)), A::sub_from_scalar(1.0, s.ad_value(810)));
        }

        if ((((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1177] != 0.0))) && (s.v[1178] != 0.0)) {
            s.store_div_ad_lhs(793, A::scale(s.ad_value(805), 0.25), 804);
        }

        if ((((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1177] != 0.0))) && (s.v[1178] != 0.0)) {
            s.store_mul_ad_lhs(809, A::add(s.ad_value(804), A::mul(s.ad_value(808), A::sub_from_scalar(2.0, s.ad_value(808)))), 793);
        }

        if ((((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1177] != 0.0))) && (s.v[1178] != 0.0)) {
            s.store_add_ad(811, A::mul(A::sub(s.ad_value(805), A::mul(A::scale(s.ad_value(809), 2.0), A::offset(s.ad_value(808), 1.0))), s.ad_value(793)), A::div(A::mul(s.ad_value(809), s.ad_value(806)), s.ad_value(805)));
        }

        if ((((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1177] != 0.0))) && (s.v[1178] != 0.0)) {
            s.store_sub_from_scalar_ad(794, 1.0, A::scale(s.ad_value(808), 0.5));
        }

        if ((((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1177] != 0.0))) && (s.v[1178] != 0.0)) {
            s.store_mul_ad_lhs(814, A::div(s.ad_value(805), s.ad_value(804)), 794);
        }

        if ((((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1177] != 0.0))) && (s.v[1178] != 0.0)) {
            s.store_div_ad_lhs(815, A::sub(A::mul(s.ad_value(806), s.ad_value(794)), A::mul(s.ad_value(805), A::add(s.ad_value(814), A::scale(s.ad_value(809), 0.5)))), 804);
        }

        if ((((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1177] != 0.0))) && (!(s.v[1178] != 0.0))) {
            s.store_scale_ad(795, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0166666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0238095238095), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.025)))))), 0.1666666666667);
        }

        if ((((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1177] != 0.0))) && (!(s.v[1178] != 0.0))) {
            s.store_offset_ad(808, A::mul(s.ad_value(804), s.ad_value(795)), 2.0);
        }

        if ((((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1177] != 0.0))) && (!(s.v[1178] != 0.0))) {
            s.store_scale_ad(793, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0333333333333)))))), 0.1666666666667);
        }

        if ((((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1177] != 0.0))) && (!(s.v[1178] != 0.0))) {
            s.store_mul(809, 805, 793);
        }

        if ((((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1177] != 0.0))) && (!(s.v[1178] != 0.0))) {
            s.store_scale_ad(794, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0714285714286), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0420875420875421)))))), 0.0055555555556);
        }

        if ((((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1177] != 0.0))) && (!(s.v[1178] != 0.0))) {
            s.store_sub_ad(811, A::mul(s.ad_value(806), s.ad_value(793)), A::mul(A::square(s.ad_value(805)), s.ad_value(794)));
        }

        if ((((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1177] != 0.0))) && (!(s.v[1178] != 0.0))) {
            s.store_mul_ad_lhs(814, A::scale(s.ad_value(805), (-0.5)), 795);
        }

        if ((((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1177] != 0.0))) && (!(s.v[1178] != 0.0))) {
            s.store_add_ad(815, A::mul(A::scale(s.ad_value(806), (-0.5)), s.ad_value(795)), A::mul(A::mul(A::scale(s.ad_value(805), (0.25 * 0.0055555555556)), s.ad_value(805)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.0238095238095), A::sub_from_scalar(2.0, A::scale(s.ad_value(804), 0.075))))));
        }

        s.v[1179] = if (s.v[804] > 0.005) { 1.0 } else { 0.0 };

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (s.v[1179] != 0.0)) {
            s.store_div_ad(794, A::scale(s.ad_value(804), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(810), A::sub_from_scalar(2.0, s.ad_value(810)))));
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (s.v[1179] != 0.0)) {
            s.store_mul(812, 794, 810);
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (s.v[1179] != 0.0)) {
            s.store_sub_ad_lhs(813, A::ln(s.ad_value(794)), 807);
        }

        s.v[1180] = if (s.v[804] < (-0.005)) { 1.0 } else { 0.0 };

        if ((((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1179] != 0.0))) && (s.v[1180] != 0.0)) {
            s.store_sin_ad(794, A::scale(s.ad_value(807), 0.5));
        }

        if ((((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1179] != 0.0))) && (s.v[1180] != 0.0)) {
            s.store_div_ad(812, A::neg(s.ad_value(804)), A::square(s.ad_value(794)));
        }

        if ((((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1179] != 0.0))) && (s.v[1180] != 0.0)) {
            s.store_ln(813, 812);
        }

        if ((((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1179] != 0.0))) && (!(s.v[1180] != 0.0))) {
            s.store_sub_from_scalar_ad(812, 4.0, A::mul(A::scale(s.ad_value(804), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(804), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(804), 0.0396825396825397))))));
        }

        if ((((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1179] != 0.0))) && (!(s.v[1180] != 0.0))) {
            s.store_ln(813, 812);
        }

        s.v[1181] = if (((1.01 * s.v[802]) + s.v[808]) > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (s.v[1181] != 0.0)) {
            s.store_add(816, 802, 808);
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (s.v[1181] != 0.0)) {
            s.store_add(817, 911, 809);
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (s.v[1181] != 0.0)) {
            s.copy_ad(818, 811);
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1181] != 0.0))) {
            s.store_div_from_scalar_ad(794, 1.0, A::sub(s.ad_value(802), s.ad_value(808)));
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1181] != 0.0))) {
            s.store_sub(795, 809, 911);
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1181] != 0.0))) {
            s.store_mul_ad_lhs(816, A::sub(s.ad_value(803), s.ad_value(812)), 794);
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1181] != 0.0))) {
            s.store_mul_ad_lhs(817, A::sub(A::sub(A::mul(s.ad_value(795), s.ad_value(816)), s.ad_value(803)), A::mul(s.ad_value(814), s.ad_value(812))), 794);
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1181] != 0.0))) {
            s.store_mul_ad_lhs(818, A::sub(A::add(A::add(A::mul(s.ad_value(811), s.ad_value(816)), A::mul(A::scale(s.ad_value(795), 2.0), s.ad_value(817))), s.ad_value(803)), A::mul(A::add(s.ad_value(815), A::square(s.ad_value(814))), s.ad_value(812))), 794);
        }

        s.v[1182] = if (s.v[816] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (s.v[1182] != 0.0)) {
            s.store_ln(819, 816);
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (s.v[1182] != 0.0)) {
            s.store_div_from_scalar(793, 1.0, 816);
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (s.v[1182] != 0.0)) {
            s.store_mul(820, 817, 793);
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (s.v[1182] != 0.0)) {
            s.store_sub_ad(821, A::mul(s.ad_value(818), s.ad_value(793)), A::square(s.ad_value(820)));
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1182] != 0.0))) {
            s.store_add_ad(819, A::offset(s.ad_value(802), 0.6931471805599), A::ln(A::neg(s.ad_value(802))));
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1182] != 0.0))) {
            s.store_div_from_scalar(793, 1.0, 980);
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1182] != 0.0))) {
            s.store_add(820, 911, 793);
        }

        if (((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) && (!(s.v[1182] != 0.0))) {
            s.store_mul_ad_lhs(821, A::neg(s.ad_value(793)), 793);
        }

        if ((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_sub_ad_lhs(822, A::add(A::add(A::sub(s.ad_value(910), s.ad_value(909)), s.ad_value(980)), A::scale(s.ad_value(819), 2.0)), 813);
        }

        if ((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_sub_ad_lhs(823, A::offset(A::scale(s.ad_value(820), 2.0), 1.0), 814);
        }

        if ((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_sub_ad_lhs(824, A::scale(s.ad_value(821), 2.0), 815);
        }

        if ((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_add_ad_rhs(825, 802, A::mul(s.ad_value(912), s.ad_value(822)));
        }

        if ((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_add_ad_rhs(826, 911, A::mul(s.ad_value(912), s.ad_value(823)));
        }

        if ((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_mul(827, 912, 824);
        }

        if ((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_sub_ad_lhs(828, A::mul(s.ad_value(825), s.ad_value(816)), 803);
        }

        if ((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_add_ad_lhs(829, A::add(A::mul(s.ad_value(826), s.ad_value(816)), A::mul(s.ad_value(825), s.ad_value(817))), 803);
        }

        if ((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_sub_ad_lhs(830, A::add(A::add(A::mul(s.ad_value(827), s.ad_value(816)), A::mul(A::scale(s.ad_value(826), 2.0), s.ad_value(817))), A::mul(s.ad_value(825), s.ad_value(818))), 803);
        }

        if ((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_sub_ad(839, A::square(s.ad_value(829)), A::mul(A::scale(s.ad_value(828), 0.5), s.ad_value(830)));
        }

        if ((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_div_ad(831, A::mul(A::mul(A::neg(s.ad_value(828)), s.ad_value(829)), s.ad_value(839)), A::offset(A::square(s.ad_value(839)), 1e-200));
        }

        if ((s.v[1174] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_add(980, 980, 831);
        }

        s.store_mul(983, 911, 980);

        s.v[1183] = if (((s.v[909] - s.v[980]) - s.v[979]) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1183] != 0.0) {
            s.store_exp_ad(793, A::sub(A::sub(s.ad_value(909), s.ad_value(980)), s.ad_value(979)));
        }

        if (!(s.v[1183] != 0.0)) {
            let assign20650_ad_e20885: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(909), s.ad_value(980)), s.ad_value(979)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(s.ad_value(909), s.ad_value(980)), s.ad_value(979)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(909), s.ad_value(980)), s.ad_value(979)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(793, assign20650_ad_e20885, 5.54062e34);
        }

        s.store_mul(986, 884, 793);

        s.store_sub_ad_lhs(985, A::square(s.ad_value(983)), 986);

        s.v[1184] = if (s.v[986] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1184] != 0.0) {
            s.store_scalar(982, 1e-80);
        }

        if (s.v[1184] != 0.0) {
            s.store_sub(984, 982, 983);
        }

        if (s.v[1184] != 0.0) {
            s.store_div(981, 984, 912);
        }

        s.v[1185] = if (s.v[985] < (-0.005)) { 1.0 } else { 0.0 };

        if ((!(s.v[1184] != 0.0)) && (s.v[1185] != 0.0)) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(985)));
        }

        if ((!(s.v[1184] != 0.0)) && (s.v[1185] != 0.0)) {
            s.store_div_ad_rhs(808, 807, A::tan(A::scale(s.ad_value(807), 0.5)));
        }

        s.v[1186] = if (s.v[985] > 0.005) { 1.0 } else { 0.0 };

        if (((!(s.v[1184] != 0.0)) && (!(s.v[1185] != 0.0))) && (s.v[1186] != 0.0)) {
            s.store_sqrt_ad(807, A::abs(s.ad_value(985)));
        }

        if (((!(s.v[1184] != 0.0)) && (!(s.v[1185] != 0.0))) && (s.v[1186] != 0.0)) {
            s.store_exp_ad(810, A::neg(s.ad_value(807)));
        }

        if (((!(s.v[1184] != 0.0)) && (!(s.v[1185] != 0.0))) && (s.v[1186] != 0.0)) {
            s.store_div_ad(808, A::mul(s.ad_value(807), A::offset(s.ad_value(810), 1.0)), A::sub_from_scalar(1.0, s.ad_value(810)));
        }

        if (((!(s.v[1184] != 0.0)) && (!(s.v[1185] != 0.0))) && (!(s.v[1186] != 0.0))) {
            s.store_offset_ad(808, A::mul(A::scale(s.ad_value(985), 0.1666666666667), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(985), 0.0166666666667), A::sub_from_scalar(1.0, A::scale(s.ad_value(985), 0.0238095238095))))), 2.0);
        }

        s.v[1187] = if (((1.01 * s.v[983]) + s.v[808]) > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1184] != 0.0)) && (s.v[1187] != 0.0)) {
            s.store_add(793, 983, 808);
        }

    }

    pub(super) fn stamp_transient_block_14(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[1188] = if ((s.v[986] * s.v[983]) < (((0.9 * s.v[983]) * s.v[983]) * s.v[793])) { 1.0 } else { 0.0 };

        if (((!(s.v[1184] != 0.0)) && (s.v[1187] != 0.0)) && (s.v[1188] != 0.0)) {
            s.store_offset_ad(982, A::div(s.ad_value(986), s.ad_value(793)), 1e-80);
        }

        if (((!(s.v[1184] != 0.0)) && (s.v[1187] != 0.0)) && (s.v[1188] != 0.0)) {
            s.store_sub(984, 982, 983);
        }

        if (((!(s.v[1184] != 0.0)) && (s.v[1187] != 0.0)) && (s.v[1188] != 0.0)) {
            s.store_div(981, 984, 912);
        }

        s.v[1189] = if (s.v[985] > 0.005) { 1.0 } else { 0.0 };

        if ((((!(s.v[1184] != 0.0)) && (s.v[1187] != 0.0)) && (!(s.v[1188] != 0.0))) && (s.v[1189] != 0.0)) {
            s.store_sub_ad_lhs(794, A::ln(A::div(A::scale(s.ad_value(985), 4.0), A::sub_from_scalar(1.0, A::mul(s.ad_value(810), A::sub_from_scalar(2.0, s.ad_value(810)))))), 807);
        }

        s.v[1190] = if (s.v[985] < (-0.005)) { 1.0 } else { 0.0 };

        if (((((!(s.v[1184] != 0.0)) && (s.v[1187] != 0.0)) && (!(s.v[1188] != 0.0))) && (!(s.v[1189] != 0.0))) && (s.v[1190] != 0.0)) {
            s.store_sin_ad(795, A::scale(s.ad_value(807), 0.5));
        }

        if (((((!(s.v[1184] != 0.0)) && (s.v[1187] != 0.0)) && (!(s.v[1188] != 0.0))) && (!(s.v[1189] != 0.0))) && (s.v[1190] != 0.0)) {
            s.store_ln_ad(794, A::div(A::neg(s.ad_value(985)), A::square(s.ad_value(795))));
        }

        if (((((!(s.v[1184] != 0.0)) && (s.v[1187] != 0.0)) && (!(s.v[1188] != 0.0))) && (!(s.v[1189] != 0.0))) && (!(s.v[1190] != 0.0))) {
            s.store_ln_ad(794, A::sub_from_scalar(4.0, A::mul(A::scale(s.ad_value(985), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(985), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(985), 0.0396825396825397)))))));
        }

        if (((!(s.v[1184] != 0.0)) && (s.v[1187] != 0.0)) && (!(s.v[1188] != 0.0))) {
            s.store_sub_ad_lhs(981, A::add(A::add(A::sub(s.ad_value(910), s.ad_value(909)), s.ad_value(980)), A::scale(A::ln(s.ad_value(793)), 2.0)), 794);
        }

        if (((!(s.v[1184] != 0.0)) && (s.v[1187] != 0.0)) && (!(s.v[1188] != 0.0))) {
            s.store_mul(984, 912, 981);
        }

        if (((!(s.v[1184] != 0.0)) && (s.v[1187] != 0.0)) && (!(s.v[1188] != 0.0))) {
            s.store_add(982, 983, 984);
        }

        s.v[1191] = if (s.v[985] > 0.005) { 1.0 } else { 0.0 };

        s.v[1192] = if ((((s.v[980] + s.v[979]) - s.v[909]) - s.v[807]) < 80.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1184] != 0.0)) && (!(s.v[1187] != 0.0))) && (s.v[1191] != 0.0)) && (s.v[1192] != 0.0)) {
            s.store_exp_ad(795, A::sub(A::sub(A::add(s.ad_value(980), s.ad_value(979)), s.ad_value(909)), s.ad_value(807)));
        }

        if ((((!(s.v[1184] != 0.0)) && (!(s.v[1187] != 0.0))) && (s.v[1191] != 0.0)) && (!(s.v[1192] != 0.0))) {
            let assign20980_ad_e21303: A = A::mul(A::offset(A::sub(A::sub(A::add(s.ad_value(980), s.ad_value(979)), s.ad_value(909)), s.ad_value(807)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(A::add(s.ad_value(980), s.ad_value(979)), s.ad_value(909)), s.ad_value(807)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(A::add(s.ad_value(980), s.ad_value(979)), s.ad_value(909)), s.ad_value(807)), (-80.0)), 0.3333333333333), 1.0)), 1.0));
            s.store_scale_ad(795, A::offset(assign20980_ad_e21303, 1.0), 5.54062e34);
        }

        if (((!(s.v[1184] != 0.0)) && (!(s.v[1187] != 0.0))) && (s.v[1191] != 0.0)) {
            s.store_div(794, 795, 884);
        }

        if (((!(s.v[1184] != 0.0)) && (!(s.v[1187] != 0.0))) && (s.v[1191] != 0.0)) {
            s.store_div_ad(793, A::mul(A::scale(s.ad_value(985), 4.0), s.ad_value(794)), A::sub_from_scalar(1.0, A::mul(s.ad_value(810), A::sub_from_scalar(2.0, s.ad_value(810)))));
        }

        s.v[1193] = if (s.v[985] < (-0.005)) { 1.0 } else { 0.0 };

        if ((((!(s.v[1184] != 0.0)) && (!(s.v[1187] != 0.0))) && (!(s.v[1191] != 0.0))) && (s.v[1193] != 0.0)) {
            s.store_sin_ad(794, A::scale(s.ad_value(807), 0.5));
        }

        if ((((!(s.v[1184] != 0.0)) && (!(s.v[1187] != 0.0))) && (!(s.v[1191] != 0.0))) && (s.v[1193] != 0.0)) {
            s.store_div_ad_lhs(793, A::div(A::neg(s.ad_value(985)), A::square(s.ad_value(794))), 986);
        }

        if ((((!(s.v[1184] != 0.0)) && (!(s.v[1187] != 0.0))) && (!(s.v[1191] != 0.0))) && (!(s.v[1193] != 0.0))) {
            s.store_div_ad_lhs(793, A::sub_from_scalar(4.0, A::mul(A::scale(s.ad_value(985), 0.3333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(985), 0.05), A::sub_from_scalar(1.0, A::scale(s.ad_value(985), 0.0396825396825397)))))), 986);
        }

        if ((!(s.v[1184] != 0.0)) && (!(s.v[1187] != 0.0))) {
            s.store_offset_ad(982, A::div(A::sub(s.ad_value(983), s.ad_value(808)), A::sub_from_scalar(1.0, s.ad_value(793))), 1e-80);
        }

        if ((!(s.v[1184] != 0.0)) && (!(s.v[1187] != 0.0))) {
            s.store_sub(984, 982, 983);
        }

        if ((!(s.v[1184] != 0.0)) && (!(s.v[1187] != 0.0))) {
            s.store_div(981, 984, 912);
        }

        s.v[1194] = if (((s.v[910] - s.v[981]) - s.v[979]) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1194] != 0.0) {
            s.store_exp_ad(793, A::sub(A::sub(s.ad_value(910), s.ad_value(981)), s.ad_value(979)));
        }

        if (!(s.v[1194] != 0.0)) {
            let assign21100_ad_e21501: A = A::offset(A::mul(A::offset(A::sub(A::sub(s.ad_value(910), s.ad_value(981)), s.ad_value(979)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::sub(A::sub(s.ad_value(910), s.ad_value(981)), s.ad_value(979)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::sub(A::sub(s.ad_value(910), s.ad_value(981)), s.ad_value(979)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0);
            s.store_scale_ad(793, assign21100_ad_e21501, 5.54062e34);
        }

        s.store_mul(987, 884, 793);

        s.v[990] = 0.0;

        s.v[991] = 0.0;

        s.v[988] = 0.0;

        s.v[989] = 0.0;

        s.v[992] = 0.0;

        s.v[993] = 0.0;

        s.v[1195] = if (s.v[917] > 1e-6) { 1.0 } else { 0.0 };

        if (s.v[1195] != 0.0) {
            s.store_mul(988, 986, 885);
        }

        if (s.v[1195] != 0.0) {
            s.store_mul(989, 987, 886);
        }

        if (s.v[1195] != 0.0) {
            s.store_add_ad_rhs(990, 988, A::scale(s.ad_value(983), 2.0));
        }

        if (s.v[1195] != 0.0) {
            s.store_add_ad_rhs(991, 989, A::scale(s.ad_value(984), 2.0));
        }

        if (s.v[1195] != 0.0) {
            s.store_add_ad_lhs(992, A::add(A::scale(s.ad_value(982), 2.0), s.ad_value(988)), 989);
        }

        s.v[1196] = if (((s.v[985]) as f64).abs() > 0.005) { 1.0 } else { 0.0 };

        if ((s.v[1195] != 0.0) && (s.v[1196] != 0.0)) {
            s.store_add_ad(2, A::add(A::mul(s.ad_value(990), s.ad_value(991)), A::mul(A::scale(A::offset(s.ad_value(980), 2.0), 2.0), s.ad_value(991))), A::mul(A::scale(A::offset(s.ad_value(981), 2.0), 2.0), s.ad_value(990)));
        }

        if ((s.v[1195] != 0.0) && (s.v[1196] != 0.0)) {
            s.store_div_ad(993, A::mul(A::scale(s.ad_value(985), (-4.0)), s.ad_value(992)), A::mul(s.ad_value(982), s.ad_value(2)));
        }

        if ((s.v[1195] != 0.0) && (!(s.v[1196] != 0.0))) {
            s.store_scale_ad(2, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(985), 0.0333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(985), 0.0357142857143), A::sub_from_scalar(1.0, A::scale(s.ad_value(985), 0.0333333333333)))))), 0.1666666666667);
        }

        if ((s.v[1195] != 0.0) && (!(s.v[1196] != 0.0))) {
            s.store_add_ad(3, A::add(A::mul(s.ad_value(990), s.ad_value(986)), A::mul(s.ad_value(991), s.ad_value(987))), A::mul(A::mul(A::mul(s.ad_value(990), s.ad_value(991)), s.ad_value(982)), A::offset(A::mul(s.ad_value(982), s.ad_value(2)), 1.0)));
        }

        if ((s.v[1195] != 0.0) && (!(s.v[1196] != 0.0))) {
            s.store_div_ad(993, A::mul(A::mul(s.ad_value(986), s.ad_value(987)), s.ad_value(992)), A::mul(s.ad_value(982), s.ad_value(3)));
        }

        s.store_add_ad_rhs(994, 979, A::ln(s.ad_value(982)));

        s.store_scaled_add(995, 917, 982, 0.5);

        s.store_sub(996, 994, 930);

        s.v[999] = 1.0;

        s.v[1197] = if (p.p9 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1197] != 0.0) {
            s.store_div_ad_lhs(997, A::scale(A::add(s.ad_value(918), s.ad_value(983)), 0.5), 911);
        }

        if (s.v[1197] != 0.0) {
            s.store_scale_ad(997, A::add(A::offset(s.ad_value(997), 1e-5), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(997), (-1e-5)), A::offset(s.ad_value(997), (-1e-5))), 1.0))), 0.5);
        }

        if (s.v[1197] != 0.0) {
            s.store_sub_ad(1, A::sqrt(A::add(A::div(s.ad_value(997), s.ad_value(227)), A::mul(A::scale(s.ad_value(250), 0.25), s.ad_value(250)))), A::scale(s.ad_value(250), 0.5));
        }

        if (s.v[1197] != 0.0) {
            s.store_mul_ad_lhs(998, A::powf(s.ad_value(1), 2.0), 227);
        }

        if (s.v[1197] != 0.0) {
            s.store_sub_from_scalar_ad(999, 1.0, A::div(s.ad_value(998), s.ad_value(997)));
        }

        s.v[1198] = if ((s.v[983] / 2.0) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1198] != 0.0) {
            s.store_ln_ad(2, A::offset(A::exp(A::scale(s.ad_value(983), 0.5)), 1.0));
        }

        if (!(s.v[1198] != 0.0)) {
            s.store_scale(2, 983, 0.5);
        }

        s.store_scale(1000, 2, 2.0);

        s.v[1199] = if ((s.v[984] / 2.0) < 80.0) { 1.0 } else { 0.0 };

        if (s.v[1199] != 0.0) {
            s.store_ln_ad(3, A::offset(A::exp(A::scale(s.ad_value(984), 0.5)), 1.0));
        }

        if (!(s.v[1199] != 0.0)) {
            s.store_scale(3, 984, 0.5);
        }

        s.store_scale(1001, 3, 2.0);

        s.store_sub(1002, 1001, 984);

        s.store_sub(1003, 1000, 983);

        s.store_add_ad(1004, A::mul(s.ad_value(270), s.ad_value(1000)), A::mul(s.ad_value(271), s.ad_value(1002)));

        s.store_add_ad(1005, A::mul(s.ad_value(270), s.ad_value(1001)), A::mul(s.ad_value(271), s.ad_value(1003)));

        s.store_scaled_add(1006, 931, 1000, 0.5);

        s.store_scaled_add(1007, 932, 1001, 0.5);

        s.store_div_from_scalar_ad(0, 1.0, A::add(s.ad_value(1006), s.ad_value(1007)));

        s.store_mul_ad_lhs(1008, A::mul(s.ad_value(995), s.ad_value(1006)), 0);

        s.store_mul_ad_lhs(1009, A::mul(s.ad_value(995), s.ad_value(1007)), 0);

        s.store_scaled_add(1010, 933, 1002, 0.5);

        s.store_scaled_add(1011, 934, 1003, 0.5);

        s.store_scaled_add(1012, 935, 1004, 0.5);

        s.store_scaled_add(1013, 936, 1005, 0.5);

        s.store_mul_ad_lhs(1014, A::mul(A::mul(s.ad_value(1006), s.ad_value(191)), A::exp(A::mul(s.ad_value(40), s.ad_value(295)))), 999);

        s.store_mul_ad(1015, A::mul(s.ad_value(1007), s.ad_value(192)), A::exp(A::mul(s.ad_value(40), s.ad_value(295))));

        s.store_add(1016, 1014, 1015);

        s.store_mul_ad_rhs(2, 50, A::add(s.ad_value(1010), A::mul(s.ad_value(51), s.ad_value(1011))));

        s.store_scale_ad(3, A::add(A::offset(s.ad_value(2), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2), 1.0), A::offset(s.ad_value(2), 1.0)), 0.01))), 0.5);

        s.store_scale_ad(4, A::add(A::offset(A::scale(s.ad_value(2), 0.2), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::scale(s.ad_value(2), 0.2), 1.0), A::offset(A::scale(s.ad_value(2), 0.2), 1.0)), 0.01))), 0.5);

        s.store_div(1017, 3, 4);

        s.store_mul_ad(1018, A::mul(s.ad_value(33), A::add(A::offset(A::mul(s.ad_value(41), s.ad_value(1010)), 1.0), A::mul(s.ad_value(42), s.ad_value(1011)))), A::exp(A::mul(A::neg(s.ad_value(44)), A::ln(A::add(A::offset(A::mul(s.ad_value(1008), s.ad_value(268)), 1.0), A::mul(s.ad_value(1009), s.ad_value(269)))))));

        s.v[1200] = if (s.v[56] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1200] != 0.0) {
            s.store_scalar(4, 1.0);
        }

        s.v[1201] = if (s.v[56] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1200] != 0.0)) && (s.v[1201] != 0.0)) {
            s.store_mul_ad_rhs(2, 56, A::exp(A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(995), 1e-12)))));
        }

        if ((!(s.v[1200] != 0.0)) && (s.v[1201] != 0.0)) {
            s.store_sub_from_scalar(4, 1.0, 2);
        }

        if ((!(s.v[1200] != 0.0)) && (!(s.v[1201] != 0.0))) {
            s.store_mul_ad_rhs(2, 56, A::exp(A::mul(s.ad_value(57), A::ln(A::offset(s.ad_value(995), 1e-12)))));
        }

        if ((!(s.v[1200] != 0.0)) && (!(s.v[1201] != 0.0))) {
            s.store_div_from_scalar_ad(4, 1.0, A::offset(s.ad_value(2), 1.0));
        }

        s.store_mul_ad_rhs(1019, 943, A::add(A::mul(s.ad_value(995), s.ad_value(4)), s.ad_value(54)));

        s.store_add_ad(1020, A::add(A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1012)), 1e-6)))), 1.0), s.ad_value(1018)), A::mul(s.ad_value(38), s.ad_value(1019)));

        s.store_add_ad(1021, A::add(A::offset(A::exp(A::mul(s.ad_value(47), A::ln(A::offset(A::mul(s.ad_value(267), s.ad_value(1013)), 1e-6)))), 1.0), s.ad_value(1018)), A::mul(s.ad_value(39), s.ad_value(1019)));

        s.store_div_ad(1022, A::mul(s.ad_value(1017), s.ad_value(1016)), A::add(A::div(s.ad_value(1014), s.ad_value(1020)), A::div(s.ad_value(1015), s.ad_value(1021))));

        s.store_div_from_scalar_ad(1023, 1.0, A::offset(s.ad_value(995), 4.0));

        s.v[1202] = if (s.v[65] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1202] != 0.0) {
            s.store_div_from_scalar_ad(0, 1.0, A::offset(A::mul(s.ad_value(65), s.ad_value(1009)), 1.0));
        }

        if (!(s.v[1202] != 0.0)) {
            s.store_sub_from_scalar_ad(0, 1.0, A::mul(s.ad_value(65), s.ad_value(1009)));
        }

        s.store_mul_ad_lhs(1024, A::mul(s.ad_value(995), s.ad_value(1023)), 0);

        s.store_mul_ad_lhs(1025, A::ln(A::offset(A::div(A::sub(s.ad_value(339), s.ad_value(979)), A::add(A::mul(s.ad_value(66), s.ad_value(227)), A::mul(A::mul(s.ad_value(67), s.ad_value(995)), s.ad_value(995)))), 1.0)), 1024);

        s.store_mul(1026, 877, 1025);

        s.store_div_from_scalar_ad(1027, 1.0, A::offset(A::mul(s.ad_value(1026), A::offset(s.ad_value(1026), 1.0)), 1.0));

        s.store_div_ad(955, A::scale(s.ad_value(1006), 100.0), A::offset(s.ad_value(1006), 100.0));

        s.v[1203] = if (s.v[61] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1203] != 0.0) {
            s.store_div_from_scalar_ad(956, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(61), s.ad_value(955))));
        }

        if (!(s.v[1203] != 0.0)) {
            s.store_offset_ad(956, A::mul(s.ad_value(61), s.ad_value(955)), 1.0);
        }

        s.store_div_ad(957, A::scale(s.ad_value(1007), 100.0), A::offset(s.ad_value(1007), 100.0));

        s.v[1204] = if (s.v[62] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1204] != 0.0) {
            s.store_div_from_scalar_ad(958, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(62), s.ad_value(957))));
        }

        if (!(s.v[1204] != 0.0)) {
            s.store_offset_ad(958, A::mul(s.ad_value(62), s.ad_value(957)), 1.0);
        }

        s.store_mul_ad(1028, A::scale(A::mul(s.ad_value(875), s.ad_value(996)), 0.5), A::add(s.ad_value(956), s.ad_value(958)));

        s.store_div_ad_rhs(1029, 1028, A::mul(s.ad_value(1022), s.ad_value(1027)));

        s.store_square(1030, 1029);

        s.store_sqrt_ad(1031, A::offset(s.ad_value(1030), 1.0));

        s.store_div_ad_lhs(1032, A::offset(A::scale(s.ad_value(1030), 1.5), 1.0), 1031);

        s.v[1205] = if (p.p13 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1205] != 0.0) {
            s.store_mul_ad(2, A::scale(s.ad_value(258), 0.6), A::exp(A::scale(A::ln(A::offset(A::square(s.ad_value(1006)), 60.0)), (-0.1666666666667))));
        }

        if (s.v[1205] != 0.0) {
            s.store_mul_ad(3, A::scale(s.ad_value(258), 0.6), A::exp(A::scale(A::ln(A::offset(A::square(s.ad_value(1007)), 60.0)), (-0.1666666666667))));
        }

        if (s.v[1205] != 0.0) {
            s.store_div_ad_lhs(1033, A::offset(A::mul(s.ad_value(911), s.ad_value(2)), 1.0), 892);
        }

        if (s.v[1205] != 0.0) {
            s.store_div_ad_lhs(1034, A::offset(A::mul(s.ad_value(912), s.ad_value(3)), 1.0), 893);
        }

        if (!(s.v[1205] != 0.0)) {
            s.store_scalar(1033, 1.0);
        }

        if (!(s.v[1205] != 0.0)) {
            s.store_scalar(1034, 1.0);
        }

        s.v[1206] = if (s.v[917] > 1e-6) { 1.0 } else { 0.0 };

        s.v[1207] = if (s.v[982] > 1e-6) { 1.0 } else { 0.0 };

        s.v[1208] = if (((s.v[991]) as f64).abs() < 0.01) { 1.0 } else { 0.0 };

        if (((s.v[1206] != 0.0) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) {
            s.store_div_ad(0, A::add(A::offset(s.ad_value(980), 2.0), A::scale(s.ad_value(990), 0.5)), A::mul(A::offset(s.ad_value(981), 2.0), s.ad_value(990)));
        }

        if (((s.v[1206] != 0.0) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) {
            s.store_mul(2, 0, 991);
        }

        if (((s.v[1206] != 0.0) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) {
            s.store_square(3, 2);
        }

        if (((s.v[1206] != 0.0) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) {
            s.store_add_ad_lhs(4, A::sub_from_scalar(1.0, s.ad_value(2)), 3);
        }

        if (((s.v[1206] != 0.0) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) {
            s.store_sub_ad_rhs(5, 4, A::mul(s.ad_value(2), s.ad_value(3)));
        }

        if (((s.v[1206] != 0.0) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) {
            s.store_div_ad(2, A::sub(s.ad_value(984), A::mul(A::mul(A::scale(s.ad_value(985), 2.0), A::sub(s.ad_value(0), A::div_from_scalar(1.0, s.ad_value(990)))), s.ad_value(5))), A::offset(s.ad_value(981), 2.0));
        }

        if (((s.v[1206] != 0.0) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) {
            s.store_div_ad_lhs(1035, A::sub(A::div(A::sub(A::mul(s.ad_value(993), s.ad_value(982)), s.ad_value(986)), s.ad_value(990)), s.ad_value(2)), 982);
        }

        if (((s.v[1206] != 0.0) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) {
            s.store_div_ad(1036, A::mul(s.ad_value(1035), s.ad_value(982)), A::offset(s.ad_value(1035), 1.0));
        }

        if (((s.v[1206] != 0.0) && (s.v[1207] != 0.0)) && (!(s.v[1208] != 0.0))) {
            s.store_sub_ad(1035, A::div(A::mul(s.ad_value(993), s.ad_value(992)), A::mul(s.ad_value(990), s.ad_value(991))), A::div(A::add(A::div(s.ad_value(986), s.ad_value(990)), A::div(s.ad_value(987), s.ad_value(991))), s.ad_value(982)));
        }

        if (((s.v[1206] != 0.0) && (s.v[1207] != 0.0)) && (!(s.v[1208] != 0.0))) {
            s.store_div_ad(1036, A::mul(s.ad_value(1035), s.ad_value(982)), A::offset(s.ad_value(1035), 1.0));
        }

        if ((s.v[1206] != 0.0) && (!(s.v[1207] != 0.0))) {
            s.copy_ad(1036, 953);
        }

        if (s.v[1206] != 0.0) {
            s.store_sub(2, 1036, 960);
        }

        if (s.v[1206] != 0.0) {
            s.store_offset_ad(3, A::mul(A::scale(s.ad_value(2), 36.0), s.ad_value(2)), 1.0);
        }

        s.v[1209] = if (((s.v[2]) as f64).abs() > 0.001) { 1.0 } else { 0.0 };

        if ((s.v[1206] != 0.0) && (s.v[1209] != 0.0)) {
            s.store_sub(4, 982, 917);
        }

        if ((s.v[1206] != 0.0) && (s.v[1209] != 0.0)) {
            s.store_sub_ad_rhs(1037, 4, A::mul(s.ad_value(1036), s.ad_value(996)));
        }

        if ((s.v[1206] != 0.0) && (s.v[1209] != 0.0)) {
            s.store_sub_ad_rhs(1038, 4, A::mul(s.ad_value(960), s.ad_value(996)));
        }

        if ((s.v[1206] != 0.0) && (s.v[1209] != 0.0)) {
            s.store_sqrt_ad(1039, A::add(A::square(s.ad_value(1037)), s.ad_value(3)));
        }

        if ((s.v[1206] != 0.0) && (s.v[1209] != 0.0)) {
            s.store_sqrt_ad(1040, A::add(A::square(s.ad_value(1038)), s.ad_value(3)));
        }

        if ((s.v[1206] != 0.0) && (s.v[1209] != 0.0)) {
            s.store_mul_ad(1041, A::div_from_scalar(0.25, s.ad_value(2)), A::add(A::sub(A::mul(s.ad_value(1040), s.ad_value(1037)), A::mul(s.ad_value(1039), s.ad_value(1038))), A::mul(s.ad_value(3), A::ln(A::div(A::add(s.ad_value(1038), s.ad_value(1040)), A::add(s.ad_value(1037), s.ad_value(1039)))))));
        }

        if ((s.v[1206] != 0.0) && (!(s.v[1209] != 0.0))) {
            s.store_mul(4, 996, 2);
        }

        if ((s.v[1206] != 0.0) && (!(s.v[1209] != 0.0))) {
            s.store_div_ad(1041, A::mul(A::mul(A::scale(s.ad_value(996), ((-0.25) * 0.1666666666667)), s.ad_value(4)), s.ad_value(4)), A::sqrt(s.ad_value(3)));
        }

        if (!(s.v[1206] != 0.0)) {
            s.copy_ad(1036, 953);
        }

        if (!(s.v[1206] != 0.0)) {
            s.store_scalar(1041, 0.0);
        }

        s.store_sub_ad_lhs(1042, A::add(A::add(A::mul(s.ad_value(995), s.ad_value(996)), s.ad_value(1041)), s.ad_value(917)), 982);

        s.v[1210] = if (s.v[917] > 1e-6) { 1.0 } else { 0.0 };

        s.v[1211] = if (s.v[1042] > 1e-30) { 1.0 } else { 0.0 };

        if ((s.v[1210] != 0.0) && (s.v[1211] != 0.0)) {
            s.store_div_ad_rhs(1043, 926, A::sub(A::div(s.ad_value(922), s.ad_value(917)), s.ad_value(929)));
        }

        if ((s.v[1210] != 0.0) && (s.v[1211] != 0.0)) {
            s.store_div_ad_rhs(1044, 990, A::sub(A::div(s.ad_value(986), s.ad_value(982)), s.ad_value(993)));
        }

        if ((s.v[1210] != 0.0) && (s.v[1211] != 0.0)) {
            s.store_div_ad_lhs(1045, A::sub(s.ad_value(1043), s.ad_value(1044)), 1042);
        }

        if ((s.v[1210] != 0.0) && (s.v[1211] != 0.0)) {
            s.store_div_ad_rhs(1046, 927, A::sub(A::div(s.ad_value(923), s.ad_value(917)), s.ad_value(929)));
        }

        if ((s.v[1210] != 0.0) && (s.v[1211] != 0.0)) {
            s.store_div_ad_rhs(1047, 991, A::sub(A::div(s.ad_value(987), s.ad_value(982)), s.ad_value(993)));
        }

        if ((s.v[1210] != 0.0) && (s.v[1211] != 0.0)) {
            s.store_div_ad_lhs(1048, A::sub(s.ad_value(1046), s.ad_value(1047)), 1042);
        }

        if ((s.v[1210] != 0.0) && (!(s.v[1211] != 0.0))) {
            s.store_scalar(1045, 0.0);
        }

        if ((s.v[1210] != 0.0) && (!(s.v[1211] != 0.0))) {
            s.store_scalar(1048, 0.0);
        }

        if (!(s.v[1210] != 0.0)) {
            s.store_mul_ad(1049, A::scale(s.ad_value(948), (-2.0)), A::add(A::div(s.ad_value(885), s.ad_value(951)), s.ad_value(954)));
        }

    }

    pub(super) fn stamp_transient_block_15(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (!(s.v[1210] != 0.0)) {
            s.store_mul_ad(1050, A::scale(s.ad_value(949), (-2.0)), A::add(A::div(s.ad_value(886), s.ad_value(952)), s.ad_value(954)));
        }

        if (!(s.v[1210] != 0.0)) {
            s.store_mul_ad_lhs(0, A::sub(s.ad_value(1050), s.ad_value(1049)), 954);
        }

        if (!(s.v[1210] != 0.0)) {
            s.store_mul(2, 1049, 885);
        }

        if (!(s.v[1210] != 0.0)) {
            s.store_mul(3, 1050, 886);
        }

        if (!(s.v[1210] != 0.0)) {
            s.store_add(4, 2, 3);
        }

        if (!(s.v[1210] != 0.0)) {
            s.store_offset_ad(5, A::scale(A::add(A::mul(s.ad_value(948), s.ad_value(885)), A::mul(s.ad_value(949), s.ad_value(886))), 2.0), 3.0);
        }

        if (!(s.v[1210] != 0.0)) {
            s.store_div_ad_lhs(1051, A::sub(A::add(s.ad_value(3), s.ad_value(0)), A::div(s.ad_value(4), s.ad_value(951))), 5);
        }

        if (!(s.v[1210] != 0.0)) {
            s.store_div_ad_lhs(1052, A::sub(A::sub(s.ad_value(2), s.ad_value(0)), A::div(s.ad_value(4), s.ad_value(952))), 5);
        }

        if (!(s.v[1210] != 0.0)) {
            s.store_mul_ad(1045, A::neg(s.ad_value(951)), A::add(A::mul(s.ad_value(1051), s.ad_value(951)), s.ad_value(954)));
        }

        if (!(s.v[1210] != 0.0)) {
            s.store_mul_ad(1048, A::neg(s.ad_value(952)), A::add(A::mul(s.ad_value(1052), s.ad_value(952)), s.ad_value(954)));
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

        s.copy_ad(427, 1028);

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

        s.store_div_ad(342, A::scale(s.ad_value(421), p.p35), A::add(s.ad_value(417), s.ad_value(418)));

        s.store_mul_ad_lhs(343, A::add(s.ad_value(63), A::mul(s.ad_value(275), s.ad_value(423))), 424);

        s.store_mul_ad_lhs(344, A::offset(A::mul(s.ad_value(343), A::offset(s.ad_value(343), 1.0)), 1.0), 425);

        s.store_mul_ad_lhs(345, A::mul(s.ad_value(422), s.ad_value(425)), 426);

        s.v[1212] = if (p.p13 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1212] != 0.0) {
            s.store_div_ad(346, A::add(s.ad_value(417), s.ad_value(418)), A::add(A::div(s.ad_value(417), s.ad_value(430)), A::div(s.ad_value(418), s.ad_value(431))));
        }

        if (!(s.v[1212] != 0.0)) {
            s.store_scalar(346, 1.0);
        }

        s.store_mul_ad_lhs(347, A::square(s.ad_value(226)), 342);

        s.store_div_ad_lhs(348, A::div(A::mul(A::mul(A::mul(s.ad_value(347), s.ad_value(390)), s.ad_value(433)), s.ad_value(344)), s.ad_value(345)), 346);

        s.store_mul_ad_lhs(704, A::neg(s.ad_value(330)), 224);

        s.store_mul_ad_lhs(705, A::neg(s.ad_value(332)), 224);

        s.store_add_ad_lhs(0, A::mul(A::scale(s.ad_value(163), p.p14), s.ad_value(224)), 234);

        s.store_add(706, 704, 0);

        s.store_add(707, 705, 0);

        s.v[714] = 0.0;

        s.v[715] = 0.0;

        s.v[716] = 0.0;

        s.v[717] = 0.0;

        s.store_div_ad_lhs(708, A::sqrt(A::mul(A::mul(A::scale(s.ad_value(19), (2.0 * 1.602176565e-19)), s.ad_value(229)), s.ad_value(224))), 241);

        s.store_square(709, 708);

        s.store_offset_scaled(710, 708, 0.707106781186545, 1.0);

        s.store_scale(711, 710, 1e-5);

        s.store_div_from_scalar(712, 1.0, 710);

        s.store_div_from_scalar_ad(713, 1.0, A::offset(A::scale(s.ad_value(708), 0.7324648775608221), 1.25));

        s.v[1213] = if (((p.p3 > 0.0) && ((s.v[69] > 0.0) || (s.v[71] > 0.0))) || ((p.p4 > 0.0) && (s.v[89] > 0.0))) { 1.0 } else { 0.0 };

        s.v[1214] = if (((s.v[704]) as f64).abs() <= s.v[711]) { 1.0 } else { 0.0 };

        if ((s.v[1213] != 0.0) && (s.v[1214] != 0.0)) {
            s.store_mul_ad_lhs(714, A::neg(s.ad_value(704)), 712);
        }

        s.v[1215] = if (s.v[704] < (-s.v[711])) { 1.0 } else { 0.0 };

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (s.v[1215] != 0.0)) {
            s.store_neg(683, 704);
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (s.v[1215] != 0.0)) {
            s.store_mul_ad_lhs(684, A::scale(s.ad_value(683), 1.25), 712);
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (s.v[1215] != 0.0)) {
            s.store_scale_ad(685, A::sub(A::offset(s.ad_value(684), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(684), (-6.0)), A::offset(s.ad_value(684), (-6.0))), 64.0))), 0.5);
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (s.v[1215] != 0.0)) {
            s.store_add_ad(686, A::mul(A::sub(s.ad_value(683), s.ad_value(685)), A::sub(s.ad_value(683), s.ad_value(685))), A::mul(s.ad_value(709), A::offset(s.ad_value(685), 1.0)));
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (s.v[1215] != 0.0)) {
            s.store_sub_ad_lhs(687, A::scale(A::sub(s.ad_value(683), s.ad_value(685)), 2.0), 709);
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (s.v[1215] != 0.0)) {
            s.store_sub_ad_lhs(688, A::ln(A::div(s.ad_value(686), s.ad_value(709))), 685);
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (s.v[1215] != 0.0)) {
            s.store_add(689, 686, 687);
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (s.v[1215] != 0.0)) {
            s.store_add_ad(690, A::square(s.ad_value(689)), A::mul(s.ad_value(688), A::sub(A::mul(A::scale(s.ad_value(687), 0.5), s.ad_value(687)), s.ad_value(686))));
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (s.v[1215] != 0.0)) {
            s.store_add_ad_rhs(691, 690, A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(689), s.ad_value(690)), s.ad_value(688)), s.ad_value(688)), s.ad_value(687)), A::sub(A::scale(A::square(s.ad_value(687)), 0.3333333333333), s.ad_value(686))));
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (s.v[1215] != 0.0)) {
            s.store_add_ad_rhs(692, 685, A::div(A::mul(A::mul(s.ad_value(686), s.ad_value(689)), s.ad_value(688)), s.ad_value(691)));
        }

        s.v[1216] = if (((s.v[692]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (s.v[1215] != 0.0)) && (s.v[1216] != 0.0)) {
            s.store_exp(693, 692);
        }

        s.v[1217] = if (s.v[692] < (-80.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (s.v[1215] != 0.0)) && (!(s.v[1216] != 0.0))) && (s.v[1217] != 0.0)) {
            s.store_div_from_scalar_ad(693, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(692)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(692)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(692)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (s.v[1215] != 0.0)) && (!(s.v[1216] != 0.0))) && (!(s.v[1217] != 0.0))) {
            s.store_scale_ad(693, A::offset(A::mul(A::offset(s.ad_value(692), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(692), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(692), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (s.v[1215] != 0.0)) {
            s.store_sub(691, 683, 692);
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (s.v[1215] != 0.0)) {
            s.store_add_ad(694, A::scale(s.ad_value(691), 2.0), A::mul(s.ad_value(709), A::offset(s.ad_value(693), (-1.0))));
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (s.v[1215] != 0.0)) {
            s.store_add_ad(695, A::square(s.ad_value(691)), A::mul(s.ad_value(709), A::sub(A::offset(s.ad_value(692), 1.0), s.ad_value(693))));
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (s.v[1215] != 0.0)) {
            s.store_sub_from_scalar_ad(696, 1.0, A::mul(A::scale(s.ad_value(709), 0.5), s.ad_value(693)));
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (s.v[1215] != 0.0)) {
            s.store_sub_ad(691, A::square(s.ad_value(694)), A::scale(A::mul(s.ad_value(696), s.ad_value(695)), 4.0));
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (s.v[1215] != 0.0)) {
            s.store_div_ad(697, A::scale(s.ad_value(695), 2.0), A::add(s.ad_value(694), A::sqrt(s.ad_value(691))));
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (s.v[1215] != 0.0)) {
            s.store_neg_ad(714, A::add(s.ad_value(692), s.ad_value(697)));
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (!(s.v[1215] != 0.0))) {
            s.store_mul_ad_lhs(698, A::offset(A::mul(A::scale(s.ad_value(710), 1.25), s.ad_value(713)), (-1.0)), 713);
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (!(s.v[1215] != 0.0))) {
            s.store_mul_ad(699, A::mul(s.ad_value(704), s.ad_value(712)), A::offset(A::mul(s.ad_value(698), s.ad_value(704)), 1.0));
        }

        s.v[1218] = if ((((-s.v[699])) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (!(s.v[1215] != 0.0))) && (s.v[1218] != 0.0)) {
            s.store_exp_ad(691, A::neg(s.ad_value(699)));
        }

        s.v[1219] = if ((-s.v[699]) < (-80.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (!(s.v[1215] != 0.0))) && (!(s.v[1218] != 0.0))) && (s.v[1219] != 0.0)) {
            s.store_div_from_scalar_ad(691, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::neg(s.ad_value(699))), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::neg(s.ad_value(699))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(699))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (!(s.v[1215] != 0.0))) && (!(s.v[1218] != 0.0))) && (!(s.v[1219] != 0.0))) {
            s.store_scale_ad(691, A::offset(A::mul(A::offset(A::neg(s.ad_value(699)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(699)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(699)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (!(s.v[1215] != 0.0))) {
            s.store_sub_from_scalar(697, 1.0, 691);
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (!(s.v[1215] != 0.0))) {
            s.store_sub_ad(700, A::add(s.ad_value(704), A::scale(s.ad_value(709), 0.5)), A::mul(s.ad_value(708), A::sqrt(A::sub(A::add(s.ad_value(704), A::scale(s.ad_value(709), 0.25)), s.ad_value(697)))));
        }

        s.v[1220] = if ((((-s.v[700])) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (!(s.v[1215] != 0.0))) && (s.v[1220] != 0.0)) {
            s.store_exp_ad(693, A::neg(s.ad_value(700)));
        }

        s.v[1221] = if ((-s.v[700]) < (-80.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (!(s.v[1215] != 0.0))) && (!(s.v[1220] != 0.0))) && (s.v[1221] != 0.0)) {
            s.store_div_from_scalar_ad(693, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::neg(s.ad_value(700))), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::neg(s.ad_value(700))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(700))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (!(s.v[1215] != 0.0))) && (!(s.v[1220] != 0.0))) && (!(s.v[1221] != 0.0))) {
            s.store_scale_ad(693, A::offset(A::mul(A::offset(A::neg(s.ad_value(700)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(700)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(700)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (!(s.v[1215] != 0.0))) {
            s.store_add_ad(694, A::scale(A::sub(s.ad_value(704), s.ad_value(700)), 2.0), A::mul(s.ad_value(709), A::sub_from_scalar(1.0, s.ad_value(693))));
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (!(s.v[1215] != 0.0))) {
            s.store_sub_ad(695, A::mul(A::sub(s.ad_value(704), s.ad_value(700)), A::sub(s.ad_value(704), s.ad_value(700))), A::mul(s.ad_value(709), A::add(A::offset(s.ad_value(700), (-1.0)), s.ad_value(693))));
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (!(s.v[1215] != 0.0))) {
            s.store_sub_from_scalar_ad(696, 1.0, A::mul(A::scale(s.ad_value(709), 0.5), s.ad_value(693)));
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (!(s.v[1215] != 0.0))) {
            s.store_sub_ad(691, A::square(s.ad_value(694)), A::scale(A::mul(s.ad_value(696), s.ad_value(695)), 4.0));
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (!(s.v[1215] != 0.0))) {
            s.store_div_ad(701, A::scale(s.ad_value(695), 2.0), A::add(s.ad_value(694), A::sqrt(s.ad_value(691))));
        }

        if (((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) && (!(s.v[1215] != 0.0))) {
            s.store_add(714, 700, 701);
        }

        if ((s.v[1213] != 0.0) && (!(s.v[1214] != 0.0))) {
            s.store_neg(714, 714);
        }

        s.v[1222] = if (s.v[159] > 0.0) { 1.0 } else { 0.0 };

        s.v[1223] = if (((s.v[706]) as f64).abs() <= s.v[711]) { 1.0 } else { 0.0 };

        if ((s.v[1222] != 0.0) && (s.v[1223] != 0.0)) {
            s.store_mul_ad_lhs(716, A::neg(s.ad_value(706)), 712);
        }

        s.v[1224] = if (s.v[706] < (-s.v[711])) { 1.0 } else { 0.0 };

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (s.v[1224] != 0.0)) {
            s.store_neg(683, 706);
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (s.v[1224] != 0.0)) {
            s.store_mul_ad_lhs(684, A::scale(s.ad_value(683), 1.25), 712);
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (s.v[1224] != 0.0)) {
            s.store_scale_ad(685, A::sub(A::offset(s.ad_value(684), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(684), (-6.0)), A::offset(s.ad_value(684), (-6.0))), 64.0))), 0.5);
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (s.v[1224] != 0.0)) {
            s.store_add_ad(686, A::mul(A::sub(s.ad_value(683), s.ad_value(685)), A::sub(s.ad_value(683), s.ad_value(685))), A::mul(s.ad_value(709), A::offset(s.ad_value(685), 1.0)));
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (s.v[1224] != 0.0)) {
            s.store_sub_ad_lhs(687, A::scale(A::sub(s.ad_value(683), s.ad_value(685)), 2.0), 709);
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (s.v[1224] != 0.0)) {
            s.store_sub_ad_lhs(688, A::ln(A::div(s.ad_value(686), s.ad_value(709))), 685);
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (s.v[1224] != 0.0)) {
            s.store_add(689, 686, 687);
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (s.v[1224] != 0.0)) {
            s.store_add_ad(690, A::square(s.ad_value(689)), A::mul(s.ad_value(688), A::sub(A::mul(A::scale(s.ad_value(687), 0.5), s.ad_value(687)), s.ad_value(686))));
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (s.v[1224] != 0.0)) {
            s.store_add_ad_rhs(691, 690, A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(689), s.ad_value(690)), s.ad_value(688)), s.ad_value(688)), s.ad_value(687)), A::sub(A::scale(A::square(s.ad_value(687)), 0.3333333333333), s.ad_value(686))));
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (s.v[1224] != 0.0)) {
            s.store_add_ad_rhs(692, 685, A::div(A::mul(A::mul(s.ad_value(686), s.ad_value(689)), s.ad_value(688)), s.ad_value(691)));
        }

        s.v[1225] = if (((s.v[692]) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (s.v[1224] != 0.0)) && (s.v[1225] != 0.0)) {
            s.store_exp(693, 692);
        }

        s.v[1226] = if (s.v[692] < (-80.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (s.v[1224] != 0.0)) && (!(s.v[1225] != 0.0))) && (s.v[1226] != 0.0)) {
            s.store_div_from_scalar_ad(693, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(s.ad_value(692)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(692)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(692)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (s.v[1224] != 0.0)) && (!(s.v[1225] != 0.0))) && (!(s.v[1226] != 0.0))) {
            s.store_scale_ad(693, A::offset(A::mul(A::offset(s.ad_value(692), (-80.0)), A::offset(A::mul(A::scale(A::offset(s.ad_value(692), (-80.0)), 0.5), A::offset(A::scale(A::offset(s.ad_value(692), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (s.v[1224] != 0.0)) {
            s.store_sub(691, 683, 692);
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (s.v[1224] != 0.0)) {
            s.store_add_ad(694, A::scale(s.ad_value(691), 2.0), A::mul(s.ad_value(709), A::offset(s.ad_value(693), (-1.0))));
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (s.v[1224] != 0.0)) {
            s.store_add_ad(695, A::square(s.ad_value(691)), A::mul(s.ad_value(709), A::sub(A::offset(s.ad_value(692), 1.0), s.ad_value(693))));
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (s.v[1224] != 0.0)) {
            s.store_sub_from_scalar_ad(696, 1.0, A::mul(A::scale(s.ad_value(709), 0.5), s.ad_value(693)));
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (s.v[1224] != 0.0)) {
            s.store_sub_ad(691, A::square(s.ad_value(694)), A::scale(A::mul(s.ad_value(696), s.ad_value(695)), 4.0));
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (s.v[1224] != 0.0)) {
            s.store_div_ad(697, A::scale(s.ad_value(695), 2.0), A::add(s.ad_value(694), A::sqrt(s.ad_value(691))));
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (s.v[1224] != 0.0)) {
            s.store_neg_ad(716, A::add(s.ad_value(692), s.ad_value(697)));
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (!(s.v[1224] != 0.0))) {
            s.store_mul_ad_lhs(698, A::offset(A::mul(A::scale(s.ad_value(710), 1.25), s.ad_value(713)), (-1.0)), 713);
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (!(s.v[1224] != 0.0))) {
            s.store_mul_ad(699, A::mul(s.ad_value(706), s.ad_value(712)), A::offset(A::mul(s.ad_value(698), s.ad_value(706)), 1.0));
        }

        s.v[1227] = if ((((-s.v[699])) as f64).abs() < 80.0) { 1.0 } else { 0.0 };

        if ((((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (!(s.v[1224] != 0.0))) && (s.v[1227] != 0.0)) {
            s.store_exp_ad(691, A::neg(s.ad_value(699)));
        }

        s.v[1228] = if ((-s.v[699]) < (-80.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (!(s.v[1224] != 0.0))) && (!(s.v[1227] != 0.0))) && (s.v[1228] != 0.0)) {
            s.store_div_from_scalar_ad(691, 1.80485e-35, A::offset(A::mul(A::offset(A::neg(A::neg(s.ad_value(699))), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(A::neg(s.ad_value(699))), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(A::neg(s.ad_value(699))), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (!(s.v[1224] != 0.0))) && (!(s.v[1227] != 0.0))) && (!(s.v[1228] != 0.0))) {
            s.store_scale_ad(691, A::offset(A::mul(A::offset(A::neg(s.ad_value(699)), (-80.0)), A::offset(A::mul(A::scale(A::offset(A::neg(s.ad_value(699)), (-80.0)), 0.5), A::offset(A::scale(A::offset(A::neg(s.ad_value(699)), (-80.0)), 0.3333333333333), 1.0)), 1.0)), 1.0), 5.54062e34);
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (!(s.v[1224] != 0.0))) {
            s.store_sub_from_scalar(697, 1.0, 691);
        }

        if (((s.v[1222] != 0.0) && (!(s.v[1223] != 0.0))) && (!(s.v[1224] != 0.0))) {
            s.store_sub_ad(700, A::add(s.ad_value(706), A::scale(s.ad_value(709), 0.5)), A::mul(s.ad_value(708), A::sqrt(A::sub(A::add(s.ad_value(706), A::scale(s.ad_value(709), 0.25)), s.ad_value(697)))));
        }

    }
}
