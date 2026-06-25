#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
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
        s.v[146] = 0.0;

        s.v[147] = 0.0;

        s.v[148] = 0.0;

        s.v[149] = 0.0;

        s.v[273] = 0.0;

        s.v[527] = if (p.p12 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[527] != 0.0) {
            s.store_scalar(212, 1.0);
        }

        if (!(s.v[527] != 0.0)) {
            s.store_scalar(212, (-1.0));
        }

        s.v[528] = if (p.p13 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[528] != 0.0) {
            s.store_scalar(213, 1.0);
        }

        if (!(s.v[528] != 0.0)) {
            s.store_scalar(213, (-1.0));
        }

        s.v[16] = (p.p59 * 8.85418e-12);

        s.v[529] = if (p.p21 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[529] != 0.0) {
            s.store_scalar(5, (p.p1 / p.p2));
        }

        if (!(s.v[529] != 0.0)) {
            s.store_scalar(5, p.p1);
        }

        s.v[0] = (p.p0 + p.p23);

        s.store_offset(5, 5, p.p24);

        s.v[6] = ((s.v[0]) as f64).powf((-p.p29));

        s.store_powf(7, 5, (-p.p30));

        s.store_scale(8, 7, s.v[6]);

        s.store_add_ad(9, A::offset(A::scale(s.ad_value(7), p.p27), (p.p25 + (p.p26 * s.v[6]))), A::scale(s.ad_value(8), p.p28));

        s.v[10] = ((s.v[0]) as f64).powf((-p.p35));

        s.store_powf(11, 5, (-p.p36));

        s.store_scale(12, 11, s.v[10]);

        s.store_add_ad(13, A::offset(A::scale(s.ad_value(11), p.p33), (p.p31 + (p.p32 * s.v[10]))), A::scale(s.ad_value(12), p.p34));

        s.store_sub_from_scalar_ad(2, s.v[0], A::scale(s.ad_value(9), 2.0));

        s.store_sub_ad_rhs(3, 5, A::scale(s.ad_value(13), 2.0));

        s.store_add_ad(14, A::offset(A::scale(s.ad_value(7), p.p39), (p.p37 + (p.p38 * s.v[6]))), A::scale(s.ad_value(8), p.p40));

        s.store_add_ad(15, A::offset(A::scale(s.ad_value(11), p.p43), (p.p41 + (p.p42 * s.v[10]))), A::scale(s.ad_value(12), p.p44));

        s.store_sub_from_scalar_ad(1, s.v[0], A::scale(s.ad_value(14), 2.0));

        s.store_sub_ad_rhs(4, 5, A::scale(s.ad_value(15), 2.0));

        s.store_div_from_scalar(278, 1e-6, 2);

        s.store_div_from_scalar(279, 1e-6, 3);

        s.store_mul(280, 278, 279);

        s.store_add_ad(281, A::add(A::offset(A::scale(s.ad_value(278), p.p319), p.p191), A::scale(s.ad_value(279), p.p320)), A::scale(s.ad_value(280), p.p321));

        s.store_add_ad(282, A::add(A::offset(A::scale(s.ad_value(278), p.p325), p.p199), A::scale(s.ad_value(279), p.p326)), A::scale(s.ad_value(280), p.p327));

        s.store_add_ad(283, A::add(A::offset(A::scale(s.ad_value(278), p.p322), p.p195), A::scale(s.ad_value(279), p.p323)), A::scale(s.ad_value(280), p.p324));

        s.store_add_ad(284, A::add(A::offset(A::scale(s.ad_value(278), p.p328), p.p202), A::scale(s.ad_value(279), p.p329)), A::scale(s.ad_value(280), p.p330));

        s.store_add_ad(285, A::add(A::offset(A::scale(s.ad_value(278), p.p331), p.p203), A::scale(s.ad_value(279), p.p332)), A::scale(s.ad_value(280), p.p333));

        s.store_add_ad(286, A::add(A::offset(A::scale(s.ad_value(278), p.p334), p.p204), A::scale(s.ad_value(279), p.p335)), A::scale(s.ad_value(280), p.p336));

        s.store_add_ad(287, A::add(A::offset(A::scale(s.ad_value(278), p.p337), p.p57), A::scale(s.ad_value(279), p.p338)), A::scale(s.ad_value(280), p.p339));

        s.store_add_ad(288, A::add(A::offset(A::scale(s.ad_value(278), p.p340), p.p58), A::scale(s.ad_value(279), p.p341)), A::scale(s.ad_value(280), p.p342));

        s.store_add_ad(289, A::add(A::offset(A::scale(s.ad_value(278), p.p343), p.p51), A::scale(s.ad_value(279), p.p344)), A::scale(s.ad_value(280), p.p345));

        s.store_add_ad(290, A::add(A::offset(A::scale(s.ad_value(278), p.p346), p.p50), A::scale(s.ad_value(279), p.p347)), A::scale(s.ad_value(280), p.p348));

        s.store_add_ad(291, A::add(A::offset(A::scale(s.ad_value(278), p.p349), p.p63), A::scale(s.ad_value(279), p.p350)), A::scale(s.ad_value(280), p.p351));

        s.store_add_ad(292, A::add(A::offset(A::scale(s.ad_value(278), p.p352), p.p64), A::scale(s.ad_value(279), p.p353)), A::scale(s.ad_value(280), p.p354));

        s.store_add_ad(293, A::add(A::offset(A::scale(s.ad_value(278), p.p355), p.p65), A::scale(s.ad_value(279), p.p356)), A::scale(s.ad_value(280), p.p357));

        s.store_add_ad(294, A::add(A::offset(A::scale(s.ad_value(278), p.p358), p.p68), A::scale(s.ad_value(279), p.p359)), A::scale(s.ad_value(280), p.p360));

        s.store_add_ad(295, A::add(A::offset(A::scale(s.ad_value(278), p.p361), p.p276), A::scale(s.ad_value(279), p.p362)), A::scale(s.ad_value(280), p.p363));

        s.store_add_ad(250, A::add(A::offset(A::scale(s.ad_value(278), p.p751), p.p291), A::scale(s.ad_value(279), p.p752)), A::scale(s.ad_value(280), p.p753));

        s.store_add_ad(252, A::add(A::offset(A::scale(s.ad_value(278), p.p757), p.p294), A::scale(s.ad_value(279), p.p758)), A::scale(s.ad_value(280), p.p759));

        s.store_add_ad(251, A::add(A::offset(A::scale(s.ad_value(278), p.p754), p.p293), A::scale(s.ad_value(279), p.p755)), A::scale(s.ad_value(280), p.p756));

        s.v[538] = if (s.v[295] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[538] != 0.0) {
            s.store_scalar(295, 0.0);
        }

        s.v[539] = if (s.v[295] > 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[538] != 0.0)) && (s.v[539] != 0.0)) {
            s.store_scalar(295, 1.0);
        }

        s.store_add_ad(296, A::add(A::offset(A::scale(s.ad_value(278), p.p364), p.p277), A::scale(s.ad_value(279), p.p365)), A::scale(s.ad_value(280), p.p366));

        s.store_add_ad(297, A::add(A::offset(A::scale(s.ad_value(278), p.p367), p.p278), A::scale(s.ad_value(279), p.p368)), A::scale(s.ad_value(280), p.p369));

        s.store_add_ad(298, A::add(A::offset(A::scale(s.ad_value(278), p.p370), p.p275), A::scale(s.ad_value(279), p.p371)), A::scale(s.ad_value(280), p.p372));

        s.store_add_ad(299, A::add(A::offset(A::scale(s.ad_value(278), p.p373), p.p272), A::scale(s.ad_value(279), p.p374)), A::scale(s.ad_value(280), p.p375));

        s.store_add_ad(300, A::add(A::offset(A::scale(s.ad_value(278), p.p376), p.p273), A::scale(s.ad_value(279), p.p377)), A::scale(s.ad_value(280), p.p378));

        s.store_add_ad(301, A::add(A::offset(A::scale(s.ad_value(278), p.p379), p.p274), A::scale(s.ad_value(279), p.p380)), A::scale(s.ad_value(280), p.p381));

        s.store_add_ad(302, A::add(A::offset(A::scale(s.ad_value(278), p.p382), p.p283), A::scale(s.ad_value(279), p.p383)), A::scale(s.ad_value(280), p.p384));

        s.v[540] = if (s.v[302] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[540] != 0.0) {
            s.store_scalar(302, 0.0);
        }

        s.v[541] = if (s.v[302] > 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[540] != 0.0)) && (s.v[541] != 0.0)) {
            s.store_scalar(302, 1.0);
        }

        s.store_add_ad(303, A::add(A::offset(A::scale(s.ad_value(278), p.p385), p.p284), A::scale(s.ad_value(279), p.p386)), A::scale(s.ad_value(280), p.p387));

        s.store_add_ad(304, A::add(A::offset(A::scale(s.ad_value(278), p.p388), p.p285), A::scale(s.ad_value(279), p.p389)), A::scale(s.ad_value(280), p.p390));

        s.store_add_ad(305, A::add(A::offset(A::scale(s.ad_value(278), p.p391), p.p282), A::scale(s.ad_value(279), p.p392)), A::scale(s.ad_value(280), p.p393));

        s.store_add_ad(306, A::add(A::offset(A::scale(s.ad_value(278), p.p394), p.p279), A::scale(s.ad_value(279), p.p395)), A::scale(s.ad_value(280), p.p396));

        s.store_add_ad(307, A::add(A::offset(A::scale(s.ad_value(278), p.p397), p.p280), A::scale(s.ad_value(279), p.p398)), A::scale(s.ad_value(280), p.p399));

        s.store_add_ad(308, A::add(A::offset(A::scale(s.ad_value(278), p.p400), p.p281), A::scale(s.ad_value(279), p.p401)), A::scale(s.ad_value(280), p.p402));

        s.store_add_ad(313, A::add(A::offset(A::scale(s.ad_value(278), p.p403), p.p71), A::scale(s.ad_value(279), p.p404)), A::scale(s.ad_value(280), p.p405));

        s.store_add_ad(314, A::add(A::offset(A::scale(s.ad_value(278), p.p406), p.p72), A::scale(s.ad_value(279), p.p407)), A::scale(s.ad_value(280), p.p408));

        s.store_add_ad(315, A::add(A::offset(A::scale(s.ad_value(278), p.p409), p.p73), A::scale(s.ad_value(279), p.p410)), A::scale(s.ad_value(280), p.p411));

        s.store_add_ad(316, A::add(A::offset(A::scale(s.ad_value(278), p.p412), p.p74), A::scale(s.ad_value(279), p.p413)), A::scale(s.ad_value(280), p.p414));

        s.store_add_ad(317, A::add(A::offset(A::scale(s.ad_value(278), p.p415), p.p75), A::scale(s.ad_value(279), p.p416)), A::scale(s.ad_value(280), p.p417));

        s.store_add_ad(318, A::add(A::offset(A::scale(s.ad_value(278), p.p418), p.p84), A::scale(s.ad_value(279), p.p419)), A::scale(s.ad_value(280), p.p420));

        s.store_add_ad(319, A::add(A::offset(A::scale(s.ad_value(278), p.p421), p.p76), A::scale(s.ad_value(279), p.p422)), A::scale(s.ad_value(280), p.p423));

        s.store_add_ad(309, A::add(A::offset(A::scale(s.ad_value(278), p.p430), p.p87), A::scale(s.ad_value(279), p.p431)), A::scale(s.ad_value(280), p.p432));

        s.store_add_ad(310, A::add(A::offset(A::scale(s.ad_value(278), p.p433), p.p88), A::scale(s.ad_value(279), p.p434)), A::scale(s.ad_value(280), p.p435));

        s.store_add_ad(311, A::add(A::offset(A::scale(s.ad_value(278), p.p436), p.p61), A::scale(s.ad_value(279), p.p437)), A::scale(s.ad_value(280), p.p438));

        s.store_add_ad(312, A::add(A::offset(A::scale(s.ad_value(278), p.p439), p.p62), A::scale(s.ad_value(279), p.p440)), A::scale(s.ad_value(280), p.p441));

        s.store_add_ad(320, A::add(A::offset(A::scale(s.ad_value(278), p.p424), p.p85), A::scale(s.ad_value(279), p.p425)), A::scale(s.ad_value(280), p.p426));

        s.store_add_ad(321, A::add(A::offset(A::scale(s.ad_value(278), p.p427), p.p86), A::scale(s.ad_value(279), p.p428)), A::scale(s.ad_value(280), p.p429));

        s.store_add_ad(326, A::add(A::offset(A::scale(s.ad_value(278), p.p460), p.p113), A::scale(s.ad_value(279), p.p461)), A::scale(s.ad_value(280), p.p462));

        s.store_add_ad(322, A::add(A::offset(A::scale(s.ad_value(278), p.p442), p.p89), A::scale(s.ad_value(279), p.p443)), A::scale(s.ad_value(280), p.p444));

        s.store_add_ad(323, A::add(A::offset(A::scale(s.ad_value(278), p.p445), p.p90), A::scale(s.ad_value(279), p.p446)), A::scale(s.ad_value(280), p.p447));

        s.store_add_ad(324, A::add(A::offset(A::scale(s.ad_value(278), p.p448), p.p91), A::scale(s.ad_value(279), p.p449)), A::scale(s.ad_value(280), p.p450));

        s.store_add_ad(325, A::add(A::offset(A::scale(s.ad_value(278), p.p451), p.p92), A::scale(s.ad_value(279), p.p452)), A::scale(s.ad_value(280), p.p453));

        s.store_add_ad(417, A::add(A::offset(A::scale(s.ad_value(278), p.p454), p.p93), A::scale(s.ad_value(279), p.p455)), A::scale(s.ad_value(280), p.p456));

        s.store_add_ad(418, A::add(A::offset(A::scale(s.ad_value(278), p.p457), p.p94), A::scale(s.ad_value(279), p.p458)), A::scale(s.ad_value(280), p.p459));

        s.store_add_ad(327, A::add(A::offset(A::scale(s.ad_value(278), p.p463), p.p116), A::scale(s.ad_value(279), p.p464)), A::scale(s.ad_value(280), p.p465));

        s.store_add_ad(328, A::add(A::offset(A::scale(s.ad_value(278), p.p466), p.p123), A::scale(s.ad_value(279), p.p467)), A::scale(s.ad_value(280), p.p468));

        s.store_add_ad(329, A::add(A::offset(A::scale(s.ad_value(278), p.p469), p.p124), A::scale(s.ad_value(279), p.p470)), A::scale(s.ad_value(280), p.p471));

        s.store_add_ad(330, A::add(A::offset(A::scale(s.ad_value(278), p.p472), p.p122), A::scale(s.ad_value(279), p.p473)), A::scale(s.ad_value(280), p.p474));

        s.store_add_ad(331, A::add(A::offset(A::scale(s.ad_value(278), p.p475), p.p135), A::scale(s.ad_value(279), p.p476)), A::scale(s.ad_value(280), p.p477));

        s.store_add_ad(332, A::add(A::offset(A::scale(s.ad_value(278), p.p478), p.p139), A::scale(s.ad_value(279), p.p479)), A::scale(s.ad_value(280), p.p480));

        s.store_add_ad(333, A::add(A::offset(A::scale(s.ad_value(278), p.p481), p.p145), A::scale(s.ad_value(279), p.p482)), A::scale(s.ad_value(280), p.p483));

        s.store_add_ad(334, A::add(A::offset(A::scale(s.ad_value(278), p.p484), p.p148), A::scale(s.ad_value(279), p.p485)), A::scale(s.ad_value(280), p.p486));

        s.store_add_ad(335, A::add(A::offset(A::scale(s.ad_value(278), p.p487), p.p155), A::scale(s.ad_value(279), p.p488)), A::scale(s.ad_value(280), p.p489));

        s.store_add_ad(336, A::add(A::offset(A::scale(s.ad_value(278), p.p490), p.p142), A::scale(s.ad_value(279), p.p491)), A::scale(s.ad_value(280), p.p492));

        s.store_add_ad(342, A::add(A::offset(A::scale(s.ad_value(278), p.p493), p.p163), A::scale(s.ad_value(279), p.p494)), A::scale(s.ad_value(280), p.p495));

        s.store_add_ad(337, A::add(A::offset(A::scale(s.ad_value(278), p.p496), p.p157), A::scale(s.ad_value(279), p.p497)), A::scale(s.ad_value(280), p.p498));

        s.store_add_ad(338, A::add(A::offset(A::scale(s.ad_value(278), p.p499), p.p156), A::scale(s.ad_value(279), p.p500)), A::scale(s.ad_value(280), p.p501));

        s.store_add_ad(339, A::add(A::offset(A::scale(s.ad_value(278), p.p502), p.p158), A::scale(s.ad_value(279), p.p503)), A::scale(s.ad_value(280), p.p504));

        s.store_add_ad(340, A::add(A::offset(A::scale(s.ad_value(278), p.p505), p.p160), A::scale(s.ad_value(279), p.p506)), A::scale(s.ad_value(280), p.p507));

        s.store_add_ad(341, A::add(A::offset(A::scale(s.ad_value(278), p.p508), p.p161), A::scale(s.ad_value(279), p.p509)), A::scale(s.ad_value(280), p.p510));

        s.store_add_ad(343, A::add(A::offset(A::scale(s.ad_value(278), p.p511), p.p136), A::scale(s.ad_value(279), p.p512)), A::scale(s.ad_value(280), p.p513));

        s.store_add_ad(344, A::add(A::offset(A::scale(s.ad_value(278), p.p514), p.p166), A::scale(s.ad_value(279), p.p515)), A::scale(s.ad_value(280), p.p516));

        s.store_add_ad(345, A::add(A::offset(A::scale(s.ad_value(278), p.p517), p.p167), A::scale(s.ad_value(279), p.p518)), A::scale(s.ad_value(280), p.p519));

        s.store_add_ad(346, A::add(A::offset(A::scale(s.ad_value(278), p.p520), p.p173), A::scale(s.ad_value(279), p.p521)), A::scale(s.ad_value(280), p.p522));

        s.store_add_ad(347, A::add(A::offset(A::scale(s.ad_value(278), p.p523), p.p176), A::scale(s.ad_value(279), p.p524)), A::scale(s.ad_value(280), p.p525));

        s.store_add_ad(348, A::add(A::offset(A::scale(s.ad_value(278), p.p526), p.p182), A::scale(s.ad_value(279), p.p527)), A::scale(s.ad_value(280), p.p528));

        s.store_add_ad(349, A::add(A::offset(A::scale(s.ad_value(278), p.p529), p.p170), A::scale(s.ad_value(279), p.p530)), A::scale(s.ad_value(280), p.p531));

        s.store_add_ad(350, A::add(A::offset(A::scale(s.ad_value(278), p.p532), p.p183), A::scale(s.ad_value(279), p.p533)), A::scale(s.ad_value(280), p.p534));

        s.store_add_ad(351, A::add(A::offset(A::scale(s.ad_value(278), p.p535), p.p186), A::scale(s.ad_value(279), p.p536)), A::scale(s.ad_value(280), p.p537));

        s.store_add_ad(353, A::add(A::offset(A::scale(s.ad_value(278), p.p538), p.p119), A::scale(s.ad_value(279), p.p539)), A::scale(s.ad_value(280), p.p540));

        s.store_add_ad(354, A::add(A::offset(A::scale(s.ad_value(278), p.p541), p.p130), A::scale(s.ad_value(279), p.p542)), A::scale(s.ad_value(280), p.p543));

        s.store_add_ad(355, A::add(A::offset(A::scale(s.ad_value(278), p.p544), p.p205), A::scale(s.ad_value(279), p.p545)), A::scale(s.ad_value(280), p.p546));

        s.store_add_ad(356, A::add(A::offset(A::scale(s.ad_value(278), p.p547), p.p305), A::scale(s.ad_value(279), p.p548)), A::scale(s.ad_value(280), p.p549));

        s.store_add_ad(357, A::add(A::offset(A::scale(s.ad_value(278), p.p550), p.p306), A::scale(s.ad_value(279), p.p551)), A::scale(s.ad_value(280), p.p552));

        s.store_add_ad(358, A::add(A::offset(A::scale(s.ad_value(278), p.p553), p.p307), A::scale(s.ad_value(279), p.p554)), A::scale(s.ad_value(280), p.p555));

        s.store_add_ad(359, A::add(A::offset(A::scale(s.ad_value(278), p.p556), p.p308), A::scale(s.ad_value(279), p.p557)), A::scale(s.ad_value(280), p.p558));

        s.store_add_ad(360, A::add(A::offset(A::scale(s.ad_value(278), p.p559), p.p210), A::scale(s.ad_value(279), p.p560)), A::scale(s.ad_value(280), p.p561));

        s.store_add_ad(361, A::add(A::offset(A::scale(s.ad_value(278), p.p562), p.p214), A::scale(s.ad_value(279), p.p563)), A::scale(s.ad_value(280), p.p564));

        s.store_add_ad(362, A::add(A::offset(A::scale(s.ad_value(278), p.p565), p.p208), A::scale(s.ad_value(279), p.p566)), A::scale(s.ad_value(280), p.p567));

        s.store_add_ad(363, A::add(A::offset(A::scale(s.ad_value(278), p.p568), p.p206), A::scale(s.ad_value(279), p.p569)), A::scale(s.ad_value(280), p.p570));

        s.store_add_ad(364, A::add(A::offset(A::scale(s.ad_value(278), p.p571), p.p207), A::scale(s.ad_value(279), p.p572)), A::scale(s.ad_value(280), p.p573));

        s.store_add_ad(365, A::add(A::offset(A::scale(s.ad_value(278), p.p574), p.p209), A::scale(s.ad_value(279), p.p575)), A::scale(s.ad_value(280), p.p576));

        s.store_add_ad(366, A::add(A::offset(A::scale(s.ad_value(278), p.p577), p.p256), A::scale(s.ad_value(279), p.p578)), A::scale(s.ad_value(280), p.p579));

        s.store_add_ad(367, A::add(A::offset(A::scale(s.ad_value(278), p.p580), p.p257), A::scale(s.ad_value(279), p.p581)), A::scale(s.ad_value(280), p.p582));

        s.store_add_ad(368, A::add(A::offset(A::scale(s.ad_value(278), p.p583), p.p258), A::scale(s.ad_value(279), p.p584)), A::scale(s.ad_value(280), p.p585));

        s.store_add_ad(408, A::add(A::offset(A::scale(s.ad_value(278), p.p706), p.p217), A::scale(s.ad_value(279), p.p707)), A::scale(s.ad_value(280), p.p708));

        s.store_add_ad(409, A::add(A::offset(A::scale(s.ad_value(278), p.p709), p.p218), A::scale(s.ad_value(279), p.p710)), A::scale(s.ad_value(280), p.p711));

        s.store_add_ad(410, A::add(A::offset(A::scale(s.ad_value(278), p.p712), p.p219), A::scale(s.ad_value(279), p.p713)), A::scale(s.ad_value(280), p.p714));

        s.store_add_ad(411, A::add(A::offset(A::scale(s.ad_value(278), p.p715), p.p220), A::scale(s.ad_value(279), p.p716)), A::scale(s.ad_value(280), p.p717));

        s.store_add_ad(412, A::add(A::offset(A::scale(s.ad_value(278), p.p718), p.p221), A::scale(s.ad_value(279), p.p719)), A::scale(s.ad_value(280), p.p720));

        s.store_add_ad(413, A::add(A::offset(A::scale(s.ad_value(278), p.p721), p.p222), A::scale(s.ad_value(279), p.p722)), A::scale(s.ad_value(280), p.p723));

        s.store_add_ad(414, A::add(A::offset(A::scale(s.ad_value(278), p.p724), p.p223), A::scale(s.ad_value(279), p.p725)), A::scale(s.ad_value(280), p.p726));

        s.store_add_ad(415, A::add(A::offset(A::scale(s.ad_value(278), p.p727), p.p224), A::scale(s.ad_value(279), p.p728)), A::scale(s.ad_value(280), p.p729));

        s.store_add_ad(416, A::add(A::offset(A::scale(s.ad_value(278), p.p730), p.p225), A::scale(s.ad_value(279), p.p731)), A::scale(s.ad_value(280), p.p732));

        s.store_add_ad(369, A::add(A::offset(A::scale(s.ad_value(278), p.p586), p.p226), A::scale(s.ad_value(279), p.p587)), A::scale(s.ad_value(280), p.p588));

        s.store_add_ad(370, A::add(A::offset(A::scale(s.ad_value(278), p.p589), p.p227), A::scale(s.ad_value(279), p.p590)), A::scale(s.ad_value(280), p.p591));

        s.store_add_ad(371, A::add(A::offset(A::scale(s.ad_value(278), p.p592), p.p228), A::scale(s.ad_value(279), p.p593)), A::scale(s.ad_value(280), p.p594));

        s.store_add_ad(373, A::add(A::offset(A::scale(s.ad_value(278), p.p595), p.p230), A::scale(s.ad_value(279), p.p596)), A::scale(s.ad_value(280), p.p597));

        s.store_add_ad(372, A::add(A::offset(A::scale(s.ad_value(278), p.p598), p.p229), A::scale(s.ad_value(279), p.p599)), A::scale(s.ad_value(280), p.p600));

        s.store_add_ad(381, A::add(A::offset(A::scale(s.ad_value(278), p.p610), p.p247), A::scale(s.ad_value(279), p.p611)), A::scale(s.ad_value(280), p.p612));

        s.store_add_ad(374, A::add(A::offset(A::scale(s.ad_value(278), p.p619), p.p250), A::scale(s.ad_value(279), p.p620)), A::scale(s.ad_value(280), p.p621));

        s.store_add_ad(375, A::add(A::offset(A::scale(s.ad_value(278), p.p622), p.p251), A::scale(s.ad_value(279), p.p623)), A::scale(s.ad_value(280), p.p624));

        s.store_add_ad(376, A::add(A::offset(A::scale(s.ad_value(278), p.p625), p.p252), A::scale(s.ad_value(279), p.p626)), A::scale(s.ad_value(280), p.p627));

        s.store_add_ad(377, A::add(A::offset(A::scale(s.ad_value(278), p.p628), p.p253), A::scale(s.ad_value(279), p.p629)), A::scale(s.ad_value(280), p.p630));

        s.store_add_ad(378, A::add(A::offset(A::scale(s.ad_value(278), p.p601), p.p244), A::scale(s.ad_value(279), p.p602)), A::scale(s.ad_value(280), p.p603));

        s.store_add_ad(379, A::add(A::offset(A::scale(s.ad_value(278), p.p604), p.p245), A::scale(s.ad_value(279), p.p605)), A::scale(s.ad_value(280), p.p606));

        s.store_add_ad(380, A::add(A::offset(A::scale(s.ad_value(278), p.p607), p.p246), A::scale(s.ad_value(279), p.p608)), A::scale(s.ad_value(280), p.p609));

        s.store_add_ad(390, A::add(A::offset(A::scale(s.ad_value(278), p.p613), p.p248), A::scale(s.ad_value(279), p.p614)), A::scale(s.ad_value(280), p.p615));

        s.store_add_ad(392, A::add(A::offset(A::scale(s.ad_value(278), p.p631), p.p254), A::scale(s.ad_value(279), p.p632)), A::scale(s.ad_value(280), p.p633));

        s.store_add_ad(391, A::add(A::offset(A::scale(s.ad_value(278), p.p616), p.p249), A::scale(s.ad_value(279), p.p617)), A::scale(s.ad_value(280), p.p618));

        s.store_add_ad(393, A::add(A::offset(A::scale(s.ad_value(278), p.p634), p.p255), A::scale(s.ad_value(279), p.p635)), A::scale(s.ad_value(280), p.p636));

        s.store_add_ad(382, A::add(A::offset(A::scale(s.ad_value(278), p.p637), p.p231), A::scale(s.ad_value(279), p.p638)), A::scale(s.ad_value(280), p.p639));

        s.store_add_ad(383, A::add(A::offset(A::scale(s.ad_value(278), p.p643), p.p232), A::scale(s.ad_value(279), p.p644)), A::scale(s.ad_value(280), p.p645));

        s.store_add_ad(384, A::add(A::offset(A::scale(s.ad_value(278), p.p649), p.p233), A::scale(s.ad_value(279), p.p650)), A::scale(s.ad_value(280), p.p651));

        s.store_add_ad(385, A::add(A::offset(A::scale(s.ad_value(278), p.p655), p.p242), A::scale(s.ad_value(279), p.p656)), A::scale(s.ad_value(280), p.p657));

        s.store_add_ad(386, A::add(A::offset(A::scale(s.ad_value(278), p.p640), p.p236), A::scale(s.ad_value(279), p.p641)), A::scale(s.ad_value(280), p.p642));

        s.store_add_ad(387, A::add(A::offset(A::scale(s.ad_value(278), p.p646), p.p237), A::scale(s.ad_value(279), p.p647)), A::scale(s.ad_value(280), p.p648));

        s.store_add_ad(388, A::add(A::offset(A::scale(s.ad_value(278), p.p652), p.p238), A::scale(s.ad_value(279), p.p653)), A::scale(s.ad_value(280), p.p654));

        s.store_add_ad(389, A::add(A::offset(A::scale(s.ad_value(278), p.p658), p.p243), A::scale(s.ad_value(279), p.p659)), A::scale(s.ad_value(280), p.p660));

        s.store_add_ad(395, A::add(A::offset(A::scale(s.ad_value(278), p.p661), p.p240), A::scale(s.ad_value(279), p.p662)), A::scale(s.ad_value(280), p.p663));

        s.store_add_ad(394, A::add(A::offset(A::scale(s.ad_value(278), p.p664), p.p241), A::scale(s.ad_value(279), p.p665)), A::scale(s.ad_value(280), p.p666));

        s.store_add_ad(396, A::add(A::offset(A::scale(s.ad_value(278), p.p667), p.p259), A::scale(s.ad_value(279), p.p668)), A::scale(s.ad_value(280), p.p669));

        s.store_add_ad(397, A::add(A::offset(A::scale(s.ad_value(278), p.p670), p.p260), A::scale(s.ad_value(279), p.p671)), A::scale(s.ad_value(280), p.p672));

        s.store_add_ad(398, A::add(A::offset(A::scale(s.ad_value(278), p.p673), p.p261), A::scale(s.ad_value(279), p.p674)), A::scale(s.ad_value(280), p.p675));

        s.store_add_ad(399, A::add(A::offset(A::scale(s.ad_value(278), p.p676), p.p262), A::scale(s.ad_value(279), p.p677)), A::scale(s.ad_value(280), p.p678));

        s.store_add_ad(400, A::add(A::offset(A::scale(s.ad_value(278), p.p679), p.p100), A::scale(s.ad_value(279), p.p680)), A::scale(s.ad_value(280), p.p681));

        s.store_add_ad(401, A::add(A::offset(A::scale(s.ad_value(278), p.p682), p.p129), A::scale(s.ad_value(279), p.p683)), A::scale(s.ad_value(280), p.p684));

        s.store_add_ad(402, A::add(A::offset(A::scale(s.ad_value(278), p.p685), p.p103), A::scale(s.ad_value(279), p.p686)), A::scale(s.ad_value(280), p.p687));

        s.store_add_ad(403, A::add(A::offset(A::scale(s.ad_value(278), p.p688), p.p106), A::scale(s.ad_value(279), p.p689)), A::scale(s.ad_value(280), p.p690));

        s.store_add_ad(404, A::add(A::offset(A::scale(s.ad_value(278), p.p691), p.p110), A::scale(s.ad_value(279), p.p692)), A::scale(s.ad_value(280), p.p693));

        s.store_add_ad(405, A::add(A::offset(A::scale(s.ad_value(278), p.p694), p.p111), A::scale(s.ad_value(279), p.p695)), A::scale(s.ad_value(280), p.p696));

        s.store_add_ad(407, A::add(A::offset(A::scale(s.ad_value(278), p.p697), p.p112), A::scale(s.ad_value(279), p.p698)), A::scale(s.ad_value(280), p.p699));

        s.store_add_ad(406, A::add(A::offset(A::scale(s.ad_value(278), p.p700), p.p137), A::scale(s.ad_value(279), p.p701)), A::scale(s.ad_value(280), p.p702));

        s.store_add_ad(352, A::add(A::offset(A::scale(s.ad_value(278), p.p703), p.p187), A::scale(s.ad_value(279), p.p704)), A::scale(s.ad_value(280), p.p705));

        s.store_add_ad(62, A::add(A::offset(A::scale(s.ad_value(278), p.p739), p.p95), A::scale(s.ad_value(279), p.p740)), A::scale(s.ad_value(280), p.p741));

        s.store_add_ad(66, A::add(A::offset(A::scale(s.ad_value(278), p.p742), p.p96), A::scale(s.ad_value(279), p.p743)), A::scale(s.ad_value(280), p.p744));

        s.store_add_ad(67, A::add(A::offset(A::scale(s.ad_value(278), p.p745), p.p97), A::scale(s.ad_value(279), p.p746)), A::scale(s.ad_value(280), p.p747));

        s.store_add_ad(68, A::add(A::offset(A::scale(s.ad_value(278), p.p748), p.p98), A::scale(s.ad_value(279), p.p749)), A::scale(s.ad_value(280), p.p750));

        s.v[542] = if ((p.p20 == 1.0) && (p.p317 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[542] != 0.0) {
            s.store_add_ad(275, A::add(A::offset(A::scale(s.ad_value(278), p.p733), p.p317), A::scale(s.ad_value(279), p.p734)), A::scale(s.ad_value(280), p.p735));
        }

        if (s.v[542] != 0.0) {
            s.store_add_ad(276, A::add(A::offset(A::scale(s.ad_value(278), p.p736), p.p318), A::scale(s.ad_value(279), p.p737)), A::scale(s.ad_value(280), p.p738));
        }

        if (!(s.v[542] != 0.0)) {
            s.store_scalar(275, 0.0);
        }

        if (!(s.v[542] != 0.0)) {
            s.store_scalar(276, 0.0);
        }

        s.v[17] = ((3.9 * 8.85418e-12) / p.p45);

        s.v[18] = ((3.9 * 8.85418e-12) / p.p47);

        s.v[19] = ((3.9 * 8.85418e-12) / p.p46);

        s.v[20] = (s.v[16] / p.p49);

        s.v[21] = (p.p59 / 3.9);

        s.v[543] = if !(if self.param_given[47] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if (s.v[543] != 0.0) {
            s.store_scalar(221, (((p.p45 * p.p60) / 3.9) - p.p48));
        }

        if (!(s.v[543] != 0.0)) {
            s.store_scalar(221, p.p47);
        }

        s.v[544] = if (p.p138 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[544] != 0.0) {
            s.store_mul_ad_rhs(331, 331, A::sub_from_scalar(1.0, A::mul(s.ad_value(406), A::powf(s.ad_value(2), (-p.p138)))));
        }

        if (!(s.v[544] != 0.0)) {
            s.store_mul_ad_rhs(331, 331, A::sub_from_scalar(1.0, s.ad_value(406)));
        }

        s.store_add_ad_rhs(332, 332, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p141))), p.p140));

        s.store_add_ad_rhs(333, 333, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p147))), p.p146));

        s.store_offset_ad(137, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p153))), p.p152), p.p151);

        s.store_add_ad_rhs(334, 334, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p150))), p.p149));

        s.store_add_ad_rhs(336, 336, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p144))), p.p143));

        s.store_add_ad_rhs(342, 342, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p165))), p.p164));

        s.v[545] = if (p.p188 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[545] != 0.0) {
            s.store_mul_ad_rhs(344, 344, A::sub_from_scalar(1.0, A::mul(s.ad_value(352), A::powf(s.ad_value(2), (-p.p188)))));
        }

        if (!(s.v[545] != 0.0)) {
            s.store_mul_ad_rhs(344, 344, A::sub_from_scalar(1.0, s.ad_value(352)));
        }

        s.store_add_ad_rhs(345, 345, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p169))), p.p168));

        s.store_add_ad_rhs(346, 346, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p175))), p.p174));

        s.store_offset_ad(138, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p181))), p.p180), p.p179);

        s.store_add_ad_rhs(347, 347, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p178))), p.p177));

        s.store_add_ad_rhs(349, 349, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p172))), p.p171));

        s.store_add_ad_rhs(350, 350, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p185))), p.p184));

        s.v[546] = if (p.p14 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[546] != 0.0) {
            s.store_add_ad_rhs(283, 283, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p197))), p.p196));
        }

        if (s.v[546] != 0.0) {
            s.store_add_ad_rhs(282, 282, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p201))), p.p200));
        }

        if (!(s.v[546] != 0.0)) {
            s.store_add_ad_rhs(281, 281, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p193))), p.p192));
        }

        s.store_add_ad_rhs(360, 360, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p212))), p.p211));

        s.store_add_ad_rhs(326, 326, A::scale(A::powf(A::scale(s.ad_value(2), 1000000.0), (-p.p115)), p.p114));

        s.store_add_ad_rhs(327, 327, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p118))), p.p117));

        s.store_add_ad_rhs(328, 328, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p126))), p.p125));

        s.store_add_ad_rhs(329, 329, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p128))), p.p127));

        s.store_add_ad_rhs(400, 400, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p102))), p.p101));

        s.store_add_ad_rhs(401, 401, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p133))), p.p132));

        s.store_add_ad_rhs(402, 402, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p105))), p.p104));

        s.store_add_ad_rhs(403, 403, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p108))), p.p107));

        s.store_offset_ad(92, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p80))), p.p79), p.p77);

        s.store_offset_ad(93, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p82))), p.p81), p.p78);

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
        s.v[547] = if (s.v[331] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[547] != 0.0) {
            s.store_scalar(331, 0.03);
        }

        s.v[548] = if (s.v[332] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[548] != 0.0) {
            s.store_scalar(332, 0.0);
        }

        s.v[549] = if (s.v[336] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[549] != 0.0) {
            s.store_scalar(336, 0.0);
        }

        s.v[550] = if (s.v[334] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[550] != 0.0) {
            s.store_scalar(334, 0.0);
        }

        s.v[551] = if (s.v[335] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[551] != 0.0) {
            s.store_scalar(335, 0.0);
        }

        s.v[552] = if (s.v[401] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[552] != 0.0) {
            s.store_scalar(401, 0.0);
        }

        s.v[134] = p.p190;

        s.v[555] = if (s.v[134] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[555] != 0.0) {
            s.store_scalar(134, 0.0);
        }

        s.v[556] = if (s.v[281] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[556] != 0.0) {
            s.store_scalar(281, 0.0);
        }

        s.v[136] = p.p194;

        s.v[557] = if (s.v[136] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[557] != 0.0) {
            s.store_scalar(136, 0.0);
        }

        s.v[558] = if (s.v[283] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[558] != 0.0) {
            s.store_scalar(283, 0.0);
        }

        s.v[135] = p.p198;

        s.v[559] = if (s.v[135] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[559] != 0.0) {
            s.store_scalar(135, 0.0);
        }

        s.v[560] = if (s.v[282] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[560] != 0.0) {
            s.store_scalar(282, 0.0);
        }

        s.v[561] = if (s.v[284] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[561] != 0.0) {
            s.store_scalar(284, 0.0);
        }

        s.v[565] = if (s.v[326] < 2.0) { 1.0 } else { 0.0 };

        if (s.v[565] != 0.0) {
            s.store_scalar(326, 2.0);
        }

        s.store_offset_ad(89, A::sqrt(A::offset(A::div(s.ad_value(321), s.ad_value(2)), 1.0)), (-1.0));

        s.v[78] = (p.p49 + (s.v[21] * (p.p45 + p.p46)));

        s.store_div_from_scalar(163, 1.0, 326);

        s.v[236] = (s.v[19] * p.p3);

        s.v[237] = (s.v[19] * p.p4);

        s.v[34] = (p.p267 * ((((1.0 + (p.p49 / p.p46))).max(1e-38)) as f64).ln());

        s.v[236] = (s.v[236] + (s.v[34] * ((p.p5 - p.p1)).max(0.0)));

        s.v[237] = (s.v[237] + (s.v[34] * ((p.p6 - p.p1)).max(0.0)));

        s.v[236] = (s.v[236]).max(1e-20);

        s.v[237] = (s.v[237]).max(1e-20);

        s.store_scale(114, 343, 0.5);

        s.v[115] = 0.5;

        s.store_scale(143, 351, 0.5);

        s.v[566] = if (p.p12 != 1.0) { 1.0 } else { 0.0 };

        if (s.v[566] != 0.0) {
            s.store_scale(114, 343, (1.0 / 3.0));
        }

        if (s.v[566] != 0.0) {
            s.store_scalar(115, (1.0 / 3.0));
        }

        if (s.v[566] != 0.0) {
            s.store_scale(143, 351, (1.0 / 3.0));
        }

        s.v[129] = (1e-8 / (s.v[21] * p.p45));

        s.store_div_from_scalar_ad(131, 1.0, A::scale(A::pow(A::scale(s.ad_value(3), 1000000.0), s.ad_value(286)), p.p2));

        s.v[253] = ((((s.v[21] * p.p45) * p.p49)) as f64).sqrt();

        s.v[144] = (1e-8 / (s.v[21] * p.p46));

        s.v[567] = if (p.p296 >= (s.v[2] / 2.0)) { 1.0 } else { 0.0 };

        if (s.v[567] != 0.0) {
            s.store_scalar(249, 0.0);
        }

        if (!(s.v[567] != 0.0)) {
            s.store_scalar(249, p.p296);
        }

        s.v[568] = if ((p.p18 != 0.0) && (p.p310 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[568] != 0.0) {
            s.store_scale_ad(269, A::offset(A::scale(s.ad_value(3), p.p2), p.p312), 1.0 / (p.p310));
        }

        if (s.v[568] != 0.0) {
            s.store_scale_ad(270, A::offset(A::scale(s.ad_value(3), p.p2), p.p312), p.p311);
        }

        if (!(s.v[568] != 0.0)) {
            s.store_scalar(269, 1.0);
        }

        if (!(s.v[568] != 0.0)) {
            s.store_scalar(270, 0.0);
        }

        s.v[132] = (p.p215 * p.p7);

        s.v[133] = (p.p216 * p.p8);

        s.v[569] = if (s.v[132] <= 0.001) { 1.0 } else { 0.0 };

        if (s.v[569] != 0.0) {
            s.store_scalar(132, 0.001);
        }

        s.v[570] = if (s.v[133] <= 0.001) { 1.0 } else { 0.0 };

        if (s.v[570] != 0.0) {
            s.store_scalar(133, 0.001);
        }

        s.v[571] = if (p.p14 == 1.0) { 1.0 } else { 0.0 };

        s.v[572] = if (s.v[136] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[571] != 0.0) && (s.v[572] != 0.0)) {
            s.store_scalar(136, 0.0);
        }

        s.v[573] = if (s.v[135] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[571] != 0.0) && (s.v[573] != 0.0)) {
            s.store_scalar(135, 0.0);
        }

        s.v[574] = if (s.v[283] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[571] != 0.0) && (s.v[574] != 0.0)) {
            s.store_scalar(283, 0.0);
        }

        s.v[575] = if (s.v[282] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[571] != 0.0) && (s.v[575] != 0.0)) {
            s.store_scalar(282, 0.0);
        }

        s.v[576] = if (s.v[134] <= 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[571] != 0.0)) && (s.v[576] != 0.0)) {
            s.store_scalar(134, 0.0);
        }

        s.v[577] = if (s.v[281] <= 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[571] != 0.0)) && (s.v[577] != 0.0)) {
            s.store_scalar(281, 0.0);
        }

        s.v[578] = if (p.p297 <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[578] != 0.0) {
            s.store_scalar(95, 300.15);
        }

        if (!(s.v[578] != 0.0)) {
            s.store_scalar(95, (p.p297 + 273.15));
        }

        s.v[579] = if (p.p12 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[579] != 0.0) {
            s.store_scalar(205, 4.97232e-7);
        }

        if (!(s.v[579] != 0.0)) {
            s.store_scalar(205, 3.42537e-7);
        }

        s.v[580] = if (p.p12 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[580] != 0.0) {
            s.store_scalar(206, 745669000000.0);
        }

        if (!(s.v[580] != 0.0)) {
            s.store_scalar(206, 1166450000000.0);
        }

        s.v[34] = (p.p99 * p.p99);

        s.store_scale(35, 394, p.p99);

        s.store_square(36, 35);

        s.store_scale_ad(207, A::limited_exp(A::scale(s.ad_value(395), ((((p.p239 / p.p99)).max(1e-38)) as f64).ln())), 1.0 / (s.v[34]));

        s.store_div_ad_lhs(208, A::limited_exp(A::mul(s.ad_value(395), A::ln(A::max_with_scalar(A::div_from_scalar(p.p239, s.ad_value(35)), 1e-38)))), 36);

        s.store_mul_ad_lhs(186, A::mul(s.ad_value(3), s.ad_value(205)), 208);

        s.store_scale_ad(273, A::offset(A::scale(A::scale(s.ad_value(3), 0.3333333333333333), 1.0 / (p.p315)), p.p313), (p.p316 * 1.0 / (((p.p315 * p.p2) * (s.v[0] - p.p314)))));

        s.v[581] = if (s.v[273] > 0.001) { 1.0 } else { 0.0 };

        if (s.v[581] != 0.0) {
            s.store_div_from_scalar(273, 1.0, 273);
        }

        if (!(s.v[581] != 0.0)) {
            s.store_scalar(273, 1000.0);
        }

        s.v[583] = if ((p.p18 != 0.0) && (p.p310 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[583] != 0.0) {
            s.store_offset_ad(271, A::offset(A::voltage(ctx, &nodes, Some(4), None), ctx.temperature()), p.p9);
        }

        if (!(s.v[583] != 0.0)) {
            s.store_scalar(271, (ctx.temperature() + p.p9));
        }

        s.v[272] = (p.p298 + 273.15);

        s.store_scale_ad(271, A::sub(A::offset(s.ad_value(271), s.v[272]), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(271), (-s.v[272])), A::offset(s.ad_value(271), (-s.v[272]))), ((0.25 * 0.01) * 0.01)))), 0.5);

        s.store_div(96, 271, 95);

        s.store_sub(97, 271, 95);

        s.store_scale(55, 271, 8.61708e-5);

        s.store_sub_from_scalar_ad(54, p.p55, A::div(A::mul(A::scale(s.ad_value(271), p.p299), s.ad_value(271)), A::offset(s.ad_value(271), p.p300)));

        s.store_mul_ad(35, A::scale(s.ad_value(271), 0.003331667499583542), A::sqrt(A::scale(s.ad_value(271), 0.003331667499583542)));

        s.store_mul_ad(100, A::scale(s.ad_value(35), p.p54), A::limited_exp(A::sub_from_scalar((p.p55 / ((2.0 * 8.61708e-5) * 300.15)), A::div(s.ad_value(54), A::scale(s.ad_value(55), 2.0)))));

        s.store_mul_ad_rhs(80, 55, A::ln(A::max_with_scalar(A::div(A::mul(s.ad_value(289), s.ad_value(290)), A::square(s.ad_value(100))), 1e-38)));

        s.store_mul_ad_rhs(50, 55, A::ln(A::max_with_scalar(A::div(s.ad_value(290), s.ad_value(100)), 1e-38)));

        let assign3610_ad_e4240: A = A::add(A::sub(A::scale(s.ad_value(54), 0.5), A::mul(s.ad_value(55), A::ln(A::max_with_scalar(A::div_from_scalar(p.p52, s.ad_value(100)), 1e-38)))), A::sqrt(A::offset(A::mul(A::sub(A::scale(s.ad_value(54), 0.5), A::mul(s.ad_value(55), A::ln(A::max_with_scalar(A::div_from_scalar(p.p52, s.ad_value(100)), 1e-38)))), A::sub(A::scale(s.ad_value(54), 0.5), A::mul(s.ad_value(55), A::ln(A::max_with_scalar(A::div_from_scalar(p.p52, s.ad_value(100)), 1e-38))))), ((4.0 * 0.0001) * 0.0001))));
        s.store_sub_ad(51, A::scale(s.ad_value(54), 0.5), A::scale(assign3610_ad_e4240, 0.5));

        s.v[585] = if ((p.p52 != 0.0) && (!(if self.param_given[58] { 1.0 } else { 0.0 } != 0.0))) { 1.0 } else { 0.0 };

        s.v[586] = if (p.p13 == (-1.0)) { 1.0 } else { 0.0 };

        if ((s.v[585] != 0.0) && (s.v[586] != 0.0)) {
            s.store_add_ad_lhs(288, A::offset(s.ad_value(288), (-(0.5 * p.p55))), 51);
        }

        if ((s.v[585] != 0.0) && (!(s.v[586] != 0.0))) {
            s.store_sub_ad_lhs(288, A::offset(s.ad_value(288), (0.5 * p.p55)), 51);
        }

        s.store_offset_scaled(98, 54, 0.5, p.p53);

        s.store_mul_ad_rhs(52, 212, A::sub(s.ad_value(287), s.ad_value(98)));

        s.store_mul_ad_rhs(53, 212, A::sub(s.ad_value(288), s.ad_value(98)));

        s.store_sub_ad(99, A::offset(A::scale(s.ad_value(54), 0.5), p.p53), A::mul(s.ad_value(212), A::min(A::scale(s.ad_value(54), 0.5), A::mul(s.ad_value(55), A::ln(A::max_with_scalar(A::div(s.ad_value(289), s.ad_value(100)), 1e-38))))));

        s.store_mul_ad_rhs(200, 212, A::sub(s.ad_value(287), s.ad_value(99)));

        s.store_mul_ad_rhs(240, 212, A::sub(s.ad_value(288), s.ad_value(99)));

        let assign3720_ad_e4372: A = A::mul(A::mul(s.ad_value(331), A::pow(s.ad_value(96), s.ad_value(338))), A::offset(A::offset(A::scale(A::add(A::offset(A::mul(s.ad_value(337), s.ad_value(97)), 0.9), A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(337), s.ad_value(97)), 0.9), A::offset(A::mul(s.ad_value(337), s.ad_value(97)), 0.9)), ((4.0 * 0.001) * 0.001)))), 0.5), 1.0), (-(0.5 * (0.9 + ((((0.9 * 0.9) + ((4.0 * 0.001) * 0.001))) as f64).sqrt())))));
        s.store_ad(126, &assign3720_ad_e4372);

        s.store_mul_ad_rhs(123, 333, A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(97), p.p159), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(97), p.p159), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(97), p.p159), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5));

        s.store_mul_ad_rhs(122, 332, A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(339), s.ad_value(97)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(339), s.ad_value(97)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(339), s.ad_value(97)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5));

        s.store_mul_ad_rhs(125, 334, A::pow(s.ad_value(96), s.ad_value(340)));

        s.store_mul_ad_rhs(124, 335, A::pow(s.ad_value(96), s.ad_value(341)));

        s.store_scale_ad(150, A::add(A::offset(A::offset(A::mul(s.ad_value(355), s.ad_value(97)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(355), s.ad_value(97)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(355), s.ad_value(97)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5);

        s.store_mul_ad_rhs(353, 353, A::offset(A::scale(s.ad_value(278), p.p120), 1.0));

        let assign3790_ad_e4534: A = A::mul(s.ad_value(400), A::offset(A::offset(A::scale(A::add(A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97))), A::sqrt(A::offset(A::mul(A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97))), A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97)))), ((4.0 * 0.001) * 0.001)))), 0.5), 1.0), (-(0.5 * (0.9 + ((((0.9 * 0.9) + ((4.0 * 0.001) * 0.001))) as f64).sqrt())))));
        s.store_ad(164, &assign3790_ad_e4534);

        s.v[587] = if (s.v[164] < 1000.0) { 1.0 } else { 0.0 };

        if (s.v[587] != 0.0) {
            s.store_scalar(164, 1000.0);
        }

        let assign3820_ad_e4586: A = A::mul(s.ad_value(402), A::offset(A::offset(A::scale(A::add(A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97))), A::sqrt(A::offset(A::mul(A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97))), A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97)))), ((4.0 * 0.001) * 0.001)))), 0.5), 1.0), (-(0.5 * (0.9 + ((((0.9 * 0.9) + ((4.0 * 0.001) * 0.001))) as f64).sqrt())))));
        s.store_ad(166, &assign3820_ad_e4586);

        s.v[588] = if (s.v[166] < 1000.0) { 1.0 } else { 0.0 };

        if (s.v[588] != 0.0) {
            s.store_scalar(166, 1000.0);
        }

        let assign3850_ad_e4638: A = A::mul(s.ad_value(403), A::offset(A::offset(A::scale(A::add(A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97))), A::sqrt(A::offset(A::mul(A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97))), A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97)))), ((4.0 * 0.001) * 0.001)))), 0.5), 1.0), (-(0.5 * (0.9 + ((((0.9 * 0.9) + ((4.0 * 0.001) * 0.001))) as f64).sqrt())))));
        s.store_ad(167, &assign3850_ad_e4638);

        s.v[589] = if (s.v[167] < 1000.0) { 1.0 } else { 0.0 };

        if (s.v[589] != 0.0) {
            s.store_scalar(167, 1000.0);
        }

        let assign3880_ad_e4688: A = A::mul(s.ad_value(316), A::offset(A::offset(A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(97), p.p309), (-(-0.9))), (-0.0001)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(97), p.p309), (-(-0.9))), (-0.0001)), A::offset(A::offset(A::scale(s.ad_value(97), p.p309), (-(-0.9))), (-0.0001))), (-((4.0 * (-0.9)) * 0.0001))))), 0.5), (-0.9)), 1.0));
        s.store_ad(107, &assign3880_ad_e4688);

        s.store_mul_ad_rhs(354, 354, A::offset(A::scale(s.ad_value(278), p.p131), 1.0));

        let assign3900_ad_e4740: A = A::mul(s.ad_value(401), A::offset(A::offset(A::scale(A::add(A::sub_from_scalar(0.9, A::mul(s.ad_value(354), s.ad_value(97))), A::sqrt(A::offset(A::mul(A::sub_from_scalar(0.9, A::mul(s.ad_value(354), s.ad_value(97))), A::sub_from_scalar(0.9, A::mul(s.ad_value(354), s.ad_value(97)))), ((4.0 * 0.001) * 0.001)))), 0.5), 1.0), (-(0.5 * (0.9 + ((((0.9 * 0.9) + ((4.0 * 0.001) * 0.001))) as f64).sqrt())))));
        s.store_ad(165, &assign3900_ad_e4740);

        let assign3910_ad_e4780: A = A::offset(A::scale(A::add(A::offset(A::mul(s.ad_value(326), A::offset(A::scale(s.ad_value(97), p.p121), 1.0)), (-2.0)), A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(326), A::offset(A::scale(s.ad_value(97), p.p121), 1.0)), (-2.0)), A::offset(A::mul(s.ad_value(326), A::offset(A::scale(s.ad_value(97), p.p121), 1.0)), (-2.0))), ((4.0 * 0.001) * 0.001)))), 0.5), 2.0);
        s.store_ad(168, &assign3910_ad_e4780);

        s.store_add_ad_rhs(175, 322, A::mul(s.ad_value(323), s.ad_value(97)));

        let assign3930_ad_e4823: A = A::add(A::offset(A::sub(A::mul(s.ad_value(325), s.ad_value(97)), A::neg(s.ad_value(324))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(325), s.ad_value(97)), A::neg(s.ad_value(324))), (-1e-6)), A::offset(A::sub(A::mul(s.ad_value(325), s.ad_value(97)), A::neg(s.ad_value(324))), (-1e-6))), A::scale(A::neg(s.ad_value(324)), (4.0 * 1e-6)))));
        s.store_add_ad_rhs(176, 324, A::sub(A::scale(assign3930_ad_e4823, 0.5), s.ad_value(324)));

        s.store_add_ad_rhs(108, 417, A::mul(s.ad_value(418), s.ad_value(97)));

        s.store_mul_ad_rhs(182, 327, A::scale(A::add(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(330), s.ad_value(97))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(330), s.ad_value(97))), (-1e-6)), A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(330), s.ad_value(97))), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5));

        s.store_mul_ad(102, A::offset(A::div_from_scalar(p.p302, s.ad_value(2)), p.p301), A::offset(s.ad_value(96), (-1.0)));

        s.store_mul_ad_rhs(103, 368, A::pow(s.ad_value(96), s.ad_value(356)));

        s.store_mul_ad_rhs(104, 379, A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(357), s.ad_value(97)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(357), s.ad_value(97)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(357), s.ad_value(97)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5));

        s.store_mul_ad_rhs(105, 375, A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(358), s.ad_value(97)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(358), s.ad_value(97)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(358), s.ad_value(97)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5));

        s.store_limited_exp_ad(106, A::mul(s.ad_value(359), A::ln(A::max_with_scalar(s.ad_value(96), 1e-38))));

        s.store_mul(185, 186, 106);

        s.store_mul_ad_rhs(29, 212, A::voltage(ctx, &nodes, Some(8), Some(6)));

        s.store_mul_ad_rhs(30, 212, A::voltage(ctx, &nodes, Some(5), Some(6)));

        s.store_mul_ad_rhs(31, 212, A::voltage(ctx, &nodes, Some(8), Some(5)));

        s.store_mul_ad_rhs(32, 212, A::voltage(ctx, &nodes, Some(3), Some(6)));

        s.store_mul_ad_rhs(33, 212, A::voltage(ctx, &nodes, Some(3), Some(5)));

        s.store_mul_ad_rhs(209, 212, A::voltage(ctx, &nodes, Some(8), Some(3)));

        s.v[27] = 1.0;

        s.v[590] = if (s.v[30] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[590] != 0.0) {
            s.store_scalar(27, (-1.0));
        }

        if (s.v[590] != 0.0) {
            s.copy_ad(22, 31);
        }

        if (s.v[590] != 0.0) {
            s.store_neg(26, 30);
        }

        if (s.v[590] != 0.0) {
            s.copy_ad(23, 33);
        }

        if (s.v[590] != 0.0) {
            s.copy_ad(24, 32);
        }

        if (!(s.v[590] != 0.0)) {
            s.copy_ad(22, 29);
        }

        if (!(s.v[590] != 0.0)) {
            s.copy_ad(26, 30);
        }

        if (!(s.v[590] != 0.0)) {
            s.copy_ad(23, 32);
        }

        if (!(s.v[590] != 0.0)) {
            s.copy_ad(24, 33);
        }

        s.store_mul_ad_rhs(234, 212, A::voltage(ctx, &nodes, Some(7), Some(5)));

        s.store_mul_ad_rhs(235, 212, A::voltage(ctx, &nodes, Some(7), Some(6)));

        s.store_offset_ad(73, A::sqrt(A::offset(A::square(s.ad_value(26)), 0.0004)), (-0.02));

        s.store_scaled_sub(74, 73, 26, 0.5);

        s.store_add(25, 23, 74);

        s.store_sub(69, 22, 52);

        s.store_sub(70, 23, 53);

        s.v[77] = ((((s.v[21] * p.p49) * p.p45)) as f64).sqrt();

        s.v[76] = (((p.p49 * ((s.v[21] * p.p45) + (0.375 * p.p49)))) as f64).sqrt();

        s.store_add_ad_lhs(34, A::scale(A::add(A::scale(s.ad_value(69), (p.p46 * s.v[21])), A::scale(s.ad_value(70), ((p.p45 * s.v[21]) + p.p49))), 1.0 / (s.v[78])), 74);

        s.store_offset_ad(35, A::scale(A::atan(A::add(s.ad_value(311), A::mul(s.ad_value(312), s.ad_value(34)))), 0.3183098861837907), 0.5);

        s.store_offset_scaled(75, 35, (s.v[77] - s.v[76]), s.v[76]);

        s.store_offset_ad(61, A::div(A::mul(s.ad_value(314), s.ad_value(2)), s.ad_value(75)), 1e-6);

        s.v[591] = if (s.v[61] < 40.0) { 1.0 } else { 0.0 };

        if (s.v[591] != 0.0) {
            s.store_div_from_scalar_ad(88, 0.5, A::offset(A::cosh(s.ad_value(61)), (-1.0)));
        }

        if (!(s.v[591] != 0.0)) {
            s.store_limited_exp_ad(88, A::neg(s.ad_value(61)));
        }

        s.store_offset_ad(61, A::div(A::mul(s.ad_value(319), s.ad_value(2)), s.ad_value(75)), 1e-6);

        s.v[592] = if (s.v[61] < 40.0) { 1.0 } else { 0.0 };

        if (s.v[592] != 0.0) {
            s.store_div_from_scalar_ad(90, 0.5, A::offset(A::cosh(s.ad_value(61)), (-1.0)));
        }

        if (!(s.v[592] != 0.0)) {
            s.store_limited_exp_ad(90, A::neg(s.ad_value(61)));
        }

        s.v[593] = if (s.v[61] < 40.0) { 1.0 } else { 0.0 };

        if (s.v[593] != 0.0) {
            s.store_div_from_scalar_ad(91, 1.0, A::max_with_scalar(A::offset(A::scale(A::offset(A::cosh(s.ad_value(61)), (-2.0)), p.p83), 1.0), 1e-6));
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
        if (!(s.v[593] != 0.0)) {
            s.store_div_ad(91, A::limited_exp(A::neg(s.ad_value(61))), A::max_with_scalar(A::offset(A::limited_exp(A::neg(s.ad_value(61))), p.p83), 1e-6));
        }

        s.store_offset_ad(61, A::div(A::mul(s.ad_value(362), s.ad_value(2)), s.ad_value(75)), 1e-6);

        s.v[594] = if (s.v[61] < 40.0) { 1.0 } else { 0.0 };

        if (s.v[594] != 0.0) {
            s.store_add_ad_lhs(153, A::div(A::scale(s.ad_value(363), 0.5), A::offset(A::cosh(s.ad_value(61)), (-1.0))), 364);
        }

        if (!(s.v[594] != 0.0)) {
            s.store_add_ad_lhs(153, A::mul(s.ad_value(363), A::limited_exp(A::neg(s.ad_value(61)))), 364);
        }

        s.v[595] = if (p.p13 == (-1.0)) { 1.0 } else { 0.0 };

        if (s.v[595] != 0.0) {
            s.store_div_ad_lhs(79, A::mul(s.ad_value(298), s.ad_value(2)), 75);
        }

        s.v[596] = if (s.v[79] > 40.0) { 1.0 } else { 0.0 };

        if ((s.v[595] != 0.0) && (s.v[596] != 0.0)) {
            s.store_scale_ad(34, A::limited_exp(s.ad_value(79)), 0.5);
        }

        if ((s.v[595] != 0.0) && (!(s.v[596] != 0.0))) {
            s.store_offset_ad(34, A::cosh(s.ad_value(79)), (-1.0));
        }

        if (s.v[595] != 0.0) {
            s.store_sub_ad_rhs(35, 299, A::div(A::scale(s.ad_value(300), 0.5), s.ad_value(34)));
        }

        if (s.v[595] != 0.0) {
            s.copy_ad(36, 301);
        }

        if (s.v[595] != 0.0) {
            s.copy_ad(246, 296);
        }

        if (s.v[595] != 0.0) {
            s.copy_ad(247, 297);
        }

        if (s.v[595] != 0.0) {
            s.copy_ad(248, 295);
        }

        if (!(s.v[595] != 0.0)) {
            s.store_div_ad_lhs(79, A::mul(s.ad_value(305), s.ad_value(2)), 75);
        }

        s.v[597] = if (s.v[79] > 40.0) { 1.0 } else { 0.0 };

        if ((!(s.v[595] != 0.0)) && (s.v[597] != 0.0)) {
            s.store_scale_ad(34, A::limited_exp(s.ad_value(79)), 0.5);
        }

        if ((!(s.v[595] != 0.0)) && (!(s.v[597] != 0.0))) {
            s.store_offset_ad(34, A::cosh(s.ad_value(79)), (-1.0));
        }

        if (!(s.v[595] != 0.0)) {
            s.store_sub_ad_rhs(35, 306, A::div(A::scale(s.ad_value(307), 0.5), s.ad_value(34)));
        }

        if (!(s.v[595] != 0.0)) {
            s.copy_ad(36, 308);
        }

        if (!(s.v[595] != 0.0)) {
            s.copy_ad(246, 303);
        }

        if (!(s.v[595] != 0.0)) {
            s.copy_ad(247, 304);
        }

        if (!(s.v[595] != 0.0)) {
            s.copy_ad(248, 302);
        }

        s.store_sub(34, 35, 36);

        s.store_add_ad_rhs(241, 36, A::scale(A::add(s.ad_value(34), A::sqrt(A::offset(A::square(s.ad_value(34)), 0.0001))), 0.5));

        s.v[244] = (((1.60219e-19 * p.p52) * s.v[16]) / ((2.0 * s.v[19]) * s.v[19]));

        s.v[598] = if (p.p52 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[598] != 0.0) {
            let assign4690_ad_e5407: A = A::scale(A::add(A::mul(s.ad_value(213), A::sub(A::mul(s.ad_value(212), s.ad_value(25)), s.ad_value(246))), A::sqrt(A::offset(A::mul(A::mul(s.ad_value(213), A::sub(A::mul(s.ad_value(212), s.ad_value(25)), s.ad_value(246))), A::mul(s.ad_value(213), A::sub(A::mul(s.ad_value(212), s.ad_value(25)), s.ad_value(246)))), ((4.0 * 0.01) * 0.01)))), (0.5 * 1.0 / (s.v[244])));
            s.store_offset_ad(34, A::sqrt(A::offset(assign4690_ad_e5407, 1.0)), (-1.0));
        }

        if (!(s.v[598] != 0.0)) {
            s.store_scalar(34, 0.0);
        }

        s.store_mul_ad_lhs(245, A::scale(s.ad_value(34), s.v[244]), 34);

        let assign4720_ad_e5459: A = A::sub(A::scale(A::add(A::offset(A::sub(A::neg(s.ad_value(245)), A::neg(s.ad_value(247))), (-0.01)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::neg(s.ad_value(245)), A::neg(s.ad_value(247))), (-0.01)), A::offset(A::sub(A::neg(s.ad_value(245)), A::neg(s.ad_value(247))), (-0.01))), A::scale(A::neg(s.ad_value(247)), (4.0 * 0.01))))), 0.5), s.ad_value(247));
        s.store_neg_ad(245, assign4720_ad_e5459);

        s.store_sub_from_scalar(72, (-1.2), 74);

        s.v[243] = (((-s.v[19]) * s.v[20]) / ((s.v[19] + s.v[20]) * s.v[17]));

        s.store_mul_ad(242, A::scale(s.ad_value(241), s.v[243]), A::sub(A::sub(s.ad_value(70), A::mul(A::mul(A::mul(s.ad_value(212), s.ad_value(213)), s.ad_value(248)), s.ad_value(245))), s.ad_value(72)));

        s.store_scale_ad(28, A::add(s.ad_value(25), A::sqrt(A::offset(A::square(s.ad_value(25)), ((4.0 * 0.001) * 0.001)))), 0.5);

        s.store_add_ad_lhs(87, A::offset(s.ad_value(50), 0.4), 315);

        s.v[599] = if (s.v[87] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[599] != 0.0) {
            s.store_scalar(84, 0.0);
        }

        if (!(s.v[599] != 0.0)) {
            s.store_mul_ad(84, A::mul(s.ad_value(320), s.ad_value(89)), A::sqrt(s.ad_value(87)));
        }

        s.store_mul_ad(83, A::mul(A::neg(s.ad_value(313)), s.ad_value(88)), A::sub(s.ad_value(80), s.ad_value(87)));

        s.store_add_ad(82, A::mul(A::mul(A::neg(A::add(s.ad_value(107), A::mul(s.ad_value(318), s.ad_value(25)))), s.ad_value(90)), A::add(s.ad_value(73), A::mul(s.ad_value(317), A::sqrt(A::offset(s.ad_value(73), 0.01))))), A::mul(A::mul(s.ad_value(92), s.ad_value(91)), A::pow(A::offset(s.ad_value(73), 0.01), s.ad_value(93))));

        s.store_mul_ad_lhs(85, A::div(A::neg(s.ad_value(309)), A::add(s.ad_value(2), s.ad_value(310))), 73);

        s.v[35] = ((s.v[20] * s.v[19]) / (s.v[20] + s.v[19]));

        s.store_mul_ad_lhs(36, A::add(s.ad_value(293), A::scale(s.ad_value(28), p.p70)), 73);

        s.store_add_ad(37, A::add(A::scale(s.ad_value(25), p.p66), A::mul(A::scale(s.ad_value(25), p.p67), s.ad_value(25))), A::mul(s.ad_value(88), A::add(A::add(A::add(s.ad_value(292), A::mul(s.ad_value(294), s.ad_value(25))), A::mul(A::scale(s.ad_value(25), p.p69), s.ad_value(25))), s.ad_value(36))));

        s.store_scale_ad(81, A::mul(s.ad_value(55), A::add(A::offset(s.ad_value(291), (s.v[17] + s.v[35])), s.ad_value(37))), 1.0 / ((s.v[17] + s.v[35])));

        s.store_scale_ad(60, A::scale(s.ad_value(290), (1.60219e-19 * (p.p49 * 1.0 / (s.v[17])))), (1.0 - ((0.5 * p.p49) / (p.p49 + (s.v[21] * p.p46)))));

        s.store_mul_ad_lhs(34, A::offset(A::div_from_scalar(p.p304, s.ad_value(2)), p.p303), 25);

        s.store_add_ad_rhs(101, 102, A::mul(s.ad_value(34), A::offset(s.ad_value(96), (-1.0))));

        s.store_add_ad_lhs(86, A::add(A::add(A::add(A::add(A::add(s.ad_value(83), s.ad_value(82)), s.ad_value(84)), s.ad_value(85)), s.ad_value(60)), s.ad_value(101)), 242);

        s.store_offset_ad(71, A::sub(s.ad_value(69), s.ad_value(86)), p.p10);

        s.store_div_ad(421, A::scale(s.ad_value(100), ((2.0 * 1.60219e-19) * (p.p49 * p.p49))), A::scale(s.ad_value(55), s.v[16]));

        s.v[419] = (s.v[17] / s.v[20]);

        s.v[420] = (s.v[19] / s.v[20]);

        s.store_ln(449, 421);

        s.store_sub_from_scalar(450, ((39.47841) as f64).ln(), 449);

        s.v[451] = (s.v[419] * s.v[419]);

        s.v[454] = (s.v[419] / (((s.v[420] * s.v[419]) + s.v[420]) + s.v[419]));

        s.v[460] = 1.0;

        s.store_sub_from_scalar_ad(461, ((s.v[451] * s.v[460]) * s.v[460]), A::mul(s.ad_value(421), A::limited_exp(A::scale(s.ad_value(50), 2.0))));

        s.store_sqrt(462, 461);

        s.store_div_ad(463, A::sub_from_scalar(1.0, A::scale(s.ad_value(462), 0.125)), A::sub_from_scalar(0.5, A::scale(s.ad_value(462), 0.041666666666666664)));

        s.store_mul_ad_lhs(35, A::sub(A::offset(A::ln(A::max_with_scalar(A::offset(A::scale(s.ad_value(463), (s.v[419] * s.v[460])), (((s.v[419] * s.v[419]) * s.v[460]) * s.v[460])), 1e-38)), 1.0), A::ln(A::max_with_scalar(s.ad_value(421), 1e-38))), 55);

        s.store_div(422, 71, 81);

        s.store_div_ad_lhs(423, A::offset(A::sub(s.ad_value(70), s.ad_value(86)), p.p10), 81);

        s.store_sub_ad_lhs(453, A::ln(A::offset(A::mul(A::scale(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451]), A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_sub_ad_lhs(424, A::ln(A::offset(A::mul(A::scale(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451]), A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_scale_ad(452, A::add(s.ad_value(424), A::scale(s.ad_value(423), s.v[420])), 1.0 / ((1.0 + s.v[420])));

        s.store_add_ad_rhs(426, 423, A::scale(A::sub(s.ad_value(422), s.ad_value(423)), s.v[454]));

        s.store_ad(430, &A::min(s.ad_value(426), s.ad_value(453)));

        s.store_ad(430, &A::min(s.ad_value(430), s.ad_value(450)));

        s.store_scale_ad(448, A::add(s.ad_value(430), A::scale(s.ad_value(422), s.v[419])), 1.0 / ((1.0 + s.v[419])));

        s.store_sub(34, 448, 430);

        s.store_div_ad_lhs(37, A::mul(A::limited_exp(s.ad_value(430)), A::offset(A::limited_exp(s.ad_value(34)), (-1.0))), 34);

        s.store_sub(429, 423, 452);

        s.store_sub_ad(442, A::mul(A::scale(s.ad_value(429), (s.v[420] * s.v[420])), s.ad_value(429)), A::mul(s.ad_value(421), A::exp(s.ad_value(452))));

        s.v[600] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[600] != 0.0) {
            s.store_scaled_sub(429, 423, 430, s.v[420]);
        }

        if (s.v[600] != 0.0) {
            s.store_scalar(440, (40.0 * s.v[419]));
        }

        if (s.v[600] != 0.0) {
            s.store_add(455, 440, 429);
        }

        if (s.v[600] != 0.0) {
            s.store_mul(37, 440, 429);
        }

        if (s.v[600] != 0.0) {
            s.store_offset_scaled(38, 455, 0.06534, 1.0);
        }

        if (s.v[600] != 0.0) {
            s.store_offset_ad(39, A::add(A::scale(s.ad_value(455), 8.57973), s.ad_value(37)), 39.47841);
        }

        if (s.v[600] != 0.0) {
            s.store_add_ad(40, A::scale(s.ad_value(455), 78.95683), A::scale(s.ad_value(37), 39.47841));
        }

        if (s.v[600] != 0.0) {
            s.store_div_ad(442, A::sub(A::sqrt(A::add(A::mul(A::scale(s.ad_value(38), (-4.0)), s.ad_value(40)), A::square(s.ad_value(39)))), s.ad_value(39)), A::scale(s.ad_value(38), 2.0));
        }

        if (s.v[600] != 0.0) {
            s.store_scale_ad(37, A::sub(A::scale(s.ad_value(450), (1.0 + s.v[419])), s.ad_value(430)), 1.0 / (s.v[419]));
        }

        if (s.v[600] != 0.0) {
            s.store_scale_ad(34, A::offset(A::sub(s.ad_value(422), s.ad_value(37)), 2.0), (40.0 * 0.2));
        }

        if (s.v[600] != 0.0) {
            s.store_mul_ad_rhs(442, 442, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::offset(A::sub(s.ad_value(422), s.ad_value(37)), 2.0)), 1.0 / ((2.0 / 0.69))))));
        }

        if (s.v[600] != 0.0) {
            s.store_ad(442, &A::min_with_scalar(s.ad_value(442), 50.0));
        }

        s.store_ad(422, &A::max(s.ad_value(422), s.ad_value(450)));

        s.store_sub_ad_lhs(424, A::ln(A::offset(A::mul(A::scale(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451]), A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_scale_ad(37, A::sub(A::scale(s.ad_value(450), (1.0 + s.v[419])), s.ad_value(430)), 1.0 / (s.v[419]));

        s.store_sub_ad_lhs(38, A::ln(A::offset(A::mul(A::scale(A::sub(s.ad_value(37), s.ad_value(450)), s.v[451]), A::sub(s.ad_value(37), s.ad_value(450))), 39.47841)), 449);

        s.store_sub(39, 38, 450);

        s.store_sub(424, 424, 39);

        s.store_sub(440, 422, 424);

        s.store_mul_ad(34, A::neg(s.ad_value(421)), A::exp(s.ad_value(424)));

        s.store_scale(35, 440, s.v[451]);

        s.store_div_ad(425, A::neg(A::sub(A::add(A::mul(s.ad_value(35), s.ad_value(440)), s.ad_value(34)), s.ad_value(442))), A::add(A::scale(s.ad_value(35), (-2.0)), s.ad_value(34)));

        s.store_add(424, 424, 425);

        s.store_sub(440, 422, 424);

        s.store_scale(36, 440, s.v[451]);

        s.store_div_from_scalar_ad(34, 1.0, A::sub(A::mul(s.ad_value(36), s.ad_value(440)), s.ad_value(442)));

        s.store_sub_ad_lhs(465, A::sub(A::ln(A::abs(A::sub(A::mul(s.ad_value(36), s.ad_value(440)), s.ad_value(442)))), s.ad_value(449)), 424);

        s.store_div_from_scalar_ad(466, 1.0, A::offset(A::mul(A::scale(s.ad_value(36), (-2.0)), s.ad_value(34)), (-1.0)));

        s.store_add_ad(467, A::mul(A::mul(A::mul(A::scale(s.ad_value(36), (-4.0)), s.ad_value(36)), s.ad_value(34)), s.ad_value(34)), A::scale(s.ad_value(34), (2.0 * s.v[451])));

        s.store_mul(35, 465, 466);

        s.store_sub_ad(425, A::neg(s.ad_value(35)), A::mul(A::mul(A::mul(A::scale(s.ad_value(35), 0.5), s.ad_value(35)), s.ad_value(467)), s.ad_value(466)));

        s.store_ad(425, &A::max_with_scalar(s.ad_value(425), (-10.0)));

        s.store_ad(425, &A::min_with_scalar(s.ad_value(425), 10.0));

        s.store_add(424, 424, 425);

        s.store_sub(440, 422, 424);

        s.store_scale(36, 440, s.v[451]);

        s.store_div_from_scalar_ad(34, 1.0, A::sub(A::mul(s.ad_value(36), s.ad_value(440)), s.ad_value(442)));

        s.store_sub_ad_lhs(465, A::sub(A::ln(A::abs(A::sub(A::mul(s.ad_value(36), s.ad_value(440)), s.ad_value(442)))), s.ad_value(449)), 424);

        s.store_div_from_scalar_ad(466, 1.0, A::offset(A::mul(A::scale(s.ad_value(36), (-2.0)), s.ad_value(34)), (-1.0)));

        s.store_add_ad(467, A::mul(A::mul(A::mul(A::scale(s.ad_value(36), (-4.0)), s.ad_value(36)), s.ad_value(34)), s.ad_value(34)), A::scale(s.ad_value(34), (2.0 * s.v[451])));

        s.store_mul(35, 465, 466);

        s.store_sub_ad(425, A::neg(s.ad_value(35)), A::mul(A::mul(A::mul(A::scale(s.ad_value(35), 0.5), s.ad_value(35)), s.ad_value(467)), s.ad_value(466)));

        s.store_ad(425, &A::max_with_scalar(s.ad_value(425), (-10.0)));

        s.store_ad(425, &A::min_with_scalar(s.ad_value(425), 10.0));

        s.store_add(424, 424, 425);

        s.store_ad(424, &A::max(s.ad_value(424), A::offset(s.ad_value(450), (-4.0))));

        s.store_div(422, 71, 81);

        s.store_sub_ad_rhs(448, 448, A::ln(A::offset(A::exp(A::sub(s.ad_value(448), A::scale(s.ad_value(424), 1.05))), 1.0)));

        s.store_ad(448, &A::min(s.ad_value(448), s.ad_value(424)));

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_ad(457, A::neg(s.ad_value(421)), A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.v[601] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[601] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[601] != 0.0) {
            s.store_div_from_scalar_ad(459, 1.0, A::sin(A::scale(s.ad_value(439), 0.5)));
        }

        if (s.v[601] != 0.0) {
            s.store_square(35, 459);
        }

        if (s.v[601] != 0.0) {
            s.store_mul_ad_lhs(458, A::cos(A::scale(s.ad_value(439), 0.5)), 459);
        }

        if (s.v[601] != 0.0) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), (-0.5)), 439);
        }

        if (s.v[601] != 0.0) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), 0.25), 34);
        }

        if (!(s.v[601] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[601] != 0.0)) {
            s.store_div_from_scalar_ad(459, 1.0, A::sinh(A::scale(s.ad_value(439), 0.5)));
        }

        if (!(s.v[601] != 0.0)) {
            s.store_square(35, 459);
        }

        if (!(s.v[601] != 0.0)) {
            s.store_sqrt_ad(458, A::offset(s.ad_value(35), 1.0));
        }

        if (!(s.v[601] != 0.0)) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), 0.5), 439);
        }

        if (!(s.v[601] != 0.0)) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), (-0.25)), 34);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_sub_ad(429, A::add(A::sub(s.ad_value(423), s.ad_value(422)), s.ad_value(440)), A::ln(A::abs(A::mul(A::mul(A::mul(s.ad_value(442), s.ad_value(35)), s.ad_value(37)), s.ad_value(37)))));

        s.store_add_ad_rhs(427, 457, A::mul(A::add(s.ad_value(456), s.ad_value(446)), A::add(A::scale(s.ad_value(429), s.v[420]), s.ad_value(456))));

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_ad_lhs(443, A::scale(s.ad_value(456), ((-2.0) * s.v[419])), 457);

        s.store_mul(444, 445, 443);

        s.store_sub_ad(441, A::offset(A::scale(A::mul(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37)), 2.0), (-1.0)), A::mul(s.ad_value(447), s.ad_value(443)));

        s.store_add_ad(428, A::add(A::sub(s.ad_value(457), A::scale(A::add(s.ad_value(456), s.ad_value(36)), s.v[419])), A::mul(s.ad_value(456), s.ad_value(444))), A::scale(A::add(A::mul(s.ad_value(441), s.ad_value(36)), A::mul(s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])))), s.v[420]));

        s.store_div_ad_lhs(425, A::neg(s.ad_value(427)), 428);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_ad(457, A::neg(s.ad_value(421)), A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.v[602] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[602] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[602] != 0.0) {
            s.store_div_from_scalar_ad(459, 1.0, A::sin(A::scale(s.ad_value(439), 0.5)));
        }

        if (s.v[602] != 0.0) {
            s.store_square(35, 459);
        }

        if (s.v[602] != 0.0) {
            s.store_mul_ad_lhs(458, A::cos(A::scale(s.ad_value(439), 0.5)), 459);
        }

        if (s.v[602] != 0.0) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), (-0.5)), 439);
        }

        if (s.v[602] != 0.0) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), 0.25), 34);
        }

        if (!(s.v[602] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[602] != 0.0)) {
            s.store_div_from_scalar_ad(459, 1.0, A::sinh(A::scale(s.ad_value(439), 0.5)));
        }

        if (!(s.v[602] != 0.0)) {
            s.store_square(35, 459);
        }

        if (!(s.v[602] != 0.0)) {
            s.store_sqrt_ad(458, A::offset(s.ad_value(35), 1.0));
        }

        if (!(s.v[602] != 0.0)) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), 0.5), 439);
        }

        if (!(s.v[602] != 0.0)) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), (-0.25)), 34);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_sub_ad(429, A::add(A::sub(s.ad_value(423), s.ad_value(422)), s.ad_value(440)), A::ln(A::abs(A::mul(A::mul(A::mul(s.ad_value(442), s.ad_value(35)), s.ad_value(37)), s.ad_value(37)))));

        s.store_add_ad_rhs(427, 457, A::mul(A::add(s.ad_value(456), s.ad_value(446)), A::add(A::scale(s.ad_value(429), s.v[420]), s.ad_value(456))));

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_ad_lhs(443, A::scale(s.ad_value(456), ((-2.0) * s.v[419])), 457);

        s.store_mul(444, 445, 443);

        s.store_sub_ad(441, A::offset(A::scale(A::mul(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37)), 2.0), (-1.0)), A::mul(s.ad_value(447), s.ad_value(443)));

        s.store_add_ad(428, A::add(A::sub(s.ad_value(457), A::scale(A::add(s.ad_value(456), s.ad_value(36)), s.v[419])), A::mul(s.ad_value(456), s.ad_value(444))), A::scale(A::add(A::mul(s.ad_value(441), s.ad_value(36)), A::mul(s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])))), s.v[420]));

        s.store_div_ad_lhs(425, A::neg(s.ad_value(427)), 428);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_ad(457, A::neg(s.ad_value(421)), A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.v[603] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[603] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[603] != 0.0) {
            s.store_div_from_scalar_ad(459, 1.0, A::sin(A::scale(s.ad_value(439), 0.5)));
        }

        if (s.v[603] != 0.0) {
            s.store_square(35, 459);
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
        if (s.v[603] != 0.0) {
            s.store_mul_ad_lhs(458, A::cos(A::scale(s.ad_value(439), 0.5)), 459);
        }

        if (s.v[603] != 0.0) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), (-0.5)), 439);
        }

        if (s.v[603] != 0.0) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), 0.25), 34);
        }

        if (!(s.v[603] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[603] != 0.0)) {
            s.store_div_from_scalar_ad(459, 1.0, A::sinh(A::scale(s.ad_value(439), 0.5)));
        }

        if (!(s.v[603] != 0.0)) {
            s.store_square(35, 459);
        }

        if (!(s.v[603] != 0.0)) {
            s.store_sqrt_ad(458, A::offset(s.ad_value(35), 1.0));
        }

        if (!(s.v[603] != 0.0)) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), 0.5), 439);
        }

        if (!(s.v[603] != 0.0)) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), (-0.25)), 34);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_sub_ad(429, A::add(A::sub(s.ad_value(423), s.ad_value(422)), s.ad_value(440)), A::ln(A::abs(A::mul(A::mul(A::mul(s.ad_value(442), s.ad_value(35)), s.ad_value(37)), s.ad_value(37)))));

        s.store_add_ad_rhs(427, 457, A::mul(A::add(s.ad_value(456), s.ad_value(446)), A::add(A::scale(s.ad_value(429), s.v[420]), s.ad_value(456))));

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_ad_lhs(443, A::scale(s.ad_value(456), ((-2.0) * s.v[419])), 457);

        s.store_mul(444, 445, 443);

        s.store_sub_ad(441, A::offset(A::scale(A::mul(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37)), 2.0), (-1.0)), A::mul(s.ad_value(447), s.ad_value(443)));

        s.store_add_ad(428, A::add(A::sub(s.ad_value(457), A::scale(A::add(s.ad_value(456), s.ad_value(36)), s.v[419])), A::mul(s.ad_value(456), s.ad_value(444))), A::scale(A::add(A::mul(s.ad_value(441), s.ad_value(36)), A::mul(s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])))), s.v[420]));

        s.store_div_ad_lhs(425, A::neg(s.ad_value(427)), 428);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_ad(457, A::neg(s.ad_value(421)), A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.v[604] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[604] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[604] != 0.0) {
            s.store_div_from_scalar_ad(459, 1.0, A::sin(A::scale(s.ad_value(439), 0.5)));
        }

        if (s.v[604] != 0.0) {
            s.store_square(35, 459);
        }

        if (s.v[604] != 0.0) {
            s.store_mul_ad_lhs(458, A::cos(A::scale(s.ad_value(439), 0.5)), 459);
        }

        if (s.v[604] != 0.0) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), (-0.5)), 439);
        }

        if (s.v[604] != 0.0) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), 0.25), 34);
        }

        if (!(s.v[604] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[604] != 0.0)) {
            s.store_div_from_scalar_ad(459, 1.0, A::sinh(A::scale(s.ad_value(439), 0.5)));
        }

        if (!(s.v[604] != 0.0)) {
            s.store_square(35, 459);
        }

        if (!(s.v[604] != 0.0)) {
            s.store_sqrt_ad(458, A::offset(s.ad_value(35), 1.0));
        }

        if (!(s.v[604] != 0.0)) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), 0.5), 439);
        }

        if (!(s.v[604] != 0.0)) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), (-0.25)), 34);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_sub_ad(429, A::add(A::sub(s.ad_value(423), s.ad_value(422)), s.ad_value(440)), A::ln(A::abs(A::mul(A::mul(A::mul(s.ad_value(442), s.ad_value(35)), s.ad_value(37)), s.ad_value(37)))));

        s.store_add_ad_rhs(427, 457, A::mul(A::add(s.ad_value(456), s.ad_value(446)), A::add(A::scale(s.ad_value(429), s.v[420]), s.ad_value(456))));

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_ad_lhs(443, A::scale(s.ad_value(456), ((-2.0) * s.v[419])), 457);

        s.store_mul(444, 445, 443);

        s.store_sub_ad(441, A::offset(A::scale(A::mul(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37)), 2.0), (-1.0)), A::mul(s.ad_value(447), s.ad_value(443)));

        s.store_add_ad(428, A::add(A::sub(s.ad_value(457), A::scale(A::add(s.ad_value(456), s.ad_value(36)), s.v[419])), A::mul(s.ad_value(456), s.ad_value(444))), A::scale(A::add(A::mul(s.ad_value(441), s.ad_value(36)), A::mul(s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])))), s.v[420]));

        s.store_div_ad_lhs(425, A::neg(s.ad_value(427)), 428);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_ad(457, A::neg(s.ad_value(421)), A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.v[605] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[605] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[605] != 0.0) {
            s.store_div_from_scalar_ad(459, 1.0, A::sin(A::scale(s.ad_value(439), 0.5)));
        }

        if (s.v[605] != 0.0) {
            s.store_square(35, 459);
        }

        if (s.v[605] != 0.0) {
            s.store_mul_ad_lhs(458, A::cos(A::scale(s.ad_value(439), 0.5)), 459);
        }

        if (s.v[605] != 0.0) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), (-0.5)), 439);
        }

        if (s.v[605] != 0.0) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), 0.25), 34);
        }

        if (!(s.v[605] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[605] != 0.0)) {
            s.store_div_from_scalar_ad(459, 1.0, A::sinh(A::scale(s.ad_value(439), 0.5)));
        }

        if (!(s.v[605] != 0.0)) {
            s.store_square(35, 459);
        }

        if (!(s.v[605] != 0.0)) {
            s.store_sqrt_ad(458, A::offset(s.ad_value(35), 1.0));
        }

        if (!(s.v[605] != 0.0)) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), 0.5), 439);
        }

        if (!(s.v[605] != 0.0)) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), (-0.25)), 34);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_sub_ad(429, A::add(A::sub(s.ad_value(423), s.ad_value(422)), s.ad_value(440)), A::ln(A::abs(A::mul(A::mul(A::mul(s.ad_value(442), s.ad_value(35)), s.ad_value(37)), s.ad_value(37)))));

        s.store_add_ad_rhs(427, 457, A::mul(A::add(s.ad_value(456), s.ad_value(446)), A::add(A::scale(s.ad_value(429), s.v[420]), s.ad_value(456))));

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_ad_lhs(443, A::scale(s.ad_value(456), ((-2.0) * s.v[419])), 457);

        s.store_mul(444, 445, 443);

        s.store_sub_ad(441, A::offset(A::scale(A::mul(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37)), 2.0), (-1.0)), A::mul(s.ad_value(447), s.ad_value(443)));

        s.store_add_ad(428, A::add(A::sub(s.ad_value(457), A::scale(A::add(s.ad_value(456), s.ad_value(36)), s.v[419])), A::mul(s.ad_value(456), s.ad_value(444))), A::scale(A::add(A::mul(s.ad_value(441), s.ad_value(36)), A::mul(s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])))), s.v[420]));

        s.store_div_ad_lhs(425, A::neg(s.ad_value(427)), 428);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_mul_ad_rhs(34, 421, A::exp(s.ad_value(448)));

        s.store_sub_ad_lhs(442, A::mul(A::scale(s.ad_value(440), s.v[451]), s.ad_value(440)), 34);

        s.v[606] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[606] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[606] != 0.0) {
            s.store_scale(36, 439, 0.5);
        }

        if (s.v[606] != 0.0) {
            s.store_div_ad_rhs(446, 439, A::tan(s.ad_value(36)));
        }

        if (s.v[606] != 0.0) {
            s.store_sin(40, 36);
        }

        if (s.v[606] != 0.0) {
            s.store_mul_ad_lhs(35, A::neg(s.ad_value(40)), 40);
        }

        if (!(s.v[606] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[606] != 0.0)) {
            s.store_scale(36, 439, 0.5);
        }

        if (!(s.v[606] != 0.0)) {
            s.store_sinh(40, 36);
        }

        if (!(s.v[606] != 0.0)) {
            s.store_square(35, 40);
        }

        if (!(s.v[606] != 0.0)) {
            s.store_div_ad_rhs(446, 439, A::tanh(s.ad_value(36)));
        }

        s.store_div_ad(437, A::sub(A::scale(s.ad_value(440), s.v[419]), s.ad_value(446)), A::sub_from_scalar(1.0, A::div(s.ad_value(442), A::mul(s.ad_value(35), s.ad_value(34)))));

        s.store_mul_ad_lhs(431, A::scale(s.ad_value(440), s.v[17]), 81);

        s.store_mul_ad_lhs(435, A::scale(s.ad_value(437), s.v[20]), 81);

        s.store_sub(433, 435, 431);

        s.store_sub_ad_rhs(430, 423, A::div(s.ad_value(433), A::scale(s.ad_value(81), s.v[19])));

        s.store_scale_ad(210, A::mul(A::add(s.ad_value(448), s.ad_value(430)), s.ad_value(81)), 0.5);

        s.store_scale(109, 435, 1.0 / (s.v[17]));

        s.store_scale(111, 290, (1.60219e-19 * (p.p49 * 1.0 / (s.v[17]))));

        s.store_add_ad_lhs(36, A::scale(A::mul(s.ad_value(114), s.ad_value(431)), 1.0 / (s.v[17])), 111);

        s.store_scale_ad(37, A::add(s.ad_value(36), A::sqrt(A::offset(A::square(s.ad_value(36)), 0.001))), 0.5);

        s.store_scale(127, 37, s.v[129]);

        s.store_add_ad_lhs(36, A::scale(A::mul(s.ad_value(143), s.ad_value(433)), 1.0 / (s.v[19])), 111);

        s.store_scale_ad(37, A::add(s.ad_value(36), A::sqrt(A::offset(A::square(s.ad_value(36)), 0.001))), 0.5);

        s.store_scale(128, 37, s.v[144]);

        s.v[59] = (0.01 / s.v[17]);

        s.store_ad(607, &A::pow(A::scale(A::offset(A::abs(A::scale(s.ad_value(109), 1.0 / (s.v[59]))), 1.0), 0.5), s.ad_value(124)));

        s.store_add_ad(608, A::mul(A::add(s.ad_value(122), A::mul(s.ad_value(23), s.ad_value(123))), A::pow(A::abs(s.ad_value(127)), A::add(s.ad_value(336), A::mul(s.ad_value(342), s.ad_value(23))))), A::div(s.ad_value(125), s.ad_value(607)));

        s.store_offset(112, 608, 1.0);

        s.store_scale_ad(112, A::add(A::offset(s.ad_value(112), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(112), (-1.0)), A::offset(s.ad_value(112), (-1.0))), ((0.25 * p.p154) * p.p154)))), 0.5);

        s.store_scale(112, 112, 1.0 / (p.p11));

        s.store_div(141, 126, 112);

        s.store_ad(609, &A::pow(A::scale(A::offset(A::abs(A::scale(s.ad_value(109), 1.0 / (s.v[59]))), 1.0), 0.5), s.ad_value(348)));

        s.store_add_ad(610, A::mul(A::add(s.ad_value(345), A::mul(s.ad_value(23), s.ad_value(346))), A::pow(A::abs(s.ad_value(128)), A::add(s.ad_value(349), A::mul(s.ad_value(350), s.ad_value(23))))), A::div(s.ad_value(347), s.ad_value(609)));

        s.store_offset(112, 610, 1.0);

        s.store_scale_ad(112, A::add(A::offset(s.ad_value(112), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(112), (-1.0)), A::offset(s.ad_value(112), (-1.0))), ((0.25 * p.p154) * p.p154)))), 0.5);

        s.store_scale(112, 112, 1.0 / (p.p11));

        s.store_div(142, 344, 112);

        s.store_sub_ad_rhs(34, 71, A::scale(s.ad_value(431), 1.0 / (s.v[17])));

        s.store_sub_ad(35, A::sub(s.ad_value(70), s.ad_value(86)), A::scale(s.ad_value(433), 1.0 / (s.v[19])));

        s.store_div_ad(139, A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::add(A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::exp(A::div(s.ad_value(35), s.ad_value(81)))));

        s.store_div_ad(140, A::exp(A::div(s.ad_value(35), s.ad_value(81))), A::add(A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::exp(A::div(s.ad_value(35), s.ad_value(81)))));

        s.store_add_ad(121, A::mul(s.ad_value(139), s.ad_value(141)), A::mul(s.ad_value(140), s.ad_value(142)));

        s.v[611] = if (p.p14 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[611] != 0.0) {
            s.store_scalar(152, 0.0);
        }

        s.v[612] = if (p.p14 == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[611] != 0.0)) && (s.v[612] != 0.0)) {
            s.store_offset_ad(38, A::mul(s.ad_value(284), s.ad_value(109)), 1.0);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[612] != 0.0)) {
            s.store_div_from_scalar(35, 1.0, 38);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[612] != 0.0)) {
            s.store_scale_ad(34, A::add(s.ad_value(35), A::sqrt(A::offset(A::square(s.ad_value(35)), 0.01))), 0.5);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[612] != 0.0)) {
            s.store_mul_ad_lhs(152, A::scale(A::mul(A::add(s.ad_value(134), A::mul(s.ad_value(281), s.ad_value(34))), s.ad_value(131)), p.p2), 150);
        }

        if ((!(s.v[611] != 0.0)) && (!(s.v[612] != 0.0))) {
            s.store_offset_ad(38, A::mul(s.ad_value(284), s.ad_value(109)), 1.0);
        }

        if ((!(s.v[611] != 0.0)) && (!(s.v[612] != 0.0))) {
            s.store_div_from_scalar(35, 1.0, 38);
        }

        if ((!(s.v[611] != 0.0)) && (!(s.v[612] != 0.0))) {
            s.store_scale_ad(34, A::add(s.ad_value(35), A::sqrt(A::offset(A::square(s.ad_value(35)), 0.01))), 0.5);
        }

        if ((!(s.v[611] != 0.0)) && (!(s.v[612] != 0.0))) {
            s.store_mul_ad_lhs(152, A::scale(A::mul(A::add(A::add(A::add(s.ad_value(132), s.ad_value(133)), s.ad_value(134)), A::mul(s.ad_value(281), s.ad_value(34))), s.ad_value(131)), p.p2), 150);
        }

        s.store_div_ad_lhs(169, A::scale(s.ad_value(164), 2.0), 121);

        s.store_mul(170, 169, 2);

        s.store_mul_ad_rhs(40, 404, A::add(A::add(s.ad_value(109), A::mul(s.ad_value(407), s.ad_value(28))), A::mul(A::scale(s.ad_value(55), 2.0), s.ad_value(405))));

        s.v[613] = if (s.v[152] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[613] != 0.0) {
            s.store_div_ad(162, A::mul(s.ad_value(170), s.ad_value(40)), A::add(s.ad_value(170), s.ad_value(40)));
        }

        if (!(s.v[613] != 0.0)) {
            s.store_scaled_mul(177, 3, 164, s.v[17]);
        }

        if (!(s.v[613] != 0.0)) {
            s.store_mul(34, 177, 152);
        }

        if (!(s.v[613] != 0.0)) {
            s.store_scale(178, 34, 2.0);
        }

        if (!(s.v[613] != 0.0)) {
            s.store_add_ad(179, A::add(s.ad_value(40), s.ad_value(170)), A::mul(A::scale(s.ad_value(40), 3.0), s.ad_value(34)));
        }

        if (!(s.v[613] != 0.0)) {
            s.store_mul_ad_rhs(180, 40, A::add(s.ad_value(170), A::mul(A::scale(s.ad_value(40), 2.0), s.ad_value(34))));
        }

        if (!(s.v[613] != 0.0)) {
            s.store_div_ad_lhs(162, A::sub(s.ad_value(179), A::sqrt(A::sub(A::square(s.ad_value(179)), A::mul(A::scale(s.ad_value(178), 2.0), s.ad_value(180))))), 178);
        }

        s.store_offset_ad(162, A::scale(A::add(A::offset(s.ad_value(162), (-0.001)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(162), (-0.001)), A::offset(s.ad_value(162), (-0.001))), ((4.0 * 1e-5) * 1e-5)))), 0.5), 0.001);

        s.store_ad(41, &A::pow(A::div(s.ad_value(26), s.ad_value(162)), s.ad_value(168)));

        s.store_ad(42, &A::pow(A::offset(s.ad_value(41), 1.0), s.ad_value(163)));

        s.store_div(113, 26, 42);

        s.v[614] = if (s.v[113] > s.v[26]) { 1.0 } else { 0.0 };

        if (s.v[614] != 0.0) {
            s.copy_ad(113, 26);
        }

        s.store_div_ad_lhs(422, A::sub(s.ad_value(71), s.ad_value(113)), 81);

        s.store_div_ad_lhs(423, A::sub(A::offset(A::sub(s.ad_value(70), s.ad_value(86)), p.p10), s.ad_value(113)), 81);

        s.store_sub_ad_lhs(453, A::ln(A::offset(A::mul(A::scale(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451]), A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_sub_ad_lhs(424, A::ln(A::offset(A::mul(A::scale(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451]), A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_scale_ad(37, A::sub(A::scale(s.ad_value(450), (1.0 + s.v[419])), s.ad_value(430)), 1.0 / (s.v[419]));

        s.store_sub_ad_lhs(38, A::ln(A::offset(A::mul(A::scale(A::sub(s.ad_value(37), s.ad_value(450)), s.v[451]), A::sub(s.ad_value(37), s.ad_value(450))), 39.47841)), 449);

        s.store_sub(39, 38, 450);

        s.store_sub(424, 424, 39);

        s.store_scale_ad(452, A::add(s.ad_value(424), A::scale(s.ad_value(423), s.v[420])), 1.0 / ((1.0 + s.v[420])));

        s.store_add_ad_rhs(426, 423, A::scale(A::sub(s.ad_value(422), s.ad_value(423)), s.v[454]));

        s.store_ad(430, &A::min(s.ad_value(426), s.ad_value(453)));

        s.store_ad(430, &A::min(s.ad_value(430), s.ad_value(450)));

        s.store_scale_ad(448, A::add(s.ad_value(430), A::scale(s.ad_value(422), s.v[419])), 1.0 / ((1.0 + s.v[419])));

        s.store_sub(34, 448, 430);

        s.store_div_ad_lhs(37, A::mul(A::limited_exp(s.ad_value(430)), A::offset(A::limited_exp(s.ad_value(34)), (-1.0))), 34);

        s.store_sub(429, 423, 452);

        s.store_sub_ad(442, A::mul(A::scale(s.ad_value(429), (s.v[420] * s.v[420])), s.ad_value(429)), A::mul(s.ad_value(421), A::exp(s.ad_value(452))));

        s.v[615] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[615] != 0.0) {
            s.store_scaled_sub(429, 423, 430, s.v[420]);
        }

        if (s.v[615] != 0.0) {
            s.store_scalar(440, (40.0 * s.v[419]));
        }

        if (s.v[615] != 0.0) {
            s.store_add(455, 440, 429);
        }

        if (s.v[615] != 0.0) {
            s.store_mul(37, 440, 429);
        }

        if (s.v[615] != 0.0) {
            s.store_offset_scaled(38, 455, 0.06534, 1.0);
        }

        if (s.v[615] != 0.0) {
            s.store_offset_ad(39, A::add(A::scale(s.ad_value(455), 8.57973), s.ad_value(37)), 39.47841);
        }

        if (s.v[615] != 0.0) {
            s.store_add_ad(40, A::scale(s.ad_value(455), 78.95683), A::scale(s.ad_value(37), 39.47841));
        }

        if (s.v[615] != 0.0) {
            s.store_div_ad(442, A::sub(A::sqrt(A::add(A::mul(A::scale(s.ad_value(38), (-4.0)), s.ad_value(40)), A::square(s.ad_value(39)))), s.ad_value(39)), A::scale(s.ad_value(38), 2.0));
        }

        if (s.v[615] != 0.0) {
            s.store_scale_ad(37, A::sub(A::scale(s.ad_value(450), (1.0 + s.v[419])), s.ad_value(430)), 1.0 / (s.v[419]));
        }

        if (s.v[615] != 0.0) {
            s.store_scale_ad(34, A::offset(A::sub(s.ad_value(422), s.ad_value(37)), 2.0), (40.0 * 0.2));
        }

        if (s.v[615] != 0.0) {
            s.store_mul_ad_rhs(442, 442, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::offset(A::sub(s.ad_value(422), s.ad_value(37)), 2.0)), 1.0 / ((2.0 / 0.69))))));
        }

        if (s.v[615] != 0.0) {
            s.store_ad(442, &A::min_with_scalar(s.ad_value(442), 50.0));
        }

        s.store_ad(422, &A::max(s.ad_value(422), s.ad_value(450)));

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
        s.store_sub_ad_lhs(424, A::ln(A::offset(A::mul(A::scale(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451]), A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_scale_ad(37, A::sub(A::scale(s.ad_value(450), (1.0 + s.v[419])), s.ad_value(430)), 1.0 / (s.v[419]));

        s.store_sub_ad_lhs(38, A::ln(A::offset(A::mul(A::scale(A::sub(s.ad_value(37), s.ad_value(450)), s.v[451]), A::sub(s.ad_value(37), s.ad_value(450))), 39.47841)), 449);

        s.store_sub(39, 38, 450);

        s.store_sub(424, 424, 39);

        s.store_sub(440, 422, 424);

        s.store_mul_ad(34, A::neg(s.ad_value(421)), A::exp(s.ad_value(424)));

        s.store_scale(35, 440, s.v[451]);

        s.store_div_ad(425, A::neg(A::sub(A::add(A::mul(s.ad_value(35), s.ad_value(440)), s.ad_value(34)), s.ad_value(442))), A::add(A::scale(s.ad_value(35), (-2.0)), s.ad_value(34)));

        s.store_add(424, 424, 425);

        s.store_sub(440, 422, 424);

        s.store_scale(36, 440, s.v[451]);

        s.store_div_from_scalar_ad(34, 1.0, A::sub(A::mul(s.ad_value(36), s.ad_value(440)), s.ad_value(442)));

        s.store_sub_ad_lhs(465, A::sub(A::ln(A::abs(A::sub(A::mul(s.ad_value(36), s.ad_value(440)), s.ad_value(442)))), s.ad_value(449)), 424);

        s.store_div_from_scalar_ad(466, 1.0, A::offset(A::mul(A::scale(s.ad_value(36), (-2.0)), s.ad_value(34)), (-1.0)));

        s.store_add_ad(467, A::mul(A::mul(A::mul(A::scale(s.ad_value(36), (-4.0)), s.ad_value(36)), s.ad_value(34)), s.ad_value(34)), A::scale(s.ad_value(34), (2.0 * s.v[451])));

        s.store_mul(35, 465, 466);

        s.store_sub_ad(425, A::neg(s.ad_value(35)), A::mul(A::mul(A::mul(A::scale(s.ad_value(35), 0.5), s.ad_value(35)), s.ad_value(467)), s.ad_value(466)));

        s.store_ad(425, &A::max_with_scalar(s.ad_value(425), (-10.0)));

        s.store_ad(425, &A::min_with_scalar(s.ad_value(425), 10.0));

        s.store_add(424, 424, 425);

        s.store_sub(440, 422, 424);

        s.store_scale(36, 440, s.v[451]);

        s.store_div_from_scalar_ad(34, 1.0, A::sub(A::mul(s.ad_value(36), s.ad_value(440)), s.ad_value(442)));

        s.store_sub_ad_lhs(465, A::sub(A::ln(A::abs(A::sub(A::mul(s.ad_value(36), s.ad_value(440)), s.ad_value(442)))), s.ad_value(449)), 424);

        s.store_div_from_scalar_ad(466, 1.0, A::offset(A::mul(A::scale(s.ad_value(36), (-2.0)), s.ad_value(34)), (-1.0)));

        s.store_add_ad(467, A::mul(A::mul(A::mul(A::scale(s.ad_value(36), (-4.0)), s.ad_value(36)), s.ad_value(34)), s.ad_value(34)), A::scale(s.ad_value(34), (2.0 * s.v[451])));

        s.store_mul(35, 465, 466);

        s.store_sub_ad(425, A::neg(s.ad_value(35)), A::mul(A::mul(A::mul(A::scale(s.ad_value(35), 0.5), s.ad_value(35)), s.ad_value(467)), s.ad_value(466)));

        s.store_ad(425, &A::max_with_scalar(s.ad_value(425), (-10.0)));

        s.store_ad(425, &A::min_with_scalar(s.ad_value(425), 10.0));

        s.store_add(424, 424, 425);

        s.store_ad(424, &A::max(s.ad_value(424), A::offset(s.ad_value(450), (-4.0))));

        s.store_div_ad_lhs(422, A::sub(s.ad_value(71), s.ad_value(113)), 81);

        s.store_sub_ad_rhs(448, 448, A::ln(A::offset(A::exp(A::sub(s.ad_value(448), A::scale(s.ad_value(424), 1.05))), 1.0)));

        s.store_ad(448, &A::min(s.ad_value(448), s.ad_value(424)));

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_ad(457, A::neg(s.ad_value(421)), A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.v[616] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[616] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[616] != 0.0) {
            s.store_div_from_scalar_ad(459, 1.0, A::sin(A::scale(s.ad_value(439), 0.5)));
        }

        if (s.v[616] != 0.0) {
            s.store_square(35, 459);
        }

        if (s.v[616] != 0.0) {
            s.store_mul_ad_lhs(458, A::cos(A::scale(s.ad_value(439), 0.5)), 459);
        }

        if (s.v[616] != 0.0) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), (-0.5)), 439);
        }

        if (s.v[616] != 0.0) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), 0.25), 34);
        }

        if (!(s.v[616] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[616] != 0.0)) {
            s.store_div_from_scalar_ad(459, 1.0, A::sinh(A::scale(s.ad_value(439), 0.5)));
        }

        if (!(s.v[616] != 0.0)) {
            s.store_square(35, 459);
        }

        if (!(s.v[616] != 0.0)) {
            s.store_sqrt_ad(458, A::offset(s.ad_value(35), 1.0));
        }

        if (!(s.v[616] != 0.0)) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), 0.5), 439);
        }

        if (!(s.v[616] != 0.0)) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), (-0.25)), 34);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_sub_ad(429, A::add(A::sub(s.ad_value(423), s.ad_value(422)), s.ad_value(440)), A::ln(A::abs(A::mul(A::mul(A::mul(s.ad_value(442), s.ad_value(35)), s.ad_value(37)), s.ad_value(37)))));

        s.store_add_ad_rhs(427, 457, A::mul(A::add(s.ad_value(456), s.ad_value(446)), A::add(A::scale(s.ad_value(429), s.v[420]), s.ad_value(456))));

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_ad_lhs(443, A::scale(s.ad_value(456), ((-2.0) * s.v[419])), 457);

        s.store_mul(444, 445, 443);

        s.store_sub_ad(441, A::offset(A::scale(A::mul(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37)), 2.0), (-1.0)), A::mul(s.ad_value(447), s.ad_value(443)));

        s.store_add_ad(428, A::add(A::sub(s.ad_value(457), A::scale(A::add(s.ad_value(456), s.ad_value(36)), s.v[419])), A::mul(s.ad_value(456), s.ad_value(444))), A::scale(A::add(A::mul(s.ad_value(441), s.ad_value(36)), A::mul(s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])))), s.v[420]));

        s.store_div_ad_lhs(425, A::neg(s.ad_value(427)), 428);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_ad(457, A::neg(s.ad_value(421)), A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.v[617] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[617] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[617] != 0.0) {
            s.store_div_from_scalar_ad(459, 1.0, A::sin(A::scale(s.ad_value(439), 0.5)));
        }

        if (s.v[617] != 0.0) {
            s.store_square(35, 459);
        }

        if (s.v[617] != 0.0) {
            s.store_mul_ad_lhs(458, A::cos(A::scale(s.ad_value(439), 0.5)), 459);
        }

        if (s.v[617] != 0.0) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), (-0.5)), 439);
        }

        if (s.v[617] != 0.0) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), 0.25), 34);
        }

        if (!(s.v[617] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[617] != 0.0)) {
            s.store_div_from_scalar_ad(459, 1.0, A::sinh(A::scale(s.ad_value(439), 0.5)));
        }

        if (!(s.v[617] != 0.0)) {
            s.store_square(35, 459);
        }

        if (!(s.v[617] != 0.0)) {
            s.store_sqrt_ad(458, A::offset(s.ad_value(35), 1.0));
        }

        if (!(s.v[617] != 0.0)) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), 0.5), 439);
        }

        if (!(s.v[617] != 0.0)) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), (-0.25)), 34);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_sub_ad(429, A::add(A::sub(s.ad_value(423), s.ad_value(422)), s.ad_value(440)), A::ln(A::abs(A::mul(A::mul(A::mul(s.ad_value(442), s.ad_value(35)), s.ad_value(37)), s.ad_value(37)))));

        s.store_add_ad_rhs(427, 457, A::mul(A::add(s.ad_value(456), s.ad_value(446)), A::add(A::scale(s.ad_value(429), s.v[420]), s.ad_value(456))));

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_ad_lhs(443, A::scale(s.ad_value(456), ((-2.0) * s.v[419])), 457);

        s.store_mul(444, 445, 443);

        s.store_sub_ad(441, A::offset(A::scale(A::mul(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37)), 2.0), (-1.0)), A::mul(s.ad_value(447), s.ad_value(443)));

        s.store_add_ad(428, A::add(A::sub(s.ad_value(457), A::scale(A::add(s.ad_value(456), s.ad_value(36)), s.v[419])), A::mul(s.ad_value(456), s.ad_value(444))), A::scale(A::add(A::mul(s.ad_value(441), s.ad_value(36)), A::mul(s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])))), s.v[420]));

        s.store_div_ad_lhs(425, A::neg(s.ad_value(427)), 428);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_ad(457, A::neg(s.ad_value(421)), A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.v[618] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[618] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[618] != 0.0) {
            s.store_div_from_scalar_ad(459, 1.0, A::sin(A::scale(s.ad_value(439), 0.5)));
        }

        if (s.v[618] != 0.0) {
            s.store_square(35, 459);
        }

        if (s.v[618] != 0.0) {
            s.store_mul_ad_lhs(458, A::cos(A::scale(s.ad_value(439), 0.5)), 459);
        }

        if (s.v[618] != 0.0) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), (-0.5)), 439);
        }

        if (s.v[618] != 0.0) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), 0.25), 34);
        }

        if (!(s.v[618] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[618] != 0.0)) {
            s.store_div_from_scalar_ad(459, 1.0, A::sinh(A::scale(s.ad_value(439), 0.5)));
        }

        if (!(s.v[618] != 0.0)) {
            s.store_square(35, 459);
        }

        if (!(s.v[618] != 0.0)) {
            s.store_sqrt_ad(458, A::offset(s.ad_value(35), 1.0));
        }

        if (!(s.v[618] != 0.0)) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), 0.5), 439);
        }

        if (!(s.v[618] != 0.0)) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), (-0.25)), 34);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_sub_ad(429, A::add(A::sub(s.ad_value(423), s.ad_value(422)), s.ad_value(440)), A::ln(A::abs(A::mul(A::mul(A::mul(s.ad_value(442), s.ad_value(35)), s.ad_value(37)), s.ad_value(37)))));

        s.store_add_ad_rhs(427, 457, A::mul(A::add(s.ad_value(456), s.ad_value(446)), A::add(A::scale(s.ad_value(429), s.v[420]), s.ad_value(456))));

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_ad_lhs(443, A::scale(s.ad_value(456), ((-2.0) * s.v[419])), 457);

        s.store_mul(444, 445, 443);

        s.store_sub_ad(441, A::offset(A::scale(A::mul(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37)), 2.0), (-1.0)), A::mul(s.ad_value(447), s.ad_value(443)));

        s.store_add_ad(428, A::add(A::sub(s.ad_value(457), A::scale(A::add(s.ad_value(456), s.ad_value(36)), s.v[419])), A::mul(s.ad_value(456), s.ad_value(444))), A::scale(A::add(A::mul(s.ad_value(441), s.ad_value(36)), A::mul(s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])))), s.v[420]));

        s.store_div_ad_lhs(425, A::neg(s.ad_value(427)), 428);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_ad(457, A::neg(s.ad_value(421)), A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.v[619] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[619] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[619] != 0.0) {
            s.store_div_from_scalar_ad(459, 1.0, A::sin(A::scale(s.ad_value(439), 0.5)));
        }

        if (s.v[619] != 0.0) {
            s.store_square(35, 459);
        }

        if (s.v[619] != 0.0) {
            s.store_mul_ad_lhs(458, A::cos(A::scale(s.ad_value(439), 0.5)), 459);
        }

        if (s.v[619] != 0.0) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), (-0.5)), 439);
        }

        if (s.v[619] != 0.0) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), 0.25), 34);
        }

        if (!(s.v[619] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[619] != 0.0)) {
            s.store_div_from_scalar_ad(459, 1.0, A::sinh(A::scale(s.ad_value(439), 0.5)));
        }

        if (!(s.v[619] != 0.0)) {
            s.store_square(35, 459);
        }

        if (!(s.v[619] != 0.0)) {
            s.store_sqrt_ad(458, A::offset(s.ad_value(35), 1.0));
        }

        if (!(s.v[619] != 0.0)) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), 0.5), 439);
        }

        if (!(s.v[619] != 0.0)) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), (-0.25)), 34);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_sub_ad(429, A::add(A::sub(s.ad_value(423), s.ad_value(422)), s.ad_value(440)), A::ln(A::abs(A::mul(A::mul(A::mul(s.ad_value(442), s.ad_value(35)), s.ad_value(37)), s.ad_value(37)))));

        s.store_add_ad_rhs(427, 457, A::mul(A::add(s.ad_value(456), s.ad_value(446)), A::add(A::scale(s.ad_value(429), s.v[420]), s.ad_value(456))));

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_ad_lhs(443, A::scale(s.ad_value(456), ((-2.0) * s.v[419])), 457);

        s.store_mul(444, 445, 443);

        s.store_sub_ad(441, A::offset(A::scale(A::mul(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37)), 2.0), (-1.0)), A::mul(s.ad_value(447), s.ad_value(443)));

        s.store_add_ad(428, A::add(A::sub(s.ad_value(457), A::scale(A::add(s.ad_value(456), s.ad_value(36)), s.v[419])), A::mul(s.ad_value(456), s.ad_value(444))), A::scale(A::add(A::mul(s.ad_value(441), s.ad_value(36)), A::mul(s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])))), s.v[420]));

        s.store_div_ad_lhs(425, A::neg(s.ad_value(427)), 428);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_ad(457, A::neg(s.ad_value(421)), A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.v[620] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[620] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[620] != 0.0) {
            s.store_div_from_scalar_ad(459, 1.0, A::sin(A::scale(s.ad_value(439), 0.5)));
        }

        if (s.v[620] != 0.0) {
            s.store_square(35, 459);
        }

        if (s.v[620] != 0.0) {
            s.store_mul_ad_lhs(458, A::cos(A::scale(s.ad_value(439), 0.5)), 459);
        }

        if (s.v[620] != 0.0) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), (-0.5)), 439);
        }

        if (s.v[620] != 0.0) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), 0.25), 34);
        }

        if (!(s.v[620] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[620] != 0.0)) {
            s.store_div_from_scalar_ad(459, 1.0, A::sinh(A::scale(s.ad_value(439), 0.5)));
        }

        if (!(s.v[620] != 0.0)) {
            s.store_square(35, 459);
        }

        if (!(s.v[620] != 0.0)) {
            s.store_sqrt_ad(458, A::offset(s.ad_value(35), 1.0));
        }

        if (!(s.v[620] != 0.0)) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), 0.5), 439);
        }

        if (!(s.v[620] != 0.0)) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), (-0.25)), 34);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_sub_ad(429, A::add(A::sub(s.ad_value(423), s.ad_value(422)), s.ad_value(440)), A::ln(A::abs(A::mul(A::mul(A::mul(s.ad_value(442), s.ad_value(35)), s.ad_value(37)), s.ad_value(37)))));

        s.store_add_ad_rhs(427, 457, A::mul(A::add(s.ad_value(456), s.ad_value(446)), A::add(A::scale(s.ad_value(429), s.v[420]), s.ad_value(456))));

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_ad_lhs(443, A::scale(s.ad_value(456), ((-2.0) * s.v[419])), 457);

        s.store_mul(444, 445, 443);

        s.store_sub_ad(441, A::offset(A::scale(A::mul(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37)), 2.0), (-1.0)), A::mul(s.ad_value(447), s.ad_value(443)));

        s.store_add_ad(428, A::add(A::sub(s.ad_value(457), A::scale(A::add(s.ad_value(456), s.ad_value(36)), s.v[419])), A::mul(s.ad_value(456), s.ad_value(444))), A::scale(A::add(A::mul(s.ad_value(441), s.ad_value(36)), A::mul(s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])))), s.v[420]));

        s.store_div_ad_lhs(425, A::neg(s.ad_value(427)), 428);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_mul_ad_rhs(34, 421, A::exp(s.ad_value(448)));

        s.store_sub_ad_lhs(442, A::mul(A::scale(s.ad_value(440), s.v[451]), s.ad_value(440)), 34);

        s.v[621] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[621] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[621] != 0.0) {
            s.store_scale(36, 439, 0.5);
        }

        if (s.v[621] != 0.0) {
            s.store_div_ad_rhs(446, 439, A::tan(s.ad_value(36)));
        }

        if (s.v[621] != 0.0) {
            s.store_sin(40, 36);
        }

        if (s.v[621] != 0.0) {
            s.store_mul_ad_lhs(35, A::neg(s.ad_value(40)), 40);
        }

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
        if (!(s.v[621] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[621] != 0.0)) {
            s.store_scale(36, 439, 0.5);
        }

        if (!(s.v[621] != 0.0)) {
            s.store_sinh(40, 36);
        }

        if (!(s.v[621] != 0.0)) {
            s.store_square(35, 40);
        }

        if (!(s.v[621] != 0.0)) {
            s.store_div_ad_rhs(446, 439, A::tanh(s.ad_value(36)));
        }

        s.store_div_ad(438, A::sub(A::scale(s.ad_value(440), s.v[419]), s.ad_value(446)), A::sub_from_scalar(1.0, A::div(s.ad_value(442), A::mul(s.ad_value(35), s.ad_value(34)))));

        s.store_mul_ad_lhs(432, A::scale(s.ad_value(440), s.v[17]), 81);

        s.store_mul_ad_lhs(436, A::scale(s.ad_value(438), s.v[20]), 81);

        s.store_sub(434, 436, 432);

        s.store_sub_ad_rhs(430, 423, A::div(s.ad_value(434), A::scale(s.ad_value(81), s.v[19])));

        s.store_scale(110, 436, 1.0 / (s.v[17]));

        s.store_scaled_add(46, 109, 110, 0.5);

        s.store_sub(49, 109, 110);

        s.store_scale(48, 290, (1.60219e-19 * (p.p49 * 1.0 / (s.v[17]))));

        s.store_scale_ad(34, A::powf(s.ad_value(113), 2.0), 1600.0);

        s.v[622] = if (p.p162 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[622] != 0.0) {
            s.store_add_ad(47, A::scale(A::add(s.ad_value(431), s.ad_value(432)), 1.0 / ((2.0 * s.v[17]))), A::scale(A::mul(A::scale(A::sub_from_scalar(1.0, A::limited_exp(A::neg(s.ad_value(34)))), (p.p162 * 0.5)), A::sub(s.ad_value(431), s.ad_value(432))), 1.0 / (s.v[17])));
        }

        if (!(s.v[622] != 0.0)) {
            s.store_scaled_add(47, 431, 432, 1.0 / ((2.0 * s.v[17])));
        }

        s.v[623] = if (p.p189 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[623] != 0.0) {
            s.store_add_ad(145, A::scale(A::add(s.ad_value(433), s.ad_value(434)), 1.0 / ((2.0 * s.v[19]))), A::scale(A::mul(A::scale(A::sub_from_scalar(1.0, A::limited_exp(A::neg(s.ad_value(34)))), (p.p189 * 0.5)), A::sub(s.ad_value(433), s.ad_value(434))), 1.0 / (s.v[19])));
        }

        if (!(s.v[623] != 0.0)) {
            s.store_scaled_add(145, 433, 434, 1.0 / ((2.0 * s.v[19])));
        }

        s.store_add_ad_lhs(36, A::mul(s.ad_value(114), s.ad_value(47)), 48);

        s.store_scale_ad(37, A::add(s.ad_value(36), A::sqrt(A::offset(A::square(s.ad_value(36)), 0.001))), 0.5);

        s.store_scale(116, 37, s.v[129]);

        s.store_add_ad_lhs(36, A::mul(s.ad_value(143), s.ad_value(145)), 48);

        s.store_scale_ad(37, A::add(s.ad_value(36), A::sqrt(A::offset(A::square(s.ad_value(36)), 0.001))), 0.5);

        s.store_scale(117, 37, s.v[144]);

        s.store_ad(624, &A::pow(A::scale(A::offset(A::abs(A::scale(s.ad_value(46), 1.0 / (s.v[59]))), 1.0), 0.5), s.ad_value(124)));

        s.store_add_ad(625, A::mul(A::add(s.ad_value(122), A::mul(s.ad_value(25), s.ad_value(123))), A::pow(A::abs(s.ad_value(116)), A::add(s.ad_value(336), A::mul(s.ad_value(342), s.ad_value(25))))), A::div(A::add(s.ad_value(125), A::mul(s.ad_value(25), s.ad_value(137))), s.ad_value(624)));

        s.store_offset(119, 625, 1.0);

        s.store_scale_ad(119, A::add(A::offset(s.ad_value(119), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(119), (-1.0)), A::offset(s.ad_value(119), (-1.0))), ((0.25 * p.p154) * p.p154)))), 0.5);

        s.store_scale(119, 119, 1.0 / (p.p11));

        s.store_div(141, 126, 119);

        s.store_ad(626, &A::pow(A::scale(A::offset(A::abs(A::scale(s.ad_value(46), 1.0 / (s.v[59]))), 1.0), 0.5), s.ad_value(348)));

        s.store_add_ad(627, A::mul(A::add(s.ad_value(345), A::mul(s.ad_value(25), s.ad_value(346))), A::pow(A::abs(s.ad_value(117)), A::add(s.ad_value(349), A::mul(s.ad_value(350), s.ad_value(25))))), A::div(A::add(s.ad_value(347), A::mul(s.ad_value(25), s.ad_value(138))), s.ad_value(626)));

        s.store_offset(119, 627, 1.0);

        s.store_scale_ad(119, A::add(A::offset(s.ad_value(119), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(119), (-1.0)), A::offset(s.ad_value(119), (-1.0))), ((0.25 * p.p154) * p.p154)))), 0.5);

        s.store_scale(119, 119, 1.0 / (p.p11));

        s.store_div(142, 344, 119);

        s.store_sub_ad_rhs(34, 71, A::scale(A::add(s.ad_value(431), s.ad_value(432)), 1.0 / ((2.0 * s.v[17]))));

        s.store_sub_ad(35, A::sub(s.ad_value(70), s.ad_value(86)), A::scale(A::add(s.ad_value(433), s.ad_value(434)), 1.0 / ((2.0 * s.v[19]))));

        s.store_div_ad(139, A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::add(A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::exp(A::div(s.ad_value(35), s.ad_value(81)))));

        s.store_div_ad(140, A::exp(A::div(s.ad_value(35), s.ad_value(81))), A::add(A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::exp(A::div(s.ad_value(35), s.ad_value(81)))));

        s.store_add_ad(121, A::mul(s.ad_value(139), s.ad_value(141)), A::mul(s.ad_value(140), s.ad_value(142)));

        s.store_div_ad_lhs(56, A::mul(A::scale(s.ad_value(121), s.v[17]), s.ad_value(3)), 2);

        s.store_scale_ad(118, A::add(s.ad_value(48), A::mul(s.ad_value(115), s.ad_value(46))), s.v[129]);

        s.store_mul_ad_rhs(37, 122, A::pow(A::abs(s.ad_value(118)), s.ad_value(336)));

        s.store_offset(120, 37, 1.0);

        s.store_scale_ad(120, A::add(A::offset(s.ad_value(120), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(120), (-1.0)), A::offset(s.ad_value(120), (-1.0))), ((0.25 * p.p154) * p.p154)))), 0.5);

        s.store_scale(120, 120, 1.0 / (p.p11));

        s.store_div_ad_lhs(173, A::scale(s.ad_value(166), 2.0), 121);

        s.store_mul(174, 173, 2);

        s.store_offset_ad(34, A::mul(s.ad_value(165), s.ad_value(25)), 0.8);

        s.store_offset_ad(181, A::scale(A::add(s.ad_value(34), A::sqrt(A::offset(A::square(s.ad_value(34)), 0.01))), 0.5), 0.2);

        s.store_mul_ad_lhs(34, A::div(s.ad_value(49), s.ad_value(174)), 181);

        s.store_scale_ad(161, A::offset(A::sqrt(A::offset(A::square(s.ad_value(34)), p.p109)), 1.0), 1.0 / ((1.0 + ((p.p109) as f64).sqrt())));

        s.store_add_ad_rhs(161, 161, A::mul(A::mul(A::mul(A::scale(A::sub(A::sub(s.ad_value(182), A::mul(s.ad_value(328), s.ad_value(28))), A::mul(s.ad_value(329), s.ad_value(25))), 0.5), s.ad_value(46)), s.ad_value(49)), s.ad_value(49)));

        s.store_scale_ad(161, A::add(A::offset(s.ad_value(161), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(161), (-1.0)), A::offset(s.ad_value(161), (-1.0))), ((0.25 * p.p134) * p.p134)))), 0.5);

        s.store_div_ad_lhs(171, A::mul(A::scale(s.ad_value(167), 2.0), s.ad_value(120)), 126);

        s.store_mul(172, 171, 1);

        s.v[628] = if (s.v[365] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[628] != 0.0) {
            s.store_offset_ad(154, A::div(A::mul(s.ad_value(365), s.ad_value(46)), s.ad_value(170)), 1.0);
        }

        if (!(s.v[628] != 0.0)) {
            s.store_div_from_scalar_ad(154, 1.0, A::sub_from_scalar(1.0, A::div(A::mul(s.ad_value(365), s.ad_value(46)), s.ad_value(170))));
        }

        s.store_sub(155, 26, 113);

        s.store_add_ad_rhs(157, 46, A::scale(s.ad_value(55), 2.0));

        s.v[629] = if (s.v[153] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[629] != 0.0) {
            s.copy_ad(35, 157);
        }

        if (s.v[629] != 0.0) {
            s.store_div_ad_rhs(37, 35, A::add(s.ad_value(162), s.ad_value(35)));
        }

        if (s.v[629] != 0.0) {
            s.store_mul_ad_lhs(156, A::mul(A::div(s.ad_value(35), s.ad_value(153)), s.ad_value(37)), 154);
        }

        if (s.v[629] != 0.0) {
            s.store_offset_ad(158, A::div(s.ad_value(155), s.ad_value(156)), 1.0);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_scalar(158, 1.0);
        }

        s.v[630] = if (s.v[360] > 0.0) { 1.0 } else { 0.0 };

        s.v[631] = if (p.p213 < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[630] != 0.0) && (s.v[631] != 0.0)) {
            s.store_div_from_scalar_ad(35, 1.0, A::sub(A::div_from_scalar(1.0, s.ad_value(360)), A::scale(s.ad_value(46), p.p213)));
        }

        if ((s.v[630] != 0.0) && (!(s.v[631] != 0.0))) {
            s.store_mul_ad_rhs(35, 360, A::offset(A::scale(s.ad_value(46), p.p213), 1.0));
        }

        if (s.v[630] != 0.0) {
            s.store_offset_ad(159, A::mul(s.ad_value(35), A::ln(A::max_with_scalar(A::offset(A::div(A::div(s.ad_value(155), s.ad_value(35)), A::add(s.ad_value(162), s.ad_value(170))), 1.0), 1e-38))), 1.0);
        }

        if (!(s.v[630] != 0.0)) {
            s.store_scalar(159, 1.0);
        }

        s.store_mul(158, 158, 159);

        s.v[632] = if (s.v[361] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[632] != 0.0) {
            s.store_offset_ad(160, A::mul(s.ad_value(361), A::ln(A::max_with_scalar(A::offset(A::div(A::div(A::sub(s.ad_value(26), s.ad_value(113)), s.ad_value(361)), A::add(s.ad_value(162), s.ad_value(172))), 1.0), 1e-38))), 1.0);
        }

        if (!(s.v[632] != 0.0)) {
            s.store_scalar(160, 1.0);
        }

        s.v[633] = if (s.v[175] != 0.0) { 1.0 } else { 0.0 };

        if (s.v[633] != 0.0) {
            s.store_div_ad_rhs(35, 175, A::add(A::mul(A::max_from_scalar(0.0, A::add(s.ad_value(176), A::mul(A::mul(s.ad_value(108), s.ad_value(49)), s.ad_value(49)))), s.ad_value(46)), A::scale(s.ad_value(81), 2.0)));
        }

        if (s.v[633] != 0.0) {
            s.store_limited_exp_ad(94, A::neg(s.ad_value(35)));
        }

        if (!(s.v[633] != 0.0)) {
            s.store_scalar(94, 1.0);
        }

        s.store_sub(34, 437, 438);

        s.store_sub_ad(35, A::square(s.ad_value(437)), A::square(s.ad_value(438)));

        s.store_add_ad(215, A::mul(A::mul(A::scale(s.ad_value(81), (s.v[20] * 2.0)), s.ad_value(55)), s.ad_value(34)), A::scale(A::mul(A::scale(A::mul(A::scale(s.ad_value(81), (s.v[20] * s.v[20])), s.ad_value(81)), 0.5), s.ad_value(35)), 1.0 / (s.v[17])));

        s.store_add_ad_lhs(216, A::scale(A::add(s.ad_value(109), s.ad_value(110)), 0.5), 55);

        s.v[640] = if (p.p14 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[640] != 0.0) {
            s.store_scalar(151, 0.0);
        }

        if (s.v[640] != 0.0) {
            s.store_scalar(130, 1.0);
        }

        if (s.v[640] != 0.0) {
            s.store_sub(638, 29, 200);
        }

        if (s.v[640] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(638)), 0.0001));
        }

        if (s.v[640] != 0.0) {
            s.store_scaled_add(636, 638, 639, 0.5);
        }

        if (s.v[640] != 0.0) {
            s.store_offset_ad(635, A::mul(s.ad_value(284), s.ad_value(636)), 1.0);
        }

        if (s.v[640] != 0.0) {
            s.store_div_from_scalar(634, 1.0, 635);
        }

        if (s.v[640] != 0.0) {
            s.store_sub_ad_rhs(634, 634, A::mul(A::scale(s.ad_value(32), 0.5), s.ad_value(285)));
        }

        if (s.v[640] != 0.0) {
            s.store_scale_ad(34, A::add(s.ad_value(634), A::sqrt(A::offset(A::square(s.ad_value(634)), 0.01))), 0.5);
        }

        if (s.v[640] != 0.0) {
            s.store_mul_ad_rhs(147, 150, A::add(s.ad_value(132), A::mul(A::add(s.ad_value(136), A::mul(s.ad_value(283), s.ad_value(34))), s.ad_value(131))));
        }

        if (s.v[640] != 0.0) {
            s.store_sub(638, 31, 200);
        }

        if (s.v[640] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(638)), 0.0001));
        }

        if (s.v[640] != 0.0) {
            s.store_scaled_add(637, 638, 639, 0.5);
        }

        if (s.v[640] != 0.0) {
            s.store_offset_ad(635, A::mul(s.ad_value(284), s.ad_value(637)), 1.0);
        }

        if (s.v[640] != 0.0) {
            s.store_div_from_scalar(634, 1.0, 635);
        }

        if (s.v[640] != 0.0) {
            s.store_sub_ad_rhs(634, 634, A::mul(A::scale(s.ad_value(33), 0.5), s.ad_value(285)));
        }

        if (s.v[640] != 0.0) {
            s.store_scale_ad(34, A::add(s.ad_value(634), A::sqrt(A::offset(A::square(s.ad_value(634)), 0.01))), 0.5);
        }

        if (s.v[640] != 0.0) {
            s.store_mul_ad_rhs(146, 150, A::add(s.ad_value(133), A::mul(A::add(s.ad_value(135), A::mul(s.ad_value(282), s.ad_value(34))), s.ad_value(131))));
        }

        if (!(s.v[640] != 0.0)) {
            s.store_offset_ad(635, A::mul(s.ad_value(284), s.ad_value(46)), 1.0);
        }

        if (!(s.v[640] != 0.0)) {
            s.store_div_from_scalar(634, 1.0, 635);
        }

        if (!(s.v[640] != 0.0)) {
            s.store_sub_ad_rhs(634, 634, A::mul(A::scale(A::add(s.ad_value(24), s.ad_value(23)), 0.5), s.ad_value(285)));
        }

        if (!(s.v[640] != 0.0)) {
            s.store_scale_ad(34, A::add(s.ad_value(634), A::sqrt(A::offset(A::square(s.ad_value(634)), 0.01))), 0.5);
        }

        if (!(s.v[640] != 0.0)) {
            s.store_mul_ad_rhs(151, 150, A::mul(A::add(s.ad_value(134), A::mul(s.ad_value(281), s.ad_value(34))), s.ad_value(131)));
        }

        if (!(s.v[640] != 0.0)) {
            s.store_offset_ad(130, A::mul(A::div(A::mul(A::scale(s.ad_value(56), p.p2), s.ad_value(216)), s.ad_value(161)), s.ad_value(151)), 1.0);
        }

        if (!(s.v[640] != 0.0)) {
            s.copy_ad(146, 133);
        }

        if (!(s.v[640] != 0.0)) {
            s.copy_ad(147, 132);
        }

        s.v[641] = if (p.p14 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[640] != 0.0)) && (s.v[641] != 0.0)) {
            s.store_offset_ad(635, A::mul(s.ad_value(284), s.ad_value(46)), 1.0);
        }

        if ((!(s.v[640] != 0.0)) && (s.v[641] != 0.0)) {
            s.store_div_from_scalar(634, 1.0, 635);
        }

        if ((!(s.v[640] != 0.0)) && (s.v[641] != 0.0)) {
            s.store_sub_ad_rhs(634, 634, A::mul(A::scale(A::add(s.ad_value(24), s.ad_value(23)), 0.5), s.ad_value(285)));
        }

        if ((!(s.v[640] != 0.0)) && (s.v[641] != 0.0)) {
            s.store_scale_ad(34, A::add(s.ad_value(634), A::sqrt(A::offset(A::square(s.ad_value(634)), 0.01))), 0.5);
        }

        if ((!(s.v[640] != 0.0)) && (s.v[641] != 0.0)) {
            s.store_mul_ad_lhs(151, A::mul(s.ad_value(150), A::add(A::add(A::add(s.ad_value(132), s.ad_value(133)), s.ad_value(134)), A::mul(s.ad_value(281), s.ad_value(34)))), 131);
        }

        if ((!(s.v[640] != 0.0)) && (s.v[641] != 0.0)) {
            s.store_offset_ad(130, A::mul(A::div(A::mul(A::scale(s.ad_value(56), p.p2), s.ad_value(216)), s.ad_value(161)), s.ad_value(151)), 1.0);
        }

        if ((!(s.v[640] != 0.0)) && (s.v[641] != 0.0)) {
            s.store_scalar(146, 0.0);
        }

        if ((!(s.v[640] != 0.0)) && (s.v[641] != 0.0)) {
            s.store_scalar(147, 0.0);
        }

        s.store_div_ad(214, A::mul(A::mul(A::mul(A::scale(s.ad_value(56), 1.0 / (s.v[17])), s.ad_value(215)), s.ad_value(158)), s.ad_value(94)), A::mul(s.ad_value(161), s.ad_value(130)));

        s.store_scale(214, 214, p.p2);

        s.store_scaled_add(219, 432, 431, 0.5);

        s.store_scale_ad(218, A::add(s.ad_value(435), A::scale(s.ad_value(436), 2.0)), (1.0 / 6.0));

        s.store_scale_ad(217, A::add(A::scale(s.ad_value(435), 2.0), s.ad_value(436)), (1.0 / 6.0));

        s.store_scaled_add(220, 434, 433, 0.5);

        s.v[642] = if (s.v[62] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[642] != 0.0) {
            s.store_div_ad_lhs(38, A::add(s.ad_value(46), A::mul(s.ad_value(66), s.ad_value(48))), 67);
        }

        if (s.v[642] != 0.0) {
            s.store_offset_ad(39, A::pow(s.ad_value(38), s.ad_value(68)), 1.0);
        }

        if (s.v[642] != 0.0) {
            s.store_scalar(63, p.p49);
        }

        if (s.v[642] != 0.0) {
            s.store_div(64, 63, 39);
        }

        if (s.v[642] != 0.0) {
            s.store_div_from_scalar_ad(65, (3.9 * 8.85418e-12), A::add(A::scale(s.ad_value(221), (3.9 * 1.0 / (p.p60))), A::scale(A::mul(s.ad_value(64), s.ad_value(62)), 1.0 / (s.v[21]))));
        }

        if (!(s.v[642] != 0.0)) {
            s.store_scalar(65, s.v[18]);
        }

        s.store_div_ad_lhs(34, A::mul(s.ad_value(4), s.ad_value(1)), 160);

        s.store_mul(219, 219, 34);

        s.store_mul_ad_lhs(218, A::neg(s.ad_value(218)), 34);

        s.store_mul(220, 220, 34);

        s.store_mul_ad_lhs(217, A::neg(s.ad_value(217)), 34);

        s.store_ad(228, &A::mul(A::scale(A::mul(s.ad_value(4), s.ad_value(396)), s.v[17]), A::voltage(ctx, &nodes, Some(7), Some(6))));

        s.store_ad(230, &A::mul(A::scale(A::mul(s.ad_value(4), s.ad_value(397)), s.v[17]), A::voltage(ctx, &nodes, Some(7), Some(5))));

        s.store_mul_ad_rhs(240, 212, A::sub(s.ad_value(288), s.ad_value(99)));

        s.store_add_ad(34, A::offset(A::sub(s.ad_value(235), s.ad_value(200)), 0.02), A::scale(A::offset(A::sub(s.ad_value(32), s.ad_value(240)), (-p.p268)), ((p.p45 / p.p46) * p.p269)));

        s.store_scale_ad(232, A::sub(s.ad_value(34), A::sqrt(A::offset(A::square(s.ad_value(34)), (4.0 * 0.02)))), 0.5);

        s.store_sub_ad_lhs(35, A::sub(s.ad_value(235), s.ad_value(200)), 232);

        s.store_add_ad_rhs(228, 228, A::mul(A::scale(A::mul(s.ad_value(212), s.ad_value(4)), p.p263), A::sub(s.ad_value(35), A::scale(A::offset(A::sqrt(A::sub_from_scalar(1.0, A::scale(s.ad_value(232), (4.0 * 1.0 / (p.p265))))), (-1.0)), (0.5 * p.p265)))));

        s.store_add_ad(34, A::offset(A::sub(s.ad_value(234), s.ad_value(200)), 0.02), A::scale(A::offset(A::sub(s.ad_value(33), s.ad_value(240)), (-p.p270)), ((p.p45 / p.p46) * p.p271)));

        s.store_scale_ad(233, A::sub(s.ad_value(34), A::sqrt(A::offset(A::square(s.ad_value(34)), (4.0 * 0.02)))), 0.5);

        s.store_sub_ad_lhs(35, A::sub(s.ad_value(234), s.ad_value(200)), 233);

        s.store_add_ad_rhs(230, 230, A::mul(A::scale(A::mul(s.ad_value(212), s.ad_value(4)), p.p264), A::sub(s.ad_value(35), A::scale(A::offset(A::sqrt(A::sub_from_scalar(1.0, A::scale(s.ad_value(233), (4.0 * 1.0 / (p.p266))))), (-1.0)), (0.5 * p.p266)))));

        s.store_ad(229, &A::mul(A::mul(s.ad_value(4), s.ad_value(398)), A::voltage(ctx, &nodes, Some(7), Some(6))));

        s.store_ad(231, &A::mul(A::mul(s.ad_value(4), s.ad_value(399)), A::voltage(ctx, &nodes, Some(7), Some(5))));

        s.store_add(226, 228, 229);

        s.store_add(227, 230, 231);

        s.store_ad(238, &A::mul(A::scale(s.ad_value(212), s.v[236]), A::voltage(ctx, &nodes, Some(6), Some(3))));

        s.store_ad(239, &A::mul(A::scale(s.ad_value(212), s.v[237]), A::voltage(ctx, &nodes, Some(5), Some(3))));

        s.store_div_ad_lhs(34, A::add(s.ad_value(366), A::mul(s.ad_value(367), s.ad_value(2))), 2);

        s.v[643] = if ((s.v[34] <= 0.0) || (s.v[103] <= 0.0)) { 1.0 } else { 0.0 };

        if (s.v[643] != 0.0) {
            s.store_scalar(211, 0.0);
        }

        s.v[644] = if (s.v[155] > (s.v[103] / 80.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[643] != 0.0)) && (s.v[644] != 0.0)) {
            s.store_div_ad_lhs(35, A::neg(s.ad_value(103)), 155);
        }

        if ((!(s.v[643] != 0.0)) && (s.v[644] != 0.0)) {
            s.store_mul_ad(211, A::mul(A::mul(s.ad_value(34), s.ad_value(155)), s.ad_value(214)), A::limited_exp(s.ad_value(35)));
        }

        if ((!(s.v[643] != 0.0)) && (!(s.v[644] != 0.0))) {
            s.store_scale_ad(211, A::mul(A::mul(s.ad_value(34), s.ad_value(155)), s.ad_value(214)), 1.804851387e-35);
        }

        s.v[184] = 0.0;

        s.v[192] = 0.0;

        s.v[193] = 0.0;

        s.v[194] = 0.0;

        s.v[201] = 0.0;

        s.v[202] = 0.0;

        s.v[645] = if (p.p17 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[645] != 0.0) {
            s.store_div_ad_lhs(35, A::div(A::sub(s.ad_value(46), s.ad_value(411)), s.ad_value(412)), 55);
        }

        if (s.v[645] != 0.0) {
            s.store_mul_ad(183, A::mul(s.ad_value(412), s.ad_value(55)), A::ln(A::max_with_scalar(A::offset(A::limited_exp(s.ad_value(35)), 1.0), 1e-38)));
        }

        if (s.v[645] != 0.0) {
            s.store_sub_ad_rhs(36, 408, A::mul(s.ad_value(409), s.ad_value(46)));
        }

        if (s.v[645] != 0.0) {
            s.store_offset_ad(37, A::mul(s.ad_value(410), s.ad_value(46)), 1.0);
        }

        if (s.v[645] != 0.0) {
            s.store_mul_ad_lhs(38, A::scale(s.ad_value(36), ((-982222000000.0) * p.p99)), 37);
        }

        if (s.v[645] != 0.0) {
            s.store_ad(39, &A::limited_exp(s.ad_value(38)));
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
        if (s.v[645] != 0.0) {
            s.store_scalar(40, 3.75956e-7);
        }

        if (s.v[645] != 0.0) {
            s.store_mul_ad_lhs(184, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(3), s.ad_value(2)), s.ad_value(40)), s.ad_value(207)), s.ad_value(209)), s.ad_value(183)), 39);
        }

        if (s.v[645] != 0.0) {
            s.store_mul(184, 184, 106);
        }

        if (s.v[645] != 0.0) {
            s.store_sub(191, 52, 50);
        }

        if (s.v[645] != 0.0) {
            s.store_sub(34, 191, 209);
        }

        if (s.v[645] != 0.0) {
            s.store_div_ad_lhs(35, A::div(s.ad_value(34), s.ad_value(416)), 55);
        }

        if (s.v[645] != 0.0) {
            s.store_mul_ad(190, A::mul(s.ad_value(416), s.ad_value(55)), A::ln(A::max_with_scalar(A::offset(A::limited_exp(s.ad_value(35)), 1.0), 1e-38)));
        }

        s.v[646] = if (s.v[191] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[645] != 0.0) && (s.v[646] != 0.0)) {
            s.store_scale_ad(189, A::add(A::offset(s.ad_value(34), (-0.02)), A::sqrt(A::sub(A::mul(A::offset(s.ad_value(34), (-0.02)), A::offset(s.ad_value(34), (-0.02))), A::scale(s.ad_value(191), 0.08)))), 0.5);
        }

        if ((s.v[645] != 0.0) && (!(s.v[646] != 0.0))) {
            s.store_scale_ad(189, A::add(A::offset(s.ad_value(34), (-0.02)), A::sqrt(A::add(A::mul(A::offset(s.ad_value(34), (-0.02)), A::offset(s.ad_value(34), (-0.02))), A::scale(s.ad_value(191), 0.08)))), 0.5);
        }

        if (s.v[645] != 0.0) {
            s.store_sub_ad_rhs(36, 413, A::mul(s.ad_value(414), s.ad_value(189)));
        }

        if (s.v[645] != 0.0) {
            s.store_offset_ad(37, A::mul(s.ad_value(415), s.ad_value(189)), 1.0);
        }

        if (s.v[645] != 0.0) {
            s.store_mul_ad_lhs(38, A::scale(s.ad_value(36), ((-745669000000.0) * p.p99)), 37);
        }

        if (s.v[645] != 0.0) {
            s.store_ad(39, &A::limited_exp(s.ad_value(38)));
        }

        if (s.v[645] != 0.0) {
            s.store_scalar(40, 4.97232e-7);
        }

        if (s.v[645] != 0.0) {
            s.store_mul_ad_lhs(192, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(3), s.ad_value(2)), s.ad_value(40)), s.ad_value(207)), s.ad_value(209)), s.ad_value(190)), 39);
        }

        if (s.v[645] != 0.0) {
            s.store_mul(192, 192, 106);
        }

        s.store_tanh_ad(34, A::div(A::scale(s.ad_value(30), 0.6), s.ad_value(55)));

        s.store_offset_scaled(57, 34, 0.5, 0.5);

        s.store_sub_from_scalar(58, 1.0, 57);

        s.store_mul_ad_rhs(187, 57, A::add(s.ad_value(184), s.ad_value(192)));

        s.store_mul_ad_rhs(188, 58, A::add(s.ad_value(184), s.ad_value(192)));

        s.v[647] = if (p.p16 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[647] != 0.0) {
            s.store_sub_ad_rhs(35, 369, A::mul(s.ad_value(370), A::sub(s.ad_value(69), A::mul(s.ad_value(373), s.ad_value(210)))));
        }

        if (s.v[647] != 0.0) {
            s.store_offset_ad(36, A::mul(s.ad_value(371), A::sub(s.ad_value(69), A::mul(s.ad_value(373), s.ad_value(210)))), 1.0);
        }

        if (s.v[647] != 0.0) {
            s.store_mul_ad_lhs(37, A::mul(A::scale(A::neg(s.ad_value(206)), p.p99), s.ad_value(35)), 36);
        }

        if (s.v[647] != 0.0) {
            s.store_mul_ad_rhs(38, 46, A::limited_exp(s.ad_value(37)));
        }

        if (s.v[647] != 0.0) {
            s.store_add_ad(39, A::add(s.ad_value(209), A::scale(s.ad_value(73), 0.5)), A::scale(A::add(s.ad_value(32), s.ad_value(33)), 0.5));
        }

        if (s.v[647] != 0.0) {
            s.store_mul_ad_lhs(195, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(3), s.ad_value(2)), s.ad_value(205)), s.ad_value(207)), s.ad_value(38)), s.ad_value(39)), 106);
        }

        if (s.v[647] != 0.0) {
            s.store_offset_ad(196, A::sqrt(A::offset(A::square(s.ad_value(113)), 0.01)), (-0.1));
        }

        if (s.v[647] != 0.0) {
            s.store_mul(35, 372, 196);
        }

        if (s.v[647] != 0.0) {
            s.store_limited_exp_ad(197, A::neg(s.ad_value(35)));
        }

        if (s.v[647] != 0.0) {
            s.store_offset_ad(37, A::offset(A::add(s.ad_value(35), s.ad_value(197)), (-1.0)), 0.0001);
        }

        if (s.v[647] != 0.0) {
            s.store_offset_ad(38, A::sub_from_scalar(1.0, A::mul(A::offset(s.ad_value(35), 1.0), s.ad_value(197))), 0.0001);
        }

        if (s.v[647] != 0.0) {
            s.store_offset_ad(39, A::square(s.ad_value(35)), 0.0002);
        }

        if (s.v[647] != 0.0) {
            s.store_div_ad_lhs(194, A::mul(s.ad_value(195), s.ad_value(38)), 39);
        }

        if (s.v[647] != 0.0) {
            s.store_div_ad_lhs(193, A::mul(s.ad_value(195), s.ad_value(37)), 39);
        }

        if (s.v[647] != 0.0) {
            s.store_add_ad(34, A::sub(s.ad_value(29), s.ad_value(200)), A::mul(A::scale(s.ad_value(385), s.v[243]), A::sub(s.ad_value(23), s.ad_value(240))));
        }

        if (s.v[647] != 0.0) {
            s.store_sqrt_ad(203, A::offset(A::square(s.ad_value(34)), 0.0001));
        }

        if (s.v[647] != 0.0) {
            s.store_sub_ad_rhs(35, 382, A::mul(s.ad_value(383), s.ad_value(203)));
        }

        if (s.v[647] != 0.0) {
            s.store_offset_ad(36, A::mul(s.ad_value(384), s.ad_value(203)), 1.0);
        }

        if (s.v[647] != 0.0) {
            s.store_mul_ad_lhs(37, A::mul(A::mul(A::scale(A::neg(s.ad_value(206)), p.p99), s.ad_value(394)), s.ad_value(35)), 36);
        }

        if (s.v[647] != 0.0) {
            s.store_ad(38, &A::limited_exp(s.ad_value(37)));
        }

        s.v[648] = if (s.v[27] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[647] != 0.0) && (s.v[648] != 0.0)) {
            s.store_mul_ad_lhs(201, A::mul(A::mul(A::scale(s.ad_value(185), p.p234), s.ad_value(29)), s.ad_value(203)), 38);
        }

        if ((s.v[647] != 0.0) && (!(s.v[648] != 0.0))) {
            s.store_mul_ad_lhs(202, A::mul(A::mul(A::scale(s.ad_value(185), p.p234), s.ad_value(29)), s.ad_value(203)), 38);
        }

        if (s.v[647] != 0.0) {
            s.store_add_ad(34, A::sub(s.ad_value(31), s.ad_value(200)), A::mul(A::scale(s.ad_value(389), s.v[243]), A::sub(s.ad_value(23), s.ad_value(240))));
        }

        if (s.v[647] != 0.0) {
            s.store_sqrt_ad(204, A::offset(A::square(s.ad_value(34)), 0.0001));
        }

        if (s.v[647] != 0.0) {
            s.store_sub_ad_rhs(35, 386, A::mul(s.ad_value(387), s.ad_value(204)));
        }

        if (s.v[647] != 0.0) {
            s.store_offset_ad(36, A::mul(s.ad_value(388), s.ad_value(204)), 1.0);
        }

        if (s.v[647] != 0.0) {
            s.store_mul_ad_lhs(37, A::mul(A::mul(A::scale(A::neg(s.ad_value(206)), p.p99), s.ad_value(394)), s.ad_value(35)), 36);
        }

        if (s.v[647] != 0.0) {
            s.store_ad(38, &A::limited_exp(s.ad_value(37)));
        }

        s.v[649] = if (s.v[27] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[647] != 0.0) && (s.v[649] != 0.0)) {
            s.store_mul_ad_lhs(202, A::mul(A::mul(A::scale(s.ad_value(185), p.p235), s.ad_value(31)), s.ad_value(204)), 38);
        }

        if ((s.v[647] != 0.0) && (!(s.v[649] != 0.0))) {
            s.store_mul_ad_lhs(201, A::mul(A::mul(A::scale(s.ad_value(185), p.p235), s.ad_value(31)), s.ad_value(204)), 38);
        }

        s.v[198] = 0.0;

        s.v[199] = 0.0;

        s.v[650] = if (p.p15 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[650] != 0.0) {
            s.store_scalar(34, (s.v[21] * p.p45));
        }

        s.v[651] = if ((s.v[378] <= 0.0) || (s.v[104] <= 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[650] != 0.0) && (s.v[651] != 0.0)) {
            s.store_scalar(40, 0.0);
        }

        if ((s.v[650] != 0.0) && (!(s.v[651] != 0.0))) {
            s.store_div_ad_lhs(35, A::add(A::add(A::sub(A::neg(s.ad_value(31)), s.ad_value(380)), s.ad_value(200)), A::mul(A::scale(s.ad_value(390), s.v[243]), A::sub(A::sub(s.ad_value(23), s.ad_value(240)), s.ad_value(391)))), 34);
        }

        if ((s.v[650] != 0.0) && (!(s.v[651] != 0.0))) {
            s.store_scale_ad(35, A::add(s.ad_value(35), A::sqrt(A::offset(A::square(s.ad_value(35)), ((4.0 * 0.01) * 0.01)))), 0.5);
        }

        if ((s.v[650] != 0.0) && (!(s.v[651] != 0.0))) {
            s.store_div_ad_rhs(36, 104, A::offset(s.ad_value(35), 0.001));
        }

        if ((s.v[650] != 0.0) && (!(s.v[651] != 0.0))) {
            s.store_limited_exp_ad(37, A::mul(s.ad_value(381), A::ln(A::max_with_scalar(s.ad_value(35), 1e-38))));
        }

        if ((s.v[650] != 0.0) && (!(s.v[651] != 0.0))) {
            s.store_mul_ad_lhs(40, A::mul(A::mul(A::mul(s.ad_value(378), s.ad_value(3)), s.ad_value(37)), A::limited_exp(A::neg(s.ad_value(36)))), 30);
        }

        s.v[652] = if (s.v[27] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[650] != 0.0) && (s.v[652] != 0.0)) {
            s.copy_ad(199, 40);
        }

        if ((s.v[650] != 0.0) && (!(s.v[652] != 0.0))) {
            s.copy_ad(198, 40);
        }

        s.v[653] = if ((s.v[374] <= 0.0) || (s.v[105] <= 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[650] != 0.0) && (s.v[653] != 0.0)) {
            s.store_scalar(40, 0.0);
        }

        if ((s.v[650] != 0.0) && (!(s.v[653] != 0.0))) {
            s.store_div_ad_lhs(35, A::add(A::add(A::sub(A::neg(s.ad_value(29)), s.ad_value(376)), s.ad_value(200)), A::mul(A::scale(s.ad_value(392), s.v[243]), A::sub(A::sub(s.ad_value(23), s.ad_value(240)), s.ad_value(393)))), 34);
        }

        if ((s.v[650] != 0.0) && (!(s.v[653] != 0.0))) {
            s.store_scale_ad(35, A::add(s.ad_value(35), A::sqrt(A::offset(A::square(s.ad_value(35)), ((4.0 * 0.01) * 0.01)))), 0.5);
        }

        if ((s.v[650] != 0.0) && (!(s.v[653] != 0.0))) {
            s.store_div_ad_rhs(36, 105, A::offset(s.ad_value(35), 0.001));
        }

        if ((s.v[650] != 0.0) && (!(s.v[653] != 0.0))) {
            s.store_limited_exp_ad(37, A::mul(s.ad_value(377), A::ln(A::max_with_scalar(s.ad_value(35), 1e-38))));
        }

        if ((s.v[650] != 0.0) && (!(s.v[653] != 0.0))) {
            s.store_mul_ad(40, A::mul(A::mul(A::mul(A::neg(s.ad_value(30)), s.ad_value(374)), s.ad_value(3)), s.ad_value(37)), A::limited_exp(A::neg(s.ad_value(36))));
        }

        s.v[654] = if (s.v[27] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[650] != 0.0) && (s.v[654] != 0.0)) {
            s.copy_ad(198, 40);
        }

        if ((s.v[650] != 0.0) && (!(s.v[654] != 0.0))) {
            s.copy_ad(199, 40);
        }

        s.store_div_ad_lhs(254, A::scale(s.ad_value(164), 2.0), 121);

        s.v[655] = if (((p.p288 > 0.0) || (p.p289 > 0.0)) || (p.p290 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[655] != 0.0) {
            s.store_sub_ad_rhs(255, 2, A::scale(s.ad_value(249), 2.0));
        }

        if (s.v[655] != 0.0) {
            s.store_square(256, 255);
        }

        s.v[656] = if (p.p287 <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[655] != 0.0) && (s.v[656] != 0.0)) {
            s.store_scalar(257, 0.0);
        }

        if ((s.v[655] != 0.0) && (!(s.v[656] != 0.0))) {
            s.store_div_ad_lhs(34, A::offset(A::scale(s.ad_value(155), 1.0 / (s.v[253])), p.p287), 254);
        }

        if ((s.v[655] != 0.0) && (!(s.v[656] != 0.0))) {
            s.store_scale_ad(257, A::ln(A::max_with_scalar(s.ad_value(34), 1e-38)), s.v[253]);
        }

        s.v[657] = if (s.v[257] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[655] != 0.0) && (!(s.v[656] != 0.0))) && (s.v[657] != 0.0)) {
            s.store_scalar(257, 0.0);
        }

        s.v[658] = if (p.p22 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[655] != 0.0) && (s.v[658] != 0.0)) {
            s.store_div(35, 47, 252);
        }

        if ((s.v[655] != 0.0) && (s.v[658] != 0.0)) {
            s.store_offset_ad(36, A::pow(s.ad_value(35), s.ad_value(251)), 1.0);
        }

        if ((s.v[655] != 0.0) && (s.v[658] != 0.0)) {
            s.store_div(37, 250, 36);
        }

        if ((s.v[655] != 0.0) && (s.v[658] != 0.0)) {
            s.store_scale(38, 37, 1.0 / (p.p288));
        }

        if ((s.v[655] != 0.0) && (s.v[658] != 0.0)) {
            s.store_scale_ad(39, A::add(A::offset(s.ad_value(38), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(38), (-1.0)), A::offset(s.ad_value(38), (-1.0))), ((0.25 * p.p292) * p.p292)))), 0.5);
        }

        if ((s.v[655] != 0.0) && (s.v[658] != 0.0)) {
            s.store_scale(258, 39, p.p288);
        }

        if ((s.v[655] != 0.0) && (!(s.v[658] != 0.0))) {
            s.store_scalar(258, p.p288);
        }

        if (s.v[655] != 0.0) {
            s.store_mul_ad_lhs(35, A::mul(A::scale(s.ad_value(55), ((1.60219e-19 * 1.60219e-19) * 1.60219e-19)), A::abs(s.ad_value(214))), 121);
        }

        if (s.v[655] != 0.0) {
            s.store_mul_ad_lhs(36, A::scale(s.ad_value(65), 10000000000.0), 256);
        }

        if (s.v[655] != 0.0) {
            s.store_scaled_mul(259, 65, 109, 6.241457005723417e18);
        }

        if (s.v[655] != 0.0) {
            s.store_scaled_mul(260, 65, 110, 6.241457005723417e18);
        }

        if (s.v[655] != 0.0) {
            s.store_mul_ad(261, A::scale(s.ad_value(55), 6.241457005723417e18), A::add(s.ad_value(65), s.ad_value(291)));
        }

        if (s.v[655] != 0.0) {
            s.store_mul_ad_rhs(37, 258, A::ln(A::max_with_scalar(A::div(A::add(s.ad_value(259), s.ad_value(261)), A::add(s.ad_value(260), s.ad_value(261))), 1e-38)));
        }

        if (s.v[655] != 0.0) {
            s.store_scaled_sub(38, 259, 260, p.p289);
        }

        if (s.v[655] != 0.0) {
            s.store_scale_ad(39, A::sub(A::square(s.ad_value(259)), A::square(s.ad_value(260))), (0.5 * p.p290));
        }

        if (s.v[655] != 0.0) {
            s.store_mul_ad_lhs(40, A::mul(A::scale(s.ad_value(55), 1.60219e-19), s.ad_value(214)), 214);
        }

        if (s.v[655] != 0.0) {
            s.store_scale_ad(41, A::mul(A::scale(s.ad_value(256), 10000000000.0), s.ad_value(3)), p.p2);
        }

        if (s.v[655] != 0.0) {
            s.store_add_ad(42, A::add(s.ad_value(258), A::scale(s.ad_value(260), p.p289)), A::mul(A::scale(s.ad_value(260), p.p290), s.ad_value(260)));
        }

        if (s.v[655] != 0.0) {
            s.store_mul_ad(43, A::add(s.ad_value(260), s.ad_value(261)), A::add(s.ad_value(260), s.ad_value(261)));
        }

        if (s.v[655] != 0.0) {
            s.store_add_ad(262, A::mul(A::div(s.ad_value(35), s.ad_value(36)), A::add(A::add(s.ad_value(37), s.ad_value(38)), s.ad_value(39))), A::div(A::mul(A::mul(A::div(s.ad_value(40), s.ad_value(41)), s.ad_value(257)), s.ad_value(42)), s.ad_value(43)));
        }

        if (s.v[655] != 0.0) {
            s.store_mul_ad_lhs(44, A::scale(s.ad_value(258), 1.60219e-19), 55);
        }

        if (s.v[655] != 0.0) {
            s.store_mul_ad_lhs(45, A::mul(A::scale(A::mul(A::scale(s.ad_value(3), p.p2), s.ad_value(255)), 10000000000.0), s.ad_value(261)), 261);
        }

        if (s.v[655] != 0.0) {
            s.store_mul_ad_lhs(263, A::mul(A::div(s.ad_value(44), s.ad_value(45)), s.ad_value(214)), 214);
        }

        if (s.v[655] != 0.0) {
            s.store_add(35, 263, 262);
        }

        s.v[659] = if (s.v[35] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[655] != 0.0) && (s.v[659] != 0.0)) {
            s.store_div_ad_lhs(264, A::mul(s.ad_value(262), s.ad_value(263)), 35);
        }

        if ((s.v[655] != 0.0) && (!(s.v[659] != 0.0))) {
            s.store_scalar(264, 0.0);
        }

        if (!(s.v[655] != 0.0)) {
            s.store_scalar(264, 0.0);
        }

        s.store_mul_ad_lhs(224, A::scale(s.ad_value(212), p.p2), 219);

        s.store_scale(225, 220, p.p2);

        s.v[660] = if (s.v[27] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[660] != 0.0) {
            s.store_scale(222, 217, p.p2);
        }

        if (s.v[660] != 0.0) {
            s.store_scale(223, 218, p.p2);
        }

        if (s.v[660] != 0.0) {
            s.store_add_ad_lhs(217, A::scale(A::sub(s.ad_value(217), s.ad_value(226)), p.p2), 238);
        }

        if (s.v[660] != 0.0) {
            s.store_add_ad_lhs(218, A::scale(A::sub(s.ad_value(218), s.ad_value(227)), p.p2), 239);
        }

        if (!(s.v[660] != 0.0)) {
            s.store_scale(222, 218, p.p2);
        }

        if (!(s.v[660] != 0.0)) {
            s.store_scale(223, 217, p.p2);
        }

        if (!(s.v[660] != 0.0)) {
            s.store_add_ad_lhs(34, A::scale(A::sub(s.ad_value(218), s.ad_value(226)), p.p2), 238);
        }

        if (!(s.v[660] != 0.0)) {
            s.store_add_ad_lhs(218, A::scale(A::sub(s.ad_value(217), s.ad_value(227)), p.p2), 239);
        }

        if (!(s.v[660] != 0.0)) {
            s.copy_ad(217, 34);
        }

        s.store_add_ad_rhs(219, 224, A::scale(A::add(s.ad_value(226), s.ad_value(227)), p.p2));

        s.store_sub_ad_lhs(220, A::sub(A::scale(s.ad_value(220), p.p2), s.ad_value(238)), 239);

        s.store_scale(226, 226, p.p2);

        s.store_scale(227, 227, p.p2);

        s.store_neg_ad(265, A::add(s.ad_value(222), s.ad_value(223)));

        s.store_mul(34, 121, 265);

        s.store_add_ad(35, A::mul(s.ad_value(34), s.ad_value(151)), A::square(s.ad_value(2)));

        s.store_scaled_div(266, 34, 35, p.p295);

        s.store_scale(268, 55, (4.0 * 1.60219e-19));

        s.store_mul(267, 268, 266);

        s.v[661] = if ((p.p20 == 1.0) && (s.v[275] != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[661] != 0.0) {
            s.store_div_ad_lhs(34, A::mul(A::scale(s.ad_value(121), s.v[17]), s.ad_value(3)), 2);
        }

        if (s.v[661] != 0.0) {
            s.store_mul(277, 34, 46);
        }

        if (s.v[661] != 0.0) {
            s.store_mul_ad(274, A::scale(s.ad_value(275), p.p2), A::add(s.ad_value(277), A::mul(A::mul(s.ad_value(276), s.ad_value(55)), s.ad_value(34))));
        }

        if (!(s.v[661] != 0.0)) {
            s.store_scalar(274, 0.0);
        }

        s.store_scale(199, 199, p.p2);

        s.store_scale(198, 198, p.p2);

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
        s.store_scale(194, 194, p.p2);

        s.store_scale(193, 193, p.p2);

        s.store_scale(201, 201, p.p2);

        s.store_scale(202, 202, p.p2);

        s.v[662] = if (s.v[27] > 0.0) { 1.0 } else { 0.0 };

        s.v[663] = if (p.p14 == 2.0) { 1.0 } else { 0.0 };

        if (!(s.v[663] != 0.0)) {
            s.store_div_from_scalar(149, 1.0, 146);
        }

        if (!(s.v[663] != 0.0)) {
            s.store_div_from_scalar(148, 1.0, 147);
        }

        s.v[664] = if ((p.p20 == 1.0) && (s.v[275] != 0.0)) { 1.0 } else { 0.0 };

        s.v[665] = if (p.p19 == 0.0) { 1.0 } else { 0.0 };

        if (!(s.v[665] != 0.0)) {
            s.copy_ad(666, 273);
        }

        if (!(s.v[665] != 0.0)) {
            s.copy_ad(667, 273);
        }

        s.v[668] = if (p.p16 != 0.0) { 1.0 } else { 0.0 };

        s.v[669] = if (s.v[27] > 0.0) { 1.0 } else { 0.0 };

        s.v[670] = if (p.p17 != 0.0) { 1.0 } else { 0.0 };

        s.v[671] = if ((p.p18 != 0.0) && (p.p310 > 0.0)) { 1.0 } else { 0.0 };

        s.v[672] = if (p.p14 != 2.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[527] = if (p.p12 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[527] != 0.0) {
            s.store_scalar(212, 1.0);
        }

        if (!(s.v[527] != 0.0)) {
            s.store_scalar(212, (-1.0));
        }

        s.v[528] = if (p.p13 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[528] != 0.0) {
            s.store_scalar(213, 1.0);
        }

        if (!(s.v[528] != 0.0)) {
            s.store_scalar(213, (-1.0));
        }

        s.v[16] = (p.p59 * 8.85418e-12);

        s.v[529] = if (p.p21 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[529] != 0.0) {
            s.store_scalar(5, (p.p1 / p.p2));
        }

        if (!(s.v[529] != 0.0)) {
            s.store_scalar(5, p.p1);
        }

        s.v[0] = (p.p0 + p.p23);

        s.store_offset(5, 5, p.p24);

        s.v[6] = ((s.v[0]) as f64).powf((-p.p29));

        s.store_powf(7, 5, (-p.p30));

        s.store_scale(8, 7, s.v[6]);

        s.store_add_ad(9, A::offset(A::scale(s.ad_value(7), p.p27), (p.p25 + (p.p26 * s.v[6]))), A::scale(s.ad_value(8), p.p28));

        s.v[10] = ((s.v[0]) as f64).powf((-p.p35));

        s.store_powf(11, 5, (-p.p36));

        s.store_scale(12, 11, s.v[10]);

        s.store_add_ad(13, A::offset(A::scale(s.ad_value(11), p.p33), (p.p31 + (p.p32 * s.v[10]))), A::scale(s.ad_value(12), p.p34));

        s.store_sub_from_scalar_ad(2, s.v[0], A::scale(s.ad_value(9), 2.0));

        s.store_sub_ad_rhs(3, 5, A::scale(s.ad_value(13), 2.0));

        s.store_add_ad(14, A::offset(A::scale(s.ad_value(7), p.p39), (p.p37 + (p.p38 * s.v[6]))), A::scale(s.ad_value(8), p.p40));

        s.store_add_ad(15, A::offset(A::scale(s.ad_value(11), p.p43), (p.p41 + (p.p42 * s.v[10]))), A::scale(s.ad_value(12), p.p44));

        s.store_sub_from_scalar_ad(1, s.v[0], A::scale(s.ad_value(14), 2.0));

        s.store_sub_ad_rhs(4, 5, A::scale(s.ad_value(15), 2.0));

        s.store_div_from_scalar(278, 1e-6, 2);

        s.store_div_from_scalar(279, 1e-6, 3);

        s.store_mul(280, 278, 279);

        s.store_add_ad(281, A::add(A::offset(A::scale(s.ad_value(278), p.p319), p.p191), A::scale(s.ad_value(279), p.p320)), A::scale(s.ad_value(280), p.p321));

        s.store_add_ad(284, A::add(A::offset(A::scale(s.ad_value(278), p.p328), p.p202), A::scale(s.ad_value(279), p.p329)), A::scale(s.ad_value(280), p.p330));

        s.store_add_ad(285, A::add(A::offset(A::scale(s.ad_value(278), p.p331), p.p203), A::scale(s.ad_value(279), p.p332)), A::scale(s.ad_value(280), p.p333));

        s.store_add_ad(286, A::add(A::offset(A::scale(s.ad_value(278), p.p334), p.p204), A::scale(s.ad_value(279), p.p335)), A::scale(s.ad_value(280), p.p336));

        s.store_add_ad(287, A::add(A::offset(A::scale(s.ad_value(278), p.p337), p.p57), A::scale(s.ad_value(279), p.p338)), A::scale(s.ad_value(280), p.p339));

        s.store_add_ad(288, A::add(A::offset(A::scale(s.ad_value(278), p.p340), p.p58), A::scale(s.ad_value(279), p.p341)), A::scale(s.ad_value(280), p.p342));

        s.store_add_ad(289, A::add(A::offset(A::scale(s.ad_value(278), p.p343), p.p51), A::scale(s.ad_value(279), p.p344)), A::scale(s.ad_value(280), p.p345));

        s.store_add_ad(290, A::add(A::offset(A::scale(s.ad_value(278), p.p346), p.p50), A::scale(s.ad_value(279), p.p347)), A::scale(s.ad_value(280), p.p348));

        s.store_add_ad(291, A::add(A::offset(A::scale(s.ad_value(278), p.p349), p.p63), A::scale(s.ad_value(279), p.p350)), A::scale(s.ad_value(280), p.p351));

        s.store_add_ad(292, A::add(A::offset(A::scale(s.ad_value(278), p.p352), p.p64), A::scale(s.ad_value(279), p.p353)), A::scale(s.ad_value(280), p.p354));

        s.store_add_ad(293, A::add(A::offset(A::scale(s.ad_value(278), p.p355), p.p65), A::scale(s.ad_value(279), p.p356)), A::scale(s.ad_value(280), p.p357));

        s.store_add_ad(294, A::add(A::offset(A::scale(s.ad_value(278), p.p358), p.p68), A::scale(s.ad_value(279), p.p359)), A::scale(s.ad_value(280), p.p360));

        s.store_add_ad(295, A::add(A::offset(A::scale(s.ad_value(278), p.p361), p.p276), A::scale(s.ad_value(279), p.p362)), A::scale(s.ad_value(280), p.p363));

        s.store_add_ad(250, A::add(A::offset(A::scale(s.ad_value(278), p.p751), p.p291), A::scale(s.ad_value(279), p.p752)), A::scale(s.ad_value(280), p.p753));

        s.store_add_ad(252, A::add(A::offset(A::scale(s.ad_value(278), p.p757), p.p294), A::scale(s.ad_value(279), p.p758)), A::scale(s.ad_value(280), p.p759));

        s.store_add_ad(251, A::add(A::offset(A::scale(s.ad_value(278), p.p754), p.p293), A::scale(s.ad_value(279), p.p755)), A::scale(s.ad_value(280), p.p756));

        s.v[538] = if (s.v[295] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[538] != 0.0) {
            s.store_scalar(295, 0.0);
        }

        s.v[539] = if (s.v[295] > 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[538] != 0.0)) && (s.v[539] != 0.0)) {
            s.store_scalar(295, 1.0);
        }

        s.store_add_ad(296, A::add(A::offset(A::scale(s.ad_value(278), p.p364), p.p277), A::scale(s.ad_value(279), p.p365)), A::scale(s.ad_value(280), p.p366));

        s.store_add_ad(297, A::add(A::offset(A::scale(s.ad_value(278), p.p367), p.p278), A::scale(s.ad_value(279), p.p368)), A::scale(s.ad_value(280), p.p369));

        s.store_add_ad(298, A::add(A::offset(A::scale(s.ad_value(278), p.p370), p.p275), A::scale(s.ad_value(279), p.p371)), A::scale(s.ad_value(280), p.p372));

        s.store_add_ad(299, A::add(A::offset(A::scale(s.ad_value(278), p.p373), p.p272), A::scale(s.ad_value(279), p.p374)), A::scale(s.ad_value(280), p.p375));

        s.store_add_ad(300, A::add(A::offset(A::scale(s.ad_value(278), p.p376), p.p273), A::scale(s.ad_value(279), p.p377)), A::scale(s.ad_value(280), p.p378));

        s.store_add_ad(301, A::add(A::offset(A::scale(s.ad_value(278), p.p379), p.p274), A::scale(s.ad_value(279), p.p380)), A::scale(s.ad_value(280), p.p381));

        s.store_add_ad(302, A::add(A::offset(A::scale(s.ad_value(278), p.p382), p.p283), A::scale(s.ad_value(279), p.p383)), A::scale(s.ad_value(280), p.p384));

        s.v[540] = if (s.v[302] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[540] != 0.0) {
            s.store_scalar(302, 0.0);
        }

        s.v[541] = if (s.v[302] > 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[540] != 0.0)) && (s.v[541] != 0.0)) {
            s.store_scalar(302, 1.0);
        }

        s.store_add_ad(303, A::add(A::offset(A::scale(s.ad_value(278), p.p385), p.p284), A::scale(s.ad_value(279), p.p386)), A::scale(s.ad_value(280), p.p387));

        s.store_add_ad(304, A::add(A::offset(A::scale(s.ad_value(278), p.p388), p.p285), A::scale(s.ad_value(279), p.p389)), A::scale(s.ad_value(280), p.p390));

        s.store_add_ad(305, A::add(A::offset(A::scale(s.ad_value(278), p.p391), p.p282), A::scale(s.ad_value(279), p.p392)), A::scale(s.ad_value(280), p.p393));

        s.store_add_ad(306, A::add(A::offset(A::scale(s.ad_value(278), p.p394), p.p279), A::scale(s.ad_value(279), p.p395)), A::scale(s.ad_value(280), p.p396));

        s.store_add_ad(307, A::add(A::offset(A::scale(s.ad_value(278), p.p397), p.p280), A::scale(s.ad_value(279), p.p398)), A::scale(s.ad_value(280), p.p399));

        s.store_add_ad(308, A::add(A::offset(A::scale(s.ad_value(278), p.p400), p.p281), A::scale(s.ad_value(279), p.p401)), A::scale(s.ad_value(280), p.p402));

        s.store_add_ad(313, A::add(A::offset(A::scale(s.ad_value(278), p.p403), p.p71), A::scale(s.ad_value(279), p.p404)), A::scale(s.ad_value(280), p.p405));

        s.store_add_ad(314, A::add(A::offset(A::scale(s.ad_value(278), p.p406), p.p72), A::scale(s.ad_value(279), p.p407)), A::scale(s.ad_value(280), p.p408));

        s.store_add_ad(315, A::add(A::offset(A::scale(s.ad_value(278), p.p409), p.p73), A::scale(s.ad_value(279), p.p410)), A::scale(s.ad_value(280), p.p411));

        s.store_add_ad(316, A::add(A::offset(A::scale(s.ad_value(278), p.p412), p.p74), A::scale(s.ad_value(279), p.p413)), A::scale(s.ad_value(280), p.p414));

        s.store_add_ad(317, A::add(A::offset(A::scale(s.ad_value(278), p.p415), p.p75), A::scale(s.ad_value(279), p.p416)), A::scale(s.ad_value(280), p.p417));

        s.store_add_ad(318, A::add(A::offset(A::scale(s.ad_value(278), p.p418), p.p84), A::scale(s.ad_value(279), p.p419)), A::scale(s.ad_value(280), p.p420));

        s.store_add_ad(319, A::add(A::offset(A::scale(s.ad_value(278), p.p421), p.p76), A::scale(s.ad_value(279), p.p422)), A::scale(s.ad_value(280), p.p423));

        s.store_add_ad(309, A::add(A::offset(A::scale(s.ad_value(278), p.p430), p.p87), A::scale(s.ad_value(279), p.p431)), A::scale(s.ad_value(280), p.p432));

        s.store_add_ad(310, A::add(A::offset(A::scale(s.ad_value(278), p.p433), p.p88), A::scale(s.ad_value(279), p.p434)), A::scale(s.ad_value(280), p.p435));

        s.store_add_ad(311, A::add(A::offset(A::scale(s.ad_value(278), p.p436), p.p61), A::scale(s.ad_value(279), p.p437)), A::scale(s.ad_value(280), p.p438));

        s.store_add_ad(312, A::add(A::offset(A::scale(s.ad_value(278), p.p439), p.p62), A::scale(s.ad_value(279), p.p440)), A::scale(s.ad_value(280), p.p441));

        s.store_add_ad(320, A::add(A::offset(A::scale(s.ad_value(278), p.p424), p.p85), A::scale(s.ad_value(279), p.p425)), A::scale(s.ad_value(280), p.p426));

        s.store_add_ad(321, A::add(A::offset(A::scale(s.ad_value(278), p.p427), p.p86), A::scale(s.ad_value(279), p.p428)), A::scale(s.ad_value(280), p.p429));

        s.store_add_ad(326, A::add(A::offset(A::scale(s.ad_value(278), p.p460), p.p113), A::scale(s.ad_value(279), p.p461)), A::scale(s.ad_value(280), p.p462));

        s.store_add_ad(322, A::add(A::offset(A::scale(s.ad_value(278), p.p442), p.p89), A::scale(s.ad_value(279), p.p443)), A::scale(s.ad_value(280), p.p444));

        s.store_add_ad(323, A::add(A::offset(A::scale(s.ad_value(278), p.p445), p.p90), A::scale(s.ad_value(279), p.p446)), A::scale(s.ad_value(280), p.p447));

        s.store_add_ad(324, A::add(A::offset(A::scale(s.ad_value(278), p.p448), p.p91), A::scale(s.ad_value(279), p.p449)), A::scale(s.ad_value(280), p.p450));

        s.store_add_ad(325, A::add(A::offset(A::scale(s.ad_value(278), p.p451), p.p92), A::scale(s.ad_value(279), p.p452)), A::scale(s.ad_value(280), p.p453));

        s.store_add_ad(417, A::add(A::offset(A::scale(s.ad_value(278), p.p454), p.p93), A::scale(s.ad_value(279), p.p455)), A::scale(s.ad_value(280), p.p456));

        s.store_add_ad(418, A::add(A::offset(A::scale(s.ad_value(278), p.p457), p.p94), A::scale(s.ad_value(279), p.p458)), A::scale(s.ad_value(280), p.p459));

        s.store_add_ad(327, A::add(A::offset(A::scale(s.ad_value(278), p.p463), p.p116), A::scale(s.ad_value(279), p.p464)), A::scale(s.ad_value(280), p.p465));

        s.store_add_ad(328, A::add(A::offset(A::scale(s.ad_value(278), p.p466), p.p123), A::scale(s.ad_value(279), p.p467)), A::scale(s.ad_value(280), p.p468));

        s.store_add_ad(329, A::add(A::offset(A::scale(s.ad_value(278), p.p469), p.p124), A::scale(s.ad_value(279), p.p470)), A::scale(s.ad_value(280), p.p471));

        s.store_add_ad(330, A::add(A::offset(A::scale(s.ad_value(278), p.p472), p.p122), A::scale(s.ad_value(279), p.p473)), A::scale(s.ad_value(280), p.p474));

        s.store_add_ad(331, A::add(A::offset(A::scale(s.ad_value(278), p.p475), p.p135), A::scale(s.ad_value(279), p.p476)), A::scale(s.ad_value(280), p.p477));

        s.store_add_ad(332, A::add(A::offset(A::scale(s.ad_value(278), p.p478), p.p139), A::scale(s.ad_value(279), p.p479)), A::scale(s.ad_value(280), p.p480));

        s.store_add_ad(333, A::add(A::offset(A::scale(s.ad_value(278), p.p481), p.p145), A::scale(s.ad_value(279), p.p482)), A::scale(s.ad_value(280), p.p483));

        s.store_add_ad(334, A::add(A::offset(A::scale(s.ad_value(278), p.p484), p.p148), A::scale(s.ad_value(279), p.p485)), A::scale(s.ad_value(280), p.p486));

        s.store_add_ad(335, A::add(A::offset(A::scale(s.ad_value(278), p.p487), p.p155), A::scale(s.ad_value(279), p.p488)), A::scale(s.ad_value(280), p.p489));

        s.store_add_ad(336, A::add(A::offset(A::scale(s.ad_value(278), p.p490), p.p142), A::scale(s.ad_value(279), p.p491)), A::scale(s.ad_value(280), p.p492));

        s.store_add_ad(342, A::add(A::offset(A::scale(s.ad_value(278), p.p493), p.p163), A::scale(s.ad_value(279), p.p494)), A::scale(s.ad_value(280), p.p495));

        s.store_add_ad(337, A::add(A::offset(A::scale(s.ad_value(278), p.p496), p.p157), A::scale(s.ad_value(279), p.p497)), A::scale(s.ad_value(280), p.p498));

        s.store_add_ad(338, A::add(A::offset(A::scale(s.ad_value(278), p.p499), p.p156), A::scale(s.ad_value(279), p.p500)), A::scale(s.ad_value(280), p.p501));

        s.store_add_ad(339, A::add(A::offset(A::scale(s.ad_value(278), p.p502), p.p158), A::scale(s.ad_value(279), p.p503)), A::scale(s.ad_value(280), p.p504));

        s.store_add_ad(340, A::add(A::offset(A::scale(s.ad_value(278), p.p505), p.p160), A::scale(s.ad_value(279), p.p506)), A::scale(s.ad_value(280), p.p507));

        s.store_add_ad(341, A::add(A::offset(A::scale(s.ad_value(278), p.p508), p.p161), A::scale(s.ad_value(279), p.p509)), A::scale(s.ad_value(280), p.p510));

        s.store_add_ad(343, A::add(A::offset(A::scale(s.ad_value(278), p.p511), p.p136), A::scale(s.ad_value(279), p.p512)), A::scale(s.ad_value(280), p.p513));

        s.store_add_ad(344, A::add(A::offset(A::scale(s.ad_value(278), p.p514), p.p166), A::scale(s.ad_value(279), p.p515)), A::scale(s.ad_value(280), p.p516));

        s.store_add_ad(345, A::add(A::offset(A::scale(s.ad_value(278), p.p517), p.p167), A::scale(s.ad_value(279), p.p518)), A::scale(s.ad_value(280), p.p519));

        s.store_add_ad(346, A::add(A::offset(A::scale(s.ad_value(278), p.p520), p.p173), A::scale(s.ad_value(279), p.p521)), A::scale(s.ad_value(280), p.p522));

        s.store_add_ad(347, A::add(A::offset(A::scale(s.ad_value(278), p.p523), p.p176), A::scale(s.ad_value(279), p.p524)), A::scale(s.ad_value(280), p.p525));

        s.store_add_ad(348, A::add(A::offset(A::scale(s.ad_value(278), p.p526), p.p182), A::scale(s.ad_value(279), p.p527)), A::scale(s.ad_value(280), p.p528));

        s.store_add_ad(349, A::add(A::offset(A::scale(s.ad_value(278), p.p529), p.p170), A::scale(s.ad_value(279), p.p530)), A::scale(s.ad_value(280), p.p531));

        s.store_add_ad(350, A::add(A::offset(A::scale(s.ad_value(278), p.p532), p.p183), A::scale(s.ad_value(279), p.p533)), A::scale(s.ad_value(280), p.p534));

        s.store_add_ad(351, A::add(A::offset(A::scale(s.ad_value(278), p.p535), p.p186), A::scale(s.ad_value(279), p.p536)), A::scale(s.ad_value(280), p.p537));

        s.store_add_ad(353, A::add(A::offset(A::scale(s.ad_value(278), p.p538), p.p119), A::scale(s.ad_value(279), p.p539)), A::scale(s.ad_value(280), p.p540));

        s.store_add_ad(354, A::add(A::offset(A::scale(s.ad_value(278), p.p541), p.p130), A::scale(s.ad_value(279), p.p542)), A::scale(s.ad_value(280), p.p543));

        s.store_add_ad(355, A::add(A::offset(A::scale(s.ad_value(278), p.p544), p.p205), A::scale(s.ad_value(279), p.p545)), A::scale(s.ad_value(280), p.p546));

        s.store_add_ad(356, A::add(A::offset(A::scale(s.ad_value(278), p.p547), p.p305), A::scale(s.ad_value(279), p.p548)), A::scale(s.ad_value(280), p.p549));

        s.store_add_ad(357, A::add(A::offset(A::scale(s.ad_value(278), p.p550), p.p306), A::scale(s.ad_value(279), p.p551)), A::scale(s.ad_value(280), p.p552));

        s.store_add_ad(358, A::add(A::offset(A::scale(s.ad_value(278), p.p553), p.p307), A::scale(s.ad_value(279), p.p554)), A::scale(s.ad_value(280), p.p555));

        s.store_add_ad(360, A::add(A::offset(A::scale(s.ad_value(278), p.p559), p.p210), A::scale(s.ad_value(279), p.p560)), A::scale(s.ad_value(280), p.p561));

        s.store_add_ad(361, A::add(A::offset(A::scale(s.ad_value(278), p.p562), p.p214), A::scale(s.ad_value(279), p.p563)), A::scale(s.ad_value(280), p.p564));

        s.store_add_ad(362, A::add(A::offset(A::scale(s.ad_value(278), p.p565), p.p208), A::scale(s.ad_value(279), p.p566)), A::scale(s.ad_value(280), p.p567));

        s.store_add_ad(363, A::add(A::offset(A::scale(s.ad_value(278), p.p568), p.p206), A::scale(s.ad_value(279), p.p569)), A::scale(s.ad_value(280), p.p570));

        s.store_add_ad(364, A::add(A::offset(A::scale(s.ad_value(278), p.p571), p.p207), A::scale(s.ad_value(279), p.p572)), A::scale(s.ad_value(280), p.p573));

        s.store_add_ad(365, A::add(A::offset(A::scale(s.ad_value(278), p.p574), p.p209), A::scale(s.ad_value(279), p.p575)), A::scale(s.ad_value(280), p.p576));

        s.store_add_ad(366, A::add(A::offset(A::scale(s.ad_value(278), p.p577), p.p256), A::scale(s.ad_value(279), p.p578)), A::scale(s.ad_value(280), p.p579));

        s.store_add_ad(367, A::add(A::offset(A::scale(s.ad_value(278), p.p580), p.p257), A::scale(s.ad_value(279), p.p581)), A::scale(s.ad_value(280), p.p582));

        s.store_add_ad(368, A::add(A::offset(A::scale(s.ad_value(278), p.p583), p.p258), A::scale(s.ad_value(279), p.p584)), A::scale(s.ad_value(280), p.p585));

        s.store_add_ad(408, A::add(A::offset(A::scale(s.ad_value(278), p.p706), p.p217), A::scale(s.ad_value(279), p.p707)), A::scale(s.ad_value(280), p.p708));

        s.store_add_ad(409, A::add(A::offset(A::scale(s.ad_value(278), p.p709), p.p218), A::scale(s.ad_value(279), p.p710)), A::scale(s.ad_value(280), p.p711));

        s.store_add_ad(410, A::add(A::offset(A::scale(s.ad_value(278), p.p712), p.p219), A::scale(s.ad_value(279), p.p713)), A::scale(s.ad_value(280), p.p714));

        s.store_add_ad(411, A::add(A::offset(A::scale(s.ad_value(278), p.p715), p.p220), A::scale(s.ad_value(279), p.p716)), A::scale(s.ad_value(280), p.p717));

        s.store_add_ad(412, A::add(A::offset(A::scale(s.ad_value(278), p.p718), p.p221), A::scale(s.ad_value(279), p.p719)), A::scale(s.ad_value(280), p.p720));

        s.store_add_ad(413, A::add(A::offset(A::scale(s.ad_value(278), p.p721), p.p222), A::scale(s.ad_value(279), p.p722)), A::scale(s.ad_value(280), p.p723));

        s.store_add_ad(414, A::add(A::offset(A::scale(s.ad_value(278), p.p724), p.p223), A::scale(s.ad_value(279), p.p725)), A::scale(s.ad_value(280), p.p726));

        s.store_add_ad(415, A::add(A::offset(A::scale(s.ad_value(278), p.p727), p.p224), A::scale(s.ad_value(279), p.p728)), A::scale(s.ad_value(280), p.p729));

        s.store_add_ad(416, A::add(A::offset(A::scale(s.ad_value(278), p.p730), p.p225), A::scale(s.ad_value(279), p.p731)), A::scale(s.ad_value(280), p.p732));

        s.store_add_ad(369, A::add(A::offset(A::scale(s.ad_value(278), p.p586), p.p226), A::scale(s.ad_value(279), p.p587)), A::scale(s.ad_value(280), p.p588));

        s.store_add_ad(370, A::add(A::offset(A::scale(s.ad_value(278), p.p589), p.p227), A::scale(s.ad_value(279), p.p590)), A::scale(s.ad_value(280), p.p591));

        s.store_add_ad(371, A::add(A::offset(A::scale(s.ad_value(278), p.p592), p.p228), A::scale(s.ad_value(279), p.p593)), A::scale(s.ad_value(280), p.p594));

        s.store_add_ad(373, A::add(A::offset(A::scale(s.ad_value(278), p.p595), p.p230), A::scale(s.ad_value(279), p.p596)), A::scale(s.ad_value(280), p.p597));

        s.store_add_ad(372, A::add(A::offset(A::scale(s.ad_value(278), p.p598), p.p229), A::scale(s.ad_value(279), p.p599)), A::scale(s.ad_value(280), p.p600));

        s.store_add_ad(381, A::add(A::offset(A::scale(s.ad_value(278), p.p610), p.p247), A::scale(s.ad_value(279), p.p611)), A::scale(s.ad_value(280), p.p612));

        s.store_add_ad(374, A::add(A::offset(A::scale(s.ad_value(278), p.p619), p.p250), A::scale(s.ad_value(279), p.p620)), A::scale(s.ad_value(280), p.p621));

        s.store_add_ad(375, A::add(A::offset(A::scale(s.ad_value(278), p.p622), p.p251), A::scale(s.ad_value(279), p.p623)), A::scale(s.ad_value(280), p.p624));

        s.store_add_ad(376, A::add(A::offset(A::scale(s.ad_value(278), p.p625), p.p252), A::scale(s.ad_value(279), p.p626)), A::scale(s.ad_value(280), p.p627));

        s.store_add_ad(377, A::add(A::offset(A::scale(s.ad_value(278), p.p628), p.p253), A::scale(s.ad_value(279), p.p629)), A::scale(s.ad_value(280), p.p630));

        s.store_add_ad(378, A::add(A::offset(A::scale(s.ad_value(278), p.p601), p.p244), A::scale(s.ad_value(279), p.p602)), A::scale(s.ad_value(280), p.p603));

        s.store_add_ad(379, A::add(A::offset(A::scale(s.ad_value(278), p.p604), p.p245), A::scale(s.ad_value(279), p.p605)), A::scale(s.ad_value(280), p.p606));

        s.store_add_ad(380, A::add(A::offset(A::scale(s.ad_value(278), p.p607), p.p246), A::scale(s.ad_value(279), p.p608)), A::scale(s.ad_value(280), p.p609));

        s.store_add_ad(390, A::add(A::offset(A::scale(s.ad_value(278), p.p613), p.p248), A::scale(s.ad_value(279), p.p614)), A::scale(s.ad_value(280), p.p615));

        s.store_add_ad(392, A::add(A::offset(A::scale(s.ad_value(278), p.p631), p.p254), A::scale(s.ad_value(279), p.p632)), A::scale(s.ad_value(280), p.p633));

        s.store_add_ad(391, A::add(A::offset(A::scale(s.ad_value(278), p.p616), p.p249), A::scale(s.ad_value(279), p.p617)), A::scale(s.ad_value(280), p.p618));

        s.store_add_ad(393, A::add(A::offset(A::scale(s.ad_value(278), p.p634), p.p255), A::scale(s.ad_value(279), p.p635)), A::scale(s.ad_value(280), p.p636));

        s.store_add_ad(382, A::add(A::offset(A::scale(s.ad_value(278), p.p637), p.p231), A::scale(s.ad_value(279), p.p638)), A::scale(s.ad_value(280), p.p639));

        s.store_add_ad(383, A::add(A::offset(A::scale(s.ad_value(278), p.p643), p.p232), A::scale(s.ad_value(279), p.p644)), A::scale(s.ad_value(280), p.p645));

        s.store_add_ad(384, A::add(A::offset(A::scale(s.ad_value(278), p.p649), p.p233), A::scale(s.ad_value(279), p.p650)), A::scale(s.ad_value(280), p.p651));

        s.store_add_ad(385, A::add(A::offset(A::scale(s.ad_value(278), p.p655), p.p242), A::scale(s.ad_value(279), p.p656)), A::scale(s.ad_value(280), p.p657));

        s.store_add_ad(386, A::add(A::offset(A::scale(s.ad_value(278), p.p640), p.p236), A::scale(s.ad_value(279), p.p641)), A::scale(s.ad_value(280), p.p642));

        s.store_add_ad(387, A::add(A::offset(A::scale(s.ad_value(278), p.p646), p.p237), A::scale(s.ad_value(279), p.p647)), A::scale(s.ad_value(280), p.p648));

        s.store_add_ad(388, A::add(A::offset(A::scale(s.ad_value(278), p.p652), p.p238), A::scale(s.ad_value(279), p.p653)), A::scale(s.ad_value(280), p.p654));

        s.store_add_ad(389, A::add(A::offset(A::scale(s.ad_value(278), p.p658), p.p243), A::scale(s.ad_value(279), p.p659)), A::scale(s.ad_value(280), p.p660));

        s.store_add_ad(394, A::add(A::offset(A::scale(s.ad_value(278), p.p664), p.p241), A::scale(s.ad_value(279), p.p665)), A::scale(s.ad_value(280), p.p666));

        s.store_add_ad(396, A::add(A::offset(A::scale(s.ad_value(278), p.p667), p.p259), A::scale(s.ad_value(279), p.p668)), A::scale(s.ad_value(280), p.p669));

        s.store_add_ad(397, A::add(A::offset(A::scale(s.ad_value(278), p.p670), p.p260), A::scale(s.ad_value(279), p.p671)), A::scale(s.ad_value(280), p.p672));

        s.store_add_ad(398, A::add(A::offset(A::scale(s.ad_value(278), p.p673), p.p261), A::scale(s.ad_value(279), p.p674)), A::scale(s.ad_value(280), p.p675));

        s.store_add_ad(399, A::add(A::offset(A::scale(s.ad_value(278), p.p676), p.p262), A::scale(s.ad_value(279), p.p677)), A::scale(s.ad_value(280), p.p678));

        s.store_add_ad(400, A::add(A::offset(A::scale(s.ad_value(278), p.p679), p.p100), A::scale(s.ad_value(279), p.p680)), A::scale(s.ad_value(280), p.p681));

        s.store_add_ad(401, A::add(A::offset(A::scale(s.ad_value(278), p.p682), p.p129), A::scale(s.ad_value(279), p.p683)), A::scale(s.ad_value(280), p.p684));

        s.store_add_ad(402, A::add(A::offset(A::scale(s.ad_value(278), p.p685), p.p103), A::scale(s.ad_value(279), p.p686)), A::scale(s.ad_value(280), p.p687));

        s.store_add_ad(403, A::add(A::offset(A::scale(s.ad_value(278), p.p688), p.p106), A::scale(s.ad_value(279), p.p689)), A::scale(s.ad_value(280), p.p690));

        s.store_add_ad(404, A::add(A::offset(A::scale(s.ad_value(278), p.p691), p.p110), A::scale(s.ad_value(279), p.p692)), A::scale(s.ad_value(280), p.p693));

        s.store_add_ad(405, A::add(A::offset(A::scale(s.ad_value(278), p.p694), p.p111), A::scale(s.ad_value(279), p.p695)), A::scale(s.ad_value(280), p.p696));

        s.store_add_ad(407, A::add(A::offset(A::scale(s.ad_value(278), p.p697), p.p112), A::scale(s.ad_value(279), p.p698)), A::scale(s.ad_value(280), p.p699));

        s.store_add_ad(406, A::add(A::offset(A::scale(s.ad_value(278), p.p700), p.p137), A::scale(s.ad_value(279), p.p701)), A::scale(s.ad_value(280), p.p702));

        s.store_add_ad(352, A::add(A::offset(A::scale(s.ad_value(278), p.p703), p.p187), A::scale(s.ad_value(279), p.p704)), A::scale(s.ad_value(280), p.p705));

        s.store_add_ad(62, A::add(A::offset(A::scale(s.ad_value(278), p.p739), p.p95), A::scale(s.ad_value(279), p.p740)), A::scale(s.ad_value(280), p.p741));

        s.store_add_ad(66, A::add(A::offset(A::scale(s.ad_value(278), p.p742), p.p96), A::scale(s.ad_value(279), p.p743)), A::scale(s.ad_value(280), p.p744));

        s.store_add_ad(67, A::add(A::offset(A::scale(s.ad_value(278), p.p745), p.p97), A::scale(s.ad_value(279), p.p746)), A::scale(s.ad_value(280), p.p747));

        s.store_add_ad(68, A::add(A::offset(A::scale(s.ad_value(278), p.p748), p.p98), A::scale(s.ad_value(279), p.p749)), A::scale(s.ad_value(280), p.p750));

        s.v[542] = if ((p.p20 == 1.0) && (p.p317 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[542] != 0.0) {
            s.store_add_ad(275, A::add(A::offset(A::scale(s.ad_value(278), p.p733), p.p317), A::scale(s.ad_value(279), p.p734)), A::scale(s.ad_value(280), p.p735));
        }

        if (!(s.v[542] != 0.0)) {
            s.store_scalar(275, 0.0);
        }

        s.v[17] = ((3.9 * 8.85418e-12) / p.p45);

        s.v[18] = ((3.9 * 8.85418e-12) / p.p47);

        s.v[19] = ((3.9 * 8.85418e-12) / p.p46);

        s.v[20] = (s.v[16] / p.p49);

        s.v[21] = (p.p59 / 3.9);

        s.v[543] = if !(if self.param_given[47] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if (s.v[543] != 0.0) {
            s.store_scalar(221, (((p.p45 * p.p60) / 3.9) - p.p48));
        }

        if (!(s.v[543] != 0.0)) {
            s.store_scalar(221, p.p47);
        }

        s.v[544] = if (p.p138 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[544] != 0.0) {
            s.store_mul_ad_rhs(331, 331, A::sub_from_scalar(1.0, A::mul(s.ad_value(406), A::powf(s.ad_value(2), (-p.p138)))));
        }

        if (!(s.v[544] != 0.0)) {
            s.store_mul_ad_rhs(331, 331, A::sub_from_scalar(1.0, s.ad_value(406)));
        }

        s.store_add_ad_rhs(332, 332, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p141))), p.p140));

        s.store_add_ad_rhs(333, 333, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p147))), p.p146));

        s.store_offset_ad(137, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p153))), p.p152), p.p151);

        s.store_add_ad_rhs(334, 334, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p150))), p.p149));

        s.store_add_ad_rhs(336, 336, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p144))), p.p143));

        s.store_add_ad_rhs(342, 342, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p165))), p.p164));

        s.v[545] = if (p.p188 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[545] != 0.0) {
            s.store_mul_ad_rhs(344, 344, A::sub_from_scalar(1.0, A::mul(s.ad_value(352), A::powf(s.ad_value(2), (-p.p188)))));
        }

        if (!(s.v[545] != 0.0)) {
            s.store_mul_ad_rhs(344, 344, A::sub_from_scalar(1.0, s.ad_value(352)));
        }

        s.store_add_ad_rhs(345, 345, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p169))), p.p168));

        s.store_add_ad_rhs(346, 346, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p175))), p.p174));

        s.store_offset_ad(138, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p181))), p.p180), p.p179);

        s.store_add_ad_rhs(347, 347, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p178))), p.p177));

        s.store_add_ad_rhs(349, 349, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p172))), p.p171));

        s.store_add_ad_rhs(350, 350, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p185))), p.p184));

        s.v[546] = if (p.p14 == 1.0) { 1.0 } else { 0.0 };

        if (!(s.v[546] != 0.0)) {
            s.store_add_ad_rhs(281, 281, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p193))), p.p192));
        }

        s.store_add_ad_rhs(360, 360, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p212))), p.p211));

        s.store_add_ad_rhs(326, 326, A::scale(A::powf(A::scale(s.ad_value(2), 1000000.0), (-p.p115)), p.p114));

        s.store_add_ad_rhs(327, 327, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p118))), p.p117));

        s.store_add_ad_rhs(328, 328, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p126))), p.p125));

        s.store_add_ad_rhs(329, 329, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p128))), p.p127));

        s.store_add_ad_rhs(400, 400, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p102))), p.p101));

        s.store_add_ad_rhs(401, 401, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p133))), p.p132));

        s.store_add_ad_rhs(402, 402, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p105))), p.p104));

        s.store_add_ad_rhs(403, 403, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p108))), p.p107));

        s.store_offset_ad(92, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p80))), p.p79), p.p77);

        s.store_offset_ad(93, A::scale(A::limited_exp(A::scale(A::neg(s.ad_value(2)), 1.0 / (p.p82))), p.p81), p.p78);

        s.v[547] = if (s.v[331] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[547] != 0.0) {
            s.store_scalar(331, 0.03);
        }

        s.v[548] = if (s.v[332] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[548] != 0.0) {
            s.store_scalar(332, 0.0);
        }

        s.v[549] = if (s.v[336] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[549] != 0.0) {
            s.store_scalar(336, 0.0);
        }

        s.v[550] = if (s.v[334] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[550] != 0.0) {
            s.store_scalar(334, 0.0);
        }

        s.v[551] = if (s.v[335] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[551] != 0.0) {
            s.store_scalar(335, 0.0);
        }

        s.v[552] = if (s.v[401] < 0.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_1(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[552] != 0.0) {
            s.store_scalar(401, 0.0);
        }

        s.v[134] = p.p190;

        s.v[555] = if (s.v[134] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[555] != 0.0) {
            s.store_scalar(134, 0.0);
        }

        s.v[556] = if (s.v[281] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[556] != 0.0) {
            s.store_scalar(281, 0.0);
        }

        s.v[561] = if (s.v[284] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[561] != 0.0) {
            s.store_scalar(284, 0.0);
        }

        s.v[565] = if (s.v[326] < 2.0) { 1.0 } else { 0.0 };

        if (s.v[565] != 0.0) {
            s.store_scalar(326, 2.0);
        }

        s.store_offset_ad(89, A::sqrt(A::offset(A::div(s.ad_value(321), s.ad_value(2)), 1.0)), (-1.0));

        s.v[78] = (p.p49 + (s.v[21] * (p.p45 + p.p46)));

        s.store_div_from_scalar(163, 1.0, 326);

        s.v[236] = (s.v[19] * p.p3);

        s.v[237] = (s.v[19] * p.p4);

        s.v[34] = (p.p267 * ((((1.0 + (p.p49 / p.p46))).max(1e-38)) as f64).ln());

        s.v[236] = (s.v[236] + (s.v[34] * ((p.p5 - p.p1)).max(0.0)));

        s.v[237] = (s.v[237] + (s.v[34] * ((p.p6 - p.p1)).max(0.0)));

        s.v[236] = (s.v[236]).max(1e-20);

        s.v[237] = (s.v[237]).max(1e-20);

        s.store_scale(114, 343, 0.5);

        s.v[115] = 0.5;

        s.store_scale(143, 351, 0.5);

        s.v[566] = if (p.p12 != 1.0) { 1.0 } else { 0.0 };

        if (s.v[566] != 0.0) {
            s.store_scale(114, 343, (1.0 / 3.0));
        }

        if (s.v[566] != 0.0) {
            s.store_scalar(115, (1.0 / 3.0));
        }

        if (s.v[566] != 0.0) {
            s.store_scale(143, 351, (1.0 / 3.0));
        }

        s.v[129] = (1e-8 / (s.v[21] * p.p45));

        s.store_div_from_scalar_ad(131, 1.0, A::scale(A::pow(A::scale(s.ad_value(3), 1000000.0), s.ad_value(286)), p.p2));

        s.v[253] = ((((s.v[21] * p.p45) * p.p49)) as f64).sqrt();

        s.v[144] = (1e-8 / (s.v[21] * p.p46));

        s.v[567] = if (p.p296 >= (s.v[2] / 2.0)) { 1.0 } else { 0.0 };

        if (s.v[567] != 0.0) {
            s.store_scalar(249, 0.0);
        }

        if (!(s.v[567] != 0.0)) {
            s.store_scalar(249, p.p296);
        }

        s.v[568] = if ((p.p18 != 0.0) && (p.p310 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[568] != 0.0) {
            s.store_scale_ad(270, A::offset(A::scale(s.ad_value(3), p.p2), p.p312), p.p311);
        }

        if (!(s.v[568] != 0.0)) {
            s.store_scalar(270, 0.0);
        }

        s.v[132] = (p.p215 * p.p7);

        s.v[133] = (p.p216 * p.p8);

        s.v[569] = if (s.v[132] <= 0.001) { 1.0 } else { 0.0 };

        if (s.v[569] != 0.0) {
            s.store_scalar(132, 0.001);
        }

        s.v[570] = if (s.v[133] <= 0.001) { 1.0 } else { 0.0 };

        if (s.v[570] != 0.0) {
            s.store_scalar(133, 0.001);
        }

        s.v[571] = if (p.p14 == 1.0) { 1.0 } else { 0.0 };

        s.v[576] = if (s.v[134] <= 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[571] != 0.0)) && (s.v[576] != 0.0)) {
            s.store_scalar(134, 0.0);
        }

        s.v[577] = if (s.v[281] <= 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[571] != 0.0)) && (s.v[577] != 0.0)) {
            s.store_scalar(281, 0.0);
        }

        s.v[578] = if (p.p297 <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[578] != 0.0) {
            s.store_scalar(95, 300.15);
        }

        if (!(s.v[578] != 0.0)) {
            s.store_scalar(95, (p.p297 + 273.15));
        }

        s.v[580] = if (p.p12 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[580] != 0.0) {
            s.store_scalar(206, 745669000000.0);
        }

        if (!(s.v[580] != 0.0)) {
            s.store_scalar(206, 1166450000000.0);
        }

        s.v[34] = (p.p99 * p.p99);

        s.store_scale(35, 394, p.p99);

        s.store_square(36, 35);

        s.v[583] = if ((p.p18 != 0.0) && (p.p310 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[583] != 0.0) {
            s.store_offset_ad(271, A::offset(A::voltage(ctx, &nodes, Some(4), None), ctx.temperature()), p.p9);
        }

        if (!(s.v[583] != 0.0)) {
            s.store_scalar(271, (ctx.temperature() + p.p9));
        }

        s.v[272] = (p.p298 + 273.15);

        s.store_scale_ad(271, A::sub(A::offset(s.ad_value(271), s.v[272]), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(271), (-s.v[272])), A::offset(s.ad_value(271), (-s.v[272]))), ((0.25 * 0.01) * 0.01)))), 0.5);

        s.store_div(96, 271, 95);

        s.store_sub(97, 271, 95);

        s.store_scale(55, 271, 8.61708e-5);

        s.store_sub_from_scalar_ad(54, p.p55, A::div(A::mul(A::scale(s.ad_value(271), p.p299), s.ad_value(271)), A::offset(s.ad_value(271), p.p300)));

        s.store_mul_ad(35, A::scale(s.ad_value(271), 0.003331667499583542), A::sqrt(A::scale(s.ad_value(271), 0.003331667499583542)));

        s.store_mul_ad(100, A::scale(s.ad_value(35), p.p54), A::limited_exp(A::sub_from_scalar((p.p55 / ((2.0 * 8.61708e-5) * 300.15)), A::div(s.ad_value(54), A::scale(s.ad_value(55), 2.0)))));

        s.store_mul_ad_rhs(80, 55, A::ln(A::max_with_scalar(A::div(A::mul(s.ad_value(289), s.ad_value(290)), A::square(s.ad_value(100))), 1e-38)));

        s.store_mul_ad_rhs(50, 55, A::ln(A::max_with_scalar(A::div(s.ad_value(290), s.ad_value(100)), 1e-38)));

        let assign3610_ad_e4240: A = A::add(A::sub(A::scale(s.ad_value(54), 0.5), A::mul(s.ad_value(55), A::ln(A::max_with_scalar(A::div_from_scalar(p.p52, s.ad_value(100)), 1e-38)))), A::sqrt(A::offset(A::mul(A::sub(A::scale(s.ad_value(54), 0.5), A::mul(s.ad_value(55), A::ln(A::max_with_scalar(A::div_from_scalar(p.p52, s.ad_value(100)), 1e-38)))), A::sub(A::scale(s.ad_value(54), 0.5), A::mul(s.ad_value(55), A::ln(A::max_with_scalar(A::div_from_scalar(p.p52, s.ad_value(100)), 1e-38))))), ((4.0 * 0.0001) * 0.0001))));
        s.store_sub_ad(51, A::scale(s.ad_value(54), 0.5), A::scale(assign3610_ad_e4240, 0.5));

        s.v[585] = if ((p.p52 != 0.0) && (!(if self.param_given[58] { 1.0 } else { 0.0 } != 0.0))) { 1.0 } else { 0.0 };

        s.v[586] = if (p.p13 == (-1.0)) { 1.0 } else { 0.0 };

        if ((s.v[585] != 0.0) && (s.v[586] != 0.0)) {
            s.store_add_ad_lhs(288, A::offset(s.ad_value(288), (-(0.5 * p.p55))), 51);
        }

        if ((s.v[585] != 0.0) && (!(s.v[586] != 0.0))) {
            s.store_sub_ad_lhs(288, A::offset(s.ad_value(288), (0.5 * p.p55)), 51);
        }

        s.store_offset_scaled(98, 54, 0.5, p.p53);

        s.store_mul_ad_rhs(52, 212, A::sub(s.ad_value(287), s.ad_value(98)));

        s.store_mul_ad_rhs(53, 212, A::sub(s.ad_value(288), s.ad_value(98)));

        s.store_sub_ad(99, A::offset(A::scale(s.ad_value(54), 0.5), p.p53), A::mul(s.ad_value(212), A::min(A::scale(s.ad_value(54), 0.5), A::mul(s.ad_value(55), A::ln(A::max_with_scalar(A::div(s.ad_value(289), s.ad_value(100)), 1e-38))))));

        s.store_mul_ad_rhs(200, 212, A::sub(s.ad_value(287), s.ad_value(99)));

        s.store_mul_ad_rhs(240, 212, A::sub(s.ad_value(288), s.ad_value(99)));

        let assign3720_ad_e4372: A = A::mul(A::mul(s.ad_value(331), A::pow(s.ad_value(96), s.ad_value(338))), A::offset(A::offset(A::scale(A::add(A::offset(A::mul(s.ad_value(337), s.ad_value(97)), 0.9), A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(337), s.ad_value(97)), 0.9), A::offset(A::mul(s.ad_value(337), s.ad_value(97)), 0.9)), ((4.0 * 0.001) * 0.001)))), 0.5), 1.0), (-(0.5 * (0.9 + ((((0.9 * 0.9) + ((4.0 * 0.001) * 0.001))) as f64).sqrt())))));
        s.store_ad(126, &assign3720_ad_e4372);

        s.store_mul_ad_rhs(123, 333, A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(97), p.p159), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(97), p.p159), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(97), p.p159), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5));

        s.store_mul_ad_rhs(122, 332, A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(339), s.ad_value(97)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(339), s.ad_value(97)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(339), s.ad_value(97)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5));

        s.store_mul_ad_rhs(125, 334, A::pow(s.ad_value(96), s.ad_value(340)));

        s.store_mul_ad_rhs(124, 335, A::pow(s.ad_value(96), s.ad_value(341)));

        s.store_scale_ad(150, A::add(A::offset(A::offset(A::mul(s.ad_value(355), s.ad_value(97)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(355), s.ad_value(97)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(355), s.ad_value(97)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5);

        s.store_mul_ad_rhs(353, 353, A::offset(A::scale(s.ad_value(278), p.p120), 1.0));

        let assign3790_ad_e4534: A = A::mul(s.ad_value(400), A::offset(A::offset(A::scale(A::add(A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97))), A::sqrt(A::offset(A::mul(A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97))), A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97)))), ((4.0 * 0.001) * 0.001)))), 0.5), 1.0), (-(0.5 * (0.9 + ((((0.9 * 0.9) + ((4.0 * 0.001) * 0.001))) as f64).sqrt())))));
        s.store_ad(164, &assign3790_ad_e4534);

        s.v[587] = if (s.v[164] < 1000.0) { 1.0 } else { 0.0 };

        if (s.v[587] != 0.0) {
            s.store_scalar(164, 1000.0);
        }

        let assign3820_ad_e4586: A = A::mul(s.ad_value(402), A::offset(A::offset(A::scale(A::add(A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97))), A::sqrt(A::offset(A::mul(A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97))), A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97)))), ((4.0 * 0.001) * 0.001)))), 0.5), 1.0), (-(0.5 * (0.9 + ((((0.9 * 0.9) + ((4.0 * 0.001) * 0.001))) as f64).sqrt())))));
        s.store_ad(166, &assign3820_ad_e4586);

        s.v[588] = if (s.v[166] < 1000.0) { 1.0 } else { 0.0 };

        if (s.v[588] != 0.0) {
            s.store_scalar(166, 1000.0);
        }

        let assign3850_ad_e4638: A = A::mul(s.ad_value(403), A::offset(A::offset(A::scale(A::add(A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97))), A::sqrt(A::offset(A::mul(A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97))), A::sub_from_scalar(0.9, A::mul(s.ad_value(353), s.ad_value(97)))), ((4.0 * 0.001) * 0.001)))), 0.5), 1.0), (-(0.5 * (0.9 + ((((0.9 * 0.9) + ((4.0 * 0.001) * 0.001))) as f64).sqrt())))));
        s.store_ad(167, &assign3850_ad_e4638);

        s.v[589] = if (s.v[167] < 1000.0) { 1.0 } else { 0.0 };

        if (s.v[589] != 0.0) {
            s.store_scalar(167, 1000.0);
        }

        let assign3880_ad_e4688: A = A::mul(s.ad_value(316), A::offset(A::offset(A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(97), p.p309), (-(-0.9))), (-0.0001)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(97), p.p309), (-(-0.9))), (-0.0001)), A::offset(A::offset(A::scale(s.ad_value(97), p.p309), (-(-0.9))), (-0.0001))), (-((4.0 * (-0.9)) * 0.0001))))), 0.5), (-0.9)), 1.0));
        s.store_ad(107, &assign3880_ad_e4688);

        s.store_mul_ad_rhs(354, 354, A::offset(A::scale(s.ad_value(278), p.p131), 1.0));

        let assign3900_ad_e4740: A = A::mul(s.ad_value(401), A::offset(A::offset(A::scale(A::add(A::sub_from_scalar(0.9, A::mul(s.ad_value(354), s.ad_value(97))), A::sqrt(A::offset(A::mul(A::sub_from_scalar(0.9, A::mul(s.ad_value(354), s.ad_value(97))), A::sub_from_scalar(0.9, A::mul(s.ad_value(354), s.ad_value(97)))), ((4.0 * 0.001) * 0.001)))), 0.5), 1.0), (-(0.5 * (0.9 + ((((0.9 * 0.9) + ((4.0 * 0.001) * 0.001))) as f64).sqrt())))));
        s.store_ad(165, &assign3900_ad_e4740);

        let assign3910_ad_e4780: A = A::offset(A::scale(A::add(A::offset(A::mul(s.ad_value(326), A::offset(A::scale(s.ad_value(97), p.p121), 1.0)), (-2.0)), A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(326), A::offset(A::scale(s.ad_value(97), p.p121), 1.0)), (-2.0)), A::offset(A::mul(s.ad_value(326), A::offset(A::scale(s.ad_value(97), p.p121), 1.0)), (-2.0))), ((4.0 * 0.001) * 0.001)))), 0.5), 2.0);
        s.store_ad(168, &assign3910_ad_e4780);

        s.store_add_ad_rhs(175, 322, A::mul(s.ad_value(323), s.ad_value(97)));

        let assign3930_ad_e4823: A = A::add(A::offset(A::sub(A::mul(s.ad_value(325), s.ad_value(97)), A::neg(s.ad_value(324))), (-1e-6)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::mul(s.ad_value(325), s.ad_value(97)), A::neg(s.ad_value(324))), (-1e-6)), A::offset(A::sub(A::mul(s.ad_value(325), s.ad_value(97)), A::neg(s.ad_value(324))), (-1e-6))), A::scale(A::neg(s.ad_value(324)), (4.0 * 1e-6)))));
        s.store_add_ad_rhs(176, 324, A::sub(A::scale(assign3930_ad_e4823, 0.5), s.ad_value(324)));

        s.store_add_ad_rhs(108, 417, A::mul(s.ad_value(418), s.ad_value(97)));

        s.store_mul_ad_rhs(182, 327, A::scale(A::add(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(330), s.ad_value(97))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(330), s.ad_value(97))), (-1e-6)), A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(330), s.ad_value(97))), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5));

        s.store_mul_ad(102, A::offset(A::div_from_scalar(p.p302, s.ad_value(2)), p.p301), A::offset(s.ad_value(96), (-1.0)));

        s.store_mul_ad_rhs(103, 368, A::pow(s.ad_value(96), s.ad_value(356)));

        s.store_mul_ad_rhs(104, 379, A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(357), s.ad_value(97)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(357), s.ad_value(97)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(357), s.ad_value(97)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5));

        s.store_mul_ad_rhs(105, 375, A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(358), s.ad_value(97)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(358), s.ad_value(97)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(358), s.ad_value(97)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5));

        s.store_mul_ad_rhs(29, 212, A::voltage(ctx, &nodes, Some(8), Some(6)));

        s.store_mul_ad_rhs(30, 212, A::voltage(ctx, &nodes, Some(5), Some(6)));

        s.store_mul_ad_rhs(31, 212, A::voltage(ctx, &nodes, Some(8), Some(5)));

        s.store_mul_ad_rhs(32, 212, A::voltage(ctx, &nodes, Some(3), Some(6)));

        s.store_mul_ad_rhs(33, 212, A::voltage(ctx, &nodes, Some(3), Some(5)));

        s.store_mul_ad_rhs(209, 212, A::voltage(ctx, &nodes, Some(8), Some(3)));

        s.v[27] = 1.0;

        s.v[590] = if (s.v[30] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[590] != 0.0) {
            s.store_scalar(27, (-1.0));
        }

        if (s.v[590] != 0.0) {
            s.copy_ad(22, 31);
        }

        if (s.v[590] != 0.0) {
            s.store_neg(26, 30);
        }

        if (s.v[590] != 0.0) {
            s.copy_ad(23, 33);
        }

        if (s.v[590] != 0.0) {
            s.copy_ad(24, 32);
        }

        if (!(s.v[590] != 0.0)) {
            s.copy_ad(22, 29);
        }

        if (!(s.v[590] != 0.0)) {
            s.copy_ad(26, 30);
        }

        if (!(s.v[590] != 0.0)) {
            s.copy_ad(23, 32);
        }

        if (!(s.v[590] != 0.0)) {
            s.copy_ad(24, 33);
        }

        s.store_mul_ad_rhs(234, 212, A::voltage(ctx, &nodes, Some(7), Some(5)));

        s.store_mul_ad_rhs(235, 212, A::voltage(ctx, &nodes, Some(7), Some(6)));

        s.store_offset_ad(73, A::sqrt(A::offset(A::square(s.ad_value(26)), 0.0004)), (-0.02));

        s.store_scaled_sub(74, 73, 26, 0.5);

        s.store_add(25, 23, 74);

        s.store_sub(69, 22, 52);

        s.store_sub(70, 23, 53);

        s.v[77] = ((((s.v[21] * p.p49) * p.p45)) as f64).sqrt();

        s.v[76] = (((p.p49 * ((s.v[21] * p.p45) + (0.375 * p.p49)))) as f64).sqrt();

        s.store_add_ad_lhs(34, A::scale(A::add(A::scale(s.ad_value(69), (p.p46 * s.v[21])), A::scale(s.ad_value(70), ((p.p45 * s.v[21]) + p.p49))), 1.0 / (s.v[78])), 74);

        s.store_offset_ad(35, A::scale(A::atan(A::add(s.ad_value(311), A::mul(s.ad_value(312), s.ad_value(34)))), 0.3183098861837907), 0.5);

        s.store_offset_scaled(75, 35, (s.v[77] - s.v[76]), s.v[76]);

        s.store_offset_ad(61, A::div(A::mul(s.ad_value(314), s.ad_value(2)), s.ad_value(75)), 1e-6);

        s.v[591] = if (s.v[61] < 40.0) { 1.0 } else { 0.0 };

        if (s.v[591] != 0.0) {
            s.store_div_from_scalar_ad(88, 0.5, A::offset(A::cosh(s.ad_value(61)), (-1.0)));
        }

        if (!(s.v[591] != 0.0)) {
            s.store_limited_exp_ad(88, A::neg(s.ad_value(61)));
        }

        s.store_offset_ad(61, A::div(A::mul(s.ad_value(319), s.ad_value(2)), s.ad_value(75)), 1e-6);

        s.v[592] = if (s.v[61] < 40.0) { 1.0 } else { 0.0 };

        if (s.v[592] != 0.0) {
            s.store_div_from_scalar_ad(90, 0.5, A::offset(A::cosh(s.ad_value(61)), (-1.0)));
        }

        if (!(s.v[592] != 0.0)) {
            s.store_limited_exp_ad(90, A::neg(s.ad_value(61)));
        }

        s.v[593] = if (s.v[61] < 40.0) { 1.0 } else { 0.0 };

        if (s.v[593] != 0.0) {
            s.store_div_from_scalar_ad(91, 1.0, A::max_with_scalar(A::offset(A::scale(A::offset(A::cosh(s.ad_value(61)), (-2.0)), p.p83), 1.0), 1e-6));
        }

        if (!(s.v[593] != 0.0)) {
            s.store_div_ad(91, A::limited_exp(A::neg(s.ad_value(61))), A::max_with_scalar(A::offset(A::limited_exp(A::neg(s.ad_value(61))), p.p83), 1e-6));
        }

        s.store_offset_ad(61, A::div(A::mul(s.ad_value(362), s.ad_value(2)), s.ad_value(75)), 1e-6);

        s.v[594] = if (s.v[61] < 40.0) { 1.0 } else { 0.0 };

        if (s.v[594] != 0.0) {
            s.store_add_ad_lhs(153, A::div(A::scale(s.ad_value(363), 0.5), A::offset(A::cosh(s.ad_value(61)), (-1.0))), 364);
        }

        if (!(s.v[594] != 0.0)) {
            s.store_add_ad_lhs(153, A::mul(s.ad_value(363), A::limited_exp(A::neg(s.ad_value(61)))), 364);
        }

        s.v[595] = if (p.p13 == (-1.0)) { 1.0 } else { 0.0 };

        if (s.v[595] != 0.0) {
            s.store_div_ad_lhs(79, A::mul(s.ad_value(298), s.ad_value(2)), 75);
        }

        s.v[596] = if (s.v[79] > 40.0) { 1.0 } else { 0.0 };

        if ((s.v[595] != 0.0) && (s.v[596] != 0.0)) {
            s.store_scale_ad(34, A::limited_exp(s.ad_value(79)), 0.5);
        }

        if ((s.v[595] != 0.0) && (!(s.v[596] != 0.0))) {
            s.store_offset_ad(34, A::cosh(s.ad_value(79)), (-1.0));
        }

        if (s.v[595] != 0.0) {
            s.store_sub_ad_rhs(35, 299, A::div(A::scale(s.ad_value(300), 0.5), s.ad_value(34)));
        }

        if (s.v[595] != 0.0) {
            s.copy_ad(36, 301);
        }

        if (s.v[595] != 0.0) {
            s.copy_ad(246, 296);
        }

        if (s.v[595] != 0.0) {
            s.copy_ad(247, 297);
        }

        if (s.v[595] != 0.0) {
            s.copy_ad(248, 295);
        }

        if (!(s.v[595] != 0.0)) {
            s.store_div_ad_lhs(79, A::mul(s.ad_value(305), s.ad_value(2)), 75);
        }

        s.v[597] = if (s.v[79] > 40.0) { 1.0 } else { 0.0 };

        if ((!(s.v[595] != 0.0)) && (s.v[597] != 0.0)) {
            s.store_scale_ad(34, A::limited_exp(s.ad_value(79)), 0.5);
        }

        if ((!(s.v[595] != 0.0)) && (!(s.v[597] != 0.0))) {
            s.store_offset_ad(34, A::cosh(s.ad_value(79)), (-1.0));
        }

        if (!(s.v[595] != 0.0)) {
            s.store_sub_ad_rhs(35, 306, A::div(A::scale(s.ad_value(307), 0.5), s.ad_value(34)));
        }

        if (!(s.v[595] != 0.0)) {
            s.copy_ad(36, 308);
        }

        if (!(s.v[595] != 0.0)) {
            s.copy_ad(246, 303);
        }

        if (!(s.v[595] != 0.0)) {
            s.copy_ad(247, 304);
        }

        if (!(s.v[595] != 0.0)) {
            s.copy_ad(248, 302);
        }

        s.store_sub(34, 35, 36);

        s.store_add_ad_rhs(241, 36, A::scale(A::add(s.ad_value(34), A::sqrt(A::offset(A::square(s.ad_value(34)), 0.0001))), 0.5));

        s.v[244] = (((1.60219e-19 * p.p52) * s.v[16]) / ((2.0 * s.v[19]) * s.v[19]));

        s.v[598] = if (p.p52 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[598] != 0.0) {
            let assign4690_ad_e5407: A = A::scale(A::add(A::mul(s.ad_value(213), A::sub(A::mul(s.ad_value(212), s.ad_value(25)), s.ad_value(246))), A::sqrt(A::offset(A::mul(A::mul(s.ad_value(213), A::sub(A::mul(s.ad_value(212), s.ad_value(25)), s.ad_value(246))), A::mul(s.ad_value(213), A::sub(A::mul(s.ad_value(212), s.ad_value(25)), s.ad_value(246)))), ((4.0 * 0.01) * 0.01)))), (0.5 * 1.0 / (s.v[244])));
            s.store_offset_ad(34, A::sqrt(A::offset(assign4690_ad_e5407, 1.0)), (-1.0));
        }

        if (!(s.v[598] != 0.0)) {
            s.store_scalar(34, 0.0);
        }

        s.store_mul_ad_lhs(245, A::scale(s.ad_value(34), s.v[244]), 34);

        let assign4720_ad_e5459: A = A::sub(A::scale(A::add(A::offset(A::sub(A::neg(s.ad_value(245)), A::neg(s.ad_value(247))), (-0.01)), A::sqrt(A::sub(A::mul(A::offset(A::sub(A::neg(s.ad_value(245)), A::neg(s.ad_value(247))), (-0.01)), A::offset(A::sub(A::neg(s.ad_value(245)), A::neg(s.ad_value(247))), (-0.01))), A::scale(A::neg(s.ad_value(247)), (4.0 * 0.01))))), 0.5), s.ad_value(247));
        s.store_neg_ad(245, assign4720_ad_e5459);

        s.store_sub_from_scalar(72, (-1.2), 74);

        s.v[243] = (((-s.v[19]) * s.v[20]) / ((s.v[19] + s.v[20]) * s.v[17]));

        s.store_mul_ad(242, A::scale(s.ad_value(241), s.v[243]), A::sub(A::sub(s.ad_value(70), A::mul(A::mul(A::mul(s.ad_value(212), s.ad_value(213)), s.ad_value(248)), s.ad_value(245))), s.ad_value(72)));

        s.store_scale_ad(28, A::add(s.ad_value(25), A::sqrt(A::offset(A::square(s.ad_value(25)), ((4.0 * 0.001) * 0.001)))), 0.5);

        s.store_add_ad_lhs(87, A::offset(s.ad_value(50), 0.4), 315);

        s.v[599] = if (s.v[87] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[599] != 0.0) {
            s.store_scalar(84, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_2(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (!(s.v[599] != 0.0)) {
            s.store_mul_ad(84, A::mul(s.ad_value(320), s.ad_value(89)), A::sqrt(s.ad_value(87)));
        }

        s.store_mul_ad(83, A::mul(A::neg(s.ad_value(313)), s.ad_value(88)), A::sub(s.ad_value(80), s.ad_value(87)));

        s.store_add_ad(82, A::mul(A::mul(A::neg(A::add(s.ad_value(107), A::mul(s.ad_value(318), s.ad_value(25)))), s.ad_value(90)), A::add(s.ad_value(73), A::mul(s.ad_value(317), A::sqrt(A::offset(s.ad_value(73), 0.01))))), A::mul(A::mul(s.ad_value(92), s.ad_value(91)), A::pow(A::offset(s.ad_value(73), 0.01), s.ad_value(93))));

        s.store_mul_ad_lhs(85, A::div(A::neg(s.ad_value(309)), A::add(s.ad_value(2), s.ad_value(310))), 73);

        s.v[35] = ((s.v[20] * s.v[19]) / (s.v[20] + s.v[19]));

        s.store_mul_ad_lhs(36, A::add(s.ad_value(293), A::scale(s.ad_value(28), p.p70)), 73);

        s.store_add_ad(37, A::add(A::scale(s.ad_value(25), p.p66), A::mul(A::scale(s.ad_value(25), p.p67), s.ad_value(25))), A::mul(s.ad_value(88), A::add(A::add(A::add(s.ad_value(292), A::mul(s.ad_value(294), s.ad_value(25))), A::mul(A::scale(s.ad_value(25), p.p69), s.ad_value(25))), s.ad_value(36))));

        s.store_scale_ad(81, A::mul(s.ad_value(55), A::add(A::offset(s.ad_value(291), (s.v[17] + s.v[35])), s.ad_value(37))), 1.0 / ((s.v[17] + s.v[35])));

        s.store_scale_ad(60, A::scale(s.ad_value(290), (1.60219e-19 * (p.p49 * 1.0 / (s.v[17])))), (1.0 - ((0.5 * p.p49) / (p.p49 + (s.v[21] * p.p46)))));

        s.store_mul_ad_lhs(34, A::offset(A::div_from_scalar(p.p304, s.ad_value(2)), p.p303), 25);

        s.store_add_ad_rhs(101, 102, A::mul(s.ad_value(34), A::offset(s.ad_value(96), (-1.0))));

        s.store_add_ad_lhs(86, A::add(A::add(A::add(A::add(A::add(s.ad_value(83), s.ad_value(82)), s.ad_value(84)), s.ad_value(85)), s.ad_value(60)), s.ad_value(101)), 242);

        s.store_offset_ad(71, A::sub(s.ad_value(69), s.ad_value(86)), p.p10);

        s.store_div_ad(421, A::scale(s.ad_value(100), ((2.0 * 1.60219e-19) * (p.p49 * p.p49))), A::scale(s.ad_value(55), s.v[16]));

        s.v[419] = (s.v[17] / s.v[20]);

        s.v[420] = (s.v[19] / s.v[20]);

        s.store_ln(449, 421);

        s.store_sub_from_scalar(450, ((39.47841) as f64).ln(), 449);

        s.v[451] = (s.v[419] * s.v[419]);

        s.v[454] = (s.v[419] / (((s.v[420] * s.v[419]) + s.v[420]) + s.v[419]));

        s.v[460] = 1.0;

        s.store_sub_from_scalar_ad(461, ((s.v[451] * s.v[460]) * s.v[460]), A::mul(s.ad_value(421), A::limited_exp(A::scale(s.ad_value(50), 2.0))));

        s.store_sqrt(462, 461);

        s.store_div_ad(463, A::sub_from_scalar(1.0, A::scale(s.ad_value(462), 0.125)), A::sub_from_scalar(0.5, A::scale(s.ad_value(462), 0.041666666666666664)));

        s.store_mul_ad_lhs(35, A::sub(A::offset(A::ln(A::max_with_scalar(A::offset(A::scale(s.ad_value(463), (s.v[419] * s.v[460])), (((s.v[419] * s.v[419]) * s.v[460]) * s.v[460])), 1e-38)), 1.0), A::ln(A::max_with_scalar(s.ad_value(421), 1e-38))), 55);

        s.store_div(422, 71, 81);

        s.store_div_ad_lhs(423, A::offset(A::sub(s.ad_value(70), s.ad_value(86)), p.p10), 81);

        s.store_sub_ad_lhs(453, A::ln(A::offset(A::mul(A::scale(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451]), A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_sub_ad_lhs(424, A::ln(A::offset(A::mul(A::scale(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451]), A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_scale_ad(452, A::add(s.ad_value(424), A::scale(s.ad_value(423), s.v[420])), 1.0 / ((1.0 + s.v[420])));

        s.store_add_ad_rhs(426, 423, A::scale(A::sub(s.ad_value(422), s.ad_value(423)), s.v[454]));

        s.store_ad(430, &A::min(s.ad_value(426), s.ad_value(453)));

        s.store_ad(430, &A::min(s.ad_value(430), s.ad_value(450)));

        s.store_scale_ad(448, A::add(s.ad_value(430), A::scale(s.ad_value(422), s.v[419])), 1.0 / ((1.0 + s.v[419])));

        s.store_sub(34, 448, 430);

        s.store_div_ad_lhs(37, A::mul(A::limited_exp(s.ad_value(430)), A::offset(A::limited_exp(s.ad_value(34)), (-1.0))), 34);

        s.store_sub(429, 423, 452);

        s.store_sub_ad(442, A::mul(A::scale(s.ad_value(429), (s.v[420] * s.v[420])), s.ad_value(429)), A::mul(s.ad_value(421), A::exp(s.ad_value(452))));

        s.v[600] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[600] != 0.0) {
            s.store_scaled_sub(429, 423, 430, s.v[420]);
        }

        if (s.v[600] != 0.0) {
            s.store_scalar(440, (40.0 * s.v[419]));
        }

        if (s.v[600] != 0.0) {
            s.store_add(455, 440, 429);
        }

        if (s.v[600] != 0.0) {
            s.store_mul(37, 440, 429);
        }

        if (s.v[600] != 0.0) {
            s.store_offset_scaled(38, 455, 0.06534, 1.0);
        }

        if (s.v[600] != 0.0) {
            s.store_offset_ad(39, A::add(A::scale(s.ad_value(455), 8.57973), s.ad_value(37)), 39.47841);
        }

        if (s.v[600] != 0.0) {
            s.store_add_ad(40, A::scale(s.ad_value(455), 78.95683), A::scale(s.ad_value(37), 39.47841));
        }

        if (s.v[600] != 0.0) {
            s.store_div_ad(442, A::sub(A::sqrt(A::add(A::mul(A::scale(s.ad_value(38), (-4.0)), s.ad_value(40)), A::square(s.ad_value(39)))), s.ad_value(39)), A::scale(s.ad_value(38), 2.0));
        }

        if (s.v[600] != 0.0) {
            s.store_scale_ad(37, A::sub(A::scale(s.ad_value(450), (1.0 + s.v[419])), s.ad_value(430)), 1.0 / (s.v[419]));
        }

        if (s.v[600] != 0.0) {
            s.store_scale_ad(34, A::offset(A::sub(s.ad_value(422), s.ad_value(37)), 2.0), (40.0 * 0.2));
        }

        if (s.v[600] != 0.0) {
            s.store_mul_ad_rhs(442, 442, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::offset(A::sub(s.ad_value(422), s.ad_value(37)), 2.0)), 1.0 / ((2.0 / 0.69))))));
        }

        if (s.v[600] != 0.0) {
            s.store_ad(442, &A::min_with_scalar(s.ad_value(442), 50.0));
        }

        s.store_ad(422, &A::max(s.ad_value(422), s.ad_value(450)));

        s.store_sub_ad_lhs(424, A::ln(A::offset(A::mul(A::scale(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451]), A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_scale_ad(37, A::sub(A::scale(s.ad_value(450), (1.0 + s.v[419])), s.ad_value(430)), 1.0 / (s.v[419]));

        s.store_sub_ad_lhs(38, A::ln(A::offset(A::mul(A::scale(A::sub(s.ad_value(37), s.ad_value(450)), s.v[451]), A::sub(s.ad_value(37), s.ad_value(450))), 39.47841)), 449);

        s.store_sub(39, 38, 450);

        s.store_sub(424, 424, 39);

        s.store_sub(440, 422, 424);

        s.store_mul_ad(34, A::neg(s.ad_value(421)), A::exp(s.ad_value(424)));

        s.store_scale(35, 440, s.v[451]);

        s.store_div_ad(425, A::neg(A::sub(A::add(A::mul(s.ad_value(35), s.ad_value(440)), s.ad_value(34)), s.ad_value(442))), A::add(A::scale(s.ad_value(35), (-2.0)), s.ad_value(34)));

        s.store_add(424, 424, 425);

        s.store_sub(440, 422, 424);

        s.store_scale(36, 440, s.v[451]);

        s.store_div_from_scalar_ad(34, 1.0, A::sub(A::mul(s.ad_value(36), s.ad_value(440)), s.ad_value(442)));

        s.store_sub_ad_lhs(465, A::sub(A::ln(A::abs(A::sub(A::mul(s.ad_value(36), s.ad_value(440)), s.ad_value(442)))), s.ad_value(449)), 424);

        s.store_div_from_scalar_ad(466, 1.0, A::offset(A::mul(A::scale(s.ad_value(36), (-2.0)), s.ad_value(34)), (-1.0)));

        s.store_add_ad(467, A::mul(A::mul(A::mul(A::scale(s.ad_value(36), (-4.0)), s.ad_value(36)), s.ad_value(34)), s.ad_value(34)), A::scale(s.ad_value(34), (2.0 * s.v[451])));

        s.store_mul(35, 465, 466);

        s.store_sub_ad(425, A::neg(s.ad_value(35)), A::mul(A::mul(A::mul(A::scale(s.ad_value(35), 0.5), s.ad_value(35)), s.ad_value(467)), s.ad_value(466)));

        s.store_ad(425, &A::max_with_scalar(s.ad_value(425), (-10.0)));

        s.store_ad(425, &A::min_with_scalar(s.ad_value(425), 10.0));

        s.store_add(424, 424, 425);

        s.store_sub(440, 422, 424);

        s.store_scale(36, 440, s.v[451]);

        s.store_div_from_scalar_ad(34, 1.0, A::sub(A::mul(s.ad_value(36), s.ad_value(440)), s.ad_value(442)));

        s.store_sub_ad_lhs(465, A::sub(A::ln(A::abs(A::sub(A::mul(s.ad_value(36), s.ad_value(440)), s.ad_value(442)))), s.ad_value(449)), 424);

        s.store_div_from_scalar_ad(466, 1.0, A::offset(A::mul(A::scale(s.ad_value(36), (-2.0)), s.ad_value(34)), (-1.0)));

        s.store_add_ad(467, A::mul(A::mul(A::mul(A::scale(s.ad_value(36), (-4.0)), s.ad_value(36)), s.ad_value(34)), s.ad_value(34)), A::scale(s.ad_value(34), (2.0 * s.v[451])));

        s.store_mul(35, 465, 466);

        s.store_sub_ad(425, A::neg(s.ad_value(35)), A::mul(A::mul(A::mul(A::scale(s.ad_value(35), 0.5), s.ad_value(35)), s.ad_value(467)), s.ad_value(466)));

        s.store_ad(425, &A::max_with_scalar(s.ad_value(425), (-10.0)));

        s.store_ad(425, &A::min_with_scalar(s.ad_value(425), 10.0));

        s.store_add(424, 424, 425);

        s.store_ad(424, &A::max(s.ad_value(424), A::offset(s.ad_value(450), (-4.0))));

        s.store_div(422, 71, 81);

        s.store_sub_ad_rhs(448, 448, A::ln(A::offset(A::exp(A::sub(s.ad_value(448), A::scale(s.ad_value(424), 1.05))), 1.0)));

        s.store_ad(448, &A::min(s.ad_value(448), s.ad_value(424)));

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_ad(457, A::neg(s.ad_value(421)), A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.v[601] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[601] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[601] != 0.0) {
            s.store_div_from_scalar_ad(459, 1.0, A::sin(A::scale(s.ad_value(439), 0.5)));
        }

        if (s.v[601] != 0.0) {
            s.store_square(35, 459);
        }

        if (s.v[601] != 0.0) {
            s.store_mul_ad_lhs(458, A::cos(A::scale(s.ad_value(439), 0.5)), 459);
        }

        if (s.v[601] != 0.0) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), (-0.5)), 439);
        }

        if (s.v[601] != 0.0) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), 0.25), 34);
        }

        if (!(s.v[601] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[601] != 0.0)) {
            s.store_div_from_scalar_ad(459, 1.0, A::sinh(A::scale(s.ad_value(439), 0.5)));
        }

        if (!(s.v[601] != 0.0)) {
            s.store_square(35, 459);
        }

        if (!(s.v[601] != 0.0)) {
            s.store_sqrt_ad(458, A::offset(s.ad_value(35), 1.0));
        }

        if (!(s.v[601] != 0.0)) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), 0.5), 439);
        }

        if (!(s.v[601] != 0.0)) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), (-0.25)), 34);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_sub_ad(429, A::add(A::sub(s.ad_value(423), s.ad_value(422)), s.ad_value(440)), A::ln(A::abs(A::mul(A::mul(A::mul(s.ad_value(442), s.ad_value(35)), s.ad_value(37)), s.ad_value(37)))));

        s.store_add_ad_rhs(427, 457, A::mul(A::add(s.ad_value(456), s.ad_value(446)), A::add(A::scale(s.ad_value(429), s.v[420]), s.ad_value(456))));

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_ad_lhs(443, A::scale(s.ad_value(456), ((-2.0) * s.v[419])), 457);

        s.store_mul(444, 445, 443);

        s.store_sub_ad(441, A::offset(A::scale(A::mul(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37)), 2.0), (-1.0)), A::mul(s.ad_value(447), s.ad_value(443)));

        s.store_add_ad(428, A::add(A::sub(s.ad_value(457), A::scale(A::add(s.ad_value(456), s.ad_value(36)), s.v[419])), A::mul(s.ad_value(456), s.ad_value(444))), A::scale(A::add(A::mul(s.ad_value(441), s.ad_value(36)), A::mul(s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])))), s.v[420]));

        s.store_div_ad_lhs(425, A::neg(s.ad_value(427)), 428);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_ad(457, A::neg(s.ad_value(421)), A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.v[602] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[602] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[602] != 0.0) {
            s.store_div_from_scalar_ad(459, 1.0, A::sin(A::scale(s.ad_value(439), 0.5)));
        }

        if (s.v[602] != 0.0) {
            s.store_square(35, 459);
        }

        if (s.v[602] != 0.0) {
            s.store_mul_ad_lhs(458, A::cos(A::scale(s.ad_value(439), 0.5)), 459);
        }

        if (s.v[602] != 0.0) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), (-0.5)), 439);
        }

        if (s.v[602] != 0.0) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), 0.25), 34);
        }

        if (!(s.v[602] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[602] != 0.0)) {
            s.store_div_from_scalar_ad(459, 1.0, A::sinh(A::scale(s.ad_value(439), 0.5)));
        }

        if (!(s.v[602] != 0.0)) {
            s.store_square(35, 459);
        }

        if (!(s.v[602] != 0.0)) {
            s.store_sqrt_ad(458, A::offset(s.ad_value(35), 1.0));
        }

        if (!(s.v[602] != 0.0)) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), 0.5), 439);
        }

        if (!(s.v[602] != 0.0)) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), (-0.25)), 34);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_sub_ad(429, A::add(A::sub(s.ad_value(423), s.ad_value(422)), s.ad_value(440)), A::ln(A::abs(A::mul(A::mul(A::mul(s.ad_value(442), s.ad_value(35)), s.ad_value(37)), s.ad_value(37)))));

        s.store_add_ad_rhs(427, 457, A::mul(A::add(s.ad_value(456), s.ad_value(446)), A::add(A::scale(s.ad_value(429), s.v[420]), s.ad_value(456))));

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_ad_lhs(443, A::scale(s.ad_value(456), ((-2.0) * s.v[419])), 457);

        s.store_mul(444, 445, 443);

        s.store_sub_ad(441, A::offset(A::scale(A::mul(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37)), 2.0), (-1.0)), A::mul(s.ad_value(447), s.ad_value(443)));

        s.store_add_ad(428, A::add(A::sub(s.ad_value(457), A::scale(A::add(s.ad_value(456), s.ad_value(36)), s.v[419])), A::mul(s.ad_value(456), s.ad_value(444))), A::scale(A::add(A::mul(s.ad_value(441), s.ad_value(36)), A::mul(s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])))), s.v[420]));

        s.store_div_ad_lhs(425, A::neg(s.ad_value(427)), 428);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_ad(457, A::neg(s.ad_value(421)), A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.v[603] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[603] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[603] != 0.0) {
            s.store_div_from_scalar_ad(459, 1.0, A::sin(A::scale(s.ad_value(439), 0.5)));
        }

        if (s.v[603] != 0.0) {
            s.store_square(35, 459);
        }

        if (s.v[603] != 0.0) {
            s.store_mul_ad_lhs(458, A::cos(A::scale(s.ad_value(439), 0.5)), 459);
        }

        if (s.v[603] != 0.0) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), (-0.5)), 439);
        }

        if (s.v[603] != 0.0) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), 0.25), 34);
        }

        if (!(s.v[603] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[603] != 0.0)) {
            s.store_div_from_scalar_ad(459, 1.0, A::sinh(A::scale(s.ad_value(439), 0.5)));
        }

        if (!(s.v[603] != 0.0)) {
            s.store_square(35, 459);
        }

        if (!(s.v[603] != 0.0)) {
            s.store_sqrt_ad(458, A::offset(s.ad_value(35), 1.0));
        }

        if (!(s.v[603] != 0.0)) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), 0.5), 439);
        }

        if (!(s.v[603] != 0.0)) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), (-0.25)), 34);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_sub_ad(429, A::add(A::sub(s.ad_value(423), s.ad_value(422)), s.ad_value(440)), A::ln(A::abs(A::mul(A::mul(A::mul(s.ad_value(442), s.ad_value(35)), s.ad_value(37)), s.ad_value(37)))));

        s.store_add_ad_rhs(427, 457, A::mul(A::add(s.ad_value(456), s.ad_value(446)), A::add(A::scale(s.ad_value(429), s.v[420]), s.ad_value(456))));

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_ad_lhs(443, A::scale(s.ad_value(456), ((-2.0) * s.v[419])), 457);

        s.store_mul(444, 445, 443);

        s.store_sub_ad(441, A::offset(A::scale(A::mul(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37)), 2.0), (-1.0)), A::mul(s.ad_value(447), s.ad_value(443)));

        s.store_add_ad(428, A::add(A::sub(s.ad_value(457), A::scale(A::add(s.ad_value(456), s.ad_value(36)), s.v[419])), A::mul(s.ad_value(456), s.ad_value(444))), A::scale(A::add(A::mul(s.ad_value(441), s.ad_value(36)), A::mul(s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])))), s.v[420]));

        s.store_div_ad_lhs(425, A::neg(s.ad_value(427)), 428);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_ad(457, A::neg(s.ad_value(421)), A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.v[604] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[604] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[604] != 0.0) {
            s.store_div_from_scalar_ad(459, 1.0, A::sin(A::scale(s.ad_value(439), 0.5)));
        }

        if (s.v[604] != 0.0) {
            s.store_square(35, 459);
        }

        if (s.v[604] != 0.0) {
            s.store_mul_ad_lhs(458, A::cos(A::scale(s.ad_value(439), 0.5)), 459);
        }

        if (s.v[604] != 0.0) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), (-0.5)), 439);
        }

        if (s.v[604] != 0.0) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), 0.25), 34);
        }

        if (!(s.v[604] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[604] != 0.0)) {
            s.store_div_from_scalar_ad(459, 1.0, A::sinh(A::scale(s.ad_value(439), 0.5)));
        }

        if (!(s.v[604] != 0.0)) {
            s.store_square(35, 459);
        }

        if (!(s.v[604] != 0.0)) {
            s.store_sqrt_ad(458, A::offset(s.ad_value(35), 1.0));
        }

        if (!(s.v[604] != 0.0)) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), 0.5), 439);
        }

        if (!(s.v[604] != 0.0)) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), (-0.25)), 34);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

    }

    pub(super) fn stamp_reactive_block_3(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.store_sub_ad(429, A::add(A::sub(s.ad_value(423), s.ad_value(422)), s.ad_value(440)), A::ln(A::abs(A::mul(A::mul(A::mul(s.ad_value(442), s.ad_value(35)), s.ad_value(37)), s.ad_value(37)))));

        s.store_add_ad_rhs(427, 457, A::mul(A::add(s.ad_value(456), s.ad_value(446)), A::add(A::scale(s.ad_value(429), s.v[420]), s.ad_value(456))));

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_ad_lhs(443, A::scale(s.ad_value(456), ((-2.0) * s.v[419])), 457);

        s.store_mul(444, 445, 443);

        s.store_sub_ad(441, A::offset(A::scale(A::mul(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37)), 2.0), (-1.0)), A::mul(s.ad_value(447), s.ad_value(443)));

        s.store_add_ad(428, A::add(A::sub(s.ad_value(457), A::scale(A::add(s.ad_value(456), s.ad_value(36)), s.v[419])), A::mul(s.ad_value(456), s.ad_value(444))), A::scale(A::add(A::mul(s.ad_value(441), s.ad_value(36)), A::mul(s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])))), s.v[420]));

        s.store_div_ad_lhs(425, A::neg(s.ad_value(427)), 428);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_ad(457, A::neg(s.ad_value(421)), A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.v[605] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[605] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[605] != 0.0) {
            s.store_div_from_scalar_ad(459, 1.0, A::sin(A::scale(s.ad_value(439), 0.5)));
        }

        if (s.v[605] != 0.0) {
            s.store_square(35, 459);
        }

        if (s.v[605] != 0.0) {
            s.store_mul_ad_lhs(458, A::cos(A::scale(s.ad_value(439), 0.5)), 459);
        }

        if (s.v[605] != 0.0) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), (-0.5)), 439);
        }

        if (s.v[605] != 0.0) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), 0.25), 34);
        }

        if (!(s.v[605] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[605] != 0.0)) {
            s.store_div_from_scalar_ad(459, 1.0, A::sinh(A::scale(s.ad_value(439), 0.5)));
        }

        if (!(s.v[605] != 0.0)) {
            s.store_square(35, 459);
        }

        if (!(s.v[605] != 0.0)) {
            s.store_sqrt_ad(458, A::offset(s.ad_value(35), 1.0));
        }

        if (!(s.v[605] != 0.0)) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), 0.5), 439);
        }

        if (!(s.v[605] != 0.0)) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), (-0.25)), 34);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_sub_ad(429, A::add(A::sub(s.ad_value(423), s.ad_value(422)), s.ad_value(440)), A::ln(A::abs(A::mul(A::mul(A::mul(s.ad_value(442), s.ad_value(35)), s.ad_value(37)), s.ad_value(37)))));

        s.store_add_ad_rhs(427, 457, A::mul(A::add(s.ad_value(456), s.ad_value(446)), A::add(A::scale(s.ad_value(429), s.v[420]), s.ad_value(456))));

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_ad_lhs(443, A::scale(s.ad_value(456), ((-2.0) * s.v[419])), 457);

        s.store_mul(444, 445, 443);

        s.store_sub_ad(441, A::offset(A::scale(A::mul(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37)), 2.0), (-1.0)), A::mul(s.ad_value(447), s.ad_value(443)));

        s.store_add_ad(428, A::add(A::sub(s.ad_value(457), A::scale(A::add(s.ad_value(456), s.ad_value(36)), s.v[419])), A::mul(s.ad_value(456), s.ad_value(444))), A::scale(A::add(A::mul(s.ad_value(441), s.ad_value(36)), A::mul(s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])))), s.v[420]));

        s.store_div_ad_lhs(425, A::neg(s.ad_value(427)), 428);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_mul_ad_rhs(34, 421, A::exp(s.ad_value(448)));

        s.store_sub_ad_lhs(442, A::mul(A::scale(s.ad_value(440), s.v[451]), s.ad_value(440)), 34);

        s.v[606] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[606] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[606] != 0.0) {
            s.store_scale(36, 439, 0.5);
        }

        if (s.v[606] != 0.0) {
            s.store_div_ad_rhs(446, 439, A::tan(s.ad_value(36)));
        }

        if (s.v[606] != 0.0) {
            s.store_sin(40, 36);
        }

        if (s.v[606] != 0.0) {
            s.store_mul_ad_lhs(35, A::neg(s.ad_value(40)), 40);
        }

        if (!(s.v[606] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[606] != 0.0)) {
            s.store_scale(36, 439, 0.5);
        }

        if (!(s.v[606] != 0.0)) {
            s.store_sinh(40, 36);
        }

        if (!(s.v[606] != 0.0)) {
            s.store_square(35, 40);
        }

        if (!(s.v[606] != 0.0)) {
            s.store_div_ad_rhs(446, 439, A::tanh(s.ad_value(36)));
        }

        s.store_div_ad(437, A::sub(A::scale(s.ad_value(440), s.v[419]), s.ad_value(446)), A::sub_from_scalar(1.0, A::div(s.ad_value(442), A::mul(s.ad_value(35), s.ad_value(34)))));

        s.store_mul_ad_lhs(431, A::scale(s.ad_value(440), s.v[17]), 81);

        s.store_mul_ad_lhs(435, A::scale(s.ad_value(437), s.v[20]), 81);

        s.store_sub(433, 435, 431);

        s.store_sub_ad_rhs(430, 423, A::div(s.ad_value(433), A::scale(s.ad_value(81), s.v[19])));

        s.store_scale_ad(210, A::mul(A::add(s.ad_value(448), s.ad_value(430)), s.ad_value(81)), 0.5);

        s.store_scale(109, 435, 1.0 / (s.v[17]));

        s.store_scale(111, 290, (1.60219e-19 * (p.p49 * 1.0 / (s.v[17]))));

        s.store_add_ad_lhs(36, A::scale(A::mul(s.ad_value(114), s.ad_value(431)), 1.0 / (s.v[17])), 111);

        s.store_scale_ad(37, A::add(s.ad_value(36), A::sqrt(A::offset(A::square(s.ad_value(36)), 0.001))), 0.5);

        s.store_scale(127, 37, s.v[129]);

        s.store_add_ad_lhs(36, A::scale(A::mul(s.ad_value(143), s.ad_value(433)), 1.0 / (s.v[19])), 111);

        s.store_scale_ad(37, A::add(s.ad_value(36), A::sqrt(A::offset(A::square(s.ad_value(36)), 0.001))), 0.5);

        s.store_scale(128, 37, s.v[144]);

        s.v[59] = (0.01 / s.v[17]);

        s.store_ad(607, &A::pow(A::scale(A::offset(A::abs(A::scale(s.ad_value(109), 1.0 / (s.v[59]))), 1.0), 0.5), s.ad_value(124)));

        s.store_add_ad(608, A::mul(A::add(s.ad_value(122), A::mul(s.ad_value(23), s.ad_value(123))), A::pow(A::abs(s.ad_value(127)), A::add(s.ad_value(336), A::mul(s.ad_value(342), s.ad_value(23))))), A::div(s.ad_value(125), s.ad_value(607)));

        s.store_offset(112, 608, 1.0);

        s.store_scale_ad(112, A::add(A::offset(s.ad_value(112), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(112), (-1.0)), A::offset(s.ad_value(112), (-1.0))), ((0.25 * p.p154) * p.p154)))), 0.5);

        s.store_scale(112, 112, 1.0 / (p.p11));

        s.store_div(141, 126, 112);

        s.store_ad(609, &A::pow(A::scale(A::offset(A::abs(A::scale(s.ad_value(109), 1.0 / (s.v[59]))), 1.0), 0.5), s.ad_value(348)));

        s.store_add_ad(610, A::mul(A::add(s.ad_value(345), A::mul(s.ad_value(23), s.ad_value(346))), A::pow(A::abs(s.ad_value(128)), A::add(s.ad_value(349), A::mul(s.ad_value(350), s.ad_value(23))))), A::div(s.ad_value(347), s.ad_value(609)));

        s.store_offset(112, 610, 1.0);

        s.store_scale_ad(112, A::add(A::offset(s.ad_value(112), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(112), (-1.0)), A::offset(s.ad_value(112), (-1.0))), ((0.25 * p.p154) * p.p154)))), 0.5);

        s.store_scale(112, 112, 1.0 / (p.p11));

        s.store_div(142, 344, 112);

        s.store_sub_ad_rhs(34, 71, A::scale(s.ad_value(431), 1.0 / (s.v[17])));

        s.store_sub_ad(35, A::sub(s.ad_value(70), s.ad_value(86)), A::scale(s.ad_value(433), 1.0 / (s.v[19])));

        s.store_div_ad(139, A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::add(A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::exp(A::div(s.ad_value(35), s.ad_value(81)))));

        s.store_div_ad(140, A::exp(A::div(s.ad_value(35), s.ad_value(81))), A::add(A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::exp(A::div(s.ad_value(35), s.ad_value(81)))));

        s.store_add_ad(121, A::mul(s.ad_value(139), s.ad_value(141)), A::mul(s.ad_value(140), s.ad_value(142)));

        s.v[611] = if (p.p14 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[611] != 0.0) {
            s.store_scalar(152, 0.0);
        }

        s.v[612] = if (p.p14 == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[611] != 0.0)) && (s.v[612] != 0.0)) {
            s.store_offset_ad(38, A::mul(s.ad_value(284), s.ad_value(109)), 1.0);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[612] != 0.0)) {
            s.store_div_from_scalar(35, 1.0, 38);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[612] != 0.0)) {
            s.store_scale_ad(34, A::add(s.ad_value(35), A::sqrt(A::offset(A::square(s.ad_value(35)), 0.01))), 0.5);
        }

        if ((!(s.v[611] != 0.0)) && (s.v[612] != 0.0)) {
            s.store_mul_ad_lhs(152, A::scale(A::mul(A::add(s.ad_value(134), A::mul(s.ad_value(281), s.ad_value(34))), s.ad_value(131)), p.p2), 150);
        }

        if ((!(s.v[611] != 0.0)) && (!(s.v[612] != 0.0))) {
            s.store_offset_ad(38, A::mul(s.ad_value(284), s.ad_value(109)), 1.0);
        }

        if ((!(s.v[611] != 0.0)) && (!(s.v[612] != 0.0))) {
            s.store_div_from_scalar(35, 1.0, 38);
        }

        if ((!(s.v[611] != 0.0)) && (!(s.v[612] != 0.0))) {
            s.store_scale_ad(34, A::add(s.ad_value(35), A::sqrt(A::offset(A::square(s.ad_value(35)), 0.01))), 0.5);
        }

        if ((!(s.v[611] != 0.0)) && (!(s.v[612] != 0.0))) {
            s.store_mul_ad_lhs(152, A::scale(A::mul(A::add(A::add(A::add(s.ad_value(132), s.ad_value(133)), s.ad_value(134)), A::mul(s.ad_value(281), s.ad_value(34))), s.ad_value(131)), p.p2), 150);
        }

        s.store_div_ad_lhs(169, A::scale(s.ad_value(164), 2.0), 121);

        s.store_mul(170, 169, 2);

        s.store_mul_ad_rhs(40, 404, A::add(A::add(s.ad_value(109), A::mul(s.ad_value(407), s.ad_value(28))), A::mul(A::scale(s.ad_value(55), 2.0), s.ad_value(405))));

        s.v[613] = if (s.v[152] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[613] != 0.0) {
            s.store_div_ad(162, A::mul(s.ad_value(170), s.ad_value(40)), A::add(s.ad_value(170), s.ad_value(40)));
        }

        if (!(s.v[613] != 0.0)) {
            s.store_scaled_mul(177, 3, 164, s.v[17]);
        }

        if (!(s.v[613] != 0.0)) {
            s.store_mul(34, 177, 152);
        }

        if (!(s.v[613] != 0.0)) {
            s.store_scale(178, 34, 2.0);
        }

        if (!(s.v[613] != 0.0)) {
            s.store_add_ad(179, A::add(s.ad_value(40), s.ad_value(170)), A::mul(A::scale(s.ad_value(40), 3.0), s.ad_value(34)));
        }

        if (!(s.v[613] != 0.0)) {
            s.store_mul_ad_rhs(180, 40, A::add(s.ad_value(170), A::mul(A::scale(s.ad_value(40), 2.0), s.ad_value(34))));
        }

        if (!(s.v[613] != 0.0)) {
            s.store_div_ad_lhs(162, A::sub(s.ad_value(179), A::sqrt(A::sub(A::square(s.ad_value(179)), A::mul(A::scale(s.ad_value(178), 2.0), s.ad_value(180))))), 178);
        }

        s.store_offset_ad(162, A::scale(A::add(A::offset(s.ad_value(162), (-0.001)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(162), (-0.001)), A::offset(s.ad_value(162), (-0.001))), ((4.0 * 1e-5) * 1e-5)))), 0.5), 0.001);

        s.store_ad(41, &A::pow(A::div(s.ad_value(26), s.ad_value(162)), s.ad_value(168)));

        s.store_ad(42, &A::pow(A::offset(s.ad_value(41), 1.0), s.ad_value(163)));

        s.store_div(113, 26, 42);

        s.v[614] = if (s.v[113] > s.v[26]) { 1.0 } else { 0.0 };

        if (s.v[614] != 0.0) {
            s.copy_ad(113, 26);
        }

        s.store_div_ad_lhs(422, A::sub(s.ad_value(71), s.ad_value(113)), 81);

        s.store_div_ad_lhs(423, A::sub(A::offset(A::sub(s.ad_value(70), s.ad_value(86)), p.p10), s.ad_value(113)), 81);

        s.store_sub_ad_lhs(453, A::ln(A::offset(A::mul(A::scale(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451]), A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_sub_ad_lhs(424, A::ln(A::offset(A::mul(A::scale(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451]), A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_scale_ad(37, A::sub(A::scale(s.ad_value(450), (1.0 + s.v[419])), s.ad_value(430)), 1.0 / (s.v[419]));

        s.store_sub_ad_lhs(38, A::ln(A::offset(A::mul(A::scale(A::sub(s.ad_value(37), s.ad_value(450)), s.v[451]), A::sub(s.ad_value(37), s.ad_value(450))), 39.47841)), 449);

        s.store_sub(39, 38, 450);

        s.store_sub(424, 424, 39);

        s.store_scale_ad(452, A::add(s.ad_value(424), A::scale(s.ad_value(423), s.v[420])), 1.0 / ((1.0 + s.v[420])));

        s.store_add_ad_rhs(426, 423, A::scale(A::sub(s.ad_value(422), s.ad_value(423)), s.v[454]));

        s.store_ad(430, &A::min(s.ad_value(426), s.ad_value(453)));

        s.store_ad(430, &A::min(s.ad_value(430), s.ad_value(450)));

        s.store_scale_ad(448, A::add(s.ad_value(430), A::scale(s.ad_value(422), s.v[419])), 1.0 / ((1.0 + s.v[419])));

        s.store_sub(34, 448, 430);

        s.store_div_ad_lhs(37, A::mul(A::limited_exp(s.ad_value(430)), A::offset(A::limited_exp(s.ad_value(34)), (-1.0))), 34);

        s.store_sub(429, 423, 452);

        s.store_sub_ad(442, A::mul(A::scale(s.ad_value(429), (s.v[420] * s.v[420])), s.ad_value(429)), A::mul(s.ad_value(421), A::exp(s.ad_value(452))));

        s.v[615] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[615] != 0.0) {
            s.store_scaled_sub(429, 423, 430, s.v[420]);
        }

        if (s.v[615] != 0.0) {
            s.store_scalar(440, (40.0 * s.v[419]));
        }

        if (s.v[615] != 0.0) {
            s.store_add(455, 440, 429);
        }

        if (s.v[615] != 0.0) {
            s.store_mul(37, 440, 429);
        }

        if (s.v[615] != 0.0) {
            s.store_offset_scaled(38, 455, 0.06534, 1.0);
        }

        if (s.v[615] != 0.0) {
            s.store_offset_ad(39, A::add(A::scale(s.ad_value(455), 8.57973), s.ad_value(37)), 39.47841);
        }

        if (s.v[615] != 0.0) {
            s.store_add_ad(40, A::scale(s.ad_value(455), 78.95683), A::scale(s.ad_value(37), 39.47841));
        }

        if (s.v[615] != 0.0) {
            s.store_div_ad(442, A::sub(A::sqrt(A::add(A::mul(A::scale(s.ad_value(38), (-4.0)), s.ad_value(40)), A::square(s.ad_value(39)))), s.ad_value(39)), A::scale(s.ad_value(38), 2.0));
        }

        if (s.v[615] != 0.0) {
            s.store_scale_ad(37, A::sub(A::scale(s.ad_value(450), (1.0 + s.v[419])), s.ad_value(430)), 1.0 / (s.v[419]));
        }

        if (s.v[615] != 0.0) {
            s.store_scale_ad(34, A::offset(A::sub(s.ad_value(422), s.ad_value(37)), 2.0), (40.0 * 0.2));
        }

        if (s.v[615] != 0.0) {
            s.store_mul_ad_rhs(442, 442, A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(A::offset(A::sub(s.ad_value(422), s.ad_value(37)), 2.0)), 1.0 / ((2.0 / 0.69))))));
        }

        if (s.v[615] != 0.0) {
            s.store_ad(442, &A::min_with_scalar(s.ad_value(442), 50.0));
        }

        s.store_ad(422, &A::max(s.ad_value(422), s.ad_value(450)));

        s.store_sub_ad_lhs(424, A::ln(A::offset(A::mul(A::scale(A::sub(s.ad_value(422), s.ad_value(450)), s.v[451]), A::sub(s.ad_value(422), s.ad_value(450))), 39.47841)), 449);

        s.store_scale_ad(37, A::sub(A::scale(s.ad_value(450), (1.0 + s.v[419])), s.ad_value(430)), 1.0 / (s.v[419]));

        s.store_sub_ad_lhs(38, A::ln(A::offset(A::mul(A::scale(A::sub(s.ad_value(37), s.ad_value(450)), s.v[451]), A::sub(s.ad_value(37), s.ad_value(450))), 39.47841)), 449);

        s.store_sub(39, 38, 450);

        s.store_sub(424, 424, 39);

        s.store_sub(440, 422, 424);

        s.store_mul_ad(34, A::neg(s.ad_value(421)), A::exp(s.ad_value(424)));

        s.store_scale(35, 440, s.v[451]);

        s.store_div_ad(425, A::neg(A::sub(A::add(A::mul(s.ad_value(35), s.ad_value(440)), s.ad_value(34)), s.ad_value(442))), A::add(A::scale(s.ad_value(35), (-2.0)), s.ad_value(34)));

        s.store_add(424, 424, 425);

        s.store_sub(440, 422, 424);

        s.store_scale(36, 440, s.v[451]);

        s.store_div_from_scalar_ad(34, 1.0, A::sub(A::mul(s.ad_value(36), s.ad_value(440)), s.ad_value(442)));

        s.store_sub_ad_lhs(465, A::sub(A::ln(A::abs(A::sub(A::mul(s.ad_value(36), s.ad_value(440)), s.ad_value(442)))), s.ad_value(449)), 424);

        s.store_div_from_scalar_ad(466, 1.0, A::offset(A::mul(A::scale(s.ad_value(36), (-2.0)), s.ad_value(34)), (-1.0)));

        s.store_add_ad(467, A::mul(A::mul(A::mul(A::scale(s.ad_value(36), (-4.0)), s.ad_value(36)), s.ad_value(34)), s.ad_value(34)), A::scale(s.ad_value(34), (2.0 * s.v[451])));

        s.store_mul(35, 465, 466);

        s.store_sub_ad(425, A::neg(s.ad_value(35)), A::mul(A::mul(A::mul(A::scale(s.ad_value(35), 0.5), s.ad_value(35)), s.ad_value(467)), s.ad_value(466)));

        s.store_ad(425, &A::max_with_scalar(s.ad_value(425), (-10.0)));

        s.store_ad(425, &A::min_with_scalar(s.ad_value(425), 10.0));

        s.store_add(424, 424, 425);

        s.store_sub(440, 422, 424);

        s.store_scale(36, 440, s.v[451]);

        s.store_div_from_scalar_ad(34, 1.0, A::sub(A::mul(s.ad_value(36), s.ad_value(440)), s.ad_value(442)));

        s.store_sub_ad_lhs(465, A::sub(A::ln(A::abs(A::sub(A::mul(s.ad_value(36), s.ad_value(440)), s.ad_value(442)))), s.ad_value(449)), 424);

        s.store_div_from_scalar_ad(466, 1.0, A::offset(A::mul(A::scale(s.ad_value(36), (-2.0)), s.ad_value(34)), (-1.0)));

        s.store_add_ad(467, A::mul(A::mul(A::mul(A::scale(s.ad_value(36), (-4.0)), s.ad_value(36)), s.ad_value(34)), s.ad_value(34)), A::scale(s.ad_value(34), (2.0 * s.v[451])));

        s.store_mul(35, 465, 466);

        s.store_sub_ad(425, A::neg(s.ad_value(35)), A::mul(A::mul(A::mul(A::scale(s.ad_value(35), 0.5), s.ad_value(35)), s.ad_value(467)), s.ad_value(466)));

        s.store_ad(425, &A::max_with_scalar(s.ad_value(425), (-10.0)));

        s.store_ad(425, &A::min_with_scalar(s.ad_value(425), 10.0));

        s.store_add(424, 424, 425);

        s.store_ad(424, &A::max(s.ad_value(424), A::offset(s.ad_value(450), (-4.0))));

        s.store_div_ad_lhs(422, A::sub(s.ad_value(71), s.ad_value(113)), 81);

        s.store_sub_ad_rhs(448, 448, A::ln(A::offset(A::exp(A::sub(s.ad_value(448), A::scale(s.ad_value(424), 1.05))), 1.0)));

        s.store_ad(448, &A::min(s.ad_value(448), s.ad_value(424)));

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_ad(457, A::neg(s.ad_value(421)), A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.v[616] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[616] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[616] != 0.0) {
            s.store_div_from_scalar_ad(459, 1.0, A::sin(A::scale(s.ad_value(439), 0.5)));
        }

        if (s.v[616] != 0.0) {
            s.store_square(35, 459);
        }

        if (s.v[616] != 0.0) {
            s.store_mul_ad_lhs(458, A::cos(A::scale(s.ad_value(439), 0.5)), 459);
        }

        if (s.v[616] != 0.0) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), (-0.5)), 439);
        }

        if (s.v[616] != 0.0) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), 0.25), 34);
        }

        if (!(s.v[616] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[616] != 0.0)) {
            s.store_div_from_scalar_ad(459, 1.0, A::sinh(A::scale(s.ad_value(439), 0.5)));
        }

        if (!(s.v[616] != 0.0)) {
            s.store_square(35, 459);
        }

        if (!(s.v[616] != 0.0)) {
            s.store_sqrt_ad(458, A::offset(s.ad_value(35), 1.0));
        }

    }

    pub(super) fn stamp_reactive_block_4(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (!(s.v[616] != 0.0)) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), 0.5), 439);
        }

        if (!(s.v[616] != 0.0)) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), (-0.25)), 34);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_sub_ad(429, A::add(A::sub(s.ad_value(423), s.ad_value(422)), s.ad_value(440)), A::ln(A::abs(A::mul(A::mul(A::mul(s.ad_value(442), s.ad_value(35)), s.ad_value(37)), s.ad_value(37)))));

        s.store_add_ad_rhs(427, 457, A::mul(A::add(s.ad_value(456), s.ad_value(446)), A::add(A::scale(s.ad_value(429), s.v[420]), s.ad_value(456))));

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_ad_lhs(443, A::scale(s.ad_value(456), ((-2.0) * s.v[419])), 457);

        s.store_mul(444, 445, 443);

        s.store_sub_ad(441, A::offset(A::scale(A::mul(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37)), 2.0), (-1.0)), A::mul(s.ad_value(447), s.ad_value(443)));

        s.store_add_ad(428, A::add(A::sub(s.ad_value(457), A::scale(A::add(s.ad_value(456), s.ad_value(36)), s.v[419])), A::mul(s.ad_value(456), s.ad_value(444))), A::scale(A::add(A::mul(s.ad_value(441), s.ad_value(36)), A::mul(s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])))), s.v[420]));

        s.store_div_ad_lhs(425, A::neg(s.ad_value(427)), 428);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_ad(457, A::neg(s.ad_value(421)), A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.v[617] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[617] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[617] != 0.0) {
            s.store_div_from_scalar_ad(459, 1.0, A::sin(A::scale(s.ad_value(439), 0.5)));
        }

        if (s.v[617] != 0.0) {
            s.store_square(35, 459);
        }

        if (s.v[617] != 0.0) {
            s.store_mul_ad_lhs(458, A::cos(A::scale(s.ad_value(439), 0.5)), 459);
        }

        if (s.v[617] != 0.0) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), (-0.5)), 439);
        }

        if (s.v[617] != 0.0) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), 0.25), 34);
        }

        if (!(s.v[617] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[617] != 0.0)) {
            s.store_div_from_scalar_ad(459, 1.0, A::sinh(A::scale(s.ad_value(439), 0.5)));
        }

        if (!(s.v[617] != 0.0)) {
            s.store_square(35, 459);
        }

        if (!(s.v[617] != 0.0)) {
            s.store_sqrt_ad(458, A::offset(s.ad_value(35), 1.0));
        }

        if (!(s.v[617] != 0.0)) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), 0.5), 439);
        }

        if (!(s.v[617] != 0.0)) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), (-0.25)), 34);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_sub_ad(429, A::add(A::sub(s.ad_value(423), s.ad_value(422)), s.ad_value(440)), A::ln(A::abs(A::mul(A::mul(A::mul(s.ad_value(442), s.ad_value(35)), s.ad_value(37)), s.ad_value(37)))));

        s.store_add_ad_rhs(427, 457, A::mul(A::add(s.ad_value(456), s.ad_value(446)), A::add(A::scale(s.ad_value(429), s.v[420]), s.ad_value(456))));

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_ad_lhs(443, A::scale(s.ad_value(456), ((-2.0) * s.v[419])), 457);

        s.store_mul(444, 445, 443);

        s.store_sub_ad(441, A::offset(A::scale(A::mul(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37)), 2.0), (-1.0)), A::mul(s.ad_value(447), s.ad_value(443)));

        s.store_add_ad(428, A::add(A::sub(s.ad_value(457), A::scale(A::add(s.ad_value(456), s.ad_value(36)), s.v[419])), A::mul(s.ad_value(456), s.ad_value(444))), A::scale(A::add(A::mul(s.ad_value(441), s.ad_value(36)), A::mul(s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])))), s.v[420]));

        s.store_div_ad_lhs(425, A::neg(s.ad_value(427)), 428);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_ad(457, A::neg(s.ad_value(421)), A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.v[618] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[618] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[618] != 0.0) {
            s.store_div_from_scalar_ad(459, 1.0, A::sin(A::scale(s.ad_value(439), 0.5)));
        }

        if (s.v[618] != 0.0) {
            s.store_square(35, 459);
        }

        if (s.v[618] != 0.0) {
            s.store_mul_ad_lhs(458, A::cos(A::scale(s.ad_value(439), 0.5)), 459);
        }

        if (s.v[618] != 0.0) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), (-0.5)), 439);
        }

        if (s.v[618] != 0.0) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), 0.25), 34);
        }

        if (!(s.v[618] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[618] != 0.0)) {
            s.store_div_from_scalar_ad(459, 1.0, A::sinh(A::scale(s.ad_value(439), 0.5)));
        }

        if (!(s.v[618] != 0.0)) {
            s.store_square(35, 459);
        }

        if (!(s.v[618] != 0.0)) {
            s.store_sqrt_ad(458, A::offset(s.ad_value(35), 1.0));
        }

        if (!(s.v[618] != 0.0)) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), 0.5), 439);
        }

        if (!(s.v[618] != 0.0)) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), (-0.25)), 34);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_sub_ad(429, A::add(A::sub(s.ad_value(423), s.ad_value(422)), s.ad_value(440)), A::ln(A::abs(A::mul(A::mul(A::mul(s.ad_value(442), s.ad_value(35)), s.ad_value(37)), s.ad_value(37)))));

        s.store_add_ad_rhs(427, 457, A::mul(A::add(s.ad_value(456), s.ad_value(446)), A::add(A::scale(s.ad_value(429), s.v[420]), s.ad_value(456))));

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_ad_lhs(443, A::scale(s.ad_value(456), ((-2.0) * s.v[419])), 457);

        s.store_mul(444, 445, 443);

        s.store_sub_ad(441, A::offset(A::scale(A::mul(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37)), 2.0), (-1.0)), A::mul(s.ad_value(447), s.ad_value(443)));

        s.store_add_ad(428, A::add(A::sub(s.ad_value(457), A::scale(A::add(s.ad_value(456), s.ad_value(36)), s.v[419])), A::mul(s.ad_value(456), s.ad_value(444))), A::scale(A::add(A::mul(s.ad_value(441), s.ad_value(36)), A::mul(s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])))), s.v[420]));

        s.store_div_ad_lhs(425, A::neg(s.ad_value(427)), 428);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_ad(457, A::neg(s.ad_value(421)), A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.v[619] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[619] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[619] != 0.0) {
            s.store_div_from_scalar_ad(459, 1.0, A::sin(A::scale(s.ad_value(439), 0.5)));
        }

        if (s.v[619] != 0.0) {
            s.store_square(35, 459);
        }

        if (s.v[619] != 0.0) {
            s.store_mul_ad_lhs(458, A::cos(A::scale(s.ad_value(439), 0.5)), 459);
        }

        if (s.v[619] != 0.0) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), (-0.5)), 439);
        }

        if (s.v[619] != 0.0) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), 0.25), 34);
        }

        if (!(s.v[619] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[619] != 0.0)) {
            s.store_div_from_scalar_ad(459, 1.0, A::sinh(A::scale(s.ad_value(439), 0.5)));
        }

        if (!(s.v[619] != 0.0)) {
            s.store_square(35, 459);
        }

        if (!(s.v[619] != 0.0)) {
            s.store_sqrt_ad(458, A::offset(s.ad_value(35), 1.0));
        }

        if (!(s.v[619] != 0.0)) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), 0.5), 439);
        }

        if (!(s.v[619] != 0.0)) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), (-0.25)), 34);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_sub_ad(429, A::add(A::sub(s.ad_value(423), s.ad_value(422)), s.ad_value(440)), A::ln(A::abs(A::mul(A::mul(A::mul(s.ad_value(442), s.ad_value(35)), s.ad_value(37)), s.ad_value(37)))));

        s.store_add_ad_rhs(427, 457, A::mul(A::add(s.ad_value(456), s.ad_value(446)), A::add(A::scale(s.ad_value(429), s.v[420]), s.ad_value(456))));

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_ad_lhs(443, A::scale(s.ad_value(456), ((-2.0) * s.v[419])), 457);

        s.store_mul(444, 445, 443);

        s.store_sub_ad(441, A::offset(A::scale(A::mul(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37)), 2.0), (-1.0)), A::mul(s.ad_value(447), s.ad_value(443)));

        s.store_add_ad(428, A::add(A::sub(s.ad_value(457), A::scale(A::add(s.ad_value(456), s.ad_value(36)), s.v[419])), A::mul(s.ad_value(456), s.ad_value(444))), A::scale(A::add(A::mul(s.ad_value(441), s.ad_value(36)), A::mul(s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])))), s.v[420]));

        s.store_div_ad_lhs(425, A::neg(s.ad_value(427)), 428);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_scale(456, 440, s.v[419]);

        s.store_mul_ad(457, A::neg(s.ad_value(421)), A::exp(s.ad_value(448)));

        s.store_add_ad_lhs(442, A::square(s.ad_value(456)), 457);

        s.v[620] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[620] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[620] != 0.0) {
            s.store_div_from_scalar_ad(459, 1.0, A::sin(A::scale(s.ad_value(439), 0.5)));
        }

        if (s.v[620] != 0.0) {
            s.store_square(35, 459);
        }

        if (s.v[620] != 0.0) {
            s.store_mul_ad_lhs(458, A::cos(A::scale(s.ad_value(439), 0.5)), 459);
        }

        if (s.v[620] != 0.0) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), (-0.5)), 439);
        }

        if (s.v[620] != 0.0) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), 0.25), 34);
        }

        if (!(s.v[620] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[620] != 0.0)) {
            s.store_div_from_scalar_ad(459, 1.0, A::sinh(A::scale(s.ad_value(439), 0.5)));
        }

        if (!(s.v[620] != 0.0)) {
            s.store_square(35, 459);
        }

        if (!(s.v[620] != 0.0)) {
            s.store_sqrt_ad(458, A::offset(s.ad_value(35), 1.0));
        }

        if (!(s.v[620] != 0.0)) {
            s.store_div_ad_lhs(34, A::scale(s.ad_value(458), 0.5), 439);
        }

        if (!(s.v[620] != 0.0)) {
            s.store_add_ad_lhs(445, A::scale(s.ad_value(35), (-0.25)), 34);
        }

        s.store_mul(446, 439, 458);

        s.store_add(36, 456, 446);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_sub_ad(429, A::add(A::sub(s.ad_value(423), s.ad_value(422)), s.ad_value(440)), A::ln(A::abs(A::mul(A::mul(A::mul(s.ad_value(442), s.ad_value(35)), s.ad_value(37)), s.ad_value(37)))));

        s.store_add_ad_rhs(427, 457, A::mul(A::add(s.ad_value(456), s.ad_value(446)), A::add(A::scale(s.ad_value(429), s.v[420]), s.ad_value(456))));

        s.store_sub_ad_lhs(447, A::div_from_scalar(1.0, s.ad_value(442)), 34);

        s.store_add_ad_lhs(443, A::scale(s.ad_value(456), ((-2.0) * s.v[419])), 457);

        s.store_mul(444, 445, 443);

        s.store_sub_ad(441, A::offset(A::scale(A::mul(A::sub(s.ad_value(444), A::constant(s.v[419])), s.ad_value(37)), 2.0), (-1.0)), A::mul(s.ad_value(447), s.ad_value(443)));

        s.store_add_ad(428, A::add(A::sub(s.ad_value(457), A::scale(A::add(s.ad_value(456), s.ad_value(36)), s.v[419])), A::mul(s.ad_value(456), s.ad_value(444))), A::scale(A::add(A::mul(s.ad_value(441), s.ad_value(36)), A::mul(s.ad_value(429), A::offset(s.ad_value(444), (-s.v[419])))), s.v[420]));

        s.store_div_ad_lhs(425, A::neg(s.ad_value(427)), 428);

        s.store_add(448, 448, 425);

        s.store_sub(440, 422, 448);

        s.store_mul_ad_rhs(34, 421, A::exp(s.ad_value(448)));

        s.store_sub_ad_lhs(442, A::mul(A::scale(s.ad_value(440), s.v[451]), s.ad_value(440)), 34);

        s.v[621] = if (s.v[442] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[621] != 0.0) {
            s.store_sqrt_ad(439, A::neg(s.ad_value(442)));
        }

        if (s.v[621] != 0.0) {
            s.store_scale(36, 439, 0.5);
        }

        if (s.v[621] != 0.0) {
            s.store_div_ad_rhs(446, 439, A::tan(s.ad_value(36)));
        }

        if (s.v[621] != 0.0) {
            s.store_sin(40, 36);
        }

        if (s.v[621] != 0.0) {
            s.store_mul_ad_lhs(35, A::neg(s.ad_value(40)), 40);
        }

        if (!(s.v[621] != 0.0)) {
            s.store_sqrt(439, 442);
        }

        if (!(s.v[621] != 0.0)) {
            s.store_scale(36, 439, 0.5);
        }

        if (!(s.v[621] != 0.0)) {
            s.store_sinh(40, 36);
        }

        if (!(s.v[621] != 0.0)) {
            s.store_square(35, 40);
        }

        if (!(s.v[621] != 0.0)) {
            s.store_div_ad_rhs(446, 439, A::tanh(s.ad_value(36)));
        }

        s.store_div_ad(438, A::sub(A::scale(s.ad_value(440), s.v[419]), s.ad_value(446)), A::sub_from_scalar(1.0, A::div(s.ad_value(442), A::mul(s.ad_value(35), s.ad_value(34)))));

        s.store_mul_ad_lhs(432, A::scale(s.ad_value(440), s.v[17]), 81);

        s.store_mul_ad_lhs(436, A::scale(s.ad_value(438), s.v[20]), 81);

        s.store_sub(434, 436, 432);

        s.store_sub_ad_rhs(430, 423, A::div(s.ad_value(434), A::scale(s.ad_value(81), s.v[19])));

        s.store_scale(110, 436, 1.0 / (s.v[17]));

        s.store_scaled_add(46, 109, 110, 0.5);

        s.store_sub(49, 109, 110);

        s.store_scale(48, 290, (1.60219e-19 * (p.p49 * 1.0 / (s.v[17]))));

        s.store_scale_ad(34, A::powf(s.ad_value(113), 2.0), 1600.0);

        s.v[622] = if (p.p162 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[622] != 0.0) {
            s.store_add_ad(47, A::scale(A::add(s.ad_value(431), s.ad_value(432)), 1.0 / ((2.0 * s.v[17]))), A::scale(A::mul(A::scale(A::sub_from_scalar(1.0, A::limited_exp(A::neg(s.ad_value(34)))), (p.p162 * 0.5)), A::sub(s.ad_value(431), s.ad_value(432))), 1.0 / (s.v[17])));
        }

        if (!(s.v[622] != 0.0)) {
            s.store_scaled_add(47, 431, 432, 1.0 / ((2.0 * s.v[17])));
        }

        s.v[623] = if (p.p189 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[623] != 0.0) {
            s.store_add_ad(145, A::scale(A::add(s.ad_value(433), s.ad_value(434)), 1.0 / ((2.0 * s.v[19]))), A::scale(A::mul(A::scale(A::sub_from_scalar(1.0, A::limited_exp(A::neg(s.ad_value(34)))), (p.p189 * 0.5)), A::sub(s.ad_value(433), s.ad_value(434))), 1.0 / (s.v[19])));
        }

        if (!(s.v[623] != 0.0)) {
            s.store_scaled_add(145, 433, 434, 1.0 / ((2.0 * s.v[19])));
        }

        s.store_add_ad_lhs(36, A::mul(s.ad_value(114), s.ad_value(47)), 48);

        s.store_scale_ad(37, A::add(s.ad_value(36), A::sqrt(A::offset(A::square(s.ad_value(36)), 0.001))), 0.5);

        s.store_scale(116, 37, s.v[129]);

        s.store_add_ad_lhs(36, A::mul(s.ad_value(143), s.ad_value(145)), 48);

        s.store_scale_ad(37, A::add(s.ad_value(36), A::sqrt(A::offset(A::square(s.ad_value(36)), 0.001))), 0.5);

        s.store_scale(117, 37, s.v[144]);

        s.store_ad(624, &A::pow(A::scale(A::offset(A::abs(A::scale(s.ad_value(46), 1.0 / (s.v[59]))), 1.0), 0.5), s.ad_value(124)));

        s.store_add_ad(625, A::mul(A::add(s.ad_value(122), A::mul(s.ad_value(25), s.ad_value(123))), A::pow(A::abs(s.ad_value(116)), A::add(s.ad_value(336), A::mul(s.ad_value(342), s.ad_value(25))))), A::div(A::add(s.ad_value(125), A::mul(s.ad_value(25), s.ad_value(137))), s.ad_value(624)));

        s.store_offset(119, 625, 1.0);

        s.store_scale_ad(119, A::add(A::offset(s.ad_value(119), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(119), (-1.0)), A::offset(s.ad_value(119), (-1.0))), ((0.25 * p.p154) * p.p154)))), 0.5);

        s.store_scale(119, 119, 1.0 / (p.p11));

        s.store_div(141, 126, 119);

        s.store_ad(626, &A::pow(A::scale(A::offset(A::abs(A::scale(s.ad_value(46), 1.0 / (s.v[59]))), 1.0), 0.5), s.ad_value(348)));

        s.store_add_ad(627, A::mul(A::add(s.ad_value(345), A::mul(s.ad_value(25), s.ad_value(346))), A::pow(A::abs(s.ad_value(117)), A::add(s.ad_value(349), A::mul(s.ad_value(350), s.ad_value(25))))), A::div(A::add(s.ad_value(347), A::mul(s.ad_value(25), s.ad_value(138))), s.ad_value(626)));

        s.store_offset(119, 627, 1.0);

        s.store_scale_ad(119, A::add(A::offset(s.ad_value(119), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(119), (-1.0)), A::offset(s.ad_value(119), (-1.0))), ((0.25 * p.p154) * p.p154)))), 0.5);

        s.store_scale(119, 119, 1.0 / (p.p11));

        s.store_div(142, 344, 119);

        s.store_sub_ad_rhs(34, 71, A::scale(A::add(s.ad_value(431), s.ad_value(432)), 1.0 / ((2.0 * s.v[17]))));

        s.store_sub_ad(35, A::sub(s.ad_value(70), s.ad_value(86)), A::scale(A::add(s.ad_value(433), s.ad_value(434)), 1.0 / ((2.0 * s.v[19]))));

        s.store_div_ad(139, A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::add(A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::exp(A::div(s.ad_value(35), s.ad_value(81)))));

        s.store_div_ad(140, A::exp(A::div(s.ad_value(35), s.ad_value(81))), A::add(A::exp(A::div(s.ad_value(34), s.ad_value(81))), A::exp(A::div(s.ad_value(35), s.ad_value(81)))));

        s.store_add_ad(121, A::mul(s.ad_value(139), s.ad_value(141)), A::mul(s.ad_value(140), s.ad_value(142)));

        s.store_div_ad_lhs(56, A::mul(A::scale(s.ad_value(121), s.v[17]), s.ad_value(3)), 2);

        s.store_scale_ad(118, A::add(s.ad_value(48), A::mul(s.ad_value(115), s.ad_value(46))), s.v[129]);

        s.store_mul_ad_rhs(37, 122, A::pow(A::abs(s.ad_value(118)), s.ad_value(336)));

        s.store_offset(120, 37, 1.0);

        s.store_scale_ad(120, A::add(A::offset(s.ad_value(120), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(120), (-1.0)), A::offset(s.ad_value(120), (-1.0))), ((0.25 * p.p154) * p.p154)))), 0.5);

        s.store_scale(120, 120, 1.0 / (p.p11));

        s.store_div_ad_lhs(173, A::scale(s.ad_value(166), 2.0), 121);

        s.store_mul(174, 173, 2);

        s.store_offset_ad(34, A::mul(s.ad_value(165), s.ad_value(25)), 0.8);

    }

    pub(super) fn stamp_reactive_block_5(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.store_offset_ad(181, A::scale(A::add(s.ad_value(34), A::sqrt(A::offset(A::square(s.ad_value(34)), 0.01))), 0.5), 0.2);

        s.store_mul_ad_lhs(34, A::div(s.ad_value(49), s.ad_value(174)), 181);

        s.store_scale_ad(161, A::offset(A::sqrt(A::offset(A::square(s.ad_value(34)), p.p109)), 1.0), 1.0 / ((1.0 + ((p.p109) as f64).sqrt())));

        s.store_add_ad_rhs(161, 161, A::mul(A::mul(A::mul(A::scale(A::sub(A::sub(s.ad_value(182), A::mul(s.ad_value(328), s.ad_value(28))), A::mul(s.ad_value(329), s.ad_value(25))), 0.5), s.ad_value(46)), s.ad_value(49)), s.ad_value(49)));

        s.store_scale_ad(161, A::add(A::offset(s.ad_value(161), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(161), (-1.0)), A::offset(s.ad_value(161), (-1.0))), ((0.25 * p.p134) * p.p134)))), 0.5);

        s.store_div_ad_lhs(171, A::mul(A::scale(s.ad_value(167), 2.0), s.ad_value(120)), 126);

        s.store_mul(172, 171, 1);

        s.v[628] = if (s.v[365] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[628] != 0.0) {
            s.store_offset_ad(154, A::div(A::mul(s.ad_value(365), s.ad_value(46)), s.ad_value(170)), 1.0);
        }

        if (!(s.v[628] != 0.0)) {
            s.store_div_from_scalar_ad(154, 1.0, A::sub_from_scalar(1.0, A::div(A::mul(s.ad_value(365), s.ad_value(46)), s.ad_value(170))));
        }

        s.store_sub(155, 26, 113);

        s.store_add_ad_rhs(157, 46, A::scale(s.ad_value(55), 2.0));

        s.v[629] = if (s.v[153] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[629] != 0.0) {
            s.copy_ad(35, 157);
        }

        if (s.v[629] != 0.0) {
            s.store_div_ad_rhs(37, 35, A::add(s.ad_value(162), s.ad_value(35)));
        }

        if (s.v[629] != 0.0) {
            s.store_mul_ad_lhs(156, A::mul(A::div(s.ad_value(35), s.ad_value(153)), s.ad_value(37)), 154);
        }

        if (s.v[629] != 0.0) {
            s.store_offset_ad(158, A::div(s.ad_value(155), s.ad_value(156)), 1.0);
        }

        if (!(s.v[629] != 0.0)) {
            s.store_scalar(158, 1.0);
        }

        s.v[630] = if (s.v[360] > 0.0) { 1.0 } else { 0.0 };

        s.v[631] = if (p.p213 < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[630] != 0.0) && (s.v[631] != 0.0)) {
            s.store_div_from_scalar_ad(35, 1.0, A::sub(A::div_from_scalar(1.0, s.ad_value(360)), A::scale(s.ad_value(46), p.p213)));
        }

        if ((s.v[630] != 0.0) && (!(s.v[631] != 0.0))) {
            s.store_mul_ad_rhs(35, 360, A::offset(A::scale(s.ad_value(46), p.p213), 1.0));
        }

        if (s.v[630] != 0.0) {
            s.store_offset_ad(159, A::mul(s.ad_value(35), A::ln(A::max_with_scalar(A::offset(A::div(A::div(s.ad_value(155), s.ad_value(35)), A::add(s.ad_value(162), s.ad_value(170))), 1.0), 1e-38))), 1.0);
        }

        if (!(s.v[630] != 0.0)) {
            s.store_scalar(159, 1.0);
        }

        s.store_mul(158, 158, 159);

        s.v[632] = if (s.v[361] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[632] != 0.0) {
            s.store_offset_ad(160, A::mul(s.ad_value(361), A::ln(A::max_with_scalar(A::offset(A::div(A::div(A::sub(s.ad_value(26), s.ad_value(113)), s.ad_value(361)), A::add(s.ad_value(162), s.ad_value(172))), 1.0), 1e-38))), 1.0);
        }

        if (!(s.v[632] != 0.0)) {
            s.store_scalar(160, 1.0);
        }

        s.v[633] = if (s.v[175] != 0.0) { 1.0 } else { 0.0 };

        if (s.v[633] != 0.0) {
            s.store_div_ad_rhs(35, 175, A::add(A::mul(A::max_from_scalar(0.0, A::add(s.ad_value(176), A::mul(A::mul(s.ad_value(108), s.ad_value(49)), s.ad_value(49)))), s.ad_value(46)), A::scale(s.ad_value(81), 2.0)));
        }

        if (s.v[633] != 0.0) {
            s.store_limited_exp_ad(94, A::neg(s.ad_value(35)));
        }

        if (!(s.v[633] != 0.0)) {
            s.store_scalar(94, 1.0);
        }

        s.store_sub(34, 437, 438);

        s.store_sub_ad(35, A::square(s.ad_value(437)), A::square(s.ad_value(438)));

        s.store_add_ad(215, A::mul(A::mul(A::scale(s.ad_value(81), (s.v[20] * 2.0)), s.ad_value(55)), s.ad_value(34)), A::scale(A::mul(A::scale(A::mul(A::scale(s.ad_value(81), (s.v[20] * s.v[20])), s.ad_value(81)), 0.5), s.ad_value(35)), 1.0 / (s.v[17])));

        s.store_add_ad_lhs(216, A::scale(A::add(s.ad_value(109), s.ad_value(110)), 0.5), 55);

        s.v[640] = if (p.p14 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[640] != 0.0) {
            s.store_scalar(151, 0.0);
        }

        if (s.v[640] != 0.0) {
            s.store_scalar(130, 1.0);
        }

        if (s.v[640] != 0.0) {
            s.store_sub(638, 29, 200);
        }

        if (s.v[640] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(638)), 0.0001));
        }

        if (s.v[640] != 0.0) {
            s.store_scaled_add(636, 638, 639, 0.5);
        }

        if (s.v[640] != 0.0) {
            s.store_offset_ad(635, A::mul(s.ad_value(284), s.ad_value(636)), 1.0);
        }

        if (s.v[640] != 0.0) {
            s.store_div_from_scalar(634, 1.0, 635);
        }

        if (s.v[640] != 0.0) {
            s.store_sub_ad_rhs(634, 634, A::mul(A::scale(s.ad_value(32), 0.5), s.ad_value(285)));
        }

        if (s.v[640] != 0.0) {
            s.store_scale_ad(34, A::add(s.ad_value(634), A::sqrt(A::offset(A::square(s.ad_value(634)), 0.01))), 0.5);
        }

        if (s.v[640] != 0.0) {
            s.store_sub(638, 31, 200);
        }

        if (s.v[640] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(638)), 0.0001));
        }

        if (s.v[640] != 0.0) {
            s.store_scaled_add(637, 638, 639, 0.5);
        }

        if (s.v[640] != 0.0) {
            s.store_offset_ad(635, A::mul(s.ad_value(284), s.ad_value(637)), 1.0);
        }

        if (s.v[640] != 0.0) {
            s.store_div_from_scalar(634, 1.0, 635);
        }

        if (s.v[640] != 0.0) {
            s.store_sub_ad_rhs(634, 634, A::mul(A::scale(s.ad_value(33), 0.5), s.ad_value(285)));
        }

        if (s.v[640] != 0.0) {
            s.store_scale_ad(34, A::add(s.ad_value(634), A::sqrt(A::offset(A::square(s.ad_value(634)), 0.01))), 0.5);
        }

        if (!(s.v[640] != 0.0)) {
            s.store_offset_ad(635, A::mul(s.ad_value(284), s.ad_value(46)), 1.0);
        }

        if (!(s.v[640] != 0.0)) {
            s.store_div_from_scalar(634, 1.0, 635);
        }

        if (!(s.v[640] != 0.0)) {
            s.store_sub_ad_rhs(634, 634, A::mul(A::scale(A::add(s.ad_value(24), s.ad_value(23)), 0.5), s.ad_value(285)));
        }

        if (!(s.v[640] != 0.0)) {
            s.store_scale_ad(34, A::add(s.ad_value(634), A::sqrt(A::offset(A::square(s.ad_value(634)), 0.01))), 0.5);
        }

        if (!(s.v[640] != 0.0)) {
            s.store_mul_ad_rhs(151, 150, A::mul(A::add(s.ad_value(134), A::mul(s.ad_value(281), s.ad_value(34))), s.ad_value(131)));
        }

        if (!(s.v[640] != 0.0)) {
            s.store_offset_ad(130, A::mul(A::div(A::mul(A::scale(s.ad_value(56), p.p2), s.ad_value(216)), s.ad_value(161)), s.ad_value(151)), 1.0);
        }

        s.v[641] = if (p.p14 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[640] != 0.0)) && (s.v[641] != 0.0)) {
            s.store_offset_ad(635, A::mul(s.ad_value(284), s.ad_value(46)), 1.0);
        }

        if ((!(s.v[640] != 0.0)) && (s.v[641] != 0.0)) {
            s.store_div_from_scalar(634, 1.0, 635);
        }

        if ((!(s.v[640] != 0.0)) && (s.v[641] != 0.0)) {
            s.store_sub_ad_rhs(634, 634, A::mul(A::scale(A::add(s.ad_value(24), s.ad_value(23)), 0.5), s.ad_value(285)));
        }

        if ((!(s.v[640] != 0.0)) && (s.v[641] != 0.0)) {
            s.store_scale_ad(34, A::add(s.ad_value(634), A::sqrt(A::offset(A::square(s.ad_value(634)), 0.01))), 0.5);
        }

        if ((!(s.v[640] != 0.0)) && (s.v[641] != 0.0)) {
            s.store_mul_ad_lhs(151, A::mul(s.ad_value(150), A::add(A::add(A::add(s.ad_value(132), s.ad_value(133)), s.ad_value(134)), A::mul(s.ad_value(281), s.ad_value(34)))), 131);
        }

        if ((!(s.v[640] != 0.0)) && (s.v[641] != 0.0)) {
            s.store_offset_ad(130, A::mul(A::div(A::mul(A::scale(s.ad_value(56), p.p2), s.ad_value(216)), s.ad_value(161)), s.ad_value(151)), 1.0);
        }

        s.store_div_ad(214, A::mul(A::mul(A::mul(A::scale(s.ad_value(56), 1.0 / (s.v[17])), s.ad_value(215)), s.ad_value(158)), s.ad_value(94)), A::mul(s.ad_value(161), s.ad_value(130)));

        s.store_scale(214, 214, p.p2);

        s.store_scaled_add(219, 432, 431, 0.5);

        s.store_scale_ad(218, A::add(s.ad_value(435), A::scale(s.ad_value(436), 2.0)), (1.0 / 6.0));

        s.store_scale_ad(217, A::add(A::scale(s.ad_value(435), 2.0), s.ad_value(436)), (1.0 / 6.0));

        s.store_scaled_add(220, 434, 433, 0.5);

        s.v[642] = if (s.v[62] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[642] != 0.0) {
            s.store_div_ad_lhs(38, A::add(s.ad_value(46), A::mul(s.ad_value(66), s.ad_value(48))), 67);
        }

        if (s.v[642] != 0.0) {
            s.store_offset_ad(39, A::pow(s.ad_value(38), s.ad_value(68)), 1.0);
        }

        if (s.v[642] != 0.0) {
            s.store_scalar(63, p.p49);
        }

        if (s.v[642] != 0.0) {
            s.store_div(64, 63, 39);
        }

        if (s.v[642] != 0.0) {
            s.store_div_from_scalar_ad(65, (3.9 * 8.85418e-12), A::add(A::scale(s.ad_value(221), (3.9 * 1.0 / (p.p60))), A::scale(A::mul(s.ad_value(64), s.ad_value(62)), 1.0 / (s.v[21]))));
        }

        if (!(s.v[642] != 0.0)) {
            s.store_scalar(65, s.v[18]);
        }

        s.store_div_ad_lhs(34, A::mul(s.ad_value(4), s.ad_value(1)), 160);

        s.store_mul(219, 219, 34);

        s.store_mul_ad_lhs(218, A::neg(s.ad_value(218)), 34);

        s.store_mul(220, 220, 34);

        s.store_mul_ad_lhs(217, A::neg(s.ad_value(217)), 34);

        s.store_ad(228, &A::mul(A::scale(A::mul(s.ad_value(4), s.ad_value(396)), s.v[17]), A::voltage(ctx, &nodes, Some(7), Some(6))));

        s.store_ad(230, &A::mul(A::scale(A::mul(s.ad_value(4), s.ad_value(397)), s.v[17]), A::voltage(ctx, &nodes, Some(7), Some(5))));

        s.store_mul_ad_rhs(240, 212, A::sub(s.ad_value(288), s.ad_value(99)));

        s.store_add_ad(34, A::offset(A::sub(s.ad_value(235), s.ad_value(200)), 0.02), A::scale(A::offset(A::sub(s.ad_value(32), s.ad_value(240)), (-p.p268)), ((p.p45 / p.p46) * p.p269)));

        s.store_scale_ad(232, A::sub(s.ad_value(34), A::sqrt(A::offset(A::square(s.ad_value(34)), (4.0 * 0.02)))), 0.5);

        s.store_sub_ad_lhs(35, A::sub(s.ad_value(235), s.ad_value(200)), 232);

        s.store_add_ad_rhs(228, 228, A::mul(A::scale(A::mul(s.ad_value(212), s.ad_value(4)), p.p263), A::sub(s.ad_value(35), A::scale(A::offset(A::sqrt(A::sub_from_scalar(1.0, A::scale(s.ad_value(232), (4.0 * 1.0 / (p.p265))))), (-1.0)), (0.5 * p.p265)))));

        s.store_add_ad(34, A::offset(A::sub(s.ad_value(234), s.ad_value(200)), 0.02), A::scale(A::offset(A::sub(s.ad_value(33), s.ad_value(240)), (-p.p270)), ((p.p45 / p.p46) * p.p271)));

        s.store_scale_ad(233, A::sub(s.ad_value(34), A::sqrt(A::offset(A::square(s.ad_value(34)), (4.0 * 0.02)))), 0.5);

        s.store_sub_ad_lhs(35, A::sub(s.ad_value(234), s.ad_value(200)), 233);

        s.store_add_ad_rhs(230, 230, A::mul(A::scale(A::mul(s.ad_value(212), s.ad_value(4)), p.p264), A::sub(s.ad_value(35), A::scale(A::offset(A::sqrt(A::sub_from_scalar(1.0, A::scale(s.ad_value(233), (4.0 * 1.0 / (p.p266))))), (-1.0)), (0.5 * p.p266)))));

        s.store_ad(229, &A::mul(A::mul(s.ad_value(4), s.ad_value(398)), A::voltage(ctx, &nodes, Some(7), Some(6))));

        s.store_ad(231, &A::mul(A::mul(s.ad_value(4), s.ad_value(399)), A::voltage(ctx, &nodes, Some(7), Some(5))));

        s.store_add(226, 228, 229);

        s.store_add(227, 230, 231);

        s.store_ad(238, &A::mul(A::scale(s.ad_value(212), s.v[236]), A::voltage(ctx, &nodes, Some(6), Some(3))));

        s.store_ad(239, &A::mul(A::scale(s.ad_value(212), s.v[237]), A::voltage(ctx, &nodes, Some(5), Some(3))));

        s.store_div_ad_lhs(34, A::add(s.ad_value(366), A::mul(s.ad_value(367), s.ad_value(2))), 2);

        s.v[643] = if ((s.v[34] <= 0.0) || (s.v[103] <= 0.0)) { 1.0 } else { 0.0 };

        s.v[644] = if (s.v[155] > (s.v[103] / 80.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[643] != 0.0)) && (s.v[644] != 0.0)) {
            s.store_div_ad_lhs(35, A::neg(s.ad_value(103)), 155);
        }

        s.v[645] = if (p.p17 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[645] != 0.0) {
            s.store_div_ad_lhs(35, A::div(A::sub(s.ad_value(46), s.ad_value(411)), s.ad_value(412)), 55);
        }

        if (s.v[645] != 0.0) {
            s.store_sub_ad_rhs(36, 408, A::mul(s.ad_value(409), s.ad_value(46)));
        }

        if (s.v[645] != 0.0) {
            s.store_offset_ad(37, A::mul(s.ad_value(410), s.ad_value(46)), 1.0);
        }

        if (s.v[645] != 0.0) {
            s.store_mul_ad_lhs(38, A::scale(s.ad_value(36), ((-982222000000.0) * p.p99)), 37);
        }

        if (s.v[645] != 0.0) {
            s.store_ad(39, &A::limited_exp(s.ad_value(38)));
        }

        if (s.v[645] != 0.0) {
            s.store_scalar(40, 3.75956e-7);
        }

        if (s.v[645] != 0.0) {
            s.store_sub(191, 52, 50);
        }

        if (s.v[645] != 0.0) {
            s.store_sub(34, 191, 209);
        }

        if (s.v[645] != 0.0) {
            s.store_div_ad_lhs(35, A::div(s.ad_value(34), s.ad_value(416)), 55);
        }

        s.v[646] = if (s.v[191] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[645] != 0.0) && (s.v[646] != 0.0)) {
            s.store_scale_ad(189, A::add(A::offset(s.ad_value(34), (-0.02)), A::sqrt(A::sub(A::mul(A::offset(s.ad_value(34), (-0.02)), A::offset(s.ad_value(34), (-0.02))), A::scale(s.ad_value(191), 0.08)))), 0.5);
        }

        if ((s.v[645] != 0.0) && (!(s.v[646] != 0.0))) {
            s.store_scale_ad(189, A::add(A::offset(s.ad_value(34), (-0.02)), A::sqrt(A::add(A::mul(A::offset(s.ad_value(34), (-0.02)), A::offset(s.ad_value(34), (-0.02))), A::scale(s.ad_value(191), 0.08)))), 0.5);
        }

        if (s.v[645] != 0.0) {
            s.store_sub_ad_rhs(36, 413, A::mul(s.ad_value(414), s.ad_value(189)));
        }

        if (s.v[645] != 0.0) {
            s.store_offset_ad(37, A::mul(s.ad_value(415), s.ad_value(189)), 1.0);
        }

        if (s.v[645] != 0.0) {
            s.store_mul_ad_lhs(38, A::scale(s.ad_value(36), ((-745669000000.0) * p.p99)), 37);
        }

        if (s.v[645] != 0.0) {
            s.store_ad(39, &A::limited_exp(s.ad_value(38)));
        }

        if (s.v[645] != 0.0) {
            s.store_scalar(40, 4.97232e-7);
        }

        s.store_tanh_ad(34, A::div(A::scale(s.ad_value(30), 0.6), s.ad_value(55)));

        s.v[647] = if (p.p16 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[647] != 0.0) {
            s.store_sub_ad_rhs(35, 369, A::mul(s.ad_value(370), A::sub(s.ad_value(69), A::mul(s.ad_value(373), s.ad_value(210)))));
        }

        if (s.v[647] != 0.0) {
            s.store_offset_ad(36, A::mul(s.ad_value(371), A::sub(s.ad_value(69), A::mul(s.ad_value(373), s.ad_value(210)))), 1.0);
        }

        if (s.v[647] != 0.0) {
            s.store_mul_ad_lhs(37, A::mul(A::scale(A::neg(s.ad_value(206)), p.p99), s.ad_value(35)), 36);
        }

        if (s.v[647] != 0.0) {
            s.store_mul_ad_rhs(38, 46, A::limited_exp(s.ad_value(37)));
        }

        if (s.v[647] != 0.0) {
            s.store_add_ad(39, A::add(s.ad_value(209), A::scale(s.ad_value(73), 0.5)), A::scale(A::add(s.ad_value(32), s.ad_value(33)), 0.5));
        }

        if (s.v[647] != 0.0) {
            s.store_offset_ad(196, A::sqrt(A::offset(A::square(s.ad_value(113)), 0.01)), (-0.1));
        }

        if (s.v[647] != 0.0) {
            s.store_mul(35, 372, 196);
        }

        if (s.v[647] != 0.0) {
            s.store_limited_exp_ad(197, A::neg(s.ad_value(35)));
        }

        if (s.v[647] != 0.0) {
            s.store_offset_ad(37, A::offset(A::add(s.ad_value(35), s.ad_value(197)), (-1.0)), 0.0001);
        }

        if (s.v[647] != 0.0) {
            s.store_offset_ad(38, A::sub_from_scalar(1.0, A::mul(A::offset(s.ad_value(35), 1.0), s.ad_value(197))), 0.0001);
        }

        if (s.v[647] != 0.0) {
            s.store_offset_ad(39, A::square(s.ad_value(35)), 0.0002);
        }

        if (s.v[647] != 0.0) {
            s.store_add_ad(34, A::sub(s.ad_value(29), s.ad_value(200)), A::mul(A::scale(s.ad_value(385), s.v[243]), A::sub(s.ad_value(23), s.ad_value(240))));
        }

        if (s.v[647] != 0.0) {
            s.store_sqrt_ad(203, A::offset(A::square(s.ad_value(34)), 0.0001));
        }

        if (s.v[647] != 0.0) {
            s.store_sub_ad_rhs(35, 382, A::mul(s.ad_value(383), s.ad_value(203)));
        }

        if (s.v[647] != 0.0) {
            s.store_offset_ad(36, A::mul(s.ad_value(384), s.ad_value(203)), 1.0);
        }

        if (s.v[647] != 0.0) {
            s.store_mul_ad_lhs(37, A::mul(A::mul(A::scale(A::neg(s.ad_value(206)), p.p99), s.ad_value(394)), s.ad_value(35)), 36);
        }

        if (s.v[647] != 0.0) {
            s.store_ad(38, &A::limited_exp(s.ad_value(37)));
        }

        if (s.v[647] != 0.0) {
            s.store_add_ad(34, A::sub(s.ad_value(31), s.ad_value(200)), A::mul(A::scale(s.ad_value(389), s.v[243]), A::sub(s.ad_value(23), s.ad_value(240))));
        }

        if (s.v[647] != 0.0) {
            s.store_sqrt_ad(204, A::offset(A::square(s.ad_value(34)), 0.0001));
        }

        if (s.v[647] != 0.0) {
            s.store_sub_ad_rhs(35, 386, A::mul(s.ad_value(387), s.ad_value(204)));
        }

        if (s.v[647] != 0.0) {
            s.store_offset_ad(36, A::mul(s.ad_value(388), s.ad_value(204)), 1.0);
        }

        if (s.v[647] != 0.0) {
            s.store_mul_ad_lhs(37, A::mul(A::mul(A::scale(A::neg(s.ad_value(206)), p.p99), s.ad_value(394)), s.ad_value(35)), 36);
        }

        if (s.v[647] != 0.0) {
            s.store_ad(38, &A::limited_exp(s.ad_value(37)));
        }

        s.v[650] = if (p.p15 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[650] != 0.0) {
            s.store_scalar(34, (s.v[21] * p.p45));
        }

        s.v[651] = if ((s.v[378] <= 0.0) || (s.v[104] <= 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[650] != 0.0) && (s.v[651] != 0.0)) {
            s.store_scalar(40, 0.0);
        }

        if ((s.v[650] != 0.0) && (!(s.v[651] != 0.0))) {
            s.store_div_ad_lhs(35, A::add(A::add(A::sub(A::neg(s.ad_value(31)), s.ad_value(380)), s.ad_value(200)), A::mul(A::scale(s.ad_value(390), s.v[243]), A::sub(A::sub(s.ad_value(23), s.ad_value(240)), s.ad_value(391)))), 34);
        }

        if ((s.v[650] != 0.0) && (!(s.v[651] != 0.0))) {
            s.store_scale_ad(35, A::add(s.ad_value(35), A::sqrt(A::offset(A::square(s.ad_value(35)), ((4.0 * 0.01) * 0.01)))), 0.5);
        }

        if ((s.v[650] != 0.0) && (!(s.v[651] != 0.0))) {
            s.store_div_ad_rhs(36, 104, A::offset(s.ad_value(35), 0.001));
        }

        if ((s.v[650] != 0.0) && (!(s.v[651] != 0.0))) {
            s.store_limited_exp_ad(37, A::mul(s.ad_value(381), A::ln(A::max_with_scalar(s.ad_value(35), 1e-38))));
        }

        if ((s.v[650] != 0.0) && (!(s.v[651] != 0.0))) {
            s.store_mul_ad_lhs(40, A::mul(A::mul(A::mul(s.ad_value(378), s.ad_value(3)), s.ad_value(37)), A::limited_exp(A::neg(s.ad_value(36)))), 30);
        }

        s.v[653] = if ((s.v[374] <= 0.0) || (s.v[105] <= 0.0)) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_6(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[650] != 0.0) && (s.v[653] != 0.0)) {
            s.store_scalar(40, 0.0);
        }

        if ((s.v[650] != 0.0) && (!(s.v[653] != 0.0))) {
            s.store_div_ad_lhs(35, A::add(A::add(A::sub(A::neg(s.ad_value(29)), s.ad_value(376)), s.ad_value(200)), A::mul(A::scale(s.ad_value(392), s.v[243]), A::sub(A::sub(s.ad_value(23), s.ad_value(240)), s.ad_value(393)))), 34);
        }

        if ((s.v[650] != 0.0) && (!(s.v[653] != 0.0))) {
            s.store_scale_ad(35, A::add(s.ad_value(35), A::sqrt(A::offset(A::square(s.ad_value(35)), ((4.0 * 0.01) * 0.01)))), 0.5);
        }

        if ((s.v[650] != 0.0) && (!(s.v[653] != 0.0))) {
            s.store_div_ad_rhs(36, 105, A::offset(s.ad_value(35), 0.001));
        }

        if ((s.v[650] != 0.0) && (!(s.v[653] != 0.0))) {
            s.store_limited_exp_ad(37, A::mul(s.ad_value(377), A::ln(A::max_with_scalar(s.ad_value(35), 1e-38))));
        }

        if ((s.v[650] != 0.0) && (!(s.v[653] != 0.0))) {
            s.store_mul_ad(40, A::mul(A::mul(A::mul(A::neg(s.ad_value(30)), s.ad_value(374)), s.ad_value(3)), s.ad_value(37)), A::limited_exp(A::neg(s.ad_value(36))));
        }

        s.store_div_ad_lhs(254, A::scale(s.ad_value(164), 2.0), 121);

        s.v[655] = if (((p.p288 > 0.0) || (p.p289 > 0.0)) || (p.p290 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[655] != 0.0) {
            s.store_sub_ad_rhs(255, 2, A::scale(s.ad_value(249), 2.0));
        }

        if (s.v[655] != 0.0) {
            s.store_square(256, 255);
        }

        s.v[656] = if (p.p287 <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[655] != 0.0) && (s.v[656] != 0.0)) {
            s.store_scalar(257, 0.0);
        }

        if ((s.v[655] != 0.0) && (!(s.v[656] != 0.0))) {
            s.store_div_ad_lhs(34, A::offset(A::scale(s.ad_value(155), 1.0 / (s.v[253])), p.p287), 254);
        }

        if ((s.v[655] != 0.0) && (!(s.v[656] != 0.0))) {
            s.store_scale_ad(257, A::ln(A::max_with_scalar(s.ad_value(34), 1e-38)), s.v[253]);
        }

        s.v[657] = if (s.v[257] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[655] != 0.0) && (!(s.v[656] != 0.0))) && (s.v[657] != 0.0)) {
            s.store_scalar(257, 0.0);
        }

        s.v[658] = if (p.p22 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[655] != 0.0) && (s.v[658] != 0.0)) {
            s.store_div(35, 47, 252);
        }

        if ((s.v[655] != 0.0) && (s.v[658] != 0.0)) {
            s.store_offset_ad(36, A::pow(s.ad_value(35), s.ad_value(251)), 1.0);
        }

        if ((s.v[655] != 0.0) && (s.v[658] != 0.0)) {
            s.store_div(37, 250, 36);
        }

        if ((s.v[655] != 0.0) && (s.v[658] != 0.0)) {
            s.store_scale(38, 37, 1.0 / (p.p288));
        }

        if ((s.v[655] != 0.0) && (s.v[658] != 0.0)) {
            s.store_scale_ad(39, A::add(A::offset(s.ad_value(38), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(38), (-1.0)), A::offset(s.ad_value(38), (-1.0))), ((0.25 * p.p292) * p.p292)))), 0.5);
        }

        if ((s.v[655] != 0.0) && (s.v[658] != 0.0)) {
            s.store_scale(258, 39, p.p288);
        }

        if ((s.v[655] != 0.0) && (!(s.v[658] != 0.0))) {
            s.store_scalar(258, p.p288);
        }

        if (s.v[655] != 0.0) {
            s.store_mul_ad_lhs(35, A::mul(A::scale(s.ad_value(55), ((1.60219e-19 * 1.60219e-19) * 1.60219e-19)), A::abs(s.ad_value(214))), 121);
        }

        if (s.v[655] != 0.0) {
            s.store_mul_ad_lhs(36, A::scale(s.ad_value(65), 10000000000.0), 256);
        }

        if (s.v[655] != 0.0) {
            s.store_scaled_mul(259, 65, 109, 6.241457005723417e18);
        }

        if (s.v[655] != 0.0) {
            s.store_scaled_mul(260, 65, 110, 6.241457005723417e18);
        }

        if (s.v[655] != 0.0) {
            s.store_mul_ad(261, A::scale(s.ad_value(55), 6.241457005723417e18), A::add(s.ad_value(65), s.ad_value(291)));
        }

        if (s.v[655] != 0.0) {
            s.store_mul_ad_rhs(37, 258, A::ln(A::max_with_scalar(A::div(A::add(s.ad_value(259), s.ad_value(261)), A::add(s.ad_value(260), s.ad_value(261))), 1e-38)));
        }

        if (s.v[655] != 0.0) {
            s.store_scaled_sub(38, 259, 260, p.p289);
        }

        if (s.v[655] != 0.0) {
            s.store_scale_ad(39, A::sub(A::square(s.ad_value(259)), A::square(s.ad_value(260))), (0.5 * p.p290));
        }

        if (s.v[655] != 0.0) {
            s.store_mul_ad_lhs(40, A::mul(A::scale(s.ad_value(55), 1.60219e-19), s.ad_value(214)), 214);
        }

        if (s.v[655] != 0.0) {
            s.store_scale_ad(41, A::mul(A::scale(s.ad_value(256), 10000000000.0), s.ad_value(3)), p.p2);
        }

        if (s.v[655] != 0.0) {
            s.store_add_ad(42, A::add(s.ad_value(258), A::scale(s.ad_value(260), p.p289)), A::mul(A::scale(s.ad_value(260), p.p290), s.ad_value(260)));
        }

        if (s.v[655] != 0.0) {
            s.store_mul_ad(43, A::add(s.ad_value(260), s.ad_value(261)), A::add(s.ad_value(260), s.ad_value(261)));
        }

        if (s.v[655] != 0.0) {
            s.store_add_ad(262, A::mul(A::div(s.ad_value(35), s.ad_value(36)), A::add(A::add(s.ad_value(37), s.ad_value(38)), s.ad_value(39))), A::div(A::mul(A::mul(A::div(s.ad_value(40), s.ad_value(41)), s.ad_value(257)), s.ad_value(42)), s.ad_value(43)));
        }

        if (s.v[655] != 0.0) {
            s.store_mul_ad_lhs(44, A::scale(s.ad_value(258), 1.60219e-19), 55);
        }

        if (s.v[655] != 0.0) {
            s.store_mul_ad_lhs(45, A::mul(A::scale(A::mul(A::scale(s.ad_value(3), p.p2), s.ad_value(255)), 10000000000.0), s.ad_value(261)), 261);
        }

        if (s.v[655] != 0.0) {
            s.store_mul_ad_lhs(263, A::mul(A::div(s.ad_value(44), s.ad_value(45)), s.ad_value(214)), 214);
        }

        if (s.v[655] != 0.0) {
            s.store_add(35, 263, 262);
        }

        s.store_mul_ad_lhs(224, A::scale(s.ad_value(212), p.p2), 219);

        s.store_scale(225, 220, p.p2);

        s.v[660] = if (s.v[27] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[660] != 0.0) {
            s.store_scale(222, 217, p.p2);
        }

        if (s.v[660] != 0.0) {
            s.store_scale(223, 218, p.p2);
        }

        if (s.v[660] != 0.0) {
            s.store_add_ad_lhs(217, A::scale(A::sub(s.ad_value(217), s.ad_value(226)), p.p2), 238);
        }

        if (s.v[660] != 0.0) {
            s.store_add_ad_lhs(218, A::scale(A::sub(s.ad_value(218), s.ad_value(227)), p.p2), 239);
        }

        if (!(s.v[660] != 0.0)) {
            s.store_scale(222, 218, p.p2);
        }

        if (!(s.v[660] != 0.0)) {
            s.store_scale(223, 217, p.p2);
        }

        if (!(s.v[660] != 0.0)) {
            s.store_add_ad_lhs(34, A::scale(A::sub(s.ad_value(218), s.ad_value(226)), p.p2), 238);
        }

        if (!(s.v[660] != 0.0)) {
            s.store_add_ad_lhs(218, A::scale(A::sub(s.ad_value(217), s.ad_value(227)), p.p2), 239);
        }

        if (!(s.v[660] != 0.0)) {
            s.copy_ad(217, 34);
        }

        s.store_add_ad_rhs(219, 224, A::scale(A::add(s.ad_value(226), s.ad_value(227)), p.p2));

        s.store_sub_ad_lhs(220, A::sub(A::scale(s.ad_value(220), p.p2), s.ad_value(238)), 239);

        s.store_scale(226, 226, p.p2);

        s.store_scale(227, 227, p.p2);

        s.store_neg_ad(265, A::add(s.ad_value(222), s.ad_value(223)));

        s.store_mul(34, 121, 265);

        s.store_add_ad(35, A::mul(s.ad_value(34), s.ad_value(151)), A::square(s.ad_value(2)));

        s.v[661] = if ((p.p20 == 1.0) && (s.v[275] != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[661] != 0.0) {
            s.store_div_ad_lhs(34, A::mul(A::scale(s.ad_value(121), s.v[17]), s.ad_value(3)), 2);
        }

        s.v[671] = if ((p.p18 != 0.0) && (p.p310 > 0.0)) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_equation_0_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq0_e787, eq0_e787_d_n0, eq0_e787_d_n1, eq0_e787_d_n2, eq0_e787_d_n3, eq0_e787_d_n4, eq0_e787_d_n5, eq0_e787_d_n6, eq0_e787_d_n7, eq0_e787_d_n8,) = {
    if (s.v[662] != 0.0) {
        let eq0_e779: f64 = (s.v[212] * s.v[214]);
        let eq0_e779_d_n0: f64 = ((s.dn[212][0] * s.v[214]) + (s.v[212] * s.dn[214][0]));
        let eq0_e779_d_n1: f64 = ((s.dn[212][1] * s.v[214]) + (s.v[212] * s.dn[214][1]));
        let eq0_e779_d_n2: f64 = ((s.dn[212][2] * s.v[214]) + (s.v[212] * s.dn[214][2]));
        let eq0_e779_d_n3: f64 = ((s.dn[212][3] * s.v[214]) + (s.v[212] * s.dn[214][3]));
        let eq0_e779_d_n4: f64 = ((s.dn[212][4] * s.v[214]) + (s.v[212] * s.dn[214][4]));
        let eq0_e779_d_n5: f64 = ((s.dn[212][5] * s.v[214]) + (s.v[212] * s.dn[214][5]));
        let eq0_e779_d_n6: f64 = ((s.dn[212][6] * s.v[214]) + (s.v[212] * s.dn[214][6]));
        let eq0_e779_d_n7: f64 = ((s.dn[212][7] * s.v[214]) + (s.v[212] * s.dn[214][7]));
        let eq0_e779_d_n8: f64 = ((s.dn[212][8] * s.v[214]) + (s.v[212] * s.dn[214][8]));
        let eq0_e782: f64 = 1e-12;
        let eq0_e784: f64 = (eq0_e782 * (nv5 - nv6));
        let eq0_e784_d_n6: f64 = (-eq0_e782);
        let eq0_e785: f64 = (eq0_e779 + eq0_e784);
        let eq0_e785_d_n5: f64 = (eq0_e779_d_n5 + eq0_e782);
        let eq0_e785_d_n6: f64 = (eq0_e779_d_n6 + eq0_e784_d_n6);
        (eq0_e785, eq0_e779_d_n0, eq0_e779_d_n1, eq0_e779_d_n2, eq0_e779_d_n3, eq0_e779_d_n4, eq0_e785_d_n5, eq0_e785_d_n6, eq0_e779_d_n7, eq0_e779_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e787;
        let eq0_node_derivatives: [f64; 9] = [eq0_e787_d_n0, eq0_e787_d_n1, eq0_e787_d_n2, eq0_e787_d_n3, eq0_e787_d_n4, eq0_e787_d_n5, eq0_e787_d_n6, eq0_e787_d_n7, eq0_e787_d_n8];
        let eq0_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq0_value),
            &nodes,
            &eq0_node_derivatives,
            &branches,
            &eq0_branch_derivatives,
            self.multiplicity,
        );
    }
}
